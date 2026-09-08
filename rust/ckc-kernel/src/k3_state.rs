#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::trace::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub enum EGoal {
    Lit { root: usize, depth: usize, path: Vec<usize> },
    NafCut { level: usize },
}

pub enum EEvent {
    Clause(usize),
    Naf(usize),
}

pub struct EEntry {
    pub path: Vec<usize>,
    pub event: EEvent,
}

pub enum EAlt {
    Cl { stack: Vec<EGoal>, fresh: usize, ci: usize, log_len: usize },
    Naf {
        stack: Vec<EGoal>,
        fresh: usize,
        log_len: usize,
        path: Vec<usize>,
        inner: usize,
        pruned: bool,
    },
}

pub struct ECfg {
    pub stack: Vec<EGoal>,
    pub alts: Vec<EAlt>,
    pub fresh: usize,
    pub ci: usize,
    pub log: Vec<EEntry>,
    pub pruned: bool,
}

pub enum EStep {
    Next(ECfg),
    Sol(Vec<EEntry>),
    Done(bool),
    Limit,
}

pub enum EOut {
    Proved(Vec<EEntry>),
    Failed(bool),
    Limit,
}

pub open spec fn path_view(path: Seq<usize>) -> Seq<nat> {
    path.map_values(|i: usize| i as nat)
}

pub open spec fn goal_view(nodes: Seq<ENode>, goal: &EGoal) -> TGoal {
    match goal {
        EGoal::Lit { root, depth, path } => TGoal::Lit(
            nodes[*root as int].term@,
            *depth as nat,
            path_view(path@),
        ),
        EGoal::NafCut { level } => TGoal::NafCut(*level as nat),
    }
}

pub open spec fn goals_view(nodes: Seq<ENode>, goals: Seq<EGoal>) -> Seq<TGoal> {
    goals.map_values(|g: EGoal| goal_view(nodes, &g))
}

pub open spec fn goal_valid(nodes: Seq<ENode>, goal: &EGoal) -> bool {
    match goal {
        EGoal::Lit { root, .. } => *root < nodes.len(),
        EGoal::NafCut { .. } => true,
    }
}

pub open spec fn goal_level(goal: &EGoal, level: nat) -> bool {
    match goal {
        EGoal::NafCut { level: l } => *l < level,
        _ => true,
    }
}

pub open spec fn goals_valid(nodes: Seq<ENode>, goals: Seq<EGoal>) -> bool {
    forall|i: int| 0 <= i < goals.len() ==> #[trigger] goal_valid(nodes, &goals[i])
}

pub open spec fn goals_levels(goals: Seq<EGoal>, level: nat) -> bool {
    forall|i: int| 0 <= i < goals.len() ==> #[trigger] goal_level(&goals[i], level)
}

pub open spec fn event_view(nodes: Seq<ENode>, event: &EEvent) -> TEv {
    match event {
        EEvent::Clause(m) => TEv::Clause(*m as nat),
        EEvent::Naf(root) => TEv::Naf(nodes[*root as int].term@),
    }
}

pub open spec fn entry_view(nodes: Seq<ENode>, entry: &EEntry) -> (Seq<nat>, TEv) {
    (path_view(entry.path@), event_view(nodes, &entry.event))
}

pub open spec fn log_view(nodes: Seq<ENode>, log: Seq<EEntry>) -> Seq<(Seq<nat>, TEv)> {
    log.map_values(|entry: EEntry| entry_view(nodes, &entry))
}

pub open spec fn entry_valid(nodes: Seq<ENode>, db_len: nat, entry: &EEntry) -> bool {
    match entry.event {
        EEvent::Clause(m) => m < db_len,
        EEvent::Naf(root) => root < nodes.len(),
    }
}

pub open spec fn log_valid(nodes: Seq<ENode>, db_len: nat, log: Seq<EEntry>) -> bool {
    forall|i: int| 0 <= i < log.len() ==> #[trigger] entry_valid(nodes, db_len, &log[i])
}

pub open spec fn alt_mark(alt: &EAlt) -> nat {
    match alt {
        EAlt::Cl { log_len, .. } | EAlt::Naf { log_len, .. } => *log_len as nat,
    }
}

