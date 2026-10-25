use super::oracle;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

pub const CASES: [&str; 2] = ["mixed", "sup-only"];
pub type Files = BTreeMap<String, Vec<u8>>;
static NEXT: AtomicU64 = AtomicU64::new(0);

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
pub fn program() -> PathBuf {
    std::env::var_os("CKC_SUP4_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}
pub fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}
pub fn write(path: &Path, bytes: impl AsRef<[u8]>) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}
pub fn sha(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn files(base: &Path) -> Files {
    fn walk(base: &Path, here: &Path, out: &mut Files) {
        for entry in fs::read_dir(here).unwrap() {
            let path = entry.unwrap().path();
            assert!(!path.is_symlink(), "fixture symlink: {}", path.display());
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                out.insert(
                    path.strip_prefix(base).unwrap().to_str().unwrap().into(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut out = Files::new();
    walk(base, base, &mut out);
    out
}
pub fn page<'a>(pages: &'a Files, path: &str) -> &'a str {
    std::str::from_utf8(
        pages
            .get(path)
            .unwrap_or_else(|| panic!("page missing: {path}")),
    )
    .unwrap()
}
pub fn response(out: &Output) -> (&str, &str) {
    std::str::from_utf8(&out.stdout)
        .unwrap()
        .split_once("\n\n")
        .expect("HTTP separator")
}
pub fn ok(out: &Output, label: &str) {
    assert!(
        out.status.success() && out.stderr.is_empty(),
        "{label}: rc={:?}; stdout={}; stderr={}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

pub struct Fixture {
    pub scratch: PathBuf,
    pub tree: PathBuf,
}
impl Fixture {
    pub fn new(case: &str) -> Self {
        let scratch = root().join(format!(
            "rust/target/sup4-ui/{}-{}-{}",
            case,
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!scratch.exists(), "scratch collision");
        let tree = scratch.join("tree");
        let source = root().join(format!("tests/sup4-ui/fixtures/{case}/tree"));
        for (name, bytes) in files(&source) {
            let name = name
                .strip_suffix(".pl.in")
                .map_or_else(|| name.clone(), |s| format!("{s}.pl"));
            write(&tree.join(name), bytes);
        }
        Self { scratch, tree }
    }
    pub fn guideline(&self) -> PathBuf {
        self.tree.join("guidelines/alpha")
    }
    pub fn command(&self, args: &[&str]) -> Output {
        Command::new(program())
            .args(args)
            .current_dir(&self.scratch)
            .env("GIT_CEILING_DIRECTORIES", &self.scratch)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("TMPDIR", &self.scratch)
            .output()
            .unwrap_or_else(|e| panic!("ckc {args:?}: {e}"))
    }
    pub fn render(&self) -> Files {
        let outdir = self.scratch.join("pages");
        if outdir.exists() {
            fs::remove_dir_all(&outdir).unwrap();
        }
        let out = self.command(&[
            "ui",
            "render",
            outdir.to_str().unwrap(),
            self.tree.to_str().unwrap(),
        ]);
        ok(&out, "fixture render");
        let pages = files(&outdir);
        assert_eq!(pages.len(), 7, "fixture file census");
        assert_eq!(
            pages.keys().filter(|s| s.ends_with(".html")).count(),
            6,
            "fixture page census"
        );
        pages
    }
    pub fn check(&self) -> Output {
        self.command(&["ui", "check", self.tree.to_str().unwrap()])
    }
    pub fn request(&self, path: &str) -> Output {
        self.command(&["ui", "request", "GET", path, self.tree.to_str().unwrap()])
    }
    pub fn table(&self, table: &str) {
        write(&self.guideline().join("vocabulary.tsv"), table);
    }
    pub fn remove_rows(&self) {
        let path = self.guideline().join("vocabulary.tsv");
        let raw = read(&path);
        assert!(
            !oracle::rows(&raw).is_empty(),
            "row-removal positive control"
        );
        if let Some(kept) = oracle::without_sup(&raw) {
            write(&path, kept);
        } else {
            fs::remove_file(path).unwrap();
        }
    }
    pub fn stale(&self, id: &str) {
        write(
            &self.guideline().join("audit/adjudication.tsv"),
            format!(
                "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n{id}\t{}\t\tapproved\tReader\t2026-01-01T00:00:00Z\tEarlier version.\n",
                "0".repeat(64)
            ),
        );
    }
    pub fn highlight(&self) {
        write(
            &self.guideline().join("align/a-2.tsv"),
            "1\tace\t26\ta-2\n1\tsrc\t21\ta-2\n",
        );
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.scratch)
            .unwrap_or_else(|e| eprintln!("SUP4 cleanup {}: {e}", self.scratch.display()));
    }
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
    pub fn equal(&mut self, actual: &[u8], expected: &[u8], label: &str) {
        self.count += 1;
        if actual != expected {
            let at = actual
                .iter()
                .zip(expected)
                .position(|(a, b)| a != b)
                .unwrap_or(actual.len().min(expected.len()));
            self.failures.push(format!(
                "{label}: offset {at}; expected {:?}, got {:?} ({} vs {} bytes)",
                String::from_utf8_lossy(&expected[at..expected.len().min(at + 100)]),
                String::from_utf8_lossy(&actual[at..actual.len().min(at + 100)]),
                expected.len(),
                actual.len()
            ));
        }
    }
    pub fn strip(
        &mut self,
        page: &str,
        path: &str,
        insertion: &str,
        label: &str,
    ) -> Option<String> {
        self.count += 1;
        match oracle::strip(page, path, insertion) {
            Ok(body) => Some(body),
            Err(why) => {
                self.failures.push(format!("{label}: {why}"));
                None
            }
        }
    }
    pub fn finish(self, label: &str) {
        eprintln!(
            "SUP4 {label}: {} comparisons, {} failures",
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
