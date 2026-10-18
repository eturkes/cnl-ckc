use crate::compare_decimal::scaled_order;
use crate::compare_schema::{ascii_eq, schema_bytes};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid};
use crate::k2_term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::compare as c;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::ascii;
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
    (arity == 4 || arity == 6) && ascii_eq(name, "guideline_satisfies")
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

fn holds4(arena: &ETermArena, args: &Vec<usize>) -> (r: bool)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, args@),
        args.len() == 4,
    ensures
        r == c::holds(Term::Comp(c::satisfies_name(), root_terms(arena.nodes@, args@))),
{
    proof {
        assert(node_ok(arena.nodes@, args@[0] as int));
        assert(node_ok(arena.nodes@, args@[1] as int));
        assert(node_ok(arena.nodes@, args@[2] as int));
        assert(node_ok(arena.nodes@, args@[3] as int));
    }
    match (
        &arena.nodes[args[0]].kind,
        &arena.nodes[args[1]].kind,
        &arena.nodes[args[2]].kind,
        &arena.nodes[args[3]].kind,
    ) {
        (
            ENodeKind::Atom { name: s },
            ENodeKind::Int { magnitude: am, negative: an, value: a, .. },
            ENodeKind::Atom { name: b },
            ENodeKind::Int { magnitude: bm, negative: bn, value: y, .. },
        ) => {
            let order = int_order(am, *an, Ghost(a@), bm, *bn, Ghost(y@));
            within_order(s, b, &order, Ghost(a@), Ghost(y@))
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
        r == c::holds(Term::Comp(c::satisfies_name(), root_terms(arena.nodes@, args@))),
{
    proof {
        assert(node_ok(arena.nodes@, args@[0] as int));
        assert(node_ok(arena.nodes@, args@[1] as int));
        assert(node_ok(arena.nodes@, args@[2] as int));
        assert(node_ok(arena.nodes@, args@[3] as int));
        assert(node_ok(arena.nodes@, args@[4] as int));
        assert(node_ok(arena.nodes@, args@[5] as int));
        reveal_strlit("time");
        reveal_strlit("calendar");
    }
    match (
        &arena.nodes[args[0]].kind,
        &arena.nodes[args[1]].kind,
        &arena.nodes[args[2]].kind,
        &arena.nodes[args[3]].kind,
        &arena.nodes[args[4]].kind,
        &arena.nodes[args[5]].kind,
    ) {
        (
            ENodeKind::Atom { name: s },
            ENodeKind::Int { magnitude: am, negative: an, value: a, .. },
            ENodeKind::Atom { name: u },
            ENodeKind::Atom { name: b },
            ENodeKind::Int { magnitude: bm, negative: bn, value: y, .. },
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
                    let order = scaled_order(am, *an, Ghost(a@), bm, *bn, Ghost(y@), ar, br);
                    within_order(s, b, &order, Ghost(a@ * ar), Ghost(y@ * br))
                },
                _ => false,
            }
        },
        _ => false,
    }
}

pub fn holds_exec(arena: &ETermArena, name: &Vec<u8>, args: &Vec<usize>) -> (r: bool)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, args@),
    ensures
        r == ckc_spec::compare::holds(
            ckc_spec::term::Term::Comp(name@, root_terms(arena.nodes@, args@)),
        ),
{
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
