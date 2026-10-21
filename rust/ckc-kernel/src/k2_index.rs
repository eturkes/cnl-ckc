// Clause index (contract cix): the K2 engine's candidate clauses by predicate +
// first-argument class, built once per composition. Every clause the index
// skips makes the spec's `unify` fail (D2), so `call` sees the spec's result.
use crate::k2_engine::{EClause, literal_matches, vec_equal};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{
    clause_valid, clause_view, db_valid, db_view, node_comp_model, root_terms, roots_valid,
};
use crate::k2_term::{ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, child_terms, node_ok, root_ok};
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::{args_of, lit_fa};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::v1text::DocClause;
use std::collections::BTreeMap;
use vstd::prelude::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};
// --- classes + candidates (D1, D2) ---
// Atoms by name, compounds by name + arity; variables, integers and [] stay
// classless, so a classless head joins every candidate set.

pub ghost enum SClass {
    Atom(Seq<u8>),
    Comp(Seq<u8>, nat),
}

pub open spec fn class_of(t: Term) -> Option<SClass> {
    match t {
        Term::Atom(name) => Some(SClass::Atom(name)),
        Term::Comp(name, args) => Some(SClass::Comp(name, args.len())),
        _ => None,
    }
}

pub open spec fn first_class(args: Seq<Term>) -> Option<SClass> {
    if args.len() > 0 {
        class_of(args[0])
    } else {
        None
    }
}

pub open spec fn head_class(c: DocClause) -> Option<SClass> {
    first_class(args_of(c.head))
}

// A clause of the goal's predicate stays a candidate unless both first
// arguments carry a class and the classes differ.
pub open spec fn candidate(c: DocClause, name: Seq<u8>, arity: nat, goal: Option<SClass>) -> bool {
    &&& lit_fa(c.head) == Some((name, arity))
    &&& (goal is None || head_class(c) is None || head_class(c) == goal)
}

// --- lists (D3) ---
pub ghost enum Sel {
    All,
    Wild,
    Class(SClass),
}

pub open spec fn selects(c: DocClause, name: Seq<u8>, arity: nat, sel: Sel) -> bool {
    &&& lit_fa(c.head) == Some((name, arity))
    &&& match sel {
        Sel::All => true,
        Sel::Wild => head_class(c) is None,
        Sel::Class(k) => head_class(c) == Some(k),
    }
}

pub open spec fn increasing(s: Seq<usize>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] < s[j]
}

// `list` = the positions below `n` of the clauses of `b` that `sel` selects, ascending.
pub open spec fn exact(
    list: Seq<usize>,
    b: Seq<DocClause>,
    n: nat,
    name: Seq<u8>,
    arity: nat,
    sel: Sel,
) -> bool {
    &&& n <= b.len() <= usize::MAX
    &&& increasing(list)
    &&& forall|i: int|
        0 <= i < list.len() ==> (#[trigger] list[i] as int) < n && selects(
            b[list[i] as int],
            name,
            arity,
            sel,
        )
    &&& forall|j: int|
        0 <= j < n && #[trigger] selects(b[j], name, arity, sel) ==> list.contains(j as usize)
}

pub enum EClass {
    Atom(Vec<u8>),
    Comp(Vec<u8>, usize),
}

pub open spec fn class_view(c: EClass) -> SClass {
    match c {
        EClass::Atom(name) => SClass::Atom(name@),
        EClass::Comp(name, arity) => SClass::Comp(name@, arity as nat),
    }
}

pub struct EClassList {
    pub class: EClass,
    pub list: Vec<usize>,
}

pub struct EPredIndex {
    pub name: Vec<u8>,
    pub arity: usize,
    pub all: Vec<usize>,
    pub wild: Vec<usize>,
    pub classes: Vec<EClassList>,
    // class hash -> indices into `classes` (contract cix2 D2)
    pub routes: BTreeMap<u64, Vec<usize>>,
}

pub struct EClauseIndex {
    pub preds: Vec<EPredIndex>,
    // predicate hash -> indices into `preds` (contract cix2 D2)
    pub routes: BTreeMap<u64, Vec<usize>>,
}

