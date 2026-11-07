use crate::features::{Counters, Domain, Expr, Model, Pattern, State, Value};
use crate::graph::Graph;
use crate::growth::{Growth, Origin};
use crate::source::Program;
use sb1_checker::program::Goal;
use sb1_checker::term::{Id, Node, quote_atom, variable_name};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Tag {
    Base,
    Skolem(usize),
    Arithmetic(usize, usize),
}
impl Tag {
    fn term(&self) -> String {
        match self {
            Self::Base => "'$sb1_base'".into(),
            Self::Skolem(o) => format!("'$sb1_template'({o})"),
            Self::Arithmetic(c, g) => format!("'$sb1_arith'({c},{g})"),
        }
    }
    fn domain(&self) -> Domain {
        Domain::one(match self {
            Self::Base => Value::Patient,
            Self::Skolem(o) => Value::Origin(*o),
            Self::Arithmetic(..) => Value::Number,
        })
    }
    fn label(&self, p: &Program, m: &Model) -> String {
        match self {
            Self::Base => "base/patient".into(),
            Self::Skolem(o) => m.abs.describe(&p.arena, Value::Origin(*o)),
            Self::Arithmetic(c, g) => format!("arithmetic({},{})", p.label(*c), g + 1),
        }
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Key {
    pub template: usize,
    pub precise: bool,
    pub deps: Vec<Tag>,
}
#[derive(Default)]
struct Registry {
    keys: Vec<Key>,
    positions: BTreeMap<Key, usize>,
    refine: Option<BTreeSet<usize>>,
}
impl Registry {
    fn precise(&self, template: usize) -> bool {
        self.refine.as_ref().is_none_or(|s| s.contains(&template))
    }
    fn put(&mut self, key: Key) -> usize {
        if let Some(i) = self.positions.get(&key) {
            return *i;
        }
        let i = self.keys.len();
        self.positions.insert(key.clone(), i);
        self.keys.push(key);
        i
    }
}
#[derive(Clone)]
struct Variant {
    original: usize,
    tags: BTreeMap<usize, Tag>,
}
pub struct Refined {
    pub program: Program,
    pub model: Model,
    pub growth: Growth,
    pub keys: Vec<Key>,
    variants: Vec<Variant>,
    pub generated_variants: usize,
    pub candidate_assignments: usize,
    pub rejected_assignments: usize,
    pub refined_templates: usize,
    pub projection_checks: usize,
    pub full_refinement: bool,
}

fn variables(p: &Program, t: Id, out: &mut BTreeSet<usize>) {
    if let Some(a) = p.arena.named(t, "$guideline_id", 5) {
        p.arena.vars(a[4], out);
    } else if let Some((_, a)) = p.arena.call(t) {
        for t in a {
            variables(p, *t, out);
        }
    }
}
fn refined_variables(p: &Program, m: &Model, t: Id, r: &Registry, out: &mut BTreeSet<usize>) {
    if let Some(a) = p.arena.named(t, "$guideline_id", 5) {
        if let Value::Origin(o) = m.abs.value(t)
            && r.precise(o)
        {
            p.arena.vars(a[4], out);
        }
    } else if let Some((_, a)) = p.arena.call(t) {
        for t in a {
            refined_variables(p, m, *t, r, out);
        }
    }
}
fn body_has_parameter(p: &Program, goals: &[Goal]) -> bool {
    goals.iter().any(|g| match g {
        Goal::Positive(t) | Goal::Builtin(t) => {
            let mut v = BTreeSet::new();
            variables(p, *t, &mut v);
            !v.is_empty()
        }
        Goal::Not(inner) => body_has_parameter(p, inner),
    })
}
fn term(
    p: &Program,
    m: &Model,
    t: Id,
    tags: &BTreeMap<usize, Tag>,
    registry: &mut Registry,
) -> Result<String, String> {
    if let Some(a) = p.arena.named(t, "$guideline_id", 5) {
        let Value::Origin(template) = m.abs.value(t) else {
            unreachable!()
        };
        let precise = registry.precise(template);
        let deps = if precise {
            p.arena
                .list(a[4])
                .ok_or("k1: malformed Deps")?
                .into_iter()
                .map(|d| match p.arena.node(d) {
                    Node::Var(v) => tags
                        .get(v)
                        .cloned()
                        .ok_or_else(|| format!("k1: untyped Deps variable {v}")),
                    _ => Err("k1: source Deps value".into()),
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            Vec::new()
        };
        let key = registry.put(Key {
            template,
            precise,
            deps,
        });
        return Ok(format!(
            "'$guideline_id'({},'$sb1_k1'({key}),{},{},{})",
            p.arena.render(a[0]),
            p.arena.render(a[2]),
            p.arena.render(a[3]),
            p.arena.render(a[4])
        ));
    }
    if let Some((n, a)) = p.arena.call(t) {
        let parts = a
            .iter()
            .map(|t| term(p, m, *t, tags, registry))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(format!(
            "{}({})",
            quote_atom(p.arena.name(n)),
            parts.join(",")
        ));
    }
    Ok(p.arena.render(t))
}
fn goals(
    p: &Program,
    m: &Model,
    gs: &[Goal],
    tags: &BTreeMap<usize, Tag>,
    r: &mut Registry,
) -> Result<String, String> {
    gs.iter()
        .map(|g| match g {
            Goal::Positive(t) | Goal::Builtin(t) => term(p, m, *t, tags, r),
            Goal::Not(inner) => {
                let s = goals(p, m, inner, tags, r)?;
                Ok(if inner.len() == 1 {
                    format!("\\+ {s}")
                } else {
                    format!("\\+ ({s})")
                })
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|s| s.join(", "))
}
struct Choices<'a> {
    p: &'a Program,
    m: &'a Model,
    pattern: Pattern,
    vars: Vec<(usize, Vec<Tag>)>,
    counters: Counters,
    attempted: usize,
    rejected: usize,
}
impl Choices<'_> {
    fn visit(
        &mut self,
        at: usize,
        state: State,
        tags: &mut BTreeMap<usize, Tag>,
        out: &mut Vec<BTreeMap<usize, Tag>>,
    ) {
        if at == self.vars.len() {
            self.attempted += 1;
            if self
                .m
                .feature_witness(&self.p.arena, &self.pattern, state, &mut self.counters)
                .is_some()
            {
                out.push(tags.clone());
            } else {
                self.rejected += 1;
            }
            return;
        }
        let (var, options) = self.vars[at].clone();
        for tag in options {
            let mut next = state.clone();
            if next.constrain(Expr::Var(var), &tag.domain(), &self.p.arena)
                && let Some(next) =
                    self.m
                        .solve(&self.p.arena, &self.pattern, next, true, &mut self.counters)
            {
                tags.insert(var, tag);
                self.visit(at + 1, next, tags, out);
                tags.remove(&var);
            } else {
                self.rejected += 1;
            }
        }
    }
}
impl Refined {
    pub fn build(p: &Program, m: &Model, base: &Growth) -> Result<Self, String> {
        let restricted = Self::build_mode(p, m, base, true)?;
        if restricted.admitted() {
            Ok(restricted)
        } else {
            Self::build_mode(p, m, base, false)
        }
    }
    pub fn build_full(p: &Program, m: &Model, base: &Growth) -> Result<Self, String> {
        Self::build_mode(p, m, base, false)
    }
    pub fn build_restricted(p: &Program, m: &Model, base: &Growth) -> Result<Self, String> {
        Self::build_mode(p, m, base, true)
    }
    fn build_mode(p: &Program, m: &Model, base: &Growth, restricted: bool) -> Result<Self, String> {
        if base.precision_fallbacks != 0 {
            return Err(format!(
                "k1: {} unrefined nested-body bindings",
                base.precision_fallbacks
            ));
        }
        crate::work::phase("k1_expand");
        let progress = std::env::var_os("SB1_K1_PROGRESS").is_some();
        let cyclic = base
            .graph
            .witnesses
            .iter()
            .map(|w| w.component)
            .collect::<BTreeSet<_>>();
        let selected = base
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| match n {
                Origin::Skolem(o) if cyclic.contains(&base.graph.components[i]) => Some(*o),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        let refined_templates = if restricted {
            selected.len()
        } else {
            m.abs.origins.len()
        };
        let mut registry = Registry {
            refine: restricted.then_some(selected),
            ..Registry::default()
        };
        let mut ir = Program::new();
        let mut variants = Vec::new();
        let mut generated = 0;
        let mut attempted = 0;
        let mut rejected = 0;
        for doc in &p.documents {
            if m.generic_patient() && doc.patient {
                continue;
            }
            let mut text = String::new();
            let mut local = Vec::new();
            for i in doc.clauses.clone() {
                let c = &p.clauses[i];
                if body_has_parameter(p, &c.body) {
                    return Err(format!("k1: parameterized body identity at {}", p.label(i)));
                }
                let mut deps = BTreeSet::new();
                refined_variables(p, m, c.head, &registry, &mut deps);
                let assignments = if deps.is_empty() {
                    vec![BTreeMap::new()]
                } else {
                    let pattern = Pattern::body(&p.arena, &m.abs, &c.body, c.names.len(), None);
                    let mut counters = Counters::default();
                    let Some(state) = m.state(&p.arena, &pattern, &mut counters) else {
                        continue;
                    };
                    let mut choices = Vec::new();
                    for var in deps {
                        let mut options = BTreeSet::new();
                        if !state
                            .domain(Expr::Var(var))
                            .meet(&Domain::one(Value::Patient), &p.arena)
                            .is_empty()
                        {
                            options.insert(Tag::Base);
                        }
                        if let Some(sources) = base.var_sources.get(&(i, var)) {
                            for source in sources {
                                options.insert(match base.nodes[*source] {
                                    Origin::Skolem(o) => Tag::Skolem(o),
                                    Origin::Arithmetic { clause, goal } => {
                                        Tag::Arithmetic(clause, goal)
                                    }
                                });
                            }
                        }
                        choices.push((var, options.into_iter().collect::<Vec<_>>()));
                    }
                    choices.sort_by_key(|(v, t)| (t.len(), *v));
                    let mut walk = Choices {
                        p,
                        m,
                        pattern,
                        vars: choices,
                        counters: Counters::default(),
                        attempted: 0,
                        rejected: 0,
                    };
                    let mut assignments = Vec::new();
                    walk.visit(0, state, &mut BTreeMap::new(), &mut assignments);
                    attempted += walk.attempted;
                    rejected += walk.rejected;
                    assignments
                };
                for tags in assignments {
                    let head = term(p, m, c.head, &tags, &mut registry)?;
                    let mut body = goals(p, m, &c.body, &tags, &mut registry)?;
                    for (var, tag) in &tags {
                        if !body.is_empty() {
                            body.push_str(", ");
                        }
                        body.push_str(&format!(
                            "'$sb1_type'({},{})",
                            variable_name(*var),
                            tag.term()
                        ));
                    }
                    text.push_str(&head);
                    if !body.is_empty() {
                        text.push_str(" :- ");
                        text.push_str(&body);
                    }
                    text.push_str(".\n");
                    local.push(Variant { original: i, tags });
                    generated += 1;
                }
            }
            if text.is_empty() {
                continue;
            }
            let start = ir.clauses.len();
            ir.add(doc.path.clone(), text, doc.patient)?;
            if ir.clauses.len() - start != local.len() {
                return Err("k1: clause mapping".into());
            }
            for (j, variant) in local.iter().enumerate() {
                let c = &mut ir.clauses[start + j];
                let source = &p.clauses[variant.original];
                if c.names.len() != source.names.len()
                    || !c
                        .names
                        .iter()
                        .enumerate()
                        .all(|(v, n)| *n == variable_name(v))
                {
                    return Err(format!(
                        "k1: variable mapping {}",
                        p.label(variant.original)
                    ));
                }
                c.line = source.line;
                c.ordinal = source.ordinal;
                Program::release_clause_metadata(c);
                ir.sentences[start + j] = p.sentences[variant.original];
            }
            ir.documents.last_mut().unwrap().text = String::new();
            variants.extend(local);
            if progress && ir.documents.len().is_multiple_of(50) {
                eprintln!(
                    "k1 documents={} variants={generated} types={}",
                    ir.documents.len(),
                    registry.keys.len()
                );
            }
        }
        let mut metadata = String::new();
        for (index, key) in registry.keys.iter().enumerate() {
            let a = p
                .arena
                .named(m.abs.origins[key.template], "$guideline_id", 5)
                .unwrap();
            metadata.push_str(&format!(
                "'$sb1_origin'('$guideline_id'({},'$sb1_k1'({index}),{},{},[]),{}).\n",
                p.arena.render(a[0]),
                p.arena.render(a[2]),
                p.arena.render(a[3]),
                Tag::Skolem(key.template).term()
            ));
        }
        if !metadata.is_empty() {
            let start = ir.clauses.len();
            ir.add(PathBuf::from("k1-registry"), metadata, false)?;
            variants.extend((start..ir.clauses.len()).map(|_| Variant {
                original: usize::MAX,
                tags: BTreeMap::new(),
            }));
            ir.documents.last_mut().unwrap().text = String::new();
        }
        if progress {
            eprintln!(
                "k1 expansion types={} variants={generated} assignments={attempted} rejected={rejected}",
                registry.keys.len()
            );
        }
        if crate::census::enabled() {
            crate::work::peak("k1_registry_keys", registry.keys.len());
            crate::work::peak(
                "k1_registry_tag_cells",
                registry.keys.iter().map(|k| k.deps.len()).sum(),
            );
            crate::work::peak("k1_variants", variants.len());
            crate::work::peak(
                "k1_variant_tag_cells",
                variants.iter().map(|v| v.tags.len()).sum(),
            );
        }
        registry.positions = BTreeMap::new();
        registry.refine = None;
        ir.record_memory("k1-parse");
        crate::work::phase("k1_may");
        let model = Model::build_mode(&ir, None, m.generic_patient(), None, m.mode);
        if progress {
            eprintln!(
                "k1 features={} clauses={}",
                model.fact_count(),
                ir.clauses.len()
            );
        }
        let ir_types = model
            .abs
            .origins
            .iter()
            .map(|id| {
                let a = ir
                    .arena
                    .named(*id, "$guideline_id", 5)
                    .ok_or("k1: IR identity")?;
                let doc = ir
                    .arena
                    .named(a[1], "$sb1_k1", 1)
                    .ok_or("k1: IR namespace")?;
                match ir.arena.node(doc[0]) {
                    Node::Integer(n) => n.parse::<usize>().map_err(|_| "k1: IR index"),
                    _ => Err("k1: IR index"),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        let arithmetic = base
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| match n {
                Origin::Arithmetic { clause, goal } => Some(((*clause, *goal), i)),
                _ => None,
            })
            .collect::<HashMap<_, _>>();
        // A refined cycle projects inside one cyclic base SCC. Other source
        // rows inherit the base condensation rank; their features remain closed.
        model.census_phase("growth");
        crate::work::phase("k1_growth");
        let mut growth = if restricted {
            Growth::for_sources(&ir, &model, |origin| match origin {
                Origin::Skolem(o) => {
                    cyclic.contains(&base.graph.components[registry.keys[ir_types[*o]].template])
                }
                Origin::Arithmetic { clause, goal } => arithmetic
                    .get(&(variants[*clause].original, *goal))
                    .is_some_and(|node| cyclic.contains(&base.graph.components[*node])),
            })
        } else {
            Growth::build(&ir, &model, None)
        };
        if progress {
            eprintln!(
                "k1 raw flow nodes={} edges={}",
                growth.nodes.len(),
                growth.graph.edge_count()
            );
        }
        crate::work::phase("k1_filter");
        let mut edges = std::mem::take(&mut growth.graph.edges);
        for (from, row) in edges.iter_mut().enumerate() {
            row.retain(|edge| {
                let source = match growth.nodes[edge.to] {
                    Origin::Skolem(o) => Tag::Skolem(registry.keys[ir_types[o]].template),
                    Origin::Arithmetic { clause, goal } => {
                        Tag::Arithmetic(variants[clause].original, goal)
                    }
                };
                growth.supports(from, edge.to).any(|(clause, var)| {
                    variants[clause]
                        .tags
                        .get(&var)
                        .is_none_or(|expected| *expected == source)
                })
            });
        }
        let projection = growth
            .nodes
            .iter()
            .map(|n| match n {
                Origin::Skolem(o) => Ok(registry.keys[ir_types[*o]].template),
                Origin::Arithmetic { clause, goal } => arithmetic
                    .get(&(variants[*clause].original, *goal))
                    .copied()
                    .ok_or("k1: arithmetic projection"),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut projection_checks = 0;
        for (from, row) in edges.iter().enumerate() {
            for edge in row {
                projection_checks += 1;
                if base.graph.edges[projection[from]]
                    .binary_search_by_key(&projection[edge.to], |e| e.to)
                    .is_err()
                {
                    return Err(format!(
                        "k1: edge projection missing {} -> {}",
                        base.label(p, m, projection[from]),
                        base.label(p, m, projection[edge.to])
                    ));
                }
            }
        }
        growth.graph = Graph::from_edges(edges, growth.graph.counters.clone());
        Ok(Self {
            program: ir,
            model,
            growth,
            keys: registry.keys,
            variants,
            generated_variants: generated,
            candidate_assignments: attempted,
            rejected_assignments: rejected,
            refined_templates,
            projection_checks,
            full_refinement: !restricted,
        })
    }
    fn type_index(&self, origin: usize) -> usize {
        let a = self
            .program
            .arena
            .named(self.model.abs.origins[origin], "$guideline_id", 5)
            .unwrap();
        let d = self.program.arena.named(a[1], "$sb1_k1", 1).unwrap();
        let Node::Integer(i) = self.program.arena.node(d[0]) else {
            unreachable!()
        };
        i.parse().unwrap()
    }
    fn edge_site(&self, from: usize, to: usize) -> (usize, usize) {
        let source = match self.growth.nodes[to] {
            Origin::Skolem(o) => Tag::Skolem(self.keys[self.type_index(o)].template),
            Origin::Arithmetic { clause, goal } => {
                Tag::Arithmetic(self.variants[clause].original, goal)
            }
        };
        self.growth
            .supports(from, to)
            .find(|(clause, var)| {
                self.variants[*clause]
                    .tags
                    .get(var)
                    .is_none_or(|expected| *expected == source)
            })
            .expect("k1 edge support")
    }
    fn label(&self, p: &Program, m: &Model, origin: usize) -> String {
        let key = &self.keys[self.type_index(origin)];
        format!(
            "{}<{}>",
            m.abs.describe(&p.arena, Value::Origin(key.template)),
            if key.precise {
                key.deps
                    .iter()
                    .map(|t| t.label(p, m))
                    .collect::<Vec<_>>()
                    .join(";")
            } else {
                "*".into()
            }
        )
    }
    fn translations(&self, p: &Program, m: &Model) -> Vec<(String, String)> {
        let mut out = self
            .model
            .abs
            .origins
            .iter()
            .enumerate()
            .map(|(o, _)| {
                (
                    self.model
                        .abs
                        .describe(&self.program.arena, Value::Origin(o)),
                    self.label(p, m, o),
                )
            })
            .collect::<Vec<_>>();
        let mut tags = BTreeSet::new();
        for key in &self.keys {
            tags.extend(&key.deps);
        }
        for tag in tags {
            out.push((tag.term(), format!("tag({})", tag.label(p, m))));
        }
        out.sort_by_key(|(raw, _)| std::cmp::Reverse(raw.len()));
        out
    }
    fn node_label(&self, p: &Program, m: &Model, node: usize) -> String {
        match self.growth.nodes[node] {
            Origin::Skolem(o) => self.label(p, m, o),
            Origin::Arithmetic { clause, goal } => {
                let v = &self.variants[clause];
                format!(
                    "arithmetic({},{})<{}>",
                    p.label(v.original),
                    goal + 1,
                    v.tags
                        .iter()
                        .map(|(var, tag)| format!("{}={}", variable_name(*var), tag.label(p, m)))
                        .collect::<Vec<_>>()
                        .join(";")
                )
            }
        }
    }
    pub fn canonical_nodes(&self, p: &Program, m: &Model) -> Vec<String> {
        (0..self.growth.nodes.len())
            .map(|n| self.node_label(p, m, n))
            .collect()
    }
    pub fn canonical_edges(&self, p: &Program, m: &Model) -> Vec<(String, String)> {
        let mut out = Vec::new();
        for (from, row) in self.growth.graph.edges.iter().enumerate() {
            for edge in row {
                out.push((self.node_label(p, m, from), self.node_label(p, m, edge.to)));
            }
        }
        out
    }
    pub fn admitted(&self) -> bool {
        self.growth.admitted()
    }
    pub fn render_witnesses(&self, p: &Program, m: &Model) -> String {
        let mut text =
            String::from("kind\tcomponent\tstep\tfrom\tsign\tgrowth\tto\tgoal\tmatched_features\n");
        if self.growth.graph.witnesses.is_empty() {
            return text;
        }
        let translations = self.translations(p, m);
        for w in &self.growth.graph.witnesses {
            for (step, (from, edge)) in w.cycle.iter().enumerate() {
                let e =
                    Growth::evidence_at(&self.program, &self.model, self.edge_site(*from, edge.to));
                let mut counters = Counters::default();
                let value = match self.growth.nodes[edge.to] {
                    Origin::Skolem(o) => Value::Origin(o),
                    Origin::Arithmetic { .. } => Value::Number,
                };
                let state = self
                    .model
                    .bound_state(
                        &self.program.arena,
                        &e.pattern,
                        Expr::Var(e.var),
                        &Domain::one(value),
                        &mut counters,
                    )
                    .and_then(|s| {
                        self.model.feature_witness(
                            &self.program.arena,
                            &e.pattern,
                            s,
                            &mut counters,
                        )
                    });
                let mut features = state.map_or_else(
                    || "unconfirmed-forwarding-witness".into(),
                    |s| {
                        self.model
                            .describe_state(&self.program.arena, &e.pattern, &s)
                            .replace(['\n', '\t'], " | ")
                    },
                );
                for (raw, typed) in &translations {
                    features = features.replace(raw, typed);
                }
                text.push_str(&format!(
                    "growth\tk1-{}\t{}\t{}\t+\ttrue\t{}\tinput({}) @ {}\t{}\n",
                    w.component,
                    step + 1,
                    self.node_label(p, m, *from),
                    self.node_label(p, m, edge.to),
                    variable_name(e.var),
                    p.label(self.variants[e.clause].original),
                    features
                ));
            }
        }
        text
    }
    pub fn write_edges(
        &self,
        p: &Program,
        m: &Model,
        out: &mut impl std::io::Write,
    ) -> std::io::Result<()> {
        writeln!(out, "from\tto\tclause\tvariable")?;
        for (from, edges) in self.growth.graph.edges.iter().enumerate() {
            for edge in edges {
                let (clause, var) = self.edge_site(from, edge.to);
                writeln!(
                    out,
                    "{}\t{}\t{}\t{}",
                    self.node_label(p, m, from),
                    self.node_label(p, m, edge.to),
                    p.label(self.variants[clause].original),
                    variable_name(var)
                )?;
            }
        }
        Ok(())
    }
}
