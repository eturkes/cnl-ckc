// S1 D8: document grade + preference tables; joins share the timing renderer.
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENodeKind, ETermArena};
use crate::k5_html as h;
use crate::k5_timing as t;
use crate::m6_term::T;
#[cfg(verus_keep_ghost)]
use crate::m6_term::{models, valid, valid_all};
use ckc_spec::term::Term;
use ckc_spec::ui::{self as u, EPage};
use ckc_spec::v1text as v;
use vstd::prelude::*;
verus! {

pub fn atom_name_exec(arena: &ETermArena, x: &T) -> (out: Vec<u8>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, x),
    ensures
        out@ == u::atom_name(x@),
{
    match t::atom_of(arena, x) {
        Some(a) => a,
        None => Vec::new(),
    }
}

pub fn grade_value_exec(value: &[u8]) -> (out: EPage)
    ensures
        out@ == u::grade_value_html(value@),
{
    if t::is(value, "strong") {
        h::fixed("strong")
    } else if t::is(value, "conditional") {
        h::fixed("conditional")
    } else if t::is(value, "high") {
        h::fixed("high")
    } else if t::is(value, "moderate") {
        h::fixed("moderate")
    } else if t::is(value, "low") {
        h::fixed("low")
    } else if t::is(value, "very_low") {
        h::fixed("very low")
    } else {
        h::fixed("not stated")
    }
}

pub fn dimension_exec(dim: &[u8]) -> (out: EPage)
    ensures
        out@ == u::dimension_html(dim@),
{
    if t::is(dim, "strength") {
        h::fixed("Strength of the recommendation")
    } else if t::is(dim, "evidence") {
        h::fixed("Certainty of the evidence")
    } else {
        h::fixed("not stated")
    }
}

pub fn noun_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    r: &T,
) -> (out: EPage)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, r),
    ensures
        out@ == u::noun_html(pl@, d, c, r@),
{
    t::joined_exec(arena, pl, join, Ghost(d), Ghost(c), "guideline_entity", 4, r, 2)
}

fn position(arena: &ETermArena, x: &T, digits: &[u8], Ghost(n): Ghost<nat>) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, x),
        digits@ == v::udec_bytes(n),
    ensures
        out == (x@ == Term::Int(n as int)),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, x.root as int));
        reveal(crate::k2_term::node_ok);
    }
    match &arena.nodes[x.root].kind {
        ENodeKind::Int { magnitude, negative, value, .. } => {
            if *negative {
                return false;
            }
            let same = crate::k5_bytes::equal(magnitude, digits);
            proof {
                if same {
                    crate::v1_term_impl::udec_bytes_injective(value@ as nat, n);
                }
            }
            same
        },
        _ => false,
    }
}

pub fn participant_exec(arena: &ETermArena, ts: &Vec<T>, event: &T, n: u64) -> (out: Option<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ts@),
        valid(arena.nodes@, event),
    ensures
        match out {
            Some(x) => valid(arena.nodes@, &x) && u::participant(models(ts@), event@, n as int)
                == Some(x@),
            None => u::participant(models(ts@), event@, n as int) is None,
        },
{
    let digits = crate::k5_bytes::nat_bytes(n);
    let ghost ms = models(ts@);
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
    }
    while i < ts.len()
        invariant
            i <= ts@.len(),
            arena_ok(arena),
            valid_all(arena.nodes@, ts@),
            valid(arena.nodes@, event),
            digits@ == v::udec_bytes(n as nat),
            ms == models(ts@),
            ms.len() == ts@.len(),
            u::participant(ms, event@, n as int) == u::participant(
                ms.skip(i as int),
                event@,
                n as int,
            ),
        decreases ts@.len() - i,
    {
        proof {
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
            assert(valid(arena.nodes@, &ts@[i as int]));
        }
        match crate::m6_term::parts(arena, &ts[i]) {
            Some((name, args)) => {
                if t::is(&name, "guideline_arg") && args.len() == 4 {
                    proof {
                        assert forall|j: int| 0 <= j < args@.len() implies models(args@)[j]
                            == args@[j]@ && valid(arena.nodes@, &args@[j]) by {}
                    }
                    if crate::m6_term::equal(arena, &args[1], event) && position(
                        arena,
                        &args[2],
                        &digits,
                        Ghost(n as nat),
                    ) {
                        return Some(args[3].cp());
                    }
                }
            },
            None => {},
        }
        i += 1;
    }
    proof {
        assert(ms.skip(i as int).len() == 0);
    }
    None
}

