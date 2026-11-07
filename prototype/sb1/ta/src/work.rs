//! Constructive-work diagnostics; immutable content IDs retain exact equality.
use crate::census::{Metrics, domain_values, literal_key};
use crate::domain::{Abstraction, Domain, Value};
use crate::features::{Feature, Label, Pattern, State};
use sb1_checker::term::{Arena, Id};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_TRACE: AtomicUsize = AtomicUsize::new(1);
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Resource {
    Unary(Feature),
    Family(Feature),
    Targets(Value, Label, Value),
    Owners(Value, Label),
    Scalars(Label),
    LinkRows,
}
impl Resource {
    fn label(self, arena: &Arena, abs: &Abstraction) -> String {
        match self {
            Self::Unary(f) => format!("unary:{}", crate::detail::feature_label(arena, abs, f)),
            Self::Family(f) => format!("family:{}", crate::detail::feature_label(arena, abs, f)),
            Self::Targets(source, label, owner) => format!(
                "targets(owner={},source={},label={}:{})",
                abs.describe(arena, owner),
                abs.describe(arena, source),
                arena.name(label.0),
                abs.describe(arena, label.1)
            ),
            Self::Owners(source, label) => format!(
                "owners(source={},label={}:{})",
                abs.describe(arena, source),
                arena.name(label.0),
                abs.describe(arena, label.1)
            ),
            Self::Scalars(label) => format!(
                "scalar-sources({}:{})",
                arena.name(label.0),
                abs.describe(arena, label.1)
            ),
            Self::LinkRows => "all-link-row-keys".into(),
        }
    }
}
#[derive(Default)]
struct Pool {
    values: Vec<Vec<Value>>,
    ids: HashMap<Vec<Value>, usize>,
    hashed_versions: usize,
    hashed_memberships: usize,
}
impl Pool {
    fn intern(&mut self, mut values: Vec<Value>) -> usize {
        self.hashed_versions += 1;
        self.hashed_memberships += values.len();
        values.sort_unstable();
        values.dedup();
        if let Some(id) = self.ids.get(&values) {
            return *id;
        }
        let id = self.values.len();
        self.values.push(values.clone());
        self.ids.insert(values, id);
        id
    }
}
#[derive(Clone, Eq, Hash, PartialEq)]
struct StateKey {
    parents: Vec<usize>,
    domains: Vec<usize>,
    all: Vec<bool>,
}
#[derive(Eq, Hash, PartialEq)]
struct Evaluation {
    pattern: usize,
    inputs: Vec<(Resource, usize)>,
}
#[derive(Clone, Copy)]
pub(crate) enum AccessKind {
    Enumerated = 1,
    Membership = 2,
    SetOperand = 4,
}
#[derive(Clone, Copy)]
struct Input {
    id: usize,
    modes: u8,
}
#[derive(Clone, Copy, Default)]
struct AccessCounts {
    enumerated_visits: usize,
    membership_tests: usize,
    operand_calls: usize,
}
struct AccessSummary {
    ordinal: usize,
    occurrences: usize,
    modes: Vec<u8>,
    max_counts: AccessCounts,
    mode_expansions: usize,
}
#[derive(Default)]
struct Evaluations {
    raw_patterns: HashMap<Vec<Id>, usize>,
    patterns: HashMap<Vec<usize>, usize>,
    current: Option<(usize, BTreeMap<Resource, Input>, AccessCounts)>,
    keys: HashMap<Evaluation, AccessSummary>,
    total: usize,
    input_memberships: usize,
    distinct_input_memberships: usize,
    reads: usize,
    output_memberships: usize,
    distinct_key_cells: usize,
}
#[derive(Default)]
struct Candidates {
    requests: usize,
    scans: usize,
    heads: usize,
    keys: HashSet<(Id, usize)>,
    distinct_scans: usize,
    distinct_heads: usize,
}
pub(crate) struct Trace {
    token: usize,
    pool: Pool,
    current_versions: HashMap<Resource, usize>,
    link_keys: HashMap<(Value, Label), usize>,
    evaluations: Evaluations,
    states: HashMap<StateKey, usize>,
    state_memberships: Vec<usize>,
    phase: &'static str,
    candidates: HashMap<&'static str, Candidates>,
}
impl Default for Trace {
    fn default() -> Self {
        Self {
            token: NEXT_TRACE.fetch_add(1, Ordering::Relaxed),
            pool: Pool::default(),
            current_versions: HashMap::new(),
            link_keys: HashMap::new(),
            evaluations: Evaluations::default(),
            states: HashMap::new(),
            state_memberships: Vec::new(),
            phase: "",
            candidates: HashMap::new(),
        }
    }
}
impl Trace {
    pub(crate) fn phase(&mut self, phase: &'static str) {
        self.phase = phase;
    }
    pub(crate) fn changed(&mut self, key: Resource) {
        self.current_versions.remove(&key);
    }
    pub(crate) fn read(&mut self, key: Resource, values: impl FnOnce() -> Vec<Value>) {
        let Some((_, inputs, _)) = &mut self.evaluations.current else {
            return;
        };
        self.evaluations.reads += 1;
        let id = *self
            .current_versions
            .entry(key)
            .or_insert_with(|| self.pool.intern(values()));
        inputs
            .entry(key)
            .and_modify(|input| input.id = id)
            .or_insert(Input { id, modes: 0 });
    }
    pub(crate) fn access(&mut self, key: Resource, kind: AccessKind, visits: usize) {
        let Some((_, inputs, counts)) = &mut self.evaluations.current else {
            return;
        };
        let input = inputs.get_mut(&key).expect("access follows version read");
        input.modes |= kind as u8;
        match kind {
            AccessKind::Enumerated => counts.enumerated_visits += visits,
            AccessKind::Membership => counts.membership_tests += visits,
            AccessKind::SetOperand => counts.operand_calls += visits,
        }
    }
    pub(crate) fn read_link_rows<'a>(&mut self, keys: impl Iterator<Item = &'a (Value, Label)>) {
        if self.evaluations.current.is_none() {
            return;
        }
        if !self.current_versions.contains_key(&Resource::LinkRows) {
            let values = keys
                .map(|key| {
                    let next = self.link_keys.len();
                    Value::Origin(*self.link_keys.entry(*key).or_insert(next))
                })
                .collect();
            let id = self.pool.intern(values);
            self.current_versions.insert(Resource::LinkRows, id);
        }
        self.read(Resource::LinkRows, || unreachable!());
        if crate::detail::enabled() {
            let width = self.pool.values[self.current_versions[&Resource::LinkRows]].len();
            self.access(Resource::LinkRows, AccessKind::Enumerated, width);
        }
    }
    pub(crate) fn begin(&mut self, arena: &Arena, abs: &Abstraction, pattern: &Pattern) {
        let evaluations = &mut self.evaluations;
        let id = if let Some(id) = evaluations.raw_patterns.get(&pattern.goals) {
            *id
        } else {
            let mut key = vec![pattern.goals.len()];
            let mut vars = HashMap::new();
            for goal in &pattern.goals {
                literal_key(arena, abs, *goal, &mut vars, &mut key);
            }
            let next = evaluations.patterns.len();
            let id = *evaluations.patterns.entry(key).or_insert(next);
            evaluations.raw_patterns.insert(pattern.goals.clone(), id);
            id
        };
        assert!(evaluations.current.is_none());
        evaluations.current = Some((id, BTreeMap::new(), AccessCounts::default()));
    }
    pub(crate) fn end(&mut self, state: Option<&State>) {
        let (pattern, inputs, counts) = self
            .evaluations
            .current
            .take()
            .expect("census evaluation begin");
        let size = inputs
            .values()
            .map(|input| self.pool.values[input.id].len())
            .sum::<usize>();
        let modes = inputs.values().map(|input| input.modes).collect::<Vec<_>>();
        let evaluation = Evaluation {
            pattern,
            inputs: inputs
                .into_iter()
                .map(|(key, input)| (key, input.id))
                .collect(),
        };
        self.evaluations.total += 1;
        self.evaluations.input_memberships += size;
        if let Some(state) = state {
            self.evaluations.output_memberships += state
                .census_parts()
                .1
                .iter()
                .map(|d| d.values().map_or(1, |s| s.len()))
                .sum::<usize>();
        }
        if let Some(summary) = self.evaluations.keys.get_mut(&evaluation) {
            summary.occurrences += 1;
            for (old, new) in summary.modes.iter_mut().zip(modes) {
                if new & !*old != 0 {
                    summary.mode_expansions += 1;
                }
                *old |= new;
            }
            summary.max_counts.enumerated_visits = summary
                .max_counts
                .enumerated_visits
                .max(counts.enumerated_visits);
            summary.max_counts.membership_tests = summary
                .max_counts
                .membership_tests
                .max(counts.membership_tests);
            summary.max_counts.operand_calls =
                summary.max_counts.operand_calls.max(counts.operand_calls);
        } else {
            self.evaluations.distinct_input_memberships += size;
            self.evaluations.distinct_key_cells += evaluation.inputs.len();
            let ordinal = self.evaluations.keys.len();
            self.evaluations.keys.insert(
                evaluation,
                AccessSummary {
                    ordinal,
                    occurrences: 1,
                    modes,
                    max_counts: counts,
                    mode_expansions: 0,
                },
            );
        }
    }
    pub(crate) fn state(&mut self, state: &State) -> usize {
        if let Some(id) = state.census_cached(self.token) {
            return id;
        }
        let (parents, domains) = state.census_parts();
        let mut memberships = 0;
        let all = domains
            .iter()
            .map(|domain| matches!(domain, Domain::All))
            .collect();
        let domains = domains
            .iter()
            .map(|domain| {
                let values = domain_values(domain);
                memberships += values.len();
                self.pool.intern(values)
            })
            .collect::<Vec<_>>();
        let key = StateKey {
            parents: parents.to_vec(),
            domains,
            all,
        };
        let next = self.states.len();
        let id = *self.states.entry(key).or_insert_with(|| {
            self.state_memberships.push(memberships);
            next
        });
        state.census_cache(self.token, id);
        id
    }
    pub(crate) fn candidates(&mut self, goal: Id, state: &State, scans: usize, heads: usize) {
        let state = self.state(state);
        let candidates = self.candidates.entry(self.phase).or_default();
        candidates.requests += 1;
        candidates.scans += scans;
        candidates.heads += heads;
        if candidates.keys.insert((goal, state)) {
            candidates.distinct_scans += scans;
            candidates.distinct_heads += heads;
        }
    }
    pub(crate) fn write_access(
        &self,
        out: &Path,
        name: &str,
        arena: &Arena,
        abs: &Abstraction,
    ) -> Result<(), String> {
        if !crate::detail::enabled() {
            return Ok(());
        }
        let mut file = std::io::BufWriter::new(
            std::fs::File::create(out.join(format!("access-{name}.tsv")))
                .map_err(|e| e.to_string())?,
        );
        writeln!(file, "evaluation\tpattern\toccurrences\tinputs\tinput_memberships\texplicit_enumerated_width\tset_operand_width\tenumerated_or_operand_width\tmembership_tested_width\toverlap_width\tidentity_only_width\texplicit_enumerated_visits_max\tmembership_tests_max\tset_operand_calls_max\twidest_enumerated_or_operand\twidest_enumerated_or_operand_width\twidest_explicit_enumerated\twidest_explicit_enumerated_width\tmode_expansions").map_err(|e| e.to_string())?;
        let mut rows = self.evaluations.keys.iter().collect::<Vec<_>>();
        rows.sort_by_key(|(_, summary)| summary.ordinal);
        for (key, summary) in rows {
            let mut sizes = [0; 7];
            let mut widest: [Option<(Resource, usize)>; 2] = [None, None];
            for ((resource, id), modes) in key.inputs.iter().zip(&summary.modes) {
                let size = self.pool.values[*id].len();
                sizes[0] += size;
                if modes & 1 != 0 {
                    sizes[1] += size;
                }
                if modes & 4 != 0 {
                    sizes[2] += size;
                }
                if modes & 5 != 0 {
                    sizes[3] += size;
                }
                if modes & 2 != 0 {
                    sizes[4] += size;
                }
                if modes & 5 != 0 && modes & 2 != 0 {
                    sizes[5] += size;
                }
                if *modes == 0 {
                    sizes[6] += size;
                }
                for (at, mask) in [5, 1].into_iter().enumerate() {
                    if modes & mask != 0
                        && widest[at].is_none_or(|(old, width)| {
                            size > width || size == width && *resource < old
                        })
                    {
                        widest[at] = Some((*resource, size));
                    }
                }
            }
            let describe = |item: Option<(Resource, usize)>| {
                item.map_or_else(
                    || ("none".into(), 0),
                    |(key, n)| (crate::detail::field(&key.label(arena, abs)), n),
                )
            };
            let (all_label, all_width) = describe(widest[0]);
            let (explicit_label, explicit_width) = describe(widest[1]);
            writeln!(
                file,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                summary.ordinal,
                key.pattern,
                summary.occurrences,
                key.inputs.len(),
                sizes[0],
                sizes[1],
                sizes[2],
                sizes[3],
                sizes[4],
                sizes[5],
                sizes[6],
                summary.max_counts.enumerated_visits,
                summary.max_counts.membership_tests,
                summary.max_counts.operand_calls,
                all_label,
                all_width,
                explicit_label,
                explicit_width,
                summary.mode_expansions
            )
            .map_err(|e| e.to_string())?;
        }
        file.flush().map_err(|e| e.to_string())
    }
    pub(crate) fn metrics(&self, out: &mut Metrics) {
        for (key, value) in [
            ("evaluations.total", self.evaluations.total),
            ("evaluations.distinct", self.evaluations.keys.len()),
            (
                "evaluations.input_memberships",
                self.evaluations.input_memberships,
            ),
            (
                "evaluations.distinct_input_memberships",
                self.evaluations.distinct_input_memberships,
            ),
            ("evaluations.read_calls", self.evaluations.reads),
            (
                "evaluations.output_memberships",
                self.evaluations.output_memberships,
            ),
            ("evaluations.key_cells", self.evaluations.distinct_key_cells),
            ("versions.current", self.current_versions.len()),
            ("versions.retained", self.pool.values.len()),
            (
                "versions.retained_memberships",
                self.pool.values.iter().map(Vec::len).sum(),
            ),
            ("versions.hashed", self.pool.hashed_versions),
            ("versions.hashed_memberships", self.pool.hashed_memberships),
            ("states.distinct", self.states.len()),
            (
                "states.key_cells",
                self.states
                    .keys()
                    .map(|s| s.parents.len() + s.domains.len() + s.all.len())
                    .sum(),
            ),
            (
                "states.distinct_memberships",
                self.state_memberships.iter().sum(),
            ),
        ] {
            out.add(key, value);
        }
        for (phase, candidates) in &self.candidates {
            for (key, value) in [
                ("requests", candidates.requests),
                ("distinct", candidates.keys.len()),
                ("scanned_postings", candidates.scans),
                ("heads", candidates.heads),
                ("distinct_scanned_postings", candidates.distinct_scans),
                ("distinct_heads", candidates.distinct_heads),
            ] {
                out.add(&format!("candidates.{phase}.{key}"), value);
            }
        }
    }
}

