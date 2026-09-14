use crate::k2_term::{ENode, ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, node_ok, root_ok};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid, roots_work};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::{Term, firsts, var_stream, var_stream_all};
use ckc_spec::trace::*;
use vstd::{assert_seqs_equal, assert_sets_equal};
use vstd::prelude::*;

verus! {

broadcast use {
    Seq::to_set_ensures,
    vstd::seq_lib::lemma_seq_concat_contains_all_elements,
    Seq::lemma_push_to_set_commute,
    Seq::lemma_index_contains,
    vstd::set::group_set_lemmas,
};

pub open spec fn keys_view(keys: Seq<usize>) -> Seq<nat> { keys.map_values(|k: usize| k as nat) }

proof fn streams_concat(left: Seq<Term>, right: Seq<Term>)
    ensures var_stream_all(left + right) == var_stream_all(left) + var_stream_all(right),
    decreases left.len(),
{
    if left.len() > 0 {
        streams_concat(left.drop_first(), right);
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
    }
    reveal_with_fuel(var_stream_all, 2);
}

proof fn contains_cons(head: nat, tail: Seq<nat>, key: nat)
    ensures (seq![head] + tail).contains(key) == (head == key || tail.contains(key)),
{
    vstd::seq_lib::lemma_seq_concat_contains_all_elements(seq![head], tail, key);
    if head == key { assert(seq![head][0] == key); }
    else { assert forall|i: int| 0 <= i < seq![head].len() implies seq![head][i] != key by {} }
}

proof fn firsts_member(values: Seq<nat>, seen: Set<nat>, key: nat)
    ensures firsts(values, seen).contains(key) == (values.contains(key) && !seen.contains(key)),
    decreases values.len(),
{
    if values.len() == 0 {
        reveal(firsts);
        vstd::seq_lib::lemma_seq_empty_contains_nothing(key);
    } else {
        let head = values[0];
        let rest = values.drop_first();
        assert_seqs_equal!(values == seq![head] + rest);
        contains_cons(head, rest, key);
        if seen.contains(head) {
            firsts_member(rest, seen, key);
        } else {
            firsts_member(rest, seen.insert(head), key);
            contains_cons(head, firsts(rest, seen.insert(head)), key);
            if key == head { vstd::set::lemma_set_insert_same(seen, head); }
            else { vstd::set::lemma_set_insert_different(seen, key, head); }
        }
        reveal(firsts);
    }
}

proof fn firsts_set(values: Seq<nat>, seen: Set<nat>)
    ensures
        firsts(values, seen).to_set() == values.to_set().difference(seen),
        firsts(values, seen).no_duplicates(),
    decreases values.len(),
{
    firsts(values, seen).to_set_ensures();
    values.to_set_ensures();
    assert_sets_equal!(firsts(values, seen).to_set() == values.to_set().difference(seen), key => {
        firsts_member(values, seen, key);
        vstd::set::lemma_set_difference(values.to_set(), seen, key);
    });
    if values.len() > 0 {
        let head = values[0];
        if seen.contains(head) {
            firsts_set(values.drop_first(), seen);
            reveal(firsts);
        } else {
            firsts_set(values.drop_first(), seen.insert(head));
            firsts_member(values.drop_first(), seen.insert(head), head);
            vstd::set::lemma_set_insert_same(seen, head);
            let rest = firsts(values.drop_first(), seen.insert(head));
            assert(!rest.contains(head));
            assert forall|i: int, j: int| 0 <= i < seq![head].len() && 0 <= j < rest.len()
                implies seq![head][i] != rest[j] by { rest.lemma_index_contains(j); }
            vstd::seq_lib::lemma_no_dup_in_concat(seq![head], rest);
            reveal(firsts);
        }
    } else { reveal(firsts); }
}

proof fn stream_cons(head: Term, tail: Seq<Term>)
    ensures var_stream_all(seq![head] + tail) == var_stream(head) + var_stream_all(tail),
{
    assert((seq![head] + tail).len() > 0);
    assert((seq![head] + tail)[0] == head);
    assert_seqs_equal!((seq![head] + tail).drop_first() == tail);
    reveal(var_stream_all);
}
proof fn firsts_cons(head: nat, tail: Seq<nat>, seen: Set<nat>)
    ensures firsts(seq![head] + tail, seen) == if seen.contains(head) {
        firsts(tail, seen)
    } else { seq![head] + firsts(tail, seen.insert(head)) },
{
    assert((seq![head] + tail).len() > 0);
    assert((seq![head] + tail)[0] == head);
    assert_seqs_equal!((seq![head] + tail).drop_first() == tail);
    reveal(firsts);
}

fn position(keys: &Vec<usize>, key: usize) -> (out: usize)
    ensures
        out <= keys.len(), out as nat == pos_of(keys_view(keys@), key as nat),
        out < keys.len() ==> keys@[out as int] == key,
        out == keys.len() <==> !keys@.contains(key),
{
    let mut i = 0usize;
    proof { assert_seqs_equal!(keys@.skip(0) == keys@); }
    while i < keys.len()
        invariant
            i <= keys.len(), forall|j: int| 0 <= j < i ==> keys@[j] != key,
            pos_of(keys_view(keys@), key as nat) == i as nat + pos_of(keys_view(keys@.skip(i as int)), key as nat),
        decreases keys.len() - i,
    {
        if keys[i] == key {
            proof { reveal(pos_of); assert(keys@.contains(key)); }
            return i;
        }
        proof {
            assert_seqs_equal!(keys_view(keys@.skip(i as int)).drop_first() == keys_view(keys@.skip(i as int + 1)));
            reveal(pos_of);
        }
        i += 1;
    }
    proof { assert_seqs_equal!(keys_view(keys@.skip(i as int)) == Seq::empty()); reveal(pos_of); }
    i
}

proof fn work_terms(nodes: Seq<ENode>, roots: Seq<usize>)
    ensures roots_work(nodes, roots) == crate::k2_engine::terms_size(root_terms(nodes, roots)),
    decreases roots.len(),
{
    if roots.len() > 0 {
        work_terms(nodes, roots.drop_first());
        assert_seqs_equal!(root_terms(nodes, roots).drop_first() == root_terms(nodes, roots.drop_first()));
    }
    reveal(roots_work); reveal(crate::k2_engine::terms_size);
}

fn collect_step(arena: &ETermArena, input_tasks: Vec<usize>, input_keys: Vec<usize>) -> (out: (Vec<usize>, Vec<usize>))
    requires arena_ok(arena), roots_valid(arena.nodes@, input_tasks@), input_tasks.len() > 0,
    ensures
        roots_valid(arena.nodes@, out.0@),
        roots_work(arena.nodes@, out.0@) < roots_work(arena.nodes@, input_tasks@),
        keys_view(input_keys@) + firsts(var_stream_all(root_terms(arena.nodes@, input_tasks@)), keys_view(input_keys@).to_set())
            == keys_view(out.1@) + firsts(var_stream_all(root_terms(arena.nodes@, out.0@)), keys_view(out.1@).to_set()),
{
    hide(firsts); hide(var_stream_all); hide(var_stream);
    let mut tasks = input_tasks;
    let mut keys = input_keys;
    let ghost old_tasks = tasks@;
    let current = tasks.remove(0);
    proof {
        assert(node_ok(arena.nodes@, current as int));
        assert(old_tasks[0] == current);
        assert(root_terms(arena.nodes@, old_tasks)[0] == arena@[current as int]);
        assert_seqs_equal!(root_terms(arena.nodes@, old_tasks).drop_first() == root_terms(arena.nodes@, tasks@));
        reveal(roots_work);
        assert_seqs_equal!(root_terms(arena.nodes@, old_tasks) == seq![arena@[current as int]] + root_terms(arena.nodes@, tasks@));
        stream_cons(arena@[current as int], root_terms(arena.nodes@, tasks@));
    }
    match &arena.nodes[current].kind {
        ENodeKind::Var { key, .. } => {
            let found = position(&keys, *key);
            let ghost previous = keys@;
            proof {
                assert(arena@[current as int] == Term::Var(*key as nat));
                assert(var_stream(arena@[current as int]) == seq![*key as nat]) by { reveal(var_stream); }
                assert(var_stream_all(root_terms(arena.nodes@, old_tasks)) == seq![*key as nat] + var_stream_all(root_terms(arena.nodes@, tasks@)));
                firsts_cons(*key as nat, var_stream_all(root_terms(arena.nodes@, tasks@)), keys_view(keys@).to_set());
            }
            if found == keys.len() {
                proof {
                    assert forall|j: int| 0 <= j < keys.len() implies keys_view(keys@)[j] != *key as nat by {
                        assert(keys@[j] != *key);
                    }
                    assert(!keys_view(keys@).contains(*key as nat));
                }
                keys.push(*key);
                proof {
                    assert_seqs_equal!(keys_view(keys@) == keys_view(previous).push(*key as nat));
                    assert_sets_equal!(keys_view(keys@).to_set() == keys_view(previous).to_set().insert(*key as nat));
                }
            } else {
                proof {
                    assert(keys@[found as int] == *key);
                    assert(keys_view(keys@)[found as int] == *key as nat);
                    assert(keys_view(keys@).to_set().contains(*key as nat));
                }
            }
        },
        ENodeKind::Comp { name, child_roots, .. } => {
            let mut children = child_roots.clone();
            proof {
                crate::k2_engine::node_comp_model(arena.nodes@, current as int, name@, child_roots@);
                assert_seqs_equal!(root_terms(arena.nodes@, children@) == crate::k2_term::child_terms(arena.nodes@, child_roots@));
                streams_concat(root_terms(arena.nodes@, children@), root_terms(arena.nodes@, tasks@));
                assert(var_stream(arena@[current as int]) == var_stream_all(root_terms(arena.nodes@, children@))) by { reveal(var_stream); }
                crate::k2_engine::roots_work_concat(arena.nodes@, children@, tasks@);
        work_terms(arena.nodes@, children@);
                reveal(var_stream);
                reveal(crate::k2_engine::term_size);
            }
            let ghost front = children@;
            let ghost rest = tasks@;
            children.append(&mut tasks);
            tasks = children;
            proof {
                assert_seqs_equal!(root_terms(arena.nodes@, tasks@) == root_terms(arena.nodes@, front) + root_terms(arena.nodes@, rest));
                assert(roots_valid(arena.nodes@, tasks@));
                assert(var_stream_all(root_terms(arena.nodes@, tasks@)) == var_stream_all(root_terms(arena.nodes@, old_tasks)));
            }
        },
        _ => { proof { reveal(var_stream); } },
    }
    (tasks, keys)
}

fn collect(arena: &ETermArena, root: usize) -> (out: Vec<usize>)
    requires root_ok(arena, root),
    ensures keys_view(out@) == firsts(var_stream(arena@[root as int]), Set::empty()),
{
    hide(firsts); hide(var_stream_all); hide(var_stream);
    let ghost model = arena@[root as int];
    let mut tasks = Vec::new(); tasks.push(root);
    let mut keys = Vec::new();
    proof {
        assert_seqs_equal!(root_terms(arena.nodes@, tasks@) == seq![model]);
        assert_seqs_equal!(keys_view(keys@) == Seq::empty());
        stream_cons(model, Seq::empty());
        reveal_with_fuel(var_stream_all, 2);
        assert(var_stream_all(root_terms(arena.nodes@, tasks@)) == var_stream(model));
        assert_sets_equal!(keys_view(keys@).to_set() == Set::<nat>::empty());
    }
    while tasks.len() > 0
        invariant
            root_ok(arena, root), model == arena@[root as int], roots_valid(arena.nodes@, tasks@),
            firsts(var_stream(model), Set::empty()) == keys_view(keys@)
                + firsts(var_stream_all(root_terms(arena.nodes@, tasks@)), keys_view(keys@).to_set()),
        decreases roots_work(arena.nodes@, tasks@),
    {
        let (next_tasks, next_keys) = collect_step(arena, tasks, keys);
        tasks = next_tasks;
        keys = next_keys;
    }
    proof {
        assert_seqs_equal!(root_terms(arena.nodes@, tasks@) == Seq::empty());
        reveal(var_stream_all); reveal(firsts);
    }
    keys
}

pub open spec fn partial(t: Term, keys: Seq<nat>, base: nat, count: nat) -> Term
    decreases t,
{
    match t {
        Term::Var(k) => if pos_of(keys, k) < count { dollar_var(base + pos_of(keys, k)) } else { t },
        Term::Comp(name, args) => Term::Comp(name, partial_all(args, keys, base, count)),
        _ => t,
    }
}

pub open spec fn partial_all(ts: Seq<Term>, keys: Seq<nat>, base: nat, count: nat) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 { Seq::empty() } else {
        seq![partial(ts[0], keys, base, count)] + partial_all(ts.drop_first(), keys, base, count)
    }
}

