use super::base::{self, Product};
use super::model::{self, Projection, Scene, atom, without_new_block};
use super::support::*;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

pub struct Observed {
    pub projection: Projection,
    pub doc: Run,
    pub proof: Run,
    pub certified: Run,
    pub baseline: Run,
    pub baseline_proof: Run,
    pub tableless: Run,
}
fn observe(scene: Scene) -> Observed {
    let compiler = compiler();
    let scratch = Scratch::new(&scene.id);
    let work = &scratch.0;
    let mut sentences = Vec::new();
    let mut last_dump = None;
    for line in scene.ace.lines() {
        let ace = format!("{line}\n");
        let parsed = compiler.dump(work, &ace);
        parsed.require_ok(&format!("{} APE DRS", scene.id));
        sentences.push(Product::from_dump(&scene.id, &ace, &parsed.stdout, true));
        last_dump = Some(parsed);
    }
    let dump = if sentences.len() == 1 {
        last_dump.unwrap()
    } else {
        compiler.dump(work, &scene.ace)
    };
    dump.require_ok(&format!("{} whole-document dump", scene.id));
    assert!(
        dump.stdout.contains("messages([])."),
        "{} syntax: {}",
        scene.id,
        dump.stdout
    );
    let projection = Projection { scene, sentences };
    let scene = &projection.scene;
    let table = scene.table(true);
    let doc = compiler.run(work, &scene.id, "doc", &scene.ace, Some(table.as_bytes()));
    let proof = compiler.run(work, &scene.id, "proof", &scene.ace, Some(table.as_bytes()));
    // The certifier always receives the independent document, never the fork's output.
    let certified = certify(
        work,
        &scene.id,
        &scene.ace,
        &dump.stdout,
        &projection.document(true, true),
        Some(&table),
    );
    let baseline_table = scene.table(false);
    let baseline = compiler.run(
        work,
        &scene.id,
        "doc",
        &scene.ace,
        Some(baseline_table.as_bytes()),
    );
    let baseline_proof = compiler.run(
        work,
        &scene.id,
        "proof",
        &scene.ace,
        Some(baseline_table.as_bytes()),
    );
    let tableless = compiler.run(work, &scene.id, "doc", &scene.ace, None);
    Observed {
        projection,
        doc,
        proof,
        certified,
        baseline,
        baseline_proof,
        tableless,
    }
}
pub fn results() -> &'static BTreeMap<String, Observed> {
    static RESULTS: OnceLock<BTreeMap<String, Observed>> = OnceLock::new();
    RESULTS.get_or_init(|| {
        // One stage for every scene: the scenes' own handles keep this one alive.
        let _stage = compiler();
        let cases = model::scenes();
        let next = AtomicUsize::new(0);
        let found = Mutex::new(BTreeMap::new());
        std::thread::scope(|scope| {
            for _ in 0..4 {
                let (cases, next, found) = (&cases, &next, &found);
                scope.spawn(move || {
                    while let Some(scene) = cases.get(next.fetch_add(1, Ordering::Relaxed)) {
                        let observed = observe(scene.clone());
                        found.lock().unwrap().insert(scene.id.clone(), observed);
                    }
                });
            }
        });
        found.into_inner().unwrap()
    })
}
fn products(family: &str) {
    let mut checks = Checks::default();
    for obs in results()
        .values()
        .filter(|o| o.projection.scene.family == family)
    {
        let p = &obs.projection;
        checks.output(&obs.doc, &Run::ok(p.document(true, true)), &p.scene.id);
        checks.output(
            &obs.proof,
            &Run::ok(p.proof(true, true)),
            &format!("{}/proof", p.scene.id),
        );
    }
    assert!(checks.count > 0);
    checks.finish(&format!("P2 exact products/{family}"));
}
macro_rules! product_tests {
    ($($name:ident => $family:literal),+ $(,)?) => { $(#[test] fn $name() { products($family); })+ };
}
product_tests! {
    p2_a_plain_should => "a",
    p2_b_must_and_may => "b",
    p2_c_horn_split_guards_each_variant => "c",
    p2_d_should_at_any_modal_depth => "d",
    p2_e_mixed_asserted_and_deontic_all_clauses_guarded => "e",
    p2_f_can_only_and_asserted_remain_unguarded => "f",
    p2_g_scope_guard_stays_last => "g",
    p2_h_exclusion_precedes_supersession_record => "h",
    p2_i_root_facts_record_only => "i",
    p2_one_record_last_in_every_sentence_bundle => "bundles",
    p2_fresh_variable_naf_and_curried_conditions => "fresh",
    p2_date_boundaries_and_atom_escaping => "boundary",
}
#[test]
fn p2_same_input_without_rows_and_without_table_characterization() {
    let mut checks = Checks::default();
    for obs in results().values() {
        let p = &obs.projection;
        checks.output(
            &Run {
                stdout: without_new_block(&obs.baseline.stdout),
                ..obs.baseline.clone()
            },
            &Run::ok(without_new_block(&p.document(false, true))),
            &format!("{}/baseline", p.scene.id),
        );
        // A schema-only insertion is irrelevant to the preserved clause bytes.
        if obs.baseline.success() {
            checks.equal(
                &without_new_block(&obs.baseline.stdout),
                &without_new_block(&p.document(false, true)),
                &format!("{}/same input", p.scene.id),
            );
        }
        checks.output(
            &Run {
                stdout: without_new_block(&obs.tableless.stdout),
                ..obs.tableless.clone()
            },
            &Run::ok(without_new_block(&p.document(false, false))),
            &format!("{}/tableless", p.scene.id),
        );
        checks.output(
            &obs.baseline_proof,
            &Run::ok(p.proof(false, true)),
            &format!("{}/unchanged proof", p.scene.id),
        );
        if p.sentences.len() == 1 {
            checks.equal(
                &base::without_block(&without_new_block(&p.document(false, true))),
                &base::without_block(&p.baseline_definition()),
                &format!("{}/prior independent definition", p.scene.id),
            );
        }
    }
    checks.finish("P2 already-passing same-input preservation");
}
#[test]
fn p2_unmatched_document_and_passage_rows_preserve_projection() {
    let compiler = compiler();
    let scratch = Scratch::new("unmatched");
    let mut checks = Checks::default();
    for obs in results()
        .values()
        .filter(|o| ["a", "i"].contains(&o.projection.scene.family))
    {
        let p = &obs.projection;
        let mut table = p.scene.table(false);
        table.push_str("supersession\tother-document\t20221229 https://a C\nsuperseded\tpassage-only\t20221229 https://a C\n");
        let expected = p.document(false, true).replace(
            &format!("vocabulary(sha256({}))", atom(&sha(p.scene.table(false)))),
            &format!("vocabulary(sha256({}))", atom(&sha(&table))),
        );
        let got = compiler.run(
            &scratch.0,
            &p.scene.id,
            "doc",
            &p.scene.ace,
            Some(table.as_bytes()),
        );
        checks.output(&got, &Run::ok(expected), &p.scene.id);
        let got = compiler.run(
            &scratch.0,
            &p.scene.id,
            "proof",
            &p.scene.ace,
            Some(table.as_bytes()),
        );
        checks.output(
            &got,
            &Run::ok(p.proof(false, true)),
            &format!("{}/no payload effect", p.scene.id),
        );
    }
    checks.finish("P2 unmatched kinds no compile effect");
}
#[test]
fn p2_root_deontic_rejects_and_existing_reject_order() {
    let compiler = compiler();
    let scratch = Scratch::new("rejects");
    let mut checks = Checks::default();
    for op in ["should", "must", "may"] {
        for wrapper in ["", "It is possible that ", "It is false that "] {
            for scoped in [false, true] {
                for second in [false, true] {
                    let ace = format!(
                        "{}{}{} patient {op} wait.\n",
                        if second { "A man sleeps.\n" } else { "" },
                        wrapper,
                        if wrapper.is_empty() { "A" } else { "a" }
                    );
                    let mut scene = Scene::new("reject", "reject", &ace);
                    scene.scope = scoped;
                    let parsed = compiler.dump(&scratch.0, &scene.ace);
                    parsed.require_ok("reject DRS");
                    assert!(
                        parsed.stdout.contains("messages([])."),
                        "reject syntax: {}",
                        parsed.stdout
                    );
                    let got = compiler.run(
                        &scratch.0,
                        &scene.id,
                        "doc",
                        &scene.ace,
                        Some(scene.table(true).as_bytes()),
                    );
                    let why = if scoped {
                        "scope_shape"
                    } else {
                        "supersession_shape"
                    };
                    let expected = Run::err(
                        1,
                        format!(
                            "ace_to_pl_error(unsupported,{why}(root_deontic,{})).\n",
                            if second { 2 } else { 1 }
                        ),
                    );
                    checks.output(
                        &got,
                        &expected,
                        &format!(
                            "{op}/{wrapper:?}/scoped={scoped}/S{}",
                            if second { 2 } else { 1 }
                        ),
                    );
                }
            }
        }
    }
    let scene = Scene::new(
        "old-reject",
        "reject",
        "If a patient should sleep then it is possible that every man waits.",
    );
    let got = compiler.run(
        &scratch.0,
        &scene.id,
        "doc",
        &scene.ace,
        Some(scene.table(true).as_bytes()),
    );
    checks.output(
        &got,
        &Run::err(
            1,
            "ace_to_pl_error(unsupported,operator_scoped_rule(can,1)).\n",
        ),
        "existing translation rejects first",
    );
    checks.finish("P2 root rejection + scoped precedence + sentence order");
}
fn questions(new_rows: bool) {
    let compiler = compiler();
    let scratch = Scratch::new("questions");
    let mut checks = Checks::default();
    for (i, ace) in [
        "Does a patient sleep?\n",
        "Should a patient wait?\n",
        "Can a patient work?\n",
        "Does a man wait?\n",
    ]
    .iter()
    .enumerate()
    {
        let id = format!("sup-query-{i}");
        let baseline_table = base::table(None);
        let mut table = baseline_table.clone();
        if new_rows {
            table.push_str(&format!(
                "supersession\t{id}\t20221229 https://a C\nsuperseded\tR1\t20221229 https://a C\n"
            ));
        }
        let before = compiler.run(&scratch.0, &id, "question", ace, None);
        before.require_ok("tableless question control");
        let got = compiler.run(&scratch.0, &id, "question", ace, Some(table.as_bytes()));
        let wanted = before.stdout.replacen(
            "vocabulary(none)",
            &format!("vocabulary(sha256({}))", atom(&sha(&table))),
            1,
        );
        assert_eq!(
            before.stdout.matches("vocabulary(none)").count(),
            1,
            "custody control"
        );
        checks.output(&got, &Run::ok(wanted), &id);
    }
    checks.finish(if new_rows {
        "P2 question new-row intake + projection identity"
    } else {
        "P2 question identity characterization"
    });
}
#[test]
fn p2_question_new_rows_change_only_vocabulary_digest() {
    questions(true);
}
#[test]
fn p2_question_without_new_rows_characterization() {
    questions(false);
}
fn payload_equality(fork: &Run, derived: &Run, model: &str) -> bool {
    fork.success() && derived.success() && fork.stdout == model && derived.stdout == model
}
#[test]
fn p5b_three_way_obligation_payload_equality() {
    let mut checks = Checks::default();
    for obs in results().values() {
        let expected = obs.projection.proof(true, true);
        checks.output(
            &obs.proof,
            &Run::ok(&expected),
            &format!("{}/fork=model", obs.projection.scene.id),
        );
        checks.output(
            &obs.certified,
            &Run::ok(&expected),
            &format!("{}/certify-one=model", obs.projection.scene.id),
        );
        checks.require(
            payload_equality(&obs.proof, &obs.certified, &expected),
            format!(
                "{}/named payload_equality(fork,certify-one,model)",
                obs.projection.scene.id
            ),
        );
    }
    checks.finish("P5b fork payload = certify-one obligations = witness model");
}
#[test]
fn p5b_altered_witness_payload_fires_equality_check() {
    let mut checks = Checks::default();
    for obs in results().values() {
        let p = &obs.projection;
        let wanted = p.proof(true, true);
        let old = format!("guideline_evaluation_date({})", p.scene.date - 1);
        if !wanted.contains(&old) {
            continue;
        }
        let plant = wanted.replacen(
            &old,
            &format!("guideline_evaluation_date({})", p.scene.date - 2),
            1,
        );
        assert_ne!(plant, wanted, "positive mutation anchor");
        checks.require(
            payload_equality(&obs.proof, &obs.certified, &wanted),
            format!(
                "{}/real payload equality prerequisite; fork {}; certifier {}",
                p.scene.id,
                obs.proof.describe(),
                obs.certified.describe()
            ),
        );
        checks.require(
            !payload_equality(&Run::ok(plant), &Run::ok(&wanted), &wanted),
            format!(
                "{}/altered witness must fail the same equality assertion",
                p.scene.id
            ),
        );
        checks.require(
            payload_equality(&Run::ok(&wanted), &Run::ok(&wanted), &wanted),
            format!("{}/oracle positive control", p.scene.id),
        );
    }
    checks.finish("P5b altered fork witness plant (not a certify input)");
}
// Codec ruling R22 over the free-text citation (SUP5 lens A finding): every admitted citation
// compiles to a document the codec + certifier accept, its atom quoted with raw UTF-8.
#[test]
fn p5c_admitted_citations_compile_to_canonical_documents() {
    let compiler = compiler();
    let ace = "If a patient sleeps then the patient should wait.\n";
    let mut checks = Checks::default();
    for (name, citation) in [
        ("ascii", "Act"),
        ("lower-accent", "é"),
        ("greek", "νόμος"),
        ("han", "法律"),
        ("capital-accent", "É"),
        ("c1-control", "Act\u{85}one"),
        ("line-separator", "Act\u{2028}one"),
        ("paragraph-separator", "Act\u{2029}one"),
        ("soft-hyphen", "Act\u{ad}one"),
        ("no-break-space", "Act\u{a0}one"),
        ("emoji", "📚"),
        ("quote-backslash", "Year zero's act \\ annex é"),
    ] {
        let scratch = Scratch::new(&format!("citation-{name}"));
        let work = &scratch.0;
        let table = format!(
            "{}supersession\tdoc\t20221229 https://example.test/ {citation}\n",
            base::HEADER
        );
        let doc = compiler.run(work, "doc", "doc", ace, Some(table.as_bytes()));
        checks.require(doc.success(), format!("{name} compile; {}", doc.describe()));
        let record = format!(
            "guideline_supersession(doc,1,source({},'https://example.test/'),20221229).",
            atom(citation)
        );
        checks.require(
            doc.stdout.lines().any(|line| line == record),
            format!("{name} record {record}; {}", doc.describe()),
        );
        write(work.join("doc.pl"), &doc.stdout);
        let codec = native(work, &["v1", "check", "doc.pl"]);
        checks.require(codec.success(), format!("{name} codec; {}", codec.describe()));
        let dump = compiler.dump(work, ace);
        let certified = certify(work, "doc", ace, &dump.stdout, &doc.stdout, Some(&table));
        checks.require(
            certified.success(),
            format!("{name} certify-one; {}", certified.describe()),
        );
    }
    checks.finish("P5 admitted citations through the codec");
}
