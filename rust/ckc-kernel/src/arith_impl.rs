use crate::arith_mag::*;
use crate::compare_schema::{ascii_eq, literal};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid};
use crate::k2_term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::arith as a;
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{ascii, dec_bytes, udec_bytes};
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::{lemma_fundamental_div_mod, lemma_fundamental_div_mod_converse};
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

// Exact rationals + value sets over arena terms (contract da1 D1–D5): the
// exec mirror of `ckc_spec::arith`.
pub struct ERat {
    pub neg: bool,
    pub num: Vec<u8>,
    pub den: Vec<u8>,
    pub n: Ghost<int>,
    pub d: Ghost<nat>,
}

pub open spec fn rat_ok(r: &ERat) -> bool {
    &&& r.neg == (r.n@ < 0)
    &&& r.num@ == udec_bytes(a::abs(r.n@))
    &&& r.den@ == udec_bytes(r.d@)
    &&& r.d@ >= 1
}

pub enum EVSet {
    Point(ERat),
    Lower(ERat, bool),
    Upper(ERat, bool),
}

pub open spec fn vs_ok(v: &EVSet) -> bool {
    match v {
        EVSet::Point(r) => rat_ok(r),
        EVSet::Lower(r, _) => rat_ok(r),
        EVSet::Upper(r, _) => rat_ok(r),
    }
}

pub open spec fn vs_view(v: &EVSet) -> a::VSet {
    match v {
        EVSet::Point(r) => a::VSet::Point(r.n@, r.d@ as int),
        EVSet::Lower(r, c) => a::VSet::Lower(r.n@, r.d@ as int, *c),
        EVSet::Upper(r, c) => a::VSet::Upper(r.n@, r.d@ as int, *c),
    }
}

// --- number theory for canonical spellings ---
pub proof fn gcd_pos(x: nat, y: nat)
    requires
        y >= 1,
    ensures
        a::gcd(x, y) >= 1,
    decreases y,
{
    reveal_with_fuel(a::gcd, 2);
    if x % y == 0 {
        assert(a::gcd(y, 0) == y);
    } else {
        gcd_pos(y, x % y);
    }
}

pub proof fn gcd_divides(x: nat, y: nat)
    requires
        x >= 1 || y >= 1,
    ensures
        a::gcd(x, y) >= 1,
        x % a::gcd(x, y) == 0,
        y % a::gcd(x, y) == 0,
    decreases y,
{
    reveal_with_fuel(a::gcd, 2);
    if y == 0 {
        assert(x >= 1);
        assert(x % x == 0) by {
            lemma_fundamental_div_mod_converse(x as int, x as int, 1, 0);
        }
        assert(0nat % x == 0) by {
            lemma_fundamental_div_mod_converse(0, x as int, 0, 0);
        }
    } else {
        let r = x % y;
        gcd_divides(y, r);
        let g = a::gcd(y, r);
        assert(a::gcd(x, y) == g);
        lemma_fundamental_div_mod(x as int, y as int);
        lemma_fundamental_div_mod(y as int, g as int);
        lemma_fundamental_div_mod(r as int, g as int);
        let k = (y / g) as int;
        let m = (r / g) as int;
        let t = (x / y) as int;
        let xi = x as int;
        let yi = y as int;
        let gi = g as int;
        let ri = r as int;
        assert(xi == (t * k + m) * gi) by (nonlinear_arith)
            requires
                xi == yi * t + ri,
                yi == gi * k,
                ri == gi * m,
        ;
        lemma_fundamental_div_mod_converse(xi, gi, t * k + m, 0);
    }
}

// p / g in the spec's integer division, for g dividing |p|.
pub proof fn exact_div(p: int, g: int)
    requires
        g >= 1,
        a::abs(p) % (g as nat) == 0,
    ensures
        p / g == if p < 0 {
            -((a::abs(p) / (g as nat)) as int)
        } else {
            (a::abs(p) / (g as nat)) as int
        },
        a::abs(p / g) == a::abs(p) / (g as nat),
        (p / g < 0) == (p < 0 && a::abs(p) / (g as nat) > 0),
{
    let m = a::abs(p);
    let q = m / (g as nat);
    lemma_fundamental_div_mod(m as int, g);
    assert(m == g * q);
    if p < 0 {
        assert(p == g * (-q)) by (nonlinear_arith)
            requires
                m == g * q,
                p == -(m as int),
        ;
        lemma_fundamental_div_mod_converse(p, g, -q, 0);
    } else {
        lemma_fundamental_div_mod_converse(p, g, q as int, 0);
    }
}

