use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_model::*;
use crate::m6_normal::canon_clause;
use crate::m6_project::group_clauses;
use crate::m6_term::*;
#[cfg(verus_keep_ghost)]
use crate::v1_term_impl::*;
use crate::v1_term_impl::{EBodyRoot, EBundleMeta, EDocClause};
use ckc_spec::emit as spec;
use ckc_spec::replay;
use ckc_spec::term::Term;
use ckc_spec::v1text::{self, BodyItem, Bundle, DocClause};
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub open spec fn lines_model(ls: Seq<Vec<u8>>) -> Seq<Seq<u8>> {
    ls.map_values(|l: Vec<u8>| l@)
}

pub proof fn nonempty_suffix(bytes: Seq<u8>, start: nat, end: nat)
    requires
        start <= end < bytes.len(),
        bytes[end as int] == 0x0A,
        forall|i: int| start <= i < end ==> bytes[i] != 0x0A,
    ensures
        spec::nonempty_lines(bytes.skip(start as int)) == if start < end {
            seq![bytes.subrange(start as int, end as int)] + spec::nonempty_lines(
                bytes.skip(end as int + 1),
            )
        } else {
            spec::nonempty_lines(bytes.skip(end as int + 1))
        },
{
    crate::k2_manifest::suffix_line(bytes, start, end);
    let line = bytes.subrange(start as int, end as int);
    replay::lines_of(bytes.skip(end as int + 1)).lemma_filter_prepend(
        line,
        |l: Seq<u8>| l.len() > 0,
    );
    reveal(spec::nonempty_lines);
}

pub proof fn nonempty_last(bytes: Seq<u8>, start: nat)
    requires
        start < bytes.len(),
        forall|i: int| start <= i < bytes.len() ==> bytes[i] != 0x0A,
    ensures
        spec::nonempty_lines(bytes.skip(start as int)) == seq![bytes.skip(start as int)],
{
    let suffix = bytes.skip(start as int);
    assert forall|i: int| 0 <= i < suffix.len() implies suffix[i] != 0x0A by {
        assert(suffix[i] == bytes[start as int + i]);
    }
    crate::k2_manifest::first_byte_exact(suffix, 0x0A, 0, suffix.len());
    reveal_with_fuel(replay::lines_of, 1);
    reveal(spec::nonempty_lines);
    reveal_with_fuel(Seq::filter, 2);
}

pub fn nonempty_lines(bytes: &[u8]) -> (out: Vec<Vec<u8>>)
    ensures
        lines_model(out@) == spec::nonempty_lines(bytes@),
{
    let mut out = Vec::new();
    let mut start = 0usize;
    proof {
        assert(bytes@.skip(0) == bytes@);
        assert_seqs_equal!(lines_model(out@) == Seq::<Seq<u8>>::empty());
    }
    while start < bytes.len()
        invariant
            start <= bytes.len(),
            lines_model(out@) + spec::nonempty_lines(bytes@.skip(start as int))
                == spec::nonempty_lines(bytes@),
        decreases bytes.len() - start,
    {
        let end = crate::k2_manifest::first_byte_exec(bytes, 0x0A, start);
        let line = slice_to_vec(&bytes[start..end]);
        let ghost before = out@;
        if end < bytes.len() {
            proof {
                nonempty_suffix(bytes@, start as nat, end as nat);
            }
            if start < end {
                out.push(line);
                proof {
                    assert_seqs_equal!(lines_model(out@) == lines_model(before) + seq![line@]);
                }
            }
            start = end + 1;
        } else {
            proof {
                nonempty_last(bytes@, start as nat);
            }
            out.push(line);
            proof {
                assert_seqs_equal!(lines_model(out@) == lines_model(before) + seq![line@]);
            }
            start = end;
        }
    }
    proof {
        reveal(spec::nonempty_lines);
        reveal(replay::lines_of);
        reveal(Seq::filter);
    }
    out
}

pub fn ulex_matches(
    actual: &Vec<u8>,
    wanted: Option<&Vec<u8>>,
    Ghost(model): Ghost<Option<Seq<u8>>>,
) -> (out: bool)
    requires
        actual@ == ulex_digest_bytes(model),
        v1text::ulex_ok(model),
    ensures
        out == (model == spec::opt_view(wanted)),
{
    proof {
        reveal(v1text::ulex_ok);
        reveal(v1text::hex64);
    }
    match wanted {
        None => actual.len() == 0,
        Some(w) => actual.len() > 0 && bytes_eq(actual, w),
    }
}

