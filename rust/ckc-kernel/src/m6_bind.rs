use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_flat::{Env, gid, slot};
use crate::m6_model::*;
use crate::m6_refs::{cond_refs, contains, lookup, nth_var};
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn identity(
    arena: &mut ETermArena,
    env: &Env,
    role: &Sym,
    slot_kind: &Sym,
    n: usize,
    deps: &T,
) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, deps),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(
            ckc_spec::replay::gid_name(),
            seq![
                Term::Atom(symbol(role)),
                Term::Atom(env.docid@),
                Term::Int(env.s as int),
                Term::Comp(symbol(slot_kind), seq![Term::Int(n as int)]),
                deps@,
            ],
        ),
{
    let ghost start = arena.nodes@;
    let sl = slot(arena, slot_kind, n);
    let ghost middle = arena.nodes@;
    proof {
        prefix(start, middle, deps);
    }
    let out = gid(arena, role, env, &sl, deps);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
    }
    out
}

pub fn push_binding(arena: &ETermArena, mut map: Vec<Binding>, key: &T, value: T) -> (out: Vec<
    Binding,
>)
    requires
        arena_ok(arena),
        map_valid(arena.nodes@, map@),
        valid(arena.nodes@, key),
        valid(arena.nodes@, &value),
    ensures
        map_valid(arena.nodes@, out@),
        map_model(out@) == map_model(map@).push((key@, value@)),
{
    let ghost before = map@;
    let next = Binding { key: key.cp(), value };
    map.push(next);
    proof {
        assert forall|i: int| 0 <= i < map.len() implies #[trigger] binding_valid(
            arena.nodes@,
            &map@[i],
        ) by {
            if i < before.len() {
                assert(map@[i] == before[i]);
            }
        }
        assert_seqs_equal!(map_model(map@) == map_model(before).push((key@, value@)));
    }
    map
}

pub fn mint(
    arena: &mut ETermArena,
    ordered: &Vec<T>,
    n: usize,
    env: &Env,
    map: &Vec<Binding>,
) -> (out: Vec<Binding>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ordered@),
        map_valid(old(arena).nodes@, map@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        map_valid(final(arena).nodes@, out@),
        map_model(out@) == spec::mint(
            models(ordered@),
            n as nat,
            env.s as nat,
            env.docid@,
            map_model(map@),
        ),
{
    let ghost start = arena.nodes@;
    let deps = nil(arena);
    proof {
        prefix_all(start, arena.nodes@, ordered@);
        map_prefix(start, arena.nodes@, map@);
    }
    let mut out = copy_map(arena, map);
    let mut i = 0usize;
    let mut next = n;
    proof {
        assert(models(ordered@).skip(0) == models(ordered@));
    }
    while i < ordered.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, ordered@),
            map_valid(arena.nodes@, map@),
            map_valid(arena.nodes@, out@),
            valid(arena.nodes@, &deps),
            deps@ == Term::Nil,
            i <= ordered.len(),
            next as nat == n as nat + i as nat,
            spec::mint(models(ordered@), n as nat, env.s as nat, env.docid@, map_model(map@))
                == spec::mint(
                models(ordered@).skip(i as int),
                next as nat,
                env.s as nat,
                env.docid@,
                map_model(out@),
            ),
        decreases ordered.len() - i,
    {
        proof {
            assert(models(ordered@).skip(i as int).drop_first() == models(ordered@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::mint, 1);
        }
        let ghost before = arena.nodes@;
        if lookup(arena, &out, &ordered[i]).is_none() {
            let value = identity(arena, env, &Sym::Product, &Sym::Ref, next, &deps);
            proof {
                map_prefix(before, arena.nodes@, out@);
                prefix_all(before, arena.nodes@, ordered@);
            }
            out = push_binding(arena, out, &ordered[i], value);
        }
        let ghost middle = arena.nodes@;
        proof {
            crate::k2_load::prefix_chain(start, before, middle);
            prefix_all(before, middle, ordered@);
            map_prefix(before, middle, map@);
            prefix(before, middle, &deps);
        }
        next = successor(arena, next);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            prefix_all(middle, arena.nodes@, ordered@);
            map_prefix(middle, arena.nodes@, out@);
            map_prefix(middle, arena.nodes@, map@);
            prefix(middle, arena.nodes@, &deps);
        }
        i += 1;
    }
    proof {
        reveal(spec::mint);
    }
    out
}

