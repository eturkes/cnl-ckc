use crate::features::{Counters, Domain, Expr, Model, Pattern};
use crate::source::Program;
use sb1_checker::program::{Goal, PREDICATES};
use sb1_checker::query::Query;
use sb1_checker::term::{Arena, Id, Node};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Edge {
    pub to: usize,
    pub negative: bool,
    pub growth: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Refusal {
    Negative,
    Growth,
}
impl Refusal {
    pub fn name(self) -> &'static str {
        match self {
            Self::Negative => "negative",
            Self::Growth => "growth",
        }
    }
}
pub struct Witness {
    pub kind: Refusal,
    pub component: usize,
    pub cycle: Vec<(usize, Edge)>,
}
pub struct Graph {
    pub edges: Vec<Vec<Edge>>,
    pub components: Vec<usize>,
    pub sizes: Vec<usize>,
    pub witnesses: Vec<Witness>,
    pub counters: Counters,
    pub body_patterns: usize,
    pub body_cache_hits: usize,
}

#[derive(Clone, Copy)]
struct Call {
    goal: Id,
    negative: bool,
    naf: usize,
}
fn calls(goals: &[Goal]) -> Vec<Call> {
    let mut out = Vec::new();
    for (n, g) in goals.iter().enumerate() {
        match g {
            Goal::Positive(goal) => out.push(Call {
                goal: *goal,
                negative: false,
                naf: 0,
            }),
            Goal::Not(inner) => {
                for g in inner {
                    if let Goal::Positive(goal) = g {
                        out.push(Call {
                            goal: *goal,
                            negative: true,
                            naf: n + 1,
                        });
                    }
                }
            }
            Goal::Builtin(_) => {}
        }
    }
    out
}
pub(crate) fn body_signature(arena: &Arena, goals: &[Goal]) -> (Vec<usize>, Vec<usize>) {
    fn term(arena: &Arena, t: Id, vars: &mut HashMap<usize, usize>, out: &mut Vec<usize>) {
        match arena.node(t) {
            Node::Var(v) => {
                let next = vars.len();
                let v = *vars.entry(*v).or_insert(next);
                out.extend([0, v]);
            }
            Node::Compound(name, args) => {
                out.extend([1, *name, args.len()]);
                for a in args {
                    term(arena, *a, vars, out);
                }
            }
            _ => out.extend([2, t]),
        }
    }
    fn visit(
        arena: &Arena,
        goals: &[Goal],
        vars: &mut HashMap<usize, usize>,
        out: &mut Vec<usize>,
    ) {
        out.push(goals.len());
        for g in goals {
            match g {
                Goal::Positive(t) => {
                    out.push(3);
                    term(arena, *t, vars, out);
                }
                Goal::Builtin(t) => {
                    out.push(4);
                    term(arena, *t, vars, out);
                }
                Goal::Not(inner) => {
                    out.push(5);
                    visit(arena, inner, vars, out);
                }
            }
        }
    }
    let mut out = Vec::new();
    let mut vars = HashMap::new();
    visit(arena, goals, &mut vars, &mut out);
    let mut order = vars.into_iter().collect::<Vec<_>>();
    order.sort_by_key(|(_, canonical)| *canonical);
    (out, order.into_iter().map(|(var, _)| var).collect())
}
fn body_key(arena: &Arena, goals: &[Goal]) -> Vec<usize> {
    body_signature(arena, goals).0
}
fn head_targets(
    program: &Program,
    model: &Model,
    pattern: &Pattern,
    base: Option<&crate::features::State>,
    target: Id,
    counters: &mut Counters,
) -> Vec<usize> {
    let Some(base) = base else { return Vec::new() };
    model
        .candidates(&program.arena, target, base)
        .into_iter()
        .filter(|to| {
            model
                .forced(program, pattern, target, *to, base, counters)
                .is_some()
        })
        .collect()
}
fn body_targets(
    program: &Program,
    model: &Model,
    clause: usize,
    counters: &mut Counters,
) -> Vec<Vec<usize>> {
    model.census_caller(program, clause);
    let c = &program.clauses[clause];
    let pattern = Pattern::body(&program.arena, &model.abs, &c.body, c.names.len(), None);
    let base = model.state(&program.arena, &pattern, counters);
    let mut out = Vec::new();
    for g in &c.body {
        match g {
            Goal::Positive(t) => out.push(head_targets(
                program,
                model,
                &pattern,
                base.as_ref(),
                *t,
                counters,
            )),
            Goal::Not(inner) => {
                let p = Pattern::body(
                    &program.arena,
                    &model.abs,
                    &c.body,
                    c.names.len(),
                    Some(inner),
                );
                let b = model.state(&program.arena, &p, counters);
                for g in inner {
                    if let Goal::Positive(t) = g {
                        out.push(head_targets(program, model, &p, b.as_ref(), *t, counters));
                    }
                }
            }
            Goal::Builtin(_) => {}
        }
    }
    out
}

