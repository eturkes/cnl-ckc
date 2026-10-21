use crate::arith::*;
use crate::term::*;
use crate::v1text::*;
use vstd::prelude::*;

verus! {

// Trusted spec: the comparison built-ins of the schema (contract nt1 D1, D10).
// A condition such as `at least 50` is met by a stated quantity when every
// value the stated quantity allows is a value the condition allows (value-set
// inclusion over the dense order): a plain number (`eq`) allows that count or
// more, as APE reads a numeral; `exactly` allows the count alone. Time units
// compare after scaling inside their group (seconds through weeks; months and
// years); a day and a month never compare. The engine evaluates the built-ins
// (`engine::step`), the trace check re-evaluates them (`trace::node_valid`),
// and `guidelines/schema.pl` states them for standard Prolog engines.
pub open spec fn satisfies_name() -> Seq<u8> {
    ascii("guideline_satisfies"@)
}

// guideline_satisfies/4 compares counts; guideline_satisfies/6 compares counts
// stated in time units (contract nt1 D1, rational counts da1 D6).
pub open spec fn is_compare(name: Seq<u8>, arity: nat) -> bool {
    name == satisfies_name() && (arity == 4 || arity == 6)
}

// The schema built-ins: the comparisons + the arithmetic of `arith.rs`. None
// is a document predicate: no clause heads one, no declaration names one.
pub open spec fn is_builtin(name: Seq<u8>, arity: nat) -> bool {
    is_compare(name, arity) || is_arith(name, arity)
}

// One row per (stated comparison, bound comparison) pair that can meet: the ISO
// comparison the two counts must pass. A pair without a row never meets.
pub open spec fn within_rows() -> Seq<(Seq<u8>, Seq<u8>, Seq<u8>)> {
    seq![
        (ascii("eq"@), ascii("geq"@), ascii(">="@)),
        (ascii("exactly"@), ascii("geq"@), ascii(">="@)),
        (ascii("geq"@), ascii("geq"@), ascii(">="@)),
        (ascii("greater"@), ascii("geq"@), ascii(">="@)),
        (ascii("eq"@), ascii("greater"@), ascii(">"@)),
        (ascii("exactly"@), ascii("greater"@), ascii(">"@)),
        (ascii("geq"@), ascii("greater"@), ascii(">"@)),
        (ascii("greater"@), ascii("greater"@), ascii(">="@)),
        (ascii("exactly"@), ascii("leq"@), ascii("=<"@)),
        (ascii("leq"@), ascii("leq"@), ascii("=<"@)),
        (ascii("less"@), ascii("leq"@), ascii("=<"@)),
        (ascii("exactly"@), ascii("less"@), ascii("<"@)),
        (ascii("leq"@), ascii("less"@), ascii("<"@)),
        (ascii("less"@), ascii("less"@), ascii("=<"@)),
        (ascii("exactly"@), ascii("exactly"@), ascii("=:="@)),
    ]
}

pub open spec fn op_holds(op: Seq<u8>, x: int, y: int) -> bool {
    if op == ascii(">="@) {
        x >= y
    } else if op == ascii(">"@) {
        x > y
    } else if op == ascii("=<"@) {
        x <= y
    } else if op == ascii("<"@) {
        x < y
    } else {
        op == ascii("=:="@) && x == y
    }
}

pub open spec fn within_from(
    rows: Seq<(Seq<u8>, Seq<u8>, Seq<u8>)>,
    s: Seq<u8>,
    x: int,
    b: Seq<u8>,
    y: int,
) -> bool
    decreases rows.len(),
{
    rows.len() > 0 && ((rows[0].0 == s && rows[0].1 == b && op_holds(rows[0].2, x, y))
        || within_from(rows.drop_first(), s, x, b, y))
}

pub open spec fn within(s: Seq<u8>, x: int, b: Seq<u8>, y: int) -> bool {
    within_from(within_rows(), s, x, b, y)
}

// Unit id (the vocabulary table's closed set), group, count of the group's
// smallest unit in one of it.
pub open spec fn scale_rows() -> Seq<(Seq<u8>, Seq<u8>, int)> {
    seq![
        (ascii("second"@), ascii("time"@), 1int),
        (ascii("minute"@), ascii("time"@), 60int),
        (ascii("hour"@), ascii("time"@), 3600int),
        (ascii("day"@), ascii("time"@), 86400int),
        (ascii("week"@), ascii("time"@), 604800int),
        (ascii("month"@), ascii("calendar"@), 1int),
        (ascii("year"@), ascii("calendar"@), 12int),
    ]
}

pub open spec fn scale_from(rows: Seq<(Seq<u8>, Seq<u8>, int)>, u: Seq<u8>) -> Option<
    (Seq<u8>, int),
>
    decreases rows.len(),
{
    if rows.len() == 0 {
        Option::None
    } else if rows[0].0 == u {
        Option::Some((rows[0].1, rows[0].2))
    } else {
        scale_from(rows.drop_first(), u)
    }
}

pub open spec fn scale_of(u: Seq<u8>) -> Option<(Seq<u8>, int)> {
    scale_from(scale_rows(), u)
}

// A comparison holds: its arguments have the shapes above and meet. Counts are
// numbers (`arith::number`); p1/q1 against p2/q2 compares p1·q2 with p2·q1, the
// denominators being positive.
pub open spec fn compare_holds(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => name == satisfies_name() && if args.len() == 4 {
            match (args[0], number(args[1]), args[2], number(args[3])) {
                (
                    Term::Atom(s),
                    Option::Some((p1, q1)),
                    Term::Atom(b),
                    Option::Some((p2, q2)),
                ) => within(s, p1 * q2, b, p2 * q1),
                _ => false,
            }
        } else if args.len() == 6 {
            match (args[0], number(args[1]), args[2], args[3], number(args[4]), args[5]) {
                (
                    Term::Atom(s),
                    Option::Some((p1, q1)),
                    Term::Atom(u),
                    Term::Atom(b),
                    Option::Some((p2, q2)),
                    Term::Atom(v),
                ) => match (scale_of(u), scale_of(v)) {
                    (Option::Some((g, r)), Option::Some((h, q))) => g == h && within(
                        s,
                        p1 * r * q2,
                        b,
                        p2 * q * q1,
                    ),
                    _ => false,
                },
                _ => false,
            }
        } else {
            false
        },
        _ => false,
    }
}

