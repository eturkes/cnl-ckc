use crate::k2_payload::EOb;
use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid, roots_models_prefix};
#[cfg(verus_keep_ghost)]
use crate::k2_payload::{ob_valid, ob_view, obs_valid, obs_view, obs_prefix, obs_wf_at};
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use ckc_spec::replay::Ob;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::{key, ds, wf_obs};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::{ground, ground_all};
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

proof fn pair_ground(left: ckc_spec::term::Term, right: ckc_spec::term::Term)
    requires ground(left), ground(right),
    ensures ground(ckc_spec::replay::pair(left, right)),
{
    reveal_with_fuel(ground, 4);
    reveal_with_fuel(ground_all, 4);
    assert_seqs_equal!(seq![left, right].drop_first() == seq![right]);
    assert_seqs_equal!(seq![right].drop_first() == Seq::empty());
    assert(ground_all(seq![right]));
    assert(ground_all(seq![left, right]));
    assert(ground(ckc_spec::replay::pair(left, right)) == ground_all(seq![left, right]));
}

proof fn ob_keys_ground(ob: Ob)
    requires wf_obs(seq![ob]),
    ensures ground(key(ob)), ground(ds(ob)),
{
    obs_wf_at(seq![ob], 0);
    let term = ckc_spec::replay::ob_term(ob);
    crate::k2_walk::ground_arg(term, 0);
    crate::k2_walk::ground_arg(term, 1);
    crate::k2_walk::ground_arg(term, 2);
    crate::k2_walk::ground_arg(ckc_spec::replay::variant(ob), 0);
    pair_ground(ob.docid, ob.s);
    pair_ground(ds(ob), ob.k);
}

fn ob_key_roots_inner(input_arena: ETermArena, obs: &Vec<EOb>)
    -> (out: (Vec<usize>, Vec<usize>, ETermArena))
    requires arena_ok(&input_arena), obs_valid(input_arena.nodes@, obs@),
        wf_obs(obs_view(input_arena.nodes@, obs@)),
    ensures
        arena_ok(&out.2), input_arena.nodes@.is_prefix_of(out.2.nodes@),
        roots_valid(out.2.nodes@, out.0@), roots_valid(out.2.nodes@, out.1@),
        root_terms(out.2.nodes@, out.0@) == obs_view(input_arena.nodes@, obs@).map_values(|ob: Ob| key(ob)),
        root_terms(out.2.nodes@, out.1@) == obs_view(input_arena.nodes@, obs@).map_values(|ob: Ob| ds(ob)),
        forall|i: int| 0 <= i < out.0@.len() ==> ground(out.2@[out.0@[i] as int]),
        forall|i: int| 0 <= i < out.1@.len() ==> ground(out.2@[out.1@[i] as int]),
{
    let ghost origin = input_arena.nodes@;
    let ghost models = obs_view(origin, obs@);
    let mut arena = input_arena;
    let mut keys = Vec::new();
    let mut pairs = Vec::new();
    let mut i = 0usize;
    let name: &[u8] = b"-";
    proof {
        reveal_byteslit(b"-"); reveal_strlit("-");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("-"@));
    }
    while i < obs.len()
        invariant
            arena_ok(&arena), origin == input_arena.nodes@, origin.is_prefix_of(arena.nodes@),
            obs_valid(arena.nodes@, obs@), models == obs_view(origin, obs@),
            models == obs_view(arena.nodes@, obs@), wf_obs(models),
            i <= obs.len(), keys.len() == i, pairs.len() == i,
            roots_valid(arena.nodes@, keys@), roots_valid(arena.nodes@, pairs@),
            forall|j: int| 0 <= j < i ==> arena@[keys@[j] as int] == key(models[j])
                && ground(arena@[keys@[j] as int]),
            forall|j: int| 0 <= j < i ==> arena@[pairs@[j] as int] == ds(models[j])
                && ground(arena@[pairs@[j] as int]),
            name@ == ckc_spec::v1text::ascii("-"@),
        decreases obs.len() - i,
    {
        let ghost before_nodes = arena.nodes@;
        let ghost before_keys = keys@;
        let ghost before_pairs = pairs@;
        proof {
            assert(ob_valid(arena.nodes@, &obs@[i as int]));
            assert(models[i as int] == ob_view(arena.nodes@, &obs@[i as int]));
            obs_wf_at(models, i as int);
            assert(wf_obs(seq![models[i as int]]));
            ob_keys_ground(models[i as int]);
        }
        let docid = obs[i].docid;
        let sentence = obs[i].sentence;
        let variant = obs[i].variant;
        let pair = crate::k2_output::comp2(&mut arena, name, docid, sentence);
        let key = crate::k2_output::comp2(&mut arena, name, pair, variant);
        proof {
            obs_prefix(before_nodes, arena.nodes@, obs@);
            roots_models_prefix(before_nodes, arena.nodes@, before_keys);
            roots_models_prefix(before_nodes, arena.nodes@, before_pairs);
            crate::k2_load::prefix_chain(origin, before_nodes, arena.nodes@);
            assert(arena@[key as int] == ckc_spec::replay::key(models[i as int]));
            assert(arena@[pair as int] == ds(models[i as int]));
        }
        keys.push(key);
        pairs.push(pair);
        proof {
            assert forall|j: int| 0 <= j < keys.len() implies
                arena@[keys@[j] as int] == ckc_spec::replay::key(models[j])
                    && ground(arena@[keys@[j] as int]) by {
                if j < before_keys.len() { assert(keys@[j] == before_keys[j]); }
            }
            assert forall|j: int| 0 <= j < pairs.len() implies
                arena@[pairs@[j] as int] == ds(models[j]) && ground(arena@[pairs@[j] as int]) by {
                if j < before_pairs.len() { assert(pairs@[j] == before_pairs[j]); }
            }
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(root_terms(arena.nodes@, keys@) == models.map_values(|ob: Ob| key(ob)));
        assert_seqs_equal!(root_terms(arena.nodes@, pairs@) == models.map_values(|ob: Ob| ds(ob)));
        assert forall|j: int| 0 <= j < keys.len() implies ground(arena@[keys@[j] as int]) by {
            assert(arena@[keys@[j] as int] == key(models[j]));
        }
        assert forall|j: int| 0 <= j < pairs.len() implies ground(arena@[pairs@[j] as int]) by {
            assert(arena@[pairs@[j] as int] == ds(models[j]));
        }
    }
    (keys, pairs, arena)
}

