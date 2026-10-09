// K5 render soundness of the m7t D10 timing table: every visible slot is a
// registry literal, a canonical decimal, or a span of the document's pl.
use crate::{
    k5_sound_build as b, k5_sound_escape as h, k5_sound_literals as l, k5_sound_model as m,
    k5_sound_source as src,
};
use ckc_spec::term::Term;
use ckc_spec::{check as ck, ui as u, v1text as v};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn all_sound(xs: Seq<u::Html>, inputs: Seq<u::Bytes>) -> bool {
    forall|i: int| 0 <= i < xs.len() ==> h::sound(#[trigger] xs[i], inputs)
}

pub open spec fn opt_sound(o: Option<u::Html>, inputs: Seq<u::Bytes>) -> bool {
    match o {
        Some(x) => h::sound(x, inputs),
        None => true,
    }
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
    l::f249(inputs);
    if w.len() > 0 && ck::first_sub(pl, w, 0) + w.len() <= pl.len() {
        first_sub_found(pl, w, 0);
        assert(src::span_at(w, pl, ck::first_sub(pl, w, 0) as int));
        m::span(w, pl, inputs);
        m::copy(w, inputs);
        h::text(w, inputs);
    }
}

proof fn joined_word(
    pl: u::Bytes,
    d: v::DocFile,
    c: v::DocClause,
    name: Seq<char>,
    arity: nat,
    key: Term,
    at: int,
    inputs: Seq<u::Bytes>,
)
    requires
        m::backed(pl, inputs),
    ensures
        h::sound(u::joined_word(pl, d, c, name, arity, key, at), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::pl_word);
    l::f249(inputs);
    if let Some(args) = u::first_with(u::join_terms(d, c), name, arity, 1, key) {
        pl_word(pl, u::atom_name(args[at]), inputs);
    }
}

proof fn decimal_text(n: nat, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::text(v::udec_bytes(n)), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    src::decimal(n, inputs);
    assert(ck::nat_bytes(n) == v::udec_bytes(n));
    h::text(v::udec_bytes(n), inputs);
}

proof fn bound(d: v::DocFile, c: v::DocClause, q: Term, un: Term, inputs: Seq<u::Bytes>)
    ensures
        opt_sound(u::bound_html(d, c, q, un), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::text);
    hide(u::lit);
    hide(u::first_with);
    h::empty(inputs, 0);
    l::f250(inputs);
    l::f251(inputs);
    l::f252(inputs);
    l::f253(inputs);
    l::f254(inputs);
    l::f262(inputs);
    l::f263(inputs);
    l::f264(inputs);
    l::f265(inputs);
    l::f266(inputs);
    l::f267(inputs);
    l::f268(inputs);
    l::f269(inputs);
    l::f270(inputs);
    l::f271(inputs);
    l::f272(inputs);
    l::f273(inputs);
    l::f274(inputs);
    l::f275(inputs);
    if let Some(args) = u::first_with(u::join_terms(d, c), "guideline_cardinality"@, 5, 1, q) {
        if let (Some(cmp), Term::Int(n)) = (u::cmp_html(args[3]), args[4]) {
            if n >= 0 {
                if let Some(w) = u::unit_html(u::atom_name(un), n == 1) {
                    decimal_text(n as nat, inputs);
                    b::add(cmp, u::text(v::udec_bytes(n as nat)), inputs);
                    b::add(cmp + u::text(v::udec_bytes(n as nat)), w, inputs);
                }
            }
        }
    }
}

proof fn count(d: v::DocFile, c: v::DocClause, q: Term, inputs: Seq<u::Bytes>)
    ensures
        opt_sound(u::count_html(d, c, q), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::text);
    hide(u::lit);
    hide(u::first_with);
    h::empty(inputs, 0);
    l::f250(inputs);
    l::f251(inputs);
    l::f252(inputs);
    l::f253(inputs);
    l::f254(inputs);
    if let Some(args) = u::first_with(u::join_terms(d, c), "guideline_cardinality"@, 5, 1, q) {
        if let (Some(cmp), Term::Int(n)) = (u::cmp_html(args[3]), args[4]) {
            if n >= 0 {
                decimal_text(n as nat, inputs);
                b::add(cmp, u::text(v::udec_bytes(n as nat)), inputs);
            }
        }
    }
}

proof fn order(role: u::Bytes, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::order_html(role), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    l::f249(inputs);
    l::f290(inputs);
    l::f291(inputs);
}

