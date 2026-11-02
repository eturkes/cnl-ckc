use crate::*;

verus! {

// An actual subsequence of the selected clauses, not the atom-induced mask.
// Filtering source indices preserves duplicate clauses + their source order.
pub open spec fn select(p: Program, origin: Seq<int>, chosen: Set<int>) -> Program {
    Program { atoms: p.atoms, rules: p.rules.filter_index(|i: int| chosen.contains(origin[i])) }
}

pub proof fn selected_source_index(p: Program, origin: Seq<int>, chosen: Set<int>, j: int) -> (i:
    int)
    requires
        origin.len() == p.rules.len(),
        0 <= j < select(p, origin, chosen).rules.len(),
    ensures
        0 <= i < p.rules.len(),
        chosen.contains(origin[i]),
        select(p, origin, chosen).rules[j] == p.rules[i],
{
    let pred = |i: int| chosen.contains(origin[i]);
    p.rules.lemma_filter_index(pred);
    assert(p.rules.filter_index(pred)[j] == select(p, origin, chosen).rules[j]);
    let i = choose|i: int|
        0 <= i < p.rules.len() && p.rules.filter_index(pred)[j] == p.rules[i] && pred(i);
    i
}

pub proof fn selection_exact(p: Program, origin: Seq<int>, chosen: Set<int>)
    requires
        origin.len() == p.rules.len(),
    ensures
        select(p, origin, chosen).atoms == p.atoms,
        select(p, origin, chosen).rules.len() <= p.rules.len(),
        forall|i: int|
            0 <= i < p.rules.len() && chosen.contains(origin[i]) ==> select(
                p,
                origin,
                chosen,
            ).rules.contains(p.rules[i]),
{
    let pred = |i: int| chosen.contains(origin[i]);
    p.rules.lemma_filter_index(pred);
}

pub proof fn selection_wf(p: Program, origin: Seq<int>, chosen: Set<int>)
    requires
        wf(p),
        origin.len() == p.rules.len(),
    ensures
        wf(select(p, origin, chosen)),
{
    selection_exact(p, origin, chosen);
    let q = select(p, origin, chosen);
    assert forall|j: int| #![trigger q.rules[j]] 0 <= j < q.rules.len() implies {
        let rule = q.rules[j];
        &&& q.atoms.contains(rule.head)
        &&& rule.positive.subset_of(q.atoms)
        &&& forall|n: int, w: int|
            0 <= n < rule.naf.len() && 0 <= w < rule.naf[n].len() ==> rule.naf[n][w].subset_of(
                q.atoms,
            )
    } by {
        let i = selected_source_index(p, origin, chosen, j);
    }
}

pub proof fn selection_stratifies(
    p: Program,
    origin: Seq<int>,
    chosen: Set<int>,
    r: spec_fn(Atom) -> nat,
)
    requires
        origin.len() == p.rules.len(),
        rank::local_stratification(p, r),
    ensures
        rank::local_stratification(select(p, origin, chosen), r),
{
    selection_exact(p, origin, chosen);
    let q = select(p, origin, chosen);
    assert forall|j: int| #![trigger q.rules[j]] 0 <= j < q.rules.len() implies {
        let rule = q.rules[j];
        &&& forall|b: Atom| rule.positive.contains(b) ==> r(b) <= r(rule.head)
        &&& forall|n: int, w: int, b: Atom|
            0 <= n < rule.naf.len() && 0 <= w < rule.naf[n].len() && rule.naf[n][w].contains(b)
                ==> r(b) < r(rule.head)
    } by {
        let i = selected_source_index(p, origin, chosen, j);
    }
}

pub proof fn selection_closed(p: Program, origin: Seq<int>, chosen: Set<int>, s: Set<Atom>)
    requires
        origin.len() == p.rules.len(),
        slice::predecessor_closed(p, s),
    ensures
        slice::predecessor_closed(select(p, origin, chosen), s),
{
    selection_exact(p, origin, chosen);
    let q = select(p, origin, chosen);
    assert forall|j: int|
        0 <= j < q.rules.len() && s.contains(q.rules[j].head) implies slice::reads(
        q.rules[j],
        s,
    ) by {
        let i = selected_source_index(p, origin, chosen, j);
    }
}

pub open spec fn same_rules_on(p: Program, q: Program, s: Set<Atom>) -> bool {
    &&& p.atoms == q.atoms
    &&& forall|i: int|
        0 <= i < p.rules.len() && s.contains(p.rules[i].head) ==> q.rules.contains(p.rules[i])
    &&& forall|j: int|
        0 <= j < q.rules.len() && s.contains(q.rules[j].head) ==> p.rules.contains(q.rules[j])
}

