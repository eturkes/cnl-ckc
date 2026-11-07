use crate::*;

verus! {

// CSR adjacency over primitive node ids: node u's successors are
// targets[offsets[u] .. offsets[u + 1]].
pub open spec fn csr_wf(offsets: Seq<usize>, targets: Seq<usize>) -> bool {
    &&& 1 <= offsets.len()
    &&& forall|u: int| 0 <= u < offsets.len() - 1 ==> #[trigger] offsets[u] <= offsets[u + 1]
    &&& forall|u: int| 0 <= u < offsets.len() ==> #[trigger] offsets[u] <= targets.len()
    &&& forall|k: int| 0 <= k < targets.len() ==> #[trigger] targets[k] < offsets.len() - 1
}

pub open spec fn csr_edge(offsets: Seq<usize>, targets: Seq<usize>, u: int, v: int) -> bool {
    0 <= u < offsets.len() - 1 && exists|k: int|
        offsets[u] <= k < offsets[u + 1] && #[trigger] targets[k] == v
}

// The checker's build obligation: every edge of the declarative graph is
// stored (T3's streamed-edge representation check).
pub open spec fn represents(g: rank::Graph, offsets: Seq<usize>, targets: Seq<usize>) -> bool {
    forall|u: int, v: int| #[trigger] rank::edge(g, u, v) ==> csr_edge(offsets, targets, u, v)
}

pub open spec fn done(offsets: Seq<usize>, targets: Seq<usize>, mark: Seq<bool>, u: int) -> bool {
    forall|k: int|
        #![trigger targets[k]]
        offsets[u] <= k < offsets[u + 1] ==> mark[targets[k] as int]
}

pub open spec fn csr_closed(offsets: Seq<usize>, targets: Seq<usize>, mark: Seq<bool>) -> bool {
    forall|u: int| 0 <= u < mark.len() && #[trigger] mark[u] ==> done(offsets, targets, mark, u)
}

pub open spec fn marked(mark: Seq<bool>) -> Set<int> {
    Set::range(0, mark.len() as int).filter(|u: int| mark[u])
}

// (d) bridge: a CSR-closed mark over a CSR representing g = a forward-closed
// node set of g, the premise of selection::selected_wfm_queries.
pub proof fn represents_closed(
    g: rank::Graph,
    offsets: Seq<usize>,
    targets: Seq<usize>,
    mark: Seq<bool>,
)
    requires
        csr_wf(offsets, targets),
        mark.len() == offsets.len() - 1,
        represents(g, offsets, targets),
        csr_closed(offsets, targets, mark),
    ensures
        rank::forward_closed(g, marked(mark)),
{
    assert forall|u: int, v: int| rank::edge(g, u, v) && marked(mark).contains(u) implies marked(
        mark,
    ).contains(v) by {
        let k = choose|k: int| offsets[u] <= k < offsets[u + 1] && #[trigger] targets[k] == v;
        assert(mark[u]);
        assert(done(offsets, targets, mark, u));
        assert(offsets[u + 1] <= targets.len());
        assert(mark[targets[k] as int]);
    }
}

pub open spec fn count_false(s: Seq<bool>) -> nat
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else {
        count_false(s.drop_last()) + if s.last() {
            0nat
        } else {
            1nat
        }
    }
}

pub proof fn count_false_update(s: Seq<bool>, i: int)
    requires
        0 <= i < s.len(),
        !s[i],
    ensures
        count_false(s.update(i, true)) + 1 == count_false(s),
    decreases s.len(),
{
    let t = s.update(i, true);
    if i == s.len() - 1 {
        assert(t.drop_last() =~= s.drop_last());
    } else {
        count_false_update(s.drop_last(), i);
        assert(t.drop_last() =~= s.drop_last().update(i, true));
    }
}

