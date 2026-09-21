use crate::k2_engine::{EBodyItem, EClause};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{
    body_item_valid, body_item_view, body_items_view, clause_valid, clause_view, db_valid, db_view,
    root_terms, roots_valid,
};
use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
use ckc_spec::v1text::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub fn term_bytes_exec(arena: &ETermArena, root: usize) -> (out: Vec<u8>)
    requires
        root_ok(arena, root),
    ensures
        out@ == term_bytes(arena@[root as int]),
{
    let mut out = crate::k2_term::term_line(arena, root);
    proof {
        reveal_strlit(".\n");
        reveal(ascii);
        assert(out@ == term_bytes(arena@[root as int]) + seq![0x2eu8, 0x0au8]);
    }
    out.pop();
    out.pop();
    proof {
        assert_seqs_equal!(out@ == term_bytes(arena@[root as int]));
    }
    out
}

fn literals_bytes(arena: &ETermArena, roots: &Vec<usize>) -> (out: Vec<u8>)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, roots@),
    ensures
        out@ == lit_list_bytes(root_terms(arena.nodes@, roots@)),
{
    let ghost models = root_terms(arena.nodes@, roots@);
    let sep: &[u8] = b", ";
    proof {
        reveal_byteslit(b", ");
        reveal_strlit(", ");
        reveal(ascii);
        assert(sep@ == ascii(", "@));
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(models.skip(0) == models);
        assert_seqs_equal!(out@ + lit_list_bytes(models) == lit_list_bytes(models));
    }
    while i < roots.len()
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, roots@),
            models == root_terms(arena.nodes@, roots@),
            i <= roots.len(),
            sep@ == ascii(", "@),
            out@ + lit_list_bytes(models.skip(i as int)) == lit_list_bytes(models),
        decreases roots.len() - i,
    {
        let mut next = term_bytes_exec(arena, roots[i]);
        out.append(&mut next);
        if i + 1 < roots.len() {
            let mut glue = slice_to_vec(sep);
            out.append(&mut glue);
        }
        proof {
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
            reveal_with_fuel(lit_list_bytes, 1);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(lit_list_bytes, 1);
    }
    out
}

fn item_bytes(arena: &ETermArena, item: &EBodyItem) -> (out: Vec<u8>)
    requires
        arena_ok(arena),
        body_item_valid(arena.nodes@, item),
    ensures
        out@ == body_item_bytes(body_item_view(arena.nodes@, item)),
{
    match item {
        EBodyItem::Pos { root } => term_bytes_exec(arena, *root),
        EBodyItem::Naf { roots } => {
            let one: &[u8] = b"\\+ ";
            let many: &[u8] = b"\\+ (";
            proof {
                reveal_byteslit(b"\\+ ");
                reveal_strlit("\\+ ");
                reveal_byteslit(b"\\+ (");
                reveal_strlit("\\+ (");
                reveal(ascii);
                assert(one@ == ascii("\\+ "@));
                assert(many@ == ascii("\\+ ("@));
            }
            if roots.len() == 1 {
                let mut out = slice_to_vec(one);
                let mut value = term_bytes_exec(arena, roots[0]);
                out.append(&mut value);
                out
            } else {
                let mut out = slice_to_vec(many);
                let mut values = literals_bytes(arena, roots);
                out.append(&mut values);
                out.push(0x29);
                out
            }
        },
    }
}

fn body_bytes_exec(arena: &ETermArena, clause: &EClause) -> (out: Vec<u8>)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, clause),
    ensures
        out@ == body_bytes(clause_view(arena.nodes@, clause).body),
{
    let ghost models = body_items_view(arena.nodes@, clause.body@);
    let sep: &[u8] = b", ";
    proof {
        reveal_byteslit(b", ");
        reveal_strlit(", ");
        reveal(ascii);
        assert(sep@ == ascii(", "@));
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(models.skip(0) == models);
        assert_seqs_equal!(out@ + body_bytes(models) == body_bytes(models));
    }
    while i < clause.body.len()
        invariant
            arena_ok(arena),
            clause_valid(arena.nodes@, clause),
            models == body_items_view(arena.nodes@, clause.body@),
            i <= clause.body.len(),
            sep@ == ascii(", "@),
            out@ + body_bytes(models.skip(i as int)) == body_bytes(models),
        decreases clause.body.len() - i,
    {
        let mut next = item_bytes(arena, &clause.body[i]);
        out.append(&mut next);
        if i + 1 < clause.body.len() {
            let mut glue = slice_to_vec(sep);
            out.append(&mut glue);
        }
        proof {
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
            reveal_with_fuel(body_bytes, 1);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(body_bytes, 1);
    }
    out
}

pub fn clause_bytes(arena: &ETermArena, clause: &EClause) -> (out: Vec<u8>)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, clause),
    ensures
        out@ == clause_line(clause_view(arena.nodes@, clause)),
{
    if clause.body.len() == 0 {
        return crate::k2_term::term_line(arena, clause.head);
    }
    let mut out = term_bytes_exec(arena, clause.head);
    let glue: &[u8] = b" :- ";
    let end: &[u8] = b".\n";
    proof {
        reveal_byteslit(b" :- ");
        reveal_strlit(" :- ");
        reveal_byteslit(b".\n");
        reveal_strlit(".\n");
        reveal(ascii);
        assert(glue@ == ascii(" :- "@));
        assert(end@ == ascii(".\n"@));
    }
    let mut glue = slice_to_vec(glue);
    out.append(&mut glue);
    let mut body = body_bytes_exec(arena, clause);
    out.append(&mut body);
    let mut end = slice_to_vec(end);
    out.append(&mut end);
    out
}

