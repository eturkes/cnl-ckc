// K2-facing seam: clause preparation + template unification; trace metadata stays outside K2.
use crate::k2_engine::{EBoundState, EClause, EGoal as KGoal, EPair, EUResult};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{
    clause_valid, clause_view, pair_roots_valid, pairs_view, root_terms, roots_valid,
};
use crate::k2_term::{ENode, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
use crate::k3_state::EGoal;
#[cfg(verus_keep_ghost)]
use crate::k3_state::{
    goal_level, goal_valid, goal_view, goals_levels, goals_valid, goals_view, path_view,
};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::trace::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub enum EUni {
    Ok(Vec<EGoal>),
    Fail,
}

pub open spec fn uni_view(nodes: Seq<ENode>, out: &EUni) -> TUni {
    match out {
        EUni::Ok(goals) => TUni::Ok(goals_view(nodes, goals@)),
        EUni::Fail => TUni::Fail,
    }
}

pub open spec fn uni_valid(nodes: Seq<ENode>, out: &EUni, level: nat) -> bool {
    match out {
        EUni::Ok(goals) => goals_valid(nodes, goals@) && goals_levels(goals@, level),
        EUni::Fail => true,
    }
}

pub open spec fn erase(g: TGoal) -> ckc_spec::engine::Goal {
    match g {
        TGoal::Lit(t, d, _) => ckc_spec::engine::Goal::Lit(t, d),
        TGoal::NafCut(l) => ckc_spec::engine::Goal::NafCut(l),
    }
}

fn to_engine(arena: &ETermArena, goals: &Vec<EGoal>, Ghost(level): Ghost<nat>) -> (out: Vec<KGoal>)
    requires
        arena_ok(arena),
        goals_valid(arena.nodes@, goals@),
        goals_levels(goals@, level),
    ensures
        crate::k2_engine::goals_valid(arena.nodes@, out@),
        crate::k2_engine::goals_levels(out@, level),
        crate::k2_engine::goals_view(arena.nodes@, out@) == goals_view(
            arena.nodes@,
            goals@,
        ).map_values(|g: TGoal| erase(g)),
{
    let ghost models = goals_view(arena.nodes@, goals@);
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < goals.len()
        invariant
            arena_ok(arena),
            goals_valid(arena.nodes@, goals@),
            goals_levels(goals@, level),
            models == goals_view(arena.nodes@, goals@),
            i <= goals.len(),
            out.len() == i,
            crate::k2_engine::goals_valid(arena.nodes@, out@),
            crate::k2_engine::goals_levels(out@, level),
            crate::k2_engine::goals_view(arena.nodes@, out@) == models.take(i as int).map_values(
                |g: TGoal| erase(g),
            ),
        decreases goals.len() - i,
    {
        proof {
            assert(goal_valid(arena.nodes@, &goals@[i as int]));
            assert(goal_level(&goals@[i as int], level));
        }
        let next = match &goals[i] {
            EGoal::Lit { root, depth, .. } => KGoal::Lit { root: *root, depth: *depth },
            EGoal::NafCut { level } => KGoal::NafCut { level: *level },
        };
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies {
                &&& crate::k2_engine::goal_valid(arena.nodes@, &out@[j])
                &&& crate::k2_engine::goal_level_valid(&out@[j], level)
            } by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                    assert(crate::k2_engine::goal_valid(arena.nodes@, &before[j]));
                    assert(crate::k2_engine::goal_level_valid(&before[j], level));
                }
            }
            assert_seqs_equal!(crate::k2_engine::goals_view(arena.nodes@, out@) == crate::k2_engine::goals_view(arena.nodes@, before).push(erase(models[i as int])));
            assert_seqs_equal!(models.take(i as int + 1).map_values(|g: TGoal| erase(g)) == models.take(i as int).map_values(|g: TGoal| erase(g)).push(erase(models[i as int])));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.take(i as int) == models);
    }
    out
}

