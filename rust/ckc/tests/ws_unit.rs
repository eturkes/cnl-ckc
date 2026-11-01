// Contract ws P1–P7. Oracles use the contract's rational cells + finite joins;
// implementation modules stay behind the compiler/certify/replay process boundary.
#[path = "ws-unit/controls.rs"]
mod controls;
#[path = "ws-unit/harness.rs"]
mod harness;
#[path = "ws-unit/joins.rs"]
mod joins;
#[path = "ws-unit/oracle.rs"]
mod oracle;

use harness::*;
use oracle::{Cmp, Expr, Q};
use std::fs;

const NOUNS: [(&str, &str, &str); 4] = [
    ("cat", "cats", "X"),
    ("dog", "dogs", "Z"),
    ("bird", "birds", "U"),
    ("mouse", "mice", "V"),
];
const SAFE: [(&str, &str); 4] = [
    ("horse", "horses"),
    ("sheep", "sheep"),
    ("cow", "cows"),
    ("pig", "pigs"),
];

struct Operand {
    cmp: Cmp,
    bound: Q,
}
struct Deny {
    operand: usize,
    cmp: Cmp,
    bound: Q,
}
struct Scene {
    operands: Vec<Operand>,
    deny: Vec<Deny>,
    expr: Expr,
}
impl Scene {
    fn raw_bound(q: Q, slot: usize) -> i128 {
        if q.1 == 1 && q.0 >= 0 {
            q.0
        } else {
            101 + 2 * slot as i128
        }
    }
    fn ace(&self, safe: bool) -> String {
        fn amount(cmp: Cmp, n: i128, singular: &str, plural: &str) -> String {
            if matches!(cmp, Cmp::Some) {
                format!("some {plural}")
            } else {
                format!(
                    "{} {n} {}",
                    cmp.word(),
                    if n == 1 { singular } else { plural }
                )
            }
        }
        let mut clauses = Vec::new();
        for (i, op) in self.operands.iter().enumerate() {
            let (sg, pl, var) = NOUNS[i];
            let n = Self::raw_bound(op.bound, i);
            clauses.push(format!(
                "the man contains {} {var}",
                amount(op.cmp, n, sg, pl)
            ));
        }
        if clauses.is_empty() {
            clauses.push("the man has a cat".to_owned());
        }
        for (i, deny) in self.deny.iter().enumerate() {
            let (sg, pl) = if safe {
                SAFE[deny.operand]
            } else {
                (NOUNS[deny.operand].0, NOUNS[deny.operand].1)
            };
            clauses.push(format!(
                "the man does not provably contain {}",
                amount(deny.cmp, Self::raw_bound(deny.bound, i + 4), sg, pl)
            ));
        }
        let antecedent = clauses.join(" and ").replacen("the man", "a man", 1);
        fn expression(e: &Expr) -> String {
            match e {
                Expr::Var(i) => NOUNS[*i].2.to_owned(),
                Expr::Lit(Q(n, 1)) => n.to_string(),
                Expr::Lit(Q(n, 2)) => format!("{}.5", n / 2),
                Expr::Lit(q) => panic!("source literal: {q}"),
                Expr::Mul(a, b) => format!("({} * {})", expression(a), expression(b)),
                Expr::Div(a, b) => format!("({} / {})", expression(a), expression(b)),
            }
        }
        format!(
            "If {antecedent} then the man includes some frogs Y and Y = {}.\n",
            expression(&self.expr)
        )
    }
    fn lists(&self) -> Vec<Vec<Q>> {
        self.operands
            .iter()
            .enumerate()
            .map(|(i, op)| {
                let bounds: Vec<_> = self
                    .deny
                    .iter()
                    .filter(|d| d.operand == i)
                    .map(|d| d.bound)
                    .collect();
                oracle::candidates(op.cmp, op.bound, &bounds)
            })
            .collect()
    }
    fn winner(&self) -> Option<Vec<usize>> {
        oracle::first(&self.lists(), |t| {
            self.expr.eval(t).is_some()
                && self
                    .deny
                    .iter()
                    .all(|d| !d.cmp.admits(t[d.operand], d.bound))
        })
    }
    fn case(&self, s: &Scratch, c: &Compiler, id: &str) -> Case {
        let ace = self.ace(false);
        let safe = self.ace(true);
        let swaps: Vec<_> = SAFE
            .iter()
            .zip(NOUNS)
            .map(|((sg, _), (target, _, _))| {
                (format!(",{sg},countable)"), format!(",{target},countable)"))
            })
            .collect();
        let refs: Vec<_> = swaps
            .iter()
            .filter(|(from, _)| {
                self.deny
                    .iter()
                    .any(|d| from == &format!(",{},countable)", SAFE[d.operand].0))
            })
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect();
        let case = Case::new(s, c, id, &ace, Some((&safe, &refs)));
        for (i, op) in self.operands.iter().enumerate() {
            let raw = Self::raw_bound(op.bound, i);
            if Q::int(raw) != op.bound {
                case.synthetic_bound(op.cmp.atom(), raw, op.bound);
            }
        }
        for (i, d) in self.deny.iter().enumerate() {
            let raw = Self::raw_bound(d.bound, i + 4);
            if Q::int(raw) != d.bound {
                case.synthetic_bound(d.cmp.atom(), raw, d.bound);
            }
        }
        case
    }
}
fn unary(cmp: Cmp, bound: i128, denied: Cmp, denied_bound: i128, divisor: bool) -> Scene {
    Scene {
        operands: vec![Operand {
            cmp,
            bound: Q::int(bound),
        }],
        deny: vec![Deny {
            operand: 0,
            cmp: denied,
            bound: Q::int(denied_bound),
        }],
        expr: if divisor {
            Expr::div(Expr::Lit(Q::int(3)), Expr::Var(0))
        } else {
            Expr::mul(Expr::Var(0), Expr::Lit(Q::new(3, 2)))
        },
    }
}
fn check_scene(
    checks: &mut Checks,
    s: &Scratch,
    c: &Compiler,
    id: &str,
    scene: &Scene,
    fragment: Option<bool>,
) {
    let case = scene.case(s, c, id);
    let expected = scene.winner();
    let lists = scene.lists();
    let synthetic = scene
        .operands
        .iter()
        .any(|o| o.bound.1 != 1 || o.bound.0 < 0)
        || scene.deny.iter().any(|d| d.bound.1 != 1 || d.bound.0 < 0);
    if !synthetic {
        checks.check(
            case.emitted.ok() == expected.is_some(),
            format!("{id} fork winner={expected:?}: {}", case.emitted.summary()),
        );
    }
    if let Some(walks) = checks.walk(&case.call(s, "walks"), id) {
        checks.check(walks.len() == 1, format!("{id} one formula variant"));
        if let Some(w) = walks.first() {
            checks.check(
                (w.sentence, w.variant) == (1, 1),
                format!("{id} document-order coordinates"),
            );
            checks.check(
                w.candidates == lists,
                format!("{id} candidates expected={lists:?} got={:?}", w.candidates),
            );
            checks.check(
                w.winner == expected,
                format!("{id} first winner expected={expected:?} got={:?}", w.winner),
            );
            if let Some(f) = fragment {
                checks.check(
                    w.fragment == f,
                    format!("{id} fragment expected={f} got={}", w.fragment),
                );
            }
            if w.candidates == lists {
                checks.check(
                    oracle::prefix(&lists, w.winner.as_deref())
                        == oracle::prefix(&lists, expected.as_deref()),
                    format!("{id} odometer prefix"),
                );
            }
        }
    }
    let payload = case.call(s, "doc");
    checks.success(&payload, &format!("{id} certification"));
    if payload.ok() {
        let digits = expected.clone().unwrap_or_else(|| vec![0; lists.len()]);
        let tuple: Vec<_> = lists.iter().zip(&digits).map(|(l, &d)| l[d]).collect();
        for (i, &value) in tuple.iter().enumerate() {
            checks.check(
                cardinalities(&payload.out, 1, NOUNS[i].0) == [value],
                format!("{id} operand {i} expected={value}"),
            );
        }
        let value = scene
            .expr
            .eval(&tuple)
            .expect("all generated defaults have defined chains");
        checks.check(
            head_counts(&payload.out, 1, "frog") == [value],
            format!("{id} target expected={value}"),
        );
        let replay = case.replay(s, &payload.out);
        checks.check(
            if expected.is_some() {
                replay.ok()
            } else {
                replay.rc == Some(1) && replay.err.contains("obligation_failed")
            },
            format!(
                "{id} native replay winner={expected:?}: {}",
                replay.summary()
            ),
        );
        if !synthetic && expected.is_some() {
            let fork = c.emit(s, id, &case.source, true);
            checks.success(&fork, &format!("{id} fork proof"));
            checks.check(
                fork.out == payload.out,
                format!("{id} fork = certified payload"),
            );
        }
    }
}

