// root-exact P1/P2: source-DRS oracle, seed-frozen scopes, projection boundaries.
// CKC_ROOT_EXACT_TEST_BIN selects a prebuilt certifier; the fork always comes from this tree.
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

const TABLE: &str = include_str!("../../../tests/root-exact/vocabulary.tsv.in");
const ULEX: &str = include_str!("../../../tests/root-exact/lexicon.ulex.in");
const INDICATORS: &[(&str, usize)] = &[
    ("schema_version", 1),
    ("document", 4),
    ("entity", 4),
    ("cardinality", 5),
    ("event", 3),
    ("arg", 4),
    ("pp", 4),
    ("property", 4),
    ("operator", 3),
    ("interval", 6),
    ("recurrence", 4),
    ("frequency", 5),
    ("order", 4),
    ("recurrence_window", 7),
    ("range", 3),
    ("grade", 5),
    ("preference", 4),
    ("qualifier", 4),
    ("unit", 3),
    ("exclusion", 1),
];
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn fixture(name: &str) -> String {
    fs::read_to_string(root().join("tests/root-exact").join(name)).unwrap()
}
fn sha(s: &str) -> String {
    Sha256::digest(s.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn quote(s: &str) -> String {
    let digits = s.strip_prefix('-').unwrap_or(s);
    if s == "-"
        || (!digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
        || (s.bytes().next().is_some_and(|b| b.is_ascii_lowercase())
            && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'))
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}
fn call(name: &str, args: &[String]) -> String {
    format!("{}({})", quote(name), args.join(","))
}
fn fact(name: &str, context: &str, args: &[String]) -> String {
    call(
        &format!("guideline_{name}"),
        &std::iter::once(context.to_string())
            .chain(args.iter().cloned())
            .collect::<Vec<_>>(),
    )
}
fn id(kind: &str, doc: &str, sentence: usize, key: &str, ordinal: usize) -> String {
    call(
        "$guideline_id",
        &[
            kind.into(),
            quote(doc),
            sentence.to_string(),
            format!("{key}({ordinal})"),
            "[]".into(),
        ],
    )
}
#[derive(Debug)]
struct Run {
    rc: Option<i32>,
    out: String,
    err: String,
}
impl Run {
    fn ok(&self) -> bool {
        self.rc == Some(0) && self.err.is_empty()
    }
    fn require_ok(&self, label: &str) {
        assert!(
            self.ok(),
            "{label}: rc={:?}, stderr={:?}",
            self.rc,
            self.err
        );
    }
}
fn run(bin: &Path, cwd: &Path, args: &[&str], input: &str) -> Run {
    let mut child = Command::new("timeout")
        .args(["--kill-after=2", "90"])
        .arg(bin)
        .args(args)
        .current_dir(cwd)
        .env("CKC_JOBS", "1")
        .env("LC_ALL", "C.UTF-8")
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
    Run {
        rc: output.status.code(),
        out: String::from_utf8(output.stdout).unwrap(),
        err: String::from_utf8(output.stderr).unwrap(),
    }
}
struct Scratch(PathBuf);
impl Scratch {
    fn new(label: &str) -> Self {
        let path = root().join(format!(
            "rust/target/root-exact/{label}-{}",
            std::process::id()
        ));
        assert!(!path.exists(), "scratch collision: {}", path.display());
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn file(&self, name: &str, text: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, text).unwrap();
        path
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        assert!(!entry.file_type().unwrap().is_symlink());
        let dest = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &dest);
        } else {
            fs::copy(entry.path(), dest).unwrap();
        }
    }
}
struct Compiler {
    stage: PathBuf,
    table: PathBuf,
    ulex: PathBuf,
}
impl Compiler {
    fn new(s: &Scratch) -> Self {
        let version = run(Path::new("swipl"), &s.0, &["--version"], "");
        version.require_ok("SWI pin");
        assert!(version.out.contains("version 10.0.2 "));
        let stage = s.0.join("ape");
        copy_tree(&root().join("vendor/ape"), &stage);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let goals = [
            format!(
                "working_directory(_,{}),[fit_to_plp],halt.",
                quote(stage.join("prolog/parser").to_str().unwrap())
            ),
            format!(
                "style_check(-discontiguous),qcompile({}),halt.",
                quote(
                    stage
                        .join("prolog/lexicon/clex_lexicon.pl")
                        .to_str()
                        .unwrap()
                )
            ),
        ];
        for goal in goals {
            let built = run(
                Path::new("swipl"),
                &s.0,
                &[
                    "-q", "-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
                ],
                "",
            );
            assert_eq!(built.rc, Some(0), "APE stage: {built:?}");
        }
        Self {
            stage,
            table: s.file("vocabulary.tsv", TABLE),
            ulex: s.file("lexicon.ulex", ULEX),
        }
    }
    fn emit(&self, p: &Probe, mode: &str) -> Run {
        let source = self.stage.join("prolog/ace_to_pl.pl");
        let mut args = vec![
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            source.to_str().unwrap(),
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
            &p.id,
            if p.table {
                self.table.to_str().unwrap()
            } else {
                "-"
            },
            self.ulex.to_str().unwrap(),
        ]);
        if mode == "proof" {
            args.push("proof");
        }
        run(
            Path::new("swipl"),
            self.stage.parent().unwrap(),
            &args,
            &p.ace,
        )
    }
    fn dump(&self, p: &Probe) -> Run {
        let source = root().join("rust/ckc/prolog/drs_dump.pl");
        run(
            Path::new("swipl"),
            self.stage.parent().unwrap(),
            &[
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-s",
                source.to_str().unwrap(),
                "-g",
                "main",
                "-t",
                "halt(9)",
                "--",
                self.stage.to_str().unwrap(),
                self.ulex.to_str().unwrap(),
            ],
            &p.ace,
        )
    }
}

// Canonical upstream DRS reader. It supplies source order and identity, not emitted clauses.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Term {
    Atom(String),
    Var(String),
    Compound(String, Vec<Term>),
    List(Vec<Term>),
}
impl Term {
    fn name(&self) -> &str {
        match self {
            Self::Atom(x) | Self::Var(x) | Self::Compound(x, _) => x,
            Self::List(_) => "[]",
        }
    }
    fn args(&self) -> &[Term] {
        match self {
            Self::Compound(_, xs) | Self::List(xs) => xs,
            _ => &[],
        }
    }
    fn variable(&self) -> bool {
        matches!(self, Self::Var(_))
    }
}
fn split(s: &str) -> Vec<&str> {
    let (mut depth, mut quoted, mut escaped, mut start) = (0isize, false, false, 0);
    let mut out = Vec::new();
    for (i, c) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && c == '\\' {
            escaped = true;
            continue;
        }
        if c == '\'' {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                out.push(s[start..i].trim());
                start = i + 1;
            }
            _ => (),
        }
    }
    if !s.is_empty() {
        out.push(s[start..].trim());
    }
    out
}
fn term(s: &str) -> Term {
    let s = s.trim();
    if s.starts_with('[') {
        return Term::List(split(&s[1..s.len() - 1]).into_iter().map(term).collect());
    }
    let (mut quoted, mut escaped) = (false, false);
    for (i, c) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && c == '\\' {
            escaped = true;
            continue;
        }
        if c == '\'' {
            quoted = !quoted;
        } else if c == '(' && !quoted {
            return Term::Compound(
                term(&s[..i]).name().into(),
                split(&s[i + 1..s.len() - 1])
                    .into_iter()
                    .map(term)
                    .collect(),
            );
        }
    }
    if s.starts_with('\'') {
        Term::Atom(s[1..s.len() - 1].replace("\\'", "'").replace("\\\\", "\\"))
    } else if s
        .bytes()
        .next()
        .is_some_and(|b| b.is_ascii_uppercase() || b == b'_')
    {
        Term::Var(s.into())
    } else {
        Term::Atom(s.into())
    }
}
fn sentence(t: &Term) -> usize {
    fn anchors(t: &Term, out: &mut Vec<usize>) {
        if t.name() == "-" && t.args().len() == 2 && t.args()[1].name() == "/" {
            out.push(t.args()[1].args()[0].name().parse().unwrap());
        } else {
            for x in t.args() {
                anchors(x, out);
            }
        }
    }
    let mut anchors_found = Vec::new();
    anchors(t, &mut anchors_found);
    let first = *anchors_found.first().expect("oracle item needs an anchor");
    assert!(
        anchors_found.iter().all(|s| *s == first),
        "oracle item spans sentences"
    );
    first
}
#[derive(Clone)]
struct Item {
    s: usize,
    context: String,
    leaf: Term,
}
fn source_items(dump: &str, doc: &str) -> Vec<Item> {
    fn visit(
        t: &Term,
        context: &str,
        doc: &str,
        boxes: &mut BTreeMap<usize, usize>,
        out: &mut Vec<Item>,
    ) {
        match t {
            Term::List(xs) => {
                for x in xs {
                    visit(x, context, doc, boxes, out);
                }
            }
            Term::Compound(n, xs) if n == "drs" => {
                visit(xs.last().unwrap(), context, doc, boxes, out)
            }
            Term::Compound(n, xs) if n == "-" && xs.len() == 2 => out.push(Item {
                s: sentence(t),
                context: context.into(),
                leaf: xs[0].clone(),
            }),
            Term::Compound(n, xs)
                if ["can", "must", "should", "-"].contains(&n.as_str()) && xs.len() == 1 =>
            {
                let s = sentence(t);
                let ordinal = boxes.entry(s).or_default();
                *ordinal += 1;
                let inner = id("context", doc, s, "box", *ordinal);
                out.push(Item {
                    s,
                    context: context.into(),
                    leaf: Term::Compound(
                        "edge".into(),
                        vec![Term::Atom(inner.clone()), Term::Atom(n.clone())],
                    ),
                });
                visit(&xs[0], &inner, doc, boxes, out);
            }
            _ => panic!("unmodeled source construct {t:?}"),
        }
    }
    assert!(
        dump.lines().any(|l| l == "messages([])."),
        "source diagnostics: {dump}"
    );
    let drs = term(
        dump.lines()
            .find(|l| l.starts_with("drs("))
            .unwrap()
            .trim_end_matches('.'),
    );
    let mut out = Vec::new();
    visit(&drs, "actual", doc, &mut BTreeMap::new(), &mut out);
    out
}
#[derive(Clone)]
struct Probe {
    name: String,
    id: String,
    ace: String,
    dump: String,
    table: bool,
}
impl Probe {
    fn fixture(name: &str, table: bool) -> Self {
        Self {
            name: name.into(),
            id: format!("root-exact-{name}"),
            ace: fixture(&format!("{name}.ace")),
            dump: fixture(&format!("{name}.drs.in")),
            table,
        }
    }
    fn expected(&self) -> (String, String) {
        let items = source_items(&self.dump, &self.id);
        let mut refs = BTreeMap::<String, String>::new();
        let mut next = BTreeMap::<usize, usize>::new();
        for item in &items {
            let xs = item.leaf.args();
            let variables: Vec<_> = match item.leaf.name() {
                "object" | "property" => vec![&xs[0]],
                "relation" | "edge" => Vec::new(),
                _ => xs.iter().filter(|t| t.variable()).collect(),
            };
            for v in variables {
                if let std::collections::btree_map::Entry::Vacant(entry) =
                    refs.entry(v.name().into())
                {
                    let n = next.entry(item.s).or_default();
                    *n += 1;
                    entry.insert(id("product", &self.id, item.s, "ref", *n));
                }
            }
        }
        let reference = |t: &Term| {
            refs.get(t.name())
                .cloned()
                .unwrap_or_else(|| quote(t.name()))
        };
        let noun = |r: &Term| {
            items
                .iter()
                .find(|x| x.leaf.name() == "object" && x.leaf.args()[0] == *r)
                .map(|x| x.leaf.args()[1].name())
        };
        let unit = |r: &Term| noun(r).filter(|n| self.table && ["hour", "day"].contains(n));
        let mut clauses = BTreeMap::<usize, Vec<String>>::new();
        for item in &items {
            let xs = item.leaf.args();
            let c = &item.context;
            let out = clauses.entry(item.s).or_default();
            match item.leaf.name() {
                "object" => {
                    out.push(fact(
                        "entity",
                        c,
                        &[reference(&xs[0]), quote(xs[1].name()), quote(xs[2].name())],
                    ));
                    out.push(fact(
                        "cardinality",
                        c,
                        &[
                            reference(&xs[0]),
                            quote(xs[3].name()),
                            quote(xs[4].name()),
                            quote(xs[5].name()),
                        ],
                    ));
                    if let Some(u) = unit(&xs[0]) {
                        out.push(fact("unit", c, &[reference(&xs[0]), quote(u)]));
                    }
                }
                "predicate" => {
                    out.push(fact("event", c, &[reference(&xs[0]), quote(xs[1].name())]));
                    for (index, r) in xs[2..].iter().enumerate() {
                        out.push(fact(
                            "arg",
                            c,
                            &[reference(&xs[0]), (index + 1).to_string(), reference(r)],
                        ));
                    }
                }
                "modifier_pp" => {
                    out.push(fact(
                        "pp",
                        c,
                        &[reference(&xs[0]), quote(xs[1].name()), reference(&xs[2])],
                    ));
                    if self.table && xs[1].name() == "at" && noun(&xs[2]) == Some("interval") {
                        let link = items
                            .iter()
                            .find(|x| {
                                x.context == *c
                                    && x.leaf.name() == "relation"
                                    && x.leaf.args()[0] == xs[2]
                                    && x.leaf.args()[1].name() == "of"
                            })
                            .unwrap();
                        let q = &link.leaf.args()[2];
                        out.push(fact(
                            "recurrence",
                            c,
                            &[reference(&xs[0]), reference(q), quote(unit(q).unwrap())],
                        ));
                    } else if self.table
                        && xs[1].name() == "for"
                        && let Some(u) = unit(&xs[2])
                    {
                        out.push(fact(
                            "interval",
                            c,
                            &[
                                reference(&xs[0]),
                                "duration".into(),
                                reference(&xs[2]),
                                quote(u),
                                "none".into(),
                            ],
                        ));
                    }
                }
                "relation" => {
                    assert!(
                        self.table
                            && xs[1].name() == "of"
                            && noun(&xs[0]) == Some("interval")
                            && unit(&xs[2]).is_some(),
                        "unconsumed relation in accepted oracle"
                    );
                }
                "property" => out.push(fact(
                    "property",
                    c,
                    &[reference(&xs[0]), quote(xs[1].name()), quote(xs[2].name())],
                )),
                "modifier_adv" => {
                    assert!(self.table && xs[1].name() == "usually" && c != "actual");
                    out.push(fact(
                        "qualifier",
                        c,
                        &[reference(&xs[0]), "usuality".into(), "usually".into()],
                    ));
                }
                "edge" => out.push(fact(
                    "operator",
                    c,
                    &[xs[0].name().into(), quote(xs[1].name())],
                )),
                other => panic!("unmodeled primitive {other}"),
            }
        }
        let table = if self.table {
            format!("sha256({})", quote(&sha(TABLE)))
        } else {
            "none".into()
        };
        let mut doc = format!(
            "% {}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n",
            self.id
        );
        for (name, arity) in INDICATORS {
            doc += &format!(
                ":- multifile(guideline_{name}/{arity}).\n:- discontiguous(guideline_{name}/{arity}).\n"
            );
        }
        doc += &format!(
            "guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex(sha256({})),vocabulary({table})).\n",
            quote(&self.id),
            quote(&sha(&self.ace)),
            quote(&sha(ULEX))
        );
        let mut proof = String::new();
        let sentences: Vec<_> = self.ace.lines().collect();
        for (s, heads) in clauses {
            doc += &format!("% S{s}: {}\n", sentences[s - 1]);
            for h in &heads {
                doc += &format!("{h}.\n");
            }
            proof += &format!(
                "{}.\n",
                call(
                    "$guideline_proof",
                    &[
                        quote(&self.id),
                        s.to_string(),
                        "variant(1)".into(),
                        "witness([])".into(),
                        format!("prove([{}])", heads.join(","))
                    ]
                )
            );
        }
        (doc, proof)
    }
}
fn accepted() -> Vec<Probe> {
    let mut cases = Vec::new();
    for name in [
        "one",
        "two",
        "two-groups",
        "nested",
        "cross",
        "modal",
        "modal-zero",
        "modal-negation",
    ] {
        for table in [false, true] {
            cases.push(Probe::fixture(name, table));
        }
    }
    for name in ["dose", "scheduled", "combination", "modal-usually"] {
        cases.push(Probe::fixture(name, true));
    }
    cases
}
fn generated() -> Vec<Probe> {
    let mut cases = Vec::new();
    for (i, count) in [
        "3".into(),
        "17".into(),
        "2147483648".into(),
        "18446744073709551616".into(),
        "9".repeat(100),
    ]
    .iter()
    .enumerate()
    {
        for table in [false, true] {
            let mut p = Probe::fixture("two", table);
            p.name = format!("count-{i}");
            p.id = format!("root-exact-count-{i}");
            p.ace = p.ace.replace("exactly 2", &format!("exactly {count}"));
            p.dump = p.dump.replace("exactly,2", &format!("exactly,{count}"));
            cases.push(p);
        }
    }
    cases
}
struct Emission {
    p: Probe,
    doc: Run,
    proof: Run,
    dump: Run,
}
struct Results {
    accepted: Vec<Emission>,
    controls: Vec<Emission>,
    rejects: BTreeMap<String, Run>,
    stable: BTreeMap<String, Run>,
}
fn results() -> &'static Results {
    static RESULTS: OnceLock<Results> = OnceLock::new();
    RESULTS.get_or_init(|| {
        let scratch = Scratch::new("compiler");
        let compiler = Compiler::new(&scratch);
        let collect = |p: Probe| Emission {
            doc: compiler.emit(&p, "doc"),
            proof: compiler.emit(&p, "proof"),
            dump: compiler.dump(&p),
            p,
        };
        let accepted = accepted()
            .into_iter()
            .chain(generated())
            .map(collect)
            .collect();
        let controls = [false, true]
            .into_iter()
            .map(|table| collect(Probe::fixture("control", table)))
            .chain(std::iter::once(collect(Probe::fixture(
                "stable-modal",
                true,
            ))))
            .collect();
        let mut rejects = BTreeMap::new();
        for name in [
            "zero",
            "at-most",
            "less-than",
            "mixed-separate",
            "mixed-nested",
            "disjunction",
            "usually",
            "approximate",
            "relation",
            "naf",
            "scoped-rule",
        ] {
            for table in [false, true] {
                let p = Probe::fixture(name, table);
                rejects.insert(format!("{name}-{table}"), compiler.emit(&p, "doc"));
            }
        }
        let mut stable = BTreeMap::new();
        for name in ["stable-modal", "stable-rule", "stable-question"] {
            for table in [false, true] {
                let p = Probe::fixture(name, table);
                let modes = if name == "stable-question" {
                    &["question"][..]
                } else {
                    &["doc", "proof"][..]
                };
                for mode in modes {
                    stable.insert(
                        format!("{name}-{}-{mode}", usize::from(table)),
                        compiler.emit(&p, mode),
                    );
                }
            }
        }
        Results {
            accepted,
            controls,
            rejects,
            stable,
        }
    })
}
#[derive(Default)]
struct Checks {
    count: usize,
    failures: Vec<String>,
}
impl Checks {
    fn check(&mut self, good: bool, why: impl Into<String>) {
        self.count += 1;
        if !good {
            self.failures.push(why.into());
        }
    }
    fn emission(&mut self, got: &Run, expected: &str, label: &str) {
        self.check(
            got.ok() && got.out == expected,
            format!(
                "{label}: rc={:?}, stderr={:?}; {}",
                got.rc,
                got.err,
                difference(expected, &got.out)
            ),
        );
    }
    fn finish(self, label: &str) {
        println!(
            "{label}: {} checks; {} failures",
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
fn difference(expected: &str, got: &str) -> String {
    let e: Vec<_> = expected.lines().collect();
    let g: Vec<_> = got.lines().collect();
    let i = e
        .iter()
        .zip(&g)
        .position(|(a, b)| a != b)
        .unwrap_or(e.len().min(g.len()));
    format!(
        "line {} expected {:?}, got {:?}; bytes {} vs {}",
        i + 1,
        e.get(i),
        g.get(i),
        expected.len(),
        got.len()
    )
}
#[test]
fn t1_full_products_and_empty_witness_proofs() {
    let mut checks = Checks::default();
    for e in &results().accepted {
        let label = format!("{} table={}", e.p.name, e.p.table);
        checks.emission(&e.dump, &e.p.dump, &format!("{label} upstream DRS"));
        let (doc, proof) = e.p.expected();
        checks.emission(&e.doc, &doc, &format!("{label} document"));
        checks.emission(&e.proof, &proof, &format!("{label} proof"));
    }
    checks.finish("T1 exact fact projection");
}
#[test]
fn t1_oracle_plain_and_modal_positive_controls() {
    let mut checks = Checks::default();
    for e in &results().controls {
        let (doc, proof) = e.p.expected();
        checks.emission(
            &e.doc,
            &doc,
            &format!("{} table={} document", e.p.name, e.p.table),
        );
        checks.emission(
            &e.proof,
            &proof,
            &format!("{} table={} proof", e.p.name, e.p.table),
        );
    }
    checks.finish("T1 independent oracle controls");
}
#[test]
fn t2_zero_and_root_upper_mixes_reject() {
    let mut checks = Checks::default();
    for name in [
        "zero",
        "at-most",
        "less-than",
        "mixed-separate",
        "mixed-nested",
    ] {
        for table in [false, true] {
            let r = &results().rejects[&format!("{name}-{table}")];
            checks.check(
                r.rc == Some(1)
                    && r.out.is_empty()
                    && r.err
                        .starts_with("ace_to_pl_error(unsupported,root_condition(")
                    && r.err.ends_with(")).\n")
                    && r.err.lines().count() == 1,
                format!("{name} table={table}: {r:?}"),
            );
        }
    }
    checks.finish("T2 root admission rejects");
}
#[test]
fn t3_admission_preserves_downstream_rejections() {
    let mut checks = Checks::default();
    for (name, why) in [
        ("disjunction", ""),
        ("usually", "qualifier_shape(asserted,1)"),
        ("approximate", "temporal_shape(approximate_bound,1)"),
        ("relation", ""),
        ("naf", ""),
        ("scoped-rule", ""),
    ] {
        let r = &results().rejects[&format!("{name}-true")];
        checks.check(
            r.rc == Some(1)
                && r.out.is_empty()
                && r.err.starts_with("ace_to_pl_error(unsupported,")
                && !r.err.contains("root_condition")
                && r.err.contains(why),
            format!("{name}: downstream {why}, got {r:?}"),
        );
    }
    checks.finish("T3 downstream refusals");
}
#[test]
fn t4_modal_consequent_and_question_bytes_stay_fixed() {
    let mut checks = Checks::default();
    for (name, r) in &results().stable {
        let rc: i32 = fixture(&format!("{name}.rc.in")).trim().parse().unwrap();
        checks.check(
            r.rc == Some(rc)
                && r.out == fixture(&format!("{name}.stdout.in"))
                && r.err == fixture(&format!("{name}.stderr.in")),
            format!(
                "{name}: rc={:?}, stderr={:?}; {}",
                r.rc,
                r.err,
                difference(&fixture(&format!("{name}.stdout.in")), &r.out)
            ),
        );
    }
    checks.finish("T4 seed-frozen scopes");
}
fn certify(s: &Scratch, p: &Probe, product: &str) -> Run {
    let ace = s.file("ace.in", &p.ace);
    let dump = s.file("dump.in", &p.dump);
    let pl = s.file("product.in", product);
    let ulex = s.file("lexicon.ulex", ULEX);
    let table = s.file("vocabulary.tsv", TABLE);
    let binary = std::env::var_os("CKC_ROOT_EXACT_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    let mut args = vec![
        "certify-one",
        "doc",
        &p.id,
        ace.to_str().unwrap(),
        ulex.to_str().unwrap(),
        dump.to_str().unwrap(),
        pl.to_str().unwrap(),
    ];
    if p.table {
        args.push(table.to_str().unwrap());
    }
    run(&binary, &s.0, &args, "")
}
#[test]
fn t5_independent_exact_products_certify() {
    let s = Scratch::new("certify");
    let mut checks = Checks::default();
    for p in accepted().into_iter().chain(generated()) {
        let r = certify(&s, &p, &p.expected().0);
        checks.check(
            r.ok() && !r.out.is_empty(),
            format!("{} table={}: {r:?}", p.name, p.table),
        );
    }
    checks.finish("T5 accepted independent products");
}
#[test]
fn t5_certification_mutants_are_sentence_clause_failures() {
    let s = Scratch::new("mutants");
    let mut checks = Checks::default();
    for name in ["dose", "scheduled", "combination"] {
        let p = Probe::fixture(name, true);
        let (doc, _) = p.expected();
        let exact = doc
            .lines()
            .find(|l| l.starts_with("guideline_cardinality(") && l.contains(",exactly,"))
            .unwrap();
        let comparator = exact.replacen(",exactly,", ",eq,", 1);
        let count = exact.rsplit_once(',').unwrap().0.to_string() + ",123456).";
        for (kind, replacement) in [("comparison", comparator), ("count", count)] {
            let altered = doc.replacen(exact, &replacement, 1);
            assert_ne!(altered, doc);
            assert_eq!(
                doc.lines().find(|l| l.starts_with("guideline_document(")),
                altered
                    .lines()
                    .find(|l| l.starts_with("guideline_document("))
            );
            let r = certify(&s, &p, &altered);
            checks.check(
                r.rc == Some(1)
                    && r.out.is_empty()
                    && r.err == format!("ckc: certify: {}: clauses(1).\n", p.id),
                format!("{name} {kind}: {r:?}"),
            );
        }
    }
    checks.finish("T5 clause-only mutants");
}

fn synthetic(group: &str) -> Probe {
    let mut p = Probe::fixture("two", false);
    let old = "[-(predicate(C,see,A,B),/(1,3)),-(object(B,dog,countable,na,exactly,2),/(1,6))]";
    assert!(p.dump.contains(old));
    p.dump = p
        .dump
        .replacen(old, group, 1)
        .replace("drs([A,B,C]", "drs([A,B,C,D]");
    p
}
fn positive_decimal(n: &str) -> bool {
    !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) && n.bytes().any(|b| b != b'0')
}
#[test]
fn t2_projection_admission_property() {
    let scratch = Scratch::new("admission");
    let mut checks = Checks::default();
    let mut totals = [0usize; 2];
    for comparison in ["eq", "geq", "greater", "exactly", "leq", "less"] {
        for count in ["0".into(), "1".into(), "2".into(), "9".repeat(100)] {
            for marker in [false, true] {
                for nesting in [0, 2] {
                    let mut leaves = vec![
                        "-(predicate(C,see,A,B),/(1,3))".to_string(),
                        format!("-(object(B,dog,countable,na,{comparison},{count}),/(1,6))"),
                    ];
                    if marker {
                        leaves.push("-(object(D,cat,countable,na,exactly,1),/(1,7))".into());
                    }
                    if nesting == 2 {
                        leaves.reverse();
                    }
                    let mut group = format!("[{}]", leaves.join(","));
                    for _ in 0..nesting {
                        group = format!("[[],{group}]");
                    }
                    let p = synthetic(&group);
                    let upper_ok = !["leq", "less"].contains(&comparison)
                        && (comparison != "exactly" || positive_decimal(&count));
                    let want = upper_ok && (marker || comparison == "exactly");
                    let r = certify(&scratch, &p, &p.expected().0);
                    let good = if want {
                        r.ok()
                    } else {
                        r.rc == Some(1)
                            && r.out.is_empty()
                            && r.err
                                == format!("ckc: certify: {}: unsupported(root_condition).\n", p.id)
                    };
                    totals[usize::from(want)] += 1;
                    checks.check(good, format!("cmp={comparison} count={count} marker={marker} depth={nesting} admission={want}: {r:?}"));
                }
            }
        }
    }
    assert_eq!(checks.count, 96);
    assert_eq!(totals, [60, 36]);
    println!("P2 domain: {} reject, {} accept", totals[0], totals[1]);
    checks.finish("P2 projection admission property");
}
#[test]
fn t2_projection_malformed_lists_counts_and_anchors() {
    let scratch = Scratch::new("malformed");
    let source = Probe::fixture("two", false);
    let expected = source.expected().0;
    let group = "[-(predicate(C,see,A,B),/(1,3)),-(object(B,dog,countable,na,exactly,2),/(1,6))]";
    let mut cases = vec![
        ("empty-group", "[]".to_string()),
        ("improper-tail", group.replacen(",-(object", "|-(object", 1)),
        ("variable-tail", "[-(object(B,dog,countable,na,exactly,2),/(1,6))|Tail]".into()),
        ("mixed-anchors", group.replacen("/(1,6)", "/(2,6)", 1)),
        ("missing-anchor", group.replacen("-(object(B,dog,countable,na,exactly,2),/(1,6))", "object(B,dog,countable,na,exactly,2)", 1)),
        ("no-anchors", "[object(B,dog,countable,na,exactly,2)]".into()),
        ("no-root-marker", "[can(drs([B,C],[-(predicate(C,see,A,B),/(1,3)),-(object(B,dog,countable,na,exactly,2),/(1,6))]))]".into()),
        ("nested-box-mixed-anchors", "[-(object(B,dog,countable,na,exactly,2),/(1,6)),can(drs([C],[-(predicate(C,see,A,B),/(2,3))]))]".into()),
    ];
    for count in ["-1", "1.5", "bogus", "N"] {
        cases.push((
            count,
            group.replace("exactly,2", &format!("exactly,{count}")),
        ));
    }
    let mut checks = Checks::default();
    for (name, body) in cases {
        let p = synthetic(&body);
        let r = certify(&scratch, &p, &expected);
        let why = if ["variable-tail", "1.5", "N"].contains(&name) {
            "dump_noncanonical"
        } else {
            "unsupported(root_condition)"
        };
        checks.check(
            r.rc == Some(1)
                && r.out.is_empty()
                && r.err == format!("ckc: certify: {}: {why}.\n", p.id),
            format!("{name}: {r:?}"),
        );
    }
    checks.finish("P2 malformed public projection inputs");
}
#[test]
fn t5_control_product_certifies_and_mutations_are_observable() {
    let scratch = Scratch::new("certify-control");
    let mut checks = Checks::default();
    for table in [false, true] {
        let p = Probe::fixture("control", table);
        let (doc, _) = p.expected();
        let good = certify(&scratch, &p, &doc);
        checks.check(good.ok(), format!("control table={table}: {good:?}"));
        for (before, after) in [("na,eq,1", "na,eq,2"), (",sleep)", ",see)")] {
            assert!(doc.contains(before));
            let altered = doc.replacen(before, after, 1);
            let bad = certify(&scratch, &p, &altered);
            let sentence = if before == ",sleep)" { 2 } else { 1 };
            checks.check(
                bad.rc == Some(1)
                    && bad.out.is_empty()
                    && bad.err == format!("ckc: certify: {}: clauses({sentence}).\n", p.id),
                format!("control mutant {before} table={table}: {bad:?}"),
            );
        }
    }
    checks.finish("T5 certifier positive/negative controls");
}
