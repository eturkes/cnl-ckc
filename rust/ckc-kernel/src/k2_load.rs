use crate::k2_engine::EClause;
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{
    db_models_prefix, db_valid, db_view, root_terms, roots_models_prefix, roots_valid,
};
use crate::k2_manifest::v1_manifest_impl;
use crate::k2_output::{atom_root, manifest_unreadable_out};
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::v1_term_impl::EParsedV1;
use ckc_spec::replay::*;
use ckc_spec::v1text::DocFile;
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn row_paths(rows: Seq<ERow>, payload: bool) -> Seq<Seq<u8>> {
    rows.map_values(
        |r: ERow|
            if payload {
                r.payload@
            } else {
                r.pl@
            },
    )
}

pub open spec fn missing_view(rows: Seq<ERow>, payload: bool, index: Option<usize>) -> Option<
    Seq<u8>,
> {
    match index {
        Some(i) => Some(row_paths(rows, payload)[i as int]),
        None => None,
    }
}

fn missing_index(rows: &Vec<ERow>, cells: &Vec<ESrc>, payload: bool) -> (out: Option<usize>)
    requires
        rows@.len() == cells@.len(),
    ensures
        out matches Some(i) ==> i < rows@.len(),
        missing_view(rows@, payload, out) == first_missing(
            row_paths(rows@, payload),
            srcs(cells@),
            0,
        ),
{
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows@.len(),
            rows@.len() == cells@.len(),
            first_missing(row_paths(rows@, payload), srcs(cells@), 0) == first_missing(
                row_paths(rows@, payload),
                srcs(cells@),
                i as nat,
            ),
        decreases rows.len() - i,
    {
        if let ESrc::Missing = &cells[i] {
            return Some(i);
        }
        proof {
            reveal_with_fuel(first_missing, 1);
        }
        i += 1;
    }
    None
}

pub fn manifest_stage_exec(mpath: &[u8], m: &ESrc, pls: &Vec<ESrc>, pys: &Vec<ESrc>) -> (out:
    Result<Vec<ERow>, EOut>)
    requires
        cells_ok(m@, srcs(pls@), srcs(pys@)),
    ensures
        rows_view(out) == manifest_stage(mpath@, m@, srcs(pls@), srcs(pys@)),
        out matches Ok(rows) ==> rows.len() == pls.len() && rows.len() == pys.len(),
{
    let rows = match v1_manifest_impl(mpath, m) {
        Err(out) => return Err(out),
        Ok(rows) => rows,
    };
    proof {
        let models = rows@.map_values(|r: ERow| r@);
        assert(parse_manifest(src_bytes(m@)) == MOut::Rows(models));
        assert_seqs_equal!(row_paths(rows@, false) == models.map_values(|r: MRow| r.pl));
        assert_seqs_equal!(row_paths(rows@, true) == models.map_values(|r: MRow| r.payload));
    }
    if let Some(i) = missing_index(&rows, pls, false) {
        return Err(manifest_unreadable_out(mpath, &rows[i].pl));
    }
    if let Some(i) = missing_index(&rows, pys, true) {
        return Err(manifest_unreadable_out(mpath, &rows[i].payload));
    }
    Ok(rows)
}

pub open spec fn path_views(paths: Seq<Vec<u8>>) -> Seq<Seq<u8>> {
    paths.map_values(|p: Vec<u8>| p@)
}

fn path_contains(paths: &Vec<Vec<u8>>, path: &Vec<u8>) -> (out: bool)
    ensures
        out == path_views(paths@).contains(path@),
{
    let mut i = 0usize;
    while i < paths.len()
        invariant
            i <= paths.len(),
            forall|j: int| 0 <= j < i ==> path_views(paths@)[j] != path@,
        decreases paths.len() - i,
    {
        if crate::k2_engine::vec_equal(&paths[i], path) {
            proof {
                assert(path_views(paths@)[i as int] == path@);
            }
            return true;
        }
        i += 1;
    }
    false
}

