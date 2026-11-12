use crate::k2_term::{ENode, ENodeKind, ETermArena};
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

// Contract mc D6: valuations over arena terms. A substitution = per variable below its length
// the root of a ground value; `th` = its ghost map.
pub open spec fn sub_ok(nodes: Seq<ENode>, sub: Seq<Option<usize>>, th: Map<nat, Term>) -> bool {
    &&& forall|k: nat| #[trigger] th.contains_key(k) <==> (k < sub.len() && sub[k as int] is Some)
    &&& forall|k: nat| #[trigger]
        th.contains_key(k) ==> {
            &&& sub[k as int]->Some_0 < nodes.len()
            &&& nodes[sub[k as int]->Some_0 as int].term@ == th[k]
            &&& ckc_spec::term::ground(th[k])
        }
}

// t3 agrees with th on th's domain.
pub open spec fn extends(t3: Map<nat, Term>, th: Map<nat, Term>) -> bool {
    forall|k: nat| #[trigger] th.contains_key(k) ==> t3.contains_key(k) && t3[k] == th[k]
}

pub proof fn sub_ok_prefix(
    before: Seq<ENode>,
    after: Seq<ENode>,
    sub: Seq<Option<usize>>,
    th: Map<nat, Term>,
)
    requires
        before.is_prefix_of(after),
        sub_ok(before, sub, th),
    ensures
        sub_ok(after, sub, th),
{
    assert forall|k: nat| #[trigger] th.contains_key(k) implies {
        &&& sub[k as int]->Some_0 < after.len()
        &&& after[sub[k as int]->Some_0 as int].term@ == th[k]
        &&& ckc_spec::term::ground(th[k])
    } by {
        assert(after[sub[k as int]->Some_0 as int] == before[sub[k as int]->Some_0 as int]);
    }
}

pub proof fn apply_all_index(ts: Seq<Term>, th: Map<nat, Term>)
    ensures
        ckc_spec::model::apply_all(ts, th).len() == ts.len(),
        forall|q: int|
            0 <= q < ts.len() ==> #[trigger] ckc_spec::model::apply_all(ts, th)[q]
                == ckc_spec::model::apply(ts[q], th),
    decreases ts.len(),
{
    if ts.len() > 0 {
        apply_all_index(ts.drop_first(), th);
        assert forall|q: int| 0 <= q < ts.len() implies #[trigger] ckc_spec::model::apply_all(
            ts,
            th,
        )[q] == ckc_spec::model::apply(ts[q], th) by {
            if q > 0 {
                assert(ts.drop_first()[q - 1] == ts[q]);
            }
        }
    }
}

// x occurs in an argument ⇒ x occurs in the compound.
pub proof fn occurs_in_arg(f: Seq<u8>, args: Seq<Term>, q: int, x: nat)
    requires
        0 <= q < args.len(),
        ckc_spec::engine::occurs(x, args[q]),
    ensures
        ckc_spec::engine::occurs(x, Term::Comp(f, args)),
{
    crate::mc_safe::occurs_all_any(x, args);
}

// apply reads the substitution at t's variables alone.
pub proof fn apply_agree(t: Term, a: Map<nat, Term>, b: Map<nat, Term>)
    requires
        forall|x: nat| #[trigger]
            ckc_spec::engine::occurs(x, t) ==> (a.contains_key(x) == b.contains_key(x) && (
            a.contains_key(x) ==> a[x] == b[x])),
    ensures
        ckc_spec::model::apply(t, a) == ckc_spec::model::apply(t, b),
    decreases t,
{
    match t {
        Term::Var(k) => {
            assert(ckc_spec::engine::occurs(k, t));
        },
        Term::Comp(f, args) => {
            apply_all_index(args, a);
            apply_all_index(args, b);
            assert forall|q: int| 0 <= q < args.len() implies #[trigger] ckc_spec::model::apply(
                args[q],
                a,
            ) == ckc_spec::model::apply(args[q], b) by {
                assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, args[q]) implies (
                a.contains_key(x) == b.contains_key(x) && (a.contains_key(x) ==> a[x] == b[x])) by {
                    occurs_in_arg(f, args, q, x);
                }
                apply_agree(args[q], a, b);
            }
            assert(ckc_spec::model::apply_all(args, a) =~= ckc_spec::model::apply_all(args, b));
        },
        _ => {},
    }
}

