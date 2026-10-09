// m7t D10: the reviewer page's "Timing as compiled" table — exec mirror of
// ckc_spec::ui::timing_section over the K1-parsed v2 document.
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENodeKind, ETermArena};
use crate::k5_html as h;
#[cfg(verus_keep_ghost)]
use crate::m6_model::body_models;
use crate::m6_model::{Body, Clause};
use crate::m6_term::T;
#[cfg(verus_keep_ghost)]
use crate::m6_term::{models, valid};
use ckc_spec::term::Term;
use ckc_spec::ui as u;
use ckc_spec::ui::{EPage, EPiece};
use ckc_spec::v1text as v;
use vstd::prelude::*;

verus! {

pub open spec fn lit_pairs(xs: Seq<(T, u8)>) -> Seq<(Term, int)> {
    xs.map_values(|p: (T, u8)| (p.0@, p.1 as int))
}

pub open spec fn lit_terms(xs: Seq<(T, u8)>) -> Seq<Term> {
    xs.map_values(|p: (T, u8)| p.0@)
}

pub open spec fn body_part(it: v::BodyItem) -> Seq<(Term, int)> {
    match it {
        v::BodyItem::Pos(l) => seq![(l, 1int)],
        v::BodyItem::Naf(gs) => gs.map_values(|g: Term| (g, 2int)),
    }
}

proof fn clause_lits_unfold(c: v::DocClause)
    ensures
        u::clause_lits(c) == seq![(c.head, 0int)] + c.body.map_values(
            |it: v::BodyItem| body_part(it),
        ).flatten(),
{
    assert(c.body.map_values(|it: v::BodyItem| body_part(it)) =~= c.body.map_values(
        |it: v::BodyItem|
            match it {
                v::BodyItem::Pos(l) => seq![(l, 1int)],
                v::BodyItem::Naf(gs) => gs.map_values(|g: Term| (g, 2int)),
            },
    ));
}

// A clause's literals in order: head (part 0), positive goals (1), NAF goals (2).
pub fn lits_of(arena: &ETermArena, c: &Clause) -> (out: Vec<(T, u8)>)
    requires
        arena_ok(arena),
        crate::m6_model::clause_valid(arena.nodes@, c),
    ensures
        lit_pairs(out@) == u::clause_lits(c@),
        forall|i: int|
            0 <= i < out@.len() ==> valid(arena.nodes@, &(#[trigger] out@[i]).0) && out@[i].1 <= 2,
{
    proof {
        clause_lits_unfold(c@);
        reveal(crate::m6_model::clause_valid);
    }
    let ghost bm = body_models(c.body@);
    let ghost f = |it: v::BodyItem| body_part(it);
    let mut out: Vec<(T, u8)> = Vec::new();
    out.push((c.head.cp(), 0u8));
    let mut k = 0usize;
    proof {
        assert(bm.take(0).map_values(f) =~= Seq::<Seq<(Term, int)>>::empty());
        reveal_with_fuel(Seq::<_>::flatten, 1);
        assert(lit_pairs(out@) =~= seq![(c.head@, 0int)] + bm.take(0).map_values(f).flatten());
    }
    while k < c.body.len()
        invariant
            arena_ok(arena),
            crate::m6_model::clause_valid(arena.nodes@, c),
            k <= c.body@.len(),
            bm == body_models(c.body@),
            bm.len() == c.body@.len(),
            f == (|it: v::BodyItem| body_part(it)),
            lit_pairs(out@) == seq![(c.head@, 0int)] + bm.take(k as int).map_values(f).flatten(),
            forall|i: int|
                0 <= i < out@.len() ==> valid(arena.nodes@, &(#[trigger] out@[i]).0) && out@[i].1
                    <= 2,
        decreases c.body@.len() - k,
    {
        let ghost before = out@;
        proof {
            assert(bm[k as int] == c.body@[k as int]@);
            assert(crate::m6_model::body_valid(arena.nodes@, &c.body@[k as int]));
        }
        match &c.body[k] {
            Body::Pos(t) => {
                out.push((t.cp(), 1u8));
                proof {
                    assert(f(bm[k as int]) =~= seq![(t@, 1int)]);
                    assert(lit_pairs(out@) =~= lit_pairs(before) + f(bm[k as int]));
                    assert forall|i: int| 0 <= i < out@.len() implies valid(
                        arena.nodes@,
                        &(#[trigger] out@[i]).0,
                    ) && out@[i].1 <= 2 by {
                        if i < before.len() {
                            assert(out@[i] == before[i]);
                        }
                    }
                }
            },
            Body::Naf(ts) => {
                let mut j = 0usize;
                while j < ts.len()
                    invariant
                        j <= ts@.len(),
                        before.len() <= out@.len(),
                        lit_pairs(out@) == lit_pairs(before) + models(ts@).take(
                            j as int,
                        ).map_values(|g: Term| (g, 2int)),
                        forall|i: int|
                            0 <= i < out@.len() ==> valid(arena.nodes@, &(#[trigger] out@[i]).0)
                                && out@[i].1 <= 2,
                        crate::m6_term::valid_all(arena.nodes@, ts@),
                    decreases ts@.len() - j,
                {
                    let ghost prev = out@;
                    out.push((ts[j].cp(), 2u8));
                    proof {
                        assert(valid(arena.nodes@, &ts@[j as int]));
                        assert forall|i: int| 0 <= i < out@.len() implies valid(
                            arena.nodes@,
                            &(#[trigger] out@[i]).0,
                        ) && out@[i].1 <= 2 by {
                            if i < prev.len() {
                                assert(out@[i] == prev[i]);
                            }
                        }
                        let g2 = |g: Term| (g, 2int);
                        assert(models(ts@).take(j + 1) =~= models(ts@).take(j as int).push(
                            ts@[j as int]@,
                        ));
                        assert(models(ts@).take(j + 1).map_values(g2) =~= models(ts@).take(
                            j as int,
                        ).map_values(g2).push((ts@[j as int]@, 2int)));
                        assert(lit_pairs(out@) =~= lit_pairs(prev).push((ts@[j as int]@, 2int)));
                        assert(lit_pairs(out@) =~= lit_pairs(before) + models(ts@).take(
                            j + 1,
                        ).map_values(g2));
                    }
                    j += 1;
                }
                proof {
                    assert(models(ts@).take(j as int) =~= models(ts@));
                    assert(f(bm[k as int]) =~= models(ts@).map_values(|g: Term| (g, 2int)));
                }
            },
        }
        proof {
            assert(bm.take(k + 1) =~= bm.take(k as int).push(bm[k as int]));
            assert(bm.take(k + 1).map_values(f) =~= bm.take(k as int).map_values(f).push(
                f(bm[k as int]),
            ));
            bm.take(k as int).map_values(f).lemma_flatten_push(f(bm[k as int]));
            assert(lit_pairs(out@) =~= lit_pairs(before) + f(bm[k as int]));
            assert(lit_pairs(out@) =~= seq![(c.head@, 0int)] + bm.take(k + 1).map_values(
                f,
            ).flatten());
        }
        k += 1;
    }
    proof {
        assert(bm.take(k as int) =~= bm);
    }
    out
}

pub open spec fn heads(cs: Seq<v::DocClause>) -> Seq<Term> {
    cs.map_values(|c: v::DocClause| c.head)
}

pub fn heads_of(cs: &Vec<Clause>) -> (out: Vec<T>)
    ensures
        models(out@) == heads(crate::m6_model::clause_models(cs@)),
        forall|i: int| 0 <= i < out@.len() ==> (#[trigger] out@[i]) == cs@[i].head,
{
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    while i < cs.len()
        invariant
            i <= cs@.len(),
            out@.len() == i,
            forall|j: int| 0 <= j < i ==> (#[trigger] out@[j]) == cs@[j].head,
        decreases cs@.len() - i,
    {
        out.push(cs[i].head.cp());
        i += 1;
    }
    proof {
        assert(models(out@) =~= heads(crate::m6_model::clause_models(cs@)));
    }
    out
}

// u::first_with with the name compared as bytes (name@ == u::lit(nc)).
pub fn first_with_exec(
    arena: &ETermArena,
    ts: &Vec<T>,
    name: &[u8],
    Ghost(nc): Ghost<Seq<char>>,
    arity: usize,
    at: usize,
    key: &T,
) -> (out: Option<Vec<T>>)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, ts@),
        valid(arena.nodes@, key),
        name@ == u::lit(nc),
        at < arity,
    ensures
        match out {
            Some(a) => crate::m6_term::valid_all(arena.nodes@, a@) && a@.len() == arity
                && u::first_with(models(ts@), nc, arity as nat, at as int, key@) == Some(
                models(a@),
            ),
            None => u::first_with(models(ts@), nc, arity as nat, at as int, key@) is None,
        },
{
    let ghost ms = models(ts@);
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
    }
    while i < ts.len()
        invariant
            i <= ts@.len(),
            arena_ok(arena),
            crate::m6_term::valid_all(arena.nodes@, ts@),
            valid(arena.nodes@, key),
            name@ == u::lit(nc),
            at < arity,
            ms == models(ts@),
            ms.len() == ts@.len(),
            u::first_with(ms, nc, arity as nat, at as int, key@) == u::first_with(
                ms.skip(i as int),
                nc,
                arity as nat,
                at as int,
                key@,
            ),
        decreases ts@.len() - i,
    {
        proof {
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
            assert(crate::m6_term::valid(arena.nodes@, &ts@[i as int]));
        }
        match crate::m6_term::parts(arena, &ts[i]) {
            Some((n, a)) => {
                if crate::k4_bytes::eq(&n, name) && a.len() == arity {
                    proof {
                        assert(models(a@)[at as int] == a@[at as int]@);
                    }
                    if crate::m6_term::equal(arena, &a[at], key) {
                        return Some(a);
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

// u::pl_word: the word as visible text when its bytes occur in the pl.
pub fn pl_word_exec(pl: &[u8], w: &[u8]) -> (out: EPage)
    ensures
        out@ == u::pl_word(pl@, w@),
{
    let nf: &str = "not stated";
    if w.len() == 0 || w.len() > pl.len() {
        proof {
            if w@.len() > 0 {
                assert(ckc_spec::check::first_sub(pl@, w@, 0) == pl@.len());
            }
        }
        return h::fixed(nf);
    }
    let last = pl.len() - w.len();
    let mut i = 0usize;
    while i <= last
        invariant
            0 < w@.len() <= pl@.len(),
            last == pl@.len() - w@.len(),
            i <= last + 1,
            ckc_spec::check::first_sub(pl@, w@, 0) == ckc_spec::check::first_sub(pl@, w@, i as nat),
        decreases last + 1 - i,
    {
        proof {
            assert(i + w@.len() <= pl@.len());
        }
        let end = pl.len() - (last - i);
        let sub = vstd::slice::slice_subrange(pl, i, end);
        if crate::k4_bytes::eq(sub, w) {
            proof {
                assert(pl@.subrange(i as int, i + w@.len()) =~= sub@);
            }
            return h::text(w);
        }
        proof {
            assert(pl@.subrange(i as int, i + w@.len()) =~= sub@);
        }
        i += 1;
    }
    h::fixed(nf)
}

pub open spec fn opt_page(o: Option<EPage>) -> Option<u::Html> {
    match o {
        Some(p) => Some(p@),
        None => None,
    }
}

pub fn atom_of(arena: &ETermArena, t: &T) -> (out: Option<Vec<u8>>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        match out {
            Some(a) => t@ == Term::Atom(a@),
            None => !(t@ is Atom),
        },
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, t.root as int));
        reveal(crate::k2_term::node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Atom { name } => Some(name.clone()),
        _ => None,
    }
}

pub fn is(a: &[u8], s: &str) -> (out: bool)
    ensures
        out == (a@ == u::lit(s@)),
{
    let b = crate::k5_bytes::literal(s);
    crate::k4_bytes::eq(a, &b)
}

pub fn cmp_exec(arena: &ETermArena, c: &T) -> (out: Option<EPage>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, c),
    ensures
        opt_page(out) == u::cmp_html(c@),
{
    match atom_of(arena, c) {
        None => None,
        Some(a) => {
            if is(&a, "eq") {
                Some(EPage { parts: Vec::new() })
            } else if is(&a, "exactly") {
                Some(h::fixed("exactly "))
            } else if is(&a, "geq") {
                Some(h::fixed("at least "))
            } else if is(&a, "greater") {
                Some(h::fixed("more than "))
            } else if is(&a, "leq") {
                Some(h::fixed("at most "))
            } else if is(&a, "less") {
                Some(h::fixed("less than "))
            } else if is(&a, "about") {
                Some(h::fixed("about "))
            } else {
                None
            }
        },
    }
}

pub fn unit_exec(un: &[u8], one: bool) -> (out: Option<EPage>)
    ensures
        opt_page(out) == u::unit_html(un@, one),
{
    if is(un, "second") {
        Some(
            if one {
                h::fixed(" second")
            } else {
                h::fixed(" seconds")
            },
        )
    } else if is(un, "minute") {
        Some(
            if one {
                h::fixed(" minute")
            } else {
                h::fixed(" minutes")
            },
        )
    } else if is(un, "hour") {
        Some(
            if one {
                h::fixed(" hour")
            } else {
                h::fixed(" hours")
            },
        )
    } else if is(un, "day") {
        Some(
            if one {
                h::fixed(" day")
            } else {
                h::fixed(" days")
            },
        )
    } else if is(un, "week") {
        Some(
            if one {
                h::fixed(" week")
            } else {
                h::fixed(" weeks")
            },
        )
    } else if is(un, "month") {
        Some(
            if one {
                h::fixed(" month")
            } else {
                h::fixed(" months")
            },
        )
    } else if is(un, "year") {
        Some(
            if one {
                h::fixed(" year")
            } else {
                h::fixed(" years")
            },
        )
    } else {
        None
    }
}

// u::bound_html over the clause's join terms (`join` = lits ++ document heads).
pub fn bound_exec(
    arena: &ETermArena,
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    q: &T,
    un: &T,
) -> (out: Option<EPage>)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, q),
        valid(arena.nodes@, un),
    ensures
        opt_page(out) == u::bound_html(d, c, q@, un@),
{
    let name = crate::k5_bytes::literal("guideline_cardinality");
    let args = match first_with_exec(arena, join, &name, Ghost("guideline_cardinality"@), 5, 1, q) {
        None => return None,
        Some(a) => a,
    };
    proof {
        assert(models(args@)[3] == args@[3]@);
        assert(models(args@)[4] == args@[4]@);
        assert(valid(arena.nodes@, &args@[3]));
        assert(valid(arena.nodes@, &args@[4]));
    }
    let cmp = match cmp_exec(arena, &args[3]) {
        None => return None,
        Some(p) => p,
    };
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, args@[4].root as int));
        reveal(crate::k2_term::node_ok);
    }
    let (magnitude, negative) = match &arena.nodes[args[4].root].kind {
        ENodeKind::Int { magnitude, negative, .. } => (magnitude.clone(), *negative),
        _ => return None,
    };
    if negative {
        return None;
    }
    let one = magnitude.len() == 1 && magnitude[0] == 0x31;
    proof {
        if let Term::Int(n) = args@[4]@ {
            reveal_with_fuel(v::udec_bytes, 2);
            reveal(v::digit_byte);
            assert(v::udec_bytes(1) =~= seq![0x31u8]);
            if one {
                assert(magnitude@ =~= v::udec_bytes(1));
                crate::v1_term_impl::udec_bytes_injective(n as nat, 1);
            }
            if n == 1 {
                assert(magnitude@ == v::udec_bytes(1));
            }
        }
    }
    let ua = match atom_of(arena, un) {
        Some(a) => a,
        None => Vec::new(),
    };
    let w = match unit_exec(&ua, one) {
        None => return None,
        Some(w) => w,
    };
    Some(h::cat(h::cat(cmp, h::text(&magnitude)), w))
}

// (magnitude, one) of a nonnegative integer literal (`one` = it is 1), else None.
fn count_parts(arena: &ETermArena, t: &T) -> (out: Option<(Vec<u8>, bool)>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        match out {
            Some((m, one)) => match t@ {
                Term::Int(n) => 0 <= n && m@ == v::udec_bytes(n as nat) && one == (n == 1),
                _ => false,
            },
            None => !(t@ matches Term::Int(n) && 0 <= n),
        },
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, t.root as int));
        reveal(crate::k2_term::node_ok);
    }
    let (magnitude, negative) = match &arena.nodes[t.root].kind {
        ENodeKind::Int { magnitude, negative, .. } => (magnitude.clone(), *negative),
        _ => return None,
    };
    if negative {
        return None;
    }
    let one = magnitude.len() == 1 && magnitude[0] == 0x31;
    proof {
        if let Term::Int(n) = t@ {
            reveal_with_fuel(v::udec_bytes, 2);
            reveal(v::digit_byte);
            assert(v::udec_bytes(1) =~= seq![0x31u8]);
            if one {
                assert(magnitude@ =~= v::udec_bytes(1));
                crate::v1_term_impl::udec_bytes_injective(n as nat, 1);
            }
            if n == 1 {
                assert(magnitude@ == v::udec_bytes(1));
            }
        }
    }
    Some((magnitude, one))
}

// u::range_bound_html: a range's low end renders `a minimum of N to M <unit>`.
pub fn range_bound_exec(
    arena: &ETermArena,
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    q: &T,
    un: &T,
) -> (out: Option<EPage>)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, q),
        valid(arena.nodes@, un),
    ensures
        opt_page(out) == u::range_bound_html(d, c, q@, un@),
{
    let rname = crate::k5_bytes::literal("guideline_range");
    let r = match first_with_exec(arena, join, &rname, Ghost("guideline_range"@), 3, 1, q) {
        None => return bound_exec(arena, join, Ghost(d), Ghost(c), q, un),
        Some(a) => a,
    };
    proof {
        assert(models(r@)[2] == r@[2]@);
        assert(valid(arena.nodes@, &r@[2]));
    }
    let name = crate::k5_bytes::literal("guideline_cardinality");
    let lo = match first_with_exec(arena, join, &name, Ghost("guideline_cardinality"@), 5, 1, q) {
        None => return None,
        Some(a) => a,
    };
    let hi = match first_with_exec(
        arena,
        join,
        &name,
        Ghost("guideline_cardinality"@),
        5,
        1,
        &r[2],
    ) {
        None => return None,
        Some(a) => a,
    };
    proof {
        assert(models(lo@)[3] == lo@[3]@);
        assert(models(lo@)[4] == lo@[4]@);
        assert(models(hi@)[3] == hi@[3]@);
        assert(models(hi@)[4] == hi@[4]@);
        assert(valid(arena.nodes@, &lo@[3]));
        assert(valid(arena.nodes@, &lo@[4]));
        assert(valid(arena.nodes@, &hi@[3]));
        assert(valid(arena.nodes@, &hi@[4]));
    }
    let (ml, _) = match count_parts(arena, &lo[4]) {
        None => return None,
        Some(p) => p,
    };
    let (mh, one) = match count_parts(arena, &hi[4]) {
        None => return None,
        Some(p) => p,
    };
    let geq = match atom_of(arena, &lo[3]) {
        Some(a) => is(&a, "geq"),
        None => false,
    };
    let eq = match atom_of(arena, &hi[3]) {
        Some(a) => is(&a, "eq"),
        None => false,
    };
    if !(geq && eq) {
        return None;
    }
    let ua = match atom_of(arena, un) {
        Some(a) => a,
        None => Vec::new(),
    };
    let w = match unit_exec(&ua, one) {
        None => return None,
        Some(w) => w,
    };
    Some(
        h::cat(
            h::cat(
                h::cat(h::cat(h::fixed("a minimum of "), h::text(&ml)), h::fixed(" to ")),
                h::text(&mh),
            ),
            w,
        ),
    )
}

pub fn timing_html_exec(role: &[u8], b: Option<EPage>) -> (out: EPage)
    ensures
        out@ == u::timing_html(role@, opt_page(b)),
{
    match b {
        None => h::fixed("not stated"),
        Some(p) => {
            if is(role, "duration") {
                h::cat(h::fixed("lasts "), p)
            } else if is(role, "after") {
                h::cat(p, h::fixed(" after"))
            } else if is(role, "before") {
                h::cat(p, h::fixed(" before"))
            } else if is(role, "within") {
                h::cat(h::cat(h::fixed("within "), p), h::fixed(" of"))
            } else if is(role, "recurrence") {
                h::cat(h::cat(h::fixed("repeats "), p), h::fixed(" apart"))
            } else {
                h::fixed("not stated")
            }
        },
    }
}

pub fn joined_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    name: &str,
    arity: usize,
    key: &T,
    at: usize,
) -> (out: EPage)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, key),
        1 < arity,
        at < arity,
    ensures
        out@ == u::joined_word(pl@, d, c, name@, arity as nat, key@, at as int),
{
    let nb = crate::k5_bytes::literal(name);
    match first_with_exec(arena, join, &nb, Ghost(name@), arity, 1, key) {
        None => h::fixed("not stated"),
        Some(args) => {
            proof {
                assert(models(args@)[at as int] == args@[at as int]@);
                assert(valid(arena.nodes@, &args@[at as int]));
            }
            let w = match atom_of(arena, &args[at]) {
                Some(a) => a,
                None => Vec::new(),
            };
            pl_word_exec(pl, &w)
        },
    }
}

pub fn part_exec(part: u8) -> (out: EPage)
    requires
        part <= 2,
    ensures
        out@ == u::part_html(part as int),
{
    if part == 0 {
        h::fixed("statement")
    } else if part == 1 {
        h::fixed("condition")
    } else {
        h::fixed("excluded condition")
    }
}

proof fn not_stated_fixed()
    ensures
        u::fixed("not stated"@) == u::fixed_bytes(u::not_stated()),
{
}

// u::timing_row for literal `l` of clause `c` in bundle `s` (`ordinal` = udec_bytes(s)).
#[verifier::rlimit(50)]
pub fn row_exec(
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
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, l),
        ordinal@ == v::udec_bytes(s),
        part <= 2,
    ensures
        opt_page(out) == u::timing_row(pl@, d, s, c, l@, part as int),
{
    hide(u::joined_word);
    hide(u::bound_html);
    hide(u::timing_html);
    hide(u::part_html);
    hide(u::lit);
    hide(u::cell);
    hide(u::row);
    hide(u::text);
    hide(u::fixed_bytes);
    let (name, args) = match crate::m6_term::parts(arena, l) {
        None => return None,
        Some(x) => x,
    };
    proof {
        assert(l@ == Term::Comp(name@, models(args@)));
        assert forall|i: int| 0 <= i < args@.len() implies models(args@)[i] == args@[i]@ && valid(
            arena.nodes@,
            &args@[i],
        ) by {}
        reveal(u::timing_row);
        not_stated_fixed();
    }
    if is(&name, "guideline_interval") && args.len() == 6 {
        let role = match atom_of(arena, &args[2]) {
            Some(a) => a,
            None => Vec::new(),
        };
        let event = joined_exec(
            arena,
            pl,
            join,
            Ghost(d),
            Ghost(c),
            "guideline_event",
            3,
            &args[1],
            2,
        );
        let bound = range_bound_exec(arena, join, Ghost(d), Ghost(c), &args[3], &args[4]);
        let timing = timing_html_exec(&role, bound);
        let anchor = if is(&role, "duration") {
            let blank = EPage { parts: Vec::new() };
            proof {
                assert(blank@ =~= Seq::<u::Piece>::empty());
            }
            blank
        } else {
            let none = match atom_of(arena, &args[5]) {
                Some(a) => is(&a, "none"),
                None => false,
            };
            if none {
                h::fixed("not stated")
            } else {
                joined_exec(arena, pl, join, Ghost(d), Ghost(c), "guideline_entity", 4, &args[5], 2)
            }
        };
        let mut cells = Vec::new();
        cells.push(h::cell(h::text(ordinal)));
        cells.push(h::cell(part_exec(part)));
        cells.push(h::cell(event));
        cells.push(h::cell(timing));
        cells.push(h::cell(anchor));
        let r = h::row(&cells);
        proof {
            assert(h::pages(cells@) =~= seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(part as int)),
                u::cell(u::joined_word(pl@, d, c, "guideline_event"@, 3, args@[1]@, 2)),
                u::cell(
                    u::timing_html(
                        u::atom_name(args@[2]@),
                        u::range_bound_html(d, c, args@[3]@, args@[4]@),
                    ),
                ),
                cells@[4]@,
            ]);
        }
        Some(r)
    } else if is(&name, "guideline_recurrence") && args.len() == 4 {
        let event = joined_exec(
            arena,
            pl,
            join,
            Ghost(d),
            Ghost(c),
            "guideline_event",
            3,
            &args[1],
            2,
        );
        let bound = bound_exec(arena, join, Ghost(d), Ghost(c), &args[2], &args[3]);
        let rb = crate::k5_bytes::literal("recurrence");
        let timing = timing_html_exec(&rb, bound);
        let mut cells = Vec::new();
        cells.push(h::cell(h::text(ordinal)));
        cells.push(h::cell(part_exec(part)));
        cells.push(h::cell(event));
        cells.push(h::cell(timing));
        let blank = EPage { parts: Vec::new() };
        proof {
            assert(blank@ =~= Seq::<u::Piece>::empty());
        }
        cells.push(h::cell(blank));
        proof {
            assert(h::pages(cells@) =~= seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(part as int)),
                u::cell(u::joined_word(pl@, d, c, "guideline_event"@, 3, args@[1]@, 2)),
                u::cell(
                    u::timing_html(
                        u::lit("recurrence"@),
                        u::bound_html(d, c, args@[2]@, args@[3]@),
                    ),
                ),
                u::cell(Seq::empty()),
            ]);
        }
        Some(h::row(&cells))
    } else if is(&name, "guideline_order") && args.len() == 4 {
        Some(order_row_exec(arena, pl, join, Ghost(d), Ghost(c), ordinal, Ghost(s), &args, part))
    } else if is(&name, "guideline_frequency") && args.len() == 5 {
        Some(
            frequency_row_exec(arena, pl, join, Ghost(d), Ghost(c), ordinal, Ghost(s), &args, part),
        )
    } else if is(&name, "guideline_recurrence_window") && args.len() == 7 {
        Some(window_row_exec(arena, pl, join, Ghost(d), Ghost(c), ordinal, Ghost(s), &args, part))
    } else {
        None
    }
}

