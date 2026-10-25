use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_drs::ERoot;
#[cfg(verus_keep_ghost)]
use crate::m6_drs::{root_models, root_valid, roots_valid};
use crate::m6_model::*;
use crate::m6_scope::{deontic_edge, has_deontic};
use crate::m6_symbols::Sym;
use crate::m6_term::*;
#[cfg(verus_keep_ghost)]
use crate::m7_annotate::tab_view;
use crate::m7_q12::EEnv;
use crate::m7_supersession::{ESuperseding, clone_sup};
#[cfg(verus_keep_ghost)]
use crate::m7_supersession::{sup_opt, sup_rows};
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text::DocClause;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn superseding_of(rows: &Vec<(Vec<u8>, ESuperseding)>, docid: &Vec<u8>) -> (out: Option<
    ESuperseding,
>)
    ensures
        sup_opt(out) == spec::superseding_of(sup_rows(rows@), docid@),
{
    hide(spec::superseding_of);
    let ghost ms = sup_rows(rows@);
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) == ms);
    }
    while i < rows.len()
        invariant
            i <= rows.len(),
            ms == sup_rows(rows@),
            spec::superseding_of(ms, docid@) == spec::superseding_of(ms.skip(i as int), docid@),
        decreases rows.len() - i,
    {
        proof {
            assert(ms.skip(i as int)[0] == (rows@[i as int].0@, rows@[i as int].1@));
            assert(ms.skip(i as int).drop_first() == ms.skip(i + 1));
            reveal_with_fuel(spec::superseding_of, 1);
        }
        if bytes_eq(&rows[i].0, docid) {
            return Some(clone_sup(&rows[i].1));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::superseding_of, 1);
    }
    None
}

pub fn supersession_record(
    arena: &mut ETermArena,
    docid: &Vec<u8>,
    s: usize,
    sup: &ESuperseding,
) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::supersession_record(docid@, s as nat, sup@),
{
    let ghost start = arena.nodes@;
    let citation = atom(arena, &sup.citation);
    let ghost a = arena.nodes@;
    let url = atom(arena, &sup.url);
    proof {
        crate::k2_load::prefix_chain(start, a, arena.nodes@);
        prefix(a, arena.nodes@, &citation);
    }
    let ghost b = arena.nodes@;
    let source = c2(arena, &Sym::Source, &citation, &url);
    proof {
        crate::k2_load::prefix_chain(start, b, arena.nodes@);
    }
    let ghost c = arena.nodes@;
    let doc = atom(arena, docid);
    proof {
        crate::k2_load::prefix_chain(start, c, arena.nodes@);
        prefix(c, arena.nodes@, &source);
    }
    let ghost d = arena.nodes@;
    let sentence = crate::m6_term::int(arena, s);
    proof {
        crate::k2_load::prefix_chain(start, d, arena.nodes@);
        prefix(d, arena.nodes@, &source);
        prefix(d, arena.nodes@, &doc);
    }
    let ghost e = arena.nodes@;
    let date = crate::m6_term::int(arena, sup.date);
    proof {
        crate::k2_load::prefix_chain(start, e, arena.nodes@);
        prefix(e, arena.nodes@, &source);
        prefix(e, arena.nodes@, &doc);
        prefix(e, arena.nodes@, &sentence);
    }
    let mut ts = Vec::new();
    ts.push(doc);
    ts.push(sentence);
    ts.push(source);
    ts.push(date);
    proof {
        assert_seqs_equal!(models(ts@) == seq![Term::Atom(docid@), Term::Int(s as int),
            Term::Comp(ckc_spec::v1text::ascii("source"@), seq![Term::Atom(sup.citation@), Term::Atom(sup.url@)]),
            Term::Int(sup.date as int)]);
    }
    let ghost f = arena.nodes@;
    let out = crate::m6_term::c(arena, &Sym::GuidelineSupersession, &ts);
    proof {
        crate::k2_load::prefix_chain(start, f, arena.nodes@);
    }
    out
}