pub fn ob_key_roots(arena: &mut ETermArena, obs: &Vec<EOb>) -> (out: (Vec<usize>, Vec<usize>))
    requires arena_ok(old(arena)), obs_valid(old(arena).nodes@, obs@),
        wf_obs(obs_view(old(arena).nodes@, obs@)),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        roots_valid(final(arena).nodes@, out.0@), roots_valid(final(arena).nodes@, out.1@),
        root_terms(final(arena).nodes@, out.0@) == obs_view(old(arena).nodes@, obs@).map_values(|ob: Ob| key(ob)),
        root_terms(final(arena).nodes@, out.1@) == obs_view(old(arena).nodes@, obs@).map_values(|ob: Ob| ds(ob)),
        forall|i: int| 0 <= i < out.0@.len() ==> ground(final(arena)@[out.0@[i] as int]),
        forall|i: int| 0 <= i < out.1@.len() ==> ground(final(arena)@[out.1@[i] as int]),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (keys, pairs, mut owned) = ob_key_roots_inner(owned, obs);
    core::mem::swap(arena, &mut owned);
    (keys, pairs)
}

pub open spec fn selected_root(nodes: Seq<crate::k2_term::ENode>, out: Option<usize>)
    -> Option<ckc_spec::term::Term>
{
    match out { Some(root) => Some(nodes[root as int].term@), None => None }
}

pub proof fn sort_views(arena: &ETermArena, roots: Seq<usize>)
    requires roots_valid(arena.nodes@, roots),
    ensures crate::k2_sort::root_terms(arena, roots) == root_terms(arena.nodes@, roots),
{
    assert_seqs_equal!(crate::k2_sort::root_terms(arena, roots) == root_terms(arena.nodes@, roots));
}

fn first_dup_root(arena: &ETermArena, roots: &Vec<usize>) -> (out: Option<usize>)
    requires crate::k2_sort::roots_ok(arena, roots@), crate::k2_sort::roots_ground(arena, roots@),
    ensures out matches Some(root) ==> root < arena.nodes.len(),
        selected_root(arena.nodes@, out) == ckc_spec::replay::first_dup(root_terms(arena.nodes@, roots@)),
{
    hide(ckc_spec::replay::first_dup);
    let ghost models = root_terms(arena.nodes@, roots@);
    let mut i = 0usize;
    proof { assert_seqs_equal!(models.skip(0) == models); }
    while roots.len() - i >= 2
        invariant
            crate::k2_sort::roots_ok(arena, roots@), crate::k2_sort::roots_ground(arena, roots@),
            i <= roots.len(), models == root_terms(arena.nodes@, roots@),
            ckc_spec::replay::first_dup(models) == ckc_spec::replay::first_dup(models.skip(i as int)),
        decreases roots.len() - i,
    {
        proof {
            reveal_with_fuel(ckc_spec::replay::first_dup, 1);
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
        }
        if crate::k2_sort::term_equal(arena, roots[i], roots[i + 1]) { return Some(roots[i]); }
        i += 1;
    }
    proof { reveal_with_fuel(ckc_spec::replay::first_dup, 1); }
    None
}

