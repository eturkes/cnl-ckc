use crate::*;

verus! {

pub open spec fn maximum(ids: Set<int>, f: spec_fn(int) -> nat) -> nat
    decreases ids.len(),
{
    if ids.is_empty() {
        0
    } else {
        let i = ids.choose();
        let rest = maximum(ids.remove(i), f);
        if f(i) > rest {
            f(i)
        } else {
            rest
        }
    }
}

pub proof fn maximum_member(ids: Set<int>, f: spec_fn(int) -> nat, i: int)
    requires
        ids.contains(i),
    ensures
        f(i) <= maximum(ids, f),
    decreases ids.len(),
{
    let a = ids.choose();
    if a != i {
        maximum_member(ids.remove(a), f, i);
    }
}

pub proof fn maximum_bound(ids: Set<int>, f: spec_fn(int) -> nat, bound: nat)
    requires
        forall|i: int| ids.contains(i) ==> f(i) <= bound,
    ensures
        maximum(ids, f) <= bound,
    decreases ids.len(),
{
    if !ids.is_empty() {
        let a = ids.choose();
        maximum_bound(ids.remove(a), f, bound);
    }
}

pub open spec fn producers(p: Program, a: Atom) -> Set<int> {
    Set::range(0, p.rules.len() as int).filter(|i: int| p.rules[i].head == a)
}

// Origin[i] = abstract clause node of ground rule instance i. One fact can
// have many producing nodes; taking their maximum rank covers every producer.
// Producerless atoms receive rank zero.
pub open spec fn atom_rank(p: Program, g: rank::Graph, origin: Seq<int>, a: Atom) -> nat {
    maximum(producers(p, a), |i: int| rank::rank(g, origin[i]))
}

pub open spec fn clause_coverage(p: Program, g: rank::Graph, origin: Seq<int>) -> bool {
    &&& origin.len() == p.rules.len()
    &&& forall|i: int| 0 <= i < p.rules.len() ==> g.nodes.contains(origin[i])
    &&& forall|i: int, j: int|
        0 <= i < p.rules.len() && 0 <= j < p.rules.len() && p.rules[i].positive.contains(
            p.rules[j].head,
        ) ==> g.positive.contains((origin[i], origin[j]))
    &&& forall|i: int, j: int, n: int, w: int|
        0 <= i < p.rules.len() && 0 <= j < p.rules.len() && 0 <= n < p.rules[i].naf.len() && 0 <= w
            < p.rules[i].naf[n].len() && p.rules[i].naf[n][w].contains(p.rules[j].head)
            ==> g.negative.contains((origin[i], origin[j]))
}

pub proof fn node_rank_positive(g: rank::Graph, node: int)
    requires
        g.nodes.contains(node),
    ensures
        rank::rank(g, node) > 0,
{
    assert(rank::reach(g, node).contains(node));
}

// (b), candidate-specific coverage lift: abstract nodes are clauses, not atom
// names. Coverage includes every possible producer of a positive/NAF atom.
// Ground instances of one source clause share its abstract node; the lift
// tolerates arbitrarily many origins for one atom.
pub proof fn clause_coverage_stratifies(p: Program, g: rank::Graph, origin: Seq<int>)
    requires
        wf(p),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        clause_coverage(p, g, origin),
    ensures
        rank::local_stratification(p, |a: Atom| atom_rank(p, g, origin, a)),
{
    rank::signed_graph_rank(g);
    let f = |j: int| rank::rank(g, origin[j]);
    assert forall|i: int| #![trigger p.rules[i]] 0 <= i < p.rules.len() implies {
        let rule = p.rules[i];
        &&& forall|b: Atom|
            rule.positive.contains(b) ==> atom_rank(p, g, origin, b) <= atom_rank(
                p,
                g,
                origin,
                rule.head,
            )
        &&& forall|n: int, w: int, b: Atom|
            0 <= n < rule.naf.len() && 0 <= w < rule.naf[n].len() && rule.naf[n][w].contains(b)
                ==> atom_rank(p, g, origin, b) < atom_rank(p, g, origin, rule.head)
    } by {
        let rule = p.rules[i];
        assert(producers(p, rule.head).contains(i));
        maximum_member(producers(p, rule.head), f, i);
        node_rank_positive(g, origin[i]);
        assert forall|b: Atom| rule.positive.contains(b) implies atom_rank(p, g, origin, b)
            <= atom_rank(p, g, origin, rule.head) by {
            assert forall|j: int| producers(p, b).contains(j) implies f(j) <= f(i) by {
                assert(g.positive.contains((origin[i], origin[j])));
            }
            maximum_bound(producers(p, b), f, f(i));
        }
        assert forall|n: int, w: int, b: Atom|
            0 <= n < rule.naf.len() && 0 <= w < rule.naf[n].len() && rule.naf[n][w].contains(
                b,
            ) implies atom_rank(p, g, origin, b) < atom_rank(p, g, origin, rule.head) by {
            assert forall|j: int| producers(p, b).contains(j) implies f(j) <= (f(i) - 1) as nat by {
                assert(g.negative.contains((origin[i], origin[j])));
            }
            maximum_bound(producers(p, b), f, (f(i) - 1) as nat);
        }
    }
}

