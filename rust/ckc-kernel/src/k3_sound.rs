use ckc_spec::engine::*;
use ckc_spec::term::*;
use ckc_spec::trace::*;
use ckc_spec::v1text::*;
use vstd::prelude::*;

#[path = "k3_sound_body.rs"]
pub mod body;

#[path = "k3_sound_bound.rs"]
pub mod bounded;

#[path = "k3_sound_control.rs"]
pub mod control;

#[path = "k3_sound_fresh.rs"]
pub mod freshness;

#[path = "k3_sound_frontier.rs"]
pub mod frontier;

#[path = "k3_sound_graph.rs"]
pub mod graph;

#[path = "k3_sound_goals.rs"]
pub mod goals;

#[path = "k3_sound_log.rs"]
pub mod log;

#[path = "k3_sound_machine.rs"]
pub mod machine;

#[path = "k3_sound_record.rs"]
pub mod record;

#[path = "k3_sound_shape.rs"]
pub mod shapes;

#[path = "k3_sound_state.rs"]
pub mod state;

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

pub proof fn naf_log_sound(db: Seq<DocClause>, goal: Term, log: Seq<(Seq<nat>, TEv)>)
    requires
        bodies_wf(db),
        ckc_spec::answers::goal_walk(goal) is None,
        trun(db, roots_cfg(conj_leaves(goal)), trace_inf()).0 == TOut::Proved(log),
    ensures
        graph::naf_log_ok(db, log),
{
    assert(false);
}

pub proof fn k3_sound_proof(db: Seq<DocClause>, goal: Term)
    requires
        bodies_wf(db),
        ckc_spec::answers::goal_walk(goal) is None,
        derived_forest(db, goal) is Some,
    ensures
        forest_valid(db, goal, derived_forest(db, goal).unwrap()),
{
    let roots = conj_leaves(goal);
    machine::roots_weak(db, goal);
    match trun(db, roots_cfg(roots), trace_inf()).0 {
        TOut::Proved(log) => {
            let cert = choose|cert: state::Cert| #[trigger]
                machine::complete_cert(db, roots, log, cert);
            naf_log_sound(db, goal, log);
            graph::forest_of_sound(db, goal, log, cert.offsets, cert.theta);
            assert(derived_forest(db, goal) == Option::Some(forest_of(db, roots, log)));
        },
        _ => {},
    }
}

} // verus!
