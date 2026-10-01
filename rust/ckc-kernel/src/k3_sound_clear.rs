use super::body::*;
use super::bounded::*;
use super::freshness::*;
use super::goals::*;
use super::unification::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};
// A logged naf payload stays clear of the live stack and below `fresh`: no
// later binding reaches its variables, so it stays the call-time instance.

pub open spec fn clear_of(t: Term, stack: Seq<TGoal>) -> bool {
    forall|i: int| 0 <= i < stack.len() ==> !shares(t, #[trigger] tgoal_term(stack[i]))
}

pub open spec fn clear_event(ev: TEv, stack: Seq<TGoal>, fresh: nat) -> bool {
    match ev {
        TEv::Naf(t) => nvars(t) <= fresh && clear_of(t, stack),
        TEv::Clause(_) => true,
    }
}

// On a stack without naf cuts (the outer level) a safe call is clear of the whole rest.
pub proof fn safe_clear(t: Term, rest: Seq<TGoal>)
    requires
        naf_safe(t, rest),
        !in_naf(rest),
    ensures
        clear_of(t, rest),
{
    assert forall|i: int| 0 <= i < rest.len() implies !shares(
        t,
        #[trigger] tgoal_term(rest[i]),
    ) by {
        if in_naf(rest.take(i)) {
            let k = choose|k: int|
                0 <= k < rest.take(i).len() && #[trigger] rest.take(i)[k] is NafCut;
            assert(rest.take(i)[k] == rest[k]);
        }
    }
}

pub open spec fn naf_clear(log: Seq<(Seq<nat>, TEv)>, stack: Seq<TGoal>, fresh: nat) -> bool {
    forall|i: int| 0 <= i < log.len() ==> #[trigger] clear_event(log[i].1, stack, fresh)
}

// A naf payload that every binding of `s` leaves fixed.
pub open spec fn naf_fixed(ev: TEv, s: Seq<(nat, Term)>) -> bool {
    match ev {
        TEv::Naf(t) => apply(t, s) == t,
        TEv::Clause(_) => true,
    }
}

// --- variables through substitution ---
pub proof fn subst_occurs(t: Term, x: nat, v: Term, y: nat)
    requires
        occurs(y, subst(t, x, v)),
    ensures
        (occurs(y, t) && y != x) || occurs(y, v),
    decreases t,
{
    if let Term::Comp(_, args) = t {
        subst_all_occurs(args, x, v, y);
    }
}

pub proof fn subst_all_occurs(ts: Seq<Term>, x: nat, v: Term, y: nat)
    requires
        occurs_all(y, subst_all(ts, x, v)),
    ensures
        (occurs_all(y, ts) && y != x) || occurs(y, v),
    decreases ts,
{
    if ts.len() > 0 {
        let st = subst_all(ts, x, v);
        assert(st[0] == subst(ts[0], x, v));
        assert(st.drop_first() =~= subst_all(ts.drop_first(), x, v));
        if occurs(y, st[0]) {
            subst_occurs(ts[0], x, v, y);
        } else {
            subst_all_occurs(ts.drop_first(), x, v, y);
        }
    }
}

pub open spec fn in_values(y: nat, s: Seq<(nat, Term)>) -> bool {
    exists|i: int| 0 <= i < s.len() && #[trigger] occurs(y, s[i].1)
}

// A variable of apply(u, s) comes from u or from some binding's value.
pub proof fn apply_occurs(u: Term, s: Seq<(nat, Term)>, y: nat)
    requires
        occurs(y, apply(u, s)),
    ensures
        occurs(y, u) || in_values(y, s),
    decreases s.len(),
{
    if s.len() > 0 {
        let u1 = subst(u, s[0].0, s[0].1);
        apply_occurs(u1, s.drop_first(), y);
        if occurs(y, u1) {
            subst_occurs(u, s[0].0, s[0].1, y);
            if occurs(y, s[0].1) {
                assert(in_values(y, s));
            }
        } else {
            let i = choose|i: int|
                0 <= i < s.drop_first().len() && #[trigger] occurs(y, s.drop_first()[i].1);
            assert(s.drop_first()[i] == s[i + 1]);
            assert(occurs(y, s[i + 1].1));
            assert(in_values(y, s));
        }
    }
}

// No binding variable occurs in t ⇒ t is a fixpoint.
pub proof fn apply_avoid(t: Term, s: Seq<(nat, Term)>)
    requires
        forall|i: int| 0 <= i < s.len() ==> !occurs((#[trigger] s[i]).0, t),
    ensures
        apply(t, s) == t,
    decreases s.len(),
{
    if s.len() > 0 {
        assert(!occurs(s[0].0, t));
        subst_absent(t, s[0].0, s[0].1);
        assert forall|i: int| 0 <= i < s.drop_first().len() implies !occurs(
            (#[trigger] s.drop_first()[i]).0,
            t,
        ) by {
            assert(s.drop_first()[i] == s[i + 1]);
        }
        apply_avoid(t, s.drop_first());
    }
}

pub proof fn occurs_below(x: nat, t: Term)
    requires
        occurs(x, t),
    ensures
        x < nvars(t),
    decreases t,
{
    if let Term::Comp(_, args) = t {
        occurs_all_below(x, args);
    }
}

pub proof fn occurs_all_below(x: nat, ts: Seq<Term>)
    requires
        occurs_all(x, ts),
    ensures
        x < nvars_all(ts),
    decreases ts,
{
    if occurs(x, ts[0]) {
        occurs_below(x, ts[0]);
    } else {
        occurs_all_below(x, ts.drop_first());
    }
}

pub proof fn occurs_all_index(y: nat, ts: Seq<Term>, j: int)
    requires
        0 <= j < ts.len(),
        occurs(y, ts[j]),
    ensures
        occurs_all(y, ts),
    decreases j,
{
    if j > 0 {
        assert(ts.drop_first()[j - 1] == ts[j]);
        occurs_all_index(y, ts.drop_first(), j - 1);
    }
}

// --- unifier bindings stay within the unified pairs' variables ---
pub open spec fn in_pairs(x: nat, ps: Seq<(Term, Term)>) -> bool {
    exists|j: int| 0 <= j < ps.len() && (occurs(x, (#[trigger] ps[j]).0) || occurs(x, ps[j].1))
}

pub open spec fn bind_within(b: (nat, Term), ps: Seq<(Term, Term)>) -> bool {
    in_pairs(b.0, ps) && forall|y: nat| #[trigger] occurs(y, b.1) ==> in_pairs(y, ps)
}

pub open spec fn within(s: Seq<(nat, Term)>, ps: Seq<(Term, Term)>) -> bool {
    forall|i: int| 0 <= i < s.len() ==> #[trigger] bind_within(s[i], ps)
}

pub open spec fn covers(big: Seq<(Term, Term)>, small: Seq<(Term, Term)>) -> bool {
    forall|x: nat| #[trigger] in_pairs(x, small) ==> in_pairs(x, big)
}

pub open spec fn within_witness(u: UState, out: UOut, b: nat, s: Seq<(nat, Term)>) -> bool {
    bounded_witness(u, out, b, s) && within(s, u.pairs)
}

pub proof fn within_mono(s: Seq<(nat, Term)>, small: Seq<(Term, Term)>, big: Seq<(Term, Term)>)
    requires
        within(s, small),
        covers(big, small),
    ensures
        within(s, big),
{
    assert forall|i: int| 0 <= i < s.len() implies #[trigger] bind_within(s[i], big) by {
        assert(bind_within(s[i], small));
        assert(in_pairs(s[i].0, small));
        assert forall|y: nat| #[trigger] occurs(y, s[i].1) implies in_pairs(y, big) by {
            assert(in_pairs(y, small));
        }
    }
}

pub proof fn covers_tail(ps: Seq<(Term, Term)>)
    requires
        ps.len() > 0,
    ensures
        covers(ps, ps.drop_first()),
{
    assert forall|x: nat| #[trigger] in_pairs(x, ps.drop_first()) implies in_pairs(x, ps) by {
        let j = choose|j: int|
            0 <= j < ps.drop_first().len() && (occurs(x, (#[trigger] ps.drop_first()[j]).0)
                || occurs(x, ps.drop_first()[j].1));
        assert(ps.drop_first()[j] == ps[j + 1]);
    }
}

pub proof fn covers_bind(ps: Seq<(Term, Term)>, x: nat, v: Term)
    requires
        ps.len() > 0,
        forall|y: nat| #[trigger] occurs(y, v) ==> in_pairs(y, ps),
    ensures
        covers(
            ps,
            ps.drop_first().map_values(|p: (Term, Term)| (subst(p.0, x, v), subst(p.1, x, v))),
        ),
{
    let rest = ps.drop_first();
    let mapped = rest.map_values(|p: (Term, Term)| (subst(p.0, x, v), subst(p.1, x, v)));
    assert forall|y: nat| #[trigger] in_pairs(y, mapped) implies in_pairs(y, ps) by {
        let j = choose|j: int|
            0 <= j < mapped.len() && (occurs(y, (#[trigger] mapped[j]).0) || occurs(
                y,
                mapped[j].1,
            ));
        assert(rest[j] == ps[j + 1]);
        if occurs(y, mapped[j].0) {
            subst_occurs(rest[j].0, x, v, y);
        } else {
            subst_occurs(rest[j].1, x, v, y);
        }
        if occurs(y, v) {
            assert(in_pairs(y, ps));
        } else {
            assert(occurs(y, ps[j + 1].0) || occurs(y, ps[j + 1].1));
        }
    }
}

pub proof fn covers_decomp(ps: Seq<(Term, Term)>, n: Seq<u8>, xs: Seq<Term>, ys: Seq<Term>)
    requires
        ps.len() > 0,
        ps[0] == (Term::Comp(n, xs), Term::Comp(n, ys)),
        xs.len() == ys.len(),
    ensures
        covers(ps, zip(xs, ys) + ps.drop_first()),
{
    let next = zip(xs, ys) + ps.drop_first();
    assert forall|y: nat| #[trigger] in_pairs(y, next) implies in_pairs(y, ps) by {
        let j = choose|j: int|
            0 <= j < next.len() && (occurs(y, (#[trigger] next[j]).0) || occurs(y, next[j].1));
        if j < xs.len() {
            assert(next[j] == (xs[j], ys[j]));
            if occurs(y, xs[j]) {
                occurs_all_index(y, xs, j);
                assert(occurs(y, ps[0].0));
            } else {
                occurs_all_index(y, ys, j);
                assert(occurs(y, ps[0].1));
            }
        } else {
            assert(next[j] == ps[j - xs.len() + 1]);
        }
    }
}

pub proof fn lift_within(u: UState, out: UOut, b: nat, x: nat, v: Term, s: Seq<(nat, Term)>)
    requires
        u.pairs.len() > 0,
        !occurs(x, v),
        x < b,
        nvars(v) <= b,
        u.pairs[0] == (Term::Var(x), v) || u.pairs[0] == (v, Term::Var(x)),
        within_witness(u_bind(u, u.pairs.drop_first(), x, v), out, b, s),
    ensures
        within_witness(u, out, b, seq![(x, v)] + s),
{
    lift_bounded_binding(u, out, b, x, v, s);
    assert forall|y: nat| #[trigger] occurs(y, v) implies in_pairs(y, u.pairs) by {
        assert(occurs(y, u.pairs[0].0) || occurs(y, u.pairs[0].1));
    }
    assert(occurs(x, Term::Var(x)));
    assert(occurs(x, u.pairs[0].0) || occurs(x, u.pairs[0].1));
    assert(bind_within((x, v), u.pairs));
    covers_bind(u.pairs, x, v);
    within_mono(s, u_bind(u, u.pairs.drop_first(), x, v).pairs, u.pairs);
    let full = seq![(x, v)] + s;
    assert forall|i: int| 0 <= i < full.len() implies #[trigger] bind_within(full[i], u.pairs) by {
        if i > 0 {
            assert(full[i] == s[i - 1]);
        }
    }
}

pub proof fn unify_n_within(u: UState, fuel: nat, bnd: nat)
    requires
        unify_n(u, fuel) is Ok,
        pairs_below(u.pairs, bnd),
    ensures
        exists|s: Seq<(nat, Term)>| #[trigger] within_witness(u, unify_n(u, fuel), bnd, s),
    decreases fuel,
{
    if fuel > 0 {
        if u.pairs.len() == 0 {
            let s = Seq::<(nat, Term)>::empty();
            assert(apply_terms(u.sol, s) =~= u.sol);
            assert(within_witness(u, unify_n(u, fuel), bnd, s));
        } else {
            let a = u.pairs[0].0;
            let b = u.pairs[0].1;
            let rest = u.pairs.drop_first();
            let f = (fuel - 1) as nat;
            assert(pair_below(u.pairs[0], bnd));
            pairs_tail(u.pairs, bnd);
            covers_tail(u.pairs);
            match (a, b) {
                (Term::Var(x), _) => {
                    if a == b {
                        let nu = UState { pairs: rest, ..u };
                        unify_n_within(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            within_witness(nu, unify_n(nu, f), bnd, s);
                        solves_join(u.pairs, s);
                        within_mono(s, rest, u.pairs);
                        assert(within_witness(u, unify_n(u, fuel), bnd, s));
                    } else if !occurs(x, b) {
                        let nu = u_bind(u, rest, x, b);
                        pairs_bind(rest, bnd, x, b);
                        unify_n_within(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            within_witness(nu, unify_n(nu, f), bnd, s);
                        lift_within(u, unify_n(nu, f), bnd, x, b, s);
                        assert(within_witness(u, unify_n(u, fuel), bnd, seq![(x, b)] + s));
                    }
                },
                (_, Term::Var(y)) => {
                    if !occurs(y, a) {
                        let nu = u_bind(u, rest, y, a);
                        pairs_bind(rest, bnd, y, a);
                        unify_n_within(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            within_witness(nu, unify_n(nu, f), bnd, s);
                        lift_within(u, unify_n(nu, f), bnd, y, a, s);
                        assert(within_witness(u, unify_n(u, fuel), bnd, seq![(y, a)] + s));
                    }
                },
                (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                    if n == m && xs.len() == ys.len() {
                        let nu = UState { pairs: zip(xs, ys) + rest, ..u };
                        pairs_decomp(xs, ys, rest, bnd);
                        unify_n_within(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            within_witness(nu, unify_n(nu, f), bnd, s);
                        solves_comp(n, xs, ys, rest, s);
                        solves_tail(zip(xs, ys), rest, s);
                        solves_join(u.pairs, s);
                        covers_decomp(u.pairs, n, xs, ys);
                        within_mono(s, zip(xs, ys) + rest, u.pairs);
                        assert(within_witness(u, unify_n(u, fuel), bnd, s));
                    }
                },
                _ => {
                    if a == b {
                        let nu = UState { pairs: rest, ..u };
                        unify_n_within(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            within_witness(nu, unify_n(nu, f), bnd, s);
                        solves_join(u.pairs, s);
                        within_mono(s, rest, u.pairs);
                        assert(within_witness(u, unify_n(u, fuel), bnd, s));
                    }
                },
            }
        }
    }
}

pub proof fn tunify_within(pairs: Seq<(Term, Term)>, stack: Seq<TGoal>, b: nat)
    requires
        tunify(pairs, stack) is Ok,
        pairs_below(pairs, b),
    ensures
        exists|s: Seq<(nat, Term)>| #[trigger]
            tunifier_witness(pairs, stack, tunify(pairs, stack), b, s) && within(s, pairs),
{
    let u = UState { pairs, stack: seq![], sol: stack.map_values(|g: TGoal| tgoal_term(g)) };
    let f = choose|f: nat| !(unify_n(u, f) is Out);
    unify_n_within(u, f, b);
    let s = choose|s: Seq<(nat, Term)>| within_witness(u, unify_n(u, f), b, s);
    assert(unify(u) == unify_n(u, f));
    if let UOut::Ok(_, terms) = unify(u) {
        assert(Seq::new(stack.len(), |i: int| tgoal_with(stack[i], terms[i])) =~= tapply_all(
            stack,
            s,
        ));
        assert(tunifier_witness(pairs, stack, tunify(pairs, stack), b, s));
    }
}

// --- naf_clear transport ---
pub proof fn safe_drop(t: Term, stack: Seq<TGoal>)
    requires
        stack.len() > 0,
        clear_of(t, stack),
    ensures
        clear_of(t, stack.drop_first()),
{
    assert forall|i: int| 0 <= i < stack.drop_first().len() implies !shares(
        t,
        #[trigger] tgoal_term(stack.drop_first()[i]),
    ) by {
        assert(stack.drop_first()[i] == stack[i + 1]);
        assert(!shares(t, tgoal_term(stack[i + 1])));
    }
}

pub proof fn clear_drop(log: Seq<(Seq<nat>, TEv)>, stack: Seq<TGoal>, fresh: nat)
    requires
        stack.len() > 0,
        naf_clear(log, stack, fresh),
    ensures
        naf_clear(log, stack.drop_first(), fresh),
{
    assert forall|i: int| 0 <= i < log.len() implies #[trigger] clear_event(
        log[i].1,
        stack.drop_first(),
        fresh,
    ) by {
        assert(clear_event(log[i].1, stack, fresh));
        if let TEv::Naf(t) = log[i].1 {
            safe_drop(t, stack);
        }
    }
}

// Same live terms ⇒ same clearance.
pub proof fn clear_terms(log: Seq<(Seq<nat>, TEv)>, a: Seq<TGoal>, b: Seq<TGoal>, fresh: nat)
    requires
        naf_clear(log, a, fresh),
        a.len() == b.len(),
        forall|i: int| 0 <= i < a.len() ==> tgoal_term(#[trigger] a[i]) == tgoal_term(b[i]),
    ensures
        naf_clear(log, b, fresh),
{
    assert forall|i: int| 0 <= i < log.len() implies #[trigger] clear_event(log[i].1, b, fresh) by {
        assert(clear_event(log[i].1, a, fresh));
        if let TEv::Naf(t) = log[i].1 {
            assert forall|j: int| 0 <= j < b.len() implies !shares(
                t,
                #[trigger] tgoal_term(b[j]),
            ) by {
                assert(clear_of(t, a));
                assert(tgoal_term(a[j]) == tgoal_term(b[j]));
                assert(!shares(t, tgoal_term(a[j])));
            }
        }
    }
}

pub open spec fn alt_safe(a: TAlt) -> bool {
    match a {
        TAlt::Naf { stack, inner, .. } => naf_safe(inner, stack),
        TAlt::Cl { .. } => true,
    }
}

pub proof fn item_absent(it: BodyItem, off: nat, x: nat)
    requires
        wf_body_item(it),
        x < off,
    ensures
        !occurs(x, item_term(it, off)),
{
    item_shift(it, off);
    shifted_absent(base_item(it), off, x);
}

// A clause step (fresh renaming at `fresh` + unifier `s` over the goal/head
// pairs) leaves every logged payload fixed and clear of the new stack.
pub proof fn clear_after_call(
    log: Seq<(Seq<nat>, TEv)>,
    stack: Seq<TGoal>,
    fresh: nat,
    nfresh: nat,
    gname: Seq<u8>,
    args: Seq<Term>,
    hname: Seq<u8>,
    head: Term,
    body: Seq<TGoal>,
    s: Seq<(nat, Term)>,
)
    requires
        naf_clear(log, stack, fresh),
        stack.len() > 0,
        tgoal_term(stack[0]) == Term::Comp(gname, args),
        args.len() == args_of(head).len(),
        head == Term::Comp(hname, args_of(head)),
        forall|x: nat| x < fresh ==> !occurs(x, head),
        forall|i: int, x: nat|
            #![trigger occurs(x, tgoal_term(body[i]))]
            0 <= i < body.len() && x < fresh ==> !occurs(x, tgoal_term(body[i])),
        within(s, zip(args, args_of(head))),
        fresh <= nfresh,
    ensures
        forall|i: int| 0 <= i < log.len() ==> #[trigger] naf_fixed(log[i].1, s),
        naf_clear(log, tapply_all(body + stack.drop_first(), s), nfresh),
{
    let pairs = zip(args, args_of(head));
    let out = tapply_all(body + stack.drop_first(), s);
    assert forall|i: int| #![trigger log[i]] 0 <= i < log.len() implies naf_fixed(log[i].1, s)
        && clear_event(log[i].1, out, nfresh) by {
        assert(clear_event(log[i].1, stack, fresh));
        if let TEv::Naf(t) = log[i].1 {
            // no variable of the pairs occurs in t
            assert forall|y: nat| #[trigger] in_pairs(y, pairs) implies !occurs(y, t) by {
                let j = choose|j: int|
                    0 <= j < pairs.len() && (occurs(y, (#[trigger] pairs[j]).0) || occurs(
                        y,
                        pairs[j].1,
                    ));
                assert(pairs[j] == (args[j], args_of(head)[j]));
                if occurs(y, t) {
                    occurs_below(y, t);
                    if occurs(y, args[j]) {
                        occurs_all_index(y, args, j);
                        assert(occurs(y, tgoal_term(stack[0])));
                        assert(!shares(t, tgoal_term(stack[0])));
                    } else {
                        occurs_all_index(y, args_of(head), j);
                        assert(occurs(y, head));
                    }
                }
            }
            assert forall|k: int| 0 <= k < s.len() implies !occurs((#[trigger] s[k]).0, t) by {
                assert(bind_within(s[k], pairs));
            }
            apply_avoid(t, s);
            assert forall|k: int| 0 <= k < out.len() implies !shares(
                t,
                #[trigger] tgoal_term(out[k]),
            ) by {
                let g = (body + stack.drop_first())[k];
                assert(out[k] == tapply(g, s));
                assert(tgoal_term(out[k]) == apply(tgoal_term(g), s)) by {
                    match g {
                        TGoal::Lit(_, _, _) => {},
                        TGoal::NafCut(_) => {
                            assert(apply(Term::Nil, s) == Term::Nil) by {
                                apply_avoid(Term::Nil, s);
                            }
                        },
                    }
                }
                if shares(t, tgoal_term(out[k])) {
                    let y = choose|y: nat| #[trigger] occurs(y, t) && occurs(y, tgoal_term(out[k]));
                    apply_occurs(tgoal_term(g), s, y);
                    if occurs(y, tgoal_term(g)) {
                        occurs_below(y, t);
                        if k < body.len() {
                            assert(g == body[k]);
                        } else {
                            assert(g == stack[k - body.len() + 1]);
                            assert(!shares(t, tgoal_term(stack[k - body.len() + 1])));
                        }
                    } else {
                        let q = choose|q: int| 0 <= q < s.len() && #[trigger] occurs(y, s[q].1);
                        assert(bind_within(s[q], pairs));
                        assert(in_pairs(y, pairs));
                    }
                }
            }
        }
    }
}

} // verus!
