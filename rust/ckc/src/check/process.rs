use super::common::*;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub(super) struct Output {
    pub rc: i32,
    pub out: Vec<u8>,
    pub err: Vec<u8>,
}
impl Output {
    pub fn relay(self) -> Failure {
        Failure {
            rc: self.rc as u8,
            out: vec![],
            err: self.err,
        }
    }
}
fn reader(mut pipe: impl Read + Send + 'static) -> mpsc::Receiver<std::io::Result<Vec<u8>>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let result = pipe.read_to_end(&mut bytes).map(|_| bytes);
        tx.send(result).ok();
    });
    rx
}
pub(super) fn walled(
    command: &mut Command,
    input: Option<&[u8]>,
    wall: Duration,
) -> Result<(bool, Output)> {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()
        .map_err(|e| fail("subprocess", e.to_string()))?;
    let pid = child.id();
    let out_rx = reader(
        child
            .stdout
            .take()
            .ok_or_else(|| fail("subprocess", "stdout pipe missing"))?,
    );
    let err_rx = reader(
        child
            .stderr
            .take()
            .ok_or_else(|| fail("subprocess", "stderr pipe missing"))?,
    );
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| fail("subprocess", "stdin pipe missing"))?;
    let input = input.unwrap_or_default().to_vec();
    let writer = std::thread::spawn(move || {
        stdin.write_all(&input).ok();
    });
    let start = Instant::now();
    let (mut stdout, mut stderr, mut status) = (None, None, None);
    let mut timed_out = false;
    loop {
        if stdout.is_none() {
            stdout = out_rx.try_recv().ok();
        }
        if stderr.is_none() {
            stderr = err_rx.try_recv().ok();
        }
        if status.is_none() {
            status = child
                .try_wait()
                .map_err(|e| fail("subprocess", e.to_string()))?;
        }
        if stdout.is_some() && stderr.is_some() && status.is_some() {
            break;
        }
        if start.elapsed() >= wall {
            timed_out = true;
            // The group also owns descendant-held pipes after the parent exits.
            Command::new("/bin/kill")
                .args(["-KILL", "--", &format!("-{pid}")])
                .output()
                .map_err(|e| fail("subprocess", e.to_string()))?;
            if status.is_none() {
                status = Some(
                    child
                        .wait()
                        .map_err(|e| fail("subprocess", e.to_string()))?,
                );
            }
            if stdout.is_none() {
                stdout = Some(
                    out_rx
                        .recv()
                        .map_err(|e| fail("subprocess", e.to_string()))?,
                );
            }
            if stderr.is_none() {
                stderr = Some(
                    err_rx
                        .recv()
                        .map_err(|e| fail("subprocess", e.to_string()))?,
                );
            }
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    writer
        .join()
        .map_err(|_| fail("subprocess", "stdin writer failed"))?;
    let status = status.ok_or_else(|| fail("subprocess", "exit status missing"))?;
    let out = stdout
        .ok_or_else(|| fail("subprocess", "stdout missing"))?
        .map_err(|e| fail("subprocess", e.to_string()))?;
    let err = stderr
        .ok_or_else(|| fail("subprocess", "stderr missing"))?
        .map_err(|e| fail("subprocess", e.to_string()))?;
    Ok((
        timed_out,
        Output {
            rc: status
                .code()
                .unwrap_or_else(|| -status.signal().unwrap_or(2)),
            out,
            err,
        },
    ))
}
pub(super) fn bounded(command: &mut Command, label: &str, input: Option<&[u8]>) -> Result<Output> {
    let (timed, result) = walled(command, input, Duration::from_secs(300))?;
    if timed {
        return Err(fail(
            "swipl-timeout",
            format!("{label} exceeded 300s wall clock"),
        ));
    }
    Ok(result)
}
pub(super) struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new() -> Result<Self> {
        let path = PathBuf::from(format!(".goal.tmp.{}", std::process::id()));
        if path.exists() {
            return Err(fail("scratch", format!("already exists: {}", show(&path))));
        }
        fs::create_dir(&path)
            .map_err(|_| fail("scratch", format!("cannot create: {}", show(&path))))?;
        Ok(Self(path))
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).ok();
    }
}
fn executable(path: &Path) -> bool {
    fs::metadata(path).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}