pub open spec fn alt_view(nodes: Seq<ENode>, log: Seq<EEntry>, alt: &EAlt) -> TAlt {
    match alt {
        EAlt::Cl { stack, fresh, ci, log_len } => TAlt::Cl {
            stack: goals_view(nodes, stack@),
            fresh: *fresh as nat,
            ci: *ci as nat,
            log: log_view(nodes, log.take(*log_len as int)),
        },
        EAlt::Naf { stack, fresh, log_len, path, inner, pruned } => TAlt::Naf {
            stack: goals_view(nodes, stack@),
            fresh: *fresh as nat,
            log: log_view(nodes, log.take(*log_len as int)),
            path: path_view(path@),
            inner: nodes[*inner as int].term@,
            pruned: *pruned,
        },
    }
}

pub open spec fn alts_view(nodes: Seq<ENode>, log: Seq<EEntry>, alts: Seq<EAlt>) -> Seq<TAlt> {
    alts.map_values(|a: EAlt| alt_view(nodes, log, &a))
}

pub open spec fn alt_valid(nodes: Seq<ENode>, log_len: nat, alt: &EAlt, level: nat) -> bool {
    alt_mark(alt) <= log_len && match alt {
        EAlt::Cl { stack, fresh, .. } => goals_valid(nodes, stack@) && goals_levels(stack@, level)
            && *fresh <= nodes.len(),
        EAlt::Naf { stack, fresh, inner, .. } => goals_valid(nodes, stack@) && goals_levels(
            stack@,
            level,
        ) && *fresh <= nodes.len() && *inner < nodes.len(),
    }
}

pub open spec fn alts_valid(nodes: Seq<ENode>, log_len: nat, alts: Seq<EAlt>) -> bool {
    forall|i: int| 0 <= i < alts.len() ==> #[trigger] alt_valid(nodes, log_len, &alts[i], i as nat)
}

pub open spec fn marks_sorted(alts: Seq<EAlt>) -> bool {
    forall|i: int, j: int|
        0 <= i <= j < alts.len() ==> #[trigger] alt_mark(&alts[i]) <= #[trigger] alt_mark(&alts[j])
}

pub open spec fn cfg_view(nodes: Seq<ENode>, c: &ECfg) -> TCfg {
    TCfg {
        stack: goals_view(nodes, c.stack@),
        alts: alts_view(nodes, c.log@, c.alts@),
        fresh: c.fresh as nat,
        ci: c.ci as nat,
        log: log_view(nodes, c.log@),
        pruned: c.pruned,
    }
}

pub open spec fn cfg_valid(nodes: Seq<ENode>, db_len: nat, c: &ECfg) -> bool {
    goals_valid(nodes, c.stack@) && goals_levels(c.stack@, c.alts.len() as nat) && alts_valid(
        nodes,
        c.log.len() as nat,
        c.alts@,
    ) && marks_sorted(c.alts@) && log_valid(nodes, db_len, c.log@) && c.fresh <= nodes.len()
}

pub open spec fn step_view(nodes: Seq<ENode>, step: &EStep) -> TStep {
    match step {
        EStep::Next(c) => TStep::Next(cfg_view(nodes, c)),
        EStep::Sol(log) => TStep::Sol(log_view(nodes, log@)),
        EStep::Done(b) => TStep::Done(*b),
        EStep::Limit => TStep::Limit,
    }
}

pub open spec fn step_valid(nodes: Seq<ENode>, db_len: nat, step: &EStep) -> bool {
    match step {
        EStep::Next(c) => cfg_valid(nodes, db_len, c),
        EStep::Sol(log) => log_valid(nodes, db_len, log@),
        _ => true,
    }
}

pub open spec fn out_view(nodes: Seq<ENode>, out: &EOut) -> TOut {
    match out {
        EOut::Proved(log) => TOut::Proved(log_view(nodes, log@)),
        EOut::Failed(b) => TOut::Failed(*b),
        EOut::Limit => TOut::Limit,
    }
}

pub open spec fn out_valid(nodes: Seq<ENode>, db_len: nat, out: &EOut) -> bool {
    match out {
        EOut::Proved(log) => log_valid(nodes, db_len, log@),
        _ => true,
    }
}