// --- q12 D11: the v3 timing rows ---
pub fn order_html_exec(role: &[u8]) -> (out: EPage)
    ensures
        out@ == u::order_html(role@),
{
    if is(role, "before") {
        h::fixed("before")
    } else if is(role, "after") {
        h::fixed("after")
    } else {
        h::fixed("not stated")
    }
}

// u::count_html: `<cmp> N` from the referent's cardinality literal.
pub fn count_exec(
    arena: &ETermArena,
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    q: &T,
) -> (out: Option<EPage>)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, q),
    ensures
        opt_page(out) == u::count_html(d, c, q@),
{
    let name = crate::k5_bytes::literal("guideline_cardinality");
    let args = match first_with_exec(arena, join, &name, Ghost("guideline_cardinality"@), 5, 1, q) {
        None => return None,
        Some(a) => a,
    };
    proof {
        assert(models(args@)[3] == args@[3]@);
        assert(models(args@)[4] == args@[4]@);
        assert(valid(arena.nodes@, &args@[3]));
        assert(valid(arena.nodes@, &args@[4]));
    }
    let cmp = match cmp_exec(arena, &args[3]) {
        None => return None,
        Some(p) => p,
    };
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, args@[4].root as int));
        reveal(crate::k2_term::node_ok);
    }
    let (magnitude, negative) = match &arena.nodes[args[4].root].kind {
        ENodeKind::Int { magnitude, negative, .. } => (magnitude.clone(), *negative),
        _ => return None,
    };
    if negative {
        return None;
    }
    Some(h::cat(cmp, h::text(&magnitude)))
}

