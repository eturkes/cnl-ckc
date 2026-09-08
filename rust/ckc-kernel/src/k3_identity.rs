use crate::k2_engine::{EBodyItem, EClause};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{
    body_item_valid, body_item_view, clause_valid, clause_view, root_terms, roots_models_prefix,
    roots_valid,
};
use crate::k2_output::{atom_root, comp_root, comp1, comp2, error_out, int_root};
use crate::k2_reject::empty_arena;
use crate::k2_term::{ENode, ENodeKind, EOrder, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{
    arena_ok, arena_prefix_stable, child_roots_before, child_terms, node_ok, root_ok,
};
#[cfg(verus_keep_ghost)]
use crate::k2_walk::{gid_pair_shape, selected_gid};
use ckc_spec::replay::EOut;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::{Term, ground, ground_all};
use ckc_spec::trace::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

fn role_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == id_role(arena@[root as int]),
{
    let context: &[u8] = b"context";
    let product: &[u8] = b"product";
    let witness: &[u8] = b"witness";
    proof {
        reveal_byteslit(b"context");
        reveal_strlit("context");
        reveal_byteslit(b"product");
        reveal_strlit("product");
        reveal_byteslit(b"witness");
        reveal_strlit("witness");
        reveal(ckc_spec::v1text::ascii);
        assert(context@ == ckc_spec::v1text::ascii("context"@));
        assert(product@ == ckc_spec::v1text::ascii("product"@));
        assert(witness@ == ckc_spec::v1text::ascii("witness"@));
    }
    crate::k3_front::is_atom(arena, root, context) || crate::k3_front::is_atom(arena, root, product)
        || crate::k3_front::is_atom(arena, root, witness)
}

fn positive_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == pos_int(arena@[root as int]),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Int { magnitude, negative, value, .. } => {
            let zero = slice_to_vec(b"0");
            proof {
                reveal_byteslit(b"0");
                reveal_with_fuel(ckc_spec::v1text::udec_bytes, 1);
                assert(zero@ == ckc_spec::v1text::udec_bytes(0));
            }
            let order = crate::k2_term::int_order(
                magnitude,
                *negative,
                *value,
                &zero,
                false,
                Ghost(0int),
            );
            matches!(order, EOrder::Greater)
        },
        _ => false,
    }
}

pub proof fn id_pairs_concat(left: Seq<Term>, right: Seq<Term>)
    ensures
        ckc_spec::trace::id_pairs_all(left + right) == ckc_spec::trace::id_pairs_all(left)
            + ckc_spec::trace::id_pairs_all(right),
    decreases left.len(),
{
    if left.len() > 0 {
        id_pairs_concat(left.drop_first(), right);
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
        reveal_with_fuel(ckc_spec::trace::id_pairs_all, 2);
    } else {
        assert_seqs_equal!(left + right == right);
        reveal_with_fuel(ckc_spec::trace::id_pairs_all, 1);
    }
}

proof fn id_pairs_step(terms: Seq<Term>)
    requires
        terms.len() > 0,
    ensures
        ckc_spec::trace::id_pairs_all(terms) == ckc_spec::trace::id_pairs(terms[0])
            + ckc_spec::trace::id_pairs_all(terms.drop_first()),
{
    reveal_with_fuel(ckc_spec::trace::id_pairs_all, 1);
}

fn id_children(arena: &ETermArena, root: usize, gid: &Vec<u8>) -> (out: (
    Vec<usize>,
    Option<(usize, usize)>,
))
    requires
        root_ok(arena, root),
        gid@ == ckc_spec::replay::gid_name(),
    ensures
        roots_valid(arena.nodes@, out.0@),
        forall|i: int| 0 <= i < out.0@.len() ==> out.0@[i] < root,
        root_terms(arena.nodes@, out.0@) == ckc_spec::engine::args_of(arena@[root as int]),
        out.1 matches Some((d, s)) ==> d < root && s < root && arena@[d as int] is Atom
            && arena@[s as int] is Int,
        ckc_spec::trace::id_pairs(arena@[root as int]) == selected_gid(arena.nodes@, out.1)
            + ckc_spec::trace::id_pairs_all(root_terms(arena.nodes@, out.0@)),
{
    proof {
        reveal(root_ok);
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok);
        reveal(child_roots_before);
        reveal(ckc_spec::trace::id_pairs);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let children = child_roots.clone();
            proof {
                assert_seqs_equal!(root_terms(arena.nodes@, children@)
                == child_terms(arena.nodes@, child_roots@));
            }
            if crate::k2_engine::vec_equal(name, gid) && child_roots.len() == 5 {
                let d = child_roots[1];
                let s = child_roots[2];
                proof {
                    assert(node_ok(arena.nodes@, d as int));
                    assert(node_ok(arena.nodes@, s as int));
                }
                if let ENodeKind::Atom { .. } = &arena.nodes[d].kind {
                    if let ENodeKind::Int { .. } = &arena.nodes[s].kind {
                        if role_exec(arena, child_roots[0]) && positive_exec(arena, s) {
                            return (children, Some((d, s)));
                        }
                    }
                }
            }
            (children, None)
        },
        _ => {
            let children = Vec::new();
            proof {
                assert_seqs_equal!(root_terms(arena.nodes@, children@) == Seq::empty());
            }
            (children, None)
        },
    }
}