fn sequence_inner(input_arena: ETermArena, roots: &Vec<usize>) -> (out: (bool, ETermArena))
    requires crate::k2_sort::roots_ok(&input_arena, roots@),
        crate::k2_sort::roots_ground(&input_arena, roots@),
    ensures arena_ok(&out.1), input_arena.nodes@.is_prefix_of(out.1.nodes@),
        out.0 == ckc_spec::replay::is_sequence(root_terms(input_arena.nodes@, roots@), 1),
{
    hide(ckc_spec::replay::is_sequence);
    let ghost origin = input_arena.nodes@;
    let ghost models = root_terms(origin, roots@);
    let mut arena = input_arena;
    let mut i = 0usize;
    proof { assert_seqs_equal!(models.skip(0) == models); }
    while i < roots.len()
        invariant
            arena_ok(&arena), origin == input_arena.nodes@, origin.is_prefix_of(arena.nodes@),
            crate::k2_sort::roots_ok(&arena, roots@), crate::k2_sort::roots_ground(&arena, roots@),
            i <= roots.len(), models == root_terms(origin, roots@), models == root_terms(arena.nodes@, roots@),
            ckc_spec::replay::is_sequence(models, 1)
                == ckc_spec::replay::is_sequence(models.skip(i as int), i as int + 1),
        decreases roots.len() - i,
    {
        let ghost before = arena.nodes@;
        let expected = crate::k2_output::int_root(&mut arena, i + 1);
        proof {
            roots_models_prefix(before, arena.nodes@, roots@);
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
            reveal_with_fuel(ckc_spec::replay::is_sequence, 1);
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
        }
        if !crate::k2_sort::term_equal(&arena, roots[i], expected) { return (false, arena); }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(ckc_spec::replay::is_sequence, 1);
    }
    (true, arena)
}

fn sequence_exec(arena: &mut ETermArena, roots: &Vec<usize>) -> (out: bool)
    requires crate::k2_sort::roots_ok(old(arena), roots@), crate::k2_sort::roots_ground(old(arena), roots@),
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ckc_spec::replay::is_sequence(root_terms(old(arena).nodes@, roots@), 1),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (out, mut owned) = sequence_inner(owned, roots);
    core::mem::swap(arena, &mut owned);
    out
}

fn variant_sequence_root(arena: &mut ETermArena, d: usize, variants: &Vec<usize>) -> (out: usize)
    requires crate::k2_term::root_ok(old(arena), d), roots_valid(old(arena).nodes@, variants@),
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        crate::k2_term::root_ok(final(arena), out),
        final(arena)@[out as int] == ckc_spec::replay::variant_sequence(
            old(arena)@[d as int], root_terms(old(arena).nodes@, variants@)),
{
    let ghost origin = arena.nodes@;
    let docid = crate::k2_walk::arg_root(arena, d, 0);
    let sentence = crate::k2_walk::arg_root(arena, d, 1);
    proof { roots_models_prefix(origin, arena.nodes@, variants@); }
    let list = crate::k2_walk::list_root(arena, variants);
    let name: &[u8] = b"variant_sequence";
    proof {
        reveal_byteslit(b"variant_sequence"); reveal_strlit("variant_sequence");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("variant_sequence"@));
    }
    crate::k2_output::comp3(arena, name, docid, sentence, list)
}

