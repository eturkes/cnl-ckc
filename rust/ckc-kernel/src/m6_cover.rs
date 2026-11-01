#[cfg(verus_keep_ghost)]
use ckc_spec::arith::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::DocClause;
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::{lemma_fundamental_div_mod, lemma_fundamental_div_mod_converse};
use vstd::prelude::*;

verus! {

// Contract ws D7 (cover): every admitted value shares its cell with a
// candidate. Numbers are canonical, so their order is total on terms.
// d divides x and y ⇒ d divides gcd(x, y).
proof fn gcd_greatest(d: nat, x: nat, y: nat)
    requires
        d >= 1,
        x % d == 0,
        y % d == 0,
    ensures
        gcd(x, y) % d == 0,
    decreases y,
{
    reveal_with_fuel(gcd, 2);
    if y > 0 {
        lemma_fundamental_div_mod(x as int, y as int);
        lemma_fundamental_div_mod(x as int, d as int);
        lemma_fundamental_div_mod(y as int, d as int);
        let q = (x / y) as int;
        let r = (x % y) as int;
        let a = (x / d) as int;
        let b = (y / d) as int;
        let di = d as int;
        assert(r == (a - q * b) * di) by (nonlinear_arith)
            requires
                x as int == (y as int) * q + r,
                x as int == di * a,
                y as int == di * b,
        ;
        assert(a - q * b >= 0) by (nonlinear_arith)
            requires
                r >= 0,
                di >= 1,
                r == (a - q * b) * di,
        ;
        lemma_fundamental_div_mod_converse(r, di, a - q * b, 0);
        gcd_greatest(d, y, (x % y) as nat);
    }
}

// Euclid: q coprime to m and dividing m·c divides c.
proof fn euclid(m: nat, q: nat, c: nat)
    requires
        q >= 1,
        gcd(m, q) == 1,
        (m * c) % q == 0,
    ensures
        c % q == 0,
{
    if c == 0 {
        lemma_fundamental_div_mod_converse(0, q as int, 0, 0);
    } else {
        crate::arith_sound::gcd_scale(c, m, q);
        assert(c * m == m * c) by (nonlinear_arith);
        assert((c * q) % q == 0) by {
            lemma_fundamental_div_mod_converse((c * q) as int, q as int, c as int, 0);
        }
        gcd_greatest(q, c * m, c * q);
        assert(c * gcd(m, q) == c) by (nonlinear_arith)
            requires
                gcd(m, q) == 1,
        ;
    }
}

// Two numbers of one value are one term (canonical forms).
pub proof fn number_unique(x: Term, y: Term)
    requires
        match (number(x), number(y)) {
            (Option::Some((p1, q1)), Option::Some((p2, q2))) => p1 * q2 == p2 * q1,
            _ => false,
        },
    ensures
        x == y,
{
    let (p1, q1) = number(x)->0;
    let (p2, q2) = number(y)->0;
    assert(q1 >= 1 && q2 >= 1);
    assert(q1 == 1 || gcd(abs(p1), q1 as nat) == 1);
    assert(q2 == 1 || gcd(abs(p2), q2 as nat) == 1);
    // q1 divides |p1|·q2 and q2 divides |p2|·q1.
    let m1 = abs(p1);
    let m2 = abs(p2);
    assert(m1 * (q2 as nat) == m2 * (q1 as nat)) by (nonlinear_arith)
        requires
            p1 * q2 == p2 * q1,
            m1 == if p1 < 0 {
                -p1
            } else {
                p1
            },
            m2 == if p2 < 0 {
                -p2
            } else {
                p2
            },
            q1 >= 1,
            q2 >= 1,
    ;
    let a = q1 as nat;
    let b = q2 as nat;
    assert((m1 * b) % a == 0) by {
        lemma_fundamental_div_mod_converse((m1 * b) as int, a as int, m2 as int, 0);
        assert(m2 * a == a * m2) by (nonlinear_arith);
    }
    assert((m2 * a) % b == 0) by {
        lemma_fundamental_div_mod_converse((m2 * a) as int, b as int, m1 as int, 0);
        assert(m1 * b == b * m1) by (nonlinear_arith);
    }
    if a == 1 {
        lemma_fundamental_div_mod_converse(b as int, 1, b as int, 0);
    } else {
        euclid(m1, a, b);
    }
    if b == 1 {
        lemma_fundamental_div_mod_converse(a as int, 1, a as int, 0);
    } else {
        euclid(m2, b, a);
    }
    lemma_fundamental_div_mod(b as int, a as int);
    lemma_fundamental_div_mod(a as int, b as int);
    assert(a == b) by (nonlinear_arith)
        requires
            b as int == (a as int) * ((b / a) as int),
            a as int == (b as int) * ((a / b) as int),
            a >= 1,
            b >= 1,
    ;
    assert(p1 == p2) by (nonlinear_arith)
        requires
            p1 * q2 == p2 * q1,
            q1 == q2,
            q1 >= 1,
    ;
    match (x, y) {
        (Term::Comp(_, xa), Term::Comp(_, ya)) => {
            assert(xa =~= seq![Term::Int(p1), Term::Int(q1)]);
            assert(ya =~= seq![Term::Int(p2), Term::Int(q2)]);
        },
        _ => {},
    }
}

// --- the rational order on number terms ---
pub proof fn lt_total(x: Term, y: Term)
    requires
        number(x) is Some,
        number(y) is Some,
    ensures
        ws::lt(x, y) || x == y || ws::lt(y, x),
        !(ws::lt(x, y) && ws::lt(y, x)),
        !ws::lt(x, x),
        x == y ==> !ws::lt(x, y),
{
    let (p1, q1) = number(x)->0;
    let (p2, q2) = number(y)->0;
    if p1 * q2 == p2 * q1 {
        number_unique(x, y);
    }
}

pub proof fn lt_trans(x: Term, y: Term, z: Term)
    requires
        ws::lt(x, y),
        ws::lt(y, z),
    ensures
        ws::lt(x, z),
{
    let (p1, q1) = number(x)->0;
    let (p2, q2) = number(y)->0;
    let (p3, q3) = number(z)->0;
    assert(p1 * q3 < p3 * q1) by (nonlinear_arith)
        requires
            p1 * q2 < p2 * q1,
            p2 * q3 < p3 * q2,
            q1 >= 1,
            q2 >= 1,
            q3 >= 1,
    ;
}

// --- values of the candidate points ---
// b − 1 < b < b + 1.
pub proof fn plus_order(b: Term)
    requires
        number(b) is Some,
    ensures
        number(ws::plus(b, 1)) is Some,
        number(ws::plus(b, -1)) is Some,
        ws::lt(b, ws::plus(b, 1)),
        ws::lt(ws::plus(b, -1), b),
{
    let (p, q) = number(b)->0;
    crate::arith_sound::canon_number(p + q, q);
    crate::arith_sound::canon_number(p - q, q);
    assert(p + 1 * q == p + q && p + (-1) * q == p - q);
    let (n1, d1) = number(canon(p + q, q))->0;
    let (n2, d2) = number(canon(p - q, q))->0;
    assert(p * d1 < n1 * q) by (nonlinear_arith)
        requires
            n1 * q == (p + q) * d1,
            d1 >= 1,
            q >= 1,
    ;
    assert(n2 * q < p * d2) by (nonlinear_arith)
        requires
            n2 * q == (p - q) * d2,
            d2 >= 1,
            q >= 1,
    ;
}

// a < b ⇒ a < mid(a, b) < b.
pub proof fn mid_order(a: Term, b: Term)
    requires
        ws::lt(a, b),
    ensures
        number(ws::mid(a, b)) is Some,
        ws::lt(a, ws::mid(a, b)),
        ws::lt(ws::mid(a, b), b),
{
    let (p1, q1) = number(a)->0;
    let (p2, q2) = number(b)->0;
    let s = p1 * q2 + p2 * q1;
    let t = 2 * q1 * q2;
    assert(t >= 1) by (nonlinear_arith)
        requires
            q1 >= 1,
            q2 >= 1,
            t == 2 * q1 * q2,
    ;
    crate::arith_sound::canon_number(s, t);
    let (n, d) = number(canon(s, t))->0;
    assert(p1 * d < n * q1) by (nonlinear_arith)
        requires
            n * t == s * d,
            s == p1 * q2 + p2 * q1,
            t == 2 * q1 * q2,
            p1 * q2 < p2 * q1,
            d >= 1,
            q1 >= 1,
            q2 >= 1,
    ;
    assert(n * q2 < p2 * d) by (nonlinear_arith)
        requires
            n * t == s * d,
            s == p1 * q2 + p2 * q1,
            t == 2 * q1 * q2,
            p1 * q2 < p2 * q1,
            d >= 1,
            q1 >= 1,
            q2 >= 1,
    ;
}

// --- admission depends on the cell ---
proof fn op_holds_sign(op: Seq<u8>, x1: int, y1: int, x2: int, y2: int)
    requires
        (x1 < y1) == (x2 < y2),
        (x1 == y1) == (x2 == y2),
    ensures
        ckc_spec::compare::op_holds(op, x1, y1) == ckc_spec::compare::op_holds(op, x2, y2),
{
}

pub proof fn within_sign(
    rows: Seq<(Seq<u8>, Seq<u8>, Seq<u8>)>,
    s: Seq<u8>,
    b: Seq<u8>,
    x1: int,
    y1: int,
    x2: int,
    y2: int,
)
    requires
        (x1 < y1) == (x2 < y2),
        (x1 == y1) == (x2 == y2),
    ensures
        ckc_spec::compare::within_from(rows, s, x1, b, y1) == ckc_spec::compare::within_from(
            rows,
            s,
            x2,
            b,
            y2,
        ),
    decreases rows.len(),
{
    if rows.len() > 0 {
        op_holds_sign(rows[0].2, x1, y1, x2, y2);
        within_sign(rows.drop_first(), s, b, x1, y1, x2, y2);
    }
}

// Two numbers on the same side of, or equal to, a condition's bound b are
// admitted alike.
pub proof fn admits_sides(cond: Option<(Term, Term)>, x: Term, y: Term)
    requires
        number(x) is Some,
        number(y) is Some,
        match cond {
            Option::Some((_, b)) => number(b) is Some ==> ws::lt(x, b) == ws::lt(y, b) && (x == b)
                == (y == b),
            Option::None => true,
        },
    ensures
        ws::admits(cond, x) == ws::admits(cond, y),
{
    if let Option::Some((op, b)) = cond {
        if let (Term::Atom(ob), Option::Some((pb, qb))) = (op, number(b)) {
            let (px, qx) = number(x)->0;
            let (py, qy) = number(y)->0;
            if px * qb == pb * qx {
                number_unique(x, b);
            }
            if py * qb == pb * qy {
                number_unique(y, b);
            }
            within_sign(
                ckc_spec::compare::within_rows(),
                ckc_spec::v1text::ascii("exactly"@),
                ob,
                px * qb,
                pb * qx,
                py * qb,
                pb * qy,
            );
            assert(ws::exactly() == Term::Atom(ckc_spec::v1text::ascii("exactly"@)));
        }
    }
}

// --- ascending lists ---
pub open spec fn sorted(s: Seq<Term>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> ws::lt(#[trigger] s[i], #[trigger] s[j])
}

pub open spec fn numbers(s: Seq<Term>) -> bool {
    forall|i: int| 0 <= i < s.len() ==> number(#[trigger] s[i]) is Some
}

proof fn contains_cons(h: Term, t: Seq<Term>)
    ensures
        forall|y: Term| #[trigger] (seq![h] + t).contains(y) <==> (y == h || t.contains(y)),
{
    let r = seq![h] + t;
    assert forall|y: Term| #[trigger] r.contains(y) <==> (y == h || t.contains(y)) by {
        if r.contains(y) {
            let k = choose|k: int| 0 <= k < r.len() && r[k] == y;
            if k > 0 {
                assert(t[k - 1] == y);
            }
        }
        if y == h {
            assert(r[0] == y);
        }
        if t.contains(y) {
            let k = choose|k: int| 0 <= k < t.len() && t[k] == y;
            assert(r[k + 1] == y);
        }
    }
}

pub proof fn insert_contains(xs: Seq<Term>, x: Term)
    ensures
        forall|y: Term| #[trigger] ws::insert(xs, x).contains(y) <==> (xs.contains(y) || y == x),
    decreases xs.len(),
{
    if xs.len() == 0 {
        contains_cons(x, Seq::<Term>::empty());
        assert(seq![x] + Seq::<Term>::empty() =~= seq![x]);
    } else {
        let t = xs.drop_first();
        assert(xs =~= seq![xs[0]] + t);
        contains_cons(xs[0], t);
        if xs[0] == x {
        } else if ws::lt(x, xs[0]) {
            contains_cons(x, xs);
        } else {
            insert_contains(t, x);
            contains_cons(xs[0], ws::insert(t, x));
        }
    }
}

proof fn sorted_tail(xs: Seq<Term>)
    requires
        xs.len() > 0,
        sorted(xs),
        numbers(xs),
    ensures
        sorted(xs.drop_first()),
        numbers(xs.drop_first()),
        forall|y: Term| #[trigger] xs.drop_first().contains(y) ==> ws::lt(xs[0], y),
{
    let t = xs.drop_first();
    assert forall|i: int, j: int| 0 <= i < j < t.len() implies ws::lt(
        #[trigger] t[i],
        #[trigger] t[j],
    ) by {
        assert(t[i] == xs[i + 1] && t[j] == xs[j + 1]);
    }
    assert forall|i: int| 0 <= i < t.len() implies number(#[trigger] t[i]) is Some by {
        assert(t[i] == xs[i + 1]);
    }
    assert forall|y: Term| #[trigger] t.contains(y) implies ws::lt(xs[0], y) by {
        let k = choose|k: int| 0 <= k < t.len() && t[k] == y;
        assert(xs[k + 1] == y);
    }
}

// h below every element of the sorted numbers t ⇒ h :: t sorted.
proof fn sorted_cons(h: Term, t: Seq<Term>)
    requires
        number(h) is Some,
        sorted(t),
        numbers(t),
        forall|y: Term| #[trigger] t.contains(y) ==> ws::lt(h, y),
    ensures
        sorted(seq![h] + t),
        numbers(seq![h] + t),
{
    let r = seq![h] + t;
    assert forall|i: int, j: int| 0 <= i < j < r.len() implies ws::lt(
        #[trigger] r[i],
        #[trigger] r[j],
    ) by {
        assert(r[j] == t[j - 1]);
        if i == 0 {
            assert(t.contains(t[j - 1]));
        } else {
            assert(r[i] == t[i - 1]);
        }
    }
    assert forall|i: int| 0 <= i < r.len() implies number(#[trigger] r[i]) is Some by {
        if i > 0 {
            assert(r[i] == t[i - 1]);
        }
    }
}

pub proof fn insert_sorted(xs: Seq<Term>, x: Term)
    requires
        sorted(xs),
        numbers(xs),
        number(x) is Some,
    ensures
        sorted(ws::insert(xs, x)),
        numbers(ws::insert(xs, x)),
    decreases xs.len(),
{
    if xs.len() == 0 {
        sorted_cons(x, Seq::<Term>::empty());
        assert(seq![x] + Seq::<Term>::empty() =~= seq![x]);
    } else if xs[0] == x {
    } else {
        sorted_tail(xs);
        let t = xs.drop_first();
        assert(xs =~= seq![xs[0]] + t);
        if ws::lt(x, xs[0]) {
            assert forall|y: Term| #[trigger] xs.contains(y) implies ws::lt(x, y) by {
                contains_cons(xs[0], t);
                if y != xs[0] {
                    lt_trans(x, xs[0], y);
                }
            }
            sorted_cons(x, xs);
        } else {
            lt_total(x, xs[0]);
            insert_sorted(t, x);
            insert_contains(t, x);
            sorted_cons(xs[0], ws::insert(t, x));
        }
    }
}