pub proof fn ground_all_from(ts: Seq<Term>)
    requires
        forall|q: int| 0 <= q < ts.len() ==> #[trigger] ckc_spec::term::ground(ts[q]),
    ensures
        ckc_spec::term::ground_all(ts),
    decreases ts.len(),
{
    if ts.len() > 0 {
        let rest = ts.drop_first();
        assert forall|q: int| 0 <= q < rest.len() implies #[trigger] ckc_spec::term::ground(
            rest[q],
        ) by {
            assert(rest[q] == ts[q + 1]);
        }
        ground_all_from(rest);
        assert(ckc_spec::term::ground(ts[0]));
    }
}

pub proof fn apply_of_ground(t: Term, th: Map<nat, Term>)
    requires
        ckc_spec::term::ground(t),
    ensures
        ckc_spec::model::apply(t, th) == t,
    decreases t,
{
    if let Term::Comp(f, args) = t {
        apply_all_index(args, th);
        assert forall|q: int| 0 <= q < args.len() implies #[trigger] ckc_spec::model::apply(
            args[q],
            th,
        ) == args[q] by {
            crate::mc_patient::ground_all_at(args, q);
            apply_of_ground(args[q], th);
        }
        assert(ckc_spec::model::apply_all(args, th) =~= args);
    }
}

// Binding every variable of t to a ground value grounds it.
pub proof fn apply_ground(t: Term, th: Map<nat, Term>)
    requires
        forall|x: nat| #[trigger]
            ckc_spec::engine::occurs(x, t) ==> th.contains_key(x) && ckc_spec::term::ground(th[x]),
    ensures
        ckc_spec::term::ground(ckc_spec::model::apply(t, th)),
    decreases t,
{
    match t {
        Term::Var(k) => {
            assert(ckc_spec::engine::occurs(k, t));
        },
        Term::Comp(f, args) => {
            apply_all_index(args, th);
            assert forall|q: int| 0 <= q < args.len() implies #[trigger] ckc_spec::term::ground(
                ckc_spec::model::apply_all(args, th)[q],
            ) by {
                assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, args[q]) implies (
                th.contains_key(x) && ckc_spec::term::ground(th[x])) by {
                    occurs_in_arg(f, args, q, x);
                }
                apply_ground(args[q], th);
            }
            ground_all_from(ckc_spec::model::apply_all(args, th));
        },
        _ => {},
    }
}

// A ground instance binds every variable of the term.
pub proof fn ground_apply_binds(t: Term, th: Map<nat, Term>, x: nat)
    requires
        ckc_spec::term::ground(ckc_spec::model::apply(t, th)),
        ckc_spec::engine::occurs(x, t),
    ensures
        th.contains_key(x),
    decreases t,
{
    if let Term::Comp(f, args) = t {
        apply_all_index(args, th);
        crate::mc_safe::occurs_all_any(x, args);
        let q = choose|q: int|
            0 <= q < args.len() && #[trigger] ckc_spec::engine::occurs(x, args[q]);
        crate::mc_patient::ground_all_at(ckc_spec::model::apply_all(args, th), q);
        ground_apply_binds(args[q], th, x);
    }
}

// A compound keeps its name + arity under a valuation, so its built-in status.
pub proof fn builtin_apply_iff(t: Term, th: Map<nat, Term>)
    requires
        t is Comp,
    ensures
        ckc_spec::model::builtin(ckc_spec::model::apply(t, th)) == ckc_spec::model::builtin(t),
{
    apply_all_index(t->Comp_1, th);
}