pub proof fn mask_closed_equivalence(
    p: Program,
    q: Program,
    s: Set<Atom>,
    m: Set<Atom>,
    x: Set<Atom>,
)
    requires
        same_rules_on(p, q, s),
        closed(slice::mask(p, s), m, x),
    ensures
        closed(slice::mask(q, s), m, x),
{
    let a = slice::mask(p, s);
    let b = slice::mask(q, s);
    assert forall|j: int|
        0 <= j < b.rules.len() && retained(b.rules[j], m) && b.rules[j].positive.subset_of(
            x,
        ) implies x.contains(b.rules[j].head) by {
        assert(s.contains(q.rules[j].head));
        let i = choose|i: int| 0 <= i < p.rules.len() && p.rules[i] == q.rules[j];
        assert(a.rules[i] == slice::mask_rule(q.rules[j], s));
    }
}

pub proof fn masks_same_reduct(p: Program, q: Program, s: Set<Atom>, m: Set<Atom>)
    requires
        same_rules_on(p, q, s),
    ensures
        lfp(slice::mask(p, s), m) == lfp(slice::mask(q, s), m),
{
    assert(same_rules_on(q, p, s));
    assert forall|x: Set<Atom>|
        closed(slice::mask(p, s), m, x) == closed(slice::mask(q, s), m, x) by {
        if closed(slice::mask(p, s), m, x) {
            mask_closed_equivalence(p, q, s, m, x);
        }
        if closed(slice::mask(q, s), m, x) {
            mask_closed_equivalence(q, p, s, m, x);
        }
    }
    assert(lfp(slice::mask(p, s), m) =~= lfp(slice::mask(q, s), m));
}

pub proof fn selection_same_on_slice(p: Program, origin: Seq<int>, chosen: Set<int>)
    requires
        origin.len() == p.rules.len(),
    ensures
        same_rules_on(p, select(p, origin, chosen), origins::atom_slice(p, origin, chosen)),
{
    selection_exact(p, origin, chosen);
    let q = select(p, origin, chosen);
    let s = origins::atom_slice(p, origin, chosen);
    assert forall|i: int|
        0 <= i < p.rules.len() && s.contains(p.rules[i].head) implies q.rules.contains(
        p.rules[i],
    ) by {
        assert(chosen.contains(origin[i]));
    }
    assert forall|j: int|
        0 <= j < q.rules.len() && s.contains(q.rules[j].head) implies p.rules.contains(
        q.rules[j],
    ) by {
        let i = selected_source_index(p, origin, chosen, j);
    }
}

// (f) Forward-closed abstract clause selection + all-producer coverage => the
// induced atom slice meets (c)'s premise. The actual selected-clause program
// (not merely its mask) has the same reduct lfp on the slice as (c)'s mask,
// for every reduct reference M. Extra selected heads outside the slice may
// still derive; no equality of the entire two programs' models is asserted.
pub proof fn clause_selection_reduct(
    p: Program,
    g: rank::Graph,
    origin: Seq<int>,
    chosen: Set<int>,
    m: Set<Atom>,
)
    requires
        wf(p),
        origins::clause_coverage(p, g, origin),
        rank::forward_closed(g, chosen),
    ensures
        wf(select(p, origin, chosen)),
        slice::predecessor_closed(p, origins::atom_slice(p, origin, chosen)),
        slice::predecessor_closed(
            select(p, origin, chosen),
            origins::atom_slice(p, origin, chosen),
        ),
        lfp(select(p, origin, chosen), m).intersect(origins::atom_slice(p, origin, chosen)) == lfp(
            slice::mask(p, origins::atom_slice(p, origin, chosen)),
            m.intersect(origins::atom_slice(p, origin, chosen)),
        ),
        lfp(select(p, origin, chosen), m).intersect(origins::atom_slice(p, origin, chosen)) == lfp(
            p,
            m,
        ).intersect(origins::atom_slice(p, origin, chosen)),
{
    let s = origins::atom_slice(p, origin, chosen);
    let q = select(p, origin, chosen);
    origins::clause_coverage_slice(p, g, origin, chosen);
    selection_wf(p, origin, chosen);
    selection_closed(p, origin, chosen, s);
    selection_same_on_slice(p, origin, chosen);
    masks_same_reduct(p, q, s, m.intersect(s));
    slice::reduct_projection(q, m, s);
    slice::reduct_projection(p, m, s);
}

pub proof fn queries_agree(
    m: Set<Atom>,
    n: Set<Atom>,
    s: Set<Atom>,
    operators: Set<Atom>,
    valuations: Set<int>,
    goals: spec_fn(int) -> Instance,
)
    requires
        slice::agrees(m, n, s),
        operators.subset_of(s),
        forall|id: int| valuations.contains(id) ==> slice::reads(goals(id), s),
    ensures
        slice::enumerate(m, operators) == slice::enumerate(n, operators),
        slice::goal_answers(m, valuations, goals) == slice::goal_answers(n, valuations, goals),
{
    assert(slice::enumerate(m, operators) =~= slice::enumerate(n, operators));
    assert forall|id: int| valuations.contains(id) implies slice::goal_holds(goals(id), m)
        == slice::goal_holds(goals(id), n) by {
        slice::retained_agrees(goals(id), m, n, s);
        slice::subset_agrees(goals(id).positive, m, n, s);
    }
    assert(slice::goal_answers(m, valuations, goals) =~= slice::goal_answers(n, valuations, goals));
}