// Every index sits exactly once in its own hash's bucket; every bucket entry is
// in range and hashes to its bucket (contract cix2 D2).
pub open spec fn routes_ok(routes: Map<u64, Vec<usize>>, hashes: Seq<u64>) -> bool {
    &&& forall|i: int|
        0 <= i < hashes.len() ==> #[trigger] routes.contains_key(hashes[i])
            && routes[hashes[i]]@.contains(i as usize)
    &&& forall|h: u64, k: int|
        routes.contains_key(h) && 0 <= k < routes[h]@.len() ==> (#[trigger] routes[h]@[k] as int)
            < hashes.len() && hashes[routes[h]@[k] as int] == h
    &&& forall|h: u64| routes.contains_key(h) ==> (#[trigger] routes[h]@).no_duplicates()
}

pub open spec fn class_hashes(p: EPredIndex) -> Seq<u64> {
    p.classes@.map_values(|c: EClassList| class_hash(class_view(c.class)))
}

pub open spec fn pred_hashes(ix: EClauseIndex) -> Seq<u64> {
    ix.preds@.map_values(|e: EPredIndex| pred_hash(e.name@, e.arity as nat))
}

pub open spec fn classes_distinct(p: EPredIndex) -> bool {
    forall|q: int, r: int|
        0 <= q < r < p.classes@.len() ==> class_view(p.classes@[q].class) != class_view(
            p.classes@[r].class,
        )
}

pub open spec fn preds_distinct(ix: EClauseIndex) -> bool {
    forall|q: int, r: int|
        0 <= q < r < ix.preds@.len() ==> (ix.preds@[q].name@, ix.preds@[q].arity) != (
            ix.preds@[r].name@,
            ix.preds@[r].arity,
        )
}

// The entry's lists exact over the first `n` clauses of `b`, every classed clause
// among them in its class list, class keys distinct + routed.
pub open spec fn pred_upto(p: EPredIndex, b: Seq<DocClause>, n: nat) -> bool {
    &&& exact(p.all@, b, n, p.name@, p.arity as nat, Sel::All)
    &&& exact(p.wild@, b, n, p.name@, p.arity as nat, Sel::Wild)
    &&& forall|q: int|
        0 <= q < p.classes@.len() ==> exact(
            (#[trigger] p.classes@[q]).list@,
            b,
            n,
            p.name@,
            p.arity as nat,
            Sel::Class(class_view(p.classes@[q].class)),
        )
    &&& forall|j: int|
        0 <= j < n && #[trigger] selects(b[j], p.name@, p.arity as nat, Sel::All) && head_class(
            b[j],
        ) is Some ==> exists|q: int|
            0 <= q < p.classes@.len() && Some(class_view(p.classes@[q].class)) == head_class(b[j])
    &&& classes_distinct(p)
    &&& routes_ok(p.routes@, class_hashes(p))
}

pub open spec fn pred_over(p: EPredIndex, b: Seq<DocClause>) -> bool {
    pred_upto(p, b, b.len())
}

pub open spec fn index_upto(ix: EClauseIndex, b: Seq<DocClause>, n: nat) -> bool {
    &&& forall|p: int| 0 <= p < ix.preds@.len() ==> pred_upto(#[trigger] ix.preds@[p], b, n)
    &&& forall|j: int|
        0 <= j < n && #[trigger] lit_fa(b[j].head) is Some ==> exists|p: int|
            0 <= p < ix.preds@.len() && lit_fa(b[j].head) == Some(
                (ix.preds@[p].name@, ix.preds@[p].arity as nat),
            )
    &&& preds_distinct(ix)
    &&& routes_ok(ix.routes@, pred_hashes(ix))
}

// Every entry exact over `b`; every compound head has its predicate's entry.
pub open spec fn index_over(ix: EClauseIndex, b: Seq<DocClause>) -> bool {
    index_upto(ix, b, b.len())
}

// The index covers db[front..]; positions below `front` (aggregate witness
// facts) are scanned directly.
pub open spec fn index_valid(ix: EClauseIndex, db: Seq<DocClause>, front: nat) -> bool {
    front <= db.len() && index_over(ix, db.skip(front as int))
}

// --- key hash (contract cix2 D1) ---
// h <- h * 31 + b mod 2^64 per byte: a hash only routes a lookup; the exact key
// comparison decides every match. Equal-length keys whose byte polynomials agree
// collide (`Aa` / `BB`).
pub open spec fn mix(h: u64, b: u8) -> u64 {
    vstd::wrapping::u64_specs::wrapping_add(
        vstd::wrapping::u64_specs::wrapping_mul(h, 31),
        b as u64,
    )
}

pub open spec fn fold_bytes(s: Seq<u8>, h: u64) -> u64
    decreases s.len(),
{
    if s.len() == 0 {
        h
    } else {
        fold_bytes(s.drop_first(), mix(h, s[0]))
    }
}

pub open spec fn pred_hash(name: Seq<u8>, arity: nat) -> u64 {
    fold_bytes(name, ((arity + 1) % 0x1_0000_0000_0000_0000) as u64)
}

pub open spec fn class_hash(c: SClass) -> u64 {
    match c {
        SClass::Atom(name) => fold_bytes(name, 1),
        SClass::Comp(name, arity) => fold_bytes(
            name,
            ((arity + 2) % 0x1_0000_0000_0000_0000) as u64,
        ),
    }
}

fn fold_exec(bytes: &Vec<u8>, seed: u64) -> (h: u64)
    ensures
        h == fold_bytes(bytes@, seed),
{
    let mut h = seed;
    let mut i = 0usize;
    while i < bytes.len()
        invariant
            i <= bytes.len(),
            fold_bytes(bytes@, seed) == fold_bytes(bytes@.skip(i as int), h),
        decreases bytes.len() - i,
    {
        proof {
            assert(bytes@.skip(i as int).drop_first() =~= bytes@.skip(i as int + 1));
            assert(bytes@.skip(i as int)[0] == bytes@[i as int]);
        }
        h = h.wrapping_mul(31).wrapping_add(bytes[i] as u64);
        i += 1;
    }
    proof {
        assert(bytes@.skip(i as int) =~= Seq::<u8>::empty());
    }
    h
}

pub fn hash_pred(name: &Vec<u8>, arity: usize) -> (h: u64)
    ensures
        h == pred_hash(name@, arity as nat),
{
    fold_exec(name, (arity as u64).wrapping_add(1))
}

pub fn hash_class(c: &EClass) -> (h: u64)
    ensures
        h == class_hash(class_view(*c)),
{
    match c {
        EClass::Atom(name) => fold_exec(name, 1),
        EClass::Comp(name, arity) => fold_exec(name, (*arity as u64).wrapping_add(2)),
    }
}

// --- list lemmas ---
// One more position examined: `n` joins the list iff it is selected.
proof fn exact_step(
    list: Seq<usize>,
    b: Seq<DocClause>,
    n: nat,
    name: Seq<u8>,
    arity: nat,
    sel: Sel,
    hit: bool,
)
    requires
        exact(list, b, n, name, arity, sel),
        n < b.len(),
        hit == selects(b[n as int], name, arity, sel),
    ensures
        exact(
            if hit {
                list.push(n as usize)
            } else {
                list
            },
            b,
            n + 1,
            name,
            arity,
            sel,
        ),
{
    let next = if hit {
        list.push(n as usize)
    } else {
        list
    };
    assert forall|i: int, k: int| 0 <= i < k < next.len() implies next[i] < next[k] by {
        if k < list.len() {
            assert(next[i] == list[i] && next[k] == list[k]);
        } else {
            assert(next[i] == list[i]);
            assert((list[i] as int) < n);
        }
    }
    assert forall|i: int| 0 <= i < next.len() implies (#[trigger] next[i] as int) < n + 1
        && selects(b[next[i] as int], name, arity, sel) by {
        if i < list.len() {
            assert(next[i] == list[i]);
        }
    }
    assert forall|j: int|
        0 <= j < n + 1 && #[trigger] selects(b[j], name, arity, sel) implies next.contains(
        j as usize,
    ) by {
        if j < n {
            assert(list.contains(j as usize));
            let i = choose|i: int| 0 <= i < list.len() && list[i] == j as usize;
            assert(next[i] == list[i]);
        } else {
            assert(next[list.len() as int] == n as usize);
        }
    }
}

// No selected position lies between `x` and the list's inclusive lower bound `v` at x.
proof fn lower_gap(
    list: Seq<usize>,
    b: Seq<DocClause>,
    name: Seq<u8>,
    arity: nat,
    sel: Sel,
    x: int,
    v: int,
)
    requires
        exact(list, b, b.len(), name, arity, sel),
        forall|i: int| 0 <= i < list.len() && x <= list[i] ==> v <= list[i],
    ensures
        forall|j: int|
            x <= j < v && 0 <= j < b.len() ==> !selects(#[trigger] b[j], name, arity, sel),
{
    assert forall|j: int| x <= j < v && 0 <= j < b.len() implies !selects(
        #[trigger] b[j],
        name,
        arity,
        sel,
    ) by {
        if selects(b[j], name, arity, sel) {
            assert(list.contains(j as usize));
            let i = choose|i: int| 0 <= i < list.len() && list[i] == j as usize;
            assert(v <= list[i]);
        }
    }
}

// --- node helpers ---
fn node_class(arena: &ETermArena, root: usize) -> (out: Option<EClass>)
    requires
        root_ok(arena, root),
    ensures
        out is None <==> class_of(arena@[root as int]) is None,
        out matches Some(c) ==> class_of(arena@[root as int]) == Some(class_view(c)),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Atom { name } => {
            let copy = name.clone();
            proof {
                assert(copy@ =~= name@);
            }
            Some(EClass::Atom(copy))
        },
        ENodeKind::Comp { name, child_roots, .. } => {
            proof {
                node_comp_model(arena.nodes@, root as int, name@, child_roots@);
            }
            let copy = name.clone();
            proof {
                assert(copy@ =~= name@);
            }
            Some(EClass::Comp(copy, child_roots.len()))
        },
        _ => None,
    }
}

fn classless(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == (class_of(arena@[root as int]) is None),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Atom { .. } => false,
        ENodeKind::Comp { .. } => false,
        _ => true,
    }
}

fn class_matches(arena: &ETermArena, root: usize, c: &EClass) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == (class_of(arena@[root as int]) == Some(class_view(*c))),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
    }
    match (&arena.nodes[root].kind, c) {
        (ENodeKind::Atom { name }, EClass::Atom(n)) => vec_equal(name, n),
        (ENodeKind::Comp { name, child_roots, .. }, EClass::Comp(n, k)) => {
            proof {
                node_comp_model(arena.nodes@, root as int, name@, child_roots@);
            }
            child_roots.len() == *k && vec_equal(name, n)
        },
        _ => false,
    }
}

