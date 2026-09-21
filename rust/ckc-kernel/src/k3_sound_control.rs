use super::frontier::*;
use super::goals::*;
use super::graph::*;
use super::shapes::*;
use super::state::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn cuts_mono(gs: Seq<TGoal>, a: nat, b: nat)
    requires
        cuts_below(gs, a),
        a <= b,
    ensures
        cuts_below(gs, b),
{
    assert forall|i: int| 0 <= i < gs.len() && (#[trigger] goal_frame(gs[i])).0 implies goal_frame(
        gs[i],
    ).1 < b by {
        assert(goal_frame(gs[i]).1 < a);
    }
}

pub proof fn cuts_add(a: Seq<TGoal>, b: Seq<TGoal>, n: nat)
    requires
        cuts_below(a, n),
        cuts_below(b, n),
    ensures
        cuts_below(a + b, n),
{
    assert forall|i: int|
        0 <= i < (a + b).len() && (#[trigger] goal_frame((a + b)[i])).0 implies goal_frame(
        (a + b)[i],
    ).1 < n by {
        if i < a.len() {
            assert((a + b)[i] == a[i]);
        } else {
            assert((a + b)[i] == b[i - a.len()]);
        }
    }
}

pub proof fn cuts_tail(gs: Seq<TGoal>, n: nat)
    requires
        gs.len() > 0,
        cuts_below(gs, n),
    ensures
        cuts_below(gs.drop_first(), n),
{
    assert forall|i: int|
        0 <= i < gs.drop_first().len() && (#[trigger] goal_frame(
            gs.drop_first()[i],
        )).0 implies goal_frame(gs.drop_first()[i]).1 < n by {
        assert(gs.drop_first()[i] == gs[i + 1]);
    }
}

pub proof fn in_naf_tail(gs: Seq<TGoal>)
    requires
        gs.len() > 0,
        gs[0] is Lit,
    ensures
        in_naf(gs) == in_naf(gs.drop_first()),
{
    if in_naf(gs) {
        let i = choose|i: int| 0 <= i < gs.len() && gs[i] is NafCut;
        assert(i > 0);
        assert(gs.drop_first()[i - 1] is NafCut);
    }
    if in_naf(gs.drop_first()) {
        let i = choose|i: int| 0 <= i < gs.drop_first().len() && gs.drop_first()[i] is NafCut;
        assert(gs[i + 1] is NafCut);
    }
}

pub proof fn in_naf_suffix(a: Seq<TGoal>, b: Seq<TGoal>)
    requires
        in_naf(b),
    ensures
        in_naf(a + b),
{
    let i = choose|i: int| 0 <= i < b.len() && b[i] is NafCut;
    assert((a + b)[a.len() + i] is NafCut);
}

pub proof fn tbody_cuts(items: Seq<BodyItem>, off: nat, d: nat, path: Seq<nat>, n: nat)
    ensures
        cuts_below(tbody_goals(items, off, d, path), n),
{
    assert forall|i: int| 0 <= i < items.len() implies !(#[trigger] goal_frame(
        tbody_goals(items, off, d, path)[i],
    )).0 by {};
}

pub open spec fn slot_key(g: TGoal) -> (bool, nat, Term, Seq<nat>) {
    match g {
        TGoal::Lit(t, _, path) => (false, 0, t, path),
        TGoal::NafCut(lvl) => (true, lvl, Term::Nil, seq![]),
    }
}

pub open spec fn same_slots(a: Seq<TGoal>, b: Seq<TGoal>) -> bool {
    a.len() == b.len() && forall|i: int|
        0 <= i < a.len() ==> #[trigger] slot_key(a[i]) == #[trigger] slot_key(b[i])
}

