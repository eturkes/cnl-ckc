#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
use vstd::prelude::*;

verus! {

// Contract ws D7: the head meeting its own clause. A head h = the clause head
// with some variables bound to ground values and the others open, unified
// with the clause head renamed apart: each open variable joins its renamed
// copy, each ground-bound one hands its value to its renamed copy; nothing
// else moves.
// Replace each variable v of t by m(v).
pub open spec fn sub(t: Term, m: spec_fn(nat) -> Term) -> Term
    decreases t,
{
    match t {
        Term::Var(v) => m(v),
        Term::Comp(n, args) => Term::Comp(n, sub_all(args, m)),
        _ => t,
    }
}

pub open spec fn sub_all(ts: Seq<Term>, m: spec_fn(nat) -> Term) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        seq![sub(ts[0], m)] + sub_all(ts.drop_first(), m)
    }
}

pub proof fn sub_all_index(ts: Seq<Term>, m: spec_fn(nat) -> Term)
    ensures
        sub_all(ts, m).len() == ts.len(),
        forall|i: int| 0 <= i < ts.len() ==> #[trigger] sub_all(ts, m)[i] == sub(ts[i], m),
    decreases ts.len(),
{
    if ts.len() > 0 {
        sub_all_index(ts.drop_first(), m);
        assert forall|i: int| 0 <= i < ts.len() implies #[trigger] sub_all(ts, m)[i] == sub(
            ts[i],
            m,
        ) by {
            if i > 0 {
                assert(ts.drop_first()[i - 1] == ts[i]);
            }
        }
    }
}

// sub agrees with m' wherever m does on t's variables.
pub proof fn sub_ext(t: Term, m: spec_fn(nat) -> Term, m2: spec_fn(nat) -> Term)
    requires
        forall|v: nat| #[trigger] occurs(v, t) ==> m(v) == m2(v),
    ensures
        sub(t, m) == sub(t, m2),
    decreases t,
{
    match t {
        Term::Var(v) => {
            assert(occurs(v, t));
        },
        Term::Comp(n, args) => {
            assert forall|v: nat| #[trigger] occurs_all(v, args) implies m(v) == m2(v) by {
                assert(occurs(v, t));
            }
            sub_all_ext(args, m, m2);
        },
        _ => {},
    }
}

pub proof fn sub_all_ext(ts: Seq<Term>, m: spec_fn(nat) -> Term, m2: spec_fn(nat) -> Term)
    requires
        forall|v: nat| #[trigger] occurs_all(v, ts) ==> m(v) == m2(v),
    ensures
        sub_all(ts, m) == sub_all(ts, m2),
    decreases ts,
{
    if ts.len() > 0 {
        assert forall|v: nat| #[trigger] occurs(v, ts[0]) implies m(v) == m2(v) by {
            assert(occurs_all(v, ts));
        }
        sub_ext(ts[0], m, m2);
        assert forall|v: nat| #[trigger] occurs_all(v, ts.drop_first()) implies m(v) == m2(v) by {
            assert(occurs_all(v, ts));
        }
        sub_all_ext(ts.drop_first(), m, m2);
    }
}

// Binding x to w after m = m with x's image bound, when only m(x)... in general:
// subst(sub(t, m), x, w) = sub(t, |v| subst(m(v), x, w)).
pub proof fn subst_sub(t: Term, m: spec_fn(nat) -> Term, x: nat, w: Term)
    ensures
        subst(sub(t, m), x, w) == sub(t, |v: nat| subst(m(v), x, w)),
    decreases t,
{
    match t {
        Term::Comp(n, args) => {
            subst_sub_all(args, m, x, w);
        },
        _ => {},
    }
}

pub proof fn subst_sub_all(ts: Seq<Term>, m: spec_fn(nat) -> Term, x: nat, w: Term)
    ensures
        subst_all(sub_all(ts, m), x, w) == sub_all(ts, |v: nat| subst(m(v), x, w)),
    decreases ts,
{
    if ts.len() > 0 {
        subst_sub(ts[0], m, x, w);
        subst_sub_all(ts.drop_first(), m, x, w);
        let s = sub_all(ts, m);
        assert(s == seq![sub(ts[0], m)] + sub_all(ts.drop_first(), m));
        assert(s[0] == sub(ts[0], m));
        assert(s.drop_first() =~= sub_all(ts.drop_first(), m));
    } else {
        assert(sub_all(ts, m) =~= Seq::<Term>::empty());
    }
}

// The identity map leaves terms alone.
pub proof fn sub_id(t: Term)
    ensures
        sub(t, |v: nat| Term::Var(v)) == t,
    decreases t,
{
    if let Term::Comp(n, args) = t {
        sub_all_id(args);
    }
}

