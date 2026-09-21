#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_model::*;
use crate::m6_symbols::Sym;
use crate::m6_term::*;
use crate::m6_vars::*;
use ckc_spec::emit as spec;
use ckc_spec::term::{self, Term};
use ckc_spec::v1text::{self, BodyItem, DocClause};
use vstd::prelude::*;
use vstd::{assert_seqs_equal, assert_sets_equal};

verus! {

pub proof fn set_concat(a: Seq<nat>, b: Seq<nat>)
    ensures
        (a + b).to_set() == a.to_set().union(b.to_set()),
{
    a.to_set_ensures();
    b.to_set_ensures();
    (a + b).to_set_ensures();
    assert_sets_equal!((a + b).to_set() == a.to_set().union(b.to_set()), x => {
        if (a + b).to_set().contains(x) {
            let j = choose|j: int| 0 <= j < (a + b).len() && (a + b)[j] == x;
            if j < a.len() { assert(a[j] == x); } else { assert(b[j - a.len()] == x); }
        }
        if a.to_set().contains(x) {
            let j = choose|j: int| 0 <= j < a.len() && a[j] == x;
            assert((a + b)[j] == x);
        }
        if b.to_set().contains(x) {
            let j = choose|j: int| 0 <= j < b.len() && b[j] == x;
            assert((a + b)[a.len() + j] == x);
        }
    });
}

pub proof fn occurrences_concat(a: Seq<nat>, b: Seq<nat>, x: nat)
    ensures
        occurrences(a + b, x) == occurrences(a, x) + occurrences(b, x),
    decreases b.len(),
{
    if b.len() > 0 {
        assert(b == b.drop_last().push(b.last()));
        assert(a + b == (a + b.drop_last()).push(b.last()));
        occurrences_concat(a, b.drop_last(), x);
        occurrence_step(a + b.drop_last(), b.last(), x);
        occurrence_step(b.drop_last(), b.last(), x);
    } else {
        reveal(occurrences);
        reveal(Seq::filter);
    }
}

pub proof fn occ_bridge(t: Term, x: nat)
    ensures
        occurrences(term::var_stream(t), x) == spec::occ(x, t),
{
    reveal(occurrences);
    reveal(spec::occ);
}

pub proof fn occ_all_bridge(ts: Seq<Term>, x: nat)
    ensures
        occurrences(term::var_stream_all(ts), x) == spec::occ_all(x, ts),
{
    reveal(occurrences);
    reveal(spec::occ_all);
}

pub proof fn positive_step(bs: Seq<BodyItem>, b: BodyItem)
    ensures
        spec::positive_goals(bs.push(b)) == match b {
            BodyItem::Pos(t) => spec::positive_goals(bs).push(t),
            BodyItem::Naf(_) => spec::positive_goals(bs),
        },
{
    reveal(Seq::filter);
    reveal(spec::positive_goals);
    reveal(spec::pos_terms);
    assert(bs.push(b).drop_last() == bs);
    assert(bs.push(b).last() == b);
    match b {
        BodyItem::Pos(t) => {
            assert_seqs_equal!(spec::positive_goals(bs.push(b)) == spec::positive_goals(bs).push(t));
        },
        BodyItem::Naf(_) => {
            assert_seqs_equal!(spec::positive_goals(bs.push(b)) == spec::positive_goals(bs));
        },
    }
}

pub fn positive_goals(arena: &ETermArena, bs: &Vec<Body>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, bs@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::positive_goals(body_models(bs@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        reveal(Seq::filter);
        reveal(spec::positive_goals);
        reveal(spec::pos_terms);
    }
    while i < bs.len()
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, bs@),
            valid_all(arena.nodes@, out@),
            i <= bs.len(),
            models(out@) == spec::positive_goals(body_models(bs@).take(i as int)),
        decreases bs.len() - i,
    {
        proof {
            assert(body_valid(arena.nodes@, &bs@[i as int]));
            assert(body_models(bs@).take(i as int + 1) == body_models(bs@).take(i as int).push(
                body_models(bs@)[i as int],
            ));
            positive_step(body_models(bs@).take(i as int), body_models(bs@)[i as int]);
        }
        if let Body::Pos(t) = &bs[i] {
            let next = t.cp();
            proof {
                extend(arena.nodes@, out@, next);
            }
            out.push(next);
        }
        i += 1;
    }
    proof {
        assert(body_models(bs@).take(i as int) == body_models(bs@));
    }
    out
}

pub open spec fn naf_stream(bs: Seq<BodyItem>) -> Seq<nat>
    decreases bs.len(),
{
    if bs.len() == 0 {
        Seq::empty()
    } else {
        (match bs[0] {
            BodyItem::Pos(_) => Seq::empty(),
            BodyItem::Naf(ts) => term::var_stream_all(ts),
        }) + naf_stream(bs.drop_first())
    }
}

pub fn naf_vars(arena: &ETermArena, bs: &Vec<Body>) -> (out: Vec<usize>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, bs@),
    ensures
        nums(out@) == naf_stream(body_models(bs@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(body_models(bs@).skip(0) == body_models(bs@));
        assert_seqs_equal!(nums(out@) == Seq::<nat>::empty());
    }
    while i < bs.len()
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, bs@),
            i <= bs.len(),
            nums(out@) + naf_stream(body_models(bs@).skip(i as int)) == naf_stream(
                body_models(bs@),
            ),
        decreases bs.len() - i,
    {
        proof {
            assert(body_valid(arena.nodes@, &bs@[i as int]));
            assert(body_models(bs@).skip(i as int).drop_first() == body_models(bs@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(naf_stream, 1);
        }
        if let Body::Naf(ts) = &bs[i] {
            let next = stream_all(arena, ts);
            out = concat_nums(&out, &next);
        }
        i += 1;
    }
    proof {
        reveal(naf_stream);
    }
    out
}

pub proof fn naf_count(bs: Seq<BodyItem>, x: nat)
    ensures
        occurrences(naf_stream(bs), x) == spec::naf_occ(x, bs),
    decreases bs.len(),
{
    reveal_with_fuel(naf_stream, 1);
    reveal_with_fuel(spec::naf_occ, 1);
    if bs.len() > 0 {
        naf_count(bs.drop_first(), x);
        match bs[0] {
            BodyItem::Pos(_) => {},
            BodyItem::Naf(ts) => {
                occurrences_concat(term::var_stream_all(ts), naf_stream(bs.drop_first()), x);
                occ_all_bridge(ts, x);
            },
        }
    } else {
        reveal(occurrences);
        reveal(Seq::filter);
    }
}

pub open spec fn clause_stream(c: DocClause) -> Seq<nat> {
    term::var_stream(c.head) + term::var_stream_all(spec::positive_goals(c.body)) + naf_stream(
        c.body,
    )
}

pub fn clause_vars(arena: &ETermArena, c: &Clause) -> (out: Vec<usize>)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, c),
    ensures
        nums(out@) == clause_stream(c@),
{
    let hs = stream(arena, &c.head);
    let pos = positive_goals(arena, &c.body);
    let ps = stream_all(arena, &pos);
    let ns = naf_vars(arena, &c.body);
    let hp = concat_nums(&hs, &ps);
    concat_nums(&hp, &ns)
}

pub proof fn clause_count(c: DocClause, x: nat)
    ensures
        occurrences(clause_stream(c), x) == spec::clause_occ(x, c),
{
    occurrences_concat(
        term::var_stream(c.head) + term::var_stream_all(spec::positive_goals(c.body)),
        naf_stream(c.body),
        x,
    );
    occurrences_concat(
        term::var_stream(c.head),
        term::var_stream_all(spec::positive_goals(c.body)),
        x,
    );
    occ_bridge(c.head, x);
    occ_all_bridge(spec::positive_goals(c.body), x);
    naf_count(c.body, x);
    reveal(spec::clause_occ);
    reveal(spec::positive_goals);
}

pub fn head_safe(arena: &ETermArena, c: &Clause) -> (out: bool)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, c),
    ensures
        out == spec::head_safe(c@),
{
    let h = stream(arena, &c.head);
    let goals = positive_goals(arena, &c.body);
    let p = stream_all(arena, &goals);
    subset(&h, &p)
}

