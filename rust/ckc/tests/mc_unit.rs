// Contract mc core: diff-blind black-box suite. Semantic oracle = the frozen SB oracle + the
// contract-derived joins/certificate/question reference in mc-unit/support.rs.
#[path = "sb-oracle/domain.rs"]
mod domain;
#[path = "support/prop.rs"]
mod harness;
#[path = "sb-oracle/model.rs"]
mod model;
#[path = "sb-oracle/numbers.rs"]
mod numbers;
#[path = "sb-oracle/oracle.rs"]
mod oracle;
#[path = "mc-unit/properties.rs"]
mod properties;
#[path = "mc-unit/support.rs"]
mod support;

use ckc_kernel::ESrc;
use model::{BuiltinKind, Goal, Predicate, Term, atom, clause, literal, number};
use proptest::prelude::*;
use std::collections::BTreeSet;
use support::{
    Case, Claim, Input, Item, Observed, Stage, good, pos, query, raw_case, refusal, unary, verified,
};

fn entity(p: Predicate, value: &str) -> String {
    unary(p, atom(value)).render("unused")
}
fn add(case: &mut Case, fact: &str, clause: usize, level: u64, valuation: Vec<(usize, String)>) {
    assert!(!case.model.iter().any(|f| f == fact));
    case.model.push(fact.to_owned());
    case.claims.push(Claim {
        fact: fact.to_owned(),
        clause,
        valuation,
        level,
    });
}
fn chain() -> (model::Program, Case) {
    good(vec![
        clause(unary(Predicate::P, atom("a")), vec![]),
        clause(
            unary(Predicate::Q, Term::Var(0)),
            vec![pos(Predicate::P, Term::Var(0))],
        ),
        clause(
            unary(Predicate::R, Term::Var(0)),
            vec![pos(Predicate::Q, Term::Var(0))],
        ),
    ])
}

