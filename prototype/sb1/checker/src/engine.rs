use crate::level::Level;
use crate::number::{arithmetic, satisfies};
use crate::program::{Clause, Goal};
use crate::term::{Arena, Env, Id, Symbol};
use crate::{Counters, Failure};
use std::collections::{BTreeMap, BTreeSet, HashMap};

type Predicate = (Symbol, usize);
type Pattern = (Predicate, u16);
type Buckets = HashMap<Vec<Id>, Vec<usize>>;

pub struct Model {
    pub facts: Vec<Id>,
    pub positions: HashMap<Id, usize>,
    pub levels: Vec<Level>,
    predicates: HashMap<Predicate, Vec<usize>>,
    patterns: HashMap<Pattern, Buckets>,
}

pub struct Claim {
    pub clause: usize,
    pub fact: usize,
    pub env: Env,
    pub level: Level,
}

fn mask(arena: &Arena, args: &[Id], bound: &BTreeSet<usize>) -> u16 {
    args.iter().enumerate().fold(0, |mask, (i, arg)| {
        let mut vars = BTreeSet::new();
        arena.vars(*arg, &mut vars);
        mask | if vars.is_subset(bound) { 1 << i } else { 0 }
    })
}

fn patterns(
    arena: &Arena,
    goals: &[Goal],
    bound: &mut BTreeSet<usize>,
    out: &mut BTreeSet<Pattern>,
) {
    for goal in goals {
        match goal {
            Goal::Positive(term) => {
                if let Some((name, args)) = arena.call(*term) {
                    let mask = mask(arena, args, bound);
                    if mask != (1 << args.len()) - 1 {
                        out.insert(((name, args.len()), mask));
                    }
                    bound.extend(goal.vars(arena));
                }
            }
            Goal::Builtin(term) => {
                arena.vars(*term, bound);
            }
            Goal::Not(inner) => {
                patterns(arena, inner, &mut bound.clone(), out);
            }
        }
    }
}

impl Model {
    pub fn new(arena: &Arena, facts: Vec<Id>, levels: Vec<Level>) -> Result<Self, Failure> {
        let mut positions = HashMap::new();
        let mut predicates: HashMap<Predicate, Vec<usize>> = HashMap::new();
        for (i, &term) in facts.iter().enumerate() {
            if positions.insert(term, i).is_some() {
                return Err(Failure::refused(
                    "claim",
                    format!("duplicate model fact {}", arena.render(term)),
                ));
            }
            let Some((name, args)) = arena.call(term) else {
                return Err(Failure::parse("model predicate expected"));
            };
            predicates.entry((name, args.len())).or_default().push(i);
        }
        Ok(Self {
            facts,
            positions,
            levels,
            predicates,
            patterns: HashMap::new(),
        })
    }

    pub fn build_indices(
        &mut self,
        arena: &Arena,
        clauses: &[Clause],
        query: &[Goal],
        counters: &mut Counters,
    ) {
        let mut needed = BTreeSet::new();
        for c in clauses {
            patterns(arena, &c.body, &mut BTreeSet::new(), &mut needed);
            patterns(
                arena,
                &c.body,
                &mut c.nonlocals.iter().copied().collect(),
                &mut needed,
            );
        }
        patterns(arena, query, &mut BTreeSet::new(), &mut needed);
        for (pred, mask) in needed {
            let mut index: Buckets = HashMap::new();
            for &row in self.predicates.get(&pred).into_iter().flatten() {
                let (_, args) = arena.call(self.facts[row]).unwrap();
                let key = args
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask & (1 << i) != 0)
                    .map(|(_, a)| *a)
                    .collect();
                index.entry(key).or_default().push(row);
                counters.runtime("index_insertions", 1);
            }
            counters.runtime("index_builds", 1);
            self.patterns.insert((pred, mask), index);
        }
        counters.logical("predicate_indices", self.predicates.len() as u64);
        counters.logical("bound_pattern_indices", self.patterns.len() as u64);
    }

    fn candidates<'a>(
        &'a self,
        arena: &mut Arena,
        term: Id,
        env: &Env,
        counters: &mut Counters,
    ) -> Result<Rows<'a>, Failure> {
        counters.runtime("index_probes", 1);
        if arena.bound(term, env) {
            let term = arena.instantiate(term, env).expect("bound term");
            return Ok(Rows::One(self.positions.get(&term).copied()));
        }
        let (name, args) = arena.call(term).expect("admitted predicate");
        let args = args.to_vec();
        let mask = args.iter().enumerate().fold(0, |m, (i, a)| {
            m | if arena.bound(*a, env) { 1 << i } else { 0 }
        });
        let key = args
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, a)| arena.instantiate(*a, env).expect("bound key"))
            .collect::<Vec<_>>();
        let index = self
            .patterns
            .get(&((name, args.len()), mask))
            .ok_or_else(|| {
                Failure::parse(format!(
                    "internal: unplanned index {}/{} mask {mask}",
                    arena.name(name),
                    args.len()
                ))
            })?;
        Ok(Rows::Slice(
            index.get(&key).map(Vec::as_slice).unwrap_or(&[]),
        ))
    }
}

