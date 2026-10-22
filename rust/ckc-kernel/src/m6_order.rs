// Binding-first body order (contract bf): the certifier's exec twin of
// `emit::bind_first` + the order theorems the contract names.
use crate::compare_schema::ascii_eq;
use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_model::*;
use crate::m6_safety::positive_terms;
#[cfg(verus_keep_ghost)]
use crate::m6_safety::{positive_terms_model, set_concat};
use crate::m6_term::*;
use crate::m6_vars::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text::BodyItem;
use vstd::prelude::*;
use vstd::{assert_seqs_equal, assert_sets_equal};

verus! {

// The union of what the items before position j bind.
pub open spec fn bound_before(s: Seq<BodyItem>, j: int) -> Set<nat>
    decreases j,
{
    if j <= 0 || s.len() == 0 {
        Set::empty()
    } else if j > s.len() {
        bound_before(s, s.len() as int)
    } else {
        bound_before(s, j - 1).union(spec::binds(s[j - 1]))
    }
}

// A NAF item or a built-in goal: the items whose early run changes an answer.
pub open spec fn moded(it: BodyItem) -> bool {
    match it {
        BodyItem::Naf(_) => true,
        BodyItem::Pos(t) => spec::is_builtin_lit(t),
    }
}

// Each moded item is ready where it stands.
pub open spec fn moded_ready(s: Seq<BodyItem>, shared: Set<nat>) -> bool {
    forall|j: int|
        0 <= j < s.len() && moded(#[trigger] s[j]) ==> spec::ready(s[j], bound_before(s, j), shared)
}

fn prefix_terms(arena: &ETermArena, args: &Vec<T>, k: usize) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, args@),
        k <= args.len(),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == models(args@).take(k as int),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(models(out@) == models(args@).take(0));
    }
    while i < k
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, args@),
            k <= args.len(),
            i <= k,
            valid_all(arena.nodes@, out@),
            models(out@) == models(args@).take(i as int),
        decreases k - i,
    {
        let next = args[i].cp();
        proof {
            extend(arena.nodes@, out@, next);
            assert(models(args@).take(i as int + 1) == models(args@).take(i as int).push(
                models(args@)[i as int],
            ));
        }
        out.push(next);
        i += 1;
    }
    out
}