pub proof fn sub_all_id(ts: Seq<Term>)
    ensures
        sub_all(ts, |v: nat| Term::Var(v)) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        sub_id(ts[0]);
        sub_all_id(ts.drop_first());
        assert(seq![ts[0]] + ts.drop_first() =~= ts);
    }
}

// sub(sub(t, m1), m2) = sub(t, m1 then m2).
pub proof fn sub_compose(t: Term, m1: spec_fn(nat) -> Term, m2: spec_fn(nat) -> Term)
    ensures
        sub(sub(t, m1), m2) == sub(t, |v: nat| sub(m1(v), m2)),
    decreases t,
{
    if let Term::Comp(n, args) = t {
        sub_all_compose(args, m1, m2);
    }
}

pub proof fn sub_all_compose(ts: Seq<Term>, m1: spec_fn(nat) -> Term, m2: spec_fn(nat) -> Term)
    ensures
        sub_all(sub_all(ts, m1), m2) == sub_all(ts, |v: nat| sub(m1(v), m2)),
    decreases ts,
{
    if ts.len() > 0 {
        sub_compose(ts[0], m1, m2);
        sub_all_compose(ts.drop_first(), m1, m2);
        let s = sub_all(ts, m1);
        assert(s[0] == sub(ts[0], m1));
        assert(s.drop_first() =~= sub_all(ts.drop_first(), m1));
    } else {
        assert(sub_all(ts, m1) =~= Seq::<Term>::empty());
    }
}

// subst as a map substitution.
pub proof fn subst_as_sub(t: Term, x: nat, w: Term)
    ensures
        subst(t, x, w) == sub(
            t,
            |z: nat|
                if z == x {
                    w
                } else {
                    Term::Var(z)
                },
        ),
    decreases t,
{
    if let Term::Comp(n, args) = t {
        subst_as_sub_all(args, x, w);
    }
}

pub proof fn subst_as_sub_all(ts: Seq<Term>, x: nat, w: Term)
    ensures
        subst_all(ts, x, w) == sub_all(
            ts,
            |z: nat|
                if z == x {
                    w
                } else {
                    Term::Var(z)
                },
        ),
    decreases ts,
{
    if ts.len() > 0 {
        subst_as_sub(ts[0], x, w);
        subst_as_sub_all(ts.drop_first(), x, w);
    }
}

pub proof fn sub_ground(t: Term, m: spec_fn(nat) -> Term)
    requires
        ground(t),
    ensures
        sub(t, m) == t,
    decreases t,
{
    if let Term::Comp(n, args) = t {
        sub_all_ground(args, m);
    }
}

pub proof fn sub_all_ground(ts: Seq<Term>, m: spec_fn(nat) -> Term)
    requires
        ground_all(ts),
    ensures
        sub_all(ts, m) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        sub_ground(ts[0], m);
        sub_all_ground(ts.drop_first(), m);
        assert(seq![ts[0]] + ts.drop_first() =~= ts);
    }
}

// --- the head matching its renamed clause (HM) ---
// A template variable v (renamed copy r(v)) is unresolved (left ground or v
// itself, right its copy) or resolved (both sides equal: the ground value or
// the copy).
pub open spec fn var_ok(
    v: nat,
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    ys: Seq<Term>,
) -> bool {
    ||| ground(ma(v)) && mb(v) == Term::Var(r(v))
    ||| ma(v) == Term::Var(v) && mb(v) == Term::Var(r(v)) && !occurs_all(v, ys)
    ||| ground(ma(v)) && mb(v) == ma(v)
    ||| ma(v) == Term::Var(r(v)) && mb(v) == Term::Var(r(v))
}

pub open spec fn hm_ok(
    ts: Seq<Term>,
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    ys: Seq<Term>,
    in0: spec_fn(nat) -> bool,
) -> bool {
    &&& forall|v: nat| #[trigger] occurs_all(v, ts) ==> in0(v)
    &&& forall|v: nat| #[trigger] in0(v) ==> var_ok(v, ma, mb, r, ys)
    &&& forall|v: nat, w: nat| in0(v) && in0(w) && #[trigger] r(v) == #[trigger] r(w) ==> v == w
    &&& forall|v: nat, w: nat|
        #![trigger in0(v), in0(w)]
        in0(v) && in0(w) && ma(w) == Term::Var(w) ==> r(v) != w
    &&& forall|v: nat| #[trigger] unres(ma, mb, r, in0, v) ==> occurs_all(v, ts)
}

pub open spec fn pending(
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    in0: spec_fn(nat) -> bool,
    v: nat,
) -> bool {
    in0(v) && ground(ma(v)) && mb(v) == Term::Var(r(v))
}

