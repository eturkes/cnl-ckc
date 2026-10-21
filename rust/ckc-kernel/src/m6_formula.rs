#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_bind::{bind_all, push_binding, witness_pairs};
use crate::m6_drs::{anchor, carrier};
#[cfg(verus_keep_ghost)]
use crate::m6_drs::{child_size, int_value, sizes_concat};
use crate::m6_flat::Env;
use crate::m6_model::*;
use crate::m6_numeral::*;
use crate::m6_refs::contains;
use crate::m6_symbols::Sym;
use crate::m6_term::*;
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::arg;
use ckc_spec::term::Term;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

// The formula pass of contract da2 (D1, D3–D5): the exec mirror of `emit.rs`
// § dose arithmetic formulas.
proof fn arg_at(t: Term, xs: Seq<T>, k: int)
    requires
        models(xs) == ckc_spec::engine::args_of(t),
        0 <= k < xs.len(),
    ensures
        xs[k]@ == arg(t, k),
{
    assert(models(xs)[k] == xs[k]@);
    assert(ckc_spec::engine::args_of(t).len() == xs.len());
    match t {
        Term::Comp(_, a) => {
            assert(ckc_spec::engine::args_of(t) == a);
            assert(arg(t, k) == a[k]);
        },
        _ => {
            assert(ckc_spec::engine::args_of(t).len() == 0);
        },
    }
}

proof fn top_object_shape(items: Seq<spec::Item>, y: Term)
    ensures
        spec::top_object(items, y) matches Some(o) ==> spec::is_comp(o, "object"@, 6) && arg(o, 0)
            == y,
    decreases items.len(),
{
    reveal_with_fuel(spec::top_object, 1);
    if items.len() > 0 {
        top_object_shape(items.drop_first(), y);
    }
}

proof fn operand_item_shape(items: Seq<spec::Item>, r: Term)
    ensures
        spec::operand_item(items, r) matches Some(c) ==> (spec::is_comp(c, "object"@, 6)
            || spec::is_comp(c, "$guideline_bound"@, 8)) && arg(c, 0) == r,
    decreases items.len(),
{
    reveal_with_fuel(spec::operand_item, 1);
    if items.len() > 0 {
        operand_item_shape(items.drop_first(), r);
    }
}

pub struct EAnch {
    pub inner: T,
    pub k: T,
}

pub open spec fn anch_models(v: Seq<EAnch>) -> Seq<(Term, int)> {
    v.map_values(|a: EAnch| (a.inner@, int_value(a.k@)))
}

pub open spec fn anchs_valid(nodes: Seq<ENode>, v: Seq<EAnch>) -> bool {
    forall|i: int|
        0 <= i < v.len() ==> valid(nodes, &(#[trigger] v[i]).inner) && valid(nodes, &v[i].k)
            && v[i].k@ is Int
}

pub proof fn anchs_prefix(before: Seq<ENode>, after: Seq<ENode>, an: Seq<EAnch>)
    requires
        before.is_prefix_of(after),
        anchs_valid(before, an),
    ensures
        anchs_valid(after, an),
{
    assert forall|x: int| 0 <= x < an.len() implies valid(after, &(#[trigger] an[x]).inner)
        && valid(after, &an[x].k) && an[x].k@ is Int by {
        prefix(before, after, &an[x].inner);
        prefix(before, after, &an[x].k);
    }
}

proof fn anchored_concat(a: Seq<Term>, b: Seq<Term>)
    ensures
        spec::anchored_all(a + b) == spec::anchored_all(a) + spec::anchored_all(b),
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b).drop_first() == a.drop_first() + b);
        anchored_concat(a.drop_first(), b);
    }
    reveal_with_fuel(spec::anchored_all, 2);
}

// Anchored conditions of `ts`, through carriers, in preorder.
pub fn anchored(arena: &ETermArena, ts: &Vec<T>) -> (out: Vec<EAnch>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ts@),
    ensures
        anchs_valid(arena.nodes@, out@),
        anch_models(out@) == spec::anchored_all(models(ts@)),
{
    let mut todo = concat(ts, &Vec::new());
    let mut out: Vec<EAnch> = Vec::new();
    proof {
        assert(todo@ == ts@ + Seq::<T>::empty());
        assert(todo@ == ts@);
    }
    while todo.len() > 0
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, todo@),
            anchs_valid(arena.nodes@, out@),
            anch_models(out@) + spec::anchored_all(models(todo@)) == spec::anchored_all(
                models(ts@),
            ),
        decreases crate::k2_engine::terms_size(models(todo@)),
    {
        let ghost before = todo@;
        let current = todo.remove(0);
        proof {
            assert(todo@ == before.drop_first());
            assert(models(before)[0] == current@);
            assert(models(before).drop_first() == models(todo@));
            reveal_with_fuel(spec::anchored_all, 1);
            reveal_with_fuel(crate::k2_engine::terms_size, 1);
            assert(crate::k2_engine::terms_size(models(before)) == crate::k2_engine::term_size(
                current@,
            ) + crate::k2_engine::terms_size(models(todo@)));
            assert(spec::anchored_all(models(before)) == spec::anchored(current@)
                + spec::anchored_all(models(todo@)));
            assert(anch_models(out@) + spec::anchored(current@) + spec::anchored_all(models(todo@))
                == spec::anchored_all(models(ts@)));
        }
        match anchor(arena, &current) {
            Some(_) => {
                let a = args(arena, &current);
                let b = args(arena, &a[1]);
                let mut children = Vec::new();
                children.push(a[0].cp());
                let ghost previous = out@;
                let entry = EAnch { inner: a[0].cp(), k: b[1].cp() };
                proof {
                    arg_at(current@, a@, 0);
                    arg_at(current@, a@, 1);
                    arg_at(a@[1]@, b@, 1);
                    assert(b@[1]@ is Int);
                }
                out.push(entry);
                proof {
                    assert(models(children@) == seq![a[0]@]);
                    assert_seqs_equal!(anch_models(out@) == anch_models(previous).push((a[0]@, int_value(b[1]@))));
                    assert forall|j: int| 0 <= j < out@.len() implies valid(
                        arena.nodes@,
                        &(#[trigger] out@[j]).inner,
                    ) && valid(arena.nodes@, &out@[j].k) && out@[j].k@ is Int by {
                        if j < previous.len() {
                            assert(out@[j] == previous[j]);
                        }
                    }
                    anchored_concat(models(children@), models(todo@));
                    sizes_concat(models(children@), models(todo@));
                    child_size(current@, 0);
                    reveal_with_fuel(spec::anchored, 1);
                    reveal_with_fuel(spec::anchored_all, 2);
                    reveal_with_fuel(crate::k2_engine::terms_size, 2);
                    assert(spec::anchored(current@) == seq![(a[0]@, int_value(b[1]@))]
                        + spec::anchored(a[0]@));
                    assert(spec::anchored_all(models(children@)) == spec::anchored(a[0]@));
                    assert(anch_models(out@) + spec::anchored_all(models(children@) + models(todo@))
                        == spec::anchored_all(models(ts@)));
                }
                todo = concat(&children, &todo);
            },
            None => {
                match parts(arena, &current) {
                    Some((name, children)) => {
                        proof {
                            assert(current@ == Term::Comp(name@, models(children@)));
                        }
                        if carrier(&name, children.len()) {
                            proof {
                                anchored_concat(models(children@), models(todo@));
                                sizes_concat(models(children@), models(todo@));
                                reveal_with_fuel(spec::anchored, 1);
                                reveal(crate::k2_engine::term_size);
                                assert(spec::anchored(current@) == spec::anchored_all(
                                    models(children@),
                                ));
                                assert(anch_models(out@) + spec::anchored_all(
                                    models(children@) + models(todo@),
                                ) == spec::anchored_all(models(ts@)));
                            }
                            todo = concat(&children, &todo);
                        } else {
                            proof {
                                reveal_with_fuel(spec::anchored, 1);
                                reveal(crate::k2_engine::term_size);
                                assert(spec::anchored(current@) == Seq::<(Term, int)>::empty());
                                assert(anch_models(out@) + spec::anchored_all(models(todo@))
                                    == spec::anchored_all(models(ts@)));
                            }
                        }
                    },
                    None => {
                        proof {
                            reveal_with_fuel(spec::anchored, 1);
                            reveal(crate::k2_engine::term_size);
                            assert(spec::anchored(current@) == Seq::<(Term, int)>::empty());
                            assert(anch_models(out@) + spec::anchored_all(models(todo@))
                                == spec::anchored_all(models(ts@)));
                        }
                    },
                }
            },
        }
    }
    proof {
        reveal(spec::anchored_all);
        assert(models(todo@) == Seq::<Term>::empty());
        assert(anch_models(out@) + Seq::<(Term, int)>::empty() == anch_models(out@));
    }
    out
}

// Whether the term k is the integer v.
fn int_is(arena: &ETermArena, k: &T, v: usize) -> (r: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, k),
    ensures
        r == (k@ == Term::Int(v as int)),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, k.root as int));
        reveal(crate::k2_term::node_ok);
    }
    match &arena.nodes[k.root].kind {
        crate::k2_term::ENodeKind::Int { magnitude, negative, value, .. } => {
            if *negative {
                return false;
            }
            let target = crate::k2_manifest::udec_vec(v);
            let r = bytes_eq(magnitude.as_slice(), target.as_slice());
            proof {
                crate::k4_numbers::decimal_print_inverse(value@ as nat);
                crate::k4_numbers::decimal_print_inverse(v as nat);
            }
            r
        },
        _ => false,
    }
}

// The index j < n with k = j + off, if one exists.
fn index_of(arena: &ETermArena, k: &T, n: usize, off: usize) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, k),
        k@ is Int,
        off <= 1,
    ensures
        match out {
            Some(j) => j < n && int_value(k@) == j + off,
            None => !(off <= int_value(k@) < n + off),
        },
{
    let mut j = 0usize;
    while j < n
        invariant
            arena_ok(arena),
            valid(arena.nodes@, k),
            k@ is Int,
            off <= 1,
            j <= n,
            !(off as int <= int_value(k@) < j as int + off as int),
        decreases n - j,
    {
        if int_is(arena, k, j + off) {
            return Some(j);
        }
        j += 1;
    }
    None
}

pub open spec fn option_model(o: Option<T>) -> Option<Term> {
    match o {
        Some(t) => Some(t@),
        None => None,
    }
}

pub open spec fn opt_k(o: Option<T>) -> Option<int> {
    match o {
        Some(k) => Some(int_value(k@)),
        None => None,
    }
}

