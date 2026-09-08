use crate::k2_term::{ENode, ENodeKind, EOrder, ETermArena, int_order, push_comp, push_var};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{
    arena_ok, arena_prefix_stable, child_roots_before, child_terms, int_order_ok, node_ok, root_ok,
};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn roots_valid(nodes: Seq<ENode>, roots: Seq<usize>) -> bool {
    forall|i: int| 0 <= i < roots.len() ==> roots[i] < nodes.len()
}

pub open spec fn root_terms(nodes: Seq<ENode>, roots: Seq<usize>) -> Seq<Term> {
    Seq::new(roots.len(), |i: int| nodes[roots[i] as int].term@)
}

proof fn node_var_model(nodes: Seq<ENode>, i: int, key: usize)
    requires
        0 <= i < nodes.len(),
        node_ok(nodes, i),
        match &nodes[i].kind {
            ENodeKind::Var { key: stored, .. } => *stored == key,
            _ => false,
        },
    ensures
        nodes[i].term@ == Term::Var(key as nat),
{
    reveal(node_ok);
    match nodes[i].term@ {
        Term::Var(k) => {
            assert(k == key as nat);
        },
        _ => {
            assert(false);
        },
    }
}

pub proof fn node_comp_model(nodes: Seq<ENode>, i: int, name: Seq<u8>, roots: Seq<usize>)
    requires
        0 <= i < nodes.len(),
        node_ok(nodes, i),
        match &nodes[i].kind {
            ENodeKind::Comp { name: stored_name, child_roots, .. } => {
                &&& stored_name@ == name
                &&& child_roots@ == roots
            },
            _ => false,
        },
    ensures
        nodes[i].term@ == Term::Comp(name, child_terms(nodes, roots)),
        roots_valid(nodes, roots),
{
    reveal(node_ok);
    reveal(roots_valid);
    match nodes[i].term@ {
        Term::Comp(spec_name, args) => {
            assert(spec_name == name);
            assert(args == child_terms(nodes, roots));
        },
        _ => {
            assert(false);
        },
    }
}

proof fn root_terms_prefix(before: Seq<ENode>, after: Seq<ENode>, roots: Seq<usize>)
    requires
        before.is_prefix_of(after),
        roots_valid(before, roots),
    ensures
        roots_valid(after, roots),
        root_terms(before, roots) == root_terms(after, roots),
{
    reveal(roots_valid);
    reveal(root_terms);
    assert_seqs_equal!(root_terms(before, roots) == root_terms(after, roots));
}

proof fn root_terms_push(nodes: Seq<ENode>, roots: Seq<usize>, root: usize)
    requires
        root < nodes.len(),
    ensures
        root_terms(nodes, roots.push(root)) == root_terms(nodes, roots).push(
            nodes[root as int].term@,
        ),
{
    reveal(root_terms);
    assert_seqs_equal!(
        root_terms(nodes, roots.push(root))
            == root_terms(nodes, roots).push(nodes[root as int].term@)
    );
}

proof fn roots_valid_push(nodes: Seq<ENode>, roots: Seq<usize>, root: usize)
    requires
        roots_valid(nodes, roots),
        root < nodes.len(),
    ensures
        roots_valid(nodes, roots.push(root)),
{
    reveal(roots_valid);
}

proof fn subst_all_map(ts: Seq<Term>, x: nat, value: Term)
    ensures
        ckc_spec::engine::subst_all(ts, x, value) == ts.map_values(
            |t: Term| ckc_spec::engine::subst(t, x, value),
        ),
    decreases ts.len(),
{
    reveal_with_fuel(ckc_spec::engine::subst_all, 2);
    if ts.len() > 0 {
        subst_all_map(ts.drop_first(), x, value);
        assert_seqs_equal!(
            ts.map_values(|t: Term| ckc_spec::engine::subst(t, x, value))
                == seq![ckc_spec::engine::subst(ts[0], x, value)]
                    + ts.drop_first().map_values(
                        |t: Term| ckc_spec::engine::subst(t, x, value),
                    )
        );
    }
}

fn append_udec(n: usize, out: &mut Vec<u8>)
    ensures
        final(out)@ == old(out)@ + ckc_spec::v1text::udec_bytes(n as nat),
{
    let ghost base = out@;
    let mut remaining = n;
    let mut digits: Vec<u8> = Vec::new();
    while remaining >= 10
        invariant
            ckc_spec::v1text::udec_bytes(n as nat) == ckc_spec::v1text::udec_bytes(remaining as nat)
                + digits@.reverse(),
        decreases remaining,
    {
        let digit = 0x30u8 + (remaining % 10) as u8;
        let next = remaining / 10;
        let ghost prior = digits@;
        digits.push(digit);
        proof {
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
            assert_seqs_equal!(digits@.reverse() == seq![digit] + prior.reverse());
        }
        remaining = next;
    }
    let digit = 0x30u8 + remaining as u8;
    let ghost prior = digits@;
    digits.push(digit);
    proof {
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
        reveal(ckc_spec::v1text::digit_byte);
        assert_seqs_equal!(digits@.reverse() == seq![digit] + prior.reverse());
        assert(digits@.reverse() == ckc_spec::v1text::udec_bytes(n as nat));
    }
    while digits.len() > 0
        invariant
            base == old(out)@,
            out@ + digits@.reverse() == base + ckc_spec::v1text::udec_bytes(n as nat),
        decreases digits.len(),
    {
        let ghost before = digits@;
        let digit = digits.pop().unwrap();
        proof {
            assert_seqs_equal!(before.reverse() == seq![digit] + digits@.reverse());
        }
        out.push(digit);
    }
}

pub fn var_spelling(key: usize) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::var_bytes(key as nat),
{
    let letter = 0x41u8 + (key % 26) as u8;
    let mut out = Vec::new();
    out.push(letter);
    let q = key / 26;
    if q > 0 {
        append_udec(q, &mut out);
    }
    proof {
        reveal(ckc_spec::v1text::var_bytes);
    }
    out
}

proof fn mapped_children_subst(
    base: Seq<ENode>,
    current: Seq<ENode>,
    map: Seq<usize>,
    roots: Seq<usize>,
    x: nat,
    value: Term,
)
    requires
        base.is_prefix_of(current),
        roots_valid(base, roots),
        forall|i: int| 0 <= i < roots.len() ==> roots[i] < map.len(),
        forall|i: int|
            0 <= i < map.len() ==> {
                &&& map[i] < current.len()
                &&& current[map[i] as int].term@ == ckc_spec::engine::subst(base[i].term@, x, value)
            },
    ensures
        roots_valid(current, map_values_at(map, roots)),
        root_terms(current, map_values_at(map, roots)) == ckc_spec::engine::subst_all(
            root_terms(base, roots),
            x,
            value,
        ),
{
    mapped_children(base, current, map, roots, |t: Term| ckc_spec::engine::subst(t, x, value));
    subst_all_map(root_terms(base, roots), x, value);
}

proof fn subst_map_push(
    base: Seq<ENode>,
    current: Seq<ENode>,
    map: Seq<usize>,
    i: usize,
    root: usize,
    x: nat,
    value: Term,
)
    requires
        map.len() == i,
        i < base.len(),
        root < current.len(),
        current[root as int].term@ == ckc_spec::engine::subst(base[i as int].term@, x, value),
        forall|j: int|
            0 <= j < map.len() ==> {
                &&& map[j] < current.len()
                &&& current[map[j] as int].term@ == ckc_spec::engine::subst(base[j].term@, x, value)
            },
    ensures
        forall|j: int|
            0 <= j < map.push(root).len() ==> {
                &&& map.push(root)[j] < current.len()
                &&& current[map.push(root)[j] as int].term@ == ckc_spec::engine::subst(
                    base[j].term@,
                    x,
                    value,
                )
            },
{
    assert forall|j: int| 0 <= j < map.push(root).len() implies {
        &&& map.push(root)[j] < current.len()
        &&& current[map.push(root)[j] as int].term@ == ckc_spec::engine::subst(
            base[j].term@,
            x,
            value,
        )
    } by {
        if j < map.len() {
            assert(map.push(root)[j] == map[j]);
        } else {
            assert(j == map.len());
            assert(j == i);
            assert(map.push(root)[j] == root);
        }
    }
}

pub open spec fn map_values_at(map: Seq<usize>, roots: Seq<usize>) -> Seq<usize> {
    Seq::new(roots.len(), |i: int| map[roots[i] as int])
}

proof fn shift_closed(t: Term, off: nat)
    requires
        ckc_spec::engine::nvars(t) == 0,
    ensures
        ckc_spec::engine::shift(t, off) == t,
    decreases t,
{
    reveal(ckc_spec::engine::nvars);
    reveal(ckc_spec::engine::shift);
    match t {
        Term::Comp(_, args) => shift_closed_all(args, off),
        _ => {},
    }
}

proof fn shift_closed_all(ts: Seq<Term>, off: nat)
    requires
        ckc_spec::engine::nvars_all(ts) == 0,
    ensures
        ckc_spec::engine::shift_all(ts, off) == ts,
    decreases ts,
{
    reveal_with_fuel(ckc_spec::engine::nvars_all, 1);
    reveal_with_fuel(ckc_spec::engine::shift_all, 1);
    if ts.len() > 0 {
        shift_closed(ts[0], off);
        shift_closed_all(ts.drop_first(), off);
        assert_seqs_equal!(seq![ts[0]] + ts.drop_first() == ts);
    } else {
        assert_seqs_equal!(Seq::<Term>::empty() == ts);
    }
}

proof fn subst_absent(t: Term, x: nat, value: Term)
    requires
        !ckc_spec::engine::occurs(x, t),
    ensures
        ckc_spec::engine::subst(t, x, value) == t,
    decreases t,
{
    reveal(ckc_spec::engine::occurs);
    reveal(ckc_spec::engine::subst);
    match t {
        Term::Comp(_, args) => subst_absent_all(args, x, value),
        _ => {},
    }
}

proof fn subst_absent_all(ts: Seq<Term>, x: nat, value: Term)
    requires
        !ckc_spec::engine::occurs_all(x, ts),
    ensures
        ckc_spec::engine::subst_all(ts, x, value) == ts,
    decreases ts,
{
    reveal_with_fuel(ckc_spec::engine::occurs_all, 1);
    reveal_with_fuel(ckc_spec::engine::subst_all, 1);
    if ts.len() > 0 {
        subst_absent(ts[0], x, value);
        subst_absent_all(ts.drop_first(), x, value);
        assert_seqs_equal!(seq![ts[0]] + ts.drop_first() == ts);
    } else {
        assert_seqs_equal!(Seq::<Term>::empty() == ts);
    }
}

proof fn occurs_bound(t: Term, x: nat)
    requires
        ckc_spec::engine::nvars(t) <= x,
    ensures
        !ckc_spec::engine::occurs(x, t),
    decreases t,
{
    reveal(ckc_spec::engine::nvars);
    reveal(ckc_spec::engine::occurs);
    match t {
        Term::Comp(_, args) => occurs_bound_all(args, x),
        _ => {},
    }
}

proof fn occurs_bound_all(ts: Seq<Term>, x: nat)
    requires
        ckc_spec::engine::nvars_all(ts) <= x,
    ensures
        !ckc_spec::engine::occurs_all(x, ts),
    decreases ts,
{
    reveal_with_fuel(ckc_spec::engine::nvars_all, 1);
    reveal_with_fuel(ckc_spec::engine::occurs_all, 1);
    if ts.len() > 0 {
        occurs_bound(ts[0], x);
        occurs_bound_all(ts.drop_first(), x);
    }
}

#[verifier::rlimit(5000)]
pub fn subst_root(arena: &mut ETermArena, root: usize, x: usize, replacement: usize) -> (result:
    usize)
    requires
        root_ok(old(arena), root),
        replacement < old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        result < final(arena).nodes@.len(),
        final(arena)@[result as int] == ckc_spec::engine::subst(
            old(arena)@[root as int],
            x as nat,
            old(arena)@[replacement as int],
        ),
{
    if !occurs_root(arena, x, root) {
        proof {
            subst_absent(arena@[root as int], x as nat, arena@[replacement as int]);
        }
        return root;
    }
    let vars: Vec<usize> = Vec::new();
    let mut owned = crate::k2_reject::empty_arena();
    std::mem::swap(arena, &mut owned);
    let (mut owned, result) = crate::k2_rewrite::rewrite_root(
        owned,
        root,
        false,
        x,
        replacement,
        &vars,
    );
    std::mem::swap(arena, &mut owned);
    result
}

pub struct EPair {
    pub left: usize,
    pub right: usize,
}

#[derive(Clone, Copy)]
pub enum EGoal {
    Lit { root: usize, depth: usize },
    NafCut { level: usize },
}

pub enum EUResult {
    Ok { stack: Vec<EGoal>, sol: Vec<usize> },
    Fail,
}

pub open spec fn pairs_view(nodes: Seq<ENode>, pairs: Seq<EPair>) -> Seq<(Term, Term)> {
    Seq::new(
        pairs.len(),
        |i: int| (nodes[pairs[i].left as int].term@, nodes[pairs[i].right as int].term@),
    )
}

pub open spec fn pair_roots_valid(nodes: Seq<ENode>, pairs: Seq<EPair>) -> bool {
    forall|i: int|
        0 <= i < pairs.len() ==> {
            &&& pairs[i].left < nodes.len()
            &&& pairs[i].right < nodes.len()
        }
}

pub open spec fn goal_view(nodes: Seq<ENode>, goal: &EGoal) -> ckc_spec::engine::Goal {
    match goal {
        EGoal::Lit { root, depth } => {
            ckc_spec::engine::Goal::Lit(nodes[*root as int].term@, *depth as nat)
        },
        EGoal::NafCut { level } => ckc_spec::engine::Goal::NafCut(*level as nat),
    }
}

pub open spec fn goals_view(nodes: Seq<ENode>, goals: Seq<EGoal>) -> Seq<ckc_spec::engine::Goal> {
    Seq::new(goals.len(), |i: int| goal_view(nodes, &goals[i]))
}

pub open spec fn goal_valid(nodes: Seq<ENode>, goal: &EGoal) -> bool {
    match goal {
        EGoal::Lit { root, .. } => *root < nodes.len(),
        EGoal::NafCut { .. } => true,
    }
}

pub open spec fn goals_valid(nodes: Seq<ENode>, goals: Seq<EGoal>) -> bool {
    forall|i: int| 0 <= i < goals.len() ==> #[trigger] goal_valid(nodes, &goals[i])
}

pub open spec fn ustate_view(
    nodes: Seq<ENode>,
    pairs: Seq<EPair>,
    stack: Seq<EGoal>,
    sol: Seq<usize>,
) -> ckc_spec::engine::UState {
    ckc_spec::engine::UState {
        pairs: pairs_view(nodes, pairs),
        stack: goals_view(nodes, stack),
        sol: root_terms(nodes, sol),
    }
}

pub open spec fn uresult_view(nodes: Seq<ENode>, result: &EUResult) -> ckc_spec::engine::UOut {
    match result {
        EUResult::Ok { stack, sol } => {
            ckc_spec::engine::UOut::Ok(goals_view(nodes, stack@), root_terms(nodes, sol@))
        },
        EUResult::Fail => ckc_spec::engine::UOut::Fail,
    }
}

pub proof fn roots_models_prefix(before: Seq<ENode>, after: Seq<ENode>, roots: Seq<usize>)
    requires
        before.is_prefix_of(after),
        roots_valid(before, roots),
    ensures
        roots_valid(after, roots),
        root_terms(after, roots) == root_terms(before, roots),
        forall|i: int|
            0 <= i < roots.len() ==> after[roots[i] as int].term@ == before[roots[i] as int].term@,
{
    root_terms_prefix(before, after, roots);
    reveal(root_terms);
    reveal(roots_valid);
}