fn bad_run_inner(input_arena: ETermArena, keys: &Vec<usize>) -> (out: (Option<usize>, ETermArena))
    requires crate::k2_sort::roots_ok(&input_arena, keys@), crate::k2_sort::roots_ground(&input_arena, keys@),
    ensures arena_ok(&out.1), input_arena.nodes@.is_prefix_of(out.1.nodes@),
        out.0 matches Some(root) ==> root < out.1.nodes.len(),
        selected_root(out.1.nodes@, out.0) == ckc_spec::replay::first_bad_run(root_terms(input_arena.nodes@, keys@)),
{
    hide(ckc_spec::replay::bad_run);
    let ghost origin = input_arena.nodes@;
    let ghost models = root_terms(origin, keys@);
    let mut arena = input_arena;
    if keys.len() == 0 { return (None, arena); }
    let mut d = crate::k2_walk::arg_root(&mut arena, keys[0], 0);
    let k = crate::k2_walk::arg_root(&mut arena, keys[0], 1);
    let mut variants = Vec::new();
    variants.push(k);
    let mut i = 1usize;
    proof {
        roots_models_prefix(origin, arena.nodes@, keys@);
        assert_seqs_equal!(root_terms(arena.nodes@, variants@) == seq![ckc_spec::replay::arg(models[0], 1)]);
        assert_seqs_equal!(models.skip(1) == models.drop_first());
    }
    while i < keys.len()
        invariant
            arena_ok(&arena), origin == input_arena.nodes@, origin.is_prefix_of(arena.nodes@),
            crate::k2_sort::roots_ok(&arena, keys@), crate::k2_sort::roots_ground(&arena, keys@),
            crate::k2_sort::roots_ok(&arena, variants@), crate::k2_sort::roots_ground(&arena, variants@),
            d < arena.nodes.len(), ground(arena@[d as int]),
            1 <= i <= keys.len(), models == root_terms(origin, keys@), models == root_terms(arena.nodes@, keys@),
            ckc_spec::replay::first_bad_run(models) == ckc_spec::replay::bad_run(
                models.skip(i as int), arena@[d as int], root_terms(arena.nodes@, variants@)),
        decreases keys.len() - i,
    {
        let ghost before = arena.nodes@;
        let ghost before_d = arena@[d as int];
        let ghost before_variants = root_terms(before, variants@);
        let next_d = crate::k2_walk::arg_root(&mut arena, keys[i], 0);
        proof {
            roots_models_prefix(before, arena.nodes@, keys@);
            roots_models_prefix(before, arena.nodes@, variants@);
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
            reveal_with_fuel(ckc_spec::replay::bad_run, 1);
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
        }
        if !crate::k2_sort::term_equal(&arena, d, next_d) {
            let ghost before_sequence = arena.nodes@;
            let sequential = sequence_exec(&mut arena, &variants);
            proof {
                roots_models_prefix(before_sequence, arena.nodes@, keys@);
                roots_models_prefix(before_sequence, arena.nodes@, variants@);
                crate::k2_load::prefix_chain(origin, before_sequence, arena.nodes@);
            }
            if !sequential {
                let ghost before_detail = arena.nodes@;
                let detail = variant_sequence_root(&mut arena, d, &variants);
                proof { crate::k2_load::prefix_chain(origin, before_detail, arena.nodes@); }
                return (Some(detail), arena);
            }
            d = next_d;
            variants = Vec::new();
            proof { assert_seqs_equal!(root_terms(arena.nodes@, variants@) == Seq::empty()); }
        }
        let ghost before_k = arena.nodes@;
        let ghost previous_variants = variants@;
        let ghost previous_models = root_terms(before_k, previous_variants);
        let next_k = crate::k2_walk::arg_root(&mut arena, keys[i], 1);
        proof {
            roots_models_prefix(before_k, arena.nodes@, keys@);
            roots_models_prefix(before_k, arena.nodes@, variants@);
            crate::k2_load::prefix_chain(origin, before_k, arena.nodes@);
        }
        variants.push(next_k);
        proof {
            assert_seqs_equal!(root_terms(arena.nodes@, variants@)
                == previous_models.push(ckc_spec::replay::arg(models[i as int], 1)));
            assert forall|j: int| 0 <= j < variants.len() implies
                variants@[j] < arena.nodes.len() && ground(arena@[variants@[j] as int]) by {
                if j < previous_variants.len() { assert(variants@[j] == previous_variants[j]); }
            }
            if ckc_spec::replay::arg(models[i as int], 0) != before_d {
                assert_seqs_equal!(previous_models.push(ckc_spec::replay::arg(models[i as int], 1))
                    == seq![ckc_spec::replay::arg(models[i as int], 1)]);
            }
        }
        i += 1;
    }
    let ghost before_sequence = arena.nodes@;
    let sequential = sequence_exec(&mut arena, &variants);
    proof {
        roots_models_prefix(before_sequence, arena.nodes@, variants@);
        crate::k2_load::prefix_chain(origin, before_sequence, arena.nodes@);
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(ckc_spec::replay::bad_run, 1);
    }
    if sequential { (None, arena) } else {
        let ghost before_detail = arena.nodes@;
        let detail = variant_sequence_root(&mut arena, d, &variants);
        proof { crate::k2_load::prefix_chain(origin, before_detail, arena.nodes@); }
        (Some(detail), arena)
    }
}

fn first_bad_run_exec(arena: &mut ETermArena, keys: &Vec<usize>) -> (out: Option<usize>)
    requires crate::k2_sort::roots_ok(old(arena), keys@), crate::k2_sort::roots_ground(old(arena), keys@),
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(root) ==> root < final(arena).nodes.len(),
        selected_root(final(arena).nodes@, out) == ckc_spec::replay::first_bad_run(root_terms(old(arena).nodes@, keys@)),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (out, mut owned) = bad_run_inner(owned, keys);
    core::mem::swap(arena, &mut owned);
    out
}

