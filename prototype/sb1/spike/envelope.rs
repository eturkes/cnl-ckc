use crate::*;

verus! {

// TA-T0. An envelope = any atom set closed under every rule whose built-ins
// hold, every NAF dropped (the positive program). Leastness is never used: a
// checked post-fixpoint (the concretization of a hinted feature model) is
// enough, so the solver's own fixpoint stays outside the trusted base.
pub open spec fn envelope(p: Program, l: Set<Atom>) -> bool {
    forall|i: int|
        #![trigger p.rules[i]]
        0 <= i < p.rules.len() && p.rules[i].builtins && p.rules[i].positive.subset_of(l)
            ==> l.contains(p.rules[i].head)
}

pub open spec fn inside(l: Set<Atom>) -> spec_fn(Set<Atom>) -> bool {
    |w: Set<Atom>| w.subset_of(l)
}

pub open spec fn live(r: Instance, l: Set<Atom>) -> bool {
    r.builtins && r.positive.subset_of(l)
}

// A live instance keeps its head + positive body; each NAF item keeps only its
// witnesses inside L. A dead instance keeps its head alone: no dependency,
// never retained. Indices stay aligned with the source program.
pub open spec fn prune_rule(r: Instance, l: Set<Atom>) -> Instance {
    if live(r, l) {
        Instance {
            head: r.head,
            positive: r.positive,
            naf: r.naf.map_values(|item: Seq<Set<Atom>>| item.filter(inside(l))),
            builtins: true,
        }
    } else {
        Instance { head: r.head, positive: Set::empty(), naf: Seq::empty(), builtins: false }
    }
}

pub open spec fn prune(p: Program, l: Set<Atom>) -> Program {
    Program { atoms: p.atoms, rules: p.rules.map_values(|r: Instance| prune_rule(r, l)) }
}

pub proof fn kept_witness(r: Instance, l: Set<Atom>, j: int)
    requires
        live(r, l),
        0 <= j < r.naf.len(),
    ensures
        prune_rule(r, l).naf.len() == r.naf.len(),
        prune_rule(r, l).naf[j] == r.naf[j].filter(inside(l)),
        forall|k: int|
            0 <= k < prune_rule(r, l).naf[j].len() ==> prune_rule(r, l).naf[j][k].subset_of(l)
                && r.naf[j].contains(prune_rule(r, l).naf[j][k]),
        forall|k: int|
            0 <= k < r.naf[j].len() && r.naf[j][k].subset_of(l) ==> prune_rule(
                r,
                l,
            ).naf[j].contains(r.naf[j][k]),
{
    let item = r.naf[j];
    let kept = item.filter(inside(l));
    assert forall|k: int| 0 <= k < kept.len() implies kept[k].subset_of(l) && item.contains(
        kept[k],
    ) by {
        item.lemma_filter_pred(inside(l), k);
        assert(kept.contains(kept[k]));
        item.lemma_filter_contains_rev(inside(l), kept[k]);
    }
    assert forall|k: int| 0 <= k < item.len() && item[k].subset_of(l) implies kept.contains(
        item[k],
    ) by {
        item.lemma_filter_contains(inside(l), k);
    }
}

// Inside L, a kept instance is retained exactly when its source is: a witness
// holding an atom outside L can never lie inside a reference set M within L.
pub proof fn prune_retained(r: Instance, l: Set<Atom>, m: Set<Atom>)
    requires
        m.subset_of(l),
    ensures
        prune_rule(r, l).head == r.head,
        retained(prune_rule(r, l), m) ==> live(r, l) && prune_rule(r, l).positive == r.positive
            && retained(r, m),
        retained(r, m) && r.positive.subset_of(l) ==> retained(prune_rule(r, l), m) && prune_rule(
            r,
            l,
        ).positive == r.positive,
{
    if live(r, l) {
        let q = prune_rule(r, l);
        if retained(q, m) {
            assert forall|j: int, k: int|
                0 <= j < r.naf.len() && 0 <= k < r.naf[j].len() implies !r.naf[j][k].subset_of(
                m,
            ) by {
                if r.naf[j][k].subset_of(m) {
                    kept_witness(r, l, j);
                    assert(q.naf[j].contains(r.naf[j][k]));
                    let k2 = choose|k2: int|
                        0 <= k2 < q.naf[j].len() && q.naf[j][k2] == r.naf[j][k];
                    assert(q.naf[j][k2].subset_of(m));
                }
            }
        }
        if retained(r, m) {
            assert forall|j: int, k: int|
                0 <= j < q.naf.len() && 0 <= k < q.naf[j].len() implies !q.naf[j][k].subset_of(
                m,
            ) by {
                kept_witness(r, l, j);
                let k0 = choose|k0: int| 0 <= k0 < r.naf[j].len() && r.naf[j][k0] == q.naf[j][k];
                assert(!r.naf[j][k0].subset_of(m));
            }
        }
    }
}

pub proof fn prune_wf(p: Program, l: Set<Atom>)
    requires
        wf(p),
    ensures
        wf(prune(p, l)),
{
    let q = prune(p, l);
    assert forall|i: int| #![trigger q.rules[i]] 0 <= i < q.rules.len() implies {
        let r = q.rules[i];
        &&& q.atoms.contains(r.head)
        &&& r.positive.subset_of(q.atoms)
        &&& forall|j: int, k: int|
            0 <= j < r.naf.len() && 0 <= k < r.naf[j].len() ==> r.naf[j][k].subset_of(q.atoms)
    } by {
        let r = p.rules[i];
        assert(q.rules[i] == prune_rule(r, l));
        if live(r, l) {
            assert forall|j: int, k: int|
                0 <= j < q.rules[i].naf.len() && 0 <= k
                    < q.rules[i].naf[j].len() implies q.rules[i].naf[j][k].subset_of(q.atoms) by {
                kept_witness(r, l, j);
                let k0 = choose|k0: int|
                    0 <= k0 < r.naf[j].len() && r.naf[j][k0] == q.rules[i].naf[j][k];
            }
        }
    }
}

