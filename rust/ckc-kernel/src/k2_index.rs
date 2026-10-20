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
}

pub struct EClauseIndex {
    pub preds: Vec<EPredIndex>,
}

// Each list exact over `b`; every classed clause of the predicate has its class list.
pub open spec fn pred_over(p: EPredIndex, b: Seq<DocClause>) -> bool {
    &&& exact(p.all@, b, b.len(), p.name@, p.arity as nat, Sel::All)
    &&& exact(p.wild@, b, b.len(), p.name@, p.arity as nat, Sel::Wild)
    &&& forall|q: int|
        0 <= q < p.classes@.len() ==> exact(
            (#[trigger] p.classes@[q]).list@,
            b,
            b.len(),
            p.name@,
            p.arity as nat,
            Sel::Class(class_view(p.classes@[q].class)),
        )
    &&& forall|j: int|
        0 <= j < b.len() && #[trigger] selects(b[j], p.name@, p.arity as nat, Sel::All)
            && head_class(b[j]) is Some ==> exists|q: int|
            0 <= q < p.classes@.len() && Some(class_view(p.classes@[q].class)) == head_class(b[j])
}

// Every entry exact over `b`; every compound head has its predicate's entry.
pub open spec fn index_over(ix: EClauseIndex, b: Seq<DocClause>) -> bool {
    &&& forall|p: int| 0 <= p < ix.preds@.len() ==> pred_over(#[trigger] ix.preds@[p], b)
    &&& forall|j: int|
        0 <= j < b.len() && #[trigger] lit_fa(b[j].head) is Some ==> exists|p: int|
            0 <= p < ix.preds@.len() && lit_fa(b[j].head) == Some(
                (ix.preds@[p].name@, ix.preds@[p].arity as nat),
            )
}

// The index covers db[front..]; positions below `front` (aggregate witness
// facts) are scanned directly.
pub open spec fn index_valid(ix: EClauseIndex, db: Seq<DocClause>, front: nat) -> bool {
    front <= db.len() && index_over(ix, db.skip(front as int))
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

// A stretch of unselected positions extends the bound.
proof fn exact_jump(
    list: Seq<usize>,
    b: Seq<DocClause>,
    n: nat,
    m: nat,
    name: Seq<u8>,
    arity: nat,
    sel: Sel,
)
    requires
        exact(list, b, n, name, arity, sel),
        n <= m <= b.len(),
        forall|j: int| n <= j < m ==> !selects(#[trigger] b[j], name, arity, sel),
    ensures
        exact(list, b, m, name, arity, sel),
{
    assert forall|j: int|
        0 <= j < m && #[trigger] selects(b[j], name, arity, sel) implies list.contains(
        j as usize,
    ) by {
        if j >= n {
            assert(!selects(b[j], name, arity, sel));
        }
    }
}

// Between consecutive entries of an exact `all` list nothing of the predicate lies.
proof fn all_gap(all: Seq<usize>, b: Seq<DocClause>, name: Seq<u8>, arity: nat, i: int, j: int)
    requires
        exact(all, b, b.len(), name, arity, Sel::All),
        0 <= i <= all.len(),
        0 <= j < b.len(),
        i > 0 ==> all[i - 1] < j,
        i < all.len() ==> j < all[i],
    ensures
        !selects(b[j], name, arity, Sel::All),
{
    if selects(b[j], name, arity, Sel::All) {
        assert(all.contains(j as usize));
        let k = choose|k: int| 0 <= k < all.len() && all[k] == j as usize;
        if k < i {
            if k < i - 1 {
                assert(all[k] < all[i - 1]);
            }
        } else {
            if k > i {
                assert(all[i] < all[k]);
            }
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

fn class_clone(c: &EClass) -> (out: EClass)
    ensures
        class_view(out) == class_view(*c),
{
    match c {
        EClass::Atom(name) => {
            let copy = name.clone();
            proof {
                assert(copy@ =~= name@);
            }
            EClass::Atom(copy)
        },
        EClass::Comp(name, arity) => {
            let copy = name.clone();
            proof {
                assert(copy@ =~= name@);
            }
            EClass::Comp(copy, *arity)
        },
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

// selects(the clause, name, arity, Wild | Class(c)): `class` none = Wild.
fn selects_exec(
    arena: &ETermArena,
    clause: &EClause,
    name: &Vec<u8>,
    arity: usize,
    class: Option<&EClass>,
) -> (out: bool)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, clause),
    ensures
        out == selects(
            clause_view(arena.nodes@, clause),
            name@,
            arity as nat,
            match class {
                None => Sel::Wild,
                Some(c) => Sel::Class(class_view(*c)),
            },
        ),
{
    if !literal_matches(arena, clause.head, name, arity) {
        return false;
    }
    match (head_first(arena, clause.head), class) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(r), None) => classless(arena, r),
        (Some(r), Some(c)) => class_matches(arena, r, c),
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

// --- build (D3) ---
fn positions_all(arena: &ETermArena, db: &Vec<EClause>, name: &Vec<u8>, arity: usize) -> (out: Vec<
    usize,
>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
    ensures
        exact(out@, db_view(arena.nodes@, db@), db.len() as nat, name@, arity as nat, Sel::All),
{
    let ghost b = db_view(arena.nodes@, db@);
    let mut out: Vec<usize> = Vec::new();
    let mut j = 0usize;
    while j < db.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            b == db_view(arena.nodes@, db@),
            j <= db.len(),
            exact(out@, b, j as nat, name@, arity as nat, Sel::All),
        decreases db.len() - j,
    {
        proof {
            assert(clause_valid(arena.nodes@, &db@[j as int]));
        }
        let hit = literal_matches(arena, db[j].head, name, arity);
        proof {
            exact_step(out@, b, j as nat, name@, arity as nat, Sel::All, hit);
        }
        if hit {
            out.push(j);
        }
        j += 1;
    }
    out
}

// The positions of `all` whose clause `class` selects (none = Wild), in order.
fn refine(
    arena: &ETermArena,
    db: &Vec<EClause>,
    all: &Vec<usize>,
    name: &Vec<u8>,
    arity: usize,
    class: Option<&EClass>,
) -> (out: Vec<usize>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
        exact(all@, db_view(arena.nodes@, db@), db.len() as nat, name@, arity as nat, Sel::All),
    ensures
        exact(
            out@,
            db_view(arena.nodes@, db@),
            db.len() as nat,
            name@,
            arity as nat,
            match class {
                None => Sel::Wild,
                Some(c) => Sel::Class(class_view(*c)),
            },
        ),
{
    let ghost b = db_view(arena.nodes@, db@);
    let ghost sel = match class {
        None => Sel::Wild,
        Some(c) => Sel::Class(class_view(*c)),
    };
    let mut out: Vec<usize> = Vec::new();
    let mut i = 0usize;
    proof {
        let lim = if all@.len() > 0 {
            all@[0] as nat
        } else {
            b.len()
        };
        assert forall|j: int| 0 <= j < lim implies !selects(
            #[trigger] b[j],
            name@,
            arity as nat,
            sel,
        ) by {
            all_gap(all@, b, name@, arity as nat, 0, j);
        }
        exact_jump(out@, b, 0, lim, name@, arity as nat, sel);
    }
    while i < all.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            b == db_view(arena.nodes@, db@),
            sel == match class {
                None => Sel::Wild,
                Some(c) => Sel::Class(class_view(*c)),
            },
            exact(all@, b, b.len(), name@, arity as nat, Sel::All),
            i <= all.len(),
            exact(
                out@,
                b,
                if i < all@.len() {
                    all@[i as int] as nat
                } else {
                    b.len()
                },
                name@,
                arity as nat,
                sel,
            ),
        decreases all.len() - i,
    {
        let v = all[i];
        proof {
            assert(clause_valid(arena.nodes@, &db@[v as int]));
        }
        let hit = selects_exec(arena, &db[v], name, arity, class);
        proof {
            exact_step(out@, b, v as nat, name@, arity as nat, sel, hit);
            let next = if hit {
                out@.push(v)
            } else {
                out@
            };
            let lim = if i + 1 < all@.len() {
                all@[i + 1] as nat
            } else {
                b.len()
            };
            if i + 1 < all@.len() {
                assert(all@[i as int] < all@[i + 1]);
            }
            assert forall|j: int| v + 1 <= j < lim implies !selects(
                #[trigger] b[j],
                name@,
                arity as nat,
                sel,
            ) by {
                all_gap(all@, b, name@, arity as nat, i + 1, j);
            }
            exact_jump(next, b, v as nat + 1, lim, name@, arity as nat, sel);
        }
        if hit {
            out.push(v);
        }
        i += 1;
    }
    out
}

fn find_kind(arena: &ETermArena, root: usize, kinds: &Vec<EClass>) -> (out: Option<usize>)
    requires
        root_ok(arena, root),
    ensures
        out matches Some(q) ==> q < kinds@.len() && class_of(arena@[root as int]) == Some(
            class_view(kinds@[q as int]),
        ),
        out is None ==> forall|q: int|
            0 <= q < kinds@.len() ==> class_of(arena@[root as int]) != Some(
                class_view(#[trigger] kinds@[q]),
            ),
{
    let mut q = 0usize;
    while q < kinds.len()
        invariant
            root_ok(arena, root),
            q <= kinds@.len(),
            forall|k: int|
                0 <= k < q ==> class_of(arena@[root as int]) != Some(
                    class_view(#[trigger] kinds@[k]),
                ),
        decreases kinds.len() - q,
    {
        if class_matches(arena, root, &kinds[q]) {
            return Some(q);
        }
        q += 1;
    }
    None
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

// One class per classed head among the clauses at `all` (first-occurrence order).
fn classes_of(arena: &ETermArena, db: &Vec<EClause>, all: &Vec<usize>) -> (out: Vec<EClass>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
        forall|i: int| 0 <= i < all@.len() ==> #[trigger] all@[i] < db.len(),
    ensures
        forall|i: int|
            0 <= i < all@.len() && head_class(
                db_view(arena.nodes@, db@)[#[trigger] all@[i] as int],
            ) is Some ==> exists|q: int|
                0 <= q < out@.len() && Some(class_view(out@[q])) == head_class(
                    db_view(arena.nodes@, db@)[all@[i] as int],
                ),
{
    let ghost b = db_view(arena.nodes@, db@);
    let mut out: Vec<EClass> = Vec::new();
    let mut i = 0usize;
    while i < all.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            b == db_view(arena.nodes@, db@),
            forall|k: int| 0 <= k < all@.len() ==> #[trigger] all@[k] < db.len(),
            i <= all.len(),
            forall|k: int|
                0 <= k < i && head_class(b[#[trigger] all@[k] as int]) is Some ==> exists|q: int|
                    0 <= q < out@.len() && Some(class_view(out@[q])) == head_class(
                        b[all@[k] as int],
                    ),
        decreases all.len() - i,
    {
        let v = all[i];
        let ghost before = out@;
        proof {
            assert(clause_valid(arena.nodes@, &db@[v as int]));
            assert(b[v as int] == clause_view(arena.nodes@, &db@[v as int]));
        }
        let first = head_first(arena, db[v].head);
        proof {
            head_class_node(arena, &db@[v as int], first);
        }
        if let Some(r) = first {
            if !classless(arena, r) {
                if let Some(q) = find_kind(arena, r, &out) {
                    proof {
                        assert(Some(class_view(out@[q as int])) == head_class(b[v as int]));
                    }
                } else {
                    if let Some(c) = node_class(arena, r) {
                        out.push(c);
                        proof {
                            assert(Some(class_view(out@[before.len() as int])) == head_class(
                                b[v as int],
                            ));
                        }
                    }
                }
            }
        }
        proof {
            assert forall|k: int|
                0 <= k < i + 1 && head_class(b[#[trigger] all@[k] as int]) is Some implies exists|
                q: int,
            |
                0 <= q < out@.len() && Some(class_view(out@[q])) == head_class(
                    b[all@[k] as int],
                ) by {
                if k < i {
                    let q = choose|q: int|
                        0 <= q < before.len() && Some(class_view(before[q])) == head_class(
                            b[all@[k] as int],
                        );
                    assert(out@[q] == before[q]);
                }
            }
        }
        i += 1;
    }
    out
}

fn find_key(keys: &Vec<(Vec<u8>, usize)>, name: &Vec<u8>, arity: usize) -> (out: Option<usize>)
    ensures
        out matches Some(p) ==> p < keys@.len() && keys@[p as int].0@ == name@ && keys@[p as int].1
            == arity,
        out is None ==> forall|p: int|
            0 <= p < keys@.len() ==> !((#[trigger] keys@[p]).0@ == name@ && keys@[p].1 == arity),
{
    let mut p = 0usize;
    while p < keys.len()
        invariant
            p <= keys@.len(),
            forall|k: int|
                0 <= k < p ==> !((#[trigger] keys@[k]).0@ == name@ && keys@[k].1 == arity),
        decreases keys.len() - p,
    {
        if keys[p].1 == arity && vec_equal(&keys[p].0, name) {
            return Some(p);
        }
        p += 1;
    }
    None
}

// One (name, arity) per predicate some compound head names (first-occurrence order).
fn collect_keys(arena: &ETermArena, db: &Vec<EClause>) -> (keys: Vec<(Vec<u8>, usize)>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
    ensures
        forall|j: int|
            0 <= j < db.len() && #[trigger] lit_fa(db_view(arena.nodes@, db@)[j].head) is Some
                ==> exists|p: int|
                0 <= p < keys@.len() && lit_fa(db_view(arena.nodes@, db@)[j].head) == Some(
                    (keys@[p].0@, keys@[p].1 as nat),
                ),
{
    let ghost b = db_view(arena.nodes@, db@);
    let mut keys: Vec<(Vec<u8>, usize)> = Vec::new();
    let mut j = 0usize;
    while j < db.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            b == db_view(arena.nodes@, db@),
            j <= db.len(),
            forall|k: int|
                0 <= k < j && #[trigger] lit_fa(b[k].head) is Some ==> exists|p: int|
                    0 <= p < keys@.len() && lit_fa(b[k].head) == Some(
                        (keys@[p].0@, keys@[p].1 as nat),
                    ),
        decreases db.len() - j,
    {
        let head = db[j].head;
        proof {
            assert(clause_valid(arena.nodes@, &db@[j as int]));
            assert(node_ok(arena.nodes@, head as int));
        }
        match &arena.nodes[head].kind {
            ENodeKind::Comp { name, child_roots, .. } => {
                proof {
                    node_comp_model(arena.nodes@, head as int, name@, child_roots@);
                }
                let found = find_key(&keys, name, child_roots.len());
                if let Some(p) = found {
                    proof {
                        assert(lit_fa(b[j as int].head) == Some(
                            (keys@[p as int].0@, keys@[p as int].1 as nat),
                        ));
                    }
                } else {
                    let copy = name.clone();
                    proof {
                        assert(copy@ =~= name@);
                    }
                    let ghost before = keys@;
                    keys.push((copy, child_roots.len()));
                    proof {
                        assert forall|k: int|
                            0 <= k < j && #[trigger] lit_fa(b[k].head) is Some implies exists|
                            p2: int,
                        |
                            0 <= p2 < keys@.len() && lit_fa(b[k].head) == Some(
                                (keys@[p2].0@, keys@[p2].1 as nat),
                            ) by {
                            let p2 = choose|p2: int|
                                0 <= p2 < before.len() && lit_fa(b[k].head) == Some(
                                    (before[p2].0@, before[p2].1 as nat),
                                );
                            assert(keys@[p2] == before[p2]);
                        }
                        assert(lit_fa(b[j as int].head) == Some(
                            (keys@[before.len() as int].0@, keys@[before.len() as int].1 as nat),
                        ));
                    }
                }
            },
            _ => {},
        }
        j += 1;
    }
    keys
}

pub fn build_index(arena: &ETermArena, db: &Vec<EClause>) -> (ix: EClauseIndex)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
    ensures
        index_over(ix, db_view(arena.nodes@, db@)),
{
    let ghost b = db_view(arena.nodes@, db@);
    let keys = collect_keys(arena, db);
    let mut preds: Vec<EPredIndex> = Vec::new();
    let mut p = 0usize;
    while p < keys.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            b == db_view(arena.nodes@, db@),
            p <= keys.len(),
            preds@.len() == p,
            forall|q: int| 0 <= q < p ==> pred_over(#[trigger] preds@[q], b),
            forall|q: int|
                0 <= q < p ==> (#[trigger] preds@[q]).name@ == keys@[q].0@ && preds@[q].arity
                    == keys@[q].1,
        decreases keys.len() - p,
    {
        let name = keys[p].0.clone();
        let arity = keys[p].1;
        proof {
            assert(name@ =~= keys@[p as int].0@);
        }
        let all = positions_all(arena, db, &name, arity);
        let wild = refine(arena, db, &all, &name, arity, None);
        proof {
            assert forall|i: int| 0 <= i < all@.len() implies #[trigger] all@[i] < db.len() by {}
        }
        let kinds = classes_of(arena, db, &all);
        let mut classes: Vec<EClassList> = Vec::new();
        let mut q = 0usize;
        while q < kinds.len()
            invariant
                arena_ok(arena),
                db_valid(arena.nodes@, db@),
                b == db_view(arena.nodes@, db@),
                exact(all@, b, b.len(), name@, arity as nat, Sel::All),
                q <= kinds.len(),
                classes@.len() == q,
                forall|r: int|
                    0 <= r < q ==> exact(
                        (#[trigger] classes@[r]).list@,
                        b,
                        b.len(),
                        name@,
                        arity as nat,
                        Sel::Class(class_view(classes@[r].class)),
                    ),
                forall|r: int|
                    0 <= r < q ==> class_view((#[trigger] classes@[r]).class) == class_view(
                        kinds@[r],
                    ),
            decreases kinds.len() - q,
        {
            let list = refine(arena, db, &all, &name, arity, Some(&kinds[q]));
            let class = class_clone(&kinds[q]);
            classes.push(EClassList { class, list });
            q += 1;
        }
        let entry = EPredIndex { name, arity, all, wild, classes };
        proof {
            assert forall|j: int|
                0 <= j < b.len() && #[trigger] selects(
                    b[j],
                    entry.name@,
                    entry.arity as nat,
                    Sel::All,
                ) && head_class(b[j]) is Some implies exists|r: int|
                0 <= r < entry.classes@.len() && Some(class_view(entry.classes@[r].class))
                    == head_class(b[j]) by {
                assert(entry.all@.contains(j as usize));
                let i = choose|i: int| 0 <= i < entry.all@.len() && entry.all@[i] == j as usize;
                assert(head_class(b[entry.all@[i] as int]) is Some);
                let r = choose|r: int|
                    0 <= r < kinds@.len() && Some(class_view(kinds@[r])) == head_class(
                        b[entry.all@[i] as int],
                    );
                assert(class_view(entry.classes@[r].class) == class_view(kinds@[r]));
            }
            assert(pred_over(entry, b));
        }
        preds.push(entry);
        p += 1;
    }
    let ix = EClauseIndex { preds };
    proof {
        assert forall|j: int|
            0 <= j < b.len() && #[trigger] lit_fa(b[j].head) is Some implies exists|q: int|
            0 <= q < ix.preds@.len() && lit_fa(b[j].head) == Some(
                (ix.preds@[q].name@, ix.preds@[q].arity as nat),
            ) by {
            let q = choose|q: int|
                0 <= q < keys@.len() && lit_fa(b[j].head) == Some((keys@[q].0@, keys@[q].1 as nat));
            assert(ix.preds@[q].name@ == keys@[q].0@);
        }
    }
    ix
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
    let mut p = 0usize;
    while p < ix.preds.len()
        invariant
            p <= ix.preds@.len(),
            forall|q: int|
                0 <= q < p ==> !((#[trigger] ix.preds@[q]).name@ == name@ && ix.preds@[q].arity
                    == arity),
        decreases ix.preds.len() - p,
    {
        if ix.preds[p].arity == arity && vec_equal(&ix.preds[p].name, name) {
            return Some(p);
        }
        p += 1;
    }
    None
}

fn find_class(arena: &ETermArena, entry: &EPredIndex, root: usize) -> (out: Option<usize>)
    requires
        root_ok(arena, root),
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
    let mut q = 0usize;
    while q < entry.classes.len()
        invariant
            root_ok(arena, root),
            q <= entry.classes@.len(),
            forall|k: int|
                0 <= k < q ==> class_of(arena@[root as int]) != Some(
                    class_view((#[trigger] entry.classes@[k]).class),
                ),
        decreases entry.classes.len() - q,
    {
        if class_matches(arena, root, &entry.classes[q].class) {
            return Some(q);
        }
        q += 1;
    }
    None
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