pub fn read_body(arena: &ETermArena, root: &EBodyRoot, Ghost(item): Ghost<BodyItem>) -> (out: Body)
    requires
        arena_ok(arena),
        body_root_ok(arena.nodes@, root, item),
    ensures
        body_valid(arena.nodes@, &out),
        out@ == item,
{
    proof {
        body_root_elim(arena.nodes@, root, item);
    }
    match root {
        EBodyRoot::Pos(index) => Body::Pos(from_root(arena, *index)),
        EBodyRoot::Naf(roots) => {
            proof {
                let terms = choose|ts: Seq<Term>| item == BodyItem::Naf(ts);
                assert(roots_ok_nodes(arena.nodes@, roots@, terms));
                assert forall|i: int| 0 <= i < roots.len() implies roots@[i]
                    < arena.nodes.len() by {
                    assert(roots_ok_nodes(arena.nodes@, roots@, terms));
                    assert(roots@[i] < arena.nodes.len() && arena.nodes@[roots@[i] as int].term@
                        == terms[i]);
                }
                assert(crate::k2_engine::roots_valid(arena.nodes@, roots@));
                assert_seqs_equal!(crate::k2_engine::root_terms(arena.nodes@, roots@) == terms);
            }
            Body::Naf(from_roots(arena, roots))
        },
    }
}

pub fn read_clause(arena: &ETermArena, source: &EDocClause) -> (out: Clause)
    requires
        arena_ok(arena),
        doc_clause_roots_ok(arena.nodes@, source),
    ensures
        clause_valid(arena.nodes@, &out),
        out@ == source@,
{
    proof {
        doc_clause_roots_elim(arena.nodes@, source);
        body_roots_elim(arena.nodes@, source.body@, source@.body);
    }
    let head = from_root(arena, source.head_root);
    let mut body = Vec::new();
    let mut i = 0usize;
    while i < source.body.len()
        invariant
            arena_ok(arena),
            doc_clause_roots_ok(arena.nodes@, source),
            body_roots_ok(arena.nodes@, source.body@, source@.body),
            source.body.len() == source@.body.len(),
            forall|j: int|
                0 <= j < source.body.len() ==> #[trigger] body_root_ok(
                    arena.nodes@,
                    &source.body@[j],
                    source@.body[j],
                ),
            bodies_valid(arena.nodes@, body@),
            i <= source.body.len(),
            body.len() == i,
            body_models(body@) == source@.body.take(i as int),
        decreases source.body.len() - i,
    {
        let next = read_body(arena, &source.body[i], Ghost(source@.body[i as int]));
        let ghost before = body@;
        body.push(next);
        proof {
            assert forall|j: int| 0 <= j < body.len() implies #[trigger] body_valid(
                arena.nodes@,
                &body@[j],
            ) by {
                if j < before.len() {
                    assert(body@[j] == before[j]);
                }
            }
            assert_seqs_equal!(body_models(body@) == source@.body.take(i as int + 1), j => { if j < before.len() { assert(body@[j] == before[j]); assert(body_models(before)[j] == source@.body[j]); } else { assert(j == i); assert(body@[j] == next); } });
        }
        i += 1;
    }
    proof {
        assert(source@.body.take(i as int) == source@.body);
        assert(head@ == source@.head);
    }
    Clause { head, body }
}

pub fn read_clauses(
    arena: &ETermArena,
    cs: &Vec<EDocClause>,
    Ghost(expected): Ghost<Seq<DocClause>>,
) -> (out: Vec<Clause>)
    requires
        arena_ok(arena),
        doc_clauses_roots_ok(arena.nodes@, cs@, expected),
    ensures
        clauses_valid(arena.nodes@, out@),
        clause_models(out@) == expected,
{
    proof {
        doc_clauses_roots_elim(arena.nodes@, cs@, expected);
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < cs.len()
        invariant
            arena_ok(arena),
            doc_clauses_roots_ok(arena.nodes@, cs@, expected),
            cs.len() == expected.len(),
            forall|j: int|
                0 <= j < cs.len() ==> #[trigger] doc_clause_roots_ok(arena.nodes@, &cs@[j]),
            forall|j: int| 0 <= j < cs.len() ==> (#[trigger] cs@[j])@ == expected[j],
            clauses_valid(arena.nodes@, out@),
            i <= cs.len(),
            out.len() == i,
            clause_models(out@) == expected.take(i as int),
        decreases cs.len() - i,
    {
        let next = read_clause(arena, &cs[i]);
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] clause_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(clause_models(out@) == expected.take(i as int + 1), j => { if j < before.len() { assert(out@[j] == before[j]); assert(clause_models(before)[j] == expected[j]); } else { assert(j == i); assert(out@[j] == next); } });
        }
        i += 1;
    }
    out
}

