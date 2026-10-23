use super::model::{self, Goal, Group, Probe, Term, a, event, gid, lit, obj, pos, rule, v};
use super::support::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

const CMPS: &[(&str, &str)] = &[
    ("exactly", "exactly"),
    ("geq", "at least"),
    ("greater", "more than"),
    ("leq", "at most"),
    ("less", "less than"),
];
fn simple() -> Vec<Probe> {
    let mut out = Vec::new();
    for &(cmp, word) in CMPS {
        for (noun, plural, other, others) in [
            ("dog", "dogs", "cat", "cats"),
            ("day", "days", "week", "weeks"),
        ] {
            for table in [false, true] {
                for mode in ["antecedent", "naf", "horn", "query", "mixed"] {
                    let id = format!("nt-{mode}-{noun}-{cmp}-{table}");
                    let mut source = obj(1, "man", "eq", 1);
                    source.extend(obj(2, noun, cmp, 2));
                    source.extend(event(3, "see", &[1, 2]));
                    let ace = format!("If a man sees {word} 2 {plural} then the man sleeps.");
                    let mut p = rule(
                        &id,
                        &ace,
                        mode,
                        table,
                        pos(source.clone()),
                        vec![1, 2, 3],
                        4,
                    );
                    match mode {
                        "naf" => {
                            p.ace = format!(
                                "Every man that does not provably see {word} 2 {plural} sleeps.\n"
                            );
                            p.groups[0].body = pos(obj(1, "man", "eq", 1));
                            p.groups[0].body.push(Goal::Naf(source[2..].to_vec()));
                            p.groups[0].refs = vec![1];
                            let e = gid("product", &id, 4, Term::L(vec![v(1)]));
                            p.groups[0].heads = vec![
                                lit("event", vec![e.clone(), a("sleep")]),
                                lit("arg", vec![e, a("1"), v(1)]),
                            ];
                        }
                        "horn" => {
                            p.ace = format!(
                                "If a man sees {word} 2 {plural} or a woman sees {word} 3 {others} then a bird sleeps.\n"
                            );
                            let mut b = obj(4, "woman", "eq", 1);
                            b.extend(obj(5, other, cmp, 3));
                            b.extend(event(6, "see", &[4, 5]));
                            p.groups.clear();
                            for (i, body, refs) in
                                [(1, source.clone(), vec![1, 2, 3]), (2, b, vec![4, 5, 6])]
                            {
                                let deps = Term::L(refs.iter().map(|&n| v(n)).collect());
                                let bird = gid("product", &id, 7, deps.clone());
                                let sleep = gid("product", &id, 8, deps);
                                p.groups.push(Group {
                                    heads: vec![
                                        lit(
                                            "entity",
                                            vec![bird.clone(), a("bird"), a("countable")],
                                        ),
                                        lit(
                                            "cardinality",
                                            vec![bird.clone(), a("na"), a("eq"), a("1")],
                                        ),
                                        lit("event", vec![sleep.clone(), a("sleep")]),
                                        lit("arg", vec![sleep, a("1"), bird]),
                                    ],
                                    body: pos(body),
                                    refs,
                                    variant: i,
                                });
                            }
                        }
                        "query" => {
                            p.ace = format!("Does a man see {word} 2 {plural}?\n");
                            p.query = Some(source);
                            p.groups.clear();
                        }
                        "mixed" => {
                            // Distinct time groups keep the positive witness outside its NAF bound.
                            let (other, others) = if noun == "day" {
                                ("month", "months")
                            } else {
                                (other, others)
                            };
                            p.ace = format!(
                                "If a man sees {word} 2 {plural} and it is not provable that the man sees {word} 3 {others} then the man sleeps.\n"
                            );
                            let mut neg = obj(4, other, cmp, 3);
                            neg.extend(event(5, "see", &[1, 4]));
                            p = rule(
                                &id,
                                p.ace.trim_end(),
                                mode,
                                table,
                                pos(source),
                                vec![1, 2, 3],
                                6,
                            );
                            p.groups[0].body.push(Goal::Naf(neg));
                        }
                        _ => (),
                    }
                    out.push(p);
                }
            }
        }
    }
    out
}
pub fn timed(shape: &'static str, cmp: &str, phrase: &str) -> (String, Vec<Term>, usize) {
    let mut ts = obj(1, "man", "eq", 1);
    let (verb, e, tail) = match shape {
        "duration" | "alias" | "after" | "before" | "within" => {
            let noun = if shape == "alias" { "d" } else { "hour" };
            ts.extend(obj(2, noun, cmp, 2));
            let anchored = ["after", "before", "within"].contains(&shape);
            if anchored {
                ts.extend(obj(3, "visit", "eq", 1));
            }
            let e = if anchored { 4 } else { 3 };
            ts.extend(event(e, "wait", &[1]));
            let prep = if shape == "duration" || shape == "alias" {
                "for"
            } else {
                shape
            };
            ts.push(lit("pp", vec![v(e), a(prep), v(2)]));
            ts.push(lit(
                "interval",
                vec![
                    v(e),
                    a(if prep == "for" { "duration" } else { prep }),
                    v(2),
                    a(if shape == "alias" { "day" } else { "hour" }),
                    if anchored { v(3) } else { a("none") },
                ],
            ));
            let plural = if shape == "alias" { "d" } else { "hours" };
            (
                "waits",
                e,
                format!(
                    "{prep} {phrase} 2 {plural}{}",
                    if anchored { " of a visit" } else { "" }
                ),
            )
        }
        "recurrence" => {
            ts.extend(obj(2, "interval", "eq", 1));
            ts.extend(obj(3, "hour", cmp, 2));
            ts.extend(event(4, "wait", &[1]));
            ts.push(lit("pp", vec![v(4), a("at"), v(2)]));
            ts.push(lit("recurrence", vec![v(4), v(3), a("hour")]));
            ("waits", 4, format!("at an interval of {phrase} 2 hours"))
        }
        "window" => {
            ts.extend(obj(2, "interval", "eq", 1));
            ts.extend(obj(3, "hour", cmp, 2));
            ts.extend(obj(4, "window", "eq", 1));
            ts.extend(obj(5, "day", "less", 7));
            ts.extend(obj(6, "visit", "eq", 1));
            ts.extend(event(7, "wait", &[1]));
            ts.push(lit("pp", vec![v(7), a("during"), v(4)]));
            ts.push(lit("pp", vec![v(7), a("at"), v(2)]));
            ts.push(lit(
                "recurrence_window",
                vec![v(7), v(3), a("hour"), v(6), v(5), a("day")],
            ));
            (
                "waits",
                7,
                format!(
                    "at an interval of {phrase} 2 hours during a window of less than 7 days of a visit"
                ),
            )
        }
        "range" => {
            ts.extend(obj(2, "hour", "geq", 2));
            ts.extend(obj(3, "hour", "eq", 7));
            ts.extend(event(4, "wait", &[1]));
            ts.push(lit("pp", vec![v(4), a("to"), v(3)]));
            ts.push(lit("range", vec![v(2), v(3)]));
            ts.push(lit("pp", vec![v(4), a("for"), v(2)]));
            ts.push(lit(
                "interval",
                vec![v(4), a("duration"), v(2), a("hour"), a("none")],
            ));
            ("waits", 4, "for at least 2 hours to 7 hours".into())
        }
        "frequency" => {
            ts.extend(obj(2, "book", cmp, 2));
            ts.extend(obj(3, "day", "eq", 1));
            ts.extend(event(4, "read", &[1, 2]));
            ts.push(lit("pp", vec![v(4), a("per"), v(3)]));
            ts.push(lit("frequency", vec![v(4), v(2), v(3), a("day")]));
            ("reads", 4, format!("{phrase} 2 books per 1 day"))
        }
        _ => panic!("shape {shape}"),
    };
    (format!("{verb} {tail}"), ts, e)
}
fn timings() -> Vec<Probe> {
    let mut out = Vec::new();
    for shape in [
        "duration",
        "alias",
        "after",
        "before",
        "within",
        "recurrence",
        "window",
        "range",
        "frequency",
    ] {
        for &(cmp, phrase) in CMPS {
            if shape == "range" && cmp != "geq" {
                continue;
            }
            for mode in ["antecedent", "naf", "query"] {
                let id = format!("nt-time-{shape}-{cmp}-{mode}");
                let (text, ts, e) = timed(shape, cmp, phrase);
                let ace = format!("If a man {text} then the man sleeps.");
                let mut p = rule(
                    &id,
                    &ace,
                    "timing",
                    true,
                    pos(ts.clone()),
                    (1..=e).collect(),
                    e + 1,
                );
                p.alias = shape == "alias";
                match mode {
                    "query" => {
                        let text = text
                            .replacen("waits ", "wait ", 1)
                            .replacen("reads ", "read ", 1);
                        p.ace = format!("Does a man {text}?\n");
                        p.query = Some(ts);
                        p.groups.clear();
                    }
                    "naf" => {
                        let text = text
                            .replacen("waits ", "wait ", 1)
                            .replacen("reads ", "read ", 1);
                        p = rule(
                            &id,
                            &format!("Every man that does not provably {text} sleeps."),
                            "timing",
                            true,
                            pos(obj(1, "man", "eq", 1)),
                            vec![1],
                            e + 1,
                        );
                        p.alias = shape == "alias";
                        p.groups[0].body.push(Goal::Naf(ts[2..].to_vec()));
                    }
                    _ => (),
                }
                out.push(p);
            }
        }
    }
    out
}
fn unchanged() -> Vec<Probe> {
    let mut out = Vec::new();
    for (cmp, phrase) in [("eq", ""), ("about", "")] {
        let mut ts = obj(1, "man", "eq", 1);
        ts.extend(obj(2, "hour", cmp, 2));
        if cmp == "about" {
            ts.push(lit("property", vec![v(2), a("approximate"), a("pos")]));
        }
        ts.extend(event(3, "wait", &[1]));
        ts.push(lit("pp", vec![v(3), a("for"), v(2)]));
        ts.push(lit(
            "interval",
            vec![v(3), a("duration"), v(2), a("hour"), a("none")],
        ));
        let text = if cmp == "about" {
            "2 approximate hours"
        } else {
            "2 hours"
        };
        let id = format!("nt-unchanged-{cmp}");
        out.push(rule(
            &id,
            &format!("If a man waits for {phrase}{text} then the man sleeps."),
            "unchanged",
            true,
            pos(ts.clone()),
            vec![1, 2, 3],
            4,
        ));
        if cmp == "about" {
            let id = "nt-root-about";
            let heads = ts
                .iter()
                .map(|t| {
                    t.substitute(&|x| {
                        if let Term::V(n) = x {
                            Some(gid("product", id, n[1..].parse().unwrap(), Term::L(vec![])))
                        } else {
                            None
                        }
                    })
                })
                .collect();
            out.push(Probe {
                row: "unchanged",
                id: id.into(),
                ace: format!("A man waits for {text}.\n"),
                table: true,
                groups: vec![Group {
                    heads,
                    body: vec![],
                    refs: vec![],
                    variant: 1,
                }],
                query: None,
                alias: false,
            });
        }
    }
    for &(cmp, phrase) in CMPS.iter().chain(&[("eq", "")]) {
        let (text, ts, e) = timed("duration", cmp, phrase);
        let text = text.replace("for  2", "for 2");
        let id = format!("nt-head-{cmp}");
        let deps = Term::L(vec![v(1), v(2)]);
        // Antecedent man + sleep event precede the consequent's quantity and wait event.
        let mut head = ts[2..].to_vec();
        head = head
            .iter()
            .map(|t| {
                t.substitute(&|x| {
                    if *x == v(2) {
                        Some(gid("product", &id, 3, deps.clone()))
                    } else if *x == v(e) {
                        Some(gid("product", &id, 4, deps.clone()))
                    } else {
                        None
                    }
                })
            })
            .collect();
        let mut body = obj(1, "man", "eq", 1);
        body.extend(event(2, "sleep", &[1]));
        out.push(Probe {
            row: "unchanged",
            id: id.clone(),
            ace: format!("If a man sleeps then the man {text}.\n"),
            table: true,
            groups: vec![Group {
                heads: head,
                body: pos(body),
                refs: vec![1, 2],
                variant: 1,
            }],
            query: None,
            alias: false,
        });
        if ["eq", "geq", "greater"].contains(&cmp) {
            let id = format!("nt-root-{cmp}");
            let heads = ts
                .iter()
                .map(|t| {
                    t.substitute(&|x| {
                        if let Term::V(n) = x {
                            Some(gid(
                                "product",
                                &id,
                                n[1..].parse().unwrap(),
                                Term::L(vec![]),
                            ))
                        } else {
                            None
                        }
                    })
                })
                .collect();
            out.push(Probe {
                row: "unchanged",
                id,
                ace: format!("A man {text}.\n"),
                table: true,
                groups: vec![Group {
                    heads,
                    body: vec![],
                    refs: vec![],
                    variant: 1,
                }],
                query: None,
                alias: false,
            });
        }
    }
    for table in [false, true] {
        let id = format!("nt-unchanged-na-{table}");
        let mut ts = obj(1, "man", "eq", 1);
        ts.push(lit("entity", vec![v(2), a("water"), a("mass")]));
        ts.push(lit("cardinality", vec![v(2), a("na"), a("na"), a("na")]));
        ts.extend(event(3, "drink", &[1, 2]));
        out.push(rule(
            &id,
            "If a man drinks some water then the man sleeps.",
            "unchanged",
            table,
            pos(ts),
            vec![1, 2, 3],
            4,
        ));
    }
    out
}
fn boundaries() -> Vec<Probe> {
    let mut out = Vec::new();
    for (index, count) in [
        "0".to_string(),
        "1".into(),
        "2".into(),
        "50".into(),
        "9".repeat(100),
    ]
    .iter()
    .enumerate()
    {
        for &(cmp, word) in CMPS {
            for noun in ["dog", "day"] {
                let id = format!("nt-boundary-{index}-{cmp}-{noun}");
                let plural = if count == "1" {
                    noun.to_string()
                } else {
                    format!("{noun}s")
                };
                let mut ts = obj(1, "man", "eq", 1);
                ts.extend(obj(2, noun, cmp, 2));
                if let Term::C(_, args) = &mut ts[3] {
                    args[4] = a(count);
                }
                ts.extend(event(3, "see", &[1, 2]));
                out.push(rule(
                    &id,
                    &format!("If a man sees {word} {count} {plural} then the man sleeps."),
                    "boundaries",
                    true,
                    pos(ts),
                    vec![1, 2, 3],
                    4,
                ));
            }
        }
    }
    out
}
pub fn probes() -> Vec<Probe> {
    simple()
        .into_iter()
        .chain(timings())
        .chain(unchanged())
        .chain(boundaries())
        .collect()
}
struct Compiler {
    stage: PathBuf,
}
impl Compiler {
    fn new(scratch: &Path) -> Self {
        let version = command(Path::new("swipl"), scratch, &["--version"], "");
        version.require_ok("SWI pin");
        assert!(version.stdout.contains("version 10.0.2 "));
        let stage = scratch.join("ape");
        copy_tree(&root().join("vendor/ape"), &stage);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let parser = stage.join("prolog/parser");
        let goal = format!(
            "working_directory(_,{}),[fit_to_plp],halt.",
            model::quote(parser.to_str().unwrap())
        );
        let built = command(
            Path::new("swipl"),
            scratch,
            &[
                "-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
            ],
            "",
        );
        assert_eq!(built.rc, Some(0), "APE stage: {built:?}");
        let clex = stage.join("prolog/lexicon/clex_lexicon.pl");
        command(
            Path::new("swipl"),
            scratch,
            &[
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-g",
                &format!(
                    "style_check(-discontiguous),qcompile({}),halt.",
                    model::quote(clex.to_str().unwrap())
                ),
                "-t",
                "halt(9)",
            ],
            "",
        )
        .require_ok("Clex stage");
        Self { stage }
    }
    fn run(&self, cwd: &Path, id: &str, mode: &str, ace: &str, table: bool) -> Run {
        fs::create_dir_all(cwd).unwrap();
        let tab = cwd.join("vocabulary.tsv");
        let ulex = cwd.join("lexicon.ulex");
        write(&tab, model::TABLE);
        write(&ulex, model::ULEX);
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
        if mode == "query" {
            args.push("question");
        }
        args.extend([
            self.stage.to_str().unwrap(),
            id,
            if table { tab.to_str().unwrap() } else { "-" },
            ulex.to_str().unwrap(),
        ]);
        if mode == "proof" {
            args.push("proof");
        }
        command(Path::new("swipl"), cwd, &args, ace)
    }
    fn dump(&self, cwd: &Path, ace: &str) -> Run {
        let driver = root().join("rust/ckc/prolog/drs_dump.pl");
        let ulex = cwd.join("lexicon.ulex");
        command(
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
                self.stage.to_str().unwrap(),
                ulex.to_str().unwrap(),
            ],
            ace,
        )
    }
}
struct Results {
    models: BTreeMap<String, Probe>,
    docs: BTreeMap<String, Run>,
    proofs: BTreeMap<String, Run>,
    reds: BTreeMap<String, Run>,
}
fn red_cases() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "root-leq",
            "doc",
            "A man sees at most 2 dogs.\n",
            "root_condition(",
        ),
        (
            "root-less",
            "doc",
            "A man sees less than 2 dogs.\n",
            "root_condition(",
        ),
        (
            "range-high-exactly",
            "query",
            "Does a man wait for at least 2 hours to exactly 3 hours?\n",
            "temporal_shape(range_shape,1)",
        ),
        (
            "range-high-less",
            "query",
            "Does a man wait for at least 2 hours to less than 3 hours?\n",
            "temporal_shape(range_shape,1)",
        ),
        (
            "range-high-leq",
            "query",
            "Does a man wait for at least 2 hours to at most 3 hours?\n",
            "temporal_shape(range_shape,1)",
        ),
        (
            "about-exactly",
            "query",
            "Does a man wait for exactly 2 approximate hours?\n",
            "temporal_shape(approximate_bound,1)",
        ),
        (
            "about-leq",
            "query",
            "Does a man wait for at most 2 approximate hours?\n",
            "temporal_shape(approximate_bound,1)",
        ),
    ]
}
fn self_blocked() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (cmp, words) in [("leq", "at most"), ("less", "less than")] {
        for (kind, noun, other) in [("time", "days", "weeks"), ("nontime", "dogs", "dogs")] {
            out.push((format!("nt-naf-subset-{kind}-{cmp}"),format!("If a man sees {words} 2 {noun} and it is not provable that the man sees {words} 3 {other} then the man sleeps.\n")));
        }
    }
    out
}
fn results() -> &'static Results {
    // Cache setup failures too: one malformed fixture must not rerun every probe per test.
    static R: OnceLock<Result<Results, ()>> = OnceLock::new();
    R.get_or_init(|| std::panic::catch_unwind(collect_results).map_err(|_| ()))
        .as_ref()
        .expect("compiler fixture initialization failed; see its first diagnostic")
}
fn collect_results() -> Results {
    let scratch = Scratch::new("compiler");
    let compiler = Compiler::new(&scratch.0);
    let cases = probes();
    let next = AtomicUsize::new(0);
    let found = Mutex::new(Results {
        models: BTreeMap::new(),
        docs: BTreeMap::new(),
        proofs: BTreeMap::new(),
        reds: BTreeMap::new(),
    });
    std::thread::scope(|scope| {
        for worker in 0..4 {
            let (compiler, cases, next, found, scratch) =
                (&compiler, &cases, &next, &found, &scratch);
            scope.spawn(move || {
                let work = scratch.0.join(format!("worker-{worker}"));
                while let Some(p) = cases.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let got = compiler.run(
                        &work,
                        &p.id,
                        if p.query.is_some() { "query" } else { "doc" },
                        &p.ace,
                        p.table,
                    );
                    let proof = p
                        .query
                        .is_none()
                        .then(|| compiler.run(&work, &p.id, "proof", &p.ace, p.table));
                    let dump = compiler.dump(&work, &p.ace);
                    dump.require_ok(&format!("{} independent DRS", p.id));
                    let mut expected = p.clone();
                    model::source_order(&mut expected, &dump.stdout);
                    let mut r = found.lock().unwrap();
                    r.models.insert(p.id.clone(), expected);
                    r.docs.insert(p.id.clone(), got);
                    if let Some(proof) = proof {
                        r.proofs.insert(p.id.clone(), proof);
                    }
                }
            });
        }
    });
    let mut found = found.into_inner().unwrap();
    for (id, mode, ace, _) in red_cases() {
        found.reds.insert(
            id.into(),
            compiler.run(&scratch.0.join("red"), id, mode, ace, true),
        );
    }
    for (id, ace) in self_blocked() {
        found.reds.insert(
            id.clone(),
            compiler.run(&scratch.0.join("self-blocked"), &id, "doc", &ace, true),
        );
    }
    found
}
fn rejected_query(p: &Probe) -> bool {
    !p.table
        && p.query.as_ref().is_some_and(|goals| {
            goals.iter().any(|g| {
                g.name() == "guideline_cardinality"
                    && ["exactly", "leq", "less"].contains(&g.args()[3].name())
            })
        })
}
fn query_refusal(got: &Run) -> bool {
    got.rc == Some(1)
        && got.stdout.is_empty()
        && got.stderr == "ace_to_pl_error(unsupported,query_unsupported(leaf(list),1)).\n"
}
fn exact_row(row: &str) {
    let r = results();
    let mut checks = Checks::default();
    for original in probes().iter().filter(|p| p.row == row) {
        let p = &r.models[&original.id];
        let got = &r.docs[&p.id];
        if rejected_query(p) {
            checks.require(
                query_refusal(got),
                format!("{} tableless upper-group admission boundary: {got:?}", p.id),
            );
            continue;
        }
        checks.require(
            got.ok(),
            format!("{} rc={:?} stderr={:?}", p.id, got.rc, got.stderr),
        );
        if got.ok() {
            checks.equal(&got.stdout, &p.expected(true), &p.id);
        }
    }
    assert!(checks.count > 0);
    checks.finish(&format!("P3 {row}"));
}
#[test]
fn p3_oracle_inverse_and_own_bad_inputs() {
    let cases = probes();
    assert_eq!(cases.len(), 287);
    assert_eq!(cases.iter().filter(|p| p.query.is_none()).count(), 226);
    assert_eq!(cases.iter().filter(|p| rejected_query(p)).count(), 6);
    assert_eq!((red_cases().len(), self_blocked().len()), (7, 4));
    let mut census = BTreeMap::new();
    for p in &cases {
        *census.entry(p.row).or_insert(0usize) += 1;
    }
    assert_eq!(
        census,
        BTreeMap::from([
            ("antecedent", 20),
            ("naf", 20),
            ("horn", 20),
            ("query", 20),
            ("mixed", 20),
            ("timing", 123),
            ("unchanged", 14),
            ("boundaries", 50)
        ])
    );
    for p in &cases {
        assert_eq!(
            model::erase(&p.expected_drs(true), p.alias),
            p.expected_drs(false),
            "{} independent inverse",
            p.id
        );
    }
    let p = cases
        .iter()
        .find(|p| p.id == "nt-antecedent-day-geq-true")
        .unwrap();
    let good = p.expected_drs(true);
    for (before, after) in [
        ("guideline_unit(actual,B,F), ", ""),
        ("guideline_satisfies(D,E,F,geq,2,day), ", ""),
        (
            "guideline_satisfies(D,E,F,geq,2,day)",
            "guideline_satisfies(D,E,F,less,2,day)",
        ),
        ("guideline_unit(actual,B,F)", "guideline_unit(actual,B,Z)"),
    ] {
        assert!(good.contains(before), "self-plant anchor {before}");
        assert_ne!(
            model::erase(&good.replacen(before, after, 1), false),
            p.expected_drs(false),
            "erasure missed {before}"
        );
    }
    println!(
        "P3 oracle: {} inverse checks + 4 planted erasure defects",
        cases.len()
    );
}
#[test]
fn p3_endpoint_boundaries_zero_one_and_large() {
    exact_row("boundaries");
}
#[test]
fn p3_antecedent_each_comparison_time_and_nontime() {
    exact_row("antecedent");
}
#[test]
fn p3_horn_split_keeps_source_ordinals_and_dependencies() {
    exact_row("horn");
}
#[test]
fn p3_naf_locals_each_comparison_time_and_nontime() {
    exact_row("naf");
}
#[test]
fn p3_mixed_positive_and_naf_freshness() {
    exact_row("mixed");
}
#[test]
fn p3_question_each_comparison_time_and_nontime() {
    exact_row("query");
}
#[test]
fn p3_timing_links_all_consumers_and_range_pp_order() {
    exact_row("timing");
}
#[test]
fn p3_eq_about_heads_and_roots_only_gain_units() {
    exact_row("unchanged");
}
#[test]
fn p3_d4_proof_payload_exact_reflexive_witnesses() {
    let r = results();
    let mut checks = Checks::default();
    for original in probes().iter().filter(|p| p.query.is_none()) {
        let p = &r.models[&original.id];
        let got = &r.proofs[&p.id];
        checks.require(
            got.ok(),
            format!("{} proof rc={:?} stderr={:?}", p.id, got.rc, got.stderr),
        );
        if got.ok() {
            checks.equal(&got.stdout, &p.proof(true), &format!("{} proof", p.id));
        }
    }
    checks.finish("P3 D4 payload");
}
#[test]
fn p3_projection_preservation_every_green_probe() {
    let r = results();
    let mut checks = Checks::default();
    for original in probes() {
        let p = &r.models[&original.id];
        assert_eq!(
            model::erase(&p.expected_drs(true), p.alias),
            p.expected_drs(false),
            "{} oracle insertion/erasure",
            p.id
        );
        let got = &r.docs[&p.id];
        if rejected_query(p) {
            checks.require(
                query_refusal(got),
                format!("{} tableless upper-group admission boundary: {got:?}", p.id),
            );
            continue;
        }
        checks.require(
            got.ok(),
            format!("{} green rc={:?} stderr={:?}", p.id, got.rc, got.stderr),
        );
        if got.ok() {
            let (erased, old) = (model::erase(&got.stdout, p.alias), p.expected(false));
            checks.require(
                same_up_to_body_order(&erased, &old),
                format!("{} old projection up to body order:\n{erased}\n{old}", p.id),
            );
        }
    }
    checks.finish("P3 projection preservation");
}