#[verifier::rlimit(5000)]
fn subst_roots(arena: &mut ETermArena, roots: &Vec<usize>, x: usize, replacement: usize) -> (out:
    Vec<usize>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
        replacement < old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        roots_valid(final(arena).nodes@, out@),
        root_terms(final(arena).nodes@, out@) == ckc_spec::engine::subst_all(
            root_terms(old(arena).nodes@, roots@),
            x as nat,
            old(arena)@[replacement as int],
        ),
{
    let ghost base = arena.nodes@;
    let ghost value = arena@[replacement as int];
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < roots.len()
        invariant
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            roots_valid(base, roots@),
            replacement < base.len(),
            value == arena@[replacement as int],
            i <= roots@.len(),
            out@.len() == i,
            roots_valid(arena.nodes@, out@),
            forall|j: int|
                0 <= j < out@.len() ==> {
                    arena@[out@[j] as int] == ckc_spec::engine::subst(
                        base[roots@[j] as int].term@,
                        x as nat,
                        value,
                    )
                },
        decreases roots.len() - i,
    {
        let source = roots[i];
        let ghost before = arena.nodes@;
        let result = subst_root(arena, source, x, replacement);
        proof {
            roots_models_prefix(before, arena.nodes@, out@);
            assert forall|j: int| 0 <= j < out@.len() implies arena@[out@[j] as int]
                == ckc_spec::engine::subst(base[roots@[j] as int].term@, x as nat, value) by {
                assert(out@[j] < before.len());
            }
            assert(arena@[result as int] == ckc_spec::engine::subst(
                base[source as int].term@,
                x as nat,
                value,
            ));
            roots_valid_push(arena.nodes@, out@, result);
        }
        out.push(result);
        i += 1;
    }
    proof {
        reveal(root_terms);
        subst_all_map(root_terms(base, roots@), x as nat, value);
        assert_seqs_equal!(
            root_terms(arena.nodes@, out@)
                == root_terms(base, roots@).map_values(
                    |t: Term| ckc_spec::engine::subst(t, x as nat, value),
                )
        );
    }
    out
}

pub open spec fn subst_pairs_model(pairs: Seq<(Term, Term)>, x: nat, value: Term) -> Seq<
    (Term, Term),
> {
    pairs.map_values(
        |p: (Term, Term)|
            (ckc_spec::engine::subst(p.0, x, value), ckc_spec::engine::subst(p.1, x, value)),
    )
}

#[verifier::rlimit(5000)]
fn subst_pairs(arena: &mut ETermArena, pairs: &Vec<EPair>, x: usize, replacement: usize) -> (out:
    Vec<EPair>)
    requires
        arena_ok(old(arena)),
        pair_roots_valid(old(arena).nodes@, pairs@),
        replacement < old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        pair_roots_valid(final(arena).nodes@, out@),
        pairs_view(final(arena).nodes@, out@) == subst_pairs_model(
            pairs_view(old(arena).nodes@, pairs@),
            x as nat,
            old(arena)@[replacement as int],
        ),
{
    let ghost base = arena.nodes@;
    let ghost value = arena@[replacement as int];
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < pairs.len()
        invariant
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            pair_roots_valid(base, pairs@),
            replacement < base.len(),
            value == arena@[replacement as int],
            i <= pairs@.len(),
            out@.len() == i,
            pair_roots_valid(arena.nodes@, out@),
            forall|j: int|
                0 <= j < out@.len() ==> {
                    &&& arena@[out@[j].left as int] == ckc_spec::engine::subst(
                        base[pairs@[j].left as int].term@,
                        x as nat,
                        value,
                    )
                    &&& arena@[out@[j].right as int] == ckc_spec::engine::subst(
                        base[pairs@[j].right as int].term@,
                        x as nat,
                        value,
                    )
                },
        decreases pairs.len() - i,
    {
        let left_source = pairs[i].left;
        let right_source = pairs[i].right;
        let ghost before_left = arena.nodes@;
        let left = subst_root(arena, left_source, x, replacement);
        let ghost before_right = arena.nodes@;
        let right = subst_root(arena, right_source, x, replacement);
        proof {
            arena_prefix_stable(before_right, arena);
            assert(left < before_right.len());
            assert(arena@[left as int] == before_right[left as int].term@);
            assert(arena@[left as int] == ckc_spec::engine::subst(
                base[left_source as int].term@,
                x as nat,
                value,
            ));
            assert(arena@[right as int] == ckc_spec::engine::subst(
                base[right_source as int].term@,
                x as nat,
                value,
            ));
            assert(before_left.is_prefix_of(arena.nodes@));
            assert forall|j: int| 0 <= j < out@.len() implies {
                &&& arena@[out@[j].left as int] == ckc_spec::engine::subst(
                    base[pairs@[j].left as int].term@,
                    x as nat,
                    value,
                )
                &&& arena@[out@[j].right as int] == ckc_spec::engine::subst(
                    base[pairs@[j].right as int].term@,
                    x as nat,
                    value,
                )
            } by {
                assert(out@[j].left < before_left.len());
                assert(out@[j].right < before_left.len());
                arena_prefix_stable(before_left, arena);
            }
        }
        out.push(EPair { left, right });
        i += 1;
    }
    proof {
        reveal(pairs_view);
        reveal(subst_pairs_model);
        assert_seqs_equal!(
            pairs_view(arena.nodes@, out@)
                == pairs_view(base, pairs@).map_values(|p: (Term, Term)| (
                    ckc_spec::engine::subst(p.0, x as nat, value),
                    ckc_spec::engine::subst(p.1, x as nat, value),
                ))
        );
    }
    out
}

proof fn goals_models_prefix(before: Seq<ENode>, after: Seq<ENode>, goals: Seq<EGoal>)
    requires
        before.is_prefix_of(after),
        goals_valid(before, goals),
    ensures
        goals_valid(after, goals),
        goals_view(after, goals) == goals_view(before, goals),
        forall|i: int|
            0 <= i < goals.len() ==> goal_view(after, &goals[i]) == goal_view(before, &goals[i]),
{
    reveal(goals_valid);
    assert forall|i: int| 0 <= i < goals.len() implies goal_valid(after, &goals[i]) by {
        assert(goal_valid(before, &goals[i]));
        reveal(goal_valid);
        match &goals[i] {
            EGoal::Lit { root, .. } => {
                assert(*root < before.len());
                assert(before[*root as int] == after[*root as int]);
            },
            EGoal::NafCut { .. } => {},
        }
    }
    assert forall|i: int| 0 <= i < goals.len() implies goal_view(after, &goals[i]) == goal_view(
        before,
        &goals[i],
    ) by {
        assert(goal_valid(before, &goals[i]));
        reveal(goal_valid);
        reveal(goal_view);
        match &goals[i] {
            EGoal::Lit { root, .. } => {
                assert(*root < before.len());
                assert(before[*root as int] == after[*root as int]);
            },
            EGoal::NafCut { .. } => {},
        }
    }
    reveal(goals_view);
    assert_seqs_equal!(goals_view(after, goals) == goals_view(before, goals));
}

proof fn goals_model_push(
    nodes: Seq<ENode>,
    base: Seq<ENode>,
    source: Seq<EGoal>,
    prior: Seq<EGoal>,
    next: EGoal,
    i: usize,
    x: nat,
    value: Term,
)
    requires
        prior.len() == i,
        i < source.len(),
        goals_valid(nodes, prior),
        goal_valid(nodes, &next),
        forall|j: int|
            0 <= j < prior.len() ==> {
                goal_view(nodes, &prior[j]) == ckc_spec::engine::subst_goal(
                    goal_view(base, &source[j]),
                    x,
                    value,
                )
            },
        goal_view(nodes, &next) == ckc_spec::engine::subst_goal(
            goal_view(base, &source[i as int]),
            x,
            value,
        ),
    ensures
        goals_valid(nodes, prior.push(next)),
        forall|j: int|
            0 <= j < prior.push(next).len() ==> {
                goal_view(nodes, &prior.push(next)[j]) == ckc_spec::engine::subst_goal(
                    goal_view(base, &source[j]),
                    x,
                    value,
                )
            },
{
    reveal(goals_valid);
    assert forall|j: int| 0 <= j < prior.push(next).len() implies goal_valid(
        nodes,
        &prior.push(next)[j],
    ) by {
        if j < prior.len() {
            assert(prior.push(next)[j] == prior[j]);
        } else {
            assert(j == prior.len());
            assert(prior.push(next)[j] == next);
        }
    }
    assert forall|j: int| 0 <= j < prior.push(next).len() implies goal_view(
        nodes,
        &prior.push(next)[j],
    ) == ckc_spec::engine::subst_goal(goal_view(base, &source[j]), x, value) by {
        if j < prior.len() {
            assert(prior.push(next)[j] == prior[j]);
        } else {
            assert(j == prior.len());
            assert(j == i);
            assert(prior.push(next)[j] == next);
        }
    }
}

#[verifier::rlimit(5000)]
fn subst_goals(arena: &mut ETermArena, goals: &Vec<EGoal>, x: usize, replacement: usize) -> (out:
    Vec<EGoal>)
    requires
        arena_ok(old(arena)),
        goals_valid(old(arena).nodes@, goals@),
        replacement < old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        goals_valid(final(arena).nodes@, out@),
        goals_view(final(arena).nodes@, out@) == goals_view(old(arena).nodes@, goals@).map_values(
            |g: ckc_spec::engine::Goal|
                ckc_spec::engine::subst_goal(g, x as nat, old(arena)@[replacement as int]),
        ),
{
    let ghost base = arena.nodes@;
    let ghost value = arena@[replacement as int];
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < goals.len()
        invariant
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            goals_valid(base, goals@),
            replacement < base.len(),
            value == arena@[replacement as int],
            i <= goals@.len(),
            out@.len() == i,
            goals_valid(arena.nodes@, out@),
            forall|j: int|
                0 <= j < out@.len() ==> {
                    goal_view(arena.nodes@, &out@[j]) == ckc_spec::engine::subst_goal(
                        goal_view(base, &goals@[j]),
                        x as nat,
                        value,
                    )
                },
        decreases goals.len() - i,
    {
        match &goals[i] {
            EGoal::Lit { root, depth } => {
                let source = *root;
                let d = *depth;
                let ghost before = arena.nodes@;
                proof {
                    assert(goals_valid(before, out@));
                    assert forall|j: int| 0 <= j < out@.len() implies goal_view(before, &out@[j])
                        == ckc_spec::engine::subst_goal(
                        goal_view(base, &goals@[j]),
                        x as nat,
                        value,
                    ) by {}
                    reveal(goals_valid);
                    assert(goal_valid(base, &goals@[i as int]));
                    reveal(goal_valid);
                    assert(source < base.len());
                    assert(base.len() <= before.len());
                    assert(source < before.len());
                    assert(replacement < before.len());
                    reveal(root_ok);
                }
                let result = subst_root(arena, source, x, replacement);
                let ghost prior = out@;
                let ghost next = EGoal::Lit { root: result, depth: d };
                proof {
                    arena_prefix_stable(before, arena);
                    goals_models_prefix(before, arena.nodes@, prior);
                    assert forall|j: int| 0 <= j < prior.len() implies goal_view(
                        arena.nodes@,
                        &prior[j],
                    ) == ckc_spec::engine::subst_goal(
                        goal_view(base, &goals@[j]),
                        x as nat,
                        value,
                    ) by {
                        assert(goal_view(arena.nodes@, &prior[j]) == goal_view(before, &prior[j]));
                        assert(goal_view(before, &prior[j]) == ckc_spec::engine::subst_goal(
                            goal_view(base, &goals@[j]),
                            x as nat,
                            value,
                        ));
                    }
                    reveal(goal_valid);
                    assert(goal_valid(arena.nodes@, &next));
                    reveal(goal_view);
                    reveal(ckc_spec::engine::subst_goal);
                    assert(goal_view(arena.nodes@, &next) == ckc_spec::engine::subst_goal(
                        goal_view(base, &goals@[i as int]),
                        x as nat,
                        value,
                    ));
                    goals_model_push(arena.nodes@, base, goals@, prior, next, i, x as nat, value);
                }
                out.push(EGoal::Lit { root: result, depth: d });
                proof {
                    assert(out@ == prior.push(next));
                }
            },
            EGoal::NafCut { level } => {
                let l = *level;
                let ghost prior = out@;
                let ghost next = EGoal::NafCut { level: l };
                proof {
                    reveal(goal_valid);
                    assert(goal_valid(arena.nodes@, &next));
                    reveal(goal_view);
                    reveal(ckc_spec::engine::subst_goal);
                    assert(goal_view(arena.nodes@, &next) == ckc_spec::engine::subst_goal(
                        goal_view(base, &goals@[i as int]),
                        x as nat,
                        value,
                    ));
                    goals_model_push(arena.nodes@, base, goals@, prior, next, i, x as nat, value);
                }
                out.push(EGoal::NafCut { level: l });
                proof {
                    assert(out@ == prior.push(next));
                }
            },
        }
        i += 1;
    }
    proof {
        reveal(goals_view);
        assert_seqs_equal!(
            goals_view(arena.nodes@, out@)
                == goals_view(base, goals@).map_values(
                    |g: ckc_spec::engine::Goal| ckc_spec::engine::subst_goal(
                        g,
                        x as nat,
                        value,
                    ),
                )
        );
    }
    out
}

pub open spec fn roots_occurs(nodes: Seq<ENode>, roots: Seq<usize>, x: nat) -> bool
    decreases roots.len(),
{
    roots.len() > 0 && (ckc_spec::engine::occurs(x, nodes[roots[0] as int].term@) || roots_occurs(
        nodes,
        roots.drop_first(),
        x,
    ))
}

pub open spec fn term_size(t: Term) -> nat
    decreases t, 0int,
{
    match t {
        Term::Comp(_, args) => 1 + terms_size(args),
        _ => 1,
    }
}

pub open spec fn terms_size(ts: Seq<Term>) -> nat
    decreases ts, 0int,
{
    if ts.len() == 0 {
        0
    } else {
        term_size(ts[0]) + terms_size(ts.drop_first())
    }
}

pub open spec fn roots_work(nodes: Seq<ENode>, roots: Seq<usize>) -> nat
    decreases roots.len(),
{
    if roots.len() == 0 {
        0
    } else {
        term_size(nodes[roots[0] as int].term@) + roots_work(nodes, roots.drop_first())
    }
}

proof fn roots_occurs_concat(nodes: Seq<ENode>, left: Seq<usize>, right: Seq<usize>, x: nat)
    ensures
        roots_occurs(nodes, left + right, x) == (roots_occurs(nodes, left, x) || roots_occurs(
            nodes,
            right,
            x,
        )),
    decreases left.len(),
{
    reveal_with_fuel(roots_occurs, 2);
    if left.len() > 0 {
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
        roots_occurs_concat(nodes, left.drop_first(), right, x);
    }
}

pub proof fn roots_work_concat(nodes: Seq<ENode>, left: Seq<usize>, right: Seq<usize>)
    ensures
        roots_work(nodes, left + right) == roots_work(nodes, left) + roots_work(nodes, right),
    decreases left.len(),
{
    reveal_with_fuel(roots_work, 2);
    if left.len() > 0 {
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
        roots_work_concat(nodes, left.drop_first(), right);
    }
}

proof fn occurs_all_root_terms(nodes: Seq<ENode>, roots: Seq<usize>, x: nat)
    requires
        roots_valid(nodes, roots),
    ensures
        ckc_spec::engine::occurs_all(x, root_terms(nodes, roots)) == roots_occurs(nodes, roots, x),
    decreases roots.len(),
{
    reveal_with_fuel(ckc_spec::engine::occurs_all, 2);
    reveal_with_fuel(roots_occurs, 2);
    reveal(root_terms);
    if roots.len() > 0 {
        assert(root_terms(nodes, roots).drop_first() == root_terms(nodes, roots.drop_first()));
        occurs_all_root_terms(nodes, roots.drop_first(), x);
    }
}

pub proof fn terms_size_root_terms(nodes: Seq<ENode>, roots: Seq<usize>)
    requires
        roots_valid(nodes, roots),
    ensures
        terms_size(root_terms(nodes, roots)) == roots_work(nodes, roots),
    decreases roots.len(),
{
    reveal_with_fuel(terms_size, 2);
    reveal_with_fuel(roots_work, 2);
    reveal(root_terms);
    if roots.len() > 0 {
        assert(root_terms(nodes, roots).drop_first() == root_terms(nodes, roots.drop_first()));
        terms_size_root_terms(nodes, roots.drop_first());
    }
}

#[verifier::rlimit(5000)]
pub fn occurs_root(arena: &ETermArena, x: usize, root: usize) -> (found: bool)
    requires
        root_ok(arena, root),
    ensures
        found == ckc_spec::engine::occurs(x as nat, arena@[root as int]),
{
    proof {
        reveal(root_ok);
    }
    let ghost model = arena@[root as int];
    let maximum = max_var_root(arena, root);
    let beyond = match maximum {
        None => true,
        Some(k) => k < x,
    };
    if beyond {
        proof {
            occurs_bound(arena@[root as int], x as nat);
        }
        return false;
    }
    let mut tasks = Vec::new();
    tasks.push(root);
    proof {
        reveal(roots_valid);
        reveal_with_fuel(roots_occurs, 2);
        reveal_with_fuel(roots_work, 2);
    }
    while tasks.len() > 0
        invariant
            arena_ok(arena),
            root < arena.nodes@.len(),
            model == arena@[root as int],
            roots_valid(arena.nodes@, tasks@),
            roots_occurs(arena.nodes@, tasks@, x as nat) == ckc_spec::engine::occurs(
                x as nat,
                model,
            ),
        decreases roots_work(arena.nodes@, tasks@),
    {
        let ghost before = tasks@;
        let ghost target = roots_occurs(arena.nodes@, before, x as nat);
        proof {
            assert(target == ckc_spec::engine::occurs(x as nat, model));
        }
        let current = tasks.remove(0);
        proof {
            assert(before.len() > 0);
            assert(current == before[0]);
            assert(tasks@ == before.drop_first());
            reveal(arena_ok);
            assert(node_ok(arena.nodes@, current as int));
            reveal(node_ok);
        }
        match &arena.nodes[current].kind {
            ENodeKind::Var { key, .. } => {
                let key_copy = *key;
                proof {
                    node_var_model(arena.nodes@, current as int, key_copy);
                    assert(arena@[current as int] == Term::Var(key_copy as nat));
                    reveal(ckc_spec::engine::occurs);
                    reveal_with_fuel(roots_occurs, 2);
                    reveal_with_fuel(roots_work, 2);
                    assert(target == ((key_copy == x) || roots_occurs(
                        arena.nodes@,
                        tasks@,
                        x as nat,
                    )));
                }
                if key_copy == x {
                    proof {
                        assert(target);
                        assert(ckc_spec::engine::occurs(x as nat, model));
                        assert(model == arena@[root as int]);
                        assert(ckc_spec::engine::occurs(x as nat, arena@[root as int]));
                    }
                    return true;
                }
            },
            ENodeKind::Comp { name, child_roots, .. } => {
                let mut children = child_roots.clone();
                let ghost child_seq = children@;
                let ghost rest = tasks@;
                proof {
                    node_comp_model(arena.nodes@, current as int, name@, child_seq);
                    assert(roots_valid(arena.nodes@, child_seq));
                    reveal(root_terms);
                    reveal(child_terms);
                    assert(root_terms(arena.nodes@, child_seq) == child_terms(
                        arena.nodes@,
                        child_seq,
                    ));
                    occurs_all_root_terms(arena.nodes@, child_seq, x as nat);
                    terms_size_root_terms(arena.nodes@, child_seq);
                    roots_occurs_concat(arena.nodes@, child_seq, rest, x as nat);
                    roots_work_concat(arena.nodes@, child_seq, rest);
                    reveal(ckc_spec::engine::occurs);
                    reveal(term_size);
                    reveal_with_fuel(roots_occurs, 2);
                    reveal_with_fuel(roots_work, 2);
                    assert(target == roots_occurs(arena.nodes@, child_seq + rest, x as nat));
                }
                children.append(&mut tasks);
                proof {
                    assert(children@ == child_seq + rest);
                    assert(roots_occurs(arena.nodes@, children@, x as nat) == target);
                }
                tasks = children;
            },
            _ => {
                proof {
                    reveal(ckc_spec::engine::occurs);
                    reveal_with_fuel(roots_occurs, 2);
                    reveal_with_fuel(roots_work, 2);
                    assert(target == roots_occurs(arena.nodes@, tasks@, x as nat));
                }
            },
        }
    }
    proof {
        reveal(roots_occurs);
    }
    false
}

proof fn unify_n_stable(u: ckc_spec::engine::UState, left: nat, right: nat)
    requires
        !(ckc_spec::engine::unify_n(u, left) is Out),
        !(ckc_spec::engine::unify_n(u, right) is Out),
    ensures
        ckc_spec::engine::unify_n(u, left) == ckc_spec::engine::unify_n(u, right),
    decreases left + right,
{
    reveal_with_fuel(ckc_spec::engine::unify_n, 2);
    assert(left > 0);
    assert(right > 0);
    if u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        match (a, b) {
            (Term::Var(x), _) => {
                if a == b {
                    unify_n_stable(
                        ckc_spec::engine::UState { pairs: rest, ..u },
                        (left - 1) as nat,
                        (right - 1) as nat,
                    );
                } else if !ckc_spec::engine::occurs(x, b) {
                    unify_n_stable(
                        ckc_spec::engine::u_bind(u, rest, x, b),
                        (left - 1) as nat,
                        (right - 1) as nat,
                    );
                }
            },
            (_, Term::Var(y)) => {
                if !ckc_spec::engine::occurs(y, a) {
                    unify_n_stable(
                        ckc_spec::engine::u_bind(u, rest, y, a),
                        (left - 1) as nat,
                        (right - 1) as nat,
                    );
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                if n == m && xs.len() == ys.len() {
                    unify_n_stable(
                        ckc_spec::engine::UState {
                            pairs: ckc_spec::engine::zip(xs, ys) + rest,
                            ..u
                        },
                        (left - 1) as nat,
                        (right - 1) as nat,
                    );
                }
            },
            _ => {
                if a == b {
                    unify_n_stable(
                        ckc_spec::engine::UState { pairs: rest, ..u },
                        (left - 1) as nat,
                        (right - 1) as nat,
                    );
                }
            },
        }
    }
}

proof fn unify_completed(u: ckc_spec::engine::UState, fuel: nat)
    requires
        !(ckc_spec::engine::unify_n(u, fuel) is Out),
    ensures
        ckc_spec::engine::unify(u) == ckc_spec::engine::unify_n(u, fuel),
{
    assert(exists|f: nat| !(ckc_spec::engine::unify_n(u, f) is Out));
    let chosen = choose|f: nat| !(ckc_spec::engine::unify_n(u, f) is Out);
    unify_n_stable(u, fuel, chosen);
    reveal(ckc_spec::engine::unify);
}

proof fn unify_step_preserves(before: ckc_spec::engine::UState, after: ckc_spec::engine::UState)
    requires
        forall|fuel: nat|
            fuel > 0 ==> #[trigger] ckc_spec::engine::unify_n(before, fuel)
                == ckc_spec::engine::unify_n(after, (fuel - 1) as nat),
    ensures
        ckc_spec::engine::unify(before) == ckc_spec::engine::unify(after),
{
    if exists|f: nat| !(ckc_spec::engine::unify_n(after, f) is Out) {
        let fuel = choose|f: nat| !(ckc_spec::engine::unify_n(after, f) is Out);
        assert(ckc_spec::engine::unify_n(before, fuel + 1) == ckc_spec::engine::unify_n(
            after,
            fuel,
        ));
        unify_completed(before, fuel + 1);
        unify_completed(after, fuel);
    } else {
        assert(!(exists|f: nat| !(ckc_spec::engine::unify_n(before, f) is Out))) by {
            if exists|f: nat| !(ckc_spec::engine::unify_n(before, f) is Out) {
                let fuel = choose|f: nat| !(ckc_spec::engine::unify_n(before, f) is Out);
                assert(fuel > 0);
                assert(ckc_spec::engine::unify_n(before, fuel) == ckc_spec::engine::unify_n(
                    after,
                    (fuel - 1) as nat,
                ));
            }
        }
        reveal(ckc_spec::engine::unify);
    }
}

pub open spec fn term_vars(t: Term) -> Set<nat>
    decreases t, 0int,
{
    match t {
        Term::Var(x) => Set::empty().insert(x),
        Term::Comp(_, args) => terms_vars(args),
        _ => Set::empty(),
    }
}

pub open spec fn terms_vars(ts: Seq<Term>) -> Set<nat>
    decreases ts, 1int,
{
    if ts.len() == 0 {
        Set::empty()
    } else {
        term_vars(ts[0]).union(terms_vars(ts.drop_first()))
    }
}

pub open spec fn pair_vars(pairs: Seq<(Term, Term)>) -> Set<nat>
    decreases pairs.len(),
{
    if pairs.len() == 0 {
        Set::empty()
    } else {
        term_vars(pairs[0].0).union(term_vars(pairs[0].1).union(pair_vars(pairs.drop_first())))
    }
}

pub open spec fn pair_work(pairs: Seq<(Term, Term)>) -> nat
    decreases pairs.len(),
{
    if pairs.len() == 0 {
        0
    } else {
        term_size(pairs[0].0) + term_size(pairs[0].1) + pair_work(pairs.drop_first())
    }
}

proof fn occurs_term_vars(t: Term, x: nat)
    ensures
        ckc_spec::engine::occurs(x, t) == term_vars(t).contains(x),
    decreases t, 0int,
{
    broadcast use vstd::set::group_set_lemmas;

    reveal(ckc_spec::engine::occurs);
    reveal(term_vars);
    match t {
        Term::Comp(_, args) => occurs_terms_vars(args, x),
        _ => {},
    }
}

proof fn occurs_terms_vars(ts: Seq<Term>, x: nat)
    ensures
        ckc_spec::engine::occurs_all(x, ts) == terms_vars(ts).contains(x),
    decreases ts, 1int,
{
    broadcast use vstd::set::group_set_lemmas;

    reveal_with_fuel(ckc_spec::engine::occurs_all, 2);
    reveal_with_fuel(terms_vars, 2);
    if ts.len() > 0 {
        occurs_term_vars(ts[0], x);
        occurs_terms_vars(ts.drop_first(), x);
    }
}

proof fn subst_term_vars(t: Term, x: nat, value: Term)
    ensures
        term_vars(ckc_spec::engine::subst(t, x, value)).subset_of(
            term_vars(t).remove(x).union(term_vars(value)),
        ),
    decreases t, 0int,
{
    broadcast use vstd::set::group_set_lemmas;

    reveal(ckc_spec::engine::subst);
    reveal(term_vars);
    match t {
        Term::Comp(_, args) => subst_terms_vars(args, x, value),
        _ => {},
    }
}

proof fn subst_terms_vars(ts: Seq<Term>, x: nat, value: Term)
    ensures
        terms_vars(ckc_spec::engine::subst_all(ts, x, value)).subset_of(
            terms_vars(ts).remove(x).union(term_vars(value)),
        ),
    decreases ts, 1int,
{
    broadcast use vstd::set::group_set_lemmas;

    reveal_with_fuel(ckc_spec::engine::subst_all, 2);
    reveal_with_fuel(terms_vars, 2);
    if ts.len() > 0 {
        let head = term_vars(ts[0]);
        let tail = terms_vars(ts.drop_first());
        let next_head = term_vars(ckc_spec::engine::subst(ts[0], x, value));
        let next_tail = terms_vars(ckc_spec::engine::subst_all(ts.drop_first(), x, value));
        let vars = term_vars(value);
        subst_term_vars(ts[0], x, value);
        subst_terms_vars(ts.drop_first(), x, value);
        assert forall|y: nat| next_head.union(next_tail).contains(y) implies head.union(
            tail,
        ).remove(x).union(vars).contains(y) by {
            if next_head.contains(y) {
                assert(head.remove(x).union(vars).contains(y));
                if !vars.contains(y) {
                    assert(head.remove(x).contains(y));
                    assert(head.contains(y));
                    assert(y != x);
                    assert(head.union(tail).contains(y));
                    assert(head.union(tail).remove(x).contains(y));
                }
            } else {
                assert(next_tail.contains(y));
                assert(tail.remove(x).union(vars).contains(y));
                if !vars.contains(y) {
                    assert(tail.remove(x).contains(y));
                    assert(tail.contains(y));
                    assert(y != x);
                    assert(head.union(tail).contains(y));
                    assert(head.union(tail).remove(x).contains(y));
                }
            }
        }
        assert(ckc_spec::engine::subst_all(ts, x, value)[0] == ckc_spec::engine::subst(
            ts[0],
            x,
            value,
        ));
        assert_seqs_equal!(
            ckc_spec::engine::subst_all(ts, x, value).drop_first()
                == ckc_spec::engine::subst_all(ts.drop_first(), x, value)
        );
        assert(terms_vars(ckc_spec::engine::subst_all(ts, x, value)) == next_head.union(next_tail));
        assert(terms_vars(ts) == head.union(tail));
        assert(terms_vars(ckc_spec::engine::subst_all(ts, x, value)).subset_of(
            terms_vars(ts).remove(x).union(term_vars(value)),
        )) by {
            reveal(Set::subset_of);
        }
    }
}

pub open spec fn subst_pairs_rec(pairs: Seq<(Term, Term)>, x: nat, value: Term) -> Seq<(Term, Term)>
    decreases pairs.len(),
{
    if pairs.len() == 0 {
        Seq::empty()
    } else {
        seq![
            (
                ckc_spec::engine::subst(pairs[0].0, x, value),
                ckc_spec::engine::subst(pairs[0].1, x, value),
            ),
        ] + subst_pairs_rec(pairs.drop_first(), x, value)
    }
}

proof fn subst_pairs_rec_model(pairs: Seq<(Term, Term)>, x: nat, value: Term)
    ensures
        subst_pairs_rec(pairs, x, value) == subst_pairs_model(pairs, x, value),
    decreases pairs.len(),
{
    reveal_with_fuel(subst_pairs_rec, 2);
    reveal(subst_pairs_model);
    if pairs.len() > 0 {
        subst_pairs_rec_model(pairs.drop_first(), x, value);
        assert(subst_pairs_model(pairs, x, value)[0] == (
            ckc_spec::engine::subst(pairs[0].0, x, value),
            ckc_spec::engine::subst(pairs[0].1, x, value),
        ));
        assert_seqs_equal!(
            subst_pairs_model(pairs, x, value).drop_first()
                == subst_pairs_model(pairs.drop_first(), x, value)
        );
    }
}

proof fn subst_pair_vars(pairs: Seq<(Term, Term)>, x: nat, value: Term)
    ensures
        pair_vars(subst_pairs_rec(pairs, x, value)).subset_of(
            pair_vars(pairs).remove(x).union(term_vars(value)),
        ),
    decreases pairs.len(),
{
    broadcast use vstd::set::group_set_lemmas;

    reveal_with_fuel(subst_pairs_rec, 2);
    reveal_with_fuel(pair_vars, 2);
    if pairs.len() > 0 {
        let left = term_vars(pairs[0].0);
        let right = term_vars(pairs[0].1);
        let tail = pair_vars(pairs.drop_first());
        let next_left = term_vars(ckc_spec::engine::subst(pairs[0].0, x, value));
        let next_right = term_vars(ckc_spec::engine::subst(pairs[0].1, x, value));
        let next_tail = pair_vars(subst_pairs_rec(pairs.drop_first(), x, value));
        let vars = term_vars(value);
        subst_term_vars(pairs[0].0, x, value);
        subst_term_vars(pairs[0].1, x, value);
        subst_pair_vars(pairs.drop_first(), x, value);
        assert forall|y: nat|
            next_left.union(next_right.union(next_tail)).contains(y) implies left.union(
            right.union(tail),
        ).remove(x).union(vars).contains(y) by {
            if next_left.contains(y) {
                assert(left.remove(x).union(vars).contains(y));
                if !vars.contains(y) {
                    assert(left.remove(x).contains(y));
                    assert(left.contains(y));
                    assert(y != x);
                    assert(left.union(right.union(tail)).contains(y));
                    assert(left.union(right.union(tail)).remove(x).contains(y));
                }
            } else if next_right.contains(y) {
                assert(right.remove(x).union(vars).contains(y));
                if !vars.contains(y) {
                    assert(right.remove(x).contains(y));
                    assert(right.contains(y));
                    assert(y != x);
                    assert(left.union(right.union(tail)).contains(y));
                    assert(left.union(right.union(tail)).remove(x).contains(y));
                }
            } else {
                assert(next_tail.contains(y));
                assert(tail.remove(x).union(vars).contains(y));
                if !vars.contains(y) {
                    assert(tail.remove(x).contains(y));
                    assert(tail.contains(y));
                    assert(y != x);
                    assert(left.union(right.union(tail)).contains(y));
                    assert(left.union(right.union(tail)).remove(x).contains(y));
                }
            }
        }
        assert(subst_pairs_rec(pairs, x, value)[0] == (
            ckc_spec::engine::subst(pairs[0].0, x, value),
            ckc_spec::engine::subst(pairs[0].1, x, value),
        ));
        assert_seqs_equal!(
            subst_pairs_rec(pairs, x, value).drop_first()
                == subst_pairs_rec(pairs.drop_first(), x, value)
        );
        assert(pair_vars(subst_pairs_rec(pairs, x, value)) == next_left.union(
            next_right.union(next_tail),
        ));
        assert(pair_vars(pairs) == left.union(right.union(tail)));
        assert(pair_vars(subst_pairs_rec(pairs, x, value)).subset_of(
            pair_vars(pairs).remove(x).union(term_vars(value)),
        )) by {
            reveal(Set::subset_of);
        }
    }
}

proof fn bind_left_vars_decrease(x: nat, value: Term, rest: Seq<(Term, Term)>)
    requires
        !ckc_spec::engine::occurs(x, value),
    ensures
        pair_vars(subst_pairs_rec(rest, x, value)).len() < pair_vars(
            seq![(Term::Var(x), value)] + rest,
        ).len(),
{
    broadcast use vstd::set::group_set_lemmas;

    let before = pair_vars(seq![(Term::Var(x), value)] + rest);
    let after = pair_vars(subst_pairs_rec(rest, x, value));
    subst_pair_vars(rest, x, value);
    occurs_term_vars(value, x);
    reveal_with_fuel(pair_vars, 2);
    reveal(term_vars);
    assert_seqs_equal!((seq![(Term::Var(x), value)] + rest).drop_first() == rest);
    assert(before == term_vars(Term::Var(x)).union(term_vars(value).union(pair_vars(rest))));
    assert(after.subset_of(pair_vars(rest).remove(x).union(term_vars(value))));
    assert(after.subset_of(before)) by {
        assert forall|y: nat| after.contains(y) implies before.contains(y) by {
            assert(pair_vars(rest).remove(x).union(term_vars(value)).contains(y));
            if term_vars(value).contains(y) {
                assert(before.contains(y));
            } else {
                assert(pair_vars(rest).remove(x).contains(y));
                assert(pair_vars(rest).contains(y));
                assert(before.contains(y));
            }
        }
        reveal(Set::subset_of);
    }
    assert(!after.contains(x)) by {
        if after.contains(x) {
            assert(pair_vars(rest).remove(x).union(term_vars(value)).contains(x));
            assert(!pair_vars(rest).remove(x).contains(x));
            assert(!term_vars(value).contains(x));
        }
    }
    assert(before.contains(x));
    after.lemma_subset_not_in_lt(before, x);
}

proof fn bind_right_vars_decrease(x: nat, value: Term, rest: Seq<(Term, Term)>)
    requires
        !ckc_spec::engine::occurs(x, value),
    ensures
        pair_vars(subst_pairs_rec(rest, x, value)).len() < pair_vars(
            seq![(value, Term::Var(x))] + rest,
        ).len(),
{
    broadcast use vstd::set::group_set_lemmas;

    let before = pair_vars(seq![(value, Term::Var(x))] + rest);
    let after = pair_vars(subst_pairs_rec(rest, x, value));
    subst_pair_vars(rest, x, value);
    occurs_term_vars(value, x);
    reveal_with_fuel(pair_vars, 2);
    reveal(term_vars);
    assert_seqs_equal!((seq![(value, Term::Var(x))] + rest).drop_first() == rest);
    assert(before == term_vars(value).union(term_vars(Term::Var(x)).union(pair_vars(rest))));
    assert(after.subset_of(pair_vars(rest).remove(x).union(term_vars(value))));
    assert(after.subset_of(before)) by {
        assert forall|y: nat| after.contains(y) implies before.contains(y) by {
            assert(pair_vars(rest).remove(x).union(term_vars(value)).contains(y));
            if term_vars(value).contains(y) {
                assert(before.contains(y));
            } else {
                assert(pair_vars(rest).remove(x).contains(y));
                assert(pair_vars(rest).contains(y));
                assert(before.contains(y));
            }
        }
        reveal(Set::subset_of);
    }
    assert(!after.contains(x)) by {
        if after.contains(x) {
            assert(pair_vars(rest).remove(x).union(term_vars(value)).contains(x));
            assert(!pair_vars(rest).remove(x).contains(x));
            assert(!term_vars(value).contains(x));
        }
    }
    assert(before.contains(x));
    after.lemma_subset_not_in_lt(before, x);
}

proof fn term_size_positive(t: Term)
    ensures
        term_size(t) > 0,
    decreases t,
{
    reveal(term_size);
}

proof fn pair_work_concat(left: Seq<(Term, Term)>, right: Seq<(Term, Term)>)
    ensures
        pair_work(left + right) == pair_work(left) + pair_work(right),
    decreases left.len(),
{
    reveal_with_fuel(pair_work, 2);
    if left.len() > 0 {
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
        pair_work_concat(left.drop_first(), right);
    }
}

proof fn zip_work(xs: Seq<Term>, ys: Seq<Term>)
    requires
        xs.len() == ys.len(),
    ensures
        pair_work(ckc_spec::engine::zip(xs, ys)) == terms_size(xs) + terms_size(ys),
    decreases xs.len(),
{
    reveal(ckc_spec::engine::zip);
    reveal_with_fuel(pair_work, 2);
    reveal_with_fuel(terms_size, 2);
    if xs.len() > 0 {
        assert(ys.len() > 0);
        assert(ckc_spec::engine::zip(xs, ys)[0] == (xs[0], ys[0]));
        assert_seqs_equal!(
            ckc_spec::engine::zip(xs, ys).drop_first()
                == ckc_spec::engine::zip(xs.drop_first(), ys.drop_first())
        );
        zip_work(xs.drop_first(), ys.drop_first());
    }
}

proof fn comp_pair_work_decrease(
    left_name: Seq<u8>,
    left_args: Seq<Term>,
    right_name: Seq<u8>,
    right_args: Seq<Term>,
    rest: Seq<(Term, Term)>,
)
    requires
        left_args.len() == right_args.len(),
    ensures
        pair_work(ckc_spec::engine::zip(left_args, right_args) + rest) < pair_work(
            seq![(Term::Comp(left_name, left_args), Term::Comp(right_name, right_args))] + rest,
        ),
{
    let zipped = ckc_spec::engine::zip(left_args, right_args);
    let before = seq![(Term::Comp(left_name, left_args), Term::Comp(right_name, right_args))]
        + rest;
    pair_work_concat(zipped, rest);
    zip_work(left_args, right_args);
    reveal_with_fuel(pair_work, 2);
    reveal(term_size);
    assert(pair_work(zipped + rest) == terms_size(left_args) + terms_size(right_args) + pair_work(
        rest,
    ));
    assert(before[0] == (Term::Comp(left_name, left_args), Term::Comp(right_name, right_args)));
    assert_seqs_equal!(before.drop_first() == rest);
    assert(pair_work(before) == 2 + terms_size(left_args) + terms_size(right_args) + pair_work(
        rest,
    ));
}

proof fn pairs_models_prefix(before: Seq<ENode>, after: Seq<ENode>, pairs: Seq<EPair>)
    requires
        before.is_prefix_of(after),
        pair_roots_valid(before, pairs),
    ensures
        pair_roots_valid(after, pairs),
        pairs_view(after, pairs) == pairs_view(before, pairs),
{
    reveal(pair_roots_valid);
    reveal(pairs_view);
    assert forall|i: int| 0 <= i < pairs.len() implies {
        &&& pairs[i].left < after.len()
        &&& pairs[i].right < after.len()
    } by {
        assert(pairs[i].left < before.len());
        assert(pairs[i].right < before.len());
    }
    assert_seqs_equal!(pairs_view(after, pairs) == pairs_view(before, pairs));
}

proof fn pair_roots_valid_drop(nodes: Seq<ENode>, pairs: Seq<EPair>)
    requires
        pairs.len() > 0,
        pair_roots_valid(nodes, pairs),
    ensures
        pair_roots_valid(nodes, pairs.drop_first()),
{
    reveal(pair_roots_valid);
}

proof fn pairs_view_drop(nodes: Seq<ENode>, pairs: Seq<EPair>)
    requires
        pairs.len() > 0,
    ensures
        pairs_view(nodes, pairs.drop_first()) == pairs_view(nodes, pairs).drop_first(),
{
    reveal(pairs_view);
    assert_seqs_equal!(pairs_view(nodes, pairs.drop_first())
        == pairs_view(nodes, pairs).drop_first());
}

proof fn pairs_view_concat(nodes: Seq<ENode>, left: Seq<EPair>, right: Seq<EPair>)
    requires
        pair_roots_valid(nodes, left),
        pair_roots_valid(nodes, right),
    ensures
        pair_roots_valid(nodes, left + right),
        pairs_view(nodes, left + right) == pairs_view(nodes, left) + pairs_view(nodes, right),
{
    reveal(pair_roots_valid);
    reveal(pairs_view);
    assert_seqs_equal!(pairs_view(nodes, left + right)
        == pairs_view(nodes, left) + pairs_view(nodes, right));
}

proof fn pair_vars_concat(left: Seq<(Term, Term)>, right: Seq<(Term, Term)>)
    ensures
        pair_vars(left + right) =~= pair_vars(left).union(pair_vars(right)),
    decreases left.len(),
{
    broadcast use vstd::set::group_set_lemmas;

    reveal_with_fuel(pair_vars, 2);
    if left.len() > 0 {
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
        pair_vars_concat(left.drop_first(), right);
        assert(pair_vars(left.drop_first() + right) =~= pair_vars(left.drop_first()).union(
            pair_vars(right),
        ));
    }
}

proof fn zip_vars(xs: Seq<Term>, ys: Seq<Term>)
    requires
        xs.len() == ys.len(),
    ensures
        pair_vars(ckc_spec::engine::zip(xs, ys)) =~= terms_vars(xs).union(terms_vars(ys)),
    decreases xs.len(),
{
    broadcast use vstd::set::group_set_lemmas;

    reveal(ckc_spec::engine::zip);
    reveal_with_fuel(pair_vars, 2);
    reveal_with_fuel(terms_vars, 2);
    if xs.len() > 0 {
        assert(ys.len() > 0);
        assert(ckc_spec::engine::zip(xs, ys)[0] == (xs[0], ys[0]));
        assert_seqs_equal!(
            ckc_spec::engine::zip(xs, ys).drop_first()
                == ckc_spec::engine::zip(xs.drop_first(), ys.drop_first())
        );
        zip_vars(xs.drop_first(), ys.drop_first());
    }
}

proof fn pair_drop_measure(first: (Term, Term), rest: Seq<(Term, Term)>)
    ensures
        pair_vars(rest).len() <= pair_vars(seq![first] + rest).len(),
        pair_work(rest) < pair_work(seq![first] + rest),
{
    broadcast use vstd::set::group_set_lemmas;

    let before = pair_vars(seq![first] + rest);
    assert_seqs_equal!((seq![first] + rest).drop_first() == rest);
    reveal_with_fuel(pair_vars, 2);
    assert(pair_vars(rest).subset_of(before)) by {
        reveal(Set::subset_of);
    }
    vstd::set_lib::lemma_len_subset(pair_vars(rest), before);
    term_size_positive(first.0);
    term_size_positive(first.1);
    reveal_with_fuel(pair_work, 2);
}

proof fn comp_pair_vars_equal(
    left_name: Seq<u8>,
    left_args: Seq<Term>,
    right_name: Seq<u8>,
    right_args: Seq<Term>,
    rest: Seq<(Term, Term)>,
)
    requires
        left_args.len() == right_args.len(),
    ensures
        pair_vars(ckc_spec::engine::zip(left_args, right_args) + rest).len() == pair_vars(
            seq![(Term::Comp(left_name, left_args), Term::Comp(right_name, right_args))] + rest,
        ).len(),
{
    broadcast use vstd::set::group_set_lemmas;

    let zipped = ckc_spec::engine::zip(left_args, right_args);
    let before = seq![(Term::Comp(left_name, left_args), Term::Comp(right_name, right_args))]
        + rest;
    pair_vars_concat(zipped, rest);
    zip_vars(left_args, right_args);
    assert_seqs_equal!(before.drop_first() == rest);
    reveal_with_fuel(pair_vars, 2);
    reveal(term_vars);
    assert(pair_vars(zipped + rest) =~= terms_vars(left_args).union(terms_vars(right_args)).union(
        pair_vars(rest),
    ));
    assert(pair_vars(before) =~= terms_vars(left_args).union(terms_vars(right_args)).union(
        pair_vars(rest),
    ));
    assert(pair_vars(zipped + rest) == pair_vars(before));
}

#[verifier::rlimit(5000)]
fn zip_pairs(arena: &ETermArena, left: &Vec<usize>, right: &Vec<usize>) -> (out: Vec<EPair>)
    requires
        arena_ok(arena),
        left@.len() == right@.len(),
        roots_valid(arena.nodes@, left@),
        roots_valid(arena.nodes@, right@),
    ensures
        pair_roots_valid(arena.nodes@, out@),
        pairs_view(arena.nodes@, out@) == ckc_spec::engine::zip(
            root_terms(arena.nodes@, left@),
            root_terms(arena.nodes@, right@),
        ),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < left.len()
        invariant
            arena_ok(arena),
            left@.len() == right@.len(),
            roots_valid(arena.nodes@, left@),
            roots_valid(arena.nodes@, right@),
            i <= left@.len(),
            out@.len() == i,
            pair_roots_valid(arena.nodes@, out@),
            forall|j: int|
                0 <= j < out@.len() ==> {
                    &&& out@[j].left == left@[j]
                    &&& out@[j].right == right@[j]
                },
        decreases left.len() - i,
    {
        let l = left[i];
        let r = right[i];
        let ghost prior = out@;
        out.push(EPair { left: l, right: r });
        proof {
            reveal(roots_valid);
            reveal(pair_roots_valid);
            assert forall|j: int| 0 <= j < out@.len() implies {
                &&& out@[j].left < arena.nodes@.len()
                &&& out@[j].right < arena.nodes@.len()
            } by {
                if j < prior.len() {
                    assert(out@[j] == prior[j]);
                } else {
                    assert(j == prior.len());
                    assert(j == i);
                    assert(out@[j].left == l);
                    assert(out@[j].right == r);
                }
            }
            assert forall|j: int| 0 <= j < out@.len() implies {
                &&& out@[j].left == left@[j]
                &&& out@[j].right == right@[j]
            } by {
                if j < prior.len() {
                    assert(out@[j] == prior[j]);
                } else {
                    assert(j == prior.len());
                    assert(j == i);
                }
            }
        }
        i += 1;
    }
    proof {
        reveal(pairs_view);
        reveal(root_terms);
        reveal(ckc_spec::engine::zip);
        assert_seqs_equal!(pairs_view(arena.nodes@, out@)
            == ckc_spec::engine::zip(
                root_terms(arena.nodes@, left@),
                root_terms(arena.nodes@, right@),
            ));
    }
    out
}