// The ground values still to reach their renamed copies.
pub open spec fn pend(
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    in0: spec_fn(nat) -> bool,
) -> spec_fn(nat) -> Term {
    |z: nat|
        if exists|v: nat| #[trigger] pending(ma, mb, r, in0, v) && r(v) == z {
            ma(choose|v: nat| #[trigger] pending(ma, mb, r, in0, v) && r(v) == z)
        } else {
            Term::Var(z)
        }
}

pub open spec fn hm_state(
    ts: Seq<Term>,
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    ys: Seq<Term>,
) -> UState {
    UState { pairs: zip(sub_all(ts, ma), sub_all(ts, mb)), stack: Seq::empty(), sol: ys }
}

pub proof fn hm(
    ts: Seq<Term>,
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    ys: Seq<Term>,
    in0: spec_fn(nat) -> bool,
)
    requires
        hm_ok(ts, ma, mb, r, ys, in0),
    ensures
        unify(hm_state(ts, ma, mb, ys)) == UOut::Ok(
            Seq::<Goal>::empty(),
            sub_all(ys, pend(ma, mb, r, in0)),
        ),
    decreases crate::k2_engine::terms_size(ts),
{
    let u = hm_state(ts, ma, mb, ys);
    sub_all_index(ts, ma);
    sub_all_index(ts, mb);
    if ts.len() == 0 {
        assert(u.pairs.len() == 0);
        reveal_with_fuel(unify_n, 1);
        crate::k3_sound::naf::unify_at(u, 1);
        assert forall|v: nat| !#[trigger] pending(ma, mb, r, in0, v) by {
            if pending(ma, mb, r, in0, v) {
                assert(unres(ma, mb, r, in0, v));
            }
        }
        hm_done(ys, ma, mb, r, in0);
        return;
    }
    let t0 = ts[0];
    let rest = ts.drop_first();
    assert(u.pairs[0] == (sub(t0, ma), sub(t0, mb)));
    assert(u.pairs.drop_first() =~= zip(sub_all(rest, ma), sub_all(rest, mb))) by {
        sub_all_index(rest, ma);
        sub_all_index(rest, mb);
    }
    assert forall|v: nat| #[trigger] occurs_all(v, rest) implies in0(v) by {
        assert(occurs_all(v, ts));
    }
    assert(crate::k2_engine::terms_size(rest) < crate::k2_engine::terms_size(ts)) by {
        reveal_with_fuel(crate::k2_engine::terms_size, 1);
        crate::k2_engine::term_size_positive(t0);
    }
    match t0 {
        Term::Var(v) => {
            assert(occurs(v, ts[0]));
            occurs_member(v, ts, 0);
            assert(in0(v));
            assert(var_ok(v, ma, mb, r, ys));
            if ground(ma(v)) && mb(v) == Term::Var(r(v)) {
                // the ground value moves to the copy
                let g = ma(v);
                crate::m6_default::ground_absent(g, r(v));
                assert(!(g is Var));
                crate::k2_engine::unify_bind_step(u, r(v), g);
                let mb2 = |w: nat|
                    if w == v {
                        g
                    } else {
                        mb(w)
                    };
                let ys2 = subst_all(ys, r(v), g);
                hm_bind_ground(ts, ma, mb, r, ys, in0, v);
                hm(rest, ma, mb2, r, ys2, in0);
            } else if ma(v) == Term::Var(v) && mb(v) == Term::Var(r(v)) && !occurs_all(v, ys) {
                // the open variable joins its copy
                assert(r(v) != v);
                crate::k2_engine::unify_bind_step(u, v, Term::Var(r(v)));
                let ma2 = |w: nat|
                    if w == v {
                        Term::Var(r(v))
                    } else {
                        ma(w)
                    };
                hm_bind_open(ts, ma, mb, r, ys, in0, v);
                hm(rest, ma2, mb, r, ys, in0);
            } else if ground(ma(v)) && mb(v) == ma(v) {
                hm_skip(u);
                hm_keep(ts, ma, mb, r, ys, in0, v);
                hm(rest, ma, mb, r, ys, in0);
            } else {
                crate::k2_engine::unify_drop_step(u);
                hm_keep(ts, ma, mb, r, ys, in0, v);
                hm(rest, ma, mb, r, ys, in0);
            }
        },
        Term::Comp(n, args) => {
            sub_all_index(args, ma);
            sub_all_index(args, mb);
            crate::k2_engine::unify_comp_step(u, n, sub_all(args, ma), sub_all(args, mb));
            let ts2 = args + rest;
            assert(zip(sub_all(args, ma), sub_all(args, mb)) + u.pairs.drop_first() =~= zip(
                sub_all(ts2, ma),
                sub_all(ts2, mb),
            )) by {
                sub_all_index(ts2, ma);
                sub_all_index(ts2, mb);
            }
            assert forall|v: nat| #[trigger] occurs_all(v, ts2) implies in0(v) by {
                hm_occurs_split(v, args, rest, ts);
            }
            assert forall|w: nat| #[trigger] unres(ma, mb, r, in0, w) implies occurs_all(
                w,
                ts2,
            ) by {
                assert(occurs_all(w, ts));
                let k = occurs_index(w, ts);
                if k == 0 {
                    let j = occurs_index(w, args);
                    occurs_member(w, ts2, j);
                } else {
                    occurs_member(w, ts2, args.len() + k - 1);
                }
            }
            assert(crate::k2_engine::terms_size(ts2) < crate::k2_engine::terms_size(ts)) by {
                assert(ts =~= seq![Term::Comp(n, args)] + rest);
                hm_size_split(args, rest, n);
            }
            hm(ts2, ma, mb, r, ys, in0);
        },
        _ => {
            crate::k2_engine::unify_drop_step(u);
            assert forall|w: nat| #[trigger] unres(ma, mb, r, in0, w) implies occurs_all(
                w,
                rest,
            ) by {
                assert(occurs_all(w, ts));
                let k = occurs_index(w, ts);
                assert(k != 0);
                occurs_member(w, rest, k - 1);
            }
            hm(rest, ma, mb, r, ys, in0);
        },
    }
}

