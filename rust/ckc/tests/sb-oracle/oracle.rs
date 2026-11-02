use super::model::{Binding, Goal, Literal, Program, Term, empty_binding};
use super::numbers;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default)]
pub struct Statistics {
    pub values: usize,
    pub schema_heads: usize,
    pub ground_clauses: usize,
    pub identity_depth: usize,
    pub rounds: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Outcome {
    pub truth: BTreeSet<Literal>,
    pub undefined: BTreeSet<Literal>,
    pub statistics: Statistics,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct GroundClause {
    head: usize,
    positive: Vec<usize>,
    negative: Vec<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Witness {
    clause: usize,
    group: usize,
    outer: Vec<Term>,
}

fn bindings(
    goals: &[Goal],
    initial: &Binding,
    potential: &BTreeSet<Literal>,
    values: &mut BTreeSet<Term>,
) -> Vec<(Binding, Vec<Literal>)> {
    let mut rows = vec![(initial.clone(), Vec::new())];
    for goal in goals {
        let mut next = Vec::new();
        match goal {
            Goal::Positive(pattern) => {
                for (env, support) in rows {
                    for fact in potential {
                        let mut candidate = env.clone();
                        if pattern.match_ground(fact, &mut candidate) {
                            let mut support = support.clone();
                            support.push(fact.clone());
                            next.push((candidate, support));
                        }
                    }
                }
            }
            Goal::Builtin(kind, args) => {
                for (env, support) in rows {
                    if let Some(candidate) = numbers::apply(*kind, args, &env) {
                        for t in candidate.iter().flatten() {
                            t.collect_values(values);
                        }
                        next.push((candidate, support));
                    }
                }
            }
            Goal::Negative(_) => next = rows,
        }
        rows = next;
    }
    rows
}

fn source_ground_literals(goals: &[Goal], out: &mut BTreeSet<Literal>) {
    for goal in goals {
        match goal {
            Goal::Positive(l) => {
                if let Some(ground) = l.substitute(&empty_binding()) {
                    out.insert(ground);
                }
            }
            Goal::Negative(gs) => source_ground_literals(gs, out),
            Goal::Builtin(_, _) => {}
        }
    }
}

fn head_universe(program: &Program, values: &mut BTreeSet<Term>) -> BTreeSet<Literal> {
    let mut potential = BTreeSet::new();
    for c in &program.clauses {
        if let Some(ground) = c.head.substitute(&empty_binding()) {
            potential.insert(ground);
        }
        source_ground_literals(&c.body, &mut potential);
    }
    // Negation-free may closure grounds independently of the claimed/true model.
    loop {
        let before = potential.len();
        for c in &program.clauses {
            for (env, _) in bindings(&c.body, &empty_binding(), &potential, values) {
                let head = c.head.substitute(&env).expect("safe head");
                for t in &head.args {
                    t.collect_values(values);
                }
                potential.insert(head);
                assert!(potential.len() <= 512, "{}: head bound", program.id());
                assert!(values.len() <= 256, "{}: value bound", program.id());
            }
        }
        if before == potential.len() {
            return potential;
        }
    }
}

fn normalize(mut c: GroundClause) -> GroundClause {
    c.positive.sort_unstable();
    c.positive.dedup();
    c.negative.sort_unstable();
    c.negative.dedup();
    c
}

fn ground(
    program: &Program,
    potential: &BTreeSet<Literal>,
    values: &mut BTreeSet<Term>,
) -> (Vec<GroundClause>, usize) {
    let schema = potential
        .iter()
        .enumerate()
        .map(|(i, l)| (l, i))
        .collect::<BTreeMap<_, _>>();
    let mut witnesses = BTreeMap::new();
    let mut rules = BTreeSet::new();
    for (clause_index, c) in program.clauses.iter().enumerate() {
        for (env, positive) in bindings(&c.body, &empty_binding(), potential, values) {
            let mut negative = Vec::new();
            for (group_index, goal) in c.body.iter().enumerate() {
                let Goal::Negative(group) = goal else {
                    continue;
                };
                let mut inside = BTreeSet::new();
                goal.variables(&mut inside);
                let mut outside = BTreeSet::new();
                c.head.variables(&mut outside);
                for (other_index, other) in c.body.iter().enumerate() {
                    if other_index != group_index {
                        other.variables(&mut outside);
                    }
                }
                let key = Witness {
                    clause: clause_index,
                    group: group_index,
                    outer: inside
                        .intersection(&outside)
                        .map(|i| env[*i].clone().expect("safe NAF outer"))
                        .collect(),
                };
                let witness_id = if let Some(id) = witnesses.get(&key) {
                    *id
                } else {
                    let id = potential.len() + witnesses.len();
                    witnesses.insert(key, id);
                    for (_, support) in bindings(group, &env, potential, values) {
                        rules.insert(normalize(GroundClause {
                            head: id,
                            positive: support.iter().map(|l| schema[l]).collect(),
                            negative: vec![],
                        }));
                    }
                    id
                };
                negative.push(witness_id);
            }
            let head = c.head.substitute(&env).expect("safe grounded head");
            rules.insert(normalize(GroundClause {
                head: schema[&head],
                positive: positive.iter().map(|l| schema[l]).collect(),
                negative,
            }));
        }
    }
    assert!(
        rules.len() <= 4_096,
        "{}: ground-clause bound",
        program.id()
    );
    (
        rules.into_iter().collect(),
        potential.len() + witnesses.len(),
    )
}

fn gamma(interpretation: &BTreeSet<usize>, clauses: &[GroundClause]) -> BTreeSet<usize> {
    let reduct = clauses
        .iter()
        .filter(|c| c.negative.iter().all(|a| !interpretation.contains(a)))
        .collect::<Vec<_>>();
    let mut least = BTreeSet::new();
    loop {
        let before = least.len();
        for c in &reduct {
            if c.positive.iter().all(|a| least.contains(a)) {
                least.insert(c.head);
            }
        }
        if before == least.len() {
            return least;
        }
    }
}

pub fn evaluate(program: &Program) -> Outcome {
    program.assert_source_bounds();
    let mut values = program.source_values();
    let potential = head_universe(program, &mut values);
    let (clauses, atoms) = ground(program, &potential, &mut values);
    assert!(values.len() <= 256, "{}: final value bound", program.id());
    for term in &values {
        if let Term::Identity(_, deps) = term {
            assert!(deps.len() <= 2);
        }
        if let Term::Number(n) = term {
            assert!(n.numerator.abs() < 1_i128 << 32);
            assert!(n.denominator < 1_i128 << 32);
        }
    }
    let mut lower = BTreeSet::new();
    let mut upper = (0..atoms).collect::<BTreeSet<_>>();
    let mut rounds = 0;
    loop {
        let next_lower = gamma(&upper, &clauses);
        let next_upper = gamma(&next_lower, &clauses);
        assert!(lower.is_subset(&next_lower));
        assert!(next_upper.is_subset(&upper));
        assert!(next_lower.is_subset(&next_upper));
        rounds += 1;
        if next_lower == lower && next_upper == upper {
            break;
        }
        (lower, upper) = (next_lower, next_upper);
    }
    assert_eq!(gamma(&upper, &clauses), lower);
    assert_eq!(gamma(&lower, &clauses), upper);
    if lower == upper {
        assert_eq!(gamma(&lower, &clauses), lower);
    }
    let schema = potential.iter().cloned().collect::<Vec<_>>();
    let project =
        |set: &BTreeSet<usize>| set.iter().filter_map(|i| schema.get(*i).cloned()).collect();
    let identity_depth = values.iter().map(Term::identity_depth).max().unwrap_or(0);
    assert!(identity_depth <= 2);
    Outcome {
        truth: project(&lower),
        undefined: project(&upper.difference(&lower).copied().collect()),
        statistics: Statistics {
            values: values.len(),
            schema_heads: schema.len(),
            ground_clauses: clauses.len(),
            identity_depth,
            rounds,
        },
    }
}