pub fn terms_equal(arena: &ETermArena, a: &Vec<T>, b: &Vec<T>) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, a@),
        valid_all(arena.nodes@, b@),
    ensures
        out == (models(a@) == models(b@)),
{
    proof {
        assert(models(a@).len() == a.len());
        assert(models(b@).len() == b.len());
    }
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0usize;
    while i < a.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, a@),
            valid_all(arena.nodes@, b@),
            a.len() == b.len(),
            i <= a.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] models(a@)[j] == models(b@)[j],
        decreases a.len() - i,
    {
        if !crate::m6_term::equal(arena, &a[i], &b[i]) {
            proof {
                assert(models(a@)[i as int] != models(b@)[i as int]);
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models(a@) == models(b@));
    }
    true
}

pub fn body_equal(arena: &ETermArena, a: &Body, b: &Body) -> (out: bool)
    requires
        arena_ok(arena),
        body_valid(arena.nodes@, a),
        body_valid(arena.nodes@, b),
    ensures
        out == (a@ == b@),
{
    match (a, b) {
        (Body::Pos(a), Body::Pos(b)) => crate::m6_term::equal(arena, a, b),
        (Body::Naf(a), Body::Naf(b)) => terms_equal(arena, a, b),
        _ => false,
    }
}

pub fn bodies_equal(arena: &ETermArena, a: &Vec<Body>, b: &Vec<Body>) -> (out: bool)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, a@),
        bodies_valid(arena.nodes@, b@),
    ensures
        out == (body_models(a@) == body_models(b@)),
{
    proof {
        assert(body_models(a@).len() == a.len());
        assert(body_models(b@).len() == b.len());
    }
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0usize;
    while i < a.len()
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, a@),
            bodies_valid(arena.nodes@, b@),
            a.len() == b.len(),
            i <= a.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] body_models(a@)[j] == body_models(b@)[j],
        decreases a.len() - i,
    {
        proof {
            assert(body_valid(arena.nodes@, &a@[i as int]));
            assert(body_valid(arena.nodes@, &b@[i as int]));
        }
        if !body_equal(arena, &a[i], &b[i]) {
            proof {
                assert(body_models(a@)[i as int] != body_models(b@)[i as int]);
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(body_models(a@) == body_models(b@));
    }
    true
}

pub fn clauses_equal(arena: &ETermArena, a: &Vec<Clause>, b: &Vec<Clause>) -> (out: bool)
    requires
        arena_ok(arena),
        clauses_valid(arena.nodes@, a@),
        clauses_valid(arena.nodes@, b@),
    ensures
        out == (clause_models(a@) == clause_models(b@)),
{
    proof {
        assert(clause_models(a@).len() == a.len());
        assert(clause_models(b@).len() == b.len());
    }
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0usize;
    while i < a.len()
        invariant
            arena_ok(arena),
            clauses_valid(arena.nodes@, a@),
            clauses_valid(arena.nodes@, b@),
            a.len() == b.len(),
            i <= a.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] clause_models(a@)[j] == clause_models(b@)[j],
        decreases a.len() - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &a@[i as int]));
            assert(clause_valid(arena.nodes@, &b@[i as int]));
        }
        if !crate::m6_term::equal(arena, &a[i].head, &b[i].head) || !bodies_equal(
            arena,
            &a[i].body,
            &b[i].body,
        ) {
            proof {
                assert(clause_models(a@)[i as int] != clause_models(b@)[i as int]);
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(clause_models(a@) == clause_models(b@));
    }
    true
}

