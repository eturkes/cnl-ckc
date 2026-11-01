#[cfg(verus_keep_ghost)]
use crate::k3_sound::naf::{lift_var, rename};
#[cfg(verus_keep_ghost)]
use crate::m6_body::*;
#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use crate::m6_ctx::*;
#[cfg(verus_keep_ghost)]
use crate::m6_default::*;
#[cfg(verus_keep_ghost)]
use crate::m6_hd::*;
#[cfg(verus_keep_ghost)]
use crate::m6_head::{lmap, occurs_index, occurs_member, pend, pending, rmap, sub, sub_item};
#[cfg(verus_keep_ghost)]
use crate::m6_lockstep::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::BodyItem;
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};
// Contract ws D7 (cell invariance), the setup of the head call's entry: each
// clause variable's image after the head unification, concrete renamed
// against the walk's instantiated by th2.
// A variable of a clause item lies in the clause's first-occurrence list.

pub proof fn item_var_in2(items: Seq<BodyItem>, i: int, y: nat)
    requires
        0 <= i < items.len(),
        item_occurs(y, items[i]),
    ensures
        ckc_spec::v1text::body_var_stream(items).contains(y),
    decreases items.len(),
{
    let bs = ckc_spec::v1text::body_var_stream(items);
    let head = ckc_spec::v1text::item_var_stream(items[0]);
    let rest = ckc_spec::v1text::body_var_stream(items.drop_first());
    assert(bs == head + rest);
    if i == 0 {
        match items[0] {
            BodyItem::Pos(l) => crate::k3_safe::stream_occurs(l, y),
            BodyItem::Naf(gs) => crate::k3_safe::stream_all_occurs(gs, y),
        }
        let k = choose|k: int| 0 <= k < head.len() && head[k] == y;
        assert(bs[k] == y);
    } else {
        item_var_in2(items.drop_first(), i - 1, y);
        let k = choose|k: int| 0 <= k < rest.len() && rest[k] == y;
        assert(bs[head.len() + k] == y);
    }
}

pub proof fn item_in_fs(g: spec::Group, j: int, i: int, y: nat)
    requires
        0 <= j < g.clauses.len(),
        0 <= i < g.clauses[j].body.len(),
        item_occurs(y, g.clauses[j].body[i]),
    ensures
        fsj(g, j).contains(y),
{
    let cl = g.clauses[j];
    item_var_in2(cl.body, i, y);
    let vs = var_stream(cl.head) + ckc_spec::v1text::body_var_stream(cl.body);
    let k = choose|k: int|
        0 <= k < ckc_spec::v1text::body_var_stream(cl.body).len()
            && ckc_spec::v1text::body_var_stream(cl.body)[k] == y;
    assert(vs[var_stream(cl.head).len() + k] == y);
    crate::k3_number::firsts_member(vs, Set::empty(), y);
}

pub proof fn head_in_fs(g: spec::Group, j: int, y: nat)
    requires
        0 <= j < g.clauses.len(),
        occurs(y, g.clauses[j].head),
    ensures
        fsj(g, j).contains(y),
{
    let cl = g.clauses[j];
    crate::k3_safe::stream_occurs(cl.head, y);
    let vs = var_stream(cl.head) + ckc_spec::v1text::body_var_stream(cl.body);
    let k = choose|k: int| 0 <= k < var_stream(cl.head).len() && var_stream(cl.head)[k] == y;
    assert(vs[k] == y);
    crate::k3_number::firsts_member(vs, Set::empty(), y);
}

// The binding of ground values brings in no variable.
pub proof fn bind_back(t: Term, p: Seq<(Term, Term)>, v: nat)
    requires
        vals_ground(p),
        occurs(v, spec::bind_all(t, p)),
    ensures
        occurs(v, t),
    decreases t,
{
    match t {
        Term::Var(y) => {
            bind_var(y, p);
            if let Some(w) = spec::lookup(p, t) {
                lookup_ground(p, t);
                crate::m6_default::ground_absent(w, v);
            }
        },
        Term::Comp(n, args) => {
            bind_comp(n, args, p);
            let a2 = spec::bind_all(t, p)->Comp_1;
            let k = occurs_index(v, a2);
            bind_back(args[k], p, v);
            occurs_member(v, args, k);
        },
        _ => {
            bind_ground(t, p);
        },
    }
}