pub proof fn goals_prefix(before: Seq<ENode>, after: Seq<ENode>, goals: Seq<EGoal>)
    requires
        before.is_prefix_of(after),
        goals_valid(before, goals),
    ensures
        goals_valid(after, goals),
        goals_view(before, goals) == goals_view(after, goals),
{
    assert forall|i: int| 0 <= i < goals.len() implies {
        &&& goal_valid(after, &goals[i])
        &&& goal_view(before, &goals[i]) == goal_view(after, &goals[i])
    } by {
        assert(goal_valid(before, &goals[i]));
        match &goals[i] {
            EGoal::Lit { root, .. } => {
                assert(before[*root as int] == after[*root as int]);
            },
            EGoal::NafCut { .. } => {},
        }
    }
    assert_seqs_equal!(goals_view(before, goals) == goals_view(after, goals));
}

pub proof fn log_prefix(before: Seq<ENode>, after: Seq<ENode>, db_len: nat, log: Seq<EEntry>)
    requires
        before.is_prefix_of(after),
        log_valid(before, db_len, log),
    ensures
        log_valid(after, db_len, log),
        log_view(before, log) == log_view(after, log),
{
    assert forall|i: int| 0 <= i < log.len() implies {
        &&& entry_valid(after, db_len, &log[i])
        &&& entry_view(before, &log[i]) == entry_view(after, &log[i])
    } by {
        assert(entry_valid(before, db_len, &log[i]));
        match log[i].event {
            EEvent::Clause(_) => {},
            EEvent::Naf(root) => {
                assert(before[root as int] == after[root as int]);
            },
        }
    }
    assert_seqs_equal!(log_view(before, log) == log_view(after, log));
}

pub proof fn alt_prefix(
    before: Seq<ENode>,
    after: Seq<ENode>,
    db_len: nat,
    log: Seq<EEntry>,
    alt: &EAlt,
    level: nat,
)
    requires
        before.is_prefix_of(after),
        log_valid(before, db_len, log),
        alt_valid(before, log.len(), alt, level),
    ensures
        alt_valid(after, log.len(), alt, level),
        alt_view(before, log, alt) == alt_view(after, log, alt),
{
    match alt {
        EAlt::Cl { stack, log_len, .. } => {
            goals_prefix(before, after, stack@);
            log_prefix(before, after, db_len, log.take(*log_len as int));
        },
        EAlt::Naf { stack, log_len, inner, .. } => {
            goals_prefix(before, after, stack@);
            log_prefix(before, after, db_len, log.take(*log_len as int));
            assert(before[*inner as int] == after[*inner as int]);
        },
    }
}

pub proof fn cfg_prefix(before: Seq<ENode>, after: Seq<ENode>, db_len: nat, c: &ECfg)
    requires
        before.is_prefix_of(after),
        cfg_valid(before, db_len, c),
    ensures
        cfg_valid(after, db_len, c),
        cfg_view(before, c) == cfg_view(after, c),
{
    goals_prefix(before, after, c.stack@);
    log_prefix(before, after, db_len, c.log@);
    assert forall|i: int| 0 <= i < c.alts.len() implies {
        &&& alt_valid(after, c.log.len() as nat, &c.alts@[i], i as nat)
        &&& alt_view(before, c.log@, &c.alts@[i]) == alt_view(after, c.log@, &c.alts@[i])
    } by {
        alt_prefix(before, after, db_len, c.log@, &c.alts@[i], i as nat);
    }
    assert_seqs_equal!(alts_view(before, c.log@, c.alts@) == alts_view(after, c.log@, c.alts@));
}

pub proof fn alts_log_prefix(
    nodes: Seq<ENode>,
    before: Seq<EEntry>,
    after: Seq<EEntry>,
    alts: Seq<EAlt>,
)
    requires
        before.is_prefix_of(after),
        alts_valid(nodes, before.len(), alts),
    ensures
        alts_valid(nodes, after.len(), alts),
        alts_view(nodes, before, alts) == alts_view(nodes, after, alts),
{
    assert forall|i: int| 0 <= i < alts.len() implies {
        &&& alt_valid(nodes, after.len(), &alts[i], i as nat)
        &&& alt_view(nodes, before, &alts[i]) == alt_view(nodes, after, &alts[i])
    } by {
        assert(alt_valid(nodes, before.len(), &alts[i], i as nat));
        let mark = alt_mark(&alts[i]);
        assert_seqs_equal!(before.take(mark as int) == after.take(mark as int));
    }
    assert_seqs_equal!(alts_view(nodes, before, alts) == alts_view(nodes, after, alts));
}

