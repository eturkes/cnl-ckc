#[cfg(verus_keep_ghost)]
use ckc_spec::arith as a;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::{occurs, occurs_all, subst};
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::arg;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::{Term, ground, ground_all};
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

// Contract ws D6: a chain-defined admitted tuple ⇒ the default tuple's chain is
// defined. Every chain input is an exact point; a node fails only at a zero
// divisor; a zero of the default forces the same zero in every admitted tuple.
pub open spec fn zero_num(x: Term) -> bool {
    a::number(x) matches Some((p, _)) && p == 0
}

// Two pair values the chain reads alike: both `exactly`, or numbers where the
// default's zero forces the tuple's.
pub open spec fn rel_val(vt: Term, vd: Term) -> bool {
    ||| vt == ws::exactly() && vd == ws::exactly()
    ||| a::number(vt) is Some && a::number(vd) is Some && (zero_num(vd) ==> zero_num(vt))
}

pub open spec fn rel_pairs(pt: Seq<(Term, Term)>, pd: Seq<(Term, Term)>) -> bool {
    &&& pt.len() == pd.len()
    &&& forall|i: int|
        0 <= i < pt.len() ==> (#[trigger] pt[i]).0 == pd[i].0 && pt[i].0 is Var && rel_val(
            pt[i].1,
            pd[i].1,
        )
}

