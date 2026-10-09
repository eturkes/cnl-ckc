// q12 D3–D6 + q13 D3/D4: the schema v3 annotation patterns — exec mirrors of
// ckc_spec::emit (declarations, plain, counted, window, frequency, order, the
// window check, approximation, ranged minimum).
use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_model::*;
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
use crate::m7_annotate::{
    EAnn, arg_at, assoc_exec, atom_bytes, bound_why_exec, int_cmp, obj_of_exec, of_links_exec,
    qty_exec,
};
#[cfg(verus_keep_ghost)]
use crate::m7_annotate::{
    ann_valid, ann_view, anns_valid, anns_view, opt_model, pairs_models, pairs_valid, pp_view,
    prepend, shape_of,
};
use crate::m7_temporal::ETemporal;
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::temporal::Temporal;
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

pub struct EDecl {
    pub var: usize,
    pub noun: Vec<u8>,
    pub bounded: bool,
}

pub open spec fn decl_view(d: &EDecl) -> spec::Decl {
    spec::Decl { var: d.var as nat, noun: d.noun@, bounded: d.bounded }
}

pub open spec fn decls_view(ds: Seq<EDecl>) -> Seq<spec::Decl> {
    ds.map_values(|d: EDecl| decl_view(&d))
}

// The annotation environment (spec::Env): the table + the referent declarations.
pub struct EEnv {
    pub tab: ETemporal,
    pub decls: Vec<EDecl>,
}

pub open spec fn env_view(e: &EEnv) -> spec::Env {
    spec::Env { tab: e.tab@, decls: decls_view(e.decls@) }
}

pub open spec fn envo_view(e: &Option<EEnv>) -> Option<spec::Env> {
    match e {
        Some(x) => Some(env_view(x)),
        None => None,
    }
}

pub open spec fn ttab_view(t: &Option<ETemporal>) -> Option<ckc_spec::temporal::Temporal> {
    match t {
        Some(x) => Some(x@),
        None => None,
    }
}

proof fn objects_all_concat(a: Seq<Term>, b: Seq<Term>)
    ensures
        spec::objects_all(a + b) == spec::objects_all(a) + spec::objects_all(b),
    decreases a.len(),
{
    reveal_with_fuel(spec::objects_all, 1);
    if a.len() == 0 {
        assert(a + b =~= b);
    } else {
        assert((a + b).drop_first() =~= a.drop_first() + b);
        objects_all_concat(a.drop_first(), b);
    }
}

// Every collected object is an `object/6` compound.
pub open spec fn all_objects(ts: Seq<Term>) -> bool {
    forall|i: int| 0 <= i < ts.len() ==> spec::is_comp(#[trigger] ts[i], "object"@, 6)
}

// spec::objects: the `object/6` subterms of t in preorder.
pub fn objects_exec(arena: &ETermArena, t: &T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::objects(t@),
        all_objects(models(out@)),
{
    let mut todo = Vec::new();
    todo.push(t.cp());
    let mut out: Vec<T> = Vec::new();
    proof {
        reveal_with_fuel(spec::objects_all, 2);
        assert(models(todo@) =~= seq![t@]);
        assert(models(out@) + spec::objects_all(models(todo@)) =~= spec::objects(t@));
    }
    while todo.len() > 0
        invariant
            arena_ok(arena),
            valid(arena.nodes@, t),
            valid_all(arena.nodes@, todo@),
            valid_all(arena.nodes@, out@),
            all_objects(models(out@)),
            models(out@) + spec::objects_all(models(todo@)) == spec::objects(t@),
        decreases crate::k2_engine::terms_size(models(todo@)),
    {
        let ghost before = todo@;
        let current = todo.remove(0);
        proof {
            assert(todo@ == before.drop_first());
            assert(models(before)[0] == current@);
            assert(models(before).drop_first() =~= models(todo@));
            reveal_with_fuel(spec::objects_all, 1);
            reveal_with_fuel(crate::k2_engine::terms_size, 1);
            assert(spec::objects_all(models(before)) == spec::objects(current@) + spec::objects_all(
                models(todo@),
            ));
            assert(crate::k2_engine::terms_size(models(before)) == crate::k2_engine::term_size(
                current@,
            ) + crate::k2_engine::terms_size(models(todo@)));
        }
        if is_comp(arena, &current, &Sym::Object, 6) {
            let ghost previous = out@;
            proof {
                extend(arena.nodes@, out@, current);
                reveal_with_fuel(spec::objects, 1);
                assert(spec::objects(current@) == seq![current@]);
            }
            out.push(current);
            proof {
                assert(models(out@) =~= models(previous).push(current@));
                assert forall|i: int| 0 <= i < models(out@).len() implies spec::is_comp(
                    #[trigger] models(out@)[i],
                    "object"@,
                    6,
                ) by {
                    if i < previous.len() {
                        assert(models(out@)[i] == models(previous)[i]);
                    }
                }
                assert(models(out@) + spec::objects_all(models(todo@)) =~= models(previous) + (seq![
                    current@,
                ] + spec::objects_all(models(todo@))));
                reveal(crate::k2_engine::term_size);
            }
        } else {
            match parts(arena, &current) {
                Some((_name, children)) => {
                    proof {
                        reveal_with_fuel(spec::objects, 1);
                        assert(spec::objects(current@) == spec::objects_all(models(children@)));
                        objects_all_concat(models(children@), models(todo@));
                        crate::m6_drs::sizes_concat(models(children@), models(todo@));
                        reveal(crate::k2_engine::term_size);
                        assert(crate::k2_engine::term_size(current@) == 1
                            + crate::k2_engine::terms_size(models(children@)));
                    }
                    todo = concat(&children, &todo);
                    proof {
                        assert forall|i: int| 0 <= i < todo@.len() implies #[trigger] valid(
                            arena.nodes@,
                            &todo@[i],
                        ) by {
                            if i < children@.len() {
                                assert(todo@[i] == children@[i]);
                            } else {
                                assert(todo@[i] == before.drop_first()[i - children@.len()]);
                            }
                        }
                    }
                },
                None => {
                    proof {
                        reveal_with_fuel(spec::objects, 1);
                        assert(spec::objects(current@) =~= Seq::<Term>::empty());
                        assert(models(out@) + spec::objects_all(models(todo@)) =~= models(out@) + (
                        Seq::<Term>::empty() + spec::objects_all(models(todo@))));
                        reveal(crate::k2_engine::term_size);
                    }
                },
            }
        }
    }
    proof {
        reveal_with_fuel(spec::objects_all, 1);
        assert(models(out@) + Seq::<Term>::empty() =~= models(out@));
    }
    out
}

// spec::decls_of over collected objects.
pub fn decls_exec(arena: &ETermArena, objs: &Vec<T>) -> (out: Vec<EDecl>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, objs@),
        all_objects(models(objs@)),
    ensures
        decls_view(out@) == spec::decls_of(models(objs@)),
{
    let ghost ms = models(objs@);
    let mut out: Vec<EDecl> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
        assert(decls_view(out@) + spec::decls_of(ms) =~= spec::decls_of(ms));
    }
    while i < objs.len()
        invariant
            i <= objs@.len(),
            arena_ok(arena),
            valid_all(arena.nodes@, objs@),
            all_objects(models(objs@)),
            ms == models(objs@),
            ms.len() == objs@.len(),
            spec::decls_of(ms) == decls_view(out@) + spec::decls_of(ms.skip(i as int)),
        decreases objs@.len() - i,
    {
        let ghost before = out@;
        let o = &objs[i];
        proof {
            assert(ms[i as int] == o@);
            assert(spec::is_comp(o@, "object"@, 6));
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
            reveal_with_fuel(spec::decls_of, 1);
        }
        let a0 = arg_at(arena, o, 0);
        if is_var(arena, &a0) {
            let var = var_index(arena, &a0);
            let a1 = arg_at(arena, o, 1);
            let noun = atom_bytes(arena, &a1);
            let bounded = bound_why_exec(arena, o).is_none();
            out.push(EDecl { var, noun, bounded });
            proof {
                assert(decls_view(out@) =~= decls_view(before).push(
                    spec::Decl { var: var as nat, noun: noun@, bounded },
                ));
            }
        }
        proof {
            let rest = spec::decls_of(ms.skip(i + 1));
            assert(spec::decls_of(ms.skip(i as int)) == (match ckc_spec::replay::arg(o@, 0) {
                Term::Var(k) => seq![
                    spec::Decl {
                        var: k,
                        noun: spec::noun_of(o@),
                        bounded: spec::bound_why(o@) is None,
                    },
                ],
                _ => Seq::empty(),
            }) + rest);
            assert(spec::decls_of(ms) =~= decls_view(out@) + rest);
        }
        i += 1;
    }
    proof {
        assert(ms.skip(i as int).len() == 0);
        reveal_with_fuel(spec::decls_of, 1);
        assert(decls_view(out@) + Seq::<spec::Decl>::empty() =~= decls_view(out@));
    }
    out
}

