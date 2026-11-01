#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use crate::m6_lockstep::*;
use ckc_spec::engine::*;
use ckc_spec::term::*;
use ckc_spec::v1text::DocClause;
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

// Contract ws D7 (cell invariance): one K2 step at t and at u, tracked by one
// abstract configuration of the walk. A frame = an abstract goal stack (walk
// naming) + the instantiation that renders it at t and at u + the walk's
// chain position and fuel for its first segment.
// The goals before the first naf cut: the segment the walk accepted.
pub open spec fn seg(s: Seq<Goal>) -> Seq<Goal>
    decreases s.len(),
{
    if s.len() == 0 {
        Seq::empty()
    } else {
        match s[0] {
            Goal::NafCut(_) => Seq::empty(),
            Goal::Lit(_, _) => seq![s[0]] + seg(s.drop_first()),
        }
    }
}

pub proof fn seg_props(s: Seq<Goal>)
    ensures
        seg(s).len() <= s.len(),
        forall|i: int| 0 <= i < seg(s).len() ==> #[trigger] seg(s)[i] == s[i] && s[i] is Lit,
        seg(s).len() < s.len() ==> s[seg(s).len() as int] is NafCut,
    decreases s.len(),
{
    if s.len() > 0 {
        seg_props(s.drop_first());
        if s[0] is Lit {
            assert forall|i: int| 0 <= i < seg(s).len() implies #[trigger] seg(s)[i] == s[i]
                && s[i] is Lit by {
                if i > 0 {
                    assert(seg(s)[i] == seg(s.drop_first())[i - 1]);
                    assert(s.drop_first()[i - 1] == s[i]);
                }
            }
            if seg(s).len() < s.len() {
                assert(s.drop_first()[seg(s.drop_first()).len() as int] == s[seg(s).len() as int]);
            }
        }
    }
}

// The fixed context of one head's lockstep at t and u.
pub ghost struct Cx {
    pub e: ws::WalkEnv,
    pub nf: nat,
    pub dbt: Seq<DocClause>,
    pub dbu: Seq<DocClause>,
    pub st: Seq<(nat, Term)>,
    pub su: Seq<(nat, Term)>,
    pub b: nat,
    pub d: nat,
    pub fresh: nat,
    // the renamed output pair of each chain node (walk naming)
    pub o4: Seq<nat>,
    pub o5: Seq<nat>,
    // the chain's pairs at t and u (operands, then each node's outputs)
    pub vt: Seq<(Term, Term)>,
    pub vu: Seq<(Term, Term)>,
    // the replay's head call: its goal at t and at u + depth; with hd, its
    // choicepoint sits at index 0 (fresh counter b)
    pub hd: bool,
    pub ht: Term,
    pub hu: Term,
    pub hdd: nat,
}

pub open spec fn is_ph(cx: Cx, x: nat) -> bool {
    cx.o4.contains(x) || cx.o5.contains(x)
}

// Arithmetic goals before position i of s.
pub open spec fn arank(s: Seq<Goal>, i: int) -> nat
    decreases i,
{
    if i <= 0 {
        0
    } else {
        arank(s, i - 1) + if s[i - 1] is Lit && ws::is_arith_lit(s[i - 1]->Lit_0) {
            1nat
        } else {
            0nat
        }
    }
}

// Node kk's arithmetic goal: its output pair and every earlier-output input
// sit on the nodes' placeholders.
pub open spec fn node_shape(cx: Cx, t: Term, kk: nat) -> bool {
    &&& kk < cx.e.chain.len()
    &&& t is Comp
    &&& t->Comp_1.len() == 6
    &&& ckc_spec::replay::arg(t, 4) == Term::Var(cx.o4[kk as int])
    &&& ckc_spec::replay::arg(t, 5) == Term::Var(cx.o5[kk as int])
    &&& forall|m: int|
        0 <= m < kk ==> #[trigger] from_node(cx, t, kk, 0, m) && from_node(cx, t, kk, 2, m)
}

// Input pair q of node kk reads node m's output ⇒ it sits on m's placeholders.
pub open spec fn from_node(cx: Cx, t: Term, kk: nat, q: int, m: int) -> bool {
    ckc_spec::replay::arg(cx.e.chain[kk as int], q + 1) == ckc_spec::replay::arg(cx.e.chain[m], 5)
        ==> ckc_spec::replay::arg(t, q) == Term::Var(cx.o4[m]) && ckc_spec::replay::arg(t, q + 1)
        == Term::Var(cx.o5[m])
}

// A goal with no arithmetic literal under its control nodes and no
// placeholder.
pub open spec fn pure(cx: Cx, t: Term) -> bool
    decreases t,
{
    &&& forall|x: nat| #[trigger] occurs(x, t) ==> !is_ph(cx, x)
    &&& match t {
        Term::Comp(name, args) => if name == comma_name() && args.len() == 2 {
            pure(cx, args[0]) && pure(cx, args[1])
        } else if name == naf_name() && args.len() == 1 {
            pure(cx, args[0])
        } else {
            !ws::is_arith_lit(t)
        },
        _ => true,
    }
}

pub open spec fn si(cx: Cx, fr: Fr) -> bool {
    forall|i: int|
        0 <= i < seg(fr.stack).len() ==> #[trigger] goal_si(
            cx,
            seg(fr.stack)[i],
            fr.k + arank(seg(fr.stack), i),
        )
}

pub open spec fn goal_si(cx: Cx, g: Goal, kk: nat) -> bool {
    match g {
        Goal::Lit(t, _) => if ws::is_arith_lit(t) {
            node_shape(cx, t, kk)
        } else {
            pure(cx, t)
        },
        Goal::NafCut(_) => true,
    }
}

// The instantiation: slots and placeholders alone; a bound placeholder
// carries its node's value at t and at u; every node before k has both
// outputs bound.
pub open spec fn th_vals(cx: Cx, fr: Fr) -> bool {
    &&& forall|x: nat| #[trigger]
        th_val(fr.tht, x) is Some ==> th_val(cx.st, x) is Some || is_ph(cx, x)
    &&& forall|m: int|
        0 <= m < cx.e.chain.len() && #[trigger] th_val(fr.tht, cx.o5[m]) is Some ==> th_val(
            fr.tht,
            cx.o5[m],
        ) == ckc_spec::emit::lookup(cx.vt, ckc_spec::replay::arg(cx.e.chain[m], 5)) && th_val(
            fr.thu,
            cx.o5[m],
        ) == ckc_spec::emit::lookup(cx.vu, ckc_spec::replay::arg(cx.e.chain[m], 5))
    &&& forall|m: int|
        0 <= m < cx.e.chain.len() && #[trigger] th_val(fr.tht, cx.o4[m]) is Some ==> th_val(
            fr.tht,
            cx.o4[m],
        ) == ckc_spec::emit::lookup(cx.vt, ckc_spec::replay::arg(cx.e.chain[m], 4)) && th_val(
            fr.thu,
            cx.o4[m],
        ) == ckc_spec::emit::lookup(cx.vu, ckc_spec::replay::arg(cx.e.chain[m], 4))
    &&& forall|m: int|
        0 <= m < fr.k ==> #[trigger] th_val(fr.tht, cx.o5[m]) is Some && th_val(
            fr.tht,
            cx.o4[m],
        ) is Some
}

pub ghost struct Fr {
    pub stack: Seq<Goal>,
    pub tht: Seq<(nat, Term)>,
    pub thu: Seq<(nat, Term)>,
    pub k: nat,
    pub n: nat,
}

// A body frame the walk accepted, rendered alike at t and u.
pub open spec fn fr_ok(cx: Cx, fr: Fr) -> bool {
    &&& th_ok(fr.tht)
    &&& th_ok(fr.thu)
    &&& forall|x: nat| #[trigger] th_val(fr.tht, x) is Some <==> th_val(fr.thu, x) is Some
    &&& forall|x: nat| #[trigger] th_val(fr.tht, x) is Some ==> x < cx.e.off
    &&& forall|x: nat| #[trigger]
        th_val(cx.st, x) is Some ==> th_val(fr.tht, x) == th_val(cx.st, x) && th_val(fr.thu, x)
            == th_val(cx.su, x)
    &&& crate::m6_fragment::structured_upto(enc(seg(fr.stack)), seg(fr.stack).len() as int)
    &&& ws::walk_ok(cx.e, enc(seg(fr.stack)), fr.n, fr.k)
    &&& si(cx, fr)
    &&& th_vals(cx, fr)
}

pub open spec fn alt_rel(cx: Cx, at: Alt, au: Alt, fr: Fr) -> bool {
    &&& srel(at.stack, fr.stack, fr.tht, cx.b, cx.d)
    &&& srel(au.stack, fr.stack, fr.thu, cx.b, cx.d)
    &&& fr_ok(cx, fr)
    &&& at.sol.len() == 0
    &&& au.sol.len() == 0
    &&& at.fresh == cx.fresh
    &&& au.fresh == cx.fresh
    &&& at.ci == au.ci
    &&& at.naf == au.naf
}

// The head call's choicepoint: the head goal at t and at u from the head's
// fresh counter.
pub open spec fn hd_alt(cx: Cx, at: Alt, au: Alt) -> bool {
    &&& at.stack == seq![Goal::Lit(cx.ht, cx.hdd)]
    &&& au.stack == seq![Goal::Lit(cx.hu, cx.hdd)]
    &&& at.sol.len() == 0
    &&& au.sol.len() == 0
    &&& at.fresh == cx.b
    &&& au.fresh == cx.b
    &&& at.ci == au.ci
    &&& at.naf is None
    &&& au.naf is None
}

// Alternative i: the head call's at index 0 (hd), a walk frame's elsewhere.
pub open spec fn alt_ok(cx: Cx, at: Alt, au: Alt, fr: Fr, i: int) -> bool {
    if i == 0 && cx.hd {
        hd_alt(cx, at, au)
    } else {
        alt_rel(cx, at, au, fr)
    }
}

// Alternative i: its stack's naf cuts stay below its own level.
pub open spec fn alts_rel(cx: Cx, at: Seq<Alt>, au: Seq<Alt>, alts: Seq<Fr>) -> bool {
    &&& at.len() == alts.len()
    &&& au.len() == alts.len()
    &&& forall|i: int|
        0 <= i < alts.len() ==> #[trigger] alt_ok(cx, at[i], au[i], alts[i], i) && cuts_ok(
            alts[i].stack,
            i as nat,
        )
}

pub open spec fn rel(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>) -> bool {
    &&& srel(ct.stack, cur.stack, cur.tht, cx.b, cx.d)
    &&& srel(cu.stack, cur.stack, cur.thu, cx.b, cx.d)
    &&& fr_ok(cx, cur)
    &&& cuts_ok(cur.stack, alts.len())
    &&& alts_rel(cx, ct.alts, cu.alts, alts)
    &&& ct.sol.len() == 0
    &&& cu.sol.len() == 0
    &&& ct.fresh == cx.fresh
    &&& cu.fresh == cx.fresh
    &&& ct.ci == cu.ci
    &&& ct.pruned == cu.pruned
    &&& !ct.collect
    &&& !cu.collect
    &&& ct.rows.len() == 0
    &&& cu.rows.len() == 0
    &&& cx.hd ==> alts.len() > 0
}

// The head call's configuration: the head goal alone, no alternative.
pub open spec fn hrel(cx: Cx, a: Cfg, b: Cfg) -> bool {
    &&& cx.hd
    &&& a.stack == seq![Goal::Lit(cx.ht, cx.hdd)]
    &&& b.stack == seq![Goal::Lit(cx.hu, cx.hdd)]
    &&& a.alts.len() == 0
    &&& b.alts.len() == 0
    &&& a.sol.len() == 0
    &&& b.sol.len() == 0
    &&& a.fresh == cx.b
    &&& b.fresh == cx.b
    &&& a.ci == b.ci
    &&& a.pruned == b.pruned
    &&& !a.collect
    &&& !b.collect
    &&& a.rows.len() == 0
    &&& b.rows.len() == 0
}

// Two steps in lockstep: both continue related, both end alike.
pub open spec fn next_rel(cx: Cx, a: Cfg, b: Cfg) -> bool {
    ||| exists|cur: Fr, alts: Seq<Fr>| #[trigger] rel(cx, a, b, cur, alts)
    ||| hrel(cx, a, b)
}

pub open spec fn step_rel(cx: Cx, st: Step, su: Step) -> bool {
    ||| st is Next && su is Next && next_rel(cx, st->Next_0, su->Next_0)
    ||| st is Sol && su is Sol
    ||| st is Done && su is Done && st->Done_0.pruned == su->Done_0.pruned && st->Done_0.rows.len()
        == 0 && su->Done_0.rows.len() == 0
    ||| st is Stuck && su is Stuck
}