pub proof fn locals_step(ordered: Seq<Term>, v: Term, ante: Seq<Term>, map: Seq<(Term, Term)>)
    ensures
        spec::cons_locals(ordered.push(v), ante, map) == if !ante.contains(v) && spec::lookup(
            map,
            v,
        ) is None {
            spec::cons_locals(ordered, ante, map).push(v)
        } else {
            spec::cons_locals(ordered, ante, map)
        },
{
    reveal(Seq::filter);
    reveal(spec::cons_locals);
    assert(ordered.push(v).drop_last() == ordered);
    assert(ordered.push(v).last() == v);
}

pub fn cons_locals(arena: &ETermArena, ordered: &Vec<T>, ante: &Vec<T>, map: &Vec<Binding>) -> (out:
    Vec<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ordered@),
        valid_all(arena.nodes@, ante@),
        map_valid(arena.nodes@, map@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::cons_locals(models(ordered@), models(ante@), map_model(map@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        reveal(Seq::filter);
        reveal(spec::cons_locals);
    }
    while i < ordered.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ordered@),
            valid_all(arena.nodes@, ante@),
            map_valid(arena.nodes@, map@),
            valid_all(arena.nodes@, out@),
            i <= ordered.len(),
            models(out@) == spec::cons_locals(
                models(ordered@).take(i as int),
                models(ante@),
                map_model(map@),
            ),
        decreases ordered.len() - i,
    {
        proof {
            assert(models(ordered@).take(i as int + 1) == models(ordered@).take(i as int).push(
                models(ordered@)[i as int],
            ));
            locals_step(
                models(ordered@).take(i as int),
                models(ordered@)[i as int],
                models(ante@),
                map_model(map@),
            );
        }
        if !contains(arena, ante, &ordered[i]) && lookup(arena, map, &ordered[i]).is_none() {
            let next = ordered[i].cp();
            proof {
                extend(arena.nodes@, out@, next);
            }
            out.push(next);
        }
        i += 1;
    }
    proof {
        assert(models(ordered@).take(i as int) == models(ordered@));
    }
    out
}

pub fn skolem(
    arena: &mut ETermArena,
    locals: &Vec<T>,
    ordered: &Vec<T>,
    deps: &T,
    env: &Env,
) -> (out: Vec<Binding>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, locals@),
        valid_all(old(arena).nodes@, ordered@),
        valid(old(arena).nodes@, deps),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        map_valid(final(arena).nodes@, out@),
        map_model(out@) == spec::skolem(
            models(locals@),
            models(ordered@),
            deps@,
            env.s as nat,
            env.docid@,
        ),
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < locals.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, locals@),
            valid_all(arena.nodes@, ordered@),
            valid(arena.nodes@, deps),
            map_valid(arena.nodes@, out@),
            i <= locals.len(),
            out.len() == i,
            map_model(out@) == spec::skolem(
                models(locals@),
                models(ordered@),
                deps@,
                env.s as nat,
                env.docid@,
            ).take(i as int),
        decreases locals.len() - i,
    {
        let ghost before = arena.nodes@;
        let slot = nth_var(arena, ordered, &locals[i]);
        let ghost middle = arena.nodes@;
        proof {
            prefix(before, middle, deps);
        }
        let value = identity(arena, env, &Sym::Product, &Sym::Ref, slot, deps);
        proof {
            crate::k2_load::prefix_chain(before, middle, arena.nodes@);
            crate::k2_load::prefix_chain(start, before, arena.nodes@);
            prefix_all(before, arena.nodes@, locals@);
            prefix_all(before, arena.nodes@, ordered@);
            prefix(before, arena.nodes@, deps);
            map_prefix(before, arena.nodes@, out@);
        }
        let ghost previous = map_model(out@);
        out = push_binding(arena, out, &locals[i], value);
        proof {
            assert_seqs_equal!(map_model(out@) == spec::skolem(models(locals@), models(ordered@), deps@, env.s as nat, env.docid@).take(i as int + 1));
        }
        i += 1;
    }
    proof {
        assert(spec::skolem(
            models(locals@),
            models(ordered@),
            deps@,
            env.s as nat,
            env.docid@,
        ).take(i as int) == spec::skolem(
            models(locals@),
            models(ordered@),
            deps@,
            env.s as nat,
            env.docid@,
        ));
    }
    out
}

