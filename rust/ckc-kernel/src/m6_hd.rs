#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use crate::m6_ctx::*;
#[cfg(verus_keep_ghost)]
use crate::m6_default::*;
#[cfg(verus_keep_ghost)]
use crate::m6_lockstep::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
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

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};
// Contract ws D7 (cell invariance), the setup of the head call: the head's
// instantiation at a tuple (its slots and the outputs it exposes) and the
// body frame's (the same values at the exposed outputs' renamed copies).
// The open head's pairs.

pub open spec fn hpart(g: spec::Group, sel: ws::Sel) -> Seq<(Term, Term)> {
    g.pairs.filter(
        |p: (Term, Term)| !ws::counts(sel).contains(p.0) && !ws::outputs(sel).contains(p.0),
    )
}

// The outputs head j exposes: held by the open head, bound by the pairs.
pub open spec fn xo(g: spec::Group, sel: ws::Sel, j: int) -> Seq<Term> {
    ws::outputs(sel).filter(
        |o: Term| ws::mentions(ws::open_head(g, sel, j), o) && spec::lookup(g.pairs, o) is Some,
    )
}

pub open spec fn th0x(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>) -> Seq<(nat, Term)> {
    xo(g, sel, j).map_values(|o: Term| (o->Var_0, spec::lookup(tpairs(g, sel, tw), o)->0))
}

pub open spec fn th2x(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>) -> Seq<(nat, Term)> {
    xo(g, sel, j).map_values(
        |o: Term|
            (
                hoff(g, sel, j) + ckc_spec::trace::pos_of(fsj(g, j), o->Var_0),
                spec::lookup(tpairs(g, sel, tw), o)->0,
            ),
    )
}

// The head's instantiation at tw and the body frame's.
pub open spec fn th0(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>) -> Seq<(nat, Term)> {
    slot_th(sel, tw) + th0x(g, sel, j, tw)
}

pub open spec fn th2(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>) -> Seq<(nat, Term)> {
    slot_th(sel, tw) + th2x(g, sel, j, tw)
}

pub proof fn th_cat(a: Seq<(nat, Term)>, b: Seq<(nat, Term)>, x: nat)
    ensures
        th_val(a + b, x) == if th_val(a, x) is Some {
            th_val(a, x)
        } else {
            th_val(b, x)
        },
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b)[0] == a[0]);
        assert((a + b).drop_first() =~= a.drop_first() + b);
        th_cat(a.drop_first(), b, x);
    } else {
        assert(a + b =~= b);
    }
}

pub proof fn first_at(s: Seq<Term>, e: Term) -> (i: int)
    requires
        s.contains(e),
    ensures
        0 <= i < s.len(),
        s[i] == e,
        forall|j: int| 0 <= j < i ==> s[j] != e,
    decreases s.len(),
{
    if s[0] == e {
        0
    } else {
        let k = choose|k: int| 0 <= k < s.len() && s[k] == e;
        assert(s.drop_first()[k - 1] == e);
        let i = first_at(s.drop_first(), e);
        assert forall|j: int| 0 <= j < i + 1 implies s[j] != e by {
            if j > 0 {
                assert(s[j] == s.drop_first()[j - 1]);
            }
        }
        i + 1
    }
}

// The exposed outputs are variables of the outputs, bound by the pairs.
pub proof fn xo_props(g: spec::Group, sel: ws::Sel, j: int)
    requires
        ws::chain_shape(sel),
    ensures
        forall|i: int|
            0 <= i < xo(g, sel, j).len() ==> (#[trigger] xo(g, sel, j)[i]) is Var && ws::outputs(
                sel,
            ).contains(xo(g, sel, j)[i]) && ws::mentions(ws::open_head(g, sel, j), xo(g, sel, j)[i])
                && spec::lookup(g.pairs, xo(g, sel, j)[i]) is Some,
        forall|o: Term|
            ws::outputs(sel).contains(o) && ws::mentions(ws::open_head(g, sel, j), o)
                && spec::lookup(g.pairs, o) is Some ==> #[trigger] xo(g, sel, j).contains(o),
{
    let f = |o: Term| ws::mentions(ws::open_head(g, sel, j), o) && spec::lookup(g.pairs, o) is Some;
    let os = ws::outputs(sel);
    assert(xo(g, sel, j) == os.filter(f));
    assert forall|i: int| 0 <= i < xo(g, sel, j).len() implies (#[trigger] xo(g, sel, j)[i]) is Var
        && os.contains(xo(g, sel, j)[i]) && ws::mentions(ws::open_head(g, sel, j), xo(g, sel, j)[i])
        && spec::lookup(g.pairs, xo(g, sel, j)[i]) is Some by {
        os.lemma_filter_pred(f, i);
        assert(os.filter(f).contains(os.filter(f)[i]));
        os.lemma_filter_contains_rev(f, os.filter(f)[i]);
        let k = choose|k: int| 0 <= k < os.len() && os[k] == os.filter(f)[i];
        assert(os[k] is Var);
    }
    assert forall|o: Term|
        os.contains(o) && ws::mentions(ws::open_head(g, sel, j), o) && spec::lookup(
            g.pairs,
            o,
        ) is Some implies #[trigger] xo(g, sel, j).contains(o) by {
        let k = choose|k: int| 0 <= k < os.len() && os[k] == o;
        os.lemma_filter_contains(f, k);
    }
}

