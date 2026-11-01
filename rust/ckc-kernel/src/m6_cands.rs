#[cfg(verus_keep_ghost)]
use crate::arith_impl::rat_ok;
use crate::arith_impl::{ERat, canon_push, decode_number};
use crate::arith_mag::{mag_add, mag_mul, mag_sub, zero};
use crate::compare_impl::scaled_value;
use crate::k2_term::{ENode, EOrder, ETermArena, int_order};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
use crate::m6_model::*;
use crate::m6_symbols::{Sym, symbol_bytes};
use crate::m6_term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::arith as a;
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::udec_bytes;
use ckc_spec::witness as ws;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

// Exact rationals for the witness candidates (contract ws D2/D3).
// a + b over signed decimal magnitudes.
fn sadd(am: &Vec<u8>, an: bool, x: Ghost<int>, bm: &Vec<u8>, bn: bool, y: Ghost<int>) -> (out: (
    Vec<u8>,
    bool,
))
    requires
        am@ == udec_bytes(a::abs(x@)),
        an == (x@ < 0),
        bm@ == udec_bytes(a::abs(y@)),
        bn == (y@ < 0),
    ensures
        out.0@ == udec_bytes(a::abs(x@ + y@)),
        out.1 == (x@ + y@ < 0),
{
    let ghost ax = a::abs(x@);
    let ghost ay = a::abs(y@);
    if an == bn {
        let m = mag_add(am, Ghost(ax), bm, Ghost(ay));
        proof {
            assert(a::abs(x@ + y@) == ax + ay);
        }
        (m, an)
    } else {
        let order = int_order(am, false, Ghost(ax as int), bm, false, Ghost(ay as int));
        match order {
            EOrder::Greater => {
                let m = mag_sub(am, Ghost(ax), bm, Ghost(ay));
                proof {
                    assert(a::abs(x@ + y@) == (ax - ay) as nat);
                }
                (m, an)
            },
            EOrder::Less => {
                let m = mag_sub(bm, Ghost(ay), am, Ghost(ax));
                proof {
                    assert(a::abs(x@ + y@) == (ay - ax) as nat);
                }
                (m, bn)
            },
            EOrder::Equal => {
                proof {
                    assert(x@ + y@ == 0);
                }
                (zero(), false)
            },
        }
    }
}

pub fn number_of(arena: &ETermArena, x: &T) -> (out: Option<ERat>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, x),
    ensures
        match out {
            Some(r) => rat_ok(&r) && a::number(x@) == Some((r.n@, r.d@ as int)),
            None => a::number(x@) is None,
        },
{
    proof {
        assert(root_ok(arena, x.root)) by {
            reveal(arena_ok);
        }
    }
    decode_number(arena, x.root)
}

// `ws::plus` for k ∈ {−2, −1, 1, 2}.
pub fn plus(arena: &mut ETermArena, x: &T, k: i8) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, x),
        k == -2 || k == -1 || k == 1 || k == 2,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == ws::plus(x@, k as int),
{
    match number_of(arena, x) {
        None => x.cp(),
        Some(r) => {
            let ghost kd = (k as int) * (r.d@ as int);
            let kq = if k == 1 || k == -1 {
                r.den.clone()
            } else {
                mag_add(&r.den, r.d, &r.den, r.d)
            };
            proof {
                assert(a::abs(kd) == if k == 1 || k == -1 {
                    r.d@
                } else {
                    r.d@ + r.d@
                }) by (nonlinear_arith)
                    requires
                        kd == (k as int) * (r.d@ as int),
                        r.d@ >= 1,
                        k == -2 || k == -1 || k == 1 || k == 2,
                ;
                assert((kd < 0) == (k < 0)) by (nonlinear_arith)
                    requires
                        kd == (k as int) * (r.d@ as int),
                        r.d@ >= 1,
                ;
            }
            let (m, neg) = sadd(&r.num, r.neg, r.n, &kq, k < 0, Ghost(kd));
            let s = ERat { neg, num: m, den: r.den.clone(), n: Ghost(r.n@ + kd), d: r.d };
            let ghost before = arena.nodes@;
            let root = canon_push(arena, &s);
            from_root(arena, root)
        },
    }
}

// `ws::mid`.
pub fn mid(arena: &mut ETermArena, x: &T, y: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, x),
        valid(old(arena).nodes@, y),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == ws::mid(x@, y@),
{
    match (number_of(arena, x), number_of(arena, y)) {
        (Some(p), Some(q)) => {
            let (am, an) = scaled_value(&p, &q.den, q.d);
            let (bm, bn) = scaled_value(&q, &p.den, p.d);
            let (m, neg) = sadd(
                &am,
                an,
                Ghost(p.n@ * (q.d@ as int)),
                &bm,
                bn,
                Ghost(q.n@ * (p.d@ as int)),
            );
            let dd = mag_mul(&p.den, p.d, &q.den, q.d);
            let d2 = mag_add(&dd, Ghost(p.d@ * q.d@), &dd, Ghost(p.d@ * q.d@));
            let ghost den = 2 * p.d@ * q.d@;
            proof {
                assert(p.d@ * q.d@ + p.d@ * q.d@ == den) by (nonlinear_arith)
                    requires
                        den == 2 * p.d@ * q.d@,
                ;
                assert(den >= 1) by (nonlinear_arith)
                    requires
                        den == 2 * p.d@ * q.d@,
                        p.d@ >= 1,
                        q.d@ >= 1,
                ;
            }
            let s = ERat {
                neg,
                num: m,
                den: d2,
                n: Ghost(p.n@ * (q.d@ as int) + q.n@ * (p.d@ as int)),
                d: Ghost(den),
            };
            let root = canon_push(arena, &s);
            proof {
                assert(2 * (p.d@ as int) * (q.d@ as int) == den as int);
            }
            from_root(arena, root)
        },
        _ => x.cp(),
    }
}

// `ws::lt`.
pub fn lt(arena: &ETermArena, x: &T, y: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, x),
        valid(arena.nodes@, y),
    ensures
        out == ws::lt(x@, y@),
{
    match (number_of(arena, x), number_of(arena, y)) {
        (Some(p), Some(q)) => {
            let (am, an) = scaled_value(&p, &q.den, q.d);
            let (bm, bn) = scaled_value(&q, &p.den, p.d);
            let order = int_order(
                &am,
                an,
                Ghost(p.n@ * (q.d@ as int)),
                &bm,
                bn,
                Ghost(q.n@ * (p.d@ as int)),
            );
            match order {
                EOrder::Less => true,
                _ => false,
            }
        },
        _ => false,
    }
}

