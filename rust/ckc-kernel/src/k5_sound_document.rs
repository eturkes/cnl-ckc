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

pub open spec fn head_parts(g: u::Guideline, d: u::Document) -> Seq<u::Html> {
    let id = d.bundle.docid;
    let k = u::state(g, id);
    let region = u::coverage_field(g, id, 0);
    let pdfs = g.source_names.filter(|n: u::Bytes| ck::ends(n, u::pdf_ext()));
    let pdf = if pdfs.len() == 1 {
        b::f(137) + u::attr(u::url_seg(pdfs[0])) + b::f(138)
    } else {
        Seq::empty()
    };
    let heading = u::text(u::title(g)) + pdf;
    let source = u::unprefix(u::coverage_field(g, id, 1), u::source_dir());
    let prov = (if region.len() > 0 {
        seq![b::f(246) + b::f(247) + u::text(region) + b::f(24)]
    } else {
        Seq::empty()
    }) + (if g.source_names.contains(source) {
        seq![u::link(u::source_2() + u::url_seg(source), b::f(141))]
    } else {
        Seq::empty()
    });
    let provenance = u::hjoin(prov, b::f(36));
    let records_href = u::records_html_2() + (if u::history(g, id).len() > 0 {
        u::copy_2() + u::url_seg(id)
    } else {
        u::empty()
    });
    let shown = match u::alignment(g, d) {
        Some(model) => model.count > 0,
        None => false,
    };
    seq![
        b::f(97) + heading + b::f(98),
        b::f(114) + u::text(u::document_title(g, id)) + b::f(36) + u::chip(k) + b::f(115),
    ] + (if prov.len() > 0 {
        seq![b::f(245) + provenance + b::f(126)]
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
    }) + u::superseded_notice(g, id) + (if shown {
        seq![b::f(150), u::fixed_bytes(u::script_html())]
    } else if u::reading_words(g, d).len() > 0 {
        seq![u::fixed_bytes(u::script_html())]
    } else {
        Seq::empty()
    })
}

// The passage + ACE literal of document_html (its association kept: the page
// proof matches the spec's concatenation tree term by term).
pub open spec fn mid_parts(g: u::Guideline, d: u::Document) -> Seq<u::Html> {
    seq![
        b::f(80),
        b::f(151),
        b::f(152) + u::aligned_text(g, d, false) + b::f(153),
        b::f(86),
        b::f(80),
        b::f(154),
        b::f(152) + u::ace_html(g, d) + b::f(153),
    ]
}

proof fn mid_sound(g: u::Guideline, d: u::Document, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
        g.documents.contains(d),
    ensures
        crate::k5_sound_meanings::all_sound(mid_parts(g, d), inputs),
{
    hide(h::fragment);
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::aligned_text);
    hide(u::ace_html);
    l::f080(inputs);
    l::f086(inputs);
    l::f151(inputs);
    l::f152(inputs);
    l::f153(inputs);
    l::f154(inputs);
    hl::aligned(g, d, false, inputs);
    crate::k5_sound_meanings::ace(g, d, inputs);
    three(b::f(152), u::aligned_text(g, d, false), b::f(153), inputs);
    three(b::f(152), u::ace_html(g, d), b::f(153), inputs);
}

pub open spec fn tail_parts(
    g: u::Guideline,
    d: u::Document,
    prev: u::Bytes,
    next: u::Bytes,
    token: u::Bytes,
    commit: u::Bytes,
) -> Seq<u::Html> {
    let nav = (if prev.len() > 0 {
        seq![u::link(u::url_seg(prev) + u::html_ext(), b::f(144))]
    } else {
        Seq::empty()
    }) + seq![b::f(145)] + (if next.len() > 0 {
        seq![u::link(u::url_seg(next) + u::html_ext(), b::f(146))]
    } else {
        Seq::empty()
    });
    let name = u::latest_name(u::records(g), u::empty(), u::empty());
    let count = u::splitline_count(u::chars(d.pl), false);
    seq![
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
        b::f(286),
        b::f(164),
        b::f(165) + u::attr(name) + b::f(166),
        u::roster(g),
        b::f(167),
        b::f(168),
        b::f(169) + u::attr(d.bundle.review) + b::f(11),
        b::f(170) + u::attr(g.ledger_digest) + b::f(11),
        b::f(239) + u::attr(commit) + b::f(11),
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
        b::f(179) + u::hjoin(nav, b::f(36)) + b::f(52),
    ]
}