pub fn all_head_safe(arena: &ETermArena, cs: &Vec<Clause>) -> (out: bool)
    requires
        arena_ok(arena),
        clauses_valid(arena.nodes@, cs@),
    ensures
        out == spec::all_head_safe(clause_models(cs@)),
{
    let mut i = 0usize;
    while i < cs.len()
        invariant
            arena_ok(arena),
            clauses_valid(arena.nodes@, cs@),
            i <= cs.len(),
            forall|j: int| 0 <= j < i ==> spec::head_safe(#[trigger] clause_models(cs@)[j]),
        decreases cs.len() - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &cs@[i as int]));
        }
        if !head_safe(arena, &cs[i]) {
            proof {
                assert(!spec::head_safe(clause_models(cs@)[i as int]));
                assert(!spec::all_head_safe(clause_models(cs@)));
            }
            return false;
        }
        i += 1;
    }
    true
}

pub fn confined(arena: &ETermArena, c: &Clause, locals: &Vec<T>, goals: &Vec<T>) -> (out: bool)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, c),
        valid_all(arena.nodes@, locals@),
        valid_all(arena.nodes@, goals@),
    ensures
        out == (forall|x: nat|
            spec::dom_vars(models(locals@)).contains(x) ==> #[trigger] spec::clause_occ(x, c@)
                == spec::occ_all(x, models(goals@))),
{
    let ls = stream_all(arena, locals);
    let gs = stream_all(arena, goals);
    let cs = clause_vars(arena, c);
    let mut i = 0usize;
    while i < ls.len()
        invariant
            arena_ok(arena),
            clause_valid(arena.nodes@, c),
            valid_all(arena.nodes@, locals@),
            valid_all(arena.nodes@, goals@),
            nums(ls@) == term::var_stream_all(models(locals@)),
            nums(gs@) == term::var_stream_all(models(goals@)),
            nums(cs@) == clause_stream(c@),
            i <= ls.len(),
            forall|j: int|
                0 <= j < i ==> spec::clause_occ(#[trigger] nums(ls@)[j], c@) == spec::occ_all(
                    nums(ls@)[j],
                    models(goals@),
                ),
        decreases ls.len() - i,
    {
        let x = ls[i];
        let all_count = count(&cs, x);
        let goal_count = count(&gs, x);
        proof {
            clause_count(c@, x as nat);
            occ_all_bridge(models(goals@), x as nat);
        }
        if all_count != goal_count {
            proof {
                assert(nums(ls@)[i as int] == x as nat);
                nums(ls@).to_set_ensures();
                assert(spec::dom_vars(models(locals@)).contains(x as nat));
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert forall|x: nat|
            spec::dom_vars(models(locals@)).contains(x) implies #[trigger] spec::clause_occ(x, c@)
            == spec::occ_all(x, models(goals@)) by {
            let j = choose|j: int| 0 <= j < nums(ls@).len() && nums(ls@)[j] == x;
            assert(spec::clause_occ(nums(ls@)[j], c@) == spec::occ_all(
                nums(ls@)[j],
                models(goals@),
            ));
        }
    }
    true
}

pub open spec fn dom_models(ds: Seq<Vec<T>>) -> Seq<Seq<Term>> {
    ds.map_values(|d: Vec<T>| models(d@))
}

pub open spec fn doms_valid(nodes: Seq<ENode>, ds: Seq<Vec<T>>) -> bool {
    forall|i: int| 0 <= i < ds.len() ==> #[trigger] valid_all(nodes, ds[i]@)
}

pub fn naf_safe_from(
    arena: &ETermArena,
    c: &Clause,
    ds: &Vec<Vec<T>>,
    i: usize,
    bound: &Vec<usize>,
) -> (out: bool)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, c),
        doms_valid(arena.nodes@, ds@),
        ds.len() == c.body.len(),
        i <= c.body.len(),
    ensures
        out == spec::naf_safe(c@, dom_models(ds@), i as nat, nums(bound@).to_set()),
    decreases c.body.len() - i,
{
    proof {
        reveal_with_fuel(spec::naf_safe, 1);
    }
    if i == c.body.len() {
        return true;
    }
    proof {
        assert(body_valid(arena.nodes@, &c.body@[i as int]));
        assert(valid_all(arena.nodes@, ds@[i as int]@));
    }
    match &c.body[i] {
        Body::Pos(t) => {
            let vars = stream(arena, t);
            let next = concat_nums(bound, &vars);
            proof {
                set_concat(nums(bound@), nums(vars@));
            }
            naf_safe_from(arena, c, ds, i + 1, &next)
        },
        Body::Naf(gs) => {
            let goals = stream_all(arena, gs);
            let locals = stream_all(arena, &ds[i]);
            let allowed = concat_nums(&locals, bound);
            proof {
                set_concat(nums(locals@), nums(bound@));
            }
            subset(&goals, &allowed) && confined(arena, c, &ds[i], gs) && naf_safe_from(
                arena,
                c,
                ds,
                i + 1,
                bound,
            )
        },
    }
}

pub fn all_naf_safe(arena: &ETermArena, cs: &Vec<Clause>, ds: &Vec<Vec<T>>) -> (out: bool)
    requires
        arena_ok(arena),
        clauses_valid(arena.nodes@, cs@),
        doms_valid(arena.nodes@, ds@),
        forall|i: int| 0 <= i < cs.len() ==> (#[trigger] cs@[i]).body.len() == ds.len(),
    ensures
        out == spec::all_naf_safe(clause_models(cs@), dom_models(ds@)),
{
    let bound = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(nums(bound@) == Seq::<nat>::empty());
        nums(bound@).to_set_ensures();
        assert_sets_equal!(nums(bound@).to_set() == Set::<nat>::empty());
    }
    while i < cs.len()
        invariant
            arena_ok(arena),
            clauses_valid(arena.nodes@, cs@),
            doms_valid(arena.nodes@, ds@),
            forall|j: int| 0 <= j < cs.len() ==> (#[trigger] cs@[j]).body.len() == ds.len(),
            nums(bound@).to_set() == Set::<nat>::empty(),
            i <= cs.len(),
            forall|j: int|
                0 <= j < i ==> spec::naf_safe(
                    #[trigger] clause_models(cs@)[j],
                    dom_models(ds@),
                    0,
                    Set::empty(),
                ),
        decreases cs.len() - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &cs@[i as int]));
        }
        if !naf_safe_from(arena, &cs[i], ds, 0, &bound) {
            proof {
                assert(!spec::naf_safe(
                    clause_models(cs@)[i as int],
                    dom_models(ds@),
                    0,
                    Set::empty(),
                ));
                assert(!spec::all_naf_safe(clause_models(cs@), dom_models(ds@)));
            }
            return false;
        }
        i += 1;
    }
    true
}

