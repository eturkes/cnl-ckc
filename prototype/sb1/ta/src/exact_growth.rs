//! Exact source-flow construction; the independent oracle stays in growth.rs.
use crate::domain::{Counters, Value};
use crate::exact::Model;
use crate::exact_heads::Index;
use crate::exact_scc::{Builder, Compressed};
use crate::exact_sets::EMPTY;
use crate::features::{Expr, Pattern};
use crate::graph::body_signature;
pub use crate::growth::Origin;
use crate::source::Program;
use sb1_checker::program::Goal;
use sb1_checker::term::{Id, Node};
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::{Rc, Weak};

type Site = (usize, usize);
type Sources = Rc<[usize]>;
#[cfg(test)]
thread_local! { static DROP_ONE_FORWARD_DIRTY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }

#[derive(Default)]
struct Pool {
    // Weak buckets release superseded rows; equality, not the hash, owns identity.
    buckets: HashMap<u64, Vec<Weak<[usize]>>>,
}
impl Pool {
    fn put(&mut self, mut values: Vec<usize>) -> Sources {
        values.sort_unstable();
        values.dedup();
        let mut hash = DefaultHasher::new();
        values.hash(&mut hash);
        let hash = hash.finish();
        #[cfg(test)]
        let hash = if crate::faults::enabled(crate::faults::Kind::HashCollision) {
            0
        } else {
            hash
        };
        let bucket = self.buckets.entry(hash).or_default();
        bucket.retain(|row| row.strong_count() != 0);
        if let Some(row) = bucket
            .iter()
            .filter_map(Weak::upgrade)
            .find(|r| **r == values)
        {
            return row;
        }
        let row = Sources::from(values);
        bucket.push(Rc::downgrade(&row));
        row
    }
    fn union(&mut self, a: &Sources, b: &Sources) -> Sources {
        if Rc::ptr_eq(a, b) || b.is_empty() {
            return a.clone();
        }
        if a.is_empty() {
            return b.clone();
        }
        self.put(a.iter().chain(b.iter()).copied().collect())
    }
    fn sweep(&mut self) {
        self.buckets.retain(|_, rows| {
            rows.retain(|r| r.strong_count() != 0);
            !rows.is_empty()
        });
    }
}
#[derive(Clone)]
struct Binding {
    roots: Sources,
    forwards: Rc<[Site]>,
    fallbacks: usize,
}
struct SiteState {
    roots: Sources,
    forwards: Rc<[Site]>,
    last: Option<Vec<Sources>>,
}
#[derive(Default)]
pub struct Stats {
    pub evaluations: usize,
    pub skipped: usize,
    pub audited: usize,
    pub forwarded_memberships: usize,
    pub allowed_checks: usize,
}
pub struct Growth {
    pub graph: Compressed,
    pub nodes: Vec<Origin>,
    incidences: Vec<Vec<Site>>,
    pub(crate) var_sources: HashMap<Site, Sources>,
    pub precision_fallbacks: usize,
    pub source_patterns: usize,
    pub source_cache_hits: usize,
    pub stats: Stats,
}
impl Growth {
    pub fn build(program: &Program, model: &mut Model, audit: bool, skip_audit: bool) -> Self {
        Self::build_sources(program, model, None, audit, skip_audit, "base_growth")
    }
    pub(crate) fn build_k1(
        program: &Program,
        model: &mut Model,
        audit: bool,
        skip_audit: bool,
    ) -> Self {
        Self::build_sources(program, model, None, audit, skip_audit, "k1_growth")
    }
    pub(crate) fn for_sources(
        program: &Program,
        model: &mut Model,
        requested: &dyn Fn(&Origin) -> bool,
        audit: bool,
        skip_audit: bool,
    ) -> Self {
        Self::build_sources(
            program,
            model,
            Some(requested),
            audit,
            skip_audit,
            "k1_growth",
        )
    }
    fn build_sources(
        program: &Program,
        model: &mut Model,
        requested: Option<&dyn Fn(&Origin) -> bool>,
        audit: bool,
        skip_audit: bool,
        phase: &'static str,
    ) -> Self {
        let arena = &program.arena;
        let mut nodes = (0..model.abs.origins.len())
            .map(Origin::Skolem)
            .collect::<Vec<_>>();
        let mut arithmetic = HashMap::<Site, BTreeSet<usize>>::new();
        let mut arithmetic_inputs = Vec::new();
        let mut requests = Vec::new();
        for (i, c) in program.clauses.iter().enumerate() {
            if program.patient(i) {
                continue;
            }
            for (g, goal) in c.body.iter().enumerate() {
                if let Goal::Builtin(t) = goal
                    && let Some((name, args)) = arena.call(*t)
                    && matches!(arena.name(name), "guideline_product" | "guideline_quotient")
                {
                    let node = nodes.len();
                    nodes.push(Origin::Arithmetic { clause: i, goal: g });
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
            fn constructors(p: &Program, t: Id, out: &mut Vec<(usize, usize)>, m: &Model) {
                if let Some(a) = p.arena.named(t, "$guideline_id", 5) {
                    let Value::Origin(origin) = m.abs.value(t) else {
                        unreachable!()
                    };
                    let mut vars = BTreeSet::new();
                    p.arena.vars(a[4], &mut vars);
                    out.extend(vars.into_iter().map(|v| (origin, v)));
                } else if let Some((_, args)) = p.arena.call(t) {
                    for t in args {
                        constructors(p, *t, out, m);
                    }
                }
            }
            let mut deps = Vec::new();
            constructors(program, c.head, &mut deps, model);
            deps.sort_unstable();
            deps.dedup();
            requests.extend(deps.into_iter().map(|(o, v)| (o, i, v)));
        }
        requests.extend(arithmetic_inputs);
        if let Some(wanted) = requested {
            requests.retain(|(from, _, _)| wanted(&nodes[*from]));
        }
        let index = Index::new(program, model, audit);
        let mut pool = Pool::default();
        let mut queue = requests
            .iter()
            .map(|(_, c, v)| (*c, *v))
            .collect::<VecDeque<_>>();
        let mut sites = HashMap::<Site, SiteState>::new();
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
                (*signatures.entry(key).or_insert(next), order)
            });
            let canonical = order
                .iter()
                .position(|v| *v == var)
                .expect("bound source variable");
            let key = (*signature, canonical);
            site_signatures.insert(site, key);
            let pattern = patterns
                .entry(i)
                .or_insert_with(|| Pattern::body(arena, &model.abs, &c.body, c.names.len(), None));
            let own = pool.put(
                arithmetic
                    .get(&site)
                    .map_or_else(Vec::new, |s| s.iter().copied().collect()),
            );
            let binding = if let Some(binding) = bindings.get(&key) {
                cache_hits += 1;
                binding.clone()
            } else {
                let base = states
                    .entry(i)
                    .or_insert_with(|| model.state(arena, pattern, &mut counters));
                let mut sources = BTreeSet::new();
                let mut forwards = BTreeSet::new();
                let mut nested_fallbacks = 0;
                if let Some(base) = base {
                    let domain = base.domain(Expr::Var(var), &model.sets);
                    for goal in &c.body {
                        if let Goal::Positive(t) = goal {
                            let (_, args) = arena.call(*t).unwrap();
                            let positions = args
                                .iter()
                                .enumerate()
                                .filter_map(|(at, a)| {
                                    matches!(arena.node(*a), Node::Var(v) if *v == var)
                                        .then_some(at)
                                })
                                .collect::<Vec<_>>();
                            let nested = args.iter().any(|a| {
                                !matches!(arena.node(*a), Node::Var(_)) && {
                                    let mut vars = BTreeSet::new();
                                    arena.vars(*a, &mut vars);
                                    vars.contains(&var)
                                }
                            });
                            if nested {
                                nested_fallbacks += 1;
                                sources.extend(0..nodes.len());
                            }
                            if positions.is_empty() {
                                continue;
                            }
                            for producer in index.candidates_in(
                                program,
                                model,
                                *t,
                                base,
                                audit,
                                Some(crate::exact_heads::QueryContext {
                                    phase,
                                    caller: i,
                                    pattern,
                                    naf: 0,
                                }),
                            ) {
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
                                            forwards.insert((producer, *v));
                                        }
                                        _ => {
                                            if let Value::Origin(origin) = model.abs.value(head[at])
                                                && model.sets.meet(
                                                    domain,
                                                    model.sets.one(Value::Origin(origin)),
                                                    arena,
                                                ) != EMPTY
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
                let binding = Binding {
                    roots: pool.put(sources.into_iter().collect()),
                    forwards: Rc::from(forwards.into_iter().collect::<Vec<_>>()),
                    fallbacks: nested_fallbacks,
                };
                bindings.insert(key, binding.clone());
                binding
            };
            fallbacks += binding.fallbacks;
            queue.extend(binding.forwards.iter().copied());
            sites.insert(
                site,
                SiteState {
                    roots: pool.union(&binding.roots, &own),
                    forwards: binding.forwards,
                    last: None,
                },
            );
            // The index contains no set IDs. Cached constraint states are the only
            // external roots; no forced/witness state survives this boundary.
            model.compact_states(program, &mut states, false);
        }
        drop(states);
        model.compact(program);
        let mut allowed = HashMap::new();
        let mut stats = Stats::default();
        let mut order = sites.keys().copied().collect::<Vec<_>>();
        order.sort_unstable();
        loop {
            let snapshot = sites
                .iter()
                .map(|(k, s)| (*k, s.roots.clone()))
                .collect::<HashMap<_, _>>();
            let mut changed = false;
            for site in &order {
                let state = sites.get_mut(site).unwrap();
                let inputs = state
                    .forwards
                    .iter()
                    .map(|s| snapshot[s].clone())
                    .collect::<Vec<_>>();
                let skip = state
                    .last
                    .as_ref()
                    .is_some_and(|old| old.iter().zip(&inputs).all(|(a, b)| Rc::ptr_eq(a, b)));
                #[cfg(test)]
                let skip = skip
                    || state.last.is_some()
                        && DROP_ONE_FORWARD_DIRTY.with(|fault| fault.replace(false));
                if skip {
                    stats.skipped += 1;
                    if !skip_audit {
                        continue;
                    }
                    stats.audited += 1;
                } else {
                    state.last = Some(inputs.clone());
                    stats.evaluations += 1;
                }
                let mut next = state.roots.clone();
                for input in inputs {
                    stats.forwarded_memberships += input.len();
                    let mut good = Vec::new();
                    for source in input.iter() {
                        let accepted = *allowed
                            .entry((site_signatures[site], *source))
                            .or_insert_with(|| {
                                stats.allowed_checks += 1;
                                let value = match nodes[*source] {
                                    Origin::Skolem(o) => Value::Origin(o),
                                    Origin::Arithmetic { .. } => Value::Number,
                                };
                                model
                                    .bound_state(
                                        arena,
                                        &patterns[&site.0],
                                        Expr::Var(site.1),
                                        model.sets.one(value),
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
                        if accepted {
                            good.push(*source);
                        }
                    }
                    let row = pool.put(good);
                    next = pool.union(&next, &row);
                }
                if skip {
                    assert_eq!(
                        next,
                        state.roots,
                        "source dirty-round skip wrote at {} variable {}",
                        program.label(site.0),
                        site.1
                    );
                } else if !Rc::ptr_eq(&next, &state.roots) {
                    changed = true;
                    state.roots = next;
                }
                model.maybe_compact(program);
            }
            pool.sweep();
            if !changed {
                break;
            }
        }
        model.compact(program);
        let mut incidences = vec![Vec::new(); nodes.len()];
        for (from, clause, var) in requests {
            if sites.contains_key(&(clause, var)) {
                incidences[from].push((clause, var));
            }
        }
        let mut builder = Builder::new(nodes.len());
        for (from, row) in incidences.iter().enumerate() {
            let mut targets = pool.put(Vec::new());
            for site in row {
                targets = pool.union(&targets, &sites[site].roots);
            }
            let row = builder.growth(targets.to_vec());
            builder.attach(from, row);
        }
        Self {
            graph: builder.finish(counters),
            nodes,
            incidences,
            var_sources: sites.into_iter().map(|(site, s)| (site, s.roots)).collect(),
            precision_fallbacks: fallbacks,
            source_patterns: bindings.len(),
            source_cache_hits: cache_hits,
            stats,
        }
    }
    pub fn admitted(&self) -> bool {
        self.graph.admitted()
    }
    pub(crate) fn supports(&self, from: usize, to: usize) -> impl Iterator<Item = Site> + '_ {
        self.incidences[from].iter().copied().filter(move |site| {
            self.var_sources
                .get(site)
                .is_some_and(|s| s.binary_search(&to).is_ok())
        })
    }
    pub(crate) fn evidence_at(
        program: &Program,
        model: &Model,
        site: Site,
    ) -> crate::growth::Evidence {
        let c = &program.clauses[site.0];
        crate::growth::Evidence {
            clause: site.0,
            var: site.1,
            pattern: Pattern::body(&program.arena, &model.abs, &c.body, c.names.len(), None),
        }
    }
    pub fn render_witnesses(&self, p: &Program, m: &Model) -> String {
        let mut text = crate::witness::HEADER.to_owned();
        for witness in &self.graph.witnesses {
            for (step, (from, edge)) in witness.cycle.iter().enumerate() {
                let site = self
                    .supports(*from, edge.to)
                    .next()
                    .expect("growth edge support");
                let e = Self::evidence_at(p, m, site);
                let mut counters = Counters::default();
                let state = match self.nodes[edge.to] {
                    Origin::Skolem(o) => m.bound_state(
                        &p.arena,
                        &e.pattern,
                        Expr::Var(e.var),
                        m.sets.one(Value::Origin(o)),
                        &mut counters,
                    ),
                    Origin::Arithmetic { .. } => m.state(&p.arena, &e.pattern, &mut counters),
                }
                .and_then(|s| m.feature_witness(&p.arena, &e.pattern, s, &mut counters));
                let features = state.map_or_else(
                    || "forwarded-source; whole-conjunction instance at forwarding site".into(),
                    |s| {
                        m.describe_state(&p.arena, &e.pattern, &s)
                            .replace(['\n', '\t'], " | ")
                    },
                );
                text.push_str(&format!(
                    "growth\torigin-{}\t{}\t{}\t+\ttrue\t{}\tDeps({}) @ {}\t{}\n",
                    witness.component,
                    step + 1,
                    self.label(p, m, *from),
                    self.label(p, m, edge.to),
                    sb1_checker::term::variable_name(e.var),
                    p.label(e.clause),
                    features
                ));
            }
        }
        text
    }
    pub fn edge_table(&self, p: &Program, m: &Model) -> String {
        let mut text = String::from("from\tto\tclause\tvariable\n");
        for from in 0..self.nodes.len() {
            for edge in self.graph.row(from) {
                let (clause, var) = self
                    .supports(from, edge.to)
                    .next()
                    .expect("growth edge support");
                text.push_str(&format!(
                    "{}\t{}\t{}\t{}\n",
                    self.label(p, m, from),
                    self.label(p, m, edge.to),
                    p.label(clause),
                    sb1_checker::term::variable_name(var)
                ));
            }
        }
        text
    }
    pub fn label(&self, program: &Program, model: &Model, node: usize) -> String {
        match self.nodes[node] {
            Origin::Skolem(o) => model.abs.describe(&program.arena, Value::Origin(o)),
            Origin::Arithmetic { clause, goal } => {
                format!("arithmetic({},{})", program.label(clause), goal + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_rows_intern_exactly_and_release_superseded_versions() {
        let mut pool = Pool::default();
        let a = pool.put(vec![3, 1, 3]);
        let b = pool.put(vec![1, 3]);
        assert!(Rc::ptr_eq(&a, &b));
        let c = pool.put(vec![2]);
        assert_eq!(&*pool.union(&a, &c), &[1, 2, 3]);
        drop(a);
        drop(b);
        drop(c);
        pool.sweep();
        assert!(pool.buckets.is_empty());
    }
    #[test]
    fn source_hash_collisions_keep_distinct_exact_rows() {
        let _guard = crate::faults::Guard::set(Some(crate::faults::Kind::HashCollision));
        let mut pool = Pool::default();
        let empty = pool.put(Vec::new());
        let a = pool.put(vec![1]);
        let b = pool.put(vec![2]);
        assert_eq!(pool.buckets.len(), 1);
        assert_ne!(a, b);
        assert!(Rc::ptr_eq(&a, &pool.put(vec![1])));
        assert!(empty.is_empty());
        assert_eq!(&*pool.union(&a, &b), &[1, 2]);
    }
    #[test]
    fn source_skip_audit_detects_a_lost_forward_notification() {
        let text = "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\nguideline_entity(actual,X,end,countable) :- guideline_entity(actual,X,next,countable).\nguideline_entity(actual,'$guideline_id'(product,d,2,ref(1),[X]),out,countable) :- guideline_entity(actual,X,end,countable).\n";
        let mut p = Program::new();
        p.add("chain".into(), text.into(), false).unwrap();
        compare(&p, false);
        let mut m = Model::build(&p, None, false, None, true);
        DROP_ONE_FORWARD_DIRTY.with(|f| f.set(true));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Growth::build(&p, &mut m, true, true)
        }));
        DROP_ONE_FORWARD_DIRTY.with(|f| f.set(false));
        assert!(
            result.is_err(),
            "forward-notification plant must turn the source skip audit red"
        );
        compare(&p, false);
    }
    pub(crate) fn compare(p: &Program, generic: bool) {
        let m = crate::features::Model::build(p, None, generic);
        let oracle = crate::growth::Growth::build(p, &m, None);
        let mut model = Model::build(p, None, generic, Some(m.abs.clone()), true);
        let actual = Growth::build(p, &mut model, true, true);
        assert_eq!(actual.nodes, oracle.nodes);
        for i in 0..oracle.nodes.len() {
            assert_eq!(actual.graph.row(i), oracle.graph.edges[i]);
            for edge in &oracle.graph.edges[i] {
                assert_eq!(
                    actual.supports(i, edge.to).collect::<Vec<_>>(),
                    oracle.supports(i, edge.to).collect::<Vec<_>>()
                );
            }
        }
        let sources = actual
            .var_sources
            .iter()
            .map(|(s, v)| (*s, v.iter().copied().collect::<BTreeSet<_>>()))
            .collect::<HashMap<_, _>>();
        assert_eq!(sources, oracle.var_sources);
        assert_eq!(actual.admitted(), oracle.admitted());
        assert_eq!(actual.precision_fallbacks, oracle.precision_fallbacks);
        assert_eq!(actual.source_patterns, oracle.source_patterns);
        assert_eq!(actual.source_cache_hits, oracle.source_cache_hits);
        assert_eq!(actual.stats.skipped, actual.stats.audited);
    }
    #[test]
    fn source_flow_matches_oracle_on_plants_and_forwarding_cycles() {
        for text in [
            include_str!("../plants/G1.pl.in"),
            include_str!("../plants/G2.pl.in"),
            include_str!("../plants/L4.pl.in"),
            "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\nguideline_entity(actual,X,end,countable) :- guideline_entity(actual,X,next,countable).\nguideline_entity(actual,'$guideline_id'(product,d,2,ref(1),[X]),out,countable) :- guideline_entity(actual,X,end,countable).\n",
        ] {
            let mut p = Program::new();
            p.add("fixture".into(), text.into(), false).unwrap();
            for generic in [true, false] {
                compare(&p, generic);
            }
        }
    }
}
