use super::bounded::*;
use super::freshness::*;
use super::unification::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn tapply(g: TGoal, s: Seq<(nat, Term)>) -> TGoal {
    tgoal_with(g, apply(tgoal_term(g), s))
}

pub open spec fn tapply_all(gs: Seq<TGoal>, s: Seq<(nat, Term)>) -> Seq<TGoal> {
    gs.map_values(|g: TGoal| tapply(g, s))
}

pub open spec fn goal_frame(g: TGoal) -> (bool, nat, Seq<nat>) {
    match g {
        TGoal::Lit(_, d, path) => (false, d, path),
        TGoal::NafCut(lvl) => (true, lvl, seq![]),
    }
}

pub open spec fn same_frames(a: Seq<TGoal>, b: Seq<TGoal>) -> bool {
    a.len() == b.len() && forall|i: int|
        0 <= i < a.len() ==> #[trigger] goal_frame(a[i]) == #[trigger] goal_frame(b[i])
}

pub open spec fn cuts_below(gs: Seq<TGoal>, n: nat) -> bool {
    forall|i: int| 0 <= i < gs.len() && (#[trigger] goal_frame(gs[i])).0 ==> goal_frame(gs[i]).1 < n
}

pub open spec fn goals_below(gs: Seq<TGoal>, b: nat) -> bool {
    forall|i: int| 0 <= i < gs.len() ==> #[trigger] nvars(tgoal_term(gs[i])) <= b
}

pub open spec fn tunifier_witness(
    pairs: Seq<(Term, Term)>,
    stack: Seq<TGoal>,
    out: TUni,
    b: nat,
    s: Seq<(nat, Term)>,
) -> bool {
    binds_below(s, b) && solves_all(pairs, s) && out == TUni::Ok(tapply_all(stack, s))
}

pub proof fn tapply_compose(g: TGoal, s: Seq<(nat, Term)>, r: Seq<(nat, Term)>)
    ensures
        tapply(tapply(g, s), r) == tapply(g, s + r),
{
    if let TGoal::Lit(t, _, _) = g {
        apply_append(t, s, r);
    }
}

pub proof fn tapply_all_compose(gs: Seq<TGoal>, s: Seq<(nat, Term)>, r: Seq<(nat, Term)>)
    ensures
        tapply_all(tapply_all(gs, s), r) == tapply_all(gs, s + r),
{
    assert forall|i: int| 0 <= i < gs.len() implies #[trigger] tapply_all(tapply_all(gs, s), r)[i]
        == tapply_all(gs, s + r)[i] by {
        tapply_compose(gs[i], s, r);
    }
    assert(tapply_all(tapply_all(gs, s), r) =~= tapply_all(gs, s + r));
}

pub proof fn tapply_all_add(a: Seq<TGoal>, b: Seq<TGoal>, s: Seq<(nat, Term)>)
    ensures
        tapply_all(a + b, s) == tapply_all(a, s) + tapply_all(b, s),
{
    assert(tapply_all(a + b, s) =~= tapply_all(a, s) + tapply_all(b, s));
}

pub proof fn tunify_frames(pairs: Seq<(Term, Term)>, stack: Seq<TGoal>)
    ensures
        tunify(pairs, stack) matches TUni::Ok(out) ==> same_frames(stack, out),
{
    let u = UState { pairs, stack: seq![], sol: stack.map_values(|g: TGoal| tgoal_term(g)) };
    if let UOut::Ok(_, terms) = unify(u) {
        let out = Seq::new(stack.len(), |i: int| tgoal_with(stack[i], terms[i]));
        assert forall|i: int| 0 <= i < stack.len() implies #[trigger] goal_frame(stack[i])
            == #[trigger] goal_frame(out[i]) by {
            match stack[i] {
                TGoal::Lit(_, _, _) => {},
                TGoal::NafCut(_) => {},
            }
        }
    }
}

pub proof fn in_naf_frames(a: Seq<TGoal>, b: Seq<TGoal>)
    requires
        same_frames(a, b),
    ensures
        in_naf(a) == in_naf(b),
{
    if in_naf(a) {
        let i = choose|i: int| 0 <= i < a.len() && a[i] is NafCut;
        assert(goal_frame(a[i]) == goal_frame(b[i]));
        assert(b[i] is NafCut);
    }
    if in_naf(b) {
        let i = choose|i: int| 0 <= i < b.len() && b[i] is NafCut;
        assert(goal_frame(a[i]) == goal_frame(b[i]));
        assert(a[i] is NafCut);
    }
}

pub proof fn cuts_frames(a: Seq<TGoal>, b: Seq<TGoal>, n: nat)
    requires
        same_frames(a, b),
        cuts_below(a, n),
    ensures
        cuts_below(b, n),
{
    assert forall|i: int| 0 <= i < b.len() && (#[trigger] goal_frame(b[i])).0 implies goal_frame(
        b[i],
    ).1 < n by {
        assert(goal_frame(a[i]) == goal_frame(b[i]));
    }
}

pub proof fn tapply_below(gs: Seq<TGoal>, s: Seq<(nat, Term)>, b: nat)
    requires
        goals_below(gs, b),
        binds_below(s, b),
    ensures
        goals_below(tapply_all(gs, s), b),
{
    assert forall|i: int| 0 <= i < gs.len() implies #[trigger] nvars(
        tgoal_term(tapply_all(gs, s)[i]),
    ) <= b by {
        if let TGoal::Lit(t, _, _) = gs[i] {
            apply_below(t, s, b);
        }
    }
}

pub proof fn tunify_bounded(pairs: Seq<(Term, Term)>, stack: Seq<TGoal>, b: nat)
    requires
        tunify(pairs, stack) is Ok,
        pairs_below(pairs, b),
    ensures
        exists|s: Seq<(nat, Term)>| #[trigger]
            tunifier_witness(pairs, stack, tunify(pairs, stack), b, s),
{
    let u = UState { pairs, stack: seq![], sol: stack.map_values(|g: TGoal| tgoal_term(g)) };
    unify_bounded(u, b);
    let s = choose|s: Seq<(nat, Term)>| bounded_witness(u, unify(u), b, s);
    if let UOut::Ok(_, terms) = unify(u) {
        assert(Seq::new(stack.len(), |i: int| tgoal_with(stack[i], terms[i])) =~= tapply_all(
            stack,
            s,
        ));
        assert(tunifier_witness(pairs, stack, tunify(pairs, stack), b, s));
    }
}

} // verus!