pub fn frequency_html_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    args: &Vec<T>,
) -> (out: EPage)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        crate::m6_term::valid_all(arena.nodes@, args@),
        args.len() == 5,
    ensures
        out@ == u::frequency_html(pl@, d, c, models(args@)),
{
    proof {
        assert(valid(arena.nodes@, &args@[2]) && valid(arena.nodes@, &args@[3]) && valid(
            arena.nodes@,
            &args@[4],
        ));
    }
    match (
        count_exec(arena, join, Ghost(d), Ghost(c), &args[2]),
        bound_exec(arena, join, Ghost(d), Ghost(c), &args[3], &args[4]),
    ) {
        (Some(n), Some(w)) => {
            let noun = joined_exec(
                arena,
                pl,
                join,
                Ghost(d),
                Ghost(c),
                "guideline_entity",
                4,
                &args[2],
                2,
            );
            h::cat(
                h::cat(h::cat(h::cat(n, h::fixed(" per ")), w), h::fixed(", counted item: ")),
                noun,
            )
        },
        _ => h::fixed("not stated"),
    }
}

pub fn window_html_exec(
    arena: &ETermArena,
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    args: &Vec<T>,
) -> (out: EPage)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        crate::m6_term::valid_all(arena.nodes@, args@),
        args.len() == 7,
    ensures
        out@ == u::window_html(d, c, models(args@)),
{
    proof {
        assert(valid(arena.nodes@, &args@[2]) && valid(arena.nodes@, &args@[3]));
        assert(valid(arena.nodes@, &args@[5]) && valid(arena.nodes@, &args@[6]));
    }
    match (
        bound_exec(arena, join, Ghost(d), Ghost(c), &args[2], &args[3]),
        bound_exec(arena, join, Ghost(d), Ghost(c), &args[5], &args[6]),
    ) {
        (Some(g), Some(l)) => h::cat(
            h::cat(h::cat(h::cat(h::fixed("repeats "), g), h::fixed(" apart during ")), l),
            h::fixed(" from"),
        ),
        _ => h::fixed("not stated"),
    }
}

