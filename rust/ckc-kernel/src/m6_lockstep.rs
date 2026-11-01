#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

// Contract ws D7 (cell invariance): the replays of two tuples in one cell run
// in lockstep. This module ties the obligation of a tuple to the walk's
// abstract objects: the witness facts are the skeleton with its slots
// instantiated by the tuple.
// The slot instantiation of a tuple: operand i's count variable ↦ t[i].
pub open spec fn slot_th(sel: ws::Sel, tw: Seq<Term>) -> Seq<(nat, Term)> {
    Seq::new(sel.ops.len(), |i: int| (sel.ops[i].n->Var_0, tw[i]))
}

// tuple_pairs' re-valuation keeps each key's first binding position.
pub proof fn lookup_revalue(pairs: Seq<(Term, Term)>, vs: Seq<(Term, Term)>, key: Term)
    ensures
        spec::lookup(
            pairs.map_values(
                |p: (Term, Term)|
                    match spec::lookup(vs, p.0) {
                        Option::Some(v) => (p.0, v),
                        Option::None => p,
                    },
            ),
            key,
        ) == match spec::lookup(pairs, key) {
            Option::None => Option::None,
            Option::Some(w) => match spec::lookup(vs, key) {
                Option::Some(v) => Option::Some(v),
                Option::None => Option::Some(w),
            },
        },
    decreases pairs.len(),
{
    reveal_with_fuel(spec::lookup, 1);
    let f = |p: (Term, Term)|
        match spec::lookup(vs, p.0) {
            Option::Some(v) => (p.0, v),
            Option::None => p,
        };
    if pairs.len() > 0 {
        let m = pairs.map_values(f);
        assert(m.drop_first() =~= pairs.drop_first().map_values(f));
        assert(m[0].0 == pairs[0].0);
        lookup_revalue(pairs.drop_first(), vs, key);
    }
}

// Dropping keys other than k keeps k's first binding.
pub proof fn lookup_filter(pairs: Seq<(Term, Term)>, cs: Seq<Term>, key: Term)
    requires
        !cs.contains(key),
    ensures
        spec::lookup(pairs.filter(|p: (Term, Term)| !cs.contains(p.0)), key) == spec::lookup(
            pairs,
            key,
        ),
    decreases pairs.len(),
{
    reveal_with_fuel(spec::lookup, 1);
    let pred = |p: (Term, Term)| !cs.contains(p.0);
    if pairs.len() > 0 {
        let rest = pairs.drop_first();
        assert(pairs =~= seq![pairs[0]] + rest);
        rest.lemma_filter_prepend(pairs[0], pred);
        lookup_filter(rest, cs, key);
        if pred(pairs[0]) {
            assert(pairs.filter(pred) == seq![pairs[0]] + rest.filter(pred));
            assert((seq![pairs[0]] + rest.filter(pred)).drop_first() =~= rest.filter(pred));
        } else {
            assert(pairs.filter(pred) == rest.filter(pred));
            assert(pairs[0].0 != key);
        }
    } else {
        assert(pairs.filter(pred) =~= Seq::<(Term, Term)>::empty());
    }
}

