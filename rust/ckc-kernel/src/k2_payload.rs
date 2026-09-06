use crate::k2_engine::{args_roots, literal_matches};
use crate::k2_term::{ENode, ETermArena};
use crate::k2_walk::list_items_exec;
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid, roots_models_prefix};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
use ckc_spec::replay::{EOut, ERow, ESrc, Ob};
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::{ob_term, wf_obs};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub struct EOb {
    pub docid: usize,
    pub sentence: usize,
    pub variant: usize,
    pub facts: Vec<usize>,
    pub heads: Vec<usize>,
}

pub open spec fn ob_view(nodes: Seq<ENode>, ob: &EOb) -> Ob {
    Ob {
        docid: nodes[ob.docid as int].term@,
        s: nodes[ob.sentence as int].term@,
        k: nodes[ob.variant as int].term@,
        facts: root_terms(nodes, ob.facts@),
        heads: root_terms(nodes, ob.heads@),
    }
}

pub open spec fn ob_valid(nodes: Seq<ENode>, ob: &EOb) -> bool {
    &&& ob.docid < nodes.len() && ob.sentence < nodes.len() && ob.variant < nodes.len()
    &&& roots_valid(nodes, ob.facts@) && roots_valid(nodes, ob.heads@)
}

pub open spec fn obs_view(nodes: Seq<ENode>, obs: Seq<EOb>) -> Seq<Ob> {
    Seq::new(obs.len(), |i: int| ob_view(nodes, &obs[i]))
}

pub open spec fn obs_valid(nodes: Seq<ENode>, obs: Seq<EOb>) -> bool {
    forall|i: int| 0 <= i < obs.len() ==> #[trigger] ob_valid(nodes, &obs[i])
}

pub proof fn ob_prefix(before: Seq<ENode>, after: Seq<ENode>, ob: &EOb)
    requires before.is_prefix_of(after), ob_valid(before, ob),
    ensures ob_valid(after, ob), ob_view(before, ob) == ob_view(after, ob),
{
    roots_models_prefix(before, after, ob.facts@);
    roots_models_prefix(before, after, ob.heads@);
}

pub proof fn obs_prefix(before: Seq<ENode>, after: Seq<ENode>, obs: Seq<EOb>)
    requires before.is_prefix_of(after), obs_valid(before, obs),
    ensures obs_valid(after, obs), obs_view(before, obs) == obs_view(after, obs),
{
    assert forall|i: int| 0 <= i < obs.len() implies {
        &&& ob_valid(after, &obs[i])
        &&& ob_view(before, &obs[i]) == ob_view(after, &obs[i])
    } by { ob_prefix(before, after, &obs[i]); }
    assert_seqs_equal!(obs_view(before, obs) == obs_view(after, obs));
}