pub fn clone_decls(ds: &Vec<EDecl>) -> (out: Vec<EDecl>)
    ensures
        decls_view(out@) == decls_view(ds@),
{
    let mut out: Vec<EDecl> = Vec::new();
    let mut i = 0usize;
    while i < ds.len()
        invariant
            i <= ds@.len(),
            decls_view(out@) == decls_view(ds@).take(i as int),
        decreases ds@.len() - i,
    {
        let ghost before = out@;
        out.push(
            EDecl {
                var: ds[i].var,
                noun: crate::k4_bytes::copy(&ds[i].noun),
                bounded: ds[i].bounded,
            },
        );
        proof {
            assert(decls_view(out@) =~= decls_view(before).push(decl_view(&ds@[i as int])));
            assert(decls_view(ds@).take(i + 1) =~= decls_view(ds@).take(i as int).push(
                decl_view(&ds@[i as int]),
            ));
        }
        i += 1;
    }
    proof {
        assert(decls_view(ds@).take(ds@.len() as int) =~= decls_view(ds@));
    }
    out
}

pub fn clone_env(e: &Option<EEnv>) -> (out: Option<EEnv>)
    ensures
        envo_view(&out) == envo_view(e),
{
    match e {
        None => None,
        Some(x) => Some(
            EEnv { tab: crate::m7_temporal::clone_tab(&x.tab), decls: clone_decls(&x.decls) },
        ),
    }
}

// spec::env_of: the table (when present) + the declarations of the DRS.
pub fn env_exec(arena: &ETermArena, tab: &Option<ETemporal>, drs: &T) -> (out: Option<EEnv>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, drs),
    ensures
        envo_view(&out) == spec::env_of(ttab_view(tab), drs@),
{
    match tab {
        None => None,
        Some(t) => {
            let objs = objects_exec(arena, drs);
            let decls = decls_exec(arena, &objs);
            Some(EEnv { tab: crate::m7_temporal::clone_tab(t), decls })
        },
    }
}

// spec::tab_version: 1 without a table, else the table's header version.
pub fn tab_version_exec(t: &Option<ETemporal>) -> (out: u8)
    ensures
        out as nat == spec::tab_version(ttab_view(t)),
{
    match t {
        None => 1,
        Some(x) => x.version,
    }
}

// spec::decl: the index of variable k's first declaration.
pub fn decl_exec(ds: &Vec<EDecl>, k: usize) -> (out: Option<usize>)
    ensures
        match out {
            None => spec::decl(decls_view(ds@), k as nat) is None,
            Some(i) => i < ds@.len() && spec::decl(decls_view(ds@), k as nat) == Some(
                decl_view(&ds@[i as int]),
            ),
        },
{
    let ghost dv = decls_view(ds@);
    let mut i = 0usize;
    proof {
        assert(dv.skip(0) =~= dv);
    }
    while i < ds.len()
        invariant
            i <= ds@.len(),
            dv == decls_view(ds@),
            dv.len() == ds@.len(),
            spec::decl(dv, k as nat) == spec::decl(dv.skip(i as int), k as nat),
        decreases ds@.len() - i,
    {
        proof {
            assert(dv.skip(i as int)[0] == dv[i as int]);
            assert(dv.skip(i as int).drop_first() =~= dv.skip(i + 1));
            assert(dv[i as int] == decl_view(&ds@[i as int]));
        }
        if ds[i].var == k {
            return Some(i);
        }
        i += 1;
    }
    proof {
        assert(dv.skip(i as int).len() == 0);
    }
    None
}

// spec::plain: a variable declared with a noun that is no unit lemma and no frame noun.
pub fn plain_exec(arena: &ETermArena, env: &EEnv, v: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, v),
    ensures
        out == spec::plain(env_view(env), v@),
{
    if !is_var(arena, v) {
        return false;
    }
    let k = var_index(arena, v);
    match decl_exec(&env.decls, k) {
        None => false,
        Some(i) => {
            let n = &env.decls[i].noun;
            n.len() > 0 && assoc_exec(&env.tab.units, n).is_none() && !crate::m7_temporal::frame_in(
                &env.tab,
                n,
            )
        },
    }
}