fn attach_paths(
    arena: &ETermArena,
    prepared: &Vec<KGoal>,
    count: usize,
    depth: usize,
    path: &Vec<usize>,
    Ghost(items): Ghost<Seq<ckc_spec::v1text::BodyItem>>,
    Ghost(off): Ghost<nat>,
) -> (out: Vec<EGoal>)
    requires
        arena_ok(arena),
        crate::k2_engine::goals_valid(arena.nodes@, prepared@),
        count <= prepared.len(),
        count == items.len(),
        crate::k2_engine::goals_view(arena.nodes@, prepared@).take(count as int)
            == ckc_spec::engine::body_goals(items, off, depth as nat),
    ensures
        goals_valid(arena.nodes@, out@),
        goals_levels(out@, 0),
        goals_view(arena.nodes@, out@) == tbody_goals(items, off, depth as nat, path_view(path@)),
{
    let ghost models = crate::k2_engine::goals_view(arena.nodes@, prepared@);
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < count
        invariant
            arena_ok(arena),
            crate::k2_engine::goals_valid(arena.nodes@, prepared@),
            count <= prepared.len(),
            count == items.len(),
            models == crate::k2_engine::goals_view(arena.nodes@, prepared@),
            models.take(count as int) == ckc_spec::engine::body_goals(items, off, depth as nat),
            i <= count,
            out.len() == i,
            goals_valid(arena.nodes@, out@),
            goals_levels(out@, 0),
            goals_view(arena.nodes@, out@) == tbody_goals(
                items.take(i as int),
                off,
                depth as nat,
                path_view(path@),
            ),
        decreases count - i,
    {
        proof {
            assert(crate::k2_engine::goal_valid(arena.nodes@, &prepared@[i as int]));
            assert(models[i as int] == ckc_spec::engine::Goal::Lit(
                ckc_spec::engine::item_term(items[i as int], off),
                depth as nat,
            ));
        }
        let root = match &prepared[i] {
            KGoal::Lit { root, .. } => *root,
            KGoal::NafCut { .. } => {
                proof {
                    assert(false);
                }
                0
            },
        };
        let mut child_path = path.clone();
        child_path.push(i);
        let next = EGoal::Lit { root, depth, path: child_path };
        let ghost before = out@;
        out.push(next);
        proof {
            assert_seqs_equal!(path_view(path@.push(i)) == path_view(path@).push(i as nat));
            assert forall|j: int| 0 <= j < out.len() implies {
                &&& goal_valid(arena.nodes@, &out@[j])
                &&& goal_level(&out@[j], 0)
            } by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                    assert(goal_valid(arena.nodes@, &before[j]));
                    assert(goal_level(&before[j], 0));
                }
            }
            assert_seqs_equal!(goals_view(arena.nodes@, out@) == goals_view(arena.nodes@, before).push(TGoal::Lit(ckc_spec::engine::item_term(items[i as int], off), depth as nat, path_view(path@).push(i as nat))));
            assert_seqs_equal!(tbody_goals(items.take(i as int + 1), off, depth as nat, path_view(path@)) == tbody_goals(items.take(i as int), off, depth as nat, path_view(path@)).push(TGoal::Lit(ckc_spec::engine::item_term(items[i as int], off), depth as nat, path_view(path@).push(i as nat))));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(items.take(i as int) == items);
    }
    out
}