fn five_cells(a: EPage, b: EPage, c: EPage, d: EPage, e: EPage) -> (out: EPage)
    ensures
        out@ == u::row(seq![u::cell(a@), u::cell(b@), u::cell(c@), u::cell(d@), u::cell(e@)]),
{
    let ghost (av, bv, cv, dv, ev) = (a@, b@, c@, d@, e@);
    let mut cells = Vec::new();
    cells.push(h::cell(a));
    cells.push(h::cell(b));
    cells.push(h::cell(c));
    cells.push(h::cell(d));
    cells.push(h::cell(e));
    proof {
        assert(h::pages(cells@) =~= seq![
            u::cell(av),
            u::cell(bv),
            u::cell(cv),
            u::cell(dv),
            u::cell(ev),
        ]);
    }
    h::row(&cells)
}

pub fn order_row_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    ordinal: &[u8],
    Ghost(s): Ghost<nat>,
    args: &Vec<T>,
    part: u8,
) -> (out: EPage)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        crate::m6_term::valid_all(arena.nodes@, args@),
        args.len() == 4,
        ordinal@ == v::udec_bytes(s),
        part <= 2,
    ensures
        out@ == u::row(
            seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(part as int)),
                u::cell(u::joined_word(pl@, d, c, "guideline_event"@, 3, models(args@)[1], 2)),
                u::cell(u::order_html(u::atom_name(models(args@)[2]))),
                u::cell(u::joined_word(pl@, d, c, "guideline_entity"@, 4, models(args@)[3], 2)),
            ],
        ),
{
    proof {
        assert(valid(arena.nodes@, &args@[1]) && valid(arena.nodes@, &args@[2]) && valid(
            arena.nodes@,
            &args@[3],
        ));
    }
    let event = joined_exec(arena, pl, join, Ghost(d), Ghost(c), "guideline_event", 3, &args[1], 2);
    let role = match atom_of(arena, &args[2]) {
        Some(a) => a,
        None => Vec::new(),
    };
    let anchor = joined_exec(
        arena,
        pl,
        join,
        Ghost(d),
        Ghost(c),
        "guideline_entity",
        4,
        &args[3],
        2,
    );
    five_cells(h::text(ordinal), part_exec(part), event, order_html_exec(&role), anchor)
}

