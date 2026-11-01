#[cfg(verus_keep_ghost)]
use crate::m6_arith::*;
#[cfg(verus_keep_ghost)]
use crate::m6_bfr::*;
#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use crate::m6_ctx::*;
#[cfg(verus_keep_ghost)]
use crate::m6_enter::*;
#[cfg(verus_keep_ghost)]
use crate::m6_entry::*;
#[cfg(verus_keep_ghost)]
use crate::m6_hd::*;
#[cfg(verus_keep_ghost)]
use crate::m6_lockstep::*;
#[cfg(verus_keep_ghost)]
use crate::m6_setup::*;
#[cfg(verus_keep_ghost)]
use crate::m6_step::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::DocClause;
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

// Contract ws D7 (cell invariance), assembled: per head, the lockstep context
// and the head call's context satisfy what the lockstep reads, and the two
// replays start related, so they end alike.
pub proof fn fr_ok_bfr(
    p: Seq<DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
    dd: nat,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        g.clauses[j].head is Comp,
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> ckc_spec::arith::number(#[trigger] t[i]) is Some,
        forall|i: int| 0 <= i < u.len() ==> ckc_spec::arith::number(#[trigger] u[i]) is Some,
        ws::tuple_pairs(g, sel, t) is Some,
        ws::tuple_pairs(g, sel, u) is Some,
    ensures
        fr_ok(mk_cx(p, g, sel, j, t, u), bfr(g, sel, j, t, u, dd)),
{
    let cx = mk_cx(p, g, sel, j, t, u);
    let fr = bfr(g, sel, j, t, u, dd);
    assert(ws::counts(sel).no_duplicates());
    assert(ws::chain_body(sel, g.clauses[j].body));
    th2_ok(g, sel, j, t);
    th2_ok(g, sel, j, u);
    assert forall|x: nat| #[trigger] th_val(fr.tht, x) is Some <==> th_val(fr.thu, x) is Some by {
        th2_dom_eq(g, sel, j, t, u, x);
    }
    assert forall|x: nat| #[trigger] th_val(fr.tht, x) is Some implies x < cx.e.off by {
        th2_below(p, g, sel, j, t, x);
    }
    assert forall|x: nat| #[trigger] th_val(cx.st, x) is Some implies th_val(fr.tht, x) == th_val(
        cx.st,
        x,
    ) && th_val(fr.thu, x) == th_val(cx.su, x) by {
        slot_val(sel, t, x);
        slot_val(sel, u, x);
        th2_slots(g, sel, j, t, x);
        th2_slots(g, sel, j, u, x);
    }
    si_bfr(p, g, sel, j, t, u, dd);
    walk_body(p, g, sel, j);
    th_vals_bfr(p, g, sel, j, t, u, dd);
}

// The head call's context of head j.
pub open spec fn mk_hx(g: spec::Group, sel: ws::Sel, j: int, t: Seq<Term>, u: Seq<Term>) -> Hx {
    let b = nvars(tob(g, sel, t).heads[j]);
    let dd = (replay_depth() - 1) as nat;
    Hx {
        h: ws::open_head(g, sel, j),
        hoff: hoff(g, sel, j),
        c: spec::canon_clause(g.clauses[j]),
        tht: th0(g, sel, j, t),
        thu: th0(g, sel, j, u),
        fr: bfr(g, sel, j, t, u, dd),
        stt: cstk(g, sel, j, t, b, dd),
        stu: cstk(g, sel, j, u, b, dd),
    }
}