proof fn pos_unique(keys: Seq<nat>, i: int)
    requires keys.no_duplicates(), 0 <= i < keys.len(),
    ensures pos_of(keys, keys[i]) == i,
    decreases keys.len(),
{
    if i > 0 {
        assert(keys[0] != keys[i]);
        pos_unique(keys.drop_first(), i - 1);
        assert(keys.drop_first()[i - 1] == keys[i]);
    }
    reveal(pos_of);
}

proof fn pos_member(keys: Seq<nat>, key: nat)
    ensures
        pos_of(keys, key) <= keys.len(),
        pos_of(keys, key) < keys.len() <==> keys.contains(key),
        pos_of(keys, key) < keys.len() ==> keys[pos_of(keys, key) as int] == key,
    decreases keys.len(),
{
    if keys.len() > 0 && keys[0] != key {
        pos_member(keys.drop_first(), key);
        assert_seqs_equal!(keys == seq![keys[0]] + keys.drop_first());
        assert(keys.contains(key) == keys.drop_first().contains(key));
    }
    reveal(pos_of);
}

proof fn dollar_subst(n: nat, key: nat, value: Term)
    ensures ckc_spec::engine::subst(dollar_var(n), key, value) == dollar_var(n),
{
    reveal(dollar_var);
    reveal_with_fuel(ckc_spec::engine::subst, 3);
    reveal_with_fuel(ckc_spec::engine::subst_all, 3);
}

