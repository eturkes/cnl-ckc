use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Q(pub i128, pub i128);
impl Q {
    pub fn new(n: i128, d: i128) -> Self {
        assert_ne!(d, 0);
        let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
        let (mut a, mut b) = (n.abs(), d);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Self(n / a, d / a)
    }
    pub fn int(n: i128) -> Self {
        Self(n, 1)
    }
    pub fn add(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
    }
    pub fn mul(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.0, self.1 * rhs.1)
    }
    pub fn div(self, rhs: Self) -> Option<Self> {
        (rhs.0 != 0).then(|| Self::new(self.0 * rhs.1, self.1 * rhs.0))
    }
}
impl Ord for Q {
    fn cmp(&self, rhs: &Self) -> Ordering {
        (self.0 * rhs.1).cmp(&(rhs.0 * self.1))
    }
}
impl PartialOrd for Q {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl fmt::Display for Q {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1 == 1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "rational({},{})", self.0, self.1)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Cmp {
    Some,
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
}
impl Cmp {
    pub fn word(self) -> &'static str {
        match self {
            Self::Some => "some",
            Self::Eq => "exactly",
            Self::Ge => "at least",
            Self::Gt => "more than",
            Self::Le => "at most",
            Self::Lt => "less than",
        }
    }
    pub fn atom(self) -> &'static str {
        match self {
            Self::Some => "some",
            Self::Eq => "exactly",
            Self::Ge => "geq",
            Self::Gt => "greater",
            Self::Le => "leq",
            Self::Lt => "less",
        }
    }
    pub fn admits(self, x: Q, b: Q) -> bool {
        match self {
            Self::Some => true,
            Self::Eq => x == b,
            Self::Ge => x >= b,
            Self::Gt => x > b,
            Self::Le => x <= b,
            Self::Lt => x < b,
        }
    }
    pub fn default(self, b: Q) -> Q {
        let one = Q::int(1);
        let zero = Q::int(0);
        match self {
            Self::Some => one,
            Self::Eq => b,
            Self::Ge => {
                if b == zero {
                    one
                } else {
                    b
                }
            }
            Self::Le => {
                if b == zero {
                    Q::int(-1)
                } else {
                    b
                }
            }
            Self::Gt => {
                let x = b.add(one);
                if x == zero { x.add(one) } else { x }
            }
            Self::Lt => {
                let x = b.add(Q::int(-1));
                if x == zero { x.add(Q::int(-1)) } else { x }
            }
        }
    }
}

pub fn candidates(cmp: Cmp, bound: Q, relevant: &[Q]) -> Vec<Q> {
    let mut b = BTreeSet::from([Q::int(0)]);
    if !matches!(cmp, Cmp::Some) {
        b.insert(bound);
    }
    b.extend(relevant);
    let b: Vec<_> = b.into_iter().collect();
    let mut cells = BTreeSet::new();
    for &x in &b {
        cells.extend([x, x.add(Q::int(-1)), x.add(Q::int(1))]);
    }
    for pair in b.windows(2) {
        cells.insert(pair[0].add(pair[1]).div(Q::int(2)).unwrap());
    }
    let w0 = cmp.default(bound);
    let mut out = vec![w0];
    out.extend(
        cells
            .into_iter()
            .filter(|&x| x != w0 && cmp.admits(x, bound)),
    );
    assert!(out.iter().all(|&x| cmp.admits(x, bound)));
    out
}

#[derive(Clone, Debug)]
pub enum Expr {
    Var(usize),
    Lit(Q),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
}
impl Expr {
    pub fn mul(a: Self, b: Self) -> Self {
        Self::Mul(Box::new(a), Box::new(b))
    }
    pub fn div(a: Self, b: Self) -> Self {
        Self::Div(Box::new(a), Box::new(b))
    }
    pub fn eval(&self, tuple: &[Q]) -> Option<Q> {
        match self {
            Self::Var(i) => Some(tuple[*i]),
            Self::Lit(x) => Some(*x),
            Self::Mul(a, b) => Some(a.eval(tuple)?.mul(b.eval(tuple)?)),
            Self::Div(a, b) => a.eval(tuple)?.div(b.eval(tuple)?),
        }
    }
}

