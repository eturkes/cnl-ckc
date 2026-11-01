use super::*;
use Cell::{Atom as A, Num as N, Slot as S, Var as V};
use oracle::{Cell, Fact};

fn basic() -> Vec<Fact> {
    vec![
        ("noun", vec![A("cat-ref"), A("cat")]),
        ("count", vec![A("cat-ref"), S(0)]),
        ("contain", vec![A("man"), A("cat-ref")]),
    ]
}
fn denied(verb: &'static str, noun: &'static str) -> Vec<Fact> {
    vec![
        ("noun", vec![V("r"), A(noun)]),
        ("count", vec![V("r"), V("d")]),
        (verb, vec![A("man"), V("r")]),
    ]
}
fn joined_case(
    id: &str,
    ace: &str,
    facts: &[Fact],
    goals: &[Fact],
    relevant_expected: bool,
    noun: &str,
) {
    let s = Scratch::new(id);
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let is_relevant = oracle::relevant(facts, goals, "d", 0);
    assert_eq!(
        is_relevant, relevant_expected,
        "finite join positive/negative control"
    );
    let bounds = if is_relevant { vec![Q::int(7)] } else { vec![] };
    let expected = oracle::candidates(Cmp::Some, Q::int(0), &bounds);
    let case = Case::new(&s, &c, id, ace, None);
    checks.success(
        &case.emitted,
        "irrelevant or default-satisfied NAF compiles",
    );
    if let Some(walks) = checks.walk(&case.call(&s, "walks"), "finite join candidates") {
        checks.check(walks.len() == 1, "one formula variant");
        if let Some(w) = walks.first() {
            checks.check(
                w.candidates == [expected],
                format!("finite labelled-slot join: {:?}", w.candidates),
            );
            checks.check(w.winner == Some(vec![0]), "default remains first winner");
        }
    }
    let got = case.call(&s, "doc");
    checks.success(&got, "join projection certifies");
    if got.ok() {
        checks.check(
            cardinalities(&got.out, 1, noun) == [Q::int(1)],
            "operand default independent of fixed nonoperand counts",
        );
        checks.success(&case.replay(&s, &got.out), "join aggregate");
        let proof = c.emit(&s, id, ace, true);
        checks.success(&proof, "join fork proof");
        checks.check(got.out == proof.out, "join payload parity");
    }
    checks.finish();
}

#[test]
fn p5b1_irrelevant_bound_insertion() {
    let goals = denied("contain", "bird");
    let facts = basic();
    assert!(
        oracle::relevant(&facts, &denied("contain", "cat"), "d", 0),
        "same-fact positive control"
    );
    joined_case(
        "ws-p5b1",
        "If a man contains some cats X and does not provably contain at least 7 birds then the man includes some frogs Y and Y = X * 2.\n",
        &facts,
        &goals,
        false,
        "cat",
    );
}
#[test]
fn p5b2_equal_number_is_not_slot_origin() {
    let mut facts = basic();
    facts.extend([
        ("noun", vec![A("bird-ref"), A("bird")]),
        ("count", vec![A("bird-ref"), N(Q::int(1))]),
        ("contain", vec![A("man"), A("bird-ref")]),
    ]);
    joined_case(
        "ws-p5b2",
        "If a man contains some cats X and the man contains exactly 1 bird and the man does not provably contain at least 7 birds then the man includes some frogs Y and Y = X * 2.\n",
        &facts,
        &denied("contain", "bird"),
        false,
        "cat",
    );
}
#[test]
fn p5b3_failed_positive_join_contributes_no_bound() {
    joined_case(
        "ws-p5b3",
        "If a man contains some cats X and does not provably chase at least 7 cats then the man includes some frogs Y and Y = X * 2.\n",
        &basic(),
        &denied("chase", "cat"),
        false,
        "cat",
    );
}
fn correlated(container: &'static str, id: &str, ace: &str, expected: bool) {
    let goals = vec![
        ("noun", vec![V("cat"), A("cat")]),
        ("noun", vec![V("bird"), A("bird")]),
        ("count", vec![V("bird"), V("d")]),
        ("chase", vec![A("man"), V("cat")]),
        ("contain", vec![V("cat"), V("bird")]),
    ];
    let facts = vec![
        ("noun", vec![A("red"), A("cat")]),
        ("noun", vec![A("blue"), A("cat")]),
        ("noun", vec![A("birds"), A("bird")]),
        ("count", vec![A("birds"), S(0)]),
        ("chase", vec![A("man"), A("red")]),
        ("contain", vec![A(container), A("birds")]),
    ];
    joined_case(id, ace, &facts, &goals, expected, "bird");
}
#[test]
fn p5b4_correlated_locals_share_one_substitution() {
    correlated(
        "blue",
        "ws-p5b4-no",
        "If a man chases a red cat and a blue cat contains some birds X and the man does not provably chase a cat that contains at least 7 birds then the man includes some frogs Y and Y = X * 2.\n",
        false,
    );
}
#[test]
fn p5b4_correlated_positive_join_collects_bound() {
    correlated(
        "red",
        "ws-p5b4-yes",
        "If a man chases a red cat and the red cat contains some birds X and the man does not provably chase a cat that contains at least 7 birds then the man includes some frogs Y and Y = X * 2.\n",
        true,
    );
}