pub fn vec_equal(left: &Vec<u8>, right: &Vec<u8>) -> (equal: bool)
    ensures
        equal == (left@ == right@),
{
    if left.len() != right.len() {
        return false;
    }
    let mut i = 0usize;
    while i < left.len()
        invariant
            left@.len() == right@.len(),
            i <= left@.len(),
            forall|j: int| 0 <= j < i ==> left@[j] == right@[j],
        decreases left.len() - i,
    {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(left@ == right@);
    }
    true
}

pub struct EBoundState {
    pub pairs: Vec<EPair>,
    pub stack: Vec<EGoal>,
    pub sol: Vec<usize>,
}

#[verifier::rlimit(5000)]
fn bind_state(
    arena: &mut ETermArena,
    pairs: &Vec<EPair>,
    stack: &Vec<EGoal>,
    sol: &Vec<usize>,
    x: usize,
    replacement: usize,
) -> (out: EBoundState)
    requires
        arena_ok(old(arena)),
        pair_roots_valid(old(arena).nodes@, pairs@),
        goals_valid(old(arena).nodes@, stack@),
        roots_valid(old(arena).nodes@, sol@),
        replacement < old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        pair_roots_valid(final(arena).nodes@, out.pairs@),
        goals_valid(final(arena).nodes@, out.stack@),
        roots_valid(final(arena).nodes@, out.sol@),
        pairs_view(final(arena).nodes@, out.pairs@) == subst_pairs_model(
            pairs_view(old(arena).nodes@, pairs@),
            x as nat,
            old(arena)@[replacement as int],
        ),
        goals_view(final(arena).nodes@, out.stack@) == goals_view(
            old(arena).nodes@,
            stack@,
        ).map_values(
            |g: ckc_spec::engine::Goal|
                ckc_spec::engine::subst_goal(g, x as nat, old(arena)@[replacement as int]),
        ),
        root_terms(final(arena).nodes@, out.sol@) == ckc_spec::engine::subst_all(
            root_terms(old(arena).nodes@, sol@),
            x as nat,
            old(arena)@[replacement as int],
        ),
{
    let ghost base = arena.nodes@;
    let ghost value = arena@[replacement as int];
    let pair_out = subst_pairs(arena, pairs, x, replacement);
    let ghost after_pairs = arena.nodes@;
    proof {
        goals_models_prefix(base, after_pairs, stack@);
        roots_models_prefix(base, after_pairs, sol@);
        arena_prefix_stable(base, arena);
        assert(value == arena@[replacement as int]);
    }
    let stack_out = subst_goals(arena, stack, x, replacement);
    let ghost after_stack = arena.nodes@;
    proof {
        roots_models_prefix(base, after_stack, sol@);
        arena_prefix_stable(base, arena);
        assert(value == arena@[replacement as int]);
    }
    let sol_out = subst_roots(arena, sol, x, replacement);
    proof {
        pairs_models_prefix(after_pairs, arena.nodes@, pair_out@);
        goals_models_prefix(after_stack, arena.nodes@, stack_out@);
        arena_prefix_stable(base, arena);
        assert(value == arena@[replacement as int]);
    }
    EBoundState { pairs: pair_out, stack: stack_out, sol: sol_out }
}

fn rigid_equal(arena: &ETermArena, left: usize, right: usize) -> (equal: bool)
    requires
        root_ok(arena, left),
        root_ok(arena, right),
        !(arena@[left as int] is Var),
        !(arena@[right as int] is Var),
        !((arena@[left as int] is Comp) && (arena@[right as int] is Comp)),
    ensures
        equal == (arena@[left as int] == arena@[right as int]),
{
    proof {
        reveal(root_ok);
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, left as int));
        assert(node_ok(arena.nodes@, right as int));
        reveal(node_ok);
    }
    match (&arena.nodes[left].kind, &arena.nodes[right].kind) {
        (
            ENodeKind::Int {
                magnitude: left_magnitude,
                negative: left_negative,
                value: left_value,
                ..
            },
            ENodeKind::Int {
                magnitude: right_magnitude,
                negative: right_negative,
                value: right_value,
                ..
            },
        ) => {
            let order = int_order(
                left_magnitude,
                *left_negative,
                *left_value,
                right_magnitude,
                *right_negative,
                *right_value,
            );
            match order {
                EOrder::Equal => {
                    proof {
                        reveal(int_order_ok);
                    }
                    true
                },
                EOrder::Less => {
                    proof {
                        reveal(int_order_ok);
                    }
                    false
                },
                EOrder::Greater => {
                    proof {
                        reveal(int_order_ok);
                    }
                    false
                },
            }
        },
        (ENodeKind::Nil, ENodeKind::Nil) => true,
        (ENodeKind::Atom { name: left_name }, ENodeKind::Atom { name: right_name }) => {
            vec_equal(left_name, right_name)
        },
        _ => false,
    }
}

pub open spec fn bound_state_view(nodes: Seq<ENode>, u: &EBoundState) -> ckc_spec::engine::UState {
    ustate_view(nodes, u.pairs@, u.stack@, u.sol@)
}

pub open spec fn bound_state_valid(nodes: Seq<ENode>, u: &EBoundState) -> bool {
    &&& pair_roots_valid(nodes, u.pairs@)
    &&& goals_valid(nodes, u.stack@)
    &&& roots_valid(nodes, u.sol@)
}

pub open spec fn uresult_valid(nodes: Seq<ENode>, result: &EUResult) -> bool {
    match result {
        EUResult::Ok { stack, sol } => { goals_valid(nodes, stack@) && roots_valid(nodes, sol@) },
        EUResult::Fail => true,
    }
}

pub open spec fn pairs_decrease(after: Seq<(Term, Term)>, before: Seq<(Term, Term)>) -> bool {
    pair_vars(after).len() < pair_vars(before).len() || (pair_vars(after).len() == pair_vars(
        before,
    ).len() && pair_work(after) < pair_work(before))
}

