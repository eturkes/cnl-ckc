use crate::k2_engine::{EBodyItem, EClause};
use crate::k2_term::{ENodeKind, ETermArena};
use ckc_spec::term::Term;
use ckc_spec::v1text::{BodyItem, DocClause};
use vstd::prelude::*;

verus! {

// Contract mc D3: left-to-right safety, decided by occurrence tests over the clause's variables
// (all below n); a variable set = one flag per variable below n.
pub open spec fn set_of(bv: Seq<bool>, b: Set<nat>) -> bool {
    forall|x: nat| #[trigger] b.contains(x) <==> (x < bv.len() && bv[x as int])
}

// Every variable of c is below n.
pub open spec fn vars_below(c: DocClause, n: nat) -> bool {
    &&& forall|x: nat| #[trigger] ckc_spec::model::vars(c.head).contains(x) ==> x < n
    &&& forall|j: int, x: nat|
        0 <= j < c.body.len() && #[trigger] ckc_spec::model::item_vars(c.body[j]).contains(x) ==> x
            < n
}

pub proof fn occurs_all_any(x: nat, ts: Seq<Term>)
    ensures
        ckc_spec::engine::occurs_all(x, ts) == exists|k: int|
            0 <= k < ts.len() && #[trigger] ckc_spec::engine::occurs(x, ts[k]),
    decreases ts.len(),
{
    if ts.len() > 0 {
        let rest = ts.drop_first();
        occurs_all_any(x, rest);
        if ckc_spec::engine::occurs_all(x, ts) {
            if !ckc_spec::engine::occurs(x, ts[0]) {
                let k = choose|k: int|
                    0 <= k < rest.len() && #[trigger] ckc_spec::engine::occurs(x, rest[k]);
                assert(rest[k] == ts[k + 1]);
            }
        } else {
            assert forall|k: int| 0 <= k < ts.len() implies !#[trigger] ckc_spec::engine::occurs(
                x,
                ts[k],
            ) by {
                if k > 0 {
                    assert(rest[k - 1] == ts[k]);
                }
            }
        }
    }
}

pub proof fn vars_occurs(t: Term, x: nat)
    ensures
        ckc_spec::model::vars(t).contains(x) == ckc_spec::engine::occurs(x, t),
{
    crate::k3_safe::stream_occurs(t, x);
    ckc_spec::term::var_stream(t).to_set_ensures();
}

pub proof fn vars_all_any(ts: Seq<Term>, x: nat)
    ensures
        ckc_spec::model::vars_all(ts).contains(x) == exists|k: int|
            0 <= k < ts.len() && #[trigger] ckc_spec::engine::occurs(x, ts[k]),
{
    crate::k3_safe::stream_all_occurs(ts, x);
    ckc_spec::term::var_stream_all(ts).to_set_ensures();
    occurs_all_any(x, ts);
}

// A compound's variables = its arguments' (vars of Comp = vars_all of its args).
pub proof fn vars_comp(t: Term, x: nat)
    requires
        t is Comp,
    ensures
        ckc_spec::model::vars(t).contains(x) == ckc_spec::model::vars_all(t->Comp_1).contains(x),
{
    vars_occurs(t, x);
    crate::k3_safe::stream_all_occurs(t->Comp_1, x);
    ckc_spec::term::var_stream_all(t->Comp_1).to_set_ensures();
}

// A variable of a subrange is a variable of the whole sequence.
pub proof fn vars_all_sub(ts: Seq<Term>, lo: int, hi: int, x: nat)
    requires
        0 <= lo <= hi <= ts.len(),
        ckc_spec::model::vars_all(ts.subrange(lo, hi)).contains(x),
    ensures
        ckc_spec::model::vars_all(ts).contains(x),
{
    vars_all_any(ts.subrange(lo, hi), x);
    vars_all_any(ts, x);
    let k = choose|k: int|
        0 <= k < hi - lo && #[trigger] ckc_spec::engine::occurs(x, ts.subrange(lo, hi)[k]);
    assert(ts.subrange(lo, hi)[k] == ts[lo + k]);
}

pub fn none_flags(n: usize) -> (out: Vec<bool>)
    ensures
        out@.len() == n,
        set_of(out@, Set::empty()),
{
    let mut out: Vec<bool> = Vec::new();
    let mut x = 0usize;
    while x < n
        invariant
            x <= n,
            out@.len() == x,
            forall|y: int| 0 <= y < x ==> !out@[y],
        decreases n - x,
    {
        out.push(false);
        x += 1;
    }
    out
}

