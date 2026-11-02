use super::numbers::Rational;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Term {
    Var(usize),
    Atom(String),
    Number(Rational),
    Identity(u8, Vec<Term>),
    Compound(&'static str, Vec<Term>),
    Document,
}

pub fn atom(s: &str) -> Term {
    Term::Atom(s.to_owned())
}

pub fn number(n: i128) -> Term {
    Term::Number(Rational::new(n, 1))
}

pub fn identity(slot: u8, deps: Vec<Term>) -> Term {
    Term::Identity(slot, deps)
}

pub fn quote(s: &str) -> String {
    if s.starts_with(|c: char| c.is_ascii_lowercase())
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        s.to_owned()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}

impl Term {
    pub fn render(&self, doc: &str) -> String {
        match self {
            Self::Var(i) => ["X", "Y", "L", "K", "S", "N"][*i].to_owned(),
            Self::Atom(s) => quote(s),
            Self::Number(n) => n.render(),
            Self::Identity(slot, deps) => format!(
                "'$guideline_id'(product,{},1,ref({slot}),[{}])",
                quote(doc),
                render_terms(deps, doc)
            ),
            Self::Compound(name, args) => format!("{name}({})", render_terms(args, doc)),
            Self::Document => quote(doc),
        }
    }

    pub fn variables(&self, out: &mut BTreeSet<usize>) {
        match self {
            Self::Var(i) => {
                out.insert(*i);
            }
            Self::Identity(_, args) | Self::Compound(_, args) => {
                for arg in args {
                    arg.variables(out);
                }
            }
            _ => {}
        }
    }

    pub fn substitute(&self, env: &Binding) -> Option<Self> {
        match self {
            Self::Var(i) => env[*i].clone(),
            Self::Identity(slot, args) => Some(identity(
                *slot,
                args.iter()
                    .map(|t| t.substitute(env))
                    .collect::<Option<Vec<_>>>()?,
            )),
            Self::Compound(name, args) => Some(Self::Compound(
                name,
                args.iter()
                    .map(|t| t.substitute(env))
                    .collect::<Option<Vec<_>>>()?,
            )),
            _ => Some(self.clone()),
        }
    }

    pub fn match_ground(&self, value: &Self, env: &mut Binding) -> bool {
        match self {
            Self::Var(i) => match &env[*i] {
                Some(old) => old == value,
                None => {
                    env[*i] = Some(value.clone());
                    true
                }
            },
            Self::Identity(slot, args) => match value {
                Self::Identity(other, values) if slot == other && args.len() == values.len() => {
                    args.iter().zip(values).all(|(a, b)| a.match_ground(b, env))
                }
                _ => false,
            },
            _ => self == value,
        }
    }

    pub fn transform(&self) -> Self {
        match self {
            Self::Var(i) => Self::Var([1, 0, 3, 2, 5, 4][*i]),
            Self::Atom(s) if s == "a" => atom("b"),
            Self::Atom(s) if s == "b" => atom("a"),
            Self::Identity(slot, args) => {
                identity(*slot, args.iter().map(Self::transform).collect())
            }
            Self::Compound(name, args) => {
                Self::Compound(name, args.iter().map(Self::transform).collect())
            }
            _ => self.clone(),
        }
    }

    pub fn collect_values(&self, out: &mut BTreeSet<Self>) {
        if self.substitute(&empty_binding()).is_some() {
            out.insert(self.clone());
        }
        match self {
            Self::Number(n) if n.denominator != 1 => {
                out.insert(number(n.numerator));
                out.insert(number(n.denominator));
            }
            Self::Identity(slot, args) => {
                out.insert(atom("product"));
                out.insert(Self::Document);
                out.insert(number(1));
                Self::Compound("ref", vec![number(i128::from(*slot))]).collect_values(out);
                for arg in args {
                    arg.collect_values(out);
                }
            }
            Self::Compound(_, args) => {
                for arg in args {
                    arg.collect_values(out);
                }
            }
            _ => {}
        }
    }

    pub fn identity_depth(&self) -> usize {
        match self {
            Self::Identity(_, deps) if !deps.is_empty() => {
                1 + deps.iter().map(Self::identity_depth).max().unwrap_or(0)
            }
            _ => 0,
        }
    }
}

fn render_terms(args: &[Term], doc: &str) -> String {
    args.iter()
        .map(|t| t.render(doc))
        .collect::<Vec<_>>()
        .join(",")
}

pub type Binding = [Option<Term>; 6];

pub fn empty_binding() -> Binding {
    [None, None, None, None, None, None]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Predicate {
    D,
    P,
    Q,
    R,
    E,
    F,
    G,
    H,
    V,
    Version,
    Document,
    Supersession,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Literal {
    pub predicate: Predicate,
    pub args: Vec<Term>,
}

pub fn literal(predicate: Predicate, args: Vec<Term>) -> Literal {
    Literal { predicate, args }
}

impl Literal {
    pub fn render(&self, doc: &str) -> String {
        use Predicate::*;
        let args = render_terms(&self.args, doc);
        match self.predicate {
            D | P | Q | F | G => {
                let label = match self.predicate {
                    D => "domain",
                    P => "p",
                    Q => "q",
                    F => "f",
                    G => "g",
                    _ => unreachable!(),
                };
                format!("guideline_entity(actual,{args},{label},countable)")
            }
            R => format!("guideline_operator(actual,{args},should)"),
            E | H => format!(
                "guideline_preference(actual,{},{args})",
                if self.predicate == E { "e" } else { "h" }
            ),
            V => format!("guideline_cardinality(actual,result,na,{args})"),
            Version => format!("guideline_schema_version({args})"),
            Document => format!("guideline_document({args})"),
            Supersession => format!("guideline_supersession({args})"),
        }
    }

    pub fn substitute(&self, env: &Binding) -> Option<Self> {
        Some(literal(
            self.predicate,
            self.args
                .iter()
                .map(|t| t.substitute(env))
                .collect::<Option<Vec<_>>>()?,
        ))
    }

    pub fn match_ground(&self, other: &Self, env: &mut Binding) -> bool {
        self.predicate == other.predicate
            && self.args.len() == other.args.len()
            && self
                .args
                .iter()
                .zip(&other.args)
                .all(|(a, b)| a.match_ground(b, env))
    }

    pub fn variables(&self, out: &mut BTreeSet<usize>) {
        for term in &self.args {
            term.variables(out);
        }
    }

    pub fn transform(&self) -> Self {
        literal(
            match self.predicate {
                Predicate::P => Predicate::Q,
                Predicate::Q => Predicate::P,
                p => p,
            },
            self.args.iter().map(Term::transform).collect(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BuiltinKind {
    Satisfies4,
    Satisfies6,
    Product,
    Quotient,
}

impl BuiltinKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Satisfies4 | Self::Satisfies6 => "guideline_satisfies",
            Self::Product => "guideline_product",
            Self::Quotient => "guideline_quotient",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Goal {
    Positive(Literal),
    Builtin(BuiltinKind, Vec<Term>),
    Negative(Vec<Goal>),
}

impl Goal {
    pub fn render(&self, doc: &str) -> String {
        match self {
            Self::Positive(l) => l.render(doc),
            Self::Builtin(kind, args) => {
                format!("{}({})", kind.name(), render_terms(args, doc))
            }
            Self::Negative(goals) => format!("\\+ ({})", render_goals(goals, doc)),
        }
    }

    pub fn variables(&self, out: &mut BTreeSet<usize>) {
        match self {
            Self::Positive(l) => l.variables(out),
            Self::Builtin(_, args) => {
                for term in args {
                    term.variables(out);
                }
            }
            Self::Negative(goals) => {
                for goal in goals {
                    goal.variables(out);
                }
            }
        }
    }

    pub fn transform(&self) -> Self {
        match self {
            Self::Positive(l) => Self::Positive(l.transform()),
            Self::Builtin(kind, args) => {
                Self::Builtin(*kind, args.iter().map(Term::transform).collect())
            }
            Self::Negative(goals) => Self::Negative(goals.iter().map(Self::transform).collect()),
        }
    }
}

pub fn render_goals(goals: &[Goal], doc: &str) -> String {
    goals
        .iter()
        .map(|g| g.render(doc))
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clause {
    pub head: Literal,
    pub body: Vec<Goal>,
}

pub fn clause(head: Literal, body: Vec<Goal>) -> Clause {
    Clause { head, body }
}

impl Clause {
    pub fn render(&self, doc: &str) -> String {
        if self.body.is_empty() {
            self.head.render(doc)
        } else {
            format!(
                "({} :- {})",
                self.head.render(doc),
                render_goals(&self.body, doc)
            )
        }
    }

    pub fn transform(&self) -> Self {
        clause(
            self.head.transform(),
            self.body.iter().map(Goal::transform).collect(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    pub family: &'static str,
    pub index: usize,
    pub clauses: Vec<Clause>,
    pub refused: bool,
    pub expected_undefined: BTreeSet<Literal>,
}

impl Program {
    pub fn new(family: &'static str, index: usize, carrier: &[Term]) -> Self {
        use Predicate::*;
        Self {
            family,
            index,
            clauses: vec![
                clause(literal(Version, vec![number(2)]), vec![]),
                clause(
                    literal(
                        Document,
                        vec![
                            Term::Document,
                            Term::Compound("ace_sha256", vec![atom(&"0".repeat(64))]),
                            Term::Compound("ulex", vec![atom("none")]),
                            Term::Compound("vocabulary", vec![atom("none")]),
                        ],
                    ),
                    vec![],
                ),
                clause(
                    literal(
                        Supersession,
                        vec![
                            Term::Document,
                            number(1),
                            Term::Compound(
                                "source",
                                vec![atom("oracle"), atom("https://example.invalid/sb-oracle")],
                            ),
                            number(20_220_101),
                        ],
                    ),
                    vec![],
                ),
                clause(literal(D, vec![carrier[0].clone()]), vec![]),
                clause(literal(D, vec![carrier[1].clone()]), vec![]),
            ],
            refused: false,
            expected_undefined: BTreeSet::new(),
        }
    }

    pub fn id(&self) -> String {
        format!(
            "sb_oracle_{}_{}",
            self.family.to_ascii_lowercase().replace('-', "_"),
            self.index
        )
    }

    pub fn render(&self) -> String {
        format!(
            "case({},{},[{}]).\n",
            self.id(),
            if self.refused { "refused" } else { "accepted" },
            self.clauses
                .iter()
                .map(|c| c.render(&self.id()))
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    pub fn transform(&self) -> Self {
        let mut p = self.clone();
        p.clauses = self.clauses.iter().rev().map(Clause::transform).collect();
        p.expected_undefined = self
            .expected_undefined
            .iter()
            .map(Literal::transform)
            .collect();
        p
    }

    pub fn source_values(&self) -> BTreeSet<Term> {
        fn collect_goal(goal: &Goal, out: &mut BTreeSet<Term>) {
            match goal {
                Goal::Positive(l) => {
                    for t in &l.args {
                        t.collect_values(out);
                    }
                }
                Goal::Builtin(_, args) => {
                    for t in args {
                        t.collect_values(out);
                    }
                }
                Goal::Negative(goals) => {
                    for g in goals {
                        collect_goal(g, out);
                    }
                }
            }
        }
        let mut out = BTreeSet::new();
        for c in &self.clauses {
            for t in &c.head.args {
                t.collect_values(&mut out);
            }
            for g in &c.body {
                collect_goal(g, &mut out);
            }
        }
        for reserved in [
            "actual",
            "domain",
            "countable",
            "p",
            "q",
            "e",
            "f",
            "g",
            "h",
            "should",
            "result",
            "na",
            "product",
        ] {
            out.insert(atom(reserved));
        }
        out
    }

    pub fn assert_source_bounds(&self) {
        assert!(self.clauses.len() <= 15, "{}: source clauses", self.id());
        assert!(self.clauses.iter().filter(|c| !c.body.is_empty()).count() <= 3);
        for c in &self.clauses {
            assert!(c.body.len() <= 4);
            let mut variables = BTreeSet::new();
            c.head.variables(&mut variables);
            let mut negatives = 0;
            for goal in &c.body {
                goal.variables(&mut variables);
                if let Goal::Negative(gs) = goal {
                    negatives += 1;
                    assert!(gs.len() <= 3);
                    assert!(!gs.iter().any(|g| matches!(g, Goal::Negative(_))));
                }
            }
            assert!(variables.len() <= 4);
            assert!(negatives <= 2);
        }
    }
}

pub fn rendered_model(model: &BTreeSet<Literal>, doc: &str) -> BTreeSet<String> {
    model.iter().map(|l| l.render(doc)).collect()
}

pub fn family_counts() -> BTreeMap<&'static str, usize> {
    BTreeMap::from([
        ("G", 10_847),
        ("V", 20_224),
        ("C4", 3_920),
        ("C6", 17_640),
        ("A-free", 12_544),
        ("A-bound", 9_072),
        ("A-partial", 2_430),
        ("I", 16_640),
        ("B-shape", 44),
        ("NC", 8),
    ])
}