// q13: the default rlimit ran out once the copy table reached 301 entries; the
// profile shows no matching loop (8.6K instantiations), 20 passes in 2 min.
#[verifier::rlimit(20)]
proof fn head_sound(g: u::Guideline, d: u::Document, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
        g.documents.contains(d),
    ensures
        crate::k5_sound_meanings::all_sound(head_parts(g, d), inputs),
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
    hide(u::ace_html);
    hide(u::reading_line);
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
    hide(u::superseded_notice);
    let id = d.bundle.docid;
    let k = u::state(g, id);
    let region = u::coverage_field(g, id, 0);
    let pdfs = g.source_names.filter(|n: u::Bytes| ck::ends(n, u::pdf_ext()));
    let pdf = if pdfs.len() == 1 {
        b::f(137) + u::attr(u::url_seg(pdfs[0])) + b::f(138)
    } else {
        Seq::empty()
    };
    let heading = u::text(u::title(g)) + pdf;
    let source = u::unprefix(u::coverage_field(g, id, 1), u::source_dir());
    let prov = (if region.len() > 0 {
        seq![b::f(246) + b::f(247) + u::text(region) + b::f(24)]
    } else {
        Seq::empty()
    }) + (if g.source_names.contains(source) {
        seq![u::link(u::source_2() + u::url_seg(source), b::f(141))]
    } else {
        Seq::empty()
    });
    let provenance = u::hjoin(prov, b::f(36));
    let records_href = u::records_html_2() + (if u::history(g, id).len() > 0 {
        u::copy_2() + u::url_seg(id)
    } else {
        u::empty()
    });
    let shown = match u::alignment(g, d) {
        Some(model) => model.count > 0,
        None => false,
    };
    l::f137(inputs);
    l::f138(inputs);
    l::f141(inputs);
    l::f245(inputs);
    l::f246(inputs);
    l::f247(inputs);
    l::f024(inputs);
    l::f147(inputs);
    l::f150(inputs);
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
    corpus::document(g, d, inputs);
    m::own(d.bundle.docid, inputs);
    corpus::title(g, inputs);
    h::text(u::title(g), inputs);
    m::document_title(g, id, inputs);
    h::text(u::document_title(g, id), inputs);
    m::coverage_field(g, id, 0, inputs);
    m::copy(region, inputs);
    h::text(region, inputs);
    b::add(b::f(246), b::f(247), inputs);
    three(b::f(246) + b::f(247), u::text(region), b::f(24), inputs);
    h::empty(inputs, 0);
    if pdfs.len() == 1 {
        hl::attribute(b::f(137), u::url_seg(pdfs[0]), b::f(138), inputs);
    }
    b::add(u::text(u::title(g)), pdf, inputs);
    b::link(u::source_2() + u::url_seg(source), b::f(141), inputs);
    b::hjoin(prov, b::f(36), inputs);
    b::link(records_href, b::f(147), inputs);
    summary::tally_text(u::tally(g, id), inputs);
    h::text(u::tally_text(u::tally(g, id)), inputs);
    b::chip(k, inputs);
    chrome::script_html(inputs);
    crate::k5_sound_superseded::notice(g, id, inputs);
    three(b::f(97), heading, b::f(98), inputs);
    five(b::f(114), u::text(u::document_title(g, id)), b::f(36), u::chip(k), b::f(115), inputs);
    three(b::f(245), provenance, b::f(126), inputs);
    five(
        b::f(99),
        u::text(u::tally_text(u::tally(g, id))),
        b::f(36),
        u::link(records_href, b::f(147)),
        b::f(126),
        inputs,
    );
}