fn same_class(arena: &ETermArena, a: usize, b: usize) -> (out: bool)
    requires
        root_ok(arena, a),
        root_ok(arena, b),
    ensures
        out == (class_of(arena@[a as int]) == class_of(arena@[b as int])),
{
    proof {
        assert(node_ok(arena.nodes@, a as int));
        assert(node_ok(arena.nodes@, b as int));
    }
    match (&arena.nodes[a].kind, &arena.nodes[b].kind) {
        (ENodeKind::Atom { name: x }, ENodeKind::Atom { name: y }) => vec_equal(x, y),
        (
            ENodeKind::Comp { name: x, child_roots: xs, .. },
            ENodeKind::Comp { name: y, child_roots: ys, .. },
        ) => {
            proof {
                node_comp_model(arena.nodes@, a as int, x@, xs@);
                node_comp_model(arena.nodes@, b as int, y@, ys@);
            }
            xs.len() == ys.len() && vec_equal(x, y)
        },
        (ENodeKind::Atom { .. }, _) => false,
        (ENodeKind::Comp { .. }, _) => false,
        (_, ENodeKind::Atom { .. }) => false,
        (_, ENodeKind::Comp { .. }) => false,
        _ => true,
    }
}

// The root of the head's first argument; none for a head without arguments.
fn head_first(arena: &ETermArena, head: usize) -> (out: Option<usize>)
    requires
        root_ok(arena, head),
    ensures
        out matches Some(r) ==> {
            &&& r < arena.nodes@.len()
            &&& args_of(arena@[head as int]).len() > 0
            &&& arena@[r as int] == args_of(arena@[head as int])[0]
        },
        out is None ==> args_of(arena@[head as int]).len() == 0,
{
    proof {
        assert(node_ok(arena.nodes@, head as int));
    }
    match &arena.nodes[head].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            proof {
                node_comp_model(arena.nodes@, head as int, name@, child_roots@);
                reveal(roots_valid);
            }
            if child_roots.len() > 0 {
                Some(child_roots[0])
            } else {
                None
            }
        },
        _ => None,
    }
}

// candidate(the clause, name, |args|, class of args[0]) (D2).
fn is_candidate(arena: &ETermArena, clause: &EClause, name: &Vec<u8>, args: &Vec<usize>) -> (out:
    bool)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, clause),
        roots_valid(arena.nodes@, args@),
    ensures
        out == candidate(
            clause_view(arena.nodes@, clause),
            name@,
            args.len() as nat,
            first_class(root_terms(arena.nodes@, args@)),
        ),
{
    proof {
        reveal(roots_valid);
    }
    if !literal_matches(arena, clause.head, name, args.len()) {
        return false;
    }
    if args.len() == 0 || classless(arena, args[0]) {
        return true;
    }
    match head_first(arena, clause.head) {
        None => true,
        Some(r) => classless(arena, r) || same_class(arena, args[0], r),
    }
}

// head_class of a clause = the class of its head's first argument node.
proof fn head_class_node(arena: &ETermArena, clause: &EClause, first: Option<usize>)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, clause),
        first matches Some(r) ==> {
            &&& r < arena.nodes@.len()
            &&& args_of(arena@[clause.head as int]).len() > 0
            &&& arena@[r as int] == args_of(arena@[clause.head as int])[0]
        },
        first is None ==> args_of(arena@[clause.head as int]).len() == 0,
    ensures
        head_class(clause_view(arena.nodes@, clause)) == match first {
            Some(r) => class_of(arena@[r as int]),
            None => None,
        },
{
    assert(clause_view(arena.nodes@, clause).head == arena@[clause.head as int]);
}

// --- build (contract cix2 D3) ---
// A clause of another predicate leaves every list of an entry exact one step further.
proof fn pred_upto_skip(e: EPredIndex, b: Seq<DocClause>, n: nat)
    requires
        pred_upto(e, b, n),
        n < b.len(),
        lit_fa(b[n as int].head) != Some((e.name@, e.arity as nat)),
    ensures
        pred_upto(e, b, n + 1),
{
    exact_step(e.all@, b, n, e.name@, e.arity as nat, Sel::All, false);
    exact_step(e.wild@, b, n, e.name@, e.arity as nat, Sel::Wild, false);
    assert forall|q: int| 0 <= q < e.classes@.len() implies exact(
        (#[trigger] e.classes@[q]).list@,
        b,
        n + 1,
        e.name@,
        e.arity as nat,
        Sel::Class(class_view(e.classes@[q].class)),
    ) by {
        exact_step(
            e.classes@[q].list@,
            b,
            n,
            e.name@,
            e.arity as nat,
            Sel::Class(class_view(e.classes@[q].class)),
            false,
        );
    }
}

proof fn routes_elim(routes: Map<u64, Vec<usize>>, hashes: Seq<u64>, i: int)
    requires
        routes_ok(routes, hashes),
        0 <= i < hashes.len(),
    ensures
        routes.contains_key(hashes[i]),
        routes[hashes[i]]@.contains(i as usize),
{
}

// Appending index t = |hashes| with hash h to h's bucket keeps the routing valid.
proof fn routes_append(
    old: Map<u64, Vec<usize>>,
    new: Map<u64, Vec<usize>>,
    hashes: Seq<u64>,
    h: u64,
    t: usize,
)
    requires
        routes_ok(old, hashes),
        t as int == hashes.len(),
        new.contains_key(h),
        new[h]@ == (if old.contains_key(h) {
            old[h]@
        } else {
            Seq::<usize>::empty()
        }).push(t),
        forall|g: u64|
            g != h ==> (#[trigger] new.contains_key(g) == old.contains_key(g)) && (old.contains_key(
                g,
            ) ==> new[g] == old[g]),
    ensures
        routes_ok(new, hashes.push(h)),
{
    let hs = hashes.push(h);
    let n = hashes.len();
    let prior = if old.contains_key(h) {
        old[h]@
    } else {
        Seq::<usize>::empty()
    };
    assert forall|i: int| 0 <= i < hs.len() implies #[trigger] new.contains_key(hs[i])
        && new[hs[i]]@.contains(i as usize) by {
        assert(i <= t as int);
        if i < n {
            assert(hs[i] == hashes[i]);
            routes_elim(old, hashes, i);
            if hashes[i] == h {
                assert(hs[i] == h);
                let k = choose|k: int| 0 <= k < old[h]@.len() && old[h]@[k] == i as usize;
                assert(new[h]@[k] == i as usize);
            } else {
                assert(new.contains_key(hashes[i]) == old.contains_key(hashes[i]));
                assert(new[hashes[i]] == old[hashes[i]]);
            }
        } else {
            assert(hs[i] == h);
            assert(i as usize == t);
            assert(new[h]@[prior.len() as int] == t);
        }
    }
    assert forall|g: u64, k: int| new.contains_key(g) && 0 <= k < new[g]@.len() implies (
    #[trigger] new[g]@[k] as int) < hs.len() && hs[new[g]@[k] as int] == g by {
        if g == h {
            if k < prior.len() {
                assert(new[h]@[k] == old[h]@[k]);
                assert(old.contains_key(h));
            }
        } else {
            assert(old.contains_key(g) && new[g] == old[g]);
        }
    }
    assert forall|g: u64| new.contains_key(g) implies (#[trigger] new[g]@).no_duplicates() by {
        if g == h {
            assert forall|x: int, y: int| 0 <= x < y < new[h]@.len() implies new[h]@[x]
                != new[h]@[y] by {
                if y == prior.len() {
                    assert(old.contains_key(h));
                    assert((old[h]@[x] as int) < n);
                } else {
                    assert(old.contains_key(h));
                    assert(old[h]@.no_duplicates());
                }
            }
        } else {
            assert(old.contains_key(g) && new[g] == old[g]);
        }
    }
}

