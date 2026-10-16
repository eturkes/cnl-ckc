// s1/s2 D3: document-only grade, preference + qualifier pass after temporal annotation.
use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_model::*;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_symbols::{Sym, symbol_bytes};
use crate::m6_term::*;
use crate::m7_annotate::{arg_at, assoc_exec, atom_bytes};
#[cfg(verus_keep_ghost)]
use crate::m7_annotate::{opt_model, prepend};
use crate::m7_q12::{EEnv, decl_exec, plain_exec};
#[cfg(verus_keep_ghost)]
use crate::m7_q12::{env_view, envo_view};
use crate::m7_temporal::{EVocabulary, seq_in};
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

pub fn is_pred(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == spec::is_pred(t@),
{
    is_comp(arena, t, &Sym::Predicate, 3) || is_comp(arena, t, &Sym::Predicate, 4) || is_comp(
        arena,
        t,
        &Sym::Predicate,
        5,
    )
}

pub fn event_pred(arena: &ETermArena, items: &Vec<I>, ctx: &T, e: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, e),
    ensures
        out matches Some(p) ==> valid(arena.nodes@, &p) && spec::is_pred(p@),
        opt_model(out) == spec::event_pred(item_models(items@), ctx@, e@),
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
            spec::event_pred(ms, ctx@, e@) == spec::event_pred(ms.skip(i as int), ctx@, e@),
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
                if crate::m6_term::equal(arena, c, ctx) && is_pred(arena, inner) {
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

pub fn preferred_prop(
    arena: &ETermArena,
    tab: &EVocabulary,
    items: &Vec<I>,
    ctx: &T,
    p: &T,
) -> (out: bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, p),
    ensures
        out == spec::preferred_prop(tab@, item_models(items@), ctx@, p@),
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
            valid(arena.nodes@, p),
            ms == item_models(items@),
            ms.len() == items@.len(),
            spec::preferred_prop(tab@, ms, ctx@, p@) == spec::preferred_prop(
                tab@,
                ms.skip(i as int),
                ctx@,
                p@,
            ),
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
                if crate::m6_term::equal(arena, c, ctx) && is_comp(
                    arena,
                    inner,
                    &Sym::Property,
                    3,
                ) {
                    let a0 = arg_at(arena, inner, 0);
                    let a1 = arg_at(arena, inner, 1);
                    let a2 = arg_at(arena, inner, 2);
                    if crate::m6_term::equal(arena, &a0, p) && is_atom(arena, &a2, &Sym::Pos)
                        && seq_in(&tab.preferences, &atom_bytes(arena, &a1)) {
                        return true;
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
    false
}

pub fn alternatives_of(
    arena: &ETermArena,
    tab: &EVocabulary,
    items: &Vec<I>,
    ctx: &T,
    e: &T,
) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, e),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::alternatives_of(tab@, item_models(items@), ctx@, e@),
{
    let ghost ms = item_models(items@);
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
        assert(models(out@) + spec::alternatives_of(tab@, ms, ctx@, e@) =~= spec::alternatives_of(
            tab@,
            ms,
            ctx@,
            e@,
        ));
    }
    while i < items.len()
        invariant
            i <= items@.len(),
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid(arena.nodes@, ctx),
            valid(arena.nodes@, e),
            valid_all(arena.nodes@, out@),
            ms == item_models(items@),
            ms.len() == items@.len(),
            spec::alternatives_of(tab@, ms, ctx@, e@) == models(out@) + spec::alternatives_of(
                tab@,
                ms.skip(i as int),
                ctx@,
                e@,
            ),
        decreases items@.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms[i as int] == items@[i as int]@);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
        }
        let ghost before = out@;
        match &items[i].kind {
            Kind::Anch(c, inner) => {
                if crate::m6_term::equal(arena, c, ctx) && is_comp(
                    arena,
                    inner,
                    &Sym::ModifierPp,
                    3,
                ) {
                    let a0 = arg_at(arena, inner, 0);
                    let a1 = arg_at(arena, inner, 1);
                    if crate::m6_term::equal(arena, &a0, e) && seq_in(
                        &tab.alternatives,
                        &atom_bytes(arena, &a1),
                    ) {
                        out.push(arg_at(arena, inner, 2));
                    }
                }
            },
            _ => {},
        }
        proof {
            assert(models(out@) =~= models(before) + models(out@).skip(before.len() as int));
            assert forall|j: int| 0 <= j < out@.len() implies #[trigger] valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(ms.skip(i as int).len() == 0);
        assert(models(out@) + Seq::<Term>::empty() =~= models(out@));
    }
    out
}

