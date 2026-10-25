use crate::{
    k5_sound_build as b, k5_sound_corpus as corpus, k5_sound_escape as h, k5_sound_literals as l,
    k5_sound_meanings as t, k5_sound_model as m, k5_sound_source as src,
};
use ckc_spec::{check as ck, replay, ui as u, vocabulary};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn spans(r: u::Superseded, raw: u::Bytes) -> bool {
    r.date.len() == 8 && src::span(r.key, raw) && src::span(r.date, raw) && src::span(
        r.date.take(4),
        raw,
    ) && src::span(r.date.subrange(4, 6), raw) && src::span(r.date.subrange(6, 8), raw)
        && src::span(r.url, raw) && src::span(r.citation, raw)
}

pub open spec fn backed(r: u::Superseded, inputs: Seq<u::Bytes>) -> bool {
    r.date.len() == 8 && m::backed(r.key, inputs) && m::backed(r.date, inputs) && m::backed(
        r.url,
        inputs,
    ) && m::backed(r.citation, inputs)
}

proof fn first_bound(s: u::Bytes, byte: u8, i: nat)
    ensures
        replay::first_byte(s, byte, i) <= s.len(),
    decreases s.len() - i,
{
    if i < s.len() && s[i as int] != byte {
        first_bound(s, byte, i + 1);
    }
}

pub proof fn row_spans(line: u::Bytes, raw: u::Bytes)
    requires
        src::span(line, raw),
    ensures
        u::superseded_row(line) matches Some(r) ==> spans(r, raw),
{
    src::split(line, 9);
    let f = ck::tab_fields(line);
    if f.len() == 3 && (f[0] == u::lit("supersession"@) || f[0] == u::lit("superseded"@))
        && f[2].len() >= 9 {
        src::span_trans(f[1], line, raw);
        src::span_trans(f[2], line, raw);
        src::span_sub(f[2], raw, 0, 8);
        src::span_sub(f[2], raw, 9, f[2].len() as int);
        let date = f[2].take(8);
        let rest = f[2].skip(9);
        assert(date =~= f[2].subrange(0, 8));
        assert(rest =~= f[2].subrange(9, f[2].len() as int));
        src::span_sub(date, raw, 0, 4);
        src::span_sub(date, raw, 4, 6);
        src::span_sub(date, raw, 6, 8);
        assert(date.take(4) =~= date.subrange(0, 4));
        let n = replay::first_byte(rest, 32, 0);
        first_bound(rest, 32, 0);
        src::span_sub(rest, raw, 0, n as int);
        assert(rest.take(n as int) =~= rest.subrange(0, n as int));
        if n < rest.len() {
            src::span_sub(rest, raw, n as int + 1, rest.len() as int);
            assert(rest.skip(n as int + 1) =~= rest.subrange(n as int + 1, rest.len() as int));
        } else {
            src::span_empty(raw);
        }
    }
}

