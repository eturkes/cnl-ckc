// S2 D7: qualifier tables contain declared copy, decimals and pl spans.
use crate::{
    k5_sound_build as b, k5_sound_escape as h, k5_sound_literals as l, k5_sound_model as m,
    k5_sound_strength as s, k5_sound_timing as t,
};
use ckc_spec::term::Term;
use ckc_spec::{ui as u, v1text as v};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn kind(value: u::Bytes, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::kind_html(value), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    l::f249(inputs);
    l::f336(inputs);
    l::f337(inputs);
    l::f338(inputs);
    l::f339(inputs);
    l::f340(inputs);
    l::f341(inputs);
    l::f342(inputs);
}

proof fn action(pl: u::Bytes, d: v::DocFile, c: v::DocClause, event: Term, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        h::sound(u::action_html(pl, d, c, event), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    hide(u::verb_html);
    s::verb(pl, d, c, event, inputs);
    let verb = u::verb_html(pl, d, c, event);
    if let Some(object) = u::participant(u::join_terms(d, c), event, 2) {
        if let Some(args) = u::first_with(u::join_terms(d, c), "guideline_entity"@, 4, 1, object) {
            let noun = u::pl_word(pl, u::atom_name(args[2]));
            t::pl_word(pl, u::atom_name(args[2]), inputs);
            l::f324(inputs);
            b::add(verb, b::f(324), inputs);
            b::add(verb + b::f(324), noun, inputs);
        }
    }
}

proof fn row(
    pl: u::Bytes,
    d: v::DocFile,
    sentence: nat,
    c: v::DocClause,
    literal: Term,
    part: int,
    inputs: Seq<u::Bytes>,
)
    requires
        m::backed(pl, inputs),
    ensures
        t::opt_sound(u::qualifier_row(pl, d, sentence, c, literal, part), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::stated_html);
    hide(u::action_html);
    hide(u::kind_html);
    hide(u::pl_word);
    hide(u::part_html);
    hide(u::cell);
    hide(u::row);
    hide(u::text);
    if let Term::Comp(name, args) = literal {
        if name == u::lit("guideline_qualifier"@) && args.len() == 4 {
            t::decimal_text(sentence, inputs);
            t::part(part, inputs);
            s::stated(d, c, args[0], inputs);
            action(pl, d, c, args[1], inputs);
            kind(u::atom_name(args[2]), inputs);
            t::pl_word(pl, u::atom_name(args[3]), inputs);
            let a = u::text(v::udec_bytes(sentence));
            let b1 = u::part_html(part);
            let stated = u::stated_html(d, c, args[0]);
            let event = u::action_html(pl, d, c, args[1]);
            let kind = u::kind_html(u::atom_name(args[2]));
            let word = u::pl_word(pl, u::atom_name(args[3]));
            b::cell(a, inputs);
            b::cell(b1, inputs);
            b::cell(stated, inputs);
            b::cell(event, inputs);
            b::cell(kind, inputs);
            b::cell(word, inputs);
            b::row(
                seq![
                    u::cell(a),
                    u::cell(b1),
                    u::cell(stated),
                    u::cell(event),
                    u::cell(kind),
                    u::cell(word),
                ],
                inputs,
            );
        }
    }
}

proof fn bundle(pl: u::Bytes, d: v::DocFile, bd: v::Bundle, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        t::all_sound(u::qualifier_bundle_rows(pl, d, bd), inputs),
{
    hide(u::qualifier_row);
    let f = |c: v::DocClause|
        u::clause_lits(c).map_values(|p: (Term, int)| u::qualifier_row(pl, d, bd.s, c, p.0, p.1));
    let xss = bd.clauses.map_values(f);
    assert forall|i: int, j: int| 0 <= i < xss.len() && 0 <= j < xss[i].len() implies t::opt_sound(
        #[trigger] xss[i][j],
        inputs,
    ) by {
        let c = bd.clauses[i];
        let p = u::clause_lits(c)[j];
        row(pl, d, bd.s, c, p.0, p.1, inputs);
    }
    t::flat_opts(xss, inputs);
    t::somes(xss.flatten(), inputs);
    t::first_rows(u::somes(xss.flatten()), inputs);
}

pub proof fn section(pl: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        t::all_sound(u::qualifier_section(pl), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::qualifier_bundle_rows);
    hide(u::lines);
    reveal(u::qualifier_rows);
    let rows = u::qualifier_rows(pl);
    if v::accepts(pl) {
        if let v::V1File::Doc(d) = ckc_spec::replay::the_v1(pl) {
            let f = |bd: v::Bundle| u::qualifier_bundle_rows(pl, d, bd);
            let xss = d.bundles.map_values(f);
            assert forall|i: int| 0 <= i < xss.len() implies t::all_sound(
                #[trigger] xss[i],
                inputs,
            ) by {
                bundle(pl, d, d.bundles[i], inputs);
            }
            t::flat_rows(xss, inputs);
        }
    }
    assert(t::all_sound(rows, inputs));
    if rows.len() > 0 {
        l::f080(inputs);
        l::f333(inputs);
        l::f334(inputs);
        l::f335(inputs);
        l::f081(inputs);
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
