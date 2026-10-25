// C1 P1/P2/P5 + S1 D1/D10 + S2 D1/P9; oracle = the contracts, not the implementation.
// CKC_C1_TEST_BIN selects a prebuilt executable; default = the release binary.
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

const INDICATORS: [(&str, usize); 22] = [
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
const TABLE: &str = "# format: kind\tlemma\tvalue\nunit\thour\thour\nrelation\tfor\tduration\n";
const SIMPLE: &str = "guideline_entity(actual,a,patient,countable)";
static SERIAL: AtomicUsize = AtomicUsize::new(0);

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn sha(bytes: impl AsRef<[u8]>) -> String {
    use sha2::Digest;
    sha2::Sha256::digest(bytes.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn atom(s: &str) -> String {
    let b = s.as_bytes();
    let alpha = b.first().is_some_and(u8::is_ascii_lowercase)
        && b.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_');
    let graphic = !b.is_empty()
        && b.iter().all(|c| b"#$&*+-./:<=>?@\\^~".contains(c))
        && s != "."
        && !s.starts_with("/*");
    if alpha || graphic || matches!(s, ";" | "!" | "{}") {
        return s.into();
    }
    let mut out = String::from("'");
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\u{b}' => out.push_str("\\v"),
            '\u{c}' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            c if c < '\u{20}' || c == '\u{7f}' => out.push_str(&format!("\\u{:04X}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('\'');
    out
}

fn digest(h: Option<&str>) -> String {
    h.map_or_else(|| "none".into(), |h| format!("sha256({})", atom(h)))
}

fn declarations(count: usize, record_arity: usize) -> String {
    INDICATORS[..count]
        .iter()
        .map(|&(name, arity)| {
            let arity = if name == "guideline_document" {
                record_arity
            } else {
                arity
            };
            format!(":- multifile({name}/{arity}).\n:- discontiguous({name}/{arity}).\n")
        })
        .collect()
}

fn document(
    id: &str,
    ace: &str,
    ulex: Option<&str>,
    table: Option<&str>,
    clauses: &[String],
) -> String {
    format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex({}),vocabulary({})).\n% S1: {}\n{}",
        declarations(22, 4),
        atom(id),
        atom(&sha(ace)),
        digest(ulex),
        digest(table),
        ace.trim_end_matches('\n'),
        clauses
            .iter()
            .map(|c| format!("{c}.\n"))
            .collect::<String>()
    )
}

fn query(
    id: &str,
    ace: &str,
    ulex: Option<&str>,
    table: Option<&str>,
    goal: &str,
    answers: &str,
) -> String {
    format!(
        "% {id} compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,{},ace_sha256({}),ulex({}),vocabulary({})).\n% Q1: {}\n'$guideline_query_projection'(goal({goal}),answers({answers})).\n",
        atom(id),
        atom(&sha(ace)),
        digest(ulex),
        digest(table),
        ace.trim_end_matches('\n')
    )
}

fn plain_doc(ulex: Option<&str>, table: Option<&str>) -> String {
    document("doc", "A patient exists.\n", ulex, table, &[SIMPLE.into()])
}

fn plain_query(ulex: Option<&str>, table: Option<&str>) -> String {
    query("query", "Is there a patient?\n", ulex, table, SIMPLE, "[]")
}

fn legacy_document(version: usize) -> String {
    let h = sha(TABLE);
    let d = plain_doc(None, (version != 1).then_some(h.as_str()));
    let d = d.replace(
        &declarations(22, 4),
        &declarations([0, 9, 11, 22][version], if version == 1 { 3 } else { 4 }),
    );
    let d = d.replace(
        "guideline_schema_version(1).",
        &format!("guideline_schema_version({version})."),
    );
    if version == 1 {
        d.replace(",vocabulary(none)", "")
    } else {
        d.replace("vocabulary(", "temporal(")
    }
}

#[derive(Debug, PartialEq, Eq)]
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

struct Cases {
    dir: PathBuf,
    row: &'static str,
    total: usize,
    failures: Vec<String>,
}

impl Cases {
    fn new(row: &'static str) -> Self {
        let dir = root().join(format!(
            ".scratch/c1-codec/{}-{}-{row}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).unwrap();
        Self {
            dir,
            row,
            total: 0,
            failures: Vec::new(),
        }
    }
    fn write(&self, name: &str, bytes: impl AsRef<[u8]>) {
        fs::write(self.dir.join(name), bytes).unwrap();
    }
    fn run(&self, args: &[&str]) -> Out {
        let program = std::env::var_os("CKC_C1_TEST_BIN")
            .map(PathBuf::from)
            .unwrap_or_else(|| root().join("rust/target/release/ckc"));
        let mut child = Command::new(program)
            .args(args)
            .current_dir(&self.dir)
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
        let deadline = Instant::now() + Duration::from_secs(120);
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break status;
            }
            if Instant::now() > deadline {
                child.kill().unwrap();
                child.wait().unwrap();
                panic!("{}: subprocess exceeded 120 s: {args:?}", self.row);
            }
            std::thread::sleep(Duration::from_millis(2));
        };
        Out {
            rc: status.code().unwrap_or(-1),
            stdout: String::from_utf8(out.join().unwrap()).unwrap(),
            stderr: String::from_utf8(err.join().unwrap()).unwrap(),
        }
    }
    fn expect(&mut self, label: &str, args: &[&str], want: Out) {
        let got = self.run(args);
        self.compare(label, got, want);
    }
    fn compare(&mut self, label: &str, got: Out, want: Out) {
        self.total += 1;
        if got != want {
            self.failures
                .push(format!("{label}: got {got:?}; want {want:?}"));
        }
    }
    fn check(&mut self, label: &str, text: &str) {
        self.write("input.pl", text);
        self.expect(label, &["v1", "check", "input.pl"], Out::ok(""));
    }
    fn reject_at(&mut self, label: &str, text: &str, at: usize) {
        self.write("input.pl", text);
        let prefix = &text.as_bytes()[..at];
        let line = prefix.iter().filter(|&&b| b == b'\n').count() + 1;
        let col = prefix
            .iter()
            .rposition(|&b| b == b'\n')
            .map_or(at + 1, |i| at - i);
        self.expect(
            label,
            &["v1", "check", "input.pl"],
            Out::err(
                2,
                format!("ace_to_pl_error(check_load,noncanonical({line},{col})).\n"),
            ),
        );
    }
    fn mutation(&mut self, label: &str, canonical: &str, from: &str, to: &str) {
        assert_eq!(
            canonical.matches(from).count(),
            1,
            "mutation anchor {label}: {from}"
        );
        let changed = canonical.replacen(from, to, 1);
        let at = canonical
            .bytes()
            .zip(changed.bytes())
            .position(|(a, b)| a != b)
            .unwrap_or(canonical.len().min(changed.len()));
        self.reject_at(label, &changed, at);
    }
    fn finish(self) {
        println!(
            "C1 {} cases={} failures={}",
            self.row,
            self.total,
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "C1 {}: {} of {} failed\n{}",
            self.row,
            self.failures.len(),
            self.total,
            self.failures.join("\n")
        );
    }
}

impl Drop for Cases {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn document_matrix(row: &'static str, has_ulex: bool, has_table: bool) {
    let mut c = Cases::new(row);
    for ch in ['a', '0'] {
        let h = ch.to_string().repeat(64);
        for id in ["a", "0", "a-b", "a9"] {
            let text = document(
                id,
                "A patient exists.\n",
                has_ulex.then_some(h.as_str()),
                has_table.then_some(h.as_str()),
                &[SIMPLE.into()],
            );
            c.check(&format!("id={id} hash={ch}"), &text);
        }
    }
    c.finish();
}

#[test]
fn a01_document_none_none() {
    document_matrix("A01", false, false);
}
#[test]
fn a02_document_digest_none() {
    document_matrix("A02", true, false);
}
#[test]
fn a03_document_none_digest() {
    document_matrix("A03", false, true);
}
#[test]
fn a04_document_digest_digest() {
    document_matrix("A04", true, true);
}

#[test]
fn a05_all_semantic_indicators_in_every_clause_position() {
    let mut c = Cases::new("A05");
    let h = sha(TABLE);
    for table in [None, Some(h.as_str())] {
        for &(name, arity) in &INDICATORS[2..] {
            for data in ["x", "'two words'", "'α'", "[x,-1,compound(a)]"] {
                let lit = format!("{name}({})", vec![data; arity].join(","));
                for (position, clause) in [
                    ("head", lit.clone()),
                    ("positive", format!("{SIMPLE} :- {lit}")),
                    ("singleton-naf", format!("{SIMPLE} :- \\+ {lit}")),
                    ("grouped-naf", format!("{SIMPLE} :- \\+ ({lit}, {SIMPLE})")),
                ] {
                    c.check(
                        &format!("table={} {name}/{arity} {position} {data}", table.is_some()),
                        &document("doc", "A structural clause.\n", None, table, &[clause]),
                    );
                }
            }
        }
    }
    assert_eq!(c.total, 640);
    c.finish();
}

#[test]
fn a06_question_custody_cartesian_product() {
    let mut c = Cases::new("A06");
    for ch in ['a', '0'] {
        let h = ch.to_string().repeat(64);
        for ulex in [None, Some(h.as_str())] {
            for table in [None, Some(h.as_str())] {
                for id in ["a", "0", "a-b"] {
                    c.check(
                        &format!(
                            "id={id} hash={ch} ulex={} table={}",
                            ulex.is_some(),
                            table.is_some()
                        ),
                        &query(id, "Is there a patient?\n", ulex, table, SIMPLE, "[]"),
                    );
                }
            }
        }
    }
    // K1 owns term structure, not K2's goal whitelist or answer-row validation.
    for (goal, answers) in [
        ("foreign(A)", "[answer(A,other)]"),
        ("A", "[]"),
        ("true", "not_a_list"),
    ] {
        c.check(
            "K1 projection structure only",
            &query("query", "Question?\n", None, None, goal, answers),
        );
    }
    c.finish();
}

fn reject_legacy_doc(row: &'static str, version: usize) {
    let mut c = Cases::new(row);
    let text = legacy_document(version);
    let h = sha(TABLE);
    let canonical = plain_doc(None, (version != 1).then_some(h.as_str()));
    let at = canonical
        .bytes()
        .zip(text.bytes())
        .position(|(a, b)| a != b)
        .unwrap();
    c.reject_at("pre-C1 envelope", &text, at);
    c.finish();
}

#[test]
fn a07_pre_c1_v1_document_rejects_at_record_declaration() {
    reject_legacy_doc("A07", 1);
}
#[test]
fn a08_pre_c1_v2_document_rejects_at_short_declarations() {
    reject_legacy_doc("A08", 2);
}
#[test]
fn a09_pre_c1_v3_document_rejects_at_schema_identifier() {
    reject_legacy_doc("A09", 3);
}

#[test]
fn a10_pre_c1_question_envelopes_reject() {
    let mut c = Cases::new("A10");
    let h = sha(TABLE);
    let q = plain_query(None, None);
    c.mutation("old v1 /4 record", &q, ",vocabulary(none)", "");
    for v in ["v2", "v3"] {
        let canonical = plain_query(None, Some(&h));
        let text = canonical
            .replace("'(v1,", &format!("'({v},"))
            .replace("vocabulary(", "temporal(");
        let at = canonical.find("'(v1,").unwrap() + 3;
        c.reject_at(v, &text, at);
    }
    c.finish();
}

#[test]
fn a11_temporal_wrappers_are_not_aliases() {
    let mut c = Cases::new("A11");
    let h = sha(TABLE);
    for text in [plain_doc(None, Some(&h)), plain_query(None, Some(&h))] {
        c.mutation("retired wrapper", &text, "vocabulary(", "temporal(");
    }
    c.finish();
}

#[test]
fn a12_mandatory_vocabulary_wrapper_and_digest_boundaries() {
    let mut c = Cases::new("A12");
    let h = "a".repeat(64);
    for text in [plain_doc(None, None), plain_query(None, None)] {
        c.mutation(
            "missing wrapper /3 doc or /4 query",
            &text,
            ",vocabulary(none)",
            "",
        );
        c.mutation(
            "bare none instead of wrapper",
            &text,
            "vocabulary(none)",
            "none",
        );
        for i in 0.."vocabulary".len() {
            let mut name = String::from("vocabulary");
            name.replace_range(i..i + 1, "z");
            c.mutation(
                &format!("vocabulary fixed-name byte {i} none"),
                &text,
                "vocabulary(",
                &format!("{name}("),
            );
        }
        c.mutation(
            "misspelled none",
            &text,
            "vocabulary(none)",
            "vocabulary(nonx)",
        );
        c.mutation("empty wrapper", &text, "vocabulary(none)", "vocabulary()");
        c.mutation(
            "extra record field",
            &text,
            ",vocabulary(none)).",
            ",vocabulary(none),extra).",
        );
    }
    for text in [plain_doc(None, Some(&h)), plain_query(None, Some(&h))] {
        for i in 0.."vocabulary".len() {
            let mut name = String::from("vocabulary");
            name.replace_range(i..i + 1, "z");
            c.mutation(
                &format!("vocabulary fixed-name byte {i} digest"),
                &text,
                "vocabulary(",
                &format!("{name}("),
            );
        }
        for i in 0.."sha256".len() {
            let mut name = String::from("sha256");
            name.replace_range(i..i + 1, "z");
            c.mutation(
                &format!("digest fixed-name byte {i}"),
                &text,
                "vocabulary(sha256(",
                &format!("vocabulary({name}("),
            );
        }
        for n in [0, 1, 63, 65] {
            c.mutation(
                &format!("digest length {n}"),
                &text,
                &format!("sha256({h})"),
                &format!("sha256({})", "a".repeat(n)),
            );
        }
        for i in [0, 31, 63] {
            let mut invalid = h.clone();
            invalid.replace_range(i..i + 1, "g");
            c.mutation(
                &format!("nonhex digest byte {i}"),
                &text,
                &format!("sha256({h})"),
                &format!("sha256({invalid})"),
            );
        }
    }
    for text in [plain_doc(Some(&h), None), plain_query(Some(&h), None)] {
        for n in [0, 1, 63, 65] {
            c.mutation(
                &format!("ulex digest length {n}"),
                &text,
                &format!("ulex(sha256({h}))"),
                &format!("ulex(sha256({}))", "a".repeat(n)),
            );
        }
        c.mutation(
            "ulex digest spelling",
            &text,
            "ulex(sha256(",
            "ulex(sha257(",
        );
    }
    for id in ["", "-a", "A", "a_b", "α"] {
        let at = if id == "a_b" { 3 } else { 2 };
        for text in [
            document(id, "A patient exists.\n", None, None, &[SIMPLE.into()]),
            query(id, "Question?\n", None, None, SIMPLE, "[]"),
        ] {
            c.reject_at(&format!("identifier {id:?}"), &text, at);
        }
    }
    for text in [plain_doc(None, None), plain_query(None, None)] {
        c.reject_at("missing final LF", &text[..text.len() - 1], text.len() - 1);
        c.reject_at("extra final LF", &(text.clone() + "\n"), text.len());
    }
    c.finish();
}

#[test]
fn a13_declaration_order_count_arity_and_semantic_arity() {
    let mut c = Cases::new("A13");
    let text = plain_doc(None, None);
    for (i, &(name, arity)) in INDICATORS.iter().enumerate() {
        for directive in ["multifile", "discontiguous"] {
            let old = format!(":- {directive}({name}/{arity}).\n");
            for wrong in [arity - 1, arity + 1] {
                c.mutation(
                    &format!("{directive} {name} arity {wrong}"),
                    &text,
                    &old,
                    &format!(":- {directive}({name}/{wrong}).\n"),
                );
            }
            c.mutation(&format!("missing {directive} {name}"), &text, &old, "");
            c.mutation(
                &format!("duplicate {directive} {name}"),
                &text,
                &old,
                &old.repeat(2),
            );
        }
        if i + 1 < INDICATORS.len() {
            let (next, next_arity) = INDICATORS[i + 1];
            let a = format!(":- multifile({name}/{arity}).\n:- discontiguous({name}/{arity}).\n");
            let b = format!(
                ":- multifile({next}/{next_arity}).\n:- discontiguous({next}/{next_arity}).\n"
            );
            c.mutation(
                &format!("swapped indicators {i}"),
                &text,
                &(a.clone() + &b),
                &(b + &a),
            );
        }
    }
    for &(name, arity) in &INDICATORS[2..] {
        let lit = format!("{name}({})", vec!["x"; arity].join(","));
        for position in 0..4 {
            let clause = match position {
                0 => lit.clone(),
                1 => format!("{SIMPLE} :- {lit}"),
                2 => format!("{SIMPLE} :- \\+ {lit}"),
                _ => format!("{SIMPLE} :- \\+ ({lit}, {SIMPLE})"),
            };
            let text = document("doc", "A structural clause.\n", None, None, &[clause]);
            for wrong in [arity - 1, arity + 1] {
                let bad = format!("{name}({})", vec!["x"; wrong].join(","));
                c.mutation(
                    &format!("{name} position {position} arity {wrong}"),
                    &text,
                    &lit,
                    &bad,
                );
            }
            let bad = format!("{}z", &name[..name.len() - 1]);
            c.mutation(
                &format!("{name} position {position} fixed-name end"),
                &text,
                &lit,
                &lit.replacen(name, &bad, 1),
            );
        }
    }
    c.finish();
}

#[test]
fn a14_non_one_schema_identifiers_reject_at_first_divergence() {
    let mut c = Cases::new("A14");
    let text = plain_doc(None, None);
    for v in [
        "0", "2", "3", "4", "10", "11", "-1", "01", "'1'", "v1", "A", "",
    ] {
        c.mutation(
            v,
            &text,
            "guideline_schema_version(1).",
            &format!("guideline_schema_version({v})."),
        );
    }
    c.finish();
}

#[test]
fn a15_query_version_tokens_are_exact() {
    let mut c = Cases::new("A15");
    let q = plain_query(None, None);
    for v in ["v0", "v2", "v3", "v10", "'v1'", "1", "V1", "v", "w1"] {
        c.mutation(v, &q, "'(v1,", &format!("'({v},"));
    }
    c.finish();
}

#[test]
fn a16_render_is_byte_exact_and_idempotent() {
    let mut c = Cases::new("A16");
    for ch in ['a', '0'] {
        let h = ch.to_string().repeat(64);
        for ulex in [None, Some(h.as_str())] {
            for table in [None, Some(h.as_str())] {
                for text in [plain_doc(ulex, table), plain_query(ulex, table)] {
                    c.write("input.pl", &text);
                    let rendered = c.run(&["v1", "render", "input.pl"]);
                    let again = (rendered.rc == 0).then(|| rendered.stdout.clone());
                    c.compare("canonical render", rendered, Out::ok(&text));
                    if let Some(bytes) = again {
                        c.write("again.pl", bytes);
                        c.expect(
                            "render(render(x)) = x",
                            &["v1", "render", "again.pl"],
                            Out::ok(&text),
                        );
                    }
                }
            }
        }
    }
    c.finish();
}

fn gid(id: &str, slot: usize) -> String {
    format!("'$guideline_id'(product,{},1,ref({slot}),[])", atom(id))
}

fn payload(id: &str, heads: &[String]) -> String {
    format!(
        "'$guideline_proof'({},1,variant(1),witness([]),prove([{}])).\n",
        atom(id),
        heads.join(",")
    )
}

struct Member {
    id: String,
    table: Option<String>,
    clauses: Vec<String>,
    heads: Vec<String>,
}

impl Member {
    fn new(id: &str, table: bool) -> Self {
        let g = gid(id, 1);
        let entity = format!("guideline_entity(actual,{g},patient,countable)");
        let property = format!("guideline_property(actual,{g},ready,pos)");
        Self {
            id: id.into(),
            table: table.then(|| sha(TABLE)),
            clauses: vec![entity.clone(), format!("{property} :- {entity}")],
            heads: vec![entity, property],
        }
    }
    fn text(&self) -> String {
        document(
            &self.id,
            "A patient is ready.\n",
            None,
            self.table.as_deref(),
            &self.clauses,
        )
    }
}

fn install(c: &Cases, members: &[Member]) {
    let mut manifest = String::new();
    for (i, m) in members.iter().enumerate() {
        c.write(&format!("d{i}.pl"), m.text());
        c.write(&format!("d{i}.proof"), payload(&m.id, &m.heads));
        manifest.push_str(&format!("d{i}.pl\td{i}.proof\n"));
    }
    c.write("m.tsv", manifest);
}

fn shapes() -> Vec<Vec<(&'static str, bool)>> {
    vec![
        vec![],
        vec![("alpha", false)],
        vec![("alpha", false), ("beta", false)],
        vec![("alpha", true)],
        vec![("alpha", true), ("beta", true)],
        vec![("alpha", false), ("beta", true)],
        vec![("beta", true), ("alpha", false)],
    ]
}

fn members(shape: &[(&str, bool)]) -> Vec<Member> {
    shape
        .iter()
        .map(|&(id, table)| Member::new(id, table))
        .collect()
}

fn attributed_literal(name: &str, arity: usize) -> String {
    let mut args = vec!["x".to_owned(); arity];
    // An arity-1 indicator (cs1 guideline_exclusion/1) carries the identity in its only slot.
    args[usize::from(arity > 1)] = gid("alpha", 1);
    format!("{name}({})", args.join(","))
}

fn answers_artifact(id: &str, q: &str, result: &str) -> String {
    format!(
        "% {id} answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,{},query_sha256({}),result({result})).\n",
        atom(id),
        atom(&sha(q))
    )
}

fn traces_artifact(id: &str, q: &str, a: &str, result: &str) -> String {
    format!(
        "% {id} traced against the loaded composition by ckc queries; do not edit.\n'$guideline_traces'(v1,{},query_sha256({}),answers_sha256({}),result({result})).\n",
        atom(id),
        atom(&sha(q)),
        atom(&sha(a))
    )
}

fn clause_node(id: &str, clause: &str, children: &[String]) -> String {
    format!(
        "clause(sentence({},1),clause_sha256({}),[{}])",
        atom(id),
        atom(&sha(format!("{clause}.\n"))),
        children.join(",")
    )
}

fn check_chain(
    c: &mut Cases,
    label: &str,
    id: &str,
    q: &str,
    result: &str,
    trace_result: &str,
    nodes: usize,
) {
    let a = answers_artifact(id, q, result);
    let t = traces_artifact(id, q, &a, trace_result);
    c.write("query.pl", q);
    c.write("answers.pl", &a);
    c.write("traces.pl", &t);
    c.expect(
        &format!("{label} answer"),
        &["v1", "answer", "m.tsv", "query.pl"],
        Out::ok(a),
    );
    c.expect(
        &format!("{label} trace"),
        &["v1", "trace", "m.tsv", "query.pl", "answers.pl"],
        Out::ok(t),
    );
    let want = if nodes == 0 {
        Out::err(1, "ace_to_pl_error(proof,trace_check(non_demo)).\n")
    } else {
        Out::ok(format!("ckc: trace-check ok {id} nodes={nodes}\n"))
    };
    c.expect(
        &format!("{label} trace-check"),
        &[
            "v1",
            "trace-check",
            "m.tsv",
            "query.pl",
            "answers.pl",
            "traces.pl",
        ],
        want,
    );
}

fn check_member_answers(c: &mut Cases, shape: &[(&str, bool)], table: Option<&str>) {
    let ms = members(shape);
    install(c, &ms);
    let q = query(
        "query",
        "Which patient is ready?\n",
        None,
        table,
        "guideline_property(actual,A,ready,pos)",
        "[answer(A,noun(patient,countable))]",
    );
    let mut sorted: Vec<_> = ms.iter().collect();
    sorted.sort_by_key(|m| &m.id);
    let result = format!(
        "solutions([{}])",
        sorted
            .iter()
            .map(|m| format!("sol([{}])", gid(&m.id, 1)))
            .collect::<Vec<_>>()
            .join(",")
    );
    let trace = format!(
        "solutions([{}])",
        sorted
            .iter()
            .map(|m| {
                let leaf = clause_node(&m.id, &m.clauses[0], &[]);
                let node = clause_node(&m.id, &m.clauses[1], &[leaf]);
                format!("sol([{}],proved([{node}]))", gid(&m.id, 1))
            })
            .collect::<Vec<_>>()
            .join(",")
    );
    check_chain(
        c,
        &format!("{shape:?} query-table={}", table.is_some()),
        "query",
        &q,
        &result,
        &trace,
        sorted.len() * 2,
    );
}

#[test]
fn a17_aggregate_composition_matrix() {
    let mut c = Cases::new("A17");
    for shape in shapes() {
        let ms = members(&shape);
        install(&c, &ms);
        c.expect(
            &format!("{shape:?}"),
            &["v1", "aggregate-check", "m.tsv"],
            Out::ok(format!(
                "ace_to_pl aggregate ok {} documents {} obligations\n",
                ms.len(),
                ms.len()
            )),
        );
    }
    c.finish();
}

#[test]
fn a18_recursion_composition_matrix() {
    let mut c = Cases::new("A18");
    for shape in shapes() {
        let ms = members(&shape);
        install(&c, &ms);
        c.expect(
            &format!("{shape:?}"),
            &["v1", "recursion-check", "m.tsv"],
            Out::ok(format!(
                "ace_to_pl recursion ok {} documents {} rule clauses\n",
                ms.len(),
                ms.len()
            )),
        );
    }
    for &(name, arity) in &INDICATORS[2..] {
        let head = attributed_literal(name, arity);
        for table in [false, true] {
            for recursive in [false, true] {
                let body = if recursive {
                    head.as_str()
                } else if name == "guideline_entity" {
                    "guideline_event(actual,event,wait)"
                } else {
                    SIMPLE
                };
                install(
                    &c,
                    &[Member {
                        id: "alpha".into(),
                        table: table.then(|| sha(TABLE)),
                        clauses: vec![format!("{head} :- {body}")],
                        heads: vec![],
                    }],
                );
                let want = if recursive {
                    Out::err(
                        1,
                        format!(
                            "ace_to_pl_error(proof,left_recursive(sentence(alpha,1),{name},{arity})).\n"
                        ),
                    )
                } else {
                    Out::ok("ace_to_pl recursion ok 1 documents 1 rule clauses\n")
                };
                c.expect(
                    &format!("{name}/{arity} table={table} recursive={recursive}"),
                    &["v1", "recursion-check", "m.tsv"],
                    want,
                );
            }
        }
    }
    c.finish();
}

#[test]
fn a19_answers_traces_and_cross_document_proofs_are_exact() {
    let mut c = Cases::new("A19");
    for shape in shapes() {
        check_member_answers(&mut c, &shape, None);
    }
    for reverse in [false, true] {
        let a = Member::new("alpha", false);
        let product = "'$guideline_id'(product,beta,1,ref(1),[A])";
        let head = format!("guideline_property(actual,{product},eligible,pos)");
        let rule = format!("{head} :- guideline_entity(actual,A,patient,countable)");
        let b = Member {
            id: "beta".into(),
            table: Some(sha(TABLE)),
            clauses: vec![rule.clone()],
            heads: vec![],
        };
        let node_a = clause_node("alpha", &a.clauses[0], &[]);
        let node_b = clause_node("beta", &rule, &[node_a]);
        let ms = if reverse { vec![b, a] } else { vec![a, b] };
        install(&c, &ms);
        let q = query(
            "cross",
            "Is a patient eligible?\n",
            None,
            None,
            "guideline_property(actual,A,eligible,pos)",
            "[]",
        );
        check_chain(
            &mut c,
            &format!("cross-document reverse={reverse}"),
            "cross",
            &q,
            "yes",
            &format!("yes(proved([{node_b}]))"),
            2,
        );
    }
    c.finish();
}

#[test]
fn a20_query_vocabulary_and_goal_whitelist_are_composition_independent() {
    let mut c = Cases::new("A20");
    let foreign_digest = sha("a query's independently authored vocabulary\n");
    for shape in shapes() {
        for table in [None, Some(foreign_digest.as_str())] {
            check_member_answers(&mut c, &shape, table);
        }
    }
    install(&c, &[]);
    for table in [None, Some(foreign_digest.as_str())] {
        for &(name, arity) in &INDICATORS[2..] {
            let goal = format!("{name}({})", vec!["x"; arity].join(","));
            let q = query("query", "Question?\n", None, table, &goal, "[]");
            check_chain(
                &mut c,
                &format!("all 16 goals {name} table={}", table.is_some()),
                "query",
                &q,
                "no(finite_failure)",
                "no(finite_failure)",
                0,
            );
        }
        for (goal, rows, why) in [
            ("foreign(A)", "[]", "goal_foreign(foreign,1)"),
            ("A", "[]", "goal_variable"),
            (
                "guideline_entity(actual,A,patient,countable)",
                "bad",
                "answers_list",
            ),
            (
                "guideline_entity(actual,A,patient,countable)",
                "[bad]",
                "answer_shape(1)",
            ),
            (
                "guideline_entity(actual,A,patient,countable)",
                "[answer(x,noun(patient,countable))]",
                "answer_var(1,nonvar)",
            ),
            (
                "guideline_entity(actual,A,patient,countable)",
                "[answer(B,noun(patient,countable))]",
                "answer_var(1,absent)",
            ),
            (
                "guideline_entity(actual,A,patient,countable)",
                "[answer(A,noun(patient,countable)),answer(A,noun(patient,countable))]",
                "answer_var(2,duplicate)",
            ),
            (
                "guideline_entity(actual,A,patient,countable)",
                "[answer(A,other)]",
                "answer_desc(1)",
            ),
        ] {
            c.write(
                "query.pl",
                query("query", "Question?\n", None, table, goal, rows),
            );
            c.expect(
                &format!("retained query validation {why} table={}", table.is_some()),
                &["v1", "answer", "m.tsv", "query.pl"],
                Out::err(
                    2,
                    format!("ace_to_pl_error(check_load,query_file({why})).\n"),
                ),
            );
        }
    }
    for &(name, arity) in &INDICATORS[2..] {
        let lit = attributed_literal(name, arity);
        let proof = format!("yes(proved([{}]))", clause_node("alpha", &lit, &[]));
        for document_table in [false, true] {
            install(
                &c,
                &[Member {
                    id: "alpha".into(),
                    table: document_table.then(|| sha(TABLE)),
                    clauses: vec![lit.clone()],
                    heads: vec![lit.clone()],
                }],
            );
            for query_table in [None, Some(foreign_digest.as_str())] {
                let q = query("query", "Question?\n", None, query_table, &lit, "[]");
                check_chain(
                    &mut c,
                    &format!(
                        "positive {name}/{arity} doc-table={document_table} query-table={}",
                        query_table.is_some()
                    ),
                    "query",
                    &q,
                    "yes",
                    &proof,
                    1,
                );
            }
        }
    }
    c.finish();
}

fn manifest_reject(why: &str) -> Out {
    Out::err(
        2,
        format!(
            "ace_to_pl_error(check_load,error(aggregate_manifest({why}),context(:(ace_to_pl,/(aggregate_read_manifest,3)),'m.tsv'))).\n"
        ),
    )
}

#[test]
fn a21_retained_composition_rejects() {
    let mut c = Cases::new("A21");
    for bad in ["a\tb", "\n", "a\n", "\tb\n", "a\t\n", "a\tb\tc\n"] {
        c.write("m.tsv", bad);
        let why = if !bad.ends_with('\n') {
            "missing_final_newline".into()
        } else {
            format!("line({})", atom(bad.trim_end_matches('\n')))
        };
        c.expect(
            &format!("manifest {bad:?}"),
            &["v1", "aggregate-check", "m.tsv"],
            manifest_reject(&why),
        );
    }
    for (label, ms) in [
        (
            "duplicate docid",
            vec![Member::new("alpha", false), Member::new("alpha", false)],
        ),
        ("duplicate path", vec![Member::new("alpha", false)]),
    ] {
        install(&c, &ms);
        if label == "duplicate path" {
            c.write("m.tsv", "d0.pl\td0.proof\nd0.pl\td0.proof\n");
        }
        c.expect(
            label,
            &["v1", "aggregate-check", "m.tsv"],
            Out::err(1, "ace_to_pl_error(proof,document_records(2,1)).\n"),
        );
    }
    let m = Member::new("alpha", false);
    let p = payload(&m.id, &m.heads);
    install(&c, &[m]);
    for (label, bytes, why) in [
        (
            "missing obligation",
            String::new(),
            "missing_obligation(alpha,1)",
        ),
        (
            "duplicate obligation",
            p.repeat(2),
            "duplicate_obligation(alpha,1,variant(1))",
        ),
        (
            "variant starts at two",
            p.replace("variant(1)", "variant(2)"),
            "variant_sequence(alpha,1,[2])",
        ),
        (
            "empty obligation",
            payload("alpha", &[]),
            "empty_obligation(alpha,1,variant(1))",
        ),
        (
            "extra obligation",
            p.clone() + &payload("other", &[SIMPLE.into()]),
            "extra_obligation(other,1)",
        ),
        (
            "unproved obligation",
            payload("alpha", &["guideline_event(actual,no_event,absent)".into()]),
            "obligation_failed(alpha,1,variant(1))",
        ),
    ] {
        c.write("d0.proof", bytes);
        c.expect(
            label,
            &["v1", "aggregate-check", "m.tsv"],
            Out::err(1, format!("ace_to_pl_error(proof,{why}).\n")),
        );
    }
    c.write("d0.proof", p.trim_end_matches('\n'));
    c.expect(
        "payload final LF",
        &["v1", "aggregate-check", "m.tsv"],
        Out::err(2, "ace_to_pl_error(check_load,payload_term('d0.proof')).\n"),
    );
    let recursive = format!("{SIMPLE} :- {SIMPLE}");
    c.write(
        "d0.pl",
        document("alpha", "A recursive clause.\n", None, None, &[recursive]),
    );
    c.expect(
        "left recursion",
        &["v1", "recursion-check", "m.tsv"],
        Out::err(
            1,
            "ace_to_pl_error(proof,left_recursive(unattributed,guideline_entity,4)).\n",
        ),
    );
    c.finish();
}

#[test]
fn a22_removed_schema_rejects_never_replace_valid_composition() {
    let mut c = Cases::new("A22");
    let different = sha("another independent table\n");
    for shape in [
        vec![("alpha", false), ("beta", true)],
        vec![("beta", true), ("alpha", false)],
    ] {
        install(&c, &members(&shape));
        for table in [None, Some(different.as_str())] {
            let q = query(
                "query",
                "Question?\n",
                None,
                table,
                "guideline_property(actual,A,ready,pos)",
                "[]",
            );
            c.write("query.pl", &q);
            for (args, want) in [
                (
                    vec!["v1", "aggregate-check", "m.tsv"],
                    Out::ok("ace_to_pl aggregate ok 2 documents 2 obligations\n"),
                ),
                (
                    vec!["v1", "recursion-check", "m.tsv"],
                    Out::ok("ace_to_pl recursion ok 2 documents 2 rule clauses\n"),
                ),
                (
                    vec!["v1", "answer", "m.tsv", "query.pl"],
                    Out::ok(answers_artifact("query", &q, "yes")),
                ),
            ] {
                let got = c.run(&args);
                assert!(
                    !got.stderr.contains("schema_version"),
                    "removed reject: {}",
                    got.stderr
                );
                c.compare(
                    &format!("{shape:?} table={} {}", table.is_some(), args[1]),
                    got,
                    want,
                );
            }
        }
    }
    c.finish();
}

fn conjunction(goals: &[String]) -> String {
    match goals {
        [last] => last.clone(),
        [first, rest @ ..] => format!("','({first},{})", conjunction(rest)),
        [] => panic!("a projection needs a goal"),
    }
}

struct CertCase {
    mode: &'static str,
    id: &'static str,
    ace: String,
    ulex: Option<String>,
    dump: String,
    pl: String,
    table: Option<String>,
    expected: String,
}

impl CertCase {
    fn new(mode: &'static str, table: bool, ulex: bool) -> Self {
        let id = if mode == "doc" {
            "certdoc"
        } else {
            "certquery"
        };
        let ace = if mode == "doc" {
            "A patient waits for 2 hours.\n"
        } else {
            "Does a patient wait for 2 hours?\n"
        };
        let ulex = ulex.then(|| "prep(for,for).\n".to_owned());
        let uhash = ulex.as_ref().map(sha);
        let thash = table.then(|| sha(TABLE));
        let (patient, event, quantity) = if mode == "doc" {
            (gid(id, 1), gid(id, 2), gid(id, 3))
        } else {
            ("A".into(), "B".into(), "C".into())
        };
        let mut goals = vec![
            format!("guideline_entity(actual,{patient},patient,countable)"),
            format!("guideline_cardinality(actual,{patient},na,eq,1)"),
            format!("guideline_event(actual,{event},wait)"),
            format!("guideline_arg(actual,{event},1,{patient})"),
            format!("guideline_entity(actual,{quantity},hour,countable)"),
            format!("guideline_cardinality(actual,{quantity},na,eq,2)"),
            format!("guideline_pp(actual,{event},for,{quantity})"),
        ];
        if table {
            goals.insert(
                goals.len() - 1,
                format!("guideline_unit(actual,{quantity},hour)"),
            );
            goals.push(format!(
                "guideline_interval(actual,{event},duration,{quantity},hour,none)"
            ));
        }
        let (tokens, anchors) = if mode == "doc" {
            ("[^,'A',patient,waits,for,2,hours,'.']", [2, 3, 6, 4])
        } else {
            ("[^,'Does',a,patient,wait,for,2,hours,?]", [3, 4, 7, 5])
        };
        let conds = format!(
            "[-(object(A,patient,countable,na,eq,1),/(1,{})),-(predicate(B,wait,A),/(1,{})),-(object(C,hour,countable,na,eq,2),/(1,{})),-(modifier_pp(B,for,C),/(1,{}))]",
            anchors[0], anchors[1], anchors[2], anchors[3]
        );
        let inner = format!("drs([A,B,C],{conds})");
        let drs = if mode == "doc" {
            inner
        } else {
            format!("drs([],[question({inner})])")
        };
        let dump = format!("sentences([{tokens}]).\ndrs({drs}).\nmessages([]).\n");
        let pl = if mode == "doc" {
            document(id, ace, uhash.as_deref(), thash.as_deref(), &goals)
        } else {
            query(
                id,
                ace,
                uhash.as_deref(),
                thash.as_deref(),
                &conjunction(&goals),
                "[]",
            )
        };
        let expected = if mode == "doc" {
            payload(id, &goals)
        } else {
            format!("ckc: certify ok {id}\n")
        };
        Self {
            mode,
            id,
            ace: ace.into(),
            ulex,
            dump,
            pl,
            table: table.then(|| TABLE.into()),
            expected,
        }
    }
    fn argv(&self, c: &Cases) -> Vec<String> {
        c.write("ace.txt", &self.ace);
        c.write("dump.txt", &self.dump);
        c.write("cert.pl", &self.pl);
        if let Some(u) = &self.ulex {
            c.write("lexicon.ulex", u);
        }
        let mut args: Vec<String> = [
            "certify-one",
            self.mode,
            self.id,
            "ace.txt",
            if self.ulex.is_some() {
                "lexicon.ulex"
            } else {
                "-"
            },
            "dump.txt",
            "cert.pl",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        if let Some(t) = &self.table {
            c.write("vocabulary.tsv", t);
            args.push("vocabulary.tsv".into());
        }
        args
    }
    fn check(&self, c: &mut Cases, label: &str, why: Option<&str>) {
        let args = self.argv(c);
        let args: Vec<_> = args.iter().map(String::as_str).collect();
        let want = why.map_or_else(
            || Out::ok(&self.expected),
            |why| Out::err(1, format!("ckc: certify: {}: {why}.\n", self.id)),
        );
        c.expect(label, &args, want);
    }
    fn repin_table(&mut self, new: Option<&str>) {
        let old_hash = self.table.as_ref().map(sha);
        let new_hash = new.map(sha);
        let old = format!("vocabulary({})", digest(old_hash.as_deref()));
        let replacement = format!("vocabulary({})", digest(new_hash.as_deref()));
        assert_eq!(self.pl.matches(&old).count(), 1);
        self.pl = self.pl.replacen(&old, &replacement, 1);
        self.table = new.map(str::to_owned);
    }
    fn mismatch(&self) -> &'static str {
        if self.mode == "doc" {
            "clauses(1)"
        } else {
            "projection"
        }
    }
}

#[test]
fn a23_clean_documents_and_questions_certify_with_and_without_tables() {
    let mut c = Cases::new("A23");
    for mode in ["doc", "query"] {
        for table in [false, true] {
            for ulex in [false, true] {
                CertCase::new(mode, table, ulex).check(
                    &mut c,
                    &format!("{mode} table={table} ulex={ulex}"),
                    None,
                );
            }
        }
    }
    c.finish();
}

#[test]
fn a24_certification_refuses_retired_temporal_wrapper() {
    let mut c = Cases::new("A24");
    for mode in ["doc", "query"] {
        let mut sample = CertCase::new(mode, true, false);
        sample.pl = sample.pl.replace("vocabulary(", "temporal(");
        sample.check(&mut c, mode, Some("noncanonical"));
    }
    c.finish();
}

#[test]
fn a25_certification_refuses_a_digest_without_table_bytes() {
    let mut c = Cases::new("A25");
    for mode in ["doc", "query"] {
        let mut sample = CertCase::new(mode, false, false);
        sample.pl = sample.pl.replace(
            "vocabulary(none)",
            &format!("vocabulary({})", digest(Some(&sha(TABLE)))),
        );
        sample.check(&mut c, mode, Some("vocabulary"));
    }
    c.finish();
}

#[test]
fn a26_certification_refuses_none_with_table_bytes() {
    let mut c = Cases::new("A26");
    for mode in ["doc", "query"] {
        let mut sample = CertCase::new(mode, true, false);
        sample.pl = sample.pl.replace(
            &format!("vocabulary({})", digest(Some(&sha(TABLE)))),
            "vocabulary(none)",
        );
        sample.check(&mut c, mode, Some("vocabulary"));
    }
    c.finish();
}

#[test]
fn a27_raw_table_custody_and_projection_use_the_supplied_bytes() {
    let mut c = Cases::new("A27");
    for mode in ["doc", "query"] {
        let mut sample = CertCase::new(mode, true, false);
        sample.pl = sample.pl.replace(
            &format!("vocabulary({})", digest(Some(&sha(TABLE)))),
            &format!("vocabulary({})", digest(Some(&sha("different bytes")))),
        );
        sample.check(&mut c, "wrong digest", Some("vocabulary"));

        let reordered = "# format: kind\tlemma\tvalue\nrelation\tfor\tduration\nunit\thour\thour\n";
        let mut sample = CertCase::new(mode, true, false);
        sample.table = Some(reordered.into());
        sample.check(
            &mut c,
            "same rows, different bytes, old digest",
            Some("vocabulary"),
        );
        let mut sample = CertCase::new(mode, true, false);
        sample.repin_table(Some(reordered));
        sample.check(&mut c, "same rows, repinned raw-byte digest", None);

        let changed = TABLE.replace("unit\thour\thour", "unit\thour\tday");
        let mut sample = CertCase::new(mode, true, false);
        sample.repin_table(Some(&changed));
        sample.check(
            &mut c,
            "valid different table, repinned digest, old annotation",
            Some(sample.mismatch()),
        );
        for malformed in [
            String::new(),
            "# format: kind\tlemma\tvalue\n".into(),
            format!("{TABLE}unit\thour\thour\n"),
            TABLE.trim_end_matches('\n').into(),
            TABLE.replace("# format:", "# formats:"),
        ] {
            let mut sample = CertCase::new(mode, true, false);
            sample.repin_table(Some(&malformed));
            sample.check(
                &mut c,
                "malformed table, matching raw digest",
                Some("vocabulary"),
            );
        }
    }
    c.finish();
}

#[test]
fn a28_annotations_without_a_table_fail_projection_not_codec() {
    let mut c = Cases::new("A28");
    for mode in ["doc", "query"] {
        let mut sample = CertCase::new(mode, true, false);
        sample.repin_table(None);
        c.check(
            "annotation is structurally legal without a table",
            &sample.pl,
        );
        sample.check(&mut c, mode, Some(sample.mismatch()));
    }
    c.finish();
}

#[test]
fn a29_changed_annotations_reject_with_primitives_retained() {
    let mut c = Cases::new("A29");
    for mode in ["doc", "query"] {
        for (from, to) in [
            (",duration,", ",within,"),
            (",hour,none)", ",day,none)"),
            (",hour,none)", ",hour,other)"),
        ] {
            let mut sample = CertCase::new(mode, true, false);
            assert_eq!(sample.pl.matches(from).count(), 1);
            sample.pl = sample.pl.replacen(from, to, 1);
            c.check(
                "changed annotation stays in the structural domain",
                &sample.pl,
            );
            sample.check(
                &mut c,
                &format!("{mode} {from} -> {to}"),
                Some(sample.mismatch()),
            );
        }
    }
    c.finish();
}

#[test]
fn a30_certification_diagnostics_name_vocabulary_not_schema_version() {
    let mut c = Cases::new("A30");
    for mode in ["doc", "query"] {
        let sample = CertCase::new(mode, true, false);
        let mut args = sample.argv(&c);
        for missing in ["missing.tsv", "."] {
            *args.last_mut().unwrap() = missing.into();
            let refs: Vec<_> = args.iter().map(String::as_str).collect();
            c.expect(
                &format!("{mode} unreadable table {missing}"),
                &refs,
                Out::err(
                    1,
                    format!("ckc: certify: {}: unreadable(vocabulary).\n", sample.id),
                ),
            );
        }
        let mut sample = CertCase::new(mode, true, false);
        sample.repin_table(Some("# malformed\n"));
        sample.check(&mut c, "table parse error", Some("vocabulary"));
        let mut sample = CertCase::new(mode, true, false);
        sample.pl = if mode == "doc" {
            sample.pl.replace(
                "guideline_schema_version(1).",
                "guideline_schema_version(2).",
            )
        } else {
            sample.pl.replace("'(v1,", "'(v2,")
        };
        sample.check(
            &mut c,
            "non-1 identifier is a codec reject",
            Some("noncanonical"),
        );
    }
    c.finish();
}