// An exposed output is a body variable of the clause.
pub proof fn xo_in_fs(g: spec::Group, sel: ws::Sel, j: int, k: int)
    requires
        ws::chain_shape(sel),
        ws::chain_body(sel, g.clauses[j].body),
        0 <= j < g.clauses.len(),
        0 <= k < xo(g, sel, j).len(),
    ensures
        xo(g, sel, j)[k] is Var,
        fsj(g, j).contains(xo(g, sel, j)[k]->Var_0),
{
    xo_props(g, sel, j);
    crate::m6_default::outputs_index(sel);
    let os = ws::outputs(sel);
    let o = xo(g, sel, j)[k];
    let idx = choose|idx: int| 0 <= idx < os.len() && os[idx] == o;
    let m = idx / 2;
    if idx % 2 == 0 {
        assert(idx == 2 * m);
        out_in(g, sel, j, m, 4);
    } else {
        assert(idx == 2 * m + 1);
        out_in(g, sel, j, m, 5);
    }
}

// th2 at a clause variable's walk copy: the variable's witness value when it
// is an exposed output, else unbound.
pub proof fn th2_at(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>, v: nat)
    requires
        ws::chain_shape(sel),
        ws::fixed_skeleton(g, sel),
        ws::chain_body(sel, g.clauses[j].body),
        0 <= j < g.clauses.len(),
        tw.len() == sel.ops.len(),
        ws::tuple_pairs(g, sel, tw) is Some,
        fsj(g, j).contains(v),
    ensures
        th_val(th2(g, sel, j, tw), hoff(g, sel, j) + ckc_spec::trace::pos_of(fsj(g, j), v))
            == if xo(g, sel, j).contains(Term::Var(v)) {
            spec::lookup(tpairs(g, sel, tw), Term::Var(v))
        } else {
            None
        },
{
    let ho = hoff(g, sel, j);
    let fs = fsj(g, j);
    let z = ho + ckc_spec::trace::pos_of(fs, v);
    let st = slot_th(sel, tw);
    let tx = th2x(g, sel, j, tw);
    let os = xo(g, sel, j);
    let cs = ws::counts(sel);
    assert(cs.no_duplicates());
    th_cat(st, tx, z);
    slot_val(sel, tw, z);
    if cs.contains(Term::Var(z)) {
        let i = choose|i: int| 0 <= i < cs.len() && cs[i] == Term::Var(z);
        assert(cs[i] == sel.ops[i].n);
        count_below(g, sel, i);
    }
    assert(th_val(st, z) is None);
    pos_of_at(fs, v);
    xo_props(g, sel, j);
    assert forall|i: int| 0 <= i < os.len() implies #[trigger] tx[i] == (
        ho + ckc_spec::trace::pos_of(fs, os[i]->Var_0),
        spec::lookup(tpairs(g, sel, tw), os[i])->0,
    ) by {}
    // the key at index i hits z exactly when xo[i] is v's variable
    assert forall|i: int| 0 <= i < os.len() implies (#[trigger] tx[i]).0 == z <==> os[i]
        == Term::Var(v) by {
        xo_in_fs(g, sel, j, i);
        pos_of_at(fs, os[i]->Var_0);
    }
    if os.contains(Term::Var(v)) {
        let i = first_at(os, Term::Var(v));
        assert forall|q: int| 0 <= q < i implies (#[trigger] tx[q]).0 != tx[i].0 by {}
        th_at(tx, i);
        tpairs_lookup(g, sel, tw, Term::Var(v));
    } else {
        assert forall|q: int| 0 <= q < tx.len() implies (#[trigger] tx[q]).0 != z by {}
        th_none(tx, z);
    }
}

// A head variable is no count: the open head holds no slot.
pub proof fn head_var_no_count(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    v: nat,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        occurs(v, g.clauses[j].head),
    ensures
        !ws::counts(sel).contains(Term::Var(v)),
{
    let h = ws::open_head(g, sel, j);
    let hp = hpart(g, sel);
    let fh = |q: (Term, Term)| !ws::counts(sel).contains(q.0) && !ws::outputs(sel).contains(q.0);
    assert(hp == g.pairs.filter(fh));
    assert(h == spec::bind_all(g.clauses[j].head, hp));
    assert(ws::clause_ok(p, g, sel, j));
    let e = wenv(p, g, sel, j);
    assert(ws::slot_free(e, h));
    if ws::counts(sel).contains(Term::Var(v)) {
        let i = choose|i: int| 0 <= i < ws::counts(sel).len() && ws::counts(sel)[i] == Term::Var(v);
        assert(vals_ground(g.pairs));
        ground_filter(g.pairs, fh);
        lookup_drop(g.pairs, fh, Term::Var(v));
        bind_keeps(g.clauses[j].head, hp, v);
        assert(e.slots[i] == Term::Var(v));
        assert(ws::mentions(h, e.slots[i]));
    }
}

// The images of clause variable v after the head unification: the concrete
// copy renamed = the walk's copy instantiated by th2.
pub proof fn var_rel(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    tw: Seq<Term>,
    b: nat,
    v: nat,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        tw.len() == sel.ops.len(),
        forall|i: int| 0 <= i < tw.len() ==> ckc_spec::arith::number(#[trigger] tw[i]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
        g.clauses[j].head is Comp,
        b <= hoff(g, sel, j),
        fsj(g, j).contains(v),
    ensures
        rename(
            sub(
                rmap(fsj(g, j), b)(v),
                pend(
                    lmap(tpairs(g, sel, tw)),
                    rmap(fsj(g, j), b),
                    rr(fsj(g, j), b),
                    hin(g.clauses[j].head->Comp_1),
                ),
            ),
            b,
            (hoff(g, sel, j) - b) as nat,
        ) == inst(
            sub(
                rmap(fsj(g, j), hoff(g, sel, j))(v),
                pend(
                    lmap(hpart(g, sel)),
                    rmap(fsj(g, j), hoff(g, sel, j)),
                    rr(fsj(g, j), hoff(g, sel, j)),
                    hin(g.clauses[j].head->Comp_1),
                ),
            ),
            th2(g, sel, j, tw),
        ),
{
    let head = g.clauses[j].head;
    let ts = head->Comp_1;
    let fs = fsj(g, j);
    let ho = hoff(g, sel, j);
    let d = (ho - b) as nat;
    let tp = tpairs(g, sel, tw);
    let hp = hpart(g, sel);
    let h = ws::open_head(g, sel, j);
    let mat = lmap(tp);
    let mbt = rmap(fs, b);
    let rt = rr(fs, b);
    let maa = lmap(hp);
    let mba = rmap(fs, ho);
    let ra = rr(fs, ho);
    let in0 = hin(ts);
    let dom = |w: nat| fs.contains(w);
    assert forall|w: nat| #[trigger] in0(w) implies dom(w) by {
        assert(occurs(w, head));
        head_in_fs(g, j, w);
    }
    assert forall|w1: nat, w2: nat|
        dom(w1) && dom(w2) && #[trigger] rt(w1) == #[trigger] rt(w2) implies w1 == w2 by {
        pos_of_at(fs, w1);
        pos_of_at(fs, w2);
    }
    assert forall|w1: nat, w2: nat|
        dom(w1) && dom(w2) && #[trigger] ra(w1) == #[trigger] ra(w2) implies w1 == w2 by {
        pos_of_at(fs, w1);
        pos_of_at(fs, w2);
    }
    pend_at(mat, mbt, rt, in0, v, dom);
    pend_at(maa, mba, ra, in0, v, dom);
    assert(mbt(v) == Term::Var(rt(v)) && mba(v) == Term::Var(ra(v)));
    let fh = |q: (Term, Term)| !ws::counts(sel).contains(q.0) && !ws::outputs(sel).contains(q.0);
    assert(hp == g.pairs.filter(fh));
    assert(h == spec::bind_all(head, hp));
    tvals_facts(g, sel, tw);
    tpairs_ground(g, sel, tw);
    assert(vals_ground(g.pairs));
    ground_filter(g.pairs, fh);
    xo_props(g, sel, j);
    keys_index(sel);
    tpairs_lookup(g, sel, tw, Term::Var(v));
    th2_at(g, sel, j, tw, v);
    let nops = sel.ops.len() as int;
    if pending(maa, mba, ra, in0, v) {
        // a head variable the open pairs bind: the same value in both runs
        if ws::counts(sel).contains(Term::Var(v)) || ws::outputs(sel).contains(Term::Var(v)) {
            lookup_drop(g.pairs, fh, Term::Var(v));
            assert(false);
        }
        lookup_keep(g.pairs, fh, Term::Var(v));
        let w = spec::lookup(hp, Term::Var(v))->0;
        if spec::lookup(tvals(sel, tw), Term::Var(v)) is Some {
            assert(keys(sel).contains(Term::Var(v)));
            let q = choose|q: int| 0 <= q < keys(sel).len() && keys(sel)[q] == Term::Var(v);
            if q < nops {
                assert(sel.ops[q].s == Term::Var(v));
            } else if q < 2 * nops {
                assert(ws::counts(sel)[q - nops] == Term::Var(v));
            } else {
                assert(ws::outputs(sel)[q - 2 * nops] == Term::Var(v));
            }
        }
        assert(mat(v) == w);
        assert(pending(mat, mbt, rt, in0, v));
        crate::k3_sound::naf::rename_ground_id(w, b, d);
        inst_ground(w, th2(g, sel, j, tw));
    } else if pending(mat, mbt, rt, in0, v) {
        // an exposed output: th2 carries its value to the walk's copy
        let x = spec::lookup(tp, Term::Var(v))->0;
        assert(mat(v) == x);
        assert(occurs(v, head));
        head_var_no_count(p, g, sel, j, v);
        if !ws::outputs(sel).contains(Term::Var(v)) {
            lookup_keep(g.pairs, fh, Term::Var(v));
            assert(maa(v) == spec::lookup(g.pairs, Term::Var(v))->0);
            lookup_ground(g.pairs, Term::Var(v));
            assert(false);
        }
        lookup_drop(g.pairs, fh, Term::Var(v));
        bind_keeps(head, hp, v);
        assert(ws::mentions(h, Term::Var(v)));
        assert(xo(g, sel, j).contains(Term::Var(v)));
        crate::k3_sound::naf::rename_ground_id(x, b, d);
    } else {
        // neither: the copy itself, unbound by th2
        if xo(g, sel, j).contains(Term::Var(v)) {
            let k = choose|k: int| 0 <= k < xo(g, sel, j).len() && xo(g, sel, j)[k] == Term::Var(v);
            assert(ws::mentions(h, xo(g, sel, j)[k]));
            bind_back(head, hp, v);
            assert(occurs_all(v, ts));
            assert(in0(v));
            assert(spec::lookup(g.pairs, xo(g, sel, j)[k]) is Some);
            lookup_ground(tp, Term::Var(v));
            assert(ground(mat(v)));
            assert(false);
        }
        assert(lift_var(rt(v), b, d) == ra(v));
    }
}

// --- the two unifications of the entry ---
pub open spec fn cpend(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>, b: nat) -> spec_fn(
    nat,
) -> Term {
    pend(
        lmap(tpairs(g, sel, tw)),
        rmap(fsj(g, j), b),
        rr(fsj(g, j), b),
        hin(g.clauses[j].head->Comp_1),
    )
}

pub open spec fn apend(g: spec::Group, sel: ws::Sel, j: int) -> spec_fn(nat) -> Term {
    pend(
        lmap(hpart(g, sel)),
        rmap(fsj(g, j), hoff(g, sel, j)),
        rr(fsj(g, j), hoff(g, sel, j)),
        hin(g.clauses[j].head->Comp_1),
    )
}

// The concrete body after the head call at tw (base b, depth dd).
pub open spec fn cstk(g: spec::Group, sel: ws::Sel, j: int, tw: Seq<Term>, b: nat, dd: nat) -> Seq<
    Goal,
> {
    Seq::new(
        g.clauses[j].body.len(),
        |i: int|
            Goal::Lit(
                sub(sub_item(g.clauses[j].body[i], rmap(fsj(g, j), b)), cpend(g, sel, j, tw, b)),
                dd,
            ),
    )
}

// The walk's body after its head unification.
pub open spec fn wbody(g: spec::Group, sel: ws::Sel, j: int) -> Seq<Term> {
    Seq::new(
        g.clauses[j].body.len(),
        |i: int|
            sub(sub_item(g.clauses[j].body[i], rmap(fsj(g, j), hoff(g, sel, j))), apend(g, sel, j)),
    )
}

// The body items renamed above off, as terms.
pub open spec fn rbody(g: spec::Group, j: int, off: nat) -> Seq<Term> {
    Seq::new(g.clauses[j].body.len(), |i: int| sub_item(g.clauses[j].body[i], rmap(fsj(g, j), off)))
}

pub proof fn rbody_facts(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    off: nat,
    dd: nat,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
    ensures
        enc(
            Seq::<Goal>::empty() + body_goals(spec::canon_clause(g.clauses[j]).body, off, dd)
                + Seq::<Goal>::empty(),
        ) == rbody(g, j, off),
        spec::canon_clause(g.clauses[j]).body.map_values(|it: BodyItem| item_term(it, off))
            == rbody(g, j, off),
        forall|x: nat| #[trigger] occurs_all(x, rbody(g, j, off)) ==> x >= off,
{
    let cl = g.clauses[j];
    let c = spec::canon_clause(cl);
    let fs = fsj(g, j);
    let rb = rbody(g, j, off);
    let st0 = Seq::<Goal>::empty() + body_goals(c.body, off, dd) + Seq::<Goal>::empty();
    assert(ws::primitive_body(cl.body));
    assert forall|i: int| 0 <= i < cl.body.len() implies #[trigger] item_term(c.body[i], off)
        == rb[i] by {
        assert(c.body[i] == spec::renumber_item(cl.body[i], fs));
        crate::m6_head::item_sub(cl.body[i], fs, off);
    }
    assert(enc(st0) =~= rb) by {
        assert forall|i: int| 0 <= i < rb.len() implies enc(st0)[i] == rb[i] by {
            assert(st0[i] == Goal::Lit(item_term(c.body[i], off), dd));
        }
    }
    assert(c.body.map_values(|it: BodyItem| item_term(it, off)) =~= rb);
    assert forall|x: nat| #[trigger] occurs_all(x, rb) implies x >= off by {
        let k = occurs_index(x, rb);
        assert(cl.body[k] is Naf ==> cl.body[k]->Naf_0.len() > 0);
        rmap_item_above(cl.body[k], fs, off, x);
    }
}

// The head call's unification with its own clause at tw: HM's outcome.
pub proof fn entry_unify(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    tw: Seq<Term>,
    b: nat,
    dd: nat,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        tw.len() == sel.ops.len(),
        forall|i: int| 0 <= i < tw.len() ==> ckc_spec::arith::number(#[trigger] tw[i]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
        g.clauses[j].head is Comp,
        b == nvars(tob(g, sel, tw).heads[j]),
    ensures
        unify(
            UState {
                pairs: zip(
                    args_of(tob(g, sel, tw).heads[j]),
                    args_of(shift(spec::canon_clause(g.clauses[j]).head, b)),
                ),
                stack: Seq::<Goal>::empty() + body_goals(
                    spec::canon_clause(g.clauses[j]).body,
                    b,
                    dd,
                ) + Seq::<Goal>::empty(),
                sol: Seq::empty(),
            },
        ) == UOut::Ok(cstk(g, sel, j, tw, b, dd), Seq::<Term>::empty()),
{
    let cl = g.clauses[j];
    let head = cl.head;
    let ts = head->Comp_1;
    let hn = head->Comp_0;
    let c = spec::canon_clause(cl);
    let fs = fsj(g, j);
    let tp = tpairs(g, sel, tw);
    let ht = tob(g, sel, tw).heads[j];
    assert(ht == spec::bind_all(head, tp));
    tvals_facts(g, sel, tw);
    tpairs_ground(g, sel, tw);
    crate::m6_head::bind_sub(head, tp);
    crate::m6_head::renum_sub(head, fs, b);
    assert(head == Term::Comp(hn, ts));
    assert(args_of(ht) == crate::m6_head::sub_all(ts, lmap(tp)));
    assert(args_of(shift(c.head, b)) == crate::m6_head::sub_all(ts, rmap(fs, b)));
    rbody_facts(p, g, sel, j, b, dd);
    let ys = rbody(g, j, b);
    assert forall|v: nat| #[trigger] occurs_all(v, ts) implies fs.contains(v) by {
        assert(occurs(v, head));
        head_in_fs(g, j, v);
    }
    assert forall|v: nat|
        occurs_all(v, ts) && #[trigger] spec::lookup(tp, Term::Var(v)) is None implies v < b by {
        assert(occurs(v, head));
        bind_keeps(head, tp, v);
        occurs_lt(ht, v);
    }
    hm_head(ts, tp, fs, b, ys);
    let st0 = Seq::<Goal>::empty() + body_goals(c.body, b, dd) + Seq::<Goal>::empty();
    let u = UState {
        pairs: zip(args_of(ht), args_of(shift(c.head, b))),
        stack: st0,
        sol: Seq::empty(),
    };
    let ue = UState { pairs: u.pairs, stack: Seq::<Goal>::empty(), sol: u.sol + enc(u.stack) };
    assert(u.sol + enc(u.stack) =~= ys);
    assert(ue == crate::m6_head::hm_state(ts, lmap(tp), rmap(fs, b), ys));
    let x = crate::m6_head::sub_all(ys, cpend(g, sel, j, tw, b));
    let f = unify_fuel(ue);
    unify_n_enc(u, f);
    let s1 = unify_n(u, f)->Ok_0;
    let x1 = unify_n(u, f)->Ok_1;
    assert(x1 + enc(s1) == x);
    crate::m6_head::sub_all_index(ys, cpend(g, sel, j, tw, b));
    assert(s1.len() == st0.len());
    assert(x1 =~= Seq::<Term>::empty());
    assert(s1 =~= cstk(g, sel, j, tw, b, dd)) by {
        assert forall|i: int| 0 <= i < s1.len() implies s1[i] == cstk(g, sel, j, tw, b, dd)[i] by {
            assert(enc(s1)[i] == x[i]);
            assert(st0[i] == Goal::Lit(item_term(c.body[i], b), dd));
            match s1[i] {
                Goal::Lit(_, _) => {},
                Goal::NafCut(_) => {},
            }
        }
    }
    crate::k3_sound::naf::unify_at(u, f);
}

// The walk's body after its head unification: HM's outcome over the open
// head's pairs; the walk accepts it.
pub proof fn walk_body(p: Seq<ckc_spec::v1text::DocClause>, g: spec::Group, sel: ws::Sel, j: int)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        g.clauses[j].head is Comp,
    ensures
        ws::walk_ok(
            wenv(p, g, sel, j),
            wbody(g, sel, j),
            1 + ws::nodes_all(rbody(g, j, hoff(g, sel, j))),
            0,
        ),
{
    let cl = g.clauses[j];
    let head = cl.head;
    let ts = head->Comp_1;
    let hn = head->Comp_0;
    let c = spec::canon_clause(cl);
    let fs = fsj(g, j);
    let ho = hoff(g, sel, j);
    let hp = hpart(g, sel);
    let h = ws::open_head(g, sel, j);
    let fh = |q: (Term, Term)| !ws::counts(sel).contains(q.0) && !ws::outputs(sel).contains(q.0);
    assert(hp == g.pairs.filter(fh));
    assert(h == spec::bind_all(head, hp));
    assert(vals_ground(g.pairs));
    ground_filter(g.pairs, fh);
    crate::m6_head::bind_sub(head, hp);
    crate::m6_head::renum_sub(head, fs, ho);
    assert(head == Term::Comp(hn, ts));
    let a1 = crate::m6_head::sub_all(ts, lmap(hp));
    let a2 = crate::m6_head::sub_all(ts, rmap(fs, ho));
    assert(h == Term::Comp(hn, a1));
    assert(shift(c.head, ho) == Term::Comp(hn, a2));
    crate::m6_head::sub_all_index(ts, lmap(hp));
    crate::m6_head::sub_all_index(ts, rmap(fs, ho));
    rbody_facts(p, g, sel, j, ho, 0);
    let ys = rbody(g, j, ho);
    assert forall|v: nat| #[trigger] occurs_all(v, ts) implies fs.contains(v) by {
        assert(occurs(v, head));
        head_in_fs(g, j, v);
    }
    assert forall|v: nat|
        occurs_all(v, ts) && #[trigger] spec::lookup(hp, Term::Var(v)) is None implies v < ho by {
        assert(occurs(v, head));
        bind_keeps(head, hp, v);
        occurs_lt(h, v);
    }
    hm_head(ts, hp, fs, ho, ys);
    unify_comp(hn, a1, a2, ys);
    assert(crate::m6_head::hm_state(ts, lmap(hp), rmap(fs, ho), ys) == UState {
        pairs: zip(a1, a2),
        stack: Seq::<Goal>::empty(),
        sol: ys,
    });
    assert(ws::clause_ok(p, g, sel, j));
    let e = wenv(p, g, sel, j);
    let body = c.body.map_values(|it: BodyItem| item_term(it, ho));
    assert(body == ys);
    assert(wbody(g, sel, j) =~= crate::m6_head::sub_all(ys, apend(g, sel, j))) by {
        crate::m6_head::sub_all_index(ys, apend(g, sel, j));
    }
}

