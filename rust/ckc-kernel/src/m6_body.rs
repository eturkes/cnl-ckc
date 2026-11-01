#[cfg(verus_keep_ghost)]
use crate::k3_sound::naf::{lift_var, rename, rename_all};
#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use crate::m6_head::{
    hm, hm_ok, hm_state, lmap, occurs_index, occurs_member, pend, pending, rmap, sub, sub_all,
    sub_all_index, sub_item, unres, var_ok,
};
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::BodyItem;
use vstd::prelude::*;

verus! {

// Contract ws D7 (cell invariance), the head call's entry: the concrete body
// after the head unification renames to the walk's body instantiated by the
// body frame's instantiation, variable by variable of the clause.
// Two substitutions after each other, renamed against instantiated.
pub proof fn rel_sub2(
    t: Term,
    mt1: spec_fn(nat) -> Term,
    mt2: spec_fn(nat) -> Term,
    ma1: spec_fn(nat) -> Term,
    ma2: spec_fn(nat) -> Term,
    th: Seq<(nat, Term)>,
    b: nat,
    d: nat,
)
    requires
        forall|v: nat| #[trigger]
            occurs(v, t) ==> rename(sub(mt1(v), mt2), b, d) == inst(sub(ma1(v), ma2), th),
    ensures
        rename(sub(sub(t, mt1), mt2), b, d) == inst(sub(sub(t, ma1), ma2), th),
    decreases t,
{
    match t {
        Term::Var(v) => {
            assert(occurs(v, t));
        },
        Term::Comp(n, args) => {
            let ct = sub_all(sub_all(args, mt1), mt2);
            let ca = sub_all(sub_all(args, ma1), ma2);
            sub_all_index(args, mt1);
            sub_all_index(sub_all(args, mt1), mt2);
            sub_all_index(args, ma1);
            sub_all_index(sub_all(args, ma1), ma2);
            crate::k3_sound::naf::rename_all_index(ct, b, d);
            inst_all_index(ca, th);
            assert forall|i: int| 0 <= i < args.len() implies rename_all(ct, b, d)[i] == inst_all(
                ca,
                th,
            )[i] by {
                assert forall|v: nat| #[trigger] occurs(v, args[i]) implies rename(
                    sub(mt1(v), mt2),
                    b,
                    d,
                ) == inst(sub(ma1(v), ma2), th) by {
                    occurs_member(v, args, i);
                    assert(occurs(v, t));
                }
                rel_sub2(args[i], mt1, mt2, ma1, ma2, th, b, d);
            }
            assert(rename_all(ct, b, d) =~= inst_all(ca, th));
        },
        _ => {},
    }
}

// The same through a conjunction of goals.
pub proof fn rel_conj2(
    gs: Seq<Term>,
    mt1: spec_fn(nat) -> Term,
    mt2: spec_fn(nat) -> Term,
    ma1: spec_fn(nat) -> Term,
    ma2: spec_fn(nat) -> Term,
    th: Seq<(nat, Term)>,
    b: nat,
    d: nat,
)
    requires
        gs.len() > 0,
        forall|v: nat| #[trigger]
            occurs_all(v, gs) ==> rename(sub(mt1(v), mt2), b, d) == inst(sub(ma1(v), ma2), th),
    ensures
        rename(sub(conj_term(sub_all(gs, mt1)), mt2), b, d) == inst(
            sub(conj_term(sub_all(gs, ma1)), ma2),
            th,
        ),
    decreases gs.len(),
{
    sub_all_index(gs, mt1);
    sub_all_index(gs, ma1);
    assert forall|v: nat| #[trigger] occurs(v, gs[0]) implies rename(sub(mt1(v), mt2), b, d)
        == inst(sub(ma1(v), ma2), th) by {
        occurs_member(v, gs, 0);
    }
    rel_sub2(gs[0], mt1, mt2, ma1, ma2, th, b, d);
    if gs.len() > 1 {
        let rest = gs.drop_first();
        assert forall|v: nat| #[trigger] occurs_all(v, rest) implies rename(sub(mt1(v), mt2), b, d)
            == inst(sub(ma1(v), ma2), th) by {
            let k = occurs_index(v, rest);
            occurs_member(v, gs, k + 1);
        }
        rel_conj2(rest, mt1, mt2, ma1, ma2, th, b, d);
        let st = sub_all(gs, mt1);
        let sa = sub_all(gs, ma1);
        assert(st.drop_first() =~= sub_all(rest, mt1));
        assert(sa.drop_first() =~= sub_all(rest, ma1));
        let lt = seq![st[0], conj_term(st.drop_first())];
        let la = seq![sa[0], conj_term(sa.drop_first())];
        sub_all_index(lt, mt2);
        sub_all_index(la, ma2);
        crate::k3_sound::naf::rename_all_index(sub_all(lt, mt2), b, d);
        inst_all_index(sub_all(la, ma2), th);
        assert(rename_all(sub_all(lt, mt2), b, d) =~= inst_all(sub_all(la, ma2), th));
    }
}