// (f), both query shapes: selected clauses' reduct model vs atom-induced mask.
// Query candidates read only the induced slice, including correlated NAF.
pub proof fn selected_reduct_queries(
    p: Program,
    g: rank::Graph,
    origin: Seq<int>,
    chosen: Set<int>,
    m: Set<Atom>,
    operators: Set<Atom>,
    valuations: Set<int>,
    goals: spec_fn(int) -> Instance,
)
    requires
        wf(p),
        origins::clause_coverage(p, g, origin),
        rank::forward_closed(g, chosen),
        operators.subset_of(origins::atom_slice(p, origin, chosen)),
        forall|id: int|
            valuations.contains(id) ==> slice::reads(
                goals(id),
                origins::atom_slice(p, origin, chosen),
            ),
    ensures
        slice::enumerate(lfp(select(p, origin, chosen), m), operators) == slice::enumerate(
            lfp(
                slice::mask(p, origins::atom_slice(p, origin, chosen)),
                m.intersect(origins::atom_slice(p, origin, chosen)),
            ),
            operators,
        ),
        slice::goal_answers(lfp(select(p, origin, chosen), m), valuations, goals)
            == slice::goal_answers(
            lfp(
                slice::mask(p, origins::atom_slice(p, origin, chosen)),
                m.intersect(origins::atom_slice(p, origin, chosen)),
            ),
            valuations,
            goals,
        ),
{
    clause_selection_reduct(p, g, origin, chosen, m);
    let s = origins::atom_slice(p, origin, chosen);
    let a = lfp(select(p, origin, chosen), m);
    let b = lfp(slice::mask(p, s), m.intersect(s));
    assert(slice::agrees(a, b, s));
    queries_agree(a, b, s, operators, valuations, goals);
}

pub proof fn selected_stable_agrees(
    p: Program,
    g: rank::Graph,
    origin: Seq<int>,
    chosen: Set<int>,
    r: spec_fn(Atom) -> nat,
    m: Set<Atom>,
    n: Set<Atom>,
)
    requires
        wf(p),
        origins::clause_coverage(p, g, origin),
        rank::forward_closed(g, chosen),
        rank::local_stratification(p, r),
        stable(p, m),
        stable(select(p, origin, chosen), n),
    ensures
        slice::agrees(m, n, origins::atom_slice(p, origin, chosen)),
{
    let s = origins::atom_slice(p, origin, chosen);
    clause_selection_reduct(p, g, origin, chosen, n);
    slice::stable_projection(p, m, s);
    assert(stable(slice::mask(p, s), n.intersect(s)));
    slice::mask_wf(p, s);
    assert(rank::local_stratification(slice::mask(p, s), r));
    stratified::stable_unique(slice::mask(p, s), r, m.intersect(s), n.intersect(s));
    assert(m.intersect(s) == n.intersect(s));
    assert forall|a: Atom| s.contains(a) implies m.contains(a) == n.contains(a) by {
        assert(m.intersect(s).contains(a) == m.contains(a));
        assert(n.intersect(s).contains(a) == n.contains(a));
    }
}

// (e)+(f): the actual selected program and full program return the same q-all
// and q-one answers over their independently defined two-valued WFM. Selected
// and full models need not agree outside the query slice.
pub proof fn selected_wfm_queries(
    p: Program,
    g: rank::Graph,
    origin: Seq<int>,
    chosen: Set<int>,
    operators: Set<Atom>,
    valuations: Set<int>,
    goals: spec_fn(int) -> Instance,
)
    requires
        wf(p),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        origins::clause_coverage(p, g, origin),
        rank::forward_closed(g, chosen),
        operators.subset_of(origins::atom_slice(p, origin, chosen)),
        forall|id: int|
            valuations.contains(id) ==> slice::reads(
                goals(id),
                origins::atom_slice(p, origin, chosen),
            ),
    ensures
        afp::lower(p) == afp::upper(p),
        afp::lower(select(p, origin, chosen)) == afp::upper(select(p, origin, chosen)),
        slice::enumerate(afp::lower(p), operators) == slice::enumerate(
            afp::lower(select(p, origin, chosen)),
            operators,
        ),
        slice::goal_answers(afp::lower(p), valuations, goals) == slice::goal_answers(
            afp::lower(select(p, origin, chosen)),
            valuations,
            goals,
        ),
{
    let q = select(p, origin, chosen);
    let r = |a: Atom| origins::atom_rank(p, g, origin, a);
    origins::clause_coverage_stratifies(p, g, origin);
    selection_wf(p, origin, chosen);
    selection_stratifies(p, origin, chosen, r);
    afp::stratified_wfm(p, r);
    afp::stratified_wfm(q, r);
    selected_stable_agrees(p, g, origin, chosen, r, afp::lower(p), afp::lower(q));
    queries_agree(
        afp::lower(p),
        afp::lower(q),
        origins::atom_slice(p, origin, chosen),
        operators,
        valuations,
        goals,
    );
}

} // verus!