// Matches pattern arena@[pat] against the ground arena@[fact], binding pattern variables `sub`
// leaves unbound. Every extension of th mapping the pattern onto the fact extends the result
// (none exists on failure). `sub` changes only on success.
pub fn match_term(
    arena: &ETermArena,
    pat: usize,
    fact: usize,
    sub: &mut Vec<Option<usize>>,
    Ghost(th): Ghost<Map<nat, Term>>,
) -> (out: (bool, Ghost<Map<nat, Term>>))
    requires
        crate::k2_term::root_ok(arena, pat),
        crate::k2_term::root_ok(arena, fact),
        ckc_spec::term::ground(arena@[fact as int]),
        sub_ok(arena.nodes@, old(sub)@, th),
        forall|x: nat| #[trigger]
            ckc_spec::engine::occurs(x, arena@[pat as int]) ==> x < old(sub)@.len(),
    ensures
        final(sub)@.len() == old(sub)@.len(),
        out.0 ==> {
            &&& sub_ok(arena.nodes@, final(sub)@, out.1@)
            &&& extends(out.1@, th)
            &&& forall|x: nat| #[trigger]
                out.1@.contains_key(x) <==> (th.contains_key(x) || ckc_spec::engine::occurs(
                    x,
                    arena@[pat as int],
                ))
            &&& ckc_spec::model::apply(arena@[pat as int], out.1@) == arena@[fact as int]
        },
        !out.0 ==> final(sub)@ == old(sub)@,
        forall|t3: Map<nat, Term>|
            extends(t3, th) && ckc_spec::model::apply(arena@[pat as int], t3) == arena@[fact as int]
                ==> out.0 && extends(t3, out.1@),
    decreases pat,
{
    let ghost p = arena@[pat as int];
    let ghost f = arena@[fact as int];
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, pat as int));
        assert(crate::k2_term::node_ok(arena.nodes@, fact as int));
    }
    match &arena.nodes[pat].kind {
        ENodeKind::Var { key, .. } => {
            let k = *key;
            proof {
                assert(p == Term::Var(k as nat));
                assert(ckc_spec::engine::occurs(k as nat, p));
            }
            match sub[k] {
                Some(r) => {
                    proof {
                        assert(th.contains_key(k as nat));
                        assert(crate::k2_term::root_ok(arena, r));
                    }
                    let same = crate::k2_sort::term_equal(arena, r, fact);
                    (same, Ghost(th))
                },
                None => {
                    proof {
                        assert(!th.contains_key(k as nat));
                    }
                    let ghost th2 = th.insert(k as nat, f);
                    sub.set(k, Some(fact));
                    proof {
                        assert forall|x: nat| #[trigger] th2.contains_key(x) implies {
                            &&& sub@[x as int]->Some_0 < arena.nodes@.len()
                            &&& arena.nodes@[sub@[x as int]->Some_0 as int].term@ == th2[x]
                            &&& ckc_spec::term::ground(th2[x])
                        } by {
                            if x != k {
                                assert(th.contains_key(x));
                            }
                        }
                        assert forall|x: nat| #[trigger]
                            th2.contains_key(x) <==> (x < sub@.len() && sub@[x as int] is Some) by {
                            if x != k as nat && x < sub@.len() {
                                assert(sub@[x as int] == old(sub)@[x as int]);
                            }
                        }
                        assert forall|x: nat| #[trigger]
                            th2.contains_key(x) <==> (th.contains_key(x)
                                || ckc_spec::engine::occurs(x, p)) by {}
                        assert forall|t3: Map<nat, Term>|
                            extends(t3, th) && ckc_spec::model::apply(p, t3) == f implies extends(
                            t3,
                            th2,
                        ) by {
                            assert(t3.contains_key(k as nat));
                        }
                    }
                    (true, Ghost(th2))
                },
            }
        },
        ENodeKind::Comp { name, child_roots, .. } => {
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            proof {
                assert(p == Term::Comp(name@, args));
            }
            let (g, fk) = match &arena.nodes[fact].kind {
                ENodeKind::Comp { name: g, child_roots: fk, .. } => (g, fk),
                _ => {
                    proof {
                        assert forall|t3: Map<nat, Term>|
                            extends(t3, th) && ckc_spec::model::apply(p, t3)
                                == f implies false by {}
                    }
                    return (false, Ghost(th));
                },
            };
            let ghost fargs = crate::k2_term::child_terms(arena.nodes@, fk@);
            proof {
                assert(f == Term::Comp(g@, fargs));
                crate::mc_patient::ground_all_at(fargs, 0);
            }
            if !(crate::k2_engine::vec_equal(name, g) && child_roots.len() == fk.len()) {
                proof {
                    assert forall|t3: Map<nat, Term>|
                        extends(t3, th) && ckc_spec::model::apply(p, t3) == f implies false by {
                        apply_all_index(args, t3);
                    }
                }
                return (false, Ghost(th));
            }
            let ghost sub0 = sub@;
            let mut work = sub.clone();
            let ghost mut cur = th;
            let mut q = 0usize;
            while q < child_roots.len()
                invariant
                    crate::k2_term::root_ok(arena, pat),
                    crate::k2_term::root_ok(arena, fact),
                    crate::k2_term::child_roots_before(child_roots@, pat as int),
                    crate::k2_term::child_roots_valid(arena.nodes@, child_roots@),
                    crate::k2_term::child_roots_valid(arena.nodes@, fk@),
                    args == crate::k2_term::child_terms(arena.nodes@, child_roots@),
                    fargs == crate::k2_term::child_terms(arena.nodes@, fk@),
                    p == arena@[pat as int],
                    f == arena@[fact as int],
                    p == Term::Comp(name@, args),
                    f == Term::Comp(name@, fargs),
                    ckc_spec::term::ground(f),
                    args.len() == fargs.len(),
                    q <= args.len(),
                    work@.len() == sub0.len(),
                    sub@ == sub0,
                    sub_ok(arena.nodes@, work@, cur),
                    extends(cur, th),
                    forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, p) ==> x < sub0.len(),
                    forall|x: nat| #[trigger]
                        cur.contains_key(x) <==> (th.contains_key(x) || exists|q2: int|
                            0 <= q2 < q && #[trigger] ckc_spec::engine::occurs(x, args[q2])),
                    forall|q2: int|
                        0 <= q2 < q ==> #[trigger] ckc_spec::model::apply(args[q2], cur)
                            == fargs[q2],
                    forall|t3: Map<nat, Term>|
                        extends(t3, th) && ckc_spec::model::apply(p, t3) == f ==> extends(t3, cur),
                decreases child_roots.len() - q,
            {
                proof {
                    assert(args[q as int] == arena@[child_roots@[q as int] as int]);
                    assert(fargs[q as int] == arena@[fk@[q as int] as int]);
                    crate::mc_patient::ground_all_at(fargs, q as int);
                    assert forall|x: nat| #[trigger]
                        ckc_spec::engine::occurs(x, args[q as int]) implies x < work@.len() by {
                        occurs_in_arg(name@, args, q as int, x);
                    }
                }
                let ghost before = cur;
                let (ok, next) = match_term(arena, child_roots[q], fk[q], &mut work, Ghost(cur));
                if !ok {
                    proof {
                        assert forall|t3: Map<nat, Term>|
                            extends(t3, th) && ckc_spec::model::apply(p, t3) == f implies false by {
                            apply_all_index(args, t3);
                            assert(ckc_spec::model::apply(args[q as int], t3) == fargs[q as int]);
                        }
                    }
                    return (false, Ghost(th));
                }
                proof {
                    let nx = next@;
                    assert forall|q2: int|
                        0 <= q2 < q + 1 implies #[trigger] ckc_spec::model::apply(args[q2], nx)
                        == fargs[q2] by {
                        if q2 < q {
                            assert forall|x: nat| #[trigger]
                                ckc_spec::engine::occurs(x, args[q2]) implies (before.contains_key(
                                x,
                            ) == nx.contains_key(x) && (before.contains_key(x) ==> before[x]
                                == nx[x])) by {
                                assert(before.contains_key(x));
                            }
                            apply_agree(args[q2], before, nx);
                        }
                    }
                    assert forall|x: nat| #[trigger]
                        nx.contains_key(x) <==> (th.contains_key(x) || exists|q2: int|
                            0 <= q2 < q + 1 && #[trigger] ckc_spec::engine::occurs(
                                x,
                                args[q2],
                            )) by {
                        if exists|q2: int|
                            0 <= q2 < q + 1 && #[trigger] ckc_spec::engine::occurs(x, args[q2]) {
                            let q2 = choose|q2: int|
                                0 <= q2 < q + 1 && #[trigger] ckc_spec::engine::occurs(x, args[q2]);
                            if q2 < q {
                                assert(before.contains_key(x));
                            }
                        }
                    }
                    assert forall|k: nat| #[trigger] th.contains_key(k) implies nx.contains_key(k)
                        && nx[k] == th[k] by {
                        assert(before.contains_key(k));
                    }
                    assert forall|t3: Map<nat, Term>|
                        extends(t3, th) && ckc_spec::model::apply(p, t3) == f implies extends(
                        t3,
                        nx,
                    ) by {
                        apply_all_index(args, t3);
                        assert(ckc_spec::model::apply(args[q as int], t3) == fargs[q as int]);
                    }
                    cur = nx;
                }
                q += 1;
            }
            proof {
                apply_all_index(args, cur);
                assert(ckc_spec::model::apply_all(args, cur) =~= fargs);
                assert forall|x: nat| #[trigger]
                    cur.contains_key(x) <==> (th.contains_key(x) || ckc_spec::engine::occurs(
                        x,
                        p,
                    )) by {
                    crate::mc_safe::occurs_all_any(x, args);
                }
            }
            *sub = work;
            (true, Ghost(cur))
        },
        _ => {
            proof {
                assert(ckc_spec::term::ground(p));
            }
            let same = crate::k2_sort::term_equal(arena, pat, fact);
            (same, Ghost(th))
        },
    }
}