// Binding under the full pairs = binding under the pairs without the opened
// keys, then instantiating the opened variables by their full values.
pub proof fn bind_open(
    t: Term,
    full: Seq<(Term, Term)>,
    part: Seq<(Term, Term)>,
    th: Seq<(nat, Term)>,
)
    requires
        crate::m6_default::vals_ground(full),
        crate::m6_default::vals_ground(part),
        forall|x: nat| #[trigger]
            occurs(x, t) ==> (th_val(th, x) is Some ==> spec::lookup(full, Term::Var(x)) == th_val(
                th,
                x,
            ) && spec::lookup(part, Term::Var(x)) is None) && (th_val(th, x) is None
                ==> spec::lookup(full, Term::Var(x)) == spec::lookup(part, Term::Var(x))),
    ensures
        spec::bind_all(t, full) == inst(spec::bind_all(t, part), th),
    decreases t,
{
    match t {
        Term::Var(x) => {
            assert(occurs(x, t));
            crate::m6_default::bind_var(x, full);
            crate::m6_default::bind_var(x, part);
            if let Some(w) = spec::lookup(part, Term::Var(x)) {
                assert(ground(w)) by {
                    lookup_ground(part, Term::Var(x));
                }
                inst_ground(w, th);
            }
        },
        Term::Comp(n, args) => {
            crate::m6_default::bind_comp(n, args, full);
            crate::m6_default::bind_comp(n, args, part);
            let a2 = spec::bind_all(t, full)->Comp_1;
            let a3 = spec::bind_all(t, part)->Comp_1;
            inst_all_index(a3, th);
            assert forall|i: int| 0 <= i < args.len() implies a2[i] == inst(a3[i], th) by {
                assert forall|x: nat| #[trigger] occurs(x, args[i]) implies occurs(x, t) by {
                    occurs_member(x, args, i);
                }
                bind_open(args[i], full, part, th);
            }
            assert(a2 =~= inst_all(a3, th));
        },
        _ => {
            crate::m6_default::bind_ground(t, full);
            crate::m6_default::bind_ground(t, part);
        },
    }
}