// A resolved variable's template leaves the invariant to the rest.
proof fn hm_keep(
    ts: Seq<Term>,
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    ys: Seq<Term>,
    in0: spec_fn(nat) -> bool,
    v: nat,
)
    requires
        hm_ok(ts, ma, mb, r, ys, in0),
        ts.len() > 0,
        ts[0] == Term::Var(v),
        !unres(ma, mb, r, in0, v),
    ensures
        hm_ok(ts.drop_first(), ma, mb, r, ys, in0),
{
    let rest = ts.drop_first();
    assert forall|w: nat| #[trigger] occurs_all(w, rest) implies in0(w) by {
        let k = occurs_index(w, rest);
        occurs_member(w, ts, k + 1);
    }
    assert forall|w: nat| #[trigger] unres(ma, mb, r, in0, w) implies occurs_all(w, rest) by {
        assert(occurs_all(w, ts));
        let k = occurs_index(w, ts);
        assert(k != 0);
        occurs_member(w, rest, k - 1);
    }
}

proof fn hm_occurs_split(v: nat, args: Seq<Term>, rest: Seq<Term>, ts: Seq<Term>)
    requires
        ts.len() > 0,
        ts[0] is Comp,
        ts[0]->Comp_1 == args,
        rest == ts.drop_first(),
        occurs_all(v, args + rest),
    ensures
        occurs_all(v, ts),
{
    let k = occurs_index(v, args + rest);
    if k < args.len() {
        occurs_member(v, args, k);
        assert(occurs(v, ts[0]));
    } else {
        assert((args + rest)[k] == rest[k - args.len()]);
        occurs_member(v, rest, k - args.len());
        occurs_member(v, ts, k - args.len() + 1);
        assert(ts.drop_first() == rest);
    }
}

proof fn terms_size_concat(a: Seq<Term>, b: Seq<Term>)
    ensures
        crate::k2_engine::terms_size(a + b) == crate::k2_engine::terms_size(a)
            + crate::k2_engine::terms_size(b),
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b).drop_first() =~= a.drop_first() + b);
        terms_size_concat(a.drop_first(), b);
    } else {
        assert(a + b =~= b);
    }
}

proof fn hm_size_split(args: Seq<Term>, rest: Seq<Term>, n: Seq<u8>)
    ensures
        crate::k2_engine::terms_size(args + rest) < crate::k2_engine::terms_size(
            seq![Term::Comp(n, args)] + rest,
        ),
{
    terms_size_concat(args, rest);
    let ts = seq![Term::Comp(n, args)] + rest;
    assert(ts.drop_first() =~= rest);
    assert(ts[0] == Term::Comp(n, args));
    assert(crate::k2_engine::terms_size(ts) == crate::k2_engine::term_size(ts[0])
        + crate::k2_engine::terms_size(ts.drop_first()));
    assert(crate::k2_engine::term_size(Term::Comp(n, args)) == 1 + crate::k2_engine::terms_size(
        args,
    ));
}

// An equal ground pair is skipped.
proof fn hm_skip(u: UState)
    requires
        u.pairs.len() > 0,
        u.pairs[0].0 == u.pairs[0].1,
        ground(u.pairs[0].0),
    ensures
        unify(u) == unify(UState { pairs: u.pairs.drop_first(), ..u }),
{
    let d = UState { pairs: u.pairs.drop_first(), ..u };
    crate::k2_engine::unify_done(d);
    reveal(unify);
    let f = choose|f: nat| !(unify_n(d, f) is Out);
    crate::k3_sound::naf::unify_at(d, f);
    crate::m6_cell::skip_number(u.pairs[0].0, u, f);
    let h = choose|h: nat| #[trigger] unify_n(u, h) == unify_n(d, f);
    crate::k3_sound::naf::unify_at(u, h);
}