fn waited(arena: &ETermArena, t: &T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::waited(t@),
{
    proof {
        reveal_strlit("guideline_satisfies");
        reveal_strlit("guideline_entity");
        reveal_strlit("guideline_event");
    }
    match parts(arena, t) {
        None => {
            let out = Vec::new();
            proof {
                assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
            }
            out
        },
        Some((name, args)) => {
            let n = args.len();
            if (n == 4 || n == 6) && ascii_eq(&name, "guideline_satisfies") {
                prefix_terms(
                    arena,
                    &args,
                    if n == 4 {
                        2
                    } else {
                        3
                    },
                )
            } else if crate::compare_impl::is_arith_exec(&name, n) {
                prefix_terms(arena, &args, 4)
            } else if crate::k2_answers::semantic_name(&name, n) && n >= 2 && !ascii_eq(
                &name,
                "guideline_entity",
            ) && !ascii_eq(&name, "guideline_event") {
                let r = args[1].cp();
                let mut out = Vec::new();
                proof {
                    extend(arena.nodes@, out@, r);
                }
                out.push(r);
                proof {
                    assert_seqs_equal!(models(out@) == seq![models(args@)[1]]);
                }
                out
            } else {
                let out = Vec::new();
                proof {
                    assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
                }
                out
            }
        },
    }
}

// Every member of xs that shared holds is in bound.
fn shared_subset(xs: &Vec<usize>, shared: &Vec<usize>, bound: &Vec<usize>) -> (out: bool)
    ensures
        out == nums(xs@).to_set().intersect(nums(shared@).to_set()).subset_of(
            nums(bound@).to_set(),
        ),
{
    let mut i = 0usize;
    while i < xs.len()
        invariant
            i <= xs.len(),
            forall|j: int|
                0 <= j < i && nums(shared@).to_set().contains(#[trigger] nums(xs@)[j]) ==> nums(
                    bound@,
                ).to_set().contains(nums(xs@)[j]),
        decreases xs.len() - i,
    {
        if member(shared, xs[i]) && !member(bound, xs[i]) {
            proof {
                let x = nums(xs@)[i as int];
                assert(nums(xs@).to_set().contains(x));
                assert(nums(shared@).contains(x));
                assert(nums(shared@).to_set().contains(x));
                assert(!nums(bound@).contains(x));
                assert(!nums(bound@).to_set().contains(x));
            }
            return false;
        }
        proof {
            let x = nums(xs@)[i as int];
            if nums(shared@).to_set().contains(x) {
                assert(nums(shared@).contains(x));
                assert(nums(bound@).contains(x));
            }
        }
        i += 1;
    }
    proof {
        assert forall|x: nat| #[trigger]
            nums(xs@).to_set().intersect(nums(shared@).to_set()).contains(x) implies nums(
            bound@,
        ).to_set().contains(x) by {
            let j = choose|j: int| 0 <= j < nums(xs@).len() && nums(xs@)[j] == x;
            assert(nums(shared@).to_set().contains(nums(xs@)[j]));
        }
    }
    true
}

fn ready(arena: &ETermArena, it: &Body, bound: &Vec<usize>, shared: &Vec<usize>) -> (out: bool)
    requires
        arena_ok(arena),
        body_valid(arena.nodes@, it),
    ensures
        out == spec::ready(it@, nums(bound@).to_set(), nums(shared@).to_set()),
{
    match it {
        Body::Pos(t) => {
            let w = waited(arena, t);
            let vs = stream_all(arena, &w);
            subset(&vs, bound)
        },
        Body::Naf(gs) => {
            let vs = stream_all(arena, gs);
            shared_subset(&vs, shared, bound)
        },
    }
}

fn pick(arena: &ETermArena, items: &Vec<Body>, bound: &Vec<usize>, shared: &Vec<usize>) -> (out:
    usize)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, items@),
        items.len() > 0,
    ensures
        out < items.len(),
        out as nat == spec::pick(
            body_models(items@),
            nums(bound@).to_set(),
            nums(shared@).to_set(),
        ),
{
    let ghost m = body_models(items@);
    let ghost b = nums(bound@).to_set();
    let ghost s = nums(shared@).to_set();
    let mut i = 0usize;
    while i < items.len()
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, items@),
            m == body_models(items@),
            b == nums(bound@).to_set(),
            s == nums(shared@).to_set(),
            i <= items.len(),
            spec::ready_from(m, b, s, 0) == spec::ready_from(m, b, s, i as nat),
        decreases items.len() - i,
    {
        proof {
            assert(body_valid(arena.nodes@, &items@[i as int]));
            assert(m[i as int] == items@[i as int]@);
            reveal_with_fuel(spec::ready_from, 1);
        }
        if ready(arena, &items[i], bound, shared) {
            return i;
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::ready_from, 1);
    }
    0
}

