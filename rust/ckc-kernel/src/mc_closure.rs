use crate::k2_engine::{EBodyItem, EClause};
use crate::k2_term::ETermArena;
use crate::mc_model::EMcModel;
use ckc_spec::term::Term;
use ckc_spec::v1text::{BodyItem, DocClause};
use vstd::prelude::*;

verus! {

// Contract mc D6 closure: every active ground instance whose body holds in M has its head in M.
// One clause = a DFS over its body items by joins over M (left-to-right safety bounds every
// built-in input and every negation's outer variable before the item).
pub open spec fn items_from(m: ISet<Term>, c: DocClause, j: int, t3: Map<nat, Term>) -> bool {
    forall|q: int|
        j <= q < c.body.len() ==> #[trigger] ckc_spec::model::item_holds(m, m, c.body[q], t3)
}

// The head of every valuation extending th that satisfies items j.. lies in M.
pub open spec fn heads_in(m: ISet<Term>, c: DocClause, j: int, th: Map<nat, Term>) -> bool {
    forall|t3: Map<nat, Term>|
        ckc_spec::model::valuation(c, t3) && crate::mc_subst::extends(t3, th) && items_from(
            m,
            c,
            j,
            t3,
        ) ==> m.contains(ckc_spec::model::apply(c.head, t3))
}

// Variables of a negation occurring in a valuation's domain are bound before it.
proof fn naf_vars_bound(c: DocClause, j: int, t3: Map<nat, Term>, x: nat)
    requires
        ckc_spec::model::safe_clause(c),
        0 <= j < c.body.len(),
        c.body[j] is Naf,
        ckc_spec::model::valuation(c, t3),
        t3.contains_key(x),
        ckc_spec::model::vars_all(c.body[j]->Naf_0).contains(x),
    ensures
        crate::mc_lemma::binds_upto(c, j).contains(x),
{
    crate::mc_lemma::naf_safe_at(c, j);
    assert(ckc_spec::model::nonlocal(c, x));
    if !ckc_spec::model::vars(c.head).contains(x) {
        let q = choose|q: int|
            0 <= q < c.body.len() && #[trigger] ckc_spec::model::item_vars(c.body[q]).contains(x)
                && (c.body[q] is Pos || ckc_spec::model::outside(c, q, x));
        if q != j {
            assert(ckc_spec::model::outside(c, j, x));
        }
    }
}

pub fn close_items(
    arena: &mut ETermArena,
    clause: &EClause,
    Ghost(cc): Ghost<DocClause>,
    n: usize,
    j: usize,
    sub: &Vec<Option<usize>>,
    Ghost(th): Ghost<Map<nat, Term>>,
    m: &EMcModel,
) -> (out: Result<(), usize>)
    requires
        crate::k2_term::arena_ok(old(arena)),
        crate::k2_engine::clause_valid(old(arena).nodes@, clause),
        cc == crate::k2_engine::clause_view(old(arena).nodes@, clause),
        ckc_spec::model::g1_clause(cc),
        ckc_spec::model::safe_clause(cc),
        crate::mc_safe::vars_below(cc, n as nat),
        j <= clause.body@.len(),
        sub@.len() == n,
        crate::mc_subst::sub_ok(old(arena).nodes@, sub@, th),
        th.dom() == crate::mc_lemma::binds_upto(cc, j as int),
        crate::mc_model::model_ok(old(arena).nodes@, m),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out is Ok ==> heads_in(m.ms@.to_iset(), cc, j as int, th),
        out matches Err(r) ==> r < final(arena).nodes@.len(),
    decreases clause.body@.len() - j,
{
    let ghost origin = arena.nodes@;
    let ghost mm = m.ms@.to_iset();
    proof {
        assert(origin.is_prefix_of(origin)) by {
            assert(origin.take(origin.len() as int) =~= origin);
        }
        crate::mc_model::sub_vals_wf(arena, sub@, th);
    }
    if j == clause.body.len() {
        // head vars ⊆ binders(body) = the bound set at the end
        proof {
            assert(ckc_spec::model::vars(cc.head).subset_of(ckc_spec::model::binders(cc.body)));
            assert forall|x: nat| #[trigger]
                ckc_spec::engine::occurs(
                    x,
                    origin[clause.head as int].term@,
                ) implies th.contains_key(x) by {
                crate::mc_safe::vars_occurs(cc.head, x);
                assert(ckc_spec::model::vars(cc.head).subset_of(ckc_spec::model::binders(cc.body)));
                binders_are_bound(cc, x);
            }
        }
        let inst = crate::mc_subst::instantiate(arena, clause.head, sub, Ghost(th));
        proof {
            crate::mc_subst::apply_ground(cc.head, th);
            crate::mc_model::model_ok_prefix(origin, arena.nodes@, m);
        }
        if !crate::mc_model::in_model(arena, m, inst) {
            return Err(inst);
        }
        proof {
            assert forall|t3: Map<nat, Term>|
                ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                    && items_from(mm, cc, j as int, t3) implies mm.contains(
                ckc_spec::model::apply(cc.head, t3),
            ) by {
                assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, cc.head) implies (
                t3.contains_key(x) == th.contains_key(x) && (t3.contains_key(x) ==> t3[x]
                    == th[x])) by {
                    crate::mc_safe::vars_occurs(cc.head, x);
                    binders_are_bound(cc, x);
                }
                crate::mc_subst::apply_agree(cc.head, t3, th);
            }
        }
        return Ok(());
    }
    let j1 = j + 1;
    proof {
        assert(crate::k2_engine::body_item_valid(origin, &clause.body@[j as int]));
        assert(cc.body[j as int] == crate::k2_engine::body_item_view(
            origin,
            &clause.body@[j as int],
        ));
        assert(ckc_spec::model::g1_item(cc.body[j as int]));
        crate::mc_lemma::items_safe_at(cc, j as nat);
        crate::mc_lemma::items_safe_at(cc, (j + 1) as nat);
        assert(crate::mc_lemma::binds_upto(cc, j as int + 1) == crate::mc_lemma::binds_upto(
            cc,
            j as int,
        ).union(ckc_spec::model::item_binds(cc.body[j as int])));
    }
    match &clause.body[j] {
        EBodyItem::Naf { roots } => {
            let ghost gs = crate::k2_engine::root_terms(origin, roots@);
            proof {
                assert(cc.body[j as int] == BodyItem::Naf(gs));
                crate::mc_lemma::naf_safe_at(cc, j as int);
                assert(gs.skip(0) =~= gs);
                assert forall|q: int, x: nat|
                    0 <= q < roots@.len() && #[trigger] ckc_spec::engine::occurs(x, gs[q]) implies x
                    < sub@.len() by {
                    crate::mc_safe::vars_all_any(gs, x);
                    assert(ckc_spec::model::item_vars(cc.body[j as int]).contains(x));
                }
                assert forall|q: int| 0 <= q < roots@.len() implies (#[trigger] gs[q]) is Comp by {
                    assert(ckc_spec::model::g1_literal(gs[q]));
                }
            }
            let found = crate::mc_join::witness_from(arena, roots, 0, sub, Ghost(th), m);
            let ghost after = arena.nodes@;
            if found {
                proof {
                    // every valuation extending th defeats this negation
                    let w = choose|w: Map<nat, Term>|
                        crate::mc_subst::extends(w, th) && crate::mc_model::goals_sat(mm, gs, 0, w);
                    assert forall|t3: Map<nat, Term>|
                        ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                            && items_from(mm, cc, j as int, t3) implies mm.contains(
                        ckc_spec::model::apply(cc.head, t3),
                    ) by {
                        assert(ckc_spec::model::item_holds(mm, mm, cc.body[j as int], t3));
                        crate::mc_subst::apply_all_index(gs, t3);
                        crate::mc_subst::apply_all_index(gs, th);
                        assert(ckc_spec::model::apply_all(gs, t3) =~= ckc_spec::model::apply_all(
                            gs,
                            th,
                        )) by {
                            assert forall|q: int| 0 <= q < gs.len() implies ckc_spec::model::apply(
                                gs[q],
                                t3,
                            ) == ckc_spec::model::apply(gs[q], th) by {
                                assert forall|x: nat| #[trigger]
                                    ckc_spec::engine::occurs(x, gs[q]) implies (t3.contains_key(x)
                                    == th.contains_key(x) && (t3.contains_key(x) ==> t3[x]
                                    == th[x])) by {
                                    crate::mc_safe::vars_all_any(gs, x);
                                    if t3.contains_key(x) {
                                        naf_vars_bound(cc, j as int, t3, x);
                                    }
                                }
                                crate::mc_subst::apply_agree(gs[q], t3, th);
                            }
                        }
                        crate::mc_subst::apply_all_index(ckc_spec::model::apply_all(gs, th), w);
                        crate::mc_subst::apply_all_index(gs, w);
                        assert(ckc_spec::model::apply_all(ckc_spec::model::apply_all(gs, th), w)
                            =~= ckc_spec::model::apply_all(gs, w)) by {
                            assert forall|q: int| 0 <= q < gs.len() implies ckc_spec::model::apply(
                                ckc_spec::model::apply_all(gs, th)[q],
                                w,
                            ) == ckc_spec::model::apply(gs[q], w) by {
                                crate::mc_model::apply_compose(gs[q], th, w);
                                assert(w.union_prefer_right(th) =~= w);
                            }
                        }
                        crate::mc_model::conj_is_goals(mm, gs, w);
                        assert(ckc_spec::model::conj_holds(
                            mm,
                            ckc_spec::model::apply_all(ckc_spec::model::apply_all(gs, t3), w),
                        ));
                    }
                }
                return Ok(());
            }
            proof {
                crate::mc_subst::sub_ok_prefix(origin, after, sub@, th);
                crate::mc_model::model_ok_prefix(origin, after, m);
                crate::k2_engine::clause_models_prefix(origin, after, clause);
                assert(crate::k2_engine::clause_valid(after, clause));
                assert(cc == crate::k2_engine::clause_view(after, clause));
                assert(crate::mc_lemma::binds_upto(cc, j as int).union(Set::<nat>::empty())
                    =~= crate::mc_lemma::binds_upto(cc, j as int));
            }
            let r = close_items(arena, clause, Ghost(cc), n, j1, sub, Ghost(th), m);
            proof {
                crate::k2_load::prefix_chain(origin, after, arena.nodes@);
            }
            r
        },
        EBodyItem::Pos { root } => {
            let root = *root;
            let ghost l = origin[root as int].term@;
            proof {
                assert(cc.body[j as int] == BodyItem::Pos(l));
                assert(l is Comp);
                assert(crate::k2_term::node_ok(origin, root as int));
                assert(ckc_spec::term::wf_term(l));
                assert forall|x: nat| #[trigger]
                    ckc_spec::engine::occurs(x, origin[root as int].term@) implies x
                    < sub@.len() by {
                    crate::mc_safe::vars_occurs(l, x);
                    assert(ckc_spec::model::item_vars(cc.body[j as int]).contains(x));
                }
            }
            let (builtin, _kids, _k_in) = crate::mc_safe::literal_parts(arena, root);
            if builtin {
                proof {
                    assert(ckc_spec::model::lit_safe(l, th.dom()));
                }
                match crate::mc_join::builtin_step(arena, root, sub, Ghost(th), Ghost(mm)) {
                    None => {
                        proof {
                            assert forall|t3: Map<nat, Term>|
                                ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(
                                    t3,
                                    th,
                                ) && items_from(mm, cc, j as int, t3) implies mm.contains(
                                ckc_spec::model::apply(cc.head, t3),
                            ) by {
                                assert(ckc_spec::model::item_holds(mm, mm, cc.body[j as int], t3));
                                pos_goal_ok(cc, j as int, t3, mm);
                            }
                        }
                        return Ok(());
                    },
                    Some((s2, th2)) => {
                        let ghost after = arena.nodes@;
                        proof {
                            crate::mc_model::model_ok_prefix(origin, after, m);
                            crate::k2_engine::clause_models_prefix(origin, after, clause);
                            assert(crate::k2_engine::clause_valid(after, clause));
                            assert(cc == crate::k2_engine::clause_view(after, clause));
                        }
                        let r = close_items(arena, clause, Ghost(cc), n, j1, &s2, th2, m);
                        proof {
                            crate::k2_load::prefix_chain(origin, after, arena.nodes@);
                            if r is Ok {
                                assert forall|t3: Map<nat, Term>|
                                    ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(
                                        t3,
                                        th,
                                    ) && items_from(mm, cc, j as int, t3) implies mm.contains(
                                    ckc_spec::model::apply(cc.head, t3),
                                ) by {
                                    assert(ckc_spec::model::item_holds(
                                        mm,
                                        mm,
                                        cc.body[j as int],
                                        t3,
                                    ));
                                    pos_goal_ok(cc, j as int, t3, mm);
                                    assert(crate::mc_subst::extends(t3, th2@));
                                    assert(items_from(mm, cc, j as int + 1, t3));
                                }
                            }
                        }
                        r
                    },
                }
            } else {
                let mut i = 0usize;
                while i < m.facts.len()
                    invariant
                        crate::k2_term::arena_ok(arena),
                        origin == old(arena).nodes@,
                        origin.is_prefix_of(arena.nodes@),
                        crate::k2_engine::clause_valid(origin, clause),
                        cc == crate::k2_engine::clause_view(origin, clause),
                        ckc_spec::model::g1_clause(cc),
                        ckc_spec::model::safe_clause(cc),
                        crate::mc_safe::vars_below(cc, n as nat),
                        j < clause.body@.len(),
                        j1 == j + 1,
                        cc.body[j as int] == BodyItem::Pos(l),
                        l == origin[root as int].term@,
                        l is Comp,
                        !ckc_spec::model::builtin(l),
                        root < origin.len(),
                        sub@.len() == n,
                        crate::mc_subst::sub_ok(arena.nodes@, sub@, th),
                        th.dom() == crate::mc_lemma::binds_upto(cc, j as int),
                        crate::mc_lemma::binds_upto(cc, j as int + 1)
                            == crate::mc_lemma::binds_upto(cc, j as int).union(
                            ckc_spec::model::binds(l),
                        ),
                        crate::mc_model::model_ok(arena.nodes@, m),
                        mm == m.ms@.to_iset(),
                        i <= m.facts@.len(),
                        forall|t3: Map<nat, Term>|
                            ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                                && items_from(mm, cc, j as int, t3) && (exists|i2: int|
                                0 <= i2 < i && #[trigger] m.ms@[i2] == ckc_spec::model::apply(
                                    l,
                                    t3,
                                )) ==> mm.contains(ckc_spec::model::apply(cc.head, t3)),
                    decreases m.facts.len() - i,
                {
                    let ghost before = arena.nodes@;
                    let mut s2 = sub.clone();
                    proof {
                        assert(before[root as int] == origin[root as int]);
                        assert forall|x: nat| #[trigger]
                            ckc_spec::engine::occurs(x, before[root as int].term@) implies x
                            < s2@.len() by {
                            crate::mc_safe::vars_occurs(l, x);
                            assert(ckc_spec::model::item_vars(cc.body[j as int]).contains(x));
                        }
                        assert(m.facts@[i as int] < before.len());
                    }
                    let (ok, th2) = crate::mc_subst::match_term(
                        arena,
                        root,
                        m.facts[i],
                        &mut s2,
                        Ghost(th),
                    );
                    if ok {
                        proof {
                            assert(th2@.dom() =~= crate::mc_lemma::binds_upto(cc, j as int + 1))
                                by {
                                assert forall|x: nat|
                                    th2@.dom().contains(x) == crate::mc_lemma::binds_upto(
                                        cc,
                                        j as int + 1,
                                    ).contains(x) by {
                                    crate::mc_safe::vars_occurs(l, x);
                                }
                            }
                            crate::k2_engine::clause_models_prefix(origin, before, clause);
                            assert(crate::k2_engine::clause_valid(before, clause));
                            assert(cc == crate::k2_engine::clause_view(before, clause));
                        }
                        let r = close_items(arena, clause, Ghost(cc), n, j1, &s2, th2, m);
                        proof {
                            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
                            crate::mc_subst::sub_ok_prefix(before, arena.nodes@, sub@, th);
                            crate::mc_model::model_ok_prefix(before, arena.nodes@, m);
                        }
                        if let Err(e) = r {
                            return Err(e);
                        }
                        proof {
                            assert forall|t3: Map<nat, Term>|
                                ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(
                                    t3,
                                    th,
                                ) && items_from(mm, cc, j as int, t3) && (exists|i2: int|
                                    0 <= i2 < i + 1 && #[trigger] m.ms@[i2]
                                        == ckc_spec::model::apply(l, t3)) implies mm.contains(
                                ckc_spec::model::apply(cc.head, t3),
                            ) by {
                                if m.ms@[i as int] == ckc_spec::model::apply(l, t3) {
                                    assert(before[m.facts@[i as int] as int].term@
                                        == m.ms@[i as int]);
                                    assert(crate::mc_subst::extends(t3, th2@));
                                    assert(items_from(mm, cc, j as int + 1, t3));
                                }
                            }
                        }
                    } else {
                        proof {
                            assert forall|t3: Map<nat, Term>|
                                ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(
                                    t3,
                                    th,
                                ) && items_from(mm, cc, j as int, t3) && (exists|i2: int|
                                    0 <= i2 < i + 1 && #[trigger] m.ms@[i2]
                                        == ckc_spec::model::apply(l, t3)) implies mm.contains(
                                ckc_spec::model::apply(cc.head, t3),
                            ) by {
                                assert(before[m.facts@[i as int] as int].term@ == m.ms@[i as int]);
                            }
                        }
                    }
                    i += 1;
                }
                proof {
                    assert forall|t3: Map<nat, Term>|
                        ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                            && items_from(mm, cc, j as int, t3) implies mm.contains(
                        ckc_spec::model::apply(cc.head, t3),
                    ) by {
                        assert(ckc_spec::model::item_holds(mm, mm, cc.body[j as int], t3));
                        crate::mc_subst::builtin_apply_iff(l, t3);
                        crate::mc_model::iset_of_seq(m.ms@, ckc_spec::model::apply(l, t3));
                        let i2 = choose|i2: int|
                            0 <= i2 < m.ms@.len() && m.ms@[i2] == ckc_spec::model::apply(l, t3);
                    }
                }
                Ok(())
            }
        },
    }
}