pub proof fn envelope_bounds(p: Program, l: Set<Atom>, m: Set<Atom>)
    requires
        envelope(p, l),
    ensures
        lfp(p, m).subset_of(l),
{
    assert(closed(p, m, l));
    support::least(p, m, l);
}

pub proof fn prune_envelope(p: Program, l: Set<Atom>)
    requires
        envelope(p, l),
    ensures
        envelope(prune(p, l), l),
{
    let q = prune(p, l);
    assert forall|i: int|
        0 <= i < q.rules.len() && q.rules[i].builtins && q.rules[i].positive.subset_of(
            l,
        ) implies l.contains(q.rules[i].head) by {
        assert(q.rules[i] == prune_rule(p.rules[i], l));
    }
}

// TA-T0, reduct form: for every reference set M inside the envelope, the
// source program's reduct least model = the pruned program's, and lies in L.
pub proof fn pruned_gamma(p: Program, l: Set<Atom>, m: Set<Atom>)
    requires
        wf(p),
        envelope(p, l),
        m.subset_of(l),
    ensures
        lfp(p, m) == lfp(prune(p, l), m),
        lfp(p, m).subset_of(l),
{
    let q = prune(p, l);
    prune_wf(p, l);
    prune_envelope(p, l);
    envelope_bounds(p, l, m);
    envelope_bounds(q, l, m);
    let x = lfp(q, m);
    let y = lfp(p, m);
    support::lfp_closed(q, m);
    support::lfp_closed(p, m);
    assert forall|i: int|
        0 <= i < p.rules.len() && retained(p.rules[i], m) && p.rules[i].positive.subset_of(
            x,
        ) implies x.contains(p.rules[i].head) by {
        prune_retained(p.rules[i], l, m);
        assert(q.rules[i] == prune_rule(p.rules[i], l));
    }
    support::least(p, m, x);
    assert forall|i: int|
        0 <= i < q.rules.len() && retained(q.rules[i], m) && q.rules[i].positive.subset_of(
            y,
        ) implies y.contains(q.rules[i].head) by {
        prune_retained(p.rules[i], l, m);
        assert(q.rules[i] == prune_rule(p.rules[i], l));
    }
    support::least(q, m, y);
    assert(x =~= y);
}

