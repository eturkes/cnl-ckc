use crate::k2_engine::{EBodyItem, EClause, max_var_root};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{max_var_count, root_terms, roots_models_prefix, roots_valid};
use crate::k2_output::comp2;
use crate::k2_reject::empty_arena;
use crate::k2_term::{push_nil, ECompForm, ENode, ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, arena_prefix_stable, child_roots_before, child_terms,
    comp_form_ok, node_ok, root_ok};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::{ground, ground_all, Term};
use vstd::prelude::*;
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;

verus! {

proof fn ground_nvars(term: Term)
    ensures ground(term) == (ckc_spec::engine::nvars(term) == 0),
    decreases term, 0int,
{
    reveal(ground);
    reveal(ckc_spec::engine::nvars);
    if let Term::Comp(_, args) = term { grounds_nvars(args); }
}

proof fn grounds_nvars(terms: Seq<Term>)
    ensures ground_all(terms) == (ckc_spec::engine::nvars_all(terms) == 0),
    decreases terms, 0int,
{
    reveal_with_fuel(ground_all, 2);
    reveal_with_fuel(ckc_spec::engine::nvars_all, 2);
    if terms.len() > 0 {
        ground_nvars(terms[0]);
        grounds_nvars(terms.drop_first());
    }
}

proof fn ground_all_at(terms: Seq<Term>, i: int)
    requires ground_all(terms), 0 <= i < terms.len(),
    ensures ground(terms[i]),
    decreases terms.len(),
{
    reveal_with_fuel(ground_all, 1);
    if i > 0 {
        assert(terms.drop_first()[i - 1] == terms[i]);
        ground_all_at(terms.drop_first(), i - 1);
    }
}

pub proof fn ground_arg(term: Term, i: int)
    requires ground(term), 0 <= i < ckc_spec::engine::args_of(term).len(),
    ensures ground(ckc_spec::engine::args_of(term)[i]),
{
    reveal(ground);
    ground_all_at(ckc_spec::engine::args_of(term), i);
}

pub fn ground_root(arena: &ETermArena, root: usize) -> (out: bool)
    requires root_ok(arena, root),
    ensures out == ground(arena@[root as int]),
{
    proof { ground_nvars(arena@[root as int]); }
    max_var_root(arena, root).is_none()
}

pub open spec fn list_view(nodes: Seq<ENode>, items: Option<Vec<usize>>) -> Option<Seq<Term>> {
    match items { Some(roots) => Some(root_terms(nodes, roots@)), None => None }
}

pub open spec fn list_prefix(prefix: Seq<Term>, rest: Option<Seq<Term>>) -> Option<Seq<Term>> {
    match rest { Some(terms) => Some(prefix + terms), None => None }
}

pub fn list_items_exec(arena: &ETermArena, root: usize) -> (out: Option<Vec<usize>>)
    requires root_ok(arena, root),
    ensures
        out matches Some(roots) ==> roots_valid(arena.nodes@, roots@),
        list_view(arena.nodes@, out) == ckc_spec::answers::list_items(arena@[root as int]),
{
    let ghost model = arena@[root as int];
    let mut current = root;
    let mut out = Vec::new();
    loop
        invariant
            root_ok(arena, current),
            root_ok(arena, root),
            model == arena@[root as int],
            roots_valid(arena.nodes@, out@),
            ckc_spec::answers::list_items(model) == list_prefix(root_terms(arena.nodes@, out@),
                ckc_spec::answers::list_items(arena@[current as int])),
        decreases current,
    {
        proof {
            reveal(root_ok);
            reveal(arena_ok);
            assert(node_ok(arena.nodes@, current as int));
            reveal(node_ok);
        }
        match &arena.nodes[current].kind {
            ENodeKind::Nil => return Some(out),
            ENodeKind::Comp { name, child_roots, form } => {
                proof { reveal(comp_form_ok); }
                if let ECompForm::Cons = form {
                    let head = child_roots[0];
                    let tail = child_roots[1];
                    let ghost previous = out@;
                    proof {
                        reveal(child_roots_before);
                        reveal(child_terms);
                        reveal_with_fuel(ckc_spec::answers::list_items, 1);
                        assert_seqs_equal!(child_terms(arena.nodes@, child_roots@)
                            == seq![arena@[head as int], arena@[tail as int]]);
                        assert(arena@[current as int] == Term::Comp(ckc_spec::v1text::cons_name(),
                            seq![arena@[head as int], arena@[tail as int]]));
                    }
                    out.push(head);
                    proof {
                        assert_seqs_equal!(root_terms(arena.nodes@, out@)
                            == root_terms(arena.nodes@, previous).push(arena@[head as int]));
                    }
                    current = tail;
                } else {
                    return None;
                }
            },
            _ => return None,
        }
    }
}

fn list_root_inner(mut arena: ETermArena, roots: &Vec<usize>) -> (out: (usize, ETermArena))
    requires arena_ok(&arena), roots_valid(arena.nodes@, roots@),
    ensures
        arena_ok(&out.1),
        arena.nodes@.is_prefix_of(out.1.nodes@),
        root_ok(&out.1, out.0),
        out.1@[out.0 as int] == ckc_spec::engine::list_term(root_terms(arena.nodes@, roots@)),
{
    let ghost base = arena.nodes@;
    let ghost models = root_terms(base, roots@);
    let mut current = push_nil(&mut arena);
    let mut i = roots.len();
    let name: &[u8] = b"[|]";
    proof {
        reveal_byteslit(b"[|]");
        reveal(ckc_spec::v1text::cons_name);
        assert(name@ == ckc_spec::v1text::cons_name());
        roots_models_prefix(base, arena.nodes@, roots@);
    }
    while i > 0
        invariant
            arena_ok(&arena),
            base.is_prefix_of(arena.nodes@),
            roots_valid(base, roots@),
            roots_valid(arena.nodes@, roots@),
            models == root_terms(base, roots@),
            models == root_terms(arena.nodes@, roots@),
            i <= roots@.len(),
            root_ok(&arena, current),
            arena@[current as int] == ckc_spec::engine::list_term(models.skip(i as int)),
            name@ == ckc_spec::v1text::cons_name(),
        decreases i,
    {
        i -= 1;
        let ghost before = arena.nodes@;
        current = comp2(&mut arena, name, roots[i], current);
        proof {
            roots_models_prefix(before, arena.nodes@, roots@);
            assert(models.skip(i as int).drop_first() == models.skip(i as int + 1));
            reveal_with_fuel(ckc_spec::engine::list_term, 1);
        }
    }
    proof { assert(models.skip(0) == models); }
    (current, arena)
}

pub fn list_root(arena: &mut ETermArena, roots: &Vec<usize>) -> (root: usize)
    requires arena_ok(old(arena)), roots_valid(old(arena).nodes@, roots@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == ckc_spec::engine::list_term(root_terms(old(arena).nodes@, roots@)),
{
    let mut working = empty_arena();
    core::mem::swap(arena, &mut working);
    let (root, mut result) = list_root_inner(working, roots);
    core::mem::swap(arena, &mut result);
    root
}

pub proof fn gid_pairs_concat(left: Seq<Term>, right: Seq<Term>)
    ensures ckc_spec::replay::gid_pairs_all(left + right)
        == ckc_spec::replay::gid_pairs_all(left) + ckc_spec::replay::gid_pairs_all(right),
    decreases left.len(),
{
    if left.len() > 0 {
        gid_pairs_concat(left.drop_first(), right);
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
        reveal_with_fuel(ckc_spec::replay::gid_pairs_all, 2);
    } else {
        assert_seqs_equal!(left + right == right);
        reveal_with_fuel(ckc_spec::replay::gid_pairs_all, 1);
    }
}

fn body_walk_roots(arena: &ETermArena, item: &EBodyItem) -> (out: Vec<usize>)
    requires arena_ok(arena), crate::k2_engine::body_item_valid(arena.nodes@, item),
    ensures
        roots_valid(arena.nodes@, out@),
        ckc_spec::replay::gid_pairs_all(root_terms(arena.nodes@, out@))
            == ckc_spec::replay::item_gids(crate::k2_engine::body_item_view(arena.nodes@, item)),
{
    match item {
        EBodyItem::Pos { root } => {
            let mut out = Vec::new();
            out.push(*root);
            proof {
                assert_seqs_equal!(root_terms(arena.nodes@, out@) == seq![arena@[*root as int]]);
                reveal_with_fuel(ckc_spec::replay::gid_pairs_all, 2);
            }
            out
        },
        EBodyItem::Naf { roots } => roots.clone(),
    }
}

pub fn clause_walk_roots(arena: &ETermArena, clause: &EClause) -> (out: Vec<usize>)
    requires arena_ok(arena), crate::k2_engine::clause_valid(arena.nodes@, clause),
    ensures
        roots_valid(arena.nodes@, out@),
        ckc_spec::replay::gid_pairs_all(root_terms(arena.nodes@, out@))
            == ckc_spec::replay::clause_gids(crate::k2_engine::clause_view(arena.nodes@, clause)),
{
    let ghost model = crate::k2_engine::clause_view(arena.nodes@, clause);
    let ghost parts = model.body.map_values(|item: ckc_spec::v1text::BodyItem| ckc_spec::replay::item_gids(item));
    let mut out = Vec::new();
    out.push(clause.head);
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(root_terms(arena.nodes@, out@) == seq![model.head]);
        assert_seqs_equal!(parts.take(0) == Seq::empty());
        reveal_with_fuel(ckc_spec::replay::gid_pairs_all, 2);
    }
    while i < clause.body.len()
        invariant
            arena_ok(arena), crate::k2_engine::clause_valid(arena.nodes@, clause),
            model == crate::k2_engine::clause_view(arena.nodes@, clause),
            parts == model.body.map_values(|item: ckc_spec::v1text::BodyItem| ckc_spec::replay::item_gids(item)),
            i <= clause.body.len(), parts.len() == clause.body.len(),
            roots_valid(arena.nodes@, out@),
            ckc_spec::replay::gid_pairs_all(root_terms(arena.nodes@, out@))
                == ckc_spec::replay::gid_pairs(model.head) + parts.take(i as int).flatten(),
        decreases clause.body.len() - i,
    {
        proof {
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &clause.body@[i as int]));
            assert(model.body[i as int] == crate::k2_engine::body_item_view(arena.nodes@, &clause.body@[i as int]));
        }
        let mut next = body_walk_roots(arena, &clause.body[i]);
        let ghost left = out@;
        let ghost right = next@;
        out.append(&mut next);
        proof {
            assert forall|j: int| 0 <= j < out@.len() implies out@[j] < arena.nodes@.len() by {
                if j < left.len() { assert(out@[j] == left[j]); }
                else { assert(out@[j] == right[j - left.len()]); }
            }
            assert_seqs_equal!(root_terms(arena.nodes@, out@)
                == root_terms(arena.nodes@, left) + root_terms(arena.nodes@, right), j => {
                if j < left.len() { assert(out@[j] == left[j]); }
                else { assert(out@[j] == right[j - left.len()]); }
            });
            gid_pairs_concat(root_terms(arena.nodes@, left), root_terms(arena.nodes@, right));
            assert_seqs_equal!(parts.take(i as int + 1) == parts.take(i as int).push(parts[i as int]));
            parts.take(i as int).lemma_flatten_push(parts[i as int]);
        }
        i += 1;
    }
    proof { assert_seqs_equal!(parts.take(i as int) == parts); }
    out
}

