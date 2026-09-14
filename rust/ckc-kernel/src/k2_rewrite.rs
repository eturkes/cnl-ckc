#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid};
use crate::k2_term::{ENode, ENodeKind, ETermArena, push_comp};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, node_ok, root_ok};
use ckc_spec::term::Term;
use std::collections::{BTreeMap, BTreeSet};
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn children(nodes: Seq<ENode>, root: usize) -> Seq<usize> {
    match &nodes[root as int].kind {
        ENodeKind::Comp { child_roots, .. } => child_roots@,
        _ => Seq::empty(),
    }
}

pub proof fn nvars_item(ts: Seq<Term>, i: int)
    requires
        0 <= i < ts.len(),
    ensures
        ckc_spec::engine::nvars(ts[i]) <= ckc_spec::engine::nvars_all(ts),
    decreases i,
{
    reveal_with_fuel(ckc_spec::engine::nvars_all, 1);
    if i > 0 {
        nvars_item(ts.drop_first(), i - 1);
    }
}

pub fn child_roots(arena: &ETermArena, root: usize) -> (out: Vec<usize>)
    requires
        root_ok(arena, root),
    ensures
        out@ == children(arena.nodes@, root),
        forall|i: int| 0 <= i < out.len() ==> out@[i] < root,
        forall|i: int|
            0 <= i < out.len() ==> ckc_spec::engine::nvars(arena@[out@[i] as int])
                <= ckc_spec::engine::nvars(arena@[root as int]),
{
    proof {
        reveal(root_ok);
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { child_roots, .. } => {
            proof {
                assert forall|i: int| 0 <= i < child_roots.len() implies ckc_spec::engine::nvars(
                    arena@[child_roots@[i] as int],
                ) <= ckc_spec::engine::nvars(arena@[root as int]) by {
                    nvars_item(crate::k2_term::child_terms(arena.nodes@, child_roots@), i);
                }
            }
            child_roots.clone()
        },
        _ => Vec::new(),
    }
}

fn missing_child(roots: &Vec<usize>, seen: &BTreeSet<usize>) -> (out: Option<usize>)
    ensures
        match out {
            Some(root) => roots@.contains(root) && !seen@.contains(root),
            None => forall|i: int| 0 <= i < roots.len() ==> seen@.contains(roots@[i]),
        },
{
    let mut i = 0usize;
    while i < roots.len()
        invariant
            i <= roots.len(),
            forall|j: int| 0 <= j < i ==> seen@.contains(roots@[j]),
        decreases roots.len() - i,
    {
        if !seen.contains(&roots[i]) {
            return Some(roots[i]);
        }
        i += 1;
    }
    None
}

pub open spec fn order_ok(nodes: Seq<ENode>, order: Seq<usize>) -> bool {
    &&& roots_valid(nodes, order)
    &&& forall|i: int, j: int|
        0 <= i < order.len() && 0 <= j < children(nodes, order[i]).len() ==> order.take(i).contains(
            #[trigger] children(nodes, order[i])[j],
        )
}