proof fn unify_drop_step(u: ckc_spec::engine::UState)
    requires
        u.pairs.len() > 0,
        u.pairs[0].0 == u.pairs[0].1,
        !(u.pairs[0].0 is Comp),
    ensures
        ckc_spec::engine::unify(u) == ckc_spec::engine::unify(
            ckc_spec::engine::UState { pairs: u.pairs.drop_first(), ..u },
        ),
        pairs_decrease(u.pairs.drop_first(), u.pairs),
{
    let after = ckc_spec::engine::UState { pairs: u.pairs.drop_first(), ..u };
    assert forall|fuel: nat| fuel > 0 implies #[trigger] ckc_spec::engine::unify_n(u, fuel)
        == ckc_spec::engine::unify_n(after, (fuel - 1) as nat) by {
        reveal_with_fuel(ckc_spec::engine::unify_n, 2);
    }
    unify_step_preserves(u, after);
    assert_seqs_equal!(u.pairs == seq![u.pairs[0]] + u.pairs.drop_first());
    pair_drop_measure(u.pairs[0], u.pairs.drop_first());
}

proof fn unify_bind_step(u: ckc_spec::engine::UState, x: nat, value: Term)
    requires
        u.pairs.len() > 0,
        u.pairs[0] == (Term::Var(x), value) || (u.pairs[0] == (value, Term::Var(x)) && !(
        value is Var)),
        !ckc_spec::engine::occurs(x, value),
    ensures
        ckc_spec::engine::unify(u) == ckc_spec::engine::unify(
            ckc_spec::engine::u_bind(u, u.pairs.drop_first(), x, value),
        ),
        pairs_decrease(ckc_spec::engine::u_bind(u, u.pairs.drop_first(), x, value).pairs, u.pairs),
{
    let rest = u.pairs.drop_first();
    let after = ckc_spec::engine::u_bind(u, rest, x, value);
    assert(value != Term::Var(x)) by {
        reveal(ckc_spec::engine::occurs);
    }
    assert forall|fuel: nat| fuel > 0 implies #[trigger] ckc_spec::engine::unify_n(u, fuel)
        == ckc_spec::engine::unify_n(after, (fuel - 1) as nat) by {
        reveal_with_fuel(ckc_spec::engine::unify_n, 2);
    }
    unify_step_preserves(u, after);
    assert_seqs_equal!(u.pairs == seq![u.pairs[0]] + rest);
    subst_pairs_rec_model(rest, x, value);
    if u.pairs[0].0 == Term::Var(x) {
        bind_left_vars_decrease(x, value, rest);
    } else {
        bind_right_vars_decrease(x, value, rest);
    }
}

proof fn unify_comp_step(
    u: ckc_spec::engine::UState,
    name: Seq<u8>,
    left: Seq<Term>,
    right: Seq<Term>,
)
    requires
        u.pairs.len() > 0,
        u.pairs[0] == (Term::Comp(name, left), Term::Comp(name, right)),
        left.len() == right.len(),
    ensures
        ckc_spec::engine::unify(u) == ckc_spec::engine::unify(
            ckc_spec::engine::UState {
                pairs: ckc_spec::engine::zip(left, right) + u.pairs.drop_first(),
                ..u
            },
        ),
        pairs_decrease(ckc_spec::engine::zip(left, right) + u.pairs.drop_first(), u.pairs),
{
    let rest = u.pairs.drop_first();
    let after = ckc_spec::engine::UState { pairs: ckc_spec::engine::zip(left, right) + rest, ..u };
    assert forall|fuel: nat| fuel > 0 implies #[trigger] ckc_spec::engine::unify_n(u, fuel)
        == ckc_spec::engine::unify_n(after, (fuel - 1) as nat) by {
        reveal_with_fuel(ckc_spec::engine::unify_n, 2);
    }
    unify_step_preserves(u, after);
    assert_seqs_equal!(u.pairs == seq![u.pairs[0]] + rest);
    comp_pair_work_decrease(name, left, name, right, rest);
    comp_pair_vars_equal(name, left, name, right, rest);
}

pub open spec fn rigid_clash(pair: (Term, Term)) -> bool
    decreases pair.0,
{
    match pair {
        (Term::Var(_), _) | (_, Term::Var(_)) => false,
        (Term::Comp(n, xs), Term::Comp(m, ys)) => n != m || xs.len() != ys.len() || args_clash(
            xs,
            ys,
        ),
        (a, b) => a != b,
    }
}

pub open spec fn args_clash(xs: Seq<Term>, ys: Seq<Term>) -> bool
    decreases xs,
{
    xs.len() > 0 && ys.len() > 0 && (rigid_clash((xs[0], ys[0])) || args_clash(
        xs.drop_first(),
        ys.drop_first(),
    ))
}

proof fn args_clash_model(xs: Seq<Term>, ys: Seq<Term>)
    requires
        xs.len() == ys.len(),
    ensures
        args_clash(xs, ys) == (exists|i: int| 0 <= i < xs.len() && rigid_clash((xs[i], ys[i]))),
    decreases xs.len(),
{
    reveal_with_fuel(args_clash, 1);
    if xs.len() > 0 {
        args_clash_model(xs.drop_first(), ys.drop_first());
        if args_clash(xs, ys) {
            if rigid_clash((xs[0], ys[0])) {
                assert(exists|i: int| 0 <= i < xs.len() && rigid_clash((xs[i], ys[i])));
            } else {
                let i = choose|i: int|
                    0 <= i < xs.drop_first().len() && rigid_clash(
                        (xs.drop_first()[i], ys.drop_first()[i]),
                    );
                assert(rigid_clash((xs[i + 1], ys[i + 1])));
                assert(exists|j: int| 0 <= j < xs.len() && rigid_clash((xs[j], ys[j])));
            }
        }
        if exists|i: int| 0 <= i < xs.len() && rigid_clash((xs[i], ys[i])) {
            let i = choose|i: int| 0 <= i < xs.len() && rigid_clash((xs[i], ys[i]));
            if i > 0 {
                assert(rigid_clash((xs.drop_first()[i - 1], ys.drop_first()[i - 1])));
                assert(exists|j: int|
                    0 <= j < xs.drop_first().len() && rigid_clash(
                        (xs.drop_first()[j], ys.drop_first()[j]),
                    ));
                assert(args_clash(xs.drop_first(), ys.drop_first()));
            }
        }
    }
}

proof fn clash_intro(name: Seq<u8>, xs: Seq<Term>, ys: Seq<Term>, i: int)
    requires
        xs.len() == ys.len(),
        0 <= i < xs.len(),
        rigid_clash((xs[i], ys[i])),
    ensures
        rigid_clash((Term::Comp(name, xs), Term::Comp(name, ys))),
{
    args_clash_model(xs, ys);
    assert(exists|j: int| 0 <= j < xs.len() && rigid_clash((xs[j], ys[j])));
    assert(args_clash(xs, ys));
    reveal_with_fuel(rigid_clash, 1);
}

proof fn clash_under_subst(a: Term, b: Term, x: nat, value: Term)
    requires
        rigid_clash((a, b)),
    ensures
        rigid_clash((ckc_spec::engine::subst(a, x, value), ckc_spec::engine::subst(b, x, value))),
    decreases a,
{
    reveal_with_fuel(rigid_clash, 1);
    reveal(ckc_spec::engine::subst);
    match (a, b) {
        (Term::Comp(n, xs), Term::Comp(m, ys)) => {
            subst_all_map(xs, x, value);
            subst_all_map(ys, x, value);
            if n == m && xs.len() == ys.len() {
                args_clash_model(xs, ys);
                let i = choose|i: int|
                    0 <= i < xs.len() && i < ys.len() && rigid_clash((xs[i], ys[i]));
                clash_under_subst(xs[i], ys[i], x, value);
                let left = ckc_spec::engine::subst_all(xs, x, value);
                let right = ckc_spec::engine::subst_all(ys, x, value);
                assert(rigid_clash((left[i], right[i])));
                clash_intro(n, left, right, i);
            }
        },
        _ => {},
    }
}

proof fn clash_under_shift(a: Term, b: Term, off: nat)
    requires
        rigid_clash((a, b)),
    ensures
        rigid_clash((a, ckc_spec::engine::shift(b, off))),
    decreases b,
{
    reveal_with_fuel(rigid_clash, 1);
    reveal(ckc_spec::engine::shift);
    match (a, b) {
        (Term::Comp(n, xs), Term::Comp(m, ys)) => {
            shift_all_map(ys, off);
            if n == m && xs.len() == ys.len() {
                args_clash_model(xs, ys);
                let i = choose|i: int|
                    0 <= i < xs.len() && i < ys.len() && rigid_clash((xs[i], ys[i]));
                clash_under_shift(xs[i], ys[i], off);
                let right = ckc_spec::engine::shift_all(ys, off);
                assert(rigid_clash((xs[i], right[i])));
                clash_intro(n, xs, right, i);
            }
        },
        _ => {},
    }
}

pub open spec fn pair_clash(pairs: Seq<(Term, Term)>) -> bool {
    exists|i: int| 0 <= i < pairs.len() && #[trigger] rigid_clash(pairs[i])
}

proof fn clash_tail(pairs: Seq<(Term, Term)>)
    requires
        pairs.len() > 0,
        pair_clash(pairs),
        !rigid_clash(pairs[0]),
    ensures
        pair_clash(pairs.drop_first()),
{
    let i = choose|i: int| 0 <= i < pairs.len() && rigid_clash(pairs[i]);
    assert(i > 0);
    assert(rigid_clash(pairs.drop_first()[i - 1]));
}

proof fn clash_subst(pairs: Seq<(Term, Term)>, x: nat, value: Term)
    requires
        pair_clash(pairs),
    ensures
        pair_clash(
            pairs.map_values(
                |p: (Term, Term)|
                    (
                        ckc_spec::engine::subst(p.0, x, value),
                        ckc_spec::engine::subst(p.1, x, value),
                    ),
            ),
        ),
{
    let i = choose|i: int| 0 <= i < pairs.len() && rigid_clash(pairs[i]);
    clash_under_subst(pairs[i].0, pairs[i].1, x, value);
    let changed = pairs.map_values(
        |p: (Term, Term)|
            (ckc_spec::engine::subst(p.0, x, value), ckc_spec::engine::subst(p.1, x, value)),
    );
    assert(rigid_clash(changed[i]));
}

proof fn clash_comp(name: Seq<u8>, xs: Seq<Term>, ys: Seq<Term>, rest: Seq<(Term, Term)>)
    requires
        xs.len() == ys.len(),
    ensures
        pair_clash(seq![(Term::Comp(name, xs), Term::Comp(name, ys))] + rest) == pair_clash(
            ckc_spec::engine::zip(xs, ys) + rest,
        ),
{
    let before = seq![(Term::Comp(name, xs), Term::Comp(name, ys))] + rest;
    let after = ckc_spec::engine::zip(xs, ys) + rest;
    if pair_clash(before) {
        let i = choose|i: int| 0 <= i < before.len() && rigid_clash(before[i]);
        if i > 0 {
            assert(after[xs.len() as int + i - 1] == before[i]);
            assert(rigid_clash(after[xs.len() as int + i - 1]));
        } else {
            args_clash_model(xs, ys);
            let j = choose|j: int| 0 <= j < xs.len() && j < ys.len() && rigid_clash((xs[j], ys[j]));
            assert(rigid_clash(after[j]));
        }
        assert(pair_clash(after));
    }
    if pair_clash(after) {
        let j = choose|j: int| 0 <= j < after.len() && rigid_clash(after[j]);
        if j < xs.len() {
            assert(rigid_clash((xs[j], ys[j])));
            clash_intro(name, xs, ys, j);
            assert(rigid_clash(before[0]));
        } else {
            assert(before[j - xs.len() as int + 1] == after[j]);
            assert(rigid_clash(before[j - xs.len() as int + 1]));
        }
        assert(pair_clash(before));
    }
}

proof fn clash_fails(u: ckc_spec::engine::UState)
    requires
        pair_clash(u.pairs),
    ensures
        ckc_spec::engine::unify(u) is Fail,
    decreases pair_vars(u.pairs).len(), pair_work(u.pairs),
{
    assert(u.pairs.len() > 0);
    let a = u.pairs[0].0;
    let b = u.pairs[0].1;
    let rest = u.pairs.drop_first();
    assert_seqs_equal!(u.pairs == seq![(a, b)] + rest);
    match (a, b) {
        (Term::Var(x), _) => {
            if a == b {
                clash_tail(u.pairs);
                unify_drop_step(u);
                clash_fails(ckc_spec::engine::UState { pairs: rest, ..u });
            } else if ckc_spec::engine::occurs(x, b) {
                reveal_with_fuel(ckc_spec::engine::unify_n, 1);
                unify_completed(u, 1);
            } else {
                clash_tail(u.pairs);
                clash_subst(rest, x, b);
                unify_bind_step(u, x, b);
                clash_fails(ckc_spec::engine::u_bind(u, rest, x, b));
            }
        },
        (_, Term::Var(y)) => {
            if ckc_spec::engine::occurs(y, a) {
                reveal_with_fuel(ckc_spec::engine::unify_n, 1);
                unify_completed(u, 1);
            } else {
                clash_tail(u.pairs);
                clash_subst(rest, y, a);
                unify_bind_step(u, y, a);
                clash_fails(ckc_spec::engine::u_bind(u, rest, y, a));
            }
        },
        (Term::Comp(n, xs), Term::Comp(m, ys)) => {
            if n == m && xs.len() == ys.len() {
                clash_comp(n, xs, ys, rest);
                unify_comp_step(u, n, xs, ys);
                clash_fails(
                    ckc_spec::engine::UState { pairs: ckc_spec::engine::zip(xs, ys) + rest, ..u },
                );
            } else {
                reveal_with_fuel(ckc_spec::engine::unify_n, 1);
                unify_completed(u, 1);
            }
        },
        _ => {
            if a == b {
                clash_tail(u.pairs);
                unify_drop_step(u);
                clash_fails(ckc_spec::engine::UState { pairs: rest, ..u });
            } else {
                reveal_with_fuel(ckc_spec::engine::unify_n, 1);
                unify_completed(u, 1);
            }
        },
    }
}

pub open spec fn same_comp(a: Term, b: Term) -> bool {
    match (a, b) {
        (Term::Comp(n, xs), Term::Comp(m, ys)) => n == m && xs.len() == ys.len(),
        _ => false,
    }
}

fn quick_clash(arena: &ETermArena, left: usize, right: usize) -> (out: Option<bool>)
    requires
        root_ok(arena, left),
        root_ok(arena, right),
    ensures
        out == Some(true) ==> rigid_clash((arena@[left as int], arena@[right as int])),
        out.is_none() ==> same_comp(arena@[left as int], arena@[right as int]),
{
    proof {
        assert(node_ok(arena.nodes@, left as int));
        assert(node_ok(arena.nodes@, right as int));
        reveal(node_ok);
        reveal_with_fuel(rigid_clash, 1);
    }
    match (&arena.nodes[left].kind, &arena.nodes[right].kind) {
        (ENodeKind::Var { .. }, _) | (_, ENodeKind::Var { .. }) => Some(false),
        (
            ENodeKind::Comp { name: n, child_roots: xs, .. },
            ENodeKind::Comp { name: m, child_roots: ys, .. },
        ) => {
            proof {
                node_comp_model(arena.nodes@, left as int, n@, xs@);
                node_comp_model(arena.nodes@, right as int, m@, ys@);
            }
            if xs.len() != ys.len() || !vec_equal(n, m) {
                Some(true)
            } else {
                None
            }
        },
        _ => Some(!rigid_equal(arena, left, right)),
    }
}

proof fn clash_prepend(first: (Term, Term), rest: Seq<(Term, Term)>)
    ensures
        pair_clash(rest) ==> pair_clash(seq![first] + rest),
{
    if pair_clash(rest) {
        let i = choose|i: int| 0 <= i < rest.len() && rigid_clash(rest[i]);
        assert(rigid_clash((seq![first] + rest)[i + 1]));
    }
}

fn clash_roots(arena: &ETermArena, left: usize, right: usize) -> (out: bool)
    requires
        root_ok(arena, left),
        root_ok(arena, right),
    ensures
        out ==> rigid_clash((arena@[left as int], arena@[right as int])),
{
    if let Some(found) = quick_clash(arena, left, right) {
        return found;
    }
    let ghost target = (arena@[left as int], arena@[right as int]);
    let mut pairs = Vec::new();
    pairs.push(EPair { left, right });
    proof {
        assert_seqs_equal!(pairs_view(arena.nodes@, pairs@) == seq![target]);
    }
    while pairs.len() > 0
        invariant
            root_ok(arena, left),
            root_ok(arena, right),
            target == (arena@[left as int], arena@[right as int]),
            pair_roots_valid(arena.nodes@, pairs@),
            pair_clash(pairs_view(arena.nodes@, pairs@)) ==> rigid_clash(target),
        decreases pair_work(pairs_view(arena.nodes@, pairs@)),
    {
        let ghost previous = pairs@;
        let ghost model = pairs_view(arena.nodes@, previous);
        let pair = pairs.remove(0);
        let a = pair.left;
        let b = pair.right;
        proof {
            pairs_view_drop(arena.nodes@, previous);
            pair_roots_valid_drop(arena.nodes@, previous);
            assert(model[0] == (arena@[a as int], arena@[b as int]));
            assert_seqs_equal!(model == seq![model[0]] + pairs_view(arena.nodes@, pairs@));
            pair_drop_measure(model[0], pairs_view(arena.nodes@, pairs@));
        }
        match quick_clash(arena, a, b) {
            Some(found) => {
                if found {
                    proof {
                        assert(rigid_clash(model[0]));
                        assert(pair_clash(model));
                    }
                    return true;
                }
                proof {
                    clash_prepend(model[0], pairs_view(arena.nodes@, pairs@));
                }
            },
            None => {
                let xs = args_roots(arena, a);
                let ys = args_roots(arena, b);
                let mut children = zip_pairs(arena, &xs, &ys);
                proof {
                    pairs_view_concat(arena.nodes@, children@, pairs@);
                    match (arena@[a as int], arena@[b as int]) {
                        (Term::Comp(n, xterms), Term::Comp(m, yterms)) => {
                            assert(n == m && xterms.len() == yterms.len());
                            clash_comp(n, xterms, yterms, pairs_view(arena.nodes@, pairs@));
                            comp_pair_work_decrease(
                                n,
                                xterms,
                                m,
                                yterms,
                                pairs_view(arena.nodes@, pairs@),
                            );
                        },
                        _ => {
                            assert(false);
                        },
                    }
                }
                let ghost head = children@;
                let ghost tail = pairs@;
                children.append(&mut pairs);
                proof {
                    assert forall|i: int| 0 <= i < children.len() implies {
                        &&& children@[i].left < arena.nodes.len()
                        &&& children@[i].right < arena.nodes.len()
                    } by {
                        if i < head.len() {
                            assert(children@[i] == head[i]);
                        } else {
                            assert(children@[i] == tail[i - head.len()]);
                        }
                    }
                }
                pairs = children;
            },
        }
    }
    false
}

// A rigid argument mismatch rejects a trial before renaming or binding its continuation.
fn head_clash(arena: &ETermArena, args: &Vec<usize>, head: usize, Ghost(off): Ghost<nat>) -> (out:
    bool)
    requires
        root_ok(arena, head),
        roots_valid(arena.nodes@, args@),
        arena@[head as int] is Comp,
        args.len() == ckc_spec::engine::args_of(arena@[head as int]).len(),
    ensures
        out ==> pair_clash(
            ckc_spec::engine::zip(
                root_terms(arena.nodes@, args@),
                ckc_spec::engine::args_of(ckc_spec::engine::shift(arena@[head as int], off)),
            ),
        ),
{
    let hs = args_roots(arena, head);
    let ghost shifted = ckc_spec::engine::args_of(
        ckc_spec::engine::shift(arena@[head as int], off),
    );
    proof {
        match arena@[head as int] {
            Term::Comp(_, terms) => shift_all_map(terms, off),
            _ => {},
        }
    }
    let mut i = args.len();
    while i > 0
        invariant
            root_ok(arena, head),
            roots_valid(arena.nodes@, args@),
            roots_valid(arena.nodes@, hs@),
            i <= args.len(),
            args.len() == hs.len(),
            arena@[head as int] is Comp,
            root_terms(arena.nodes@, hs@) == ckc_spec::engine::args_of(arena@[head as int]),
            shifted == ckc_spec::engine::args_of(ckc_spec::engine::shift(arena@[head as int], off)),
            shifted.len() == hs.len(),
            forall|j: int|
                0 <= j < hs.len() ==> shifted[j] == ckc_spec::engine::shift(
                    arena@[hs@[j] as int],
                    off,
                ),
        decreases i,
    {
        i -= 1;
        let a = args[i];
        let b = hs[i];
        if clash_roots(arena, a, b) {
            proof {
                clash_under_shift(arena@[a as int], arena@[b as int], off);
                let pairs = ckc_spec::engine::zip(root_terms(arena.nodes@, args@), shifted);
                assert(rigid_clash(pairs[i as int]));
            }
            return true;
        }
    }
    false
}

pub enum EUStep {
    Next(EBoundState),
    Done(EUResult),
}

pub open spec fn ustep_valid(nodes: Seq<ENode>, step: &EUStep) -> bool {
    match step {
        EUStep::Next(u) => bound_state_valid(nodes, u),
        EUStep::Done(result) => uresult_valid(nodes, result),
    }
}

pub open spec fn ustep_refines(
    nodes: Seq<ENode>,
    step: &EUStep,
    before: ckc_spec::engine::UState,
) -> bool {
    match step {
        EUStep::Next(u) => {
            &&& ckc_spec::engine::unify(before) == ckc_spec::engine::unify(
                bound_state_view(nodes, u),
            )
            &&& pairs_decrease(bound_state_view(nodes, u).pairs, before.pairs)
        },
        EUStep::Done(result) => { uresult_view(nodes, result) == ckc_spec::engine::unify(before) },
    }
}

#[verifier::rlimit(5000)]
fn unify_variable(
    arena: &mut ETermArena,
    pairs: Vec<EPair>,
    stack: Vec<EGoal>,
    sol: Vec<usize>,
    x: usize,
    replacement: usize,
    Ghost(before): Ghost<ckc_spec::engine::UState>,
) -> (step: EUStep)
    requires
        arena_ok(old(arena)),
        pair_roots_valid(old(arena).nodes@, pairs@),
        goals_valid(old(arena).nodes@, stack@),
        roots_valid(old(arena).nodes@, sol@),
        replacement < old(arena).nodes@.len(),
        before.pairs.len() > 0,
        before.pairs[0].0 != before.pairs[0].1,
        before.pairs[0] == (Term::Var(x as nat), old(arena)@[replacement as int]) || (
        before.pairs[0] == (old(arena)@[replacement as int], Term::Var(x as nat)) && !(old(
            arena,
        )@[replacement as int] is Var)),
        pairs_view(old(arena).nodes@, pairs@) == before.pairs.drop_first(),
        goals_view(old(arena).nodes@, stack@) == before.stack,
        root_terms(old(arena).nodes@, sol@) == before.sol,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        ustep_valid(final(arena).nodes@, &step),
        ustep_refines(final(arena).nodes@, &step, before),
{
    let ghost value = arena@[replacement as int];
    if occurs_root(arena, x, replacement) {
        proof {
            reveal_with_fuel(ckc_spec::engine::unify_n, 2);
            assert(ckc_spec::engine::unify_n(before, 1) is Fail);
            unify_completed(before, 1);
        }
        EUStep::Done(EUResult::Fail)
    } else {
        proof {
            unify_bind_step(before, x as nat, value);
        }
        let next = bind_state(arena, &pairs, &stack, &sol, x, replacement);
        proof {
            assert(bound_state_view(arena.nodes@, &next) == ckc_spec::engine::u_bind(
                before,
                before.pairs.drop_first(),
                x as nat,
                value,
            ));
        }
        EUStep::Next(next)
    }
}