pub open spec fn pref_error(s: Sym) -> Term {
    Term::Comp(symbol(&Sym::PreferenceShape), seq![Term::Atom(symbol(&s))])
}

pub fn shape(arena: &mut ETermArena, kind: &Sym, why: &Sym) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(symbol(kind), seq![Term::Atom(symbol(why))]),
{
    let ghost start = arena.nodes@;
    let w = named(arena, why);
    let ghost mid = arena.nodes@;
    let out = c1(arena, kind, &w);
    proof {
        crate::k2_load::prefix_chain(start, mid, arena.nodes@);
    }
    out
}

pub open spec fn preference_result(r: Result<Option<(T, T)>, Sym>) -> Result<
    Option<(Term, Term)>,
    Term,
> {
    match r {
        Ok(Some((p, a))) => Ok(Some((p@, a@))),
        Ok(None) => Ok(None),
        Err(s) => Err(pref_error(s)),
    }
}

pub fn preference_of(
    arena: &ETermArena,
    env: &EEnv,
    items: &Vec<I>,
    ctx: &T,
    pred: &T,
    none: &T,
) -> (out: Result<Option<(T, T)>, Sym>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, pred),
        spec::is_pred(pred@),
        valid(arena.nodes@, none),
        none@ == ckc_spec::replay::atom("none"@),
    ensures
        out matches Ok(Some((p, a))) ==> valid(arena.nodes@, &p) && valid(arena.nodes@, &a),
        preference_result(out) == spec::preference_of(
            env_view(env),
            item_models(items@),
            ctx@,
            pred@,
        ),
{
    let e = arg_at(arena, pred, 0);
    let alts = alternatives_of(arena, &env.tab, items, ctx, &e);
    let copula = if is_comp(arena, pred, &Sym::Predicate, 4) {
        let verb = arg_at(arena, pred, 1);
        let p = arg_at(arena, pred, 3);
        is_atom(arena, &verb, &Sym::Be) && preferred_prop(arena, &env.tab, items, ctx, &p)
    } else {
        false
    };
    if alts.len() > 1 {
        return Err(Sym::AlternativeCount);
    }
    if !copula && alts.len() == 0 {
        return Ok(None);
    }
    if !copula && is_comp(arena, pred, &Sym::Predicate, 3) {
        return Err(Sym::NoPreferred);
    }
    let p = arg_at(
        arena,
        pred,
        if copula {
            2
        } else {
            3
        },
    );
    let a = if alts.len() == 1 {
        proof {
            assert(models(alts@)[0] == alts@[0]@);
        }
        alts[0].cp()
    } else {
        none.cp()
    };
    if !plain_exec(arena, env, &p) || (alts.len() == 1 && !plain_exec(arena, env, &a)) {
        Err(Sym::Endpoint)
    } else {
        Ok(Some((p, a)))
    }
}

pub open spec fn grade_result(r: Option<(Vec<u8>, Vec<u8>)>) -> Option<(Seq<u8>, Seq<u8>)> {
    match r {
        Some((d, v)) => Some((d@, v@)),
        None => None,
    }
}

