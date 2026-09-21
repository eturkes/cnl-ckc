#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_drs::{box_parts, modal};
use crate::m6_expand::one_term;
use crate::m6_query_drs::strip_anchor;
use crate::m6_refs::contains;
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub struct Source {
    pub r: T,
    pub noun: T,
    pub class: T,
}

impl View for Source {
    type V = (Term, Term, Term);

    open spec fn view(&self) -> Self::V {
        (self.r@, self.noun@, self.class@)
    }
}

pub open spec fn source_valid(nodes: Seq<ENode>, s: &Source) -> bool {
    valid(nodes, &s.r) && valid(nodes, &s.noun) && valid(nodes, &s.class)
}

pub open spec fn sources_valid(nodes: Seq<ENode>, ss: Seq<Source>) -> bool {
    forall|i: int| 0 <= i < ss.len() ==> #[trigger] source_valid(nodes, &ss[i])
}

pub open spec fn source_models(ss: Seq<Source>) -> Seq<(Term, Term, Term)> {
    ss.map_values(|s: Source| s@)
}

pub proof fn sources_prefix(before: Seq<ENode>, after: Seq<ENode>, ss: Seq<Source>)
    requires
        before.is_prefix_of(after),
        sources_valid(before, ss),
    ensures
        sources_valid(after, ss),
{
    assert forall|i: int| 0 <= i < ss.len() implies #[trigger] source_valid(after, &ss[i]) by {
        assert(source_valid(before, &ss[i]));
        prefix(before, after, &ss[i].r);
        prefix(before, after, &ss[i].noun);
        prefix(before, after, &ss[i].class);
    }
}

pub fn copy_source(s: &Source) -> (out: Source)
    ensures
        out@ == s@,
        out.r.root == s.r.root,
        out.noun.root == s.noun.root,
        out.class.root == s.class.root,
{
    Source { r: s.r.cp(), noun: s.noun.cp(), class: s.class.cp() }
}

pub fn copy_sources(arena: &ETermArena, ss: &Vec<Source>) -> (out: Vec<Source>)
    requires
        arena_ok(arena),
        sources_valid(arena.nodes@, ss@),
    ensures
        sources_valid(arena.nodes@, out@),
        source_models(out@) == source_models(ss@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < ss.len()
        invariant
            arena_ok(arena),
            sources_valid(arena.nodes@, ss@),
            sources_valid(arena.nodes@, out@),
            i <= ss.len(),
            out.len() == i,
            source_models(out@) == source_models(ss@).take(i as int),
        decreases ss.len() - i,
    {
        proof {
            assert(source_valid(arena.nodes@, &ss@[i as int]));
        }
        let next = copy_source(&ss[i]);
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] source_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(source_models(out@) == source_models(ss@).take(i as int + 1), j => { if j < before.len() { assert(out@[j] == before[j]); assert(source_models(before)[j] == source_models(ss@)[j]); } else { assert(j == i); assert(out@[j] == next); } });
        }
        i += 1;
    }
    out
}

pub fn join_sources(arena: &ETermArena, mut a: Vec<Source>, mut b: Vec<Source>) -> (out: Vec<
    Source,
>)
    requires
        arena_ok(arena),
        sources_valid(arena.nodes@, a@),
        sources_valid(arena.nodes@, b@),
    ensures
        sources_valid(arena.nodes@, out@),
        source_models(out@) == source_models(a@) + source_models(b@),
{
    let ghost first = a@;
    let ghost last = b@;
    a.append(&mut b);
    proof {
        assert_seqs_equal!(source_models(a@) == source_models(first) + source_models(last));
        assert forall|i: int| 0 <= i < a.len() implies #[trigger] source_valid(
            arena.nodes@,
            &a@[i],
        ) by {
            if i < first.len() {
                assert(a@[i] == first[i]);
            } else {
                assert(a@[i] == last[i - first.len()]);
            }
        }
    }
    a
}

