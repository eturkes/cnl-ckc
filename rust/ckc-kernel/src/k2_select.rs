// Goal selection (contract eso): selector facts over the spec stack + exec
// scans computing `ckc_spec::engine::selected` on the K2 stack.
use crate::k2_engine::EGoal;
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{
    goal_valid, goal_view, goals_valid, goals_view, node_comp_model, roots_work, term_size,
};
use crate::k2_term::{ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, child_terms, node_ok, root_ok};
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::{
    Goal, binds, comma_name, naf_name, operator_name, run_binds, selected, waits,
};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};
// --- selector facts ---

pub open spec fn run_goal(g: Goal) -> bool {
    match g {
        Goal::Lit(Term::Comp(name, args), _) => !(name == naf_name() && args.len() == 1),
        _ => false,
    }
}

pub proof fn run_binds_front(gs: Seq<Goal>, x: Term)
    requires
        run_binds(gs, x),
    ensures
        gs.len() > 0,
        run_goal(gs[0]),
{
    reveal_with_fuel(run_binds, 1);
}

pub proof fn selected_unfold(stack: Seq<Goal>)
    ensures
        selected(stack) == if stack.len() > 1 && waits(stack[0], stack.drop_first()) {
            1 + selected(stack.drop_first())
        } else {
            0
        },
{
    reveal_with_fuel(selected, 1);
}