// The walk's body goals at depth dd.
pub open spec fn bstk(g: spec::Group, sel: ws::Sel, j: int, dd: nat) -> Seq<Goal> {
    Seq::new(g.clauses[j].body.len(), |i: int| Goal::Lit(wbody(g, sel, j)[i], dd))
}

// The concrete body renders the walk's body under th2.
pub proof fn entry_srel(
    p: Seq<ckc_spec::v1text::DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    tw: Seq<Term>,
    b: nat,
    dd: nat,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        tw.len() == sel.ops.len(),
        forall|i: int| 0 <= i < tw.len() ==> ckc_spec::arith::number(#[trigger] tw[i]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
        g.clauses[j].head is Comp,
        b <= hoff(g, sel, j),
    ensures
        crate::m6_lockstep::srel(
            cstk(g, sel, j, tw, b, dd),
            bstk(g, sel, j, dd),
            th2(g, sel, j, tw),
            b,
            (hoff(g, sel, j) - b) as nat,
        ),
{
    let cl = g.clauses[j];
    let fs = fsj(g, j);
    let ho = hoff(g, sel, j);
    let d = (ho - b) as nat;
    let th = th2(g, sel, j, tw);
    let cs = cstk(g, sel, j, tw, b, dd);
    let bs = bstk(g, sel, j, dd);
    assert(ws::primitive_body(cl.body));
    assert forall|i: int| 0 <= i < cs.len() implies #[trigger] crate::m6_lockstep::grel(
        cs[i],
        bs[i],
        th,
        b,
        d,
    ) by {
        let it = cl.body[i];
        assert(it is Naf ==> it->Naf_0.len() > 0);
        assert forall|v: nat| #[trigger] item_occurs(v, it) implies rename(
            sub(rmap(fs, b)(v), cpend(g, sel, j, tw, b)),
            b,
            d,
        ) == inst(sub(rmap(fs, ho)(v), apend(g, sel, j)), th) by {
            item_in_fs(g, j, i, v);
            var_rel(p, g, sel, j, tw, b, v);
        }
        rel_item2(
            it,
            rmap(fs, b),
            cpend(g, sel, j, tw, b),
            rmap(fs, ho),
            apend(g, sel, j),
            th,
            b,
            d,
        );
    }
}

} // verus!
