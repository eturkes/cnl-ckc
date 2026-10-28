use super::common::*;
use super::inventories::Guideline;
use std::collections::{BTreeMap, HashSet};
// Formula cycle scan (contracts da2 D9 + da3; corpus.md Dose arithmetic): plain leftmost search
// runs a formula rule's body in written order, and a condition goal whose key (argument 2) is
// still unbound enumerates every clause head it unifies with. When that search can reach a
// clause of the rule's own sentence — a shared verb + noun, a fresh-entity rule whose body
// reads back into the rule, an entity-less time-quantity cardinality — a standard engine
// searches without end and the native engine spends its bound on each failing branch. The
// scan walks each formula body as the prototype did: bound = the variables of the goals before
// the open one; a reached clause of another sentence walks its own body (depth ≤ 7; a repeated
// clause is allowed, so the search finds every path the bounded walk can). Text-level over
// canonical v1 lines.
const DEPTH: usize = 7;
const BUILTINS: [&str; 3] = [
    "guideline_product",
    "guideline_quotient",
    "guideline_satisfies",
];

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum T {
    V(usize),
    A(String),
    N(String),
    C(String, Vec<T>),
}
enum Goal {
    Plain(T),
    Naf,
}
struct Clause {
    doc: String,
    line: usize,
    own: Option<(String, String)>,
    head: T,
    body: Vec<Goal>,
    vars: usize,
}

pub(super) fn check(g: &Guideline) -> Result {
    let mut clauses = Vec::new();
    for id in &g.docids {
        let src = corpus_text(&g.pl(id), "formula-cycle")?;
        for (n, line) in src.split('\n').enumerate() {
            if line.is_empty() || line.starts_with('%') || line.starts_with(":-") {
                continue;
            }
            // A line the reader cannot parse is the canonical-form checks' to report.
            let Some(c) = clause(line) else { continue };
            clauses.push(Clause {
                doc: id.clone(),
                line: n + 1,
                ..c
            });
        }
    }
    let mut index: Index = BTreeMap::new();
    for (i, c) in clauses.iter().enumerate() {
        if let T::C(f, a) = &c.head {
            index.entry((f.clone(), a.len())).or_default().push((
                konst(a.first()),
                konst(a.get(2)),
                i,
            ));
        }
    }
    let mut formulas = 0;
    for (i, c) in clauses.iter().enumerate() {
        let formula = c.body.iter().any(|g| {
            matches!(g, Goal::Plain(T::C(f, a)) if a.len() == 6
                && (f == "guideline_product" || f == "guideline_quotient"))
        });
        if !formula {
            continue;
        }
        formulas += 1;
        // corpus.md Dose arithmetic: a formula stays out of any rule with a time condition (its
        // converted time quantity counts before its event in written order); kept as a red case.
        if c.body
            .iter()
            .any(|g| matches!(g, Goal::Plain(T::C(f, _)) if f == "guideline_unit"))
        {
            return Err(violation(
                "formula-cycle",
                format!(
                    "guidelines/{}/pl/{}.pl:{}: a formula condition holds a time quantity",
                    name(&g.path),
                    c.doc,
                    c.line
                ),
            ));
        }
        let mut s = Search {
            clauses: &clauses,
            index: &index,
            store: vec![None; c.vars],
            trail: vec![],
            failed: HashSet::new(),
        };
        let body: Vec<T> = goals(&c.body);
        if let Some(path) = s.walk(&body, &[], c.own.as_ref(), &mut vec![i]) {
            let at = clauses[path[path.len() - 1]].line;
            return Err(violation(
                "formula-cycle",
                format!(
                    "guidelines/{}/pl/{}.pl:{}: a formula condition reaches its own sentence (:{at}) through {}",
                    name(&g.path),
                    c.doc,
                    c.line,
                    {
                        let mut steps: Vec<String> = path
                            .iter()
                            .skip(1)
                            .map(|k| format!("{}:{}", clauses[*k].doc, clauses[*k].line))
                            .collect();
                        steps.dedup();
                        steps.join(" → ")
                    }
                ),
            ));
        }
    }
    println!(
        "ckc: formula-cycles ok {} {formulas} formula clauses",
        name(&g.path)
    );
    Ok(())
}