proof fn partial_zero(t: Term, keys: Seq<nat>, base: nat)
    ensures partial(t, keys, base, 0) == t,
    decreases t,
{
    if let Term::Comp(_, args) = t { partial_all_zero(args, keys, base); }
}
proof fn partial_all_zero(ts: Seq<Term>, keys: Seq<nat>, base: nat)
    ensures partial_all(ts, keys, base, 0) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        partial_zero(ts[0], keys, base);
        partial_all_zero(ts.drop_first(), keys, base);
        assert_seqs_equal!(seq![ts[0]] + ts.drop_first() == ts);
    }
    reveal(partial_all);
}

proof fn partial_step(t: Term, keys: Seq<nat>, base: nat, i: nat)
    requires keys.no_duplicates(), i < keys.len(),
    ensures ckc_spec::engine::subst(partial(t, keys, base, i), keys[i as int], dollar_var(base + i)) == partial(t, keys, base, i + 1),
    decreases t,
{
    match t {
        Term::Var(k) => {
            pos_unique(keys, i as int);
            pos_member(keys, k);
            if pos_of(keys, k) < i { dollar_subst(base + pos_of(keys, k), keys[i as int], dollar_var(base + i)); }
            reveal(partial); reveal(ckc_spec::engine::subst);
        },
        Term::Comp(_, args) => {
            partial_all_step(args, keys, base, i);
            reveal(partial); reveal(ckc_spec::engine::subst);
        },
        _ => { reveal(partial); reveal(ckc_spec::engine::subst); },
    }
}
proof fn partial_all_step(ts: Seq<Term>, keys: Seq<nat>, base: nat, i: nat)
    requires keys.no_duplicates(), i < keys.len(),
    ensures ckc_spec::engine::subst_all(partial_all(ts, keys, base, i), keys[i as int], dollar_var(base + i)) == partial_all(ts, keys, base, i + 1),
    decreases ts,
{
    if ts.len() > 0 {
        partial_step(ts[0], keys, base, i);
        partial_all_step(ts.drop_first(), keys, base, i);
        reveal(partial_all);
        let left = partial_all(ts, keys, base, i);
        assert(left.len() > 0);
        assert(left[0] == partial(ts[0], keys, base, i));
        assert_seqs_equal!(left.drop_first() == partial_all(ts.drop_first(), keys, base, i));
    }
    reveal_with_fuel(partial_all, 2); reveal_with_fuel(ckc_spec::engine::subst_all, 2);
}