pub fn term_flags(arena: &ETermArena, root: usize, n: usize) -> (out: Vec<bool>)
    requires
        crate::k2_term::root_ok(arena, root),
        forall|x: nat| #[trigger] ckc_spec::model::vars(arena@[root as int]).contains(x) ==> x < n,
    ensures
        out@.len() == n,
        set_of(out@, ckc_spec::model::vars(arena@[root as int])),
{
    let ghost vs = ckc_spec::model::vars(arena@[root as int]);
    let mut out: Vec<bool> = Vec::new();
    let mut x = 0usize;
    while x < n
        invariant
            crate::k2_term::root_ok(arena, root),
            vs == ckc_spec::model::vars(arena@[root as int]),
            forall|y: nat| #[trigger] vs.contains(y) ==> y < n,
            x <= n,
            out@.len() == x,
            forall|y: int| 0 <= y < x ==> out@[y] == vs.contains(y as nat),
        decreases n - x,
    {
        proof {
            vars_occurs(arena@[root as int], x as nat);
        }
        out.push(crate::k2_engine::occurs_root(arena, x, root));
        x += 1;
    }
    proof {
        assert forall|y: nat| #[trigger] vs.contains(y) <==> (y < out@.len() && out@[y as int]) by {
            if vs.contains(y) {
                assert(y < n);
            }
        }
    }
    out
}

// x occurs in one of the terms at roots[from..to].
fn occ_range(arena: &ETermArena, x: usize, roots: &Vec<usize>, from: usize, to: usize) -> (out:
    bool)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::roots_valid(arena.nodes@, roots@),
        from <= to <= roots@.len(),
    ensures
        out == ckc_spec::model::vars_all(
            crate::k2_engine::root_terms(arena.nodes@, roots@).subrange(from as int, to as int),
        ).contains(x as nat),
{
    let ghost ts = crate::k2_engine::root_terms(arena.nodes@, roots@).subrange(
        from as int,
        to as int,
    );
    let mut k = from;
    while k < to
        invariant
            crate::k2_term::arena_ok(arena),
            crate::k2_engine::roots_valid(arena.nodes@, roots@),
            from <= k <= to <= roots@.len(),
            ts == crate::k2_engine::root_terms(arena.nodes@, roots@).subrange(
                from as int,
                to as int,
            ),
            forall|q: int|
                0 <= q < k - from ==> !#[trigger] ckc_spec::engine::occurs(x as nat, ts[q]),
        decreases to - k,
    {
        proof {
            assert(ts[k - from] == arena@[roots@[k as int] as int]);
        }
        if crate::k2_engine::occurs_root(arena, x, roots[k]) {
            proof {
                assert(ckc_spec::engine::occurs(x as nat, ts[k - from]));
                vars_all_any(ts, x as nat);
            }
            return true;
        }
        k += 1;
    }
    proof {
        vars_all_any(ts, x as nat);
    }
    false
}

fn range_flags(arena: &ETermArena, roots: &Vec<usize>, from: usize, to: usize, n: usize) -> (out:
    Vec<bool>)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::roots_valid(arena.nodes@, roots@),
        from <= to <= roots@.len(),
        forall|x: nat| #[trigger]
            ckc_spec::model::vars_all(
                crate::k2_engine::root_terms(arena.nodes@, roots@).subrange(from as int, to as int),
            ).contains(x) ==> x < n,
    ensures
        out@.len() == n,
        set_of(
            out@,
            ckc_spec::model::vars_all(
                crate::k2_engine::root_terms(arena.nodes@, roots@).subrange(from as int, to as int),
            ),
        ),
{
    let ghost vs = ckc_spec::model::vars_all(
        crate::k2_engine::root_terms(arena.nodes@, roots@).subrange(from as int, to as int),
    );
    let mut out: Vec<bool> = Vec::new();
    let mut x = 0usize;
    while x < n
        invariant
            crate::k2_term::arena_ok(arena),
            crate::k2_engine::roots_valid(arena.nodes@, roots@),
            from <= to <= roots@.len(),
            vs == ckc_spec::model::vars_all(
                crate::k2_engine::root_terms(arena.nodes@, roots@).subrange(from as int, to as int),
            ),
            forall|y: nat| #[trigger] vs.contains(y) ==> y < n,
            x <= n,
            out@.len() == x,
            forall|y: int| 0 <= y < x ==> out@[y] == vs.contains(y as nat),
        decreases n - x,
    {
        out.push(occ_range(arena, x, roots, from, to));
        x += 1;
    }
    proof {
        assert forall|y: nat| #[trigger] vs.contains(y) <==> (y < out@.len() && out@[y as int]) by {
            if vs.contains(y) {
                assert(y < n);
            }
        }
    }
    out
}

