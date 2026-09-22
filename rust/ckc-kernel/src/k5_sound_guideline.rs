use crate::{
    k5_sound_build as b, k5_sound_corpus as corpus, k5_sound_escape as h, k5_sound_literals as l,
    k5_sound_model as m, k5_sound_summary as summary,
};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn page(g: u::Guideline, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        h::sound(u::guideline_html(g), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::title);
    hide(u::class_counts);
    hide(ck::count_status);
    hide(u::state);
    hide(u::tally);
    hide(u::tally_cell);
    hide(u::review_summary);
    hide(u::document_title);
    hide(u::coverage_field);
    hide(u::region_status);
    hide(u::field);
    hide(u::cell);
    hide(u::row);
    hide(u::link);
    hide(u::chip);
    hide(u::frame);
    hide(u::lines);
    hide(u::number);
    hide(u::url_seg);
    l::l093(inputs);
    l::l094(inputs);
    l::l092(inputs);
    l::l018(inputs);
    l::l019(inputs);
    l::l020(inputs);
    l::l021(inputs);
    l::l022(inputs);
    let labels = seq![
        b::c(93),
        b::c(94),
        b::c(92),
        b::c(18),
        b::c(19),
        b::c(20),
        b::c(21),
        b::c(22),
    ];
    let counts = seq![
        g.coverage.rows.len(),
        ck::count_status(g.coverage.rows, 1),
        ck::count_status(g.coverage.rows, 0),
    ] + u::class_counts(g);
    let status_rows = Seq::new(
        labels.len(),
        |i: int|
            u::row(
                seq![
                    u::cell(u::text(labels[i])),
                    u::cell(
                        u::number(
                            if i < counts.len() {
                                counts[i]
                            } else {
                                0
                            },
                        ),
                    ),
                ],
            ),
    );
    assert forall|i: int| 0 <= i < status_rows.len() implies h::sound(
        #[trigger] status_rows[i],
        inputs,
    ) by {
        h::text(labels[i], inputs);
        b::cell(u::text(labels[i]), inputs);
        let n = if i < counts.len() {
            counts[i]
        } else {
            0
        };
        h::number(n, inputs, 0);
        b::cell(u::number(n), inputs);
        b::row(seq![u::cell(u::text(labels[i])), u::cell(u::number(n))], inputs);
    }
    let doc_rows = g.documents.map_values(
        |d: u::Document|
            {
                let id = d.bundle.docid;
                u::row(
                    seq![
                        u::cell(
                            u::link(
                                u::lit("doc/"@) + u::url_seg(id) + u::lit(".html"@),
                                u::text(u::document_title(g, id)),
                            ),
                        ),
                        u::cell(u::chip(u::state(g, id))),
                        u::cell(u::text(u::tally_cell(u::tally(g, id)))),
                        u::cell(u::text(u::coverage_field(g, id, 0))),
                    ],
                )
            },
    );
    assert forall|i: int| 0 <= i < doc_rows.len() implies h::sound(
        #[trigger] doc_rows[i],
        inputs,
    ) by {
        let d = g.documents[i];
        let id = d.bundle.docid;
        assert(g.documents.contains(d));
        corpus::document(g, d, inputs);
        m::own(id, inputs);
        m::document_title(g, id, inputs);
        h::text(u::document_title(g, id), inputs);
        let href = u::lit("doc/"@) + u::url_seg(id) + u::lit(".html"@);
        b::link(href, u::text(u::document_title(g, id)), inputs);
        b::cell(u::link(href, u::text(u::document_title(g, id))), inputs);
        b::chip(u::state(g, id), inputs);
        b::cell(u::chip(u::state(g, id)), inputs);
        summary::tally_cell(u::tally(g, id), inputs);
        h::text(u::tally_cell(u::tally(g, id)), inputs);
        b::cell(u::text(u::tally_cell(u::tally(g, id))), inputs);
        m::coverage_field(g, id, 0, inputs);
        m::copy(u::coverage_field(g, id, 0), inputs);
        h::text(u::coverage_field(g, id, 0), inputs);
        b::cell(u::text(u::coverage_field(g, id, 0)), inputs);
        b::row(
            seq![
                u::cell(u::link(href, u::text(u::document_title(g, id)))),
                u::cell(u::chip(u::state(g, id))),
                u::cell(u::text(u::tally_cell(u::tally(g, id)))),
                u::cell(u::text(u::coverage_field(g, id, 0))),
            ],
            inputs,
        );
    }
    let rs = g.coverage.rows.filter(|r: ck::Row| !matches!(r.status,ck::Status::Ace(_)));
    let others = rs.map_values(
        |r: ck::Row|
            u::row(
                seq![
                    u::cell(u::text(r.id)),
                    u::cell(u::text(u::region_status(r))),
                    u::cell(u::text(u::field(r, 3))),
                ],
            ),
    );
    assert forall|i: int| 0 <= i < others.len() implies h::sound(#[trigger] others[i], inputs) by {
        let r = rs[i];
        assert(rs.contains(r));
        g.coverage.rows.lemma_filter_contains_rev(
            |r: ck::Row| !matches!(r.status,ck::Status::Ace(_)),
            r,
        );
        corpus::row(g, r, inputs);
        m::own(r.id, inputs);
        m::own(r.line, inputs);
        m::copy(r.id, inputs);
        summary::region(r, inputs);
        m::field(r, 3, inputs);
        m::copy(u::field(r, 3), inputs);
        h::text(r.id, inputs);
        h::text(u::region_status(r), inputs);
        h::text(u::field(r, 3), inputs);
        b::cell(u::text(r.id), inputs);
        b::cell(u::text(u::region_status(r)), inputs);
        b::cell(u::text(u::field(r, 3)), inputs);
        b::row(
            seq![
                u::cell(u::text(r.id)),
                u::cell(u::text(u::region_status(r))),
                u::cell(u::text(u::field(r, 3))),
            ],
            inputs,
        );
    }
    b::lines(status_rows, inputs);
    b::lines(doc_rows, inputs);
    b::lines(others, inputs);
    corpus::title(g, inputs);
    h::text(u::title(g), inputs);
    summary::review(g, inputs);
    h::text(u::review_summary(g), inputs);
    l::f097(inputs);
    l::f098(inputs);
    l::f099(inputs);
    l::f100(inputs);
    l::f080(inputs);
    l::f101(inputs);
    l::f102(inputs);
    l::f103(inputs);
    l::f083(inputs);
    l::f084(inputs);
    l::f085(inputs);
    l::f086(inputs);
    l::f104(inputs);
    l::f105(inputs);
    l::f106(inputs);
    l::f107(inputs);
    l::f108(inputs);
    b::add(b::f(97), u::text(u::title(g)), inputs);
    b::add(b::f(97) + u::text(u::title(g)), b::f(98), inputs);
    b::add(b::f(99), u::text(u::review_summary(g)), inputs);
    b::add(b::f(99) + u::text(u::review_summary(g)), b::f(100), inputs);
    b::add(b::f(83), u::lines(status_rows), inputs);
    b::add(b::f(83) + u::lines(status_rows), b::f(84), inputs);
    b::add(b::f(83), u::lines(doc_rows), inputs);
    b::add(b::f(83) + u::lines(doc_rows), b::f(84), inputs);
    b::add(b::f(83), u::lines(others), inputs);
    b::add(b::f(83) + u::lines(others), b::f(84), inputs);
    let parts = seq![
        b::f(97) + u::text(u::title(g)) + b::f(98),
        b::f(99) + u::text(u::review_summary(g)) + b::f(100),
        b::f(80),
        b::f(101),
        b::f(102),
        b::f(103),
        b::f(83) + u::lines(status_rows) + b::f(84),
        b::f(85),
        b::f(86),
        b::f(80),
        b::f(104),
        b::f(102),
        b::f(105),
        b::f(83) + u::lines(doc_rows) + b::f(84),
        b::f(85),
        b::f(86),
        b::f(80),
        b::f(106),
        b::f(102),
        b::f(107),
        b::f(83) + u::lines(others) + b::f(84),
        b::f(85),
        b::f(86),
    ];
    b::lines(parts, inputs);
    b::add(b::f(108), u::text(u::title(g)), inputs);
    b::frame(u::title(g), b::f(108) + u::text(u::title(g)), u::lines(parts), inputs);
    assert(u::guideline_html(g) == u::frame(
        u::title(g),
        b::f(108) + u::text(u::title(g)),
        u::lines(parts),
    ));
}

} // verus!
