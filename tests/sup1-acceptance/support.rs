use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

pub const INDICATORS: [(&str, usize); 22] = [
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
    ("guideline_exclusion", 1),
    ("guideline_supersession", 4),
    ("guideline_evaluation_date", 1),
];
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
pub fn atom(raw: &str) -> String {
    if raw == "-"
        || (raw.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
            && raw.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'))
    {
        raw.into()
    } else {
        format!("'{}'", raw.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}
pub fn digest(bytes: Option<&str>) -> String {
    bytes.map_or_else(|| "none".into(), |s| format!("sha256({})", atom(&sha(s))))
}
pub fn pair(name: &str, arity: usize) -> String {
    format!(":- multifile({name}/{arity}).\n:- discontiguous({name}/{arity}).\n")
}
pub fn declarations() -> String {
    INDICATORS.iter().map(|&(n, a)| pair(n, a)).collect()
}
pub fn document_text(id: &str, ace: &str, table: Option<&str>, bundles: &[Vec<String>]) -> String {
    assert_eq!(ace.lines().count(), bundles.len());
    let mut text = format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex(none),vocabulary({})).\n",
        declarations(),
        atom(id),
        atom(&sha(ace)),
        digest(table)
    );
    for (i, (line, clauses)) in ace.lines().zip(bundles).enumerate() {
        text.push_str(&format!("% S{}: {line}\n", i + 1));
        for clause in clauses {
            text.push_str(&format!("{clause}.\n"));
        }
    }
    text
}
pub fn document(id: &str, clauses: &[String]) -> String {
    document_text(id, "A structural clause.\n", None, &[clauses.to_vec()])
}
pub fn query(goal: &str, manifest: &str) -> String {
    format!(
        "% supersession compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,supersession,ace_sha256({}),ulex(none),vocabulary(none)).\n% Q1: Is there a result?\n'$guideline_query_projection'(goal({goal}),answers({manifest})).\n",
        atom(&sha("Is there a result?\n"))
    )
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
        "% supersession answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,supersession,query_sha256({}),result({result})).\n",
        atom(&sha(q))
    )
}
pub fn traces(q: &str, a: &str, result: &str) -> String {
    format!(
        "% supersession traced against the loaded composition by ckc queries; do not edit.\n'$guideline_traces'(v1,supersession,query_sha256({}),answers_sha256({}),result({result})).\n",
        atom(&sha(q)),
        atom(&sha(a))
    )
}
pub fn node(id: &str, s: usize, clause: &str, children: &[String]) -> String {
    format!(
        "clause(sentence({},{s}),clause_sha256({}),[{}])",
        atom(id),
        atom(&sha(format!("{clause}.\n"))),
        children.join(",")
    )
}
pub fn proof(id: &str, s: usize, k: usize, witness: &[String], heads: &[String]) -> String {
    format!(
        "'$guideline_proof'({},{s},variant({k}),witness([{}]),prove([{}])).\n",
        atom(id),
        witness.join(","),
        heads.join(",")
    )
}
// Model variables are single capitals; quoted source atoms stay opaque.
pub fn substitute(text: &str, mut replacement: impl FnMut(char) -> String) -> String {
    let (mut quoted, mut escaped) = (false, false);
    let mut out = String::new();
    for ch in text.chars() {
        if escaped {
            out.push(ch);
            escaped = false;
        } else if quoted && ch == '\\' {
            out.push(ch);
            escaped = true;
        } else if ch == '\'' {
            quoted = !quoted;
            out.push(ch);
        } else if !quoted && ch.is_ascii_uppercase() {
            out.push_str(&replacement(ch));
        } else {
            out.push(ch);
        }
    }
    out
}
pub fn canonical(text: &str) -> String {
    let mut variables = Vec::new();
    substitute(text, |ch| {
        let at = variables.iter().position(|&v| v == ch).unwrap_or_else(|| {
            variables.push(ch);
            variables.len() - 1
        });
        char::from(b'A' + u8::try_from(at).unwrap()).to_string()
    })
}
pub fn parts(text: &str) -> Vec<String> {
    let (mut depth, mut quoted, mut escaped, mut start) = (0, false, false, 0);
    let mut out = Vec::new();
    for (at, ch) in text.char_indices() {
        if escaped {
            escaped = false;
        } else if quoted && ch == '\\' {
            escaped = true;
        } else if ch == '\'' {
            quoted = !quoted;
        } else if !quoted {
            match ch {
                '(' | '[' => depth += 1,
                ')' | ']' => depth -= 1,
                ',' if depth == 0 => {
                    out.push(text[start..at].trim().into());
                    start = at + 1;
                }
                _ => {}
            }
        }
    }
    out.push(text[start..].trim().into());
    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn summary(&self) -> String {
        let clip = |s: &str| {
            let p: String = s.chars().take(450).collect();
            if p.len() == s.len() {
                p
            } else {
                format!("{p}… ({} bytes)", s.len())
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
pub fn command(bin: &Path, cwd: &Path, args: &[&str], input: &str) -> Out {
    let mut child = Command::new("timeout")
        .args(["--kill-after=2", "90"])
        .arg(bin)
        .args(args)
        .current_dir(cwd)
        .env("LC_ALL", "C.UTF-8")
        .env("CKC_JOBS", "1")
        .env("GIT_CEILING_DIRECTORIES", cwd)
        .env("TMPDIR", cwd)
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
    let output = child.wait_with_output().unwrap();
    Out {
        rc: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}
static SERIAL: AtomicUsize = AtomicUsize::new(0);
pub struct Cases {
    pub dir: PathBuf,
    pub row: &'static str,
    pub total: usize,
    failures: Vec<String>,
}
impl Cases {
    pub fn new(row: &'static str) -> Self {
        let dir = root().join(format!(
            "rust/target/sup1-acceptance/{}-{}-{row}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!dir.exists(), "scratch collision {}", dir.display());
        fs::create_dir_all(&dir).unwrap();
        Self {
            dir,
            row,
            total: 0,
            failures: Vec::new(),
        }
    }
    pub fn write(&self, name: &str, text: &str) {
        let path = self.dir.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    pub fn run(&self, args: &[&str]) -> Out {
        command(&program(), &self.dir, args, "")
    }
    pub fn compare(&mut self, label: &str, got: Out, want: Out) {
        self.total += 1;
        if got != want {
            let detail = format!("{label}: got {}; want {}", got.summary(), want.summary());
            if std::env::var_os("CKC_SUP1_REPORT_ALL").is_some() {
                eprintln!("SUP1FAIL {} {detail}", self.row);
            }
            self.failures.push(detail);
        }
    }
    pub fn expect(&mut self, label: &str, args: &[&str], want: Out) {
        self.compare(label, self.run(args), want);
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
        let prefix = &text.as_bytes()[..at];
        let line = 1 + prefix.iter().filter(|&&b| b == b'\n').count();
        let column = prefix
            .iter()
            .rposition(|&b| b == b'\n')
            .map_or(at + 1, |p| at - p);
        self.write("input.pl", text);
        self.expect(
            label,
            &["v1", "check", "input.pl"],
            Out::err(
                2,
                format!("ace_to_pl_error(check_load,noncanonical({line},{column})).\n"),
            ),
        );
    }
    pub fn mutation(&mut self, label: &str, text: &str, from: &str, to: &str) {
        assert_eq!(text.matches(from).count(), 1, "{label}: unique anchor");
        let changed = text.replacen(from, to, 1);
        assert_ne!(changed, text);
        let at = text
            .bytes()
            .zip(changed.bytes())
            .position(|(a, b)| a != b)
            .unwrap_or(text.len().min(changed.len()));
        self.reject_at(label, &changed, at);
    }
    pub fn finish(self) {
        println!(
            "SUP1 {} checks={} passed={} failures={}",
            self.row,
            self.total,
            self.total - self.failures.len(),
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "SUP1 {}: {} failures; first 12 (all {} checks executed)\n{}",
            self.row,
            self.failures.len(),
            self.total,
            self.failures
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
impl Drop for Cases {
    fn drop(&mut self) {
        if std::env::var_os("CKC_SUP1_KEEP_CASES").is_some() {
            eprintln!("SUP1 cases: {}", self.dir.display());
        } else {
            fs::remove_dir_all(&self.dir).unwrap();
        }
    }
}
#[derive(Clone)]
pub struct Member {
    pub id: String,
    pub pl: String,
    pub payload: String,
    pub clauses: Vec<(usize, String)>,
}
impl Member {
    pub fn raw(id: &str, clauses: &[String]) -> Self {
        let mut clauses: Vec<_> = clauses.iter().map(|s| canonical(s)).collect();
        // v1text::wf_bundle requires a clause even when this world's fact set is empty.
        if clauses.is_empty() {
            clauses.push(format!(
                "guideline_property(actual,{},fixture,pos)",
                atom(id)
            ));
        }
        Self {
            id: id.into(),
            pl: document(id, &clauses),
            payload: String::new(),
            clauses: clauses.into_iter().map(|s| (1, s)).collect(),
        }
    }
    pub fn trace(&self, at: usize, children: &[String]) -> String {
        let (s, clause) = &self.clauses[at];
        node(&self.id, *s, clause, children)
    }
}
pub fn legacy_binary(c: &Cases) -> bool {
    c.write(
        "schema-probe.pl",
        &document("probe", &["guideline_event(actual,e,wait)".into()]),
    );
    let probe = c.run(&["v1", "check", "schema-probe.pl"]);
    let legacy = probe == Out::err(2, "ace_to_pl_error(check_load,noncanonical(42,1)).\n");
    assert!(
        probe == Out::ok("") || legacy,
        "schema adapter: {}",
        probe.summary()
    );
    legacy
}
pub fn legacy_block(text: &str) -> String {
    text.replace(
        &(pair("guideline_supersession", 4) + &pair("guideline_evaluation_date", 1)),
        "",
    )
}
pub fn install(c: &Cases, members: &[Member]) {
    let mut manifest = String::new();
    for (i, m) in members.iter().enumerate() {
        c.write(&format!("d{i}.pl"), &m.pl);
        c.write(&format!("d{i}.proof"), &m.payload);
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
        Out::ok(t),
    );
    let demo =
        trace_result.starts_with("yes(proved([") || trace_result.starts_with("solutions([sol(");
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
                "ckc: trace-check ok supersession nodes={}\n",
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