pub fn prepare(
    arena: &mut ETermArena,
    clause: &EClause,
    args: &Vec<usize>,
    rest: &Vec<EGoal>,
    off: usize,
    depth: usize,
    path: &Vec<usize>,
    Ghost(level): Ghost<nat>,
) -> (out: (Vec<EPair>, Vec<EGoal>, usize))
    requires
        arena_ok(old(arena)),
        clause_valid(old(arena).nodes@, clause),
        roots_valid(old(arena).nodes@, args@),
        goals_valid(old(arena).nodes@, rest@),
        goals_levels(rest@, level),
        off <= old(arena).nodes.len(),
        clause_view(old(arena).nodes@, clause).head is Comp,
        args.len() == ckc_spec::engine::args_of(clause_view(old(arena).nodes@, clause).head).len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        pair_roots_valid(final(arena).nodes@, out.0@),
        goals_valid(final(arena).nodes@, out.1@),
        goals_levels(out.1@, level),
        pairs_view(final(arena).nodes@, out.0@) == ckc_spec::engine::zip(
            root_terms(old(arena).nodes@, args@),
            ckc_spec::engine::args_of(
                ckc_spec::engine::shift(clause_view(old(arena).nodes@, clause).head, off as nat),
            ),
        ),
        goals_view(final(arena).nodes@, out.1@) == tbody_goals(
            clause_view(old(arena).nodes@, clause).body,
            off as nat,
            depth as nat,
            path_view(path@),
        ) + goals_view(old(arena).nodes@, rest@),
        out.2 as nat == off as nat + ckc_spec::engine::clause_nvars(
            clause_view(old(arena).nodes@, clause),
        ),
        out.2 <= final(arena).nodes.len(),
{
    let ghost before = arena.nodes@;
    let ghost model = clause_view(before, clause);
    let engine_rest = to_engine(arena, rest, Ghost(level));
    let empty = Vec::new();
    let (state, fresh) = crate::k2_engine::prepare_clause(
        arena,
        clause,
        args,
        &engine_rest,
        &empty,
        off,
        depth,
        Ghost(level),
    );
    proof {
        crate::k3_state::goals_prefix(before, arena.nodes@, rest@);
        assert_seqs_equal!(crate::k2_engine::goals_view(arena.nodes@, state.stack@).take(clause.body.len() as int) == ckc_spec::engine::body_goals(model.body, off as nat, depth as nat));
    }
    let mut body = attach_paths(
        arena,
        &state.stack,
        clause.body.len(),
        depth,
        path,
        Ghost(model.body),
        Ghost(off as nat),
    );
    let mut continuation = crate::k3_state::clone_goals(arena, rest, 0, Ghost(level));
    proof {
        crate::k3_state::levels_weaken(body@, 0, level);
        crate::k3_state::goals_concat(arena.nodes@, body@, continuation@, level);
    }
    body.append(&mut continuation);
    (state.pairs, body, fresh)
}

proof fn subst_length(ts: Seq<Term>, x: nat, value: Term)
    ensures
        ckc_spec::engine::subst_all(ts, x, value).len() == ts.len(),
    decreases ts.len(),
{
    reveal_with_fuel(ckc_spec::engine::subst_all, 1);
    if ts.len() > 0 {
        subst_length(ts.drop_first(), x, value);
    }
}

proof fn unify_n_length(u: ckc_spec::engine::UState, fuel: nat)
    ensures
        ckc_spec::engine::unify_n(u, fuel) matches ckc_spec::engine::UOut::Ok(_, sol) ==> sol.len()
            == u.sol.len(),
    decreases fuel,
{
    reveal_with_fuel(ckc_spec::engine::unify_n, 2);
    if fuel > 0 && u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let next = match (a, b) {
            (Term::Var(x), _) => if a == b {
                Some(ckc_spec::engine::UState { pairs: rest, ..u })
            } else if !ckc_spec::engine::occurs(x, b) {
                subst_length(u.sol, x, b);
                Some(ckc_spec::engine::u_bind(u, rest, x, b))
            } else {
                None
            },
            (_, Term::Var(y)) => if !ckc_spec::engine::occurs(y, a) {
                subst_length(u.sol, y, a);
                Some(ckc_spec::engine::u_bind(u, rest, y, a))
            } else {
                None
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => if n == m && xs.len() == ys.len() {
                Some(ckc_spec::engine::UState { pairs: ckc_spec::engine::zip(xs, ys) + rest, ..u })
            } else {
                None
            },
            _ => if a == b {
                Some(ckc_spec::engine::UState { pairs: rest, ..u })
            } else {
                None
            },
        };
        if let Some(state) = next {
            unify_n_length(state, (fuel - 1) as nat);
        }
    }
}