fn extract_ob(arena: &ETermArena, root: usize, Ghost(expected): Ghost<Option<Ob>>) -> (out: Option<EOb>)
    requires
        root_ok(arena, root),
        expected matches Some(ob) ==> arena@[root as int] == ob_term(ob),
    ensures
        out matches Some(ob) ==> ob_valid(arena.nodes@, &ob)
            && ob_term(ob_view(arena.nodes@, &ob)) == arena@[root as int],
        expected matches Some(model) ==> out matches Some(ob) && ob_view(arena.nodes@, &ob) == model,
{
    let proof_name = slice_to_vec(b"$guideline_proof");
    let variant_name = slice_to_vec(b"variant");
    let witness_name = slice_to_vec(b"witness");
    let prove_name = slice_to_vec(b"prove");
    proof {
        reveal_byteslit(b"$guideline_proof"); reveal_strlit("$guideline_proof");
        reveal_byteslit(b"variant"); reveal_strlit("variant");
        reveal_byteslit(b"witness"); reveal_strlit("witness");
        reveal_byteslit(b"prove"); reveal_strlit("prove");
        reveal(ckc_spec::v1text::ascii);
        assert(proof_name@ == ckc_spec::v1text::ascii("$guideline_proof"@));
        assert(variant_name@ == ckc_spec::v1text::ascii("variant"@));
        assert(witness_name@ == ckc_spec::v1text::ascii("witness"@));
        assert(prove_name@ == ckc_spec::v1text::ascii("prove"@));
        if let Some(ob) = expected {
            crate::k2_walk::list_items_of_list(ob.facts);
            crate::k2_walk::list_items_of_list(ob.heads);
        }
    }
    if !literal_matches(arena, root, &proof_name, 5) { return None; }
    let args = args_roots(arena, root);
    let docid = args[0];
    let sentence = args[1];
    let variant_root = args[2];
    let witness_root = args[3];
    let prove_root = args[4];
    if !literal_matches(arena, variant_root, &variant_name, 1)
        || !literal_matches(arena, witness_root, &witness_name, 1)
        || !literal_matches(arena, prove_root, &prove_name, 1) { return None; }
    let variant_args = args_roots(arena, variant_root);
    let witness_args = args_roots(arena, witness_root);
    let prove_args = args_roots(arena, prove_root);
    let variant = variant_args[0];
    let facts_root = witness_args[0];
    let heads_root = prove_args[0];
    proof {
        if let Some(model) = expected {
            assert(arena@[docid as int] == model.docid);
            assert(arena@[sentence as int] == model.s);
            assert(arena@[variant_root as int] == Term::Comp(variant_name@, seq![model.k]));
            assert(arena@[witness_root as int] == Term::Comp(witness_name@,
                seq![ckc_spec::engine::list_term(model.facts)]));
            assert(arena@[prove_root as int] == Term::Comp(prove_name@,
                seq![ckc_spec::engine::list_term(model.heads)]));
            assert(root_terms(arena.nodes@, variant_args@) == seq![model.k]);
            assert(root_terms(arena.nodes@, witness_args@) == seq![ckc_spec::engine::list_term(model.facts)]);
            assert(root_terms(arena.nodes@, prove_args@) == seq![ckc_spec::engine::list_term(model.heads)]);
            assert(root_terms(arena.nodes@, variant_args@)[0] == arena@[variant as int]);
            assert(root_terms(arena.nodes@, witness_args@)[0] == arena@[facts_root as int]);
            assert(root_terms(arena.nodes@, prove_args@)[0] == arena@[heads_root as int]);
            assert(arena@[variant as int] == model.k);
            assert(arena@[facts_root as int] == ckc_spec::engine::list_term(model.facts));
            assert(arena@[heads_root as int] == ckc_spec::engine::list_term(model.heads));
            crate::k2_walk::list_items_of_list(model.facts);
            crate::k2_walk::list_items_of_list(model.heads);
        }
    }
    let facts = match list_items_exec(arena, facts_root) { Some(roots) => roots, None => return None };
    let heads = match list_items_exec(arena, heads_root) { Some(roots) => roots, None => return None };
    proof {
        crate::k2_walk::list_items_reconstruct(arena@[facts_root as int]);
        crate::k2_walk::list_items_reconstruct(arena@[heads_root as int]);
        assert_seqs_equal!(ckc_spec::engine::args_of(arena@[variant_root as int])
            == seq![arena@[variant as int]]);
        assert_seqs_equal!(ckc_spec::engine::args_of(arena@[witness_root as int])
            == seq![arena@[facts_root as int]]);
        assert_seqs_equal!(ckc_spec::engine::args_of(arena@[prove_root as int])
            == seq![arena@[heads_root as int]]);
        assert_seqs_equal!(ckc_spec::engine::args_of(arena@[root as int]) == seq![
            arena@[docid as int], arena@[sentence as int], arena@[variant_root as int],
            arena@[witness_root as int], arena@[prove_root as int]]);
    }
    Some(EOb { docid, sentence, variant, facts, heads })
}

pub open spec fn ob_at(bytes: Seq<u8>, start: int, ob: Ob) -> bool {
    let end = start + ckc_spec::v1text::term_line(ob_term(ob)).len();
    &&& 0 <= start < end <= bytes.len()
    &&& wf_obs(seq![ob])
    &&& bytes.subrange(start, end) == ckc_spec::v1text::term_line(ob_term(ob))
}

proof fn ground_keys_fit(term: Term)
    requires ckc_spec::term::ground(term),
    ensures crate::v1_term_impl::term_keys_fit(term),
    decreases term, 0int,
{
    reveal(ckc_spec::term::ground);
    reveal(crate::v1_term_impl::term_keys_fit);
    if let Term::Comp(_, args) = term { ground_keys_fit_all(args); }
}