pub fn bind_first(arena: &ETermArena, body: Vec<Body>) -> (out: Vec<Body>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, body@),
    ensures
        bodies_valid(arena.nodes@, out@),
        body_models(out@) == spec::bind_first(body_models(body@)),
{
    let pos = positive_terms(arena, &body);
    let shared = stream_all(arena, &pos);
    let ghost s = nums(shared@).to_set();
    let ghost goal = spec::bind_first(body_models(body@));
    proof {
        reveal(positive_terms_model);
        assert(s == spec::positive_vars(body_models(body@)));
    }
    let mut bound: Vec<usize> = Vec::new();
    let mut rest = body;
    let mut out: Vec<Body> = Vec::new();
    proof {
        assert_sets_equal!(nums(bound@).to_set() == Set::<nat>::empty());
        assert_seqs_equal!(body_models(out@) == Seq::<BodyItem>::empty());
    }
    while rest.len() > 0
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, rest@),
            bodies_valid(arena.nodes@, out@),
            s == nums(shared@).to_set(),
            body_models(out@) + spec::order_from(body_models(rest@), nums(bound@).to_set(), s)
                == goal,
        decreases rest.len(),
    {
        let i = pick(arena, &rest, &bound, &shared);
        let ghost m = body_models(rest@);
        let ghost b = nums(bound@).to_set();
        let ghost before = body_models(out@);
        proof {
            spec::ready_from_bound(m, b, s, 0);
            assert(body_valid(arena.nodes@, &rest@[i as int]));
        }
        let ghost rest0 = rest@;
        let it = rest.remove(i);
        let vars = match &it {
            Body::Pos(t) => stream(arena, t),
            Body::Naf(_) => Vec::new(),
        };
        let next = concat_nums(&bound, &vars);
        proof {
            assert(it@ == m[i as int]);
            set_concat(nums(bound@), nums(vars@));
            match it@ {
                BodyItem::Pos(_) => {},
                BodyItem::Naf(_) => {
                    assert_sets_equal!(nums(vars@).to_set() == Set::<nat>::empty());
                },
            }
            assert(nums(next@).to_set() == b.union(spec::binds(m[i as int])));
            assert_seqs_equal!(body_models(rest@) == m.remove(i as int));
            assert(bodies_valid(arena.nodes@, rest@)) by {
                assert forall|k: int| 0 <= k < rest@.len() implies #[trigger] body_valid(
                    arena.nodes@,
                    &rest@[k],
                ) by {
                    if k < i {
                        assert(rest@[k] == rest0[k]);
                    } else {
                        assert(rest@[k] == rest0[k + 1]);
                    }
                }
            }
            assert(spec::order_from(m, b, s) == seq![m[i as int]] + spec::order_from(
                m.remove(i as int),
                b.union(spec::binds(m[i as int])),
                s,
            ));
        }
        bound = next;
        out.push(it);
        proof {
            assert_seqs_equal!(body_models(out@) == before.push(m[i as int]));
            assert(before.push(m[i as int]) + spec::order_from(
                m.remove(i as int),
                nums(bound@).to_set(),
                s,
            ) == before + (seq![m[i as int]] + spec::order_from(
                m.remove(i as int),
                nums(bound@).to_set(),
                s,
            )));
        }
    }
    proof {
        assert(body_models(rest@) == Seq::<BodyItem>::empty());
        assert(body_models(out@) + Seq::<BodyItem>::empty() == body_models(out@));
    }
    out
}

pub proof fn order_from_permutation(items: Seq<BodyItem>, bound: Set<nat>, shared: Set<nat>)
    ensures
        spec::order_from(items, bound, shared).to_multiset() == items.to_multiset(),
    decreases items.len(),
{
    broadcast use vstd::multiset::group_multiset_axioms;
    broadcast use vstd::seq_lib::group_to_multiset_ensures;

    if items.len() == 0 {
        assert(spec::order_from(items, bound, shared) == Seq::<BodyItem>::empty());
    } else {
        spec::ready_from_bound(items, bound, shared, 0);
        let i = spec::pick(items, bound, shared) as int;
        let x = items[i];
        let rest = items.remove(i);
        let tail = spec::order_from(rest, bound.union(spec::binds(x)), shared);
        order_from_permutation(rest, bound.union(spec::binds(x)), shared);
        let o = seq![x] + tail;
        assert(spec::order_from(items, bound, shared) == o);
        assert(o =~= tail.insert(0, x));
        vstd::seq_lib::to_multiset_insert(tail, 0, x);
        vstd::seq_lib::to_multiset_remove(items, i);
        assert(items.to_multiset().count(x) > 0) by {
            assert(items.contains(x));
        }
        assert(o.to_multiset() =~= items.to_multiset());
    }
}

pub proof fn bind_first_permutation(b: Seq<BodyItem>)
    ensures
        spec::bind_first(b).to_multiset() == b.to_multiset(),
{
    order_from_permutation(b, Set::empty(), spec::positive_vars(b));
}

proof fn bound_before_step(s: Seq<BodyItem>, j: int)
    requires
        0 <= j < s.len(),
    ensures
        bound_before(s, j + 1) == bound_before(s, j).union(spec::binds(s[j])),
{
}

