use super::matching::{Heads, forced, state};
use super::{Budget, Failure, May, Pattern, Program, Result, bad, hash, id};
use sb1_checker::program::Goal;
use std::collections::{HashMap, VecDeque};

pub struct Csr {
    pub offsets: Vec<usize>,
    pub targets: Vec<u32>,
}
impl Csr {
    pub fn nodes(&self) -> usize {
        self.offsets.len() - 1
    }
    pub fn row(&self, node: usize) -> &[u32] {
        &self.targets[self.offsets[node]..self.offsets[node + 1]]
    }
    pub fn reach(&self, roots: &[usize]) -> Result<Vec<bool>> {
        let mut mark = vec![false; self.nodes()];
        let mut todo = Vec::new();
        for root in roots {
            if *root >= mark.len() {
                return Err(bad("query root outside CSR"));
            }
            if !mark[*root] {
                mark[*root] = true;
                todo.push(*root);
            }
        }
        while let Some(from) = todo.pop() {
            for edge in self.row(from) {
                let to = (edge >> 1) as usize;
                if !mark[to] {
                    mark[to] = true;
                    todo.push(to);
                }
            }
        }
        Ok(mark)
    }
    pub fn partition(&self) -> Vec<usize> {
        let n = self.nodes();
        let mut seen = vec![false; n];
        let mut order = Vec::with_capacity(n);
        let mut todo = Vec::new();
        for root in 0..n {
            if seen[root] {
                continue;
            }
            seen[root] = true;
            todo.push((root, self.offsets[root]));
            while let Some((node, cursor)) = todo.last_mut() {
                if *cursor == self.offsets[*node + 1] {
                    order.push(*node);
                    todo.pop();
                } else {
                    let to = (self.targets[*cursor] >> 1) as usize;
                    *cursor += 1;
                    if !seen[to] {
                        seen[to] = true;
                        todo.push((to, self.offsets[to]));
                    }
                }
            }
        }
        let mut reverse = vec![0_usize; n + 1];
        for edge in &self.targets {
            reverse[(edge >> 1) as usize + 1] += 1;
        }
        for i in 0..n {
            reverse[i + 1] += reverse[i];
        }
        let mut cursor = reverse.clone();
        let mut targets = vec![0_u32; self.targets.len()];
        for from in 0..n {
            for edge in self.row(from) {
                let to = (edge >> 1) as usize;
                targets[cursor[to]] = from as u32;
                cursor[to] += 1;
            }
        }
        let mut component = vec![usize::MAX; n];
        let mut count = 0;
        let mut stack = Vec::new();
        for root in order.into_iter().rev() {
            if component[root] != usize::MAX {
                continue;
            }
            component[root] = count;
            stack.push(root);
            while let Some(node) = stack.pop() {
                for from in &targets[reverse[node]..reverse[node + 1]] {
                    let from = *from as usize;
                    if component[from] == usize::MAX {
                        component[from] = count;
                        stack.push(from);
                    }
                }
            }
            count += 1;
        }
        component
    }
    pub fn negative_cycle(&self) -> Option<Vec<(usize, usize, bool)>> {
        let components = self.partition();
        for from in 0..self.nodes() {
            for edge in self.row(from) {
                let to = (edge >> 1) as usize;
                if edge & 1 == 0 || components[from] != components[to] {
                    continue;
                }
                let mut predecessor = vec![None; self.nodes()];
                let mut queue = VecDeque::from([to]);
                predecessor[to] = Some((to, false));
                while let Some(node) = queue.pop_front() {
                    if node == from {
                        break;
                    }
                    for e in self.row(node) {
                        let next = (e >> 1) as usize;
                        if components[next] == components[from] && predecessor[next].is_none() {
                            predecessor[next] = Some((node, e & 1 != 0));
                            queue.push_back(next);
                        }
                    }
                }
                let mut path = Vec::new();
                let mut at = from;
                while at != to {
                    let (previous, negative) = predecessor[at].expect("same SCC path");
                    path.push((previous, at, negative));
                    at = previous;
                }
                path.reverse();
                path.insert(0, (from, to, true));
                return Some(path);
            }
        }
        None
    }
    pub fn rank_check(&self, ranks: &[u64], strict_all: bool) -> Result<()> {
        if ranks.len() != self.nodes() || ranks.contains(&0) {
            return Err(bad("missing/zero rank"));
        }
        for from in 0..self.nodes() {
            for edge in self.row(from) {
                let to = (edge >> 1) as usize;
                let strict = strict_all || edge & 1 != 0;
                if if strict {
                    ranks[from] <= ranks[to]
                } else {
                    ranks[from] < ranks[to]
                } {
                    return Err(bad(format!("invalid rank {from} -> {to} strict={strict}")));
                }
            }
        }
        Ok(())
    }
}
fn body_key(goals: &[Goal], vars: usize) -> Result<Vec<u32>> {
    fn add(goals: &[Goal], key: &mut Vec<u32>) -> Result<()> {
        key.push(id(goals.len())?);
        for goal in goals {
            match goal {
                Goal::Positive(t) => key.extend([0, id(*t)?]),
                Goal::Builtin(t) => key.extend([1, id(*t)?]),
                Goal::Not(inner) => {
                    key.push(2);
                    add(inner, key)?;
                }
            }
        }
        Ok(())
    }
    let mut key = vec![id(vars)?];
    add(goals, &mut key)?;
    Ok(key)
}
// Raw enumeration is diagnostic-only: callers still need the rank/cycle gate.
pub fn enumerate(p: &Program, m: &May, budget: &mut Budget) -> Result<Csr> {
    enumerate_inner(p, m, None, budget)
}
fn enumerate_inner(
    p: &Program,
    m: &May,
    ranks: Option<&[u64]>,
    budget: &mut Budget,
) -> Result<Csr> {
    if p.clauses.len() >= (1_usize << 31) {
        return Err(Failure::Indeterminate("CSR node id space exhausted".into()));
    }
    if let Some(rank) = ranks
        && (rank.len() != p.clauses.len() || rank.contains(&0))
    {
        return Err(bad("missing/zero clause rank"));
    }
    let heads = Heads::build(p, m)?;
    let mut memo = HashMap::<u64, Vec<(Vec<u32>, usize, usize)>>::new();
    let mut csr = Csr {
        offsets: vec![0],
        targets: Vec::new(),
    };
    for (i, c) in p.clauses.iter().enumerate() {
        let key = body_key(&c.body, c.names.len())?;
        let h = hash(&key);
        let cached = memo
            .get(&h)
            .and_then(|rows| rows.iter().find(|(k, _, _)| *k == key))
            .map(|(_, a, b)| (*a, *b));
        if p.patient[i] || c.body.is_empty() {
        } else if let Some((a, b)) = cached {
            csr.targets.extend_from_within(a..b);
        } else {
            let mut row = Vec::new();
            let positive = Pattern::body(&p.arena, &m.abs, &c.body, c.names.len(), None);
            let base = state(&p.arena, m, &positive, budget)?;
            let mut targets = |pattern: &Pattern,
                               s: Option<&super::State>,
                               goal,
                               negative,
                               budget: &mut Budget|
             -> Result<()> {
                let Some(s) = s else {
                    return Ok(());
                };
                for to in heads.candidates(p, m, goal, s) {
                    if forced(p, m, pattern, goal, to, s, budget)?.is_some() {
                        #[cfg(test)]
                        if crate::faults::on("dropped-signed-edge") {
                            continue;
                        }
                        #[cfg(test)]
                        let negative = negative && !crate::faults::on("wrong-sign");
                        row.push(((to as u32) << 1) | u32::from(negative));
                    }
                }
                Ok(())
            };
            for goal in &c.body {
                match goal {
                    Goal::Positive(t) => targets(&positive, base.as_ref(), *t, false, budget)?,
                    Goal::Not(inner) => {
                        let pattern =
                            Pattern::body(&p.arena, &m.abs, &c.body, c.names.len(), Some(inner));
                        // Keep the borrow of budget outside the target closure.
                        let s = state(&p.arena, m, &pattern, budget)?;
                        for goal in inner {
                            if let Goal::Positive(t) = goal {
                                targets(&pattern, s.as_ref(), *t, true, budget)?;
                            }
                        }
                    }
                    Goal::Builtin(_) => {}
                }
            }
            row.sort_unstable();
            row.dedup();
            #[cfg(test)]
            if crate::faults::on("csr-drop") && !row.is_empty() {
                row.pop();
            }
            let a = csr.targets.len();
            csr.targets.extend(row);
            let b = csr.targets.len();
            memo.entry(h).or_default().push((key, a, b));
        }
        csr.offsets.push(csr.targets.len());
        if let Some(rank) = ranks {
            for edge in csr.row(i) {
                let to = (edge >> 1) as usize;
                if rank[i] < rank[to] || edge & 1 != 0 && rank[i] == rank[to] {
                    return Err(bad(format!(
                        "{} -> {} invalid clause rank",
                        p.label(i),
                        p.label(to)
                    )));
                }
            }
        }
        m.reset_scratch();
    }
    Ok(csr)
}
pub fn build(p: &Program, m: &May, ranks: Option<&[u64]>, budget: &mut Budget) -> Result<Csr> {
    let csr = enumerate_inner(p, m, ranks, budget)?;
    if ranks.is_none() {
        if let Some(edges) = csr.negative_cycle() {
            return Err(Failure::Cycle {
                kind: "negative",
                edges,
                sites: Vec::new(),
                labels: Vec::new(),
            });
        }
        return Err(bad("missing clause ranks on an acyclic signed graph"));
    }
    Ok(csr)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn csr_reach_covers_all_edges_and_keeps_signs() {
        let graph = Csr {
            offsets: vec![0, 2, 3, 3, 4],
            targets: vec![2, 5, 4, 7],
        };
        assert_eq!(graph.reach(&[0]).unwrap(), vec![true, true, true, false]);
        assert!(graph.reach(&[4]).is_err());
        assert!(!(graph.partition()[0] == graph.partition()[1]));
        assert!(graph.negative_cycle().is_some());
        assert!(graph.rank_check(&[3, 2, 1, 1], false).is_err());
    }
}