proof fn unify_length(u: ckc_spec::engine::UState)
    ensures
        ckc_spec::engine::unify(u) matches ckc_spec::engine::UOut::Ok(_, sol) ==> sol.len()
            == u.sol.len(),
{
    reveal(ckc_spec::engine::unify);
    if exists|f: nat| !(ckc_spec::engine::unify_n(u, f) is Out) {
        let f = choose|f: nat| !(ckc_spec::engine::unify_n(u, f) is Out);
        unify_n_length(u, f);
    }
}

fn templates(arena: &ETermArena, goals: &Vec<EGoal>, nil: usize) -> (out: Vec<usize>)
    requires
        arena_ok(arena),
        goals_valid(arena.nodes@, goals@),
        root_ok(arena, nil),
        arena@[nil as int] == Term::Nil,
    ensures
        roots_valid(arena.nodes@, out@),
        root_terms(arena.nodes@, out@) == goals_view(arena.nodes@, goals@).map_values(
            |g: TGoal| tgoal_term(g),
        ),
{
    let ghost models = goals_view(arena.nodes@, goals@);
    let mut i = 0usize;
    let mut out = Vec::new();
    while i < goals.len()
        invariant
            arena_ok(arena),
            goals_valid(arena.nodes@, goals@),
            root_ok(arena, nil),
            arena@[nil as int] == Term::Nil,
            models == goals_view(arena.nodes@, goals@),
            i <= goals.len(),
            out.len() == i,
            roots_valid(arena.nodes@, out@),
            root_terms(arena.nodes@, out@) == models.take(i as int).map_values(
                |g: TGoal| tgoal_term(g),
            ),
        decreases goals.len() - i,
    {
        proof {
            assert(goal_valid(arena.nodes@, &goals@[i as int]));
        }
        let root = match &goals[i] {
            EGoal::Lit { root, .. } => *root,
            EGoal::NafCut { .. } => nil,
        };
        let ghost before = out@;
        out.push(root);
        proof {
            assert_seqs_equal!(root_terms(arena.nodes@, out@) == root_terms(arena.nodes@, before).push(tgoal_term(models[i as int])));
            assert_seqs_equal!(models.take(i as int + 1).map_values(|g: TGoal| tgoal_term(g)) == models.take(i as int).map_values(|g: TGoal| tgoal_term(g)).push(tgoal_term(models[i as int])));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.take(i as int) == models);
    }
    out
}

