// Contract ur D5: the exclusion sentences of a document — bundles holding the
// scope exclusion record of the key, paired with their ACE line (ported from the
// retired CS1b classifier, archive/cs1b-general-3 k5_scope.rs).
use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k4_bytes::eq;
use crate::m6_model::Clause;
#[cfg(verus_keep_ghost)]
use crate::m6_model::{clause_models, clause_valid, clauses_valid};
use crate::m6_term::T;
#[cfg(verus_keep_ghost)]
use crate::m6_term::valid;
use crate::v1_term_impl::EBundleMeta;
#[cfg(verus_keep_ghost)]
use crate::v1_term_impl::{bundle_meta_models, bundle_meta_ok, bundle_metas_ok};
use ckc_spec::ui::EDocument;
use ckc_spec::{check as ck, emit, ui as u, v1text as v};
use vstd::prelude::*;
verus! {

proof fn decimal_prefix(s: u::Bytes, n: int)
    requires
        0 <= n <= s.len(),
        v::all_in(s, |b: u8| v::is_digit_b(b)),
    ensures
        ck::dec_of(s.take(n)) <= ck::dec_of(s),
    decreases s.len(),
{
    if n < s.len() {
        decimal_prefix(s.drop_last(), n);
        assert(s.take(n) =~= s.drop_last().take(n));
        reveal_with_fuel(ck::dec_of, 2);
    } else {
        assert(s.take(n) =~= s);
    }
}

fn bounded_ordinal(raw: &[u8], Ghost(s): Ghost<nat>, max: usize) -> (out: Option<usize>)
    requires
        raw@ == v::udec_bytes(s),
    ensures
        match out {
            Some(n) => n == s && n <= max,
            None => s > max,
        },
{
    proof {
        crate::k4_numbers::decimal_print_inverse(s);
    }
    let mut n = 0usize;
    let mut i = 0usize;
    proof {
        reveal_with_fuel(ck::dec_of, 1);
    }
    while i < raw.len()
        invariant
            i <= raw.len(),
            n <= max,
            raw@ == v::udec_bytes(s),
            v::all_in(raw@, |b: u8| v::is_digit_b(b)),
            ck::dec_of(raw@) == s,
            n as nat == ck::dec_of(raw@.take(i as int)),
        decreases raw.len() - i,
    {
        let digit = (raw[i] - 48) as usize;
        let ghost prefix = raw@.take(i as int);
        let ghost next = raw@.take(i + 1);
        proof {
            assert(next =~= prefix.push(raw@[i as int]));
            assert(next.drop_last() =~= prefix);
            assert(next.last() == raw@[i as int]);
            reveal_with_fuel(ck::dec_of, 1);
            assert(ck::dec_of(next) == n as nat * 10 + digit as nat);
            decimal_prefix(raw@, i + 1);
        }
        if n > max / 10 {
            proof {
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod(max as int, 10);
                assert(n as nat * 10 > max);
                assert(s > max);
            }
            return None;
        }
        let tens = n * 10;
        if digit > max - tens {
            proof {
                assert(s > max);
            }
            return None;
        }
        n = tens + digit;
        i += 1;
    }
    proof {
        assert(raw@.take(i as int) =~= raw@);
    }
    Some(n)
}

fn paired(lines: &Vec<Vec<u8>>, meta: &EBundleMeta, Ghost(ace): Ghost<u::Bytes>) -> (out: Option<
    usize,
>)
    requires
        crate::m6_custody::lines_model(lines@) == emit::nonempty_lines(ace),
        bundle_meta_ok(meta),
    ensures
        out.is_some() == u::paired(ace, meta@),
        out matches Some(n) ==> n == meta@.s && 1 <= n <= lines.len(),
{
    let n = match bounded_ordinal(&meta.ordinal, Ghost(meta@.s), lines.len()) {
        Some(n) => n,
        None => return None,
    };
    if n == 0 {
        return None;
    }
    if eq(&lines[n - 1], &meta.text) {
        Some(n)
    } else {
        None
    }
}

fn bundle_record(
    arena: &ETermArena,
    cs: &Vec<Clause>,
    lo: usize,
    hi: usize,
    record: &T,
    Ghost(b): Ghost<v::Bundle>,
    Ghost(key): Ghost<u::Bytes>,
) -> (out: bool)
    requires
        arena_ok(arena),
        clauses_valid(arena.nodes@, cs@),
        lo <= hi <= cs.len(),
        clause_models(cs@.subrange(lo as int, hi as int)) == b.clauses,
        valid(arena.nodes@, record),
        record@ == emit::exclusion_record(key),
    ensures
        out == u::has_record(b, key),
{
    let mut i = lo;
    while i < hi
        invariant
            lo <= i <= hi <= cs.len(),
            arena_ok(arena),
            clauses_valid(arena.nodes@, cs@),
            clause_models(cs@.subrange(lo as int, hi as int)) == b.clauses,
            valid(arena.nodes@, record),
            record@ == emit::exclusion_record(key),
            forall|j: int|
                0 <= j < i - lo ==> (#[trigger] b.clauses[j]).head != emit::exclusion_record(key),
        decreases hi - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &cs@[i as int]));
            assert(b.clauses[i - lo] == cs@[i as int]@);
        }
        if crate::m6_term::equal(arena, &cs[i].head, record) {
            proof {
                assert(b.clauses[i - lo].head == emit::exclusion_record(key));
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn exclusion_lines_exec(d: &EDocument, key: &Vec<u8>) -> (out: Vec<u64>)
    ensures
        out@.map_values(|n: u64| n as nat) == u::exclusion_lines(d@, key@),
{
    reveal(u::exclusion_lines);
    let mut arena = crate::k2_reject::empty_arena();
    let record = crate::m6_scope::exclusion_record(&mut arena, key);
    let ghost before_parse = arena.nodes@;
    let parsed = match crate::v1_impl::v1_parse(&d.pl, &mut arena) {
        Some(p) => p,
        None => {
            let out: Vec<u64> = Vec::new();
            proof {
                assert(out@.map_values(|n: u64| n as nat) =~= Seq::<nat>::empty());
            }
            return out;
        },
    };
    proof {
        crate::m6_term::prefix(before_parse, arena.nodes@, &record);
    }
    match &parsed.class {
        crate::v1_term_impl::EV1Class::Doc => {},
        _ => {
            proof {
                assert(!(parsed@ is Doc));
            }
            let out: Vec<u64> = Vec::new();
            proof {
                assert(out@.map_values(|n: u64| n as nat) =~= Seq::<nat>::empty());
            }
            return out;
        },
    }
    let ghost doc = choose|doc: v::DocFile| parsed@ == v::V1File::Doc(doc);
    proof {
        assert(parsed@ == v::V1File::Doc(doc));
        crate::v1_term_impl::parsed_doc_roots_elim(arena.nodes@, &parsed);
        crate::v1_term_impl::doc_clause_models_flatten(doc.bundles);
    }
    let cs = crate::m6_custody::read_clauses(
        &arena,
        &parsed.clauses,
        Ghost(crate::v1_term_impl::doc_clause_models(doc.bundles)),
    );
    let lines = crate::m6_custody::nonempty_lines(&d.ace);
    let ghost bs = doc.bundles;
    let ghost parts = bs.map_values(|b: v::Bundle| b.clauses);
    let ghost selected = |b: v::Bundle| u::has_record(b, key@) && u::paired(d@.ace, b);
    let ghost number = |b: v::Bundle| b.s;
    let mut out: Vec<u64> = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    proof {
        reveal_with_fuel(Seq::<_>::flatten, 1);
        assert(parts.take(0) =~= Seq::<Seq<v::DocClause>>::empty());
        assert(out@.map_values(|n: u64| n as nat) =~= bs.take(0).filter(selected).map_values(
            number,
        ));
    }
    while i < parsed.bundles.len()
        invariant
            i <= parsed.bundles.len(),
            arena_ok(&arena),
            clauses_valid(arena.nodes@, cs@),
            valid(arena.nodes@, &record),
            record@ == emit::exclusion_record(key@),
            bundle_metas_ok(parsed.bundles@),
            bundle_meta_models(parsed.bundles@) == bs,
            clause_models(cs@) == parts.flatten(),
            parts == bs.map_values(|b: v::Bundle| b.clauses),
            start == parts.take(i as int).flatten().len(),
            crate::m6_custody::lines_model(lines@) == emit::nonempty_lines(d@.ace),
            selected == (|b: v::Bundle| u::has_record(b, key@) && u::paired(d@.ace, b)),
            number == (|b: v::Bundle| b.s),
            out@.map_values(|n: u64| n as nat) == bs.take(i as int).filter(selected).map_values(
                number,
            ),
        decreases parsed.bundles.len() - i,
    {
        let meta = &parsed.bundles[i];
        let n = meta.count;
        let ghost seg = parts[i as int];
        proof {
            assert(bundle_meta_ok(meta));
            assert(meta@ == bs[i as int]);
            let pre = parts.take(i as int);
            assert(parts =~= parts.take(i + 1) + parts.skip(i + 1));
            vstd::seq_lib::lemma_flatten_concat(parts.take(i + 1), parts.skip(i + 1));
            assert(parts.take(i + 1) =~= pre.push(seg));
            pre.lemma_flatten_push(seg);
            assert(parts.flatten() =~= pre.flatten() + seg + parts.skip(i + 1).flatten());
            assert(start + n <= cs.len());
            assert(seg == meta@.clauses);
            assert forall|j: int| 0 <= j < n implies cs@[start + j]@ == seg[j] by {
                assert(clause_models(cs@)[start + j] == cs@[start + j]@);
                assert(parts.flatten()[start + j] == seg[j]);
            }
            assert(clause_models(cs@.subrange(start as int, start + n)) =~= meta@.clauses);
        }
        let found = bundle_record(
            &arena,
            &cs,
            start,
            start + n,
            &record,
            Ghost(meta@),
            Ghost(key@),
        );
        let pairing = paired(&lines, meta, Ghost(d@.ace));
        let ghost before = out@.map_values(|n: u64| n as nat);
        proof {
            let pre = bs.take(i as int);
            assert(bs.take(i + 1) =~= pre.push(meta@));
            pre.lemma_filter_push(meta@, selected);
        }
        if found {
            if let Some(number_of) = pairing {
                out.push(number_of as u64);
                proof {
                    assert(out@.map_values(|n: u64| n as nat) =~= before.push(meta@.s));
                    assert(bs.take(i as int).filter(selected).push(meta@).map_values(number)
                        =~= bs.take(i as int).filter(selected).map_values(number).push(meta@.s));
                }
            }
        }
        proof {
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
