use crate::term::*;
use crate::v1text::*;
use vstd::prelude::*;

verus! {

// Trusted spec: exact numbers + the arithmetic built-ins of the schema
// (contract da1 D1–D5). A number term = an integer or `rational(P, Q)` with
// Q ≥ 2 and gcd(|P|, Q) = 1, value P/Q: one term per rational value, so
// unifying number terms compares values. A (comparison, count) pair denotes
// a value set (nt1: a plain number states at least that many); a product is
// computed when its image is a point or a ray, a quotient needs an exact
// nonzero divisor, and anything else derives nothing. The engine evaluates
// the built-ins (`engine::step`), the trace check re-evaluates them
// (`trace::node_valid`), and `guidelines/schema.pl` states them for standard
// Prolog engines (`compare::schema_bytes`).
pub open spec fn rational_name() -> Seq<u8> {
    ascii("rational"@)
}

pub open spec fn product_name() -> Seq<u8> {
    ascii("guideline_product"@)
}

pub open spec fn quotient_name() -> Seq<u8> {
    ascii("guideline_quotient"@)
}

// guideline_product/6 and guideline_quotient/6: inputs S1, N1, S2, N2;
// outputs S, N. Neither is a document predicate: no clause heads one.
pub open spec fn is_arith(name: Seq<u8>, arity: nat) -> bool {
    arity == 6 && (name == product_name() || name == quotient_name())
}

pub open spec fn gcd(a: nat, b: nat) -> nat
    decreases b,
{
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub open spec fn abs(p: int) -> nat {
    if p < 0 {
        (-p) as nat
    } else {
        p as nat
    }
}

// The value (numerator, denominator ≥ 1) of a number term.
pub open spec fn number(t: Term) -> Option<(int, int)> {
    match t {
        Term::Int(n) => Option::Some((n, 1)),
        Term::Comp(name, args) => if name == rational_name() && args.len() == 2 {
            match (args[0], args[1]) {
                (Term::Int(p), Term::Int(q)) => if q >= 2 && gcd(abs(p), q as nat) == 1 {
                    Option::Some((p, q))
                } else {
                    Option::None
                },
                _ => Option::None,
            }
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

// The number term of p/q (q ≥ 1).
pub open spec fn canon(p: int, q: int) -> Term {
    let g = gcd(abs(p), q as nat) as int;
    if q / g == 1 {
        Term::Int(p / g)
    } else {
        Term::Comp(rational_name(), seq![Term::Int(p / g), Term::Int(q / g)])
    }
}

// A value set over the rationals; each bound = (numerator, denominator ≥ 1).
pub ghost enum VSet {
    Point(int, int),  // {n/d}
    Lower(int, int, bool),  // [n/d, ∞) when closed, else (n/d, ∞)
    Upper(int, int, bool),  // (−∞, n/d] when closed, else (−∞, n/d)
}

pub open spec fn kw(s: Seq<char>) -> Term {
    Term::Atom(ascii(s))
}

pub open spec fn vset(s: Term, n: Term) -> Option<VSet> {
    match number(n) {
        Option::None => Option::None,
        Option::Some((p, q)) => if s == kw("exactly"@) {
            Option::Some(VSet::Point(p, q))
        } else if s == kw("eq"@) || s == kw("geq"@) {
            Option::Some(VSet::Lower(p, q, true))
        } else if s == kw("greater"@) {
            Option::Some(VSet::Lower(p, q, false))
        } else if s == kw("leq"@) {
            Option::Some(VSet::Upper(p, q, true))
        } else if s == kw("less"@) {
            Option::Some(VSet::Upper(p, q, false))
        } else {
            Option::None
        },
    }
}

pub open spec fn zero_point(v: VSet) -> bool {
    match v {
        VSet::Point(n, _) => n == 0,
        _ => false,
    }
}

// The bound of a ray product is attained iff an included zero bound makes the
// other factor irrelevant, or both bounds are attained.
pub open spec fn att(x: int, c1: bool, y: int, c2: bool) -> bool {
    (x == 0 && c1) || (y == 0 && c2) || (c1 && c2)
}

// The nonzero point x/xd times a value set (a negative factor flips a ray).
pub open spec fn stretch(x: int, xd: int, v: VSet) -> VSet {
    match v {
        VSet::Point(y, yd) => VSet::Point(x * y, xd * yd),
        VSet::Lower(y, yd, c) => if x > 0 {
            VSet::Lower(x * y, xd * yd, c)
        } else {
            VSet::Upper(x * y, xd * yd, c)
        },
        VSet::Upper(y, yd, c) => if x > 0 {
            VSet::Upper(x * y, xd * yd, c)
        } else {
            VSet::Lower(x * y, xd * yd, c)
        },
    }
}

// {u·v : u ∈ a, v ∈ b} when that set is a point or a ray; None = the whole line.
pub open spec fn vprod(a: VSet, b: VSet) -> Option<VSet> {
    if zero_point(a) || zero_point(b) {
        Option::Some(VSet::Point(0, 1))
    } else {
        match (a, b) {
            (VSet::Point(x, xd), _) => Option::Some(stretch(x, xd, b)),
            (_, VSet::Point(y, yd)) => Option::Some(stretch(y, yd, a)),
            (VSet::Lower(x, xd, c1), VSet::Lower(y, yd, c2)) => if x >= 0 && y >= 0 {
                Option::Some(VSet::Lower(x * y, xd * yd, att(x, c1, y, c2)))
            } else {
                Option::None
            },
            (VSet::Upper(x, xd, c1), VSet::Upper(y, yd, c2)) => if x <= 0 && y <= 0 {
                Option::Some(VSet::Lower(x * y, xd * yd, att(x, c1, y, c2)))
            } else {
                Option::None
            },
            (VSet::Lower(x, xd, c1), VSet::Upper(y, yd, c2)) => if x >= 0 && y <= 0 {
                Option::Some(VSet::Upper(x * y, xd * yd, att(x, c1, y, c2)))
            } else {
                Option::None
            },
            (VSet::Upper(x, xd, c1), VSet::Lower(y, yd, c2)) => if x <= 0 && y >= 0 {
                Option::Some(VSet::Upper(x * y, xd * yd, att(x, c1, y, c2)))
            } else {
                Option::None
            },
        }
    }
}

// Division by an exact nonzero divisor = the product with its reciprocal.
pub open spec fn vquot(a: VSet, b: VSet) -> Option<VSet> {
    match b {
        VSet::Point(y, yd) => if y > 0 {
            vprod(a, VSet::Point(yd, y))
        } else if y < 0 {
            vprod(a, VSet::Point(-yd, -y))
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

// A result spells back as a comparison + its canonical count, never `eq`.
pub open spec fn spell(v: VSet) -> (Term, Term) {
    match v {
        VSet::Point(n, d) => (kw("exactly"@), canon(n, d)),
        VSet::Lower(n, d, c) => (
            if c {
                kw("geq"@)
            } else {
                kw("greater"@)
            },
            canon(n, d),
        ),
        VSet::Upper(n, d, c) => (
            if c {
                kw("leq"@)
            } else {
                kw("less"@)
            },
            canon(n, d),
        ),
    }
}

// The output pair an arithmetic goal computes from its inputs, if any.
pub open spec fn arith_result(t: Term) -> Option<(Term, Term)> {
    match t {
        Term::Comp(name, args) => if is_arith(name, args.len()) {
            match (vset(args[0], args[1]), vset(args[2], args[3])) {
                (Option::Some(a), Option::Some(b)) => {
                    let r = if name == product_name() {
                        vprod(a, b)
                    } else {
                        vquot(a, b)
                    };
                    match r {
                        Option::Some(v) => Option::Some(spell(v)),
                        Option::None => Option::None,
                    }
                },
                _ => Option::None,
            }
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

// A ground arithmetic goal holds: its outputs are the computed pair.
pub open spec fn arith_holds(t: Term) -> bool {
    match (t, arith_result(t)) {
        (Term::Comp(_, args), Option::Some((s, n))) => args[4] == s && args[5] == n,
        _ => false,
    }
}

} // verus!