// A nonempty stack selects an index in range; a goal selected past the front
// is a run goal (a predicate literal or a conjunction); the skipped goals wait.
pub proof fn selected_facts(stack: Seq<Goal>)
    requires
        stack.len() > 0,
    ensures
        selected(stack) < stack.len(),
        selected(stack) > 0 ==> run_goal(stack[selected(stack) as int]),
        forall|i: int| 0 <= i < selected(stack) ==> waits(#[trigger] stack[i], stack.skip(i + 1)),
    decreases stack.len(),
{
    selected_unfold(stack);
    if stack.len() > 1 && waits(stack[0], stack.drop_first()) {
        let tail = stack.drop_first();
        selected_facts(tail);
        let k = selected(tail);
        match stack[0] {
            Goal::Lit(Term::Comp(_, args), _) => {
                run_binds_front(tail, args[1]);
            },
            _ => {},
        }
        assert(stack[k as int + 1] == tail[k as int]);
        assert forall|i: int| 0 <= i < selected(stack) implies waits(
            #[trigger] stack[i],
            stack.skip(i + 1),
        ) by {
            if i == 0 {
                assert(stack.skip(1) =~= tail);
            } else {
                assert(stack[i] == tail[i - 1]);
                assert(stack.skip(i + 1) =~= tail.skip(i));
            }
        }
    }
}

// The skip-k view of the recursion the exec loop walks.
pub proof fn selected_step(stack: Seq<Goal>, k: int)
    requires
        0 <= k < stack.len() - 1,
        waits(stack[k], stack.skip(k + 1)),
    ensures
        selected(stack.skip(k)) == 1 + selected(stack.skip(k + 1)),
{
    let s = stack.skip(k);
    selected_unfold(s);
    assert(s[0] == stack[k]);
    assert(s.drop_first() =~= stack.skip(k + 1));
}

pub proof fn selected_stop(stack: Seq<Goal>, k: int)
    requires
        0 <= k < stack.len(),
        k + 1 >= stack.len() || !waits(stack[k], stack.skip(k + 1)),
    ensures
        selected(stack.skip(k)) == 0,
{
    let s = stack.skip(k);
    selected_unfold(s);
    assert(s[0] == stack[k]);
    assert(s.drop_first() =~= stack.skip(k + 1));
}

// --- one-step unfoldings of the selector's recursive spec fns ---
pub proof fn run_binds_empty(gs: Seq<Goal>, x: Term)
    requires
        gs.len() == 0,
    ensures
        !run_binds(gs, x),
{
    reveal_with_fuel(run_binds, 1);
}

pub proof fn run_binds_cut(gs: Seq<Goal>, x: Term, l: nat)
    requires
        gs.len() > 0,
        gs[0] == Goal::NafCut(l),
    ensures
        !run_binds(gs, x),
{
    reveal_with_fuel(run_binds, 1);
}

pub proof fn run_binds_lit(gs: Seq<Goal>, x: Term, t: Term, d: nat)
    requires
        gs.len() > 0,
        gs[0] == Goal::Lit(t, d),
    ensures
        run_binds(gs, x) == match t {
            Term::Comp(name, args) => !(name == naf_name() && args.len() == 1) && !(
            ckc_spec::compare::is_builtin(name, args.len()) && !ckc_spec::term::ground(t)) && (
            binds(t, x) || run_binds(gs.drop_first(), x)),
            _ => false,
        },
{
    reveal_with_fuel(run_binds, 1);
}

pub proof fn binds_unfold(t: Term, x: Term)
    ensures
        binds(t, x) == match t {
            Term::Comp(name, args) => if name == comma_name() && args.len() == 2 {
                binds(args[0], x) || binds(args[1], x)
            } else {
                !ckc_spec::compare::is_builtin(name, args.len()) && args.len() > 0 && args[0] == x
            },
            _ => false,
        },
{
    reveal_with_fuel(binds, 1);
}

proof fn roots_binds_unfold(nodes: Seq<crate::k2_term::ENode>, roots: Seq<usize>, x: Term)
    ensures
        roots_binds(nodes, roots, x) == (roots.len() > 0 && (binds(
            nodes[roots.last() as int].term@,
            x,
        ) || roots_binds(nodes, roots.drop_last(), x))),
{
    reveal_with_fuel(roots_binds, 1);
}

// --- term scan: `binds` without recursion on term depth ---
pub open spec fn roots_binds(nodes: Seq<crate::k2_term::ENode>, roots: Seq<usize>, x: Term) -> bool
    decreases roots.len(),
{
    roots.len() > 0 && (binds(nodes[roots.last() as int].term@, x) || roots_binds(
        nodes,
        roots.drop_last(),
        x,
    ))
}

proof fn roots_work_push(nodes: Seq<crate::k2_term::ENode>, roots: Seq<usize>, r: usize)
    ensures
        roots_work(nodes, roots.push(r)) == roots_work(nodes, roots) + term_size(
            nodes[r as int].term@,
        ),
{
    crate::k2_engine::roots_work_concat(nodes, roots, seq![r]);
    assert(roots + seq![r] =~= roots.push(r));
    reveal_with_fuel(roots_work, 2);
}

proof fn roots_work_pop(nodes: Seq<crate::k2_term::ENode>, roots: Seq<usize>)
    requires
        roots.len() > 0,
    ensures
        roots_work(nodes, roots) == roots_work(nodes, roots.drop_last()) + term_size(
            nodes[roots.last() as int].term@,
        ),
{
    roots_work_push(nodes, roots.drop_last(), roots.last());
    assert(roots.drop_last().push(roots.last()) =~= roots);
}

proof fn term_size_pos(t: Term)
    ensures
        term_size(t) >= 1,
{
    reveal(term_size);
}

proof fn terms_size_two(ts: Seq<Term>)
    requires
        ts.len() == 2,
    ensures
        crate::k2_engine::terms_size(ts) == term_size(ts[0]) + term_size(ts[1]),
{
    reveal_with_fuel(crate::k2_engine::terms_size, 3);
    assert(ts.drop_first().drop_first() =~= Seq::<Term>::empty());
}

pub fn binds_root(arena: &ETermArena, root: usize, key: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == binds(arena@[root as int], Term::Var(key as nat)),
{
    let ghost x = Term::Var(key as nat);
    let ghost model = arena@[root as int];
    let mut tasks: Vec<usize> = Vec::new();
    tasks.push(root);
    proof {
        roots_binds_unfold(arena.nodes@, tasks@, x);
        assert(tasks@.drop_last() =~= Seq::<usize>::empty());
        roots_binds_unfold(arena.nodes@, tasks@.drop_last(), x);
    }
    while tasks.len() > 0
        invariant
            arena_ok(arena),
            root < arena.nodes@.len(),
            model == arena@[root as int],
            x == Term::Var(key as nat),
            forall|i: int| 0 <= i < tasks@.len() ==> #[trigger] tasks@[i] < arena.nodes@.len(),
            roots_binds(arena.nodes@, tasks@, x) == binds(model, x),
        decreases roots_work(arena.nodes@, tasks@),
    {
        let ghost before = tasks@;
        let current = tasks.pop().unwrap();
        let ghost rest = tasks@;
        let ghost t = arena@[current as int];
        proof {
            assert(current == before.last());
            assert(rest == before.drop_last());
            assert(current < arena.nodes@.len());
            roots_work_pop(arena.nodes@, before);
            roots_binds_unfold(arena.nodes@, before, x);
            assert(roots_binds(arena.nodes@, before, x) == (binds(t, x) || roots_binds(
                arena.nodes@,
                rest,
                x,
            )));
            assert(node_ok(arena.nodes@, current as int));
            term_size_pos(t);
            binds_unfold(t, x);
        }
        match &arena.nodes[current].kind {
            ENodeKind::Comp { name, child_roots, .. } => {
                let ghost args = child_terms(arena.nodes@, child_roots@);
                proof {
                    node_comp_model(arena.nodes@, current as int, name@, child_roots@);
                    reveal(child_terms);
                    assert(t == Term::Comp(name@, args));
                }
                let comma: &[u8] = b",";
                proof {
                    reveal_byteslit(b",");
                    reveal_strlit(",");
                    reveal(ckc_spec::v1text::ascii);
                    reveal(comma_name);
                    assert(comma@ == comma_name());
                }
                if child_roots.len() == 2 && crate::k3_front::bytes_equal(name, comma) {
                    let left = child_roots[0];
                    let right = child_roots[1];
                    proof {
                        assert(args.len() == 2);
                        assert(args[0] == arena@[left as int]);
                        assert(args[1] == arena@[right as int]);
                        assert(binds(t, x) == (binds(args[0], x) || binds(args[1], x)));
                        reveal(term_size);
                        terms_size_two(args);
                        assert(term_size(t) == 1 + term_size(args[0]) + term_size(args[1]));
                    }
                    tasks.push(left);
                    let ghost mid = tasks@;
                    tasks.push(right);
                    proof {
                        roots_work_push(arena.nodes@, rest, left);
                        roots_work_push(arena.nodes@, mid, right);
                        assert(mid == rest.push(left));
                        assert(tasks@ == mid.push(right));
                        roots_binds_unfold(arena.nodes@, tasks@, x);
                        assert(tasks@.drop_last() =~= mid);
                        roots_binds_unfold(arena.nodes@, mid, x);
                        assert(mid.drop_last() =~= rest);
                    }
                } else if crate::compare_impl::is_builtin_exec(name, child_roots.len()) {
                    proof {
                        assert(!binds(t, x));
                    }
                } else if child_roots.len() > 0 {
                    let first = child_roots[0];
                    proof {
                        assert(args[0] == arena@[first as int]);
                        assert(binds(t, x) == (args[0] == x));
                        assert(node_ok(arena.nodes@, first as int));
                    }
                    match &arena.nodes[first].kind {
                        ENodeKind::Var { key: k, .. } => {
                            proof {
                                crate::k2_engine::node_var_model(arena.nodes@, first as int, *k);
                            }
                            if *k == key {
                                return true;
                            }
                        },
                        _ => {},
                    }
                } else {
                    proof {
                        assert(!binds(t, x));
                    }
                }
            },
            _ => {
                proof {
                    assert(!(t is Comp));
                }
            },
        }
    }
    proof {
        roots_binds_unfold(arena.nodes@, tasks@, x);
    }
    false
}

// --- stack scans ---
pub fn is_naf_root(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == match arena@[root as int] {
            Term::Comp(n, args) => n == naf_name() && args.len() == 1,
            _ => false,
        },
{
    let naf: &[u8] = b"\\+";
    proof {
        reveal_byteslit(b"\\+");
        reveal_strlit("\\+");
        reveal(ckc_spec::v1text::ascii);
        reveal(naf_name);
        assert(naf@ == naf_name());
    }
    crate::k3_front::is_comp(arena, root, naf, 1)
}

pub fn is_comp_root(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == (arena@[root as int] is Comp),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { .. } => true,
        _ => false,
    }
}

pub fn nonground_builtin_root(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == match arena@[root as int] {
            Term::Comp(name, args) => ckc_spec::compare::is_builtin(name, args.len())
                && !ckc_spec::term::ground(arena@[root as int]),
            _ => false,
        },
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            proof {
                node_comp_model(arena.nodes@, root as int, name@, child_roots@);
                reveal(child_terms);
            }
            crate::compare_impl::is_builtin_exec(name, child_roots.len())
                && !crate::k2_walk::ground_root(arena, root)
        },
        _ => false,
    }
}