// The item terms of the walk and of K2 (a clause item under two maps).
pub proof fn rel_item2(
    it: BodyItem,
    mt1: spec_fn(nat) -> Term,
    mt2: spec_fn(nat) -> Term,
    ma1: spec_fn(nat) -> Term,
    ma2: spec_fn(nat) -> Term,
    th: Seq<(nat, Term)>,
    b: nat,
    d: nat,
)
    requires
        it is Naf ==> it->Naf_0.len() > 0,
        forall|v: nat| #[trigger]
            item_occurs(v, it) ==> rename(sub(mt1(v), mt2), b, d) == inst(sub(ma1(v), ma2), th),
    ensures
        rename(sub(sub_item(it, mt1), mt2), b, d) == inst(sub(sub_item(it, ma1), ma2), th),
{
    match it {
        BodyItem::Pos(l) => {
            assert forall|v: nat| #[trigger] occurs(v, l) implies rename(sub(mt1(v), mt2), b, d)
                == inst(sub(ma1(v), ma2), th) by {
                assert(item_occurs(v, it));
            }
            rel_sub2(l, mt1, mt2, ma1, ma2, th, b, d);
        },
        BodyItem::Naf(gs) => {
            assert forall|v: nat| #[trigger] occurs_all(v, gs) implies rename(
                sub(mt1(v), mt2),
                b,
                d,
            ) == inst(sub(ma1(v), ma2), th) by {
                assert(item_occurs(v, it));
            }
            rel_conj2(gs, mt1, mt2, ma1, ma2, th, b, d);
            let ct = conj_term(sub_all(gs, mt1));
            let ca = conj_term(sub_all(gs, ma1));
            sub_all_index(seq![ct], mt2);
            sub_all_index(seq![ca], ma2);
            crate::k3_sound::naf::rename_all_index(sub_all(seq![ct], mt2), b, d);
            inst_all_index(sub_all(seq![ca], ma2), th);
            assert(rename_all(sub_all(seq![ct], mt2), b, d) =~= inst_all(
                sub_all(seq![ca], ma2),
                th,
            ));
        },
    }
}

pub open spec fn item_occurs(v: nat, it: BodyItem) -> bool {
    match it {
        BodyItem::Pos(l) => occurs(v, l),
        BodyItem::Naf(gs) => occurs_all(v, gs),
    }
}

// The pending map at a renamed variable: v's ground value when v pends, else
// the copy itself (r injective on the variables considered).
pub proof fn pend_at(
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    in0: spec_fn(nat) -> bool,
    v: nat,
    dom: spec_fn(nat) -> bool,
)
    requires
        dom(v),
        forall|w: nat| #[trigger] in0(w) ==> dom(w),
        forall|w1: nat, w2: nat|
            dom(w1) && dom(w2) && #[trigger] r(w1) == #[trigger] r(w2) ==> w1 == w2,
    ensures
        pend(ma, mb, r, in0)(r(v)) == if pending(ma, mb, r, in0, v) {
            ma(v)
        } else {
            Term::Var(r(v))
        },
{
    if exists|w: nat| #[trigger] pending(ma, mb, r, in0, w) && r(w) == r(v) {
        let w = choose|w: nat| #[trigger] pending(ma, mb, r, in0, w) && r(w) == r(v);
        assert(in0(w) && dom(w));
        assert(w == v);
    } else {
        assert(!pending(ma, mb, r, in0, v));
    }
}

// The renamed copy of a clause variable and the head's variables.
pub open spec fn rr(fs: Seq<nat>, off: nat) -> spec_fn(nat) -> nat {
    |v: nat| off + ckc_spec::trace::pos_of(fs, v)
}

pub open spec fn hin(ts: Seq<Term>) -> spec_fn(nat) -> bool {
    |v: nat| occurs_all(v, ts)
}

