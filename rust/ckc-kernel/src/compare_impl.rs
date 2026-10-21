use crate::compare_schema::{ascii_eq, schema_bytes};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid};
use crate::k2_term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::compare as c;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{ascii, udec_bytes};
use vstd::prelude::*;

verus! {

pub fn schema_file_impl() -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::compare::schema_bytes(),
{
    schema_bytes()
}

pub fn is_builtin_exec(name: &Vec<u8>, arity: usize) -> (r: bool)
    ensures
        r == ckc_spec::compare::is_builtin(name@, arity as nat),
{
    proof {
        reveal_strlit("guideline_satisfies");
    }
    (arity == 4 || arity == 6) && ascii_eq(name, "guideline_satisfies") || is_arith_exec(
        name,
        arity,
    )
}

proof fn operator_cases(x: int, y: int)
    ensures
        c::op_holds(ascii(">="@), x, y) == (x >= y),
        c::op_holds(ascii(">"@), x, y) == (x > y),
        c::op_holds(ascii("=<"@), x, y) == (x <= y),
        c::op_holds(ascii("<"@), x, y) == (x < y),
        c::op_holds(ascii("=:="@), x, y) == (x == y),
{
    reveal_strlit(">=");
    reveal_strlit(">");
    reveal_strlit("=<");
    reveal_strlit("<");
    reveal_strlit("=:=");
    assert(ascii(">="@).len() == 2);
    assert(ascii(">"@).len() == 1);
    assert(ascii("=<"@).len() == 2);
    assert(ascii("<"@).len() == 1);
    assert(ascii("=:="@).len() == 3);
    assert(ascii(">="@)[0] != ascii("=<"@)[0]);
    assert(ascii(">"@)[0] != ascii("<"@)[0]);
}

proof fn within_cases(s: Seq<u8>, x: int, b: Seq<u8>, y: int)
    ensures
        c::within(s, x, b, y) == ((b == ascii("geq"@) && (s == ascii("eq"@) || s == ascii(
            "exactly"@,
        ) || s == ascii("geq"@) || s == ascii("greater"@)) && x >= y) || (b == ascii("greater"@)
            && ((s == ascii("eq"@) || s == ascii("exactly"@) || s == ascii("geq"@)) && x > y || s
            == ascii("greater"@) && x >= y)) || (b == ascii("leq"@) && (s == ascii("exactly"@) || s
            == ascii("leq"@) || s == ascii("less"@)) && x <= y) || (b == ascii("less"@) && ((s
            == ascii("exactly"@) || s == ascii("leq"@)) && x < y || s == ascii("less"@) && x <= y))
            || (b == ascii("exactly"@) && s == ascii("exactly"@) && x == y)),
{
    operator_cases(x, y);
    reveal(c::within);
    reveal(c::within_rows);
    assert(c::within_rows().len() == 15);
    reveal_with_fuel(c::within_from, 16);
}

fn within_order(s: &Vec<u8>, b: &Vec<u8>, order: &EOrder, x: Ghost<int>, y: Ghost<int>) -> (r: bool)
    requires
        int_order_ok(order, x@, y@),
    ensures
        r == c::within(s@, x@, b@, y@),
{
    proof {
        reveal_strlit("eq");
        reveal_strlit("exactly");
        reveal_strlit("geq");
        reveal_strlit("greater");
        reveal_strlit("leq");
        reveal_strlit("less");
        assert(ascii("geq"@)[0] != ascii("leq"@)[0]);
        assert(ascii("greater"@)[0] != ascii("exactly"@)[0]);
        within_cases(s@, x@, b@, y@);
    }
    let exact = ascii_eq(s, "exactly");
    let lower = exact || ascii_eq(s, "eq") || ascii_eq(s, "geq");
    let greater = ascii_eq(s, "greater");
    let upper = exact || ascii_eq(s, "leq");
    let less = ascii_eq(s, "less");
    let (lt, equal, gt) = match order {
        EOrder::Less => (true, false, false),
        EOrder::Equal => (false, true, false),
        EOrder::Greater => (false, false, true),
    };
    if ascii_eq(b, "geq") {
        (lower || greater) && !lt
    } else if ascii_eq(b, "greater") {
        lower && gt || greater && !lt
    } else if ascii_eq(b, "leq") {
        (upper || less) && !gt
    } else if ascii_eq(b, "less") {
        upper && lt || less && !gt
    } else {
        ascii_eq(b, "exactly") && exact && equal
    }
}

pub open spec fn scale_group(calendar: bool) -> Seq<u8> {
    if calendar {
        ascii("calendar"@)
    } else {
        ascii("time"@)
    }
}

pub open spec fn valid_scale(calendar: bool, n: usize) -> bool {
    if calendar {
        n == 1 || n == 12
    } else {
        n == 1 || n == 60 || n == 3600 || n == 86400 || n == 604800
    }
}

proof fn scale_cases(u: Seq<u8>)
    ensures
        c::scale_of(u) == (if u == ascii("second"@) {
            Some((ascii("time"@), 1int))
        } else if u == ascii("minute"@) {
            Some((ascii("time"@), 60int))
        } else if u == ascii("hour"@) {
            Some((ascii("time"@), 3600int))
        } else if u == ascii("day"@) {
            Some((ascii("time"@), 86400int))
        } else if u == ascii("week"@) {
            Some((ascii("time"@), 604800int))
        } else if u == ascii("month"@) {
            Some((ascii("calendar"@), 1int))
        } else if u == ascii("year"@) {
            Some((ascii("calendar"@), 12int))
        } else {
            None
        }),
{
    reveal_with_fuel(c::scale_from, 8);
}

fn unit_scale(u: &Vec<u8>) -> (r: Option<(bool, usize)>)
    ensures
        match r {
            Some((calendar, n)) => c::scale_of(u@) == Some((scale_group(calendar), n as int))
                && valid_scale(calendar, n),
            None => c::scale_of(u@).is_none(),
        },
{
    proof {
        reveal_strlit("second");
        reveal_strlit("minute");
        reveal_strlit("hour");
        reveal_strlit("day");
        reveal_strlit("week");
        reveal_strlit("month");
        reveal_strlit("year");
        scale_cases(u@);
    }
    if ascii_eq(u, "second") {
        Some((false, 1))
    } else if ascii_eq(u, "minute") {
        Some((false, 60))
    } else if ascii_eq(u, "hour") {
        Some((false, 3600))
    } else if ascii_eq(u, "day") {
        Some((false, 86400))
    } else if ascii_eq(u, "week") {
        Some((false, 604800))
    } else if ascii_eq(u, "month") {
        Some((true, 1))
    } else if ascii_eq(u, "year") {
        Some((true, 12))
    } else {
        None
    }
}

// p/q as a signed decimal times the positive integer m: (magnitude, sign).
fn scaled_value(r: &crate::arith_impl::ERat, m: &Vec<u8>, mv: Ghost<nat>) -> (out: (Vec<u8>, bool))
    requires
        crate::arith_impl::rat_ok(r),
        m@ == udec_bytes(mv@),
        mv@ >= 1,
    ensures
        out.0@ == udec_bytes(ckc_spec::arith::abs(r.n@ * mv@)),
        out.1 == (r.n@ * mv@ < 0),
{
    let mag = crate::arith_mag::mag_mul(&r.num, Ghost(ckc_spec::arith::abs(r.n@)), m, mv);
    proof {
        assert(ckc_spec::arith::abs(r.n@ * mv@) == ckc_spec::arith::abs(r.n@) * mv@)
            by (nonlinear_arith)
            requires
                mv@ >= 1,
        ;
        assert((r.n@ * mv@ < 0) == (r.n@ < 0)) by (nonlinear_arith)
            requires
                mv@ >= 1,
        ;
    }
    (mag, r.neg)
}

fn holds4(arena: &ETermArena, args: &Vec<usize>) -> (r: bool)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, args@),
        args.len() == 4,
    ensures
        r == c::compare_holds(Term::Comp(c::satisfies_name(), root_terms(arena.nodes@, args@))),
{
    proof {
        assert(node_ok(arena.nodes@, args@[0] as int));
        assert(node_ok(arena.nodes@, args@[2] as int));
    }
    let x = crate::arith_impl::decode_number(arena, args[1]);
    let y = crate::arith_impl::decode_number(arena, args[3]);
    match (&arena.nodes[args[0]].kind, x, &arena.nodes[args[2]].kind, y) {
        (ENodeKind::Atom { name: s }, Some(p), ENodeKind::Atom { name: b }, Some(q)) => {
            // p.n/p.d against q.n/q.d compares p.n·q.d with q.n·p.d
            let (xm, xn) = scaled_value(&p, &q.den, q.d);
            let (ym, yn) = scaled_value(&q, &p.den, p.d);
            let order = int_order(&xm, xn, Ghost(p.n@ * q.d@), &ym, yn, Ghost(q.n@ * p.d@));
            within_order(s, b, &order, Ghost(p.n@ * q.d@), Ghost(q.n@ * p.d@))
        },
        _ => false,
    }
}