pub fn frame_shaped_exec(arena: &ETermArena, o: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, o),
        spec::is_comp(o@, "object"@, 6),
    ensures
        out == spec::frame_shaped(o@),
{
    let a2 = arg_at(arena, o, 2);
    let a3 = arg_at(arena, o, 3);
    let a4 = arg_at(arena, o, 4);
    let a5 = arg_at(arena, o, 5);
    is_atom(arena, &a2, &Sym::Countable) && is_atom(arena, &a3, &Sym::Na) && is_atom(
        arena,
        &a4,
        &Sym::Eq,
    ) && int_cmp(arena, &a5, true)
}

// spec::pred_of: E's first same-context predicate/4|5 condition.
pub fn pred_of_exec(arena: &ETermArena, items: &Vec<I>, ctx: &T, e: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, e),
    ensures
        out matches Some(p) ==> valid(arena.nodes@, &p) && (spec::is_comp(p@, "predicate"@, 4)
            || spec::is_comp(p@, "predicate"@, 5)),
        opt_model(out) == spec::pred_of(item_models(items@), ctx@, e@),
{
    let ghost ms = item_models(items@);
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
    }
    while i < items.len()
        invariant
            i <= items@.len(),
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid(arena.nodes@, ctx),
            valid(arena.nodes@, e),
            ms == item_models(items@),
            ms.len() == items@.len(),
            spec::pred_of(ms, ctx@, e@) == spec::pred_of(ms.skip(i as int), ctx@, e@),
        decreases items@.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms[i as int] == items@[i as int]@);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
        }
        match &items[i].kind {
            Kind::Anch(c, inner) => {
                if crate::m6_term::equal(arena, c, ctx) && (is_comp(
                    arena,
                    inner,
                    &Sym::Predicate,
                    4,
                ) || is_comp(arena, inner, &Sym::Predicate, 5)) {
                    let a0 = arg_at(arena, inner, 0);
                    if crate::m6_term::equal(arena, &a0, e) {
                        return Some(inner.cp());
                    }
                }
            },
            _ => {},
        }
        i += 1;
    }
    proof {
        assert(ms.skip(i as int).len() == 0);
    }
    None
}

// spec::counted: E's second participant, a plain bounded referent.
pub fn counted_exec(arena: &ETermArena, env: &EEnv, items: &Vec<I>, ctx: &T, e: &T) -> (out: Option<
    T,
>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, e),
    ensures
        out matches Some(c) ==> valid(arena.nodes@, &c),
        opt_model(out) == spec::counted(env_view(env), item_models(items@), ctx@, e@),
{
    match pred_of_exec(arena, items, ctx, e) {
        None => None,
        Some(pr) => {
            let c = arg_at(arena, &pr, 3);
            if !is_var(arena, &c) {
                return None;
            }
            let k = var_index(arena, &c);
            match decl_exec(&env.decls, k) {
                None => None,
                Some(i) => {
                    if plain_exec(arena, env, &c) && env.decls[i].bounded {
                        Some(c)
                    } else {
                        None
                    }
                },
            }
        },
    }
}

