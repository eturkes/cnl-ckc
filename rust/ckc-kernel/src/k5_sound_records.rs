use crate::{
    k5_sound_build as b, k5_sound_corpus as corpus, k5_sound_escape as h, k5_sound_literals as l,
    k5_sound_model as m, k5_sound_record_source as source, k5_sound_source as bytes,
    k5_sound_summary as summary,
};
use ckc_spec::ui as u;
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn version(r: u::Record, label: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        u::copy_derived(label, inputs, u::copy_registry()),
    ensures
        h::sound(u::version_link(r, label), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    h::text(label, inputs);
    if r.decision.commit.len() > 0 {
        b::link(
            u::lit("https://github.com/eturkes/cnl-ckc/commit/"@) + r.decision.commit,
            u::text(label),
            inputs,
        );
    }
}

pub proof fn row(g: u::Guideline, r: u::Record, inputs: Seq<u::Bytes>)
    requires
        source::record(r, inputs),
    ensures
        h::sound(u::record_row(g, r), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::human_date);
    hide(u::current);
    hide(u::cell);
    hide(u::row);
    hide(u::version_link);
    let k = if r.decision.approved {
        0
    } else {
        1
    };
    b::state_label(k, inputs);
    h::text(u::state_label(k), inputs);
    b::cell(u::text(u::state_label(k)), inputs);
    m::copy(r.reviewer, inputs);
    h::text(r.reviewer, inputs);
    b::cell(u::text(r.reviewer), inputs);
    source::human_date(r.decision.date, inputs);
    h::text(u::human_date(r.decision.date), inputs);
    b::cell(u::text(u::human_date(r.decision.date)), inputs);
    l::l110(inputs);
    l::l111(inputs);
    let label = if u::current(g, r) {
        u::lit("Current"@)
    } else {
        u::lit("Earlier"@)
    };
    version(r, label, inputs);
    b::cell(u::version_link(r, label), inputs);
    l::l112(inputs);
    m::copy(r.comment, inputs);
    let comment = if r.comment.len() == 0 {
        u::lit("Not given"@)
    } else {
        r.comment
    };
    h::text(comment, inputs);
    b::cell(u::text(comment), inputs);
    b::row(
        seq![
            u::cell(u::text(u::state_label(k))),
            u::cell(u::text(r.reviewer)),
            u::cell(u::text(u::human_date(r.decision.date))),
            u::cell(u::version_link(r, label)),
            u::cell(u::text(comment)),
        ],
        inputs,
    );
}

pub proof fn section(g: u::Guideline, id: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
        m::backed(id, inputs),
    ensures
        forall|i: int|
            0 <= i < u::record_section(g, id).len() ==> h::sound(
                #[trigger] u::record_section(g, id)[i],
                inputs,
            ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::history);
    hide(u::record_row);
    hide(u::document_title);
    hide(u::url_seg);
    hide(u::link);
    hide(u::lines);
    let hs = u::history(g, id);
    source::history(g, id, inputs);
    if hs.len() > 0 {
        let rows = Seq::new(hs.len(), |i: int| u::record_row(g, hs[hs.len() - i - 1]));
        assert forall|i: int| 0 <= i < rows.len() implies h::sound(#[trigger] rows[i], inputs) by {
            row(g, hs[hs.len() - i - 1], inputs);
        }
        b::lines(rows, inputs);
        l::f113(inputs);
        l::f011(inputs);
        h::attr(id, inputs, 2);
        h::add(b::f(113), u::attr(id), inputs, 0, 2, 2);
        h::add(b::f(113) + u::attr(id), b::f(11), inputs, 0, 2, 0);
        m::document_title(g, id, inputs);
        h::text(u::document_title(g, id), inputs);
        let href = u::lit("doc/"@) + u::url_seg(id) + u::lit(".html"@);
        b::link(href, u::text(u::document_title(g, id)), inputs);
        l::f114(inputs);
        l::f115(inputs);
        b::add(b::f(114), u::link(href, u::text(u::document_title(g, id))), inputs);
        b::add(b::f(114) + u::link(href, u::text(u::document_title(g, id))), b::f(115), inputs);
        l::f116(inputs);
        l::f117(inputs);
        l::f083(inputs);
        l::f084(inputs);
        l::f085(inputs);
        l::f086(inputs);
        b::add(b::f(83), u::lines(rows), inputs);
        b::add(b::f(83) + u::lines(rows), b::f(84), inputs);
    }
}