#[test]
fn p1_phase_example() {
    let s = Scratch::new("p1");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let source = fixture("phase.ace.in");
    let safe = source.replace("at least 1 cat then", "at least 1 bird then");
    let case = Case::new(
        &s,
        &c,
        "ws-p1",
        &source,
        Some((&safe, &[(",bird,countable)", ",cat,countable)")])),
    );
    checks.success(&case.emitted, "Phase compiles");
    let payload = case.call(&s, "doc");
    checks.success(&payload, "Phase certifies");
    if payload.ok() {
        let xs = cardinalities(&payload.out, 1, "cat");
        checks.check(
            xs == [Q::int(-1)],
            format!("Phase exact first operand: {xs:?}"),
        );
        checks.check(xs.iter().all(|&x| x < Q::int(1)), "Phase antecedent + NAF");
        checks.success(&case.replay(&s, &payload.out), "Phase aggregate");
    }
    checks.finish();
}
#[test]
fn p2a_midpoint_below_100() {
    let s = Scratch::new("p2a");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let scene = unary(Cmp::Lt, 100, Cmp::Le, 99, false);
    assert_eq!(scene.lists()[0][scene.winner().unwrap()[0]], Q::new(199, 2));
    check_scene(&mut checks, &s, &c, "ws-p2a", &scene, Some(true));
    checks.finish();
}
#[test]
fn p2b_defined_divisor_below_one() {
    let s = Scratch::new("p2b");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    check_scene(
        &mut checks,
        &s,
        &c,
        "ws-p2b",
        &unary(Cmp::Some, 0, Cmp::Ge, 1, true),
        Some(true),
    );
    checks.finish();
}
#[test]
fn p2c_midpoint_below_two() {
    let s = Scratch::new("p2c");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    check_scene(
        &mut checks,
        &s,
        &c,
        "ws-p2c",
        &unary(Cmp::Lt, 2, Cmp::Le, 1, false),
        Some(true),
    );
    checks.finish();
}
#[test]
fn p2d_midpoint_above_zero() {
    let s = Scratch::new("p2d");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    check_scene(
        &mut checks,
        &s,
        &c,
        "ws-p2d",
        &unary(Cmp::Gt, 0, Cmp::Ge, 1, false),
        Some(true),
    );
    checks.finish();
}
#[test]
fn p3_contradictory_naf_fallback() {
    let s = Scratch::new("p3");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    check_scene(
        &mut checks,
        &s,
        &c,
        "ws-p3",
        &unary(Cmp::Ge, 2, Cmp::Ge, 1, false),
        Some(true),
    );
    checks.finish();
}

