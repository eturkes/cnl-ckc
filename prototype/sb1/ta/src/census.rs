//! Optional structure census of the unchanged, explicit oracle.
use crate::domain::{Abstraction, Domain, Value};
use crate::features::{Label, Pattern};
use crate::graph::Graph;
use sb1_checker::term::{Arena, Id, Node};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(false);
pub fn enable() {
    ENABLED.store(true, Ordering::Relaxed);
}
pub(crate) fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

#[derive(Debug, Default, PartialEq)]
struct Sets {
    rows: usize,
    flat: usize,
    distinct: usize,
    distinct_members: usize,
    universe: usize,
    atoms: usize,
    distinct_atom_refs: usize,
    stored_atom_refs: usize,
}
impl Sets {
    fn of<T: Clone + Eq + Hash + Ord>(rows: impl IntoIterator<Item = Vec<T>>) -> Self {
        let mut counts = HashMap::<Vec<T>, usize>::new();
        let mut out = Self::default();
        for mut row in rows {
            row.sort_unstable();
            row.dedup();
            out.rows += 1;
            out.flat += row.len();
            *counts.entry(row).or_default() += 1;
        }
        out.distinct = counts.len();
        let mut signatures = HashMap::<T, Vec<usize>>::new();
        let mut multiplicities = Vec::new();
        for (id, (row, count)) in counts.into_iter().enumerate() {
            multiplicities.push(count);
            out.distinct_members += row.len();
            for value in row {
                signatures.entry(value).or_default().push(id);
            }
        }
        out.universe = signatures.len();
        let atoms = signatures.into_values().collect::<HashSet<_>>();
        out.atoms = atoms.len();
        out.distinct_atom_refs = atoms.iter().map(Vec::len).sum();
        out.stored_atom_refs = atoms
            .iter()
            .flat_map(|signature| signature.iter())
            .map(|id| multiplicities[*id])
            .sum();
        out
    }
    fn metrics(&self, prefix: &str, out: &mut Metrics) {
        for (key, value) in [
            ("rows", self.rows),
            ("flat", self.flat),
            ("distinct", self.distinct),
            ("distinct_members", self.distinct_members),
            ("universe", self.universe),
            ("atoms", self.atoms),
            ("distinct_atom_refs", self.distinct_atom_refs),
            ("stored_atom_refs", self.stored_atom_refs),
            ("atom_cost", self.universe + self.stored_atom_refs),
            (
                "intern_atom_cost",
                self.universe + self.distinct_atom_refs + self.rows,
            ),
        ] {
            out.add(&format!("{prefix}.{key}"), value);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Rectangle {
    owner: usize,
    source: usize,
    label: Label,
    target: usize,
}
#[derive(Default)]
pub(crate) struct LinkWrites {
    domains: Vec<Vec<Value>>,
    domain_ids: HashMap<Vec<Value>, usize>,
    latest: HashMap<(usize, bool, Label), Rectangle>,
    writes: usize,
}
fn values(domain: &Domain) -> Vec<Value> {
    match domain {
        Domain::All => vec![Value::Any],
        Domain::Set(set) => set.iter().copied().collect(),
    }
}
impl LinkWrites {
    fn intern(&mut self, domain: Vec<Value>) -> usize {
        if let Some(id) = self.domain_ids.get(&domain) {
            return *id;
        }
        let id = self.domains.len();
        self.domains.push(domain.clone());
        self.domain_ids.insert(domain, id);
        id
    }
    pub(crate) fn record(
        &mut self,
        clause: usize,
        seed: bool,
        owner: Vec<Value>,
        source: Vec<Value>,
        label: Label,
        target: &Domain,
    ) {
        self.writes += 1;
        let rectangle = Rectangle {
            owner: self.intern(owner),
            source: self.intern(source),
            label,
            target: self.intern(values(target)),
        };
        self.latest.insert((clause, seed, label), rectangle);
    }
    fn metrics(
        &self,
        links: &HashMap<(Value, Label), HashMap<Value, Domain>>,
        out: &mut Metrics,
    ) -> Result<(), String> {
        let distinct = self.latest.values().copied().collect::<BTreeSet<_>>();
        let mut replay = HashMap::<(Value, Label), HashMap<Value, Domain>>::new();
        let mut used = BTreeSet::new();
        let mut expanded = 0;
        for rect in &distinct {
            used.extend([rect.owner, rect.source, rect.target]);
            expanded += self.domains[rect.owner].len()
                * self.domains[rect.source].len()
                * self.domains[rect.target].len();
            let target = if self.domains[rect.target] == [Value::Any] {
                Domain::All
            } else {
                Domain::Set(self.domains[rect.target].iter().copied().collect())
            };
            for source in &self.domains[rect.source] {
                for owner in &self.domains[rect.owner] {
                    replay
                        .entry((*source, rect.label))
                        .or_default()
                        .entry(*owner)
                        .or_insert_with(Domain::empty)
                        .union(&target);
                }
            }
        }
        let exact = replay == *links;
        out.add("links.rectangle_writes", self.writes);
        out.add("links.head_rectangles", self.latest.len());
        out.add("links.distinct_rectangles", distinct.len());
        out.add("links.rectangle_expansion", expanded);
        out.add("links.rectangle_exact", usize::from(exact));
        let sets = Sets::of(used.into_iter().map(|id| self.domains[id].clone()));
        sets.metrics("links.rectangle_domains", out);
        out.add(
            "links.rectangle_cost",
            4 * distinct.len() + sets.distinct_members,
        );
        out.add(
            "links.rectangle_atom_cost",
            4 * distinct.len() + sets.universe + sets.distinct_atom_refs,
        );
        if exact {
            Ok(())
        } else {
            Err("census: latest head rectangles do not reconstruct the stored link model".into())
        }
    }
}

#[derive(Default)]
struct Probes {
    total: usize,
    raw_patterns: HashMap<Vec<Id>, HashMap<Id, (usize, usize)>>,
    raw_pattern_count: usize,
    patterns: HashMap<Vec<usize>, usize>,
    raw_heads: HashMap<Id, usize>,
    heads: HashMap<Vec<usize>, usize>,
    pairs: HashSet<(usize, usize)>,
    full_pairs: HashSet<(usize, usize, usize)>,
}
fn value_key(value: Value, out: &mut Vec<usize>) {
    match value {
        Value::Any => out.extend([0, 0]),
        Value::Patient => out.extend([1, 0]),
        Value::Number => out.extend([2, 0]),
        Value::Origin(id) => out.extend([3, id]),
        Value::Exact(id) => out.extend([4, id]),
    }
}
pub(crate) fn literal_key(
    arena: &Arena,
    abs: &Abstraction,
    term: Id,
    vars: &mut HashMap<usize, usize>,
    out: &mut Vec<usize>,
) {
    let (predicate, args) = arena.call(term).unwrap();
    out.extend([predicate, args.len()]);
    for arg in args {
        if let Node::Var(var) = arena.node(*arg) {
            let next = vars.len();
            out.extend([0, *vars.entry(*var).or_insert(next)]);
        } else {
            out.push(1);
            value_key(abs.value(*arg), out);
        }
    }
}
impl Probes {
    fn record(
        &mut self,
        arena: &Arena,
        abs: &Abstraction,
        pattern: &Pattern,
        goal: Id,
        head: Id,
    ) -> (usize, usize, usize) {
        self.total += 1;
        let (pattern_id, raw_pattern_id) = if let Some(id) = self
            .raw_patterns
            .get(&pattern.goals)
            .and_then(|p| p.get(&goal))
        {
            *id
        } else {
            let mut key = vec![pattern.goals.len()];
            let mut vars = HashMap::new();
            for term in &pattern.goals {
                literal_key(arena, abs, *term, &mut vars, &mut key);
            }
            literal_key(arena, abs, goal, &mut vars, &mut key);
            let next = self.patterns.len();
            let id = *self.patterns.entry(key).or_insert(next);
            let raw = self.raw_pattern_count;
            self.raw_pattern_count += 1;
            self.raw_patterns
                .entry(pattern.goals.clone())
                .or_default()
                .insert(goal, (id, raw));
            (id, raw)
        };
        let head_id = if let Some(id) = self.raw_heads.get(&head) {
            *id
        } else {
            let mut key = Vec::new();
            literal_key(arena, abs, head, &mut HashMap::new(), &mut key);
            let next = self.heads.len();
            let id = *self.heads.entry(key).or_insert(next);
            self.raw_heads.insert(head, id);
            id
        };
        self.pairs.insert((pattern_id, head_id));
        (pattern_id, head_id, raw_pattern_id)
    }
}

#[derive(Default)]
pub(crate) struct Trace {
    pub(crate) links: LinkWrites,
    pub(crate) work: crate::work::Trace,
    pub(crate) details: Option<crate::detail::Details>,
    phase: &'static str,
    probes: HashMap<&'static str, Probes>,
}
impl Trace {
    pub(crate) fn new(program: &crate::source::Program) -> Self {
        Self {
            details: crate::detail::enabled().then(|| crate::detail::Details::new(program)),
            ..Self::default()
        }
    }
    pub(crate) fn caller(&mut self, program: &crate::source::Program, clause: usize) {
        if let Some(details) = &mut self.details {
            details.caller(program, clause);
        }
    }
    pub(crate) fn begin_forced(&mut self, program: &crate::source::Program, producer: usize) {
        if let Some(details) = &mut self.details {
            details.begin(program, producer, self.phase);
        }
    }
    pub(crate) fn fail(&mut self, outcome: crate::detail::Outcome) {
        if let Some(details) = &mut self.details {
            details.fail(outcome);
        }
    }
    pub(crate) fn finish_forced(&mut self, outcome: Option<crate::detail::Outcome>) {
        if let Some(details) = &mut self.details {
            let outcome = outcome.unwrap_or_else(|| details.last_failure());
            details.finish(outcome);
        }
    }
    pub(crate) fn phase(&mut self, phase: &'static str) {
        self.phase = phase;
        self.work.phase(phase);
    }
    pub(crate) fn probe(
        &mut self,
        arena: &Arena,
        abs: &Abstraction,
        pattern: &Pattern,
        goal: Id,
        head: Id,
        base: &crate::features::State,
    ) {
        let state = self.work.state(base);
        let probes = self.probes.entry(self.phase).or_default();
        let (_, head, raw_pattern) = probes.record(arena, abs, pattern, goal, head);
        probes.full_pairs.insert((raw_pattern, head, state));
    }
    pub(crate) fn metrics(
        &self,
        links: &HashMap<(Value, Label), HashMap<Value, Domain>>,
        out: &mut Metrics,
    ) -> Result<(), String> {
        self.links.metrics(links, out)?;
        self.work.metrics(out);
        for (phase, probes) in &self.probes {
            for (key, value) in [
                ("total", probes.total),
                ("distinct", probes.pairs.len()),
                ("full_distinct", probes.full_pairs.len()),
                ("full_key_cells", probes.full_pairs.len() * 3),
                ("raw_patterns", probes.raw_pattern_count),
                ("patterns", probes.patterns.len()),
                ("heads", probes.heads.len()),
            ] {
                out.add(&format!("probes.{phase}.{key}"), value);
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct Metrics(Vec<(String, usize)>);
impl Metrics {
    pub(crate) fn add(&mut self, key: &str, value: usize) {
        self.0.push((key.into(), value));
    }
    pub(crate) fn sets<T: Clone + Eq + Hash + Ord>(
        &mut self,
        key: &str,
        rows: impl IntoIterator<Item = Vec<T>>,
    ) {
        Sets::of(rows).metrics(key, self);
    }
    pub(crate) fn write(mut self, out: &Path, name: &str) -> Result<(), String> {
        self.0.sort_unstable();
        let path = out.join(format!("census-{name}.tsv"));
        let mut file =
            std::io::BufWriter::new(std::fs::File::create(&path).map_err(|e| e.to_string())?);
        writeln!(file, "metric\tvalue").map_err(|e| e.to_string())?;
        for (key, value) in self.0 {
            writeln!(file, "{key}\t{value}").map_err(|e| e.to_string())?;
        }
        file.flush().map_err(|e| e.to_string())
    }
}
pub fn graph(out: &Path, name: &str, graph: &Graph) -> Result<(), String> {
    if !enabled() {
        return Ok(());
    }
    let mut metrics = Metrics::default();
    metrics.add("nodes", graph.edges.len());
    metrics.sets("adjacency", graph.edges.iter().cloned());
    let mut callers = Vec::new();
    for row in &graph.edges {
        let mut groups = HashMap::<(bool, bool), Vec<usize>>::new();
        for edge in row {
            groups
                .entry((edge.negative, edge.growth))
                .or_default()
                .push(edge.to);
        }
        callers.extend(groups.into_values());
    }
    let rows = Sets::of(callers);
    rows.metrics("signed_targets", &mut metrics);
    metrics.add("hub_caller_associations", rows.rows);
    metrics.add("hub_dag_arcs", 0);
    metrics.add("hub_leaf_memberships", rows.distinct_members);
    metrics.add("atom_hub_caller_associations", rows.rows);
    metrics.add("atom_hub_dag_arcs", rows.distinct_atom_refs);
    metrics.add("atom_hub_leaf_memberships", rows.universe);
    metrics.add("hub_scc_live_arcs", 2 * (rows.rows + rows.distinct_members));
    metrics.add(
        "atom_hub_scc_live_arcs",
        2 * (rows.rows + rows.universe + rows.distinct_atom_refs),
    );
    metrics.add("hub_cost", rows.rows + rows.distinct_members);
    metrics.add(
        "atom_hub_cost",
        rows.rows + rows.universe + rows.distinct_atom_refs,
    );
    metrics.add("witnesses", graph.witnesses.len());
    metrics.write(out, name)
}
pub(crate) fn domain_values(domain: &Domain) -> Vec<Value> {
    values(domain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Edge;
    #[test]
    fn atom_partition_matches_exhaustive_three_element_families() {
        for bits in 0..512 {
            let rows = (0..3)
                .map(|row| {
                    (0..3)
                        .filter(|value| bits & (1 << (3 * row + value)) != 0)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let stats = Sets::of(rows.clone());
            let signatures = (0..3)
                .map(|value| {
                    rows.iter()
                        .enumerate()
                        .filter_map(|(r, row)| row.contains(&value).then_some(r))
                        .collect::<Vec<_>>()
                })
                .filter(|s| !s.is_empty())
                .collect::<BTreeSet<_>>();
            assert_eq!(
                stats.stored_atom_refs,
                signatures.iter().map(Vec::len).sum::<usize>()
            );
            assert_eq!(stats.atoms, signatures.len());
            assert_eq!(stats.flat, rows.iter().map(Vec::len).sum::<usize>());
            assert!(stats.universe + stats.stored_atom_refs <= stats.flat + stats.universe);
        }
    }
    #[test]
    fn rectangles_replay_products_and_detect_a_missing_earlier_write() {
        let label = Label(0, Value::Exact(1));
        let owner = Value::Exact(0);
        let source = Value::Origin(0);
        let target = Domain::one(Value::Origin(1));
        let mut writes = LinkWrites::default();
        writes.record(0, false, vec![owner], vec![source], label, &target);
        let links = HashMap::from([((source, label), HashMap::from([(owner, target)]))]);
        writes.metrics(&links, &mut Metrics::default()).unwrap();
        writes.record(0, false, vec![owner], vec![source], label, &Domain::empty());
        assert!(writes.metrics(&links, &mut Metrics::default()).is_err());
    }
    #[test]
    fn probe_keys_keep_owners_roles_and_shared_variables() {
        use crate::source::Program;
        use sb1_checker::program::Goal;
        let mut program = Program::new();
        program.add("keys".into(), concat!(
            "guideline_property(actual,X,a,pos) :- guideline_event(actual,E,go), guideline_arg(actual,E,1,X).\n",
            "guideline_property(actual,B,b,pos) :- guideline_event(actual,A,go), guideline_arg(actual,A,1,B).\n",
            "guideline_property(actual,X,c,pos) :- guideline_event(actual,E,go), guideline_arg(actual,E,2,X).\n",
            "guideline_property(actual,X,d,pos) :- guideline_event(actual,E,go), guideline_arg(context,E,1,X).\n",
            "guideline_property(actual,E,e,pos) :- guideline_event(actual,E,go), guideline_arg(actual,E,1,E).\n",
        ).into(), false).unwrap();
        let abs = Abstraction::new(&program.arena);
        let mut probes = Probes::default();
        for clause in &program.clauses {
            let pattern =
                Pattern::body(&program.arena, &abs, &clause.body, clause.names.len(), None);
            let Goal::Positive(goal) = clause.body[1] else {
                unreachable!()
            };
            probes.record(
                &program.arena,
                &abs,
                &pattern,
                goal,
                program.clauses[0].head,
            );
        }
        assert_eq!(probes.total, 5);
        assert_eq!(probes.pairs.len(), 4);
    }
    #[test]
    fn full_probe_keys_keep_raw_bindings_under_alpha_renaming() {
        use crate::features::{Expr, State};
        use crate::source::Program;
        use sb1_checker::program::Goal;
        let mut program = Program::new();
        program
            .add(
                "bindings".into(),
                concat!(
                    "guideline_property(actual,A,a,pos) :- guideline_arg(actual,A,1,B).\n",
                    "guideline_property(actual,A,b,pos) :- guideline_arg(actual,B,1,A).\n",
                )
                .into(),
                false,
            )
            .unwrap();
        let abs = Abstraction::new(&program.arena);
        let mut base = State::new(2);
        base.constrain(Expr::Var(0), &Domain::one(Value::Origin(0)), &program.arena);
        base.constrain(Expr::Var(1), &Domain::one(Value::Origin(1)), &program.arena);
        let mut trace = Trace::default();
        for clause in &program.clauses {
            let pattern = Pattern::body(&program.arena, &abs, &clause.body, 2, None);
            let Goal::Positive(goal) = clause.body[0] else {
                unreachable!()
            };
            trace.probe(
                &program.arena,
                &abs,
                &pattern,
                goal,
                program.clauses[0].head,
                &base,
            );
        }
        assert_eq!(trace.probes[""].pairs.len(), 1);
        assert_eq!(trace.probes[""].full_pairs.len(), 2);
    }
    #[test]
    fn signed_hubs_count_negative_callers_separately() {
        let edge = |negative| Edge {
            to: 2,
            negative,
            growth: false,
        };
        let graph = Graph::from_edges(
            vec![vec![edge(false), edge(true)], vec![edge(false)], vec![]],
            Default::default(),
        );
        let rows = Sets::of(vec![vec![2], vec![2], vec![2]]);
        assert_eq!(graph.edge_count(), 3);
        assert_eq!(rows.rows + rows.distinct_members, 4);
        assert_eq!(rows.rows + rows.universe + rows.distinct_atom_refs, 5);
    }
}
