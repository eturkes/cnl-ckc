use crate::program::{Clause, Goal};
use crate::query::Query;
use crate::term::{Arena, Id, Node, Symbol};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

type Predicate = (Symbol, usize);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Key {
    Rigid(Id),
    Identity(Vec<Id>),
    Functor(Symbol, usize),
}
fn key(arena: &Arena, term: Id) -> Option<Key> {
    match arena.node(term) {
        Node::Var(_) => None,
        Node::Compound(name, args) => {
            if arena.name(*name) == "$guideline_id"
                && args.len() == 5
                && args[..4].iter().all(|arg| arena.is_ground(*arg))
            {
                Some(Key::Identity(args[..4].to_vec()))
            } else {
                Some(Key::Functor(*name, args.len()))
            }
        }
        _ => Some(Key::Rigid(term)),
    }
}
fn compatible(arena: &Arena, a: Id, b: Id) -> bool {
    match (arena.node(a), arena.node(b)) {
        (Node::Var(_), _) | (_, Node::Var(_)) => true,
        (Node::Compound(a, aa), Node::Compound(b, bb)) => {
            a == b
                && aa.len() == bb.len()
                && aa.iter().zip(bb).all(|(a, b)| compatible(arena, *a, *b))
        }
        _ => a == b,
    }
}
fn meet(arena: &mut Arena, a: Id, b: Id) -> Option<Id> {
    match (arena.node(a).clone(), arena.node(b).clone()) {
        (Node::Var(_), _) => Some(b),
        (_, Node::Var(_)) => Some(a),
        (Node::Compound(a, aa), Node::Compound(b, bb)) if a == b && aa.len() == bb.len() => {
            let args = aa
                .into_iter()
                .zip(bb)
                .map(|(a, b)| meet(arena, a, b))
                .collect::<Option<Vec<_>>>()?;
            Some(arena.put(Node::Compound(a, args)))
        }
        _ => (a == b).then_some(a),
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Domain {
    Any,
    Terms(Vec<Id>),
}
impl Domain {
    fn terms(arena: &Arena, terms: impl IntoIterator<Item = Id>) -> Self {
        let mut values = terms.into_iter().collect::<Vec<_>>();
        if values
            .iter()
            .any(|id| matches!(arena.node(*id), Node::Var(_)))
        {
            return Self::Any;
        }
        values.sort_unstable();
        values.dedup();
        Self::Terms(values)
    }
    fn intersection(&self, arena: &mut Arena, other: &Self) -> Self {
        match (self, other) {
            (Self::Any, _) => other.clone(),
            (_, Self::Any) => self.clone(),
            (Self::Terms(a), Self::Terms(b)) => {
                let mut terms = Vec::new();
                for a in a {
                    for b in b {
                        if let Some(term) = meet(arena, *a, *b) {
                            terms.push(term);
                        }
                    }
                }
                Self::terms(arena, terms)
            }
        }
    }
    fn permits(&self, arena: &Arena, term: Id) -> bool {
        match self {
            Self::Any => true,
            Self::Terms(terms) => terms.iter().any(|p| compatible(arena, *p, term)),
        }
    }
    fn empty(&self) -> bool {
        matches!(self, Self::Terms(terms) if terms.is_empty())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Call {
    predicate: Predicate,
    args: Vec<Domain>,
}

#[derive(Default)]
pub(crate) struct HeadIndex {
    heads: HashMap<usize, Vec<Id>>,
    predicates: HashMap<Predicate, Vec<usize>>,
    slots: HashMap<(Predicate, usize, Key), Vec<usize>>,
    wild: HashMap<(Predicate, usize), Vec<usize>>,
}
impl HeadIndex {
    pub(crate) fn build(arena: &Arena, clauses: &[Clause], offset: usize) -> Self {
        let mut index = Self::default();
        for (i, clause) in clauses.iter().enumerate() {
            let row = offset + i;
            let (name, args) = arena.call(clause.head).expect("admitted head");
            let predicate = (name, args.len());
            index.predicates.entry(predicate).or_default().push(row);
            for (position, arg) in args.iter().enumerate() {
                if let Some(key) = key(arena, *arg) {
                    index
                        .slots
                        .entry((predicate, position, key))
                        .or_default()
                        .push(row);
                } else {
                    index
                        .wild
                        .entry((predicate, position))
                        .or_default()
                        .push(row);
                }
            }
            index.heads.insert(row, args.to_vec());
        }
        index
    }
    fn route(&self, arena: &Arena, call: &Call) -> (usize, Option<(usize, HashSet<Key>)>) {
        let mut size = self.predicates.get(&call.predicate).map_or(0, Vec::len);
        let mut route = None;
        for (position, domain) in call.args.iter().enumerate() {
            let Domain::Terms(terms) = domain else {
                continue;
            };
            let keys = terms
                .iter()
                .map(|term| key(arena, *term))
                .collect::<Option<HashSet<_>>>();
            let Some(keys) = keys else { continue };
            let wild = self
                .wild
                .get(&(call.predicate, position))
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let next_size = wild.len()
                + keys
                    .iter()
                    .map(|key| {
                        self.slots
                            .get(&(call.predicate, position, key.clone()))
                            .map_or(0, Vec::len)
                    })
                    .sum::<usize>();
            if next_size < size {
                size = next_size;
                route = Some((position, keys));
            }
        }
        (size, route)
    }
    fn candidates(&self, arena: &Arena, call: &Call) -> Vec<usize> {
        let (size, route) = self.route(arena, call);
        if size == 0 {
            return Vec::new();
        }
        let rows = if let Some((position, keys)) = route {
            let mut rows = self
                .wild
                .get(&(call.predicate, position))
                .cloned()
                .unwrap_or_default();
            for key in keys {
                rows.extend(
                    self.slots
                        .get(&(call.predicate, position, key))
                        .into_iter()
                        .flatten(),
                );
            }
            rows.sort_unstable();
            rows
        } else {
            self.predicates
                .get(&call.predicate)
                .cloned()
                .unwrap_or_default()
        };
        rows.into_iter()
            .filter(|row| {
                call.args
                    .iter()
                    .zip(&self.heads[row])
                    .all(|(domain, term)| domain.permits(arena, *term))
            })
            .collect()
    }
}

struct View<'a> {
    base: &'a HeadIndex,
    patient: &'a HeadIndex,
}
impl View<'_> {
    fn estimate(&self, arena: &Arena, call: &Call) -> usize {
        self.base.route(arena, call).0 + self.patient.route(arena, call).0
    }
    fn candidates(&self, arena: &Arena, call: &Call) -> Vec<usize> {
        let mut rows = self.base.candidates(arena, call);
        rows.extend(self.patient.candidates(arena, call));
        rows
    }
    fn args(&self, row: usize) -> &[Id] {
        self.base
            .heads
            .get(&row)
            .or_else(|| self.patient.heads.get(&row))
            .expect("indexed head")
    }
}

fn abstract_term(arena: &mut Arena, term: Id, env: &[Domain]) -> Domain {
    match arena.node(term).clone() {
        Node::Var(v) => env.get(v).cloned().unwrap_or(Domain::Any),
        Node::Compound(name, args) => {
            // Keep rigid constructor prefixes. Nested alternatives lose correlation,
            // so only singleton substitutions strengthen a constructor pattern.
            let any = arena.put(Node::Var(0));
            let mut out = Vec::with_capacity(args.len());
            for arg in args {
                out.push(match abstract_term(arena, arg, env) {
                    Domain::Terms(values) if values.len() == 1 => values[0],
                    Domain::Terms(values) if values.is_empty() => return Domain::Terms(Vec::new()),
                    _ => any,
                });
            }
            let term = arena.put(Node::Compound(name, out));
            Domain::Terms(vec![term])
        }
        _ => Domain::Terms(vec![term]),
    }
}
fn call(arena: &mut Arena, term: Id, env: &[Domain]) -> Call {
    let (name, args) = arena.call(term).expect("admitted call");
    let args = args.to_vec();
    Call {
        predicate: (name, args.len()),
        args: args
            .into_iter()
            .map(|arg| abstract_term(arena, arg, env))
            .collect(),
    }
}
fn constrain(arena: &mut Arena, term: Id, possible: &Domain, env: &mut [Domain]) -> bool {
    match arena.node(term).clone() {
        Node::Var(v) => {
            env[v] = env[v].intersection(arena, possible);
            !env[v].empty()
        }
        Node::Compound(name, args) => {
            let Domain::Terms(terms) = possible else {
                return true;
            };
            if terms.iter().any(|t| matches!(arena.node(*t), Node::Var(_))) {
                return true;
            }
            for (position, arg) in args.iter().enumerate() {
                let values = terms
                    .iter()
                    .filter_map(|t| {
                        let (other, values) = arena.call(*t)?;
                        (name == other && args.len() == values.len()).then_some(values[position])
                    })
                    .collect::<Vec<_>>();
                let values = Domain::terms(arena, values);
                if !constrain(arena, *arg, &values, env) {
                    return false;
                }
            }
            true
        }
        _ => possible.permits(arena, term),
    }
}
fn refine(arena: &mut Arena, view: &View<'_>, goals: &[Goal], env: &mut [Domain]) -> bool {
    loop {
        let previous = env.to_vec();
        let mut remaining = goals
            .iter()
            .filter_map(|g| {
                if let Goal::Positive(term) = g {
                    Some(*term)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        while !remaining.is_empty() {
            let at = remaining
                .iter()
                .enumerate()
                .min_by_key(|(_, term)| {
                    let goal = call(arena, **term, env);
                    view.estimate(arena, &goal)
                })
                .map(|(at, _)| at)
                .expect("positive goals");
            let term = remaining.swap_remove(at);
            let goal = call(arena, term, env);
            let rows = view.candidates(arena, &goal);
            if rows.is_empty() {
                return false;
            }
            let (_, args) = arena.call(term).expect("goal call");
            let args = args.to_vec();
            for (position, arg) in args.into_iter().enumerate() {
                let values = Domain::terms(arena, rows.iter().map(|row| view.args(*row)[position]));
                if !constrain(arena, arg, &values, env) {
                    return false;
                }
            }
        }
        if env == previous {
            return true;
        }
    }
}
fn demands(
    arena: &mut Arena,
    view: &View<'_>,
    goals: &[Goal],
    mut env: Vec<Domain>,
    out: &mut VecDeque<Call>,
) {
    if !refine(arena, view, goals, &mut env) {
        return;
    }
    for goal in goals {
        match goal {
            Goal::Positive(term) => out.push_back(call(arena, *term, &env)),
            Goal::Not(inner) => demands(arena, view, inner, env.clone(), out),
            Goal::Builtin(_) => {}
        }
    }
}

pub(crate) fn static_slice(
    arena: &mut Arena,
    clauses: &[Clause],
    base: &HeadIndex,
    patient: &HeadIndex,
    query: &Query,
) -> Vec<usize> {
    let Query::Projected { vars, goals, .. } = query else {
        // q-all includes every context payload, not only the operator edge.
        return (0..clauses.len()).collect();
    };
    let view = View { base, patient };
    let mut queue = VecDeque::new();
    demands(arena, &view, goals, vec![Domain::Any; *vars], &mut queue);
    let mut seen = HashSet::new();
    let mut selected = BTreeSet::new();
    while let Some(call) = queue.pop_front() {
        if !seen.insert(call.clone()) {
            continue;
        }
        for row in view.candidates(arena, &call) {
            if !selected.insert(row) {
                continue;
            }
            let clause = &clauses[row];
            // Selection keeps whole clauses. Close all their possible body calls,
            // rather than assuming the query's head binding guards the clause.
            demands(
                arena,
                &view,
                &clause.body,
                vec![Domain::Any; clause.names.len()],
                &mut queue,
            );
        }
    }
    selected.into_iter().collect()
}