pub proof fn ascending_props(xs: Seq<Term>)
    requires
        numbers(xs),
    ensures
        sorted(ws::ascending(xs)),
        numbers(ws::ascending(xs)),
        forall|y: Term| #[trigger] ws::ascending(xs).contains(y) <==> xs.contains(y),
    decreases xs.len(),
{
    if xs.len() > 0 {
        let a = xs.drop_last();
        assert forall|i: int| 0 <= i < a.len() implies number(#[trigger] a[i]) is Some by {
            assert(a[i] == xs[i]);
        }
        ascending_props(a);
        insert_sorted(ws::ascending(a), xs.last());
        insert_contains(ws::ascending(a), xs.last());
        assert forall|y: Term| #[trigger] ws::ascending(xs).contains(y) <==> xs.contains(y) by {
            if a.contains(y) {
                let k = choose|k: int| 0 <= k < a.len() && a[k] == y;
                assert(xs[k] == y);
            }
            if xs.contains(y) {
                let k = choose|k: int| 0 <= k < xs.len() && xs[k] == y;
                if k < xs.len() - 1 {
                    assert(a[k] == y);
                }
            }
            assert(ws::ascending(a).contains(y) == a.contains(y));
        }
    } else {
        assert(ws::ascending(xs) =~= Seq::<Term>::empty());
    }
}