fn holds6(arena: &ETermArena, args: &Vec<usize>) -> (r: bool)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, args@),
        args.len() == 6,
    ensures
        r == c::compare_holds(Term::Comp(c::satisfies_name(), root_terms(arena.nodes@, args@))),
{
    proof {
        assert(node_ok(arena.nodes@, args@[0] as int));
        assert(node_ok(arena.nodes@, args@[2] as int));
        assert(node_ok(arena.nodes@, args@[3] as int));
        assert(node_ok(arena.nodes@, args@[5] as int));
        reveal_strlit("time");
        reveal_strlit("calendar");
    }
    let x = crate::arith_impl::decode_number(arena, args[1]);
    let y = crate::arith_impl::decode_number(arena, args[4]);
    match (
        &arena.nodes[args[0]].kind,
        x,
        &arena.nodes[args[2]].kind,
        &arena.nodes[args[3]].kind,
        y,
        &arena.nodes[args[5]].kind,
    ) {
        (
            ENodeKind::Atom { name: s },
            Some(p),
            ENodeKind::Atom { name: u },
            ENodeKind::Atom { name: b },
            Some(q),
            ENodeKind::Atom { name: v },
        ) => {
            match (unit_scale(u), unit_scale(v)) {
                (Some((ag, ar)), Some((bg, br))) => {
                    if ag != bg {
                        proof {
                            assert(ascii("time"@).len() == 4);
                            assert(ascii("calendar"@).len() == 8);
                            assert(scale_group(ag) != scale_group(bg));
                        }
                        return false;
                    }
                    // p.n·ar·q.d against q.n·br·p.d

                    let ghost xv = p.n@ * ar as int * q.d@;
                    let ghost yv = q.n@ * br as int * p.d@;
                    let xs = crate::arith_mag::mag_mul(
                        &q.den,
                        q.d,
                        &crate::k2_manifest::udec_vec(ar),
                        Ghost(ar as nat),
                    );
                    let ys = crate::arith_mag::mag_mul(
                        &p.den,
                        p.d,
                        &crate::k2_manifest::udec_vec(br),
                        Ghost(br as nat),
                    );
                    proof {
                        assert(q.d@ * (ar as nat) >= 1) by (nonlinear_arith)
                            requires
                                q.d@ >= 1,
                                ar >= 1,
                        ;
                        assert(p.d@ * (br as nat) >= 1) by (nonlinear_arith)
                            requires
                                p.d@ >= 1,
                                br >= 1,
                        ;
                    }
                    let (xm, xn) = scaled_value(&p, &xs, Ghost(q.d@ * ar as nat));
                    let (ym, yn) = scaled_value(&q, &ys, Ghost(p.d@ * br as nat));
                    proof {
                        assert(p.n@ * (q.d@ * ar as nat) == xv) by (nonlinear_arith)
                            requires
                                xv == p.n@ * ar as int * q.d@,
                        ;
                        assert(q.n@ * (p.d@ * br as nat) == yv) by (nonlinear_arith)
                            requires
                                yv == q.n@ * br as int * p.d@,
                        ;
                    }
                    let order = int_order(&xm, xn, Ghost(xv), &ym, yn, Ghost(yv));
                    within_order(s, b, &order, Ghost(xv), Ghost(yv))
                },
                _ => false,
            }
        },
        _ => false,
    }
}

pub fn is_arith_exec(name: &Vec<u8>, arity: usize) -> (r: bool)
    ensures
        r == ckc_spec::arith::is_arith(name@, arity as nat),
{
    proof {
        reveal_strlit("guideline_product");
        reveal_strlit("guideline_quotient");
    }
    arity == 6 && (ascii_eq(name, "guideline_product") || ascii_eq(name, "guideline_quotient"))
}

// A comparison goal holds (an arithmetic goal goes through
// `arith_impl::arith_result_exec` and unification instead).
pub fn holds_exec(arena: &ETermArena, name: &Vec<u8>, args: &Vec<usize>) -> (r: bool)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, args@),
        !ckc_spec::arith::is_arith(name@, args@.len() as nat),
    ensures
        r == ckc_spec::compare::holds(
            ckc_spec::term::Term::Comp(name@, root_terms(arena.nodes@, args@)),
        ),
{
    proof {
        reveal(ckc_spec::arith::arith_holds);
        reveal(ckc_spec::arith::arith_result);
    }
    if !is_builtin_exec(name, args.len()) {
        return false;
    }
    if args.len() == 4 {
        holds4(arena, args)
    } else {
        holds6(arena, args)
    }
}

} // verus!