pub proof fn step_next(cx: Cx, a: Cfg, b: Cfg, cur: Fr, alts: Seq<Fr>)
    requires
        rel(cx, a, b, cur, alts),
    ensures
        step_rel(cx, Step::Next(a), Step::Next(b)),
{
    assert(next_rel(cx, a, b)) by {
        assert(rel(cx, a, b, cur, alts));
    }
    assert(Step::Next(a)->Next_0 == a);
    assert(Step::Next(b)->Next_0 == b);
}

pub proof fn alts_at(cx: Cx, at: Seq<Alt>, au: Seq<Alt>, alts: Seq<Fr>, i: int)
    requires
        alts_rel(cx, at, au, alts),
        0 <= i < alts.len(),
    ensures
        alt_ok(cx, at[i], au[i], alts[i], i),
        cuts_ok(alts[i].stack, i as nat),
{
    let ii = i;
    assert(0 <= ii < alts.len() ==> alt_ok(cx, at[ii], au[ii], alts[ii], ii) && cuts_ok(
        alts[ii].stack,
        ii as nat,
    ));
}

// What backtracking reads: the alternatives + the run's flags.
pub open spec fn back_rel(cx: Cx, ct: Cfg, cu: Cfg, alts: Seq<Fr>) -> bool {
    &&& alts_rel(cx, ct.alts, cu.alts, alts)
    &&& ct.pruned == cu.pruned
    &&& !ct.collect
    &&& !cu.collect
    &&& ct.rows.len() == 0
    &&& cu.rows.len() == 0
}

// Backtracking resumes the related newest choicepoints (or ends alike).
pub proof fn fail_rel(cx: Cx, ct: Cfg, cu: Cfg, alts: Seq<Fr>)
    requires
        back_rel(cx, ct, cu, alts),
    ensures
        step_rel(cx, fail(ct), fail(cu)),
{
    if alts.len() > 0 {
        let i = alts.len() - 1;
        alts_at(cx, ct.alts, cu.alts, alts, i);
        let at = ct.alts.last();
        let au = cu.alts.last();
        let fr = alts.last();
        let rest = alts.drop_last();
        if i == 0 && cx.hd {
            // the head call's choicepoint: back to the head goal
            assert(hd_alt(cx, at, au));
            let nt = Cfg {
                stack: at.stack,
                sol: at.sol,
                fresh: at.fresh,
                ci: at.ci,
                alts: ct.alts.drop_last(),
                ..ct
            };
            let nu = Cfg {
                stack: au.stack,
                sol: au.sol,
                fresh: au.fresh,
                ci: au.ci,
                alts: cu.alts.drop_last(),
                ..cu
            };
            assert(fail(ct) == Step::Next(nt));
            assert(fail(cu) == Step::Next(nu));
            assert(hrel(cx, nt, nu));
            assert(next_rel(cx, nt, nu));
        } else {
            assert(alt_rel(cx, at, au, fr));
            assert forall|j: int| 0 <= j < rest.len() implies #[trigger] alt_ok(
                cx,
                ct.alts.drop_last()[j],
                cu.alts.drop_last()[j],
                rest[j],
                j,
            ) && cuts_ok(rest[j].stack, j as nat) by {
                alts_at(cx, ct.alts, cu.alts, alts, j);
            }
            match at.naf {
                Option::Some(entry) => {
                    if !ct.pruned {
                        let nt = Cfg {
                            stack: at.stack,
                            sol: at.sol,
                            fresh: at.fresh,
                            ci: at.ci,
                            alts: ct.alts.drop_last(),
                            pruned: entry,
                            ..ct
                        };
                        let nu = Cfg {
                            stack: au.stack,
                            sol: au.sol,
                            fresh: au.fresh,
                            ci: au.ci,
                            alts: cu.alts.drop_last(),
                            pruned: entry,
                            ..cu
                        };
                        assert(rel(cx, nt, nu, fr, rest));
                        assert(fail(ct) == Step::Next(nt));
                        assert(fail(cu) == Step::Next(nu));
                        step_next(cx, nt, nu, fr, rest);
                    } else {
                        assert(fail(ct) == Step::Done(ct));
                        assert(fail(cu) == Step::Done(cu));
                    }
                },
                Option::None => {
                    let nt = Cfg {
                        stack: at.stack,
                        sol: at.sol,
                        fresh: at.fresh,
                        ci: at.ci,
                        alts: ct.alts.drop_last(),
                        ..ct
                    };
                    let nu = Cfg {
                        stack: au.stack,
                        sol: au.sol,
                        fresh: au.fresh,
                        ci: au.ci,
                        alts: cu.alts.drop_last(),
                        ..cu
                    };
                    assert(rel(cx, nt, nu, fr, rest));
                    assert(fail(ct) == Step::Next(nt));
                    assert(fail(cu) == Step::Next(nu));
                    step_next(cx, nt, nu, fr, rest);
                },
            }
        }
    } else {
        assert(fail(ct) == Step::Done(ct));
        assert(fail(cu) == Step::Done(cu));
    }
}

// --- small facts ---
pub proof fn rel_root(c: Term, a: Term, th: Seq<(nat, Term)>, b: nat, d: nat)
    requires
        crate::k3_sound::naf::rename(c, b, d) == inst(a, th),
        a is Comp,
    ensures
        c is Comp,
        c->Comp_0 == a->Comp_0,
        c->Comp_1.len() == a->Comp_1.len(),
        forall|i: int|
            0 <= i < c->Comp_1.len() ==> crate::k3_sound::naf::rename(#[trigger] c->Comp_1[i], b, d)
                == inst(a->Comp_1[i], th),
{
    let xs = a->Comp_1;
    assert(inst(a, th) == Term::Comp(a->Comp_0, inst_all(xs, th)));
    inst_all_index(xs, th);
    match c {
        Term::Comp(m, ys) => {
            crate::k3_sound::naf::rename_all_index(ys, b, d);
            assert(crate::k3_sound::naf::rename(c, b, d) == Term::Comp(
                m,
                crate::k3_sound::naf::rename_all(ys, b, d),
            ));
            assert(crate::k3_sound::naf::rename_all(ys, b, d) == inst_all(xs, th));
            assert forall|i: int| 0 <= i < ys.len() implies crate::k3_sound::naf::rename(
                #[trigger] ys[i],
                b,
                d,
            ) == inst(xs[i], th) by {
                assert(crate::k3_sound::naf::rename_all(ys, b, d)[i] == inst_all(xs, th)[i]);
            }
        },
        _ => {},
    }
}

pub proof fn names_distinct()
    ensures
        comma_name() != operator_name(),
        naf_name() != operator_name(),
        comma_name() != naf_name(),
{
    reveal_strlit(",");
    reveal_strlit("\\+");
    reveal_strlit("guideline_operator");
    assert(comma_name().len() == 1);
    assert(naf_name().len() == 2);
    assert(operator_name().len() == 18);
}

// A first-segment goal of a frame the walk accepted is no operator literal.
pub proof fn walk_not_operator(cx: Cx, cur: Fr)
    requires
        fr_ok(cx, cur),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
    ensures
        cur.stack[0]->Lit_0 is Comp,
        cur.stack[0]->Lit_0->Comp_0 != operator_name(),
{
    let ga = cur.stack[0]->Lit_0;
    let ra = cur.stack.drop_first();
    assert(cur.stack =~= seq![cur.stack[0]] + ra);
    seg_cons(cur.stack[0], ra);
    let ts1 = enc(seg(cur.stack));
    assert(ts1 =~= seq![ga] + enc(seg(ra)));
    assert(crate::m6_fragment::structured(ga));
    names_distinct();
    reveal_strlit("guideline_satisfies");
    reveal_strlit("guideline_product");
    reveal_strlit("guideline_quotient");
    reveal_strlit("guideline_operator");
    reveal(ckc_spec::compare::is_builtin);
    reveal(ckc_spec::arith::is_arith);
    reveal_with_fuel(ws::walk_ok, 1);
    assert(ts1[0] == ga);
    if ga->Comp_0 == operator_name() {
        assert(operator_name().len() == 18);
        assert(ckc_spec::compare::satisfies_name().len() == 19);
        assert(ckc_spec::arith::product_name().len() == 17);
        assert(ckc_spec::arith::quotient_name().len() == 18);
        assert(ckc_spec::arith::quotient_name()[10] != operator_name()[10]);
        assert(!ckc_spec::compare::is_builtin(ga->Comp_0, ga->Comp_1.len()));
    }
}

pub proof fn selected_zero(s: Seq<Goal>)
    requires
        s.len() > 0,
        match s[0] {
            Goal::Lit(t, _) => !(t is Comp && t->Comp_0 == operator_name()),
            Goal::NafCut(_) => true,
        },
    ensures
        selected(s) == 0,
{
    if s.len() > 1 {
        assert(!waits(s[0], s.drop_first()));
    }
}

pub proof fn arank_cons(g: Goal, s: Seq<Goal>, i: int)
    requires
        0 <= i <= s.len(),
    ensures
        arank(seq![g] + s, i + 1) == arank(s, i) + if g is Lit && ws::is_arith_lit(g->Lit_0) {
            1nat
        } else {
            0nat
        },
    decreases i,
{
    let t = seq![g] + s;
    if i > 0 {
        arank_cons(g, s, i - 1);
        assert(t[i] == s[i - 1]);
    } else {
        assert(t[0] == g);
        reveal_with_fuel(arank, 2);
    }
}

pub proof fn seg_cons(g: Goal, s: Seq<Goal>)
    requires
        g is Lit,
    ensures
        seg(seq![g] + s) == seq![g] + seg(s),
{
    let t = seq![g] + s;
    assert(t[0] == g);
    assert(t.drop_first() =~= s);
}

// No naf cut beyond the alternatives that can absorb it.
pub open spec fn cuts_ok(s: Seq<Goal>, n: nat) -> bool {
    forall|j: int| 0 <= j < s.len() && (#[trigger] s[j]) is NafCut ==> s[j]->NafCut_0 < n
}

// The first goal of a related stack: its concrete renderings and the walk.
pub proof fn first_lit(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>)
    requires
        rel(cx, ct, cu, cur, alts),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        cur.stack[0]->Lit_0 is Comp,
    ensures
        ct.stack.len() == cur.stack.len(),
        ct.stack[0] is Lit && cu.stack[0] is Lit,
        ct.stack[0]->Lit_1 == cur.stack[0]->Lit_1 && cu.stack[0]->Lit_1 == cur.stack[0]->Lit_1,
        ({
            let ga = cur.stack[0]->Lit_0;
            let gt = ct.stack[0]->Lit_0;
            let gu = cu.stack[0]->Lit_0;
            &&& gt is Comp && gu is Comp
            &&& gt->Comp_0 == ga->Comp_0 && gu->Comp_0 == ga->Comp_0
            &&& gt->Comp_1.len() == ga->Comp_1.len() && gu->Comp_1.len() == ga->Comp_1.len()
            &&& forall|i: int|
                0 <= i < ga->Comp_1.len() ==> crate::k3_sound::naf::rename(
                    #[trigger] gt->Comp_1[i],
                    cx.b,
                    cx.d,
                ) == inst(ga->Comp_1[i], cur.tht) && crate::k3_sound::naf::rename(
                    gu->Comp_1[i],
                    cx.b,
                    cx.d,
                ) == inst(ga->Comp_1[i], cur.thu)
        }),
        seg(cur.stack) == seq![cur.stack[0]] + seg(cur.stack.drop_first()),
{
    assert(grel(ct.stack[0], cur.stack[0], cur.tht, cx.b, cx.d));
    assert(grel(cu.stack[0], cur.stack[0], cur.thu, cx.b, cx.d));
    let ga = cur.stack[0]->Lit_0;
    rel_root(ct.stack[0]->Lit_0, ga, cur.tht, cx.b, cx.d);
    rel_root(cu.stack[0]->Lit_0, ga, cur.thu, cx.b, cx.d);
    assert(cur.stack =~= seq![cur.stack[0]] + cur.stack.drop_first());
    seg_cons(cur.stack[0], cur.stack.drop_first());
}

// The rest of a related stack stays related.
pub proof fn srel_tail(cs: Seq<Goal>, as_: Seq<Goal>, th: Seq<(nat, Term)>, b: nat, d: nat, k: int)
    requires
        srel(cs, as_, th, b, d),
        0 <= k <= cs.len(),
    ensures
        srel(cs.skip(k), as_.skip(k), th, b, d),
{
    assert forall|i: int| 0 <= i < cs.skip(k).len() implies #[trigger] grel(
        cs.skip(k)[i],
        as_.skip(k)[i],
        th,
        b,
        d,
    ) by {
        assert(grel(cs[k + i], as_[k + i], th, b, d));
    }
}

pub proof fn srel_prepend(
    c: Seq<Goal>,
    a: Seq<Goal>,
    cs: Seq<Goal>,
    as_: Seq<Goal>,
    th: Seq<(nat, Term)>,
    b: nat,
    d: nat,
)
    requires
        srel(c, a, th, b, d),
        srel(cs, as_, th, b, d),
    ensures
        srel(c + cs, a + as_, th, b, d),
{
    assert forall|i: int| 0 <= i < (c + cs).len() implies #[trigger] grel(
        (c + cs)[i],
        (a + as_)[i],
        th,
        b,
        d,
    ) by {
        if i < c.len() {
            assert(grel(c[i], a[i], th, b, d));
        } else {
            assert(grel(cs[i - c.len()], as_[i - c.len()], th, b, d));
        }
    }
}