// The suffix from k runs in place once its items are ready in place.
proof fn order_from_stable(b: Seq<BodyItem>, k: int, shared: Set<nat>)
    requires
        0 <= k <= b.len(),
        forall|i: int|
            0 <= i < b.len() ==> spec::ready(#[trigger] b[i], bound_before(b, i), shared),
    ensures
        spec::order_from(b.skip(k), bound_before(b, k), shared) == b.skip(k),
    decreases b.len() - k,
{
    if k < b.len() {
        let items = b.skip(k);
        reveal_with_fuel(spec::ready_from, 1);
        assert(items[0] == b[k]);
        assert(spec::ready(items[0], bound_before(b, k), shared));
        assert(spec::pick(items, bound_before(b, k), shared) == 0);
        assert(items.remove(0) == b.skip(k + 1));
        bound_before_step(b, k);
        order_from_stable(b, k + 1, shared);
        assert(items == seq![b[k]] + b.skip(k + 1));
    } else {
        assert(b.skip(k) == Seq::<BodyItem>::empty());
    }
}

pub proof fn bind_first_stable(b: Seq<BodyItem>)
    requires
        forall|i: int|
            0 <= i < b.len() ==> spec::ready(
                #[trigger] b[i],
                bound_before(b, i),
                spec::positive_vars(b),
            ),
    ensures
        spec::bind_first(b) == b,
{
    order_from_stable(b, 0, spec::positive_vars(b));
    assert(b.skip(0) == b);
}

proof fn ready_mono(it: BodyItem, b1: Set<nat>, b2: Set<nat>, shared: Set<nat>)
    requires
        spec::ready(it, b1, shared),
        b1.subset_of(b2),
    ensures
        spec::ready(it, b2, shared),
{
}

// x is bound before j iff an item before j binds it.
proof fn bound_before_has(s: Seq<BodyItem>, j: int, x: nat)
    requires
        0 <= j <= s.len(),
    ensures
        bound_before(s, j).contains(x) <==> exists|q: int|
            0 <= q < j && #[trigger] spec::binds(s[q]).contains(x),
    decreases j,
{
    if j > 0 {
        bound_before_has(s, j - 1, x);
        bound_before_step(s, j - 1);
        if bound_before(s, j).contains(x) && !bound_before(s, j - 1).contains(x) {
            assert(spec::binds(s[j - 1]).contains(x));
        }
    }
}

// The binders before an item, after another item leaves, are bound by it or stay before.
proof fn bound_before_remove(s: Seq<BodyItem>, i: int, p: int, bound: Set<nat>)
    requires
        0 <= i < s.len(),
        0 <= p < s.len(),
        p != i,
    ensures
        bound.union(bound_before(s, p)).subset_of(
            bound.union(spec::binds(s[i])).union(
                bound_before(
                    s.remove(i),
                    if p < i {
                        p
                    } else {
                        p - 1
                    },
                ),
            ),
        ),
{
    let r = s.remove(i);
    let p2 = if p < i {
        p
    } else {
        p - 1
    };
    assert forall|x: nat| #[trigger]
        bound.union(bound_before(s, p)).contains(x) implies bound.union(spec::binds(s[i])).union(
        bound_before(r, p2),
    ).contains(x) by {
        if !bound.contains(x) {
            bound_before_has(s, p, x);
            let q = choose|q: int| 0 <= q < p && #[trigger] spec::binds(s[q]).contains(x);
            if q != i {
                let q2 = if q < i {
                    q
                } else {
                    q - 1
                };
                assert(r[q2] == s[q]);
                assert(spec::binds(r[q2]).contains(x));
                bound_before_has(r, p2, x);
            }
        }
    }
}

// Every moded item of order_from's output is ready where it stands.
proof fn order_from_moded(items: Seq<BodyItem>, bound: Set<nat>, shared: Set<nat>)
    requires
        forall|p: int|
            0 <= p < items.len() && moded(#[trigger] items[p]) ==> spec::ready(
                items[p],
                bound.union(bound_before(items, p)),
                shared,
            ),
    ensures
        forall|j: int|
            0 <= j < spec::order_from(items, bound, shared).len() && moded(
                #[trigger] spec::order_from(items, bound, shared)[j],
            ) ==> spec::ready(
                spec::order_from(items, bound, shared)[j],
                bound.union(bound_before(spec::order_from(items, bound, shared), j)),
                shared,
            ),
    decreases items.len(),
{
    if items.len() > 0 {
        spec::ready_from_bound(items, bound, shared, 0);
        let i = spec::pick(items, bound, shared) as int;
        let x = items[i];
        let rest = items.remove(i);
        let bound2 = bound.union(spec::binds(x));
        let tail = spec::order_from(rest, bound2, shared);
        let o = seq![x] + tail;
        assert(spec::order_from(items, bound, shared) == o);
        // x itself: chosen ready, or the leftmost item when none is ready.
        if moded(x) {
            match spec::ready_from(items, bound, shared, 0) {
                Option::Some(_) => {
                    ready_from_ready(items, bound, shared, 0);
                },
                Option::None => {
                    assert(i == 0);
                    assert(bound_before(items, 0) == Set::<nat>::empty());
                    assert(bound.union(Set::<nat>::empty()) =~= bound);
                },
            }
            assert(spec::ready(x, bound, shared));
        }
        // The rest keeps the premise under the larger bound.

        assert forall|p2: int|
            0 <= p2 < rest.len() && moded(#[trigger] rest[p2]) implies spec::ready(
            rest[p2],
            bound2.union(bound_before(rest, p2)),
            shared,
        ) by {
            let p = if p2 < i {
                p2
            } else {
                p2 + 1
            };
            assert(rest[p2] == items[p]);
            bound_before_remove(items, i, p, bound);
            assert(bound.union(spec::binds(items[i])).union(bound_before(rest, p2))
                =~= bound2.union(bound_before(rest, p2)));
            ready_mono(
                items[p],
                bound.union(bound_before(items, p)),
                bound2.union(bound_before(rest, p2)),
                shared,
            );
        }
        order_from_moded(rest, bound2, shared);
        assert forall|j: int| 0 <= j < o.len() && moded(#[trigger] o[j]) implies spec::ready(
            o[j],
            bound.union(bound_before(o, j)),
            shared,
        ) by {
            if j == 0 {
                assert(bound_before(o, 0) == Set::<nat>::empty());
                assert(bound.union(Set::<nat>::empty()) =~= bound);
            } else {
                assert(o[j] == tail[j - 1]);
                bound_before_cons(x, tail, j);
                assert(bound.union(bound_before(o, j)) =~= bound2.union(bound_before(tail, j - 1)));
            }
        }
    }
}

proof fn ready_from_ready(items: Seq<BodyItem>, bound: Set<nat>, shared: Set<nat>, from: nat)
    requires
        spec::ready_from(items, bound, shared, from) is Some,
    ensures
        spec::ready(
            items[spec::ready_from(items, bound, shared, from)->Some_0 as int],
            bound,
            shared,
        ),
    decreases items.len() - from,
{
    if from < items.len() && !spec::ready(items[from as int], bound, shared) {
        ready_from_ready(items, bound, shared, from + 1);
    }
}

// Binders before position j of x + tail = x's binds + binders before j - 1 of tail.
proof fn bound_before_cons(x: BodyItem, tail: Seq<BodyItem>, j: int)
    requires
        1 <= j <= tail.len() + 1,
    ensures
        bound_before(seq![x] + tail, j) == spec::binds(x).union(bound_before(tail, j - 1)),
    decreases j,
{
    let o = seq![x] + tail;
    if j == 1 {
        assert(bound_before(o, 1) == bound_before(o, 0).union(spec::binds(o[0])));
        assert(bound_before(tail, 0) == Set::<nat>::empty());
        assert(Set::<nat>::empty().union(spec::binds(x)) =~= spec::binds(x).union(
            Set::<nat>::empty(),
        ));
    } else {
        bound_before_cons(x, tail, j - 1);
        bound_before_step(o, j - 1);
        bound_before_step(tail, j - 2);
        assert(o[j - 1] == tail[j - 2]);
        assert(bound_before(o, j) =~= spec::binds(x).union(bound_before(tail, j - 1)));
    }
}

pub proof fn bind_first_moded(b: Seq<BodyItem>)
    requires
        moded_ready(b, spec::positive_vars(b)),
    ensures
        moded_ready(spec::bind_first(b), spec::positive_vars(b)),
{
    let s = spec::positive_vars(b);
    assert forall|p: int| 0 <= p < b.len() && moded(#[trigger] b[p]) implies spec::ready(
        b[p],
        Set::<nat>::empty().union(bound_before(b, p)),
        s,
    ) by {
        assert(Set::<nat>::empty().union(bound_before(b, p)) =~= bound_before(b, p));
    }
    order_from_moded(b, Set::empty(), s);
    let o = spec::bind_first(b);
    assert forall|j: int| 0 <= j < o.len() && moded(#[trigger] o[j]) implies spec::ready(
        o[j],
        bound_before(o, j),
        s,
    ) by {
        assert(Set::<nat>::empty().union(bound_before(o, j)) =~= bound_before(o, j));
    }
}

} // verus!