pub fn db_lines(arena: &ETermArena, db: &Vec<EClause>) -> (out: Vec<Vec<u8>>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
    ensures
        ckc_spec::trace::digests_view(out@) == db_view(arena.nodes@, db@).map_values(
            |c: DocClause| clause_line(c),
        ),
{
    let ghost models = db_view(arena.nodes@, db@);
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < db.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            models == db_view(arena.nodes@, db@),
            i <= db.len(),
            out.len() == i,
            ckc_spec::trace::digests_view(out@) == models.take(i as int).map_values(
                |c: DocClause| clause_line(c),
            ),
        decreases db.len() - i,
    {
        let next = clause_bytes(arena, &db[i]);
        let ghost before = out@;
        out.push(next);
        proof {
            assert_seqs_equal!(ckc_spec::trace::digests_view(out@) == ckc_spec::trace::digests_view(before).push(next@));
            assert_seqs_equal!(models.take(i as int + 1).map_values(|c: DocClause| clause_line(c))
                == models.take(i as int).map_values(|c: DocClause| clause_line(c)).push(clause_line(models[i as int])));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.take(i as int) == models);
    }
    out
}

pub fn traces_exec(
    arena: &mut ETermArena,
    qid: &[u8],
    qsha: &[u8],
    asha: &[u8],
    result: usize,
) -> (out: ckc_spec::replay::EOut)
    requires
        root_ok(old(arena), result),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == ckc_spec::replay::ok(
            print_traces(
                TracesFile {
                    qid: qid@,
                    qsha: qsha@,
                    asha: asha@,
                    result: old(arena)@[result as int],
                },
            ),
        ),
{
    let ghost before = arena.nodes@;
    let version: &[u8] = b"v1";
    let query_name: &[u8] = b"query_sha256";
    let answers_name: &[u8] = b"answers_sha256";
    let result_name: &[u8] = b"result";
    let record_name: &[u8] = b"$guideline_traces";
    let prefix: &[u8] = b"% ";
    let suffix: &[u8] =
        b" traced against the loaded composition by ace_to_pl trace mode; do not edit.\n";
    proof {
        reveal_byteslit(b"v1");
        reveal_strlit("v1");
        reveal_byteslit(b"query_sha256");
        reveal_strlit("query_sha256");
        reveal_byteslit(b"answers_sha256");
        reveal_strlit("answers_sha256");
        reveal_byteslit(b"result");
        reveal_strlit("result");
        reveal_byteslit(b"$guideline_traces");
        reveal_strlit("$guideline_traces");
        reveal_byteslit(b"% ");
        reveal_strlit("% ");
        reveal_byteslit(
            b" traced against the loaded composition by ace_to_pl trace mode; do not edit.\n",
        );
        reveal_strlit(
            " traced against the loaded composition by ace_to_pl trace mode; do not edit.\n",
        );
        reveal(ascii);
        assert(version@ == ascii("v1"@));
        assert(query_name@ == ascii("query_sha256"@));
        assert(answers_name@ == ascii("answers_sha256"@));
        assert(result_name@ == ascii("result"@));
        assert(record_name@ == ascii("$guideline_traces"@));
        assert(prefix@ == ascii("% "@));
        assert(suffix@ == ascii(
            " traced against the loaded composition by ace_to_pl trace mode; do not edit.\n"@,
        ));
    }
    let v1 = crate::k2_output::atom_root(arena, version);
    let id = crate::k2_output::atom_root(arena, qid);
    let query_digest = crate::k2_output::atom_root(arena, qsha);
    let answer_digest = crate::k2_output::atom_root(arena, asha);
    let query = crate::k2_output::comp1(arena, query_name, query_digest);
    let answer = crate::k2_output::comp1(arena, answers_name, answer_digest);
    let payload = crate::k2_output::comp1(arena, result_name, result);
    let mut fields = Vec::new();
    fields.push(v1);
    fields.push(id);
    fields.push(query);
    fields.push(answer);
    fields.push(payload);
    proof {
        crate::k2_term::arena_prefix_stable(before, arena);
        crate::k2_term::child_terms_match(
            arena.nodes@,
            fields@,
            seq![
                ckc_spec::term::Term::Atom(version@),
                ckc_spec::term::Term::Atom(qid@),
                ckc_spec::term::Term::Comp(query_name@, seq![ckc_spec::term::Term::Atom(qsha@)]),
                ckc_spec::term::Term::Comp(answers_name@, seq![ckc_spec::term::Term::Atom(asha@)]),
                ckc_spec::term::Term::Comp(result_name@, seq![before[result as int].term@]),
            ],
        );
    }
    let record = crate::k2_output::comp_root(arena, record_name, fields);
    let mut bytes = slice_to_vec(prefix);
    let mut id_bytes = slice_to_vec(qid);
    bytes.append(&mut id_bytes);
    let mut suffix_bytes = slice_to_vec(suffix);
    bytes.append(&mut suffix_bytes);
    let mut record_bytes = crate::k2_term::term_line(arena, record);
    bytes.append(&mut record_bytes);
    ckc_spec::replay::EOut { rc: 0, out: bytes, err: Vec::new() }
}

} // verus!
