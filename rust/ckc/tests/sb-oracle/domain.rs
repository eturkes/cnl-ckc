use super::model::{
    BuiltinKind, Clause, Goal, Literal, Predicate, Program, Term, atom, clause, identity, literal,
    number,
};
use super::numbers::{Rational, conversion};
use std::collections::BTreeSet;

const C8: [&str; 8] = [
    "exactly", "eq", "geq", "greater", "leq", "less", "about", "na",
];
const C5: [&str; 5] = ["exactly", "geq", "greater", "leq", "less"];
const C3: [&str; 3] = ["exactly", "geq", "less"];
const UNITS: [&str; 7] = ["second", "minute", "hour", "day", "week", "month", "year"];
const OPS: [BuiltinKind; 2] = [BuiltinKind::Product, BuiltinKind::Quotient];

fn n7() -> Vec<Term> {
    [(-2, 1), (-1, 1), (-1, 2), (0, 1), (1, 2), (1, 1), (2, 1)]
        .iter()
        .map(|(p, q)| Term::Number(Rational::new(*p, *q)))
        .collect()
}

fn n3() -> Vec<Term> {
    [-1, 0, 1].into_iter().map(number).collect()
}

fn pairs(comparisons: &[&str], counts: &[Term]) -> Vec<Vec<Term>> {
    comparisons
        .iter()
        .flat_map(|s| counts.iter().map(move |n| vec![atom(s), n.clone()]))
        .collect()
}

fn unary(predicate: Predicate, term: Term) -> Literal {
    literal(predicate, vec![term])
}

fn binary(predicate: Predicate, left: Term, right: Term) -> Literal {
    literal(predicate, vec![left, right])
}

fn positive(l: Literal) -> Goal {
    Goal::Positive(l)
}

fn negative(ls: Vec<Literal>) -> Goal {
    Goal::Negative(ls.into_iter().map(positive).collect())
}

fn append_program(
    family: &'static str,
    index: &mut usize,
    carrier: &[Term],
    extra: Vec<Clause>,
    emit: &mut impl FnMut(Program),
) {
    let mut program = Program::new(family, *index, carrier);
    program.clauses.extend(extra);
    *index += 1;
    emit(program);
}

fn subsets(size: usize, max: usize) -> Vec<Vec<usize>> {
    fn choose(
        size: usize,
        remaining: usize,
        next: usize,
        prefix: &mut Vec<usize>,
        out: &mut Vec<Vec<usize>>,
    ) {
        if remaining == 0 {
            out.push(prefix.clone());
        } else {
            for i in next..=size - remaining {
                prefix.push(i);
                choose(size, remaining - 1, i + 1, prefix, out);
                prefix.pop();
            }
        }
    }
    let mut out = Vec::new();
    for count in 0..=max {
        choose(size, count, 0, &mut Vec::new(), &mut out);
    }
    out
}

pub fn ground_catalogue() -> Vec<Clause> {
    use Predicate::*;
    let atoms = [P, Q, R].map(|p| unary(p, atom("a")));
    let mut bodies = vec![vec![]];
    for l in &atoms {
        bodies.push(vec![positive(l.clone())]);
    }
    for l in &atoms {
        bodies.push(vec![negative(vec![l.clone()])]);
    }
    for i in 0..3 {
        for j in i + 1..3 {
            bodies.push(vec![positive(atoms[i].clone()), positive(atoms[j].clone())]);
        }
    }
    for a in &atoms {
        for b in &atoms {
            bodies.push(vec![positive(a.clone()), negative(vec![b.clone()])]);
        }
    }
    for i in 0..3 {
        for j in i + 1..3 {
            bodies.push(vec![negative(vec![atoms[i].clone(), atoms[j].clone()])]);
        }
    }
    assert_eq!(bodies.len(), 22);
    atoms
        .iter()
        .flat_map(|h| bodies.iter().map(move |b| clause(h.clone(), b.clone())))
        .collect()
}

