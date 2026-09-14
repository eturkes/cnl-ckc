use super::unification::*;
use super::*;
use ckc_spec::answers::goal_walk;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn positive(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => is_semantic_pred(name, args.len()),
        _ => false,
    }
}

pub open spec fn trace_literal(t: Term) -> bool {
    positive(t) || match t {
        Term::Comp(name, args) => name == naf_name() && args.len() == 1 && goal_walk(
            args[0],
        ) is None,
        _ => false,
    }
}

pub proof fn semantic_arity(name: Seq<u8>, arity: nat)
    requires
        is_semantic_pred(name, arity),
    ensures
        3 <= arity <= 5,
{
    let i = choose|i: int| 2 <= i < 9 && indicator(i) == (name, arity);
}

pub proof fn shift_all_map(ts: Seq<Term>, off: nat)
    ensures
        shift_all(ts, off) == ts.map_values(|t: Term| shift(t, off)),
    decreases ts.len(),
{
    if ts.len() > 0 {
        shift_all_map(ts.drop_first(), off);
    }
    assert(shift_all(ts, off) =~= ts.map_values(|t: Term| shift(t, off)));
}

pub proof fn positive_walk(t: Term)
    requires
        positive(t),
    ensures
        goal_walk(t) is None,
{
    if let Term::Comp(name, args) = t {
        semantic_arity(name, args.len());
    }
}

pub proof fn positive_shift(t: Term, off: nat)
    requires
        positive(t),
    ensures
        positive(shift(t, off)),
{
    if let Term::Comp(_, args) = t {
        shift_all_map(args, off);
    }
}

pub proof fn positive_apply(t: Term, s: Seq<(nat, Term)>)
    requires
        positive(t),
    ensures
        positive(apply(t, s)),
{
    if let Term::Comp(name, args) = t {
        apply_comp(name, args, s);
    }
}

pub proof fn goal_walk_subst(t: Term, x: nat, v: Term)
    requires
        goal_walk(t) is None,
    ensures
        goal_walk(subst(t, x, v)) is None,
    decreases t,
{
    if let Term::Comp(name, args) = t {
        subst_all_map(args, x, v);
        if name == comma_name() && args.len() == 2 {
            goal_walk_subst(args[0], x, v);
            goal_walk_subst(args[1], x, v);
        }
    }
}

pub proof fn goal_walk_shift(t: Term, off: nat)
    requires
        goal_walk(t) is None,
    ensures
        goal_walk(shift(t, off)) is None,
    decreases t,
{
    if let Term::Comp(name, args) = t {
        shift_all_map(args, off);
        if name == comma_name() && args.len() == 2 {
            goal_walk_shift(args[0], off);
            goal_walk_shift(args[1], off);
        }
    }
}

pub proof fn goal_walk_apply(t: Term, s: Seq<(nat, Term)>)
    requires
        goal_walk(t) is None,
    ensures
        goal_walk(apply(t, s)) is None,
    decreases s.len(),
{
    if s.len() > 0 {
        goal_walk_subst(t, s[0].0, s[0].1);
        goal_walk_apply(subst(t, s[0].0, s[0].1), s.drop_first());
    }
}

pub proof fn root_shapes(t: Term)
    requires
        goal_walk(t) is None,
    ensures
        forall|i: int| 0 <= i < conj_leaves(t).len() ==> #[trigger] positive(conj_leaves(t)[i]),
    decreases t,
{
    if let Term::Comp(name, args) = t {
        if name == comma_name() && args.len() == 2 {
            root_shapes(args[0]);
            root_shapes(args[1]);
            let left = conj_leaves(args[0]);
            let right = conj_leaves(args[1]);
            assert forall|i: int| 0 <= i < (left + right).len() implies #[trigger] positive(
                (left + right)[i],
            ) by {
                if i < left.len() {
                    assert((left + right)[i] == left[i]);
                } else {
                    assert((left + right)[i] == right[i - left.len()]);
                }
            }
        }
    }
}

pub proof fn conj_term_walk(gs: Seq<Term>)
    requires
        gs.len() > 0,
        forall|i: int| 0 <= i < gs.len() ==> #[trigger] positive(gs[i]),
    ensures
        goal_walk(conj_term(gs)) is None,
    decreases gs.len(),
{
    positive_walk(gs[0]);
    if gs.len() > 1 {
        assert forall|i: int| 0 <= i < gs.drop_first().len() implies #[trigger] positive(
            gs.drop_first()[i],
        ) by {
            assert(gs.drop_first()[i] == gs[i + 1]);
        }
        conj_term_walk(gs.drop_first());
    }
}

pub proof fn trace_literal_apply(t: Term, s: Seq<(nat, Term)>)
    requires
        trace_literal(t),
    ensures
        trace_literal(apply(t, s)),
{
    if positive(t) {
        positive_apply(t, s);
    } else if let Term::Comp(name, args) = t {
        apply_comp(name, args, s);
        goal_walk_apply(args[0], s);
    }
}

pub proof fn item_literal(it: BodyItem, off: nat)
    requires
        wf_body_item(it),
    ensures
        trace_literal(item_term(it, off)),
{
    match it {
        BodyItem::Pos(t) => {
            positive_shift(t, off);
        },
        BodyItem::Naf(gs) => {
            shift_all_map(gs, off);
            assert forall|i: int| 0 <= i < shift_all(gs, off).len() implies #[trigger] positive(
                shift_all(gs, off)[i],
            ) by {
                assert(wf_literal(gs[i]));
                positive_shift(gs[i], off);
            }
            conj_term_walk(shift_all(gs, off));
        },
    }
}

pub proof fn body_literals(db: Seq<DocClause>, m: nat, off: nat)
    requires
        bodies_wf(db),
        m < db.len(),
    ensures
        forall|i: int|
            0 <= i < body_terms(db[m as int].body, off).len() ==> #[trigger] trace_literal(
                body_terms(db[m as int].body, off)[i],
            ),
{
    assert forall|i: int|
        0 <= i < body_terms(db[m as int].body, off).len() implies #[trigger] trace_literal(
        body_terms(db[m as int].body, off)[i],
    ) by {
        assert(wf_body_item(db[m as int].body[i]));
        item_literal(db[m as int].body[i], off);
    }
}

} // verus!
