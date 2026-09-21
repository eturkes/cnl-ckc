use crate::k5_sound_source as b;
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn inputs(g: u::Guideline) -> Seq<u::Bytes> {
    seq![g.gid, u::ledger_data(g.ledger)] + (match g.readme {
        Some(x) => seq![x],
        None => Seq::empty(),
    }) + g.coverage.rows.map_values(|r: ck::Row| seq![r.id, r.line]).flatten()
        + g.coverage.evidence.map_values(
        |e: ck::Evidence|
            e.ordinal + e.payloads.map_values(|p: (u::Bytes, Seq<u::Bytes>)| p.1).flatten(),
    ).flatten() + g.documents.map_values(
        |d: u::Document| seq![d.bundle.docid, d.ace, d.pl],
    ).flatten()
}

pub open spec fn sources(g: u::Guideline, xs: Seq<u::Bytes>) -> bool {
    forall|s: u::Bytes| #[trigger] inputs(g).contains(s) ==> xs.contains(s)
}

pub proof fn flat<T>(xs: Seq<Seq<T>>, i: int, s: T)
    requires
        0 <= i < xs.len(),
        xs[i].contains(s),
    ensures
        xs.flatten().contains(s),
    decreases xs.len(),
{
    if i > 0 {
        flat(xs.drop_first(), i - 1, s);
    }
}

pub proof fn guideline(c: u::Corpus, i: int)
    requires
        0 <= i < c.guidelines.len(),
    ensures
        sources(c.guidelines[i], u::corpus_bytes(c)),
{
    let bags = c.guidelines.map_values(|g: u::Guideline| inputs(g));
    let raw = c.guidelines.map_values(
        |g: u::Guideline|
            {
                seq![g.gid, u::ledger_data(g.ledger)] + (match g.readme {
                    Some(x) => seq![x],
                    None => Seq::empty(),
                }) + g.coverage.rows.map_values(|r: ck::Row| seq![r.id, r.line]).flatten()
                    + g.coverage.evidence.map_values(
                    |e: ck::Evidence|
                        e.ordinal + e.payloads.map_values(
                            |p: (u::Bytes, Seq<u::Bytes>)| p.1,
                        ).flatten(),
                ).flatten() + g.documents.map_values(
                    |d: u::Document| seq![d.bundle.docid, d.ace, d.pl],
                ).flatten()
            },
    );
    assert(bags =~= raw);
    assert(bags.flatten() == u::corpus_bytes(c));
    assert forall|s: u::Bytes| #[trigger]
        inputs(c.guidelines[i]).contains(s) implies u::corpus_bytes(c).contains(s) by {
        flat(bags, i, s);
    }
}

pub proof fn basics(g: u::Guideline, xs: Seq<u::Bytes>)
    requires
        sources(g, xs),
    ensures
        xs.contains(g.gid),
        xs.contains(u::ledger_data(g.ledger)),
        match g.readme {
            Some(x) => xs.contains(x),
            None => true,
        },
{
    assert(inputs(g)[0] == g.gid);
    assert(inputs(g).contains(g.gid));
    assert(inputs(g)[1] == u::ledger_data(g.ledger));
    assert(inputs(g).contains(u::ledger_data(g.ledger)));
    if let Some(x) = g.readme {
        assert(inputs(g)[2] == x);
        assert(inputs(g).contains(x));
    }
}

pub proof fn row(g: u::Guideline, r: ck::Row, xs: Seq<u::Bytes>)
    requires
        sources(g, xs),
        g.coverage.rows.contains(r),
    ensures
        xs.contains(r.id),
        xs.contains(r.line),
{
    let i = choose|i: int| 0 <= i < g.coverage.rows.len() && g.coverage.rows[i] == r;
    let bags = g.coverage.rows.map_values(|r: ck::Row| seq![r.id, r.line]);
    flat(bags, i, r.id);
    flat(bags, i, r.line);
    assert(inputs(g).contains(r.id));
    assert(inputs(g).contains(r.line));
}

pub proof fn document(g: u::Guideline, d: u::Document, xs: Seq<u::Bytes>)
    requires
        sources(g, xs),
        g.documents.contains(d),
    ensures
        xs.contains(d.bundle.docid),
        xs.contains(d.ace),
        xs.contains(d.pl),
{
    let i = choose|i: int| 0 <= i < g.documents.len() && g.documents[i] == d;
    let bags = g.documents.map_values(|d: u::Document| seq![d.bundle.docid, d.ace, d.pl]);
    flat(bags, i, d.bundle.docid);
    flat(bags, i, d.ace);
    flat(bags, i, d.pl);
    assert(inputs(g).contains(d.bundle.docid));
    assert(inputs(g).contains(d.ace));
    assert(inputs(g).contains(d.pl));
}

pub proof fn first_title(ls: Seq<u::Bytes>, fallback: u::Bytes, root: u::Bytes)
    requires
        forall|i: int| 0 <= i < ls.len() ==> b::span(#[trigger] ls[i], root),
    ensures
        u::first_title(ls, fallback) == fallback || b::span(u::first_title(ls, fallback), root),
    decreases ls.len(),
{
    reveal_strlit("# ");
    vstd::utf8::is_ascii_chars_encode_utf8("# "@);
    if ls.len() > 0 {
        if ck::starts(ls[0], u::lit("# "@)) && ck::strip_ws(ls[0].skip(2)).len() > 0 {
            b::span_sub(ls[0], root, 2, ls[0].len() as int);
            assert(ls[0].skip(2) =~= ls[0].subrange(2, ls[0].len() as int));
            b::stripped(ls[0].skip(2));
            b::span_trans(ck::strip_ws(ls[0].skip(2)), ls[0].skip(2), root);
        } else {
            first_title(ls.drop_first(), fallback, root);
        }
    }
}

pub proof fn title(g: u::Guideline, xs: Seq<u::Bytes>)
    requires
        sources(g, xs),
    ensures
        u::copy_derived(u::title(g), xs, u::copy_registry()),
{
    hide(u::copy_registry);
    basics(g, xs);
    match g.readme {
        None => {
            b::span_self(g.gid);
            b::source(g.gid, g.gid, xs);
        },
        Some(root) => {
            b::split(root, 10);
            first_title(ck::split_on(root, 10), g.gid, root);
            if u::title(g) == g.gid {
                b::span_self(g.gid);
                b::source(g.gid, g.gid, xs);
            } else {
                b::source(u::title(g), root, xs);
            }
        },
    }
}

} // verus!
