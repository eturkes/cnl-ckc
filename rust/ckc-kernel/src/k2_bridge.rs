use crate::k2_engine::{EBodyItem, EClause};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{
    body_item_valid, body_item_view, body_items_valid, body_items_view, clause_valid, clause_view,
    db_valid, db_view, root_terms, roots_valid,
};
use crate::k2_output::{atom_root, comp1, comp3, int_root};
use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::root_ok;
use crate::v1_term_impl::{EBodyRoot, EDocClause, EParsedV1};
#[cfg(verus_keep_ghost)]
use crate::v1_term_impl::{
    body_root_elim, body_root_ok, body_roots_elim, doc_clause_roots_elim, doc_clause_roots_ok,
    doc_clauses_roots_elim, doc_clauses_roots_ok,
};
use ckc_spec::v1text::{BodyItem, DocClause};
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{wf_body_item, wf_clause};
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn fact_exec(arena: &ETermArena, head: usize) -> (out: EClause)
    requires
        root_ok(arena, head),
    ensures
        clause_valid(arena.nodes@, &out),
        clause_view(arena.nodes@, &out) == ckc_spec::engine::fact_clause(arena@[head as int]),
{
    let out = EClause { head, body: Vec::new() };
    proof {
        assert_seqs_equal!(body_items_view(arena.nodes@, out.body@) == Seq::empty());
    }
    out
}

pub fn append_db(arena: &ETermArena, target: &mut Vec<EClause>, mut rest: Vec<EClause>)
    requires
        db_valid(arena.nodes@, old(target)@),
        db_valid(arena.nodes@, rest@),
    ensures
        db_valid(arena.nodes@, final(target)@),
        db_view(arena.nodes@, final(target)@) == db_view(arena.nodes@, old(target)@) + db_view(
            arena.nodes@,
            rest@,
        ),
{
    let ghost left = target@;
    let ghost right = rest@;
    target.append(&mut rest);
    proof {
        assert forall|i: int| 0 <= i < target@.len() implies #[trigger] clause_valid(
            arena.nodes@,
            &target@[i],
        ) by {
            if i < left.len() {
                assert(target@[i] == left[i]);
            } else {
                assert(target@[i] == right[i - left.len()]);
            }
        }
        assert_seqs_equal!(db_view(arena.nodes@, target@)
            == db_view(arena.nodes@, left) + db_view(arena.nodes@, right), i => {
            if i < left.len() { assert(target@[i] == left[i]); }
            else { assert(target@[i] == right[i - left.len()]); }
        });
    }
}

fn body_from_parser(arena: &ETermArena, root: &EBodyRoot, Ghost(item): Ghost<BodyItem>) -> (out:
    EBodyItem)
    requires
        body_root_ok(arena.nodes@, root, item),
        wf_body_item(item),
    ensures
        body_item_valid(arena.nodes@, &out),
        body_item_view(arena.nodes@, &out) == item,
{
    proof {
        body_root_elim(arena.nodes@, root, item);
    }
    match root {
        EBodyRoot::Pos(index) => {
            proof {
                assert(item is Pos);
                assert(*index < arena.nodes@.len());
                reveal(body_item_valid);
                reveal(body_item_view);
            }
            EBodyItem::Pos { root: *index }
        },
        EBodyRoot::Naf(roots) => {
            proof {
                assert(item is Naf);
                let terms = match item {
                    BodyItem::Naf(terms) => terms,
                    _ => Seq::empty(),
                };
                reveal(wf_body_item);
                assert(roots@.len() > 0);
                assert_seqs_equal!(root_terms(arena.nodes@, roots@) == terms);
                assert forall|j: int| 0 <= j < roots@.len() implies roots@[j]
                    < arena.nodes@.len() by {
                    assert(arena.nodes@[roots@[j] as int].term@ == terms[j]);
                }
            }
            let out = EBodyItem::Naf { roots: roots.clone() };
            proof {
                assert(body_item_valid(arena.nodes@, &out));
            }
            out
        },
    }
}

