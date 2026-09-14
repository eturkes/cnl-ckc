#[cfg(verus_keep_ghost)]
use ckc_spec::{engine::*, term::Term, trace::apply};
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn terms_bounded(terms: Seq<Term>, limit: nat) -> bool {
    forall|i: int| 0 <= i < terms.len() ==> #[trigger] nvars(terms[i]) <= limit
}

pub open spec fn pairs_bounded(pairs: Seq<(Term, Term)>, limit: nat) -> bool {
    forall|i: int|
        0 <= i < pairs.len() ==> {
            &&& nvars(#[trigger] pairs[i].0) <= limit
            &&& nvars(pairs[i].1) <= limit
        }
}

pub open spec fn state_bounded(state: UState, limit: nat) -> bool {
    pairs_bounded(state.pairs, limit) && terms_bounded(state.sol, limit)
}

pub open spec fn bindings_bounded(bindings: Seq<(nat, Term)>, limit: nat) -> bool {
    forall|i: int|
        0 <= i < bindings.len() ==> {
            &&& (#[trigger] bindings[i]).0 < limit
            &&& nvars(bindings[i].1) <= limit
        }
}

pub proof fn nvars_all_iff(terms: Seq<Term>, limit: nat)
    ensures
        (nvars_all(terms) <= limit) == terms_bounded(terms, limit),
    decreases terms.len(),
{
    if terms.len() > 0 {
        nvars_all_iff(terms.drop_first(), limit);
        assert(terms_bounded(terms, limit) == (nvars(terms[0]) <= limit && terms_bounded(
            terms.drop_first(),
            limit,
        ))) by {
            if terms_bounded(terms, limit) {
                assert forall|i: int| 0 <= i < terms.drop_first().len() implies #[trigger] nvars(
                    terms.drop_first()[i],
                ) <= limit by {
                    assert(terms.drop_first()[i] == terms[i + 1]);
                }
            } else if nvars(terms[0]) <= limit && terms_bounded(terms.drop_first(), limit) {
                assert forall|i: int| 0 <= i < terms.len() implies #[trigger] nvars(terms[i])
                    <= limit by {
                    if i > 0 {
                        assert(terms.drop_first()[i - 1] == terms[i]);
                    }
                }
            }
        }
    }
    reveal(nvars_all);
    reveal(max_nat);
}

pub proof fn subst_bounded(term: Term, key: nat, value: Term, limit: nat)
    requires
        nvars(term) <= limit,
        nvars(value) <= limit,
    ensures
        nvars(subst(term, key, value)) <= limit,
    decreases term,
{
    if let Term::Comp(_, args) = term {
        reveal(nvars);
        nvars_all_iff(args, limit);
        subst_all_bounded(args, key, value, limit);
        nvars_all_iff(subst_all(args, key, value), limit);
    }
    reveal(subst);
    reveal(nvars);
}

pub proof fn subst_all_bounded(terms: Seq<Term>, key: nat, value: Term, limit: nat)
    requires
        terms_bounded(terms, limit),
        nvars(value) <= limit,
    ensures
        terms_bounded(subst_all(terms, key, value), limit),
        subst_all(terms, key, value).len() == terms.len(),
    decreases terms,
{
    if terms.len() > 0 {
        assert(nvars(terms[0]) <= limit);
        assert forall|i: int| 0 <= i < terms.drop_first().len() implies #[trigger] nvars(
            terms.drop_first()[i],
        ) <= limit by {
            assert(terms.drop_first()[i] == terms[i + 1]);
        }
        subst_bounded(terms[0], key, value, limit);
        subst_all_bounded(terms.drop_first(), key, value, limit);
        reveal(subst_all);
        assert forall|i: int| 0 <= i < subst_all(terms, key, value).len() implies #[trigger] nvars(
            subst_all(terms, key, value)[i],
        ) <= limit by {
            if i > 0 {
                assert(subst_all(terms, key, value)[i] == subst_all(
                    terms.drop_first(),
                    key,
                    value,
                )[i - 1]);
            }
        }
    } else {
        reveal(subst_all);
    }
}

