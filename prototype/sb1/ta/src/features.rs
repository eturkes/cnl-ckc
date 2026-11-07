use crate::detail::{Outcome, Role};
pub use crate::domain::{Abstraction, Counters, Domain, Value};
use crate::source::Program;
use crate::work::{AccessKind, Resource, visit};
use sb1_checker::program::{Goal, builtin};
use sb1_checker::term::{Arena, Id, Node, Symbol};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Feature {
    Noun(Value),
    Lemma(Value),
    Operator(Value),
    Property(Value),
    Payload(Symbol, Value),
    Tag(Value),
}
impl Feature {
    pub(crate) fn wildcard(self) -> Self {
        match self {
            Self::Noun(_) => Self::Noun(Value::Any),
            Self::Lemma(_) => Self::Lemma(Value::Any),
            Self::Operator(_) => Self::Operator(Value::Any),
            Self::Property(_) => Self::Property(Value::Any),
            Self::Payload(p, _) => Self::Payload(p, Value::Any),
            Self::Tag(_) => Self::Tag(Value::Any),
        }
    }
    pub(crate) fn selector(self) -> Value {
        match self {
            Self::Noun(v)
            | Self::Lemma(v)
            | Self::Operator(v)
            | Self::Property(v)
            | Self::Payload(_, v)
            | Self::Tag(v) => v,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Label(pub Symbol, pub Value);
#[derive(Clone, Copy, Debug)]
pub enum Expr {
    Var(usize),
    Constant(Value),
}
#[derive(Clone, Debug)]
pub struct Unary {
    pub term: Expr,
    pub feature: Feature,
}
#[derive(Clone, Debug)]
pub struct Link {
    pub owner: Expr,
    pub source: Expr,
    pub label: Label,
    pub target: Expr,
}
#[derive(Clone, Debug)]
pub struct Pattern {
    pub goals: Vec<Id>,
    pub vars: usize,
    pub unary: Vec<Unary>,
    pub links: Vec<Link>,
}

fn expr(arena: &Arena, abs: &Abstraction, t: Id) -> Expr {
    match arena.node(t) {
        Node::Var(v) => Expr::Var(*v),
        _ => Expr::Constant(abs.value(t)),
    }
}
pub(crate) fn literal_features(arena: &Arena, abs: &Abstraction, t: Id) -> (Vec<Unary>, Vec<Link>) {
    if builtin(arena, t) {
        return (Vec::new(), Vec::new());
    }
    let (p, a) = arena.call(t).unwrap();
    let name = arena.name(p);
    let mut unary = Vec::new();
    let mut links = Vec::new();
    if matches!(name, "$sb1_type" | "$sb1_origin") {
        unary.push(Unary {
            term: expr(arena, abs, a[0]),
            feature: Feature::Tag(abs.value(a[1])),
        });
        return (unary, links);
    }
    let selector = |at| abs.value(a[at]);
    let own = match name {
        "guideline_entity" => Some(Feature::Noun(selector(2))),
        "guideline_event" => Some(Feature::Lemma(selector(2))),
        "guideline_operator" => Some(Feature::Operator(selector(2))),
        "guideline_property" => Some(Feature::Property(selector(2))),
        _ => None,
    };
    if let Some(feature) = own {
        unary.push(Unary {
            term: expr(arena, abs, a[1]),
            feature,
        });
    }
    if !matches!(
        name,
        "guideline_schema_version"
            | "guideline_document"
            | "guideline_supersession"
            | "guideline_exclusion"
            | "guideline_evaluation_date"
    ) {
        let value = if own.is_some() {
            selector(2)
        } else {
            Value::Any
        };
        unary.push(Unary {
            term: expr(arena, abs, a[0]),
            feature: Feature::Payload(p, value),
        });
    }
    if matches!(name, "guideline_arg" | "guideline_pp") {
        links.push(Link {
            owner: expr(arena, abs, a[0]),
            source: expr(arena, abs, a[1]),
            label: Label(p, selector(2)),
            target: expr(arena, abs, a[3]),
        });
    }
    (unary, links)
}
impl Pattern {
    pub fn new(arena: &Arena, abs: &Abstraction, goals: Vec<Id>, vars: usize) -> Self {
        let mut unary = Vec::new();
        let mut links = Vec::new();
        for &t in &goals {
            if !builtin(arena, t) {
                let (u, l) = literal_features(arena, abs, t);
                unary.extend(u);
                links.extend(l);
            }
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
        abs: &Abstraction,
        goals: &[Goal],
        vars: usize,
        negative: Option<&[Goal]>,
    ) -> Self {
        let mut terms = goals
            .iter()
            .filter_map(|g| match g {
                Goal::Positive(t) | Goal::Builtin(t) => Some(*t),
                _ => None,
            })
            .collect::<Vec<_>>();
        if let Some(inner) = negative {
            terms.extend(inner.iter().filter_map(|g| match g {
                Goal::Positive(t) | Goal::Builtin(t) => Some(*t),
                _ => None,
            }));
        }
        Self::new(arena, abs, terms, vars)
    }
}

#[derive(Debug)]
pub struct State {
    parent: Vec<usize>,
    domains: Vec<Domain>,
    census_key: RefCell<Option<(usize, usize)>>,
}
impl Clone for State {
    fn clone(&self) -> Self {
        if crate::census::enabled() {
            visit(
                "state_clone_member_visits",
                self.domains
                    .iter()
                    .map(|d| d.values().map_or(1, BTreeSet::len))
                    .sum(),
            );
        }
        Self {
            parent: self.parent.clone(),
            domains: self.domains.clone(),
            census_key: self.census_key.clone(),
        }
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.domains == other.domains
    }
}
impl Eq for State {}
impl State {
    pub(crate) fn new(vars: usize) -> Self {
        Self {
            parent: (0..vars).collect(),
            domains: vec![Domain::All; vars],
            census_key: RefCell::new(None),
        }
    }
    pub fn audit_state(parent: Vec<usize>, domains: Vec<Domain>) -> Self {
        assert_eq!(parent.len(), domains.len());
        Self {
            parent,
            domains,
            census_key: RefCell::new(None),
        }
    }
    pub fn audit_header_prefix(
        &self,
        arena: &Arena,
        goal: &[Expr],
        head: &[Expr],
        head_vars: usize,
    ) -> bool {
        let mut state = self.clone();
        let offset = state.parent.len();
        state.parent.extend(offset..offset + head_vars);
        state.domains.extend(vec![Domain::All; head_vars]);
        for (g, h) in goal.iter().zip(head) {
            let h = match h {
                Expr::Var(v) => Expr::Var(offset + v),
                x => *x,
            };
            if !state.unify(*g, h, arena) {
                return false;
            }
        }
        true
    }
    pub(crate) fn census_parts(&self) -> (&[usize], &[Domain]) {
        (&self.parent, &self.domains)
    }
    pub(crate) fn census_cached(&self, token: usize) -> Option<usize> {
        self.census_key
            .borrow()
            .filter(|(t, _)| *t == token)
            .map(|(_, id)| id)
    }
    pub(crate) fn census_cache(&self, token: usize, id: usize) {
        *self.census_key.borrow_mut() = Some((token, id));
    }
    fn census_invalidate(&mut self) {
        *self.census_key.get_mut() = None;
    }
    pub(crate) fn memory_bytes(&self) -> usize {
        use crate::memory::bytes;
        bytes::<usize>(self.parent.capacity())
            + bytes::<Domain>(self.domains.capacity())
            + self
                .domains
                .iter()
                .map(|d| d.values().map_or(0, |s| bytes::<Value>(s.len())))
                .sum::<usize>()
    }
    fn root(&self, mut v: usize) -> usize {
        while self.parent[v] != v {
            v = self.parent[v];
        }
        v
    }
    pub fn domain(&self, e: Expr) -> Domain {
        match e {
            Expr::Var(v) => {
                let domain = &self.domains[self.root(v)];
                visit(
                    "domain_clone_member_visits",
                    domain.values().map_or(1, BTreeSet::len),
                );
                domain.clone()
            }
            Expr::Constant(v) => Domain::one(v),
        }
    }
    pub(crate) fn constrain(&mut self, e: Expr, d: &Domain, arena: &Arena) -> bool {
        let Expr::Var(v) = e else {
            return !self.domain(e).meet(d, arena).is_empty();
        };
        self.census_invalidate();
        let r = self.root(v);
        self.domains[r] = self.domains[r].meet(d, arena);
        !self.domains[r].is_empty()
    }
    fn unify(&mut self, a: Expr, b: Expr, arena: &Arena) -> bool {
        match (a, b) {
            (Expr::Var(a), Expr::Var(b)) => {
                let a = self.root(a);
                let b = self.root(b);
                if a != b {
                    let d = self.domains[a].meet(&self.domains[b], arena);
                    if d.is_empty() {
                        return false;
                    }
                    self.census_invalidate();
                    self.parent[b] = a;
                    self.domains[a] = d;
                }
                true
            }
            (a, b) => {
                let d = self.domain(b);
                self.constrain(a, &d, arena) && {
                    let d = self.domain(a);
                    self.constrain(b, &d, arena)
                }
            }
        }
    }
    fn limit(&mut self, e: Expr, d: &Domain, arena: &Arena) -> Option<bool> {
        let old = self.domain(e);
        if !self.constrain(e, d, arena) {
            None
        } else {
            Some(old != self.domain(e))
        }
    }
}

#[derive(Default)]
pub(crate) struct Heads {
    rows: Vec<usize>,
    columns: Vec<HashMap<Value, Vec<usize>>>,
    atomic: Vec<Vec<usize>>,
}
impl Heads {
    pub(crate) fn insert(&mut self, clause: usize, args: &[Id], arena: &Arena, abs: &Abstraction) {
        self.rows.push(clause);
        self.columns.resize_with(args.len(), HashMap::new);
        self.atomic.resize_with(args.len(), Vec::new);
        for ((column, atomic), t) in self.columns.iter_mut().zip(&mut self.atomic).zip(args) {
            let value = match arena.node(*t) {
                Node::Var(_) => Value::Any,
                _ => abs.value(*t),
            };
            column.entry(value).or_default().push(clause);
            if matches!(value, Value::Exact(_)) {
                atomic.push(clause);
            }
        }
    }
    pub(crate) fn candidates(
        &self,
        args: &[Id],
        state: &State,
        arena: &Arena,
        abs: &Abstraction,
        scanned: &mut usize,
    ) -> Vec<usize> {
        let mut best: Option<(usize, Vec<&[usize]>)> = None;
        for (at, (arg, column)) in args.iter().zip(&self.columns).enumerate() {
            let domain = state.domain(expr(arena, abs, *arg));
            let Some(values) = domain.values() else {
                continue;
            };
            visit("head_index_value_visits", values.len());
            let mut buckets = Vec::new();
            if let Some(wild) = column.get(&Value::Any) {
                buckets.push(wild.as_slice());
            }
            for v in values {
                if let Some(exact) = column.get(v) {
                    buckets.push(exact.as_slice());
                }
                if matches!(v, Value::Patient | Value::Number) {
                    buckets.push(self.atomic[at].as_slice());
                }
            }
            let size = buckets.iter().map(|b| b.len()).sum();
            if best.as_ref().is_none_or(|(old, _)| size < *old) {
                best = Some((size, buckets));
            }
        }
        *scanned = best.as_ref().map_or(self.rows.len(), |(size, _)| *size);
        visit("head_posting_member_visits", *scanned);
        match best {
            None => self.rows.clone(),
            Some((_, buckets)) => buckets
                .into_iter()
                .flat_map(|b| b.iter().copied())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Mode {
    #[default]
    Refined,
    Reference,
}
type Owners = HashMap<Value, Domain>;

pub struct Model {
    pub abs: Rc<Abstraction>,
    pub counters: Counters,
    pub mode: Mode,
    unary: HashMap<Feature, BTreeSet<Value>>,
    families: HashMap<Feature, BTreeSet<Value>>,
    links: HashMap<(Value, Label), Owners>,
    scalar_sources: HashMap<Label, BTreeSet<Value>>,
    predicates: HashSet<Symbol>,
    generic: bool,
    heads: HashMap<Symbol, Heads>,
    census: Option<RefCell<crate::census::Trace>>,
}
impl Model {
    fn new(program: &Program, generic: bool, abs: Option<Rc<Abstraction>>, mode: Mode) -> Self {
        Self {
            abs: abs.unwrap_or_else(|| Rc::new(Abstraction::new(&program.arena))),
            counters: Counters::default(),
            mode,
            unary: HashMap::new(),
            families: HashMap::new(),
            links: HashMap::new(),
            scalar_sources: HashMap::new(),
            predicates: HashSet::new(),
            generic,
            heads: HashMap::new(),
            census: crate::census::enabled()
                .then(|| RefCell::new(crate::census::Trace::new(program))),
        }
    }
    fn top(&self, arena: &Arena, v: Value) -> bool {
        v == Value::Any
            || self.generic
                && (matches!(v, Value::Patient | Value::Number)
                    || matches!(v,Value::Exact(t) if matches!(arena.node(t),Node::Atom(_)|Node::Integer(_)) || arena.named(t,"rational",2).is_some()))
    }
    fn tag_domain(arena: &Arena, f: Feature) -> Option<Domain> {
        if let Feature::Tag(Value::Exact(t)) = f {
            if arena.atom_name(t) == Some("$sb1_base") {
                return Some(Domain::one(Value::Patient));
            }
            if arena.named(t, "$sb1_arith", 2).is_some() {
                return Some(Domain::one(Value::Number));
            }
        }
        None
    }
    pub fn census_caller(&self, program: &Program, clause: usize) {
        if let Some(census) = &self.census {
            census.borrow_mut().caller(program, clause);
        }
    }
    fn census_fail(&self, outcome: Outcome) {
        if let Some(census) = &self.census {
            census.borrow_mut().fail(outcome);
        }
    }
    fn census_forced_end(&self, outcome: Option<Outcome>) {
        if let Some(census) = &self.census {
            census.borrow_mut().finish_forced(outcome);
        }
    }
    fn census_access(&self, key: Resource, kind: AccessKind, visits: usize) {
        if !crate::detail::enabled() {
            return;
        }
        if let Some(census) = &self.census {
            census.borrow_mut().work.access(key, kind, visits);
        }
    }
    fn census_postings(&self, feature: Feature) -> Vec<(Resource, Option<&BTreeSet<Value>>)> {
        if feature.selector() == Value::Any {
            vec![(Resource::Family(feature), self.families.get(&feature))]
        } else {
            vec![
                (Resource::Unary(feature), self.unary.get(&feature)),
                (
                    Resource::Unary(feature.wildcard()),
                    self.unary.get(&feature.wildcard()),
                ),
            ]
        }
    }
    #[cfg(test)]
    pub(crate) fn detail_for_test(&mut self, program: &Program) {
        let mut trace = crate::census::Trace::new(program);
        trace.details = Some(crate::detail::Details::new(program));
        self.census = Some(RefCell::new(trace));
    }
    pub fn write_detail(
        &self,
        program: &Program,
        out: &std::path::Path,
        name: &str,
    ) -> Result<(), String> {
        if let Some(census) = &self.census {
            let census = census.borrow();
            if let Some(details) = &census.details {
                details.write(out, name, &program.arena, &self.abs)?;
            }
            census
                .work
                .write_access(out, name, &program.arena, &self.abs)?;
        }
        Ok(())
    }
    fn census_unary_inputs(&self, feature: Feature) {
        if let Some(census) = &self.census {
            let mut census = census.borrow_mut();
            if feature.selector() == Value::Any {
                census.work.read(Resource::Family(feature), || {
                    self.families
                        .get(&feature)
                        .map_or_else(Vec::new, |s| s.iter().copied().collect())
                });
            } else {
                for f in [feature, feature.wildcard()] {
                    census.work.read(Resource::Unary(f), || {
                        self.unary
                            .get(&f)
                            .map_or_else(Vec::new, |s| s.iter().copied().collect())
                    });
                }
            }
        }
    }
    fn sources(&self, arena: &Arena, f: Feature) -> Domain {
        if let Some(domain) = Self::tag_domain(arena, f) {
            return domain;
        }
        self.census_unary_inputs(f);
        let mut out = BTreeSet::new();
        for (key, sources) in self.census_postings(f) {
            let Some(sources) = sources else {
                continue;
            };
            self.census_access(key, AccessKind::Membership, 1);
            if sources.contains(&Value::Any) {
                return Domain::All;
            }
            self.census_access(key, AccessKind::Enumerated, sources.len());
            visit("unary_source_member_visits", sources.len());
            out.extend(sources);
        }
        if self.generic && !matches!(f, Feature::Tag(_)) {
            out.insert(Value::Patient);
        }
        Domain::Set(out)
    }
    fn has(&self, arena: &Arena, v: Value, f: Feature) -> bool {
        if let Some(domain) = Self::tag_domain(arena, f) {
            return !Domain::one(v).meet(&domain, arena).is_empty();
        }
        if !matches!(f, Feature::Tag(_)) && self.top(arena, v) {
            return true;
        }
        self.census_unary_inputs(f);
        visit("unary_membership_queries", 1);
        self.census_postings(f).into_iter().any(|(key, sources)| {
            let Some(sources) = sources else {
                return false;
            };
            self.census_access(key, AccessKind::Membership, 1);
            if sources.contains(&v) {
                return true;
            }
            self.census_access(key, AccessKind::Membership, 1);
            sources.contains(&Value::Any)
        })
    }
    fn targets(&self, arena: &Arena, owner: Value, source: Value, label: Label) -> Domain {
        self.targets_limited(arena, owner, source, label, None)
    }
    fn targets_limited(
        &self,
        arena: &Arena,
        owner: Value,
        source: Value,
        label: Label,
        limit: Option<&Domain>,
    ) -> Domain {
        let narrow = |domain: Domain| {
            limit.map_or_else(|| domain.clone(), |target| domain.meet(target, arena))
        };
        if source == Value::Any {
            return narrow(Domain::All);
        }
        if self.mode == Mode::Reference && self.top(arena, source) {
            return narrow(Domain::one(Value::Patient));
        }
        // Patient facts cannot name a KB origin. Rules still can attach a KB
        // target to a patient referent: implicit patient links must union with
        // stored links, never replace them.
        let mut out = narrow(if self.top(arena, owner) && self.top(arena, source) {
            Domain::one(Value::Patient)
        } else {
            Domain::empty()
        });
        let mut sources = vec![source, Value::Any];
        if self.mode == Mode::Refined {
            if Abstraction::intersect(arena, source, Value::Patient).is_some() {
                sources.push(Value::Patient);
            }
            if Abstraction::intersect(arena, source, Value::Number).is_some() {
                sources.push(Value::Number);
            }
            if matches!(source, Value::Patient | Value::Number) {
                for l in [label, Label(label.0, Value::Any)] {
                    if let Some(census) = &self.census {
                        census.borrow_mut().work.read(Resource::Scalars(l), || {
                            self.scalar_sources
                                .get(&l)
                                .map_or_else(Vec::new, |s| s.iter().copied().collect())
                        });
                    }
                    if let Some(values) = self.scalar_sources.get(&l) {
                        self.census_access(
                            Resource::Scalars(l),
                            AccessKind::Enumerated,
                            values.len(),
                        );
                        visit("scalar_source_member_visits", values.len());
                        sources.extend(
                            values
                                .iter()
                                .filter(|v| Abstraction::intersect(arena, source, **v).is_some()),
                        );
                    }
                }
            }
        }
        sources.sort_unstable();
        sources.dedup();
        let mut include = |source: Value, label: Label, owners: &Owners| {
            visit("owner_row_visits", owners.len());
            if let Some(census) = &self.census {
                census
                    .borrow_mut()
                    .work
                    .read(Resource::Owners(source, label), || {
                        owners.keys().copied().collect()
                    });
            }
            self.census_access(
                Resource::Owners(source, label),
                AccessKind::Enumerated,
                owners.len(),
            );
            for (o, targets) in owners {
                if self.mode == Mode::Reference
                    || Abstraction::intersect(arena, owner, *o).is_some()
                {
                    if let Some(census) = &self.census {
                        census
                            .borrow_mut()
                            .work
                            .read(Resource::Targets(source, label, *o), || {
                                crate::census::domain_values(targets)
                            });
                    }
                    if let Some(target) = limit {
                        self.census_access(
                            Resource::Targets(source, label, *o),
                            AccessKind::SetOperand,
                            1,
                        );
                        out.union(&targets.meet(target, arena));
                    } else {
                        if out != Domain::All {
                            self.census_access(
                                Resource::Targets(source, label, *o),
                                AccessKind::Enumerated,
                                targets.values().map_or(1, BTreeSet::len),
                            );
                        }
                        out.union(targets);
                    }
                }
            }
        };
        for s in &sources {
            for l in [label, Label(label.0, Value::Any)] {
                visit("link_index_lookups", 1);
                if let Some(owners) = self.links.get(&(*s, l)) {
                    include(*s, l, owners);
                } else if let Some(census) = &self.census {
                    census
                        .borrow_mut()
                        .work
                        .read(Resource::Owners(*s, l), Vec::new);
                }
            }
        }
        if label.1 == Value::Any {
            if let Some(census) = &self.census {
                census.borrow_mut().work.read_link_rows(self.links.keys());
            }
            visit("link_wildcard_row_visits", self.links.len());
            for ((s, l), owners) in &self.links {
                if l.0 == label.0
                    && (sources.contains(s)
                        || self.mode == Mode::Refined
                            && Abstraction::intersect(arena, source, *s).is_some())
                {
                    include(*s, *l, owners);
                }
            }
        }
        if limit.is_some() {
            out.meet(&Domain::All, arena)
        } else {
            out
        }
    }
    fn source_size(&self, arena: &Arena, f: Feature) -> usize {
        if Self::tag_domain(arena, f).is_some() {
            return 1;
        }
        self.census_unary_inputs(f);
        let sources = self.census_postings(f);
        if sources.iter().any(|(key, sources)| {
            sources.is_some_and(|s| {
                self.census_access(*key, AccessKind::Membership, 1);
                s.contains(&Value::Any)
            })
        }) {
            usize::MAX
        } else {
            sources
                .into_iter()
                .filter_map(|(_, s)| s)
                .map(BTreeSet::len)
                .sum::<usize>()
                .saturating_add(1)
        }
    }
    pub fn solve(
        &self,
        arena: &Arena,
        pattern: &Pattern,
        state: State,
        links: bool,
        counters: &mut Counters,
    ) -> Option<State> {
        self.solve_mode(arena, pattern, state, links, true, counters)
    }
    #[cfg(test)]
    pub(crate) fn solve_reference(
        &self,
        arena: &Arena,
        pattern: &Pattern,
        state: State,
        counters: &mut Counters,
    ) -> Option<State> {
        self.solve_mode(arena, pattern, state, true, false, counters)
    }
    fn limit_link(
        &self,
        state: &mut State,
        expr: Expr,
        domain: &Domain,
        arena: &Arena,
        link: &Link,
        role: Role,
    ) -> Option<bool> {
        let result = state.limit(expr, domain, arena);
        if result.is_none() {
            self.census_fail(Outcome::Link(link.label, role));
        }
        result
    }
    fn solve_mode(
        &self,
        arena: &Arena,
        pattern: &Pattern,
        mut state: State,
        links: bool,
        selective: bool,
        counters: &mut Counters,
    ) -> Option<State> {
        for &goal in &pattern.goals {
            let (p, _) = arena.call(goal).unwrap();
            if !builtin(arena, goal)
                && arena.name(p) != "$sb1_type"
                && !self.generic
                && !self.predicates.contains(&p)
            {
                self.census_fail(Outcome::Predicate(p));
                return None;
            }
        }
        let mut unary = pattern.unary.iter().collect::<Vec<_>>();
        if selective {
            unary.sort_by_key(|u| match u.term {
                Expr::Constant(_) => 0,
                _ => self.source_size(arena, u.feature),
            });
        }
        for u in unary {
            match u.term {
                Expr::Constant(v) => {
                    if !self.has(arena, v, u.feature) {
                        self.census_fail(Outcome::Unary(u.feature));
                        return None;
                    }
                }
                e => {
                    let allowed = if selective && Self::tag_domain(arena, u.feature).is_none() {
                        match state.domain(e) {
                            Domain::Set(values)
                                if self.generic
                                    || !values.iter().any(|v| {
                                        matches!(v, Value::Patient | Value::Number | Value::Any)
                                    }) =>
                            {
                                Domain::Set(
                                    values
                                        .into_iter()
                                        .filter(|v| self.has(arena, *v, u.feature))
                                        .collect(),
                                )
                            }
                            _ => self.sources(arena, u.feature),
                        }
                    } else {
                        self.sources(arena, u.feature)
                    };
                    if !state.constrain(e, &allowed, arena) {
                        self.census_fail(Outcome::Unary(u.feature));
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
                let source = state.domain(link.source);
                let target = state.domain(link.target);
                let owner = if self.mode == Mode::Reference {
                    Domain::All
                } else {
                    state.domain(link.owner)
                };
                if let Some(sources) = source.values() {
                    let mut allowed_sources = BTreeSet::new();
                    let mut allowed_owners = Domain::empty();
                    let mut allowed_targets = Domain::empty();
                    let owners = owner.values().map_or_else(
                        || vec![Value::Any],
                        |values| values.iter().copied().collect(),
                    );
                    for o in owners {
                        for s in sources {
                            let compatible = if selective && self.mode == Mode::Refined {
                                self.targets_limited(arena, o, *s, link.label, Some(&target))
                            } else {
                                self.targets(arena, o, *s, link.label).meet(&target, arena)
                            };
                            if !compatible.is_empty() {
                                allowed_sources.insert(*s);
                                allowed_owners.union(&Domain::one(o));
                                allowed_targets.union(&compatible);
                            }
                        }
                    }
                    changed |= self.limit_link(
                        &mut state,
                        link.source,
                        &Domain::Set(allowed_sources),
                        arena,
                        link,
                        Role::Source,
                    )?;
                    changed |= self.limit_link(
                        &mut state,
                        link.target,
                        &allowed_targets,
                        arena,
                        link,
                        Role::Target,
                    )?;
                    if self.mode == Mode::Refined {
                        changed |= self.limit_link(
                            &mut state,
                            link.owner,
                            &allowed_owners,
                            arena,
                            link,
                            Role::Owner,
                        )?;
                    }
                }
            }
            counters.join_rows += u64::from(changed);
            if !changed {
                return Some(state);
            }
        }
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
            .filter_map(|v| match state.domain(Expr::Var(v)) {
                Domain::Set(values) if values.len() > 1 => Some((v, values)),
                _ => None,
            })
            .min_by_key(|(_, values)| values.len());
        let Some((var, values)) = choice else {
            return Some(state);
        };
        for value in values {
            let mut next = state.clone();
            if next.constrain(Expr::Var(var), &Domain::one(value), arena)
                && let Some(found) = self.feature_witness(arena, pattern, next, counters)
            {
                return Some(found);
            }
        }
        None
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
        domain: &Domain,
        counters: &mut Counters,
    ) -> Option<State> {
        let mut state = State::new(pattern.vars);
        if !state.constrain(term, domain, arena) {
            return None;
        }
        self.solve(arena, pattern, state, true, counters)
    }
    fn add_unary(&mut self, feature: Feature, source: Value) -> bool {
        let changed = self.unary.entry(feature).or_default().insert(source);
        if changed {
            let family_changed = self
                .families
                .entry(feature.wildcard())
                .or_default()
                .insert(source);
            if let Some(census) = &mut self.census {
                let work = &mut census.get_mut().work;
                work.changed(Resource::Unary(feature));
                if family_changed {
                    work.changed(Resource::Family(feature.wildcard()));
                }
            }
            self.counters.feature_insertions += 1;
            self.counters.writer_insertions += 1;
        }
        changed
    }
    fn add_link(&mut self, owner: Value, source: Value, label: Label, target: &Domain) -> bool {
        if matches!(source, Value::Exact(_)) {
            let changed = self.scalar_sources.entry(label).or_default().insert(source);
            if changed && let Some(census) = &mut self.census {
                census.get_mut().work.changed(Resource::Scalars(label));
            }
        }
        let mut new_row = false;
        let mut new_owner = false;
        let d = self
            .links
            .entry((source, label))
            .or_insert_with(|| {
                new_row = true;
                HashMap::new()
            })
            .entry(owner)
            .or_insert_with(|| {
                new_owner = true;
                Domain::empty()
            });
        let before = d.values().map_or(1, BTreeSet::len);
        let changed = d.union(target);
        if let Some(census) = &mut self.census {
            let work = &mut census.get_mut().work;
            if new_row {
                work.changed(Resource::LinkRows);
            }
            if new_owner {
                work.changed(Resource::Owners(source, label));
            }
            if changed || new_owner {
                work.changed(Resource::Targets(source, label, owner));
            }
        }
        if changed {
            let after = d.values().map_or(1, BTreeSet::len);
            self.counters.feature_insertions += after.saturating_sub(before).max(1) as u64;
            self.counters.writer_insertions += 1;
        }
        changed
    }
    fn head(
        &mut self,
        program: &Program,
        index: usize,
        state: Option<&State>,
        constant_only: bool,
    ) -> bool {
        let arena = &program.arena;
        let c = &program.clauses[index];
        let (unary, links) = literal_features(arena, &self.abs, c.head);
        let mut changed = false;
        if self.predicates.insert(arena.call(c.head).unwrap().0) {
            changed = true;
        }
        let domain = |e: Expr| match e {
            Expr::Constant(v) => Domain::one(v),
            Expr::Var(_) => state.map_or(Domain::All, |s| s.domain(e)),
        };
        for u in unary {
            if constant_only && matches!(u.term, Expr::Var(_)) {
                continue;
            }
            match domain(u.term) {
                Domain::All => changed |= self.add_unary(u.feature, Value::Any),
                Domain::Set(ss) => {
                    for s in ss {
                        changed |= self.add_unary(u.feature, s);
                    }
                }
            }
        }
        for l in links {
            if constant_only
                && (matches!(l.source, Expr::Var(_))
                    || matches!(l.target, Expr::Var(_))
                    || self.mode == Mode::Refined && matches!(l.owner, Expr::Var(_)))
            {
                continue;
            }
            let values = |d: Domain| match d {
                Domain::All => vec![Value::Any],
                Domain::Set(s) => s.into_iter().collect(),
            };
            let target = domain(l.target);
            let owners = values(if self.mode == Mode::Reference {
                Domain::All
            } else {
                domain(l.owner)
            });
            let sources = values(domain(l.source));
            if let Some(census) = &mut self.census {
                census.get_mut().links.record(
                    index,
                    constant_only,
                    owners.clone(),
                    sources.clone(),
                    l.label,
                    &target,
                );
            }
            for s in sources {
                for o in &owners {
                    changed |= self.add_link(*o, s, l.label, &target);
                }
            }
        }
        changed
    }
    pub fn build(program: &Program, selected: Option<&BTreeSet<usize>>, generic: bool) -> Self {
        Self::build_shared(program, selected, generic, None)
    }
    pub fn build_shared(
        program: &Program,
        selected: Option<&BTreeSet<usize>>,
        generic: bool,
        abs: Option<Rc<Abstraction>>,
    ) -> Self {
        Self::build_mode(program, selected, generic, abs, Mode::Refined)
    }
    pub fn build_mode(
        program: &Program,
        selected: Option<&BTreeSet<usize>>,
        generic: bool,
        abs: Option<Rc<Abstraction>>,
        mode: Mode,
    ) -> Self {
        let mut model = Self::new(program, generic, abs, mode);
        let arena = &program.arena;
        let ids = selected.map_or_else(
            || (0..program.clauses.len()).collect::<Vec<_>>(),
            |s| s.iter().copied().collect(),
        );
        let mut rules = Vec::new();
        for i in ids {
            if generic && program.patient(i) {
                continue;
            }
            let c = &program.clauses[i];
            let (p, args) = arena.call(c.head).unwrap();
            model
                .heads
                .entry(p)
                .or_default()
                .insert(i, args, arena, &model.abs);
            if generic || c.body.is_empty() {
                model.head(program, i, None, true);
            }
            if !c.body.is_empty() {
                rules.push((
                    i,
                    Pattern::body(arena, &model.abs, &c.body, c.names.len(), None),
                ));
            }
        }
        let mut counters = Counters::default();
        loop {
            let mut changed = false;
            for (i, pattern) in &rules {
                model.counters.worklist_steps += 1;
                if let Some(census) = &model.census {
                    census.borrow_mut().work.begin(arena, &model.abs, pattern);
                }
                let state = model.solve(
                    arena,
                    pattern,
                    State::new(pattern.vars),
                    !generic || mode == Mode::Refined,
                    &mut counters,
                );
                if let Some(census) = &model.census {
                    census.borrow_mut().work.end(state.as_ref());
                }
                if let Some(state) = state {
                    changed |= model.head(program, *i, Some(&state), false);
                }
            }
            if !changed {
                break;
            }
        }
        model.counters.join_rows += counters.join_rows;
        model
    }
    pub fn candidates(&self, arena: &Arena, goal: Id, state: &State) -> Vec<usize> {
        let (p, args) = arena.call(goal).unwrap();
        let mut scanned = 0;
        let result = self.heads.get(&p).map_or_else(Vec::new, |h| {
            h.candidates(args, state, arena, &self.abs, &mut scanned)
        });
        if let Some(census) = &self.census {
            census
                .borrow_mut()
                .work
                .candidates(goal, state, scanned, result.len());
        }
        result
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
        counters.candidate_pairs += 1;
        let arena = &program.arena;
        let c = &program.clauses[clause];
        if let Some(census) = &self.census {
            let mut census = census.borrow_mut();
            census.begin_forced(program, clause);
            census.probe(arena, &self.abs, pattern, goal, c.head, base);
        }
        let (_, ga) = arena.call(goal).unwrap();
        let (_, ha) = arena.call(c.head).unwrap();
        let mut state = base.clone();
        state.census_invalidate();
        let offset = state.parent.len();
        state.parent.extend(offset..offset + c.names.len());
        state.domains.extend(vec![Domain::All; c.names.len()]);
        for (at, (g, h)) in ga.iter().zip(ha).enumerate() {
            let g = expr(arena, &self.abs, *g);
            let h = match expr(arena, &self.abs, *h) {
                Expr::Var(v) => Expr::Var(offset + v),
                x => x,
            };
            if !state.unify(g, h, arena) {
                self.census_forced_end(Some(Outcome::HeadArg(at, matches!(h, Expr::Var(_)))));
                return None;
            }
        }
        let Some(state) = self.solve(arena, pattern, state, true, counters) else {
            self.census_forced_end(None);
            return None;
        };
        if self
            .feature_witness(arena, pattern, state.clone(), counters)
            .is_none()
        {
            self.census_forced_end(Some(Outcome::Witness));
            return None;
        }
        self.census_forced_end(Some(Outcome::Edge));
        Some(state)
    }
    pub fn noun_at(&self, arena: &Arena, origin: Id, noun: &str) -> bool {
        arena
            .nodes
            .iter()
            .enumerate()
            .find_map(|(id, _)| (arena.atom_name(id) == Some(noun)).then_some(id))
            .is_some_and(|id| {
                self.has(
                    arena,
                    self.abs.value(origin),
                    Feature::Noun(self.abs.value(id)),
                )
            })
    }
    pub(crate) fn generic_patient(&self) -> bool {
        self.generic
    }
    pub fn record_memory(&self, stage: &str) {
        if !crate::memory::enabled() {
            return;
        }
        use crate::memory::{bytes, record};
        let unary_values = self.unary.values().map(BTreeSet::len).sum();
        let family_values = self.families.values().map(BTreeSet::len).sum();
        let link_values = self
            .links
            .values()
            .flat_map(HashMap::values)
            .filter_map(Domain::values)
            .map(BTreeSet::len)
            .sum();
        let heads = self
            .heads
            .values()
            .map(|h| {
                bytes::<usize>(h.rows.capacity())
                    + bytes::<HashMap<Value, Vec<usize>>>(h.columns.capacity())
                    + bytes::<Vec<usize>>(h.atomic.capacity())
                    + h.atomic
                        .iter()
                        .map(|v| bytes::<usize>(v.capacity()))
                        .sum::<usize>()
                    + h.columns
                        .iter()
                        .map(|c| {
                            bytes::<(Value, Vec<usize>)>(c.capacity())
                                + c.values()
                                    .map(|v| bytes::<usize>(v.capacity()))
                                    .sum::<usize>()
                        })
                        .sum::<usize>()
            })
            .sum();
        let mut rows = vec![
            (
                "unary-map",
                self.unary.len(),
                bytes::<(Feature, BTreeSet<Value>)>(self.unary.capacity()),
                "payload-lower-bound",
            ),
            (
                "unary-values",
                unary_values,
                bytes::<Value>(unary_values),
                "payload-lower-bound",
            ),
            (
                "feature-family-map",
                self.families.len(),
                bytes::<(Feature, BTreeSet<Value>)>(self.families.capacity()),
                "payload-lower-bound",
            ),
            (
                "feature-family-values",
                family_values,
                bytes::<Value>(family_values),
                "payload-lower-bound",
            ),
            (
                "origin-link-map",
                self.links.len(),
                bytes::<((Value, Label), Owners)>(self.links.capacity())
                    + self
                        .links
                        .values()
                        .map(|o| bytes::<(Value, Domain)>(o.capacity()))
                        .sum::<usize>(),
                "payload-lower-bound",
            ),
            (
                "origin-link-values",
                link_values,
                bytes::<Value>(link_values),
                "payload-lower-bound",
            ),
            (
                "head-indices",
                self.heads.len(),
                heads,
                "buffers+map-payload-lower-bound",
            ),
        ];
        rows.extend(self.abs.memory_rows());
        record(stage, &rows);
    }
    pub fn census_phase(&self, phase: &'static str) {
        if let Some(census) = &self.census {
            census.borrow_mut().phase(phase);
        }
    }
    pub fn write_census(&self, out: &std::path::Path, name: &str) -> Result<(), String> {
        let Some(census) = &self.census else {
            return Ok(());
        };
        let mut metrics = crate::census::Metrics::default();
        for (key, value) in [
            (
                "index.head_rows",
                self.heads.values().map(|h| h.rows.len()).sum(),
            ),
            (
                "index.head_postings",
                self.heads
                    .values()
                    .flat_map(|h| h.columns.iter())
                    .flat_map(HashMap::values)
                    .map(Vec::len)
                    .sum(),
            ),
            (
                "index.atomic_postings",
                self.heads
                    .values()
                    .flat_map(|h| h.atomic.iter())
                    .map(Vec::len)
                    .sum(),
            ),
            (
                "index.keys",
                self.heads
                    .values()
                    .flat_map(|h| h.columns.iter())
                    .map(HashMap::len)
                    .sum(),
            ),
            (
                "index.scalar_source_memberships",
                self.scalar_sources.values().map(BTreeSet::len).sum(),
            ),
            (
                "storage.current_set_versions",
                self.unary.len()
                    + self.families.len()
                    + self.links.values().map(HashMap::len).sum::<usize>()
                    + self.links.len()
                    + self.scalar_sources.len(),
            ),
            (
                "storage.current_memberships",
                self.unary
                    .values()
                    .chain(self.families.values())
                    .map(BTreeSet::len)
                    .sum::<usize>()
                    + self
                        .links
                        .values()
                        .flat_map(HashMap::values)
                        .map(|d| d.values().map_or(1, BTreeSet::len))
                        .sum::<usize>()
                    + self.links.values().map(HashMap::len).sum::<usize>()
                    + self
                        .scalar_sources
                        .values()
                        .map(BTreeSet::len)
                        .sum::<usize>(),
            ),
        ] {
            metrics.add(key, value);
        }
        metrics.sets(
            "unary",
            self.unary.values().map(|s| s.iter().copied().collect()),
        );
        metrics.sets(
            "families",
            self.families.values().map(|s| s.iter().copied().collect()),
        );
        metrics.sets(
            "unary_all",
            self.unary
                .values()
                .chain(self.families.values())
                .map(|s| s.iter().copied().collect()),
        );
        metrics.sets(
            "links.targets",
            self.links
                .values()
                .flat_map(HashMap::values)
                .map(crate::census::domain_values),
        );
        census.borrow().metrics(&self.links, &mut metrics)?;
        metrics.write(out, name)
    }
    pub fn exact_dump(&self) -> crate::front::Dump {
        (
            self.unary.iter().map(|(f, s)| (*f, s.clone())).collect(),
            self.links
                .iter()
                .flat_map(|((s, l), owners)| {
                    owners.iter().map(move |(o, t)| ((*o, *s, *l), t.clone()))
                })
                .collect(),
        )
    }
    pub fn feature_counts(&self) -> (usize, usize) {
        (
            self.unary.values().map(BTreeSet::len).sum(),
            self.links
                .values()
                .flat_map(HashMap::values)
                .map(|d| d.values().map_or(1, BTreeSet::len))
                .sum(),
        )
    }
    pub fn fact_count(&self) -> usize {
        let (unary, links) = self.feature_counts();
        unary + links
    }
    pub fn refines_reference(&self, reference: &Self, arena: &Arena) -> Result<(), String> {
        if self.mode != Mode::Refined
            || reference.mode != Mode::Reference
            || !Rc::ptr_eq(&self.abs, &reference.abs)
        {
            return Err(
                "feature comparison requires shared abstraction + refined/reference models".into(),
            );
        }
        for (feature, values) in &self.unary {
            let allowed = reference
                .unary
                .get(feature)
                .ok_or_else(|| format!("missing reference feature {feature:?}"))?;
            if !values.is_subset(allowed) && !allowed.contains(&Value::Any) {
                return Err(format!("unary refinement not contained: {feature:?}"));
            }
        }
        for (key @ (source, label), owners) in &self.links {
            let domain = Domain::one(*source);
            let mut allowed = Domain::empty();
            for broad in [*source, Value::Any, Value::Patient, Value::Number] {
                if domain.meet(&Domain::one(broad), arena) != domain {
                    continue;
                }
                for label in [*label, Label(label.0, Value::Any)] {
                    if let Some(targets) = reference
                        .links
                        .get(&(broad, label))
                        .and_then(|o| o.get(&Value::Any))
                    {
                        allowed.union(targets);
                    }
                }
            }
            for targets in owners.values() {
                if targets.meet(&allowed, arena) != targets.meet(&Domain::All, arena) {
                    return Err(format!("link refinement not contained: {key:?}"));
                }
            }
        }
        Ok(())
    }
    pub fn describe_state(&self, arena: &Arena, pattern: &Pattern, state: &State) -> String {
        let mut out = String::new();
        for (i, t) in pattern.goals.iter().enumerate() {
            let (u, l) = literal_features(arena, &self.abs, *t);
            out.push_str(&format!("G{} {}", i + 1, arena.render(*t)));
            if builtin(arena, *t) {
                out.push_str(" built-in(truth-unrefined)");
            }
            for unary in u {
                out.push_str(&format!(
                    " {}={}",
                    match unary.feature {
                        Feature::Noun(v) => format!("noun({})", self.abs.describe(arena, v)),
                        Feature::Lemma(v) => format!("lemma({})", self.abs.describe(arena, v)),
                        Feature::Operator(v) =>
                            format!("operator({})", self.abs.describe(arena, v)),
                        Feature::Property(v) =>
                            format!("property({})", self.abs.describe(arena, v)),
                        Feature::Payload(p, v) =>
                            format!("payload({},{})", arena.name(p), self.abs.describe(arena, v)),
                        Feature::Tag(v) => format!("origin_tag({})", self.abs.describe(arena, v)),
                    },
                    self.describe_domain(arena, &state.domain(unary.term))
                ));
            }
            for link in l {
                out.push_str(&format!(
                    " link({},{},owner={})={}->{}",
                    arena.name(link.label.0),
                    self.abs.describe(arena, link.label.1),
                    self.describe_domain(arena, &state.domain(link.owner)),
                    self.describe_domain(arena, &state.domain(link.source)),
                    self.describe_domain(arena, &state.domain(link.target))
                ));
            }
            out.push('\n');
        }
        out
    }
    pub fn describe_domain(&self, arena: &Arena, d: &Domain) -> String {
        match d {
            Domain::All => "any(missing-precision)".into(),
            Domain::Set(s) => s
                .iter()
                .map(|v| self.abs.describe(arena, *v))
                .collect::<Vec<_>>()
                .join("|"),
        }
    }
}