fn duplicate_error(arena: &mut ETermArena, key: usize) -> (out: ckc_spec::replay::EOut)
    requires crate::k2_term::root_ok(old(arena), key),
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == ckc_spec::replay::proof_fail(ckc_spec::term::Term::Comp(
            ckc_spec::v1text::ascii("duplicate_obligation"@), seq![
                ckc_spec::replay::arg(ckc_spec::replay::arg(old(arena)@[key as int], 0), 0),
                ckc_spec::replay::arg(ckc_spec::replay::arg(old(arena)@[key as int], 0), 1),
                ckc_spec::term::Term::Comp(ckc_spec::v1text::ascii("variant"@),
                    seq![ckc_spec::replay::arg(old(arena)@[key as int], 1)])])),
{
    let d = crate::k2_walk::arg_root(arena, key, 0);
    let k = crate::k2_walk::arg_root(arena, key, 1);
    let docid = crate::k2_walk::arg_root(arena, d, 0);
    let sentence = crate::k2_walk::arg_root(arena, d, 1);
    let variant_name: &[u8] = b"variant";
    let name: &[u8] = b"duplicate_obligation";
    proof {
        reveal_byteslit(b"variant"); reveal_strlit("variant");
        reveal_byteslit(b"duplicate_obligation"); reveal_strlit("duplicate_obligation");
        reveal(ckc_spec::v1text::ascii);
        assert(variant_name@ == ckc_spec::v1text::ascii("variant"@));
        assert(name@ == ckc_spec::v1text::ascii("duplicate_obligation"@));
    }
    let variant = crate::k2_output::comp1(arena, variant_name, k);
    let detail = crate::k2_output::comp3(arena, name, docid, sentence, variant);
    crate::k2_output::error_out(arena, detail, true)
}

pub fn key_check_exec(arena: &mut ETermArena, obs: &Vec<EOb>)
    -> (out: (Option<ckc_spec::replay::EOut>, Vec<usize>))
    requires arena_ok(old(arena)), obs_valid(old(arena).nodes@, obs@), wf_obs(obs_view(old(arena).nodes@, obs@)),
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        roots_valid(final(arena).nodes@, out.1@), crate::k2_sort::roots_ground(final(arena), out.1@),
        root_terms(final(arena).nodes@, out.1@) == obs_view(old(arena).nodes@, obs@).map_values(|ob: Ob| ds(ob)),
        crate::k2_output::option_out_view(out.0) == ckc_spec::replay::key_check(obs_view(old(arena).nodes@, obs@)),
{
    let ghost origin = arena.nodes@;
    let (keys, pairs) = ob_key_roots(arena, obs);
    let ghost key_nodes = arena.nodes@;
    proof { sort_views(arena, keys@); }
    let sorted = crate::k2_sort::msort(arena, &keys);
    proof { sort_views(arena, sorted@); }
    if let Some(root) = first_dup_root(arena, &sorted) {
        let error = duplicate_error(arena, root);
        proof {
            roots_models_prefix(key_nodes, arena.nodes@, pairs@);
            crate::k2_load::prefix_chain(origin, key_nodes, arena.nodes@);
        }
        return (Some(error), pairs);
    }
    let unique = crate::k2_sort::sort_unique(arena, &keys);
    proof { sort_views(arena, unique@); }
    let failed = first_bad_run_exec(arena, &unique);
    proof {
        roots_models_prefix(key_nodes, arena.nodes@, pairs@);
        crate::k2_load::prefix_chain(origin, key_nodes, arena.nodes@);
    }
    if let Some(root) = failed {
        let ghost before = arena.nodes@;
        let error = crate::k2_output::error_out(arena, root, true);
        proof {
            roots_models_prefix(before, arena.nodes@, pairs@);
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
        }
        return (Some(error), pairs);
    }
    (None, pairs)
}

fn contains_root(arena: &ETermArena, roots: &Vec<usize>, root: usize) -> (out: bool)
    requires crate::k2_sort::roots_ok(arena, roots@), crate::k2_sort::roots_ground(arena, roots@),
        crate::k2_term::root_ok(arena, root), ground(arena@[root as int]),
    ensures out == root_terms(arena.nodes@, roots@).contains(arena@[root as int]),
{
    let mut i = 0usize;
    while i < roots.len()
        invariant crate::k2_sort::roots_ok(arena, roots@), crate::k2_sort::roots_ground(arena, roots@),
            crate::k2_term::root_ok(arena, root), ground(arena@[root as int]), i <= roots.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] root_terms(arena.nodes@, roots@)[j] != arena@[root as int],
        decreases roots.len() - i,
    {
        if crate::k2_sort::term_equal(arena, roots[i], root) {
            proof { assert(root_terms(arena.nodes@, roots@)[i as int] == arena@[root as int]); }
            return true;
        }
        i += 1;
    }
    false
}