// spec::window_pps over the whole scope: the window pps on event e in context ctx.
pub fn window_pps_exec(arena: &ETermArena, env: &EEnv, items: &Vec<I>, ctx: &T, e: &T) -> (out:
    usize)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, e),
    ensures
        out as nat == spec::window_pps(
            env_view(env).tab,
            item_models(items@),
            item_models(items@),
            ctx@,
            e@,
        ),
{
    let ghost ms = item_models(items@);
    let mut n = 0usize;
    let mut i = items.len();
    proof {
        assert(ms.skip(items@.len() as int) =~= Seq::<spec::Item>::empty());
    }
    while i > 0
        invariant
            i <= items@.len(),
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid(arena.nodes@, ctx),
            valid(arena.nodes@, e),
            ms == item_models(items@),
            ms.len() == items@.len(),
            n + i <= items.len(),
            n as nat == spec::window_pps(env_view(env).tab, ms, ms.skip(i as int), ctx@, e@),
        decreases i,
    {
        i -= 1;
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms[i as int] == items@[i as int]@);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
            reveal_with_fuel(spec::window_pps, 1);
        }
        let hit = match &items[i].kind {
            Kind::Anch(c, inner) => {
                if crate::m6_term::equal(arena, c, ctx) && is_comp(
                    arena,
                    inner,
                    &Sym::ModifierPp,
                    3,
                ) {
                    let e0 = arg_at(arena, inner, 0);
                    if crate::m6_term::equal(arena, &e0, e) {
                        let x = arg_at(arena, inner, 2);
                        match obj_of_exec(arena, items, ctx, &x) {
                            Some(o) => {
                                let a1 = arg_at(arena, inner, 1);
                                let p = atom_bytes(arena, &a1);
                                let o1 = arg_at(arena, &o, 1);
                                let noun = atom_bytes(arena, &o1);
                                crate::m7_temporal::pair_in(&env.tab.windows, &p, &noun)
                            },
                            None => false,
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            _ => false,
        };
        if hit {
            n += 1;
        }
    }
    proof {
        assert(ms.skip(0) =~= ms);
    }
    n
}

// spec::window_ann (o = the pp object's declaration in scope).
pub fn window_ann_exec(
    arena: &ETermArena,
    env: &EEnv,
    items: &Vec<I>,
    ctx: &T,
    pp: &T,
    o: &T,
) -> (out: Result<Option<EAnn>, Sym>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, pp),
        valid(arena.nodes@, o),
        spec::is_comp(pp@, "modifier_pp"@, 3),
        spec::is_comp(o@, "object"@, 6),
    ensures
        out matches Ok(Some(a)) ==> ann_valid(arena.nodes@, &a),
        pp_view(out) == spec::window_ann(env_view(env), item_models(items@), ctx@, pp@, o@),
{
    let e = arg_at(arena, pp, 0);
    let f = arg_at(arena, pp, 2);
    let fl = of_links_exec(arena, items, ctx, &f);
    if window_pps_exec(arena, env, items, ctx, &e) > 1 || !frame_shaped_exec(arena, o) || fl.len()
        != 1 {
        return Err(Sym::WindowShape);
    }
    proof {
        assert(models(fl@)[0] == fl@[0]@);
    }
    let l = fl[0].cp();
    match qty_exec(arena, &env.tab, items, ctx, &l) {
        None => Err(Sym::WindowShape),
        Some((unit, lo)) => {
            if let Some(why) = bound_why_exec(arena, &lo) {
                return Err(why);
            }
            if approx_why_exec(arena, &env.tab, items, ctx, &l, &f, &lo) {
                return Err(Sym::ApproximateBound);
            }
            let al = of_links_exec(arena, items, ctx, &l);
            if al.len() != 1 {
                return Err(Sym::WindowShape);
            }
            proof {
                assert(models(al@)[0] == al@[0]@);
            }
            let a = al[0].cp();
            if !plain_exec(arena, env, &a) {
                return Err(Sym::WindowShape);
            }
            Ok(Some(EAnn::Window(e, f, l, unit, a)))
        },
    }
}

// spec::frequency_ann.
pub fn frequency_ann_exec(arena: &ETermArena, env: &EEnv, items: &Vec<I>, ctx: &T, pp: &T) -> (out:
    Result<Option<EAnn>, Sym>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, pp),
        spec::is_comp(pp@, "modifier_pp"@, 3),
    ensures
        out matches Ok(Some(a)) ==> ann_valid(arena.nodes@, &a),
        pp_view(out) == spec::frequency_ann(env_view(env), item_models(items@), ctx@, pp@),
{
    let e = arg_at(arena, pp, 0);
    let w = arg_at(arena, pp, 2);
    match qty_exec(arena, &env.tab, items, ctx, &w) {
        None => Ok(None),
        Some((unit, wo)) => {
            if let Some(why) = bound_why_exec(arena, &wo) {
                return Err(why);
            }
            let w4 = arg_at(arena, &wo, 4);
            if !is_atom(arena, &w4, &Sym::Eq) || of_links_exec(arena, items, ctx, &w).len() > 0 {
                return Err(Sym::FrequencyShape);
            }
            match counted_exec(arena, env, items, ctx, &e) {
                None => Err(Sym::FrequencyShape),
                Some(c) => Ok(Some(EAnn::Frequency(e, c, w, unit))),
            }
        },
    }
}

// spec::v3_ann: window, frequency, then order (v2 tables: none).
pub fn v3_ann_exec(arena: &ETermArena, env: &EEnv, items: &Vec<I>, ctx: &T, pp: &T) -> (out: Result<
    Option<EAnn>,
    Sym,
>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, pp),
        spec::is_comp(pp@, "modifier_pp"@, 3),
    ensures
        out matches Ok(Some(a)) ==> ann_valid(arena.nodes@, &a),
        pp_view(out) == spec::v3_ann(env_view(env), item_models(items@), ctx@, pp@),
{
    if env.tab.version != 3 {
        return Ok(None);
    }
    let e = arg_at(arena, pp, 0);
    let a1 = arg_at(arena, pp, 1);
    let x = arg_at(arena, pp, 2);
    let p = atom_bytes(arena, &a1);
    let role = assoc_exec(&env.tab.relations, &p);
    let ordered = match &role {
        Some(r) => crate::k4_bytes::eq(r, b"before") || crate::k4_bytes::eq(r, b"after"),
        None => false,
    };
    proof {
        reveal_byteslit(b"before");
        reveal_byteslit(b"after");
        reveal_strlit("before");
        reveal_strlit("after");
        reveal(ckc_spec::v1text::ascii);
        assert(b"before"@ =~= ckc_spec::v1text::ascii("before"@));
        assert(b"after"@ =~= ckc_spec::v1text::ascii("after"@));
    }
    match obj_of_exec(arena, items, ctx, &x) {
        Some(o) => {
            let o1 = arg_at(arena, &o, 1);
            let noun = atom_bytes(arena, &o1);
            if crate::m7_temporal::pair_in(&env.tab.windows, &p, &noun) {
                return window_ann_exec(arena, env, items, ctx, pp, &o);
            }
            if crate::m7_temporal::seq_in(&env.tab.frequencies, &p) {
                return frequency_ann_exec(arena, env, items, ctx, pp);
            }
            if crate::m7_temporal::seq_in(&env.tab.ranges, &p) && qty_exec(
                arena,
                &env.tab,
                items,
                ctx,
                &x,
            ).is_some() {
                return range_ann_exec(arena, env, items, ctx, pp);
            }
            if ordered && plain_exec(arena, env, &x) {
                return Ok(Some(EAnn::Order(e, role.unwrap(), x)));
            }
            Ok(None)
        },
        None => {
            if ordered && plain_exec(arena, env, &x) {
                return Ok(Some(EAnn::Order(e, role.unwrap(), x)));
            }
            Ok(None)
        },
    }
}

// spec::window_of: the scope's window on event e in context c.
pub fn window_of_exec(arena: &ETermArena, ms: &Vec<(T, EAnn)>, c: &T, e: &T) -> (out: Option<
    (T, Vec<u8>, T),
>)
    requires
        arena_ok(arena),
        anns_valid(arena.nodes@, ms@),
        valid(arena.nodes@, c),
        valid(arena.nodes@, e),
    ensures
        out matches Some((l, _, a)) ==> valid(arena.nodes@, &l) && valid(arena.nodes@, &a),
        match out {
            Some((l, u, a)) => spec::window_of(anns_view(ms@), c@, e@) == Some((l@, u@, a@)),
            None => spec::window_of(anns_view(ms@), c@, e@) is None,
        },
{
    let ghost av = anns_view(ms@);
    let mut i = 0usize;
    proof {
        assert(av.skip(0) =~= av);
    }
    while i < ms.len()
        invariant
            i <= ms@.len(),
            arena_ok(arena),
            anns_valid(arena.nodes@, ms@),
            valid(arena.nodes@, c),
            valid(arena.nodes@, e),
            av == anns_view(ms@),
            av.len() == ms@.len(),
            spec::window_of(av, c@, e@) == spec::window_of(av.skip(i as int), c@, e@),
        decreases ms@.len() - i,
    {
        proof {
            assert(av.skip(i as int)[0] == av[i as int]);
            assert(av.skip(i as int).drop_first() =~= av.skip(i + 1));
            assert(av[i as int] == (ms@[i as int].0@, ann_view(&ms@[i as int].1)));
            assert(valid(arena.nodes@, &ms@[i as int].0) && ann_valid(
                arena.nodes@,
                &ms@[i as int].1,
            ));
        }
        match &ms[i].1 {
            EAnn::Window(e2, _, l, u, a) => {
                if crate::m6_term::equal(arena, &ms[i].0, c) && crate::m6_term::equal(
                    arena,
                    e2,
                    e,
                ) {
                    return Some((l.cp(), crate::k4_bytes::copy(u), a.cp()));
                }
            },
            _ => {},
        }
        i += 1;
    }
    proof {
        assert(av.skip(i as int).len() == 0);
    }
    None
}

pub open spec fn event_t(a: &EAnn) -> T {
    match a {
        EAnn::Interval(e, _, _, _, _) => *e,
        EAnn::Recurrence(e, _, _, _) => *e,
        EAnn::Window(e, _, _, _, _) => *e,
        EAnn::Frequency(e, _, _, _) => *e,
        EAnn::Order(e, _, _) => *e,
        EAnn::Range(e, _, _) => *e,
    }
}

fn event_exec(a: &EAnn) -> (out: T)
    ensures
        out@ == spec::event_of(ann_view(a)),
        out == event_t(a),
{
    match a {
        EAnn::Interval(e, _, _, _, _) => e.cp(),
        EAnn::Recurrence(e, _, _, _) => e.cp(),
        EAnn::Window(e, _, _, _, _) => e.cp(),
        EAnn::Frequency(e, _, _, _) => e.cp(),
        EAnn::Order(e, _, _) => e.cp(),
        EAnn::Range(e, _, _) => e.cp(),
    }
}

// spec::recurrences_on: recurrences on event e in context c.
pub fn recurrences_on_exec(arena: &ETermArena, ms: &Vec<(T, EAnn)>, c: &T, e: &T) -> (out: usize)
    requires
        arena_ok(arena),
        anns_valid(arena.nodes@, ms@),
        valid(arena.nodes@, c),
        valid(arena.nodes@, e),
    ensures
        out as nat == spec::recurrences_on(anns_view(ms@), c@, e@),
{
    let ghost av = anns_view(ms@);
    let mut n = 0usize;
    let mut i = ms.len();
    proof {
        assert(av.skip(ms@.len() as int) =~= Seq::<(Term, spec::Ann)>::empty());
    }
    while i > 0
        invariant
            i <= ms@.len(),
            arena_ok(arena),
            anns_valid(arena.nodes@, ms@),
            valid(arena.nodes@, c),
            valid(arena.nodes@, e),
            av == anns_view(ms@),
            av.len() == ms@.len(),
            n + i <= ms.len(),
            n as nat == spec::recurrences_on(av.skip(i as int), c@, e@),
        decreases i,
    {
        i -= 1;
        proof {
            assert(av.skip(i as int)[0] == av[i as int]);
            assert(av.skip(i as int).drop_first() =~= av.skip(i + 1));
            assert(av[i as int] == (ms@[i as int].0@, ann_view(&ms@[i as int].1)));
            assert(valid(arena.nodes@, &ms@[i as int].0) && ann_valid(
                arena.nodes@,
                &ms@[i as int].1,
            ));
            reveal_with_fuel(spec::recurrences_on, 1);
        }
        let ev = event_exec(&ms[i].1);
        proof {
            assert(valid(arena.nodes@, &ev)) by {
                match &ms@[i as int].1 {
                    EAnn::Interval(_, _, _, _, _) => {},
                    EAnn::Recurrence(_, _, _, _) => {},
                    EAnn::Window(_, _, _, _, _) => {},
                    EAnn::Frequency(_, _, _, _) => {},
                    EAnn::Order(_, _, _) => {},
                    EAnn::Range(_, _, _) => {},
                }
            }
        }
        let kind = match &ms[i].1 {
            EAnn::Recurrence(..) => true,
            _ => false,
        };
        if kind && crate::m6_term::equal(arena, &ms[i].0, c) && crate::m6_term::equal(
            arena,
            &ev,
            e,
        ) {
            n += 1;
        }
    }
    proof {
        assert(av.skip(0) =~= av);
    }
    n
}

// spec::windows_ok: each window's event carries ≥1 recurrence.
pub fn windows_ok_exec(arena: &ETermArena, ms: &Vec<(T, EAnn)>) -> (out: bool)
    requires
        arena_ok(arena),
        anns_valid(arena.nodes@, ms@),
    ensures
        out == spec::windows_ok(anns_view(ms@), anns_view(ms@)),
{
    let ghost av = anns_view(ms@);
    let mut i = 0usize;
    proof {
        assert(av.skip(0) =~= av);
    }
    while i < ms.len()
        invariant
            i <= ms@.len(),
            arena_ok(arena),
            anns_valid(arena.nodes@, ms@),
            av == anns_view(ms@),
            av.len() == ms@.len(),
            spec::windows_ok(av, av) == spec::windows_ok(av.skip(i as int), av),
        decreases ms@.len() - i,
    {
        proof {
            assert(av.skip(i as int)[0] == av[i as int]);
            assert(av.skip(i as int).drop_first() =~= av.skip(i + 1));
            assert(av[i as int] == (ms@[i as int].0@, ann_view(&ms@[i as int].1)));
            assert(valid(arena.nodes@, &ms@[i as int].0) && ann_valid(
                arena.nodes@,
                &ms@[i as int].1,
            ));
            reveal_with_fuel(spec::windows_ok, 1);
        }
        match &ms[i].1 {
            EAnn::Window(e, _, _, _, _) => {
                if recurrences_on_exec(arena, ms, &ms[i].0, e) == 0 {
                    return false;
                }
            },
            _ => {},
        }
        i += 1;
    }
    proof {
        assert(av.skip(i as int).len() == 0);
        reveal_with_fuel(spec::windows_ok, 1);
    }
    true
}

// --- q13 D3/D4: about + ranged minimum ---
// spec::approximate: a same-context approximation property on r.
pub fn approximate_exec(
    arena: &ETermArena,
    tab: &ETemporal,
    items: &Vec<I>,
    ctx: &T,
    r: &T,
) -> (out: bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, r),
    ensures
        out == spec::approximate(tab@, item_models(items@), ctx@, r@),
{
    let ghost ms = item_models(items@);
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
    }
    while i < items.len()
        invariant
            i <= items@.len(),
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid(arena.nodes@, ctx),
            valid(arena.nodes@, r),
            ms == item_models(items@),
            ms.len() == items@.len(),
            spec::approximate(tab@, ms, ctx@, r@) == spec::approximate(
                tab@,
                ms.skip(i as int),
                ctx@,
                r@,
            ),
        decreases items@.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms[i as int] == items@[i as int]@);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
            reveal_with_fuel(spec::approximate, 1);
        }
        let hit = match &items[i].kind {
            Kind::Anch(c, inner) => {
                if crate::m6_term::equal(arena, c, ctx) && is_comp(
                    arena,
                    inner,
                    &Sym::Property,
                    3,
                ) {
                    let a0 = arg_at(arena, inner, 0);
                    let a1 = arg_at(arena, inner, 1);
                    let a2 = arg_at(arena, inner, 2);
                    let noun = atom_bytes(arena, &a1);
                    crate::m6_term::equal(arena, &a0, r) && is_atom(arena, &a2, &Sym::Pos)
                        && crate::m7_temporal::seq_in(&tab.approximations, &noun)
                } else {
                    false
                }
            },
            _ => false,
        };
        if hit {
            return true;
        }
        i += 1;
    }
    proof {
        assert(ms.skip(i as int).len() == 0);
        reveal_with_fuel(spec::approximate, 1);
    }
    false
}

