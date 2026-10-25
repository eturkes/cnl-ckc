// S2 D1/D3/D5/D6, P1/P4/P5; oracle = contract + spec at 37b5dbd6.
// Clauses, proof trees and offsets are hand-modeled, never S2 output snapshots.
// CKC_S2_TEST_BIN selects a prebuilt executable for seed-red replay.
#[path = "../../../tests/bf/reorder.rs"]
mod reorder;
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
const CLASSES: [(&str, &str); 7] = [
    ("usuality", "usually"),
    ("ideality", "ideally"),
    ("focus", "particularly"),
    ("manner", "carefully"),
    ("force", "strongly"),
    ("possibility", "possibly"),
    ("applicability", "feasible"),
];
static SERIAL: AtomicUsize = AtomicUsize::new(0);

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn fixture(name: &str) -> String {
    fs::read_to_string(root().join("tests/s2-codec").join(name)).unwrap()
}
fn sha(bytes: impl AsRef<[u8]>) -> String {
    use sha2::Digest;
    sha2::Sha256::digest(bytes.as_ref())
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
fn digest(bytes: Option<&str>) -> String {
    bytes.map_or_else(|| "none".into(), |b| format!("sha256({})", atom(&sha(b))))
}
fn pair(name: &str, arity: usize) -> String {
    format!(":- multifile({name}/{arity}).\n:- discontiguous({name}/{arity}).\n")
}
fn declarations() -> String {
    INDICATORS.iter().map(|&(n, a)| pair(n, a)).collect()
}
fn gid(id: &str, slot: usize, deps: &str) -> String {
    format!("'$guideline_id'(product,{},1,ref({slot}),{deps})", atom(id))
}
fn context(id: &str, slot: usize, deps: &str) -> String {
    format!("'$guideline_id'(context,{},1,box({slot}),{deps})", atom(id))
}
fn lit(name: &str, args: &[&str]) -> String {
    format!("{name}({})", args.join(","))
}
fn qualifier(ctx: &str, event: &str, class: &str, word: &str) -> String {
    lit("guideline_qualifier", &[ctx, event, class, word])
}
fn entity(ctx: &str, referent: &str, noun: &str) -> Vec<String> {
    vec![
        lit("guideline_entity", &[ctx, referent, noun, "countable"]),
        lit("guideline_cardinality", &[ctx, referent, "na", "eq", "1"]),
    ]
}
fn event(ctx: &str, e: &str, verb: &str, subject: &str, object: &str) -> Vec<String> {
    vec![
        lit("guideline_event", &[ctx, e, verb]),
        lit("guideline_arg", &[ctx, e, "1", subject]),
        lit("guideline_arg", &[ctx, e, "2", object]),
    ]
}
fn conjunction(goals: &[String]) -> String {
    match goals {
        [last] => last.clone(),
        [first, rest @ ..] => format!("','({first},{})", conjunction(rest)),
        [] => panic!("nonempty conjunction"),
    }
}
fn query_text(
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
        ace.trim_end()
    )
}
fn query(id: &str, goal: &str) -> String {
    query_text(id, "Is there a result?\n", None, None, goal, "[]")
}
fn document_text(
    id: &str,
    ace: &str,
    ulex: Option<&str>,
    table: Option<&str>,
    clauses: &[String],
) -> String {
    format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex({}),vocabulary({})).\n% S1: {}\n{}",
        declarations(),
        atom(id),
        atom(&sha(ace)),
        digest(ulex),
        digest(table),
        ace.trim_end(),
        clauses
            .iter()
            .map(|c| format!("{c}.\n"))
            .collect::<String>()
    )
}
fn document(id: &str, clauses: &[String]) -> String {
    document_text(id, "A structural clause.\n", None, None, clauses)
}
fn proof(id: &str, witness: &[String], heads: &[String]) -> String {
    format!(
        "'$guideline_proof'({},1,variant(1),witness([{}]),prove([{}])).\n",
        atom(id),
        witness.join(","),
        heads.join(",")
    )
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
    fn summary(&self) -> String {
        fn clip(s: &str) -> String {
            let prefix: String = s.chars().take(240).collect();
            if prefix.len() == s.len() {
                prefix
            } else {
                format!("{prefix}… ({} bytes)", s.len())
            }
        }
        format!(
            "rc={} stdout={:?} stderr={:?}",
            self.rc,
            clip(&self.stdout),
            clip(&self.stderr)
        )
    }
}
fn execute(command: &mut Command) -> Out {
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
            panic!("S2 subprocess exceeded 30 s: {command:?}");
        }
        std::thread::sleep(Duration::from_millis(1));
    };
    Out {
        rc: status.code().unwrap_or(-1),
        stdout: String::from_utf8(out.join().unwrap()).unwrap(),
        stderr: String::from_utf8(err.join().unwrap()).unwrap(),
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
            ".scratch/s2-codec/run-{}-{}-{row}",
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
    fn write(&self, name: &str, text: &str) {
        fs::write(self.dir.join(name), text).unwrap();
    }
    fn run(&self, args: &[&str]) -> Out {
        let program = std::env::var_os("CKC_S2_TEST_BIN")
            .map(PathBuf::from)
            .unwrap_or_else(|| root().join("rust/target/release/ckc"));
        execute(Command::new(program).args(args).current_dir(&self.dir))
    }
    fn expect(&mut self, label: &str, args: &[&str], want: Out) {
        let got = self.run(args);
        self.total += 1;
        if got != want {
            self.failures.push(format!(
                "{label}: got {}; want {}",
                got.summary(),
                want.summary()
            ));
        } else if std::env::var_os("CKC_S2_REPORT_ALL").is_some() {
            println!("S2PASS {} {label}", self.row);
        }
    }
    fn check(&mut self, label: &str, text: &str) {
        self.write("input.pl", text);
        self.expect(label, &["v1", "check", "input.pl"], Out::ok(""));
        self.expect(
            &format!("{label}/roundtrip"),
            &["v1", "render", "input.pl"],
            Out::ok(text),
        );
    }
    fn reject_at(&mut self, label: &str, text: &str, at: usize) {
        self.write("input.pl", text);
        let prefix = &text.as_bytes()[..at];
        let line = prefix.iter().filter(|&&b| b == b'\n').count() + 1;
        let column = prefix
            .iter()
            .rposition(|&b| b == b'\n')
            .map_or(at + 1, |p| at - p);
        self.expect(
            label,
            &["v1", "check", "input.pl"],
            Out::err(
                2,
                format!("ace_to_pl_error(check_load,noncanonical({line},{column})).\n"),
            ),
        );
    }
    fn mutation(&mut self, label: &str, text: &str, from: &str, to: &str) {
        assert_eq!(text.matches(from).count(), 1, "{label}: unique anchor");
        let changed = text.replacen(from, to, 1);
        let at = text
            .bytes()
            .zip(changed.bytes())
            .position(|(a, b)| a != b)
            .unwrap_or(text.len().min(changed.len()));
        self.reject_at(label, &changed, at);
    }
    fn finish(self) {
        println!(
            "S2 {} checks={} passed={} failures={}",
            self.row,
            self.total,
            self.total - self.failures.len(),
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "S2 {}: {} of {} failed\n{}",
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

#[test]
fn c1a_eighteen_indicators_replace_seventeen() {
    let mut c = Cases::new("C1a");
    let text = document(
        "doc",
        &["guideline_entity(actual,a,patient,countable)".into()],
    );
    c.check("22-pair block", &text);
    c.mutation("21-pair block", &text, &pair("guideline_qualifier", 4), "");
    c.finish();
}
fn position_clause(literal: &str, position: usize) -> String {
    let head = "guideline_entity(actual,a,patient,countable)";
    match position {
        0 => literal.into(),
        1 => format!("{head} :- {literal}"),
        2 => format!("{head} :- \\+ {literal}"),
        3 => format!("{head} :- \\+ ({literal}, {head})"),
        _ => unreachable!(),
    }
}
fn term_domain() -> Vec<String> {
    let leaves = [
        "x",
        "-1",
        "'two words'",
        "'α'",
        "[]",
        "A",
        "2147483647",
        "-2147483648",
        "'it\\'s'",
        "'a\\\\b'",
    ];
    (0..40)
        .map(|n| {
            let mut term = leaves[n % leaves.len()].to_owned();
            for level in 0..n / leaves.len() {
                term = match (n + level) % 3 {
                    0 => format!("compound({term},[])"),
                    1 => format!("[{term},x]"),
                    _ => format!("[x|tail({term})]"),
                };
            }
            term
        })
        .collect()
}
#[test]
fn c1b_qualifier_terms_in_every_position() {
    let mut c = Cases::new("C1b-terms");
    // D1's arbitrary-term property: each argument, every clause position,
    // constructors of depth 0..3, variables, UTF-8/escaped atoms and int bounds.
    for term in term_domain() {
        for slot in 0..4 {
            let mut args = ["actual", "e", "manner", "carefully"];
            args[slot] = &term;
            let q = lit("guideline_qualifier", &args);
            for position in 0..4 {
                c.check(
                    &format!("slot={slot}/position={position}/{term}"),
                    &document("doc", &[position_clause(&q, position)]),
                );
            }
        }
    }
    assert_eq!(c.total, 1280);
    c.finish();
}
#[test]
fn c1b_raw_qualifier_questions() {
    let mut c = Cases::new("C1b-query");
    c.write("m.tsv", "");
    for (class, word) in CLASSES {
        let q = query("typed", &qualifier("actual", "e", class, word));
        c.check(class, &q);
        chain(
            &mut c,
            class,
            "typed",
            &q,
            "no(finite_failure)",
            "no(finite_failure)",
        );
    }
    for arity in [0, 1, 2, 3, 5, 6] {
        let goal = if arity == 0 {
            "guideline_qualifier".into()
        } else {
            lit("guideline_qualifier", &vec!["x"; arity])
        };
        c.write("query.pl", &query("typed", &goal));
        c.expect(&format!("raw-query-arity-{arity}"), &["v1", "answer", "m.tsv", "query.pl"], Out::err(2, format!("ace_to_pl_error(check_load,query_file(goal_foreign(guideline_qualifier,{arity}))).\n")));
    }
    c.finish();
}
#[test]
fn c1c_declaration_and_literal_boundaries() {
    let mut c = Cases::new("C1c");
    let text = document(
        "doc",
        &["guideline_entity(actual,a,patient,countable)".into()],
    );
    let pairs: Vec<_> = INDICATORS.iter().map(|&(n, a)| pair(n, a)).collect();
    for count in 0..22 {
        c.mutation(
            &format!("count={count}"),
            &text,
            &declarations(),
            &pairs[..count].concat(),
        );
    }
    c.mutation(
        "count=23",
        &text,
        &declarations(),
        &(declarations() + &pairs[17]),
    );
    for i in 0..21 {
        c.mutation(
            &format!("swapped-pairs-{i}"),
            &text,
            &(pairs[i].clone() + &pairs[i + 1]),
            &(pairs[i + 1].clone() + &pairs[i]),
        );
    }
    for directive in ["multifile", "discontiguous"] {
        let from = format!(":- {directive}(guideline_qualifier/4).\n");
        for arity in [0, 1, 3, 5, 10] {
            c.mutation(
                &format!("{directive}/arity={arity}"),
                &text,
                &from,
                &format!(":- {directive}(guideline_qualifier/{arity}).\n"),
            );
        }
        c.mutation(&format!("missing-{directive}"), &text, &from, "");
    }
    let q = qualifier("x", "x", "x", "x");
    for position in 0..4 {
        let doc = document("doc", &[position_clause(&q, position)]);
        for arity in [1, 2, 3, 5, 6] {
            c.mutation(
                &format!("position={position}/arity={arity}"),
                &doc,
                &q,
                &lit("guideline_qualifier", &vec!["x"; arity]),
            );
        }
        for bad in [
            "guideline_qualifie",
            "guideline_qualifierx",
            "'guideline_qualifier'",
        ] {
            c.mutation(
                &format!("position={position}/name={bad}"),
                &doc,
                &q,
                &q.replacen("guideline_qualifier", bad, 1),
            );
        }
    }
    c.finish();
}
#[test]
fn c1d_raw_qualifier_domain_is_not_codec_law() {
    let mut c = Cases::new("C1d");
    for class in [
        "other",
        "'Possibility'",
        "[]",
        "-1",
        "class(possibility)",
        "A",
    ] {
        for word in [
            "[]",
            "-1",
            "word(possibly)",
            "[possibly]",
            "'not a lemma'",
            "A",
        ] {
            // Shared A is deliberate: the reader checks terms, not certification's identities.
            let q = qualifier("other", "not_an_event", class, word);
            c.check(
                &format!("class={class}/word={word}"),
                &document("doc", std::slice::from_ref(&q)),
            );
            c.check("raw typed question", &query("typed", &q));
        }
    }
    c.finish();
}

#[derive(Clone)]
struct Member {
    id: String,
    clauses: Vec<String>,
    witness: Vec<String>,
    heads: Vec<String>,
}
impl Member {
    fn facts(id: &str, clauses: Vec<String>) -> Self {
        Self {
            id: id.into(),
            heads: clauses.clone(),
            clauses,
            witness: Vec::new(),
        }
    }
    fn rule(id: &str, heads: Vec<String>, positive: &[String], naf: &[String]) -> Self {
        let mut body = positive.to_vec();
        if let [single] = naf {
            body.push(format!("\\+ {single}"));
        } else if !naf.is_empty() {
            body.push(format!("\\+ ({})", naf.join(", ")));
        }
        let clauses = heads
            .iter()
            .map(|h| format!("{h} :- {}", body.join(", ")))
            .collect();
        let bind = |s: &str| {
            substitute(s, |v| {
                format!(
                    "'$guideline_id'(witness,{},1,ref({}),variant(1))",
                    atom(id),
                    v as u32 - 'A' as u32 + 1
                )
            })
        };
        Self {
            id: id.into(),
            clauses,
            witness: positive.iter().map(|s| bind(s)).collect(),
            heads: heads.iter().map(|s| bind(s)).collect(),
        }
    }
    fn trace(&self, clause: usize, children: &[String]) -> String {
        node(&self.id, &self.clauses[clause], children)
    }
    fn fact_traces(&self) -> Vec<String> {
        self.clauses
            .iter()
            .map(|c| node(&self.id, c, &[]))
            .collect()
    }
}
// The modeled rules use single-letter variables. Quoted source atoms stay opaque.
fn substitute(s: &str, value: impl Fn(char) -> String) -> String {
    let mut result = String::new();
    let mut quoted = false;
    let mut escaped = false;
    for ch in s.chars() {
        if escaped {
            result.push(ch);
            escaped = false;
        } else if quoted && ch == '\\' {
            result.push(ch);
            escaped = true;
        } else if ch == '\'' {
            quoted = !quoted;
            result.push(ch);
        } else if !quoted && ch.is_ascii_uppercase() {
            result.push_str(&value(ch));
        } else {
            result.push(ch);
        }
    }
    result
}
fn install(c: &Cases, members: &[Member], reverse: bool) {
    let order: Vec<_> = if reverse {
        members.iter().rev().collect()
    } else {
        members.iter().collect()
    };
    let mut manifest = String::new();
    for (i, m) in order.iter().enumerate() {
        c.write(&format!("d{i}.pl"), &document(&m.id, &m.clauses));
        c.write(&format!("d{i}.proof"), &proof(&m.id, &m.witness, &m.heads));
        manifest.push_str(&format!("d{i}.pl\td{i}.proof\n"));
    }
    c.write("m.tsv", &manifest);
}
fn answers(id: &str, q: &str, result: &str) -> String {
    format!(
        "% {id} answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,{},query_sha256({}),result({result})).\n",
        atom(id),
        atom(&sha(q))
    )
}
fn traces(id: &str, q: &str, a: &str, result: &str) -> String {
    format!(
        "% {id} traced against the loaded composition by ckc queries; do not edit.\n'$guideline_traces'(v1,{},query_sha256({}),answers_sha256({}),result({result})).\n",
        atom(id),
        atom(&sha(q)),
        atom(&sha(a))
    )
}
fn node(id: &str, clause: &str, children: &[String]) -> String {
    format!(
        "clause(sentence({},1),clause_sha256({}),[{}])",
        atom(id),
        atom(&sha(format!("{clause}.\n"))),
        children.join(",")
    )
}
fn chain(c: &mut Cases, label: &str, id: &str, q: &str, result: &str, trace_result: &str) {
    let a = answers(id, q, result);
    let t = traces(id, q, &a, trace_result);
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
    let nodes = trace_result.matches("clause(").count();
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
        if nodes == 0 {
            Out::err(1, "ace_to_pl_error(proof,trace_check(non_demo)).\n")
        } else {
            Out::ok(format!("ckc: trace-check ok {id} nodes={nodes}\n"))
        },
    );
}
fn boolean(c: &mut Cases, label: &str, goal: &str, roots: &[String]) {
    let q = query("typed", goal);
    if roots.is_empty() {
        chain(
            c,
            label,
            "typed",
            &q,
            "no(finite_failure)",
            "no(finite_failure)",
        );
    } else {
        chain(
            c,
            label,
            "typed",
            &q,
            "yes",
            &format!("yes(proved([{}]))", roots.join(",")),
        );
    }
}
fn transitive_facts(id: &str, subject: &str, object: &str, verb: &str) -> Member {
    let p = gid(id, 1, "[]");
    let o = gid(id, 2, "[]");
    let e = gid(id, 3, "[]");
    let mut clauses = entity("actual", &p, subject);
    clauses.extend(entity("actual", &o, object));
    clauses.extend(event("actual", &e, verb, &p, &o));
    Member::facts(id, clauses)
}
fn transitive_body(subject: &str, object: &str, verb: &str) -> Vec<String> {
    let mut body = entity("actual", "A", subject);
    body.extend(entity("actual", "B", object));
    body.extend(event("actual", "C", verb, "A", "B"));
    body
}

struct Possibility {
    members: Vec<Member>,
    fields: Vec<&'static str>,
}
impl Possibility {
    fn source_node(&self, field: &str) -> String {
        self.members[1].trace(
            self.fields.iter().position(|&f| f == field).unwrap(),
            &self.members[0].fact_traces(),
        )
    }
}
fn possibility(modal: bool, negative: bool) -> Possibility {
    let facts = transitive_facts("case", "patient", "pain", "have");
    let body = transitive_body("patient", "pain", "have");
    let mut heads = Vec::new();
    let mut fields = Vec::new();
    let mut ctx = "actual".to_owned();
    if modal {
        let b = context("claim", 1, "[A,B,C]");
        heads.push(lit("guideline_operator", &[&ctx, &b, "can"]));
        fields.push("can");
        ctx = b;
    }
    if negative {
        let b = context("claim", if modal { 2 } else { 1 }, "[A,B,C]");
        heads.push(lit("guideline_operator", &[&ctx, &b, "-"]));
        fields.push("negative");
        ctx = b;
    }
    let o = gid("claim", 4, "[A,B,C]");
    let e = gid("claim", 5, "[A,B,C]");
    heads.extend(entity(
        &ctx,
        &o,
        if negative { "opioid" } else { "medication" },
    ));
    fields.extend(["entity", "cardinality"]);
    heads.push(qualifier(&ctx, &e, "possibility", "possibly"));
    fields.push("qualifier");
    heads.extend(event(
        &ctx,
        &e,
        if negative { "take" } else { "need" },
        "A",
        &o,
    ));
    fields.extend(["event", "arg1", "arg2"]);
    let claim = Member::rule("claim", heads, &body, &[]);
    let consumer_body = if negative {
        let mut body = entity("actual", "A", "patient");
        body.push("guideline_operator(actual,B,-)".into());
        body.extend(entity("B", "C", "opioid"));
        body.extend(event("B", "D", "take", "A", "C"));
        body
    } else {
        transitive_body("patient", "medication", "need")
    };
    let deps = if negative { "[A]" } else { "[A,B,C]" };
    let o = gid("consumer", 4, deps);
    let e = gid("consumer", 5, deps);
    let mut heads = entity("actual", &o, "treatment");
    heads.extend(event(
        "actual",
        &e,
        if negative { "need" } else { "require" },
        "A",
        &o,
    ));
    let consumer = Member::rule("consumer", heads, &consumer_body, &[]);
    Possibility {
        members: vec![facts, claim, consumer],
        fields,
    }
}
#[test]
fn c4a_possible_claims_do_not_become_actual() {
    let mut c = Cases::new("C4a");
    // Native controls: consultant-3 possible-rule/actual-rule and
    // r2-negative-can/r2-negative-plain + their second-document consumers.
    // The actual+qualifier controls are raw clauses, not certifiable ACE.
    for negative in [false, true] {
        for modal in [false, true] {
            let p = possibility(modal, negative);
            for reverse in [false, true] {
                install(&c, &p.members, reverse);
                let label = format!("negative={negative}/can={modal}/reverse={reverse}");
                boolean(
                    &mut c,
                    &format!("{label}/typed"),
                    "guideline_qualifier(A,B,possibility,possibly)",
                    &[p.source_node("qualifier")],
                );
                let actual = if negative {
                    "','(guideline_operator(actual,A,-),guideline_event(A,B,take))"
                } else {
                    "guideline_event(actual,A,need)"
                };
                let actual_nodes = if modal {
                    vec![]
                } else if negative {
                    vec![p.source_node("negative"), p.source_node("event")]
                } else {
                    vec![p.source_node("event")]
                };
                boolean(&mut c, &format!("{label}/actual"), actual, &actual_nodes);
                let modal_goal = if negative {
                    "','(guideline_operator(actual,A,can),','(guideline_operator(A,B,-),guideline_event(B,C,take)))"
                } else {
                    "','(guideline_operator(actual,A,can),guideline_event(A,B,need))"
                };
                let modal_nodes = if !modal {
                    vec![]
                } else if negative {
                    vec![
                        p.source_node("can"),
                        p.source_node("negative"),
                        p.source_node("event"),
                    ]
                } else {
                    vec![p.source_node("can"), p.source_node("event")]
                };
                boolean(&mut c, &format!("{label}/modal"), modal_goal, &modal_nodes);
                let mut children = p.members[0].fact_traces()[..2].to_vec();
                if negative {
                    children.push(p.source_node("negative"));
                }
                for field in ["entity", "cardinality", "event", "arg1", "arg2"] {
                    children.push(p.source_node(field));
                }
                let downstream = if modal {
                    vec![]
                } else {
                    vec![p.members[2].trace(2, &children)]
                };
                boolean(
                    &mut c,
                    &format!("{label}/consumer"),
                    if negative {
                        "guideline_event(actual,A,need)"
                    } else {
                        "guideline_event(actual,A,require)"
                    },
                    &downstream,
                );
            }
        }
    }
    c.finish();
}

fn applicability(judgment: &str) -> Vec<Member> {
    let facts = transitive_facts("case", "clinician", "patient", "treat");
    let mut body = transitive_body("clinician", "patient", "treat");
    body.extend(entity("actual", "D", "treatment"));
    body.push("guideline_property(actual,E,feasible,pos)".into());
    body.extend(event("actual", "F", "be", "D", "E"));
    body.push(qualifier("actual", "F", "applicability", "feasible"));
    let b = context("guard", 1, "[A,B,C,D,E,F]");
    let r = gid("guard", 7, "[A,B,C,D,E,F]");
    let e = gid("guard", 8, "[A,B,C,D,E,F]");
    let mut heads = vec![lit("guideline_operator", &["actual", &b, "should"])];
    heads.extend(entity(&b, &r, "record"));
    heads.extend(event(&b, &e, "review", "A", &r));
    let guard = Member::rule("guard", heads, &body, &[]);
    let mut members = vec![facts, guard];
    if judgment != "absent" {
        let t = gid("judgment", 1, "[]");
        let p = gid("judgment", 2, "[]");
        let e = gid("judgment", 3, "[]");
        let mut clauses = entity("actual", &t, "treatment");
        clauses.push(lit(
            "guideline_property",
            &["actual", &p, "feasible", "pos"],
        ));
        clauses.extend(event("actual", &e, "be", &t, &p));
        if judgment != "primitive-only" {
            clauses.push(qualifier(
                if judgment == "other-context" {
                    "other"
                } else {
                    "actual"
                },
                if judgment == "other-event" {
                    "unrelated"
                } else {
                    &e
                },
                if judgment == "other-class" {
                    "possibility"
                } else {
                    "applicability"
                },
                if judgment == "other-word" {
                    "appropriate"
                } else {
                    "feasible"
                },
            ));
        }
        members.push(Member::facts("judgment", clauses));
    }
    members
}
#[test]
fn c4b_applicability_is_an_antecedent() {
    let mut c = Cases::new("C4b");
    // consultant-3 applicability-guard/positive: the copula remains a guard;
    // its primitive property alone does not satisfy the added qualifier goal.
    for state in [
        "absent",
        "primitive-only",
        "present",
        "other-context",
        "other-event",
        "other-class",
        "other-word",
    ] {
        let members = applicability(state);
        for reverse in [false, true] {
            install(&c, &members, reverse);
            let roots = if state == "present" {
                let mut children = members[0].fact_traces();
                children.extend(members[2].fact_traces());
                vec![
                    members[1].trace(0, &children),
                    members[1].trace(3, &children),
                ]
            } else {
                vec![]
            };
            boolean(
                &mut c,
                &format!("{state}/reverse={reverse}"),
                "','(guideline_operator(actual,A,should),guideline_event(A,B,review))",
                &roots,
            );
        }
    }
    c.finish();
}
fn naf_members(state: &str, grouped: bool) -> (Vec<Member>, Vec<String>) {
    let mut facts = transitive_facts("case", "patient", "opioid", "take");
    let e = gid("case", 3, "[]");
    if state != "absent" {
        facts.clauses.push(qualifier(
            if state == "other-context" {
                "other"
            } else {
                "actual"
            },
            if state == "other-event" {
                "unrelated"
            } else {
                &e
            },
            if state == "other-class" {
                "manner"
            } else {
                "usuality"
            },
            if state == "other-word" {
                "often"
            } else {
                "usually"
            },
        ));
        facts.heads = facts.clauses.clone();
    }
    let positive = transitive_body("patient", "opioid", "take");
    let mut naf = vec![qualifier(
        "actual",
        if grouped { "D" } else { "C" },
        "usuality",
        "usually",
    )];
    if grouped {
        naf.extend(event("actual", "D", "take", "A", "B"));
    }
    let head = lit(
        "guideline_event",
        &[
            "actual",
            &gid("naf-rule", if grouped { 5 } else { 4 }, "[A,B,C]"),
            "recommend",
        ],
    );
    let rule = Member::rule("naf-rule", vec![head], &positive, &naf);
    // A grouped ACE-shaped NAF has its own existential event D; its trace
    // retains that local variable rather than binding it to the positive C.
    let grounded = naf
        .iter()
        .map(|g| {
            substitute(g, |v| {
                if v == 'D' {
                    "'$VAR'(0)".into()
                } else {
                    gid("case", v as usize - 'A' as usize + 1, "[]")
                }
            })
        })
        .collect();
    (vec![facts, rule], grounded)
}
#[test]
fn c4c_naf_qualifier_membership() {
    let mut c = Cases::new("C4c");
    for grouped in [false, true] {
        for state in [
            "absent",
            "present",
            "other-context",
            "other-event",
            "other-class",
            "other-word",
        ] {
            let (members, inner) = naf_members(state, grouped);
            for reverse in [false, true] {
                install(&c, &members, reverse);
                let roots = if state == "present" {
                    vec![]
                } else {
                    let mut children = members[0].fact_traces()[..7].to_vec();
                    children.push(format!("naf({})", conjunction(&inner)));
                    vec![members[1].trace(0, &children)]
                };
                boolean(
                    &mut c,
                    &format!("grouped={grouped}/{state}/reverse={reverse}"),
                    "guideline_event(actual,A,recommend)",
                    &roots,
                );
            }
        }
    }
    c.finish();
}
fn consequent(class: &str, word: &str, ready: bool) -> Vec<Member> {
    let p = gid("case", 1, "[]");
    let mut facts = entity("actual", &p, "patient");
    if ready {
        facts.push(lit("guideline_property", &["actual", &p, "ready", "pos"]));
    }
    let mut body = entity("actual", "A", "patient");
    body.push("guideline_property(actual,A,ready,pos)".into());
    let b = context("policy", 1, "[A]");
    let e = gid("policy", 2, "[A]");
    let heads = vec![
        lit("guideline_operator", &["actual", &b, "should"]),
        lit("guideline_event", &[&b, &e, "review"]),
        qualifier(&b, &e, class, word),
        lit("guideline_event", &[&b, &gid("policy", 3, "[A]"), "review"]),
    ];
    vec![
        Member::facts("case", facts),
        Member::rule("policy", heads, &body, &[]),
    ]
}
#[test]
fn c4d_consequent_qualifier_is_data_only() {
    let mut c = Cases::new("C4d");
    for (class, word) in CLASSES {
        for ready in [false, true] {
            let members = consequent(class, word, ready);
            install(&c, &members, false);
            let roots = if ready {
                vec![members[1].trace(2, &members[0].fact_traces())]
            } else {
                vec![]
            };
            boolean(
                &mut c,
                &format!("{class}/ready={ready}/qualifier"),
                &qualifier("A", "B", class, word),
                &roots,
            );
            let roots = if ready {
                vec![
                    members[1].trace(1, &members[0].fact_traces()),
                    members[1].trace(2, &members[0].fact_traces()),
                ]
            } else {
                vec![]
            };
            boolean(
                &mut c,
                &format!("{class}/ready={ready}/same-head"),
                &conjunction(&[
                    "guideline_event(A,B,review)".into(),
                    qualifier("A", "B", class, word),
                ]),
                &roots,
            );
            let other = gid("policy", 3, "[B]");
            boolean(
                &mut c,
                &format!("{class}/ready={ready}/other-event"),
                &qualifier("A", &other, class, word),
                &[],
            );
            if ready {
                for goal in [
                    "guideline_event(actual,A,review)",
                    "guideline_event(actual,A,avoid)",
                    "guideline_operator(actual,A,must)",
                    "guideline_operator(actual,A,can)",
                    "guideline_operator(actual,A,may)",
                    "guideline_operator(actual,A,-)",
                    "guideline_frequency(A,B,C,D,E)",
                    "guideline_interval(A,B,C,D,E,F)",
                    "guideline_grade(A,B,C,D,E)",
                    "guideline_preference(A,B,C,D)",
                ] {
                    boolean(&mut c, &format!("{class}/no-extra/{goal}"), goal, &[]);
                }
            }
        }
    }
    c.finish();
}
#[test]
fn c4e_aggregate_and_recursion() {
    let mut c = Cases::new("C4e");
    let groups = [
        possibility(true, false).members,
        possibility(true, true).members,
        applicability("present"),
        naf_members("absent", false).0,
        naf_members("absent", true).0,
        consequent("ideality", "ideally", true),
    ];
    for (i, members) in groups.iter().enumerate() {
        let rule_count = members
            .iter()
            .flat_map(|m| &m.clauses)
            .filter(|s| s.contains(" :- "))
            .count();
        for reverse in [false, true] {
            install(&c, members, reverse);
            c.expect(
                &format!("family={i}/reverse={reverse}/aggregate"),
                &["v1", "aggregate-check", "m.tsv"],
                Out::ok(format!(
                    "ace_to_pl aggregate ok {} documents {} obligations\n",
                    members.len(),
                    members.len()
                )),
            );
            c.expect(
                &format!("family={i}/reverse={reverse}/recursion"),
                &["v1", "recursion-check", "m.tsv"],
                Out::ok(format!(
                    "ace_to_pl recursion ok {} documents {rule_count} rule clauses\n",
                    members.len()
                )),
            );
        }
    }
    let head = qualifier("actual", &gid("recursive", 1, "[]"), "manner", "carefully");
    install(
        &c,
        &[Member::rule("recursive", vec![head.clone()], &[head], &[])],
        false,
    );
    c.expect(
        "qualifier-left-recursion",
        &["v1", "recursion-check", "m.tsv"],
        Out::err(
            1,
            "ace_to_pl_error(proof,left_recursive(sentence(recursive,1),guideline_qualifier,4)).\n",
        ),
    );
    c.finish();
}

struct Cert {
    id: &'static str,
    pl: String,
    table: Option<String>,
}
fn cert_rule(id: &str) -> Member {
    let (positive, naf, slot, verb) = match id {
        "adverb-rule" => {
            let mut body = entity("actual", "A", "clinician");
            body.extend(entity("actual", "B", "record"));
            body.push(qualifier("actual", "C", "manner", "carefully"));
            body.extend(event("actual", "C", "review", "A", "B"));
            (body, vec![], 4, "wait")
        }
        "copula-rule" => {
            let mut body = entity("actual", "A", "treatment");
            body.push("guideline_property(actual,B,feasible,pos)".into());
            body.extend(event("actual", "C", "be", "A", "B"));
            body.push(qualifier("actual", "C", "applicability", "feasible"));
            (body, vec![], 4, "exist")
        }
        "naf-rule" => {
            let body = transitive_body("clinician", "record", "review");
            let mut naf = vec![qualifier("actual", "D", "manner", "carefully")];
            naf.extend(event("actual", "D", "review", "A", "B"));
            (body, naf, 5, "wait")
        }
        _ => unreachable!(),
    };
    let e = gid(id, slot, "[A,B,C]");
    Member::rule(
        id,
        vec![
            lit("guideline_event", &["actual", &e, verb]),
            lit("guideline_arg", &["actual", &e, "1", "A"]),
        ],
        &positive,
        &naf,
    )
}
fn cert_clauses(id: &str) -> Vec<String> {
    if id.ends_with("-rule") {
        return cert_rule(id).clauses;
    }
    let x = gid(id, 1, "[]");
    let p = gid(id, 2, "[]");
    let e = gid(id, 3, "[]");
    if id == "adverb" {
        let b = context(id, 1, "[]");
        let mut clauses = entity("actual", &x, "clinician");
        clauses.push(lit("guideline_operator", &["actual", &b, "should"]));
        clauses.extend(entity(&b, &p, "record"));
        clauses.push(qualifier(&b, &e, "ideality", "ideally"));
        clauses.extend(event(&b, &e, "review", &x, &p));
        clauses
    } else {
        assert_eq!(id, "copula");
        let mut clauses = entity("actual", &x, "treatment");
        clauses.push(lit(
            "guideline_property",
            &["actual", &p, "feasible", "pos"],
        ));
        clauses.push(lit(
            "guideline_property",
            &["actual", &p, "appropriate", "pos"],
        ));
        clauses.extend(event("actual", &e, "be", &x, &p));
        clauses.push(qualifier("actual", &e, "applicability", "feasible"));
        clauses.push(qualifier("actual", &e, "applicability", "appropriate"));
        clauses
    }
}
fn cert_question(goals: &[String], table: Option<&str>) -> String {
    query_text(
        "question",
        &fixture("question/ace.txt.in"),
        Some(&fixture("lexicon.ulex.in")),
        table,
        &conjunction(goals),
        "[]",
    )
}
fn question_goals() -> Vec<String> {
    entity("actual", "A", "patient")
}
impl Cert {
    fn new(id: &'static str) -> Self {
        let table = fixture("vocabulary.tsv.in");
        let pl = if id == "question" {
            cert_question(&question_goals(), Some(&table))
        } else {
            // The compiler runs rule bodies binding-first (contract bf).
            reorder::document(&document_text(
                id,
                &fixture(&format!("{id}/ace.txt.in")),
                Some(&fixture("lexicon.ulex.in")),
                Some(&table),
                &cert_clauses(id),
            ))
        };
        Self {
            id,
            pl,
            table: Some(table),
        }
    }
    fn repin(&mut self, table: Option<String>) {
        let old = format!("vocabulary({})", digest(self.table.as_deref()));
        assert_eq!(self.pl.matches(&old).count(), 1);
        self.pl = self.pl.replacen(
            &old,
            &format!("vocabulary({})", digest(table.as_deref())),
            1,
        );
        self.table = table;
    }
    fn run(&self, c: &mut Cases, label: &str, why: Option<&str>) {
        c.write("ace.txt", &fixture(&format!("{}/ace.txt.in", self.id)));
        c.write("dump.txt", &fixture(&format!("{}/dump.txt.in", self.id)));
        c.write("lexicon.ulex", &fixture("lexicon.ulex.in"));
        c.write("cert.pl", &self.pl);
        let mut args = vec![
            "certify-one",
            if self.id == "question" {
                "query"
            } else {
                "doc"
            },
            self.id,
            "ace.txt",
            "lexicon.ulex",
            "dump.txt",
            "cert.pl",
        ];
        if let Some(table) = &self.table {
            c.write("vocabulary.tsv", table);
            args.push("vocabulary.tsv");
        }
        let want = why.map_or_else(
            || {
                if self.id == "question" {
                    Out::ok("ckc: certify ok question\n")
                } else if self.id.ends_with("-rule") {
                    let m = cert_rule(self.id);
                    Out::ok(reorder::document(&proof(self.id, &m.witness, &m.heads)))
                } else {
                    let heads: Vec<_> = self
                        .pl
                        .split_once("% S1: ")
                        .unwrap()
                        .1
                        .lines()
                        .skip(1)
                        .map(|s| s.strip_suffix('.').unwrap().to_owned())
                        .collect();
                    Out::ok(proof(self.id, &[], &heads))
                }
            },
            |why| Out::err(1, format!("ckc: certify: {}: {why}.\n", self.id)),
        );
        c.expect(label, &args, want);
    }
}
fn qualifier_lines(pl: &str) -> Vec<String> {
    pl.lines()
        .filter(|s| s.starts_with("guideline_qualifier("))
        .map(|s| s.strip_suffix('.').unwrap().to_owned())
        .collect()
}
fn comma_items(text: &str) -> Vec<String> {
    let (mut depth, mut quoted, mut escaped, mut start) = (0, false, false, 0);
    let mut out = Vec::new();
    for (i, ch) in text.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '\'' {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        match ch {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                out.push(text[start..i].trim().into());
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(text[start..].trim().into());
    out
}
fn erase_goal(goal: &str) -> Option<String> {
    if goal.starts_with("guideline_qualifier(") {
        None
    } else if let Some(inner) = goal.strip_prefix("\\+ (").and_then(|s| s.strip_suffix(')')) {
        let goals: Vec<_> = comma_items(inner)
            .iter()
            .filter_map(|s| erase_goal(s))
            .collect();
        match goals.as_slice() {
            [] => None,
            [single] => Some(format!("\\+ {single}")),
            _ => Some(format!("\\+ ({})", goals.join(", "))),
        }
    } else if let Some(inner) = goal.strip_prefix("\\+ ") {
        erase_goal(inner).map(|s| format!("\\+ {s}"))
    } else {
        Some(goal.into())
    }
}
fn erase_qualifiers(pl: &str) -> String {
    pl.lines()
        .filter_map(|line| {
            if line.starts_with("guideline_qualifier(") {
                None
            } else if let Some((head, body)) = line.split_once(" :- ") {
                let goals: Vec<_> = comma_items(body.strip_suffix('.').unwrap())
                    .iter()
                    .filter_map(|s| erase_goal(s))
                    .collect();
                Some(if goals.is_empty() {
                    format!("{head}.\n")
                } else {
                    format!("{head} :- {}.\n", goals.join(", "))
                })
            } else {
                Some(format!("{line}\n"))
            }
        })
        .collect()
}
fn arguments(literal: &str) -> Vec<String> {
    let args = comma_items(
        literal
            .strip_prefix("guideline_qualifier(")
            .unwrap()
            .strip_suffix(')')
            .unwrap(),
    );
    assert_eq!(args.len(), 4);
    args
}
fn replace_qualifier(sample: &mut Cert, old: &str, new: &str) {
    let before = erase_qualifiers(&sample.pl);
    let anchor = format!("{old}.\n");
    assert_eq!(sample.pl.matches(&anchor).count(), 1);
    sample.pl = sample.pl.replacen(&anchor, &format!("{new}.\n"), 1);
    assert_eq!(
        erase_qualifiers(&sample.pl),
        before,
        "primitive projection unchanged"
    );
}
#[test]
fn c5_clean_certification_and_reclassification() {
    let mut c = Cases::new("C5-controls");
    for id in [
        "adverb",
        "copula",
        "adverb-rule",
        "copula-rule",
        "naf-rule",
        "question",
    ] {
        let sample = Cert::new(id);
        c.check(id, &sample.pl);
        sample.run(&mut c, id, None);
    }
    // D3.7 permits every class under should, and every class on a copula.
    // The same lemma reclassified changes only Class plus the table digest.
    for id in ["adverb", "copula"] {
        for (class, _) in CLASSES {
            let mut sample = Cert::new(id);
            let old = if id == "adverb" {
                "ideality"
            } else {
                "applicability"
            };
            let table = sample
                .table
                .as_ref()
                .unwrap()
                .replace(&format!("\t{old}\n"), &format!("\t{class}\n"));
            sample.repin(Some(table));
            for literal in qualifier_lines(&sample.pl) {
                let args = arguments(&literal);
                let changed = qualifier(&args[0], &args[1], class, &args[3]);
                replace_qualifier(&mut sample, &literal, &changed);
            }
            sample.run(&mut c, &format!("{id}/class={class}"), None);
        }
    }
    c.finish();
}
#[test]
fn c5a_wrong_qualifier_arguments() {
    let mut c = Cases::new("C5a");
    for id in ["adverb", "copula"] {
        let clean = Cert::new(id);
        for literal in qualifier_lines(&clean.pl) {
            let args = arguments(&literal);
            let domains = [
                vec!["other".into(), "actual".into(), context("other", 1, "[]")],
                vec![
                    "other".into(),
                    gid(id, 2, "[]"),
                    gid("other", 3, "[]"),
                    gid(id, 3, "[a]"),
                ],
                CLASSES.iter().map(|(k, _)| (*k).to_owned()).collect(),
                vec![
                    "other".into(),
                    "ideally".into(),
                    "feasible".into(),
                    "appropriate".into(),
                    "'Ideally'".into(),
                ],
            ];
            for (slot, values) in domains.iter().enumerate() {
                for value in values {
                    if value == &args[slot] {
                        continue;
                    }
                    let mut sample = Cert::new(id);
                    let mut changed = args.clone();
                    changed[slot] = value.clone();
                    replace_qualifier(
                        &mut sample,
                        &literal,
                        &qualifier(&changed[0], &changed[1], &changed[2], &changed[3]),
                    );
                    let label = format!("{id}/{}/slot={slot}/{value}", args[3]);
                    c.check(&format!("{label}/canonical"), &sample.pl);
                    sample.run(&mut c, &label, Some("clauses(1)"));
                }
            }
        }
    }
    c.finish();
}
#[test]
fn c5b_qualifier_projection_mutants() {
    let mut c = Cases::new("C5b");
    for id in ["adverb", "copula"] {
        let clean = Cert::new(id);
        for literal in qualifier_lines(&clean.pl) {
            for action in ["drop", "add", "misplace", "duplicate"] {
                let mut sample = Cert::new(id);
                let line = format!("{literal}.\n");
                sample.pl = match action {
                    "drop" => sample.pl.replacen(&line, "", 1),
                    "duplicate" => sample.pl.replacen(&line, &line.repeat(2), 1),
                    "add" => {
                        let args = arguments(&literal);
                        let extra = qualifier("other", &args[1], &args[2], &args[3]);
                        sample.pl.replacen(&line, &format!("{line}{extra}.\n"), 1)
                    }
                    "misplace" => {
                        let without = sample.pl.replacen(&line, "", 1);
                        let comment = format!(
                            "% S1: {}\n",
                            fixture(&format!("{id}/ace.txt.in")).trim_end()
                        );
                        without.replacen(&comment, &format!("{comment}{line}"), 1)
                    }
                    _ => unreachable!(),
                };
                assert_ne!(sample.pl, clean.pl);
                assert_eq!(erase_qualifiers(&sample.pl), erase_qualifiers(&clean.pl));
                let label = format!("{id}/{}/{action}", arguments(&literal)[3]);
                c.check(&format!("{label}/canonical"), &sample.pl);
                sample.run(&mut c, &label, Some("clauses(1)"));
            }
        }
    }
    c.finish();
}
#[test]
fn c5c_vocabulary_custody() {
    let mut c = Cases::new("C5c-table");
    for id in ["adverb", "copula", "question"] {
        let mut sample = Cert::new(id);
        sample
            .table
            .as_mut()
            .unwrap()
            .push_str("unit\tunused-hour\thour\n");
        sample.run(
            &mut c,
            &format!("{id}/changed-bytes-old-digest"),
            Some("vocabulary"),
        );
        let mut sample = Cert::new(id);
        let wanted = sample.table.clone();
        sample.repin(Some("wrong digest\n".into()));
        sample.table = wanted;
        sample.run(&mut c, &format!("{id}/wrong-digest"), Some("vocabulary"));
        let mut sample = Cert::new(id);
        sample.table = None;
        sample.run(&mut c, &format!("{id}/missing-bytes"), Some("vocabulary"));
        let mut sample = Cert::new(id);
        let wanted = sample.table.clone();
        sample.repin(None);
        sample.table = wanted;
        sample.run(&mut c, &format!("{id}/none-with-bytes"), Some("vocabulary"));
        for bad in [
            "# malformed\n",
            "# format: kind\tlemma\tvalue\nqualifier\tideally\tother\n",
            "# format: kind\tlemma\tvalue\nqualifier\tideally\tideality",
        ] {
            let mut sample = Cert::new(id);
            sample.repin(Some(bad.into()));
            sample.run(
                &mut c,
                &format!("{id}/bad-table/{bad:?}"),
                Some("vocabulary"),
            );
        }
    }
    for id in ["adverb", "copula"] {
        let mut sample = Cert::new(id);
        let table = sample
            .table
            .as_ref()
            .unwrap()
            .replace("\tideality\n", "\tmanner\n")
            .replace("\tapplicability\n", "\tpossibility\n");
        sample.repin(Some(table));
        sample.run(
            &mut c,
            &format!("{id}/valid-remapping-stale-literals"),
            Some("clauses(1)"),
        );
    }
    c.finish();
}
#[test]
fn c5c_qualifier_without_table() {
    let mut c = Cases::new("C5c-none");
    let mut sample = Cert::new("copula");
    sample.repin(None);
    c.check("no-table qualifier loads", &sample.pl);
    sample.run(&mut c, "no-table qualifier", Some("clauses(1)"));
    sample.pl = erase_qualifiers(&sample.pl);
    sample.run(&mut c, "no-table primitive control", None);
    c.finish();
}
#[test]
fn c5d_committed_question_cannot_gain_a_qualifier() {
    let mut c = Cases::new("C5d");
    for slot in 0..=2 {
        for table in [false, true] {
            let mut sample = Cert::new("question");
            if !table {
                sample.repin(None);
            }
            let mut goals = question_goals();
            goals.insert(slot, qualifier("actual", "e", "possibility", "possibly"));
            assert_eq!(
                goals
                    .iter()
                    .filter(|g| !g.starts_with("guideline_qualifier("))
                    .cloned()
                    .collect::<Vec<_>>(),
                question_goals()
            );
            sample.pl = cert_question(&goals, sample.table.as_deref());
            c.check(&format!("slot={slot}/table={table}/raw-query"), &sample.pl);
            sample.run(
                &mut c,
                &format!("slot={slot}/table={table}"),
                Some("projection"),
            );
        }
    }
    c.finish();
}
#[test]
fn c5e_out_of_domain_qualifier_has_no_derivation() {
    let mut c = Cases::new("C5e");
    for id in ["adverb", "copula"] {
        for literal in qualifier_lines(&Cert::new(id).pl) {
            let args = arguments(&literal);
            for (slot, values) in [
                (
                    2,
                    vec!["other", "'Possibility'", "-1", "[]", "class(manner)", "A"],
                ),
                (3, vec!["-1", "[]", "[ideally]", "word(ideally)", "A"]),
            ] {
                for value in values {
                    let mut sample = Cert::new(id);
                    let mut changed = args.clone();
                    changed[slot] = value.into();
                    replace_qualifier(
                        &mut sample,
                        &literal,
                        &qualifier(&changed[0], &changed[1], &changed[2], &changed[3]),
                    );
                    let label = format!("{id}/{}/slot={slot}/{value}", args[3]);
                    c.check(&format!("{label}/codec"), &sample.pl);
                    sample.run(&mut c, &label, Some("clauses(1)"));
                }
            }
        }
    }
    c.finish();
}

#[test]
fn c5ab_antecedent_and_naf_qualifier_mutants() {
    let mut c = Cases::new("C5ab-rule");
    for id in ["adverb-rule", "copula-rule", "naf-rule"] {
        let clean = Cert::new(id);
        let copula = id == "copula-rule";
        let e = if id == "naf-rule" { "D" } else { "C" };
        let q = qualifier(
            "actual",
            e,
            if copula { "applicability" } else { "manner" },
            if copula { "feasible" } else { "carefully" },
        );
        assert_eq!(clean.pl.matches(&q).count(), 2, "one goal per split head");
        for slot in 0..4 {
            let mut sample = Cert::new(id);
            let mut args = arguments(&q);
            args[slot] = "other".into();
            sample.pl =
                sample
                    .pl
                    .replacen(&q, &qualifier(&args[0], &args[1], &args[2], &args[3]), 1);
            assert_eq!(erase_qualifiers(&sample.pl), erase_qualifiers(&clean.pl));
            c.check(&format!("{id}/slot={slot}/canonical"), &sample.pl);
            sample.run(&mut c, &format!("{id}/slot={slot}"), Some("clauses(1)"));
        }
        for action in ["drop", "add", "misplace", "duplicate"] {
            let mut sample = Cert::new(id);
            sample.pl = match action {
                "drop" => {
                    if copula {
                        sample.pl.replacen(&format!(", {q}"), "", 1)
                    } else {
                        sample.pl.replacen(&format!("{q}, "), "", 1)
                    }
                }
                "duplicate" => sample.pl.replacen(&q, &format!("{q}, {q}"), 1),
                "add" => sample.pl.replacen(
                    &q,
                    &format!("{q}, {}", qualifier("other", e, "manner", "carefully")),
                    1,
                ),
                "misplace" => {
                    // Swap the qualifier with the goal before it (binding-first order,
                    // contract bf, can put a different goal there than the DRS order did).
                    // A qualifier opening a NAF group swaps with the goal after it.
                    match sample.pl.find(&format!(", {q}")) {
                        Some(at) => {
                            let start = sample.pl[..at].rfind(", ").map_or_else(
                                || sample.pl[..at].rfind(" :- ").unwrap() + 4,
                                |i| i + 2,
                            );
                            let neighbor = sample.pl[start..at].to_owned();
                            sample.pl.replacen(
                                &format!("{neighbor}, {q}"),
                                &format!("{q}, {neighbor}"),
                                1,
                            )
                        }
                        None => {
                            let from = sample
                                .pl
                                .find(&format!("{q}, "))
                                .expect("qualifier beside a goal")
                                + q.len()
                                + 2;
                            let end = from + sample.pl[from..].find(", ").unwrap();
                            let neighbor = sample.pl[from..end].to_owned();
                            sample.pl.replacen(
                                &format!("{q}, {neighbor}"),
                                &format!("{neighbor}, {q}"),
                                1,
                            )
                        }
                    }
                }
                _ => unreachable!(),
            };
            assert_ne!(sample.pl, clean.pl);
            assert_eq!(erase_qualifiers(&sample.pl), erase_qualifiers(&clean.pl));
            c.check(&format!("{id}/{action}/canonical"), &sample.pl);
            sample.run(&mut c, &format!("{id}/{action}"), Some("clauses(1)"));
        }
        if id == "naf-rule" {
            let mut sample = Cert::new(id);
            sample.pl = sample
                .pl
                .replacen(&format!("\\+ ({q}, "), &format!("{q}, \\+ ("), 1);
            assert_ne!(sample.pl, clean.pl);
            assert_eq!(erase_qualifiers(&sample.pl), erase_qualifiers(&clean.pl));
            c.check("NAF qualifier moved to positive body", &sample.pl);
            sample.run(&mut c, "NAF membership changed", Some("clauses(1)"));
        }
    }
    c.finish();
}