pub open spec fn vals_ground(p: Seq<(Term, Term)>) -> bool {
    forall|i: int| 0 <= i < p.len() ==> ground((#[trigger] p[i]).1)
}

pub proof fn ground_absent(t: Term, x: nat)
    requires
        ground(t),
    ensures
        !occurs(x, t),
    decreases t,
{
    match t {
        Term::Comp(_, args) => ground_all_absent(args, x),
        _ => {},
    }
}

proof fn ground_all_absent(ts: Seq<Term>, x: nat)
    requires
        ground_all(ts),
    ensures
        !occurs_all(x, ts),
    decreases ts,
{
    if ts.len() > 0 {
        ground_absent(ts[0], x);
        ground_all_absent(ts.drop_first(), x);
    }
}

proof fn rel_ground(vt: Term, vd: Term)
    requires
        rel_val(vt, vd),
    ensures
        ground(vt),
        ground(vd),
{
    if a::number(vt) is Some {
        crate::k3_sound::log::number_ground(vt);
    }
    if a::number(vd) is Some {
        crate::k3_sound::log::number_ground(vd);
    }
}

pub proof fn bind_ground(t: Term, p: Seq<(Term, Term)>)
    requires
        ground(t),
    ensures
        spec::bind_all(t, p) == t,
    decreases p.len(),
{
    if p.len() > 0 {
        match p[0].0 {
            Term::Var(x) => {
                ground_absent(t, x);
                crate::k3_sound::unification::subst_absent(t, x, p[0].1);
            },
            _ => {},
        }
        bind_ground(t, p.drop_first());
    }
}

pub proof fn bind_var(x: nat, p: Seq<(Term, Term)>)
    requires
        vals_ground(p),
    ensures
        spec::bind_all(Term::Var(x), p) == match spec::lookup(p, Term::Var(x)) {
            Some(v) => v,
            None => Term::Var(x),
        },
    decreases p.len(),
{
    if p.len() > 0 {
        assert(vals_ground(p.drop_first())) by {
            assert forall|i: int| 0 <= i < p.drop_first().len() implies ground(
                (#[trigger] p.drop_first()[i]).1,
            ) by {
                assert(p.drop_first()[i] == p[i + 1]);
            }
        }
        match p[0].0 {
            Term::Var(y) => {
                if y == x {
                    assert(ground(p[0].1));
                    bind_ground(p[0].1, p.drop_first());
                } else {
                    bind_var(x, p.drop_first());
                }
            },
            _ => {
                bind_var(x, p.drop_first());
            },
        }
    }
}

pub proof fn bind_comp(name: Seq<u8>, args: Seq<Term>, p: Seq<(Term, Term)>)
    ensures
        spec::bind_all(Term::Comp(name, args), p) matches Term::Comp(n2, a2) && n2 == name
            && a2.len() == args.len() && forall|i: int|
            0 <= i < args.len() ==> #[trigger] a2[i] == spec::bind_all(args[i], p),
    decreases p.len(),
{
    if p.len() > 0 {
        match p[0].0 {
            Term::Var(x) => {
                let a1 = ckc_spec::engine::subst_all(args, x, p[0].1);
                crate::k3_sound::naf::subst_all_index(args, x, p[0].1);
                assert(subst(Term::Comp(name, args), x, p[0].1) == Term::Comp(name, a1));
                bind_comp(name, a1, p.drop_first());
                let r = spec::bind_all(Term::Comp(name, a1), p.drop_first());
                assert forall|i: int| 0 <= i < args.len() implies #[trigger] r->Comp_1[i]
                    == spec::bind_all(args[i], p) by {
                    assert(a1[i] == subst(args[i], x, p[0].1));
                }
            },
            _ => {
                bind_comp(name, args, p.drop_first());
            },
        }
    }
}

pub proof fn lookup_rel(pt: Seq<(Term, Term)>, pd: Seq<(Term, Term)>, k: Term)
    requires
        rel_pairs(pt, pd),
    ensures
        match (spec::lookup(pt, k), spec::lookup(pd, k)) {
            (Some(vt), Some(vd)) => rel_val(vt, vd),
            (None, None) => true,
            _ => false,
        },
    decreases pt.len(),
{
    if pt.len() > 0 {
        assert(pt[0].0 == pd[0].0);
        if pt[0].0 != k {
            assert(rel_pairs(pt.drop_first(), pd.drop_first())) by {
                assert forall|i: int| 0 <= i < pt.drop_first().len() implies (
                #[trigger] pt.drop_first()[i]).0 == pd.drop_first()[i].0
                    && pt.drop_first()[i].0 is Var && rel_val(
                    pt.drop_first()[i].1,
                    pd.drop_first()[i].1,
                ) by {
                    assert(pt.drop_first()[i] == pt[i + 1]);
                    assert(pd.drop_first()[i] == pd[i + 1]);
                }
            }
            lookup_rel(pt.drop_first(), pd.drop_first(), k);
        }
    }
}

pub proof fn rel_vals_ground(pt: Seq<(Term, Term)>, pd: Seq<(Term, Term)>)
    requires
        rel_pairs(pt, pd),
    ensures
        vals_ground(pt),
        vals_ground(pd),
{
    assert forall|i: int| 0 <= i < pt.len() implies ground((#[trigger] pt[i]).1) by {
        rel_ground(pt[i].1, pd[i].1);
    }
    assert forall|i: int| 0 <= i < pd.len() implies ground((#[trigger] pd[i]).1) by {
        assert(pt[i].0 == pd[i].0);
        rel_ground(pt[i].1, pd[i].1);
    }
}

pub proof fn lookup_at(p: Seq<(Term, Term)>, idx: int)
    requires
        0 <= idx < p.len(),
        forall|j: int| 0 <= j < idx ==> (#[trigger] p[j]).0 != p[idx].0,
    ensures
        spec::lookup(p, p[idx].0) == Some(p[idx].1),
    decreases p.len(),
{
    if idx > 0 {
        assert(p[0].0 != p[idx].0);
        assert forall|j: int| 0 <= j < idx - 1 implies (#[trigger] p.drop_first()[j]).0
            != p.drop_first()[idx - 1].0 by {
            assert(p.drop_first()[j] == p[j + 1]);
            assert(p.drop_first()[idx - 1] == p[idx]);
        }
        lookup_at(p.drop_first(), idx - 1);
        assert(p.drop_first()[idx - 1] == p[idx]);
    }
}

pub proof fn lookup_absent(p: Seq<(Term, Term)>, k: Term)
    requires
        forall|j: int| 0 <= j < p.len() ==> (#[trigger] p[j]).0 != k,
    ensures
        spec::lookup(p, k) is None,
    decreases p.len(),
{
    if p.len() > 0 {
        assert forall|j: int| 0 <= j < p.drop_first().len() implies (#[trigger] p.drop_first()[j]).0
            != k by {
            assert(p.drop_first()[j] == p[j + 1]);
        }
        lookup_absent(p.drop_first(), k);
    }
}

// --- the arithmetic of exact points ---
proof fn canon_zero(p: int, q: int)
    requires
        q >= 1,
    ensures
        a::number(a::canon(p, q)) is Some,
        zero_num(a::canon(p, q)) == (p == 0),
{
    crate::arith_sound::canon_number(p, q);
    let (n, d) = a::number(a::canon(p, q))->Some_0;
    if n == 0 {
        assert(p * d == 0) by (nonlinear_arith)
            requires
                n * q == p * d,
                n == 0,
        ;
        assert(p == 0) by (nonlinear_arith)
            requires
                p * d == 0,
                d >= 1,
        ;
    } else {
        assert(p != 0) by (nonlinear_arith)
            requires
                n * q == p * d,
                n != 0,
                q >= 1,
        ;
    }
}

proof fn number_den(x: Term)
    requires
        a::number(x) is Some,
    ensures
        a::number(x)->Some_0.1 >= 1,
{
}

// One chain node over exact points: the tuple's node derives ⇒ the default's
// derives, both spell `exactly` with related counts.
pub proof fn node_points(
    name: Seq<u8>,
    xt: Term,
    yt: Term,
    xd: Term,
    yd: Term,
    o: Seq<Term>,
    od: Seq<Term>,
)
    requires
        a::is_arith(name, 6),
        o.len() == 2,
        od.len() == 2,
        a::number(xt) is Some,
        a::number(yt) is Some,
        a::number(xd) is Some,
        a::number(yd) is Some,
        zero_num(xd) ==> zero_num(xt),
        zero_num(yd) ==> zero_num(yt),
        a::arith_result(Term::Comp(name, seq![ws::exactly(), xt, ws::exactly(), yt] + o)) is Some,
    ensures
        a::arith_result(Term::Comp(name, seq![ws::exactly(), xd, ws::exactly(), yd] + od)) is Some,
        a::arith_result(Term::Comp(name, seq![ws::exactly(), xt, ws::exactly(), yt] + o))->Some_0.0
            == ws::exactly(),
        a::arith_result(Term::Comp(name, seq![ws::exactly(), xd, ws::exactly(), yd] + od))->Some_0.0
            == ws::exactly(),
        rel_val(
            a::arith_result(
                Term::Comp(name, seq![ws::exactly(), xt, ws::exactly(), yt] + o),
            )->Some_0.1,
            a::arith_result(
                Term::Comp(name, seq![ws::exactly(), xd, ws::exactly(), yd] + od),
            )->Some_0.1,
        ),
{
    reveal_strlit("exactly");
    let gt = Term::Comp(name, seq![ws::exactly(), xt, ws::exactly(), yt] + o);
    let gd = Term::Comp(name, seq![ws::exactly(), xd, ws::exactly(), yd] + od);
    let argst = seq![ws::exactly(), xt, ws::exactly(), yt] + o;
    let argsd = seq![ws::exactly(), xd, ws::exactly(), yd] + od;
    assert(argst[0] == ws::exactly() && argst[1] == xt && argst[2] == ws::exactly() && argst[3]
        == yt);
    assert(argsd[0] == ws::exactly() && argsd[1] == xd && argsd[2] == ws::exactly() && argsd[3]
        == yd);
    assert(argst.len() == 6 && argsd.len() == 6);
    assert(ws::exactly() == a::kw("exactly"@));
    let (pxt, qxt) = a::number(xt)->Some_0;
    let (pyt, qyt) = a::number(yt)->Some_0;
    let (pxd, qxd) = a::number(xd)->Some_0;
    let (pyd, qyd) = a::number(yd)->Some_0;
    number_den(xt);
    number_den(yt);
    number_den(xd);
    number_den(yd);
    assert(a::vset(argst[0], argst[1]) == Some(a::VSet::Point(pxt, qxt)));
    assert(a::vset(argst[2], argst[3]) == Some(a::VSet::Point(pyt, qyt)));
    assert(a::vset(argsd[0], argsd[1]) == Some(a::VSet::Point(pxd, qxd)));
    assert(a::vset(argsd[2], argsd[3]) == Some(a::VSet::Point(pyd, qyd)));
    if name == a::product_name() {
        // a point times a point: a point, zero iff a factor is zero
        let nt = if pxt == 0 || pyt == 0 {
            0
        } else {
            pxt * pyt
        };
        let dt = if pxt == 0 || pyt == 0 {
            1
        } else {
            qxt * qyt
        };
        let nd = if pxd == 0 || pyd == 0 {
            0
        } else {
            pxd * pyd
        };
        let dd = if pxd == 0 || pyd == 0 {
            1
        } else {
            qxd * qyd
        };
        assert(a::vprod(a::VSet::Point(pxt, qxt), a::VSet::Point(pyt, qyt)) == Some(
            a::VSet::Point(nt, dt),
        ));
        assert(a::vprod(a::VSet::Point(pxd, qxd), a::VSet::Point(pyd, qyd)) == Some(
            a::VSet::Point(nd, dd),
        ));
        assert(dt >= 1) by (nonlinear_arith)
            requires
                dt == 1 || dt == qxt * qyt,
                qxt >= 1,
                qyt >= 1,
        ;
        assert(dd >= 1) by (nonlinear_arith)
            requires
                dd == 1 || dd == qxd * qyd,
                qxd >= 1,
                qyd >= 1,
        ;
        assert(nt == 0 <==> (pxt == 0 || pyt == 0)) by (nonlinear_arith)
            requires
                nt == (if pxt == 0 || pyt == 0 {
                    0
                } else {
                    pxt * pyt
                }),
        ;
        assert(nd == 0 <==> (pxd == 0 || pyd == 0)) by (nonlinear_arith)
            requires
                nd == (if pxd == 0 || pyd == 0 {
                    0
                } else {
                    pxd * pyd
                }),
        ;
        canon_zero(nt, dt);
        canon_zero(nd, dd);
    } else {
        assert(name == a::quotient_name());
        // a nonzero divisor: the tuple's divisor is nonzero, so is the default's
        assert(pyt != 0);
        assert(pyd != 0);
        let (rt, rtd) = if pyt > 0 {
            (qyt, pyt)
        } else {
            (-qyt, -pyt)
        };
        let (rd, rdd) = if pyd > 0 {
            (qyd, pyd)
        } else {
            (-qyd, -pyd)
        };
        assert(a::vquot(a::VSet::Point(pxt, qxt), a::VSet::Point(pyt, qyt)) == a::vprod(
            a::VSet::Point(pxt, qxt),
            a::VSet::Point(rt, rtd),
        ));
        assert(a::vquot(a::VSet::Point(pxd, qxd), a::VSet::Point(pyd, qyd)) == a::vprod(
            a::VSet::Point(pxd, qxd),
            a::VSet::Point(rd, rdd),
        ));
        assert(rt != 0 && rd != 0);
        let nt = if pxt == 0 {
            0
        } else {
            pxt * rt
        };
        let dt = if pxt == 0 {
            1
        } else {
            qxt * rtd
        };
        let nd = if pxd == 0 {
            0
        } else {
            pxd * rd
        };
        let dd = if pxd == 0 {
            1
        } else {
            qxd * rdd
        };
        assert(a::vprod(a::VSet::Point(pxt, qxt), a::VSet::Point(rt, rtd)) == Some(
            a::VSet::Point(nt, dt),
        ));
        assert(a::vprod(a::VSet::Point(pxd, qxd), a::VSet::Point(rd, rdd)) == Some(
            a::VSet::Point(nd, dd),
        ));
        assert(dt >= 1) by (nonlinear_arith)
            requires
                dt == 1 || dt == qxt * rtd,
                qxt >= 1,
                rtd >= 1,
        ;
        assert(dd >= 1) by (nonlinear_arith)
            requires
                dd == 1 || dd == qxd * rdd,
                qxd >= 1,
                rdd >= 1,
        ;
        assert(nt == 0 <==> pxt == 0) by (nonlinear_arith)
            requires
                nt == (if pxt == 0 {
                    0
                } else {
                    pxt * rt
                }),
                rt != 0,
        ;
        assert(nd == 0 <==> pxd == 0) by (nonlinear_arith)
            requires
                nd == (if pxd == 0 {
                    0
                } else {
                    pxd * rd
                }),
                rd != 0,
        ;
        canon_zero(nt, dt);
        canon_zero(nd, dd);
    }
}

// --- the default witness ---
// The comparisons an exact count can meet (compare.rs `within_rows`).
proof fn within_exactly(bn: Seq<u8>, x: int, y: int)
    requires
        ckc_spec::compare::within(ckc_spec::v1text::ascii("exactly"@), x, bn, y),
    ensures
        (bn == ckc_spec::v1text::ascii("geq"@) && x >= y) || (bn == ckc_spec::v1text::ascii(
            "greater"@,
        ) && x > y) || (bn == ckc_spec::v1text::ascii("leq"@) && x <= y) || (bn
            == ckc_spec::v1text::ascii("less"@) && x < y) || (bn == ckc_spec::v1text::ascii(
            "exactly"@,
        ) && x == y),
{
    let rows = ckc_spec::compare::within_rows();
    within_row(rows, ckc_spec::v1text::ascii("exactly"@), x, bn, y);
    let i = choose|i: int|
        0 <= i < rows.len() && (#[trigger] rows[i]).0 == ckc_spec::v1text::ascii("exactly"@)
            && rows[i].1 == bn && ckc_spec::compare::op_holds(rows[i].2, x, y);
    reveal_strlit("exactly");
    reveal_strlit("eq");
    reveal_strlit("geq");
    reveal_strlit("greater");
    reveal_strlit("leq");
    reveal_strlit("less");
    reveal_strlit(">=");
    reveal_strlit(">");
    reveal_strlit("=<");
    reveal_strlit("<");
    reveal_strlit("=:=");
    reveal(ckc_spec::v1text::ascii);
    assert(rows.len() == 15);
    assert(rows[0] == (
        ckc_spec::v1text::ascii("eq"@),
        ckc_spec::v1text::ascii("geq"@),
        ckc_spec::v1text::ascii(">="@),
    ));
    assert(rows[1] == (
        ckc_spec::v1text::ascii("exactly"@),
        ckc_spec::v1text::ascii("geq"@),
        ckc_spec::v1text::ascii(">="@),
    ));
    assert(rows[2] == (
        ckc_spec::v1text::ascii("geq"@),
        ckc_spec::v1text::ascii("geq"@),
        ckc_spec::v1text::ascii(">="@),
    ));
    assert(rows[3] == (
        ckc_spec::v1text::ascii("greater"@),
        ckc_spec::v1text::ascii("geq"@),
        ckc_spec::v1text::ascii(">="@),
    ));
    assert(rows[4] == (
        ckc_spec::v1text::ascii("eq"@),
        ckc_spec::v1text::ascii("greater"@),
        ckc_spec::v1text::ascii(">"@),
    ));
    assert(rows[5] == (
        ckc_spec::v1text::ascii("exactly"@),
        ckc_spec::v1text::ascii("greater"@),
        ckc_spec::v1text::ascii(">"@),
    ));
    assert(rows[6] == (
        ckc_spec::v1text::ascii("geq"@),
        ckc_spec::v1text::ascii("greater"@),
        ckc_spec::v1text::ascii(">"@),
    ));
    assert(rows[7] == (
        ckc_spec::v1text::ascii("greater"@),
        ckc_spec::v1text::ascii("greater"@),
        ckc_spec::v1text::ascii(">="@),
    ));
    assert(rows[8] == (
        ckc_spec::v1text::ascii("exactly"@),
        ckc_spec::v1text::ascii("leq"@),
        ckc_spec::v1text::ascii("=<"@),
    ));
    assert(rows[9] == (
        ckc_spec::v1text::ascii("leq"@),
        ckc_spec::v1text::ascii("leq"@),
        ckc_spec::v1text::ascii("=<"@),
    ));
    assert(rows[10] == (
        ckc_spec::v1text::ascii("less"@),
        ckc_spec::v1text::ascii("leq"@),
        ckc_spec::v1text::ascii("=<"@),
    ));
    assert(rows[11] == (
        ckc_spec::v1text::ascii("exactly"@),
        ckc_spec::v1text::ascii("less"@),
        ckc_spec::v1text::ascii("<"@),
    ));
    assert(rows[12] == (
        ckc_spec::v1text::ascii("leq"@),
        ckc_spec::v1text::ascii("less"@),
        ckc_spec::v1text::ascii("<"@),
    ));
    assert(rows[13] == (
        ckc_spec::v1text::ascii("less"@),
        ckc_spec::v1text::ascii("less"@),
        ckc_spec::v1text::ascii("=<"@),
    ));
    assert(rows[14] == (
        ckc_spec::v1text::ascii("exactly"@),
        ckc_spec::v1text::ascii("exactly"@),
        ckc_spec::v1text::ascii("=:="@),
    ));
    let ex = ckc_spec::v1text::ascii("exactly"@);
    assert(ex.len() == 7 && ex[0] == 101u8);
    assert(ckc_spec::v1text::ascii("eq"@).len() == 2);
    assert(ckc_spec::v1text::ascii("geq"@).len() == 3);
    assert(ckc_spec::v1text::ascii("greater"@)[0] == 103u8);
    assert(ckc_spec::v1text::ascii("leq"@).len() == 3);
    assert(ckc_spec::v1text::ascii("less"@).len() == 4);
    assert(ckc_spec::v1text::ascii("eq"@) != ex);
    assert(ckc_spec::v1text::ascii("geq"@) != ex);
    assert(ckc_spec::v1text::ascii("greater"@) != ex);
    assert(ckc_spec::v1text::ascii("leq"@) != ex);
    assert(ckc_spec::v1text::ascii("less"@) != ex);
    assert(i == 1 || i == 5 || i == 8 || i == 11 || i == 14);
    let ge = ckc_spec::v1text::ascii(">="@);
    let gt = ckc_spec::v1text::ascii(">"@);
    let le = ckc_spec::v1text::ascii("=<"@);
    let lt = ckc_spec::v1text::ascii("<"@);
    let eqs = ckc_spec::v1text::ascii("=:="@);
    assert(ge.len() == 2 && ge[0] == 62u8);
    assert(gt.len() == 1 && gt[0] == 62u8);
    assert(le.len() == 2 && le[0] == 61u8);
    assert(lt.len() == 1 && lt[0] == 60u8);
    assert(eqs.len() == 3);
    assert(gt != ge && le != ge && le != gt && lt != ge && lt != gt && lt != le);
    assert(eqs != ge && eqs != gt && eqs != le && eqs != lt);
    if i == 1 {
        assert(ckc_spec::compare::op_holds(ge, x, y) == (x >= y));
    } else if i == 5 {
        assert(ckc_spec::compare::op_holds(gt, x, y) == (x > y));
    } else if i == 8 {
        assert(ckc_spec::compare::op_holds(le, x, y) == (x <= y));
    } else if i == 11 {
        assert(ckc_spec::compare::op_holds(lt, x, y) == (x < y));
    } else {
        assert(ckc_spec::compare::op_holds(eqs, x, y) == (x == y));
    }
}

proof fn within_row(rows: Seq<(Seq<u8>, Seq<u8>, Seq<u8>)>, s: Seq<u8>, x: int, b: Seq<u8>, y: int)
    requires
        ckc_spec::compare::within_from(rows, s, x, b, y),
    ensures
        exists|i: int|
            0 <= i < rows.len() && (#[trigger] rows[i]).0 == s && rows[i].1 == b
                && ckc_spec::compare::op_holds(rows[i].2, x, y),
    decreases rows.len(),
{
    if rows[0].0 == s && rows[0].1 == b && ckc_spec::compare::op_holds(rows[0].2, x, y) {
        assert(rows[0].0 == s);
    } else {
        within_row(rows.drop_first(), s, x, b, y);
        let i = choose|i: int|
            0 <= i < rows.drop_first().len() && (#[trigger] rows.drop_first()[i]).0 == s
                && rows.drop_first()[i].1 == b && ckc_spec::compare::op_holds(
                rows.drop_first()[i].2,
                x,
                y,
            );
        assert(rows.drop_first()[i] == rows[i + 1]);
    }
}

proof fn w0_dominates(cond: Option<(Term, Term)>, v: Term)
    requires
        ws::admits(cond, v),
    ensures
        a::number(v) is Some,
        a::number(ws::w0(cond)) is Some,
        zero_num(ws::w0(cond)) ==> zero_num(v),
{
    reveal_strlit("exactly");
    reveal_strlit("geq");
    reveal_strlit("leq");
    reveal_strlit("greater");
    reveal_strlit("less");
    match cond {
        None => {},
        Some((op, b)) => {
            let t = spec::lit("guideline_satisfies"@, seq![ws::exactly(), v, op, b]);
            assert(ckc_spec::compare::compare_holds(t));
            let xs = seq![ws::exactly(), v, op, b];
            assert(t == Term::Comp(ckc_spec::v1text::ascii("guideline_satisfies"@), xs));
            assert(a::number(b) is Some);
            let (p, q) = a::number(b)->Some_0;
            number_den(b);
            assert(op is Atom);
            let bn = op->Atom_0;
            let (pv, qv) = a::number(v)->Some_0;
            assert(ckc_spec::compare::within(
                ckc_spec::v1text::ascii("exactly"@),
                pv * q,
                bn,
                p * qv,
            ));
            within_exactly(bn, pv * q, p * qv);
            if op == a::kw("geq"@) && p == 0 {
            } else if op == a::kw("leq"@) && p == 0 {
            } else if op == a::kw("greater"@) {
                let k: int = if p + q == 0 {
                    2
                } else {
                    1
                };
                canon_zero(p + k * q, q);
            } else if op == a::kw("less"@) {
                let k: int = if p == q {
                    -2
                } else {
                    -1
                };
                canon_zero(p + k * q, q);
            } else {
                // the remaining rows: geq / leq away from 0 (nonzero) or exactly
                if p == 0 {
                    assert(op == a::kw("exactly"@));
                    assert(bn == ckc_spec::v1text::ascii("exactly"@));
                    assert(pv * q == p * qv);
                    assert(pv == 0) by (nonlinear_arith)
                        requires
                            pv * q == p * qv,
                            p == 0,
                            q >= 1,
                    ;
                }
            }
        },
    }
}

// --- the chain over related pairs ---
pub proof fn op_pairs_index(sel: ws::Sel, t: Seq<Term>)
    requires
        t.len() == sel.ops.len(),
    ensures
        ws::op_pairs(sel, t).len() == 2 * sel.ops.len(),
        forall|i: int|
            0 <= i < sel.ops.len() ==> ws::op_pairs(sel, t)[2 * i] == (
                (#[trigger] sel.ops[i]).s,
                ws::exactly(),
            ) && ws::op_pairs(sel, t)[2 * i + 1] == (sel.ops[i].n, t[i]),
{
    let rows = Seq::new(
        sel.ops.len(),
        |i: int| seq![(sel.ops[i].s, ws::exactly()), (sel.ops[i].n, t[i])],
    );
    assert(ws::op_pairs(sel, t) == rows.flatten());
    flatten_pairs(rows, sel.ops.len() as int);
    assert forall|i: int| 0 <= i < sel.ops.len() implies ws::op_pairs(sel, t)[2 * i] == (
        (#[trigger] sel.ops[i]).s,
        ws::exactly(),
    ) && ws::op_pairs(sel, t)[2 * i + 1] == (sel.ops[i].n, t[i]) by {
        assert(rows[i] == seq![(sel.ops[i].s, ws::exactly()), (sel.ops[i].n, t[i])]);
    }
}

// The flatten of a sequence of 2-element rows: row i at 2i, 2i + 1.
pub proof fn flatten_pairs<A>(rows: Seq<Seq<A>>, n: int)
    requires
        n == rows.len(),
        forall|i: int| 0 <= i < n ==> (#[trigger] rows[i]).len() == 2,
    ensures
        rows.flatten().len() == 2 * n,
        forall|i: int|
            0 <= i < n ==> rows.flatten()[2 * i] == (#[trigger] rows[i])[0] && rows.flatten()[2 * i
                + 1] == rows[i][1],
    decreases n,
{
    if n > 0 {
        let pre = rows.drop_last();
        assert forall|i: int| 0 <= i < n - 1 implies (#[trigger] pre[i]).len() == 2 by {
            assert(pre[i] == rows[i]);
        }
        flatten_pairs(pre, n - 1);
        assert(rows =~= pre.push(rows[n - 1]));
        pre.lemma_flatten_push(rows[n - 1]);
        assert(rows.flatten() =~= pre.flatten() + rows[n - 1]);
        assert forall|i: int| 0 <= i < n implies rows.flatten()[2 * i] == (#[trigger] rows[i])[0]
            && rows.flatten()[2 * i + 1] == rows[i][1] by {
            if i < n - 1 {
                assert(pre[i] == rows[i]);
            }
        }
    } else {
        assert(rows =~= Seq::<Seq<A>>::empty());
    }
}

pub proof fn outputs_index(sel: ws::Sel)
    ensures
        ws::outputs(sel).len() == 2 * sel.chain.len(),
        forall|m: int|
            0 <= m < sel.chain.len() ==> ws::outputs(sel)[2 * m] == arg(#[trigger] sel.chain[m], 4)
                && ws::outputs(sel)[2 * m + 1] == arg(sel.chain[m], 5),
{
    let rows = sel.chain.map_values(|c: Term| seq![arg(c, 4), arg(c, 5)]);
    assert(ws::outputs(sel) == rows.flatten());
    flatten_pairs(rows, sel.chain.len() as int);
    assert forall|m: int| 0 <= m < sel.chain.len() implies ws::outputs(sel)[2 * m] == arg(
        #[trigger] sel.chain[m],
        4,
    ) && ws::outputs(sel)[2 * m + 1] == arg(sel.chain[m], 5) by {
        assert(rows[m] == seq![arg(sel.chain[m], 4), arg(sel.chain[m], 5)]);
    }
}

// The distinct keys of a well-shaped chain: comparisons, counts, outputs.
pub open spec fn keys(sel: ws::Sel) -> Seq<Term> {
    sel.ops.map_values(|o: ws::Opnd| o.s) + ws::counts(sel) + ws::outputs(sel)
}

pub proof fn keys_index(sel: ws::Sel)
    ensures
        keys(sel).len() == 2 * sel.ops.len() + 2 * sel.chain.len(),
        forall|i: int|
            0 <= i < sel.ops.len() ==> keys(sel)[i] == (#[trigger] sel.ops[i]).s && keys(
                sel,
            )[sel.ops.len() + i] == sel.ops[i].n,
        forall|m: int|
            0 <= m < sel.chain.len() ==> keys(sel)[2 * sel.ops.len() + 2 * m] == arg(
                #[trigger] sel.chain[m],
                4,
            ) && keys(sel)[2 * sel.ops.len() + 2 * m + 1] == arg(sel.chain[m], 5),
{
    outputs_index(sel);
    let n = sel.ops.len() as int;
    assert forall|m: int| 0 <= m < sel.chain.len() implies keys(sel)[2 * n + 2 * m] == arg(
        #[trigger] sel.chain[m],
        4,
    ) && keys(sel)[2 * n + 2 * m + 1] == arg(sel.chain[m], 5) by {
        assert(keys(sel)[2 * n + 2 * m] == ws::outputs(sel)[2 * m]);
        assert(keys(sel)[2 * n + 2 * m + 1] == ws::outputs(sel)[2 * m + 1]);
    }
}

// Node k's inputs resolve: each comparison to `exactly`, each count to a number;
// the outputs of node k on are absent.
pub open spec fn keys_ok(sel: ws::Sel, k: int, p: Seq<(Term, Term)>) -> bool {
    &&& forall|i: int|
        0 <= i < sel.ops.len() ==> spec::lookup(p, (#[trigger] sel.ops[i]).s) == Some(ws::exactly())
            && (spec::lookup(p, sel.ops[i].n) matches Some(v) && a::number(v) is Some)
    &&& forall|m: int|
        0 <= m < k ==> spec::lookup(p, arg(#[trigger] sel.chain[m], 4)) == Some(ws::exactly()) && (
        spec::lookup(p, arg(sel.chain[m], 5)) matches Some(v) && a::number(v) is Some)
    &&& forall|m: int|
        k <= m < sel.chain.len() ==> spec::lookup(p, arg(#[trigger] sel.chain[m], 4)) is None
            && spec::lookup(p, arg(sel.chain[m], 5)) is None
}

pub proof fn lookup_push(p: Seq<(Term, Term)>, k: Term, v: Term, key: Term)
    ensures
        spec::lookup(p.push((k, v)), key) == match spec::lookup(p, key) {
            Some(w) => Some(w),
            None => if key == k {
                Some(v)
            } else {
                None
            },
        },
    decreases p.len(),
{
    reveal_with_fuel(spec::lookup, 2);
    if p.len() > 0 {
        assert(p.push((k, v))[0] == p[0]);
        assert(p.push((k, v)).drop_first() =~= p.drop_first().push((k, v)));
        lookup_push(p.drop_first(), k, v, key);
    } else {
        assert(p.push((k, v)).drop_first() =~= Seq::<(Term, Term)>::empty());
    }
}

// An input pair (s, n) of node k binds to `exactly` and a number in both runs,
// the counts related.
pub proof fn input_binds(
    sel: ws::Sel,
    k: int,
    s: Term,
    n: Term,
    pt: Seq<(Term, Term)>,
    pd: Seq<(Term, Term)>,
)
    requires
        ws::chain_shape(sel),
        0 <= k < sel.chain.len(),
        ws::chain_input(sel, k, s, n),
        rel_pairs(pt, pd),
        keys_ok(sel, k, pt),
    ensures
        spec::bind_all(s, pt) == ws::exactly(),
        spec::bind_all(s, pd) == ws::exactly(),
        a::number(spec::bind_all(n, pt)) is Some,
        a::number(spec::bind_all(n, pd)) is Some,
        zero_num(spec::bind_all(n, pd)) ==> zero_num(spec::bind_all(n, pt)),
{
    reveal_strlit("exactly");
    rel_vals_ground(pt, pd);
    if exists|i: int|
        0 <= i < sel.ops.len() && s == (#[trigger] sel.ops[i]).s && n == sel.ops[i].n {
        let i = choose|i: int|
            0 <= i < sel.ops.len() && s == (#[trigger] sel.ops[i]).s && n == sel.ops[i].n;
        assert(sel.ops[i].s is Var && sel.ops[i].n is Var);
        lookup_rel(pt, pd, s);
        lookup_rel(pt, pd, n);
        bind_var(s->Var_0, pt);
        bind_var(s->Var_0, pd);
        bind_var(n->Var_0, pt);
        bind_var(n->Var_0, pd);
        let vt = spec::lookup(pt, n)->Some_0;
        let vd = spec::lookup(pd, n)->Some_0;
        assert(rel_val(vt, vd));
        assert(!(vt == ws::exactly()));
    } else if s == ws::exactly() && a::number(n) is Some {
        crate::k3_sound::log::number_ground(n);
        bind_ground(s, pt);
        bind_ground(s, pd);
        bind_ground(n, pt);
        bind_ground(n, pd);
    } else {
        let m = choose|m: int|
            0 <= m < k && s == arg(#[trigger] sel.chain[m], 4) && n == arg(sel.chain[m], 5);
        outputs_index(sel);
        assert(ws::outputs(sel)[2 * m] == s && ws::outputs(sel)[2 * m + 1] == n);
        assert(s is Var && n is Var);
        lookup_rel(pt, pd, s);
        lookup_rel(pt, pd, n);
        bind_var(s->Var_0, pt);
        bind_var(s->Var_0, pd);
        bind_var(n->Var_0, pt);
        bind_var(n->Var_0, pd);
        let vt = spec::lookup(pt, n)->Some_0;
        let vd = spec::lookup(pd, n)->Some_0;
        assert(rel_val(vt, vd));
        assert(!(vt == ws::exactly()));
    }
}

pub proof fn chain_step(sel: ws::Sel, k: nat, pt: Seq<(Term, Term)>, pd: Seq<(Term, Term)>)
    requires
        ws::chain_shape(sel),
        k <= sel.chain.len(),
        rel_pairs(pt, pd),
        keys_ok(sel, k as int, pt),
        spec::chain_pairs(sel.chain.skip(k as int), pt) is Some,
    ensures
        spec::chain_pairs(sel.chain.skip(k as int), pd) is Some,
    decreases sel.chain.len() - k,
{
    reveal_with_fuel(spec::chain_pairs, 1);
    if k < sel.chain.len() {
        let goals = sel.chain.skip(k as int);
        let g = sel.chain[k as int];
        assert(goals[0] == g);
        assert(goals.drop_first() =~= sel.chain.skip(k as int + 1));
        assert(ws::is_arith_lit(g));
        let name = g->Comp_0;
        let xs = g->Comp_1;
        assert(xs.len() == 6) by {
            reveal(a::is_arith);
        }
        input_binds(sel, k as int, arg(g, 0), arg(g, 1), pt, pd);
        input_binds(sel, k as int, arg(g, 2), arg(g, 3), pt, pd);
        bind_comp(name, xs, pt);
        bind_comp(name, xs, pd);
        let bt = spec::bind_all(g, pt);
        let bd = spec::bind_all(g, pd);
        let at = bt->Comp_1;
        let ad = bd->Comp_1;
        let xt = at[1];
        let yt = at[3];
        let xd = ad[1];
        let yd = ad[3];
        assert(at =~= seq![ws::exactly(), xt, ws::exactly(), yt] + seq![at[4], at[5]]);
        assert(ad =~= seq![ws::exactly(), xd, ws::exactly(), yd] + seq![ad[4], ad[5]]);
        node_points(name, xt, yt, xd, yd, seq![at[4], at[5]], seq![ad[4], ad[5]]);
        let (svt, nvt) = a::arith_result(bt)->Some_0;
        let (svd, nvd) = a::arith_result(bd)->Some_0;
        let o4 = arg(g, 4);
        let o5 = arg(g, 5);
        let pt2 = pt.push((o4, svt)).push((o5, nvt));
        let pd2 = pd.push((o4, svd)).push((o5, nvd));
        // the outputs: distinct variables, absent from both runs' pairs so far
        keys_index(sel);
        let nops = sel.ops.len() as int;
        assert(keys(sel)[2 * nops + 2 * (k as int)] == o4 && keys(sel)[2 * nops + 2 * (k as int)
            + 1] == o5);
        assert(o4 is Var && o5 is Var) by {
            outputs_index(sel);
            assert(ws::outputs(sel)[2 * (k as int)] == o4);
            assert(ws::outputs(sel)[2 * (k as int) + 1] == o5);
        }
        assert(o4 != o5);
        assert(rel_pairs(pt2, pd2)) by {
            assert forall|i: int| 0 <= i < pt2.len() implies (#[trigger] pt2[i]).0 == pd2[i].0
                && pt2[i].0 is Var && rel_val(pt2[i].1, pd2[i].1) by {
                if i < pt.len() {
                    assert(pt2[i] == pt[i] && pd2[i] == pd[i]);
                }
            }
        }
        assert(keys_ok(sel, k as int + 1, pt2)) by {
            lookup_rel(pt, pd, o4);
            assert forall|i: int| 0 <= i < sel.ops.len() implies spec::lookup(
                pt2,
                (#[trigger] sel.ops[i]).s,
            ) == Some(ws::exactly()) && (spec::lookup(pt2, sel.ops[i].n) matches Some(v)
                && a::number(v) is Some) by {
                lookup_push(pt, o4, svt, sel.ops[i].s);
                lookup_push(pt.push((o4, svt)), o5, nvt, sel.ops[i].s);
                lookup_push(pt, o4, svt, sel.ops[i].n);
                lookup_push(pt.push((o4, svt)), o5, nvt, sel.ops[i].n);
            }
            assert forall|m: int| 0 <= m < k + 1 implies spec::lookup(
                pt2,
                arg(#[trigger] sel.chain[m], 4),
            ) == Some(ws::exactly()) && (spec::lookup(pt2, arg(sel.chain[m], 5)) matches Some(v)
                && a::number(v) is Some) by {
                lookup_push(pt, o4, svt, arg(sel.chain[m], 4));
                lookup_push(pt.push((o4, svt)), o5, nvt, arg(sel.chain[m], 4));
                lookup_push(pt, o4, svt, arg(sel.chain[m], 5));
                lookup_push(pt.push((o4, svt)), o5, nvt, arg(sel.chain[m], 5));
                if m == k {
                    assert(arg(sel.chain[m], 5) != o4);
                }
            }
            assert forall|m: int| k + 1 <= m < sel.chain.len() implies spec::lookup(
                pt2,
                arg(#[trigger] sel.chain[m], 4),
            ) is None && spec::lookup(pt2, arg(sel.chain[m], 5)) is None by {
                let a4 = arg(sel.chain[m], 4);
                let a5 = arg(sel.chain[m], 5);
                assert(keys(sel)[2 * nops + 2 * m] == a4 && keys(sel)[2 * nops + 2 * m + 1] == a5);
                assert(a4 != o4 && a4 != o5 && a5 != o4 && a5 != o5);
                lookup_push(pt, o4, svt, a4);
                lookup_push(pt.push((o4, svt)), o5, nvt, a4);
                lookup_push(pt, o4, svt, a5);
                lookup_push(pt.push((o4, svt)), o5, nvt, a5);
            }
        }
        chain_step(sel, k + 1, pt2, pd2);
    }
}

proof fn base_rel(sel: ws::Sel, t: Seq<Term>)
    requires
        ws::chain_shape(sel),
        t.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> ws::admits(sel.ops[i].cond, #[trigger] t[i]),
    ensures
        rel_pairs(ws::op_pairs(sel, t), ws::op_pairs(sel, ws::defaults(sel))),
{
    let d = ws::defaults(sel);
    let pt = ws::op_pairs(sel, t);
    let pd = ws::op_pairs(sel, d);
    op_pairs_index(sel, t);
    op_pairs_index(sel, d);
    let n = sel.ops.len() as int;
    assert forall|i: int| 0 <= i < n implies a::number(#[trigger] t[i]) is Some && a::number(
        d[i],
    ) is Some && (zero_num(d[i]) ==> zero_num(t[i])) by {
        w0_dominates(sel.ops[i].cond, t[i]);
        assert(d[i] == ws::w0(sel.ops[i].cond));
    }
    assert forall|j: int| 0 <= j < pt.len() implies (#[trigger] pt[j]).0 == pd[j].0
        && pt[j].0 is Var && rel_val(pt[j].1, pd[j].1) by {
        let i = j / 2;
        assert(0 <= i < n);
        let o = sel.ops[i];
        if j % 2 == 0 {
            assert(j == 2 * i);
        } else {
            assert(j == 2 * i + 1);
        }
    }
}

pub proof fn base_keys(sel: ws::Sel, t: Seq<Term>)
    requires
        ws::chain_shape(sel),
        t.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> a::number(#[trigger] t[i]) is Some,
    ensures
        keys_ok(sel, 0, ws::op_pairs(sel, t)),
        forall|i: int|
            0 <= i < sel.ops.len() ==> spec::lookup(ws::op_pairs(sel, t), (#[trigger] sel.ops[i]).n)
                == Some(t[i]),
{
    let pt = ws::op_pairs(sel, t);
    op_pairs_index(sel, t);
    keys_index(sel);
    let n = sel.ops.len() as int;
    assert(keys(sel).no_duplicates());
    assert forall|i: int| 0 <= i < n implies spec::lookup(pt, (#[trigger] sel.ops[i]).s) == Some(
        ws::exactly(),
    ) && (spec::lookup(pt, sel.ops[i].n) matches Some(v) && a::number(v) is Some) && spec::lookup(
        pt,
        sel.ops[i].n,
    ) == Some(t[i]) by {
        assert forall|j: int| 0 <= j < 2 * i implies (#[trigger] pt[j]).0 != pt[2 * i].0 by {
            let q = j / 2;
            if j % 2 == 0 {
                assert(j == 2 * q);
                assert(keys(sel)[q] != keys(sel)[i]);
            } else {
                assert(j == 2 * q + 1);
                assert(keys(sel)[n + q] != keys(sel)[i]);
            }
        }
        lookup_at(pt, 2 * i);
        assert forall|j: int| 0 <= j < 2 * i + 1 implies (#[trigger] pt[j]).0 != pt[2 * i
            + 1].0 by {
            let q = j / 2;
            if j % 2 == 0 {
                assert(j == 2 * q);
                assert(keys(sel)[q] != keys(sel)[n + i]);
            } else {
                assert(j == 2 * q + 1);
                assert(keys(sel)[n + q] != keys(sel)[n + i]);
            }
        }
        lookup_at(pt, 2 * i + 1);
    }
    assert forall|m: int| 0 <= m < sel.chain.len() implies spec::lookup(
        pt,
        arg(#[trigger] sel.chain[m], 4),
    ) is None && spec::lookup(pt, arg(sel.chain[m], 5)) is None by {
        let a4 = arg(sel.chain[m], 4);
        let a5 = arg(sel.chain[m], 5);
        assert forall|j: int| 0 <= j < pt.len() implies (#[trigger] pt[j]).0 != a4 && pt[j].0
            != a5 by {
            let q = j / 2;
            if j % 2 == 0 {
                assert(j == 2 * q);
                assert(keys(sel)[q] != keys(sel)[2 * n + 2 * m]);
                assert(keys(sel)[q] != keys(sel)[2 * n + 2 * m + 1]);
            } else {
                assert(j == 2 * q + 1);
                assert(keys(sel)[n + q] != keys(sel)[2 * n + 2 * m]);
                assert(keys(sel)[n + q] != keys(sel)[2 * n + 2 * m + 1]);
            }
        }
        lookup_absent(pt, a4);
        lookup_absent(pt, a5);
    }
}

pub proof fn default_chain_holds(g: spec::Group, sel: ws::Sel, t: Seq<Term>)
    requires
        ws::chain_shape(sel),
        t.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> ws::admits(sel.ops[i].cond, #[trigger] t[i]),
        ws::tuple_pairs(g, sel, t) is Some,
    ensures
        ws::tuple_pairs(g, sel, ws::defaults(sel)) is Some,
{
    let d = ws::defaults(sel);
    let pt = ws::op_pairs(sel, t);
    let pd = ws::op_pairs(sel, d);
    base_rel(sel, t);
    assert forall|i: int| 0 <= i < t.len() implies a::number(#[trigger] t[i]) is Some by {
        w0_dominates(sel.ops[i].cond, t[i]);
    }
    base_keys(sel, t);
    assert(sel.chain.skip(0) =~= sel.chain);
    chain_step(sel, 0, pt, pd);
    reveal(ws::tuple_pairs);
}

} // verus!