#[test]
fn founded_model_with_metadata_and_ground_valuations_verifies() {
    verified(&chain().1);
}
#[test]
fn p3_level_must_strictly_exceed_each_positive_support() {
    let (_, mut c) = chain();
    let q = c.position(&entity(Predicate::Q, "a"));
    let r = c.position(&entity(Predicate::R, "a"));
    c.claims[r].level = c.claims[q].level;
    refusal(&c, Stage::Claim, &[Item::Claim(r)]);
}
#[test]
fn p3_rule_without_positive_literals_still_needs_a_positive_level() {
    let (_, mut c) = good(vec![clause(
        unary(Predicate::R, atom("a")),
        vec![Goal::Builtin(
            BuiltinKind::Satisfies4,
            vec![atom("exactly"), number(1), atom("geq"), number(0)],
        )],
    )]);
    let i = c.position(&entity(Predicate::R, "a"));
    c.claims[i].level = 0;
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_fact_support_requires_level_zero() {
    let (_, mut c) = chain();
    let i = c.position(&entity(Predicate::P, "a"));
    c.claims[i].level = 1;
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_clause_id_out_of_range_is_a_claim_refusal() {
    let (_, mut c) = chain();
    c.claims[0].clause = c.all_ids().len();
    refusal(&c, Stage::Claim, &[Item::Claim(0)]);
}
#[test]
fn p3_original_clause_outside_slice_is_a_claim_refusal() {
    let (_, mut c) = chain();
    let i = c.position(&entity(Predicate::R, "a"));
    c.slice = Some(
        c.all_ids()
            .into_iter()
            .filter(|id| *id != c.claims[i].clause)
            .collect(),
    );
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_ground_valuation_must_instantiate_the_claimed_head() {
    let (_, mut c) = chain();
    let i = c.position(&entity(Predicate::Q, "a"));
    assert_eq!(c.claims[i].valuation, vec![(0, "a".into())]);
    c.claims[i].valuation[0].1 = "b".into();
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_valuation_must_bind_every_nonlocal_variable() {
    let (_, mut c) = chain();
    let i = c.position(&entity(Predicate::Q, "a"));
    c.claims[i].valuation.clear();
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_correlated_naf_witness_invalidates_a_claim() {
    let (p, mut c) = good(vec![
        clause(literal(Predicate::E, vec![atom("a"), atom("b")]), vec![]),
        clause(unary(Predicate::Q, atom("b")), vec![]),
        clause(
            unary(Predicate::R, Term::Var(0)),
            vec![
                pos(Predicate::D, Term::Var(0)),
                Goal::Negative(vec![
                    Goal::Positive(literal(Predicate::E, vec![Term::Var(0), Term::Var(2)])),
                    pos(Predicate::Q, Term::Var(2)),
                ]),
            ],
        ),
    ]);
    let i = c.claims.len();
    add(
        &mut c,
        &entity(Predicate::R, "a"),
        p.clauses.len() - 1,
        1,
        vec![(0, "a".into())],
    );
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_separate_naf_substitutions_are_not_one_correlated_witness() {
    let (_, c) = good(vec![
        clause(literal(Predicate::E, vec![atom("a"), atom("a")]), vec![]),
        clause(unary(Predicate::Q, atom("b")), vec![]),
        clause(
            unary(Predicate::R, atom("a")),
            vec![Goal::Negative(vec![
                Goal::Positive(literal(Predicate::E, vec![atom("a"), Term::Var(2)])),
                pos(Predicate::Q, Term::Var(2)),
            ])],
        ),
    ]);
    assert!(c.model.contains(&entity(Predicate::R, "a")));
    verified(&c);
}
#[test]
fn p3_naf_can_compute_a_local_output_absent_from_the_model() {
    let (p, mut c) = good(vec![clause(
        unary(Predicate::R, atom("a")),
        vec![Goal::Negative(vec![Goal::Builtin(
            BuiltinKind::Quotient,
            vec![
                atom("exactly"),
                number(1),
                atom("exactly"),
                number(2),
                Term::Var(4),
                Term::Var(5),
            ],
        )])],
    )]);
    let i = c.claims.len();
    add(
        &mut c,
        &entity(Predicate::R, "a"),
        p.clauses.len() - 1,
        1,
        vec![],
    );
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_builtins_are_rechecked_for_each_claim() {
    let (p, mut c) = good(vec![clause(
        unary(Predicate::R, atom("a")),
        vec![Goal::Builtin(
            BuiltinKind::Satisfies4,
            vec![atom("exactly"), number(1), atom("geq"), number(2)],
        )],
    )]);
    let i = c.claims.len();
    add(
        &mut c,
        &entity(Predicate::R, "a"),
        p.clauses.len() - 1,
        1,
        vec![],
    );
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_extra_fact_with_exact_coverage_still_needs_support() {
    let (_, mut c) = chain();
    let i = c.claims.len();
    add(&mut c, &entity(Predicate::F, "invented"), 0, 0, vec![]);
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_missing_enabled_head_is_a_closure_refusal() {
    let (p, mut c) = good(vec![clause(
        unary(Predicate::R, Term::Var(0)),
        vec![pos(Predicate::D, Term::Var(0))],
    )]);
    let ra = entity(Predicate::R, "a");
    c.remove(&ra);
    refusal(
        &c,
        Stage::Closure,
        &[Item::Derivable(p.clauses.len() - 1, ra)],
    );
}
#[test]
fn p3_positive_self_support_cannot_replace_a_founded_derivation() {
    let (p, mut c) = good(vec![clause(
        unary(Predicate::P, atom("a")),
        vec![pos(Predicate::P, atom("a"))],
    )]);
    let i = c.claims.len();
    add(
        &mut c,
        &entity(Predicate::P, "a"),
        p.clauses.len() - 1,
        1,
        vec![],
    );
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
#[test]
fn p3_wrong_root_identity_precedes_certificate_checks() {
    let (_, mut c) = chain();
    c.cert_id = b"other-root".to_vec();
    refusal(&c, Stage::Identity, &[Item::Composition]);
}
#[test]
fn p3_duplicate_claim_names_its_later_position() {
    let (_, mut c) = chain();
    let later = c.claims.len();
    c.claims.push(c.claims[0].clone());
    refusal(&c, Stage::Coverage, &[Item::Claim(later)]);
}
#[test]
fn p3_unclaimed_model_fact_is_a_coverage_refusal() {
    let (_, mut c) = chain();
    let missing = c.claims.remove(0).fact;
    refusal(&c, Stage::Coverage, &[Item::Fact(missing)]);
}
#[test]
fn p3_claim_outside_model_is_a_coverage_refusal() {
    let (_, mut c) = chain();
    let i = c.claims.len();
    c.claims
        .push(Claim::fact(&entity(Predicate::F, "absent"), 0));
    refusal(&c, Stage::Coverage, &[Item::Claim(i)]);
}
#[test]
fn p3_omitted_prefix_and_gaps_keep_original_ids_and_root_identity() {
    let p = support::program(vec![
        clause(unary(Predicate::P, atom("a")), vec![]),
        clause(unary(Predicate::Q, atom("discard_one")), vec![]),
        clause(
            unary(Predicate::R, atom("a")),
            vec![pos(Predicate::P, atom("a"))],
        ),
        clause(unary(Predicate::F, atom("discard_two")), vec![]),
        clause(
            unary(Predicate::G, atom("a")),
            vec![pos(Predicate::R, atom("a"))],
        ),
    ]);
    let m = [Predicate::P, Predicate::R, Predicate::G]
        .map(|pred| unary(pred, atom("a")))
        .into_iter()
        .collect();
    let mut c = Case::new(&p, &m, false, 1);
    c.slice = Some(vec![9, 5, 7, 7, usize::MAX]);
    assert_eq!(
        c.claims.iter().map(|c| c.clause).collect::<BTreeSet<_>>(),
        BTreeSet::from([5, 7, 9])
    );
    verified(&c);
    c.cert_id = b"slice-renamed-root".to_vec();
    refusal(&c, Stage::Identity, &[Item::Composition]);
}
#[test]
fn p3_empty_selection_has_a_complete_empty_model() {
    let (_, mut c) = chain();
    c.slice = Some(vec![]);
    c.model.clear();
    c.claims.clear();
    verified(&c);
}

#[test]
fn p5_unsafe_head_variable_names_its_document_and_clause() {
    let c = raw_case(&[
        "guideline_entity(actual,X,p,countable) :- guideline_entity(actual,a,q,countable)",
    ]);
    refusal(&c, Stage::Admission, &[Item::Clause(0, 2)]);
}
#[test]
fn p5_builtin_input_requires_an_earlier_binding() {
    let c = raw_case(&[
        "guideline_cardinality(actual,result,na,exactly,Y) :- guideline_product(exactly,X,exactly,1,exactly,Y), guideline_entity(actual,X,p,countable)",
    ]);
    refusal(&c, Stage::Admission, &[Item::Clause(0, 2)]);
}
#[test]
fn p5_naf_outer_variable_requires_an_earlier_binding() {
    let c = raw_case(&[
        "guideline_operator(actual,X,should) :- \\+ guideline_entity(actual,X,q,countable), guideline_entity(actual,X,p,countable)",
    ]);
    refusal(&c, Stage::Admission, &[Item::Clause(0, 2)]);
}
#[test]
fn p5_naf_local_builtin_input_requires_a_local_binding() {
    let c = raw_case(&[
        "guideline_operator(actual,a,should) :- \\+ guideline_satisfies(exactly,X,geq,0)",
    ]);
    refusal(&c, Stage::Admission, &[Item::Clause(0, 2)]);
}
#[test]
fn p5_foreign_head_constructor_from_acceptance_is_refused() {
    let text = include_str!("../../../tests/mc-unit/foreign-head.clauses.in");
    let lines = text
        .lines()
        .filter(|s| !s.starts_with('%') && !s.is_empty())
        .collect::<Vec<_>>();
    let c = raw_case(&lines);
    refusal(&c, Stage::Admission, &[Item::Clause(0, 2)]);
}
#[test]
fn p5_patient_document_cannot_add_a_rule() {
    let mut c = raw_case(&["guideline_entity(actual,a,p,countable)"]);
    c.docs.push(support::raw_document(
        "patient",
        &["guideline_operator(actual,X,should) :- guideline_entity(actual,X,p,countable)"],
    ));
    refusal(&c, Stage::Admission, &[Item::Clause(1, 2)]);
}
#[test]
fn p5_patient_identity_cannot_reuse_a_namespace_only_an_identity_mentions() {
    let mut c = raw_case(&[
        "guideline_entity(actual,'$guideline_id'(product,hidden,1,ref(1),[]),p,countable)",
    ]);
    c.facts.push(
        b"guideline_entity(actual,'$guideline_id'(product,hidden,1,ref(2),[]),patient,countable)"
            .to_vec(),
    );
    refusal(&c, Stage::Admission, &[Item::Clause(1, 0)]);
}
#[test]
fn p5_patient_bare_atom_cannot_equal_a_knowledge_docid() {
    let mut c = raw_case(&["guideline_entity(actual,a,p,countable)"]);
    c.facts
        .push(b"guideline_entity(actual,'mc-raw',patient,countable)".to_vec());
    refusal(&c, Stage::Admission, &[Item::Clause(1, 0)]);
}
#[test]
fn p5_patient_cannot_supply_a_knowledge_document_record() {
    let mut c = raw_case(&["guideline_entity(actual,a,p,countable)"]);
    c.facts.push(
        format!(
            "guideline_document('mc-raw',ace_sha256('{}'),ulex(none),vocabulary(none))",
            "0".repeat(64)
        )
        .into_bytes(),
    );
    refusal(&c, Stage::Admission, &[Item::Clause(1, 0)]);
}
#[test]
fn p5_namespace_collection_visits_heads_goals_builtins_and_negations() {
    for line in [
        "guideline_entity(actual,'$guideline_id'(product,hidden,1,ref(1),[]),p,countable)",
        "guideline_operator(actual,a,should) :- guideline_entity(actual,'$guideline_id'(product,hidden,1,ref(1),[]),p,countable)",
        "guideline_operator(actual,a,should) :- guideline_satisfies(exactly,'$guideline_id'(product,hidden,1,ref(1),[]),geq,0)",
        "guideline_operator(actual,a,should) :- \\+ guideline_entity(actual,'$guideline_id'(product,hidden,1,ref(1),[]),p,countable)",
    ] {
        let mut c = raw_case(&[line]);
        c.facts
            .push(b"guideline_entity(actual,hidden,patient,countable)".to_vec());
        refusal(&c, Stage::Admission, &[Item::Clause(1, 0)]);
    }
}
#[test]
fn p5_patient_identity_dependencies_must_be_empty() {
    let mut c = raw_case(&["guideline_entity(actual,a,p,countable)"]);
    c.facts.push(
        b"guideline_entity(actual,'$guideline_id'(product,patient,1,ref(1),[a]),patient,countable)"
            .to_vec(),
    );
    refusal(&c, Stage::Admission, &[Item::Clause(1, 0)]);
}
#[test]
fn p5_question_unbound_arithmetic_input_is_a_question_refusal() {
    let (_, c) = chain();
    let q = ESrc::Bytes(support::query_text(
        "guideline_product(exactly,X,exactly,1,exactly,Y)",
        "[answer(X,wh(what)),answer(Y,wh(what))]",
    ));
    let got = c.answer(q);
    assert!(
        matches!(
            got,
            Observed::Refused(Stage::Question, Item::Goal(1) | Item::Row(1))
        ),
        "{got:?}"
    );
}
#[test]
fn p5_question_unbound_projection_is_numbered_by_answer_row() {
    let (_, c) = chain();
    // X occurs in the goal, so custody accepts it, but it is only an unbound input.
    let q = ESrc::Bytes(support::query_text(
        "guideline_satisfies(exactly,X,geq,0)",
        "[answer(X,wh(what))]",
    ));
    let got = c.answer(q);
    assert!(
        matches!(
            got,
            Observed::Refused(Stage::Question, Item::Goal(1) | Item::Row(1))
        ),
        "{got:?}"
    );
}
#[test]
fn p5_question_naf_is_custodys_goal_foreign_not_a_question_stage_item() {
    let (_, c) = chain();
    let q = ESrc::Bytes(support::query_text(
        "\\+(guideline_entity(actual,X,p,countable))",
        "[answer(X,wh(what))]",
    ));
    match c.answer(q) {
        Observed::Input(Input::Query(2, out, err)) => {
            assert!(out.is_empty());
            assert!(String::from_utf8_lossy(&err).contains("goal_foreign"));
        }
        got => panic!("expected custody goal_foreign, got {got:?}"),
    }
}
#[test]
fn p5_question_projection_variable_absent_from_goal_stays_a_custody_reject() {
    let (_, c) = chain();
    let q = ESrc::Bytes(support::query_text(
        "guideline_entity(actual,a,p,countable)",
        "[answer(X,wh(what))]",
    ));
    match c.answer(q) {
        Observed::Input(Input::Query(2, out, err)) => {
            assert!(out.is_empty());
            assert!(String::from_utf8_lossy(&err).contains("answer_var(1,absent)"));
        }
        got => panic!("expected custody absent row, got {got:?}"),
    }
}
#[test]
fn p5_committed_corpus_passes_admission_before_identity() {
    let root = support::repo().join("guidelines");
    let mut docs = vec![];
    for guideline in std::fs::read_dir(root).unwrap() {
        let pl = guideline.unwrap().path().join("pl");
        if !pl.is_dir() {
            continue;
        }
        let mut paths = std::fs::read_dir(pl)
            .unwrap()
            .map(|p| p.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x == "pl"))
            .collect::<Vec<_>>();
        paths.sort();
        for path in paths {
            docs.push(std::fs::read(path).unwrap());
        }
    }
    assert!(!docs.is_empty());
    let n = docs
        .iter()
        .map(|d| support::clause_lines(d).len())
        .sum::<usize>();
    assert_eq!(n, 18_845, "contract mc D3 committed admission census");
    let c = Case {
        patient: docs.len(),
        docs,
        facts: vec![],
        identity: support::ROOT_ID.to_vec(),
        slice: None,
        model: vec![],
        cert_id: b"wrong-root".to_vec(),
        claims: vec![],
    };
    refusal(&c, Stage::Identity, &[Item::Composition]);
}
#[test]
fn p5_four_consumer_worlds_admit_through_the_patient_fact_api() {
    for path in [
        "tests/consumer/adult.pl.in",
        "tests/consumer/minor.pl.in",
        "tests/consumer/cancer.pl.in",
        "tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl",
    ] {
        let (_, mut c) = good(vec![]);
        let bytes = std::fs::read(support::repo().join(path)).unwrap();
        let facts = support::clause_lines(&bytes);
        assert!(facts.len() > 10);
        assert!(facts.iter().all(|s| !s.contains(" :- ")));
        let start = c.all_ids().len();
        for (i, f) in facts.iter().enumerate() {
            c.facts.push(f.as_bytes().to_vec());
            if !c.model.contains(f) {
                c.model.push(f.clone());
                c.claims.push(Claim::fact(f, start + i));
            }
        }
        verified(&c);
    }
}
#[test]
fn p5_compiled_patient_document_and_direct_facts_share_the_same_boundary() {
    let (_, mut c) = good(vec![]);
    let doc = support::raw_document(
        "patient",
        &[
            "guideline_entity(actual,'$guideline_id'(product,patient,1,ref(1),[]),patient,countable)",
        ],
    );
    let start = c.all_ids().len();
    for (i, f) in support::clause_lines(&doc).into_iter().enumerate() {
        if !c.model.contains(&f) {
            c.model.push(f.clone());
            c.claims.push(Claim::fact(&f, start + i));
        }
    }
    c.docs.push(doc);
    verified(&c);
}

#[test]
fn d2_g1_shape_plants_are_admission_refusals() {
    for term in [
        "rational(2,4)",
        "rational(1,1)",
        "rational(0,2)",
        "rational(1,-2)",
        "'$guideline_id'(product,d,1,ref(1),[a])",
        "'$guideline_id'(product,d,1,wrong(1),[])",
        "ace_sha256(a)",
        "arbitrary(a)",
    ] {
        let line = format!("guideline_entity(actual,{term},p,countable)");
        let c = raw_case(&[&line]);
        refusal(&c, Stage::Admission, &[Item::Clause(0, 2)]);
    }
}
#[test]
fn d2_ground_nested_identities_are_valid_model_values_and_valuations() {
    let x = Term::Var(0);
    let (p, c) = good(vec![
        clause(unary(Predicate::P, atom("a")), vec![]),
        clause(
            unary(Predicate::F, model::identity(1, vec![x.clone()])),
            vec![pos(Predicate::P, x.clone())],
        ),
        clause(
            unary(Predicate::G, model::identity(2, vec![x.clone()])),
            vec![pos(Predicate::F, x)],
        ),
    ]);
    assert!(
        c.model
            .iter()
            .any(|s| s.contains("[\u{27}$guideline_id\u{27}")),
        "{}",
        p.render()
    );
    verified(&c);
}
#[test]
fn d2_generated_canonical_rationals_and_separated_patient_atoms_admit() {
    harness::check(48, (-64_i32..65, 1_i32..17, 0_u8..16), |(n, d, label)| {
        let r = numbers::Rational::new(i128::from(n), i128::from(d));
        let (_, mut c) = good(vec![]);
        let fact = format!(
            "guideline_cardinality(actual,patient_{label},na,exactly,{})",
            r.render()
        );
        let id = c.all_ids().len();
        c.facts.push(fact.as_bytes().to_vec());
        c.model.push(fact.clone());
        c.claims.push(Claim::fact(&fact, id));
        prop_assert_eq!(c.check(), Observed::Verified);
        Ok(())
    });
}
#[test]
fn d3_generated_safety_plants_keep_the_original_clause_location() {
    harness::check(32, (0_usize..4, 0_usize..4), |(doc, prefix)| {
        let mut c = raw_case(&["guideline_entity(actual,a,p,countable)"]);
        for i in 1..=doc {
            c.docs.push(support::raw_document(
                &format!("safe-{i}"),
                &["guideline_entity(actual,a,q,countable)"],
            ));
        }
        let mut lines = vec!["guideline_entity(actual,a,p,countable)"; prefix];
        lines.push("guideline_operator(actual,X,should) :- guideline_entity(actual,a,q,countable)");
        c.docs[doc] = support::raw_document(&format!("bad-{doc}"), &lines);
        c.patient = c.docs.len();
        prop_assert_eq!(
            c.check(),
            Observed::Refused(Stage::Admission, Item::Clause(doc, prefix + 2))
        );
        Ok(())
    });
}

#[test]
fn d9_stage_precedence_is_admission_question_identity_coverage_claim_closure() {
    // Each pair contains two independent defects; only decoded inputs enter this stage law.
    let mut admission = raw_case(&[
        "guideline_entity(actual,X,p,countable) :- guideline_entity(actual,a,q,countable)",
    ]);
    admission.cert_id = b"wrong".to_vec();
    refusal(&admission, Stage::Admission, &[Item::Clause(0, 2)]);
    let (_, mut c) = chain();
    c.cert_id = b"wrong".to_vec();
    let q = ESrc::Bytes(support::query_text(
        "guideline_product(exactly,X,exactly,1,exactly,Y)",
        "[answer(Y,wh(what))]",
    ));
    assert_eq!(
        c.answer(q),
        Observed::Refused(Stage::Question, Item::Goal(1))
    );
    c.claims.clear();
    refusal(&c, Stage::Identity, &[Item::Composition]);
    let (_, mut c) = chain();
    c.claims[0].clause = usize::MAX;
    let missing = c.claims.remove(1).fact;
    refusal(&c, Stage::Coverage, &[Item::Fact(missing)]);
    let (_, mut c) = chain();
    let r = entity(Predicate::R, "a");
    c.remove(&r);
    c.claims[0].clause = usize::MAX;
    refusal(&c, Stage::Claim, &[Item::Claim(0)]);
}
#[test]
fn d9_two_fresh_runs_choose_the_same_member_of_each_failure_set() {
    let (_, mut c) = chain();
    c.claims[0].clause = usize::MAX;
    c.claims[1].clause = usize::MAX;
    refusal(&c, Stage::Claim, &[Item::Claim(0), Item::Claim(1)]);
    let (_, mut c) = chain();
    let a = c.claims.remove(0).fact;
    let b = c.claims.remove(0).fact;
    refusal(&c, Stage::Coverage, &[Item::Fact(a), Item::Fact(b)]);
    let (p, mut c) = good(vec![clause(
        unary(Predicate::R, Term::Var(0)),
        vec![pos(Predicate::D, Term::Var(0))],
    )]);
    let a = entity(Predicate::R, "a");
    let b = entity(Predicate::R, "b");
    c.remove(&a);
    c.remove(&b);
    refusal(
        &c,
        Stage::Closure,
        &[
            Item::Derivable(p.clauses.len() - 1, a),
            Item::Derivable(p.clauses.len() - 1, b),
        ],
    );
}
#[test]
fn d9_input_decode_precedence_is_custody_documents_facts_model_claims() {
    let (_, mut c) = chain();
    // Decode errors precede selection; keep the helper from reading the deliberately bad file.
    c.slice = Some(vec![]);
    c.docs[0] = b"not a document".to_vec();
    c.facts = vec![b"(".to_vec()];
    c.model = vec!["(".into()];
    c.claims[0].fact = "(".into();
    match c.answer(ESrc::Missing) {
        Observed::Input(Input::Query(2, out, _)) => assert!(out.is_empty()),
        got => panic!("custody precedes document decode: {got:?}"),
    }
    assert_eq!(c.check(), Observed::Input(Input::Document(0)));
    let (_, valid) = chain();
    c.docs = valid.docs;
    assert_eq!(c.check(), Observed::Input(Input::Fact(0)));
    c.facts.clear();
    assert_eq!(c.check(), Observed::Input(Input::Model(0)));
    c.model = valid.model;
    assert_eq!(c.check(), Observed::Input(Input::Claim(0)));
}
#[test]
fn d9_first_bad_item_of_each_input_sequence_is_reported() {
    let (_, mut c) = chain();
    let original = c.docs[0].clone();
    c.docs = vec![original.clone(), b"bad".to_vec(), b"also bad".to_vec()];
    c.patient = 3;
    assert_eq!(c.check(), Observed::Input(Input::Document(1)));
    c.docs = vec![original];
    c.patient = 1;
    c.facts = vec![
        b"guideline_entity(actual,patient,p,countable)".to_vec(),
        b"(".to_vec(),
        b"(".to_vec(),
    ];
    assert_eq!(c.check(), Observed::Input(Input::Fact(1)));
    c.facts.clear();
    c.model.push("(".into());
    let at = c.model.len() - 1;
    c.model.push("(".into());
    assert_eq!(c.check(), Observed::Input(Input::Model(at)));
    let (_, mut c) = chain();
    c.claims[1].valuation.push((0, "(".into()));
    c.claims[2].fact = "(".into();
    assert_eq!(c.check(), Observed::Input(Input::Claim(1)));
}
#[test]
fn d9_custody_noncanonical_and_row_errors_precede_core_outcomes() {
    let (_, mut c) = chain();
    c.cert_id = b"wrong".to_vec();
    let mut q = support::query_text("guideline_entity(actual,a,p,countable)", "[]");
    q.push(b' ');
    match c.answer(ESrc::Bytes(q)) {
        Observed::Input(Input::Query(2, out, err)) => {
            assert!(out.is_empty());
            assert!(String::from_utf8_lossy(&err).contains("noncanonical"));
        }
        got => panic!("{got:?}"),
    }
    match c.answer(ESrc::Bad(3)) {
        Observed::Input(Input::Query(2, out, _)) => assert!(out.is_empty()),
        got => panic!("{got:?}"),
    }
}
#[test]
fn d9_question_items_and_determinism_use_one_based_positions() {
    let (_, c) = chain();
    let make = || {
        ESrc::Bytes(support::query_text(
            "','(guideline_entity(actual,X,p,countable),guideline_product(exactly,Y,exactly,1,exactly,Z))",
            "[answer(X,wh(what)),answer(Y,wh(what)),answer(Z,wh(what))]",
        ))
    };
    let first = c.answer(make());
    assert_eq!(first, c.answer(make()));
    assert!(
        matches!(
            first,
            Observed::Refused(Stage::Question, Item::Goal(2) | Item::Row(2))
        ),
        "{first:?}"
    );
}
#[test]
fn mc_t3_computed_output_absent_from_model_is_projected() {
    let (p, c) = good(vec![]);
    let goals = vec![Goal::Builtin(
        BuiltinKind::Quotient,
        vec![
            atom("exactly"),
            number(1),
            atom("exactly"),
            number(2),
            Term::Var(4),
            Term::Var(5),
        ],
    )];
    assert!(!c.model.iter().any(|s| s.contains("rational(1,2)")));
    assert_eq!(
        c.answer(query(
            &goals,
            &[Term::Var(5), Term::Var(4)],
            &support::doc(&p)
        )),
        Observed::Answer(b"solutions([sol([rational(1,2),exactly])])".to_vec())
    );
}
#[test]
fn mc_t3_empty_answers_are_complete_not_indeterminate() {
    let (p, c) = good(vec![]);
    let goals = vec![pos(Predicate::P, Term::Var(0))];
    assert_eq!(
        c.answer(query(&goals, &[Term::Var(0)], &support::doc(&p))),
        Observed::Answer(b"solutions([])".to_vec())
    );
    assert_eq!(
        c.answer(query(&goals, &[], &support::doc(&p))),
        Observed::Answer(b"no(finite_failure)".to_vec())
    );
    let goals = vec![pos(Predicate::D, Term::Var(0))];
    assert_eq!(
        c.answer(query(&goals, &[], &support::doc(&p))),
        Observed::Answer(b"yes".to_vec())
    );
}
#[test]
fn mc_t3_projection_order_standard_sort_and_dedup_are_observable() {
    let (p, c) = good(vec![
        clause(literal(Predicate::E, vec![atom("z"), number(2)]), vec![]),
        clause(literal(Predicate::E, vec![atom("a"), number(10)]), vec![]),
        clause(literal(Predicate::E, vec![atom("b"), number(-1)]), vec![]),
        clause(literal(Predicate::E, vec![atom("a"), number(10)]), vec![]),
    ]);
    let g = vec![Goal::Positive(literal(
        Predicate::E,
        vec![Term::Var(0), Term::Var(1)],
    ))];
    assert_eq!(
        c.answer(query(&g, &[Term::Var(1), Term::Var(0)], &support::doc(&p))),
        Observed::Answer(b"solutions([sol([-1,b]),sol([2,z]),sol([10,a])])".to_vec())
    );
}
#[test]
fn mc_t3_bound_inputs_chained_outputs_and_comparisons_match_reference() {
    harness::check(
        48,
        (-32_i16..33, 1_i16..9, -4_i16..5, -16_i16..17),
        |(n, d, divisor, bound)| {
            let n = i128::from(n);
            let (p, c) = good(vec![
                clause(unary(Predicate::P, number(n)), vec![]),
                clause(unary(Predicate::P, number(n + 1)), vec![]),
                clause(unary(Predicate::P, number(n)), vec![]),
            ]);
            let goals = vec![
                pos(Predicate::P, Term::Var(0)),
                Goal::Builtin(
                    BuiltinKind::Product,
                    vec![
                        atom("exactly"),
                        Term::Var(0),
                        atom("exactly"),
                        Term::Number(numbers::Rational::new(1, i128::from(d))),
                        Term::Var(4),
                        Term::Var(5),
                    ],
                ),
                Goal::Builtin(
                    BuiltinKind::Quotient,
                    vec![
                        Term::Var(4),
                        Term::Var(5),
                        atom("exactly"),
                        number(i128::from(divisor)),
                        Term::Var(2),
                        Term::Var(3),
                    ],
                ),
                Goal::Builtin(
                    BuiltinKind::Satisfies4,
                    vec![
                        Term::Var(2),
                        Term::Var(3),
                        atom("greater"),
                        number(i128::from(bound)),
                    ],
                ),
            ];
            let projection = vec![Term::Var(3), Term::Var(0), Term::Var(2)];
            let m = oracle::evaluate(&p).truth;
            prop_assert_eq!(
                c.answer(query(&goals, &projection, &support::doc(&p))),
                Observed::Answer(support::answer_oracle(
                    &m,
                    &goals,
                    &projection,
                    &support::doc(&p)
                ))
            );
            Ok(())
        },
    );
}
#[test]
fn certificate_nonminimal_levels_and_alternate_founded_supports_are_accepted() {
    let (p, mut c) = good(vec![
        clause(unary(Predicate::P, atom("a")), vec![]),
        clause(unary(Predicate::Q, atom("a")), vec![]),
        clause(
            unary(Predicate::R, atom("a")),
            vec![pos(Predicate::P, atom("a"))],
        ),
        clause(
            unary(Predicate::R, atom("a")),
            vec![pos(Predicate::Q, atom("a"))],
        ),
    ]);
    let i = c.position(&entity(Predicate::R, "a"));
    c.claims[i].clause = p.clauses.len() - 1;
    c.claims[i].level = u64::MAX;
    verified(&c);
    c.claims.reverse();
    c.model.reverse();
    verified(&c);
}

// MAIN pins (not diff-blind): contract mc R4 (reviewer-11 F1). Expected values = D7 by unification:
// the one fact `p('$guideline_id'(…, ref(1), []))` answers a goal holding a variable at its Slot or
// Deps with that value, G2 or not. Claim valuations stay G2: a claim binding `ref(1)` is refused.
fn identity_answer(from: &str, to: &str, rows: &str) -> String {
    let (p, c) = good(vec![clause(
        unary(Predicate::P, model::identity(1, vec![])),
        vec![],
    )]);
    verified(&c);
    let fact = unary(Predicate::P, model::identity(1, vec![])).render(&support::doc(&p));
    let goal = fact.replacen(from, to, 1);
    assert_ne!(goal, fact);
    match c.answer(ESrc::Bytes(support::query_text(&goal, rows))) {
        Observed::Answer(bytes) => String::from_utf8(bytes).unwrap(),
        other => panic!("expected complete answers, got {other:?}"),
    }
}
#[test]
fn r4_question_binds_an_identity_slot() {
    assert_eq!(
        identity_answer("ref(1)", "S", "[answer(S,wh(what))]"),
        "solutions([sol([ref(1)])])"
    );
}
#[test]
fn r4_question_binds_a_dependency_list() {
    assert_eq!(
        identity_answer(",[])", ",D)", "[answer(D,wh(what))]"),
        "solutions([sol([[]])])"
    );
}
#[test]
fn r4_claim_valuation_outside_g2_is_a_claim_refusal() {
    // `guideline_entity(actual,ref(1),q,countable)` instantiates the head of `q(X) :- p(X)`, but
    // its `p` literal has no founded claim.
    let (_, mut c) = chain();
    let fact = entity(Predicate::Q, "a");
    let j = c.position(&fact);
    let (clause, level) = (c.claims[j].clause, c.claims[j].level);
    let bad = fact.replacen(",a,", ",ref(1),", 1);
    assert_ne!(bad, fact);
    let i = c.claims.len();
    add(&mut c, &bad, clause, level, vec![(0, "ref(1)".into())]);
    refusal(&c, Stage::Claim, &[Item::Claim(i)]);
}