proof fn ground_keys_fit_all(terms: Seq<Term>)
    requires ckc_spec::term::ground_all(terms),
    ensures crate::v1_term_impl::terms_keys_fit(terms),
    decreases terms, 1int,
{
    reveal_with_fuel(ckc_spec::term::ground_all, 1);
    reveal_with_fuel(crate::v1_term_impl::terms_keys_fit, 1);
    if terms.len() > 0 {
        ground_keys_fit(terms[0]);
        ground_keys_fit_all(terms.drop_first());
    }
}

pub proof fn obs_wf_at(obs: Seq<Ob>, i: int)
    requires wf_obs(obs), 0 <= i < obs.len(),
    ensures ckc_spec::term::wf_term(ob_term(obs[i])),
        ckc_spec::term::ground(ob_term(obs[i])),
        ckc_spec::term::no_dollar_var(ob_term(obs[i])),
{}

proof fn term_line_suffix(term: Term)
    ensures ckc_spec::v1text::term_line(term)
        == ckc_spec::v1text::term_bytes(term) + seq![0x2eu8, 0x0au8],
{
    reveal_strlit(".\n");
    reveal(ckc_spec::v1text::ascii);
}

proof fn ob_term_guide(bytes: Seq<u8>, start: int, ob: Ob)
    requires ob_at(bytes, start, ob),
    ensures
        crate::v1_term_impl::term_at(bytes, start,
            start + ckc_spec::v1text::term_bytes(ob_term(ob)).len(), ob_term(ob)),
        crate::v1_term_impl::term_keys_fit(ob_term(ob)),
        ckc_spec::v1text::term_bytes(ob_term(ob)).len() > 0,
        start + ckc_spec::v1text::term_bytes(ob_term(ob)).len() + 2 <= bytes.len(),
        bytes[start + ckc_spec::v1text::term_bytes(ob_term(ob)).len()] == 0x2eu8,
        bytes[start + ckc_spec::v1text::term_bytes(ob_term(ob)).len() + 1] == 0x0au8,
{
    let term = ob_term(ob);
    let printed = ckc_spec::v1text::term_bytes(term);
    let end = start + printed.len();
    obs_wf_at(seq![ob], 0);
    term_line_suffix(term);
    ground_keys_fit(term);
    reveal(ckc_spec::v1text::term_line);
    reveal_with_fuel(ckc_spec::v1text::term_bytes, 1);
    assert(bytes.subrange(start, end + 2) == printed + seq![0x2eu8, 0x0au8]);
    assert(bytes.subrange(start, end + 2)[printed.len() as int] == bytes[end]);
    assert(bytes.subrange(start, end + 2)[printed.len() as int + 1] == bytes[end + 1]);
    assert(bytes[end] == 0x2eu8);
    assert(bytes[end + 1] == 0x0au8);
    assert_seqs_equal!(bytes.subrange(start, end) == printed);
}