// A frame whose first segment begins with ga, rebuilt with gs in its place
// (the walk's fuel n2): the shared obligations other than the walk.
pub proof fn si_replace(cx: Cx, cur: Fr, gs: Seq<Goal>, fr2: Fr)
    requires
        si(cx, cur),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        !ws::is_arith_lit(cur.stack[0]->Lit_0),
        fr2.stack == gs + cur.stack.drop_first(),
        fr2.k == cur.k,
        forall|i: int| 0 <= i < gs.len() ==> (#[trigger] gs[i]) is Lit && pure(cx, gs[i]->Lit_0),
    ensures
        si(cx, fr2),
        seg(fr2.stack) == gs + seg(cur.stack.drop_first()),
    decreases gs.len(),
{
    let rest = cur.stack.drop_first();
    assert(cur.stack =~= seq![cur.stack[0]] + rest);
    seg_cons(cur.stack[0], rest);
    seg_prepend(gs, rest);
    let s1 = seg(cur.stack);
    let s2 = seg(fr2.stack);
    assert forall|i: int| 0 <= i < s2.len() implies #[trigger] goal_si(
        cx,
        s2[i],
        fr2.k + arank(s2, i),
    ) by {
        if i < gs.len() {
            assert(s2[i] == gs[i]);
            assert(pure(cx, gs[i]->Lit_0));
            pure_not_arith(cx, gs[i]->Lit_0);
        } else {
            let j = i - gs.len();
            assert(s2[i] == seg(rest)[j]);
            assert(s1[j + 1] == seg(rest)[j]);
            arank_prepend(gs, seg(rest), j, cx);
            arank_cons(cur.stack[0], seg(rest), j);
            assert(goal_si(cx, s1[j + 1], cur.k + arank(s1, j + 1)));
        }
    }
}

pub proof fn pure_not_arith(cx: Cx, t: Term)
    requires
        pure(cx, t),
    ensures
        !ws::is_arith_lit(t),
{
    if let Term::Comp(name, args) = t {
        if (name == comma_name() && args.len() == 2) || (name == naf_name() && args.len() == 1) {
            assert(!ckc_spec::arith::is_arith(name, args.len()));
        }
    }
}