// --- B: ascending numbers holding 0 ---
proof fn numbers_concat(a: Seq<Term>, b: Seq<Term>)
    requires
        numbers(a),
        numbers(b),
    ensures
        numbers(a + b),
{
    assert forall|i: int| 0 <= i < (a + b).len() implies number(#[trigger] (a + b)[i]) is Some by {
        if i >= a.len() {
            assert((a + b)[i] == b[i - a.len()]);
        }
    }
}

proof fn item_bounds_numbers(gs: Seq<Term>, cs: Seq<Term>, facts: Seq<Term>, slot: Term)
    ensures
        numbers(ws::item_bounds(gs, cs, facts, slot)),
    decreases cs.len(),
{
    if cs.len() > 0 {
        item_bounds_numbers(gs, cs.drop_first(), facts, slot);
        let c = cs[0];
        let head = if ckc_spec::emit::is_comp(c, "guideline_satisfies"@, 4) && number(
            ckc_spec::replay::arg(c, 3),
        ) is Some && ws::relevant(gs, facts, c, slot) {
            seq![ckc_spec::replay::arg(c, 3)]
        } else {
            Seq::<Term>::empty()
        };
        assert(numbers(head));
        numbers_concat(head, ws::item_bounds(gs, cs.drop_first(), facts, slot));
    }
}

