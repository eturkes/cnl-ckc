#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid, rows_valid, rows_view};
use crate::k2_term::{
    ENode, ENodeKind, ETermArena, push_atom, push_comp, push_int, push_nil, push_var,
};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, node_ok, root_ok};
use std::collections::BTreeMap;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn truncate(arena: &mut ETermArena, mark: usize)
    requires
        arena_ok(old(arena)),
        mark <= old(arena).nodes.len(),
    ensures
        arena_ok(final(arena)),
        final(arena).nodes@ == old(arena).nodes@.take(mark as int),
{
    let ghost before = arena.nodes@;
    arena.nodes.truncate(mark);
    proof {
        assert forall|i: int| 0 <= i < arena.nodes.len() implies node_ok(arena.nodes@, i) by {
            assert(node_ok(before, i));
            reveal(node_ok);
            assert(arena.nodes@[i] == before[i]);
            match &before[i].kind {
                ENodeKind::Comp { child_roots, .. } => {
                    assert forall|j: int| 0 <= j < child_roots.len() implies child_roots@[j]
                        < arena.nodes.len() by {
                        assert(child_roots@[j] < i);
                    }
                    assert_seqs_equal!(crate::k2_term::child_terms(before, child_roots@)
                        == crate::k2_term::child_terms(arena.nodes@, child_roots@));
                },
                _ => {},
            }
        }
    }
}

fn copy_root(source: &ETermArena, input: ETermArena, root: usize) -> (out: (ETermArena, usize))
    requires
        root_ok(source, root),
        arena_ok(&input),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        root_ok(&out.0, out.1),
        out.0@[out.1 as int] == source@[root as int],
{
    let order = crate::k2_rewrite::postorder(source, root);
    let ghost base = input.nodes@;
    let mut target = input;
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut i = 0usize;
    while i < order.len()
        invariant
            root_ok(source, root),
            base == input.nodes@,
            arena_ok(&target),
            base.is_prefix_of(target.nodes@),
            i <= order.len(),
            order.len() > 0,
            order@.last() == root,
            crate::k2_rewrite::order_ok(source.nodes@, order@),
            forall|j: int| 0 <= j < i ==> map@.contains_key(order@[j]),
            forall|r: usize| #[trigger]
                map@.contains_key(r) ==> r < source.nodes.len() && map@[r] < target.nodes.len()
                    && target@[map@[r] as int] == source@[r as int],
        decreases order.len() - i,
    {
        let current = order[i];
        let ghost before = target.nodes@;
        proof {
            assert(node_ok(source.nodes@, current as int));
            reveal(node_ok);
        }
        let next = match &source.nodes[current].kind {
            ENodeKind::Var { key, spelling, .. } => push_var(&mut target, *key, spelling.clone()),
            ENodeKind::Int { spelling, magnitude, negative, value } => push_int(
                &mut target,
                spelling.clone(),
                magnitude.clone(),
                *negative,
                *value,
            ),
            ENodeKind::Nil => push_nil(&mut target),
            ENodeKind::Atom { name } => push_atom(&mut target, name.clone()),
            ENodeKind::Comp { name, child_roots, .. } => {
                proof {
                    crate::k2_engine::node_comp_model(
                        source.nodes@,
                        current as int,
                        name@,
                        child_roots@,
                    );
                    assert(child_roots@ == crate::k2_rewrite::children(source.nodes@, current));
                    assert forall|j: int| 0 <= j < child_roots.len() implies map@.contains_key(
                        child_roots@[j],
                    ) by {
                        assert(order@.take(i as int).contains(child_roots@[j]));
                        let k = choose|k: int| 0 <= k < i && order@[k] == child_roots@[j];
                    }
                }
                let (mapped, _) = crate::k2_rewrite::mapped_roots(child_roots, &map);
                proof {
                    assert forall|j: int| 0 <= j < mapped.len() implies mapped@[j]
                        < target.nodes.len() by {
                        assert(map@.contains_key(child_roots@[j]));
                        assert(mapped@[j] == map@[child_roots@[j]]);
                    }
                    assert(roots_valid(target.nodes@, mapped@));
                    assert_seqs_equal!(root_terms(target.nodes@, mapped@)
                        == root_terms(source.nodes@, child_roots@));
                }
                push_comp(&mut target, name.clone(), mapped)
            },
        };
        proof {
            crate::k2_load::prefix_chain(base, before, target.nodes@);
            assert forall|r: usize| #[trigger] map@.contains_key(r) implies r < source.nodes.len()
                && map@[r] < target.nodes.len() && target@[map@[r] as int] == source@[r as int] by {
                assert(target.nodes@[map@[r] as int] == before[map@[r] as int]);
            }
            assert(target@[next as int] == source@[current as int]);
        }
        let unused = map.insert(current, next);
        i += 1;
    }
    let result = *map.get(&root).unwrap();
    (target, result)
}

