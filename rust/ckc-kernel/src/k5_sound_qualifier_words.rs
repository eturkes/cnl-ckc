// S2 D7: qualifier words contain declared copy and vocabulary-table spans.
use crate::{
    k5_qualifier_words as w, k5_sound_build as b, k5_sound_corpus as corpus, k5_sound_escape as h,
    k5_sound_literals as l, k5_sound_model as m, k5_sound_qualifier as q, k5_sound_timing as t,
};
use ckc_spec::{ui as u, vocabulary::*};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

proof fn word(raw: u::Bytes, p: (u::Bytes, u::Bytes), inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(w::word(raw, p), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::pl_word);
    hide(u::kind_html);
    hide(u::cell);
    hide(u::row);
    t::pl_word(raw, p.0, inputs);
    q::kind(p.1, inputs);
    b::cell(u::pl_word(raw, p.0), inputs);
    b::cell(u::kind_html(p.1), inputs);
    b::row(seq![u::cell(u::pl_word(raw, p.0)), u::cell(u::kind_html(p.1))], inputs);
}

pub proof fn section(g: u::Guideline, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        t::all_sound(u::qualifier_words_section(g.vocabulary), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::lines);
    hide(w::word);
    let rows = u::qualifier_word_rows(g.vocabulary);
    w::rows_unfold(g.vocabulary);
    corpus::vocabulary(g, inputs);
    if let Some(raw) = g.vocabulary {
        if let Ok(table) = parse_vocabulary(raw) {
            m::own(raw, inputs);
            assert forall|i: int| 0 <= i < rows.len() implies h::sound(
                #[trigger] rows[i],
                inputs,
            ) by {
                word(raw, table.qualifiers[i], inputs);
            }
        }
    }
    assert(t::all_sound(rows, inputs));
    if rows.len() > 0 {
        l::f080(inputs);
        l::f345(inputs);
        l::f346(inputs);
        l::f102(inputs);
        l::f278(inputs);
        l::f083(inputs);
        l::f084(inputs);
        l::f085(inputs);
        l::f086(inputs);
        b::lines(rows, inputs);
        b::add(b::f(83), u::lines(rows), inputs);
        b::add(b::f(83) + u::lines(rows), b::f(84), inputs);
    }
}

} // verus!