// A head meeting its clause renamed above off, the head's variables bound by
// pp where pp has a key (else open, below off): HM's outcome.
pub proof fn hm_head(ts: Seq<Term>, pp: Seq<(Term, Term)>, fs: Seq<nat>, off: nat, ys: Seq<Term>)
    requires
        crate::m6_default::vals_ground(pp),
        forall|v: nat| #[trigger] occurs_all(v, ts) ==> fs.contains(v),
        forall|v: nat|
            occurs_all(v, ts) && #[trigger] spec::lookup(pp, Term::Var(v)) is None ==> v < off,
        forall|x: nat| #[trigger] occurs_all(x, ys) ==> x >= off,
    ensures
        unify(hm_state(ts, lmap(pp), rmap(fs, off), ys)) == UOut::Ok(
            Seq::<Goal>::empty(),
            sub_all(ys, pend(lmap(pp), rmap(fs, off), rr(fs, off), hin(ts))),
        ),
{
    let ma = lmap(pp);
    let mb = rmap(fs, off);
    let r = rr(fs, off);
    let in0 = hin(ts);
    assert forall|v: nat| #[trigger] in0(v) implies var_ok(v, ma, mb, r, ys) by {
        assert(mb(v) == Term::Var(r(v)));
        match spec::lookup(pp, Term::Var(v)) {
            Option::Some(w) => {
                crate::m6_lockstep::lookup_ground(pp, Term::Var(v));
                assert(ma(v) == w);
            },
            Option::None => {
                assert(ma(v) == Term::Var(v));
                assert(v < off);
            },
        }
    }
    assert forall|v: nat, w: nat| in0(v) && in0(w) && #[trigger] r(v) == #[trigger] r(w) implies v
        == w by {
        crate::m6_ctx::pos_of_at(fs, v);
        crate::m6_ctx::pos_of_at(fs, w);
    }
    assert forall|v: nat, w: nat|
        #![trigger in0(v), in0(w)]
        in0(v) && in0(w) && ma(w) == Term::Var(w) implies r(v) != w by {
        if spec::lookup(pp, Term::Var(w)) is Some {
            crate::m6_lockstep::lookup_ground(pp, Term::Var(w));
        }
    }
    assert(hm_ok(ts, ma, mb, r, ys, in0));
    hm(ts, ma, mb, r, ys, in0);
}

// The variables of a term renamed into rmap's copies lie at or above off.
pub proof fn rmap_above(t: Term, fs: Seq<nat>, off: nat, x: nat)
    requires
        occurs(x, sub(t, rmap(fs, off))),
    ensures
        x >= off,
    decreases t,
{
    match t {
        Term::Var(v) => {},
        Term::Comp(n, args) => {
            sub_all_index(args, rmap(fs, off));
            let k = occurs_index(x, sub_all(args, rmap(fs, off)));
            rmap_above(args[k], fs, off, x);
        },
        _ => {},
    }
}

pub proof fn rmap_item_above(it: BodyItem, fs: Seq<nat>, off: nat, x: nat)
    requires
        it is Naf ==> it->Naf_0.len() > 0,
        occurs(x, sub_item(it, rmap(fs, off))),
    ensures
        x >= off,
{
    match it {
        BodyItem::Pos(l) => {
            rmap_above(l, fs, off, x);
        },
        BodyItem::Naf(gs) => {
            let ss = sub_all(gs, rmap(fs, off));
            sub_all_index(gs, rmap(fs, off));
            let one = seq![conj_term(ss)];
            reveal_with_fuel(occurs_all, 2);
            assert(occurs_all(x, one));
            assert(one[0] == conj_term(ss));
            assert(one.drop_first() =~= Seq::<Term>::empty());
            assert(occurs(x, conj_term(ss)));
            conj_vars(ss, x);
            let k = occurs_index(x, ss);
            rmap_above(gs[k], fs, off, x);
        },
    }
}

// A conjunction's variables are its goals' variables.
pub proof fn conj_vars(gs: Seq<Term>, x: nat)
    requires
        gs.len() > 0,
        occurs(x, conj_term(gs)),
    ensures
        occurs_all(x, gs),
    decreases gs.len(),
{
    if gs.len() <= 1 {
        if gs.len() == 1 {
            occurs_member(x, gs, 0);
        }
    } else {
        let rest = gs.drop_first();
        let l = seq![gs[0], conj_term(rest)];
        assert(conj_term(gs) == Term::Comp(comma_name(), l));
        let k = occurs_index(x, l);
        if k == 0 {
            occurs_member(x, gs, 0);
        } else {
            conj_vars(rest, x);
            let k2 = occurs_index(x, rest);
            occurs_member(x, gs, k2 + 1);
        }
    }
}

} // verus!
