#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_drs::{anchor, box_parts, op};
use crate::m6_model::*;
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub struct Env {
    pub docid: Vec<u8>,
    pub s: usize,
    pub base: usize,
}

pub open spec fn flat_valid_result(nodes: Seq<ENode>, r: &Result<Flat, T>) -> bool {
    match r { Ok(f) => flat_valid(nodes, f), Err(t) => valid(nodes, t) }
}

pub fn slot(arena: &mut ETermArena, sym: &Sym, n: usize) -> (out: T)
    requires arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(symbol(sym), seq![Term::Int(n as int)]),
{
    let ghost start = arena.nodes@;
    let v = crate::m6_term::int(arena, n);
    let ghost middle = arena.nodes@;
    let out = c1(arena, sym, &v);
    proof { crate::k2_load::prefix_chain(start, middle, arena.nodes@); }
    out
}

pub fn gid(arena: &mut ETermArena, role: &Sym, env: &Env, slot: &T, deps: &T) -> (out: T)
    requires
        arena_ok(old(arena)), valid(old(arena).nodes@, slot), valid(old(arena).nodes@, deps),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(ckc_spec::replay::gid_name(), seq![
            Term::Atom(symbol(role)), Term::Atom(env.docid@), Term::Int(env.s as int), slot@, deps@,
        ]),
{
    let ghost n0 = arena.nodes@;
    proof {
        reveal_strlit("$guideline_id"); reveal(ckc_spec::v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::DollarGuidelineId) == ckc_spec::replay::gid_name());
    }
    let role = named(arena, role);
    let ghost n1 = arena.nodes@;
    let doc = atom(arena, &env.docid);
    let ghost n2 = arena.nodes@;
    let sentence = crate::m6_term::int(arena, env.s);
    proof {
        crate::k2_load::prefix_chain(n0, n1, n2);
        crate::k2_load::prefix_chain(n0, n2, arena.nodes@);
        crate::k2_load::prefix_chain(n1, n2, arena.nodes@);
        prefix(n0, arena.nodes@, slot); prefix(n0, arena.nodes@, deps);
        prefix(n1, arena.nodes@, &role); prefix(n2, arena.nodes@, &doc);
    }
    let mut args = Vec::new();
    args.push(role); args.push(doc); args.push(sentence); args.push(slot.cp()); args.push(deps.cp());
    let ghost middle = arena.nodes@;
    let out = c(arena, &Sym::DollarGuidelineId, &args);
    proof { crate::k2_load::prefix_chain(n0, middle, arena.nodes@); }
    out
}

pub fn add(arena: &mut ETermArena, a: usize, b: usize) -> (out: usize)
    requires arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == a as nat + b as nat,
{
    let ghost start = arena.nodes@;
    let mut out = a;
    let mut i = 0usize;
    while i < b
        invariant
            arena_ok(arena), start.is_prefix_of(arena.nodes@),
            i <= b, out as nat == a as nat + i as nat,
        decreases b - i,
    {
        let ghost middle = arena.nodes@;
        out = successor(arena, out);
        proof { crate::k2_load::prefix_chain(start, middle, arena.nodes@); }
        i += 1;
    }
    out
}

pub fn context(arena: &mut ETermArena, w: &W, env: &Env, deps: &T, b: usize) -> (out: T)
    requires arena_ok(old(arena)), valid(old(arena).nodes@, deps),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::op_context(w@, env.docid@, env.s as nat, deps@, b as nat, env.base as nat),
{
    let ghost start = arena.nodes@;
    if let W::Antecedent = w {
        let key = add(arena, env.base, b);
        let ghost middle = arena.nodes@;
        let out = var(arena, key);
        proof { crate::k2_load::prefix_chain(start, middle, arena.nodes@); }
        out
    } else {
        let sl = slot(arena, &Sym::Box, b);
        let ghost middle = arena.nodes@;
        proof { prefix(start, middle, deps); }
        let out = gid(arena, &Sym::Context, env, &sl, deps);
        proof { crate::k2_load::prefix_chain(start, middle, arena.nodes@); }
        out
    }
}