pub proof fn flatten_sections(xs: Seq<Seq<u::Html>>, inputs: Seq<u::Bytes>)
    requires
        forall|i: int, j: int|
            0 <= i < xs.len() && 0 <= j < xs[i].len() ==> h::sound(#[trigger] xs[i][j], inputs),
    ensures
        forall|i: int| 0 <= i < xs.flatten().len() ==> h::sound(#[trigger] xs.flatten()[i], inputs),
    decreases xs.len(),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    if xs.len() > 0 {
        flatten_sections(xs.drop_first(), inputs);
        assert forall|i: int| 0 <= i < xs.flatten().len() implies h::sound(
            #[trigger] xs.flatten()[i],
            inputs,
        ) by {
            if i < xs[0].len() {
                assert(xs.flatten()[i] == xs[0][i]);
            } else {
                assert(xs.flatten()[i] == xs.drop_first().flatten()[i - xs[0].len()]);
            }
        }
    }
}

pub proof fn page(g: u::Guideline, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        h::sound(u::records_html(g), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::record_section);
    hide(u::records);
    hide(u::decisions);
    hide(u::review_summary);
    hide(u::title);
    hide(u::lines);
    hide(u::frame);
    let bags = g.documents.map_values(|d: u::Document| u::record_section(g, d.bundle.docid));
    assert forall|i: int, j: int| 0 <= i < bags.len() && 0 <= j < bags[i].len() implies h::sound(
        #[trigger] bags[i][j],
        inputs,
    ) by {
        let d = g.documents[i];
        assert(g.documents.contains(d));
        corpus::document(g, d, inputs);
        m::own(d.bundle.docid, inputs);
        section(g, d.bundle.docid, inputs);
    }
    flatten_sections(bags, inputs);
    let sections = bags.flatten();
    summary::review(g, inputs);
    l::l118(inputs);
    m::copy(u::empty(), inputs);
    let suffix = if u::decisions(g).len() > 0 {
        u::lit(" The newest decision for each document is first."@)
    } else {
        u::empty()
    };
    bytes::cat(u::review_summary(g), suffix, inputs);
    let summary = u::review_summary(g) + suffix;
    l::f119(inputs);
    l::f120(inputs);
    l::f121(inputs);
    let notes = if sections.len() == 0 {
        seq![b::f(119)]
    } else {
        seq![b::f(120)] + (if u::records(g).filter(|r: u::Record| r.decision.commit.len() > 0).len()
            > 0 {
            seq![b::f(121)]
        } else {
            Seq::empty()
        })
    };
    l::l122(inputs);
    l::f123(inputs);
    l::f124(inputs);
    l::f125(inputs);
    l::f099(inputs);
    l::f126(inputs);
    l::f127(inputs);
    corpus::title(g, inputs);
    h::text(u::title(g), inputs);
    b::add(b::f(123), u::text(u::title(g)), inputs);
    b::add(b::f(123) + u::text(u::title(g)), b::f(124), inputs);
    h::text(summary, inputs);
    b::add(b::f(99), u::text(summary), inputs);
    b::add(b::f(99) + u::text(summary), b::f(126), inputs);
    let parts = seq![b::f(125), b::f(99) + u::text(summary) + b::f(126)] + sections + notes + seq![
        b::f(127),
    ];
    b::lines(parts, inputs);
    b::frame(
        u::lit("Decision records"@),
        b::f(123) + u::text(u::title(g)) + b::f(124),
        u::lines(parts),
        inputs,
    );
    assert(u::records_html(g) == u::frame(
        u::lit("Decision records"@),
        b::f(123) + u::text(u::title(g)) + b::f(124),
        u::lines(parts),
    ));
}

} // verus!