fn unique_paths_exec(rows: &Vec<ERow>) -> (out: Vec<Vec<u8>>)
    ensures
        path_views(out@) == unique_paths(rows@.map_values(|r: ERow| r@)),
        out@.len() <= rows@.len(),
        forall|j: int|
            0 <= j < out@.len() ==> row_paths(rows@, false).contains(
                #[trigger] path_views(out@)[j],
            ),
{
    let ghost models = rows@.map_values(|r: ERow| r@);
    let mut out = Vec::new();
    proof {
        assert_seqs_equal!(path_views(out@) == Seq::empty());
    }
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows.len(),
            out@.len() <= i,
            models == rows@.map_values(|r: ERow| r@),
            unique_paths(models) == unique_from(models, i as nat, path_views(out@)),
            forall|j: int|
                0 <= j < out@.len() ==> row_paths(rows@, false).contains(
                    #[trigger] path_views(out@)[j],
                ),
        decreases rows.len() - i,
    {
        let ghost before = out@;
        let duplicate = path_contains(&out, &rows[i].pl);
        if !duplicate {
            out.push(rows[i].pl.clone());
            proof {
                assert_seqs_equal!(path_views(out@) == path_views(before).push(rows@[i as int].pl@));
                assert(row_paths(rows@, false)[i as int] == rows@[i as int].pl@);
                assert forall|j: int| 0 <= j < out@.len() implies row_paths(rows@, false).contains(
                    #[trigger] path_views(out@)[j],
                ) by {
                    if j < before.len() {
                        assert(path_views(out@)[j] == path_views(before)[j]);
                    }
                }
            }
        }
        proof {
            reveal_with_fuel(unique_from, 1);
        }
        i += 1;
    }
    out
}

fn row_index_exec(rows: &Vec<ERow>, path: &Vec<u8>) -> (out: usize)
    ensures
        out <= rows.len(),
        out as nat == row_index(rows@.map_values(|r: ERow| r@), path@, 0),
        row_paths(rows@, false).contains(path@) ==> out < rows.len(),
        out < rows.len() ==> rows@[out as int].pl@ == path@,
{
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows.len(),
            row_index(rows@.map_values(|r: ERow| r@), path@, 0) == row_index(
                rows@.map_values(|r: ERow| r@),
                path@,
                i as nat,
            ),
            forall|j: int| 0 <= j < i ==> row_paths(rows@, false)[j] != path@,
        decreases rows.len() - i,
    {
        if crate::k2_engine::vec_equal(&rows[i].pl, path) {
            return i;
        }
        proof {
            reveal_with_fuel(row_index, 1);
        }
        i += 1;
    }
    i
}

pub struct ELoaded {
    pub db: Vec<EClause>,
    pub docids: Vec<usize>,
    pub docs: Ghost<Seq<DocFile>>,
}

pub open spec fn loaded_ok(nodes: Seq<ENode>, loaded: &ELoaded) -> bool {
    &&& db_valid(nodes, loaded.db@)
    &&& db_view(nodes, loaded.db@) == db_of(loaded.docs@)
    &&& roots_valid(nodes, loaded.docids@)
    &&& loaded.docids@.len() == loaded.docs@.len()
    &&& root_terms(nodes, loaded.docids@) == loaded.docs@.map_values(
        |d: DocFile| ckc_spec::term::Term::Atom(d.docid),
    )
}

pub open spec fn loaded_view(out: Result<ELoaded, EOut>) -> Result<Seq<DocFile>, Out> {
    match out {
        Ok(loaded) => Ok(loaded.docs@),
        Err(out) => Err(out@),
    }
}

pub proof fn loaded_prefix(before: Seq<ENode>, after: Seq<ENode>, loaded: &ELoaded)
    requires
        before.is_prefix_of(after),
        loaded_ok(before, loaded),
    ensures
        loaded_ok(after, loaded),
{
    db_models_prefix(before, after, loaded.db@);
    roots_models_prefix(before, after, loaded.docids@);
}

fn empty_loaded(arena: &ETermArena) -> (out: ELoaded)
    ensures
        loaded_ok(arena.nodes@, &out),
        out.docs@ == Seq::<DocFile>::empty(),
{
    let out = ELoaded { db: Vec::new(), docids: Vec::new(), docs: Ghost(Seq::empty()) };
    proof {
        assert_seqs_equal!(db_view(arena.nodes@, out.db@) == Seq::empty());
        assert_seqs_equal!(out.docs@.map_values(|d: DocFile| doc_db(d)) == Seq::empty());
        assert_seqs_equal!(root_terms(arena.nodes@, out.docids@)
            == out.docs@.map_values(|d: DocFile| ckc_spec::term::Term::Atom(d.docid)));
    }
    out
}

