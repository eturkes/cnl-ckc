// Shared K1 traversal for the timing, grade, preference and qualifier tables.
use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k5_html as h;
use crate::k5_timing::{first_rows_exec, heads_of, lit_terms_of, lits_of};
#[cfg(verus_keep_ghost)]
use crate::k5_timing::{heads, lit_pairs, opt_page};
use crate::m6_model::Clause;
use crate::m6_term::T;
#[cfg(verus_keep_ghost)]
use crate::m6_term::{models, valid};
use ckc_spec::term::Term;
use ckc_spec::ui::{self as u, EPage};
use ckc_spec::v1text as v;
use vstd::prelude::*;
verus! {

// kind: 0 = timing, 1 = grades, 2 = preferences, 3 = qualifiers.
pub open spec fn row_kind(
    pl: u::Bytes,
    d: v::DocFile,
    s: nat,
    c: v::DocClause,
    l: Term,
    part: int,
    kind: u8,
) -> Option<u::Html> {
    if kind == 0 {
        u::timing_row(pl, d, s, c, l, part)
    } else if kind == 1 {
        u::grade_row(pl, d, c, l)
    } else if kind == 3 {
        u::qualifier_row(pl, d, s, c, l, part)
    } else {
        u::preference_row(pl, d, s, c, l, part)
    }
}

pub open spec fn bundle_kind(pl: u::Bytes, d: v::DocFile, b: v::Bundle, kind: u8) -> Seq<u::Html> {
    if kind == 0 {
        u::bundle_rows(pl, d, b)
    } else if kind == 3 {
        u::qualifier_bundle_rows(pl, d, b)
    } else {
        u::strength_bundle_rows(pl, d, b, kind == 1)
    }
}

pub open spec fn rows_kind(pl: u::Bytes, kind: u8) -> Seq<u::Html> {
    if kind == 0 {
        u::timing_rows(pl)
    } else if kind == 3 {
        u::qualifier_rows(pl)
    } else {
        u::strength_rows(pl, kind == 1)
    }
}

proof fn bundle_unfold(pl: u::Bytes, d: v::DocFile, b: v::Bundle, kind: u8)
    ensures
        bundle_kind(pl, d, b, kind) == u::first_rows(
            u::somes(
                b.clauses.map_values(|c: v::DocClause| row_opts(pl, d, b.s, c, kind)).flatten(),
            ),
        ),
{
    hide(u::timing_row);
    hide(u::grade_row);
    hide(u::preference_row);
    hide(u::qualifier_row);
    hide(u::first_rows);
    hide(u::somes);
    let f = |c: v::DocClause| row_opts(pl, d, b.s, c, kind);
    let g = |c: v::DocClause|
        if kind == 0 {
            u::clause_lits(c).map_values(|p: (Term, int)| u::timing_row(pl, d, b.s, c, p.0, p.1))
        } else if kind == 3 {
            u::clause_lits(c).map_values(|p: (Term, int)| u::qualifier_row(pl, d, b.s, c, p.0, p.1))
        } else {
            u::clause_lits(c).map_values(
                |p: (Term, int)|
                    if kind == 1 {
                        u::grade_row(pl, d, c, p.0)
                    } else {
                        u::preference_row(pl, d, b.s, c, p.0, p.1)
                    },
            )
        };
    assert forall|i: int| 0 <= i < b.clauses.len() implies #[trigger] b.clauses.map_values(f)[i]
        == b.clauses.map_values(g)[i] by {
        let c = b.clauses[i];
        assert(f(c) =~= g(c));
    }
    assert(b.clauses.map_values(f) =~= b.clauses.map_values(g));
    if kind == 0 {
        assert(b.clauses.map_values(g) =~= b.clauses.map_values(
            |c: v::DocClause|
                u::clause_lits(c).map_values(
                    |p: (Term, int)| u::timing_row(pl, d, b.s, c, p.0, p.1),
                ),
        ));
    } else if kind == 3 {
        assert(b.clauses.map_values(g) =~= b.clauses.map_values(
            |c: v::DocClause|
                u::clause_lits(c).map_values(
                    |p: (Term, int)| u::qualifier_row(pl, d, b.s, c, p.0, p.1),
                ),
        ));
    } else {
        assert(b.clauses.map_values(g) =~= b.clauses.map_values(
            |c: v::DocClause|
                u::clause_lits(c).map_values(
                    |p: (Term, int)|
                        if kind == 1 {
                            u::grade_row(pl, d, c, p.0)
                        } else {
                            u::preference_row(pl, d, b.s, c, p.0, p.1)
                        },
                ),
        ));
    }
}

proof fn rows_unfold(pl: u::Bytes, kind: u8)
    ensures
        rows_kind(pl, kind) == if v::accepts(pl) {
            match ckc_spec::replay::the_v1(pl) {
                v::V1File::Doc(d) => d.bundles.map_values(
                    |b: v::Bundle| bundle_kind(pl, d, b, kind),
                ).flatten(),
                _ => Seq::<u::Html>::empty(),
            }
        } else {
            Seq::<u::Html>::empty()
        },
{
    hide(u::bundle_rows);
    hide(u::strength_bundle_rows);
    hide(u::qualifier_bundle_rows);
    reveal(u::timing_rows);
    reveal(u::strength_rows);
    reveal(u::qualifier_rows);
    if v::accepts(pl) {
        if let v::V1File::Doc(d) = ckc_spec::replay::the_v1(pl) {
            let f = |b: v::Bundle| bundle_kind(pl, d, b, kind);
            if kind == 0 {
                assert(d.bundles.map_values(f) =~= d.bundles.map_values(
                    |b: v::Bundle| u::bundle_rows(pl, d, b),
                ));
            } else if kind == 3 {
                assert(d.bundles.map_values(f) =~= d.bundles.map_values(
                    |b: v::Bundle| u::qualifier_bundle_rows(pl, d, b),
                ));
            } else {
                assert(d.bundles.map_values(f) =~= d.bundles.map_values(
                    |b: v::Bundle| u::strength_bundle_rows(pl, d, b, kind == 1),
                ));
            }
        }
    }
}

fn row_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    ordinal: &[u8],
    Ghost(s): Ghost<nat>,
    l: &T,
    part: u8,
    kind: u8,
) -> (out: Option<EPage>)
    requires
        arena_ok(arena),
        crate::m6_term::valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, l),
        ordinal@ == v::udec_bytes(s),
        part <= 2,
    ensures
        opt_page(out) == row_kind(pl@, d, s, c, l@, part as int, kind),
{
    hide(u::timing_row);
    hide(u::grade_row);
    hide(u::preference_row);
    hide(u::qualifier_row);
    if kind == 0 {
        crate::k5_timing::row_exec(arena, pl, join, Ghost(d), Ghost(c), ordinal, Ghost(s), l, part)
    } else if kind == 1 {
        crate::k5_strength::grade_row_exec(arena, pl, join, Ghost(d), Ghost(c), l)
    } else if kind == 3 {
        crate::k5_qualifier::row_exec(
            arena,
            pl,
            join,
            Ghost(d),
            Ghost(c),
            ordinal,
            Ghost(s),
            l,
            part,
        )
    } else {
        crate::k5_strength::preference_row_exec(
            arena,
            pl,
            join,
            Ghost(d),
            Ghost(c),
            ordinal,
            Ghost(s),
            l,
            part,
        )
    }
}