fn parse_ob_line(
    input_arena: ETermArena, bytes: &[u8], start: usize, Ghost(expected): Ghost<Option<Ob>>,
) -> (out: (Option<(EOb, usize)>, ETermArena))
    requires
        arena_ok(&input_arena), start < bytes.len(),
        expected matches Some(ob) ==> ob_at(bytes@, start as int, ob),
    ensures
        arena_ok(&out.1), input_arena.nodes@.is_prefix_of(out.1.nodes@),
        out.0 matches Some((ob, next)) ==> {
            &&& start < next <= bytes.len()
            &&& ob_valid(out.1.nodes@, &ob)
            &&& wf_obs(seq![ob_view(out.1.nodes@, &ob)])
            &&& bytes@.subrange(start as int, next as int)
                == ckc_spec::v1text::term_line(ob_term(ob_view(out.1.nodes@, &ob)))
        },
        expected matches Some(model) ==> out.0 matches Some((ob, next))
            && ob_view(out.1.nodes@, &ob) == model
            && next == start + ckc_spec::v1text::term_line(ob_term(model)).len(),
{
    let mut arena = input_arena;
    let ghost guided = match expected {
        Some(ob) => Some(crate::v1_term_impl::GTermExpected {
            term: ob_term(ob),
            end: (start + ckc_spec::v1text::term_bytes(ob_term(ob)).len()) as usize,
        }),
        None => None,
    };
    proof { if let Some(ob) = expected { ob_term_guide(bytes@, start as int, ob); } }
    let mut tracker = crate::v1_term_impl::EVarTracker {
        next: 0, stream: Ghost(Seq::empty()), valid: true,
    };
    proof {
        reveal(crate::v1_term_impl::tracker_state_ok);
        reveal_with_fuel(ckc_spec::term::firsts, 1);
        reveal_with_fuel(crate::v1_term_impl::seen_after, 1);
        reveal(crate::v1_term_impl::nat_prefix);
        reveal_with_fuel(crate::v1_term_impl::canonical_seen, 1);
        assert_seqs_equal!(crate::v1_term_impl::nat_prefix(0) == Seq::empty());
    }
    let mut at = start;
    let parsed = match crate::v1_term_impl::parse_term(bytes, start, &mut arena, Ghost(guided),
        Ghost(crate::v1_term_impl::GTermExpected { term: Term::Nil, end: 0 }),
        Ghost(Seq::empty()), false, &mut tracker, &mut at) {
        Some(parsed) => parsed,
        None => return (None, arena),
    };
    proof {
        if let Some(model) = expected {
            obs_wf_at(seq![model], 0);
            term_line_suffix(ob_term(model));
        }
    }
    if !parsed.parsed.ground || !parsed.parsed.no_dollar { return (None, arena); }
    if parsed.end >= bytes.len() || bytes[parsed.end] != 0x2e { return (None, arena); }
    if parsed.end + 1 >= bytes.len() || bytes[parsed.end + 1] != 0x0a { return (None, arena); }
    let next = parsed.end + 2;
    let ob = match extract_ob(&arena, parsed.root, Ghost(expected)) {
        Some(ob) => ob,
        None => return (None, arena),
    };
    proof {
        let model = ob_view(arena.nodes@, &ob);
        assert(ob_term(model) == parsed@);
        assert(wf_obs(seq![model]));
        term_line_suffix(parsed@);
        assert_seqs_equal!(bytes@.subrange(start as int, next as int)
            == ckc_spec::v1text::term_line(ob_term(model)));
    }
    (Some((ob, next)), arena)
}

pub proof fn payload_print_concat(left: Seq<Ob>, right: Seq<Ob>)
    ensures ckc_spec::replay::print_payload(left + right)
        == ckc_spec::replay::print_payload(left) + ckc_spec::replay::print_payload(right),
{
    let lines = |ob: Ob| ckc_spec::v1text::term_line(ob_term(ob));
    assert_seqs_equal!((left + right).map_values(lines)
        == left.map_values(lines) + right.map_values(lines));
    vstd::seq_lib::lemma_flatten_concat(left.map_values(lines), right.map_values(lines));
}

proof fn payload_print_step(obs: Seq<Ob>)
    requires obs.len() > 0,
    ensures ckc_spec::replay::print_payload(obs)
        == ckc_spec::v1text::term_line(ob_term(obs[0]))
            + ckc_spec::replay::print_payload(obs.drop_first()),
{
    assert_seqs_equal!(seq![obs[0]] + obs.drop_first() == obs);
    payload_print_concat(seq![obs[0]], obs.drop_first());
    assert_seqs_equal!(seq![obs[0]].map_values(|ob: Ob| ckc_spec::v1text::term_line(ob_term(ob)))
        == seq![ckc_spec::v1text::term_line(ob_term(obs[0]))]);
    seq![ckc_spec::v1text::term_line(ob_term(obs[0]))].lemma_flatten_singleton();
}

pub proof fn payload_print_push(obs: Seq<Ob>, ob: Ob)
    ensures ckc_spec::replay::print_payload(obs.push(ob))
        == ckc_spec::replay::print_payload(obs) + ckc_spec::v1text::term_line(ob_term(ob)),
{
    assert_seqs_equal!(obs + seq![ob] == obs.push(ob));
    payload_print_concat(obs, seq![ob]);
    assert_seqs_equal!(seq![ob].map_values(|o: Ob| ckc_spec::v1text::term_line(ob_term(o)))
        == seq![ckc_spec::v1text::term_line(ob_term(ob))]);
    seq![ckc_spec::v1text::term_line(ob_term(ob))].lemma_flatten_singleton();
}

proof fn payload_print_nonempty(obs: Seq<Ob>)
    requires obs.len() > 0,
    ensures ckc_spec::replay::print_payload(obs).len() > 0,
{
    payload_print_step(obs);
    term_line_suffix(ob_term(obs[0]));
}

