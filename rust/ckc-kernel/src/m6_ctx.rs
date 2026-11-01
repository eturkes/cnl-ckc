#[cfg(verus_keep_ghost)]
use crate::m6_arith::*;
#[cfg(verus_keep_ghost)]
use crate::m6_cell::*;
#[cfg(verus_keep_ghost)]
use crate::m6_default::*;
#[cfg(verus_keep_ghost)]
use crate::m6_lockstep::*;
#[cfg(verus_keep_ghost)]
use crate::m6_setup::*;
#[cfg(verus_keep_ghost)]
use crate::m6_step::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::arith as a;
#[cfg(verus_keep_ghost)]
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::arg;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::DocClause;
#[cfg(verus_keep_ghost)]
use ckc_spec::witness as ws;
use vstd::prelude::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};
// Contract ws D7 (cell invariance), the setup: a tuple's pairs, chain
// values and witness facts against the walk's objects.

pub open spec fn tpairs(g: spec::Group, sel: ws::Sel, tw: Seq<Term>) -> Seq<(Term, Term)> {
    ws::tuple_pairs(g, sel, tw)->Some_0
}

pub open spec fn tvals(sel: ws::Sel, tw: Seq<Term>) -> Seq<(Term, Term)> {
    spec::chain_pairs(sel.chain, ws::op_pairs(sel, tw))->Some_0
}

pub open spec fn tob(g: spec::Group, sel: ws::Sel, tw: Seq<Term>) -> ckc_spec::replay::Ob {
    spec::obligation(spec::Group { pairs: tpairs(g, sel, tw), ..g }, Seq::empty(), 0)
}

// --- lookups through a filter ---
pub proof fn lookup_keep(pairs: Seq<(Term, Term)>, f: spec_fn((Term, Term)) -> bool, key: Term)
    requires
        forall|p: (Term, Term)| p.0 == key ==> #[trigger] f(p),
    ensures
        spec::lookup(pairs.filter(f), key) == spec::lookup(pairs, key),
    decreases pairs.len(),
{
    reveal_with_fuel(spec::lookup, 1);
    if pairs.len() > 0 {
        let rest = pairs.drop_first();
        assert(pairs =~= seq![pairs[0]] + rest);
        rest.lemma_filter_prepend(pairs[0], f);
        lookup_keep(rest, f, key);
        if f(pairs[0]) {
            assert(pairs.filter(f) == seq![pairs[0]] + rest.filter(f));
            assert((seq![pairs[0]] + rest.filter(f)).drop_first() =~= rest.filter(f));
        } else {
            assert(pairs.filter(f) == rest.filter(f));
            assert(pairs[0].0 != key);
        }
    } else {
        assert(pairs.filter(f) =~= Seq::<(Term, Term)>::empty());
    }
}

pub proof fn lookup_drop(pairs: Seq<(Term, Term)>, f: spec_fn((Term, Term)) -> bool, key: Term)
    requires
        forall|p: (Term, Term)| p.0 == key ==> !#[trigger] f(p),
    ensures
        spec::lookup(pairs.filter(f), key) is None,
    decreases pairs.len(),
{
    reveal_with_fuel(spec::lookup, 1);
    if pairs.len() > 0 {
        let rest = pairs.drop_first();
        assert(pairs =~= seq![pairs[0]] + rest);
        rest.lemma_filter_prepend(pairs[0], f);
        lookup_drop(rest, f, key);
        if f(pairs[0]) {
            assert(pairs.filter(f) == seq![pairs[0]] + rest.filter(f));
            assert((seq![pairs[0]] + rest.filter(f)).drop_first() =~= rest.filter(f));
            assert(pairs[0].0 != key);
        } else {
            assert(pairs.filter(f) == rest.filter(f));
        }
    } else {
        assert(pairs.filter(f) =~= Seq::<(Term, Term)>::empty());
    }
}

