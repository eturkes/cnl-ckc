// S1 D8: strength tables contain only declared copy, decimals and pl spans.
use crate::{
    k5_sound_build as b, k5_sound_escape as h, k5_sound_literals as l, k5_sound_model as m,
    k5_sound_timing as t,
};
use ckc_spec::term::Term;
use ckc_spec::{ui as u, v1text as v};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn grade_value(value: u::Bytes, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::grade_value_html(value), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    l::f249(inputs);
    l::f308(inputs);
    l::f309(inputs);
    l::f310(inputs);
    l::f311(inputs);
    l::f312(inputs);
    l::f313(inputs);
}

proof fn dimension(dim: u::Bytes, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::dimension_html(dim), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    l::f249(inputs);
    l::f306(inputs);
    l::f307(inputs);
}

proof fn noun(pl: u::Bytes, d: v::DocFile, c: v::DocClause, r: Term, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        h::sound(u::noun_html(pl, d, c, r), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::pl_word);
    t::joined_word(pl, d, c, "guideline_entity"@, 4, r, 2, inputs);
}

pub proof fn verb(pl: u::Bytes, d: v::DocFile, c: v::DocClause, event: Term, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        h::sound(u::verb_html(pl, d, c, event), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::pl_word);
    hide(u::noun_html);
    let ts = u::join_terms(d, c);
    l::f249(inputs);
    if let Some(args) = u::first_with(ts, "guideline_event"@, 3, 1, event) {
        if u::atom_name(args[2]) == u::lit("be"@) {
            if let Some(subject) = u::participant(ts, event, 1) {
                noun(pl, d, c, subject, inputs);
            }
        } else {
            t::pl_word(pl, u::atom_name(args[2]), inputs);
        }
    }
}

proof fn grade_row(pl: u::Bytes, d: v::DocFile, c: v::DocClause, x: Term, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        t::opt_sound(u::grade_row(pl, d, c, x), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::noun_html);
    hide(u::grade_value_html);
    hide(u::dimension_html);
    hide(u::cell);
    hide(u::row);
    if let Term::Comp(name, args) = x {
        if name == u::lit("guideline_grade"@) && args.len() == 5 {
            let a = u::dimension_html(u::atom_name(args[2]));
            let v = u::grade_value_html(u::atom_name(args[3]));
            let n = u::noun_html(pl, d, c, args[4]);
            dimension(u::atom_name(args[2]), inputs);
            grade_value(u::atom_name(args[3]), inputs);
            noun(pl, d, c, args[4], inputs);
            b::cell(a, inputs);
            b::cell(v, inputs);
            b::cell(n, inputs);
            b::row(seq![u::cell(a), u::cell(v), u::cell(n)], inputs);
        }
    }
}

proof fn op(value: u::Bytes, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::op_html(value), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    l::f249(inputs);
    l::f317(inputs);
    l::f318(inputs);
    l::f319(inputs);
    l::f320(inputs);
    l::f321(inputs);
}

proof fn stated_ops(ts: Seq<Term>, ctx: Term, fuel: nat, inputs: Seq<u::Bytes>)
    ensures
        t::all_sound(u::stated_ops(ts, ctx, fuel), inputs),
    decreases fuel,
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::op_html);
    if fuel > 0 && ctx != Term::Atom(u::lit("actual"@)) {
        if let Some(args) = u::first_with(ts, "guideline_operator"@, 3, 1, ctx) {
            stated_ops(ts, args[0], (fuel - 1) as nat, inputs);
            op(u::atom_name(args[2]), inputs);
            let before = u::stated_ops(ts, args[0], (fuel - 1) as nat);
            let result = before.push(u::op_html(u::atom_name(args[2])));
            assert forall|i: int| 0 <= i < result.len() implies h::sound(
                #[trigger] result[i],
                inputs,
            ) by {
                if i < before.len() {
                    assert(result[i] == before[i]);
                }
            }
        }
    }
}

proof fn spaced(xs: Seq<u::Html>, inputs: Seq<u::Bytes>)
    requires
        t::all_sound(xs, inputs),
    ensures
        h::sound(u::spaced(xs), inputs),
    decreases xs.len(),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    if xs.len() == 0 {
        h::empty(inputs, 0);
    } else if xs.len() == 1 {
        assert(h::sound(xs[0], inputs));
    } else {
        let init = xs.drop_last();
        assert forall|i: int| 0 <= i < init.len() implies h::sound(#[trigger] init[i], inputs) by {
            assert(init[i] == xs[i]);
        }
        spaced(init, inputs);
        l::f324(inputs);
        assert(h::sound(xs[xs.len() - 1], inputs));
        b::add(u::spaced(init), b::f(324), inputs);
        b::add(u::spaced(init) + b::f(324), xs.last(), inputs);
    }
}

