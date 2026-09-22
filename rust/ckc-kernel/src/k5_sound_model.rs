use crate::{k5_sound_corpus as corpus, k5_sound_literals as l, k5_sound_source as b};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn from(s: u::Bytes, root: u::Bytes, inputs: Seq<u::Bytes>) -> bool {
    inputs.contains(root) && b::span(s, root)
}

pub open spec fn backed(s: u::Bytes, inputs: Seq<u::Bytes>) -> bool {
    s.len() == 0 || exists|root: u::Bytes| #[trigger] from(s, root, inputs)
}

pub proof fn own(s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        inputs.contains(s),
    ensures
        backed(s, inputs),
{
    b::span_self(s);
    assert(from(s, s, inputs));
}

pub proof fn span(s: u::Bytes, root: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        b::span(s, root),
        backed(root, inputs),
    ensures
        backed(s, inputs),
{
    if root.len() == 0 {
        let a = choose|a: int| #[trigger] b::span_at(s, root, a);
    } else {
        let r = choose|r: u::Bytes| #[trigger] from(root, r, inputs);
        b::span_trans(s, root, r);
        assert(from(s, r, inputs));
    }
}

pub proof fn copy(s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        backed(s, inputs),
    ensures
        u::copy_derived(s, inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    if s.len() > 0 {
        let r = choose|r: u::Bytes| #[trigger] from(s, r, inputs);
        b::source(s, r, inputs);
    }
}

pub proof fn sub(s: u::Bytes, inputs: Seq<u::Bytes>, lo: int, hi: int)
    requires
        backed(s, inputs),
        0 <= lo <= hi <= s.len(),
    ensures
        backed(s.subrange(lo, hi), inputs),
{
    b::span_self(s);
    b::span_sub(s, s, lo, hi);
    span(s.subrange(lo, hi), s, inputs);
}

pub proof fn split(s: u::Bytes, inputs: Seq<u::Bytes>, delim: u8)
    requires
        backed(s, inputs),
    ensures
        forall|i: int|
            0 <= i < ck::split_on(s, delim).len() ==> backed(
                #[trigger] ck::split_on(s, delim)[i],
                inputs,
            ),
{
    b::split(s, delim);
    assert forall|i: int| 0 <= i < ck::split_on(s, delim).len() implies backed(
        #[trigger] ck::split_on(s, delim)[i],
        inputs,
    ) by {
        span(ck::split_on(s, delim)[i], s, inputs);
    }
}

pub proof fn at(xs: Seq<u::Bytes>, i: int, inputs: Seq<u::Bytes>)
    requires
        forall|j: int| 0 <= j < xs.len() ==> backed(#[trigger] xs[j], inputs),
    ensures
        backed(u::at(xs, i), inputs),
{
}

pub proof fn stripped(s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        backed(s, inputs),
    ensures
        backed(ck::strip_ws(s), inputs),
{
    b::stripped(s);
    span(ck::strip_ws(s), s, inputs);
}

pub proof fn unprefix(s: u::Bytes, p: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        backed(s, inputs),
    ensures
        backed(u::unprefix(s, p), inputs),
{
    b::unprefix(s, p);
    span(u::unprefix(s, p), s, inputs);
}

pub proof fn unsuffix(s: u::Bytes, p: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        backed(s, inputs),
    ensures
        backed(u::unsuffix(s, p), inputs),
{
    b::unsuffix(s, p);
    span(u::unsuffix(s, p), s, inputs);
}

pub proof fn filter(xs: Seq<u::Bytes>, pred: spec_fn(u::Bytes) -> bool, inputs: Seq<u::Bytes>)
    requires
        forall|j: int| 0 <= j < xs.len() ==> backed(#[trigger] xs[j], inputs),
    ensures
        forall|j: int|
            0 <= j < xs.filter(pred).len() ==> backed(#[trigger] xs.filter(pred)[j], inputs),
{
    assert forall|j: int| 0 <= j < xs.filter(pred).len() implies backed(
        #[trigger] xs.filter(pred)[j],
        inputs,
    ) by {
        xs.lemma_filter_contains_rev(pred, xs.filter(pred)[j]);
        let i = choose|i: int| 0 <= i < xs.len() && xs[i] == xs.filter(pred)[j];
    }
}

pub proof fn field(r: ck::Row, n: int, inputs: Seq<u::Bytes>)
    requires
        backed(r.line, inputs),
    ensures
        backed(u::field(r, n), inputs),
{
    unsuffix(r.line, u::lit("\n"@), inputs);
    split(u::unsuffix(r.line, u::lit("\n"@)), inputs, 9);
    at(ck::tab_fields(u::unsuffix(r.line, u::lit("\n"@))), n, inputs);
}

pub proof fn ace_row(rows: Seq<ck::Row>, id: u::Bytes)
    ensures
        match ck::first_ace_row(rows, id) {
            Some(r) => rows.contains(r),
            None => true,
        },
    decreases rows.len(),
{
    if rows.len() > 0 && ck::ace_docid(rows[0]) != Some(id) {
        ace_row(rows.drop_first(), id);
    }
}

pub proof fn coverage_field(g: u::Guideline, id: u::Bytes, n: int, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        backed(u::coverage_field(g, id, n), inputs),
{
    ace_row(g.coverage.rows, id);
    if let Some(r) = ck::ace_row(g.coverage, id) {
        corpus::row(g, r, inputs);
        own(r.line, inputs);
        field(r, n, inputs);
    }
}

pub proof fn split_copy(xs: Seq<u::Bytes>, inputs: Seq<u::Bytes>)
    requires
        forall|i: int| 0 <= i < xs.len() ==> backed(#[trigger] xs[i], inputs),
    ensures
        forall|i: int|
            0 <= i < xs.len() ==> u::copy_derived(#[trigger] xs[i], inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    assert forall|i: int| 0 <= i < xs.len() implies u::copy_derived(
        #[trigger] xs[i],
        inputs,
        u::copy_registry(),
    ) by {
        copy(xs[i], inputs);
    }
}

pub proof fn human_section(s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        backed(s, inputs),
    ensures
        u::copy_derived(u::human_section(s), inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    split(s, inputs, 62);
    let fields = ck::split_on(s, 62);
    let stripped_fields = fields.map_values(|x: u::Bytes| ck::strip_ws(x));
    assert forall|i: int| 0 <= i < stripped_fields.len() implies backed(
        #[trigger] stripped_fields[i],
        inputs,
    ) by {
        stripped(fields[i], inputs);
    }
    filter(stripped_fields, |x: u::Bytes| x.len() > 0, inputs);
    let ss = stripped_fields.filter(|x: u::Bytes| x.len() > 0);
    if ss.len() == 0 {
        copy(u::empty(), inputs);
    } else {
        split(ss[0], inputs, 32);
        let hs = ck::split_on(ss[0], 32);
        at(hs, 1, inputs);
        copy(u::at(hs, 1), inputs);
        copy(ss[0], inputs);
        let special = hs.len() == 2 && u::digits(u::at(hs, 1));
        l::l027(inputs);
        l::l029(inputs);
        b::cat(u::lit("Recommendation "@), u::at(hs, 1), inputs);
        let head = if special && u::at(hs, 0) == u::lit("Rec"@) {
            seq![u::lit("Recommendation "@) + u::at(hs, 1)]
        } else if special && u::at(hs, 0) == u::lit("BOX"@) && ss.len() > 1 {
            Seq::empty()
        } else {
            seq![ss[0]]
        };
        split_copy(ss.drop_first(), inputs);
        b::join(head + ss.drop_first(), u::lit(" · "@), inputs);
    }
}

pub proof fn document_title(g: u::Guideline, id: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
        backed(id, inputs),
    ensures
        u::copy_derived(u::document_title(g, id), inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::coverage_field);
    hide(u::human_section);
    coverage_field(g, id, 3, inputs);
    coverage_field(g, id, 0, inputs);
    coverage_field(g, id, 2, inputs);
    let section = u::coverage_field(g, id, 3);
    human_section(section, inputs);
    let base = u::human_section(section);
    let region = u::coverage_field(g, id, 0);
    stripped(u::coverage_field(g, id, 2), inputs);
    unprefix(ck::strip_ws(u::coverage_field(g, id, 2)), u::lit("p"@), inputs);
    let page = u::unprefix(ck::strip_ws(u::coverage_field(g, id, 2)), u::lit("p"@));
    split(region, inputs, 45);
    let segs = ck::split_on(region, 45);
    at(segs, segs.len() - 1, inputs);
    let last = u::at(segs, segs.len() - 1);
    if base.len() == 0 {
        copy(id, inputs);
    } else if g.documents.filter(
        |d: u::Document| u::coverage_field(g, d.bundle.docid, 3) == section,
    ).len() > 1 {
        if u::digits(page) && u::digits(last) {
            l::l031(inputs);
            l::l032(inputs);
            copy(page, inputs);
            b::decimal(ck::dec_of(last), inputs);
            b::cat(base, u::lit(", page "@), inputs);
            b::cat(base + u::lit(", page "@), page, inputs);
            b::cat(base + u::lit(", page "@) + page, u::lit(", passage "@), inputs);
            b::cat(
                base + u::lit(", page "@) + page + u::lit(", passage "@),
                ck::nat_bytes(ck::dec_of(last)),
                inputs,
            );
        } else {
            l::l033(inputs);
            l::l034(inputs);
            copy(region, inputs);
            b::cat(base, u::lit(" ("@), inputs);
            b::cat(base + u::lit(" ("@), region, inputs);
            b::cat(base + u::lit(" ("@) + region, u::lit(")"@), inputs);
        }
    }
}

} // verus!