pub fn grade_of(arena: &ETermArena, env: &EEnv, ctx: &T, pred: &T) -> (out: Option<
    (Vec<u8>, Vec<u8>),
>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, pred),
    ensures
        grade_result(out) == spec::grade_of(env_view(env), ctx@, pred@),
{
    if !is_atom(arena, ctx, &Sym::Actual) || !is_comp(arena, pred, &Sym::Predicate, 4) {
        return None;
    }
    let verb = arg_at(arena, pred, 1);
    let rec = arg_at(arena, pred, 2);
    if !is_atom(arena, &verb, &Sym::Be) || !plain_exec(arena, env, &rec) {
        return None;
    }
    let label = arg_at(arena, pred, 3);
    if !is_var(arena, &label) {
        return None;
    }
    let k = var_index(arena, &label);
    match decl_exec(&env.decls, k) {
        None => None,
        Some(i) => {
            let noun = &env.decls[i].noun;
            match assoc_exec(&env.tab.strengths, noun) {
                Some(v) => Some((symbol_bytes(&Sym::Strength), v)),
                None => match assoc_exec(&env.tab.evidences, noun) {
                    Some(v) => Some((symbol_bytes(&Sym::Evidence), v)),
                    None => None,
                },
            }
        },
    }
}

pub fn preference_item(arena: &mut ETermArena, ctx: &T, e: &T, pa: &(T, T)) -> (out: I)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, ctx),
        valid(old(arena).nodes@, e),
        valid(old(arena).nodes@, &pa.0),
        valid(old(arena).nodes@, &pa.1),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        item_valid(final(arena).nodes@, &out),
        out@ == spec::preference_item(ctx@, e@, (pa.0@, pa.1@)),
{
    let ghost start = arena.nodes@;
    let inner = c3(arena, &Sym::DollarGuidelinePreference, e, &pa.0, &pa.1);
    proof {
        prefix(start, arena.nodes@, ctx);
    }
    anch(arena, ctx, &inner)
}

pub fn grade_item(
    arena: &mut ETermArena,
    ctx: &T,
    pred: &T,
    dim: &Vec<u8>,
    value: &Vec<u8>,
) -> (out: I)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, ctx),
        valid(old(arena).nodes@, pred),
        spec::is_comp(pred@, "predicate"@, 4),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        item_valid(final(arena).nodes@, &out),
        out@ == spec::Item::Anch(
            ctx@,
            Term::Comp(
                symbol(&Sym::DollarGuidelineGrade),
                seq![
                    ckc_spec::replay::arg(pred@, 2),
                    Term::Atom(dim@),
                    Term::Atom(value@),
                    ckc_spec::replay::arg(pred@, 3),
                ],
            ),
        ),
{
    let rec = arg_at(arena, pred, 2);
    let label = arg_at(arena, pred, 3);
    let ghost start = arena.nodes@;
    let d = atom(arena, dim);
    let ghost n1 = arena.nodes@;
    let v = atom(arena, value);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
        prefix(n1, n2, &d);
        prefix(start, n2, &rec);
        prefix(start, n2, &label);
    }
    let inner = crate::m6_expand::lit4(arena, &Sym::DollarGuidelineGrade, &rec, &d, &v, &label);
    proof {
        crate::k2_load::prefix_chain(start, n2, arena.nodes@);
        prefix(start, arena.nodes@, ctx);
    }
    anch(arena, ctx, &inner)
}

pub fn qualifier_item(arena: &mut ETermArena, ctx: &T, e: &T, k: &Vec<u8>, word: &T) -> (out: I)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, ctx),
        valid(old(arena).nodes@, e),
        valid(old(arena).nodes@, word),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        item_valid(final(arena).nodes@, &out),
        out@ == spec::qualifier_item(ctx@, e@, k@, word@),
{
    let ghost start = arena.nodes@;
    let class = atom(arena, k);
    let ghost middle = arena.nodes@;
    proof {
        prefix(start, middle, e);
        prefix(start, middle, word);
    }
    let inner = c3(arena, &Sym::DollarGuidelineQualifier, e, &class, word);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
        prefix(start, arena.nodes@, ctx);
    }
    anch(arena, ctx, &inner)
}

pub fn attenuating(k: &Vec<u8>) -> (out: bool)
    ensures
        out == spec::attenuating(k@),
{
    has_name(k, &Sym::Usuality) || has_name(k, &Sym::Ideality) || has_name(k, &Sym::Possibility)
        || has_name(k, &Sym::Applicability)
}

