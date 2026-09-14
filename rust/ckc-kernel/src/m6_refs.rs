use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_model::*;
use crate::m6_symbols::Sym;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn contains(arena: &ETermArena, ts: &Vec<T>, v: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ts@),
        valid(arena.nodes@, v),
    ensures
        out == models(ts@).contains(v@),
{
    let mut i = 0usize;
    while i < ts.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ts@),
            valid(arena.nodes@, v),
            i <= ts.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] models(ts@)[j] != v@,
        decreases ts.len() - i,
    {
        if crate::m6_term::equal(arena, &ts[i], v) {
            proof {
                assert(models(ts@)[i as int] == v@);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn first_vars(arena: &ETermArena, slots: &Vec<T>, acc: &Vec<T>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, slots@),
        valid_all(arena.nodes@, acc@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::first_vars(models(slots@), models(acc@)),
{
    let mut out = copy(acc);
    let mut i = 0usize;
    proof {
        assert(models(slots@).skip(0) == models(slots@));
    }
    while i < slots.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, slots@),
            valid_all(arena.nodes@, acc@),
            valid_all(arena.nodes@, out@),
            i <= slots.len(),
            spec::first_vars(models(slots@), models(acc@)) == spec::first_vars(
                models(slots@).skip(i as int),
                models(out@),
            ),
        decreases slots.len() - i,
    {
        proof {
            assert(models(slots@).skip(i as int).drop_first() == models(slots@).skip(i as int + 1));
            reveal_with_fuel(spec::first_vars, 1);
        }
        if is_var(arena, &slots[i]) && !contains(arena, &out, &slots[i]) {
            let value = slots[i].cp();
            proof {
                extend(arena.nodes@, out@, value);
            }
            out.push(value);
        }
        i += 1;
    }
    proof {
        reveal(spec::first_vars);
    }
    out
}

pub fn nth_var(arena: &mut ETermArena, ordered: &Vec<T>, v: &T) -> (out: usize)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ordered@),
        valid(old(arena).nodes@, v),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == spec::nth_var(models(ordered@), v@),
{
    let ghost start = arena.nodes@;
    let mut out = 1usize;
    let mut i = 0usize;
    proof {
        assert(models(ordered@).skip(0) == models(ordered@));
    }
    while i < ordered.len()
        invariant
            arena_ok(arena),
            start.is_prefix_of(arena.nodes@),
            start == old(arena).nodes@,
            valid_all(arena.nodes@, ordered@),
            valid(arena.nodes@, v),
            i <= ordered.len(),
            out as nat == i as nat + 1,
            spec::nth_var(models(ordered@), v@) == i as nat + spec::nth_var(
                models(ordered@).skip(i as int),
                v@,
            ),
        decreases ordered.len() - i,
    {
        proof {
            assert(models(ordered@).skip(i as int).drop_first() == models(ordered@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::nth_var, 1);
        }
        if crate::m6_term::equal(arena, &ordered[i], v) {
            return out;
        }
        let ghost middle = arena.nodes@;
        out = successor(arena, out);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            prefix_all(middle, arena.nodes@, ordered@);
            prefix(middle, arena.nodes@, v);
        }
        i += 1;
    }
    proof {
        reveal(spec::nth_var);
    }
    out
}

pub fn lookup(arena: &ETermArena, map: &Vec<Binding>, v: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        map_valid(arena.nodes@, map@),
        valid(arena.nodes@, v),
    ensures
        out matches Some(t) ==> valid(arena.nodes@, &t),
        crate::m6_drs::term_opt(out) == spec::lookup(map_model(map@), v@),
{
    let mut i = 0usize;
    proof {
        assert(map_model(map@).skip(0) == map_model(map@));
    }
    while i < map.len()
        invariant
            arena_ok(arena),
            map_valid(arena.nodes@, map@),
            valid(arena.nodes@, v),
            i <= map.len(),
            spec::lookup(map_model(map@), v@) == spec::lookup(map_model(map@).skip(i as int), v@),
        decreases map.len() - i,
    {
        proof {
            assert(binding_valid(arena.nodes@, &map@[i as int]));
            assert(map_model(map@).skip(i as int).drop_first() == map_model(map@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::lookup, 1);
        }
        if crate::m6_term::equal(arena, &map[i].key, v) {
            return Some(map[i].value.cp());
        }
        i += 1;
    }
    proof {
        reveal(spec::lookup);
    }
    None
}

pub fn cond_refs(arena: &ETermArena, inner: &T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, inner),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::cond_refs(inner@),
{
    let mut out = Vec::new();
    match parts(arena, inner) {
        Some((name, args)) => {
            if has_name(&name, &Sym::Object) && args.len() == 6 {
                out.push(args[0].cp());
            } else if has_name(&name, &Sym::Predicate) && args.len() >= 3 {
                out.push(args[0].cp());
                let rest = tail(&args, 2);
                return concat(&out, &rest);
            } else if has_name(&name, &Sym::ModifierPp) && args.len() == 3 {
                out.push(args[0].cp());
                out.push(args[2].cp());
            } else if has_name(&name, &Sym::Property) && args.len() == 3 {
                out.push(args[0].cp());
            }
        },
        None => {},
    }
    out
}

pub fn item_refs(arena: &ETermArena, it: &I) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        item_valid(arena.nodes@, it),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::item_refs(it@),
    decreases it, 0int,
{
    proof {
        reveal(item_valid);
        reveal_with_fuel(spec::item_refs, 1);
    }
    match &it.kind {
        Kind::Anch(_, inner) => cond_refs(arena, inner),
        Kind::Op { .. } => Vec::new(),
        Kind::Naf { payload, .. } => ref_slots(arena, payload),
    }
}

pub fn ref_slots(arena: &ETermArena, items: &Vec<I>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::ref_slots(item_models(items@)),
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
            models(out@) + spec::ref_slots(item_models(items@).skip(i as int)) == spec::ref_slots(
                item_models(items@),
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::ref_slots, 1);
        }
        let next = item_refs(arena, &items[i]);
        out = concat(&out, &next);
        i += 1;
    }
    proof {
        reveal(spec::ref_slots);
    }
    out
}

} // verus!
