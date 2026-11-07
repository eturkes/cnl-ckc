//! Necessary head filters; survivors still run the full native-ordered recipe.
use crate::domain::{Abstraction, Value};
use crate::exact::{Model, State, expr};
use crate::exact_sets::{Set, Sets};
use crate::features::Expr;
use crate::source::Program;
use sb1_checker::term::{Arena, Id, Node, Symbol};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::time::Instant;

#[path = "candidate_probe.rs"]
mod probe;
pub use probe::ForcedTimer;
pub fn enable_census(out: std::path::PathBuf) {
    probe::enable(out);
}
pub struct QueryContext<'a> {
    pub phase: &'static str,
    pub caller: usize,
    pub pattern: &'a crate::features::Pattern,
    pub naf: usize,
}

#[cfg(test)]
thread_local! { static EQUALITY_FILTER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }

struct Argument {
    values: Option<std::rc::Rc<[Value]>>,
    any: bool,
    patient: bool,
    number: bool,
}
impl Argument {
    fn new(domain: Set, sets: &Sets) -> Self {
        let values = sets.values(domain);
        let has = |v| values.as_ref().is_some_and(|s| s.binary_search(&v).is_ok());
        Self {
            any: has(Value::Any),
            patient: has(Value::Patient),
            number: has(Value::Number),
            values,
        }
    }
    fn accepts(&self, head: Value, arena: &Arena) -> bool {
        let Some(values) = &self.values else {
            return true;
        };
        if head == Value::Any {
            return true;
        }
        #[cfg(test)]
        if EQUALITY_FILTER.with(std::cell::Cell::get) {
            return values.binary_search(&head).is_ok();
        }
        if self.any || values.binary_search(&head).is_ok() {
            return true;
        }
        if self.patient && Abstraction::intersect(arena, Value::Patient, head).is_some()
            || self.number && Abstraction::intersect(arena, Value::Number, head).is_some()
        {
            return true;
        }
        matches!(head, Value::Patient | Value::Number)
            && values
                .iter()
                .any(|v| Abstraction::intersect(arena, *v, head).is_some())
    }
}

#[derive(Default)]
struct Column {
    values: HashMap<Value, Vec<usize>>,
    exact: Vec<usize>,
    #[cfg(test)]
    atomic: Vec<usize>,
    #[cfg(test)]
    numeric: Vec<usize>,
}
impl Column {
    fn insert(&mut self, value: Value, row: usize, _arena: &Arena) {
        self.values.entry(value).or_default().push(row);
        if matches!(value, Value::Exact(_)) {
            self.exact.push(row);
        }
        #[cfg(test)]
        if let Value::Exact(t) = value {
            if matches!(_arena.node(t), Node::Atom(_) | Node::Integer(_))
                || _arena.named(t, "rational", 2).is_some()
            {
                self.atomic.push(row);
            }
            if matches!(_arena.node(t), Node::Integer(_))
                || _arena.named(t, "rational", 2).is_some()
            {
                self.numeric.push(row);
            }
        }
    }
    #[cfg(test)]
    fn compatible(&self, domain: Set, sets: &Sets, arena: &Arena, all: &[usize]) -> Vec<usize> {
        let Some(values) = sets.values(domain) else {
            return all.to_vec();
        };
        #[cfg(test)]
        if EQUALITY_FILTER.with(std::cell::Cell::get) {
            return values
                .iter()
                .chain([&Value::Any])
                .filter_map(|v| self.values.get(v))
                .flatten()
                .copied()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
        }
        if values.contains(&Value::Any) {
            return all.to_vec();
        }
        let mut out = BTreeSet::new();
        if let Some(wild) = self.values.get(&Value::Any) {
            out.extend(wild);
        }
        for v in values.iter() {
            if let Some(exact) = self.values.get(v) {
                out.extend(exact);
            }
            for broad in [Value::Patient, Value::Number] {
                if Abstraction::intersect(arena, *v, broad).is_some()
                    && let Some(rows) = self.values.get(&broad)
                {
                    out.extend(rows);
                }
            }
            match v {
                Value::Patient => out.extend(&self.atomic),
                Value::Number => out.extend(&self.numeric),
                _ => {}
            }
        }
        out.into_iter().collect()
    }
}
#[derive(Default)]
struct Heads {
    rows: Vec<usize>,
    columns: Vec<Column>,
    audit_native: Option<crate::features::Heads>,
}
impl Heads {
    fn native(&self, args: &[Expr], state: &State, sets: &Sets) -> Vec<usize> {
        let mut best: Option<(usize, Vec<&[usize]>)> = None;
        for (arg, column) in args.iter().zip(&self.columns) {
            let Some(values) = sets.values(state.domain(*arg, sets)) else {
                continue;
            };
            let mut buckets = Vec::new();
            if let Some(wild) = column.values.get(&Value::Any) {
                buckets.push(wild.as_slice());
            }
            for v in values.iter() {
                if let Some(exact) = column.values.get(v) {
                    buckets.push(exact.as_slice());
                }
                if matches!(v, Value::Patient | Value::Number) {
                    buckets.push(column.exact.as_slice());
                }
            }
            let size = buckets.iter().map(|b| b.len()).sum();
            if best.as_ref().is_none_or(|(old, _)| size < *old) {
                best = Some((size, buckets));
            }
        }
        best.map_or_else(
            || self.rows.clone(),
            |(_, buckets)| {
                buckets
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect()
            },
        )
    }
}