// A built-in goal holds: a comparison meets, or an arithmetic goal's outputs
// are its computed pair.
pub open spec fn holds(t: Term) -> bool {
    compare_holds(t) || arith_holds(t)
}

// --- guidelines/schema.pl, byte for byte (contracts nt1 D10, da1 D10) ---
// Guards make every call total: an input of another shape fails, as `holds`
// is false there, instead of raising an ISO arithmetic error. The arithmetic
// built-ins validate their four inputs without binding them, compute the
// result in fresh terms, then unify the two outputs (as the engine does);
// helpers keep explicit input modes; no cut, at most one solution.
pub open spec fn schema_header() -> Seq<u8> {
    ascii(
        "% schema.pl: the comparison and arithmetic built-ins of the compiled Prolog schema (docs/REFERENCE.md); generated by ckc from the formal specification; do not edit; first-party code, Apache-2.0 WITH LLVM-exception.\n"@,
    )
}

// The four public built-ins.
pub open spec fn schema_entry() -> Seq<u8> {
    ascii(
        "guideline_satisfies(S, N, B, M) :- atom(S), atom(B), guideline_number(N, P, Q), guideline_number(M, R, T), X is P * T, Y is R * Q, guideline_within(S, X, B, Y).\n"@,
    ) + ascii(
        "guideline_satisfies(S, N, U, B, M, V) :- atom(S), atom(U), atom(B), atom(V), guideline_number(N, P, Q), guideline_number(M, R, T), guideline_scale(U, G, A), guideline_scale(V, G, C), X is P * A * T, Y is R * C * Q, guideline_within(S, X, B, Y).\n"@,
    ) + ascii(
        "guideline_product(S1, N1, S2, N2, S, N) :- guideline_set(S1, N1, A), guideline_set(S2, N2, B), guideline_times(A, B, C), guideline_spell(C, S0, N0), S = S0, N = N0.\n"@,
    ) + ascii(
        "guideline_quotient(S1, N1, S2, N2, S, N) :- guideline_set(S1, N1, A), guideline_set(S2, N2, B), guideline_inverse(B, D), guideline_times(A, D, C), guideline_spell(C, S0, N0), S = S0, N = N0.\n"@,
    )
}