proof fn bounds_of_numbers(nafs: Seq<Seq<Term>>, facts: Seq<Term>, slot: Term)
    ensures
        numbers(ws::bounds_of(nafs, facts, slot)),
    decreases nafs.len(),
{
    if nafs.len() > 0 {
        item_bounds_numbers(nafs[0], nafs[0], facts, slot);
        bounds_of_numbers(nafs.drop_first(), facts, slot);
        numbers_concat(
            ws::item_bounds(nafs[0], nafs[0], facts, slot),
            ws::bounds_of(nafs.drop_first(), facts, slot),
        );
    }
}

// The boundaries are ascending numbers holding 0 and the operand's own bound.
pub proof fn boundaries_props(g: ckc_spec::emit::Group, sel: ws::Sel, i: int)
    requires
        0 <= i < sel.ops.len(),
    ensures
        sorted(ws::boundaries(g, sel, i)),
        numbers(ws::boundaries(g, sel, i)),
        ws::boundaries(g, sel, i).contains(Term::Int(0)),
        match sel.ops[i].cond {
            Option::Some((_, b)) => number(b) is Some ==> ws::boundaries(g, sel, i).contains(b),
            Option::None => true,
        },
{
    let own = match sel.ops[i].cond {
        Option::Some((_, b)) => if number(b) is Some {
            seq![b]
        } else {
            Seq::<Term>::empty()
        },
        Option::None => Seq::<Term>::empty(),
    };
    let nb = ws::naf_bounds(g, sel, i);
    bounds_of_numbers(ws::naf_items(g, sel), ws::skeleton(g, sel), sel.ops[i].n);
    assert(numbers(own));
    assert(numbers(seq![Term::Int(0)]));
    numbers_concat(own, nb);
    numbers_concat(own + nb, seq![Term::Int(0)]);
    let src = own + nb + seq![Term::Int(0)];
    ascending_props(src);
    assert(src[src.len() - 1] == Term::Int(0));
    assert(src.contains(Term::Int(0)));
    if let Option::Some((_, b)) = sel.ops[i].cond {
        if number(b) is Some {
            assert(src[0] == b);
            assert(src.contains(b));
        }
    }
}