pub fn date_goals(arena: &mut ETermArena, v: &T, date: usize) -> (out: Vec<Body>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, v),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        bodies_valid(final(arena).nodes@, out@),
        body_models(out@) == spec::date_goals(v@, date as nat),
{
    let ghost start = arena.nodes@;
    let evaluation = c1(arena, &Sym::GuidelineEvaluationDate, v);
    proof {
        prefix(start, arena.nodes@, v);
    }
    let ghost a = arena.nodes@;
    let exact = named(arena, &Sym::Exactly);
    proof {
        crate::k2_load::prefix_chain(start, a, arena.nodes@);
        prefix(a, arena.nodes@, v);
        prefix(a, arena.nodes@, &evaluation);
    }
    let ghost b = arena.nodes@;
    let less = named(arena, &Sym::Less);
    proof {
        crate::k2_load::prefix_chain(start, b, arena.nodes@);
        prefix(b, arena.nodes@, v);
        prefix(b, arena.nodes@, &evaluation);
        prefix(b, arena.nodes@, &exact);
    }
    let ghost c = arena.nodes@;
    let bound = crate::m6_term::int(arena, date);
    proof {
        crate::k2_load::prefix_chain(start, c, arena.nodes@);
        prefix(c, arena.nodes@, v);
        prefix(c, arena.nodes@, &evaluation);
        prefix(c, arena.nodes@, &exact);
        prefix(c, arena.nodes@, &less);
    }
    let mut args = Vec::new();
    args.push(exact);
    args.push(v.cp());
    args.push(less);
    args.push(bound);
    proof {
        assert_seqs_equal!(models(args@) == seq![ckc_spec::replay::atom("exactly"@), v@, ckc_spec::replay::atom("less"@), Term::Int(date as int)]);
    }
    let ghost d = arena.nodes@;
    let satisfies = crate::m6_term::c(arena, &Sym::GuidelineSatisfies, &args);
    proof {
        crate::k2_load::prefix_chain(start, d, arena.nodes@);
        prefix(d, arena.nodes@, &evaluation);
    }
    let mut out = Vec::new();
    out.push(Body::Pos(evaluation));
    out.push(Body::Pos(satisfies));
    proof {
        assert_seqs_equal!(body_models(out@) == spec::date_goals(v@, date as nat));
    }
    out
}

pub fn scope_guard_item(arena: &ETermArena, it: &Body) -> (out: bool)
    requires
        arena_ok(arena),
        body_valid(arena.nodes@, it),
    ensures
        out == spec::scope_guard_item(it@),
{
    match it {
        Body::Naf(gs) => gs.len() == 1 && is_comp(arena, &gs[0], &Sym::GuidelineExclusion, 1),
        _ => false,
    }
}

pub fn date_guarded(arena: &ETermArena, body: &Vec<Body>, goals: &Vec<Body>) -> (out: Vec<Body>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, body@),
        bodies_valid(arena.nodes@, goals@),
    ensures
        bodies_valid(arena.nodes@, out@),
        body_models(out@) == spec::date_guarded(body_models(body@), body_models(goals@)),
{
    let mut out = copy_body(arena, body);
    proof {
        assert(body_models(out@).len() == out.len());
        assert(body_models(body@).len() == body.len());
        assert(out.len() == body.len());
    }
    let mut tail: Vec<Body> = Vec::new();
    if body.len() > 0 {
        proof {
            assert(body_valid(arena.nodes@, &body@[body.len() - 1]));
        }
        if scope_guard_item(arena, &body[body.len() - 1]) {
            let last = out.remove(out.len() - 1);
            tail.push(last);
            proof {
                assert(body_models(out@) == body_models(body@).drop_last());
                assert(body_models(tail@) == seq![body_models(body@).last()]);
                assert(bodies_valid(arena.nodes@, out@));
                assert(bodies_valid(arena.nodes@, tail@));
            }
        }
    }
    let mut added = copy_body(arena, goals);
    let ghost left = out@;
    let ghost right = added@;
    out.append(&mut added);
    proof {
        assert_seqs_equal!(body_models(out@) == body_models(left) + body_models(right));
        assert forall|i: int| 0 <= i < out.len() implies #[trigger] body_valid(
            arena.nodes@,
            &out@[i],
        ) by {
            if i < left.len() {
                assert(out@[i] == left[i]);
            } else {
                assert(out@[i] == right[i - left.len()]);
            }
        }
    }
    let ghost left = out@;
    let ghost right = tail@;
    out.append(&mut tail);
    proof {
        assert_seqs_equal!(body_models(out@) == body_models(left) + body_models(right));
        assert forall|i: int| 0 <= i < out.len() implies #[trigger] body_valid(
            arena.nodes@,
            &out@[i],
        ) by {
            if i < left.len() {
                assert(out@[i] == left[i]);
            } else {
                assert(out@[i] == right[i - left.len()]);
            }
        }
    }
    out
}

