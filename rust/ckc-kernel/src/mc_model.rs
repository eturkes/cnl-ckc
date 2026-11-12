use crate::k2_term::{ENode, ETermArena};
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

// Contract mc D6: the proposed model M = the decoded model lines (ground terms).
pub struct EMcModel {
    pub facts: Vec<usize>,
    pub ms: Ghost<Seq<Term>>,
}

pub open spec fn model_ok(nodes: Seq<ENode>, m: &EMcModel) -> bool {
    &&& m.facts@.len() == m.ms@.len()
    &&& forall|i: int|
        0 <= i < m.facts@.len() ==> {
            &&& #[trigger] m.facts@[i] < nodes.len()
            &&& nodes[m.facts@[i] as int].term@ == m.ms@[i]
            &&& ckc_spec::term::ground(m.ms@[i])
        }
}

pub proof fn model_ok_prefix(before: Seq<ENode>, after: Seq<ENode>, m: &EMcModel)
    requires
        before.is_prefix_of(after),
        model_ok(before, m),
    ensures
        model_ok(after, m),
{
    assert forall|i: int| 0 <= i < m.facts@.len() implies {
        &&& #[trigger] m.facts@[i] < after.len()
        &&& after[m.facts@[i] as int].term@ == m.ms@[i]
        &&& ckc_spec::term::ground(m.ms@[i])
    } by {
        assert(after[m.facts@[i] as int] == before[m.facts@[i] as int]);
    }
}

pub proof fn iset_of_seq(s: Seq<Term>, t: Term)
    ensures
        s.to_iset().contains(t) == s.contains(t),
{
    s.to_set_ensures();
}

// arena@[root] ∈ M.
pub fn in_model(arena: &ETermArena, m: &EMcModel, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
        ckc_spec::term::ground(arena@[root as int]),
        model_ok(arena.nodes@, m),
    ensures
        out == m.ms@.to_iset().contains(arena@[root as int]),
{
    proof {
        iset_of_seq(m.ms@, arena@[root as int]);
    }
    let mut i = 0usize;
    while i < m.facts.len()
        invariant
            crate::k2_term::root_ok(arena, root),
            ckc_spec::term::ground(arena@[root as int]),
            model_ok(arena.nodes@, m),
            m.ms@.to_iset().contains(arena@[root as int]) == m.ms@.contains(arena@[root as int]),
            i <= m.facts@.len(),
            forall|q: int| 0 <= q < i ==> #[trigger] m.ms@[q] != arena@[root as int],
        decreases m.facts.len() - i,
    {
        proof {
            assert(m.facts@[i as int] < arena.nodes@.len());
        }
        if crate::k2_sort::term_equal(arena, root, m.facts[i]) {
            proof {
                assert(m.ms@[i as int] == arena@[root as int]);
            }
            return true;
        }
        i += 1;
    }
    false
}

// The ghost model's fact i matches arena@[pat] under sub, extending th.
pub open spec fn some_fact_matches(
    ms: Seq<Term>,
    pat: Term,
    th: Map<nat, Term>,
    t3: Map<nat, Term>,
) -> bool {
    &&& crate::mc_subst::extends(t3, th)
    &&& ms.contains(ckc_spec::model::apply(pat, t3))
}

// apply(apply(t, th), s) = apply(t, th ⊕ s), th's values ground (th wins).
pub proof fn apply_compose(t: Term, th: Map<nat, Term>, s: Map<nat, Term>)
    requires
        forall|k: nat| #[trigger] th.contains_key(k) ==> ckc_spec::term::ground(th[k]),
    ensures
        ckc_spec::model::apply(ckc_spec::model::apply(t, th), s) == ckc_spec::model::apply(
            t,
            s.union_prefer_right(th),
        ),
    decreases t,
{
    match t {
        Term::Var(k) => {
            if th.contains_key(k) {
                crate::mc_subst::apply_of_ground(th[k], s);
            }
        },
        Term::Comp(f, args) => {
            let u = s.union_prefer_right(th);
            crate::mc_subst::apply_all_index(args, th);
            crate::mc_subst::apply_all_index(ckc_spec::model::apply_all(args, th), s);
            crate::mc_subst::apply_all_index(args, u);
            assert forall|q: int| 0 <= q < args.len() implies ckc_spec::model::apply(
                ckc_spec::model::apply_all(args, th)[q],
                s,
            ) == ckc_spec::model::apply_all(args, u)[q] by {
                apply_compose(args[q], th, s);
            }
            assert(ckc_spec::model::apply_all(ckc_spec::model::apply_all(args, th), s)
                =~= ckc_spec::model::apply_all(args, u));
        },
        _ => {},
    }
}

// One goal holds in M under t3: a ground, well-formed instance that holds.
pub open spec fn goal_ok(m: ISet<Term>, g: Term, t3: Map<nat, Term>) -> bool {
    &&& ckc_spec::term::ground(ckc_spec::model::apply(g, t3))
    &&& ckc_spec::term::wf_term(ckc_spec::model::apply(g, t3))
    &&& ckc_spec::model::holds(m, ckc_spec::model::apply(g, t3))
}

// The goals of gs from index k hold in M under t3.
pub open spec fn goals_sat(m: ISet<Term>, gs: Seq<Term>, k: int, t3: Map<nat, Term>) -> bool {
    forall|q: int| k <= q < gs.len() ==> #[trigger] goal_ok(m, gs[q], t3)
}

pub proof fn wf_terms_from(ts: Seq<Term>)
    requires
        forall|q: int| 0 <= q < ts.len() ==> #[trigger] ckc_spec::term::wf_term(ts[q]),
    ensures
        ckc_spec::term::wf_terms(ts),
    decreases ts.len(),
{
    if ts.len() > 0 {
        let rest = ts.drop_first();
        assert forall|q: int| 0 <= q < rest.len() implies #[trigger] ckc_spec::term::wf_term(
            rest[q],
        ) by {
            assert(rest[q] == ts[q + 1]);
        }
        wf_terms_from(rest);
        assert(ckc_spec::term::wf_term(ts[0]));
    }
}

