#[path = "sb-oracle/domain.rs"]
mod domain;
#[path = "sb-oracle/model.rs"]
mod model;
#[path = "sb-oracle/numbers.rs"]
mod numbers;
#[path = "sb-oracle/oracle.rs"]
mod oracle;
#[path = "sb-oracle/swi.rs"]
mod swi;

use model::{BuiltinKind, Goal, Predicate, Program, Term, atom, clause, literal, number};
use oracle::{Outcome, Statistics};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn value_bound_counts_rational_and_identity_components() {
    let mut values = BTreeSet::new();
    Term::Number(numbers::Rational::new(1, 4)).collect_values(&mut values);
    model::identity(2, vec![model::identity(1, vec![atom("a")])]).collect_values(&mut values);
    for required in [
        number(1),
        number(2),
        number(4),
        atom("product"),
        Term::Document,
        Term::Compound("ref", vec![number(1)]),
        Term::Compound("ref", vec![number(2)]),
    ] {
        assert!(
            values.contains(&required),
            "missing ground component: {required:?}"
        );
    }
}

#[test]
fn frozen_domain_counts_and_source_bounds() {
    let mut counts = BTreeMap::new();
    for (family, expected) in model::family_counts() {
        let mut count = 0;
        domain::enumerate(family, |p| {
            assert_eq!(p.index, count);
            p.assert_source_bounds();
            count += 1;
        });
        assert_eq!(count, expected, "{family}");
        counts.insert(family, count);
    }
    assert_eq!(counts.values().sum::<usize>(), 93_369);
    assert!(include_str!("sb-oracle/manifest.md").contains("93,369"));
    eprintln!("sb-oracle: frozen domain {counts:?}, total=93369");
}

#[test]
fn oracle_positive_recursion_is_the_least_fixpoint() {
    let mut p = Program::new("G", 0, &[atom("a"), atom("b")]);
    let pa = literal(Predicate::P, vec![atom("a")]);
    let qa = literal(Predicate::Q, vec![atom("a")]);
    let mut expected = p
        .clauses
        .iter()
        .map(|c| c.head.clone())
        .collect::<BTreeSet<_>>();
    expected.extend([pa.clone(), qa.clone()]);
    p.clauses.extend([
        clause(pa.clone(), vec![]),
        clause(qa.clone(), vec![Goal::Positive(pa.clone())]),
        clause(pa, vec![Goal::Positive(qa)]),
    ]);
    let result = oracle::evaluate(&p);
    assert_eq!(result.truth, expected);
    assert!(result.undefined.is_empty());
}

#[test]
fn oracle_refusal_controls_preserve_undefined() {
    let mut count = 0;
    domain::enumerate("NC", |p| {
        assert!(p.refused);
        let result = oracle::evaluate(&p);
        assert_eq!(result.undefined, p.expected_undefined, "{}", p.id());
        assert!(!result.undefined.is_empty());
        count += 1;
    });
    assert_eq!(count, 8);
}

#[test]
fn oracle_naf_finds_local_arithmetic_outputs_outside_source_terms() {
    let mut p = Program::new("A-free", 6845, &[atom("a"), atom("b")]);
    let expected = p
        .clauses
        .iter()
        .map(|c| c.head.clone())
        .collect::<BTreeSet<_>>();
    p.clauses.push(clause(
        literal(Predicate::R, vec![atom("a")]),
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
    ));
    let result = oracle::evaluate(&p);
    assert_eq!(result.truth, expected);
    assert!(result.undefined.is_empty());
}

fn maxima(max: &mut Statistics, next: &Statistics) {
    max.values = max.values.max(next.values);
    max.schema_heads = max.schema_heads.max(next.schema_heads);
    max.ground_clauses = max.ground_clauses.max(next.ground_clauses);
    max.identity_depth = max.identity_depth.max(next.identity_depth);
    max.rounds = max.rounds.max(next.rounds);
}