proof fn tail_sound(
    g: u::Guideline,
    d: u::Document,
    prev: u::Bytes,
    next: u::Bytes,
    token: u::Bytes,
    commit: u::Bytes,
    inputs: Seq<u::Bytes>,
)
    requires
        corpus::sources(g, inputs),
        g.documents.contains(d),
    ensures
        crate::k5_sound_meanings::all_sound(tail_parts(g, d, prev, next, token, commit), inputs),
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
    hide(u::ace_html);
    hide(u::reading_line);
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
    let nav = (if prev.len() > 0 {
        seq![u::link(u::url_seg(prev) + u::html_ext(), b::f(144))]
    } else {
        Seq::empty()
    }) + seq![b::f(145)] + (if next.len() > 0 {
        seq![u::link(u::url_seg(next) + u::html_ext(), b::f(146))]
    } else {
        Seq::empty()
    });
    let name = u::latest_name(u::records(g), u::empty(), u::empty());
    let count = u::splitline_count(u::chars(d.pl), false);
    l::f036(inputs);
    l::f144(inputs);
    l::f145(inputs);
    l::f146(inputs);
    l::f086(inputs);
    l::f080(inputs);
    l::f153(inputs);
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
    l::f286(inputs);
    l::f165(inputs);
    l::f166(inputs);
    l::f167(inputs);
    l::f168(inputs);
    l::f169(inputs);
    l::f170(inputs);
    l::f171(inputs);
    l::f239(inputs);
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
    corpus::document(g, d, inputs);
    m::own(d.pl, inputs);
    m::copy(d.pl, inputs);
    h::text(d.pl, inputs);
    hl::roster(g, inputs);
    hl::attribute(b::f(165), name, b::f(166), inputs);
    hl::attribute(b::f(169), d.bundle.review, b::f(11), inputs);
    hl::attribute(b::f(170), g.ledger_digest, b::f(11), inputs);
    hl::attribute(b::f(239), commit, b::f(11), inputs);
    hl::attribute(b::f(171), token, b::f(11), inputs);
    h::number(count, inputs, 0);
    three(b::f(175), u::number(count), b::f(176), inputs);
    three(b::f(177), u::text(d.pl), b::f(153), inputs);
    b::link(u::url_seg(prev) + u::html_ext(), b::f(144), inputs);
    b::link(u::url_seg(next) + u::html_ext(), b::f(146), inputs);
    b::hjoin(nav, b::f(36), inputs);
    three(b::f(179), u::hjoin(nav, b::f(36)), b::f(52), inputs);
}

// Each mirror is matched before the final page's sequence algebra.
#[verifier::spinoff_prover]
proof fn head_equiv(g: u::Guideline, d: u::Document)
    ensures
        head_parts(g, d) == crate::k5_document::leading_spec(
            g,
            d,
            match u::alignment(g, d) {
                Some(model) => model.count > 0,
                None => false,
            },
        ),
{
    hide(u::lit);
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::title);
    hide(u::document_title);
    hide(u::coverage_field);
    hide(u::state);
    hide(u::tally);
    hide(u::tally_text);
    hide(u::history);
    hide(u::alignment);
    hide(u::superseded_notice);
    hide(u::script_html);
    hide(u::unprefix);
    hide(ck::ends);
    hide(u::url_seg);
    hide(u::link);
    hide(u::hjoin);
    hide(u::chip);
    let inputs = Seq::<u::Bytes>::empty();
    l::l137(inputs);
    l::l138(inputs);
    l::l141(inputs);
    l::l245(inputs);
    l::l246(inputs);
    l::l247(inputs);
    l::l024(inputs);
    l::l147(inputs);
    l::l150(inputs);
    l::l097(inputs);
    l::l098(inputs);
    l::l114(inputs);
    l::l115(inputs);
    l::l036(inputs);
    l::l099(inputs);
    l::l126(inputs);
    l::l148(inputs);
    l::l149(inputs);
    l::l086(inputs);
    let shown = match u::alignment(g, d) {
        Some(model) => model.count > 0,
        None => false,
    };
    assert(head_parts(g, d) =~= crate::k5_document::leading_spec(g, d, shown));
}

