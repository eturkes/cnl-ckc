use crate::*;

verus! {

// Gamma(M) is exactly lemma (a)'s positive-reduct least fixed point. Each NAF
// witness remains one whole correlated conjunction, including empty witnesses.
pub open spec fn double_gamma(p: Program, m: Set<Atom>) -> Set<Atom> {
    lfp(p, lfp(p, m))
}

pub proof fn retained_antitone(rule: Instance, m: Set<Atom>, n: Set<Atom>)
    requires
        m.subset_of(n),
        retained(rule, n),
    ensures
        retained(rule, m),
{
}

pub proof fn gamma_antitone(p: Program, m: Set<Atom>, n: Set<Atom>)
    requires
        wf(p),
        m.subset_of(n),
    ensures
        lfp(p, n).subset_of(lfp(p, m)),
{
    support::lfp_closed(p, m);
    assert forall|i: int|
        0 <= i < p.rules.len() && retained(p.rules[i], n) && p.rules[i].positive.subset_of(
            lfp(p, m),
        ) implies lfp(p, m).contains(p.rules[i].head) by {
        retained_antitone(p.rules[i], m, n);
    }
    support::least(p, n, lfp(p, m));
}

pub proof fn double_monotone(p: Program, m: Set<Atom>, n: Set<Atom>)
    requires
        wf(p),
        m.subset_of(n),
    ensures
        double_gamma(p, m).subset_of(double_gamma(p, n)),
{
    gamma_antitone(p, m, n);
    gamma_antitone(p, lfp(p, n), lfp(p, m));
}

// Independent alternating-fixpoint definition: lower = lfp(Gamma o Gamma),
// expressed as the intersection of all its pre-fixed points. Upper = Gamma
// lower; true/false/undefined are lower / (Atoms - upper) / (upper - lower).
// No rank, perfect model, certificate or stable-model choice enters this def.
pub open spec fn lower(p: Program) -> Set<Atom> {
    p.atoms.filter(|a: Atom| forall|x: Set<Atom>| double_gamma(p, x).subset_of(x) ==> x.contains(a))
}

pub open spec fn upper(p: Program) -> Set<Atom> {
    lfp(p, lower(p))
}

pub open spec fn false_atoms(p: Program) -> Set<Atom> {
    p.atoms.difference(upper(p))
}

pub open spec fn undefined_atoms(p: Program) -> Set<Atom> {
    upper(p).difference(lower(p))
}

pub proof fn lower_least(p: Program, x: Set<Atom>)
    requires
        double_gamma(p, x).subset_of(x),
    ensures
        lower(p).subset_of(x),
{
}

pub proof fn lower_fixed(p: Program)
    requires
        wf(p),
    ensures
        lower(p) == double_gamma(p, lower(p)),
{
    let l = lower(p);
    let step = double_gamma(p, l);
    assert forall|a: Atom| step.contains(a) implies l.contains(a) by {
        assert forall|x: Set<Atom>| double_gamma(p, x).subset_of(x) implies x.contains(a) by {
            lower_least(p, x);
            double_monotone(p, l, x);
        }
    }
    double_monotone(p, step, l);
    lower_least(p, step);
    assert(l =~= step);
}

pub proof fn alternating_bounds(p: Program)
    requires
        wf(p),
    ensures
        lower(p).subset_of(upper(p)),
        upper(p).subset_of(p.atoms),
        lower(p) == lfp(p, upper(p)),
        upper(p) == lfp(p, lower(p)),
        double_gamma(p, upper(p)) == upper(p),
{
    lower_fixed(p);
    assert(double_gamma(p, upper(p)) == upper(p));
    lower_least(p, upper(p));
}

pub open spec fn iterate(p: Program, k: nat) -> Set<Atom>
    decreases k,
{
    if k == 0 {
        Set::empty()
    } else {
        double_gamma(p, iterate(p, (k - 1) as nat))
    }
}

pub proof fn iteration_chain(p: Program, k: nat)
    requires
        wf(p),
    ensures
        iterate(p, k).subset_of(p.atoms),
        iterate(p, k).subset_of(iterate(p, k + 1)),
    decreases k,
{
    if k > 0 {
        iteration_chain(p, (k - 1) as nat);
        double_monotone(p, iterate(p, (k - 1) as nat), iterate(p, k));
    }
}

pub proof fn iteration_below_prefixed(p: Program, k: nat, x: Set<Atom>)
    requires
        wf(p),
        double_gamma(p, x).subset_of(x),
    ensures
        iterate(p, k).subset_of(x),
    decreases k,
{
    if k > 0 {
        iteration_below_prefixed(p, (k - 1) as nat, x);
        double_monotone(p, iterate(p, (k - 1) as nat), x);
    }
}

pub proof fn iteration_growth(p: Program, k: nat)
    requires
        wf(p),
    ensures
        iterate(p, k) == double_gamma(p, iterate(p, k)) || iterate(p, k).len() >= k,
    decreases k,
{
    if k > 0 {
        iteration_growth(p, (k - 1) as nat);
        let previous = iterate(p, (k - 1) as nat);
        let current = iterate(p, k);
        iteration_chain(p, (k - 1) as nat);
        if previous != current {
            vstd::set_lib::lemma_len_subset(previous, current);
            if previous.len() == current.len() {
                vstd::set_lib::lemma_subset_equality(previous, current);
            }
        }
    }
}

// The independent lfp definition is exactly finite alternating iteration from
// the empty true set, not a definition by the stratum/perfect model.
pub proof fn finite_alternation(p: Program)
    requires
        wf(p),
    ensures
        lower(p) == iterate(p, p.atoms.len()),
        iterate(p, p.atoms.len()) == double_gamma(p, iterate(p, p.atoms.len())),
{
    let n = p.atoms.len();
    let result = iterate(p, n);
    iteration_chain(p, n);
    iteration_growth(p, n);
    vstd::set_lib::lemma_len_subset(result, p.atoms);
    if result != double_gamma(p, result) {
        vstd::set_lib::lemma_subset_equality(result, p.atoms);
        assert(result =~= double_gamma(p, result));
    }
    lower_least(p, result);
    lower_fixed(p);
    iteration_below_prefixed(p, n, lower(p));
    assert(result =~= lower(p));
}

pub proof fn cross_stratum_inclusion(
    p: Program,
    r: spec_fn(Atom) -> nat,
    m: Set<Atom>,
    n: Set<Atom>,
    k: nat,
)
    requires
        wf(p),
        rank::local_stratification(p, r),
        m == lfp(p, n),
        n == lfp(p, m),
        slice::agrees(m, n, stratified::below(p, r, k)),
    ensures
        forall|a: Atom| p.atoms.contains(a) && r(a) <= k && m.contains(a) ==> n.contains(a),
{
    support::lfp_closed(p, m);
    let x = n.union(p.atoms.filter(|a: Atom| r(a) > k));
    assert forall|i: int|
        0 <= i < p.rules.len() && retained(p.rules[i], n) && p.rules[i].positive.subset_of(
            x,
        ) implies x.contains(p.rules[i].head) by {
        let rule = p.rules[i];
        if r(rule.head) <= k {
            stratified::negative_below(p, r, i, k);
            stratified::negative_agrees(rule, m, n, stratified::below(p, r, k));
            assert(rule.positive.subset_of(n));
            assert(n.contains(rule.head));
        }
    }
    support::least(p, n, x);
}

pub proof fn cross_agreement(
    p: Program,
    r: spec_fn(Atom) -> nat,
    m: Set<Atom>,
    n: Set<Atom>,
    k: nat,
)
    requires
        wf(p),
        rank::local_stratification(p, r),
        m == lfp(p, n),
        n == lfp(p, m),
    ensures
        slice::agrees(m, n, stratified::below(p, r, k)),
    decreases k,
{
    if k > 0 {
        cross_agreement(p, r, m, n, (k - 1) as nat);
        cross_stratum_inclusion(p, r, m, n, (k - 1) as nat);
        cross_stratum_inclusion(p, r, n, m, (k - 1) as nat);
    }
}

// Local stratification rules out distinct alternating pairs, a stronger fact
// than stable-model uniqueness alone (which does not imply total WFS).
pub proof fn cross_pair_total(p: Program, r: spec_fn(Atom) -> nat, m: Set<Atom>, n: Set<Atom>)
    requires
        wf(p),
        rank::local_stratification(p, r),
        m == lfp(p, n),
        n == lfp(p, m),
    ensures
        m == n,
        stable(p, m),
{
    assert forall|a: Atom| p.atoms.contains(a) implies m.contains(a) == n.contains(a) by {
        cross_agreement(p, r, m, n, r(a) + 1);
    }
    assert(m =~= n);
}

// (e) Finite well-formed ground program + lemma (b)'s local stratification =>
// its independently defined alternating-fixpoint WFM is two-valued and equals
// (b)'s constructed unique stable model. Neither totality nor equality is a
// premise; Gamma and every reduct are the exact definitions from lemma (a).
pub proof fn stratified_wfm(p: Program, r: spec_fn(Atom) -> nat)
    requires
        wf(p),
        rank::local_stratification(p, r),
    ensures
        lower(p) == upper(p),
        stable(p, lower(p)),
        lower(p) == iterate(p, p.atoms.len()),
        lower(p) == stratified::perfect(p, r, stratified::height(p.atoms, r)),
        undefined_atoms(p) == Set::<Atom>::empty(),
        false_atoms(p) == p.atoms.difference(
            stratified::perfect(p, r, stratified::height(p.atoms, r)),
        ),
{
    finite_alternation(p);
    alternating_bounds(p);
    cross_pair_total(p, r, lower(p), upper(p));
    stratified::stratified_unique_model(p, r);
    assert(undefined_atoms(p) =~= Set::<Atom>::empty());
}

pub proof fn covered_wfm(p: Program, g: rank::Graph, origin: Seq<int>)
    requires
        wf(p),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        origins::clause_coverage(p, g, origin),
    ensures
        lower(p) == upper(p),
        stable(p, lower(p)),
        forall|m: Set<Atom>| stable(p, m) ==> m == lower(p),
{
    origins::clause_coverage_stratifies(p, g, origin);
    let r = |a: Atom| origins::atom_rank(p, g, origin, a);
    stratified_wfm(p, r);
    assert forall|m: Set<Atom>| stable(p, m) implies m == lower(p) by {
        stratified::stable_unique(p, r, m, lower(p));
    }
}

pub proof fn certified_is_wfm(
    p: Program,
    r: spec_fn(Atom) -> nat,
    m: Set<Atom>,
    level: spec_fn(Atom) -> nat,
)
    requires
        wf(p),
        rank::local_stratification(p, r),
        m.subset_of(p.atoms),
        founded(p, m, level),
        closed(p, m, m),
    ensures
        m == lower(p),
        m == upper(p),
{
    support::support_closure_lfp(p, m, level);
    stratified_wfm(p, r);
    stratified::stable_unique(p, r, m, lower(p));
}

} // verus!