proof fn abs_mul(x: int, y: int)
    ensures
        a::abs(x * y) == a::abs(x) * a::abs(y),
        (x * y < 0) == (x * y != 0 && ((x < 0) != (y < 0))),
        (x * y == 0) == (x == 0 || y == 0),
{
    assert(a::abs(x * y) == a::abs(x) * a::abs(y)) by (nonlinear_arith);
    assert((x * y < 0) == (x * y != 0 && ((x < 0) != (y < 0)))) by (nonlinear_arith);
    assert((x * y == 0) == (x == 0 || y == 0)) by (nonlinear_arith);
}

// --- decoding ---
pub fn decode_number(arena: &ETermArena, root: usize) -> (out: Option<ERat>)
    requires
        root_ok(arena, root),
    ensures
        match out {
            Option::Some(r) => rat_ok(&r) && a::number(arena@[root as int]) == Option::Some(
                (r.n@, r.d@ as int),
            ),
            Option::None => a::number(arena@[root as int]) is None,
        },
{
    proof {
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok);
        reveal_with_fuel(udec_bytes, 2);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Int { magnitude, negative, value, .. } => {
            let den = one();
            Option::Some(
                ERat { neg: *negative, num: magnitude.clone(), den, n: Ghost(value@), d: Ghost(1) },
            )
        },
        ENodeKind::Comp { name, child_roots, .. } => {
            proof {
                reveal_strlit("rational");
            }
            if child_roots.len() != 2 || !ascii_eq(name, "rational") {
                return Option::None;
            }
            let pr = child_roots[0];
            let qr = child_roots[1];
            proof {
                crate::k2_engine::node_comp_model(arena.nodes@, root as int, name@, child_roots@);
                reveal(roots_valid);
                assert(node_ok(arena.nodes@, pr as int));
                assert(node_ok(arena.nodes@, qr as int));
                reveal(child_terms);
                assert(child_terms(arena.nodes@, child_roots@) =~= seq![
                    arena@[pr as int],
                    arena@[qr as int],
                ]);
                assert(arena@[root as int] == Term::Comp(
                    a::rational_name(),
                    seq![arena@[pr as int], arena@[qr as int]],
                ));
            }
            match (&arena.nodes[pr].kind, &arena.nodes[qr].kind) {
                (
                    ENodeKind::Int { magnitude: pm, negative: pn, value: pv, .. },
                    ENodeKind::Int { magnitude: qm, negative: qn, value: qv, .. },
                ) => {
                    if *qn {
                        return Option::None;
                    }
                    let unit = one();
                    let order = int_order(qm, false, Ghost(qv@), &unit, false, Ghost(1int));
                    match order {
                        EOrder::Greater => {},
                        _ => {
                            proof {
                                reveal(int_order_ok);
                            }
                            return Option::None;
                        },
                    }
                    proof {
                        reveal(int_order_ok);
                    }
                    let g = mag_gcd(pm, Ghost(a::abs(pv@)), qm, Ghost(qv@ as nat));
                    if !is_one(&g, Ghost(a::gcd(a::abs(pv@), qv@ as nat))) {
                        return Option::None;
                    }
                    Option::Some(
                        ERat {
                            neg: *pn,
                            num: pm.clone(),
                            den: qm.clone(),
                            n: Ghost(pv@),
                            d: Ghost(qv@ as nat),
                        },
                    )
                },
                _ => Option::None,
            }
        },
        _ => Option::None,
    }
}

