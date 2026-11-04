use crate::features::{Counters, Domain, Expr, Model, Pattern, Value};
use crate::graph::{Edge, Graph, body_signature};
use crate::source::Program;
use sb1_checker::program::Goal;
use sb1_checker::term::{Id, Node};
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Origin {
    Skolem(usize),
    Arithmetic { clause: usize, goal: usize },
}
#[derive(Clone)]
pub(crate) struct Evidence {
    pub clause: usize,
    pub var: usize,
    pub pattern: Pattern,
}
type Site = (usize, usize);
#[derive(Clone)]
struct Binding {
    roots: BTreeSet<usize>,
    forwards: BTreeSet<Site>,
    fallbacks: usize,
}
pub struct Growth {
    pub graph: Graph,
    pub nodes: Vec<Origin>,
    incidences: Vec<Vec<Site>>,
    pub(crate) var_sources: HashMap<(usize, usize), BTreeSet<usize>>,
    pub precision_fallbacks: usize,
    pub source_patterns: usize,
    pub source_cache_hits: usize,
}
impl Growth {
    pub fn build(program: &Program, model: &Model, selected: Option<&BTreeSet<usize>>) -> Self {
        Self::build_cached(program, model, selected, true, None)
    }
    pub(crate) fn for_sources(
        program: &Program,
        model: &Model,
        sources: impl Fn(&Origin) -> bool,
    ) -> Self {
        Self::build_cached(program, model, None, true, Some(&sources))
    }
    #[cfg(test)]
    pub(crate) fn build_reference(program: &Program, model: &Model) -> Self {
        Self::build_cached(program, model, None, false, None)
    }
    fn build_cached(
        program: &Program,
        model: &Model,
        selected: Option<&BTreeSet<usize>>,
        enabled: bool,
        requested: Option<&dyn Fn(&Origin) -> bool>,
    ) -> Self {
        let arena = &program.arena;
        let ids = selected.map_or_else(
            || (0..program.clauses.len()).collect::<Vec<_>>(),
            |s| s.iter().copied().collect(),
        );
        let mut nodes = (0..model.abs.origins.len())
            .map(Origin::Skolem)
            .collect::<Vec<_>>();
        let mut arithmetic = HashMap::<(usize, usize), BTreeSet<usize>>::new();
        let mut arithmetic_inputs = Vec::new();
        let mut requests = Vec::new();
        for &i in &ids {
            if program.patient(i) {
                continue;
            }
            let c = &program.clauses[i];
            for (g, goal) in c.body.iter().enumerate() {
                if let Goal::Builtin(t) = goal
                    && let Some((name, args)) = arena.call(*t)
                    && matches!(arena.name(name), "guideline_product" | "guideline_quotient")
                {
                    let node = nodes.len();
                    nodes.push(Origin::Arithmetic { clause: i, goal: g });
                    // Bound labels have a fixed finite alphabet; numeric outputs grow U.
                    let mut outputs = BTreeSet::new();
                    arena.vars(args[5], &mut outputs);
                    for v in outputs {
                        arithmetic.entry((i, v)).or_default().insert(node);
                    }
                    let mut inputs = BTreeSet::new();
                    for t in [args[1], args[3]] {
                        arena.vars(t, &mut inputs);
                    }
                    for var in inputs {
                        arithmetic_inputs.push((node, i, var));
                    }
                }
            }
            fn constructors(
                program: &Program,
                t: Id,
                out: &mut Vec<(usize, usize)>,
                model: &Model,
            ) {
                if let Some(a) = program.arena.named(t, "$guideline_id", 5) {
                    let Value::Origin(origin) = model.abs.value(t) else {
                        unreachable!()
                    };
                    let mut vs = BTreeSet::new();
                    program.arena.vars(a[4], &mut vs);
                    for v in vs {
                        out.push((origin, v));
                    }
                } else if let Some((_, args)) = program.arena.call(t) {
                    for t in args {
                        constructors(program, *t, out, model);
                    }
                }
            }
            let mut deps = Vec::new();
            constructors(program, c.head, &mut deps, model);
            deps.sort_unstable();
            deps.dedup();
            for (origin, var) in deps {
                requests.push((origin, i, var));
            }
        }
        requests.extend(arithmetic_inputs);
        if let Some(requested) = requested {
            requests.retain(|(from, _, _)| requested(&nodes[*from]));
        }
        let mut queue = requests
            .iter()
            .map(|(_, c, v)| (*c, *v))
            .collect::<VecDeque<_>>();
        let mut sites =
            HashMap::<(usize, usize), (BTreeSet<usize>, BTreeSet<(usize, usize)>)>::new();
        let mut patterns = HashMap::new();
        let mut states = HashMap::new();
        let mut counters = Counters::default();
        let mut fallbacks = 0;
        let mut signatures = HashMap::<Vec<usize>, usize>::new();
        let mut clause_signatures = HashMap::<usize, (usize, Vec<usize>)>::new();
        let mut site_signatures = HashMap::<Site, Site>::new();
        let mut bindings = HashMap::<Site, Binding>::new();
        let mut cache_hits = 0;
        while let Some(site) = queue.pop_front() {
            if sites.contains_key(&site) {
                continue;
            }
            let (i, var) = site;
            let c = &program.clauses[i];
            let (signature, order) = clause_signatures.entry(i).or_insert_with(|| {
                let (key, order) = body_signature(arena, &c.body);
                let next = signatures.len();
                let id = *signatures.entry(key).or_insert(next);
                (id, order)
            });
            let canonical = order
                .iter()
                .position(|v| *v == var)
                .expect("bound source variable");
            let key = (*signature, canonical);
            site_signatures.insert(site, if enabled { key } else { site });
            let pattern = patterns.entry(i).or_insert_with(|| {
                Rc::new(Pattern::body(
                    arena,
                    &model.abs,
                    &c.body,
                    c.names.len(),
                    None,
                ))
            });
            let own = arithmetic.get(&site).cloned().unwrap_or_default();
            if enabled && let Some(cached) = bindings.get(&key) {
                let mut sources = cached.roots.clone();
                sources.extend(own);
                let forwards = cached.forwards.clone();
                queue.extend(forwards.iter().copied());
                fallbacks += cached.fallbacks;
                cache_hits += 1;
                sites.insert(site, (sources, forwards));
                continue;
            }
            let before_fallbacks = fallbacks;
            let base = states
                .entry(i)
                .or_insert_with(|| model.state(arena, pattern, &mut counters));
            let mut sources = BTreeSet::new();
            let mut forwards = BTreeSet::new();
            if let Some(base) = base {
                let domain = base.domain(Expr::Var(var));
                // Unary features omit location. Require a matching producer
                // head: a boxed event must not masquerade as an actual event.
                for goal in &c.body {
                    if let Goal::Positive(t) = goal {
                        let (_, args) = arena.call(*t).unwrap();
                        let positions = args
                            .iter()
                            .enumerate()
                            .filter_map(|(at, a)| {
                                matches!(arena.node(*a),Node::Var(v)if *v==var).then_some(at)
                            })
                            .collect::<Vec<_>>();
                        let nested = args.iter().any(|a| {
                            !matches!(arena.node(*a), Node::Var(_)) && {
                                let mut vs = BTreeSet::new();
                                arena.vars(*a, &mut vs);
                                vs.contains(&var)
                            }
                        });
                        if nested {
                            fallbacks += 1;
                            sources.extend(0..nodes.len());
                        }
                        if positions.is_empty() {
                            continue;
                        }
                        for producer in model.candidates(arena, *t, base) {
                            if model
                                .forced(program, pattern, *t, producer, base, &mut counters)
                                .is_none()
                            {
                                continue;
                            }
                            let (_, head) = arena.call(program.clauses[producer].head).unwrap();
                            for &at in &positions {
                                match arena.node(head[at]) {
                                    Node::Var(v) => {
                                        let next = (producer, *v);
                                        forwards.insert(next);
                                        queue.push_back(next);
                                    }
                                    _ => {
                                        if let Value::Origin(origin) = model.abs.value(head[at])
                                            && !domain
                                                .meet(&Domain::one(Value::Origin(origin)), arena)
                                                .is_empty()
                                        {
                                            sources.insert(origin);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if enabled {
                bindings.insert(
                    key,
                    Binding {
                        roots: sources.clone(),
                        forwards: forwards.clone(),
                        fallbacks: fallbacks - before_fallbacks,
                    },
                );
            }
            sources.extend(own);
            sites.insert(site, (sources, forwards));
        }
        let mut allowed = HashMap::new();
        loop {
            let mut changed = false;
            let snapshot = sites
                .iter()
                .map(|(k, (s, _))| (*k, s.clone()))
                .collect::<HashMap<_, _>>();
            for (site, (sources, forwards)) in &mut sites {
                for target in forwards.iter() {
                    if let Some(next) = snapshot.get(target) {
                        for source in next {
                            let good = *allowed
                                .entry((site_signatures[site], *source))
                                .or_insert_with(|| {
                                    let value = match nodes[*source] {
                                        Origin::Skolem(o) => Value::Origin(o),
                                        Origin::Arithmetic { .. } => Value::Number,
                                    };
                                    model
                                        .bound_state(
                                            arena,
                                            &patterns[&site.0],
                                            Expr::Var(site.1),
                                            &Domain::one(value),
                                            &mut counters,
                                        )
                                        .and_then(|s| {
                                            model.feature_witness(
                                                arena,
                                                &patterns[&site.0],
                                                s,
                                                &mut counters,
                                            )
                                        })
                                        .is_some()
                                });
                            if good {
                                changed |= sources.insert(*source);
                            }
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        let source_patterns = if enabled { bindings.len() } else { sites.len() };
        if crate::memory::enabled() {
            use crate::memory::{bytes, record};
            let root_values = sites.values().map(|(s, _)| s.len()).sum();
            let forwarding = sites.values().map(|(_, f)| f.len()).sum();
            let cached_roots = bindings.values().map(|b| b.roots.len()).sum();
            let cached_forwards = bindings.values().map(|b| b.forwards.len()).sum();
            record(
                "growth-cache",
                &[
                    ("program-clauses", program.clauses.len(), 0, "count"),
                    (
                        "patterns",
                        patterns.len(),
                        patterns
                            .values()
                            .map(|p| {
                                bytes::<Id>(p.goals.capacity())
                                    + bytes::<crate::features::Unary>(p.unary.capacity())
                                    + bytes::<crate::features::Link>(p.links.capacity())
                            })
                            .sum(),
                        "buffer-capacity",
                    ),
                    (
                        "constraint-states",
                        states.len(),
                        states.values().flatten().map(|s| s.memory_bytes()).sum(),
                        "buffers+tree-payload-lower-bound",
                    ),
                    (
                        "site-map",
                        sites.len(),
                        bytes::<(Site, (BTreeSet<usize>, BTreeSet<Site>))>(sites.capacity()),
                        "payload-lower-bound",
                    ),
                    (
                        "site-root-values",
                        root_values,
                        bytes::<usize>(root_values),
                        "payload-lower-bound",
                    ),
                    (
                        "forwarding-values",
                        forwarding,
                        bytes::<Site>(forwarding),
                        "payload-lower-bound",
                    ),
                    (
                        "source-cache-roots",
                        cached_roots,
                        bytes::<usize>(cached_roots),
                        "payload-lower-bound",
                    ),
                    (
                        "source-cache-forwards",
                        cached_forwards,
                        bytes::<Site>(cached_forwards),
                        "payload-lower-bound",
                    ),
                    (
                        "allowed-source-cache",
                        allowed.len(),
                        bytes::<((Site, usize), bool)>(allowed.capacity()),
                        "payload-lower-bound",
                    ),
                    (
                        "body-signatures",
                        signatures.len(),
                        signatures
                            .keys()
                            .map(|k| bytes::<usize>(k.capacity()))
                            .sum(),
                        "buffer-capacity",
                    ),
                ],
            );
        }
        drop(patterns);
        drop(states);
        drop(bindings);
        drop(allowed);
        drop(site_signatures);
        drop(clause_signatures);
        drop(signatures);
        drop(queue);
        drop(arithmetic);
        drop(ids);
        let mut incidences = vec![Vec::new(); nodes.len()];
        for (from, clause, var) in requests {
            if sites.contains_key(&(clause, var)) {
                incidences[from].push((clause, var));
            }
        }
        let edges = incidences
            .iter()
            .map(|row| {
                let mut targets = row
                    .iter()
                    .flat_map(|site| sites[site].0.iter().copied())
                    .collect::<Vec<_>>();
                targets.sort_unstable();
                targets.dedup();
                let mut edges = Vec::with_capacity(targets.len());
                edges.extend(targets.into_iter().map(|to| Edge {
                    to,
                    negative: false,
                    growth: true,
                }));
                edges
            })
            .collect();
        Self {
            graph: Graph::from_edges(edges, counters),
            nodes,
            incidences,
            var_sources: sites
                .into_iter()
                .map(|(site, (sources, _))| (site, sources))
                .collect(),
            precision_fallbacks: fallbacks,
            source_patterns,
            source_cache_hits: cache_hits,
        }
    }
    pub fn admitted(&self) -> bool {
        self.graph.admitted()
    }
    pub fn record_memory(&self, stage: &str) {
        if !crate::memory::enabled() {
            return;
        }
        use crate::memory::{bytes, record};
        let sources = self.var_sources.values().map(BTreeSet::len).sum();
        record(
            stage,
            &[
                (
                    "flow-nodes",
                    self.nodes.len(),
                    bytes::<Origin>(self.nodes.capacity()),
                    "buffer-capacity",
                ),
                (
                    "flow-incidence-rows",
                    self.incidences.len(),
                    bytes::<Vec<Site>>(self.incidences.capacity()),
                    "buffer-capacity",
                ),
                (
                    "flow-incidences",
                    self.incidences.iter().map(Vec::len).sum(),
                    self.incidences
                        .iter()
                        .map(|v| bytes::<Site>(v.capacity()))
                        .sum(),
                    "buffer-capacity",
                ),
                (
                    "variable-source-map",
                    self.var_sources.len(),
                    bytes::<(Site, BTreeSet<usize>)>(self.var_sources.capacity()),
                    "payload-lower-bound",
                ),
                (
                    "variable-source-values",
                    sources,
                    bytes::<usize>(sources),
                    "payload-lower-bound",
                ),
            ],
        );
        self.graph.record_memory(stage);
    }
    pub(crate) fn supports(&self, from: usize, to: usize) -> impl Iterator<Item = Site> + '_ {
        self.incidences[from].iter().copied().filter(move |site| {
            self.var_sources
                .get(site)
                .is_some_and(|sources| sources.contains(&to))
        })
    }
    pub(crate) fn evidence_at(program: &Program, model: &Model, site: Site) -> Evidence {
        let (clause, var) = site;
        let c = &program.clauses[clause];
        Evidence {
            clause,
            var,
            pattern: Pattern::body(&program.arena, &model.abs, &c.body, c.names.len(), None),
        }
    }
    pub fn label(&self, program: &Program, model: &Model, node: usize) -> String {
        match self.nodes[node] {
            Origin::Skolem(o) => model.abs.describe(&program.arena, Value::Origin(o)),
            Origin::Arithmetic { clause, goal } => {
                format!("arithmetic({},{})", program.label(clause), goal + 1)
            }
        }
    }
    pub fn render_witnesses(&self, program: &Program, model: &Model) -> String {
        let mut out =
            String::from("kind\tcomponent\tstep\tfrom\tsign\tgrowth\tto\tgoal\tmatched_features\n");
        for witness in &self.graph.witnesses {
            for (step, (from, edge)) in witness.cycle.iter().enumerate() {
                let site = self
                    .supports(*from, edge.to)
                    .next()
                    .expect("growth edge support");
                let e = Self::evidence_at(program, model, site);
                let mut counters = Counters::default();
                let state = match self.nodes[edge.to] {
                    Origin::Skolem(o) => model.bound_state(
                        &program.arena,
                        &e.pattern,
                        Expr::Var(e.var),
                        &Domain::one(Value::Origin(o)),
                        &mut counters,
                    ),
                    Origin::Arithmetic { .. } => {
                        model.state(&program.arena, &e.pattern, &mut counters)
                    }
                }
                .and_then(|s| model.feature_witness(&program.arena, &e.pattern, s, &mut counters));
                let features = state.map_or_else(
                    || "forwarded-source; whole-conjunction instance at forwarding site".into(),
                    |s| {
                        model
                            .describe_state(&program.arena, &e.pattern, &s)
                            .replace(['\n', '\t'], " | ")
                    },
                );
                out.push_str(&format!(
                    "growth\torigin-{}\t{}\t{}\t+\ttrue\t{}\tDeps({}) @ {}\t{}\n",
                    witness.component,
                    step + 1,
                    self.label(program, model, *from),
                    self.label(program, model, edge.to),
                    sb1_checker::term::variable_name(e.var),
                    program.label(e.clause),
                    features
                ));
            }
        }
        out
    }
    pub fn edge_table(&self, program: &Program, model: &Model) -> String {
        let mut out = String::from("from\tto\tclause\tvariable\n");
        for (from, edges) in self.graph.edges.iter().enumerate() {
            for edge in edges {
                let (clause, var) = self
                    .supports(from, edge.to)
                    .next()
                    .expect("growth edge support");
                out.push_str(&format!(
                    "{}\t{}\t{}\t{}\n",
                    self.label(program, model, from),
                    self.label(program, model, edge.to),
                    program.label(clause),
                    sb1_checker::term::variable_name(var)
                ));
            }
        }
        out
    }
}
