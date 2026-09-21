#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_ante::flatten_ante;
use crate::m6_custody::{nonempty_lines, ulex_matches};
use crate::m6_doc::sentence_count;
use crate::m6_drs::{box_parts, collides, inner_sentence};
use crate::m6_dump::parse_dump;
use crate::m6_expand::{all_pos, expand, pos_terms};
use crate::m6_flat::Env;
use crate::m6_group::error;
use crate::m6_model::*;
use crate::m6_normal::canon_pair;
use crate::m6_payload::{reject, reject_sym};
use crate::m6_query_drs::{scan_box, strip_box};
use crate::m6_query_markers::{answers_from, box_markers};
#[cfg(verus_keep_ghost)]
use crate::m6_query_markers::{answers_shape, marker_models};
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
#[cfg(verus_keep_ghost)]
use crate::m6_vars::nums;
use crate::m6_vars::{member, stream};
use crate::v1_term_impl::EV1Class;
#[cfg(verus_keep_ghost)]
use crate::v1_term_impl::*;
use ckc_spec::emit as spec;
use ckc_spec::replay::{self, EOut};
use ckc_spec::term::Term;
use ckc_spec::v1text::{self, QueryFile, V1File};
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub fn sentence_one(arena: &mut ETermArena, q: &T) -> (out: bool)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, q),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == (spec::inner_sentence(q@) == Some(1int)),
{
    match inner_sentence(arena, q) {
        None => false,
        Some(s) => {
            let ghost start = arena.nodes@;
            let one = crate::m6_term::int(arena, 1);
            proof {
                prefix(start, arena.nodes@, &s);
            }
            crate::m6_term::equal(arena, &s, &one)
        },
    }
}

pub fn conjunction(arena: &mut ETermArena, ts: &Vec<T>) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ts@),
        ts.len() > 0,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == ckc_spec::engine::conj_term(models(ts@)),
{
    let ghost start = arena.nodes@;
    let mut i = ts.len() - 1;
    let mut out = ts[i].cp();
    proof {
        reveal_with_fuel(ckc_spec::engine::conj_term, 1);
        reveal_strlit(",");
        reveal(v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::Comma) == ckc_spec::engine::comma_name());
    }
    while i > 0
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, ts@),
            valid(arena.nodes@, &out),
            i < ts.len(),
            out@ == ckc_spec::engine::conj_term(models(ts@).skip(i as int)),
        decreases i,
    {
        i -= 1;
        proof {
            assert(models(ts@).skip(i as int).drop_first() == models(ts@).skip(i as int + 1));
            reveal_with_fuel(ckc_spec::engine::conj_term, 1);
        }
        let ghost middle = arena.nodes@;
        out = c2(arena, &Sym::Comma, &ts[i], &out);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            prefix_all(middle, arena.nodes@, ts@);
        }
    }
    proof {
        assert(models(ts@).skip(0) == models(ts@));
    }
    out
}