proof fn frequency(
    pl: u::Bytes,
    d: v::DocFile,
    c: v::DocClause,
    args: Seq<Term>,
    inputs: Seq<u::Bytes>,
)
    requires
        m::backed(pl, inputs),
        args.len() == 5,
    ensures
        h::sound(u::frequency_html(pl, d, c, args), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::count_html);
    hide(u::bound_html);
    hide(u::joined_word);
    l::f249(inputs);
    l::f292(inputs);
    l::f293(inputs);
    count(d, c, args[2], inputs);
    bound(d, c, args[3], args[4], inputs);
    joined_word(pl, d, c, "guideline_entity"@, 4, args[2], 2, inputs);
    if let (Some(n), Some(w)) = (
        u::count_html(d, c, args[2]),
        u::bound_html(d, c, args[3], args[4]),
    ) {
        b::add(n, b::f(292), inputs);
        b::add(n + b::f(292), w, inputs);
        b::add(n + b::f(292) + w, b::f(293), inputs);
        b::add(
            n + b::f(292) + w + b::f(293),
            u::joined_word(pl, d, c, "guideline_entity"@, 4, args[2], 2),
            inputs,
        );
    }
}

proof fn window(d: v::DocFile, c: v::DocClause, args: Seq<Term>, inputs: Seq<u::Bytes>)
    requires
        args.len() == 7,
    ensures
        h::sound(u::window_html(d, c, args), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::bound_html);
    l::f249(inputs);
    l::f257(inputs);
    l::f294(inputs);
    l::f295(inputs);
    bound(d, c, args[2], args[3], inputs);
    bound(d, c, args[5], args[6], inputs);
    if let (Some(g), Some(ln)) = (
        u::bound_html(d, c, args[2], args[3]),
        u::bound_html(d, c, args[5], args[6]),
    ) {
        b::add(b::f(257), g, inputs);
        b::add(b::f(257) + g, b::f(294), inputs);
        b::add(b::f(257) + g + b::f(294), ln, inputs);
        b::add(b::f(257) + g + b::f(294) + ln, b::f(295), inputs);
    }
}

proof fn timing(role: u::Bytes, bnd: Option<u::Html>, inputs: Seq<u::Bytes>)
    requires
        opt_sound(bnd, inputs),
    ensures
        h::sound(u::timing_html(role, bnd), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    l::f249(inputs);
    l::f255(inputs);
    l::f256(inputs);
    l::f257(inputs);
    l::f258(inputs);
    l::f259(inputs);
    l::f260(inputs);
    l::f261(inputs);
    if let Some(x) = bnd {
        b::add(b::f(255), x, inputs);
        b::add(x, b::f(258), inputs);
        b::add(x, b::f(259), inputs);
        b::add(b::f(256), x, inputs);
        b::add(b::f(256) + x, b::f(260), inputs);
        b::add(b::f(257), x, inputs);
        b::add(b::f(257) + x, b::f(261), inputs);
    }
}

proof fn part(k: int, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::part_html(k), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::f246(inputs);
    l::f247(inputs);
    l::f248(inputs);
}

