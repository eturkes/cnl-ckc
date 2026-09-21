use crate::{
    k5_sound_build as b, k5_sound_chrome as chrome, k5_sound_corpus as corpus,
    k5_sound_escape as h, k5_sound_highlight as hl, k5_sound_literals as l, k5_sound_model as m,
    k5_sound_summary as summary,
};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn three(a: u::Html, c: u::Html, d: u::Html, inputs: Seq<u::Bytes>)
    requires
        h::sound(a, inputs),
        h::sound(c, inputs),
        h::sound(d, inputs),
    ensures
        h::sound(a + c + d, inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    b::add(a, c, inputs);
    b::add(a + c, d, inputs);
}

pub proof fn four(a: u::Html, c: u::Html, d: u::Html, e: u::Html, inputs: Seq<u::Bytes>)
    requires
        h::sound(a, inputs),
        h::sound(c, inputs),
        h::sound(d, inputs),
        h::sound(e, inputs),
    ensures
        h::sound(a + c + d + e, inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    three(a, c, d, inputs);
    b::add(a + c + d, e, inputs);
}

pub proof fn five(a: u::Html, c: u::Html, d: u::Html, e: u::Html, f: u::Html, inputs: Seq<u::Bytes>)
    requires
        h::sound(a, inputs),
        h::sound(c, inputs),
        h::sound(d, inputs),
        h::sound(e, inputs),
        h::sound(f, inputs),
    ensures
        h::sound(a + c + d + e + f, inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    four(a, c, d, e, inputs);
    b::add(a + c + d + e, f, inputs);
}