pub proof fn goals_concat(nodes: Seq<ENode>, left: Seq<EGoal>, right: Seq<EGoal>, level: nat)
    requires
        goals_valid(nodes, left),
        goals_valid(nodes, right),
        goals_levels(left, level),
        goals_levels(right, level),
    ensures
        goals_valid(nodes, left + right),
        goals_levels(left + right, level),
        goals_view(nodes, left + right) == goals_view(nodes, left) + goals_view(nodes, right),
{
    assert forall|i: int| 0 <= i < (left + right).len() implies {
        &&& goal_valid(nodes, &(left + right)[i])
        &&& goal_level(&(left + right)[i], level)
    } by {
        if i < left.len() {
            assert((left + right)[i] == left[i]);
        } else {
            assert((left + right)[i] == right[i - left.len()]);
        }
    }
    assert_seqs_equal!(goals_view(nodes, left + right) == goals_view(nodes, left) + goals_view(nodes, right));
}

pub proof fn levels_weaken(goals: Seq<EGoal>, before: nat, after: nat)
    requires
        goals_levels(goals, before),
        before <= after,
    ensures
        goals_levels(goals, after),
{
    assert forall|i: int| 0 <= i < goals.len() implies #[trigger] goal_level(&goals[i], after) by {
        assert(goal_level(&goals[i], before));
        match &goals[i] {
            EGoal::Lit { .. } => {},
            EGoal::NafCut { level } => {
                assert(*level < before);
            },
        }
    }
}

pub fn clone_goals(
    arena: &ETermArena,
    goals: &Vec<EGoal>,
    from: usize,
    Ghost(level): Ghost<nat>,
) -> (out: Vec<EGoal>)
    requires
        arena_ok(arena),
        goals_valid(arena.nodes@, goals@),
        goals_levels(goals@, level),
        from <= goals.len(),
    ensures
        goals_valid(arena.nodes@, out@),
        goals_levels(out@, level),
        goals_view(arena.nodes@, out@) == goals_view(arena.nodes@, goals@).skip(from as int),
{
    let ghost models = goals_view(arena.nodes@, goals@);
    let mut i = from;
    let mut out = Vec::new();
    proof {
        assert_seqs_equal!(models.subrange(from as int, from as int) == Seq::empty());
    }
    while i < goals.len()
        invariant
            arena_ok(arena),
            goals_valid(arena.nodes@, goals@),
            goals_levels(goals@, level),
            from <= i <= goals.len(),
            models == goals_view(arena.nodes@, goals@),
            out.len() == i - from,
            goals_valid(arena.nodes@, out@),
            goals_levels(out@, level),
            goals_view(arena.nodes@, out@) == models.subrange(from as int, i as int),
        decreases goals.len() - i,
    {
        proof {
            assert(goal_valid(arena.nodes@, &goals@[i as int]));
            assert(goal_level(&goals@[i as int], level));
        }
        let next = match &goals[i] {
            EGoal::Lit { root, depth, path } => EGoal::Lit {
                root: *root,
                depth: *depth,
                path: path.clone(),
            },
            EGoal::NafCut { level } => EGoal::NafCut { level: *level },
        };
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies {
                &&& goal_valid(arena.nodes@, &out@[j])
                &&& goal_level(&out@[j], level)
            } by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                    assert(goal_valid(arena.nodes@, &before[j]));
                    assert(goal_level(&before[j], level));
                }
            }
            assert_seqs_equal!(goals_view(arena.nodes@, out@) == goals_view(arena.nodes@, before).push(goal_view(arena.nodes@, &next)));
            assert_seqs_equal!(models.subrange(from as int, i as int + 1) == models.subrange(from as int, i as int).push(models[i as int]));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.subrange(from as int, i as int) == models.skip(from as int));
    }
    out
}