// spec::approx_why is Some: q (object o, frame f) is approximate and not eq.
pub fn approx_why_exec(
    arena: &ETermArena,
    tab: &ETemporal,
    items: &Vec<I>,
    ctx: &T,
    q: &T,
    f: &T,
    o: &T,
) -> (out: bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, q),
        valid(arena.nodes@, f),
        valid(arena.nodes@, o),
        spec::is_comp(o@, "object"@, 6),
    ensures
        out == (spec::approx_why(tab@, item_models(items@), ctx@, q@, f@, o@) is Some),
        out ==> spec::approx_why(tab@, item_models(items@), ctx@, q@, f@, o@) == Some(
            Term::Comp(
                symbol(&Sym::TemporalShape),
                seq![Term::Atom(symbol(&Sym::ApproximateBound))],
            ),
        ),
{
    let o4 = arg_at(arena, o, 4);
    let marked = approximate_exec(arena, tab, items, ctx, q) || approximate_exec(
        arena,
        tab,
        items,
        ctx,
        f,
    );
    marked && !is_atom(arena, &o4, &Sym::Eq)
}

// spec::range_ann.
pub fn range_ann_exec(arena: &ETermArena, env: &EEnv, items: &Vec<I>, ctx: &T, pp: &T) -> (out:
    Result<Option<EAnn>, Sym>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, pp),
        spec::is_comp(pp@, "modifier_pp"@, 3),
    ensures
        out matches Ok(Some(a)) ==> ann_valid(arena.nodes@, &a),
        pp_view(out) == spec::range_ann(env_view(env), item_models(items@), ctx@, pp@),
{
    let e = arg_at(arena, pp, 0);
    let h = arg_at(arena, pp, 2);
    match qty_exec(arena, &env.tab, items, ctx, &h) {
        None => Ok(None),
        Some((unit, ho)) => {
            if let Some(why) = bound_why_exec(arena, &ho) {
                return Err(why);
            }
            let h4 = arg_at(arena, &ho, 4);
            if !is_atom(arena, &h4, &Sym::Eq) || of_links_exec(arena, items, ctx, &h).len() > 0
                || approximate_exec(arena, &env.tab, items, ctx, &h) {
                return Err(Sym::RangeShape);
            }
            Ok(Some(EAnn::Range(e, h, unit)))
        },
    }
}