fn add_edges(
    program: &Program,
    model: &Model,
    pattern: &Pattern,
    target: Id,
    negative: bool,
    out: &mut HashSet<Edge>,
    counters: &mut Counters,
) {
    let Some(base) = model.state(&program.arena, pattern, counters) else {
        return;
    };
    for to in model.candidates(&program.arena, target, &base) {
        if model
            .forced(program, pattern, target, to, &base, counters)
            .is_some()
        {
            out.insert(Edge {
                to,
                negative,
                growth: false,
            });
        }
    }
}

impl Graph {
    pub fn build(program: &Program, model: &Model, selected: Option<&BTreeSet<usize>>) -> Self {
        Self::build_cached(program, model, selected, true)
    }
    #[cfg(test)]
    pub(crate) fn build_uncached(program: &Program, model: &Model) -> Self {
        Self::build_cached(program, model, None, false)
    }
    fn build_cached(
        program: &Program,
        model: &Model,
        selected: Option<&BTreeSet<usize>>,
        enabled: bool,
    ) -> Self {
        let mut edges = vec![Vec::new(); program.clauses.len()];
        let mut counters = Counters::default();
        let mut cache = HashMap::<Vec<usize>, Vec<Vec<usize>>>::new();
        let mut hits = 0;
        let mut patterns = 0;
        let ids = selected.map_or_else(
            || (0..program.clauses.len()).collect::<Vec<_>>(),
            |s| s.iter().copied().collect(),
        );
        for i in ids {
            if program.patient(i) || program.clauses[i].body.is_empty() {
                continue;
            }
            let c = &program.clauses[i];
            let key = body_key(&program.arena, &c.body);
            let targets = if enabled && cache.contains_key(&key) {
                hits += 1;
                cache[&key].clone()
            } else {
                patterns += 1;
                let result = body_targets(program, model, i, &mut counters);
                if enabled {
                    cache.insert(key, result.clone());
                }
                result
            };
            let mut row = calls(&c.body)
                .into_iter()
                .zip(targets)
                .flat_map(|(call, targets)| {
                    targets.into_iter().map(move |to| Edge {
                        to,
                        negative: call.negative,
                        growth: false,
                    })
                })
                .collect::<Vec<_>>();
            crate::work::visit("clause_edge_expansions", row.len());
            row.sort_unstable();
            row.dedup_by_key(|e| (e.to, e.negative, e.growth));
            row.shrink_to_fit();
            edges[i] = row;
        }
        if crate::memory::enabled() {
            use crate::memory::{bytes, record};
            record(
                "graph-cache",
                &[
                    (
                        "body-keys",
                        cache.len(),
                        cache.keys().map(|k| bytes::<usize>(k.capacity())).sum(),
                        "buffer-capacity",
                    ),
                    (
                        "target-rows",
                        cache.len(),
                        cache
                            .values()
                            .map(|v| bytes::<Vec<usize>>(v.capacity()))
                            .sum(),
                        "buffer-capacity",
                    ),
                    (
                        "target-ids",
                        cache.values().flat_map(|v| v.iter()).map(Vec::len).sum(),
                        cache
                            .values()
                            .flat_map(|v| v.iter())
                            .map(|v| bytes::<usize>(v.capacity()))
                            .sum(),
                        "buffer-capacity",
                    ),
                ],
            );
        }
        if crate::census::enabled() {
            crate::work::peak("body_cache_key_cells", cache.keys().map(Vec::len).sum());
            crate::work::peak("body_cache_value_rows", cache.values().map(Vec::len).sum());
            crate::work::peak(
                "body_cache_target_memberships",
                cache.values().flat_map(|v| v.iter()).map(Vec::len).sum(),
            );
        }
        drop(cache);
        let mut graph = Self::from_edges(edges, counters);
        graph.body_patterns = patterns;
        graph.body_cache_hits = hits;
        graph
    }
    pub fn from_edges(edges: Vec<Vec<Edge>>, counters: Counters) -> Self {
        let n = edges.len();
        let mut seen = vec![false; n];
        let mut order = Vec::with_capacity(n);
        for start in 0..n {
            if seen[start] {
                continue;
            }
            seen[start] = true;
            let mut stack = vec![(start, 0)];
            while let Some((node, next)) = stack.last_mut() {
                if let Some(e) = edges[*node].get(*next) {
                    *next += 1;
                    if !seen[e.to] {
                        seen[e.to] = true;
                        stack.push((e.to, 0));
                    }
                } else {
                    order.push(*node);
                    stack.pop();
                }
            }
        }
        let mut reverse = vec![Vec::new(); n];
        for (from, row) in edges.iter().enumerate() {
            for e in row {
                reverse[e.to].push(from);
            }
        }
        let mut components = vec![usize::MAX; n];
        let mut sizes = Vec::new();
        for &start in order.iter().rev() {
            if components[start] != usize::MAX {
                continue;
            }
            let component = sizes.len();
            let mut size = 0;
            let mut stack = vec![start];
            components[start] = component;
            while let Some(node) = stack.pop() {
                size += 1;
                for &from in &reverse[node] {
                    if components[from] == usize::MAX {
                        components[from] = component;
                        stack.push(from);
                    }
                }
            }
            sizes.push(size);
        }
        if crate::memory::enabled() {
            use crate::memory::{bytes, record};
            record(
                "scc-transient",
                &[
                    (
                        "scc-input-edges",
                        edges.iter().map(Vec::len).sum(),
                        edges.iter().map(|v| bytes::<Edge>(v.capacity())).sum(),
                        "buffer-capacity",
                    ),
                    (
                        "scc-reverse-rows",
                        reverse.len(),
                        bytes::<Vec<usize>>(reverse.capacity()),
                        "buffer-capacity",
                    ),
                    (
                        "scc-reverse-ids",
                        reverse.iter().map(Vec::len).sum(),
                        reverse.iter().map(|v| bytes::<usize>(v.capacity())).sum(),
                        "buffer-capacity",
                    ),
                    (
                        "scc-seen",
                        seen.len(),
                        bytes::<bool>(seen.capacity()),
                        "buffer-capacity",
                    ),
                    (
                        "scc-order",
                        order.len(),
                        bytes::<usize>(order.capacity()),
                        "buffer-capacity",
                    ),
                ],
            );
        }
        if crate::census::enabled() {
            let memberships = edges.iter().map(Vec::len).sum::<usize>();
            crate::work::peak("scc_forward_memberships", memberships);
            crate::work::peak(
                "scc_reverse_memberships",
                reverse.iter().map(Vec::len).sum(),
            );
            crate::work::peak(
                "scc_live_edge_memberships",
                memberships + reverse.iter().map(Vec::len).sum::<usize>(),
            );
            crate::work::peak(
                "scc_scratch_cells",
                reverse.len() + seen.len() + order.len() + components.len() + sizes.len(),
            );
            crate::work::visit("scc_edge_member_visits", memberships * 4);
        }
        drop(reverse);
        drop(seen);
        drop(order);
        let mut out = Self {
            edges,
            components,
            sizes,
            witnesses: Vec::new(),
            counters,
            body_patterns: 0,
            body_cache_hits: 0,
        };
        let mut negative = vec![Vec::new(); out.sizes.len()];
        let mut growth = vec![Vec::new(); out.sizes.len()];
        for (from, row) in out.edges.iter().enumerate() {
            for e in row {
                if out.components[from] == out.components[e.to] {
                    if e.negative {
                        negative[out.components[from]].push((from, *e));
                    }
                    if e.growth {
                        growth[out.components[from]].push((from, *e));
                    }
                }
            }
        }
        for (kind, groups) in [(Refusal::Negative, negative), (Refusal::Growth, growth)] {
            for (component, candidates) in groups.into_iter().enumerate() {
                if candidates.is_empty() {
                    continue;
                }
                let mut best: Option<Vec<(usize, Edge)>> = None;
                for (from, e) in candidates {
                    if let Some(cycle) =
                        out.cycle(from, e, best.as_ref().map_or(usize::MAX, Vec::len))
                        && best.as_ref().is_none_or(|old| cycle.len() < old.len())
                    {
                        best = Some(cycle);
                    }
                    if best.as_ref().is_some_and(|c| c.len() == 1) {
                        break;
                    }
                }
                out.witnesses.push(Witness {
                    kind,
                    component,
                    cycle: best.expect("SCC return path"),
                });
            }
        }
        out
    }
    fn cycle(&self, from: usize, edge: Edge, limit: usize) -> Option<Vec<(usize, Edge)>> {
        if edge.to == from {
            return Some(vec![(from, edge)]);
        }
        let component = self.components[from];
        let mut queue = VecDeque::from([(edge.to, 1)]);
        let mut predecessor = std::collections::HashMap::new();
        predecessor.insert(edge.to, None);
        while let Some((node, depth)) = queue.pop_front() {
            if depth + 1 >= limit {
                continue;
            }
            for e in &self.edges[node] {
                if self.components[e.to] != component || predecessor.contains_key(&e.to) {
                    continue;
                }
                predecessor.insert(e.to, Some((node, *e)));
                if e.to == from {
                    let mut path = Vec::new();
                    let mut at = from;
                    while let Some(Some((node, e))) = predecessor.get(&at) {
                        path.push((*node, *e));
                        at = *node;
                    }
                    path.reverse();
                    path.insert(0, (from, edge));
                    return Some(path);
                }
                queue.push_back((e.to, depth + 1));
            }
        }
        None
    }
    pub fn reachable(&self, roots: &BTreeSet<usize>) -> BTreeSet<usize> {
        let mut out = roots.clone();
        let mut queue = roots.iter().copied().collect::<VecDeque<_>>();
        while let Some(i) = queue.pop_front() {
            for e in &self.edges[i] {
                if out.insert(e.to) {
                    queue.push_back(e.to);
                }
            }
        }
        out
    }
    pub fn admitted(&self) -> bool {
        self.witnesses.is_empty()
    }
    pub fn edge_count(&self) -> usize {
        self.edges.iter().map(Vec::len).sum()
    }
    pub fn record_memory(&self, stage: &str) {
        if !crate::memory::enabled() {
            return;
        }
        use crate::memory::{bytes, record};
        record(
            stage,
            &[
                (
                    "graph-edge-rows",
                    self.edges.len(),
                    bytes::<Vec<Edge>>(self.edges.capacity()),
                    "buffer-capacity",
                ),
                (
                    "graph-edges",
                    self.edge_count(),
                    self.edges.iter().map(|v| bytes::<Edge>(v.capacity())).sum(),
                    "buffer-capacity",
                ),
                (
                    "graph-components",
                    self.components.len(),
                    bytes::<usize>(self.components.capacity()),
                    "buffer-capacity",
                ),
                (
                    "graph-sizes",
                    self.sizes.len(),
                    bytes::<usize>(self.sizes.capacity()),
                    "buffer-capacity",
                ),
                (
                    "graph-witnesses",
                    self.witnesses.len(),
                    bytes::<Witness>(self.witnesses.capacity())
                        + self
                            .witnesses
                            .iter()
                            .map(|w| bytes::<(usize, Edge)>(w.cycle.capacity()))
                            .sum::<usize>(),
                    "buffer-capacity",
                ),
            ],
        );
    }
    pub fn render_witnesses(&self, program: &Program, model: &Model) -> String {
        let mut out =
            String::from("kind\tcomponent\tstep\tfrom\tsign\tgrowth\tto\tgoal\tmatched_features\n");
        for witness in &self.witnesses {
            for (step, (from, e)) in witness.cycle.iter().enumerate() {
                let c = &program.clauses[*from];
                let (goal, features) = calls(&c.body)
                    .into_iter()
                    .filter(|call| call.negative == e.negative)
                    .find_map(|call| {
                        let inner = if call.negative {
                            match &c.body[call.naf - 1] {
                                Goal::Not(gs) => Some(gs.as_slice()),
                                _ => None,
                            }
                        } else {
                            None
                        };
                        let pattern = Pattern::body(
                            &program.arena,
                            &model.abs,
                            &c.body,
                            c.names.len(),
                            inner,
                        );
                        let mut counters = Counters::default();
                        let base = model.state(&program.arena, &pattern, &mut counters)?;
                        if !model
                            .candidates(&program.arena, call.goal, &base)
                            .contains(&e.to)
                        {
                            return None;
                        }
                        let state = model.forced(
                            program,
                            &pattern,
                            call.goal,
                            e.to,
                            &base,
                            &mut counters,
                        )?;
                        let state = model.feature_witness(
                            &program.arena,
                            &pattern,
                            state,
                            &mut counters,
                        )?;
                        Some((
                            program.arena.render(call.goal),
                            model
                                .describe_state(&program.arena, &pattern, &state)
                                .replace(['\n', '\t'], " | "),
                        ))
                    })
                    .unwrap_or_else(|| {
                        (
                            "unconfirmed-call".into(),
                            "unconfirmed-reconstruction".into(),
                        )
                    });
                out.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    witness.kind.name(),
                    witness.component,
                    step + 1,
                    program.label(*from),
                    if e.negative { "-" } else { "+" },
                    e.growth,
                    program.label(e.to),
                    goal,
                    features
                ));
            }
        }
        out
    }
}