pub fn clause_range(arena: &ETermArena, cs: &Vec<Clause>, start: usize, end: usize) -> (out: Vec<
    Clause,
>)
    requires
        arena_ok(arena),
        clauses_valid(arena.nodes@, cs@),
        start <= end <= cs.len(),
    ensures
        clauses_valid(arena.nodes@, out@),
        clause_models(out@) == clause_models(cs@).subrange(start as int, end as int),
{
    let mut out = Vec::new();
    let mut i = start;
    while i < end
        invariant
            arena_ok(arena),
            clauses_valid(arena.nodes@, cs@),
            clauses_valid(arena.nodes@, out@),
            start <= i <= end <= cs.len(),
            out.len() == i - start,
            clause_models(out@) == clause_models(cs@).subrange(start as int, i as int),
        decreases end - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &cs@[i as int]));
        }
        let next = Clause { head: cs[i].head.cp(), body: copy_body(arena, &cs[i].body) };
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] clause_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(clause_models(out@) == clause_models(cs@).subrange(start as int, i as int + 1), j => {
                if j < before.len() { assert(out@[j] == before[j]); assert(clause_models(before)[j] == clause_models(cs@)[start as int + j]); }
                else { assert(start as int + j == i); assert(out@[j] == next); }
            });
        }
        i += 1;
    }
    out
}

pub fn canon_many(arena: &mut ETermArena, cs: &Vec<Clause>) -> (out: Vec<Clause>)
    requires
        arena_ok(old(arena)),
        clauses_valid(old(arena).nodes@, cs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        clauses_valid(final(arena).nodes@, out@),
        clause_models(out@) == clause_models(cs@).map_values(|c: DocClause| spec::canon_clause(c)),
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < cs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            clauses_valid(arena.nodes@, cs@),
            clauses_valid(arena.nodes@, out@),
            i <= cs.len(),
            out.len() == i,
            clause_models(out@) == clause_models(cs@).take(i as int).map_values(
                |c: DocClause| spec::canon_clause(c),
            ),
        decreases cs.len() - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &cs@[i as int]));
        }
        let ghost middle = arena.nodes@;
        let next = canon_clause(arena, &cs[i]);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            clauses_prefix(middle, arena.nodes@, cs@);
            clauses_prefix(middle, arena.nodes@, out@);
        }
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] clause_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(clause_models(out@) == clause_models(cs@).take(i as int + 1).map_values(|c: DocClause| spec::canon_clause(c)), j => {
                if j < before.len() { assert(out@[j] == before[j]); assert(clause_models(before)[j] == spec::canon_clause(clause_models(cs@)[j])); }
                else { assert(j == i); assert(out@[j] == next); }
            });
        }
        i += 1;
    }
    proof {
        assert(clause_models(cs@).take(i as int) == clause_models(cs@));
    }
    out
}

pub fn bundle_matches(
    arena: &mut ETermArena,
    meta: &EBundleMeta,
    actual: &Vec<Clause>,
    p: &Projected,
    line: &Vec<u8>,
) -> (out: bool)
    requires
        arena_ok(old(arena)),
        bundle_meta_ok(meta),
        clauses_valid(old(arena).nodes@, actual@),
        clause_models(actual@) == meta@.clauses,
        projected_valid(old(arena).nodes@, p),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == spec::bundle_matches(meta@, p@, line@),
{
    let ordinal = crate::k2_manifest::udec_vec(p.s);
    if !bytes_eq(&meta.ordinal, &ordinal) {
        return false;
    }
    proof {
        udec_decimal_value(meta@.s);
        udec_decimal_value(p.s as nat);
        assert(meta@.s == p.s as nat);
    }
    if !bytes_eq(&meta.text, line) {
        return false;
    }
    let projected = group_clauses(arena, &p.groups);
    let ghost start = arena.nodes@;
    let canon = canon_many(arena, &projected);
    proof {
        clauses_prefix(start, arena.nodes@, actual@);
    }
    clauses_equal(arena, actual, &canon)
}

pub proof fn bundle_clauses_unroll(bs: Seq<Bundle>)
    requires
        bs.len() > 0,
    ensures
        doc_clause_models(bs) == bs[0].clauses + doc_clause_models(bs.drop_first()),
{
    doc_clause_models_flatten(bs);
    doc_clause_models_flatten(bs.drop_first());
    let parts = bs.map_values(|b: Bundle| b.clauses);
    assert_seqs_equal!(parts.drop_first() == bs.drop_first().map_values(|b: Bundle| b.clauses));
    reveal_with_fuel(Seq::flatten, 1);
}

