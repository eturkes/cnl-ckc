use super::compiler::{Compiler, compiled};
use super::model::{self, Goal, KEY, Product, a, c, quote};
use super::support::*;
use std::path::{Path, PathBuf};

fn query(id: &str, goal: &str) -> String {
    let goal = model::parse(goal).render();
    format!(
        "% {id} compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,{},ace_sha256({}),ulex(none),vocabulary(none)).\n% Q1: Is there a result?\n'$guideline_query_projection'(goal({goal}),answers([])).\n",
        quote(id),
        quote(&sha("Is there a result?\n"))
    )
}
fn expected_answer(id: &str, q: &str, result: &str) -> String {
    format!(
        "% {id} answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,{},query_sha256({}),result({result})).\n",
        quote(id),
        quote(&sha(q))
    )
}
fn structural_doc(id: &str, clauses: &[String]) -> String {
    format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex(none),vocabulary(none)).\n% S1: A composition fixture.\n{}",
        model::declarations(),
        quote(id),
        quote(&sha("A composition fixture.\n")),
        clauses
            .iter()
            .map(|l| format!("{l}.\n"))
            .collect::<String>()
    )
}
fn native_goal(
    cwd: &Path,
    manifest: &Path,
    id: &str,
    goal: &str,
    result: &str,
    checks: &mut Checks,
) {
    let q = query(id, goal);
    let path = cwd.join(format!("{id}.query"));
    write(&path, &q);
    let got = native(
        cwd,
        &[
            "v1",
            "answer",
            manifest.to_str().unwrap(),
            path.to_str().unwrap(),
        ],
    );
    checks.require(got.ok(), format!("{id} native {}", got.describe()));
    if got.ok() {
        checks.equal(&got.stdout, &expected_answer(id, &q, result), id);
    }
}
fn edge(doc: &str, op: &str, inner: usize, outer: Option<usize>) -> String {
    let ctx = |n| format!("'$guideline_id'(context,{},1,box({n}),D)", quote(doc));
    format!(
        "guideline_operator({},{},{op})",
        outer.map_or_else(|| "actual".into(), ctx),
        ctx(inner)
    )
}
fn swi(
    cwd: &Path,
    docs: &[PathBuf],
    goals: &[(&str, String)],
    checks: &mut Checks,
    label: &str,
    expected: &str,
) {
    let driver = cwd.join("portability-driver.pl");
    let files = docs
        .iter()
        .map(|p| quote(p.to_str().unwrap()))
        .collect::<Vec<_>>()
        .join(",");
    let calls = goals
        .iter()
        .map(|(id, g)| format!("verdict({}, {})", quote(id), quote(g)))
        .collect::<Vec<_>>()
        .join(",\n");
    write(
        &driver,
        format!(
            "verdict(Label, Text) :- read_term_from_atom(Text, Goal, []), (once(Goal) -> Answer=yes ; Answer=no), format('~w=~w~n', [Label, Answer]).\nprobe_main :- consult([{files}]), {calls}, halt.\n"
        ),
    );
    let got = command(
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
            "probe_main",
            "-t",
            "halt(9)",
        ],
        "",
    );
    checks.require(
        got.ok(),
        format!("{label} SWI consult/query: {}", got.describe()),
    );
    if got.ok() {
        checks.equal(&got.stdout, expected, &format!("{label} SWI edge verdicts"));
    }
}
fn store(
    cwd: &Path,
    id: &str,
    doc: &str,
    proof: &str,
    docs: &mut Vec<PathBuf>,
    manifest: &mut String,
) {
    let pl = cwd.join(format!("{id}.pl"));
    let payload = cwd.join(format!("{id}.proof"));
    write(&pl, doc);
    write(&payload, proof);
    manifest.push_str(&format!("{}\t{}\n", pl.display(), payload.display()));
    docs.push(pl);
}
#[test]
fn a8_swi_native_complete_worlds_and_empty_stderr() {
    let scratch = Scratch::new("portability");
    let compiler = Compiler::new(&scratch.0);
    let policies = [
        "cs-guard-should-top",
        "cs-guard-must-top",
        "cs-guard-may-top",
        "cs-guard-should-can",
    ];
    let mut checks = Checks::default();
    for (world, exclude, facts, gated) in [
        ("absent", "cs-exclude-one", vec![], false),
        (
            "ordinary",
            "cs-exclude-one",
            vec!["A patient sleeps.\n"],
            true,
        ),
        (
            "cancer",
            "cs-exclude-one",
            vec!["A patient sleeps and has a cancer-related-pain.\n"],
            false,
        ),
        (
            "any-loaded-patient",
            "cs-exclude-one",
            vec![
                "A patient sleeps.\n",
                "A patient has a cancer-related-pain.\n",
            ],
            false,
        ),
        (
            "hospital",
            "cs-exclude-naf",
            vec!["A patient sleeps and receives a hospital-care.\n"],
            false,
        ),
        (
            "discharge",
            "cs-exclude-naf",
            vec!["A patient sleeps and receives a hospital-care and has a facility-discharge.\n"],
            true,
        ),
        (
            "other-patient-discharge",
            "cs-exclude-naf",
            vec![
                "A patient sleeps and receives a hospital-care.\n",
                "A patient has a facility-discharge.\n",
            ],
            false,
        ),
        (
            "exception-no-hospital",
            "cs-exclude-naf",
            vec!["A patient sleeps and has a facility-discharge.\n"],
            true,
        ),
    ] {
        let cwd = scratch.0.join(world);
        std::fs::create_dir_all(&cwd).unwrap();
        let (mut docs, mut manifest) = (Vec::new(), String::new());
        let mut ready = true;
        for id in policies.into_iter().chain([exclude]) {
            let x = compiled(id);
            checks.require(
                x.doc.ok() && x.proof.ok(),
                format!(
                    "{world}/{id} compiled composition prerequisite: {}",
                    x.doc.describe()
                ),
            );
            if !x.doc.ok() || !x.proof.ok() {
                ready = false;
                continue;
            }
            store(
                &cwd,
                id,
                &x.doc.stdout,
                &x.proof.stdout,
                &mut docs,
                &mut manifest,
            );
        }
        for (label, key) in [("other", Some("scope-other")), ("unscoped", None)] {
            let id = format!("cs-{label}");
            let ace = "If a patient sleeps then it is recommended that the patient waits.\n";
            let tab = model::table(key);
            let doc = compiler.run(&cwd.join(label), &id, "doc", ace, Some(&tab));
            checks.require(doc.ok(), format!("{world}/{label}: {}", doc.describe()));
            if !doc.ok() {
                ready = false;
                continue;
            }
            store(&cwd, &id, &doc.stdout, "", &mut docs, &mut manifest);
        }
        for (i, ace) in facts.iter().enumerate() {
            let id = format!("{world}-patient-{i}");
            let out = compiler.run(&cwd.join("facts"), &id, "doc", ace, None);
            out.require_ok(&id);
            store(&cwd, &id, &out.stdout, "", &mut docs, &mut manifest);
        }
        if !ready {
            continue;
        }
        let m = cwd.join("manifest.tsv");
        write(&m, &manifest);
        let scan = native(&cwd, &["v1", "recursion-check", m.to_str().unwrap()]);
        checks.require(
            scan.ok(),
            format!("{world} exact composition guard scan: {}", scan.describe()),
        );
        let has_patient = !facts.is_empty();
        let goals = vec![
            ("should", edge(policies[0], "should", 1, None)),
            ("must", edge(policies[1], "must", 1, None)),
            ("may", edge(policies[2], "may", 1, None)),
            ("nested-should", edge(policies[3], "should", 2, Some(1))),
            ("can", edge(policies[3], "can", 1, None)),
            (
                "payload",
                format!(
                    "guideline_event('$guideline_id'(context,{},1,box(1),D),E,wait)",
                    quote(policies[0])
                ),
            ),
            ("other-key", edge("cs-other", "should", 1, None)),
            ("unscoped", edge("cs-unscoped", "should", 1, None)),
            ("exclusion", format!("guideline_exclusion({})", quote(KEY))),
        ];
        let mut expected = String::new();
        for (label, goal) in &goals {
            let yes = match *label {
                "should" | "must" | "may" | "nested-should" => gated,
                "exclusion" => has_patient && !gated,
                _ => has_patient,
            };
            let id = format!("{world}-{label}");
            native_goal(
                &cwd,
                &m,
                &id,
                goal,
                if yes { "yes" } else { "no(finite_failure)" },
                &mut checks,
            );
            expected.push_str(&format!("{label}={}\n", if yes { "yes" } else { "no" }));
        }
        swi(&cwd, &docs, &goals, &mut checks, world, &expected);
    }
    checks.finish("A8 P6 complete-world native/SWI differential");
}
#[test]
fn a8_knowledge_witness_edge_portability() {
    let scratch = Scratch::new("witness-portability");
    let mut checks = Checks::default();
    let empty = scratch.0.join("empty.tsv");
    write(&empty, "");
    native_goal(
        &scratch.0,
        &empty,
        "empty-control",
        &edge("cs-guard-should-top", "should", 1, None),
        "no(finite_failure)",
        &mut checks,
    );
    let x = compiled("cs-guard-should-top");
    let exclusion = compiled("cs-exclude-one");
    checks.require(
        x.doc.ok() && exclusion.doc.ok(),
        format!(
            "knowledge prerequisites: {}; {}",
            x.doc.describe(),
            exclusion.doc.describe()
        ),
    );
    if x.doc.ok() && exclusion.doc.ok() {
        let (mut docs, mut manifest) = (Vec::new(), String::new());
        store(
            &scratch.0,
            &x.probe.id,
            &x.doc.stdout,
            &x.proof.stdout,
            &mut docs,
            &mut manifest,
        );
        store(
            &scratch.0,
            &exclusion.probe.id,
            &exclusion.doc.stdout,
            &exclusion.proof.stdout,
            &mut docs,
            &mut manifest,
        );
        let g = &x.model.groups[0];
        let facts: Vec<_> = g
            .body
            .iter()
            .filter_map(|b| match b {
                Goal::Pos(t) if t.name() != "guideline_satisfies" => {
                    Some(t.sub(&g.witness).render())
                }
                _ => None,
            })
            .collect();
        store(
            &scratch.0,
            "witness",
            &structural_doc("witness", &facts),
            "",
            &mut docs,
            &mut manifest,
        );
        let m = scratch.0.join("manifest.tsv");
        write(&m, &manifest);
        let goals = vec![("should", edge(&x.probe.id, "should", 1, None))];
        native_goal(
            &scratch.0,
            &m,
            "witness-should",
            &goals[0].1,
            "yes",
            &mut checks,
        );
        swi(
            &scratch.0,
            &docs,
            &goals,
            &mut checks,
            "knowledge witness",
            "should=yes\n",
        );
    }
    checks.finish("A8 P6 own-witness portability");
}
#[test]
fn a8_deep_exclusion_native_limit_swi_blocks() {
    let scratch = Scratch::new("deep-portability");
    let compiler = Compiler::new(&scratch.0);
    let mut checks = Checks::default();
    let policy = compiled("cs-guard-should-top");
    let exclude = compiled("cs-exclude-one");
    checks.require(
        policy.doc.ok() && exclude.doc.ok(),
        format!(
            "deep compiled prerequisites: {}; {}",
            policy.doc.describe(),
            exclude.doc.describe()
        ),
    );
    if policy.doc.ok() && exclude.doc.ok() {
        let (mut docs, mut manifest) = (Vec::new(), String::new());
        for x in [policy, exclude] {
            store(
                &scratch.0,
                &x.probe.id,
                &x.doc.stdout,
                &x.proof.stdout,
                &mut docs,
                &mut manifest,
            );
        }
        let ace = "A patient sleeps.\n";
        let patient = compiler.run(&scratch.0.join("patient"), "deep-patient", "doc", ace, None);
        patient.require_ok("deep patient");
        let dump = compiler.dump(&scratch.0.join("dump"), ace);
        dump.require_ok("deep patient DRS");
        let scene = Product::from_dump("deep-patient", ace, &dump.stdout, false);
        let person = scene.groups[0]
            .heads
            .iter()
            .find(|h| h.name() == "guideline_entity")
            .unwrap()
            .args()[1]
            .render();
        store(
            &scratch.0,
            "deep-patient",
            &patient.stdout,
            "",
            &mut docs,
            &mut manifest,
        );
        let mut tail = vec![
            "guideline_entity(actual,cancer,'cancer-related-pain',countable)".into(),
            "guideline_cardinality(actual,cancer,na,eq,1)".into(),
            format!("guideline_arg(actual,deep0,1,{person})"),
            "guideline_arg(actual,deep0,2,cancer)".into(),
        ];
        for n in 0..128 {
            tail.push(format!(
                "guideline_event(actual,deep{n},have) :- guideline_event(actual,deep{},have)",
                n + 1
            ));
        }
        tail.push("guideline_event(actual,deep128,have)".into());
        store(
            &scratch.0,
            "deep-chain",
            &structural_doc("deep-chain", &tail),
            "",
            &mut docs,
            &mut manifest,
        );
        let m = scratch.0.join("manifest.tsv");
        write(&m, &manifest);
        let goal = edge(&policy.probe.id, "should", 1, None);
        native_goal(
            &scratch.0,
            &m,
            "deep-should",
            &goal,
            "indeterminate(limit)",
            &mut checks,
        );
        let exclusion = c("guideline_exclusion", vec![a(KEY)]).render();
        let goals = vec![("should", goal), ("exclusion", exclusion)];
        swi(
            &scratch.0,
            &docs,
            &goals,
            &mut checks,
            "depth 128 > answer depth 100",
            "should=no\nexclusion=yes\n",
        );
    }
    checks.finish("A8 deep exception: bounded native differs from unbounded SWI");
}
