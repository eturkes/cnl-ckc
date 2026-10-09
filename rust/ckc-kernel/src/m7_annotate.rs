// m7t D4/D5: the v2 annotate pass — exec mirror of ckc_spec::emit::annotate.
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ENodeKind, ETermArena};
use crate::m6_model::*;
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
use crate::m7_temporal::ETemporal;
#[cfg(verus_keep_ghost)]
use crate::m7_temporal::pairs_view;
use crate::m7_v3::EEnv;
#[cfg(verus_keep_ghost)]
use crate::m7_v3::{env_view, envo_view, ttab_view};
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::temporal::*;
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

pub open spec fn tab_view(t: &Option<EEnv>) -> Option<spec::Env> {
    envo_view(t)
}

pub open spec fn opt_bytes(o: Option<Vec<u8>>) -> Option<Seq<u8>> {
    match o {
        Some(v) => Some(v@),
        None => None,
    }
}

pub open spec fn opt_model(o: Option<T>) -> Option<Term> {
    match o {
        Some(t) => Some(t@),
        None => None,
    }
}

pub open spec fn atom_name(t: Term) -> Seq<u8> {
    match t {
        Term::Atom(a) => a,
        _ => Seq::empty(),
    }
}

pub fn assoc_exec(rows: &Vec<(Vec<u8>, Vec<u8>)>, k: &[u8]) -> (out: Option<Vec<u8>>)
    ensures
        opt_bytes(out) == spec::assoc(pairs_view(rows@), k@),
{
    let ghost pv = pairs_view(rows@);
    let mut i = 0usize;
    proof {
        assert(pv.skip(0) =~= pv);
    }
    while i < rows.len()
        invariant
            i <= rows@.len(),
            pv == pairs_view(rows@),
            pv.len() == rows@.len(),
            spec::assoc(pv, k@) == spec::assoc(pv.skip(i as int), k@),
        decreases rows@.len() - i,
    {
        proof {
            assert(pv.skip(i as int)[0] == pv[i as int]);
            assert(pv.skip(i as int).drop_first() =~= pv.skip(i + 1));
        }
        if crate::k4_bytes::eq(&rows[i].0, k) {
            return Some(crate::k4_bytes::copy(&rows[i].1));
        }
        i += 1;
    }
    proof {
        assert(pv.skip(i as int).len() == 0);
    }
    None
}

pub fn atom_bytes(arena: &ETermArena, t: &T) -> (out: Vec<u8>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out@ == atom_name(t@),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, t.root as int));
        reveal(crate::k2_term::node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Atom { name } => name.clone(),
        _ => Vec::new(),
    }
}

// The n-th argument of a compound with at least n + 1 arguments.
pub fn arg_at(arena: &ETermArena, t: &T, n: usize) -> (out: T)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
        n < ckc_spec::engine::args_of(t@).len(),
    ensures
        valid(arena.nodes@, &out),
        out@ == ckc_spec::replay::arg(t@, n as int),
{
    let a = args(arena, t);
    proof {
        assert(models(a@)[n as int] == a@[n as int]@);
    }
    a[n].cp()
}

pub fn int_cmp(arena: &ETermArena, t: &T, one: bool) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == if one {
            t@ == Term::Int(1)
        } else {
            match t@ {
                Term::Int(n) => n >= 1,
                _ => false,
            }
        },
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, t.root as int));
        reveal(crate::k2_term::node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Int { magnitude, negative, .. } => {
            let zero = magnitude.len() == 1 && magnitude[0] == 0x30;
            let unit = magnitude.len() == 1 && magnitude[0] == 0x31;
            proof {
                if let Term::Int(n) = t@ {
                    let m: nat = if n < 0 {
                        (-n) as nat
                    } else {
                        n as nat
                    };
                    reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
                    reveal(ckc_spec::v1text::digit_byte);
                    assert(ckc_spec::v1text::udec_bytes(0) =~= seq![0x30u8]);
                    assert(ckc_spec::v1text::udec_bytes(1) =~= seq![0x31u8]);
                    if zero {
                        assert(magnitude@ =~= ckc_spec::v1text::udec_bytes(0));
                        crate::v1_term_impl::udec_bytes_injective(m, 0);
                    }
                    if unit {
                        assert(magnitude@ =~= ckc_spec::v1text::udec_bytes(1));
                        crate::v1_term_impl::udec_bytes_injective(m, 1);
                    }
                    if m == 0 {
                        assert(magnitude@ == ckc_spec::v1text::udec_bytes(0));
                    }
                    if m == 1 {
                        assert(magnitude@ == ckc_spec::v1text::udec_bytes(1));
                    }
                }
            }
            if one {
                !*negative && unit
            } else {
                !*negative && !zero
            }
        },
        _ => false,
    }
}

