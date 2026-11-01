// Shared process + scratch helpers for the independently merged WS + ID2 suites.
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::Instant;

pub static SERIAL: Mutex<()> = Mutex::new(());
pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn bin() -> PathBuf {
    std::env::var_os("CKC_IMP_WS_ID2_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}
pub fn path(p: &Path) -> &str {
    p.to_str().unwrap()
}
pub fn sha(s: &str) -> String {
    Sha256::digest(s.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn quoted(s: &str) -> String {
    format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
}
pub fn atom(s: &str) -> String {
    if s.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        s.into()
    } else {
        quoted(s)
    }
}

pub struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new(name: &str) -> Self {
        let p = root().join(format!(
            "rust/target/imp-acceptance-ws-id2/{}-{name}",
            std::process::id()
        ));
        assert!(!p.exists(), "scratch collision: {}", p.display());
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
    pub fn file(&self, name: &str, bytes: impl AsRef<[u8]>) -> PathBuf {
        let p = self.0.join(name);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, bytes).unwrap();
        p
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

pub struct Run {
    pub rc: i32,
    pub stdout: String,
    pub stderr: String,
}
impl Run {
    pub fn refused(&self) -> bool {
        self.rc != 0 && !matches!(self.rc, -1 | 124 | 125 | 126 | 127 | 137)
    }
    pub fn names(&self, id: &str) -> bool {
        self.stderr.contains(id) || self.stdout.contains(id)
    }
    pub fn summary(&self) -> String {
        let clip = |s: &str| s.chars().take(400).collect::<String>();
        format!(
            "rc={} stdout={:?} stderr={:?}",
            self.rc,
            clip(&self.stdout),
            clip(&self.stderr)
        )
    }
}
pub fn run(program: &Path, cwd: &Path, args: &[&str], input: &[u8], label: &str) -> Run {
    // A watchdog is a harness failure, never a semantic refusal or a timing grade.
    let start = Instant::now();
    let mut child = Command::new("timeout")
        .args(["--kill-after=2", "120"])
        .arg(program)
        .args(args)
        .current_dir(cwd)
        .env("LANG", "C.UTF-8")
        .env("LC_ALL", "C.UTF-8")
        .env("CKC_JOBS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = child.stdin.take().unwrap().write_all(input);
    let out = child.wait_with_output().unwrap();
    let got = Run {
        rc: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8(out.stdout).unwrap(),
        stderr: String::from_utf8(out.stderr).unwrap(),
    };
    println!(
        "IMP {label}: {}; time={:.3}s",
        got.summary(),
        start.elapsed().as_secs_f64()
    );
    got
}
pub fn native(s: &Scratch, args: &[&str], label: &str) -> Run {
    run(&bin(), &s.0, args, b"", label)
}
#[derive(Default)]
pub struct Checks(Vec<String>);
impl Checks {
    pub fn require(&mut self, yes: bool, label: &str, got: &Run) {
        if !yes {
            self.0.push(format!("{label}: {}", got.summary()));
        }
    }
    pub fn finish(self, label: &str) {
        assert!(
            self.0.is_empty(),
            "{label}: {} failures\n{}",
            self.0.len(),
            self.0.join("\n")
        );
    }
}

pub fn substitute(text: String, from: &str, to: &str) -> String {
    assert_eq!(text.matches(from).count(), 1, "fixture anchor: {from}");
    text.replacen(from, to, 1)
}
