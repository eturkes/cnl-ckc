use crate::k2_engine::{EBodyItem, EClause, EROut, solve};
use crate::k2_index::{EClass, EClauseIndex, build_index, next_candidate};
use crate::k2_term::{ENodeKind, ETermArena, push_atom, push_comp, push_int, push_nil, push_var};
use std::path::PathBuf;
use vstd::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Term {
    Var(usize),
    Int(i64),
    Nil,
    Atom(Vec<u8>),
    Comp(Vec<u8>, Vec<Term>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Class {
    Atom(Vec<u8>),
    Comp(Vec<u8>, usize),
}

fn atom(name: &str) -> Term {
    Term::Atom(name.as_bytes().to_vec())
}

fn comp(name: &str, args: Vec<Term>) -> Term {
    assert!(!args.is_empty());
    Term::Comp(name.as_bytes().to_vec(), args)
}

fn p(first: Term) -> Term {
    comp("p", vec![first])
}

fn class(term: &Term) -> Option<Class> {
    match term {
        Term::Atom(name) => Some(Class::Atom(name.clone())),
        Term::Comp(name, args) => Some(Class::Comp(name.clone(), args.len())),
        _ => None,
    }
}

// D2, evaluated from the independent term model at each original position.
fn candidate(head: &Term, name: &[u8], args: &[Term]) -> bool {
    let Term::Comp(hname, hargs) = head else {
        return false;
    };
    if hname != name || hargs.len() != args.len() {
        return false;
    }
    match (args.first().and_then(class), hargs.first().and_then(class)) {
        (Some(goal), Some(head)) => goal == head,
        _ => true,
    }
}

fn linear(heads: &[Term], name: &[u8], args: &[Term], from: usize) -> Option<usize> {
    heads
        .iter()
        .enumerate()
        .skip(from)
        .find_map(|(j, head)| candidate(head, name, args).then_some(j))
}

verus! {

fn integer_root(
    arena: &mut ETermArena,
    value: i64,
    spelling: Vec<u8>,
    magnitude: Vec<u8>,
    negative: bool,
) -> usize {
    push_int(arena, spelling, magnitude, negative, Ghost(value as int))
}

} // verus!
fn put(arena: &mut ETermArena, term: &Term) -> usize {
    match term {
        Term::Var(key) => {
            let mut spelling = vec![b'A' + (key % 26) as u8];
            if key / 26 != 0 {
                spelling.extend((key / 26).to_string().bytes());
            }
            push_var(arena, *key, spelling)
        }
        Term::Int(value) => integer_root(
            arena,
            *value,
            value.to_string().into_bytes(),
            value.unsigned_abs().to_string().into_bytes(),
            *value < 0,
        ),
        Term::Nil => push_nil(arena),
        Term::Atom(name) => push_atom(arena, name.clone()),
        Term::Comp(name, args) => {
            let roots = args.iter().map(|t| put(arena, t)).collect();
            push_comp(arena, name.clone(), roots)
        }
    }
}

fn get(arena: &ETermArena, root: usize) -> Term {
    match &arena.nodes[root].kind {
        ENodeKind::Var { key, .. } => Term::Var(*key),
        ENodeKind::Int { spelling, .. } => {
            Term::Int(std::str::from_utf8(spelling).unwrap().parse().unwrap())
        }
        ENodeKind::Nil => Term::Nil,
        ENodeKind::Atom { name } => Term::Atom(name.clone()),
        ENodeKind::Comp {
            name, child_roots, ..
        } => Term::Comp(
            name.clone(),
            child_roots.iter().map(|&r| get(arena, r)).collect(),
        ),
    }
}

#[derive(Default)]
struct Checks {
    count: usize,
    failed: usize,
    examples: Vec<String>,
}

impl Checks {
    fn equal<T: std::fmt::Debug + PartialEq>(&mut self, got: T, want: T, label: &str) {
        self.count += 1;
        if got != want {
            self.failed += 1;
            if self.examples.len() < 6 {
                self.examples
                    .push(format!("{label}: got {got:?}, expected {want:?}"));
            }
        }
    }

    fn finish(self) {
        assert_eq!(
            self.failed,
            0,
            "{} / {} checks failed\n{}",
            self.failed,
            self.count,
            self.examples.join("\n")
        );
    }
}

fn class_value(class: &EClass) -> Class {
    match class {
        EClass::Atom(name) => Class::Atom(name.clone()),
        EClass::Comp(name, arity) => Class::Comp(name.clone(), *arity),
    }
}

fn index_exact(checks: &mut Checks, heads: &[Term], ix: &EClauseIndex) {
    for pred in &ix.preds {
        let all: Vec<usize> = heads
            .iter()
            .enumerate()
            .filter_map(|(j, head)| match head {
                Term::Comp(name, args) if *name == pred.name && args.len() == pred.arity => Some(j),
                _ => None,
            })
            .collect();
        let first = |j: usize| match &heads[j] {
            Term::Comp(_, args) => args.first().and_then(class),
            _ => None,
        };
        let wild: Vec<usize> = all
            .iter()
            .copied()
            .filter(|&j| first(j).is_none())
            .collect();
        checks.equal(&pred.all, &all, "D3 all exact/increasing");
        checks.equal(&pred.wild, &wild, "D3 wild exact/increasing");
        for list in &pred.classes {
            let want: Vec<usize> = all
                .iter()
                .copied()
                .filter(|&j| first(j) == Some(class_value(&list.class)))
                .collect();
            checks.equal(&list.list, &want, "D3 class exact/increasing");
        }
        for j in all {
            if let Some(class) = first(j) {
                checks.equal(
                    pred.classes.iter().any(|p| class_value(&p.class) == class),
                    true,
                    "D3 class coverage",
                );
            }
        }
    }
    for head in heads {
        if let Term::Comp(name, args) = head {
            checks.equal(
                ix.preds
                    .iter()
                    .any(|p| p.name == *name && p.arity == args.len()),
                true,
                "D3 predicate coverage",
            );
        }
    }
}

fn check_db(checks: &mut Checks, heads: &[Term], goals: &[(Vec<u8>, Vec<Term>)], fronts: &[usize]) {
    let mut arena = ETermArena { nodes: vec![] };
    let db: Vec<EClause> = heads
        .iter()
        .map(|head| EClause {
            head: put(&mut arena, head),
            body: vec![],
        })
        .collect();
    for &front in fronts {
        let suffix: Vec<EClause> = db[front..]
            .iter()
            .map(|c| EClause {
                head: c.head,
                body: vec![],
            })
            .collect();
        let ix = build_index(&arena, &suffix);
        // Goal allocation grows the arena after index construction (D5).
        for (name, args) in goals {
            let roots: Vec<usize> = args.iter().map(|t| put(&mut arena, t)).collect();
            for from in (0..=heads.len() + 1).chain(std::iter::once(usize::MAX)) {
                checks.equal(
                    next_candidate(&arena, &db, &ix, front, name, &roots, from),
                    linear(heads, name, args, from),
                    &format!("front={front} from={from} goal={name:?}/{args:?}"),
                );
            }
        }
        index_exact(checks, &heads[front..], &ix);
    }
}

fn goals(name: &str, firsts: &[Term]) -> Vec<(Vec<u8>, Vec<Term>)> {
    firsts
        .iter()
        .map(|t| (name.as_bytes().to_vec(), vec![t.clone()]))
        .collect()
}

fn domain() -> Vec<Term> {
    vec![
        Term::Var(0),
        Term::Var(52),
        Term::Int(i64::MIN),
        Term::Int(-1),
        Term::Int(0),
        Term::Int(i64::MAX),
        Term::Nil,
        atom("alpha"),
        atom("beta"),
        atom(""),
        atom("a\0b"),
        atom("λ"),
        comp("f", vec![atom("alpha")]),
        comp("f", vec![atom("beta")]),
        comp("f", vec![Term::Var(1)]),
        comp("f", vec![atom("alpha"), atom("beta")]),
        comp("g", vec![atom("alpha")]),
        comp("", vec![Term::Nil]),
        comp("[|]", vec![atom("alpha"), Term::Nil]),
        comp("{}", vec![atom("alpha")]),
    ]
}

#[test]
fn b01_every_resume_position() {
    let firsts = domain();
    let heads: Vec<Term> = firsts.iter().cloned().map(p).collect();
    let mut checks = Checks::default();
    check_db(&mut checks, &heads, &goals("p", &firsts), &[0]);
    checks.finish();
}

#[test]
fn b02_every_prefix_boundary_generated() {
    let alphabet = [
        p(atom("alpha")),
        p(atom("beta")),
        p(Term::Var(0)),
        p(Term::Int(7)),
        p(Term::Nil),
        p(comp("f", vec![atom("alpha")])),
        p(comp("f", vec![atom("alpha"), atom("beta")])),
        comp("q", vec![atom("alpha")]),
        atom("p"),
    ];
    let mut queries = goals("p", &domain());
    queries.push((b"p".to_vec(), vec![]));
    queries.push((b"q".to_vec(), vec![atom("alpha")]));
    queries.push((b"missing".to_vec(), vec![Term::Var(0)]));
    let mut checks = Checks::default();
    for len in 0..=3 {
        for mut code in 0..alphabet.len().pow(len as u32) {
            let heads: Vec<Term> = (0..len)
                .map(|_| {
                    let t = alphabet[code % alphabet.len()].clone();
                    code /= alphabet.len();
                    t
                })
                .collect();
            check_db(
                &mut checks,
                &heads,
                &queries,
                &(0..=len).collect::<Vec<_>>(),
            );
        }
    }
    checks.finish();
}

#[test]
fn b03_empty_database_prefix_suffix() {
    let queries = goals("p", &[atom("alpha"), Term::Var(0), Term::Nil]);
    let mut checks = Checks::default();
    check_db(&mut checks, &[], &queries, &[0]);
    check_db(&mut checks, &[p(atom("alpha"))], &queries, &[0, 1]);
    check_db(
        &mut checks,
        &[p(atom("alpha")), p(Term::Var(0))],
        &queries,
        &[0, 1, 2],
    );
    checks.finish();
}

#[test]
fn b04_missing_predicate_and_class() {
    let mut queries = goals("p", &[atom("absent"), comp("missing", vec![Term::Nil])]);
    queries.extend(goals("missing", &domain()));
    let mut checks = Checks::default();
    for wild in [vec![], vec![p(Term::Var(0))], vec![p(Term::Nil)]] {
        let mut heads = vec![p(atom("alpha")), comp("q", vec![atom("absent")])];
        heads.extend(wild);
        check_db(
            &mut checks,
            &heads,
            &queries,
            &(0..=heads.len()).collect::<Vec<_>>(),
        );
    }
    checks.finish();
}

#[test]
fn b05_name_and_arity_are_both_keys() {
    let mut checks = Checks::default();
    for name in ["p", "", "a\0b", "λ"] {
        let heads = vec![
            comp(name, vec![atom("alpha")]),
            comp(name, vec![atom("alpha"), Term::Nil]),
            comp("other", vec![atom("alpha")]),
            comp(name, vec![Term::Var(0)]),
            comp(name, vec![Term::Nil, Term::Var(0)]),
        ];
        let queries: Vec<_> = (0..=3)
            .map(|n| (name.as_bytes().to_vec(), vec![atom("alpha"); n]))
            .collect();
        check_db(&mut checks, &heads, &queries, &[0, 1, 2, 3, 4, 5]);
    }
    checks.finish();
}

#[test]
fn b06_noncompound_heads_never_match() {
    let heads = vec![
        atom("p"),
        Term::Var(0),
        Term::Int(7),
        Term::Nil,
        p(atom("alpha")),
        atom("alpha"),
        p(Term::Var(1)),
    ];
    let mut queries = goals("p", &domain());
    queries.push((b"p".to_vec(), vec![]));
    let mut checks = Checks::default();
    check_db(&mut checks, &heads, &queries, &(0..=7).collect::<Vec<_>>());
    checks.finish();
}

#[test]
fn b07_variables_integers_and_nil_are_wild() {
    let heads = vec![
        p(Term::Var(0)),
        p(Term::Int(i64::MIN)),
        p(Term::Nil),
        p(Term::Var(52)),
        p(Term::Int(i64::MAX)),
    ];
    let mut checks = Checks::default();
    check_db(
        &mut checks,
        &heads,
        &goals("p", &domain()),
        &(0..=5).collect::<Vec<_>>(),
    );
    checks.finish();
}

#[test]
fn b08_class_and_wild_lists_merge_in_original_order() {
    let heads = vec![
        p(atom("alpha")),
        p(Term::Var(0)),
        p(atom("beta")),
        p(comp("f", vec![atom("x")])),
        p(Term::Nil),
        comp("q", vec![atom("alpha")]),
        p(atom("alpha")),
        p(Term::Int(7)),
        p(comp("f", vec![atom("y")])),
        p(comp("f", vec![atom("x"), atom("y")])),
        p(Term::Var(1)),
    ];
    let mut checks = Checks::default();
    check_db(
        &mut checks,
        &heads,
        &goals("p", &domain()),
        &(0..=heads.len()).collect::<Vec<_>>(),
    );
    checks.finish();
}

#[test]
fn b09_duplicate_heads_keep_distinct_positions() {
    let mut checks = Checks::default();
    for first in domain() {
        let head = p(first.clone());
        let heads = vec![head.clone(), head.clone(), p(Term::Var(0)), head];
        check_db(
            &mut checks,
            &heads,
            &goals("p", &[first, Term::Var(1)]),
            &[0, 1, 2, 3, 4],
        );
    }
    checks.finish();
}

#[derive(Clone)]
enum Body {
    Pos(Term),
    Naf(Vec<Term>),
}

type Clause = (Term, Vec<Body>);

fn two(first: Term, second: Term) -> Term {
    comp("p", vec![first, second])
}

fn raw_case(name: &str) -> (Vec<Clause>, Term, Vec<Term>) {
    let a = atom("alpha");
    let v = Term::Var(0);
    match name {
        "mixed" => (
            vec![
                (two(a.clone(), atom("zeta")), vec![]),
                (comp("q", vec![a.clone()]), vec![]),
                (two(v.clone(), atom("alpha")), vec![]),
                (two(atom("beta"), atom("clash")), vec![]),
                (two(Term::Int(7), atom("integer")), vec![]),
                (two(Term::Nil, atom("nil")), vec![]),
                (two(a.clone(), atom("zeta")), vec![]),
                (two(a.clone(), atom("mu")), vec![]),
            ],
            two(a, v.clone()),
            vec![v],
        ),
        "occurs" => {
            let f = comp("f", vec![v.clone()]);
            let z = comp("f", vec![atom("z")]);
            (
                vec![
                    (two(comp("g", vec![a.clone()]), a.clone()), vec![]),
                    (two(f.clone(), v.clone()), vec![]),
                    (two(z.clone(), z.clone()), vec![]),
                    (two(z.clone(), z), vec![]),
                ],
                two(f.clone(), f),
                vec![v],
            )
        }
        "chain" => (
            vec![
                (p(a.clone()), vec![Body::Pos(p(atom("beta")))]),
                (p(atom("beta")), vec![Body::Pos(p(atom("gamma")))]),
                (p(atom("gamma")), vec![]),
                (p(a.clone()), vec![]),
            ],
            p(a.clone()),
            vec![a],
        ),
        "recursive" => (
            vec![
                (two(a.clone(), atom("zeta")), vec![]),
                (
                    two(a.clone(), v.clone()),
                    vec![Body::Pos(two(a.clone(), v.clone()))],
                ),
                (two(a.clone(), atom("alpha")), vec![]),
            ],
            two(a, v.clone()),
            vec![v],
        ),
        "negation" => (
            vec![
                (p(a.clone()), vec![Body::Naf(vec![p(atom("beta"))])]),
                (p(atom("beta")), vec![Body::Pos(p(atom("gamma")))]),
                (p(atom("gamma")), vec![Body::Pos(p(atom("delta")))]),
                (p(a.clone()), vec![]),
            ],
            p(a.clone()),
            vec![a],
        ),
        "eso" => {
            let operator = |inner| {
                comp(
                    "guideline_operator",
                    vec![atom("actual"), inner, atom("may")],
                )
            };
            let content = |first, second| {
                comp(
                    "guideline_property",
                    vec![first, second, atom("item"), atom("pos")],
                )
            };
            let f = comp("f", vec![atom("box")]);
            let g = comp("g", vec![atom("box")]);
            (
                vec![
                    (operator(f.clone()), vec![]),
                    (operator(g.clone()), vec![]),
                    (content(g, atom("zeta")), vec![]),
                    (content(f, atom("alpha")), vec![]),
                ],
                comp(",", vec![operator(v.clone()), content(v, Term::Var(1))]),
                vec![Term::Var(1)],
            )
        }
        _ => panic!("unlisted raw case: {name}"),
    }
}

fn term_text(term: &Term) -> String {
    match term {
        Term::Var(key) => format!("v{key}"),
        Term::Int(value) => value.to_string(),
        Term::Nil => "[]".into(),
        Term::Atom(name) => String::from_utf8(name.clone()).unwrap(),
        Term::Comp(name, args) => format!(
            "{}({})",
            String::from_utf8(name.clone()).unwrap(),
            args.iter().map(term_text).collect::<Vec<_>>().join(",")
        ),
    }
}

fn raw_output(name: &str, depth: usize, fuel: usize, collect: bool) -> String {
    let (clauses, goal, projection) = raw_case(name);
    let mut arena = ETermArena { nodes: vec![] };
    let db: Vec<EClause> = clauses
        .iter()
        .map(|(head, body)| EClause {
            head: put(&mut arena, head),
            body: body
                .iter()
                .map(|item| match item {
                    Body::Pos(t) => EBodyItem::Pos {
                        root: put(&mut arena, t),
                    },
                    Body::Naf(ts) => EBodyItem::Naf {
                        roots: ts.iter().map(|t| put(&mut arena, t)).collect(),
                    },
                })
                .collect(),
        })
        .collect();
    let goal = put(&mut arena, &goal);
    let projection = projection.iter().map(|t| put(&mut arena, t)).collect();
    match solve(&mut arena, &db, goal, depth, projection, collect, fuel) {
        EROut::Sol => "sol".into(),
        EROut::End { complete, rows } => format!(
            "end({complete},[{}])",
            rows.iter()
                .map(|row| format!(
                    "[{}]",
                    row.iter()
                        .map(|&root| term_text(&get(&arena, root)))
                        .collect::<Vec<_>>()
                        .join(",")
                ))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

#[test]
fn b10_raw_solve_order_duplicates_and_bounds() {
    let mut actual = String::from("case\tdepth\tfuel\tcollect\tresult\n");
    for name in ["mixed", "occurs", "chain", "recursive", "negation", "eso"] {
        for depth in [0, 1, 2, 3, 8] {
            for fuel in [0, 1, 2, 3, 4, 5, 8, 12, 24, 32] {
                for collect in [false, true] {
                    actual.push_str(&format!(
                        "{name}\t{depth}\t{fuel}\t{collect}\t{}\n",
                        raw_output(name, depth, fuel, collect)
                    ));
                }
            }
        }
    }
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/cix/raw-solve.tsv");
    let expected = std::fs::read_to_string(file).unwrap();
    assert_eq!(actual.lines().count(), 601);
    assert_eq!(expected.lines().count(), 601);
    for (line, (actual, expected)) in actual.lines().zip(expected.lines()).enumerate() {
        assert_eq!(actual, expected, "raw solve pin line {}", line + 1);
    }
}