pub open spec fn edge_view(e: Option<(T, Vec<u8>)>) -> Option<(Term, Seq<u8>)> {
    match e {
        Some((outer, op)) => Some((outer@, op@)),
        None => None,
    }
}

pub fn edge_of(arena: &ETermArena, items: &Vec<I>, ctx: &T) -> (out: Option<(T, Vec<u8>)>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
    ensures
        out matches Some((outer, _)) ==> valid(arena.nodes@, &outer),
        edge_view(out) == spec::edge_of(item_models(items@), ctx@),
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
            ms == item_models(items@),
            ms.len() == items@.len(),
            spec::edge_of(ms, ctx@) == spec::edge_of(ms.skip(i as int), ctx@),
        decreases items@.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms[i as int] == items@[i as int]@);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
        }
        if let Kind::Op { outer, inner, op, .. } = &items[i].kind {
            if crate::m6_term::equal(arena, inner, ctx) {
                return Some((outer.cp(), crate::k4_bytes::copy(op)));
            }
        }
        i += 1;
    }
    proof {
        assert(ms.skip(i as int).len() == 0);
    }
    None
}

pub fn under_modal(arena: &ETermArena, scope: &Vec<I>, ctx: &T, fuel: usize) -> (out: bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        valid(arena.nodes@, ctx),
    ensures
        out == spec::under_modal(item_models(scope@), ctx@, fuel as nat),
    decreases fuel,
{
    if fuel == 0 || is_atom(arena, ctx, &Sym::Actual) {
        false
    } else {
        match edge_of(arena, scope, ctx) {
            Some((outer, op)) => crate::m6_drs::modal(&op) || under_modal(
                arena,
                scope,
                &outer,
                fuel - 1,
            ),
            None => false,
        }
    }
}

pub fn property_only(arena: &ETermArena, env: &EEnv, v: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, v),
    ensures
        out == spec::property_only(env_view(env), v@),
{
    if is_var(arena, v) {
        decl_exec(&env.decls, var_index(arena, v)).is_none()
    } else {
        false
    }
}

pub fn copula_qualifiers(
    arena: &mut ETermArena,
    tab: &EVocabulary,
    items: &Vec<I>,
    ctx: &T,
    e: &T,
    p: &T,
) -> (out: Vec<I>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
        valid(old(arena).nodes@, ctx),
        valid(old(arena).nodes@, e),
        valid(old(arena).nodes@, p),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        items_valid(final(arena).nodes@, out@),
        item_models(out@) == spec::copula_qualifiers(tab@, item_models(items@), ctx@, e@, p@),
{
    let ghost start = arena.nodes@;
    let ghost ms = item_models(items@);
    let mut out: Vec<I> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
        reveal_with_fuel(items_valid, 1);
        assert(item_models(out@) =~= Seq::<spec::Item>::empty());
        assert(item_models(out@) + spec::copula_qualifiers(tab@, ms, ctx@, e@, p@)
            =~= spec::copula_qualifiers(tab@, ms, ctx@, e@, p@));
    }
    while i < items.len()
        invariant
            i <= items@.len(),
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            items_valid(arena.nodes@, items@),
            items_valid(arena.nodes@, out@),
            valid(arena.nodes@, ctx),
            valid(arena.nodes@, e),
            valid(arena.nodes@, p),
            ms == item_models(items@),
            ms.len() == items@.len(),
            spec::copula_qualifiers(tab@, ms, ctx@, e@, p@) == item_models(out@)
                + spec::copula_qualifiers(tab@, ms.skip(i as int), ctx@, e@, p@),
        decreases items@.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms[i as int] == items@[i as int]@);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
        }
        let ghost before = out@;
        let ghost before_nodes = arena.nodes@;
        if let Kind::Anch(c, inner) = &items[i].kind {
            if crate::m6_term::equal(arena, c, ctx) && is_comp(arena, inner, &Sym::Property, 3) {
                let a0 = arg_at(arena, inner, 0);
                let a1 = arg_at(arena, inner, 1);
                let a2 = arg_at(arena, inner, 2);
                if crate::m6_term::equal(arena, &a0, p) && is_atom(arena, &a2, &Sym::Pos) {
                    if let Some(k) = assoc_exec(&tab.qualifiers, &atom_bytes(arena, &a1)) {
                        let qi = qualifier_item(arena, ctx, e, &k, &a1);
                        proof {
                            items_prefix(before_nodes, arena.nodes@, out@);
                            reveal_with_fuel(items_valid, 2);
                            assert(items_valid(arena.nodes@, seq![qi]));
                            items_concat(arena.nodes@, out@, seq![qi]);
                        }
                        out.push(qi);
                        proof {
                            assert(out@ =~= before + seq![qi]);
                        }
                    }
                }
            }
        }
        proof {
            items_prefix(before_nodes, arena.nodes@, items@);
            prefix(before_nodes, arena.nodes@, ctx);
            prefix(before_nodes, arena.nodes@, e);
            prefix(before_nodes, arena.nodes@, p);
            crate::k2_load::prefix_chain(start, before_nodes, arena.nodes@);
            assert(item_models(out@) =~= item_models(before) + item_models(out@).skip(
                before.len() as int,
            ));
        }
        i += 1;
    }
    proof {
        assert(ms.skip(i as int).len() == 0);
        assert(item_models(out@) + Seq::<spec::Item>::empty() =~= item_models(out@));
    }
    out
}