pub struct Index {
    predicates: HashMap<Symbol, Heads>,
    trace: Option<Rc<RefCell<probe::Trace>>>,
}
impl Drop for Index {
    fn drop(&mut self) {
        if let Some(trace) = &self.trace {
            probe::Trace::finish(trace);
        }
    }
}
impl Index {
    pub fn new(program: &Program, model: &Model, audit: bool) -> Self {
        Self::selected(program, model, None, audit)
    }
    pub fn selected(
        program: &Program,
        model: &Model,
        selected: Option<&BTreeSet<usize>>,
        audit: bool,
    ) -> Self {
        let mut predicates = HashMap::<Symbol, Heads>::new();
        for (i, c) in program.clauses.iter().enumerate() {
            if selected.is_some_and(|s| !s.contains(&i))
                || model.generic_patient() && program.patient(i)
            {
                continue;
            }
            let (p, args) = program.arena.call(c.head).unwrap();
            let h = predicates.entry(p).or_default();
            if audit {
                h.audit_native
                    .get_or_insert_with(crate::features::Heads::default)
                    .insert(i, args, &program.arena, &model.abs);
            }
            h.rows.push(i);
            h.columns.resize_with(args.len(), Column::default);
            for (t, column) in args.iter().zip(&mut h.columns) {
                let value = match program.arena.node(*t) {
                    Node::Var(_) => Value::Any,
                    _ => model.abs.value(*t),
                };
                column.insert(value, i, &program.arena);
            }
        }
        Self {
            predicates,
            trace: probe::Trace::new(program),
        }
    }
    pub fn candidates(
        &self,
        program: &Program,
        model: &Model,
        goal: Id,
        state: &State,
        audit: bool,
    ) -> Vec<usize> {
        self.candidates_in(program, model, goal, state, audit, None)
    }
    pub fn candidates_in(
        &self,
        program: &Program,
        model: &Model,
        goal: Id,
        state: &State,
        audit: bool,
        context: Option<QueryContext<'_>>,
    ) -> Vec<usize> {
        let all_timer = self.trace.as_ref().map(|_| Instant::now());
        model.stats.borrow_mut().header_requests += 1;
        let (p, terms) = program.arena.call(goal).unwrap();
        let args = terms
            .iter()
            .map(|t| expr(&program.arena, &model.abs, *t))
            .collect::<Vec<_>>();
        let Some(heads) = self.predicates.get(&p) else {
            if let (Some(trace), Some(context)) = (&self.trace, &context) {
                let empty = Heads::default();
                let ticket = trace.borrow_mut().record(probe::Observation {
                    program,
                    model,
                    heads: &empty,
                    goal,
                    state,
                    context,
                    native: &[],
                    survivors: &[],
                    rejects: vec![0; args.len()],
                    points: 0,
                    point_pass: 0,
                    times: [0, 0, 0, all_timer.map_or(0, |t| t.elapsed().as_nanos())],
                });
                probe::Trace::activate(trace.clone(), ticket);
            }
            return Vec::new();
        };
        let timer = self.trace.as_ref().map(|_| Instant::now());
        let native = heads.native(&args, state, &model.sets);
        let native_ns = timer.map_or(0, |t| t.elapsed().as_nanos());
        model.stats.borrow_mut().header_candidates += native.len();
        let arguments = args
            .iter()
            .map(|arg| Argument::new(state.domain(*arg, &model.sets), &model.sets))
            .collect::<Vec<_>>();
        let mut filtered = native.clone();
        let mut point_checks = 0;
        let mut rejects = vec![0; if self.trace.is_some() { args.len() } else { 0 }];
        let timer = self.trace.as_ref().map(|_| Instant::now());
        // The native posting already narrows the row universe. Probe other
        // columns on those rows; broad unions would rebuild unselective postings
        // for every narrow query.
        filtered.retain(|to| {
            let (_, head) = program.arena.call(program.clauses[*to].head).unwrap();
            arguments
                .iter()
                .zip(head)
                .enumerate()
                .all(|(at, (argument, t))| {
                    point_checks += 1;
                    let value = match program.arena.node(*t) {
                        Node::Var(_) => Value::Any,
                        _ => model.abs.value(*t),
                    };
                    let accepted = argument.accepts(value, &program.arena);
                    if !accepted && !rejects.is_empty() {
                        rejects[at] += 1;
                    }
                    accepted
                })
        });
        let point_ns = timer.map_or(0, |t| t.elapsed().as_nanos());
        let point_pass = filtered.len();
        model.stats.borrow_mut().header_point_checks += point_checks;
        let timer = self.trace.as_ref().map(|_| Instant::now());
        filtered.retain(|to| {
            let c = &program.clauses[*to];
            let (_, head) = program.arena.call(c.head).unwrap();
            let head = head
                .iter()
                .map(|t| expr(&program.arena, &model.abs, *t))
                .collect::<Vec<_>>();
            small_prefix(
                &program.arena,
                &model.sets,
                &args,
                &head,
                c.names.len(),
                state,
            )
        });
        let prefix_ns = timer.map_or(0, |t| t.elapsed().as_nanos());
        if let (Some(trace), Some(context)) = (&self.trace, &context) {
            let ticket = trace.borrow_mut().record(probe::Observation {
                program,
                model,
                heads,
                goal,
                state,
                context,
                native: &native,
                survivors: &filtered,
                rejects,
                points: point_checks,
                point_pass,
                times: [
                    native_ns,
                    point_ns,
                    prefix_ns,
                    all_timer.map_or(0, |t| t.elapsed().as_nanos()),
                ],
            });
            probe::Trace::activate(trace.clone(), ticket);
        }
        if audit {
            let original = crate::features::State::audit_state(
                state.parent.clone(),
                state
                    .domains
                    .iter()
                    .map(|s| model.sets.domain(*s))
                    .collect(),
            );
            let mut scanned = 0;
            let actual_native = heads
                .audit_native
                .as_ref()
                .expect("native audit index")
                .candidates(terms, &original, &program.arena, &model.abs, &mut scanned);
            assert_eq!(native, actual_native, "native candidate recipe");
            assert!(
                filtered
                    .iter()
                    .all(|to| actual_native.binary_search(to).is_ok()),
                "candidate enlarged native recipe"
            );
            for to in native {
                model.stats.borrow_mut().header_audits += 1;
                let c = &program.clauses[to];
                let (_, h) = program.arena.call(c.head).unwrap();
                let head = h
                    .iter()
                    .map(|t| expr(&program.arena, &model.abs, *t))
                    .collect::<Vec<_>>();
                let expected =
                    original.audit_header_prefix(&program.arena, &args, &head, c.names.len());
                let small = small_prefix(
                    &program.arena,
                    &model.sets,
                    &args,
                    &head,
                    c.names.len(),
                    state,
                );
                assert_eq!(small, expected, "small unifier at {}", program.label(to));
                assert_eq!(
                    filtered.binary_search(&to).is_ok(),
                    expected,
                    "head filter at {}",
                    program.label(to)
                );
            }
        }
        model.stats.borrow_mut().header_survivors += filtered.len();
        filtered
    }
}

