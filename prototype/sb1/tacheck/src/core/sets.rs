use super::{Abs, hash};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type SetId = u32;
pub const ALL: SetId = 0;
pub const EMPTY: SetId = 1;
#[derive(Default)]
struct Store {
    values: Vec<Rc<[u32]>>,
    buckets: HashMap<u64, Vec<u32>>,
    meets: HashMap<u64, Vec<(u32, u32, u32)>>,
    unions: HashMap<u64, Vec<(u32, u32, u32)>>,
    retained: usize,
}
pub struct Sets(RefCell<Store>);
impl Default for Sets {
    fn default() -> Self {
        Self(RefCell::new(Store {
            values: vec![Rc::from([]), Rc::from([])],
            buckets: HashMap::from([(hash(&[]), vec![EMPTY])]),
            ..Store::default()
        }))
    }
}
impl Sets {
    pub fn intern(&self, mut values: Vec<u32>) -> SetId {
        values.sort_unstable();
        values.dedup();
        let h = hash(&values);
        let mut s = self.0.borrow_mut();
        if let Some(ids) = s.buckets.get(&h) {
            for i in ids {
                if s.values[*i as usize].as_ref() == values {
                    return *i;
                }
            }
        }
        let i =
            u32::try_from(s.values.len()).expect("set id capacity checked by construction fence");
        s.values.push(Rc::from(values));
        s.buckets.entry(h).or_default().push(i);
        i
    }
    pub fn one(&self, v: u32) -> SetId {
        if v == Abs::ANY {
            ALL
        } else {
            self.intern(vec![v])
        }
    }
    pub fn raw_one(&self, v: u32) -> SetId {
        self.intern(vec![v])
    }
    pub fn values(&self, set: SetId) -> Option<Rc<[u32]>> {
        (set != ALL).then(|| self.0.borrow().values[set as usize].clone())
    }
    pub fn len(&self, set: SetId) -> usize {
        self.values(set).map_or(usize::MAX, |v| v.len())
    }
    pub fn has(&self, set: SetId, value: u32) -> bool {
        self.values(set)
            .is_none_or(|v| v.binary_search(&value).is_ok())
    }
    pub fn raw_keys(&self, set: SetId) -> SetId {
        if set == ALL {
            self.raw_one(Abs::ANY)
        } else {
            set
        }
    }
    pub fn normalize(&self, set: SetId, abs: &Abs) -> SetId {
        let Some(v) = self.values(set) else {
            return ALL;
        };
        let patient = v.binary_search(&Abs::PATIENT).is_ok();
        let number = v.binary_search(&Abs::NUMBER).is_ok();
        if !patient && !number {
            return set;
        }
        self.intern(
            v.iter()
                .copied()
                .filter(|x| {
                    !(*x == Abs::NUMBER && patient
                        || abs.atomic(*x) && patient
                        || abs.numeric(*x) && number)
                })
                .collect(),
        )
    }
    pub fn union(&self, a: SetId, b: SetId) -> SetId {
        if a == ALL || b == ALL {
            return ALL;
        }
        if a == b || b == EMPTY {
            return a;
        }
        if a == EMPTY {
            return b;
        }
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        let h = hash(&[a, b]);
        if let Some(rows) = self.0.borrow().unions.get(&h)
            && let Some((_, _, set)) = rows.iter().find(|(x, y, _)| *x == a && *y == b)
        {
            return *set;
        }
        let (av, bv) = (self.values(a).unwrap(), self.values(b).unwrap());
        let mut out = Vec::with_capacity(av.len() + bv.len());
        let (mut i, mut j) = (0, 0);
        while i < av.len() || j < bv.len() {
            if j == bv.len() || i < av.len() && av[i] < bv[j] {
                out.push(av[i]);
                i += 1;
            } else if i == av.len() || bv[j] < av[i] {
                out.push(bv[j]);
                j += 1;
            } else {
                out.push(av[i]);
                i += 1;
                j += 1;
            }
        }
        let set = self.intern(out);
        self.0
            .borrow_mut()
            .unions
            .entry(h)
            .or_default()
            .push((a, b, set));
        set
    }
    pub fn meet(&self, a: SetId, b: SetId, abs: &Abs) -> SetId {
        if a == ALL {
            return self.normalize(b, abs);
        }
        if b == ALL {
            return self.normalize(a, abs);
        }
        if a == EMPTY || b == EMPTY {
            return EMPTY;
        }
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        let h = hash(&[a, b]);
        if let Some(rows) = self.0.borrow().meets.get(&h)
            && let Some((_, _, set)) = rows.iter().find(|(x, y, _)| *x == a && *y == b)
        {
            return *set;
        }
        let (av, bv) = (self.values(a).unwrap(), self.values(b).unwrap());
        let mut out = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < av.len() && j < bv.len() {
            if av[i] < bv[j] {
                i += 1;
            } else if bv[j] < av[i] {
                j += 1;
            } else {
                out.push(av[i]);
                i += 1;
                j += 1;
            }
        }
        // Finite Any is a raw writer key here, not Domain::All. Only these
        // two semantic scalar classes broaden a meet.
        for wild in [Abs::PATIENT, Abs::NUMBER] {
            if av.binary_search(&wild).is_ok() {
                out.extend(bv.iter().filter_map(|v| abs.intersect(wild, *v)));
            }
            if bv.binary_search(&wild).is_ok() {
                out.extend(av.iter().filter_map(|v| abs.intersect(*v, wild)));
            }
        }
        let set = self.normalize(self.intern(out), abs);
        self.0
            .borrow_mut()
            .meets
            .entry(h)
            .or_default()
            .push((a, b, set));
        set
    }
    pub fn compatible(&self, a: SetId, b: SetId, abs: &Abs) -> SetId {
        if a == ALL {
            return if b == EMPTY { EMPTY } else { ALL };
        }
        if b == ALL {
            return a;
        }
        let (av, bv) = (self.values(a).unwrap(), self.values(b).unwrap());
        if bv.binary_search(&Abs::ANY).is_ok() {
            return a;
        }
        let patient = bv.binary_search(&Abs::PATIENT).is_ok();
        let number = bv.binary_search(&Abs::NUMBER).is_ok();
        let numeric = bv.iter().any(|v| abs.numeric(*v));
        let atomic = bv.iter().any(|v| abs.atomic(*v));
        self.intern(
            av.iter()
                .copied()
                .filter(|v| {
                    bv.binary_search(v).is_ok()
                        || match *v {
                            Abs::ANY => !bv.is_empty(),
                            Abs::PATIENT => patient || number || atomic,
                            Abs::NUMBER => patient || number || numeric,
                            _ => patient && abs.atomic(*v) || number && abs.numeric(*v),
                        }
                })
                .collect(),
        )
    }
    pub fn subset_raw(&self, a: SetId, b: SetId) -> bool {
        if b == ALL {
            return true;
        }
        if a == ALL {
            return false;
        }
        let b = self.values(b).unwrap();
        self.values(a)
            .unwrap()
            .iter()
            .all(|v| b.binary_search(v).is_ok())
    }
    pub fn freeze(&self) {
        let mut s = self.0.borrow_mut();
        s.retained = s.values.len();
    }
    pub fn reset_scratch(&self) {
        let mut s = self.0.borrow_mut();
        let retained = s.retained;
        if retained == 0 {
            return;
        }
        s.meets.clear();
        s.unions.clear();
        // Only appended cells can die. Scanning all retained F buckets per
        // source clause would make this boundary quadratic in the KB size.
        while s.values.len() > retained {
            let index = (s.values.len() - 1) as u32;
            let values = s.values.pop().unwrap();
            let h = hash(&values);
            let bucket = s.buckets.get_mut(&h).unwrap();
            assert_eq!(bucket.pop(), Some(index));
            if bucket.is_empty() {
                s.buckets.remove(&h);
            }
        }
    }
    pub fn retained_cells(&self) -> usize {
        self.0.borrow().retained
    }
    pub fn payload_bytes(&self) -> usize {
        let s = self.0.borrow();
        s.values.iter().map(|v| v.len() * 4).sum::<usize>()
            + s.values.capacity() * std::mem::size_of::<Rc<[u32]>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scratch_reset_preserves_every_root_and_drops_appended_ids_and_cached_results() {
        let sets = Sets::default();
        let roots = (3..2003).map(|v| sets.intern(vec![v])).collect::<Vec<_>>();
        sets.freeze();
        let retained = sets.0.borrow().values.len();
        for round in 0..3 {
            for i in 0..500 {
                sets.intern(vec![5000 + round, i]);
            }
            sets.union(roots[0], roots[1]);
            assert!(sets.0.borrow().values.len() > retained);
            sets.reset_scratch();
            let store = sets.0.borrow();
            assert_eq!(store.values.len(), retained);
            assert!(store.unions.is_empty() && store.meets.is_empty());
            assert!(
                store
                    .buckets
                    .values()
                    .flatten()
                    .all(|i| (*i as usize) < retained)
            );
            drop(store);
            for (i, root) in roots.iter().enumerate() {
                assert_eq!(&*sets.values(*root).unwrap(), &[i as u32 + 3]);
            }
        }
    }
}