pub proof fn page(
    g: u::Guideline,
    d: u::Document,
    prev: u::Bytes,
    next: u::Bytes,
    token: u::Bytes,
    inputs: Seq<u::Bytes>,
)
    requires
        corpus::sources(g, inputs),
        g.documents.contains(d),
    ensures
        h::sound(u::document_html(g, d, prev, next, token), inputs),
{
    hide(h::fragment);
    hide(u::lit);
    hide(u::unprefix);
    hide(ck::ends);
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::title);
    hide(u::document_title);
    hide(u::coverage_field);
    hide(u::state);
    hide(u::tally);
    hide(u::tally_text);
    hide(u::history);
    hide(u::records);
    hide(u::latest_name);
    hide(u::alignment);
    hide(u::aligned_text);
    hide(u::roster);
    hide(u::url_seg);
    hide(u::splitline_count);
    hide(u::chars);
    hide(u::link);
    hide(u::chip);
    hide(u::number);
    hide(u::hjoin);
    hide(u::lines);
    hide(u::frame);
    hide(u::css_text);
    hide(u::script_html);
    let id = d.bundle.docid;
    let k = u::state(g, id);
    let region = u::coverage_field(g, id, 0);
    corpus::document(g, d, inputs);
    m::own(id, inputs);
    m::own(d.pl, inputs);
    m::copy(d.pl, inputs);
    h::text(d.pl, inputs);
    corpus::title(g, inputs);
    h::text(u::title(g), inputs);
    m::document_title(g, id, inputs);
    h::text(u::document_title(g, id), inputs);
    m::coverage_field(g, id, 0, inputs);
    m::copy(region, inputs);
    h::text(region, inputs);
    let pdfs = g.source_names.filter(|n: u::Bytes| ck::ends(n, u::lit(".pdf"@)));
    l::f137(inputs);
    l::f138(inputs);
    h::empty(inputs, 0);
    let pdf = if pdfs.len() == 1 {
        b::f(137) + u::attr(u::url_seg(pdfs[0])) + b::f(138)
    } else {
        Seq::empty()
    };
    if pdfs.len() == 1 {
        hl::attribute(b::f(137), u::url_seg(pdfs[0]), b::f(138), inputs);
    }
    b::add(u::text(u::title(g)), pdf, inputs);
    let heading = u::text(u::title(g)) + pdf;
    let source = u::unprefix(u::coverage_field(g, id, 1), u::lit("source/"@));
    l::f141(inputs);
    b::link(u::lit("../source/"@) + u::url_seg(source), b::f(141), inputs);
    let prov = (if region.len() > 0 {
        seq![u::text(region)]
    } else {
        Seq::empty()
    }) + (if g.source_names.contains(source) {
        seq![u::link(u::lit("../source/"@) + u::url_seg(source), b::f(141))]
    } else {
        Seq::empty()
    });
    l::f029(inputs);
    b::hjoin(prov, b::f(29), inputs);
    let provenance = u::hjoin(prov, b::f(29));
    let records_href = u::lit("../records.html"@) + (if u::history(g, id).len() > 0 {
        u::lit("#"@) + u::url_seg(id)
    } else {
        u::empty()
    });
    l::f147(inputs);
    b::link(records_href, b::f(147), inputs);
    summary::tally_text(u::tally(g, id), inputs);
    h::text(u::tally_text(u::tally(g, id)), inputs);
    b::chip(k, inputs);
    let shown = match u::alignment(g, d) {
        Some(model) => model.count > 0,
        None => false,
    };
    l::f150(inputs);
    chrome::script_html(inputs);
    l::f144(inputs);
    l::f145(inputs);
    l::f146(inputs);
    b::link(u::url_seg(prev) + u::lit(".html"@), b::f(144), inputs);
    b::link(u::url_seg(next) + u::lit(".html"@), b::f(146), inputs);
    let nav = (if prev.len() > 0 {
        seq![u::link(u::url_seg(prev) + u::lit(".html"@), b::f(144))]
    } else {
        Seq::empty()
    }) + seq![b::f(145)] + (if next.len() > 0 {
        seq![u::link(u::url_seg(next) + u::lit(".html"@), b::f(146))]
    } else {
        Seq::empty()
    });
    b::hjoin(nav, b::f(29), inputs);
    hl::aligned(g, d, false, inputs);
    hl::aligned(g, d, true, inputs);
    hl::roster(g, inputs);
    l::f097(inputs);
    l::f098(inputs);
    l::f114(inputs);
    l::f115(inputs);
    l::f036(inputs);
    l::f099(inputs);
    l::f126(inputs);
    l::f148(inputs);
    l::f149(inputs);
    l::f086(inputs);
    l::f080(inputs);
    l::f151(inputs);
    l::f152(inputs);
    l::f153(inputs);
    l::f154(inputs);
    l::f155(inputs);
    l::f156(inputs);
    l::f157(inputs);
    l::f158(inputs);
    l::f159(inputs);
    l::f160(inputs);
    l::f161(inputs);
    l::f162(inputs);
    l::f163(inputs);
    l::f164(inputs);
    l::f165(inputs);
    l::f166(inputs);
    l::f167(inputs);
    l::f168(inputs);
    l::f169(inputs);
    l::f170(inputs);
    l::f171(inputs);
    l::f011(inputs);
    l::f172(inputs);
    l::f173(inputs);
    l::f174(inputs);
    l::f175(inputs);
    l::f176(inputs);
    l::f177(inputs);
    l::f178(inputs);
    l::f179(inputs);
    l::f052(inputs);
    l::f180(inputs);
    l::f181(inputs);
    three(b::f(97), heading, b::f(98), inputs);
    five(b::f(114), u::text(u::document_title(g, id)), b::f(36), u::chip(k), b::f(115), inputs);
    three(b::f(99), provenance, b::f(126), inputs);
    five(
        b::f(99),
        u::text(u::tally_text(u::tally(g, id))),
        b::f(36),
        u::link(records_href, b::f(147)),
        b::f(126),
        inputs,
    );
    three(b::f(152), u::aligned_text(g, d, false), b::f(153), inputs);
    three(b::f(152), u::aligned_text(g, d, true), b::f(153), inputs);
    let name = u::latest_name(u::records(g), u::empty(), u::empty());
    hl::attribute(b::f(165), name, b::f(166), inputs);
    hl::attribute(b::f(169), d.bundle.review, b::f(11), inputs);
    hl::attribute(b::f(170), g.ledger_digest, b::f(11), inputs);
    hl::attribute(b::f(171), token, b::f(11), inputs);
    let count = u::splitline_count(u::chars(d.pl), false);
    h::number(count, inputs, 0);
    three(b::f(175), u::number(count), b::f(176), inputs);
    three(b::f(177), u::text(d.pl), b::f(153), inputs);
    three(b::f(179), u::hjoin(nav, b::f(29)), b::f(52), inputs);
    let parts = seq![
        b::f(97) + heading + b::f(98),
        b::f(114) + u::text(u::document_title(g, id)) + b::f(36) + u::chip(k) + b::f(115),
    ] + (if prov.len() > 0 {
        seq![b::f(99) + provenance + b::f(126)]
    } else {
        Seq::empty()
    }) + seq![
        b::f(99) + u::text(u::tally_text(u::tally(g, id))) + b::f(36) + u::link(
            records_href,
            b::f(147),
        ) + b::f(126),
    ] + (if k == 3 {
        seq![b::f(148), b::f(149), b::f(86)]
    } else {
        Seq::empty()
    }) + (if shown {
        seq![b::f(150), u::fixed_bytes(u::script_html())]
    } else {
        Seq::empty()
    }) + seq![
        b::f(80),
        b::f(151),
        b::f(152) + u::aligned_text(g, d, false) + b::f(153),
        b::f(86),
        b::f(80),
        b::f(154),
        b::f(152) + u::aligned_text(g, d, true) + b::f(153),
        b::f(86),
        b::f(155),
        b::f(156),
        b::f(157),
        b::f(158),
        b::f(159),
        b::f(160),
        b::f(161),
        b::f(162),
        b::f(163),
        b::f(164),
        b::f(165) + u::attr(name) + b::f(166),
        u::roster(g),
        b::f(167),
        b::f(168),
        b::f(169) + u::attr(d.bundle.review) + b::f(11),
        b::f(170) + u::attr(g.ledger_digest) + b::f(11),
        b::f(171) + u::attr(token) + b::f(11),
        b::f(172),
        b::f(173),
        b::f(86),
        b::f(80),
        b::f(174),
        b::f(175) + u::number(count) + b::f(176),
        b::f(177) + u::text(d.pl) + b::f(153),
        b::f(178),
        b::f(86),
        b::f(179) + u::hjoin(nav, b::f(29)) + b::f(52),
    ];
    b::lines(parts, inputs);
    four(b::f(180), u::text(u::title(g)), b::f(181), u::text(region), inputs);
    let crumbs = b::f(180) + u::text(u::title(g)) + b::f(181) + u::text(region);
    b::frame(u::document_title(g, id), crumbs, u::lines(parts), inputs);
    assert(u::document_html(g, d, prev, next, token) == u::frame(
        u::document_title(g, id),
        crumbs,
        u::lines(parts),
    ));
}

} // verus!