pub proof fn wf_terms_at(ts: Seq<Term>, q: int)
    requires
        ckc_spec::term::wf_terms(ts),
        0 <= q < ts.len(),
    ensures
        ckc_spec::term::wf_term(ts[q]),
    decreases ts.len(),
{
    if q > 0 {
        assert(ts.drop_first()[q - 1] == ts[q]);
        wf_terms_at(ts.drop_first(), q - 1);
    }
}

// conj_holds over apply_all(gs, t3) = goals_sat from 0.
pub proof fn conj_is_goals(m: ISet<Term>, gs: Seq<Term>, t3: Map<nat, Term>)
    ensures
        ckc_spec::model::conj_holds(m, ckc_spec::model::apply_all(gs, t3)) == goals_sat(
            m,
            gs,
            0,
            t3,
        ),
{
    let w = ckc_spec::model::apply_all(gs, t3);
    crate::mc_subst::apply_all_index(gs, t3);
    if ckc_spec::model::conj_holds(m, w) {
        assert forall|q: int| 0 <= q < gs.len() implies #[trigger] goal_ok(m, gs[q], t3) by {
            assert(w[q] == ckc_spec::model::apply(gs[q], t3));
            crate::mc_patient::ground_all_at(w, q);
            wf_terms_at(w, q);
            assert(ckc_spec::model::holds(m, w[q]));
        }
    }
    if goals_sat(m, gs, 0, t3) {
        assert forall|q: int| 0 <= q < w.len() implies #[trigger] ckc_spec::term::ground(w[q]) by {
            assert(w[q] == ckc_spec::model::apply(gs[q], t3));
            assert(goal_ok(m, gs[q], t3));
        }
        crate::mc_subst::ground_all_from(w);
        assert forall|q: int| 0 <= q < w.len() implies #[trigger] ckc_spec::term::wf_term(w[q]) by {
            assert(w[q] == ckc_spec::model::apply(gs[q], t3));
            assert(goal_ok(m, gs[q], t3));
        }
        wf_terms_from(w);
        assert forall|q: int| 0 <= q < w.len() implies #[trigger] ckc_spec::model::holds(
            m,
            w[q],
        ) by {
            assert(w[q] == ckc_spec::model::apply(gs[q], t3));
            assert(goal_ok(m, gs[q], t3));
        }
    }
}

