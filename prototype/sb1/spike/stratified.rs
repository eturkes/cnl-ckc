use crate::*;

verus! {

pub open spec fn below(p: Program, r: spec_fn(Atom) -> nat, k: nat) -> Set<Atom> {
    p.atoms.filter(|a: Atom| r(a) < k)
}

pub proof fn prefix_closed(p: Program, r: spec_fn(Atom) -> nat, k: nat)
    requires
        wf(p),
        rank::local_stratification(p, r),
    ensures
        slice::predecessor_closed(p, below(p, r, k)),
{
}

pub proof fn negative_below(p: Program, r: spec_fn(Atom) -> nat, i: int, k: nat)
    requires
        wf(p),
        rank::local_stratification(p, r),
        0 <= i < p.rules.len(),
        r(p.rules[i].head) <= k,
    ensures
        forall|j: int, w: int|
            0 <= j < p.rules[i].naf.len() && 0 <= w < p.rules[i].naf[j].len()
                ==> p.rules[i].naf[j][w].subset_of(below(p, r, k)),
{
}

pub proof fn negative_agrees(rule: Instance, m: Set<Atom>, n: Set<Atom>, s: Set<Atom>)
    requires
        slice::agrees(m, n, s),
        forall|j: int, k: int|
            0 <= j < rule.naf.len() && 0 <= k < rule.naf[j].len() ==> rule.naf[j][k].subset_of(s),
    ensures
        retained(rule, m) == retained(rule, n),
{
    assert forall|j: int, k: int|
        0 <= j < rule.naf.len() && 0 <= k < rule.naf[j].len() implies rule.naf[j][k].subset_of(m)
        == rule.naf[j][k].subset_of(n) by {
        slice::subset_agrees(rule.naf[j][k], m, n, s);
    }
}

pub proof fn stratum_inclusion(
    p: Program,
    r: spec_fn(Atom) -> nat,
    m: Set<Atom>,
    n: Set<Atom>,
    k: nat,
)
    requires
        wf(p),
        rank::local_stratification(p, r),
        stable(p, m),
        stable(p, n),
        slice::agrees(m, n, below(p, r, k)),
    ensures
        forall|a: Atom| p.atoms.contains(a) && r(a) <= k && m.contains(a) ==> n.contains(a),
{
    support::lfp_closed(p, n);
    let x = n.union(p.atoms.filter(|a: Atom| r(a) > k));
    assert forall|i: int|
        0 <= i < p.rules.len() && retained(p.rules[i], m) && p.rules[i].positive.subset_of(
            x,
        ) implies x.contains(p.rules[i].head) by {
        let rule = p.rules[i];
        if r(rule.head) <= k {
            negative_below(p, r, i, k);
            negative_agrees(rule, m, n, below(p, r, k));
            assert(rule.positive.subset_of(n));
            assert(n.contains(rule.head));
        }
    }
    support::least(p, m, x);
}

pub proof fn stable_agreement(
    p: Program,
    r: spec_fn(Atom) -> nat,
    m: Set<Atom>,
    n: Set<Atom>,
    k: nat,
)
    requires
        wf(p),
        rank::local_stratification(p, r),
        stable(p, m),
        stable(p, n),
    ensures
        slice::agrees(m, n, below(p, r, k)),
    decreases k,
{
    if k > 0 {
        stable_agreement(p, r, m, n, (k - 1) as nat);
        stratum_inclusion(p, r, m, n, (k - 1) as nat);
        stratum_inclusion(p, r, n, m, (k - 1) as nat);
    }
}

pub proof fn stable_unique(p: Program, r: spec_fn(Atom) -> nat, m: Set<Atom>, n: Set<Atom>)
    requires
        wf(p),
        rank::local_stratification(p, r),
        stable(p, m),
        stable(p, n),
    ensures
        m == n,
{
    assert forall|a: Atom| p.atoms.contains(a) implies m.contains(a) == n.contains(a) by {
        stable_agreement(p, r, m, n, r(a) + 1);
    }
    assert(m =~= n);
}

pub proof fn equal_reduct(p: Program, m: Set<Atom>, n: Set<Atom>)
    requires
        forall|i: int|
            0 <= i < p.rules.len() ==> retained(p.rules[i], m) == retained(p.rules[i], n),
    ensures
        lfp(p, m) == lfp(p, n),
{
    assert forall|x: Set<Atom>| closed(p, m, x) == closed(p, n, x) by {};
    assert(lfp(p, m) =~= lfp(p, n));
}

pub proof fn nested_mask(p: Program, s: Set<Atom>, t: Set<Atom>)
    requires
        t.subset_of(s),
    ensures
        slice::mask(slice::mask(p, s), t) == slice::mask(p, t),
{
    let a = slice::mask(slice::mask(p, s), t);
    let b = slice::mask(p, t);
    assert(a.rules =~= b.rules);
    assert(a == b);
}

pub proof fn full_mask(p: Program)
    requires
        wf(p),
    ensures
        slice::mask(p, p.atoms) == p,
{
    assert(slice::mask(p, p.atoms).rules =~= p.rules);
}

pub open spec fn perfect(p: Program, r: spec_fn(Atom) -> nat, k: nat) -> Set<Atom>
    decreases k,
{
    if k == 0 {
        Set::empty()
    } else {
        lfp(slice::mask(p, below(p, r, k)), perfect(p, r, (k - 1) as nat))
    }
}

// Each stratum takes a positive least fixed point after lower strata have
// fixed every NAF witness. Positive recursion stays inside that fixed point.
pub proof fn perfect_stable(p: Program, r: spec_fn(Atom) -> nat, k: nat)
    requires
        wf(p),
        rank::local_stratification(p, r),
    ensures
        stable(slice::mask(p, below(p, r, k)), perfect(p, r, k)),
        perfect(p, r, k).subset_of(below(p, r, k)),
    decreases k,
{
    let s = below(p, r, k);
    let q = slice::mask(p, s);
    slice::mask_wf(p, s);
    if k == 0 {
        assert(s =~= Set::<Atom>::empty());
        assert(closed(q, Set::empty(), Set::empty()));
        support::least(q, Set::empty(), Set::empty());
        assert(lfp(q, Set::empty()) =~= Set::<Atom>::empty());
    } else {
        perfect_stable(p, r, (k - 1) as nat);
        let previous = perfect(p, r, (k - 1) as nat);
        let t = below(p, r, (k - 1) as nat);
        let current = perfect(p, r, k);
        assert(t.subset_of(s));
        assert(rank::local_stratification(q, r));
        assert(closed(q, previous, s));
        support::least(q, previous, s);
        prefix_closed(q, r, (k - 1) as nat);
        assert(below(q, r, (k - 1) as nat) == t);
        slice::reduct_projection(q, previous, t);
        nested_mask(p, s, t);
        assert(previous.intersect(t) =~= previous);
        assert(current.intersect(t) == previous);
        assert(slice::agrees(previous, current, t));
        assert forall|i: int| 0 <= i < q.rules.len() implies retained(q.rules[i], previous)
            == retained(q.rules[i], current) by {
            if q.rules[i].builtins {
                negative_below(p, r, i, (k - 1) as nat);
                negative_agrees(q.rules[i], previous, current, t);
            }
        }
        equal_reduct(q, previous, current);
    }
}

pub open spec fn height(atoms: Set<Atom>, r: spec_fn(Atom) -> nat) -> nat
    decreases atoms.len(),
{
    if atoms.is_empty() {
        0
    } else {
        let a = atoms.choose();
        let rest = height(atoms.remove(a), r);
        if r(a) + 1 > rest {
            r(a) + 1
        } else {
            rest
        }
    }
}

pub proof fn height_bound(atoms: Set<Atom>, r: spec_fn(Atom) -> nat)
    ensures
        forall|a: Atom| atoms.contains(a) ==> r(a) < height(atoms, r),
    decreases atoms.len(),
{
    if !atoms.is_empty() {
        let a = atoms.choose();
        height_bound(atoms.remove(a), r);
    }
}

// (b), semantic half: every finite locally stratified ground program has
// exactly one stable model, using the same lfp/reduct as lemma (a). The model
// is constructed stratum by stratum; neither existence nor uniqueness is a
// precondition. Parser/coverage correctness remains a separate production duty.
pub proof fn stratified_unique_model(p: Program, r: spec_fn(Atom) -> nat)
    requires
        wf(p),
        rank::local_stratification(p, r),
    ensures
        stable(p, perfect(p, r, height(p.atoms, r))),
        forall|m: Set<Atom>| stable(p, m) ==> m == perfect(p, r, height(p.atoms, r)),
{
    height_bound(p.atoms, r);
    let h = height(p.atoms, r);
    assert(below(p, r, h) =~= p.atoms);
    perfect_stable(p, r, h);
    full_mask(p);
    assert forall|m: Set<Atom>| stable(p, m) implies m == perfect(p, r, h) by {
        stable_unique(p, r, m, perfect(p, r, h));
    }
}

pub proof fn covered_unique_model(p: Program, g: rank::Graph, origin: spec_fn(Atom) -> int)
    requires
        wf(p),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        rank::coverage(p, g, origin),
    ensures
        exists|m: Set<Atom>| stable(p, m) && forall|n: Set<Atom>| stable(p, n) ==> n == m,
{
    rank::coverage_stratifies(p, g, origin);
    let r = |a: Atom| rank::rank(g, origin(a));
    stratified_unique_model(p, r);
    let m = perfect(p, r, height(p.atoms, r));
    assert(stable(p, m) && forall|n: Set<Atom>| stable(p, n) ==> n == m);
}

} // verus!
