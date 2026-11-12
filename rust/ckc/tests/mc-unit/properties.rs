use super::{domain, harness, model, numbers, oracle, support};
use model::{
    BuiltinKind, Clause, Goal, Literal, Predicate, Program, Term, atom, clause, literal, number,
};
use proptest::prelude::*;
use proptest::test_runner::TestCaseResult;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;
use support::{Case, Claim, Observed, answer_oracle, pos, query, unary};

type Checker = fn(&Case) -> Observed;
fn accept_all(_: &Case) -> Observed {
    Observed::Verified
}
fn succeeds(o: &Observed) -> bool {
    matches!(o, Observed::Verified | Observed::Answer(_))
}

// Each quantified implication takes its checker as an argument. The accept-all control below
// executes these very functions, not a parallel imitation of their assertions.
fn model_soundness(checker: Checker, c: &Case, expected: &BTreeSet<String>) -> TestCaseResult {
    let got = checker(c);
    if succeeds(&got) {
        prop_assert_eq!(
            c.model.iter().cloned().collect::<BTreeSet<_>>(),
            expected.clone(),
            "MC-SOUNDNESS-COUNTEREXAMPLE: verified model differs from independent oracle"
        );
        prop_assert_eq!(
            got,
            Observed::Verified,
            "check_model exposes no answer payload"
        );
    }
    Ok(())
}
fn stable_soundness(
    checker: Checker,
    c: &Case,
    p: &Program,
    m: &BTreeSet<Literal>,
) -> TestCaseResult {
    if succeeds(&checker(c)) {
        prop_assert!(
            support::stable_reference(p, m),
            "MC-STABILITY-COUNTEREXAMPLE: accepted undefined-control model is not stable"
        );
    }
    Ok(())
}
fn complete(checker: Checker, c: &Case) -> TestCaseResult {
    prop_assert_eq!(checker(c), Observed::Verified);
    Ok(())
}
fn invalid_certificate(checker: Checker, c: &Case) -> TestCaseResult {
    prop_assert!(
        matches!(checker(c), Observed::Refused(_, _) | Observed::Input(_)),
        "a finite invalid certificate must refuse, not verify or become indeterminate"
    );
    Ok(())
}
fn question_matches(
    c: &Case,
    m: &BTreeSet<Literal>,
    p: &Program,
    goals: &[Goal],
    projection: &[Term],
) -> TestCaseResult {
    prop_assert_eq!(
        c.answer(query(goals, projection, &support::doc(p))),
        Observed::Answer(answer_oracle(m, goals, projection, &support::doc(p)))
    );
    Ok(())
}

fn bad_certificate(c: &Case, selector: usize) -> Case {
    let mut out = c.clone();
    match selector % 6 {
        0 => out.cert_id.push(0),
        1 => out.claims.push(out.claims[0].clone()),
        2 => {
            out.claims.remove(0);
        }
        3 => out.claims[0].clause = usize::MAX,
        4 => out.claims[0].level = u64::MAX,
        _ => out.claims.push(Claim::fact(
            "guideline_entity(actual,certificate_extra,f,countable)",
            0,
        )),
    }
    out
}
fn wrong_model(c: &Case, selector: usize) -> Case {
    let mut out = c.clone();
    let at = selector % out.model.len();
    let missing = out.model[at].clone();
    out.remove(&missing);
    out
}
fn expected(m: &BTreeSet<Literal>, p: &Program) -> BTreeSet<String> {
    model::rendered_model(m, &support::doc(p))
}
fn operator_question() -> (Vec<Goal>, Vec<Term>) {
    (vec![pos(Predicate::R, Term::Var(0))], vec![Term::Var(0)])
}
fn arithmetic_question() -> (Vec<Goal>, Vec<Term>) {
    (
        vec![Goal::Builtin(
            BuiltinKind::Quotient,
            vec![
                atom("exactly"),
                number(1),
                atom("exactly"),
                number(3),
                Term::Var(4),
                Term::Var(5),
            ],
        )],
        vec![Term::Var(5), Term::Var(4)],
    )
}