pub proof fn list_items_of_list(terms: Seq<Term>)
    ensures ckc_spec::answers::list_items(ckc_spec::engine::list_term(terms)) == Some(terms),
    decreases terms.len(),
{
    reveal_with_fuel(ckc_spec::engine::list_term, 1);
    reveal_with_fuel(ckc_spec::answers::list_items, 1);
    if terms.len() > 0 {
        list_items_of_list(terms.drop_first());
        assert_seqs_equal!(seq![terms[0]] + terms.drop_first() == terms);
    }
}

pub proof fn list_items_reconstruct(term: Term)
    ensures ckc_spec::answers::list_items(term) matches Some(terms)
        ==> ckc_spec::engine::list_term(terms) == term,
    decreases term,
{
    reveal_with_fuel(ckc_spec::answers::list_items, 1);
    reveal_with_fuel(ckc_spec::engine::list_term, 1);
    if let Term::Comp(name, args) = term {
        if name == ckc_spec::v1text::cons_name() && args.len() == 2 {
            list_items_reconstruct(args[1]);
            if let Some(rest) = ckc_spec::answers::list_items(args[1]) {
                assert_seqs_equal!((seq![args[0]] + rest).drop_first() == rest);
                assert_seqs_equal!(args == seq![args[0], args[1]]);
                assert(ckc_spec::engine::list_term(seq![args[0]] + rest) == term);
            }
        }
    }
}

