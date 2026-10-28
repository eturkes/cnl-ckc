use super::model::{BASE, quoted};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

pub fn program() -> PathBuf {
    std::env::var_os("CKC_SGW_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}

pub fn sha(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

pub fn write(path: impl AsRef<Path>, data: impl AsRef<[u8]>) {
    fs::create_dir_all(path.as_ref().parent().unwrap()).unwrap();
    fs::write(path, data).unwrap();
}

pub fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for e in fs::read_dir(from).unwrap() {
        let e = e.unwrap();
        assert!(
            !e.file_type().unwrap().is_symlink(),
            "{}",
            e.path().display()
        );
        if e.file_type().unwrap().is_dir() {
            copy_tree(&e.path(), &to.join(e.file_name()));
        } else {
            fs::copy(e.path(), to.join(e.file_name())).unwrap();
        }
    }
}

pub struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new(label: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let p = root().join(format!(
            "rust/target/sgw/{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!p.exists(), "scratch collision {}", p.display());
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
    pub fn file(&self, name: &str, bytes: impl AsRef<[u8]>) -> PathBuf {
        let p = self.0.join(name);
        write(&p, bytes);
        p
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0)
            .unwrap_or_else(|e| eprintln!("SGW cleanup {}: {e}", self.0.display()));
    }
}

#[derive(Debug)]
pub struct Run {
    pub rc: i32,
    pub out: Vec<u8>,
    pub err: Vec<u8>,
}
impl Run {
    pub fn clean(&self) -> bool {
        self.rc == 0 && self.err.is_empty()
    }
    pub fn description(&self) -> String {
        format!(
            "rc={} stdout={:?} stderr={:?}",
            self.rc,
            String::from_utf8_lossy(&self.out)
                .chars()
                .take(200)
                .collect::<String>(),
            String::from_utf8_lossy(&self.err)
                .chars()
                .take(600)
                .collect::<String>()
        )
    }
    pub fn require(&self, label: &str) {
        assert!(self.clean(), "{label}: {}", self.description());
    }
}

pub fn run(bin: &Path, cwd: &Path, args: &[&str], input: &[u8]) -> Run {
    let mut child = Command::new("timeout")
        .args(["--kill-after=2", "180"])
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
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    let output = child.wait_with_output().unwrap();
    Run {
        rc: output.status.code().unwrap_or(-1),
        out: output.stdout,
        err: output.stderr,
    }
}

pub fn native(cwd: &Path, args: &[&str]) -> Run {
    run(&program(), cwd, args, b"")
}
pub fn swi(cwd: &Path, goal: &str) -> Run {
    run(
        Path::new("swipl"),
        cwd,
        &[
            "-q", "-f", "none", "-F", "none", "-g", goal, "-t", "halt(9)",
        ],
        b"",
    )
}
pub fn git(args: &[&str]) -> Vec<u8> {
    let got = run(Path::new("git"), &root(), args, b"");
    got.require("git");
    got.out
}
pub fn tracked(path: &str) -> Vec<String> {
    String::from_utf8(git(&["ls-files", path]))
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}
pub fn baseline(path: &str) -> Vec<u8> {
    git(&["show", &format!("{BASE}:{path}")])
}
pub fn corpus() -> Vec<String> {
    let paths = tracked(":(glob)guidelines/*/pl/*.pl");
    assert!(!paths.is_empty(), "positive corpus inventory");
    paths
}

#[derive(Default)]
pub struct Checks {
    pub total: usize,
    pub failures: Vec<String>,
}
impl Checks {
    pub fn require(&mut self, yes: bool, label: impl Into<String>) {
        self.total += 1;
        if !yes {
            self.failures.push(label.into());
        }
    }
    pub fn bytes(&mut self, got: &[u8], want: &[u8], label: &str) {
        let at = got
            .iter()
            .zip(want)
            .position(|(a, b)| a != b)
            .unwrap_or(got.len().min(want.len()));
        self.require(
            got == want,
            format!(
                "{label}: byte {at}, got {:?}, expected {:?}; lengths {}/{}",
                String::from_utf8_lossy(&got[at.saturating_sub(25)..got.len().min(at + 50)]),
                String::from_utf8_lossy(&want[at.saturating_sub(25)..want.len().min(at + 50)]),
                got.len(),
                want.len()
            ),
        );
    }
    pub fn finish(self, label: &str) {
        println!(
            "SGW {label}: {} checks; {} failures",
            self.total,
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "SGW {label}: {} failures\n{}",
            self.failures.len(),
            self.failures
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

// No process-global cache: one stage per emission test, owned until its last compiler call.
pub struct Compiler {
    pub scratch: Scratch,
}
impl Compiler {
    pub fn new() -> Self {
        let scratch = Scratch::new("compiler");
        let version = swi(
            &scratch.0,
            "current_prolog_flag(version_data,swi(10,0,2,[])),halt.",
        );
        version.require("SWI 10.0.2 pin");
        copy_tree(&root().join("vendor/ape"), &scratch.0);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            scratch.0.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let goal = format!(
            "working_directory(_,{}),[fit_to_plp],halt.",
            quoted(scratch.0.join("prolog/parser").to_str().unwrap())
        );
        let stage = swi(&scratch.0, &goal);
        assert_eq!(stage.rc, 0, "APE stage {}", stage.description());
        let goal = format!(
            "style_check(-discontiguous),qcompile({}),halt.",
            quoted(
                scratch
                    .0
                    .join("prolog/lexicon/clex_lexicon.pl")
                    .to_str()
                    .unwrap()
            )
        );
        swi(&scratch.0, &goal).require("Clex stage");
        Self { scratch }
    }
    pub fn compile(
        &self,
        cwd: &Path,
        id: &str,
        ace: &str,
        ulex: Option<&Path>,
        table: Option<&Path>,
        mode: &str,
    ) -> Run {
        let path = self.scratch.0.join("prolog/ace_to_pl.pl");
        let mut args = vec![
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            path.to_str().unwrap(),
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
            self.scratch.0.to_str().unwrap(),
            id,
            table.map_or("-", |p| p.to_str().unwrap()),
            ulex.map_or("-", |p| p.to_str().unwrap()),
        ]);
        if mode == "proof" {
            args.push("proof");
        }
        run(Path::new("swipl"), cwd, &args, ace.as_bytes())
    }
    pub fn dump(&self, cwd: &Path, ace: &str, ulex: &Path) -> Run {
        let driver = root().join("rust/ckc/prolog/drs_dump.pl");
        run(
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
                self.scratch.0.to_str().unwrap(),
                ulex.to_str().unwrap(),
            ],
            ace.as_bytes(),
        )
    }
}