#[verifier::rlimit(5000)]
fn unify_compound(
    arena: &ETermArena,
    mut pairs: Vec<EPair>,
    stack: Vec<EGoal>,
    sol: Vec<usize>,
    left: usize,
    right: usize,
    Ghost(before): Ghost<ckc_spec::engine::UState>,
) -> (step: EUStep)
    requires
        root_ok(arena, left),
        root_ok(arena, right),
        pair_roots_valid(arena.nodes@, pairs@),
        goals_valid(arena.nodes@, stack@),
        roots_valid(arena.nodes@, sol@),
        arena@[left as int] is Comp,
        arena@[right as int] is Comp,
        before.pairs.len() > 0,
        before.pairs[0] == (arena@[left as int], arena@[right as int]),
        pairs_view(arena.nodes@, pairs@) == before.pairs.drop_first(),
        goals_view(arena.nodes@, stack@) == before.stack,
        root_terms(arena.nodes@, sol@) == before.sol,
    ensures
        ustep_valid(arena.nodes@, &step),
        ustep_refines(arena.nodes@, &step, before),
{
    proof {
        reveal(root_ok);
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, left as int));
        assert(node_ok(arena.nodes@, right as int));
        reveal(node_ok);
    }
    match (&arena.nodes[left].kind, &arena.nodes[right].kind) {
        (
            ENodeKind::Comp { name: ln, child_roots: ls, .. },
            ENodeKind::Comp { name: rn, child_roots: rs, .. },
        ) => {
            proof {
                node_comp_model(arena.nodes@, left as int, ln@, ls@);
                node_comp_model(arena.nodes@, right as int, rn@, rs@);
            }
            if ls.len() != rs.len() || !vec_equal(ln, rn) {
                proof {
                    reveal_with_fuel(ckc_spec::engine::unify_n, 2);
                    assert(ckc_spec::engine::unify_n(before, 1) is Fail);
                    unify_completed(before, 1);
                }
                EUStep::Done(EUResult::Fail)
            } else {
                let mut next_pairs = zip_pairs(arena, ls, rs);
                proof {
                    unify_comp_step(
                        before,
                        ln@,
                        root_terms(arena.nodes@, ls@),
                        root_terms(arena.nodes@, rs@),
                    );
                    pairs_view_concat(arena.nodes@, next_pairs@, pairs@);
                }
                next_pairs.append(&mut pairs);
                EUStep::Next(EBoundState { pairs: next_pairs, stack, sol })
            }
        },
        _ => {
            proof {
                assert(false);
            }
            EUStep::Done(EUResult::Fail)
        },
    }
}

#[verifier::rlimit(5000)]
fn unify_step(arena: &mut ETermArena, u: EBoundState) -> (step: EUStep)
    requires
        arena_ok(old(arena)),
        bound_state_valid(old(arena).nodes@, &u),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        ustep_valid(final(arena).nodes@, &step),
        ustep_refines(final(arena).nodes@, &step, bound_state_view(old(arena).nodes@, &u)),
{
    let ghost before = bound_state_view(arena.nodes@, &u);
    let EBoundState { mut pairs, stack, sol } = u;
    if pairs.len() == 0 {
        proof {
            reveal_with_fuel(ckc_spec::engine::unify_n, 2);
            assert(ckc_spec::engine::unify_n(before, 1) == ckc_spec::engine::UOut::Ok(
                before.stack,
                before.sol,
            ));
            unify_completed(before, 1);
        }
        return EUStep::Done(EUResult::Ok { stack, sol });
    }
    let ghost initial_pairs = pairs@;
    let EPair { left, right } = pairs.remove(0);
    proof {
        pairs_view_drop(arena.nodes@, initial_pairs);
        pair_roots_valid_drop(arena.nodes@, initial_pairs);
        assert(pairs@ == initial_pairs.drop_first());
        assert(before.pairs[0] == (arena@[left as int], arena@[right as int]));
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, left as int));
        assert(node_ok(arena.nodes@, right as int));
        reveal(node_ok);
    }
    match (&arena.nodes[left].kind, &arena.nodes[right].kind) {
        (ENodeKind::Var { key: x, .. }, ENodeKind::Var { key: y, .. }) if x == y => {
            proof {
                node_var_model(arena.nodes@, left as int, *x);
                node_var_model(arena.nodes@, right as int, *y);
                unify_drop_step(before);
            }
            EUStep::Next(EBoundState { pairs, stack, sol })
        },
        (ENodeKind::Var { key: x, .. }, _) => {
            let key = *x;
            proof {
                node_var_model(arena.nodes@, left as int, key);
            }
            unify_variable(arena, pairs, stack, sol, key, right, Ghost(before))
        },
        (_, ENodeKind::Var { key: y, .. }) => {
            let key = *y;
            proof {
                node_var_model(arena.nodes@, right as int, key);
            }
            unify_variable(arena, pairs, stack, sol, key, left, Ghost(before))
        },
        (ENodeKind::Comp { .. }, ENodeKind::Comp { .. }) => {
            unify_compound(arena, pairs, stack, sol, left, right, Ghost(before))
        },
        _ => {
            if rigid_equal(arena, left, right) {
                proof {
                    unify_drop_step(before);
                }
                EUStep::Next(EBoundState { pairs, stack, sol })
            } else {
                proof {
                    reveal_with_fuel(ckc_spec::engine::unify_n, 2);
                    assert(ckc_spec::engine::unify_n(before, 1) is Fail);
                    unify_completed(before, 1);
                }
                EUStep::Done(EUResult::Fail)
            }
        },
    }
}

#[verifier::rlimit(5000)]
pub fn unify(arena: &mut ETermArena, initial_state: EBoundState) -> (result: EUResult)
    requires
        arena_ok(old(arena)),
        bound_state_valid(old(arena).nodes@, &initial_state),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        uresult_valid(final(arena).nodes@, &result),
        uresult_view(final(arena).nodes@, &result) == ckc_spec::engine::unify(
            bound_state_view(old(arena).nodes@, &initial_state),
        ),
{
    let ghost base = arena.nodes@;
    let ghost initial = bound_state_view(base, &initial_state);
    let mut u = initial_state;
    loop
        invariant
            base == old(arena).nodes@,
            initial == bound_state_view(base, &initial_state),
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            bound_state_valid(arena.nodes@, &u),
            ckc_spec::engine::unify(bound_state_view(arena.nodes@, &u)) == ckc_spec::engine::unify(
                initial,
            ),
        decreases
                pair_vars(bound_state_view(arena.nodes@, &u).pairs).len(),
                pair_work(bound_state_view(arena.nodes@, &u).pairs),
    {
        let step = unify_step(arena, u);
        proof {
            assert(base.is_prefix_of(arena.nodes@));
        }
        match step {
            EUStep::Next(next) => {
                u = next;
            },
            EUStep::Done(done) => {
                return done;
            },
        }
    }
}

pub struct EAlt {
    pub mark: usize,
    pub stack: Vec<EGoal>,
    pub sol: Vec<usize>,
    pub fresh: usize,
    pub ci: usize,
}

pub struct ECfg {
    pub base: usize,
    pub mark: usize,
    pub saved: ETermArena,
    pub stack: Vec<EGoal>,
    pub sol: Vec<usize>,
    pub alts: Vec<EAlt>,
    pub fresh: usize,
    pub ci: usize,
    pub pruned: bool,
    pub rows: Vec<Vec<usize>>,
    pub collect: bool,
}

pub enum EStep {
    Next(ECfg),
    Sol,
    Done(ECfg),
}

pub enum EROut {
    Sol,
    End { complete: bool, rows: Vec<Vec<usize>> },
}

pub open spec fn alt_view(nodes: Seq<ENode>, alt: &EAlt) -> ckc_spec::engine::Alt {
    ckc_spec::engine::Alt {
        stack: goals_view(nodes, alt.stack@),
        sol: root_terms(nodes, alt.sol@),
        fresh: alt.fresh as nat,
        ci: alt.ci as nat,
    }
}

pub open spec fn alts_view(nodes: Seq<ENode>, alts: Seq<EAlt>) -> Seq<ckc_spec::engine::Alt> {
    Seq::new(alts.len(), |i: int| alt_view(nodes, &alts[i]))
}

pub open spec fn rows_view(nodes: Seq<ENode>, rows: Seq<Vec<usize>>) -> Seq<Seq<Term>> {
    Seq::new(rows.len(), |i: int| root_terms(nodes, rows[i]@))
}

pub open spec fn cfg_view(nodes: Seq<ENode>, c: &ECfg) -> ckc_spec::engine::Cfg {
    ckc_spec::engine::Cfg {
        stack: goals_view(nodes, c.stack@),
        sol: root_terms(nodes, c.sol@),
        alts: alts_view(nodes, c.alts@),
        fresh: c.fresh as nat,
        ci: c.ci as nat,
        pruned: c.pruned,
        rows: rows_view(c.saved.nodes@, c.rows@),
        collect: c.collect,
    }
}

pub open spec fn step_view(nodes: Seq<ENode>, step: &EStep) -> ckc_spec::engine::Step {
    match step {
        EStep::Next(c) => ckc_spec::engine::Step::Next(cfg_view(nodes, c)),
        EStep::Sol => ckc_spec::engine::Step::Sol,
        EStep::Done(c) => ckc_spec::engine::Step::Done(cfg_view(nodes, c)),
    }
}

pub open spec fn rout_view(nodes: Seq<ENode>, result: &EROut) -> ckc_spec::engine::ROut {
    match result {
        EROut::Sol => ckc_spec::engine::ROut::Sol,
        EROut::End { complete, rows } => ckc_spec::engine::ROut::End {
            complete: *complete,
            rows: rows_view(nodes, rows@),
        },
    }
}

pub open spec fn goal_level_valid(goal: &EGoal, level: nat) -> bool {
    match goal {
        EGoal::NafCut { level: cut } => *cut <= level,
        _ => true,
    }
}

pub open spec fn goals_levels(goals: Seq<EGoal>, level: nat) -> bool {
    forall|i: int| 0 <= i < goals.len() ==> #[trigger] goal_level_valid(&goals[i], level)
}

pub open spec fn alt_valid(nodes: Seq<ENode>, alt: &EAlt, level: nat) -> bool {
    &&& goals_valid(nodes, alt.stack@)
    &&& roots_valid(nodes, alt.sol@)
    &&& alt.mark <= nodes.len()
    &&& alt.fresh <= alt.mark
    &&& goals_valid(nodes.take(alt.mark as int), alt.stack@)
    &&& goals_levels(alt.stack@, level)
    &&& roots_valid(nodes.take(alt.mark as int), alt.sol@)
}

pub open spec fn rows_valid(nodes: Seq<ENode>, rows: Seq<Vec<usize>>) -> bool {
    forall|i: int| 0 <= i < rows.len() ==> #[trigger] roots_valid(nodes, rows[i]@)
}

pub open spec fn cfg_storage_valid(nodes: Seq<ENode>, c: &ECfg) -> bool {
    &&& c.base <= c.mark <= nodes.len()
    &&& arena_ok(&c.saved)
    &&& rows_valid(c.saved.nodes@, c.rows@)
    &&& forall|i: int| 0 <= i < c.alts.len() ==> #[trigger] alt_valid(nodes, &c.alts@[i], i as nat)
    &&& forall|i: int|
        #![trigger c.alts@[i].mark]
        0 <= i < c.alts.len() ==> c.base <= c.alts@[i].mark <= c.mark
    &&& forall|i: int, j: int| 0 <= i < j < c.alts.len() ==> c.alts@[i].mark <= c.alts@[j].mark
}

pub open spec fn cfg_valid(nodes: Seq<ENode>, c: &ECfg) -> bool {
    &&& goals_valid(nodes, c.stack@)
    &&& roots_valid(nodes, c.sol@)
    &&& c.fresh <= nodes.len()
    &&& forall|i: int| 0 <= i < c.alts.len() ==> #[trigger] alt_valid(nodes, &c.alts@[i], i as nat)
    &&& cfg_storage_valid(nodes, c)
    &&& c.fresh <= c.mark
    &&& goals_valid(nodes.take(c.mark as int), c.stack@)
    &&& goals_levels(c.stack@, c.alts@.len())
    &&& roots_valid(nodes.take(c.mark as int), c.sol@)
}

pub open spec fn step_base(step: &EStep, base: usize) -> bool {
    match step {
        EStep::Next(c) | EStep::Done(c) => c.base == base,
        EStep::Sol => true,
    }
}

pub open spec fn step_valid(nodes: Seq<ENode>, step: &EStep) -> bool {
    match step {
        EStep::Next(c) => cfg_valid(nodes, c),
        EStep::Done(c) => arena_ok(&c.saved) && rows_valid(c.saved.nodes@, c.rows@),
        EStep::Sol => true,
    }
}

pub open spec fn rout_valid(nodes: Seq<ENode>, result: &EROut) -> bool {
    match result {
        EROut::Sol => true,
        EROut::End { rows, .. } => rows_valid(nodes, rows@),
    }
}

pub proof fn cfg_models_prefix(before: Seq<ENode>, after: Seq<ENode>, c: &ECfg)
    requires
        before.is_prefix_of(after),
        cfg_valid(before, c),
    ensures
        cfg_valid(after, c),
        cfg_view(before, c) == cfg_view(after, c),
{
    reveal(cfg_valid);
    reveal(cfg_storage_valid);
    reveal(alt_valid);
    assert(before.take(c.mark as int) == after.take(c.mark as int));
    goals_models_prefix(before, after, c.stack@);
    roots_models_prefix(before, after, c.sol@);
    assert forall|i: int| 0 <= i < c.alts@.len() implies {
        &&& alt_valid(after, &c.alts@[i], i as nat)
        &&& alt_view(before, &c.alts@[i]) == alt_view(after, &c.alts@[i])
    } by {
        assert(alt_valid(before, &c.alts@[i], i as nat));
        assert(before.take(c.alts@[i].mark as int) == after.take(c.alts@[i].mark as int));
        goals_models_prefix(before, after, c.alts@[i].stack@);
        roots_models_prefix(before, after, c.alts@[i].sol@);
    }
    assert forall|i: int| 0 <= i < c.alts.len() implies {
        &&& alt_valid(after, &c.alts@[i], i as nat)
        &&& c.base <= c.alts@[i].mark <= c.mark
    } by {
        assert(alt_valid(after, &c.alts@[i], i as nat));
    }
    assert(cfg_storage_valid(after, c));
    assert(goals_valid(after.take(c.mark as int), c.stack@));
    assert(roots_valid(after.take(c.mark as int), c.sol@));
    assert_seqs_equal!(alts_view(before, c.alts@) == alts_view(after, c.alts@));
}

proof fn goals_resize(before: Seq<ENode>, after: Seq<ENode>, goals: Seq<EGoal>)
    requires
        goals_valid(before, goals),
        before.len() <= after.len(),
    ensures
        goals_valid(after, goals),
{
    assert forall|i: int| 0 <= i < goals.len() implies goal_valid(after, &goals[i]) by {
        assert(goal_valid(before, &goals[i]));
        reveal(goal_valid);
    }
}

proof fn cfg_compact(before: Seq<ENode>, after: Seq<ENode>, c: &ECfg)
    requires
        cfg_valid(before, c),
        after == before.take(c.mark as int),
    ensures
        cfg_valid(after, c),
        cfg_view(after, c) == cfg_view(before, c),
{
    reveal(cfg_valid);
    reveal(cfg_storage_valid);
    reveal(alt_valid);
    assert_seqs_equal!(after.take(c.mark as int) == after);
    goals_resize(before.take(c.mark as int), after, c.stack@);
    assert forall|i: int| 0 <= i < c.alts.len() implies alt_valid(after, &c.alts@[i], i as nat) by {
        assert(alt_valid(before, &c.alts@[i], i as nat));
        assert(before.take(c.alts@[i].mark as int) == after.take(c.alts@[i].mark as int));
        goals_resize(before.take(c.alts@[i].mark as int), after, c.alts@[i].stack@);
    }
    assert(cfg_storage_valid(after, c));
    assert(goals_valid(after.take(c.mark as int), c.stack@));
    assert(roots_valid(after.take(c.mark as int), c.sol@));
    assert(cfg_valid(after, c));
    cfg_models_prefix(after, before, c);
}

pub fn fail(arena: &ETermArena, mut c: ECfg) -> (step: EStep)
    requires
        arena_ok(arena),
        cfg_storage_valid(arena.nodes@, &c),
    ensures
        step_valid(arena.nodes@, &step),
        step_base(&step, c.base),
        step_view(arena.nodes@, &step) == ckc_spec::engine::fail(cfg_view(arena.nodes@, &c)),
{
    reveal(cfg_valid);
    reveal(cfg_storage_valid);
    reveal(alt_valid);
    if c.alts.len() == 0 {
        return EStep::Done(c);
    }
    let ghost prior = c.alts@;
    proof {
        assert(alt_valid(arena.nodes@, &prior[prior.len() - 1], (prior.len() - 1) as nat));
    }
    let alt = c.alts.pop().unwrap();
    proof {
        assert(alt == prior[prior.len() - 1]);
        assert(alt_valid(arena.nodes@, &alt, c.alts@.len()));
        assert(alts_view(arena.nodes@, prior).last() == alt_view(arena.nodes@, &alt));
        assert_seqs_equal!(alts_view(arena.nodes@, c.alts@)
            == alts_view(arena.nodes@, prior).drop_last());
    }
    c.mark = alt.mark;
    c.stack = alt.stack;
    c.sol = alt.sol;
    c.fresh = alt.fresh;
    c.ci = alt.ci;
    proof {
        assert forall|i: int| 0 <= i < c.alts.len() implies {
            &&& alt_valid(arena.nodes@, &c.alts@[i], i as nat)
            &&& c.base <= c.alts@[i].mark <= c.mark
        } by {
            assert(c.alts@[i] == prior[i]);
            assert(prior[i].mark <= prior.last().mark);
        }
        assert(cfg_valid(arena.nodes@, &c));
    }
    EStep::Next(c)
}

pub open spec fn max_var_count(key: Option<usize>) -> nat {
    match key {
        Some(k) => k as nat + 1,
        None => 0,
    }
}

fn max_var_merge(left: Option<usize>, right: Option<usize>) -> (out: Option<usize>)
    ensures
        max_var_count(out) == ckc_spec::engine::max_nat(max_var_count(left), max_var_count(right)),
{
    match (left, right) {
        (Some(a), Some(b)) => Some(
            if a >= b {
                a
            } else {
                b
            },
        ),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

pub open spec fn roots_nvars(nodes: Seq<ENode>, roots: Seq<usize>) -> nat
    decreases roots.len(),
{
    if roots.len() == 0 {
        0
    } else {
        ckc_spec::engine::max_nat(
            ckc_spec::engine::nvars(nodes[roots[0] as int].term@),
            roots_nvars(nodes, roots.drop_first()),
        )
    }
}

proof fn roots_nvars_concat(nodes: Seq<ENode>, left: Seq<usize>, right: Seq<usize>)
    ensures
        roots_nvars(nodes, left + right) == ckc_spec::engine::max_nat(
            roots_nvars(nodes, left),
            roots_nvars(nodes, right),
        ),
    decreases left.len(),
{
    reveal_with_fuel(roots_nvars, 2);
    if left.len() > 0 {
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
        roots_nvars_concat(nodes, left.drop_first(), right);
    }
}

proof fn nvars_all_root_terms(nodes: Seq<ENode>, roots: Seq<usize>)
    ensures
        roots_nvars(nodes, roots) == ckc_spec::engine::nvars_all(root_terms(nodes, roots)),
    decreases roots.len(),
{
    reveal_with_fuel(roots_nvars, 2);
    reveal_with_fuel(ckc_spec::engine::nvars_all, 2);
    if roots.len() > 0 {
        assert(root_terms(nodes, roots).drop_first() == root_terms(nodes, roots.drop_first()));
        nvars_all_root_terms(nodes, roots.drop_first());
    }
}

#[verifier::rlimit(5000)]
pub fn max_var_root(arena: &ETermArena, root: usize) -> (out: Option<usize>)
    requires
        root_ok(arena, root),
    ensures
        max_var_count(out) == ckc_spec::engine::nvars(arena@[root as int]),
{
    proof {
        reveal(root_ok);
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, root as int));
        crate::k2_term::node_cache_elim(arena.nodes@, root as int);
    }
    arena.nodes[root].maximum
}

// Sparse labels consume arena capacity before their exact natural count enters a machine integer.
pub fn ensure_var_capacity(arena: &mut ETermArena, maximum: Option<usize>) -> (count: usize)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        count == max_var_count(maximum),
        count <= final(arena).nodes@.len(),
{
    match maximum {
        None => 0,
        Some(key) => {
            let ghost base = arena.nodes@;
            while arena.nodes.len() <= key
                invariant
                    base == old(arena).nodes@,
                    arena_ok(arena),
                    base.is_prefix_of(arena.nodes@),
                decreases key as int + 1 - arena.nodes@.len(),
            {
                let next = arena.nodes.len();
                let spelling = var_spelling(next);
                let unused = push_var(arena, next, spelling);
                proof {
                    assert(base.is_prefix_of(arena.nodes@));
                }
            }
            key + 1
        },
    }
}

pub fn nvars_root(arena: &mut ETermArena, root: usize) -> (count: usize)
    requires
        root_ok(old(arena), root),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        count == ckc_spec::engine::nvars(old(arena)@[root as int]),
        count <= final(arena).nodes@.len(),
{
    let maximum = max_var_root(arena, root);
    ensure_var_capacity(arena, maximum)
}

pub open spec fn node_var_count(node: &ENode) -> nat {
    match &node.kind {
        ENodeKind::Var { key, .. } => *key as nat + 1,
        _ => 0,
    }
}

fn max_var_prefix(arena: &ETermArena, root: usize) -> (out: Option<usize>)
    requires
        root_ok(arena, root),
    ensures
        forall|i: int|
            0 <= i <= root ==> #[trigger] node_var_count(&arena.nodes@[i]) <= max_var_count(out),
{
    let node_count = arena.nodes.len();
    let mut out = None;
    let mut i = 0usize;
    while i <= root
        invariant
            root_ok(arena, root),
            node_count == arena.nodes@.len(),
            root < node_count,
            i <= root as nat + 1,
            forall|j: int|
                0 <= j < i ==> #[trigger] node_var_count(&arena.nodes@[j]) <= max_var_count(out),
        decreases root as int + 1 - i,
    {
        match &arena.nodes[i].kind {
            ENodeKind::Var { key, .. } => {
                out = max_var_merge(out, Some(*key));
            },
            _ => {},
        }
        i += 1;
    }
    out
}