fn terms_nvars(arena: &mut ETermArena, ts: &Vec<T>) -> (out: usize)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ts@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == ckc_spec::engine::nvars_all(models(ts@)),
{
    let ghost start = arena.nodes@;
    let mut i = ts.len();
    let mut out = 0usize;
    proof {
        reveal_with_fuel(ckc_spec::engine::nvars_all, 1);
    }
    while i > 0
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, ts@),
            i <= ts.len(),
            out as nat == ckc_spec::engine::nvars_all(models(ts@).skip(i as int)),
        decreases i,
    {
        i -= 1;
        let ghost before = arena.nodes@;
        let n = nvars(arena, &ts[i]);
        proof {
            crate::k2_load::prefix_chain(start, before, arena.nodes@);
            prefix_all(before, arena.nodes@, ts@);
            assert(models(ts@).skip(i as int)[0] == ts@[i as int]@);
            assert(models(ts@).skip(i as int).drop_first() == models(ts@).skip(i + 1));
            reveal_with_fuel(ckc_spec::engine::nvars_all, 1);
        }
        if n > out {
            out = n;
        }
    }
    proof {
        assert(models(ts@).skip(0) =~= models(ts@));
    }
    out
}

fn item_nvars(arena: &mut ETermArena, b: &Body) -> (out: usize)
    requires
        arena_ok(old(arena)),
        body_valid(old(arena).nodes@, b),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == spec::item_nvars(b@),
{
    match b {
        Body::Pos(t) => nvars(arena, t),
        Body::Naf(ts) => terms_nvars(arena, ts),
    }
}

fn body_nvars(arena: &mut ETermArena, bs: &Vec<Body>) -> (out: usize)
    requires
        arena_ok(old(arena)),
        bodies_valid(old(arena).nodes@, bs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == spec::body_nvars(body_models(bs@)),
{
    let ghost start = arena.nodes@;
    let mut i = bs.len();
    let mut out = 0usize;
    proof {
        reveal_with_fuel(spec::body_nvars, 1);
    }
    while i > 0
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            bodies_valid(arena.nodes@, bs@),
            i <= bs.len(),
            out as nat == spec::body_nvars(body_models(bs@).skip(i as int)),
        decreases i,
    {
        i -= 1;
        proof {
            assert(body_valid(arena.nodes@, &bs@[i as int]));
        }
        let ghost before = arena.nodes@;
        let n = item_nvars(arena, &bs[i]);
        proof {
            crate::k2_load::prefix_chain(start, before, arena.nodes@);
            bodies_prefix(before, arena.nodes@, bs@);
            assert(body_models(bs@).skip(i as int)[0] == bs@[i as int]@);
            assert(body_models(bs@).skip(i as int).drop_first() == body_models(bs@).skip(i + 1));
            reveal_with_fuel(spec::body_nvars, 1);
        }
        if n > out {
            out = n;
        }
    }
    proof {
        assert(body_models(bs@).skip(0) =~= body_models(bs@));
    }
    out
}

pub fn clauses_nvars(arena: &mut ETermArena, cs: &Vec<Clause>) -> (out: usize)
    requires
        arena_ok(old(arena)),
        clauses_valid(old(arena).nodes@, cs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == spec::clauses_nvars(clause_models(cs@)),
{
    let ghost start = arena.nodes@;
    let mut i = cs.len();
    let mut out = 0usize;
    proof {
        reveal_with_fuel(spec::clauses_nvars, 1);
    }
    while i > 0
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            clauses_valid(arena.nodes@, cs@),
            i <= cs.len(),
            out as nat == spec::clauses_nvars(clause_models(cs@).skip(i as int)),
        decreases i,
    {
        i -= 1;
        proof {
            assert(clause_valid(arena.nodes@, &cs@[i as int]));
        }
        let ghost before = arena.nodes@;
        let h = nvars(arena, &cs[i].head);
        proof {
            crate::k2_load::prefix_chain(start, before, arena.nodes@);
            clauses_prefix(before, arena.nodes@, cs@);
            assert(clause_valid(arena.nodes@, &cs@[i as int]));
        }
        let ghost middle = arena.nodes@;
        let b = body_nvars(arena, &cs[i].body);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            clauses_prefix(middle, arena.nodes@, cs@);
            assert(clause_models(cs@).skip(i as int)[0] == cs@[i as int]@);
            assert(clause_models(cs@).skip(i as int).drop_first() == clause_models(cs@).skip(
                i + 1,
            ));
            reveal_with_fuel(spec::clauses_nvars, 1);
        }
        if h > out {
            out = h;
        }
        if b > out {
            out = b;
        }
    }
    proof {
        assert(clause_models(cs@).skip(0) =~= clause_models(cs@));
    }
    out
}

pub fn has_deontic_edge(arena: &ETermArena, g: &Group) -> (out: bool)
    requires
        arena_ok(arena),
        group_valid(arena.nodes@, g),
    ensures
        out == spec::has_deontic_edge(g@),
{
    let mut i = 0usize;
    while i < g.clauses.len()
        invariant
            arena_ok(arena),
            group_valid(arena.nodes@, g),
            i <= g.clauses.len(),
            forall|j: int| 0 <= j < i ==> !spec::deontic_edge(#[trigger] g@.clauses[j].head),
        decreases g.clauses.len() - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &g.clauses@[i as int]));
        }
        if deontic_edge(arena, &g.clauses[i].head) {
            proof {
                assert(spec::deontic_edge(g@.clauses[i as int].head));
            }
            return true;
        }
        i += 1;
    }
    false
}