fn check_properties(program: &Program, result: &Outcome) {
    if program.refused {
        assert_eq!(
            result.undefined,
            program.expected_undefined,
            "{}",
            program.id()
        );
        assert!(!result.undefined.is_empty());
    } else {
        assert!(
            result.undefined.is_empty(),
            "{}: admitted undefined",
            program.id()
        );
    }
    let renamed = oracle::evaluate(&program.transform());
    assert_eq!(
        renamed.truth,
        result.truth.iter().map(model::Literal::transform).collect(),
        "{}: renaming + clause order + alpha property",
        program.id()
    );
    assert_eq!(
        renamed.undefined,
        result
            .undefined
            .iter()
            .map(model::Literal::transform)
            .collect(),
        "{}: undefined renaming property",
        program.id()
    );
}

fn planted_model(
    program: &Program,
    model: &swi::Observed,
) -> Option<(&'static str, swi::Observed)> {
    let target = match program.family {
        "V" => 4_273,
        "I" => 131,
        _ => 0,
    };
    if program.index != target {
        return None;
    }
    let mut wrong = model.clone();
    let ra = literal(Predicate::R, vec![atom("a")]).render(&program.id());
    let kind = match program.family {
        "G" => {
            assert!(wrong.truth.remove("guideline_schema_version(2)"));
            "omitted-fact"
        }
        "V" => {
            assert!(wrong.truth.remove(&ra));
            "flattened-correlated-NAF"
        }
        "C4" => {
            assert!(wrong.truth.remove(&ra));
            "inverted-inclusion"
        }
        "C6" | "A-bound" | "B-shape" => {
            assert!(wrong.truth.insert(ra));
            "false-built-in-accepted"
        }
        "A-free" | "A-partial" => {
            let old = wrong
                .truth
                .iter()
                .find(|s| s.starts_with("guideline_cardinality("))
                .unwrap()
                .clone();
            wrong.truth.remove(&old);
            let new = if program.family == "A-free" {
                assert!(old.ends_with(",exactly,4)"));
                old.replace(",exactly,4)", ",exactly,3)")
            } else {
                assert!(old.ends_with(",exactly,1)"));
                old.replace(",exactly,1)", ",geq,1)")
            };
            assert!(wrong.truth.insert(new));
            "wrong-arithmetic-output"
        }
        "I" => {
            let old = wrong
                .truth
                .iter()
                .find(|s| s.contains("'$guideline_id'") && s.contains("[a]"))
                .unwrap()
                .clone();
            wrong.truth.remove(&old);
            assert!(wrong.truth.insert(old.replacen("[a]", "[b]", 1)));
            "mismatched-Deps"
        }
        "NC" => {
            assert!(!wrong.undefined.is_empty());
            wrong.undefined.clear();
            "undefined-collapsed-to-false"
        }
        _ => unreachable!(),
    };
    Some((kind, wrong))
}

fn compare_batch(programs: &[Program], outcomes: &[Outcome], plant: bool) {
    let mut observations = swi::models(programs);
    for (program, expected) in programs.iter().zip(outcomes) {
        for mode in if program.refused {
            vec!["wfs"]
        } else {
            vec!["plain", "wfs"]
        } {
            let observed = observations
                .remove(&(program.id(), mode.to_owned()))
                .expect("model for every declared case/mode");
            let observed = if plant && mode == "wfs" {
                if let Some((kind, wrong)) = planted_model(program, &observed) {
                    eprintln!(
                        "sb-oracle: planted wrong model family={} case={} kind={kind}",
                        program.family,
                        program.id()
                    );
                    wrong
                } else {
                    observed
                }
            } else {
                observed
            };
            swi::compare(program, expected, &observed).unwrap_or_else(|e| panic!("{mode}: {e}"));
        }
    }
    assert!(observations.is_empty());
}