// `ws::admits`: the comparison the operand's clause runs on its count.
pub fn admits(arena: &mut ETermArena, cond: &Option<(T, T)>, v: &T) -> (out: bool)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, v),
        match cond {
            Some((op, b)) => valid(old(arena).nodes@, op) && valid(old(arena).nodes@, b),
            None => true,
        },
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ws::admits(
            match cond {
                Some((op, b)) => Some((op@, b@)),
                None => None,
            },
            v@,
        ),
{
    if number_of(arena, v).is_none() {
        return false;
    }
    match cond {
        None => true,
        Some((op, b)) => {
            let ghost before = arena.nodes@;
            let ex = named(arena, &Sym::Exactly);
            proof {
                prefix(before, arena.nodes@, v);
                prefix(before, arena.nodes@, op);
                prefix(before, arena.nodes@, b);
                reveal_strlit("exactly");
                reveal_strlit("guideline_satisfies");
            }
            let mut rs: Vec<usize> = Vec::new();
            rs.push(ex.root);
            rs.push(v.root);
            rs.push(op.root);
            rs.push(b.root);
            let name = symbol_bytes(&Sym::GuidelineSatisfies);
            proof {
                assert(crate::k2_engine::roots_valid(arena.nodes@, rs@)) by {
                    reveal(crate::k2_engine::roots_valid);
                }
                assert(crate::k2_engine::root_terms(arena.nodes@, rs@) =~= seq![ex@, v@, op@, b@]);
                assert(!ckc_spec::arith::is_arith(name@, 4));
            }
            crate::compare_impl::holds_exec(arena, &name, &rs)
        },
    }
}

// `ws::w0`.
pub fn w0(arena: &mut ETermArena, cond: &Option<(T, T)>) -> (out: T)
    requires
        arena_ok(old(arena)),
        match cond {
            Some((op, b)) => valid(old(arena).nodes@, op) && valid(old(arena).nodes@, b),
            None => true,
        },
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == ws::w0(
            match cond {
                Some((op, b)) => Some((op@, b@)),
                None => None,
            },
        ),
{
    match cond {
        None => crate::m6_term::int(arena, 1),
        Some((op, b)) => match number_of(arena, b) {
            None => b.cp(),
            Some(r) => {
                let geq = is_atom(arena, op, &Sym::Geq);
                let leq = is_atom(arena, op, &Sym::Leq);
                let greater = is_atom(arena, op, &Sym::Greater);
                let less = is_atom(arena, op, &Sym::Less);
                let pz = crate::arith_mag::is_zero(&r.num, Ghost(a::abs(r.n@)));
                proof {
                    reveal_strlit("geq");
                    reveal_strlit("leq");
                    reveal_strlit("greater");
                    reveal_strlit("less");
                }
                if geq && pz {
                    crate::m6_term::int(arena, 1)
                } else if leq && pz {
                    neg_one(arena)
                } else if greater {
                    let (s, _) = sadd(&r.num, r.neg, r.n, &r.den, false, Ghost(r.d@ as int));
                    let m1 = crate::arith_mag::is_zero(&s, Ghost(a::abs(r.n@ + r.d@)));
                    plus(
                        arena,
                        b,
                        if m1 {
                            2
                        } else {
                            1
                        },
                    )
                } else if less {
                    let (s, _) = sadd(&r.num, r.neg, r.n, &r.den, true, Ghost(-(r.d@ as int)));
                    let p1 = crate::arith_mag::is_zero(&s, Ghost(a::abs(r.n@ - r.d@)));
                    plus(
                        arena,
                        b,
                        if p1 {
                            -2
                        } else {
                            -1
                        },
                    )
                } else {
                    b.cp()
                }
            },
        },
    }
}

fn neg_one(arena: &mut ETermArena) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Int(-1),
{
    let one = crate::arith_mag::one();
    let s = ERat { neg: true, num: one, den: crate::arith_mag::one(), n: Ghost(-1), d: Ghost(1) };
    let root = canon_push(arena, &s);
    proof {
        assert(a::gcd(1, 1) == 1) by {
            reveal_with_fuel(a::gcd, 3);
        }
        assert(a::canon(-1, 1) == Term::Int(-1)) by (compute);
    }
    from_root(arena, root)
}

// --- ordered candidate sets ---
// xs[..i] ++ [x] ++ xs[i..].
fn splice(xs: &Vec<T>, i: usize, x: &T) -> (out: Vec<T>)
    requires
        i <= xs.len(),
    ensures
        out@ == xs@.take(i as int) + seq![*x] + xs@.skip(i as int),
        models(out@) == models(xs@).take(i as int) + seq![x@] + models(xs@).skip(i as int),
{
    let mut out: Vec<T> = Vec::new();
    let mut j = 0usize;
    while j < i
        invariant
            j <= i,
            i <= xs.len(),
            out@ == xs@.take(j as int),
        decreases i - j,
    {
        out.push(xs[j].cp());
        proof {
            assert(xs@.take(j + 1) == xs@.take(j as int).push(xs@[j as int]));
        }
        j += 1;
    }
    out.push(x.cp());
    let mut k = i;
    while k < xs.len()
        invariant
            i <= k <= xs.len(),
            out@ == xs@.take(i as int) + seq![*x] + xs@.subrange(i as int, k as int),
        decreases xs.len() - k,
    {
        out.push(xs[k].cp());
        proof {
            assert(xs@.subrange(i as int, k + 1) == xs@.subrange(i as int, k as int).push(
                xs@[k as int],
            ));
        }
        k += 1;
    }
    proof {
        assert(xs@.subrange(i as int, xs.len() as int) == xs@.skip(i as int));
        assert_seqs_equal!(models(out@) == models(xs@).take(i as int) + seq![x@] + models(xs@).skip(i as int));
    }
    out
}

