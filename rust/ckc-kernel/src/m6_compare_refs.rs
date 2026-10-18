use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_model::*;
use crate::m6_symbols::Sym;
use crate::m6_term::*;
use crate::m7_annotate::arg_at;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn convertible(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == spec::convertible(t@),
{
    is_atom(arena, t, &Sym::Exactly) || is_atom(arena, t, &Sym::Geq) || is_atom(
        arena,
        t,
        &Sym::Greater,
    ) || is_atom(arena, t, &Sym::Leq) || is_atom(arena, t, &Sym::Less)
}

pub fn is_object_cmp(arena: &ETermArena, inner: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, inner),
    ensures
        out == spec::is_object_cmp(inner@),
{
    if is_comp(arena, inner, &Sym::Object, 6) {
        let op = arg_at(arena, inner, 4);
        convertible(arena, &op)
    } else {
        false
    }
}

pub fn conv_item(arena: &ETermArena, it: &I) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        item_valid(arena.nodes@, it),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::conv_item(it@),
    decreases it, 0int,
{
    proof {
        reveal(item_valid);
        reveal_with_fuel(spec::conv_item, 1);
    }
    match &it.kind {
        Kind::Anch(_, inner) => {
            let mut out = Vec::new();
            if is_object_cmp(arena, inner) {
                out.push(arg_at(arena, inner, 0));
            }
            out
        },
        Kind::Naf { payload, .. } => conv_refs(arena, payload),
        Kind::Op { .. } => Vec::new(),
    }
}

pub fn conv_refs(arena: &ETermArena, items: &Vec<I>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::conv_refs(item_models(items@)),
    decreases items@, 1int,
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
        assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid_all(arena.nodes@, out@),
            i <= items.len(),
            models(out@) + spec::conv_refs(item_models(items@).skip(i as int)) == spec::conv_refs(
                item_models(items@),
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::conv_refs, 1);
        }
        let next = conv_item(arena, &items[i]);
        out = concat(&out, &next);
        i += 1;
    }
    proof {
        reveal(spec::conv_refs);
    }
    out
}

pub fn ref_pos(arena: &ETermArena, refs: &Vec<T>, r: &T) -> (out: usize)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, refs@),
        valid(arena.nodes@, r),
    ensures
        out as nat == spec::ref_pos(models(refs@), r@),
        out <= refs.len(),
{
    let mut i = 0usize;
    proof {
        assert(models(refs@).skip(0) == models(refs@));
    }
    while i < refs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, refs@),
            valid(arena.nodes@, r),
            i <= refs.len(),
            i as nat + spec::ref_pos(models(refs@).skip(i as int), r@) == spec::ref_pos(
                models(refs@),
                r@,
            ),
        decreases refs.len() - i,
    {
        proof {
            assert(models(refs@).skip(i as int).drop_first() == models(refs@).skip(i as int + 1));
            reveal_with_fuel(spec::ref_pos, 1);
        }
        if crate::m6_term::equal(arena, &refs[i], r) {
            return i;
        }
        i += 1;
    }
    proof {
        reveal(spec::ref_pos);
    }
    i
}

pub fn unit_of(arena: &ETermArena, scope: &Vec<I>, r: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        valid(arena.nodes@, r),
    ensures
        out matches Some(t) ==> valid(arena.nodes@, &t),
        crate::m6_drs::term_opt(out) == spec::unit_of(item_models(scope@), r@),
{
    let mut i = 0usize;
    proof {
        assert(item_models(scope@).skip(0) == item_models(scope@));
    }
    while i < scope.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, scope@),
            valid(arena.nodes@, r),
            i <= scope.len(),
            spec::unit_of(item_models(scope@), r@) == spec::unit_of(
                item_models(scope@).skip(i as int),
                r@,
            ),
        decreases scope.len() - i,
    {
        proof {
            items_at(arena.nodes@, scope@, i as int);
            reveal(item_valid);
            assert(item_models(scope@).skip(i as int).drop_first() == item_models(scope@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::unit_of, 1);
        }
        if let Kind::Anch(_, inner) = &scope[i].kind {
            if is_comp(arena, inner, &Sym::DollarGuidelineUnit, 2) {
                let a0 = arg_at(arena, inner, 0);
                if crate::m6_term::equal(arena, &a0, r) {
                    return Some(arg_at(arena, inner, 1));
                }
            }
        }
        i += 1;
    }
    proof {
        reveal(spec::unit_of);
    }
    None
}

pub fn object_of(arena: &ETermArena, scope: &Vec<I>, r: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        valid(arena.nodes@, r),
    ensures
        out matches Some(t) ==> valid(arena.nodes@, &t) && spec::is_comp(t@, "object"@, 6),
        crate::m6_drs::term_opt(out) == spec::object_of(item_models(scope@), r@),
{
    let mut i = 0usize;
    proof {
        assert(item_models(scope@).skip(0) == item_models(scope@));
    }
    while i < scope.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, scope@),
            valid(arena.nodes@, r),
            i <= scope.len(),
            spec::object_of(item_models(scope@), r@) == spec::object_of(
                item_models(scope@).skip(i as int),
                r@,
            ),
        decreases scope.len() - i,
    {
        proof {
            items_at(arena.nodes@, scope@, i as int);
            reveal(item_valid);
            assert(item_models(scope@).skip(i as int).drop_first() == item_models(scope@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::object_of, 1);
        }
        if let Kind::Anch(_, inner) = &scope[i].kind {
            if is_comp(arena, inner, &Sym::Object, 6) {
                let a0 = arg_at(arena, inner, 0);
                if crate::m6_term::equal(arena, &a0, r) {
                    return Some(inner.cp());
                }
            }
        }
        i += 1;
    }
    proof {
        reveal(spec::object_of);
    }
    None
}

pub fn scope_times(arena: &ETermArena, scope: &Vec<I>, items: &Vec<I>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        items_valid(arena.nodes@, items@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::scope_times(item_models(scope@), item_models(items@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
        assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, scope@),
            items_valid(arena.nodes@, items@),
            valid_all(arena.nodes@, out@),
            i <= items.len(),
            models(out@) + spec::scope_times(
                item_models(scope@),
                item_models(items@).skip(i as int),
            ) == spec::scope_times(item_models(scope@), item_models(items@)),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::scope_times, 1);
        }
        if let Kind::Anch(_, inner) = &items[i].kind {
            if is_object_cmp(arena, inner) {
                let r = arg_at(arena, inner, 0);
                if unit_of(arena, scope, &r).is_some() {
                    proof {
                        extend(arena.nodes@, out@, r);
                    }
                    out.push(r);
                }
            }
        }
        i += 1;
    }
    proof {
        reveal(spec::scope_times);
    }
    out
}

pub fn time_item(arena: &ETermArena, it: &I) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        item_valid(arena.nodes@, it),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::time_item(it@),
    decreases it, 0int,
{
    proof {
        reveal(item_valid);
        reveal_with_fuel(spec::time_item, 1);
    }
    match &it.kind {
        Kind::Naf { payload, .. } => time_refs(arena, payload),
        _ => Vec::new(),
    }
}

pub fn nested_times(arena: &ETermArena, items: &Vec<I>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::nested_times(item_models(items@)),
    decreases items@, 1int,
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
        assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid_all(arena.nodes@, out@),
            i <= items.len(),
            models(out@) + spec::nested_times(item_models(items@).skip(i as int))
                == spec::nested_times(item_models(items@)),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::nested_times, 1);
        }
        let next = time_item(arena, &items[i]);
        out = concat(&out, &next);
        i += 1;
    }
    proof {
        reveal(spec::nested_times);
    }
    out
}

