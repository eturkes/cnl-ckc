//! Flag-gated D6 census + primitive replays. No candidate result is replaced.
use super::{Argument, Heads, QueryContext};
use crate::domain::Value;
use crate::exact::{Model, State};
use crate::features::Expr;
use crate::source::Program;
use sb1_checker::term::{Id, Node, Symbol};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Instant;

thread_local! {
    static CONFIG: RefCell<Option<(PathBuf, usize)>> = const { RefCell::new(None) };
    static ACTIVE: RefCell<Option<(Rc<RefCell<Trace>>, usize)>> = const { RefCell::new(None) };
}
pub fn enable(out: PathBuf) {
    CONFIG.with_borrow_mut(|c| *c = Some((out, 0)));
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Key {
    phase: &'static str,
    epoch: usize,
    goal: Id,
    domains: Vec<usize>,
    aliases: Vec<usize>,
    naf: usize,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FullKey {
    key: Key,
    goals: Vec<Id>,
    vars: usize,
    parent: Vec<usize>,
    domains: Vec<usize>,
}
#[derive(Clone, Copy)]
struct Seen {
    id: usize,
    group: Option<usize>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Bucket {
    Value(Value),
    Exact,
}
struct Plan<'a> {
    at: usize,
    size: usize,
    buckets: Vec<(Bucket, &'a [usize])>,
}
fn plans<'a>(heads: &'a Heads, args: &[Expr], state: &State, model: &Model) -> Vec<Plan<'a>> {
    args.iter()
        .zip(&heads.columns)
        .enumerate()
        .filter_map(|(at, (arg, col))| {
            let values = model.sets.values(state.domain(*arg, &model.sets))?;
            let mut buckets = Vec::new();
            if let Some(v) = col.values.get(&Value::Any) {
                buckets.push((Bucket::Value(Value::Any), v.as_slice()));
            }
            for value in values.iter() {
                if let Some(v) = col.values.get(value) {
                    buckets.push((Bucket::Value(*value), v.as_slice()));
                }
                if matches!(value, Value::Patient | Value::Number) {
                    buckets.push((Bucket::Exact, col.exact.as_slice()));
                }
            }
            Some(Plan {
                at,
                size: buckets.iter().map(|(_, v)| v.len()).sum(),
                buckets,
            })
        })
        .collect()
}
pub(super) fn merged(postings: &[&[usize]]) -> Vec<usize> {
    let mut heap = BinaryHeap::new();
    for (i, row) in postings.iter().enumerate() {
        if let Some(value) = row.first() {
            heap.push(Reverse((*value, i, 0)));
        }
    }
    let mut out = Vec::new();
    while let Some(Reverse((value, row, at))) = heap.pop() {
        if out.last() != Some(&value) {
            out.push(value);
        }
        if let Some(next) = postings[row].get(at + 1) {
            heap.push(Reverse((*next, row, at + 1)));
        }
    }
    out
}
struct Composite {
    groups: HashMap<Bucket, BTreeMap<Value, Vec<usize>>>,
}
impl Composite {
    fn build(program: &Program, model: &Model, heads: &Heads, first: usize, second: usize) -> Self {
        let mut groups = HashMap::new();
        let col = &heads.columns[first];
        for (bucket, rows) in col
            .values
            .iter()
            .map(|(v, r)| (Bucket::Value(*v), r.as_slice()))
            .chain([(Bucket::Exact, col.exact.as_slice())])
        {
            let group = groups
                .entry(bucket)
                .or_insert_with(BTreeMap::<Value, Vec<usize>>::new);
            for row in rows {
                let (_, args) = program.arena.call(program.clauses[*row].head).unwrap();
                let value = match program.arena.node(args[second]) {
                    Node::Var(_) => Value::Any,
                    _ => model.abs.value(args[second]),
                };
                group.entry(value).or_default().push(*row);
            }
        }
        Self { groups }
    }
    fn matching<'a>(
        &'a self,
        plan: &Plan<'_>,
        arg: &Argument,
        program: &Program,
    ) -> (Vec<&'a [usize]>, usize) {
        let mut rows = Vec::new();
        let mut checks = 0;
        let mut seen = BTreeSet::new();
        for (bucket, _) in &plan.buckets {
            if !seen.insert(*bucket) {
                continue;
            }
            if let Some(groups) = self.groups.get(bucket) {
                for (value, posting) in groups {
                    checks += 1;
                    if arg.accepts(*value, &program.arena) {
                        rows.push(posting.as_slice());
                    }
                }
            }
        }
        (rows, checks)
    }
    fn memberships(&self) -> usize {
        self.groups
            .values()
            .flat_map(BTreeMap::values)
            .map(Vec::len)
            .sum()
    }
}
struct Row {
    phase: &'static str,
    caller: usize,
    group: Option<usize>,
    epoch: usize,
    goal: Id,
    key: usize,
    hit: &'static str,
    full: usize,
    full_hit: &'static str,
    chosen: Option<usize>,
    sizes: Vec<usize>,
    cards: Vec<usize>,
    rejects: Vec<usize>,
    native: usize,
    points: usize,
    point_pass: usize,
    survivors: usize,
    native_ns: u128,
    point_ns: u128,
    prefix_ns: u128,
    merged_ns: u128,
    composite_ns: u128,
    composite_heads: usize,
    composite_groups: usize,
    composite_column: Option<usize>,
    forced: usize,
    forced_ns: u128,
    successes: usize,
    plan_ns: u128,
    key_ns: u128,
    composite_merge_ns: u128,
    composite_members: usize,
    composite_points: usize,
    index_total_ns: u128,
    key_cells: usize,
    cache_store_ns: u128,
    cache_hit_ns: u128,
}
pub(super) struct Trace {
    out: PathBuf,
    id: usize,
    groups: Vec<Option<usize>>,
    group_names: Vec<String>,
    keys: HashMap<Key, Seen>,
    full: HashMap<FullKey, Seen>,
    rows: Vec<Row>,
    composites: HashMap<(Symbol, usize, usize), Composite>,
    build_ns: u128,
    build_memberships: usize,
    terms: BTreeMap<Id, String>,
}
fn group(program: &Program, clause: usize) -> Option<String> {
    let doc = &program.documents[program.clauses[clause].document - 1];
    if doc.patient || doc.path == std::path::Path::new("k1-registry") {
        return None;
    }
    let root = doc.path.ancestors().find(|p| {
        p.parent()
            .and_then(std::path::Path::file_name)
            .is_some_and(|n| n == "copies" || n == "guidelines")
    });
    root.map(|root| root.display().to_string())
}
pub(super) struct Observation<'a> {
    pub program: &'a Program,
    pub model: &'a Model,
    pub heads: &'a Heads,
    pub goal: Id,
    pub state: &'a State,
    pub context: &'a QueryContext<'a>,
    pub native: &'a [usize],
    pub survivors: &'a [usize],
    pub rejects: Vec<usize>,
    pub points: usize,
    pub point_pass: usize,
    pub times: [u128; 4],
}
impl Trace {
    pub(super) fn new(program: &Program) -> Option<Rc<RefCell<Self>>> {
        let (out, id) = CONFIG.with_borrow_mut(|c| {
            let (out, id) = c.as_mut()?;
            let result = (out.clone(), *id);
            *id += 1;
            Some(result)
        })?;
        let mut names = Vec::new();
        let mut ids = HashMap::new();
        let groups = (0..program.clauses.len())
            .map(|i| {
                group(program, i).map(|name| {
                    let next = ids.len();
                    *ids.entry(name.clone()).or_insert_with(|| {
                        names.push(name);
                        next
                    })
                })
            })
            .collect();
        Some(Rc::new(RefCell::new(Self {
            out,
            id,
            groups,
            group_names: names,
            keys: HashMap::new(),
            full: HashMap::new(),
            rows: Vec::new(),
            composites: HashMap::new(),
            build_ns: 0,
            build_memberships: 0,
            terms: BTreeMap::new(),
        })))
    }
    fn key(
        program: &Program,
        model: &Model,
        goal: Id,
        state: &State,
        context: &QueryContext<'_>,
    ) -> (Key, FullKey) {
        let (_, terms) = program.arena.call(goal).unwrap();
        let mut roots = HashMap::new();
        let mut aliases = Vec::new();
        let mut domains = Vec::new();
        for t in terms {
            let expr = crate::exact::expr(&program.arena, &model.abs, *t);
            match expr {
                Expr::Var(v) => {
                    domains.push(state.domain(expr, &model.sets));
                    let next = roots.len();
                    aliases.push(*roots.entry(state.root(v)).or_insert(next));
                }
                Expr::Constant(_) => {
                    // The raw goal owns constants; reading telemetry must not
                    // intern a singleton on the absent-predicate fast path.
                    domains.push(usize::MAX);
                    aliases.push(usize::MAX);
                }
            }
        }
        let key = Key {
            phase: context.phase,
            epoch: model.stats.borrow().collections,
            goal,
            domains,
            aliases,
            naf: context.naf,
        };
        let full = FullKey {
            key: key.clone(),
            goals: context.pattern.goals.clone(),
            vars: context.pattern.vars,
            parent: state.parent.clone(),
            domains: state.domains.clone(),
        };
        (key, full)
    }
    fn hit<K: Eq + std::hash::Hash>(
        map: &mut HashMap<K, Seen>,
        key: K,
        group: Option<usize>,
    ) -> (usize, &'static str) {
        let next = map.len();
        match map.entry(key) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(Seen { id: next, group });
                (next, "new")
            }
            std::collections::hash_map::Entry::Occupied(e) => {
                let s = e.get();
                (
                    s.id,
                    if s.group.is_none() || group.is_none() {
                        "unclassified"
                    } else if s.group == group {
                        "same"
                    } else {
                        "cross"
                    },
                )
            }
        }
    }
    pub(super) fn record(&mut self, observation: Observation<'_>) -> usize {
        let Observation {
            program,
            model,
            heads,
            goal,
            state,
            context,
            native,
            survivors,
            rejects,
            points,
            point_pass,
            times,
        } = observation;
        let start = Instant::now();
        let (key, full) = Self::key(program, model, goal, state, context);
        let key_cells = 2 * (4 + key.domains.len() + key.aliases.len())
            + full.goals.len()
            + 1
            + full.parent.len()
            + full.domains.len();
        let group = self.groups.get(context.caller).copied().flatten();
        let (key_id, hit) = Self::hit(&mut self.keys, key.clone(), group);
        let (full_id, full_hit) = Self::hit(&mut self.full, full, group);
        let key_ns = start.elapsed().as_nanos();
        for term in std::iter::once(goal).chain(context.pattern.goals.iter().copied()) {
            self.terms
                .entry(term)
                .or_insert_with(|| program.arena.render(term));
        }
        let (_, terms) = program.arena.call(goal).unwrap();
        let args = terms
            .iter()
            .map(|t| crate::exact::expr(&program.arena, &model.abs, *t))
            .collect::<Vec<_>>();
        let start = Instant::now();
        let plans = plans(heads, &args, state, model);
        let plan = plans.iter().min_by_key(|p| p.size);
        let plan_ns = start.elapsed().as_nanos();
        let mut sizes = vec![usize::MAX; args.len()];
        for p in &plans {
            sizes[p.at] = p.size;
        }
        let cards = args
            .iter()
            .map(|a| match a {
                Expr::Constant(_) => 1,
                _ => model
                    .sets
                    .values(state.domain(*a, &model.sets))
                    .map_or(usize::MAX, |v| v.len()),
            })
            .collect();
        let mut merged_ns = 0;
        let mut composite_ns = 0;
        let mut composite_heads = native.len();
        let mut composite_groups = 0;
        let mut composite_column = None;
        let mut composite_merge_ns = 0;
        let mut composite_members = 0;
        let mut composite_points = points;
        if let Some(plan) = plan {
            let postings = plan.buckets.iter().map(|(_, v)| *v).collect::<Vec<_>>();
            let start = Instant::now();
            let merged_result = merged(&postings);
            merged_ns = start.elapsed().as_nanos();
            assert_eq!(merged_result, native, "merged native replay");
            let predicate = program.arena.call(goal).unwrap().0;
            // Demand pairs once per index; query planning inspects exact value
            // groups, then uses its smallest posting-size bound. No row oracle.
            for p in &plans {
                if !self.composites.contains_key(&(predicate, plan.at, p.at)) {
                    let start = Instant::now();
                    let c = Composite::build(program, model, heads, plan.at, p.at);
                    self.build_ns += start.elapsed().as_nanos();
                    self.build_memberships += c.memberships();
                    self.composites.insert((predicate, plan.at, p.at), c);
                }
            }
            let start = Instant::now();
            let mut best = None;
            for p in &plans {
                let arg = Argument::new(state.domain(args[p.at], &model.sets), &model.sets);
                let (rows, checks) =
                    self.composites[&(predicate, plan.at, p.at)].matching(plan, &arg, program);
                composite_groups += checks;
                let size = rows.iter().map(|r| r.len()).sum::<usize>();
                if best.as_ref().is_none_or(|(old, _, _)| size < *old) {
                    best = Some((size, p.at, rows));
                }
            }
            composite_ns = start.elapsed().as_nanos();
            if let Some((members, at, rows)) = best {
                composite_members = members;
                let start = Instant::now();
                let result = merged(&rows);
                composite_merge_ns = start.elapsed().as_nanos();
                composite_heads = result.len();
                composite_column = Some(at);
                let arg = Argument::new(state.domain(args[at], &model.sets), &model.sets);
                let expected = native
                    .iter()
                    .copied()
                    .filter(|to| {
                        let (_, head) = program.arena.call(program.clauses[*to].head).unwrap();
                        let value = match program.arena.node(head[at]) {
                            Node::Var(_) => Value::Any,
                            _ => model.abs.value(head[at]),
                        };
                        arg.accepts(value, &program.arena)
                    })
                    .collect::<Vec<_>>();
                assert_eq!(result, expected, "composite native replay");
                let arguments = args
                    .iter()
                    .map(|a| Argument::new(state.domain(*a, &model.sets), &model.sets))
                    .collect::<Vec<_>>();
                composite_points = 0;
                let passed = result
                    .iter()
                    .filter(|to| {
                        let (_, head) = program.arena.call(program.clauses[**to].head).unwrap();
                        arguments.iter().zip(head).all(|(arg, term)| {
                            composite_points += 1;
                            let value = match program.arena.node(*term) {
                                Node::Var(_) => Value::Any,
                                _ => model.abs.value(*term),
                            };
                            arg.accepts(value, &program.arena)
                        })
                    })
                    .count();
                assert_eq!(
                    passed, point_pass,
                    "composite keeps every point-filter survivor"
                );
            }
        }
        let start = Instant::now();
        let cached = Rc::<[usize]>::from(survivors.to_vec());
        let cache_store_ns = start.elapsed().as_nanos();
        let start = Instant::now();
        drop(std::hint::black_box(cached.clone()));
        let cache_hit_ns = start.elapsed().as_nanos();
        let ticket = self.rows.len();
        self.rows.push(Row {
            phase: context.phase,
            caller: context.caller,
            group,
            epoch: key.epoch,
            goal,
            key: key_id,
            hit,
            full: full_id,
            full_hit,
            chosen: plan.map(|p| p.at),
            sizes,
            cards,
            rejects,
            native: native.len(),
            points,
            point_pass,
            survivors: survivors.len(),
            native_ns: times[0],
            point_ns: times[1],
            prefix_ns: times[2],
            merged_ns,
            composite_ns,
            composite_heads,
            composite_groups,
            composite_column,
            forced: 0,
            forced_ns: 0,
            successes: 0,
            plan_ns,
            key_ns,
            composite_merge_ns,
            composite_members,
            composite_points,
            index_total_ns: times[3],
            key_cells,
            cache_store_ns,
            cache_hit_ns,
        });
        ticket
    }
    pub(super) fn activate(trace: Rc<RefCell<Self>>, ticket: usize) {
        ACTIVE.with_borrow_mut(|a| *a = Some((trace, ticket)));
    }
    pub(super) fn finish(trace: &Rc<RefCell<Self>>) {
        ACTIVE.with_borrow_mut(|active| {
            if active.as_ref().is_some_and(|(t, _)| Rc::ptr_eq(t, trace)) {
                *active = None;
            }
        });
        trace.borrow().write().expect("candidate census write");
    }
    fn write(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.out)?;
        let prefix = format!("index-{}", self.id);
        let mut file = BufWriter::new(File::create(self.out.join(format!("{prefix}-calls.tsv")))?);
        writeln!(
            file,
            "call\tphase\tcaller\tgroup\tepoch\tgoal\tkey\thit\tfull_key\tfull_hit\tchosen\tcolumn_unions\tdomain_cards\trejects\tnative\tpoint_checks\tpoint_pass\tsurvivors\tnative_ns\tpoint_ns\tprefix_ns\tmerged_ns\tcomposite_ns\tcomposite_heads\tcomposite_groups\tcomposite_column\tforced\tforced_ns\tsuccesses\tplan_ns\tkey_ns\tcomposite_merge_ns\tcomposite_members\tcomposite_points\tindex_total_ns\tkey_cells\tcache_store_ns\tcache_hit_ns"
        )?;
        let vec = |v: &[usize]| v.iter().map(usize::to_string).collect::<Vec<_>>().join(",");
        let domains = |v: &[usize]| {
            v.iter()
                .map(|n| {
                    if *n == usize::MAX {
                        "all".into()
                    } else {
                        n.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(",")
        };
        let opt = |v: Option<usize>| v.map_or_else(|| "none".into(), |v| v.to_string());
        for (i, r) in self.rows.iter().enumerate() {
            assert_eq!(
                r.forced, r.survivors,
                "every indexed survivor receives its forced check"
            );
            write!(
                file,
                "{i}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                r.phase,
                r.caller,
                opt(r.group),
                r.epoch,
                r.goal,
                r.key,
                r.hit,
                r.full,
                r.full_hit,
                opt(r.chosen),
                domains(&r.sizes),
                domains(&r.cards),
                vec(&r.rejects),
                r.native,
                r.points,
                r.point_pass,
                r.survivors,
                r.native_ns,
                r.point_ns,
                r.prefix_ns,
                r.merged_ns,
                r.composite_ns,
                r.composite_heads,
                r.composite_groups,
                opt(r.composite_column),
                r.forced,
                r.forced_ns,
                r.successes
            )?;
            writeln!(
                file,
                "\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                r.plan_ns,
                r.key_ns,
                r.composite_merge_ns,
                r.composite_members,
                r.composite_points,
                r.index_total_ns,
                r.key_cells,
                r.cache_store_ns,
                r.cache_hit_ns
            )?;
        }
        file.flush()?;
        let mut file = BufWriter::new(File::create(self.out.join(format!("{prefix}-keys.tsv")))?);
        writeln!(file, "key\tphase\tepoch\tgoal\tdomains\taliases\tnaf")?;
        let mut keys = self.keys.iter().collect::<Vec<_>>();
        keys.sort_by_key(|(_, s)| s.id);
        for (k, s) in keys {
            writeln!(
                file,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                s.id,
                k.phase,
                k.epoch,
                k.goal,
                vec(&k.domains),
                vec(&k.aliases),
                k.naf
            )?;
        }
        file.flush()?;
        let mut file = BufWriter::new(File::create(
            self.out.join(format!("{prefix}-full-keys.tsv")),
        )?);
        writeln!(
            file,
            "key\tphase\tepoch\tgoal\targument_domains\taliases\tnaf\tgoals\tvars\tparent\tdomains"
        )?;
        let mut keys = self.full.iter().collect::<Vec<_>>();
        keys.sort_by_key(|(_, s)| s.id);
        for (k, s) in keys {
            writeln!(
                file,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                s.id,
                k.key.phase,
                k.key.epoch,
                k.key.goal,
                vec(&k.key.domains),
                vec(&k.key.aliases),
                k.key.naf,
                vec(&k.goals),
                k.vars,
                vec(&k.parent),
                vec(&k.domains)
            )?;
        }
        file.flush()?;
        let mut file = BufWriter::new(File::create(self.out.join(format!("{prefix}-terms.tsv")))?);
        writeln!(file, "term\ttext")?;
        for (id, term) in &self.terms {
            writeln!(file, "{id}\t{}", term.replace(['\t', '\n', '\r'], " "))?;
        }
        file.flush()?;
        let mut file = BufWriter::new(File::create(self.out.join(format!("{prefix}-groups.tsv")))?);
        writeln!(file, "group\tpath")?;
        for (id, name) in self.group_names.iter().enumerate() {
            writeln!(file, "{id}\t{name}")?;
        }
        file.flush()?;
        fs::write(
            self.out.join(format!("{prefix}-build.tsv")),
            format!(
                "metric\tvalue\ncomposite_build_ns\t{}\ncomposite_memberships\t{}\ncomposite_pairs\t{}\n",
                self.build_ns,
                self.build_memberships,
                self.composites.len()
            ),
        )
    }
}
pub struct ForcedTimer {
    start: Option<Instant>,
    active: Option<(Rc<RefCell<Trace>>, usize)>,
    pub success: bool,
}
impl ForcedTimer {
    pub fn start() -> Self {
        let active = ACTIVE.with_borrow(Clone::clone);
        Self {
            start: active.as_ref().map(|_| Instant::now()),
            active,
            success: false,
        }
    }
}
impl Drop for ForcedTimer {
    fn drop(&mut self) {
        if let Some((trace, ticket)) = &self.active {
            let ns = self.start.unwrap().elapsed().as_nanos();
            let mut trace = trace.borrow_mut();
            let row = &mut trace.rows[*ticket];
            row.forced += 1;
            row.forced_ns += ns;
            row.successes += usize::from(self.success);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merged_postings_equal_sorted_set_union() {
        let mut rng = 0x6f25_8b1au64;
        for _ in 0..2000 {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            let mut rows = Vec::new();
            for j in 0..(rng as usize % 8) {
                let mut row = (0..(rng as usize % 30))
                    .map(|i| (i * 7 + j * 3) % 37)
                    .collect::<Vec<_>>();
                row.sort_unstable();
                row.dedup();
                rows.push(row);
            }
            let references = rows.iter().map(Vec::as_slice).collect::<Vec<_>>();
            assert_eq!(
                merged(&references),
                rows.into_iter()
                    .flatten()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
            );
        }
    }
}

#[cfg(test)]
#[path = "candidate_probe_tests.rs"]
mod key_and_composite_tests;