fn run_family(family: &'static str) {
    let mut programs = Vec::new();
    let mut outcomes = Vec::new();
    let mut count = 0;
    let mut seen = 0;
    let mut max = Statistics::default();
    let selected = std::env::var("CKC_SB_ORACLE_CASE").ok();
    let plant = std::env::var("CKC_SB_ORACLE_PLANT").is_ok_and(|s| s == "all" || s == family);
    domain::enumerate(family, |program| {
        seen += 1;
        if selected.as_ref().is_some_and(|id| *id != program.id()) {
            return;
        }
        let outcome = oracle::evaluate(&program);
        check_properties(&program, &outcome);
        maxima(&mut max, &outcome.statistics);
        programs.push(program);
        outcomes.push(outcome);
        count += 1;
        if programs.len() == 256 {
            compare_batch(&programs, &outcomes, plant);
            programs.clear();
            outcomes.clear();
        }
    });
    if !programs.is_empty() {
        compare_batch(&programs, &outcomes, plant);
    }
    assert_eq!(seen, model::family_counts()[family]);
    assert_eq!(count, if selected.is_some() { 1 } else { seen });
    eprintln!(
        "sb-oracle: {family} exact={count}/{seen} modes={} property-pairs={count} maxima={max:?}",
        if family == "NC" {
            "tnot; refused"
        } else {
            "recipe+tnot"
        }
    );
}

#[test]
fn ground_programs_match_swi() {
    run_family("G");
}
#[test]
fn variable_programs_match_swi() {
    run_family("V");
}
#[test]
fn comparison_four_match_swi() {
    run_family("C4");
}
#[test]
fn comparison_six_match_swi() {
    run_family("C6");
}
#[test]
fn arithmetic_free_match_swi() {
    run_family("A-free");
}
#[test]
fn arithmetic_bound_match_swi() {
    run_family("A-bound");
}
#[test]
fn arithmetic_partial_match_swi() {
    run_family("A-partial");
}
#[test]
fn identity_programs_match_swi() {
    run_family("I");
}
#[test]
fn bad_shapes_match_swi() {
    run_family("B-shape");
}
#[test]
fn negative_cycles_match_swi() {
    run_family("NC");
}

#[test]
fn comparison_wrong_model_plants_are_detected() {
    for family in model::family_counts().keys() {
        let mut checked = 0;
        domain::enumerate(family, |program| {
            let target = match *family {
                "V" => 4_273,
                "I" => 131,
                _ => 0,
            };
            if program.index != target {
                return;
            }
            let result = oracle::evaluate(&program);
            let observed = swi::Observed {
                truth: model::rendered_model(&result.truth, &program.id()),
                undefined: model::rendered_model(&result.undefined, &program.id()),
            };
            assert!(swi::compare(&program, &result, &observed).is_ok());
            let (kind, wrong) = planted_model(&program, &observed).unwrap();
            assert!(
                swi::compare(&program, &result, &wrong).is_err(),
                "{family}: {kind}"
            );
            checked += 1;
        });
        assert_eq!(checked, 1);
    }
}

#[test]
fn comparison_protocol_rejects_incomplete_or_ambiguous_records() {
    let valid = "B\tsb_case\twfs\nT\tguideline_schema_version(2)\nE\tsb_case\twfs\n";
    assert_eq!(swi::parse(valid).unwrap().len(), 1);
    for bad in [
        "B\tsb_case\twfs\n",
        "T\tguideline_schema_version(2)\n",
        "B\tsb_case\twfs\nE\tother\twfs\n",
        "B\tsb_case\twfs\nT\tguideline_schema_version(2)\nT\tguideline_schema_version(2)\nE\tsb_case\twfs\n",
        "B\tsb_case\twfs\nT\tguideline_schema_version(2)\nU\tguideline_schema_version(2)\nE\tsb_case\twfs\n",
        "garbage\n",
    ] {
        assert!(swi::parse(bad).is_err(), "{bad:?}");
    }
}