// Erasing the annotations rebuilds an old time noun where its measure now stands, so the
// erased binding-first body (contract bf) can order the old goals differently: same lines,
// each rule with the same head and the same body goals.
fn same_up_to_body_order(a: &str, b: &str) -> bool {
    let goals = |l: &str| -> (String, Vec<String>) {
        match l.strip_suffix('.').and_then(|l| l.split_once(" :- ")) {
            Some((head, body)) => {
                let (mut gs, mut depth, mut start) = (Vec::new(), 0i32, 0);
                for (i, ch) in body.char_indices() {
                    match ch {
                        '(' | '[' => depth += 1,
                        ')' | ']' => depth -= 1,
                        ',' if depth == 0 => {
                            gs.push(body[start..i].trim().to_owned());
                            start = i + 1;
                        }
                        _ => {}
                    }
                }
                gs.push(body[start..].trim().to_owned());
                gs.sort();
                (head.to_owned(), gs)
            }
            None => (l.to_owned(), Vec::new()),
        }
    };
    a.lines().count() == b.lines().count() && a.lines().zip(b.lines()).all(|(x, y)| goals(x) == goals(y))
}
#[test]
fn p3_rejects_keep_upper_root_range_and_about_boundaries() {
    let r = results();
    let mut checks = Checks::default();
    for (id, _, _, why) in red_cases() {
        let got = &r.reds[id];
        let expected = format!("ace_to_pl_error(unsupported,{why}).\n");
        let diagnostic = if why == "root_condition(" {
            got.stderr
                .starts_with("ace_to_pl_error(unsupported,root_condition(")
                && got.stderr.ends_with(")).\n")
                && got.stderr.lines().count() == 1
        } else {
            got.stderr == expected
        };
        checks.require(
            got.rc == Some(1) && got.stdout.is_empty() && diagnostic,
            format!("{id} expected rc=1 diagnostic={why:?}; got={got:?}"),
        );
    }
    checks.finish("P3 reject boundaries");
}