// The head instantiation's tail at x: x's witness value when x is an exposed
// output.
pub proof fn th0x_val(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>, x: nat)
    requires
        ws::chain_shape(sel),
        ws::tuple_pairs(g, sel, tw) is Some,
    ensures
        th_val(th0x(g, sel, j, tw), x) == if xo(g, sel, j).contains(Term::Var(x)) {
            spec::lookup(tpairs(g, sel, tw), Term::Var(x))
        } else {
            None
        },
{
    let os = xo(g, sel, j);
    let th = th0x(g, sel, j, tw);
    xo_props(g, sel, j);
    assert forall|i: int| 0 <= i < os.len() implies #[trigger] th[i] == (
        os[i]->Var_0,
        spec::lookup(tpairs(g, sel, tw), os[i])->0,
    ) by {}
    if os.contains(Term::Var(x)) {
        let i = first_at(os, Term::Var(x));
        assert forall|q: int| 0 <= q < i implies (#[trigger] th[q]).0 != th[i].0 by {
            assert(os[q] is Var && os[q] != Term::Var(x));
        }
        th_at(th, i);
        tpairs_lookup(g, sel, tw, Term::Var(x));
    } else {
        assert forall|q: int| 0 <= q < th.len() implies (#[trigger] th[q]).0 != x by {
            assert(os[q] is Var && os[q] != Term::Var(x));
        }
        th_none(th, x);
    }
}

// A variable the pairs leave unbound survives the binding.
pub proof fn bind_keeps(t: Term, p: Seq<(Term, Term)>, x: nat)
    requires
        vals_ground(p),
        occurs(x, t),
        spec::lookup(p, Term::Var(x)) is None,
    ensures
        occurs(x, spec::bind_all(t, p)),
    decreases t,
{
    match t {
        Term::Var(y) => {
            bind_var(y, p);
        },
        Term::Comp(n, args) => {
            bind_comp(n, args, p);
            let k = crate::m6_head::occurs_index(x, args);
            bind_keeps(args[k], p, x);
            let a2 = spec::bind_all(t, p)->Comp_1;
            crate::m6_head::occurs_member(x, a2, k);
        },
        _ => {},
    }
}