pub proof fn participants_len(
    ctx: Term,
    e: Term,
    args: Seq<Term>,
    pos: nat,
    map: Seq<(Term, Term)>,
    sko: Seq<(Term, Term)>,
)
    ensures
        spec::participants(ctx, e, args, pos, map, sko) matches Ok(ts) ==> ts.len() == args.len(),
    decreases args.len(),
{
    if args.len() > 0 {
        participants_len(ctx, e, args.drop_first(), pos + 1, map, sko);
    }
    reveal_with_fuel(spec::participants, 1);
}

pub proof fn condition_len(ctx: Term, inner: Term, map: Seq<(Term, Term)>, sko: Seq<(Term, Term)>)
    ensures
        spec::condition(ctx, inner, map, sko) matches Ok(ts) ==> ts.len() == spec::expanded_len(
            inner,
        ),
{
    reveal(spec::condition);
    if let Term::Comp(name, args) = inner {
        if name == v1text::ascii("predicate"@) && 3 <= args.len() <= 5 {
            if let Ok(e) = spec::resolve(args[0], map, sko) {
                participants_len(ctx, e, args.skip(2), 1, map, sko);
            }
        }
    }
}

pub proof fn expand_item_len(it: spec::Item, map: Seq<(Term, Term)>, sko: Seq<(Term, Term)>)
    ensures
        spec::expand_item(it, map, sko) matches Ok(bs) ==> bs.len() == spec::body_doms(
            seq![it],
        ).len(),
{
    reveal(spec::expand_item);
    reveal_with_fuel(spec::body_doms, 2);
    if let spec::Item::Anch(ctx, inner) = it {
        condition_len(ctx, inner, map, sko);
    }
}