proof fn gid_pairs_step(terms: Seq<Term>)
    requires terms.len() > 0,
    ensures ckc_spec::replay::gid_pairs_all(terms)
        == ckc_spec::replay::gid_pairs(terms[0])
            + ckc_spec::replay::gid_pairs_all(terms.drop_first()),
{
    reveal_with_fuel(ckc_spec::replay::gid_pairs_all, 1);
}

pub open spec fn selected_gid(nodes: Seq<ENode>, selected: Option<(usize, usize)>) -> Seq<Term> {
    match selected {
        Some((d, s)) => seq![ckc_spec::replay::pair(nodes[d as int].term@, nodes[s as int].term@)],
        None => Seq::empty(),
    }
}

pub open spec fn gid_pair_shape(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => name == ckc_spec::v1text::ascii("-"@)
            && args.len() == 2 && args[0] is Atom && args[1] is Int,
        _ => false,
    }
}

fn gid_children(arena: &ETermArena, root: usize, gid: &Vec<u8>)
    -> (out: (Vec<usize>, Option<(usize, usize)>))
    requires root_ok(arena, root), gid@ == ckc_spec::replay::gid_name(),
    ensures
        roots_valid(arena.nodes@, out.0@),
        forall|i: int| 0 <= i < out.0@.len() ==> out.0@[i] < root,
        root_terms(arena.nodes@, out.0@) == ckc_spec::engine::args_of(arena@[root as int]),
        out.1 matches Some((d, s)) ==> d < root && s < root
            && arena@[d as int] is Atom && arena@[s as int] is Int,
        ckc_spec::replay::gid_pairs(arena@[root as int])
            == selected_gid(arena.nodes@, out.1)
                + ckc_spec::replay::gid_pairs_all(root_terms(arena.nodes@, out.0@)),
{
    proof {
        reveal(root_ok); reveal(arena_ok);
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok); reveal(child_roots_before);
        reveal(ckc_spec::replay::gid_pairs);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let children = child_roots.clone();
            proof { assert_seqs_equal!(root_terms(arena.nodes@, children@)
                == child_terms(arena.nodes@, child_roots@)); }
            if crate::k2_engine::vec_equal(name, gid) && child_roots.len() == 5 {
                let d = child_roots[1];
                let s = child_roots[2];
                proof {
                    assert(node_ok(arena.nodes@, d as int));
                    assert(node_ok(arena.nodes@, s as int));
                }
                if let ENodeKind::Atom { .. } = &arena.nodes[d].kind {
                    if let ENodeKind::Int { .. } = &arena.nodes[s].kind {
                        return (children, Some((d, s)));
                    }
                }
            }
            (children, None)
        },
        _ => {
            let children = Vec::new();
            proof { assert_seqs_equal!(root_terms(arena.nodes@, children@) == Seq::empty()); }
            (children, None)
        },
    }
}

