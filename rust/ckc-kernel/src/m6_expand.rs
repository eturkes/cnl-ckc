#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_model::*;
use crate::m6_refs::lookup;
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text::BodyItem;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn term_result(r: Result<T, T>) -> Result<Term, Term> {
    match r {
        Ok(t) => Ok(t@),
        Err(e) => Err(e@),
    }
}

pub open spec fn term_result_valid(nodes: Seq<ENode>, r: &Result<T, T>) -> bool {
    match r {
        Ok(t) | Err(t) => valid(nodes, t),
    }
}

pub open spec fn terms_result(r: Result<Vec<T>, T>) -> Result<Seq<Term>, Term> {
    match r {
        Ok(ts) => Ok(models(ts@)),
        Err(e) => Err(e@),
    }
}

pub open spec fn terms_result_valid(nodes: Seq<ENode>, r: &Result<Vec<T>, T>) -> bool {
    match r {
        Ok(ts) => valid_all(nodes, ts@),
        Err(e) => valid(nodes, e),
    }
}

pub open spec fn bodies_result(r: Result<Vec<Body>, T>) -> Result<Seq<BodyItem>, Term> {
    match r {
        Ok(bs) => Ok(body_models(bs@)),
        Err(e) => Err(e@),
    }
}

pub open spec fn bodies_result_valid(nodes: Seq<ENode>, r: &Result<Vec<Body>, T>) -> bool {
    match r {
        Ok(bs) => bodies_valid(nodes, bs@),
        Err(e) => valid(nodes, e),
    }
}

pub fn one_term(arena: &ETermArena, t: T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, &t),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == seq![t@],
{
    let mut out = Vec::new();
    out.push(t);
    proof {
        assert_seqs_equal!(models(out@) == seq![t@]);
    }
    out
}

pub fn lit4(arena: &mut ETermArena, sym: &Sym, a: &T, b: &T, d: &T, e: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, a),
        valid(old(arena).nodes@, b),
        valid(old(arena).nodes@, d),
        valid(old(arena).nodes@, e),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(symbol(sym), seq![a@, b@, d@, e@]),
{
    let mut ts = Vec::new();
    ts.push(a.cp());
    ts.push(b.cp());
    ts.push(d.cp());
    ts.push(e.cp());
    proof {
        assert_seqs_equal!(models(ts@) == seq![a@, b@, d@, e@]);
    }
    crate::m6_term::c(arena, sym, &ts)
}

pub fn lit5(arena: &mut ETermArena, sym: &Sym, a: &T, b: &T, d: &T, e: &T, f: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, a),
        valid(old(arena).nodes@, b),
        valid(old(arena).nodes@, d),
        valid(old(arena).nodes@, e),
        valid(old(arena).nodes@, f),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(symbol(sym), seq![a@, b@, d@, e@, f@]),
{
    let mut ts = Vec::new();
    ts.push(a.cp());
    ts.push(b.cp());
    ts.push(d.cp());
    ts.push(e.cp());
    ts.push(f.cp());
    proof {
        assert_seqs_equal!(models(ts@) == seq![a@, b@, d@, e@, f@]);
    }
    crate::m6_term::c(arena, sym, &ts)
}