// D7's slice selection, executable: forward reachability over the checked CSR
// from the query roots. Each node is pushed once (marked before its push), so
// unmarked count + stack length falls by one per pop.
pub fn reach(offsets: &Vec<usize>, targets: &Vec<usize>, roots: &Vec<usize>) -> (mark: Vec<bool>)
    requires
        csr_wf(offsets@, targets@),
        forall|i: int| 0 <= i < roots.len() ==> roots[i] < offsets.len() - 1,
    ensures
        mark.len() == offsets.len() - 1,
        forall|i: int| 0 <= i < roots.len() ==> mark[roots[i] as int],
        csr_closed(offsets@, targets@, mark@),
{
    let n = offsets.len() - 1;
    let mut mark: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            n == offsets.len() - 1,
            mark.len() == i,
            forall|j: int| 0 <= j < i ==> !mark[j],
        decreases n - i,
    {
        mark.push(false);
        i = i + 1;
    }
    let mut stack: Vec<usize> = Vec::new();
    let mut r: usize = 0;
    while r < roots.len()
        invariant
            n == offsets.len() - 1,
            mark.len() == n,
            r <= roots.len(),
            forall|i: int| 0 <= i < roots.len() ==> roots[i] < n,
            forall|i: int| 0 <= i < r ==> mark[roots[i] as int],
            forall|s: int| 0 <= s < stack.len() ==> stack[s] < n && mark[stack[s] as int],
            forall|u: int| 0 <= u < n && #[trigger] mark[u] ==> stack@.contains(u as usize),
        decreases roots.len() - r,
    {
        let v = roots[r];
        if !mark[v] {
            let ghost old_stack = stack@;
            mark.set(v, true);
            stack.push(v);
            assert forall|u: int| 0 <= u < n && #[trigger] mark[u] implies stack@.contains(
                u as usize,
            ) by {
                if u == v {
                    assert(stack@[stack.len() - 1] == v);
                } else {
                    let s = choose|s: int| 0 <= s < old_stack.len() && old_stack[s] == u as usize;
                    assert(stack@[s] == u as usize);
                }
            }
        }
        r = r + 1;
    }
    while stack.len() > 0
        invariant
            n == offsets.len() - 1,
            csr_wf(offsets@, targets@),
            mark.len() == n,
            forall|i: int| 0 <= i < roots.len() ==> mark[roots[i] as int],
            forall|s: int| 0 <= s < stack.len() ==> stack[s] < n && mark[stack[s] as int],
            forall|u: int|
                0 <= u < n && #[trigger] mark[u] ==> stack@.contains(u as usize) || done(
                    offsets@,
                    targets@,
                    mark@,
                    u,
                ),
        decreases count_false(mark@) + stack.len(),
    {
        let ghost before = count_false(mark@) + stack.len();
        let ghost full = stack@;
        let v = stack.pop().unwrap();
        assert(full =~= stack@.push(v));
        assert forall|u: int| 0 <= u < n && u != v && #[trigger] mark[u] implies stack@.contains(
            u as usize,
        ) || done(offsets@, targets@, mark@, u) by {
            if full.contains(u as usize) {
                let s = choose|s: int| 0 <= s < full.len() && full[s] == u as usize;
                assert(s < full.len() - 1);
                assert(stack@[s] == u as usize);
            }
        }
        let start = offsets[v];
        let end = offsets[v + 1];
        let mut k = start;
        while k < end
            invariant
                n == offsets.len() - 1,
                csr_wf(offsets@, targets@),
                mark.len() == n,
                v < n,
                start == offsets[v as int],
                end == offsets[v as int + 1],
                start <= k <= end,
                end <= targets.len(),
                mark[v as int],
                forall|i: int| 0 <= i < roots.len() ==> mark[roots[i] as int],
                forall|s: int| 0 <= s < stack.len() ==> stack[s] < n && mark[stack[s] as int],
                forall|u: int|
                    0 <= u < n && u != v && #[trigger] mark[u] ==> stack@.contains(u as usize)
                        || done(offsets@, targets@, mark@, u),
                forall|q: int| start <= q < k ==> mark[targets[q] as int],
                count_false(mark@) + stack.len() + 1 == before,
            decreases end - k,
        {
            let w = targets[k];
            if !mark[w] {
                proof {
                    count_false_update(mark@, w as int);
                }
                let ghost old_stack = stack@;
                mark.set(w, true);
                stack.push(w);
                assert forall|u: int|
                    0 <= u < n && u != v && #[trigger] mark[u] implies stack@.contains(u as usize)
                    || done(offsets@, targets@, mark@, u) by {
                    if u == w {
                        assert(stack@[stack.len() - 1] == w);
                    } else if old_stack.contains(u as usize) {
                        let s = choose|s: int|
                            0 <= s < old_stack.len() && old_stack[s] == u as usize;
                        assert(stack@[s] == u as usize);
                    }
                }
            }
            k = k + 1;
        }
    }
    mark
}

} // verus!
verus! {

// (c) + (d): the executable reachability's mark over a CSR representing the
// pruned program's covered graph selects SOURCE clauses whose q-all + goal
// answers equal the source program's.
pub proof fn csr_selected_wfm_queries(
    p: Program,
    l: Set<Atom>,
    g: rank::Graph,
    origin: Seq<int>,
    offsets: Seq<usize>,
    targets: Seq<usize>,
    mark: Seq<bool>,
    operators: Set<Atom>,
    valuations: Set<int>,
    goals: spec_fn(int) -> Instance,
)
    requires
        wf(p),
        envelope::envelope(p, l),
        rank::graph_wf(g),
        rank::no_negative_cycle(g),
        origins::clause_coverage(envelope::prune(p, l), g, origin),
        csr_wf(offsets, targets),
        mark.len() == offsets.len() - 1,
        represents(g, offsets, targets),
        csr_closed(offsets, targets, mark),
        operators.subset_of(origins::atom_slice(envelope::prune(p, l), origin, marked(mark))),
        forall|id: int|
            valuations.contains(id) ==> slice::reads(
                goals(id),
                origins::atom_slice(envelope::prune(p, l), origin, marked(mark)),
            ),
    ensures
        afp::lower(p) == afp::upper(p),
        slice::enumerate(afp::lower(p), operators) == slice::enumerate(
            afp::lower(selection::select(p, origin, marked(mark))),
            operators,
        ),
        slice::goal_answers(afp::lower(p), valuations, goals) == slice::goal_answers(
            afp::lower(selection::select(p, origin, marked(mark))),
            valuations,
            goals,
        ),
{
    represents_closed(g, offsets, targets, mark);
    envelope::pruned_selected_wfm_queries(
        p,
        l,
        g,
        origin,
        marked(mark),
        operators,
        valuations,
        goals,
    );
}

// Every premise of represents_closed + reach's precondition holds on a
// two-node CSR (0 -> 1) whose closed mark covers the graph's one edge.
pub proof fn csr_control() {
    let offsets = seq![0usize, 1usize, 1usize];
    let targets = seq![1usize];
    let mark = seq![true, true];
    let g = rank::Graph {
        nodes: set![0int, 1int],
        positive: set![(0int, 1int)],
        negative: Set::empty(),
    };
    assert(csr_wf(offsets, targets));
    assert forall|u: int, v: int| #[trigger] rank::edge(g, u, v) implies csr_edge(
        offsets,
        targets,
        u,
        v,
    ) by {
        assert(u == 0 && v == 1);
        assert(targets[0] == 1);
    }
    assert(represents(g, offsets, targets));
    assert(csr_closed(offsets, targets, mark));
    represents_closed(g, offsets, targets, mark);
    assert(marked(mark).contains(0));
    assert(rank::forward_closed(g, marked(mark)));
}

} // verus!