fn gid_pairs_inner(input_arena: ETermArena, roots: &Vec<usize>) -> (out: (Vec<usize>, ETermArena))
    requires arena_ok(&input_arena), roots_valid(input_arena.nodes@, roots@),
    ensures
        arena_ok(&out.1), input_arena.nodes@.is_prefix_of(out.1.nodes@),
        roots_valid(out.1.nodes@, out.0@),
        forall|i: int| 0 <= i < out.0@.len() ==> gid_pair_shape(out.1@[out.0@[i] as int]),
        root_terms(out.1.nodes@, out.0@)
            == ckc_spec::replay::gid_pairs_all(root_terms(input_arena.nodes@, roots@)),
{
    hide(ckc_spec::replay::gid_pairs_all);
    hide(ckc_spec::replay::gid_pairs);
    let ghost origin = input_arena.nodes@;
    let ghost target = ckc_spec::replay::gid_pairs_all(root_terms(origin, roots@));
    let mut arena = input_arena;
    let mut tasks = roots.clone();
    let mut out = Vec::new();
    let gid = vstd::slice::slice_to_vec(b"$guideline_id");
    let pair_name: &[u8] = b"-";
    proof {
        reveal_byteslit(b"$guideline_id"); reveal_strlit("$guideline_id");
        reveal_byteslit(b"-"); reveal_strlit("-");
        reveal(ckc_spec::v1text::ascii);
        assert(gid@ == ckc_spec::replay::gid_name());
        assert(pair_name@ == ckc_spec::v1text::ascii("-"@));
        assert_seqs_equal!(root_terms(arena.nodes@, out@) == Seq::empty());
    }
    while tasks.len() > 0
        invariant
            arena_ok(&arena), origin == input_arena.nodes@, origin.is_prefix_of(arena.nodes@),
            roots_valid(origin, roots@), roots_valid(origin, tasks@),
            roots_valid(arena.nodes@, out@),
            forall|i: int| 0 <= i < out@.len() ==> gid_pair_shape(arena@[out@[i] as int]),
            target == ckc_spec::replay::gid_pairs_all(root_terms(origin, roots@)),
            target == root_terms(arena.nodes@, out@)
                + ckc_spec::replay::gid_pairs_all(root_terms(origin, tasks@)),
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
            gid_pairs_step(root_terms(origin, before_tasks));
            assert(ckc_spec::replay::gid_pairs_all(root_terms(origin, before_tasks))
                == ckc_spec::replay::gid_pairs(origin[current as int].term@)
                    + ckc_spec::replay::gid_pairs_all(root_terms(origin, tasks@)));
            assert_seqs_equal!(target == root_terms(before_nodes, before_out)
                + ckc_spec::replay::gid_pairs(origin[current as int].term@)
                + ckc_spec::replay::gid_pairs_all(root_terms(origin, tasks@)));
        }
        let (mut children, selected) = gid_children(&arena, current, &gid);
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
            assert forall|i: int| 0 <= i < out@.len()
                implies gid_pair_shape(arena@[out@[i] as int]) by {
                if i < before_out.len() { assert(out@[i] == before_out[i]); }
            }
            gid_pairs_concat(root_terms(origin, children@), root_terms(origin, tasks@));
            assert_seqs_equal!(root_terms(origin, children@ + tasks@)
                == root_terms(origin, children@) + root_terms(origin, tasks@));
            assert(target == root_terms(arena.nodes@, out@)
                + ckc_spec::replay::gid_pairs_all(root_terms(origin, children@ + tasks@)));
        }
        children.append(&mut tasks);
        tasks = children;
    }
    proof {
        assert_seqs_equal!(root_terms(origin, tasks@) == Seq::empty());
        reveal_with_fuel(ckc_spec::replay::gid_pairs_all, 1);
    }
    (out, arena)
}