pub proof fn pruned_iterate(p: Program, l: Set<Atom>, k: nat)
    requires
        wf(p),
        envelope(p, l),
    ensures
        afp::iterate(p, k) == afp::iterate(prune(p, l), k),
        afp::iterate(p, k).subset_of(l),
    decreases k,
{
    if k > 0 {
        pruned_iterate(p, l, (k - 1) as nat);
        let previous = afp::iterate(p, (k - 1) as nat);
        pruned_gamma(p, l, previous);
        pruned_gamma(p, l, lfp(p, previous));
    }
}

// TA-T0: the source program and its envelope-pruned program have the same
// alternating-fixpoint WFM (true + undefined sets) and the same stable models.
pub proof fn pruned_wfm(p: Program, l: Set<Atom>)
    requires
        wf(p),
        envelope(p, l),
    ensures
        afp::lower(p) == afp::lower(prune(p, l)),
        afp::upper(p) == afp::upper(prune(p, l)),
        forall|m: Set<Atom>| stable(p, m) == stable(prune(p, l), m),
{
    let q = prune(p, l);
    prune_wf(p, l);
    prune_envelope(p, l);
    afp::finite_alternation(p);
    afp::finite_alternation(q);
    pruned_iterate(p, l, p.atoms.len());
    pruned_gamma(p, l, afp::lower(p));
    assert forall|m: Set<Atom>| stable(p, m) == stable(q, m) by {
        if stable(p, m) {
            envelope_bounds(p, l, m);
            pruned_gamma(p, l, m);
        }
        if stable(q, m) {
            envelope_bounds(q, l, m);
            pruned_gamma(p, l, m);
        }
    }
}

// TA-T0 composed with (b) + (e): signed coverage of the PRUNED program by a
// graph without negative cycles => the SOURCE program's WFM is two-valued and
// is its unique stable model. Dead instances + outside witnesses need no edge.
pub proof fn pruned_covered_wfm(p: Program, l: Set<Atom>, g: rank::Graph, origin: Seq<int>)
    requires
        wf(p),
        envelope(p, l),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        origins::clause_coverage(prune(p, l), g, origin),
    ensures
        afp::lower(p) == afp::upper(p),
        stable(p, afp::lower(p)),
        forall|m: Set<Atom>| stable(p, m) ==> m == afp::lower(p),
{
    prune_wf(p, l);
    afp::covered_wfm(prune(p, l), g, origin);
    pruned_wfm(p, l);
}

