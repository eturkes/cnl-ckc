use crate::{k5_sound_provenance as p, k5_sound_source as source};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

pub open spec fn block(g: u::Guideline) -> Seq<u::Bytes> {
    seq![g.gid, u::ledger_data(g.ledger)] + (match g.readme {
        Some(b) => seq![b],
        None => Seq::empty(),
    }) + g.coverage.rows.map_values(|r: ck::Row| seq![r.id, r.line]).flatten()
        + g.coverage.evidence.map_values(
        |e: ck::Evidence|
            e.ordinal + e.payloads.map_values(|p: (u::Bytes, Seq<u::Bytes>)| p.1).flatten(),
    ).flatten() + g.documents.map_values(
        |d: u::Document| seq![d.bundle.docid, d.ace, d.pl],
    ).flatten()
}

pub proof fn decomposition(c: u::Corpus)
    ensures
        u::corpus_bytes(c) == c.guidelines.map_values(|g: u::Guideline| block(g)).flatten(),
{
    assert((|g: u::Guideline| block(g)) =~= (|g: u::Guideline|
        seq![g.gid, u::ledger_data(g.ledger)] + (match g.readme {
            Some(b) => seq![b],
            None => Seq::empty(),
        }) + g.coverage.rows.map_values(|r: ck::Row| seq![r.id, r.line]).flatten()
            + g.coverage.evidence.map_values(
            |e: ck::Evidence|
                e.ordinal + e.payloads.map_values(|p: (u::Bytes, Seq<u::Bytes>)| p.1).flatten(),
        ).flatten() + g.documents.map_values(
            |d: u::Document| seq![d.bundle.docid, d.ace, d.pl],
        ).flatten()));
}

pub proof fn in_corpus(c: u::Corpus, i: int, b: u::Bytes)
    requires
        0 <= i < c.guidelines.len(),
        block(c.guidelines[i]).contains(b),
    ensures
        u::corpus_bytes(c).contains(b),
{
    decomposition(c);
    let bs = c.guidelines.map_values(|g: u::Guideline| block(g));
    assert(bs[i] == block(c.guidelines[i]));
    let j = choose|j: int|
        0 <= j < block(c.guidelines[i]).len() && #[trigger] block(c.guidelines[i])[j] == b;
    source::flat_member(bs, i, j);
}

pub proof fn gid(c: u::Corpus, i: int)
    requires
        0 <= i < c.guidelines.len(),
    ensures
        u::copied_span(c.guidelines[i].gid, u::corpus_bytes(c)),
{
    let g = c.guidelines[i];
    assert(block(g).len() >= 2);
    assert(block(g)[0] == g.gid);
    assert(block(g).contains(g.gid));
    in_corpus(c, i, g.gid);
    p::copied_member(g.gid, u::corpus_bytes(c));
}

pub proof fn readme(c: u::Corpus, i: int, b: u::Bytes)
    requires
        0 <= i < c.guidelines.len(),
        c.guidelines[i].readme == Some(b),
    ensures
        u::copied_span(b, u::corpus_bytes(c)),
{
    let g = c.guidelines[i];
    assert(block(g).len() >= 3);
    assert(block(g)[2] == b);
    assert(block(g).contains(b));
    in_corpus(c, i, b);
    p::copied_member(b, u::corpus_bytes(c));
}

pub proof fn title(c: u::Corpus, i: int)
    requires
        0 <= i < c.guidelines.len(),
    ensures
        u::copied_span(u::title(c.guidelines[i]), u::corpus_bytes(c)),
{
    let g = c.guidelines[i];
    gid(c, i);
    match g.readme {
        Some(b) => {
            readme(c, i, b);
            source::split_spans(b, 10, u::corpus_bytes(c));
            source::first_title_span(ck::split_on(b, 10), g.gid, u::corpus_bytes(c));
        },
        None => {},
    }
}

pub proof fn row_bytes(c: u::Corpus, i: int, j: int)
    requires
        0 <= i < c.guidelines.len(),
        0 <= j < c.guidelines[i].coverage.rows.len(),
    ensures
        u::copied_span(c.guidelines[i].coverage.rows[j].id, u::corpus_bytes(c)),
        u::copied_span(c.guidelines[i].coverage.rows[j].line, u::corpus_bytes(c)),
{
    let g = c.guidelines[i];
    let r = g.coverage.rows[j];
    let prefix = seq![g.gid, u::ledger_data(g.ledger)] + (match g.readme {
        Some(b) => seq![b],
        None => Seq::empty(),
    });
    let rows = g.coverage.rows.map_values(|r: ck::Row| seq![r.id, r.line]);
    let evidence = g.coverage.evidence.map_values(
        |e: ck::Evidence|
            e.ordinal + e.payloads.map_values(|p: (u::Bytes, Seq<u::Bytes>)| p.1).flatten(),
    ).flatten();
    let docs = g.documents.map_values(|d: u::Document| seq![d.bundle.docid, d.ace, d.pl]).flatten();
    source::flat_member(rows, j, 0);
    source::flat_member(rows, j, 1);
    assert(rows[j][0] == r.id);
    assert(rows[j][1] == r.line);
    source::append_member(prefix, rows.flatten(), r.id);
    source::append_member(prefix, rows.flatten(), r.line);
    source::append_member(prefix + rows.flatten(), evidence, r.id);
    source::append_member(prefix + rows.flatten(), evidence, r.line);
    source::append_member(prefix + rows.flatten() + evidence, docs, r.id);
    source::append_member(prefix + rows.flatten() + evidence, docs, r.line);
    in_corpus(c, i, r.id);
    in_corpus(c, i, r.line);
    p::copied_member(r.id, u::corpus_bytes(c));
    p::copied_member(r.line, u::corpus_bytes(c));
}

} // verus!