proof fn mid_equiv(g: u::Guideline, d: u::Document)
    ensures
        mid_parts(g, d) + u::reading_line(g, d) + seq![b::f(86)]
            == crate::k5_document::passages_spec(g, d),
{
    hide(u::lit);
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::aligned_text);
    hide(u::ace_html);
    hide(u::reading_line);
    let inputs = Seq::<u::Bytes>::empty();
    l::l080(inputs);
    l::l086(inputs);
    l::l151(inputs);
    l::l152(inputs);
    l::l153(inputs);
    l::l154(inputs);
    assert(mid_parts(g, d) + u::reading_line(g, d) + seq![b::f(86)]
        =~= crate::k5_document::passages_spec(g, d));
}

#[verifier::spinoff_prover]
proof fn tail_equiv(
    g: u::Guideline,
    d: u::Document,
    prev: u::Bytes,
    next: u::Bytes,
    token: u::Bytes,
    commit: u::Bytes,
)
    ensures
        tail_parts(g, d, prev, next, token, commit) == seq![b::f(86)]
            + crate::k5_document::form_spec(g, d, token, commit)
            + crate::k5_document::compiled_spec(d, prev, next),
{
    hide(u::lit);
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::latest_name);
    hide(u::roster);
    hide(u::url_seg);
    hide(u::splitline_count);
    hide(u::chars);
    hide(u::link);
    hide(u::hjoin);
    hide(u::number);
    let inputs = Seq::<u::Bytes>::empty();
    l::l036(inputs);
    l::l144(inputs);
    l::l145(inputs);
    l::l146(inputs);
    l::l086(inputs);
    l::l080(inputs);
    l::l153(inputs);
    l::l155(inputs);
    l::l156(inputs);
    l::l157(inputs);
    l::l158(inputs);
    l::l159(inputs);
    l::l160(inputs);
    l::l161(inputs);
    l::l162(inputs);
    l::l163(inputs);
    l::l164(inputs);
    l::l286(inputs);
    l::l165(inputs);
    l::l166(inputs);
    l::l167(inputs);
    l::l168(inputs);
    l::l169(inputs);
    l::l170(inputs);
    l::l171(inputs);
    l::l239(inputs);
    l::l011(inputs);
    l::l172(inputs);
    l::l173(inputs);
    l::l174(inputs);
    l::l175(inputs);
    l::l176(inputs);
    l::l177(inputs);
    l::l178(inputs);
    l::l179(inputs);
    l::l052(inputs);
    assert(tail_parts(g, d, prev, next, token, commit) =~= seq![b::f(86)]
        + crate::k5_document::form_spec(g, d, token, commit) + crate::k5_document::compiled_spec(
        d,
        prev,
        next,
    ));
}

