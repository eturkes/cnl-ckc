use super::unification::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn nvars_all_index(ts: Seq<Term>, i: int)
    requires
        0 <= i < ts.len(),
    ensures
        nvars(ts[i]) <= nvars_all(ts),
    decreases ts.len(),
{
    if i > 0 {
        nvars_all_index(ts.drop_first(), i - 1);
    }
}

pub proof fn nvars_all_upper(ts: Seq<Term>, b: nat)
    requires
        forall|i: int| 0 <= i < ts.len() ==> #[trigger] nvars(ts[i]) <= b,
    ensures
        nvars_all(ts) <= b,
    decreases ts.len(),
{
    if ts.len() > 0 {
        assert forall|i: int| 0 <= i < ts.drop_first().len() implies #[trigger] nvars(
            ts.drop_first()[i],
        ) <= b by {
            assert(ts.drop_first()[i] == ts[i + 1]);
        }
        nvars_all_upper(ts.drop_first(), b);
        assert(nvars(ts[0]) <= b);
    }
}

pub proof fn subst_below(t: Term, x: nat, v: Term, b: nat)
    requires
        nvars(t) <= b,
        nvars(v) <= b,
    ensures
        nvars(subst(t, x, v)) <= b,
    decreases t,
{
    if let Term::Comp(_, args) = t {
        subst_all_below(args, x, v, b);
    }
}

pub proof fn subst_all_below(ts: Seq<Term>, x: nat, v: Term, b: nat)
    requires
        nvars_all(ts) <= b,
        nvars(v) <= b,
    ensures
        nvars_all(subst_all(ts, x, v)) <= b,
    decreases ts,
{
    if ts.len() > 0 {
        subst_below(ts[0], x, v, b);
        subst_all_below(ts.drop_first(), x, v, b);
        reveal_with_fuel(nvars_all, 2);
        assert((seq![subst(ts[0], x, v)] + subst_all(ts.drop_first(), x, v)).drop_first()
            =~= subst_all(ts.drop_first(), x, v));
    }
}

pub proof fn shift_bound(t: Term, off: nat)
    ensures
        nvars(shift(t, off)) <= nvars(t) + off,
    decreases t,
{
    if let Term::Comp(_, args) = t {
        shift_all_bound(args, off);
    }
}

pub proof fn shift_all_bound(ts: Seq<Term>, off: nat)
    ensures
        nvars_all(shift_all(ts, off)) <= nvars_all(ts) + off,
    decreases ts,
{
    if ts.len() > 0 {
        shift_bound(ts[0], off);
        shift_all_bound(ts.drop_first(), off);
        reveal_with_fuel(nvars_all, 2);
        assert((seq![shift(ts[0], off)] + shift_all(ts.drop_first(), off)).drop_first()
            =~= shift_all(ts.drop_first(), off));
    }
}

pub proof fn shifted_absent(t: Term, off: nat, x: nat)
    requires
        x < off,
    ensures
        !occurs(x, shift(t, off)),
    decreases t,
{
    if let Term::Comp(_, args) = t {
        shifted_all_absent(args, off, x);
    }
}

pub proof fn shifted_all_absent(ts: Seq<Term>, off: nat, x: nat)
    requires
        x < off,
    ensures
        !occurs_all(x, shift_all(ts, off)),
    decreases ts,
{
    if ts.len() > 0 {
        shifted_absent(ts[0], off, x);
        shifted_all_absent(ts.drop_first(), off, x);
        reveal_with_fuel(occurs_all, 2);
        assert((seq![shift(ts[0], off)] + shift_all(ts.drop_first(), off)).drop_first()
            =~= shift_all(ts.drop_first(), off));
    }
}

pub open spec fn binds_below(s: Seq<(nat, Term)>, b: nat) -> bool {
    forall|i: int| 0 <= i < s.len() ==> (#[trigger] s[i]).0 < b && nvars(s[i].1) <= b
}

pub proof fn binds_tail(s: Seq<(nat, Term)>, b: nat)
    requires
        s.len() > 0,
        binds_below(s, b),
    ensures
        binds_below(s.drop_first(), b),
{
    assert forall|i: int| 0 <= i < s.drop_first().len() implies (#[trigger] s.drop_first()[i]).0 < b
        && nvars(s.drop_first()[i].1) <= b by {
        assert(s.drop_first()[i] == s[i + 1]);
    }
}

pub proof fn binds_append(s: Seq<(nat, Term)>, r: Seq<(nat, Term)>, b: nat)
    requires
        binds_below(s, b),
        binds_below(r, b),
    ensures
        binds_below(s + r, b),
{
    assert forall|i: int| 0 <= i < (s + r).len() implies (#[trigger] (s + r)[i]).0 < b && nvars(
        (s + r)[i].1,
    ) <= b by {
        if i < s.len() {
            assert((s + r)[i] == s[i]);
        } else {
            assert((s + r)[i] == r[i - s.len()]);
        }
    }
}

pub proof fn apply_below(t: Term, s: Seq<(nat, Term)>, b: nat)
    requires
        nvars(t) <= b,
        binds_below(s, b),
    ensures
        nvars(apply(t, s)) <= b,
    decreases s.len(),
{
    if s.len() > 0 {
        binds_tail(s, b);
        subst_below(t, s[0].0, s[0].1, b);
        apply_below(subst(t, s[0].0, s[0].1), s.drop_first(), b);
    }
}

pub proof fn apply_fresh(t: Term, off: nat, s: Seq<(nat, Term)>)
    requires
        binds_below(s, off),
    ensures
        apply(shift(t, off), s) == shift(t, off),
    decreases s.len(),
{
    if s.len() > 0 {
        binds_tail(s, off);
        shifted_absent(t, off, s[0].0);
        subst_absent(shift(t, off), s[0].0, s[0].1);
        apply_fresh(t, off, s.drop_first());
    }
}

} // verus!