pub fn union_flags(
    a: &Vec<bool>,
    s: &Vec<bool>,
    Ghost(b): Ghost<Set<nat>>,
    Ghost(t): Ghost<Set<nat>>,
) -> (out: Vec<bool>)
    requires
        a@.len() == s@.len(),
        set_of(a@, b),
        set_of(s@, t),
    ensures
        out@.len() == a@.len(),
        set_of(out@, b.union(t)),
{
    let mut out: Vec<bool> = Vec::new();
    let mut x = 0usize;
    while x < a.len()
        invariant
            a@.len() == s@.len(),
            set_of(a@, b),
            set_of(s@, t),
            x <= a@.len(),
            out@.len() == x,
            forall|y: int| 0 <= y < x ==> out@[y] == (a@[y] || s@[y]),
        decreases a.len() - x,
    {
        out.push(a[x] || s[x]);
        x += 1;
    }
    proof {
        assert forall|y: nat| #[trigger]
            b.union(t).contains(y) <==> (y < out@.len() && out@[y as int]) by {
            assert(b.contains(y) <==> (y < a@.len() && a@[y as int]));
            assert(t.contains(y) <==> (y < s@.len() && s@[y as int]));
        }
    }
    out
}

pub fn subset_flags(
    s: &Vec<bool>,
    a: &Vec<bool>,
    Ghost(t): Ghost<Set<nat>>,
    Ghost(b): Ghost<Set<nat>>,
) -> (out: bool)
    requires
        s@.len() == a@.len(),
        set_of(s@, t),
        set_of(a@, b),
    ensures
        out == t.subset_of(b),
{
    let mut x = 0usize;
    while x < s.len()
        invariant
            s@.len() == a@.len(),
            set_of(s@, t),
            set_of(a@, b),
            x <= s@.len(),
            forall|y: int| 0 <= y < x ==> (s@[y] ==> a@[y]),
        decreases s.len() - x,
    {
        if s[x] && !a[x] {
            proof {
                assert(t.contains(x as nat));
                assert(!b.contains(x as nat));
            }
            return false;
        }
        x += 1;
    }
    proof {
        assert forall|y: nat| t.contains(y) implies b.contains(y) by {
            assert(y < s@.len() && s@[y as int]);
        }
    }
    true
}

// A positive literal: (built-in, its argument roots, its input count).
pub fn literal_parts(arena: &ETermArena, root: usize) -> (out: (bool, Vec<usize>, usize))
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out.0 == ckc_spec::model::builtin(arena@[root as int]),
        out.0 ==> {
            &&& arena@[root as int] is Comp
            &&& crate::k2_engine::roots_valid(arena.nodes@, out.1@)
            &&& arena@[root as int]->Comp_1 == crate::k2_engine::root_terms(arena.nodes@, out.1@)
            &&& out.2 <= out.1@.len()
            &&& ckc_spec::model::builtin_inputs(arena@[root as int])
                == crate::k2_engine::root_terms(arena.nodes@, out.1@).subrange(0, out.2 as int)
            &&& ckc_spec::model::builtin_outputs(arena@[root as int])
                == crate::k2_engine::root_terms(arena.nodes@, out.1@).subrange(
                out.2 as int,
                out.1@.len() as int,
            )
        },
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            proof {
                assert(arena@[root as int] == Term::Comp(name@, args));
            }
            if !crate::compare_impl::is_builtin_exec(name, child_roots.len()) {
                return (false, Vec::new(), 0);
            }
            let kids = child_roots.clone();
            let arith = crate::compare_impl::is_arith_exec(name, child_roots.len());
            let k_in = if arith {
                4
            } else {
                kids.len()
            };
            proof {
                let ts = crate::k2_engine::root_terms(arena.nodes@, kids@);
                assert(ts =~= args);
                if arith {
                    assert(args.len() == 6);
                    assert(args.take(4) =~= ts.subrange(0, 4));
                    assert(args.skip(4) =~= ts.subrange(4, 6));
                } else {
                    assert(args =~= ts.subrange(0, ts.len() as int));
                    assert(Seq::<Term>::empty() =~= ts.subrange(ts.len() as int, ts.len() as int));
                }
            }
            (true, kids, k_in)
        },
        _ => (false, Vec::new(), 0),
    }
}