proof fn row(
    pl: u::Bytes,
    d: v::DocFile,
    s: nat,
    c: v::DocClause,
    t: Term,
    k: int,
    inputs: Seq<u::Bytes>,
)
    requires
        m::backed(pl, inputs),
    ensures
        opt_sound(u::timing_row(pl, d, s, c, t, k), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::text);
    hide(u::cell);
    hide(u::row);
    hide(u::part_html);
    hide(u::joined_word);
    hide(u::timing_html);
    hide(u::bound_html);
    hide(u::order_html);
    hide(u::frequency_html);
    hide(u::window_html);
    if let Term::Comp(name, args) = t {
        decimal_text(s, inputs);
        b::cell(u::text(v::udec_bytes(s)), inputs);
        part(k, inputs);
        b::cell(u::part_html(k), inputs);
        joined_word(pl, d, c, "guideline_event"@, 3, args[1], 2, inputs);
        b::cell(u::joined_word(pl, d, c, "guideline_event"@, 3, args[1], 2), inputs);
        h::empty(inputs, 0);
        b::cell(Seq::empty(), inputs);
        if name == u::lit("guideline_interval"@) && args.len() == 6 {
            let role = u::atom_name(args[2]);
            bound(d, c, args[3], args[4], inputs);
            timing(role, u::bound_html(d, c, args[3], args[4]), inputs);
            let tm = u::timing_html(role, u::bound_html(d, c, args[3], args[4]));
            b::cell(tm, inputs);
            l::f249(inputs);
            joined_word(pl, d, c, "guideline_entity"@, 4, args[5], 2, inputs);
            let anchor = if role == u::lit("duration"@) {
                Seq::empty()
            } else if args[5] == Term::Atom(u::lit("none"@)) {
                u::fixed_bytes(u::not_stated())
            } else {
                u::joined_word(pl, d, c, "guideline_entity"@, 4, args[5], 2)
            };
            b::cell(anchor, inputs);
            let xs = seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(k)),
                u::cell(u::joined_word(pl, d, c, "guideline_event"@, 3, args[1], 2)),
                u::cell(tm),
                u::cell(anchor),
            ];
            assert(all_sound(xs, inputs));
            b::row(xs, inputs);
        } else if name == u::lit("guideline_recurrence"@) && args.len() == 4 {
            let rc = u::lit("recurrence"@);
            bound(d, c, args[2], args[3], inputs);
            timing(rc, u::bound_html(d, c, args[2], args[3]), inputs);
            let tm = u::timing_html(rc, u::bound_html(d, c, args[2], args[3]));
            b::cell(tm, inputs);
            let xs = seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(k)),
                u::cell(u::joined_word(pl, d, c, "guideline_event"@, 3, args[1], 2)),
                u::cell(tm),
                u::cell(Seq::empty()),
            ];
            assert(all_sound(xs, inputs));
            b::row(xs, inputs);
        } else if name == u::lit("guideline_order"@) && args.len() == 4 {
            order(u::atom_name(args[2]), inputs);
            b::cell(u::order_html(u::atom_name(args[2])), inputs);
            joined_word(pl, d, c, "guideline_entity"@, 4, args[3], 2, inputs);
            b::cell(u::joined_word(pl, d, c, "guideline_entity"@, 4, args[3], 2), inputs);
            let xs = seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(k)),
                u::cell(u::joined_word(pl, d, c, "guideline_event"@, 3, args[1], 2)),
                u::cell(u::order_html(u::atom_name(args[2]))),
                u::cell(u::joined_word(pl, d, c, "guideline_entity"@, 4, args[3], 2)),
            ];
            assert(all_sound(xs, inputs));
            b::row(xs, inputs);
        } else if name == u::lit("guideline_frequency"@) && args.len() == 5 {
            frequency(pl, d, c, args, inputs);
            b::cell(u::frequency_html(pl, d, c, args), inputs);
            let xs = seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(k)),
                u::cell(u::joined_word(pl, d, c, "guideline_event"@, 3, args[1], 2)),
                u::cell(u::frequency_html(pl, d, c, args)),
                u::cell(Seq::empty()),
            ];
            assert(all_sound(xs, inputs));
            b::row(xs, inputs);
        } else if name == u::lit("guideline_recurrence_window"@) && args.len() == 7 {
            window(d, c, args, inputs);
            b::cell(u::window_html(d, c, args), inputs);
            joined_word(pl, d, c, "guideline_entity"@, 4, args[4], 2, inputs);
            b::cell(u::joined_word(pl, d, c, "guideline_entity"@, 4, args[4], 2), inputs);
            let xs = seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(k)),
                u::cell(u::joined_word(pl, d, c, "guideline_event"@, 3, args[1], 2)),
                u::cell(u::window_html(d, c, args)),
                u::cell(u::joined_word(pl, d, c, "guideline_entity"@, 4, args[4], 2)),
            ];
            assert(all_sound(xs, inputs));
            b::row(xs, inputs);
        }
    }
}

