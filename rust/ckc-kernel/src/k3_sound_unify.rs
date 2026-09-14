#[cfg(verus_keep_ghost)]
use ckc_spec::{engine::*, term::Term, trace::apply};
#[cfg(verus_keep_ghost)]
use crate::k3_sound_bounds::{bindings_bounded, bindings_prepend, state_bind, state_bounded, state_comp, state_tail};
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn applied_all(ts: Seq<Term>, bindings: Seq<(nat, Term)>) -> Seq<Term> {
    ts.map_values(|t: Term| apply(t, bindings))
}

pub open spec fn solved(pairs: Seq<(Term, Term)>, bindings: Seq<(nat, Term)>) -> bool {
    forall|i: int| 0 <= i < pairs.len() ==> #[trigger] apply(pairs[i].0, bindings) == apply(pairs[i].1, bindings)
}

pub open spec fn witness(input: UState, out: UOut, bindings: Seq<(nat, Term)>) -> bool {
    solved(input.pairs, bindings) && match out {
        UOut::Ok(_, terms) => terms == applied_all(input.sol, bindings),
        _ => false,
    }
}

pub proof fn apply_prepend(t: Term, key: nat, value: Term, rest: Seq<(nat, Term)>)
    ensures apply(t, seq![(key, value)] + rest) == apply(subst(t, key, value), rest),
{
    assert((seq![(key, value)] + rest).len() > 0);
    assert((seq![(key, value)] + rest)[0] == (key, value));
    assert_seqs_equal!((seq![(key, value)] + rest).drop_first() == rest);
    reveal(apply);
}

proof fn subst_map(ts: Seq<Term>, key: nat, value: Term)
    ensures subst_all(ts, key, value) == ts.map_values(|t: Term| subst(t, key, value)),
    decreases ts.len(),
{
    if ts.len() > 0 {
        subst_map(ts.drop_first(), key, value);
        assert_seqs_equal!(ts.map_values(|t: Term| subst(t, key, value)) == seq![subst(ts[0], key, value)] + ts.drop_first().map_values(|t: Term| subst(t, key, value)));
    }
    reveal(subst_all);
}

pub proof fn apply_all_prepend(ts: Seq<Term>, key: nat, value: Term, rest: Seq<(nat, Term)>)
    ensures applied_all(ts, seq![(key, value)] + rest) == applied_all(subst_all(ts, key, value), rest),
{
    subst_map(ts, key, value);
    assert_seqs_equal!(applied_all(ts, seq![(key, value)] + rest) == applied_all(subst_all(ts, key, value), rest), i => {
        apply_prepend(ts[i], key, value, rest);
    });
}

pub proof fn apply_empty(ts: Seq<Term>)
    ensures applied_all(ts, Seq::empty()) == ts,
{
    assert_seqs_equal!(applied_all(ts, Seq::empty()) == ts, i => { reveal(apply); });
}

pub proof fn apply_comp(name: Seq<u8>, args: Seq<Term>, bindings: Seq<(nat, Term)>)
    ensures apply(Term::Comp(name, args), bindings) == Term::Comp(name, applied_all(args, bindings)),
    decreases bindings.len(),
{
    if bindings.len() == 0 {
        assert_seqs_equal!(bindings == Seq::empty());
        apply_empty(args);
    } else {
        let key = bindings[0].0;
        let value = bindings[0].1;
        let rest = bindings.drop_first();
        assert_seqs_equal!(bindings == seq![(key, value)] + rest);
        apply_all_prepend(args, key, value, rest);
        apply_comp(name, subst_all(args, key, value), rest);
        apply_prepend(Term::Comp(name, args), key, value, rest);
        reveal(subst);
        assert(subst(Term::Comp(name, args), key, value) == Term::Comp(name, subst_all(args, key, value)));
    }
    reveal(apply);
}

pub proof fn apply_concat(t: Term, left: Seq<(nat, Term)>, right: Seq<(nat, Term)>)
    ensures apply(t, left + right) == apply(apply(t, left), right),
    decreases left.len(),
{
    if left.len() > 0 {
        apply_concat(subst(t, left[0].0, left[0].1), left.drop_first(), right);
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
    }
    reveal_with_fuel(apply, 2);
}

pub proof fn subst_absent(t: Term, key: nat, value: Term)
    requires !occurs(key, t),
    ensures subst(t, key, value) == t,
    decreases t,
{
    match t {
        Term::Comp(_, args) => {
            reveal(occurs);
            subst_all_absent(args, key, value);
        },
        _ => {},
    }
    reveal(occurs); reveal(subst);
}