pub proof fn state_tail(input: UState, limit: nat)
    requires
        state_bounded(input, limit),
        input.pairs.len() > 0,
    ensures
        state_bounded(UState { pairs: input.pairs.drop_first(), ..input }, limit),
{
    assert forall|i: int| 0 <= i < input.pairs.drop_first().len() implies {
        &&& nvars(#[trigger] input.pairs.drop_first()[i].0) <= limit
        &&& nvars(input.pairs.drop_first()[i].1) <= limit
    } by {
        assert(input.pairs.drop_first()[i] == input.pairs[i + 1]);
        assert(nvars(input.pairs[i + 1].0) <= limit);
        assert(nvars(input.pairs[i + 1].1) <= limit);
    }
}

pub proof fn state_bind(input: UState, key: nat, value: Term, limit: nat)
    requires
        state_bounded(input, limit),
        input.pairs.len() > 0,
        nvars(value) <= limit,
    ensures
        state_bounded(u_bind(input, input.pairs.drop_first(), key, value), limit),
{
    let next = u_bind(input, input.pairs.drop_first(), key, value);
    subst_all_bounded(input.sol, key, value, limit);
    assert forall|i: int| 0 <= i < next.pairs.len() implies {
        &&& nvars(#[trigger] next.pairs[i].0) <= limit
        &&& nvars(next.pairs[i].1) <= limit
    } by {
        assert(input.pairs.drop_first()[i] == input.pairs[i + 1]);
        assert(nvars(input.pairs[i + 1].0) <= limit);
        assert(nvars(input.pairs[i + 1].1) <= limit);
        subst_bounded(input.pairs[i + 1].0, key, value, limit);
        subst_bounded(input.pairs[i + 1].1, key, value, limit);
        assert(next.pairs[i] == (
            subst(input.pairs[i + 1].0, key, value),
            subst(input.pairs[i + 1].1, key, value),
        ));
    }
}

pub proof fn state_comp(input: UState, name: Seq<u8>, xs: Seq<Term>, ys: Seq<Term>, limit: nat)
    requires
        state_bounded(input, limit),
        input.pairs.len() > 0,
        input.pairs[0] == (Term::Comp(name, xs), Term::Comp(name, ys)),
        xs.len() == ys.len(),
    ensures
        state_bounded(UState { pairs: zip(xs, ys) + input.pairs.drop_first(), ..input }, limit),
{
    assert(nvars(input.pairs[0].0) <= limit);
    assert(nvars(input.pairs[0].1) <= limit);
    reveal(nvars);
    nvars_all_iff(xs, limit);
    nvars_all_iff(ys, limit);
    let pairs = zip(xs, ys) + input.pairs.drop_first();
    assert forall|i: int| 0 <= i < pairs.len() implies {
        &&& nvars(#[trigger] pairs[i].0) <= limit
        &&& nvars(pairs[i].1) <= limit
    } by {
        if i < xs.len() {
            assert(pairs[i] == (xs[i], ys[i]));
        } else {
            assert(pairs[i] == input.pairs[i - xs.len() + 1]);
        }
    }
}

pub proof fn bindings_prepend(key: nat, value: Term, bindings: Seq<(nat, Term)>, limit: nat)
    requires
        key < limit,
        nvars(value) <= limit,
        bindings_bounded(bindings, limit),
    ensures
        bindings_bounded(seq![(key, value)] + bindings, limit),
{
    assert forall|i: int| 0 <= i < (seq![(key, value)] + bindings).len() implies {
        &&& (#[trigger] (seq![(key, value)] + bindings)[i]).0 < limit
        &&& nvars((seq![(key, value)] + bindings)[i].1) <= limit
    } by {
        if i > 0 {
            assert((seq![(key, value)] + bindings)[i] == bindings[i - 1]);
        }
    }
}

pub proof fn bindings_concat(a: Seq<(nat, Term)>, b: Seq<(nat, Term)>, limit: nat)
    requires
        bindings_bounded(a, limit),
        bindings_bounded(b, limit),
    ensures
        bindings_bounded(a + b, limit),
{
    assert forall|i: int| 0 <= i < (a + b).len() implies {
        &&& (#[trigger] (a + b)[i]).0 < limit
        &&& nvars((a + b)[i].1) <= limit
    } by {
        if i < a.len() {
            assert((a + b)[i] == a[i]);
        } else {
            assert((a + b)[i] == b[i - a.len()]);
        }
    }
}

pub proof fn bindings_weaken(bindings: Seq<(nat, Term)>, before: nat, after: nat)
    requires
        bindings_bounded(bindings, before),
        before <= after,
    ensures
        bindings_bounded(bindings, after),
{
    assert forall|i: int| 0 <= i < bindings.len() implies {
        &&& (#[trigger] bindings[i]).0 < after
        &&& nvars(bindings[i].1) <= after
    } by {
        assert(bindings[i].0 < before);
        assert(nvars(bindings[i].1) <= before);
    }
}

pub open spec fn above(term: Term, floor: nat) -> bool
    decreases term,
{
    match term {
        Term::Var(key) => key >= floor,
        Term::Comp(_, args) => above_all(args, floor),
        _ => true,
    }
}

pub open spec fn above_all(terms: Seq<Term>, floor: nat) -> bool
    decreases terms,
{
    terms.len() == 0 || (above(terms[0], floor) && above_all(terms.drop_first(), floor))
}

pub proof fn shift_above(term: Term, floor: nat)
    ensures
        above(shift(term, floor), floor),
    decreases term,
{
    if let Term::Comp(_, args) = term {
        shift_all_above(args, floor);
    }
    reveal(shift);
    reveal(above);
}

proof fn shift_all_above(terms: Seq<Term>, floor: nat)
    ensures
        above_all(shift_all(terms, floor), floor),
    decreases terms,
{
    if terms.len() > 0 {
        shift_above(terms[0], floor);
        shift_all_above(terms.drop_first(), floor);
        reveal(shift_all);
        assert(shift_all(terms, floor).len() > 0);
        assert(shift_all(terms, floor)[0] == shift(terms[0], floor));
        assert_seqs_equal!(shift_all(terms, floor).drop_first() == shift_all(terms.drop_first(), floor));
    }
    reveal(shift_all);
    reveal(above_all);
}

proof fn above_absent(term: Term, floor: nat, key: nat)
    requires
        above(term, floor),
        key < floor,
    ensures
        !occurs(key, term),
    decreases term,
{
    if let Term::Comp(_, args) = term {
        above_all_absent(args, floor, key);
    }
    reveal(above);
    reveal(occurs);
}

proof fn above_all_absent(terms: Seq<Term>, floor: nat, key: nat)
    requires
        above_all(terms, floor),
        key < floor,
    ensures
        !occurs_all(key, terms),
    decreases terms,
{
    if terms.len() > 0 {
        reveal(above_all);
        above_absent(terms[0], floor, key);
        above_all_absent(terms.drop_first(), floor, key);
    }
    reveal(occurs_all);
}

pub proof fn apply_fresh(term: Term, bindings: Seq<(nat, Term)>, floor: nat)
    requires
        above(term, floor),
        bindings_bounded(bindings, floor),
    ensures
        apply(term, bindings) == term,
    decreases bindings.len(),
{
    if bindings.len() > 0 {
        assert(bindings[0].0 < floor);
        above_absent(term, floor, bindings[0].0);
        crate::k3_sound_unify::subst_absent(term, bindings[0].0, bindings[0].1);
        assert forall|i: int| 0 <= i < bindings.drop_first().len() implies {
            &&& (#[trigger] bindings.drop_first()[i]).0 < floor
            &&& nvars(bindings.drop_first()[i].1) <= floor
        } by {
            assert(bindings.drop_first()[i] == bindings[i + 1]);
        }
        apply_fresh(term, bindings.drop_first(), floor);
    }
    reveal(apply);
}

} // verus!