pub proof fn seg_prepend(gs: Seq<Goal>, s: Seq<Goal>)
    requires
        forall|i: int| 0 <= i < gs.len() ==> (#[trigger] gs[i]) is Lit,
    ensures
        seg(gs + s) == gs + seg(s),
    decreases gs.len(),
{
    if gs.len() > 0 {
        let t = gs.drop_first();
        assert forall|i: int| 0 <= i < t.len() implies (#[trigger] t[i]) is Lit by {
            assert(t[i] == gs[i + 1]);
        }
        seg_prepend(t, s);
        assert(gs + s =~= seq![gs[0]] + (t + s));
        seg_cons(gs[0], t + s);
        assert(gs =~= seq![gs[0]] + t);
    } else {
        assert(gs + s =~= s);
    }
}

// Prepending non-arithmetic goals shifts no arithmetic rank.
pub proof fn arank_prepend(gs: Seq<Goal>, s: Seq<Goal>, j: int, cx: Cx)
    requires
        0 <= j <= s.len(),
        forall|i: int| 0 <= i < gs.len() ==> (#[trigger] gs[i]) is Lit && pure(cx, gs[i]->Lit_0),
    ensures
        arank(gs + s, gs.len() + j) == arank(s, j),
    decreases gs.len(),
{
    if gs.len() > 0 {
        let t = gs.drop_first();
        assert forall|i: int| 0 <= i < t.len() implies (#[trigger] t[i]) is Lit && pure(
            cx,
            t[i]->Lit_0,
        ) by {
            assert(t[i] == gs[i + 1]);
        }
        arank_prepend(t, s, j, cx);
        assert(gs + s =~= seq![gs[0]] + (t + s));
        arank_cons(gs[0], t + s, t.len() + j);
        pure_not_arith(cx, gs[0]->Lit_0);
    } else {
        assert(gs + s =~= s);
    }
}

pub open spec fn is_conj(t: Term) -> bool {
    t is Comp && t->Comp_0 == comma_name() && t->Comp_1.len() == 2
}

pub proof fn step_comma(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>)
    requires
        rel(cx, ct, cu, cur, alts),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        cur.stack[0]->Lit_1 > 0,
        is_conj(cur.stack[0]->Lit_0),
    ensures
        step_rel(cx, step(cx.dbt, ct), step(cx.dbu, cu)),
{
    let ga = cur.stack[0]->Lit_0;
    let dd = cur.stack[0]->Lit_1;
    first_lit(cx, ct, cu, cur, alts);
    let gt = ct.stack[0]->Lit_0;
    let gu = cu.stack[0]->Lit_0;
    let a0 = ga->Comp_1[0];
    let a1 = ga->Comp_1[1];
    names_distinct();
    selected_zero(ct.stack);
    selected_zero(cu.stack);
    let rt = ct.stack.skip(1);
    let ru = cu.stack.skip(1);
    let ra = cur.stack.drop_first();
    let nt = Cfg {
        stack: Seq::<Goal>::empty() + seq![
            Goal::Lit(gt->Comp_1[0], dd),
            Goal::Lit(gt->Comp_1[1], dd),
        ] + rt,
        ..ct
    };
    let nu = Cfg {
        stack: Seq::<Goal>::empty() + seq![
            Goal::Lit(gu->Comp_1[0], dd),
            Goal::Lit(gu->Comp_1[1], dd),
        ] + ru,
        ..cu
    };
    assert(ct.stack.take(0) =~= Seq::<Goal>::empty());
    assert(cu.stack.take(0) =~= Seq::<Goal>::empty());
    assert(step(cx.dbt, ct) == Step::Next(nt));
    assert(step(cx.dbu, cu) == Step::Next(nu));
    let gs = seq![Goal::Lit(a0, dd), Goal::Lit(a1, dd)];
    let fr2 = Fr { stack: gs + ra, n: (cur.n - 1) as nat, ..cur };
    // the stacks
    srel_tail(ct.stack, cur.stack, cur.tht, cx.b, cx.d, 1);
    srel_tail(cu.stack, cur.stack, cur.thu, cx.b, cx.d, 1);
    assert(cur.stack.skip(1) =~= ra);
    let ct2 = seq![Goal::Lit(gt->Comp_1[0], dd), Goal::Lit(gt->Comp_1[1], dd)];
    let cu2 = seq![Goal::Lit(gu->Comp_1[0], dd), Goal::Lit(gu->Comp_1[1], dd)];
    assert(srel(ct2, gs, cur.tht, cx.b, cx.d)) by {
        assert(grel(ct2[0], gs[0], cur.tht, cx.b, cx.d));
        assert(grel(ct2[1], gs[1], cur.tht, cx.b, cx.d));
    }
    assert(srel(cu2, gs, cur.thu, cx.b, cx.d)) by {
        assert(grel(cu2[0], gs[0], cur.thu, cx.b, cx.d));
        assert(grel(cu2[1], gs[1], cur.thu, cx.b, cx.d));
    }
    srel_prepend(ct2, gs, rt, ra, cur.tht, cx.b, cx.d);
    srel_prepend(cu2, gs, ru, ra, cur.thu, cx.b, cx.d);
    assert(nt.stack =~= ct2 + rt);
    assert(nu.stack =~= cu2 + ru);
    // the walk and the shape invariants
    let ts1 = enc(seg(cur.stack));
    assert(ts1 == seq![ga] + enc(seg(ra))) by {
        assert(enc(seq![cur.stack[0]] + seg(ra)) =~= seq![ga] + enc(seg(ra)));
    }
    assert(ws::walk_ok(cx.e, ts1, cur.n, cur.k));
    reveal_with_fuel(ws::walk_ok, 1);
    assert(ts1[0] == ga && ts1.drop_first() =~= enc(seg(ra)));
    assert(cur.n > 0);
    assert(ws::walk_ok(cx.e, seq![a0, a1] + enc(seg(ra)), (cur.n - 1) as nat, cur.k));
    assert(goal_si(cx, seg(cur.stack)[0], cur.k + arank(seg(cur.stack), 0)));
    assert(pure(cx, ga));
    assert(pure(cx, a0) && pure(cx, a1));
    assert forall|i: int| 0 <= i < gs.len() implies (#[trigger] gs[i]) is Lit && pure(
        cx,
        gs[i]->Lit_0,
    ) by {}
    si_replace(cx, cur, gs, fr2);
    assert(enc(seg(fr2.stack)) =~= seq![a0, a1] + enc(seg(ra))) by {
        assert(seg(fr2.stack) == gs + seg(ra));
    }
    assert(crate::m6_fragment::structured_upto(enc(seg(fr2.stack)), seg(fr2.stack).len() as int))
        by {
        assert(crate::m6_fragment::structured(ga));
        assert forall|i: int| 0 <= i < seg(fr2.stack).len() implies crate::m6_fragment::structured(
            #[trigger] enc(seg(fr2.stack))[i],
        ) by {
            if i >= 2 {
                assert(enc(seg(fr2.stack))[i] == enc(seg(ra))[i - 2]);
                assert(enc(seg(ra))[i - 2] == ts1[i - 1]);
            }
        }
    }
    assert(cuts_ok(fr2.stack, alts.len())) by {
        assert forall|j: int|
            0 <= j < fr2.stack.len() && (
            #[trigger] fr2.stack[j]) is NafCut implies fr2.stack[j]->NafCut_0 < alts.len() by {
            assert(j >= 2);
            assert(fr2.stack[j] == cur.stack[j - 1]);
        }
    }
    assert(rel(cx, nt, nu, fr2, alts));
    step_next(cx, nt, nu, fr2, alts);
}

pub open spec fn is_neg(t: Term) -> bool {
    t is Comp && t->Comp_0 == naf_name() && t->Comp_1.len() == 1
}

// The walk on a first goal that is a negation: its inner goal and the rest.
proof fn walk_neg(cx: Cx, cur: Fr)
    requires
        fr_ok(cx, cur),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        is_neg(cur.stack[0]->Lit_0),
    ensures
        cur.n > 0,
        ws::walk_ok(cx.e, seq![cur.stack[0]->Lit_0->Comp_1[0]], (cur.n - 1) as nat, cur.k),
        ws::walk_ok(cx.e, enc(seg(cur.stack.drop_first())), (cur.n - 1) as nat, cur.k),
        seg(cur.stack) == seq![cur.stack[0]] + seg(cur.stack.drop_first()),
{
    let ga = cur.stack[0]->Lit_0;
    let ra = cur.stack.drop_first();
    assert(cur.stack =~= seq![cur.stack[0]] + ra);
    seg_cons(cur.stack[0], ra);
    let ts1 = enc(seg(cur.stack));
    assert(ts1 =~= seq![ga] + enc(seg(ra)));
    reveal_with_fuel(ws::walk_ok, 1);
    assert(ts1[0] == ga && ts1.drop_first() =~= enc(seg(ra)));
}

pub proof fn step_neg(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>)
    requires
        rel(cx, ct, cu, cur, alts),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        cur.stack[0]->Lit_1 > 0,
        is_neg(cur.stack[0]->Lit_0),
    ensures
        step_rel(cx, step(cx.dbt, ct), step(cx.dbu, cu)),
{
    let ga = cur.stack[0]->Lit_0;
    let dd = cur.stack[0]->Lit_1;
    let a0 = ga->Comp_1[0];
    first_lit(cx, ct, cu, cur, alts);
    walk_neg(cx, cur);
    let gt = ct.stack[0]->Lit_0;
    let gu = cu.stack[0]->Lit_0;
    names_distinct();
    selected_zero(ct.stack);
    selected_zero(cu.stack);
    let rt = ct.stack.skip(1);
    let ru = cu.stack.skip(1);
    let ra = cur.stack.drop_first();
    let l = ct.alts.len();
    let nt = Cfg {
        alts: ct.alts.push(
            Alt { stack: rt, sol: ct.sol, fresh: ct.fresh, ci: 0, naf: Option::Some(ct.pruned) },
        ),
        stack: seq![Goal::Lit(gt->Comp_1[0], (dd - 1) as nat), Goal::NafCut(l)] + rt,
        pruned: false,
        ..ct
    };
    let nu = Cfg {
        alts: cu.alts.push(
            Alt { stack: ru, sol: cu.sol, fresh: cu.fresh, ci: 0, naf: Option::Some(cu.pruned) },
        ),
        stack: seq![Goal::Lit(gu->Comp_1[0], (dd - 1) as nat), Goal::NafCut(l)] + ru,
        pruned: false,
        ..cu
    };
    assert(ct.stack.take(0) =~= Seq::<Goal>::empty());
    assert(cu.stack.take(0) =~= Seq::<Goal>::empty());
    assert(ct.stack.skip(1) =~= rt);
    assert(step(cx.dbt, ct) == Step::Next(nt));
    assert(step(cx.dbu, cu) == Step::Next(nu));
    // frames: the inner goal, then the continuation as a choicepoint
    let gs = seq![Goal::Lit(a0, (dd - 1) as nat), Goal::NafCut(l)];
    let fr2 = Fr { stack: gs + ra, n: (cur.n - 1) as nat, ..cur };
    let fa = Fr { stack: ra, n: (cur.n - 1) as nat, ..cur };
    let alts2 = alts.push(fa);
    srel_tail(ct.stack, cur.stack, cur.tht, cx.b, cx.d, 1);
    srel_tail(cu.stack, cur.stack, cur.thu, cx.b, cx.d, 1);
    assert(cur.stack.skip(1) =~= ra);
    let ct2 = seq![Goal::Lit(gt->Comp_1[0], (dd - 1) as nat), Goal::NafCut(l)];
    let cu2 = seq![Goal::Lit(gu->Comp_1[0], (dd - 1) as nat), Goal::NafCut(l)];
    assert(srel(ct2, gs, cur.tht, cx.b, cx.d)) by {
        assert(grel(ct2[0], gs[0], cur.tht, cx.b, cx.d));
        assert(grel(ct2[1], gs[1], cur.tht, cx.b, cx.d));
    }
    assert(srel(cu2, gs, cur.thu, cx.b, cx.d)) by {
        assert(grel(cu2[0], gs[0], cur.thu, cx.b, cx.d));
        assert(grel(cu2[1], gs[1], cur.thu, cx.b, cx.d));
    }
    srel_prepend(ct2, gs, rt, ra, cur.tht, cx.b, cx.d);
    srel_prepend(cu2, gs, ru, ra, cur.thu, cx.b, cx.d);
    assert(nt.stack =~= ct2 + rt);
    assert(nu.stack =~= cu2 + ru);
    // the inner frame: one goal before its cut
    assert(seg(fr2.stack) =~= seq![Goal::Lit(a0, (dd - 1) as nat)]) by {
        assert(fr2.stack =~= seq![gs[0]] + (seq![gs[1]] + ra));
        seg_cons(gs[0], seq![gs[1]] + ra);
        assert((seq![gs[1]] + ra)[0] == gs[1]);
    }
    assert(enc(seg(fr2.stack)) =~= seq![a0]);
    assert(goal_si(cx, seg(cur.stack)[0], cur.k + arank(seg(cur.stack), 0)));
    assert(pure(cx, ga));
    assert(pure(cx, a0));
    pure_not_arith(cx, a0);
    assert(si(cx, fr2)) by {
        assert forall|i: int| 0 <= i < seg(fr2.stack).len() implies #[trigger] goal_si(
            cx,
            seg(fr2.stack)[i],
            fr2.k + arank(seg(fr2.stack), i),
        ) by {
            assert(i == 0);
        }
    }
    assert(crate::m6_fragment::structured(ga));
    assert(crate::m6_fragment::structured_upto(enc(seg(fr2.stack)), 1));
    // the continuation frame
    si_replace(cx, cur, Seq::<Goal>::empty(), fa);
    assert(Seq::<Goal>::empty() + ra =~= ra);
    assert(crate::m6_fragment::structured_upto(enc(seg(ra)), seg(ra).len() as int)) by {
        assert forall|i: int| 0 <= i < seg(ra).len() implies crate::m6_fragment::structured(
            #[trigger] enc(seg(ra))[i],
        ) by {
            assert(enc(seg(cur.stack))[i + 1] == enc(seg(ra))[i]);
        }
    }
    assert(fr_ok(cx, fa));
    assert(fr_ok(cx, fr2));
    // naf cuts + alternatives
    assert(cuts_ok(fr2.stack, alts2.len())) by {
        assert forall|j: int|
            0 <= j < fr2.stack.len() && (
            #[trigger] fr2.stack[j]) is NafCut implies fr2.stack[j]->NafCut_0 < alts2.len() by {
            if j >= 2 {
                assert(fr2.stack[j] == cur.stack[j - 1]);
            }
        }
    }
    assert(alts_rel(cx, nt.alts, nu.alts, alts2)) by {
        assert forall|i: int| 0 <= i < alts2.len() implies #[trigger] alt_ok(
            cx,
            nt.alts[i],
            nu.alts[i],
            alts2[i],
            i,
        ) && cuts_ok(alts2[i].stack, i as nat) by {
            if i < alts.len() {
                alts_at(cx, ct.alts, cu.alts, alts, i);
            } else {
                assert(cuts_ok(ra, l)) by {
                    assert forall|j: int|
                        0 <= j < ra.len() && (#[trigger] ra[j]) is NafCut implies ra[j]->NafCut_0
                        < l by {
                        assert(ra[j] == cur.stack[j + 1]);
                    }
                }
            }
        }
    }
    assert(rel(cx, nt, nu, fr2, alts2));
    step_next(cx, nt, nu, fr2, alts2);
}

pub open spec fn slot_of(cx: Cx, i: int) -> nat {
    cx.e.slots[i]->Var_0
}

// What the lockstep reads of the context.
pub open spec fn cx_ok(cx: Cx) -> bool {
    &&& th_ok(cx.st)
    &&& th_ok(cx.su)
    &&& cx.e.bounds.len() == cx.e.slots.len()
    &&& forall|i: int| 0 <= i < cx.e.slots.len() ==> (#[trigger] cx.e.slots[i]) is Var
    &&& forall|x: nat| #[trigger] th_val(cx.st, x) is Some <==> cx.e.slots.contains(Term::Var(x))
    &&& forall|x: nat| #[trigger] th_val(cx.su, x) is Some <==> cx.e.slots.contains(Term::Var(x))
    &&& forall|i: int|
        0 <= i < cx.e.slots.len() ==> #[trigger] ws::same_cell(
            cx.e.bounds[i],
            th_val(cx.st, slot_of(cx, i))->0,
            th_val(cx.su, slot_of(cx, i))->0,
        )
    &&& forall|x: nat| #[trigger] is_ph(cx, x) ==> th_val(cx.st, x) is None
    &&& forall|i: int, j: int|
        0 <= i < cx.e.bounds.len() && 0 <= j < cx.e.bounds[i].len() ==> ckc_spec::arith::number(
            #[trigger] cx.e.bounds[i][j],
        ) is Some
    &&& db_ok(cx)
    &&& cx.o4.len() == cx.e.chain.len()
    &&& cx.o5.len() == cx.e.chain.len()
}

// The two databases: the witness facts at t and u in front (each an instance
// of a skeleton fact, reversed as asserta leaves them), then the program.
pub open spec fn db_ok(cx: Cx) -> bool {
    &&& cx.nf <= cx.e.facts.len()
    &&& cx.dbt.len() == cx.nf + cx.e.p.len()
    &&& cx.dbu.len() == cx.nf + cx.e.p.len()
    &&& forall|m: int|
        0 <= m < cx.nf ==> #[trigger] cx.dbt[m] == fact_clause(
            inst(cx.e.facts[cx.nf - 1 - m], cx.st),
        ) && cx.dbu[m] == fact_clause(inst(cx.e.facts[cx.nf - 1 - m], cx.su))
    &&& forall|j: int|
        0 <= j < cx.nf ==> (#[trigger] cx.e.facts[j]) is Comp && ground(inst(cx.e.facts[j], cx.st))
            && ground(inst(cx.e.facts[j], cx.su)) && skel_fact(cx, cx.e.facts[j])
    &&& forall|q: int|
        0 <= q < cx.e.p.len() ==> #[trigger] cx.dbt[cx.nf + q] == cx.e.p[q] && cx.dbu[cx.nf + q]
            == cx.e.p[q]
    &&& forall|q: int|
        0 <= q < cx.e.p.len() && ws::ground_fact(#[trigger] cx.e.p[q]) ==> exists|j: int|
            0 <= j < cx.e.facts.len() && cx.e.facts[j] == cx.e.p[q].head
    &&& cx.fresh >= cx.b
    &&& cx.fresh + cx.d == cx.e.off
}

// A skeleton fact holds slots alone.
pub open spec fn skel_fact(cx: Cx, f: Term) -> bool {
    forall|x: nat| #[trigger] occurs(x, f) ==> th_val(cx.st, x) is Some
}

pub proof fn inst_absent(t: Term, th: Seq<(nat, Term)>)
    requires
        forall|x: nat| #[trigger] occurs(x, t) ==> th_val(th, x) is None,
    ensures
        inst(t, th) == t,
    decreases t,
{
    match t {
        Term::Var(x) => {
            assert(occurs(x, t));
        },
        Term::Comp(n, args) => {
            inst_all_index(args, th);
            assert forall|i: int| 0 <= i < args.len() implies inst(#[trigger] args[i], th)
                == args[i] by {
                assert forall|x: nat| #[trigger] occurs(x, args[i]) implies th_val(
                    th,
                    x,
                ) is None by {
                    crate::m6_head::occurs_member(x, args, i);
                    assert(occurs(x, t));
                }
                inst_absent(args[i], th);
            }
            assert(inst_all(args, th) =~= args);
        },
        _ => {},
    }
}

// A frame's instantiation reaches only slots + placeholders.
proof fn inst_pure(cx: Cx, fr: Fr, t: Term)
    requires
        cx_ok(cx),
        fr_ok(cx, fr),
        forall|x: nat| #[trigger] occurs(x, t) ==> !is_ph(cx, x),
        ws::slot_free(cx.e, t),
    ensures
        inst(t, fr.tht) == t,
        inst(t, fr.thu) == t,
{
    assert forall|x: nat| #[trigger] occurs(x, t) implies th_val(fr.tht, x) is None && th_val(
        fr.thu,
        x,
    ) is None by {
        if th_val(fr.tht, x) is Some {
            assert(th_val(cx.st, x) is Some || is_ph(cx, x));
            if th_val(cx.st, x) is Some {
                assert(cx.e.slots.contains(Term::Var(x)));
                let i = choose|i: int| 0 <= i < cx.e.slots.len() && cx.e.slots[i] == Term::Var(x);
                assert(ws::mentions(t, cx.e.slots[i]));
            }
        }
    }
    inst_absent(t, fr.tht);
    inst_absent(t, fr.thu);
}

// The walk on a built-in first goal: the test, then the rest.
proof fn walk_builtin(cx: Cx, cur: Fr)
    requires
        fr_ok(cx, cur),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        cur.stack[0]->Lit_0 is Comp,
        ckc_spec::compare::is_builtin(
            cur.stack[0]->Lit_0->Comp_0,
            cur.stack[0]->Lit_0->Comp_1.len(),
        ),
        !ckc_spec::arith::is_arith(cur.stack[0]->Lit_0->Comp_0, cur.stack[0]->Lit_0->Comp_1.len()),
    ensures
        cur.n > 0,
        ws::test_ok(cx.e, cur.stack[0]->Lit_0),
        ws::walk_ok(cx.e, enc(seg(cur.stack.drop_first())), (cur.n - 1) as nat, cur.k),
{
    let ga = cur.stack[0]->Lit_0;
    let ra = cur.stack.drop_first();
    assert(cur.stack =~= seq![cur.stack[0]] + ra);
    seg_cons(cur.stack[0], ra);
    let ts1 = enc(seg(cur.stack));
    assert(ts1 =~= seq![ga] + enc(seg(ra)));
    names_distinct();
    reveal(ckc_spec::compare::is_builtin);
    reveal(ckc_spec::arith::is_arith);
    reveal_with_fuel(ws::walk_ok, 1);
    assert(ts1[0] == ga && ts1.drop_first() =~= enc(seg(ra)));
    assert(!(ga->Comp_0 == comma_name() && ga->Comp_1.len() == 2));
    assert(!(ga->Comp_0 == naf_name() && ga->Comp_1.len() == 1));
}

// A comparison reads alike at t and u.
proof fn cmp_holds_eq(cx: Cx, cur: Fr, ga: Term, gt: Term, gu: Term)
    requires
        cx_ok(cx),
        fr_ok(cx, cur),
        ws::test_ok(cx.e, ga),
        pure(cx, ga),
        ga is Comp,
        crate::k3_sound::naf::rename(gt, cx.b, cx.d) == inst(ga, cur.tht),
        crate::k3_sound::naf::rename(gu, cx.b, cx.d) == inst(ga, cur.thu),
    ensures
        ckc_spec::compare::holds(gt) == ckc_spec::compare::holds(gu),
{
    if ws::slot_free(cx.e, ga) {
        inst_pure(cx, cur, ga);
        crate::k3_sound::naf::rename_inj(gt, gu, cx.b, cx.d);
    } else {
        let i = choose|i: int|
            0 <= i < cx.e.slots.len() && ckc_spec::emit::is_comp(ga, "guideline_satisfies"@, 4)
                && ckc_spec::replay::arg(ga, 1) == #[trigger] cx.e.slots[i] && ws::slot_free(
                cx.e,
                ckc_spec::replay::arg(ga, 0),
            ) && ws::slot_free(cx.e, ckc_spec::replay::arg(ga, 2)) && cx.e.bounds[i].contains(
                ckc_spec::replay::arg(ga, 3),
            );
        let args = ga->Comp_1;
        let x = slot_of(cx, i);
        assert(cx.e.slots[i] == Term::Var(x));
        let tv = th_val(cx.st, x)->0;
        let uv = th_val(cx.su, x)->0;
        assert(cx.e.slots.contains(Term::Var(x)));
        assert(th_val(cur.tht, x) == Some(tv) && th_val(cur.thu, x) == Some(uv));
        assert(ws::same_cell(cx.e.bounds[i], tv, uv));
        let bnd = args[3];
        let k = choose|k: int| 0 <= k < cx.e.bounds[i].len() && cx.e.bounds[i][k] == bnd;
        assert(ckc_spec::arith::number(tv) is Some && ckc_spec::arith::number(uv) is Some);
        assert(ws::lt(tv, bnd) == ws::lt(uv, bnd) && (tv == bnd) == (uv == bnd));
        // the other arguments carry no instantiated variable; the bound is a number
        assert forall|y: nat| #[trigger] occurs(y, args[0]) implies !is_ph(cx, y) by {
            crate::m6_head::occurs_member(y, args, 0);
            assert(occurs(y, ga));
        }
        assert forall|y: nat| #[trigger] occurs(y, args[2]) implies !is_ph(cx, y) by {
            crate::m6_head::occurs_member(y, args, 2);
            assert(occurs(y, ga));
        }
        inst_pure(cx, cur, args[0]);
        inst_pure(cx, cur, args[2]);
        assert(ckc_spec::arith::number(bnd) is Some);
        crate::k3_sound::log::number_ground(bnd);
        crate::m6_cell::inst_ground(bnd, cur.tht);
        crate::m6_cell::inst_ground(bnd, cur.thu);
        rel_root(gt, ga, cur.tht, cx.b, cx.d);
        rel_root(gu, ga, cur.thu, cx.b, cx.d);
        inst_all_index(args, cur.tht);
        inst_all_index(args, cur.thu);
        let at = gt->Comp_1;
        let au = gu->Comp_1;
        assert(crate::k3_sound::naf::rename(at[1], cx.b, cx.d) == tv);
        assert(crate::k3_sound::naf::rename(au[1], cx.b, cx.d) == uv);
        crate::k3_sound::log::number_ground(tv);
        crate::k3_sound::log::number_ground(uv);
        crate::k3_sound::naf::rename_ground_id(tv, cx.b, cx.d);
        crate::k3_sound::naf::rename_ground_id(uv, cx.b, cx.d);
        crate::k3_sound::naf::rename_inj(at[1], tv, cx.b, cx.d);
        crate::k3_sound::naf::rename_inj(au[1], uv, cx.b, cx.d);
        crate::k3_sound::naf::rename_ground_id(bnd, cx.b, cx.d);
        crate::k3_sound::naf::rename_inj(at[3], bnd, cx.b, cx.d);
        crate::k3_sound::naf::rename_inj(au[3], bnd, cx.b, cx.d);
        crate::k3_sound::naf::rename_inj(at[0], au[0], cx.b, cx.d);
        crate::k3_sound::naf::rename_inj(at[2], au[2], cx.b, cx.d);
        assert(at[1] == tv && au[1] == uv && at[3] == bnd && au[3] == bnd);
        assert(at[0] == au[0] && at[2] == au[2]);
        // the built-in's verdict reads only the sign of tv - bnd
        let (p1, q1) = ckc_spec::arith::number(tv)->0;
        let (p3, q3) = ckc_spec::arith::number(uv)->0;
        let (p2, q2) = ckc_spec::arith::number(bnd)->0;
        if p1 * q2 == p2 * q1 {
            crate::m6_cover::number_unique(tv, bnd);
        }
        if p3 * q2 == p2 * q3 {
            crate::m6_cover::number_unique(uv, bnd);
        }
        if let (Term::Atom(sa), Term::Atom(ob)) = (at[0], at[2]) {
            crate::m6_cover::within_sign(
                ckc_spec::compare::within_rows(),
                sa,
                ob,
                p1 * q2,
                p2 * q1,
                p3 * q2,
                p2 * q3,
            );
        }
        reveal(ckc_spec::arith::is_arith);
        assert(!ckc_spec::arith::is_arith(gt->Comp_0, at.len()));
        assert(ckc_spec::arith::arith_result(gt) is None);
        assert(ckc_spec::arith::arith_result(gu) is None);
    }
}