// root-exact: the root `exactly N` group (N >= 1) that `red_cases` once rejected compiles as
// facts that keep their literals; product ordinals follow APE's list order (see before dog),
// which `source_order` (raw referent = ordinal) does not model, so heads stay in emitted order.
#[test]
fn p3_root_exactly_admitted_as_literal_facts() {
    let scratch = Scratch::new("root-exactly");
    let compiler = Compiler::new(&scratch.0);
    let id = "nt-root-exactly";
    let mut ts = obj(1, "man", "eq", 1);
    ts.extend(event(2, "see", &[1, 3]));
    ts.extend(obj(3, "dog", "exactly", 2));
    let heads = ts
        .iter()
        .map(|t| {
            t.substitute(&|x| {
                if let Term::V(n) = x {
                    Some(gid("product", id, n[1..].parse().unwrap(), Term::L(vec![])))
                } else {
                    None
                }
            })
        })
        .collect();
    let probe = Probe {
        row: "root-exact",
        id: id.into(),
        ace: "A man sees exactly 2 dogs.\n".into(),
        table: false,
        groups: vec![Group {
            heads,
            body: vec![],
            refs: vec![],
            variant: 1,
        }],
        query: None,
        alias: false,
    };
    let got = compiler.run(&scratch.0.join("doc"), id, "doc", &probe.ace, false);
    assert!(got.ok(), "{id} rc={:?} stderr={:?}", got.rc, got.stderr);
    assert_eq!(got.stdout, probe.expected(true), "{id}");
}
#[test]
fn p3_naf_inclusion_rejects_self_blocked_witnesses() {
    let results = results();
    let mut checks = Checks::default();
    for (id, _) in self_blocked() {
        let got = &results.reds[&id];
        checks.require(got.rc==Some(1) && got.stdout.is_empty() && got.stderr=="ace_to_pl_error(proof,underivable_obligation(1,variant(1))).\n",format!("{id}: expected proof refusal; rc={:?}, stdout bytes={}, first line={:?}, stderr={:?}",got.rc,got.stdout.len(),got.stdout.lines().next(),got.stderr));
    }
    checks.finish("P3 self-blocked NAF witnesses");
}