pub proof fn scan_spans(ls: Seq<u::Bytes>, raw: u::Bytes)
    requires
        forall|i: int| 0 <= i < ls.len() ==> src::span(#[trigger] ls[i], raw),
    ensures
        forall|i: int|
            0 <= i < u::superseded_scan(ls).len() ==> spans(
                #[trigger] u::superseded_scan(ls)[i],
                raw,
            ),
    decreases ls.len(),
{
    hide(u::superseded_row);
    if ls.len() > 0 {
        row_spans(ls[0], raw);
        scan_spans(ls.drop_first(), raw);
        let front = match u::superseded_row(ls[0]) {
            Some(r) => seq![r],
            None => Seq::empty(),
        };
        let tail = u::superseded_scan(ls.drop_first());
        assert forall|i: int| 0 <= i < u::superseded_scan(ls).len() implies spans(
            #[trigger] u::superseded_scan(ls)[i],
            raw,
        ) by {
            if i < front.len() {
                assert(u::superseded_scan(ls)[i] == front[0]);
            } else {
                assert(u::superseded_scan(ls)[i] == tail[i - front.len()]);
            }
        }
    }
}

pub proof fn table_spans(raw: u::Bytes)
    ensures
        forall|i: int|
            0 <= i < u::superseded_rows(Some(raw)).len() ==> spans(
                #[trigger] u::superseded_rows(Some(raw))[i],
                raw,
            ),
{
    hide(vocabulary::parse_vocabulary);
    hide(u::superseded_scan);
    if vocabulary::parse_vocabulary(raw).is_ok() {
        crate::k5_superseded::header_bound(raw);
        src::span_self(raw);
        let start = vocabulary::vocabulary_header().len() as int;
        src::span_sub(raw, raw, start, raw.len() as int);
        let body = raw.skip(start);
        assert(body =~= raw.subrange(start, raw.len() as int));
        src::split(body, 10);
        let ls = ck::body_lines(body);
        assert forall|i: int| 0 <= i < ls.len() implies src::span(#[trigger] ls[i], raw) by {
            assert(ls[i] == ck::split_on(body, 10)[i]);
            src::span_trans(ls[i], body, raw);
        }
        scan_spans(ls, raw);
    }
}

pub proof fn rows(g: u::Guideline, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        forall|i: int|
            0 <= i < u::superseded_rows(g.vocabulary).len() ==> backed(
                #[trigger] u::superseded_rows(g.vocabulary)[i],
                inputs,
            ),
{
    corpus::vocabulary(g, inputs);
    if let Some(raw) = g.vocabulary {
        table_spans(raw);
        m::own(raw, inputs);
        let rs = u::superseded_rows(g.vocabulary);
        assert forall|i: int| 0 <= i < rs.len() implies backed(#[trigger] rs[i], inputs) by {
            m::span(rs[i].key, raw, inputs);
            m::span(rs[i].date, raw, inputs);
            m::span(rs[i].url, raw, inputs);
            m::span(rs[i].citation, raw, inputs);
        }
    }
}

pub proof fn iso_date(date: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        date.len() >= 8,
        m::backed(date, inputs),
    ensures
        h::sound(u::iso_date(date), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::f289(inputs);
    m::sub(date, inputs, 0, 4);
    m::sub(date, inputs, 4, 6);
    m::sub(date, inputs, 6, 8);
    assert(date.take(4) =~= date.subrange(0, 4));
    m::copy(date.take(4), inputs);
    m::copy(date.subrange(4, 6), inputs);
    m::copy(date.subrange(6, 8), inputs);
    h::text(date.take(4), inputs);
    h::text(date.subrange(4, 6), inputs);
    h::text(date.subrange(6, 8), inputs);
    b::add(u::text(date.take(4)), b::f(289), inputs);
    b::add(u::text(date.take(4)) + b::f(289), u::text(date.subrange(4, 6)), inputs);
    b::add(u::text(date.take(4)) + b::f(289) + u::text(date.subrange(4, 6)), b::f(289), inputs);
    b::add(
        u::text(date.take(4)) + b::f(289) + u::text(date.subrange(4, 6)) + b::f(289),
        u::text(date.subrange(6, 8)),
        inputs,
    );
}

pub proof fn source_link(r: u::Superseded, inputs: Seq<u::Bytes>)
    requires
        backed(r, inputs),
    ensures
        h::sound(u::source_link(r), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    m::copy(r.citation, inputs);
    h::text(r.citation, inputs);
    b::link(r.url, u::text(r.citation), inputs);
}

pub proof fn notice(g: u::Guideline, id: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        t::all_sound(u::superseded_notice(g, id), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::superseded_rows);
    hide(u::iso_date);
    hide(u::source_link);
    rows(g, inputs);
    let all = u::superseded_rows(g.vocabulary);
    let pred = |r: u::Superseded| r.document && r.key == id;
    let rs = all.filter(pred);
    if rs.len() > 0 {
        all.lemma_filter_contains_rev(pred, rs[0]);
        let i = choose|i: int| 0 <= i < all.len() && all[i] == rs[0];
        assert(backed(rs[0], inputs));
        iso_date(rs[0].date, inputs);
        source_link(rs[0], inputs);
        l::f287(inputs);
        l::f288(inputs);
        l::f062(inputs);
        l::f290(inputs);
        l::f086(inputs);
        b::add(b::f(288), u::iso_date(rs[0].date), inputs);
        b::add(b::f(288) + u::iso_date(rs[0].date), b::f(62), inputs);
        b::add(b::f(288) + u::iso_date(rs[0].date) + b::f(62), u::source_link(rs[0]), inputs);
        b::add(
            b::f(288) + u::iso_date(rs[0].date) + b::f(62) + u::source_link(rs[0]),
            b::f(290),
            inputs,
        );
    }
}

pub proof fn cells(g: u::Guideline, r: u::Superseded, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
        backed(r, inputs),
    ensures
        h::sound(u::superseded_cells(g, r), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::coverage_field);
    hide(u::document_title);
    hide(u::cell);
    hide(u::row);
    hide(u::link);
    hide(u::url_seg);
    hide(u::iso_date);
    hide(u::source_link);
    let passage = if r.document {
        u::coverage_field(g, r.key, 0)
    } else {
        r.key
    };
    if r.document {
        m::coverage_field(g, r.key, 0, inputs);
    }
    m::copy(passage, inputs);
    h::text(passage, inputs);
    let document = if r.document && u::docids(g).contains(r.key) {
        u::link(
            u::document_dir() + u::url_seg(r.key) + u::html_ext(),
            u::text(u::document_title(g, r.key)),
        )
    } else {
        Seq::empty()
    };
    if r.document && u::docids(g).contains(r.key) {
        m::document_title(g, r.key, inputs);
        h::text(u::document_title(g, r.key), inputs);
        b::link(
            u::document_dir() + u::url_seg(r.key) + u::html_ext(),
            u::text(u::document_title(g, r.key)),
            inputs,
        );
    } else {
        h::empty(inputs, 0);
    }
    source_link(r, inputs);
    iso_date(r.date, inputs);
    b::cell(u::text(passage), inputs);
    b::cell(document, inputs);
    b::cell(u::source_link(r), inputs);
    b::cell(u::iso_date(r.date), inputs);
    b::row(
        seq![
            u::cell(u::text(passage)),
            u::cell(document),
            u::cell(u::source_link(r)),
            u::cell(u::iso_date(r.date)),
        ],
        inputs,
    );
}

pub proof fn section(g: u::Guideline, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        t::all_sound(u::superseded_section(g), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::superseded_rows);
    hide(u::superseded_cells);
    hide(u::lines);
    rows(g, inputs);
    let rs = u::superseded_rows(g.vocabulary);
    if rs.len() > 0 {
        let table = rs.map_values(|r: u::Superseded| u::superseded_cells(g, r));
        assert forall|i: int| 0 <= i < table.len() implies h::sound(
            #[trigger] table[i],
            inputs,
        ) by {
            cells(g, rs[i], inputs);
        }
        b::lines(table, inputs);
        l::f080(inputs);
        l::f291(inputs);
        l::f292(inputs);
        l::f102(inputs);
        l::f293(inputs);
        l::f083(inputs);
        l::f084(inputs);
        l::f085(inputs);
        l::f086(inputs);
        b::add(b::f(83), u::lines(table), inputs);
        b::add(b::f(83) + u::lines(table), b::f(84), inputs);
    }
}

} // verus!