pub(crate) proof fn keywords_distinct()
    ensures
        ascii("exactly"@) != ascii("eq"@),
        ascii("exactly"@) != ascii("geq"@),
        ascii("exactly"@) != ascii("greater"@),
        ascii("exactly"@) != ascii("leq"@),
        ascii("exactly"@) != ascii("less"@),
        ascii("eq"@) != ascii("greater"@),
        ascii("eq"@) != ascii("leq"@),
        ascii("eq"@) != ascii("less"@),
        ascii("geq"@) != ascii("greater"@),
        ascii("geq"@) != ascii("leq"@),
        ascii("geq"@) != ascii("less"@),
        ascii("greater"@) != ascii("leq"@),
        ascii("greater"@) != ascii("less"@),
        ascii("leq"@) != ascii("less"@),
{
    reveal_strlit("exactly");
    reveal_strlit("eq");
    reveal_strlit("geq");
    reveal_strlit("greater");
    reveal_strlit("leq");
    reveal_strlit("less");
    assert(ascii("exactly"@).len() == 7);
    assert(ascii("eq"@).len() == 2);
    assert(ascii("geq"@).len() == 3);
    assert(ascii("greater"@).len() == 7);
    assert(ascii("leq"@).len() == 3);
    assert(ascii("less"@).len() == 4);
    assert(ascii("exactly"@)[0] != ascii("greater"@)[0]);
    assert(ascii("geq"@)[0] != ascii("leq"@)[0]);
}

// The atom of a comparison: 1 = exactly, 2 = eq | geq, 3 = greater, 4 = leq,
// 5 = less, 0 = another term.
fn comparison_code(arena: &ETermArena, root: usize) -> (k: u8)
    requires
        root_ok(arena, root),
    ensures
        k <= 5,
        (k == 1) == (arena@[root as int] == a::kw("exactly"@)),
        (k == 2) == (arena@[root as int] == a::kw("eq"@) || arena@[root as int] == a::kw("geq"@)),
        (k == 3) == (arena@[root as int] == a::kw("greater"@)),
        (k == 4) == (arena@[root as int] == a::kw("leq"@)),
        (k == 5) == (arena@[root as int] == a::kw("less"@)),
{
    proof {
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, root as int));
        reveal(node_ok);
        keywords_distinct();
        reveal_strlit("exactly");
        reveal_strlit("eq");
        reveal_strlit("geq");
        reveal_strlit("greater");
        reveal_strlit("leq");
        reveal_strlit("less");
    }
    match &arena.nodes[root].kind {
        ENodeKind::Atom { name } => {
            if ascii_eq(name, "exactly") {
                1
            } else if ascii_eq(name, "eq") || ascii_eq(name, "geq") {
                2
            } else if ascii_eq(name, "greater") {
                3
            } else if ascii_eq(name, "leq") {
                4
            } else if ascii_eq(name, "less") {
                5
            } else {
                0
            }
        },
        _ => 0,
    }
}

pub fn decode_vset(arena: &ETermArena, s: usize, n: usize) -> (out: Option<EVSet>)
    requires
        root_ok(arena, s),
        root_ok(arena, n),
    ensures
        match out {
            Option::Some(v) => vs_ok(&v) && a::vset(arena@[s as int], arena@[n as int])
                == Option::Some(vs_view(&v)),
            Option::None => a::vset(arena@[s as int], arena@[n as int]) is None,
        },
{
    let k = comparison_code(arena, s);
    match decode_number(arena, n) {
        Option::None => Option::None,
        Option::Some(r) => {
            if k == 1 {
                Option::Some(EVSet::Point(r))
            } else if k == 2 {
                Option::Some(EVSet::Lower(r, true))
            } else if k == 3 {
                Option::Some(EVSet::Lower(r, false))
            } else if k == 4 {
                Option::Some(EVSet::Upper(r, true))
            } else if k == 5 {
                Option::Some(EVSet::Upper(r, false))
            } else {
                Option::None
            }
        },
    }
}

// --- arithmetic ---
fn rat_zero() -> (r: ERat)
    ensures
        rat_ok(&r),
        r.n@ == 0,
        r.d@ == 1,
{
    ERat { neg: false, num: zero(), den: one(), n: Ghost(0), d: Ghost(1) }
}

fn rat_is_zero(r: &ERat) -> (z: bool)
    requires
        rat_ok(r),
    ensures
        z == (r.n@ == 0),
{
    is_zero(&r.num, Ghost(a::abs(r.n@)))
}