#[test]
fn p4a_document_rules_defeat_local_first_tuple() {
    let s = Scratch::new("p4a");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let source = fixture("replay-probe.ace.in");
    let safe = source.replace("at least 1 cat then", "at least 1 bird then");
    let case = Case::new(
        &s,
        &c,
        "ws-p4a",
        &source,
        Some((&safe, &[(",bird,countable)", ",cat,countable)")])),
    );
    checks.success(&case.emitted, "replay-checked fork");
    if let Some(walks) = checks.walk(&case.call(&s, "walks"), "replay-checked walks") {
        checks.check(
            walks.len() == 1,
            "one formula group, later nonformula omitted",
        );
        let w = &walks[0];
        checks.check(
            w.candidates == [oracle::candidates(Cmp::Some, Q::int(0), &[Q::int(1)])],
            "only relevant NAF bound 1",
        );
        checks.check(
            w.winner == Some(vec![2]),
            "own later rule defeats -1; 0 first replayable tuple",
        );
        checks.check(!w.fragment, "rule call outside completeness fragment");
    }
    let payload = case.call(&s, "doc");
    checks.success(&payload, "replay-checked certifies");
    if payload.ok() {
        checks.check(
            cardinalities(&payload.out, 1, "cat") == [Q::int(0)],
            "zero witness",
        );
        checks.check(
            head_counts(&payload.out, 1, "dog") == [Q::int(0)],
            "zero product is defined",
        );
        checks.success(&case.replay(&s, &payload.out), "replay-checked aggregate");
        let fork = c.emit(&s, "ws-p4a", &source, true);
        checks.success(&fork, "replay-checked proof");
        checks.check(
            fork.out == payload.out,
            "fork = certified payload, including later sentence",
        );
    }
    checks.finish();
}