fn ids_inner(input_arena: ETermArena, roots: &Vec<usize>) -> (out: (Vec<usize>, ETermArena))
    requires
        arena_ok(&input_arena),
        roots_valid(input_arena.nodes@, roots@),
    ensures
        arena_ok(&out.1),
        input_arena.nodes@.is_prefix_of(out.1.nodes@),
        roots_valid(out.1.nodes@, out.0@),
        forall|i: int| 0 <= i < out.0@.len() ==> gid_pair_shape(out.1@[out.0@[i] as int]),
        root_terms(out.1.nodes@, out.0@) == ckc_spec::trace::id_pairs_all(
            root_terms(input_arena.nodes@, roots@),
        ),
{
    hide(ckc_spec::trace::id_pairs_all);
    hide(ckc_spec::trace::id_pairs);
    let ghost origin = input_arena.nodes@;
    let ghost target = ckc_spec::trace::id_pairs_all(root_terms(origin, roots@));
    let mut arena = input_arena;
    let mut tasks = roots.clone();
    let mut out = Vec::new();
    let gid = vstd::slice::slice_to_vec(b"$guideline_id");
    let pair_name: &[u8] = b"-";
    proof {
        reveal_byteslit(b"$guideline_id");
        reveal_strlit("$guideline_id");
        reveal_byteslit(b"-");
        reveal_strlit("-");
        reveal(ckc_spec::v1text::ascii);
        assert(gid@ == ckc_spec::replay::gid_name());
        assert(pair_name@ == ckc_spec::v1text::ascii("-"@));
        assert_seqs_equal!(root_terms(arena.nodes@, out@) == Seq::empty());
    }
    while tasks.len() > 0
        invariant
            arena_ok(&arena),
            origin == input_arena.nodes@,
            origin.is_prefix_of(arena.nodes@),
            roots_valid(origin, roots@),
            roots_valid(origin, tasks@),
            roots_valid(arena.nodes@, out@),
            forall|i: int| 0 <= i < out@.len() ==> gid_pair_shape(arena@[out@[i] as int]),
            target == ckc_spec::trace::id_pairs_all(root_terms(origin, roots@)),
            target == root_terms(arena.nodes@, out@) + ckc_spec::trace::id_pairs_all(
                root_terms(origin, tasks@),
            ),
            gid@ == ckc_spec::replay::gid_name(),
            pair_name@ == ckc_spec::v1text::ascii("-"@),
        decreases crate::k2_engine::roots_work(origin, tasks@),
    {
        let ghost before_tasks = tasks@;
        let ghost before_nodes = arena.nodes@;
        let ghost before_out = out@;
        let current = tasks.remove(0);
        proof {
            roots_models_prefix(origin, arena.nodes@, before_tasks);
            assert(tasks@ == before_tasks.drop_first());
            assert(current == before_tasks[0]);
            assert(root_terms(origin, before_tasks).len() > 0);
            assert(root_terms(origin, before_tasks)[0] == origin[current as int].term@);
            assert_seqs_equal!(root_terms(origin, before_tasks).drop_first()
                == root_terms(origin, tasks@));
            id_pairs_step(root_terms(origin, before_tasks));
            assert(ckc_spec::trace::id_pairs_all(root_terms(origin, before_tasks))
                == ckc_spec::trace::id_pairs(origin[current as int].term@)
                + ckc_spec::trace::id_pairs_all(root_terms(origin, tasks@)));
            assert_seqs_equal!(target == root_terms(before_nodes, before_out)
                + ckc_spec::trace::id_pairs(origin[current as int].term@)
                + ckc_spec::trace::id_pairs_all(root_terms(origin, tasks@)));
        }
        let (mut children, selected) = id_children(&arena, current, &gid);
        proof {
            assert(roots_valid(origin, children@));
            roots_models_prefix(origin, arena.nodes@, children@);
            crate::k2_engine::terms_size_root_terms(origin, children@);
            crate::k2_engine::roots_work_concat(origin, children@, tasks@);
            reveal(crate::k2_engine::term_size);
            reveal_with_fuel(crate::k2_engine::roots_work, 1);
        }
        if let Some((d, s)) = selected {
            let pair = comp2(&mut arena, pair_name, d, s);
            proof {
                roots_models_prefix(before_nodes, arena.nodes@, before_out);
                crate::k2_load::prefix_chain(origin, before_nodes, arena.nodes@);
                assert(gid_pair_shape(arena@[pair as int]));
            }
            out.push(pair);
        }
        proof {
            assert_seqs_equal!(root_terms(arena.nodes@, out@)
                == root_terms(before_nodes, before_out) + selected_gid(before_nodes, selected));
            assert forall|i: int| 0 <= i < out@.len() implies gid_pair_shape(
                arena@[out@[i] as int],
            ) by {
                if i < before_out.len() {
                    assert(out@[i] == before_out[i]);
                }
            }
            id_pairs_concat(root_terms(origin, children@), root_terms(origin, tasks@));
            assert_seqs_equal!(root_terms(origin, children@ + tasks@)
                == root_terms(origin, children@) + root_terms(origin, tasks@));
            assert(target == root_terms(arena.nodes@, out@) + ckc_spec::trace::id_pairs_all(
                root_terms(origin, children@ + tasks@),
            ));
        }
        children.append(&mut tasks);
        tasks = children;
    }
    proof {
        assert_seqs_equal!(root_terms(origin, tasks@) == Seq::empty());
        reveal_with_fuel(ckc_spec::trace::id_pairs_all, 1);
    }
    (out, arena)
}