proof fn payload_head_at(bytes: Seq<u8>, start: int, obs: Seq<Ob>)
    requires 0 <= start < bytes.len(), wf_obs(obs),
        bytes.skip(start) == ckc_spec::replay::print_payload(obs),
    ensures obs.len() > 0, ob_at(bytes, start, obs[0]),
        bytes.skip(start + ckc_spec::v1text::term_line(ob_term(obs[0])).len())
            == ckc_spec::replay::print_payload(obs.drop_first()),
{
    if obs.len() == 0 {
        assert_seqs_equal!(obs.map_values(|o: Ob| ckc_spec::v1text::term_line(ob_term(o)))
            == Seq::empty());
        assert(false);
    }
    payload_print_step(obs);
    obs_wf_at(obs, 0);
    term_line_suffix(ob_term(obs[0]));
    let line = ckc_spec::v1text::term_line(ob_term(obs[0]));
    let rest = ckc_spec::replay::print_payload(obs.drop_first());
    assert(bytes.skip(start) == line + rest);
    assert(bytes.skip(start).len() == bytes.len() - start);
    assert(start + line.len() <= bytes.len());
    assert(wf_obs(seq![obs[0]]));
    assert_seqs_equal!(bytes.subrange(start, start + line.len()) == bytes.skip(start).take(line.len() as int));
    assert_seqs_equal!(bytes.skip(start).take(line.len() as int) == line);
    assert_seqs_equal!(bytes.skip(start + line.len()) == bytes.skip(start).skip(line.len() as int));
    assert_seqs_equal!(bytes.skip(start).skip(line.len() as int) == rest);
}

proof fn selected_payload(bytes: Seq<u8>)
    requires ckc_spec::replay::payload_accepts(bytes),
    ensures wf_obs(ckc_spec::replay::the_payload(bytes)),
        ckc_spec::replay::print_payload(ckc_spec::replay::the_payload(bytes)) == bytes,
{
    reveal(ckc_spec::replay::payload_accepts);
    reveal(ckc_spec::replay::the_payload);
}