pub fn ground_stratified(clauses: &[Clause]) -> bool {
    let index = |p| match p {
        Predicate::P => 0,
        Predicate::Q => 1,
        Predicate::R => 2,
        _ => unreachable!(),
    };
    let mut edges = [[false; 3]; 3];
    let mut negatives = Vec::new();
    for (i, row) in edges.iter_mut().enumerate() {
        row[i] = true;
    }
    for c in clauses {
        let h = index(c.head.predicate);
        for goal in &c.body {
            match goal {
                Goal::Positive(l) => edges[h][index(l.predicate)] = true,
                Goal::Negative(gs) => {
                    for g in gs {
                        let Goal::Positive(l) = g else { unreachable!() };
                        let b = index(l.predicate);
                        edges[h][b] = true;
                        negatives.push((h, b));
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    for k in 0..3 {
        for i in 0..3 {
            for j in 0..3 {
                edges[i][j] |= edges[i][k] && edges[k][j];
            }
        }
    }
    negatives.iter().all(|(h, b)| !edges[*b][*h])
}

fn variable_rules() -> Vec<Clause> {
    use Predicate::*;
    let (x, y, l, k) = (Term::Var(0), Term::Var(1), Term::Var(2), Term::Var(3));
    let u = |p, t: &Term| unary(p, t.clone());
    let e = |a: &Term, b: &Term| binary(E, a.clone(), b.clone());
    let r = || u(R, &x);
    vec![
        clause(u(P, &x), vec![positive(u(Q, &x))]),
        clause(u(Q, &x), vec![positive(u(P, &x))]),
        clause(u(P, &y), vec![positive(u(P, &x)), positive(e(&x, &y))]),
        clause(u(Q, &x), vec![positive(e(&x, &y)), positive(u(P, &y))]),
        clause(r(), vec![positive(u(D, &x)), negative(vec![u(Q, &x)])]),
        clause(
            r(),
            vec![positive(u(D, &x)), negative(vec![e(&x, &l), u(Q, &l)])],
        ),
        clause(
            r(),
            vec![
                positive(u(D, &x)),
                negative(vec![e(&x, &l), u(Q, &l), u(P, &l)]),
            ],
        ),
        clause(
            r(),
            vec![positive(u(D, &x)), negative(vec![e(&x, &l), e(&l, &x)])],
        ),
        clause(
            r(),
            vec![
                positive(u(D, &x)),
                negative(vec![e(&x, &l), e(&k, &l), u(P, &k)]),
            ],
        ),
        clause(
            r(),
            vec![
                positive(u(D, &x)),
                negative(vec![e(&x, &l), e(&x, &k), e(&l, &k)]),
            ],
        ),
        clause(
            r(),
            vec![
                positive(u(D, &x)),
                negative(vec![e(&x, &l), u(Q, &l)]),
                negative(vec![e(&x, &k), u(P, &k)]),
            ],
        ),
        clause(
            r(),
            vec![
                positive(u(D, &x)),
                positive(e(&x, &y)),
                negative(vec![e(&y, &l), u(Q, &l)]),
                negative(vec![e(&x, &k), u(P, &k)]),
            ],
        ),
    ]
}

fn variable_edb(carrier: &[Term], include_q: bool) -> Vec<Literal> {
    use Predicate::*;
    let mut facts = carrier
        .iter()
        .map(|t| unary(P, t.clone()))
        .collect::<Vec<_>>();
    if include_q {
        facts.extend(carrier.iter().map(|t| unary(Q, t.clone())));
    }
    for x in carrier {
        for y in carrier {
            facts.push(binary(E, x.clone(), y.clone()));
        }
    }
    facts
}

fn identity_rules() -> Vec<Clause> {
    use Predicate::*;
    let (x, y, l) = (Term::Var(0), Term::Var(1), Term::Var(2));
    let u = |p, t: &Term| unary(p, t.clone());
    let e = |a: &Term, b: &Term| binary(E, a.clone(), b.clone());
    let i = |t: &Term| identity(1, vec![t.clone()]);
    let j = |t: &Term| identity(2, vec![t.clone()]);
    vec![
        clause(u(F, &i(&x)), vec![positive(u(P, &x))]),
        clause(u(F, &i(&x)), vec![positive(e(&x, &y)), positive(u(P, &y))]),
        clause(u(G, &j(&x)), vec![positive(u(F, &x))]),
        clause(u(Q, &x), vec![positive(u(F, &i(&x)))]),
        clause(
            u(R, &x),
            vec![positive(u(D, &x)), negative(vec![u(F, &i(&x))])],
        ),
        clause(
            u(R, &x),
            vec![positive(u(D, &x)), negative(vec![u(F, &i(&l)), e(&x, &l)])],
        ),
        clause(
            binary(H, x.clone(), y.clone()),
            vec![positive(u(F, &x)), positive(u(G, &y))],
        ),
        clause(u(G, &j(&x)), vec![positive(u(P, &x))]),
        clause(
            u(F, &identity(3, vec![x.clone(), y.clone()])),
            vec![positive(e(&x, &y))],
        ),
    ]
}

fn catalogue_cases(family: &'static str, emit: &mut impl FnMut(Program)) {
    let mut index = 0;
    if family == "G" {
        let catalogue = ground_catalogue();
        for indices in subsets(catalogue.len(), 3) {
            let extra = indices
                .iter()
                .map(|i| catalogue[*i].clone())
                .collect::<Vec<_>>();
            if ground_stratified(&extra) {
                append_program(family, &mut index, &[atom("a"), atom("b")], extra, emit);
            }
        }
    } else {
        let is_identity = family == "I";
        let catalogue = if is_identity {
            identity_rules()
        } else {
            variable_rules()
        };
        let choices = subsets(catalogue.len(), if is_identity { 3 } else { 2 });
        for variant in 0..if is_identity { 2 } else { 1 } {
            let carrier = [
                atom("a"),
                if variant == 0 {
                    atom("b")
                } else {
                    identity(0, vec![])
                },
            ];
            let edb = variable_edb(&carrier, !is_identity);
            for mask in 0..1 << edb.len() {
                for selected in &choices {
                    let mut extra = edb
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << i) != 0)
                        .map(|(_, f)| clause(f.clone(), vec![]))
                        .collect::<Vec<_>>();
                    extra.extend(selected.iter().map(|i| catalogue[*i].clone()));
                    append_program(family, &mut index, &carrier, extra, emit);
                }
            }
        }
    }
}

fn numeric_rule(
    family: &'static str,
    index: &mut usize,
    head: Literal,
    body: Goal,
    emit: &mut impl FnMut(Program),
) {
    append_program(
        family,
        index,
        &[atom("a"), atom("b")],
        vec![clause(head, vec![body])],
        emit,
    );
}

fn comparison_cases(family: &'static str, emit: &mut impl FnMut(Program)) {
    let mut index = 0;
    if family == "C4" {
        for left in pairs(&C8, &n7()) {
            for right in pairs(&C5, &n7()) {
                for naf in [false, true] {
                    let call = Goal::Builtin(
                        BuiltinKind::Satisfies4,
                        [left.clone(), right.clone()].concat(),
                    );
                    numeric_rule(
                        family,
                        &mut index,
                        unary(Predicate::R, atom("a")),
                        if naf {
                            Goal::Negative(vec![call])
                        } else {
                            call
                        },
                        emit,
                    );
                }
            }
        }
    } else {
        for s1 in C8 {
            for n in [-1, 0, 1] {
                for s2 in C5 {
                    for delta in [-1, 0, 1] {
                        for u1 in UNITS {
                            for u2 in UNITS {
                                let n = Rational::new(n, 1);
                                let m = n
                                    .times(conversion(u1, u2).unwrap_or(Rational::new(1, 1)))
                                    .plus(Rational::new(delta, 1));
                                numeric_rule(
                                    family,
                                    &mut index,
                                    unary(Predicate::R, atom("a")),
                                    Goal::Builtin(
                                        BuiltinKind::Satisfies6,
                                        vec![
                                            atom(s1),
                                            Term::Number(n),
                                            atom(u1),
                                            atom(s2),
                                            Term::Number(m),
                                            atom(u2),
                                        ],
                                    ),
                                    emit,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn arithmetic_cases(family: &'static str, emit: &mut impl FnMut(Program)) {
    let mut index = 0;
    let operands = if family == "A-free" {
        pairs(&C8, &n7())
    } else {
        pairs(&C3, &n3())
    };
    let outputs = pairs(&C8, &n7());
    for op in OPS {
        for left in &operands {
            for right in &operands {
                let inputs = [left.clone(), right.clone()].concat();
                let mut emit_output = |output: Vec<Term>, naf: bool| {
                    let mut args = inputs.clone();
                    args.extend(output.clone());
                    let call = Goal::Builtin(op, args);
                    numeric_rule(
                        family,
                        &mut index,
                        if naf || family == "A-bound" {
                            unary(Predicate::R, atom("a"))
                        } else {
                            literal(Predicate::V, output)
                        },
                        if naf {
                            Goal::Negative(vec![call])
                        } else {
                            call
                        },
                        emit,
                    );
                };
                if family == "A-free" {
                    emit_output(vec![Term::Var(4), Term::Var(5)], false);
                    emit_output(vec![Term::Var(4), Term::Var(5)], true);
                } else if family == "A-bound" {
                    for output in &outputs {
                        emit_output(output.clone(), false);
                    }
                } else {
                    for s in C8 {
                        emit_output(vec![atom(s), Term::Var(5)], false);
                    }
                    for n in n7() {
                        emit_output(vec![Term::Var(4), n], false);
                    }
                }
            }
        }
    }
}

fn shape_cases(emit: &mut impl FnMut(Program)) {
    let mut index = 0;
    let seeds = [
        (
            BuiltinKind::Satisfies4,
            vec![atom("exactly"), number(1), atom("geq"), number(0)],
        ),
        (
            BuiltinKind::Satisfies6,
            vec![
                atom("exactly"),
                number(1),
                atom("day"),
                atom("geq"),
                number(0),
                atom("day"),
            ],
        ),
        (
            BuiltinKind::Product,
            vec![
                atom("exactly"),
                number(1),
                atom("exactly"),
                number(1),
                atom("exactly"),
                number(1),
            ],
        ),
        (
            BuiltinKind::Quotient,
            vec![
                atom("exactly"),
                number(1),
                atom("exactly"),
                number(1),
                atom("exactly"),
                number(1),
            ],
        ),
    ];
    for wrong in [atom("a"), identity(0, vec![])] {
        for (kind, args) in &seeds {
            for position in 0..args.len() {
                let mut mutated = args.clone();
                mutated[position] = wrong.clone();
                numeric_rule(
                    "B-shape",
                    &mut index,
                    unary(Predicate::R, atom("a")),
                    Goal::Builtin(*kind, mutated),
                    emit,
                );
            }
        }
    }
}

fn refusal_cases(emit: &mut impl FnMut(Program)) {
    use Predicate::*;
    let (a, b, x, l) = (atom("a"), atom("b"), Term::Var(0), Term::Var(2));
    let u = |p, t: &Term| unary(p, t.clone());
    let pa = u(P, &a);
    let qa = u(Q, &a);
    let ra = u(R, &a);
    let cases = vec![
        (
            vec![clause(pa.clone(), vec![negative(vec![pa.clone()])])],
            vec![pa.clone()],
        ),
        (
            vec![
                clause(pa.clone(), vec![negative(vec![qa.clone()])]),
                clause(qa.clone(), vec![negative(vec![pa.clone()])]),
            ],
            vec![pa.clone(), qa.clone()],
        ),
        (
            vec![
                clause(pa.clone(), vec![positive(qa.clone())]),
                clause(qa.clone(), vec![negative(vec![pa.clone()])]),
            ],
            vec![pa.clone(), qa.clone()],
        ),
        (
            vec![
                clause(pa.clone(), vec![negative(vec![pa.clone(), qa.clone()])]),
                clause(qa.clone(), vec![positive(pa.clone())]),
            ],
            vec![pa.clone(), qa.clone()],
        ),
        (
            vec![
                clause(binary(E, a.clone(), a.clone()), vec![]),
                clause(binary(E, b.clone(), b.clone()), vec![]),
                clause(
                    u(P, &x),
                    vec![
                        positive(u(D, &x)),
                        negative(vec![binary(E, x.clone(), l.clone()), u(Q, &l)]),
                    ],
                ),
                clause(u(Q, &x), vec![positive(u(P, &x))]),
            ],
            vec![pa.clone(), u(P, &b), qa.clone(), u(Q, &b)],
        ),
        (
            vec![
                clause(u(P, &x), vec![positive(u(D, &x)), negative(vec![u(Q, &x)])]),
                clause(u(Q, &x), vec![positive(u(D, &x)), negative(vec![u(P, &x)])]),
            ],
            vec![pa.clone(), u(P, &b), qa.clone(), u(Q, &b)],
        ),
        (
            vec![
                clause(
                    pa.clone(),
                    vec![
                        Goal::Builtin(
                            BuiltinKind::Product,
                            vec![
                                atom("exactly"),
                                number(1),
                                atom("exactly"),
                                number(1),
                                atom("exactly"),
                                number(1),
                            ],
                        ),
                        negative(vec![qa.clone()]),
                    ],
                ),
                clause(qa.clone(), vec![negative(vec![pa.clone()])]),
            ],
            vec![pa.clone(), qa.clone()],
        ),
        (
            vec![
                clause(
                    pa.clone(),
                    vec![negative(vec![qa.clone()]), negative(vec![ra.clone()])],
                ),
                clause(qa.clone(), vec![negative(vec![pa.clone()])]),
                clause(ra.clone(), vec![positive(ra)]),
            ],
            vec![pa, qa],
        ),
    ];
    for (index, (extra, undefined)) in cases.into_iter().enumerate() {
        let mut p = Program::new("NC", index, &[a.clone(), b.clone()]);
        p.refused = true;
        p.expected_undefined = undefined.into_iter().collect::<BTreeSet<_>>();
        p.clauses.extend(extra);
        emit(p);
    }
}

pub fn enumerate(family: &'static str, mut emit: impl FnMut(Program)) {
    match family {
        "G" | "V" | "I" => catalogue_cases(family, &mut emit),
        "C4" | "C6" => comparison_cases(family, &mut emit),
        "A-free" | "A-bound" | "A-partial" => arithmetic_cases(family, &mut emit),
        "B-shape" => shape_cases(&mut emit),
        "NC" => refusal_cases(&mut emit),
        _ => panic!("family not in frozen manifest: {family}"),
    }
}