pub fn answers_bound(arena: &ETermArena, goal: &T, answers: &Vec<T>) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, goal),
        valid_all(arena.nodes@, answers@),
        forall|i: int|
            0 <= i < answers.len() ==> spec::is_comp(#[trigger] models(answers@)[i], "answer"@, 2),
    ensures
        out == (forall|i: int|
            0 <= i < answers.len() ==> spec::vars_of(goal@).contains(
                spec::var_index(replay::arg(#[trigger] models(answers@)[i], 0)),
            )),
{
    let vars = stream(arena, goal);
    let mut i = 0usize;
    proof {
        nums(vars@).to_set_ensures();
    }
    while i < answers.len()
        invariant
            arena_ok(arena),
            valid(arena.nodes@, goal),
            valid_all(arena.nodes@, answers@),
            nums(vars@) == ckc_spec::term::var_stream(goal@),
            forall|j: int|
                0 <= j < answers.len() ==> spec::is_comp(
                    #[trigger] models(answers@)[j],
                    "answer"@,
                    2,
                ),
            i <= answers.len(),
            forall|j: int|
                0 <= j < i ==> spec::vars_of(goal@).contains(
                    spec::var_index(replay::arg(#[trigger] models(answers@)[j], 0)),
                ),
        decreases answers.len() - i,
    {
        proof {
            assert(spec::is_comp(models(answers@)[i as int], "answer"@, 2));
        }
        let a = args(arena, &answers[i]);
        let key = var_index(arena, &a[0]);
        if !member(&vars, key) {
            proof {
                assert(!spec::vars_of(goal@).contains(
                    spec::var_index(replay::arg(models(answers@)[i as int], 0)),
                ));
            }
            return false;
        }
        i += 1;
    }
    true
}

pub open spec fn query_result(r: Result<(T, T), T>) -> Result<(Term, Term), Term> {
    match r {
        Ok((g, a)) => Ok((g@, a@)),
        Err(e) => Err(e@),
    }
}

pub open spec fn query_result_valid(nodes: Seq<ENode>, r: &Result<(T, T), T>) -> bool {
    match r {
        Ok((g, a)) => valid(nodes, g) && valid(nodes, a),
        Err(e) => valid(nodes, e),
    }
}

pub fn project_query(arena: &mut ETermArena, drs: &T, qid: &Vec<u8>) -> (out: Result<(T, T), T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, drs),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        query_result_valid(final(arena).nodes@, &out),
        query_result(out) == spec::project_query(drs@, qid@),
{
    hide(spec::collides);
    hide(spec::scan_box);
    hide(spec::box_markers);
    hide(spec::answers_of);
    hide(spec::strip_box);
    hide(spec::flatten_list);
    hide(spec::expand);
    let ghost start = arena.nodes@;
    let root = match box_parts(arena, drs) {
        Some(b) => b,
        None => return Err(error(arena, &Sym::InvalidDrsShape, Ghost(start))),
    };
    if collides(arena, drs) {
        return Err(error(arena, &Sym::ReservedNameCollision, Ghost(start)));
    }
    if root.dom.len() != 0 || !is_comp(arena, &root.conds, &Sym::Cons, 2) {
        return Err(error(arena, &Sym::QueryRoot, Ghost(start)));
    }
    let conds = args(arena, &root.conds);
    if !is_nil(arena, &conds[1]) || !is_comp(arena, &conds[0], &Sym::Question, 1) {
        return Err(error(arena, &Sym::QueryRoot, Ghost(start)));
    }
    let qa = args(arena, &conds[0]);
    let q = qa[0].cp();
    if !sentence_one(arena, &q) {
        return Err(error(arena, &Sym::MixedOrMissingSentenceAnchors, Ghost(start)));
    }
    let ghost n1 = arena.nodes@;
    proof {
        prefix(start, n1, &q);
    }
    if !scan_box(arena, &q) {
        return Err(error(arena, &Sym::QueryUnsupported, Ghost(start)));
    }
    let markers = box_markers(arena, &q);
    let seen = Vec::new();
    proof {
        assert(marker_models(markers@).skip(0) == marker_models(markers@));
        assert_seqs_equal!(models(seen@) == Seq::<Term>::empty());
    }
    let answers_result = answers_from(arena, &markers, 0, &seen);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
    }
    let answers = match answers_result {
        Some(a) => a,
        None => return Err(error(arena, &Sym::QueryMarker, Ghost(start))),
    };
    proof {
        prefix(n1, n2, &q);
    }
    let clean = strip_box(arena, &q);
    let ghost n3 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n2, n3);
        prefix_all(n2, n3, answers@);
        prefix(start, n3, drs);
    }
    let b = match box_parts(arena, &clean) {
        Some(b) => b,
        None => return Err(error(arena, &Sym::InvalidDrsShape, Ghost(start))),
    };
    let base = nvars(arena, drs);
    let ghost n4 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n3, n4);
        prefix(n3, n4, &b.conds);
        prefix_all(n3, n4, answers@);
    }
    let env = Env { docid: qid.clone(), s: 1, base };
    let flat_result = flatten_ante(arena, &b.conds, &env, 1);
    let ghost n5 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n4, n5);
        prefix_all(n4, n5, answers@);
    }
    let flat = match flat_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let empty = Vec::new();
    proof {
        assert_seqs_equal!(map_model(empty@) == Seq::<(Term, Term)>::empty());
    }
    let goals_result = expand(arena, &flat.items, &empty, &empty);
    let ghost n6 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n5, n6);
        prefix_all(n5, n6, answers@);
    }
    let goals = match goals_result {
        Ok(g) => g,
        Err(e) => return Err(e),
    };
    if goals.len() == 0 || !all_pos(&goals) {
        return Err(error(arena, &Sym::QueryUnsupported, Ghost(start)));
    }
    let terms = pos_terms(arena, &goals);
    let goal = conjunction(arena, &terms);
    let ghost n7 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n6, n7);
        prefix_all(n6, n7, answers@);
        answers_shape(marker_models(markers@), Seq::empty());
    }
    if !answers_bound(arena, &goal, &answers) {
        return Err(error(arena, &Sym::QueryMarkerUnbound, Ghost(start)));
    }
    let answer_list = list(arena, &answers);
    proof {
        crate::k2_load::prefix_chain(start, n7, arena.nodes@);
        prefix(n7, arena.nodes@, &goal);
    }
    Ok((goal, answer_list))
}

pub fn query_text(line: &Vec<u8>) -> (out: Vec<u8>)
    ensures
        out@ == spec::query_text(line@),
{
    if line.len() > 0 && line[line.len() - 1] == 0x0D {
        slice_to_vec(&line[0..line.len() - 1])
    } else {
        line.clone()
    }
}