pub fn verb_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    event: &T,
) -> (out: EPage)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, event),
    ensures
        out@ == u::verb_html(pl@, d, c, event@),
{
    hide(u::noun_html);
    hide(u::pl_word);
    let name = crate::k5_bytes::literal("guideline_event");
    let args = match t::first_with_exec(
        arena,
        join,
        &name,
        Ghost("guideline_event"@),
        3,
        1,
        event,
    ) {
        None => return h::fixed("not stated"),
        Some(a) => a,
    };
    proof {
        assert(models(args@)[2] == args@[2]@);
        assert(valid(arena.nodes@, &args@[2]));
    }
    let lemma = atom_name_exec(arena, &args[2]);
    if t::is(&lemma, "be") {
        match participant_exec(arena, join, event, 1) {
            Some(subject) => noun_exec(arena, pl, join, Ghost(d), Ghost(c), &subject),
            None => h::fixed("not stated"),
        }
    } else {
        t::pl_word_exec(pl, &lemma)
    }
}

pub fn grade_row_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    l: &T,
) -> (out: Option<EPage>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, l),
    ensures
        t::opt_page(out) == u::grade_row(pl@, d, c, l@),
{
    hide(u::noun_html);
    hide(u::grade_value_html);
    hide(u::dimension_html);
    hide(u::row);
    hide(u::cell);
    hide(u::lit);
    let (name, args) = match crate::m6_term::parts(arena, l) {
        None => return None,
        Some(x) => x,
    };
    if !t::is(&name, "guideline_grade") || args.len() != 5 {
        return None;
    }
    proof {
        assert forall|i: int| 0 <= i < args@.len() implies models(args@)[i] == args@[i]@ && valid(
            arena.nodes@,
            &args@[i],
        ) by {}
    }
    let a = dimension_exec(&atom_name_exec(arena, &args[2]));
    let b = grade_value_exec(&atom_name_exec(arena, &args[3]));
    let c = noun_exec(arena, pl, join, Ghost(d), Ghost(c), &args[4]);
    let ghost av = a@;
    let ghost bv = b@;
    let ghost cv = c@;
    let mut cells: Vec<EPage> = Vec::new();
    cells.push(h::cell(a));
    cells.push(h::cell(b));
    cells.push(h::cell(c));
    proof {
        assert(h::pages(cells@) =~= seq![u::cell(av), u::cell(bv), u::cell(cv)]);
    }
    Some(h::row(&cells))
}

pub fn op_exec(op: &[u8]) -> (out: EPage)
    ensures
        out@ == u::op_html(op@),
{
    if t::is(op, "should") {
        h::fixed("should")
    } else if t::is(op, "must") {
        h::fixed("must")
    } else if t::is(op, "can") {
        h::fixed("can")
    } else if t::is(op, "may") {
        h::fixed("may")
    } else if t::is(op, "-") {
        h::fixed("not")
    } else {
        h::fixed("not stated")
    }
}

fn reverse_pages(mut xs: Vec<EPage>) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == h::pages(xs@).reverse(),
{
    let ghost original = h::pages(xs@);
    let mut out: Vec<EPage> = Vec::new();
    proof {
        assert(h::pages(out@) + h::pages(xs@).reverse() =~= original.reverse());
    }
    while xs.len() > 0
        invariant
            h::pages(out@) + h::pages(xs@).reverse() == original.reverse(),
        decreases xs@.len(),
    {
        let ghost before = h::pages(xs@);
        let ghost prefix = h::pages(out@);
        let last = xs.pop().unwrap();
        out.push(last);
        proof {
            assert(before =~= h::pages(xs@).push(last@));
            assert(before.reverse() =~= seq![last@] + h::pages(xs@).reverse());
            assert(h::pages(out@) =~= prefix.push(last@));
        }
    }
    proof {
        assert(h::pages(xs@).reverse() =~= Seq::<u::Html>::empty());
    }
    out
}