// The worklist follows only descendants; unrelated arena prefixes cost nothing.
pub fn postorder(arena: &ETermArena, root: usize) -> (out: Vec<usize>)
    requires
        root_ok(arena, root),
    ensures
        out.len() > 0,
        out@.last() == root,
        order_ok(arena.nodes@, out@),
        forall|i: int|
            0 <= i < out.len() ==> ckc_spec::engine::nvars(arena@[out@[i] as int])
                <= ckc_spec::engine::nvars(arena@[root as int]),
{
    let mut pending = Vec::new();
    pending.push(root);
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    let node_count = arena.nodes.len();
    let ghost mut remaining = Set::<int>::range(0, arena.nodes@.len() as int);
    proof {
        broadcast use vstd::set_lib::range_set_properties;

    }
    while pending.len() > 0
        invariant
            root_ok(arena, root),
            order_ok(arena.nodes@, out@),
            seen@ == out@.to_set(),
            node_count == arena.nodes@.len(),
            forall|r: int|
                0 <= r < arena.nodes@.len() ==> (remaining.contains(r) <==> !seen@.contains(
                    r as usize,
                )),
            roots_valid(arena.nodes@, pending@),
            pending.len() > 0 ==> pending@[0] == root,
            pending.len() == 0 ==> out.len() > 0 && out@.last() == root,
            forall|i: int| 0 <= i < pending.len() ==> !seen@.contains(pending@[i]),
            forall|i: int, j: int| 0 <= i < j < pending.len() ==> pending@[j] < pending@[i],
            forall|i: int|
                0 <= i < pending.len() ==> ckc_spec::engine::nvars(arena@[pending@[i] as int])
                    <= ckc_spec::engine::nvars(arena@[root as int]),
            forall|i: int|
                0 <= i < out.len() ==> ckc_spec::engine::nvars(arena@[out@[i] as int])
                    <= ckc_spec::engine::nvars(arena@[root as int]),
        decreases
                remaining.len(),
                if pending.len() > 0 {
                    pending@.last() as int + 1
                } else {
                    0int
                },
    {
        let current = pending[pending.len() - 1];
        let roots = child_roots(arena, current);
        match missing_child(&roots, &seen) {
            Some(child) => {
                proof {
                    let j = choose|j: int| 0 <= j < roots.len() && roots@[j] == child;
                    assert(child < current);
                    assert(ckc_spec::engine::nvars(arena@[child as int]) <= ckc_spec::engine::nvars(
                        arena@[current as int],
                    ));
                }
                pending.push(child);
            },
            None => {
                let ghost before = out@;
                let ghost prior_pending = pending@;
                let ghost prior_remaining = remaining;
                let ghost prior_seen = seen@;
                proof {
                    assert(remaining.contains(current as int));
                    remaining = remaining.remove(current as int);
                }
                let unused = pending.pop();
                out.push(current);
                let unused = seen.insert(current);
                proof {
                    before.lemma_push_to_set_commute(current);
                    assert(out@.to_set() == before.to_set().insert(current));
                    assert forall|r: int| 0 <= r < arena.nodes@.len() implies (remaining.contains(r)
                        <==> !seen@.contains(r as usize)) by {
                        assert((r as usize) as int == r);
                        assert(prior_remaining.contains(r) <==> !prior_seen.contains(r as usize));
                    }
                    assert forall|i: int, j: int|
                        0 <= i < out.len() && 0 <= j < children(
                            arena.nodes@,
                            out@[i],
                        ).len() implies out@.take(i).contains(
                        #[trigger] children(arena.nodes@, out@[i])[j],
                    ) by {
                        if i < before.len() {
                            assert(out@[i] == before[i]);
                            assert(out@.take(i) == before.take(i));
                        } else {
                            assert(i == before.len());
                            assert(out@.take(i) == before);
                        }
                    }
                    assert forall|i: int| 0 <= i < pending.len() implies !seen@.contains(
                        pending@[i],
                    ) by {
                        assert(pending@[i] == prior_pending[i]);
                        assert(current < prior_pending[i]);
                    }
                }
            },
        }
    }
    out
}

pub open spec fn apply(t: Term, shifted: bool, x: nat, value: Term) -> Term {
    if shifted {
        ckc_spec::engine::shift(t, x)
    } else {
        ckc_spec::engine::subst(t, x, value)
    }
}

proof fn apply_all(ts: Seq<Term>, shifted: bool, x: nat, value: Term)
    ensures
        ts.map_values(|t: Term| apply(t, shifted, x, value)) == if shifted {
            ckc_spec::engine::shift_all(ts, x)
        } else {
            ckc_spec::engine::subst_all(ts, x, value)
        },
    decreases ts.len(),
{
    if ts.len() > 0 {
        apply_all(ts.drop_first(), shifted, x, value);
        assert_seqs_equal!(ts.map_values(|t: Term| apply(t, shifted, x, value))
            == seq![apply(ts[0], shifted, x, value)]
                + ts.drop_first().map_values(|t: Term| apply(t, shifted, x, value)));
    }
    reveal_with_fuel(ckc_spec::engine::shift_all, 1);
    reveal_with_fuel(ckc_spec::engine::subst_all, 1);
}

