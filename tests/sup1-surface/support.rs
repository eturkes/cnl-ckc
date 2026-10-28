use super::base::{ULEX, quote};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, Weak};
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
pub fn program() -> PathBuf {
    std::env::var_os("CKC_SUP1_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}
pub fn sha(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn write(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) {
    fs::create_dir_all(path.as_ref().parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}
pub fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for e in fs::read_dir(from).unwrap() {
        let e = e.unwrap();
        assert!(!e.file_type().unwrap().is_symlink());
        if e.file_type().unwrap().is_dir() {
            copy_tree(&e.path(), &to.join(e.file_name()));
        } else {
            fs::copy(e.path(), to.join(e.file_name())).unwrap();
        }
    }
}
static NEXT: AtomicUsize = AtomicUsize::new(0);
pub struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new(label: &str) -> Self {
        let path = root().join(format!(
            "rust/target/sup1-surface/{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!path.exists(), "scratch collision {}", path.display());
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0)
            .unwrap_or_else(|e| eprintln!("SUP1 cleanup {}: {e}", self.0.display()));
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    pub rc: i32,
    pub stdout: String,
    pub stderr: String,
}
impl Run {
    pub fn ok(stdout: impl Into<String>) -> Self {
        Self {
            rc: 0,
            stdout: stdout.into(),
            stderr: String::new(),
        }
    }
    pub fn err(rc: i32, stderr: impl Into<String>) -> Self {
        Self {
            rc,
            stdout: String::new(),
            stderr: stderr.into(),
        }
    }
    pub fn success(&self) -> bool {
        self.rc == 0 && self.stderr.is_empty()
    }
    pub fn require_ok(&self, label: &str) {
        assert!(self.success(), "{label}: {}", self.describe());
    }
    pub fn describe(&self) -> String {
        let clip = |s: &str| s.chars().take(700).collect::<String>();
        format!(
            "rc={} stdout={:?} stderr={:?}",
            self.rc,
            clip(&self.stdout),
            clip(&self.stderr)
        )
    }
    pub fn violation(&self) -> &str {
        if self.stderr.is_empty() {
            self.stdout.lines().last().unwrap_or("")
        } else {
            self.stderr.trim_end()
        }
    }
}
pub fn command_env(
    bin: &Path,
    cwd: &Path,
    args: &[&str],
    input: &[u8],
    env: &[(&str, &Path)],
) -> Run {
    let mut child = Command::new("timeout")
        .args(["--kill-after=2", "90"])
        .arg(bin)
        .args(args)
        .current_dir(cwd)
        .env("LC_ALL", "C.UTF-8")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env("GIT_CEILING_DIRECTORIES", cwd)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("TMPDIR", cwd)
        .env("CKC_JOBS", "1")
        .envs(env.iter().copied())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = child.stdin.take().unwrap().write_all(input);
    let out = child.wait_with_output().unwrap();
    Run {
        rc: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}
pub fn command(bin: &Path, cwd: &Path, args: &[&str], input: &[u8]) -> Run {
    command_env(bin, cwd, args, input, &[])
}
pub fn native(cwd: &Path, args: &[&str]) -> Run {
    command(&program(), cwd, args, b"")
}
pub fn git(cwd: &Path, args: &[&str]) -> String {
    let out = command(Path::new("git"), cwd, args, b"");
    assert_eq!(out.rc, 0, "git {args:?}: {}", out.describe());
    out.stdout
}
#[derive(Default)]
pub struct Checks {
    pub count: usize,
    pub failures: Vec<String>,
}
impl Checks {
    pub fn require(&mut self, yes: bool, label: impl Into<String>) {
        self.count += 1;
        if !yes {
            self.failures.push(label.into());
        }
    }
    pub fn equal(&mut self, got: &str, want: &str, label: &str) {
        let i = got
            .lines()
            .zip(want.lines())
            .position(|(a, b)| a != b)
            .unwrap_or(got.lines().count().min(want.lines().count()));
        self.require(
            got == want,
            format!(
                "{label}: line {} expected {:?}, got {:?}; bytes {} / {}",
                i + 1,
                want.lines().nth(i),
                got.lines().nth(i),
                want.len(),
                got.len()
            ),
        );
    }
    pub fn output(&mut self, got: &Run, want: &Run, label: &str) {
        if got.rc == want.rc && got.stderr == want.stderr {
            self.equal(&got.stdout, &want.stdout, label);
        } else {
            self.require(
                false,
                format!(
                    "{label}: expected {}; got {}",
                    want.describe(),
                    got.describe()
                ),
            );
        }
    }
    pub fn finish(self, label: &str) {
        println!(
            "SUP1 {label}: {} checks; {} failures",
            self.count,
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "SUP1 {label}:\n{}",
            self.failures.join("\n")
        );
    }
}

// One staged compiler while some test holds it: the registry keeps a Weak, every user an Arc,
// and the last drop removes rust/target/sup1-surface/stage-<pid>-<generation> (setup + teardown
// symmetric; a teardown racing a successor's setup never removes the successor's directory).
static GENERATION: AtomicUsize = AtomicUsize::new(0);
pub struct Compiler {
    pub stage: PathBuf,
}
impl Drop for Compiler {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.stage)
            .unwrap_or_else(|e| eprintln!("SUP1 cleanup {}: {e}", self.stage.display()));
    }
}
pub fn compiler() -> Arc<Compiler> {
    static COMPILER: Mutex<Weak<Compiler>> = Mutex::new(Weak::new());
    let mut weak = COMPILER.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(live) = weak.upgrade() {
        return live;
    }
    let made = Arc::new({
        let base = root().join("rust/target/sup1-surface");
        // A killed run's stage carries a dead pid; the pre-pid name carries none.
        for entry in fs::read_dir(&base).into_iter().flatten().flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let stale = name == "stage"
                || name
                    .strip_prefix("stage-")
                    .and_then(|r| r.split('-').next())
                    .and_then(|pid| pid.parse::<u32>().ok())
                    .is_some_and(|pid| !Path::new("/proc").join(pid.to_string()).exists());
            if stale {
                fs::remove_dir_all(entry.path()).unwrap();
            }
        }
        let generation = GENERATION.fetch_add(1, Ordering::Relaxed);
        let stage = base.join(format!("stage-{}-{generation}", std::process::id()));
        if stage.exists() {
            fs::remove_dir_all(&stage).unwrap();
        }
        fs::create_dir_all(&stage).unwrap();
        let version = command(Path::new("swipl"), &stage, &["--version"], b"");
        version.require_ok("SWI pin");
        assert!(version.stdout.contains("version 10.0.2 "));
        copy_tree(&root().join("vendor/ape"), &stage);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let goal = format!(
            "working_directory(_,{}),[fit_to_plp],halt.",
            quote(stage.join("prolog/parser").to_str().unwrap())
        );
        let got = command(
            Path::new("swipl"),
            &stage,
            &[
                "-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
            ],
            b"",
        );
        assert_eq!(got.rc, 0, "APE stage: {}", got.describe());
        let goal = format!(
            "style_check(-discontiguous),qcompile({}),halt.",
            quote(
                stage
                    .join("prolog/lexicon/clex_lexicon.pl")
                    .to_str()
                    .unwrap()
            )
        );
        command(
            Path::new("swipl"),
            &stage,
            &[
                "-q", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
            ],
            b"",
        )
        .require_ok("Clex stage");
        Compiler { stage }
    });
    *weak = Arc::downgrade(&made);
    made
}
impl Compiler {
    pub fn run(&self, cwd: &Path, id: &str, mode: &str, ace: &str, table: Option<&[u8]>) -> Run {
        fs::create_dir_all(cwd).unwrap();
        let tab = cwd.join("vocabulary.tsv");
        let ulex = cwd.join("lexicon.ulex");
        if let Some(table) = table {
            write(&tab, table);
        }
        write(&ulex, ULEX);
        let script = self.stage.join("prolog/ace_to_pl.pl");
        let mut args = vec![
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            script.to_str().unwrap(),
            "-g",
            "main",
            "-t",
            "halt(9)",
            "--",
        ];
        if mode == "question" {
            args.push("question");
        }
        args.extend([
            self.stage.to_str().unwrap(),
            id,
            if table.is_some() {
                tab.to_str().unwrap()
            } else {
                "-"
            },
            ulex.to_str().unwrap(),
        ]);
        if mode == "proof" {
            args.push("proof");
        }
        command(Path::new("swipl"), cwd, &args, ace.as_bytes())
    }
    pub fn dump(&self, cwd: &Path, ace: &str) -> Run {
        fs::create_dir_all(cwd).unwrap();
        let ulex = cwd.join("lexicon.ulex");
        write(&ulex, ULEX);
        let driver = root().join("rust/ckc/prolog/drs_dump.pl");
        command(
            Path::new("swipl"),
            cwd,
            &[
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-s",
                driver.to_str().unwrap(),
                "-g",
                "main",
                "-t",
                "halt(9)",
                "--",
                self.stage.to_str().unwrap(),
                ulex.to_str().unwrap(),
            ],
            ace.as_bytes(),
        )
    }
}
pub fn certify(cwd: &Path, id: &str, ace: &str, dump: &str, pl: &str, table: Option<&str>) -> Run {
    write(cwd.join("input.ace"), ace);
    write(cwd.join("input.dump"), dump);
    write(cwd.join("input.pl"), pl);
    write(cwd.join("lexicon.ulex"), ULEX);
    let mut args = vec![
        "certify-one",
        "doc",
        id,
        "input.ace",
        "lexicon.ulex",
        "input.dump",
        "input.pl",
    ];
    if let Some(table) = table {
        write(cwd.join("vocabulary.tsv"), table);
        args.push("vocabulary.tsv");
    }
    native(cwd, &args)
}