#[verifier::rlimit(5000)]
fn reserve_shift_vars(arena: &mut ETermArena, off: usize, maximum: Option<usize>) -> (out: Vec<
    usize,
>)
    requires
        arena_ok(old(arena)),
        off <= old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@.len() == max_var_count(maximum),
        off as nat + out@.len() <= final(arena).nodes@.len(),
        roots_valid(final(arena).nodes@, out@),
        forall|i: int|
            0 <= i < out@.len() ==> final(arena)@[out@[i] as int] == Term::Var(
                off as nat + i as nat,
            ),
{
    let ghost base = arena.nodes@;
    let mut out: Vec<usize> = Vec::new();
    match maximum {
        None => {},
        Some(last) => {
            while out.len() <= last
                invariant
                    base == old(arena).nodes@,
                    arena_ok(arena),
                    base.is_prefix_of(arena.nodes@),
                    off <= base.len(),
                    arena.nodes@.len() == base.len() + out@.len(),
                    out@.len() <= last as nat + 1,
                    roots_valid(arena.nodes@, out@),
                    forall|i: int|
                        0 <= i < out@.len() ==> arena@[out@[i] as int] == Term::Var(
                            off as nat + i as nat,
                        ),
                decreases last as int + 1 - out@.len(),
            {
                let node_count = arena.nodes.len();
                proof {
                    assert(off as nat + out@.len() <= node_count);
                }
                let key = off + out.len();
                let spelling = var_spelling(key);
                let ghost before = arena.nodes@;
                let root = push_var(arena, key, spelling);
                proof {
                    roots_models_prefix(before, arena.nodes@, out@);
                    assert(base.is_prefix_of(arena.nodes@));
                    roots_valid_push(arena.nodes@, out@, root);
                }
                out.push(root);
            }
        },
    }
    out
}

proof fn shift_all_map(ts: Seq<Term>, off: nat)
    ensures
        ckc_spec::engine::shift_all(ts, off) == ts.map_values(
            |t: Term| ckc_spec::engine::shift(t, off),
        ),
    decreases ts.len(),
{
    reveal_with_fuel(ckc_spec::engine::shift_all, 2);
    if ts.len() > 0 {
        shift_all_map(ts.drop_first(), off);
        assert_seqs_equal!(ts.map_values(|t: Term| ckc_spec::engine::shift(t, off))
            == seq![ckc_spec::engine::shift(ts[0], off)]
                + ts.drop_first().map_values(|t: Term| ckc_spec::engine::shift(t, off)));
    }
}

proof fn nvars_all_bound(ts: Seq<Term>, bound: nat)
    requires
        forall|i: int| 0 <= i < ts.len() ==> #[trigger] ckc_spec::engine::nvars(ts[i]) <= bound,
    ensures
        ckc_spec::engine::nvars_all(ts) <= bound,
    decreases ts.len(),
{
    reveal_with_fuel(ckc_spec::engine::nvars_all, 2);
    if ts.len() > 0 {
        assert(ckc_spec::engine::nvars(ts[0]) <= bound);
        nvars_all_bound(ts.drop_first(), bound);
        reveal(ckc_spec::engine::max_nat);
    }
}

fn map_child_roots(map: &Vec<usize>, roots: &Vec<usize>) -> (out: Vec<usize>)
    requires
        forall|i: int| 0 <= i < roots@.len() ==> roots@[i] < map@.len(),
    ensures
        out@ == map_values_at(map@, roots@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < roots.len()
        invariant
            forall|j: int| 0 <= j < roots@.len() ==> roots@[j] < map@.len(),
            i <= roots@.len(),
            out@.len() == i,
            forall|j: int| 0 <= j < out@.len() ==> out@[j] == map@[roots@[j] as int],
        decreases roots.len() - i,
    {
        out.push(map[roots[i]]);
        i += 1;
    }
    proof {
        assert_seqs_equal!(out@ == map_values_at(map@, roots@));
    }
    out
}

proof fn mapped_children(
    base: Seq<ENode>,
    current: Seq<ENode>,
    map: Seq<usize>,
    roots: Seq<usize>,
    f: spec_fn(Term) -> Term,
)
    requires
        roots_valid(base, roots),
        forall|i: int| 0 <= i < roots.len() ==> roots[i] < map.len(),
        forall|i: int|
            0 <= i < map.len() ==> {
                &&& map[i] < current.len()
                &&& current[map[i] as int].term@ == f(base[i].term@)
            },
    ensures
        roots_valid(current, map_values_at(map, roots)),
        root_terms(current, map_values_at(map, roots)) == root_terms(base, roots).map_values(f),
{
    assert_seqs_equal!(root_terms(current, map_values_at(map, roots))
        == root_terms(base, roots).map_values(f));
}

#[verifier::rlimit(5000)]
pub fn shift_root(arena: &mut ETermArena, root: usize, off: usize) -> (out: usize)
    requires
        root_ok(old(arena), root),
        off <= old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out < final(arena).nodes@.len(),
        final(arena)@[out as int] == ckc_spec::engine::shift(old(arena)@[root as int], off as nat),
        off as nat + ckc_spec::engine::nvars(old(arena)@[root as int]) <= final(arena).nodes@.len(),
{
    let ghost base = arena.nodes@;
    let maximum = max_var_root(arena, root);
    if maximum.is_none() {
        proof {
            shift_closed(arena@[root as int], off as nat);
        }
        return root;
    }
    let vars = reserve_shift_vars(arena, off, maximum);
    proof {
        arena_prefix_stable(base, arena);
    }
    let mut owned = crate::k2_reject::empty_arena();
    std::mem::swap(arena, &mut owned);
    let (mut owned, result) = crate::k2_rewrite::rewrite_root(owned, root, true, off, root, &vars);
    std::mem::swap(arena, &mut owned);
    proof {
        assert(base.is_prefix_of(arena.nodes@));
    }
    result
}

pub fn unifiable_apart(arena: &mut ETermArena, goal: usize, head: usize) -> (out: bool)
    requires
        root_ok(old(arena), goal),
        root_ok(old(arena), head),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ckc_spec::engine::unifiable_apart(
            old(arena)@[goal as int],
            old(arena)@[head as int],
        ),
{
    let mark = arena.nodes.len();
    let ghost base = arena.nodes@;
    let off = nvars_root(arena, goal);
    proof {
        arena_prefix_stable(base, arena);
    }
    let shifted = shift_root(arena, head, off);
    proof {
        arena_prefix_stable(base, arena);
    }
    let mut pairs = Vec::new();
    pairs.push(EPair { left: goal, right: shifted });
    let initial = EBoundState { pairs, stack: Vec::new(), sol: Vec::new() };
    proof {
        assert(off == ckc_spec::engine::nvars(base[goal as int].term@));
        assert(arena@[goal as int] == base[goal as int].term@);
        assert(arena@[shifted as int] == ckc_spec::engine::shift(
            base[head as int].term@,
            off as nat,
        ));
        assert_seqs_equal!(pairs_view(arena.nodes@, initial.pairs@)
            == seq![(base[goal as int].term@, ckc_spec::engine::shift(base[head as int].term@, off as nat))]);
        assert_seqs_equal!(goals_view(arena.nodes@, initial.stack@) == Seq::empty());
        assert_seqs_equal!(root_terms(arena.nodes@, initial.sol@) == Seq::empty());
        reveal(bound_state_view);
        reveal(ustate_view);
        assert(bound_state_view(arena.nodes@, &initial) == ckc_spec::engine::UState {
            pairs: seq![
                (
                    base[goal as int].term@,
                    ckc_spec::engine::shift(
                        base[head as int].term@,
                        ckc_spec::engine::nvars(base[goal as int].term@),
                    ),
                ),
            ],
            stack: Seq::empty(),
            sol: Seq::empty(),
        });
    }
    let result = unify(arena, initial);
    proof {
        assert(base.is_prefix_of(arena.nodes@));
        reveal(ckc_spec::engine::unifiable_apart);
    }
    let found = match result {
        EUResult::Ok { .. } => true,
        EUResult::Fail => false,
    };
    crate::k2_store::truncate(arena, mark);
    found
}

pub open spec fn model_goal_level(goal: ckc_spec::engine::Goal, level: nat) -> bool {
    match goal {
        ckc_spec::engine::Goal::Lit(_, _) => true,
        ckc_spec::engine::Goal::NafCut(cut) => cut <= level,
    }
}

pub open spec fn model_goals_levels(goals: Seq<ckc_spec::engine::Goal>, level: nat) -> bool {
    forall|i: int| 0 <= i < goals.len() ==> #[trigger] model_goal_level(goals[i], level)
}

proof fn goals_levels_model(nodes: Seq<ENode>, goals: Seq<EGoal>, level: nat)
    ensures
        goals_levels(goals, level) == model_goals_levels(goals_view(nodes, goals), level),
{
    assert forall|i: int| 0 <= i < goals.len() implies goal_level_valid(&goals[i], level)
        == model_goal_level(goals_view(nodes, goals)[i], level) by {
        reveal(goal_level_valid);
        reveal(goal_view);
        reveal(model_goal_level);
    }
}

proof fn subst_preserves_levels(goals: Seq<ckc_spec::engine::Goal>, x: nat, value: Term, level: nat)
    requires
        model_goals_levels(goals, level),
    ensures
        model_goals_levels(
            goals.map_values(|g: ckc_spec::engine::Goal| ckc_spec::engine::subst_goal(g, x, value)),
            level,
        ),
{
    let out = goals.map_values(
        |g: ckc_spec::engine::Goal| ckc_spec::engine::subst_goal(g, x, value),
    );
    assert forall|i: int| 0 <= i < out.len() implies model_goal_level(out[i], level) by {
        assert(model_goal_level(goals[i], level));
        reveal(ckc_spec::engine::subst_goal);
        reveal(model_goal_level);
    }
}

proof fn unify_n_preserves_levels(u: ckc_spec::engine::UState, fuel: nat, level: nat)
    requires
        model_goals_levels(u.stack, level),
    ensures
        ckc_spec::engine::unify_n(u, fuel) matches ckc_spec::engine::UOut::Ok(stack, _)
            ==> model_goals_levels(stack, level),
    decreases fuel,
{
    reveal_with_fuel(ckc_spec::engine::unify_n, 2);
    if fuel > 0 && u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let next = match (a, b) {
            (Term::Var(x), _) => {
                if a == b {
                    Some(ckc_spec::engine::UState { pairs: rest, ..u })
                } else if !ckc_spec::engine::occurs(x, b) {
                    subst_preserves_levels(u.stack, x, b, level);
                    Some(ckc_spec::engine::u_bind(u, rest, x, b))
                } else {
                    None
                }
            },
            (_, Term::Var(y)) => {
                if !ckc_spec::engine::occurs(y, a) {
                    subst_preserves_levels(u.stack, y, a, level);
                    Some(ckc_spec::engine::u_bind(u, rest, y, a))
                } else {
                    None
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                if n == m && xs.len() == ys.len() {
                    Some(
                        ckc_spec::engine::UState {
                            pairs: ckc_spec::engine::zip(xs, ys) + rest,
                            ..u
                        },
                    )
                } else {
                    None
                }
            },
            _ => if a == b {
                Some(ckc_spec::engine::UState { pairs: rest, ..u })
            } else {
                None
            },
        };
        if let Some(state) = next {
            unify_n_preserves_levels(state, (fuel - 1) as nat, level);
        }
    }
}

pub proof fn unify_preserves_levels(u: ckc_spec::engine::UState, level: nat)
    requires
        model_goals_levels(u.stack, level),
    ensures
        ckc_spec::engine::unify(u) matches ckc_spec::engine::UOut::Ok(stack, _)
            ==> model_goals_levels(stack, level),
{
    reveal(ckc_spec::engine::unify);
    if exists|fuel: nat| !(ckc_spec::engine::unify_n(u, fuel) is Out) {
        let fuel = choose|fuel: nat| !(ckc_spec::engine::unify_n(u, fuel) is Out);
        unify_n_preserves_levels(u, fuel, level);
    }
}

pub open spec fn predicate_dispatch(c: ckc_spec::engine::Cfg) -> bool {
    c.stack.len() > 0 && match c.stack[0] {
        ckc_spec::engine::Goal::Lit(Term::Comp(name, args), d) => {
            d > 0 && !(name == ckc_spec::engine::comma_name() && args.len() == 2) && !(name
                == ckc_spec::engine::naf_name() && args.len() == 1)
        },
        _ => false,
    }
}

proof fn goals_view_concat(nodes: Seq<ENode>, left: Seq<EGoal>, right: Seq<EGoal>)
    ensures
        goals_view(nodes, left + right) == goals_view(nodes, left) + goals_view(nodes, right),
{
    assert_seqs_equal!(goals_view(nodes, left + right) == goals_view(nodes, left) + goals_view(nodes, right));
}

proof fn goals_levels_weaken(goals: Seq<EGoal>, before: nat, after: nat)
    requires
        goals_levels(goals, before),
        before <= after,
    ensures
        goals_levels(goals, after),
{
    assert forall|i: int| 0 <= i < goals.len() implies goal_level_valid(&goals[i], after) by {
        assert(goal_level_valid(&goals[i], before));
        reveal(goal_level_valid);
    }
}

#[verifier::rlimit(5000)]
pub fn step_simple(
    arena: &ETermArena,
    initial: ECfg,
    Ghost(db): Ghost<Seq<ckc_spec::v1text::DocClause>>,
) -> (step: EStep)
    requires
        arena_ok(arena),
        cfg_valid(arena.nodes@, &initial),
        !predicate_dispatch(cfg_view(arena.nodes@, &initial)),
    ensures
        step_valid(arena.nodes@, &step),
        step_base(&step, initial.base),
        step_view(arena.nodes@, &step) == ckc_spec::engine::step(
            db,
            cfg_view(arena.nodes@, &initial),
        ),
{
    reveal(cfg_valid);
    reveal(cfg_storage_valid);
    reveal(alt_valid);
    let ghost before = cfg_view(arena.nodes@, &initial);
    let mut c = initial;
    if c.stack.len() == 0 {
        if !c.collect {
            return EStep::Sol;
        }
        let ghost prior_rows = c.rows@;
        let ghost prior_saved = c.saved.nodes@;
        let copied = crate::k2_store::copy_roots(arena, &mut c.saved, &c.sol);
        proof {
            crate::k2_store::rows_prefix(prior_saved, c.saved.nodes@, prior_rows);
        }
        c.rows.push(copied);
        proof {
            assert_seqs_equal!(rows_view(c.saved.nodes@, c.rows@)
                == rows_view(prior_saved, prior_rows).push(before.sol));
            assert(cfg_view(arena.nodes@, &c) == ckc_spec::engine::Cfg {
                rows: before.rows.push(before.sol),
                ..before
            });
        }
        return fail(arena, c);
    }
    let front = c.stack[0];
    proof {
        assert(before.stack[0] == goal_view(arena.nodes@, &front));
        reveal(goal_view);
        reveal(ckc_spec::engine::step);
        reveal(predicate_dispatch);
    }
    match front {
        EGoal::NafCut { level } => {
            proof {
                assert(goal_level_valid(&c.stack@[0], c.alts@.len()));
                reveal(goal_level_valid);
            }
            let ghost old_alts = c.alts@;
            c.alts.truncate(level);
            proof {
                assert forall|i: int| 0 <= i < c.alts.len() implies {
                    &&& alt_valid(arena.nodes@, &c.alts@[i], i as nat)
                    &&& c.base <= c.alts@[i].mark <= c.mark
                } by {
                    assert(c.alts@[i] == old_alts[i]);
                }
                assert forall|i: int, j: int| 0 <= i < j < c.alts.len() implies c.alts@[i].mark
                    <= c.alts@[j].mark by {
                    assert(c.alts@[i] == old_alts[i]);
                    assert(c.alts@[j] == old_alts[j]);
                }
                assert(cfg_storage_valid(arena.nodes@, &c));
                assert_seqs_equal!(alts_view(arena.nodes@, c.alts@)
                    == alts_view(arena.nodes@, old_alts).take(level as int));
                assert(cfg_view(arena.nodes@, &c) == ckc_spec::engine::Cfg {
                    alts: before.alts.take(level as int),
                    ..before
                });
                assert(before.stack.len() > 0);
                assert(before.stack[0] == ckc_spec::engine::Goal::NafCut(level as nat));
                assert(ckc_spec::engine::step(db, before) == ckc_spec::engine::fail(
                    cfg_view(arena.nodes@, &c),
                ));
            }
            let result = fail(arena, c);
            proof {
                assert(step_view(arena.nodes@, &result) == ckc_spec::engine::step(db, before));
                assert(before == cfg_view(arena.nodes@, &initial));
                assert(step_view(arena.nodes@, &result) == ckc_spec::engine::step(
                    db,
                    cfg_view(arena.nodes@, &initial),
                ));
            }
            return result;
        },
        EGoal::Lit { root, depth } => {
            if depth == 0 {
                c.pruned = true;
                return fail(arena, c);
            }
            proof {
                reveal(cfg_valid);
                reveal(goals_valid);
                assert(goal_valid(arena.nodes@, &c.stack@[0]));
                assert(goal_valid(arena.nodes@.take(c.mark as int), &c.stack@[0]));
                assert(root < c.mark);
                reveal(goal_valid);
                reveal(arena_ok);
                assert(node_ok(arena.nodes@, root as int));
                reveal(node_ok);
            }
            match &arena.nodes[root].kind {
                ENodeKind::Comp { name, child_roots, .. } => {
                    let mut rest = c.stack.clone();
                    let unused = rest.remove(0);
                    let ghost rest_model = goals_view(arena.nodes@, rest@);
                    proof {
                        assert_seqs_equal!(rest_model == before.stack.drop_first());
                        node_comp_model(arena.nodes@, root as int, name@, child_roots@);
                    }
                    let mut comma = Vec::new();
                    comma.push(b',');
                    let mut naf = Vec::new();
                    naf.push(b'\\');
                    naf.push(b'+');
                    proof {
                        reveal_strlit(",");
                        reveal_strlit("\\+");
                        reveal(ckc_spec::engine::comma_name);
                        reveal(ckc_spec::engine::naf_name);
                        reveal(ckc_spec::v1text::ascii);
                        assert_seqs_equal!(comma@ == ckc_spec::engine::comma_name());
                        assert_seqs_equal!(naf@ == ckc_spec::engine::naf_name());
                    }
                    if child_roots.len() == 2 && vec_equal(name, &comma) {
                        let mut next = Vec::new();
                        next.push(EGoal::Lit { root: child_roots[0], depth });
                        next.push(EGoal::Lit { root: child_roots[1], depth });
                        proof {
                            goals_view_concat(arena.nodes@, next@, rest@);
                        }
                        next.append(&mut rest);
                        c.stack = next;
                        proof {
                            assert_seqs_equal!(goals_view(arena.nodes@, c.stack@) == seq![
                                ckc_spec::engine::Goal::Lit(arena@[child_roots@[0] as int], depth as nat),
                                ckc_spec::engine::Goal::Lit(arena@[child_roots@[1] as int], depth as nat),
                            ] + rest_model);
                        }
                        proof {
                            assert(cfg_valid(arena.nodes@, &c));
                        }
                        EStep::Next(c)
                    } else if child_roots.len() == 1 && vec_equal(name, &naf) {
                        let level = c.alts.len();
                        let continuation = EAlt {
                            mark: c.mark,
                            stack: rest.clone(),
                            sol: c.sol.clone(),
                            fresh: c.fresh,
                            ci: 0,
                        };
                        let ghost old_alts = c.alts@;
                        proof {
                            assert(alt_valid(arena.nodes@, &continuation, level as nat));
                            goals_levels_weaken(rest@, level as nat, level as nat + 1);
                        }
                        c.alts.push(continuation);
                        let mut next = Vec::new();
                        next.push(EGoal::Lit { root: child_roots[0], depth: depth - 1 });
                        next.push(EGoal::NafCut { level });
                        proof {
                            goals_view_concat(arena.nodes@, next@, rest@);
                        }
                        next.append(&mut rest);
                        c.stack = next;
                        proof {
                            assert_seqs_equal!(goals_view(arena.nodes@, c.stack@) == seq![
                                ckc_spec::engine::Goal::Lit(arena@[child_roots@[0] as int], (depth - 1) as nat),
                                ckc_spec::engine::Goal::NafCut(level as nat),
                            ] + rest_model);
                            assert_seqs_equal!(alts_view(arena.nodes@, c.alts@)
                                == alts_view(arena.nodes@, old_alts).push(
                                    ckc_spec::engine::Alt {
                                        stack: rest_model, sol: before.sol,
                                        fresh: before.fresh, ci: 0,
                                    }));
                        }
                        proof {
                            assert forall|i: int| 0 <= i < c.alts.len() implies {
                                &&& alt_valid(arena.nodes@, &c.alts@[i], i as nat)
                                &&& c.base <= c.alts@[i].mark <= c.mark
                            } by {
                                if i < old_alts.len() {
                                    assert(c.alts@[i] == old_alts[i]);
                                } else {
                                    assert(c.alts@[i] == continuation);
                                }
                            }
                            assert forall|i: int, j: int|
                                0 <= i < j < c.alts.len() implies c.alts@[i].mark
                                <= c.alts@[j].mark by {
                                if j < old_alts.len() {
                                    assert(c.alts@[i] == old_alts[i]);
                                    assert(c.alts@[j] == old_alts[j]);
                                } else {
                                    assert(c.alts@[i] == old_alts[i]);
                                    assert(c.alts@[j] == continuation);
                                }
                            }
                            assert(cfg_storage_valid(arena.nodes@, &c));
                            assert(cfg_valid(arena.nodes@, &c));
                        }
                        EStep::Next(c)
                    } else {
                        proof {
                            assert(false);
                        }
                        EStep::Sol
                    }
                },
                _ => fail(arena, c),
            }
        },
    }
}

pub enum EBodyItem {
    Pos { root: usize },
    Naf { roots: Vec<usize> },
}

pub struct EClause {
    pub head: usize,
    pub body: Vec<EBodyItem>,
}

pub open spec fn body_item_view(nodes: Seq<ENode>, item: &EBodyItem) -> ckc_spec::v1text::BodyItem {
    match item {
        EBodyItem::Pos { root } => ckc_spec::v1text::BodyItem::Pos(nodes[*root as int].term@),
        EBodyItem::Naf { roots } => ckc_spec::v1text::BodyItem::Naf(root_terms(nodes, roots@)),
    }
}

pub open spec fn body_items_view(nodes: Seq<ENode>, items: Seq<EBodyItem>) -> Seq<
    ckc_spec::v1text::BodyItem,
> {
    Seq::new(items.len(), |i: int| body_item_view(nodes, &items[i]))
}

pub open spec fn clause_view(nodes: Seq<ENode>, clause: &EClause) -> ckc_spec::v1text::DocClause {
    ckc_spec::v1text::DocClause {
        head: nodes[clause.head as int].term@,
        body: body_items_view(nodes, clause.body@),
    }
}

pub open spec fn db_view(nodes: Seq<ENode>, db: Seq<EClause>) -> Seq<ckc_spec::v1text::DocClause> {
    Seq::new(db.len(), |i: int| clause_view(nodes, &db[i]))
}

