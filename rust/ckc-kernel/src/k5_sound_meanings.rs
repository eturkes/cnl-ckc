// Contract ur: Word meanings + reading words hold declared copy, table spans and ACE spans.
use crate::{
    k5_sound_build as b, k5_sound_corpus as corpus, k5_sound_escape as h, k5_sound_highlight as hl,
    k5_sound_literals as l, k5_sound_model as m, k5_sound_source as src,
};
#[cfg(verus_keep_ghost)]
use ckc_spec::vocabulary::{Vocabulary, parse_vocabulary};
use ckc_spec::{align, check as ck, ui as u};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn all_sound(xs: Seq<u::Html>, inputs: Seq<u::Bytes>) -> bool {
    forall|i: int| 0 <= i < xs.len() ==> h::sound(#[trigger] xs[i], inputs)
}

pub proof fn cat3(a: Seq<u::Html>, t: Seq<u::Html>, c: Seq<u::Html>, inputs: Seq<u::Bytes>)
    requires
        all_sound(a, inputs),
        all_sound(t, inputs),
        all_sound(c, inputs),
    ensures
        all_sound(a + t + c, inputs),
{
    assert forall|i: int| 0 <= i < (a + t + c).len() implies h::sound(
        #[trigger] (a + t + c)[i],
        inputs,
    ) by {
        if i < a.len() {
            assert((a + t + c)[i] == a[i]);
        } else if i < a.len() + t.len() {
            assert((a + t + c)[i] == t[i - a.len()]);
        } else {
            assert((a + t + c)[i] == c[i - a.len() - t.len()]);
        }
    }
}

proof fn first_sub_found(s: u::Bytes, p: u::Bytes, i: nat)
    requires
        p.len() > 0,
        ck::first_sub(s, p, i) + p.len() <= s.len(),
    ensures
        s.subrange(ck::first_sub(s, p, i) as int, (ck::first_sub(s, p, i) + p.len()) as int) == p,
    decreases s.len() + 1 - i,
{
    if i <= s.len() && i + p.len() <= s.len() && s.subrange(i as int, (i + p.len()) as int) != p {
        first_sub_found(s, p, i + 1);
    }
}

pub proof fn pl_word(pl: u::Bytes, w: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        h::sound(u::pl_word(pl, w), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::f243(inputs);
    if w.len() > 0 && ck::first_sub(pl, w, 0) + w.len() <= pl.len() {
        first_sub_found(pl, w, 0);
        assert(src::span_at(w, pl, ck::first_sub(pl, w, 0) as int));
        m::span(w, pl, inputs);
        m::copy(w, inputs);
        h::text(w, inputs);
    }
}

pub proof fn kind(value: u::Bytes, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::kind_html(value), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    l::f243(inputs);
    l::f248(inputs);
    l::f249(inputs);
    l::f250(inputs);
    l::f251(inputs);
    l::f252(inputs);
    l::f253(inputs);
    l::f254(inputs);
}

// --- D3: every reading is declared copy (+ a kind word).
pub proof fn reading_of(kind: int, v: u::Bytes, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::reading_of(kind, v), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::kind_html);
    l::f243(inputs);
    l::f259(inputs);
    l::f260(inputs);
    l::f261(inputs);
    l::f262(inputs);
    l::f263(inputs);
    l::f264(inputs);
    l::f265(inputs);
    l::f266(inputs);
    l::f267(inputs);
    l::f268(inputs);
    l::f269(inputs);
    if kind == 4 {
        self::kind(v, inputs);
        b::add(b::f(267), u::kind_html(v), inputs);
    }
}

pub proof fn rows_sound(t: Vocabulary, inputs: Seq<u::Bytes>)
    ensures
        forall|i: int|
            0 <= i < u::reading_rows(t).len() ==> h::sound(
                (#[trigger] u::reading_rows(t)[i]).1,
                inputs,
            ),
{
    hide(u::reading_of);
    assert forall|i: int| 0 <= i < u::reading_rows(t).len() implies h::sound(
        (#[trigger] u::reading_rows(t)[i]).1,
        inputs,
    ) by {
        let r = u::reading_rows(t)[i];
        let n0 = t.strengths.len() as int;
        let n1 = n0 + t.evidences.len();
        let n2 = n1 + t.preferences.len();
        let n3 = n2 + t.alternatives.len();
        let n4 = n3 + t.qualifiers.len();
        let n5 = n4 + t.scopes.len();
        if i < n0 {
            reading_of(0, t.strengths[i].1, inputs);
        } else if i < n1 {
            reading_of(1, t.evidences[i - n0].1, inputs);
        } else if i < n2 {
            reading_of(2, u::empty(), inputs);
        } else if i < n3 {
            reading_of(3, u::empty(), inputs);
        } else if i < n4 {
            reading_of(4, t.qualifiers[i - n3].1, inputs);
        } else if i < n5 {
            reading_of(5, u::empty(), inputs);
        } else {
            reading_of(6, u::empty(), inputs);
        }
    }
}

pub proof fn first_reading(rs: Seq<(u::Bytes, u::Html)>, w: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        forall|i: int| 0 <= i < rs.len() ==> h::sound((#[trigger] rs[i]).1, inputs),
    ensures
        h::sound(u::first_reading(rs, w), inputs),
    decreases rs.len(),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::f243(inputs);
    if rs.len() > 0 && rs[0].0 != w {
        assert forall|i: int| 0 <= i < rs.drop_first().len() implies h::sound(
            (#[trigger] rs.drop_first()[i]).1,
            inputs,
        ) by {
            assert(rs.drop_first()[i] == rs[i + 1]);
        }
        first_reading(rs.drop_first(), w, inputs);
    }
}

// --- D4: Word meanings.
proof fn table_row(raw: u::Bytes, r: (u::Bytes, u::Html), inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
        h::sound(r.1, inputs),
    ensures
        h::sound(u::word_row(u::pl_word(raw, r.0), r.1), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::pl_word);
    hide(u::cell);
    hide(u::row);
    pl_word(raw, r.0, inputs);
    b::cell(u::pl_word(raw, r.0), inputs);
    b::cell(r.1, inputs);
    b::row(seq![u::cell(u::pl_word(raw, r.0)), u::cell(r.1)], inputs);
}

// Own solver process: the module's shared query context pushed this one past its rlimit
// once the copy registry grew (SEC7: 286 rows).
#[verifier::spinoff_prover]
pub proof fn section(g: u::Guideline, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        all_sound(u::meanings_section(g.vocabulary), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::lines);
    hide(u::pl_word);
    hide(u::reading_rows);
    hide(u::word_row);
    corpus::vocabulary(g, inputs);
    if let Some(raw) = g.vocabulary {
        if let Ok(t) = parse_vocabulary(raw) {
            let rows = u::reading_rows(t);
            if rows.len() > 0 {
                m::own(raw, inputs);
                rows_sound(t, inputs);
                let table = rows.map_values(
                    |r: (u::Bytes, u::Html)| u::word_row(u::pl_word(raw, r.0), r.1),
                );
                assert forall|i: int| 0 <= i < table.len() implies h::sound(
                    #[trigger] table[i],
                    inputs,
                ) by {
                    table_row(raw, rows[i], inputs);
                }
                b::lines(table, inputs);
                l::f080(inputs);
                l::f083(inputs);
                l::f084(inputs);
                l::f085(inputs);
                l::f086(inputs);
                l::f099(inputs);
                l::f064(inputs);
                l::f102(inputs);
                l::f255(inputs);
                l::f256(inputs);
                l::f257(inputs);
                l::f258(inputs);
                b::add(b::f(83), u::lines(table), inputs);
                b::add(b::f(83) + u::lines(table), b::f(84), inputs);
                if u::scope_shown(t) {
                    let noun = u::pl_word(raw, t.scopes[0].0);
                    let verb = u::pl_word(raw, t.exclusions[0]);
                    pl_word(raw, t.scopes[0].0, inputs);
                    pl_word(raw, t.exclusions[0], inputs);
                    l::f276(inputs);
                    b::add(b::f(276), noun, inputs);
                    b::add(b::f(276) + noun, b::f(64), inputs);
                    b::add(b::f(276) + noun + b::f(64), verb, inputs);
                    b::add(b::f(276) + noun + b::f(64) + verb, b::f(258), inputs);
                }
            }
        }
    }
}

// --- D5: the ACE box with reading words.
proof fn piece(s: u::Bytes, spans: Seq<align::OutSpan>, a: int, z: int, inputs: Seq<u::Bytes>)
    requires
        m::backed(s, inputs),
    ensures
        h::sound(u::piece_html(s, spans, a, z), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::marked_html);
    hide(u::slice_text);
    hide(u::clip);
    hl::slice(s, a, z, inputs);
    hl::marked(u::slice_text(s, a, z), u::clip(spans, a, z), true, 0, inputs);
}

proof fn words(
    s: u::Bytes,
    spans: Seq<align::OutSpan>,
    ws: Seq<u::Reading>,
    cursor: int,
    inputs: Seq<u::Bytes>,
)
    requires
        m::backed(s, inputs),
    ensures
        h::sound(u::words_html(s, spans, ws, cursor), inputs),
    decreases ws.len(),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::piece_html);
    hide(u::lit);
    if ws.len() == 0 {
        piece(s, spans, cursor, u::chars(s).len() as int, inputs);
    } else {
        let w = ws[0];
        piece(s, spans, cursor, w.start, inputs);
        piece(s, spans, w.start, w.end, inputs);
        words(s, spans, ws.drop_first(), w.end, inputs);
        l::f270(inputs);
        l::f011(inputs);
        l::f271(inputs);
        let first = u::piece_html(s, spans, cursor, w.start);
        let inner = u::piece_html(s, spans, w.start, w.end);
        let rest = u::words_html(s, spans, ws.drop_first(), w.end);
        h::attr(w.lemma, inputs, 2);
        h::add(first, b::f(270), inputs, 0, 0, 2);
        h::add(first + b::f(270), u::attr(w.lemma), inputs, 0, 2, 2);
        h::add(first + b::f(270) + u::attr(w.lemma), b::f(11), inputs, 0, 2, 0);
        let open = first + b::f(270) + u::attr(w.lemma) + b::f(11);
        b::add(open, inner, inputs);
        b::add(open + inner, b::f(271), inputs);
        b::add(open + inner + b::f(271), rest, inputs);
    }
}

pub proof fn ace(g: u::Guideline, d: u::Document, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
        g.documents.contains(d),
    ensures
        h::sound(u::ace_html(g, d), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::aligned_text);
    hide(u::words_html);
    hide(u::alignment);
    corpus::document(g, d, inputs);
    m::own(d.ace, inputs);
    if u::reading_words(g, d).len() == 0 {
        hl::aligned(g, d, true, inputs);
    } else {
        words(
            d.ace,
            match u::alignment(g, d) {
                Option::Some(model) => model.ace,
                Option::None => Seq::empty(),
            },
            u::reading_words(g, d),
            0,
            inputs,
        );
    }
}

// --- D6: the reserved line.
proof fn span(raw: u::Bytes, t: Vocabulary, w: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(u::reading_span(raw, t, w), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    hide(u::first_reading);
    hide(u::reading_rows);
    l::f273(inputs);
    l::f274(inputs);
    l::f275(inputs);
    l::f065(inputs);
    l::f024(inputs);
    hl::attribute(b::f(273), w, b::f(274), inputs);
    pl_word(raw, w, inputs);
    rows_sound(t, inputs);
    first_reading(u::reading_rows(t), w, inputs);
    let open = b::f(273) + u::attr(w) + b::f(274);
    let word = u::pl_word(raw, w);
    let read = u::first_reading(u::reading_rows(t), w);
    b::add(open, word, inputs);
    b::add(open + word, b::f(275), inputs);
    b::add(open + word + b::f(275), read, inputs);
    b::add(open + word + b::f(275) + read, b::f(65), inputs);
    b::add(open + word + b::f(275) + read + b::f(65), b::f(24), inputs);
}

pub proof fn line(g: u::Guideline, d: u::Document, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
        g.documents.contains(d),
    ensures
        all_sound(u::reading_line(g, d), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::reading_span);
    corpus::vocabulary(g, inputs);
    let ws = u::reading_words(g, d);
    if let Some(raw) = g.vocabulary {
        if let Ok(t) = parse_vocabulary(raw) {
            if ws.len() > 0 {
                m::own(raw, inputs);
                let uniq = ck::dedup_bytes(ws.map_values(|w: u::Reading| w.lemma));
                let spans = uniq.map_values(|w: u::Bytes| u::reading_span(raw, t, w));
                assert forall|i: int| 0 <= i < spans.len() implies h::sound(
                    #[trigger] spans[i],
                    inputs,
                ) by {
                    span(raw, t, uniq[i], inputs);
                }
                b::flatten(spans, inputs);
                l::f272(inputs);
                l::f126(inputs);
                b::add(b::f(272), spans.flatten(), inputs);
                b::add(b::f(272) + spans.flatten(), b::f(126), inputs);
            }
        }
    }
}

} // verus!
