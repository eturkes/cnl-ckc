#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
use vstd::prelude::*;

verus! {

// Contract ws D7 (cell invariance): the replays of two tuples in one cell run in
// lockstep. Their states are instances of one abstract state whose slot
// variables stand for the coordinates; `th` instantiates the slots.
pub open spec fn th_val(th: Seq<(nat, Term)>, x: nat) -> Option<Term>
    decreases th.len(),
{
    if th.len() == 0 {
        None
    } else if th[0].0 == x {
        Some(th[0].1)
    } else {
        th_val(th.drop_first(), x)
    }
}

// Every instantiated value is ground (a number, or an output's comparator).
pub open spec fn th_ok(th: Seq<(nat, Term)>) -> bool {
    forall|i: int| 0 <= i < th.len() ==> ground((#[trigger] th[i]).1)
}

pub open spec fn inst(t: Term, th: Seq<(nat, Term)>) -> Term
    decreases t,
{
    match t {
        Term::Var(x) => match th_val(th, x) {
            Some(v) => v,
            None => t,
        },
        Term::Comp(n, args) => Term::Comp(n, inst_all(args, th)),
        _ => t,
    }
}

pub open spec fn inst_all(ts: Seq<Term>, th: Seq<(nat, Term)>) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        seq![inst(ts[0], th)] + inst_all(ts.drop_first(), th)
    }
}

pub open spec fn inst_pairs(ps: Seq<(Term, Term)>, th: Seq<(nat, Term)>) -> Seq<(Term, Term)> {
    ps.map_values(|p: (Term, Term)| (inst(p.0, th), inst(p.1, th)))
}

pub open spec fn inst_u(u: UState, th: Seq<(nat, Term)>) -> UState {
    UState { pairs: inst_pairs(u.pairs, th), stack: u.stack, sol: inst_all(u.sol, th) }
}

// The abstract trace binds no slot variable (unify_n's recursion).
pub open spec fn no_slot_bind(u: UState, fuel: nat, th: Seq<(nat, Term)>) -> bool
    decreases fuel,
{
    if fuel == 0 || u.pairs.len() == 0 {
        true
    } else {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let f = (fuel - 1) as nat;
        match (a, b) {
            (Term::Var(x), _) => if a == b {
                no_slot_bind(UState { pairs: rest, ..u }, f, th)
            } else if occurs(x, b) {
                true
            } else {
                th_val(th, x) is None && no_slot_bind(u_bind(u, rest, x, b), f, th)
            },
            (_, Term::Var(y)) => if occurs(y, a) {
                true
            } else {
                th_val(th, y) is None && no_slot_bind(u_bind(u, rest, y, a), f, th)
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => if n == m && xs.len() == ys.len() {
                no_slot_bind(UState { pairs: zip(xs, ys) + rest, ..u }, f, th)
            } else {
                true
            },
            _ => if a == b {
                no_slot_bind(UState { pairs: rest, ..u }, f, th)
            } else {
                true
            },
        }
    }
}

// --- instantiation facts ---
pub proof fn th_val_number(th: Seq<(nat, Term)>, x: nat)
    requires
        th_ok(th),
    ensures
        th_val(th, x) matches Some(v) ==> ground(v),
    decreases th.len(),
{
    if th.len() > 0 && th[0].0 != x {
        assert forall|i: int| 0 <= i < th.drop_first().len() implies ground(
            (#[trigger] th.drop_first()[i]).1,
        ) by {
            assert(th.drop_first()[i] == th[i + 1]);
        }
        th_val_number(th.drop_first(), x);
    }
}

proof fn number_closed(v: Term, x: nat, w: Term)
    requires
        ground(v),
    ensures
        !occurs(x, v),
        subst(v, x, w) == v,
        !(v is Var),
{
    crate::m6_default::ground_absent(v, x);
    crate::k3_sound::unification::subst_absent(v, x, w);
}

pub proof fn inst_all_index(ts: Seq<Term>, th: Seq<(nat, Term)>)
    ensures
        inst_all(ts, th).len() == ts.len(),
        forall|i: int| 0 <= i < ts.len() ==> #[trigger] inst_all(ts, th)[i] == inst(ts[i], th),
    decreases ts.len(),
{
    if ts.len() > 0 {
        inst_all_index(ts.drop_first(), th);
        assert forall|i: int| 0 <= i < ts.len() implies #[trigger] inst_all(ts, th)[i] == inst(
            ts[i],
            th,
        ) by {
            if i > 0 {
                assert(ts.drop_first()[i - 1] == ts[i]);
            }
        }
    }
}

proof fn occurs_inst(x: nat, t: Term, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th_val(th, x) is None,
    ensures
        occurs(x, inst(t, th)) == occurs(x, t),
    decreases t,
{
    match t {
        Term::Var(y) => {
            th_val_number(th, y);
            if let Some(v) = th_val(th, y) {
                number_closed(v, x, v);
                assert(y != x);
            }
        },
        Term::Comp(_, args) => {
            occurs_inst_all(x, args, th);
        },
        _ => {},
    }
}

proof fn occurs_inst_all(x: nat, ts: Seq<Term>, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th_val(th, x) is None,
    ensures
        occurs_all(x, inst_all(ts, th)) == occurs_all(x, ts),
    decreases ts,
{
    if ts.len() > 0 {
        occurs_inst(x, ts[0], th);
        occurs_inst_all(x, ts.drop_first(), th);
        assert(inst_all(ts, th).drop_first() =~= inst_all(ts.drop_first(), th));
        assert(inst_all(ts, th)[0] == inst(ts[0], th));
    }
}

// Instantiation commutes with binding a non-slot variable.
proof fn inst_subst(t: Term, x: nat, v: Term, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th_val(th, x) is None,
    ensures
        inst(subst(t, x, v), th) == subst(inst(t, th), x, inst(v, th)),
    decreases t,
{
    match t {
        Term::Var(y) => {
            th_val_number(th, y);
            if y != x {
                if let Some(w) = th_val(th, y) {
                    number_closed(w, x, inst(v, th));
                }
            }
        },
        Term::Comp(n, args) => {
            inst_subst_all(args, x, v, th);
        },
        _ => {},
    }
}

proof fn inst_subst_all(ts: Seq<Term>, x: nat, v: Term, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th_val(th, x) is None,
    ensures
        inst_all(subst_all(ts, x, v), th) == subst_all(inst_all(ts, th), x, inst(v, th)),
    decreases ts,
{
    if ts.len() > 0 {
        inst_subst(ts[0], x, v, th);
        inst_subst_all(ts.drop_first(), x, v, th);
        assert(subst_all(ts, x, v)[0] == subst(ts[0], x, v));
        assert(subst_all(ts, x, v).drop_first() =~= subst_all(ts.drop_first(), x, v));
        assert(inst_all(ts, th)[0] == inst(ts[0], th));
        assert(inst_all(ts, th).drop_first() =~= inst_all(ts.drop_first(), th));
    }
}

// The instantiation extended by an unbound x ↦ v = the instantiation, then x
// replaced by v.
pub proof fn inst_ext(t: Term, x: nat, v: Term, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th_val(th, x) is None,
        ground(v),
    ensures
        inst(t, seq![(x, v)] + th) == subst(inst(t, th), x, v),
    decreases t,
{
    let th2 = seq![(x, v)] + th;
    assert(th2[0] == (x, v));
    assert(th2.drop_first() =~= th);
    match t {
        Term::Var(y) => {
            th_val_number(th, y);
            if y != x {
                assert(th_val(th2, y) == th_val(th, y));
                if let Some(w) = th_val(th, y) {
                    number_closed(w, x, v);
                }
            }
        },
        Term::Comp(n, args) => {
            inst_ext_all(args, x, v, th);
        },
        _ => {},
    }
}

pub proof fn inst_ext_all(ts: Seq<Term>, x: nat, v: Term, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th_val(th, x) is None,
        ground(v),
    ensures
        inst_all(ts, seq![(x, v)] + th) == subst_all(inst_all(ts, th), x, v),
    decreases ts,
{
    if ts.len() > 0 {
        inst_ext(ts[0], x, v, th);
        inst_ext_all(ts.drop_first(), x, v, th);
        assert(inst_all(ts, th).drop_first() =~= inst_all(ts.drop_first(), th));
    }
}

proof fn inst_bind(u: UState, rest: Seq<(Term, Term)>, x: nat, v: Term, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th_val(th, x) is None,
        u.stack.len() == 0,
    ensures
        inst_u(u_bind(u, rest, x, v), th) == u_bind(
            inst_u(u, th),
            inst_pairs(rest, th),
            x,
            inst(v, th),
        ),
{
    let l = inst_u(u_bind(u, rest, x, v), th);
    let r = u_bind(inst_u(u, th), inst_pairs(rest, th), x, inst(v, th));
    assert(l.pairs =~= r.pairs) by {
        assert forall|i: int| 0 <= i < l.pairs.len() implies l.pairs[i] == r.pairs[i] by {
            inst_subst(rest[i].0, x, v, th);
            inst_subst(rest[i].1, x, v, th);
        }
    }
    assert(l.stack =~= r.stack);
    inst_subst_all(u.sol, x, v, th);
}

proof fn inst_zip(xs: Seq<Term>, ys: Seq<Term>, rest: Seq<(Term, Term)>, th: Seq<(nat, Term)>)
    requires
        xs.len() == ys.len(),
    ensures
        inst_pairs(zip(xs, ys) + rest, th) == zip(inst_all(xs, th), inst_all(ys, th)) + inst_pairs(
            rest,
            th,
        ),
{
    inst_all_index(xs, th);
    inst_all_index(ys, th);
    assert(inst_pairs(zip(xs, ys) + rest, th) =~= zip(inst_all(xs, th), inst_all(ys, th))
        + inst_pairs(rest, th));
}

// A ground term meeting itself: the trace skips the pair without binding.
pub proof fn skip_number(v: Term, u: UState, g: nat)
    requires
        ground(v),
        u.pairs.len() > 0,
        u.pairs[0] == (v, v),
    ensures
        exists|h: nat| #[trigger]
            unify_n(u, h) == unify_n(UState { pairs: u.pairs.drop_first(), ..u }, g),
    decreases v, 1nat,
{
    reveal_with_fuel(unify_n, 1);
    match v {
        Term::Comp(n, args) => {
            skip_args(args, u.pairs.drop_first(), u, g);
            let h = choose|h: nat| #[trigger]
                unify_n(UState { pairs: zip(args, args) + u.pairs.drop_first(), ..u }, h)
                    == unify_n(UState { pairs: u.pairs.drop_first(), ..u }, g);
            assert(unify_n(u, h + 1) == unify_n(
                UState { pairs: zip(args, args) + u.pairs.drop_first(), ..u },
                h,
            ));
        },
        _ => {
            assert(unify_n(u, g + 1) == unify_n(UState { pairs: u.pairs.drop_first(), ..u }, g));
        },
    }
}

proof fn skip_args(args: Seq<Term>, rest: Seq<(Term, Term)>, u: UState, g: nat)
    requires
        ground_all(args),
    ensures
        exists|h: nat| #[trigger]
            unify_n(UState { pairs: zip(args, args) + rest, ..u }, h) == unify_n(
                UState { pairs: rest, ..u },
                g,
            ),
    decreases args, 0nat,
{
    if args.len() == 0 {
        assert(zip(args, args) + rest =~= rest);
        assert(unify_n(UState { pairs: zip(args, args) + rest, ..u }, g) == unify_n(
            UState { pairs: rest, ..u },
            g,
        ));
    } else {
        let t = args.drop_first();
        assert(ground(args[0]) && ground_all(t));
        skip_args(t, rest, u, g);
        let h2 = choose|h: nat| #[trigger]
            unify_n(UState { pairs: zip(t, t) + rest, ..u }, h) == unify_n(
                UState { pairs: rest, ..u },
                g,
            );
        let s0 = UState { pairs: zip(args, args) + rest, ..u };
        assert(s0.pairs[0] == (args[0], args[0]));
        assert(s0.pairs.drop_first() =~= zip(t, t) + rest);
        skip_number(args[0], s0, h2);
        let h1 = choose|h: nat| #[trigger]
            unify_n(s0, h) == unify_n(UState { pairs: s0.pairs.drop_first(), ..s0 }, h2);
        assert(UState { pairs: s0.pairs.drop_first(), ..s0 } == UState {
            pairs: zip(t, t) + rest,
            ..u
        });
        assert(unify_n(s0, h1) == unify_n(UState { pairs: rest, ..u }, g));
    }
}

// An Ok trace that binds no slot: the instantiated state unifies to the
// instantiated solution.
pub proof fn unify_n_inst(u: UState, f: nat, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        u.stack.len() == 0,
        no_slot_bind(u, f, th),
        unify_n(u, f) is Ok,
    ensures
        exists|g: nat|
            unify_n(inst_u(u, th), g) == UOut::Ok(Seq::empty(), inst_all(unify_n(u, f)->Ok_1, th)),
    decreases f,
{
    reveal_with_fuel(unify_n, 1);
    reveal_with_fuel(no_slot_bind, 1);
    let ui = inst_u(u, th);
    if u.pairs.len() == 0 {
        assert(u.stack =~= Seq::<Goal>::empty());
        assert(ui.pairs.len() == 0);
        assert(unify_n(ui, 1) == UOut::Ok(Seq::<Goal>::empty(), inst_all(u.sol, th)));
        return;
    }
    let a = u.pairs[0].0;
    let b = u.pairs[0].1;
    let rest = u.pairs.drop_first();
    let f1 = (f - 1) as nat;
    let ia = inst(a, th);
    let ib = inst(b, th);
    assert(ui.pairs[0] == (ia, ib));
    assert(ui.pairs.drop_first() =~= inst_pairs(rest, th));
    match (a, b) {
        (Term::Var(x), _) => {
            if a == b {
                let next = UState { pairs: rest, ..u };
                unify_n_inst(next, f1, th);
                let g = choose|g: nat|
                    unify_n(inst_u(next, th), g) == UOut::Ok(
                        Seq::empty(),
                        inst_all(unify_n(next, f1)->Ok_1, th),
                    );
                assert(inst_u(next, th) == UState { pairs: ui.pairs.drop_first(), ..ui });
                th_val_number(th, x);
                match th_val(th, x) {
                    Some(v) => {
                        assert(ia == v && ib == v);
                        skip_number(v, ui, g);
                        let h = choose|h: nat| #[trigger]
                            unify_n(ui, h) == unify_n(
                                UState { pairs: ui.pairs.drop_first(), ..ui },
                                g,
                            );
                        assert(unify_n(ui, h) == UOut::Ok(
                            Seq::<Goal>::empty(),
                            inst_all(unify_n(u, f)->Ok_1, th),
                        ));
                    },
                    None => {
                        assert(ia == a && ib == a);
                        assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
                    },
                }
            } else {
                assert(!occurs(x, b));
                assert(th_val(th, x) is None);
                occurs_inst(x, b, th);
                let next = u_bind(u, rest, x, b);
                unify_n_inst(next, f1, th);
                let g = choose|g: nat|
                    unify_n(inst_u(next, th), g) == UOut::Ok(
                        Seq::empty(),
                        inst_all(unify_n(next, f1)->Ok_1, th),
                    );
                inst_bind(u, rest, x, b, th);
                assert(ia == a);
                th_val_number(th, x);
                if let Term::Var(y) = b {
                    th_val_number(th, y);
                    if let Some(w) = th_val(th, y) {
                        number_closed(w, x, w);
                    }
                }
                assert(ib != a);
                assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
            }
        },
        (_, Term::Var(y)) => {
            assert(!occurs(y, a));
            assert(th_val(th, y) is None);
            occurs_inst(y, a, th);
            let next = u_bind(u, rest, y, a);
            unify_n_inst(next, f1, th);
            let g = choose|g: nat|
                unify_n(inst_u(next, th), g) == UOut::Ok(
                    Seq::empty(),
                    inst_all(unify_n(next, f1)->Ok_1, th),
                );
            inst_bind(u, rest, y, a, th);
            assert(ib == b);
            assert(!(ia is Var));
            assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
        },
        (Term::Comp(n, xs), Term::Comp(m, ys)) => {
            assert(n == m && xs.len() == ys.len());
            let next = UState { pairs: zip(xs, ys) + rest, ..u };
            unify_n_inst(next, f1, th);
            let g = choose|g: nat|
                unify_n(inst_u(next, th), g) == UOut::Ok(
                    Seq::empty(),
                    inst_all(unify_n(next, f1)->Ok_1, th),
                );
            inst_zip(xs, ys, rest, th);
            inst_all_index(xs, th);
            inst_all_index(ys, th);
            assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
        },
        _ => {
            assert(a == b);
            let next = UState { pairs: rest, ..u };
            unify_n_inst(next, f1, th);
            let g = choose|g: nat|
                unify_n(inst_u(next, th), g) == UOut::Ok(
                    Seq::empty(),
                    inst_all(unify_n(next, f1)->Ok_1, th),
                );
            assert(ia == a && ib == b);
            assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
        },
    }
}

// --- failure under instantiation (U2), through unifier completeness ---
proof fn apply_flat(t: Term, s: Seq<(nat, Term)>)
    requires
        !(t is Var),
        !(t is Comp),
    ensures
        ckc_spec::trace::apply(t, s) == t,
    decreases s.len(),
{
    if s.len() > 0 {
        apply_flat(subst(t, s[0].0, s[0].1), s.drop_first());
    }
}

// Binding x to a term of x's own image keeps every image.
proof fn apply_subst_eq(t: Term, x: nat, v: Term, s: Seq<(nat, Term)>)
    requires
        ckc_spec::trace::apply(Term::Var(x), s) == ckc_spec::trace::apply(v, s),
    ensures
        ckc_spec::trace::apply(subst(t, x, v), s) == ckc_spec::trace::apply(t, s),
    decreases t,
{
    match t {
        Term::Comp(n, args) => {
            assert forall|i: int| 0 <= i < args.len() implies ckc_spec::trace::apply(
                #[trigger] subst_all(args, x, v)[i],
                s,
            ) == ckc_spec::trace::apply(args[i], s) by {
                crate::k3_sound::unification::subst_all_map(args, x, v);
                apply_subst_eq(args[i], x, v, s);
            }
            crate::k3_sound::unification::subst_all_map(args, x, v);
            assert(crate::k3_sound::unification::apply_terms(subst_all(args, x, v), s)
                =~= crate::k3_sound::unification::apply_terms(args, s));
            crate::k3_sound::unification::apply_comp(n, subst_all(args, x, v), s);
            crate::k3_sound::unification::apply_comp(n, args, s);
        },
        _ => {},
    }
}

proof fn terms_size_index(ts: Seq<Term>, i: int)
    requires
        0 <= i < ts.len(),
    ensures
        crate::k2_engine::terms_size(ts) >= crate::k2_engine::term_size(ts[i]),
    decreases ts.len(),
{
    if i > 0 {
        terms_size_index(ts.drop_first(), i - 1);
    }
}

// x strictly inside b: b's image is larger than x's.
proof fn apply_size(b: Term, x: nat, s: Seq<(nat, Term)>)
    requires
        occurs(x, b),
        b != Term::Var(x),
    ensures
        crate::k2_engine::term_size(ckc_spec::trace::apply(b, s)) > crate::k2_engine::term_size(
            ckc_spec::trace::apply(Term::Var(x), s),
        ),
    decreases b,
{
    if let Term::Comp(n, args) = b {
        crate::k3_sound::unification::apply_comp(n, args, s);
        apply_size_all(args, x, s);
    }
}

// Some argument holds x: the arguments' images outweigh x's image.
proof fn apply_size_all(ts: Seq<Term>, x: nat, s: Seq<(nat, Term)>)
    requires
        occurs_all(x, ts),
    ensures
        crate::k2_engine::terms_size(crate::k3_sound::unification::apply_terms(ts, s))
            >= crate::k2_engine::term_size(ckc_spec::trace::apply(Term::Var(x), s)),
    decreases ts,
{
    let ia = crate::k3_sound::unification::apply_terms(ts, s);
    assert(ia.drop_first() =~= crate::k3_sound::unification::apply_terms(ts.drop_first(), s));
    if occurs(x, ts[0]) {
        if ts[0] != Term::Var(x) {
            apply_size(ts[0], x, s);
        }
    } else {
        apply_size_all(ts.drop_first(), x, s);
    }
}

// A unifiable list of pairs never fails.
proof fn unify_n_complete(u: UState, f: nat, s: Seq<(nat, Term)>)
    requires
        crate::k3_sound::unification::solves_all(u.pairs, s),
    ensures
        !(unify_n(u, f) is Fail),
    decreases f,
{
    reveal_with_fuel(unify_n, 1);
    if f > 0 && u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let f1 = (f - 1) as nat;
        assert(crate::k3_sound::unification::solves(u.pairs[0], s));
        crate::k3_sound::unification::solves_tail(seq![u.pairs[0]], rest, s);
        assert(seq![u.pairs[0]] + rest =~= u.pairs);
        match (a, b) {
            (Term::Var(x), _) => {
                if a == b {
                    unify_n_complete(UState { pairs: rest, ..u }, f1, s);
                } else if occurs(x, b) {
                    apply_size(b, x, s);
                } else {
                    let next = u_bind(u, rest, x, b);
                    assert forall|i: int|
                        0 <= i
                            < next.pairs.len() implies #[trigger] crate::k3_sound::unification::solves(
                    next.pairs[i], s) by {
                        apply_subst_eq(rest[i].0, x, b, s);
                        apply_subst_eq(rest[i].1, x, b, s);
                        assert(crate::k3_sound::unification::solves(rest[i], s));
                    }
                    unify_n_complete(next, f1, s);
                }
            },
            (_, Term::Var(y)) => {
                if occurs(y, a) {
                    apply_size(a, y, s);
                } else {
                    let next = u_bind(u, rest, y, a);
                    assert forall|i: int|
                        0 <= i
                            < next.pairs.len() implies #[trigger] crate::k3_sound::unification::solves(
                    next.pairs[i], s) by {
                        apply_subst_eq(rest[i].0, y, a, s);
                        apply_subst_eq(rest[i].1, y, a, s);
                        assert(crate::k3_sound::unification::solves(rest[i], s));
                    }
                    unify_n_complete(next, f1, s);
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                crate::k3_sound::unification::apply_comp(n, xs, s);
                crate::k3_sound::unification::apply_comp(m, ys, s);
                let ixs = crate::k3_sound::unification::apply_terms(xs, s);
                let iys = crate::k3_sound::unification::apply_terms(ys, s);
                assert(n == m && ixs == iys);
                assert(ixs.len() == xs.len() && iys.len() == ys.len());
                assert(xs.len() == ys.len());
                let pairs = zip(xs, ys) + rest;
                assert forall|i: int|
                    0 <= i < pairs.len() implies #[trigger] crate::k3_sound::unification::solves(
                    pairs[i],
                    s,
                ) by {
                    if i < xs.len() {
                        assert(pairs[i] == (xs[i], ys[i]));
                        assert(ixs[i] == iys[i]);
                    } else {
                        assert(pairs[i] == rest[i - xs.len()]);
                    }
                }
                unify_n_complete(UState { pairs, ..u }, f1, s);
            },
            _ => {
                if let Term::Comp(n, xs) = a {
                    crate::k3_sound::unification::apply_comp(n, xs, s);
                    apply_flat(b, s);
                } else if let Term::Comp(m, ys) = b {
                    crate::k3_sound::unification::apply_comp(m, ys, s);
                    apply_flat(a, s);
                } else {
                    apply_flat(a, s);
                    apply_flat(b, s);
                    unify_n_complete(UState { pairs: rest, ..u }, f1, s);
                }
            },
        }
    }
}

// Sequential application of number bindings = their instantiation.
proof fn apply_inst(t: Term, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
    ensures
        ckc_spec::trace::apply(t, th) == inst(t, th),
    decreases th.len(), t,
{
    if th.len() == 0 {
        inst_none(t, th);
    } else {
        let x = th[0].0;
        let v = th[0].1;
        let rest = th.drop_first();
        assert forall|i: int| 0 <= i < rest.len() implies ground((#[trigger] rest[i]).1) by {
            assert(rest[i] == th[i + 1]);
        }
        apply_inst(subst(t, x, v), rest);
        inst_head(t, th);
    }
}

proof fn inst_none(t: Term, th: Seq<(nat, Term)>)
    requires
        th.len() == 0,
    ensures
        inst(t, th) == t,
    decreases t,
{
    if let Term::Comp(n, args) = t {
        inst_all_none(args, th);
    }
}

proof fn inst_all_none(ts: Seq<Term>, th: Seq<(nat, Term)>)
    requires
        th.len() == 0,
    ensures
        inst_all(ts, th) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        inst_none(ts[0], th);
        inst_all_none(ts.drop_first(), th);
        assert(seq![ts[0]] + ts.drop_first() =~= ts);
    }
}

// inst under (x, v) :: rest = inst under rest after binding x to the number v.
proof fn inst_head(t: Term, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th.len() > 0,
    ensures
        inst(t, th) == inst(subst(t, th[0].0, th[0].1), th.drop_first()),
    decreases t,
{
    let x = th[0].0;
    let v = th[0].1;
    let rest = th.drop_first();
    match t {
        Term::Var(y) => {
            assert(ground(v));
            if y == x {
                inst_ground(v, rest);
            }
        },
        Term::Comp(n, args) => {
            inst_all_head(args, th);
        },
        _ => {},
    }
}

proof fn inst_all_head(ts: Seq<Term>, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th.len() > 0,
    ensures
        inst_all(ts, th) == inst_all(subst_all(ts, th[0].0, th[0].1), th.drop_first()),
    decreases ts,
{
    if ts.len() > 0 {
        inst_head(ts[0], th);
        inst_all_head(ts.drop_first(), th);
        assert(subst_all(ts, th[0].0, th[0].1)[0] == subst(ts[0], th[0].0, th[0].1));
        assert(subst_all(ts, th[0].0, th[0].1).drop_first() =~= subst_all(
            ts.drop_first(),
            th[0].0,
            th[0].1,
        ));
    }
}

pub proof fn inst_ground(t: Term, th: Seq<(nat, Term)>)
    requires
        ground(t),
    ensures
        inst(t, th) == t,
    decreases t,
{
    if let Term::Comp(n, args) = t {
        inst_all_ground(args, th);
    }
}

proof fn inst_all_ground(ts: Seq<Term>, th: Seq<(nat, Term)>)
    requires
        ground_all(ts),
    ensures
        inst_all(ts, th) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        inst_ground(ts[0], th);
        inst_all_ground(ts.drop_first(), th);
        assert(seq![ts[0]] + ts.drop_first() =~= ts);
    }
}

// U2: a failing unification fails under every instantiation of its slots.
pub proof fn unify_fail_inst(u: UState, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        unify(u) is Fail,
    ensures
        unify(inst_u(u, th)) is Fail,
{
    let ui = inst_u(u, th);
    crate::k2_engine::unify_done(ui);
    if unify(ui) is Ok {
        crate::k3_sound::unification::unify_witness(ui);
        let s = choose|s: Seq<(nat, Term)>| #[trigger]
            crate::k3_sound::unification::unifier_witness(ui, unify(ui), s);
        assert forall|i: int|
            0 <= i < u.pairs.len() implies #[trigger] crate::k3_sound::unification::solves(
            u.pairs[i],
            th + s,
        ) by {
            assert(ui.pairs[i] == (inst(u.pairs[i].0, th), inst(u.pairs[i].1, th)));
            assert(crate::k3_sound::unification::solves(ui.pairs[i], s));
            crate::k3_sound::apply_append(u.pairs[i].0, th, s);
            crate::k3_sound::apply_append(u.pairs[i].1, th, s);
            apply_inst(u.pairs[i].0, th);
            apply_inst(u.pairs[i].1, th);
        }
        assert(exists|f: nat| !(unify_n(u, f) is Out));
        let f = choose|f: nat| !(unify_n(u, f) is Out);
        unify_n_complete(u, f, th + s);
    }
}

// --- K2's goal stack as passive terms ---
// A call unifies with its goals on the stack; the walk keeps them in the
// solution. Both run the same pair trace, so outcomes correspond.
pub open spec fn goal_term(g: Goal) -> Term {
    match g {
        Goal::Lit(t, _) => t,
        Goal::NafCut(l) => Term::Int(l as int),
    }
}

pub open spec fn enc(s: Seq<Goal>) -> Seq<Term> {
    s.map_values(|g: Goal| goal_term(g))
}

pub open spec fn same_shape(a: Seq<Goal>, b: Seq<Goal>) -> bool {
    &&& a.len() == b.len()
    &&& forall|i: int|
        0 <= i < a.len() ==> match (#[trigger] a[i], b[i]) {
            (Goal::Lit(_, d1), Goal::Lit(_, d2)) => d1 == d2,
            (Goal::NafCut(l1), Goal::NafCut(l2)) => l1 == l2,
            _ => false,
        }
}

proof fn subst_all_concat(a: Seq<Term>, b: Seq<Term>, x: nat, v: Term)
    ensures
        subst_all(a + b, x, v) == subst_all(a, x, v) + subst_all(b, x, v),
{
    crate::k3_sound::unification::subst_all_map(a + b, x, v);
    crate::k3_sound::unification::subst_all_map(a, x, v);
    crate::k3_sound::unification::subst_all_map(b, x, v);
    assert(subst_all(a + b, x, v) =~= subst_all(a, x, v) + subst_all(b, x, v));
}

proof fn enc_subst(s: Seq<Goal>, x: nat, v: Term)
    ensures
        enc(s.map_values(|g: Goal| subst_goal(g, x, v))) == subst_all(enc(s), x, v),
        same_shape(s.map_values(|g: Goal| subst_goal(g, x, v)), s),
{
    crate::k3_sound::unification::subst_all_map(enc(s), x, v);
    assert(enc(s.map_values(|g: Goal| subst_goal(g, x, v))) =~= subst_all(enc(s), x, v));
}

pub proof fn unify_n_enc(u: UState, f: nat)
    ensures
        match (
            unify_n(u, f),
            unify_n(UState { pairs: u.pairs, stack: Seq::empty(), sol: u.sol + enc(u.stack) }, f),
        ) {
            (UOut::Ok(s1, x1), UOut::Ok(s2, x2)) => s2.len() == 0 && x2 == x1 + enc(s1)
                && same_shape(s1, u.stack),
            (UOut::Fail, UOut::Fail) => true,
            (UOut::Out, UOut::Out) => true,
            _ => false,
        },
    decreases f,
{
    reveal_with_fuel(unify_n, 1);
    let w = UState { pairs: u.pairs, stack: Seq::<Goal>::empty(), sol: u.sol + enc(u.stack) };
    if f > 0 && u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let f1 = (f - 1) as nat;
        match (a, b) {
            (Term::Var(x), _) => {
                if a == b {
                    unify_n_enc(UState { pairs: rest, ..u }, f1);
                } else if !occurs(x, b) {
                    let n = u_bind(u, rest, x, b);
                    enc_subst(u.stack, x, b);
                    subst_all_concat(u.sol, enc(u.stack), x, b);
                    assert(u_bind(w, rest, x, b) == UState {
                        pairs: n.pairs,
                        stack: Seq::<Goal>::empty(),
                        sol: n.sol + enc(n.stack),
                    }) by {
                        assert(Seq::<Goal>::empty().map_values(|g: Goal| subst_goal(g, x, b))
                            =~= Seq::<Goal>::empty());
                    }
                    unify_n_enc(n, f1);
                    if let UOut::Ok(s1, _) = unify_n(n, f1) {
                        assert forall|i: int| 0 <= i < s1.len() implies match (#[trigger]
                            s1[i],
                            u.stack[i],
                        ) {
                            (Goal::Lit(_, d1), Goal::Lit(_, d2)) => d1 == d2,
                            (Goal::NafCut(l1), Goal::NafCut(l2)) => l1 == l2,
                            _ => false,
                        } by {
                            assert(same_shape(n.stack, u.stack));
                            match (s1[i], n.stack[i]) {
                                _ => {},
                            }
                        }
                    }
                }
            },
            (_, Term::Var(y)) => {
                if !occurs(y, a) {
                    let n = u_bind(u, rest, y, a);
                    enc_subst(u.stack, y, a);
                    subst_all_concat(u.sol, enc(u.stack), y, a);
                    assert(u_bind(w, rest, y, a) == UState {
                        pairs: n.pairs,
                        stack: Seq::<Goal>::empty(),
                        sol: n.sol + enc(n.stack),
                    }) by {
                        assert(Seq::<Goal>::empty().map_values(|g: Goal| subst_goal(g, y, a))
                            =~= Seq::<Goal>::empty());
                    }
                    unify_n_enc(n, f1);
                    if let UOut::Ok(s1, _) = unify_n(n, f1) {
                        assert forall|i: int| 0 <= i < s1.len() implies match (#[trigger]
                            s1[i],
                            u.stack[i],
                        ) {
                            (Goal::Lit(_, d1), Goal::Lit(_, d2)) => d1 == d2,
                            (Goal::NafCut(l1), Goal::NafCut(l2)) => l1 == l2,
                            _ => false,
                        } by {
                            assert(same_shape(n.stack, u.stack));
                            match (s1[i], n.stack[i]) {
                                _ => {},
                            }
                        }
                    }
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                if n == m && xs.len() == ys.len() {
                    unify_n_enc(UState { pairs: zip(xs, ys) + rest, ..u }, f1);
                }
            },
            _ => {
                if a == b {
                    unify_n_enc(UState { pairs: rest, ..u }, f1);
                }
            },
        }
    } else if f > 0 {
        assert(same_shape(u.stack, u.stack));
    }
}

// --- instantiation through a variable-to-variable renaming of a slot ---
// The abstract trace may bind an instantiated variable x to a fresh variable
// z (the head meeting its own clause: an open output joins the clause's
// variable); the instantiation then carries x's value to z.
pub open spec fn tame(u: UState, fuel: nat, th: Seq<(nat, Term)>) -> bool
    decreases fuel,
{
    if fuel == 0 || u.pairs.len() == 0 {
        true
    } else {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let f = (fuel - 1) as nat;
        match (a, b) {
            (Term::Var(x), _) => if a == b {
                tame(UState { pairs: rest, ..u }, f, th)
            } else if occurs(x, b) {
                true
            } else {
                match th_val(th, x) {
                    Option::None => tame(u_bind(u, rest, x, b), f, th),
                    Option::Some(v) => match b {
                        Term::Var(z) => th_val(th, z) is None && tame(
                            u_bind(u, rest, x, b),
                            f,
                            seq![(z, v)] + th,
                        ),
                        _ => false,
                    },
                }
            },
            (_, Term::Var(y)) => if occurs(y, a) {
                true
            } else {
                th_val(th, y) is None && tame(u_bind(u, rest, y, a), f, th)
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => if n == m && xs.len() == ys.len() {
                tame(UState { pairs: zip(xs, ys) + rest, ..u }, f, th)
            } else {
                true
            },
            _ => if a == b {
                tame(UState { pairs: rest, ..u }, f, th)
            } else {
                true
            },
        }
    }
}

// The instantiation once x's value moves to the fresh z.
proof fn inst_move(t: Term, x: nat, z: nat, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th_val(th, x) is Some,
        th_val(th, z) is None,
        x != z,
    ensures
        inst(subst(t, x, Term::Var(z)), seq![(z, th_val(th, x)->0)] + th) == subst(
            inst(t, th),
            z,
            th_val(th, x)->0,
        ),
    decreases t,
{
    let v = th_val(th, x)->0;
    let th2 = seq![(z, v)] + th;
    th_val_number(th, x);
    assert(th2.drop_first() =~= th);
    match t {
        Term::Var(y) => {
            th_val_number(th, y);
            if y == x {
                assert(th_val(th2, z) == Some(v));
                assert(inst(t, th) == v);
                number_closed(v, z, v);
            } else if y == z {
                assert(inst(t, th) == t);
            } else {
                assert(th_val(th2, y) == th_val(th, y));
                if let Some(w) = th_val(th, y) {
                    number_closed(w, z, v);
                }
            }
        },
        Term::Comp(n, args) => {
            inst_move_all(args, x, z, th);
        },
        _ => {},
    }
}

proof fn inst_move_all(ts: Seq<Term>, x: nat, z: nat, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        th_val(th, x) is Some,
        th_val(th, z) is None,
        x != z,
    ensures
        inst_all(subst_all(ts, x, Term::Var(z)), seq![(z, th_val(th, x)->0)] + th) == subst_all(
            inst_all(ts, th),
            z,
            th_val(th, x)->0,
        ),
    decreases ts,
{
    if ts.len() > 0 {
        inst_move(ts[0], x, z, th);
        inst_move_all(ts.drop_first(), x, z, th);
        assert(subst_all(ts, x, Term::Var(z)).drop_first() =~= subst_all(
            ts.drop_first(),
            x,
            Term::Var(z),
        ));
        assert(inst_all(ts, th).drop_first() =~= inst_all(ts.drop_first(), th));
    }
}

pub proof fn th_extend_ok(th: Seq<(nat, Term)>, z: nat, v: Term)
    requires
        th_ok(th),
        ground(v),
    ensures
        th_ok(seq![(z, v)] + th),
{
    let th2 = seq![(z, v)] + th;
    assert forall|i: int| 0 <= i < th2.len() implies ground((#[trigger] th2[i]).1) by {
        if i > 0 {
            assert(th2[i] == th[i - 1]);
        }
    }
}

pub open spec fn th_grows(th: Seq<(nat, Term)>, th2: Seq<(nat, Term)>) -> bool {
    forall|y: nat| th_val(th, y) is Some ==> #[trigger] th_val(th2, y) == th_val(th, y)
}

pub open spec fn tame_out(
    u: UState,
    f: nat,
    th: Seq<(nat, Term)>,
    g: nat,
    th2: Seq<(nat, Term)>,
) -> bool {
    &&& th_ok(th2)
    &&& th_grows(th, th2)
    &&& unify_n(inst_u(u, th), g) == UOut::Ok(Seq::empty(), inst_all(unify_n(u, f)->Ok_1, th2))
}

// U1': a tame Ok trace — the instantiated state unifies to the instantiated
// solution under an extension of th.
pub proof fn unify_n_tame(u: UState, f: nat, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        u.stack.len() == 0,
        tame(u, f, th),
        unify_n(u, f) is Ok,
    ensures
        exists|g: nat, th2: Seq<(nat, Term)>| #[trigger] tame_out(u, f, th, g, th2),
    decreases f,
{
    reveal_with_fuel(unify_n, 1);
    reveal_with_fuel(tame, 1);
    let ui = inst_u(u, th);
    if u.pairs.len() == 0 {
        assert(u.stack =~= Seq::<Goal>::empty());
        assert(ui.pairs.len() == 0);
        assert(unify_n(ui, 1) == UOut::Ok(Seq::<Goal>::empty(), inst_all(u.sol, th)));
        assert(unify_n(u, f) == UOut::Ok(u.stack, u.sol));
        assert(th_grows(th, th));
        assert(tame_out(u, f, th, 1, th));
        return;
    }
    let a = u.pairs[0].0;
    let b = u.pairs[0].1;
    let rest = u.pairs.drop_first();
    let f1 = (f - 1) as nat;
    let ia = inst(a, th);
    let ib = inst(b, th);
    assert(ui.pairs[0] == (ia, ib));
    assert(ui.pairs.drop_first() =~= inst_pairs(rest, th));
    match (a, b) {
        (Term::Var(x), _) => {
            if a == b {
                let next = UState { pairs: rest, ..u };
                unify_n_tame(next, f1, th);
                let (g, th2) = choose|g: nat, th2: Seq<(nat, Term)>| #[trigger]
                    tame_out(next, f1, th, g, th2);
                assert(inst_u(next, th) == UState { pairs: ui.pairs.drop_first(), ..ui });
                th_val_number(th, x);
                match th_val(th, x) {
                    Some(v) => {
                        skip_number(v, ui, g);
                        let h = choose|h: nat| #[trigger]
                            unify_n(ui, h) == unify_n(
                                UState { pairs: ui.pairs.drop_first(), ..ui },
                                g,
                            );
                        assert(tame_out(u, f, th, h, th2));
                    },
                    None => {
                        assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
                        assert(tame_out(u, f, th, g + 1, th2));
                    },
                }
            } else {
                assert(!occurs(x, b));
                match th_val(th, x) {
                    Option::None => {
                        occurs_inst(x, b, th);
                        let next = u_bind(u, rest, x, b);
                        unify_n_tame(next, f1, th);
                        let (g, th2) = choose|g: nat, th2: Seq<(nat, Term)>| #[trigger]
                            tame_out(next, f1, th, g, th2);
                        inst_bind(u, rest, x, b, th);
                        if let Term::Var(y) = b {
                            th_val_number(th, y);
                            if let Some(w) = th_val(th, y) {
                                number_closed(w, x, w);
                            }
                        }
                        assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
                        assert(tame_out(u, f, th, g + 1, th2));
                    },
                    Option::Some(v) => {
                        let z = b->Var_0;
                        th_val_number(th, x);
                        number_closed(v, z, v);
                        let th1 = seq![(z, v)] + th;
                        assert(th1.drop_first() =~= th);
                        th_extend_ok(th, z, v);
                        let next = u_bind(u, rest, x, b);
                        unify_n_tame(next, f1, th1);
                        let (g, th2) = choose|g: nat, th2: Seq<(nat, Term)>| #[trigger]
                            tame_out(next, f1, th1, g, th2);
                        assert(ia == v && ib == b);
                        assert(!(v is Var));
                        let cnext = u_bind(ui, inst_pairs(rest, th), z, v);
                        assert(inst_u(next, th1) == cnext) by {
                            assert forall|i: int| 0 <= i < rest.len() implies #[trigger] inst_u(
                                next,
                                th1,
                            ).pairs[i] == cnext.pairs[i] by {
                                inst_move(rest[i].0, x, z, th);
                                inst_move(rest[i].1, x, z, th);
                            }
                            assert(inst_u(next, th1).pairs =~= cnext.pairs);
                            assert(inst_u(next, th1).stack =~= cnext.stack);
                            inst_move_all(u.sol, x, z, th);
                        }
                        assert(unify_n(ui, g + 1) == unify_n(cnext, g));
                        assert forall|y: nat| th_val(th, y) is Some implies #[trigger] th_val(
                            th2,
                            y,
                        ) == th_val(th, y) by {
                            assert(y != z);
                            assert(th_val(th1, y) == th_val(th, y));
                        }
                        assert(tame_out(u, f, th, g + 1, th2));
                    },
                }
            }
        },
        (_, Term::Var(y)) => {
            assert(!occurs(y, a));
            assert(th_val(th, y) is None);
            occurs_inst(y, a, th);
            let next = u_bind(u, rest, y, a);
            unify_n_tame(next, f1, th);
            let (g, th2) = choose|g: nat, th2: Seq<(nat, Term)>| #[trigger]
                tame_out(next, f1, th, g, th2);
            inst_bind(u, rest, y, a, th);
            assert(!(ia is Var));
            assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
            assert(tame_out(u, f, th, g + 1, th2));
        },
        (Term::Comp(n, xs), Term::Comp(m, ys)) => {
            let next = UState { pairs: zip(xs, ys) + rest, ..u };
            unify_n_tame(next, f1, th);
            let (g, th2) = choose|g: nat, th2: Seq<(nat, Term)>| #[trigger]
                tame_out(next, f1, th, g, th2);
            inst_zip(xs, ys, rest, th);
            inst_all_index(xs, th);
            inst_all_index(ys, th);
            assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
            assert(tame_out(u, f, th, g + 1, th2));
        },
        _ => {
            let next = UState { pairs: rest, ..u };
            unify_n_tame(next, f1, th);
            let (g, th2) = choose|g: nat, th2: Seq<(nat, Term)>| #[trigger]
                tame_out(next, f1, th, g, th2);
            assert(ia == a && ib == b);
            assert(unify_n(ui, g + 1) == unify_n(inst_u(next, th), g));
            assert(tame_out(u, f, th, g + 1, th2));
        },
    }
}

// --- the walk's slot check binds no slot (L-final) ---
pub open spec fn occ_pairs(x: nat, ps: Seq<(Term, Term)>) -> bool {
    exists|i: int| 0 <= i < ps.len() && (occurs(x, #[trigger] ps[i].0) || occurs(x, ps[i].1))
}

proof fn subst_elim(t: Term, x: nat, b: Term)
    requires
        !occurs(x, b),
    ensures
        !occurs(x, subst(t, x, b)),
    decreases t,
{
    if let Term::Comp(n, args) = t {
        subst_elim_all(args, x, b);
    }
}

proof fn subst_elim_all(ts: Seq<Term>, x: nat, b: Term)
    requires
        !occurs(x, b),
    ensures
        !occurs_all(x, subst_all(ts, x, b)),
    decreases ts,
{
    if ts.len() > 0 {
        subst_elim(ts[0], x, b);
        subst_elim_all(ts.drop_first(), x, b);
        assert(subst_all(ts, x, b).drop_first() =~= subst_all(ts.drop_first(), x, b));
    }
}

// Substituting y by b introduces only b's variables.
pub proof fn subst_occurs(t: Term, y: nat, b: Term, x: nat)
    requires
        !occurs(x, t),
        !occurs(x, b),
    ensures
        !occurs(x, subst(t, y, b)),
    decreases t,
{
    if let Term::Comp(n, args) = t {
        subst_occurs_all(args, y, b, x);
    }
}

pub proof fn subst_occurs_all(ts: Seq<Term>, y: nat, b: Term, x: nat)
    requires
        !occurs_all(x, ts),
        !occurs(x, b),
    ensures
        !occurs_all(x, subst_all(ts, y, b)),
    decreases ts,
{
    if ts.len() > 0 {
        subst_occurs(ts[0], y, b, x);
        subst_occurs_all(ts.drop_first(), y, b, x);
        assert(subst_all(ts, y, b).drop_first() =~= subst_all(ts.drop_first(), y, b));
    }
}

proof fn occurs_all_index(x: nat, ts: Seq<Term>, i: int)
    requires
        0 <= i < ts.len(),
        !occurs_all(x, ts),
    ensures
        !occurs(x, ts[i]),
    decreases ts.len(),
{
    if i > 0 {
        occurs_all_index(x, ts.drop_first(), i - 1);
    }
}

proof fn occurs_all_from(x: nat, ts: Seq<Term>)
    requires
        forall|i: int| 0 <= i < ts.len() ==> !occurs(x, #[trigger] ts[i]),
    ensures
        !occurs_all(x, ts),
    decreases ts.len(),
{
    if ts.len() > 0 {
        assert forall|i: int| 0 <= i < ts.drop_first().len() implies !occurs(
            x,
            #[trigger] ts.drop_first()[i],
        ) by {
            assert(ts.drop_first()[i] == ts[i + 1]);
        }
        occurs_all_from(x, ts.drop_first());
    }
}

// A variable absent from the pairs and the bound value stays absent.
proof fn occ_pairs_bind(rest: Seq<(Term, Term)>, y: nat, v: Term, x: nat)
    requires
        !occ_pairs(x, rest),
        !occurs(x, v),
    ensures
        !occ_pairs(x, rest.map_values(|p: (Term, Term)| (subst(p.0, y, v), subst(p.1, y, v)))),
{
    let n = rest.map_values(|p: (Term, Term)| (subst(p.0, y, v), subst(p.1, y, v)));
    assert forall|j: int| 0 <= j < n.len() implies !occurs(x, #[trigger] n[j].0) && !occurs(
        x,
        n[j].1,
    ) by {
        assert(!occurs(x, rest[j].0) && !occurs(x, rest[j].1)) by {
            if occurs(x, rest[j].0) || occurs(x, rest[j].1) {
                assert(occ_pairs(x, rest));
            }
        }
        subst_occurs(rest[j].0, y, v, x);
        subst_occurs(rest[j].1, y, v, x);
    }
    assert(!occ_pairs(x, n)) by {
        if occ_pairs(x, n) {
            let j = choose|j: int|
                0 <= j < n.len() && (occurs(x, #[trigger] n[j].0) || occurs(x, n[j].1));
            assert(!occurs(x, n[j].0) && !occurs(x, n[j].1));
        }
    }
}

// A variable absent from a state stays absent from its unifier's solution.
proof fn unify_n_no_new(u: UState, f: nat, x: nat)
    requires
        !occ_pairs(x, u.pairs),
        !occurs_all(x, u.sol),
        unify_n(u, f) is Ok,
    ensures
        !occurs_all(x, unify_n(u, f)->Ok_1),
    decreases f,
{
    reveal_with_fuel(unify_n, 1);
    if u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let f1 = (f - 1) as nat;
        assert(!occurs(x, a) && !occurs(x, b));
        assert(!occ_pairs(x, rest)) by {
            if occ_pairs(x, rest) {
                let i = choose|i: int|
                    0 <= i < rest.len() && (occurs(x, #[trigger] rest[i].0) || occurs(
                        x,
                        rest[i].1,
                    ));
                assert(rest[i] == u.pairs[i + 1]);
            }
        }
        match (a, b) {
            (Term::Var(y), _) => {
                if a == b {
                    unify_n_no_new(UState { pairs: rest, ..u }, f1, x);
                } else if !occurs(y, b) {
                    let n = u_bind(u, rest, y, b);
                    occ_pairs_bind(rest, y, b, x);
                    subst_occurs_all(u.sol, y, b, x);
                    unify_n_no_new(n, f1, x);
                }
            },
            (_, Term::Var(y)) => {
                if !occurs(y, a) {
                    let n = u_bind(u, rest, y, a);
                    occ_pairs_bind(rest, y, a, x);
                    subst_occurs_all(u.sol, y, a, x);
                    unify_n_no_new(n, f1, x);
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                if n == m && xs.len() == ys.len() {
                    let ps = zip(xs, ys) + rest;
                    assert(!occ_pairs(x, ps)) by {
                        if occ_pairs(x, ps) {
                            let i = choose|i: int|
                                0 <= i < ps.len() && (occurs(x, #[trigger] ps[i].0) || occurs(
                                    x,
                                    ps[i].1,
                                ));
                            if i < xs.len() {
                                assert(ps[i] == (xs[i], ys[i]));
                                occurs_all_index(x, xs, i);
                                occurs_all_index(x, ys, i);
                            } else {
                                assert(ps[i] == rest[i - xs.len()]);
                                assert(occ_pairs(x, rest));
                            }
                        }
                    }
                    unify_n_no_new(UState { pairs: ps, ..u }, f1, x);
                }
            },
            _ => {
                if a == b {
                    unify_n_no_new(UState { pairs: rest, ..u }, f1, x);
                }
            },
        }
    }
}

// The walk keeps the slots at the end of the solution and checks them
// unchanged: then no slot was bound, and no instantiated variable absent from
// the pairs either.
pub proof fn slots_kept(u: UState, f: nat, th: Seq<(nat, Term)>, base: int)
    requires
        u.stack.len() == 0,
        0 <= base <= u.sol.len(),
        unify_n(u, f) is Ok,
        unify_n(u, f)->Ok_1.len() == u.sol.len(),
        unify_n(u, f)->Ok_1.skip(base) == u.sol.skip(base),
        forall|x: nat|
            th_val(th, x) is Some ==> occ_pairs(x, u.pairs) ==> exists|i: int|
                base <= i < u.sol.len() && #[trigger] u.sol[i] == Term::Var(x),
    ensures
        no_slot_bind(u, f, th),
    decreases f,
{
    reveal_with_fuel(unify_n, 1);
    reveal_with_fuel(no_slot_bind, 1);
    if u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let f1 = (f - 1) as nat;
        assert forall|x: nat| th_val(th, x) is Some && occ_pairs(x, rest) implies exists|i: int|
            base <= i < u.sol.len() && #[trigger] u.sol[i] == Term::Var(x) by {
            let i = choose|i: int|
                0 <= i < rest.len() && (occurs(x, #[trigger] rest[i].0) || occurs(x, rest[i].1));
            assert(rest[i] == u.pairs[i + 1]);
            assert(occ_pairs(x, u.pairs));
        }
        match (a, b) {
            (Term::Var(x), _) => {
                if a == b {
                    slots_kept(UState { pairs: rest, ..u }, f1, th, base);
                } else if !occurs(x, b) {
                    let n = u_bind(u, rest, x, b);
                    if th_val(th, x) is Some {
                        // the bound slot leaves every term and never returns
                        assert(occurs(x, a));
                        assert(occ_pairs(x, u.pairs));
                        let i = choose|i: int|
                            base <= i < u.sol.len() && #[trigger] u.sol[i] == Term::Var(x);
                        assert(!occ_pairs(x, n.pairs)) by {
                            if occ_pairs(x, n.pairs) {
                                let j = choose|j: int|
                                    0 <= j < n.pairs.len() && (occurs(x, #[trigger] n.pairs[j].0)
                                        || occurs(x, n.pairs[j].1));
                                subst_elim(rest[j].0, x, b);
                                subst_elim(rest[j].1, x, b);
                            }
                        }
                        subst_elim_all(u.sol, x, b);
                        unify_n_no_new(n, f1, x);
                        let r = unify_n(u, f)->Ok_1;
                        assert(r == unify_n(n, f1)->Ok_1);
                        occurs_all_index(x, r, i);
                        assert(r.skip(base)[i - base] == r[i]);
                        assert(u.sol.skip(base)[i - base] == u.sol[i]);
                        assert(occurs(x, Term::Var(x)));
                    } else {
                        assert forall|y: nat|
                            th_val(th, y) is Some && occ_pairs(y, n.pairs) implies exists|i: int|
                            base <= i < n.sol.len() && #[trigger] n.sol[i] == Term::Var(y) by {
                            let j = choose|j: int|
                                0 <= j < n.pairs.len() && (occurs(y, #[trigger] n.pairs[j].0)
                                    || occurs(y, n.pairs[j].1));
                            if !occurs(y, rest[j].0) && !occurs(y, rest[j].1) && !occurs(y, b) {
                                subst_occurs(rest[j].0, x, b, y);
                                subst_occurs(rest[j].1, x, b, y);
                            }
                            if occurs(y, b) {
                                assert(occ_pairs(y, u.pairs));
                            } else {
                                assert(occurs(y, rest[j].0) || occurs(y, rest[j].1));
                                assert(rest[j] == u.pairs[j + 1]);
                                assert(occ_pairs(y, u.pairs));
                            }
                            let i = choose|i: int|
                                base <= i < u.sol.len() && #[trigger] u.sol[i] == Term::Var(y);
                            crate::k3_sound::unification::subst_all_map(u.sol, x, b);
                            assert(n.sol[i] == subst(u.sol[i], x, b));
                            assert(y != x);
                        }
                        crate::k3_sound::unification::subst_all_map(u.sol, x, b);
                        assert(n.sol.skip(base) =~= u.sol.skip(base)) by {
                            assert forall|i: int|
                                0 <= i < n.sol.skip(base).len() implies n.sol.skip(base)[i]
                                == u.sol.skip(base)[i] by {
                                let t = u.sol[base + i];
                                assert(u.sol.skip(base)[i] == t);
                                let r = unify_n(u, f)->Ok_1;
                                assert(r.skip(base)[i] == t);
                                // t is the unchanged final entry; it holds no x (bound) unless x stays
                                if occurs(x, t) {
                                    subst_elim(t, x, b);
                                    subst_elim_all(u.sol, x, b);
                                    assert(!occ_pairs(x, n.pairs)) by {
                                        if occ_pairs(x, n.pairs) {
                                            let j = choose|j: int|
                                                0 <= j < n.pairs.len() && (occurs(
                                                    x,
                                                    #[trigger] n.pairs[j].0,
                                                ) || occurs(x, n.pairs[j].1));
                                            subst_elim(rest[j].0, x, b);
                                            subst_elim(rest[j].1, x, b);
                                        }
                                    }
                                    unify_n_no_new(n, f1, x);
                                    occurs_all_index(x, r, base + i);
                                    assert(r[base + i] == t);
                                } else {
                                    crate::k3_sound::unification::subst_absent(t, x, b);
                                }
                            }
                        }
                        assert(unify_n(n, f1)->Ok_1.skip(base) == n.sol.skip(base));
                        slots_kept(n, f1, th, base);
                    }
                }
            },
            (_, Term::Var(y)) => {
                if !occurs(y, a) {
                    let n = u_bind(u, rest, y, a);
                    if th_val(th, y) is Some {
                        assert(occurs(y, b));
                        assert(occ_pairs(y, u.pairs));
                        let i = choose|i: int|
                            base <= i < u.sol.len() && #[trigger] u.sol[i] == Term::Var(y);
                        assert(!occ_pairs(y, n.pairs)) by {
                            if occ_pairs(y, n.pairs) {
                                let j = choose|j: int|
                                    0 <= j < n.pairs.len() && (occurs(y, #[trigger] n.pairs[j].0)
                                        || occurs(y, n.pairs[j].1));
                                subst_elim(rest[j].0, y, a);
                                subst_elim(rest[j].1, y, a);
                            }
                        }
                        subst_elim_all(u.sol, y, a);
                        unify_n_no_new(n, f1, y);
                        let r = unify_n(u, f)->Ok_1;
                        occurs_all_index(y, r, i);
                        assert(r.skip(base)[i - base] == r[i]);
                        assert(u.sol.skip(base)[i - base] == u.sol[i]);
                        assert(occurs(y, Term::Var(y)));
                    } else {
                        assert forall|z: nat|
                            th_val(th, z) is Some && occ_pairs(z, n.pairs) implies exists|i: int|
                            base <= i < n.sol.len() && #[trigger] n.sol[i] == Term::Var(z) by {
                            let j = choose|j: int|
                                0 <= j < n.pairs.len() && (occurs(z, #[trigger] n.pairs[j].0)
                                    || occurs(z, n.pairs[j].1));
                            if !occurs(z, rest[j].0) && !occurs(z, rest[j].1) && !occurs(z, a) {
                                subst_occurs(rest[j].0, y, a, z);
                                subst_occurs(rest[j].1, y, a, z);
                            }
                            if occurs(z, a) {
                                assert(occ_pairs(z, u.pairs));
                            } else {
                                assert(rest[j] == u.pairs[j + 1]);
                                assert(occ_pairs(z, u.pairs));
                            }
                            let i = choose|i: int|
                                base <= i < u.sol.len() && #[trigger] u.sol[i] == Term::Var(z);
                            crate::k3_sound::unification::subst_all_map(u.sol, y, a);
                            assert(n.sol[i] == subst(u.sol[i], y, a));
                            assert(z != y);
                        }
                        crate::k3_sound::unification::subst_all_map(u.sol, y, a);
                        assert(n.sol.skip(base) =~= u.sol.skip(base)) by {
                            assert forall|i: int|
                                0 <= i < n.sol.skip(base).len() implies n.sol.skip(base)[i]
                                == u.sol.skip(base)[i] by {
                                let t = u.sol[base + i];
                                assert(u.sol.skip(base)[i] == t);
                                let r = unify_n(u, f)->Ok_1;
                                assert(r.skip(base)[i] == t);
                                if occurs(y, t) {
                                    subst_elim_all(u.sol, y, a);
                                    assert(!occ_pairs(y, n.pairs)) by {
                                        if occ_pairs(y, n.pairs) {
                                            let j = choose|j: int|
                                                0 <= j < n.pairs.len() && (occurs(
                                                    y,
                                                    #[trigger] n.pairs[j].0,
                                                ) || occurs(y, n.pairs[j].1));
                                            subst_elim(rest[j].0, y, a);
                                            subst_elim(rest[j].1, y, a);
                                        }
                                    }
                                    unify_n_no_new(n, f1, y);
                                    occurs_all_index(y, r, base + i);
                                    assert(r[base + i] == t);
                                } else {
                                    crate::k3_sound::unification::subst_absent(t, y, a);
                                }
                            }
                        }
                        assert(unify_n(n, f1)->Ok_1.skip(base) == n.sol.skip(base));
                        slots_kept(n, f1, th, base);
                    }
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                if n == m && xs.len() == ys.len() {
                    let ps = zip(xs, ys) + rest;
                    assert forall|x: nat| th_val(th, x) is Some && occ_pairs(x, ps) implies exists|
                        i: int,
                    |
                        base <= i < u.sol.len() && #[trigger] u.sol[i] == Term::Var(x) by {
                        let j = choose|j: int|
                            0 <= j < ps.len() && (occurs(x, #[trigger] ps[j].0) || occurs(
                                x,
                                ps[j].1,
                            ));
                        if j < xs.len() {
                            assert(ps[j] == (xs[j], ys[j]));
                            if occurs(x, xs[j]) {
                                if !occurs_all(x, xs) {
                                    occurs_all_index(x, xs, j);
                                }
                            } else {
                                if !occurs_all(x, ys) {
                                    occurs_all_index(x, ys, j);
                                }
                            }
                            assert(occ_pairs(x, u.pairs));
                        } else {
                            assert(ps[j] == rest[j - xs.len()]);
                            assert(occ_pairs(x, rest));
                        }
                    }
                    slots_kept(UState { pairs: ps, ..u }, f1, th, base);
                }
            },
            _ => {
                if a == b {
                    slots_kept(UState { pairs: rest, ..u }, f1, th, base);
                }
            },
        }
    }
}

// --- the solution is passive: equal entries meet equal fates ---
pub proof fn unify_n_passive(u: UState, v: UState, f: nat)
    requires
        u.pairs == v.pairs,
        u.stack.len() == 0,
        v.stack.len() == 0,
    ensures
        (unify_n(u, f) is Ok) == (unify_n(v, f) is Ok),
        (unify_n(u, f) is Fail) == (unify_n(v, f) is Fail),
        unify_n(u, f) is Ok ==> unify_n(u, f)->Ok_1.len() == u.sol.len() && unify_n(
            v,
            f,
        )->Ok_1.len() == v.sol.len() && unify_n(u, f)->Ok_0.len() == 0 && unify_n(v, f)->Ok_0.len()
            == 0 && forall|i: int, j: int|
            0 <= i < u.sol.len() && 0 <= j < v.sol.len() && u.sol[i] == v.sol[j]
                ==> #[trigger] unify_n(u, f)->Ok_1[i] == #[trigger] unify_n(v, f)->Ok_1[j],
    decreases f,
{
    reveal_with_fuel(unify_n, 1);
    if f > 0 && u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let f1 = (f - 1) as nat;
        match (a, b) {
            (Term::Var(x), _) => {
                if a == b {
                    unify_n_passive(UState { pairs: rest, ..u }, UState { pairs: rest, ..v }, f1);
                } else if !occurs(x, b) {
                    let nu = u_bind(u, rest, x, b);
                    let nv = u_bind(v, rest, x, b);
                    assert(nu.stack =~= Seq::<Goal>::empty());
                    assert(nv.stack =~= Seq::<Goal>::empty());
                    crate::k3_sound::unification::subst_all_map(u.sol, x, b);
                    crate::k3_sound::unification::subst_all_map(v.sol, x, b);
                    unify_n_passive(nu, nv, f1);
                }
            },
            (_, Term::Var(y)) => {
                if !occurs(y, a) {
                    let nu = u_bind(u, rest, y, a);
                    let nv = u_bind(v, rest, y, a);
                    assert(nu.stack =~= Seq::<Goal>::empty());
                    assert(nv.stack =~= Seq::<Goal>::empty());
                    crate::k3_sound::unification::subst_all_map(u.sol, y, a);
                    crate::k3_sound::unification::subst_all_map(v.sol, y, a);
                    unify_n_passive(nu, nv, f1);
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                if n == m && xs.len() == ys.len() {
                    unify_n_passive(
                        UState { pairs: zip(xs, ys) + rest, ..u },
                        UState { pairs: zip(xs, ys) + rest, ..v },
                        f1,
                    );
                }
            },
            _ => {
                if a == b {
                    unify_n_passive(UState { pairs: rest, ..u }, UState { pairs: rest, ..v }, f1);
                }
            },
        }
    }
}

// --- a variable no pair names: entries keep it in place ---
pub open spec fn keeps(a: Term, b: Term, x: nat) -> bool {
    &&& (a == Term::Var(x) ==> b == Term::Var(x))
    &&& (a is Comp ==> b is Comp && b->Comp_0 == a->Comp_0 && b->Comp_1.len() == a->Comp_1.len()
        && forall|q: int|
        0 <= q < a->Comp_1.len() && a->Comp_1[q] == Term::Var(x) ==> #[trigger] b->Comp_1[q]
            == Term::Var(x))
    &&& (occurs(x, b) ==> occurs(x, a))
}

proof fn keeps_subst(t: Term, y: nat, v: Term, x: nat)
    requires
        y != x,
        !occurs(x, v),
    ensures
        keeps(t, subst(t, y, v), x),
{
    if !occurs(x, t) {
        subst_occurs(t, y, v, x);
    }
    if let Term::Comp(n, args) = t {
        crate::k3_sound::naf::subst_all_index(args, y, v);
    }
}

proof fn keeps_trans(a: Term, b: Term, c: Term, x: nat)
    requires
        keeps(a, b, x),
        keeps(b, c, x),
    ensures
        keeps(a, c, x),
{
    if a is Comp {
        assert forall|q: int|
            0 <= q < a->Comp_1.len() && a->Comp_1[q] == Term::Var(x) implies #[trigger] c->Comp_1[q]
            == Term::Var(x) by {
            assert(b->Comp_1[q] == Term::Var(x));
        }
    }
}

pub proof fn unify_n_keeps(u: UState, f: nat, x: nat, i: int)
    requires
        !occ_pairs(x, u.pairs),
        0 <= i < u.sol.len(),
        unify_n(u, f) is Ok,
    ensures
        i < unify_n(u, f)->Ok_1.len(),
        keeps(u.sol[i], unify_n(u, f)->Ok_1[i], x),
    decreases f,
{
    reveal_with_fuel(unify_n, 1);
    if u.pairs.len() == 0 {
        if u.sol[i] is Comp {
            assert(u.sol[i]->Comp_1 == u.sol[i]->Comp_1);
        }
        return;
    }
    let a = u.pairs[0].0;
    let b = u.pairs[0].1;
    let rest = u.pairs.drop_first();
    let f1 = (f - 1) as nat;
    assert(!occurs(x, a) && !occurs(x, b));
    assert(!occ_pairs(x, rest)) by {
        if occ_pairs(x, rest) {
            let j = choose|j: int|
                0 <= j < rest.len() && (occurs(x, #[trigger] rest[j].0) || occurs(x, rest[j].1));
            assert(rest[j] == u.pairs[j + 1]);
        }
    }
    match (a, b) {
        (Term::Var(y), _) => {
            if a == b {
                unify_n_keeps(UState { pairs: rest, ..u }, f1, x, i);
            } else if !occurs(y, b) {
                let n = u_bind(u, rest, y, b);
                assert(y != x);
                occ_pairs_bind(rest, y, b, x);
                crate::k3_sound::unification::subst_all_map(u.sol, y, b);
                assert(n.sol[i] == subst(u.sol[i], y, b));
                keeps_subst(u.sol[i], y, b, x);
                unify_n_keeps(n, f1, x, i);
                keeps_trans(u.sol[i], n.sol[i], unify_n(n, f1)->Ok_1[i], x);
            }
        },
        (_, Term::Var(y)) => {
            if !occurs(y, a) {
                let n = u_bind(u, rest, y, a);
                assert(y != x);
                occ_pairs_bind(rest, y, a, x);
                crate::k3_sound::unification::subst_all_map(u.sol, y, a);
                assert(n.sol[i] == subst(u.sol[i], y, a));
                keeps_subst(u.sol[i], y, a, x);
                unify_n_keeps(n, f1, x, i);
                keeps_trans(u.sol[i], n.sol[i], unify_n(n, f1)->Ok_1[i], x);
            }
        },
        (Term::Comp(n, xs), Term::Comp(m, ys)) => {
            if n == m && xs.len() == ys.len() {
                let ps = zip(xs, ys) + rest;
                assert(!occ_pairs(x, ps)) by {
                    if occ_pairs(x, ps) {
                        let j = choose|j: int|
                            0 <= j < ps.len() && (occurs(x, #[trigger] ps[j].0) || occurs(
                                x,
                                ps[j].1,
                            ));
                        if j < xs.len() {
                            assert(ps[j] == (xs[j], ys[j]));
                            occurs_all_index(x, xs, j);
                            occurs_all_index(x, ys, j);
                        } else {
                            assert(ps[j] == rest[j - xs.len()]);
                            assert(occ_pairs(x, rest));
                        }
                    }
                }
                unify_n_keeps(UState { pairs: ps, ..u }, f1, x, i);
            }
        },
        _ => {
            if a == b {
                unify_n_keeps(UState { pairs: rest, ..u }, f1, x, i);
            }
        },
    }
}

} // verus!