pub fn success(qid: &[u8]) -> (out: EOut)
    ensures
        out@ == replay::ok(v1text::ascii("ckc: certify ok "@) + qid@ + seq![0x0Au8]),
{
    let mut out = slice_to_vec(b"ckc: certify ok ");
    let mut id = slice_to_vec(qid);
    out.append(&mut id);
    out.push(0x0A);
    proof {
        reveal_byteslit(b"ckc: certify ok ");
        reveal_strlit("ckc: certify ok ");
        reveal(v1text::ascii);
        assert_seqs_equal!(out@ == v1text::ascii("ckc: certify ok "@) + qid@ + seq![0x0Au8]);
    }
    EOut { rc: 0, out, err: Vec::new() }
}

pub fn certify_query_impl(
    ace: &[u8],
    asha: &[u8],
    usha: Option<&Vec<u8>>,
    qid: &[u8],
    dump: &[u8],
    pl: &[u8],
) -> (out: EOut)
    ensures
        out@ == spec::certify_query_output(ace@, asha@, spec::opt_view(usha), qid@, dump@, pl@),
{
    hide(spec::project_query);
    hide(spec::canon_pair);
    let mut arena = crate::k2_reject::empty_arena();
    let parsed = match crate::v1_impl::v1_parse(pl, &mut arena) {
        Some(p) => p,
        None => return reject_sym(&mut arena, qid, &Sym::Noncanonical),
    };
    match &parsed.class {
        EV1Class::Query => {},
        _ => {
            proof {
                reveal(parsed_metadata_ok);
                assert(!(parsed@ is Query));
            }
            return reject_sym(&mut arena, qid, &Sym::RecordShape);
        },
    }
    let ghost query = choose|q: QueryFile| parsed@ == V1File::Query(q);
    proof {
        reveal(parsed_metadata_ok);
        assert(parsed@ is Query);
        assert(parsed@ == V1File::Query(query));
        reveal(parsed_certify_metadata_ok);
        reveal(parsed_v1_ok);
        reveal(v1text::wf_v1);
        reveal(v1text::wf_query);
        parsed_query_roots_elim(arena.nodes@, &parsed);
    }
    let actual_goal = from_root(&arena, parsed.goal_root);
    let actual_answers = from_root(&arena, parsed.answers_root);
    let ghost before_dump = arena.nodes@;
    let d = match parse_dump(dump, &mut arena) {
        Some(d) => d,
        None => return reject_sym(&mut arena, qid, &Sym::DumpNoncanonical),
    };
    proof {
        prefix(before_dump, arena.nodes@, &actual_goal);
        prefix(before_dump, arena.nodes@, &actual_answers);
    }
    let messages = from_root(&arena, d.messages);
    let lines = nonempty_lines(ace);
    if !is_nil(&arena, &messages) {
        return reject_sym(&mut arena, qid, &Sym::ApeMessages);
    }
    let count = match sentence_count(&arena, &d) {
        Some(c) => c,
        None => return reject_sym(&mut arena, qid, &Sym::QuerySentences),
    };
    if count != 1 || lines.len() != 1 {
        return reject_sym(&mut arena, qid, &Sym::QuerySentences);
    }
    if !bytes_eq(&parsed.qid, qid) {
        return reject_sym(&mut arena, qid, &Sym::Qid);
    }
    if !bytes_eq(&parsed.query_ace, asha) {
        return reject_sym(&mut arena, qid, &Sym::AceSha256);
    }
    if !ulex_matches(&parsed.query_ulex, usha, Ghost(query.ulex)) {
        return reject_sym(&mut arena, qid, &Sym::Ulex);
    }
    let text = query_text(&lines[0]);
    if !bytes_eq(&parsed.query_text, &text) {
        return reject_sym(&mut arena, qid, &Sym::QueryText);
    }
    let drs = from_root(&arena, d.drs);
    let ghost before_project = arena.nodes@;
    let (goal, answers) = match project_query(&mut arena, &drs, &parsed.qid) {
        Ok(pair) => pair,
        Err(e) => {
            let why = c1(&mut arena, &Sym::Unsupported, &e);
            return reject(&arena, qid, &why);
        },
    };
    proof {
        prefix(before_project, arena.nodes@, &actual_goal);
        prefix(before_project, arena.nodes@, &actual_answers);
    }
    let ghost before_canon = arena.nodes@;
    let (goal, answers) = canon_pair(&mut arena, &goal, &answers);
    proof {
        prefix(before_canon, arena.nodes@, &actual_goal);
        prefix(before_canon, arena.nodes@, &actual_answers);
    }
    if !crate::m6_term::equal(&arena, &goal, &actual_goal) || !crate::m6_term::equal(
        &arena,
        &answers,
        &actual_answers,
    ) {
        return reject_sym(&mut arena, qid, &Sym::Projection);
    }
    success(qid)
}

} // verus!