pub enum Question {
    Goals {
        goals: Vec<Goal>,
        vars: usize,
    },
    All {
        operators: Vec<Id>,
        payload: Vec<Id>,
    },
}
impl Question {
    pub fn parse(arena: &mut Arena, bytes: &[u8]) -> Result<Self, String> {
        let q = Query::parse(arena, Some(bytes)).map_err(|e| e.message)?;
        match q {
            Query::Projected { goals, vars, .. } => Ok(Self::Goals { goals, vars }),
            Query::All => {
                let actual = arena.atom("actual");
                let context = arena.put(Node::Var(0));
                let operators = ["should", "must", "may"]
                    .into_iter()
                    .map(|o| {
                        let o = arena.atom(o);
                        arena.compound("guideline_operator", vec![actual, context, o])
                    })
                    .collect();
                let payload = PREDICATES[2..]
                    .iter()
                    .filter(|(n, _)| {
                        !matches!(
                            *n,
                            "guideline_exclusion"
                                | "guideline_supersession"
                                | "guideline_evaluation_date"
                        )
                    })
                    .map(|(name, arity)| {
                        let args = (0..*arity).map(|v| arena.put(Node::Var(v))).collect();
                        arena.compound(name, args)
                    })
                    .collect();
                Ok(Self::All { operators, payload })
            }
            Query::None => Err("question missing".into()),
        }
    }
    pub fn roots(
        &self,
        program: &Program,
        model: &Model,
        counters: &mut Counters,
    ) -> BTreeSet<usize> {
        let arena = &program.arena;
        let mut roots = BTreeSet::new();
        match self {
            Self::Goals { goals, vars } => {
                let positive = Pattern::body(arena, &model.abs, goals, *vars, None);
                for g in goals {
                    match g {
                        Goal::Positive(t) => {
                            let mut edges = HashSet::new();
                            add_edges(program, model, &positive, *t, false, &mut edges, counters);
                            roots.extend(edges.into_iter().map(|e| e.to));
                        }
                        Goal::Not(inner) => {
                            let p = Pattern::body(arena, &model.abs, goals, *vars, Some(inner));
                            for g in inner {
                                if let Goal::Positive(t) = g {
                                    let mut edges = HashSet::new();
                                    add_edges(program, model, &p, *t, true, &mut edges, counters);
                                    roots.extend(edges.into_iter().map(|e| e.to));
                                }
                            }
                        }
                        Goal::Builtin(_) => {}
                    }
                }
            }
            Self::All { operators, payload } => {
                let mut contexts = Domain::empty();
                for &op in operators {
                    let p = Pattern::new(arena, &model.abs, vec![op], 8);
                    if let Some(base) = model.state(arena, &p, counters) {
                        for c in model.candidates(arena, op, &base) {
                            if let Some(state) = model.forced(program, &p, op, c, &base, counters) {
                                roots.insert(c);
                                contexts.union(&state.domain(Expr::Var(0)));
                            }
                        }
                    }
                }
                for &goal in payload {
                    let p = Pattern::new(arena, &model.abs, vec![goal], 8);
                    if let Some(base) =
                        model.bound_state(arena, &p, Expr::Var(0), &contexts, counters)
                    {
                        for c in model.candidates(arena, goal, &base) {
                            if model
                                .forced(program, &p, goal, c, &base, counters)
                                .is_some()
                            {
                                roots.insert(c);
                            }
                        }
                    }
                }
            }
        }
        roots
    }
}