pub fn frequency_row_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    ordinal: &[u8],
    Ghost(s): Ghost<nat>,
    args: &Vec<T>,
    part: u8,
) -> (out: EPage)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        crate::m6_term::valid_all(arena.nodes@, args@),
        args.len() == 5,
        ordinal@ == v::udec_bytes(s),
        part <= 2,
    ensures
        out@ == u::row(
            seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(part as int)),
                u::cell(u::joined_word(pl@, d, c, "guideline_event"@, 3, models(args@)[1], 2)),
                u::cell(u::frequency_html(pl@, d, c, models(args@))),
                u::cell(Seq::empty()),
            ],
        ),
{
    proof {
        assert(valid(arena.nodes@, &args@[1]));
    }
    let event = joined_exec(arena, pl, join, Ghost(d), Ghost(c), "guideline_event", 3, &args[1], 2);
    let timing = frequency_html_exec(arena, pl, join, Ghost(d), Ghost(c), args);
    let blank = EPage { parts: Vec::new() };
    proof {
        assert(blank@ =~= Seq::<u::Piece>::empty());
    }
    five_cells(h::text(ordinal), part_exec(part), event, timing, blank)
}

pub fn window_row_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    ordinal: &[u8],
    Ghost(s): Ghost<nat>,
    args: &Vec<T>,
    part: u8,
) -> (out: EPage)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        crate::m6_term::valid_all(arena.nodes@, args@),
        args.len() == 7,
        ordinal@ == v::udec_bytes(s),
        part <= 2,
    ensures
        out@ == u::row(
            seq![
                u::cell(u::text(v::udec_bytes(s))),
                u::cell(u::part_html(part as int)),
                u::cell(u::joined_word(pl@, d, c, "guideline_event"@, 3, models(args@)[1], 2)),
                u::cell(u::window_html(d, c, models(args@))),
                u::cell(u::joined_word(pl@, d, c, "guideline_entity"@, 4, models(args@)[4], 2)),
            ],
        ),
{
    proof {
        assert(valid(arena.nodes@, &args@[1]) && valid(arena.nodes@, &args@[4]));
    }
    let event = joined_exec(arena, pl, join, Ghost(d), Ghost(c), "guideline_event", 3, &args[1], 2);
    let timing = window_html_exec(arena, join, Ghost(d), Ghost(c), args);
    let anchor = joined_exec(
        arena,
        pl,
        join,
        Ghost(d),
        Ghost(c),
        "guideline_entity",
        4,
        &args[4],
        2,
    );
    five_cells(h::text(ordinal), part_exec(part), event, timing, anchor)
}