pub fn resolve(arena: &mut ETermArena, a: &T, map: &Vec<Binding>, sko: &Vec<Binding>) -> (out:
    Result<T, T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, a),
        map_valid(old(arena).nodes@, map@),
        map_valid(old(arena).nodes@, sko@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        term_result_valid(final(arena).nodes@, &out),
        out is Ok ==> final(arena).nodes@ == old(arena).nodes@,
        term_result(out) == spec::resolve(a@, map_model(map@), map_model(sko@)),
{
    if is_var(arena, a) {
        if let Some(t) = lookup(arena, map, a) {
            Ok(t)
        } else if let Some(t) = lookup(arena, sko, a) {
            Ok(t)
        } else {
            Ok(a.cp())
        }
    } else {
        Err(named(arena, &Sym::UnresolvedArgument))
    }
}

pub fn card_ok(arena: &ETermArena, op: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, op),
    ensures
        out == spec::card_op_ok(op@),
{
    is_atom(arena, op, &Sym::Eq) || is_atom(arena, op, &Sym::Geq) || is_atom(
        arena,
        op,
        &Sym::Greater,
    ) || is_atom(arena, op, &Sym::Leq) || is_atom(arena, op, &Sym::Less) || is_atom(
        arena,
        op,
        &Sym::Exactly,
    ) || is_atom(arena, op, &Sym::Na)
}

pub fn participants(
    arena: &mut ETermArena,
    ctx: &T,
    event: &T,
    args: &Vec<T>,
    pos: usize,
    map: &Vec<Binding>,
    sko: &Vec<Binding>,
) -> (out: Result<Vec<T>, T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, ctx),
        valid(old(arena).nodes@, event),
        valid_all(old(arena).nodes@, args@),
        map_valid(old(arena).nodes@, map@),
        map_valid(old(arena).nodes@, sko@),
        pos as nat + args.len() <= usize::MAX as nat,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        terms_result_valid(final(arena).nodes@, &out),
        terms_result(out) == spec::participants(
            ctx@,
            event@,
            models(args@),
            pos as nat,
            map_model(map@),
            map_model(sko@),
        ),
    decreases args.len(),
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::participants, 1);
    }
    if args.len() == 0 {
        let out = Vec::new();
        proof {
            assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
        }
        return Ok(out);
    }
    let resolved = match resolve(arena, &args[0], map, sko) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };
    let rest_args = tail(args, 1);
    let p = crate::m6_term::int(arena, pos);
    let ghost n1 = arena.nodes@;
    proof {
        prefix(start, n1, ctx);
        prefix(start, n1, event);
        prefix(start, n1, &resolved);
    }
    let first = lit4(arena, &Sym::GuidelineArg, ctx, event, &p, &resolved);
    let ghost middle = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, middle);
        prefix(start, middle, ctx);
        prefix(start, middle, event);
        prefix_all(start, middle, rest_args@);
        map_prefix(start, middle, map@);
        map_prefix(start, middle, sko@);
    }
    let rest = participants(arena, ctx, event, &rest_args, pos + 1, map, sko);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
    }
    match rest {
        Err(e) => Err(e),
        Ok(ts) => {
            proof {
                prefix(middle, arena.nodes@, &first);
            }
            let head = one_term(arena, first);
            Ok(concat(&head, &ts))
        },
    }
}

