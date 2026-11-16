//! Exact term/substitution/join oracle; parser/IR is the sole shared surface.
use super::{Model, Program, Rule};
use sb1_checker::program::{Clause, Goal};
use sb1_checker::term::{Arena, Node};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Term {
    Var(usize),
    Atom(String),
    Integer(String),
    Nil,
    Call(String, Vec<Term>),
}
impl Term {
    pub fn from(arena: &Arena, id: usize) -> Self {
        match arena.node(id) {
            Node::Var(v) => Self::Var(*v),
            Node::Atom(a) => Self::Atom(arena.name(*a).into()),
            Node::Integer(n) => Self::Integer(n.clone()),
            Node::Nil => Self::Nil,
            Node::Compound(f, args) => Self::Call(
                arena.name(*f).into(),
                args.iter().map(|t| Self::from(arena, *t)).collect(),
            ),
        }
    }
    pub fn list(&self) -> Option<Vec<&Term>> {
        let mut out = Vec::new();
        let mut at = self;
        loop {
            match at {
                Term::Nil => return Some(out),
                Term::Call(f, args) if f == "[|]" && args.len() == 2 => {
                    out.push(&args[0]);
                    at = &args[1];
                }
                _ => return None,
            }
        }
    }
    pub fn origin(&self) -> Option<[Term; 4]> {
        match self {
            Self::Call(f, a) if f == "$guideline_id" && a.len() == 5 => {
                Some([a[0].clone(), a[1].clone(), a[2].clone(), a[3].clone()])
            }
            _ => None,
        }
    }
    pub fn identity_depth(&self) -> usize {
        match self {
            Self::Call(f, a) if f == "$guideline_id" => 1 + a[4].identity_depth(),
            Self::Call(_, a) => a.iter().map(Self::identity_depth).max().unwrap_or(0),
            _ => 0,
        }
    }
    fn vars(&self, out: &mut BTreeSet<usize>) {
        match self {
            Self::Var(v) => {
                out.insert(*v);
            }
            Self::Call(_, a) => {
                for t in a {
                    t.vars(out);
                }
            }
            _ => {}
        }
    }
    fn values(&self, out: &mut BTreeSet<Term>) {
        if let Self::Call(_, a) = self {
            for t in a {
                out.insert(t.clone());
                t.values(out);
            }
        }
    }
    fn apply(&self, env: &[Option<Term>]) -> Option<Term> {
        Some(match self {
            Self::Var(v) => env[*v].clone()?,
            Self::Call(f, a) => Self::Call(
                f.clone(),
                a.iter().map(|t| t.apply(env)).collect::<Option<Vec<_>>>()?,
            ),
            t => t.clone(),
        })
    }
    fn matches(&self, value: &Term, env: &mut [Option<Term>]) -> bool {
        match self {
            Self::Var(v) => match &env[*v] {
                Some(old) => old == value,
                None => {
                    env[*v] = Some(value.clone());
                    true
                }
            },
            Self::Call(f, a) => {
                matches!(value,Self::Call(g,b) if f==g&&a.len()==b.len()&&a.iter().zip(b).all(|(x,y)|x.matches(y,env)))
            }
            t => t == value,
        }
    }
}
#[derive(Clone, Debug)]
pub enum Item {
    Positive(Term),
    Naf(Vec<Term>),
}
#[derive(Clone, Debug)]
pub struct Template {
    pub head: Term,
    pub items: Vec<Item>,
    pub variables: usize,
}
impl Template {
    pub fn from(arena: &Arena, c: &Clause) -> Self {
        let items = c
            .body
            .iter()
            .map(|g| match g {
                Goal::Positive(t) => Item::Positive(Term::from(arena, *t)),
                Goal::Not(inner) => Item::Naf(
                    inner
                        .iter()
                        .map(|g| {
                            if let Goal::Positive(t) = g {
                                Term::from(arena, *t)
                            } else {
                                panic!("finite oracle domain: flat relational NAF alone")
                            }
                        })
                        .collect(),
                ),
                Goal::Builtin(_) => panic!("finite oracle domain: no arithmetic built-ins"),
            })
            .collect();
        Self {
            head: Term::from(arena, c.head),
            items,
            variables: c.names.len(),
        }
    }
    fn positive(&self) -> Vec<Term> {
        self.items
            .iter()
            .filter_map(|g| {
                if let Item::Positive(t) = g {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
fn join(
    patterns: &[Term],
    env: Vec<Option<Term>>,
    facts: &BTreeSet<Term>,
) -> Vec<Vec<Option<Term>>> {
    let mut rows = vec![env];
    for pattern in patterns {
        let mut next = BTreeSet::new();
        for row in rows {
            for fact in facts {
                let mut row = row.clone();
                if pattern.matches(fact, &mut row) {
                    next.insert(row);
                }
            }
        }
        rows = next.into_iter().collect();
    }
    rows
}
pub struct Grounded {
    pub program: Program,
    pub atoms: Vec<Term>,
    pub values: BTreeSet<Term>,
}
impl Grounded {
    pub fn new(templates: &[Template]) -> Self {
        let mut envelope = BTreeSet::new();
        let mut complete = false;
        for _ in 0..32 {
            let mut added = Vec::new();
            for t in templates {
                for env in join(&t.positive(), vec![None; t.variables], &envelope) {
                    let head = t
                        .head
                        .apply(&env)
                        .expect("source safety binds every head variable");
                    assert!(
                        head.identity_depth() <= 6,
                        "finite oracle incomplete: depth guard"
                    );
                    added.push(head);
                }
            }
            let before = envelope.len();
            envelope.extend(added);
            assert!(
                envelope.len() <= 10000,
                "finite oracle incomplete: atom guard"
            );
            if before == envelope.len() {
                complete = true;
                break;
            }
        }
        assert!(complete, "finite oracle incomplete: closure round guard");
        let atoms = envelope.iter().cloned().collect::<Vec<_>>();
        let ids = atoms
            .iter()
            .enumerate()
            .map(|(i, t)| (t.clone(), i))
            .collect::<BTreeMap<_, _>>();
        let mut rules = Vec::new();
        for t in templates {
            for env in join(&t.positive(), vec![None; t.variables], &envelope) {
                let head = ids[&t.head.apply(&env).unwrap()];
                let positive = t
                    .positive()
                    .iter()
                    .map(|p| ids[&p.apply(&env).unwrap()])
                    .collect();
                let mut naf = Vec::new();
                for g in &t.items {
                    if let Item::Naf(inner) = g {
                        let witnesses = join(inner, env.clone(), &envelope)
                            .iter()
                            .map(|valuation| {
                                inner
                                    .iter()
                                    .map(|g| ids[&g.apply(valuation).unwrap()])
                                    .collect::<Model>()
                            })
                            .collect::<BTreeSet<_>>();
                        naf.push(witnesses.into_iter().collect());
                    }
                }
                rules.push(Rule {
                    head,
                    positive,
                    naf,
                    builtins: true,
                });
            }
        }
        let mut values = BTreeSet::new();
        for atom in &atoms {
            atom.values(&mut values);
        }
        Self {
            program: Program {
                atoms: atoms.len(),
                rules,
            },
            atoms,
            values,
        }
    }
    pub fn wfm(&self) -> (BTreeSet<Term>, BTreeSet<Term>) {
        let (l, u) = self.program.wfm();
        (
            l.iter().map(|i| self.atoms[*i].clone()).collect(),
            u.iter().map(|i| self.atoms[*i].clone()).collect(),
        )
    }
    pub fn answers(&self, query: &Template) -> BTreeSet<Vec<Term>> {
        let (facts, _) = self.wfm();
        let mut vars = BTreeSet::new();
        for item in &query.items {
            if let Item::Positive(t) = item {
                t.vars(&mut vars);
            }
        }
        let mut rows = vec![vec![None; query.variables]];
        for item in &query.items {
            rows = match item {
                Item::Positive(term) => rows
                    .into_iter()
                    .flat_map(|env| join(std::slice::from_ref(term), env, &facts))
                    .collect(),
                Item::Naf(inner) => rows
                    .into_iter()
                    .filter(|env| join(inner, env.clone(), &facts).is_empty())
                    .collect(),
            };
        }
        rows.into_iter()
            .map(|env| {
                vars.iter()
                    .map(|v| env[*v].clone().expect("positive projection is bound"))
                    .collect()
            })
            .collect()
    }
}
pub fn query(text: &str) -> Template {
    let mut arena = Arena::default();
    let clauses = sb1_checker::program::parse_clauses(
        &mut arena,
        &format!("guideline_exclusion(query) :- {text}.\n"),
        1,
    )
    .unwrap();
    Template::from(&arena, &clauses[0])
}
pub fn canonical_goal(text: &str) -> String {
    fn chain(xs: Vec<String>) -> String {
        let mut it = xs.into_iter().rev();
        let mut out = it.next().unwrap();
        for x in it {
            out = format!("','({x},{out})");
        }
        out
    }
    let mut arena = Arena::default();
    let clauses = sb1_checker::program::parse_clauses(
        &mut arena,
        &format!("guideline_exclusion(query) :- {text}.\n"),
        1,
    )
    .unwrap();
    let items = clauses[0]
        .body
        .iter()
        .map(|g| match g {
            Goal::Positive(t) => arena.render(*t),
            Goal::Not(gs) => format!(
                "\\+({})",
                chain(
                    gs.iter()
                        .map(|g| if let Goal::Positive(t) = g {
                            arena.render(*t)
                        } else {
                            panic!("query domain")
                        })
                        .collect()
                )
            ),
            _ => panic!("query domain"),
        })
        .collect();
    chain(items)
}
