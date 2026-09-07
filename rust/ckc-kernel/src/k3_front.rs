use crate::k2_answers::EQuery;
#[cfg(verus_keep_ghost)]
use crate::k2_answers::{query_ok, query_prefix};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid};
use crate::k2_load::ELoaded;
#[cfg(verus_keep_ghost)]
use crate::k2_load::loaded_ok;
#[cfg(verus_keep_ghost)]
use crate::k2_output::option_out_view;
use crate::k2_output::{atom_root, comp1, comp3, error_out, int_root};
use crate::k2_term::{ENode, ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
use crate::v1_term_impl::EV1Class;
#[cfg(verus_keep_ghost)]
use crate::v1_term_impl::parsed_query_roots_elim;
use ckc_spec::replay::{EOut, ESrc};
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::{Out, cells_ok, srcs};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::trace::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::ascii;
use ckc_spec::v1text::{AnswersFile, QueryFile, TracesFile, V1File};
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub fn bytes_equal(left: &Vec<u8>, right: &[u8]) -> (out: bool)
    ensures
        out == (left@ == right@),
{
    crate::k2_engine::vec_equal(left, &slice_to_vec(right))
}

pub fn is_atom(arena: &ETermArena, root: usize, name: &[u8]) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == (arena@[root as int] == Term::Atom(name@)),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Atom { name: stored } => bytes_equal(stored, name),
        _ => false,
    }
}

