use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

const DOCUMENT: &str = include_str!("document.pl.in");
const QUERY: &str = include_str!("query.pl.in");
static SERIAL: AtomicUsize = AtomicUsize::new(0);

pub fn sha(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes.as_ref())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
fn digest(bytes: impl AsRef<[u8]>) -> String {
    let hex = sha(bytes);
    if hex.as_bytes()[0].is_ascii_lowercase() {
        hex
    } else {
        format!("'{hex}'")
    }
}
pub fn document(id: &str, clauses: &[String]) -> String {
    DOCUMENT.replace("@DOC@", id).replace(
        "@CLAUSES@",
        &clauses
            .iter()
            .map(|s| format!("{s}.\n"))
            .collect::<String>(),
    )
}
pub fn query(goal: &str, wh: bool) -> String {
    QUERY
        .replace("@GOAL@", goal)
        .replace("@ANSWERS@", if wh { "[answer(A,wh(what))]" } else { "[]" })
}
pub fn answer(query: &str, result: &str) -> String {
    format!(
        "% cs0 answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,cs0,query_sha256({}),result({result})).\n",
        digest(query)
    )
}
pub fn trace(query: &str, answer: &str, result: &str) -> String {
    format!(
        "% cs0 traced against the loaded composition by ckc queries; do not edit.\n'$guideline_traces'(v1,cs0,query_sha256({}),answers_sha256({}),result({result})).\n",
        digest(query),
        digest(answer)
    )
}
pub fn node(clause: &str, children: &[String]) -> String {
    format!(
        "clause(sentence(cs0,1),clause_sha256({}),[{}])",
        digest(format!("{clause}.\n")),
        children.join(",")
    )
}
pub fn proved(clause: &str, children: &[String]) -> String {
    format!("yes(proved([{}]))", node(clause, children))
}
pub fn proof(heads: &[String]) -> String {
    format!(
        "'$guideline_proof'(cs0,1,variant(1),witness([]),prove([{}])).\n",
        heads.join(",")
    )
}

#[derive(Debug, PartialEq, Eq)]
pub struct Out {
    pub rc: i32,
    pub stdout: String,
    pub stderr: String,
}
impl Out {
    pub fn ok(stdout: impl Into<String>) -> Self {
        Self {
            rc: 0,
            stdout: stdout.into(),
            stderr: String::new(),
        }
    }
    pub fn error(stderr: impl Into<String>) -> Self {
        Self {
            rc: 1,
            stdout: String::new(),
            stderr: stderr.into(),
        }
    }
    fn summary(&self) -> String {
        format!(
            "rc={} stdout={:?} stderr={:?}",
            self.rc, self.stdout, self.stderr
        )
    }
}

pub struct Cases {
    pub dir: PathBuf,
    row: &'static str,
    count: usize,
    failures: Vec<String>,
}
impl Cases {
    pub fn new(row: &'static str) -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/cs0");
        let dir = root.join(format!(
            ".run-{}-{}-{row}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).unwrap();
        Self {
            dir,
            row,
            count: 0,
            failures: Vec::new(),
        }
    }
    pub fn write(&self, name: &str, text: &str) {
        fs::write(self.dir.join(name), text).unwrap();
    }
    pub fn install(&self, clauses: &[String]) {
        self.write("doc.pl", &document("cs0", clauses));
        self.write("payload.pl", "");
        self.write("manifest.tsv", include_str!("manifest.tsv.in"));
    }
    pub fn run(&self, args: &[&str]) -> Out {
        let binary = std::env::var_os("CKC_CS0_TEST_BIN")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
        let mut child = Command::new(binary)
            .args(args)
            .current_dir(&self.dir)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut out = child.stdout.take().unwrap();
        let mut err = child.stderr.take().unwrap();
        let stdout = std::thread::spawn(move || {
            let mut bytes = Vec::new();
            out.read_to_end(&mut bytes).unwrap();
            bytes
        });
        let stderr = std::thread::spawn(move || {
            let mut bytes = Vec::new();
            err.read_to_end(&mut bytes).unwrap();
            bytes
        });
        let deadline = Instant::now() + Duration::from_secs(120);
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break status;
            }
            if Instant::now() >= deadline {
                let _ = child.kill();
                let _ = child.wait();
                panic!("CS0 {} exceeded 120 s: {args:?}", self.row);
            }
            std::thread::sleep(Duration::from_millis(1));
        };
        Out {
            rc: status.code().unwrap_or(-1),
            stdout: String::from_utf8(stdout.join().unwrap()).unwrap(),
            stderr: String::from_utf8(stderr.join().unwrap()).unwrap(),
        }
    }
    pub fn compare(&mut self, label: &str, got: Out, expected: Out) {
        self.count += 1;
        if got != expected {
            self.failures.push(format!(
                "{label}: got {}; expected {}",
                got.summary(),
                expected.summary()
            ));
        }
    }
    pub fn expect(&mut self, label: &str, args: &[&str], expected: Out) {
        let got = self.run(args);
        self.compare(label, got, expected);
    }
    pub fn finish(self) {
        println!(
            "CS0 {} checks={} passed={} failures={}",
            self.row,
            self.count,
            self.count - self.failures.len(),
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "CS0 {}\n{}",
            self.row,
            self.failures.join("\n")
        );
    }
}
impl Drop for Cases {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).unwrap();
    }
}
