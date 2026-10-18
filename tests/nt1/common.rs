use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

pub const INDICATORS: [(&str, usize); 19] = [
    ("guideline_schema_version", 1),
    ("guideline_document", 4),
    ("guideline_entity", 4),
    ("guideline_cardinality", 5),
    ("guideline_event", 3),
    ("guideline_arg", 4),
    ("guideline_pp", 4),
    ("guideline_property", 4),
    ("guideline_operator", 3),
    ("guideline_interval", 6),
    ("guideline_recurrence", 4),
    ("guideline_frequency", 5),
    ("guideline_order", 4),
    ("guideline_recurrence_window", 7),
    ("guideline_range", 3),
    ("guideline_grade", 5),
    ("guideline_preference", 4),
    ("guideline_qualifier", 4),
    ("guideline_unit", 3),
];
static SERIAL: AtomicUsize = AtomicUsize::new(0);

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
pub fn fixture(name: &str) -> String {
    fs::read_to_string(root().join("tests/nt1").join(name)).unwrap()
}
pub fn sha(bytes: impl AsRef<[u8]>) -> String {
    use sha2::Digest;
    sha2::Sha256::digest(bytes.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn atom(s: &str) -> String {
    if s.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}
pub fn digest(bytes: Option<&str>) -> String {
    bytes.map_or_else(|| "none".into(), |b| format!("sha256({})", atom(&sha(b))))
}
pub fn pair(name: &str, arity: usize) -> String {
    format!(":- multifile({name}/{arity}).\n:- discontiguous({name}/{arity}).\n")
}
pub fn declarations() -> String {
    INDICATORS.iter().map(|&(n, a)| pair(n, a)).collect()
}
pub fn document_text(id: &str, ace: &str, table: Option<&str>, clauses: &[String]) -> String {
    format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex(none),vocabulary({})).\n% S1: {}\n{}",
        declarations(),
        atom(id),
        atom(&sha(ace)),
        digest(table),
        ace.trim_end(),
        clauses
            .iter()
            .map(|s| format!("{s}.\n"))
            .collect::<String>()
    )
}
pub fn document(id: &str, clauses: &[String]) -> String {
    document_text(id, "A structural clause.\n", None, clauses)
}
pub fn query_text(id: &str, ace: &str, table: Option<&str>, goal: &str, answers: &str) -> String {
    format!(
        "% {id} compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,{},ace_sha256({}),ulex(none),vocabulary({})).\n% Q1: {}\n'$guideline_query_projection'(goal({goal}),answers({answers})).\n",
        atom(id),
        atom(&sha(ace)),
        digest(table),
        ace.trim_end()
    )
}
pub fn query(goal: &str, answers: &str) -> String {
    query_text("numeric", "Is there a result?\n", None, goal, answers)
}
pub fn conjunction(goals: &[String]) -> String {
    match goals {
        [only] => only.clone(),
        [first, rest @ ..] => format!("','({first},{})", conjunction(rest)),
        [] => panic!("nonempty conjunction"),
    }
}
pub fn answers(q: &str, result: &str) -> String {
    format!(
        "% numeric answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,numeric,query_sha256({}),result({result})).\n",
        atom(&sha(q))
    )
}
pub fn traces(q: &str, a: &str, result: &str) -> String {
    format!(
        "% numeric traced against the loaded composition by ckc queries; do not edit.\n'$guideline_traces'(v1,numeric,query_sha256({}),answers_sha256({}),result({result})).\n",
        atom(&sha(q)),
        atom(&sha(a))
    )
}
pub fn node(id: &str, clause: &str, children: &[String]) -> String {
    format!(
        "clause(sentence({},1),clause_sha256({}),[{}])",
        atom(id),
        atom(&sha(format!("{clause}.\n"))),
        children.join(",")
    )
}
pub fn builtin(goal: &str) -> String {
    format!("builtin({goal})")
}
pub fn proof(id: &str, witness: &[String], heads: &[String]) -> String {
    format!(
        "'$guideline_proof'({},1,variant(1),witness([{}]),prove([{}])).\n",
        atom(id),
        witness.join(","),
        heads.join(",")
    )
}
pub fn gid(kind: &str, id: &str, slot: usize, deps: &str) -> String {
    format!("'$guideline_id'({kind},{},1,ref({slot}),{deps})", atom(id))
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
    pub fn err(rc: i32, stderr: impl Into<String>) -> Self {
        Self {
            rc,
            stdout: String::new(),
            stderr: stderr.into(),
        }
    }
    fn summary(&self) -> String {
        let clip = |s: &str| {
            let prefix: String = s.chars().take(600).collect();
            if prefix.len() == s.len() {
                prefix
            } else {
                format!("{prefix}… ({} bytes)", s.len())
            }
        };
        format!(
            "rc={} stdout={:?} stderr={:?}",
            self.rc,
            clip(&self.stdout),
            clip(&self.stderr)
        )
    }
}
pub fn execute(command: &mut Command) -> Out {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let out = std::thread::spawn(move || {
        let mut b = Vec::new();
        stdout.read_to_end(&mut b).unwrap();
        b
    });
    let err = std::thread::spawn(move || {
        let mut b = Vec::new();
        stderr.read_to_end(&mut b).unwrap();
        b
    });
    let deadline = Instant::now() + Duration::from_secs(30);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("NT1 subprocess exceeded 30 s: {command:?}");
        }
        std::thread::sleep(Duration::from_millis(1));
    };
    Out {
        rc: status.code().unwrap_or(-1),
        stdout: String::from_utf8(out.join().unwrap()).unwrap(),
        stderr: String::from_utf8(err.join().unwrap()).unwrap(),
    }
}
pub struct Cases {
    pub dir: PathBuf,
    row: &'static str,
    pub total: usize,
    failed: usize,
    details: Vec<String>,
}
impl Cases {
    pub fn new(row: &'static str) -> Self {
        let dir = root().join(format!(
            "rust/target/nt1/{}-{}-{row}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).unwrap();
        Self {
            dir,
            row,
            total: 0,
            failed: 0,
            details: Vec::new(),
        }
    }
    pub fn write(&self, name: &str, text: &str) {
        fs::write(self.dir.join(name), text).unwrap();
    }
    pub fn run(&self, args: &[&str]) -> Out {
        let program = std::env::var_os("CKC_NT1_TEST_BIN")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
        execute(Command::new(program).args(args).current_dir(&self.dir))
    }
    pub fn compare(&mut self, label: &str, got: Out, want: Out) {
        self.total += 1;
        if got != want {
            self.failed += 1;
            let detail = format!("{label}: got {}; want {}", got.summary(), want.summary());
            if std::env::var_os("CKC_NT1_REPORT_ALL").is_some() {
                eprintln!("NT1FAIL {} {detail}", self.row);
            }
            if self.details.len() < 12 {
                self.details.push(detail);
            }
        }
    }
    pub fn expect(&mut self, label: &str, args: &[&str], want: Out) {
        let got = self.run(args);
        self.compare(label, got, want);
    }
    pub fn check(&mut self, label: &str, text: &str) {
        self.write("input.pl", text);
        self.expect(label, &["v1", "check", "input.pl"], Out::ok(""));
        self.expect(
            &format!("{label}/render"),
            &["v1", "render", "input.pl"],
            Out::ok(text),
        );
    }
    pub fn reject_at(&mut self, label: &str, text: &str, at: usize) {
        self.write("input.pl", text);
        let prefix = &text.as_bytes()[..at];
        let line = 1 + prefix.iter().filter(|&&b| b == b'\n').count();
        let col = prefix
            .iter()
            .rposition(|&b| b == b'\n')
            .map_or(at + 1, |p| at - p);
        self.expect(
            label,
            &["v1", "check", "input.pl"],
            Out::err(
                2,
                format!("ace_to_pl_error(check_load,noncanonical({line},{col})).\n"),
            ),
        );
    }
    pub fn mutation(&mut self, label: &str, text: &str, from: &str, to: &str) {
        assert_eq!(text.matches(from).count(), 1, "{label}: unique anchor");
        let changed = text.replacen(from, to, 1);
        let at = text
            .bytes()
            .zip(changed.bytes())
            .position(|(a, b)| a != b)
            .unwrap_or(text.len().min(changed.len()));
        self.reject_at(label, &changed, at);
    }
    pub fn answer(&mut self, label: &str, goal: &str, holds: bool) {
        let q = query(goal, "[]");
        self.write("query.pl", &q);
        self.expect(
            label,
            &["v1", "answer", "m.tsv", "query.pl"],
            Out::ok(answers(
                &q,
                if holds { "yes" } else { "no(finite_failure)" },
            )),
        );
    }
    pub fn finish(self) {
        println!(
            "NT1 {} checks={} passed={} failures={}",
            self.row,
            self.total,
            self.total - self.failed,
            self.failed
        );
        assert_eq!(
            self.failed,
            0,
            "NT1 {}: {} failures; first {} below (all {} checks executed)\n{}",
            self.row,
            self.failed,
            self.details.len(),
            self.total,
            self.details.join("\n")
        );
    }
}
impl Drop for Cases {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

pub fn install(c: &Cases, members: &[(&str, Vec<String>, String)]) {
    let mut manifest = String::new();
    for (i, (id, clauses, payload)) in members.iter().enumerate() {
        c.write(&format!("d{i}.pl"), &document(id, clauses));
        c.write(&format!("d{i}.proof"), payload);
        manifest.push_str(&format!("d{i}.pl\td{i}.proof\n"));
    }
    c.write("m.tsv", &manifest);
}
pub fn chain(c: &mut Cases, label: &str, q: &str, result: &str, trace_result: &str) {
    let a = answers(q, result);
    let t = traces(q, &a, trace_result);
    c.write("query.pl", q);
    c.write("answers.pl", &a);
    c.write("traces.pl", &t);
    c.expect(
        &format!("{label}/answer"),
        &["v1", "answer", "m.tsv", "query.pl"],
        Out::ok(&a),
    );
    c.expect(
        &format!("{label}/trace"),
        &["v1", "trace", "m.tsv", "query.pl", "answers.pl"],
        Out::ok(&t),
    );
    let demo = trace_result.starts_with("yes(proved([");
    let demo = demo
        || (trace_result.starts_with("solutions([sol(") && !trace_result.contains("unproved("));
    c.expect(
        &format!("{label}/trace-check"),
        &[
            "v1",
            "trace-check",
            "m.tsv",
            "query.pl",
            "answers.pl",
            "traces.pl",
        ],
        if demo {
            Out::ok(format!(
                "ckc: trace-check ok numeric nodes={}\n",
                trace_result.matches("clause(").count()
            ))
        } else {
            Out::err(1, "ace_to_pl_error(proof,trace_check(non_demo)).\n")
        },
    );
}
pub fn boolean(c: &mut Cases, label: &str, goal: &str, roots: Option<&[String]>) {
    let q = query(goal, "[]");
    if let Some(roots) = roots {
        assert!(!roots.is_empty());
        chain(
            c,
            label,
            &q,
            "yes",
            &format!("yes(proved([{}]))", roots.join(",")),
        );
    } else {
        chain(c, label, &q, "no(finite_failure)", "no(finite_failure)");
    }
}
