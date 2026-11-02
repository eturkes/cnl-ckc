use crate::*;

verus! {

pub open spec fn reads(r: Instance, s: Set<Atom>) -> bool {
    r.positive.subset_of(s) && forall|j: int, k: int|
        0 <= j < r.naf.len() && 0 <= k < r.naf[j].len() ==> r.naf[j][k].subset_of(s)
}

pub open spec fn predecessor_closed(p: Program, s: Set<Atom>) -> bool {
    s.subset_of(p.atoms) && forall|i: int|
        0 <= i < p.rules.len() && s.contains(p.rules[i].head) ==> reads(p.rules[i], s)
}

// False built-ins delete rules semantically while keeping clause ids stable.
pub open spec fn mask_rule(r: Instance, s: Set<Atom>) -> Instance {
    Instance {
        head: r.head,
        positive: r.positive,
        naf: r.naf,
        builtins: r.builtins && s.contains(r.head),
    }
}

pub open spec fn mask(p: Program, s: Set<Atom>) -> Program {
    Program { atoms: p.atoms, rules: p.rules.map_values(|r: Instance| mask_rule(r, s)) }
}

pub open spec fn agrees(m: Set<Atom>, n: Set<Atom>, s: Set<Atom>) -> bool {
    forall|a: Atom| s.contains(a) ==> m.contains(a) == n.contains(a)
}

pub proof fn subset_agrees(w: Set<Atom>, m: Set<Atom>, n: Set<Atom>, s: Set<Atom>)
    requires
        w.subset_of(s),
        agrees(m, n, s),
    ensures
        w.subset_of(m) == w.subset_of(n),
{
}

pub proof fn retained_agrees(r: Instance, m: Set<Atom>, n: Set<Atom>, s: Set<Atom>)
    requires
        reads(r, s),
        agrees(m, n, s),
    ensures
        retained(r, m) == retained(r, n),
{
    assert forall|j: int, k: int|
        0 <= j < r.naf.len() && 0 <= k < r.naf[j].len() implies r.naf[j][k].subset_of(m)
        == r.naf[j][k].subset_of(n) by {
        subset_agrees(r.naf[j][k], m, n, s);
    }
}

pub proof fn mask_wf(p: Program, s: Set<Atom>)
    requires
        wf(p),
    ensures
        wf(mask(p, s)),
{
}

pub proof fn mask_retained(r: Instance, m: Set<Atom>, n: Set<Atom>, s: Set<Atom>)
    requires
        s.contains(r.head),
        reads(r, s),
        agrees(m, n, s),
    ensures
        retained(mask_rule(r, s), n) == retained(r, m),
{
    retained_agrees(r, m, n, s);
}

// (c), reduct projection: a predecessor-closed atom slice includes all positive
// dependencies and every candidate conjunction witness inside every NAF item.
// For arbitrary reduct reference M, restricting the rules and M to the slice
// commutes with the least positive fixed point; stable-model projection follows.
pub proof fn reduct_projection(p: Program, m: Set<Atom>, s: Set<Atom>)
    requires
        wf(p),
        predecessor_closed(p, s),
    ensures
        lfp(mask(p, s), m.intersect(s)) == lfp(p, m).intersect(s),
{
    let q = mask(p, s);
    let n = m.intersect(s);
    mask_wf(p, s);
    support::lfp_closed(p, m);
    support::lfp_closed(q, n);
    assert(agrees(m, n, s));
    assert(closed(q, n, s));
    support::least(q, n, s);
    assert forall|i: int|
        0 <= i < q.rules.len() && retained(q.rules[i], n) && q.rules[i].positive.subset_of(
            lfp(p, m),
        ) implies lfp(p, m).contains(q.rules[i].head) by {
        let r = p.rules[i];
        mask_retained(r, m, n, s);
    }
    support::least(q, n, lfp(p, m));
    let x = lfp(q, n).union(p.atoms.difference(s));
    assert forall|i: int|
        0 <= i < p.rules.len() && retained(p.rules[i], m) && p.rules[i].positive.subset_of(
            x,
        ) implies x.contains(p.rules[i].head) by {
        let r = p.rules[i];
        if s.contains(r.head) {
            mask_retained(r, m, n, s);
            assert(r.positive.subset_of(lfp(q, n)));
            assert(q.rules[i] == mask_rule(r, s));
            assert(lfp(q, n).contains(r.head));
        }
    }
    support::least(p, m, x);
    assert(lfp(q, n) =~= lfp(p, m).intersect(s));
}

pub proof fn stable_projection(p: Program, m: Set<Atom>, s: Set<Atom>)
    requires
        wf(p),
        predecessor_closed(p, s),
        stable(p, m),
    ensures
        stable(mask(p, s), m.intersect(s)),
{
    reduct_projection(p, m, s);
}

pub open spec fn enumerate(m: Set<Atom>, candidates: Set<Atom>) -> Set<Atom> {
    m.intersect(candidates)
}

pub open spec fn goal_holds(r: Instance, m: Set<Atom>) -> bool {
    retained(r, m) && r.positive.subset_of(m)
}

// Each id denotes one ground valuation of a goal list. Projection/dedup can
// identify ids afterwards; equality of these sets preserves every projection.
pub open spec fn goal_answers(
    m: Set<Atom>,
    domain: Set<int>,
    goals: spec_fn(int) -> Instance,
) -> Set<int> {
    domain.filter(|id: int| goal_holds(goals(id), m))
}

// (c), query shapes: enumeration of designated operator atoms (q-all), and all
// ground valuations of a goal list (q-one; also q-all's payload goal lists),
// return identical answers on the full and projected stable models.
pub proof fn query_preservation(
    p: Program,
    m: Set<Atom>,
    s: Set<Atom>,
    operators: Set<Atom>,
    valuations: Set<int>,
    goals: spec_fn(int) -> Instance,
)
    requires
        wf(p),
        predecessor_closed(p, s),
        stable(p, m),
        operators.subset_of(s),
        forall|id: int| valuations.contains(id) ==> reads(goals(id), s),
    ensures
        stable(mask(p, s), m.intersect(s)),
        enumerate(m, operators) == enumerate(m.intersect(s), operators),
        goal_answers(m, valuations, goals) == goal_answers(m.intersect(s), valuations, goals),
{
    stable_projection(p, m, s);
    assert(enumerate(m, operators) =~= enumerate(m.intersect(s), operators));
    assert forall|id: int| valuations.contains(id) implies goal_holds(goals(id), m) == goal_holds(
        goals(id),
        m.intersect(s),
    ) by {
        retained_agrees(goals(id), m, m.intersect(s), s);
        subset_agrees(goals(id).positive, m, m.intersect(s), s);
    }
    assert(goal_answers(m, valuations, goals) =~= goal_answers(m.intersect(s), valuations, goals));
}

} // verus!