#[test]
fn p4b_naf_depth_cut_is_not_absence() {
    let s = Scratch::new("p4b");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let source = fixture("naf-depth-cut.ace.in");
    for proof in [false, true] {
        let got = c.emit(&s, "ws-p4b", &source, proof);
        checks.check(
            got.rc == Some(1) && got.out.is_empty() && got.err.contains("underivable_obligation"),
            format!("depth-cut fork refuses (proof={proof}): {}", got.summary()),
        );
    }
    // Constant-term recursion reaches the same K2 cut without building growing
    // Skolem terms; the real-ACE producer path above remains separate.
    let got = controls::constant_cut(&s);
    checks.check(
        got.rc == Some(1) && got.out.is_empty() && got.err.contains("obligation_failed"),
        format!("depth-cut native refuses: {}", got.summary()),
    );
    checks.finish();
}

#[test]
fn p5a_interval_and_chain_differential() {
    let s = Scratch::new("p5a");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let mut count = 0;
    for (lo, hi) in [(0, 1), (1, 2), (2, 3), (9, 10), (2, 2), (3, 2)] {
        for divisor in [false, true] {
            let scene = unary(Cmp::Lt, hi, Cmp::Le, lo, divisor);
            checks.check(
                scene.winner().is_some() == (lo < hi),
                format!("independent dense interval ({lo},{hi})"),
            );
            check_scene(
                &mut checks,
                &s,
                &c,
                &format!("ws-p5a-{count}"),
                &scene,
                Some(true),
            );
            count += 1;
        }
    }
    assert_eq!(count, 12);
    checks.finish();
}

#[test]
fn p5c1_rational_default_and_strict_bounds() {
    let s = Scratch::new("p5c1");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    for (i, cmp, q) in [
        (0, Cmp::Gt, Q::new(1, 2)),
        (1, Cmp::Lt, Q::new(1, 2)),
        (2, Cmp::Ge, Q::new(3, 2)),
        (3, Cmp::Le, Q::new(3, 2)),
        (4, Cmp::Eq, Q::new(3, 2)),
    ] {
        let scene = Scene {
            operands: vec![Operand { cmp, bound: q }],
            deny: vec![],
            expr: Expr::mul(Expr::Var(0), Expr::Lit(Q::int(2))),
        };
        check_scene(
            &mut checks,
            &s,
            &c,
            &format!("ws-p5c1-{i}"),
            &scene,
            Some(true),
        );
    }
    checks.finish();
}
#[test]
fn p5c2_every_comparator_default_and_boundaries() {
    let s = Scratch::new("p5c2");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    for (i, cmp) in [Cmp::Some, Cmp::Eq, Cmp::Ge, Cmp::Gt, Cmp::Le, Cmp::Lt]
        .into_iter()
        .enumerate()
    {
        for bound in [0, 1, 2] {
            let scene = Scene {
                operands: vec![Operand {
                    cmp,
                    bound: Q::int(bound),
                }],
                deny: vec![],
                expr: Expr::mul(Expr::Var(0), Expr::Lit(Q::int(1))),
            };
            check_scene(
                &mut checks,
                &s,
                &c,
                &format!("ws-p5c2-{i}-{bound}"),
                &scene,
                Some(true),
            );
        }
    }
    checks.finish();
}
#[test]
fn p5c3_signed_integer_and_rational_cells() {
    let s = Scratch::new("p5c3");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    for (i, cmp, q) in [
        (0, Cmp::Gt, Q::int(-1)),
        (1, Cmp::Lt, Q::int(1)),
        (2, Cmp::Gt, Q::new(-3, 2)),
        (3, Cmp::Lt, Q::new(-1, 2)),
        (4, Cmp::Ge, Q::int(-2)),
        (5, Cmp::Le, Q::int(-2)),
    ] {
        let scene = Scene {
            operands: vec![Operand { cmp, bound: q }],
            deny: vec![Deny {
                operand: 0,
                cmp: Cmp::Eq,
                bound: cmp.default(q),
            }],
            expr: Expr::mul(Expr::Var(0), Expr::Lit(Q::int(2))),
        };
        check_scene(
            &mut checks,
            &s,
            &c,
            &format!("ws-p5c3-{i}"),
            &scene,
            Some(true),
        );
    }
    checks.finish();
}