// One positive literal under the bound set (cur, cb): lit_safe, then the new bound set.
pub fn pos_step(
    arena: &ETermArena,
    root: usize,
    cur: &Vec<bool>,
    Ghost(cb): Ghost<Set<nat>>,
    n: usize,
) -> (out: Option<Vec<bool>>)
    requires
        crate::k2_term::root_ok(arena, root),
        cur@.len() == n,
        set_of(cur@, cb),
        forall|x: nat| #[trigger] ckc_spec::model::vars(arena@[root as int]).contains(x) ==> x < n,
    ensures
        out is Some == ckc_spec::model::lit_safe(arena@[root as int], cb),
        out matches Some(next) ==> next@.len() == n && set_of(
            next@,
            cb.union(ckc_spec::model::binds(arena@[root as int])),
        ),
{
    let ghost l = arena@[root as int];
    let (builtin, kids, k_in) = literal_parts(arena, root);
    if !builtin {
        let fl = term_flags(arena, root, n);
        return Some(union_flags(cur, &fl, Ghost(cb), Ghost(ckc_spec::model::vars(l))));
    }
    let ghost ts = crate::k2_engine::root_terms(arena.nodes@, kids@);
    proof {
        assert forall|x: nat| #[trigger]
            ckc_spec::model::vars_all(ts.subrange(0, k_in as int)).contains(x) implies x < n by {
            vars_all_sub(ts, 0, k_in as int, x);
            vars_comp(l, x);
        }
        assert forall|x: nat| #[trigger]
            ckc_spec::model::vars_all(ts.subrange(k_in as int, ts.len() as int)).contains(
                x,
            ) implies x < n by {
            vars_all_sub(ts, k_in as int, ts.len() as int, x);
            vars_comp(l, x);
        }
    }
    let ins = range_flags(arena, &kids, 0, k_in, n);
    if !subset_flags(
        &ins,
        cur,
        Ghost(ckc_spec::model::vars_all(ts.subrange(0, k_in as int))),
        Ghost(cb),
    ) {
        return None;
    }
    let outs = range_flags(arena, &kids, k_in, kids.len(), n);
    Some(
        union_flags(
            cur,
            &outs,
            Ghost(cb),
            Ghost(ckc_spec::model::vars_all(ts.subrange(k_in as int, ts.len() as int))),
        ),
    )
}

fn conj_safe_exec(
    arena: &ETermArena,
    roots: &Vec<usize>,
    bv: &Vec<bool>,
    Ghost(b): Ghost<Set<nat>>,
    n: usize,
) -> (out: bool)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::roots_valid(arena.nodes@, roots@),
        bv@.len() == n,
        set_of(bv@, b),
        forall|x: nat| #[trigger]
            ckc_spec::model::vars_all(crate::k2_engine::root_terms(arena.nodes@, roots@)).contains(
                x,
            ) ==> x < n,
    ensures
        out == ckc_spec::model::conj_safe(crate::k2_engine::root_terms(arena.nodes@, roots@), b),
{
    let ghost gs = crate::k2_engine::root_terms(arena.nodes@, roots@);
    let mut cur = bv.clone();
    let ghost mut cb = b;
    let mut k = 0usize;
    proof {
        assert(gs.skip(0) =~= gs);
    }
    while k < roots.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::k2_engine::roots_valid(arena.nodes@, roots@),
            gs == crate::k2_engine::root_terms(arena.nodes@, roots@),
            forall|x: nat| #[trigger] ckc_spec::model::vars_all(gs).contains(x) ==> x < n,
            k <= roots@.len(),
            cur@.len() == n,
            set_of(cur@, cb),
            ckc_spec::model::conj_safe(gs, b) == ckc_spec::model::conj_safe(gs.skip(k as int), cb),
        decreases roots.len() - k,
    {
        let root = roots[k];
        proof {
            assert(gs.skip(k as int)[0] == gs[k as int]);
            assert(gs.skip(k as int).drop_first() =~= gs.skip(k as int + 1));
            assert(gs[k as int] == arena@[root as int]);
            assert forall|x: nat| #[trigger]
                ckc_spec::model::vars(arena@[root as int]).contains(x) implies x < n by {
                vars_occurs(arena@[root as int], x);
                vars_all_any(gs, x);
                assert(ckc_spec::engine::occurs(x, gs[k as int]));
            }
        }
        match pos_step(arena, root, &cur, Ghost(cb), n) {
            None => return false,
            Some(next) => {
                proof {
                    cb = cb.union(ckc_spec::model::binds(arena@[root as int]));
                }
                cur = next;
            },
        }
        k += 1;
    }
    proof {
        assert(gs.skip(k as int).len() == 0);
    }
    true
}

