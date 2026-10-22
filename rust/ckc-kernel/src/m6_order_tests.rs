use crate::k2_term::{ENodeKind, ETermArena, push_atom, push_comp, push_int, push_nil, push_var};
use crate::m6_model::Body;
use crate::m6_term::from_root;
use std::collections::BTreeSet;
use vstd::prelude::*;

#[path = "../../../tests/bf/model.rs"]
mod model;
use model::{INDICATORS, Item, Term};

fn atom(s: &str) -> Term {
    Term::Atom(s.into())
}
fn comp(s: &str, args: Vec<Term>) -> Term {
    Term::Comp(s.into(), args)
}
fn entity(v: usize) -> Item {
    Item::Pos(comp(
        "guideline_entity",
        vec![
            atom("actual"),
            Term::Var(v),
            atom("patient"),
            atom("countable"),
        ],
    ))
}
fn property(v: usize, value: Term) -> Item {
    Item::Pos(comp(
        "guideline_property",
        vec![atom("actual"), Term::Var(v), value, atom("pos")],
    ))
}
fn naf(v: usize) -> Item {
    Item::Naf(vec![comp("p", vec![Term::Var(v)])])
}

verus! {

fn integer(arena: &mut ETermArena, n: i64, text: Vec<u8>, mag: Vec<u8>) -> usize {
    push_int(arena, text, mag, n < 0, Ghost(n as int))
}

} // verus!
fn put(arena: &mut ETermArena, t: &Term) -> usize {
    match t {
        Term::Var(v) => push_var(arena, *v, format!("V{v}").into_bytes()),
        Term::Atom(n) => push_atom(arena, n.as_bytes().to_vec()),
        Term::Int(n) => integer(
            arena,
            *n,
            n.to_string().into_bytes(),
            n.unsigned_abs().to_string().into_bytes(),
        ),
        Term::Nil => push_nil(arena),
        Term::Comp(n, a) => {
            let roots = a.iter().map(|t| put(arena, t)).collect();
            push_comp(arena, n.as_bytes().to_vec(), roots)
        }
    }
}
fn get(arena: &ETermArena, root: usize) -> Term {
    match &arena.nodes[root].kind {
        ENodeKind::Var { key, .. } => Term::Var(*key),
        ENodeKind::Atom { name } => Term::Atom(String::from_utf8(name.clone()).unwrap()),
        ENodeKind::Int { spelling, .. } => {
            Term::Int(std::str::from_utf8(spelling).unwrap().parse().unwrap())
        }
        ENodeKind::Nil => Term::Nil,
        ENodeKind::Comp {
            name, child_roots, ..
        } => comp(
            std::str::from_utf8(name).unwrap(),
            child_roots.iter().map(|&r| get(arena, r)).collect(),
        ),
    }
}
fn execute(input: &[Item]) -> Vec<Item> {
    let mut arena = ETermArena { nodes: vec![] };
    let body = input
        .iter()
        .map(|item| match item {
            Item::Pos(t) => {
                let root = put(&mut arena, t);
                Body::Pos(from_root(&arena, root))
            }
            Item::Naf(ts) => Body::Naf(
                ts.iter()
                    .map(|t| {
                        let root = put(&mut arena, t);
                        from_root(&arena, root)
                    })
                    .collect(),
            ),
        })
        .collect();
    let before = arena.nodes.len();
    let result = crate::m6_order::bind_first(&arena, body);
    assert_eq!(arena.nodes.len(), before, "scheduling changes no term");
    result
        .iter()
        .map(|item| match item {
            Body::Pos(t) => Item::Pos(get(&arena, t.root)),
            Body::Naf(ts) => Item::Naf(ts.iter().map(|t| get(&arena, t.root)).collect()),
        })
        .collect()
}

#[derive(Default)]
struct Checks {
    total: usize,
    failed: usize,
    examples: Vec<String>,
}
impl Checks {
    fn equal(&mut self, input: &[Item]) {
        self.total += 1;
        let (got, want) = (execute(input), model::ordered(input));
        if got != want {
            self.failed += 1;
            if self.examples.len() < 3 {
                self.examples
                    .push(format!("input={input:?}\ngot={got:?}\nwant={want:?}"));
            }
        }
    }
    fn finish(self) {
        assert_eq!(
            self.failed,
            0,
            "{} / {} differential cases failed\n{}",
            self.failed,
            self.total,
            self.examples.join("\n")
        );
    }
}

