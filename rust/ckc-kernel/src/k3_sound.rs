use ckc_spec::engine::*;
use ckc_spec::term::*;
use ckc_spec::trace::*;
use ckc_spec::v1text::*;
use vstd::prelude::*;

#[path = "k3_sound_fresh.rs"]
pub mod freshness;

#[path = "k3_sound_unify.rs"]
pub mod unification;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn apply_append(t: Term, s: Seq<(nat, Term)>, r: Seq<(nat, Term)>)
    ensures
        apply(t, s + r) == apply(apply(t, s), r),
    decreases s.len(),
{
    if s.len() > 0 {
        assert((s + r).drop_first() =~= s.drop_first() + r);
        apply_append(subst(t, s[0].0, s[0].1), s.drop_first(), r);
    } else {
        assert(s + r =~= r);
    }
}

pub proof fn apply_cons(t: Term, x: nat, v: Term, s: Seq<(nat, Term)>)
    ensures
        apply(t, seq![(x, v)] + s) == apply(subst(t, x, v), s),
{
    assert((seq![(x, v)] + s).drop_first() =~= s);
}

pub proof fn apply_extension(a: Term, b: Term, s: Seq<(nat, Term)>, r: Seq<(nat, Term)>)
    requires
        apply(a, s) == apply(b, s),
    ensures
        apply(a, s + r) == apply(b, s + r),
{
    apply_append(a, s, r);
    apply_append(b, s, r);
}

pub proof fn k3_sound_proof(db: Seq<DocClause>, goal: Term)
    requires
        bodies_wf(db),
        derived_forest(db, goal) is Some,
    ensures
        forest_valid(db, goal, derived_forest(db, goal).unwrap()),
{
    assert(false);
}

} // verus!