pub fn is_comp(arena: &ETermArena, root: usize, name: &[u8], arity: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == match arena@[root as int] {
            Term::Comp(n, args) => n == name@ && args.len() == arity,
            _ => false,
        },
{
    crate::k2_engine::literal_matches(arena, root, &slice_to_vec(name), arity)
}

fn limit_shape(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == (arena@[root as int] == ckc_spec::answers::limit()),
{
    let outer: &[u8] = b"indeterminate";
    let inner: &[u8] = b"limit";
    proof {
        reveal_byteslit(b"indeterminate");
        reveal_strlit("indeterminate");
        reveal_byteslit(b"limit");
        reveal_strlit("limit");
        reveal(ascii);
        assert(outer@ == ascii("indeterminate"@));
        assert(inner@ == ascii("limit"@));
    }
    if !is_comp(arena, root, outer, 1) {
        return false;
    }
    let args = crate::k2_engine::args_roots(arena, root);
    proof {
        assert(args.len() == 1);
        assert(arena@[args@[0] as int] == ckc_spec::engine::args_of(arena@[root as int])[0]);
        assert(arena@[root as int] == Term::Comp(
            outer@,
            ckc_spec::engine::args_of(arena@[root as int]),
        ));
    }
    let found = is_atom(arena, args[0], inner);
    proof {
        if found {
            assert_seqs_equal!(ckc_spec::engine::args_of(arena@[root as int]) == seq![Term::Atom(inner@)]);
            assert(arena@[root as int] == Term::Comp(outer@, seq![Term::Atom(inner@)]));
        } else {
            assert(ckc_spec::engine::args_of(arena@[root as int])[0] != Term::Atom(inner@));
            assert(arena@[root as int] != Term::Comp(outer@, seq![Term::Atom(inner@)]));
        }
    }
    found
}

fn yesno_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == yesno_shape(arena@[root as int]),
{
    let yes: &[u8] = b"yes";
    let no: &[u8] = b"no";
    let failure: &[u8] = b"finite_failure";
    proof {
        reveal_byteslit(b"yes");
        reveal_strlit("yes");
        reveal_byteslit(b"no");
        reveal_strlit("no");
        reveal_byteslit(b"finite_failure");
        reveal_strlit("finite_failure");
        reveal(ascii);
        assert(yes@ == ascii("yes"@));
        assert(no@ == ascii("no"@));
        assert(failure@ == ascii("finite_failure"@));
    }
    if is_atom(arena, root, yes) || limit_shape(arena, root) {
        return true;
    }
    if !is_comp(arena, root, no, 1) {
        return false;
    }
    let args = crate::k2_engine::args_roots(arena, root);
    proof {
        assert(args.len() == 1);
        assert(arena@[args@[0] as int] == ckc_spec::engine::args_of(arena@[root as int])[0]);
        assert(arena@[root as int] == Term::Comp(
            no@,
            ckc_spec::engine::args_of(arena@[root as int]),
        ));
    }
    let found = is_atom(arena, args[0], failure);
    proof {
        if found {
            assert_seqs_equal!(ckc_spec::engine::args_of(arena@[root as int]) == seq![Term::Atom(failure@)]);
            assert(arena@[root as int] == Term::Comp(no@, seq![Term::Atom(failure@)]));
        } else {
            assert(ckc_spec::engine::args_of(arena@[root as int])[0] != Term::Atom(failure@));
            assert(arena@[root as int] != Term::Comp(no@, seq![Term::Atom(failure@)]));
        }
    }
    found
}

pub fn file_error(arena: &mut ETermArena, why: usize, trace: bool) -> (out: EOut)
    requires
        root_ok(old(arena), why),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == if trace {
            trace_reject(old(arena)@[why as int])
        } else {
            answers_reject(old(arena)@[why as int])
        },
{
    let name: &[u8] = if trace {
        b"trace_file"
    } else {
        b"answers_file"
    };
    proof {
        reveal_byteslit(b"trace_file");
        reveal_strlit("trace_file");
        reveal_byteslit(b"answers_file");
        reveal_strlit("answers_file");
        reveal(ascii);
        assert(name@ == if trace {
            ascii("trace_file"@)
        } else {
            ascii("answers_file"@)
        });
    }
    let detail = comp1(arena, name, why);
    error_out(arena, detail, false)
}

pub fn file_atom_error(why: &[u8], trace: bool) -> (out: EOut)
    ensures
        out@ == if trace {
            trace_reject(Term::Atom(why@))
        } else {
            answers_reject(Term::Atom(why@))
        },
{
    let mut arena = crate::k2_reject::empty_arena();
    let why = atom_root(&mut arena, why);
    file_error(&mut arena, why, trace)
}

fn count_error(count: usize) -> (out: EOut)
    ensures
        out@ == answers_reject(Term::Comp(ascii("term_count"@), seq![Term::Int(count as int)])),
{
    let name: &[u8] = b"term_count";
    proof {
        reveal_byteslit(b"term_count");
        reveal_strlit("term_count");
        reveal(ascii);
        assert(name@ == ascii("term_count"@));
    }
    let mut arena = crate::k2_reject::empty_arena();
    let count = int_root(&mut arena, count);
    let why = comp1(&mut arena, name, count);
    file_error(&mut arena, why, false)
}

fn sol_error(index: usize, why: &[u8]) -> (out: EOut)
    ensures
        out@ == answers_reject(Term::Comp(why@, seq![Term::Int(index as int)])),
{
    let mut arena = crate::k2_reject::empty_arena();
    let index = int_root(&mut arena, index);
    let why = comp1(&mut arena, why, index);
    file_error(&mut arena, why, false)
}

fn arity_error(index: usize, arity: usize, actual: usize) -> (out: EOut)
    ensures
        out@ == answers_reject(
            Term::Comp(
                ascii("solution_arity"@),
                seq![Term::Int(index as int), Term::Int(arity as int), Term::Int(actual as int)],
            ),
        ),
{
    let name: &[u8] = b"solution_arity";
    proof {
        reveal_byteslit(b"solution_arity");
        reveal_strlit("solution_arity");
        reveal(ascii);
        assert(name@ == ascii("solution_arity"@));
    }
    let mut arena = crate::k2_reject::empty_arena();
    let index = int_root(&mut arena, index);
    let arity = int_root(&mut arena, arity);
    let actual = int_root(&mut arena, actual);
    let why = comp3(&mut arena, name, index, arity, actual);
    file_error(&mut arena, why, false)
}

pub open spec fn checked_out(r: Option<Term>) -> Option<Out> {
    match r {
        Some(why) => Some(answers_reject(why)),
        None => None,
    }
}

fn sols_check_exec(arena: &ETermArena, rows: &Vec<usize>, arity: usize) -> (out: Option<EOut>)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, rows@),
    ensures
        option_out_view(out) == checked_out(
            sols_check(root_terms(arena.nodes@, rows@), 1, arity as nat),
        ),
{
    hide(sols_check);
    let ghost models = root_terms(arena.nodes@, rows@);
    let sol: &[u8] = b"sol";
    let shape: &[u8] = b"solution_shape";
    let values: &[u8] = b"solution_values";
    proof {
        reveal_byteslit(b"sol");
        reveal_strlit("sol");
        reveal_byteslit(b"solution_shape");
        reveal_strlit("solution_shape");
        reveal_byteslit(b"solution_values");
        reveal_strlit("solution_values");
        reveal(ascii);
        assert(sol@ == ascii("sol"@));
        assert(shape@ == ascii("solution_shape"@));
        assert(values@ == ascii("solution_values"@));
    }
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(models.skip(0) == models);
    }
    while i < rows.len()
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, rows@),
            models == root_terms(arena.nodes@, rows@),
            i <= rows.len(),
            sols_check(models, 1, arity as nat) == sols_check(
                models.skip(i as int),
                i as nat + 1,
                arity as nat,
            ),
            sol@ == ascii("sol"@),
            shape@ == ascii("solution_shape"@),
            values@ == ascii("solution_values"@),
        decreases rows.len() - i,
    {
        proof {
            reveal_with_fuel(sols_check, 1);
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
        }
        if !is_comp(arena, rows[i], sol, 1) {
            return Some(sol_error(i + 1, shape));
        }
        let args = crate::k2_engine::args_roots(arena, rows[i]);
        let vs = match crate::k2_walk::list_items_exec(arena, args[0]) {
            None => return Some(sol_error(i + 1, values)),
            Some(vs) => vs,
        };
        if vs.len() != arity {
            return Some(arity_error(i + 1, arity, vs.len()));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(sols_check, 1);
    }
    None
}