proof fn somes(xs: Seq<Option<u::Html>>, inputs: Seq<u::Bytes>)
    requires
        forall|i: int| 0 <= i < xs.len() ==> opt_sound(#[trigger] xs[i], inputs),
    ensures
        all_sound(u::somes(xs), inputs),
    decreases xs.len(),
{
    if xs.len() > 0 {
        let init = xs.drop_last();
        assert forall|i: int| 0 <= i < init.len() implies opt_sound(#[trigger] init[i], inputs) by {
            assert(init[i] == xs[i]);
        }
        somes(init, inputs);
        assert(opt_sound(xs[xs.len() - 1], inputs));
    }
}

proof fn first_rows(xs: Seq<u::Html>, inputs: Seq<u::Bytes>)
    requires
        all_sound(xs, inputs),
    ensures
        all_sound(u::first_rows(xs), inputs),
    decreases xs.len(),
{
    if xs.len() > 0 {
        let init = xs.drop_last();
        assert forall|i: int| 0 <= i < init.len() implies h::sound(#[trigger] init[i], inputs) by {
            assert(init[i] == xs[i]);
        }
        first_rows(init, inputs);
        assert(h::sound(xs[xs.len() - 1], inputs));
    }
}

proof fn flat_opts(xss: Seq<Seq<Option<u::Html>>>, inputs: Seq<u::Bytes>)
    requires
        forall|i: int, j: int|
            0 <= i < xss.len() && 0 <= j < xss[i].len() ==> opt_sound(#[trigger] xss[i][j], inputs),
    ensures
        forall|k: int|
            0 <= k < xss.flatten().len() ==> opt_sound(#[trigger] xss.flatten()[k], inputs),
    decreases xss.len(),
{
    reveal_with_fuel(Seq::<_>::flatten, 1);
    if xss.len() > 0 {
        let init = xss.drop_last();
        assert forall|i: int, j: int|
            0 <= i < init.len() && 0 <= j < init[i].len() implies opt_sound(
            #[trigger] init[i][j],
            inputs,
        ) by {
            assert(init[i] == xss[i]);
        }
        flat_opts(init, inputs);
        assert(xss =~= init.push(xss.last()));
        init.lemma_flatten_push(xss.last());
        assert forall|k: int| 0 <= k < xss.flatten().len() implies opt_sound(
            #[trigger] xss.flatten()[k],
            inputs,
        ) by {
            if k < init.flatten().len() {
                assert(xss.flatten()[k] == init.flatten()[k]);
            } else {
                let j = k - init.flatten().len();
                assert(xss.flatten()[k] == xss[xss.len() - 1][j]);
            }
        }
    }
}

proof fn flat_rows(xss: Seq<Seq<u::Html>>, inputs: Seq<u::Bytes>)
    requires
        forall|i: int| 0 <= i < xss.len() ==> all_sound(#[trigger] xss[i], inputs),
    ensures
        all_sound(xss.flatten(), inputs),
    decreases xss.len(),
{
    reveal_with_fuel(Seq::<_>::flatten, 1);
    if xss.len() > 0 {
        let init = xss.drop_last();
        assert forall|i: int| 0 <= i < init.len() implies all_sound(#[trigger] init[i], inputs) by {
            assert(init[i] == xss[i]);
        }
        flat_rows(init, inputs);
        assert(xss =~= init.push(xss.last()));
        init.lemma_flatten_push(xss.last());
        assert(all_sound(xss[xss.len() - 1], inputs));
        assert forall|k: int| 0 <= k < xss.flatten().len() implies h::sound(
            #[trigger] xss.flatten()[k],
            inputs,
        ) by {
            if k < init.flatten().len() {
                assert(xss.flatten()[k] == init.flatten()[k]);
            } else {
                assert(xss.flatten()[k] == xss[xss.len() - 1][k - init.flatten().len()]);
            }
        }
    }
}

proof fn bundle(pl: u::Bytes, d: v::DocFile, bd: v::Bundle, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        all_sound(u::bundle_rows(pl, d, bd), inputs),
{
    hide(u::timing_row);
    let f = |c: v::DocClause|
        u::clause_lits(c).map_values(|p: (Term, int)| u::timing_row(pl, d, bd.s, c, p.0, p.1));
    let xss = bd.clauses.map_values(f);
    assert forall|i: int, j: int| 0 <= i < xss.len() && 0 <= j < xss[i].len() implies opt_sound(
        #[trigger] xss[i][j],
        inputs,
    ) by {
        let c = bd.clauses[i];
        let p = u::clause_lits(c)[j];
        assert(xss[i][j] == u::timing_row(pl, d, bd.s, c, p.0, p.1));
        row(pl, d, bd.s, c, p.0, p.1, inputs);
    }
    flat_opts(xss, inputs);
    somes(xss.flatten(), inputs);
    first_rows(u::somes(xss.flatten()), inputs);
}

// Every line of the timing section is sound when the pl is backed by the inputs.
pub proof fn section(pl: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        m::backed(pl, inputs),
    ensures
        all_sound(u::timing_section(pl), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lit);
    hide(u::bundle_rows);
    hide(u::lines);
    reveal(u::timing_rows);
    let rows = u::timing_rows(pl);
    if v::accepts(pl) {
        if let v::V1File::Doc(d) = ckc_spec::replay::the_v1(pl) {
            if d.version >= 2 {
                let g = |bd: v::Bundle| u::bundle_rows(pl, d, bd);
                let xss = d.bundles.map_values(g);
                assert forall|i: int| 0 <= i < xss.len() implies all_sound(
                    #[trigger] xss[i],
                    inputs,
                ) by {
                    bundle(pl, d, d.bundles[i], inputs);
                }
                flat_rows(xss, inputs);
            }
        }
    }
    assert(all_sound(rows, inputs));
    if rows.len() > 0 {
        l::f080(inputs);
        l::f243(inputs);
        l::f244(inputs);
        l::f081(inputs);
        l::f245(inputs);
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