fn parse_payload_inner(input_arena: ETermArena, bytes: &[u8])
    -> (out: (Option<Vec<EOb>>, ETermArena))
    requires arena_ok(&input_arena),
    ensures
        arena_ok(&out.1), input_arena.nodes@.is_prefix_of(out.1.nodes@),
        out.0.is_some() == ckc_spec::replay::payload_accepts(bytes@),
        out.0 matches Some(obs) ==> obs_valid(out.1.nodes@, obs@)
            && wf_obs(obs_view(out.1.nodes@, obs@))
            && obs_view(out.1.nodes@, obs@) == ckc_spec::replay::the_payload(bytes@),
{
    hide(ckc_spec::replay::payload_accepts);
    hide(ckc_spec::replay::the_payload);
    hide(ckc_spec::replay::print_payload);
    let ghost origin = input_arena.nodes@;
    let ghost canonical = ckc_spec::replay::payload_accepts(bytes@);
    let ghost models = ckc_spec::replay::the_payload(bytes@);
    let mut arena = input_arena;
    let mut out = Vec::new();
    let mut pos = 0usize;
    proof {
        assert_seqs_equal!(obs_view(arena.nodes@, out@) == Seq::empty());
        assert_seqs_equal!(bytes@.take(0) == Seq::empty());
        reveal(ckc_spec::replay::print_payload);
        assert_seqs_equal!(Seq::<Ob>::empty().map_values(|o: Ob| ckc_spec::v1text::term_line(ob_term(o)))
            == Seq::empty());
        if canonical {
            selected_payload(bytes@);
            assert_seqs_equal!(models.take(0) == Seq::empty());
            assert_seqs_equal!(models.skip(0) == models);
            assert_seqs_equal!(bytes@.skip(0) == bytes@);
        }
    }
    while pos < bytes.len()
        invariant
            arena_ok(&arena), origin == input_arena.nodes@, origin.is_prefix_of(arena.nodes@),
            pos <= bytes.len(), obs_valid(arena.nodes@, out@), wf_obs(obs_view(arena.nodes@, out@)),
            bytes@.take(pos as int) == ckc_spec::replay::print_payload(obs_view(arena.nodes@, out@)),
            canonical == ckc_spec::replay::payload_accepts(bytes@),
            models == ckc_spec::replay::the_payload(bytes@),
            canonical ==> wf_obs(models) && out.len() <= models.len()
                && obs_view(arena.nodes@, out@) == models.take(out.len() as int)
                && bytes@.skip(pos as int) == ckc_spec::replay::print_payload(models.skip(out.len() as int)),
        decreases bytes.len() - pos,
    {
        let ghost before_nodes = arena.nodes@;
        let ghost before_out = out@;
        let ghost before_models = obs_view(before_nodes, before_out);
        let ghost expected = if canonical { Some(models[out.len() as int]) } else { None };
        proof {
            if canonical {
                let rest = models.skip(out.len() as int);
                assert forall|i: int| 0 <= i < rest.len() implies
                    ckc_spec::term::wf_term(ob_term(rest[i]))
                        && ckc_spec::term::ground(ob_term(rest[i]))
                        && ckc_spec::term::no_dollar_var(ob_term(rest[i])) by {
                    obs_wf_at(models, out.len() as int + i);
                }
                payload_head_at(bytes@, pos as int, rest);
                assert(rest[0] == models[out.len() as int]);
            }
        }
        let (parsed, next_arena) = parse_ob_line(arena, bytes, pos, Ghost(expected));
        arena = next_arena;
        proof {
            obs_prefix(before_nodes, arena.nodes@, out@);
            crate::k2_load::prefix_chain(origin, before_nodes, arena.nodes@);
        }
        let (ob, next) = match parsed { Some(pair) => pair, None => return (None, arena) };
        let ghost model = ob_view(arena.nodes@, &ob);
        out.push(ob);
        proof {
            assert forall|i: int| 0 <= i < out@.len() implies ob_valid(arena.nodes@, &out@[i]) by {
                if i < before_out.len() { assert(out@[i] == before_out[i]); }
            }
            assert_seqs_equal!(obs_view(arena.nodes@, out@) == before_models.push(model), i => {
                if i < before_out.len() { assert(out@[i] == before_out[i]); }
            });
            assert forall|i: int| 0 <= i < out@.len() implies
                ckc_spec::term::wf_term(ob_term(obs_view(arena.nodes@, out@)[i]))
                    && ckc_spec::term::ground(ob_term(obs_view(arena.nodes@, out@)[i]))
                    && ckc_spec::term::no_dollar_var(ob_term(obs_view(arena.nodes@, out@)[i])) by {
                if i < before_out.len() { obs_wf_at(before_models, i); }
                else { obs_wf_at(seq![model], 0); }
            }
            payload_print_push(before_models, model);
            assert_seqs_equal!(bytes@.take(next as int)
                == bytes@.take(pos as int) + bytes@.subrange(pos as int, next as int));
            if canonical {
                assert_seqs_equal!(models.take(out.len() as int)
                    == models.take(before_out.len() as int).push(model));
                assert_seqs_equal!(models.skip(before_out.len() as int).drop_first()
                    == models.skip(out.len() as int));
            }
        }
        pos = next;
    }
    proof {
        assert_seqs_equal!(bytes@.take(pos as int) == bytes@);
        reveal(ckc_spec::replay::payload_accepts);
        assert(exists|obs: Seq<Ob>| #[trigger] wf_obs(obs)
            && ckc_spec::replay::print_payload(obs) == bytes@) by {
            assert(wf_obs(obs_view(arena.nodes@, out@)));
        }
        assert(canonical);
        let rest = models.skip(out.len() as int);
        if rest.len() > 0 { payload_print_nonempty(rest); assert(false); }
        assert(out.len() == models.len());
        assert_seqs_equal!(models.take(out.len() as int) == models);
    }
    (Some(out), arena)
}

pub fn parse_payload_file(arena: &mut ETermArena, bytes: &[u8]) -> (out: Option<Vec<EOb>>)
    requires arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out.is_some() == ckc_spec::replay::payload_accepts(bytes@),
        out matches Some(obs) ==> obs_valid(final(arena).nodes@, obs@)
            && wf_obs(obs_view(final(arena).nodes@, obs@))
            && obs_view(final(arena).nodes@, obs@) == ckc_spec::replay::the_payload(bytes@),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (out, mut owned) = parse_payload_inner(owned, bytes);
    core::mem::swap(arena, &mut owned);
    out
}

pub open spec fn selected_ob(nodes: Seq<ENode>, obs: Seq<EOb>, index: Option<usize>) -> Option<Ob> {
    match index { Some(i) => Some(ob_view(nodes, &obs[i as int])), None => None }
}