fn previous_date(arena: &mut ETermArena, date: usize) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Int(date as int - 1),
{
    if date > 0 {
        return crate::m6_term::int(arena, date - 1);
    }
    let spelling = crate::k4_bytes::copy(b"-1");
    let magnitude = crate::k4_bytes::copy(b"1");
    proof {
        reveal_byteslit(b"-1");
        reveal_byteslit(b"1");
        reveal(ckc_spec::v1text::dec_bytes);
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 1);
        reveal(ckc_spec::v1text::digit_byte);
        assert(spelling@ =~= ckc_spec::v1text::dec_bytes(-1));
        assert(magnitude@ =~= ckc_spec::v1text::udec_bytes(1));
    }
    let root = crate::k2_term::push_int(arena, spelling, magnitude, true, Ghost(-1int));
    from_root(arena, root)
}

pub fn date_group(arena: &mut ETermArena, g: &Group, date: usize) -> (out: Group)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        group_valid(final(arena).nodes@, &out),
        out@ == spec::date_group(g@, date as nat),
{
    if !has_deontic_edge(arena, g) {
        return Group {
            k: g.k,
            pairs: copy_map(arena, &g.pairs),
            clauses: crate::m6_project::copy_clauses(arena, &g.clauses),
        };
    }
    let ghost start = arena.nodes@;
    let n = clauses_nvars(arena, &g.clauses);
    proof {
        map_prefix(start, arena.nodes@, g.pairs@);
        clauses_prefix(start, arena.nodes@, g.clauses@);
    }
    let ghost a = arena.nodes@;
    let v = var(arena, n);
    proof {
        crate::k2_load::prefix_chain(start, a, arena.nodes@);
        map_prefix(a, arena.nodes@, g.pairs@);
        clauses_prefix(a, arena.nodes@, g.clauses@);
    }
    let ghost b = arena.nodes@;
    let witness = previous_date(arena, date);
    proof {
        crate::k2_load::prefix_chain(start, b, arena.nodes@);
        map_prefix(b, arena.nodes@, g.pairs@);
        clauses_prefix(b, arena.nodes@, g.clauses@);
        prefix(b, arena.nodes@, &v);
    }
    let ghost c = arena.nodes@;
    let goals = date_goals(arena, &v, date);
    proof {
        crate::k2_load::prefix_chain(start, c, arena.nodes@);
        map_prefix(c, arena.nodes@, g.pairs@);
        clauses_prefix(c, arena.nodes@, g.clauses@);
        prefix(c, arena.nodes@, &v);
        prefix(c, arena.nodes@, &witness);
    }
    let mut pairs = copy_map(arena, &g.pairs);
    let pair = Binding { key: v.cp(), value: witness };
    let ghost before = pairs@;
    pairs.push(pair);
    proof {
        assert_seqs_equal!(map_model(pairs@) == map_model(before).push(pair@));
        assert forall|j: int| 0 <= j < pairs.len() implies #[trigger] binding_valid(
            arena.nodes@,
            &pairs@[j],
        ) by {
            if j < before.len() {
                assert(pairs@[j] == before[j]);
            }
        }
    }
    let mut clauses = Vec::new();
    let mut i = 0usize;
    while i < g.clauses.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            group_valid(arena.nodes@, g),
            spec::has_deontic_edge(g@),
            clauses_valid(arena.nodes@, clauses@),
            map_valid(arena.nodes@, pairs@),
            map_model(pairs@) == g@.pairs.push((v@, Term::Int(date as int - 1))),
            v@ == Term::Var(spec::clauses_nvars(g@.clauses)),
            bodies_valid(arena.nodes@, goals@),
            body_models(goals@) == spec::date_goals(v@, date as nat),
            i <= g.clauses.len(),
            clauses.len() == i,
            clause_models(clauses@) == g@.clauses.take(i as int).map_values(
                |c: DocClause|
                    DocClause {
                        head: c.head,
                        body: spec::date_guarded(c.body, spec::date_goals(v@, date as nat)),
                    },
            ),
        decreases g.clauses.len() - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &g.clauses@[i as int]));
        }
        let body = date_guarded(arena, &g.clauses[i].body, &goals);
        let next = Clause { head: g.clauses[i].head.cp(), body };
        let ghost before = clauses@;
        clauses.push(next);
        proof {
            assert forall|j: int| 0 <= j < clauses.len() implies #[trigger] clause_valid(
                arena.nodes@,
                &clauses@[j],
            ) by {
                if j < before.len() {
                    assert(clauses@[j] == before[j]);
                }
            }
            assert_seqs_equal!(clause_models(clauses@) == g@.clauses.take(i + 1).map_values(|c: DocClause|
                DocClause { head: c.head, body: spec::date_guarded(c.body, spec::date_goals(v@, date as nat)) }), j => {
                if j < before.len() {
                    assert(clauses@[j] == before[j]);
                    assert(clause_models(before)[j] == DocClause {
                        head: g@.clauses[j].head,
                        body: spec::date_guarded(g@.clauses[j].body, spec::date_goals(v@, date as nat)),
                    });
                } else { assert(j == i); assert(clauses@[j] == next); assert(g@.clauses[j] == g.clauses@[j]@); }
            });
        }
        i += 1;
    }
    proof {
        assert(g@.clauses.take(i as int) =~= g@.clauses);
    }
    Group { k: g.k, pairs, clauses }
}