// x occurs in body item `item` (its literal or any goal of its negation).
pub fn occ_item(arena: &ETermArena, item: &EBodyItem, x: usize) -> (out: bool)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::body_item_valid(arena.nodes@, item),
    ensures
        out == ckc_spec::model::item_vars(
            crate::k2_engine::body_item_view(arena.nodes@, item),
        ).contains(x as nat),
{
    match item {
        EBodyItem::Pos { root } => {
            proof {
                vars_occurs(arena@[*root as int], x as nat);
            }
            crate::k2_engine::occurs_root(arena, x, *root)
        },
        EBodyItem::Naf { roots } => {
            proof {
                let ts = crate::k2_engine::root_terms(arena.nodes@, roots@);
                assert(ts.subrange(0, ts.len() as int) =~= ts);
            }
            occ_range(arena, x, roots, 0, roots.len())
        },
    }
}

pub fn outside_exec(arena: &ETermArena, clause: &EClause, i: usize, x: usize) -> (out: bool)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::clause_valid(arena.nodes@, clause),
        i < clause.body@.len(),
    ensures
        out == ckc_spec::model::outside(
            crate::k2_engine::clause_view(arena.nodes@, clause),
            i as int,
            x as nat,
        ),
{
    let ghost c = crate::k2_engine::clause_view(arena.nodes@, clause);
    proof {
        vars_occurs(c.head, x as nat);
    }
    if crate::k2_engine::occurs_root(arena, x, clause.head) {
        return true;
    }
    let mut j = 0usize;
    while j < clause.body.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::k2_engine::clause_valid(arena.nodes@, clause),
            c == crate::k2_engine::clause_view(arena.nodes@, clause),
            !ckc_spec::model::vars(c.head).contains(x as nat),
            i < clause.body@.len(),
            j <= clause.body@.len(),
            forall|q: int|
                0 <= q < j && q != i ==> !#[trigger] ckc_spec::model::item_vars(c.body[q]).contains(
                    x as nat,
                ),
        decreases clause.body.len() - j,
    {
        proof {
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &clause.body@[j as int]));
            assert(c.body[j as int] == crate::k2_engine::body_item_view(
                arena.nodes@,
                &clause.body@[j as int],
            ));
        }
        if j != i && occ_item(arena, &clause.body[j], x) {
            return true;
        }
        j += 1;
    }
    false
}

// binders(items) ∋ x ⟺ some item binds x.
pub proof fn binders_any(items: Seq<BodyItem>, x: nat)
    ensures
        ckc_spec::model::binders(items).contains(x) == exists|q: int|
            0 <= q < items.len() && #[trigger] ckc_spec::model::item_binds(items[q]).contains(x),
    decreases items.len(),
{
    if items.len() > 0 {
        let rest = items.drop_first();
        binders_any(rest, x);
        if ckc_spec::model::binders(items).contains(x) {
            if !ckc_spec::model::item_binds(items[0]).contains(x) {
                let q = choose|q: int|
                    0 <= q < rest.len() && #[trigger] ckc_spec::model::item_binds(rest[q]).contains(
                        x,
                    );
                assert(rest[q] == items[q + 1]);
            }
        } else {
            assert forall|q: int|
                0 <= q < items.len() implies !#[trigger] ckc_spec::model::item_binds(
                items[q],
            ).contains(x) by {
                if q > 0 {
                    assert(rest[q - 1] == items[q]);
                }
            }
        }
    }
}