fn with_terms(
    arena: &ETermArena,
    mut goals: Vec<EGoal>,
    terms: &Vec<usize>,
    Ghost(level): Ghost<nat>,
) -> (out: Vec<EGoal>)
    requires
        arena_ok(arena),
        goals_valid(arena.nodes@, goals@),
        goals_levels(goals@, level),
        roots_valid(arena.nodes@, terms@),
        goals.len() == terms.len(),
    ensures
        goals_valid(arena.nodes@, out@),
        goals_levels(out@, level),
        goals_view(arena.nodes@, out@) == Seq::new(
            goals.len() as nat,
            |i: int|
                tgoal_with(
                    goals_view(arena.nodes@, goals@)[i],
                    root_terms(arena.nodes@, terms@)[i],
                ),
        ),
{
    let ghost models = goals_view(arena.nodes@, goals@);
    let ghost values = root_terms(arena.nodes@, terms@);
    let mut i = 0usize;
    let mut out = Vec::new();
    while i < terms.len()
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, terms@),
            values == root_terms(arena.nodes@, terms@),
            i <= terms.len(),
            models.len() == terms.len(),
            goals.len() == terms.len() - i,
            out.len() == i,
            goals_valid(arena.nodes@, goals@),
            goals_levels(goals@, level),
            goals_view(arena.nodes@, goals@) == models.skip(i as int),
            goals_valid(arena.nodes@, out@),
            goals_levels(out@, level),
            goals_view(arena.nodes@, out@) == Seq::new(
                i as nat,
                |j: int| tgoal_with(models[j], values[j]),
            ),
        decreases terms.len() - i,
    {
        let ghost remaining = goals@;
        proof {
            assert(goal_valid(arena.nodes@, &goals@[0]));
            assert(goal_level(&goals@[0], level));
        }
        let g = goals.remove(0);
        let next = match g {
            EGoal::Lit { depth, path, .. } => EGoal::Lit { root: terms[i], depth, path },
            EGoal::NafCut { level } => EGoal::NafCut { level },
        };
        let ghost previous = out@;
        out.push(next);
        proof {
            assert_seqs_equal!(goals_view(arena.nodes@, goals@) == goals_view(arena.nodes@, remaining).drop_first());
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
            assert forall|j: int| 0 <= j < out.len() implies {
                &&& goal_valid(arena.nodes@, &out@[j])
                &&& goal_level(&out@[j], level)
            } by {
                if j < previous.len() {
                    assert(out@[j] == previous[j]);
                    assert(goal_valid(arena.nodes@, &previous[j]));
                    assert(goal_level(&previous[j], level));
                }
            }
            assert_seqs_equal!(goals_view(arena.nodes@, out@) == Seq::new(i as nat + 1, |j: int| tgoal_with(models[j], values[j])), j => {
                if j < previous.len() { assert(out@[j] == previous[j]); assert(goals_view(arena.nodes@, previous)[j] == tgoal_with(models[j], values[j])); }
            });
        }
        i += 1;
    }
    out
}

pub fn tunify_exec(
    arena: &mut ETermArena,
    pairs: Vec<EPair>,
    goals: Vec<EGoal>,
    Ghost(level): Ghost<nat>,
) -> (out: EUni)
    requires
        arena_ok(old(arena)),
        pair_roots_valid(old(arena).nodes@, pairs@),
        goals_valid(old(arena).nodes@, goals@),
        goals_levels(goals@, level),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        uni_valid(final(arena).nodes@, &out, level),
        uni_view(final(arena).nodes@, &out) == tunify(
            pairs_view(old(arena).nodes@, pairs@),
            goals_view(old(arena).nodes@, goals@),
        ),
{
    let ghost before = arena.nodes@;
    let ghost initial_pairs = pairs_view(before, pairs@);
    let ghost initial_goals = goals_view(before, goals@);
    let nil = crate::k2_term::push_nil(arena);
    proof {
        crate::k3_state::goals_prefix(before, arena.nodes@, goals@);
        crate::k2_engine::pairs_models_prefix(before, arena.nodes@, pairs@);
    }
    let sol = templates(arena, &goals, nil);
    let state = EBoundState { pairs, stack: Vec::new(), sol };
    let ghost unifier = crate::k2_engine::bound_state_view(arena.nodes@, &state);
    proof {
        assert_seqs_equal!(unifier.stack == Seq::empty());
        unify_length(unifier);
    }
    let ghost prepared = arena.nodes@;
    match crate::k2_engine::unify(arena, state) {
        EUResult::Fail => EUni::Fail,
        EUResult::Ok { stack: _, sol } => {
            proof {
                crate::k3_state::goals_prefix(prepared, arena.nodes@, goals@);
                assert(sol.len() == goals.len());
            }
            EUni::Ok(with_terms(arena, goals, &sol, Ghost(level)))
        },
    }
}

pub fn fresh_for_roots(arena: &mut ETermArena, roots: &Vec<usize>) -> (out: usize)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == ckc_spec::engine::nvars_all(root_terms(old(arena).nodes@, roots@)),
        out <= final(arena).nodes.len(),
{
    let maximum = crate::k2_engine::roots_max_var(arena, roots);
    crate::k2_engine::ensure_var_capacity(arena, maximum)
}

} // verus!