pub open spec fn partners_view(v: Seq<(T, Vec<u8>)>) -> Seq<(Term, Seq<u8>)> {
    v.map_values(|p: (T, Vec<u8>)| (p.0@, p.1@))
}

// spec::partners: the at-least interval quantities on event e in context c.
pub fn partners_exec(arena: &ETermArena, scope: &Vec<I>, ms: &Vec<(T, EAnn)>, c: &T, e: &T) -> (out:
    Vec<(T, Vec<u8>)>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        anns_valid(arena.nodes@, ms@),
        valid(arena.nodes@, c),
        valid(arena.nodes@, e),
    ensures
        forall|j: int| 0 <= j < out@.len() ==> valid(arena.nodes@, &(#[trigger] out@[j]).0),
        partners_view(out@) == spec::partners(item_models(scope@), anns_view(ms@), c@, e@),
{
    let ghost sm = item_models(scope@);
    let ghost av = anns_view(ms@);
    let mut out: Vec<(T, Vec<u8>)> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(av.skip(0) =~= av);
        assert(partners_view(out@) + spec::partners(sm, av, c@, e@) =~= spec::partners(
            sm,
            av,
            c@,
            e@,
        ));
    }
    while i < ms.len()
        invariant
            i <= ms@.len(),
            arena_ok(arena),
            items_valid(arena.nodes@, scope@),
            anns_valid(arena.nodes@, ms@),
            valid(arena.nodes@, c),
            valid(arena.nodes@, e),
            sm == item_models(scope@),
            av == anns_view(ms@),
            av.len() == ms@.len(),
            forall|j: int| 0 <= j < out@.len() ==> valid(arena.nodes@, &(#[trigger] out@[j]).0),
            spec::partners(sm, av, c@, e@) == partners_view(out@) + spec::partners(
                sm,
                av.skip(i as int),
                c@,
                e@,
            ),
        decreases ms@.len() - i,
    {
        proof {
            assert(av.skip(i as int)[0] == av[i as int]);
            assert(av.skip(i as int).drop_first() =~= av.skip(i + 1));
            assert(av[i as int] == (ms@[i as int].0@, ann_view(&ms@[i as int].1)));
            assert(valid(arena.nodes@, &ms@[i as int].0) && ann_valid(
                arena.nodes@,
                &ms@[i as int].1,
            ));
            reveal_with_fuel(spec::partners, 1);
        }
        let ghost before = out@;
        let ghost mut h: Seq<(Term, Seq<u8>)> = Seq::empty();
        match &ms[i].1 {
            EAnn::Interval(e2, _, q, unit, _) => {
                if crate::m6_term::equal(arena, &ms[i].0, c) && crate::m6_term::equal(
                    arena,
                    e2,
                    e,
                ) {
                    match obj_of_exec(arena, scope, c, q) {
                        Some(o) => {
                            let o4 = arg_at(arena, &o, 4);
                            if is_atom(arena, &o4, &Sym::Geq) {
                                proof {
                                    h = seq![(q@, unit@)];
                                }
                                out.push((q.cp(), crate::k4_bytes::copy(unit)));
                            }
                        },
                        None => {},
                    }
                }
            },
            _ => {},
        }
        proof {
            assert(partners_view(out@) =~= partners_view(before) + h);
            assert(spec::partners(sm, av.skip(i as int), c@, e@) == h + spec::partners(
                sm,
                av.skip(i + 1),
                c@,
                e@,
            ));
            assert((partners_view(before) + h) + spec::partners(sm, av.skip(i + 1), c@, e@)
                =~= partners_view(before) + (h + spec::partners(sm, av.skip(i + 1), c@, e@)));
            assert forall|j: int| 0 <= j < out@.len() implies valid(
                arena.nodes@,
                &(#[trigger] out@[j]).0,
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(av.skip(i as int).len() == 0);
        assert(partners_view(out@) + Seq::<(Term, Seq<u8>)>::empty() =~= partners_view(out@));
    }
    out
}

// spec::count_of(scope, c, l) < spec::count_of(scope, c, h).
pub fn count_less_exec(arena: &ETermArena, scope: &Vec<I>, c: &T, l: &T, h: &T) -> (out: bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        valid(arena.nodes@, c),
        valid(arena.nodes@, l),
        valid(arena.nodes@, h),
    ensures
        out == (spec::count_of(item_models(scope@), c@, l@) < spec::count_of(
            item_models(scope@),
            c@,
            h@,
        )),
{
    let a = match obj_of_exec(arena, scope, c, l) {
        Some(o) => {
            let a5 = arg_at(arena, &o, 5);
            Some(a5.root)
        },
        None => None,
    };
    let b = match obj_of_exec(arena, scope, c, h) {
        Some(o) => {
            let b5 = arg_at(arena, &o, 5);
            Some(b5.root)
        },
        None => None,
    };
    crate::k2_term::opt_int_less(arena, a, b)
}

pub open spec fn lows_res(r: Result<Vec<(T, T)>, Sym>) -> Result<Seq<(Term, Term)>, Term> {
    match r {
        Ok(v) => Ok(pairs_models(v@)),
        Err(s) => Err(shape_of(s)),
    }
}

// spec::range_lows(scope, ms, ms): each range's (context, low end), else range_shape.
pub fn range_lows_exec(arena: &ETermArena, scope: &Vec<I>, ms: &Vec<(T, EAnn)>) -> (out: Result<
    Vec<(T, T)>,
    Sym,
>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        anns_valid(arena.nodes@, ms@),
    ensures
        out matches Ok(v) ==> pairs_valid(arena.nodes@, v@),
        lows_res(out) == spec::range_lows(item_models(scope@), anns_view(ms@), anns_view(ms@)),
{
    let ghost sm = item_models(scope@);
    let ghost av = anns_view(ms@);
    let mut out: Vec<(T, T)> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(av.skip(0) =~= av);
        assert(pairs_models(out@) =~= Seq::<(Term, Term)>::empty());
        match spec::range_lows(sm, av, av) {
            Ok(t) => assert(Seq::<(Term, Term)>::empty() + t =~= t),
            Err(_) => {},
        }
    }
    while i < ms.len()
        invariant
            i <= ms@.len(),
            arena_ok(arena),
            items_valid(arena.nodes@, scope@),
            anns_valid(arena.nodes@, ms@),
            sm == item_models(scope@),
            av == anns_view(ms@),
            av.len() == ms@.len(),
            pairs_valid(arena.nodes@, out@),
            spec::range_lows(sm, av, av) == prepend(
                pairs_models(out@),
                spec::range_lows(sm, av, av.skip(i as int)),
            ),
        decreases ms@.len() - i,
    {
        proof {
            assert(av.skip(i as int)[0] == av[i as int]);
            assert(av.skip(i as int).drop_first() =~= av.skip(i + 1));
            assert(av[i as int] == (ms@[i as int].0@, ann_view(&ms@[i as int].1)));
            assert(valid(arena.nodes@, &ms@[i as int].0) && ann_valid(
                arena.nodes@,
                &ms@[i as int].1,
            ));
            reveal_with_fuel(spec::range_lows, 1);
        }
        let ghost before = out@;
        let ghost rest = spec::range_lows(sm, av, av.skip(i + 1));
        let ghost mut hd: Seq<(Term, Term)> = Seq::empty();
        match &ms[i].1 {
            EAnn::Range(e, h, unit) => {
                let ps = partners_exec(arena, scope, ms, &ms[i].0, e);
                if ps.len() != 1 {
                    return Err(Sym::RangeShape);
                }
                proof {
                    assert(partners_view(ps@)[0] == (ps@[0].0@, ps@[0].1@));
                }
                if !crate::k4_bytes::eq(&ps[0].1, unit) {
                    return Err(Sym::RangeShape);
                }
                if !count_less_exec(arena, scope, &ms[i].0, &ps[0].0, h) {
                    return Err(Sym::RangeShape);
                }
                proof {
                    hd = seq![(ms@[i as int].0@, ps@[0].0@)];
                }
                out.push((ms[i].0.cp(), ps[0].0.cp()));
            },
            _ => {},
        }
        proof {
            assert(pairs_models(out@) =~= pairs_models(before) + hd);
            match rest {
                Ok(t) => assert((pairs_models(before) + hd) + t =~= pairs_models(before) + (hd
                    + t)),
                Err(_) => {},
            }
            assert forall|j: int| 0 <= j < out@.len() implies valid(
                arena.nodes@,
                &(#[trigger] out@[j]).0,
            ) && valid(arena.nodes@, &out@[j].1) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(av.skip(i as int).len() == 0);
        assert(pairs_models(out@) + Seq::<(Term, Term)>::empty() =~= pairs_models(out@));
    }
    Ok(out)
}

// spec::eq_approx.
pub fn eq_approx_exec(
    arena: &ETermArena,
    tab: &ETemporal,
    scope: &Vec<I>,
    c: &T,
    q: &T,
    f: &T,
) -> (out: bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        valid(arena.nodes@, c),
        valid(arena.nodes@, q),
        valid(arena.nodes@, f),
    ensures
        out == spec::eq_approx(tab@, item_models(scope@), c@, q@, f@),
{
    match obj_of_exec(arena, scope, c, q) {
        None => false,
        Some(o) => {
            let o4 = arg_at(arena, &o, 4);
            is_atom(arena, &o4, &Sym::Eq) && (approximate_exec(arena, tab, scope, c, q)
                || approximate_exec(arena, tab, scope, c, f))
        },
    }
}

pub open spec fn about_of(tab: Temporal, scope: Seq<spec::Item>, m: (Term, spec::Ann)) -> Seq<
    (Term, Term),
> {
    match m.1 {
        spec::Ann::Interval(_, _, q, _, _) => if spec::eq_approx(tab, scope, m.0, q, q) {
            seq![(m.0, q)]
        } else {
            Seq::empty()
        },
        spec::Ann::Recurrence(_, f, q, _) => if spec::eq_approx(tab, scope, m.0, q, f) {
            seq![(m.0, q)]
        } else {
            Seq::empty()
        },
        spec::Ann::Window(_, f, l, _, _) => if spec::eq_approx(tab, scope, m.0, l, f) {
            seq![(m.0, l)]
        } else {
            Seq::empty()
        },
        spec::Ann::Frequency(_, _, w, _) => if spec::eq_approx(tab, scope, m.0, w, w) {
            seq![(m.0, w)]
        } else {
            Seq::empty()
        },
        _ => Seq::empty(),
    }
}

// spec::abouts: the approximate eq quantities of the annotations.
pub fn abouts_exec(arena: &ETermArena, tab: &ETemporal, scope: &Vec<I>, v: &Vec<(T, EAnn)>) -> (out:
    Vec<(T, T)>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        anns_valid(arena.nodes@, v@),
    ensures
        pairs_valid(arena.nodes@, out@),
        pairs_models(out@) == spec::abouts(tab@, item_models(scope@), anns_view(v@)),
{
    let ghost av = anns_view(v@);
    let ghost sm = item_models(scope@);
    let ghost f: spec_fn((Term, spec::Ann)) -> Seq<(Term, Term)> = |m: (Term, spec::Ann)|
        about_of(tab@, sm, m);
    proof {
        assert(spec::abouts(tab@, sm, av) =~= av.map_values(f).flatten()) by {
            assert(av.map_values(f) =~= av.map_values(
                |m: (Term, spec::Ann)|
                    match m.1 {
                        spec::Ann::Interval(_, _, q, _, _) => if spec::eq_approx(
                            tab@,
                            sm,
                            m.0,
                            q,
                            q,
                        ) {
                            seq![(m.0, q)]
                        } else {
                            Seq::empty()
                        },
                        spec::Ann::Recurrence(_, f, q, _) => if spec::eq_approx(
                            tab@,
                            sm,
                            m.0,
                            q,
                            f,
                        ) {
                            seq![(m.0, q)]
                        } else {
                            Seq::empty()
                        },
                        spec::Ann::Window(_, f, l, _, _) => if spec::eq_approx(
                            tab@,
                            sm,
                            m.0,
                            l,
                            f,
                        ) {
                            seq![(m.0, l)]
                        } else {
                            Seq::empty()
                        },
                        spec::Ann::Frequency(_, _, w, _) => if spec::eq_approx(
                            tab@,
                            sm,
                            m.0,
                            w,
                            w,
                        ) {
                            seq![(m.0, w)]
                        } else {
                            Seq::empty()
                        },
                        _ => Seq::empty(),
                    },
            ));
        }
        assert(av.take(0).map_values(f) =~= Seq::<Seq<(Term, Term)>>::empty());
        reveal_with_fuel(Seq::<_>::flatten, 1);
    }
    let mut out: Vec<(T, T)> = Vec::new();
    let mut i = 0usize;
    while i < v.len()
        invariant
            i <= v@.len(),
            arena_ok(arena),
            items_valid(arena.nodes@, scope@),
            anns_valid(arena.nodes@, v@),
            av == anns_view(v@),
            sm == item_models(scope@),
            av.len() == v@.len(),
            pairs_valid(arena.nodes@, out@),
            pairs_models(out@) == av.take(i as int).map_values(f).flatten(),
            f == (|m: (Term, spec::Ann)| about_of(tab@, sm, m)),
        decreases v@.len() - i,
    {
        let ghost before = out@;
        proof {
            assert(valid(arena.nodes@, &v@[i as int].0) && ann_valid(
                arena.nodes@,
                &v@[i as int].1,
            ));
        }
        let c = &v[i].0;
        match &v[i].1 {
            EAnn::Interval(_, _, q, _, _) => {
                if eq_approx_exec(arena, tab, scope, c, q, q) {
                    out.push((c.cp(), q.cp()));
                }
            },
            EAnn::Recurrence(_, fr, q, _) => {
                if eq_approx_exec(arena, tab, scope, c, q, fr) {
                    out.push((c.cp(), q.cp()));
                }
            },
            EAnn::Window(_, fr, l, _, _) => {
                if eq_approx_exec(arena, tab, scope, c, l, fr) {
                    out.push((c.cp(), l.cp()));
                }
            },
            EAnn::Frequency(_, _, w, _) => {
                if eq_approx_exec(arena, tab, scope, c, w, w) {
                    out.push((c.cp(), w.cp()));
                }
            },
            _ => {},
        }
        proof {
            crate::m7_annotate::flat_take(av, f, i as int);
            assert(av[i as int] == (v@[i as int].0@, ann_view(&v@[i as int].1)));
            assert(pairs_models(out@) =~= pairs_models(before) + f(av[i as int]));
            assert forall|j: int| 0 <= j < out@.len() implies valid(
                arena.nodes@,
                &(#[trigger] out@[j]).0,
            ) && valid(arena.nodes@, &out@[j].1) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(av.take(i as int) =~= av);
    }
    out
}

// spec::about_object: the object condition of q compared `about`.
pub fn about_object_exec(arena: &mut ETermArena, o: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, o),
        spec::is_comp(o@, "object"@, 6),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::about_object(o@),
{
    let ghost start = arena.nodes@;
    let a0 = arg_at(arena, o, 0);
    let a1 = arg_at(arena, o, 1);
    let a2 = arg_at(arena, o, 2);
    let a3 = arg_at(arena, o, 3);
    let a5 = arg_at(arena, o, 5);
    let ab = named(arena, &Sym::About);
    proof {
        crate::m6_term::prefix(start, arena.nodes@, &a0);
        crate::m6_term::prefix(start, arena.nodes@, &a1);
        crate::m6_term::prefix(start, arena.nodes@, &a2);
        crate::m6_term::prefix(start, arena.nodes@, &a3);
        crate::m6_term::prefix(start, arena.nodes@, &a5);
    }
    let mut ts = Vec::new();
    ts.push(a0);
    ts.push(a1);
    ts.push(a2);
    ts.push(a3);
    ts.push(ab);
    ts.push(a5);
    proof {
        assert(models(ts@) =~= seq![
            ckc_spec::replay::arg(o@, 0),
            ckc_spec::replay::arg(o@, 1),
            ckc_spec::replay::arg(o@, 2),
            ckc_spec::replay::arg(o@, 3),
            Term::Atom(symbol(&Sym::About)),
            ckc_spec::replay::arg(o@, 5),
        ]);
    }
    c(arena, &Sym::Object, &ts)
}

} // verus!
