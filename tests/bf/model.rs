// Contract bf Definition, modeled independently of the compiler and certifier.
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Term {
    Var(usize),
    Atom(String),
    Int(i64),
    Nil,
    Comp(String, Vec<Term>),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Item {
    Pos(Term),
    Naf(Vec<Term>),
}

pub const INDICATORS: [(&str, usize); 24] = [
    ("guideline_schema_version", 1),
    ("guideline_document", 4),
    ("guideline_entity", 4),
    ("guideline_cardinality", 5),
    ("guideline_event", 3),
    ("guideline_arg", 4),
    ("guideline_pp", 4),
    ("guideline_property", 4),
    ("guideline_operator", 3),
    ("guideline_interval", 6),
    ("guideline_recurrence", 4),
    ("guideline_frequency", 5),
    ("guideline_order", 4),
    ("guideline_recurrence_window", 7),
    ("guideline_range", 3),
    ("guideline_grade", 5),
    ("guideline_preference", 4),
    ("guideline_qualifier", 4),
    ("guideline_unit", 3),
    ("guideline_exclusion", 1),
    ("guideline_satisfies", 4),
    ("guideline_satisfies", 6),
    ("guideline_product", 6),
    ("guideline_quotient", 6),
];

pub fn vars(term: &Term) -> BTreeSet<usize> {
    match term {
        Term::Var(v) => BTreeSet::from([*v]),
        Term::Comp(_, args) => args.iter().flat_map(vars).collect(),
        _ => BTreeSet::new(),
    }
}

pub fn positive_vars(body: &[Item]) -> BTreeSet<usize> {
    body.iter()
        .filter_map(|item| match item {
            Item::Pos(t) => Some(vars(t)),
            Item::Naf(_) => None,
        })
        .flatten()
        .collect()
}

pub fn builtin(term: &Term) -> bool {
    matches!(term, Term::Comp(n, a) if INDICATORS[20..].contains(&(n.as_str(), a.len())))
}

pub fn required(term: &Term) -> BTreeSet<usize> {
    let Term::Comp(name, args) = term else {
        return BTreeSet::new();
    };
    let inputs: &[Term] = if builtin(term) {
        let n = match (name.as_str(), args.len()) {
            ("guideline_satisfies", 4) => 2,
            ("guideline_satisfies", 6) => 3,
            _ => 4,
        };
        &args[..n]
    } else if INDICATORS[2..20].contains(&(name.as_str(), args.len()))
        && args.len() >= 2
        && !matches!(name.as_str(), "guideline_entity" | "guideline_event")
    {
        &args[1..2]
    } else {
        &[]
    };
    inputs.iter().flat_map(vars).collect()
}

pub fn ready(item: &Item, bound: &BTreeSet<usize>, shared: &BTreeSet<usize>) -> bool {
    let needed = match item {
        Item::Pos(t) => required(t),
        Item::Naf(ts) => ts
            .iter()
            .flat_map(vars)
            .filter(|v| shared.contains(v))
            .collect(),
    };
    needed.is_subset(bound)
}

// Keep original positions: duplicates remain distinct, and removing an item cannot
// change which variables a NAF shares with the whole body.
pub fn order(body: &[Item]) -> Vec<usize> {
    let shared = positive_vars(body);
    let mut remaining: Vec<_> = (0..body.len()).collect();
    let mut bound = BTreeSet::new();
    let mut output = Vec::with_capacity(body.len());
    while !remaining.is_empty() {
        let next = remaining
            .iter()
            .position(|&i| ready(&body[i], &bound, &shared))
            .unwrap_or(0);
        let index = remaining.remove(next);
        if let Item::Pos(t) = &body[index] {
            bound.extend(vars(t));
        }
        output.push(index);
    }
    output
}

pub fn ordered(body: &[Item]) -> Vec<Item> {
    order(body).into_iter().map(|i| body[i].clone()).collect()
}