fn first_absent_root(arena: &ETermArena, xs: &Vec<usize>, ys: &Vec<usize>) -> (out: Option<usize>)
    requires crate::k2_sort::roots_ok(arena, xs@), crate::k2_sort::roots_ground(arena, xs@),
        crate::k2_sort::roots_ok(arena, ys@), crate::k2_sort::roots_ground(arena, ys@),
    ensures out matches Some(root) ==> root < arena.nodes.len() && xs@.contains(root),
        selected_root(arena.nodes@, out) == ckc_spec::replay::first_absent(
            root_terms(arena.nodes@, xs@), root_terms(arena.nodes@, ys@)),
{
    hide(ckc_spec::replay::first_absent);
    let ghost models = root_terms(arena.nodes@, xs@);
    let ghost others = root_terms(arena.nodes@, ys@);
    let mut i = 0usize;
    proof { assert_seqs_equal!(models.skip(0) == models); }
    while i < xs.len()
        invariant crate::k2_sort::roots_ok(arena, xs@), crate::k2_sort::roots_ground(arena, xs@),
            crate::k2_sort::roots_ok(arena, ys@), crate::k2_sort::roots_ground(arena, ys@),
            i <= xs.len(), models == root_terms(arena.nodes@, xs@), others == root_terms(arena.nodes@, ys@),
            ckc_spec::replay::first_absent(models, others)
                == ckc_spec::replay::first_absent(models.skip(i as int), others),
        decreases xs.len() - i,
    {
        proof {
            reveal_with_fuel(ckc_spec::replay::first_absent, 1);
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
        }
        if !contains_root(arena, ys, xs[i]) {
            proof { assert(xs@.contains(xs@[i as int])); }
            return Some(xs[i]);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(ckc_spec::replay::first_absent, 1);
    }
    None
}

fn loaded_pairs_inner(input_arena: ETermArena, db: &Vec<crate::k2_engine::EClause>)
    -> (out: (Vec<usize>, ETermArena))
    requires arena_ok(&input_arena), crate::k2_engine::db_valid(input_arena.nodes@, db@),
    ensures arena_ok(&out.1), input_arena.nodes@.is_prefix_of(out.1.nodes@),
        roots_valid(out.1.nodes@, out.0@),
        forall|i: int| 0 <= i < out.0.len() ==> ground(out.1@[out.0@[i] as int])
            && ckc_spec::engine::args_of(out.1@[out.0@[i] as int]).len() == 2,
        root_terms(out.1.nodes@, out.0@) == crate::k2_engine::db_view(input_arena.nodes@, db@)
            .map_values(|c: ckc_spec::v1text::DocClause| ckc_spec::replay::clause_gids(c)).flatten(),
{
    let ghost origin = input_arena.nodes@;
    let ghost models = crate::k2_engine::db_view(origin, db@);
    let ghost parts = models.map_values(|c: ckc_spec::v1text::DocClause| ckc_spec::replay::clause_gids(c));
    let mut arena = input_arena;
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(root_terms(arena.nodes@, out@) == Seq::empty());
        assert_seqs_equal!(parts.take(0) == Seq::empty());
    }
    while i < db.len()
        invariant arena_ok(&arena), origin == input_arena.nodes@, origin.is_prefix_of(arena.nodes@),
            crate::k2_engine::db_valid(arena.nodes@, db@), models == crate::k2_engine::db_view(origin, db@),
            models == crate::k2_engine::db_view(arena.nodes@, db@),
            parts == models.map_values(|c: ckc_spec::v1text::DocClause| ckc_spec::replay::clause_gids(c)),
            i <= db.len(), roots_valid(arena.nodes@, out@),
            root_terms(arena.nodes@, out@) == parts.take(i as int).flatten(),
            forall|j: int| 0 <= j < out.len() ==> ground(arena@[out@[j] as int])
                && ckc_spec::engine::args_of(arena@[out@[j] as int]).len() == 2,
        decreases db.len() - i,
    {
        let ghost before = arena.nodes@;
        let ghost left = out@;
        proof { assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[i as int])); }
        let roots = crate::k2_walk::clause_walk_roots(&arena, &db[i]);
        let mut pairs = crate::k2_walk::gid_pairs_all_exec(&mut arena, &roots);
        proof {
            roots_models_prefix(before, arena.nodes@, left);
            crate::k2_engine::db_models_prefix(before, arena.nodes@, db@);
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
            assert forall|j: int| 0 <= j < pairs.len() implies ground(arena@[pairs@[j] as int])
                && ckc_spec::engine::args_of(arena@[pairs@[j] as int]).len() == 2 by {
                let t = arena@[pairs@[j] as int];
                assert(crate::k2_walk::gid_pair_shape(t));
                let args = ckc_spec::engine::args_of(t);
                assert_seqs_equal!(args == seq![args[0], args[1]]);
                pair_ground(args[0], args[1]);
                assert(t == ckc_spec::replay::pair(args[0], args[1]));
            }
        }
        let ghost right = pairs@;
        out.append(&mut pairs);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies out@[j] < arena.nodes.len()
                && ground(arena@[out@[j] as int]) && ckc_spec::engine::args_of(arena@[out@[j] as int]).len() == 2 by {
                if j < left.len() { assert(out@[j] == left[j]); }
                else { assert(out@[j] == right[j - left.len()]); }
            }
            assert_seqs_equal!(root_terms(arena.nodes@, out@) == root_terms(arena.nodes@, left) + root_terms(arena.nodes@, right));
            assert_seqs_equal!(parts.take(i as int + 1) == parts.take(i as int) + seq![parts[i as int]]);
            vstd::seq_lib::lemma_flatten_concat(parts.take(i as int), seq![parts[i as int]]);
            seq![parts[i as int]].lemma_flatten_singleton();
        }
        i += 1;
    }
    proof { assert_seqs_equal!(parts.take(i as int) == parts); }
    (out, arena)
}