proof fn subst_all_absent(ts: Seq<Term>, key: nat, value: Term)
    requires !occurs_all(key, ts),
    ensures subst_all(ts, key, value) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        reveal(occurs_all);
        subst_absent(ts[0], key, value);
        subst_all_absent(ts.drop_first(), key, value);
        assert_seqs_equal!(seq![ts[0]] + ts.drop_first() == ts);
    }
    reveal(subst_all);
}

proof fn skip_witness(input: UState, out: UOut, bindings: Seq<(nat, Term)>)
    requires input.pairs.len() > 0, input.pairs[0].0 == input.pairs[0].1,
        witness(UState { pairs: input.pairs.drop_first(), ..input }, out, bindings),
    ensures witness(input, out, bindings),
{
    assert forall|i: int| 0 <= i < input.pairs.len() implies
        #[trigger] apply(input.pairs[i].0, bindings) == apply(input.pairs[i].1, bindings) by {
        if i > 0 { assert(input.pairs.drop_first()[i - 1] == input.pairs[i]); }
    }
}

proof fn bind_witness(input: UState, key: nat, value: Term, out: UOut, bindings: Seq<(nat, Term)>)
    requires
        input.pairs.len() > 0,
        subst(input.pairs[0].0, key, value) == subst(input.pairs[0].1, key, value),
        witness(u_bind(input, input.pairs.drop_first(), key, value), out, bindings),
    ensures witness(input, out, seq![(key, value)] + bindings),
{
    let next = u_bind(input, input.pairs.drop_first(), key, value);
    let full = seq![(key, value)] + bindings;
    apply_all_prepend(input.sol, key, value, bindings);
    assert forall|i: int| 0 <= i < input.pairs.len() implies
        #[trigger] apply(input.pairs[i].0, full) == apply(input.pairs[i].1, full) by {
        apply_prepend(input.pairs[i].0, key, value, bindings);
        apply_prepend(input.pairs[i].1, key, value, bindings);
        if i > 0 {
            assert(input.pairs.drop_first()[i - 1] == input.pairs[i]);
            assert(next.pairs[i - 1] == (subst(input.pairs[i].0, key, value), subst(input.pairs[i].1, key, value)));
            assert(apply(next.pairs[i - 1].0, bindings) == apply(next.pairs[i - 1].1, bindings));
        }
    }
}

proof fn comp_witness(input: UState, name: Seq<u8>, xs: Seq<Term>, ys: Seq<Term>, out: UOut, bindings: Seq<(nat, Term)>)
    requires
        input.pairs.len() > 0,
        input.pairs[0] == (Term::Comp(name, xs), Term::Comp(name, ys)), xs.len() == ys.len(),
        witness(UState { pairs: zip(xs, ys) + input.pairs.drop_first(), ..input }, out, bindings),
    ensures witness(input, out, bindings),
{
    let pairs = zip(xs, ys) + input.pairs.drop_first();
    assert_seqs_equal!(applied_all(xs, bindings) == applied_all(ys, bindings), i => {
        assert(pairs[i] == (xs[i], ys[i]));
        assert(apply(pairs[i].0, bindings) == apply(pairs[i].1, bindings));
    });
    apply_comp(name, xs, bindings);
    apply_comp(name, ys, bindings);
    assert forall|i: int| 0 <= i < input.pairs.len() implies
        #[trigger] apply(input.pairs[i].0, bindings) == apply(input.pairs[i].1, bindings) by {
        if i > 0 {
            assert(pairs[xs.len() as int + i - 1] == input.pairs[i]);
            assert(apply(pairs[xs.len() as int + i - 1].0, bindings) == apply(pairs[xs.len() as int + i - 1].1, bindings));
        }
    }
}