pub fn box_sources(arena: &ETermArena, l: &T) -> (out: Vec<Source>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, l),
    ensures
        sources_valid(arena.nodes@, out@),
        source_models(out@) == spec::box_sources(l@),
    decreases crate::k2_engine::term_size(l@),
{
    let mut out = Vec::new();
    proof {
        reveal_with_fuel(spec::box_sources, 1);
        reveal_strlit("[|]");
        reveal(v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::Cons) == v1text::cons_name());
    }
    if let Some((name, args)) = parts(arena, l) {
        if has_name(&name, &Sym::Cons) && args.len() == 2 {
            let inner = strip_anchor(arena, &args[0]);
            proof {
                crate::m6_drs::child_size(l@, 1);
            }
            let rest = box_sources(arena, &args[1]);
            if is_comp(arena, &inner, &Sym::Object, 6) {
                let a = crate::m6_term::args(arena, &inner);
                out.push(Source { r: a[0].cp(), noun: a[1].cp(), class: a[2].cp() });
                proof {
                    assert_seqs_equal!(source_models(out@) == seq![(a[0]@, a[1]@, a[2]@)]);
                }
                return join_sources(arena, out, rest);
            }
            return rest;
        }
    }
    proof {
        assert_seqs_equal!(source_models(out@) == Seq::<(Term, Term, Term)>::empty());
    }
    out
}

pub struct Marker {
    pub r: T,
    pub tag: T,
    pub sources: Vec<Source>,
}

impl View for Marker {
    type V = spec::Marker;

    open spec fn view(&self) -> Self::V {
        spec::Marker { r: self.r@, tag: self.tag@, sources: source_models(self.sources@) }
    }
}

pub open spec fn marker_valid(nodes: Seq<ENode>, m: &Marker) -> bool {
    valid(nodes, &m.r) && valid(nodes, &m.tag) && sources_valid(nodes, m.sources@)
}

pub open spec fn markers_valid(nodes: Seq<ENode>, ms: Seq<Marker>) -> bool {
    forall|i: int| 0 <= i < ms.len() ==> #[trigger] marker_valid(nodes, &ms[i])
}

pub open spec fn marker_models(ms: Seq<Marker>) -> Seq<spec::Marker> {
    ms.map_values(|m: Marker| m@)
}

pub proof fn markers_prefix(before: Seq<ENode>, after: Seq<ENode>, ms: Seq<Marker>)
    requires
        before.is_prefix_of(after),
        markers_valid(before, ms),
    ensures
        markers_valid(after, ms),
{
    assert forall|i: int| 0 <= i < ms.len() implies #[trigger] marker_valid(after, &ms[i]) by {
        assert(marker_valid(before, &ms[i]));
        prefix(before, after, &ms[i].r);
        prefix(before, after, &ms[i].tag);
        sources_prefix(before, after, ms[i].sources@);
    }
}

pub fn join_markers(arena: &ETermArena, mut a: Vec<Marker>, mut b: Vec<Marker>) -> (out: Vec<
    Marker,
>)
    requires
        arena_ok(arena),
        markers_valid(arena.nodes@, a@),
        markers_valid(arena.nodes@, b@),
    ensures
        markers_valid(arena.nodes@, out@),
        marker_models(out@) == marker_models(a@) + marker_models(b@),
{
    let ghost first = a@;
    let ghost last = b@;
    a.append(&mut b);
    proof {
        assert_seqs_equal!(marker_models(a@) == marker_models(first) + marker_models(last));
        assert forall|i: int| 0 <= i < a.len() implies #[trigger] marker_valid(
            arena.nodes@,
            &a@[i],
        ) by {
            if i < first.len() {
                assert(a@[i] == first[i]);
            } else {
                assert(a@[i] == last[i - first.len()]);
            }
        }
    }
    a
}

pub fn box_markers(arena: &ETermArena, b: &T) -> (out: Vec<Marker>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, b),
    ensures
        markers_valid(arena.nodes@, out@),
        marker_models(out@) == spec::box_markers(b@),
    decreases crate::k2_engine::term_size(b@), 0int,
{
    proof {
        reveal_with_fuel(spec::box_markers, 1);
    }
    match box_parts(arena, b) {
        None => {
            let out = Vec::new();
            proof {
                assert_seqs_equal!(marker_models(out@) == Seq::<spec::Marker>::empty());
            }
            out
        },
        Some(bp) => {
            let sources = box_sources(arena, &bp.conds);
            proof {
                crate::m6_drs::child_size(b@, 1);
            }
            conds_markers(arena, &bp.conds, &sources)
        },
    }
}

