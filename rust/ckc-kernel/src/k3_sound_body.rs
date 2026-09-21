use super::freshness::*;
use super::shapes::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn base_item(it: BodyItem) -> Term {
    match it {
        BodyItem::Pos(t) => t,
        BodyItem::Naf(gs) => Term::Comp(naf_name(), seq![conj_term(gs)]),
    }
}

pub open spec fn item_variables(it: BodyItem) -> nat {
    match it {
        BodyItem::Pos(t) => nvars(t),
        BodyItem::Naf(gs) => nvars_all(gs),
    }
}

pub proof fn conj_shift(gs: Seq<Term>, off: nat)
    requires
        gs.len() > 0,
    ensures
        conj_term(shift_all(gs, off)) == shift(conj_term(gs), off),
    decreases gs.len(),
{
    shift_all_map(gs, off);
    if gs.len() > 1 {
        conj_shift(gs.drop_first(), off);
        let tail = conj_term(gs.drop_first());
        shift_all_map(seq![gs[0], tail], off);
        assert(shift_all(seq![gs[0], tail], off) =~= seq![shift(gs[0], off), shift(tail, off)]);
        assert(shift_all(gs, off).drop_first() =~= shift_all(gs.drop_first(), off));
        assert(shift(conj_term(gs), off) == Term::Comp(
            comma_name(),
            seq![shift(gs[0], off), shift(tail, off)],
        ));
    } else {
        assert(conj_term(gs) == gs[0]);
        assert(conj_term(shift_all(gs, off)) == shift(gs[0], off));
    }
}

pub proof fn item_shift(it: BodyItem, off: nat)
    requires
        wf_body_item(it),
    ensures
        item_term(it, off) == shift(base_item(it), off),
{
    if let BodyItem::Naf(gs) = it {
        conj_shift(gs, off);
        shift_all_map(seq![conj_term(gs)], off);
        assert(shift_all(seq![conj_term(gs)], off) =~= seq![shift(conj_term(gs), off)]);
    }
}

pub proof fn item_fresh(it: BodyItem, off: nat, s: Seq<(nat, Term)>)
    requires
        wf_body_item(it),
        binds_below(s, off),
    ensures
        apply(item_term(it, off), s) == item_term(it, off),
{
    item_shift(it, off);
    apply_fresh(base_item(it), off, s);
}

pub proof fn nvars_cons(t: Term, ts: Seq<Term>)
    ensures
        nvars_all(seq![t] + ts) == max_nat(nvars(t), nvars_all(ts)),
{
    assert((seq![t] + ts).drop_first() =~= ts);
}

pub proof fn conj_variables(gs: Seq<Term>)
    requires
        gs.len() > 0,
    ensures
        nvars(conj_term(gs)) == nvars_all(gs),
    decreases gs.len(),
{
    nvars_cons(gs[0], gs.drop_first());
    assert(seq![gs[0]] + gs.drop_first() =~= gs);
    if gs.len() > 1 {
        conj_variables(gs.drop_first());
        let tail = conj_term(gs.drop_first());
        nvars_cons(tail, seq![]);
        nvars_cons(gs[0], seq![tail]);
        assert(seq![tail] + seq![] =~= seq![tail]);
        assert(seq![gs[0]] + seq![tail] =~= seq![gs[0], tail]);
        assert(nvars(conj_term(gs)) == nvars_all(seq![gs[0], tail]));
    }
}

pub proof fn base_item_variables(it: BodyItem)
    requires
        wf_body_item(it),
    ensures
        nvars(base_item(it)) == item_variables(it),
{
    if let BodyItem::Naf(gs) = it {
        conj_variables(gs);
        nvars_cons(conj_term(gs), seq![]);
        assert(seq![conj_term(gs)] + seq![] =~= seq![conj_term(gs)]);
    }
}

pub proof fn item_bound(it: BodyItem, off: nat)
    requires
        wf_body_item(it),
    ensures
        nvars(item_term(it, off)) <= off + item_variables(it),
{
    item_shift(it, off);
    base_item_variables(it);
    shift_bound(base_item(it), off);
}

pub proof fn item_variables_bound(items: Seq<BodyItem>, i: int)
    requires
        0 <= i < items.len(),
    ensures
        item_variables(items[i]) <= items_nvars(items),
    decreases items.len(),
{
    if i > 0 {
        item_variables_bound(items.drop_first(), i - 1);
    }
}

pub proof fn body_bound(db: Seq<DocClause>, m: nat, off: nat)
    requires
        bodies_wf(db),
        m < db.len(),
    ensures
        forall|i: int|
            0 <= i < db[m as int].body.len() ==> #[trigger] nvars(
                item_term(db[m as int].body[i], off),
            ) <= off + clause_nvars(db[m as int]),
{
    assert forall|i: int| 0 <= i < db[m as int].body.len() implies #[trigger] nvars(
        item_term(db[m as int].body[i], off),
    ) <= off + clause_nvars(db[m as int]) by {
        assert(wf_body_item(db[m as int].body[i]));
        item_bound(db[m as int].body[i], off);
        item_variables_bound(db[m as int].body, i);
    }
}

pub proof fn head_match(db: Seq<DocClause>, name: Seq<u8>, arity: nat, from: nat)
    ensures
        next_match(db, name, arity, from) matches Option::Some(m) ==> lit_fa(db[m as int].head)
            == Option::Some((name, arity)),
    decreases db.len() - from,
{
    next_match_bound(db, name, arity, from);
    if from < db.len() && lit_fa(db[from as int].head) != Option::Some((name, arity)) {
        head_match(db, name, arity, from + 1);
    }
}

pub proof fn shift_head_fa(head: Term, name: Seq<u8>, arity: nat, off: nat)
    requires
        lit_fa(head) == Option::Some((name, arity)),
    ensures
        lit_fa(shift(head, off)) == Option::Some((name, arity)),
        args_of(shift(head, off)).len() == arity,
{
    if let Term::Comp(_, args) = head {
        shift_all_map(args, off);
    }
}

} // verus!