pub fn in_naf_exec(arena: &ETermArena, goals: &Vec<EGoal>) -> (out: bool)
    ensures
        out == in_naf(goals_view(arena.nodes@, goals@)),
{
    let mut i = 0usize;
    while i < goals.len()
        invariant
            i <= goals.len(),
            forall|j: int| 0 <= j < i ==> !(goals_view(arena.nodes@, goals@)[j] is NafCut),
        decreases goals.len() - i,
    {
        if matches!(&goals[i], EGoal::NafCut { .. }) {
            proof {
                assert(goals_view(arena.nodes@, goals@)[i as int] is NafCut);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn fail(arena: &ETermArena, initial: ECfg, Ghost(db_len): Ghost<nat>) -> (out: EStep)
    requires
        cfg_valid(arena.nodes@, db_len, &initial),
    ensures
        step_valid(arena.nodes@, db_len, &out),
        step_view(arena.nodes@, &out) == tfail(cfg_view(arena.nodes@, &initial)),
{
    let ghost old_model = cfg_view(arena.nodes@, &initial);
    let mut c = initial;
    if c.alts.len() == 0 {
        return EStep::Done(!c.pruned);
    }
    let ghost old_alts = c.alts@;
    let ghost old_log = c.log@;
    proof {
        assert(alts_valid(arena.nodes@, old_log.len(), old_alts));
        assert(marks_sorted(old_alts));
        assert(log_valid(arena.nodes@, db_len, old_log));
        assert(alt_valid(
            arena.nodes@,
            old_log.len(),
            &old_alts[old_alts.len() as int - 1],
            (old_alts.len() - 1) as nat,
        ));
    }
    let a = c.alts.pop().unwrap();
    let ghost mark = alt_mark(&a);
    proof {
        assert(a == old_alts.last());
        assert_seqs_equal!(c.alts@ == old_alts.drop_last());
        assert_seqs_equal!(alts_view(arena.nodes@, old_log, c.alts@) == old_model.alts.drop_last());
        assert(alt_view(arena.nodes@, old_log, &a) == old_model.alts.last());
        assert(alt_valid(arena.nodes@, old_log.len(), &a, c.alts.len() as nat));
        assert forall|i: int| 0 <= i < c.alts.len() implies alt_mark(&c.alts@[i]) <= mark by {
            assert(c.alts@[i] == old_alts[i]);
        }
        assert forall|i: int| 0 <= i < c.alts.len() implies #[trigger] alt_valid(
            arena.nodes@,
            mark,
            &c.alts@[i],
            i as nat,
        ) by {
            assert(alt_valid(arena.nodes@, old_log.len(), &old_alts[i], i as nat));
            assert(c.alts@[i] == old_alts[i]);
        }
        assert(alts_valid(arena.nodes@, mark, c.alts@));
        assert(marks_sorted(c.alts@));
    }
    match a {
        EAlt::Cl { stack, fresh, ci, log_len } => {
            c.log.truncate(log_len);
            proof {
                assert_seqs_equal!(c.log@ == old_log.take(log_len as int));
                alts_log_prefix(arena.nodes@, c.log@, old_log, c.alts@);
            }
            c.stack = stack;
            c.fresh = fresh;
            c.ci = ci;
            EStep::Next(c)
        },
        EAlt::Naf { stack, fresh, log_len, path, inner, pruned } => {
            if c.pruned {
                return EStep::Limit;
            }
            c.log.truncate(log_len);
            proof {
                assert_seqs_equal!(c.log@ == old_log.take(log_len as int));
                alts_log_prefix(arena.nodes@, c.log@, old_log, c.alts@);
            }
            c.stack = stack;
            c.fresh = fresh;
            c.ci = 0;
            c.pruned = pruned;
            if !in_naf_exec(arena, &c.stack) {
                let ghost before = c.log@;
                c.log.push(EEntry { path, event: EEvent::Naf(inner) });
                proof {
                    alts_log_prefix(arena.nodes@, before, c.log@, c.alts@);
                    assert forall|i: int| 0 <= i < c.log.len() implies #[trigger] entry_valid(
                        arena.nodes@,
                        db_len,
                        &c.log@[i],
                    ) by {
                        if i < before.len() {
                            assert(c.log@[i] == before[i]);
                        }
                    }
                    assert_seqs_equal!(log_view(arena.nodes@, c.log@) == log_view(arena.nodes@, before).push((path_view(path@), TEv::Naf(arena.nodes@[inner as int].term@))));
                }
            }
            EStep::Next(c)
        },
    }
}

} // verus!