pub proof fn lookup_ground(p: Seq<(Term, Term)>, k: Term)
    requires
        crate::m6_default::vals_ground(p),
    ensures
        spec::lookup(p, k) matches Some(w) ==> ground(w),
    decreases p.len(),
{
    reveal_with_fuel(spec::lookup, 1);
    if p.len() > 0 && p[0].0 != k {
        assert(crate::m6_default::vals_ground(p.drop_first())) by {
            assert forall|i: int| 0 <= i < p.drop_first().len() implies ground(
                (#[trigger] p.drop_first()[i]).1,
            ) by {
                assert(p.drop_first()[i] == p[i + 1]);
            }
        }
        lookup_ground(p.drop_first(), k);
    }
}

proof fn occurs_member(x: nat, args: Seq<Term>, i: int)
    requires
        0 <= i < args.len(),
        occurs(x, args[i]),
    ensures
        occurs_all(x, args),
    decreases args.len(),
{
    if i > 0 {
        occurs_member(x, args.drop_first(), i - 1);
    }
}

// --- naming: K2 renames above `fresh`, the walk above `off` ---
pub proof fn rename_shift(t: Term, f: nat, b: nat, d: nat)
    requires
        f >= b,
    ensures
        crate::k3_sound::naf::rename(shift(t, f), b, d) == shift(t, f + d),
    decreases t,
{
    if let Term::Comp(n, args) = t {
        rename_shift_all(args, f, b, d);
    }
}

pub proof fn rename_shift_all(ts: Seq<Term>, f: nat, b: nat, d: nat)
    requires
        f >= b,
    ensures
        crate::k3_sound::naf::rename_all(shift_all(ts, f), b, d) == shift_all(ts, f + d),
    decreases ts,
{
    if ts.len() > 0 {
        rename_shift(ts[0], f, b, d);
        rename_shift_all(ts.drop_first(), f, b, d);
        let s = shift_all(ts, f);
        assert(s == seq![shift(ts[0], f)] + shift_all(ts.drop_first(), f));
        assert(s[0] == shift(ts[0], f));
        assert(s.drop_first() =~= shift_all(ts.drop_first(), f));
        assert(crate::k3_sound::naf::rename_all(s, b, d) == seq![
            crate::k3_sound::naf::rename(s[0], b, d),
        ] + crate::k3_sound::naf::rename_all(s.drop_first(), b, d));
    } else {
        assert(shift_all(ts, f) =~= Seq::<Term>::empty());
        assert(shift_all(ts, f + d) =~= Seq::<Term>::empty());
    }
}

pub proof fn inst_shift(t: Term, o: nat, th: Seq<(nat, Term)>)
    requires
        forall|x: nat| #[trigger] th_val(th, x) is Some ==> x < o,
    ensures
        inst(shift(t, o), th) == shift(t, o),
    decreases t,
{
    match t {
        Term::Var(k) => {
            assert(th_val(th, k + o) is None);
        },
        Term::Comp(n, args) => {
            inst_shift_all(args, o, th);
        },
        _ => {},
    }
}

pub proof fn inst_shift_all(ts: Seq<Term>, o: nat, th: Seq<(nat, Term)>)
    requires
        forall|x: nat| #[trigger] th_val(th, x) is Some ==> x < o,
    ensures
        inst_all(shift_all(ts, o), th) == shift_all(ts, o),
    decreases ts,
{
    if ts.len() > 0 {
        inst_shift(ts[0], o, th);
        inst_shift_all(ts.drop_first(), o, th);
        let s = shift_all(ts, o);
        assert(s == seq![shift(ts[0], o)] + shift_all(ts.drop_first(), o));
        assert(s[0] == shift(ts[0], o));
        assert(s.drop_first() =~= shift_all(ts.drop_first(), o));
        assert(inst_all(s, th) == seq![inst(s[0], th)] + inst_all(s.drop_first(), th));
    } else {
        assert(shift_all(ts, o) =~= Seq::<Term>::empty());
    }
}

// --- concrete ↔ abstract goals ---
// A concrete term renamed into walk naming = the abstract term instantiated.
pub open spec fn grel(cg: Goal, ag: Goal, th: Seq<(nat, Term)>, b: nat, d: nat) -> bool {
    match (cg, ag) {
        (Goal::Lit(ct, d1), Goal::Lit(at, d2)) => d1 == d2 && crate::k3_sound::naf::rename(ct, b, d)
            == inst(at, th),
        (Goal::NafCut(l1), Goal::NafCut(l2)) => l1 == l2,
        _ => false,
    }
}

pub open spec fn srel(cs: Seq<Goal>, as_: Seq<Goal>, th: Seq<(nat, Term)>, b: nat, d: nat) -> bool {
    &&& cs.len() == as_.len()
    &&& forall|i: int| 0 <= i < cs.len() ==> #[trigger] grel(cs[i], as_[i], th, b, d)
}

// The abstract goals with the unifier's terms in place.
pub open spec fn decode(as_: Seq<Goal>, ts: Seq<Term>) -> Seq<Goal> {
    Seq::new(
        as_.len(),
        |i: int|
            match as_[i] {
                Goal::Lit(_, dd) => Goal::Lit(ts[i], dd),
                Goal::NafCut(l) => Goal::NafCut(l),
            },
    )
}

pub proof fn srel_enc(cs: Seq<Goal>, as_: Seq<Goal>, th: Seq<(nat, Term)>, b: nat, d: nat)
    requires
        srel(cs, as_, th, b, d),
    ensures
        crate::k3_sound::naf::rename_all(enc(cs), b, d) == inst_all(enc(as_), th),
{
    crate::k3_sound::naf::rename_all_index(enc(cs), b, d);
    inst_all_index(enc(as_), th);
    assert forall|i: int| 0 <= i < cs.len() implies crate::k3_sound::naf::rename_all(
        enc(cs),
        b,
        d,
    )[i] == inst_all(enc(as_), th)[i] by {
        assert(grel(cs[i], as_[i], th, b, d));
        match (cs[i], as_[i]) {
            (Goal::NafCut(l1), Goal::NafCut(l2)) => {
                assert(enc(cs)[i] == Term::Int(l1 as int));
                assert(enc(as_)[i] == Term::Int(l2 as int));
            },
            _ => {},
        }
    }
    assert(crate::k3_sound::naf::rename_all(enc(cs), b, d) =~= inst_all(enc(as_), th));
}

// The unifier's concrete stack, decoded against the abstract one.
pub proof fn srel_decode(
    cs2: Seq<Goal>,
    cs: Seq<Goal>,
    as_: Seq<Goal>,
    ts: Seq<Term>,
    th: Seq<(nat, Term)>,
    b: nat,
    d: nat,
)
    requires
        srel(cs, as_, th, b, d),
        same_shape(cs2, cs),
        ts.len() == as_.len(),
        crate::k3_sound::naf::rename_all(enc(cs2), b, d) == inst_all(ts, th),
    ensures
        srel(cs2, decode(as_, ts), th, b, d),
{
    crate::k3_sound::naf::rename_all_index(enc(cs2), b, d);
    inst_all_index(ts, th);
    let da = decode(as_, ts);
    assert forall|i: int| 0 <= i < cs2.len() implies #[trigger] grel(cs2[i], da[i], th, b, d) by {
        assert(grel(cs[i], as_[i], th, b, d));
        assert(crate::k3_sound::naf::rename_all(enc(cs2), b, d)[i] == inst_all(ts, th)[i]);
        assert(match (cs2[i], cs[i]) {
            (Goal::Lit(_, d1), Goal::Lit(_, d2)) => d1 == d2,
            (Goal::NafCut(l1), Goal::NafCut(l2)) => l1 == l2,
            _ => false,
        });
        match (cs2[i], cs[i]) {
            (Goal::Lit(t2, d2), Goal::Lit(_, d1)) => {
                assert(enc(cs2)[i] == t2);
                assert(crate::k3_sound::naf::rename_all(enc(cs2), b, d)[i]
                    == crate::k3_sound::naf::rename(t2, b, d));
                assert(inst_all(ts, th)[i] == inst(ts[i], th));
                match as_[i] {
                    Goal::Lit(_, dd) => {
                        assert(da[i] == Goal::Lit(ts[i], dd));
                        assert(d1 == dd);
                        assert(crate::k3_sound::naf::rename(t2, b, d) == inst(ts[i], th));
                        assert(grel(cs2[i], da[i], th, b, d));
                    },
                    _ => {},
                }
            },
            (Goal::NafCut(l2), Goal::NafCut(l1)) => {
                match as_[i] {
                    Goal::NafCut(l) => {
                        assert(da[i] == Goal::NafCut(l));
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}

// --- one fact candidate of a walked call ---
pub proof fn ground_elem(ts: Seq<Term>, i: int)
    requires
        ground_all(ts),
        0 <= i < ts.len(),
    ensures
        ground(ts[i]),
    decreases ts.len(),
{
    if i > 0 {
        ground_elem(ts.drop_first(), i - 1);
    }
}

// unify on a non-Out fuel.
pub proof fn unify_fuel(u: UState) -> (f: nat)
    ensures
        !(unify_n(u, f) is Out),
        unify(u) == unify_n(u, f),
{
    crate::k2_engine::unify_done(u);
    reveal(unify);
    let f = choose|f: nat| !(unify_n(u, f) is Out);
    crate::k3_sound::naf::unify_at(u, f);
    f
}

// The same outcome on states with equal pairs (empty stacks): kind and the
// results at entries holding equal terms.
pub proof fn unify_passive(u: UState, v: UState)
    requires
        u.pairs == v.pairs,
        u.stack.len() == 0,
        v.stack.len() == 0,
    ensures
        (unify(u) is Ok) == (unify(v) is Ok),
        (unify(u) is Fail) == (unify(v) is Fail),
        unify(u) is Ok ==> unify(u)->Ok_1.len() == u.sol.len() && unify(v)->Ok_1.len()
            == v.sol.len() && forall|i: int, j: int|
            0 <= i < u.sol.len() && 0 <= j < v.sol.len() && u.sol[i] == v.sol[j]
                ==> #[trigger] unify(u)->Ok_1[i] == #[trigger] unify(v)->Ok_1[j],
{
    let f = unify_fuel(u);
    unify_n_passive(u, v, f);
    if !(unify_n(v, f) is Out) {
        crate::k3_sound::naf::unify_at(v, f);
    }
    crate::k2_engine::unify_done(v);
    assert(!(unify_n(v, f) is Out)) by {
        crate::k2_engine::unify_done(u);
    }
}

// One comparison of two compounds of one name/arity = the comparison of
// their arguments.
pub proof fn unify_comp(n: Seq<u8>, xs: Seq<Term>, ys: Seq<Term>, sol: Seq<Term>)
    requires
        xs.len() == ys.len(),
    ensures
        unify(
            UState {
                pairs: seq![(Term::Comp(n, xs), Term::Comp(n, ys))],
                stack: Seq::empty(),
                sol,
            },
        ) == unify(UState { pairs: zip(xs, ys), stack: Seq::empty(), sol }),
{
    let u = UState {
        pairs: seq![(Term::Comp(n, xs), Term::Comp(n, ys))],
        stack: Seq::empty(),
        sol,
    };
    let w = UState { pairs: zip(xs, ys), stack: Seq::empty(), sol };
    let f = unify_fuel(w);
    reveal_with_fuel(unify_n, 1);
    assert(zip(xs, ys) + u.pairs.drop_first() =~= zip(xs, ys));
    assert(unify_n(u, f + 1) == unify_n(w, f));
    crate::k3_sound::naf::unify_at(u, f + 1);
}

pub proof fn cand_fact(
    ag: Term,
    fa: Term,
    arest: Seq<Goal>,
    slots: Seq<Term>,
    seg: Seq<Term>,
    th: Seq<(nat, Term)>,
    cg: Term,
    crest: Seq<Goal>,
    fc: Term,
    b: nat,
    d: nat,
)
    requires
        th_ok(th),
        ag is Comp,
        fa is Comp,
        crate::k3_sound::naf::rename(cg, b, d) == inst(ag, th),
        srel(crest, arest, th, b, d),
        fc == inst(fa, th),
        ground(fc),
        lit_fa(cg) == lit_fa(fc),
        forall|x: nat|
            th_val(th, x) is Some && (occurs(x, ag) || occurs(x, fa)) ==> #[trigger] slots.contains(
                Term::Var(x),
            ),
        match unify(UState { pairs: seq![(ag, fa)], stack: Seq::empty(), sol: seg + slots }) {
            UOut::Ok(_, s) => s.skip(seg.len() as int) == slots,
            UOut::Fail => true,
            UOut::Out => false,
        },
    ensures
        ({
            let uc = UState {
                pairs: zip(args_of(cg), args_of(fc)),
                stack: crest,
                sol: Seq::empty(),
            };
            let ua = UState { pairs: seq![(ag, fa)], stack: Seq::empty(), sol: enc(arest) };
            &&& (unify(uc) is Ok) == (unify(ua) is Ok)
            &&& !(unify(uc) is Out)
            &&& unify(uc) is Ok ==> unify(uc)->Ok_1.len() == 0 && srel(
                unify(uc)->Ok_0,
                decode(arest, unify(ua)->Ok_1),
                th,
                b,
                d,
            )
        }),
{
    let uc = UState { pairs: zip(args_of(cg), args_of(fc)), stack: crest, sol: Seq::empty() };
    let ua = UState { pairs: seq![(ag, fa)], stack: Seq::empty(), sol: enc(arest) };
    let ua2 = UState { pairs: seq![(ag, fa)], stack: Seq::empty(), sol: enc(arest) + slots };
    let uw = UState { pairs: seq![(ag, fa)], stack: Seq::empty(), sol: seg + slots };
    let ne = enc(arest).len();
    crate::k2_engine::unify_done(ua2);
    crate::k2_engine::unify_done(uw);
    crate::k2_engine::unify_done(uc);
    unify_passive(ua2, uw);
    unify_passive(ua2, ua);
    // the concrete state, encoded then renamed into walk naming
    let (n1, a1) = (cg->Comp_0, cg->Comp_1);
    let a2 = fc->Comp_1;
    let uce = UState {
        pairs: uc.pairs,
        stack: Seq::<Goal>::empty(),
        sol: Seq::<Term>::empty() + enc(crest),
    };
    let f0 = unify_fuel(uc);
    unify_n_enc(uc, f0);
    crate::k3_sound::naf::unify_at(uce, f0);
    let ia = inst_all(ag->Comp_1, th);
    let ifa = inst_all(fa->Comp_1, th);
    assert(inst(ag, th) == Term::Comp(ag->Comp_0, ia));
    assert(inst(fa, th) == Term::Comp(fa->Comp_0, ifa));
    crate::k3_sound::naf::rename_all_index(a1, b, d);
    assert(crate::k3_sound::naf::rename(cg, b, d) == Term::Comp(
        n1,
        crate::k3_sound::naf::rename_all(a1, b, d),
    ));
    assert(crate::k3_sound::naf::rename_all(a1, b, d) == ia);
    crate::k3_sound::naf::rename_ground_id(fc, b, d);
    srel_enc(crest, arest, th, b, d);
    let v = UState {
        pairs: zip(ia, ifa),
        stack: Seq::<Goal>::empty(),
        sol: inst_all(enc(arest), th),
    };
    assert(crate::k3_sound::naf::unify_rel(uce, v, b, d)) by {
        assert(Seq::<Term>::empty() + enc(crest) =~= enc(crest));
        crate::k3_sound::naf::rename_all_index(enc(crest), b, d);
        assert forall|i: int| 0 <= i < uce.pairs.len() implies #[trigger] v.pairs[i] == (
            crate::k3_sound::naf::rename(uce.pairs[i].0, b, d),
            crate::k3_sound::naf::rename(uce.pairs[i].1, b, d),
        ) by {
            assert(uce.pairs[i] == (a1[i], a2[i]));
            assert(ground(a2[i])) by {
                ground_elem(a2, i);
            }
            crate::k3_sound::naf::rename_ground_id(a2[i], b, d);
            assert(ifa[i] == a2[i]) by {
                inst_all_index(fa->Comp_1, th);
                assert(fc == Term::Comp(fa->Comp_0, ifa));
            }
        }
        assert forall|i: int| 0 <= i < uce.sol.len() implies #[trigger] v.sol[i]
            == crate::k3_sound::naf::rename(uce.sol[i], b, d) by {
            assert(uce.sol[i] == enc(crest)[i]);
        }
    }
    crate::k3_sound::naf::unify_related(uce, v, b, d);
    // the instantiated abstract state, one comparison step from v
    let ui = inst_u(ua2, th);
    inst_all_index(enc(arest) + slots, th);
    assert(ui.pairs == seq![(inst(ag, th), inst(fa, th))]) by {
        assert(ui.pairs =~= seq![(inst(ag, th), inst(fa, th))]);
    }
    let uiz = UState { pairs: zip(ia, ifa), stack: Seq::<Goal>::empty(), sol: ui.sol };
    assert(ag->Comp_0 == fa->Comp_0 && ia.len() == ifa.len()) by {
        inst_all_index(ag->Comp_1, th);
        inst_all_index(fa->Comp_1, th);
        crate::k3_sound::naf::rename_all_index(a1, b, d);
    }
    unify_comp(ag->Comp_0, ia, ifa, ui.sol);
    assert(ui == UState {
        pairs: seq![(Term::Comp(ag->Comp_0, ia), Term::Comp(ag->Comp_0, ifa))],
        stack: Seq::<Goal>::empty(),
        sol: ui.sol,
    });
    unify_passive(uiz, v);
    crate::k3_sound::naf::unify_output_len(uce);
    crate::k3_sound::naf::unify_output_len(v);
    if unify(ua2) is Ok {
        // the walk kept its slots ⇒ no instantiated variable was bound
        let f = unify_fuel(ua2);
        let s2 = unify(ua2)->Ok_1;
        let sw = unify(uw)->Ok_1;
        assert(s2.skip(ne as int) == slots) by {
            assert forall|k: int| 0 <= k < slots.len() implies s2.skip(ne as int)[k]
                == slots[k] by {
                assert(ua2.sol[ne + k] == uw.sol[seg.len() + k]);
                assert(s2[ne + k] == sw[seg.len() + k]);
                assert(sw.skip(seg.len() as int)[k] == sw[seg.len() + k]);
            }
            assert(s2.skip(ne as int) =~= slots);
        }
        assert forall|x: nat| th_val(th, x) is Some && occ_pairs(x, ua2.pairs) implies exists|
            i: int,
        |
            ne <= i < ua2.sol.len() && #[trigger] ua2.sol[i] == Term::Var(x) by {
            let j = choose|j: int|
                0 <= j < ua2.pairs.len() && (occurs(x, #[trigger] ua2.pairs[j].0) || occurs(
                    x,
                    ua2.pairs[j].1,
                ));
            assert(j == 0);
            assert(occurs(x, ag) || occurs(x, fa));
            assert(slots.contains(Term::Var(x)));
            let k = choose|k: int| 0 <= k < slots.len() && slots[k] == Term::Var(x);
            assert(ua2.sol[ne + k] == Term::Var(x));
        }
        assert(ua2.sol.skip(ne as int) =~= slots);
        assert(unify_n(ua2, f)->Ok_1 == s2);
        slots_kept(ua2, f, th, ne as int);
        unify_n_inst(ua2, f, th);
        let g = choose|g: nat|
            unify_n(ui, g) == UOut::Ok(Seq::<Goal>::empty(), inst_all(unify_n(ua2, f)->Ok_1, th));
        crate::k3_sound::naf::unify_at(ui, g);
        assert(unify(ui) is Ok);
        // the concrete unifier, pointwise
        assert(unify(uc) is Ok);
        let sc = unify(uc)->Ok_0;
        let rc = unify(uce)->Ok_1;
        assert(rc == Seq::<Term>::empty() + enc(sc));
        let rv = unify(v)->Ok_1;
        let rz = unify(uiz)->Ok_1;
        let ra = unify(ua)->Ok_1;
        inst_all_index(s2, th);
        crate::k3_sound::naf::rename_all_index(enc(sc), b, d);
        assert(ra.len() == arest.len());
        assert forall|i: int| 0 <= i < crest.len() implies crate::k3_sound::naf::rename_all(
            enc(sc),
            b,
            d,
        )[i] == inst_all(ra, th)[i] by {
            assert(rv[i] == crate::k3_sound::naf::rename(rc[i], b, d));
            assert(rc[i] == enc(sc)[i]);
            inst_all_index(enc(arest), th);
            assert((enc(arest) + slots)[i] == enc(arest)[i]);
            assert(uiz.sol[i] == v.sol[i]);
            assert(rz[i] == rv[i]);
            assert(rz[i] == unify(ui)->Ok_1[i]);
            assert(unify(ui)->Ok_1[i] == inst(s2[i], th));
            assert(ua2.sol[i] == ua.sol[i]);
            assert(s2[i] == ra[i]);
            inst_all_index(ra, th);
        }
        assert(crate::k3_sound::naf::rename_all(enc(sc), b, d) =~= inst_all(ra, th)) by {
            inst_all_index(ra, th);
        }
        srel_decode(sc, crest, arest, ra, th, b, d);
    } else {
        unify_fail_inst(ua2, th);
        assert(!(unify(uc) is Ok));
    }
}

// A rule of the program meets a walked call: it never unifies (the walk saw
// no rule head unify with the call), so neither concrete call does.
pub proof fn cand_rule(
    ag: Term,
    hd: Term,
    eoff: nat,
    arest: Seq<Goal>,
    th: Seq<(nat, Term)>,
    cg: Term,
    cstack: Seq<Goal>,
    fresh: nat,
    b: nat,
    d: nat,
)
    requires
        th_ok(th),
        ag is Comp,
        crate::k3_sound::naf::rename(cg, b, d) == inst(ag, th),
        fresh >= b,
        fresh + d == eoff,
        forall|x: nat| #[trigger] th_val(th, x) is Some ==> x < eoff,
        !ws::unifiable(ag, shift(hd, eoff)),
        lit_fa(cg) == lit_fa(hd),
    ensures
        unify(
            UState {
                pairs: zip(args_of(cg), args_of(shift(hd, fresh))),
                stack: cstack,
                sol: Seq::empty(),
            },
        ) is Fail,
{
    let sh = shift(hd, eoff);
    let ua = UState {
        pairs: seq![(ag, sh)],
        stack: Seq::<Goal>::empty(),
        sol: Seq::<Term>::empty(),
    };
    crate::k2_engine::unify_done(ua);
    assert(unify(ua) is Fail);
    let n = ag->Comp_0;
    let ax = ag->Comp_1;
    assert(hd is Comp);
    let sx = sh->Comp_1;
    assert(sh == Term::Comp(hd->Comp_0, shift_all(hd->Comp_1, eoff)));
    // the names agree: the concrete goal renames to the instantiated goal
    crate::k3_sound::naf::rename_all_index(cg->Comp_1, b, d);
    inst_all_index(ax, th);
    assert(cg->Comp_0 == n);
    assert(ax.len() == cg->Comp_1.len());
    crate::k3_sound::naf::shift_all_len(hd->Comp_1, eoff);
    crate::k3_sound::naf::shift_all_len(hd->Comp_1, fresh);
    assert(hd->Comp_0 == n && hd->Comp_1.len() == ax.len());
    assert(sx.len() == ax.len());
    unify_comp(n, ax, sx, Seq::<Term>::empty());
    let uz = UState { pairs: zip(ax, sx), stack: Seq::<Goal>::empty(), sol: Seq::<Term>::empty() };
    assert(unify(uz) is Fail);
    unify_fail_inst(uz, th);
    // the instantiated state = the concrete state renamed
    inst_shift_all(hd->Comp_1, eoff, th);
    let ui = inst_u(uz, th);
    assert(ui.pairs =~= zip(inst_all(ax, th), sx)) by {
        inst_all_index(sx, th);
    }
    let uc = UState {
        pairs: zip(args_of(cg), args_of(shift(hd, fresh))),
        stack: cstack,
        sol: Seq::<Term>::empty(),
    };
    let uce = UState {
        pairs: uc.pairs,
        stack: Seq::<Goal>::empty(),
        sol: Seq::<Term>::empty() + enc(cstack),
    };
    let f0 = unify_fuel(uc);
    unify_n_enc(uc, f0);
    crate::k3_sound::naf::unify_at(uce, f0);
    rename_shift_all(hd->Comp_1, fresh, b, d);
    let v = UState {
        pairs: zip(inst_all(ax, th), sx),
        stack: Seq::<Goal>::empty(),
        sol: crate::k3_sound::naf::rename_all(enc(cstack), b, d),
    };
    assert(crate::k3_sound::naf::unify_rel(uce, v, b, d)) by {
        assert(Seq::<Term>::empty() + enc(cstack) =~= enc(cstack));
        crate::k3_sound::naf::rename_all_index(enc(cstack), b, d);
        crate::k3_sound::naf::rename_all_index(shift_all(hd->Comp_1, fresh), b, d);
        assert forall|i: int| 0 <= i < uce.pairs.len() implies #[trigger] v.pairs[i] == (
            crate::k3_sound::naf::rename(uce.pairs[i].0, b, d),
            crate::k3_sound::naf::rename(uce.pairs[i].1, b, d),
        ) by {
            assert(shift(hd, fresh) == Term::Comp(hd->Comp_0, shift_all(hd->Comp_1, fresh)));
            assert(crate::k3_sound::naf::rename_all(shift_all(hd->Comp_1, fresh), b, d)[i]
                == crate::k3_sound::naf::rename(shift_all(hd->Comp_1, fresh)[i], b, d));
        }
    }
    crate::k3_sound::naf::unify_related(uce, v, b, d);
    unify_passive(v, ui);
}

} // verus!