// th0's values are ground.
proof fn th0_ok(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>)
    requires
        ws::chain_shape(sel),
        forall|k: int| 0 <= k < g.pairs.len() ==> ground((#[trigger] g.pairs[k]).1),
        tw.len() == sel.ops.len(),
        forall|i: int| 0 <= i < tw.len() ==> ckc_spec::arith::number(#[trigger] tw[i]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
    ensures
        th_ok(th0(g, sel, j, tw)),
{
    let tp = tpairs(g, sel, tw);
    let th = th0(g, sel, j, tw);
    let st = slot_th(sel, tw);
    let tx = th0x(g, sel, j, tw);
    tvals_facts(g, sel, tw);
    tpairs_ground(g, sel, tw);
    xo_props(g, sel, j);
    assert forall|k: int| 0 <= k < th.len() implies ground((#[trigger] th[k]).1) by {
        if k < st.len() {
            assert(th[k] == st[k]);
            crate::k3_sound::log::number_ground(tw[k]);
        } else {
            let o = xo(g, sel, j)[k - st.len()];
            assert(th[k] == tx[k - st.len()]);
            tpairs_lookup(g, sel, tw, o);
            lookup_ground(tp, o);
        }
    }
}

// th0 binds below the head offset.
proof fn th0_dom(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>, x: nat)
    requires
        ws::chain_shape(sel),
        ws::fixed_skeleton(g, sel),
        tw.len() == sel.ops.len(),
        ws::tuple_pairs(g, sel, tw) is Some,
        th_val(th0(g, sel, j, tw), x) is Some,
    ensures
        x < hoff(g, sel, j),
{
    let h = ws::open_head(g, sel, j);
    let st = slot_th(sel, tw);
    let tx = th0x(g, sel, j, tw);
    let cs = ws::counts(sel);
    assert(cs.no_duplicates());
    xo_props(g, sel, j);
    th_cat(st, tx, x);
    slot_val(sel, tw, x);
    th0x_val(g, sel, j, tw, x);
    if th_val(st, x) is Some {
        let i = choose|i: int| 0 <= i < cs.len() && cs[i] == Term::Var(x);
        assert(cs[i] == sel.ops[i].n);
        count_below(g, sel, i);
    } else {
        assert(xo(g, sel, j).contains(Term::Var(x)));
        let k = choose|k: int| 0 <= k < xo(g, sel, j).len() && xo(g, sel, j)[k] == Term::Var(x);
        assert(ws::mentions(h, xo(g, sel, j)[k]));
        occurs_lt(h, x);
    }
}

// A head variable: th0 binds it to its witness value exactly when the open
// head leaves it open; otherwise the witness and open pairs agree on it.
proof fn th0_var(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>, x: nat)
    requires
        ws::chain_shape(sel),
        ws::exact_comparators(g, sel),
        ws::counts(sel).no_duplicates(),
        forall|k: int| 0 <= k < g.pairs.len() ==> ground((#[trigger] g.pairs[k]).1),
        tw.len() == sel.ops.len(),
        forall|i: int| 0 <= i < tw.len() ==> ckc_spec::arith::number(#[trigger] tw[i]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
        forall|i: int|
            0 <= i < ws::counts(sel).len() ==> !ws::mentions(
                ws::open_head(g, sel, j),
                #[trigger] ws::counts(sel)[i],
            ),
        occurs(x, g.clauses[j].head),
        ws::open_head(g, sel, j) == spec::bind_all(g.clauses[j].head, hpart(g, sel)),
    ensures
        ({
            let th = th0(g, sel, j, tw);
            let tp = tpairs(g, sel, tw);
            let hp = hpart(g, sel);
            &&& th_val(th, x) is Some ==> spec::lookup(tp, Term::Var(x)) == th_val(th, x)
                && spec::lookup(hp, Term::Var(x)) is None
            &&& th_val(th, x) is None ==> spec::lookup(tp, Term::Var(x)) == spec::lookup(
                hp,
                Term::Var(x),
            )
        }),
{
    let h = ws::open_head(g, sel, j);
    let head = g.clauses[j].head;
    let tp = tpairs(g, sel, tw);
    let hp = hpart(g, sel);
    let th = th0(g, sel, j, tw);
    let st = slot_th(sel, tw);
    let tx = th0x(g, sel, j, tw);
    let cs = ws::counts(sel);
    let fh = |q: (Term, Term)| !ws::counts(sel).contains(q.0) && !ws::outputs(sel).contains(q.0);
    assert(hp == g.pairs.filter(fh));
    assert(cs.no_duplicates());
    tvals_facts(g, sel, tw);
    assert(vals_ground(g.pairs));
    ground_filter(g.pairs, fh);
    xo_props(g, sel, j);
    keys_index(sel);
    let vs = tvals(sel, tw);
    let nops = sel.ops.len() as int;
    th_cat(st, tx, x);
    slot_val(sel, tw, x);
    th0x_val(g, sel, j, tw, x);
    tpairs_lookup(g, sel, tw, Term::Var(x));
    if cs.contains(Term::Var(x)) {
        let i = choose|i: int| 0 <= i < cs.len() && cs[i] == Term::Var(x);
        lookup_drop(g.pairs, fh, Term::Var(x));
        bind_keeps(head, hp, x);
        assert(ws::mentions(h, cs[i]));
    }
    assert(th_val(st, x) is None);
    if ws::outputs(sel).contains(Term::Var(x)) {
        lookup_drop(g.pairs, fh, Term::Var(x));
        bind_keeps(head, hp, x);
        assert(ws::mentions(h, Term::Var(x)));
    } else {
        lookup_keep(g.pairs, fh, Term::Var(x));
        if xo(g, sel, j).contains(Term::Var(x)) {
            let k = choose|k: int| 0 <= k < xo(g, sel, j).len() && xo(g, sel, j)[k] == Term::Var(x);
            assert(ws::outputs(sel).contains(xo(g, sel, j)[k]));
        }
        if spec::lookup(vs, Term::Var(x)) is Some && spec::lookup(g.pairs, Term::Var(x)) is Some {
            assert(keys(sel).contains(Term::Var(x)));
            let q = choose|q: int| 0 <= q < keys(sel).len() && keys(sel)[q] == Term::Var(x);
            if q < nops {
                assert(sel.ops[q].s == Term::Var(x));
            } else if q < 2 * nops {
                assert(cs[q - nops] == Term::Var(x));
            } else {
                assert(ws::outputs(sel)[q - 2 * nops] == Term::Var(x));
            }
        }
    }
}

// The head at tw = the open head under th0, whose variables lie below the
// walk's head offset; th0 agrees with the slots on the skeleton.
pub proof fn head_facts(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    tw: Seq<Term>,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        tw.len() == sel.ops.len(),
        forall|i: int| 0 <= i < tw.len() ==> ckc_spec::arith::number(#[trigger] tw[i]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
    ensures
        th_ok(th0(g, sel, j, tw)),
        forall|x: nat| #[trigger] th_val(th0(g, sel, j, tw), x) is Some ==> x < hoff(g, sel, j),
        tob(g, sel, tw).heads[j] == inst(ws::open_head(g, sel, j), th0(g, sel, j, tw)),
        forall|k: int|
            0 <= k < ws::skeleton(g, sel).len() ==> inst(
                #[trigger] ws::skeleton(g, sel)[k],
                th0(g, sel, j, tw),
            ) == inst(ws::skeleton(g, sel)[k], slot_th(sel, tw)),
{
    let h = ws::open_head(g, sel, j);
    let head = g.clauses[j].head;
    let tp = tpairs(g, sel, tw);
    let hp = hpart(g, sel);
    let th = th0(g, sel, j, tw);
    let st = slot_th(sel, tw);
    let tx = th0x(g, sel, j, tw);
    let sk = ws::skeleton(g, sel);
    let fh = |q: (Term, Term)| !ws::counts(sel).contains(q.0) && !ws::outputs(sel).contains(q.0);
    assert(hp == g.pairs.filter(fh));
    assert(h == spec::bind_all(head, hp));
    assert(tob(g, sel, tw).heads[j] == spec::bind_all(head, tp));
    assert(ws::clause_ok(p, g, sel, j));
    let e = wenv(p, g, sel, j);
    assert(ws::slot_free(e, h));
    assert forall|i: int| 0 <= i < ws::counts(sel).len() implies !ws::mentions(
        h,
        #[trigger] ws::counts(sel)[i],
    ) by {
        assert(e.slots[i] == ws::counts(sel)[i]);
    }
    assert(ws::counts(sel).no_duplicates());
    th0_ok(g, sel, j, tw);
    assert forall|x: nat| #[trigger] th_val(th, x) is Some implies x < hoff(g, sel, j) by {
        th0_dom(g, sel, j, tw, x);
    }
    tvals_facts(g, sel, tw);
    tpairs_ground(g, sel, tw);
    assert(vals_ground(g.pairs));
    ground_filter(g.pairs, fh);
    assert forall|x: nat| #[trigger] occurs(x, head) implies (th_val(th, x) is Some
        ==> spec::lookup(tp, Term::Var(x)) == th_val(th, x) && spec::lookup(
        hp,
        Term::Var(x),
    ) is None) && (th_val(th, x) is None ==> spec::lookup(tp, Term::Var(x)) == spec::lookup(
        hp,
        Term::Var(x),
    )) by {
        th0_var(g, sel, j, tw, x);
    }
    bind_open(head, tp, hp, th);
    assert forall|k: int| 0 <= k < sk.len() implies inst(#[trigger] sk[k], th) == inst(
        sk[k],
        st,
    ) by {
        assert forall|x: nat| #[trigger] occurs(x, sk[k]) implies th_val(th, x) == th_val(
            st,
            x,
        ) by {
            skel_slot(g, sel, k, x);
            slot_val(sel, tw, x);
            th_cat(st, tx, x);
        }
        crate::m6_step::inst_agree(sk[k], th, st);
    }
}

// --- the head's variable bound ---
pub proof fn ground_nvars(t: Term)
    requires
        ground(t),
    ensures
        nvars(t) == 0,
    decreases t,
{
    if let Term::Comp(n, args) = t {
        assert forall|i: int| 0 <= i < args.len() implies nvars(#[trigger] args[i]) <= 0 by {
            ground_elem(args, i);
            ground_nvars(args[i]);
        }
        crate::k3_sound::naf::nvars_all_upper(args, 0);
    }
}

// Instantiation removes variables; two instantiations binding the same
// variables of t leave the same bound.
pub proof fn inst_nvars(t: Term, th1: Seq<(nat, Term)>, th2: Seq<(nat, Term)>)
    requires
        th_ok(th1),
        th_ok(th2),
        forall|x: nat| #[trigger]
            occurs(x, t) ==> (th_val(th1, x) is Some) == (th_val(th2, x) is Some),
    ensures
        nvars(inst(t, th1)) <= nvars(t),
        nvars(inst(t, th1)) == nvars(inst(t, th2)),
    decreases t,
{
    match t {
        Term::Var(x) => {
            assert(occurs(x, t));
            th_val_number(th1, x);
            th_val_number(th2, x);
            if let Some(w) = th_val(th1, x) {
                ground_nvars(w);
            }
            if let Some(w) = th_val(th2, x) {
                ground_nvars(w);
            }
        },
        Term::Comp(n, args) => {
            let a1 = inst_all(args, th1);
            let a2 = inst_all(args, th2);
            inst_all_index(args, th1);
            inst_all_index(args, th2);
            assert forall|i: int| 0 <= i < args.len() implies nvars(#[trigger] a1[i]) <= nvars(
                args[i],
            ) && nvars(a1[i]) == nvars(a2[i]) by {
                assert forall|x: nat| #[trigger] occurs(x, args[i]) implies (th_val(th1, x) is Some)
                    == (th_val(th2, x) is Some) by {
                    crate::m6_head::occurs_member(x, args, i);
                    assert(occurs(x, t));
                }
                inst_nvars(args[i], th1, th2);
            }
            assert forall|i: int| 0 <= i < a1.len() implies nvars(#[trigger] a1[i]) <= nvars_all(
                args,
            ) by {
                crate::k3_sound::naf::nvars_all_index(args, i);
            }
            crate::k3_sound::naf::nvars_all_upper(a1, nvars_all(args));
            assert forall|i: int| 0 <= i < a1.len() implies nvars(#[trigger] a1[i]) <= nvars_all(
                a2,
            ) by {
                crate::k3_sound::naf::nvars_all_index(a2, i);
            }
            crate::k3_sound::naf::nvars_all_upper(a1, nvars_all(a2));
            assert forall|i: int| 0 <= i < a2.len() implies nvars(#[trigger] a2[i]) <= nvars_all(
                a1,
            ) by {
                crate::k3_sound::naf::nvars_all_index(a1, i);
            }
            crate::k3_sound::naf::nvars_all_upper(a2, nvars_all(a1));
        },
        _ => {},
    }
}

// The heads at t and at u share their variable bound, at most the open
// head's.
pub proof fn head_nvars(
    p: Seq<ckc_spec::v1text::DocClause>,
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
        nvars(tob(g, sel, t).heads[j]) == nvars(tob(g, sel, u).heads[j]),
        nvars(tob(g, sel, t).heads[j]) <= hoff(g, sel, j),
{
    let h = ws::open_head(g, sel, j);
    head_facts(p, g, sel, j, t);
    head_facts(p, g, sel, j, u);
    assert(ws::counts(sel).no_duplicates());
    xo_props(g, sel, j);
    assert forall|x: nat| #[trigger] occurs(x, h) implies (th_val(th0(g, sel, j, t), x) is Some)
        == (th_val(th0(g, sel, j, u), x) is Some) by {
        th_cat(slot_th(sel, t), th0x(g, sel, j, t), x);
        th_cat(slot_th(sel, u), th0x(g, sel, j, u), x);
        slot_val(sel, t, x);
        slot_val(sel, u, x);
        th0x_val(g, sel, j, t, x);
        th0x_val(g, sel, j, u, x);
        tpairs_lookup(g, sel, t, Term::Var(x));
        tpairs_lookup(g, sel, u, Term::Var(x));
        if xo(g, sel, j).contains(Term::Var(x)) {
            let k = choose|k: int| 0 <= k < xo(g, sel, j).len() && xo(g, sel, j)[k] == Term::Var(x);
            assert(spec::lookup(g.pairs, xo(g, sel, j)[k]) is Some);
        }
    }
    inst_nvars(h, th0(g, sel, j, t), th0(g, sel, j, u));
}

} // verus!
