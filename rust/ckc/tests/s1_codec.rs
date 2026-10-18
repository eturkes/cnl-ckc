// S1 D1/D3–D7/P1/P4/P5 + S2 D1/P9; oracle = the contracts.
// C1 fixture bytes + explicit D3/D4 insertions are the oracle, never S1 output.
// CKC_S1_TEST_BIN selects a prebuilt executable; compiler = this tree's fork.
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

const INDICATORS: [(&str, usize); 19] = [
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

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn fixture(name: &str) -> String {
    fs::read_to_string(root().join("tests/s1-codec").join(name)).unwrap()
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
fn declarations() -> String {
    INDICATORS
        .iter()
        .map(|(n, a)| format!(":- multifile({n}/{a}).\n:- discontiguous({n}/{a}).\n"))
        .collect()
}
fn gid(id: &str, sentence: usize, slot: usize) -> String {
    format!(
        "'$guideline_id'(product,{},{sentence},ref({slot}),[])",
        atom(id)
    )
}
fn context(id: &str, sentence: usize, slot: usize) -> String {
    format!(
        "'$guideline_id'(context,{},{sentence},box({slot}),[])",
        atom(id)
    )
}
fn compound(name: &str, args: &[&str]) -> String {
    format!("{name}({})", args.join(","))
}
fn conjunction(goals: &[String]) -> String {
    match goals {
        [last] => last.clone(),
        [first, rest @ ..] => format!("','({first},{})", conjunction(rest)),
        [] => panic!("nonempty query"),
    }
}
fn query(id: &str, goal: &str, answers: &str) -> String {
    let ace = "Which result exists?\n";
    format!(
        "% {id} compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,{},ace_sha256({}),ulex(none),vocabulary(none)).\n% Q1: {}\n'$guideline_query_projection'(goal({goal}),answers({answers})).\n",
        atom(id),
        atom(&sha(ace)),
        ace.trim_end()
    )
}
fn document(id: &str, clauses: &[String]) -> String {
    format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex(none),vocabulary(none)).\n% S1: A structural clause.\n{}",
        declarations(),
        atom(id),
        atom(&sha("A structural clause.\n")),
        clauses
            .iter()
            .map(|c| format!("{c}.\n"))
            .collect::<String>()
    )
}
fn proof(id: &str, sentence: usize, witness: &[String], heads: &[String]) -> String {
    format!(
        "'$guideline_proof'({},{sentence},variant(1),witness([{}]),prove([{}])).\n",
        atom(id),
        witness.join(","),
        heads.join(",")
    )
}
fn fact_proof(id: &str, doc: &str) -> String {
    let mut out = String::new();
    let mut heads = Vec::new();
    let mut sentence = 0;
    for line in doc.lines() {
        if let Some(comment) = line.strip_prefix("% S") {
            if sentence != 0 {
                out.push_str(&proof(id, sentence, &[], &heads));
            }
            sentence = comment.split_once(':').unwrap().0.parse().unwrap();
            heads.clear();
        } else if sentence != 0 {
            assert!(!line.contains(" :- "), "fact fixture only");
            heads.push(line.strip_suffix('.').unwrap().into());
        }
    }
    if sentence != 0 {
        out.push_str(&proof(id, sentence, &[], &heads));
    }
    out
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
fn execute(command: &mut Command, input: &str) -> Out {
    let mut child = command
        .stdin(Stdio::piped())
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
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("S1 subprocess exceeded 30 s: {command:?}");
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
            ".scratch/s1-codec/run-{}-{}-{row}",
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
        let program = std::env::var_os("CKC_S1_TEST_BIN")
            .map(PathBuf::from)
            .unwrap_or_else(|| root().join("rust/target/release/ckc"));
        execute(Command::new(program).args(args).current_dir(&self.dir), "")
    }
    fn compare(&mut self, label: &str, got: Out, want: Out) {
        self.total += 1;
        if got != want {
            self.failures
                .push(format!("{label}: got {got:?}; want {want:?}"));
        }
    }
    fn expect(&mut self, label: &str, args: &[&str], want: Out) {
        let got = self.run(args);
        self.compare(label, got, want);
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
            "S1 {} checks={} failures={}",
            self.row,
            self.total,
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "S1 {}: {} of {} failed\n{}",
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
fn a01_seventeen_indicators_and_r9_boundaries() {
    let mut c = Cases::new("A1");
    let text = document(
        "doc",
        &["guideline_entity(actual,a,patient,countable)".into()],
    );
    c.check("19-pair block", &text);
    let pairs: Vec<_> = INDICATORS
        .iter()
        .map(|(n, a)| format!(":- multifile({n}/{a}).\n:- discontiguous({n}/{a}).\n"))
        .collect();
    for count in [15, 16, 17, 18] {
        c.mutation(
            &format!("{count}-pair block"),
            &text,
            &declarations(),
            &pairs[..count].concat(),
        );
    }
    c.mutation(
        "20-pair block",
        &text,
        &declarations(),
        &(declarations() + &pairs[17]),
    );
    c.mutation(
        "swapped new pairs",
        &text,
        &(pairs[15].clone() + &pairs[16]),
        &(pairs[16].clone() + &pairs[15]),
    );
    for &(name, arity) in &INDICATORS[15..] {
        for directive in ["multifile", "discontiguous"] {
            let from = format!(":- {directive}({name}/{arity}).\n");
            for wrong in [arity - 1, arity + 1] {
                c.mutation(
                    &format!("{directive}/{name}/{wrong}"),
                    &text,
                    &from,
                    &format!(":- {directive}({name}/{wrong}).\n"),
                );
            }
        }
        for position in 0..4 {
            let lit = compound(name, &vec!["x"; arity]);
            let clause = position_clause(&lit, position);
            let doc = document("doc", &[clause]);
            for wrong in [arity - 1, arity + 1] {
                c.mutation(
                    &format!("{name}/{position}/arity-{wrong}"),
                    &doc,
                    &lit,
                    &compound(name, &vec!["x"; wrong]),
                );
            }
        }
    }
    c.finish();
}
fn position_clause(lit: &str, position: usize) -> String {
    let head = "guideline_entity(actual,a,patient,countable)";
    match position {
        0 => lit.into(),
        1 => format!("{head} :- {lit}"),
        2 => format!("{head} :- \\+ {lit}"),
        _ => format!("{head} :- \\+ ({lit}, {head})"),
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
    ];
    (0..32)
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
fn a02_new_literals_are_structural_in_every_position() {
    let mut c = Cases::new("A2");
    // Generated canonical terms: depth 0..3, variable/atom/int/list/compound.
    // Values, endpoints and context are certification law, never codec law.
    for &(name, arity) in &INDICATORS[15..] {
        for data in term_domain() {
            let lit = compound(name, &vec![data.as_str(); arity]);
            for position in 0..4 {
                c.check(
                    &format!("{name}/{position}/{data}"),
                    &document("doc", &[position_clause(&lit, position)]),
                );
            }
        }
    }
    assert_eq!(c.total, 1024);
    c.finish();
}
#[test]
fn a03_raw_typed_questions_load_and_answer() {
    let mut c = Cases::new("A3");
    c.write("m.tsv", "");
    for &(name, arity) in &INDICATORS[15..] {
        let goal = compound(name, &vec!["x"; arity]);
        let q = query("typed", &goal, "[]");
        c.check(name, &q);
        chain(
            &mut c,
            name,
            "typed",
            &q,
            "no(finite_failure)",
            "no(finite_failure)",
            0,
        );
        for n in [arity - 1, arity + 1] {
            let q = query("typed", &compound(name, &vec!["x"; n]), "[]");
            c.write("query.pl", &q);
            c.expect(
                "query wrong arity",
                &["v1", "answer", "m.tsv", "query.pl"],
                Out::err(
                    2,
                    format!("ace_to_pl_error(check_load,query_file(goal_foreign({name},{n}))).\n"),
                ),
            );
        }
    }
    c.finish();
}

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
    fn text(&self) -> String {
        document(&self.id, &self.clauses)
    }
}
fn install(c: &Cases, members: &[Member]) {
    let mut manifest = String::new();
    for (i, m) in members.iter().enumerate() {
        c.write(&format!("d{i}.pl"), &m.text());
        c.write(
            &format!("d{i}.proof"),
            &proof(&m.id, 1, &m.witness, &m.heads),
        );
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
fn node(id: &str, sentence: usize, clause: &str, children: &[String]) -> String {
    format!(
        "clause(sentence({},{sentence}),clause_sha256({}),[{}])",
        atom(id),
        atom(&sha(format!("{clause}.\n"))),
        children.join(",")
    )
}
fn chain(
    c: &mut Cases,
    label: &str,
    id: &str,
    q: &str,
    result: &str,
    trace_result: &str,
    nodes: usize,
) {
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
fn graded_members() -> Vec<Member> {
    [
        ("alpha", Some("strong")),
        ("beta", Some("conditional")),
        ("gamma", None),
    ]
    .into_iter()
    .map(|(id, value)| {
        let r = gid(id, 1, 1);
        let label = gid(id, 1, 2);
        let e = gid(id, 1, 3);
        let mut clauses = vec![
            format!("guideline_entity(actual,{r},recommendation,countable)"),
            format!("guideline_entity(actual,{label},grade,countable)"),
            format!("guideline_event(actual,{e},wait)"),
        ];
        if let Some(value) = value {
            clauses.push(format!(
                "guideline_grade(actual,{r},strength,{value},{label})"
            ));
        }
        Member::facts(id, clauses)
    })
    .collect()
}
#[test]
fn a05_grade_join_is_document_local() {
    let mut c = Cases::new("A5");
    for reverse in [false, true] {
        let mut members = graded_members();
        for (value, id) in [("strong", "alpha"), ("conditional", "beta")] {
            if reverse {
                members.reverse();
            }
            install(&c, &members);
            let goal = conjunction(&[
                format!(
                    "guideline_grade(actual,'$guideline_id'(product,A,B,ref(C),[]),strength,{value},D)"
                ),
                "guideline_event(actual,'$guideline_id'(product,A,E,ref(F),[]),wait)".into(),
            ]);
            let q = query("graded", &goal, "[answer(A,wh(what))]");
            let m = members.iter().find(|m| m.id == id).unwrap();
            let nodes = [
                node(id, 1, &m.clauses[3], &[]),
                node(id, 1, &m.clauses[2], &[]),
            ];
            chain(
                &mut c,
                &format!("{value}/reverse={reverse}"),
                "graded",
                &q,
                &format!("solutions([sol([{id}])])"),
                &format!("solutions([sol([{id}],proved([{}]))])", nodes.join(",")),
                2,
            );
        }
        // A recommendation and a same-noun statement in another document share actual,
        // but not the grade target's DocId.
        let mut members = graded_members();
        members[0].clauses.remove(2);
        members[0].heads = members[0].clauses.clone();
        install(&c, &members);
        let q = query(
            "ungraded",
            "','(guideline_grade(actual,'$guideline_id'(product,A,B,ref(C),[]),strength,strong,D),guideline_event(actual,'$guideline_id'(product,A,E,ref(F),[]),wait))",
            "[]",
        );
        chain(
            &mut c,
            "shared actual does not transfer grade",
            "ungraded",
            &q,
            "no(finite_failure)",
            "no(finite_failure)",
            0,
        );
    }
    let members = graded_members();
    install(&c, &members);
    let alpha = &members[0];
    let root_facts = vec![
        alpha.clauses[0].clone(),
        alpha.clauses[1].clone(),
        alpha.clauses[3].clone(),
    ];
    let later = format!("guideline_event(actual,{},wait)", gid("alpha", 2, 3));
    let doc = document("alpha", &root_facts) + &format!("% S2: A patient waits.\n{later}.\n");
    c.write("d0.pl", &doc);
    c.write(
        "d0.proof",
        &(proof("alpha", 1, &[], &root_facts)
            + &proof("alpha", 2, &[], std::slice::from_ref(&later))),
    );
    let q = query(
        "cross-sentence",
        "','(guideline_grade(actual,'$guideline_id'(product,A,B,ref(C),[]),strength,strong,D),guideline_event(actual,'$guideline_id'(product,A,E,ref(F),[]),wait))",
        "[answer(A,wh(what))]",
    );
    let nodes = [
        node("alpha", 1, &alpha.clauses[3], &[]),
        node("alpha", 2, &later, &[]),
    ];
    chain(
        &mut c,
        "DocId join crosses the sentence boundary",
        "cross-sentence",
        &q,
        "solutions([sol([alpha])])",
        &format!("solutions([sol([alpha],proved([{}]))])", nodes.join(",")),
        2,
    );
    c.expect(
        "cross-sentence aggregate",
        &["v1", "aggregate-check", "m.tsv"],
        Out::ok("ace_to_pl aggregate ok 3 documents 4 obligations\n"),
    );
    c.finish();
}
fn patient_members(ready: bool, blocked: bool) -> Vec<Member> {
    let p = gid("patients", 1, 1);
    let other = gid("patients", 1, 2);
    let mut facts = vec![
        format!("guideline_entity(actual,{p},patient,countable)"),
        format!("guideline_entity(actual,{other},patient,countable)"),
    ];
    if ready {
        facts.push(format!("guideline_property(actual,{p},ready,pos)"));
    }
    if blocked {
        facts.push(format!(
            "guideline_property(actual,{p},contraindicated,pos)"
        ));
    }
    let head = "guideline_preference(actual,'$guideline_id'(product,policy,1,ref(2),[A]),'$guideline_id'(product,policy,1,ref(3),[A]),none)";
    let body = "guideline_entity(actual,A,patient,countable), guideline_property(actual,A,ready,pos), \\+ guideline_property(actual,A,contraindicated,pos)";
    let witness = "'$guideline_id'(witness,policy,1,variant(1),ref(1))";
    vec![
        Member::facts("patients", facts),
        Member {
            id: "policy".into(),
            clauses: vec![format!("{head} :- {body}")],
            witness: vec![
                format!("guideline_entity(actual,{witness},patient,countable)"),
                format!("guideline_property(actual,{witness},ready,pos)"),
            ],
            heads: vec![head.replace("[A]", &format!("[{witness}]"))],
        },
    ]
}
fn patient_query() -> String {
    query(
        "preferred",
        "guideline_preference(actual,'$guideline_id'(product,policy,1,ref(2),[A]),B,none)",
        "[answer(A,noun(patient,countable)),answer(B,noun(medicine,countable))]",
    )
}
#[test]
fn a06_preference_answers_and_naf_traces_are_patient_bound() {
    let mut c = Cases::new("A6");
    for reverse in [false, true] {
        let mut members = patient_members(true, false);
        let p = gid("patients", 1, 1);
        let medicine = format!("'$guideline_id'(product,policy,1,ref(3),[{p}])");
        let children = [
            node("patients", 1, &members[0].clauses[0], &[]),
            node("patients", 1, &members[0].clauses[2], &[]),
            format!("naf(guideline_property(actual,{p},contraindicated,pos))"),
        ];
        let n = node("policy", 1, &members[1].clauses[0], &children);
        if reverse {
            members.reverse();
        }
        install(&c, &members);
        chain(
            &mut c,
            "one matching patient",
            "preferred",
            &patient_query(),
            &format!("solutions([sol([{p},{medicine}])])"),
            &format!("solutions([sol([{p},{medicine}],proved([{n}]))])"),
            3,
        );
    }
    c.finish();
}
#[test]
fn a07_missing_condition_and_contraindication_block() {
    let mut c = Cases::new("A7");
    for (ready, blocked) in [(false, false), (false, true), (true, true)] {
        install(&c, &patient_members(ready, blocked));
        chain(
            &mut c,
            &format!("ready={ready}/contraindicated={blocked}"),
            "preferred",
            &patient_query(),
            "solutions([])",
            "solutions([])",
            0,
        );
    }
    c.finish();
}

struct Compiler {
    scratch: Cases,
    tree: PathBuf,
}
impl Compiler {
    fn new() -> Self {
        let scratch = Cases::new("compiler");
        let tree = scratch.dir.join("ape");
        let out = Command::new("cp")
            .args(["-a"])
            .arg(root().join("vendor/ape"))
            .arg(&tree)
            .output()
            .unwrap();
        assert!(out.status.success(), "stage copy: {out:?}");
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            tree.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let result = execute(
            Command::new("swipl")
                .args(["-O", "-f", "none", "-F", "none", "-g"])
                .arg(format!(
                    "working_directory(_, {}), [fit_to_plp], halt.",
                    atom(tree.join("prolog/parser").to_str().unwrap())
                ))
                .args(["-t", "halt"]),
            "",
        );
        assert_eq!(result.rc, 0, "stage parser: {result:?}");
        Self { scratch, tree }
    }
    fn compile(&self, name: &str, table: &str, question: bool) -> Out {
        let dir = self.scratch.dir.join(name);
        fs::create_dir_all(&dir).unwrap();
        let tab = dir.join("vocabulary.tsv");
        let ulex = dir.join("lexicon.ulex");
        fs::write(&tab, table).unwrap();
        fs::write(&ulex, fixture("lexicon.ulex.in")).unwrap();
        let mut cmd = Command::new("swipl");
        cmd.args(["-q", "-f", "none", "-F", "none", "-s"])
            .arg(self.tree.join("prolog/ace_to_pl.pl"))
            .args(["-g", "main", "-t", "halt(9)", "--"]);
        if question {
            cmd.arg("question");
        }
        cmd.arg(&self.tree).arg(name).arg(&tab).arg(&ulex);
        execute(&mut cmd, &fixture(&format!("{name}/ace.txt.in")))
    }
}
fn insertion(
    id: &str,
    sentence: usize,
    event: usize,
    preferred: usize,
    alternative: Option<usize>,
    ctx: &str,
) -> (String, String) {
    let e = gid(id, sentence, event);
    let p = gid(id, sentence, preferred);
    let a = alternative.map_or_else(|| "none".into(), |n| gid(id, sentence, n));
    let after = if alternative.is_some() {
        format!("guideline_pp({ctx},{e},'instead-of',{a})")
    } else {
        format!("guideline_arg({ctx},{e},2,{})", gid(id, sentence, 2))
    };
    (after, format!("guideline_preference({ctx},{e},{p},{a})"))
}
fn insertions(id: &str) -> Vec<(String, String)> {
    match id {
        "bare" => vec![insertion(id, 1, 3, 1, None, "actual")],
        "combined" => vec![insertion(id, 1, 4, 1, Some(2), "actual")],
        "certdoc" => {
            let r = gid(id, 1, 1);
            let mut out: Vec<_> = [(3, 2, "strength", "strong"), (5, 4, "evidence", "low")]
                .into_iter()
                .map(|(e, l, dim, value)| {
                    let l = gid(id, 1, l);
                    (
                        format!("guideline_arg(actual,{},2,{l})", gid(id, 1, e)),
                        format!("guideline_grade(actual,{r},{dim},{value},{l})"),
                    )
                })
                .collect();
            out.extend([
                insertion(id, 2, 3, 1, None, "actual"),
                insertion(id, 3, 4, 1, Some(2), "actual"),
                insertion(id, 4, 4, 2, Some(3), "actual"),
                insertion(id, 5, 4, 1, Some(2), &context(id, 5, 2)),
            ]);
            out
        }
        _ => Vec::new(),
    }
}
fn rebind_table(pl: &str, from: Option<&str>, to: Option<&str>) -> String {
    let old = format!("vocabulary({})", digest(from));
    assert_eq!(pl.matches(&old).count(), 1);
    pl.replacen(&old, &format!("vocabulary({})", digest(to)), 1)
}
fn expected_pl(id: &str) -> String {
    let base = fixture(&format!("{id}/c1.pl.in"));
    let mut out = rebind_table(
        &base,
        Some(&fixture("c1-vocabulary.tsv.in")),
        Some(&fixture("vocabulary.tsv.in")),
    );
    if !id.ends_with("query") {
        let anchor = ":- discontiguous(guideline_range/3).\n";
        let extra: String = INDICATORS[15..]
            .iter()
            .map(|(n, a)| format!(":- multifile({n}/{a}).\n:- discontiguous({n}/{a}).\n"))
            .collect();
        assert_eq!(out.matches(anchor).count(), 1);
        out = out.replacen(anchor, &(anchor.to_owned() + &extra), 1);
        for (after, annotation) in insertions(id) {
            let anchor = format!("{after}.\n");
            assert_eq!(out.matches(&anchor).count(), 1);
            out = out.replacen(&anchor, &format!("{anchor}{annotation}.\n"), 1);
        }
        assert_eq!(
            fact_proof(id, &base),
            fixture(&format!("{id}/c1-proof.txt.in")),
            "C1 proof oracle"
        );
    }
    out
}
#[test]
fn a08_comparator_free_ace_question_matches_bare_and_combined() {
    let mut c = Cases::new("A8");
    let staged = OnceLock::new();
    let compiled = std::thread::scope(|scope| {
        let jobs: Vec<_> = [
            "bare",
            "combined",
            "certdoc",
            "pref-query",
            "alternative-query",
            "grade-query",
        ]
        .into_iter()
        .map(|id| {
            let staged = &staged;
            scope.spawn(move || {
                (
                    id,
                    staged.get_or_init(Compiler::new).compile(
                        id,
                        &fixture("vocabulary.tsv.in"),
                        id.ends_with("query"),
                    ),
                )
            })
        })
        .collect();
        jobs.into_iter()
            .map(|job| job.join().unwrap())
            .collect::<Vec<_>>()
    });
    for (id, got) in compiled {
        c.compare(&format!("compiler/{id}"), got, Out::ok(expected_pl(id)));
    }
    let mut manifest = String::new();
    let mut results = Vec::new();
    let mut proofs = Vec::new();
    for id in ["bare", "combined"] {
        let doc = expected_pl(id);
        c.write(&format!("{id}.pl"), &doc);
        c.write(&format!("{id}.proof"), &fact_proof(id, &doc));
        manifest.push_str(&format!("{id}.pl\t{id}.proof\n"));
        let lines: Vec<_> = doc
            .lines()
            .filter(|line| {
                line.starts_with("guideline_")
                    && !line.starts_with("guideline_document")
                    && !line.starts_with("guideline_schema_version")
            })
            .map(|line| line.strip_suffix('.').unwrap())
            .collect();
        let indices: &[usize] = if id == "bare" {
            &[0, 1, 2, 3, 4, 5]
        } else {
            &[0, 1, 4, 5, 6, 7]
        };
        let ns: Vec<_> = indices
            .iter()
            .map(|&n| node(id, 1, lines[n], &[]))
            .collect();
        let r = gid(id, 1, 1);
        results.push(format!("sol([{r}])"));
        proofs.push(format!("sol([{r}],proved([{}]))", ns.join(",")));
    }
    c.write("m.tsv", &manifest);
    let q = expected_pl("pref-query");
    chain(
        &mut c,
        "primitive ACE query",
        "pref-query",
        &q,
        &format!("solutions([{}])", results.join(",")),
        &format!("solutions([{}])", proofs.join(",")),
        12,
    );
    c.finish();
}
#[test]
fn a09_annotations_infer_no_inverse_transitivity_or_duty() {
    let mut c = Cases::new("A9");
    let a = gid("options", 1, 1);
    let b = gid("options", 1, 2);
    let d = gid("options", 1, 3);
    let clauses = vec![
        format!(
            "guideline_preference(actual,{},{a},{b})",
            gid("options", 1, 4)
        ),
        format!(
            "guideline_preference(actual,{},{b},{d})",
            gid("options", 1, 5)
        ),
    ];
    install(&c, &[Member::facts("options", clauses.clone())]);
    for clause in &clauses {
        let q = query("asserted", clause, "[]");
        chain(
            &mut c,
            "asserted preference survives",
            "asserted",
            &q,
            "yes",
            &format!("yes(proved([{}]))", node("options", 1, clause, &[])),
            1,
        );
    }
    for goal in [
        format!("guideline_preference(actual,A,{b},{a})"),
        format!("guideline_preference(actual,A,{a},{d})"),
        "guideline_operator(actual,A,must)".into(),
        "guideline_operator(actual,A,should)".into(),
        "guideline_operator(actual,A,-)".into(),
        "guideline_event(actual,A,avoid)".into(),
    ] {
        let q = query("no-inference", &goal, "[]");
        chain(
            &mut c,
            &goal,
            "no-inference",
            &q,
            "no(finite_failure)",
            "no(finite_failure)",
            0,
        );
    }
    c.finish();
}
#[test]
fn a10_aggregate_replay_and_recursion_cover_typed_clauses() {
    let mut c = Cases::new("A10");
    let mut members = graded_members();
    members.extend(patient_members(true, false));
    for reverse in [false, true] {
        if reverse {
            members.reverse();
        }
        install(&c, &members);
        c.expect(
            "aggregate",
            &["v1", "aggregate-check", "m.tsv"],
            Out::ok("ace_to_pl aggregate ok 5 documents 5 obligations\n"),
        );
        c.expect(
            "recursion",
            &["v1", "recursion-check", "m.tsv"],
            Out::ok("ace_to_pl recursion ok 5 documents 1 rule clauses\n"),
        );
    }
    for &(name, arity) in &INDICATORS[15..] {
        let mut args = vec!["x".into(); arity];
        args[1] = gid("recursive", 1, 1);
        let head = format!("{name}({})", args.join(","));
        install(
            &c,
            &[Member {
                id: "recursive".into(),
                clauses: vec![format!("{head} :- {head}")],
                witness: vec![],
                heads: vec![],
            }],
        );
        c.expect(
            name,
            &["v1", "recursion-check", "m.tsv"],
            Out::err(
                1,
                format!(
                    "ace_to_pl_error(proof,left_recursive(sentence(recursive,1),{name},{arity})).\n"
                ),
            ),
        );
    }
    c.finish();
}

struct Cert {
    id: &'static str,
    pl: String,
    table: Option<String>,
}
impl Cert {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            pl: expected_pl(id),
            table: Some(fixture("vocabulary.tsv.in")),
        }
    }
    fn run(&self, c: &mut Cases, label: &str, why: Option<&str>) {
        c.write("ace.txt", &fixture(&format!("{}/ace.txt.in", self.id)));
        c.write("dump.txt", &fixture(&format!("{}/dump.txt.in", self.id)));
        c.write("lexicon.ulex", &fixture("lexicon.ulex.in"));
        c.write("cert.pl", &self.pl);
        let question = self.id.ends_with("query");
        let mut args = vec![
            "certify-one",
            if question { "query" } else { "doc" },
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
                Out::ok(if question {
                    format!("ckc: certify ok {}\n", self.id)
                } else {
                    fact_proof(self.id, &self.pl)
                })
            },
            |why| Out::err(1, format!("ckc: certify: {}: {why}.\n", self.id)),
        );
        c.expect(label, &args, want);
    }
    fn repin(&mut self, table: Option<String>) {
        self.pl = rebind_table(&self.pl, self.table.as_deref(), table.as_deref());
        self.table = table;
    }
}
fn erase_annotations(pl: &str) -> String {
    pl.lines()
        .filter(|l| !l.starts_with("guideline_grade(") && !l.starts_with("guideline_preference("))
        .map(|l| format!("{l}\n"))
        .collect()
}
fn annotation_args(lit: &str) -> (&str, Vec<String>) {
    let (name, tail) = lit.split_once('(').unwrap();
    let mut depth = 0;
    let mut quoted = false;
    let mut escaped = false;
    let mut start = 0;
    let mut args = Vec::new();
    let tail = tail.strip_suffix(')').unwrap();
    for (i, ch) in tail.char_indices() {
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
                args.push(tail[start..i].into());
                start = i + 1;
            }
            _ => {}
        }
    }
    args.push(tail[start..].into());
    (name, args)
}
fn annotation_mutants(row: &'static str, kind: &str, index: usize, replacements: &[&str]) {
    let mut c = Cases::new(row);
    let clean = Cert::new("certdoc");
    for (_, lit) in insertions("certdoc")
        .into_iter()
        .filter(|(_, lit)| lit.starts_with(kind))
    {
        let (name, args) = annotation_args(&lit);
        let sentence = if name == "guideline_grade" {
            1
        } else {
            args[1].split(',').nth(2).unwrap().parse().unwrap()
        };
        for replacement in replacements {
            let mut sample = Cert::new("certdoc");
            let mut changed = args.clone();
            changed[index] = (*replacement).into();
            let bad = format!("{name}({})", changed.join(","));
            if bad == lit {
                continue;
            }
            assert_eq!(sample.pl.matches(&format!("{lit}.\n")).count(), 1);
            sample.pl = sample
                .pl
                .replacen(&format!("{lit}.\n"), &format!("{bad}.\n"), 1);
            assert_eq!(
                erase_annotations(&sample.pl),
                erase_annotations(&clean.pl),
                "{row}: primitives unchanged"
            );
            c.check("mutant is canonical", &sample.pl);
            sample.run(
                &mut c,
                &format!("{name}/arg-{index}/{replacement}"),
                Some(&format!("clauses({sentence})")),
            );
        }
    }
    c.finish();
}
#[test]
fn a11_clean_s1_documents_and_questions_certify() {
    let mut c = Cases::new("A11");
    for id in [
        "certdoc",
        "bare",
        "combined",
        "pref-query",
        "alternative-query",
        "grade-query",
    ] {
        let sample = Cert::new(id);
        c.check(id, &sample.pl);
        sample.run(&mut c, id, None);
    }
    for strength in ["strong", "conditional"] {
        for evidence in ["high", "moderate", "low", "very_low"] {
            let mut sample = Cert::new("certdoc");
            let table = sample
                .table
                .as_ref()
                .unwrap()
                .replace("\tstrong\n", &format!("\t{strength}\n"))
                .replace("\tlow\n", &format!("\t{evidence}\n"));
            sample.repin(Some(table));
            sample.pl = sample
                .pl
                .replace(",strength,strong,", &format!(",strength,{strength},"))
                .replace(",evidence,low,", &format!(",evidence,{evidence},"));
            sample.run(&mut c, &format!("value-domain/{strength}/{evidence}"), None);
        }
    }
    c.finish();
}
#[test]
fn a12_wrong_grade_context_rejects() {
    annotation_mutants(
        "A12-grade",
        "guideline_grade",
        0,
        &["other", "'$guideline_id'(context,certdoc,1,box(1),[])"],
    );
}
#[test]
fn a12_wrong_preference_context_rejects() {
    annotation_mutants(
        "A12-preference",
        "guideline_preference",
        0,
        &["other", "actual"],
    );
}
#[test]
fn a13_wrong_event_rejects() {
    annotation_mutants(
        "A13",
        "guideline_preference",
        1,
        &["other", "'$guideline_id'(product,certdoc,2,ref(2),[])"],
    );
}
#[test]
fn a14_wrong_recommendation_rejects() {
    annotation_mutants(
        "A14",
        "guideline_grade",
        1,
        &[
            "other",
            "'$guideline_id'(product,other,1,ref(1),[])",
            "'$guideline_id'(product,certdoc,1,ref(2),[])",
        ],
    );
}
#[test]
fn a15_wrong_dimension_rejects() {
    annotation_mutants(
        "A15",
        "guideline_grade",
        2,
        &["strength", "evidence", "other", "[strength]"],
    );
}
#[test]
fn a16_wrong_value_rejects() {
    annotation_mutants(
        "A16",
        "guideline_grade",
        3,
        &["conditional", "high", "moderate", "very_low", "other", "-1"],
    );
}
#[test]
fn a17_wrong_label_rejects() {
    annotation_mutants(
        "A17",
        "guideline_grade",
        4,
        &[
            "other",
            "'$guideline_id'(product,certdoc,1,ref(1),[])",
            "'$guideline_id'(product,other,1,ref(2),[])",
        ],
    );
}
#[test]
fn a18_wrong_preferred_rejects() {
    annotation_mutants(
        "A18",
        "guideline_preference",
        2,
        &[
            "none",
            "'$guideline_id'(product,certdoc,2,ref(2),[])",
            "'$guideline_id'(product,other,1,ref(1),[])",
        ],
    );
}
#[test]
fn a19_wrong_alternative_rejects() {
    annotation_mutants(
        "A19",
        "guideline_preference",
        3,
        &[
            "none",
            "other",
            "'$guideline_id'(product,certdoc,2,ref(1),[])",
        ],
    );
}
fn structural_mutants(row: &'static str, action: &str) {
    let mut c = Cases::new(row);
    let clean = Cert::new("certdoc");
    for (after, lit) in insertions("certdoc") {
        let (_, args) = annotation_args(&lit);
        let sentence: usize = if lit.starts_with("guideline_grade") {
            1
        } else {
            args[1].split(',').nth(2).unwrap().parse().unwrap()
        };
        let mut sample = Cert::new("certdoc");
        let line = format!("{lit}.\n");
        sample.pl = match action {
            "drop" => sample.pl.replacen(&line, "", 1),
            "duplicate" => sample.pl.replacen(&line, &line.repeat(2), 1),
            "misplace" => sample.pl.replacen(
                &format!("{after}.\n{line}"),
                &format!("{line}{after}.\n"),
                1,
            ),
            "add" => {
                let (name, mut args) = annotation_args(&lit);
                args[0] = "other".into();
                let extra = format!("{name}({})", args.join(","));
                sample.pl.replacen(&line, &format!("{line}{extra}.\n"), 1)
            }
            _ => unreachable!(),
        };
        assert_ne!(sample.pl, clean.pl, "mutation changes bytes");
        assert_eq!(
            erase_annotations(&sample.pl),
            erase_annotations(&clean.pl),
            "primitives unchanged"
        );
        c.check(action, &sample.pl);
        sample.run(&mut c, action, Some(&format!("clauses({sentence})")));
    }
    c.finish();
}
#[test]
fn a20_dropped_annotation_rejects() {
    structural_mutants("A20", "drop");
}
#[test]
fn a21_added_annotation_rejects() {
    structural_mutants("A21", "add");
}
#[test]
fn a22_misplaced_annotation_rejects() {
    structural_mutants("A22", "misplace");
}
#[test]
fn a23_duplicated_annotation_rejects() {
    structural_mutants("A23", "duplicate");
}
#[test]
fn a24_table_bytes_and_digest_are_bound() {
    let mut c = Cases::new("A24");
    for id in ["certdoc", "pref-query"] {
        let mut sample = Cert::new(id);
        sample
            .table
            .as_mut()
            .unwrap()
            .push_str("unit\tunused-hour\thour\n");
        sample.run(&mut c, "changed bytes old digest", Some("vocabulary"));
        let mut sample = Cert::new(id);
        sample.pl = rebind_table(&sample.pl, sample.table.as_deref(), Some("wrong digest\n"));
        sample.run(&mut c, "wrong digest", Some("vocabulary"));
        let mut sample = Cert::new(id);
        sample.table = None;
        sample.run(&mut c, "missing table bytes", Some("vocabulary"));
        let mut sample = Cert::new(id);
        sample.pl = rebind_table(&sample.pl, sample.table.as_deref(), None);
        sample.run(&mut c, "none with bytes", Some("vocabulary"));
        for malformed in [
            "# format: kind\tlemma\tvalue\n",
            "# malformed\n",
            "# format: kind\tlemma\tvalue\nstrength\tstrong-grade\tother\n",
        ] {
            let mut sample = Cert::new(id);
            sample.repin(Some(malformed.into()));
            sample.run(&mut c, "bad table matching digest", Some("vocabulary"));
        }
    }
    let mut sample = Cert::new("certdoc");
    let changed = sample
        .table
        .as_ref()
        .unwrap()
        .replace("\tstrong\n", "\tconditional\n");
    sample.repin(Some(changed));
    sample.run(&mut c, "valid remapping old grade", Some("clauses(1)"));
    c.finish();
}
#[test]
fn a25_annotation_without_table_fails_certification_not_codec() {
    let mut c = Cases::new("A25");
    for id in ["certdoc", "bare", "combined"] {
        let mut sample = Cert::new(id);
        sample.repin(None);
        c.check("annotation with vocabulary(none)", &sample.pl);
        sample.run(&mut c, id, Some("clauses(1)"));
        sample.pl = erase_annotations(&sample.pl);
        sample.run(&mut c, "no-table primitive projection", None);
    }
    c.finish();
}
#[test]
fn a26_committed_questions_cannot_gain_typed_goals() {
    let mut c = Cases::new("A26");
    for id in ["pref-query", "alternative-query", "grade-query"] {
        for (name, arity) in INDICATORS[15..].iter().copied() {
            let mut sample = Cert::new(id);
            let literal = compound(name, &vec!["x"; arity]);
            let anchor = "'$guideline_query_projection'(goal(";
            sample.pl = sample
                .pl
                .replacen(anchor, &format!("{anchor}','({literal},"), 1)
                .replacen("),answers(", ")),answers(", 1);
            c.check("raw query goal remains canonical", &sample.pl);
            sample.run(&mut c, name, Some("projection"));
        }
    }
    c.finish();
}
