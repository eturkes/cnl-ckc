use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::root_ok;
use crate::k3_state::EGoal;
#[cfg(verus_keep_ghost)]
use crate::k3_state::{goal_valid, goal_view, goals_valid, goals_view};
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::{occurs, occurs_all};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::{Term, firsts, var_stream, var_stream_all};
use ckc_spec::trace::*;
use vstd::prelude::*;

verus! {

proof fn stream_occurs(t: Term, x: nat)
    ensures
        var_stream(t).contains(x) == occurs(x, t),
    decreases t,
{
    match t {
        Term::Var(k) => {
            assert(var_stream(t) == seq![k]);
            assert(seq![k][0] == k);
        },
        Term::Comp(_, args) => {
            stream_all_occurs(args, x);
        },
        _ => {
            assert(var_stream(t) == Seq::<nat>::empty());
        },
    }
}

proof fn stream_all_occurs(ts: Seq<Term>, x: nat)
    ensures
        var_stream_all(ts).contains(x) == occurs_all(x, ts),
    decreases ts,
{
    if ts.len() > 0 {
        stream_occurs(ts[0], x);
        stream_all_occurs(ts.drop_first(), x);
        vstd::seq_lib::lemma_seq_concat_contains_all_elements(
            var_stream(ts[0]),
            var_stream_all(ts.drop_first()),
            x,
        );
    } else {
        assert(var_stream_all(ts) == Seq::<nat>::empty());
    }
}

fn shares_var(arena: &ETermArena, a: usize, b: usize) -> (out: bool)
    requires
        root_ok(arena, a),
        root_ok(arena, b),
    ensures
        out == shares(arena@[a as int], arena@[b as int]),
{
    let ghost ta = arena@[a as int];
    let ghost tb = arena@[b as int];
    let keys = crate::k3_number::collect(arena, a);
    let mut i = 0usize;
    while i < keys.len()
        invariant
            root_ok(arena, a),
            root_ok(arena, b),
            ta == arena@[a as int],
            tb == arena@[b as int],
            crate::k3_number::keys_view(keys@) == firsts(var_stream(ta), Set::empty()),
            i <= keys.len(),
            forall|j: int| 0 <= j < i ==> !occurs(#[trigger] keys@[j] as nat, tb),
        decreases keys.len() - i,
    {
        if crate::k2_engine::occurs_root(arena, keys[i], b) {
            proof {
                let x = keys@[i as int] as nat;
                assert(crate::k3_number::keys_view(keys@)[i as int] == x);
                crate::k3_number::firsts_member(var_stream(ta), Set::empty(), x);
                stream_occurs(ta, x);
                assert(occurs(x, ta) && occurs(x, tb));
            }
            return true;
        }
        i += 1;
    }
    proof {
        assert forall|x: nat| !(occurs(x, ta) && occurs(x, tb)) by {
            if occurs(x, ta) {
                stream_occurs(ta, x);
                crate::k3_number::firsts_member(var_stream(ta), Set::empty(), x);
                let ks = crate::k3_number::keys_view(keys@);
                let j = ks.index_of(x);
                assert(keys@[j] as nat == x);
            }
        }
    }
    false
}

// `naf_safe` over the exec stack: no Lit goal of `rest` before its first naf
// cut shares a variable with `inner`.
pub fn naf_safe_exec(arena: &ETermArena, inner: usize, rest: &Vec<EGoal>) -> (out: bool)
    requires
        root_ok(arena, inner),
        goals_valid(arena.nodes@, rest@),
    ensures
        out == naf_safe(arena@[inner as int], goals_view(arena.nodes@, rest@)),
{
    let ghost t = arena@[inner as int];
    let ghost view = goals_view(arena.nodes@, rest@);
    let mut i = 0usize;
    while i < rest.len()
        invariant
            root_ok(arena, inner),
            goals_valid(arena.nodes@, rest@),
            t == arena@[inner as int],
            view == goals_view(arena.nodes@, rest@),
            i <= rest.len(),
            !in_naf(view.take(i as int)),
            forall|j: int| 0 <= j < i ==> !shares(t, #[trigger] tgoal_term(view[j])),
        decreases rest.len() - i,
    {
        proof {
            assert(goal_valid(arena.nodes@, &rest@[i as int]));
            assert(view[i as int] == goal_view(arena.nodes@, &rest@[i as int]));
        }
        match &rest[i] {
            EGoal::Lit { root, .. } => {
                if shares_var(arena, inner, *root) {
                    proof {
                        assert(tgoal_term(view[i as int]) == arena@[*root as int]);
                        assert(!naf_safe(t, view));
                    }
                    return false;
                }
                proof {
                    assert(tgoal_term(view[i as int]) == arena@[*root as int]);
                    assert(view.take(i + 1) =~= view.take(i as int).push(view[i as int]));
                    if in_naf(view.take(i + 1)) {
                        let k = choose|k: int|
                            0 <= k < view.take(i + 1).len() && #[trigger] view.take(
                                i + 1,
                            )[k] is NafCut;
                        if k < i {
                            assert(view.take(i as int)[k] == view.take(i + 1)[k]);
                        }
                    }
                }
            },
            EGoal::NafCut { .. } => {
                proof {
                    assert(view[i as int] is NafCut);
                    assert forall|j: int|
                        0 <= j < view.len() && !in_naf(view.take(j)) implies !shares(
                        t,
                        #[trigger] tgoal_term(view[j]),
                    ) by {
                        if j > i {
                            assert(view.take(j)[i as int] == view[i as int]);
                        } else if j == i {
                            assert(tgoal_term(view[j]) == Term::Nil);
                        }
                    }
                }
                return true;
            },
        }
        i += 1;
    }
    proof {
        assert forall|j: int| 0 <= j < view.len() && !in_naf(view.take(j)) implies !shares(
            t,
            #[trigger] tgoal_term(view[j]),
        ) by {}
    }
    true
}

} // verus!