pub fn witness_refs(
    arena: &mut ETermArena,
    refs: &Vec<T>,
    ordered: &Vec<T>,
    map: &Vec<Binding>,
    env: &Env,
    k: usize,
    acc: &Vec<Binding>,
) -> (out: Vec<Binding>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, refs@),
        valid_all(old(arena).nodes@, ordered@),
        map_valid(old(arena).nodes@, map@),
        map_valid(old(arena).nodes@, acc@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        map_valid(final(arena).nodes@, out@),
        map_model(out@) == spec::witness_refs(
            models(refs@),
            models(ordered@),
            map_model(map@),
            env.docid@,
            env.s as nat,
            k as nat,
            map_model(acc@),
        ),
{
    let ghost start = arena.nodes@;
    let variant = slot(arena, &Sym::Variant, k);
    proof {
        prefix_all(start, arena.nodes@, refs@);
        prefix_all(start, arena.nodes@, ordered@);
        map_prefix(start, arena.nodes@, map@);
        map_prefix(start, arena.nodes@, acc@);
    }
    let mut out = copy_map(arena, acc);
    let mut i = 0usize;
    proof {
        assert(models(refs@).skip(0) == models(refs@));
    }
    while i < refs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, refs@),
            valid_all(arena.nodes@, ordered@),
            map_valid(arena.nodes@, map@),
            map_valid(arena.nodes@, acc@),
            map_valid(arena.nodes@, out@),
            valid(arena.nodes@, &variant),
            variant@ == spec::variant_slot(k as nat),
            i <= refs.len(),
            spec::witness_refs(
                models(refs@),
                models(ordered@),
                map_model(map@),
                env.docid@,
                env.s as nat,
                k as nat,
                map_model(acc@),
            ) == spec::witness_refs(
                models(refs@).skip(i as int),
                models(ordered@),
                map_model(map@),
                env.docid@,
                env.s as nat,
                k as nat,
                map_model(out@),
            ),
        decreases refs.len() - i,
    {
        proof {
            assert(models(refs@).skip(i as int).drop_first() == models(refs@).skip(i as int + 1));
            reveal_with_fuel(spec::witness_refs, 1);
        }
        let ghost before = arena.nodes@;
        if is_var(arena, &refs[i]) && lookup(arena, map, &refs[i]).is_none() && lookup(
            arena,
            &out,
            &refs[i],
        ).is_none() {
            let n = nth_var(arena, ordered, &refs[i]);
            let ghost middle = arena.nodes@;
            proof {
                prefix(before, middle, &variant);
            }
            let value = identity(arena, env, &Sym::Witness, &Sym::Ref, n, &variant);
            proof {
                crate::k2_load::prefix_chain(before, middle, arena.nodes@);
                prefix_all(before, arena.nodes@, refs@);
                map_prefix(before, arena.nodes@, out@);
            }
            out = push_binding(arena, out, &refs[i], value);
        }
        proof {
            crate::k2_load::prefix_chain(start, before, arena.nodes@);
            prefix_all(before, arena.nodes@, refs@);
            prefix_all(before, arena.nodes@, ordered@);
            map_prefix(before, arena.nodes@, map@);
            map_prefix(before, arena.nodes@, acc@);
            prefix(before, arena.nodes@, &variant);
        }
        i += 1;
    }
    proof {
        reveal(spec::witness_refs);
    }
    out
}