// --- a candidate in every admitted value's cell ---
// The representative of x's cell among the sorted boundaries bs.
pub open spec fn rep(bs: Seq<Term>, x: Term) -> Term {
    if bs.contains(x) {
        x
    } else if ws::lt(x, bs[0]) {
        ws::plus(bs[0], -1)
    } else if ws::lt(bs.last(), x) {
        ws::plus(bs.last(), 1)
    } else {
        let k = choose|k: int| 0 <= k < bs.len() - 1 && ws::lt(bs[k], x) && ws::lt(x, bs[k + 1]);
        ws::mid(bs[k], bs[k + 1])
    }
}

pub open spec fn pool(bs: Seq<Term>) -> Seq<Term> {
    bs + bs.map_values(|b: Term| ws::plus(b, -1)) + bs.map_values(|b: Term| ws::plus(b, 1))
        + ws::midpoints(bs)
}

proof fn find_gap(bs: Seq<Term>, x: Term, j: int)
    requires
        sorted(bs),
        numbers(bs),
        number(x) is Some,
        0 <= j < bs.len() - 1,
        ws::lt(bs[j], x),
        ws::lt(x, bs.last()),
        !bs.contains(x),
    ensures
        exists|k: int| 0 <= k < bs.len() - 1 && ws::lt(bs[k], x) && ws::lt(x, bs[k + 1]),
    decreases bs.len() - j,
{
    lt_total(x, bs[j + 1]);
    if !ws::lt(x, bs[j + 1]) {
        if bs[j + 1] == x {
            assert(bs.contains(x));
        }
        if j + 1 == bs.len() - 1 {
            lt_total(x, bs.last());
        } else {
            find_gap(bs, x, j + 1);
        }
    }
}