// The checker's end claim: a support + closure certificate over the SOURCE
// program + the pruned program's covering graph => the model = WFM(source).
pub proof fn pruned_certified_is_wfm(
    p: Program,
    l: Set<Atom>,
    g: rank::Graph,
    origin: Seq<int>,
    m: Set<Atom>,
    level: spec_fn(Atom) -> nat,
)
    requires
        wf(p),
        envelope(p, l),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        origins::clause_coverage(prune(p, l), g, origin),
        m.subset_of(p.atoms),
        founded(p, m, level),
        closed(p, m, m),
    ensures
        m == afp::lower(p),
        m == afp::upper(p),
{
    support::support_closure_lfp(p, m, level);
    pruned_covered_wfm(p, l, g, origin);
}

} // verus!
verus! {

pub proof fn filter_index_map<A, B>(s: Seq<A>, f: spec_fn(A) -> B, pred: spec_fn(int) -> bool)
    ensures
        s.map_values(f).filter_index(pred) == s.filter_index(pred).map_values(f),
    decreases s.len(),
{
    reveal(Seq::filter_index);
    if s.len() == 0 {
        assert(s.map_values(f) =~= Seq::<B>::empty());
        assert(s.filter_index(pred).map_values(f) =~= Seq::<B>::empty());
    } else {
        let t = s.drop_last();
        filter_index_map(t, f, pred);
        assert(s.map_values(f).drop_last() =~= t.map_values(f));
        assert(s.map_values(f).last() == f(s.last()));
        if pred(s.len() - 1) {
            assert(t.filter_index(pred).push(s.last()).map_values(f) =~= t.filter_index(
                pred,
            ).map_values(f).push(f(s.last())));
        }
    }
}

// Pruning commutes with clause selection: both keep source indices aligned.
pub proof fn select_prune(p: Program, l: Set<Atom>, origin: Seq<int>, chosen: Set<int>)
    ensures
        selection::select(prune(p, l), origin, chosen) == prune(
            selection::select(p, origin, chosen),
            l,
        ),
{
    let pred = |i: int| chosen.contains(origin[i]);
    filter_index_map(p.rules, |r: Instance| prune_rule(r, l), pred);
}

pub proof fn select_envelope(p: Program, l: Set<Atom>, origin: Seq<int>, chosen: Set<int>)
    requires
        envelope(p, l),
        origin.len() == p.rules.len(),
    ensures
        envelope(selection::select(p, origin, chosen), l),
{
    let q = selection::select(p, origin, chosen);
    assert forall|j: int|
        0 <= j < q.rules.len() && q.rules[j].builtins && q.rules[j].positive.subset_of(
            l,
        ) implies l.contains(q.rules[j].head) by {
        let i = selection::selected_source_index(p, origin, chosen, j);
    }
}

// (c) with TA-T0: query roots inside the atom slice of the PRUNED program's
// covered graph + a forward-closed choice => the selected SOURCE clauses give
// the source program's q-all + goal answers, both WFMs two-valued.
pub proof fn pruned_selected_wfm_queries(
    p: Program,
    l: Set<Atom>,
    g: rank::Graph,
    origin: Seq<int>,
    chosen: Set<int>,
    operators: Set<Atom>,
    valuations: Set<int>,
    goals: spec_fn(int) -> Instance,
)
    requires
        wf(p),
        envelope(p, l),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        origins::clause_coverage(prune(p, l), g, origin),
        rank::forward_closed(g, chosen),
        operators.subset_of(origins::atom_slice(prune(p, l), origin, chosen)),
        forall|id: int|
            valuations.contains(id) ==> slice::reads(
                goals(id),
                origins::atom_slice(prune(p, l), origin, chosen),
            ),
    ensures
        afp::lower(p) == afp::upper(p),
        afp::lower(selection::select(p, origin, chosen)) == afp::upper(
            selection::select(p, origin, chosen),
        ),
        slice::enumerate(afp::lower(p), operators) == slice::enumerate(
            afp::lower(selection::select(p, origin, chosen)),
            operators,
        ),
        slice::goal_answers(afp::lower(p), valuations, goals) == slice::goal_answers(
            afp::lower(selection::select(p, origin, chosen)),
            valuations,
            goals,
        ),
{
    let q = prune(p, l);
    let s = selection::select(p, origin, chosen);
    prune_wf(p, l);
    selection::selected_wfm_queries(q, g, origin, chosen, operators, valuations, goals);
    pruned_wfm(p, l);
    selection::selection_wf(p, origin, chosen);
    select_envelope(p, l, origin, chosen);
    pruned_wfm(s, l);
    select_prune(p, l, origin, chosen);
}

} // verus!
verus! {

// P' = P with NAF items added alone (the DC shape appends
// `\+ guideline_exclusion(Key)`): same atoms, heads, positive bodies and
// built-ins; every NAF item of P kept.
pub open spec fn naf_extends(p: Program, q: Program) -> bool {
    &&& p.atoms == q.atoms
    &&& p.rules.len() == q.rules.len()
    &&& forall|i: int|
        #![trigger q.rules[i]]
        0 <= i < p.rules.len() ==> {
            &&& q.rules[i].head == p.rules[i].head
            &&& q.rules[i].positive == p.rules[i].positive
            &&& q.rules[i].builtins == p.rules[i].builtins
            &&& forall|j: int|
                0 <= j < p.rules[i].naf.len() ==> q.rules[i].naf.contains(
                    #[trigger] p.rules[i].naf[j],
                )
        }
}

pub proof fn naf_extension_envelope(p: Program, q: Program, l: Set<Atom>)
    requires
        naf_extends(p, q),
        envelope(q, l),
    ensures
        envelope(p, l),
{
    assert forall|i: int|
        0 <= i < p.rules.len() && p.rules[i].builtins && p.rules[i].positive.subset_of(
            l,
        ) implies l.contains(p.rules[i].head) by {
        assert(q.rules[i].head == p.rules[i].head);
    }
}

// A graph covering the pruned extension covers the pruned original: every
// kept witness of P is a kept witness of P'.
pub proof fn naf_extension_coverage(
    p: Program,
    q: Program,
    l: Set<Atom>,
    g: rank::Graph,
    origin: Seq<int>,
)
    requires
        naf_extends(p, q),
        origins::clause_coverage(prune(q, l), g, origin),
    ensures
        origins::clause_coverage(prune(p, l), g, origin),
{
    let pp = prune(p, l);
    let pq = prune(q, l);
    assert forall|i: int, j: int|
        0 <= i < pp.rules.len() && 0 <= j < pp.rules.len() && pp.rules[i].positive.contains(
            pp.rules[j].head,
        ) implies g.positive.contains((origin[i], origin[j])) by {
        assert(q.rules[i].head == p.rules[i].head);
        assert(q.rules[j].head == p.rules[j].head);
        assert(pq.rules[i].positive == pp.rules[i].positive);
        assert(pq.rules[j].head == pp.rules[j].head);
    }
    assert forall|i: int, j: int, n: int, w: int|
        0 <= i < pp.rules.len() && 0 <= j < pp.rules.len() && 0 <= n < pp.rules[i].naf.len() && 0
            <= w < pp.rules[i].naf[n].len() && pp.rules[i].naf[n][w].contains(
            pp.rules[j].head,
        ) implies g.negative.contains((origin[i], origin[j])) by {
        let r = p.rules[i];
        let s = q.rules[i];
        assert(q.rules[j].head == p.rules[j].head);
        assert(s.head == r.head);
        assert(live(r, l));
        assert(live(s, l));
        kept_witness(r, l, n);
        let wit = pp.rules[i].naf[n][w];
        assert(r.naf[n].contains(wit));
        assert(s.naf.contains(r.naf[n]));
        let n2 = choose|n2: int| 0 <= n2 < s.naf.len() && s.naf[n2] == r.naf[n];
        kept_witness(s, l, n2);
        let k0 = choose|k0: int| 0 <= k0 < r.naf[n].len() && r.naf[n][k0] == wit;
        assert(s.naf[n2][k0] == wit);
        assert(pq.rules[i].naf[n2].contains(wit));
        let w2 = choose|w2: int|
            0 <= w2 < pq.rules[i].naf[n2].len() && pq.rules[i].naf[n2][w2] == wit;
        assert(pq.rules[j].head == pp.rules[j].head);
    }
}

pub proof fn naf_extension_slice(
    p: Program,
    q: Program,
    l: Set<Atom>,
    origin: Seq<int>,
    chosen: Set<int>,
)
    requires
        naf_extends(p, q),
    ensures
        origins::atom_slice(prune(p, l), origin, chosen) == origins::atom_slice(
            prune(q, l),
            origin,
            chosen,
        ),
{
    assert forall|i: int| 0 <= i < p.rules.len() implies prune(q, l).rules[i].head == prune(
        p,
        l,
    ).rules[i].head by {
        assert(q.rules[i].head == p.rules[i].head);
    }
    assert(origins::atom_slice(prune(p, l), origin, chosen) =~= origins::atom_slice(
        prune(q, l),
        origin,
        chosen,
    ));
}

// TAS's composition: TA admission of the DC-shaped program (the census
// baseline) certifies the original clauses the producer + model check run,
// and its forward-closed slices answer as the original program.
pub proof fn naf_extension_wfm(
    p: Program,
    q: Program,
    l: Set<Atom>,
    g: rank::Graph,
    origin: Seq<int>,
)
    requires
        wf(p),
        naf_extends(p, q),
        envelope(q, l),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        origins::clause_coverage(prune(q, l), g, origin),
    ensures
        afp::lower(p) == afp::upper(p),
        stable(p, afp::lower(p)),
        forall|m: Set<Atom>| stable(p, m) ==> m == afp::lower(p),
        envelope(p, l),
        origins::clause_coverage(prune(p, l), g, origin),
{
    naf_extension_envelope(p, q, l);
    naf_extension_coverage(p, q, l, g, origin);
    pruned_covered_wfm(p, l, g, origin);
}

} // verus!