#[derive(Default)]
struct Failures {
    count: usize,
    first: Vec<String>,
}
impl Failures {
    fn add(&mut self, id: &str, result: TestCaseResult) {
        if let Err(e) = result {
            self.count += 1;
            if self.first.len() < 12 {
                self.first.push(format!("{id}: {e}"));
            }
        }
    }
    fn assert_clear(&self, property: &str) -> TestCaseResult {
        prop_assert_eq!(
            self.count,
            0,
            "{}: {} failures; first 12 = {:?}",
            property,
            self.count,
            self.first
        );
        Ok(())
    }
}
#[derive(Default)]
struct Frozen {
    counts: BTreeMap<&'static str, usize>,
    soundness: Failures,
    model_completeness: Failures,
    certificate_completeness: Failures,
    questions: Failures,
    undefined_soundness: Failures,
    maxima: (usize, usize, usize, usize, usize),
}
static FROZEN: OnceLock<Frozen> = OnceLock::new();
fn frozen() -> &'static Frozen {
    FROZEN.get_or_init(||{
        let mut r=Frozen::default();
        let mut transported_names=BTreeSet::new();
        // One complete traversal shared by the independently asserted properties. Failure detail
        // is capped; the evaluated domain + its counters never are.
        for (family,count) in model::family_counts() {
            let mut seen=0;
            domain::enumerate(family,|p|{
                assert_eq!(p.index,seen);seen+=1;p.assert_source_bounds();
                assert!(transported_names.insert(support::doc(&p)),"transport namespace remains injective");
                let o=oracle::evaluate(&p);
                let s=&o.statistics;
                r.maxima=(r.maxima.0.max(s.values),r.maxima.1.max(s.schema_heads),r.maxima.2.max(s.ground_clauses),r.maxima.3.max(s.identity_depth),r.maxima.4.max(s.rounds));
                if family=="NC" {
                    assert!(p.refused);assert_eq!(o.undefined,p.expected_undefined);assert!(!o.undefined.is_empty());
                    let uncertain=o.undefined.iter().cloned().collect::<Vec<_>>();
                    for mask in 0..1_usize<<uncertain.len() {
                        let mut m=o.truth.clone();m.extend(uncertain.iter().enumerate().filter(|(i,_)|mask&(1<<i)!=0).map(|(_,f)|f.clone()));
                        let mut c=Case::new(&p,&o.truth,false,1);
                        if support::stable_reference(&p,&m) { c=Case::new(&p,&m,false,1); }
                        else { c.model=m.iter().map(|f|f.render(&support::doc(&p))).collect(); }
                        r.undefined_soundness.add(&format!("{} model={mask}",p.id()),stable_soundness(Case::check,&c,&p,&m));
                    }
                    return;
                }
                assert!(!p.refused);assert!(o.undefined.is_empty());
                let c=Case::new(&p,&o.truth,false,1);
                let reference=expected(&o.truth,&p);
                r.soundness.add(&p.id(),model_soundness(Case::check,&c,&reference));
                r.model_completeness.add(&p.id(),complete(Case::check,&c));
                let alternate=Case::new(&p,&o.truth,true,97);
                r.certificate_completeness.add(&p.id(),complete(Case::check,&alternate));
                let invalid=bad_certificate(&c,p.index);
                r.certificate_completeness.add(&p.id(),invalid_certificate(Case::check,&invalid));
                r.soundness.add(&p.id(),model_soundness(Case::check,&invalid,&reference));
                let wrong=wrong_model(&c,p.index);
                r.soundness.add(&p.id(),model_soundness(Case::check,&wrong,&reference));
                let mut malformed=wrong.clone();
                // Valid coverage, arbitrary support + arbitrary ground values; input-decode
                // failures join the domain through generated certificates below.
                if !malformed.claims.is_empty() {
                    malformed.claims[0].clause=p.index%(p.clauses.len()+2);
                    malformed.claims[0].valuation=vec![(p.index%7,"rational(1,2)".into())];
                    malformed.claims[0].level=(p.index%11) as u64;
                }
                r.soundness.add(&p.id(),model_soundness(Case::check,&malformed,&reference));
                let (goals,projection)=operator_question();
                r.questions.add(&p.id(),question_matches(&c,&o.truth,&p,&goals,&projection));
                let (goals,projection)=arithmetic_question();
                r.questions.add(&p.id(),question_matches(&c,&o.truth,&p,&goals,&projection));
            });
            assert_eq!(seen,count,"frozen domain {family}");r.counts.insert(family,seen);
        }
        assert_eq!(r.counts.values().sum::<usize>(),93_369);
        assert_eq!(r.counts.iter().filter(|(f,_)|**f!="NC").map(|(_,n)|n).sum::<usize>(),93_361);
        eprintln!("mc-unit: frozen exact={:?} maxima={:?}; no sampling; failure details capped at 12/property",r.counts,r.maxima);
        r
    })
}
#[test]
fn p4_frozen_model_soundness_all_certificates_tested() {
    harness::check(1, Just(()), |()| {
        frozen().soundness.assert_clear("soundness")
    });
}
#[test]
fn p4_frozen_model_completeness() {
    harness::check(1, Just(()), |()| {
        frozen()
            .model_completeness
            .assert_clear("model completeness")
    });
}
#[test]
fn p4_frozen_certificate_completeness_nonminimal_alternate_and_invalid() {
    harness::check(1, Just(()), |()| {
        frozen()
            .certificate_completeness
            .assert_clear("certificate completeness")
    });
}
#[test]
fn p4_frozen_questions_equal_independent_relational_answers() {
    harness::check(1, Just(()), |()| {
        frozen().questions.assert_clear("questions")
    });
}
#[test]
fn p4_undefined_controls_check_stability_not_an_unavailable_two_valued_hypothesis() {
    harness::check(1, Just(()), |()| {
        frozen()
            .undefined_soundness
            .assert_clear("MC-T1 on NC controls")
    });
}