proof fn partial_done(t: Term, keys: Seq<nat>, base: nat)
    requires var_stream(t).to_set().subset_of(keys.to_set()),
    ensures partial(t, keys, base, keys.len()) == number_with(t, keys, base),
    decreases t,
{
    match t {
        Term::Var(k) => {
            pos_member(keys, k); reveal(var_stream);
            assert(var_stream(t) == seq![k]);
            assert(var_stream(t)[0] == k);
            assert(var_stream(t).contains(k));
            assert(keys.to_set().contains(k));
            reveal(partial); reveal(number_with);
        },
        Term::Comp(_, args) => {
            partial_all_done(args, keys, base);
            reveal(partial); reveal(number_with);
        },
        _ => { reveal(partial); reveal(number_with); },
    }
}
proof fn partial_all_done(ts: Seq<Term>, keys: Seq<nat>, base: nat)
    requires var_stream_all(ts).to_set().subset_of(keys.to_set()),
    ensures partial_all(ts, keys, base, keys.len()) == number_all(ts, keys, base),
    decreases ts,
{
    if ts.len() > 0 {
        reveal(var_stream_all);
        vstd::seq_lib::seq_to_set_distributes_over_add(var_stream(ts[0]), var_stream_all(ts.drop_first()));
        assert(var_stream(ts[0]).to_set().subset_of(keys.to_set()));
        assert(var_stream_all(ts.drop_first()).to_set().subset_of(keys.to_set()));
        partial_done(ts[0], keys, base);
        partial_all_done(ts.drop_first(), keys, base);
    }
    reveal_with_fuel(partial_all, 2); reveal_with_fuel(number_all, 2);
}