proof fn decomposition(
    g: u::Guideline,
    d: u::Document,
    prev: u::Bytes,
    next: u::Bytes,
    token: u::Bytes,
    commit: u::Bytes,
)
    ensures
        u::document_html(g, d, prev, next, token, commit) == u::frame(
            u::document_title(g, d.bundle.docid),
            b::f(180) + u::text(u::title(g)) + b::f(181) + u::text(
                u::coverage_field(g, d.bundle.docid, 0),
            ),
            u::lines(
                head_parts(g, d) + mid_parts(g, d) + u::reading_line(g, d) + tail_parts(
                    g,
                    d,
                    prev,
                    next,
                    token,
                    commit,
                ),
            ),
        ),
{
    hide(u::lit);
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::document_html);
    hide(u::title);
    hide(u::document_title);
    hide(u::coverage_field);
    hide(u::alignment);
    hide(u::reading_line);
    hide(u::frame);
    hide(u::lines);
    hide(crate::k5_document::body_spec);
    hide(crate::k5_document::leading_spec);
    hide(crate::k5_document::passages_spec);
    hide(crate::k5_document::form_spec);
    hide(crate::k5_document::compiled_spec);
    hide(head_parts);
    hide(mid_parts);
    hide(tail_parts);
    let shown = match u::alignment(g, d) {
        Some(model) => model.count > 0,
        None => false,
    };
    head_equiv(g, d);
    mid_equiv(g, d);
    tail_equiv(g, d, prev, next, token, commit);
    crate::k5_document::body_groups(g, d, prev, next, token, commit, shown);
    crate::k5_document::decomposition(g, d, prev, next, token, commit);
    let parts = head_parts(g, d) + mid_parts(g, d) + u::reading_line(g, d) + tail_parts(
        g,
        d,
        prev,
        next,
        token,
        commit,
    );
    assert(parts =~= crate::k5_document::leading_spec(g, d, shown)
        + crate::k5_document::passages_spec(g, d) + crate::k5_document::form_spec(
        g,
        d,
        token,
        commit,
    ) + crate::k5_document::compiled_spec(d, prev, next));
    let inputs = Seq::<u::Bytes>::empty();
    l::l180(inputs);
    l::l181(inputs);
}

pub proof fn page(
    g: u::Guideline,
    d: u::Document,
    prev: u::Bytes,
    next: u::Bytes,
    token: u::Bytes,
    commit: u::Bytes,
    inputs: Seq<u::Bytes>,
)
    requires
        corpus::sources(g, inputs),
        g.documents.contains(d),
    ensures
        h::sound(u::document_html(g, d, prev, next, token, commit), inputs),
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
    hide(u::ace_html);
    hide(u::reading_line);
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
    hide(u::superseded_notice);
    hide(u::document_html);
    hide(head_parts);
    hide(mid_parts);
    hide(tail_parts);
    let id = d.bundle.docid;
    let k = u::state(g, id);
    let region = u::coverage_field(g, id, 0);
    let name = u::latest_name(u::records(g), u::empty(), u::empty());
    corpus::document(g, d, inputs);
    m::own(id, inputs);
    l::f180(inputs);
    l::f181(inputs);
    head_sound(g, d, inputs);
    tail_sound(g, d, prev, next, token, commit, inputs);
    mid_sound(g, d, inputs);
    crate::k5_sound_meanings::line(g, d, inputs);
    let head = head_parts(g, d);
    let mid = mid_parts(g, d);
    let line = u::reading_line(g, d);
    let tail = tail_parts(g, d, prev, next, token, commit);
    crate::k5_sound_meanings::cat3(head, mid, line, inputs);
    crate::k5_sound_meanings::cat3(head + mid + line, tail, Seq::empty(), inputs);
    let parts = head + mid + line + tail;
    assert(parts =~= head + mid + line + tail + Seq::<u::Html>::empty());
    b::lines(parts, inputs);
    let crumbs = b::f(180) + u::text(u::title(g)) + b::f(181) + u::text(region);
    assert(h::sound(crumbs, inputs)) by {
        corpus::title(g, inputs);
        h::text(u::title(g), inputs);
        m::coverage_field(g, id, 0, inputs);
        m::copy(region, inputs);
        h::text(region, inputs);
        four(b::f(180), u::text(u::title(g)), b::f(181), u::text(region), inputs);
    }
    m::document_title(g, id, inputs);
    b::frame(u::document_title(g, id), crumbs, u::lines(parts), inputs);
    decomposition(g, d, prev, next, token, commit);
    assert(u::document_html(g, d, prev, next, token, commit) == u::frame(
        u::document_title(g, id),
        crumbs,
        u::lines(parts),
    ));
}

} // verus!
