#[cfg(verus_keep_ghost)]
use ckc_spec::arith::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::{lemma_fundamental_div_mod, lemma_fundamental_div_mod_converse};
use vstd::prelude::*;

verus! {

// Soundness of the product table (contract da1 D3, P10): a derived bound never
// excludes a true product. Rationals = (numerator, denominator ≥ 1); a value
// set's membership compares by cross-multiplication.
pub open spec fn den_ok(v: VSet) -> bool {
    match v {
        VSet::Point(_, d) => d >= 1,
        VSet::Lower(_, d, _) => d >= 1,
        VSet::Upper(_, d, _) => d >= 1,
    }
}

pub open spec fn member(un: int, ud: int, v: VSet) -> bool {
    match v {
        VSet::Point(n, d) => un * d == n * ud,
        VSet::Lower(n, d, c) => if c {
            n * ud <= un * d
        } else {
            n * ud < un * d
        },
        VSet::Upper(n, d, c) => if c {
            un * d <= n * ud
        } else {
            un * d < n * ud
        },
    }
}

proof fn point_times(x: int, xd: int, b: VSet, un: int, ud: int, vn: int, vd: int)
    requires
        xd >= 1,
        ud >= 1,
        vd >= 1,
        den_ok(b),
        un * xd == x * ud,
        member(vn, vd, b),
        x != 0,
    ensures
        member(un * vn, ud * vd, stretch(x, xd, b)),
{
    match b {
        VSet::Point(y, yd) => {
            assert((un * vn) * (xd * yd) == (x * y) * (ud * vd)) by (nonlinear_arith)
                requires
                    un * xd == x * ud,
                    vn * yd == y * vd,
            ;
        },
        VSet::Lower(y, yd, c) => {
            if x > 0 {
                assert(c ==> (x * y) * (ud * vd) <= (un * vn) * (xd * yd)) by (nonlinear_arith)
                    requires
                        un * xd == x * ud,
                        c ==> y * vd <= vn * yd,
                        x > 0,
                        xd >= 1,
                        ud >= 1,
                ;
                assert(!c ==> (x * y) * (ud * vd) < (un * vn) * (xd * yd)) by (nonlinear_arith)
                    requires
                        un * xd == x * ud,
                        !c ==> y * vd < vn * yd,
                        x > 0,
                        xd >= 1,
                        ud >= 1,
                ;
            } else {
                assert(c ==> (un * vn) * (xd * yd) <= (x * y) * (ud * vd)) by (nonlinear_arith)
                    requires
                        un * xd == x * ud,
                        c ==> y * vd <= vn * yd,
                        x < 0,
                        xd >= 1,
                        ud >= 1,
                ;
                assert(!c ==> (un * vn) * (xd * yd) < (x * y) * (ud * vd)) by (nonlinear_arith)
                    requires
                        un * xd == x * ud,
                        !c ==> y * vd < vn * yd,
                        x < 0,
                        xd >= 1,
                        ud >= 1,
                ;
            }
        },
        VSet::Upper(y, yd, c) => {
            if x > 0 {
                assert(c ==> (un * vn) * (xd * yd) <= (x * y) * (ud * vd)) by (nonlinear_arith)
                    requires
                        un * xd == x * ud,
                        c ==> vn * yd <= y * vd,
                        x > 0,
                        xd >= 1,
                        ud >= 1,
                ;
                assert(!c ==> (un * vn) * (xd * yd) < (x * y) * (ud * vd)) by (nonlinear_arith)
                    requires
                        un * xd == x * ud,
                        !c ==> vn * yd < y * vd,
                        x > 0,
                        xd >= 1,
                        ud >= 1,
                ;
            } else {
                assert(c ==> (x * y) * (ud * vd) <= (un * vn) * (xd * yd)) by (nonlinear_arith)
                    requires
                        un * xd == x * ud,
                        c ==> vn * yd <= y * vd,
                        x < 0,
                        xd >= 1,
                        ud >= 1,
                ;
                assert(!c ==> (x * y) * (ud * vd) < (un * vn) * (xd * yd)) by (nonlinear_arith)
                    requires
                        un * xd == x * ud,
                        !c ==> vn * yd < y * vd,
                        x < 0,
                        xd >= 1,
                        ud >= 1,
                ;
            }
        },
    }
}

// Two nonnegative lower rays: the product of the lower bounds bounds the product.
proof fn lower_lower(
    x: int,
    xd: int,
    c1: bool,
    y: int,
    yd: int,
    c2: bool,
    un: int,
    ud: int,
    vn: int,
    vd: int,
)
    requires
        xd >= 1,
        yd >= 1,
        ud >= 1,
        vd >= 1,
        x >= 0,
        y >= 0,
        if c1 {
            x * ud <= un * xd
        } else {
            x * ud < un * xd
        },
        if c2 {
            y * vd <= vn * yd
        } else {
            y * vd < vn * yd
        },
    ensures
        if att(x, c1, y, c2) {
            (x * y) * (ud * vd) <= (un * vn) * (xd * yd)
        } else {
            (x * y) * (ud * vd) < (un * vn) * (xd * yd)
        },
{
    let p = x * ud;
    let q = y * vd;
    let pp = un * xd;
    let qq = vn * yd;
    assert(p >= 0 && q >= 0) by (nonlinear_arith)
        requires
            x >= 0,
            y >= 0,
            ud >= 1,
            vd >= 1,
            p == x * ud,
            q == y * vd,
    ;
    assert((x * y) * (ud * vd) == p * q) by (nonlinear_arith)
        requires
            p == x * ud,
            q == y * vd,
    ;
    assert((un * vn) * (xd * yd) == pp * qq) by (nonlinear_arith)
        requires
            pp == un * xd,
            qq == vn * yd,
    ;
    assert((x == 0) == (p == 0)) by (nonlinear_arith)
        requires
            p == x * ud,
            ud >= 1,
    ;
    assert((y == 0) == (q == 0)) by (nonlinear_arith)
        requires
            q == y * vd,
            vd >= 1,
    ;
    assert(p <= pp && q <= qq);
    assert(p * q <= pp * qq) by (nonlinear_arith)
        requires
            0 <= p <= pp,
            0 <= q <= qq,
    ;
    if !att(x, c1, y, c2) {
        // no zero bound is attained and some bound is strict
        assert(p * q < pp * qq) by (nonlinear_arith)
            requires
                0 <= p,
                0 <= q,
                p <= pp,
                q <= qq,
                (!c1 ==> p < pp),
                (!c2 ==> q < qq),
                !((x == 0 && c1) || (y == 0 && c2) || (c1 && c2)),
                x == 0 <==> p == 0,
                y == 0 <==> q == 0,
        ;
    }
}

// The product table's soundness (contract da1 P10).
pub proof fn vprod_sound(a: VSet, b: VSet, un: int, ud: int, vn: int, vd: int)
    requires
        den_ok(a),
        den_ok(b),
        ud >= 1,
        vd >= 1,
        member(un, ud, a),
        member(vn, vd, b),
        vprod(a, b) is Some,
    ensures
        member(un * vn, ud * vd, vprod(a, b)->Some_0),
{
    if zero_point(a) {
        assert(un == 0) by (nonlinear_arith)
            requires
                (match a {
                    VSet::Point(n, d) => n == 0 && un * d == n * ud && d >= 1,
                    _ => false,
                }),
        ;
        assert((un * vn) * 1 == 0 * (ud * vd)) by (nonlinear_arith)
            requires
                un == 0,
        ;
    } else if zero_point(b) {
        assert(vn == 0) by (nonlinear_arith)
            requires
                (match b {
                    VSet::Point(n, d) => n == 0 && vn * d == n * vd && d >= 1,
                    _ => false,
                }),
        ;
        assert((un * vn) * 1 == 0 * (ud * vd)) by (nonlinear_arith)
            requires
                vn == 0,
        ;
    } else {
        match (a, b) {
            (VSet::Point(x, xd), _) => point_times(x, xd, b, un, ud, vn, vd),
            (_, VSet::Point(y, yd)) => {
                point_times(y, yd, a, vn, vd, un, ud);
                assert(vn * un == un * vn && vd * ud == ud * vd) by (nonlinear_arith);
            },
            (VSet::Lower(x, xd, c1), VSet::Lower(y, yd, c2)) => {
                lower_lower(x, xd, c1, y, yd, c2, un, ud, vn, vd);
            },
            (VSet::Upper(x, xd, c1), VSet::Upper(y, yd, c2)) => {
                assert(if c1 {
                    (-x) * ud <= (-un) * xd
                } else {
                    (-x) * ud < (-un) * xd
                }) by (nonlinear_arith)
                    requires
                        if c1 {
                            un * xd <= x * ud
                        } else {
                            un * xd < x * ud
                        },
                ;
                assert(if c2 {
                    (-y) * vd <= (-vn) * yd
                } else {
                    (-y) * vd < (-vn) * yd
                }) by (nonlinear_arith)
                    requires
                        if c2 {
                            vn * yd <= y * vd
                        } else {
                            vn * yd < y * vd
                        },
                ;
                lower_lower(-x, xd, c1, -y, yd, c2, -un, ud, -vn, vd);
                assert(att(-x, c1, -y, c2) == att(x, c1, y, c2));
                assert((-x) * (-y) == x * y && (-un) * (-vn) == un * vn) by (nonlinear_arith);
            },
            (VSet::Lower(x, xd, c1), VSet::Upper(y, yd, c2)) => {
                assert(if c2 {
                    (-y) * vd <= (-vn) * yd
                } else {
                    (-y) * vd < (-vn) * yd
                }) by (nonlinear_arith)
                    requires
                        if c2 {
                            vn * yd <= y * vd
                        } else {
                            vn * yd < y * vd
                        },
                ;
                lower_lower(x, xd, c1, -y, yd, c2, un, ud, -vn, vd);
                assert(att(x, c1, -y, c2) == att(x, c1, y, c2));
                assert(x * (-y) == -(x * y) && un * (-vn) == -(un * vn)) by (nonlinear_arith);
                assert((x * (-y)) * (ud * vd) == -((x * y) * (ud * vd))) by (nonlinear_arith);
                assert((un * (-vn)) * (xd * yd) == -((un * vn) * (xd * yd))) by (nonlinear_arith);
            },
            (VSet::Upper(x, xd, c1), VSet::Lower(y, yd, c2)) => {
                assert(if c1 {
                    (-x) * ud <= (-un) * xd
                } else {
                    (-x) * ud < (-un) * xd
                }) by (nonlinear_arith)
                    requires
                        if c1 {
                            un * xd <= x * ud
                        } else {
                            un * xd < x * ud
                        },
                ;
                lower_lower(-x, xd, c1, y, yd, c2, -un, ud, vn, vd);
                assert(att(-x, c1, y, c2) == att(x, c1, y, c2));
                assert(((-x) * y) * (ud * vd) == -((x * y) * (ud * vd))) by (nonlinear_arith);
                assert(((-un) * vn) * (xd * yd) == -((un * vn) * (xd * yd))) by (nonlinear_arith);
            },
        }
    }
}

// Division by an exact nonzero value is the product with its reciprocal; the
// quotient's value (un/ud)/(vn/vd) is written over a positive denominator.
pub proof fn vquot_sound(a: VSet, b: VSet, un: int, ud: int, vn: int, vd: int)
    requires
        den_ok(a),
        den_ok(b),
        ud >= 1,
        vd >= 1,
        member(un, ud, a),
        member(vn, vd, b),
        vquot(a, b) is Some,
    ensures
        vn != 0,
        vn > 0 ==> member(un * vd, ud * vn, vquot(a, b)->Some_0),
        vn < 0 ==> member(-(un * vd), ud * (-vn), vquot(a, b)->Some_0),
{
    if let VSet::Point(y, yd) = b {
        assert(y != 0);
        assert(vn * yd == y * vd);
        assert((vn > 0) == (y > 0) && (vn < 0) == (y < 0)) by (nonlinear_arith)
            requires
                vn * yd == y * vd,
                yd >= 1,
                vd >= 1,
                y != 0,
        ;
        if y > 0 {
            assert(vd * y == yd * vn) by (nonlinear_arith)
                requires
                    vn * yd == y * vd,
            ;
            vprod_sound(a, VSet::Point(yd, y), un, ud, vd, vn);
        } else {
            assert((-vd) * (-y) == (-yd) * (-vn)) by (nonlinear_arith)
                requires
                    vn * yd == y * vd,
            ;
            vprod_sound(a, VSet::Point(-yd, -y), un, ud, -vd, -vn);
            assert(un * (-vd) == -(un * vd)) by (nonlinear_arith);
        }
    }
}

proof fn mod_scale(k: int, x: int, y: int)
    requires
        k >= 1,
        x >= 0,
        y >= 1,
    ensures
        (k * x) % (k * y) == k * (x % y),
        k * y >= 1,
{
    lemma_fundamental_div_mod(x, y);
    let q = x / y;
    let r = x % y;
    assert(k * x == (k * y) * q + k * r) by (nonlinear_arith)
        requires
            x == y * q + r,
    ;
    assert(0 <= k * r < k * y) by (nonlinear_arith)
        requires
            0 <= r < y,
            k >= 1,
    ;
    assert(k * y >= 1) by (nonlinear_arith)
        requires
            k >= 1,
            y >= 1,
    ;
    lemma_fundamental_div_mod_converse(k * x, k * y, q, k * r);
}

proof fn gcd_scale(k: nat, x: nat, y: nat)
    requires
        k >= 1,
    ensures
        gcd(k * x, k * y) == k * gcd(x, y),
    decreases y,
{
    reveal_with_fuel(gcd, 2);
    if y == 0 {
        assert(k * y == 0) by (nonlinear_arith)
            requires
                y == 0,
        ;
    } else {
        mod_scale(k as int, x as int, y as int);
        assert(k * y > 0) by (nonlinear_arith)
            requires
                k >= 1,
                y >= 1,
        ;
        gcd_scale(k, y, x % y);
        assert(((k * x) % (k * y)) as nat == k * (x % y));
    }
}

// canon(p, q) is an admitted number of value p/q (contract da1 P10).
pub proof fn canon_number(p: int, q: int)
    requires
        q >= 1,
    ensures
        number(canon(p, q)) matches Option::Some((n, d)) && n * q == p * d && d >= 1,
{
    let m = abs(p);
    crate::arith_impl::gcd_divides(m, q as nat);
    let g = gcd(m, q as nat);
    lemma_fundamental_div_mod(m as int, g as int);
    lemma_fundamental_div_mod(q, g as int);
    let mq = m / g;
    let qq = (q as nat) / g;
    assert(m == g * mq && q == g * qq);
    crate::arith_impl::exact_div(p, g as int);
    let pp = p / (g as int);
    assert(abs(pp) == mq);
    assert(pp * q == p * (qq as int)) by (nonlinear_arith)
        requires
            q == g * qq,
            p == (g as int) * pp || (p == g * mq && pp == mq as int) || (p == -(g * mq) as int && pp
                == -(mq as int)),
            (p >= 0 ==> pp == mq as int && p == (g * mq) as int),
            (p < 0 ==> pp == -(mq as int) && p == -((g * mq) as int)),
    ;
    assert(qq >= 1) by (nonlinear_arith)
        requires
            q == g * qq,
            q >= 1,
            g >= 1,
    ;
    assert((q / (g as int)) as nat == qq);
    if qq != 1 {
        gcd_scale(g, mq, qq);
        assert(g * mq == m && g * qq == q as nat);
        assert(gcd(mq, qq) == 1) by (nonlinear_arith)
            requires
                g * gcd(mq, qq) == g,
                g >= 1,
        ;
        assert(canon(p, q) == Term::Comp(
            rational_name(),
            seq![Term::Int(pp), Term::Int(qq as int)],
        ));
    }
}

proof fn vset_den(s: Term, n: Term)
    requires
        vset(s, n) is Some,
    ensures
        den_ok(vset(s, n)->Some_0),
{
}

proof fn vprod_den(a: VSet, b: VSet)
    requires
        den_ok(a),
        den_ok(b),
        vprod(a, b) is Some,
    ensures
        den_ok(vprod(a, b)->Some_0),
{
    let (ad, bd) = match (a, b) {
        (VSet::Point(_, x), VSet::Point(_, y)) => (x, y),
        (VSet::Point(_, x), VSet::Lower(_, y, _)) => (x, y),
        (VSet::Point(_, x), VSet::Upper(_, y, _)) => (x, y),
        (VSet::Lower(_, x, _), VSet::Point(_, y)) => (x, y),
        (VSet::Lower(_, x, _), VSet::Lower(_, y, _)) => (x, y),
        (VSet::Lower(_, x, _), VSet::Upper(_, y, _)) => (x, y),
        (VSet::Upper(_, x, _), VSet::Point(_, y)) => (x, y),
        (VSet::Upper(_, x, _), VSet::Lower(_, y, _)) => (x, y),
        (VSet::Upper(_, x, _), VSet::Upper(_, y, _)) => (x, y),
    };
    assert(ad * bd >= 1 && bd * ad >= 1) by (nonlinear_arith)
        requires
            ad >= 1,
            bd >= 1,
    ;
}

proof fn vquot_den(a: VSet, b: VSet)
    requires
        den_ok(a),
        den_ok(b),
        vquot(a, b) is Some,
    ensures
        den_ok(vquot(a, b)->Some_0),
{
    if let VSet::Point(y, yd) = b {
        if y > 0 {
            vprod_den(a, VSet::Point(yd, y));
        } else {
            vprod_den(a, VSet::Point(-yd, -y));
        }
    }
}

// A spelled result decodes to a value set with the members of the set it spells.
proof fn spell_decodes(v: VSet, x: int, xd: int)
    requires
        den_ok(v),
        xd >= 1,
        member(x, xd, v),
    ensures
        vset(spell(v).0, spell(v).1) matches Option::Some(w) && member(x, xd, w),
{
    crate::arith_impl::keywords_distinct();
    match v {
        VSet::Point(n, d) => {
            canon_number(n, d);
            let (m, e) = number(canon(n, d))->Some_0;
            assert((x * e - m * xd) * d == 0) by (nonlinear_arith)
                requires
                    x * d == n * xd,
                    m * d == n * e,
            ;
            assert(x * e == m * xd) by (nonlinear_arith)
                requires
                    (x * e - m * xd) * d == 0,
                    d >= 1,
            ;
        },
        VSet::Lower(n, d, c) => {
            canon_number(n, d);
            let (m, e) = number(canon(n, d))->Some_0;
            assert((m * xd) * d == e * (n * xd) && (x * e) * d == e * (x * d)) by (nonlinear_arith)
                requires
                    m * d == n * e,
            ;
            if c {
                assert(e * (n * xd) <= e * (x * d)) by (nonlinear_arith)
                    requires
                        n * xd <= x * d,
                        e >= 1,
                ;
                assert(m * xd <= x * e) by (nonlinear_arith)
                    requires
                        (m * xd) * d <= (x * e) * d,
                        d >= 1,
                ;
            } else {
                assert(e * (n * xd) < e * (x * d)) by (nonlinear_arith)
                    requires
                        n * xd < x * d,
                        e >= 1,
                ;
                assert(m * xd < x * e) by (nonlinear_arith)
                    requires
                        (m * xd) * d < (x * e) * d,
                        d >= 1,
                ;
            }
        },
        VSet::Upper(n, d, c) => {
            canon_number(n, d);
            let (m, e) = number(canon(n, d))->Some_0;
            assert((m * xd) * d == e * (n * xd) && (x * e) * d == e * (x * d)) by (nonlinear_arith)
                requires
                    m * d == n * e,
            ;
            if c {
                assert(e * (x * d) <= e * (n * xd)) by (nonlinear_arith)
                    requires
                        x * d <= n * xd,
                        e >= 1,
                ;
                assert(x * e <= m * xd) by (nonlinear_arith)
                    requires
                        (x * e) * d <= (m * xd) * d,
                        d >= 1,
                ;
            } else {
                assert(e * (x * d) < e * (n * xd)) by (nonlinear_arith)
                    requires
                        x * d < n * xd,
                        e >= 1,
                ;
                assert(x * e < m * xd) by (nonlinear_arith)
                    requires
                        (x * e) * d < (m * xd) * d,
                        d >= 1,
                ;
            }
        },
    }
}

// P10 over public terms: a computed result pair decodes to a value set that
// holds every product (quotient) of values of the input sets.
pub proof fn arith_result_sound(name: Seq<u8>, args: Seq<Term>, un: int, ud: int, vn: int, vd: int)
    requires
        is_arith(name, args.len()),
        arith_result(Term::Comp(name, args)) is Some,
        ud >= 1,
        vd >= 1,
        vset(args[0], args[1]) matches Option::Some(a) && member(un, ud, a),
        vset(args[2], args[3]) matches Option::Some(b) && member(vn, vd, b),
    ensures
        ({
            let (s, n) = arith_result(Term::Comp(name, args))->Some_0;
            vset(s, n) matches Option::Some(w) && (name == product_name() ==> member(
                un * vn,
                ud * vd,
                w,
            )) && (name != product_name() ==> vn != 0 && (vn > 0 ==> member(un * vd, ud * vn, w))
                && (vn < 0 ==> member(-(un * vd), ud * (-vn), w)))
        }),
{
    let a = vset(args[0], args[1])->Some_0;
    let b = vset(args[2], args[3])->Some_0;
    vset_den(args[0], args[1]);
    vset_den(args[2], args[3]);
    if name == product_name() {
        vprod_sound(a, b, un, ud, vn, vd);
        vprod_den(a, b);
        assert(ud * vd >= 1) by (nonlinear_arith)
            requires
                ud >= 1,
                vd >= 1,
        ;
        spell_decodes(vprod(a, b)->Some_0, un * vn, ud * vd);
    } else {
        vquot_sound(a, b, un, ud, vn, vd);
        vquot_den(a, b);
        if vn > 0 {
            assert(ud * vn >= 1) by (nonlinear_arith)
                requires
                    ud >= 1,
                    vn >= 1,
            ;
            spell_decodes(vquot(a, b)->Some_0, un * vd, ud * vn);
        } else {
            assert(ud * (-vn) >= 1) by (nonlinear_arith)
                requires
                    ud >= 1,
                    -vn >= 1,
            ;
            spell_decodes(vquot(a, b)->Some_0, -(un * vd), ud * (-vn));
        }
    }
}

} // verus!