proof fn db_of_push(docs: Seq<DocFile>, doc: DocFile)
    ensures
        db_of(docs.push(doc)) == db_of(docs) + doc_db(doc),
{
    let parts = docs.map_values(|d: DocFile| doc_db(d));
    assert_seqs_equal!(docs.push(doc).map_values(|d: DocFile| doc_db(d)) == parts.push(doc_db(doc)));
    parts.lemma_flatten_push(doc_db(doc));
}

proof fn docs_of_push(members: Seq<ckc_spec::v1text::V1File>, member: ckc_spec::v1text::V1File)
    ensures
        docs_of(members.push(member)) == docs_of(members) + match member {
            ckc_spec::v1text::V1File::Doc(d) => seq![d],
            _ => Seq::empty(),
        },
    decreases members.len(),
{
    if members.len() > 0 {
        docs_of_push(members.drop_first(), member);
        assert_seqs_equal!(members.push(member).drop_first() == members.drop_first().push(member));
        reveal_with_fuel(docs_of, 2);
    } else {
        reveal_with_fuel(docs_of, 2);
    }
}

fn append_document(
    arena: &mut ETermArena,
    loaded: &mut ELoaded,
    parsed: &EParsedV1,
    Ghost(doc): Ghost<DocFile>,
)
    requires
        arena_ok(old(arena)),
        loaded_ok(old(arena).nodes@, old(loaded)),
        crate::v1_term_impl::parsed_doc_roots_ok(old(arena).nodes@, parsed),
        crate::v1_term_impl::parsed_metadata_ok(parsed),
        parsed@ == ckc_spec::v1text::V1File::Doc(doc),
        ckc_spec::v1text::wf_doc(doc),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        loaded_ok(final(arena).nodes@, final(loaded)),
        final(loaded).docs@ == old(loaded).docs@.push(doc),
{
    let ghost before = arena.nodes@;
    let ghost docs = loaded.docs@;
    let db = crate::k2_bridge::document_db(arena, parsed, Ghost(doc));
    let ghost after_db = arena.nodes@;
    let docid = atom_root(arena, &parsed.docid);
    proof {
        loaded_prefix(before, arena.nodes@, loaded);
        db_models_prefix(after_db, arena.nodes@, db@);
        db_of_push(docs, doc);
    }
    crate::k2_bridge::append_db(arena, &mut loaded.db, db);
    let ghost prior_ids = loaded.docids@;
    proof {
        assert(prior_ids.len() == docs.len());
        assert(root_terms(arena.nodes@, prior_ids) == docs.map_values(
            |d: DocFile| ckc_spec::term::Term::Atom(d.docid),
        ));
        assert(arena.nodes@[docid as int].term@ == ckc_spec::term::Term::Atom(doc.docid));
    }
    loaded.docids.push(docid);
    loaded.docs = Ghost(docs.push(doc));
    proof {
        assert forall|i: int| 0 <= i < loaded.docids@.len() implies loaded.docids@[i]
            < arena.nodes@.len() by {
            if i < prior_ids.len() {
                assert(loaded.docids@[i] == prior_ids[i]);
            }
        }
        assert_seqs_equal!(root_terms(arena.nodes@, loaded.docids@)
            == loaded.docs@.map_values(|d: DocFile| ckc_spec::term::Term::Atom(d.docid)), i => {
            if i < prior_ids.len() {
                assert(loaded.docids@[i] == prior_ids[i]);
                assert(root_terms(arena.nodes@, prior_ids)[i] == ckc_spec::term::Term::Atom(docs[i].docid));
                assert(loaded.docs@[i] == docs[i]);
            } else {
                assert(i == prior_ids.len());
                assert(loaded.docids@[i] == docid);
                assert(loaded.docs@[i] == doc);
            }
        });
    }
}