// The anchor term of r's object, if any.
pub fn object_token(arena: &ETermArena, an: &Vec<EAnch>, r: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        anchs_valid(arena.nodes@, an@),
        valid(arena.nodes@, r),
    ensures
        out matches Some(k) ==> valid(arena.nodes@, &k) && k@ is Int,
        opt_k(out) == spec::object_token(anch_models(an@), r@),
{
    let mut i = 0usize;
    proof {
        assert(anch_models(an@).skip(0) == anch_models(an@));
    }
    while i < an.len()
        invariant
            arena_ok(arena),
            anchs_valid(arena.nodes@, an@),
            valid(arena.nodes@, r),
            i <= an.len(),
            spec::object_token(anch_models(an@), r@) == spec::object_token(
                anch_models(an@).skip(i as int),
                r@,
            ),
        decreases an.len() - i,
    {
        proof {
            assert(anch_models(an@).skip(i as int).drop_first() == anch_models(an@).skip(
                i as int + 1,
            ));
            assert(anch_models(an@).skip(i as int)[0] == (
                an@[i as int].inner@,
                int_value(an@[i as int].k@),
            ));
            reveal_with_fuel(spec::object_token, 1);
        }
        let inner = &an[i].inner;
        if is_comp(arena, inner, &Sym::Object, 6) {
            let xs = args(arena, inner);
            if crate::m6_term::equal(arena, &xs[0], r) {
                proof {
                    arg_at(inner@, xs@, 0);
                }
                return Some(an[i].k.cp());
            }
            proof {
                arg_at(inner@, xs@, 0);
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::object_token, 1);
    }
    None
}

// The anchor term of the formula f, if any.
pub fn formula_token(arena: &ETermArena, an: &Vec<EAnch>, f: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        anchs_valid(arena.nodes@, an@),
        valid(arena.nodes@, f),
    ensures
        out matches Some(k) ==> valid(arena.nodes@, &k) && k@ is Int,
        opt_k(out) == spec::formula_token(anch_models(an@), f@),
{
    let mut i = 0usize;
    proof {
        assert(anch_models(an@).skip(0) == anch_models(an@));
    }
    while i < an.len()
        invariant
            arena_ok(arena),
            anchs_valid(arena.nodes@, an@),
            valid(arena.nodes@, f),
            i <= an.len(),
            spec::formula_token(anch_models(an@), f@) == spec::formula_token(
                anch_models(an@).skip(i as int),
                f@,
            ),
        decreases an.len() - i,
    {
        proof {
            assert(anch_models(an@).skip(i as int).drop_first() == anch_models(an@).skip(
                i as int + 1,
            ));
            assert(anch_models(an@).skip(i as int)[0] == (
                an@[i as int].inner@,
                int_value(an@[i as int].k@),
            ));
            reveal_with_fuel(spec::formula_token, 1);
        }
        if crate::m6_term::equal(arena, &an[i].inner, f) {
            return Some(an[i].k.cp());
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::formula_token, 1);
    }
    None
}

pub fn some_before(arena: &ETermArena, src: &ESrc, an: &Vec<EAnch>, r: &T) -> (out: bool)
    requires
        arena_ok(arena),
        src_valid(arena.nodes@, src),
        anchs_valid(arena.nodes@, an@),
        valid(arena.nodes@, r),
    ensures
        out == spec::some_before(src@, anch_models(an@), r@),
{
    match object_token(arena, an, r) {
        None => false,
        Some(k) => match index_of(arena, &k, src.toks.len(), 1) {
            None => false,
            Some(j) => {
                proof {
                    assert(valid(arena.nodes@, &src.toks@[j as int]));
                    assert(models(src.toks@)[j as int] == src.toks@[j as int]@);
                    reveal_strlit("some");
                }
                is_atom(arena, &src.toks[j], &Sym::SomeWord)
            },
        },
    }
}

// The top-level object on y, if any.
pub fn top_object(arena: &ETermArena, items: &Vec<I>, y: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, y),
    ensures
        out matches Some(o) ==> valid(arena.nodes@, &o),
        option_model(out) == spec::top_object(item_models(items@), y@),
{
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid(arena.nodes@, y),
            i <= items.len(),
            spec::top_object(item_models(items@), y@) == spec::top_object(
                item_models(items@).skip(i as int),
                y@,
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::top_object, 1);
        }
        match &items[i].kind {
            Kind::Anch(c, inner) => {
                if is_atom(arena, c, &Sym::Actual) && is_comp(arena, inner, &Sym::Object, 6) {
                    let xs = args(arena, inner);
                    proof {
                        arg_at(inner@, xs@, 0);
                        reveal_strlit("actual");
                    }
                    if crate::m6_term::equal(arena, &xs[0], y) {
                        return Some(inner.cp());
                    }
                }
                proof {
                    reveal_strlit("actual");
                }
            },
            _ => {},
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::top_object, 1);
    }
    None
}

// The top-level antecedent object or nt1 bound item on r, if any.
pub fn operand_item(arena: &ETermArena, items: &Vec<I>, r: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, r),
    ensures
        out matches Some(o) ==> valid(arena.nodes@, &o),
        option_model(out) == spec::operand_item(item_models(items@), r@),
{
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid(arena.nodes@, r),
            i <= items.len(),
            spec::operand_item(item_models(items@), r@) == spec::operand_item(
                item_models(items@).skip(i as int),
                r@,
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::operand_item, 1);
            reveal_strlit("actual");
        }
        match &items[i].kind {
            Kind::Anch(c, inner) => {
                if is_atom(arena, c, &Sym::Actual) && (is_comp(arena, inner, &Sym::Object, 6)
                    || is_comp(arena, inner, &Sym::DollarGuidelineBound, 8)) {
                    let xs = args(arena, inner);
                    proof {
                        arg_at(inner@, xs@, 0);
                    }
                    if crate::m6_term::equal(arena, &xs[0], r) {
                        return Some(inner.cp());
                    }
                }
            },
            _ => {},
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::operand_item, 1);
    }
    None
}

pub struct EPair {
    pub key: T,
    pub s: T,
    pub n: T,
}

pub open spec fn pair_models(ps: Seq<EPair>) -> Seq<(Term, (Term, Term))> {
    ps.map_values(|p: EPair| (p.key@, (p.s@, p.n@)))
}