fn loaded_pairs_exec(arena: &mut ETermArena, db: &Vec<crate::k2_engine::EClause>) -> (out: Vec<usize>)
    requires arena_ok(old(arena)), crate::k2_engine::db_valid(old(arena).nodes@, db@),
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        roots_valid(final(arena).nodes@, out@),
        forall|i: int| 0 <= i < out.len() ==> ground(final(arena)@[out@[i] as int])
            && ckc_spec::engine::args_of(final(arena)@[out@[i] as int]).len() == 2,
        root_terms(final(arena).nodes@, out@) == crate::k2_engine::db_view(old(arena).nodes@, db@)
            .map_values(|c: ckc_spec::v1text::DocClause| ckc_spec::replay::clause_gids(c)).flatten(),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (out, mut owned) = loaded_pairs_inner(owned, db);
    core::mem::swap(arena, &mut owned);
    out
}

fn pair_error(arena: &mut ETermArena, pair: usize, name: &[u8]) -> (out: ckc_spec::replay::EOut)
    requires crate::k2_term::root_ok(old(arena), pair),
        ckc_spec::engine::args_of(old(arena)@[pair as int]).len() == 2,
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == ckc_spec::replay::proof_fail(ckc_spec::term::Term::Comp(name@,
            ckc_spec::engine::args_of(old(arena)@[pair as int]))),
{
    let args = crate::k2_engine::args_roots(arena, pair);
    proof {
        assert_seqs_equal!(ckc_spec::engine::args_of(arena@[pair as int])
            == seq![arena@[args@[0] as int], arena@[args@[1] as int]]);
    }
    let detail = crate::k2_output::comp2(arena, name, args[0], args[1]);
    crate::k2_output::error_out(arena, detail, true)
}

pub fn coverage_check_exec(arena: &mut ETermArena, db: &Vec<crate::k2_engine::EClause>, pairs: &Vec<usize>,
    Ghost(obs): Ghost<Seq<Ob>>) -> (out: Option<ckc_spec::replay::EOut>)
    requires arena_ok(old(arena)), crate::k2_engine::db_valid(old(arena).nodes@, db@),
        roots_valid(old(arena).nodes@, pairs@), crate::k2_sort::roots_ground(old(arena), pairs@),
        root_terms(old(arena).nodes@, pairs@) == obs.map_values(|ob: Ob| ds(ob)),
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        crate::k2_output::option_out_view(out) == ckc_spec::replay::coverage_check(crate::k2_engine::db_view(old(arena).nodes@, db@), obs),
{
    let ghost origin = arena.nodes@;
    let all = loaded_pairs_exec(arena, db);
    proof {
        roots_models_prefix(origin, arena.nodes@, pairs@);
        sort_views(arena, all@); sort_views(arena, pairs@);
    }
    let loaded = crate::k2_sort::sort_unique(arena, &all);
    let covered = crate::k2_sort::sort_unique(arena, pairs);
    proof {
        sort_views(arena, loaded@); sort_views(arena, covered@);
        let shape = |t: ckc_spec::term::Term| ckc_spec::engine::args_of(t).len() == 2;
        assert forall|i: int| 0 <= i < all.len() implies shape(#[trigger] root_terms(arena.nodes@, all@)[i]) by {
            assert(root_terms(arena.nodes@, all@)[i] == arena@[all@[i] as int]);
        }
        assert forall|i: int| 0 <= i < pairs.len() implies shape(#[trigger] root_terms(arena.nodes@, pairs@)[i]) by {
            assert(root_terms(arena.nodes@, pairs@)[i] == ds(obs[i]));
        }
        crate::k2_sort::sort_unique_property(root_terms(arena.nodes@, all@), shape);
        crate::k2_sort::sort_unique_property(root_terms(arena.nodes@, pairs@), shape);
    }
    let missing_name: &[u8] = b"missing_obligation";
    let extra_name: &[u8] = b"extra_obligation";
    proof {
        reveal_byteslit(b"missing_obligation"); reveal_strlit("missing_obligation");
        reveal_byteslit(b"extra_obligation"); reveal_strlit("extra_obligation");
        reveal(ckc_spec::v1text::ascii);
        assert(missing_name@ == ckc_spec::v1text::ascii("missing_obligation"@));
        assert(extra_name@ == ckc_spec::v1text::ascii("extra_obligation"@));
    }
    let ghost before_error = arena.nodes@;
    if let Some(root) = first_absent_root(arena, &loaded, &covered) {
        proof {
            let index = choose|i: int| 0 <= i < loaded.len() && loaded@[i] == root;
            assert(root_terms(arena.nodes@, loaded@)[index] == arena@[root as int]);
        }
        let error = pair_error(arena, root, missing_name);
        proof { crate::k2_load::prefix_chain(origin, before_error, arena.nodes@); }
        return Some(error);
    }
    if let Some(root) = first_absent_root(arena, &covered, &loaded) {
        proof {
            let index = choose|i: int| 0 <= i < covered.len() && covered@[i] == root;
            assert(root_terms(arena.nodes@, covered@)[index] == arena@[root as int]);
        }
        let error = pair_error(arena, root, extra_name);
        proof { crate::k2_load::prefix_chain(origin, before_error, arena.nodes@); }
        return Some(error);
    }
    None
}