pub fn result_check_exec(arena: &ETermArena, root: usize, arity: usize) -> (out: Option<EOut>)
    requires
        root_ok(arena, root),
    ensures
        option_out_view(out) == checked_out(result_check(arena@[root as int], arity as nat)),
{
    hide(sols_check);
    let solutions: &[u8] = b"solutions";
    let shape: &[u8] = b"result_shape";
    let mode: &[u8] = b"result_mode_mismatch";
    let list: &[u8] = b"solutions_list";
    proof {
        reveal_byteslit(b"solutions");
        reveal_strlit("solutions");
        reveal_byteslit(b"result_shape");
        reveal_strlit("result_shape");
        reveal_byteslit(b"result_mode_mismatch");
        reveal_strlit("result_mode_mismatch");
        reveal_byteslit(b"solutions_list");
        reveal_strlit("solutions_list");
        reveal(ascii);
        assert(solutions@ == ascii("solutions"@));
        assert(shape@ == ascii("result_shape"@));
        assert(mode@ == ascii("result_mode_mismatch"@));
        assert(list@ == ascii("solutions_list"@));
    }
    let yesno = yesno_exec(arena, root);
    let sols = is_comp(arena, root, solutions, 1);
    if !(yesno || sols) {
        return Some(file_atom_error(shape, false));
    }
    if arity == 0 {
        return if yesno {
            None
        } else {
            Some(file_atom_error(mode, false))
        };
    }
    if !(sols || limit_shape(arena, root)) {
        return Some(file_atom_error(mode, false));
    }
    if sols {
        let args = crate::k2_engine::args_roots(arena, root);
        let rows = match crate::k2_walk::list_items_exec(arena, args[0]) {
            None => return Some(file_atom_error(list, false)),
            Some(rows) => rows,
        };
        sols_check_exec(arena, &rows, arity)
    } else {
        None
    }
}

pub struct EAnswers {
    pub result: usize,
    pub file: Ghost<AnswersFile>,
}

pub open spec fn answers_ok(nodes: Seq<ENode>, a: &EAnswers) -> bool {
    a.result < nodes.len() && nodes[a.result as int].term@ == a.file@.result
}

pub open spec fn answers_view(out: Result<EAnswers, EOut>) -> Result<AnswersFile, Out> {
    match out {
        Ok(a) => Ok(a.file@),
        Err(o) => Err(o@),
    }
}