// Number terms (contract da1 D1) + the gcd.
pub open spec fn schema_numbers() -> Seq<u8> {
    ascii("guideline_number(N, N, 1) :- integer(N).\n"@) + ascii(
        "guideline_number(N, P, Q) :- compound(N), functor(N, rational, 2), arg(1, N, P), arg(2, N, Q), integer(P), integer(Q), Q >= 2, A is abs(P), guideline_gcd(A, Q, 1).\n"@,
    ) + ascii("guideline_gcd(A, 0, A).\n"@) + ascii(
        "guideline_gcd(A, B, G) :- B > 0, C is A mod B, guideline_gcd(B, C, G).\n"@,
    )
}

// Value sets (da1 D2).
pub open spec fn schema_sets() -> Seq<u8> {
    ascii(
        "guideline_set(S, N, V) :- atom(S), guideline_number(N, P, Q), guideline_ray(S, P, Q, V).\n"@,
    ) + ascii("guideline_ray(exactly, P, Q, point(P, Q)).\n"@) + ascii(
        "guideline_ray(eq, P, Q, lower(P, Q, true)).\n"@,
    ) + ascii("guideline_ray(geq, P, Q, lower(P, Q, true)).\n"@) + ascii(
        "guideline_ray(greater, P, Q, lower(P, Q, false)).\n"@,
    ) + ascii("guideline_ray(leq, P, Q, upper(P, Q, true)).\n"@) + ascii(
        "guideline_ray(less, P, Q, upper(P, Q, false)).\n"@,
    )
}

// The product table (da1 D3).
pub open spec fn schema_times() -> Seq<u8> {
    ascii("guideline_times(point(X, _), _, point(0, 1)) :- X =:= 0.\n"@) + ascii(
        "guideline_times(A, point(Y, _), point(0, 1)) :- Y =:= 0, guideline_nonzero(A).\n"@,
    ) + ascii(
        "guideline_times(point(X, D), B, C) :- X =\\= 0, guideline_nonzero(B), guideline_stretch(X, D, B, C).\n"@,
    ) + ascii(
        "guideline_times(A, point(Y, E), C) :- Y =\\= 0, guideline_ray_kind(A), guideline_stretch(Y, E, A, C).\n"@,
    ) + ascii(
        "guideline_times(lower(X, D, F), lower(Y, E, G), lower(Z, W, K)) :- X >= 0, Y >= 0, Z is X * Y, W is D * E, guideline_att(X, F, Y, G, K).\n"@,
    ) + ascii(
        "guideline_times(upper(X, D, F), upper(Y, E, G), lower(Z, W, K)) :- X =< 0, Y =< 0, Z is X * Y, W is D * E, guideline_att(X, F, Y, G, K).\n"@,
    ) + ascii(
        "guideline_times(lower(X, D, F), upper(Y, E, G), upper(Z, W, K)) :- X >= 0, Y =< 0, Z is X * Y, W is D * E, guideline_att(X, F, Y, G, K).\n"@,
    ) + ascii(
        "guideline_times(upper(X, D, F), lower(Y, E, G), upper(Z, W, K)) :- X =< 0, Y >= 0, Z is X * Y, W is D * E, guideline_att(X, F, Y, G, K).\n"@,
    )
}

// Its operand kinds + the scaling of a set by a nonzero point.
pub open spec fn schema_stretch() -> Seq<u8> {
    ascii("guideline_nonzero(point(X, _)) :- X =\\= 0.\n"@) + ascii(
        "guideline_nonzero(lower(_, _, _)).\n"@,
    ) + ascii("guideline_nonzero(upper(_, _, _)).\n"@) + ascii(
        "guideline_ray_kind(lower(_, _, _)).\n"@,
    ) + ascii("guideline_ray_kind(upper(_, _, _)).\n"@) + ascii(
        "guideline_stretch(X, D, point(Y, E), point(Z, W)) :- Z is X * Y, W is D * E.\n"@,
    ) + ascii(
        "guideline_stretch(X, D, lower(Y, E, F), lower(Z, W, F)) :- X > 0, Z is X * Y, W is D * E.\n"@,
    ) + ascii(
        "guideline_stretch(X, D, lower(Y, E, F), upper(Z, W, F)) :- X < 0, Z is X * Y, W is D * E.\n"@,
    ) + ascii(
        "guideline_stretch(X, D, upper(Y, E, F), upper(Z, W, F)) :- X > 0, Z is X * Y, W is D * E.\n"@,
    ) + ascii(
        "guideline_stretch(X, D, upper(Y, E, F), lower(Z, W, F)) :- X < 0, Z is X * Y, W is D * E.\n"@,
    )
}