pub fn run_binds_from(arena: &ETermArena, goals: &Vec<EGoal>, from: usize, key: usize) -> (out:
    bool)
    requires
        arena_ok(arena),
        goals_valid(arena.nodes@, goals@),
        from <= goals.len(),
    ensures
        out == run_binds(goals_view(arena.nodes@, goals@).skip(from as int), Term::Var(key as nat)),
{
    let ghost x = Term::Var(key as nat);
    let ghost view = goals_view(arena.nodes@, goals@);
    let mut j = from;
    while j < goals.len()
        invariant
            arena_ok(arena),
            goals_valid(arena.nodes@, goals@),
            view == goals_view(arena.nodes@, goals@),
            from <= j <= goals.len(),
            x == Term::Var(key as nat),
            run_binds(view.skip(from as int), x) == run_binds(view.skip(j as int), x),
        decreases goals.len() - j,
    {
        let ghost s = view.skip(j as int);
        proof {
            assert(s.len() > 0);
            assert(s[0] == view[j as int]);
            assert(s.drop_first() =~= view.skip(j as int + 1));
            assert(goal_valid(arena.nodes@, &goals@[j as int]));
        }
        match goals[j] {
            EGoal::Lit { root, depth } => {
                proof {
                    assert(view[j as int] == goal_view(arena.nodes@, &goals@[j as int]));
                    assert(view[j as int] == Goal::Lit(arena@[root as int], depth as nat));
                    run_binds_lit(s, x, arena@[root as int], depth as nat);
                }
                if !is_comp_root(arena, root) || is_naf_root(arena, root) || nonground_builtin_root(
                    arena,
                    root,
                ) {
                    return false;
                }
                if binds_root(arena, root, key) {
                    return true;
                }
            },
            EGoal::NafCut { level } => {
                proof {
                    assert(view[j as int] == goal_view(arena.nodes@, &goals@[j as int]));
                    run_binds_cut(s, x, level as nat);
                }
                return false;
            },
        }
        j += 1;
    }
    proof {
        assert(view.skip(j as int).len() == 0);
        run_binds_empty(view.skip(j as int), x);
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
            goals_view(arena.nodes@, goals@)[k as int],
            goals_view(arena.nodes@, goals@).skip(k as int + 1),
        ),
{
    let ghost view = goals_view(arena.nodes@, goals@);
    proof {
        assert(goal_valid(arena.nodes@, &goals@[k as int]));
    }
    match goals[k] {
        EGoal::Lit { root, .. } => {
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
                        node_comp_model(arena.nodes@, root as int, name@, child_roots@);
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

pub fn selected_exec(arena: &ETermArena, goals: &Vec<EGoal>) -> (k: usize)
    requires
        arena_ok(arena),
        goals_valid(arena.nodes@, goals@),
    ensures
        k == selected(goals_view(arena.nodes@, goals@)),
{
    let ghost view = goals_view(arena.nodes@, goals@);
    if goals.len() == 0 {
        proof {
            selected_unfold(view);
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
            view == goals_view(arena.nodes@, goals@),
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
                selected_step(view, k as int);
            }
            k += 1;
        } else {
            stop = true;
        }
    }
    proof {
        selected_stop(view, k as int);
    }
    k
}

} // verus!