pub fn ids_exec(arena: &mut ETermArena, roots: &Vec<usize>) -> (out: Vec<usize>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        roots_valid(final(arena).nodes@, out@),
        forall|i: int| 0 <= i < out@.len() ==> gid_pair_shape(final(arena)@[out@[i] as int]),
        root_terms(final(arena).nodes@, out@) == ckc_spec::trace::id_pairs_all(
            root_terms(old(arena).nodes@, roots@),
        ),
{
    let mut owned = empty_arena();
    core::mem::swap(arena, &mut owned);
    let (out, mut owned) = ids_inner(owned, roots);
    core::mem::swap(arena, &mut owned);
    out
}

fn item_ids_exec(arena: &mut ETermArena, item: &EBodyItem) -> (out: Vec<usize>)
    requires
        arena_ok(old(arena)),
        body_item_valid(old(arena).nodes@, item),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        roots_valid(final(arena).nodes@, out@),
        forall|i: int| 0 <= i < out.len() ==> gid_pair_shape(final(arena)@[out@[i] as int]),
        root_terms(final(arena).nodes@, out@) == item_ids(body_item_view(old(arena).nodes@, item)),
{
    match item {
        EBodyItem::Pos { root } => {
            let mut roots = Vec::new();
            roots.push(*root);
            proof {
                assert_seqs_equal!(root_terms(arena.nodes@, roots@) == seq![arena@[*root as int]]);
                reveal_with_fuel(id_pairs_all, 2);
            }
            ids_exec(arena, &roots)
        },
        EBodyItem::Naf { roots } => ids_exec(arena, roots),
    }
}

fn collect_inner(mut arena: ETermArena, clause: &EClause) -> (out: (Vec<usize>, ETermArena))
    requires
        arena_ok(&arena),
        clause_valid(arena.nodes@, clause),
    ensures
        arena_ok(&out.1),
        arena.nodes@.is_prefix_of(out.1.nodes@),
        roots_valid(out.1.nodes@, out.0@),
        forall|i: int| 0 <= i < out.0.len() ==> gid_pair_shape(out.1@[out.0@[i] as int]),
        root_terms(out.1.nodes@, out.0@) == id_pairs(clause_view(arena.nodes@, clause).head)
            + clause_view(arena.nodes@, clause).body.map_values(
            |it: ckc_spec::v1text::BodyItem| item_ids(it),
        ).flatten(),
{
    let ghost origin = arena.nodes@;
    let ghost model = clause_view(origin, clause);
    let ghost parts = model.body.map_values(|it: ckc_spec::v1text::BodyItem| item_ids(it));
    let mut head = Vec::new();
    head.push(clause.head);
    proof {
        assert_seqs_equal!(root_terms(origin, head@) == seq![model.head]);
        reveal_with_fuel(id_pairs_all, 2);
    }
    let mut out = ids_exec(&mut arena, &head);
    let mut i = 0usize;
    proof {
        crate::k2_engine::clause_models_prefix(origin, arena.nodes@, clause);
        assert_seqs_equal!(parts.take(0) == Seq::empty());
        reveal_with_fuel(Seq::<_>::flatten, 1);
    }
    while i < clause.body.len()
        invariant
            arena_ok(&arena),
            origin.is_prefix_of(arena.nodes@),
            clause_valid(arena.nodes@, clause),
            clause_valid(origin, clause),
            model == clause_view(origin, clause),
            model == clause_view(arena.nodes@, clause),
            parts == model.body.map_values(|it: ckc_spec::v1text::BodyItem| item_ids(it)),
            i <= clause.body.len(),
            roots_valid(arena.nodes@, out@),
            forall|j: int| 0 <= j < out.len() ==> gid_pair_shape(arena@[out@[j] as int]),
            root_terms(arena.nodes@, out@) == id_pairs(model.head) + parts.take(i as int).flatten(),
        decreases clause.body.len() - i,
    {
        let ghost before = arena.nodes@;
        let ghost left = out@;
        let mut next = item_ids_exec(&mut arena, &clause.body[i]);
        let ghost right = next@;
        proof {
            roots_models_prefix(before, arena.nodes@, left);
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
            crate::k2_engine::clause_models_prefix(origin, arena.nodes@, clause);
        }
        out.append(&mut next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies {
                &&& out@[j] < arena.nodes.len()
                &&& gid_pair_shape(arena@[out@[j] as int])
            } by {
                if j < left.len() {
                    assert(out@[j] == left[j]);
                } else {
                    assert(out@[j] == right[j - left.len()]);
                }
            }
            assert_seqs_equal!(root_terms(arena.nodes@, out@) == root_terms(arena.nodes@, left) + root_terms(arena.nodes@, right));
            assert(root_terms(arena.nodes@, right) == parts[i as int]);
            assert_seqs_equal!(parts.take(i as int + 1) == parts.take(i as int).push(parts[i as int]));
            parts.take(i as int).lemma_flatten_push(parts[i as int]);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(parts.take(i as int) == parts);
    }
    (out, arena)
}

proof fn pair_ground(t: Term)
    requires
        gid_pair_shape(t),
    ensures
        ground(t),
{
    if let Term::Comp(_, args) = t {
        assert_seqs_equal!(args == seq![args[0], args[1]]);
        reveal_with_fuel(ground, 5);
        reveal_with_fuel(ground_all, 5);
    }
}

pub fn identity_exec(arena: &mut ETermArena, clause: &EClause) -> (out: Result<usize, EOut>)
    requires
        arena_ok(old(arena)),
        clause_valid(old(arena).nodes@, clause),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(root) ==> root_ok(final(arena), root),
        crate::k2_answers::result_view(final(arena).nodes@, out) == identity(
            clause_view(old(arena).nodes@, clause),
        ),
{
    let ghost before = arena.nodes@;
    let ghost model = clause_view(before, clause);
    let mut owned = empty_arena();
    core::mem::swap(arena, &mut owned);
    let (all, mut owned) = collect_inner(owned, clause);
    core::mem::swap(arena, &mut owned);
    proof {
        assert forall|i: int| 0 <= i < all.len() implies ground(arena@[all@[i] as int]) by {
            pair_ground(arena@[all@[i] as int]);
        }
        assert_seqs_equal!(crate::k2_sort::root_terms(arena, all@) == root_terms(arena.nodes@, all@));
    }
    let ids = crate::k2_sort::sort_unique(arena, &all);
    proof {
        crate::k2_sort::sort_unique_property(
            root_terms(arena.nodes@, all@),
            |t: Term| gid_pair_shape(t),
        );
        assert_seqs_equal!(crate::k2_sort::root_terms(arena, ids@) == root_terms(arena.nodes@, ids@));
    }
    if ids.len() == 1 {
        proof {
            assert(gid_pair_shape(arena@[ids@[0] as int]));
        }
        let args = crate::k2_engine::args_roots(arena, ids[0]);
        let name: &[u8] = b"sentence";
        proof {
            reveal_byteslit(b"sentence");
            reveal_strlit("sentence");
            reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("sentence"@));
        }
        return Ok(comp_root(arena, name, args));
    }
    let name: &[u8] = b"clause_identity";
    let none: &[u8] = b"none";
    let multiple: &[u8] = b"multiple";
    proof {
        reveal_byteslit(b"clause_identity");
        reveal_strlit("clause_identity");
        reveal_byteslit(b"none");
        reveal_strlit("none");
        reveal_byteslit(b"multiple");
        reveal_strlit("multiple");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("clause_identity"@));
        assert(none@ == ckc_spec::v1text::ascii("none"@));
        assert(multiple@ == ckc_spec::v1text::ascii("multiple"@));
    }
    let detail = if ids.len() == 0 {
        atom_root(arena, none)
    } else {
        let count = int_root(arena, ids.len());
        comp1(arena, multiple, count)
    };
    let why = comp1(arena, name, detail);
    Err(error_out(arena, why, true))
}

} // verus!