pub proof fn slot_keys_properties(a: Seq<TGoal>, b: Seq<TGoal>, n: nat)
    requires
        same_slots(a, b),
    ensures
        paths(a) == paths(b),
        in_naf(a) == in_naf(b),
        cuts_below(a, n) == cuts_below(b, n),
{
    assert forall|i: int| 0 <= i < a.len() implies #[trigger] paths(a)[i] == paths(b)[i] by {
        assert(slot_key(a[i]) == slot_key(b[i]));
    }
    assert(paths(a) =~= paths(b));
    if in_naf(a) {
        let i = choose|i: int| 0 <= i < a.len() && a[i] is NafCut;
        assert(slot_key(a[i]) == slot_key(b[i]));
        assert(b[i] is NafCut);
    }
    if in_naf(b) {
        let i = choose|i: int| 0 <= i < b.len() && b[i] is NafCut;
        assert(slot_key(a[i]) == slot_key(b[i]));
        assert(a[i] is NafCut);
    }
    if cuts_below(a, n) {
        assert forall|i: int|
            0 <= i < b.len() && (#[trigger] goal_frame(b[i])).0 implies goal_frame(b[i]).1 < n by {
            assert(slot_key(a[i]) == slot_key(b[i]));
            assert(goal_frame(a[i]).0 && goal_frame(a[i]).1 < n);
        }
    }
    if cuts_below(b, n) {
        assert forall|i: int|
            0 <= i < a.len() && (#[trigger] goal_frame(a[i])).0 implies goal_frame(a[i]).1 < n by {
            assert(slot_key(a[i]) == slot_key(b[i]));
            assert(goal_frame(b[i]).0 && goal_frame(b[i]).1 < n);
        }
    }
}

pub proof fn state_transport(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    v: Snapshot,
    next: Snapshot,
    state: Option<Cert>,
)
    requires
        state_valid(db, roots, v, state),
        v.log == next.log,
        v.fresh == next.fresh,
        same_slots(v.stack, next.stack),
    ensures
        state_valid(db, roots, next, state),
{
    slot_keys_properties(v.stack, next.stack, 0);
    if let Option::Some(cert) = state {
        assert forall|i: int| 0 <= i < next.stack.len() implies #[trigger] slot_valid(
            db,
            roots,
            next,
            cert,
            next.stack[i],
        ) by {
            assert(slot_valid(db, roots, v, cert, v.stack[i]));
            assert(slot_key(v.stack[i]) == slot_key(next.stack[i]));
        }
    }
}

pub proof fn naf_alternative(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    c: TCfg,
    aux: Aux,
    inner: Term,
    depth: nat,
    path: Seq<nat>,
    rest: Seq<TGoal>,
)
    requires
        aux_valid(db, roots, c, aux),
        c.stack == seq![TGoal::Lit(Term::Comp(naf_name(), seq![inner]), depth, path)] + rest,
    ensures
        alt_valid(
            db,
            roots,
            TAlt::Naf { stack: rest, fresh: c.fresh, log: c.log, path, inner, pruned: c.pruned },
            aux.current,
            c.alts.len(),
        ),
{
    let a = TAlt::Naf { stack: rest, fresh: c.fresh, log: c.log, path, inner, pruned: c.pruned };
    let next = alt_snapshot(a);
    assert forall|i: int| 0 <= i < c.stack.len() implies #[trigger] slot_key(c.stack[i])
        == #[trigger] slot_key(next.stack[i]) by {
        if i > 0 {
            assert(c.stack[i] == next.stack[i]);
        }
    }
    assert(same_slots(c.stack, next.stack));
    state_transport(db, roots, snapshot(c), next, aux.current);
    slot_keys_properties(c.stack, next.stack, c.alts.len());
}

pub proof fn outer_literal_shape(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    v: Snapshot,
    cert: Cert,
    i: int,
)
    requires
        outer_valid(db, roots, v, cert),
        0 <= i < v.stack.len(),
    ensures
        v.stack[i] is Lit,
        trace_literal(tgoal_term(v.stack[i])),
        tgoal_term(v.stack[i]) matches Term::Comp(_, args) ==> args.len() != 2,
{
    assert(slot_valid(db, roots, v, cert, v.stack[i]));
    if let TGoal::Lit(t, _, path) = v.stack[i] {
        trace_literal_apply(raw_at(db, roots, v.log, cert.offsets, path), cert.theta);
        if positive(t) {
            if let Term::Comp(name, args) = t {
                semantic_arity(name, args.len());
            }
        }
    }
}

} // verus!
