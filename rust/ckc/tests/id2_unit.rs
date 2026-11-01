// ID2 P1–P7. Oracle = frozen id2 contract + Decisions Schema evolution; implementation unread.
// CKC_ID2_TEST_BIN = candidate; CKC_ID2_MIGRATION / CKC_ID2_CHURN = external script overrides.
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

const SEED: &str = "9e8455bd";
const DOSE: &str = "tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl";
static SERIAL: Mutex<()> = Mutex::new(());
static NEXT: AtomicUsize = AtomicUsize::new(0);
const GIT_ENV: [(&str, &str); 10] = [
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_CONFIG_SYSTEM", "/dev/null"),
    ("GIT_DEFAULT_HASH", "sha1"),
    ("GIT_AUTHOR_NAME", "fixture"),
    ("GIT_AUTHOR_EMAIL", "fixture@localhost"),
    ("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+00:00"),
    ("GIT_COMMITTER_NAME", "fixture"),
    ("GIT_COMMITTER_EMAIL", "fixture@localhost"),
    ("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+00:00"),
    ("LC_ALL", "C.UTF-8"),
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn program() -> PathBuf {
    std::env::var_os("CKC_ID2_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}
fn script(var: &str, name: &str) -> PathBuf {
    std::env::var_os(var)
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join(".agent/archive/migrations").join(name))
}
fn text(path: &Path) -> String {
    path.to_str().unwrap().to_owned()
}
fn read(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).unwrap()
}
fn write(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> PathBuf {
    let path = path.as_ref();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
    path.to_owned()
}
fn sha(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn atom(s: &str) -> String {
    if s.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}
fn quote(path: &Path) -> String {
    format!(
        "'{}'",
        text(path).replace('\\', "\\\\").replace('\'', "\\'")
    )
}
fn swap_once(input: &str, from: &str, to: &str) -> String {
    assert_eq!(input.matches(from).count(), 1, "plant anchor: {from}");
    input.replacen(from, to, 1)
}

struct Scratch(PathBuf);
impl Scratch {
    fn new(name: &str) -> Self {
        let path = root().join(format!(
            "rust/target/id2-unit/{}-{}-{name}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!path.exists(), "scratch collision: {}", path.display());
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn file(&self, name: &str, bytes: impl AsRef<[u8]>) -> PathBuf {
        write(self.0.join(name), bytes)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Out {
    rc: i32,
    stdout: String,
    stderr: String,
}
impl Out {
    fn ok(stdout: impl Into<String>) -> Self {
        Self {
            rc: 0,
            stdout: stdout.into(),
            stderr: String::new(),
        }
    }
    fn err(rc: i32, stderr: impl Into<String>) -> Self {
        Self {
            rc,
            stdout: String::new(),
            stderr: stderr.into(),
        }
    }
}
fn run(bin: &Path, cwd: &Path, args: &[String], env: &[(&str, String)]) -> Out {
    let out = Command::new("timeout")
        .args(["--kill-after=5", "1800"])
        .arg(bin)
        .args(args)
        .current_dir(cwd)
        .envs(GIT_ENV)
        .env("LANG", "C.UTF-8")
        .env("CKC_JOBS", "1")
        .env("TZ", "UTC")
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .envs(env.iter().map(|(k, v)| (k, v)))
        .stdin(Stdio::null())
        .output()
        .unwrap();
    Out {
        rc: out.status.code().unwrap_or(128),
        stdout: String::from_utf8(out.stdout).unwrap(),
        stderr: String::from_utf8(out.stderr).unwrap(),
    }
}
fn native(cwd: &Path, args: &[&str]) -> Out {
    run(
        &program(),
        cwd,
        &args.iter().map(|s| (*s).into()).collect::<Vec<_>>(),
        &[],
    )
}
fn git(cwd: &Path, args: &[&str]) -> Out {
    let mut all = vec!["-c".into(), "commit.gpgsign=false".into()];
    all.extend(args.iter().map(|s| (*s).into()));
    run(Path::new("git"), cwd, &all, &[])
}
fn git_ok(cwd: &Path, args: &[&str]) -> String {
    let out = git(cwd, args);
    assert_eq!(
        (out.rc, out.stderr.as_str()),
        (0, ""),
        "git {args:?}: {out:?}"
    );
    out.stdout
}
fn commit(cwd: &Path, message: &str) -> String {
    git_ok(cwd, &["add", "-A"]);
    git_ok(cwd, &["commit", "-q", "-m", message]);
    git_ok(cwd, &["rev-parse", "HEAD"]).trim().into()
}
fn seed_file(path: &str) -> String {
    git_ok(&root(), &["show", &format!("{SEED}:{path}")])
}
fn sorted(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut paths: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    paths
}
fn files(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    for path in sorted(dir) {
        if path.is_dir() {
            result.extend(files(&path));
        } else {
            result.push(path);
        }
    }
    result
}
fn pl_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    sorted(dir)
        .into_iter()
        .filter(|p| p.extension().is_some_and(|e| e == "pl"))
        .collect()
}

#[derive(Default)]
struct Cases {
    count: usize,
    failed: Vec<String>,
}
impl Cases {
    fn check(&mut self, label: &str, ok: bool, detail: impl AsRef<str>) {
        self.count += 1;
        println!(
            "{label}: {} {}",
            if ok { "ok" } else { "FAIL" },
            detail.as_ref()
        );
        if !ok {
            self.failed.push(format!("{label}: {}", detail.as_ref()));
        }
    }
    fn same(&mut self, label: &str, got: Out, want: Out) {
        let ok = got == want;
        let preview = |s: &str| {
            if s.chars().count() > 500 {
                format!(
                    "{}… ({} bytes, sha256={})",
                    s.chars().take(500).collect::<String>(),
                    s.len(),
                    sha(s)
                )
            } else {
                s.to_owned()
            }
        };
        self.check(
            label,
            ok,
            format!(
                "rc={} stdout={:?} stderr={:?}{}",
                got.rc,
                preview(&got.stdout),
                preview(&got.stderr),
                if ok {
                    String::new()
                } else {
                    format!(
                        "; want rc={} stdout={:?} stderr={:?}",
                        want.rc,
                        preview(&want.stdout),
                        preview(&want.stderr)
                    )
                }
            ),
        );
    }
    fn finish(self) {
        assert!(
            self.failed.is_empty(),
            "{} of {} comparisons failed:\n{}",
            self.failed.len(),
            self.count,
            self.failed.join("\n")
        );
        println!("id2: {} comparisons ok", self.count);
    }
}

#[path = "../../../tests/id2-unit/inventory.rs"]
mod inventory;
#[path = "../../../tests/id2-unit/migration.rs"]
mod migration;
#[path = "../../../tests/id2-unit/readers.rs"]
mod readers;
#[path = "../../../tests/id2-unit/recipe.rs"]
mod recipe;
#[path = "../../../tests/id2-unit/release.rs"]
mod release;
