// S1 D8: word tables contain declared copy and vocabulary-table spans.
use crate::{
    k5_sound_build as b, k5_sound_corpus as corpus, k5_sound_escape as h, k5_sound_literals as l,
    k5_sound_model as m, k5_sound_strength as strength, k5_sound_timing as t,
    k5_strength_words as w,
};
use ckc_spec::{ui as u, vocabulary::*};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

proof fn grade_word(
    raw: u::Bytes,
    p: (u::Bytes, u::Bytes),
    is_strength: bool,
    inputs: Seq<u::Bytes>,
)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(w::grade_word(raw, p, is_strength), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    hide(u::grade_value_html);
    hide(u::cell);
    hide(u::row);
    t::pl_word(raw, p.0, inputs);
    strength::grade_value(p.1, inputs);
    l::f327(inputs);
    l::f328(inputs);
    let prefix = if is_strength {
        b::f(327)
    } else {
        b::f(328)
    };
    let value = prefix + u::grade_value_html(p.1);
    b::add(prefix, u::grade_value_html(p.1), inputs);
    b::cell(u::pl_word(raw, p.0), inputs);
    b::cell(value, inputs);
    b::row(seq![u::cell(u::pl_word(raw, p.0)), u::cell(value)], inputs);
}

proof fn preference_word(raw: u::Bytes, p: u::Bytes, preference: bool, inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        h::sound(w::preference_word(raw, p, preference), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    hide(u::cell);
    hide(u::row);
    t::pl_word(raw, p, inputs);
    l::f331(inputs);
    l::f332(inputs);
    let value = if preference {
        b::f(331)
    } else {
        b::f(332)
    };
    b::cell(u::pl_word(raw, p), inputs);
    b::cell(value, inputs);
    b::row(seq![u::cell(u::pl_word(raw, p)), u::cell(value)], inputs);
}

proof fn rows(raw: u::Bytes, tm: Vocabulary, grades: bool, inputs: Seq<u::Bytes>)
    requires
        m::backed(raw, inputs),
    ensures
        t::all_sound(w::rows_of(raw, tm, grades), inputs),
{
    hide(w::grade_word);
    hide(w::preference_word);
    if grades {
        let a = tm.strengths.map_values(|p: (u::Bytes, u::Bytes)| w::grade_word(raw, p, true));
        let b1 = tm.evidences.map_values(|p: (u::Bytes, u::Bytes)| w::grade_word(raw, p, false));
        assert forall|i: int| 0 <= i < a.len() implies h::sound(#[trigger] a[i], inputs) by {
            grade_word(raw, tm.strengths[i], true, inputs);
        }
        assert forall|i: int| 0 <= i < b1.len() implies h::sound(#[trigger] b1[i], inputs) by {
            grade_word(raw, tm.evidences[i], false, inputs);
        }
        t::cat3(a, b1, Seq::empty(), inputs);
        assert(w::rows_of(raw, tm, grades) =~= a + b1 + Seq::<u::Html>::empty());
    } else {
        let a = tm.preferences.map_values(|p: u::Bytes| w::preference_word(raw, p, true));
        let b1 = tm.alternatives.map_values(|p: u::Bytes| w::preference_word(raw, p, false));
        assert forall|i: int| 0 <= i < a.len() implies h::sound(#[trigger] a[i], inputs) by {
            preference_word(raw, tm.preferences[i], true, inputs);
        }
        assert forall|i: int| 0 <= i < b1.len() implies h::sound(#[trigger] b1[i], inputs) by {
            preference_word(raw, tm.alternatives[i], false, inputs);
        }
        t::cat3(a, b1, Seq::empty(), inputs);
        assert(w::rows_of(raw, tm, grades) =~= a + b1 + Seq::<u::Html>::empty());
    }
}

pub proof fn section(g: u::Guideline, grades: bool, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        t::all_sound(u::strength_words_section(g.vocabulary, grades), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::lines);
    hide(w::rows_of);
    let rs = u::strength_word_rows(g.vocabulary, grades);
    w::rows_unfold(g.vocabulary, grades);
    corpus::vocabulary(g, inputs);
    if let Some(raw) = g.vocabulary {
        if let Ok(tm) = parse_vocabulary(raw) {
            m::own(raw, inputs);
            rows(raw, tm, grades, inputs);
        }
    }
    assert(t::all_sound(rs, inputs));
    if rs.len() > 0 {
        l::f080(inputs);
        l::f325(inputs);
        l::f326(inputs);
        l::f329(inputs);
        l::f330(inputs);
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
