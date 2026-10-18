use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_compare_refs::{conv_refs, cvar, fresh_of, is_object_cmp, object_of, time_refs};
use crate::m6_model::*;
use crate::m6_refs::contains;
use crate::m6_symbols::Sym;
use crate::m6_term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::arg;
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn one_item(arena: &ETermArena, it: I) -> (out: Vec<I>)
    requires
        arena_ok(arena),
        item_valid(arena.nodes@, &it),
    ensures
        items_valid(arena.nodes@, out@),
        item_models(out@) == seq![it@],
{
    let mut out = Vec::new();
    out.push(it);
    proof {
        reveal_with_fuel(items_valid, 2);
        assert_seqs_equal!(item_models(out@) == seq![it@]);
    }
    out
}

pub fn cvars(arena: &mut ETermArena, cb: usize, refs: &Vec<T>, r: &T) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, refs@),
        valid(old(arena).nodes@, r),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        out.len() == 3,
        models(out@) == seq![
            spec::cvar(cb as nat, models(refs@), r@, 0),
            spec::cvar(cb as nat, models(refs@), r@, 1),
            spec::cvar(cb as nat, models(refs@), r@, 2),
        ],
{
    let rs = crate::m6_expand::one_term(arena, r.cp());
    let out = fresh_of(arena, cb, refs, &rs);
    proof {
        reveal_with_fuel(spec::fresh_of, 2);
        assert(models(out@).len() == 3);
        reveal(models);
    }
    out
}

pub fn relink(arena: &mut ETermArena, times: &Vec<T>, refs: &Vec<T>, cb: usize, inner: &T) -> (out:
    T)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, times@),
        valid_all(old(arena).nodes@, refs@),
        valid(old(arena).nodes@, inner),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::relink(models(times@), models(refs@), cb as nat, inner@),
{
    let ghost start = arena.nodes@;
    if is_comp(arena, inner, &Sym::DollarGuidelineInterval, 5) {
        let a = args(arena, inner);
        if contains(arena, times, &a[2]) {
            let u = cvar(arena, cb, refs, &a[2], 2);
            let ghost n1 = arena.nodes@;
            proof {
                prefix_all(start, n1, a@);
            }
            let mut ts = Vec::new();
            ts.push(a[0].cp());
            ts.push(a[1].cp());
            ts.push(a[2].cp());
            ts.push(u);
            ts.push(a[4].cp());
            proof {
                assert_seqs_equal!(models(ts@) == seq![a[0]@, a[1]@, a[2]@, u@, a[4]@]);
            }
            let out = c(arena, &Sym::DollarGuidelineInterval, &ts);
            proof {
                crate::k2_load::prefix_chain(start, n1, arena.nodes@);
            }
            return out;
        }
    } else if is_comp(arena, inner, &Sym::DollarGuidelineRecurrence, 3) {
        let a = args(arena, inner);
        if contains(arena, times, &a[1]) {
            let u = cvar(arena, cb, refs, &a[1], 2);
            let ghost n1 = arena.nodes@;
            proof {
                prefix_all(start, n1, a@);
            }
            let out = c3(arena, &Sym::DollarGuidelineRecurrence, &a[0], &a[1], &u);
            proof {
                crate::k2_load::prefix_chain(start, n1, arena.nodes@);
            }
            return out;
        }
    } else if is_comp(arena, inner, &Sym::DollarGuidelineRecurrenceWindow, 6) {
        let a = args(arena, inner);
        let gap = if contains(arena, times, &a[1]) {
            cvar(arena, cb, refs, &a[1], 2)
        } else {
            a[2].cp()
        };
        let ghost n1 = arena.nodes@;
        proof {
            prefix_all(start, n1, a@);
            prefix_all(start, n1, times@);
            prefix_all(start, n1, refs@);
        }
        let length = if contains(arena, times, &a[4]) {
            cvar(arena, cb, refs, &a[4], 2)
        } else {
            a[5].cp()
        };
        let ghost n2 = arena.nodes@;
        proof {
            crate::k2_load::prefix_chain(start, n1, n2);
            prefix_all(n1, n2, a@);
            prefix(n1, n2, &gap);
        }
        let mut ts = Vec::new();
        ts.push(a[0].cp());
        ts.push(a[1].cp());
        ts.push(gap);
        ts.push(a[3].cp());
        ts.push(a[4].cp());
        ts.push(length);
        proof {
            assert_seqs_equal!(models(ts@) == seq![a[0]@, a[1]@, gap@, a[3]@, a[4]@, length@]);
        }
        let out = c(arena, &Sym::DollarGuidelineRecurrenceWindow, &ts);
        proof {
            crate::k2_load::prefix_chain(start, n2, arena.nodes@);
        }
        return out;
    }
    inner.cp()
}