pub open spec fn row_opts(pl: Seq<u8>, d: v::DocFile, s: nat, c: v::DocClause, kind: u8) -> Seq<
    Option<u::Html>,
> {
    u::clause_lits(c).map_values(|p: (Term, int)| row_kind(pl, d, s, c, p.0, p.1, kind))
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
    kind: u8,
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
        h::pages(out@) == u::somes(pre + row_opts(pl@, d, s, c, kind)),
{
    let ghost opts = row_opts(pl@, d, s, c, kind);
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
            opts == row_opts(pl@, d, s, c, kind),
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
            kind,
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

// Bundle clauses = cs[lo..hi]; heads = every document clause head.
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
    kind: u8,
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
        h::pages(out@) == bundle_kind(pl@, d, b, kind),
{
    let ghost f = |c: v::DocClause| row_opts(pl@, d, b.s, c, kind);
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
            f == (|c: v::DocClause| row_opts(pl@, d, b.s, c, kind)),
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
            kind,
        );
        proof {
            assert(all[k - lo] == row_opts(pl@, d, b.s, c, kind));
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
                    |p: (Term, int)| row_kind(pl@, d, b.s, c, p.0, p.1, kind),
                ),
        ));
    }
    proof {
        bundle_unfold(pl@, d, b, kind);
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

pub fn no_rows() -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == Seq::<u::Html>::empty(),
{
    let out: Vec<EPage> = Vec::new();
    proof {
        assert(h::pages(out@) =~= Seq::<u::Html>::empty());
    }
    out
}

// The compiled tables share one K1 parse traversal.
pub fn rows_exec(pl: &[u8], kind: u8) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == rows_kind(pl@, kind),
{
    hide(bundle_kind);
    proof {
        rows_unfold(pl@, kind);
    }
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
    let ghost g = |b: v::Bundle| bundle_kind(pl@, doc, b, kind);
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
            g == (|b: v::Bundle| bundle_kind(pl@, doc, b, kind)),
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
            kind,
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

} // verus!