pub proof fn prefix_chain(before: Seq<ENode>, middle: Seq<ENode>, after: Seq<ENode>)
    requires
        before.is_prefix_of(middle),
        middle.is_prefix_of(after),
    ensures
        before.is_prefix_of(after),
{
    assert_seqs_equal!(before == after.take(before.len() as int), i => {
        assert(before[i] == middle[i]);
        assert(middle[i] == after[i]);
    });
}

fn load_inner(input_arena: ETermArena, rows: &Vec<ERow>, pls: &Vec<ESrc>) -> (out: (
    Result<ELoaded, EOut>,
    ETermArena,
))
    requires
        arena_ok(&input_arena),
        rows.len() == pls.len(),
    ensures
        arena_ok(&out.1),
        input_arena.nodes@.is_prefix_of(out.1.nodes@),
        out.0 matches Ok(loaded) ==> loaded_ok(out.1.nodes@, &loaded),
        loaded_view(out.0) == load_stage(rows@.map_values(|r: ERow| r@), srcs(pls@)),
{
    hide(loaded_ok);
    hide(db_of);
    hide(doc_db);
    hide(docs_of);
    hide(crate::v1_term_impl::parsed_metadata_ok);
    let name: &[u8] = b"noncanonical";
    proof {
        reveal_byteslit(b"noncanonical");
        reveal_strlit("noncanonical");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("noncanonical"@));
    }
    let ghost origin = input_arena.nodes@;
    let mut arena = input_arena;
    let ghost models = rows@.map_values(|r: ERow| r@);
    let paths = unique_paths_exec(rows);
    let ghost ms = members(models, srcs(pls@));
    let mut loaded = empty_loaded(&arena);
    let mut i = 0usize;
    proof {
        assert(ms.len() == paths@.len());
        assert_seqs_equal!(ms.take(0) == Seq::empty());
        reveal_with_fuel(docs_of, 1);
    }
    while i < paths.len()
        invariant
            arena_ok(&arena),
            origin == input_arena.nodes@,
            origin.is_prefix_of(arena.nodes@),
            rows.len() == pls.len(),
            i <= paths.len(),
            models == rows@.map_values(|r: ERow| r@),
            path_views(paths@) == unique_paths(models),
            ms == members(models, srcs(pls@)),
            ms.len() == paths@.len(),
            forall|j: int|
                0 <= j < paths@.len() ==> row_paths(rows@, false).contains(
                    #[trigger] path_views(paths@)[j],
                ),
            loaded_ok(arena.nodes@, &loaded),
            loaded.docs@ == docs_of(ms.take(i as int)),
            first_bad_member(models, srcs(pls@), path_views(paths@), 0) == first_bad_member(
                models,
                srcs(pls@),
                path_views(paths@),
                i as nat,
            ),
            name@ == ckc_spec::v1text::ascii("noncanonical"@),
        decreases paths.len() - i,
    {
        let index = row_index_exec(rows, &paths[i]);
        let ghost source = src_of(models, srcs(pls@), paths@[i as int]@);
        let ghost before_parse = arena.nodes@;
        proof {
            assert(source == pls@[index as int]@);
            assert(ms[i as int] == the_v1(src_bytes(source)));
        }
        let parsed = match &pls[index] {
            ESrc::Bytes(bytes) => crate::v1_impl::v1_parse(bytes, &mut arena),
            _ => None,
        };
        proof {
            loaded_prefix(before_parse, arena.nodes@, &loaded);
            prefix_chain(origin, before_parse, arena.nodes@);
        }
        let parsed = match parsed {
            None => {
                proof {
                    assert(!(source is Bytes) || !ckc_spec::v1text::accepts(src_bytes(source)));
                    reveal_with_fuel(first_bad_member, 1);
                }
                let error = crate::k2_output::named_atom_error(name, &paths[i], false);
                return (Err(error), arena);
            },
            Some(parsed) => parsed,
        };
        proof {
            assert(source is Bytes);
            assert(ckc_spec::v1text::accepts(src_bytes(source)));
            assert(parsed@ == ms[i as int]);
            assert(ckc_spec::v1text::wf_v1(parsed@));
            docs_of_push(ms.take(i as int), parsed@);
            assert_seqs_equal!(ms.take(i as int + 1) == ms.take(i as int).push(parsed@));
            reveal(crate::v1_term_impl::parsed_metadata_ok);
        }
        let ghost before_doc = arena.nodes@;
        match parsed.class {
            crate::v1_term_impl::EV1Class::Doc => {
                let ghost doc = choose|d: DocFile| parsed@ == ckc_spec::v1text::V1File::Doc(d);
                proof {
                    assert(parsed@ is Doc);
                }
                append_document(&mut arena, &mut loaded, &parsed, Ghost(doc));
                proof {
                    prefix_chain(origin, before_doc, arena.nodes@);
                    assert_seqs_equal!(loaded.docs@ == docs_of(ms.take(i as int)) + seq![doc]);
                    assert(loaded.docs@ == docs_of(ms.take(i as int + 1)));
                }
            },
            _ => {
                proof {
                    assert(!(parsed@ is Doc));
                    assert_seqs_equal!(loaded.docs@ == loaded.docs@ + Seq::empty());
                    assert(loaded.docs@ == docs_of(ms.take(i as int + 1)));
                }
            },
        }
        proof {
            reveal_with_fuel(first_bad_member, 1);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(ms.take(i as int) == ms);
        reveal_with_fuel(first_bad_member, 1);
    }
    (Ok(loaded), arena)
}