// At the end every template variable is resolved: nothing pends.
proof fn hm_done(
    ys: Seq<Term>,
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    in0: spec_fn(nat) -> bool,
)
    requires
        forall|v: nat| !#[trigger] pending(ma, mb, r, in0, v),
    ensures
        sub_all(ys, pend(ma, mb, r, in0)) == ys,
{
    sub_all_ext(ys, pend(ma, mb, r, in0), |v: nat| Term::Var(v));
    sub_all_id(ys);
}

pub proof fn occurs_index(v: nat, ts: Seq<Term>) -> (k: int)
    requires
        occurs_all(v, ts),
    ensures
        0 <= k < ts.len(),
        occurs(v, ts[k]),
    decreases ts.len(),
{
    if occurs(v, ts[0]) {
        0
    } else {
        let k = occurs_index(v, ts.drop_first());
        k + 1
    }
}

pub proof fn occurs_member(v: nat, ts: Seq<Term>, k: int)
    requires
        0 <= k < ts.len(),
        occurs(v, ts[k]),
    ensures
        occurs_all(v, ts),
    decreases ts.len(),
{
    if k > 0 {
        occurs_member(v, ts.drop_first(), k - 1);
    }
}

pub open spec fn unres(
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    in0: spec_fn(nat) -> bool,
    v: nat,
) -> bool {
    in0(v) && mb(v) == Term::Var(r(v)) && (ground(ma(v)) || ma(v) == Term::Var(v))
}

// The bound pairs as a pair map = the template pairs under the bound maps.
proof fn bind_pairs(
    ts: Seq<Term>,
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    x: nat,
    w: Term,
    ma2: spec_fn(nat) -> Term,
    mb2: spec_fn(nat) -> Term,
)
    requires
        forall|v: nat| #[trigger]
            occurs_all(v, ts) ==> subst(ma(v), x, w) == ma2(v) && subst(mb(v), x, w) == mb2(v),
    ensures
        zip(sub_all(ts, ma), sub_all(ts, mb)).map_values(
            |p: (Term, Term)| (subst(p.0, x, w), subst(p.1, x, w)),
        ) == zip(sub_all(ts, ma2), sub_all(ts, mb2)),
{
    let sa = |v: nat| subst(ma(v), x, w);
    let sb = |v: nat| subst(mb(v), x, w);
    subst_sub_all(ts, ma, x, w);
    subst_sub_all(ts, mb, x, w);
    sub_all_ext(ts, sa, ma2);
    sub_all_ext(ts, sb, mb2);
    sub_all_index(ts, ma);
    sub_all_index(ts, mb);
    sub_all_index(ts, ma2);
    sub_all_index(ts, mb2);
    crate::k3_sound::naf::subst_all_index(sub_all(ts, ma), x, w);
    crate::k3_sound::naf::subst_all_index(sub_all(ts, mb), x, w);
    assert(zip(sub_all(ts, ma), sub_all(ts, mb)).map_values(
        |p: (Term, Term)| (subst(p.0, x, w), subst(p.1, x, w)),
    ) =~= zip(sub_all(ts, ma2), sub_all(ts, mb2))) by {
        assert forall|i: int| 0 <= i < ts.len() implies zip(
            sub_all(ts, ma),
            sub_all(ts, mb),
        ).map_values(|p: (Term, Term)| (subst(p.0, x, w), subst(p.1, x, w)))[i] == zip(
            sub_all(ts, ma2),
            sub_all(ts, mb2),
        )[i] by {
            assert(subst_all(sub_all(ts, ma), x, w)[i] == subst(sub_all(ts, ma)[i], x, w));
            assert(subst_all(sub_all(ts, mb), x, w)[i] == subst(sub_all(ts, mb)[i], x, w));
        }
    }
}