pub proof fn stated(d: v::DocFile, c: v::DocClause, ctx: Term, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::stated_html(d, c, ctx), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::stated_ops);
    hide(u::spaced);
    let ts = u::join_terms(d, c);
    let ops = u::stated_ops(ts, ctx, ts.len());
    stated_ops(ts, ctx, ts.len(), inputs);
    if ops.len() == 0 {
        l::f322(inputs);
    } else {
        spaced(ops, inputs);
    }
}

proof fn preference_row(
    pl: u::Bytes,
    d: v::DocFile,
    s: nat,
    c: v::DocClause,
    x: Term,
    part: int,
    inputs: Seq<u::Bytes>,
)
    requires
        m::backed(pl, inputs),
    ensures
        t::opt_sound(u::preference_row(pl, d, s, c, x, part), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::noun_html);
    hide(u::stated_html);
    hide(u::verb_html);
    hide(u::part_html);
    hide(u::cell);
    hide(u::row);
    hide(u::text);
    if let Term::Comp(name, args) = x {
        if name == u::lit("guideline_preference"@) && args.len() == 4 {
            t::decimal_text(s, inputs);
            t::part(part, inputs);
            stated(d, c, args[0], inputs);
            verb(pl, d, c, args[1], inputs);
            noun(pl, d, c, args[2], inputs);
            noun(pl, d, c, args[3], inputs);
            l::f323(inputs);
            let a = u::text(v::udec_bytes(s));
            let b1 = u::part_html(part);
            let c1 = u::stated_html(d, c, args[0]);
            let e = u::verb_html(pl, d, c, args[1]);
            let p = u::noun_html(pl, d, c, args[2]);
            let alt = if args[3] == Term::Atom(u::lit("none"@)) {
                b::f(323)
            } else {
                u::noun_html(pl, d, c, args[3])
            };
            b::cell(a, inputs);
            b::cell(b1, inputs);
            b::cell(c1, inputs);
            b::cell(e, inputs);
            b::cell(p, inputs);
            b::cell(alt, inputs);
            b::row(
                seq![u::cell(a), u::cell(b1), u::cell(c1), u::cell(e), u::cell(p), u::cell(alt)],
                inputs,
            );
            assert(seq![u::cell(a), u::cell(b1), u::cell(c1), u::cell(e), u::cell(p), u::cell(alt)]
                =~= crate::k5_strength::preference_cells(pl, d, s, c, args, part));
            crate::k5_strength::preference_unfold(pl, d, s, c, name, args, part);
        }
    }
}

proof fn bundle(pl: u::Bytes, d: v::DocFile, bd: v::Bundle, grades: bool, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        t::all_sound(u::strength_bundle_rows(pl, d, bd, grades), inputs),
{
    hide(u::grade_row);
    hide(u::preference_row);
    let f = |c: v::DocClause|
        u::clause_lits(c).map_values(
            |p: (Term, int)|
                if grades {
                    u::grade_row(pl, d, c, p.0)
                } else {
                    u::preference_row(pl, d, bd.s, c, p.0, p.1)
                },
        );
    let xss = bd.clauses.map_values(f);
    assert forall|i: int, j: int| 0 <= i < xss.len() && 0 <= j < xss[i].len() implies t::opt_sound(
        #[trigger] xss[i][j],
        inputs,
    ) by {
        let c = bd.clauses[i];
        let p = u::clause_lits(c)[j];
        if grades {
            grade_row(pl, d, c, p.0, inputs);
        } else {
            preference_row(pl, d, bd.s, c, p.0, p.1, inputs);
        }
    }
    t::flat_opts(xss, inputs);
    t::somes(xss.flatten(), inputs);
    t::first_rows(u::somes(xss.flatten()), inputs);
}

pub proof fn section(pl: u::Bytes, grades: bool, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        t::all_sound(u::strength_section(pl, grades), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::strength_bundle_rows);
    hide(u::lines);
    reveal(u::strength_rows);
    let rows = u::strength_rows(pl, grades);
    if v::accepts(pl) {
        if let v::V1File::Doc(d) = ckc_spec::replay::the_v1(pl) {
            let g = |bd: v::Bundle| u::strength_bundle_rows(pl, d, bd, grades);
            let xss = d.bundles.map_values(g);
            assert forall|i: int| 0 <= i < xss.len() implies t::all_sound(
                #[trigger] xss[i],
                inputs,
            ) by {
                bundle(pl, d, d.bundles[i], grades, inputs);
            }
            t::flat_rows(xss, inputs);
        }
    }
    assert(t::all_sound(rows, inputs));
    if rows.len() > 0 {
        l::f080(inputs);
        l::f303(inputs);
        l::f304(inputs);
        l::f305(inputs);
        l::f314(inputs);
        l::f315(inputs);
        l::f316(inputs);
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