#[test]
fn p5c5_zero_multiplicand_and_literal() {
    let s = Scratch::new("p5c5");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    for (i, expr) in [
        Expr::mul(Expr::Var(0), Expr::Lit(Q::int(2))),
        Expr::mul(Expr::Lit(Q::int(0)), Expr::Var(0)),
        Expr::div(Expr::Lit(Q::int(0)), Expr::Lit(Q::int(2))),
    ]
    .into_iter()
    .enumerate()
    {
        let scene = Scene {
            operands: vec![Operand {
                cmp: Cmp::Eq,
                bound: Q::int(0),
            }],
            deny: vec![],
            expr,
        };
        if i < 2 {
            check_scene(
                &mut checks,
                &s,
                &c,
                &format!("ws-p5c5-{i}"),
                &scene,
                Some(true),
            );
        } else {
            let literal = Scene {
                operands: vec![],
                deny: vec![],
                expr: scene.expr,
            };
            check_scene(&mut checks, &s, &c, "ws-p5c5-literal", &literal, Some(true));
        }
    }
    checks.finish();
}

#[test]
fn p5c6_repeated_shared_operands_and_chain_order() {
    let s = Scratch::new("p5c6");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let mut scene = Scene {
        operands: vec![Operand {
            cmp: Cmp::Some,
            bound: Q::int(0),
        }],
        deny: vec![Deny {
            operand: 0,
            cmp: Cmp::Ge,
            bound: Q::int(1),
        }],
        expr: Expr::mul(Expr::Var(0), Expr::Var(0)),
    };
    check_scene(&mut checks, &s, &c, "ws-p5c6-repeat", &scene, Some(true));
    scene.operands.push(Operand {
        cmp: Cmp::Some,
        bound: Q::int(0),
    });
    scene.expr = Expr::div(Expr::mul(Expr::Var(0), Expr::Lit(Q::int(2))), Expr::Var(1));
    let ordered = scene.case(&s, &c, "ws-p5c6-order");
    let pl = fs::read_to_string(&ordered.pl).unwrap();
    let bodies: Vec<_> = pl
        .lines()
        .filter_map(|l| l.split_once(" :- ").map(|(_, b)| b))
        .collect();
    assert!(!bodies.is_empty());
    assert!(bodies.iter().all(|b| b.find("guideline_product(").unwrap() < b.find("guideline_quotient(").unwrap()), "public clauses keep the formula's product→quotient postorder");
    check_scene(&mut checks, &s, &c, "ws-p5c6-chain", &scene, Some(true));
    let ace = "If a man contains some cats X and does not provably contain at least 1 cat then the man includes some frogs Y and Y = X * 2 and the man includes some wolves Z and Z = X * 3.\n";
    let safe = ace.replace("at least 1 cat then", "at least 1 bird then");
    let case = Case::new(
        &s,
        &c,
        "ws-p5c6-shared",
        ace,
        Some((&safe, &[(",bird,countable)", ",cat,countable)")])),
    );
    if let Some(w) = checks.walk(&case.call(&s, "walks"), "shared walks") {
        checks.check(
            w.len() == 1 && w[0].candidates.len() == 1 && w[0].winner == Some(vec![1]),
            "operand once across both formulas",
        );
    }
    let proof = case.call(&s, "doc");
    checks.success(&proof, "shared certifies");
    if proof.ok() {
        checks.check(
            cardinalities(&proof.out, 1, "cat") == [Q::int(-1)],
            "shared substitution",
        );
        checks.check(
            head_counts(&proof.out, 1, "frog") == [Q::int(-2)]
                && head_counts(&proof.out, 1, "wolf") == [Q::int(-3)],
            "both targets use shared selection",
        );
        checks.success(&case.replay(&s, &proof.out), "shared replays");
    }
    checks.finish();
}