pub open spec fn strength_result(r: Result<Vec<I>, (Sym, Sym)>) -> Result<Seq<spec::Item>, Term> {
    match r {
        Ok(v) => Ok(item_models(v@)),
        Err((kind, why)) => Err(Term::Comp(symbol(&kind), seq![Term::Atom(symbol(&why))])),
    }
}

pub fn strength_item(
    arena: &mut ETermArena,
    env: &EEnv,
    w: &W,
    scope: &Vec<I>,
    it: &I,
    none: &T,
) -> (out: Result<Vec<I>, (Sym, Sym)>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, scope@),
        item_valid(old(arena).nodes@, it),
        valid(old(arena).nodes@, none),
        none@ == ckc_spec::replay::atom("none"@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(v) ==> items_valid(final(arena).nodes@, v@),
        strength_result(out) == spec::strength_item(env_view(env), w@, item_models(scope@), it@),
    decreases it, 0int,
{
    let ghost start = arena.nodes@;
    proof {
        reveal(item_valid);
        reveal_with_fuel(spec::strength_item, 1);
    }
    match &it.kind {
        Kind::Anch(ctx, inner) => {
            let first = copy_item(arena, it);
            let mut v = Vec::new();
            v.push(first);
            proof {
                reveal_with_fuel(items_valid, 2);
                assert(item_models(v@) =~= seq![it@]);
            }
            if is_pred(arena, inner) {
                let g = if matches!(w, W::Root) {
                    grade_of(arena, env, ctx, inner)
                } else {
                    None
                };
                let ghost gs = match grade_result(g) {
                    Some((dim, value)) => seq![
                        spec::Item::Anch(
                            ctx@,
                            Term::Comp(
                                symbol(&Sym::DollarGuidelineGrade),
                                seq![
                                    ckc_spec::replay::arg(inner@, 2),
                                    Term::Atom(dim),
                                    Term::Atom(value),
                                    ckc_spec::replay::arg(inner@, 3),
                                ],
                            ),
                        ),
                    ],
                    None => Seq::empty(),
                };
                let pa = match preference_of(arena, env, scope, ctx, inner, none) {
                    Err(s) => return Err((Sym::PreferenceShape, s)),
                    Ok(pa) => pa,
                };
                let e = arg_at(arena, inner, 0);
                match g {
                    Some((dim, value)) => {
                        let gi = grade_item(arena, ctx, inner, &dim, &value);
                        proof {
                            items_prefix(start, arena.nodes@, v@);
                        }
                        v.push(gi);
                        proof {
                            reveal_with_fuel(items_valid, 3);
                            assert(item_models(v@) =~= seq![it@, gi@]);
                        }
                    },
                    None => {},
                }
                proof {
                    assert(item_models(v@) =~= seq![it@] + gs);
                    prefix(start, arena.nodes@, inner);
                    prefix(start, arena.nodes@, ctx);
                    prefix(start, arena.nodes@, &e);
                    items_prefix(start, arena.nodes@, scope@);
                }
                let ghost graded = arena.nodes@;
                let mut qs = if is_comp(arena, inner, &Sym::Predicate, 4) {
                    let verb = arg_at(arena, inner, 1);
                    let p = arg_at(arena, inner, 3);
                    if is_atom(arena, &verb, &Sym::Be) && property_only(arena, env, &p) {
                        copula_qualifiers(arena, &env.tab, scope, ctx, &e, &p)
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                };
                let ghost qm = if spec::is_comp(inner@, "predicate"@, 4) && ckc_spec::replay::arg(
                    inner@,
                    1,
                ) == ckc_spec::replay::atom("be"@) && spec::property_only(
                    env_view(env),
                    ckc_spec::replay::arg(inner@, 3),
                ) {
                    spec::copula_qualifiers(
                        env.tab@,
                        item_models(scope@),
                        ctx@,
                        e@,
                        ckc_spec::replay::arg(inner@, 3),
                    )
                } else {
                    Seq::empty()
                };
                proof {
                    reveal_with_fuel(items_valid, 1);
                    assert(item_models(qs@) =~= qm);
                    items_prefix(graded, arena.nodes@, v@);
                    items_concat(arena.nodes@, v@, qs@);
                    crate::k2_load::prefix_chain(start, graded, arena.nodes@);
                }
                v.append(&mut qs);
                proof {
                    assert(item_models(v@) =~= seq![it@] + gs + qm);
                }
                let ghost mid = arena.nodes@;
                match pa {
                    Some(pa) => {
                        proof {
                            prefix(start, mid, ctx);
                            prefix(start, mid, &e);
                            prefix(start, mid, &pa.0);
                            prefix(start, mid, &pa.1);
                        }
                        if is_atom(arena, &pa.1, &Sym::None) {
                            let pi = preference_item(arena, ctx, &e, &pa);
                            proof {
                                items_prefix(mid, arena.nodes@, v@);
                                crate::k2_load::prefix_chain(start, mid, arena.nodes@);
                            }
                            let ghost before = v@;
                            proof {
                                reveal_with_fuel(items_valid, 2);
                                items_concat(arena.nodes@, v@, seq![pi]);
                            }
                            v.push(pi);
                            proof {
                                assert(v@ =~= before + seq![pi]);
                                assert(item_models(v@) =~= item_models(before) + seq![pi@]);
                            }
                        }
                    },
                    None => {},
                }
                Ok(v)
            } else if is_comp(arena, inner, &Sym::ModifierPp, 3) {
                proof {
                    reveal_strlit("modifier_pp");
                    reveal_strlit("modifier_adv");
                    reveal(ckc_spec::v1text::ascii);
                    assert(spec::is_comp(inner@, "modifier_pp"@, 3));
                    assert(ckc_spec::v1text::ascii("modifier_pp"@).len() == 11);
                    assert(ckc_spec::v1text::ascii("modifier_adv"@).len() == 12);
                    reveal(spec::is_comp);
                    assert(!spec::is_comp(inner@, "modifier_adv"@, 3));
                }
                let prep = arg_at(arena, inner, 1);
                if seq_in(&env.tab.alternatives, &atom_bytes(arena, &prep)) {
                    let e = arg_at(arena, inner, 0);
                    if let Some(pred) = event_pred(arena, scope, ctx, &e) {
                        if let Ok(Some(pa)) = preference_of(arena, env, scope, ctx, &pred, none) {
                            let pi = preference_item(arena, ctx, &e, &pa);
                            proof {
                                items_prefix(start, arena.nodes@, v@);
                            }
                            v.push(pi);
                            proof {
                                reveal_with_fuel(items_valid, 3);
                                assert(item_models(v@) =~= seq![it@, pi@]);
                            }
                        }
                    }
                }
                Ok(v)
            } else if is_comp(arena, inner, &Sym::ModifierAdv, 3) {
                let word = arg_at(arena, inner, 1);
                match assoc_exec(&env.tab.qualifiers, &atom_bytes(arena, &word)) {
                    None => Ok(v),
                    Some(k) => {
                        let degree = arg_at(arena, inner, 2);
                        if !is_atom(arena, &degree, &Sym::Pos) {
                            return Err((Sym::QualifierShape, Sym::Degree));
                        }
                        if attenuating(&k) && !matches!(w, W::Antecedent) && !under_modal(
                            arena,
                            scope,
                            ctx,
                            scope.len(),
                        ) {
                            return Err((Sym::QualifierShape, Sym::Asserted));
                        }
                        let e = arg_at(arena, inner, 0);
                        let qi = qualifier_item(arena, ctx, &e, &k, &word);
                        let mut out = Vec::new();
                        out.push(qi);
                        proof {
                            reveal_with_fuel(items_valid, 2);
                            assert(item_models(out@) =~= seq![qi@]);
                        }
                        Ok(out)
                    },
                }
            } else {
                Ok(v)
            }
        },
        Kind::Naf { dom, payload } => match strength(arena, env, &W::Antecedent, payload, none) {
            Err(s) => Err(s),
            Ok(p) => {
                proof {
                    prefix_all(start, arena.nodes@, dom@);
                }
                let n = naf(arena, copy(dom), p);
                let mut v = Vec::new();
                v.push(n);
                proof {
                    reveal_with_fuel(items_valid, 2);
                    assert(item_models(v@) =~= seq![n@]);
                }
                Ok(v)
            },
        },
        Kind::Op { .. } => {
            let one = copy_item(arena, it);
            let mut v = Vec::new();
            v.push(one);
            proof {
                reveal_with_fuel(items_valid, 2);
                assert(item_models(v@) =~= seq![it@]);
            }
            Ok(v)
        },
    }
}

pub fn strength_items(
    arena: &mut ETermArena,
    env: &EEnv,
    w: &W,
    scope: &Vec<I>,
    items: &Vec<I>,
    none: &T,
) -> (out: Result<Vec<I>, (Sym, Sym)>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, scope@),
        items_valid(old(arena).nodes@, items@),
        valid(old(arena).nodes@, none),
        none@ == ckc_spec::replay::atom("none"@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(v) ==> items_valid(final(arena).nodes@, v@),
        strength_result(out) == spec::strength_items(
            env_view(env),
            w@,
            item_models(scope@),
            item_models(items@),
        ),
    decreases items@, 1int,
{
    let ghost start = arena.nodes@;
    let ghost ms = item_models(items@);
    let ghost sm = item_models(scope@);
    let mut out: Vec<I> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
        reveal_with_fuel(items_valid, 1);
        assert(item_models(out@) =~= Seq::<spec::Item>::empty());
        match spec::strength_items(env_view(env), w@, sm, ms) {
            Ok(t) => assert(Seq::<spec::Item>::empty() + t =~= t),
            Err(_) => {},
        }
    }
    while i < items.len()
        invariant
            i <= items@.len(),
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            items_valid(arena.nodes@, scope@),
            items_valid(arena.nodes@, items@),
            items_valid(arena.nodes@, out@),
            valid(arena.nodes@, none),
            none@ == ckc_spec::replay::atom("none"@),
            ms == item_models(items@),
            sm == item_models(scope@),
            ms.len() == items@.len(),
            spec::strength_items(env_view(env), w@, sm, ms) == prepend(
                item_models(out@),
                spec::strength_items(env_view(env), w@, sm, ms.skip(i as int)),
            ),
        decreases items@.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms[i as int] == items@[i as int]@);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
        }
        let ghost before_nodes = arena.nodes@;
        let ghost before = out@;
        let ghost rest = spec::strength_items(env_view(env), w@, sm, ms.skip(i + 1));
        proof {
            reveal_with_fuel(spec::strength_items, 1);
            assert(spec::strength_items(env_view(env), w@, sm, ms.skip(i as int)) == match (
                spec::strength_item(env_view(env), w@, sm, ms[i as int]),
                rest,
            ) {
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
                (Ok(h), Ok(t)) => Ok(h + t),
            });
        }
        let r = strength_item(arena, env, w, scope, &items[i], none);
        proof {
            items_prefix(before_nodes, arena.nodes@, scope@);
            items_prefix(before_nodes, arena.nodes@, items@);
            items_prefix(before_nodes, arena.nodes@, out@);
            prefix(before_nodes, arena.nodes@, none);
            crate::k2_load::prefix_chain(start, before_nodes, arena.nodes@);
        }
        match r {
            Err(s) => return Err(s),
            Ok(mut v) => {
                let ghost h = item_models(v@);
                proof {
                    items_concat(arena.nodes@, out@, v@);
                }
                out.append(&mut v);
                proof {
                    assert(item_models(out@) =~= item_models(before) + h);
                    match rest {
                        Ok(t) => assert((item_models(before) + h) + t =~= item_models(before) + (h
                            + t)),
                        Err(_) => {},
                    }
                }
            },
        }
        i += 1;
    }
    proof {
        assert(ms.skip(i as int).len() == 0);
        assert(item_models(out@) + Seq::<spec::Item>::empty() =~= item_models(out@));
    }
    Ok(out)
}