pub fn rat_mul(x: &ERat, y: &ERat) -> (r: ERat)
    requires
        rat_ok(x),
        rat_ok(y),
    ensures
        rat_ok(&r),
        r.n@ == x.n@ * y.n@,
        r.d@ == x.d@ * y.d@,
{
    let num = mag_mul(&x.num, Ghost(a::abs(x.n@)), &y.num, Ghost(a::abs(y.n@)));
    let den = mag_mul(&x.den, x.d, &y.den, y.d);
    proof {
        abs_mul(x.n@, y.n@);
        assert(x.d@ * y.d@ >= 1) by (nonlinear_arith)
            requires
                x.d@ >= 1,
                y.d@ >= 1,
        ;
    }
    let z = is_zero(&num, Ghost(a::abs(x.n@) * a::abs(y.n@)));
    let neg = !z && (x.neg != y.neg);
    ERat { neg, num, den, n: Ghost(x.n@ * y.n@), d: Ghost(x.d@ * y.d@) }
}

fn att_exec(x: &ERat, c1: bool, y: &ERat, c2: bool) -> (r: bool)
    requires
        rat_ok(x),
        rat_ok(y),
    ensures
        r == a::att(x.n@, c1, y.n@, c2),
{
    (rat_is_zero(x) && c1) || (rat_is_zero(y) && c2) || (c1 && c2)
}

// The nonzero point x times v.
fn stretch_exec(x: &ERat, v: &EVSet) -> (out: EVSet)
    requires
        rat_ok(x),
        x.n@ != 0,
        vs_ok(v),
    ensures
        vs_ok(&out),
        vs_view(&out) == a::stretch(x.n@, x.d@ as int, vs_view(v)),
{
    proof {
        assert((x.d@ * 1) as int == x.d@ as int);
    }
    match v {
        EVSet::Point(y) => {
            let p = rat_mul(x, y);
            proof {
                assert((x.d@ * y.d@) as int == x.d@ as int * y.d@ as int);
            }
            EVSet::Point(p)
        },
        EVSet::Lower(y, c) => {
            let p = rat_mul(x, y);
            proof {
                assert((x.d@ * y.d@) as int == x.d@ as int * y.d@ as int);
            }
            if !x.neg {
                EVSet::Lower(p, *c)
            } else {
                EVSet::Upper(p, *c)
            }
        },
        EVSet::Upper(y, c) => {
            let p = rat_mul(x, y);
            proof {
                assert((x.d@ * y.d@) as int == x.d@ as int * y.d@ as int);
            }
            if !x.neg {
                EVSet::Upper(p, *c)
            } else {
                EVSet::Lower(p, *c)
            }
        },
    }
}

fn is_zero_point(v: &EVSet) -> (z: bool)
    requires
        vs_ok(v),
    ensures
        z == a::zero_point(vs_view(v)),
{
    match v {
        EVSet::Point(r) => rat_is_zero(r),
        _ => false,
    }
}

pub fn vprod_exec(x: &EVSet, y: &EVSet) -> (out: Option<EVSet>)
    requires
        vs_ok(x),
        vs_ok(y),
    ensures
        match out {
            Option::Some(v) => vs_ok(&v) && a::vprod(vs_view(x), vs_view(y)) == Option::Some(
                vs_view(&v),
            ),
            Option::None => a::vprod(vs_view(x), vs_view(y)) is None,
        },
{
    if is_zero_point(x) || is_zero_point(y) {
        return Option::Some(EVSet::Point(rat_zero()));
    }
    match (x, y) {
        (EVSet::Point(p), _) => Option::Some(stretch_exec(p, y)),
        (_, EVSet::Point(q)) => Option::Some(stretch_exec(q, x)),
        (EVSet::Lower(p, c1), EVSet::Lower(q, c2)) => {
            if !p.neg && !q.neg {
                let m = rat_mul(p, q);
                let k = att_exec(p, *c1, q, *c2);
                proof {
                    assert((p.d@ * q.d@) as int == p.d@ as int * q.d@ as int);
                }
                Option::Some(EVSet::Lower(m, k))
            } else {
                Option::None
            }
        },
        (EVSet::Upper(p, c1), EVSet::Upper(q, c2)) => {
            if (p.neg || rat_is_zero(p)) && (q.neg || rat_is_zero(q)) {
                let m = rat_mul(p, q);
                let k = att_exec(p, *c1, q, *c2);
                proof {
                    assert((p.d@ * q.d@) as int == p.d@ as int * q.d@ as int);
                }
                Option::Some(EVSet::Lower(m, k))
            } else {
                Option::None
            }
        },
        (EVSet::Lower(p, c1), EVSet::Upper(q, c2)) => {
            if !p.neg && (q.neg || rat_is_zero(q)) {
                let m = rat_mul(p, q);
                let k = att_exec(p, *c1, q, *c2);
                proof {
                    assert((p.d@ * q.d@) as int == p.d@ as int * q.d@ as int);
                }
                Option::Some(EVSet::Upper(m, k))
            } else {
                Option::None
            }
        },
        (EVSet::Upper(p, c1), EVSet::Lower(q, c2)) => {
            if (p.neg || rat_is_zero(p)) && !q.neg {
                let m = rat_mul(p, q);
                let k = att_exec(p, *c1, q, *c2);
                proof {
                    assert((p.d@ * q.d@) as int == p.d@ as int * q.d@ as int);
                }
                Option::Some(EVSet::Upper(m, k))
            } else {
                Option::None
            }
        },
    }
}