#[test]
fn p5c7_literal_only_one_empty_tuple() {
    let s = Scratch::new("p5c7");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let scene = Scene {
        operands: vec![],
        deny: vec![],
        expr: Expr::mul(Expr::Lit(Q::int(2)), Expr::Lit(Q::int(3))),
    };
    check_scene(&mut checks, &s, &c, "ws-p5c7", &scene, Some(true));
    checks.finish();
}

fn walk_family(kind: &str) {
    let s = Scratch::new(kind);
    let c = Compiler::shared();
    let mut checks = Checks::default();
    for n in 0..=4 {
        if n == 0 && kind == "none" {
            let ace = "If a man has a cat and does not provably have a cat then the man includes some frogs Y and Y = 2 * 3.\n";
            let safe = ace.replace("provably have a cat then", "provably have a horse then");
            let case = Case::new(
                &s,
                &c,
                "ws-p6-none-0",
                ace,
                Some((&safe, &[(",horse,countable)", ",cat,countable)")])),
            );
            checks.check(
                case.emitted.rc == Some(1) && case.emitted.err.contains("underivable_obligation"),
                "empty tuple refutes fixed NAF",
            );
            if let Some(w) = checks.walk(&case.call(&s, "walks"), "empty-tuple no winner") {
                checks.check(
                    w.len() == 1
                        && w[0].candidates.is_empty()
                        && w[0].winner.is_none()
                        && w[0].fragment,
                    "zero operands, one failed empty tuple, inside fragment",
                );
            }
            let proof = case.call(&s, "doc");
            checks.success(&proof, "empty-tuple fallback projects");
            if proof.ok() {
                let replay = case.replay(&s, &proof.out);
                checks.check(
                    replay.rc == Some(1) && replay.err.contains("obligation_failed"),
                    "empty-tuple fallback fails aggregate",
                );
            }
            assert_eq!(oracle::prefix(&[], None), 1);
            continue;
        }
        let mut scene = Scene {
            operands: (0..n)
                .map(|_| Operand {
                    cmp: if kind == "late" {
                        Cmp::Lt
                    } else if kind == "none" {
                        Cmp::Ge
                    } else {
                        Cmp::Some
                    },
                    bound: Q::int(if kind == "late" { 2 } else { 0 }),
                })
                .collect(),
            deny: (0..n)
                .filter(|_| kind != "first")
                .map(|i| Deny {
                    operand: i,
                    cmp: if kind == "late" { Cmp::Le } else { Cmp::Ge },
                    bound: Q::int(if kind == "late" { 1 } else { 0 }),
                })
                .collect(),
            expr: Expr::mul(Expr::Lit(Q::int(2)), Expr::Lit(Q::int(3))),
        };
        if n > 0 {
            scene.expr = (1..n).fold(Expr::mul(Expr::Var(0), Expr::Lit(Q::int(2))), |e, i| {
                Expr::mul(e, Expr::Var(i))
            });
        }
        // Every chain is postorder; its own intermediate reads stay inside D7.
        check_scene(
            &mut checks,
            &s,
            &c,
            &format!("ws-p6-{kind}-{n}"),
            &scene,
            Some(true),
        );
        let lists = scene.lists();
        let winner = scene.winner();
        let expected_visits = if kind == "first" || n == 0 {
            1
        } else {
            lists.iter().map(Vec::len).product()
        };
        assert_eq!(
            oracle::prefix(&lists, winner.as_deref()),
            expected_visits,
            "reference odometer {kind}/{n}"
        );
    }
    checks.finish();
}
#[test]
fn p6a_first_winner_zero_to_four_operands() {
    walk_family("first");
}
#[test]
fn p6b_late_winner_zero_to_four_operands() {
    walk_family("late");
}
#[test]
fn p6c_no_winner_zero_to_four_operands() {
    walk_family("none");
}