pub fn empty(arena: &ETermArena, n: usize) -> (out: Flat)
    requires arena_ok(arena),
    ensures flat_valid(arena.nodes@, &out), out@ == (spec::Flat { items: Seq::empty(), n: n as nat }),
{
    proof { reveal_with_fuel(items_valid, 1); }
    let items = Vec::new();
    proof { assert_seqs_equal!(item_models(items@) == Seq::<spec::Item>::empty()); }
    Flat { items, n }
}

pub fn one(arena: &ETermArena, it: I, n: usize) -> (out: Flat)
    requires arena_ok(arena), item_valid(arena.nodes@, &it),
    ensures flat_valid(arena.nodes@, &out), out@ == (spec::Flat { items: seq![it@], n: n as nat }),
{
    let mut items = Vec::new(); items.push(it);
    proof { reveal_with_fuel(items_valid, 2); assert(item_models(items@) == seq![it@]); }
    Flat { items, n }
}

pub fn join(arena: &ETermArena, mut first: Flat, mut last: Flat) -> (out: Flat)
    requires arena_ok(arena), flat_valid(arena.nodes@, &first), flat_valid(arena.nodes@, &last),
    ensures flat_valid(arena.nodes@, &out), out@ == (spec::Flat { items: first@.items + last@.items, n: last@.n }),
{
    proof { items_concat(arena.nodes@, first.items@, last.items@); }
    first.items.append(&mut last.items);
    Flat { items: first.items, n: last.n }
}

pub fn flatten_cond(
    arena: &mut ETermArena, c: &T, w: &W, env: &Env, deps: &T, outer: &T, encl: &E, n: usize,
) -> (out: Result<Flat, T>)
    requires
        arena_ok(old(arena)), valid(old(arena).nodes@, c),
        valid(old(arena).nodes@, deps), valid(old(arena).nodes@, outer),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        flat_valid_result(final(arena).nodes@, &out),
        flat_result(out) == spec::flatten_cond(c@, w@, env.s as nat, env.docid@,
            deps@, outer@, encl@, n as nat, env.base as nat),
    decreases c@, 1int,
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::flatten_cond, 1);
        reveal_strlit("[|]"); reveal(ckc_spec::v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::Cons) == ckc_spec::v1text::cons_name());
    }
    if anchor(arena, c).is_some() {
        let a = args(arena, c);
        let it = anch(arena, outer, &a[0]);
        return Ok(one(arena, it, n));
    }
    if is_nil(arena, c) { return Ok(empty(arena, n)); }
    match parts(arena, c) {
        None => Err(named(arena, &Sym::ConditionShape)),
        Some((name, a)) => {
            proof { assert(c@ == Term::Comp(name@, models(a@))); }
            if has_name(&name, &Sym::Cons) && a.len() == 2 {
                flatten_list(arena, c, w, env, deps, outer, encl, n)
            } else if has_name(&name, &Sym::Implies) && a.len() == 2 {
                if let E::Op = encl { Err(named(arena, &Sym::OperatorScopedRule)) }
                else { Err(named(arena, &Sym::ConditionShape)) }
            } else if has_name(&name, &Sym::V) && a.len() == 2 {
                Err(named(arena, &Sym::Disjunction))
            } else if has_name(&name, &Sym::Naf) && a.len() == 1 {
                if let (W::Antecedent, E::Top) = (w, encl) {
                    match box_parts(arena, &a[0]) {
                        None => Err(named(arena, &Sym::InvalidDrsShape)),
                        Some(b) => {
                            let r = flatten_list(arena, &b.conds, w, env, deps, outer, &E::Naf, n);
                            match r {
                                Err(e) => Err(e),
                                Ok(f) => {
                                    if has_anch(arena, &f.items) {
                                        proof { prefix_all(start, arena.nodes@, b.dom@); }
                                        let it = naf(arena, b.dom, f.items);
                                        Ok(one(arena, it, f.n))
                                    } else {
                                        let ghost middle = arena.nodes@;
                                        let err = named(arena, &Sym::NafShape);
                                        proof { crate::k2_load::prefix_chain(start, middle, arena.nodes@); }
                                        Err(err)
                                    }
                                },
                            }
                        },
                    }
                } else { Err(named(arena, &Sym::NafPlacement)) }
            } else if op(&name) && a.len() == 1 {
                if let E::Naf = encl { return Err(named(arena, &Sym::DeferredOperator)); }
                match box_parts(arena, &a[0]) {
                    None => Err(named(arena, &Sym::InvalidDrsShape)),
                    Some(b) => {
                        let inner = context(arena, w, env, deps, n);
                        let ghost n1 = arena.nodes@;
                        let next = successor(arena, n);
                        let ghost n2 = arena.nodes@;
                        proof {
                            crate::k2_load::prefix_chain(start, n1, n2);
                            prefix(start, n2, deps); prefix(start, n2, &b.conds); prefix(n1, n2, &inner);
                        }
                        let r = flatten_list(arena, &b.conds, w, env, deps, &inner, &E::Op, next);
                        proof {
                            crate::k2_load::prefix_chain(start, n2, arena.nodes@);
                            crate::k2_load::prefix_chain(n1, n2, arena.nodes@);
                        }
                        match r {
                            Err(e) => Err(e),
                            Ok(f) => {
                                if has_anch(arena, &f.items) {
                                    proof { prefix(start, arena.nodes@, outer); prefix(n1, arena.nodes@, &inner); }
                                    let edge = operator(arena, n, outer, &inner, &name);
                                    let first = one(arena, edge, n);
                                    Ok(join(arena, first, f))
                                } else {
                                    let ghost middle = arena.nodes@;
                                    let err = named(arena, &Sym::ConditionShape);
                                    proof { crate::k2_load::prefix_chain(start, middle, arena.nodes@); }
                                    Err(err)
                                }
                            },
                        }
                    },
                }
            } else { Err(named(arena, &Sym::ConditionShape)) }
        },
    }
}