pub fn date_groups(arena: &mut ETermArena, gs: &Vec<Group>, date: usize) -> (out: Vec<Group>)
    requires
        arena_ok(old(arena)),
        groups_valid(old(arena).nodes@, gs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        groups_valid(final(arena).nodes@, out@),
        group_models(out@) == group_models(gs@).map_values(
            |g: spec::Group| spec::date_group(g, date as nat),
        ),
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < gs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            groups_valid(arena.nodes@, gs@),
            groups_valid(arena.nodes@, out@),
            i <= gs.len(),
            out.len() == i,
            group_models(out@) == group_models(gs@).take(i as int).map_values(
                |g: spec::Group| spec::date_group(g, date as nat),
            ),
        decreases gs.len() - i,
    {
        proof {
            assert(group_valid(arena.nodes@, &gs@[i as int]));
        }
        let ghost before_arena = arena.nodes@;
        let next = date_group(arena, &gs[i], date);
        proof {
            crate::k2_load::prefix_chain(start, before_arena, arena.nodes@);
            groups_prefix(before_arena, arena.nodes@, gs@);
            groups_prefix(before_arena, arena.nodes@, out@);
        }
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] group_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(group_models(out@) == group_models(gs@).take(i + 1).map_values(|g: spec::Group| spec::date_group(g, date as nat)), j => {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                    assert(group_models(before)[j] == spec::date_group(group_models(gs@)[j], date as nat));
                } else { assert(j == i); assert(out@[j] == next); assert(group_models(gs@)[j] == gs@[j]@); }
            });
        }
        i += 1;
    }
    proof {
        assert(group_models(gs@).take(i as int) =~= group_models(gs@));
    }
    out
}