pub fn condition(
    arena: &mut ETermArena,
    ctx: &T,
    inner: &T,
    map: &Vec<Binding>,
    sko: &Vec<Binding>,
) -> (out: Result<Vec<T>, T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, ctx),
        valid(old(arena).nodes@, inner),
        map_valid(old(arena).nodes@, map@),
        map_valid(old(arena).nodes@, sko@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        terms_result_valid(final(arena).nodes@, &out),
        terms_result(out) == spec::condition(ctx@, inner@, map_model(map@), map_model(sko@)),
{
    let ghost start = arena.nodes@;
    proof {
        reveal(spec::condition);
    }
    match parts(arena, inner) {
        None => Err(named(arena, &Sym::ConditionShape)),
        Some((name, args)) => {
            proof {
                assert(inner@ == Term::Comp(name@, models(args@)));
            }
            if has_name(&name, &Sym::Object) && args.len() == 6 {
                if !card_ok(arena, &args[4]) {
                    return Err(named(arena, &Sym::ObjectOperator));
                }
                let r = match resolve(arena, &args[0], map, sko) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                let entity = lit4(arena, &Sym::GuidelineEntity, ctx, &r, &args[1], &args[2]);
                let ghost middle = arena.nodes@;
                proof {
                    prefix(start, middle, ctx);
                    prefix(start, middle, &r);
                    prefix_all(start, middle, args@);
                }
                let card = lit5(
                    arena,
                    &Sym::GuidelineCardinality,
                    ctx,
                    &r,
                    &args[3],
                    &args[4],
                    &args[5],
                );
                proof {
                    crate::k2_load::prefix_chain(start, middle, arena.nodes@);
                    prefix(middle, arena.nodes@, &entity);
                }
                let mut out = Vec::new();
                out.push(entity);
                out.push(card);
                proof {
                    assert_seqs_equal!(models(out@) == seq![entity@, card@]);
                }
                Ok(out)
            } else if has_name(&name, &Sym::Predicate) && args.len() >= 3 {
                if args.len() > 5 {
                    return Err(named(arena, &Sym::ConditionShape));
                }
                let event = match resolve(arena, &args[0], map, sko) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                let as_ = tail(&args, 2);
                let ps = match participants(arena, ctx, &event, &as_, 1, map, sko) {
                    Ok(ts) => ts,
                    Err(e) => return Err(e),
                };
                let ghost middle = arena.nodes@;
                proof {
                    prefix(start, middle, ctx);
                    prefix(start, middle, &event);
                    prefix_all(start, middle, args@);
                }
                let event_term = c3(arena, &Sym::GuidelineEvent, ctx, &event, &args[1]);
                proof {
                    crate::k2_load::prefix_chain(start, middle, arena.nodes@);
                    prefix_all(middle, arena.nodes@, ps@);
                }
                let head = one_term(arena, event_term);
                Ok(concat(&head, &ps))
            } else if has_name(&name, &Sym::ModifierPp) && args.len() == 3 {
                let event = match resolve(arena, &args[0], map, sko) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                let obj = match resolve(arena, &args[2], map, sko) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                let t = lit4(arena, &Sym::GuidelinePp, ctx, &event, &args[1], &obj);
                Ok(one_term(arena, t))
            } else if has_name(&name, &Sym::Property) && args.len() == 3 {
                if !is_atom(arena, &args[2], &Sym::Pos) {
                    return Err(named(arena, &Sym::PropertyPolarity));
                }
                let r = match resolve(arena, &args[0], map, sko) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                let t = lit4(arena, &Sym::GuidelineProperty, ctx, &r, &args[1], &args[2]);
                Ok(one_term(arena, t))
            } else {
                Err(named(arena, &Sym::ConditionShape))
            }
        },
    }
}

pub fn all_pos(bs: &Vec<Body>) -> (out: bool)
    ensures
        out == spec::all_pos(body_models(bs@)),
{
    let mut i = 0usize;
    while i < bs.len()
        invariant
            i <= bs.len(),
            forall|j: int| 0 <= j < i ==> (#[trigger] body_models(bs@)[j]) is Pos,
        decreases bs.len() - i,
    {
        if let Body::Naf(_) = &bs[i] {
            proof {
                assert(body_models(bs@)[i as int] is Naf);
                assert(!spec::all_pos(body_models(bs@)));
            }
            return false;
        }
        i += 1;
    }
    true
}

pub fn pos_terms(arena: &ETermArena, bs: &Vec<Body>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, bs@),
        spec::all_pos(body_models(bs@)),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::pos_terms(body_models(bs@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bs.len()
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, bs@),
            spec::all_pos(body_models(bs@)),
            valid_all(arena.nodes@, out@),
            i <= bs.len(),
            out.len() == i,
            models(out@) == spec::pos_terms(body_models(bs@)).take(i as int),
        decreases bs.len() - i,
    {
        proof {
            assert(body_valid(arena.nodes@, &bs@[i as int]));
            assert(body_models(bs@)[i as int] is Pos);
        }
        if let Body::Pos(t) = &bs[i] {
            let value = t.cp();
            proof {
                extend(arena.nodes@, out@, value);
            }
            out.push(value);
            proof {
                assert_seqs_equal!(models(out@) == spec::pos_terms(body_models(bs@)).take(i as int + 1));
            }
        }
        i += 1;
    }
    out
}

pub fn pos_all(arena: &ETermArena, ts: &Vec<T>) -> (out: Vec<Body>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ts@),
    ensures
        bodies_valid(arena.nodes@, out@),
        body_models(out@) == spec::pos_all(models(ts@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < ts.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ts@),
            bodies_valid(arena.nodes@, out@),
            i <= ts.len(),
            out.len() == i,
            body_models(out@) == spec::pos_all(models(ts@)).take(i as int),
        decreases ts.len() - i,
    {
        let next = Body::Pos(ts[i].cp());
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] body_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(body_models(out@) == spec::pos_all(models(ts@)).take(i as int + 1));
        }
        i += 1;
    }
    out
}

pub fn one_body(arena: &ETermArena, b: Body) -> (out: Vec<Body>)
    requires
        arena_ok(arena),
        body_valid(arena.nodes@, &b),
    ensures
        bodies_valid(arena.nodes@, out@),
        body_models(out@) == seq![b@],
{
    let mut out = Vec::new();
    out.push(b);
    proof {
        assert_seqs_equal!(body_models(out@) == seq![b@]);
    }
    out
}

pub fn join_bodies(arena: &ETermArena, a: &Vec<Body>, b: &Vec<Body>) -> (out: Vec<Body>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, a@),
        bodies_valid(arena.nodes@, b@),
    ensures
        bodies_valid(arena.nodes@, out@),
        body_models(out@) == body_models(a@) + body_models(b@),
{
    let mut out = copy_body(arena, a);
    let mut rest = copy_body(arena, b);
    let ghost first = out@;
    let ghost last = rest@;
    out.append(&mut rest);
    proof {
        assert forall|j: int| 0 <= j < out.len() implies #[trigger] body_valid(
            arena.nodes@,
            &out@[j],
        ) by {
            if j < first.len() {
                assert(out@[j] == first[j]);
            } else {
                assert(out@[j] == last[j - first.len()]);
            }
        }
        assert_seqs_equal!(body_models(out@) == body_models(a@) + body_models(b@));
    }
    out
}

