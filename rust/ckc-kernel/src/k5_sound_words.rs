// K5 render soundness of the q14 time-word section: every visible slot is a
// registry literal or a span of the guideline's temporal.tsv bytes.
use crate::{
    k5_sound_build as b, k5_sound_corpus as corpus, k5_sound_escape as h, k5_sound_literals as l,
    k5_sound_model as m, k5_sound_timing as t, k5_words as w,
};
use ckc_spec::temporal::*;
use ckc_spec::ui as u;
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

proof fn unit(raw: u::Bytes, p: (u::Bytes, u::Bytes), inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(w::unit_row(raw, p), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    hide(u::cell);
    hide(u::row);
    t::pl_word(raw, p.0, inputs);
    l::f249(inputs);
    l::f262(inputs);
    l::f264(inputs);
    l::f266(inputs);
    l::f268(inputs);
    l::f270(inputs);
    l::f272(inputs);
    l::f274(inputs);
    l::f279(inputs);
    let word = match u::unit_html(p.1, true) {
        Option::Some(x) => x,
        Option::None => u::fixed_bytes(u::not_stated()),
    };
    assert(h::sound(word, inputs));
    b::add(b::f(279), word, inputs);
    b::cell(u::pl_word(raw, p.0), inputs);
    b::cell(b::f(279) + word, inputs);
    b::row(seq![u::cell(u::pl_word(raw, p.0)), u::cell(b::f(279) + word)], inputs);
}

proof fn relation(raw: u::Bytes, v: nat, p: (u::Bytes, u::Bytes), inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(w::relation_row(raw, v, p), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    hide(u::cell);
    hide(u::row);
    t::pl_word(raw, p.0, inputs);
    l::f249(inputs);
    l::f280(inputs);
    l::f281(inputs);
    l::f282(inputs);
    l::f283(inputs);
    l::f286(inputs);
    l::f287(inputs);
    assert(h::sound(u::role_html(p.1), inputs));
    assert(h::sound(u::role_html_v(p.1, v), inputs));
    b::cell(u::pl_word(raw, p.0), inputs);
    b::cell(u::role_html_v(p.1, v), inputs);
    b::row(seq![u::cell(u::pl_word(raw, p.0)), u::cell(u::role_html_v(p.1, v))], inputs);
}

proof fn framed(raw: u::Bytes, a: u::Bytes, f: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(u::framed_word(raw, a, f), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    t::pl_word(raw, a, inputs);
    t::pl_word(raw, f, inputs);
    l::f033(inputs);
    l::f034(inputs);
    l::f285(inputs);
    let a1 = u::pl_word(raw, a) + b::f(33);
    let a2 = a1 + b::f(285);
    let a3 = a2 + u::pl_word(raw, f);
    b::add(u::pl_word(raw, a), b::f(33), inputs);
    b::add(a1, b::f(285), inputs);
    b::add(a2, u::pl_word(raw, f), inputs);
    b::add(a3, b::f(34), inputs);
}

proof fn spacing(raw: u::Bytes, p: (u::Bytes, u::Bytes), inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(w::spacing_row(raw, p), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::framed_word);
    hide(u::cell);
    hide(u::row);
    framed(raw, p.0, p.1, inputs);
    l::f284(inputs);
    b::cell(u::framed_word(raw, p.0, p.1), inputs);
    b::cell(b::f(284), inputs);
    b::row(seq![u::cell(u::framed_word(raw, p.0, p.1)), u::cell(b::f(284))], inputs);
}

proof fn window(raw: u::Bytes, p: (u::Bytes, u::Bytes), inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(w::window_row(raw, p), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::framed_word);
    hide(u::cell);
    hide(u::row);
    framed(raw, p.0, p.1, inputs);
    l::f288(inputs);
    b::cell(u::framed_word(raw, p.0, p.1), inputs);
    b::cell(b::f(288), inputs);
    b::row(seq![u::cell(u::framed_word(raw, p.0, p.1)), u::cell(b::f(288))], inputs);
}

proof fn frequency(raw: u::Bytes, f: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(w::frequency_row(raw, f), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    hide(u::cell);
    hide(u::row);
    t::pl_word(raw, f, inputs);
    l::f289(inputs);
    b::cell(u::pl_word(raw, f), inputs);
    b::cell(b::f(289), inputs);
    b::row(seq![u::cell(u::pl_word(raw, f)), u::cell(b::f(289))], inputs);
}

proof fn lemma_word(raw: u::Bytes, a: u::Bytes, k: int, inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
        k == 299 || k == 300,
    ensures
        k == 299 ==> h::sound(w::approximation_row(raw, a), inputs),
        k == 300 ==> h::sound(w::range_row(raw, a), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    hide(u::cell);
    hide(u::row);
    t::pl_word(raw, a, inputs);
    l::f299(inputs);
    l::f300(inputs);
    b::cell(u::pl_word(raw, a), inputs);
    b::cell(b::f(k), inputs);
    b::row(seq![u::cell(u::pl_word(raw, a)), u::cell(b::f(k))], inputs);
}

proof fn rows(raw: u::Bytes, tm: Temporal, inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        t::all_sound(w::rows_of(raw, tm), inputs),
{
    hide(w::unit_row);
    hide(w::relation_row);
    hide(w::spacing_row);
    hide(w::window_row);
    hide(w::frequency_row);
    hide(w::approximation_row);
    hide(w::range_row);
    let us = tm.units.map_values(|p: (u::Bytes, u::Bytes)| w::unit_row(raw, p));
    let rs = tm.relations.map_values(|p: (u::Bytes, u::Bytes)| w::relation_row(raw, tm.version, p));
    let ss = tm.spacings.map_values(|p: (u::Bytes, u::Bytes)| w::spacing_row(raw, p));
    let ws = tm.windows.map_values(|p: (u::Bytes, u::Bytes)| w::window_row(raw, p));
    let fs = tm.frequencies.map_values(|f: u::Bytes| w::frequency_row(raw, f));
    let xs = tm.approximations.map_values(|a: u::Bytes| w::approximation_row(raw, a));
    let gs = tm.ranges.map_values(|r: u::Bytes| w::range_row(raw, r));
    assert forall|i: int| 0 <= i < us.len() implies h::sound(#[trigger] us[i], inputs) by {
        unit(raw, tm.units[i], inputs);
    }
    assert forall|i: int| 0 <= i < rs.len() implies h::sound(#[trigger] rs[i], inputs) by {
        relation(raw, tm.version, tm.relations[i], inputs);
    }
    assert forall|i: int| 0 <= i < ss.len() implies h::sound(#[trigger] ss[i], inputs) by {
        spacing(raw, tm.spacings[i], inputs);
    }
    assert forall|i: int| 0 <= i < ws.len() implies h::sound(#[trigger] ws[i], inputs) by {
        window(raw, tm.windows[i], inputs);
    }
    assert forall|i: int| 0 <= i < fs.len() implies h::sound(#[trigger] fs[i], inputs) by {
        frequency(raw, tm.frequencies[i], inputs);
    }
    assert forall|i: int| 0 <= i < xs.len() implies h::sound(#[trigger] xs[i], inputs) by {
        lemma_word(raw, tm.approximations[i], 299, inputs);
    }
    assert forall|i: int| 0 <= i < gs.len() implies h::sound(#[trigger] gs[i], inputs) by {
        lemma_word(raw, tm.ranges[i], 300, inputs);
    }
    t::cat3(us, rs, ss, inputs);
    t::cat3(us + rs + ss, ws, fs, inputs);
    t::cat3(us + rs + ss + ws + fs, xs, gs, inputs);
    assert(w::rows_of(raw, tm) =~= us + rs + ss + ws + fs + xs + gs);
}

pub proof fn section(g: u::Guideline, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        t::all_sound(u::words_section(g.temporal), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::lines);
    hide(w::rows_of);
    let rs = u::word_rows(g.temporal);
    w::rows_unfold(g.temporal);
    corpus::temporal(g, inputs);
    if let Some(raw) = g.temporal {
        if let Ok(tm) = parse_temporal(raw) {
            m::own(raw, inputs);
            rows(raw, tm, inputs);
        }
    }
    assert(t::all_sound(rs, inputs));
    if rs.len() > 0 {
        l::f080(inputs);
        l::f276(inputs);
        l::f277(inputs);
        l::f102(inputs);
        l::f278(inputs);
        l::f083(inputs);
        l::f084(inputs);
        l::f085(inputs);
        l::f086(inputs);
        b::lines(rs, inputs);
        b::add(b::f(83), u::lines(rs), inputs);
        b::add(b::f(83) + u::lines(rs), b::f(84), inputs);
    }
}

} // verus!