pub fn time_refs(arena: &ETermArena, items: &Vec<I>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::time_refs(item_models(items@)),
    decreases items@, 2int,
{
    let scoped = scope_times(arena, items, items);
    let nested = nested_times(arena, items);
    concat(&scoped, &nested)
}

pub fn cvar(arena: &mut ETermArena, cb: usize, refs: &Vec<T>, r: &T, i: usize) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, refs@),
        valid(old(arena).nodes@, r),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::cvar(cb as nat, models(refs@), r@, i as nat),
{
    let ghost start = arena.nodes@;
    let p = ref_pos(arena, refs, r);
    let x = crate::m6_flat::add(arena, cb, p);
    let ghost n1 = arena.nodes@;
    let y = crate::m6_flat::add(arena, x, p);
    let ghost n2 = arena.nodes@;
    let z = crate::m6_flat::add(arena, y, p);
    let ghost n3 = arena.nodes@;
    let key = crate::m6_flat::add(arena, z, i);
    let ghost n4 = arena.nodes@;
    let out = var(arena, key);
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
        crate::k2_load::prefix_chain(start, n2, n3);
        crate::k2_load::prefix_chain(start, n3, n4);
        crate::k2_load::prefix_chain(start, n4, arena.nodes@);
    }
    out
}

pub fn fresh_of(arena: &mut ETermArena, cb: usize, refs: &Vec<T>, rs: &Vec<T>) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, refs@),
        valid_all(old(arena).nodes@, rs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        models(out@) == spec::fresh_of(cb as nat, models(refs@), models(rs@)),
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(models(rs@).skip(0) == models(rs@));
        assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
    }
    while i < rs.len()
        invariant
            arena_ok(arena),
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, refs@),
            valid_all(arena.nodes@, rs@),
            valid_all(arena.nodes@, out@),
            i <= rs.len(),
            models(out@) + spec::fresh_of(cb as nat, models(refs@), models(rs@).skip(i as int))
                == spec::fresh_of(cb as nat, models(refs@), models(rs@)),
        decreases rs.len() - i,
    {
        proof {
            assert(models(rs@).skip(i as int).drop_first() == models(rs@).skip(i as int + 1));
            reveal_with_fuel(spec::fresh_of, 1);
        }
        let ghost before = arena.nodes@;
        let a = cvar(arena, cb, refs, &rs[i], 0);
        let ghost n1 = arena.nodes@;
        proof {
            prefix_all(before, n1, refs@);
            prefix_all(before, n1, rs@);
        }
        let b = cvar(arena, cb, refs, &rs[i], 1);
        let ghost n2 = arena.nodes@;
        proof {
            prefix_all(n1, n2, refs@);
            prefix_all(n1, n2, rs@);
        }
        let c = cvar(arena, cb, refs, &rs[i], 2);
        proof {
            crate::k2_load::prefix_chain(n1, n2, arena.nodes@);
            crate::k2_load::prefix_chain(before, n1, arena.nodes@);
            crate::k2_load::prefix_chain(start, before, arena.nodes@);
            prefix(n1, arena.nodes@, &a);
            prefix(n2, arena.nodes@, &b);
            prefix_all(before, arena.nodes@, out@);
            prefix_all(n2, arena.nodes@, refs@);
            prefix_all(n2, arena.nodes@, rs@);
        }
        let mut next = Vec::new();
        next.push(a);
        next.push(b);
        next.push(c);
        proof {
            assert_seqs_equal!(models(next@) == seq![a@, b@, c@]);
        }
        out = concat(&out, &next);
        i += 1;
    }
    proof {
        reveal(spec::fresh_of);
    }
    out
}

} // verus!