// Dropping the first (non-arithmetic) goal of a frame: the rest's frame.
proof fn drop_first_goal(cx: Cx, cur: Fr, n2: nat)
    requires
        fr_ok(cx, cur),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        !ws::is_arith_lit(cur.stack[0]->Lit_0),
        ws::walk_ok(cx.e, enc(seg(cur.stack.drop_first())), n2, cur.k),
    ensures
        fr_ok(cx, Fr { stack: cur.stack.drop_first(), n: n2, ..cur }),
{
    let ra = cur.stack.drop_first();
    let fa = Fr { stack: ra, n: n2, ..cur };
    si_replace(
        cx,
        cur,
        Seq::<Goal>::empty(),
        Fr { stack: Seq::<Goal>::empty() + ra, n: n2, ..cur },
    );
    assert(Seq::<Goal>::empty() + ra =~= ra);
    assert(cur.stack =~= seq![cur.stack[0]] + ra);
    seg_cons(cur.stack[0], ra);
    assert(crate::m6_fragment::structured_upto(enc(seg(ra)), seg(ra).len() as int)) by {
        assert forall|i: int| 0 <= i < seg(ra).len() implies crate::m6_fragment::structured(
            #[trigger] enc(seg(ra))[i],
        ) by {
            assert(enc(seg(cur.stack))[i + 1] == enc(seg(ra))[i]);
        }
    }
}

pub proof fn step_cmp(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>)
    requires
        cx_ok(cx),
        rel(cx, ct, cu, cur, alts),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        cur.stack[0]->Lit_1 > 0,
        cur.stack[0]->Lit_0 is Comp,
        ckc_spec::compare::is_builtin(
            cur.stack[0]->Lit_0->Comp_0,
            cur.stack[0]->Lit_0->Comp_1.len(),
        ),
        !ckc_spec::arith::is_arith(cur.stack[0]->Lit_0->Comp_0, cur.stack[0]->Lit_0->Comp_1.len()),
    ensures
        step_rel(cx, step(cx.dbt, ct), step(cx.dbu, cu)),
{
    let ga = cur.stack[0]->Lit_0;
    first_lit(cx, ct, cu, cur, alts);
    walk_builtin(cx, cur);
    let gt = ct.stack[0]->Lit_0;
    let gu = cu.stack[0]->Lit_0;
    names_distinct();
    reveal(ckc_spec::compare::is_builtin);
    reveal(ckc_spec::arith::is_arith);
    walk_not_operator(cx, cur);
    assert(grel(ct.stack[0], cur.stack[0], cur.tht, cx.b, cx.d));
    assert(grel(cu.stack[0], cur.stack[0], cur.thu, cx.b, cx.d));
    selected_zero(ct.stack);
    selected_zero(cu.stack);
    assert(!ws::is_arith_lit(ga));
    assert(goal_si(cx, seg(cur.stack)[0], cur.k + arank(seg(cur.stack), 0)));
    assert(pure(cx, ga));
    cmp_holds_eq(cx, cur, ga, gt, gu);
    assert(ct.stack.take(0) =~= Seq::<Goal>::empty());
    assert(cu.stack.take(0) =~= Seq::<Goal>::empty());
    if ckc_spec::compare::holds(gt) {
        let nt = Cfg { stack: Seq::<Goal>::empty() + ct.stack.skip(1), ..ct };
        let nu = Cfg { stack: Seq::<Goal>::empty() + cu.stack.skip(1), ..cu };
        assert(step(cx.dbt, ct) == Step::Next(nt));
        assert(step(cx.dbu, cu) == Step::Next(nu));
        let ra = cur.stack.drop_first();
        let fr2 = Fr { stack: ra, n: (cur.n - 1) as nat, ..cur };
        drop_first_goal(cx, cur, (cur.n - 1) as nat);
        srel_tail(ct.stack, cur.stack, cur.tht, cx.b, cx.d, 1);
        srel_tail(cu.stack, cur.stack, cur.thu, cx.b, cx.d, 1);
        assert(cur.stack.skip(1) =~= ra);
        assert(nt.stack =~= ct.stack.skip(1));
        assert(nu.stack =~= cu.stack.skip(1));
        assert(cuts_ok(ra, alts.len())) by {
            assert forall|j: int|
                0 <= j < ra.len() && (#[trigger] ra[j]) is NafCut implies ra[j]->NafCut_0
                < alts.len() by {
                assert(ra[j] == cur.stack[j + 1]);
            }
        }
        assert(rel(cx, nt, nu, fr2, alts));
        step_next(cx, nt, nu, fr2, alts);
    } else {
        assert(step(cx.dbt, ct) == fail(ct));
        assert(step(cx.dbu, cu) == fail(cu));
        fail_rel(cx, ct, cu, alts);
    }
}

// The steps that never read the program: an empty stack, a naf cut, a goal
// at depth 0, a goal that is no compound.
pub proof fn step_trivial(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>)
    requires
        rel(cx, ct, cu, cur, alts),
        cur.stack.len() == 0 || cur.stack[0] is NafCut || (cur.stack[0] is Lit && (
        cur.stack[0]->Lit_1 == 0 || !(cur.stack[0]->Lit_0 is Comp))),
    ensures
        step_rel(cx, step(cx.dbt, ct), step(cx.dbu, cu)),
{
    if cur.stack.len() == 0 {
        assert(step(cx.dbt, ct) == Step::Sol);
        assert(step(cx.dbu, cu) == Step::Sol);
    } else {
        assert(grel(ct.stack[0], cur.stack[0], cur.tht, cx.b, cx.d));
        assert(grel(cu.stack[0], cur.stack[0], cur.thu, cx.b, cx.d));
        if cur.stack[0] is Lit {
            walk_not_operator(cx, cur);
            rel_root(ct.stack[0]->Lit_0, cur.stack[0]->Lit_0, cur.tht, cx.b, cx.d);
            rel_root(cu.stack[0]->Lit_0, cur.stack[0]->Lit_0, cur.thu, cx.b, cx.d);
        }
        selected_zero(ct.stack);
        selected_zero(cu.stack);
        if cur.stack[0] is NafCut {
            let lvl = cur.stack[0]->NafCut_0;
            assert(lvl < alts.len());
            let at2 = Cfg { alts: ct.alts.take(lvl as int), pruned: entry_pruned(ct, lvl), ..ct };
            let au2 = Cfg { alts: cu.alts.take(lvl as int), pruned: entry_pruned(cu, lvl), ..cu };
            assert(step(cx.dbt, ct) == fail(at2));
            assert(step(cx.dbu, cu) == fail(au2));
            alts_at(cx, ct.alts, cu.alts, alts, lvl as int);
            assert(entry_pruned(ct, lvl) == entry_pruned(cu, lvl));
            let a2 = alts.take(lvl as int);
            assert(alts_rel(cx, ct.alts, cu.alts, alts));
            assert(alts_rel(cx, at2.alts, au2.alts, a2)) by {
                assert forall|i: int| 0 <= i < a2.len() implies #[trigger] alt_ok(
                    cx,
                    at2.alts[i],
                    au2.alts[i],
                    a2[i],
                    i,
                ) && cuts_ok(a2[i].stack, i as nat) by {
                    assert(at2.alts[i] == ct.alts[i] && au2.alts[i] == cu.alts[i] && a2[i]
                        == alts[i]);
                    alts_at(cx, ct.alts, cu.alts, alts, i);
                }
            }
            fail_rel(cx, at2, au2, a2);
        } else if cur.stack[0]->Lit_1 == 0 {
            let at2 = Cfg { pruned: true, ..ct };
            let au2 = Cfg { pruned: true, ..cu };
            assert(step(cx.dbt, ct) == fail(at2));
            assert(step(cx.dbu, cu) == fail(au2));
            fail_rel(cx, at2, au2, alts);
        } else {
            // every first-segment goal is a compound (the walk's structured goals)
            let ra = cur.stack.drop_first();
            assert(cur.stack =~= seq![cur.stack[0]] + ra);
            seg_cons(cur.stack[0], ra);
            assert(enc(seg(cur.stack))[0] == cur.stack[0]->Lit_0);
            assert(crate::m6_fragment::structured(cur.stack[0]->Lit_0));
        }
    }
}