fn piece_eq(a: &EPiece, b: &EPiece) -> (out: bool)
    ensures
        out == (a@ == b@),
{
    match (a, b) {
        (EPiece::Fixed(x), EPiece::Fixed(y)) => crate::k4_bytes::eq(x, y),
        (EPiece::Text(x), EPiece::Text(y)) => crate::k4_bytes::eq(x, y),
        (EPiece::Attr(x), EPiece::Attr(y)) => crate::k4_bytes::eq(x, y),
        (EPiece::Decimal(x), EPiece::Decimal(y)) => *x == *y,
        (EPiece::Comment(x), EPiece::Comment(y)) => crate::k4_bytes::eq(x, y),
        _ => false,
    }
}

pub fn page_eq(a: &EPage, b: &EPage) -> (out: bool)
    ensures
        out == (a@ == b@),
{
    proof {
        assert(a@.len() == a.parts@.len());
        assert(b@.len() == b.parts@.len());
    }
    if a.parts.len() != b.parts.len() {
        return false;
    }
    let mut i = 0usize;
    while i < a.parts.len()
        invariant
            i <= a.parts@.len(),
            a.parts@.len() == b.parts@.len(),
            forall|j: int| 0 <= j < i ==> a@[j] == b@[j],
        decreases a.parts@.len() - i,
    {
        proof {
            assert(a@[i as int] == a.parts@[i as int]@);
            assert(b@[i as int] == b.parts@[i as int]@);
        }
        if !piece_eq(&a.parts[i], &b.parts[i]) {
            return false;
        }
        i += 1;
    }
    proof {
        assert(a@ =~= b@);
    }
    true
}

proof fn first_rows_contains(ys: Seq<u::Html>, x: u::Html)
    ensures
        u::first_rows(ys).contains(x) == ys.contains(x),
    decreases ys.len(),
{
    if ys.len() > 0 {
        let init = ys.drop_last();
        let l = ys.last();
        first_rows_contains(init, x);
        assert(ys =~= init.push(l));
        assert(ys.contains(x) == (init.contains(x) || x == l)) by {
            if ys.contains(x) {
                let k = choose|k: int| 0 <= k < ys.len() && ys[k] == x;
                if k < ys.len() - 1 {
                    assert(init[k] == x);
                }
            }
            if init.contains(x) {
                let k = choose|k: int| 0 <= k < init.len() && init[k] == x;
                assert(ys[k] == x);
            }
            if x == l {
                assert(ys[ys.len() - 1] == x);
            }
        }
        if !init.contains(l) {
            let fr = u::first_rows(init);
            assert(fr.push(l).contains(x) == (fr.contains(x) || x == l)) by {
                if fr.push(l).contains(x) {
                    let k = choose|k: int| 0 <= k < fr.push(l).len() && fr.push(l)[k] == x;
                    if k < fr.len() {
                        assert(fr[k] == x);
                    }
                }
                if fr.contains(x) {
                    let k = choose|k: int| 0 <= k < fr.len() && fr[k] == x;
                    assert(fr.push(l)[k] == x);
                }
                if x == l {
                    assert(fr.push(l)[fr.len() as int] == x);
                }
            }
        }
    }
}

pub fn first_rows_exec(rows: Vec<EPage>) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::first_rows(h::pages(rows@)),
{
    let ghost ps = h::pages(rows@);
    let mut out: Vec<EPage> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ps.take(0) =~= Seq::<u::Html>::empty());
        assert(h::pages(out@) =~= Seq::<u::Html>::empty());
    }
    while i < rows.len()
        invariant
            i <= rows@.len(),
            ps == h::pages(rows@),
            ps.len() == rows@.len(),
            h::pages(out@) == u::first_rows(ps.take(i as int)),
        decreases rows@.len() - i,
    {
        let mut seen = false;
        let mut j = 0usize;
        while j < out.len()
            invariant
                j <= out@.len(),
                i < rows@.len(),
                seen == exists|k: int| 0 <= k < j && h::pages(out@)[k] == rows@[i as int]@,
            decreases out@.len() - j,
        {
            if page_eq(&out[j], &rows[i]) {
                seen = true;
            }
            proof {
                assert(h::pages(out@)[j as int] == out@[j as int]@);
            }
            j += 1;
        }
        let ghost before = h::pages(out@);
        proof {
            assert(ps.take(i + 1) =~= ps.take(i as int).push(ps[i as int]));
            assert(ps.take(i + 1).drop_last() =~= ps.take(i as int));
            assert(ps[i as int] == rows@[i as int]@);
            first_rows_contains(ps.take(i as int), ps[i as int]);
            assert(seen == before.contains(ps[i as int]));
        }
        if !seen {
            out.push(h::copy(&rows[i]));
            proof {
                assert(h::pages(out@) =~= before.push(ps[i as int]));
            }
        }
        i += 1;
    }
    proof {
        assert(ps.take(i as int) =~= ps);
    }
    out
}

