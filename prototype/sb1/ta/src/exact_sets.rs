//! Exact finite-set identities. Raw unions and normalized meets are distinct operations.
use crate::domain::{Abstraction, Domain, Value};
use sb1_checker::term::{Arena, Node};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Set = usize;
pub const ALL: Set = 0;
pub const EMPTY: Set = 1;

#[derive(Default)]
struct Store {
    values: Vec<Rc<[Value]>>,
    ids: HashMap<Rc<[Value]>, Set>,
    meets: HashMap<(Set, Set), Set>,
    unions: HashMap<(Set, Set), Set>,
    normalized: HashMap<Set, Set>,
    visits: usize,
    members: usize,
}
pub struct Sets(RefCell<Store>);
impl Default for Sets {
    fn default() -> Self {
        let pool = Self(RefCell::new(Store::default()));
        // All has no finite denotation; its reserved cell is never read as one.
        pool.0.borrow_mut().values.push(Rc::from([]));
        assert_eq!(pool.intern(Vec::new()), EMPTY);
        pool
    }
}
impl Sets {
    pub fn intern(&self, mut values: Vec<Value>) -> Set {
        values.sort_unstable();
        values.dedup();
        let mut store = self.0.borrow_mut();
        if let Some(id) = store.ids.get(values.as_slice()) {
            return *id;
        }
        store.visits += values.len();
        store.members += values.len();
        let id = store.values.len();
        let values = Rc::<[Value]>::from(values);
        store.ids.insert(values.clone(), id);
        store.values.push(values);
        id
    }
    pub fn one(&self, value: Value) -> Set {
        if value == Value::Any {
            ALL
        } else {
            self.intern(vec![value])
        }
    }
    pub fn raw_one(&self, value: Value) -> Set {
        self.intern(vec![value])
    }
    pub fn values(&self, id: Set) -> Option<Rc<[Value]>> {
        (id != ALL).then(|| self.0.borrow().values[id].clone())
    }
    pub fn len(&self, id: Set) -> usize {
        if id == ALL {
            1
        } else {
            self.0.borrow().values[id].len()
        }
    }
    pub fn contains(&self, id: Set, value: Value) -> bool {
        self.values(id)
            .is_none_or(|s| s.binary_search(&value).is_ok())
    }
    pub fn domain(&self, id: Set) -> Domain {
        self.values(id)
            .map_or(Domain::All, |s| Domain::Set(s.iter().copied().collect()))
    }
    pub fn from_domain(&self, domain: &Domain) -> Set {
        domain
            .values()
            .map_or(ALL, |s| self.intern(s.iter().copied().collect()))
    }
    pub fn normalize(&self, id: Set, arena: &Arena) -> Set {
        if id == ALL {
            return ALL;
        }
        if let Some(result) = self.0.borrow().normalized.get(&id) {
            return *result;
        }
        let values = self.values(id).unwrap();
        let patient = values.binary_search(&Value::Patient).is_ok();
        let number = values.binary_search(&Value::Number).is_ok();
        let normalized = self.intern(
            values
                .iter()
                .copied()
                .filter(|value| match value {
                    Value::Number => !patient,
                    Value::Exact(t) => {
                        !(patient && matches!(arena.node(*t), Node::Atom(_))
                            || (patient || number)
                                && (matches!(arena.node(*t), Node::Integer(_))
                                    || arena.named(*t, "rational", 2).is_some()))
                    }
                    _ => true,
                })
                .collect(),
        );
        self.0.borrow_mut().normalized.insert(id, normalized);
        normalized
    }
    pub fn meet(&self, a: Set, b: Set, arena: &Arena) -> Set {
        if a == ALL {
            return self.normalize(b, arena);
        }
        if b == ALL {
            return self.normalize(a, arena);
        }
        let key = if a < b { (a, b) } else { (b, a) };
        if let Some(result) = self.0.borrow().meets.get(&key) {
            return *result;
        }
        let av = self.values(a).unwrap();
        let bv = self.values(b).unwrap();
        let mut out = Vec::new();
        // Domain::meet treats an Any member inside a finite set as an ordinary
        // member, unlike Abstraction::intersect. Only Patient/Number broaden it.
        let (small, large) = if av.len() <= bv.len() {
            (&av, &bv)
        } else {
            (&bv, &av)
        };
        for value in small.iter() {
            if large.binary_search(value).is_ok() {
                out.push(*value);
            }
        }
        for wild in [Value::Patient, Value::Number] {
            if av.binary_search(&wild).is_ok() {
                out.extend(
                    bv.iter()
                        .filter_map(|v| Abstraction::intersect(arena, wild, *v)),
                );
            }
            if bv.binary_search(&wild).is_ok() {
                out.extend(
                    av.iter()
                        .filter_map(|v| Abstraction::intersect(arena, *v, wild)),
                );
            }
        }
        let raw = self.intern(out);
        let result = self.normalize(raw, arena);
        self.0.borrow_mut().meets.insert(key, result);
        result
    }
    pub fn union(&self, a: Set, b: Set) -> Set {
        if a == ALL || b == ALL {
            return ALL;
        }
        if a == b || b == EMPTY {
            return a;
        }
        if a == EMPTY {
            return b;
        }
        let key = if a < b { (a, b) } else { (b, a) };
        if let Some(result) = self.0.borrow().unions.get(&key) {
            return *result;
        }
        let av = self.values(a).unwrap();
        let bv = self.values(b).unwrap();
        let result = self.intern(av.iter().chain(bv.iter()).copied().collect());
        self.0.borrow_mut().unions.insert(key, result);
        result
    }
    pub fn raw_keys(&self, domain: Set) -> Set {
        if domain == ALL {
            self.raw_one(Value::Any)
        } else {
            domain
        }
    }
    pub fn compatible(&self, a: Set, b: Set, arena: &Arena) -> Set {
        if a == ALL {
            return if b == EMPTY { EMPTY } else { ALL };
        }
        if b == ALL {
            return a;
        }
        let av = self.values(a).unwrap();
        let bv = self.values(b).unwrap();
        if bv.binary_search(&Value::Any).is_ok() {
            return a;
        }
        let patient = bv.binary_search(&Value::Patient).is_ok();
        let number = bv.binary_search(&Value::Number).is_ok();
        let numeric = |v| matches!(v, Value::Exact(t) if matches!(arena.node(t), Node::Integer(_)) || arena.named(t, "rational", 2).is_some());
        let atomic = |v| {
            numeric(v) || matches!(v, Value::Exact(t) if matches!(arena.node(t), Node::Atom(_)))
        };
        let has_number = bv.iter().copied().any(numeric);
        let has_atom = bv.iter().copied().any(atomic);
        // Keep raw caller members. Patient/Number subsumption is compatibility,
        // not Domain::meet's normalization (and Any-in-Set is a wildcard here).
        self.intern(
            av.iter()
                .copied()
                .filter(|v| {
                    bv.binary_search(v).is_ok()
                        || match v {
                            Value::Any => !bv.is_empty(),
                            Value::Patient => patient || number || has_atom,
                            Value::Number => patient || number || has_number,
                            Value::Exact(_) => patient && atomic(*v) || number && numeric(*v),
                            Value::Origin(_) => false,
                        }
                })
                .collect(),
        )
    }
    pub fn allocation_charge(&self) -> usize {
        use std::mem::size_of;
        let s = self.0.borrow();
        s.members * size_of::<Value>()
            + s.values.capacity() * size_of::<Rc<[Value]>>()
            + s.ids.capacity() * size_of::<(Rc<[Value]>, Set)>()
            + (s.meets.capacity() + s.unions.capacity()) * size_of::<((Set, Set), Set)>()
            + s.normalized.capacity() * size_of::<(Set, Set)>()
    }
    pub fn retention(&self, roots: &[Set]) -> Vec<(&'static str, usize)> {
        use std::mem::size_of;
        let s = self.0.borrow();
        let mut roots = roots.to_vec();
        roots.extend([ALL, EMPTY]);
        roots.sort_unstable();
        roots.dedup();
        let live_members = roots.iter().map(|r| s.values[*r].len()).sum::<usize>();
        // Equal per-entry payload charge on both sides. Hash/allocator slack
        // stays separate: this ratio detects retained obsolete set versions.
        let entry = 2 * size_of::<Rc<[Value]>>() + size_of::<Set>() + 2 * size_of::<usize>();
        vec![
            ("retained_entries", s.values.len()),
            ("live_entries", roots.len()),
            (
                "retained_bytes",
                s.members * size_of::<Value>() + s.values.len() * entry,
            ),
            (
                "live_bytes",
                live_members * size_of::<Value>() + roots.len() * entry,
            ),
        ]
    }
    pub fn compact(&self, roots: &[Set]) -> Vec<Set> {
        let mut keep = roots.to_vec();
        keep.extend([ALL, EMPTY]);
        keep.sort_unstable();
        keep.dedup();
        let mut old = self.0.borrow_mut();
        let mut mapping = vec![usize::MAX; old.values.len()];
        let mut next = Store {
            visits: old.visits,
            ..Store::default()
        };
        for original in keep {
            let id = next.values.len();
            mapping[original] = id;
            let values = old.values[original].clone();
            next.members += values.len();
            if original != ALL {
                next.ids.insert(values.clone(), id);
            }
            next.values.push(values);
        }
        assert_eq!((mapping[ALL], mapping[EMPTY]), (ALL, EMPTY));
        next.values.shrink_to_fit();
        next.ids.shrink_to_fit();
        *old = next;
        mapping
    }
    pub fn metrics(&self) -> Vec<(&'static str, usize)> {
        let s = self.0.borrow();
        vec![
            ("sets", s.values.len()),
            ("set_memberships", s.values.iter().map(|s| s.len()).sum()),
            ("meet_cache", s.meets.len()),
            ("union_cache", s.unions.len()),
            ("interned_member_visits", s.visits),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn set_operations_equal_native_domain_on_generated_raw_sets() {
        let mut arena = Arena::default();
        let atom = arena.atom("a");
        let num = arena.put(Node::Integer("1".into()));
        let denominator = arena.put(Node::Integer("2".into()));
        let rational = arena.compound("rational", vec![num, denominator]);
        let compound = arena.compound("other", vec![atom]);
        let values = [
            Value::Any,
            Value::Patient,
            Value::Number,
            Value::Exact(atom),
            Value::Exact(num),
            Value::Exact(rational),
            Value::Exact(compound),
            Value::Origin(0),
        ];
        let pool = Sets::default();
        for left in 0..=256 {
            for right in 0..=256 {
                let domain = |mask: usize| {
                    if mask == 256 {
                        Domain::All
                    } else {
                        Domain::Set(
                            values
                                .iter()
                                .enumerate()
                                .filter_map(|(i, v)| (mask & (1 << i) != 0).then_some(*v))
                                .collect::<BTreeSet<_>>(),
                        )
                    }
                };
                let a = domain(left);
                let b = domain(right);
                let ai = pool.from_domain(&a);
                let bi = pool.from_domain(&b);
                assert_eq!(pool.domain(pool.meet(ai, bi, &arena)), a.meet(&b, &arena));
                let mut union = a.clone();
                union.union(&b);
                assert_eq!(pool.domain(pool.union(ai, bi)), union);
                if let (Some(av), Some(bv)) = (a.values(), b.values()) {
                    let expected = av
                        .iter()
                        .copied()
                        .filter(|a| {
                            bv.iter()
                                .any(|b| Abstraction::intersect(&arena, *a, *b).is_some())
                        })
                        .collect();
                    assert_eq!(
                        pool.domain(pool.compatible(ai, bi, &arena)),
                        Domain::Set(expected)
                    );
                }
            }
        }
    }
    #[test]
    fn compaction_preserves_all_live_states_and_releases_unreferenced_versions() {
        let mut arena = Arena::default();
        let a = arena.atom("a");
        let pool = Sets::default();
        let live = [
            pool.one(Value::Origin(3)),
            pool.intern(vec![Value::Patient, Value::Exact(a)]),
        ];
        let expected = live.map(|id| pool.domain(id));
        for i in 0..500 {
            pool.intern(vec![Value::Origin(i), Value::Origin(i + 1)]);
        }
        let before = pool.retention(&live);
        assert!(before[0].1 > before[1].1);
        let ids = pool.compact(&live);
        for (old, expected) in live.into_iter().zip(expected) {
            assert_eq!(pool.domain(ids[old]), expected);
        }
        let after = pool.retention(&live.map(|old| ids[old]));
        assert_eq!(after[0].1, after[1].1);
        assert_eq!(after[2].1, after[3].1);
        assert_eq!(pool.meet(ids[live[0]], ALL, &arena), ids[live[0]]);
    }
    #[test]
    fn sharing_preserves_raw_normalization_boundaries() {
        let mut arena = Arena::default();
        let a = arena.atom("a");
        let pool = Sets::default();
        let raw = pool.intern(vec![Value::Patient, Value::Exact(a)]);
        assert_eq!(pool.len(raw), 2);
        assert_eq!(pool.meet(raw, ALL, &arena), pool.one(Value::Patient));
        assert_eq!(pool.intern(vec![Value::Exact(a), Value::Patient]), raw);
    }
}