pub open spec fn body_prefix(a: Seq<BodyItem>, b: Result<Seq<BodyItem>, Term>) -> Result<
    Seq<BodyItem>,
    Term,
> {
    match b {
        Ok(b) => Ok(a + b),
        Err(e) => Err(e),
    }
}

pub proof fn body_prefix_assoc(a: Seq<BodyItem>, b: Seq<BodyItem>, r: Result<Seq<BodyItem>, Term>)
    ensures
        body_prefix(a, body_prefix(b, r)) == body_prefix(a + b, r),
{
    match r {
        Ok(c) => {
            assert(a + (b + c) == (a + b) + c);
        },
        Err(_) => {},
    }
}

pub fn expand_item(arena: &mut ETermArena, it: &I, map: &Vec<Binding>, sko: &Vec<Binding>) -> (out:
    Result<Vec<Body>, T>)
    requires
        arena_ok(old(arena)),
        item_valid(old(arena).nodes@, it),
        map_valid(old(arena).nodes@, map@),
        map_valid(old(arena).nodes@, sko@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        bodies_result_valid(final(arena).nodes@, &out),
        bodies_result(out) == spec::expand_item(it@, map_model(map@), map_model(sko@)),
    decreases it, 0int,
{
    let ghost start = arena.nodes@;
    proof {
        reveal(item_valid);
        reveal_with_fuel(spec::expand_item, 1);
    }
    match &it.kind {
        Kind::Op { outer, inner, op, .. } => {
            let operator = atom(arena, op);
            let ghost middle = arena.nodes@;
            proof {
                prefix(start, middle, outer);
                prefix(start, middle, inner);
            }
            let t = c3(arena, &Sym::GuidelineOperator, outer, inner, &operator);
            proof {
                crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            }
            Ok(one_body(arena, Body::Pos(t)))
        },
        Kind::Anch(ctx, inner) => match condition(arena, ctx, inner, map, sko) {
            Err(e) => Err(e),
            Ok(ts) => Ok(pos_all(arena, &ts)),
        },
        Kind::Naf { payload, .. } => match expand(arena, payload, map, sko) {
            Err(e) => Err(e),
            Ok(goals) => {
                if all_pos(&goals) {
                    let ts = pos_terms(arena, &goals);
                    Ok(one_body(arena, Body::Naf(ts)))
                } else {
                    let ghost middle = arena.nodes@;
                    let e = named(arena, &Sym::NafShape);
                    proof {
                        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
                    }
                    Err(e)
                }
            },
        },
    }
}

pub fn expand(
    arena: &mut ETermArena,
    items: &Vec<I>,
    map: &Vec<Binding>,
    sko: &Vec<Binding>,
) -> (out: Result<Vec<Body>, T>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
        map_valid(old(arena).nodes@, map@),
        map_valid(old(arena).nodes@, sko@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        bodies_result_valid(final(arena).nodes@, &out),
        bodies_result(out) == spec::expand(item_models(items@), map_model(map@), map_model(sko@)),
    decreases items@, 1int,
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
        assert_seqs_equal!(body_models(out@) == Seq::<BodyItem>::empty());
        match spec::expand(item_models(items@), map_model(map@), map_model(sko@)) {
            Ok(_) => {},
            Err(_) => {},
        }
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            items_valid(arena.nodes@, items@),
            map_valid(arena.nodes@, map@),
            map_valid(arena.nodes@, sko@),
            bodies_valid(arena.nodes@, out@),
            i <= items.len(),
            spec::expand(item_models(items@), map_model(map@), map_model(sko@)) == body_prefix(
                body_models(out@),
                spec::expand(item_models(items@).skip(i as int), map_model(map@), map_model(sko@)),
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::expand, 1);
        }
        let ghost middle = arena.nodes@;
        let next = expand_item(arena, &items[i], map, sko);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            items_prefix(middle, arena.nodes@, items@);
            map_prefix(middle, arena.nodes@, map@);
            map_prefix(middle, arena.nodes@, sko@);
            bodies_prefix(middle, arena.nodes@, out@);
        }
        match next {
            Err(e) => return Err(e),
            Ok(bs) => {
                proof {
                    body_prefix_assoc(
                        body_models(out@),
                        body_models(bs@),
                        spec::expand(
                            item_models(items@).skip(i as int + 1),
                            map_model(map@),
                            map_model(sko@),
                        ),
                    );
                }
                out = join_bodies(arena, &out, &bs);
            },
        }
        i += 1;
    }
    proof {
        reveal(spec::expand);
    }
    Ok(out)
}

} // verus!