pub open spec fn option_nat(x: Option<usize>) -> Option<nat> {
    match x {
        Some(i) => Some(i as nat),
        None => None,
    }
}

pub fn first_mismatch(
    arena: &mut ETermArena,
    ms: &Vec<EBundleMeta>,
    cs: &Vec<Clause>,
    ps: &Vec<Projected>,
    lines: &Vec<Vec<u8>>,
) -> (out: Option<usize>)
    requires
        arena_ok(old(arena)),
        bundle_metas_ok(ms@),
        clauses_valid(old(arena).nodes@, cs@),
        clause_models(cs@) == doc_clause_models(bundle_meta_models(ms@)),
        projections_valid(old(arena).nodes@, ps@),
        ms.len() == ps.len(),
        ms.len() == lines.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(s) ==> 1 <= s <= ms.len(),
        option_nat(out) == spec::first_mismatch(
            bundle_meta_models(ms@),
            project_models(ps@),
            lines_model(lines@),
            0,
        ),
{
    hide(spec::bundle_matches);
    let ghost start = arena.nodes@;
    let mut i = 0usize;
    let mut cursor = 0usize;
    proof {
        assert(bundle_meta_models(ms@).skip(0) == bundle_meta_models(ms@));
        assert(clause_models(cs@).skip(0) == clause_models(cs@));
    }
    while i < ms.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            bundle_metas_ok(ms@),
            clauses_valid(arena.nodes@, cs@),
            projections_valid(arena.nodes@, ps@),
            ms.len() == ps.len(),
            ms.len() == lines.len(),
            i <= ms.len(),
            cursor <= cs.len(),
            clause_models(cs@).skip(cursor as int) == doc_clause_models(
                bundle_meta_models(ms@).skip(i as int),
            ),
            spec::first_mismatch(
                bundle_meta_models(ms@),
                project_models(ps@),
                lines_model(lines@),
                0,
            ) == spec::first_mismatch(
                bundle_meta_models(ms@),
                project_models(ps@),
                lines_model(lines@),
                i as nat,
            ),
        decreases ms.len() - i,
    {
        proof {
            assert(bundle_meta_ok(&ms@[i as int]));
            assert(projected_valid(arena.nodes@, &ps@[i as int]));
            bundle_clauses_unroll(bundle_meta_models(ms@).skip(i as int));
            assert(bundle_meta_models(ms@).skip(i as int).drop_first() == bundle_meta_models(
                ms@,
            ).skip(i as int + 1));
            assert(bundle_meta_models(ms@).skip(i as int)[0] == ms@[i as int]@);
            assert(clause_models(cs@).skip(cursor as int) == ms@[i as int]@.clauses
                + doc_clause_models(bundle_meta_models(ms@).skip(i as int + 1)));
            assert(clause_models(cs@).len() == cs.len());
            assert(clause_models(cs@).skip(cursor as int).len() == cs.len() - cursor);
            assert(cursor as nat + ms@[i as int].count as nat <= cs.len());
            reveal_with_fuel(spec::first_mismatch, 1);
        }
        let end = cursor + ms[i].count;
        let actual = clause_range(arena, cs, cursor, end);
        proof {
            assert_seqs_equal!(clause_models(actual@) == ms@[i as int]@.clauses);
        }
        let ghost middle = arena.nodes@;
        let matched = bundle_matches(arena, &ms[i], &actual, &ps[i], &lines[i]);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            clauses_prefix(middle, arena.nodes@, cs@);
            projections_prefix(middle, arena.nodes@, ps@);
        }
        if !matched {
            return Some(i + 1);
        }
        proof {
            assert_seqs_equal!(clause_models(cs@).skip(end as int) == clause_models(cs@).skip(cursor as int).skip(ms@[i as int].count as int));
            assert_seqs_equal!((ms@[i as int]@.clauses + doc_clause_models(bundle_meta_models(ms@).skip(i as int + 1))).skip(ms@[i as int].count as int) == doc_clause_models(bundle_meta_models(ms@).skip(i as int + 1)));
            assert_seqs_equal!(clause_models(cs@).skip(end as int) == doc_clause_models(bundle_meta_models(ms@).skip(i as int + 1)));
        }
        cursor = end;
        i += 1;
    }
    proof {
        reveal(spec::first_mismatch);
    }
    None
}

} // verus!