// Bound attainment.
pub open spec fn schema_att() -> Seq<u8> {
    ascii("guideline_att(_, true, _, true, true).\n"@) + ascii(
        "guideline_att(X, true, _, false, K) :- guideline_zero(X, K).\n"@,
    ) + ascii("guideline_att(_, false, Y, true, K) :- guideline_zero(Y, K).\n"@) + ascii(
        "guideline_att(_, false, _, false, false).\n"@,
    ) + ascii("guideline_zero(X, true) :- X =:= 0.\n"@) + ascii(
        "guideline_zero(X, false) :- X =\\= 0.\n"@,
    )
}

// The reciprocal (da1 D4) + the canonical spelling.
pub open spec fn schema_spell() -> Seq<u8> {
    ascii("guideline_inverse(point(Y, E), point(E, Y)) :- Y > 0.\n"@) + ascii(
        "guideline_inverse(point(Y, E), point(F, Z)) :- Y < 0, F is -E, Z is -Y.\n"@,
    ) + ascii("guideline_spell(point(P, Q), exactly, N) :- guideline_canon(P, Q, N).\n"@) + ascii(
        "guideline_spell(lower(P, Q, true), geq, N) :- guideline_canon(P, Q, N).\n"@,
    ) + ascii("guideline_spell(lower(P, Q, false), greater, N) :- guideline_canon(P, Q, N).\n"@)
        + ascii("guideline_spell(upper(P, Q, true), leq, N) :- guideline_canon(P, Q, N).\n"@)
        + ascii("guideline_spell(upper(P, Q, false), less, N) :- guideline_canon(P, Q, N).\n"@)
        + ascii(
        "guideline_canon(P, Q, N) :- A is abs(P), guideline_gcd(A, Q, G), R is P // G, T is Q // G, guideline_make(R, T, N).\n"@,
    ) + ascii("guideline_make(P, 1, P).\n"@) + ascii(
        "guideline_make(P, Q, rational(P, Q)) :- Q > 1.\n"@,
    )
}

pub open spec fn schema_rules() -> Seq<u8> {
    schema_entry() + schema_numbers() + schema_sets() + schema_times() + schema_stretch()
        + schema_att() + schema_spell()
}

pub open spec fn within_line(r: (Seq<u8>, Seq<u8>, Seq<u8>)) -> Seq<u8> {
    ascii("guideline_within("@) + r.0 + ascii(", X, "@) + r.1 + ascii(", Y) :- X "@) + r.2 + ascii(
        " Y.\n"@,
    )
}

pub open spec fn scale_line(r: (Seq<u8>, Seq<u8>, int)) -> Seq<u8> {
    ascii("guideline_scale("@) + r.0 + ascii(", "@) + r.1 + ascii(", "@) + udec_bytes(r.2 as nat)
        + ascii(").\n"@)
}

pub open spec fn within_lines(rows: Seq<(Seq<u8>, Seq<u8>, Seq<u8>)>) -> Seq<u8>
    decreases rows.len(),
{
    if rows.len() == 0 {
        Seq::empty()
    } else {
        within_line(rows[0]) + within_lines(rows.drop_first())
    }
}

pub open spec fn scale_lines(rows: Seq<(Seq<u8>, Seq<u8>, int)>) -> Seq<u8>
    decreases rows.len(),
{
    if rows.len() == 0 {
        Seq::empty()
    } else {
        scale_line(rows[0]) + scale_lines(rows.drop_first())
    }
}

pub open spec fn schema_bytes() -> Seq<u8> {
    schema_header() + schema_rules() + within_lines(within_rows()) + scale_lines(scale_rows())
}

} // verus!