pub fn obj_of_exec(arena: &ETermArena, items: &Vec<I>, ctx: &T, v: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, v),
    ensures
        out matches Some(o) ==> valid(arena.nodes@, &o) && spec::is_comp(o@, "object"@, 6),
        opt_model(out) == spec::obj_of(item_models(items@), ctx@, v@),
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
            valid(arena.nodes@, v),
            ms == item_models(items@),
            ms.len() == items@.len(),
            spec::obj_of(ms, ctx@, v@) == spec::obj_of(ms.skip(i as int), ctx@, v@),
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
                if crate::m6_term::equal(arena, c, ctx) && is_comp(arena, inner, &Sym::Object, 6) {
                    let a0 = arg_at(arena, inner, 0);
                    if crate::m6_term::equal(arena, &a0, v) {
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

pub fn of_links_exec(arena: &ETermArena, items: &Vec<I>, ctx: &T, v: &T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, v),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::of_links(item_models(items@), ctx@, v@),
{
    let ghost ms = item_models(items@);
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
        assert(models(out@) + spec::of_links(ms, ctx@, v@) =~= spec::of_links(ms, ctx@, v@));
    }
    while i < items.len()
        invariant
            i <= items@.len(),
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid(arena.nodes@, ctx),
            valid(arena.nodes@, v),
            ms == item_models(items@),
            ms.len() == items@.len(),
            valid_all(arena.nodes@, out@),
            spec::of_links(ms, ctx@, v@) == models(out@) + spec::of_links(
                ms.skip(i as int),
                ctx@,
                v@,
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
                    &Sym::Relation,
                    3,
                ) {
                    let a0 = arg_at(arena, inner, 0);
                    let a1 = arg_at(arena, inner, 1);
                    if crate::m6_term::equal(arena, &a0, v) && is_atom(arena, &a1, &Sym::Of) {
                        out.push(arg_at(arena, inner, 2));
                    }
                }
            },
            _ => {},
        }
        proof {
            assert(models(out@) =~= models(before) + (models(out@).skip(before.len() as int)));
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

pub fn qty_exec(arena: &ETermArena, tab: &ETemporal, items: &Vec<I>, ctx: &T, v: &T) -> (out:
    Option<(Vec<u8>, T)>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, v),
    ensures
        out matches Some((_, o)) ==> valid(arena.nodes@, &o) && spec::is_comp(o@, "object"@, 6),
        match out {
            Some((u, o)) => spec::qty(tab@, item_models(items@), ctx@, v@) == Some((u@, o@)),
            None => spec::qty(tab@, item_models(items@), ctx@, v@) is None,
        },
{
    match obj_of_exec(arena, items, ctx, v) {
        None => None,
        Some(o) => {
            let a1 = arg_at(arena, &o, 1);
            let noun = atom_bytes(arena, &a1);
            match assoc_exec(&tab.units, &noun) {
                Some(u) => Some((u, o)),
                None => None,
            }
        },
    }
}

pub fn is_frame_exec(arena: &ETermArena, tab: &ETemporal, items: &Vec<I>, ctx: &T, v: &T) -> (out:
    bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, v),
    ensures
        out == spec::is_frame(tab@, item_models(items@), ctx@, v@),
{
    match obj_of_exec(arena, items, ctx, v) {
        None => false,
        Some(o) => {
            let a1 = arg_at(arena, &o, 1);
            let noun = atom_bytes(arena, &a1);
            crate::m7_temporal::frame_in(tab, &noun)
        },
    }
}

pub fn shape_exec(arena: &mut ETermArena, why: &Sym) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(symbol(&Sym::TemporalShape), seq![Term::Atom(symbol(why))]),
{
    let w = named(arena, why);
    c1(arena, &Sym::TemporalShape, &w)
}

// None = a bound the projection accepts; Some(why) = the bound's reject.
pub fn bound_why_exec(arena: &ETermArena, o: &T) -> (out: Option<Sym>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, o),
        spec::is_comp(o@, "object"@, 6),
    ensures
        match out {
            None => spec::bound_why(o@) is None,
            Some(s) => spec::bound_why(o@) == Some(
                Term::Comp(symbol(&Sym::TemporalShape), seq![Term::Atom(symbol(&s))]),
            ),
        },
{
    let a2 = arg_at(arena, o, 2);
    let a3 = arg_at(arena, o, 3);
    let a4 = arg_at(arena, o, 4);
    let a5 = arg_at(arena, o, 5);
    let ok = is_atom(arena, &a2, &Sym::Countable) && is_atom(arena, &a3, &Sym::Na) && !is_atom(
        arena,
        &a4,
        &Sym::Na,
    );
    if !ok {
        return Some(Sym::NoBound);
    }
    if !is_int(arena, &a5) {
        return Some(Sym::NoBound);
    }
    if int_cmp(arena, &a5, false) {
        None
    } else {
        Some(Sym::ZeroBound)
    }
}

pub enum EAnn {
    Interval(T, Vec<u8>, T, Vec<u8>, T),
    Recurrence(T, T, T, Vec<u8>),
    Window(T, T, T, Vec<u8>, T),
    Frequency(T, T, T, Vec<u8>),
    Order(T, Vec<u8>, T),
}

pub open spec fn ann_view(a: &EAnn) -> spec::Ann {
    match a {
        EAnn::Interval(e, r, q, u, an) => spec::Ann::Interval(e@, r@, q@, u@, an@),
        EAnn::Recurrence(e, f, q, u) => spec::Ann::Recurrence(e@, f@, q@, u@),
        EAnn::Window(e, f, l, u, an) => spec::Ann::Window(e@, f@, l@, u@, an@),
        EAnn::Frequency(e, c, w, u) => spec::Ann::Frequency(e@, c@, w@, u@),
        EAnn::Order(e, r, an) => spec::Ann::Order(e@, r@, an@),
    }
}

pub open spec fn ann_valid(nodes: Seq<ENode>, a: &EAnn) -> bool {
    match a {
        EAnn::Interval(e, _, q, _, an) => valid(nodes, e) && valid(nodes, q) && valid(nodes, an),
        EAnn::Recurrence(e, f, q, _) => valid(nodes, e) && valid(nodes, f) && valid(nodes, q),
        EAnn::Window(e, f, l, _, an) => valid(nodes, e) && valid(nodes, f) && valid(nodes, l)
            && valid(nodes, an),
        EAnn::Frequency(e, c, w, _) => valid(nodes, e) && valid(nodes, c) && valid(nodes, w),
        EAnn::Order(e, _, an) => valid(nodes, e) && valid(nodes, an),
    }
}

pub open spec fn pp_view(r: Result<Option<EAnn>, Sym>) -> Result<Option<spec::Ann>, Term> {
    match r {
        Ok(Some(a)) => Ok(Some(ann_view(&a))),
        Ok(None) => Ok(None),
        Err(s) => Err(Term::Comp(symbol(&Sym::TemporalShape), seq![Term::Atom(symbol(&s))])),
    }
}

// The annotation of one pp in its scope; `none` = the atom `none` (pre-made, so
// the success paths never allocate). Err(why) names the temporal_shape reject.
pub fn pp_ann_exec(
    arena: &ETermArena,
    env: &EEnv,
    scope: &Vec<I>,
    ctx: &T,
    pp: &T,
    none: &T,
) -> (out: Result<Option<EAnn>, Sym>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, pp),
        valid(arena.nodes@, none),
        none@ == ckc_spec::replay::atom("none"@),
        spec::is_comp(pp@, "modifier_pp"@, 3),
    ensures
        out matches Ok(Some(a)) ==> ann_valid(arena.nodes@, &a),
        pp_view(out) == spec::pp_ann(env_view(env), item_models(scope@), ctx@, pp@),
{
    let tab = &env.tab;
    let e = arg_at(arena, pp, 0);
    let a1 = arg_at(arena, pp, 1);
    let x = arg_at(arena, pp, 2);
    let p = atom_bytes(arena, &a1);
    let links = of_links_exec(arena, scope, ctx, &x);
    let role = assoc_exec(&tab.relations, &p);
    let q = qty_exec(arena, tab, scope, ctx, &x);
    proof {
        reveal_strlit("duration");
        reveal(ckc_spec::v1text::ascii);
    }
    if let (Some(role), Some((unit, o))) = (role, q) {
        if let Some(why) = bound_why_exec(arena, &o) {
            return Err(why);
        }
        if links.len() > 1 {
            return Err(Sym::AnchorCount);
        }
        if links.len() == 0 {
            return Ok(Some(EAnn::Interval(e, role, x, unit, none.cp())));
        }
        let is_duration = crate::k4_bytes::eq(&role, b"duration");
        proof {
            reveal_byteslit(b"duration");
            assert(b"duration"@ =~= ckc_spec::v1text::ascii("duration"@));
        }
        if is_duration {
            return Err(Sym::DurationAnchor);
        }
        proof {
            assert(models(links@)[0] == links@[0]@);
        }
        let anchor = links[0].cp();
        if !is_var(arena, &anchor) || qty_exec(arena, tab, scope, ctx, &anchor).is_some()
            || is_frame_exec(arena, tab, scope, ctx, &anchor) {
            return Err(Sym::AnchorShape);
        }
        return Ok(Some(EAnn::Interval(e, role, x, unit, anchor)));
    }
    match obj_of_exec(arena, scope, ctx, &x) {
        None => crate::m7_v3::v3_ann_exec(arena, env, scope, ctx, pp),
        Some(f) => {
            let f1 = arg_at(arena, &f, 1);
            let noun = atom_bytes(arena, &f1);
            if !crate::m7_temporal::pair_in(&tab.spacings, &p, &noun) {
                return crate::m7_v3::v3_ann_exec(arena, env, scope, ctx, pp);
            }
            let f2 = arg_at(arena, &f, 2);
            let f3 = arg_at(arena, &f, 3);
            let f4 = arg_at(arena, &f, 4);
            let f5 = arg_at(arena, &f, 5);
            if !(is_atom(arena, &f2, &Sym::Countable) && is_atom(arena, &f3, &Sym::Na) && is_atom(
                arena,
                &f4,
                &Sym::Eq,
            ) && int_cmp(arena, &f5, true)) {
                return Err(Sym::FrameShape);
            }
            if links.len() != 1 {
                return Err(Sym::AnchorCount);
            }
            proof {
                assert(models(links@)[0] == links@[0]@);
            }
            let qv = links[0].cp();
            match qty_exec(arena, tab, scope, ctx, &qv) {
                None => Err(Sym::FrameShape),
                Some((unit, o)) => {
                    if let Some(why) = bound_why_exec(arena, &o) {
                        return Err(why);
                    }
                    if of_links_exec(arena, scope, ctx, &qv).len() > 0 {
                        return Err(Sym::AnchorCount);
                    }
                    Ok(Some(EAnn::Recurrence(e, x, qv, unit)))
                },
            }
        },
    }
}

pub open spec fn shape_of(s: Sym) -> Term {
    Term::Comp(symbol(&Sym::TemporalShape), seq![Term::Atom(symbol(&s))])
}

pub open spec fn anns_view(v: Seq<(T, EAnn)>) -> Seq<(Term, spec::Ann)> {
    v.map_values(|p: (T, EAnn)| (p.0@, ann_view(&p.1)))
}

pub open spec fn anns_valid(nodes: Seq<ENode>, v: Seq<(T, EAnn)>) -> bool {
    forall|i: int|
        0 <= i < v.len() ==> valid(nodes, &(#[trigger] v[i]).0) && ann_valid(nodes, &v[i].1)
}

pub open spec fn prepend<A>(acc: Seq<A>, r: Result<Seq<A>, Term>) -> Result<Seq<A>, Term> {
    match r {
        Ok(t) => Ok(acc + t),
        Err(e) => Err(e),
    }
}

pub open spec fn err_view<A>(r: Result<A, Sym>) -> Result<A, Term> {
    match r {
        Ok(a) => Ok(a),
        Err(s) => Err(shape_of(s)),
    }
}

// The scope's annotations (`spec::anns` with items == scope).
pub fn anns_exec(arena: &ETermArena, env: &EEnv, scope: &Vec<I>, none: &T) -> (out: Result<
    Vec<(T, EAnn)>,
    Sym,
>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, scope@),
        valid(arena.nodes@, none),
        none@ == ckc_spec::replay::atom("none"@),
    ensures
        out matches Ok(v) ==> anns_valid(arena.nodes@, v@),
        err_view(
            match out {
                Ok(v) => Ok(anns_view(v@)),
                Err(s) => Err(s),
            },
        ) == spec::anns(env_view(env), item_models(scope@), item_models(scope@)),
{
    let ghost ms = item_models(scope@);
    let mut out: Vec<(T, EAnn)> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
        assert(anns_view(out@) =~= Seq::<(Term, spec::Ann)>::empty());
        match spec::anns(env_view(env), ms, ms) {
            Ok(t) => assert(Seq::<(Term, spec::Ann)>::empty() + t =~= t),
            Err(_) => {},
        }
    }
    while i < scope.len()
        invariant
            i <= scope@.len(),
            arena_ok(arena),
            items_valid(arena.nodes@, scope@),
            valid(arena.nodes@, none),
            none@ == ckc_spec::replay::atom("none"@),
            ms == item_models(scope@),
            ms.len() == scope@.len(),
            anns_valid(arena.nodes@, out@),
            spec::anns(env_view(env), ms, ms) == prepend(
                anns_view(out@),
                spec::anns(env_view(env), ms, ms.skip(i as int)),
            ),
        decreases scope@.len() - i,
    {
        proof {
            items_at(arena.nodes@, scope@, i as int);
            assert(ms.skip(i as int)[0] == ms[i as int]);
            assert(ms[i as int] == scope@[i as int]@);
            assert(ms.skip(i as int).drop_first() =~= ms.skip(i + 1));
        }
        let ghost before = out@;
        let ghost rest = spec::anns(env_view(env), ms, ms.skip(i + 1));
        let ghost mut h: Seq<(Term, spec::Ann)> = Seq::empty();
        proof {
            reveal(item_valid);
            reveal_with_fuel(spec::anns, 1);
            assert(spec::anns(env_view(env), ms, ms.skip(i as int)) == {
                let head = match ms[i as int] {
                    spec::Item::Anch(c, inner) => if spec::is_comp(inner, "modifier_pp"@, 3) {
                        match spec::pp_ann(env_view(env), ms, c, inner) {
                            Err(e) => Err(e),
                            Ok(Some(a)) => Ok(seq![(c, a)]),
                            Ok(None) => Ok(Seq::empty()),
                        }
                    } else {
                        Ok(Seq::empty())
                    },
                    _ => Ok(Seq::empty()),
                };
                match (head, rest) {
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                    (Ok(h), Ok(t)) => Ok(h + t),
                }
            });
        }
        match &scope[i].kind {
            Kind::Anch(c, inner) => {
                if is_comp(arena, inner, &Sym::ModifierPp, 3) {
                    match pp_ann_exec(arena, env, scope, c, inner, none) {
                        Err(s) => {
                            return Err(s);
                        },
                        Ok(Some(a)) => {
                            proof {
                                h = seq![(c@, ann_view(&a))];
                            }
                            out.push((c.cp(), a));
                        },
                        Ok(None) => {},
                    }
                }
            },
            _ => {},
        }
        proof {
            assert(anns_view(out@) =~= anns_view(before) + h);
            match rest {
                Ok(t) => assert((anns_view(before) + h) + t =~= anns_view(before) + (h + t)),
                Err(_) => {},
            }
            assert forall|j: int| 0 <= j < out@.len() implies valid(
                arena.nodes@,
                &(#[trigger] out@[j]).0,
            ) && ann_valid(arena.nodes@, &out@[j].1) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(ms.skip(i as int).len() == 0);
        assert(anns_view(out@) + Seq::<(Term, spec::Ann)>::empty() =~= anns_view(out@));
    }
    Ok(out)
}

pub open spec fn pairs_models(v: Seq<(T, T)>) -> Seq<(Term, Term)> {
    v.map_values(|p: (T, T)| (p.0@, p.1@))
}

pub open spec fn pairs_valid(nodes: Seq<ENode>, v: Seq<(T, T)>) -> bool {
    forall|i: int| 0 <= i < v.len() ==> valid(nodes, &(#[trigger] v[i]).0) && valid(nodes, &v[i].1)
}

pub open spec fn claim_of(m: (Term, spec::Ann)) -> Seq<(Term, Term)> {
    match m.1 {
        spec::Ann::Interval(_, _, q, _, _) => seq![(m.0, q)],
        spec::Ann::Recurrence(_, f, q, _) => seq![(m.0, f), (m.0, q)],
        spec::Ann::Window(_, f, l, _, _) => seq![(m.0, f), (m.0, l)],
        spec::Ann::Frequency(_, c, w, _) => seq![(m.0, c), (m.0, w)],
        spec::Ann::Order(_, _, _) => Seq::empty(),
    }
}

pub open spec fn consumed_of(m: (Term, spec::Ann)) -> Seq<(Term, Term)> {
    match m.1 {
        spec::Ann::Interval(_, _, q, _, a) => if a is Var {
            seq![(m.0, q)]
        } else {
            Seq::empty()
        },
        spec::Ann::Recurrence(_, f, _, _) => seq![(m.0, f)],
        spec::Ann::Window(_, f, l, _, _) => seq![(m.0, f), (m.0, l)],
        _ => Seq::empty(),
    }
}

proof fn flat_take(
    s: Seq<(Term, spec::Ann)>,
    f: spec_fn((Term, spec::Ann)) -> Seq<(Term, Term)>,
    i: int,
)
    requires
        0 <= i < s.len(),
    ensures
        s.take(i + 1).map_values(f).flatten() == s.take(i).map_values(f).flatten() + f(s[i]),
{
    assert(s.take(i + 1) =~= s.take(i).push(s[i]));
    assert(s.take(i + 1).map_values(f) =~= s.take(i).map_values(f).push(f(s[i])));
    s.take(i).map_values(f).lemma_flatten_push(f(s[i]));
}

// claims (kind = true) or consumed (kind = false) links of the annotations.
pub fn links_exec(arena: &ETermArena, v: &Vec<(T, EAnn)>, kind: bool) -> (out: Vec<(T, T)>)
    requires
        arena_ok(arena),
        anns_valid(arena.nodes@, v@),
    ensures
        pairs_valid(arena.nodes@, out@),
        pairs_models(out@) == if kind {
            spec::claims(anns_view(v@))
        } else {
            spec::consumed(anns_view(v@))
        },
{
    let ghost av = anns_view(v@);
    let ghost f: spec_fn((Term, spec::Ann)) -> Seq<(Term, Term)> = if kind {
        |m: (Term, spec::Ann)| claim_of(m)
    } else {
        |m: (Term, spec::Ann)| consumed_of(m)
    };
    proof {
        if kind {
            assert(spec::claims(av) =~= av.map_values(f).flatten()) by {
                assert(av.map_values(f) =~= av.map_values(
                    |m: (Term, spec::Ann)|
                        match m.1 {
                            spec::Ann::Interval(_, _, q, _, _) => seq![(m.0, q)],
                            spec::Ann::Recurrence(_, f, q, _) => seq![(m.0, f), (m.0, q)],
                            spec::Ann::Window(_, f, l, _, _) => seq![(m.0, f), (m.0, l)],
                            spec::Ann::Frequency(_, c, w, _) => seq![(m.0, c), (m.0, w)],
                            spec::Ann::Order(_, _, _) => Seq::empty(),
                        },
                ));
            }
        } else {
            assert(spec::consumed(av) =~= av.map_values(f).flatten()) by {
                assert(av.map_values(f) =~= av.map_values(
                    |m: (Term, spec::Ann)|
                        match m.1 {
                            spec::Ann::Interval(_, _, q, _, a) => if a is Var {
                                seq![(m.0, q)]
                            } else {
                                Seq::empty()
                            },
                            spec::Ann::Recurrence(_, f, _, _) => seq![(m.0, f)],
                            spec::Ann::Window(_, f, l, _, _) => seq![(m.0, f), (m.0, l)],
                            _ => Seq::empty(),
                        },
                ));
            }
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
            anns_valid(arena.nodes@, v@),
            av == anns_view(v@),
            av.len() == v@.len(),
            pairs_valid(arena.nodes@, out@),
            pairs_models(out@) == av.take(i as int).map_values(f).flatten(),
            f == (if kind {
                |m: (Term, spec::Ann)| claim_of(m)
            } else {
                |m: (Term, spec::Ann)| consumed_of(m)
            }),
        decreases v@.len() - i,
    {
        let ghost before = out@;
        let c = &v[i].0;
        match &v[i].1 {
            EAnn::Interval(_, _, q, _, a) => {
                if kind || is_var(arena, a) {
                    out.push((c.cp(), q.cp()));
                }
            },
            EAnn::Recurrence(_, fr, q, _) => {
                if kind {
                    out.push((c.cp(), fr.cp()));
                    out.push((c.cp(), q.cp()));
                } else {
                    out.push((c.cp(), fr.cp()));
                }
            },
            EAnn::Window(_, fr, l, _, _) => {
                out.push((c.cp(), fr.cp()));
                out.push((c.cp(), l.cp()));
            },
            EAnn::Frequency(_, cnt, w, _) => {
                if kind {
                    out.push((c.cp(), cnt.cp()));
                    out.push((c.cp(), w.cp()));
                }
            },
            EAnn::Order(_, _, _) => {},
        }
        proof {
            flat_take(av, f, i as int);
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

pub fn no_dup_exec(arena: &ETermArena, v: &Vec<(T, T)>) -> (out: bool)
    requires
        arena_ok(arena),
        pairs_valid(arena.nodes@, v@),
    ensures
        out == pairs_models(v@).no_duplicates(),
{
    let ghost pm = pairs_models(v@);
    let mut i = 0usize;
    while i < v.len()
        invariant
            i <= v@.len(),
            arena_ok(arena),
            pairs_valid(arena.nodes@, v@),
            pm == pairs_models(v@),
            pm.len() == v@.len(),
            forall|a: int, b: int| 0 <= a < b < i ==> pm[a] != pm[b],
        decreases v@.len() - i,
    {
        let mut j = 0usize;
        while j < i
            invariant
                j <= i < v@.len(),
                arena_ok(arena),
                pairs_valid(arena.nodes@, v@),
                pm == pairs_models(v@),
                pm.len() == v@.len(),
                forall|a: int, b: int| 0 <= a < b < i ==> pm[a] != pm[b],
                forall|a: int| 0 <= a < j ==> pm[a] != pm[i as int],
            decreases i - j,
        {
            if crate::m6_term::equal(arena, &v[i].0, &v[j].0) && crate::m6_term::equal(
                arena,
                &v[i].1,
                &v[j].1,
            ) {
                proof {
                    assert(pm[j as int] == pm[i as int]);
                }
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

pub fn link_in(arena: &ETermArena, links: &Vec<(T, T)>, c: &T, x: &T) -> (out: bool)
    requires
        arena_ok(arena),
        pairs_valid(arena.nodes@, links@),
        valid(arena.nodes@, c),
        valid(arena.nodes@, x),
    ensures
        out == pairs_models(links@).contains((c@, x@)),
{
    let ghost pm = pairs_models(links@);
    let mut i = 0usize;
    while i < links.len()
        invariant
            i <= links@.len(),
            arena_ok(arena),
            pairs_valid(arena.nodes@, links@),
            valid(arena.nodes@, c),
            valid(arena.nodes@, x),
            pm == pairs_models(links@),
            pm.len() == links@.len(),
            forall|a: int| 0 <= a < i ==> pm[a] != (c@, x@),
        decreases links@.len() - i,
    {
        if crate::m6_term::equal(arena, &links[i].0, c) && crate::m6_term::equal(
            arena,
            &links[i].1,
            x,
        ) {
            proof {
                assert(pm[i as int] == (c@, x@));
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn ann_inner_exec(arena: &mut ETermArena, a: &EAnn, ms: &Vec<(T, EAnn)>, cx: &T) -> (out:
    Option<T>)
    requires
        arena_ok(old(arena)),
        ann_valid(old(arena).nodes@, a),
        anns_valid(old(arena).nodes@, ms@),
        valid(old(arena).nodes@, cx),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(t) ==> valid(final(arena).nodes@, &t),
        opt_model(out) == spec::ann_inner(ann_view(a), anns_view(ms@), cx@),
{
    let ghost start = arena.nodes@;
    match a {
        EAnn::Interval(e, role, q, unit, anchor) => {
            let r = atom(arena, role);
            let ghost mid = arena.nodes@;
            let u = atom(arena, unit);
            proof {
                crate::m6_term::prefix(start, arena.nodes@, e);
                crate::m6_term::prefix(start, arena.nodes@, q);
                crate::m6_term::prefix(start, arena.nodes@, anchor);
                crate::m6_term::prefix(mid, arena.nodes@, &r);
            }
            let mut ts = Vec::new();
            ts.push(e.cp());
            ts.push(r);
            ts.push(q.cp());
            ts.push(u);
            ts.push(anchor.cp());
            proof {
                assert(models(ts@) =~= seq![e@, Term::Atom(role@), q@, Term::Atom(unit@), anchor@]);
            }
            Some(c(arena, &Sym::DollarGuidelineInterval, &ts))
        },
        EAnn::Recurrence(e, _, q, unit) => {
            match crate::m7_v3::window_of_exec(arena, ms, cx, e) {
                None => {
                    let u = atom(arena, unit);
                    proof {
                        crate::m6_term::prefix(start, arena.nodes@, e);
                        crate::m6_term::prefix(start, arena.nodes@, q);
                    }
                    let mut ts = Vec::new();
                    ts.push(e.cp());
                    ts.push(q.cp());
                    ts.push(u);
                    proof {
                        assert(models(ts@) =~= seq![e@, q@, Term::Atom(unit@)]);
                    }
                    Some(c(arena, &Sym::DollarGuidelineRecurrence, &ts))
                },
                Some((l, lunit, anchor)) => {
                    let u = atom(arena, unit);
                    let ghost mid = arena.nodes@;
                    let lu = atom(arena, &lunit);
                    proof {
                        crate::m6_term::prefix(start, arena.nodes@, e);
                        crate::m6_term::prefix(start, arena.nodes@, q);
                        crate::m6_term::prefix(start, arena.nodes@, &l);
                        crate::m6_term::prefix(start, arena.nodes@, &anchor);
                        crate::m6_term::prefix(mid, arena.nodes@, &u);
                    }
                    let mut ts = Vec::new();
                    ts.push(e.cp());
                    ts.push(q.cp());
                    ts.push(u);
                    ts.push(anchor.cp());
                    ts.push(l.cp());
                    ts.push(lu);
                    proof {
                        assert(models(ts@) =~= seq![
                            e@,
                            q@,
                            Term::Atom(unit@),
                            anchor@,
                            l@,
                            Term::Atom(lunit@),
                        ]);
                    }
                    Some(c(arena, &Sym::DollarGuidelineRecurrenceWindow, &ts))
                },
            }
        },
        EAnn::Window(_, _, _, _, _) => None,
        EAnn::Frequency(e, cnt, w, unit) => {
            let u = atom(arena, unit);
            proof {
                crate::m6_term::prefix(start, arena.nodes@, e);
                crate::m6_term::prefix(start, arena.nodes@, cnt);
                crate::m6_term::prefix(start, arena.nodes@, w);
            }
            let mut ts = Vec::new();
            ts.push(e.cp());
            ts.push(cnt.cp());
            ts.push(w.cp());
            ts.push(u);
            proof {
                assert(models(ts@) =~= seq![e@, cnt@, w@, Term::Atom(unit@)]);
            }
            Some(c(arena, &Sym::DollarGuidelineFrequency, &ts))
        },
        EAnn::Order(e, role, anchor) => {
            let r = atom(arena, role);
            proof {
                crate::m6_term::prefix(start, arena.nodes@, e);
                crate::m6_term::prefix(start, arena.nodes@, anchor);
            }
            let mut ts = Vec::new();
            ts.push(e.cp());
            ts.push(r);
            ts.push(anchor.cp());
            proof {
                assert(models(ts@) =~= seq![e@, Term::Atom(role@), anchor@]);
            }
            Some(c(arena, &Sym::DollarGuidelineOrder, &ts))
        },
    }
}

pub open spec fn items_res(r: Result<Vec<I>, Sym>) -> Result<Seq<spec::Item>, Term> {
    match r {
        Ok(v) => Ok(item_models(v@)),
        Err(s) => Err(shape_of(s)),
    }
}

pub fn rebuild_item_exec(
    arena: &mut ETermArena,
    env: &EEnv,
    scope: &Vec<I>,
    anns: &Vec<(T, EAnn)>,
    links: &Vec<(T, T)>,
    none: &T,
    it: &I,
) -> (out: Result<Vec<I>, Sym>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, scope@),
        anns_valid(old(arena).nodes@, anns@),
        pairs_valid(old(arena).nodes@, links@),
        valid(old(arena).nodes@, none),
        none@ == ckc_spec::replay::atom("none"@),
        item_valid(old(arena).nodes@, it),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(v) ==> items_valid(final(arena).nodes@, v@),
        items_res(out) == spec::rebuild_item(
            env_view(env),
            item_models(scope@),
            anns_view(anns@),
            pairs_models(links@),
            it@,
        ),
    decreases it, 0int,
{
    let ghost start = arena.nodes@;
    proof {
        reveal(item_valid);
    }
    match &it.kind {
        Kind::Anch(c, inner) => {
            if is_comp(arena, inner, &Sym::ModifierPp, 3) {
                match pp_ann_exec(arena, env, scope, c, inner, none) {
                    Ok(Some(a)) => {
                        let inner_t = ann_inner_exec(arena, &a, anns, c);
                        proof {
                            item_prefix(start, arena.nodes@, it);
                            crate::m6_term::prefix(start, arena.nodes@, c);
                        }
                        match inner_t {
                            Some(ai) => {
                                let first = copy_item(arena, it);
                                let second = anch(arena, c, &ai);
                                let mut v = Vec::new();
                                v.push(first);
                                v.push(second);
                                proof {
                                    reveal_with_fuel(items_valid, 3);
                                    assert(item_models(v@) =~= seq![
                                        it@,
                                        spec::Item::Anch(c@, ai@),
                                    ]);
                                }
                                return Ok(v);
                            },
                            None => {
                                let one = copy_item(arena, it);
                                let mut v = Vec::new();
                                v.push(one);
                                proof {
                                    reveal_with_fuel(items_valid, 2);
                                    assert(item_models(v@) =~= seq![it@]);
                                }
                                return Ok(v);
                            },
                        }
                    },
                    _ => {},
                }
            } else if is_comp(arena, inner, &Sym::Relation, 3) {
                let a1 = arg_at(arena, inner, 1);
                let a0 = arg_at(arena, inner, 0);
                if is_atom(arena, &a1, &Sym::Of) && link_in(arena, links, c, &a0) {
                    proof {
                        reveal_with_fuel(items_valid, 1);
                        assert(item_models(Seq::<I>::empty()) =~= Seq::<spec::Item>::empty());
                    }
                    return Ok(Vec::new());
                }
            }
        },
        Kind::Naf { dom, payload } => {
            return match annotate_exec(arena, env, payload, none) {
                Err(s) => Err(s),
                Ok(p) => {
                    proof {
                        crate::m6_term::prefix_all(start, arena.nodes@, dom@);
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
            };
        },
        Kind::Op { .. } => {},
    }
    let one = copy_item(arena, it);
    let mut v = Vec::new();
    v.push(one);
    proof {
        reveal_with_fuel(items_valid, 2);
        assert(item_models(v@) =~= seq![it@]);
    }
    Ok(v)
}

pub fn rebuild_exec(
    arena: &mut ETermArena,
    env: &EEnv,
    scope: &Vec<I>,
    anns: &Vec<(T, EAnn)>,
    links: &Vec<(T, T)>,
    none: &T,
    items: &Vec<I>,
) -> (out: Result<Vec<I>, Sym>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, scope@),
        items_valid(old(arena).nodes@, items@),
        anns_valid(old(arena).nodes@, anns@),
        pairs_valid(old(arena).nodes@, links@),
        valid(old(arena).nodes@, none),
        none@ == ckc_spec::replay::atom("none"@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(v) ==> items_valid(final(arena).nodes@, v@),
        items_res(out) == spec::rebuild(
            env_view(env),
            item_models(scope@),
            anns_view(anns@),
            pairs_models(links@),
            item_models(items@),
        ),
    decreases items@, 1int,
{
    let ghost start = arena.nodes@;
    proof {
        assert(start == old(arena).nodes@);
    }
    let ghost ms = item_models(items@);
    let ghost sm = item_models(scope@);
    let ghost lm = pairs_models(links@);
    let ghost am = anns_view(anns@);
    let mut out: Vec<I> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) =~= ms);
        reveal_with_fuel(items_valid, 1);
        assert(item_models(out@) =~= Seq::<spec::Item>::empty());
        match spec::rebuild(env_view(env), sm, am, lm, ms) {
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
            anns_valid(arena.nodes@, anns@),
            am == anns_view(anns@),
            pairs_valid(arena.nodes@, links@),
            valid(arena.nodes@, none),
            none@ == ckc_spec::replay::atom("none"@),
            items_valid(arena.nodes@, out@),
            ms == item_models(items@),
            sm == item_models(scope@),
            lm == pairs_models(links@),
            ms.len() == items@.len(),
            spec::rebuild(env_view(env), sm, am, lm, ms) == prepend(
                item_models(out@),
                spec::rebuild(env_view(env), sm, am, lm, ms.skip(i as int)),
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
        let ghost rest = spec::rebuild(env_view(env), sm, am, lm, ms.skip(i + 1));
        proof {
            reveal_with_fuel(spec::rebuild, 1);
            assert(spec::rebuild(env_view(env), sm, am, lm, ms.skip(i as int)) == match (
                spec::rebuild_item(env_view(env), sm, am, lm, ms[i as int]),
                rest,
            ) {
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
                (Ok(h), Ok(t)) => Ok(h + t),
            });
        }
        let r = rebuild_item_exec(arena, env, scope, anns, links, none, &items[i]);
        proof {
            items_prefix(before_nodes, arena.nodes@, scope@);
            items_prefix(before_nodes, arena.nodes@, items@);
            items_prefix(before_nodes, arena.nodes@, out@);
            crate::m6_term::prefix(before_nodes, arena.nodes@, none);
            anns_prefix(before_nodes, arena.nodes@, anns@);
            assert forall|j: int| 0 <= j < links@.len() implies valid(
                arena.nodes@,
                &(#[trigger] links@[j]).0,
            ) && valid(arena.nodes@, &links@[j].1) by {
                crate::m6_term::prefix(before_nodes, arena.nodes@, &links@[j].0);
                crate::m6_term::prefix(before_nodes, arena.nodes@, &links@[j].1);
            }
            crate::k2_load::prefix_chain(start, before_nodes, arena.nodes@);
        }
        match r {
            Err(s) => {
                proof {
                    assert(start.is_prefix_of(arena.nodes@));
                }
                return Err(s);
            },
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

pub fn annotate_exec(arena: &mut ETermArena, env: &EEnv, items: &Vec<I>, none: &T) -> (out: Result<
    Vec<I>,
    Sym,
>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
        valid(old(arena).nodes@, none),
        none@ == ckc_spec::replay::atom("none"@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(v) ==> items_valid(final(arena).nodes@, v@),
        items_res(out) == spec::annotate(env_view(env), item_models(items@)),
    decreases items@, 2int,
{
    let ms = match anns_exec(arena, env, items, none) {
        Err(s) => return Err(s),
        Ok(ms) => ms,
    };
    let claims = links_exec(arena, &ms, true);
    if !no_dup_exec(arena, &claims) {
        return Err(Sym::SharedQuantity);
    }
    if !crate::m7_v3::windows_ok_exec(arena, &ms) {
        return Err(Sym::WindowShape);
    }
    let consumed = links_exec(arena, &ms, false);
    rebuild_exec(arena, env, items, &ms, &consumed, none, items)
}

pub proof fn anns_prefix(before: Seq<ENode>, after: Seq<ENode>, v: Seq<(T, EAnn)>)
    requires
        before.is_prefix_of(after),
        anns_valid(before, v),
    ensures
        anns_valid(after, v),
{
    assert forall|i: int| 0 <= i < v.len() implies valid(after, &(#[trigger] v[i]).0) && ann_valid(
        after,
        &v[i].1,
    ) by {
        assert(valid(before, &v[i].0) && ann_valid(before, &v[i].1));
        crate::m6_term::prefix(before, after, &v[i].0);
        match &v[i].1 {
            EAnn::Interval(e, _, q, _, an) => {
                crate::m6_term::prefix(before, after, e);
                crate::m6_term::prefix(before, after, q);
                crate::m6_term::prefix(before, after, an);
            },
            EAnn::Recurrence(e, f, q, _) => {
                crate::m6_term::prefix(before, after, e);
                crate::m6_term::prefix(before, after, f);
                crate::m6_term::prefix(before, after, q);
            },
            EAnn::Window(e, f, l, _, an) => {
                crate::m6_term::prefix(before, after, e);
                crate::m6_term::prefix(before, after, f);
                crate::m6_term::prefix(before, after, l);
                crate::m6_term::prefix(before, after, an);
            },
            EAnn::Frequency(e, c, w, _) => {
                crate::m6_term::prefix(before, after, e);
                crate::m6_term::prefix(before, after, c);
                crate::m6_term::prefix(before, after, w);
            },
            EAnn::Order(e, _, an) => {
                crate::m6_term::prefix(before, after, e);
                crate::m6_term::prefix(before, after, an);
            },
        }
    }
}

pub open spec fn flat_res(r: Result<Flat, T>) -> Result<spec::Flat, Term> {
    flat_result(r)
}

// flatten_list, then (v2) the annotate pass over the flat items.
pub fn annotated_exec(arena: &mut ETermArena, tab: &Option<EEnv>, items: Vec<I>) -> (out: Result<
    Vec<I>,
    T,
>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match out {
            Ok(v) => items_valid(final(arena).nodes@, v@),
            Err(t) => valid(final(arena).nodes@, &t),
        },
        match out {
            Ok(v) => Ok(item_models(v@)),
            Err(t) => Err(t@),
        } == spec::annotated(tab_view(tab), item_models(items@)),
{
    match tab {
        None => Ok(items),
        Some(t) => {
            let ghost start = arena.nodes@;
            let none = named(arena, &Sym::None);
            proof {
                items_prefix(start, arena.nodes@, items@);
                reveal_strlit("none");
                reveal(ckc_spec::v1text::ascii);
            }
            match annotate_exec(arena, t, &items, &none) {
                Ok(v) => Ok(v),
                Err(s) => {
                    let w = shape_exec(arena, &s);
                    Err(w)
                },
            }
        },
    }
}

pub fn flatten_ann_exec(
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
        flat_result(out) == spec::flatten_ann(
            tab_view(tab),
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
    match crate::m6_flat::flatten_list(arena, l, w, env, deps, outer, encl, n) {
        Err(e) => Err(e),
        Ok(f) => {
            let fn_ = f.n;
            match annotated_exec(arena, tab, f.items) {
                Err(e) => Err(e),
                Ok(items) => Ok(Flat { items, n: fn_ }),
            }
        },
    }
}

pub fn clone_tab(t: &Option<EEnv>) -> (out: Option<EEnv>)
    ensures
        tab_view(&out) == tab_view(t),
{
    crate::m7_v3::clone_env(t)
}

pub open spec fn table_view(o: Option<Option<ETemporal>>) -> Option<Option<Temporal>> {
    match o {
        None => None,
        Some(t) => Some(ttab_view(&t)),
    }
}

// spec::table_of: the projection table of a certification (m7t D8).
pub fn table_exec(traw: Option<&Vec<u8>>, tsha: Option<&Vec<u8>>) -> (out: Option<
    Option<ETemporal>,
>)
    ensures
        table_view(out) == spec::table_of(spec::opt_view(traw), spec::opt_view(tsha)),
{
    match (traw, tsha) {
        (None, None) => Some(None),
        (Some(raw), Some(_)) => match crate::m7_temporal::parse_temporal_exec(raw) {
            Ok((t, _)) => Some(Some(t)),
            Err(_) => None,
        },
        _ => None,
    }
}

// The recorded temporal digest (`digest` = its bytes, empty = none) equals tsha.
pub fn temporal_ok(
    version: u8,
    digest: &Vec<u8>,
    tsha: Option<&Vec<u8>>,
    Ghost(t): Ghost<Option<Seq<u8>>>,
) -> (out: bool)
    requires
        ckc_spec::v1text::version_ok(version as nat, t),
        digest@ == crate::v1_term_impl::ulex_digest_bytes(t),
    ensures
        out == (t == spec::opt_view(tsha)),
{
    proof {
        reveal(ckc_spec::v1text::version_ok);
    }
    match tsha {
        None => version == 1,
        Some(h) => (version == 2 || version == 3) && crate::k4_bytes::eq(digest, h),
    }
}

} // verus!