pub fn stated_ops_exec(arena: &ETermArena, ts: &Vec<T>, ctx: &T, fuel: usize) -> (out: Vec<EPage>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ts@),
        valid(arena.nodes@, ctx),
    ensures
        h::pages(out@) == u::stated_ops(models(ts@), ctx@, fuel as nat),
{
    hide(u::op_html);
    hide(u::lit);
    let name = crate::k5_bytes::literal("guideline_operator");
    let mut current = ctx.cp();
    let mut remaining = fuel;
    let mut pending: Vec<EPage> = Vec::new();
    let ghost target = u::stated_ops(models(ts@), ctx@, fuel as nat);
    proof {
        assert(h::pages(pending@).reverse() =~= Seq::<u::Html>::empty());
    }
    while remaining > 0
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ts@),
            valid(arena.nodes@, &current),
            name@ == u::lit("guideline_operator"@),
            target == u::stated_ops(models(ts@), ctx@, fuel as nat),
            target == u::stated_ops(models(ts@), current@, remaining as nat) + h::pages(
                pending@,
            ).reverse(),
        decreases remaining,
    {
        let actual = match t::atom_of(arena, &current) {
            Some(a) => t::is(&a, "actual"),
            None => false,
        };
        if actual {
            proof {
                assert(current@ == Term::Atom(u::lit("actual"@)));
                assert(u::stated_ops(models(ts@), current@, remaining as nat) =~= Seq::<
                    u::Html,
                >::empty());
            }
            return reverse_pages(pending);
        }
        let args = match t::first_with_exec(
            arena,
            ts,
            &name,
            Ghost("guideline_operator"@),
            3,
            1,
            &current,
        ) {
            None => return reverse_pages(pending),
            Some(a) => a,
        };
        proof {
            assert forall|i: int| 0 <= i < args@.len() implies models(args@)[i] == args@[i]@
                && valid(arena.nodes@, &args@[i]) by {}
        }
        let op = op_exec(&atom_name_exec(arena, &args[2]));
        let ghost before = h::pages(pending@);
        let ghost old_context = current@;
        pending.push(op);
        current = args[0].cp();
        proof {
            assert(h::pages(pending@) =~= before.push(op@));
            assert(h::pages(pending@).reverse() =~= seq![op@] + before.reverse());
            assert(u::stated_ops(models(ts@), old_context, remaining as nat) =~= u::stated_ops(
                models(ts@),
                current@,
                (remaining - 1) as nat,
            ).push(op@));
        }
        remaining -= 1;
    }
    reverse_pages(pending)
}

pub fn spaced_exec(xs: &Vec<EPage>) -> (out: EPage)
    ensures
        out@ == u::spaced(h::pages(xs@)),
{
    let mut out = h::empty();
    let mut i = 0usize;
    proof {
        assert(h::pages(xs@).take(0) =~= Seq::<u::Html>::empty());
    }
    while i < xs.len()
        invariant
            i <= xs@.len(),
            out@ == u::spaced(h::pages(xs@).take(i as int)),
        decreases xs@.len() - i,
    {
        proof {
            let ps = h::pages(xs@);
            assert(ps.take(i + 1).drop_last() =~= ps.take(i as int));
            assert(ps.take(i + 1).last() == xs@[i as int]@);
        }
        if i > 0 {
            h::append(&mut out, h::fixed(" "));
        }
        h::append(&mut out, h::copy(&xs[i]));
        i += 1;
    }
    proof {
        assert(h::pages(xs@).take(i as int) =~= h::pages(xs@));
    }
    out
}

pub fn stated_exec(
    arena: &ETermArena,
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    ctx: &T,
) -> (out: EPage)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, ctx),
    ensures
        out@ == u::stated_html(d, c, ctx@),
{
    let ops = stated_ops_exec(arena, join, ctx, join.len());
    if ops.len() == 0 {
        h::fixed("as a fact")
    } else {
        spaced_exec(&ops)
    }
}

pub open spec fn preference_cells(
    pl: u::Bytes,
    d: v::DocFile,
    s: nat,
    c: v::DocClause,
    args: Seq<Term>,
    part: int,
) -> Seq<u::Html> {
    seq![
        u::cell(u::text(v::udec_bytes(s))),
        u::cell(u::part_html(part)),
        u::cell(u::stated_html(d, c, args[0])),
        u::cell(u::verb_html(pl, d, c, args[1])),
        u::cell(u::noun_html(pl, d, c, args[2])),
        u::cell(
            if args[3] == Term::Atom(u::lit("none"@)) {
                u::fixed_bytes(u::none_named())
            } else {
                u::noun_html(pl, d, c, args[3])
            },
        ),
    ]
}

// Expose the six-cell flatten; opaque row applications obscure the join equivalence.
pub proof fn preference_unfold(
    pl: u::Bytes,
    d: v::DocFile,
    s: nat,
    c: v::DocClause,
    name: u::Bytes,
    args: Seq<Term>,
    part: int,
)
    requires
        name == u::lit("guideline_preference"@),
        args.len() == 4,
    ensures
        u::preference_row(pl, d, s, c, Term::Comp(name, args), part) == Some(
            u::row(preference_cells(pl, d, s, c, args, part)),
        ),
{
    hide(u::noun_html);
    hide(u::stated_html);
    hide(u::part_html);
    hide(u::verb_html);
    hide(u::lit);
    reveal_with_fuel(Seq::<_>::flatten, 7);
    broadcast use {vstd::seq::group_seq_lemmas, vstd::seq_lib::group_seq_properties};

    let action = u::verb_html(pl, d, c, args[1]);
    let cells = seq![
        u::cell(u::text(v::udec_bytes(s))),
        u::cell(u::part_html(part)),
        u::cell(u::stated_html(d, c, args[0])),
        u::cell(action),
        u::cell(u::noun_html(pl, d, c, args[2])),
        u::cell(
            if args[3] == Term::Atom(u::lit("none"@)) {
                u::fixed_bytes(u::none_named())
            } else {
                u::noun_html(pl, d, c, args[3])
            },
        ),
    ];
    assert(preference_cells(pl, d, s, c, args, part) =~= cells);
    assert(u::preference_row(pl, d, s, c, Term::Comp(name, args), part) == Some(u::row(cells)));
}