// below/above every boundary from index k on, alike for x and c
proof fn sorted_le(bs: Seq<Term>, a: int, b: int)
    requires
        sorted(bs),
        0 <= a <= b < bs.len(),
    ensures
        a == b || ws::lt(bs[a], bs[b]),
{
}

pub proof fn rep_cell(bs: Seq<Term>, x: Term)
    requires
        sorted(bs),
        numbers(bs),
        bs.len() > 0,
        number(x) is Some,
    ensures
        ws::same_cell(bs, x, rep(bs, x)),
        pool(bs).contains(rep(bs, x)),
{
    let c = rep(bs, x);
    let last = bs.len() - 1;
    let p = pool(bs);
    let m1 = bs.map_values(|b: Term| ws::plus(b, -1));
    let p1 = bs.map_values(|b: Term| ws::plus(b, 1));
    let mids = ws::midpoints(bs);
    assert(p == bs + m1 + p1 + mids);
    if bs.contains(x) {
        let k = choose|k: int| 0 <= k < bs.len() && bs[k] == x;
        assert(p[k] == x);
    } else if ws::lt(x, bs[0]) {
        plus_order(bs[0]);
        assert(p[bs.len() as int] == c);
        assert forall|k: int| 0 <= k < bs.len() implies ws::lt(x, #[trigger] bs[k]) == ws::lt(
            c,
            bs[k],
        ) && (x == bs[k]) == (c == bs[k]) by {
            sorted_le(bs, 0, k);
            if k > 0 {
                lt_trans(x, bs[0], bs[k]);
                lt_trans(c, bs[0], bs[k]);
            }
            lt_total(x, bs[k]);
            lt_total(c, bs[k]);
        }
    } else if ws::lt(bs.last(), x) {
        plus_order(bs.last());
        assert(p[(2 * bs.len() + last) as int] == c);
        assert forall|k: int| 0 <= k < bs.len() implies ws::lt(x, #[trigger] bs[k]) == ws::lt(
            c,
            bs[k],
        ) && (x == bs[k]) == (c == bs[k]) by {
            sorted_le(bs, k, last);
            if k < last {
                lt_trans(bs[k], bs.last(), x);
                lt_trans(bs[k], bs.last(), c);
            }
            lt_total(x, bs[k]);
            lt_total(c, bs[k]);
        }
    } else {
        lt_total(x, bs[0]);
        lt_total(x, bs.last());
        assert(bs[0] != x);
        assert(bs.last() != x);
        assert(last > 0) by {
            if last == 0 {
                lt_total(bs[0], x);
            }
        }
        find_gap(bs, x, 0);
        let k0 = choose|k: int| 0 <= k < bs.len() - 1 && ws::lt(bs[k], x) && ws::lt(x, bs[k + 1]);
        assert(c == ws::mid(bs[k0], bs[k0 + 1]));
        lt_total(bs[k0], bs[k0 + 1]);
        sorted_le(bs, k0, k0 + 1);
        mid_order(bs[k0], bs[k0 + 1]);
        assert(mids[k0] == c);
        assert(p[(3 * bs.len() + k0) as int] == c);
        assert forall|k: int| 0 <= k < bs.len() implies ws::lt(x, #[trigger] bs[k]) == ws::lt(
            c,
            bs[k],
        ) && (x == bs[k]) == (c == bs[k]) by {
            if k <= k0 {
                sorted_le(bs, k, k0);
                if k < k0 {
                    lt_trans(bs[k], bs[k0], x);
                    lt_trans(bs[k], bs[k0], c);
                }
            } else {
                sorted_le(bs, k0 + 1, k);
                if k > k0 + 1 {
                    lt_trans(x, bs[k0 + 1], bs[k]);
                    lt_trans(c, bs[k0 + 1], bs[k]);
                }
            }
            lt_total(x, bs[k]);
            lt_total(c, bs[k]);
        }
    }
}