pub fn clause_from_parser(arena: &ETermArena, clause: &EDocClause) -> (out: EClause)
    requires
        doc_clause_roots_ok(arena.nodes@, clause),
        wf_clause(clause@),
    ensures
        clause_valid(arena.nodes@, &out),
        clause_view(arena.nodes@, &out) == clause@,
{
    proof {
        doc_clause_roots_elim(arena.nodes@, clause);
        body_roots_elim(arena.nodes@, clause.body@, clause@.body);
    }
    let mut body = Vec::new();
    let mut i = 0usize;
    while i < clause.body.len()
        invariant
            i <= clause.body.len(),
            body@.len() == i,
            doc_clause_roots_ok(arena.nodes@, clause),
            clause.body@.len() == clause@.body.len(),
            forall|j: int|
                0 <= j < clause.body@.len() ==> #[trigger] body_root_ok(
                    arena.nodes@,
                    &clause.body@[j],
                    clause@.body[j],
                ),
            wf_clause(clause@),
            body_items_valid(arena.nodes@, body@),
            body_items_view(arena.nodes@, body@) == clause@.body.take(i as int),
        decreases clause.body.len() - i,
    {
        let item = body_from_parser(arena, &clause.body[i], Ghost(clause@.body[i as int]));
        let ghost before = body@;
        body.push(item);
        proof {
            assert forall|j: int| 0 <= j < body@.len() implies #[trigger] body_item_valid(
                arena.nodes@,
                &body@[j],
            ) by {
                if j < before.len() {
                    assert(body@[j] == before[j]);
                }
            }
            assert_seqs_equal!(body_items_view(arena.nodes@, body@)
                == clause@.body.take(i as int + 1), j => {
                if j < before.len() { assert(body@[j] == before[j]); }
            });
        }
        i += 1;
    }
    proof {
        doc_clause_roots_elim(arena.nodes@, clause);
        assert_seqs_equal!(body_items_view(arena.nodes@, body@) == clause@.body);
    }
    let out = EClause { head: clause.head_root, body };
    proof {
        assert(clause_view(arena.nodes@, &out) == clause@);
    }
    out
}

