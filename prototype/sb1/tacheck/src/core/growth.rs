use super::matching::{Expr, Heads, forced, state, witness};
use super::{Abs, Budget, Csr, Failure, May, Pattern, Program, Result, State, bad, hash, id};
use sb1_checker::program::Goal;
use sb1_checker::term::{Id, Node};
use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeNode {
    Scalar,
    Identity(usize),
    Arithmetic { clause: usize, goal: usize },
}
pub struct Evidence {
    pub types: Types,
    pub sources: Sources,
    pub ranks: Option<Vec<u64>>,
}
pub struct Types {
    pub nodes: Vec<TypeNode>,
    arithmetic: HashMap<u64, Vec<u32>>,
}
impl Types {
    pub fn new(p: &Program, m: &May) -> Result<Self> {
        let mut nodes = vec![TypeNode::Scalar];
        nodes.extend((0..m.abs.origins.len()).map(TypeNode::Identity));
        let mut arithmetic = HashMap::<u64, Vec<u32>>::new();
        for (c, clause) in p.clauses.iter().enumerate() {
            if p.patient[c] {
                continue;
            }
            for (g, goal) in clause.body.iter().enumerate() {
                if let Goal::Builtin(t) = goal
                    && let Some((name, _)) = p.arena.call(*t)
                    && matches!(
                        p.arena.name(name),
                        "guideline_product" | "guideline_quotient"
                    )
                {
                    let at = id(nodes.len())?;
                    nodes.push(TypeNode::Arithmetic { clause: c, goal: g });
                    arithmetic
                        .entry(hash(&[id(c)?, id(g)?]))
                        .or_default()
                        .push(at);
                }
            }
        }
        Ok(Self { nodes, arithmetic })
    }
    pub fn identity(&self, origin: usize) -> u32 {
        (origin + 1) as u32
    }
    pub fn arithmetic(&self, clause: usize, goal: usize) -> Option<u32> {
        self.arithmetic
            .get(&hash(&[clause as u32, goal as u32]))
            .and_then(|rows| {
                rows.iter()
                    .find(|r| self.nodes[**r as usize] == TypeNode::Arithmetic { clause, goal })
                    .copied()
            })
    }
    pub fn value(&self, m: &May, ty: u32) -> u32 {
        match self.nodes[ty as usize] {
            TypeNode::Scalar => Abs::PATIENT,
            TypeNode::Identity(o) => m.abs.origin_values[o],
            TypeNode::Arithmetic { .. } => Abs::NUMBER,
        }
    }
    pub fn label(&self, p: &Program, m: &May, ty: usize) -> String {
        match self.nodes[ty] {
            TypeNode::Scalar => "scalar".into(),
            TypeNode::Identity(o) => m
                .abs
                .describe(&p.arena, m.abs.origin_values[o])
                .trim_start_matches("origin:")
                .to_owned(),
            TypeNode::Arithmetic { clause, goal } => {
                format!("arithmetic({},{})", p.detail_label(clause), goal + 1)
            }
        }
    }
    pub fn source_term(
        &self,
        p: &Program,
        m: &May,
        clause: usize,
        term: Id,
        sources: &Sources,
    ) -> Result<Vec<u32>> {
        match p.arena.node(term) {
            Node::Var(v) => Ok(sources.row(clause, *v)?.types.clone()),
            _ => Ok(vec![
                m.abs
                    .origin(m.abs.terms[term])
                    .map_or(0, |o| self.identity(o)),
            ]),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Source {
    pub clause: usize,
    pub variable: usize,
    pub types: Vec<u32>,
    pub binder: Option<(usize, usize)>,
}
pub struct Sources {
    pub rows: Vec<Source>,
    by_clause: Vec<Vec<Option<usize>>>,
}
impl Sources {
    pub fn new(p: &Program, types: &Types, mut rows: Vec<Source>) -> Result<Self> {
        let mut by_clause = p
            .clauses
            .iter()
            .map(|c| vec![None; c.names.len()])
            .collect::<Vec<_>>();
        for (i, row) in rows.iter_mut().enumerate() {
            let Some(slot) = by_clause
                .get_mut(row.clause)
                .and_then(|c| c.get_mut(row.variable))
            else {
                return Err(bad("source-hint clause/variable index"));
            };
            if slot.replace(i).is_some() {
                return Err(bad("duplicate source-hint variable"));
            }
            if row.types.iter().any(|t| *t as usize >= types.nodes.len()) {
                return Err(bad("source type outside registry"));
            }
            let n = row.types.len();
            row.types.sort_unstable();
            row.types.dedup();
            if n != row.types.len() {
                return Err(bad("duplicate source type"));
            }
            if let Some((j, k)) = row.binder {
                let valid = p.clauses[row.clause]
                    .body
                    .get(j)
                    .and_then(|g| match g {
                        Goal::Positive(t) => p.arena.call(*t).and_then(|(_, a)| a.get(k)),
                        _ => None,
                    })
                    .is_some_and(
                        |t| matches!(p.arena.node(*t), Node::Var(v) if *v == row.variable),
                    );
                if !valid {
                    return Err(bad(format!(
                        "{} invalid source binder",
                        p.label(row.clause)
                    )));
                }
            }
        }
        for (i, c) in p.clauses.iter().enumerate() {
            if p.patient[i] {
                continue;
            }
            let mut vars = BTreeSet::new();
            p.arena.vars(c.head, &mut vars);
            for v in vars {
                if by_clause[i][v].is_none() {
                    return Err(bad(format!(
                        "{} missing source hint for variable {v}",
                        p.label(i)
                    )));
                }
            }
        }
        Ok(Self { rows, by_clause })
    }
    pub fn row(&self, clause: usize, variable: usize) -> Result<&Source> {
        self.by_clause
            .get(clause)
            .and_then(|c| c.get(variable))
            .copied()
            .flatten()
            .map(|i| &self.rows[i])
            .ok_or_else(|| {
                bad(format!(
                    "missing forwarding source at clause {clause}, variable {variable}"
                ))
            })
    }
    pub fn fits(
        p: &Program,
        m: &May,
        types: &Types,
        clause: usize,
        variable: usize,
        ty: u32,
        budget: &mut Budget,
    ) -> Result<bool> {
        let c = &p.clauses[clause];
        let pattern = Pattern::body(&p.arena, &m.abs, &c.body, c.names.len(), None);
        let mut s = State::new(c.names.len());
        if !s.constrain(Expr::Var(variable), m.sets.one(types.value(m, ty)), m) {
            return Ok(false);
        }
        witness(&p.arena, m, &pattern, s, budget)
    }
    pub fn complete_scalars(
        &mut self,
        p: &Program,
        m: &May,
        types: &Types,
        budget: &mut Budget,
    ) -> Result<()> {
        for row in &mut self.rows {
            if row.types.binary_search(&0).is_err()
                && Self::fits(p, m, types, row.clause, row.variable, 0, budget)?
            {
                row.types.insert(0, 0);
            }
            m.reset_scratch();
        }
        Ok(())
    }
    pub fn check(&self, p: &Program, m: &May, types: &Types, budget: &mut Budget) -> Result<()> {
        let heads = Heads::build(p, m)?;
        for row in &self.rows {
            let c = &p.clauses[row.clause];
            let pattern = Pattern::body(&p.arena, &m.abs, &c.body, c.names.len(), None);
            let base = state(&p.arena, m, &pattern, budget)?;
            if base.is_none() {
                m.reset_scratch();
                continue;
            }
            if let Some((j, k)) = row.binder {
                let Goal::Positive(goal) = c.body[j] else {
                    unreachable!()
                };
                for producer in heads.candidates(p, m, goal, base.as_ref().unwrap()) {
                    if forced(
                        p,
                        m,
                        &pattern,
                        goal,
                        producer,
                        base.as_ref().unwrap(),
                        budget,
                    )?
                    .is_none()
                    {
                        continue;
                    }
                    let (_, a) = p.arena.call(p.clauses[producer].head).unwrap();
                    for ty in types.source_term(p, m, producer, a[k], self)? {
                        if row.types.binary_search(&ty).is_err()
                            && Self::fits(p, m, types, row.clause, row.variable, ty, budget)?
                        {
                            return Err(bad(format!(
                                "{} variable {} missing forwarding source from {} type {ty}",
                                p.label(row.clause),
                                row.variable,
                                p.label(producer)
                            )));
                        }
                    }
                }
            } else {
                // Arithmetic outputs are checked operationally here; the Verus
                // forwarding seam covers flat positive binders alone.
                let mut bound_by_output = false;
                for (g, goal) in c.body.iter().enumerate() {
                    if let Goal::Builtin(t) = goal
                        && let Some((name, a)) = p.arena.call(*t)
                        && matches!(
                            p.arena.name(name),
                            "guideline_product" | "guideline_quotient"
                        )
                    {
                        for (at, term) in a.iter().enumerate().skip(4) {
                            if matches!(p.arena.node(*term), Node::Var(v) if *v == row.variable) {
                                bound_by_output = true;
                                let ty = if at == 5 {
                                    types
                                        .arithmetic(row.clause, g)
                                        .expect("arithmetic output type")
                                } else {
                                    0
                                };
                                if row.types.binary_search(&ty).is_err()
                                    && Self::fits(
                                        p,
                                        m,
                                        types,
                                        row.clause,
                                        row.variable,
                                        ty,
                                        budget,
                                    )?
                                {
                                    return Err(bad(format!(
                                        "{} missing arithmetic output source",
                                        p.label(row.clause)
                                    )));
                                }
                            }
                        }
                    }
                }
                if !bound_by_output {
                    return Err(Failure::Indeterminate(format!(
                        "{} nested/nonflat source binder is outside TAS seam",
                        p.label(row.clause)
                    )));
                }
            }
            m.reset_scratch();
        }
        Ok(())
    }
}
fn constructors(p: &Program, term: Id, out: &mut Vec<Id>) {
    if p.arena.named(term, "$guideline_id", 5).is_some() {
        out.push(term);
    } else if let Some((_, a)) = p.arena.call(term) {
        for t in a {
            constructors(p, *t, out);
        }
    }
}

// Diagnostic graph only. Acceptance always calls `graph`, including ranks.
pub fn enumerate(
    p: &Program,
    m: &May,
    types: &Types,
    sources: &Sources,
    budget: &mut Budget,
) -> Result<Csr> {
    let mut rows = vec![Vec::<u32>::new(); types.nodes.len()];
    for (i, c) in p.clauses.iter().enumerate() {
        if p.patient[i] {
            continue;
        }
        let mut terms = Vec::new();
        constructors(p, c.head, &mut terms);
        for term in terms {
            budget.tick()?;
            let a = p.arena.named(term, "$guideline_id", 5).unwrap();
            let deps = p
                .arena
                .list(a[4])
                .ok_or_else(|| bad("constructor Deps list"))?;
            let from = types.identity(m.abs.origin(m.abs.terms[term]).expect("constructor origin"));
            let variables = deps
                .into_iter()
                .map(|t| match p.arena.node(t) {
                    Node::Var(v) => Ok(*v),
                    _ => Err(bad("nonvariable constructor Deps")),
                })
                .collect::<Result<Vec<_>>>()?;
            let choices = variables
                .iter()
                .map(|v| sources.row(i, *v).map(|r| r.types.as_slice()))
                .collect::<Result<Vec<_>>>()?;
            if choices.iter().any(|v| v.is_empty()) {
                continue;
            }
            for choices in choices {
                rows[from as usize].extend(choices.iter().map(|t| (t << 1) | 1));
            }
        }
        for (g, goal) in c.body.iter().enumerate() {
            let Some(from) = types.arithmetic(i, g) else {
                continue;
            };
            let Goal::Builtin(t) = goal else {
                unreachable!()
            };
            let (_, a) = p.arena.call(*t).unwrap();
            for input in [a[1], a[3]] {
                let mut vars = BTreeSet::new();
                p.arena.vars(input, &mut vars);
                for v in vars {
                    rows[from as usize]
                        .extend(sources.row(i, v)?.types.iter().map(|t| (t << 1) | 1));
                }
            }
        }
    }
    #[cfg(test)]
    if crate::faults::on("complement-growth-edge")
        && let Some(row) = rows
            .iter_mut()
            .rev()
            .find(|r| r.iter().any(|e| e >> 1 != 0))
    {
        row.retain(|edge| edge >> 1 == 0);
    }
    let mut graph = Csr {
        offsets: vec![0],
        targets: Vec::new(),
    };
    for mut row in rows {
        row.sort_unstable();
        row.dedup();
        graph.targets.extend(row);
        graph.offsets.push(graph.targets.len());
    }
    Ok(graph)
}
pub fn graph(
    p: &Program,
    m: &May,
    types: &Types,
    sources: &Sources,
    ranks: Option<&[u64]>,
    budget: &mut Budget,
) -> Result<Csr> {
    if let Some(ranks) = ranks
        && (ranks.len() != types.nodes.len() || ranks[0] != 0 || ranks[1..].contains(&0))
    {
        return Err(bad("missing/zero growth rank"));
    }
    let graph = enumerate(p, m, types, sources, budget)?;
    if let Some(ranks) = ranks {
        for from in 0..graph.nodes() {
            for edge in graph.row(from) {
                let to = (edge >> 1) as usize;
                if ranks[from] <= ranks[to] {
                    return Err(bad(format!("invalid strict growth rank {from} -> {to}")));
                }
            }
        }
    } else {
        if let Some(edges) = graph.negative_cycle() {
            let mut sites = Vec::new();
            for (from, to, _) in &edges {
                let mut support = None;
                for (i, c) in p.clauses.iter().enumerate() {
                    if p.patient[i] {
                        continue;
                    }
                    let mut terms = Vec::new();
                    constructors(p, c.head, &mut terms);
                    for term in terms {
                        if types.identity(m.abs.origin(m.abs.terms[term]).unwrap()) as usize
                            != *from
                        {
                            continue;
                        }
                        let a = p.arena.named(term, "$guideline_id", 5).unwrap();
                        for t in p.arena.list(a[4]).unwrap() {
                            if let Node::Var(v) = p.arena.node(t)
                                && sources
                                    .row(i, *v)?
                                    .types
                                    .binary_search(&(*to as u32))
                                    .is_ok()
                            {
                                support = Some((i, *v));
                                break;
                            }
                        }
                        if support.is_some() {
                            break;
                        }
                    }
                    if let TypeNode::Arithmetic { clause, goal } = types.nodes[*from]
                        && clause == i
                    {
                        let Goal::Builtin(t) = c.body[goal] else {
                            unreachable!()
                        };
                        let (_, a) = p.arena.call(t).unwrap();
                        for input in [a[1], a[3]] {
                            let mut vars = BTreeSet::new();
                            p.arena.vars(input, &mut vars);
                            for v in vars {
                                if sources
                                    .row(i, v)?
                                    .types
                                    .binary_search(&(*to as u32))
                                    .is_ok()
                                {
                                    support = Some((i, v));
                                    break;
                                }
                            }
                        }
                    }
                    if support.is_some() {
                        break;
                    }
                }
                sites.push(
                    support.ok_or_else(|| bad("growth cycle has no original supporting clause"))?,
                );
            }
            let labels = edges
                .iter()
                .map(|(from, to, _)| (types.label(p, m, *from), types.label(p, m, *to)))
                .collect();
            return Err(Failure::Cycle {
                kind: "growth",
                edges,
                sites,
                labels,
            });
        }
        return Err(bad("missing growth ranks on an acyclic graph"));
    }
    Ok(graph)
}