spec fn root_has_deontic(root: spec::Root) -> bool {
    match root {
        spec::Root::Anchored(c) | spec::Root::Boxed(c) => spec::has_deontic(c),
        spec::Root::Rule(_, _) => false,
    }
}

pub fn root_deontic(arena: &ETermArena, roots: &Vec<ERoot>) -> (out: bool)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, roots@),
    ensures
        out == spec::root_deontic(root_models(roots@)),
{
    let mut i = 0usize;
    while i < roots.len()
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, roots@),
            i <= roots.len(),
            forall|j: int| 0 <= j < i ==> !root_has_deontic(#[trigger] root_models(roots@)[j]),
        decreases roots.len() - i,
    {
        proof {
            assert(root_valid(arena.nodes@, &roots@[i as int]));
        }
        let found = match &roots[i] {
            ERoot::Anchored(c) | ERoot::Boxed(c) => has_deontic(arena, c),
            ERoot::Rule(_, _) => false,
        };
        if found {
            proof {
                assert(root_has_deontic(root_models(roots@)[i as int]));
            }
            return true;
        }
        i += 1;
    }
    proof {
        assert forall|j: int| 0 <= j < roots.len() implies !match #[trigger] root_models(
            roots@,
        )[j] {
            spec::Root::Anchored(c) => spec::has_deontic(c),
            spec::Root::Boxed(c) => spec::has_deontic(c),
            spec::Root::Rule(_, _) => false,
        } by {
            assert(!root_has_deontic(root_models(roots@)[j]));
        }
    }
    false
}

pub open spec fn pass_result(out: Result<(Vec<Group>, Vec<Clause>), T>) -> Result<
    (Seq<spec::Group>, Seq<DocClause>),
    Term,
> {
    match out {
        Ok((groups, records)) => Ok((group_models(groups@), clause_models(records@))),
        Err(e) => Err(e@),
    }
}

pub open spec fn pass_valid(
    nodes: Seq<crate::k2_term::ENode>,
    out: &Result<(Vec<Group>, Vec<Clause>), T>,
) -> bool {
    match out {
        Ok((groups, records)) => groups_valid(nodes, groups@) && clauses_valid(nodes, records@),
        Err(e) => valid(nodes, e),
    }
}

pub fn supersession_pass(
    arena: &mut ETermArena,
    tab: &Option<EEnv>,
    docid: &Vec<u8>,
    s: usize,
    roots: &Vec<ERoot>,
    gs: Vec<Group>,
) -> (out: Result<(Vec<Group>, Vec<Clause>), T>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
        groups_valid(old(arena).nodes@, gs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        pass_valid(final(arena).nodes@, &out),
        pass_result(out) == spec::supersession_pass(
            tab_view(tab),
            docid@,
            s as nat,
            root_models(roots@),
            group_models(gs@),
        ),
{
    let empty_records: Vec<Clause> = Vec::new();
    proof {
        assert(clause_models(empty_records@) =~= Seq::<DocClause>::empty());
    }
    let env = match tab {
        Some(env) => env,
        None => return Ok((gs, empty_records)),
    };
    let sup = match superseding_of(&env.tab.supersessions, docid) {
        Some(sup) => sup,
        None => return Ok((gs, empty_records)),
    };
    if root_deontic(arena, roots) {
        return Err(crate::m7_strength::shape(arena, &Sym::SupersessionShape, &Sym::RootDeontic));
    }
    let ghost start = arena.nodes@;
    let groups = date_groups(arena, &gs, sup.date);
    let ghost middle = arena.nodes@;
    let record = supersession_record(arena, docid, s, &sup);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
        groups_prefix(middle, arena.nodes@, groups@);
    }
    let body: Vec<Body> = Vec::new();
    proof {
        assert(body_models(body@) =~= Seq::<ckc_spec::v1text::BodyItem>::empty());
    }
    let fact = Clause { head: record, body };
    proof {
        assert(fact@ == ckc_spec::engine::fact_clause(
            spec::supersession_record(docid@, s as nat, sup@),
        ));
    }
    let mut records = Vec::new();
    records.push(fact);
    proof {
        assert_seqs_equal!(clause_models(records@) == seq![ckc_spec::engine::fact_clause(spec::supersession_record(docid@, s as nat, sup@))]);
    }
    Ok((groups, records))
}

} // verus!