pub fn vquot_exec(x: &EVSet, y: &EVSet) -> (out: Option<EVSet>)
    requires
        vs_ok(x),
        vs_ok(y),
    ensures
        match out {
            Option::Some(v) => vs_ok(&v) && a::vquot(vs_view(x), vs_view(y)) == Option::Some(
                vs_view(&v),
            ),
            Option::None => a::vquot(vs_view(x), vs_view(y)) is None,
        },
{
    match y {
        EVSet::Point(r) => {
            if rat_is_zero(r) {
                Option::None
            } else if !r.neg {
                // 1/(n/d) = d/n
                let inv = ERat {
                    neg: false,
                    num: r.den.clone(),
                    den: r.num.clone(),
                    n: Ghost(r.d@ as int),
                    d: Ghost(a::abs(r.n@)),
                };
                vprod_exec(x, &EVSet::Point(inv))
            } else {
                // 1/(n/d) = (−d)/(−n)
                let inv = ERat {
                    neg: true,
                    num: r.den.clone(),
                    den: r.num.clone(),
                    n: Ghost(-(r.d@ as int)),
                    d: Ghost(a::abs(r.n@)),
                };
                vprod_exec(x, &EVSet::Point(inv))
            }
        },
        _ => Option::None,
    }
}

// --- canonical spellings into the arena ---
fn signed_spelling(neg: bool, magnitude: &Vec<u8>) -> (out: Vec<u8>)
    ensures
        out@ == if neg {
            seq![0x2Du8] + magnitude@
        } else {
            magnitude@
        },
{
    if neg {
        let mut out: Vec<u8> = Vec::new();
        out.push(0x2D);
        let mut i = 0usize;
        while i < magnitude.len()
            invariant
                i <= magnitude@.len(),
                out@ == seq![0x2Du8] + magnitude@.take(i as int),
            decreases magnitude.len() - i,
        {
            out.push(magnitude[i]);
            proof {
                assert_seqs_equal!(magnitude@.take(i as int + 1) == magnitude@.take(i as int).push(magnitude@[i as int]));
            }
            i += 1;
        }
        proof {
            assert_seqs_equal!(magnitude@.take(magnitude@.len() as int) == magnitude@);
        }
        out
    } else {
        magnitude.clone()
    }
}