// The pending maps agree when the pending predicates agree off v and v's
// copy maps to the same value.
proof fn pend_agree(
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    ma2: spec_fn(nat) -> Term,
    mb2: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    in0: spec_fn(nat) -> bool,
    z: nat,
)
    requires
        forall|v: nat, w: nat| in0(v) && in0(w) && #[trigger] r(v) == #[trigger] r(w) ==> v == w,
        forall|w: nat| #[trigger]
            pending(ma2, mb2, r, in0, w) && r(w) == z ==> pending(ma, mb, r, in0, w) && ma2(w)
                == ma(w),
        forall|w: nat| #[trigger]
            pending(ma, mb, r, in0, w) && r(w) == z ==> pending(ma2, mb2, r, in0, w),
    ensures
        pend(ma2, mb2, r, in0)(z) == pend(ma, mb, r, in0)(z),
{
    if exists|w: nat| #[trigger] pending(ma, mb, r, in0, w) && r(w) == z {
        let w1 = choose|w: nat| #[trigger] pending(ma, mb, r, in0, w) && r(w) == z;
        assert(pending(ma2, mb2, r, in0, w1) && r(w1) == z);
        let w2 = choose|w: nat| #[trigger] pending(ma2, mb2, r, in0, w) && r(w) == z;
        assert(r(w1) == r(w2));
    } else {
        assert(!exists|w: nat| #[trigger] pending(ma2, mb2, r, in0, w) && r(w) == z);
    }
}

proof fn hm_bind_ground(
    ts: Seq<Term>,
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    ys: Seq<Term>,
    in0: spec_fn(nat) -> bool,
    v: nat,
)
    requires
        hm_ok(ts, ma, mb, r, ys, in0),
        ts.len() > 0,
        ts[0] == Term::Var(v),
        ground(ma(v)),
        mb(v) == Term::Var(r(v)),
    ensures
        ({
            let u = hm_state(ts, ma, mb, ys);
            let g = ma(v);
            let mb2 = |w: nat|
                if w == v {
                    g
                } else {
                    mb(w)
                };
            let ys2 = subst_all(ys, r(v), g);
            &&& u_bind(u, u.pairs.drop_first(), r(v), g) == hm_state(ts.drop_first(), ma, mb2, ys2)
            &&& hm_ok(ts.drop_first(), ma, mb2, r, ys2, in0)
            &&& sub_all(ys2, pend(ma, mb2, r, in0)) == sub_all(ys, pend(ma, mb, r, in0))
        }),
{
    let u = hm_state(ts, ma, mb, ys);
    let g = ma(v);
    let mb2 = |w: nat|
        if w == v {
            g
        } else {
            mb(w)
        };
    let ys2 = subst_all(ys, r(v), g);
    let rest = ts.drop_first();
    assert(in0(v)) by {
        assert(occurs(v, ts[0]));
        occurs_member(v, ts, 0);
    }
    sub_all_index(ts, ma);
    sub_all_index(ts, mb);
    sub_all_index(rest, ma);
    sub_all_index(rest, mb);
    assert(u.pairs.drop_first() =~= zip(sub_all(rest, ma), sub_all(rest, mb)));
    assert forall|w: nat| #[trigger] occurs_all(w, rest) implies subst(ma(w), r(v), g) == ma(w)
        && subst(mb(w), r(v), g) == mb2(w) by {
        assert(occurs_all(w, ts)) by {
            let k = occurs_index(w, rest);
            occurs_member(w, ts, k + 1);
        }
        assert(in0(w));
        assert(var_ok(w, ma, mb, r, ys));
        if ma(w) == Term::Var(w) {
            assert(r(v) != w);
        }
        if ground(ma(w)) {
            crate::m6_default::ground_absent(ma(w), r(v));
            crate::k3_sound::unification::subst_absent(ma(w), r(v), g);
        }
        if w != v {
            assert(r(w) != r(v));
            if ground(mb(w)) {
                crate::m6_default::ground_absent(mb(w), r(v));
                crate::k3_sound::unification::subst_absent(mb(w), r(v), g);
            }
        }
    }
    bind_pairs(rest, ma, mb, r(v), g, ma, mb2);
    assert(u_bind(u, u.pairs.drop_first(), r(v), g).stack =~= Seq::<Goal>::empty());
    // the invariant after the bind
    assert forall|w: nat| #[trigger] in0(w) implies var_ok(w, ma, mb2, r, ys2) by {
        assert(var_ok(w, ma, mb, r, ys));
        if w != v && ma(w) == Term::Var(w) && mb(w) == Term::Var(r(w)) && !occurs_all(w, ys) {
            crate::m6_default::ground_absent(g, w);
            crate::m6_cell::subst_occurs_all(ys, r(v), g, w);
        }
    }
    assert forall|w: nat| #[trigger] unres(ma, mb2, r, in0, w) implies occurs_all(w, rest) by {
        assert(w != v);
        assert(unres(ma, mb, r, in0, w));
        assert(occurs_all(w, ts));
        let k = occurs_index(w, ts);
        assert(k != 0);
        occurs_member(w, rest, k - 1);
    }
    // the pending map
    subst_as_sub_all(ys, r(v), g);
    let x = r(v);
    let s1 = |z: nat|
        if z == x {
            g
        } else {
            Term::Var(z)
        };
    assert(ys2 == sub_all(ys, s1));
    sub_all_compose(ys, s1, pend(ma, mb2, r, in0));
    assert forall|z: nat| #[trigger] occurs_all(z, ys) implies sub(s1(z), pend(ma, mb2, r, in0))
        == pend(ma, mb, r, in0)(z) by {
        if z == r(v) {
            sub_ground(g, pend(ma, mb2, r, in0));
            assert(pending(ma, mb, r, in0, v) && r(v) == z);
            let w1 = choose|w: nat| #[trigger] pending(ma, mb, r, in0, w) && r(w) == z;
            assert(r(w1) == r(v));
        } else {
            pend_agree(ma, mb, ma, mb2, r, in0, z);
        }
    }
    sub_all_ext(ys, |z: nat| sub(s1(z), pend(ma, mb2, r, in0)), pend(ma, mb, r, in0));
    assert forall|w: nat| #[trigger] occurs_all(w, rest) implies in0(w) by {
        let k = occurs_index(w, rest);
        occurs_member(w, ts, k + 1);
    }
    assert(hm_ok(rest, ma, mb2, r, ys2, in0));
    assert(sub_all(ys2, pend(ma, mb2, r, in0)) == sub_all(ys, pend(ma, mb, r, in0)));
    assert(u_bind(u, u.pairs.drop_first(), r(v), g).pairs == hm_state(rest, ma, mb2, ys2).pairs);
    assert(u_bind(u, u.pairs.drop_first(), r(v), g) == hm_state(rest, ma, mb2, ys2));
}