// --- purity survives fact matches ---
pub open spec fn ph_free(cx: Cx, t: Term) -> bool {
    forall|x: nat| #[trigger] occurs(x, t) ==> !is_ph(cx, x)
}

pub open spec fn ph_free_pairs(cx: Cx, ps: Seq<(Term, Term)>) -> bool {
    forall|i: int| 0 <= i < ps.len() ==> ph_free(cx, (#[trigger] ps[i]).0) && ph_free(cx, ps[i].1)
}

pub proof fn pure_subst(cx: Cx, t: Term, z: nat, v: Term)
    requires
        crate::m6_fragment::structured(t),
        pure(cx, t),
        ph_free(cx, v),
    ensures
        pure(cx, subst(t, z, v)),
        crate::m6_fragment::structured(subst(t, z, v)),
    decreases t,
{
    let r = subst(t, z, v);
    assert forall|x: nat| #[trigger] occurs(x, r) implies !is_ph(cx, x) by {
        if !occurs(x, t) && !occurs(x, v) {
            crate::m6_cell::subst_occurs(t, z, v, x);
        }
    }
    if let Term::Comp(name, args) = t {
        crate::k3_sound::naf::subst_all_index(args, z, v);
        if name == comma_name() && args.len() == 2 {
            assert forall|x: nat| #[trigger] occurs(x, args[0]) implies !is_ph(cx, x) by {
                crate::m6_head::occurs_member(x, args, 0);
                assert(occurs(x, t));
            }
            assert forall|x: nat| #[trigger] occurs(x, args[1]) implies !is_ph(cx, x) by {
                crate::m6_head::occurs_member(x, args, 1);
                assert(occurs(x, t));
            }
            pure_subst(cx, args[0], z, v);
            pure_subst(cx, args[1], z, v);
        } else if name == naf_name() && args.len() == 1 {
            pure_subst(cx, args[0], z, v);
        } else {
            reveal(ckc_spec::arith::is_arith);
        }
    }
}

// Each solution entry of a placeholder-free unification stays pure.
pub proof fn unify_n_pure(cx: Cx, u: UState, f: nat, i: int)
    requires
        ph_free_pairs(cx, u.pairs),
        0 <= i < u.sol.len(),
        crate::m6_fragment::structured(u.sol[i]),
        pure(cx, u.sol[i]),
        unify_n(u, f) is Ok,
    ensures
        i < unify_n(u, f)->Ok_1.len(),
        pure(cx, unify_n(u, f)->Ok_1[i]),
        crate::m6_fragment::structured(unify_n(u, f)->Ok_1[i]),
    decreases f,
{
    reveal_with_fuel(unify_n, 1);
    if u.pairs.len() == 0 {
        return;
    }
    let a = u.pairs[0].0;
    let b = u.pairs[0].1;
    let rest = u.pairs.drop_first();
    let f1 = (f - 1) as nat;
    assert(ph_free(cx, a) && ph_free(cx, b));
    assert(ph_free_pairs(cx, rest)) by {
        assert forall|j: int| 0 <= j < rest.len() implies ph_free(cx, (#[trigger] rest[j]).0)
            && ph_free(cx, rest[j].1) by {
            assert(rest[j] == u.pairs[j + 1]);
        }
    }
    match (a, b) {
        (Term::Var(y), _) => {
            if a == b {
                unify_n_pure(cx, UState { pairs: rest, ..u }, f1, i);
            } else if !occurs(y, b) {
                let n = u_bind(u, rest, y, b);
                bind_ph_free(cx, rest, y, b);
                crate::k3_sound::unification::subst_all_map(u.sol, y, b);
                assert(n.sol[i] == subst(u.sol[i], y, b));
                pure_subst(cx, u.sol[i], y, b);
                unify_n_pure(cx, n, f1, i);
            }
        },
        (_, Term::Var(y)) => {
            if !occurs(y, a) {
                let n = u_bind(u, rest, y, a);
                bind_ph_free(cx, rest, y, a);
                crate::k3_sound::unification::subst_all_map(u.sol, y, a);
                assert(n.sol[i] == subst(u.sol[i], y, a));
                pure_subst(cx, u.sol[i], y, a);
                unify_n_pure(cx, n, f1, i);
            }
        },
        (Term::Comp(n, xs), Term::Comp(m, ys)) => {
            if n == m && xs.len() == ys.len() {
                let ps = zip(xs, ys) + rest;
                assert(ph_free_pairs(cx, ps)) by {
                    assert forall|j: int| 0 <= j < ps.len() implies ph_free(
                        cx,
                        (#[trigger] ps[j]).0,
                    ) && ph_free(cx, ps[j].1) by {
                        if j < xs.len() {
                            assert(ps[j] == (xs[j], ys[j]));
                            assert forall|x: nat| #[trigger] occurs(x, xs[j]) implies !is_ph(
                                cx,
                                x,
                            ) by {
                                crate::m6_head::occurs_member(x, xs, j);
                                assert(occurs(x, a));
                            }
                            assert forall|x: nat| #[trigger] occurs(x, ys[j]) implies !is_ph(
                                cx,
                                x,
                            ) by {
                                crate::m6_head::occurs_member(x, ys, j);
                                assert(occurs(x, b));
                            }
                        } else {
                            assert(ps[j] == rest[j - xs.len()]);
                        }
                    }
                }
                unify_n_pure(cx, UState { pairs: ps, ..u }, f1, i);
            }
        },
        _ => {
            if a == b {
                unify_n_pure(cx, UState { pairs: rest, ..u }, f1, i);
            }
        },
    }
}

proof fn bind_ph_free(cx: Cx, rest: Seq<(Term, Term)>, y: nat, v: Term)
    requires
        ph_free_pairs(cx, rest),
        ph_free(cx, v),
    ensures
        ph_free_pairs(cx, rest.map_values(|p: (Term, Term)| (subst(p.0, y, v), subst(p.1, y, v)))),
{
    let n = rest.map_values(|p: (Term, Term)| (subst(p.0, y, v), subst(p.1, y, v)));
    assert forall|j: int| 0 <= j < n.len() implies ph_free(cx, (#[trigger] n[j]).0) && ph_free(
        cx,
        n[j].1,
    ) by {
        assert(ph_free(cx, rest[j].0) && ph_free(cx, rest[j].1));
        assert forall|x: nat| #[trigger] occurs(x, n[j].0) implies !is_ph(cx, x) by {
            if !occurs(x, rest[j].0) && !occurs(x, v) {
                crate::m6_cell::subst_occurs(rest[j].0, y, v, x);
            }
        }
        assert forall|x: nat| #[trigger] occurs(x, n[j].1) implies !is_ph(cx, x) by {
            if !occurs(x, rest[j].1) && !occurs(x, v) {
                crate::m6_cell::subst_occurs(rest[j].1, y, v, x);
            }
        }
    }
}

// --- the walk at a call ---
pub open spec fn verdict(e: ws::WalkEnv, g: Term, rest: Seq<Term>, n: nat, k: nat, j: int) -> bool {
    match unify(UState { pairs: seq![(g, e.facts[j])], stack: Seq::empty(), sol: rest + e.slots }) {
        UOut::Ok(_, s) => s.skip(rest.len() as int) == e.slots && ws::walk_ok(
            e,
            s.take(rest.len() as int),
            n,
            k,
        ),
        UOut::Fail => true,
        UOut::Out => false,
    }
}

pub proof fn facts_ok_at(e: ws::WalkEnv, g: Term, rest: Seq<Term>, n: nat, k: nat, j0: nat, j: int)
    requires
        ws::facts_ok(e, g, rest, n, k, j0),
        j0 <= j < e.facts.len(),
    ensures
        verdict(e, g, rest, n, k, j),
    decreases j - j0,
{
    reveal_with_fuel(ws::facts_ok, 1);
    if j > j0 {
        facts_ok_at(e, g, rest, n, k, j0 + 1, j);
    }
}

pub open spec fn is_call(t: Term) -> bool {
    &&& t is Comp
    &&& !(t->Comp_0 == comma_name() && t->Comp_1.len() == 2)
    &&& !(t->Comp_0 == naf_name() && t->Comp_1.len() == 1)
    &&& !ckc_spec::arith::is_arith(t->Comp_0, t->Comp_1.len())
    &&& !ckc_spec::compare::is_builtin(t->Comp_0, t->Comp_1.len())
}

// The walk on a call first goal: its fact verdicts + the absent rules.
pub proof fn walk_call(cx: Cx, cur: Fr)
    requires
        fr_ok(cx, cur),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        is_call(cur.stack[0]->Lit_0),
    ensures
        cur.n > 0,
        ws::call_ok(
            cx.e,
            cur.stack[0]->Lit_0,
            enc(seg(cur.stack.drop_first())),
            (cur.n - 1) as nat,
            cur.k,
        ),
{
    let ga = cur.stack[0]->Lit_0;
    let ra = cur.stack.drop_first();
    assert(cur.stack =~= seq![cur.stack[0]] + ra);
    seg_cons(cur.stack[0], ra);
    let ts1 = enc(seg(cur.stack));
    assert(ts1 =~= seq![ga] + enc(seg(ra)));
    walk_not_operator(cx, cur);
    reveal_with_fuel(ws::walk_ok, 1);
    assert(ts1[0] == ga && ts1.drop_first() =~= enc(seg(ra)));
}

pub proof fn seg_decode(as_: Seq<Goal>, ts: Seq<Term>)
    requires
        ts.len() == as_.len(),
    ensures
        seg(decode(as_, ts)).len() == seg(as_).len(),
        forall|i: int|
            0 <= i < seg(as_).len() ==> #[trigger] seg(decode(as_, ts))[i] == Goal::Lit(
                ts[i],
                as_[i]->Lit_1,
            ),
        enc(seg(decode(as_, ts))) == ts.take(seg(as_).len() as int),
    decreases as_.len(),
{
    let da = decode(as_, ts);
    seg_props(as_);
    seg_props(da);
    if as_.len() > 0 {
        let a2 = as_.drop_first();
        let t2 = ts.drop_first();
        seg_decode(a2, t2);
        assert(da.drop_first() =~= decode(a2, t2));
        if as_[0] is Lit {
            assert(da[0] == Goal::Lit(ts[0], as_[0]->Lit_1));
            assert(as_ =~= seq![as_[0]] + a2);
            seg_cons(as_[0], a2);
            assert(da =~= seq![da[0]] + decode(a2, t2));
            seg_cons(da[0], decode(a2, t2));
            assert forall|i: int| 0 <= i < seg(as_).len() implies #[trigger] seg(da)[i]
                == Goal::Lit(ts[i], as_[i]->Lit_1) by {
                if i > 0 {
                    assert(seg(da)[i] == seg(decode(a2, t2))[i - 1]);
                    assert(a2[i - 1] == as_[i]);
                }
            }
            assert(enc(seg(da)) =~= ts.take(seg(as_).len() as int)) by {
                assert forall|i: int| 0 <= i < seg(as_).len() implies enc(seg(da))[i] == ts[i] by {
                    assert(seg(da)[i] == Goal::Lit(ts[i], as_[i]->Lit_1));
                }
            }
        } else {
            assert(da[0] == Goal::NafCut(as_[0]->NafCut_0));
            assert(seg(da) =~= Seq::<Goal>::empty());
            assert(seg(as_) =~= Seq::<Goal>::empty());
        }
    } else {
        assert(seg(da) =~= Seq::<Goal>::empty());
    }
}