fn control(label: &str, run: impl FnOnce() + std::panic::UnwindSafe) {
    let failure = std::panic::catch_unwind(run)
        .expect_err("accept-all checker must produce a counterexample on every run");
    let s = if let Some(s) = failure.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = failure.downcast_ref::<&str>() {
        (*s).to_owned()
    } else {
        "non-string counterexample".into()
    };
    let marker = if label == "undefined" {
        "MC-STABILITY-COUNTEREXAMPLE"
    } else {
        "MC-SOUNDNESS-COUNTEREXAMPLE"
    };
    assert!(
        s.contains(marker),
        "semantic counterexample required; setup panics earn no credit: {s}"
    );
    eprintln!("mc-unit: {label} accept-all counterexample detected: {s}");
}
#[test]
fn soundness_controls_refuse_a_setup_panic_as_red_evidence() {
    let failure =
        std::panic::catch_unwind(|| control("frozen", || panic!("PLANTED-SETUP-FAILURE")))
            .expect_err("the firing control must reject a setup failure");
    let message = failure.downcast_ref::<String>().expect("diagnostic string");
    assert!(message.contains("semantic counterexample required"));
}
#[test]
fn p4_frozen_soundness_accept_all_control_fires() {
    control("frozen", || {
        harness::check(1, Just(()), |()| {
            let p = Program::new("G", 0, &[atom("a"), atom("b")]);
            let m = oracle::evaluate(&p).truth;
            let c = wrong_model(&Case::new(&p, &m, false, 1), 0);
            model_soundness(accept_all, &c, &expected(&m, &p))
        })
    });
}
#[test]
fn p4_undefined_soundness_accept_all_control_fires() {
    control("undefined", || {
        harness::check(1, Just(()), |()| {
            let mut first = None;
            domain::enumerate("NC", |p| {
                if first.is_none() {
                    first = Some(p)
                }
            });
            let p = first.unwrap();
            let m = oracle::evaluate(&p).truth;
            let c = Case::new(&p, &m, false, 1);
            stable_soundness(accept_all, &c, &p, &m)
        })
    });
}