pub proof fn expand_len(items: Seq<spec::Item>, map: Seq<(Term, Term)>, sko: Seq<(Term, Term)>)
    ensures
        spec::expand(items, map, sko) matches Ok(bs) ==> bs.len() == spec::body_doms(items).len(),
    decreases items.len(),
{
    if items.len() > 0 {
        expand_item_len(items[0], map, sko);
        expand_len(items.drop_first(), map, sko);
    }
    reveal_with_fuel(spec::expand, 1);
    reveal_with_fuel(spec::body_doms, 2);
}

pub fn expanded_len(arena: &ETermArena, inner: &T) -> (out: usize)
    requires
        arena_ok(arena),
        valid(arena.nodes@, inner),
    ensures
        out as nat == spec::expanded_len(inner@),
{
    match parts(arena, inner) {
        Some((name, args)) => {
            if has_name(&name, &Sym::Object) && args.len() == 6 {
                2
            } else if has_name(&name, &Sym::Predicate) && args.len() >= 3 {
                args.len() - 1
            } else {
                1
            }
        },
        None => 1,
    }
}

pub fn empty_doms(n: usize) -> (out: Vec<Vec<T>>)
    ensures
        dom_models(out@) == Seq::new(n as nat, |i: int| Seq::<Term>::empty()),
        forall|nodes: Seq<ENode>| #[trigger] doms_valid(nodes, out@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < n
        invariant
            i <= n,
            out.len() == i,
            dom_models(out@) == Seq::new(i as nat, |j: int| Seq::<Term>::empty()),
            forall|nodes: Seq<ENode>| #[trigger] doms_valid(nodes, out@),
        decreases n - i,
    {
        let ghost before = out@;
        let empty: Vec<T> = Vec::new();
        proof {
            assert_seqs_equal!(models(empty@) == Seq::<Term>::empty());
        }
        out.push(empty);
        proof {
            assert_seqs_equal!(dom_models(out@) == Seq::new(i as nat + 1, |j: int| Seq::<Term>::empty()), j => {
                if j < before.len() { assert(out@[j] == before[j]); assert(dom_models(before)[j] == Seq::<Term>::empty()); }
                else { assert(j == i); assert(out@[j] == empty); }
            });
            assert forall|nodes: Seq<ENode>| #[trigger] doms_valid(nodes, out@) by {
                assert(doms_valid(nodes, before));
                assert forall|j: int| 0 <= j < out.len() implies #[trigger] valid_all(
                    nodes,
                    out@[j]@,
                ) by {
                    if j < before.len() {
                        assert(out@[j] == before[j]);
                    }
                }
            }
        }
        i += 1;
    }
    out
}