// (b) Clause-origin signed coverage + no negative cycle => existence and
// uniqueness of a stable model, under exactly lemma (a)'s reduct/lfp semantics.
pub proof fn clause_covered_unique_model(p: Program, g: rank::Graph, origin: Seq<int>)
    requires
        wf(p),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        clause_coverage(p, g, origin),
    ensures
        exists|m: Set<Atom>| stable(p, m) && forall|n: Set<Atom>| stable(p, n) ==> n == m,
{
    clause_coverage_stratifies(p, g, origin);
    let r = |a: Atom| atom_rank(p, g, origin, a);
    stratified::stratified_unique_model(p, r);
    let m = stratified::perfect(p, r, stratified::height(p.atoms, r));
    assert(stable(p, m) && forall|n: Set<Atom>| stable(p, n) ==> n == m);
}

pub open spec fn atom_slice(p: Program, origin: Seq<int>, selected: Set<int>) -> Set<Atom> {
    p.atoms.filter(
        |a: Atom|
            forall|i: int|
                0 <= i < p.rules.len() && p.rules[i].head == a ==> selected.contains(origin[i]),
    )
}

// (c), clause-node slice lift: a query selects every producer of each root.
// Transitive abstract predecessor closure then includes every producer of each
// positive/NAF dependency. Producerless atoms are harmless and remain included.
pub proof fn clause_coverage_slice(p: Program, g: rank::Graph, origin: Seq<int>, selected: Set<int>)
    requires
        wf(p),
        clause_coverage(p, g, origin),
        rank::forward_closed(g, selected),
    ensures
        slice::predecessor_closed(p, atom_slice(p, origin, selected)),
{
    let s = atom_slice(p, origin, selected);
    assert forall|i: int|
        0 <= i < p.rules.len() && s.contains(p.rules[i].head) implies slice::reads(
        p.rules[i],
        s,
    ) by {
        let rule = p.rules[i];
        assert(selected.contains(origin[i]));
        assert forall|b: Atom| rule.positive.contains(b) implies s.contains(b) by {
            assert forall|j: int|
                0 <= j < p.rules.len() && p.rules[j].head == b implies selected.contains(
                origin[j],
            ) by {
                assert(g.positive.contains((origin[i], origin[j])));
                assert(rank::edge(g, origin[i], origin[j]));
            }
        }
        assert forall|n: int, w: int|
            0 <= n < rule.naf.len() && 0 <= w < rule.naf[n].len() implies rule.naf[n][w].subset_of(
            s,
        ) by {
            assert forall|b: Atom| rule.naf[n][w].contains(b) implies s.contains(b) by {
                assert forall|j: int|
                    0 <= j < p.rules.len() && p.rules[j].head == b implies selected.contains(
                    origin[j],
                ) by {
                    assert(g.negative.contains((origin[i], origin[j])));
                    assert(rank::edge(g, origin[i], origin[j]));
                }
            }
        }
    }
}

} // verus!
