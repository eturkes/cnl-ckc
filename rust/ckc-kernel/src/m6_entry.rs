#[cfg(verus_keep_ghost)]
use crate::k3_sound::naf::rename;
#[cfg(verus_keep_ghost)]
use crate::m6_arith::*;
#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use crate::m6_lockstep::*;
use crate::m6_step::*;
use ckc_spec::engine::*;
use ckc_spec::term::*;
use ckc_spec::v1text::DocClause;
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

// Contract ws D7 (cell invariance): the replay's head call and the whole run.
// The head goal meets the witness facts and the program clause by clause; only
// its own clause unifies, entering the body frame the walk accepted; the runs
// then step in lockstep until both end alike.
pub ghost struct Hx {
    pub h: Term,  // the walk's open head
    pub hoff: nat,  // the walk's renaming offset of the head's clause
    pub c: DocClause,  // the canonical clause the head reaches
    pub tht: Seq<(nat, Term)>,  // the head's instantiation at t and at u
    pub thu: Seq<(nat, Term)>,
    pub fr: Fr,  // the body frame after the head call
    pub stt: Seq<Goal>,  // the head call's body stacks at t and at u
    pub stu: Seq<Goal>,
}

// The head call's unification with clause cl in the head configuration.
pub open spec fn entry(cx: Cx, g: Term, cl: DocClause) -> UState {
    UState {
        pairs: zip(args_of(g), args_of(shift(cl.head, cx.b))),
        stack: Seq::<Goal>::empty() + body_goals(cl.body, cx.b, (cx.hdd - 1) as nat) + Seq::<
            Goal,
        >::empty(),
        sol: Seq::empty(),
    }
}

