#[cfg(verus_keep_ghost)]
use crate::m6_default::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::arith as a;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::arg;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

// Contract ws D7 (cell invariance), the setup: a fragment variant and two
// tuples of one cell give the lockstep's context. This part: the chain of a
// tuple and its definedness.
proof fn zero_is_int(x: Term)
    requires
        zero_num(x),
    ensures
        x == Term::Int(0),
{
    crate::m6_cover::number_unique(x, Term::Int(0));
}

// Two tuples of one cell: the operand pairs read alike, zeros together (0
// is a boundary of every operand).
pub proof fn cell_rel(g: spec::Group, sel: ws::Sel, t: Seq<Term>, u: Seq<Term>)
    requires
        ws::chain_shape(sel),
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int|
            0 <= i < sel.ops.len() ==> ws::same_cell(
                ws::boundaries(g, sel, i),
                #[trigger] t[i],
                u[i],
            ),
    ensures
        rel_pairs(ws::op_pairs(sel, t), ws::op_pairs(sel, u)),
{
    let pt = ws::op_pairs(sel, t);
    let pu = ws::op_pairs(sel, u);
    op_pairs_index(sel, t);
    op_pairs_index(sel, u);
    let n = sel.ops.len() as int;
    assert forall|i: int| 0 <= i < n implies a::number(#[trigger] t[i]) is Some && a::number(
        u[i],
    ) is Some && (zero_num(u[i]) ==> zero_num(t[i])) by {
        let bs = ws::boundaries(g, sel, i);
        assert(ws::same_cell(bs, t[i], u[i]));
        crate::m6_cover::boundaries_props(g, sel, i);
        let k = choose|k: int| 0 <= k < bs.len() && bs[k] == Term::Int(0);
        assert((t[i] == bs[k]) == (u[i] == bs[k]));
        if zero_num(u[i]) {
            zero_is_int(u[i]);
        }
    }
    assert forall|j: int| 0 <= j < pt.len() implies (#[trigger] pt[j]).0 == pu[j].0
        && pt[j].0 is Var && rel_val(pt[j].1, pu[j].1) by {
        let i = j / 2;
        assert(0 <= i < n);
        assert(sel.ops[i].s is Var && sel.ops[i].n is Var);
        if j % 2 == 0 {
            assert(j == 2 * i);
        } else {
            assert(j == 2 * i + 1);
        }
    }
}

// Two tuples of one cell: the chain derives at both or at neither.
pub proof fn cell_defined(g: spec::Group, sel: ws::Sel, t: Seq<Term>, u: Seq<Term>)
    requires
        ws::chain_shape(sel),
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int|
            0 <= i < sel.ops.len() ==> ws::same_cell(
                ws::boundaries(g, sel, i),
                #[trigger] t[i],
                u[i],
            ),
    ensures
        (ws::tuple_pairs(g, sel, t) is Some) == (ws::tuple_pairs(g, sel, u) is Some),
{
    cell_rel(g, sel, t, u);
    assert forall|i: int| 0 <= i < sel.ops.len() implies ws::same_cell(
        ws::boundaries(g, sel, i),
        #[trigger] u[i],
        t[i],
    ) by {
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    cell_rel(g, sel, u, t);
    assert forall|i: int| 0 <= i < t.len() implies a::number(#[trigger] t[i]) is Some by {
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    assert forall|i: int| 0 <= i < u.len() implies a::number(#[trigger] u[i]) is Some by {
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    base_keys(sel, t);
    base_keys(sel, u);
    assert(sel.chain.skip(0) =~= sel.chain);
    if spec::chain_pairs(sel.chain, ws::op_pairs(sel, t)) is Some {
        chain_step(sel, 0, ws::op_pairs(sel, t), ws::op_pairs(sel, u));
    }
    if spec::chain_pairs(sel.chain, ws::op_pairs(sel, u)) is Some {
        chain_step(sel, 0, ws::op_pairs(sel, u), ws::op_pairs(sel, t));
    }
}

// An input pair of node k binds alike under p and under any extension of p
// that keeps p's bindings.
proof fn input_kept(
    sel: ws::Sel,
    k: int,
    s: Term,
    n: Term,
    p: Seq<(Term, Term)>,
    v: Seq<(Term, Term)>,
)
    requires
        ws::chain_shape(sel),
        0 <= k < sel.chain.len(),
        ws::chain_input(sel, k, s, n),
        keys_ok(sel, k, p),
        vals_ground(p),
        vals_ground(v),
        forall|key: Term| #[trigger]
            spec::lookup(p, key) is Some ==> spec::lookup(v, key) == spec::lookup(p, key),
    ensures
        spec::bind_all(s, v) == spec::bind_all(s, p),
        spec::bind_all(n, v) == spec::bind_all(n, p),
{
    if exists|i: int|
        0 <= i < sel.ops.len() && s == (#[trigger] sel.ops[i]).s && n == sel.ops[i].n {
        let i = choose|i: int|
            0 <= i < sel.ops.len() && s == (#[trigger] sel.ops[i]).s && n == sel.ops[i].n;
        assert(s is Var && n is Var);
        assert(spec::lookup(p, s) is Some && spec::lookup(p, n) is Some);
        bind_var(s->Var_0, p);
        bind_var(s->Var_0, v);
        bind_var(n->Var_0, p);
        bind_var(n->Var_0, v);
    } else if s == ws::exactly() && a::number(n) is Some {
        crate::k3_sound::log::number_ground(n);
        assert(ground(ws::exactly()));
        bind_ground(s, p);
        bind_ground(s, v);
        bind_ground(n, p);
        bind_ground(n, v);
    } else {
        let m = choose|m: int|
            0 <= m < k && s == arg(#[trigger] sel.chain[m], 4) && n == arg(sel.chain[m], 5);
        outputs_index(sel);
        assert(ws::outputs(sel)[2 * m] == s && ws::outputs(sel)[2 * m + 1] == n);
        assert(s is Var && n is Var);
        assert(spec::lookup(p, s) is Some && spec::lookup(p, n) is Some);
        bind_var(s->Var_0, p);
        bind_var(s->Var_0, v);
        bind_var(n->Var_0, p);
        bind_var(n->Var_0, v);
    }
}

// The chain's pairs from node k on: they keep p's bindings, end with every
// key bound, and each node from k computes the outputs it pushed.
pub proof fn chain_vals(sel: ws::Sel, k: nat, p: Seq<(Term, Term)>)
    requires
        ws::chain_shape(sel),
        k <= sel.chain.len(),
        keys_ok(sel, k as int, p),
        rel_pairs(p, p),
        spec::chain_pairs(sel.chain.skip(k as int), p) is Some,
    ensures
        ({
            let v = spec::chain_pairs(sel.chain.skip(k as int), p)->Some_0;
            &&& rel_pairs(v, v)
            &&& keys_ok(sel, sel.chain.len() as int, v)
            &&& forall|key: Term| #[trigger]
                spec::lookup(p, key) is Some ==> spec::lookup(v, key) == spec::lookup(p, key)
            &&& forall|key: Term|
                !keys(sel).contains(key) && #[trigger] spec::lookup(p, key) is None
                    ==> spec::lookup(v, key) is None
            &&& forall|m: int|
                k <= m < sel.chain.len() ==> #[trigger] a::arith_result(
                    spec::bind_all(sel.chain[m], v),
                ) == Some(
                    (
                        spec::lookup(v, arg(sel.chain[m], 4))->0,
                        spec::lookup(v, arg(sel.chain[m], 5))->0,
                    ),
                )
        }),
    decreases sel.chain.len() - k,
{
    reveal_with_fuel(spec::chain_pairs, 1);
    if k == sel.chain.len() {
        assert(sel.chain.skip(k as int) =~= Seq::<Term>::empty());
    } else {
        let goals = sel.chain.skip(k as int);
        let g = sel.chain[k as int];
        assert(goals[0] == g);
        assert(goals.drop_first() =~= sel.chain.skip(k as int + 1));
        assert(ws::is_arith_lit(g));
        let name = g->Comp_0;
        let xs = g->Comp_1;
        assert(xs.len() == 6) by {
            reveal(a::is_arith);
        }
        input_binds(sel, k as int, arg(g, 0), arg(g, 1), p, p);
        input_binds(sel, k as int, arg(g, 2), arg(g, 3), p, p);
        bind_comp(name, xs, p);
        let bt = spec::bind_all(g, p);
        let at = bt->Comp_1;
        let xt = at[1];
        let yt = at[3];
        assert(at =~= seq![ws::exactly(), xt, ws::exactly(), yt] + seq![at[4], at[5]]);
        node_points(name, xt, yt, xt, yt, seq![at[4], at[5]], seq![at[4], at[5]]);
        let (sv, nv) = a::arith_result(bt)->Some_0;
        let o4 = arg(g, 4);
        let o5 = arg(g, 5);
        let p2 = p.push((o4, sv)).push((o5, nv));
        assert(spec::chain_pairs(goals, p) == spec::chain_pairs(sel.chain.skip(k as int + 1), p2));
        keys_index(sel);
        let nops = sel.ops.len() as int;
        assert(keys(sel)[2 * nops + 2 * (k as int)] == o4 && keys(sel)[2 * nops + 2 * (k as int)
            + 1] == o5);
        assert(o4 is Var && o5 is Var) by {
            outputs_index(sel);
            assert(ws::outputs(sel)[2 * (k as int)] == o4);
            assert(ws::outputs(sel)[2 * (k as int) + 1] == o5);
        }
        assert(o4 != o5);
        assert(rel_pairs(p2, p2)) by {
            assert forall|i: int| 0 <= i < p2.len() implies (#[trigger] p2[i]).0 == p2[i].0
                && p2[i].0 is Var && rel_val(p2[i].1, p2[i].1) by {
                if i < p.len() {
                    assert(p2[i] == p[i]);
                }
            }
        }
        assert(keys_ok(sel, k as int + 1, p2)) by {
            assert forall|i: int| 0 <= i < sel.ops.len() implies spec::lookup(
                p2,
                (#[trigger] sel.ops[i]).s,
            ) == Some(ws::exactly()) && (spec::lookup(p2, sel.ops[i].n) matches Some(v)
                && a::number(v) is Some) by {
                lookup_push(p, o4, sv, sel.ops[i].s);
                lookup_push(p.push((o4, sv)), o5, nv, sel.ops[i].s);
                lookup_push(p, o4, sv, sel.ops[i].n);
                lookup_push(p.push((o4, sv)), o5, nv, sel.ops[i].n);
            }
            assert forall|m: int| 0 <= m < k + 1 implies spec::lookup(
                p2,
                arg(#[trigger] sel.chain[m], 4),
            ) == Some(ws::exactly()) && (spec::lookup(p2, arg(sel.chain[m], 5)) matches Some(v)
                && a::number(v) is Some) by {
                lookup_push(p, o4, sv, arg(sel.chain[m], 4));
                lookup_push(p.push((o4, sv)), o5, nv, arg(sel.chain[m], 4));
                lookup_push(p, o4, sv, arg(sel.chain[m], 5));
                lookup_push(p.push((o4, sv)), o5, nv, arg(sel.chain[m], 5));
                if m == k {
                    assert(arg(sel.chain[m], 5) != o4);
                }
            }
            assert forall|m: int| k + 1 <= m < sel.chain.len() implies spec::lookup(
                p2,
                arg(#[trigger] sel.chain[m], 4),
            ) is None && spec::lookup(p2, arg(sel.chain[m], 5)) is None by {
                let a4 = arg(sel.chain[m], 4);
                let a5 = arg(sel.chain[m], 5);
                assert(keys(sel)[2 * nops + 2 * m] == a4 && keys(sel)[2 * nops + 2 * m + 1] == a5);
                assert(a4 != o4 && a4 != o5 && a5 != o4 && a5 != o5);
                lookup_push(p, o4, sv, a4);
                lookup_push(p.push((o4, sv)), o5, nv, a4);
                lookup_push(p, o4, sv, a5);
                lookup_push(p.push((o4, sv)), o5, nv, a5);
            }
        }
        chain_vals(sel, k + 1, p2);
        let v = spec::chain_pairs(sel.chain.skip(k as int + 1), p2)->Some_0;
        // p's bindings: kept through the push
        assert forall|key: Term| #[trigger] spec::lookup(p, key) is Some implies spec::lookup(
            v,
            key,
        ) == spec::lookup(p, key) by {
            lookup_push(p, o4, sv, key);
            lookup_push(p.push((o4, sv)), o5, nv, key);
        }
        // a key outside the chain's keys stays unbound
        assert forall|key: Term|
            !keys(sel).contains(key) && #[trigger] spec::lookup(
                p,
                key,
            ) is None implies spec::lookup(v, key) is None by {
            assert(keys(sel)[2 * nops + 2 * (k as int)] == o4);
            assert(keys(sel)[2 * nops + 2 * (k as int) + 1] == o5);
            assert(key != o4 && key != o5);
            lookup_push(p, o4, sv, key);
            lookup_push(p.push((o4, sv)), o5, nv, key);
        }
        // node k: its outputs, and its inputs bound alike under v and p
        assert(spec::lookup(p, o4) is None && spec::lookup(p, o5) is None);
        lookup_push(p, o4, sv, o4);
        lookup_push(p.push((o4, sv)), o5, nv, o4);
        lookup_push(p, o4, sv, o5);
        lookup_push(p.push((o4, sv)), o5, nv, o5);
        assert(spec::lookup(p2, o4) == Some(sv) && spec::lookup(p2, o5) == Some(nv));
        rel_vals_ground(p, p);
        rel_vals_ground(v, v);
        input_kept(sel, k as int, arg(g, 0), arg(g, 1), p, v);
        input_kept(sel, k as int, arg(g, 2), arg(g, 3), p, v);
        bind_comp(name, xs, v);
        let bv = spec::bind_all(g, v);
        assert(g == Term::Comp(name, xs));
        assert forall|i: int| 0 <= i < 4 implies bv->Comp_1[i] == at[i] by {
            assert(arg(g, i) == xs[i]);
        }
        assert(bt == Term::Comp(name, at));
        assert(bv == Term::Comp(name, bv->Comp_1));
        assert(a::arith_result(bv) == a::arith_result(bt));
    }
}

} // verus!