proof fn pool_numbers(bs: Seq<Term>)
    requires
        sorted(bs),
        numbers(bs),
    ensures
        numbers(pool(bs)),
{
    let m1 = bs.map_values(|b: Term| ws::plus(b, -1));
    let p1 = bs.map_values(|b: Term| ws::plus(b, 1));
    let mids = ws::midpoints(bs);
    assert forall|i: int| 0 <= i < m1.len() implies number(#[trigger] m1[i]) is Some by {
        plus_order(bs[i]);
    }
    assert forall|i: int| 0 <= i < p1.len() implies number(#[trigger] p1[i]) is Some by {
        plus_order(bs[i]);
    }
    assert forall|i: int| 0 <= i < mids.len() implies number(#[trigger] mids[i]) is Some by {
        sorted_le(bs, i, i + 1);
        mid_order(bs[i], bs[i + 1]);
    }
    numbers_concat(bs, m1);
    numbers_concat(bs + m1, p1);
    numbers_concat(bs + m1 + p1, mids);
}

// Every admitted value of operand i shares its cell with one of its candidates.
pub proof fn cover(g: ckc_spec::emit::Group, sel: ws::Sel, i: int, x: Term)
    requires
        0 <= i < sel.ops.len(),
        ws::admits(sel.ops[i].cond, x),
    ensures
        ws::candidates(g, sel, i).contains(rep(ws::boundaries(g, sel, i), x)),
        ws::same_cell(ws::boundaries(g, sel, i), x, rep(ws::boundaries(g, sel, i), x)),
{
    let bs = ws::boundaries(g, sel, i);
    boundaries_props(g, sel, i);
    rep_cell(bs, x);
    let c = rep(bs, x);
    let cond = sel.ops[i].cond;
    let d = ws::w0(cond);
    admits_sides(cond, x, c);
    assert(ws::admits(cond, c)) by {
        if let Option::Some((_, b)) = cond {
            if number(b) is Some {
                let k = choose|k: int| 0 <= k < bs.len() && bs[k] == b;
                assert(ws::lt(x, bs[k]) == ws::lt(c, bs[k]));
            }
        }
    }
    let p = pool(bs);
    assert(p == bs + bs.map_values(|b: Term| ws::plus(b, -1)) + bs.map_values(
        |b: Term| ws::plus(b, 1),
    ) + ws::midpoints(bs));
    let pred = |x: Term| ws::admits(cond, x) && x != d;
    let f = p.filter(pred);
    let cs = ws::candidates(g, sel, i);
    assert(cs == seq![d] + ws::ascending(f));
    if c == d {
        assert(cs[0] == c);
    } else {
        let k = choose|k: int| 0 <= k < p.len() && p[k] == c;
        p.lemma_filter_contains(pred, k);
        pool_numbers(bs);
        assert forall|j: int| 0 <= j < f.len() implies number(#[trigger] f[j]) is Some by {
            assert(f.contains(f[j]));
            p.lemma_filter_contains_rev(pred, f[j]);
        }
        ascending_props(f);
        assert(ws::ascending(f).contains(c));
        let m = choose|m: int| 0 <= m < ws::ascending(f).len() && ws::ascending(f)[m] == c;
        assert(cs[m + 1] == c);
    }
}