pub fn answers_custody_exec(
    arena: &mut ETermArena,
    source: &ESrc,
    qsha: &[u8],
    qid: &Vec<u8>,
    arity: usize,
    Ghost(q): Ghost<QueryFile>,
) -> (out: Result<EAnswers, EOut>)
    requires
        arena_ok(old(arena)),
        qid@ == q.qid,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(a) ==> answers_ok(final(arena).nodes@, &a),
        answers_view(out) == answers_custody(source@, qsha@, q, arity as nat),
{
    hide(answers_custody);
    hide(result_check);
    let bytes = match source {
        ESrc::Missing => {
            proof {
                reveal(answers_custody);
            }
            return Err(crate::k2_reject::unreadable_out());
        },
        ESrc::Bad(off) => {
            proof {
                reveal(answers_custody);
            }
            return Err(crate::k2_reject::utf8_out(*off));
        },
        ESrc::Bytes(bytes) => bytes,
    };
    let parsed = match crate::v1_impl::v1_parse(bytes.as_slice(), arena) {
        None => {
            let why: &[u8] = b"noncanonical";
            proof {
                reveal(answers_custody);
                reveal_byteslit(b"noncanonical");
                reveal_strlit("noncanonical");
                reveal(ascii);
                assert(why@ == ascii("noncanonical"@));
            }
            return Err(file_atom_error(why, false));
        },
        Some(parsed) => parsed,
    };
    proof {
        reveal(answers_custody);
    }
    if !matches!(parsed.class, EV1Class::Answers) {
        if matches!(parsed.class, EV1Class::Traces) {
            let why: &[u8] = b"record_shape";
            proof {
                reveal_byteslit(b"record_shape");
                reveal_strlit("record_shape");
                reveal(ascii);
                assert(why@ == ascii("record_shape"@));
            }
            return Err(file_atom_error(why, false));
        }
        return Err(
            count_error(crate::k2_answers::parsed_term_count(arena, bytes.as_slice(), &parsed)),
        );
    }
    let ghost a = choose|a: AnswersFile| parsed@ == V1File::Answers(a);
    proof {
        assert(parsed@ is Answers);
        parsed_query_roots_elim(arena.nodes@, &parsed);
    }
    if !crate::k2_engine::vec_equal(&parsed.qid, qid) {
        let why: &[u8] = b"qid_mismatch";
        proof {
            reveal_byteslit(b"qid_mismatch");
            reveal_strlit("qid_mismatch");
            reveal(ascii);
            assert(why@ == ascii("qid_mismatch"@));
        }
        return Err(file_atom_error(why, false));
    }
    if !bytes_equal(&parsed.qsha, qsha) {
        let why: &[u8] = b"query_sha256_mismatch";
        proof {
            reveal_byteslit(b"query_sha256_mismatch");
            reveal_strlit("query_sha256_mismatch");
            reveal(ascii);
            assert(why@ == ascii("query_sha256_mismatch"@));
        }
        return Err(file_atom_error(why, false));
    }
    if let Some(out) = result_check_exec(arena, parsed.result_root, arity) {
        return Err(out);
    }
    Ok(EAnswers { result: parsed.result_root, file: Ghost(a) })
}

pub struct ETrace {
    pub result: usize,
    pub file: Ghost<TracesFile>,
}

pub open spec fn trace_ok(nodes: Seq<ENode>, t: &ETrace) -> bool {
    t.result < nodes.len() && nodes[t.result as int].term@ == t.file@.result
}

pub open spec fn trace_view(out: Result<ETrace, EOut>) -> Result<TracesFile, Out> {
    match out {
        Ok(t) => Ok(t.file@),
        Err(o) => Err(o@),
    }
}

pub fn trace_custody_exec(
    arena: &mut ETermArena,
    source: &ESrc,
    qsha: &[u8],
    asha: &[u8],
    qid: &Vec<u8>,
    Ghost(q): Ghost<QueryFile>,
) -> (out: Result<ETrace, EOut>)
    requires
        arena_ok(old(arena)),
        qid@ == q.qid,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(t) ==> trace_ok(final(arena).nodes@, &t),
        trace_view(out) == trace_custody(source@, q, qsha@, asha@),
{
    hide(trace_custody);
    let bytes = match source {
        ESrc::Missing => {
            proof {
                reveal(trace_custody);
            }
            return Err(crate::k2_reject::unreadable_out());
        },
        ESrc::Bad(off) => {
            proof {
                reveal(trace_custody);
            }
            return Err(crate::k2_reject::utf8_out(*off));
        },
        ESrc::Bytes(bytes) => bytes,
    };
    let parsed = match crate::v1_impl::v1_parse(bytes.as_slice(), arena) {
        None => {
            let why: &[u8] = b"noncanonical";
            proof {
                reveal(trace_custody);
                reveal_byteslit(b"noncanonical");
                reveal_strlit("noncanonical");
                reveal(ascii);
                assert(why@ == ascii("noncanonical"@));
            }
            return Err(file_atom_error(why, true));
        },
        Some(parsed) => parsed,
    };
    proof {
        reveal(trace_custody);
    }
    if !matches!(parsed.class, EV1Class::Traces) {
        let why: &[u8] = b"record_shape";
        proof {
            reveal_byteslit(b"record_shape");
            reveal_strlit("record_shape");
            reveal(ascii);
            assert(why@ == ascii("record_shape"@));
        }
        return Err(file_atom_error(why, true));
    }
    let ghost t = choose|t: TracesFile| parsed@ == V1File::Traces(t);
    proof {
        assert(parsed@ is Traces);
        parsed_query_roots_elim(arena.nodes@, &parsed);
    }
    if !crate::k2_engine::vec_equal(&parsed.qid, qid) {
        let why: &[u8] = b"qid_mismatch";
        proof {
            reveal_byteslit(b"qid_mismatch");
            reveal_strlit("qid_mismatch");
            reveal(ascii);
            assert(why@ == ascii("qid_mismatch"@));
        }
        return Err(file_atom_error(why, true));
    }
    if !bytes_equal(&parsed.qsha, qsha) {
        let why: &[u8] = b"query_sha256_mismatch";
        proof {
            reveal_byteslit(b"query_sha256_mismatch");
            reveal_strlit("query_sha256_mismatch");
            reveal(ascii);
            assert(why@ == ascii("query_sha256_mismatch"@));
        }
        return Err(file_atom_error(why, true));
    }
    if !bytes_equal(&parsed.asha, asha) {
        let why: &[u8] = b"answers_sha256_mismatch";
        proof {
            reveal_byteslit(b"answers_sha256_mismatch");
            reveal_strlit("answers_sha256_mismatch");
            reveal(ascii);
            assert(why@ == ascii("answers_sha256_mismatch"@));
        }
        return Err(file_atom_error(why, true));
    }
    Ok(ETrace { result: parsed.result_root, file: Ghost(t) })
}