pub proof fn unify_n_witness(input: UState, fuel: nat, limit: nat) -> (bindings: Seq<(nat, Term)>)
    requires unify_n(input, fuel) is Ok, state_bounded(input, limit),
    ensures witness(input, unify_n(input, fuel), bindings), bindings_bounded(bindings, limit),
    decreases fuel,
{
    reveal_with_fuel(unify_n, 1);
    if fuel == 0 { assert(false); return Seq::empty(); }
    let out = unify_n(input, fuel);
    if input.pairs.len() == 0 {
        let empty = Seq::empty();
        apply_empty(input.sol);
        return empty;
    }
    let a = input.pairs[0].0;
    let b = input.pairs[0].1;
    let rest = input.pairs.drop_first();
    let left = (fuel - 1) as nat;
    assert(nvars(a) <= limit);
    assert(nvars(b) <= limit);
    state_tail(input, limit);
    match (a, b) {
        (Term::Var(key), _) => {
            if a == b {
                let next = UState { pairs: rest, ..input };
                let bindings = unify_n_witness(next, left, limit);
                skip_witness(input, out, bindings);
                bindings
            } else if occurs(key, b) {
                assert(false); Seq::empty()
            } else {
                reveal(nvars);
                assert(key < limit);
                state_bind(input, key, b, limit);
                let next = u_bind(input, rest, key, b);
                let bindings = unify_n_witness(next, left, limit);
                subst_absent(b, key, b);
                reveal(subst);
                bind_witness(input, key, b, out, bindings);
                bindings_prepend(key, b, bindings, limit);
                seq![(key, b)] + bindings
            }
        },
        (_, Term::Var(key)) => {
            if occurs(key, a) { assert(false); return Seq::empty(); }
            reveal(nvars);
            assert(key < limit);
            state_bind(input, key, a, limit);
            let next = u_bind(input, rest, key, a);
            let bindings = unify_n_witness(next, left, limit);
            subst_absent(a, key, a);
            reveal(subst);
            bind_witness(input, key, a, out, bindings);
            bindings_prepend(key, a, bindings, limit);
            seq![(key, a)] + bindings
        },
        (Term::Comp(name, xs), Term::Comp(other, ys)) => {
            if name != other || xs.len() != ys.len() { assert(false); return Seq::empty(); }
            state_comp(input, name, xs, ys, limit);
            let next = UState { pairs: zip(xs, ys) + rest, ..input };
            let bindings = unify_n_witness(next, left, limit);
            comp_witness(input, name, xs, ys, out, bindings);
            bindings
        },
        _ => {
            if a != b { assert(false); return Seq::empty(); }
            let next = UState { pairs: rest, ..input };
            let bindings = unify_n_witness(next, left, limit);
            skip_witness(input, out, bindings);
            bindings
        },
    }
}

pub proof fn unify_witness(input: UState, limit: nat) -> (bindings: Seq<(nat, Term)>)
    requires unify(input) is Ok, state_bounded(input, limit),
    ensures witness(input, unify(input), bindings), bindings_bounded(bindings, limit),
{
    reveal(unify);
    let fuel = choose|f: nat| !(unify_n(input, f) is Out);
    unify_n_witness(input, fuel, limit)
}

pub open spec fn apply_goal(goal: ckc_spec::trace::TGoal, bindings: Seq<(nat, Term)>) -> ckc_spec::trace::TGoal {
    match goal {
        ckc_spec::trace::TGoal::Lit(t, depth, path) => ckc_spec::trace::TGoal::Lit(apply(t, bindings), depth, path),
        ckc_spec::trace::TGoal::NafCut(level) => ckc_spec::trace::TGoal::NafCut(level),
    }
}

pub proof fn tunify_witness(pairs: Seq<(Term, Term)>, goals: Seq<ckc_spec::trace::TGoal>, limit: nat) -> (bindings: Seq<(nat, Term)>)
    requires
        ckc_spec::trace::tunify(pairs, goals) is Ok,
        state_bounded(UState { pairs, stack: Seq::empty(), sol: goals.map_values(|g: ckc_spec::trace::TGoal| ckc_spec::trace::tgoal_term(g)) }, limit),
    ensures
        solved(pairs, bindings), bindings_bounded(bindings, limit),
        ckc_spec::trace::tunify(pairs, goals) == ckc_spec::trace::TUni::Ok(goals.map_values(|g: ckc_spec::trace::TGoal| apply_goal(g, bindings))),
{
    let input = UState { pairs, stack: Seq::empty(), sol: goals.map_values(|g: ckc_spec::trace::TGoal| ckc_spec::trace::tgoal_term(g)) };
    reveal(ckc_spec::trace::tunify);
    assert(unify(input) is Ok);
    let bindings = unify_witness(input, limit);
    let terms = applied_all(input.sol, bindings);
    assert_seqs_equal!(Seq::new(goals.len(), |i: int| ckc_spec::trace::tgoal_with(goals[i], terms[i]))
        == goals.map_values(|g: ckc_spec::trace::TGoal| apply_goal(g, bindings)), i => {
            match goals[i] {
                ckc_spec::trace::TGoal::Lit(_, _, _) => {},
                ckc_spec::trace::TGoal::NafCut(_) => {},
            }
        });
    bindings
}

} // verus!