pub fn compare_object(
    arena: &mut ETermArena,
    times: &Vec<T>,
    refs: &Vec<T>,
    cb: usize,
    inner: &T,
) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, times@),
        valid_all(old(arena).nodes@, refs@),
        valid(old(arena).nodes@, inner),
        spec::is_object_cmp(inner@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == if models(times@).contains(arg(inner@, 0)) {
            spec::comp(
                "$guideline_measure"@,
                seq![
                    arg(inner@, 0),
                    arg(inner@, 3),
                    spec::cvar(cb as nat, models(refs@), arg(inner@, 0), 0),
                    spec::cvar(cb as nat, models(refs@), arg(inner@, 0), 1),
                ],
            )
        } else {
            spec::comp(
                "$guideline_bound"@,
                seq![
                    arg(inner@, 0),
                    arg(inner@, 1),
                    arg(inner@, 2),
                    arg(inner@, 3),
                    spec::cvar(cb as nat, models(refs@), arg(inner@, 0), 0),
                    spec::cvar(cb as nat, models(refs@), arg(inner@, 0), 1),
                    arg(inner@, 4),
                    arg(inner@, 5),
                ],
            )
        },
{
    let ghost start = arena.nodes@;
    let a = args(arena, inner);
    let time = contains(arena, times, &a[0]);
    let vs = cvars(arena, cb, refs, &a[0]);
    let ghost n1 = arena.nodes@;
    proof {
        prefix_all(start, n1, a@);
    }
    let mut ts = Vec::new();
    ts.push(a[0].cp());
    let out = if time {
        ts.push(a[3].cp());
        ts.push(vs[0].cp());
        ts.push(vs[1].cp());
        proof {
            assert_seqs_equal!(models(ts@) == seq![a[0]@, a[3]@, vs[0]@, vs[1]@]);
        }
        c(arena, &Sym::DollarGuidelineMeasure, &ts)
    } else {
        ts.push(a[1].cp());
        ts.push(a[2].cp());
        ts.push(a[3].cp());
        ts.push(vs[0].cp());
        ts.push(vs[1].cp());
        ts.push(a[4].cp());
        ts.push(a[5].cp());
        proof {
            assert_seqs_equal!(models(ts@) == seq![a[0]@, a[1]@, a[2]@, a[3]@, vs[0]@, vs[1]@, a[4]@, a[5]@]);
        }
        c(arena, &Sym::DollarGuidelineBound, &ts)
    };
    proof {
        crate::k2_load::prefix_chain(start, n1, arena.nodes@);
        reveal_with_fuel(spec::compare_item, 1);
    }
    out
}

pub fn compare_item(
    arena: &mut ETermArena,
    scope: &Vec<I>,
    times: &Vec<T>,
    refs: &Vec<T>,
    cb: usize,
    it: &I,
) -> (out: Vec<I>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, scope@),
        valid_all(old(arena).nodes@, times@),
        valid_all(old(arena).nodes@, refs@),
        item_valid(old(arena).nodes@, it),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        items_valid(final(arena).nodes@, out@),
        item_models(out@) == spec::compare_item(
            item_models(scope@),
            models(times@),
            models(refs@),
            cb as nat,
            it@,
        ),
    decreases it, 0int,
{
    let ghost start = arena.nodes@;
    proof {
        reveal(item_valid);
        reveal_with_fuel(spec::compare_item, 1);
    }
    match &it.kind {
        Kind::Anch(ctx, inner) => {
            if is_object_cmp(arena, inner) {
                let t = compare_object(arena, times, refs, cb, inner);
                proof {
                    prefix(start, arena.nodes@, ctx);
                }
                return one_item(arena, anch(arena, ctx, &t));
            }
            if is_comp(arena, inner, &Sym::DollarGuidelineUnit, 2) {
                let a = args(arena, inner);
                if contains(arena, times, &a[0]) {
                    if let Some(o) = object_of(arena, scope, &a[0]) {
                        let oa = args(arena, &o);
                        let vs = cvars(arena, cb, refs, &a[0]);
                        let ghost n1 = arena.nodes@;
                        proof {
                            prefix_all(start, n1, a@);
                            prefix_all(start, n1, oa@);
                        }
                        let unit = c2(arena, &Sym::DollarGuidelineUnit, &a[0], &vs[2]);
                        let ghost n2 = arena.nodes@;
                        proof {
                            crate::k2_load::prefix_chain(start, n1, n2);
                            prefix_all(n1, n2, a@);
                            prefix_all(n1, n2, oa@);
                            prefix_all(n1, n2, vs@);
                        }
                        let mut ts = Vec::new();
                        ts.push(vs[0].cp());
                        ts.push(vs[1].cp());
                        ts.push(vs[2].cp());
                        ts.push(oa[4].cp());
                        ts.push(oa[5].cp());
                        ts.push(a[1].cp());
                        proof {
                            assert_seqs_equal!(models(ts@) == seq![vs[0]@, vs[1]@, vs[2]@, oa[4]@, oa[5]@, a[1]@]);
                        }
                        let scaled = c(arena, &Sym::DollarGuidelineScaled, &ts);
                        proof {
                            crate::k2_load::prefix_chain(start, n2, arena.nodes@);
                            prefix(start, arena.nodes@, ctx);
                            prefix(n2, arena.nodes@, &unit);
                        }
                        let first = anch(arena, ctx, &unit);
                        let second = anch(arena, ctx, &scaled);
                        let mut out = Vec::new();
                        out.push(first);
                        out.push(second);
                        proof {
                            reveal_with_fuel(items_valid, 3);
                            assert_seqs_equal!(item_models(out@) == seq![first@, second@]);
                        }
                        return out;
                    }
                    return one_item(arena, copy_item(arena, it));
                }
            }
            let t = relink(arena, times, refs, cb, inner);
            proof {
                prefix(start, arena.nodes@, ctx);
            }
            one_item(arena, anch(arena, ctx, &t))
        },
        Kind::Naf { dom, payload } => {
            let rs = conv_refs(arena, payload);
            let fresh = fresh_of(arena, cb, refs, &rs);
            let ghost n1 = arena.nodes@;
            proof {
                prefix_all(start, n1, dom@);
                items_prefix(start, n1, payload@);
                prefix_all(start, n1, times@);
                prefix_all(start, n1, refs@);
            }
            let local = concat(dom, &fresh);
            let converted = compare_list(arena, payload, times, refs, cb, payload);
            proof {
                crate::k2_load::prefix_chain(start, n1, arena.nodes@);
                prefix_all(n1, arena.nodes@, local@);
            }
            one_item(arena, naf(arena, local, converted))
        },
        Kind::Op { .. } => one_item(arena, copy_item(arena, it)),
    }
}

pub fn compare_list(
    arena: &mut ETermArena,
    scope: &Vec<I>,
    times: &Vec<T>,
    refs: &Vec<T>,
    cb: usize,
    items: &Vec<I>,
) -> (out: Vec<I>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, scope@),
        items_valid(old(arena).nodes@, items@),
        valid_all(old(arena).nodes@, times@),
        valid_all(old(arena).nodes@, refs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        items_valid(final(arena).nodes@, out@),
        item_models(out@) == spec::compare_list(
            item_models(scope@),
            models(times@),
            models(refs@),
            cb as nat,
            item_models(items@),
        ),
    decreases items@, 1int,
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        reveal_with_fuel(items_valid, 1);
        assert(item_models(items@).skip(0) == item_models(items@));
        assert_seqs_equal!(item_models(out@) == Seq::<spec::Item>::empty());
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            start.is_prefix_of(arena.nodes@),
            items_valid(arena.nodes@, scope@),
            items_valid(arena.nodes@, items@),
            items_valid(arena.nodes@, out@),
            valid_all(arena.nodes@, times@),
            valid_all(arena.nodes@, refs@),
            i <= items.len(),
            item_models(out@) + spec::compare_list(
                item_models(scope@),
                models(times@),
                models(refs@),
                cb as nat,
                item_models(items@).skip(i as int),
            ) == spec::compare_list(
                item_models(scope@),
                models(times@),
                models(refs@),
                cb as nat,
                item_models(items@),
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::compare_list, 1);
        }
        let ghost before = arena.nodes@;
        let next = compare_item(arena, scope, times, refs, cb, &items[i]);
        proof {
            crate::k2_load::prefix_chain(start, before, arena.nodes@);
            items_prefix(before, arena.nodes@, scope@);
            items_prefix(before, arena.nodes@, items@);
            items_prefix(before, arena.nodes@, out@);
            prefix_all(before, arena.nodes@, times@);
            prefix_all(before, arena.nodes@, refs@);
        }
        out = crate::m6_ante::concat_items(arena, &out, &next);
        i += 1;
    }
    proof {
        reveal(spec::compare_list);
    }
    out
}

pub fn compared(arena: &mut ETermArena, items: &Vec<I>, cb: usize) -> (out: Vec<I>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        items_valid(final(arena).nodes@, out@),
        item_models(out@) == spec::compared(item_models(items@), cb as nat),
{
    let times = time_refs(arena, items);
    let refs = conv_refs(arena, items);
    compare_list(arena, items, &times, &refs, cb, items)
}

} // verus!