// A bound ≥ n above every variable of the terms at roots (None = a key at usize::MAX).
fn roots_bound(arena: &ETermArena, roots: &Vec<usize>, n: usize) -> (out: Option<usize>)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::roots_valid(arena.nodes@, roots@),
    ensures
        out matches Some(m) ==> m >= n && forall|x: nat| #[trigger]
            ckc_spec::model::vars_all(crate::k2_engine::root_terms(arena.nodes@, roots@)).contains(
                x,
            ) ==> x < m,
{
    let ghost ts = crate::k2_engine::root_terms(arena.nodes@, roots@);
    let mut m = n;
    let mut k = 0usize;
    while k < roots.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::k2_engine::roots_valid(arena.nodes@, roots@),
            ts == crate::k2_engine::root_terms(arena.nodes@, roots@),
            k <= roots@.len(),
            m >= n,
            forall|q: int, x: nat|
                0 <= q < k && #[trigger] ckc_spec::engine::occurs(x, ts[q]) ==> x < m,
        decreases roots.len() - k,
    {
        let ghost m0 = m;
        m =
        match bump(m, crate::k2_engine::max_var_root(arena, roots[k])) {
            Some(v) => v,
            None => return None,
        };
        proof {
            assert(ts[k as int] == arena@[roots@[k as int] as int]);
            assert forall|q: int, x: nat|
                0 <= q < k + 1 && #[trigger] ckc_spec::engine::occurs(x, ts[q]) implies x < m by {
                if q == k && x >= m {
                    crate::k2_engine::occurs_bound(ts[q], x);
                }
            }
        }
        k += 1;
    }
    proof {
        assert forall|x: nat| #[trigger] ckc_spec::model::vars_all(ts).contains(x) implies x
            < m by {
            vars_all_any(ts, x);
            let q = choose|q: int|
                0 <= q < ts.len() && #[trigger] ckc_spec::engine::occurs(x, ts[q]);
        }
    }
    Some(m)
}

// Every variable of the clause is below the returned bound (None = a key at usize::MAX).
pub fn clause_bound(arena: &ETermArena, clause: &EClause) -> (out: Option<usize>)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::clause_valid(arena.nodes@, clause),
    ensures
        out matches Some(n) ==> vars_below(
            crate::k2_engine::clause_view(arena.nodes@, clause),
            n as nat,
        ),
{
    let ghost c = crate::k2_engine::clause_view(arena.nodes@, clause);
    let mut n = match bump(0, crate::k2_engine::max_var_root(arena, clause.head)) {
        Some(n) => n,
        None => return None,
    };
    proof {
        assert forall|x: nat| #[trigger] ckc_spec::model::vars(c.head).contains(x) implies x
            < n by {
            vars_occurs(c.head, x);
            if x >= n {
                crate::k2_engine::occurs_bound(c.head, x);
            }
        }
    }
    let mut j = 0usize;
    while j < clause.body.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::k2_engine::clause_valid(arena.nodes@, clause),
            c == crate::k2_engine::clause_view(arena.nodes@, clause),
            j <= clause.body@.len(),
            forall|x: nat| #[trigger] ckc_spec::model::vars(c.head).contains(x) ==> x < n,
            forall|q: int, x: nat|
                0 <= q < j && #[trigger] ckc_spec::model::item_vars(c.body[q]).contains(x) ==> x
                    < n,
        decreases clause.body.len() - j,
    {
        let ghost n0 = n;
        proof {
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &clause.body@[j as int]));
            assert(c.body[j as int] == crate::k2_engine::body_item_view(
                arena.nodes@,
                &clause.body@[j as int],
            ));
        }
        match &clause.body[j] {
            EBodyItem::Pos { root } => {
                n =
                match bump(n, crate::k2_engine::max_var_root(arena, *root)) {
                    Some(m) => m,
                    None => return None,
                };
                proof {
                    assert forall|x: nat| #[trigger]
                        ckc_spec::model::vars(arena@[*root as int]).contains(x) implies x < n by {
                        vars_occurs(arena@[*root as int], x);
                        if x >= n {
                            crate::k2_engine::occurs_bound(arena@[*root as int], x);
                        }
                    }
                }
            },
            EBodyItem::Naf { roots } => {
                n =
                match roots_bound(arena, roots, n) {
                    Some(m) => m,
                    None => return None,
                };
            },
        }
        proof {
            assert forall|q: int, x: nat|
                0 <= q < j + 1 && #[trigger] ckc_spec::model::item_vars(c.body[q]).contains(
                    x,
                ) implies x < n by {
                if q < j {
                    assert(x < n0);
                }
            }
        }
        j += 1;
    }
    Some(n)
}