// `ws::insert`.
pub fn insert(arena: &ETermArena, xs: &Vec<T>, x: &T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, xs@),
        valid(arena.nodes@, x),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == ws::insert(models(xs@), x@),
{
    let mut i = 0usize;
    proof {
        assert(models(xs@).take(0) + models(xs@).skip(0) =~= models(xs@));
    }
    while i < xs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, xs@),
            valid(arena.nodes@, x),
            i <= xs.len(),
            ws::insert(models(xs@), x@) == models(xs@).take(i as int) + ws::insert(
                models(xs@).skip(i as int),
                x@,
            ),
        decreases xs.len() - i,
    {
        proof {
            assert(models(xs@).skip(i as int)[0] == xs@[i as int]@);
            assert(models(xs@).skip(i as int).drop_first() =~= models(xs@).skip(i as int + 1));
            reveal_with_fuel(ws::insert, 1);
        }
        if crate::m6_term::equal(arena, &xs[i], x) {
            proof {
                assert(models(xs@).take(i as int) + models(xs@).skip(i as int) =~= models(xs@));
            }
            return copy(xs);
        }
        if lt(arena, x, &xs[i]) {
            let out = splice(xs, i, x);
            proof {
                assert forall|j: int| 0 <= j < out.len() implies #[trigger] valid(
                    arena.nodes@,
                    &out@[j],
                ) by {
                    if j < i {
                        assert(out@[j] == xs@[j]);
                    } else if j > i {
                        assert(out@[j] == xs@[j - 1]);
                    }
                }
            }
            return out;
        }
        proof {
            assert(models(xs@).take(i as int + 1) =~= models(xs@).take(i as int) + seq![
                xs@[i as int]@,
            ]);
        }
        i += 1;
    }
    let out = splice(xs, i, x);
    proof {
        reveal_with_fuel(ws::insert, 1);
        assert(models(xs@).skip(i as int) =~= Seq::<Term>::empty());
        assert forall|j: int| 0 <= j < out.len() implies #[trigger] valid(
            arena.nodes@,
            &out@[j],
        ) by {
            if j < i {
                assert(out@[j] == xs@[j]);
            }
        }
    }
    out
}

// `ws::ascending`.
pub fn ascending(arena: &ETermArena, xs: &Vec<T>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, xs@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == ws::ascending(models(xs@)),
{
    let mut acc: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        reveal_with_fuel(ws::ascending, 1);
        assert(models(xs@).take(0) =~= Seq::<Term>::empty());
        assert_seqs_equal!(models(acc@) == Seq::<Term>::empty());
    }
    while i < xs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, xs@),
            valid_all(arena.nodes@, acc@),
            i <= xs.len(),
            models(acc@) == ws::ascending(models(xs@).take(i as int)),
        decreases xs.len() - i,
    {
        proof {
            let s = models(xs@).take(i as int + 1);
            assert(s.drop_last() =~= models(xs@).take(i as int));
            assert(s.last() == xs@[i as int]@);
            reveal_with_fuel(ws::ascending, 1);
        }
        acc = insert(arena, &acc, &xs[i]);
        i += 1;
    }
    proof {
        assert(models(xs@).take(xs.len() as int) =~= models(xs@));
    }
    acc
}

// b ± k over a sequence (`ws::plus` mapped).
pub fn plus_all(arena: &mut ETermArena, bs: &Vec<T>, k: i8) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, bs@),
        k == -2 || k == -1 || k == 1 || k == 2,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        models(out@) == models(bs@).map_values(|b: Term| ws::plus(b, k as int)),
{
    let ghost start = arena.nodes@;
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    while i < bs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, bs@),
            valid_all(arena.nodes@, out@),
            k == -2 || k == -1 || k == 1 || k == 2,
            i <= bs.len(),
            out.len() == i,
            forall|j: int| 0 <= j < i ==> (#[trigger] out@[j])@ == ws::plus(bs@[j]@, k as int),
        decreases bs.len() - i,
    {
        let ghost before = arena.nodes@;
        let y = plus(arena, &bs[i], k);
        proof {
            prefix_all(before, arena.nodes@, bs@);
            prefix_all(before, arena.nodes@, out@);
            extend(arena.nodes@, out@, y);
        }
        out.push(y);
        i += 1;
    }
    proof {
        assert_seqs_equal!(models(out@) == models(bs@).map_values(|b: Term| ws::plus(b, k as int)));
    }
    out
}

// `ws::midpoints`.
pub fn midpoints(arena: &mut ETermArena, bs: &Vec<T>) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, bs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        models(out@) == ws::midpoints(models(bs@)),
{
    let ghost start = arena.nodes@;
    let mut out: Vec<T> = Vec::new();
    let n = if bs.len() == 0 {
        0
    } else {
        bs.len() - 1
    };
    let mut m = 0usize;
    while m < n
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, bs@),
            valid_all(arena.nodes@, out@),
            n == (if bs.len() == 0 {
                0
            } else {
                bs.len() - 1
            }),
            m <= n,
            out.len() == m,
            forall|j: int| 0 <= j < m ==> (#[trigger] out@[j])@ == ws::mid(bs@[j]@, bs@[j + 1]@),
        decreases n - m,
    {
        let ghost before = arena.nodes@;
        let y = mid(arena, &bs[m], &bs[m + 1]);
        proof {
            prefix_all(before, arena.nodes@, bs@);
            prefix_all(before, arena.nodes@, out@);
            extend(arena.nodes@, out@, y);
        }
        out.push(y);
        m += 1;
    }
    proof {
        assert_seqs_equal!(models(out@) == ws::midpoints(models(bs@)));
    }
    out
}

// --- the skeleton + NAF items (D3) ---
pub open spec fn seqs_models(xs: Seq<Vec<T>>) -> Seq<Seq<Term>> {
    xs.map_values(|v: Vec<T>| models(v@))
}