fn permutations<T: Clone>(input: &[T]) -> Vec<Vec<T>> {
    if input.is_empty() {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    for i in 0..input.len() {
        let mut rest = input.to_vec();
        let first = rest.remove(i);
        for mut tail in permutations(&rest) {
            tail.insert(0, first.clone());
            out.push(tail);
        }
    }
    out
}

#[test]
fn all_indicators_and_argument_positions() {
    let mut checks = Checks::default();
    let mut seen = BTreeSet::new();
    for &(name, arity) in &INDICATORS {
        seen.insert((name, arity));
        for position in 0..arity {
            for nested in [false, true] {
                let mut args = vec![atom("constant"); arity];
                args[position] = if nested {
                    comp(
                        "outer",
                        vec![Term::Int(-7), comp("inner", vec![Term::Var(0), Term::Nil])],
                    )
                } else {
                    Term::Var(0)
                };
                let goal = Item::Pos(comp(name, args));
                for body in permutations(&[goal, entity(0), entity(1)]) {
                    checks.equal(&body);
                }
            }
        }
    }
    assert_eq!(seen.len(), 24);
    assert_eq!(checks.total, 1200);
    checks.finish();
}

#[test]
fn builtins_wait_for_every_input_but_not_outputs() {
    let mut checks = Checks::default();
    for &(name, arity) in &INDICATORS[20..] {
        let n = if name == "guideline_satisfies" {
            arity / 2
        } else {
            4
        };
        for absent in 0..n {
            let mut args = vec![atom("exactly"); arity];
            for (j, a) in args.iter_mut().enumerate().take(n) {
                *a = comp("nested", vec![Term::Var(j), Term::Var(j)]);
            }
            for a in &mut args[n..] {
                *a = Term::Var(100);
            }
            let g = Item::Pos(comp(name, args));
            let mut prefix: Vec<_> = (0..n).filter(|&j| j != absent).map(entity).collect();
            prefix.extend([g, entity(absent), property(100, atom("output"))]);
            checks.equal(&prefix);
        }
    }
    checks.finish();
}

#[test]
fn readiness_is_recomputed_from_the_left_after_each_pick() {
    let body = vec![
        property(0, Term::Var(1)),
        property(1, Term::Var(2)),
        entity(0),
        entity(3),
        property(2, atom("last")),
    ];
    assert_eq!(model::order(&body), [2, 0, 1, 3, 4]);
    assert_eq!(execute(&body), model::ordered(&body));
}

#[test]
fn naf_shared_local_and_nested_variables() {
    let mut checks = Checks::default();
    for ts in [
        vec![],
        vec![comp("p", vec![Term::Var(0), Term::Var(90)])],
        vec![
            comp("p", vec![comp("deep", vec![Term::Var(0)])]),
            comp("q", vec![Term::Var(1), Term::Var(91)]),
        ],
    ] {
        for body in permutations(&[Item::Naf(ts), property(0, Term::Var(1)), entity(0), naf(91)]) {
            checks.equal(&body);
        }
    }
    // A fallback NAF binds neither its shared nor its local variables.
    checks.equal(&[
        Item::Naf(vec![comp("p", vec![Term::Var(0), Term::Var(1)])]),
        property(1, atom("waits")),
        property(0, Term::Var(1)),
    ]);
    checks.equal(&[
        naf(0),
        property(2, atom("first")),
        property(0, atom("last")),
    ]);
    checks.finish();
}

fn alphabet() -> Vec<Item> {
    vec![
        entity(0),
        entity(1),
        property(0, Term::Var(1)),
        property(1, Term::Var(0)),
        naf(0),
        naf(80),
        Item::Naf(vec![]),
        Item::Pos(comp(
            "guideline_product",
            vec![
                Term::Var(0),
                Term::Int(1),
                atom("exactly"),
                Term::Int(2),
                Term::Var(1),
                Term::Var(2),
            ],
        )),
        Item::Pos(comp(
            "guideline_satisfies",
            vec![Term::Var(1), Term::Var(2), atom("geq"), Term::Int(3)],
        )),
    ]
}
fn generated(mut check: impl FnMut(&[Item])) -> usize {
    let alphabet = alphabet();
    let mut count = 0;
    for len in 0..=4 {
        for mut code in 0..alphabet.len().pow(len as u32) {
            let body: Vec<_> = (0..len)
                .map(|_| {
                    let item = alphabet[code % alphabet.len()].clone();
                    code /= alphabet.len();
                    item
                })
                .collect();
            check(&body);
            count += 1;
        }
    }
    count
}

#[test]
fn generated_bodies_match_reference_with_duplicates() {
    let mut checks = Checks::default();
    assert_eq!(generated(|b| checks.equal(b)), 7381);
    checks.finish();
}

#[test]
fn sparse_variable_keys_and_long_dependency_chains() {
    let mut checks = Checks::default();
    for key in [0, 1, 255, usize::MAX - 1, usize::MAX] {
        checks.equal(&[property(key, atom("late")), naf(key), entity(key)]);
    }
    for len in [1, 2, 31, 128, 257] {
        let mut body: Vec<_> = (0..len).map(|i| property(i, Term::Var(i + 1))).collect();
        body.push(entity(0));
        checks.equal(&body);
    }
    checks.finish();
}

#[test]
fn permutation_preserves_multiplicity_and_terms() {
    assert_eq!(
        generated(|body| {
            let mut want = body.to_vec();
            let mut got = execute(body);
            want.sort();
            got.sort();
            assert_eq!(got, want);
        }),
        7381
    );
}

fn positions_ready(body: &[Item], moded_only: bool) -> bool {
    let shared = model::positive_vars(body);
    let mut bound = BTreeSet::new();
    for item in body {
        let moded = match item {
            Item::Pos(t) => model::builtin(t),
            Item::Naf(_) => true,
        };
        if (!moded_only || moded) && !model::ready(item, &bound, &shared) {
            return false;
        }
        if let Item::Pos(t) = item {
            bound.extend(model::vars(t));
        }
    }
    true
}

#[test]
fn already_ready_bodies_are_stable() {
    let mut cases = 0;
    generated(|body| {
        if positions_ready(body, false) {
            assert_eq!(execute(body), body);
            cases += 1;
        }
    });
    assert!(cases > 1000, "stability cases={cases}");
}

#[test]
fn safe_moded_bodies_keep_all_builtin_and_naf_inputs_ready() {
    let mut cases = 0;
    generated(|body| {
        if positions_ready(body, true) {
            assert!(positions_ready(&execute(body), true), "body={body:?}");
            cases += 1;
        }
    });
    assert!(cases > 1000, "moded cases={cases}");
}

#[test]
fn fallback_is_leftmost_and_then_resumes_ready_selection() {
    let body = vec![
        property(0, Term::Var(1)),
        property(2, Term::Var(3)),
        property(1, Term::Var(2)),
    ];
    assert_eq!(model::order(&body), [0, 2, 1]);
    assert_eq!(execute(&body), model::ordered(&body));
    for body in [
        vec![],
        vec![naf(0)],
        vec![property(0, atom("x"))],
        vec![property(0, atom("x")), property(1, atom("y"))],
    ] {
        assert_eq!(execute(&body), body);
    }
}

#[test]
fn nonsemantic_names_wrong_arities_and_noncompound_terms_are_ready() {
    let mut checks = Checks::default();
    for term in [
        atom("guideline_operator"),
        Term::Var(0),
        Term::Nil,
        Term::Int(i64::MIN),
        comp("foreign", vec![Term::Var(0), Term::Var(1)]),
    ] {
        let body = vec![Item::Pos(term), property(1, atom("x")), entity(1)];
        checks.equal(&body);
    }
    for &(name, arity) in &INDICATORS {
        for wrong in [arity + 1, arity + 4] {
            let body = vec![
                Item::Pos(comp(name, vec![Term::Var(0); wrong])),
                property(1, atom("x")),
                entity(1),
            ];
            checks.equal(&body);
        }
    }
    checks.finish();
}
