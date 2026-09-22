use crate::{
    k5_sound_build as b, k5_sound_corpus as corpus, k5_sound_escape as h, k5_sound_literals as l,
    k5_sound_model as m, k5_sound_payload as payload, k5_sound_source as source,
};
use ckc_spec::{align, ui as u, v1text as v};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn attribute(prefix: u::Html, value: u::Bytes, suffix: u::Html, inputs: Seq<u::Bytes>)
    requires
        h::fragment(prefix, inputs, 0, 2),
        h::fragment(suffix, inputs, 2, 0),
    ensures
        h::sound(prefix + u::attr(value) + suffix, inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    h::attr(value, inputs, 2);
    h::add(prefix, u::attr(value), inputs, 0, 2, 2);
    h::add(prefix + u::attr(value), suffix, inputs, 0, 2, 0);
}

pub proof fn keyword(s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        m::backed(s, inputs),
    ensures
        h::sound(u::keyword_html(s), inputs),
    decreases s.len(),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::token_end);
    hide(u::stop_words);
    hide(u::lower);
    if s.len() == 0 {
        h::empty(inputs, 0);
    } else if u::token_byte(s[0]) {
        let end = u::token_end(s, 1);
        let e = if 1 <= end <= s.len() {
            end
        } else {
            1
        };
        let word = s.take(e as int);
        m::sub(s, inputs, 0, e as int);
        assert(word =~= s.subrange(0, e as int));
        m::copy(word, inputs);
        h::text(word, inputs);
        m::sub(s, inputs, e as int, s.len() as int);
        assert(s.skip(e as int) =~= s.subrange(e as int, s.len() as int));
        keyword(s.skip(e as int), inputs);
        let head = if u::stop_words().contains(u::lower(word)) {
            b::f(128) + u::text(word) + b::f(24)
        } else {
            u::text(word)
        };
        if u::stop_words().contains(u::lower(word)) {
            l::f128(inputs);
            l::f024(inputs);
            b::add(b::f(128), u::text(word), inputs);
            b::add(b::f(128) + u::text(word), b::f(24), inputs);
        }
        b::add(head, u::keyword_html(s.skip(e as int)), inputs);
    } else {
        m::sub(s, inputs, 0, 1);
        assert(s.subrange(0, 1) =~= seq![s[0]]);
        m::copy(seq![s[0]], inputs);
        h::text(seq![s[0]], inputs);
        m::sub(s, inputs, 1, s.len() as int);
        assert(s.drop_first() =~= s.subrange(1, s.len() as int));
        keyword(s.drop_first(), inputs);
        b::add(u::text(seq![s[0]]), u::keyword_html(s.drop_first()), inputs);
    }
}

pub proof fn slice(s: u::Bytes, start: int, end: int, inputs: Seq<u::Bytes>)
    requires
        m::backed(s, inputs),
    ensures
        m::backed(u::slice_text(s, start, end), inputs),
{
    source::slice(s, start, end);
    m::span(u::slice_text(s, start, end), s, inputs);
}

pub proof fn marked(
    s: u::Bytes,
    spans: Seq<align::OutSpan>,
    keywords: bool,
    cursor: int,
    inputs: Seq<u::Bytes>,
)
    requires
        m::backed(s, inputs),
    ensures
        h::sound(u::marked_html(s, spans, keywords, cursor), inputs),
    decreases spans.len(),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::keyword_html);
    hide(u::slice_text);
    hide(u::chars);
    hide(v::dec_bytes);
    if spans.len() == 0 {
        slice(s, cursor, u::chars(s).len() as int, inputs);
        let tail = u::slice_text(s, cursor, u::chars(s).len() as int);
        if keywords {
            keyword(tail, inputs);
        } else {
            m::copy(tail, inputs);
            h::text(tail, inputs);
        }
    } else {
        let p = spans[0];
        slice(s, cursor, p.start, inputs);
        slice(s, p.start, p.end, inputs);
        let gap = u::slice_text(s, cursor, p.start);
        let part = u::slice_text(s, p.start, p.end);
        let first = if keywords {
            u::keyword_html(gap)
        } else {
            u::text(gap)
        };
        if keywords {
            keyword(gap, inputs);
        } else {
            m::copy(gap, inputs);
            h::text(gap, inputs);
        }
        let open = if p.index < 48 {
            b::f(129) + u::attr(v::dec_bytes(p.index)) + b::f(11)
        } else {
            b::f(130)
        };
        if p.index < 48 {
            l::f129(inputs);
            l::f011(inputs);
            attribute(b::f(129), v::dec_bytes(p.index), b::f(11), inputs);
        } else {
            l::f130(inputs);
        }
        m::copy(part, inputs);
        h::text(part, inputs);
        l::f131(inputs);
        marked(s, spans.drop_first(), keywords, p.end, inputs);
        b::add(first, open, inputs);
        b::add(first + open, u::text(part), inputs);
        b::add(first + open + u::text(part), b::f(131), inputs);
        b::add(
            first + open + u::text(part) + b::f(131),
            u::marked_html(s, spans.drop_first(), keywords, p.end),
            inputs,
        );
    }
}

pub proof fn aligned(g: u::Guideline, d: u::Document, ace: bool, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
        g.documents.contains(d),
    ensures
        h::sound(u::aligned_text(g, d, ace), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::alignment);
    hide(u::payload);
    hide(u::marked_html);
    hide(u::keyword_html);
    corpus::document(g, d, inputs);
    m::own(d.ace, inputs);
    payload::payload(g, d.bundle.docid, inputs);
    let s = if ace {
        d.ace
    } else {
        u::payload(g, d.bundle.docid)
    };
    match u::alignment(g, d) {
        Some(model) => marked(
            s,
            if ace {
                model.ace
            } else {
                model.src
            },
            ace,
            0,
            inputs,
        ),
        None => if ace {
            keyword(s, inputs);
        } else {
            m::copy(s, inputs);
            h::text(s, inputs);
        },
    }
}

pub proof fn roster(g: u::Guideline, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::roster(g), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::names);
    let names = u::names(g);
    let options = names.map_values(|n: u::Bytes| b::f(133) + u::attr(n) + b::f(134));
    l::f132(inputs);
    l::f133(inputs);
    l::f134(inputs);
    l::f135(inputs);
    assert forall|i: int| 0 <= i < options.len() implies h::sound(
        #[trigger] options[i],
        inputs,
    ) by {
        attribute(b::f(133), names[i], b::f(134), inputs);
    }
    b::flatten(options, inputs);
    b::add(b::f(132), options.flatten(), inputs);
    b::add(b::f(132) + options.flatten(), b::f(135), inputs);
}

} // verus!
