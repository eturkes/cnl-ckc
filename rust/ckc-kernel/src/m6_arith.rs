#[cfg(verus_keep_ghost)]
use crate::k3_sound::naf::{lift_var, rename};
#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use crate::m6_lockstep::*;
#[cfg(verus_keep_ghost)]
use crate::m6_step::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::arith::arith_result;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit::{bind_all, lookup};
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::arg;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

// Contract ws D7 (cell invariance), the arithmetic step: node k's goal reads
// its chain node's inputs in each run, computes that run's node values and
// binds its output pair to them; the frame's instantiation takes the values.
// Input pair q of chain node kk in one run (chain values vv, slot values ss):
// an operand's count (comparison `exactly`, value = the slot's), an earlier
// node's output pair, or a literal.
pub open spec fn cin(
    e: ws::WalkEnv,
    vv: Seq<(Term, Term)>,
    ss: Seq<(nat, Term)>,
    kk: int,
    q: int,
) -> bool {
    let c = e.chain[kk];
    let s = arg(c, q);
    let n = arg(c, q + 1);
    if e.slots.contains(n) {
        &&& s is Var
        &&& n is Var
        &&& lookup(vv, s) == Some(ws::exactly())
        &&& lookup(vv, n) == th_val(ss, n->Var_0)
    } else if n is Var {
        &&& s is Var
        &&& exists|m: int|
            0 <= m < kk && s == arg(#[trigger] e.chain[m], 4) && n == arg(e.chain[m], 5)
    } else {
        ground(s) && ground(n)
    }
}

// Chain node kk in one run: an arithmetic goal over chain inputs whose values
// compute its output values.
pub open spec fn node_at(
    e: ws::WalkEnv,
    vv: Seq<(Term, Term)>,
    ss: Seq<(nat, Term)>,
    kk: int,
) -> bool {
    let c = e.chain[kk];
    &&& ws::is_arith_lit(c)
    &&& cin(e, vv, ss, kk, 0)
    &&& cin(e, vv, ss, kk, 2)
    &&& lookup(vv, arg(c, 4)) is Some
    &&& lookup(vv, arg(c, 5)) is Some
    &&& arith_result(bind_all(c, vv)) == Some((lookup(vv, arg(c, 4))->0, lookup(vv, arg(c, 5))->0))
}

// What the arithmetic steps read of the context: both runs' chains, the
// placeholders below the walk's offset and pairwise distinct.
pub open spec fn chain_ok(cx: Cx) -> bool {
    &&& crate::m6_default::vals_ground(cx.vt)
    &&& crate::m6_default::vals_ground(cx.vu)
    &&& forall|kk: int| 0 <= kk < cx.e.chain.len() ==> #[trigger] node_at(cx.e, cx.vt, cx.st, kk)
    &&& forall|kk: int| 0 <= kk < cx.e.chain.len() ==> #[trigger] node_at(cx.e, cx.vu, cx.su, kk)
    &&& forall|m: int|
        0 <= m < cx.e.chain.len() ==> #[trigger] cx.o4[m] < cx.e.off && cx.o5[m] < cx.e.off
    &&& forall|m1: int, m2: int|
        0 <= m1 < cx.e.chain.len() && 0 <= m2 < cx.e.chain.len() ==> #[trigger] cx.o4[m1]
            != #[trigger] cx.o5[m2]
    &&& forall|m1: int, m2: int|
        0 <= m1 < cx.e.chain.len() && 0 <= m2 < cx.e.chain.len() && m1 != m2
            ==> #[trigger] cx.o4[m1] != #[trigger] cx.o4[m2]
    &&& forall|m1: int, m2: int|
        0 <= m1 < cx.e.chain.len() && 0 <= m2 < cx.e.chain.len() && m1 != m2
            ==> #[trigger] cx.o5[m1] != #[trigger] cx.o5[m2]
}

// --- the concrete transition ---
pub open spec fn out_goals(st: Seq<Goal>, a: Term, v: Term) -> Seq<Goal> {
    match a {
        Term::Var(z) => st.map_values(|g: Goal| subst_goal(g, z, v)),
        _ => st,
    }
}

pub open spec fn out_terms(sol: Seq<Term>, a: Term, v: Term) -> Seq<Term> {
    match a {
        Term::Var(z) => subst_all(sol, z, v),
        _ => sol,
    }
}

// The two output pairs, each value ground: a variable binds its value, a
// value meets itself.
proof fn out_pairs(u0: UState, a4: Term, s: Term, a5: Term, n: Term)
    requires
        u0.pairs == seq![(a4, s), (a5, n)],
        ground(s),
        ground(n),
        a4 is Var || a4 == s,
        a5 is Var || a5 == n,
        a4 is Var && a5 is Var ==> a4 != a5,
    ensures
        exists|h: nat| #[trigger]
            unify_n(u0, h) == UOut::Ok(
                out_goals(out_goals(u0.stack, a4, s), a5, n),
                out_terms(out_terms(u0.sol, a4, s), a5, n),
            ),
{
    let st = u0.stack;
    let sol = u0.sol;
    let st1 = out_goals(st, a4, s);
    let sol1 = out_terms(sol, a4, s);
    let st2 = out_goals(st1, a5, n);
    let sol2 = out_terms(sol1, a5, n);
    let u1 = UState { pairs: seq![(a5, n)], stack: st1, sol: sol1 };
    let u2 = UState { pairs: Seq::<(Term, Term)>::empty(), stack: st2, sol: sol2 };
    reveal_with_fuel(unify_n, 1);
    assert(unify_n(u2, 1) == UOut::Ok(st2, sol2));
    let h1: nat = if a5 is Var {
        let z = a5->Var_0;
        crate::m6_default::ground_absent(n, z);
        assert(u1.pairs.drop_first() =~= Seq::<(Term, Term)>::empty());
        let ub = u_bind(u1, u1.pairs.drop_first(), z, n);
        assert(ub.pairs =~= u2.pairs);
        assert(ub.stack =~= u2.stack);
        assert(ub == u2);
        assert(unify_n(u1, 2) == unify_n(u2, 1));
        2
    } else {
        skip_number(n, u1, 1);
        assert(u1.pairs.drop_first() =~= u2.pairs);
        assert(UState { pairs: u1.pairs.drop_first(), ..u1 } == u2);
        choose|h: nat| #[trigger]
            unify_n(u1, h) == unify_n(UState { pairs: u1.pairs.drop_first(), ..u1 }, 1)
    };
    assert(unify_n(u1, h1) == UOut::Ok(st2, sol2));
    if a4 is Var {
        let z = a4->Var_0;
        crate::m6_default::ground_absent(s, z);
        crate::m6_default::ground_absent(n, z);
        crate::k3_sound::unification::subst_absent(n, z, s);
        let rest = u0.pairs.drop_first();
        assert(rest =~= seq![(a5, n)]);
        assert(subst(a5, z, s) == a5) by {
            if !(a5 is Var) {
                crate::k3_sound::unification::subst_absent(a5, z, s);
            }
        }
        let ub = u_bind(u0, rest, z, s);
        assert(ub.pairs =~= u1.pairs);
        assert(ub.stack =~= u1.stack);
        assert(ub == u1);
        assert(unify_n(u0, h1 + 1) == unify_n(u1, h1));
    } else {
        skip_number(s, u0, h1);
        assert(u0.pairs.drop_first() =~= u1.pairs);
        assert(UState { pairs: u0.pairs.drop_first(), ..u0 } == u1);
    }
}

// Pairs i..4 of the transition: ground terms meeting themselves.
proof fn skip_inputs(ps: Seq<(Term, Term)>, i: int, st: Seq<Goal>, sol: Seq<Term>, g: nat)
    requires
        0 <= i <= 4,
        ps.len() == 6,
        forall|j: int| 0 <= j < 4 ==> (#[trigger] ps[j]).0 == ps[j].1 && ground(ps[j].0),
    ensures
        exists|h: nat| #[trigger]
            unify_n(UState { pairs: ps.skip(i), stack: st, sol }, h) == unify_n(
                UState { pairs: ps.skip(4), stack: st, sol },
                g,
            ),
    decreases 4 - i,
{
    if i == 4 {
        assert(unify_n(UState { pairs: ps.skip(4), stack: st, sol }, g) == unify_n(
            UState { pairs: ps.skip(4), stack: st, sol },
            g,
        ));
    } else {
        skip_inputs(ps, i + 1, st, sol, g);
        let h1 = choose|h: nat| #[trigger]
            unify_n(UState { pairs: ps.skip(i + 1), stack: st, sol }, h) == unify_n(
                UState { pairs: ps.skip(4), stack: st, sol },
                g,
            );
        let u = UState { pairs: ps.skip(i), stack: st, sol };
        assert(u.pairs[0] == ps[i]);
        assert(u.pairs.drop_first() =~= ps.skip(i + 1));
        skip_number(ps[i].0, u, h1);
        let h0 = choose|h: nat| #[trigger]
            unify_n(u, h) == unify_n(UState { pairs: u.pairs.drop_first(), ..u }, h1);
        assert(UState { pairs: u.pairs.drop_first(), ..u } == UState {
            pairs: ps.skip(i + 1),
            stack: st,
            sol,
        });
        assert(unify_n(u, h0) == unify_n(UState { pairs: ps.skip(4), stack: st, sol }, g));
    }
}

// The arithmetic transition's unification: the four ground input pairs meet
// themselves, then the output pairs.
pub proof fn arith_unify(args: Seq<Term>, s: Term, n: Term, st: Seq<Goal>, sol: Seq<Term>)
    requires
        args.len() == 6,
        forall|i: int| 0 <= i < 4 ==> ground(#[trigger] args[i]),
        ground(s),
        ground(n),
        args[4] is Var || args[4] == s,
        args[5] is Var || args[5] == n,
        args[4] is Var && args[5] is Var ==> args[4] != args[5],
    ensures
        unify(UState { pairs: zip(args, args.take(4) + seq![s, n]), stack: st, sol }) == UOut::Ok(
            out_goals(out_goals(st, args[4], s), args[5], n),
            out_terms(out_terms(sol, args[4], s), args[5], n),
        ),
{
    let ys = args.take(4) + seq![s, n];
    let ps = zip(args, ys);
    assert(ps.len() == 6);
    assert forall|j: int| 0 <= j < 4 implies (#[trigger] ps[j]).0 == ps[j].1 && ground(ps[j].0) by {
        assert(ps[j] == (args[j], ys[j]));
        assert(ys[j] == args[j]);
    }
    assert(ps.skip(4) =~= seq![(args[4], s), (args[5], n)]) by {
        assert(ys[4] == s);
        assert(ys[5] == n);
    }
    let fin = UOut::Ok(
        out_goals(out_goals(st, args[4], s), args[5], n),
        out_terms(out_terms(sol, args[4], s), args[5], n),
    );
    let u4 = UState { pairs: seq![(args[4], s), (args[5], n)], stack: st, sol };
    out_pairs(u4, args[4], s, args[5], n);
    let h4 = choose|h: nat| #[trigger] unify_n(u4, h) == fin;
    assert(UState { pairs: ps.skip(4), stack: st, sol } == u4);
    skip_inputs(ps, 0, st, sol, h4);
    let h0 = choose|h: nat| #[trigger]
        unify_n(UState { pairs: ps.skip(0), stack: st, sol }, h) == unify_n(
            UState { pairs: ps.skip(4), stack: st, sol },
            h4,
        );
    assert(ps.skip(0) =~= ps);
    assert(unify_n(UState { pairs: ps, stack: st, sol }, h0) == fin);
    crate::k3_sound::naf::unify_at(UState { pairs: ps, stack: st, sol }, h0);
}

// --- concrete ↔ abstract terms ---
proof fn rename_pre_ground(c: Term, g: Term, b: nat, d: nat)
    requires
        rename(c, b, d) == g,
        ground(g),
    ensures
        c == g,
{
    crate::k3_sound::naf::rename_ground(c, b, d);
    crate::k3_sound::naf::rename_ground_id(c, b, d);
}

proof fn rename_pre_var(c: Term, y: nat, b: nat, d: nat)
    requires
        rename(c, b, d) == Term::Var(y),
    ensures
        c is Var,
        lift_var(c->Var_0, b, d) == y,
{
}

// An output argument of the concrete goal: its placeholder unbound ⇒ a
// variable renaming to it; bound ⇒ its value.
proof fn out_arg(a: Term, x: nat, v: Term, th: Seq<(nat, Term)>, b: nat, d: nat)
    requires
        rename(a, b, d) == inst(Term::Var(x), th),
        th_val(th, x) is Some ==> th_val(th, x) == Some(v),
        ground(v),
    ensures
        th_val(th, x) is None ==> a is Var && lift_var(a->Var_0, b, d) == x,
        th_val(th, x) is Some ==> a == v,
{
    if th_val(th, x) is Some {
        rename_pre_ground(a, v, b, d);
    } else {
        rename_pre_var(a, x, b, d);
    }
}

// The instantiation once an unbound x takes v.
pub open spec fn ext(th: Seq<(nat, Term)>, x: nat, v: Term) -> Seq<(nat, Term)> {
    if th_val(th, x) is None {
        seq![(x, v)] + th
    } else {
        th
    }
}

proof fn ext_val(th: Seq<(nat, Term)>, x: nat, v: Term, y: nat)
    ensures
        th_val(ext(th, x, v), y) == if y == x && th_val(th, x) is None {
            Some(v)
        } else {
            th_val(th, y)
        },
{
    let th2 = seq![(x, v)] + th;
    assert(th2[0] == (x, v));
    assert(th2.drop_first() =~= th);
}

proof fn ext_ok(th: Seq<(nat, Term)>, x: nat, v: Term)
    requires
        th_ok(th),
        ground(v),
    ensures
        th_ok(ext(th, x, v)),
{
    th_extend_ok(th, x, v);
}

// A binding of the concrete goal's output carries the related stack along:
// the stack renders under the instantiation extended by the value.
proof fn out_srel(
    r: Seq<Goal>,
    ra: Seq<Goal>,
    th: Seq<(nat, Term)>,
    a: Term,
    x: nat,
    v: Term,
    b: nat,
    d: nat,
)
    requires
        srel(r, ra, th, b, d),
        th_ok(th),
        ground(v),
        rename(a, b, d) == inst(Term::Var(x), th),
        th_val(th, x) is Some ==> th_val(th, x) == Some(v),
    ensures
        srel(out_goals(r, a, v), ra, ext(th, x, v), b, d),
        th_ok(ext(th, x, v)),
{
    ext_ok(th, x, v);
    out_arg(a, x, v, th, b, d);
    if th_val(th, x) is None {
        let z = a->Var_0;
        let r2 = out_goals(r, a, v);
        let th2 = seq![(x, v)] + th;
        assert(ext(th, x, v) == th2);
        assert forall|i: int| 0 <= i < r2.len() implies #[trigger] grel(
            r2[i],
            ra[i],
            th2,
            b,
            d,
        ) by {
            assert(grel(r[i], ra[i], th, b, d));
            assert(r2[i] == subst_goal(r[i], z, v));
            match (r[i], ra[i]) {
                (Goal::Lit(ct, d1), Goal::Lit(at, d2)) => {
                    crate::k3_sound::naf::rename_subst(ct, z, v, b, d);
                    crate::k3_sound::naf::rename_ground_id(v, b, d);
                    inst_ext(at, x, v, th);
                },
                _ => {},
            }
        }
    } else {
        assert(!(a is Var)) by {
            assert(ground(a));
        }
    }
}

// --- the inputs ---
// Input pair q of node kk in one run: the concrete goal's pair = the chain
// node's pair bound by the run's chain values.
proof fn node_input(
    cx: Cx,
    th: Seq<(nat, Term)>,
    vv: Seq<(Term, Term)>,
    ss: Seq<(nat, Term)>,
    ga: Term,
    g: Term,
    kk: nat,
    q: int,
)
    requires
        q == 0 || q == 2,
        kk < cx.e.chain.len(),
        th_ok(th),
        th_ok(ss),
        crate::m6_default::vals_ground(vv),
        forall|x: nat| #[trigger] th_val(ss, x) is Some ==> th_val(th, x) == th_val(ss, x),
        forall|x: nat| #[trigger] th_val(ss, x) is Some <==> cx.e.slots.contains(Term::Var(x)),
        forall|m: int|
            0 <= m < kk ==> #[trigger] th_val(th, cx.o4[m]) == lookup(vv, arg(cx.e.chain[m], 4))
                && th_val(th, cx.o5[m]) == lookup(vv, arg(cx.e.chain[m], 5)),
        forall|m: int|
            0 <= m < kk ==> lookup(vv, arg(#[trigger] cx.e.chain[m], 4)) is Some && lookup(
                vv,
                arg(cx.e.chain[m], 5),
            ) is Some,
        cin(cx.e, vv, ss, kk as int, q),
        ws::input_ok(cx.e, ga, cx.e.chain[kk as int], q),
        node_shape(cx, ga, kk),
        g is Comp,
        g->Comp_1.len() == 6,
        rename(g->Comp_1[q], cx.b, cx.d) == inst(ga->Comp_1[q], th),
        rename(g->Comp_1[q + 1], cx.b, cx.d) == inst(ga->Comp_1[q + 1], th),
    ensures
        g->Comp_1[q] == bind_all(arg(cx.e.chain[kk as int], q), vv),
        g->Comp_1[q + 1] == bind_all(arg(cx.e.chain[kk as int], q + 1), vv),
        ground(g->Comp_1[q]),
        ground(g->Comp_1[q + 1]),
{
    let c = cx.e.chain[kk as int];
    let s = arg(c, q);
    let n = arg(c, q + 1);
    let gs = g->Comp_1[q];
    let gn = g->Comp_1[q + 1];
    let as_ = ga->Comp_1[q];
    let an = ga->Comp_1[q + 1];
    assert(arg(ga, q) == as_ && arg(ga, q + 1) == an);
    if cx.e.slots.contains(n) {
        let x = n->Var_0;
        assert(n == Term::Var(x));
        assert(th_val(ss, x) is Some);
        th_val_number(ss, x);
        let w = th_val(ss, x)->0;
        assert(inst(an, th) == w);
        rename_pre_ground(gn, w, cx.b, cx.d);
        assert(ground(ws::exactly()));
        inst_ground(ws::exactly(), th);
        rename_pre_ground(gs, ws::exactly(), cx.b, cx.d);
        crate::m6_default::bind_var(s->Var_0, vv);
        crate::m6_default::bind_var(x, vv);
    } else if n is Var {
        let m = choose|m: int|
            0 <= m < kk && s == arg(#[trigger] cx.e.chain[m], 4) && n == arg(cx.e.chain[m], 5);
        assert(0 <= m < kk);
        assert(from_node(cx, ga, kk, 0, m));
        assert(from_node(cx, ga, kk, 2, m));
        assert(as_ == Term::Var(cx.o4[m]) && an == Term::Var(cx.o5[m]));
        lookup_ground(vv, s);
        lookup_ground(vv, n);
        assert(th_val(th, cx.o4[m]) == lookup(vv, s));
        assert(th_val(th, cx.o5[m]) == lookup(vv, n));
        rename_pre_ground(gs, lookup(vv, s)->0, cx.b, cx.d);
        rename_pre_ground(gn, lookup(vv, n)->0, cx.b, cx.d);
        crate::m6_default::bind_var(s->Var_0, vv);
        crate::m6_default::bind_var(n->Var_0, vv);
    } else {
        inst_ground(s, th);
        inst_ground(n, th);
        rename_pre_ground(gs, s, cx.b, cx.d);
        rename_pre_ground(gn, n, cx.b, cx.d);
        crate::m6_default::bind_ground(s, vv);
        crate::m6_default::bind_ground(n, vv);
    }
}

// The concrete goal of node kk in one run computes the node's values there.
proof fn node_eval(
    cx: Cx,
    th: Seq<(nat, Term)>,
    vv: Seq<(Term, Term)>,
    ss: Seq<(nat, Term)>,
    ga: Term,
    g: Term,
    kk: nat,
)
    requires
        kk < cx.e.chain.len(),
        th_ok(th),
        th_ok(ss),
        crate::m6_default::vals_ground(vv),
        forall|x: nat| #[trigger] th_val(ss, x) is Some ==> th_val(th, x) == th_val(ss, x),
        forall|x: nat| #[trigger] th_val(ss, x) is Some <==> cx.e.slots.contains(Term::Var(x)),
        forall|m: int|
            0 <= m < kk ==> #[trigger] th_val(th, cx.o4[m]) == lookup(vv, arg(cx.e.chain[m], 4))
                && th_val(th, cx.o5[m]) == lookup(vv, arg(cx.e.chain[m], 5)),
        forall|m: int|
            0 <= m < kk ==> lookup(vv, arg(#[trigger] cx.e.chain[m], 4)) is Some && lookup(
                vv,
                arg(cx.e.chain[m], 5),
            ) is Some,
        node_at(cx.e, vv, ss, kk as int),
        ws::arith_ok(cx.e, ga, cx.e.chain[kk as int]),
        node_shape(cx, ga, kk),
        g is Comp,
        g->Comp_0 == ga->Comp_0,
        g->Comp_1.len() == 6,
        forall|i: int|
            0 <= i < 6 ==> rename(#[trigger] g->Comp_1[i], cx.b, cx.d) == inst(ga->Comp_1[i], th),
    ensures
        forall|i: int| 0 <= i < 4 ==> ground(#[trigger] g->Comp_1[i]),
        arith_result(g) == Some(
            (
                lookup(vv, arg(cx.e.chain[kk as int], 4))->0,
                lookup(vv, arg(cx.e.chain[kk as int], 5))->0,
            ),
        ),
{
    let c = cx.e.chain[kk as int];
    assert(rename(g->Comp_1[0], cx.b, cx.d) == inst(ga->Comp_1[0], th));
    assert(rename(g->Comp_1[1], cx.b, cx.d) == inst(ga->Comp_1[1], th));
    assert(rename(g->Comp_1[2], cx.b, cx.d) == inst(ga->Comp_1[2], th));
    assert(rename(g->Comp_1[3], cx.b, cx.d) == inst(ga->Comp_1[3], th));
    node_input(cx, th, vv, ss, ga, g, kk, 0);
    node_input(cx, th, vv, ss, ga, g, kk, 2);
    crate::m6_default::bind_comp(c->Comp_0, c->Comp_1, vv);
    let bc = bind_all(c, vv);
    assert(c == Term::Comp(c->Comp_0, c->Comp_1));
    assert forall|i: int| 0 <= i < 4 implies bc->Comp_1[i] == g->Comp_1[i] by {
        assert(arg(c, i) == c->Comp_1[i]);
    }
    assert(g == Term::Comp(g->Comp_0, g->Comp_1));
    assert(arith_result(g) == arith_result(bc));
}

// --- the step ---
// The walk on an arithmetic first goal: node k, then the rest at k + 1.
proof fn walk_arith(cx: Cx, cur: Fr)
    requires
        fr_ok(cx, cur),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        ws::is_arith_lit(cur.stack[0]->Lit_0),
    ensures
        cur.n > 0,
        cur.k < cx.e.chain.len(),
        ws::arith_ok(cx.e, cur.stack[0]->Lit_0, cx.e.chain[cur.k as int]),
        ws::walk_ok(cx.e, enc(seg(cur.stack.drop_first())), (cur.n - 1) as nat, cur.k + 1),
{
    let ga = cur.stack[0]->Lit_0;
    let ra = cur.stack.drop_first();
    assert(cur.stack =~= seq![cur.stack[0]] + ra);
    seg_cons(cur.stack[0], ra);
    let ts1 = enc(seg(cur.stack));
    assert(ts1 =~= seq![ga] + enc(seg(ra)));
    reveal_with_fuel(ws::walk_ok, 1);
    assert(ts1[0] == ga && ts1.drop_first() =~= enc(seg(ra)));
}

proof fn ext2_val(th: Seq<(nat, Term)>, x4: nat, v4: Term, x5: nat, v5: Term, y: nat)
    requires
        x4 != x5,
    ensures
        th_val(ext(ext(th, x4, v4), x5, v5), y) == if y == x4 && th_val(th, x4) is None {
            Some(v4)
        } else if y == x5 && th_val(th, x5) is None {
            Some(v5)
        } else {
            th_val(th, y)
        },
{
    ext_val(th, x4, v4, y);
    ext_val(ext(th, x4, v4), x5, v5, y);
    ext_val(th, x4, v4, x5);
}

// The frame after node k's step: the rest of the stack at node k + 1, the
// node's outputs bound to their values in each run.
proof fn arith_frame(cx: Cx, cur: Fr, svt: Term, nvt: Term, svu: Term, nvu: Term)
    requires
        cx_ok(cx),
        chain_ok(cx),
        fr_ok(cx, cur),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        ws::is_arith_lit(cur.stack[0]->Lit_0),
        cur.n > 0,
        cur.k < cx.e.chain.len(),
        ws::walk_ok(cx.e, enc(seg(cur.stack.drop_first())), (cur.n - 1) as nat, cur.k + 1),
        lookup(cx.vt, arg(cx.e.chain[cur.k as int], 4)) == Some(svt),
        lookup(cx.vt, arg(cx.e.chain[cur.k as int], 5)) == Some(nvt),
        lookup(cx.vu, arg(cx.e.chain[cur.k as int], 4)) == Some(svu),
        lookup(cx.vu, arg(cx.e.chain[cur.k as int], 5)) == Some(nvu),
        ground(svt),
        ground(nvt),
        ground(svu),
        ground(nvu),
    ensures
        fr_ok(
            cx,
            Fr {
                stack: cur.stack.drop_first(),
                tht: ext(ext(cur.tht, cx.o4[cur.k as int], svt), cx.o5[cur.k as int], nvt),
                thu: ext(ext(cur.thu, cx.o4[cur.k as int], svu), cx.o5[cur.k as int], nvu),
                k: cur.k + 1,
                n: (cur.n - 1) as nat,
            },
        ),
{
    let kk = cur.k as int;
    let o4 = cx.o4[kk];
    let o5 = cx.o5[kk];
    let ra = cur.stack.drop_first();
    let t2 = ext(ext(cur.tht, o4, svt), o5, nvt);
    let u2 = ext(ext(cur.thu, o4, svu), o5, nvu);
    let fr2 = Fr { stack: ra, tht: t2, thu: u2, k: cur.k + 1, n: (cur.n - 1) as nat };
    assert(o4 != o5);
    assert(o4 < cx.e.off && o5 < cx.e.off);
    assert(cx.o4.contains(o4));
    assert(cx.o5.contains(o5));
    assert(is_ph(cx, o4) && is_ph(cx, o5));
    ext_ok(cur.tht, o4, svt);
    ext_ok(ext(cur.tht, o4, svt), o5, nvt);
    ext_ok(cur.thu, o4, svu);
    ext_ok(ext(cur.thu, o4, svu), o5, nvu);
    // the instantiation's domain and the slots
    assert forall|x: nat| #[trigger] th_val(t2, x) is Some <==> th_val(u2, x) is Some by {
        ext2_val(cur.tht, o4, svt, o5, nvt, x);
        ext2_val(cur.thu, o4, svu, o5, nvu, x);
        assert(th_val(cur.tht, x) is Some <==> th_val(cur.thu, x) is Some);
        assert(th_val(cur.tht, o4) is Some <==> th_val(cur.thu, o4) is Some);
        assert(th_val(cur.tht, o5) is Some <==> th_val(cur.thu, o5) is Some);
    }
    assert forall|x: nat| #[trigger] th_val(t2, x) is Some implies x < cx.e.off by {
        ext2_val(cur.tht, o4, svt, o5, nvt, x);
        if th_val(cur.tht, x) is Some {
        }
    }
    assert forall|x: nat| #[trigger] th_val(cx.st, x) is Some implies th_val(t2, x) == th_val(
        cx.st,
        x,
    ) && th_val(u2, x) == th_val(cx.su, x) by {
        ext2_val(cur.tht, o4, svt, o5, nvt, x);
        ext2_val(cur.thu, o4, svu, o5, nvu, x);
        assert(th_val(cx.st, o4) is None && th_val(cx.st, o5) is None);
    }
    // the first segment
    assert(cur.stack =~= seq![cur.stack[0]] + ra);
    seg_cons(cur.stack[0], ra);
    assert(crate::m6_fragment::structured_upto(enc(seg(ra)), seg(ra).len() as int)) by {
        assert forall|i: int| 0 <= i < seg(ra).len() implies crate::m6_fragment::structured(
            #[trigger] enc(seg(ra))[i],
        ) by {
            assert(enc(seg(cur.stack))[i + 1] == enc(seg(ra))[i]);
        }
    }
    assert(si(cx, fr2)) by {
        assert forall|i: int| 0 <= i < seg(ra).len() implies #[trigger] goal_si(
            cx,
            seg(ra)[i],
            fr2.k + arank(seg(ra), i),
        ) by {
            assert(seg(cur.stack)[i + 1] == seg(ra)[i]);
            arank_cons(cur.stack[0], seg(ra), i);
            assert(goal_si(cx, seg(cur.stack)[i + 1], cur.k + arank(seg(cur.stack), i + 1)));
        }
    }
    // the placeholders' values
    assert forall|x: nat| #[trigger] th_val(t2, x) is Some implies th_val(cx.st, x) is Some
        || is_ph(cx, x) by {
        ext2_val(cur.tht, o4, svt, o5, nvt, x);
    }
    assert forall|m: int|
        0 <= m < cx.e.chain.len() && #[trigger] th_val(t2, cx.o5[m]) is Some implies th_val(
        t2,
        cx.o5[m],
    ) == lookup(cx.vt, arg(cx.e.chain[m], 5)) && th_val(u2, cx.o5[m]) == lookup(
        cx.vu,
        arg(cx.e.chain[m], 5),
    ) by {
        ext2_val(cur.tht, o4, svt, o5, nvt, cx.o5[m]);
        ext2_val(cur.thu, o4, svu, o5, nvu, cx.o5[m]);
        if m == kk {
            assert(th_val(cur.tht, o5) is Some <==> th_val(cur.thu, o5) is Some);
        } else {
            assert(cx.o5[m] != o4 && cx.o5[m] != o5);
        }
        assert(th_val(cur.tht, cx.o5[m]) is Some <==> th_val(cur.thu, cx.o5[m]) is Some);
    }
    assert forall|m: int|
        0 <= m < cx.e.chain.len() && #[trigger] th_val(t2, cx.o4[m]) is Some implies th_val(
        t2,
        cx.o4[m],
    ) == lookup(cx.vt, arg(cx.e.chain[m], 4)) && th_val(u2, cx.o4[m]) == lookup(
        cx.vu,
        arg(cx.e.chain[m], 4),
    ) by {
        ext2_val(cur.tht, o4, svt, o5, nvt, cx.o4[m]);
        ext2_val(cur.thu, o4, svu, o5, nvu, cx.o4[m]);
        if m == kk {
            assert(th_val(cur.tht, o4) is Some <==> th_val(cur.thu, o4) is Some);
        } else {
            assert(cx.o4[m] != o4 && cx.o4[m] != o5);
        }
        assert(th_val(cur.tht, cx.o4[m]) is Some <==> th_val(cur.thu, cx.o4[m]) is Some);
    }
    assert forall|m: int| 0 <= m < fr2.k implies #[trigger] th_val(t2, cx.o5[m]) is Some && th_val(
        t2,
        cx.o4[m],
    ) is Some by {
        ext2_val(cur.tht, o4, svt, o5, nvt, cx.o5[m]);
        ext2_val(cur.tht, o4, svt, o5, nvt, cx.o4[m]);
        if m < kk {
            assert(th_val(cur.tht, cx.o5[m]) is Some);
        }
    }
    assert(th_vals(cx, fr2));
    assert(fr_ok(cx, fr2));
}

