use crate::{
    k5_sound_build as b, k5_sound_corpus as corpus, k5_sound_escape as h, k5_sound_literals as l,
};
use ckc_spec::ui as u;
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn index(c: u::Corpus)
    ensures
        h::sound(u::index_html(c), u::corpus_bytes(c)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::corpus_bytes);
    hide(u::title);
    hide(u::class_counts);
    hide(u::url_seg);
    hide(u::cell);
    hide(u::row);
    hide(u::link);
    hide(u::frame);
    hide(u::number);
    let inputs = u::corpus_bytes(c);
    let rows = c.guidelines.map_values(
        |g: u::Guideline|
            u::row(
                seq![
                    u::cell(
                        u::link(
                            u::lit("g/"@) + u::url_seg(g.gid) + u::lit("/index.html"@),
                            u::text(u::title(g)),
                        ),
                    ),
                    u::cell(u::number(g.documents.len())),
                    u::cell(u::number(g.coverage.rows.len())),
                ] + u::class_counts(g).map_values(|n: nat| u::cell(u::number(n))),
            ),
    );
    assert forall|i: int| 0 <= i < rows.len() implies h::sound(#[trigger] rows[i], inputs) by {
        let g = c.guidelines[i];
        corpus::guideline(c, i);
        corpus::title(g, inputs);
        h::text(u::title(g), inputs);
        let href = u::lit("g/"@) + u::url_seg(g.gid) + u::lit("/index.html"@);
        b::link(href, u::text(u::title(g)), inputs);
        b::cell(u::link(href, u::text(u::title(g))), inputs);
        h::number(g.documents.len(), inputs, 0);
        b::cell(u::number(g.documents.len()), inputs);
        h::number(g.coverage.rows.len(), inputs, 0);
        b::cell(u::number(g.coverage.rows.len()), inputs);
        let counts = u::class_counts(g);
        let cells = counts.map_values(|n: nat| u::cell(u::number(n)));
        assert forall|j: int| 0 <= j < cells.len() implies h::sound(
            #[trigger] cells[j],
            inputs,
        ) by {
            h::number(counts[j], inputs, 0);
            b::cell(u::number(counts[j]), inputs);
        }
        b::row(
            seq![
                u::cell(u::link(href, u::text(u::title(g)))),
                u::cell(u::number(g.documents.len())),
                u::cell(u::number(g.coverage.rows.len())),
            ] + cells,
            inputs,
        );
    }
    b::lines(rows, inputs);
    l::f079(inputs);
    l::f080(inputs);
    l::f081(inputs);
    l::f082(inputs);
    l::f083(inputs);
    l::f084(inputs);
    l::f085(inputs);
    l::f086(inputs);
    b::add(b::f(83), u::lines(rows), inputs);
    b::add(b::f(83) + u::lines(rows), b::f(84), inputs);
    let parts = seq![
        b::f(79),
        b::f(80),
        b::f(81),
        b::f(82),
        b::f(83) + u::lines(rows) + b::f(84),
        b::f(85),
        b::f(86),
    ];
    b::lines(parts, inputs);
    l::l077(inputs);
    l::f078(inputs);
    b::frame(u::lit("Guidelines"@), b::f(78), u::lines(parts), inputs);
    assert(u::index_html(c) == u::frame(u::lit("Guidelines"@), b::f(78), u::lines(parts)));
}

} // verus!