// Deliberately materialized reference product: independent of a lazy index-vector walk.
pub fn product(lists: &[Vec<Q>]) -> Vec<Vec<Q>> {
    lists.iter().fold(vec![vec![]], |rows, next| {
        rows.into_iter()
            .flat_map(|row| {
                next.iter().map(move |&x| {
                    let mut out = row.clone();
                    out.push(x);
                    out
                })
            })
            .collect()
    })
}
pub fn first(lists: &[Vec<Q>], win: impl Fn(&[Q]) -> bool) -> Option<Vec<usize>> {
    product(lists).into_iter().find(|t| win(t)).map(|tuple| {
        tuple
            .iter()
            .zip(lists)
            .map(|(x, list)| list.iter().position(|y| y == x).unwrap())
            .collect()
    })
}
pub fn prefix(lists: &[Vec<Q>], winner: Option<&[usize]>) -> usize {
    let all = product(lists);
    match winner {
        None => all.len(),
        Some(digits) => {
            let tuple: Vec<_> = lists.iter().zip(digits).map(|(l, &d)| l[d]).collect();
            all.iter().position(|t| *t == tuple).unwrap() + 1
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cell {
    Atom(&'static str),
    Num(Q),
    Slot(usize),
    Var(&'static str),
}
pub type Fact = (&'static str, Vec<Cell>);

// Slots are labelled symbols, not numeric witnesses. One branch-local environment
// spans the whole conjunction; equal numbers in other facts cannot acquire a label.
pub fn relevant(facts: &[Fact], goals: &[Fact], count: &'static str, slot: usize) -> bool {
    let mut rows = vec![BTreeMap::<&str, Cell>::new()];
    for (name, args) in goals {
        let mut next = Vec::new();
        for env in rows {
            for (_, values) in facts
                .iter()
                .filter(|(n, a)| n == name && a.len() == args.len())
            {
                let mut branch = env.clone();
                if args.iter().zip(values).all(|(goal, fact)| match goal {
                    Cell::Var(v) => match branch.get(v) {
                        Some(bound) => bound == fact,
                        None => {
                            branch.insert(v, fact.clone());
                            true
                        }
                    },
                    value => value == fact,
                }) {
                    next.push(branch);
                }
            }
        }
        rows = next;
    }
    rows.iter().any(|r| r.get(count) == Some(&Cell::Slot(slot)))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Atom(String),
    Num(i128),
    Fun(String, Vec<Self>),
    List(Vec<Self>),
}
impl Term {
    pub fn args(&self, functor: &str) -> &[Self] {
        match self {
            Self::Fun(f, xs) if f == functor => xs,
            _ => panic!("expected {functor}: {self:?}"),
        }
    }
    pub fn list(&self) -> &[Self] {
        match self {
            Self::List(xs) => xs,
            _ => panic!("expected list: {self:?}"),
        }
    }
    pub fn q(&self) -> Q {
        match self {
            Self::Num(n) => Q::int(*n),
            Self::Fun(f, ns) if f == "rational" && ns.len() == 2 => {
                let (Self::Num(n), Self::Num(d)) = (&ns[0], &ns[1]) else {
                    panic!("number: {self:?}")
                };
                let q = Q::new(*n, *d);
                assert_eq!((q.0, q.1), (*n, *d), "canonical rational");
                assert!(*d >= 2);
                q
            }
            _ => panic!("expected number: {self:?}"),
        }
    }
    pub fn atom(&self) -> &str {
        match self {
            Self::Atom(a) => a,
            _ => panic!("expected atom: {self:?}"),
        }
    }
}
pub fn terms(s: &str) -> Vec<Term> {
    struct Parser<'a>(&'a [u8], usize);
    impl Parser<'_> {
        fn ws(&mut self) {
            while self.1 < self.0.len() && self.0[self.1].is_ascii_whitespace() {
                self.1 += 1;
            }
        }
        fn eat(&mut self, x: u8) -> bool {
            self.ws();
            if self.0.get(self.1) == Some(&x) {
                self.1 += 1;
                true
            } else {
                false
            }
        }
        fn term(&mut self) -> Term {
            self.ws();
            if self.eat(b'[') {
                return Term::List(self.seq(b']'));
            }
            let mut token = String::new();
            if self.eat(b'\'') {
                loop {
                    let c = self.0[self.1];
                    self.1 += 1;
                    if c == b'\'' {
                        break;
                    }
                    if c == b'\\' {
                        token.push(self.0[self.1] as char);
                        self.1 += 1;
                    } else {
                        token.push(c as char);
                    }
                }
            } else {
                while self.1 < self.0.len() && !b",()[] .\n\r\t".contains(&self.0[self.1]) {
                    token.push(self.0[self.1] as char);
                    self.1 += 1;
                }
            }
            assert!(!token.is_empty(), "term at byte {}", self.1);
            if self.eat(b'(') {
                Term::Fun(token, self.seq(b')'))
            } else if let Ok(n) = token.parse() {
                Term::Num(n)
            } else {
                Term::Atom(token)
            }
        }
        fn seq(&mut self, end: u8) -> Vec<Term> {
            let mut out = Vec::new();
            if self.eat(end) {
                return out;
            }
            loop {
                out.push(self.term());
                if self.eat(end) {
                    return out;
                }
                assert!(self.eat(b','), "comma at byte {}", self.1);
            }
        }
    }
    let mut p = Parser(s.as_bytes(), 0);
    let mut out = Vec::new();
    loop {
        p.ws();
        if p.1 == p.0.len() {
            return out;
        }
        out.push(p.term());
        assert!(p.eat(b'.'), "period at {}", p.1);
    }
}

#[derive(Debug)]
pub struct Walk {
    pub sentence: i128,
    pub variant: i128,
    pub candidates: Vec<Vec<Q>>,
    pub winner: Option<Vec<usize>>,
    pub fragment: bool,
}
pub fn walks(s: &str) -> Vec<Walk> {
    terms(s)
        .iter()
        .map(|t| {
            let a = t.args("$guideline_walk");
            assert_eq!(a.len(), 5);
            let number = |t: &Term| match t {
                Term::Num(n) => *n,
                _ => panic!("integer: {t:?}"),
            };
            let winner = match &a[3] {
                Term::Atom(n) if n == "none" => None,
                t => Some(
                    t.args("winner")[0]
                        .list()
                        .iter()
                        .map(|d| usize::try_from(number(d)).unwrap())
                        .collect(),
                ),
            };
            let fragment = match a[4].args("fragment")[0].atom() {
                "true" => true,
                "false" => false,
                x => panic!("fragment: {x}"),
            };
            Walk {
                sentence: number(&a[0]),
                variant: number(&a[1].args("variant")[0]),
                candidates: a[2]
                    .list()
                    .iter()
                    .map(|l| l.list().iter().map(Term::q).collect())
                    .collect(),
                winner,
                fragment,
            }
        })
        .collect()
}