pub fn preference_row_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    ordinal: &[u8],
    Ghost(s): Ghost<nat>,
    l: &T,
    part: u8,
) -> (out: Option<EPage>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, l),
        ordinal@ == v::udec_bytes(s),
        part <= 2,
    ensures
        t::opt_page(out) == u::preference_row(pl@, d, s, c, l@, part as int),
{
    hide(u::noun_html);
    hide(u::stated_html);
    hide(u::verb_html);
    hide(u::part_html);
    hide(u::row);
    hide(u::cell);
    hide(u::lit);
    let (name, args) = match crate::m6_term::parts(arena, l) {
        None => return None,
        Some(x) => x,
    };
    if !t::is(&name, "guideline_preference") || args.len() != 4 {
        return None;
    }
    proof {
        assert forall|i: int| 0 <= i < args@.len() implies models(args@)[i] == args@[i]@ && valid(
            arena.nodes@,
            &args@[i],
        ) by {}
    }
    let a = h::text(ordinal);
    let b = t::part_exec(part);
    let stated = stated_exec(arena, join, Ghost(d), Ghost(c), &args[0]);
    let event = verb_exec(arena, pl, join, Ghost(d), Ghost(c), &args[1]);
    let preferred = noun_exec(arena, pl, join, Ghost(d), Ghost(c), &args[2]);
    let none = match t::atom_of(arena, &args[3]) {
        Some(a) => t::is(&a, "none"),
        None => false,
    };
    let alternative = if none {
        h::fixed("none named")
    } else {
        noun_exec(arena, pl, join, Ghost(d), Ghost(c), &args[3])
    };
    let ghost av = a@;
    let ghost bv = b@;
    let ghost cv = stated@;
    let ghost dv = event@;
    let ghost ev = preferred@;
    let ghost fv = alternative@;
    let mut cells: Vec<EPage> = Vec::new();
    cells.push(h::cell(a));
    cells.push(h::cell(b));
    cells.push(h::cell(stated));
    cells.push(h::cell(event));
    cells.push(h::cell(preferred));
    cells.push(h::cell(alternative));
    proof {
        assert(h::pages(cells@) =~= seq![
            u::cell(av),
            u::cell(bv),
            u::cell(cv),
            u::cell(dv),
            u::cell(ev),
            u::cell(fv),
        ]);
        assert(h::pages(cells@) =~= preference_cells(pl@, d, s, c, models(args@), part as int));
        preference_unfold(pl@, d, s, c, name@, models(args@), part as int);
    }
    Some(h::row(&cells))
}

pub fn strength_rows_exec(pl: &[u8], grades: bool) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::strength_rows(pl@, grades),
{
    crate::k5_compiled::rows_exec(
        pl,
        if grades {
            1
        } else {
            2
        },
    )
}

pub fn strength_section_exec(pl: &[u8], grades: bool) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::strength_section(pl@, grades),
{
    hide(u::lit);
    let rows = strength_rows_exec(pl, grades);
    if rows.len() == 0 {
        return crate::k5_compiled::no_rows();
    }
    let mut out: Vec<EPage> = Vec::new();
    out.push(h::fixed("<section>"));
    out.push(
        h::fixed(
            if grades {
                "<h3>Grade as compiled</h3>"
            } else {
                "<h3>Preferences as compiled</h3>"
            },
        ),
    );
    out.push(
        h::fixed(
            if grades {
                "<p>Each row is one grade that the compiler read from this document. A grade applies to every statement of the document.</p>"
            } else {
                "<p>Each row is one preference that the compiler read from the ACE text. A preference names the option that the source prefers. It does not forbid the other option.</p>"
            },
        ),
    );
    out.push(h::fixed("<table>"));
    out.push(
        h::fixed(
            if grades {
                "<thead><tr><th>Measure</th><th>Value</th><th>Label in the source</th></tr></thead>"
            } else {
                "<thead><tr><th>Sentence</th><th>Part</th><th>Stated as</th><th>Action</th><th>Preferred</th><th>Instead of</th></tr></thead>"
            },
        ),
    );
    out.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&rows)), h::fixed("</tbody>")));
    out.push(h::fixed("</table>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= u::strength_section(pl@, grades));
    }
    out
}

} // verus!