#[derive(Default)]
struct Phases {
    phase: &'static str,
    forced: Option<crate::detail::ForcedWork>,
    counts: BTreeMap<(&'static str, &'static str), usize>,
}
thread_local! {
    static PHASES: RefCell<Phases> = RefCell::new(Phases::default());
}
pub fn phase(phase: &'static str) {
    if crate::census::enabled() {
        PHASES.with_borrow_mut(|p| p.phase = phase);
    }
}
pub(crate) fn forced_begin() {
    PHASES.with_borrow_mut(|p| {
        assert!(p.forced.is_none());
        p.forced = Some(crate::detail::ForcedWork::default());
    });
}
pub(crate) fn forced_end() -> crate::detail::ForcedWork {
    PHASES.with_borrow_mut(|p| p.forced.take().expect("forced-work begin"))
}
pub(crate) fn visit(key: &'static str, count: usize) {
    if crate::census::enabled() {
        PHASES.with_borrow_mut(|p| {
            *p.counts.entry((p.phase, key)).or_default() += count;
            if let Some(forced) = &mut p.forced {
                forced.visit(key, count);
            }
        });
    }
}
pub(crate) fn peak(key: &'static str, count: usize) {
    if crate::census::enabled() {
        PHASES.with_borrow_mut(|p| {
            let value = p.counts.entry((p.phase, key)).or_default();
            *value = (*value).max(count);
        });
    }
}
pub fn write(out: &Path) -> Result<(), String> {
    if !crate::census::enabled() {
        return Ok(());
    }
    let mut metrics = Metrics::default();
    PHASES.with_borrow(|p| {
        for ((phase, key), value) in &p.counts {
            metrics.add(&format!("{phase}.{key}"), *value);
        }
    });
    metrics.write(out, "work")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::Expr;
    #[test]
    fn evaluation_keys_reuse_each_version_and_distinguish_a_changed_input() {
        let arena = Arena::default();
        let abs = Abstraction::new(&arena);
        let pattern = Pattern::new(&arena, &abs, Vec::new(), 0);
        let key = Resource::Unary(Feature::Noun(Value::Origin(0)));
        let mut trace = Trace::default();
        for _ in 0..2 {
            trace.begin(&arena, &abs, &pattern);
            trace.read(key, || vec![Value::Origin(0), Value::Origin(1)]);
            trace.read(key, || panic!("a stored version hashes once"));
            trace.end(None);
        }
        assert_eq!(trace.pool.hashed_versions, 1);
        assert_eq!(trace.evaluations.total, 2);
        assert_eq!(trace.evaluations.keys.len(), 1);
        assert_eq!(trace.evaluations.input_memberships, 4);
        assert_eq!(trace.evaluations.distinct_input_memberships, 2);
        trace.changed(key);
        trace.begin(&arena, &abs, &pattern);
        trace.read(key, || {
            vec![Value::Origin(0), Value::Origin(1), Value::Origin(2)]
        });
        trace.end(None);
        assert_eq!(trace.pool.hashed_versions, 2);
        assert_eq!(trace.evaluations.keys.len(), 2);
        assert_eq!(trace.evaluations.input_memberships, 7);
        assert_eq!(trace.evaluations.distinct_input_memberships, 5);
    }
    #[test]
    fn input_access_modes_distinguish_enumeration_point_tests_and_set_operands() {
        let arena = Arena::default();
        let abs = Abstraction::new(&arena);
        let pattern = Pattern::new(&arena, &abs, Vec::new(), 0);
        let a = Resource::Unary(Feature::Noun(Value::Origin(0)));
        let b = Resource::Unary(Feature::Noun(Value::Origin(1)));
        let c = Resource::Targets(Value::Origin(0), Label(0, Value::Exact(0)), Value::Patient);
        let mut trace = Trace::default();
        trace.begin(&arena, &abs, &pattern);
        trace.read(a, || (0..3).map(Value::Origin).collect());
        trace.read(b, || (0..5).map(Value::Origin).collect());
        trace.read(c, || (0..7).map(Value::Origin).collect());
        trace.access(a, AccessKind::Enumerated, 3);
        trace.access(b, AccessKind::Membership, 2);
        trace.access(c, AccessKind::SetOperand, 1);
        trace.end(None);
        let summary = trace.evaluations.keys.values().next().unwrap();
        assert_eq!(summary.modes, [1, 2, 4]);
        assert_eq!(summary.max_counts.enumerated_visits, 3);
        assert_eq!(summary.max_counts.membership_tests, 2);
        assert_eq!(summary.max_counts.operand_calls, 1);
        trace.begin(&arena, &abs, &pattern);
        for key in [a, b, c] {
            trace.read(key, || panic!("unchanged input versions stay cached"));
        }
        trace.access(a, AccessKind::Membership, 1);
        trace.end(None);
        let summary = trace.evaluations.keys.values().next().unwrap();
        assert_eq!(summary.occurrences, 2);
        assert_eq!(summary.modes, [3, 2, 4]);
        assert_eq!(summary.mode_expansions, 1);
        assert_eq!(trace.evaluations.distinct_input_memberships, 15);
    }
    #[test]
    fn full_state_key_invalidates_on_constraint_and_preserves_clone_equality() {
        let arena = Arena::default();
        let mut trace = Trace::default();
        let mut state = State::new(1);
        let all = trace.state(&state);
        assert_eq!(trace.state(&state), all);
        assert_eq!(trace.pool.hashed_versions, 1);
        assert!(state.constrain(Expr::Var(0), &Domain::one(Value::Origin(0)), &arena));
        let narrowed = trace.state(&state);
        assert_ne!(all, narrowed);
        let mut clone = state.clone();
        assert_eq!(clone, state);
        assert_eq!(trace.state(&clone), narrowed);
        assert!(!clone.constrain(Expr::Var(0), &Domain::one(Value::Origin(1)), &arena));
        assert_ne!(trace.state(&clone), narrowed);
        assert_eq!(trace.state(&state), narrowed);
    }
}