pub fn mapped_roots(roots: &Vec<usize>, map: &BTreeMap<usize, usize>) -> (out: (Vec<usize>, bool))
    requires
        forall|i: int| 0 <= i < roots.len() ==> map@.contains_key(roots@[i]),
    ensures
        out.0.len() == roots.len(),
        forall|i: int| 0 <= i < roots.len() ==> out.0@[i] == map@[roots@[i]],
        out.1 <==> out.0@ == roots@,
{
    let mut out = Vec::new();
    let mut same = true;
    let mut i = 0usize;
    while i < roots.len()
        invariant
            i <= roots.len(),
            out.len() == i,
            forall|j: int| 0 <= j < roots.len() ==> map@.contains_key(roots@[j]),
            forall|j: int| 0 <= j < i ==> out@[j] == map@[roots@[j]],
            same <==> forall|j: int| 0 <= j < i ==> out@[j] == roots@[j],
        decreases roots.len() - i,
    {
        let next = *map.get(&roots[i]).unwrap();
        let ghost was_same = same;
        if next != roots[i] {
            same = false;
        }
        let ghost before = out@;
        out.push(next);
        proof {
            if !was_same {
                let j = choose|j: int| 0 <= j < i && before[j] != roots@[j];
                assert(out@[j] != roots@[j]);
            }
        }
        i += 1;
    }
    proof {
        if same {
            assert_seqs_equal!(out@ == roots@);
        } else {
            let j = choose|j: int| 0 <= j < i && out@[j] != roots@[j];
            assert(out@ != roots@);
        }
    }
    (out, same)
}