proof fn hm_bind_open(
    ts: Seq<Term>,
    ma: spec_fn(nat) -> Term,
    mb: spec_fn(nat) -> Term,
    r: spec_fn(nat) -> nat,
    ys: Seq<Term>,
    in0: spec_fn(nat) -> bool,
    v: nat,
)
    requires
        hm_ok(ts, ma, mb, r, ys, in0),
        ts.len() > 0,
        ts[0] == Term::Var(v),
        ma(v) == Term::Var(v),
        mb(v) == Term::Var(r(v)),
        !occurs_all(v, ys),
    ensures
        ({
            let u = hm_state(ts, ma, mb, ys);
            let ma2 = |w: nat|
                if w == v {
                    Term::Var(r(v))
                } else {
                    ma(w)
                };
            &&& u_bind(u, u.pairs.drop_first(), v, Term::Var(r(v))) == hm_state(
                ts.drop_first(),
                ma2,
                mb,
                ys,
            )
            &&& hm_ok(ts.drop_first(), ma2, mb, r, ys, in0)
            &&& sub_all(ys, pend(ma2, mb, r, in0)) == sub_all(ys, pend(ma, mb, r, in0))
        }),
{
    let u = hm_state(ts, ma, mb, ys);
    let rv = Term::Var(r(v));
    let ma2 = |w: nat|
        if w == v {
            rv
        } else {
            ma(w)
        };
    let rest = ts.drop_first();
    assert(in0(v)) by {
        assert(occurs(v, ts[0]));
        occurs_member(v, ts, 0);
    }
    sub_all_index(ts, ma);
    sub_all_index(ts, mb);
    sub_all_index(rest, ma);
    sub_all_index(rest, mb);
    assert(u.pairs.drop_first() =~= zip(sub_all(rest, ma), sub_all(rest, mb)));
    assert forall|w: nat| #[trigger] occurs_all(w, rest) implies subst(ma(w), v, rv) == ma2(w)
        && subst(mb(w), v, rv) == mb(w) by {
        assert(occurs_all(w, ts)) by {
            let k = occurs_index(w, rest);
            occurs_member(w, ts, k + 1);
        }
        assert(in0(w));
        assert(var_ok(w, ma, mb, r, ys));
        assert(r(w) != v);
        if ground(ma(w)) {
            crate::m6_default::ground_absent(ma(w), v);
            crate::k3_sound::unification::subst_absent(ma(w), v, rv);
        }
        if ground(mb(w)) {
            crate::m6_default::ground_absent(mb(w), v);
            crate::k3_sound::unification::subst_absent(mb(w), v, rv);
        }
    }
    bind_pairs(rest, ma, mb, v, rv, ma2, mb);
    assert(u_bind(u, u.pairs.drop_first(), v, rv).stack =~= Seq::<Goal>::empty());
    crate::k3_sound::unification::subst_all_absent(ys, v, rv);
    assert forall|w: nat| #[trigger] in0(w) implies var_ok(w, ma2, mb, r, ys) by {
        assert(var_ok(w, ma, mb, r, ys));
    }
    assert forall|w: nat| #[trigger] unres(ma2, mb, r, in0, w) implies occurs_all(w, rest) by {
        assert(w != v);
        assert(unres(ma, mb, r, in0, w));
        assert(occurs_all(w, ts));
        let k = occurs_index(w, ts);
        assert(k != 0);
        occurs_member(w, rest, k - 1);
    }
    assert forall|z: nat| #[trigger] occurs_all(z, ys) implies pend(ma2, mb, r, in0)(z) == pend(
        ma,
        mb,
        r,
        in0,
    )(z) by {
        pend_agree(ma, mb, ma2, mb, r, in0, z);
    }
    sub_all_ext(ys, pend(ma2, mb, r, in0), pend(ma, mb, r, in0));
    assert forall|w: nat| #[trigger] occurs_all(w, rest) implies in0(w) by {
        let k = occurs_index(w, rest);
        occurs_member(w, ts, k + 1);
    }
    assert(hm_ok(rest, ma2, mb, r, ys, in0));
    assert(u_bind(u, u.pairs.drop_first(), v, rv).pairs == hm_state(rest, ma2, mb, ys).pairs);
    assert(u_bind(u, u.pairs.drop_first(), v, rv) == hm_state(rest, ma2, mb, ys));
}