pub fn strength(arena: &mut ETermArena, env: &EEnv, w: &W, items: &Vec<I>, none: &T) -> (out:
    Result<Vec<I>, (Sym, Sym)>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
        valid(old(arena).nodes@, none),
        none@ == ckc_spec::replay::atom("none"@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(v) ==> items_valid(final(arena).nodes@, v@),
        strength_result(out) == spec::strength(env_view(env), w@, item_models(items@)),
    decreases items@, 2int,
{
    strength_items(arena, env, w, items, items, none)
}

pub fn flatten_doc(
    arena: &mut ETermArena,
    tab: &Option<EEnv>,
    l: &T,
    w: &W,
    env: &crate::m6_flat::Env,
    deps: &T,
    outer: &T,
    encl: &E,
    n: usize,
) -> (out: Result<Flat, T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, l),
        valid(old(arena).nodes@, deps),
        valid(old(arena).nodes@, outer),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        crate::m6_flat::flat_valid_result(final(arena).nodes@, &out),
        flat_result(out) == spec::flatten_doc(
            envo_view(tab),
            l@,
            w@,
            env.s as nat,
            env.docid@,
            deps@,
            outer@,
            encl@,
            n as nat,
            env.base as nat,
        ),
{
    let ghost start = arena.nodes@;
    let f = match crate::m7_annotate::flatten_ann_exec(
        arena,
        tab,
        l,
        w,
        env,
        deps,
        outer,
        encl,
        n,
    ) {
        Err(e) => return Err(e),
        Ok(f) => f,
    };
    match tab {
        None => Ok(f),
        Some(t) => {
            let ghost before = arena.nodes@;
            let none = named(arena, &Sym::None);
            let ghost middle = arena.nodes@;
            proof {
                crate::k2_load::prefix_chain(start, before, middle);
                items_prefix(before, middle, f.items@);
            }
            let r = strength(arena, t, w, &f.items, &none);
            let ghost after = arena.nodes@;
            proof {
                crate::k2_load::prefix_chain(start, middle, after);
            }
            match r {
                Ok(items) => Ok(Flat { items, n: f.n }),
                Err((kind, why)) => {
                    let e = shape(arena, &kind, &why);
                    proof {
                        crate::k2_load::prefix_chain(start, after, arena.nodes@);
                    }
                    Err(e)
                },
            }
        },
    }
}

} // verus!
