use super::model::Feature;
use super::{ALL, Abs, EMPTY, Failure, May, Program, Result, SetId, bad, id};
use sb1_checker::program::{Goal, builtin};
use sb1_checker::term::{Arena, Id, Node};
use std::collections::HashMap;

pub struct Budget {
    pub used: u64,
    pub limit: u64,
}
impl Default for Budget {
    fn default() -> Self {
        Self {
            used: 0,
            limit: u64::MAX,
        }
    }
}
impl Budget {
    pub fn tick(&mut self) -> Result<()> {
        if self.used >= self.limit {
            return Err(Failure::Indeterminate("may-match search exhausted".into()));
        }
        self.used += 1;
        Ok(())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Expr {
    Var(usize),
    Value(u32),
}
#[derive(Clone)]
pub struct Link {
    pub owner: Expr,
    pub source: Expr,
    pub target: Expr,
    pub predicate: u32,
    pub selector: u32,
}
#[derive(Clone)]
pub struct Pattern {
    pub goals: Vec<Id>,
    pub vars: usize,
    pub unary: Vec<(Expr, Feature)>,
    pub links: Vec<Link>,
}
impl Pattern {
    pub fn expr(arena: &Arena, abs: &Abs, t: Id) -> Expr {
        match arena.node(t) {
            Node::Var(v) => Expr::Var(*v),
            _ => Expr::Value(abs.terms[t]),
        }
    }
    pub fn literal(arena: &Arena, abs: &Abs, t: Id) -> (Vec<(Expr, Feature)>, Vec<Link>) {
        if builtin(arena, t) {
            return (Vec::new(), Vec::new());
        }
        let (pred, a) = arena.call(t).expect("admitted literal");
        let name = arena.name(pred);
        let mut unary = Vec::new();
        let mut links = Vec::new();
        let expr = |t| Self::expr(arena, abs, t);
        if matches!(name, "$sb1_type" | "$sb1_origin") {
            unary.push((
                expr(a[0]),
                Feature {
                    kind: 5,
                    predicate: 0,
                    selector: abs.terms[a[1]],
                },
            ));
            return (unary, links);
        }
        let own = match name {
            "guideline_entity" => Some(0),
            "guideline_event" => Some(1),
            "guideline_operator" => Some(2),
            "guideline_property" => Some(3),
            _ => None,
        };
        if let Some(kind) = own {
            unary.push((
                expr(a[1]),
                Feature {
                    kind,
                    predicate: 0,
                    selector: abs.terms[a[2]],
                },
            ));
        }
        if !matches!(
            name,
            "guideline_schema_version"
                | "guideline_document"
                | "guideline_supersession"
                | "guideline_exclusion"
                | "guideline_evaluation_date"
        ) {
            unary.push((
                expr(a[0]),
                Feature {
                    kind: 4,
                    predicate: pred as u32,
                    selector: own.map_or(Abs::ANY, |_| abs.terms[a[2]]),
                },
            ));
        }
        if matches!(name, "guideline_arg" | "guideline_pp") {
            links.push(Link {
                owner: expr(a[0]),
                source: expr(a[1]),
                target: expr(a[3]),
                predicate: pred as u32,
                selector: abs.terms[a[2]],
            });
        }
        (unary, links)
    }
    pub fn new(arena: &Arena, abs: &Abs, goals: Vec<Id>, vars: usize) -> Self {
        let mut unary = Vec::new();
        let mut links = Vec::new();
        for t in &goals {
            let (u, l) = Self::literal(arena, abs, *t);
            unary.extend(u);
            links.extend(l);
        }
        Self {
            goals,
            vars,
            unary,
            links,
        }
    }
    pub fn body(
        arena: &Arena,
        abs: &Abs,
        goals: &[Goal],
        vars: usize,
        naf: Option<&[Goal]>,
    ) -> Self {
        let mut terms = goals
            .iter()
            .filter_map(|g| match g {
                Goal::Positive(t) | Goal::Builtin(t) => Some(*t),
                _ => None,
            })
            .collect::<Vec<_>>();
        if let Some(inner) = naf {
            terms.extend(inner.iter().filter_map(|g| match g {
                Goal::Positive(t) | Goal::Builtin(t) => Some(*t),
                _ => None,
            }));
        }
        Self::new(arena, abs, terms, vars)
    }
}
#[derive(Clone, Debug)]
pub struct State {
    pub parent: Vec<usize>,
    pub domains: Vec<SetId>,
}
impl State {
    pub fn new(vars: usize) -> Self {
        Self {
            parent: (0..vars).collect(),
            domains: vec![ALL; vars],
        }
    }
    fn root(&self, mut v: usize) -> usize {
        while self.parent[v] != v {
            v = self.parent[v];
        }
        v
    }
    pub fn domain(&self, e: Expr, m: &May) -> SetId {
        match e {
            Expr::Var(v) => self.domains[self.root(v)],
            Expr::Value(v) => m.sets.one(v),
        }
    }
    pub fn constrain(&mut self, e: Expr, allowed: SetId, m: &May) -> bool {
        let narrowed = m.sets.meet(self.domain(e, m), allowed, &m.abs);
        if let Expr::Var(v) = e {
            let r = self.root(v);
            self.domains[r] = narrowed;
        }
        narrowed != EMPTY
    }
    fn unify(&mut self, a: Expr, b: Expr, m: &May) -> bool {
        if let (Expr::Var(a), Expr::Var(b)) = (a, b) {
            let (a, b) = (self.root(a), self.root(b));
            if a != b {
                let d = m.sets.meet(self.domains[a], self.domains[b], &m.abs);
                if d == EMPTY {
                    return false;
                }
                self.parent[b] = a;
                self.domains[a] = d;
            }
            true
        } else {
            self.constrain(a, self.domain(b, m), m) && self.constrain(b, self.domain(a, m), m)
        }
    }
    fn limit(&mut self, e: Expr, allowed: SetId, m: &May) -> Option<bool> {
        let old = self.domain(e, m);
        self.constrain(e, allowed, m)
            .then(|| old != self.domain(e, m))
    }
}
pub fn solve(
    arena: &Arena,
    m: &May,
    pattern: &Pattern,
    mut s: State,
    budget: &mut Budget,
) -> Result<Option<State>> {
    budget.tick()?;
    if !m.generic
        && pattern.goals.iter().any(|t| {
            let (p, _) = arena.call(*t).unwrap();
            !builtin(arena, *t)
                && arena.name(p) != "$sb1_type"
                && m.predicates.binary_search(&(p as u32)).is_err()
        })
    {
        return Ok(None);
    }
    let mut order = (0..pattern.unary.len()).collect::<Vec<_>>();
    order.sort_unstable_by_key(|i| match pattern.unary[*i].0 {
        Expr::Value(_) => 0,
        _ => m.unary_width(pattern.unary[*i].1),
    });
    for i in order {
        let (term, feature) = pattern.unary[i];
        if let Expr::Value(v) = term {
            if !m.has(arena, v, feature) {
                return Ok(None);
            }
        } else {
            let old = s.domain(term, m);
            let allowed = if feature.kind != 5
                && let Some(values) = m.sets.values(old)
                && (m.generic
                    || !values
                        .iter()
                        .any(|v| matches!(*v, Abs::ANY | Abs::PATIENT | Abs::NUMBER)))
            {
                m.sets.intern(
                    values
                        .iter()
                        .copied()
                        .filter(|v| m.has(arena, *v, feature))
                        .collect(),
                )
            } else {
                m.sources(arena, feature)
            };
            if !s.constrain(term, allowed, m) {
                return Ok(None);
            }
        }
    }
    loop {
        let mut changed = false;
        for link in &pattern.links {
            let source = s.domain(link.source, m);
            if source == ALL {
                continue;
            }
            let owner = s.domain(link.owner, m);
            let target = s.domain(link.target, m);
            let (o, src, t) = m.support(link.predicate, link.selector, owner, source, target);
            let Some(a) = s.limit(link.source, src, m) else {
                return Ok(None);
            };
            let Some(b) = s.limit(link.target, t, m) else {
                return Ok(None);
            };
            let Some(c) = s.limit(link.owner, o, m) else {
                return Ok(None);
            };
            changed |= a || b || c;
        }
        if !changed {
            return Ok(Some(s));
        }
        budget.tick()?;
    }
}
pub fn state(
    arena: &Arena,
    m: &May,
    pattern: &Pattern,
    budget: &mut Budget,
) -> Result<Option<State>> {
    let result = solve(arena, m, pattern, State::new(pattern.vars), budget);
    #[cfg(test)]
    if crate::faults::on("exhaustion-as-no-edge")
        && matches!(result, Err(Failure::Indeterminate(_)))
    {
        return Ok(None);
    }
    result
}
pub fn witness(
    arena: &Arena,
    m: &May,
    pattern: &Pattern,
    s: State,
    budget: &mut Budget,
) -> Result<bool> {
    let Some(s) = solve(arena, m, pattern, s, budget)? else {
        return Ok(false);
    };
    let choice = (0..pattern.vars)
        .filter_map(|v| {
            let set = s.domain(Expr::Var(v), m);
            (set != ALL && m.sets.len(set) > 1).then_some((v, set))
        })
        .min_by_key(|(_, set)| m.sets.len(*set));
    let Some((var, set)) = choice else {
        return Ok(true);
    };
    for value in m.sets.values(set).unwrap().iter() {
        let mut branch = s.clone();
        if branch.constrain(Expr::Var(var), m.sets.one(*value), m)
            && witness(arena, m, pattern, branch, budget)?
        {
            return Ok(true);
        }
    }
    Ok(false)
}
pub fn forced(
    p: &Program,
    m: &May,
    pattern: &Pattern,
    goal: Id,
    producer: usize,
    base: &State,
    budget: &mut Budget,
) -> Result<Option<State>> {
    budget.tick()?;
    let c = &p.clauses[producer];
    let (gp, ga) = p.arena.call(goal).unwrap();
    let (hp, ha) = p.arena.call(c.head).unwrap();
    if gp != hp || ga.len() != ha.len() {
        return Ok(None);
    }
    let mut s = base.clone();
    let offset = s.parent.len();
    s.parent.extend(offset..offset + c.names.len());
    s.domains.resize(s.parent.len(), ALL);
    for (g, h) in ga.iter().zip(ha) {
        let a = Pattern::expr(&p.arena, &m.abs, *g);
        let b = match Pattern::expr(&p.arena, &m.abs, *h) {
            Expr::Var(v) => Expr::Var(offset + v),
            v => v,
        };
        if !s.unify(a, b, m) {
            return Ok(None);
        }
    }
    let Some(s) = solve(&p.arena, m, pattern, s, budget)? else {
        return Ok(None);
    };
    if witness(&p.arena, m, pattern, s.clone(), budget)? {
        Ok(Some(s))
    } else {
        Ok(None)
    }
}

#[derive(Default)]
struct Column {
    by_value: HashMap<u32, Vec<u32>>,
    exact: Vec<u32>,
}
struct Group {
    predicate: u32,
    arity: usize,
    rows: Vec<u32>,
    columns: Vec<Column>,
}
pub struct Heads {
    groups: Vec<Group>,
    by_predicate: HashMap<u32, Vec<u32>>,
}
impl Heads {
    pub fn build(p: &Program, m: &May) -> Result<Self> {
        let mut out = Self {
            groups: Vec::new(),
            by_predicate: HashMap::new(),
        };
        for (i, c) in p.clauses.iter().enumerate() {
            if m.generic && p.patient[i] {
                continue;
            }
            let (pred, args) = p.arena.call(c.head).unwrap();
            let pred = id(pred)?;
            let group = out
                .by_predicate
                .get(&pred)
                .and_then(|rows| {
                    rows.iter()
                        .find(|g| out.groups[**g as usize].arity == args.len())
                })
                .copied();
            let group = if let Some(g) = group {
                g
            } else {
                let g = id(out.groups.len())?;
                out.groups.push(Group {
                    predicate: pred,
                    arity: args.len(),
                    rows: Vec::new(),
                    columns: (0..args.len()).map(|_| Column::default()).collect(),
                });
                out.by_predicate.entry(pred).or_default().push(g);
                g
            };
            let group = &mut out.groups[group as usize];
            let i = id(i)?;
            group.rows.push(i);
            for (t, col) in args.iter().zip(&mut group.columns) {
                let v = m.abs.terms[*t];
                col.by_value.entry(v).or_default().push(i);
                if m.abs.exact(v).is_some() {
                    col.exact.push(i);
                }
            }
        }
        Ok(out)
    }
    pub fn candidates(&self, p: &Program, m: &May, goal: Id, s: &State) -> Vec<usize> {
        let (pred, args) = p.arena.call(goal).unwrap();
        let group = self.by_predicate.get(&(pred as u32)).and_then(|gs| {
            gs.iter().find_map(|g| {
                let g = &self.groups[*g as usize];
                (g.predicate == pred as u32 && g.arity == args.len()).then_some(g)
            })
        });
        let Some(group) = group else {
            return Vec::new();
        };
        let mut best: Option<(usize, Vec<&[u32]>)> = None;
        for (t, col) in args.iter().zip(&group.columns) {
            let set = s.domain(Pattern::expr(&p.arena, &m.abs, *t), m);
            let Some(values) = m.sets.values(set) else {
                continue;
            };
            if values.binary_search(&Abs::ANY).is_ok() {
                continue;
            }
            let mut buckets = Vec::new();
            if let Some(rows) = col.by_value.get(&Abs::ANY) {
                buckets.push(rows.as_slice());
            }
            for v in values.iter() {
                if let Some(rows) = col.by_value.get(v) {
                    buckets.push(rows.as_slice());
                }
                if matches!(*v, Abs::PATIENT | Abs::NUMBER) {
                    buckets.push(col.exact.as_slice());
                }
                for broad in [Abs::PATIENT, Abs::NUMBER] {
                    if m.abs.intersect(*v, broad).is_some()
                        && let Some(rows) = col.by_value.get(&broad)
                    {
                        buckets.push(rows.as_slice());
                    }
                }
            }
            let width = buckets.iter().map(|r| r.len()).sum();
            if best.as_ref().is_none_or(|(old, _)| width < *old) {
                best = Some((width, buckets));
            }
        }
        let mut rows = best.map_or_else(
            || group.rows.clone(),
            |(_, buckets)| buckets.into_iter().flatten().copied().collect(),
        );
        rows.sort_unstable();
        rows.dedup();
        rows.retain(|i| {
            let (_, head) = p.arena.call(p.clauses[*i as usize].head).unwrap();
            args.iter().zip(head).all(|(g, h)| {
                let domain = s.domain(Pattern::expr(&p.arena, &m.abs, *g), m);
                let v = m.abs.terms[*h];
                v == Abs::ANY
                    || domain == ALL
                    || m.sets
                        .values(domain)
                        .unwrap()
                        .iter()
                        .any(|a| m.abs.intersect(*a, v).is_some())
            })
        });
        #[cfg(test)]
        {
            if crate::faults::on("missing-variable-head") {
                rows.retain(|i| {
                    let (_, args) = p.arena.call(p.clauses[*i as usize].head).unwrap();
                    !args
                        .iter()
                        .any(|t| matches!(p.arena.node(*t), Node::Var(_)))
                });
            }
            if crate::faults::on("missing-top-compatible-head") {
                rows.retain(|i| {
                    let (_, args) = p.arena.call(p.clauses[*i as usize].head).unwrap();
                    args.iter()
                        .any(|t| matches!(p.arena.node(*t), Node::Var(_)))
                });
            }
        }
        rows.into_iter().map(|i| i as usize).collect()
    }
}

pub fn check_envelope(p: &Program, m: &May, budget: &mut Budget) -> Result<()> {
    for (i, c) in p.clauses.iter().enumerate() {
        if m.generic && p.patient[i] {
            continue;
        }
        let check = |s: Option<&State>, constants: bool| -> Result<()> {
            let (unary, links) = Pattern::literal(&p.arena, &m.abs, c.head);
            let domain = |e| match e {
                Expr::Value(v) => m.sets.one(v),
                _ => s.map_or(ALL, |s| s.domain(e, m)),
            };
            for (e, feature) in unary {
                if constants && matches!(e, Expr::Var(_)) {
                    continue;
                }
                m.require_unary(feature, domain(e))?;
            }
            for l in links {
                if constants
                    && [l.owner, l.source, l.target]
                        .iter()
                        .any(|e| matches!(e, Expr::Var(_)))
                {
                    continue;
                }
                m.require_rect(
                    l.predicate,
                    l.selector,
                    domain(l.owner),
                    domain(l.source),
                    domain(l.target),
                )?;
            }
            Ok(())
        };
        let result = (|| {
            if m.generic || c.body.is_empty() {
                check(None, true)?;
            }
            if !c.body.is_empty() {
                let pattern = Pattern::body(&p.arena, &m.abs, &c.body, c.names.len(), None);
                if let Some(s) = state(&p.arena, m, &pattern, budget)? {
                    check(Some(&s), false)?;
                }
            }
            Ok(())
        })();
        m.reset_scratch();
        result.map_err(|failure| match failure {
            Failure::Certificate(detail) => bad(format!("{} {detail}", p.label(i))),
            other => other,
        })?;
    }
    Ok(())
}