enum Rows<'a> {
    Slice(&'a [usize]),
    One(Option<usize>),
}
impl<'a> IntoIterator for Rows<'a> {
    type Item = usize;
    type IntoIter = RowIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        RowIter { rows: self, at: 0 }
    }
}
struct RowIter<'a> {
    rows: Rows<'a>,
    at: usize,
}
impl Iterator for RowIter<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let item = match &self.rows {
            Rows::Slice(rows) => rows.get(self.at).copied(),
            Rows::One(row) => {
                if self.at == 0 {
                    *row
                } else {
                    None
                }
            }
        };
        self.at += 1;
        item
    }
}

fn builtin(arena: &mut Arena, term: Id, env: &mut Env, counters: &mut Counters) -> bool {
    counters.runtime("builtin_evaluations", 1);
    let (name, args) = arena.call(term).unwrap();
    let name = arena.name(name).to_string();
    let args = args.to_vec();
    let count = if name == "guideline_satisfies" {
        args.len()
    } else {
        4
    };
    let Some(inputs) = args[..count]
        .iter()
        .map(|arg| arena.instantiate(*arg, env))
        .collect::<Option<Vec<_>>>()
    else {
        return false;
    };
    if name == "guideline_satisfies" {
        return satisfies(arena, &inputs);
    }
    let Some((cmp, n)) = arithmetic(arena, &name, &inputs) else {
        return false;
    };
    arena.match_ground(args[4], cmp, env) && arena.match_ground(args[5], n, env)
}

pub struct Evaluation<'a> {
    pub arena: &'a mut Arena,
    pub model: &'a Model,
    pub counters: &'a mut Counters,
}

impl Evaluation<'_> {
    pub fn walk(
        &mut self,
        goals: &[Goal],
        at: usize,
        env: &Env,
        visit: &mut dyn FnMut(&mut Arena, &Env, &mut Counters) -> Result<bool, Failure>,
    ) -> Result<bool, Failure> {
        if at == goals.len() {
            return visit(self.arena, env, self.counters);
        }
        match &goals[at] {
            Goal::Positive(term) => {
                let rows = self
                    .model
                    .candidates(self.arena, *term, env, self.counters)?;
                for row in rows {
                    self.counters.runtime("candidate_joins", 1);
                    self.counters.runtime("term_matches", 1);
                    let mut next = env.clone();
                    if self
                        .arena
                        .match_ground(*term, self.model.facts[row], &mut next)
                    {
                        self.counters.runtime("successful_joins", 1);
                        if self.walk(goals, at + 1, &next, visit)? {
                            return Ok(true);
                        }
                    } else {
                        self.counters.runtime("failed_joins", 1);
                    }
                }
                Ok(false)
            }
            Goal::Builtin(term) => {
                let mut next = env.clone();
                if builtin(self.arena, *term, &mut next, self.counters) {
                    self.walk(goals, at + 1, &next, visit)
                } else {
                    Ok(false)
                }
            }
            Goal::Not(inner) => {
                self.counters.runtime("naf_checks", 1);
                if self.exists(inner, env)? {
                    Ok(false)
                } else {
                    self.walk(goals, at + 1, env, visit)
                }
            }
        }
    }

    fn exists(&mut self, goals: &[Goal], env: &Env) -> Result<bool, Failure> {
        self.counters.runtime("naf_scans", 1);
        self.walk(goals, 0, env, &mut |_, _, _| Ok(true))
    }
}