fn push_value(arena: &mut ETermArena, neg: bool, magnitude: Vec<u8>, value: Ghost<int>) -> (root:
    usize)
    requires
        arena_ok(old(arena)),
        neg == (value@ < 0),
        magnitude@ == udec_bytes(a::abs(value@)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root < final(arena).nodes@.len(),
        final(arena)@[root as int] == Term::Int(value@),
{
    let spelling = signed_spelling(neg, &magnitude);
    proof {
        reveal(dec_bytes);
    }
    let root = push_int(arena, spelling, magnitude, neg, value);
    proof {
        assert(old(arena).nodes@.is_prefix_of(final(arena).nodes@));
    }
    root
}

pub fn canon_push(arena: &mut ETermArena, r: &ERat) -> (root: usize)
    requires
        arena_ok(old(arena)),
        rat_ok(r),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root < final(arena).nodes@.len(),
        final(arena)@[root as int] == a::canon(r.n@, r.d@ as int),
{
    let ghost m = a::abs(r.n@);
    let ghost gv = a::gcd(m, r.d@);
    proof {
        gcd_divides(m, r.d@);
    }
    let g = mag_gcd(&r.num, Ghost(m), &r.den, r.d);
    let (pq, _) = mag_divmod(&r.num, Ghost(m), &g, Ghost(gv));
    let (dq, _) = mag_divmod(&r.den, r.d, &g, Ghost(gv));
    proof {
        exact_div(r.n@, gv as int);
        assert((r.d@ as int) / (gv as int) == (r.d@ / gv) as int);
    }
    let pz = is_zero(&pq, Ghost(m / gv));
    let neg = r.neg && !pz;
    let ghost pval = r.n@ / (gv as int);
    proof {
        assert(a::canon(r.n@, r.d@ as int) == if r.d@ / gv == 1 {
            Term::Int(pval)
        } else {
            Term::Comp(a::rational_name(), seq![Term::Int(pval), Term::Int((r.d@ / gv) as int)])
        });
    }
    let p_root = push_value(arena, neg, pq, Ghost(pval));
    if is_one(&dq, Ghost(r.d@ / gv)) {
        p_root
    } else {
        let ghost mid = arena.nodes@;
        let q_root = push_value(arena, false, dq, Ghost((r.d@ / gv) as int));
        proof {
            reveal_strlit("rational");
        }
        let name = literal("rational");
        let mut kids: Vec<usize> = Vec::new();
        kids.push(p_root);
        kids.push(q_root);
        let ghost before = arena.nodes@;
        proof {
            arena_prefix_stable(mid, arena);
            assert(arena@[p_root as int] == Term::Int(pval));
            reveal(child_terms);
            assert(child_terms(before, kids@) =~= seq![
                Term::Int(pval),
                Term::Int((r.d@ / gv) as int),
            ]);
        }
        let root = push_comp(arena, name, kids);
        root
    }
}

pub fn spell_push(arena: &mut ETermArena, v: &EVSet) -> (out: (usize, usize))
    requires
        arena_ok(old(arena)),
        vs_ok(v),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out.0 < final(arena).nodes@.len(),
        out.1 < final(arena).nodes@.len(),
        (final(arena)@[out.0 as int], final(arena)@[out.1 as int]) == a::spell(vs_view(v)),
{
    proof {
        reveal_strlit("exactly");
        reveal_strlit("geq");
        reveal_strlit("greater");
        reveal_strlit("leq");
        reveal_strlit("less");
    }
    let (word, r) = match v {
        EVSet::Point(r) => (literal("exactly"), r),
        EVSet::Lower(r, c) => (
            if *c {
                literal("geq")
            } else {
                literal("greater")
            },
            r,
        ),
        EVSet::Upper(r, c) => (
            if *c {
                literal("leq")
            } else {
                literal("less")
            },
            r,
        ),
    };
    let ghost w = word@;
    let s = push_atom(arena, word);
    let ghost mid = arena.nodes@;
    let n = canon_push(arena, r);
    proof {
        arena_prefix_stable(mid, arena);
    }
    (s, n)
}

// The output pair an arithmetic goal computes (contract da1 D5), pushed into
// the arena; None = the spec's None.
pub fn arith_result_exec(arena: &mut ETermArena, name: &Vec<u8>, args: &Vec<usize>) -> (out: Option<
    (usize, usize),
>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, args@),
        args@.len() == 6,
        a::is_arith(name@, 6),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match out {
            Option::Some((s, n)) => s < final(arena).nodes@.len() && n < final(arena).nodes@.len()
                && a::arith_result(Term::Comp(name@, root_terms(old(arena).nodes@, args@)))
                == Option::Some((final(arena)@[s as int], final(arena)@[n as int])),
            Option::None => a::arith_result(
                Term::Comp(name@, root_terms(old(arena).nodes@, args@)),
            ) is None,
        },
{
    let ghost terms = root_terms(arena.nodes@, args@);
    proof {
        reveal_strlit("guideline_product");
        assert(terms.len() == 6);
    }
    let x = decode_vset(arena, args[0], args[1]);
    let y = decode_vset(arena, args[2], args[3]);
    let r = match (x, y) {
        (Option::Some(p), Option::Some(q)) => {
            if ascii_eq(name, "guideline_product") {
                vprod_exec(&p, &q)
            } else {
                vquot_exec(&p, &q)
            }
        },
        _ => Option::None,
    };
    match r {
        Option::Some(v) => {
            let out = spell_push(arena, &v);
            Option::Some(out)
        },
        Option::None => Option::None,
    }
}

} // verus!