// Heads per predicate + arity, each with its argument 1 + 3 when an atom or number.
type Index = BTreeMap<(String, usize), Vec<(Option<T>, Option<T>, usize)>>;
fn konst(t: Option<&T>) -> Option<T> {
    match t {
        Some(T::A(a)) => Some(T::A(a.clone())),
        Some(T::N(n)) => Some(T::N(n.clone())),
        _ => None,
    }
}

fn goals(body: &[Goal]) -> Vec<T> {
    body.iter()
        .map(|g| match g {
            Goal::Plain(t) => t.clone(),
            Goal::Naf => T::A("\\+".into()),
        })
        .collect()
}

struct Search<'a> {
    clauses: &'a [Clause],
    index: &'a Index,
    store: Vec<Option<T>>,
    trail: Vec<usize>,
    // Failed entries: (clause, its head after unification with variables numbered by first
    // occurrence + marked when bound by the caller, remaining depth). The walk below an entry
    // depends on exactly that state, so a recorded failure prunes an identical later entry.
    failed: HashSet<(usize, String, usize)>,
}
impl Search<'_> {
    fn canon(&self, t: &T, bound: &[usize], names: &mut Vec<usize>, out: &mut String) {
        match self.deref(t) {
            T::V(v) => {
                let n = names.iter().position(|x| *x == v).unwrap_or_else(|| {
                    names.push(v);
                    names.len() - 1
                });
                out.push_str(if bound.contains(&v) { "B" } else { "V" });
                out.push_str(&n.to_string());
            }
            T::A(a) => {
                out.push('\'');
                out.push_str(&a);
            }
            T::N(n) => out.push_str(&n),
            T::C(f, a) => {
                out.push_str(&f);
                out.push('(');
                for x in &a {
                    self.canon(x, bound, names, out);
                    out.push(',');
                }
                out.push(')');
            }
        }
    }
    fn deref(&self, t: &T) -> T {
        let mut t = t.clone();
        while let T::V(v) = t {
            match &self.store[v] {
                Some(b) => t = b.clone(),
                None => return T::V(v),
            }
        }
        t
    }
    fn vars(&self, t: &T, out: &mut Vec<usize>) {
        match self.deref(t) {
            T::V(v) => out.push(v),
            T::C(_, a) => a.iter().for_each(|x| self.vars(x, out)),
            _ => {}
        }
    }
    fn occurs(&self, v: usize, t: &T) -> bool {
        match self.deref(t) {
            T::V(w) => v == w,
            T::C(_, a) => a.iter().any(|x| self.occurs(v, x)),
            _ => false,
        }
    }
    fn bind(&mut self, v: usize, t: T) {
        self.store[v] = Some(t);
        self.trail.push(v);
    }
    fn unify(&mut self, a: &T, b: &T) -> bool {
        match (self.deref(a), self.deref(b)) {
            (T::V(x), T::V(y)) if x == y => true,
            (T::V(x), t) | (t, T::V(x)) => {
                if self.occurs(x, &t) {
                    return false;
                }
                self.bind(x, t);
                true
            }
            (T::C(f, xs), T::C(g, ys)) => {
                f == g && xs.len() == ys.len() && xs.iter().zip(&ys).all(|(x, y)| self.unify(x, y))
            }
            (x, y) => x == y,
        }
    }
    fn undo(&mut self, mark: usize) {
        while self.trail.len() > mark {
            let v = self.trail.pop().unwrap();
            self.store[v] = None;
        }
    }
    // A fresh copy of clause k: its variables renumbered after the store's end.
    fn rename(&mut self, k: usize) -> (T, Vec<T>) {
        let base = self.store.len();
        let c = &self.clauses[k];
        self.store.resize(base + c.vars, None);
        let shift = |t: &T| shift(t, base);
        (shift(&c.head), goals(&c.body).iter().map(shift).collect())
    }
    fn walk(
        &mut self,
        body: &[T],
        bound: &[usize],
        own: Option<&(String, String)>,
        vis: &mut Vec<usize>,
    ) -> Option<Vec<usize>> {
        let mut bound = bound.to_vec();
        for g in body {
            let g = self.deref(g);
            let T::C(f, args) = &g else { continue };
            if f == "\\+" {
                continue;
            }
            if BUILTINS.contains(&f.as_str()) {
                self.vars(&g, &mut bound);
                continue;
            }
            if let Some(T::V(k)) = args.get(1).map(|k| self.deref(k))
                && !bound.contains(&k)
                && let Some(p) = self.reach(&g, own, vis, &bound)
            {
                return Some(p);
            }
            self.vars(&g, &mut bound);
        }
        None
    }
    fn reach(
        &mut self,
        g: &T,
        own: Option<&(String, String)>,
        vis: &mut Vec<usize>,
        bound: &[usize],
    ) -> Option<Vec<usize>> {
        if vis.len() >= DEPTH {
            return None;
        }
        let T::C(f, a) = g else { return None };
        // Same predicate; a head whose argument 1 or 3 is a constant that clashes with the
        // goal's constant there cannot unify, so it is skipped before the unifier runs.
        let (g1, g3) = (
            konst(a.first().map(|x| self.deref(x)).as_ref()),
            konst(a.get(2).map(|x| self.deref(x)).as_ref()),
        );
        let fits =
            |want: &Option<T>, have: &Option<T>| want.is_none() || have.is_none() || want == have;
        let candidates: Vec<usize> = self
            .index
            .get(&(f.clone(), a.len()))
            .map(|v| {
                v.iter()
                    .filter(|(h1, h3, _)| fits(&g1, h1) && fits(&g3, h3))
                    .map(|e| e.2)
                    .collect()
            })
            .unwrap_or_default();
        for k in candidates {
            let mark = self.trail.len();
            let len = self.store.len();
            let (h, b) = self.rename(k);
            let ok = self.unify(g, &h);
            if ok && own.is_some() && self.clauses[k].own.as_ref() == own {
                vis.push(k);
                return Some(vis.clone());
            }
            if ok && !b.is_empty() {
                let mut key = String::new();
                self.canon(&h, bound, &mut Vec::new(), &mut key);
                let key = (k, key, DEPTH - vis.len());
                if !self.failed.contains(&key) {
                    vis.push(k);
                    if let Some(p) = self.walk(&b, bound, own, vis) {
                        return Some(p);
                    }
                    vis.pop();
                    self.failed.insert(key);
                }
            }
            self.undo(mark);
            self.store.truncate(len);
        }
        None
    }
}