pub open spec fn body_item_valid(nodes: Seq<ENode>, item: &EBodyItem) -> bool {
    match item {
        EBodyItem::Pos { root } => *root < nodes.len(),
        EBodyItem::Naf { roots } => roots@.len() > 0 && roots_valid(nodes, roots@),
    }
}

pub open spec fn body_items_valid(nodes: Seq<ENode>, items: Seq<EBodyItem>) -> bool {
    forall|i: int| 0 <= i < items.len() ==> #[trigger] body_item_valid(nodes, &items[i])
}

pub open spec fn clause_valid(nodes: Seq<ENode>, clause: &EClause) -> bool {
    clause.head < nodes.len() && body_items_valid(nodes, clause.body@)
}

pub open spec fn db_valid(nodes: Seq<ENode>, db: Seq<EClause>) -> bool {
    forall|i: int| 0 <= i < db.len() ==> #[trigger] clause_valid(nodes, &db[i])
}

proof fn body_item_models_prefix(before: Seq<ENode>, after: Seq<ENode>, item: &EBodyItem)
    requires
        before.is_prefix_of(after),
        body_item_valid(before, item),
    ensures
        body_item_valid(after, item),
        body_item_view(before, item) == body_item_view(after, item),
{
    match item {
        EBodyItem::Pos { root } => {
            assert(before[*root as int] == after[*root as int]);
        },
        EBodyItem::Naf { roots } => {
            roots_models_prefix(before, after, roots@);
        },
    }
}

pub proof fn clause_models_prefix(before: Seq<ENode>, after: Seq<ENode>, clause: &EClause)
    requires
        before.is_prefix_of(after),
        clause_valid(before, clause),
    ensures
        clause_valid(after, clause),
        clause_view(before, clause) == clause_view(after, clause),
{
    assert forall|i: int| 0 <= i < clause.body@.len() implies {
        &&& body_item_valid(after, &clause.body@[i])
        &&& body_item_view(before, &clause.body@[i]) == body_item_view(after, &clause.body@[i])
    } by {
        assert(body_item_valid(before, &clause.body@[i]));
        body_item_models_prefix(before, after, &clause.body@[i]);
    }
    assert_seqs_equal!(body_items_view(before, clause.body@) == body_items_view(after, clause.body@));
}

pub proof fn db_models_prefix(before: Seq<ENode>, after: Seq<ENode>, db: Seq<EClause>)
    requires
        before.is_prefix_of(after),
        db_valid(before, db),
    ensures
        db_valid(after, db),
        db_view(before, db) == db_view(after, db),
{
    assert forall|i: int| 0 <= i < db.len() implies {
        &&& clause_valid(after, &db[i])
        &&& clause_view(before, &db[i]) == clause_view(after, &db[i])
    } by {
        assert(clause_valid(before, &db[i]));
        clause_models_prefix(before, after, &db[i]);
    }
    assert_seqs_equal!(db_view(before, db) == db_view(after, db));
}

pub fn literal_matches(arena: &ETermArena, root: usize, name: &Vec<u8>, arity: usize) -> (matched:
    bool)
    requires
        root_ok(arena, root),
    ensures
        matched == (ckc_spec::engine::lit_fa(arena@[root as int]) == Some((name@, arity as nat))),
{
    proof {
        reveal(root_ok);
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok);
        reveal(ckc_spec::engine::lit_fa);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name: stored, child_roots, .. } => {
            proof {
                node_comp_model(arena.nodes@, root as int, stored@, child_roots@);
            }
            child_roots.len() == arity && vec_equal(stored, name)
        },
        _ => false,
    }
}

pub fn next_match(
    arena: &ETermArena,
    db: &Vec<EClause>,
    name: &Vec<u8>,
    arity: usize,
    from: usize,
) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
    ensures
        match out {
            Some(m) => {
                &&& from <= m < db@.len()
                &&& ckc_spec::engine::next_match(
                    db_view(arena.nodes@, db@),
                    name@,
                    arity as nat,
                    from as nat,
                ) == Some(m as nat)
                &&& ckc_spec::engine::lit_fa(clause_view(arena.nodes@, &db@[m as int]).head)
                    == Some((name@, arity as nat))
            },
            None => ckc_spec::engine::next_match(
                db_view(arena.nodes@, db@),
                name@,
                arity as nat,
                from as nat,
            ) is None,
        },
{
    let mut i = from;
    while i < db.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            from <= i,
            ckc_spec::engine::next_match(
                db_view(arena.nodes@, db@),
                name@,
                arity as nat,
                from as nat,
            ) == ckc_spec::engine::next_match(
                db_view(arena.nodes@, db@),
                name@,
                arity as nat,
                i as nat,
            ),
        decreases db.len() as int - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &db@[i as int]));
        }
        let matched = literal_matches(arena, db[i].head, name, arity);
        proof {
            reveal(ckc_spec::engine::next_match);
        }
        if matched {
            return Some(i);
        }
        i += 1;
    }
    proof {
        reveal(ckc_spec::engine::next_match);
    }
    None
}

#[verifier::rlimit(5000)]
fn shift_roots(arena: &mut ETermArena, roots: &Vec<usize>, off: usize) -> (out: Vec<usize>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
        off <= old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        roots_valid(final(arena).nodes@, out@),
        root_terms(final(arena).nodes@, out@) == ckc_spec::engine::shift_all(
            root_terms(old(arena).nodes@, roots@),
            off as nat,
        ),
        off as nat + ckc_spec::engine::nvars_all(root_terms(old(arena).nodes@, roots@))
            <= final(arena).nodes@.len(),
{
    let ghost base = arena.nodes@;
    let mut i = roots.len();
    let mut out = Vec::new();
    proof {
        reveal(ckc_spec::engine::shift_all);
        reveal(roots_nvars);
    }
    while i > 0
        invariant
            base == old(arena).nodes@,
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            roots_valid(base, roots@),
            off <= base.len(),
            i <= roots@.len(),
            roots_valid(arena.nodes@, out@),
            root_terms(arena.nodes@, out@) == ckc_spec::engine::shift_all(
                root_terms(base, roots@.skip(i as int)),
                off as nat,
            ),
            off as nat + roots_nvars(base, roots@.skip(i as int)) <= arena.nodes@.len(),
        decreases i,
    {
        let ghost before = arena.nodes@;
        let ghost suffix = roots@.skip(i as int);
        let ghost prior = out@;
        i -= 1;
        let root = roots[i];
        proof {
            arena_prefix_stable(base, arena);
        }
        let shifted = shift_root(arena, root, off);
        proof {
            roots_models_prefix(before, arena.nodes@, prior);
            assert(base.is_prefix_of(arena.nodes@));
            assert(arena@[shifted as int] == ckc_spec::engine::shift(
                base[root as int].term@,
                off as nat,
            ));
            assert_seqs_equal!(roots@.skip(i as int) == seq![root] + suffix);
            assert_seqs_equal!(root_terms(base, roots@.skip(i as int))
                == seq![base[root as int].term@] + root_terms(base, suffix));
            assert(roots@.skip(i as int).drop_first() == suffix);
            assert(root_terms(base, roots@.skip(i as int)).drop_first() == root_terms(
                base,
                suffix,
            ));
            assert(root_terms(base, roots@.skip(i as int))[0] == base[root as int].term@);
            assert(off as nat + ckc_spec::engine::nvars(base[root as int].term@)
                <= arena.nodes@.len());
            reveal_with_fuel(ckc_spec::engine::shift_all, 2);
            reveal_with_fuel(roots_nvars, 2);
            assert(ckc_spec::engine::shift_all(root_terms(base, roots@.skip(i as int)), off as nat)
                == seq![arena@[shifted as int]] + root_terms(arena.nodes@, prior));
            assert(off as nat + roots_nvars(base, roots@.skip(i as int)) <= arena.nodes@.len());
        }
        out.insert(0, shifted);
        proof {
            assert_seqs_equal!(out@ == seq![shifted] + prior);
            assert_seqs_equal!(root_terms(arena.nodes@, out@)
                == seq![arena@[shifted as int]] + root_terms(arena.nodes@, prior));
        }
    }
    proof {
        assert(roots@.skip(0) == roots@);
        nvars_all_root_terms(base, roots@);
    }
    out
}

#[verifier::rlimit(5000)]
fn conj_root(arena: &mut ETermArena, roots: &Vec<usize>) -> (out: usize)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
        roots@.len() > 0,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out < final(arena).nodes@.len(),
        final(arena)@[out as int] == ckc_spec::engine::conj_term(
            root_terms(old(arena).nodes@, roots@),
        ),
{
    let ghost base = arena.nodes@;
    let mut i = roots.len() - 1;
    let mut out = roots[i];
    let mut comma = Vec::new();
    comma.push(b',');
    proof {
        reveal_strlit(",");
        reveal(ckc_spec::engine::comma_name);
        reveal(ckc_spec::v1text::ascii);
        assert_seqs_equal!(comma@ == ckc_spec::engine::comma_name());
        reveal(ckc_spec::engine::conj_term);
    }
    while i > 0
        invariant
            base == old(arena).nodes@,
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            roots_valid(base, roots@),
            i < roots@.len(),
            out < arena.nodes@.len(),
            comma@ == ckc_spec::engine::comma_name(),
            arena@[out as int] == ckc_spec::engine::conj_term(
                root_terms(base, roots@.skip(i as int)),
            ),
        decreases i,
    {
        let ghost previous = arena@[out as int];
        let ghost suffix = roots@.skip(i as int);
        i -= 1;
        let first = roots[i];
        let mut children = Vec::new();
        children.push(first);
        children.push(out);
        proof {
            arena_prefix_stable(base, arena);
            assert_seqs_equal!(child_terms(arena.nodes@, children@)
                == seq![base[first as int].term@, previous]);
        }
        out = push_comp(arena, comma.clone(), children);
        proof {
            assert(base.is_prefix_of(arena.nodes@));
            assert_seqs_equal!(root_terms(base, roots@.skip(i as int))
                == seq![base[first as int].term@] + root_terms(base, suffix));
            assert(root_terms(base, roots@.skip(i as int)).drop_first() == root_terms(
                base,
                suffix,
            ));
            assert(root_terms(base, roots@.skip(i as int))[0] == base[first as int].term@);
            reveal(ckc_spec::engine::conj_term);
        }
    }
    proof {
        assert(roots@.skip(0) == roots@);
    }
    out
}

pub open spec fn item_nvars(item: ckc_spec::v1text::BodyItem) -> nat {
    match item {
        ckc_spec::v1text::BodyItem::Pos(root) => ckc_spec::engine::nvars(root),
        ckc_spec::v1text::BodyItem::Naf(roots) => ckc_spec::engine::nvars_all(roots),
    }
}

fn shift_item(arena: &mut ETermArena, item: &EBodyItem, off: usize) -> (out: usize)
    requires
        arena_ok(old(arena)),
        body_item_valid(old(arena).nodes@, item),
        off <= old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out < final(arena).nodes@.len(),
        final(arena)@[out as int] == ckc_spec::engine::item_term(
            body_item_view(old(arena).nodes@, item),
            off as nat,
        ),
        off as nat + item_nvars(body_item_view(old(arena).nodes@, item))
            <= final(arena).nodes@.len(),
{
    match item {
        EBodyItem::Pos { root } => shift_root(arena, *root, off),
        EBodyItem::Naf { roots } => {
            let ghost base = arena.nodes@;
            let shifted = shift_roots(arena, roots, off);
            proof {
                shift_all_map(root_terms(base, roots@), off as nat);
                assert(shifted@.len() == roots@.len());
            }
            let conjunction = conj_root(arena, &shifted);
            let mut name = Vec::new();
            name.push(b'\\');
            name.push(b'+');
            let mut children = Vec::new();
            children.push(conjunction);
            proof {
                reveal_strlit("\\+");
                reveal(ckc_spec::engine::naf_name);
                reveal(ckc_spec::v1text::ascii);
                assert_seqs_equal!(name@ == ckc_spec::engine::naf_name());
                assert_seqs_equal!(child_terms(arena.nodes@, children@) == seq![arena@[conjunction as int]]);
            }
            let out = push_comp(arena, name, children);
            proof {
                assert(base.is_prefix_of(arena.nodes@));
            }
            out
        },
    }
}

pub fn roots_max_var(arena: &ETermArena, roots: &Vec<usize>) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, roots@),
    ensures
        max_var_count(out) == ckc_spec::engine::nvars_all(root_terms(arena.nodes@, roots@)),
{
    let mut i = roots.len();
    let mut out = None;
    proof {
        reveal(roots_nvars);
    }
    while i > 0
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, roots@),
            i <= roots@.len(),
            max_var_count(out) == roots_nvars(arena.nodes@, roots@.skip(i as int)),
        decreases i,
    {
        let ghost suffix = roots@.skip(i as int);
        i -= 1;
        let maximum = max_var_root(arena, roots[i]);
        out = max_var_merge(maximum, out);
        proof {
            assert(roots@.skip(i as int).drop_first() == suffix);
            reveal(roots_nvars);
        }
    }
    proof {
        assert(roots@.skip(0) == roots@);
        nvars_all_root_terms(arena.nodes@, roots@);
    }
    out
}

fn items_max_var(arena: &ETermArena, items: &Vec<EBodyItem>) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        body_items_valid(arena.nodes@, items@),
    ensures
        max_var_count(out) == ckc_spec::engine::items_nvars(body_items_view(arena.nodes@, items@)),
{
    let mut i = items.len();
    let mut out = None;
    proof {
        reveal(ckc_spec::engine::items_nvars);
    }
    while i > 0
        invariant
            arena_ok(arena),
            body_items_valid(arena.nodes@, items@),
            i <= items@.len(),
            max_var_count(out) == ckc_spec::engine::items_nvars(
                body_items_view(arena.nodes@, items@.skip(i as int)),
            ),
        decreases i,
    {
        let ghost suffix = items@.skip(i as int);
        i -= 1;
        proof {
            assert(body_item_valid(arena.nodes@, &items@[i as int]));
        }
        let maximum = match &items[i] {
            EBodyItem::Pos { root } => max_var_root(arena, *root),
            EBodyItem::Naf { roots } => roots_max_var(arena, roots),
        };
        proof {
            assert(max_var_count(maximum) == item_nvars(
                body_item_view(arena.nodes@, &items@[i as int]),
            ));
        }
        out = max_var_merge(maximum, out);
        proof {
            assert_seqs_equal!(body_items_view(arena.nodes@, items@.skip(i as int)).drop_first()
                == body_items_view(arena.nodes@, suffix));
            reveal(ckc_spec::engine::items_nvars);
        }
    }
    proof {
        assert(items@.skip(0) == items@);
    }
    out
}

pub fn clause_nvars(arena: &mut ETermArena, clause: &EClause) -> (out: usize)
    requires
        arena_ok(old(arena)),
        clause_valid(old(arena).nodes@, clause),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ckc_spec::engine::clause_nvars(clause_view(old(arena).nodes@, clause)),
        out <= final(arena).nodes@.len(),
{
    let head = max_var_root(arena, clause.head);
    let body = items_max_var(arena, &clause.body);
    let maximum = max_var_merge(head, body);
    ensure_var_capacity(arena, maximum)
}

#[verifier::rlimit(5000)]
fn body_goals(arena: &mut ETermArena, items: &Vec<EBodyItem>, off: usize, depth: usize) -> (out:
    Vec<EGoal>)
    requires
        arena_ok(old(arena)),
        body_items_valid(old(arena).nodes@, items@),
        off <= old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        goals_valid(final(arena).nodes@, out@),
        goals_levels(out@, 0),
        goals_view(final(arena).nodes@, out@) == ckc_spec::engine::body_goals(
            body_items_view(old(arena).nodes@, items@),
            off as nat,
            depth as nat,
        ),
        off as nat + ckc_spec::engine::items_nvars(body_items_view(old(arena).nodes@, items@))
            <= final(arena).nodes@.len(),
{
    let ghost base = arena.nodes@;
    let mut i = items.len();
    let mut out = Vec::new();
    proof {
        reveal(ckc_spec::engine::items_nvars);
    }
    while i > 0
        invariant
            base == old(arena).nodes@,
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            body_items_valid(base, items@),
            off <= base.len(),
            i <= items@.len(),
            goals_valid(arena.nodes@, out@),
            goals_levels(out@, 0),
            goals_view(arena.nodes@, out@) == ckc_spec::engine::body_goals(
                body_items_view(base, items@.skip(i as int)),
                off as nat,
                depth as nat,
            ),
            off as nat + ckc_spec::engine::items_nvars(body_items_view(base, items@.skip(i as int)))
                <= arena.nodes@.len(),
        decreases i,
    {
        let ghost before = arena.nodes@;
        let ghost suffix = items@.skip(i as int);
        let ghost prior = out@;
        i -= 1;
        proof {
            assert(body_item_valid(base, &items@[i as int]));
            body_item_models_prefix(base, arena.nodes@, &items@[i as int]);
        }
        let root = shift_item(arena, &items[i], off);
        proof {
            goals_models_prefix(before, arena.nodes@, prior);
            assert(base.is_prefix_of(arena.nodes@));
            assert_seqs_equal!(body_items_view(base, items@.skip(i as int))
                == seq![body_item_view(base, &items@[i as int])] + body_items_view(base, suffix));
            assert(body_items_view(base, items@.skip(i as int)).drop_first() == body_items_view(
                base,
                suffix,
            ));
            reveal(ckc_spec::engine::items_nvars);
            assert(off as nat + ckc_spec::engine::items_nvars(
                body_items_view(base, items@.skip(i as int)),
            ) <= arena.nodes@.len());
        }
        out.insert(0, EGoal::Lit { root, depth });
        proof {
            assert(arena@[root as int] == ckc_spec::engine::item_term(
                body_item_view(base, &items@[i as int]),
                off as nat,
            ));
            assert_seqs_equal!(out@ == seq![EGoal::Lit { root, depth }] + prior);
            goals_view_concat(arena.nodes@, seq![EGoal::Lit { root, depth }], prior);
            assert_seqs_equal!(ckc_spec::engine::body_goals(
                body_items_view(base, items@.skip(i as int)), off as nat, depth as nat,
            ) == seq![ckc_spec::engine::Goal::Lit(arena@[root as int], depth as nat)]
                + ckc_spec::engine::body_goals(body_items_view(base, suffix), off as nat, depth as nat));
        }
    }
    proof {
        assert(items@.skip(0) == items@);
    }
    out
}

pub fn args_roots(arena: &ETermArena, root: usize) -> (out: Vec<usize>)
    requires
        root_ok(arena, root),
    ensures
        roots_valid(arena.nodes@, out@),
        root_terms(arena.nodes@, out@) == ckc_spec::engine::args_of(arena@[root as int]),
{
    proof {
        reveal(root_ok);
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok);
        reveal(ckc_spec::engine::args_of);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            proof {
                node_comp_model(arena.nodes@, root as int, name@, child_roots@);
            }
            child_roots.clone()
        },
        _ => Vec::new(),
    }
}

#[verifier::rlimit(5000)]
fn prepare_clause(
    arena: &mut ETermArena,
    clause: &EClause,
    args: &Vec<usize>,
    rest: &Vec<EGoal>,
    sol: &Vec<usize>,
    off: usize,
    depth: usize,
    Ghost(level): Ghost<nat>,
) -> (out: (EBoundState, usize))
    requires
        arena_ok(old(arena)),
        clause_valid(old(arena).nodes@, clause),
        roots_valid(old(arena).nodes@, args@),
        roots_valid(old(arena).nodes@, sol@),
        goals_valid(old(arena).nodes@, rest@),
        goals_levels(rest@, level),
        off <= old(arena).nodes@.len(),
        clause_view(old(arena).nodes@, clause).head is Comp,
        args@.len() == ckc_spec::engine::args_of(clause_view(old(arena).nodes@, clause).head).len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        bound_state_valid(final(arena).nodes@, &out.0),
        goals_levels(out.0.stack@, level),
        out.1 == off as nat + ckc_spec::engine::clause_nvars(
            clause_view(old(arena).nodes@, clause),
        ),
        out.1 <= final(arena).nodes@.len(),
        bound_state_view(final(arena).nodes@, &out.0) == (ckc_spec::engine::UState {
            pairs: ckc_spec::engine::zip(
                root_terms(old(arena).nodes@, args@),
                ckc_spec::engine::args_of(
                    ckc_spec::engine::shift(
                        clause_view(old(arena).nodes@, clause).head,
                        off as nat,
                    ),
                ),
            ),
            stack: ckc_spec::engine::body_goals(
                body_items_view(old(arena).nodes@, clause.body@),
                off as nat,
                depth as nat,
            ) + goals_view(old(arena).nodes@, rest@),
            sol: root_terms(old(arena).nodes@, sol@),
        }),
{
    let ghost base = arena.nodes@;
    let count = clause_nvars(arena, clause);
    proof {
        arena_prefix_stable(base, arena);
        clause_models_prefix(base, arena.nodes@, clause);
    }
    let head = shift_root(arena, clause.head, off);
    let head_args = args_roots(arena, head);
    let ghost with_head = arena.nodes@;
    proof {
        clause_models_prefix(base, arena.nodes@, clause);
        assert(arena@[head as int] == ckc_spec::engine::shift(
            clause_view(base, clause).head,
            off as nat,
        ));
        match clause_view(base, clause).head {
            Term::Comp(name, children) => {
                shift_all_map(children, off as nat);
                reveal(ckc_spec::engine::shift);
                reveal(ckc_spec::engine::args_of);
                assert(root_terms(arena.nodes@, head_args@) == ckc_spec::engine::shift_all(
                    children,
                    off as nat,
                ));
                assert(head_args@.len() == children.len());
            },
            _ => {
                assert(false);
            },
        }
        assert(head_args@.len() == args@.len());
    }
    let mut stack = body_goals(arena, &clause.body, off, depth);
    proof {
        assert(base.is_prefix_of(arena.nodes@));
        roots_models_prefix(base, arena.nodes@, args@);
        roots_models_prefix(base, arena.nodes@, sol@);
        roots_models_prefix(with_head, arena.nodes@, head_args@);
        goals_models_prefix(base, arena.nodes@, rest@);
        goals_view_concat(arena.nodes@, stack@, rest@);
        goals_levels_weaken(stack@, 0, level);
        assert(off as nat + count <= arena.nodes@.len());
    }
    let mut continuation = rest.clone();
    stack.append(&mut continuation);
    let pairs = zip_pairs(arena, args, &head_args);
    let state = EBoundState { pairs, stack, sol: sol.clone() };
    let node_count = arena.nodes.len();
    proof {
        assert(off as nat + count <= node_count);
    }
    (state, off + count)
}