// What the head call reads: the head goals render the walk's open head; no
// witness fact and no clause but its own unifies with the open head; its own
// clause enters a body frame the walk accepted, at both points.
pub open spec fn head_ok(cx: Cx, hx: Hx) -> bool {
    &&& cx.hd
    &&& cx.hdd > 0
    &&& cx.b + cx.d == hx.hoff
    &&& is_call(hx.h)
    &&& th_ok(hx.tht)
    &&& th_ok(hx.thu)
    &&& forall|x: nat| #[trigger] th_val(hx.tht, x) is Some ==> x < hx.hoff
    &&& forall|x: nat| #[trigger] th_val(hx.thu, x) is Some ==> x < hx.hoff
    &&& rename(cx.ht, cx.b, cx.d) == inst(hx.h, hx.tht)
    &&& rename(cx.hu, cx.b, cx.d) == inst(hx.h, hx.thu)
    &&& forall|j: int| 0 <= j < cx.nf ==> !ws::unifiable(hx.h, #[trigger] cx.e.facts[j])
    &&& forall|j: int|
        0 <= j < cx.nf ==> inst(#[trigger] cx.e.facts[j], hx.tht) == inst(cx.e.facts[j], cx.st)
            && inst(cx.e.facts[j], hx.thu) == inst(cx.e.facts[j], cx.su)
    &&& forall|q: int|
        0 <= q < cx.e.p.len() && ws::unifiable(hx.h, shift((#[trigger] cx.e.p[q]).head, hx.hoff))
            ==> cx.e.p[q] == hx.c
    &&& lit_fa(hx.c.head) == lit_fa(hx.h) ==> entry_ok(cx, hx)
    &&& cx.fresh == cx.b + clause_nvars(hx.c)
}

// The head call's own clause (a candidate only when its head carries the
// head's name and arity): both unifications enter the related body frame.
pub open spec fn entry_ok(cx: Cx, hx: Hx) -> bool {
    &&& unify(entry(cx, cx.ht, hx.c)) == UOut::Ok(hx.stt, Seq::<Term>::empty())
    &&& unify(entry(cx, cx.hu, hx.c)) == UOut::Ok(hx.stu, Seq::<Term>::empty())
    &&& srel(hx.stt, hx.fr.stack, hx.fr.tht, cx.b, cx.d)
    &&& srel(hx.stu, hx.fr.stack, hx.fr.thu, cx.b, cx.d)
    &&& fr_ok(cx, hx.fr)
    &&& cuts_ok(hx.fr.stack, 1)
}

// A witness fact never meets the head: the walk's skeleton fact does not
// unify with the open head, so their instances do not either.
proof fn head_fact_fail(
    g: Term,
    h: Term,
    th: Seq<(nat, Term)>,
    sk: Term,
    f: Term,
    cstack: Seq<Goal>,
    fresh: nat,
    b: nat,
    d: nat,
)
    requires
        th_ok(th),
        h is Comp,
        sk is Comp,
        rename(g, b, d) == inst(h, th),
        !ws::unifiable(h, sk),
        f == inst(sk, th),
        ground(f),
        lit_fa(g) == lit_fa(f),
    ensures
        unify(
            UState {
                pairs: zip(args_of(g), args_of(shift(f, fresh))),
                stack: cstack,
                sol: Seq::empty(),
            },
        ) is Fail,
{
    let ua = UState {
        pairs: seq![(h, sk)],
        stack: Seq::<Goal>::empty(),
        sol: Seq::<Term>::empty(),
    };
    crate::k2_engine::unify_done(ua);
    assert(unify(ua) is Fail);
    let n = h->Comp_0;
    let ax = h->Comp_1;
    let sx = sk->Comp_1;
    crate::k3_sound::naf::rename_all_index(g->Comp_1, b, d);
    inst_all_index(ax, th);
    inst_all_index(sx, th);
    assert(inst(h, th) == Term::Comp(n, inst_all(ax, th)));
    assert(inst(sk, th) == Term::Comp(sk->Comp_0, inst_all(sx, th)));
    assert(g->Comp_0 == n);
    assert(ax.len() == g->Comp_1.len());
    assert(sk->Comp_0 == n && sx.len() == ax.len());
    unify_comp(n, ax, sx, Seq::<Term>::empty());
    let uz = UState { pairs: zip(ax, sx), stack: Seq::<Goal>::empty(), sol: Seq::<Term>::empty() };
    assert(unify(uz) is Fail);
    unify_fail_inst(uz, th);
    let ui = inst_u(uz, th);
    let fx = f->Comp_1;
    assert(fx == inst_all(sx, th));
    assert(ui.pairs =~= zip(inst_all(ax, th), fx));
    crate::k3_probe::shift_ground(f, fresh);
    let uc = UState {
        pairs: zip(args_of(g), args_of(shift(f, fresh))),
        stack: cstack,
        sol: Seq::<Term>::empty(),
    };
    let uce = UState {
        pairs: uc.pairs,
        stack: Seq::<Goal>::empty(),
        sol: Seq::<Term>::empty() + enc(cstack),
    };
    let f0 = unify_fuel(uc);
    unify_n_enc(uc, f0);
    crate::k3_sound::naf::unify_at(uce, f0);
    let v = UState {
        pairs: zip(inst_all(ax, th), fx),
        stack: Seq::<Goal>::empty(),
        sol: crate::k3_sound::naf::rename_all(enc(cstack), b, d),
    };
    assert(crate::k3_sound::naf::unify_rel(uce, v, b, d)) by {
        assert(Seq::<Term>::empty() + enc(cstack) =~= enc(cstack));
        crate::k3_sound::naf::rename_all_index(enc(cstack), b, d);
        assert forall|i: int| 0 <= i < uce.pairs.len() implies #[trigger] v.pairs[i] == (
            rename(uce.pairs[i].0, b, d),
            rename(uce.pairs[i].1, b, d),
        ) by {
            ground_elem_of(fx, i, f);
            crate::k3_sound::naf::rename_ground_id(fx[i], b, d);
        }
    }
    crate::k3_sound::naf::unify_related(uce, v, b, d);
    unify_passive(v, ui);
}

proof fn ground_elem_of(fx: Seq<Term>, i: int, f: Term)
    requires
        f is Comp,
        fx == f->Comp_1,
        ground(f),
        0 <= i < fx.len(),
    ensures
        ground(fx[i]),
{
    ground_elem(fx, i);
}

// The head goal's call from clause index ci: the witness facts and every
// clause but its own fail at t and at u alike; its own clause enters the
// related body frame.
proof fn head_call(cx: Cx, hx: Hx, ct: Cfg, cu: Cfg, ci: nat)
    requires
        cx_ok(cx),
        head_ok(cx, hx),
        hrel(cx, ct, cu),
    ensures
        step_rel(
            cx,
            call(
                cx.dbt,
                ct,
                cx.ht->Comp_0,
                cx.ht->Comp_1,
                cx.hdd,
                Seq::empty(),
                ct.stack.skip(1),
                ci,
            ),
            call(
                cx.dbu,
                cu,
                cx.hu->Comp_0,
                cx.hu->Comp_1,
                cx.hdd,
                Seq::empty(),
                cu.stack.skip(1),
                ci,
            ),
        ),
    decreases cx.dbt.len() - ci,
{
    let gt = cx.ht;
    let gu = cx.hu;
    let dd = cx.hdd;
    rel_root(gt, hx.h, hx.tht, cx.b, cx.d);
    rel_root(gu, hx.h, hx.thu, cx.b, cx.d);
    let name = gt->Comp_0;
    let ar = gt->Comp_1.len();
    assert(gu->Comp_0 == name && gu->Comp_1.len() == ar);
    next_match_eq(cx, name, ar, ci);
    next_match_bound(cx.dbt, name, ar, ci);
    crate::k3_sound::naf::next_match_shape(cx.dbt, name, ar, ci);
    assert(ct.stack[0] == Goal::Lit(gt, dd));
    assert(cu.stack[0] == Goal::Lit(gu, dd));
    assert(ct.stack.skip(1) =~= Seq::<Goal>::empty());
    assert(cu.stack.skip(1) =~= Seq::<Goal>::empty());
    assert(ct.sol =~= Seq::<Term>::empty());
    assert(cu.sol =~= Seq::<Term>::empty());
    let st = call(cx.dbt, ct, name, gt->Comp_1, dd, Seq::empty(), ct.stack.skip(1), ci);
    let su = call(cx.dbu, cu, name, gu->Comp_1, dd, Seq::empty(), cu.stack.skip(1), ci);
    assert(args_of(gt) == gt->Comp_1 && args_of(gu) == gu->Comp_1);
    match next_match(cx.dbt, name, ar, ci) {
        Option::None => {
            assert(st == fail(ct));
            assert(su == fail(cu));
            assert(fail(ct) == Step::Done(ct));
            assert(fail(cu) == Step::Done(cu));
        },
        Option::Some(m) => {
            let clt = cx.dbt[m as int];
            let clu = cx.dbu[m as int];
            assert(lit_fa(clt.head) == Option::Some((name, ar)));
            let ut = cstate(ct, clt, dd);
            let uu = cstate(cu, clu, dd);
            if m < cx.nf {
                let j = cx.nf - 1 - m;
                assert(clt == fact_clause(inst(cx.e.facts[j], cx.st)));
                assert(clu == fact_clause(inst(cx.e.facts[j], cx.su)));
                assert(cx.e.facts[j] is Comp);
                assert(inst(cx.e.facts[j], hx.tht) == inst(cx.e.facts[j], cx.st));
                assert(inst(cx.e.facts[j], hx.thu) == inst(cx.e.facts[j], cx.su));
                inst_all_index(cx.e.facts[j]->Comp_1, cx.st);
                inst_all_index(cx.e.facts[j]->Comp_1, cx.su);
                assert(lit_fa(clu.head) == lit_fa(clt.head));
                head_fact_fail(
                    gt,
                    hx.h,
                    hx.tht,
                    cx.e.facts[j],
                    clt.head,
                    ut.stack,
                    ct.fresh,
                    cx.b,
                    cx.d,
                );
                head_fact_fail(
                    gu,
                    hx.h,
                    hx.thu,
                    cx.e.facts[j],
                    clu.head,
                    uu.stack,
                    cu.fresh,
                    cx.b,
                    cx.d,
                );
                assert(ut == UState {
                    pairs: zip(args_of(gt), args_of(shift(clt.head, ct.fresh))),
                    stack: ut.stack,
                    sol: Seq::<Term>::empty(),
                });
                assert(uu == UState {
                    pairs: zip(args_of(gu), args_of(shift(clu.head, cu.fresh))),
                    stack: uu.stack,
                    sol: Seq::<Term>::empty(),
                });
                assert(st == call(
                    cx.dbt,
                    ct,
                    name,
                    gt->Comp_1,
                    dd,
                    Seq::empty(),
                    ct.stack.skip(1),
                    m + 1,
                ));
                assert(su == call(
                    cx.dbu,
                    cu,
                    name,
                    gu->Comp_1,
                    dd,
                    Seq::empty(),
                    cu.stack.skip(1),
                    m + 1,
                ));
                head_call(cx, hx, ct, cu, m + 1);
            } else {
                let q: int = m - cx.nf;
                assert(0 <= q < cx.e.p.len());
                assert(cx.dbt[cx.nf + q] == cx.e.p[q] && cx.dbu[cx.nf + q] == cx.e.p[q]);
                assert(cx.nf + q == m);
                assert(clt == cx.e.p[q] && clu == cx.e.p[q]);
                if cx.e.p[q] == hx.c {
                    // its own clause: the body frame
                    assert(lit_fa(hx.c.head) == lit_fa(hx.h));
                    assert(entry_ok(cx, hx));
                    assert(ut == entry(cx, gt, hx.c));
                    assert(uu == entry(cx, gu, hx.c));
                    let nt = called(ct, clt, m, hx.stt, Seq::<Term>::empty());
                    let nu = called(cu, clu, m, hx.stu, Seq::<Term>::empty());
                    assert(st == Step::Next(nt));
                    assert(su == Step::Next(nu));
                    let fd = Fr { stack: Seq::<Goal>::empty(), ..hx.fr };
                    let alts1 = seq![fd];
                    assert(nt.alts.len() == 1 && nu.alts.len() == 1);
                    assert(hd_alt(cx, nt.alts[0], nu.alts[0]));
                    assert(alt_ok(cx, nt.alts[0], nu.alts[0], alts1[0], 0));
                    assert(cuts_ok(alts1[0].stack, 0));
                    assert(alts_rel(cx, nt.alts, nu.alts, alts1));
                    assert(rel(cx, nt, nu, hx.fr, alts1));
                    step_next(cx, nt, nu, hx.fr, alts1);
                } else {
                    // another clause never unifies with the open head
                    assert(!ws::unifiable(hx.h, shift(cx.e.p[q].head, hx.hoff)));
                    cand_rule(
                        hx.h,
                        clt.head,
                        hx.hoff,
                        Seq::empty(),
                        hx.tht,
                        gt,
                        ut.stack,
                        ct.fresh,
                        cx.b,
                        cx.d,
                    );
                    cand_rule(
                        hx.h,
                        clu.head,
                        hx.hoff,
                        Seq::empty(),
                        hx.thu,
                        gu,
                        uu.stack,
                        cu.fresh,
                        cx.b,
                        cx.d,
                    );
                    assert(ut == UState {
                        pairs: zip(args_of(gt), args_of(shift(clt.head, ct.fresh))),
                        stack: ut.stack,
                        sol: Seq::<Term>::empty(),
                    });
                    assert(uu == UState {
                        pairs: zip(args_of(gu), args_of(shift(clu.head, cu.fresh))),
                        stack: uu.stack,
                        sol: Seq::<Term>::empty(),
                    });
                    assert(st == call(
                        cx.dbt,
                        ct,
                        name,
                        gt->Comp_1,
                        dd,
                        Seq::empty(),
                        ct.stack.skip(1),
                        m + 1,
                    ));
                    assert(su == call(
                        cx.dbu,
                        cu,
                        name,
                        gu->Comp_1,
                        dd,
                        Seq::empty(),
                        cu.stack.skip(1),
                        m + 1,
                    ));
                    head_call(cx, hx, ct, cu, m + 1);
                }
            }
        },
    }
}

// The head configuration's step: the head goal's call.
proof fn step_head(cx: Cx, hx: Hx, ct: Cfg, cu: Cfg)
    requires
        cx_ok(cx),
        head_ok(cx, hx),
        hrel(cx, ct, cu),
    ensures
        step_rel(cx, step(cx.dbt, ct), step(cx.dbu, cu)),
{
    let gt = cx.ht;
    let gu = cx.hu;
    rel_root(gt, hx.h, hx.tht, cx.b, cx.d);
    rel_root(gu, hx.h, hx.thu, cx.b, cx.d);
    assert(ct.stack[0] == Goal::Lit(gt, cx.hdd));
    assert(cu.stack[0] == Goal::Lit(gu, cx.hdd));
    assert(selected(ct.stack) == 0);
    assert(selected(cu.stack) == 0);
    assert(ct.stack.take(0) =~= Seq::<Goal>::empty());
    assert(cu.stack.take(0) =~= Seq::<Goal>::empty());
    assert(gt == Term::Comp(gt->Comp_0, gt->Comp_1));
    assert(gu == Term::Comp(gu->Comp_0, gu->Comp_1));
    assert(step(cx.dbt, ct) == call(
        cx.dbt,
        ct,
        gt->Comp_0,
        gt->Comp_1,
        cx.hdd,
        Seq::empty(),
        ct.stack.skip(1),
        ct.ci,
    ));
    assert(step(cx.dbu, cu) == call(
        cx.dbu,
        cu,
        gu->Comp_0,
        gu->Comp_1,
        cx.hdd,
        Seq::empty(),
        cu.stack.skip(1),
        cu.ci,
    ));
    head_call(cx, hx, ct, cu, ct.ci);
}

// Contract ws D7: related configurations run alike under any fuel.
pub proof fn run_lock(cx: Cx, hx: Hx, ct: Cfg, cu: Cfg, fuel: nat)
    requires
        cx_ok(cx),
        chain_ok(cx),
        head_ok(cx, hx),
        next_rel(cx, ct, cu),
    ensures
        run(cx.dbt, ct, fuel) == run(cx.dbu, cu, fuel),
    decreases fuel,
{
    assert(ct.rows =~= Seq::<Seq<Term>>::empty());
    assert(cu.rows =~= Seq::<Seq<Term>>::empty());
    if fuel > 0 {
        if hrel(cx, ct, cu) {
            step_head(cx, hx, ct, cu);
        } else {
            let (cur, alts) = choose|cur: Fr, alts: Seq<Fr>| rel(cx, ct, cu, cur, alts);
            step_lock(cx, ct, cu, cur, alts);
        }
        let st = step(cx.dbt, ct);
        let su = step(cx.dbu, cu);
        match st {
            Step::Next(c2) => {
                run_lock(cx, hx, c2, su->Next_0, (fuel - 1) as nat);
            },
            Step::Done(c2) => {
                assert(c2.rows =~= Seq::<Seq<Term>>::empty());
                assert(su->Done_0.rows =~= Seq::<Seq<Term>>::empty());
            },
            _ => {},
        }
    }
}

} // verus!