// Contract ws D7, the arithmetic step: both runs compute node k's values and
// continue related, the frame's instantiation extended by them.
pub proof fn step_arith(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>)
    requires
        cx_ok(cx),
        chain_ok(cx),
        rel(cx, ct, cu, cur, alts),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        cur.stack[0]->Lit_1 > 0,
        ws::is_arith_lit(cur.stack[0]->Lit_0),
    ensures
        step_rel(cx, step(cx.dbt, ct), step(cx.dbu, cu)),
{
    let ga = cur.stack[0]->Lit_0;
    let kk = cur.k;
    let ra = cur.stack.drop_first();
    first_lit(cx, ct, cu, cur, alts);
    walk_arith(cx, cur);
    walk_not_operator(cx, cur);
    selected_zero(ct.stack);
    selected_zero(cu.stack);
    let c = cx.e.chain[kk as int];
    let gt = ct.stack[0]->Lit_0;
    let gu = cu.stack[0]->Lit_0;
    let at = gt->Comp_1;
    let au = gu->Comp_1;
    assert(seg(cur.stack)[0] == cur.stack[0]);
    assert(arank(seg(cur.stack), 0) == 0);
    assert(goal_si(cx, seg(cur.stack)[0], cur.k + arank(seg(cur.stack), 0)));
    assert(node_shape(cx, ga, kk));
    let o4 = cx.o4[kk as int];
    let o5 = cx.o5[kk as int];
    assert(node_at(cx.e, cx.vt, cx.st, kk as int));
    assert(node_at(cx.e, cx.vu, cx.su, kk as int));
    // the runs below node kk
    assert forall|m: int| 0 <= m < kk implies #[trigger] th_val(cur.tht, cx.o4[m]) == lookup(
        cx.vt,
        arg(cx.e.chain[m], 4),
    ) && th_val(cur.tht, cx.o5[m]) == lookup(cx.vt, arg(cx.e.chain[m], 5)) by {
        assert(th_val(cur.tht, cx.o5[m]) is Some);
        assert(th_val(cur.tht, cx.o4[m]) is Some);
    }
    assert forall|m: int| 0 <= m < kk implies #[trigger] th_val(cur.thu, cx.o4[m]) == lookup(
        cx.vu,
        arg(cx.e.chain[m], 4),
    ) && th_val(cur.thu, cx.o5[m]) == lookup(cx.vu, arg(cx.e.chain[m], 5)) by {
        assert(th_val(cur.tht, cx.o5[m]) is Some);
        assert(th_val(cur.tht, cx.o4[m]) is Some);
    }
    assert forall|m: int| 0 <= m < kk implies lookup(
        cx.vt,
        arg(#[trigger] cx.e.chain[m], 4),
    ) is Some && lookup(cx.vt, arg(cx.e.chain[m], 5)) is Some by {
        assert(node_at(cx.e, cx.vt, cx.st, m));
    }
    assert forall|m: int| 0 <= m < kk implies lookup(
        cx.vu,
        arg(#[trigger] cx.e.chain[m], 4),
    ) is Some && lookup(cx.vu, arg(cx.e.chain[m], 5)) is Some by {
        assert(node_at(cx.e, cx.vu, cx.su, m));
    }
    assert forall|x: nat| #[trigger] th_val(cx.su, x) is Some implies th_val(cur.thu, x) == th_val(
        cx.su,
        x,
    ) by {
        assert(cx.e.slots.contains(Term::Var(x)));
        assert(th_val(cx.st, x) is Some);
    }
    assert forall|i: int| 0 <= i < 6 implies rename(#[trigger] at[i], cx.b, cx.d) == inst(
        ga->Comp_1[i],
        cur.tht,
    ) by {}
    assert forall|i: int| 0 <= i < 6 implies rename(#[trigger] au[i], cx.b, cx.d) == inst(
        ga->Comp_1[i],
        cur.thu,
    ) by {
        assert(rename(at[i], cx.b, cx.d) == inst(ga->Comp_1[i], cur.tht));
    }
    node_eval(cx, cur.tht, cx.vt, cx.st, ga, gt, kk);
    node_eval(cx, cur.thu, cx.vu, cx.su, ga, gu, kk);
    let svt = lookup(cx.vt, arg(c, 4))->0;
    let nvt = lookup(cx.vt, arg(c, 5))->0;
    let svu = lookup(cx.vu, arg(c, 4))->0;
    let nvu = lookup(cx.vu, arg(c, 5))->0;
    lookup_ground(cx.vt, arg(c, 4));
    lookup_ground(cx.vt, arg(c, 5));
    lookup_ground(cx.vu, arg(c, 4));
    lookup_ground(cx.vu, arg(c, 5));
    // the outputs: a placeholder bound in the frame carries its node's value
    assert(ga->Comp_1[4] == Term::Var(o4) && ga->Comp_1[5] == Term::Var(o5));
    assert(th_val(cur.tht, o4) is Some ==> th_val(cur.tht, o4) == Some(svt) && th_val(cur.thu, o4)
        == Some(svu));
    assert(th_val(cur.tht, o5) is Some ==> th_val(cur.tht, o5) == Some(nvt) && th_val(cur.thu, o5)
        == Some(nvu));
    assert(th_val(cur.tht, o4) is Some <==> th_val(cur.thu, o4) is Some);
    assert(th_val(cur.tht, o5) is Some <==> th_val(cur.thu, o5) is Some);
    out_arg(at[4], o4, svt, cur.tht, cx.b, cx.d);
    out_arg(at[5], o5, nvt, cur.tht, cx.b, cx.d);
    out_arg(au[4], o4, svu, cur.thu, cx.b, cx.d);
    out_arg(au[5], o5, nvu, cur.thu, cx.b, cx.d);
    assert(o4 != o5);
    let rt = ct.stack.skip(1);
    let ru = cu.stack.skip(1);
    let pre_t = ct.stack.take(0);
    let pre_u = cu.stack.take(0);
    arith_unify(at, svt, nvt, pre_t + rt, ct.sol);
    arith_unify(au, svu, nvu, pre_u + ru, cu.sol);
    assert(pre_t + rt =~= rt);
    assert(pre_u + ru =~= ru);
    let nt = Cfg {
        stack: out_goals(out_goals(rt, at[4], svt), at[5], nvt),
        sol: out_terms(out_terms(ct.sol, at[4], svt), at[5], nvt),
        ..ct
    };
    let nu = Cfg {
        stack: out_goals(out_goals(ru, au[4], svu), au[5], nvu),
        sol: out_terms(out_terms(cu.sol, au[4], svu), au[5], nvu),
        ..cu
    };
    assert(gt == Term::Comp(gt->Comp_0, at));
    assert(gu == Term::Comp(gu->Comp_0, au));
    assert(ckc_spec::arith::is_arith(gt->Comp_0, at.len()));
    assert(step(cx.dbt, ct) == Step::Next(nt));
    assert(step(cx.dbu, cu) == Step::Next(nu));
    // the new frame
    srel_tail(ct.stack, cur.stack, cur.tht, cx.b, cx.d, 1);
    srel_tail(cu.stack, cur.stack, cur.thu, cx.b, cx.d, 1);
    assert(cur.stack.skip(1) =~= ra);
    let t1 = ext(cur.tht, o4, svt);
    let u1 = ext(cur.thu, o4, svu);
    out_srel(rt, ra, cur.tht, at[4], o4, svt, cx.b, cx.d);
    ext_val(cur.tht, o4, svt, o5);
    out_srel(out_goals(rt, at[4], svt), ra, t1, at[5], o5, nvt, cx.b, cx.d);
    out_srel(ru, ra, cur.thu, au[4], o4, svu, cx.b, cx.d);
    ext_val(cur.thu, o4, svu, o5);
    out_srel(out_goals(ru, au[4], svu), ra, u1, au[5], o5, nvu, cx.b, cx.d);
    arith_frame(cx, cur, svt, nvt, svu, nvu);
    let fr2 = Fr {
        stack: ra,
        tht: ext(t1, o5, nvt),
        thu: ext(u1, o5, nvu),
        k: kk + 1,
        n: (cur.n - 1) as nat,
    };
    assert(cuts_ok(ra, alts.len())) by {
        assert forall|j: int|
            0 <= j < ra.len() && (#[trigger] ra[j]) is NafCut implies ra[j]->NafCut_0
            < alts.len() by {
            assert(ra[j] == cur.stack[j + 1]);
        }
    }
    assert(ct.sol =~= Seq::<Term>::empty());
    assert(cu.sol =~= Seq::<Term>::empty());
    assert(nt.sol.len() == 0);
    assert(nu.sol.len() == 0);
    assert(rel(cx, nt, nu, fr2, alts));
    step_next(cx, nt, nu, fr2, alts);
}

// Contract ws D7: one K2 step at t and at u from related configurations.
pub proof fn step_lock(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>)
    requires
        cx_ok(cx),
        chain_ok(cx),
        rel(cx, ct, cu, cur, alts),
    ensures
        step_rel(cx, step(cx.dbt, ct), step(cx.dbu, cu)),
{
    if cur.stack.len() == 0 || cur.stack[0] is NafCut || cur.stack[0]->Lit_1 == 0 || !(
    cur.stack[0]->Lit_0 is Comp) {
        step_trivial(cx, ct, cu, cur, alts);
    } else {
        let ga = cur.stack[0]->Lit_0;
        if is_conj(ga) {
            step_comma(cx, ct, cu, cur, alts);
        } else if is_neg(ga) {
            step_neg(cx, ct, cu, cur, alts);
        } else if ws::is_arith_lit(ga) {
            step_arith(cx, ct, cu, cur, alts);
        } else if ckc_spec::compare::is_builtin(ga->Comp_0, ga->Comp_1.len()) {
            step_cmp(cx, ct, cu, cur, alts);
        } else {
            step_call(cx, ct, cu, cur, alts);
        }
    }
}

} // verus!
