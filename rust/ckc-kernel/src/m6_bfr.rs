#[cfg(verus_keep_ghost)]
use crate::m6_body::*;
#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use crate::m6_ctx::*;
#[cfg(verus_keep_ghost)]
use crate::m6_default::*;
#[cfg(verus_keep_ghost)]
use crate::m6_enter::*;
#[cfg(verus_keep_ghost)]
use crate::m6_hd::*;
#[cfg(verus_keep_ghost)]
use crate::m6_head::{occurs_index, occurs_member, rmap, sub, sub_all, sub_all_index, sub_item};
#[cfg(verus_keep_ghost)]
use crate::m6_lockstep::*;
#[cfg(verus_keep_ghost)]
use crate::m6_step::{Cx, Fr, seg};
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::arg;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::BodyItem;
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};
// Contract ws D7 (cell invariance), the setup of the body frame: the walk's
// body after the head unification, instantiated by th2 at t and at u.

pub open spec fn bfr(
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
    dd: nat,
) -> Fr {
    Fr {
        stack: bstk(g, sel, j, dd),
        tht: th2(g, sel, j, t),
        thu: th2(g, sel, j, u),
        k: 0,
        n: 1 + ws::nodes_all(rbody(g, j, hoff(g, sel, j))),
    }
}

pub proof fn seg_lits(s: Seq<Goal>)
    requires
        forall|i: int| 0 <= i < s.len() ==> (#[trigger] s[i]) is Lit,
    ensures
        seg(s) == s,
    decreases s.len(),
{
    if s.len() > 0 {
        assert forall|i: int| 0 <= i < s.drop_first().len() implies (
        #[trigger] s.drop_first()[i]) is Lit by {
            assert(s.drop_first()[i] == s[i + 1]);
        }
        seg_lits(s.drop_first());
        assert(s =~= seq![s[0]] + s.drop_first());
    }
}

pub proof fn th_key(th: Seq<(nat, Term)>, x: nat) -> (i: int)
    requires
        th_val(th, x) is Some,
    ensures
        0 <= i < th.len(),
        th[i].0 == x,
    decreases th.len(),
{
    if th[0].0 == x {
        0
    } else {
        let i = th_key(th.drop_first(), x);
        assert(th.drop_first()[i] == th[i + 1]);
        i + 1
    }
}

pub proof fn th_keys_eq(a: Seq<(nat, Term)>, b: Seq<(nat, Term)>, x: nat)
    requires
        a.len() == b.len(),
        forall|i: int| 0 <= i < a.len() ==> (#[trigger] a[i]).0 == b[i].0,
    ensures
        (th_val(a, x) is Some) == (th_val(b, x) is Some),
    decreases a.len(),
{
    if a.len() > 0 {
        assert forall|i: int| 0 <= i < a.drop_first().len() implies (#[trigger] a.drop_first()[i]).0
            == b.drop_first()[i].0 by {
            assert(a.drop_first()[i] == a[i + 1]);
            assert(b.drop_first()[i] == b[i + 1]);
        }
        th_keys_eq(a.drop_first(), b.drop_first(), x);
        assert(a[0].0 == b[0].0);
    }
}

// th2's values are ground.
pub proof fn th2_ok(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>)
    requires
        ws::chain_shape(sel),
        forall|k: int| 0 <= k < g.pairs.len() ==> ground((#[trigger] g.pairs[k]).1),
        tw.len() == sel.ops.len(),
        forall|i: int| 0 <= i < tw.len() ==> ckc_spec::arith::number(#[trigger] tw[i]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
    ensures
        th_ok(th2(g, sel, j, tw)),
{
    let os = xo(g, sel, j);
    let st = slot_th(sel, tw);
    let xt = th2x(g, sel, j, tw);
    xo_props(g, sel, j);
    tvals_facts(g, sel, tw);
    tpairs_ground(g, sel, tw);
    assert forall|k: int| 0 <= k < th2(g, sel, j, tw).len() implies ground(
        (#[trigger] th2(g, sel, j, tw)[k]).1,
    ) by {
        if k < st.len() {
            crate::k3_sound::log::number_ground(tw[k]);
        } else {
            let o = os[k - st.len()];
            assert(th2(g, sel, j, tw)[k] == xt[k - st.len()]);
            tpairs_lookup(g, sel, tw, o);
            lookup_ground(tpairs(g, sel, tw), o);
        }
    }
}

pub proof fn th2_dom_eq(g: spec::Group, sel: ws::Sel, j: int, t: Seq<Term>, u: Seq<Term>, x: nat)
    requires
        ws::chain_shape(sel),
        ws::counts(sel).no_duplicates(),
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
    ensures
        (th_val(th2(g, sel, j, t), x) is Some) == (th_val(th2(g, sel, j, u), x) is Some),
{
    let xt = th2x(g, sel, j, t);
    let xu = th2x(g, sel, j, u);
    th_cat(slot_th(sel, t), xt, x);
    th_cat(slot_th(sel, u), xu, x);
    slot_val(sel, t, x);
    slot_val(sel, u, x);
    th_keys_eq(xt, xu, x);
}

pub proof fn th2_below(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    tw: Seq<Term>,
    x: nat,
)
    requires
        ws::chain_shape(sel),
        ws::fixed_skeleton(g, sel),
        ws::chain_body(sel, g.clauses[j].body),
        0 <= j < g.clauses.len(),
        tw.len() == sel.ops.len(),
        th_val(th2(g, sel, j, tw), x) is Some,
    ensures
        x < wenv(p, g, sel, j).off,
{
    let st = slot_th(sel, tw);
    let xt = th2x(g, sel, j, tw);
    let os = xo(g, sel, j);
    let cs = ws::counts(sel);
    assert(cs.no_duplicates());
    th_cat(st, xt, x);
    slot_val(sel, tw, x);
    if th_val(st, x) is Some {
        let i = choose|i: int| 0 <= i < cs.len() && cs[i] == Term::Var(x);
        assert(cs[i] == sel.ops[i].n);
        count_below(g, sel, i);
    } else {
        let k = th_key(xt, x);
        xo_props(g, sel, j);
        xo_in_fs(g, sel, j, k);
        let o = os[k];
        assert(xt[k].0 == hoff(g, sel, j) + ckc_spec::trace::pos_of(fsj(g, j), o->Var_0));
        let idx = choose|idx: int| 0 <= idx < ws::outputs(sel).len() && ws::outputs(sel)[idx] == o;
        crate::m6_default::outputs_index(sel);
        let m = idx / 2;
        if idx % 2 == 0 {
            assert(idx == 2 * m);
            out_in(g, sel, j, m, 4);
        } else {
            assert(idx == 2 * m + 1);
            out_in(g, sel, j, m, 5);
        }
    }
}

pub proof fn th2_slots(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>, x: nat)
    requires
        ws::chain_shape(sel),
        ws::counts(sel).no_duplicates(),
        tw.len() == sel.ops.len(),
        th_val(slot_th(sel, tw), x) is Some,
    ensures
        th_val(th2(g, sel, j, tw), x) == th_val(slot_th(sel, tw), x),
{
    th_cat(slot_th(sel, tw), th2x(g, sel, j, tw), x);
}

// --- the body's arithmetic goals in chain order ---
pub open spec fn ap(it: BodyItem) -> bool {
    it is Pos && ws::is_arith_lit(it->Pos_0)
}

// Arithmetic items before position i.
pub open spec fn acount(items: Seq<BodyItem>, i: int) -> nat
    decreases i,
{
    if i <= 0 {
        0
    } else {
        acount(items, i - 1) + if ap(items[i - 1]) {
            1nat
        } else {
            0nat
        }
    }
}

pub open spec fn aseq(items: Seq<BodyItem>) -> Seq<Term> {
    spec::pos_terms(items.filter(|it: BodyItem| it is Pos)).filter(|t: Term| ws::is_arith_lit(t))
}

proof fn acount_prefix(items: Seq<BodyItem>, x: BodyItem, i: int)
    requires
        0 <= i <= items.len(),
    ensures
        acount(items.push(x), i) == acount(items, i),
    decreases i,
{
    if i > 0 {
        acount_prefix(items, x, i - 1);
        assert(items.push(x)[i - 1] == items[i - 1]);
    }
}

pub proof fn filter_push<A>(s: Seq<A>, x: A, f: spec_fn(A) -> bool)
    ensures
        s.push(x).filter(f) == if f(x) {
            s.filter(f).push(x)
        } else {
            s.filter(f)
        },
{
    reveal(Seq::filter);
    assert(s.push(x).drop_last() == s);
    assert(s.push(x).last() == x);
}

proof fn filter_empty<A>(s: Seq<A>, f: spec_fn(A) -> bool)
    requires
        s.len() == 0,
    ensures
        s.filter(f).len() == 0,
{
    reveal(Seq::filter);
}

// One more item: the arithmetic terms grow by its term exactly when it is
// arithmetic.
proof fn aseq_push(init: Seq<BodyItem>, last: BodyItem)
    ensures
        aseq(init.push(last)) == if ap(last) {
            aseq(init).push(last->Pos_0)
        } else {
            aseq(init)
        },
{
    let fp = |it: BodyItem| it is Pos;
    let fa = |t: Term| ws::is_arith_lit(t);
    let pi = spec::pos_terms(init.filter(fp));
    filter_push(init, last, fp);
    if last is Pos {
        let pf = |it: BodyItem|
            match it {
                BodyItem::Pos(t) => t,
                BodyItem::Naf(_) => Term::Nil,
            };
        assert(init.filter(fp).push(last).map_values(pf) =~= pi.push(last->Pos_0));
        filter_push(pi, last->Pos_0, fa);
    }
}

proof fn acount_mono(items: Seq<BodyItem>, i: int, k: int)
    requires
        0 <= i <= k <= items.len(),
    ensures
        acount(items, i) <= acount(items, k),
    decreases k - i,
{
    if i < k {
        acount_mono(items, i, k - 1);
    }
}

proof fn acount_lt(items: Seq<BodyItem>, i: int)
    requires
        0 <= i < items.len(),
        ap(items[i]),
    ensures
        acount(items, i) < acount(items, items.len() as int),
{
    acount_mono(items, i + 1, items.len() as int);
    assert(acount(items, i + 1) == acount(items, i) + 1);
}

// The arithmetic terms of a body in order: their count, and item i's term at
// its arithmetic rank.
#[verifier::spinoff_prover]
pub proof fn aseq_at(items: Seq<BodyItem>)
    ensures
        aseq(items).len() == acount(items, items.len() as int),
        forall|i: int|
            0 <= i < items.len() && ap(#[trigger] items[i]) ==> aseq(items)[acount(items, i) as int]
                == items[i]->Pos_0,
    decreases items.len(),
{
    if items.len() == 0 {
        let fp = |it: BodyItem| it is Pos;
        let fa = |t: Term| ws::is_arith_lit(t);
        filter_empty(items, fp);
        filter_empty(spec::pos_terms(items.filter(fp)), fa);
    } else {
        let init = items.drop_last();
        let last = items.last();
        let n = items.len() as int;
        assert(items == init.push(last));
        aseq_at(init);
        aseq_push(init, last);
        acount_prefix(init, last, n - 1);
        assert(acount(items, n) == acount(items, n - 1) + if ap(items[n - 1]) {
            1nat
        } else {
            0nat
        });
        assert forall|i: int| 0 <= i < items.len() && ap(#[trigger] items[i]) implies aseq(
            items,
        )[acount(items, i) as int] == items[i]->Pos_0 by {
            if i < n - 1 {
                assert(items[i] == init[i]);
                acount_prefix(init, last, i);
                acount_lt(init, i);
            }
        }
    }
}

// --- the images of the clause's variables in the walk's body ---
pub proof fn flatten_contains(ss: Seq<Seq<Term>>, i: int, k: int)
    requires
        0 <= i < ss.len(),
        0 <= k < ss[i].len(),
    ensures
        ss.flatten().contains(ss[i][k]),
    decreases ss.len(),
{
    let rest = ss.drop_first();
    assert(ss.flatten() == ss.first() + rest.flatten());
    if i == 0 {
        assert(ss.flatten()[k] == ss[0][k]);
    } else {
        assert(rest[i - 1] == ss[i]);
        flatten_contains(rest, i - 1, k);
        let q = choose|q: int| 0 <= q < rest.flatten().len() && rest.flatten()[q] == ss[i][k];
        assert(ss.flatten()[ss.first().len() + q] == ss[i][k]);
    }
}

// A goal of a negation item: no arithmetic literal, no chain output.
pub proof fn naf_goal_facts(sel: ws::Sel, body: Seq<BodyItem>, i: int, k: int, m: int)
    requires
        ws::chain_body(sel, body),
        0 <= i < body.len(),
        body[i] is Naf,
        0 <= k < body[i]->Naf_0.len(),
        0 <= m < ws::outputs(sel).len(),
    ensures
        !ws::is_arith_lit(body[i]->Naf_0[k]),
        !ws::mentions(body[i]->Naf_0[k], ws::outputs(sel)[m]),
{
    let gs = body[i]->Naf_0;
    let ss = body.map_values(
        |it: BodyItem|
            match it {
                BodyItem::Naf(gs) => gs,
                BodyItem::Pos(_) => Seq::empty(),
            },
    );
    assert(ws::naf_goals(body) == ss.flatten());
    assert(ss[i] == gs);
    flatten_contains(ss, i, k);
    let q = choose|q: int| 0 <= q < ss.flatten().len() && ss.flatten()[q] == gs[k];
    let pos = spec::pos_terms(body.filter(|it: BodyItem| it is Pos));
    let others = pos.filter(|t: Term| !ws::is_arith_lit(t)) + ws::naf_goals(body);
    assert(others[pos.filter(|t: Term| !ws::is_arith_lit(t)).len() + q] == gs[k]);
}

// A positive item that is not arithmetic mentions no chain output.
pub proof fn pos_goal_facts(sel: ws::Sel, body: Seq<BodyItem>, i: int, m: int)
    requires
        ws::chain_body(sel, body),
        0 <= i < body.len(),
        body[i] is Pos,
        !ws::is_arith_lit(body[i]->Pos_0),
        0 <= m < ws::outputs(sel).len(),
    ensures
        !ws::mentions(body[i]->Pos_0, ws::outputs(sel)[m]),
{
    let fp = |it: BodyItem| it is Pos;
    let fa = |t: Term| !ws::is_arith_lit(t);
    let l = body[i]->Pos_0;
    let bp = body.filter(fp);
    let pos = spec::pos_terms(bp);
    body.lemma_filter_contains(fp, i);
    let k = choose|k: int| 0 <= k < bp.len() && bp[k] == body[i];
    assert(pos[k] == l);
    pos.lemma_filter_contains(fa, k);
    let k2 = choose|k2: int| 0 <= k2 < pos.filter(fa).len() && pos.filter(fa)[k2] == l;
    let others = pos.filter(fa) + ws::naf_goals(body);
    assert(others[k2] == l);
}

// A variable of a substituted term comes from a variable's image.
pub proof fn sub_occ(t: Term, m: spec_fn(nat) -> Term, x: nat) -> (v: nat)
    requires
        occurs(x, sub(t, m)),
    ensures
        occurs(v, t),
        occurs(x, m(v)),
    decreases t,
{
    match t {
        Term::Var(v) => v,
        Term::Comp(n, args) => {
            sub_all_index(args, m);
            let k = occurs_index(x, sub_all(args, m));
            let v = sub_occ(args[k], m, x);
            occurs_member(v, args, k);
            v
        },
        _ => {
            assert(false);
            0
        },
    }
}

// An output's image in the walk's body: its own copy.
pub proof fn out_img(g: spec::Group, sel: ws::Sel, j: int, mm: int, q: int)
    requires
        ws::chain_shape(sel),
        ws::chain_body(sel, g.clauses[j].body),
        0 <= j < g.clauses.len(),
        0 <= mm < sel.chain.len(),
        q == 4 || q == 5,
        g.clauses[j].head is Comp,
    ensures
        sub(sub(arg(sel.chain[mm], q), rmap(fsj(g, j), hoff(g, sel, j))), apend(g, sel, j))
            == Term::Var(ph(g, sel, j, q)[mm]),
{
    let fs = fsj(g, j);
    let ho = hoff(g, sel, j);
    let y = arg(sel.chain[mm], q)->Var_0;
    out_in(g, sel, j, mm, q);
    let hp = hpart(g, sel);
    let ts = g.clauses[j].head->Comp_1;
    let maa = crate::m6_head::lmap(hp);
    let mba = crate::m6_head::rmap(fs, ho);
    let ra = rr(fs, ho);
    let in0 = hin(ts);
    let dom = |w: nat| fs.contains(w);
    assert forall|w: nat| #[trigger] in0(w) implies dom(w) by {
        assert(occurs(w, g.clauses[j].head));
        head_in_fs(g, j, w);
    }
    assert forall|w1: nat, w2: nat|
        dom(w1) && dom(w2) && #[trigger] ra(w1) == #[trigger] ra(w2) implies w1 == w2 by {
        pos_of_at(fs, w1);
        pos_of_at(fs, w2);
    }
    pend_at(maa, mba, ra, in0, y, dom);
    let fh = |p: (Term, Term)| !ws::counts(sel).contains(p.0) && !ws::outputs(sel).contains(p.0);
    assert(hp == g.pairs.filter(fh));
    crate::m6_default::outputs_index(sel);
    if q == 4 {
        assert(ws::outputs(sel)[2 * mm] == Term::Var(y));
    } else {
        assert(ws::outputs(sel)[2 * mm + 1] == Term::Var(y));
    }
    lookup_drop(g.pairs, fh, Term::Var(y));
    assert(maa(y) == Term::Var(y));
}

// Input pair q of node kk reads node m's output ⇒ it is m's output pair.
pub proof fn from_out(sel: ws::Sel, kk: int, q: int, m: int)
    requires
        ws::chain_shape(sel),
        0 <= m < kk < sel.chain.len(),
        q == 0 || q == 2,
        arg(sel.chain[kk], q + 1) == arg(sel.chain[m], 5),
    ensures
        arg(sel.chain[kk], q) == arg(sel.chain[m], 4),
{
    let s = arg(sel.chain[kk], q);
    let n = arg(sel.chain[kk], q + 1);
    assert(ws::chain_input(sel, kk, s, n));
    keys_index(sel);
    crate::m6_default::outputs_index(sel);
    let nops = sel.ops.len() as int;
    assert(keys(sel).no_duplicates());
    assert(keys(sel)[2 * nops + 2 * m + 1] == n);
    if exists|i: int|
        0 <= i < sel.ops.len() && s == (#[trigger] sel.ops[i]).s && n == sel.ops[i].n {
        let i = choose|i: int|
            0 <= i < sel.ops.len() && s == (#[trigger] sel.ops[i]).s && n == sel.ops[i].n;
        assert(keys(sel)[nops + i] == n);
    } else if s == ws::exactly() && ckc_spec::arith::number(n) is Some {
        assert(n is Var);
    } else {
        let m2 = choose|m2: int|
            0 <= m2 < kk && s == arg(#[trigger] sel.chain[m2], 4) && n == arg(sel.chain[m2], 5);
        assert(keys(sel)[2 * nops + 2 * m2 + 1] == n);
    }
}

// --- the body frame's goal shapes ---
// The walk's body item i: its item term under the two maps.
proof fn wbody_at(g: spec::Group, sel: ws::Sel, j: int, i: int)
    requires
        0 <= j < g.clauses.len(),
        0 <= i < g.clauses[j].body.len(),
    ensures
        wbody(g, sel, j)[i] == sub(
            sub_item(g.clauses[j].body[i], rmap(fsj(g, j), hoff(g, sel, j))),
            apend(g, sel, j),
        ),
{
}

// Arithmetic item i: its goal sits on node acount(i)'s placeholders.
pub proof fn node_item(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
    i: int,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        g.clauses[j].head is Comp,
        0 <= i < g.clauses[j].body.len(),
        ap(g.clauses[j].body[i]),
    ensures
        crate::m6_step::node_shape(
            mk_cx(p, g, sel, j, t, u),
            wbody(g, sel, j)[i],
            acount(g.clauses[j].body, i),
        ),
{
    let cx = mk_cx(p, g, sel, j, t, u);
    let body = g.clauses[j].body;
    let kk = acount(body, i) as int;
    let fs = fsj(g, j);
    let ho = hoff(g, sel, j);
    let m1 = rmap(fs, ho);
    let m2 = apend(g, sel, j);
    assert(ws::chain_body(sel, body));
    aseq_at(body);
    acount_lt(body, i);
    assert(aseq(body) == sel.chain);
    let l = body[i]->Pos_0;
    assert(l == sel.chain[kk]);
    wbody_at(g, sel, j, i);
    let w = wbody(g, sel, j)[i];
    assert(w == sub(sub(l, m1), m2));
    assert(ws::is_arith_lit(l));
    let n = l->Comp_0;
    let a = l->Comp_1;
    assert(a.len() == 6) by {
        reveal(ckc_spec::arith::is_arith);
    }
    sub_all_index(a, m1);
    sub_all_index(sub_all(a, m1), m2);
    assert(w == Term::Comp(n, sub_all(sub_all(a, m1), m2)));
    assert forall|q: int| 0 <= q < 6 implies arg(w, q) == sub(sub(arg(l, q), m1), m2) by {}
    out_img(g, sel, j, kk, 4);
    out_img(g, sel, j, kk, 5);
    assert forall|m: int| 0 <= m < kk implies #[trigger] crate::m6_step::from_node(
        cx,
        w,
        kk as nat,
        0,
        m,
    ) && crate::m6_step::from_node(cx, w, kk as nat, 2, m) by {
        if arg(sel.chain[kk], 1) == arg(sel.chain[m], 5) {
            from_out(sel, kk, 0, m);
            out_img(g, sel, j, m, 4);
            out_img(g, sel, j, m, 5);
        }
        if arg(sel.chain[kk], 3) == arg(sel.chain[m], 5) {
            from_out(sel, kk, 2, m);
            out_img(g, sel, j, m, 4);
            out_img(g, sel, j, m, 5);
        }
    }
}

// A variable of the walk's body item i is a clause variable's copy.
pub proof fn wbody_var(g: spec::Group, sel: ws::Sel, j: int, i: int, x: nat) -> (v: nat)
    requires
        ws::chain_shape(sel),
        0 <= j < g.clauses.len(),
        g.clauses[j].head is Comp,
        0 <= i < g.clauses[j].body.len(),
        ws::primitive_body(g.clauses[j].body),
        occurs(x, wbody(g, sel, j)[i]),
    ensures
        item_occurs(v, g.clauses[j].body[i]),
        x == hoff(g, sel, j) + ckc_spec::trace::pos_of(fsj(g, j), v),
{
    let it = g.clauses[j].body[i];
    let fs = fsj(g, j);
    let ho = hoff(g, sel, j);
    let m1 = rmap(fs, ho);
    let m2 = apend(g, sel, j);
    wbody_at(g, sel, j, i);
    let v1 = sub_occ(sub_item(it, m1), m2, x);
    let v = match it {
        BodyItem::Pos(l) => {
            let v = sub_occ(l, m1, v1);
            v
        },
        BodyItem::Naf(gs) => {
            assert(gs.len() > 0);
            let ss = sub_all(gs, m1);
            sub_all_index(gs, m1);
            let one = seq![conj_term(ss)];
            reveal_with_fuel(occurs_all, 2);
            assert(sub_item(it, m1) == Term::Comp(naf_name(), one));
            assert(occurs_all(v1, one));
            assert(one[0] == conj_term(ss));
            assert(one.drop_first() =~= Seq::<Term>::empty());
            assert(occurs(v1, conj_term(ss)));
            conj_vars(ss, v1);
            let k = occurs_index(v1, ss);
            let v = sub_occ(gs[k], m1, v1);
            occurs_member(v, gs, k);
            v
        },
    };
    assert(v1 == ho + ckc_spec::trace::pos_of(fs, v));
    // the pending map at v's copy: ground (no variable) or the copy itself
    item_in_fs(g, j, i, v);
    let hp = hpart(g, sel);
    let ts = g.clauses[j].head->Comp_1;
    let maa = crate::m6_head::lmap(hp);
    let ra = rr(fs, ho);
    let in0 = hin(ts);
    let dom = |w: nat| fs.contains(w);
    assert forall|w: nat| #[trigger] in0(w) implies dom(w) by {
        assert(occurs(w, g.clauses[j].head));
        head_in_fs(g, j, w);
    }
    assert forall|w1: nat, w2: nat|
        dom(w1) && dom(w2) && #[trigger] ra(w1) == #[trigger] ra(w2) implies w1 == w2 by {
        pos_of_at(fs, w1);
        pos_of_at(fs, w2);
    }
    pend_at(maa, m1, ra, in0, v, dom);
    if crate::m6_head::pending(maa, m1, ra, in0, v) {
        crate::m6_default::ground_absent(maa(v), x);
    }
    v
}

// A goal of a non-arithmetic item holds no placeholder.
pub proof fn no_ph(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
    i: int,
    x: nat,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        g.clauses[j].head is Comp,
        0 <= i < g.clauses[j].body.len(),
        !ap(g.clauses[j].body[i]),
        occurs(x, wbody(g, sel, j)[i]),
    ensures
        !crate::m6_step::is_ph(mk_cx(p, g, sel, j, t, u), x),
{
    let cx = mk_cx(p, g, sel, j, t, u);
    let body = g.clauses[j].body;
    let it = body[i];
    let fs = fsj(g, j);
    assert(ws::chain_body(sel, body));
    assert(ws::primitive_body(body));
    let v = wbody_var(g, sel, j, i, x);
    item_in_fs(g, j, i, v);
    pos_of_at(fs, v);
    crate::m6_default::outputs_index(sel);
    if crate::m6_step::is_ph(cx, x) {
        let (m, q) = if cx.o4.contains(x) {
            let m = choose|m: int| 0 <= m < cx.o4.len() && cx.o4[m] == x;
            (m, 4int)
        } else {
            let m = choose|m: int| 0 <= m < cx.o5.len() && cx.o5[m] == x;
            (m, 5int)
        };
        out_in(g, sel, j, m, q);
        let y = arg(sel.chain[m], q)->Var_0;
        pos_of_at(fs, y);
        assert(y == v);
        let idx = if q == 4 {
            2 * m
        } else {
            2 * m + 1
        };
        assert(ws::outputs(sel)[idx] == Term::Var(v));
        match it {
            BodyItem::Pos(l) => {
                pos_goal_facts(sel, body, i, idx);
            },
            BodyItem::Naf(gs) => {
                let k = occurs_index(v, gs);
                naf_goal_facts(sel, body, i, k, idx);
            },
        }
    }
}

// A primitive, non-arithmetic goal under the two maps, free of
// placeholders, is pure.
proof fn pure_leaf(cx: Cx, g0: Term, m1: spec_fn(nat) -> Term, m2: spec_fn(nat) -> Term)
    requires
        ws::primitive(g0),
        !ws::is_arith_lit(g0),
        forall|x: nat| #[trigger] occurs(x, sub(sub(g0, m1), m2)) ==> !crate::m6_step::is_ph(cx, x),
    ensures
        crate::m6_step::pure(cx, sub(sub(g0, m1), m2)),
{
    let n = g0->Comp_0;
    let a = g0->Comp_1;
    sub_all_index(a, m1);
    sub_all_index(sub_all(a, m1), m2);
    assert(sub(sub(g0, m1), m2) == Term::Comp(n, sub_all(sub_all(a, m1), m2)));
}

// The conjunction of a negation item's goals under the two maps is pure.
proof fn pure_conj(cx: Cx, gs: Seq<Term>, m1: spec_fn(nat) -> Term, m2: spec_fn(nat) -> Term)
    requires
        gs.len() > 0,
        forall|k: int|
            0 <= k < gs.len() ==> ws::primitive(#[trigger] gs[k]) && !ws::is_arith_lit(gs[k]),
        forall|x: nat| #[trigger]
            occurs(x, sub(conj_term(sub_all(gs, m1)), m2)) ==> !crate::m6_step::is_ph(cx, x),
    ensures
        crate::m6_step::pure(cx, sub(conj_term(sub_all(gs, m1)), m2)),
    decreases gs.len(),
{
    sub_all_index(gs, m1);
    let ss = sub_all(gs, m1);
    if gs.len() == 1 {
        assert(conj_term(ss) == sub(gs[0], m1));
        pure_leaf(cx, gs[0], m1, m2);
    } else {
        let rest = gs.drop_first();
        assert(ss.drop_first() =~= sub_all(rest, m1));
        let l = seq![sub(gs[0], m1), conj_term(sub_all(rest, m1))];
        assert(conj_term(ss) == Term::Comp(comma_name(), l));
        sub_all_index(l, m2);
        let whole = sub(conj_term(ss), m2);
        assert(whole == Term::Comp(comma_name(), sub_all(l, m2)));
        let a0 = sub(sub(gs[0], m1), m2);
        let a1 = sub(conj_term(sub_all(rest, m1)), m2);
        assert(sub_all(l, m2)[0] == a0 && sub_all(l, m2)[1] == a1);
        assert forall|x: nat| #[trigger] occurs(x, a0) implies !crate::m6_step::is_ph(cx, x) by {
            occurs_member(x, sub_all(l, m2), 0);
            assert(occurs(x, whole));
        }
        assert forall|x: nat| #[trigger] occurs(x, a1) implies !crate::m6_step::is_ph(cx, x) by {
            occurs_member(x, sub_all(l, m2), 1);
            assert(occurs(x, whole));
        }
        pure_leaf(cx, gs[0], m1, m2);
        assert forall|k: int| 0 <= k < rest.len() implies ws::primitive(#[trigger] rest[k])
            && !ws::is_arith_lit(rest[k]) by {
            assert(rest[k] == gs[k + 1]);
        }
        pure_conj(cx, rest, m1, m2);
    }
}

// A non-arithmetic item's goal is pure.
pub proof fn pure_item(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
    i: int,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        g.clauses[j].head is Comp,
        0 <= i < g.clauses[j].body.len(),
        !ap(g.clauses[j].body[i]),
    ensures
        crate::m6_step::pure(mk_cx(p, g, sel, j, t, u), wbody(g, sel, j)[i]),
{
    let cx = mk_cx(p, g, sel, j, t, u);
    let body = g.clauses[j].body;
    let it = body[i];
    let fs = fsj(g, j);
    let ho = hoff(g, sel, j);
    let m1 = rmap(fs, ho);
    let m2 = apend(g, sel, j);
    assert(ws::primitive_body(body));
    assert(ws::chain_body(sel, body));
    wbody_at(g, sel, j, i);
    let w = wbody(g, sel, j)[i];
    assert forall|x: nat| #[trigger] occurs(x, w) implies !crate::m6_step::is_ph(cx, x) by {
        no_ph(p, g, sel, j, t, u, i, x);
    }
    match it {
        BodyItem::Pos(l) => {
            pure_leaf(cx, l, m1, m2);
        },
        BodyItem::Naf(gs) => {
            let c = conj_term(sub_all(gs, m1));
            assert(sub_item(it, m1) == Term::Comp(naf_name(), seq![c]));
            sub_all_index(seq![c], m2);
            assert(w == Term::Comp(naf_name(), seq![sub(c, m2)]));
            assert forall|x: nat| #[trigger] occurs(x, sub(c, m2)) implies !crate::m6_step::is_ph(
                cx,
                x,
            ) by {
                occurs_member(x, seq![sub(c, m2)], 0);
                assert(occurs(x, w));
            }
            crate::m6_default::outputs_index(sel);
            assert forall|k: int| 0 <= k < gs.len() implies ws::primitive(#[trigger] gs[k])
                && !ws::is_arith_lit(gs[k]) by {
                assert(ws::outputs(sel).len() == 2 * sel.chain.len());
                if ws::outputs(sel).len() > 0 {
                    naf_goal_facts(sel, body, i, k, 0);
                } else {
                    let ss = body.map_values(
                        |it2: BodyItem|
                            match it2 {
                                BodyItem::Naf(gs2) => gs2,
                                BodyItem::Pos(_) => Seq::empty(),
                            },
                    );
                    assert(ws::naf_goals(body) == ss.flatten());
                    assert(ss[i] == gs);
                    flatten_contains(ss, i, k);
                }
            }
            pure_conj(cx, gs, m1, m2);
        },
    }
}

// --- the body frame: the walk's invariants ---
proof fn arith_item(g: spec::Group, sel: ws::Sel, j: int, i: int)
    requires
        0 <= j < g.clauses.len(),
        0 <= i < g.clauses[j].body.len(),
        ws::primitive_body(g.clauses[j].body),
    ensures
        ws::is_arith_lit(wbody(g, sel, j)[i]) == ap(g.clauses[j].body[i]),
        crate::m6_fragment::structured(wbody(g, sel, j)[i]),
{
    let it = g.clauses[j].body[i];
    let m1 = rmap(fsj(g, j), hoff(g, sel, j));
    let m2 = apend(g, sel, j);
    wbody_at(g, sel, j, i);
    let w = wbody(g, sel, j)[i];
    match it {
        BodyItem::Pos(l) => {
            let n = l->Comp_0;
            let a = l->Comp_1;
            sub_all_index(a, m1);
            sub_all_index(sub_all(a, m1), m2);
            assert(w == Term::Comp(n, sub_all(sub_all(a, m1), m2)));
        },
        BodyItem::Naf(gs) => {
            let c = conj_term(sub_all(gs, m1));
            assert(sub_item(it, m1) == Term::Comp(naf_name(), seq![c]));
            sub_all_index(seq![c], m2);
            assert(w == Term::Comp(naf_name(), seq![sub(c, m2)]));
            reveal(ckc_spec::arith::is_arith);
            structured_conj(gs, m1, m2);
        },
    }
}

proof fn structured_conj(gs: Seq<Term>, m1: spec_fn(nat) -> Term, m2: spec_fn(nat) -> Term)
    requires
        gs.len() > 0,
        forall|k: int| 0 <= k < gs.len() ==> ws::primitive(#[trigger] gs[k]),
    ensures
        crate::m6_fragment::structured(sub(conj_term(sub_all(gs, m1)), m2)),
    decreases gs.len(),
{
    sub_all_index(gs, m1);
    let ss = sub_all(gs, m1);
    let g0 = gs[0];
    sub_all_index(g0->Comp_1, m1);
    sub_all_index(sub_all(g0->Comp_1, m1), m2);
    assert(sub(sub(g0, m1), m2) == Term::Comp(g0->Comp_0, sub_all(sub_all(g0->Comp_1, m1), m2)));
    assert(ws::primitive(g0));
    assert(crate::m6_fragment::structured(sub(sub(g0, m1), m2)));
    if gs.len() > 1 {
        let rest = gs.drop_first();
        assert(ss.drop_first() =~= sub_all(rest, m1));
        let l = seq![sub(gs[0], m1), conj_term(sub_all(rest, m1))];
        assert(conj_term(ss) == Term::Comp(comma_name(), l));
        sub_all_index(l, m2);
        let whole = sub(conj_term(ss), m2);
        assert(whole == Term::Comp(comma_name(), sub_all(l, m2)));
        assert(sub_all(l, m2)[0] == sub(sub(gs[0], m1), m2));
        assert(sub_all(l, m2)[1] == sub(conj_term(sub_all(rest, m1)), m2));
        assert forall|k: int| 0 <= k < rest.len() implies ws::primitive(#[trigger] rest[k]) by {
            assert(rest[k] == gs[k + 1]);
        }
        structured_conj(rest, m1, m2);
    } else {
        assert(conj_term(ss) == ss[0]);
    }
}

proof fn arank_acount(g: spec::Group, sel: ws::Sel, j: int, dd: nat, i: int)
    requires
        0 <= j < g.clauses.len(),
        0 <= i <= g.clauses[j].body.len(),
        ws::primitive_body(g.clauses[j].body),
    ensures
        crate::m6_step::arank(bstk(g, sel, j, dd), i) == acount(g.clauses[j].body, i),
    decreases i,
{
    if i > 0 {
        arank_acount(g, sel, j, dd, i - 1);
        arith_item(g, sel, j, i - 1);
        assert(bstk(g, sel, j, dd)[i - 1] == Goal::Lit(wbody(g, sel, j)[i - 1], dd));
    }
}

pub proof fn si_bfr(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
    dd: nat,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        g.clauses[j].head is Comp,
    ensures
        crate::m6_step::si(mk_cx(p, g, sel, j, t, u), bfr(g, sel, j, t, u, dd)),
        crate::m6_fragment::structured_upto(
            crate::m6_cell::enc(seg(bstk(g, sel, j, dd))),
            seg(bstk(g, sel, j, dd)).len() as int,
        ),
        crate::m6_cell::enc(seg(bstk(g, sel, j, dd))) == wbody(g, sel, j),
{
    let cx = mk_cx(p, g, sel, j, t, u);
    let fr = bfr(g, sel, j, t, u, dd);
    let s = bstk(g, sel, j, dd);
    let body = g.clauses[j].body;
    assert(ws::primitive_body(body));
    seg_lits(s);
    assert(crate::m6_cell::enc(s) =~= wbody(g, sel, j));
    assert forall|i: int| 0 <= i < seg(s).len() implies #[trigger] crate::m6_step::goal_si(
        cx,
        seg(s)[i],
        fr.k + crate::m6_step::arank(seg(s), i),
    ) by {
        arank_acount(g, sel, j, dd, i);
        arith_item(g, sel, j, i);
        assert(s[i] == Goal::Lit(wbody(g, sel, j)[i], dd));
        if ap(body[i]) {
            node_item(p, g, sel, j, t, u, i);
        } else {
            pure_item(p, g, sel, j, t, u, i);
        }
    }
    assert forall|i: int| 0 <= i < seg(s).len() implies crate::m6_fragment::structured(
        #[trigger] crate::m6_cell::enc(seg(s))[i],
    ) by {
        arith_item(g, sel, j, i);
    }
}

// The body frame's placeholders carry the chain values at t and at u.
pub proof fn th_vals_bfr(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
    dd: nat,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        g.clauses[j].head is Comp,
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> ckc_spec::arith::number(#[trigger] t[i]) is Some,
        forall|i: int| 0 <= i < u.len() ==> ckc_spec::arith::number(#[trigger] u[i]) is Some,
        ws::tuple_pairs(g, sel, t) is Some,
        ws::tuple_pairs(g, sel, u) is Some,
    ensures
        crate::m6_step::th_vals(mk_cx(p, g, sel, j, t, u), bfr(g, sel, j, t, u, dd)),
{
    let cx = mk_cx(p, g, sel, j, t, u);
    let fr = bfr(g, sel, j, t, u, dd);
    let os = xo(g, sel, j);
    let st = slot_th(sel, t);
    let xt = th2x(g, sel, j, t);
    let cs = ws::counts(sel);
    assert(cs.no_duplicates());
    xo_props(g, sel, j);
    crate::m6_default::outputs_index(sel);
    tvals_facts(g, sel, t);
    tvals_facts(g, sel, u);
    assert forall|x: nat| #[trigger] th_val(fr.tht, x) is Some implies th_val(cx.st, x) is Some
        || crate::m6_step::is_ph(cx, x) by {
        th_cat(st, xt, x);
        slot_val(sel, t, x);
        if th_val(st, x) is None {
            let k = th_key(xt, x);
            let o = os[k];
            assert(xt[k].0 == hoff(g, sel, j) + ckc_spec::trace::pos_of(fsj(g, j), o->Var_0));
            let idx = choose|idx: int|
                0 <= idx < ws::outputs(sel).len() && ws::outputs(sel)[idx] == o;
            let m = idx / 2;
            if idx % 2 == 0 {
                assert(idx == 2 * m);
                assert(cx.o4[m] == x);
            } else {
                assert(idx == 2 * m + 1);
                assert(cx.o5[m] == x);
            }
        }
    }
    assert forall|m: int|
        0 <= m < cx.e.chain.len() && #[trigger] th_val(fr.tht, cx.o5[m]) is Some implies th_val(
        fr.tht,
        cx.o5[m],
    ) == spec::lookup(cx.vt, arg(cx.e.chain[m], 5)) && th_val(fr.thu, cx.o5[m]) == spec::lookup(
        cx.vu,
        arg(cx.e.chain[m], 5),
    ) by {
        out_in(g, sel, j, m, 5);
        let y = arg(sel.chain[m], 5)->Var_0;
        th2_at(g, sel, j, t, y);
        th2_at(g, sel, j, u, y);
        tpairs_lookup(g, sel, t, Term::Var(y));
        tpairs_lookup(g, sel, u, Term::Var(y));
        if os.contains(Term::Var(y)) {
            let k = choose|k: int| 0 <= k < os.len() && os[k] == Term::Var(y);
            assert(spec::lookup(g.pairs, os[k]) is Some);
        }
    }
    assert forall|m: int|
        0 <= m < cx.e.chain.len() && #[trigger] th_val(fr.tht, cx.o4[m]) is Some implies th_val(
        fr.tht,
        cx.o4[m],
    ) == spec::lookup(cx.vt, arg(cx.e.chain[m], 4)) && th_val(fr.thu, cx.o4[m]) == spec::lookup(
        cx.vu,
        arg(cx.e.chain[m], 4),
    ) by {
        out_in(g, sel, j, m, 4);
        let y = arg(sel.chain[m], 4)->Var_0;
        th2_at(g, sel, j, t, y);
        th2_at(g, sel, j, u, y);
        tpairs_lookup(g, sel, t, Term::Var(y));
        tpairs_lookup(g, sel, u, Term::Var(y));
        if os.contains(Term::Var(y)) {
            let k = choose|k: int| 0 <= k < os.len() && os[k] == Term::Var(y);
            assert(spec::lookup(g.pairs, os[k]) is Some);
        }
    }
}

} // verus!
