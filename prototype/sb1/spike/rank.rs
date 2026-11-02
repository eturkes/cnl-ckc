use crate::*;

verus! {

pub struct Graph {
    pub nodes: Set<int>,
    pub positive: Set<(int, int)>,
    pub negative: Set<(int, int)>,
}

pub open spec fn edge(g: Graph, u: int, v: int) -> bool {
    g.positive.contains((u, v)) || g.negative.contains((u, v))
}

pub open spec fn graph_wf(g: Graph) -> bool {
    forall|u: int, v: int| edge(g, u, v) ==> g.nodes.contains(u) && g.nodes.contains(v)
}

pub open spec fn forward_closed(g: Graph, s: Set<int>) -> bool {
    forall|u: int, v: int| edge(g, u, v) && s.contains(u) ==> s.contains(v)
}

// Reachability = the least forward-closed set containing u. A negative cycle
// is a negative edge u -> v with a return path v ->* u; reflexive reachability
// includes the one-edge negative self-loop.
pub open spec fn reach(g: Graph, u: int) -> Set<int> {
    g.nodes.filter(
        |v: int| forall|s: Set<int>| s.contains(u) && forward_closed(g, s) ==> s.contains(v),
    )
}

pub open spec fn no_negative_cycle(g: Graph) -> bool {
    forall|u: int, v: int| g.negative.contains((u, v)) ==> !reach(g, v).contains(u)
}

pub open spec fn rank(g: Graph, u: int) -> nat {
    reach(g, u).len()
}

pub proof fn reach_edge(g: Graph, u: int, v: int)
    requires
        graph_wf(g),
        edge(g, u, v),
    ensures
        reach(g, v).subset_of(reach(g, u)),
        reach(g, u).contains(u),
{
    assert forall|w: int| reach(g, v).contains(w) implies reach(g, u).contains(w) by {
        assert forall|s: Set<int>| s.contains(u) && forward_closed(g, s) implies s.contains(w) by {
            assert(s.contains(v));
        }
    }
}

// (b), graph half: finite signed graph + no negative-edge cycle => a natural
// rank, descending strictly on negative edges and weakly on positive edges.
// Counting each node's reachable nodes handles arbitrary positive SCCs.
pub proof fn signed_graph_rank(g: Graph)
    requires
        graph_wf(g),
        no_negative_cycle(g),
    ensures
        forall|u: int, v: int| g.positive.contains((u, v)) ==> rank(g, v) <= rank(g, u),
        forall|u: int, v: int| g.negative.contains((u, v)) ==> rank(g, v) < rank(g, u),
{
    assert forall|u: int, v: int| g.positive.contains((u, v)) implies rank(g, v) <= rank(g, u) by {
        reach_edge(g, u, v);
        vstd::set_lib::lemma_len_subset(reach(g, v), reach(g, u));
    }
    assert forall|u: int, v: int| g.negative.contains((u, v)) implies rank(g, v) < rank(g, u) by {
        reach_edge(g, u, v);
        reach(g, v).lemma_subset_not_in_lt(reach(g, u), u);
    }
}

pub open spec fn local_stratification(p: Program, r: spec_fn(Atom) -> nat) -> bool {
    forall|i: int|
        #![trigger p.rules[i]]
        0 <= i < p.rules.len() ==> {
            let rule = p.rules[i];
            &&& forall|b: Atom| rule.positive.contains(b) ==> r(b) <= r(rule.head)
            &&& forall|j: int, k: int, b: Atom|
                0 <= j < rule.naf.len() && 0 <= k < rule.naf[j].len() && rule.naf[j][k].contains(b)
                    ==> r(b) < r(rule.head)
        }
}

pub open spec fn coverage(p: Program, g: Graph, origin: spec_fn(Atom) -> int) -> bool {
    &&& forall|a: Atom| p.atoms.contains(a) ==> g.nodes.contains(origin(a))
    &&& forall|i: int|
        #![trigger p.rules[i]]
        0 <= i < p.rules.len() ==> {
            let r = p.rules[i];
            &&& forall|b: Atom|
                r.positive.contains(b) ==> g.positive.contains((origin(r.head), origin(b)))
            &&& forall|j: int, k: int, b: Atom|
                0 <= j < r.naf.len() && 0 <= k < r.naf[j].len() && r.naf[j][k].contains(b)
                    ==> g.negative.contains((origin(r.head), origin(b)))
        }
}

pub proof fn coverage_stratifies(p: Program, g: Graph, origin: spec_fn(Atom) -> int)
    requires
        wf(p),
        graph_wf(g),
        no_negative_cycle(g),
        coverage(p, g, origin),
    ensures
        local_stratification(p, |a: Atom| rank(g, origin(a))),
{
    signed_graph_rank(g);
}

// (c), coverage bridge: abstract predecessor closure implies concrete
// predecessor closure. Every NAF witness atom participates in coverage.
pub proof fn coverage_slice(p: Program, g: Graph, origin: spec_fn(Atom) -> int, selected: Set<int>)
    requires
        wf(p),
        coverage(p, g, origin),
        forward_closed(g, selected),
    ensures
        slice::predecessor_closed(p, p.atoms.filter(|a: Atom| selected.contains(origin(a)))),
{
    let s = p.atoms.filter(|a: Atom| selected.contains(origin(a)));
    assert forall|i: int|
        0 <= i < p.rules.len() && s.contains(p.rules[i].head) implies slice::reads(
        p.rules[i],
        s,
    ) by {
        let r = p.rules[i];
        assert forall|b: Atom| r.positive.contains(b) implies s.contains(b) by {
            assert(g.positive.contains((origin(r.head), origin(b))));
            assert(edge(g, origin(r.head), origin(b)));
        }
        assert forall|j: int, k: int|
            0 <= j < r.naf.len() && 0 <= k < r.naf[j].len() implies r.naf[j][k].subset_of(s) by {
            assert forall|b: Atom| r.naf[j][k].contains(b) implies s.contains(b) by {
                assert(g.negative.contains((origin(r.head), origin(b))));
                assert(edge(g, origin(r.head), origin(b)));
            }
        }
    }
}

} // verus!