// --- the walk's objects as substitutions of the clause ---
pub open spec fn lmap(p: Seq<(Term, Term)>) -> spec_fn(nat) -> Term {
    |v: nat|
        match ckc_spec::emit::lookup(p, Term::Var(v)) {
            Option::Some(w) => w,
            Option::None => Term::Var(v),
        }
}

pub proof fn bind_sub(t: Term, p: Seq<(Term, Term)>)
    requires
        crate::m6_default::vals_ground(p),
    ensures
        ckc_spec::emit::bind_all(t, p) == sub(t, lmap(p)),
    decreases t,
{
    match t {
        Term::Var(v) => {
            crate::m6_default::bind_var(v, p);
        },
        Term::Comp(n, args) => {
            crate::m6_default::bind_comp(n, args, p);
            let a2 = ckc_spec::emit::bind_all(t, p)->Comp_1;
            sub_all_index(args, lmap(p));
            assert forall|i: int| 0 <= i < args.len() implies a2[i] == sub_all(
                args,
                lmap(p),
            )[i] by {
                bind_sub(args[i], p);
            }
            assert(a2 =~= sub_all(args, lmap(p)));
        },
        _ => {
            crate::m6_default::bind_ground(t, p);
        },
    }
}

pub open spec fn rmap(fs: Seq<nat>, off: nat) -> spec_fn(nat) -> Term {
    |v: nat| Term::Var(off + ckc_spec::trace::pos_of(fs, v))
}

pub proof fn renum_sub(t: Term, fs: Seq<nat>, off: nat)
    ensures
        shift(ckc_spec::emit::renumber(t, fs), off) == sub(t, rmap(fs, off)),
    decreases t,
{
    if let Term::Comp(n, args) = t {
        renum_sub_all(args, fs, off);
    }
}

pub proof fn renum_sub_all(ts: Seq<Term>, fs: Seq<nat>, off: nat)
    ensures
        shift_all(ckc_spec::emit::renumber_all(ts, fs), off) == sub_all(ts, rmap(fs, off)),
    decreases ts,
{
    if ts.len() > 0 {
        renum_sub(ts[0], fs, off);
        renum_sub_all(ts.drop_first(), fs, off);
        let rn = ckc_spec::emit::renumber_all(ts, fs);
        assert(rn[0] == ckc_spec::emit::renumber(ts[0], fs));
        assert(rn.drop_first() =~= ckc_spec::emit::renumber_all(ts.drop_first(), fs));
        assert(shift_all(rn, off) == seq![shift(rn[0], off)] + shift_all(rn.drop_first(), off));
    } else {
        assert(ckc_spec::emit::renumber_all(ts, fs) =~= Seq::<Term>::empty());
    }
}

// The renamed body item of the canonical clause, as a substitution of the
// original item.
pub open spec fn sub_item(it: ckc_spec::v1text::BodyItem, m: spec_fn(nat) -> Term) -> Term {
    match it {
        ckc_spec::v1text::BodyItem::Pos(l) => sub(l, m),
        ckc_spec::v1text::BodyItem::Naf(gs) => Term::Comp(
            naf_name(),
            seq![conj_term(sub_all(gs, m))],
        ),
    }
}

pub proof fn item_sub(it: ckc_spec::v1text::BodyItem, fs: Seq<nat>, off: nat)
    ensures
        item_term(ckc_spec::emit::renumber_item(it, fs), off) == sub_item(it, rmap(fs, off)),
{
    match it {
        ckc_spec::v1text::BodyItem::Pos(l) => {
            renum_sub(l, fs, off);
        },
        ckc_spec::v1text::BodyItem::Naf(gs) => {
            renum_sub_all(gs, fs, off);
        },
    }
}

} // verus!