pub fn small_prefix(
    arena: &Arena,
    sets: &Sets,
    goal: &[Expr],
    head: &[Expr],
    head_vars: usize,
    base: &State,
) -> bool {
    let mut roots = HashMap::new();
    let mut domains = Vec::new();
    let goal = goal
        .iter()
        .map(|e| match e {
            Expr::Var(v) => {
                let root = base.root(*v);
                let next = roots.len();
                let index = *roots.entry(root).or_insert_with(|| {
                    domains.push(base.domains[root]);
                    next
                });
                Expr::Var(index)
            }
            x => *x,
        })
        .collect::<Vec<_>>();
    let offset = roots.len();
    let mut state = State::new(offset + head_vars);
    state.domains[..offset].copy_from_slice(&domains);
    for (g, h) in goal.iter().zip(head) {
        let h = match h {
            Expr::Var(v) => Expr::Var(offset + v),
            x => *x,
        };
        if !state.unify(*g, h, sets, arena) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Domain;
    use crate::exact_sets::{ALL, EMPTY};
    fn coarse(
        arena: &Arena,
        sets: &Sets,
        goal: &[Expr],
        head: &[Expr],
        base: &State,
        equality_plant: bool,
    ) -> bool {
        EQUALITY_FILTER.with(|fault| fault.set(equality_plant));
        let result = goal.iter().zip(head).all(|(g, h)| {
            let value = match h {
                Expr::Constant(v) => *v,
                Expr::Var(_) => Value::Any,
            };
            let mut column = Column::default();
            column.insert(value, 0, arena);
            let domain = base.domain(*g, sets);
            let actual = Argument::new(domain, sets).accepts(value, arena);
            assert_eq!(
                actual,
                column.compatible(domain, sets, arena, &[0]).contains(&0)
            );
            actual
        });
        EQUALITY_FILTER.with(|fault| fault.set(false));
        result
    }
    #[test]
    fn generated_header_prefix_exactness_and_conservative_filter() {
        let mut arena = Arena::default();
        let atom = arena.atom("a");
        let atom2 = arena.atom("b");
        let n = arena.put(Node::Integer("1".into()));
        let d = arena.put(Node::Integer("2".into()));
        let rational = arena.compound("rational", vec![n, d]);
        let other = arena.compound("other", vec![atom]);
        let values = [
            Value::Any,
            Value::Patient,
            Value::Number,
            Value::Exact(atom),
            Value::Exact(atom2),
            Value::Exact(n),
            Value::Exact(rational),
            Value::Exact(other),
            Value::Origin(0),
            Value::Origin(1),
        ];
        let sets = Sets::default();
        let mut rng = 0xa5f0_126a_c985_731du64;
        let mut next = |n| {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng as usize % n
        };
        let mut plant_catches = 0;
        for case in 0..30_000 {
            let vars = 1 + next(6);
            let mut base = State::new(vars);
            for v in 0..vars {
                base.parent[v] = next(v + 1);
                base.domains[v] = match next(5) {
                    0 => ALL,
                    1 => EMPTY,
                    _ => sets.intern(
                        (0..1 + next(5))
                            .map(|_| values[next(values.len())])
                            .collect(),
                    ),
                };
            }
            let arity = 1 + next(7);
            let head_vars = 1 + next(5);
            let goal = (0..arity)
                .map(|_| {
                    if next(3) == 0 {
                        Expr::Constant(values[next(values.len())])
                    } else {
                        Expr::Var(next(vars))
                    }
                })
                .collect::<Vec<_>>();
            let head = (0..arity)
                .map(|_| {
                    if next(2) == 0 {
                        Expr::Constant(values[next(values.len())])
                    } else {
                        Expr::Var(next(head_vars))
                    }
                })
                .collect::<Vec<_>>();
            let native = crate::features::State::audit_state(
                base.parent.clone(),
                base.domains
                    .iter()
                    .map(|id| sets.domain(*id))
                    .collect::<Vec<Domain>>(),
            );
            let expected = native.audit_header_prefix(&arena, &goal, &head, head_vars);
            assert_eq!(
                small_prefix(&arena, &sets, &goal, &head, head_vars, &base),
                expected,
                "case {case}: {goal:?} {head:?}"
            );
            if expected {
                assert!(
                    coarse(&arena, &sets, &goal, &head, &base, false),
                    "case {case}"
                );
                plant_catches += usize::from(!coarse(&arena, &sets, &goal, &head, &base, true));
            }
        }
        assert!(
            plant_catches > 0,
            "equality-for-compatibility plant must turn property (1) red"
        );
        eprintln!(
            "header property: 30000 cases; equality plant rejected {plant_catches} successful native prefixes"
        );
    }
}