fn shift(t: &T, base: usize) -> T {
    match t {
        T::V(v) => T::V(v + base),
        T::C(f, a) => T::C(f.clone(), a.iter().map(|x| shift(x, base)).collect()),
        x => x.clone(),
    }
}

// ---- canonical v1 clause reader (one line) ----
struct P<'a> {
    s: &'a [u8],
    i: usize,
    names: BTreeMap<String, usize>,
    vars: usize,
}
fn clause(line: &str) -> Option<Clause> {
    let mut p = P {
        s: line.as_bytes(),
        i: 0,
        names: BTreeMap::new(),
        vars: 0,
    };
    let head = p.term()?;
    let mut body = Vec::new();
    if p.eat(" :- ") {
        loop {
            body.push(p.goal()?);
            if !p.eat(", ") {
                break;
            }
        }
    }
    p.eat(".").then_some(())?;
    (p.i == p.s.len()).then_some(())?;
    let own = own(&head);
    Some(Clause {
        doc: String::new(),
        line: 0,
        own,
        head,
        body,
        vars: p.vars,
    })
}
fn own(head: &T) -> Option<(String, String)> {
    fn find(t: &T) -> Option<(String, String)> {
        match t {
            T::C(f, a) if f == "$guideline_id" && a.len() == 5 => match (&a[1], &a[2]) {
                (T::A(d), T::N(s)) => Some((d.clone(), s.clone())),
                _ => None,
            },
            T::C(_, a) => a.iter().find_map(find),
            _ => None,
        }
    }
    find(head)
}
impl P<'_> {
    fn eat(&mut self, t: &str) -> bool {
        let ok = self.s[self.i..].starts_with(t.as_bytes());
        if ok {
            self.i += t.len();
        }
        ok
    }
    fn goal(&mut self) -> Option<Goal> {
        if self.eat("\\+ ") {
            if self.eat("(") {
                loop {
                    self.goal()?;
                    if !self.eat(", ") {
                        break;
                    }
                }
                self.eat(")").then_some(())?;
            } else {
                self.term()?;
            }
            return Some(Goal::Naf);
        }
        Some(Goal::Plain(self.term()?))
    }
    fn term(&mut self) -> Option<T> {
        let c = *self.s.get(self.i)?;
        if c == b'[' {
            self.i += 1;
            let mut items = Vec::new();
            if !self.eat("]") {
                loop {
                    items.push(self.term()?);
                    if !self.eat(",") {
                        break;
                    }
                }
                let tail = if self.eat("|") {
                    self.term()?
                } else {
                    T::A("[]".into())
                };
                self.eat("]").then_some(())?;
                return Some(
                    items
                        .into_iter()
                        .rev()
                        .fold(tail, |t, h| T::C(".".into(), vec![h, t])),
                );
            }
            return Some(T::A("[]".into()));
        }
        if c == b'_' || c.is_ascii_uppercase() {
            let n = self.ident();
            let name = String::from_utf8_lossy(&self.s[self.i - n..self.i]).into_owned();
            if name == "_" {
                self.vars += 1;
                return Some(T::V(self.vars - 1));
            }
            let next = self.vars;
            let v = *self.names.entry(name).or_insert(next);
            if v == next {
                self.vars += 1;
            }
            return Some(T::V(v));
        }
        if c.is_ascii_digit()
            || (c == b'-' && self.s.get(self.i + 1).is_some_and(u8::is_ascii_digit))
        {
            let start = self.i;
            self.i += 1;
            while self
                .s
                .get(self.i)
                .is_some_and(|b| b.is_ascii_digit() || *b == b'.')
            {
                self.i += 1;
            }
            return Some(T::N(
                String::from_utf8_lossy(&self.s[start..self.i]).into_owned(),
            ));
        }
        let name = self.atom()?;
        if self.eat("(") {
            let mut args = Vec::new();
            loop {
                args.push(self.term()?);
                if !self.eat(",") {
                    break;
                }
            }
            self.eat(")").then_some(())?;
            return Some(T::C(name, args));
        }
        Some(T::A(name))
    }
    fn ident(&mut self) -> usize {
        let start = self.i;
        while self
            .s
            .get(self.i)
            .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
        {
            self.i += 1;
        }
        self.i - start
    }
    fn atom(&mut self) -> Option<String> {
        if self.eat("'") {
            let mut out = Vec::new();
            loop {
                match *self.s.get(self.i)? {
                    b'\\' => {
                        out.push(*self.s.get(self.i + 1)?);
                        self.i += 2;
                    }
                    b'\'' => {
                        self.i += 1;
                        return String::from_utf8(out).ok();
                    }
                    b => {
                        out.push(b);
                        self.i += 1;
                    }
                }
            }
        }
        let c = *self.s.get(self.i)?;
        if c.is_ascii_lowercase() {
            let n = self.ident();
            return Some(String::from_utf8_lossy(&self.s[self.i - n..self.i]).into_owned());
        }
        // symbol-char atoms of the canonical writer (`=`, `<`, `-`, …) up to a delimiter
        let start = self.i;
        while self
            .s
            .get(self.i)
            .is_some_and(|b| b"+-*/\\^<>=~:.?@#&$".contains(b))
        {
            self.i += 1;
        }
        (self.i > start).then(|| String::from_utf8_lossy(&self.s[start..self.i]).into_owned())
    }
}