// Instances over well-formed values stay well-formed.
pub proof fn apply_wf(t: Term, th: Map<nat, Term>)
    requires
        ckc_spec::term::wf_term(t),
        forall|k: nat| #[trigger] th.contains_key(k) ==> ckc_spec::term::wf_term(th[k]),
    ensures
        ckc_spec::term::wf_term(ckc_spec::model::apply(t, th)),
    decreases t,
{
    if let Term::Comp(f, args) = t {
        crate::mc_subst::apply_all_index(args, th);
        assert forall|q: int| 0 <= q < args.len() implies #[trigger] ckc_spec::term::wf_term(
            ckc_spec::model::apply_all(args, th)[q],
        ) by {
            wf_terms_at(args, q);
            apply_wf(args[q], th);
        }
        wf_terms_from(ckc_spec::model::apply_all(args, th));
    }
}

// A substitution's values are arena terms, hence well-formed.
pub proof fn sub_vals_wf(arena: &ETermArena, sub: Seq<Option<usize>>, th: Map<nat, Term>)
    requires
        crate::k2_term::arena_ok(arena),
        crate::mc_subst::sub_ok(arena.nodes@, sub, th),
    ensures
        forall|k: nat| #[trigger] th.contains_key(k) ==> ckc_spec::term::wf_term(th[k]),
{
    assert forall|k: nat| #[trigger] th.contains_key(k) implies ckc_spec::term::wf_term(th[k]) by {
        assert(crate::k2_term::node_ok(arena.nodes@, sub[k as int]->Some_0 as int));
    }
}

// Decodes the model lines (Err(i) = the first line that does not decode).
pub fn decode_model(arena: &mut ETermArena, model: &Vec<Vec<u8>>) -> (out: Result<EMcModel, usize>)
    requires
        crate::k2_term::arena_ok(old(arena)),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(m) ==> model_ok(final(arena).nodes@, &m) && ckc_spec::model::terms_decode(
            ckc_spec::model::bytes_seq(model@),
            m.ms@,
        ),
{
    let ghost origin = arena.nodes@;
    proof {
        assert(origin.is_prefix_of(origin)) by {
            assert(origin.take(origin.len() as int) =~= origin);
        }
    }
    let mut facts: Vec<usize> = Vec::new();
    let ghost mut ms: Seq<Term> = Seq::empty();
    let mut i = 0usize;
    while i < model.len()
        invariant
            crate::k2_term::arena_ok(arena),
            origin == old(arena).nodes@,
            origin.is_prefix_of(arena.nodes@),
            i <= model@.len(),
            facts@.len() == i,
            ms.len() == i,
            forall|q: int|
                0 <= q < i ==> {
                    &&& #[trigger] facts@[q] < arena.nodes@.len()
                    &&& arena.nodes@[facts@[q] as int].term@ == ms[q]
                    &&& ckc_spec::term::ground(ms[q])
                },
            forall|q: int|
                0 <= q < i ==> ckc_spec::model::term_decodes(#[trigger] model@[q]@, ms[q]),
        decreases model.len() - i,
    {
        let ghost before = arena.nodes@;
        let r = crate::mc_term::decode_term(arena, &model[i]);
        proof {
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
        }
        let r = match r {
            Some(r) => r,
            None => return Err(i),
        };
        proof {
            assert forall|q: int| 0 <= q < i implies arena.nodes@[facts@[q] as int]
                == before[facts@[q] as int] by {}
            ms = ms.push(arena@[r as int]);
        }
        facts.push(r);
        i += 1;
    }
    proof {
        assert forall|q: int| 0 <= q < model@.len() implies ckc_spec::model::term_decodes(
            #[trigger] ckc_spec::model::bytes_seq(model@)[q],
            ms[q],
        ) by {
            assert(ckc_spec::model::bytes_seq(model@)[q] == model@[q]@);
        }
    }
    Ok(EMcModel { facts, ms: Ghost(ms) })
}

} // verus!
