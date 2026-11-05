use sb1_checker::term::{Arena, Id, Node};
use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Value {
    Any,
    Number,
    Patient,
    Exact(Id),
    Origin(usize),
}

#[derive(Default, Debug, Clone)]
pub struct Counters {
    pub feature_insertions: u64,
    pub candidate_pairs: u64,
    pub worklist_steps: u64,
    pub join_rows: u64,
    pub writer_insertions: u64,
}

pub struct Abstraction {
    terms: Vec<Value>,
    pub origins: Vec<Id>,
}
impl Abstraction {
    pub fn new(arena: &Arena) -> Self {
        let mut keys = HashMap::new();
        let mut origins = Vec::new();
        let terms = arena
            .nodes
            .iter()
            .enumerate()
            .map(|(id, node)| match node.as_ref() {
                Node::Var(_) => Value::Any,
                Node::Compound(n, a) if arena.name(*n) == "$guideline_id" && a.len() == 5 => {
                    let next = keys.len();
                    let key = [a[0], a[1], a[2], a[3]];
                    Value::Origin(*keys.entry(key).or_insert_with(|| {
                        origins.push(id);
                        next
                    }))
                }
                _ => Value::Exact(id),
            })
            .collect();
        Self { terms, origins }
    }
    pub fn value(&self, t: Id) -> Value {
        self.terms[t]
    }
    pub(crate) fn memory_rows(&self) -> [crate::memory::Row; 2] {
        use crate::memory::bytes;
        [
            (
                "abstract-values",
                self.terms.len(),
                bytes::<Value>(self.terms.capacity()),
                "buffer-capacity",
            ),
            (
                "abstract-origins",
                self.origins.len(),
                bytes::<Id>(self.origins.capacity()),
                "buffer-capacity",
            ),
        ]
    }
    pub fn intersect(arena: &Arena, a: Value, b: Value) -> Option<Value> {
        if a == b {
            return Some(a);
        }
        match (a, b) {
            (Value::Any, x) | (x, Value::Any) => Some(x),
            (Value::Patient, x) | (x, Value::Patient)
                if x == Value::Number
                    || matches!(x,Value::Exact(t) if matches!(arena.node(t),Node::Atom(_) | Node::Integer(_)) || arena.named(t,"rational",2).is_some()) =>
            {
                Some(x)
            }
            (Value::Number, x) | (x, Value::Number) if matches!(x,Value::Exact(t) if matches!(arena.node(t),Node::Integer(_)) || arena.named(t,"rational",2).is_some()) => {
                Some(x)
            }
            _ => None,
        }
    }
    pub fn describe(&self, arena: &Arena, value: Value) -> String {
        match value {
            Value::Any => "any".into(),
            Value::Number => "number".into(),
            Value::Patient => "patient(all-features)".into(),
            Value::Exact(t) => arena.render(t),
            Value::Origin(o) => {
                let a = arena.named(self.origins[o], "$guideline_id", 5).unwrap();
                format!(
                    "origin({},{},{},{})",
                    arena.render(a[0]),
                    arena.render(a[1]),
                    arena.render(a[2]),
                    arena.render(a[3])
                )
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Domain {
    All,
    Set(BTreeSet<Value>),
}
impl Domain {
    pub fn one(v: Value) -> Self {
        if v == Value::Any {
            Self::All
        } else {
            Self::Set(BTreeSet::from([v]))
        }
    }
    pub fn empty() -> Self {
        Self::Set(BTreeSet::new())
    }
    pub fn values(&self) -> Option<&BTreeSet<Value>> {
        match self {
            Self::All => None,
            Self::Set(s) => Some(s),
        }
    }
    pub fn is_empty(&self) -> bool {
        matches!(self,Self::Set(s) if s.is_empty())
    }
    fn normalized(mut self, arena: &Arena) -> Self {
        if let Self::Set(values) = &mut self {
            let patient = values.contains(&Value::Patient);
            let number = values.contains(&Value::Number);
            values.retain(|v| match v {
                Value::Number => !patient,
                Value::Exact(t) => {
                    !(patient && matches!(arena.node(*t), Node::Atom(_))
                        || (patient || number)
                            && (matches!(arena.node(*t), Node::Integer(_))
                                || arena.named(*t, "rational", 2).is_some()))
                }
                _ => true,
            });
        }
        self
    }
    #[cfg(test)]
    pub(crate) fn meet_reference(&self, other: &Self, arena: &Arena) -> Self {
        match (self, other) {
            (Self::All, x) | (x, Self::All) => x.clone().normalized(arena),
            (Self::Set(a), Self::Set(b)) => {
                let mut out = a.intersection(b).copied().collect::<BTreeSet<_>>();
                for wild in [Value::Patient, Value::Number] {
                    if a.contains(&wild) {
                        out.extend(
                            b.iter()
                                .filter_map(|b| Abstraction::intersect(arena, wild, *b)),
                        );
                    }
                    if b.contains(&wild) {
                        out.extend(
                            a.iter()
                                .filter_map(|a| Abstraction::intersect(arena, *a, wild)),
                        );
                    }
                }
                Self::Set(out).normalized(arena)
            }
        }
    }
    pub fn meet(&self, other: &Self, arena: &Arena) -> Self {
        match (self, other) {
            (Self::All, x) | (x, Self::All) => x.clone().normalized(arena),
            (Self::Set(a), Self::Set(b)) => {
                let mut out = a.intersection(b).copied().collect::<BTreeSet<_>>();
                for wild in [Value::Patient, Value::Number] {
                    // The shared wildcard already covers every scalar another
                    // scan could contribute. Keep only non-subsumed choices.
                    if out.contains(&Value::Patient)
                        || wild == Value::Number && out.contains(&Value::Number)
                    {
                        continue;
                    }
                    if a.contains(&wild) {
                        out.extend(
                            b.iter()
                                .filter_map(|b| Abstraction::intersect(arena, wild, *b)),
                        );
                    }
                    if b.contains(&wild) {
                        out.extend(
                            a.iter()
                                .filter_map(|a| Abstraction::intersect(arena, *a, wild)),
                        );
                    }
                }
                Self::Set(out).normalized(arena)
            }
        }
    }
    pub fn union(&mut self, other: &Self) -> bool {
        if *self == Self::All {
            return false;
        }
        match other {
            Self::All => {
                *self = Self::All;
                true
            }
            Self::Set(b) => {
                let Self::Set(a) = self else { unreachable!() };
                let before = a.len();
                a.extend(b);
                before != a.len()
            }
        }
    }
}