#[test]
fn p6e_undefined_chain_and_reject_priority() {
    let s = Scratch::new("p6e");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    for (i, ace) in [
        "If a man contains exactly 0 cats X then the man includes some frogs Y and Y = 3 / X.\n",
        "If a man contains some cats X then the man includes some frogs Y and Y = X / 0.\n",
        "If a man contains some cats X then the man includes some frogs Y and Y = X / 0.\nIf a man contains some cats X then the man includes exactly 1 frog Y and Y = X * 1.\n",
    ].into_iter().enumerate() {
        let got = c.emit(&s, &format!("ws-p6e-{i}"), ace, false);
        checks.check(got.rc == Some(1) && got.out.is_empty() && got.err == "ace_to_pl_error(unsupported,formula_shape(witness,1)).\n", format!("reject priority {i}: {}", got.summary()));
        let dump = c.dump(&s, ace, &s.file("lexicon.ulex", ""));
        let valid = "If a man contains some cats X then the man includes some frogs Y and Y = X * 1.\n".repeat(if i == 2 { 2 } else { 1 });
        let projected = c.emit(&s, &format!("ws-p6e-{i}"), &valid, false); assert!(projected.ok(), "reject control");
        let pl = s.file(&format!("reject-{i}.pl"), &custody(&projected.out, &valid, ace));
        let source = s.file(&format!("reject-{i}.ace"), ace); let dump = s.file(&format!("reject-{i}.dump"), &dump);
        let got = native(&s, &["certify-one", "doc", &format!("ws-p6e-{i}"), p(&source), p(&s.0.join("lexicon.ulex")), p(&dump), p(&pl)]);
        checks.check(got.rc == Some(1) && got.out.is_empty() && got.err.contains("formula_shape(witness)"), format!("native reject priority {i}: {}", got.summary()));
    }
    checks.finish();
}

fn committed_payload(id: &str) {
    let s = Scratch::new(id);
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let base = root().join("guidelines/cdc-2022-opioid");
    let source = base.join(format!("ace/{id}.ace"));
    let pl = base.join(format!("pl/{id}.pl"));
    let ulex = base.join("lexicon.ulex");
    let vocabulary = base.join("vocabulary.tsv");
    let ace = fs::read_to_string(&source).unwrap();
    let dump = s.file("doc.dump", &c.dump(&s, &ace, &ulex));
    let got = native(
        &s,
        &[
            "certify-one",
            "doc",
            id,
            p(&source),
            p(&ulex),
            p(&dump),
            p(&pl),
            p(&vocabulary),
        ],
    );
    checks.success(&got, "committed formula certifies");
    if got.ok() {
        let pins = fixture("committed-payloads.in");
        let row = pins.lines().find(|l| l.starts_with(id)).unwrap();
        let cells: Vec<_> = row.split('\t').collect();
        checks.check(
            got.out.len() == cells[1].parse::<usize>().unwrap(),
            "seed payload byte length",
        );
        checks.check(sha(&got.out) == cells[2], "seed payload sha256");
    }
    checks.finish();
}
#[test]
fn p7a_committed_s33_payload_byte_identity() {
    committed_payload("cdc2022-opioid-s33-01");
}
#[test]
fn p7b_committed_s42_payload_byte_identity() {
    committed_payload("cdc2022-opioid-s42-15");
}
