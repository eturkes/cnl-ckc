//! Exact representation candidate; the explicit refined oracle stays in features.rs.
use crate::domain::{Abstraction, Counters, Value};
use crate::exact_sets::{ALL, EMPTY, Set, Sets};
use crate::features::{Expr, Feature, Label, Pattern, literal_features};
use crate::source::Program;
use sb1_checker::program::builtin;
use sb1_checker::term::{Arena, Id, Node, Symbol};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub(crate) parent: Vec<usize>,
    pub(crate) domains: Vec<Set>,
}
impl State {
    pub fn new(vars: usize) -> Self {
        Self {
            parent: (0..vars).collect(),
            domains: vec![ALL; vars],
        }
    }
    pub(crate) fn root(&self, mut v: usize) -> usize {
        while self.parent[v] != v {
            v = self.parent[v];
        }
        v
    }
    pub fn domain(&self, expr: Expr, sets: &Sets) -> Set {
        match expr {
            Expr::Var(v) => self.domains[self.root(v)],
            Expr::Constant(v) => sets.one(v),
        }
    }
    pub fn constrain(&mut self, expr: Expr, domain: Set, sets: &Sets, arena: &Arena) -> bool {
        let old = self.domain(expr, sets);
        let narrowed = sets.meet(old, domain, arena);
        if let Expr::Var(v) = expr {
            let root = self.root(v);
            self.domains[root] = narrowed;
        }
        narrowed != EMPTY
    }
    pub fn unify(&mut self, a: Expr, b: Expr, sets: &Sets, arena: &Arena) -> bool {
        match (a, b) {
            (Expr::Var(a), Expr::Var(b)) => {
                let a = self.root(a);
                let b = self.root(b);
                if a != b {
                    let d = sets.meet(self.domains[a], self.domains[b], arena);
                    if d == EMPTY {
                        return false;
                    }
                    self.parent[b] = a;
                    self.domains[a] = d;
                }
                true
            }
            (a, b) => {
                self.constrain(a, self.domain(b, sets), sets, arena)
                    && self.constrain(b, self.domain(a, sets), sets, arena)
            }
        }
    }
    fn limit(&mut self, expr: Expr, domain: Set, sets: &Sets, arena: &Arena) -> Option<bool> {
        let old = self.domain(expr, sets);
        self.constrain(expr, domain, sets, arena)
            .then(|| old != self.domain(expr, sets))
    }
}
pub fn expr(arena: &Arena, abs: &Abstraction, t: Id) -> Expr {
    match arena.node(t) {
        Node::Var(v) => Expr::Var(*v),
        _ => Expr::Constant(abs.value(t)),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dependency {
    Unary(Feature),
    Family(Feature),
    Link(Symbol),
    Predicate(Symbol),
}

#[derive(Default)]
struct Relation {
    // Separate rectangles with unequal axes; only identical axes union targets.
    sources: HashMap<Set, BTreeMap<Set, Set>>,
    postings: HashMap<Value, BTreeSet<Set>>,
    atoms: BTreeSet<Set>,
    numbers: BTreeSet<Set>,
}
impl Relation {
    fn add(&mut self, owner: Set, source: Set, target: Set, sets: &Sets, arena: &Arena) -> bool {
        if !self.sources.contains_key(&source) {
            for v in sets.values(source).expect("raw source keys").iter() {
                self.postings.entry(*v).or_default().insert(source);
                if let Value::Exact(t) = v {
                    if matches!(arena.node(*t), Node::Atom(_) | Node::Integer(_))
                        || arena.named(*t, "rational", 2).is_some()
                    {
                        self.atoms.insert(source);
                    }
                    if matches!(arena.node(*t), Node::Integer(_))
                        || arena.named(*t, "rational", 2).is_some()
                    {
                        self.numbers.insert(source);
                    }
                }
            }
        }
        let row = self.sources.entry(source).or_default();
        let old = row.get(&owner).copied().unwrap_or(EMPTY);
        let new = sets.union(old, target);
        // An empty native row can exist without changing the logical relation.
        if new == old && row.contains_key(&owner) {
            return false;
        }
        row.insert(owner, new);
        true
    }
    fn candidates(&self, source: Set, sets: &Sets, arena: &Arena) -> BTreeSet<Set> {
        let mut out = BTreeSet::new();
        if let Some(any) = self.postings.get(&Value::Any) {
            out.extend(any);
        }
        for v in sets.values(source).expect("finite GAC source").iter() {
            if let Some(exact) = self.postings.get(v) {
                out.extend(exact);
            }
            for broad in [Value::Patient, Value::Number] {
                if Abstraction::intersect(arena, *v, broad).is_some()
                    && let Some(rows) = self.postings.get(&broad)
                {
                    out.extend(rows);
                }
            }
            match v {
                Value::Patient => out.extend(&self.atoms),
                Value::Number => out.extend(&self.numbers),
                Value::Any => out.extend(self.sources.keys()),
                _ => {}
            }
        }
        out
    }
}
#[derive(Default)]
pub struct Stats {
    pub evaluations: usize,
    pub skipped: usize,
    pub audited: usize,
    pub link_groups: usize,
    pub link_rectangles: usize,
    pub state_cells_copied: usize,
    pub header_candidates: usize,
    pub header_requests: usize,
    pub header_survivors: usize,
    pub header_audits: usize,
    pub header_column_targets: usize,
    pub header_point_checks: usize,
    pub body_solves: usize,
    pub collections: usize,
    pub retention_before_final: Vec<(&'static str, usize)>,
}

type SupportKey = (Label, Set, Set, Set, usize);
type Supports = (Set, Set, Set);

#[cfg(test)]
thread_local! { static DROP_ONE_NOTIFICATION: Cell<bool> = const { Cell::new(false) }; }

#[derive(Default)]
struct Posting {
    values: BTreeSet<Value>,
    frozen: Cell<Option<Set>>,
}
impl Posting {
    fn add(&mut self, values: &[Value]) -> usize {
        let before = self.values.len();
        self.values.extend(values);
        let added = self.values.len() - before;
        if added > 0 {
            self.frozen.set(None);
        }
        added
    }
    fn id(&self, sets: &Sets) -> Set {
        if let Some(id) = self.frozen.get() {
            return id;
        }
        let id = sets.intern(self.values.iter().copied().collect());
        self.frozen.set(Some(id));
        id
    }
}

pub struct Model {
    pub abs: Rc<Abstraction>,
    pub sets: Sets,
    pub counters: Counters,
    pub stats: RefCell<Stats>,
    generic: bool,
    unary: HashMap<Feature, Posting>,
    families: HashMap<Feature, Posting>,
    links: HashMap<Label, Relation>,
    predicates: BTreeSet<Symbol>,
    versions: HashMap<Dependency, usize>,
    compat: RefCell<HashMap<(Set, Set), Set>>,
    supports: RefCell<HashMap<SupportKey, Supports>>,
    gc_limit: usize,
    // The notification plant targets a dependency a transfer already consumed.
    #[cfg(test)]
    notification_reads: BTreeSet<Dependency>,
}
impl Model {
    fn new(program: &Program, generic: bool, abs: Option<Rc<Abstraction>>) -> Self {
        Self {
            abs: abs.unwrap_or_else(|| Rc::new(Abstraction::new(&program.arena))),
            sets: Sets::default(),
            counters: Counters::default(),
            stats: RefCell::new(Stats::default()),
            generic,
            unary: HashMap::new(),
            families: HashMap::new(),
            links: HashMap::new(),
            predicates: BTreeSet::new(),
            versions: HashMap::new(),
            compat: RefCell::new(HashMap::new()),
            supports: RefCell::new(HashMap::new()),
            gc_limit: 64 * 1024 * 1024,
            #[cfg(test)]
            notification_reads: BTreeSet::new(),
        }
    }
    fn roots(&self) -> Vec<Set> {
        self.unary
            .values()
            .chain(self.families.values())
            .filter_map(|p| p.frozen.get())
            .chain(self.links.values().flat_map(|r| {
                r.sources.iter().flat_map(|(s, owners)| {
                    std::iter::once(*s).chain(owners.iter().flat_map(|(o, t)| [*o, *t]))
                })
            }))
            .collect()
    }
    pub fn retention(&self) -> Vec<(&'static str, usize)> {
        self.sets.retention(&self.roots())
    }
    pub fn compact(&mut self, program: &Program) {
        self.compact_roots(program, &[]);
    }
    fn compact_roots(&mut self, program: &Program, extra: &[Set]) -> Vec<Set> {
        // Callers include every retained State's domains at this safe point.
        // Pure operation/support caches are recomputable scratch.
        self.compat = RefCell::new(HashMap::new());
        self.supports = RefCell::new(HashMap::new());
        let mut roots = self.roots();
        roots.extend_from_slice(extra);
        let mapping = self.sets.compact(&roots);
        for posting in self.unary.values_mut().chain(self.families.values_mut()) {
            posting
                .frozen
                .set(posting.frozen.get().map(|old| mapping[old]));
        }
        for relation in self.links.values_mut() {
            let sources = std::mem::take(&mut relation.sources);
            *relation = Relation::default();
            for (source, owners) in sources {
                for (owner, target) in owners {
                    relation.add(
                        mapping[owner],
                        mapping[source],
                        mapping[target],
                        &self.sets,
                        &program.arena,
                    );
                }
            }
        }
        self.stats.borrow_mut().collections += 1;
        self.gc_limit = self
            .sets
            .allocation_charge()
            .saturating_mul(2)
            .saturating_add(64 * 1024 * 1024);
        mapping
    }
    fn scratch_charge(&self) -> usize {
        self.sets.allocation_charge()
            + self.supports.borrow().capacity() * std::mem::size_of::<(SupportKey, Supports)>()
            + self.compat.borrow().capacity() * std::mem::size_of::<((Set, Set), Set)>()
    }
    pub(crate) fn maybe_compact(&mut self, program: &Program) {
        if self.scratch_charge() > self.gc_limit {
            self.compact(program);
        }
    }
    pub(crate) fn compact_states(
        &mut self,
        program: &Program,
        states: &mut HashMap<usize, Option<State>>,
        force: bool,
    ) {
        if force || self.scratch_charge() > self.gc_limit {
            let roots = states
                .values()
                .flatten()
                .flat_map(|s| s.domains.iter().copied())
                .collect::<Vec<_>>();
            let mapping = self.compact_roots(program, &roots);
            for state in states.values_mut().flatten() {
                for domain in &mut state.domains {
                    *domain = mapping[*domain];
                }
            }
        }
    }
    fn changed(&mut self, dep: Dependency) {
        #[cfg(test)]
        if self.stats.borrow().evaluations > 0
            && (DROP_ONE_NOTIFICATION.with(|fault| fault.replace(false))
                || self.notification_reads.contains(&dep)
                    && crate::faults::take(crate::faults::Kind::Notification))
        {
            return;
        }
        *self.versions.entry(dep).or_default() += 1;
    }
    fn top(&self, arena: &Arena, v: Value) -> bool {
        v == Value::Any
            || self.generic
                && (matches!(v, Value::Patient | Value::Number)
                    || matches!(v, Value::Exact(t) if matches!(arena.node(t), Node::Atom(_) | Node::Integer(_))
                || arena.named(t, "rational", 2).is_some()))
    }
    fn tag(&self, arena: &Arena, f: Feature) -> Option<Set> {
        if let Feature::Tag(Value::Exact(t)) = f {
            if arena.atom_name(t) == Some("$sb1_base") {
                return Some(self.sets.one(Value::Patient));
            }
            if arena.named(t, "$sb1_arith", 2).is_some() {
                return Some(self.sets.one(Value::Number));
            }
        }
        None
    }
    fn postings(&self, f: Feature) -> Vec<Set> {
        if f.selector() == Value::Any {
            vec![self.families.get(&f).map_or(EMPTY, |p| p.id(&self.sets))]
        } else {
            vec![
                self.unary.get(&f).map_or(EMPTY, |p| p.id(&self.sets)),
                self.unary
                    .get(&f.wildcard())
                    .map_or(EMPTY, |p| p.id(&self.sets)),
            ]
        }
    }
    fn sources(&self, arena: &Arena, f: Feature) -> Set {
        if let Some(tag) = self.tag(arena, f) {
            return tag;
        }
        let mut out = EMPTY;
        for set in self.postings(f) {
            if self.sets.contains(set, Value::Any) {
                return ALL;
            }
            out = self.sets.union(out, set);
        }
        if self.generic && !matches!(f, Feature::Tag(_)) {
            out = self.sets.union(out, self.sets.one(Value::Patient));
        }
        out
    }
    fn has(&self, arena: &Arena, v: Value, f: Feature) -> bool {
        if let Some(tag) = self.tag(arena, f) {
            return self.sets.meet(self.sets.one(v), tag, arena) != EMPTY;
        }
        if !matches!(f, Feature::Tag(_)) && self.top(arena, v) {
            return true;
        }
        self.postings(f)
            .into_iter()
            .any(|s| self.sets.contains(s, v) || self.sets.contains(s, Value::Any))
    }
    fn source_size(&self, arena: &Arena, f: Feature) -> usize {
        if self.tag(arena, f).is_some() {
            return 1;
        }
        let postings = self.postings(f);
        if postings.iter().any(|s| self.sets.contains(*s, Value::Any)) {
            usize::MAX
        } else {
            postings
                .into_iter()
                .map(|s| self.sets.len(s))
                .sum::<usize>()
                .saturating_add(1)
        }
    }
    fn compatible(&self, a: Set, b: Set, arena: &Arena) -> Set {
        if let Some(result) = self.compat.borrow().get(&(a, b)) {
            return *result;
        }
        let result = self.sets.compatible(a, b, arena);
        self.compat.borrow_mut().insert((a, b), result);
        result
    }
    fn support(
        &self,
        arena: &Arena,
        owner: Set,
        source: Set,
        target: Set,
        label: Label,
    ) -> (Set, Set, Set) {
        let version = self
            .versions
            .get(&Dependency::Link(label.0))
            .copied()
            .unwrap_or(0);
        let key = (label, owner, source, target, version);
        if let Some(result) = self.supports.borrow().get(&key) {
            return *result;
        }
        let owners = self.sets.raw_keys(owner);
        let mut allowed_o = EMPTY;
        let mut allowed_s = EMPTY;
        let mut allowed_t = EMPTY;
        // A finite Any source takes targets_limited's early All branch.
        if self.sets.contains(source, Value::Any) {
            let t = self.sets.meet(ALL, target, arena);
            if t != EMPTY {
                allowed_o = owner;
                allowed_s = self.sets.raw_one(Value::Any);
                allowed_t = t;
            }
        }
        let top = |set| {
            self.sets.intern(
                self.sets
                    .values(set)
                    .unwrap()
                    .iter()
                    .copied()
                    .filter(|v| self.top(arena, *v))
                    .collect(),
            )
        };
        let top_o = top(owners);
        let top_s = top(source);
        let patient = self.sets.meet(self.sets.one(Value::Patient), target, arena);
        if top_o != EMPTY && top_s != EMPTY && patient != EMPTY {
            allowed_o = self.sets.union(
                allowed_o,
                if self.sets.contains(top_o, Value::Any) {
                    ALL
                } else {
                    top_o
                },
            );
            allowed_s = self.sets.union(allowed_s, top_s);
            allowed_t = self.sets.union(allowed_t, patient);
        }
        let mut labels = vec![label];
        if label.1 == Value::Any {
            labels = self
                .links
                .keys()
                .filter(|l| l.0 == label.0)
                .copied()
                .collect();
        } else {
            labels.push(Label(label.0, Value::Any));
        }
        labels.sort_unstable();
        labels.dedup();
        for label in labels {
            let Some(relation) = self.links.get(&label) else {
                continue;
            };
            for sources in relation.candidates(source, &self.sets, arena) {
                self.stats.borrow_mut().link_groups += 1;
                let supported_s = self.compatible(source, sources, arena);
                if supported_s == EMPTY {
                    continue;
                }
                for (&rect_o, &rect_t) in &relation.sources[&sources] {
                    self.stats.borrow_mut().link_rectangles += 1;
                    let supported_t = self.sets.meet(rect_t, target, arena);
                    if supported_t == EMPTY {
                        continue;
                    }
                    let supported_o = self.compatible(owners, rect_o, arena);
                    if supported_o == EMPTY {
                        continue;
                    }
                    allowed_s = self.sets.union(allowed_s, supported_s);
                    allowed_t = self.sets.union(allowed_t, supported_t);
                    allowed_o = self.sets.union(
                        allowed_o,
                        if self.sets.contains(supported_o, Value::Any) {
                            ALL
                        } else {
                            supported_o
                        },
                    );
                }
            }
        }
        let result = (allowed_o, allowed_s, self.sets.normalize(allowed_t, arena));
        self.supports.borrow_mut().insert(key, result);
        result
    }
    pub fn solve(
        &self,
        arena: &Arena,
        pattern: &Pattern,
        mut state: State,
        links: bool,
        counters: &mut Counters,
    ) -> Option<State> {
        for &goal in &pattern.goals {
            let (p, _) = arena.call(goal).unwrap();
            if !builtin(arena, goal)
                && arena.name(p) != "$sb1_type"
                && !self.generic
                && !self.predicates.contains(&p)
            {
                return None;
            }
        }
        let mut unary = pattern.unary.iter().collect::<Vec<_>>();
        unary.sort_by_key(|u| {
            if matches!(u.term, Expr::Constant(_)) {
                0
            } else {
                self.source_size(arena, u.feature)
            }
        });
        for u in unary {
            match u.term {
                Expr::Constant(v) => {
                    if !self.has(arena, v, u.feature) {
                        return None;
                    }
                }
                e => {
                    let current = state.domain(e, &self.sets);
                    let allowed = if self.tag(arena, u.feature).is_none()
                        && let Some(values) = self.sets.values(current)
                        && (self.generic
                            || !values
                                .iter()
                                .any(|v| matches!(v, Value::Patient | Value::Number | Value::Any)))
                    {
                        self.sets.intern(
                            values
                                .iter()
                                .copied()
                                .filter(|v| self.has(arena, *v, u.feature))
                                .collect(),
                        )
                    } else {
                        self.sources(arena, u.feature)
                    };
                    if !state.constrain(e, allowed, &self.sets, arena) {
                        return None;
                    }
                }
            }
        }
        if !links {
            return Some(state);
        }
        loop {
            let mut changed = false;
            for link in &pattern.links {
                let source = state.domain(link.source, &self.sets);
                if source == ALL {
                    continue;
                }
                let target = state.domain(link.target, &self.sets);
                let owner = state.domain(link.owner, &self.sets);
                let (o, s, t) = self.support(arena, owner, source, target, link.label);
                // Native GAC computes independent marginals, even if role roots
                // alias. Keep source→target→owner update order after the join.
                changed |= state.limit(link.source, s, &self.sets, arena)?;
                changed |= state.limit(link.target, t, &self.sets, arena)?;
                changed |= state.limit(link.owner, o, &self.sets, arena)?;
            }
            counters.join_rows += u64::from(changed);
            if !changed {
                return Some(state);
            }
        }
    }
    pub fn state(
        &self,
        arena: &Arena,
        pattern: &Pattern,
        counters: &mut Counters,
    ) -> Option<State> {
        self.solve(arena, pattern, State::new(pattern.vars), true, counters)
    }
    pub fn bound_state(
        &self,
        arena: &Arena,
        pattern: &Pattern,
        term: Expr,
        domain: Set,
        counters: &mut Counters,
    ) -> Option<State> {
        let mut state = State::new(pattern.vars);
        if !state.constrain(term, domain, &self.sets, arena) {
            return None;
        }
        self.solve(arena, pattern, state, true, counters)
    }
    pub fn feature_witness(
        &self,
        arena: &Arena,
        pattern: &Pattern,
        state: State,
        counters: &mut Counters,
    ) -> Option<State> {
        let state = self.solve(arena, pattern, state, true, counters)?;
        let choice = (0..pattern.vars)
            .filter_map(|v| {
                let set = state.domain(Expr::Var(v), &self.sets);
                (set != ALL && self.sets.len(set) > 1).then_some((v, set))
            })
            .min_by_key(|(_, set)| self.sets.len(*set));
        let Some((var, set)) = choice else {
            return Some(state);
        };
        for value in self.sets.values(set).unwrap().iter() {
            let mut next = state.clone();
            if next.constrain(Expr::Var(var), self.sets.one(*value), &self.sets, arena)
                && let Some(found) = self.feature_witness(arena, pattern, next, counters)
            {
                return Some(found);
            }
        }
        None
    }
    fn add_unary(&mut self, feature: Feature, domain: Set) -> bool {
        #[cfg(test)]
        if crate::faults::take(crate::faults::Kind::Unary) {
            return false;
        }
        let values = self.sets.values(self.sets.raw_keys(domain)).unwrap();
        let added = self.unary.entry(feature).or_default().add(&values);
        if added == 0 {
            return false;
        }
        self.changed(Dependency::Unary(feature));
        let family = feature.wildcard();
        if self.families.entry(family).or_default().add(&values) > 0 {
            self.changed(Dependency::Family(family));
        }
        self.counters.feature_insertions += added as u64;
        self.counters.writer_insertions += 1;
        true
    }
    fn head(
        &mut self,
        program: &Program,
        clause: usize,
        state: Option<&State>,
        constant_only: bool,
    ) -> bool {
        let c = &program.clauses[clause];
        let (unary, links) = literal_features(&program.arena, &self.abs, c.head);
        let p = program.arena.call(c.head).unwrap().0;
        let mut changed = self.predicates.insert(p);
        if changed {
            self.changed(Dependency::Predicate(p));
        }
        for u in unary {
            if constant_only && matches!(u.term, Expr::Var(_)) {
                continue;
            }
            let domain = match u.term {
                Expr::Constant(v) => self.sets.one(v),
                _ => state.map_or(ALL, |s| s.domain(u.term, &self.sets)),
            };
            changed |= self.add_unary(u.feature, domain);
        }
        for l in links {
            if constant_only
                && [l.owner, l.source, l.target]
                    .iter()
                    .any(|e| matches!(e, Expr::Var(_)))
            {
                continue;
            }
            let domain = |e| match e {
                Expr::Constant(v) => self.sets.one(v),
                _ => state.map_or(ALL, |s| s.domain(e, &self.sets)),
            };
            #[cfg(test)]
            if crate::faults::take(crate::faults::Kind::Rectangle) {
                continue;
            }
            let owner = self.sets.raw_keys(domain(l.owner));
            #[cfg(test)]
            let owner = if crate::faults::take(crate::faults::Kind::Owner) {
                self.sets.raw_one(Value::Any)
            } else {
                owner
            };
            let source = self.sets.raw_keys(domain(l.source));
            let target = domain(l.target);
            if self.links.entry(l.label).or_default().add(
                owner,
                source,
                target,
                &self.sets,
                &program.arena,
            ) {
                self.changed(Dependency::Link(l.label.0));
                self.counters.writer_insertions += 1;
                changed = true;
            }
        }
        changed
    }
    fn dependencies(pattern: &Pattern, arena: &Arena) -> Vec<Dependency> {
        let mut deps = BTreeSet::new();
        for u in &pattern.unary {
            if u.feature.selector() == Value::Any {
                deps.insert(Dependency::Family(u.feature));
            } else {
                deps.insert(Dependency::Unary(u.feature));
                deps.insert(Dependency::Unary(u.feature.wildcard()));
            }
        }
        deps.extend(pattern.links.iter().map(|l| Dependency::Link(l.label.0)));
        deps.extend(
            pattern
                .goals
                .iter()
                .map(|g| Dependency::Predicate(arena.call(*g).unwrap().0)),
        );
        deps.into_iter().collect()
    }
    pub fn build(
        program: &Program,
        selected: Option<&BTreeSet<usize>>,
        generic: bool,
        abs: Option<Rc<Abstraction>>,
        skip_audit: bool,
    ) -> Self {
        let mut model = Self::new(program, generic, abs);
        let mut rules = Vec::new();
        for i in 0..program.clauses.len() {
            if selected.is_some_and(|s| !s.contains(&i)) || generic && program.patient(i) {
                continue;
            }
            let c = &program.clauses[i];
            if generic || c.body.is_empty() {
                model.head(program, i, None, true);
            }
            if !c.body.is_empty() {
                let pattern =
                    Pattern::body(&program.arena, &model.abs, &c.body, c.names.len(), None);
                let deps = Self::dependencies(&pattern, &program.arena);
                rules.push((i, pattern, deps, None));
            }
        }
        let mut counters = Counters::default();
        loop {
            let mut changed = false;
            for (i, pattern, deps, last) in &mut rules {
                #[cfg(test)]
                model.notification_reads.extend(deps.iter().copied());
                let key = deps
                    .iter()
                    .map(|d| model.versions.get(d).copied().unwrap_or(0))
                    .collect::<Vec<_>>();
                if last.as_ref() == Some(&key) {
                    model.stats.borrow_mut().skipped += 1;
                    if skip_audit {
                        model.stats.borrow_mut().audited += 1;
                        let result = model.state(&program.arena, pattern, &mut counters);
                        assert!(
                            !result.is_some_and(|s| model.head(program, *i, Some(&s), false)),
                            "dirty-round skip wrote at {}",
                            program.label(*i)
                        );
                    }
                    model.maybe_compact(program);
                    continue;
                }
                *last = Some(key);
                model.stats.borrow_mut().evaluations += 1;
                model.counters.worklist_steps += 1;
                if let Some(state) = model.state(&program.arena, pattern, &mut counters) {
                    changed |= model.head(program, *i, Some(&state), false);
                }
                model.maybe_compact(program);
            }
            if !changed {
                break;
            }
        }
        model.counters.join_rows += counters.join_rows;
        model.stats.borrow_mut().retention_before_final = model.retention();
        model.compact(program);
        model
    }
    pub fn generic_patient(&self) -> bool {
        self.generic
    }
    pub fn forced(
        &self,
        program: &Program,
        pattern: &Pattern,
        goal: Id,
        clause: usize,
        base: &State,
        counters: &mut Counters,
    ) -> Option<State> {
        let mut probe_timer = crate::exact_heads::ForcedTimer::start();
        counters.candidate_pairs += 1;
        let c = &program.clauses[clause];
        let (_, ga) = program.arena.call(goal).unwrap();
        let (_, ha) = program.arena.call(c.head).unwrap();
        self.stats.borrow_mut().state_cells_copied += 2 * base.parent.len();
        let mut state = base.clone();
        let offset = state.parent.len();
        state.parent.extend(offset..offset + c.names.len());
        state.domains.extend(vec![ALL; c.names.len()]);
        for (g, h) in ga.iter().zip(ha) {
            let g = expr(&program.arena, &self.abs, *g);
            let h = match expr(&program.arena, &self.abs, *h) {
                Expr::Var(v) => Expr::Var(offset + v),
                x => x,
            };
            if !state.unify(g, h, &self.sets, &program.arena) {
                return None;
            }
        }
        self.stats.borrow_mut().body_solves += 1;
        let state = self.solve(&program.arena, pattern, state, true, counters)?;
        self.stats.borrow_mut().state_cells_copied += 2 * state.parent.len();
        self.feature_witness(&program.arena, pattern, state.clone(), counters)?;
        probe_timer.success = true;
        Some(state)
    }
    pub(crate) fn describe_state(&self, arena: &Arena, pattern: &Pattern, state: &State) -> String {
        let domain = |e| match self.sets.values(state.domain(e, &self.sets)) {
            None => "any(missing-precision)".into(),
            Some(values) => values
                .iter()
                .map(|v| self.abs.describe(arena, *v))
                .collect::<Vec<_>>()
                .join("|"),
        };
        let mut out = String::new();
        for (i, goal) in pattern.goals.iter().enumerate() {
            let (unary, links) = literal_features(arena, &self.abs, *goal);
            out.push_str(&format!("G{} {}", i + 1, arena.render(*goal)));
            if builtin(arena, *goal) {
                out.push_str(" built-in(truth-unrefined)");
            }
            for u in unary {
                let feature = match u.feature {
                    Feature::Noun(v) => format!("noun({})", self.abs.describe(arena, v)),
                    Feature::Lemma(v) => format!("lemma({})", self.abs.describe(arena, v)),
                    Feature::Operator(v) => format!("operator({})", self.abs.describe(arena, v)),
                    Feature::Property(v) => format!("property({})", self.abs.describe(arena, v)),
                    Feature::Payload(p, v) => {
                        format!("payload({},{})", arena.name(p), self.abs.describe(arena, v))
                    }
                    Feature::Tag(v) => format!("origin_tag({})", self.abs.describe(arena, v)),
                };
                out.push_str(&format!(" {feature}={}", domain(u.term)));
            }
            for link in links {
                out.push_str(&format!(
                    " link({},{},owner={})={}->{}",
                    arena.name(link.label.0),
                    self.abs.describe(arena, link.label.1),
                    domain(link.owner),
                    domain(link.source),
                    domain(link.target)
                ));
            }
            out.push('\n');
        }
        out
    }
    pub fn dump(&self) -> crate::front::Dump {
        let unary = self
            .unary
            .iter()
            .filter(|(_, p)| !p.values.is_empty())
            .map(|(f, p)| (*f, p.values.clone()))
            .collect();
        let mut links = BTreeMap::<_, crate::domain::Domain>::new();
        for (label, relation) in &self.links {
            for (sources, owners) in &relation.sources {
                for (owner, target) in owners {
                    for o in self.sets.values(*owner).unwrap().iter() {
                        for s in self.sets.values(*sources).unwrap().iter() {
                            links
                                .entry((*o, *s, *label))
                                .or_insert_with(crate::domain::Domain::empty)
                                .union(&self.sets.domain(*target));
                        }
                    }
                }
            }
        }
        (unary, links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn compare(text: &str) {
        let mut p = Program::new();
        p.add("fixture".into(), text.into(), false).unwrap();
        for generic in [true, false] {
            let oracle = crate::features::Model::build(&p, None, generic);
            let mut candidate = Model::build(&p, None, generic, Some(oracle.abs.clone()), true);
            assert_eq!(
                candidate.dump(),
                oracle.exact_dump(),
                "generic={generic}\n{text}"
            );
            for clause in &p.clauses {
                let pattern = Pattern::body(
                    &p.arena,
                    &oracle.abs,
                    &clause.body,
                    clause.names.len(),
                    None,
                );
                let original = oracle.state(&p.arena, &pattern, &mut Counters::default());
                let actual = candidate.state(&p.arena, &pattern, &mut Counters::default());
                assert_eq!(original.is_some(), actual.is_some(), "{text}");
                if let (Some(a), Some(b)) = (original, actual) {
                    for v in 0..pattern.vars {
                        assert_eq!(
                            a.domain(Expr::Var(v)),
                            candidate
                                .sets
                                .domain(b.domain(Expr::Var(v), &candidate.sets)),
                            "{text}"
                        );
                    }
                }
            }
            let expected = crate::graph::Graph::build(&p, &oracle, None);
            let actual = crate::exact_graph::build(&p, &mut candidate, true);
            assert_eq!(
                actual.edges, expected.edges,
                "graph generic={generic}\n{text}"
            );
            assert_eq!(actual.admitted(), expected.admitted());
            let compressed = crate::exact_graph::build_compressed(&p, &mut candidate, true);
            for (i, row) in expected.edges.iter().enumerate() {
                assert_eq!(compressed.row(i), *row);
            }
            assert_eq!(compressed.admitted(), expected.admitted());
            for i in 0..p.clauses.len() {
                for j in 0..p.clauses.len() {
                    assert_eq!(
                        compressed.components[i] == compressed.components[j],
                        expected.components[i] == expected.components[j]
                    );
                }
            }
        }
    }
    #[test]
    fn skip_audit_detects_one_missing_notification() {
        let text = "'$sb1_origin'(A,c) :- '$sb1_origin'(A,b).\n'$sb1_origin'(A,b) :- '$sb1_origin'(A,a).\n'$sb1_origin'(p,a).\n";
        compare(text);
        let mut p = Program::new();
        p.add("chain".into(), text.into(), false).unwrap();
        DROP_ONE_NOTIFICATION.with(|fault| fault.set(true));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Model::build(&p, None, false, None, true)
        }));
        DROP_ONE_NOTIFICATION.with(|fault| fault.set(false));
        assert!(
            result.is_err(),
            "one skipped notification must turn the audit red"
        );
        compare(text);
    }
    #[test]
    fn scratch_collection_boundary_is_strict_and_preserves_live_model() {
        let mut p = Program::new();
        p.add(
            "fixture".into(),
            "guideline_entity(actual,p,a,countable).\n".into(),
            false,
        )
        .unwrap();
        let mut m = Model::build(&p, None, true, None, false);
        let expected = m.dump();
        let charge = m.sets.allocation_charge();
        let collections = m.stats.borrow().collections;
        m.gc_limit = charge + 1;
        m.maybe_compact(&p);
        assert_eq!(m.stats.borrow().collections, collections);
        m.gc_limit = charge;
        m.maybe_compact(&p);
        assert_eq!(m.stats.borrow().collections, collections);
        m.gc_limit = charge - 1;
        m.maybe_compact(&p);
        assert_eq!(m.stats.borrow().collections, collections + 1);
        assert_eq!(m.dump(), expected);
    }
    #[test]
    fn collection_remaps_every_cached_constraint_state() {
        let mut p = Program::new();
        p.add("fixture".into(), "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),noun,countable).\nguideline_entity(actual,A,out,countable) :- guideline_entity(actual,A,noun,countable).\n".into(), false).unwrap();
        let mut model = Model::build(&p, None, true, None, true);
        let c = &p.clauses[1];
        let pattern = Pattern::body(&p.arena, &model.abs, &c.body, c.names.len(), None);
        let state = model
            .state(&p.arena, &pattern, &mut Counters::default())
            .unwrap();
        let expected = state
            .domains
            .iter()
            .map(|d| model.sets.domain(*d))
            .collect::<Vec<_>>();
        let parents = state.parent.clone();
        let mut states = HashMap::from([(0, Some(state.clone())), (1, Some(state)), (2, None)]);
        for i in 0..1000 {
            model
                .sets
                .intern(vec![Value::Origin(i), Value::Origin(i + 1)]);
        }
        model.compact_states(&p, &mut states, true);
        for state in states.values().flatten() {
            assert_eq!(state.parent, parents);
            assert_eq!(
                state
                    .domains
                    .iter()
                    .map(|d| model.sets.domain(*d))
                    .collect::<Vec<_>>(),
                expected
            );
            assert!(
                model
                    .solve(
                        &p.arena,
                        &pattern,
                        state.clone(),
                        true,
                        &mut Counters::default()
                    )
                    .is_some()
            );
        }
        drop(states);
        model.compact(&p);
        let retained = model.retention();
        assert_eq!(retained[0].1, retained[1].1);
        assert_eq!(retained[2].1, retained[3].1);
    }
    #[test]
    fn posting_snapshots_charge_reads_not_growing_write_prefixes() {
        let sets = Sets::default();
        let mut posting = Posting::default();
        for i in 0..10_000 {
            posting.add(&[Value::Origin(i)]);
        }
        assert_eq!(sets.metrics()[1].1, 0);
        let id = posting.id(&sets);
        assert_eq!(sets.len(id), 10_000);
        assert_eq!(posting.id(&sets), id);
        assert_eq!(sets.metrics()[1].1, 10_000);
    }
    #[test]
    fn rectangle_closure_equals_oracle_on_frozen_plants() {
        for text in [
            include_str!("../plants/L1.pl.in"),
            include_str!("../plants/L2.pl.in"),
            include_str!("../plants/L3.pl.in"),
            include_str!("../plants/L4.pl.in"),
            include_str!("../plants/L5.pl.in"),
            include_str!("../plants/G1.pl.in"),
            include_str!("../plants/G2.pl.in"),
            include_str!("../plants/P3.pl.in"),
            include_str!("../plants/P4.pl.in"),
            include_str!("../plants/P5.pl.in"),
            include_str!("../plants/P6.pl.in"),
        ] {
            compare(text);
        }
    }
    #[test]
    fn generated_rectangle_and_dirty_round_differential() {
        let mut rng = 0xe7a1_12d9_b537_0341u64;
        let mut next = |n: usize| {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng as usize % n
        };
        for _ in 0..300 {
            let mut lines = Vec::new();
            for i in 0..5 {
                let value = match i {
                    0 => "p".to_owned(),
                    1 => "7".into(),
                    2 => "rational(1,2)".into(),
                    _ => format!("'$guideline_id'(product,d,1,ref({i}),[])"),
                };
                lines.push(format!(
                    "guideline_entity(actual,{value},n{},countable).",
                    next(3)
                ));
                lines.push(format!("'$sb1_origin'({value},known)."));
                let owner = if next(2) == 0 {
                    "actual"
                } else {
                    "'$guideline_id'(context,d,1,box(1),[])"
                };
                let target = if next(2) == 0 {
                    "p".to_owned()
                } else {
                    format!("'$guideline_id'(product,d,1,ref({}),[])", 3 + next(2))
                };
                lines.push(format!(
                    "guideline_arg({owner},{value},{},{target}).",
                    1 + next(2)
                ));
            }
            for _ in 0..5 {
                let noun = next(3);
                let target = next(3);
                lines.push(format!("guideline_entity(actual,A,n{target},countable) :- guideline_entity(actual,A,n{noun},countable), guideline_arg(actual,A,{},B), guideline_entity(actual,B,n{},countable).", 1 + next(2), next(3)));
            }
            for i in (1..lines.len()).rev() {
                let j = next(i + 1);
                lines.swap(i, j);
            }
            compare(&(lines.join("\n") + "\n"));
        }
    }
    #[test]
    fn rectangle_transfer_keeps_marginal_cartesian_aliases() {
        compare(concat!(
            "'$sb1_origin'('$guideline_id'(product,d,1,ref(1),[]),known).\n",
            "'$sb1_origin'('$guideline_id'(product,d,1,ref(2),[]),known).\n",
            "guideline_arg(actual,'$guideline_id'(product,d,1,ref(1),[]),1,'$guideline_id'(product,d,1,ref(2),[])).\n",
            "guideline_arg(actual,'$guideline_id'(product,d,1,ref(2),[]),1,'$guideline_id'(product,d,1,ref(1),[])).\n",
            "guideline_arg(actual,X,2,Y) :- '$sb1_origin'(X,known), '$sb1_origin'(Y,known), guideline_arg(actual,X,1,Y), guideline_arg(actual,Y,1,X).\n",
            "guideline_entity(actual,X,derived,countable) :- '$sb1_origin'(X,known), '$sb1_origin'(Y,known), '$sb1_origin'(Z,known), guideline_arg(actual,X,1,Y), guideline_arg(actual,Y,1,Z), guideline_arg(actual,Z,1,X).\n"
        ));
    }
}