// Push x onto h's bucket (the bucket's earlier entries kept).
fn route_push(routes: &mut BTreeMap<u64, Vec<usize>>, h: u64, x: usize)
    ensures
        final(routes)@.contains_key(h),
        final(routes)@[h]@ == (if old(routes)@.contains_key(h) {
            old(routes)@[h]@
        } else {
            Seq::<usize>::empty()
        }).push(x),
        forall|g: u64|
            g != h ==> (#[trigger] final(routes)@.contains_key(g) == old(routes)@.contains_key(g))
                && (old(routes)@.contains_key(g) ==> final(routes)@[g] == old(routes)@[g]),
{
    let mut chain = match routes.remove(&h) {
        Some(c) => c,
        None => Vec::new(),
    };
    chain.push(x);
    routes.insert(h, chain);
}

// The entry for the predicate (name, arity), created (empty, routed) when absent.
fn pred_slot(
    ix: &mut EClauseIndex,
    name: &Vec<u8>,
    arity: usize,
    Ghost(b): Ghost<Seq<DocClause>>,
    Ghost(n): Ghost<nat>,
) -> (p: usize)
    requires
        index_upto(*old(ix), b, n),
        n <= b.len() <= usize::MAX,
    ensures
        p < final(ix).preds@.len(),
        final(ix).preds@[p as int].name@ == name@,
        final(ix).preds@[p as int].arity == arity,
        index_upto(*final(ix), b, n),
        old(ix).preds@.len() <= final(ix).preds@.len() <= old(ix).preds@.len() + 1,
        forall|q: int|
            0 <= q < old(ix).preds@.len() ==> #[trigger] final(ix).preds@[q] == old(ix).preds@[q],
{
    let h = hash_pred(name, arity);
    let ghost hashes = pred_hashes(*ix);
    let plen = ix.preds.len();
    match ix.routes.get(&h) {
        Some(chain) => {
            let mut k = 0usize;
            while k < chain.len()
                invariant
                    index_upto(*ix, b, n),
                    ix.routes@.contains_key(h),
                    chain@ == ix.routes@[h]@,
                    k <= chain.len(),
                    forall|t: int|
                        0 <= t < k ==> !((#[trigger] ix.preds@[chain@[t] as int]).name@ == name@
                            && ix.preds@[chain@[t] as int].arity == arity),
                decreases chain.len() - k,
            {
                let q = chain[k];
                proof {
                    assert((ix.routes@[h]@[k as int] as int) < pred_hashes(*ix).len());
                }
                if ix.preds[q].arity == arity && vec_equal(&ix.preds[q].name, name) {
                    return q;
                }
                k += 1;
            }
            proof {
                assert forall|q: int| 0 <= q < ix.preds@.len() implies !((
                #[trigger] ix.preds@[q]).name@ == name@ && ix.preds@[q].arity == arity) by {
                    if ix.preds@[q].name@ == name@ && ix.preds@[q].arity == arity {
                        assert(pred_hashes(*ix)[q] == h);
                        routes_elim(ix.routes@, pred_hashes(*ix), q);
                        let w = choose|w: int| 0 <= w < chain@.len() && chain@[w] == q as usize;
                        assert(0 <= q < plen as int);
                        assert(chain@[w] as int == q);
                    }
                }
            }
        },
        None => {
            proof {
                assert forall|q: int| 0 <= q < ix.preds@.len() implies !((
                #[trigger] ix.preds@[q]).name@ == name@ && ix.preds@[q].arity == arity) by {
                    if ix.preds@[q].name@ == name@ && ix.preds@[q].arity == arity {
                        assert(pred_hashes(*ix)[q] == h);
                        routes_elim(ix.routes@, pred_hashes(*ix), q);
                    }
                }
            }
        },
    }
    let ghost before = *ix;
    let q = ix.preds.len();
    let copy = name.clone();
    proof {
        assert(copy@ =~= name@);
    }
    ix.preds.push(
        EPredIndex {
            name: copy,
            arity,
            all: Vec::new(),
            wild: Vec::new(),
            classes: Vec::new(),
            routes: BTreeMap::new(),
        },
    );
    let ghost grown = *ix;
    route_push(&mut ix.routes, h, q);
    proof {
        let e = ix.preds@[q as int];
        assert(pred_hashes(*ix) =~= pred_hashes(before).push(h));
        routes_append(before.routes@, ix.routes@, pred_hashes(before), h, q);
        assert(class_hashes(e) =~= Seq::<u64>::empty());
        // no earlier clause names this predicate: its entry would exist
        assert forall|j: int| 0 <= j < n implies !(#[trigger] selects(
            b[j],
            name@,
            arity as nat,
            Sel::All,
        )) by {
            if selects(b[j], name@, arity as nat, Sel::All) {
                assert(lit_fa(b[j].head) is Some);
                let r = choose|r: int|
                    0 <= r < before.preds@.len() && lit_fa(b[j].head) == Some(
                        (before.preds@[r].name@, before.preds@[r].arity as nat),
                    );
                assert(before.preds@[r].name@ == name@ && before.preds@[r].arity == arity);
            }
        }
        assert(exact(e.all@, b, n, e.name@, e.arity as nat, Sel::All));
        assert(exact(e.wild@, b, n, e.name@, e.arity as nat, Sel::Wild));
        assert(pred_upto(e, b, n));
        assert forall|r: int| 0 <= r < ix.preds@.len() implies pred_upto(
            #[trigger] ix.preds@[r],
            b,
            n,
        ) by {
            if r < before.preds@.len() {
                assert(ix.preds@[r] == before.preds@[r]);
            }
        }
        assert forall|j: int| 0 <= j < n && #[trigger] lit_fa(b[j].head) is Some implies exists|
            r: int,
        |
            0 <= r < ix.preds@.len() && lit_fa(b[j].head) == Some(
                (ix.preds@[r].name@, ix.preds@[r].arity as nat),
            ) by {
            let r = choose|r: int|
                0 <= r < before.preds@.len() && lit_fa(b[j].head) == Some(
                    (before.preds@[r].name@, before.preds@[r].arity as nat),
                );
            assert(ix.preds@[r] == before.preds@[r]);
        }
    }
    q
}

// The class list of the root's class in entry `e`, created (empty, routed) when absent.
fn class_slot(
    e: &mut EPredIndex,
    arena: &ETermArena,
    root: usize,
    Ghost(b): Ghost<Seq<DocClause>>,
    Ghost(n): Ghost<nat>,
) -> (q: usize)
    requires
        pred_upto(*old(e), b, n),
        n <= b.len() <= usize::MAX,
        root_ok(arena, root),
        class_of(arena@[root as int]) is Some,
    ensures
        q < final(e).classes@.len(),
        Some(class_view(final(e).classes@[q as int].class)) == class_of(arena@[root as int]),
        final(e).name@ == old(e).name@,
        final(e).arity == old(e).arity,
        final(e).all@ == old(e).all@,
        final(e).wild@ == old(e).wild@,
        pred_upto(*final(e), b, n),
        old(e).classes@.len() <= final(e).classes@.len() <= old(e).classes@.len() + 1,
        forall|t: int|
            0 <= t < old(e).classes@.len() ==> #[trigger] final(e).classes@[t] == old(
                e,
            ).classes@[t],
{
    let c = match node_class(arena, root) {
        Some(c) => c,
        None => {
            proof {
                assert(false);
            }
            EClass::Atom(Vec::new())
        },
    };
    let h = hash_class(&c);
    let clen = e.classes.len();
    match e.routes.get(&h) {
        Some(chain) => {
            let mut k = 0usize;
            while k < chain.len()
                invariant
                    pred_upto(*e, b, n),
                    root_ok(arena, root),
                    Some(class_view(c)) == class_of(arena@[root as int]),
                    e.routes@.contains_key(h),
                    chain@ == e.routes@[h]@,
                    k <= chain.len(),
                    forall|t: int|
                        0 <= t < k ==> class_of(arena@[root as int]) != Some(
                            class_view((#[trigger] e.classes@[chain@[t] as int]).class),
                        ),
                decreases chain.len() - k,
            {
                let t = chain[k];
                proof {
                    assert((e.routes@[h]@[k as int] as int) < class_hashes(*e).len());
                }
                if class_matches(arena, root, &e.classes[t].class) {
                    return t;
                }
                k += 1;
            }
            proof {
                assert forall|t: int| 0 <= t < e.classes@.len() implies class_of(
                    arena@[root as int],
                ) != Some(class_view((#[trigger] e.classes@[t]).class)) by {
                    if class_of(arena@[root as int]) == Some(class_view(e.classes@[t].class)) {
                        assert(class_hashes(*e)[t] == h);
                        routes_elim(e.routes@, class_hashes(*e), t);
                        let w = choose|w: int| 0 <= w < chain@.len() && chain@[w] == t as usize;
                        assert(0 <= t < clen as int);
                        assert(chain@[w] as int == t);
                    }
                }
            }
        },
        None => {
            proof {
                assert forall|t: int| 0 <= t < e.classes@.len() implies class_of(
                    arena@[root as int],
                ) != Some(class_view((#[trigger] e.classes@[t]).class)) by {
                    if class_of(arena@[root as int]) == Some(class_view(e.classes@[t].class)) {
                        assert(class_hashes(*e)[t] == h);
                        routes_elim(e.routes@, class_hashes(*e), t);
                    }
                }
            }
        },
    }
    let ghost before = *e;
    let t = e.classes.len();
    e.classes.push(EClassList { class: c, list: Vec::new() });
    route_push(&mut e.routes, h, t);
    proof {
        assert(class_hashes(*e) =~= class_hashes(before).push(h));
        routes_append(before.routes@, e.routes@, class_hashes(before), h, t);
        let k = class_view(c);
        // no earlier clause of the predicate carries this class: its list would exist
        assert forall|j: int| 0 <= j < n implies !(#[trigger] selects(
            b[j],
            e.name@,
            e.arity as nat,
            Sel::Class(k),
        )) by {
            if selects(b[j], e.name@, e.arity as nat, Sel::Class(k)) {
                assert(selects(b[j], e.name@, e.arity as nat, Sel::All));
                let r = choose|r: int|
                    0 <= r < before.classes@.len() && Some(class_view(before.classes@[r].class))
                        == head_class(b[j]);
            }
        }
        assert forall|r: int| 0 <= r < e.classes@.len() implies exact(
            (#[trigger] e.classes@[r]).list@,
            b,
            n,
            e.name@,
            e.arity as nat,
            Sel::Class(class_view(e.classes@[r].class)),
        ) by {
            if r < before.classes@.len() {
                assert(e.classes@[r] == before.classes@[r]);
            }
        }
        assert forall|j: int|
            0 <= j < n && #[trigger] selects(b[j], e.name@, e.arity as nat, Sel::All) && head_class(
                b[j],
            ) is Some implies exists|r: int|
            0 <= r < e.classes@.len() && Some(class_view(e.classes@[r].class)) == head_class(
                b[j],
            ) by {
            let r = choose|r: int|
                0 <= r < before.classes@.len() && Some(class_view(before.classes@[r].class))
                    == head_class(b[j]);
            assert(e.classes@[r] == before.classes@[r]);
        }
    }
    t
}

pub fn build_index(arena: &ETermArena, db: &Vec<EClause>) -> (ix: EClauseIndex)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
    ensures
        index_over(ix, db_view(arena.nodes@, db@)),
{
    let ghost b = db_view(arena.nodes@, db@);
    let mut ix = EClauseIndex { preds: Vec::new(), routes: BTreeMap::new() };
    proof {
        assert(pred_hashes(ix) =~= Seq::<u64>::empty());
    }
    let mut j = 0usize;
    while j < db.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            b == db_view(arena.nodes@, db@),
            j <= db.len(),
            ix.preds@.len() <= j,
            index_upto(ix, b, j as nat),
        decreases db.len() - j,
    {
        let head = db[j].head;
        proof {
            assert(clause_valid(arena.nodes@, &db@[j as int]));
            assert(node_ok(arena.nodes@, head as int));
            assert(b[j as int] == clause_view(arena.nodes@, &db@[j as int]));
        }
        let ghost before = ix;
        match &arena.nodes[head].kind {
            ENodeKind::Comp { name, child_roots, .. } => {
                proof {
                    node_comp_model(arena.nodes@, head as int, name@, child_roots@);
                }
                let arity = child_roots.len();
                let p = pred_slot(&mut ix, name, arity, Ghost(b), Ghost(j as nat));
                let ghost mid = ix;
                let first = head_first(arena, head);
                proof {
                    head_class_node(arena, &db@[j as int], first);
                }
                let mut e = EPredIndex {
                    name: Vec::new(),
                    arity: 0,
                    all: Vec::new(),
                    wild: Vec::new(),
                    classes: Vec::new(),
                    routes: BTreeMap::new(),
                };
                ix.preds.set_and_swap(p, &mut e);
                let ghost taken = e;
                let mut target: Option<usize> = None;
                if let Some(r) = first {
                    if !classless(arena, r) {
                        target = Some(class_slot(&mut e, arena, r, Ghost(b), Ghost(j as nat)));
                    }
                }
                let ghost slotted = e;
                e.all.push(j);
                match target {
                    Some(q) => {
                        let mut cl = EClassList {
                            class: EClass::Atom(Vec::new()),
                            list: Vec::new(),
                        };
                        e.classes.set_and_swap(q, &mut cl);
                        cl.list.push(j);
                        e.classes.set_and_swap(q, &mut cl);
                    },
                    None => {
                        e.wild.push(j);
                    },
                }
                proof {
                    build_entry_step(slotted, e, b, j as nat, target);
                }
                ix.preds.set_and_swap(p, &mut e);
                proof {
                    build_index_step(mid, ix, b, j as nat, p as int);
                }
            },
            _ => {
                proof {
                    assert(lit_fa(b[j as int].head) is None);
                    assert forall|r: int| 0 <= r < ix.preds@.len() implies pred_upto(
                        #[trigger] ix.preds@[r],
                        b,
                        j as nat + 1,
                    ) by {
                        pred_upto_skip(ix.preds@[r], b, j as nat);
                    }
                }
            },
        }
        j += 1;
    }
    ix
}

// The entry that clause n joins: `all` gains n; its class list (or `wild`) gains n.
proof fn build_entry_step(
    old: EPredIndex,
    new: EPredIndex,
    b: Seq<DocClause>,
    n: nat,
    target: Option<usize>,
)
    requires
        pred_upto(old, b, n),
        n < b.len(),
        lit_fa(b[n as int].head) == Some((old.name@, old.arity as nat)),
        new.name@ == old.name@,
        new.arity == old.arity,
        new.routes@ == old.routes@,
        new.all@ == old.all@.push(n as usize),
        new.classes@.len() == old.classes@.len(),
        match target {
            Some(q) => {
                &&& q < old.classes@.len()
                &&& Some(class_view(old.classes@[q as int].class)) == head_class(b[n as int])
                &&& new.wild@ == old.wild@
                &&& new.classes@[q as int].list@ == old.classes@[q as int].list@.push(n as usize)
                &&& class_view(new.classes@[q as int].class) == class_view(
                    old.classes@[q as int].class,
                )
                &&& forall|t: int|
                    0 <= t < old.classes@.len() && t != q ==> #[trigger] new.classes@[t]
                        == old.classes@[t]
            },
            None => {
                &&& head_class(b[n as int]) is None
                &&& new.wild@ == old.wild@.push(n as usize)
                &&& forall|t: int|
                    0 <= t < old.classes@.len() ==> #[trigger] new.classes@[t] == old.classes@[t]
            },
        },
    ensures
        pred_upto(new, b, n + 1),
{
    let name = old.name@;
    let arity = old.arity as nat;
    exact_step(old.all@, b, n, name, arity, Sel::All, true);
    exact_step(old.wild@, b, n, name, arity, Sel::Wild, target is None);
    assert forall|t: int| 0 <= t < new.classes@.len() implies exact(
        (#[trigger] new.classes@[t]).list@,
        b,
        n + 1,
        name,
        arity,
        Sel::Class(class_view(new.classes@[t].class)),
    ) by {
        let k = class_view(old.classes@[t].class);
        let hit = selects(b[n as int], name, arity, Sel::Class(k));
        exact_step(old.classes@[t].list@, b, n, name, arity, Sel::Class(k), hit);
        match target {
            Some(q) => {
                if t != q {
                    assert(new.classes@[t] == old.classes@[t]);
                    assert(class_view(old.classes@[t].class) != class_view(
                        old.classes@[q as int].class,
                    ));
                }
            },
            None => {
                assert(new.classes@[t] == old.classes@[t]);
            },
        }
    }
    assert(class_hashes(new) =~= class_hashes(old)) by {
        assert forall|t: int| 0 <= t < new.classes@.len() implies class_view(new.classes@[t].class)
            == class_view(old.classes@[t].class) by {
            match target {
                Some(q) => {
                    if t != q {
                        assert(new.classes@[t] == old.classes@[t]);
                    }
                },
                None => {
                    assert(new.classes@[t] == old.classes@[t]);
                },
            }
        }
    }
    assert forall|x: int, y: int| 0 <= x < y < new.classes@.len() implies class_view(
        new.classes@[x].class,
    ) != class_view(new.classes@[y].class) by {
        assert(class_hashes(new)[x] == class_hashes(old)[x]);
    }
    assert forall|j: int|
        0 <= j < n + 1 && #[trigger] selects(b[j], name, arity, Sel::All) && head_class(
            b[j],
        ) is Some implies exists|t: int|
        0 <= t < new.classes@.len() && Some(class_view(new.classes@[t].class)) == head_class(
            b[j],
        ) by {
        if j < n {
            let t = choose|t: int|
                0 <= t < old.classes@.len() && Some(class_view(old.classes@[t].class))
                    == head_class(b[j]);
            match target {
                Some(q) => {
                    if t != q {
                        assert(new.classes@[t] == old.classes@[t]);
                    }
                },
                None => {
                    assert(new.classes@[t] == old.classes@[t]);
                },
            }
        } else {
            if let Some(q) = target {
                assert(class_view(new.classes@[q as int].class) == class_view(
                    old.classes@[q as int].class,
                ));
            }
        }
    }
}

// Restoring the grown entry at p: every entry exact over n + 1, index coverage, keys + routes kept.
proof fn build_index_step(mid: EClauseIndex, new: EClauseIndex, b: Seq<DocClause>, n: nat, p: int)
    requires
        index_upto(mid, b, n),
        n < b.len(),
        0 <= p < mid.preds@.len(),
        lit_fa(b[n as int].head) == Some((mid.preds@[p].name@, mid.preds@[p].arity as nat)),
        new.routes@ == mid.routes@,
        new.preds@.len() == mid.preds@.len(),
        pred_upto(new.preds@[p], b, n + 1),
        new.preds@[p].name@ == mid.preds@[p].name@,
        new.preds@[p].arity == mid.preds@[p].arity,
        forall|r: int|
            0 <= r < mid.preds@.len() && r != p ==> #[trigger] new.preds@[r] == mid.preds@[r],
    ensures
        index_upto(new, b, n + 1),
{
    assert forall|r: int| 0 <= r < new.preds@.len() implies pred_upto(
        #[trigger] new.preds@[r],
        b,
        n + 1,
    ) by {
        if r != p {
            assert(new.preds@[r] == mid.preds@[r]);
            assert((mid.preds@[r].name@, mid.preds@[r].arity) != (
                mid.preds@[p].name@,
                mid.preds@[p].arity,
            ));
            pred_upto_skip(mid.preds@[r], b, n);
        }
    }
    assert(pred_hashes(new) =~= pred_hashes(mid)) by {
        assert forall|r: int| 0 <= r < new.preds@.len() implies new.preds@[r].name@
            == mid.preds@[r].name@ && new.preds@[r].arity == mid.preds@[r].arity by {
            if r != p {
                assert(new.preds@[r] == mid.preds@[r]);
            }
        }
    }
    assert forall|j: int| 0 <= j < n + 1 && #[trigger] lit_fa(b[j].head) is Some implies exists|
        r: int,
    |
        0 <= r < new.preds@.len() && lit_fa(b[j].head) == Some(
            (new.preds@[r].name@, new.preds@[r].arity as nat),
        ) by {
        if j < n {
            let r = choose|r: int|
                0 <= r < mid.preds@.len() && lit_fa(b[j].head) == Some(
                    (mid.preds@[r].name@, mid.preds@[r].arity as nat),
                );
            if r != p {
                assert(new.preds@[r] == mid.preds@[r]);
            }
        }
    }
    assert forall|q: int, r: int| 0 <= q < r < new.preds@.len() implies (
        new.preds@[q].name@,
        new.preds@[q].arity,
    ) != (new.preds@[r].name@, new.preds@[r].arity) by {
        assert(pred_hashes(new)[q] == pred_hashes(mid)[q]);
        if q != p {
            assert(new.preds@[q] == mid.preds@[q]);
        }
        if r != p {
            assert(new.preds@[r] == mid.preds@[r]);
        }
    }
}

// --- lookup (D4) ---
// The least element of `list` at or after x.
fn lower_bound(list: &Vec<usize>, x: usize) -> (out: Option<usize>)
    requires
        increasing(list@),
    ensures
        out matches Some(v) ==> {
            &&& list@.contains(v)
            &&& x <= v
            &&& forall|i: int| 0 <= i < list@.len() && x <= list@[i] ==> v <= list@[i]
        },
        out is None ==> forall|i: int| 0 <= i < list@.len() ==> list@[i] < x,
{
    let mut lo = 0usize;
    let mut hi = list.len();
    while lo < hi
        invariant
            lo <= hi <= list@.len(),
            increasing(list@),
            forall|i: int| 0 <= i < lo ==> list@[i] < x,
            forall|i: int| hi <= i < list@.len() ==> x <= list@[i],
        decreases hi - lo,
    {
        let mid = lo + (hi - lo) / 2;
        if list[mid] < x {
            proof {
                assert forall|i: int| 0 <= i < mid + 1 implies list@[i] < x by {
                    if i < mid {
                        assert(list@[i] < list@[mid as int]);
                    }
                }
            }
            lo = mid + 1;
        } else {
            proof {
                assert forall|i: int| mid <= i < list@.len() implies x <= list@[i] by {
                    if i > mid {
                        assert(list@[mid as int] < list@[i]);
                    }
                }
            }
            hi = mid;
        }
    }
    if lo < list.len() {
        proof {
            assert forall|i: int| 0 <= i < list@.len() && x <= list@[i] implies list@[lo as int]
                <= list@[i] by {
                if i > lo {
                    assert(list@[lo as int] < list@[i]);
                }
            }
        }
        Some(list[lo])
    } else {
        None
    }
}

fn find_pred(ix: &EClauseIndex, name: &Vec<u8>, arity: usize) -> (out: Option<usize>)
    requires
        routes_ok(ix.routes@, pred_hashes(*ix)),
    ensures
        out matches Some(p) ==> {
            &&& p < ix.preds@.len()
            &&& ix.preds@[p as int].name@ == name@
            &&& ix.preds@[p as int].arity == arity
        },
        out is None ==> forall|p: int|
            0 <= p < ix.preds@.len() ==> !((#[trigger] ix.preds@[p]).name@ == name@
                && ix.preds@[p].arity == arity),
{
    let h = hash_pred(name, arity);
    let plen = ix.preds.len();
    match ix.routes.get(&h) {
        Some(chain) => {
            let mut k = 0usize;
            while k < chain.len()
                invariant
                    routes_ok(ix.routes@, pred_hashes(*ix)),
                    ix.routes@.contains_key(h),
                    chain@ == ix.routes@[h]@,
                    k <= chain.len(),
                    forall|t: int|
                        0 <= t < k ==> !((#[trigger] ix.preds@[chain@[t] as int]).name@ == name@
                            && ix.preds@[chain@[t] as int].arity == arity),
                decreases chain.len() - k,
            {
                let q = chain[k];
                proof {
                    assert((ix.routes@[h]@[k as int] as int) < pred_hashes(*ix).len());
                }
                if ix.preds[q].arity == arity && vec_equal(&ix.preds[q].name, name) {
                    return Some(q);
                }
                k += 1;
            }
            proof {
                assert forall|q: int| 0 <= q < ix.preds@.len() implies !((ix.preds@[q]).name@
                    == name@ && ix.preds@[q].arity == arity) by {
                    if !(!((ix.preds@[q]).name@ == name@ && ix.preds@[q].arity == arity)) {
                        assert(pred_hashes(*ix)[q] == h);
                        routes_elim(ix.routes@, pred_hashes(*ix), q);
                        let w = choose|w: int| 0 <= w < chain@.len() && chain@[w] == q as usize;
                        assert(0 <= q < plen as int);
                        assert(chain@[w] as int == q);
                    }
                }
            }
            None
        },
        None => {
            proof {
                assert forall|q: int| 0 <= q < ix.preds@.len() implies !((ix.preds@[q]).name@
                    == name@ && ix.preds@[q].arity == arity) by {
                    if !(!((ix.preds@[q]).name@ == name@ && ix.preds@[q].arity == arity)) {
                        assert(pred_hashes(*ix)[q] == h);
                        routes_elim(ix.routes@, pred_hashes(*ix), q);

                    }
                }
            }
            None
        },
    }
}

// The class hash of the node at `root` (none = classless), without copying its name.
fn node_class_hash(arena: &ETermArena, root: usize) -> (out: Option<u64>)
    requires
        root_ok(arena, root),
    ensures
        out is None <==> class_of(arena@[root as int]) is None,
        out matches Some(h) ==> h == class_hash(class_of(arena@[root as int])->0),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Atom { name } => Some(fold_exec(name, 1)),
        ENodeKind::Comp { name, child_roots, .. } => {
            proof {
                node_comp_model(arena.nodes@, root as int, name@, child_roots@);
            }
            Some(fold_exec(name, (child_roots.len() as u64).wrapping_add(2)))
        },
        _ => None,
    }
}

fn find_class(arena: &ETermArena, entry: &EPredIndex, root: usize) -> (out: Option<usize>)
    requires
        root_ok(arena, root),
        routes_ok(entry.routes@, class_hashes(*entry)),
    ensures
        out matches Some(q) ==> {
            &&& q < entry.classes@.len()
            &&& class_of(arena@[root as int]) == Some(class_view(entry.classes@[q as int].class))
        },
        out is None ==> forall|q: int|
            0 <= q < entry.classes@.len() ==> class_of(arena@[root as int]) != Some(
                class_view((#[trigger] entry.classes@[q]).class),
            ),
{
    let h = match node_class_hash(arena, root) {
        Some(h) => h,
        None => {
            return None;
        },
    };
    let clen = entry.classes.len();
    match entry.routes.get(&h) {
        Some(chain) => {
            let mut k = 0usize;
            while k < chain.len()
                invariant
                    root_ok(arena, root),
                    routes_ok(entry.routes@, class_hashes(*entry)),
                    entry.routes@.contains_key(h),
                    chain@ == entry.routes@[h]@,
                    k <= chain.len(),
                    forall|t: int|
                        0 <= t < k ==> class_of(arena@[root as int]) != Some(
                            class_view((#[trigger] entry.classes@[chain@[t] as int]).class),
                        ),
                decreases chain.len() - k,
            {
                let q = chain[k];
                proof {
                    assert((entry.routes@[h]@[k as int] as int) < class_hashes(*entry).len());
                }
                if class_matches(arena, root, &entry.classes[q].class) {
                    return Some(q);
                }
                k += 1;
            }
            proof {
                assert forall|q: int| 0 <= q < entry.classes@.len() implies class_of(
                    arena@[root as int],
                ) != Some(class_view((entry.classes@[q]).class)) by {
                    if !(class_of(arena@[root as int]) != Some(
                        class_view((entry.classes@[q]).class),
                    )) {
                        assert(class_hashes(*entry)[q] == h);
                        routes_elim(entry.routes@, class_hashes(*entry), q);
                        let w = choose|w: int| 0 <= w < chain@.len() && chain@[w] == q as usize;
                        assert(0 <= q < clen as int);
                        assert(chain@[w] as int == q);
                    }
                }
            }
            None
        },
        None => {
            proof {
                assert forall|q: int| 0 <= q < entry.classes@.len() implies class_of(
                    arena@[root as int],
                ) != Some(class_view((entry.classes@[q]).class)) by {
                    if !(class_of(arena@[root as int]) != Some(
                        class_view((entry.classes@[q]).class),
                    )) {
                        assert(class_hashes(*entry)[q] == h);
                        routes_elim(entry.routes@, class_hashes(*entry), q);

                    }
                }
            }
            None
        },
    }
}

// The first candidate in [from, front) by a direct scan.
fn prefix_scan(
    arena: &ETermArena,
    db: &Vec<EClause>,
    front: usize,
    name: &Vec<u8>,
    args: &Vec<usize>,
    from: usize,
) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
        roots_valid(arena.nodes@, args@),
        front <= db.len(),
    ensures
        out matches Some(m) ==> {
            &&& from <= m < front
            &&& candidate(
                db_view(arena.nodes@, db@)[m as int],
                name@,
                args.len() as nat,
                first_class(root_terms(arena.nodes@, args@)),
            )
            &&& forall|k: int|
                from <= k < m ==> !candidate(
                    #[trigger] db_view(arena.nodes@, db@)[k],
                    name@,
                    args.len() as nat,
                    first_class(root_terms(arena.nodes@, args@)),
                )
        },
        out is None ==> forall|k: int|
            from <= k < front ==> !candidate(
                #[trigger] db_view(arena.nodes@, db@)[k],
                name@,
                args.len() as nat,
                first_class(root_terms(arena.nodes@, args@)),
            ),
{
    let ghost d = db_view(arena.nodes@, db@);
    let ghost gc = first_class(root_terms(arena.nodes@, args@));
    let mut j = from;
    while j < front
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            roots_valid(arena.nodes@, args@),
            d == db_view(arena.nodes@, db@),
            gc == first_class(root_terms(arena.nodes@, args@)),
            front <= db.len(),
            from <= j,
            forall|k: int|
                from <= k < j ==> !candidate(#[trigger] d[k], name@, args.len() as nat, gc),
        decreases front - j,
    {
        proof {
            assert(clause_valid(arena.nodes@, &db@[j as int]));
        }
        if is_candidate(arena, &db[j], name, args) {
            return Some(j);
        }
        j += 1;
    }
    None
}

// The least position at or after `start` that `sel` selects in `b`, through its exact list.
fn list_from(
    list: &Vec<usize>,
    Ghost(b): Ghost<Seq<DocClause>>,
    Ghost(name): Ghost<Seq<u8>>,
    Ghost(arity): Ghost<nat>,
    Ghost(sel): Ghost<Sel>,
    start: usize,
) -> (out: Option<usize>)
    requires
        exact(list@, b, b.len(), name, arity, sel),
    ensures
        out matches Some(v) ==> {
            &&& start <= v < b.len()
            &&& selects(b[v as int], name, arity, sel)
            &&& forall|k: int| start <= k < v ==> !selects(#[trigger] b[k], name, arity, sel)
        },
        out is None ==> forall|k: int|
            start <= k < b.len() ==> !selects(#[trigger] b[k], name, arity, sel),
{
    let v = lower_bound(list, start);
    proof {
        let bound = match v {
            Some(x) => x as int,
            None => b.len() as int,
        };
        lower_gap(list@, b, name, arity, sel, start as int, bound);
        if let Some(x) = v {
            let i = choose|i: int| 0 <= i < list@.len() && list@[i] == x;
        }
    }
    v
}

// The least candidate at or after `start` among the clauses of `b` that one
// predicate entry indexes (D4): `all` for a classless goal, else the earlier of
// its class list and `wild`.
fn entry_from(
    arena: &ETermArena,
    entry: &EPredIndex,
    Ghost(b): Ghost<Seq<DocClause>>,
    args: &Vec<usize>,
    start: usize,
) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, args@),
        pred_over(*entry, b),
        entry.arity == args.len(),
    ensures
        out matches Some(v) ==> {
            &&& start <= v < b.len()
            &&& candidate(
                b[v as int],
                entry.name@,
                args.len() as nat,
                first_class(root_terms(arena.nodes@, args@)),
            )
            &&& forall|k: int|
                start <= k < v ==> !candidate(
                    #[trigger] b[k],
                    entry.name@,
                    args.len() as nat,
                    first_class(root_terms(arena.nodes@, args@)),
                )
        },
        out is None ==> forall|k: int|
            start <= k < b.len() ==> !candidate(
                #[trigger] b[k],
                entry.name@,
                args.len() as nat,
                first_class(root_terms(arena.nodes@, args@)),
            ),
{
    let ghost gc = first_class(root_terms(arena.nodes@, args@));
    let ghost name = entry.name@;
    let ghost arity = args.len() as nat;
    proof {
        reveal(roots_valid);
    }
    if args.len() == 0 || classless(arena, args[0]) {
        proof {
            assert(gc is None);
        }
        return list_from(&entry.all, Ghost(b), Ghost(name), Ghost(arity), Ghost(Sel::All), start);
    }
    proof {
        assert(gc == class_of(arena@[args@[0] as int]));
    }
    let ghost k = gc->0;
    let w = list_from(&entry.wild, Ghost(b), Ghost(name), Ghost(arity), Ghost(Sel::Wild), start);
    let c = match find_class(arena, entry, args[0]) {
        None => {
            proof {
                assert forall|j: int| 0 <= j < b.len() implies !selects(
                    #[trigger] b[j],
                    name,
                    arity,
                    Sel::Class(k),
                ) by {
                    if selects(b[j], name, arity, Sel::Class(k)) {
                        assert(selects(b[j], name, arity, Sel::All));
                        let r = choose|r: int|
                            0 <= r < entry.classes@.len() && Some(
                                class_view(entry.classes@[r].class),
                            ) == head_class(b[j]);
                        assert(class_of(arena@[args@[0] as int]) != Some(
                            class_view(entry.classes@[r].class),
                        ));
                    }
                }
            }
            None
        },
        Some(q) => {
            proof {
                assert(Sel::Class(k) == Sel::Class(class_view(entry.classes@[q as int].class)));
            }
            list_from(
                &entry.classes[q].list,
                Ghost(b),
                Ghost(name),
                Ghost(arity),
                Ghost(Sel::Class(k)),
                start,
            )
        },
    };
    match (w, c) {
        (Some(x), Some(y)) => if x <= y {
            Some(x)
        } else {
            Some(y)
        },
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

// The least candidate at or after `from`; none from |db| on (D4).
pub fn next_candidate(
    arena: &ETermArena,
    db: &Vec<EClause>,
    ix: &EClauseIndex,
    front: usize,
    name: &Vec<u8>,
    args: &Vec<usize>,
    from: usize,
) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
        roots_valid(arena.nodes@, args@),
        index_valid(*ix, db_view(arena.nodes@, db@), front as nat),
    ensures
        match out {
            Some(m) => {
                &&& from <= m < db.len()
                &&& candidate(
                    db_view(arena.nodes@, db@)[m as int],
                    name@,
                    args.len() as nat,
                    first_class(root_terms(arena.nodes@, args@)),
                )
                &&& forall|j: int|
                    from <= j < m ==> !candidate(
                        #[trigger] db_view(arena.nodes@, db@)[j],
                        name@,
                        args.len() as nat,
                        first_class(root_terms(arena.nodes@, args@)),
                    )
            },
            None => forall|j: int|
                from <= j < db.len() ==> !candidate(
                    #[trigger] db_view(arena.nodes@, db@)[j],
                    name@,
                    args.len() as nat,
                    first_class(root_terms(arena.nodes@, args@)),
                ),
        },
{
    let ghost d = db_view(arena.nodes@, db@);
    let ghost gc = first_class(root_terms(arena.nodes@, args@));
    let ghost b = d.skip(front as int);
    if from >= db.len() {
        return None;
    }
    if let Some(m) = prefix_scan(arena, db, front, name, args, from) {
        return Some(m);
    }
    let start = from.saturating_sub(front);
    let found = match find_pred(ix, name, args.len()) {
        None => {
            proof {
                assert forall|k: int| 0 <= k < b.len() implies !candidate(
                    #[trigger] b[k],
                    name@,
                    args.len() as nat,
                    gc,
                ) by {
                    if lit_fa(b[k].head) == Some((name@, args.len() as nat)) {
                        let p = choose|p: int|
                            0 <= p < ix.preds@.len() && lit_fa(b[k].head) == Some(
                                (ix.preds@[p].name@, ix.preds@[p].arity as nat),
                            );
                        assert(!(ix.preds@[p].name@ == name@ && ix.preds@[p].arity == args.len()));
                    }
                }
            }
            None
        },
        Some(p) => {
            proof {
                assert(pred_over(ix.preds@[p as int], b));
            }
            entry_from(arena, &ix.preds[p], Ghost(b), args, start)
        },
    };
    proof {
        assert forall|k: int| 0 <= k < b.len() implies #[trigger] b[k] == d[front + k] by {}
        assert forall|j: int| front <= j < d.len() implies d[j] == #[trigger] b[j - front] by {}
    }
    match found {
        Some(v) => Some(front + v),
        None => None,
    }
}

} // verus!