pub fn conds_markers(arena: &ETermArena, l: &T, sources: &Vec<Source>) -> (out: Vec<Marker>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, l),
        sources_valid(arena.nodes@, sources@),
    ensures
        markers_valid(arena.nodes@, out@),
        marker_models(out@) == spec::conds_markers(l@, source_models(sources@)),
    decreases crate::k2_engine::term_size(l@), 1int,
{
    let mut head = Vec::new();
    proof {
        reveal_with_fuel(spec::conds_markers, 1);
        reveal_strlit("[|]");
        reveal(v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::Cons) == v1text::cons_name());
        assert_seqs_equal!(marker_models(head@) == Seq::<spec::Marker>::empty());
    }
    if let Some((name, a)) = parts(arena, l) {
        if has_name(&name, &Sym::Cons) && a.len() == 2 {
            let inner = strip_anchor(arena, &a[0]);
            proof {
                crate::m6_drs::child_size(l@, 0);
                crate::m6_drs::child_size(l@, 1);
            }
            if is_comp(arena, &inner, &Sym::Query, 2) {
                let args = crate::m6_term::args(arena, &inner);
                let m = Marker {
                    r: args[0].cp(),
                    tag: args[1].cp(),
                    sources: copy_sources(arena, sources),
                };
                head.push(m);
                proof {
                    assert_seqs_equal!(marker_models(head@) == seq![m@]);
                }
            } else if let Some((name, args)) = parts(arena, &inner) {
                if modal(&name) && args.len() == 1 {
                    proof {
                        crate::m6_drs::child_size(inner@, 0);
                    }
                    head = box_markers(arena, &args[0]);
                }
            }
            let rest = conds_markers(arena, &a[1], sources);
            return join_markers(arena, head, rest);
        }
    }
    head
}

pub proof fn sources_step(ss: Seq<(Term, Term, Term)>, s: (Term, Term, Term), r: Term)
    ensures
        spec::sources_of(ss.push(s), r) == if s.0 == r {
            spec::sources_of(ss, r).push(s)
        } else {
            spec::sources_of(ss, r)
        },
{
    reveal(Seq::filter);
    reveal(spec::sources_of);
    assert(ss.push(s).drop_last() == ss);
    assert(ss.push(s).last() == s);
}

pub fn sources_of(arena: &ETermArena, ss: &Vec<Source>, r: &T) -> (out: Vec<Source>)
    requires
        arena_ok(arena),
        sources_valid(arena.nodes@, ss@),
        valid(arena.nodes@, r),
    ensures
        sources_valid(arena.nodes@, out@),
        source_models(out@) == spec::sources_of(source_models(ss@), r@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        reveal(Seq::filter);
        reveal(spec::sources_of);
    }
    while i < ss.len()
        invariant
            arena_ok(arena),
            sources_valid(arena.nodes@, ss@),
            valid(arena.nodes@, r),
            sources_valid(arena.nodes@, out@),
            i <= ss.len(),
            source_models(out@) == spec::sources_of(source_models(ss@).take(i as int), r@),
        decreases ss.len() - i,
    {
        proof {
            assert(source_valid(arena.nodes@, &ss@[i as int]));
            assert(source_models(ss@).take(i as int + 1) == source_models(ss@).take(i as int).push(
                source_models(ss@)[i as int],
            ));
            sources_step(source_models(ss@).take(i as int), source_models(ss@)[i as int], r@);
        }
        if crate::m6_term::equal(arena, &ss[i].r, r) {
            let next = copy_source(&ss[i]);
            let ghost before = out@;
            out.push(next);
            proof {
                assert_seqs_equal!(source_models(out@) == source_models(before).push(next@));
                assert forall|j: int| 0 <= j < out.len() implies #[trigger] source_valid(
                    arena.nodes@,
                    &out@[j],
                ) by {
                    if j < before.len() {
                        assert(out@[j] == before[j]);
                    }
                }
            }
        }
        i += 1;
    }
    proof {
        assert(source_models(ss@).take(i as int) == source_models(ss@));
    }
    out
}