fn first_empty_index(arena: &ETermArena, obs: &Vec<EOb>) -> (out: Option<usize>)
    requires obs_valid(arena.nodes@, obs@),
    ensures out matches Some(i) ==> i < obs.len(),
        selected_ob(arena.nodes@, obs@, out) == ckc_spec::replay::first_empty(obs_view(arena.nodes@, obs@)),
{
    hide(ckc_spec::replay::first_empty);
    let ghost models = obs_view(arena.nodes@, obs@);
    let mut i = 0usize;
    proof { assert_seqs_equal!(models.skip(0) == models); }
    while i < obs.len()
        invariant obs_valid(arena.nodes@, obs@), i <= obs.len(),
            models == obs_view(arena.nodes@, obs@),
            ckc_spec::replay::first_empty(models) == ckc_spec::replay::first_empty(models.skip(i as int)),
        decreases obs.len() - i,
    {
        proof {
            assert(models[i as int] == ob_view(arena.nodes@, &obs@[i as int]));
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
            reveal_with_fuel(ckc_spec::replay::first_empty, 1);
        }
        if obs[i].heads.len() == 0 { return Some(i); }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(ckc_spec::replay::first_empty, 1);
    }
    None
}

pub fn obligation_error(arena: &mut ETermArena, ob: &EOb, name: &[u8]) -> (out: EOut)
    requires arena_ok(old(arena)), ob_valid(old(arena).nodes@, ob),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == ckc_spec::replay::proof_fail(Term::Comp(name@, seq![
            ob_view(old(arena).nodes@, ob).docid, ob_view(old(arena).nodes@, ob).s,
            ckc_spec::replay::variant(ob_view(old(arena).nodes@, ob))])),
{
    let variant_name: &[u8] = b"variant";
    proof {
        reveal_byteslit(b"variant"); reveal_strlit("variant");
        reveal(ckc_spec::v1text::ascii);
        assert(variant_name@ == ckc_spec::v1text::ascii("variant"@));
    }
    let variant = crate::k2_output::comp1(arena, variant_name, ob.variant);
    let detail = crate::k2_output::comp3(arena, name, ob.docid, ob.sentence, variant);
    crate::k2_output::error_out(arena, detail, true)
}

pub open spec fn payload_stage_view(nodes: Seq<ENode>, out: Result<Vec<EOb>, EOut>)
    -> ckc_spec::replay::Stage
{
    match out {
        Ok(obs) => ckc_spec::replay::Stage::Obs(obs_view(nodes, obs@)),
        Err(out) => ckc_spec::replay::Stage::Fail(out@),
    }
}

