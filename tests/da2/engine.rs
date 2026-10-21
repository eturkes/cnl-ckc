use super::cases::{self, Case, ULEX};
use super::model::{self, Product};
use super::support::*;
use super::surface::results;
use super::term::*;

fn install(dir: &std::path::Path, docs: &[(&str, &str)]) {
    let mut manifest = String::new();
    for (i, (pl, proof)) in docs.iter().enumerate() {
        write(dir.join(format!("d{i}.pl")), pl);
        write(dir.join(format!("d{i}.proof")), proof);
        manifest.push_str(&format!("d{i}.pl\td{i}.proof\n"));
    }
    write(dir.join("m.tsv"), manifest);
}
fn query(goal: &Term) -> String {
    format!(
        "% arithmetic compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,arithmetic,ace_sha256({}),ulex(none),vocabulary(none)).\n% Q1: Is there a result?\n{}.\n",
        quote(&sha("Is there a result?\n")),
        c(
            "$guideline_query_projection",
            vec![
                c("goal", vec![goal.clone()]),
                c("answers", vec![Term::List(vec![])])
            ]
        )
        .render()
    )
}
fn answers(q: &str, holds: bool) -> String {
    format!(
        "% arithmetic answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,arithmetic,query_sha256({}),result({})).\n",
        quote(&sha(q)),
        if holds { "yes" } else { "no(finite_failure)" }
    )
}
fn check_query(checks: &mut Checks, dir: &std::path::Path, goal: &Term, holds: bool, label: &str) {
    let q = query(goal);
    let a = answers(&q, holds);
    write(dir.join("query.pl"), &q);
    checks.output(
        &native(dir, &["v1", "answer", "m.tsv", "query.pl"]),
        &Run::ok(&a),
        label,
    );
    write(dir.join("answers.pl"), &a);
}
#[test]
fn p4_payload_replay_coherent_alternative_and_wrong_head() {
    let obs = &results()["da2-product"];
    let work = Scratch::new("payload-replay");
    let mut checks = Checks::default();
    let pl = obs.model.document();
    let proof = obs.model.proof();
    install(&work.0, &[(&pl, &proof)]);
    checks.output(
        &native(&work.0, &["v1", "aggregate-check", "m.tsv"]),
        &Run::ok("ace_to_pl aggregate ok 1 documents 1 obligations\n"),
        "model payload replay",
    );
    checks.output(
        &obs.proof,
        &Run::ok(&proof),
        "fork pins deterministic witness",
    );
    checks.output(
        &obs.certified,
        &Run::ok(&proof),
        "certifier pins deterministic witness",
    );
    let witness = "na,exactly,1)";
    let head = "na,exactly,rational(3,2))";
    assert_eq!(proof.matches(witness).count(), 1);
    assert_eq!(proof.matches(head).count(), 1);
    let coherent = proof
        .replacen(witness, "na,exactly,2)", 1)
        .replacen(head, "na,exactly,3)", 1);
    assert_ne!(coherent, proof, "equality detector plant");
    install(&work.0, &[(&pl, &coherent)]);
    checks.output(
        &native(&work.0, &["v1", "aggregate-check", "m.tsv"]),
        &Run::ok("ace_to_pl aggregate ok 1 documents 1 obligations\n"),
        "coherent alternative remains derivable",
    );
    let wrong = proof.replacen(head, "na,exactly,99)", 1);
    install(&work.0, &[(&pl, &wrong)]);
    let rejected = native(&work.0, &["v1", "aggregate-check", "m.tsv"]);
    checks.require(
        rejected.rc == 1
            && rejected.stdout.is_empty()
            && rejected.stderr.contains("obligation_failed("),
        format!("wrong head must fail replay: {}", rejected.summary()),
    );
    checks.finish("P4/payload plants");
}
#[test]
fn p4_every_compiled_document_replays() {
    let work = Scratch::new("per-document-replay");
    let mut checks = Checks::default();
    for obs in results().values() {
        install(&work.0, &[(&obs.model.document(), &obs.model.proof())]);
        checks.output(
            &native(&work.0, &["v1", "aggregate-check", "m.tsv"]),
            &Run::ok(format!(
                "ace_to_pl aggregate ok 1 documents {} obligations\n",
                obs.model.groups.len()
            )),
            &format!("{}/independent model replay", obs.case.id),
        );
        checks.require(
            obs.doc.success() && obs.proof.success(),
            format!(
                "{} compile + per-document replay: {} / {}",
                obs.case.id,
                obs.doc.summary(),
                obs.proof.summary()
            ),
        );
        if obs.doc.success() && obs.proof.success() {
            install(&work.0, &[(&obs.doc.stdout, &obs.proof.stdout)]);
            checks.output(
                &native(&work.0, &["v1", "aggregate-check", "m.tsv"]),
                &Run::ok(format!(
                    "ace_to_pl aggregate ok 1 documents {} obligations\n",
                    obs.model.groups.len()
                )),
                &obs.case.id,
            );
        }
    }
    let expected: Vec<_> = results()
        .values()
        .map(|obs| (obs.model.document(), obs.model.proof()))
        .collect();
    let refs: Vec<_> = expected
        .iter()
        .map(|(pl, proof)| (pl.as_str(), proof.as_str()))
        .collect();
    let obligations: usize = results().values().map(|obs| obs.model.groups.len()).sum();
    let aggregate = Run::ok(format!(
        "ace_to_pl aggregate ok {} documents {obligations} obligations\n",
        refs.len()
    ));
    install(&work.0, &refs);
    checks.output(
        &native(&work.0, &["v1", "aggregate-check", "m.tsv"]),
        &aggregate,
        "all independent formula documents co-loaded",
    );
    if results()
        .values()
        .all(|obs| obs.doc.success() && obs.proof.success())
    {
        let actual: Vec<_> = results()
            .values()
            .map(|obs| (obs.doc.stdout.as_str(), obs.proof.stdout.as_str()))
            .collect();
        install(&work.0, &actual);
        checks.output(
            &native(&work.0, &["v1", "aggregate-check", "m.tsv"]),
            &aggregate,
            "all compiled formula documents co-loaded",
        );
    }
    checks.finish("P4/P7 aggregate replay");
}
fn patient(noun: &str, cmp: &str, value: Rat) -> String {
    let clauses = [
        "guideline_entity(actual,dosage,'opioid-dosage',countable)".into(),
        "guideline_cardinality(actual,dosage,na,eq,1)".into(),
        format!("guideline_entity(actual,amount,{},countable)", quote(noun)),
        format!(
            "guideline_cardinality(actual,amount,na,{cmp},{})",
            value.term().render()
        ),
        "guideline_event(actual,contains,contain)".into(),
        "guideline_arg(actual,contains,1,dosage)".into(),
        "guideline_arg(actual,contains,2,amount)".into(),
    ]
    .into_iter()
    .map(|s: String| format!("{s}.\n"))
    .collect::<String>();
    model::document("patient", "A patient quantity.\n", None, None, &clauses)
}
fn compile_case(case: &Case) -> (Run, Product) {
    let compiler = compiler();
    let work = Scratch::new("engine-compile");
    let ulex = work.0.join("lexicon.ulex");
    write(&ulex, ULEX);
    let dump = compiler.dump(&work.0, case.ace.as_bytes(), Some(&ulex), None);
    dump.require_ok(&case.id);
    let model = Product::from_dump(case, &dump.stdout);
    (
        compiler.compile(
            &work.0,
            &case.id,
            case.ace.as_bytes(),
            Some(&ulex),
            None,
            "doc",
        ),
        model,
    )
}
fn end_to_end(noun: &str, plural: &str, factor: &str, cmp: &str, input: &str, threshold: bool) {
    let case = Case::new(
        "conversion",
        cases::EXAMPLE
            .replace("oxycodone-milligrams-per-day", plural)
            .replace("1.5", factor),
        "engine",
    );
    let (compiled, _) = compile_case(&case);
    let threshold_case = Case::new(
        "threshold",
        "If an opioid-dosage includes at least 50 MME-units-per-day then the opioid-dosage sleeps.",
        "engine",
    );
    let (threshold_pl, _) = compile_case(&threshold_case);
    let mut checks = Checks::default();
    checks.require(
        compiled.success(),
        format!("conversion compilation: {}", compiled.summary()),
    );
    checks.require(
        threshold_pl.success(),
        format!("threshold compilation: {}", threshold_pl.summary()),
    );
    if compiled.success() && threshold_pl.success() {
        let work = Scratch::new("engine-world");
        let patient = patient(noun, cmp, Rat::decimal(input));
        install(
            &work.0,
            &[
                (&patient, ""),
                (&compiled.stdout, ""),
                (&threshold_pl.stdout, ""),
            ],
        );
        let expected = Rat::decimal(input).mul(Rat::decimal(factor));
        let count_goal = conjunction(&[
            c(
                "guideline_entity",
                vec![
                    a("actual"),
                    v("quantity"),
                    a("MME-unit-per-day"),
                    a("countable"),
                ],
            ),
            c(
                "guideline_cardinality",
                vec![a("actual"), v("quantity"), a("na"), a(cmp), expected.term()],
            ),
        ]);
        check_query(
            &mut checks,
            &work.0,
            &count_goal,
            true,
            "computed count and comparison",
        );
        let trace = native(&work.0, &["v1", "trace", "m.tsv", "query.pl", "answers.pl"]);
        checks.require(trace.success(), format!("trace: {}", trace.summary()));
        let leaf = format!(
            "builtin(guideline_product({cmp},{},exactly,{}, {cmp},{}))",
            Rat::decimal(input).term().render(),
            Rat::decimal(factor).term().render(),
            expected.term().render()
        )
        .replace(", ", ",");
        checks.require(
            trace.stdout.contains(&leaf),
            format!("trace lacks {leaf}: {}", trace.summary()),
        );
        if trace.success() {
            write(work.0.join("traces.pl"), &trace.stdout);
            let checked = native(
                &work.0,
                &[
                    "v1",
                    "trace-check",
                    "m.tsv",
                    "query.pl",
                    "answers.pl",
                    "traces.pl",
                ],
            );
            checks.require(
                checked.success()
                    && checked
                        .stdout
                        .starts_with("ckc: trace-check ok arithmetic nodes="),
                format!("trace replay: {}", checked.summary()),
            );
        }
        let threshold_goal = conjunction(&[
            c("guideline_event", vec![a("actual"), v("event"), a("sleep")]),
            c(
                "guideline_arg",
                vec![a("actual"), v("event"), a(1), a("dosage")],
            ),
        ]);
        check_query(
            &mut checks,
            &work.0,
            &threshold_goal,
            threshold,
            "50 MME threshold",
        );
    }
    checks.finish(&format!("P6/{noun}/{cmp}/{input}"));
}
#[test]
fn p6_oxycodone_exact_40_to_60_and_threshold() {
    end_to_end(
        "oxycodone-milligram-per-day",
        "oxycodone-milligrams-per-day",
        "1.5",
        "exactly",
        "40",
        true,
    );
}
#[test]
fn p6_oxycodone_lower_bound_40_to_60() {
    end_to_end(
        "oxycodone-milligram-per-day",
        "oxycodone-milligrams-per-day",
        "1.5",
        "geq",
        "40",
        true,
    );
}
#[test]
fn p6_codeine_fraction_below_threshold() {
    end_to_end(
        "codeine-milligram-per-day",
        "codeine-milligrams-per-day",
        "0.15",
        "exactly",
        "333.3",
        false,
    );
}
#[test]
fn p6_fentanyl_micrograms_per_hour_factor() {
    end_to_end(
        "fentanyl-microgram-per-hour",
        "fentanyl-micrograms-per-hour",
        "2.4",
        "exactly",
        "25",
        true,
    );
}