fn bump(n: usize, m: Option<usize>) -> (out: Option<usize>)
    ensures
        out matches Some(v) ==> v >= n && v as nat >= crate::k2_engine::max_var_count(m),
{
    match m {
        None => Some(n),
        Some(k) => if k == usize::MAX {
            None
        } else if k + 1 > n {
            Some(k + 1)
        } else {
            Some(n)
        },
    }
}

// Contract mc D3: `safe_clause` (None = a variable key at usize::MAX, outside every bound).
pub fn safe_clause_exec(arena: &ETermArena, clause: &EClause) -> (out: Option<bool>)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::clause_valid(arena.nodes@, clause),
    ensures
        out matches Some(v) ==> v == ckc_spec::model::safe_clause(
            crate::k2_engine::clause_view(arena.nodes@, clause),
        ),
{
    let ghost c = crate::k2_engine::clause_view(arena.nodes@, clause);
    let n = match clause_bound(arena, clause) {
        Some(n) => n,
        None => return None,
    };
    let mut bv = none_flags(n);
    let ghost mut b: Set<nat> = Set::empty();
    let mut binders = none_flags(n);
    let ghost mut bs: Set<nat> = Set::empty();
    let mut j = 0usize;
    while j < clause.body.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::k2_engine::clause_valid(arena.nodes@, clause),
            c == crate::k2_engine::clause_view(arena.nodes@, clause),
            vars_below(c, n as nat),
            j <= clause.body@.len(),
            bv@.len() == n,
            set_of(bv@, b),
            binders@.len() == n,
            set_of(binders@, bs),
            forall|x: nat| #[trigger]
                bs.contains(x) == exists|q: int|
                    0 <= q < j && #[trigger] ckc_spec::model::item_binds(c.body[q]).contains(x),
            ckc_spec::model::items_safe(c, 0, Set::empty()) == ckc_spec::model::items_safe(
                c,
                j as nat,
                b,
            ),
        decreases clause.body.len() - j,
    {
        proof {
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &clause.body@[j as int]));
            assert(c.body[j as int] == crate::k2_engine::body_item_view(
                arena.nodes@,
                &clause.body@[j as int],
            ));
        }
        match &clause.body[j] {
            EBodyItem::Pos { root } => {
                let root = *root;
                proof {
                    assert(c.body[j as int] == BodyItem::Pos(arena@[root as int]));
                    assert forall|x: nat| #[trigger]
                        ckc_spec::model::vars(arena@[root as int]).contains(x) implies x < n by {
                        assert(ckc_spec::model::item_vars(c.body[j as int]).contains(x));
                    }
                }
                let next = match pos_step(arena, root, &bv, Ghost(b), n) {
                    None => return Some(false),
                    Some(next) => next,
                };
                let ghost l = arena@[root as int];
                let ghost lb = ckc_spec::model::binds(l);
                proof {
                    assert(ckc_spec::model::item_binds(c.body[j as int]) == lb);
                    assert forall|x: nat| #[trigger] lb.contains(x) implies x < n by {
                        assert(next@.len() == n);
                        assert(b.union(lb).contains(x));
                    }
                }
                let lflags = pos_binds(arena, root, n);
                binders = union_flags(&binders, &lflags, Ghost(bs), Ghost(lb));
                proof {
                    let bs0 = bs;
                    bs = bs.union(lb);
                    assert forall|x: nat| #[trigger]
                        bs.contains(x) == exists|q: int|
                            0 <= q < j + 1 && #[trigger] ckc_spec::model::item_binds(
                                c.body[q],
                            ).contains(x) by {
                        if lb.contains(x) {
                            assert(ckc_spec::model::item_binds(c.body[j as int]).contains(x));
                        }
                    }
                    b = b.union(lb);
                }
                bv = next;
            },
            EBodyItem::Naf { roots } => {
                let ghost gs = crate::k2_engine::root_terms(arena.nodes@, roots@);
                proof {
                    assert(c.body[j as int] == BodyItem::Naf(gs));
                    assert forall|x: nat| #[trigger]
                        ckc_spec::model::vars_all(gs).contains(x) implies x < n by {
                        assert(ckc_spec::model::item_vars(c.body[j as int]).contains(x));
                    }
                    assert(gs.subrange(0, gs.len() as int) =~= gs);
                }
                let gflags = range_flags(arena, roots, 0, roots.len(), n);
                let mut x = 0usize;
                while x < n
                    invariant
                        crate::k2_term::arena_ok(arena),
                        crate::k2_engine::clause_valid(arena.nodes@, clause),
                        c == crate::k2_engine::clause_view(arena.nodes@, clause),
                        j < clause.body@.len(),
                        c.body[j as int] == BodyItem::Naf(gs),
                        ckc_spec::model::items_safe(c, 0, Set::empty())
                            == ckc_spec::model::items_safe(c, j as nat, b),
                        gflags@.len() == n,
                        set_of(gflags@, ckc_spec::model::vars_all(gs)),
                        bv@.len() == n,
                        set_of(bv@, b),
                        x <= n,
                        forall|y: nat|
                            y < x && #[trigger] ckc_spec::model::vars_all(gs).contains(y)
                                && ckc_spec::model::outside(c, j as int, y) ==> b.contains(y),
                    decreases n - x,
                {
                    if gflags[x] && !bv[x] && outside_exec(arena, clause, j, x) {
                        proof {
                            assert(ckc_spec::model::vars_all(gs).contains(x as nat));
                            assert(!b.contains(x as nat));
                        }
                        return Some(false);
                    }
                    x += 1;
                }
                proof {
                    assert forall|y: nat| #[trigger]
                        ckc_spec::model::vars_all(gs).contains(y) && ckc_spec::model::outside(
                            c,
                            j as int,
                            y,
                        ) implies b.contains(y) by {
                        assert(y < n);
                    }
                }
                if !conj_safe_exec(arena, roots, &bv, Ghost(b), n) {
                    return Some(false);
                }
                proof {
                    assert(ckc_spec::model::item_binds(c.body[j as int]) == Set::<nat>::empty());
                    assert forall|x: nat| #[trigger]
                        bs.contains(x) == exists|q: int|
                            0 <= q < j + 1 && #[trigger] ckc_spec::model::item_binds(
                                c.body[q],
                            ).contains(x) by {
                        if exists|q: int|
                            0 <= q < j + 1 && #[trigger] ckc_spec::model::item_binds(
                                c.body[q],
                            ).contains(x) {
                            let q = choose|q: int|
                                0 <= q < j + 1 && #[trigger] ckc_spec::model::item_binds(
                                    c.body[q],
                                ).contains(x);
                            assert(q != j);
                        }
                    }
                }
            },
        }
        j += 1;
    }
    proof {
        assert forall|x: nat| #[trigger]
            bs.contains(x) == ckc_spec::model::binders(c.body).contains(x) by {
            binders_any(c.body, x);
        }
        assert forall|x: nat| #[trigger] ckc_spec::model::vars(c.head).contains(x) implies x
            < n by {}
    }
    let hflags = term_flags(arena, clause.head, n);
    let head_ok = subset_flags(&hflags, &binders, Ghost(ckc_spec::model::vars(c.head)), Ghost(bs));
    proof {
        assert(bs =~= ckc_spec::model::binders(c.body));
    }
    Some(head_ok)
}

// The variables a positive literal binds, as flags below n.
pub fn pos_binds(arena: &ETermArena, root: usize, n: usize) -> (out: Vec<bool>)
    requires
        crate::k2_term::root_ok(arena, root),
        forall|x: nat| #[trigger] ckc_spec::model::vars(arena@[root as int]).contains(x) ==> x < n,
    ensures
        out@.len() == n,
        set_of(out@, ckc_spec::model::binds(arena@[root as int])),
{
    let ghost l = arena@[root as int];
    let (builtin, kids, k_in) = literal_parts(arena, root);
    if !builtin {
        return term_flags(arena, root, n);
    }
    let ghost ts = crate::k2_engine::root_terms(arena.nodes@, kids@);
    proof {
        assert forall|x: nat| #[trigger]
            ckc_spec::model::vars_all(ts.subrange(k_in as int, ts.len() as int)).contains(
                x,
            ) implies x < n by {
            vars_all_sub(ts, k_in as int, ts.len() as int, x);
            vars_comp(l, x);
        }
    }
    range_flags(arena, &kids, k_in, kids.len(), n)
}

} // verus!
