use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_drs::{anchor, box_parts, modal};
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn strip_anchor(arena: &ETermArena, c: &T) -> (out: T)
    requires
        arena_ok(arena),
        valid(arena.nodes@, c),
    ensures
        valid(arena.nodes@, &out),
        out@ == spec::strip_anchor(c@),
        crate::k2_engine::term_size(out@) <= crate::k2_engine::term_size(c@),
{
    if anchor(arena, c).is_some() {
        let a = args(arena, c);
        proof {
            crate::m6_drs::child_size(c@, 0);
        }
        a[0].cp()
    } else {
        c.cp()
    }
}

pub fn supported_leaf(name: &Vec<u8>, arity: usize) -> (out: bool)
    ensures
        out == spec::supported_leaf(name@, arity as nat),
{
    (has_name(name, &Sym::Object) && arity == 6) || (has_name(name, &Sym::Predicate) && 3 <= arity
        && arity <= 5) || (has_name(name, &Sym::ModifierPp) && arity == 3) || (has_name(
        name,
        &Sym::Property,
    ) && arity == 3)
}

pub fn wh_tag_ok(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == spec::wh_tag_ok(t@),
{
    is_atom(arena, t, &Sym::Who) || is_atom(arena, t, &Sym::Which) || is_atom(arena, t, &Sym::What)
}

pub fn scan_box(arena: &ETermArena, b: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, b),
    ensures
        out == spec::scan_box(b@),
    decreases crate::k2_engine::term_size(b@), 0int,
{
    proof {
        reveal_with_fuel(spec::scan_box, 1);
    }
    match box_parts(arena, b) {
        None => false,
        Some(parts) => {
            proof {
                crate::m6_drs::child_size(b@, 1);
            }
            scan_conds(arena, &parts.conds)
        },
    }
}

pub fn scan_conds(arena: &ETermArena, l: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, l),
    ensures
        out == spec::scan_conds(l@),
    decreases crate::k2_engine::term_size(l@), 1int,
{
    proof {
        reveal_with_fuel(spec::scan_conds, 1);
        reveal_strlit("[|]");
        reveal(v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::Cons) == v1text::cons_name());
    }
    if is_nil(arena, l) {
        return true;
    }
    match parts(arena, l) {
        Some((name, args)) => {
            if has_name(&name, &Sym::Cons) && args.len() == 2 {
                let inner = strip_anchor(arena, &args[0]);
                proof {
                    crate::m6_drs::child_size(l@, 0);
                    crate::m6_drs::child_size(l@, 1);
                }
                scan_leaf(arena, &inner) && scan_conds(arena, &args[1])
            } else {
                false
            }
        },
        None => false,
    }
}

pub fn scan_leaf(arena: &ETermArena, leaf: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, leaf),
    ensures
        out == spec::scan_leaf(leaf@),
    decreases crate::k2_engine::term_size(leaf@), 2int,
{
    proof {
        reveal_with_fuel(spec::scan_leaf, 1);
    }
    match parts(arena, leaf) {
        None => false,
        Some((name, args)) => {
            if modal(&name) && args.len() == 1 {
                proof {
                    crate::m6_drs::child_size(leaf@, 0);
                }
                scan_box(arena, &args[0])
            } else if has_name(&name, &Sym::Query) && args.len() == 2 {
                wh_tag_ok(arena, &args[1])
            } else {
                supported_leaf(&name, args.len())
            }
        },
    }
}

pub fn strip_box(arena: &mut ETermArena, b: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, b),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::strip_box(b@),
    decreases crate::k2_engine::term_size(b@), 0int,
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::strip_box, 1);
    }
    match box_parts(arena, b) {
        None => b.cp(),
        Some(parts) => {
            let dom = list(arena, &parts.dom);
            let ghost middle = arena.nodes@;
            proof {
                prefix(start, middle, &parts.conds);
                crate::m6_drs::child_size(b@, 1);
            }
            let conds = strip_conds(arena, &parts.conds);
            let ghost before_build = arena.nodes@;
            proof {
                crate::k2_load::prefix_chain(start, middle, before_build);
                prefix(middle, before_build, &dom);
            }
            let out = c2(arena, &Sym::Drs, &dom, &conds);
            proof {
                crate::k2_load::prefix_chain(start, before_build, arena.nodes@);
            }
            out
        },
    }
}

pub fn strip_conds(arena: &mut ETermArena, l: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, l),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::strip_conds(l@),
    decreases crate::k2_engine::term_size(l@), 1int,
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::strip_conds, 1);
        reveal_strlit("[|]");
        reveal(v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::Cons) == v1text::cons_name());
    }
    match parts(arena, l) {
        Some((name, args)) => {
            if has_name(&name, &Sym::Cons) && args.len() == 2 {
                let inner = strip_anchor(arena, &args[0]);
                proof {
                    crate::m6_drs::child_size(l@, 0);
                    crate::m6_drs::child_size(l@, 1);
                }
                let rest = strip_conds(arena, &args[1]);
                let ghost n1 = arena.nodes@;
                proof {
                    prefix_all(start, n1, args@);
                    prefix(start, n1, &inner);
                }
                if is_comp(arena, &inner, &Sym::Query, 2) {
                    return rest;
                }
                if let Some((inner_name, a)) = parts(arena, &inner) {
                    if modal(&inner_name) && a.len() == 1 && is_comp(arena, &a[0], &Sym::Drs, 2) {
                        proof {
                            crate::m6_drs::child_size(inner@, 0);
                        }
                        let clean = strip_box(arena, &a[0]);
                        let ghost n2 = arena.nodes@;
                        let mut only = Vec::new();
                        only.push(clean);
                        proof {
                            assert_seqs_equal!(models(only@) == seq![clean@]);
                        }
                        let wrapped = comp(arena, &inner_name, &only);
                        let ghost n3 = arena.nodes@;
                        proof {
                            crate::k2_load::prefix_chain(n1, n2, n3);
                            crate::k2_load::prefix_chain(start, n1, n3);
                            prefix(n1, n3, &rest);
                        }
                        let out = c2(arena, &Sym::Cons, &wrapped, &rest);
                        proof {
                            crate::k2_load::prefix_chain(start, n3, arena.nodes@);
                        }
                        return out;
                    }
                }
                let out = c2(arena, &Sym::Cons, &args[0], &rest);
                proof {
                    crate::k2_load::prefix_chain(start, n1, arena.nodes@);
                }
                out
            } else {
                l.cp()
            }
        },
        None => l.cp(),
    }
}

} // verus!