pub open spec fn seqs_valid(nodes: Seq<ENode>, xs: Seq<Vec<T>>) -> bool {
    forall|i: int| 0 <= i < xs.len() ==> valid_all(nodes, (#[trigger] xs[i])@)
}

pub proof fn seqs_prefix(before: Seq<ENode>, after: Seq<ENode>, xs: Seq<Vec<T>>)
    requires
        before.is_prefix_of(after),
        seqs_valid(before, xs),
    ensures
        seqs_valid(after, xs),
{
    assert forall|i: int| 0 <= i < xs.len() implies valid_all(after, (#[trigger] xs[i])@) by {
        prefix_all(before, after, xs[i]@);
    }
}

pub fn counts(sel: &ESel) -> (out: Vec<T>)
    ensures
        out.len() == sel.ops.len(),
        forall|j: int| 0 <= j < out.len() ==> (#[trigger] out@[j]) == sel.ops@[j].n,
        models(out@) == ws::counts(sel@),
{
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    while i < sel.ops.len()
        invariant
            i <= sel.ops.len(),
            out.len() == i,
            forall|j: int| 0 <= j < i ==> (#[trigger] out@[j]) == sel.ops@[j].n,
        decreases sel.ops.len() - i,
    {
        out.push(sel.ops[i].n.cp());
        i += 1;
    }
    proof {
        assert_seqs_equal!(models(out@) == ws::counts(sel@));
    }
    out
}

// `ws::open_pairs`.
pub fn open_pairs(arena: &ETermArena, g: &Group, sel: &ESel, cs: &Vec<T>) -> (out: Vec<Binding>)
    requires
        arena_ok(arena),
        map_valid(arena.nodes@, g.pairs@),
        valid_all(arena.nodes@, cs@),
        models(cs@) == ws::counts(sel@),
    ensures
        map_valid(arena.nodes@, out@),
        map_model(out@) == ws::open_pairs(g@, sel@),
{
    let ghost pred = |p: (Term, Term)| !ws::counts(sel@).contains(p.0);
    let mut out: Vec<Binding> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(map_model(g.pairs@).take(0) =~= Seq::<(Term, Term)>::empty());
        assert_seqs_equal!(map_model(out@) == Seq::<(Term, Term)>::empty());
        reveal(Seq::filter);
    }
    while i < g.pairs.len()
        invariant
            arena_ok(arena),
            map_valid(arena.nodes@, g.pairs@),
            valid_all(arena.nodes@, cs@),
            models(cs@) == ws::counts(sel@),
            map_valid(arena.nodes@, out@),
            i <= g.pairs.len(),
            pred == (|p: (Term, Term)| !ws::counts(sel@).contains(p.0)),
            map_model(out@) == map_model(g.pairs@).take(i as int).filter(pred),
        decreases g.pairs.len() - i,
    {
        proof {
            assert(binding_valid(arena.nodes@, &g.pairs@[i as int]));
            assert(map_model(g.pairs@).take(i as int + 1) == map_model(g.pairs@).take(
                i as int,
            ).push(map_model(g.pairs@)[i as int]));
            map_model(g.pairs@).take(i as int).lemma_filter_push(
                map_model(g.pairs@)[i as int],
                pred,
            );
            assert(map_model(g.pairs@)[i as int].0 == g.pairs@[i as int].key@);
        }
        if !crate::m6_refs::contains(arena, cs, &g.pairs[i].key) {
            let b = Binding { key: g.pairs[i].key.cp(), value: g.pairs[i].value.cp() };
            let ghost before = out@;
            out.push(b);
            proof {
                assert_seqs_equal!(map_model(out@) == map_model(before).push(b@));
                assert forall|j: int| 0 <= j < out.len() implies #[trigger] binding_valid(
                    arena.nodes@,
                    &out@[j],
                ) by {
                    if j < before.len() {
                        assert(out@[j] == before[j]);
                    }
                }
            }
        }
        i += 1;
    }
    proof {
        assert(map_model(g.pairs@).take(i as int) =~= map_model(g.pairs@));
        assert(g@.pairs == map_model(g.pairs@));
    }
    out
}

// `ws::skeleton`, given the open pairs.
pub fn skeleton(arena: &mut ETermArena, g: &Group, sel: &ESel, op: &Vec<Binding>) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        map_valid(old(arena).nodes@, op@),
        map_model(op@) == ws::open_pairs(g@, sel@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        models(out@) == ws::skeleton(g@, sel@),
{
    if g.clauses.len() == 0 {
        let out: Vec<T> = Vec::new();
        proof {
            assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
        }
        out
    } else {
        proof {
            assert(clause_valid(arena.nodes@, &g.clauses@[0]));
            assert(g@.clauses[0] == g.clauses@[0]@);
        }
        let gs = crate::m6_safety::positive_goals(arena, &g.clauses[0].body);
        crate::m6_payload::bind_terms(arena, &gs, op)
    }
}

// `ws::naf_items`, given the open pairs.
pub fn naf_items(arena: &mut ETermArena, g: &Group, sel: &ESel, op: &Vec<Binding>) -> (out: Vec<
    Vec<T>,
>)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        map_valid(old(arena).nodes@, op@),
        map_model(op@) == ws::open_pairs(g@, sel@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        seqs_valid(final(arena).nodes@, out@),
        seqs_models(out@) == ws::naf_items(g@, sel@),
{
    let ghost start = arena.nodes@;
    let mut out: Vec<Vec<T>> = Vec::new();
    if g.clauses.len() == 0 {
        proof {
            assert_seqs_equal!(seqs_models(out@) == Seq::<Seq<Term>>::empty());
        }
        return out;
    }
    proof {
        assert(clause_valid(arena.nodes@, &g.clauses@[0]));
        assert(g@.clauses[0] == g.clauses@[0]@);
    }
    let body = &g.clauses[0].body;
    let ghost bm = body_models(body@);
    let ghost pairs = ws::open_pairs(g@, sel@);
    let mut i = 0usize;
    proof {
        assert(bm.skip(0) =~= bm);
        assert_seqs_equal!(seqs_models(out@) + ws::nafs_of(bm, pairs) == ws::nafs_of(bm, pairs));
    }
    while i < body.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            g.clauses.len() > 0,
            body == &g.clauses@[0].body,
            bm == body_models(body@),
            bodies_valid(arena.nodes@, body@),
            map_valid(arena.nodes@, op@),
            map_model(op@) == pairs,
            pairs == ws::open_pairs(g@, sel@),
            seqs_valid(arena.nodes@, out@),
            i <= body.len(),
            ws::nafs_of(bm, pairs) == seqs_models(out@) + ws::nafs_of(bm.skip(i as int), pairs),
        decreases body.len() - i,
    {
        proof {
            assert(bm.skip(i as int)[0] == bm[i as int]);
            assert(bm.skip(i as int).drop_first() =~= bm.skip(i as int + 1));
            reveal_with_fuel(ws::nafs_of, 1);
            assert(body_valid(arena.nodes@, &body@[i as int]));
        }
        match &body[i] {
            Body::Naf(gs) => {
                let ghost before = arena.nodes@;
                let ghost prev = out@;
                let bound = crate::m6_payload::bind_terms(arena, gs, op);
                proof {
                    seqs_prefix(before, arena.nodes@, out@);
                    map_prefix(before, arena.nodes@, op@);
                    bodies_prefix(before, arena.nodes@, body@);
                }
                out.push(bound);
                proof {
                    assert(bm[i as int] == ckc_spec::v1text::BodyItem::Naf(models(gs@)));
                    assert(models(bound@) =~= models(gs@).map_values(
                        |t: Term| ckc_spec::emit::bind_all(t, pairs),
                    ));
                    assert_seqs_equal!(seqs_models(out@) == seqs_models(prev) + seq![models(bound@)], j => {
                        if j < prev.len() {
                            assert(out@[j] == prev[j]);
                        }
                    });
                    assert forall|j: int| 0 <= j < out.len() implies valid_all(
                        arena.nodes@,
                        (#[trigger] out@[j])@,
                    ) by {
                        if j < prev.len() {
                            assert(out@[j] == prev[j]);
                        }
                    }
                }
            },
            Body::Pos(_) => {},
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(ws::nafs_of, 1);
        assert(bm.skip(i as int) =~= Seq::<ckc_spec::v1text::BodyItem>::empty());
        assert(g@.clauses[0].body == bm);
        assert_seqs_equal!(seqs_models(out@) + Seq::<Seq<Term>>::empty() == seqs_models(out@));
    }
    out
}

// --- the D3 join + relevance ---
// ts[a..b].
fn sub(ts: &Vec<T>, a: usize, b: usize) -> (out: Vec<T>)
    requires
        a <= b <= ts.len(),
    ensures
        out@ == ts@.subrange(a as int, b as int),
{
    let mut out: Vec<T> = Vec::new();
    let mut i = a;
    while i < b
        invariant
            a <= i <= b,
            b <= ts.len(),
            out@ == ts@.subrange(a as int, i as int),
        decreases b - i,
    {
        out.push(ts[i].cp());
        proof {
            assert(ts@.subrange(a as int, i + 1) == ts@.subrange(a as int, i as int).push(
                ts@[i as int],
            ));
        }
        i += 1;
    }
    out
}

proof fn sub_valid(nodes: Seq<ENode>, ts: Seq<T>, a: int, b: int)
    requires
        0 <= a <= b <= ts.len(),
        valid_all(nodes, ts),
    ensures
        valid_all(nodes, ts.subrange(a, b)),
        models(ts.subrange(a, b)) == models(ts).subrange(a, b),
{
    assert(models(ts.subrange(a, b)) =~= models(ts).subrange(a, b));
}

// `ws::joins`.
pub fn joins(arena: &mut ETermArena, n: usize, gs: &Vec<T>, facts: &Vec<T>, w: &Vec<T>) -> (out:
    Vec<Vec<T>>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, gs@),
        valid_all(old(arena).nodes@, facts@),
        valid_all(old(arena).nodes@, w@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        seqs_valid(final(arena).nodes@, out@),
        seqs_models(out@) == ws::joins(n as nat, models(gs@), models(facts@), models(w@)),
    decreases n,
{
    if n == 0 || gs.len() == 0 {
        let mut out: Vec<Vec<T>> = Vec::new();
        out.push(copy(w));
        proof {
            reveal_with_fuel(ws::joins, 1);
            assert_seqs_equal!(seqs_models(out@) == seq![models(w@)]);
        }
        return out;
    }
    let ghost start = arena.nodes@;
    let ghost gm = models(gs@);
    let ghost fm = models(facts@);
    let ghost wm = models(w@);
    let mut out: Vec<Vec<T>> = Vec::new();
    let k = gs.len() - 1;
    let rest = sub(gs, 1, gs.len());
    let front = concat(w, &rest);
    let all = concat(&front, facts);
    proof {
        sub_valid(arena.nodes@, gs@, 1, gs.len() as int);
        assert(models(rest@) =~= gm.drop_first());
        reveal_with_fuel(ws::joins, 1);
        assert_seqs_equal!(seqs_models(out@) + ws::joins_from(n as nat, gm, fm, wm, 0) == ws::joins_from(n as nat, gm, fm, wm, 0));
    }
    let mut j = 0usize;
    while j < facts.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, gs@),
            valid_all(arena.nodes@, facts@),
            valid_all(arena.nodes@, w@),
            valid_all(arena.nodes@, all@),
            gm == models(gs@),
            fm == models(facts@),
            wm == models(w@),
            models(all@) == wm + gm.drop_first() + fm,
            seqs_valid(arena.nodes@, out@),
            n > 0,
            gs.len() > 0,
            k == gs.len() - 1,
            j <= facts.len(),
            ws::joins(n as nat, gm, fm, wm) == ws::joins_from(n as nat, gm, fm, wm, 0),
            ws::joins_from(n as nat, gm, fm, wm, 0) == seqs_models(out@) + ws::joins_from(
                n as nat,
                gm,
                fm,
                wm,
                j as nat,
            ),
        decreases facts.len() - j,
    {
        let ghost here_nodes = arena.nodes@;
        let sol = roots(arena, &all);
        let mut pairs: Vec<crate::k2_engine::EPair> = Vec::new();
        pairs.push(crate::k2_engine::EPair { left: gs[0].root, right: facts[j].root });
        let st = crate::k2_engine::EBoundState { pairs, stack: Vec::new(), sol };
        let ghost u = ckc_spec::engine::UState {
            pairs: seq![(gm[0], fm[j as int])],
            stack: Seq::empty(),
            sol: wm + gm.drop_first() + fm,
        };
        proof {
            assert(valid(arena.nodes@, &gs@[0]));
            assert(valid(arena.nodes@, &facts@[j as int]));
            assert(crate::k2_engine::bound_state_valid(arena.nodes@, &st)) by {
                reveal(crate::k2_engine::roots_valid);
            }
            assert(crate::k2_engine::pairs_view(arena.nodes@, st.pairs@) =~= seq![
                (gm[0], fm[j as int]),
            ]);
            assert(crate::k2_engine::goals_view(arena.nodes@, st.stack@) =~= Seq::<
                ckc_spec::engine::Goal,
            >::empty());
            assert(crate::k2_engine::bound_state_view(arena.nodes@, &st) == u);
            crate::k3_sound::naf::unify_output_len(u);
            reveal_with_fuel(ws::joins_from, 1);
        }
        let res = crate::k2_engine::unify(arena, st);
        proof {
            prefix_all(here_nodes, arena.nodes@, gs@);
            prefix_all(here_nodes, arena.nodes@, facts@);
            prefix_all(here_nodes, arena.nodes@, w@);
            prefix_all(here_nodes, arena.nodes@, all@);
            seqs_prefix(here_nodes, arena.nodes@, out@);
        }
        match res {
            crate::k2_engine::EUResult::Ok { stack: _, sol: s } => {
                let hs = from_roots(arena, &s);
                let ghost sm = models(hs@);
                proof {
                    assert(sm.len() == wm.len() + k + fm.len());
                }
                let w2 = sub(&hs, 0, w.len());
                let r2 = sub(&hs, w.len(), w.len() + k);
                let f2 = sub(&hs, w.len() + k, hs.len());
                proof {
                    sub_valid(arena.nodes@, hs@, 0, w.len() as int);
                    sub_valid(arena.nodes@, hs@, w.len() as int, w.len() + k);
                    sub_valid(arena.nodes@, hs@, w.len() + k, hs.len() as int);
                    assert(sm.take(wm.len() as int) =~= sm.subrange(0, wm.len() as int));
                    assert(sm.skip(wm.len() + k) =~= sm.subrange(wm.len() + k, sm.len() as int));
                }
                let ghost mid_nodes = arena.nodes@;
                let ghost prev = out@;
                let found = joins(arena, n - 1, &r2, &f2, &w2);
                proof {
                    prefix_all(mid_nodes, arena.nodes@, gs@);
                    prefix_all(mid_nodes, arena.nodes@, facts@);
                    prefix_all(mid_nodes, arena.nodes@, w@);
                    prefix_all(mid_nodes, arena.nodes@, all@);
                    seqs_prefix(mid_nodes, arena.nodes@, out@);
                }
                let mut q = 0usize;
                while q < found.len()
                    invariant
                        q <= found.len(),
                        seqs_valid(arena.nodes@, found@),
                        seqs_valid(arena.nodes@, out@),
                        seqs_models(out@) == seqs_models(prev) + seqs_models(found@).take(q as int),
                    decreases found.len() - q,
                {
                    let ghost before = out@;
                    out.push(copy(&found[q]));
                    proof {
                        assert(seqs_models(found@).take(q as int + 1) =~= seqs_models(found@).take(
                            q as int,
                        ).push(models(found@[q as int]@)));
                        assert_seqs_equal!(seqs_models(out@) == seqs_models(before).push(models(found@[q as int]@)), x => {
                            if x < before.len() { assert(out@[x] == before[x]); }
                        });
                        assert forall|x: int| 0 <= x < out.len() implies valid_all(
                            arena.nodes@,
                            (#[trigger] out@[x])@,
                        ) by {
                            if x < before.len() {
                                assert(out@[x] == before[x]);
                            }
                        }
                    }
                    q += 1;
                }
                proof {
                    assert(seqs_models(found@).take(found.len() as int) =~= seqs_models(found@));
                }
            },
            crate::k2_engine::EUResult::Fail => {},
        }
        proof {
            reveal_with_fuel(ws::joins_from, 1);
        }
        j += 1;
    }
    proof {
        reveal_with_fuel(ws::joins_from, 1);
        assert_seqs_equal!(seqs_models(out@) + Seq::<Seq<Term>>::empty() == seqs_models(out@));
    }
    out
}

// `ws::semantic_goals`.
fn semantic(arena: &ETermArena, gs: &Vec<T>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, gs@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == ws::semantic_goals(models(gs@)),
{
    let ghost gm = models(gs@);
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(gm.skip(0) =~= gm);
        assert_seqs_equal!(models(out@) + ws::semantic_goals(gm) == ws::semantic_goals(gm));
    }
    while i < gs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, gs@),
            valid_all(arena.nodes@, out@),
            gm == models(gs@),
            i <= gs.len(),
            ws::semantic_goals(gm) == models(out@) + ws::semantic_goals(gm.skip(i as int)),
        decreases gs.len() - i,
    {
        proof {
            assert(gm.skip(i as int)[0] == gs@[i as int]@);
            assert(gm.skip(i as int).drop_first() =~= gm.skip(i as int + 1));
            reveal_with_fuel(ws::semantic_goals, 1);
        }
        if !crate::m6_safety::is_builtin_lit(arena, &gs[i]) {
            let next = gs[i].cp();
            let ghost prev = out@;
            proof {
                extend(arena.nodes@, out@, next);
            }
            out.push(next);
            proof {
                assert_seqs_equal!(models(out@) == models(prev) + seq![next@]);
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(ws::semantic_goals, 1);
        assert(gm.skip(i as int) =~= Seq::<Term>::empty());
        assert_seqs_equal!(models(out@) + Seq::<Term>::empty() == models(out@));
    }
    out
}

// `ws::relevant`.
pub fn relevant(arena: &mut ETermArena, gs: &Vec<T>, facts: &Vec<T>, c: &T, slot: &T) -> (out: bool)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, gs@),
        valid_all(old(arena).nodes@, facts@),
        valid(old(arena).nodes@, c),
        valid(old(arena).nodes@, slot),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ws::relevant(models(gs@), models(facts@), c@, slot@),
{
    let ghost start = arena.nodes@;
    let sem = semantic(arena, gs);
    let xs = args(arena, c);
    let d = if xs.len() > 1 {
        proof {
            assert(valid(arena.nodes@, &xs@[1]));
            assert(models(xs@)[1] == xs@[1]@);
            assert(ckc_spec::engine::args_of(c@).len() > 1);
            assert(c@ is Comp);
            assert(ckc_spec::replay::arg(c@, 1) == xs@[1]@);
        }
        xs[1].cp()
    } else {
        let ghost before = arena.nodes@;
        let n = nil(arena);
        proof {
            prefix_all(before, arena.nodes@, sem@);
            prefix_all(before, arena.nodes@, facts@);
            prefix(before, arena.nodes@, slot);
            assert(ckc_spec::replay::arg(c@, 1) == Term::Nil);
        }
        n
    };
    let mut w: Vec<T> = Vec::new();
    w.push(d);
    w.push(slot.cp());
    proof {
        assert_seqs_equal!(models(w@) == seq![ckc_spec::replay::arg(c@, 1), slot@]);
    }
    let js = joins(arena, sem.len(), &sem, facts, &w);
    let ghost jm = seqs_models(js@);
    let mut k = 0usize;
    let ghost spec_js = ws::joins(
        models(sem@).len() as nat,
        models(sem@),
        models(facts@),
        seq![ckc_spec::replay::arg(c@, 1), slot@],
    );
    proof {
        assert(models(sem@).len() == sem.len());
        assert(jm == spec_js);
    }
    while k < js.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            seqs_valid(arena.nodes@, js@),
            jm == seqs_models(js@),
            jm == spec_js,
            models(sem@) == ws::semantic_goals(models(gs@)),
            spec_js == ws::joins(
                ws::semantic_goals(models(gs@)).len(),
                ws::semantic_goals(models(gs@)),
                models(facts@),
                seq![ckc_spec::replay::arg(c@, 1), slot@],
            ),
            k <= js.len(),
            forall|q: int|
                0 <= q < k ==> !((#[trigger] jm[q]).len() == 2 && jm[q][1] is Var && jm[q][0]
                    == jm[q][1]),
        decreases js.len() - k,
    {
        let row = &js[k];
        proof {
            assert(jm[k as int] == models(row@));
        }
        if row.len() == 2 {
            proof {
                assert(valid_all(arena.nodes@, row@));
                assert(valid(arena.nodes@, &row@[0]));
                assert(valid(arena.nodes@, &row@[1]));
            }
            if is_var(arena, &row[1]) && crate::m6_term::equal(arena, &row[0], &row[1]) {
                proof {
                    assert(jm[k as int].len() == 2);
                    assert(jm[k as int][1] == row@[1]@);
                    assert(jm[k as int][0] == row@[0]@);
                    assert(spec_js[k as int].len() == 2 && spec_js[k as int][1] is Var
                        && spec_js[k as int][0] == spec_js[k as int][1]);
                    let sem_s = ws::semantic_goals(models(gs@));
                    let js_s = ws::joins(
                        sem_s.len(),
                        sem_s,
                        models(facts@),
                        seq![ckc_spec::replay::arg(c@, 1), slot@],
                    );
                    assert(js_s == spec_js);
                    assert(js_s[k as int].len() == 2 && js_s[k as int][1] is Var
                        && js_s[k as int][0] == js_s[k as int][1]);
                    assert(ws::relevant(models(gs@), models(facts@), c@, slot@));
                }
                return true;
            }
        }
        k += 1;
    }
    false
}

// `ws::item_bounds` over cs = gs[i..].
fn item_bounds(arena: &mut ETermArena, gs: &Vec<T>, facts: &Vec<T>, slot: &T) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, gs@),
        valid_all(old(arena).nodes@, facts@),
        valid(old(arena).nodes@, slot),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        models(out@) == ws::item_bounds(models(gs@), models(gs@), models(facts@), slot@),
{
    let ghost start = arena.nodes@;
    let ghost gm = models(gs@);
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(gm.skip(0) =~= gm);
        assert_seqs_equal!(models(out@) + ws::item_bounds(gm, gm, models(facts@), slot@) == ws::item_bounds(gm, gm, models(facts@), slot@));
        reveal_strlit("guideline_satisfies");
    }
    while i < gs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, gs@),
            valid_all(arena.nodes@, facts@),
            valid(arena.nodes@, slot),
            valid_all(arena.nodes@, out@),
            gm == models(gs@),
            i <= gs.len(),
            ws::item_bounds(gm, gm, models(facts@), slot@) == models(out@) + ws::item_bounds(
                gm,
                gm.skip(i as int),
                models(facts@),
                slot@,
            ),
        decreases gs.len() - i,
    {
        proof {
            assert(gm.skip(i as int)[0] == gs@[i as int]@);
            assert(gm.skip(i as int).drop_first() =~= gm.skip(i as int + 1));
            reveal_with_fuel(ws::item_bounds, 1);
            assert(valid(arena.nodes@, &gs@[i as int]));
        }
        let c = &gs[i];
        if is_comp(arena, c, &Sym::GuidelineSatisfies, 4) {
            let xs = args(arena, c);
            proof {
                assert(valid(arena.nodes@, &xs@[3]));
                assert(ckc_spec::replay::arg(c@, 3) == xs@[3]@);
            }
            if number_of(arena, &xs[3]).is_some() {
                let ghost before = arena.nodes@;
                let rel = relevant(arena, gs, facts, c, slot);
                proof {
                    prefix_all(before, arena.nodes@, gs@);
                    prefix_all(before, arena.nodes@, facts@);
                    prefix_all(before, arena.nodes@, out@);
                    prefix_all(before, arena.nodes@, xs@);
                    prefix(before, arena.nodes@, slot);
                }
                if rel {
                    let b = xs[3].cp();
                    let ghost prev = out@;
                    proof {
                        extend(arena.nodes@, out@, b);
                    }
                    out.push(b);
                    proof {
                        assert_seqs_equal!(models(out@) == models(prev) + seq![b@]);
                    }
                }
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(ws::item_bounds, 1);
        assert(gm.skip(i as int) =~= Seq::<Term>::empty());
        assert_seqs_equal!(models(out@) + Seq::<Term>::empty() == models(out@));
    }
    out
}

// `ws::bounds_of`.
pub fn bounds_of(arena: &mut ETermArena, nafs: &Vec<Vec<T>>, facts: &Vec<T>, slot: &T) -> (out: Vec<
    T,
>)
    requires
        arena_ok(old(arena)),
        seqs_valid(old(arena).nodes@, nafs@),
        valid_all(old(arena).nodes@, facts@),
        valid(old(arena).nodes@, slot),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        models(out@) == ws::bounds_of(seqs_models(nafs@), models(facts@), slot@),
{
    let ghost start = arena.nodes@;
    let ghost nm = seqs_models(nafs@);
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(nm.skip(0) =~= nm);
        assert_seqs_equal!(models(out@) + ws::bounds_of(nm, models(facts@), slot@) == ws::bounds_of(nm, models(facts@), slot@));
    }
    while i < nafs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            seqs_valid(arena.nodes@, nafs@),
            valid_all(arena.nodes@, facts@),
            valid(arena.nodes@, slot),
            valid_all(arena.nodes@, out@),
            nm == seqs_models(nafs@),
            i <= nafs.len(),
            ws::bounds_of(nm, models(facts@), slot@) == models(out@) + ws::bounds_of(
                nm.skip(i as int),
                models(facts@),
                slot@,
            ),
        decreases nafs.len() - i,
    {
        proof {
            assert(nm.skip(i as int)[0] == models(nafs@[i as int]@));
            assert(nm.skip(i as int).drop_first() =~= nm.skip(i as int + 1));
            reveal_with_fuel(ws::bounds_of, 1);
        }
        let ghost before = arena.nodes@;
        let part = item_bounds(arena, &nafs[i], facts, slot);
        proof {
            seqs_prefix(before, arena.nodes@, nafs@);
            prefix_all(before, arena.nodes@, facts@);
            prefix_all(before, arena.nodes@, out@);
            prefix(before, arena.nodes@, slot);
        }
        let ghost prev = out@;
        out = concat(&out, &part);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < prev.len() {
                    assert(out@[j] == prev[j]);
                } else {
                    assert(out@[j] == part@[j - prev.len()]);
                }
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(ws::bounds_of, 1);
        assert(nm.skip(i as int) =~= Seq::<Seq<Term>>::empty());
        assert_seqs_equal!(models(out@) + Seq::<Term>::empty() == models(out@));
    }
    out
}

pub open spec fn cond_model(cond: Option<(T, T)>) -> Option<(Term, Term)> {
    match cond {
        Some((op, b)) => Some((op@, b@)),
        None => None,
    }
}

pub open spec fn cond_valid(nodes: Seq<ENode>, cond: Option<(T, T)>) -> bool {
    match cond {
        Some((op, b)) => valid(nodes, &op) && valid(nodes, &b),
        None => true,
    }
}

// `ws::boundaries`, given the item bounds' inputs.
pub fn boundaries(
    arena: &mut ETermArena,
    cond: &Option<(T, T)>,
    nafs: &Vec<Vec<T>>,
    facts: &Vec<T>,
    slot: &T,
) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        cond_valid(old(arena).nodes@, *cond),
        seqs_valid(old(arena).nodes@, nafs@),
        valid_all(old(arena).nodes@, facts@),
        valid(old(arena).nodes@, slot),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        models(out@) == ws::ascending(
            (match cond_model(*cond) {
                Some((_, b)) => if a::number(b) is Some {
                    seq![b]
                } else {
                    Seq::empty()
                },
                None => Seq::empty(),
            }) + ws::bounds_of(seqs_models(nafs@), models(facts@), slot@) + seq![Term::Int(0)],
        ),
{
    let ghost start = arena.nodes@;
    let mut own: Vec<T> = Vec::new();
    match cond {
        Some((_, b)) => {
            if number_of(arena, b).is_some() {
                own.push(b.cp());
            }
        },
        None => {},
    }
    let nb = bounds_of(arena, nafs, facts, slot);
    let ghost before = arena.nodes@;
    let z = crate::m6_term::int(arena, 0);
    proof {
        prefix_all(before, arena.nodes@, nb@);
        match cond {
            Some((_, b)) => {
                prefix(start, arena.nodes@, b);
            },
            None => {},
        }
        assert(valid_all(arena.nodes@, own@));
    }
    let mut zs: Vec<T> = Vec::new();
    zs.push(z);
    let all = concat(&concat(&own, &nb), &zs);
    proof {
        assert forall|j: int| 0 <= j < all.len() implies #[trigger] valid(
            arena.nodes@,
            &all@[j],
        ) by {
            if j < own.len() {
                assert(all@[j] == own@[j]);
            } else if j < own.len() + nb.len() {
                assert(all@[j] == nb@[j - own.len()]);
            } else {
                assert(all@[j] == zs@[0]);
            }
        }
        assert_seqs_equal!(models(own@) == (match cond_model(*cond) {
            Some((_, b)) => if a::number(b) is Some { seq![b] } else { Seq::empty() },
            None => Seq::empty(),
        }));
        assert_seqs_equal!(models(zs@) == seq![Term::Int(0)]);
    }
    ascending(arena, &all)
}

// `ws::candidates`, given B.
pub fn candidates(arena: &mut ETermArena, cond: &Option<(T, T)>, bs: &Vec<T>) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        cond_valid(old(arena).nodes@, *cond),
        valid_all(old(arena).nodes@, bs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        models(out@) == seq![ws::w0(cond_model(*cond))] + ws::ascending(
            (models(bs@) + models(bs@).map_values(|b: Term| ws::plus(b, -1)) + models(
                bs@,
            ).map_values(|b: Term| ws::plus(b, 1)) + ws::midpoints(models(bs@))).filter(
                |x: Term| ws::admits(cond_model(*cond), x) && x != ws::w0(cond_model(*cond)),
            ),
        ),
{
    let ghost start = arena.nodes@;
    let ghost cm = cond_model(*cond);
    let d = w0(arena, cond);
    let ghost n1 = arena.nodes@;
    proof {
        prefix_all(start, n1, bs@);
    }
    let lo = plus_all(arena, bs, -1);
    let ghost n2 = arena.nodes@;
    proof {
        prefix_all(n1, n2, bs@);
        prefix(n1, n2, &d);
    }
    let hi = plus_all(arena, bs, 1);
    let ghost n3 = arena.nodes@;
    proof {
        prefix_all(n2, n3, bs@);
        prefix_all(n2, n3, lo@);
        prefix(n1, n3, &d);
    }
    let mids = midpoints(arena, bs);
    let ghost n4 = arena.nodes@;
    proof {
        prefix_all(n3, n4, bs@);
        prefix_all(n2, n4, lo@);
        prefix_all(n3, n4, hi@);
        prefix(n1, n4, &d);
        match cond {
            Some((op, b)) => {
                prefix(start, n4, op);
                prefix(start, n4, b);
            },
            None => {},
        }
    }
    let pool = concat(&concat(&concat(bs, &lo), &hi), &mids);
    let ghost rm = models(pool@);
    let ghost keep = |x: Term| ws::admits(cm, x) && x != ws::w0(cm);
    proof {
        assert forall|j: int| 0 <= j < pool.len() implies #[trigger] valid(
            arena.nodes@,
            &pool@[j],
        ) by {
            if j < bs.len() {
                assert(pool@[j] == bs@[j]);
            } else if j < bs.len() + lo.len() {
                assert(pool@[j] == lo@[j - bs.len()]);
            } else if j < bs.len() + lo.len() + hi.len() {
                assert(pool@[j] == hi@[j - bs.len() - lo.len()]);
            } else {
                assert(pool@[j] == mids@[j - bs.len() - lo.len() - hi.len()]);
            }
        }
    }
    let mut kept: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        reveal(Seq::filter);
        assert(rm.take(0) =~= Seq::<Term>::empty());
        assert_seqs_equal!(models(kept@) == Seq::<Term>::empty());
    }
    while i < pool.len()
        invariant
            arena_ok(arena),
            n4.is_prefix_of(arena.nodes@),
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, pool@),
            valid(arena.nodes@, &d),
            cond_valid(arena.nodes@, *cond),
            valid_all(arena.nodes@, kept@),
            rm == models(pool@),
            cm == cond_model(*cond),
            d@ == ws::w0(cm),
            keep == (|x: Term| ws::admits(cm, x) && x != ws::w0(cm)),
            i <= pool.len(),
            models(kept@) == rm.take(i as int).filter(keep),
        decreases pool.len() - i,
    {
        proof {
            assert(rm.take(i as int + 1) == rm.take(i as int).push(rm[i as int]));
            rm.take(i as int).lemma_filter_push(rm[i as int], keep);
        }
        let ghost before = arena.nodes@;
        let ok = admits(arena, cond, &pool[i]);
        proof {
            prefix_all(before, arena.nodes@, pool@);
            prefix_all(before, arena.nodes@, kept@);
            prefix(before, arena.nodes@, &d);
            match cond {
                Some((op, b)) => {
                    prefix(before, arena.nodes@, op);
                    prefix(before, arena.nodes@, b);
                },
                None => {},
            }
        }
        if ok && !crate::m6_term::equal(arena, &pool[i], &d) {
            let x = pool[i].cp();
            proof {
                extend(arena.nodes@, kept@, x);
            }
            kept.push(x);
        }
        i += 1;
    }
    proof {
        assert(rm.take(i as int) =~= rm);
    }
    let sorted = ascending(arena, &kept);
    let mut out: Vec<T> = Vec::new();
    out.push(d);
    let out = concat(&out, &sorted);
    proof {
        assert forall|j: int| 0 <= j < out.len() implies #[trigger] valid(
            arena.nodes@,
            &out@[j],
        ) by {
            if j > 0 {
                assert(out@[j] == sorted@[j - 1]);
            }
        }
    }
    out
}

} // verus!