fn enter_clause(
    arena: &ETermArena,
    initial: ECfg,
    stack: Vec<EGoal>,
    sol: Vec<usize>,
    fresh: usize,
    next: usize,
) -> (out: EStep)
    requires
        arena_ok(arena),
        cfg_valid(arena.nodes@, &initial),
        goals_valid(arena.nodes@, stack@),
        goals_levels(stack@, initial.alts@.len()),
        roots_valid(arena.nodes@, sol@),
        fresh <= arena.nodes@.len(),
    ensures
        step_valid(arena.nodes@, &out),
        step_base(&out, initial.base),
        step_view(arena.nodes@, &out) == ckc_spec::engine::Step::Next(
            ckc_spec::engine::Cfg {
                stack: goals_view(arena.nodes@, stack@),
                sol: root_terms(arena.nodes@, sol@),
                fresh: fresh as nat,
                ci: 0,
                alts: cfg_view(arena.nodes@, &initial).alts.push(
                    ckc_spec::engine::Alt {
                        stack: cfg_view(arena.nodes@, &initial).stack,
                        sol: cfg_view(arena.nodes@, &initial).sol,
                        fresh: initial.fresh as nat,
                        ci: next as nat,
                    },
                ),
                ..cfg_view(arena.nodes@, &initial)
            },
        ),
{
    reveal(cfg_valid);
    reveal(cfg_storage_valid);
    reveal(alt_valid);
    let ghost before = cfg_view(arena.nodes@, &initial);
    let mut c = initial;
    let choice = EAlt {
        mark: c.mark,
        stack: c.stack.clone(),
        sol: c.sol.clone(),
        fresh: c.fresh,
        ci: next,
    };
    let ghost prior = c.alts@;
    proof {
        assert(alt_valid(arena.nodes@, &choice, prior.len()));
        goals_levels_weaken(stack@, prior.len(), prior.len() + 1);
    }
    c.alts.push(choice);
    c.mark = arena.nodes.len();
    c.stack = stack;
    c.sol = sol;
    c.fresh = fresh;
    c.ci = 0;
    proof {
        assert forall|i: int| 0 <= i < c.alts.len() implies {
            &&& alt_valid(arena.nodes@, &c.alts@[i], i as nat)
            &&& c.base <= c.alts@[i].mark <= c.mark
        } by {
            if i < prior.len() {
                assert(c.alts@[i] == prior[i]);
            } else {
                assert(c.alts@[i] == choice);
            }
        }
        assert forall|i: int, j: int| 0 <= i < j < c.alts.len() implies c.alts@[i].mark
            <= c.alts@[j].mark by {
            if j < prior.len() {
                assert(c.alts@[i] == prior[i]);
                assert(c.alts@[j] == prior[j]);
            } else {
                assert(c.alts@[j] == choice);
                assert(c.alts@[i] == prior[i]);
            }
        }
        assert(cfg_storage_valid(arena.nodes@, &c));
        assert_seqs_equal!(arena.nodes@.take(c.mark as int) == arena.nodes@);
        assert(goals_valid(arena.nodes@.take(c.mark as int), c.stack@));
        assert(roots_valid(arena.nodes@.take(c.mark as int), c.sol@));
        assert(cfg_valid(arena.nodes@, &c));
        assert_seqs_equal!(alts_view(arena.nodes@, c.alts@)
            == before.alts.push(ckc_spec::engine::Alt {
                stack: before.stack, sol: before.sol, fresh: before.fresh, ci: next as nat,
            }));
    }
    EStep::Next(c)
}

proof fn call_no_match(
    db: Seq<ckc_spec::v1text::DocClause>,
    c: ckc_spec::engine::Cfg,
    name: Seq<u8>,
    args: Seq<Term>,
    depth: nat,
    rest: Seq<ckc_spec::engine::Goal>,
    from: nat,
)
    requires
        ckc_spec::engine::next_match(db, name, args.len(), from) is None,
    ensures
        ckc_spec::engine::call(db, c, name, args, depth, rest, from) == ckc_spec::engine::fail(c),
{
    reveal(ckc_spec::engine::call);
}

proof fn call_match(
    db: Seq<ckc_spec::v1text::DocClause>,
    c: ckc_spec::engine::Cfg,
    name: Seq<u8>,
    args: Seq<Term>,
    depth: nat,
    rest: Seq<ckc_spec::engine::Goal>,
    from: nat,
    matched: nat,
    u: ckc_spec::engine::UState,
)
    requires
        ckc_spec::engine::next_match(db, name, args.len(), from) == Some(matched),
        u == (ckc_spec::engine::UState {
            pairs: ckc_spec::engine::zip(
                args,
                ckc_spec::engine::args_of(
                    ckc_spec::engine::shift(db[matched as int].head, c.fresh),
                ),
            ),
            stack: ckc_spec::engine::body_goals(
                db[matched as int].body,
                c.fresh,
                (depth - 1) as nat,
            ) + rest,
            sol: c.sol,
        }),
    ensures
        ckc_spec::engine::call(db, c, name, args, depth, rest, from)
            == match ckc_spec::engine::unify(u) {
            ckc_spec::engine::UOut::Ok(stack, sol) => ckc_spec::engine::Step::Next(
                ckc_spec::engine::Cfg {
                    stack,
                    sol,
                    fresh: c.fresh + ckc_spec::engine::clause_nvars(db[matched as int]),
                    ci: 0,
                    alts: c.alts.push(
                        ckc_spec::engine::Alt {
                            stack: c.stack,
                            sol: c.sol,
                            fresh: c.fresh,
                            ci: matched + 1,
                        },
                    ),
                    ..c
                },
            ),
            ckc_spec::engine::UOut::Fail => ckc_spec::engine::call(
                db,
                c,
                name,
                args,
                depth,
                rest,
                matched + 1,
            ),
            ckc_spec::engine::UOut::Out => ckc_spec::engine::Step::Stuck,
        },
{
    reveal(ckc_spec::engine::call);
}

#[verifier::rlimit(5000)]
pub fn call(
    arena: &mut ETermArena,
    db: &Vec<EClause>,
    initial: ECfg,
    name: Vec<u8>,
    args: Vec<usize>,
    depth: usize,
    rest: Vec<EGoal>,
    from: usize,
) -> (out: EStep)
    requires
        arena_ok(old(arena)),
        db_valid(old(arena).nodes@, db@),
        cfg_valid(old(arena).nodes@, &initial),
        roots_valid(old(arena).nodes@, args@),
        goals_valid(old(arena).nodes@, rest@),
        goals_levels(rest@, initial.alts@.len()),
        depth > 0,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        step_valid(final(arena).nodes@, &out),
        step_base(&out, initial.base),
        step_view(final(arena).nodes@, &out) == ckc_spec::engine::call(
            db_view(old(arena).nodes@, db@),
            cfg_view(old(arena).nodes@, &initial),
            name@,
            root_terms(old(arena).nodes@, args@),
            depth as nat,
            goals_view(old(arena).nodes@, rest@),
            from as nat,
        ),
{
    let mark = arena.nodes.len();
    let ghost base = arena.nodes@;
    let ghost model_db = db_view(base, db@);
    let ghost c = cfg_view(base, &initial);
    let ghost actual_args = root_terms(base, args@);
    let ghost continuation = goals_view(base, rest@);
    let mut ci = from;
    while ci < db.len()
        invariant
            base == old(arena).nodes@,
            mark == base.len(),
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            db_valid(base, db@),
            cfg_valid(base, &initial),
            roots_valid(base, args@),
            goals_valid(base, rest@),
            goals_levels(rest@, initial.alts@.len()),
            depth > 0,
            model_db == db_view(base, db@),
            c == cfg_view(base, &initial),
            actual_args == root_terms(base, args@),
            continuation == goals_view(base, rest@),
            ckc_spec::engine::call(
                model_db,
                c,
                name@,
                actual_args,
                depth as nat,
                continuation,
                from as nat,
            ) == ckc_spec::engine::call(
                model_db,
                c,
                name@,
                actual_args,
                depth as nat,
                continuation,
                ci as nat,
            ),
        decreases db.len() - ci,
    {
        proof {
            db_models_prefix(base, arena.nodes@, db@);
            cfg_models_prefix(base, arena.nodes@, &initial);
            roots_models_prefix(base, arena.nodes@, args@);
            goals_models_prefix(base, arena.nodes@, rest@);
        }
        let matched = next_match(arena, db, &name, args.len(), ci);
        match matched {
            None => {
                proof {
                    call_no_match(
                        model_db,
                        c,
                        name@,
                        actual_args,
                        depth as nat,
                        continuation,
                        ci as nat,
                    );
                }
                return fail(arena, initial);
            },
            Some(m) => {
                proof {
                    assert(clause_valid(arena.nodes@, &db@[m as int]));
                    reveal(ckc_spec::engine::lit_fa);
                }
                if head_clash(arena, &args, db[m].head, Ghost(initial.fresh as nat)) {
                    proof {
                        let u = ckc_spec::engine::UState {
                            pairs: ckc_spec::engine::zip(
                                actual_args,
                                ckc_spec::engine::args_of(
                                    ckc_spec::engine::shift(model_db[m as int].head, c.fresh),
                                ),
                            ),
                            stack: ckc_spec::engine::body_goals(
                                model_db[m as int].body,
                                c.fresh,
                                (depth - 1) as nat,
                            ) + continuation,
                            sol: c.sol,
                        };
                        clash_fails(u);
                        call_match(
                            model_db,
                            c,
                            name@,
                            actual_args,
                            depth as nat,
                            continuation,
                            ci as nat,
                            m as nat,
                            u,
                        );
                    }
                    ci = m + 1;
                    continue;
                }
                let (state, fresh) = prepare_clause(
                    arena,
                    &db[m],
                    &args,
                    &rest,
                    &initial.sol,
                    initial.fresh,
                    depth - 1,
                    Ghost(initial.alts@.len()),
                );
                let ghost u = bound_state_view(arena.nodes@, &state);
                proof {
                    goals_levels_model(arena.nodes@, state.stack@, initial.alts@.len());
                    unify_preserves_levels(u, initial.alts@.len());
                    call_match(
                        model_db,
                        c,
                        name@,
                        actual_args,
                        depth as nat,
                        continuation,
                        ci as nat,
                        m as nat,
                        u,
                    );
                }
                let result = unify(arena, state);
                proof {
                    assert(base.is_prefix_of(arena.nodes@));
                    cfg_models_prefix(base, arena.nodes@, &initial);
                }
                match result {
                    EUResult::Ok { stack, sol } => {
                        proof {
                            goals_levels_model(arena.nodes@, stack@, initial.alts@.len());
                        }
                        return enter_clause(arena, initial, stack, sol, fresh, m + 1);
                    },
                    EUResult::Fail => {
                        crate::k2_store::truncate(arena, mark);
                        proof {
                            assert(arena.nodes@ == base);
                        }
                        ci = m + 1;
                    },
                }
            },
        }
    }
    proof {
        cfg_models_prefix(base, arena.nodes@, &initial);
        reveal(ckc_spec::engine::next_match);
        call_no_match(model_db, c, name@, actual_args, depth as nat, continuation, ci as nat);
    }
    fail(arena, initial)
}

fn predicate_parts(arena: &ETermArena, c: &ECfg) -> (out: Option<(Vec<u8>, Vec<usize>, usize)>)
    requires
        arena_ok(arena),
        cfg_valid(arena.nodes@, c),
    ensures
        match out {
            Some((name, args, depth)) => {
                &&& predicate_dispatch(cfg_view(arena.nodes@, c))
                &&& roots_valid(arena.nodes@, args@)
                &&& depth > 0
                &&& cfg_view(arena.nodes@, c).stack.len() > 0
                &&& cfg_view(arena.nodes@, c).stack[0] == ckc_spec::engine::Goal::Lit(
                    Term::Comp(name@, root_terms(arena.nodes@, args@)),
                    depth as nat,
                )
            },
            None => !predicate_dispatch(cfg_view(arena.nodes@, c)),
        },
{
    if c.stack.len() == 0 {
        return None;
    }
    let front = c.stack[0];
    proof {
        assert(goal_valid(arena.nodes@, &c.stack@[0]));
        assert(cfg_view(arena.nodes@, c).stack[0] == goal_view(arena.nodes@, &front));
        reveal(predicate_dispatch);
        reveal(goal_view);
    }
    match front {
        EGoal::Lit { root, depth } => {
            if depth == 0 {
                return None;
            }
            proof {
                reveal(arena_ok);
                assert(node_ok(arena.nodes@, root as int));
                reveal(node_ok);
            }
            match &arena.nodes[root].kind {
                ENodeKind::Comp { name, child_roots, .. } => {
                    proof {
                        node_comp_model(arena.nodes@, root as int, name@, child_roots@);
                    }
                    let mut comma = Vec::new();
                    comma.push(b',');
                    let mut naf = Vec::new();
                    naf.push(b'\\');
                    naf.push(b'+');
                    proof {
                        reveal_strlit(",");
                        reveal_strlit("\\+");
                        reveal(ckc_spec::engine::comma_name);
                        reveal(ckc_spec::engine::naf_name);
                        reveal(ckc_spec::v1text::ascii);
                        assert_seqs_equal!(comma@ == ckc_spec::engine::comma_name());
                        assert_seqs_equal!(naf@ == ckc_spec::engine::naf_name());
                    }
                    if (child_roots.len() == 2 && vec_equal(name, &comma)) || (child_roots.len()
                        == 1 && vec_equal(name, &naf)) {
                        None
                    } else {
                        Some((name.clone(), child_roots.clone(), depth))
                    }
                },
                _ => None,
            }
        },
        EGoal::NafCut { .. } => None,
    }
}

pub fn step(arena: &mut ETermArena, db: &Vec<EClause>, initial: ECfg) -> (out: EStep)
    requires
        arena_ok(old(arena)),
        db_valid(old(arena).nodes@, db@),
        cfg_valid(old(arena).nodes@, &initial),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        step_valid(final(arena).nodes@, &out),
        step_base(&out, initial.base),
        step_view(final(arena).nodes@, &out) == ckc_spec::engine::step(
            db_view(old(arena).nodes@, db@),
            cfg_view(old(arena).nodes@, &initial),
        ),
{
    let ghost before = cfg_view(arena.nodes@, &initial);
    match predicate_parts(arena, &initial) {
        None => step_simple(arena, initial, Ghost(db_view(arena.nodes@, db@))),
        Some((name, args, depth)) => {
            let mut rest = initial.stack.clone();
            let unused = rest.remove(0);
            let from = initial.ci;
            proof {
                assert_seqs_equal!(goals_view(arena.nodes@, rest@) == before.stack.drop_first());
                reveal(ckc_spec::engine::step);
                reveal(predicate_dispatch);
            }
            call(arena, db, initial, name, args, depth, rest, from)
        },
    }
}

fn finish(arena: &mut ETermArena, c: ECfg, complete: bool, floor: usize) -> (out: EROut)
    requires
        arena_ok(old(arena)),
        floor <= old(arena).nodes.len(),
        arena_ok(&c.saved),
        rows_valid(c.saved.nodes@, c.rows@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.take(floor as int).is_prefix_of(final(arena).nodes@),
        rout_valid(final(arena).nodes@, &out),
        rout_view(final(arena).nodes@, &out) == (ckc_spec::engine::ROut::End {
            complete,
            rows: rows_view(c.saved.nodes@, c.rows@),
        }),
{
    crate::k2_store::truncate(arena, floor);
    let rows = crate::k2_store::copy_rows(&c.saved, arena, &c.rows);
    EROut::End { complete, rows }
}

#[verifier::rlimit(5000)]
fn run_inner(input: ETermArena, db: &Vec<EClause>, initial: ECfg, fuel: usize) -> (out: (
    ETermArena,
    EROut,
))
    requires
        arena_ok(&input),
        db_valid(input.nodes@, db@),
        cfg_valid(input.nodes@, &initial),
        initial.base == input.nodes.len(),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        rout_valid(out.0.nodes@, &out.1),
        rout_view(out.0.nodes@, &out.1) == ckc_spec::engine::run(
            db_view(input.nodes@, db@),
            cfg_view(input.nodes@, &initial),
            fuel as nat,
        ),
{
    let floor = input.nodes.len();
    let ghost base = input.nodes@;
    let ghost model_db = db_view(base, db@);
    let ghost start = cfg_view(base, &initial);
    let ghost expected = ckc_spec::engine::run(model_db, start, fuel as nat);
    let mut arena = input;
    let mut c = initial;
    let mut remaining = fuel;
    while remaining > 0
        invariant
            base == input.nodes@,
            floor == base.len(),
            c.base == floor,
            arena_ok(&arena),
            base.is_prefix_of(arena.nodes@),
            db_valid(base, db@),
            db_valid(arena.nodes@, db@),
            model_db == db_view(arena.nodes@, db@),
            model_db == db_view(base, db@),
            cfg_valid(arena.nodes@, &c),
            start == cfg_view(base, &initial),
            expected == ckc_spec::engine::run(model_db, start, fuel as nat),
            expected == ckc_spec::engine::run(
                model_db,
                cfg_view(arena.nodes@, &c),
                remaining as nat,
            ),
        decreases remaining,
    {
        let ghost before = arena.nodes@;
        let ghost model = cfg_view(before, &c);
        let next = step(&mut arena, db, c);
        proof {
            crate::k2_load::prefix_chain(base, before, arena.nodes@);
            db_models_prefix(before, arena.nodes@, db@);
            reveal(ckc_spec::engine::run);
        }
        match next {
            EStep::Next(next_c) => {
                let ghost grown = arena.nodes@;
                crate::k2_store::truncate(&mut arena, next_c.mark);
                proof {
                    cfg_compact(grown, arena.nodes@, &next_c);
                    assert(base.is_prefix_of(arena.nodes@));
                    db_models_prefix(base, arena.nodes@, db@);
                }
                c = next_c;
                remaining -= 1;
            },
            EStep::Sol => {
                crate::k2_store::truncate(&mut arena, floor);
                proof {
                    assert(arena.nodes@ == base);
                }
                return (arena, EROut::Sol);
            },
            EStep::Done(done) => {
                let complete = !done.pruned;
                let result = finish(&mut arena, done, complete, floor);
                proof {
                    assert(base.is_prefix_of(arena.nodes@));
                }
                return (arena, result);
            },
        }
    }
    proof {
        reveal(ckc_spec::engine::run);
    }
    let result = finish(&mut arena, c, false, floor);
    proof {
        assert(base.is_prefix_of(arena.nodes@));
    }
    (arena, result)
}

pub fn run(arena: &mut ETermArena, db: &Vec<EClause>, initial: ECfg, fuel: usize) -> (out: EROut)
    requires
        arena_ok(old(arena)),
        db_valid(old(arena).nodes@, db@),
        cfg_valid(old(arena).nodes@, &initial),
        initial.base == old(arena).nodes.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        rout_valid(final(arena).nodes@, &out),
        rout_view(final(arena).nodes@, &out) == ckc_spec::engine::run(
            db_view(old(arena).nodes@, db@),
            cfg_view(old(arena).nodes@, &initial),
            fuel as nat,
        ),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (mut owned, out) = run_inner(owned, db, initial, fuel);
    core::mem::swap(arena, &mut owned);
    out
}

pub fn solve(
    arena: &mut ETermArena,
    db: &Vec<EClause>,
    goal: usize,
    depth: usize,
    sol: Vec<usize>,
    collect: bool,
    fuel: usize,
) -> (out: EROut)
    requires
        root_ok(old(arena), goal),
        db_valid(old(arena).nodes@, db@),
        roots_valid(old(arena).nodes@, sol@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        rout_valid(final(arena).nodes@, &out),
        rout_view(final(arena).nodes@, &out) == ckc_spec::engine::solve(
            db_view(old(arena).nodes@, db@),
            old(arena)@[goal as int],
            depth as nat,
            root_terms(old(arena).nodes@, sol@),
            collect,
            fuel as nat,
        ),
{
    let ghost base = arena.nodes@;
    let goal_max = max_var_root(arena, goal);
    let sol_max = roots_max_var(arena, &sol);
    let maximum = max_var_merge(goal_max, sol_max);
    let fresh = ensure_var_capacity(arena, maximum);
    proof {
        arena_prefix_stable(base, arena);
        db_models_prefix(base, arena.nodes@, db@);
        roots_models_prefix(base, arena.nodes@, sol@);
    }
    let mut stack = Vec::new();
    stack.push(EGoal::Lit { root: goal, depth });
    let initial = ECfg {
        base: arena.nodes.len(),
        mark: arena.nodes.len(),
        saved: crate::k2_reject::empty_arena(),
        stack,
        sol,
        fresh,
        collect,
        ci: 0,
        pruned: false,
        alts: Vec::new(),
        rows: Vec::new(),
    };
    proof {
        assert_seqs_equal!(goals_view(arena.nodes@, initial.stack@)
            == seq![ckc_spec::engine::Goal::Lit(base[goal as int].term@, depth as nat)]);
        assert_seqs_equal!(alts_view(arena.nodes@, initial.alts@) == Seq::empty());
        assert_seqs_equal!(rows_view(initial.saved.nodes@, initial.rows@) == Seq::empty());
        reveal(ckc_spec::engine::solve);
    }
    let out = run(arena, db, initial, fuel);
    proof {
        assert(base.is_prefix_of(arena.nodes@));
    }
    out
}

pub fn head_proved(arena: &mut ETermArena, db: &Vec<EClause>, head: usize) -> (out: bool)
    requires
        root_ok(old(arena), head),
        db_valid(old(arena).nodes@, db@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ckc_spec::engine::head_proved(
            db_view(old(arena).nodes@, db@),
            old(arena)@[head as int],
        ),
{
    let mark = arena.nodes.len();
    let empty: Vec<usize> = Vec::new();
    proof {
        assert_seqs_equal!(root_terms(arena.nodes@, empty@) == Seq::empty());
    }
    let result = solve(arena, db, head, 4000, empty, false, 1000000);
    proof {
        reveal(ckc_spec::engine::head_proved);
        reveal(ckc_spec::engine::replay_depth);
        reveal(ckc_spec::engine::replay_inf);
    }
    let proved = match result {
        EROut::Sol => true,
        EROut::End { .. } => false,
    };
    crate::k2_store::truncate(arena, mark);
    proved
}

pub fn heads_proved(arena: &mut ETermArena, db: &Vec<EClause>, heads: &Vec<usize>) -> (out: bool)
    requires
        arena_ok(old(arena)),
        db_valid(old(arena).nodes@, db@),
        roots_valid(old(arena).nodes@, heads@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ckc_spec::engine::heads_proved(
            db_view(old(arena).nodes@, db@),
            root_terms(old(arena).nodes@, heads@),
        ),
{
    let ghost base = arena.nodes@;
    let mut i = 0usize;
    while i < heads.len()
        invariant
            base == old(arena).nodes@,
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            db_valid(base, db@),
            roots_valid(base, heads@),
            i <= heads@.len(),
            forall|j: int|
                0 <= j < i ==> ckc_spec::engine::head_proved(
                    db_view(base, db@),
                    #[trigger] root_terms(base, heads@)[j],
                ),
        decreases heads.len() - i,
    {
        proof {
            db_models_prefix(base, arena.nodes@, db@);
            roots_models_prefix(base, arena.nodes@, heads@);
        }
        let proved = head_proved(arena, db, heads[i]);
        proof {
            assert(base.is_prefix_of(arena.nodes@));
        }
        if !proved {
            proof {
                assert(!ckc_spec::engine::head_proved(
                    db_view(base, db@),
                    root_terms(base, heads@)[i as int],
                ));
                reveal(ckc_spec::engine::heads_proved);
            }
            return false;
        }
        i += 1;
    }
    true
}

} // verus!