fn copy_roots_inner(source: &ETermArena, input: ETermArena, roots: &Vec<usize>) -> (out: (
    ETermArena,
    Vec<usize>,
))
    requires
        arena_ok(source),
        roots_valid(source.nodes@, roots@),
        arena_ok(&input),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        roots_valid(out.0.nodes@, out.1@),
        root_terms(out.0.nodes@, out.1@) == root_terms(source.nodes@, roots@),
{
    let ghost base = input.nodes@;
    let ghost models = root_terms(source.nodes@, roots@);
    let mut target = input;
    let mut result = Vec::new();
    let mut i = 0usize;
    while i < roots.len()
        invariant
            arena_ok(source),
            roots_valid(source.nodes@, roots@),
            base == input.nodes@,
            arena_ok(&target),
            base.is_prefix_of(target.nodes@),
            models == root_terms(source.nodes@, roots@),
            i <= roots.len(),
            result.len() == i,
            roots_valid(target.nodes@, result@),
            root_terms(target.nodes@, result@) == models.take(i as int),
        decreases roots.len() - i,
    {
        let ghost before = target.nodes@;
        let ghost prior = result@;
        let (next, root) = copy_root(source, target, roots[i]);
        target = next;
        proof {
            crate::k2_load::prefix_chain(base, before, target.nodes@);
            crate::k2_engine::roots_models_prefix(before, target.nodes@, prior);
        }
        result.push(root);
        proof {
            assert(roots_valid(target.nodes@, result@));
            assert_seqs_equal!(root_terms(target.nodes@, result@) == models.take(i as int + 1));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.take(i as int) == models);
    }
    (target, result)
}

pub fn copy_roots(source: &ETermArena, target: &mut ETermArena, roots: &Vec<usize>) -> (out: Vec<
    usize,
>)
    requires
        arena_ok(source),
        roots_valid(source.nodes@, roots@),
        arena_ok(old(target)),
    ensures
        arena_ok(final(target)),
        old(target).nodes@.is_prefix_of(final(target).nodes@),
        roots_valid(final(target).nodes@, out@),
        root_terms(final(target).nodes@, out@) == root_terms(source.nodes@, roots@),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(target, &mut owned);
    let (mut owned, roots) = copy_roots_inner(source, owned, roots);
    core::mem::swap(target, &mut owned);
    roots
}

pub proof fn rows_prefix(before: Seq<ENode>, after: Seq<ENode>, rows: Seq<Vec<usize>>)
    requires
        before.is_prefix_of(after),
        rows_valid(before, rows),
    ensures
        rows_valid(after, rows),
        rows_view(after, rows) == rows_view(before, rows),
{
    assert forall|i: int| 0 <= i < rows.len() implies roots_valid(after, rows[i]@) && root_terms(
        after,
        rows[i]@,
    ) == root_terms(before, rows[i]@) by {
        crate::k2_engine::roots_models_prefix(before, after, rows[i]@);
    }
    assert_seqs_equal!(rows_view(after, rows) == rows_view(before, rows));
}

fn copy_rows_inner(source: &ETermArena, input: ETermArena, rows: &Vec<Vec<usize>>) -> (out: (
    ETermArena,
    Vec<Vec<usize>>,
))
    requires
        arena_ok(source),
        rows_valid(source.nodes@, rows@),
        arena_ok(&input),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        rows_valid(out.0.nodes@, out.1@),
        rows_view(out.0.nodes@, out.1@) == rows_view(source.nodes@, rows@),
{
    let ghost base = input.nodes@;
    let ghost models = rows_view(source.nodes@, rows@);
    let mut target = input;
    let mut result = Vec::new();
    let mut i = 0usize;
    while i < rows.len()
        invariant
            arena_ok(source),
            rows_valid(source.nodes@, rows@),
            base == input.nodes@,
            arena_ok(&target),
            base.is_prefix_of(target.nodes@),
            models == rows_view(source.nodes@, rows@),
            i <= rows.len(),
            result.len() == i,
            rows_valid(target.nodes@, result@),
            rows_view(target.nodes@, result@) == models.take(i as int),
        decreases rows.len() - i,
    {
        let ghost before = target.nodes@;
        let ghost prior = result@;
        proof {
            assert(roots_valid(source.nodes@, rows@[i as int]@));
        }
        let copied = copy_roots(source, &mut target, &rows[i]);
        proof {
            crate::k2_load::prefix_chain(base, before, target.nodes@);
            rows_prefix(before, target.nodes@, prior);
        }
        result.push(copied);
        proof {
            assert(rows_valid(target.nodes@, result@));
            assert_seqs_equal!(rows_view(target.nodes@, result@) == models.take(i as int + 1));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.take(i as int) == models);
    }
    (target, result)
}

pub fn copy_rows(source: &ETermArena, target: &mut ETermArena, rows: &Vec<Vec<usize>>) -> (out: Vec<
    Vec<usize>,
>)
    requires
        arena_ok(source),
        rows_valid(source.nodes@, rows@),
        arena_ok(old(target)),
    ensures
        arena_ok(final(target)),
        old(target).nodes@.is_prefix_of(final(target).nodes@),
        rows_valid(final(target).nodes@, out@),
        rows_view(final(target).nodes@, out@) == rows_view(source.nodes@, rows@),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(target, &mut owned);
    let (mut owned, rows) = copy_rows_inner(source, owned, rows);
    core::mem::swap(target, &mut owned);
    rows
}

} // verus!