// --- the odometer reaches every candidate tuple ---
// The index whose digits are ms (the last operand varies fastest).
pub open spec fn encode(cs: Seq<Seq<Term>>, ms: Seq<nat>) -> nat
    decreases cs.len(),
{
    if cs.len() == 0 || ms.len() == 0 {
        0
    } else {
        encode(cs.drop_last(), ms.drop_last()) * cs.last().len() + ms.last()
    }
}

pub proof fn encode_digits(cs: Seq<Seq<Term>>, ms: Seq<nat>)
    requires
        ms.len() == cs.len(),
        forall|k: int| 0 <= k < cs.len() ==> #[trigger] ms[k] < cs[k].len(),
    ensures
        encode(cs, ms) < ws::tuple_count(cs),
        ws::digits(cs, encode(cs, ms)) == ms,
    decreases cs.len(),
{
    if cs.len() == 0 {
        assert(ms =~= Seq::<nat>::empty());
    } else {
        let pc = cs.drop_last();
        let pm = ms.drop_last();
        assert forall|k: int| 0 <= k < pc.len() implies #[trigger] pm[k] < pc[k].len() by {
            assert(pm[k] == ms[k] && pc[k] == cs[k]);
        }
        encode_digits(pc, pm);
        let e = encode(pc, pm);
        let l = cs.last().len();
        let m = ms.last();
        assert(m < l) by {
            assert(ms[ms.len() - 1] < cs[cs.len() - 1].len());
        }
        let i = e * l + m;
        assert(encode(cs, ms) == i);
        lemma_fundamental_div_mod_converse(i as int, l as int, e as int, m as int);
        let tp = ws::tuple_count(pc);
        assert(i < tp * l) by (nonlinear_arith)
            requires
                e < tp,
                m < l,
                i == e * l + m,
        ;
        assert(ws::tuple_count(cs) == tp * l);
        assert(ws::digits(cs, i) == ws::digits(pc, i / l) + seq![i % l]);
        assert(ws::digits(cs, i) =~= ms);
    }
}

proof fn first_win_scan(p: Seq<DocClause>, g: ckc_spec::emit::Group, sel: ws::Sel, i: nat, idx: nat)
    requires
        i <= idx < ws::tuple_count(ws::cand_lists(g, sel)),
        ws::win(p, g, sel, ws::nth_tuple(ws::cand_lists(g, sel), idx)),
    ensures
        ws::first_win(p, g, sel, i) is Some,
    decreases idx - i,
{
    if !ws::win(p, g, sel, ws::nth_tuple(ws::cand_lists(g, sel), i)) {
        assert(i != idx);
        first_win_scan(p, g, sel, i + 1, idx);
    }
}

// D7 from cell invariance: the candidate tuple in t's cells wins with t.
pub proof fn complete(p: Seq<DocClause>, g: ckc_spec::emit::Group, sel: ws::Sel, t: Seq<Term>)
    requires
        ws::in_fragment(p, g, sel),
        t.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> ws::admits(sel.ops[i].cond, #[trigger] t[i]),
        ws::win(p, g, sel, t),
    ensures
        ws::first_win(p, g, sel, 0) is Some,
{
    let n = sel.ops.len();
    let cl = ws::cand_lists(g, sel);
    let c = Seq::new(n, |k: int| rep(ws::boundaries(g, sel, k), t[k]));
    assert forall|k: int| 0 <= k < n implies #[trigger] cl[k].contains(c[k]) && ws::same_cell(
        ws::boundaries(g, sel, k),
        t[k],
        c[k],
    ) by {
        cover(g, sel, k, t[k]);
    }
    let ms = Seq::new(n, |k: int| (choose|m: int| 0 <= m < cl[k].len() && cl[k][m] == c[k]) as nat);
    assert forall|k: int| 0 <= k < cl.len() implies #[trigger] ms[k] < cl[k].len()
        && cl[k][ms[k] as int] == c[k] by {
        assert(cl[k].contains(c[k]));
    }
    encode_digits(cl, ms);
    let idx = encode(cl, ms);
    assert(ws::nth_tuple(cl, idx) =~= c);
    assert forall|i: int| 0 <= i < n implies ws::same_cell(
        ws::boundaries(g, sel, i),
        #[trigger] t[i],
        c[i],
    ) by {
        assert(cl[i].contains(c[i]));
    }
    crate::m6_witness::cell_invariance(p, g, sel, t, c);
    first_win_scan(p, g, sel, 0, idx);
}

} // verus!