fn number_inner(mut arena: ETermArena, root: usize, base: usize) -> (out: (ETermArena, usize, usize))
    requires root_ok(&arena, root), base <= arena.nodes.len(),
    ensures
        arena_ok(&out.0), arena.nodes@.is_prefix_of(out.0.nodes@), root_ok(&out.0, out.1), out.2 <= out.0.nodes.len(),
        (out.0@[out.1 as int], out.2 as nat) == number(arena@[root as int], base as nat),
{
    let ghost origin = arena.nodes@;
    let ghost model = arena@[root as int];
    let keys = collect(&arena, root);
    let ghost fs = keys_view(keys@);
    proof {
        firsts_set(var_stream(model), Set::empty());
        assert_sets_equal!(var_stream(model).to_set().difference(Set::empty()) == var_stream(model).to_set());
        partial_zero(model, fs, base as nat);
    }
    let mut current = root;
    let mut next = base;
    let mut i = 0usize;
    while i < keys.len()
        invariant
            arena_ok(&arena), origin.is_prefix_of(arena.nodes@), root_ok(&arena, current),
            root < origin.len(), model == origin[root as int].term@, fs == keys_view(keys@),
            fs == firsts(var_stream(model), Set::empty()), fs.no_duplicates(),
            fs.to_set() == var_stream(model).to_set(), i <= keys.len(),
            next as nat == base as nat + i as nat, next <= arena.nodes.len(),
            arena@[current as int] == partial(model, fs, base as nat, i as nat),
        decreases keys.len() - i,
    {
        let ghost before = arena.nodes@;
        let n = crate::k2_output::int_root(&mut arena, next);
        let name: &[u8] = b"$VAR";
        proof { reveal_byteslit(b"$VAR"); reveal(ckc_spec::term::dollar_var_name); }
        let mut args = Vec::new(); args.push(n);
        proof {
            assert(name@ == ckc_spec::term::dollar_var_name());
            crate::k2_term::child_terms_match(arena.nodes@, args@, seq![Term::Int(next as int)]);
        }
        let replacement = crate::k2_term::push_comp(&mut arena, vstd::slice::slice_to_vec(name), args);
        proof {
            crate::k2_term::arena_prefix_stable(before, &arena);
            assert(arena@[replacement as int] == dollar_var(next as nat));
            assert(next < arena.nodes.len());
            partial_step(model, fs, base as nat, i as nat);
        }
        current = crate::k2_engine::subst_root(&mut arena, current, keys[i], replacement);
        next += 1;
        i += 1;
    }
    proof { partial_done(model, fs, base as nat); }
    (arena, current, next)
}

pub fn number_exec(arena: &mut ETermArena, root: usize, base: usize) -> (out: (usize, usize))
    requires root_ok(old(arena), root), base <= old(arena).nodes.len(),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), out.0), out.1 <= final(arena).nodes.len(),
        (final(arena)@[out.0 as int], out.1 as nat) == number(old(arena)@[root as int], base as nat),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (mut owned, numbered, next) = number_inner(owned, root, base);
    core::mem::swap(arena, &mut owned);
    (numbered, next)
}

} // verus!