#[test]
fn p5c4_fixed_nonoperand_facts_remain_unchanged() {
    let s = Scratch::new("p5c4");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let ace = "If a man contains some cats X and the man has exactly 3 birds and the man does not provably contain at least 1 cat then the man includes some frogs Y and Y = X * 2.\n";
    let safe = ace.replace("at least 1 cat then", "at least 1 horse then");
    let case = Case::new(
        &s,
        &c,
        "ws-p5c4",
        ace,
        Some((&safe, &[(",horse,countable)", ",cat,countable)")])),
    );
    let got = case.call(&s, "doc");
    checks.success(&got, "fixed facts certifies");
    if got.ok() {
        checks.check(
            cardinalities(&got.out, 1, "cat") == [Q::int(-1)],
            "formula operand searched",
        );
        let terms = oracle::terms(&got.out);
        let facts = terms[0].args("$guideline_proof")[3].args("witness")[0].list();
        let bird = facts
            .iter()
            .find_map(|f| match f {
                oracle::Term::Fun(n, xs)
                    if n == "guideline_entity" && xs[2] == oracle::Term::Atom("bird".into()) =>
                {
                    Some(&xs[1])
                }
                _ => None,
            })
            .unwrap();
        checks.check(
            facts.iter().any(|f| match f {
                oracle::Term::Fun(n, xs) if n == "guideline_cardinality" && &xs[1] == bird => {
                    xs[3].atom() == "exactly" && xs[4].q() == Q::int(3)
                }
                _ => false,
            }),
            "fixed nonoperand exact 3 retained",
        );
        checks.success(&case.replay(&s, &got.out), "fixed fact replay");
    }
    if let Some(w) = checks.walk(&case.call(&s, "walks"), "fixed fact walks") {
        checks.check(
            w.len() == 1 && w[0].candidates.len() == 1,
            "nonoperand excluded from product",
        );
    }
    checks.finish();
}

#[test]
fn p5b5_extraction_ignores_builtin_truth() {
    let mut facts = basic();
    facts.extend([
        ("noun", vec![A("bird-ref"), A("bird")]),
        ("count", vec![A("bird-ref"), N(Q::int(1))]),
        ("have", vec![A("cat-ref"), A("bird-ref")]),
    ]);
    let mut goals = denied("contain", "cat");
    goals.extend([
        ("noun", vec![V("bird"), A("bird")]),
        ("count", vec![V("bird"), V("fixed-count")]),
        ("have", vec![V("r"), V("bird")]),
    ]);
    joined_case(
        "ws-p5b5",
        "If a man contains some cats X and the cats have exactly 1 bird and the man does not provably contain at least 7 cats that have at least 3 birds then the man includes some frogs Y and Y = X * 2.\n",
        &facts,
        &goals,
        true,
        "cat",
    );
}

#[test]
fn p5b6_other_operand_slots_keep_separate_boundaries() {
    let s = Scratch::new("p5b6");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let scene = Scene {
        operands: vec![
            Operand {
                cmp: Cmp::Some,
                bound: Q::int(0),
            },
            Operand {
                cmp: Cmp::Some,
                bound: Q::int(0),
            },
        ],
        deny: vec![
            Deny {
                operand: 0,
                cmp: Cmp::Ge,
                bound: Q::int(1),
            },
            Deny {
                operand: 1,
                cmp: Cmp::Ge,
                bound: Q::int(7),
            },
        ],
        expr: Expr::mul(Expr::Var(0), Expr::Var(1)),
    };
    check_scene(&mut checks, &s, &c, "ws-p5b6", &scene, Some(true));
    checks.finish();
}
