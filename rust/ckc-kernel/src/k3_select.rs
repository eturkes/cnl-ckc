// Goal selection over the K3 trace stack (contract eso D4): the engine's
// selector on the erased stack, scanned lazily.
use crate::k2_select::{binds_root, is_comp_root, is_naf_root};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{ENode, arena_ok, child_terms, node_ok};
use crate::k2_term::{ENodeKind, ETermArena};
use crate::k3_state::EGoal;
#[cfg(verus_keep_ghost)]
use crate::k3_state::{goal_valid, goal_view, goals_valid, goals_view};
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::{Goal, operator_name, run_binds, selected, waits};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use ckc_spec::trace::{TGoal, erase, tselected};
use vstd::prelude::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn erased(nodes: Seq<ENode>, goals: Seq<EGoal>) -> Seq<Goal> {
    goals_view(nodes, goals).map_values(|g: TGoal| erase(g))
}

pub fn run_binds_from(arena: &ETermArena, goals: &Vec<EGoal>, from: usize, key: usize) -> (out:
    bool)
    requires
        arena_ok(arena),
        goals_valid(arena.nodes@, goals@),
        from <= goals.len(),
    ensures
        out == run_binds(erased(arena.nodes@, goals@).skip(from as int), Term::Var(key as nat)),
{
    let ghost x = Term::Var(key as nat);
    let ghost view = erased(arena.nodes@, goals@);
    let mut j = from;
    while j < goals.len()
        invariant
            arena_ok(arena),
            goals_valid(arena.nodes@, goals@),
            view == erased(arena.nodes@, goals@),
            from <= j <= goals.len(),
            x == Term::Var(key as nat),
            run_binds(view.skip(from as int), x) == run_binds(view.skip(j as int), x),
        decreases goals.len() - j,
    {
        let ghost s = view.skip(j as int);
        proof {
            reveal_with_fuel(run_binds, 1);
            assert(s[0] == view[j as int]);
            assert(s.drop_first() =~= view.skip(j as int + 1));
            assert(goal_valid(arena.nodes@, &goals@[j as int]));
        }
        match &goals[j] {
            EGoal::Lit { root, .. } => {
                let root = *root;
                if !is_comp_root(arena, root) || is_naf_root(arena, root) {
                    return false;
                }
                if binds_root(arena, root, key) {
                    return true;
                }
            },
            EGoal::NafCut { .. } => {
                return false;
            },
        }
        j += 1;
    }
    proof {
        reveal_with_fuel(run_binds, 1);
        assert(view.skip(j as int).len() == 0);
    }
    false
}

pub fn waits_at(arena: &ETermArena, goals: &Vec<EGoal>, k: usize) -> (out: bool)
    requires
        arena_ok(arena),
        goals_valid(arena.nodes@, goals@),
        k < goals.len(),
    ensures
        out == waits(
            erased(arena.nodes@, goals@)[k as int],
            erased(arena.nodes@, goals@).skip(k as int + 1),
        ),
{
    proof {
        assert(goal_valid(arena.nodes@, &goals@[k as int]));
    }
    match &goals[k] {
        EGoal::Lit { root, .. } => {
            let root = *root;
            let op: &[u8] = b"guideline_operator";
            proof {
                reveal_byteslit(b"guideline_operator");
                reveal_strlit("guideline_operator");
                reveal(ckc_spec::v1text::ascii);
                reveal(operator_name);
                assert(op@ == operator_name());
            }
            if !crate::k3_front::is_comp(arena, root, op, 3) {
                return false;
            }
            proof {
                assert(node_ok(arena.nodes@, root as int));
                reveal(node_ok);
            }
            match &arena.nodes[root].kind {
                ENodeKind::Comp { name, child_roots, .. } => {
                    proof {
                        crate::k2_engine::node_comp_model(
                            arena.nodes@,
                            root as int,
                            name@,
                            child_roots@,
                        );
                        reveal(child_terms);
                    }
                    let inner = child_roots[1];
                    proof {
                        assert(node_ok(arena.nodes@, inner as int));
                    }
                    match &arena.nodes[inner].kind {
                        ENodeKind::Var { key, .. } => {
                            proof {
                                assert(arena@[inner as int] == Term::Var(*key as nat));
                            }
                            run_binds_from(arena, goals, k + 1, *key)
                        },
                        _ => false,
                    }
                },
                _ => false,
            }
        },
        EGoal::NafCut { .. } => false,
    }
}

pub fn tselected_exec(arena: &ETermArena, goals: &Vec<EGoal>) -> (k: usize)
    requires
        arena_ok(arena),
        goals_valid(arena.nodes@, goals@),
    ensures
        k == tselected(goals_view(arena.nodes@, goals@)),
{
    let ghost view = erased(arena.nodes@, goals@);
    proof {
        reveal(tselected);
        assert(tselected(goals_view(arena.nodes@, goals@)) == selected(view));
    }
    if goals.len() == 0 {
        proof {
            crate::k2_select::selected_unfold(view);
        }
        return 0;
    }
    let mut k = 0usize;
    let mut stop = false;
    proof {
        assert(view.skip(0) =~= view);
    }
    while !stop && k + 1 < goals.len()
        invariant
            arena_ok(arena),
            goals_valid(arena.nodes@, goals@),
            view == erased(arena.nodes@, goals@),
            k < goals.len(),
            selected(view) == k + selected(view.skip(k as int)),
            stop ==> !waits(view[k as int], view.skip(k as int + 1)),
        decreases
                2 * (goals.len() - k) + if stop {
                    0int
                } else {
                    1int
                },
    {
        if waits_at(arena, goals, k) {
            proof {
                crate::k2_select::selected_step(view, k as int);
            }
            k += 1;
        } else {
            stop = true;
        }
    }
    proof {
        crate::k2_select::selected_stop(view, k as int);
    }
    k
}

} // verus!