pub fn clauses_from_parser(
    arena: &ETermArena,
    clauses: &Vec<EDocClause>,
    Ghost(models): Ghost<Seq<DocClause>>,
) -> (out: Vec<EClause>)
    requires
        doc_clauses_roots_ok(arena.nodes@, clauses@, models),
        forall|i: int| 0 <= i < models.len() ==> wf_clause(#[trigger] models[i]),
    ensures
        db_valid(arena.nodes@, out@),
        db_view(arena.nodes@, out@) == models,
{
    proof {
        doc_clauses_roots_elim(arena.nodes@, clauses@, models);
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < clauses.len()
        invariant
            i <= clauses.len(),
            out@.len() == i,
            clauses@.len() == models.len(),
            forall|j: int| 0 <= j < clauses@.len() ==> (#[trigger] clauses@[j])@ == models[j],
            forall|j: int|
                0 <= j < clauses@.len() ==> #[trigger] doc_clause_roots_ok(
                    arena.nodes@,
                    &clauses@[j],
                ),
            forall|j: int| 0 <= j < models.len() ==> wf_clause(#[trigger] models[j]),
            db_valid(arena.nodes@, out@),
            db_view(arena.nodes@, out@) == models.take(i as int),
        decreases clauses.len() - i,
    {
        let clause = clause_from_parser(arena, &clauses[i]);
        let ghost before = out@;
        out.push(clause);
        proof {
            assert forall|j: int| 0 <= j < out@.len() implies #[trigger] clause_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(db_view(arena.nodes@, out@) == models.take(i as int + 1), j => {
                if j < before.len() { assert(out@[j] == before[j]); }
            });
        }
        i += 1;
    }
    out
}

proof fn bundle_clauses_wf(bundles: Seq<ckc_spec::v1text::Bundle>)
    requires
        forall|i: int|
            0 <= i < bundles.len() ==> ckc_spec::v1text::wf_bundle(#[trigger] bundles[i]),
    ensures
        forall|i: int|
            0 <= i < crate::v1_term_impl::doc_clause_models(bundles).len() ==> wf_clause(
                #[trigger] crate::v1_term_impl::doc_clause_models(bundles)[i],
            ),
    decreases bundles.len(),
{
    crate::v1_term_impl::doc_clause_models_flatten(bundles);
    if bundles.len() > 0 {
        let rest = bundles.drop_first();
        crate::v1_term_impl::doc_clause_models_flatten(rest);
        assert forall|i: int| 0 <= i < rest.len() implies ckc_spec::v1text::wf_bundle(
            #[trigger] rest[i],
        ) by {
            assert(rest[i] == bundles[i + 1]);
        }
        bundle_clauses_wf(rest);
        let parts = bundles.map_values(|b: ckc_spec::v1text::Bundle| b.clauses);
        let rest_parts = rest.map_values(|b: ckc_spec::v1text::Bundle| b.clauses);
        assert_seqs_equal!(parts.drop_first() == rest_parts);
        assert forall|i: int| 0 <= i < parts.flatten().len() implies wf_clause(
            #[trigger] parts.flatten()[i],
        ) by {
            if i < bundles[0].clauses.len() {
                assert(parts.flatten()[i] == bundles[0].clauses[i]);
            } else {
                assert(parts.flatten()[i] == rest_parts.flatten()[i - bundles[0].clauses.len()]);
            }
        }
    }
}

fn schema_root(arena: &mut ETermArena) -> (root: usize)
    requires
        crate::k2_term::arena_ok(old(arena)),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == ckc_spec::v1text::schema_version_term(),
{
    let name: &[u8] = b"guideline_schema_version";
    let one = int_root(arena, 1);
    let root = comp1(arena, name, one);
    proof {
        reveal_byteslit(b"guideline_schema_version");
        reveal_strlit("guideline_schema_version");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("guideline_schema_version"@));
    }
    root
}

fn document_record_root(
    arena: &mut ETermArena,
    parsed: &EParsedV1,
    Ghost(doc): Ghost<ckc_spec::v1text::DocFile>,
) -> (root: usize)
    requires
        crate::k2_term::arena_ok(old(arena)),
        crate::v1_term_impl::parsed_metadata_ok(parsed),
        parsed@ == ckc_spec::v1text::V1File::Doc(doc),
        ckc_spec::v1text::wf_doc(doc),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == ckc_spec::v1text::doc_record_term(doc),
{
    let none_bytes: &[u8] = b"none";
    let sha_bytes: &[u8] = b"sha256";
    let ace_bytes: &[u8] = b"ace_sha256";
    let ulex_bytes: &[u8] = b"ulex";
    let doc_bytes: &[u8] = b"guideline_document";
    proof {
        reveal_byteslit(b"none");
        reveal_strlit("none");
        reveal_byteslit(b"sha256");
        reveal_strlit("sha256");
        reveal_byteslit(b"ace_sha256");
        reveal_strlit("ace_sha256");
        reveal_byteslit(b"ulex");
        reveal_strlit("ulex");
        reveal_byteslit(b"guideline_document");
        reveal_strlit("guideline_document");
        reveal(ckc_spec::v1text::ascii);
        assert(none_bytes@ == ckc_spec::v1text::ascii("none"@));
        assert(sha_bytes@ == ckc_spec::v1text::ascii("sha256"@));
        assert(ace_bytes@ == ckc_spec::v1text::ascii("ace_sha256"@));
        assert(ulex_bytes@ == ckc_spec::v1text::ascii("ulex"@));
        assert(doc_bytes@ == ckc_spec::v1text::ascii("guideline_document"@));
    }
    let ulex = if parsed.doc_ulex.len() == 0 {
        proof {
            assert(doc.ulex is None);
        }
        atom_root(arena, none_bytes)
    } else {
        let digest = atom_root(arena, &parsed.doc_ulex);
        comp1(arena, sha_bytes, digest)
    };
    proof {
        assert(arena@[ulex as int] == ckc_spec::v1text::ulex_term(doc.ulex));
    }
    let ulex_field = comp1(arena, ulex_bytes, ulex);
    let ace = atom_root(arena, &parsed.doc_ace);
    let ace_field = comp1(arena, ace_bytes, ace);
    let docid = atom_root(arena, &parsed.docid);
    comp3(arena, doc_bytes, docid, ace_field, ulex_field)
}

pub fn document_db(
    arena: &mut ETermArena,
    parsed: &EParsedV1,
    Ghost(doc): Ghost<ckc_spec::v1text::DocFile>,
) -> (out: Vec<EClause>)
    requires
        crate::k2_term::arena_ok(old(arena)),
        crate::v1_term_impl::parsed_doc_roots_ok(old(arena).nodes@, parsed),
        crate::v1_term_impl::parsed_metadata_ok(parsed),
        parsed@ == ckc_spec::v1text::V1File::Doc(doc),
        ckc_spec::v1text::wf_doc(doc),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        db_valid(final(arena).nodes@, out@),
        db_view(final(arena).nodes@, out@) == ckc_spec::replay::doc_db(doc),
{
    let ghost before = arena.nodes@;
    proof {
        crate::v1_term_impl::parsed_doc_roots_elim(arena.nodes@, parsed);
        crate::v1_term_impl::doc_clause_models_flatten(doc.bundles);
        bundle_clauses_wf(doc.bundles);
    }
    let clauses = clauses_from_parser(
        arena,
        &parsed.clauses,
        Ghost(doc.bundles.map_values(|b: ckc_spec::v1text::Bundle| b.clauses).flatten()),
    );
    let schema = schema_root(arena);
    let ghost with_schema = arena.nodes@;
    let record = document_record_root(arena, parsed, Ghost(doc));
    proof {
        crate::k2_term::arena_prefix_stable(with_schema, arena);
        crate::k2_engine::db_models_prefix(before, arena.nodes@, clauses@);
    }
    let schema_fact = fact_exec(arena, schema);
    let record_fact = fact_exec(arena, record);
    let mut out = Vec::new();
    out.push(schema_fact);
    out.push(record_fact);
    proof {
        assert(db_valid(arena.nodes@, out@));
        assert_seqs_equal!(db_view(arena.nodes@, out@) == seq![
            ckc_spec::engine::fact_clause(ckc_spec::v1text::schema_version_term()),
            ckc_spec::engine::fact_clause(ckc_spec::v1text::doc_record_term(doc)),
        ]);
    }
    append_db(arena, &mut out, clauses);
    out
}

fn copy_clause(arena: &ETermArena, clause: &EClause) -> (out: EClause)
    requires
        clause_valid(arena.nodes@, clause),
    ensures
        clause_valid(arena.nodes@, &out),
        clause_view(arena.nodes@, &out) == clause_view(arena.nodes@, clause),
{
    let mut body = Vec::new();
    let mut i = 0usize;
    while i < clause.body.len()
        invariant
            clause_valid(arena.nodes@, clause),
            i <= clause.body.len(),
            body.len() == i,
            crate::k2_engine::body_items_valid(arena.nodes@, body@),
            forall|j: int|
                0 <= j < i ==> crate::k2_engine::body_item_view(arena.nodes@, &body@[j])
                    == crate::k2_engine::body_item_view(arena.nodes@, &clause.body@[j]),
        decreases clause.body.len() - i,
    {
        let ghost before = body@;
        proof {
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &clause.body@[i as int]));
        }
        let item = match &clause.body[i] {
            EBodyItem::Pos { root } => EBodyItem::Pos { root: *root },
            EBodyItem::Naf { roots } => EBodyItem::Naf { roots: roots.clone() },
        };
        body.push(item);
        proof {
            assert forall|j: int| 0 <= j < body.len() implies crate::k2_engine::body_item_valid(
                arena.nodes@,
                &body@[j],
            ) && crate::k2_engine::body_item_view(arena.nodes@, &body@[j])
                == crate::k2_engine::body_item_view(arena.nodes@, &clause.body@[j]) by {
                if j < before.len() {
                    assert(body@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(body_items_view(arena.nodes@, body@) == body_items_view(arena.nodes@, clause.body@));
    }
    EClause { head: clause.head, body }
}

pub fn witness_db_exec(arena: &ETermArena, facts: &Vec<usize>, db: &Vec<EClause>) -> (out: Vec<
    EClause,
>)
    requires
        crate::k2_term::arena_ok(arena),
        roots_valid(arena.nodes@, facts@),
        db_valid(arena.nodes@, db@),
    ensures
        db_valid(arena.nodes@, out@),
        db_view(arena.nodes@, out@) == ckc_spec::engine::witness_db(
            root_terms(arena.nodes@, facts@),
            db_view(arena.nodes@, db@),
        ),
{
    let ghost models = root_terms(arena.nodes@, facts@).reverse().map_values(
        |t: ckc_spec::term::Term| ckc_spec::engine::fact_clause(t),
    );
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(db_view(arena.nodes@, out@) == models.take(0));
    }
    while i < facts.len()
        invariant
            crate::k2_term::arena_ok(arena),
            roots_valid(arena.nodes@, facts@),
            models == root_terms(arena.nodes@, facts@).reverse().map_values(
                |t: ckc_spec::term::Term| ckc_spec::engine::fact_clause(t),
            ),
            i <= facts.len(),
            out.len() == i,
            db_valid(arena.nodes@, out@),
            db_view(arena.nodes@, out@) == models.take(i as int),
        decreases facts.len() - i,
    {
        let ghost before = out@;
        let fact = fact_exec(arena, facts[facts.len() - 1 - i]);
        out.push(fact);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies clause_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(db_view(arena.nodes@, out@) == models.take(i as int + 1));
        }
        i += 1;
    }
    let mut j = 0usize;
    proof {
        assert_seqs_equal!(models.take(i as int) == models);
        assert_seqs_equal!(db_view(arena.nodes@, db@).take(0) == Seq::empty());
    }
    while j < db.len()
        invariant
            db_valid(arena.nodes@, db@),
            j <= db.len(),
            db_valid(arena.nodes@, out@),
            out.len() == models.len() + j,
            db_view(arena.nodes@, out@) == models + db_view(arena.nodes@, db@).take(j as int),
            models == root_terms(arena.nodes@, facts@).reverse().map_values(
                |t: ckc_spec::term::Term| ckc_spec::engine::fact_clause(t),
            ),
        decreases db.len() - j,
    {
        let ghost before = out@;
        proof {
            assert(clause_valid(arena.nodes@, &db@[j as int]));
        }
        let clause = copy_clause(arena, &db[j]);
        out.push(clause);
        proof {
            assert forall|k: int| 0 <= k < out.len() implies clause_valid(
                arena.nodes@,
                &out@[k],
            ) by {
                if k < before.len() {
                    assert(out@[k] == before[k]);
                }
            }
            let base = db_view(arena.nodes@, before);
            let next = db_view(arena.nodes@, db@)[j as int];
            assert(base == models + db_view(arena.nodes@, db@).take(j as int));
            assert_seqs_equal!(db_view(arena.nodes@, out@) == base.push(next), k => {
                if k < before.len() { assert(out@[k] == before[k]); }
                else { assert(db_view(arena.nodes@, out@)[k] == next); }
            });
            assert_seqs_equal!(base.push(next) == models + db_view(arena.nodes@, db@).take(j as int + 1));
        }
        j += 1;
    }
    proof {
        assert_seqs_equal!(db_view(arena.nodes@, db@).take(j as int) == db_view(arena.nodes@, db@));
    }
    out
}

} // verus!