pub(super) fn swipl() -> Result<PathBuf> {
    let value = std::env::var_os("SWIPL").unwrap_or_else(|| "swipl".into());
    let wanted = PathBuf::from(&value);
    let found = if value.as_encoded_bytes().contains(&b'/') {
        executable(&wanted).then_some(wanted)
    } else {
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .map(|p| p.join(&value))
            .find(|p| executable(p))
    };
    let path = found.ok_or_else(|| {
        fail(
            "swipl-exec",
            format!("not executable: {}", value.to_string_lossy()),
        )
    })?;
    let out = Command::new(&path).arg("--version").output().map_err(|_| {
        fail(
            "swipl-version",
            format!("version probe failed: {}", show(&path)),
        )
    })?;
    if !out.status.success() {
        return Err(fail(
            "swipl-version",
            format!("version probe failed: {}", show(&path)),
        ));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let words: Vec<&str> = text
        .split(super::text::space)
        .filter(|s| !s.is_empty())
        .collect();
    if words.len() < 3 || words[..3] != ["SWI-Prolog", "version", "9.2.9"] {
        return Err(fail(
            "swipl-version",
            format!("expected 9.2.9, found: {}", super::text::strip(&text)),
        ));
    }
    Ok(path)
}
fn copy_tree(src: &Path, dst: &Path) -> Result {
    fs::create_dir(dst).map_err(|e| fail("ape-stage", e.to_string()))?;
    for path in entries(src, "ape-stage")? {
        let to = dst.join(path.file_name().unwrap_or_default());
        if path.is_dir() {
            copy_tree(&path, &to)?;
        } else {
            fs::copy(&path, &to).map_err(|e| fail("ape-stage", e.to_string()))?;
        }
    }
    Ok(())
}
pub(super) fn stage(scratch: &Scratch, swipl: &Path) -> Result<PathBuf> {
    if !Path::new("vendor/ape/prolog/ace_to_pl.pl").is_file() {
        return Err(fail(
            "compiler-source",
            "missing: vendor/ape/prolog/ace_to_pl.pl",
        ));
    }
    let stage = scratch.0.join("ape-stage");
    copy_tree(Path::new("vendor/ape"), &stage)?;
    fs::copy(
        "vendor/clex/clex_lexicon.pl",
        stage.join("prolog/lexicon/clex_lexicon.pl"),
    )
    .map_err(|e| fail("ape-stage", e.to_string()))?;
    let parser = stage.join("prolog/parser");
    let goal = format!(
        "working_directory(_, '{}'), [fit_to_plp], halt.",
        show(&parser)
    );
    let out = bounded(
        Command::new(swipl).args(["-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt"]),
        "ape-stage build",
        None,
    )?;
    if out.rc != 0 {
        return Err(out.relay());
    }
    if !parser.join("grammar.plp").is_file() {
        return Err(fail("ape-stage", "missing grammar.plp after build"));
    }
    Ok(stage)
}
pub(super) fn compiler(swipl: &Path, stage: &Path, tail: &[String]) -> Command {
    let mut c = Command::new(swipl);
    c.args(["-q", "-f", "none", "-F", "none", "-s"])
        .arg(stage.join("prolog/ace_to_pl.pl"))
        .args(["-g", "main", "-t", "halt(9)", "--"])
        .args(tail);
    c
}
pub(super) fn write(path: &Path, bytes: &[u8]) -> Result {
    fs::write(path, bytes).map_err(|e| fail("scratch", e.to_string()))
}