pub open spec fn pairs_valid(nodes: Seq<ENode>, ps: Seq<EPair>) -> bool {
    forall|i: int|
        0 <= i < ps.len() ==> valid(nodes, &(#[trigger] ps[i]).key) && valid(nodes, &ps[i].s)
            && valid(nodes, &ps[i].n)
}

fn shape(arena: &mut ETermArena, why: &Sym) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(
            ckc_spec::v1text::ascii("formula_shape"@),
            seq![Term::Atom(crate::m6_symbols::symbol(why))],
        ),
{
    let w = named(arena, why);
    let ghost mid = arena.nodes@;
    let out = c1(arena, &Sym::FormulaShape, &w);
    proof {
        reveal_strlit("formula_shape");
    }
    out
}

// D1's operand law on a leaf referent (`spec::operand_ok`).
fn operand_ok(arena: &ETermArena, items: &Vec<I>, r: &T) -> (out: bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid(arena.nodes@, r),
    ensures
        out == spec::operand_ok(item_models(items@), r@),
{
    proof {
        operand_item_shape(item_models(items@), r@);
        reveal(spec::operand_ok);
        reveal_strlit("countable");
        reveal_strlit("na");
    }
    match operand_item(arena, items, r) {
        None => false,
        Some(c) => {
            let xs = args(arena, &c);
            proof {
                assert(xs@.len() == 6 || xs@.len() == 8);
                arg_at(c@, xs@, 2);
                arg_at(c@, xs@, 3);
            }
            is_var(arena, r) && is_atom(arena, &xs[2], &Sym::Countable) && is_atom(
                arena,
                &xs[3],
                &Sym::Na,
            ) && crate::m6_compare_refs::unit_of(arena, items, r).is_none()
        },
    }
}

// The (S, N) pairs of the operand leaves, else the reject: every leaf's
// operand law before any leaf's bound (contract da2 D5).
pub fn operand_pairs(arena: &mut ETermArena, items: &Vec<I>, rs: &Vec<T>) -> (out: Result<
    Vec<EPair>,
    T,
>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
        valid_all(old(arena).nodes@, rs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match out {
            Ok(ps) => pairs_valid(final(arena).nodes@, ps@) && Ok::<
                Seq<(Term, (Term, Term))>,
                Term,
            >(pair_models(ps@)) == spec::operand_pairs(item_models(items@), models(rs@)),
            Err(e) => valid(final(arena).nodes@, &e) && Err::<Seq<(Term, (Term, Term))>, Term>(e@)
                == spec::operand_pairs(item_models(items@), models(rs@)),
        },
{
    let mut i = 0usize;
    proof {
        assert(models(rs@).skip(0) == models(rs@));
    }
    while i < rs.len()
        invariant
            arena_ok(arena),
            arena.nodes@ == old(arena).nodes@,
            items_valid(arena.nodes@, items@),
            valid_all(arena.nodes@, rs@),
            i <= rs.len(),
            spec::operands_ok(item_models(items@), models(rs@)) == spec::operands_ok(
                item_models(items@),
                models(rs@).skip(i as int),
            ),
        decreases rs.len() - i,
    {
        proof {
            assert(models(rs@).skip(i as int).drop_first() == models(rs@).skip(i as int + 1));
            assert(models(rs@).skip(i as int)[0] == rs@[i as int]@);
            assert(valid(arena.nodes@, &rs@[i as int]));
            reveal_with_fuel(spec::operands_ok, 1);
        }
        if !operand_ok(arena, items, &rs[i]) {
            let e = shape(arena, &Sym::Operand);
            proof {
                reveal(spec::operand_pairs);
                reveal(spec::fshape);
                reveal_strlit("operand");
            }
            return Err(e);
        }
        i += 1;
    }
    proof {
        reveal(spec::operand_pairs);
        reveal_with_fuel(spec::operands_ok, 1);
        assert(models(rs@).skip(i as int).len() == 0);
    }
    bound_pairs(arena, items, rs)
}

fn bound_pairs(arena: &mut ETermArena, items: &Vec<I>, rs: &Vec<T>) -> (out: Result<Vec<EPair>, T>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
        valid_all(old(arena).nodes@, rs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match out {
            Ok(ps) => pairs_valid(final(arena).nodes@, ps@) && Ok::<
                Seq<(Term, (Term, Term))>,
                Term,
            >(pair_models(ps@)) == spec::bound_pairs(item_models(items@), models(rs@)),
            Err(e) => valid(final(arena).nodes@, &e) && Err::<Seq<(Term, (Term, Term))>, Term>(e@)
                == spec::bound_pairs(item_models(items@), models(rs@)),
        },
{
    let ghost start = arena.nodes@;
    let mut out: Vec<EPair> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(models(rs@).skip(0) == models(rs@));
    }
    proof {
        assert_seqs_equal!(pair_models(out@) == Seq::<(Term, (Term, Term))>::empty());
    }
    while i < rs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            items_valid(arena.nodes@, items@),
            valid_all(arena.nodes@, rs@),
            i <= rs.len(),
            pairs_valid(arena.nodes@, out@),
            spec::bound_pairs(item_models(items@), models(rs@)) == match spec::bound_pairs(
                item_models(items@),
                models(rs@).skip(i as int),
            ) {
                Ok(rest) => Ok(pair_models(out@) + rest),
                Err(e) => Err(e),
            },
        decreases rs.len() - i,
    {
        proof {
            assert(models(rs@).skip(i as int).drop_first() == models(rs@).skip(i as int + 1));
            assert(models(rs@).skip(i as int)[0] == rs@[i as int]@);
            assert(valid(arena.nodes@, &rs@[i as int]));
            reveal_with_fuel(spec::bound_pairs, 1);
            reveal(spec::operand_pair);
            reveal(spec::fshape);
            reveal_strlit("operand");
            reveal_strlit("operand_bound");
        }
        let r = &rs[i];
        proof {
            operand_item_shape(item_models(items@), r@);
        }
        let ghost before = arena.nodes@;
        let pair = match operand_item(arena, items, r) {
            None => {
                let e = shape(arena, &Sym::Operand);
                proof {
                    crate::k2_load::prefix_chain(start, before, arena.nodes@);
                }
                return Err(e);
            },
            Some(c) => {
                let xs = args(arena, &c);
                proof {
                    assert(xs@.len() == 6 || xs@.len() == 8);
                }
                if is_comp(arena, &c, &Sym::DollarGuidelineBound, 8) {
                    proof {
                        arg_at(c@, xs@, 4);
                        arg_at(c@, xs@, 5);
                        assert(spec::operand_pair(item_models(items@), r@) == Ok::<
                            (Term, Term),
                            Term,
                        >((xs@[4]@, xs@[5]@)));
                    }
                    EPair { key: r.cp(), s: xs[4].cp(), n: xs[5].cp() }
                } else {
                    let e = shape(arena, &Sym::OperandBound);
                    proof {
                        crate::k2_load::prefix_chain(start, before, arena.nodes@);
                    }
                    return Err(e);
                }
            },
        };
        let ghost before = out@;
        let ghost p = (pair.s@, pair.n@);
        out.push(pair);
        proof {
            assert_seqs_equal!(pair_models(out@) == pair_models(before) + seq![(rs@[i as int]@, p)]);
            assert forall|x: int| 0 <= x < out.len() implies valid(
                arena.nodes@,
                &(#[trigger] out@[x]).key,
            ) && valid(arena.nodes@, &out@[x].s) && valid(arena.nodes@, &out@[x].n) by {
                if x < before.len() {
                    assert(out@[x] == before[x]);
                }
            }
            reveal_with_fuel(spec::bound_pairs, 1);
            let rest_i = models(rs@).skip(i as int);
            let rest_n = models(rs@).skip(i as int + 1);
            assert(rest_i.len() > 0 && rest_i[0] == rs@[i as int]@ && rest_i.drop_first()
                == rest_n);
            match spec::bound_pairs(item_models(items@), rest_n) {
                Ok(rest) => {
                    assert(spec::bound_pairs(item_models(items@), rest_i) == Ok::<
                        Seq<(Term, (Term, Term))>,
                        Term,
                    >(seq![(rs@[i as int]@, p)] + rest));
                    assert_seqs_equal!(pair_models(before) + (seq![(rs@[i as int]@, p)] + rest)
                        == pair_models(out@) + rest);
                },
                Err(e) => {
                    assert(spec::bound_pairs(item_models(items@), rest_i) == Err::<
                        Seq<(Term, (Term, Term))>,
                        Term,
                    >(e));
                },
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::bound_pairs, 1);
        assert_seqs_equal!(pair_models(out@) + Seq::<(Term, (Term, Term))>::empty() == pair_models(out@));
    }
    Ok(out)
}

pub open spec fn has_key(ps: Seq<EPair>, t: Term) -> bool {
    exists|i: int| 0 <= i < ps.len() && (#[trigger] ps[i]).key@ == t
}

pub fn pair_of(arena: &ETermArena, ps: &Vec<EPair>, e: &T) -> (out: (T, T))
    requires
        arena_ok(arena),
        pairs_valid(arena.nodes@, ps@),
        valid(arena.nodes@, e),
        has_key(ps@, e@),
    ensures
        valid(arena.nodes@, &out.0),
        valid(arena.nodes@, &out.1),
        (out.0@, out.1@) == spec::pair_of(pair_models(ps@), e@),
{
    let mut i = 0usize;
    proof {
        assert(pair_models(ps@).skip(0) == pair_models(ps@));
    }
    while i < ps.len()
        invariant
            arena_ok(arena),
            pairs_valid(arena.nodes@, ps@),
            valid(arena.nodes@, e),
            i <= ps.len(),
            spec::pair_of(pair_models(ps@), e@) == spec::pair_of(
                pair_models(ps@).skip(i as int),
                e@,
            ),
            forall|x: int| 0 <= x < i ==> (#[trigger] ps@[x]).key@ != e@,
        decreases ps.len() - i,
    {
        proof {
            assert(pair_models(ps@).skip(i as int).drop_first() == pair_models(ps@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::pair_of, 1);
        }
        if crate::m6_term::equal(arena, &ps[i].key, e) {
            return (ps[i].s.cp(), ps[i].n.cp());
        }
        i += 1;
    }
    proof {
        let i = choose|i: int| 0 <= i < ps.len() && (#[trigger] ps@[i]).key@ == e@;
        assert(ps@[i].key@ != e@);
    }
    (e.cp(), e.cp())
}

proof fn models_concat(a: Seq<T>, b: Seq<T>)
    ensures
        models(a + b) == models(a) + models(b),
{
    assert_seqs_equal!(models(a + b) == models(a) + models(b));
}

proof fn valid_concat(nodes: Seq<ENode>, a: Seq<T>, b: Seq<T>)
    requires
        valid_all(nodes, a),
        valid_all(nodes, b),
    ensures
        valid_all(nodes, a + b),
{
    assert forall|i: int| 0 <= i < (a + b).len() implies valid(nodes, #[trigger] &(a + b)[i]) by {
        if i < a.len() {
            assert((a + b)[i] == a[i]);
        } else {
            assert((a + b)[i] == b[i - a.len()]);
        }
    }
}

pub fn var_leaves(arena: &ETermArena, e: &T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, e),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::var_leaves(e@),
    decreases crate::k2_engine::term_size(e@),
{
    if is_comp(arena, e, &Sym::Expr, 3) {
        let xs = args(arena, e);
        proof {
            child_size(e@, 1);
            child_size(e@, 2);
            assert(valid(arena.nodes@, &xs@[1]));
            assert(valid(arena.nodes@, &xs@[2]));
            reveal_strlit("expr");
        }
        let l = var_leaves(arena, &xs[1]);
        let r = var_leaves(arena, &xs[2]);
        proof {
            models_concat(l@, r@);
            valid_concat(arena.nodes@, l@, r@);
            reveal_with_fuel(spec::var_leaves, 1);
        }
        concat(&l, &r)
    } else {
        proof {
            reveal_with_fuel(spec::var_leaves, 1);
            reveal_strlit("expr");
        }
        let mut out = Vec::new();
        if is_var(arena, e) {
            out.push(e.cp());
        }
        proof {
            assert_seqs_equal!(models(out@) == spec::var_leaves(e@));
        }
        out
    }
}

pub fn is_literal(arena: &ETermArena, e: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, e),
    ensures
        out == spec::is_literal(e@),
{
    proof {
        reveal_strlit("int");
        reveal_strlit("real");
    }
    is_comp(arena, e, &Sym::Int, 1) || is_comp(arena, e, &Sym::Real, 1)
}

pub fn lit_leaves(arena: &ETermArena, e: &T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, e),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::lit_leaves(e@),
    decreases crate::k2_engine::term_size(e@),
{
    if is_comp(arena, e, &Sym::Expr, 3) {
        let xs = args(arena, e);
        proof {
            child_size(e@, 1);
            child_size(e@, 2);
            assert(valid(arena.nodes@, &xs@[1]));
            assert(valid(arena.nodes@, &xs@[2]));
            reveal_strlit("expr");
        }
        let l = lit_leaves(arena, &xs[1]);
        let r = lit_leaves(arena, &xs[2]);
        proof {
            models_concat(l@, r@);
            valid_concat(arena.nodes@, l@, r@);
            reveal_with_fuel(spec::lit_leaves, 1);
        }
        concat(&l, &r)
    } else {
        proof {
            reveal_with_fuel(spec::lit_leaves, 1);
            reveal_strlit("expr");
        }
        let mut out = Vec::new();
        if is_literal(arena, e) {
            out.push(e.cp());
        }
        proof {
            assert_seqs_equal!(models(out@) == spec::lit_leaves(e@));
        }
        out
    }
}

pub fn ops_ok(arena: &ETermArena, e: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, e),
    ensures
        out == spec::ops_ok(e@),
    decreases crate::k2_engine::term_size(e@),
{
    proof {
        reveal_with_fuel(spec::ops_ok, 1);
        reveal_strlit("expr");
        reveal_strlit("*");
        reveal_strlit("/");
    }
    if is_comp(arena, e, &Sym::Expr, 3) {
        let xs = args(arena, e);
        proof {
            child_size(e@, 1);
            child_size(e@, 2);
            assert(valid(arena.nodes@, &xs@[0]));
            assert(valid(arena.nodes@, &xs@[1]));
            assert(valid(arena.nodes@, &xs@[2]));
        }
        (is_atom(arena, &xs[0], &Sym::Star) || is_atom(arena, &xs[0], &Sym::Slash)) && ops_ok(
            arena,
            &xs[1],
        ) && ops_ok(arena, &xs[2])
    } else if is_var(arena, e) {
        true
    } else {
        match parts(arena, e) {
            Some(_) => is_literal(arena, e),
            None => false,
        }
    }
}

pub struct EChain {
    pub goals: Vec<T>,
    pub out_s: T,
    pub out_n: T,
    pub v: usize,
    pub m: usize,
}

pub open spec fn chain_model(c: &EChain) -> spec::Chain {
    spec::Chain { goals: models(c.goals@), out: (c.out_s@, c.out_n@), v: c.v as nat, m: c.m as nat }
}

pub open spec fn chain_valid(nodes: Seq<ENode>, c: &EChain) -> bool {
    valid_all(nodes, c.goals@) && valid(nodes, &c.out_s) && valid(nodes, &c.out_n)
}

// The postorder chain of E (contract da2 D3).
pub fn chain(
    arena: &mut ETermArena,
    e: &T,
    ps: &Vec<EPair>,
    vals: &Vec<T>,
    v: usize,
    m: usize,
) -> (out: EChain)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, e),
        pairs_valid(old(arena).nodes@, ps@),
        valid_all(old(arena).nodes@, vals@),
        spec::ops_ok(e@),
        forall|x: int|
            0 <= x < spec::var_leaves(e@).len() ==> has_key(
                ps@,
                #[trigger] spec::var_leaves(e@)[x],
            ),
        m + spec::lit_leaves(e@).len() <= vals.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        chain_valid(final(arena).nodes@, &out),
        chain_model(&out) == spec::chain(e@, pair_models(ps@), models(vals@), v as nat, m as nat),
        out.m == m + spec::lit_leaves(e@).len(),
    decreases crate::k2_engine::term_size(e@),
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::chain, 1);
        reveal_with_fuel(spec::ops_ok, 1);
        reveal_with_fuel(spec::var_leaves, 1);
        reveal_with_fuel(spec::lit_leaves, 1);
        reveal_strlit("expr");
        reveal_strlit("*");
        reveal_strlit("exactly");
    }
    if is_comp(arena, e, &Sym::Expr, 3) {
        let xs = args(arena, e);
        proof {
            child_size(e@, 1);
            child_size(e@, 2);
            assert(valid(arena.nodes@, &xs@[0]));
            assert(valid(arena.nodes@, &xs@[1]));
            assert(valid(arena.nodes@, &xs@[2]));
            assert forall|x: int| 0 <= x < spec::var_leaves(xs@[1]@).len() implies has_key(
                ps@,
                #[trigger] spec::var_leaves(xs@[1]@)[x],
            ) by {
                assert(spec::var_leaves(e@)[x] == spec::var_leaves(xs@[1]@)[x]);
            }
            assert forall|x: int| 0 <= x < spec::var_leaves(xs@[2]@).len() implies has_key(
                ps@,
                #[trigger] spec::var_leaves(xs@[2]@)[x],
            ) by {
                assert(spec::var_leaves(e@)[spec::var_leaves(xs@[1]@).len() + x]
                    == spec::var_leaves(xs@[2]@)[x]);
            }
        }
        let l = chain(arena, &xs[1], ps, vals, v, m);
        let ghost n1 = arena.nodes@;
        proof {
            prefix(start, n1, &xs[2]);
            assert forall|i: int| 0 <= i < ps.len() implies valid(n1, &(#[trigger] ps@[i]).key)
                && valid(n1, &ps@[i].s) && valid(n1, &ps@[i].n) by {
                prefix(start, n1, &ps@[i].key);
                prefix(start, n1, &ps@[i].s);
                prefix(start, n1, &ps@[i].n);
            }
            prefix_all(start, n1, vals@);
        }
        let r = chain(arena, &xs[2], ps, vals, l.v, l.m);
        let ghost n2 = arena.nodes@;
        let sv = var(arena, r.v);
        let v1 = successor(arena, r.v);
        let nv = var(arena, v1);
        let v2 = successor(arena, v1);
        let ghost n3 = arena.nodes@;
        proof {
            crate::k2_load::prefix_chain(start, n1, n2);
            crate::k2_load::prefix_chain(n1, n2, n3);
            prefix_all(n1, n3, l.goals@);
            prefix(n1, n3, &l.out_s);
            prefix(n1, n3, &l.out_n);
            prefix_all(n2, n3, r.goals@);
            prefix(n2, n3, &r.out_s);
            prefix(n2, n3, &r.out_n);
        }
        let mut gargs: Vec<T> = Vec::new();
        gargs.push(l.out_s.cp());
        gargs.push(l.out_n.cp());
        gargs.push(r.out_s.cp());
        gargs.push(r.out_n.cp());
        gargs.push(sv.cp());
        gargs.push(nv.cp());
        proof {
            prefix(start, arena.nodes@, &xs@[0]);
        }
        let star = is_atom(arena, &xs[0], &Sym::Star);
        let goal = if star {
            c(arena, &Sym::GuidelineProduct, &gargs)
        } else {
            c(arena, &Sym::GuidelineQuotient, &gargs)
        };
        let ghost n4 = arena.nodes@;
        proof {
            crate::k2_load::prefix_chain(n1, n3, n4);
            crate::k2_load::prefix_chain(n2, n3, n4);
            prefix_all(n3, n4, l.goals@);
            prefix_all(n3, n4, r.goals@);
            prefix(n3, n4, &sv);
            prefix(n3, n4, &nv);
            assert(models(gargs@) == seq![l.out_s@, l.out_n@, r.out_s@, r.out_n@, sv@, nv@]);
            reveal_strlit("guideline_product");
            reveal_strlit("guideline_quotient");
        }
        let mut one = Vec::new();
        one.push(goal);
        let lr = concat(&l.goals, &r.goals);
        let goals = concat(&lr, &one);
        proof {
            models_concat(l.goals@, r.goals@);
            models_concat(lr@, one@);
            valid_concat(n4, l.goals@, r.goals@);
            valid_concat(n4, lr@, one@);
            assert(models(one@) == seq![goal@]);
            arg_at(e@, xs@, 0);
            arg_at(e@, xs@, 1);
            arg_at(e@, xs@, 2);
            reveal_with_fuel(spec::chain, 1);
            reveal_strlit("expr");
            reveal_strlit("*");
            reveal_strlit("guideline_product");
            reveal_strlit("guideline_quotient");
            let lm = spec::chain(xs@[1]@, pair_models(ps@), models(vals@), v as nat, m as nat);
            let rm = spec::chain(xs@[2]@, pair_models(ps@), models(vals@), lm.v, lm.m);
            assert(chain_model(&l) == lm);
            assert(chain_model(&r) == rm);
            assert(sv@ == Term::Var(rm.v) && nv@ == Term::Var(rm.v + 1));
            assert(models(goals@) == lm.goals + rm.goals + seq![goal@]);
            assert(goal@ == Term::Comp(
                if xs@[0]@ == ckc_spec::replay::atom("*"@) {
                    ckc_spec::v1text::ascii("guideline_product"@)
                } else {
                    ckc_spec::v1text::ascii("guideline_quotient"@)
                },
                seq![lm.out.0, lm.out.1, rm.out.0, rm.out.1, sv@, nv@],
            ));
            assert(spec::chain(e@, pair_models(ps@), models(vals@), v as nat, m as nat)
                == spec::Chain {
                goals: lm.goals + rm.goals + seq![goal@],
                out: (sv@, nv@),
                v: rm.v + 2,
                m: rm.m,
            });
        }
        EChain { goals, out_s: sv, out_n: nv, v: v2, m: r.m }
    } else if is_literal(arena, e) {
        proof {
            assert(valid(arena.nodes@, &vals@[m as int]));
            assert(models(vals@)[m as int] == vals@[m as int]@);
        }
        let ex = named(arena, &Sym::Exactly);
        proof {
            prefix(start, arena.nodes@, &vals@[m as int]);
        }
        let out = EChain { goals: Vec::new(), out_s: ex, out_n: vals[m].cp(), v, m: m + 1 };
        proof {
            reveal_with_fuel(spec::chain, 1);
            reveal_with_fuel(spec::lit_leaves, 1);
            reveal_strlit("exactly");
            assert(models(out.goals@) == Seq::<Term>::empty());
            assert(spec::lit_leaves(e@) == seq![e@]);
        }
        out
    } else {
        proof {
            assert(e@ is Var);
            assert(spec::var_leaves(e@) == seq![e@]);
            assert(spec::var_leaves(e@)[0] == e@);
        }
        let (s0, n0) = pair_of(arena, ps, e);
        let out = EChain { goals: Vec::new(), out_s: s0, out_n: n0, v, m };
        proof {
            reveal_with_fuel(spec::chain, 1);
            reveal_with_fuel(spec::lit_leaves, 1);
            assert(models(out.goals@) == Seq::<Term>::empty());
            assert(spec::lit_leaves(e@) == Seq::<Term>::empty());
        }
        out
    }
}

proof fn subst_all_len(ts: Seq<Term>, x: nat, v: Term)
    ensures
        ckc_spec::engine::subst_all(ts, x, v).len() == ts.len(),
    decreases ts.len(),
{
    reveal_with_fuel(ckc_spec::engine::subst_all, 1);
    if ts.len() > 0 {
        subst_all_len(ts.drop_first(), x, v);
    }
}

proof fn bind_all_comp(t: Term, pairs: Seq<(Term, Term)>)
    requires
        t is Comp,
    ensures
        match (t, spec::bind_all(t, pairs)) {
            (Term::Comp(n, a), Term::Comp(n2, a2)) => n2 == n && a2.len() == a.len(),
            _ => false,
        },
    decreases pairs.len(),
{
    reveal_with_fuel(spec::bind_all, 1);
    if pairs.len() > 0 {
        match pairs[0].0 {
            Term::Var(x) => {
                match t {
                    Term::Comp(n, a) => {
                        subst_all_len(a, x, pairs[0].1);
                    },
                    _ => {},
                }
                bind_all_comp(ckc_spec::engine::subst(t, x, pairs[0].1), pairs.drop_first());
            },
            _ => bind_all_comp(t, pairs.drop_first()),
        }
    }
}

// Witness values of the chain outputs (contract da2 D4), else None.
pub fn chain_pairs(arena: &mut ETermArena, goals: &Vec<T>, pairs: &Vec<Binding>) -> (out: Option<
    Vec<Binding>,
>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, goals@),
        map_valid(old(arena).nodes@, pairs@),
        forall|i: int|
            0 <= i < goals.len() ==> ((#[trigger] goals@[i])@ matches Term::Comp(name, args)
                && ckc_spec::arith::is_arith(name, args.len())),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(ps) ==> map_valid(final(arena).nodes@, ps@),
        match out {
            Some(ps) => spec::chain_pairs(models(goals@), map_model(pairs@)) == Some(
                map_model(ps@),
            ),
            None => spec::chain_pairs(models(goals@), map_model(pairs@)) is None,
        },
{
    let ghost start = arena.nodes@;
    let mut acc = copy_map(arena, pairs);
    let mut i = 0usize;
    proof {
        assert(models(goals@).skip(0) == models(goals@));
    }
    while i < goals.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, goals@),
            map_valid(arena.nodes@, acc@),
            i <= goals.len(),
            forall|x: int|
                0 <= x < goals.len() ==> ((#[trigger] goals@[x])@ matches Term::Comp(name, args)
                    && ckc_spec::arith::is_arith(name, args.len())),
            spec::chain_pairs(models(goals@), map_model(pairs@)) == spec::chain_pairs(
                models(goals@).skip(i as int),
                map_model(acc@),
            ),
        decreases goals.len() - i,
    {
        proof {
            assert(models(goals@).skip(i as int).drop_first() == models(goals@).skip(i as int + 1));
            assert(models(goals@).skip(i as int)[0] == goals@[i as int]@);
            assert(valid(arena.nodes@, &goals@[i as int]));
            reveal_with_fuel(spec::chain_pairs, 1);
        }
        let ghost n0 = arena.nodes@;
        let bound = bind_all(arena, &goals[i], &acc);
        let ghost n1 = arena.nodes@;
        proof {
            crate::k2_load::prefix_chain(start, n0, n1);
            prefix_all(n0, n1, goals@);
            map_prefix(n0, n1, acc@);
            bind_all_comp(goals@[i as int]@, map_model(acc@));
        }
        let (name, bargs) = match parts(arena, &bound) {
            Some(p) => p,
            None => return None,
        };
        let rs = roots(arena, &bargs);
        let r = crate::arith_impl::arith_result_exec(arena, &name, &rs);
        let ghost n2 = arena.nodes@;
        proof {
            crate::k2_load::prefix_chain(start, n1, n2);
            prefix_all(n1, n2, goals@);
            map_prefix(n1, n2, acc@);
        }
        match r {
            None => return None,
            Some((sr, nr)) => {
                let st = from_root(arena, sr);
                let nt = from_root(arena, nr);
                let gs = args(arena, &goals[i]);
                proof {
                    arg_at(goals@[i as int]@, gs@, 4);
                    arg_at(goals@[i as int]@, gs@, 5);
                }
                acc = push_binding(arena, acc, &gs[4], st);
                acc = push_binding(arena, acc, &gs[5], nt);
            },
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::chain_pairs, 1);
    }
    Some(acc)
}

pub fn formula_refs(arena: &ETermArena, fs: &Vec<T>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, fs@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::formula_refs(models(fs@)),
{
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(models(fs@).skip(0) == models(fs@));
    }
    proof {
        assert_seqs_equal!(models(out@) + spec::formula_refs(models(fs@)) == spec::formula_refs(models(fs@)));
    }
    while i < fs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, fs@),
            valid_all(arena.nodes@, out@),
            i <= fs.len(),
            spec::formula_refs(models(fs@)) == models(out@) + spec::formula_refs(
                models(fs@).skip(i as int),
            ),
        decreases fs.len() - i,
    {
        proof {
            assert(models(fs@).skip(i as int).drop_first() == models(fs@).skip(i as int + 1));
            assert(models(fs@).skip(i as int)[0] == fs@[i as int]@);
            assert(valid(arena.nodes@, &fs@[i as int]));
            reveal_with_fuel(spec::formula_refs, 1);
        }
        let xs = args(arena, &fs[i]);
        let vs = if xs.len() > 2 {
            proof {
                assert(valid(arena.nodes@, &xs@[2]));
                arg_at(fs@[i as int]@, xs@, 2);
            }
            var_leaves(arena, &xs[2])
        } else {
            proof {
                reveal_with_fuel(spec::var_leaves, 1);
                assert(arg(fs@[i as int]@, 2) == Term::Nil);
            }
            Vec::new()
        };
        let ghost before = out@;
        proof {
            models_concat(before, vs@);
            valid_concat(arena.nodes@, before, vs@);
            assert((models(before) + models(vs@)) + spec::formula_refs(
                models(fs@).skip(i as int + 1),
            ) == models(before) + (models(vs@) + spec::formula_refs(
                models(fs@).skip(i as int + 1),
            )));
        }
        out = concat(&out, &vs);
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::formula_refs, 1);
        assert_seqs_equal!(models(out@) + Seq::<Term>::empty() == models(out@));
    }
    out
}

pub fn targets(arena: &ETermArena, fs: &Vec<T>, y: &T) -> (out: usize)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, fs@),
        valid(arena.nodes@, y),
    ensures
        out as nat == spec::targets(models(fs@), y@),
{
    let mut n = 0usize;
    let mut i = 0usize;
    proof {
        assert(models(fs@).skip(0) == models(fs@));
    }
    while i < fs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, fs@),
            valid(arena.nodes@, y),
            i <= fs.len(),
            n <= i,
            spec::targets(models(fs@), y@) == n as nat + spec::targets(
                models(fs@).skip(i as int),
                y@,
            ),
        decreases fs.len() - i,
    {
        proof {
            assert(models(fs@).skip(i as int).drop_first() == models(fs@).skip(i as int + 1));
            assert(models(fs@).skip(i as int)[0] == fs@[i as int]@);
            assert(valid(arena.nodes@, &fs@[i as int]));
            reveal_with_fuel(spec::targets, 1);
        }
        let xs = args(arena, &fs[i]);
        let hit = if xs.len() > 0 {
            proof {
                assert(valid(arena.nodes@, &xs@[0]));
                arg_at(fs@[i as int]@, xs@, 0);
            }
            crate::m6_term::equal(arena, &xs[0], y)
        } else {
            proof {
                assert(arg(fs@[i as int]@, 0) == Term::Nil);
            }
            is_nil(arena, y)
        };
        if hit {
            n += 1;
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::targets, 1);
    }
    n
}

pub fn free_operand(
    arena: &ETermArena,
    src: &ESrc,
    an: &Vec<EAnch>,
    ops: &Vec<T>,
    inner: &T,
) -> (out: bool)
    requires
        arena_ok(arena),
        src_valid(arena.nodes@, src),
        anchs_valid(arena.nodes@, an@),
        valid_all(arena.nodes@, ops@),
        valid(arena.nodes@, inner),
    ensures
        out == spec::free_operand(src@, anch_models(an@), models(ops@), inner@),
{
    if !is_comp(arena, inner, &Sym::DollarGuidelineBound, 8) {
        return false;
    }
    let xs = args(arena, inner);
    proof {
        arg_at(inner@, xs@, 0);
        arg_at(inner@, xs@, 6);
        arg_at(inner@, xs@, 7);
        reveal_strlit("geq");
    }
    contains(arena, ops, &xs[0]) && is_atom(arena, &xs[6], &Sym::Geq) && int_is(arena, &xs[7], 2)
        && some_before(arena, src, an, &xs[0])
}

fn amount(arena: &mut ETermArena, inner: &T, sv: &T, nv: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, inner),
        valid(old(arena).nodes@, sv),
        valid(old(arena).nodes@, nv),
        args_of_len(inner@) >= 4,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::amount(inner@, sv@, nv@),
{
    let xs = args(arena, inner);
    let mut parts: Vec<T> = Vec::new();
    parts.push(xs[0].cp());
    parts.push(xs[1].cp());
    parts.push(xs[2].cp());
    parts.push(xs[3].cp());
    parts.push(sv.cp());
    parts.push(nv.cp());
    proof {
        assert(models(parts@) == seq![
            arg(inner@, 0),
            arg(inner@, 1),
            arg(inner@, 2),
            arg(inner@, 3),
            sv@,
            nv@,
        ]);
        reveal_strlit("$guideline_amount");
    }
    c(arena, &Sym::DollarGuidelineAmount, &parts)
}

pub open spec fn args_of_len(t: Term) -> nat {
    ckc_spec::engine::args_of(t).len()
}

pub fn free_items(
    arena: &mut ETermArena,
    src: &ESrc,
    an: &Vec<EAnch>,
    ops: &Vec<T>,
    items: &Vec<I>,
) -> (out: Vec<I>)
    requires
        arena_ok(old(arena)),
        src_valid(old(arena).nodes@, src),
        anchs_valid(old(arena).nodes@, an@),
        valid_all(old(arena).nodes@, ops@),
        items_valid(old(arena).nodes@, items@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        items_valid(final(arena).nodes@, out@),
        item_models(out@) == spec::free_items(
            src@,
            anch_models(an@),
            models(ops@),
            item_models(items@),
        ),
{
    let ghost start = arena.nodes@;
    let mut out: Vec<I> = Vec::new();
    let mut i = 0usize;
    proof {
        reveal_with_fuel(items_valid, 1);
    }
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
    }
    proof {
        assert_seqs_equal!(item_models(out@) + spec::free_items(src@, anch_models(an@), models(ops@), item_models(items@)) == spec::free_items(src@, anch_models(an@), models(ops@), item_models(items@)));
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            src_valid(arena.nodes@, src),
            anchs_valid(arena.nodes@, an@),
            valid_all(arena.nodes@, ops@),
            items_valid(arena.nodes@, items@),
            items_valid(arena.nodes@, out@),
            i <= items.len(),
            spec::free_items(src@, anch_models(an@), models(ops@), item_models(items@))
                == item_models(out@) + spec::free_items(
                src@,
                anch_models(an@),
                models(ops@),
                item_models(items@).skip(i as int),
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            assert(item_models(items@).skip(i as int)[0] == items@[i as int]@);
            reveal_with_fuel(spec::free_items, 1);
            reveal_strlit("actual");
        }
        let ghost before_nodes = arena.nodes@;
        let next = match &items[i].kind {
            Kind::Anch(c, inner) => {
                if is_atom(arena, c, &Sym::Actual) && free_operand(arena, src, an, ops, inner) {
                    let xs = args(arena, inner);
                    proof {
                        arg_at(inner@, xs@, 4);
                        arg_at(inner@, xs@, 5);
                    }
                    let t = amount(arena, inner, &xs[4], &xs[5]);
                    proof {
                        prefix(before_nodes, arena.nodes@, c);
                    }
                    anch(arena, c, &t)
                } else {
                    copy_item(arena, &items[i])
                }
            },
            _ => copy_item(arena, &items[i]),
        };
        proof {
            items_prefix(before_nodes, arena.nodes@, out@);
            items_prefix(before_nodes, arena.nodes@, items@);
            prefix_all(before_nodes, arena.nodes@, ops@);
            prefix_all(before_nodes, arena.nodes@, src.toks@);
            assert forall|x: int| 0 <= x < an.len() implies valid(
                arena.nodes@,
                &(#[trigger] an@[x]).inner,
            ) && valid(arena.nodes@, &an@[x].k) && an@[x].k@ is Int by {
                prefix(before_nodes, arena.nodes@, &an@[x].inner);
                prefix(before_nodes, arena.nodes@, &an@[x].k);
            }
        }
        let ghost prev = out@;
        out.push(next);
        proof {
            reveal_with_fuel(items_valid, 2);
            assert(items_valid(arena.nodes@, seq![next]));
            items_concat(arena.nodes@, prev, seq![next]);
            assert(out@ == prev + seq![next]);
            assert(item_models(seq![next]) == seq![next@]);
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::free_items, 1);
        assert_seqs_equal!(item_models(out@) + Seq::<spec::Item>::empty() == item_models(out@));
    }
    out
}

pub fn misplaced_item(arena: &ETermArena, it: &I, top_ok: bool) -> (out: bool)
    requires
        arena_ok(arena),
        item_valid(arena.nodes@, it),
    ensures
        out == spec::misplaced_item(it@, top_ok),
    decreases it, 0int,
{
    proof {
        reveal(item_valid);
        reveal_with_fuel(spec::misplaced_item, 1);
        reveal_strlit("actual");
        reveal_strlit("formula");
    }
    match &it.kind {
        Kind::Anch(c, inner) => is_comp(arena, inner, &Sym::Formula, 3) && !(top_ok && is_atom(
            arena,
            c,
            &Sym::Actual,
        )),
        Kind::Naf { payload, .. } => misplaced(arena, payload, false),
        Kind::Op { .. } => false,
    }
}

pub fn misplaced(arena: &ETermArena, items: &Vec<I>, top_ok: bool) -> (out: bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        out == spec::misplaced(item_models(items@), top_ok),
    decreases items@, 1int,
{
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            i <= items.len(),
            spec::misplaced(item_models(items@), top_ok) == spec::misplaced(
                item_models(items@).skip(i as int),
                top_ok,
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            assert(item_models(items@).skip(i as int)[0] == items@[i as int]@);
            reveal_with_fuel(spec::misplaced, 1);
        }
        if misplaced_item(arena, &items[i], top_ok) {
            return true;
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::misplaced, 1);
        assert(item_models(items@).skip(i as int).len() == 0);
    }
    false
}

pub fn top_formula(arena: &ETermArena, it: &I) -> (out: bool)
    requires
        arena_ok(arena),
        item_valid(arena.nodes@, it),
    ensures
        out == spec::top_formula(it@),
{
    proof {
        reveal(item_valid);
        reveal_strlit("actual");
        reveal_strlit("formula");
    }
    match &it.kind {
        Kind::Anch(c, inner) => is_atom(arena, c, &Sym::Actual) && is_comp(
            arena,
            inner,
            &Sym::Formula,
            3,
        ),
        _ => false,
    }
}

pub fn top_formulas(arena: &ETermArena, items: &Vec<I>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::top_formulas(item_models(items@)),
{
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
    }
    proof {
        assert_seqs_equal!(models(out@) + spec::top_formulas(item_models(items@)) == spec::top_formulas(item_models(items@)));
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid_all(arena.nodes@, out@),
            i <= items.len(),
            spec::top_formulas(item_models(items@)) == models(out@) + spec::top_formulas(
                item_models(items@).skip(i as int),
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            assert(item_models(items@).skip(i as int)[0] == items@[i as int]@);
            reveal_with_fuel(spec::top_formulas, 1);
            reveal_strlit("actual");
            reveal_strlit("formula");
        }
        match &items[i].kind {
            Kind::Anch(c, inner) => {
                if is_atom(arena, c, &Sym::Actual) && is_comp(arena, inner, &Sym::Formula, 3) {
                    let ghost prev = out@;
                    let t = inner.cp();
                    proof {
                        extend(arena.nodes@, out@, t);
                    }
                    out.push(t);
                    proof {
                        assert_seqs_equal!(models(out@) == models(prev) + seq![inner@]);
                    }
                }
            },
            _ => {},
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::top_formulas, 1);
        assert_seqs_equal!(models(out@) + Seq::<Term>::empty() == models(out@));
    }
    out
}

pub fn drop_formulas(arena: &ETermArena, items: &Vec<I>) -> (out: Vec<I>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        items_valid(arena.nodes@, out@),
        item_models(out@) == spec::drop_formulas(item_models(items@)),
{
    let mut out: Vec<I> = Vec::new();
    let mut i = 0usize;
    proof {
        reveal_with_fuel(items_valid, 1);
    }
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
    }
    proof {
        assert_seqs_equal!(item_models(out@) + spec::drop_formulas(item_models(items@)) == spec::drop_formulas(item_models(items@)));
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            items_valid(arena.nodes@, out@),
            i <= items.len(),
            spec::drop_formulas(item_models(items@)) == item_models(out@) + spec::drop_formulas(
                item_models(items@).skip(i as int),
            ),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            assert(item_models(items@).skip(i as int)[0] == items@[i as int]@);
            reveal_with_fuel(spec::drop_formulas, 1);
        }
        if !top_formula(arena, &items[i]) {
            let next = copy_item(arena, &items[i]);
            let ghost prev = out@;
            out.push(next);
            proof {
                reveal_with_fuel(items_valid, 2);
                assert(items_valid(arena.nodes@, seq![next]));
                items_concat(arena.nodes@, prev, seq![next]);
                assert(out@ == prev + seq![next]);
                assert(item_models(seq![next]) == seq![next@]);
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::drop_formulas, 1);
        assert_seqs_equal!(item_models(out@) + Seq::<spec::Item>::empty() == item_models(out@));
    }
    out
}

pub fn target_ok(arena: &ETermArena, src: &ESrc, an: &Vec<EAnch>, citems: &Vec<I>, y: &T) -> (out:
    bool)
    requires
        arena_ok(arena),
        src_valid(arena.nodes@, src),
        anchs_valid(arena.nodes@, an@),
        items_valid(arena.nodes@, citems@),
        valid(arena.nodes@, y),
    ensures
        out == spec::target_ok(src@, anch_models(an@), item_models(citems@), y@),
{
    proof {
        reveal_strlit("countable");
        reveal_strlit("na");
        reveal_strlit("geq");
    }
    proof {
        top_object_shape(item_models(citems@), y@);
        reveal(spec::target_ok);
    }
    match top_object(arena, citems, y) {
        None => false,
        Some(o) => {
            let xs = args(arena, &o);
            proof {
                assert(spec::is_comp(o@, "object"@, 6));
                assert(xs@.len() == 6);
                arg_at(o@, xs@, 2);
                arg_at(o@, xs@, 3);
                arg_at(o@, xs@, 4);
                arg_at(o@, xs@, 5);
            }
            xs.len() == 6 && is_atom(arena, &xs[2], &Sym::Countable) && is_atom(
                arena,
                &xs[3],
                &Sym::Na,
            ) && is_atom(arena, &xs[4], &Sym::Geq) && int_is(arena, &xs[5], 2) && some_before(
                arena,
                src,
                an,
                y,
            )
        },
    }
}

pub fn rewrite_target(arena: &mut ETermArena, items: &Vec<I>, y: &T, sv: &T, nv: &T) -> (out: Vec<
    I,
>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, items@),
        valid(old(arena).nodes@, y),
        valid(old(arena).nodes@, sv),
        valid(old(arena).nodes@, nv),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        items_valid(final(arena).nodes@, out@),
        item_models(out@) == spec::rewrite_target(item_models(items@), y@, sv@, nv@),
{
    let ghost start = arena.nodes@;
    let mut out: Vec<I> = Vec::new();
    let mut i = 0usize;
    proof {
        reveal_with_fuel(items_valid, 1);
    }
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
    }
    proof {
        assert_seqs_equal!(item_models(out@) + spec::rewrite_target(item_models(items@), y@, sv@, nv@) == spec::rewrite_target(item_models(items@), y@, sv@, nv@));
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            items_valid(arena.nodes@, items@),
            items_valid(arena.nodes@, out@),
            valid(arena.nodes@, y),
            valid(arena.nodes@, sv),
            valid(arena.nodes@, nv),
            i <= items.len(),
            spec::rewrite_target(item_models(items@), y@, sv@, nv@) == item_models(out@)
                + spec::rewrite_target(item_models(items@).skip(i as int), y@, sv@, nv@),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            assert(item_models(items@).skip(i as int)[0] == items@[i as int]@);
            reveal_with_fuel(spec::rewrite_target, 1);
            reveal_strlit("actual");
        }
        let ghost before_nodes = arena.nodes@;
        let next = match &items[i].kind {
            Kind::Anch(c, inner) => {
                let hit = if is_atom(arena, c, &Sym::Actual) && is_comp(
                    arena,
                    inner,
                    &Sym::Object,
                    6,
                ) {
                    let xs = args(arena, inner);
                    proof {
                        arg_at(inner@, xs@, 0);
                    }
                    crate::m6_term::equal(arena, &xs[0], y)
                } else {
                    false
                };
                if hit {
                    let t = amount(arena, inner, sv, nv);
                    proof {
                        prefix(before_nodes, arena.nodes@, c);
                    }
                    anch(arena, c, &t)
                } else {
                    copy_item(arena, &items[i])
                }
            },
            _ => copy_item(arena, &items[i]),
        };
        proof {
            items_prefix(before_nodes, arena.nodes@, out@);
            items_prefix(before_nodes, arena.nodes@, items@);
            prefix(before_nodes, arena.nodes@, y);
            prefix(before_nodes, arena.nodes@, sv);
            prefix(before_nodes, arena.nodes@, nv);
        }
        let ghost prev = out@;
        out.push(next);
        proof {
            reveal_with_fuel(items_valid, 2);
            assert(items_valid(arena.nodes@, seq![next]));
            items_concat(arena.nodes@, prev, seq![next]);
            assert(out@ == prev + seq![next]);
            assert(item_models(seq![next]) == seq![next@]);
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::rewrite_target, 1);
        assert_seqs_equal!(item_models(out@) + Seq::<spec::Item>::empty() == item_models(out@));
    }
    out
}

proof fn operand_pairs_keys(items: Seq<spec::Item>, rs: Seq<Term>)
    ensures
        spec::operand_pairs(items, rs) matches Ok(ps) ==> ps.len() == rs.len() && forall|x: int|
            0 <= x < rs.len() ==> #[trigger] ps[x].0 == rs[x],
{
    reveal(spec::operand_pairs);
    bound_pairs_keys(items, rs);
}

proof fn bound_pairs_keys(items: Seq<spec::Item>, rs: Seq<Term>)
    ensures
        spec::bound_pairs(items, rs) matches Ok(ps) ==> ps.len() == rs.len() && forall|x: int|
            0 <= x < rs.len() ==> #[trigger] ps[x].0 == rs[x],
    decreases rs.len(),
{
    reveal_with_fuel(spec::bound_pairs, 1);
    if rs.len() > 0 {
        bound_pairs_keys(items, rs.drop_first());
        if let Ok(ps) = spec::bound_pairs(items, rs) {
            assert forall|x: int| 0 <= x < rs.len() implies #[trigger] ps[x].0 == rs[x] by {
                if x > 0 {
                    assert(rs.drop_first()[x - 1] == rs[x]);
                }
            }
        }
    }
}

proof fn top_formulas_shape(items: Seq<spec::Item>)
    ensures
        forall|x: int|
            0 <= x < spec::top_formulas(items).len() ==> spec::is_comp(
                #[trigger] spec::top_formulas(items)[x],
                "formula"@,
                3,
            ),
    decreases items.len(),
{
    reveal_with_fuel(spec::top_formulas, 1);
    if items.len() > 0 {
        top_formulas_shape(items.drop_first());
        let rest = spec::top_formulas(items.drop_first());
        match items[0] {
            spec::Item::Anch(c, inner) => if c == spec::actual() && spec::is_formula(inner) {
                assert forall|x: int|
                    0 <= x < spec::top_formulas(items).len() implies spec::is_comp(
                    #[trigger] spec::top_formulas(items)[x],
                    "formula"@,
                    3,
                ) by {
                    if x > 0 {
                        assert(spec::top_formulas(items)[x] == rest[x - 1]);
                    }
                }
            },
            _ => {},
        }
    }
}

pub struct EFPass {
    pub aitems: Vec<I>,
    pub citems: Vec<I>,
    pub chain: Vec<T>,
    pub pairs: Vec<Binding>,
}

pub open spec fn fpass_model(f: &EFPass) -> spec::FPass {
    spec::FPass {
        aitems: item_models(f.aitems@),
        citems: item_models(f.citems@),
        chain: models(f.chain@),
        pairs: map_model(f.pairs@),
    }
}

pub open spec fn fpass_valid(nodes: Seq<ENode>, f: &EFPass) -> bool {
    items_valid(nodes, f.aitems@) && items_valid(nodes, f.citems@) && valid_all(nodes, f.chain@)
        && map_valid(nodes, f.pairs@)
}

pub open spec fn step_result(r: &Result<(EFPass, usize), T>) -> Result<(spec::FPass, nat), Term> {
    match r {
        Ok((f, v)) => Ok((fpass_model(f), *v as nat)),
        Err(e) => Err(e@),
    }
}

// The relation, operator and target checks of a formula (contract da2 D5),
// in order: the first three branches of `spec::formula_step`.
pub open spec fn checks(
    src: spec::Src,
    an: Seq<(Term, int)>,
    fs: Seq<Term>,
    f: Term,
    citems: Seq<spec::Item>,
) -> Result<(), Term> {
    let y = arg(f, 0);
    let e = arg(f, 2);
    if arg(f, 1) != ckc_spec::replay::atom("="@) {
        Err(spec::fshape("relation"@))
    } else if !spec::is_comp(e, "expr"@, 3) || !spec::ops_ok(e) {
        Err(spec::fshape("operator"@))
    } else if !(y is Var) || spec::formula_refs(fs).contains(y) || spec::targets(fs, y) != 1
        || !spec::target_ok(src, an, citems, y) {
        Err(spec::fshape("target"@))
    } else {
        Ok(())
    }
}

fn step_checks(
    arena: &mut ETermArena,
    src: &ESrc,
    an: &Vec<EAnch>,
    fs: &Vec<T>,
    f: &T,
    citems: &Vec<I>,
) -> (out: Result<(T, T), T>)
    requires
        arena_ok(old(arena)),
        src_valid(old(arena).nodes@, src),
        anchs_valid(old(arena).nodes@, an@),
        valid_all(old(arena).nodes@, fs@),
        valid(old(arena).nodes@, f),
        spec::is_comp(f@, "formula"@, 3),
        items_valid(old(arena).nodes@, citems@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match &out {
            Ok((y, e)) => valid(final(arena).nodes@, y) && valid(final(arena).nodes@, e) && y@
                == arg(f@, 0) && e@ == arg(f@, 2) && checks(
                src@,
                anch_models(an@),
                models(fs@),
                f@,
                item_models(citems@),
            ) == Ok::<(), Term>(()),
            Err(r) => valid(final(arena).nodes@, r) && checks(
                src@,
                anch_models(an@),
                models(fs@),
                f@,
                item_models(citems@),
            ) == Err::<(), Term>(r@),
        },
{
    proof {
        reveal(spec::fshape);
        reveal_strlit("=");
        reveal_strlit("expr");
        reveal_strlit("relation");
        reveal_strlit("operator");
        reveal_strlit("target");
    }
    let xs = args(arena, f);
    proof {
        assert(xs@.len() == 3);
        arg_at(f@, xs@, 0);
        arg_at(f@, xs@, 1);
        arg_at(f@, xs@, 2);
    }
    let y = xs[0].cp();
    let e = xs[2].cp();
    if !is_atom(arena, &xs[1], &Sym::Equals) {
        return Err(shape(arena, &Sym::Relation));
    }
    if !is_comp(arena, &e, &Sym::Expr, 3) || !ops_ok(arena, &e) {
        return Err(shape(arena, &Sym::Operator));
    }
    let refs = formula_refs(arena, fs);
    if !is_var(arena, &y) || contains(arena, &refs, &y) || targets(arena, fs, &y) != 1
        || !target_ok(arena, src, an, citems, &y) {
        return Err(shape(arena, &Sym::Target));
    }
    Ok((y, e))
}

proof fn chain_arith(e: Term, ps: Seq<(Term, (Term, Term))>, vals: Seq<Term>, v: nat, m: nat)
    ensures
        forall|i: int|
            0 <= i < spec::chain(e, ps, vals, v, m).goals.len() ==> ((#[trigger] spec::chain(
                e,
                ps,
                vals,
                v,
                m,
            ).goals[i]) matches Term::Comp(name, args) && ckc_spec::arith::is_arith(
                name,
                args.len(),
            )),
    decreases e,
{
    reveal_with_fuel(spec::chain, 1);
    reveal_strlit("expr");
    reveal_strlit("guideline_product");
    reveal_strlit("guideline_quotient");
    match e {
        Term::Comp(name, args) => {
            if name == ckc_spec::v1text::ascii("expr"@) && args.len() == 3 {
                let l = spec::chain(args[1], ps, vals, v, m);
                let r = spec::chain(args[2], ps, vals, l.v, l.m);
                chain_arith(args[1], ps, vals, v, m);
                chain_arith(args[2], ps, vals, l.v, l.m);
                let g = spec::chain(e, ps, vals, v, m).goals;
                assert(g.len() == l.goals.len() + r.goals.len() + 1);
                assert forall|i: int| 0 <= i < g.len() implies ((
                #[trigger] g[i]) matches Term::Comp(name, args) && ckc_spec::arith::is_arith(
                    name,
                    args.len(),
                )) by {
                    if i < l.goals.len() {
                        assert(g[i] == l.goals[i]);
                    } else if i < l.goals.len() + r.goals.len() {
                        assert(g[i] == r.goals[i - l.goals.len()]);
                    }
                }
            }
        },
        _ => {},
    }
}

// The operand pairs + literal values of a formula that passed its checks:
// the D5 operand, operand_bound + literal branches of `spec::formula_step`.
pub open spec fn inputs(
    src: spec::Src,
    an: Seq<(Term, int)>,
    f: Term,
    aitems: Seq<spec::Item>,
) -> Result<(Seq<(Term, (Term, Term))>, Seq<Term>), Term> {
    let e = arg(f, 2);
    match spec::operand_pairs(aitems, spec::var_leaves(e)) {
        Err(r) => Err(r),
        Ok(ps) => match spec::formula_token(an, f) {
            None => Err(spec::fshape("literal"@)),
            Some(kf) => match spec::literal_values(src, kf, spec::lit_leaves(e)) {
                None => Err(spec::fshape("literal"@)),
                Some(vals) => Ok((ps, vals)),
            },
        },
    }
}

// The chain, witness + target rewrite of a formula with its inputs.
pub open spec fn tail(
    e: Term,
    y: Term,
    ps: Seq<(Term, (Term, Term))>,
    vals: Seq<Term>,
    acc: spec::FPass,
    v: nat,
) -> Result<(spec::FPass, nat), Term> {
    let ch = spec::chain(e, ps, vals, v, 0);
    match spec::chain_pairs(ch.goals, acc.pairs) {
        None => Err(spec::fshape("witness"@)),
        Some(pairs) => Ok(
            (
                spec::FPass {
                    aitems: acc.aitems,
                    citems: spec::rewrite_target(acc.citems, y, ch.out.0, ch.out.1),
                    chain: acc.chain + ch.goals,
                    pairs,
                },
                ch.v,
            ),
        ),
    }
}

proof fn step_split(
    src: spec::Src,
    an: Seq<(Term, int)>,
    fs: Seq<Term>,
    f: Term,
    aitems: Seq<spec::Item>,
    acc: spec::FPass,
    v: nat,
)
    ensures
        spec::formula_step(src, an, fs, f, aitems, acc, v) == match checks(
            src,
            an,
            fs,
            f,
            acc.citems,
        ) {
            Err(r) => Err(r),
            Ok(_) => match inputs(src, an, f, aitems) {
                Err(r) => Err(r),
                Ok((ps, vals)) => tail(arg(f, 2), arg(f, 0), ps, vals, acc, v),
            },
        },
{
}

fn step_inputs(
    arena: &mut ETermArena,
    src: &ESrc,
    an: &Vec<EAnch>,
    f: &T,
    aitems: &Vec<I>,
    e: &T,
) -> (out: Result<(Vec<EPair>, Vec<T>), T>)
    requires
        arena_ok(old(arena)),
        src_valid(old(arena).nodes@, src),
        anchs_valid(old(arena).nodes@, an@),
        valid(old(arena).nodes@, f),
        items_valid(old(arena).nodes@, aitems@),
        valid(old(arena).nodes@, e),
        e@ == arg(f@, 2),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match &out {
            Ok((ps, vals)) => pairs_valid(final(arena).nodes@, ps@) && valid_all(
                final(arena).nodes@,
                vals@,
            ) && inputs(src@, anch_models(an@), f@, item_models(aitems@)) == Ok::<
                (Seq<(Term, (Term, Term))>, Seq<Term>),
                Term,
            >((pair_models(ps@), models(vals@))) && spec::lit_leaves(e@).len() == vals.len() && (
            forall|x: int|
                0 <= x < spec::var_leaves(e@).len() ==> has_key(
                    ps@,
                    #[trigger] spec::var_leaves(e@)[x],
                )),
            Err(r) => valid(final(arena).nodes@, r) && inputs(
                src@,
                anch_models(an@),
                f@,
                item_models(aitems@),
            ) == Err::<(Seq<(Term, (Term, Term))>, Seq<Term>), Term>(r@),
        },
{
    hide(spec::operand_pairs);
    hide(spec::literal_values);
    let ghost start = arena.nodes@;
    let vars = var_leaves(arena, e);
    let ps = match operand_pairs(arena, aitems, &vars) {
        Ok(ps) => ps,
        Err(r) => return Err(r),
    };
    let ghost n0 = arena.nodes@;
    proof {
        operand_pairs_keys(item_models(aitems@), models(vars@));
        assert forall|x: int| 0 <= x < spec::var_leaves(e@).len() implies has_key(
            ps@,
            #[trigger] spec::var_leaves(e@)[x],
        ) by {
            assert(pair_models(ps@)[x].0 == models(vars@)[x]);
            assert(ps@[x].key@ == spec::var_leaves(e@)[x]);
        }
        prefix_all(start, n0, src.toks@);
        prefix(start, n0, e);
        prefix(start, n0, f);
        assert forall|x: int| 0 <= x < an.len() implies valid(n0, &(#[trigger] an@[x]).inner)
            && valid(n0, &an@[x].k) && an@[x].k@ is Int by {
            prefix(start, n0, &an@[x].inner);
            prefix(start, n0, &an@[x].k);
        }
    }
    let lits = lit_leaves(arena, e);
    let kf = match formula_token(arena, an, f) {
        None => {
            let r = shape(arena, &Sym::Literal);
            proof {
                reveal(spec::fshape);
                reveal_strlit("literal");
                crate::k2_load::prefix_chain(start, n0, arena.nodes@);
            }
            return Err(r);
        },
        Some(k) => k,
    };
    let j = match index_of(arena, &kf, src.toks.len(), 0) {
        None => {
            proof {
                reveal(spec::literal_values);
                assert(spec::literal_values(src@, int_value(kf@), models(lits@)) is None);
            }
            let r = shape(arena, &Sym::Literal);
            proof {
                reveal(spec::fshape);
                reveal_strlit("literal");
                crate::k2_load::prefix_chain(start, n0, arena.nodes@);
            }
            return Err(r);
        },
        Some(j) => j,
    };
    let lv = literal_values(arena, src, j, &lits);
    let vals = match lv {
        None => {
            let r = shape(arena, &Sym::Literal);
            proof {
                reveal(spec::fshape);
                reveal_strlit("literal");
                crate::k2_load::prefix_chain(start, n0, arena.nodes@);
            }
            return Err(r);
        },
        Some(vals) => vals,
    };
    let ghost n1 = arena.nodes@;
    proof {
        reveal(spec::literal_values);
        assert(models(vals@).len() == models(lits@).len());
        assert(vals@.len() == lits@.len());
        crate::k2_load::prefix_chain(start, n0, n1);
        assert forall|x: int| 0 <= x < ps.len() implies valid(n1, &(#[trigger] ps@[x]).key)
            && valid(n1, &ps@[x].s) && valid(n1, &ps@[x].n) by {
            prefix(n0, n1, &ps@[x].key);
            prefix(n0, n1, &ps@[x].s);
            prefix(n0, n1, &ps@[x].n);
        }
    }
    Ok((ps, vals))
}

fn step_tail(
    arena: &mut ETermArena,
    e: &T,
    y: &T,
    ps: &Vec<EPair>,
    vals: &Vec<T>,
    acc: EFPass,
    v: usize,
) -> (out: Result<(EFPass, usize), T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, e),
        valid(old(arena).nodes@, y),
        pairs_valid(old(arena).nodes@, ps@),
        valid_all(old(arena).nodes@, vals@),
        fpass_valid(old(arena).nodes@, &acc),
        spec::ops_ok(e@),
        forall|x: int|
            0 <= x < spec::var_leaves(e@).len() ==> has_key(
                ps@,
                #[trigger] spec::var_leaves(e@)[x],
            ),
        spec::lit_leaves(e@).len() <= vals.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match &out {
            Ok((p, _)) => fpass_valid(final(arena).nodes@, p),
            Err(r) => valid(final(arena).nodes@, r),
        },
        step_result(&out) == tail(
            e@,
            y@,
            pair_models(ps@),
            models(vals@),
            fpass_model(&acc),
            v as nat,
        ),
{
    hide(spec::chain);
    hide(spec::chain_pairs);
    hide(spec::rewrite_target);
    let ghost n1 = arena.nodes@;
    let ch = chain(arena, e, ps, vals, v, 0);
    let ghost n2 = arena.nodes@;
    proof {
        chain_arith(e@, pair_models(ps@), models(vals@), v as nat, 0);
        map_prefix(n1, n2, acc.pairs@);
        prefix(n1, n2, y);
        items_prefix(n1, n2, acc.aitems@);
        items_prefix(n1, n2, acc.citems@);
        prefix_all(n1, n2, acc.chain@);
        assert forall|i: int| 0 <= i < ch.goals.len() implies ((
        #[trigger] ch.goals@[i])@ matches Term::Comp(name, args) && ckc_spec::arith::is_arith(
            name,
            args.len(),
        )) by {
            assert(models(ch.goals@)[i] == ch.goals@[i]@);
        }
    }
    let witnessed = chain_pairs(arena, &ch.goals, &acc.pairs);
    let pairs = match witnessed {
        None => {
            let r = shape(arena, &Sym::Witness);
            proof {
                reveal(spec::fshape);
                reveal_strlit("witness");
            }
            return Err(r);
        },
        Some(p) => p,
    };
    let ghost n3 = arena.nodes@;
    proof {
        prefix(n2, n3, y);
        prefix(n2, n3, &ch.out_s);
        prefix(n2, n3, &ch.out_n);
        prefix_all(n2, n3, ch.goals@);
        items_prefix(n2, n3, acc.aitems@);
        items_prefix(n2, n3, acc.citems@);
        prefix_all(n2, n3, acc.chain@);
    }
    let citems = rewrite_target(arena, &acc.citems, y, &ch.out_s, &ch.out_n);
    let ghost n4 = arena.nodes@;
    proof {
        items_prefix(n3, n4, acc.aitems@);
        prefix_all(n3, n4, acc.chain@);
        prefix_all(n3, n4, ch.goals@);
        map_prefix(n3, n4, pairs@);
    }
    let chain_all = concat(&acc.chain, &ch.goals);
    proof {
        models_concat(acc.chain@, ch.goals@);
        valid_concat(n4, acc.chain@, ch.goals@);
    }
    Ok((EFPass { aitems: acc.aitems, citems, chain: chain_all, pairs }, ch.v))
}

// One formula after the earlier ones (contract da2 D1–D5).
pub fn formula_step(
    arena: &mut ETermArena,
    src: &ESrc,
    an: &Vec<EAnch>,
    fs: &Vec<T>,
    f: &T,
    aitems: &Vec<I>,
    acc: EFPass,
    v: usize,
) -> (out: Result<(EFPass, usize), T>)
    requires
        arena_ok(old(arena)),
        src_valid(old(arena).nodes@, src),
        anchs_valid(old(arena).nodes@, an@),
        valid_all(old(arena).nodes@, fs@),
        valid(old(arena).nodes@, f),
        spec::is_comp(f@, "formula"@, 3),
        items_valid(old(arena).nodes@, aitems@),
        fpass_valid(old(arena).nodes@, &acc),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match &out {
            Ok((p, _)) => fpass_valid(final(arena).nodes@, p),
            Err(e) => valid(final(arena).nodes@, e),
        },
        step_result(&out) == spec::formula_step(
            src@,
            anch_models(an@),
            models(fs@),
            f@,
            item_models(aitems@),
            fpass_model(&acc),
            v as nat,
        ),
{
    hide(spec::formula_step);
    hide(checks);
    hide(inputs);
    hide(tail);
    let ghost start = arena.nodes@;
    proof {
        step_split(
            src@,
            anch_models(an@),
            models(fs@),
            f@,
            item_models(aitems@),
            fpass_model(&acc),
            v as nat,
        );
    }
    let (y, e) = match step_checks(arena, src, an, fs, f, &acc.citems) {
        Ok(p) => p,
        Err(r) => return Err(r),
    };
    let ghost n0 = arena.nodes@;
    proof {
        items_prefix(start, n0, aitems@);
        prefix_all(start, n0, src.toks@);
        assert forall|x: int| 0 <= x < an.len() implies valid(n0, &(#[trigger] an@[x]).inner)
            && valid(n0, &an@[x].k) && an@[x].k@ is Int by {
            prefix(start, n0, &an@[x].inner);
            prefix(start, n0, &an@[x].k);
        }
        prefix(start, n0, f);
    }
    let (ps, vals) = match step_inputs(arena, src, an, f, aitems, &e) {
        Ok(p) => p,
        Err(r) => return Err(r),
    };
    let ghost n1 = arena.nodes@;
    proof {
        reveal(checks);
        reveal_strlit("expr");
        assert(spec::ops_ok(e@));
        prefix(n0, n1, &e);
        prefix(n0, n1, &y);
        crate::k2_load::prefix_chain(start, n0, n1);
        items_prefix(start, n1, acc.aitems@);
        items_prefix(start, n1, acc.citems@);
        prefix_all(start, n1, acc.chain@);
        map_prefix(start, n1, acc.pairs@);
    }
    step_tail(arena, &e, &y, &ps, &vals, acc, v)
}

pub fn formula_pass(
    arena: &mut ETermArena,
    src: &ESrc,
    an: &Vec<EAnch>,
    aitems: &Vec<I>,
    citems: &Vec<I>,
    ordered: &Vec<T>,
    map: &Vec<Binding>,
    env: &Env,
    k: usize,
    v: usize,
) -> (out: Result<EFPass, T>)
    requires
        arena_ok(old(arena)),
        src_valid(old(arena).nodes@, src),
        anchs_valid(old(arena).nodes@, an@),
        items_valid(old(arena).nodes@, aitems@),
        items_valid(old(arena).nodes@, citems@),
        valid_all(old(arena).nodes@, ordered@),
        map_valid(old(arena).nodes@, map@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match &out {
            Ok(p) => fpass_valid(final(arena).nodes@, p),
            Err(e) => valid(final(arena).nodes@, e),
        },
        match &out {
            Ok(p) => Ok::<spec::FPass, Term>(fpass_model(p)),
            Err(e) => Err::<spec::FPass, Term>(e@),
        } == spec::formula_pass(
            src@,
            anch_models(an@),
            item_models(aitems@),
            item_models(citems@),
            models(ordered@),
            map_model(map@),
            env.docid@,
            env.s as nat,
            k as nat,
            v as nat,
        ),
{
    let ghost start = arena.nodes@;
    proof {
        reveal(spec::formula_pass);
        reveal(spec::fshape);
        reveal_strlit("position");
    }
    let fs = top_formulas(arena, citems);
    let ops = formula_refs(arena, &fs);
    let free = free_items(arena, src, an, &ops, aitems);
    let ghost n1 = arena.nodes@;
    proof {
        items_prefix(start, n1, aitems@);
        items_prefix(start, n1, citems@);
        prefix_all(start, n1, fs@);
        prefix_all(start, n1, ops@);
        prefix_all(start, n1, ordered@);
        map_prefix(start, n1, map@);
        prefix_all(start, n1, src.toks@);
        assert forall|x: int| 0 <= x < an.len() implies valid(n1, &(#[trigger] an@[x]).inner)
            && valid(n1, &an@[x].k) && an@[x].k@ is Int by {
            prefix(start, n1, &an@[x].inner);
            prefix(start, n1, &an@[x].k);
        }
    }
    if misplaced(arena, aitems, false) || misplaced(arena, citems, true) {
        return Err(shape(arena, &Sym::Position));
    }
    let rest = drop_formulas(arena, citems);
    let acc0 = Vec::new();
    proof {
        assert(map_model(acc0@) == Seq::<(Term, Term)>::empty());
    }
    let pairs = witness_pairs(arena, &free, ordered, map, env, k, &ops, &acc0);
    let ghost n2 = arena.nodes@;
    proof {
        items_prefix(n1, n2, free@);
        items_prefix(n1, n2, rest@);
        items_prefix(n1, n2, aitems@);
        prefix_all(n1, n2, fs@);
        prefix_all(n1, n2, src.toks@);
        assert forall|x: int| 0 <= x < an.len() implies valid(n2, &(#[trigger] an@[x]).inner)
            && valid(n2, &an@[x].k) && an@[x].k@ is Int by {
            prefix(n1, n2, &an@[x].inner);
            prefix(n1, n2, &an@[x].k);
        }
        assert(valid_all(n2, Seq::<T>::empty()));
    }
    let mut acc = EFPass { aitems: free, citems: rest, chain: Vec::new(), pairs };
    let mut vv = v;
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(models(acc.chain@) == Seq::<Term>::empty());
        top_formulas_shape(item_models(citems@));
    }
    while i < fs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            fpass_valid(arena.nodes@, &acc),
            valid_all(arena.nodes@, fs@),
            src_valid(arena.nodes@, src),
            anchs_valid(arena.nodes@, an@),
            items_valid(arena.nodes@, aitems@),
            forall|x: int|
                0 <= x < fs.len() ==> spec::is_comp(#[trigger] models(fs@)[x], "formula"@, 3),
            models(fs@) == spec::top_formulas(item_models(citems@)),
            i <= fs.len(),
            spec::formula_pass(
                src@,
                anch_models(an@),
                item_models(aitems@),
                item_models(citems@),
                models(ordered@),
                map_model(map@),
                env.docid@,
                env.s as nat,
                k as nat,
                v as nat,
            ) == spec::formula_steps(
                src@,
                anch_models(an@),
                models(fs@),
                i as nat,
                item_models(aitems@),
                fpass_model(&acc),
                vv as nat,
            ),
        decreases fs.len() - i,
    {
        proof {
            reveal_with_fuel(spec::formula_steps, 1);
        }
        let ghost before = arena.nodes@;
        proof {
            assert(valid(arena.nodes@, &fs@[i as int]));
            assert(models(fs@)[i as int] == fs@[i as int]@);
        }
        match formula_step(arena, src, an, &fs, &fs[i], aitems, acc, vv) {
            Err(e) => return Err(e),
            Ok((next, v2)) => {
                acc = next;
                vv = v2;
            },
        }
        proof {
            prefix_all(before, arena.nodes@, fs@);
            prefix_all(before, arena.nodes@, src.toks@);
            items_prefix(before, arena.nodes@, aitems@);
            assert forall|x: int| 0 <= x < an.len() implies valid(
                arena.nodes@,
                &(#[trigger] an@[x]).inner,
            ) && valid(arena.nodes@, &an@[x].k) && an@[x].k@ is Int by {
                prefix(before, arena.nodes@, &an@[x].inner);
                prefix(before, arena.nodes@, &an@[x].k);
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::formula_steps, 1);
    }
    Ok(acc)
}

} // verus!