pub fn lit_terms_of(lits: &Vec<(T, u8)>) -> (out: Vec<T>)
    ensures
        models(out@) == lit_pairs(lits@).map_values(|p: (Term, int)| p.0),
        forall|i: int| 0 <= i < out@.len() ==> (#[trigger] out@[i]) == lits@[i].0,
        out@.len() == lits@.len(),
{
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    while i < lits.len()
        invariant
            i <= lits@.len(),
            out@.len() == i,
            forall|j: int| 0 <= j < i ==> (#[trigger] out@[j]) == lits@[j].0,
        decreases lits@.len() - i,
    {
        out.push(lits[i].0.cp());
        i += 1;
    }
    proof {
        assert(models(out@) =~= lit_pairs(lits@).map_values(|p: (Term, int)| p.0));
    }
    out
}

pub open spec fn row_opts(pl: Seq<u8>, d: v::DocFile, s: nat, c: v::DocClause) -> Seq<
    Option<u::Html>,
> {
    u::clause_lits(c).map_values(|p: (Term, int)| u::timing_row(pl, d, s, c, p.0, p.1))
}

proof fn somes_push(xs: Seq<Option<u::Html>>, o: Option<u::Html>)
    ensures
        u::somes(xs.push(o)) == u::somes(xs) + match o {
            Some(x) => seq![x],
            None => Seq::<u::Html>::empty(),
        },
{
    assert(xs.push(o).drop_last() =~= xs);
    assert(xs.push(o).last() == o);
}

// Appends the rows of clause `c` (`lits` = its literals, `join` = their terms ++ document heads).
fn clause_rows_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    lits: &Vec<(T, u8)>,
    ordinal: &[u8],
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    Ghost(s): Ghost<nat>,
    Ghost(pre): Ghost<Seq<Option<u::Html>>>,
    rows: Vec<EPage>,
) -> (out: Vec<EPage>)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        lit_pairs(lits@) == u::clause_lits(c),
        forall|i: int|
            0 <= i < lits@.len() ==> valid(arena.nodes@, &(#[trigger] lits@[i]).0) && lits@[i].1
                <= 2,
        ordinal@ == v::udec_bytes(s),
        h::pages(rows@) == u::somes(pre),
    ensures
        h::pages(out@) == u::somes(pre + row_opts(pl@, d, s, c)),
{
    let ghost opts = row_opts(pl@, d, s, c);
    let mut rows = rows;
    let mut j = 0usize;
    proof {
        assert(pre + opts.take(0) =~= pre);
        assert(opts.len() == lit_pairs(lits@).len());
    }
    while j < lits.len()
        invariant
            j <= lits@.len(),
            arena_ok(arena),
            crate::m6_term::valid_all(arena.nodes@, join@),
            models(join@) == u::join_terms(d, c),
            lit_pairs(lits@) == u::clause_lits(c),
            forall|i: int|
                0 <= i < lits@.len() ==> valid(arena.nodes@, &(#[trigger] lits@[i]).0) && lits@[i].1
                    <= 2,
            ordinal@ == v::udec_bytes(s),
            opts == row_opts(pl@, d, s, c),
            opts.len() == lits@.len(),
            h::pages(rows@) == u::somes(pre + opts.take(j as int)),
        decreases lits@.len() - j,
    {
        proof {
            assert(lit_pairs(lits@)[j as int] == (lits@[j as int].0@, lits@[j as int].1 as int));
            assert(valid(arena.nodes@, &lits@[j as int].0) && lits@[j as int].1 <= 2);
        }
        let r = row_exec(
            arena,
            pl,
            join,
            Ghost(d),
            Ghost(c),
            ordinal,
            Ghost(s),
            &lits[j].0,
            lits[j].1,
        );
        let ghost prev = pre + opts.take(j as int);
        let ghost before = h::pages(rows@);
        proof {
            assert(opts[j as int] == opt_page(r));
            assert(pre + opts.take(j + 1) =~= prev.push(opts[j as int]));
            somes_push(prev, opts[j as int]);
        }
        match r {
            Some(p) => {
                let ghost pv = p@;
                rows.push(p);
                proof {
                    assert(h::pages(rows@) =~= before + seq![pv]);
                }
            },
            None => {
                proof {
                    assert(before + Seq::<u::Html>::empty() =~= before);
                }
            },
        }
        j += 1;
    }
    proof {
        assert(opts.take(j as int) =~= opts);
    }
    rows
}

// u::bundle_rows for bundle `b` = clauses cs[lo..hi]; `heads` = every document clause head.
pub fn bundle_rows_exec(
    arena: &ETermArena,
    pl: &[u8],
    cs: &Vec<Clause>,
    lo: usize,
    hi: usize,
    heads: &Vec<T>,
    ordinal: &[u8],
    Ghost(d): Ghost<v::DocFile>,
    Ghost(b): Ghost<v::Bundle>,
) -> (out: Vec<EPage>)
    requires
        arena_ok(arena),
        lo <= hi <= cs@.len(),
        crate::m6_model::clauses_valid(arena.nodes@, cs@),
        crate::m6_model::clause_models(cs@.subrange(lo as int, hi as int)) == b.clauses,
        crate::m6_term::valid_all(arena.nodes@, heads@),
        models(heads@) == u::doc_heads(d),
        ordinal@ == v::udec_bytes(b.s),
    ensures
        h::pages(out@) == u::bundle_rows(pl@, d, b),
{
    let ghost f = |c: v::DocClause| row_opts(pl@, d, b.s, c);
    let ghost all = b.clauses.map_values(f);
    let mut rows: Vec<EPage> = Vec::new();
    let mut k = lo;
    proof {
        assert(all.take(0) =~= Seq::<Seq<Option<u::Html>>>::empty());
        reveal_with_fuel(Seq::<_>::flatten, 1);
        assert(h::pages(rows@) =~= u::somes(all.take(0).flatten()));
    }
    while k < hi
        invariant
            lo <= k <= hi <= cs@.len(),
            arena_ok(arena),
            crate::m6_model::clauses_valid(arena.nodes@, cs@),
            crate::m6_model::clause_models(cs@.subrange(lo as int, hi as int)) == b.clauses,
            crate::m6_term::valid_all(arena.nodes@, heads@),
            models(heads@) == u::doc_heads(d),
            ordinal@ == v::udec_bytes(b.s),
            f == (|c: v::DocClause| row_opts(pl@, d, b.s, c)),
            all == b.clauses.map_values(f),
            h::pages(rows@) == u::somes(all.take(k - lo).flatten()),
        decreases hi - k,
    {
        proof {
            assert(crate::m6_model::clause_valid(arena.nodes@, &cs@[k as int]));
            assert(b.clauses[k - lo] == cs@[k as int]@);
        }
        let ghost c = cs@[k as int]@;
        let lits = lits_of(arena, &cs[k]);
        let terms = lit_terms_of(&lits);
        let join = crate::m6_term::concat(&terms, heads);
        proof {
            assert(models(join@) =~= u::join_terms(d, c));
            assert forall|i: int| 0 <= i < join@.len() implies #[trigger] valid(
                arena.nodes@,
                &join@[i],
            ) by {
                if i < terms@.len() {
                    assert(join@[i] == terms@[i]);
                    assert(terms@[i] == lits@[i].0);
                } else {
                    assert(join@[i] == heads@[i - terms@.len()]);
                }
            }
        }
        let ghost pre = all.take(k - lo).flatten();
        rows =
        clause_rows_exec(
            arena,
            pl,
            &join,
            &lits,
            ordinal,
            Ghost(d),
            Ghost(c),
            Ghost(b.s),
            Ghost(pre),
            rows,
        );
        proof {
            assert(all[k - lo] == row_opts(pl@, d, b.s, c));
            assert(all.take(k + 1 - lo) =~= all.take(k - lo).push(all[k - lo]));
            all.take(k - lo).lemma_flatten_push(all[k - lo]);
        }
        k += 1;
    }
    proof {
        assert(all.take(k - lo) =~= all);
        assert(all =~= b.clauses.map_values(
            |c: v::DocClause|
                u::clause_lits(c).map_values(
                    |p: (Term, int)| u::timing_row(pl@, d, b.s, c, p.0, p.1),
                ),
        ));
    }
    first_rows_exec(rows)
}

proof fn doc_heads_flat(bs: Seq<v::Bundle>)
    ensures
        heads(bs.map_values(|b: v::Bundle| b.clauses).flatten()) == bs.map_values(
            |b: v::Bundle| b.clauses.map_values(|c: v::DocClause| c.head),
        ).flatten(),
    decreases bs.len(),
{
    let p = |b: v::Bundle| b.clauses;
    let q = |b: v::Bundle| b.clauses.map_values(|c: v::DocClause| c.head);
    reveal_with_fuel(Seq::<_>::flatten, 1);
    if bs.len() == 0 {
        assert(bs.map_values(p) =~= Seq::<Seq<v::DocClause>>::empty());
        assert(bs.map_values(q) =~= Seq::<Seq<Term>>::empty());
        assert(heads(Seq::<v::DocClause>::empty()) =~= Seq::<Term>::empty());
    } else {
        let init = bs.drop_last();
        doc_heads_flat(init);
        assert(bs.map_values(p) =~= init.map_values(p).push(bs.last().clauses));
        assert(bs.map_values(q) =~= init.map_values(q).push(q(bs.last())));
        init.map_values(p).lemma_flatten_push(bs.last().clauses);
        init.map_values(q).lemma_flatten_push(q(bs.last()));
        assert(heads(init.map_values(p).flatten() + bs.last().clauses) =~= heads(
            init.map_values(p).flatten(),
        ) + heads(bs.last().clauses));
    }
}

fn no_rows() -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == Seq::<u::Html>::empty(),
{
    let out: Vec<EPage> = Vec::new();
    proof {
        assert(h::pages(out@) =~= Seq::<u::Html>::empty());
    }
    out
}

// u::timing_rows over the K1 parse of `pl`, bundle by bundle.
pub fn timing_rows_exec(pl: &[u8]) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::timing_rows(pl@),
{
    hide(u::bundle_rows);
    reveal(u::timing_rows);
    let mut arena = crate::k2_reject::empty_arena();
    let parsed = match crate::v1_impl::v1_parse(pl, &mut arena) {
        Some(p) => p,
        None => return no_rows(),
    };
    match &parsed.class {
        crate::v1_term_impl::EV1Class::Doc => {},
        _ => {
            proof {
                assert(!(parsed@ is Doc));
            }
            return no_rows();
        },
    }
    let ghost doc = choose|d: v::DocFile| parsed@ == v::V1File::Doc(d);
    proof {
        assert(parsed@ is Doc);
        assert(parsed@ == v::V1File::Doc(doc));
    }
    if parsed.doc_version < 2 {
        return no_rows();
    }
    proof {
        crate::v1_term_impl::parsed_doc_roots_elim(arena.nodes@, &parsed);
        crate::v1_term_impl::doc_clause_models_flatten(doc.bundles);
        doc_heads_flat(doc.bundles);
    }
    let cs = crate::m6_custody::read_clauses(
        &arena,
        &parsed.clauses,
        Ghost(crate::v1_term_impl::doc_clause_models(doc.bundles)),
    );
    let heads = heads_of(&cs);
    let total = cs.len();
    let ghost bs = doc.bundles;
    let ghost parts = bs.map_values(|b: v::Bundle| b.clauses);
    let ghost g = |b: v::Bundle| u::bundle_rows(pl@, doc, b);
    proof {
        assert(models(heads@) == u::doc_heads(doc));
        assert(heads@.len() == cs@.len());
        assert forall|i: int| 0 <= i < heads@.len() implies #[trigger] valid(
            arena.nodes@,
            &heads@[i],
        ) by {
            assert(crate::m6_model::clause_valid(arena.nodes@, &cs@[i]));
        }
    }
    let mut out: Vec<EPage> = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    proof {
        reveal_with_fuel(Seq::<_>::flatten, 1);
        assert(parts.take(0) =~= Seq::<Seq<v::DocClause>>::empty());
        assert(bs.take(0).map_values(g) =~= Seq::<Seq<u::Html>>::empty());
        assert(h::pages(out@) =~= Seq::<u::Html>::empty());
    }
    while i < parsed.bundle_meta.len()
        invariant
            i <= parsed.bundle_meta@.len(),
            arena_ok(&arena),
            crate::v1_term_impl::bundle_metadata_ok(parsed.bundle_meta@, bs),
            crate::m6_model::clauses_valid(arena.nodes@, cs@),
            crate::m6_model::clause_models(cs@) == parts.flatten(),
            parts == bs.map_values(|b: v::Bundle| b.clauses),
            crate::m6_term::valid_all(arena.nodes@, heads@),
            models(heads@) == u::doc_heads(doc),
            start == parts.take(i as int).flatten().len(),
            total == cs@.len(),
            g == (|b: v::Bundle| u::bundle_rows(pl@, doc, b)),
            h::pages(out@) == bs.take(i as int).map_values(g).flatten(),
        decreases parsed.bundle_meta@.len() - i,
    {
        let n = parsed.bundle_meta[i].count;
        let ghost seg = parts[i as int];
        proof {
            assert(n == bs[i as int].clauses.len());
            assert(seg == bs[i as int].clauses);
            let pre = parts.take(i as int);
            assert(parts =~= parts.take(i + 1) + parts.skip(i + 1));
            vstd::seq_lib::lemma_flatten_concat(parts.take(i + 1), parts.skip(i + 1));
            assert(parts.take(i + 1) =~= pre.push(seg));
            pre.lemma_flatten_push(seg);
            let flat = parts.flatten();
            assert(flat =~= pre.flatten() + seg + parts.skip(i + 1).flatten());
            assert(flat.len() == cs@.len());
            assert(start + n <= total);
            assert forall|j: int| 0 <= j < n implies cs@[start + j]@ == seg[j] by {
                assert(crate::m6_model::clause_models(cs@)[start + j] == cs@[start + j]@);
                assert(flat[start + j] == seg[j]);
            }
            assert(crate::m6_model::clause_models(cs@.subrange(start as int, start + n)) =~= seg);
        }
        let mut rows = bundle_rows_exec(
            &arena,
            pl,
            &cs,
            start,
            start + n,
            &heads,
            &parsed.bundle_meta[i].ordinal,
            Ghost(doc),
            Ghost(bs[i as int]),
        );
        let ghost before = h::pages(out@);
        let ghost added = h::pages(rows@);
        out.append(&mut rows);
        proof {
            assert(h::pages(out@) =~= before + added);
            assert(bs.take(i + 1).map_values(g) =~= bs.take(i as int).map_values(g).push(
                g(bs[i as int]),
            ));
            bs.take(i as int).map_values(g).lemma_flatten_push(g(bs[i as int]));
            assert(parts.take(i + 1) =~= parts.take(i as int).push(seg));
            parts.take(i as int).lemma_flatten_push(seg);
        }
        start += n;
        i += 1;
    }
    proof {
        assert(bs.take(i as int) =~= bs);
    }
    out
}

// u::timing_section: the table, or nothing for a document without timing rows.
pub fn timing_section_exec(pl: &[u8]) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::timing_section(pl@),
{
    hide(u::lit);
    let rows = timing_rows_exec(pl);
    if rows.len() == 0 {
        return no_rows();
    }
    let mut out: Vec<EPage> = Vec::new();
    out.push(h::fixed("<section>"));
    out.push(h::fixed("<h3>Timing as compiled</h3>"));
    out.push(
        h::fixed(
            "<p>Each row is one time limit that the compiler read from the ACE text. The compiler records these limits and does not calculate dates.</p>",
        ),
    );
    out.push(h::fixed("<table>"));
    out.push(
        h::fixed(
            "<thead><tr><th>Sentence</th><th>Part</th><th>Action</th><th>Timing</th><th>Reference point</th></tr></thead>",
        ),
    );
    out.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&rows)), h::fixed("</tbody>")));
    out.push(h::fixed("</table>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= u::timing_section(pl@));
    }
    out
}

} // verus!