pub fn support(
    arena: &mut Arena,
    model: &Model,
    clauses: &[Clause],
    claims: &[Claim],
    counters: &mut Counters,
) -> Result<(), Failure> {
    for claim in claims {
        let c = &clauses[claim.clause];
        let detail = || format!("{}:{} fact {}", c.document, c.ordinal, claim.fact + 1);
        let mut env = claim.env.clone();
        if arena.instantiate(c.head, &env) != Some(model.facts[claim.fact]) {
            return Err(Failure::refused(
                "claim",
                format!("{} head/valuation", detail()),
            ));
        }
        if c.body.is_empty() {
            if !claim.level.is_zero() {
                return Err(Failure::refused(
                    "claim",
                    format!("{} fact level is not 0", detail()),
                ));
            }
            continue;
        }
        if claim.level.is_zero() {
            return Err(Failure::refused(
                "claim",
                format!("{} rule level is 0", detail()),
            ));
        }
        for g in &c.body {
            match g {
                Goal::Positive(term) => {
                    counters.logical("support_incidences", 1);
                    let Some(ground) = arena.instantiate(*term, &env) else {
                        return Err(Failure::refused(
                            "claim",
                            format!("{} unground body", detail()),
                        ));
                    };
                    let Some(&row) = model.positions.get(&ground) else {
                        return Err(Failure::refused(
                            "claim",
                            format!("{} absent positive body", detail()),
                        ));
                    };
                    if model.levels[row] >= claim.level {
                        return Err(Failure::refused(
                            "claim",
                            format!("{} nondecreasing support level", detail()),
                        ));
                    }
                }
                Goal::Builtin(term) => {
                    counters.logical("support_builtins", 1);
                    if !builtin(arena, *term, &mut env, counters) {
                        return Err(Failure::refused("claim", format!("{} built-in", detail())));
                    }
                }
                Goal::Not(inner) => {
                    counters.logical("support_negations", 1);
                    let mut evaluation = Evaluation {
                        arena,
                        model,
                        counters,
                    };
                    if evaluation.exists(inner, &env)? {
                        return Err(Failure::refused(
                            "claim",
                            format!("{} negation witness", detail()),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn closure(
    arena: &mut Arena,
    model: &Model,
    clauses: &[Clause],
    counters: &mut Counters,
) -> Result<(), Failure> {
    for c in clauses {
        let env = vec![None; c.names.len()];
        let mut evaluation = Evaluation {
            arena,
            model,
            counters,
        };
        evaluation.walk(&c.body, 0, &env, &mut |arena, env, counters| {
            counters.logical("enabled_instances", 1);
            let Some(head) = arena.instantiate(c.head, env) else {
                return Err(Failure::parse("internal: admitted head is unground"));
            };
            if model.positions.contains_key(&head) {
                Ok(false)
            } else {
                Err(Failure::refused(
                    "closure",
                    format!(
                        "{}:{} omitted {}",
                        c.document,
                        c.ordinal,
                        arena.render(head)
                    ),
                ))
            }
        })?;
    }
    Ok(())
}

pub fn q_all(arena: &mut Arena, model: &Model, counters: &mut Counters) -> String {
    let mut boxes = Vec::new();
    let mut payloads: BTreeMap<Id, Vec<Id>> = BTreeMap::new();
    for &fact in &model.facts {
        if let Some((_, args)) = arena.call(fact)
            && let Some(&context) = args.first()
        {
            payloads.entry(context).or_default().push(fact);
        }
        if let Some(args) = arena.named(fact, "guideline_operator", 3)
            && arena.atom_name(args[0]) == Some("actual")
            && matches!(arena.atom_name(args[2]), Some("should" | "must" | "may"))
        {
            boxes.push((args[1], args[2]));
        }
    }
    boxes.sort_by(|(a, x), (b, y)| {
        counters.runtime("comparisons", 1);
        arena.compare(*a, *b).then_with(|| arena.compare(*x, *y))
    });
    boxes.dedup();
    let mut out = String::new();
    for (context, operator) in boxes {
        let mut payload = payloads.get(&context).cloned().unwrap_or_default();
        payload.sort_by(|a, b| {
            counters.runtime("comparisons", 1);
            arena.compare(*a, *b)
        });
        payload.dedup();
        let payload = arena.make_list(&payload);
        let row = arena.compound("box", vec![context, operator, payload]);
        out.push_str(&arena.render(row));
        out.push_str(".\n");
        counters.logical("answer_rows", 1);
    }
    out
}
