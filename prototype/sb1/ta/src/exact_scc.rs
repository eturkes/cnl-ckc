//! Exact signed caller→hub→target storage. Auxiliary arcs never carry a sign.
use crate::domain::Counters;
use crate::graph::{Edge, Graph, Witness};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Association {
    hub: usize,
    negative: bool,
    growth: bool,
}

pub struct Builder {
    targets: Vec<Rc<[usize]>>,
    target_ids: HashMap<Rc<[usize]>, usize>,
    rows: Vec<Rc<[Association]>>,
    row_ids: HashMap<Rc<[Association]>, usize>,
    row_counts: Vec<usize>,
    callers: Vec<usize>,
}
impl Builder {
    pub fn new(clauses: usize) -> Self {
        let mut out = Self {
            targets: Vec::new(),
            target_ids: HashMap::new(),
            rows: Vec::new(),
            row_ids: HashMap::new(),
            row_counts: Vec::new(),
            callers: vec![0; clauses],
        };
        assert_eq!(out.row(Vec::new()), 0);
        out
    }
    fn hub(&mut self, mut targets: Vec<usize>) -> usize {
        targets.sort_unstable();
        targets.dedup();
        #[cfg(test)]
        if !targets.is_empty() && crate::faults::take(crate::faults::Kind::HubTarget) {
            targets.pop();
        }
        if let Some(id) = self.target_ids.get(targets.as_slice()) {
            return *id;
        }
        let id = self.targets.len();
        let targets = Rc::<[usize]>::from(targets);
        self.target_ids.insert(targets.clone(), id);
        self.targets.push(targets);
        id
    }
    fn row(&mut self, mut row: Vec<Association>) -> usize {
        row.sort_unstable();
        row.dedup();
        if let Some(id) = self.row_ids.get(row.as_slice()) {
            return *id;
        }
        let count = expand(&row, &self.targets).len();
        let id = self.rows.len();
        let row = Rc::<[Association]>::from(row);
        self.row_ids.insert(row.clone(), id);
        self.rows.push(row);
        self.row_counts.push(count);
        id
    }
    pub fn calls(&mut self, calls: Vec<(bool, Vec<usize>)>) -> usize {
        let row = calls
            .into_iter()
            .filter(|(_, targets)| !targets.is_empty())
            .map(|(negative, targets)| {
                #[cfg(test)]
                let negative = negative && !crate::faults::take(crate::faults::Kind::Sign);
                Association {
                    hub: self.hub(targets),
                    negative,
                    growth: false,
                }
            })
            .collect();
        self.row(row)
    }
    pub fn growth(&mut self, targets: Vec<usize>) -> usize {
        let row = if targets.is_empty() {
            Vec::new()
        } else {
            vec![Association {
                hub: self.hub(targets),
                negative: false,
                growth: true,
            }]
        };
        self.row(row)
    }
    pub fn attach(&mut self, caller: usize, row: usize) {
        self.callers[caller] = row;
    }
    pub fn finish(self, counters: Counters) -> Compressed {
        let n = self.callers.len();
        let mut edges = self
            .callers
            .iter()
            .map(|r| {
                self.rows[*r]
                    .iter()
                    .map(|a| Edge {
                        to: n + a.hub,
                        negative: false,
                        growth: false,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        edges.extend(self.targets.iter().map(|row| {
            row.iter()
                .map(|to| Edge {
                    to: *to,
                    negative: false,
                    growth: false,
                })
                .collect::<Vec<_>>()
        }));
        // Unsigned auxiliary SCCs decide reachability alone. A signed caller
        // association is cyclic iff its caller and hub share that component.
        let aux = Graph::from_edges(edges, Counters::default());
        let cyclic = self.callers.iter().enumerate().any(|(from, row)| {
            self.rows[*row].iter().any(|a| {
                (a.negative || a.growth) && aux.components[from] == aux.components[n + a.hub]
            })
        });
        let mut remap = HashMap::new();
        let mut sizes = Vec::new();
        let components = aux.components[..n]
            .iter()
            .map(|c| {
                let next = remap.len();
                let id = *remap.entry(*c).or_insert_with(|| {
                    sizes.push(0);
                    next
                });
                sizes[id] += 1;
                id
            })
            .collect();
        let aux_nodes = aux.edges.len();
        let aux_arcs = aux.edge_count();
        drop(aux);
        let mut out = Compressed {
            targets: self.targets,
            rows: self.rows,
            callers: self.callers,
            row_counts: self.row_counts,
            components,
            sizes,
            witnesses: Vec::new(),
            counters,
            body_patterns: 0,
            body_cache_hits: 0,
            aux_nodes,
            aux_arcs,
        };
        if cyclic {
            // Refusal witnesses count original transitions, not virtual hops.
            // Reuse the oracle's original-edge shortest-cycle rule only here.
            let native =
                Graph::from_edges((0..n).map(|i| out.row(i)).collect(), out.counters.clone());
            out.components = native.components;
            out.sizes = native.sizes;
            out.witnesses = native.witnesses;
        }
        out
    }
}
fn expand(row: &[Association], targets: &[Rc<[usize]>]) -> Vec<Edge> {
    let mut out = row
        .iter()
        .flat_map(|a| {
            targets[a.hub].iter().map(|to| Edge {
                to: *to,
                negative: a.negative,
                growth: a.growth,
            })
        })
        .collect::<Vec<_>>();
    out.sort_unstable();
    out.dedup();
    out
}

pub struct Compressed {
    targets: Vec<Rc<[usize]>>,
    rows: Vec<Rc<[Association]>>,
    callers: Vec<usize>,
    row_counts: Vec<usize>,
    pub components: Vec<usize>,
    pub sizes: Vec<usize>,
    pub witnesses: Vec<Witness>,
    pub counters: Counters,
    pub body_patterns: usize,
    pub body_cache_hits: usize,
    pub aux_nodes: usize,
    pub aux_arcs: usize,
}
impl Compressed {
    pub fn from_edges(edges: Vec<Vec<Edge>>, counters: Counters) -> Self {
        let mut builder = Builder::new(edges.len());
        for (from, row) in edges.into_iter().enumerate() {
            let mut groups = BTreeMap::<(bool, bool), Vec<usize>>::new();
            for e in row {
                groups.entry((e.negative, e.growth)).or_default().push(e.to);
            }
            let row = groups
                .into_iter()
                .filter(|(_, targets)| !targets.is_empty())
                .map(|((negative, growth), targets)| Association {
                    hub: builder.hub(targets),
                    negative,
                    growth,
                })
                .collect();
            let row = builder.row(row);
            builder.attach(from, row);
        }
        builder.finish(counters)
    }
    pub fn row(&self, original: usize) -> Vec<Edge> {
        expand(&self.rows[self.callers[original]], &self.targets)
    }
    pub fn edge_count(&self) -> usize {
        self.callers.iter().map(|r| self.row_counts[*r]).sum()
    }
    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.rows[self.callers[from]]
            .iter()
            .any(|a| self.targets[a.hub].binary_search(&to).is_ok())
    }
    pub fn admitted(&self) -> bool {
        self.witnesses.is_empty()
    }
    pub fn reachable(&self, roots: &BTreeSet<usize>) -> BTreeSet<usize> {
        let n = self.callers.len();
        let mut seen = vec![false; n + self.targets.len()];
        let mut stack = Vec::new();
        for root in roots {
            seen[*root] = true;
            stack.push(*root);
        }
        while let Some(node) = stack.pop() {
            if node < n {
                for a in self.rows[self.callers[node]].iter() {
                    let next = n + a.hub;
                    if !seen[next] {
                        seen[next] = true;
                        stack.push(next);
                    }
                }
            } else {
                for next in self.targets[node - n].iter() {
                    if !seen[*next] {
                        seen[*next] = true;
                        stack.push(*next);
                    }
                }
            }
        }
        seen[..n]
            .iter()
            .enumerate()
            .filter_map(|(i, yes)| yes.then_some(i))
            .collect()
    }
    pub fn metrics(&self) -> Vec<(&'static str, usize)> {
        let associations = self
            .callers
            .iter()
            .map(|r| self.rows[*r].len())
            .sum::<usize>();
        let leaves = self.targets.iter().map(|r| r.len()).sum::<usize>();
        vec![
            ("graph_hubs", self.targets.len()),
            ("graph_rows", self.rows.len()),
            ("graph_caller_associations", associations),
            ("graph_leaf_memberships", leaves),
            ("graph_H", associations + leaves),
            ("graph_aux_nodes", self.aux_nodes),
            ("graph_aux_arcs", self.aux_arcs),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn partition(components: &[usize]) -> Vec<Vec<usize>> {
        let mut groups = BTreeMap::<usize, Vec<usize>>::new();
        for (i, c) in components.iter().enumerate() {
            groups.entry(*c).or_default().push(i);
        }
        let mut groups = groups.into_values().collect::<Vec<_>>();
        groups.sort();
        groups
    }
    fn compare(mut edges: Vec<Vec<Edge>>) {
        for row in &mut edges {
            row.sort_unstable();
            row.dedup();
        }
        let actual = Compressed::from_edges(edges.clone(), Counters::default());
        let expected = Graph::from_edges(edges.clone(), Counters::default());
        for (i, row) in edges.iter().enumerate() {
            assert_eq!(actual.row(i), *row);
        }
        assert_eq!(actual.edge_count(), expected.edge_count());
        assert_eq!(
            partition(&actual.components),
            partition(&expected.components)
        );
        assert_eq!(actual.admitted(), expected.admitted());
        for (a, b) in actual.witnesses.iter().zip(&expected.witnesses) {
            assert_eq!(a.kind, b.kind);
            assert_eq!(a.cycle, b.cycle);
        }
        assert_eq!(actual.witnesses.len(), expected.witnesses.len());
        for i in 0..edges.len() {
            let roots = BTreeSet::from([i]);
            assert_eq!(actual.reachable(&roots), expected.reachable(&roots));
        }
    }
    #[test]
    fn compressed_scc_reachability_and_minimal_original_witnesses_equal_native() {
        let mut rng = 0xe927_481a_7369_1245u64;
        let mut next = |n| {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng as usize % n
        };
        for _ in 0..1500 {
            let n = 1 + next(8);
            let mut edges = vec![Vec::new(); n];
            for row in &mut edges {
                for to in 0..n {
                    if next(4) == 0 {
                        row.push(Edge {
                            to,
                            negative: next(4) == 0,
                            growth: next(5) == 0,
                        });
                    }
                }
            }
            compare(edges);
        }
        compare(Vec::new());
    }
    #[test]
    fn shared_hub_keeps_positive_and_negative_caller_signs_separate() {
        let e = |to, negative| Edge {
            to,
            negative,
            growth: false,
        };
        let edges = vec![vec![e(2, false)], vec![e(2, true)], vec![e(1, false)]];
        let compressed = Compressed::from_edges(edges.clone(), Counters::default());
        assert_eq!(compressed.targets.len(), 2);
        assert_eq!(compressed.metrics()[2].1, 3);
        compare(edges);
    }
}