pub fn witness_pairs(
    arena: &mut ETermArena,
    items: &Vec<I>,
    ordered: &Vec<T>,
    map: &Vec<Binding>,
    env: &Env,
    k: usize,
    acc: &Vec<Binding>,
) -> (out: Vec<Binding>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
        valid_all(old(arena).nodes@, ordered@),
        map_valid(old(arena).nodes@, map@),
        map_valid(old(arena).nodes@, acc@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        map_valid(final(arena).nodes@, out@),
        map_model(out@) == spec::witness_pairs(
            item_models(items@),
            models(ordered@),
            map_model(map@),
            env.docid@,
            env.s as nat,
            k as nat,
            map_model(acc@),
        ),
{
    let ghost start = arena.nodes@;
    let variant = slot(arena, &Sym::Variant, k);
    proof {
        items_prefix(start, arena.nodes@, items@);
        prefix_all(start, arena.nodes@, ordered@);
        map_prefix(start, arena.nodes@, map@);
        map_prefix(start, arena.nodes@, acc@);
    }
    let mut out = copy_map(arena, acc);
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            items_valid(arena.nodes@, items@),
            valid_all(arena.nodes@, ordered@),
            map_valid(arena.nodes@, map@),
            map_valid(arena.nodes@, acc@),
            map_valid(arena.nodes@, out@),
            valid(arena.nodes@, &variant),
            variant@ == spec::variant_slot(k as nat),
            i <= items.len(),
            spec::witness_pairs(
                item_models(items@),
                models(ordered@),
                map_model(map@),
                env.docid@,
                env.s as nat,
                k as nat,
                map_model(acc@),
            ) == spec::witness_pairs(
                item_models(items@).skip(i as int),
                models(ordered@),
                map_model(map@),
                env.docid@,
                env.s as nat,
                k as nat,
                map_model(out@),
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::witness_pairs, 1);
        }
        let ghost before = arena.nodes@;
        match &items[i].kind {
            Kind::Op { b, inner, .. } => {
                if is_var(arena, inner) && lookup(arena, &out, inner).is_none() {
                    let value = identity(arena, env, &Sym::Witness, &Sym::Box, *b, &variant);
                    proof {
                        map_prefix(before, arena.nodes@, out@);
                        prefix(before, arena.nodes@, inner);
                    }
                    out = push_binding(arena, out, inner, value);
                }
            },
            Kind::Naf { .. } => {},
            Kind::Anch(_, inner) => {
                let refs = cond_refs(arena, inner);
                out = witness_refs(arena, &refs, ordered, map, env, k, &out);
            },
        }
        proof {
            crate::k2_load::prefix_chain(start, before, arena.nodes@);
            items_prefix(before, arena.nodes@, items@);
            prefix_all(before, arena.nodes@, ordered@);
            map_prefix(before, arena.nodes@, map@);
            map_prefix(before, arena.nodes@, acc@);
            prefix(before, arena.nodes@, &variant);
        }
        i += 1;
    }
    proof {
        reveal(spec::witness_pairs);
    }
    out
}

pub fn bind_all(arena: &mut ETermArena, t: &T, pairs: &Vec<Binding>) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, t),
        map_valid(old(arena).nodes@, pairs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::bind_all(t@, map_model(pairs@)),
{
    let ghost start = arena.nodes@;
    let mut out = t.cp();
    let mut i = 0usize;
    proof {
        assert(map_model(pairs@).skip(0) == map_model(pairs@));
    }
    while i < pairs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid(arena.nodes@, &out),
            map_valid(arena.nodes@, pairs@),
            i <= pairs.len(),
            spec::bind_all(t@, map_model(pairs@)) == spec::bind_all(
                out@,
                map_model(pairs@).skip(i as int),
            ),
        decreases pairs.len() - i,
    {
        proof {
            assert(binding_valid(arena.nodes@, &pairs@[i as int]));
            assert(map_model(pairs@).skip(i as int).drop_first() == map_model(pairs@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::bind_all, 1);
        }
        let ghost middle = arena.nodes@;
        if is_var(arena, &pairs[i].key) {
            let x = var_index(arena, &pairs[i].key);
            out = substitute(arena, &out, x, &pairs[i].value);
        }
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            map_prefix(middle, arena.nodes@, pairs@);
        }
        i += 1;
    }
    proof {
        reveal(spec::bind_all);
    }
    out
}

} // verus!
