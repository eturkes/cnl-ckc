use ckc_spec::engine::*;
use ckc_spec::term::*;
use ckc_spec::trace::*;
use ckc_spec::v1text::*;
use vstd::prelude::*;

verus! {

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