pub fn item_doms(arena: &ETermArena, it: &I) -> (out: Vec<Vec<T>>)
    requires
        arena_ok(arena),
        item_valid(arena.nodes@, it),
    ensures
        doms_valid(arena.nodes@, out@),
        dom_models(out@) == spec::body_doms(seq![it@]),
{
    proof {
        reveal(item_valid);
        reveal_with_fuel(spec::body_doms, 2);
    }
    match &it.kind {
        Kind::Anch(_, inner) => {
            let n = expanded_len(arena, inner);
            empty_doms(n)
        },
        Kind::Op { .. } => empty_doms(1),
        Kind::Naf { dom, .. } => {
            let mut out = Vec::new();
            out.push(copy(dom));
            proof {
                assert_seqs_equal!(dom_models(out@) == seq![models(dom@)]);
            }
            out
        },
    }
}

pub fn body_doms(arena: &ETermArena, items: &Vec<I>) -> (out: Vec<Vec<T>>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        doms_valid(arena.nodes@, out@),
        dom_models(out@) == spec::body_doms(item_models(items@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
        assert_seqs_equal!(dom_models(out@) == Seq::<Seq<Term>>::empty());
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            doms_valid(arena.nodes@, out@),
            i <= items.len(),
            dom_models(out@) + spec::body_doms(item_models(items@).skip(i as int))
                == spec::body_doms(item_models(items@)),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::body_doms, 2);
        }
        let mut next = item_doms(arena, &items[i]);
        let ghost before = out@;
        let ghost added = next@;
        out.append(&mut next);
        proof {
            assert_seqs_equal!(dom_models(out@) == dom_models(before) + dom_models(added));
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] valid_all(
                arena.nodes@,
                out@[j]@,
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                } else {
                    assert(out@[j] == added[j - before.len()]);
                }
            }
        }
        i += 1;
    }
    proof {
        reveal(spec::body_doms);
    }
    out
}

} // verus!