pub proof fn hx_good(
    p: Seq<DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> ckc_spec::arith::number(#[trigger] t[i]) is Some,
        forall|i: int| 0 <= i < u.len() ==> ckc_spec::arith::number(#[trigger] u[i]) is Some,
        ws::tuple_pairs(g, sel, t) is Some,
        ws::tuple_pairs(g, sel, u) is Some,
    ensures
        head_ok(mk_cx(p, g, sel, j, t, u), mk_hx(g, sel, j, t, u)),
{
    let cx = mk_cx(p, g, sel, j, t, u);
    let hx = mk_hx(g, sel, j, t, u);
    let h = ws::open_head(g, sel, j);
    let sk = ws::skeleton(g, sel);
    let dd = (replay_depth() - 1) as nat;
    head_nvars(p, g, sel, j, t, u);
    head_facts(p, g, sel, j, t);
    head_facts(p, g, sel, j, u);
    let b = cx.b;
    assert(b <= hoff(g, sel, j));
    assert(nvars(cx.hu) == b);
    crate::k3_sound::naf::rename_fixed(cx.ht, cx.b, cx.d);
    crate::k3_sound::naf::rename_fixed(cx.hu, cx.b, cx.d);
    assert(ws::clause_ok(p, g, sel, j));
    assert(ws::head_call(h));
    assert(is_call(h)) by {
        reveal(ckc_spec::arith::is_arith);
    }
    assert forall|k: int| 0 <= k < cx.nf implies !ws::unifiable(hx.h, #[trigger] cx.e.facts[k]) by {
        assert(cx.e.facts[k] == sk[k]);
    }
    assert forall|k: int| 0 <= k < cx.nf implies inst(#[trigger] cx.e.facts[k], hx.tht) == inst(
        cx.e.facts[k],
        cx.st,
    ) && inst(cx.e.facts[k], hx.thu) == inst(cx.e.facts[k], cx.su) by {
        assert(cx.e.facts[k] == sk[k]);
    }
    if lit_fa(hx.c.head) == lit_fa(hx.h) {
        assert(g.clauses[j].head is Comp);
        entry_unify(p, g, sel, j, t, b, dd);
        entry_unify(p, g, sel, j, u, b, dd);
        entry_srel(p, g, sel, j, t, b, dd);
        entry_srel(p, g, sel, j, u, b, dd);
        fr_ok_bfr(p, g, sel, j, t, u, dd);
        assert(entry(cx, cx.ht, hx.c).stack == Seq::<Goal>::empty() + body_goals(hx.c.body, b, dd)
            + Seq::<Goal>::empty());
        assert(cuts_ok(hx.fr.stack, 1)) by {
            assert forall|q: int|
                0 <= q < hx.fr.stack.len() && (
                #[trigger] hx.fr.stack[q]) is NafCut implies hx.fr.stack[q]->NafCut_0 < 1 by {
                assert(hx.fr.stack[q] == Goal::Lit(wbody(g, sel, j)[q], dd));
            }
        }
        assert(entry_ok(cx, hx));
    }
}

// The replays of head j at two tuples of one cell end alike.
pub proof fn replay_eq(
    p: Seq<DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int|
            0 <= i < sel.ops.len() ==> ws::same_cell(
                ws::boundaries(g, sel, i),
                #[trigger] t[i],
                u[i],
            ),
        ws::tuple_pairs(g, sel, t) is Some,
        ws::tuple_pairs(g, sel, u) is Some,
    ensures
        solve(
            witness_db(tob(g, sel, t).facts, p),
            tob(g, sel, t).heads[j],
            replay_depth(),
            Seq::empty(),
            false,
            replay_inf(),
        ) == solve(
            witness_db(tob(g, sel, u).facts, p),
            tob(g, sel, u).heads[j],
            replay_depth(),
            Seq::empty(),
            false,
            replay_inf(),
        ),
{
    assert forall|i: int| 0 <= i < t.len() implies ckc_spec::arith::number(
        #[trigger] t[i],
    ) is Some by {
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    assert forall|i: int| 0 <= i < u.len() implies ckc_spec::arith::number(
        #[trigger] u[i],
    ) is Some by {
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    let cx = mk_cx(p, g, sel, j, t, u);
    let hx = mk_hx(g, sel, j, t, u);
    head_nvars(p, g, sel, j, t, u);
    cx_good(p, g, sel, j, t, u);
    chain_good(p, g, sel, j, t, u);
    hx_good(p, g, sel, j, t, u);
    let ht = tob(g, sel, t).heads[j];
    let hu = tob(g, sel, u).heads[j];
    let ct = Cfg {
        stack: seq![Goal::Lit(ht, replay_depth())],
        sol: Seq::empty(),
        alts: Seq::empty(),
        fresh: max_nat(nvars(ht), nvars_all(Seq::empty())),
        ci: 0,
        pruned: false,
        rows: Seq::empty(),
        collect: false,
    };
    let cu = Cfg {
        stack: seq![Goal::Lit(hu, replay_depth())],
        sol: Seq::empty(),
        alts: Seq::empty(),
        fresh: max_nat(nvars(hu), nvars_all(Seq::empty())),
        ci: 0,
        pruned: false,
        rows: Seq::empty(),
        collect: false,
    };
    assert(nvars_all(Seq::<Term>::empty()) == 0);
    assert(hrel(cx, ct, cu));
    assert(next_rel(cx, ct, cu));
    run_lock(cx, hx, ct, cu, replay_inf());
}

} // verus!