pub struct EFront {
    pub query: EQuery,
    pub answer: EAnswers,
    pub loaded: ELoaded,
}

pub open spec fn front_ok(nodes: Seq<ENode>, f: &EFront) -> bool {
    query_ok(nodes, &f.query) && answers_ok(nodes, &f.answer) && loaded_ok(nodes, &f.loaded)
}

pub open spec fn front_model(nodes: Seq<ENode>, f: &EFront) -> Front {
    Front {
        q: f.query.file@,
        arows: root_terms(nodes, f.query.rows@),
        a: f.answer.file@,
        docs: f.loaded.docs@,
    }
}

pub open spec fn front_view(nodes: Seq<ENode>, out: Result<EFront, EOut>) -> Result<Front, Out> {
    match out {
        Ok(f) => Ok(front_model(nodes, &f)),
        Err(o) => Err(o@),
    }
}

pub proof fn front_prefix(before: Seq<ENode>, after: Seq<ENode>, f: &EFront)
    requires
        before.is_prefix_of(after),
        front_ok(before, f),
    ensures
        front_ok(after, f),
        front_model(before, f) == front_model(after, f),
{
    query_prefix(before, after, &f.query);
    crate::k2_load::loaded_prefix(before, after, &f.loaded);
    assert(after[f.answer.result as int] == before[f.answer.result as int]);
}

pub fn front_exec(
    arena: &mut ETermArena,
    mpath: &[u8],
    manifest: &ESrc,
    pls: &Vec<ESrc>,
    pys: &Vec<ESrc>,
    query: &ESrc,
    qsha: &[u8],
    answers: &ESrc,
) -> (out: Result<EFront, EOut>)
    requires
        arena_ok(old(arena)),
        cells_ok(manifest@, srcs(pls@), srcs(pys@)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(f) ==> front_ok(final(arena).nodes@, &f),
        front_view(final(arena).nodes@, out) == front(
            mpath@,
            manifest@,
            srcs(pls@),
            srcs(pys@),
            query@,
            qsha@,
            answers@,
        ),
{
    hide(ckc_spec::answers::custody);
    hide(answers_custody);
    hide(ckc_spec::answers::composition);
    let rows = match crate::k2_load::manifest_stage_exec(mpath, manifest, pls, pys) {
        Err(o) => return Err(o),
        Ok(rows) => rows,
    };
    let query = match crate::k2_answers::custody_exec(arena, query) {
        Err(o) => return Err(o),
        Ok(q) => q,
    };
    let ghost before_answers = arena.nodes@;
    let answer = match answers_custody_exec(
        arena,
        answers,
        qsha,
        &query.qid,
        query.rows.len(),
        Ghost(query.file@),
    ) {
        Err(o) => return Err(o),
        Ok(a) => a,
    };
    proof {
        query_prefix(before_answers, arena.nodes@, &query);
    }
    let ghost before_load = arena.nodes@;
    let loaded = match crate::k2_load::composition_exec(arena, &rows, pls) {
        Err(o) => return Err(o),
        Ok(loaded) => loaded,
    };
    proof {
        query_prefix(before_load, arena.nodes@, &query);
        assert(arena.nodes@[answer.result as int] == before_load[answer.result as int]);
    }
    Ok(EFront { query, answer, loaded })
}

} // verus!