// A filter keeps ground values.
pub proof fn ground_filter(pairs: Seq<(Term, Term)>, f: spec_fn((Term, Term)) -> bool)
    requires
        vals_ground(pairs),
    ensures
        vals_ground(pairs.filter(f)),
{
    assert forall|i: int| 0 <= i < pairs.filter(f).len() implies ground(
        (#[trigger] pairs.filter(f)[i]).1,
    ) by {
        pairs.lemma_filter_contains_rev(f, pairs.filter(f)[i]);
        let k = choose|k: int| 0 <= k < pairs.len() && pairs[k] == pairs.filter(f)[i];
        assert(ground(pairs[k].1));
    }
}

// --- the slot instantiation ---
pub proof fn th_at(th: Seq<(nat, Term)>, idx: int)
    requires
        0 <= idx < th.len(),
        forall|j: int| 0 <= j < idx ==> (#[trigger] th[j]).0 != th[idx].0,
    ensures
        th_val(th, th[idx].0) == Some(th[idx].1),
    decreases th.len(),
{
    if idx > 0 {
        assert forall|j: int| 0 <= j < idx - 1 implies (#[trigger] th.drop_first()[j]).0
            != th.drop_first()[idx - 1].0 by {
            assert(th.drop_first()[j] == th[j + 1]);
        }
        th_at(th.drop_first(), idx - 1);
        assert(th[0].0 != th[idx].0);
    }
}

pub proof fn th_none(th: Seq<(nat, Term)>, x: nat)
    requires
        forall|j: int| 0 <= j < th.len() ==> (#[trigger] th[j]).0 != x,
    ensures
        th_val(th, x) is None,
    decreases th.len(),
{
    if th.len() > 0 {
        assert forall|j: int| 0 <= j < th.drop_first().len() implies (
        #[trigger] th.drop_first()[j]).0 != x by {
            assert(th.drop_first()[j] == th[j + 1]);
        }
        th_none(th.drop_first(), x);
        assert(th[0].0 != x);
    }
}

// Operand i's slot carries tw[i]; a variable outside the counts is unbound.
pub proof fn slot_val(sel: ws::Sel, tw: Seq<Term>, x: nat)
    requires
        ws::chain_shape(sel),
        ws::counts(sel).no_duplicates(),
        tw.len() == sel.ops.len(),
    ensures
        ws::counts(sel).contains(Term::Var(x)) ==> exists|i: int|
            0 <= i < sel.ops.len() && sel.ops[i].n == Term::Var(x) && #[trigger] th_val(
                slot_th(sel, tw),
                x,
            ) == Some(tw[i]),
        !ws::counts(sel).contains(Term::Var(x)) ==> th_val(slot_th(sel, tw), x) is None,
{
    let th = slot_th(sel, tw);
    let cs = ws::counts(sel);
    assert(cs.len() == sel.ops.len());
    assert forall|j: int| 0 <= j < sel.ops.len() implies #[trigger] cs[j] == sel.ops[j].n && th[j]
        == (sel.ops[j].n->Var_0, tw[j]) by {}
    if cs.contains(Term::Var(x)) {
        let i = choose|i: int| 0 <= i < cs.len() && cs[i] == Term::Var(x);
        assert forall|j: int| 0 <= j < i implies (#[trigger] th[j]).0 != th[i].0 by {
            assert(cs[j] != cs[i]);
            assert(sel.ops[j].n is Var);
        }
        th_at(th, i);
        assert(th_val(th, x) == Some(tw[i]));
    } else {
        assert forall|j: int| 0 <= j < th.len() implies (#[trigger] th[j]).0 != x by {
            assert(cs[j] != Term::Var(x));
            assert(sel.ops[j].n is Var);
        }
        th_none(th, x);
    }
}

// --- the chain values of a tuple ---
pub proof fn tvals_facts(g: spec::Group, sel: ws::Sel, tw: Seq<Term>)
    requires
        ws::chain_shape(sel),
        tw.len() == sel.ops.len(),
        forall|i: int| 0 <= i < tw.len() ==> a::number(#[trigger] tw[i]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
    ensures
        ({
            let v = tvals(sel, tw);
            &&& rel_pairs(v, v)
            &&& vals_ground(v)
            &&& keys_ok(sel, sel.chain.len() as int, v)
            &&& forall|i: int|
                0 <= i < sel.ops.len() ==> spec::lookup(v, (#[trigger] sel.ops[i]).n) == Some(tw[i])
                    && spec::lookup(v, sel.ops[i].s) == Some(ws::exactly())
            &&& forall|key: Term|
                !keys(sel).contains(key) ==> #[trigger] spec::lookup(v, key) is None
            &&& forall|m: int|
                0 <= m < sel.chain.len() ==> #[trigger] a::arith_result(
                    spec::bind_all(sel.chain[m], v),
                ) == Some(
                    (
                        spec::lookup(v, arg(sel.chain[m], 4))->0,
                        spec::lookup(v, arg(sel.chain[m], 5))->0,
                    ),
                )
        }),
{
    let pt = ws::op_pairs(sel, tw);
    base_keys(sel, tw);
    assert forall|i: int| 0 <= i < sel.ops.len() implies ws::same_cell(
        ws::boundaries(g, sel, i),
        #[trigger] tw[i],
        tw[i],
    ) by {}
    cell_rel(g, sel, tw, tw);
    assert(sel.chain.skip(0) =~= sel.chain);
    chain_vals(sel, 0, pt);
    let v = tvals(sel, tw);
    rel_vals_ground(v, v);
    // the operand keys
    op_pairs_index(sel, tw);
    keys_index(sel);
    let n = sel.ops.len() as int;
    assert forall|key: Term| !keys(sel).contains(key) implies #[trigger] spec::lookup(
        v,
        key,
    ) is None by {
        assert forall|j: int| 0 <= j < pt.len() implies (#[trigger] pt[j]).0 != key by {
            let q = j / 2;
            if j % 2 == 0 {
                assert(j == 2 * q);
                assert(keys(sel)[q] == pt[j].0);
            } else {
                assert(j == 2 * q + 1);
                assert(keys(sel)[n + q] == pt[j].0);
            }
        }
        lookup_absent(pt, key);
    }
}

// The witness pairs of a tuple: each key keeps its first binding, re-valued
// where the chain values bind it.
pub proof fn tpairs_lookup(g: spec::Group, sel: ws::Sel, tw: Seq<Term>, key: Term)
    requires
        ws::tuple_pairs(g, sel, tw) is Some,
    ensures
        spec::lookup(tpairs(g, sel, tw), key) == match spec::lookup(g.pairs, key) {
            Option::None => Option::None,
            Option::Some(w) => match spec::lookup(tvals(sel, tw), key) {
                Option::Some(v) => Option::Some(v),
                Option::None => Option::Some(w),
            },
        },
{
    lookup_revalue(g.pairs, tvals(sel, tw), key);
}

// --- variables against their bounds ---
pub proof fn occurs_lt(t: Term, x: nat)
    requires
        occurs(x, t),
    ensures
        x < nvars(t),
{
    if nvars(t) <= x {
        crate::k2_engine::occurs_bound(t, x);
    }
}

pub proof fn occurs_all_lt(ts: Seq<Term>, x: nat)
    requires
        occurs_all(x, ts),
    ensures
        x < nvars_all(ts),
{
    if nvars_all(ts) <= x {
        crate::k2_engine::occurs_bound_all(ts, x);
    }
}

proof fn occurrences_pos(xs: Seq<nat>, x: nat)
    requires
        crate::m6_vars::occurrences(xs, x) > 0,
    ensures
        xs.contains(x),
    decreases xs.len(),
{
    if xs.len() == 0 {
        reveal(Seq::filter);
        assert(crate::m6_vars::occurrences(xs, x) == 0);
    } else {
        let init = xs.drop_last();
        assert(xs == init.push(xs.last()));
        crate::m6_vars::occurrence_step(init, xs.last(), x);
        if xs.last() != x {
            occurrences_pos(init, x);
            let i = choose|i: int| 0 <= i < init.len() && init[i] == x;
            assert(xs[i] == x);
        } else {
            assert(xs[xs.len() - 1] == x);
        }
    }
}

// A count variable lies below the skeleton's variable bound.
pub proof fn count_below(g: spec::Group, sel: ws::Sel, i: int)
    requires
        ws::fixed_skeleton(g, sel),
        0 <= i < sel.ops.len(),
    ensures
        sel.ops[i].n is Var,
        sel.ops[i].n->Var_0 < nvars_all(ws::skeleton(g, sel)),
{
    let x = sel.ops[i].n->Var_0;
    let sk = ws::skeleton(g, sel);
    assert(spec::occ_all(x, sk) == 1);
    crate::m6_safety::occ_all_bridge(sk, x);
    occurrences_pos(var_stream_all(sk), x);
    crate::k3_safe::stream_all_occurs(sk, x);
    occurs_all_lt(sk, x);
}

// A positive witness goal reads no chain output.
pub proof fn no_out(sel: ws::Sel, body: Seq<ckc_spec::v1text::BodyItem>, i: int, m: int)
    requires
        ws::chain_body(sel, body),
        0 <= i < spec::positive_goals(body).len(),
        0 <= m < ws::outputs(sel).len(),
    ensures
        !ws::mentions(spec::positive_goals(body)[i], ws::outputs(sel)[m]),
{
    let pos = spec::pos_terms(body.filter(|it: ckc_spec::v1text::BodyItem| it is Pos));
    let fb = |t: Term| !spec::is_builtin_lit(t);
    let fa = |t: Term| !ws::is_arith_lit(t);
    let pgs = spec::positive_goals(body);
    assert(pgs == pos.filter(fb));
    let pg = pgs[i];
    pos.lemma_filter_pred(fb, i);
    assert(pos.filter(fb).contains(pg));
    pos.lemma_filter_contains_rev(fb, pg);
    let idx = choose|idx: int| 0 <= idx < pos.len() && pos[idx] == pg;
    assert(fa(pg)) by {
        reveal(a::is_arith);
    }
    pos.lemma_filter_contains(fa, idx);
    let k = choose|k: int| 0 <= k < pos.filter(fa).len() && pos.filter(fa)[k] == pg;
    let others = pos.filter(fa) + ws::naf_goals(body);
    assert(others[k] == pg);
}

// The witness pairs keep ground values.
pub proof fn tpairs_ground(g: spec::Group, sel: ws::Sel, tw: Seq<Term>)
    requires
        forall|k: int| 0 <= k < g.pairs.len() ==> ground((#[trigger] g.pairs[k]).1),
        ws::tuple_pairs(g, sel, tw) is Some,
        vals_ground(tvals(sel, tw)),
    ensures
        vals_ground(tpairs(g, sel, tw)),
{
    let vs = tvals(sel, tw);
    let tp = tpairs(g, sel, tw);
    assert forall|k: int| 0 <= k < tp.len() implies ground((#[trigger] tp[k]).1) by {
        lookup_ground(vs, g.pairs[k].0);
        assert(ground(g.pairs[k].1));
    }
}

// The witness facts of a tuple: the skeleton with its slots instantiated.
pub proof fn wfacts(g: spec::Group, sel: ws::Sel, tw: Seq<Term>, i: int)
    requires
        g.clauses.len() > 0,
        forall|k: int| 0 <= k < g.pairs.len() ==> ground((#[trigger] g.pairs[k]).1),
        ws::chain_shape(sel),
        ws::exact_comparators(g, sel),
        ws::fixed_skeleton(g, sel),
        ws::chain_body(sel, g.clauses[0].body),
        tw.len() == sel.ops.len(),
        forall|k: int| 0 <= k < tw.len() ==> a::number(#[trigger] tw[k]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
        0 <= i < ws::skeleton(g, sel).len(),
    ensures
        tob(g, sel, tw).facts[i] == inst(ws::skeleton(g, sel)[i], slot_th(sel, tw)),
{
    let pgs = spec::positive_goals(g.clauses[0].body);
    let pg = pgs[i];
    let tp = tpairs(g, sel, tw);
    let fo = |p: (Term, Term)| !ws::counts(sel).contains(p.0);
    let op = g.pairs.filter(fo);
    let st = slot_th(sel, tw);
    assert(op == ws::open_pairs(g, sel));
    assert(tob(g, sel, tw).facts[i] == spec::bind_all(pg, tp));
    assert(ws::skeleton(g, sel)[i] == spec::bind_all(pg, op));
    tvals_facts(g, sel, tw);
    let vs = tvals(sel, tw);
    tpairs_ground(g, sel, tw);
    assert(vals_ground(g.pairs));
    ground_filter(g.pairs, fo);
    keys_index(sel);
    outputs_index(sel);
    let nops = sel.ops.len() as int;
    assert forall|x: nat| #[trigger] occurs(x, pg) implies (th_val(st, x) is Some ==> spec::lookup(
        tp,
        Term::Var(x),
    ) == th_val(st, x) && spec::lookup(op, Term::Var(x)) is None) && (th_val(st, x) is None
        ==> spec::lookup(tp, Term::Var(x)) == spec::lookup(op, Term::Var(x))) by {
        slot_val(sel, tw, x);
        tpairs_lookup(g, sel, tw, Term::Var(x));
        if ws::counts(sel).contains(Term::Var(x)) {
            let i2 = choose|i2: int|
                0 <= i2 < sel.ops.len() && sel.ops[i2].n == Term::Var(x) && #[trigger] th_val(st, x)
                    == Some(tw[i2]);
            assert(spec::lookup(g.pairs, sel.ops[i2].n) is Some);
            assert(spec::lookup(vs, sel.ops[i2].n) == Some(tw[i2]));
            lookup_drop(g.pairs, fo, Term::Var(x));
        } else {
            lookup_keep(g.pairs, fo, Term::Var(x));
            if spec::lookup(vs, Term::Var(x)) is Some && spec::lookup(
                g.pairs,
                Term::Var(x),
            ) is Some {
                assert(keys(sel).contains(Term::Var(x)));
                let q = choose|q: int| 0 <= q < keys(sel).len() && keys(sel)[q] == Term::Var(x);
                if q < nops {
                    assert(sel.ops[q].s == Term::Var(x));
                } else if q < 2 * nops {
                    assert(ws::counts(sel)[q - nops] == Term::Var(x));
                } else {
                    let m = q - 2 * nops;
                    assert(ws::outputs(sel)[m] == Term::Var(x));
                    no_out(sel, g.clauses[0].body, i, m);
                }
            }
        }
    }
    bind_open(pg, tp, op, st);
}

// --- the clause of head j: variable positions ---
pub open spec fn fsj(g: spec::Group, j: int) -> Seq<nat> {
    firsts(
        var_stream(g.clauses[j].head) + ckc_spec::v1text::body_var_stream(g.clauses[j].body),
        Set::empty(),
    )
}

pub open spec fn hoff(g: spec::Group, sel: ws::Sel, j: int) -> nat {
    max_nat(nvars_all(ws::skeleton(g, sel)), nvars(ws::open_head(g, sel, j)))
}

// A variable of a clause item lies in the clause's first-occurrence list.
pub proof fn item_var_in(items: Seq<ckc_spec::v1text::BodyItem>, i: int, y: nat)
    requires
        0 <= i < items.len(),
        items[i] is Pos,
        occurs(y, items[i]->Pos_0),
    ensures
        ckc_spec::v1text::body_var_stream(items).contains(y),
    decreases items.len(),
{
    let bs = ckc_spec::v1text::body_var_stream(items);
    let head = ckc_spec::v1text::item_var_stream(items[0]);
    let rest = ckc_spec::v1text::body_var_stream(items.drop_first());
    assert(bs == head + rest);
    if i == 0 {
        crate::k3_safe::stream_occurs(items[0]->Pos_0, y);
        let k = choose|k: int| 0 <= k < head.len() && head[k] == y;
        assert(bs[k] == y);
    } else {
        item_var_in(items.drop_first(), i - 1, y);
        let k = choose|k: int| 0 <= k < rest.len() && rest[k] == y;
        assert(bs[head.len() + k] == y);
    }
}

pub proof fn in_fsj(g: spec::Group, j: int, i: int, y: nat)
    requires
        0 <= j < g.clauses.len(),
        0 <= i < g.clauses[j].body.len(),
        g.clauses[j].body[i] is Pos,
        occurs(y, g.clauses[j].body[i]->Pos_0),
    ensures
        fsj(g, j).contains(y),
{
    let cl = g.clauses[j];
    item_var_in(cl.body, i, y);
    let vs = var_stream(cl.head) + ckc_spec::v1text::body_var_stream(cl.body);
    let k = choose|k: int|
        0 <= k < ckc_spec::v1text::body_var_stream(cl.body).len()
            && ckc_spec::v1text::body_var_stream(cl.body)[k] == y;
    assert(vs[var_stream(cl.head).len() + k] == y);
    crate::k3_number::firsts_member(vs, Set::empty(), y);
}

// pos_of finds a member's index.
pub proof fn pos_of_at(fs: Seq<nat>, y: nat)
    requires
        fs.contains(y),
    ensures
        ckc_spec::trace::pos_of(fs, y) < fs.len(),
        fs[ckc_spec::trace::pos_of(fs, y) as int] == y,
    decreases fs.len(),
{
    if fs[0] != y {
        let k = choose|k: int| 0 <= k < fs.len() && fs[k] == y;
        assert(fs.drop_first()[k - 1] == y);
        pos_of_at(fs.drop_first(), y);
    }
}

pub proof fn renumber_occurs(t: Term, fs: Seq<nat>, y: nat)
    requires
        occurs(y, t),
    ensures
        occurs(ckc_spec::trace::pos_of(fs, y), spec::renumber(t, fs)),
    decreases t,
{
    match t {
        Term::Comp(n, args) => {
            let k = crate::m6_head::occurs_index(y, args);
            renumber_occurs(args[k], fs, y);
            renumber_all_index(args, fs);
            crate::m6_head::occurs_member(
                ckc_spec::trace::pos_of(fs, y),
                spec::renumber_all(args, fs),
                k,
            );
        },
        _ => {},
    }
}

pub proof fn renumber_all_index(ts: Seq<Term>, fs: Seq<nat>)
    ensures
        spec::renumber_all(ts, fs).len() == ts.len(),
        forall|i: int|
            0 <= i < ts.len() ==> #[trigger] spec::renumber_all(ts, fs)[i] == spec::renumber(
                ts[i],
                fs,
            ),
    decreases ts.len(),
{
    if ts.len() > 0 {
        renumber_all_index(ts.drop_first(), fs);
        assert forall|i: int| 0 <= i < ts.len() implies #[trigger] spec::renumber_all(ts, fs)[i]
            == spec::renumber(ts[i], fs) by {
            if i > 0 {
                assert(ts.drop_first()[i - 1] == ts[i]);
            }
        }
    }
}

// A body variable's position lies below the canonical clause's variable bound.
pub proof fn pos_below(g: spec::Group, j: int, i: int, y: nat)
    requires
        0 <= j < g.clauses.len(),
        0 <= i < g.clauses[j].body.len(),
        g.clauses[j].body[i] is Pos,
        occurs(y, g.clauses[j].body[i]->Pos_0),
    ensures
        ckc_spec::trace::pos_of(fsj(g, j), y) < clause_nvars(spec::canon_clause(g.clauses[j])),
{
    let cl = g.clauses[j];
    let fs = fsj(g, j);
    let c = spec::canon_clause(cl);
    let t = cl.body[i]->Pos_0;
    assert(c.body[i] == spec::renumber_item(cl.body[i], fs));
    renumber_occurs(t, fs, y);
    occurs_lt(spec::renumber(t, fs), ckc_spec::trace::pos_of(fs, y));
    crate::k3_sound::body::item_variables_bound(c.body, i);
}

// A chain node is a positive item of every clause of the variant.
pub proof fn chain_item(sel: ws::Sel, body: Seq<ckc_spec::v1text::BodyItem>, m: int) -> (i: int)
    requires
        ws::chain_body(sel, body),
        0 <= m < sel.chain.len(),
    ensures
        0 <= i < body.len(),
        body[i] == ckc_spec::v1text::BodyItem::Pos(sel.chain[m]),
{
    let fp = |it: ckc_spec::v1text::BodyItem| it is Pos;
    let fa = |t: Term| ws::is_arith_lit(t);
    let bp = body.filter(fp);
    let pos = spec::pos_terms(bp);
    assert(pos.filter(fa) == sel.chain);
    assert(pos.filter(fa).contains(sel.chain[m]));
    pos.lemma_filter_contains_rev(fa, sel.chain[m]);
    let k = choose|k: int| 0 <= k < pos.len() && pos[k] == sel.chain[m];
    assert(bp.len() == pos.len());
    body.lemma_filter_pred(fp, k);
    let it = bp[k];
    assert(it is Pos);
    assert(it == ckc_spec::v1text::BodyItem::Pos(sel.chain[m]));
    assert(bp.contains(it));
    body.lemma_filter_contains_rev(fp, it);
    let i = choose|i: int| 0 <= i < body.len() && body[i] == it;
    i
}

// --- the lockstep context of head j ---
pub open spec fn wenv(p: Seq<DocClause>, g: spec::Group, sel: ws::Sel, j: int) -> ws::WalkEnv {
    let c = spec::canon_clause(g.clauses[j]);
    let h = ws::open_head(g, sel, j);
    let sk = ws::skeleton(g, sel);
    let off = max_nat(nvars_all(sk), nvars(h));
    ws::WalkEnv {
        p,
        facts: sk + p.filter(|d: DocClause| ws::ground_fact(d)).map_values(|d: DocClause| d.head),
        slots: ws::counts(sel),
        bounds: Seq::new(sel.ops.len(), |i: int| ws::boundaries(g, sel, i)),
        chain: sel.chain,
        off: off + clause_nvars(c),
    }
}

// Node m's output q (4 or 5) renamed into the walk's naming.
pub open spec fn ph(g: spec::Group, sel: ws::Sel, j: int, q: int) -> Seq<nat> {
    Seq::new(
        sel.chain.len(),
        |m: int| hoff(g, sel, j) + ckc_spec::trace::pos_of(fsj(g, j), arg(sel.chain[m], q)->Var_0),
    )
}

pub open spec fn mk_cx(
    p: Seq<DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
) -> Cx {
    let b = nvars(tob(g, sel, t).heads[j]);
    Cx {
        e: wenv(p, g, sel, j),
        nf: ws::skeleton(g, sel).len(),
        dbt: witness_db(tob(g, sel, t).facts, p),
        dbu: witness_db(tob(g, sel, u).facts, p),
        st: slot_th(sel, t),
        su: slot_th(sel, u),
        b,
        d: (hoff(g, sel, j) - b) as nat,
        fresh: b + clause_nvars(spec::canon_clause(g.clauses[j])),
        o4: ph(g, sel, j, 4),
        o5: ph(g, sel, j, 5),
        vt: tvals(sel, t),
        vu: tvals(sel, u),
        hd: true,
        ht: tob(g, sel, t).heads[j],
        hu: tob(g, sel, u).heads[j],
        hdd: replay_depth(),
    }
}

// A positive witness goal is a compound.
pub proof fn pg_comp(body: Seq<ckc_spec::v1text::BodyItem>, i: int)
    requires
        ws::primitive_body(body),
        0 <= i < spec::positive_goals(body).len(),
    ensures
        spec::positive_goals(body)[i] is Comp,
{
    let fp = |it: ckc_spec::v1text::BodyItem| it is Pos;
    let fb = |t: Term| !spec::is_builtin_lit(t);
    let bp = body.filter(fp);
    let pos = spec::pos_terms(bp);
    let pg = spec::positive_goals(body)[i];
    assert(spec::positive_goals(body) == pos.filter(fb));
    assert(pos.filter(fb).contains(pg));
    pos.lemma_filter_contains_rev(fb, pg);
    let k = choose|k: int| 0 <= k < pos.len() && pos[k] == pg;
    assert(bp.len() == pos.len());
    body.lemma_filter_pred(fp, k);
    let it = bp[k];
    assert(bp.contains(it));
    body.lemma_filter_contains_rev(fp, it);
    let q = choose|q: int| 0 <= q < body.len() && body[q] == it;
    assert(ws::primitive(it->Pos_0));
}

// An instantiation binding every variable of t ground-values it.
pub proof fn inst_closed(t: Term, th: Seq<(nat, Term)>)
    requires
        th_ok(th),
        forall|x: nat| #[trigger] occurs(x, t) ==> th_val(th, x) is Some,
    ensures
        ground(inst(t, th)),
    decreases t,
{
    match t {
        Term::Var(x) => {
            assert(occurs(x, t));
            th_val_number(th, x);
        },
        Term::Comp(n, args) => {
            inst_all_index(args, th);
            assert forall|i: int| 0 <= i < args.len() implies ground(
                inst(#[trigger] args[i], th),
            ) by {
                assert forall|x: nat| #[trigger] occurs(x, args[i]) implies th_val(
                    th,
                    x,
                ) is Some by {
                    crate::m6_head::occurs_member(x, args, i);
                    assert(occurs(x, t));
                }
                inst_closed(args[i], th);
            }
            crate::k3_sound::naf::ground_all_index(inst_all(args, th));
        },
        _ => {},
    }
}

// The skeleton's variables are slots.
pub proof fn skel_slot(g: spec::Group, sel: ws::Sel, k: int, x: nat)
    requires
        ws::fixed_skeleton(g, sel),
        0 <= k < ws::skeleton(g, sel).len(),
        occurs(x, ws::skeleton(g, sel)[k]),
    ensures
        ws::counts(sel).contains(Term::Var(x)),
{
    let sk = ws::skeleton(g, sel);
    crate::m6_head::occurs_member(x, sk, k);
    crate::k3_safe::stream_all_occurs(sk, x);
    var_stream_all(sk).to_set_ensures();
    assert(spec::vars_all(sk).contains(x));
}

// The context of head j satisfies what the lockstep reads.
pub proof fn cx_good(
    p: Seq<DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int|
            0 <= i < sel.ops.len() ==> ws::same_cell(
                ws::boundaries(g, sel, i),
                #[trigger] t[i],
                u[i],
            ),
        ws::tuple_pairs(g, sel, t) is Some,
        ws::tuple_pairs(g, sel, u) is Some,
        nvars(tob(g, sel, t).heads[j]) <= hoff(g, sel, j),
    ensures
        cx_ok(mk_cx(p, g, sel, j, t, u)),
{
    let cx = mk_cx(p, g, sel, j, t, u);
    let sk = ws::skeleton(g, sel);
    let cs = ws::counts(sel);
    assert(cs.no_duplicates());
    assert forall|i: int| 0 <= i < t.len() implies a::number(#[trigger] t[i]) is Some by {
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    assert forall|i: int| 0 <= i < u.len() implies a::number(#[trigger] u[i]) is Some by {
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    assert(th_ok(cx.st)) by {
        assert forall|k: int| 0 <= k < cx.st.len() implies ground((#[trigger] cx.st[k]).1) by {
            crate::k3_sound::log::number_ground(t[k]);
        }
    }
    assert(th_ok(cx.su)) by {
        assert forall|k: int| 0 <= k < cx.su.len() implies ground((#[trigger] cx.su[k]).1) by {
            crate::k3_sound::log::number_ground(u[k]);
        }
    }
    assert forall|i: int| 0 <= i < cx.e.slots.len() implies (#[trigger] cx.e.slots[i]) is Var by {
        assert(cs[i] == sel.ops[i].n);
    }
    assert forall|x: nat| #[trigger]
        th_val(cx.st, x) is Some <==> cx.e.slots.contains(Term::Var(x)) by {
        slot_val(sel, t, x);
    }
    assert forall|x: nat| #[trigger]
        th_val(cx.su, x) is Some <==> cx.e.slots.contains(Term::Var(x)) by {
        slot_val(sel, u, x);
    }
    assert forall|i: int| 0 <= i < cx.e.slots.len() implies #[trigger] ws::same_cell(
        cx.e.bounds[i],
        th_val(cx.st, slot_of(cx, i))->0,
        th_val(cx.su, slot_of(cx, i))->0,
    ) by {
        assert(cs[i] == sel.ops[i].n);
        let x = sel.ops[i].n->Var_0;
        assert(cs[i] == Term::Var(x));
        slot_val(sel, t, x);
        slot_val(sel, u, x);
        let i2 = choose|i2: int|
            0 <= i2 < sel.ops.len() && sel.ops[i2].n == Term::Var(x) && #[trigger] th_val(
                slot_th(sel, t),
                x,
            ) == Some(t[i2]);
        let i3 = choose|i3: int|
            0 <= i3 < sel.ops.len() && sel.ops[i3].n == Term::Var(x) && #[trigger] th_val(
                slot_th(sel, u),
                x,
            ) == Some(u[i3]);
        assert(cs[i2] == cs[i] && cs[i3] == cs[i]);
        assert(i2 == i && i3 == i);
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    assert forall|x: nat| #[trigger] is_ph(cx, x) implies th_val(cx.st, x) is None by {
        slot_val(sel, t, x);
        if cs.contains(Term::Var(x)) {
            let i = choose|i: int| 0 <= i < cs.len() && cs[i] == Term::Var(x);
            assert(cs[i] == sel.ops[i].n);
            count_below(g, sel, i);
            if cx.o4.contains(x) {
                let m = choose|m: int| 0 <= m < cx.o4.len() && cx.o4[m] == x;
                assert(cx.o4[m] >= hoff(g, sel, j));
            } else {
                let m = choose|m: int| 0 <= m < cx.o5.len() && cx.o5[m] == x;
                assert(cx.o5[m] >= hoff(g, sel, j));
            }
        }
    }
    assert forall|i: int, i2: int|
        0 <= i < cx.e.bounds.len() && 0 <= i2 < cx.e.bounds[i].len() implies a::number(
        #[trigger] cx.e.bounds[i][i2],
    ) is Some by {
        crate::m6_cover::boundaries_props(g, sel, i);
    }
    // the databases
    let ft = tob(g, sel, t).facts;
    let fu = tob(g, sel, u).facts;
    assert(ft.len() == sk.len() && fu.len() == sk.len());
    assert forall|m: int| 0 <= m < cx.nf implies #[trigger] cx.dbt[m] == fact_clause(
        inst(cx.e.facts[cx.nf - 1 - m], cx.st),
    ) && cx.dbu[m] == fact_clause(inst(cx.e.facts[cx.nf - 1 - m], cx.su)) by {
        let k = cx.nf - 1 - m;
        assert(ft.reverse()[m] == ft[k]);
        assert(fu.reverse()[m] == fu[k]);
        wfacts(g, sel, t, k);
        wfacts(g, sel, u, k);
        assert(cx.e.facts[k] == sk[k]);
    }
    assert forall|k: int| 0 <= k < cx.nf implies (#[trigger] cx.e.facts[k]) is Comp && ground(
        inst(cx.e.facts[k], cx.st),
    ) && ground(inst(cx.e.facts[k], cx.su)) && skel_fact(cx, cx.e.facts[k]) by {
        assert(cx.e.facts[k] == sk[k]);
        let pg = spec::positive_goals(g.clauses[0].body)[k];
        pg_comp(g.clauses[0].body, k);
        bind_comp(pg->Comp_0, pg->Comp_1, ws::open_pairs(g, sel));
        assert forall|x: nat| #[trigger] occurs(x, sk[k]) implies th_val(cx.st, x) is Some
            && th_val(cx.su, x) is Some by {
            skel_slot(g, sel, k, x);
            slot_val(sel, t, x);
            slot_val(sel, u, x);
        }
        inst_closed(sk[k], cx.st);
        inst_closed(sk[k], cx.su);
    }
    let fg = |d: DocClause| ws::ground_fact(d);
    assert forall|q: int| 0 <= q < p.len() && ws::ground_fact(#[trigger] p[q]) implies exists|
        k: int,
    |
        0 <= k < cx.e.facts.len() && cx.e.facts[k] == p[q].head by {
        p.lemma_filter_contains(fg, q);
        let pf = p.filter(fg);
        let k = choose|k: int| 0 <= k < pf.len() && pf[k] == p[q];
        assert(cx.e.facts[sk.len() + k] == p[q].head);
    }
    assert(db_ok(cx));
}

// --- the chain of the context ---
// Input pair q of node kk reads as the chain reads it, in either run.
pub proof fn cin_good(
    p: Seq<DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    tw: Seq<Term>,
    kk: int,
    q: int,
)
    requires
        ws::chain_shape(sel),
        ws::counts(sel).no_duplicates(),
        tw.len() == sel.ops.len(),
        forall|i: int| 0 <= i < tw.len() ==> a::number(#[trigger] tw[i]) is Some,
        ws::tuple_pairs(g, sel, tw) is Some,
        0 <= kk < sel.chain.len(),
        q == 0 || q == 2,
    ensures
        cin(wenv(p, g, sel, j), tvals(sel, tw), slot_th(sel, tw), kk, q),
{
    let e = wenv(p, g, sel, j);
    let c = sel.chain[kk];
    let s = arg(c, q);
    let n = arg(c, q + 1);
    assert(ws::chain_input(sel, kk, s, n));
    tvals_facts(g, sel, tw);
    keys_index(sel);
    outputs_index(sel);
    let nops = sel.ops.len() as int;
    let cs = ws::counts(sel);
    assert(keys(sel).no_duplicates());
    if exists|i: int|
        0 <= i < sel.ops.len() && s == (#[trigger] sel.ops[i]).s && n == sel.ops[i].n {
        let i = choose|i: int|
            0 <= i < sel.ops.len() && s == (#[trigger] sel.ops[i]).s && n == sel.ops[i].n;
        assert(cs[i] == n);
        assert(e.slots.contains(n));
        let x = n->Var_0;
        slot_val(sel, tw, x);
        let i2 = choose|i2: int|
            0 <= i2 < sel.ops.len() && sel.ops[i2].n == Term::Var(x) && #[trigger] th_val(
                slot_th(sel, tw),
                x,
            ) == Some(tw[i2]);
        assert(cs[i2] == cs[i]);
    } else if s == ws::exactly() && a::number(n) is Some {
        crate::k3_sound::log::number_ground(n);
        assert(ground(ws::exactly()));
        assert(!e.slots.contains(n)) by {
            if e.slots.contains(n) {
                let k = choose|k: int| 0 <= k < cs.len() && cs[k] == n;
                assert(cs[k] == sel.ops[k].n);
            }
        }
    } else {
        let m = choose|m: int|
            0 <= m < kk && s == arg(#[trigger] sel.chain[m], 4) && n == arg(sel.chain[m], 5);
        assert(ws::outputs(sel)[2 * m] == s && ws::outputs(sel)[2 * m + 1] == n);
        assert(s is Var && n is Var);
        assert(!e.slots.contains(n)) by {
            if e.slots.contains(n) {
                let k = choose|k: int| 0 <= k < cs.len() && cs[k] == n;
                assert(keys(sel)[nops + k] == n);
                assert(keys(sel)[2 * nops + 2 * m + 1] == n);
            }
        }
    }
}

// Node m's output q of the context: a body variable of clause j, renamed.
pub proof fn out_in(g: spec::Group, sel: ws::Sel, j: int, m: int, q: int)
    requires
        ws::chain_shape(sel),
        ws::chain_body(sel, g.clauses[j].body),
        0 <= j < g.clauses.len(),
        0 <= m < sel.chain.len(),
        q == 4 || q == 5,
    ensures
        arg(sel.chain[m], q) is Var,
        fsj(g, j).contains(arg(sel.chain[m], q)->Var_0),
        ckc_spec::trace::pos_of(fsj(g, j), arg(sel.chain[m], q)->Var_0) < clause_nvars(
            spec::canon_clause(g.clauses[j]),
        ),
{
    outputs_index(sel);
    let c = sel.chain[m];
    assert(ws::is_arith_lit(c));
    assert(c->Comp_1.len() == 6) by {
        reveal(a::is_arith);
    }
    assert(ws::outputs(sel)[2 * m] == arg(c, 4) && ws::outputs(sel)[2 * m + 1] == arg(c, 5));
    assert(arg(c, q) is Var);
    let y = arg(c, q)->Var_0;
    assert(c->Comp_1[q] == Term::Var(y));
    crate::m6_head::occurs_member(y, c->Comp_1, q);
    assert(occurs(y, c));
    let i = chain_item(sel, g.clauses[j].body, m);
    in_fsj(g, j, i, y);
    pos_below(g, j, i, y);
}

// Both runs' chain nodes of the context.
#[verifier::spinoff_prover]
proof fn chain_nodes(
    p: Seq<DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
)
    requires
        ws::chain_shape(sel),
        ws::counts(sel).no_duplicates(),
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> a::number(#[trigger] t[i]) is Some,
        forall|i: int| 0 <= i < u.len() ==> a::number(#[trigger] u[i]) is Some,
        ws::tuple_pairs(g, sel, t) is Some,
        ws::tuple_pairs(g, sel, u) is Some,
    ensures
        forall|kk: int|
            0 <= kk < sel.chain.len() ==> #[trigger] node_at(
                wenv(p, g, sel, j),
                tvals(sel, t),
                slot_th(sel, t),
                kk,
            ),
        forall|kk: int|
            0 <= kk < sel.chain.len() ==> #[trigger] node_at(
                wenv(p, g, sel, j),
                tvals(sel, u),
                slot_th(sel, u),
                kk,
            ),
{
    tvals_facts(g, sel, t);
    tvals_facts(g, sel, u);
    assert forall|kk: int| 0 <= kk < sel.chain.len() implies #[trigger] node_at(
        wenv(p, g, sel, j),
        tvals(sel, t),
        slot_th(sel, t),
        kk,
    ) by {
        cin_good(p, g, sel, j, t, kk, 0);
        cin_good(p, g, sel, j, t, kk, 2);
    }
    assert forall|kk: int| 0 <= kk < sel.chain.len() implies #[trigger] node_at(
        wenv(p, g, sel, j),
        tvals(sel, u),
        slot_th(sel, u),
        kk,
    ) by {
        cin_good(p, g, sel, j, u, kk, 0);
        cin_good(p, g, sel, j, u, kk, 2);
    }
}

// The context's placeholders: below the walk's offset, pairwise apart.
#[verifier::spinoff_prover]
proof fn chain_phs(g: spec::Group, sel: ws::Sel, j: int, off: nat)
    requires
        ws::chain_shape(sel),
        ws::chain_body(sel, g.clauses[j].body),
        0 <= j < g.clauses.len(),
        off == hoff(g, sel, j) + clause_nvars(spec::canon_clause(g.clauses[j])),
    ensures
        forall|m: int|
            0 <= m < sel.chain.len() ==> #[trigger] ph(g, sel, j, 4)[m] < off && ph(g, sel, j, 5)[m]
                < off,
        forall|m1: int, m2: int|
            0 <= m1 < sel.chain.len() && 0 <= m2 < sel.chain.len() ==> #[trigger] ph(
                g,
                sel,
                j,
                4,
            )[m1] != #[trigger] ph(g, sel, j, 5)[m2],
        forall|m1: int, m2: int|
            0 <= m1 < sel.chain.len() && 0 <= m2 < sel.chain.len() && m1 != m2 ==> #[trigger] ph(
                g,
                sel,
                j,
                4,
            )[m1] != #[trigger] ph(g, sel, j, 4)[m2],
        forall|m1: int, m2: int|
            0 <= m1 < sel.chain.len() && 0 <= m2 < sel.chain.len() && m1 != m2 ==> #[trigger] ph(
                g,
                sel,
                j,
                5,
            )[m1] != #[trigger] ph(g, sel, j, 5)[m2],
{
    let fs = fsj(g, j);
    let o4 = ph(g, sel, j, 4);
    let o5 = ph(g, sel, j, 5);
    assert forall|m: int| 0 <= m < sel.chain.len() implies #[trigger] o4[m] < off && o5[m]
        < off by {
        out_in(g, sel, j, m, 4);
        out_in(g, sel, j, m, 5);
    }
    keys_index(sel);
    outputs_index(sel);
    let nops = sel.ops.len() as int;
    assert(keys(sel).no_duplicates());
    assert forall|m1: int, m2: int|
        0 <= m1 < sel.chain.len() && 0 <= m2 < sel.chain.len() implies #[trigger] o4[m1]
        != #[trigger] o5[m2] by {
        out_in(g, sel, j, m1, 4);
        out_in(g, sel, j, m2, 5);
        pos_of_at(fs, arg(sel.chain[m1], 4)->Var_0);
        pos_of_at(fs, arg(sel.chain[m2], 5)->Var_0);
        assert(keys(sel)[2 * nops + 2 * m1] != keys(sel)[2 * nops + 2 * m2 + 1]);
    }
    assert forall|m1: int, m2: int|
        0 <= m1 < sel.chain.len() && 0 <= m2 < sel.chain.len() && m1 != m2 implies #[trigger] o4[m1]
        != #[trigger] o4[m2] by {
        out_in(g, sel, j, m1, 4);
        out_in(g, sel, j, m2, 4);
        pos_of_at(fs, arg(sel.chain[m1], 4)->Var_0);
        pos_of_at(fs, arg(sel.chain[m2], 4)->Var_0);
        assert(keys(sel)[2 * nops + 2 * m1] != keys(sel)[2 * nops + 2 * m2]);
    }
    assert forall|m1: int, m2: int|
        0 <= m1 < sel.chain.len() && 0 <= m2 < sel.chain.len() && m1 != m2 implies #[trigger] o5[m1]
        != #[trigger] o5[m2] by {
        out_in(g, sel, j, m1, 5);
        out_in(g, sel, j, m2, 5);
        pos_of_at(fs, arg(sel.chain[m1], 5)->Var_0);
        pos_of_at(fs, arg(sel.chain[m2], 5)->Var_0);
        assert(keys(sel)[2 * nops + 2 * m1 + 1] != keys(sel)[2 * nops + 2 * m2 + 1]);
    }
}

// The context's chain: both runs' node values, the placeholders below the
// walk's offset and pairwise apart.
pub proof fn chain_good(
    p: Seq<DocClause>,
    g: spec::Group,
    sel: ws::Sel,
    j: int,
    t: Seq<Term>,
    u: Seq<Term>,
)
    requires
        ws::in_fragment(p, g, sel),
        0 <= j < g.clauses.len(),
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int|
            0 <= i < sel.ops.len() ==> ws::same_cell(
                ws::boundaries(g, sel, i),
                #[trigger] t[i],
                u[i],
            ),
        ws::tuple_pairs(g, sel, t) is Some,
        ws::tuple_pairs(g, sel, u) is Some,
    ensures
        chain_ok(mk_cx(p, g, sel, j, t, u)),
{
    let cx = mk_cx(p, g, sel, j, t, u);
    assert(ws::counts(sel).no_duplicates());
    assert(ws::chain_body(sel, g.clauses[j].body));
    assert forall|i: int| 0 <= i < t.len() implies a::number(#[trigger] t[i]) is Some by {
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    assert forall|i: int| 0 <= i < u.len() implies a::number(#[trigger] u[i]) is Some by {
        assert(ws::same_cell(ws::boundaries(g, sel, i), t[i], u[i]));
    }
    tvals_facts(g, sel, t);
    tvals_facts(g, sel, u);
    chain_nodes(p, g, sel, j, t, u);
    chain_phs(g, sel, j, cx.e.off);
}

} // verus!