// A positive item holding under a valuation is a goal that holds (ground, well-formed instance).
pub proof fn pos_goal_ok(c: DocClause, j: int, t3: Map<nat, Term>, mm: ISet<Term>)
    requires
        0 <= j < c.body.len(),
        c.body[j] is Pos,
        ckc_spec::term::wf_term(c.body[j]->Pos_0),
        ckc_spec::model::valuation(c, t3),
        ckc_spec::model::item_holds(mm, mm, c.body[j], t3),
    ensures
        crate::mc_model::goal_ok(mm, c.body[j]->Pos_0, t3),
{
    let l = c.body[j]->Pos_0;
    assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, l) implies t3.contains_key(x)
        && ckc_spec::term::ground(t3[x]) by {
        crate::mc_safe::vars_occurs(l, x);
        assert(ckc_spec::model::item_vars(c.body[j]).contains(x));
    }
    crate::mc_subst::apply_ground(l, t3);
    crate::mc_model::apply_wf(l, t3);
}

// Every variable of the head lies in the bound set at the end of the body.
pub proof fn binders_are_bound(c: DocClause, x: nat)
    requires
        ckc_spec::model::binders(c.body).contains(x),
    ensures
        crate::mc_lemma::binds_upto(c, c.body.len() as int).contains(x),
{
    crate::mc_safe::binders_any(c.body, x);
    let q = choose|q: int|
        0 <= q < c.body.len() && #[trigger] ckc_spec::model::item_binds(c.body[q]).contains(x);
    binds_upto_mono(c, q + 1, c.body.len() as int, x);
}

proof fn binds_upto_mono(c: DocClause, a: int, b: int, x: nat)
    requires
        0 < a <= b <= c.body.len(),
        ckc_spec::model::item_binds(c.body[a - 1]).contains(x),
    ensures
        crate::mc_lemma::binds_upto(c, b).contains(x),
    decreases b - a,
{
    if b > a {
        binds_upto_mono(c, a, b - 1, x);
    }
}

} // verus!