// Builds apply(arena@[root], th) in the arena; every variable of the term is bound.
pub fn instantiate(
    arena: &mut ETermArena,
    root: usize,
    sub: &Vec<Option<usize>>,
    Ghost(th): Ghost<Map<nat, Term>>,
) -> (out: usize)
    requires
        crate::k2_term::arena_ok(old(arena)),
        root < old(arena).nodes@.len(),
        sub_ok(old(arena).nodes@, sub@, th),
        forall|x: nat| #[trigger]
            ckc_spec::engine::occurs(x, old(arena)@[root as int]) ==> th.contains_key(x),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out < final(arena).nodes@.len(),
        final(arena)@[out as int] == ckc_spec::model::apply(old(arena)@[root as int], th),
    decreases root,
{
    let ghost t = arena@[root as int];
    let ghost origin = arena.nodes@;
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
        assert(origin.is_prefix_of(origin)) by {
            assert(origin.take(origin.len() as int) =~= origin);
        }
    }
    if crate::k2_walk::ground_root(arena, root) {
        proof {
            apply_of_ground(t, th);
        }
        return root;
    }
    match &arena.nodes[root].kind {
        ENodeKind::Var { key, .. } => {
            let k = *key;
            proof {
                assert(t == Term::Var(k as nat));
                assert(ckc_spec::engine::occurs(k as nat, t));
                assert(th.contains_key(k as nat));
            }
            match sub[k] {
                Some(r) => r,
                None => root,
            }
        },
        ENodeKind::Comp { name, child_roots, .. } => {
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            proof {
                assert(t == Term::Comp(name@, args));
            }
            let name2 = name.clone();
            let kids = child_roots.clone();
            let mut built: Vec<usize> = Vec::new();
            let mut q = 0usize;
            while q < kids.len()
                invariant
                    crate::k2_term::arena_ok(arena),
                    origin.is_prefix_of(arena.nodes@),
                    root < origin.len(),
                    t == origin[root as int].term@,
                    t == Term::Comp(name2@, args),
                    crate::k2_term::child_roots_before(kids@, root as int),
                    crate::k2_term::child_roots_valid(origin, kids@),
                    args == crate::k2_term::child_terms(origin, kids@),
                    sub_ok(arena.nodes@, sub@, th),
                    forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, t) ==> th.contains_key(x),
                    q <= kids@.len(),
                    built@.len() == q,
                    forall|q2: int|
                        0 <= q2 < q ==> #[trigger] built@[q2] < arena.nodes@.len()
                            && arena@[built@[q2] as int] == ckc_spec::model::apply(args[q2], th),
                decreases kids.len() - q,
            {
                let ghost before = arena.nodes@;
                proof {
                    assert(origin[kids@[q as int] as int] == before[kids@[q as int] as int]);
                    assert forall|x: nat| #[trigger]
                        ckc_spec::engine::occurs(
                            x,
                            before[kids@[q as int] as int].term@,
                        ) implies th.contains_key(x) by {
                        occurs_in_arg(name2@, args, q as int, x);
                    }
                }
                let b = instantiate(arena, kids[q], sub, Ghost(th));
                proof {
                    crate::k2_load::prefix_chain(origin, before, arena.nodes@);
                    sub_ok_prefix(before, arena.nodes@, sub@, th);
                    assert forall|q2: int| 0 <= q2 < q implies #[trigger] built@[q2]
                        < arena.nodes@.len() && arena@[built@[q2] as int] == ckc_spec::model::apply(
                        args[q2],
                        th,
                    ) by {
                        assert(before[built@[q2] as int] == arena.nodes@[built@[q2] as int]);
                    }
                }
                built.push(b);
                q += 1;
            }
            let ghost before_push = arena.nodes@;
            proof {
                assert(args.len() >= 1);
            }
            let out = crate::k2_term::push_comp(arena, name2, built);
            proof {
                crate::k2_load::prefix_chain(origin, before_push, arena.nodes@);
                apply_all_index(args, th);
                assert(crate::k2_term::child_terms(before_push, built@)
                    =~= ckc_spec::model::apply_all(args, th));
            }
            out
        },
        _ => {
            proof {
                assert(ckc_spec::term::ground(t));
            }
            root
        },
    }
}

} // verus!