fn unproved_inner(input_arena: ETermArena, db: &Vec<crate::k2_engine::EClause>, obs: &Vec<EOb>)
    -> (out: (Option<usize>, ETermArena))
    requires arena_ok(&input_arena), crate::k2_engine::db_valid(input_arena.nodes@, db@),
        obs_valid(input_arena.nodes@, obs@),
    ensures arena_ok(&out.1), input_arena.nodes@.is_prefix_of(out.1.nodes@),
        out.0 matches Some(i) ==> i < obs.len(),
        crate::k2_payload::selected_ob(out.1.nodes@, obs@, out.0) == ckc_spec::replay::first_unproved(
            crate::k2_engine::db_view(input_arena.nodes@, db@), obs_view(input_arena.nodes@, obs@)),
{
    hide(ckc_spec::replay::first_unproved);
    let ghost origin = input_arena.nodes@;
    let ghost database = crate::k2_engine::db_view(origin, db@);
    let ghost models = obs_view(origin, obs@);
    let mut arena = input_arena;
    let mut i = 0usize;
    proof { assert_seqs_equal!(models.skip(0) == models); }
    while i < obs.len()
        invariant arena_ok(&arena), origin == input_arena.nodes@, origin.is_prefix_of(arena.nodes@),
            crate::k2_engine::db_valid(arena.nodes@, db@), obs_valid(arena.nodes@, obs@),
            database == crate::k2_engine::db_view(origin, db@), database == crate::k2_engine::db_view(arena.nodes@, db@),
            models == obs_view(origin, obs@), models == obs_view(arena.nodes@, obs@), i <= obs.len(),
            ckc_spec::replay::first_unproved(database, models)
                == ckc_spec::replay::first_unproved(database, models.skip(i as int)),
        decreases obs.len() - i,
    {
        let ghost before = arena.nodes@;
        proof {
            assert(ob_valid(arena.nodes@, &obs@[i as int]));
            assert(models[i as int] == ob_view(arena.nodes@, &obs@[i as int]));
            reveal_with_fuel(ckc_spec::replay::first_unproved, 1);
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
        }
        let witnessed = crate::k2_bridge::witness_db_exec(&arena, &obs[i].facts, db);
        let proved = crate::k2_engine::heads_proved(&mut arena, &witnessed, &obs[i].heads);
        proof {
            crate::k2_engine::db_models_prefix(before, arena.nodes@, db@);
            obs_prefix(before, arena.nodes@, obs@);
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
        }
        if !proved { return (Some(i), arena); }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(ckc_spec::replay::first_unproved, 1);
    }
    (None, arena)
}

pub fn first_unproved_exec(arena: &mut ETermArena, db: &Vec<crate::k2_engine::EClause>, obs: &Vec<EOb>)
    -> (out: Option<usize>)
    requires arena_ok(old(arena)), crate::k2_engine::db_valid(old(arena).nodes@, db@), obs_valid(old(arena).nodes@, obs@),
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(i) ==> i < obs.len(),
        crate::k2_payload::selected_ob(final(arena).nodes@, obs@, out) == ckc_spec::replay::first_unproved(
            crate::k2_engine::db_view(old(arena).nodes@, db@), obs_view(old(arena).nodes@, obs@)),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (out, mut owned) = unproved_inner(owned, db, obs);
    core::mem::swap(arena, &mut owned);
    out
}

} // verus!
