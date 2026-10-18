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
// stated in time units. Neither is a document predicate: no clause heads one.
pub open spec fn is_builtin(name: Seq<u8>, arity: nat) -> bool {
    name == satisfies_name() && (arity == 4 || arity == 6)
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

// The built-in goal holds: its arguments have the shapes above and meet.
pub open spec fn holds(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => name == satisfies_name() && if args.len() == 4 {
            match (args[0], args[1], args[2], args[3]) {
                (Term::Atom(s), Term::Int(n), Term::Atom(b), Term::Int(m)) => within(s, n, b, m),
                _ => false,
            }
        } else if args.len() == 6 {
            match (args[0], args[1], args[2], args[3], args[4], args[5]) {
                (
                    Term::Atom(s),
                    Term::Int(n),
                    Term::Atom(u),
                    Term::Atom(b),
                    Term::Int(m),
                    Term::Atom(v),
                ) => match (scale_of(u), scale_of(v)) {
                    (Option::Some((g, r)), Option::Some((h, q))) => g == h && within(
                        s,
                        n * r,
                        b,
                        m * q,
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

// --- guidelines/schema.pl, byte for byte (contract nt1 D10) ---
// Guards make every call total: an argument of another shape fails, as `holds`
// is false there, instead of raising an ISO arithmetic error.
pub open spec fn schema_header() -> Seq<u8> {
    ascii(
        "% schema.pl: the comparison built-ins of the compiled Prolog schema (docs/REFERENCE.md); generated by ckc from the formal specification; do not edit; first-party code, Apache-2.0 WITH LLVM-exception.\n"@,
    )
}

pub open spec fn schema_rules() -> Seq<u8> {
    ascii(
        "guideline_satisfies(S, N, B, M) :- atom(S), integer(N), atom(B), integer(M), guideline_within(S, N, B, M).\n"@,
    ) + ascii(
        "guideline_satisfies(S, N, U, B, M, V) :- atom(S), integer(N), atom(U), atom(B), integer(M), atom(V), guideline_scale(U, G, R), guideline_scale(V, G, Q), X is N * R, Y is M * Q, guideline_within(S, X, B, Y).\n"@,
    )
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