pub fn rewrite_root(
    input_arena: ETermArena,
    root: usize,
    shifted: bool,
    x: usize,
    replacement: usize,
    vars: &Vec<usize>,
) -> (out: (ETermArena, usize))
    requires
        root_ok(&input_arena, root),
        replacement < input_arena.nodes.len(),
        shifted ==> roots_valid(input_arena.nodes@, vars@),
        shifted ==> ckc_spec::engine::nvars(input_arena@[root as int]) <= vars.len(),
        shifted ==> forall|i: int|
            0 <= i < vars.len() ==> input_arena@[vars@[i] as int] == Term::Var(x as nat + i as nat),
    ensures
        arena_ok(&out.0),
        input_arena.nodes@.is_prefix_of(out.0.nodes@),
        root_ok(&out.0, out.1),
        out.0@[out.1 as int] == apply(
            input_arena@[root as int],
            shifted,
            x as nat,
            input_arena@[replacement as int],
        ),
{
    let ghost base = input_arena.nodes@;
    let ghost value = input_arena@[replacement as int];
    let order = postorder(&input_arena, root);
    let mut arena = input_arena;
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut i = 0usize;
    while i < order.len()
        invariant
            base == input_arena.nodes@,
            value == base[replacement as int].term@,
            arena_ok(&arena),
            base.is_prefix_of(arena.nodes@),
            root < base.len(),
            replacement < base.len(),
            i <= order.len(),
            order.len() > 0,
            order@.last() == root,
            order_ok(base, order@),
            shifted ==> roots_valid(base, vars@),
            shifted ==> forall|j: int|
                0 <= j < vars.len() ==> base[vars@[j] as int].term@ == Term::Var(
                    x as nat + j as nat,
                ),
            shifted ==> forall|j: int|
                0 <= j < order.len() ==> ckc_spec::engine::nvars(base[order@[j] as int].term@)
                    <= vars.len(),
            forall|j: int| 0 <= j < i ==> map@.contains_key(order@[j]),
            forall|r: usize|
                map@.contains_key(r) ==> r < base.len() && map@[r] < arena.nodes.len()
                    && arena@[map@[r] as int] == apply(
                    base[r as int].term@,
                    shifted,
                    x as nat,
                    value,
                ),
        decreases order.len() - i,
    {
        let current = order[i];
        let ghost before = arena.nodes@;
        proof {
            assert(node_ok(arena.nodes@, current as int));
            reveal(node_ok);
            assert(base[current as int] == arena.nodes@[current as int]);
        }
        let next = match &arena.nodes[current].kind {
            ENodeKind::Var { key, .. } => {
                proof {
                    reveal(apply);
                    reveal(ckc_spec::engine::shift);
                    reveal(ckc_spec::engine::subst);
                }
                let result = if shifted {
                    proof {
                        assert(base[current as int].term@ == Term::Var(*key as nat));
                        assert(ckc_spec::engine::nvars(base[current as int].term@) <= vars.len());
                    }
                    vars[*key]
                } else if *key == x {
                    replacement
                } else {
                    current
                };
                proof {
                    assert(arena@[result as int] == apply(
                        base[current as int].term@,
                        shifted,
                        x as nat,
                        value,
                    ));
                }
                result
            },
            ENodeKind::Comp { name, child_roots, .. } => {
                let name = name.clone();
                let roots = child_roots.clone();
                proof {
                    crate::k2_engine::node_comp_model(arena.nodes@, current as int, name@, roots@);
                    assert_seqs_equal!(root_terms(base, roots@) == crate::k2_term::child_terms(arena.nodes@, roots@));
                    assert(base[current as int].term@ == Term::Comp(
                        name@,
                        root_terms(base, roots@),
                    ));
                    assert(roots@ == children(base, current));
                    assert forall|j: int| 0 <= j < roots.len() implies map@.contains_key(
                        roots@[j],
                    ) by {
                        assert(order@.take(i as int).contains(roots@[j]));
                        let k = choose|k: int| 0 <= k < i && order@[k] == roots@[j];
                    }
                }
                let (mapped, same) = mapped_roots(&roots, &map);
                proof {
                    assert(roots_valid(arena.nodes@, mapped@));
                    assert_seqs_equal!(root_terms(arena.nodes@, mapped@)
                        == root_terms(base, roots@).map_values(|t: Term| apply(t, shifted, x as nat, value)));
                    apply_all(root_terms(base, roots@), shifted, x as nat, value);
                    reveal(apply);
                    reveal(ckc_spec::engine::shift);
                    reveal(ckc_spec::engine::subst);
                }
                let ghost expected = apply(base[current as int].term@, shifted, x as nat, value);
                proof {
                    assert(base[current as int].term@ == Term::Comp(
                        name@,
                        root_terms(base, roots@),
                    ));
                    assert(Term::Comp(name@, root_terms(arena.nodes@, mapped@)) == expected);
                }
                let result = if same {
                    current
                } else {
                    push_comp(&mut arena, name, mapped)
                };
                proof {
                    assert(arena@[result as int] == expected);
                }
                result
            },
            _ => {
                proof {
                    reveal(apply);
                    reveal(ckc_spec::engine::shift);
                    reveal(ckc_spec::engine::subst);
                }
                current
            },
        };
        proof {
            crate::k2_load::prefix_chain(base, before, arena.nodes@);
            assert forall|r: usize| map@.contains_key(r) implies r < base.len() && map@[r]
                < arena.nodes.len() && arena@[map@[r] as int] == apply(
                base[r as int].term@,
                shifted,
                x as nat,
                value,
            ) by {
                assert(before[map@[r] as int] == arena.nodes@[map@[r] as int]);
            }
            assert(next < arena.nodes.len());
            assert(arena@[next as int] == apply(
                base[current as int].term@,
                shifted,
                x as nat,
                value,
            ));
        }
        let unused = map.insert(current, next);
        i += 1;
    }
    let result = *map.get(&root).unwrap();
    (arena, result)
}

} // verus!
