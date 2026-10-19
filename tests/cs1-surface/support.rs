use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
pub fn program() -> PathBuf {
    std::env::var_os("CKC_CS1_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}
pub fn sha(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn write(path: &Path, bytes: impl AsRef<[u8]>) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}
pub fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        assert!(!entry.file_type().unwrap().is_symlink());
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}
static NEXT: AtomicU64 = AtomicU64::new(0);
pub struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new(label: &str) -> Self {
        let path = root().join(format!(
            "rust/target/cs1-surface/{label}-{}-{}",
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
            .unwrap_or_else(|e| eprintln!("cs1 cleanup {}: {e}", self.0.display()));
    }
}
#[derive(Clone, Debug)]
pub struct Run {
    pub rc: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}
impl From<Output> for Run {
    fn from(out: Output) -> Self {
        Self {
            rc: out.status.code(),
            stdout: String::from_utf8(out.stdout).unwrap(),
            stderr: String::from_utf8(out.stderr).unwrap(),
        }
    }
}
impl Run {
    pub fn ok(&self) -> bool {
        self.rc == Some(0) && self.stderr.is_empty()
    }
    pub fn require_ok(&self, label: &str) {
        assert!(self.ok(), "{label}: {}", self.describe());
    }
    pub fn describe(&self) -> String {
        let clip = |s: &str| {
            let p: String = s.chars().take(700).collect();
            if p.len() == s.len() {
                p
            } else {
                format!("{p}… ({} bytes)", s.len())
            }
        };
        format!(
            "rc={:?}, stdout={:?}, stderr={:?}",
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
    input: &str,
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
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap().into()
}
pub fn command(bin: &Path, cwd: &Path, args: &[&str], input: &str) -> Run {
    command_env(bin, cwd, args, input, &[])
}
pub fn git(cwd: &Path, args: &[&str]) -> String {
    let out = command(Path::new("git"), cwd, args, "");
    assert_eq!(out.rc, Some(0), "git {args:?}: {}", out.describe());
    out.stdout
}
pub fn native(cwd: &Path, args: &[&str]) -> Run {
    command(&program(), cwd, args, "")
}
#[derive(Default)]
pub struct Checks {
    pub count: usize,
    pub failures: Vec<String>,
}
impl Checks {
    pub fn require(&mut self, yes: bool, detail: impl Into<String>) {
        self.count += 1;
        if !yes {
            self.failures.push(detail.into());
        }
    }
    pub fn equal(&mut self, actual: &str, expected: &str, label: &str) {
        let a: Vec<_> = actual.lines().collect();
        let e: Vec<_> = expected.lines().collect();
        let i = a
            .iter()
            .zip(&e)
            .position(|(x, y)| x != y)
            .unwrap_or(a.len().min(e.len()));
        self.require(
            actual == expected,
            format!(
                "{label}: line {} expected {:?}, got {:?}; bytes {} vs {}",
                i + 1,
                e.get(i),
                a.get(i),
                expected.len(),
                actual.len()
            ),
        );
    }
    pub fn finish(self, label: &str) {
        println!(
            "{label}: {} checks, {} failures",
            self.count,
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "{label}:\n{}",
            self.failures.join("\n")
        );
    }
}