pub fn marker_desc(arena: &mut ETermArena, m: &Marker) -> (out: Option<T>)
    requires
        arena_ok(old(arena)),
        marker_valid(old(arena).nodes@, m),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(t) ==> valid(final(arena).nodes@, &t),
        crate::m6_drs::term_opt(out) == spec::marker_desc(m@),
{
    let ss = sources_of(arena, &m.sources, &m.r);
    if ss.len() == 1 {
        proof {
            assert(source_valid(arena.nodes@, &ss@[0]));
        }
        return Some(c2(arena, &Sym::Noun, &ss[0].noun, &ss[0].class));
    }
    if ss.len() > 1 {
        return None;
    }
    if is_atom(arena, &m.tag, &Sym::Who) || is_atom(arena, &m.tag, &Sym::What) {
        Some(c1(arena, &Sym::Wh, &m.tag))
    } else {
        None
    }
}

pub fn answers_from(arena: &mut ETermArena, ms: &Vec<Marker>, i: usize, seen: &Vec<T>) -> (out:
    Option<Vec<T>>)
    requires
        arena_ok(old(arena)),
        markers_valid(old(arena).nodes@, ms@),
        valid_all(old(arena).nodes@, seen@),
        i <= ms.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(ts) ==> valid_all(final(arena).nodes@, ts@),
        option_models(out) == spec::answers_of(marker_models(ms@).skip(i as int), models(seen@)),
    decreases ms.len() - i,
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::answers_of, 1);
    }
    if i == ms.len() {
        let out = Vec::new();
        proof {
            assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
        }
        return Some(out);
    }
    proof {
        assert(marker_valid(arena.nodes@, &ms@[i as int]));
        assert(marker_models(ms@).skip(i as int).drop_first() == marker_models(ms@).skip(
            i as int + 1,
        ));
    }
    if !is_var(arena, &ms[i].r) || contains(arena, seen, &ms[i].r) {
        return None;
    }
    let desc = match marker_desc(arena, &ms[i]) {
        Some(d) => d,
        None => return None,
    };
    let ghost n1 = arena.nodes@;
    proof {
        markers_prefix(start, n1, ms@);
        prefix_all(start, n1, seen@);
        assert(marker_valid(n1, &ms@[i as int]));
    }
    let mut next_seen = copy(seen);
    let r = ms[i].r.cp();
    proof {
        extend(n1, next_seen@, r);
    }
    next_seen.push(r);
    let head = c2(arena, &Sym::Answer, &ms[i].r, &desc);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
        markers_prefix(n1, n2, ms@);
        prefix_all(n1, n2, next_seen@);
    }
    let rest = answers_from(arena, ms, i + 1, &next_seen);
    proof {
        crate::k2_load::prefix_chain(start, n2, arena.nodes@);
    }
    match rest {
        None => None,
        Some(ts) => {
            proof {
                prefix(n2, arena.nodes@, &head);
            }
            let first = one_term(arena, head);
            Some(concat(&first, &ts))
        },
    }
}

pub proof fn answers_shape(ms: Seq<spec::Marker>, seen: Seq<Term>)
    ensures
        spec::answers_of(ms, seen) matches Some(ts) ==> (forall|i: int|
            0 <= i < ts.len() ==> spec::is_comp(#[trigger] ts[i], "answer"@, 2)),
    decreases ms.len(),
{
    reveal_with_fuel(spec::answers_of, 1);
    if ms.len() > 0 {
        answers_shape(ms.drop_first(), seen.push(ms[0].r));
        if let Some(ts) = spec::answers_of(ms, seen) {
            assert forall|i: int| 0 <= i < ts.len() implies spec::is_comp(
                #[trigger] ts[i],
                "answer"@,
                2,
            ) by {
                if i > 0 {
                    if let Some(rest) = spec::answers_of(ms.drop_first(), seen.push(ms[0].r)) {
                        assert(ts[i] == rest[i - 1]);
                    }
                }
            }
        }
    }
}

} // verus!