// Six generated shapes complement, rather than sample or replace, the frozen manifest.
// Their ground constants overlap across rules; their recursive edges are positive; arithmetic
// has no feedback; identities follow a constructor DAG.
type Generated = (u8, u8, i16, u8);
fn generated_strategy() -> impl Strategy<Value = Generated> {
    (any::<u8>(), 0_u8..6, -8_i16..9, 1_u8..6)
}
fn generated(input: Generated) -> Program {
    let (mask, shape, n, d) = input;
    let mut p = Program::new("generated", 0, &[atom("a"), atom("b")]);
    let facts = [
        unary(Predicate::P, atom("a")),
        unary(Predicate::P, atom("b")),
        unary(Predicate::Q, atom("a")),
        unary(Predicate::Q, atom("b")),
        literal(Predicate::E, vec![atom("a"), atom("b")]),
        literal(Predicate::E, vec![atom("b"), atom("a")]),
    ];
    p.clauses.extend(
        facts
            .into_iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, f)| clause(f, vec![])),
    );
    let (x, y, l, k) = (Term::Var(0), Term::Var(1), Term::Var(2), Term::Var(3));
    let e = |a: Term, b: Term| Goal::Positive(literal(Predicate::E, vec![a, b]));
    let r = |body| clause(unary(Predicate::R, x.clone()), body);
    let rules: Vec<Clause> = match shape {
        0 => vec![
            clause(
                unary(Predicate::Q, x.clone()),
                vec![pos(Predicate::P, x.clone())],
            ),
            clause(
                unary(Predicate::P, x.clone()),
                vec![pos(Predicate::Q, x.clone())],
            ),
            r(vec![pos(Predicate::P, x.clone())]),
        ],
        1 => vec![
            r(vec![
                pos(Predicate::D, x.clone()),
                Goal::Negative(vec![e(x.clone(), l.clone()), pos(Predicate::Q, l.clone())]),
            ]),
            clause(
                unary(Predicate::F, x.clone()),
                vec![pos(Predicate::P, x.clone())],
            ),
            clause(
                unary(Predicate::G, x.clone()),
                vec![pos(Predicate::F, x.clone())],
            ),
        ],
        2 => vec![
            r(vec![pos(Predicate::P, x.clone())]),
            r(vec![pos(Predicate::Q, x.clone())]),
            clause(
                unary(Predicate::Q, x.clone()),
                vec![pos(Predicate::P, x.clone())],
            ),
        ],
        3 => vec![
            clause(
                literal(Predicate::V, vec![Term::Var(4), Term::Var(5)]),
                vec![Goal::Builtin(
                    BuiltinKind::Product,
                    vec![
                        atom("exactly"),
                        number(i128::from(n)),
                        atom("exactly"),
                        Term::Number(numbers::Rational::new(1, i128::from(d))),
                        Term::Var(4),
                        Term::Var(5),
                    ],
                )],
            ),
            r(vec![
                pos(Predicate::D, x.clone()),
                Goal::Negative(vec![Goal::Builtin(
                    BuiltinKind::Quotient,
                    vec![
                        atom("exactly"),
                        number(i128::from(n)),
                        atom("exactly"),
                        number(i128::from(d)),
                        Term::Var(4),
                        Term::Var(5),
                    ],
                )]),
            ]),
            clause(
                unary(Predicate::F, x.clone()),
                vec![pos(Predicate::P, x.clone())],
            ),
        ],
        4 => vec![
            clause(
                unary(Predicate::F, model::identity(1, vec![x.clone()])),
                vec![pos(Predicate::P, x.clone())],
            ),
            clause(
                unary(Predicate::G, model::identity(2, vec![x.clone()])),
                vec![pos(Predicate::F, x.clone())],
            ),
            r(vec![
                pos(Predicate::D, x.clone()),
                Goal::Negative(vec![
                    pos(Predicate::F, model::identity(1, vec![l.clone()])),
                    e(x.clone(), l.clone()),
                ]),
            ]),
        ],
        _ => vec![
            r(vec![
                pos(Predicate::D, x.clone()),
                e(x.clone(), y.clone()),
                Goal::Negative(vec![e(y, l.clone()), pos(Predicate::Q, l)]),
                Goal::Negative(vec![e(x.clone(), k.clone()), pos(Predicate::P, k)]),
            ]),
            clause(
                unary(Predicate::F, x.clone()),
                vec![pos(Predicate::P, x.clone())],
            ),
            clause(unary(Predicate::G, x.clone()), vec![pos(Predicate::Q, x)]),
        ],
    };
    p.clauses.extend(rules);
    p
}
fn generated_completeness(input: Generated, alternate: bool, stretch: u64) -> TestCaseResult {
    let p = generated(input);
    let o = oracle::evaluate(&p);
    prop_assert!(o.undefined.is_empty());
    complete(Case::check, &Case::new(&p, &o.truth, alternate, stretch))
}
type ArbitraryClaim = (u8, u8, u64, u8, Vec<(u8, u8)>);
fn arbitrary_strategy() -> impl Strategy<Value = (Generated, u64, u8, Vec<ArbitraryClaim>)> {
    (
        generated_strategy(),
        any::<u64>(),
        0_u8..6,
        proptest::collection::vec(
            (
                any::<u8>(),
                any::<u8>(),
                any::<u64>(),
                0_u8..4,
                proptest::collection::vec((0_u8..8, 0_u8..8), 0..5),
            ),
            0..12,
        ),
    )
}
fn arbitrary_case(base: &Case, mask: u64, flavor: u8, claims: &[ArbitraryClaim]) -> Case {
    let mut c = base.clone();
    c.model = base
        .model
        .iter()
        .enumerate()
        .filter(|(i, _)| mask & (1_u64 << i) != 0)
        .map(|(_, f)| f.clone())
        .collect();
    if mask >> 63 != 0 {
        c.model
            .push("guideline_entity(actual,producer_extra,f,countable)".into());
    }
    if flavor == 0 {
        return c;
    }
    let mut facts = base.model.clone();
    facts.extend([
        "guideline_entity(actual,producer_extra,f,countable)".into(),
        "(".into(),
        "guideline_entity(actual,X,p,countable)".into(),
    ]);
    c.claims = claims
        .iter()
        .map(|(fi, ci, level, mode, valuation)| {
            let mut claim = base.claims[usize::from(*fi) % base.claims.len()].clone();
            claim.fact = facts[usize::from(*fi) % facts.len()].clone();
            claim.clause = usize::from(*ci);
            claim.level = *level;
            if *mode != 0 {
                claim.valuation = valuation
                    .iter()
                    .map(|(i, t)| {
                        (
                            *i as usize,
                            [
                                "a",
                                "b",
                                "rational(1,2)",
                                "'$guideline_id'(product,patient,1,ref(1),[])",
                                "(",
                                "X",
                                "exactly",
                                "-1",
                            ][usize::from(*t)]
                            .to_owned(),
                        )
                    })
                    .collect();
            }
            claim
        })
        .collect();
    if flavor == 2 {
        c.cert_id.push(0);
    }
    if flavor == 3 {
        c.model.push("(".into());
    }
    if flavor == 4 {
        c.facts.push(b"(".to_vec());
    }
    if flavor == 5 {
        c.docs.push(b"not a document".to_vec());
        c.patient = c.docs.len();
    }
    c
}
fn generated_soundness(
    checker: Checker,
    input: (Generated, u64, u8, Vec<ArbitraryClaim>),
) -> TestCaseResult {
    let (g, mask, flavor, claims) = input;
    let p = generated(g);
    let o = oracle::evaluate(&p);
    prop_assert!(o.undefined.is_empty());
    let base = Case::new(&p, &o.truth, false, 1);
    let expected = expected(&o.truth, &p);
    model_soundness(
        checker,
        &arbitrary_case(&base, mask, flavor, &claims),
        &expected,
    )?;
    // This deliberate false-model candidate keeps the control's firing non-vacuous for every
    // generated input, independent of the random certificate's ability to reach a verdict.
    model_soundness(checker, &wrong_model(&base, 0), &expected)
}
#[test]
fn p4_generated_model_soundness_including_malformed_adversarial_certificates() {
    harness::check(128, arbitrary_strategy(), |input| {
        generated_soundness(Case::check, input)
    });
}
#[test]
fn p4_generated_soundness_accept_all_control_fires() {
    control("generated", || {
        harness::check(32, arbitrary_strategy(), |input| {
            generated_soundness(accept_all, input)
        })
    });
}
#[test]
fn p4_generated_model_completeness() {
    harness::check(96, generated_strategy(), |g| {
        generated_completeness(g, false, 1)
    });
}
#[test]
fn p4_generated_certificate_completeness_with_nonminimal_levels_and_alternate_supports() {
    harness::check(
        96,
        (generated_strategy(), 1_u64..1_000_000, any::<bool>()),
        |(g, stretch, alternate)| {
            generated_completeness(g, alternate, stretch)?;
            let p = generated(g);
            let m = oracle::evaluate(&p).truth;
            let c = Case::new(&p, &m, alternate, stretch);
            invalid_certificate(Case::check, &bad_certificate(&c, g.0 as usize))
        },
    );
}
#[test]
fn p4_generated_questions_match_the_oracle_over_the_same_model() {
    harness::check(96, generated_strategy(), |g| {
        let p = generated(g);
        let m = oracle::evaluate(&p).truth;
        let c = Case::new(&p, &m, false, 1);
        let (goals, projection) = operator_question();
        question_matches(&c, &m, &p, &goals, &projection)?;
        question_matches(&c, &m, &p, &goals, &[])?;
        let (goals, projection) = arithmetic_question();
        question_matches(&c, &m, &p, &goals, &projection)?;
        let goals = vec![
            Goal::Positive(literal(Predicate::E, vec![Term::Var(0), Term::Var(1)])),
            pos(Predicate::P, Term::Var(1)),
        ];
        question_matches(&c, &m, &p, &goals, &[Term::Var(1), Term::Var(0)])
    });
}
#[test]
fn independent_oracle_renaming_clause_order_and_alpha_invariance() {
    harness::check(64, generated_strategy(), |g| {
        let p = generated(g);
        let left = oracle::evaluate(&p);
        let right = oracle::evaluate(&p.transform());
        prop_assert_eq!(
            right.truth,
            left.truth.iter().map(Literal::transform).collect()
        );
        prop_assert_eq!(
            right.undefined,
            left.undefined.iter().map(Literal::transform).collect()
        );
        Ok(())
    });
}
#[test]
fn frozen_enumeration_manifest_and_negative_cycle_exclusions_are_pinned() {
    harness::check(1, Just(()), |()| {
        let manifest = include_str!("../sb-oracle/manifest.md");
        prop_assert!(manifest.contains("93,369"));
        prop_assert!(manifest.contains("37,125"));
        let pool = domain::ground_catalogue();
        prop_assert_eq!(pool.len(), 66);
        let mut admitted = [0_usize; 4];
        let mut excluded = [0_usize; 4];
        let mut grade = |is: &[usize]| {
            let cs = is.iter().map(|i| pool[*i].clone()).collect::<Vec<_>>();
            if domain::ground_stratified(&cs) {
                admitted[is.len()] += 1;
            } else {
                excluded[is.len()] += 1;
            }
        };
        grade(&[]);
        for a in 0..pool.len() {
            grade(&[a]);
            for b in a + 1..pool.len() {
                grade(&[a, b]);
                for c in b + 1..pool.len() {
                    grade(&[a, b, c]);
                }
            }
        }
        prop_assert_eq!(admitted, [1, 48, 933, 9865]);
        prop_assert_eq!(excluded, [0, 18, 1212, 35895]);
        Ok(())
    });
}