pub fn gid_pairs_all_exec(arena: &mut ETermArena, roots: &Vec<usize>) -> (out: Vec<usize>)
    requires arena_ok(old(arena)), roots_valid(old(arena).nodes@, roots@),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        roots_valid(final(arena).nodes@, out@),
        forall|i: int| 0 <= i < out@.len() ==> gid_pair_shape(final(arena)@[out@[i] as int]),
        root_terms(final(arena).nodes@, out@)
            == ckc_spec::replay::gid_pairs_all(root_terms(old(arena).nodes@, roots@)),
{
    let mut owned = empty_arena();
    core::mem::swap(arena, &mut owned);
    let (out, mut owned) = gid_pairs_inner(owned, roots);
    core::mem::swap(arena, &mut owned);
    out
}

pub fn arg_root(arena: &mut ETermArena, root: usize, i: usize) -> (out: usize)
    requires root_ok(old(arena), root),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), out),
        final(arena)@[out as int] == ckc_spec::replay::arg(old(arena)@[root as int], i as int),
        ground(old(arena)@[root as int]) ==> ground(final(arena)@[out as int]),
{
    let args = crate::k2_engine::args_roots(arena, root);
    if i < args.len() {
        proof {
            assert(root_terms(arena.nodes@, args@)[i as int] == arena@[args@[i as int] as int]);
            if ground(arena@[root as int]) { ground_arg(arena@[root as int], i as int); }
        }
        args[i]
    } else {
        push_nil(arena)
    }
}

} // verus!