pub fn load_stage_exec(arena: &mut ETermArena, rows: &Vec<ERow>, pls: &Vec<ESrc>) -> (out: Result<
    ELoaded,
    EOut,
>)
    requires
        arena_ok(old(arena)),
        rows.len() == pls.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(loaded) ==> loaded_ok(final(arena).nodes@, &loaded),
        loaded_view(out) == load_stage(rows@.map_values(|r: ERow| r@), srcs(pls@)),
{
    let mut owned = crate::k2_reject::empty_arena();
    std::mem::swap(arena, &mut owned);
    let (out, mut owned) = load_inner(owned, rows, pls);
    std::mem::swap(arena, &mut owned);
    out
}

pub fn assertions_exec(arena: &ETermArena, rows: &Vec<ERow>, loaded: &ELoaded) -> (out: Option<
    EOut,
>)
    requires
        arena_ok(arena),
        loaded_ok(arena.nodes@, loaded),
    ensures
        crate::k2_output::option_out_view(out) == assertions(
            rows@.map_values(|r: ERow| r@),
            loaded.docs@,
        ),
{
    let name: &[u8] = b"document_records";
    proof {
        reveal_byteslit(b"document_records");
        reveal_strlit("document_records");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("document_records"@));
    }
    if rows.len() != loaded.docids.len() {
        return Some(crate::k2_output::counts_error(name, rows.len(), loaded.docids.len()));
    }
    proof {
        assert_seqs_equal!(crate::k2_sort::root_terms(arena, loaded.docids@)
            == loaded.docs@.map_values(|d: DocFile| ckc_spec::term::Term::Atom(d.docid)));
        assert forall|i: int| 0 <= i < loaded.docids@.len() implies ckc_spec::term::ground(
            arena@[loaded.docids@[i] as int],
        ) by {
            assert(root_terms(arena.nodes@, loaded.docids@)[i] == ckc_spec::term::Term::Atom(
                loaded.docs@[i].docid,
            ));
        }
    }
    let distinct = crate::k2_sort::sort_unique(arena, &loaded.docids);
    if rows.len() != distinct.len() {
        Some(crate::k2_output::counts_error(name, rows.len(), distinct.len()))
    } else {
        None
    }
}

pub fn composition_exec(arena: &mut ETermArena, rows: &Vec<ERow>, pls: &Vec<ESrc>) -> (out: Result<
    ELoaded,
    EOut,
>)
    requires
        arena_ok(old(arena)),
        rows.len() == pls.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(loaded) ==> loaded_ok(final(arena).nodes@, &loaded),
        loaded_view(out) == ckc_spec::answers::composition(
            rows@.map_values(|r: ERow| r@),
            srcs(pls@),
        ),
{
    if rows.len() == 0 {
        return Ok(empty_loaded(arena));
    }
    let loaded = match load_stage_exec(arena, rows, pls) {
        Err(out) => return Err(out),
        Ok(loaded) => loaded,
    };
    if let Some(out) = assertions_exec(arena, rows, &loaded) {
        return Err(out);
    }
    Ok(loaded)
}

} // verus!