// --- call helpers ---
pub proof fn inst_agree(t: Term, th1: Seq<(nat, Term)>, th2: Seq<(nat, Term)>)
    requires
        forall|x: nat| #[trigger] occurs(x, t) ==> th_val(th1, x) == th_val(th2, x),
    ensures
        inst(t, th1) == inst(t, th2),
    decreases t,
{
    match t {
        Term::Var(x) => {
            assert(occurs(x, t));
        },
        Term::Comp(n, args) => {
            inst_all_index(args, th1);
            inst_all_index(args, th2);
            assert forall|i: int| 0 <= i < args.len() implies inst(#[trigger] args[i], th1) == inst(
                args[i],
                th2,
            ) by {
                assert forall|x: nat| #[trigger] occurs(x, args[i]) implies th_val(th1, x)
                    == th_val(th2, x) by {
                    crate::m6_head::occurs_member(x, args, i);
                    assert(occurs(x, t));
                }
                inst_agree(args[i], th1, th2);
            }
            assert(inst_all(args, th1) =~= inst_all(args, th2));
        },
        _ => {},
    }
}

pub proof fn next_match_eq(cx: Cx, name: Seq<u8>, ar: nat, from: nat)
    requires
        cx_ok(cx),
    ensures
        next_match(cx.dbt, name, ar, from) == next_match(cx.dbu, name, ar, from),
    decreases cx.dbt.len() - from,
{
    if from < cx.dbt.len() {
        assert(lit_fa(cx.dbt[from as int].head) == lit_fa(cx.dbu[from as int].head)) by {
            if from < cx.nf {
                let j = cx.nf - 1 - from;
                assert(cx.e.facts[j] is Comp);
                assert(cx.dbt[from as int] == fact_clause(inst(cx.e.facts[j], cx.st)));
                assert(cx.dbu[from as int] == fact_clause(inst(cx.e.facts[j], cx.su)));
                inst_all_index(cx.e.facts[j]->Comp_1, cx.st);
                inst_all_index(cx.e.facts[j]->Comp_1, cx.su);
            } else {
                let q = from - cx.nf;
                assert(cx.dbt[cx.nf + q] == cx.e.p[q]);
            }
        }
        next_match_eq(cx, name, ar, from + 1);
    }
}

// Two goal lists with the same arithmetic pattern rank alike.
pub proof fn arank_same(s1: Seq<Goal>, s2: Seq<Goal>, i: int)
    requires
        s1.len() == s2.len(),
        0 <= i <= s1.len(),
        forall|j: int|
            0 <= j < s1.len() ==> ((#[trigger] s1[j]) is Lit && ws::is_arith_lit(s1[j]->Lit_0)) == (
            s2[j] is Lit && ws::is_arith_lit(s2[j]->Lit_0)),
    ensures
        arank(s1, i) == arank(s2, i),
    decreases i,
{
    if i > 0 {
        arank_same(s1, s2, i - 1);
        assert((s1[i - 1] is Lit && ws::is_arith_lit(s1[i - 1]->Lit_0)) == (s2[i - 1] is Lit
            && ws::is_arith_lit(s2[i - 1]->Lit_0)));
    }
}

// Node shapes survive a unifier that keeps every placeholder in place.
pub proof fn node_shape_keeps(cx: Cx, t: Term, t2: Term, kk: nat)
    requires
        cx.o4.len() == cx.e.chain.len(),
        cx.o5.len() == cx.e.chain.len(),
        node_shape(cx, t, kk),
        forall|x: nat| #[trigger] is_ph(cx, x) ==> crate::m6_cell::keeps(t, t2, x),
    ensures
        node_shape(cx, t2, kk),
{
    let a = t->Comp_1;
    let o4 = cx.o4[kk as int];
    let o5 = cx.o5[kk as int];
    assert(cx.o4.contains(o4) && cx.o5.contains(o5));
    assert(is_ph(cx, o4) && is_ph(cx, o5));
    assert(crate::m6_cell::keeps(t, t2, o4));
    assert(crate::m6_cell::keeps(t, t2, o5));
    assert(a[4] == Term::Var(o4) && a[5] == Term::Var(o5));
    assert forall|m: int| 0 <= m < kk implies #[trigger] from_node(cx, t2, kk, 0, m) && from_node(
        cx,
        t2,
        kk,
        2,
        m,
    ) by {
        assert(from_node(cx, t, kk, 0, m) && from_node(cx, t, kk, 2, m));
        let p4 = cx.o4[m];
        let p5 = cx.o5[m];
        assert(cx.o4.contains(p4) && cx.o5.contains(p5));
        assert(is_ph(cx, p4) && is_ph(cx, p5));
        assert(crate::m6_cell::keeps(t, t2, p4));
        assert(crate::m6_cell::keeps(t, t2, p5));
    }
}

// The configuration a call's success leaves (`engine::call`).
pub open spec fn called(c: Cfg, cl: DocClause, m: nat, stack: Seq<Goal>, sol: Seq<Term>) -> Cfg {
    Cfg {
        alts: c.alts.push(
            Alt { stack: c.stack, sol: c.sol, fresh: c.fresh, ci: m + 1, naf: Option::None },
        ),
        stack,
        sol,
        fresh: c.fresh + clause_nvars(cl),
        ci: 0,
        ..c
    }
}

pub open spec fn cstate(c: Cfg, cl: DocClause, dd: nat) -> UState {
    UState {
        pairs: zip(args_of(c.stack[0]->Lit_0), args_of(shift(cl.head, c.fresh))),
        stack: Seq::<Goal>::empty() + body_goals(cl.body, c.fresh, (dd - 1) as nat) + c.stack.skip(
            1,
        ),
        sol: c.sol,
    }
}

// A fact candidate (abstract fa at walk index j; concrete heads ft, fu):
// both calls succeed or both fail, and successes stay related.
pub proof fn fact_step(
    cx: Cx,
    ct: Cfg,
    cu: Cfg,
    cur: Fr,
    alts: Seq<Fr>,
    m: nat,
    j: int,
    clt: DocClause,
    clu: DocClause,
)
    requires
        cx_ok(cx),
        rel(cx, ct, cu, cur, alts),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        cur.stack[0]->Lit_1 > 0,
        is_call(cur.stack[0]->Lit_0),
        0 <= j < cx.e.facts.len(),
        cx.e.facts[j] is Comp,
        skel_fact(cx, cx.e.facts[j]),
        clt == fact_clause(inst(cx.e.facts[j], cx.st)),
        clu == fact_clause(inst(cx.e.facts[j], cx.su)),
        ground(clt.head),
        ground(clu.head),
        lit_fa(clt.head) == lit_fa(ct.stack[0]->Lit_0),
    ensures
        ({
            let dd = cur.stack[0]->Lit_1;
            let ut = cstate(ct, clt, dd);
            let uu = cstate(cu, clu, dd);
            &&& (unify(ut) is Ok) == (unify(uu) is Ok)
            &&& !(unify(ut) is Out)
            &&& !(unify(uu) is Out)
            &&& unify(ut) is Ok ==> step_rel(
                cx,
                Step::Next(called(ct, clt, m, unify(ut)->Ok_0, unify(ut)->Ok_1)),
                Step::Next(called(cu, clu, m, unify(uu)->Ok_0, unify(uu)->Ok_1)),
            )
        }),
{
    let ga = cur.stack[0]->Lit_0;
    let dd = cur.stack[0]->Lit_1;
    let fa = cx.e.facts[j];
    let arest = cur.stack.drop_first();
    first_lit(cx, ct, cu, cur, alts);
    walk_call(cx, cur);
    let gt = ct.stack[0]->Lit_0;
    let gu = cu.stack[0]->Lit_0;
    let rt = ct.stack.skip(1);
    let ru = cu.stack.skip(1);
    let segt = enc(seg(arest));
    // the walk's verdict on fa
    reveal_with_fuel(ws::call_ok, 1);
    facts_ok_at(cx.e, ga, segt, (cur.n - 1) as nat, cur.k, 0, j);
    // the instantiations agree on fa's slots
    assert forall|x: nat| #[trigger] occurs(x, fa) implies th_val(cur.tht, x) == th_val(cx.st, x)
        && th_val(cur.thu, x) == th_val(cx.su, x) by {
        assert(th_val(cx.st, x) is Some);
    }
    inst_agree(fa, cur.tht, cx.st);
    inst_agree(fa, cur.thu, cx.su);
    // goal + fact carry no instantiated variable other than slots
    assert(goal_si(cx, seg(cur.stack)[0], cur.k + arank(seg(cur.stack), 0)));
    assert(!ws::is_arith_lit(ga)) by {
        reveal(ckc_spec::arith::is_arith);
    }
    assert(pure(cx, ga));
    assert forall|x: nat|
        th_val(cur.tht, x) is Some && (occurs(x, ga) || occurs(
            x,
            fa,
        )) implies #[trigger] cx.e.slots.contains(Term::Var(x)) by {
        if occurs(x, fa) {
            assert(th_val(cx.st, x) is Some);
        } else {
            assert(!is_ph(cx, x));
            assert(th_val(cx.st, x) is Some || is_ph(cx, x));
        }
    }
    assert forall|x: nat|
        th_val(cur.thu, x) is Some && (occurs(x, ga) || occurs(
            x,
            fa,
        )) implies #[trigger] cx.e.slots.contains(Term::Var(x)) by {
        assert(th_val(cur.tht, x) is Some);
        if occurs(x, fa) {
            assert(th_val(cx.st, x) is Some);
        } else {
            assert(!is_ph(cx, x));
        }
    }
    // the concrete states, as cand_fact reads them
    crate::k3_probe::shift_ground(clt.head, ct.fresh);
    crate::k3_probe::shift_ground(clu.head, cu.fresh);
    assert(body_goals(clt.body, ct.fresh, (dd - 1) as nat) =~= Seq::<Goal>::empty());
    assert(body_goals(clu.body, cu.fresh, (dd - 1) as nat) =~= Seq::<Goal>::empty());
    let ut = cstate(ct, clt, dd);
    let uu = cstate(cu, clu, dd);
    assert(ut.stack =~= rt);
    assert(uu.stack =~= ru);
    assert(ct.sol =~= Seq::<Term>::empty());
    assert(cu.sol =~= Seq::<Term>::empty());
    assert(ut == UState {
        pairs: zip(args_of(gt), args_of(clt.head)),
        stack: rt,
        sol: Seq::<Term>::empty(),
    });
    assert(uu == UState {
        pairs: zip(args_of(gu), args_of(clu.head)),
        stack: ru,
        sol: Seq::<Term>::empty(),
    });
    srel_tail(ct.stack, cur.stack, cur.tht, cx.b, cx.d, 1);
    srel_tail(cu.stack, cur.stack, cur.thu, cx.b, cx.d, 1);
    assert(cur.stack.skip(1) =~= arest);
    assert(grel(ct.stack[0], cur.stack[0], cur.tht, cx.b, cx.d));
    assert(grel(cu.stack[0], cur.stack[0], cur.thu, cx.b, cx.d));
    inst_all_index(fa->Comp_1, cx.st);
    inst_all_index(fa->Comp_1, cx.su);
    assert(lit_fa(clu.head) == lit_fa(clt.head));
    assert(lit_fa(gu) == lit_fa(gt));
    cand_fact(ga, fa, arest, cx.e.slots, segt, cur.tht, gt, rt, clt.head, cx.b, cx.d);
    cand_fact(ga, fa, arest, cx.e.slots, segt, cur.thu, gu, ru, clu.head, cx.b, cx.d);
    let ua = UState { pairs: seq![(ga, fa)], stack: Seq::<Goal>::empty(), sol: enc(arest) };
    if unify(ut) is Ok {
        let ra = unify(ua)->Ok_1;
        let sct = unify(ut)->Ok_0;
        let scu = unify(uu)->Ok_0;
        let nt = called(ct, clt, m, sct, unify(ut)->Ok_1);
        let nu = called(cu, clu, m, scu, unify(uu)->Ok_1);
        // facts add no variable: the fresh counter stays
        crate::k2_walk::ground_nvars(clt.head);
        crate::k2_walk::ground_nvars(clu.head);
        assert(clause_nvars(clt) == 0 && clause_nvars(clu) == 0);
        let fr2 = Fr { stack: decode(arest, ra), n: (cur.n - 1) as nat, ..cur };
        let alts2 = alts.push(cur);
        // the abstract unifier, entry by entry
        let f = crate::m6_lockstep::unify_fuel(ua);
        crate::k3_sound::naf::unify_output_len(ua);
        assert(ra.len() == arest.len());
        seg_decode(arest, ra);
        seg_props(arest);
        let sa = seg(arest);
        assert forall|x: nat| #[trigger] is_ph(cx, x) implies !occ_pairs(x, ua.pairs) by {
            if occ_pairs(x, ua.pairs) {
                let q = choose|q: int|
                    0 <= q < ua.pairs.len() && (occurs(x, #[trigger] ua.pairs[q].0) || occurs(
                        x,
                        ua.pairs[q].1,
                    ));
                assert(q == 0);
                if occurs(x, fa) {
                    assert(th_val(cx.st, x) is Some);
                }
            }
        }
        assert(ph_free_pairs(cx, ua.pairs)) by {
            assert forall|q: int| 0 <= q < ua.pairs.len() implies ph_free(
                cx,
                (#[trigger] ua.pairs[q]).0,
            ) && ph_free(cx, ua.pairs[q].1) by {
                assert(q == 0);
                assert forall|x: nat| #[trigger] occurs(x, fa) implies !is_ph(cx, x) by {
                    assert(th_val(cx.st, x) is Some);
                }
            }
        }
        // the walk's continuation = the new first segment
        let uw = UState {
            pairs: seq![(ga, fa)],
            stack: Seq::<Goal>::empty(),
            sol: segt + cx.e.slots,
        };
        crate::m6_lockstep::unify_passive(ua, uw);
        assert(verdict(cx.e, ga, segt, (cur.n - 1) as nat, cur.k, j));
        let sw = unify(uw)->Ok_1;
        assert(segt.len() == sa.len());
        assert(enc(seg(fr2.stack)) =~= sw.take(segt.len() as int)) by {
            assert forall|i: int| 0 <= i < sa.len() implies enc(seg(fr2.stack))[i] == sw[i] by {
                assert(enc(arest)[i] == (segt + cx.e.slots)[i]) by {
                    assert(sa[i] == arest[i]);
                }
                assert(ra[i] == sw[i]);
            }
        }
        // structure + shapes of the new segment
        assert forall|i: int| 0 <= i < sa.len() implies crate::m6_fragment::structured(
            #[trigger] ra[i],
        ) && goal_si(cx, seg(fr2.stack)[i], cur.k + arank(sa, i)) && (ws::is_arith_lit(ra[i])
            == ws::is_arith_lit(enc(arest)[i])) by {
            assert(seg(cur.stack)[i + 1] == sa[i]);
            assert(sa[i] == arest[i]);
            assert(goal_si(cx, seg(cur.stack)[i + 1], cur.k + arank(seg(cur.stack), i + 1)));
            arank_cons(cur.stack[0], sa, i);
            assert(enc(seg(cur.stack))[i + 1] == enc(arest)[i]);
            let t0 = enc(arest)[i];
            assert(crate::m6_fragment::structured(t0));
            if ws::is_arith_lit(t0) {
                assert(node_shape(cx, t0, cur.k + arank(sa, i)));
                assert forall|x: nat| #[trigger] is_ph(cx, x) implies crate::m6_cell::keeps(
                    t0,
                    ra[i],
                    x,
                ) by {
                    crate::m6_cell::unify_n_keeps(ua, f, x, i);
                }
                node_shape_keeps(cx, t0, ra[i], cur.k + arank(sa, i));
                let x4 = cx.o4[(cur.k + arank(sa, i)) as int];
                assert(cx.o4.contains(x4));
                crate::m6_cell::unify_n_keeps(ua, f, x4, i);
            } else {
                unify_n_pure(cx, ua, f, i);
                pure_not_arith(cx, ra[i]);
            }
        }
        assert(si(cx, fr2)) by {
            assert forall|i: int| 0 <= i < seg(fr2.stack).len() implies #[trigger] goal_si(
                cx,
                seg(fr2.stack)[i],
                fr2.k + arank(seg(fr2.stack), i),
            ) by {
                arank_same(seg(fr2.stack), sa, i);
            }
        }
        assert(crate::m6_fragment::structured_upto(
            enc(seg(fr2.stack)),
            seg(fr2.stack).len() as int,
        )) by {
            assert forall|i: int|
                0 <= i < seg(fr2.stack).len() implies crate::m6_fragment::structured(
                #[trigger] enc(seg(fr2.stack))[i],
            ) by {
                assert(enc(seg(fr2.stack))[i] == ra[i]);
            }
        }
        assert(ws::walk_ok(cx.e, enc(seg(fr2.stack)), fr2.n, fr2.k));
        assert(fr_ok(cx, fr2));
        assert(cuts_ok(fr2.stack, alts2.len())) by {
            assert forall|q: int|
                0 <= q < fr2.stack.len() && (
                #[trigger] fr2.stack[q]) is NafCut implies fr2.stack[q]->NafCut_0 < alts2.len() by {
                assert(arest[q] == cur.stack[q + 1]);
            }
        }
        assert(alts_rel(cx, nt.alts, nu.alts, alts2)) by {
            assert forall|i: int| 0 <= i < alts2.len() implies #[trigger] alt_ok(
                cx,
                nt.alts[i],
                nu.alts[i],
                alts2[i],
                i,
            ) && cuts_ok(alts2[i].stack, i as nat) by {
                if i < alts.len() {
                    alts_at(cx, ct.alts, cu.alts, alts, i);
                }
            }
        }
        assert(rel(cx, nt, nu, fr2, alts2));
        step_next(cx, nt, nu, fr2, alts2);
    }
}