fn payload_stage_inner(input_arena: ETermArena, rows: &Vec<ERow>, pys: &Vec<ESrc>)
    -> (out: (Result<Vec<EOb>, EOut>, ETermArena))
    requires arena_ok(&input_arena), rows.len() == pys.len(),
    ensures
        arena_ok(&out.1), input_arena.nodes@.is_prefix_of(out.1.nodes@),
        out.0 matches Ok(obs) ==> obs_valid(out.1.nodes@, obs@) && wf_obs(obs_view(out.1.nodes@, obs@)),
        payload_stage_view(out.1.nodes@, out.0) == ckc_spec::replay::payload_stage(
            rows@.map_values(|r: ERow| r@), ckc_spec::replay::srcs(pys@), 0, Seq::empty()),
{
    hide(ckc_spec::replay::payload_stage);
    let ghost origin = input_arena.nodes@;
    let ghost models = rows@.map_values(|r: ERow| r@);
    let ghost cells = ckc_spec::replay::srcs(pys@);
    let mut arena = input_arena;
    let mut out = Vec::new();
    let mut i = 0usize;
    let payload_name: &[u8] = b"payload_term";
    let empty_name: &[u8] = b"empty_obligation";
    proof {
        reveal_byteslit(b"payload_term"); reveal_strlit("payload_term");
        reveal_byteslit(b"empty_obligation"); reveal_strlit("empty_obligation");
        reveal(ckc_spec::v1text::ascii);
        assert(payload_name@ == ckc_spec::v1text::ascii("payload_term"@));
        assert(empty_name@ == ckc_spec::v1text::ascii("empty_obligation"@));
        assert_seqs_equal!(obs_view(arena.nodes@, out@) == Seq::empty());
    }
    while i < rows.len()
        invariant
            arena_ok(&arena), origin == input_arena.nodes@, origin.is_prefix_of(arena.nodes@),
            rows.len() == pys.len(), i <= rows.len(),
            models == rows@.map_values(|r: ERow| r@), cells == ckc_spec::replay::srcs(pys@),
            obs_valid(arena.nodes@, out@), wf_obs(obs_view(arena.nodes@, out@)),
            ckc_spec::replay::payload_stage(models, cells, 0, Seq::empty())
                == ckc_spec::replay::payload_stage(models, cells, i as nat, obs_view(arena.nodes@, out@)),
            payload_name@ == ckc_spec::v1text::ascii("payload_term"@),
            empty_name@ == ckc_spec::v1text::ascii("empty_obligation"@),
        decreases rows.len() - i,
    {
        let ghost before_nodes = arena.nodes@;
        let ghost before_out = out@;
        let ghost before_models = obs_view(before_nodes, before_out);
        proof { reveal_with_fuel(ckc_spec::replay::payload_stage, 1); }
        let raw: &[u8] = match &pys[i] {
            ESrc::Bad(off) => return (Err(crate::k2_reject::utf8_out(*off)), arena),
            ESrc::Bytes(bytes) => bytes.as_slice(),
            ESrc::Missing => b"",
        };
        proof { assert(raw@ == ckc_spec::replay::src_bytes(cells[i as int])); }
        let mut parsed = match parse_payload_file(&mut arena, raw) {
            Some(obs) => obs,
            None => return (Err(crate::k2_output::named_atom_error(payload_name, &rows[i].payload, false)), arena),
        };
        proof {
            obs_prefix(before_nodes, arena.nodes@, before_out);
            crate::k2_load::prefix_chain(origin, before_nodes, arena.nodes@);
        }
        if let Some(index) = first_empty_index(&arena, &parsed) {
            proof { assert(ob_valid(arena.nodes@, &parsed@[index as int])); }
            let ghost before_error = arena.nodes@;
            let error = obligation_error(&mut arena, &parsed[index], empty_name);
            proof { crate::k2_load::prefix_chain(origin, before_error, arena.nodes@); }
            return (Err(error), arena);
        }
        let ghost right = parsed@;
        let ghost right_models = obs_view(arena.nodes@, right);
        out.append(&mut parsed);
        proof {
            assert forall|j: int| 0 <= j < out@.len() implies ob_valid(arena.nodes@, &out@[j]) by {
                if j < before_out.len() { assert(out@[j] == before_out[j]); }
                else { assert(out@[j] == right[j - before_out.len()]); }
            }
            assert_seqs_equal!(obs_view(arena.nodes@, out@) == before_models + right_models, j => {
                if j < before_out.len() { assert(out@[j] == before_out[j]); }
                else { assert(out@[j] == right[j - before_out.len()]); }
            });
            assert forall|j: int| 0 <= j < out@.len() implies
                ckc_spec::term::wf_term(ob_term(obs_view(arena.nodes@, out@)[j]))
                    && ckc_spec::term::ground(ob_term(obs_view(arena.nodes@, out@)[j]))
                    && ckc_spec::term::no_dollar_var(ob_term(obs_view(arena.nodes@, out@)[j])) by {
                if j < before_out.len() { obs_wf_at(before_models, j); }
                else { obs_wf_at(right_models, j - before_out.len()); }
            }
        }
        i += 1;
    }
    proof { reveal_with_fuel(ckc_spec::replay::payload_stage, 1); }
    (Ok(out), arena)
}

pub fn payload_stage_exec(arena: &mut ETermArena, rows: &Vec<ERow>, pys: &Vec<ESrc>)
    -> (out: Result<Vec<EOb>, EOut>)
    requires arena_ok(old(arena)), rows.len() == pys.len(),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(obs) ==> obs_valid(final(arena).nodes@, obs@) && wf_obs(obs_view(final(arena).nodes@, obs@)),
        payload_stage_view(final(arena).nodes@, out) == ckc_spec::replay::payload_stage(
            rows@.map_values(|r: ERow| r@), ckc_spec::replay::srcs(pys@), 0, Seq::empty()),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (out, mut owned) = payload_stage_inner(owned, rows, pys);
    core::mem::swap(arena, &mut owned);
    out
}

} // verus!