pub fn flatten_list(
    arena: &mut ETermArena, l: &T, w: &W, env: &Env, deps: &T, outer: &T, encl: &E, n: usize,
) -> (out: Result<Flat, T>)
    requires
        arena_ok(old(arena)), valid(old(arena).nodes@, l),
        valid(old(arena).nodes@, deps), valid(old(arena).nodes@, outer),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        flat_valid_result(final(arena).nodes@, &out),
        flat_result(out) == spec::flatten_list(l@, w@, env.s as nat, env.docid@,
            deps@, outer@, encl@, n as nat, env.base as nat),
    decreases l@, 0int,
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::flatten_list, 1);
        reveal_strlit("[|]"); reveal(ckc_spec::v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::Cons) == ckc_spec::v1text::cons_name());
    }
    if is_nil(arena, l) { return Ok(empty(arena, n)); }
    match parts(arena, l) {
        Some((name, a)) => {
            proof { assert(l@ == Term::Comp(name@, models(a@))); }
            if has_name(&name, &Sym::Cons) && a.len() == 2 {
                match flatten_cond(arena, &a[0], w, env, deps, outer, encl, n) {
                    Err(e) => Err(e),
                    Ok(f1) => {
                        let ghost middle = arena.nodes@;
                        proof {
                            prefix(start, middle, deps); prefix(start, middle, outer);
                            prefix_all(start, middle, a@);
                        }
                        let r = flatten_list(arena, &a[1], w, env, deps, outer, encl, f1.n);
                        proof { crate::k2_load::prefix_chain(start, middle, arena.nodes@); }
                        match r {
                            Err(e) => Err(e),
                            Ok(f2) => {
                                proof { items_prefix(middle, arena.nodes@, f1.items@); }
                                Ok(join(arena, f1, f2))
                            },
                        }
                    },
                }
            } else { Err(named(arena, &Sym::InvalidDrsShape)) }
        },
        None => Err(named(arena, &Sym::InvalidDrsShape)),
    }
}

} // verus!