// A call, candidate by candidate from ci: both runs meet the same clause
// index with the same outcome.
pub proof fn call_body(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>, ci: nat)
    requires
        cx_ok(cx),
        rel(cx, ct, cu, cur, alts),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        cur.stack[0]->Lit_1 > 0,
        is_call(cur.stack[0]->Lit_0),
    ensures
        ({
            let dd = cur.stack[0]->Lit_1;
            let gt = ct.stack[0]->Lit_0;
            let gu = cu.stack[0]->Lit_0;
            step_rel(
                cx,
                call(cx.dbt, ct, gt->Comp_0, gt->Comp_1, dd, Seq::empty(), ct.stack.skip(1), ci),
                call(cx.dbu, cu, gu->Comp_0, gu->Comp_1, dd, Seq::empty(), cu.stack.skip(1), ci),
            )
        }),
    decreases cx.dbt.len() - ci,
{
    let ga = cur.stack[0]->Lit_0;
    let dd = cur.stack[0]->Lit_1;
    first_lit(cx, ct, cu, cur, alts);
    walk_call(cx, cur);
    let gt = ct.stack[0]->Lit_0;
    let gu = cu.stack[0]->Lit_0;
    let name = gt->Comp_0;
    let ar = gt->Comp_1.len();
    assert(gu->Comp_0 == name && gu->Comp_1.len() == ar);
    next_match_eq(cx, name, ar, ci);
    next_match_bound(cx.dbt, name, ar, ci);
    let st = call(cx.dbt, ct, name, gt->Comp_1, dd, Seq::empty(), ct.stack.skip(1), ci);
    let su = call(cx.dbu, cu, name, gu->Comp_1, dd, Seq::empty(), cu.stack.skip(1), ci);
    match next_match(cx.dbt, name, ar, ci) {
        Option::None => {
            assert(st == fail(ct));
            assert(su == fail(cu));
            fail_rel(cx, ct, cu, alts);
        },
        Option::Some(m) => {
            let clt = cx.dbt[m as int];
            let clu = cx.dbu[m as int];
            crate::k3_sound::naf::next_match_shape(cx.dbt, name, ar, ci);
            assert(lit_fa(clt.head) == Option::Some((name, ar)));
            let ut = cstate(ct, clt, dd);
            let uu = cstate(cu, clu, dd);
            assert(args_of(gt) == gt->Comp_1 && args_of(gu) == gu->Comp_1);
            if m < cx.nf {
                let j = cx.nf - 1 - m;
                assert(cx.dbt[m as int] == fact_clause(inst(cx.e.facts[j], cx.st)));
                assert(cx.e.facts[j] is Comp);
                fact_step(cx, ct, cu, cur, alts, m, j, clt, clu);
                if unify(ut) is Ok {
                    assert(st == Step::Next(called(ct, clt, m, unify(ut)->Ok_0, unify(ut)->Ok_1)));
                    assert(su == Step::Next(called(cu, clu, m, unify(uu)->Ok_0, unify(uu)->Ok_1)));
                } else {
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
                    call_body(cx, ct, cu, cur, alts, m + 1);
                }
            } else {
                let q: int = m - cx.nf;
                assert(0 <= q < cx.e.p.len());
                assert(cx.dbt[cx.nf + q] == cx.e.p[q] && cx.dbu[cx.nf + q] == cx.e.p[q]);
                assert(cx.nf + q == m);
                assert(clt == cx.e.p[q] && clu == cx.e.p[q]);
                if ws::ground_fact(clt) {
                    let j = choose|j: int|
                        0 <= j < cx.e.facts.len() && cx.e.facts[j] == cx.e.p[q].head;
                    assert(clt.body =~= Seq::<ckc_spec::v1text::BodyItem>::empty());
                    crate::m6_cell::inst_ground(clt.head, cx.st);
                    crate::m6_cell::inst_ground(clt.head, cx.su);
                    assert(clt == fact_clause(inst(cx.e.facts[j], cx.st)));
                    assert(clu == fact_clause(inst(cx.e.facts[j], cx.su)));
                    assert(skel_fact(cx, cx.e.facts[j])) by {
                        assert forall|x: nat| #[trigger] occurs(x, cx.e.facts[j]) implies th_val(
                            cx.st,
                            x,
                        ) is Some by {
                            crate::m6_default::ground_absent(clt.head, x);
                        }
                    }
                    fact_step(cx, ct, cu, cur, alts, m, j, clt, clu);
                    if unify(ut) is Ok {
                        assert(st == Step::Next(
                            called(ct, clt, m, unify(ut)->Ok_0, unify(ut)->Ok_1),
                        ));
                        assert(su == Step::Next(
                            called(cu, clu, m, unify(uu)->Ok_0, unify(uu)->Ok_1),
                        ));
                    } else {
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
                        call_body(cx, ct, cu, cur, alts, m + 1);
                    }
                } else {
                    // a rule: the walk saw its head never unify with the call
                    reveal_with_fuel(ws::call_ok, 1);
                    assert(!ws::unifiable(ga, shift(cx.e.p[q].head, cx.e.off)));
                    assert(grel(ct.stack[0], cur.stack[0], cur.tht, cx.b, cx.d));
                    assert(grel(cu.stack[0], cur.stack[0], cur.thu, cx.b, cx.d));
                    assert(ct.sol =~= Seq::<Term>::empty());
                    assert(cu.sol =~= Seq::<Term>::empty());
                    assert forall|x: nat| #[trigger] th_val(cur.thu, x) is Some implies x
                        < cx.e.off by {
                        assert(th_val(cur.tht, x) is Some <==> th_val(cur.thu, x) is Some);
                    }
                    crate::m6_lockstep::cand_rule(
                        ga,
                        clt.head,
                        cx.e.off,
                        cur.stack.drop_first(),
                        cur.tht,
                        gt,
                        ut.stack,
                        ct.fresh,
                        cx.b,
                        cx.d,
                    );
                    crate::m6_lockstep::cand_rule(
                        ga,
                        clu.head,
                        cx.e.off,
                        cur.stack.drop_first(),
                        cur.thu,
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
                    call_body(cx, ct, cu, cur, alts, m + 1);
                }
            }
        },
    }
}

// A call first goal: the engine's call from the frame's clause index.
pub proof fn step_call(cx: Cx, ct: Cfg, cu: Cfg, cur: Fr, alts: Seq<Fr>)
    requires
        cx_ok(cx),
        rel(cx, ct, cu, cur, alts),
        cur.stack.len() > 0,
        cur.stack[0] is Lit,
        cur.stack[0]->Lit_1 > 0,
        is_call(cur.stack[0]->Lit_0),
    ensures
        step_rel(cx, step(cx.dbt, ct), step(cx.dbu, cu)),
{
    let dd = cur.stack[0]->Lit_1;
    first_lit(cx, ct, cu, cur, alts);
    walk_not_operator(cx, cur);
    let gt = ct.stack[0]->Lit_0;
    let gu = cu.stack[0]->Lit_0;
    selected_zero(ct.stack);
    selected_zero(cu.stack);
    assert(ct.stack.take(0) =~= Seq::<Goal>::empty());
    assert(cu.stack.take(0) =~= Seq::<Goal>::empty());
    assert(step(cx.dbt, ct) == call(
        cx.dbt,
        ct,
        gt->Comp_0,
        gt->Comp_1,
        dd,
        Seq::empty(),
        ct.stack.skip(1),
        ct.ci,
    ));
    assert(step(cx.dbu, cu) == call(
        cx.dbu,
        cu,
        gu->Comp_0,
        gu->Comp_1,
        dd,
        Seq::empty(),
        cu.stack.skip(1),
        cu.ci,
    ));
    call_body(cx, ct, cu, cur, alts, ct.ci);
}

} // verus!
