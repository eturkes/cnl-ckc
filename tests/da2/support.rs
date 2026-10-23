use super::term::quote;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
pub fn sha(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn read(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path.as_ref()).unwrap_or_else(|e| panic!("{}: {e}", path.as_ref().display()))
}
pub fn write(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) {
    fs::create_dir_all(path.as_ref().parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}
pub fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for e in fs::read_dir(from).unwrap() {
        let e = e.unwrap();
        assert!(
            !e.file_type().unwrap().is_symlink(),
            "copy source symlink {}",
            e.path().display()
        );
        if e.file_type().unwrap().is_dir() {
            copy_tree(&e.path(), &to.join(e.file_name()));
        } else {
            fs::copy(e.path(), to.join(e.file_name())).unwrap();
        }
    }
}
pub fn files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            out.extend(files(&e.path()));
        } else {
            out.push(e.path());
        }
    }
    out.sort();
    out
}
static NEXT: AtomicUsize = AtomicUsize::new(0);
pub struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new(label: &str) -> Self {
        let p = root().join(format!(
            "rust/target/da2/{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!p.exists(), "scratch collision {}", p.display());
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
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
        assert!(self.success(), "{label}: {}", self.summary());
    }
    pub fn summary(&self) -> String {
        fn clip(s: &str) -> String {
            s.chars().take(700).collect()
        }
        format!(
            "rc={} stdout={:?} stderr={:?}",
            self.rc,
            clip(&self.stdout),
            clip(&self.stderr)
        )
    }
}
pub fn run(bin: &Path, cwd: &Path, args: &[&str], input: &[u8]) -> Run {
    let mut child = Command::new("timeout")
        .args(["--kill-after=2", "120"])
        .arg(bin)
        .args(args)
        .current_dir(cwd)
        .env("LC_ALL", "C.UTF-8")
        .env("CKC_JOBS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = child.stdin.take().unwrap().write_all(input);
    let out = child.wait_with_output().unwrap();
    Run {
        rc: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8(out.stdout).unwrap(),
        stderr: String::from_utf8(out.stderr).unwrap(),
    }
}
pub fn native(cwd: &Path, args: &[&str]) -> Run {
    let bin = std::env::var_os("CKC_DA2_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"));
    run(&bin, cwd, args, b"")
}
#[derive(Default)]
pub struct Checks {
    pub total: usize,
    pub failures: Vec<String>,
}
impl Checks {
    pub fn require(&mut self, yes: bool, detail: impl Into<String>) {
        self.total += 1;
        if !yes {
            self.failures.push(detail.into());
        }
    }
    pub fn bytes(&mut self, got: &str, want: &str, label: &str) {
        let i = got
            .lines()
            .zip(want.lines())
            .position(|(a, b)| a != b)
            .unwrap_or(got.lines().count().min(want.lines().count()));
        self.require(
            got == want,
            format!(
                "{label}: line {} got {:?}; want {:?}; bytes {} / {}",
                i + 1,
                got.lines().nth(i),
                want.lines().nth(i),
                got.len(),
                want.len()
            ),
        );
    }
    pub fn output(&mut self, got: &Run, want: &Run, label: &str) {
        if got.rc == want.rc && got.stderr == want.stderr {
            self.bytes(&got.stdout, &want.stdout, label);
        } else {
            self.require(
                false,
                format!("{label}: got {}; want {}", got.summary(), want.summary()),
            );
        }
    }
    pub fn finish(self, label: &str) {
        println!(
            "DA2 {label}: checks={} failures={}",
            self.total,
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "DA2 {label}: {} failures ({} checks ran)\n{}",
            self.failures.len(),
            self.total,
            self.failures.join("\n")
        );
    }
}

pub struct Compiler {
    _scratch: Scratch,
    pub stage: PathBuf,
}
impl Compiler {
    fn new() -> Self {
        let scratch = Scratch::new("stage");
        let version = run(Path::new("swipl"), &scratch.0, &["--version"], b"");
        version.require_ok("SWI pin");
        assert!(version.stdout.contains("version 10.0.2 "));
        let stage = scratch.0.join("ape");
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
        let built = run(
            Path::new("swipl"),
            &scratch.0,
            &[
                "-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
            ],
            b"",
        );
        assert_eq!(built.rc, 0, "stage: {}", built.summary());
        let goal = format!(
            "style_check(-discontiguous),qcompile({}),halt.",
            quote(
                stage
                    .join("prolog/lexicon/clex_lexicon.pl")
                    .to_str()
                    .unwrap()
            )
        );
        run(
            Path::new("swipl"),
            &scratch.0,
            &[
                "-q", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
            ],
            b"",
        )
        .require_ok("Clex stage");
        Self {
            _scratch: scratch,
            stage,
        }
    }
    pub fn compile(
        &self,
        cwd: &Path,
        id: &str,
        ace: &[u8],
        ulex: Option<&Path>,
        table: Option<&Path>,
        mode: &str,
    ) -> Run {
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
            table.map_or("-", |p| p.to_str().unwrap()),
            ulex.map_or("-", |p| p.to_str().unwrap()),
        ]);
        if mode == "proof" {
            args.push("proof");
        }
        run(Path::new("swipl"), cwd, &args, ace)
    }
    pub fn custom(&self, cwd: &Path, tail: &[String], ace: &[u8]) -> Run {
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
        args.extend(tail.iter().map(String::as_str));
        run(Path::new("swipl"), cwd, &args, ace)
    }
    pub fn dump(&self, cwd: &Path, ace: &[u8], ulex: Option<&Path>, driver: Option<&Path>) -> Run {
        let current = root().join("rust/ckc/prolog/drs_dump.pl");
        let script = driver.unwrap_or(&current);
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
            self.stage.to_str().unwrap(),
        ];
        if let Some(p) = ulex {
            args.push(p.to_str().unwrap());
        }
        run(Path::new("swipl"), cwd, &args, ace)
    }
}
pub fn compiler() -> Arc<Compiler> {
    static COMPILER: OnceLock<Mutex<Weak<Compiler>>> = OnceLock::new();
    let mut weak = COMPILER
        .get_or_init(|| Mutex::new(Weak::new()))
        .lock()
        .unwrap();
    if let Some(shared) = weak.upgrade() {
        return shared;
    }
    let shared = Arc::new(Compiler::new());
    *weak = Arc::downgrade(&shared);
    shared
}
pub fn certify(cwd: &Path, id: &str, ace: &str, dump: &str, pl: &str, table: Option<&str>) -> Run {
    write(cwd.join("input.ace"), ace);
    write(cwd.join("input.dump"), dump);
    write(cwd.join("input.pl"), pl);
    write(cwd.join("lexicon.ulex"), super::cases::ULEX);
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
