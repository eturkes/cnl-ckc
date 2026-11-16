use super::sets::Sets;
use super::{ALL, EMPTY, Result, SetId, bad, hash, id};
use sb1_checker::term::{Arena, Id, Node};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct RawValue {
    pub kind: u8,
    pub at: usize,
}
#[derive(Clone, Copy)]
struct Value {
    kind: u8,
    at: usize,
}
pub struct Abs {
    pub terms: Vec<u32>,
    pub origins: Vec<[Id; 4]>,
    pub origin_values: Vec<u32>,
    origin_buckets: HashMap<u64, Vec<u32>>,
    values: Vec<Value>,
}
impl Abs {
    pub const ANY: u32 = 0;
    pub const NUMBER: u32 = 1;
    pub const PATIENT: u32 = 2;
    pub fn new(arena: &Arena) -> Result<Self> {
        let mut abs = Self {
            terms: Vec::new(),
            origins: Vec::new(),
            origin_values: Vec::new(),
            origin_buckets: HashMap::new(),
            values: vec![
                Value { kind: 0, at: 0 },
                Value { kind: 1, at: 0 },
                Value { kind: 2, at: 0 },
            ],
        };
        abs.extend(arena)?;
        Ok(abs)
    }
    pub fn extend(&mut self, arena: &Arena) -> Result<()> {
        for t in self.terms.len()..arena.nodes.len() {
            let v = if matches!(arena.node(t), Node::Var(_)) {
                Self::ANY
            } else if let Some(a) = arena.named(t, "$guideline_id", 5) {
                let key = [a[0], a[1], a[2], a[3]];
                if let Some(o) = self.origin_key(key) {
                    self.origin_values[o]
                } else {
                    let o = id(self.origins.len())?;
                    let h = hash(&key.map(|x| x as u32));
                    let v = id(self.values.len())?;
                    self.values.push(Value {
                        kind: 6,
                        at: o as usize,
                    });
                    self.origins.push(key);
                    self.origin_values.push(v);
                    self.origin_buckets.entry(h).or_default().push(o);
                    v
                }
            } else {
                let kind = if matches!(arena.node(t), Node::Integer(_))
                    || arena.named(t, "rational", 2).is_some()
                {
                    4
                } else if matches!(arena.node(t), Node::Atom(_)) {
                    3
                } else {
                    5
                };
                let v = id(self.values.len())?;
                self.values.push(Value { kind, at: t });
                v
            };
            self.terms.push(v);
        }
        Ok(())
    }
    pub fn checkpoint(&self) -> (usize, usize, usize) {
        (self.terms.len(), self.values.len(), self.origins.len())
    }
    pub fn rewind(&mut self, checkpoint: (usize, usize, usize)) {
        // Query rollback visits appended origins alone; the prepared KB's
        // term vector + origin buckets stay untouched.
        while self.origins.len() > checkpoint.2 {
            let index = (self.origins.len() - 1) as u32;
            let key = self.origins.pop().unwrap();
            self.origin_values.pop();
            let h = hash(&key.map(|x| x as u32));
            let bucket = self.origin_buckets.get_mut(&h).unwrap();
            assert_eq!(bucket.pop(), Some(index));
            if bucket.is_empty() {
                self.origin_buckets.remove(&h);
            }
        }
        self.terms.truncate(checkpoint.0);
        self.values.truncate(checkpoint.1);
    }
    pub fn origin_key(&self, key: [Id; 4]) -> Option<usize> {
        self.origin_buckets
            .get(&hash(&key.map(|x| x as u32)))
            .and_then(|ids| {
                ids.iter()
                    .find(|o| self.origins[**o as usize] == key)
                    .map(|o| *o as usize)
            })
    }
    pub fn origin(&self, value: u32) -> Option<usize> {
        let v = self.values[value as usize];
        (v.kind == 6).then_some(v.at)
    }
    pub fn exact(&self, value: u32) -> Option<Id> {
        let v = self.values[value as usize];
        matches!(v.kind, 3..=5).then_some(v.at)
    }
    pub fn numeric(&self, value: u32) -> bool {
        self.values[value as usize].kind == 4
    }
    pub fn atomic(&self, value: u32) -> bool {
        matches!(self.values[value as usize].kind, 3 | 4)
    }
    pub fn intersect(&self, a: u32, b: u32) -> Option<u32> {
        if a == b {
            return Some(a);
        }
        if a == Self::ANY {
            return Some(b);
        }
        if b == Self::ANY {
            return Some(a);
        }
        if a == Self::PATIENT && (b == Self::NUMBER || self.atomic(b)) {
            return Some(b);
        }
        if b == Self::PATIENT && (a == Self::NUMBER || self.atomic(a)) {
            return Some(a);
        }
        if a == Self::NUMBER && self.numeric(b) {
            return Some(b);
        }
        if b == Self::NUMBER && self.numeric(a) {
            return Some(a);
        }
        None
    }
    pub fn decode(&self, value: RawValue, origin_map: &[u32]) -> Result<u32> {
        match value.kind {
            0 => Ok(Self::ANY),
            1 => Ok(Self::NUMBER),
            2 => Ok(Self::PATIENT),
            3 => self
                .terms
                .get(value.at)
                .copied()
                .filter(|v| self.exact(*v).is_some())
                .ok_or_else(|| bad("invalid exact feature value")),
            4 => origin_map
                .get(value.at)
                .copied()
                .ok_or_else(|| bad("invalid feature origin")),
            _ => Err(bad("unknown feature-value kind")),
        }
    }
    pub fn describe(&self, arena: &Arena, v: u32) -> String {
        match v {
            Self::ANY => "any:any".into(),
            Self::NUMBER => "number:number".into(),
            Self::PATIENT => "patient:patient(all-features)".into(),
            _ => {
                if let Some(o) = self.origin(v) {
                    let k = self.origins[o];
                    format!(
                        "origin:origin({},{},{},{})",
                        arena.render(k[0]),
                        arena.render(k[1]),
                        arena.render(k[2]),
                        arena.render(k[3])
                    )
                } else {
                    format!("exact:{}", arena.render(self.exact(v).unwrap()))
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Feature {
    pub kind: u8,
    pub predicate: u32,
    pub selector: u32,
}
impl Feature {
    fn words(self) -> [u32; 3] {
        [u32::from(self.kind), self.predicate, self.selector]
    }
}
#[derive(Clone, Copy)]
pub struct Rect {
    pub predicate: u32,
    pub selector: u32,
    pub owner: SetId,
    pub source: SetId,
    pub target: SetId,
}
#[derive(Clone, Copy)]
struct Unary {
    feature: Feature,
    set: SetId,
}
#[derive(Clone, Copy)]
struct Family {
    kind: u8,
    predicate: u32,
    set: SetId,
}
#[derive(Clone, Copy)]
struct Support {
    key: [u32; 5],
    value: (SetId, SetId, SetId),
}
pub struct May {
    pub abs: Abs,
    pub sets: Sets,
    pub generic: bool,
    pub input_sets: Vec<SetId>,
    pub input_origins: Vec<u32>,
    unary: Vec<Unary>,
    unary_index: HashMap<u64, Vec<u32>>,
    families: Vec<Family>,
    family_index: HashMap<u64, Vec<u32>>,
    pub rectangles: Vec<Rect>,
    rect_sources: HashMap<u64, Vec<u32>>,
    rect_atomic: HashMap<u32, Vec<u32>>,
    rect_numeric: HashMap<u32, Vec<u32>>,
    pub predicates: Vec<u32>,
    supports: RefCell<HashMap<u64, Vec<Support>>>,
}
impl May {
    pub fn begin(arena: &Arena, generic: bool, origins: &[[Id; 4]]) -> Result<Self> {
        let abs = Abs::new(arena)?;
        let mut seen = vec![false; abs.origins.len()];
        let mut input_origins = Vec::new();
        for key in origins {
            let Some(o) = abs.origin_key(*key) else {
                return Err(bad("hint origin is not a source origin"));
            };
            if seen[o] {
                return Err(bad("duplicate hint origin"));
            }
            seen[o] = true;
            input_origins.push(abs.origin_values[o]);
        }
        if seen.iter().any(|v| !v) {
            return Err(bad("hint omits a source origin"));
        }
        Ok(Self {
            abs,
            sets: Sets::default(),
            generic,
            input_sets: Vec::new(),
            input_origins,
            unary: Vec::new(),
            unary_index: HashMap::new(),
            families: Vec::new(),
            family_index: HashMap::new(),
            rectangles: Vec::new(),
            rect_sources: HashMap::new(),
            rect_atomic: HashMap::new(),
            rect_numeric: HashMap::new(),
            predicates: Vec::new(),
            supports: RefCell::new(HashMap::new()),
        })
    }
    pub fn add_set(&mut self, values: Option<impl Iterator<Item = RawValue>>) -> Result<()> {
        let set = match values {
            None => ALL,
            Some(values) => {
                let mut v = values
                    .map(|v| self.abs.decode(v, &self.input_origins))
                    .collect::<Result<Vec<_>>>()?;
                let before = v.len();
                v.sort_unstable();
                v.dedup();
                if before != v.len() {
                    return Err(bad("duplicate hinted set member"));
                }
                self.sets.intern(v)
            }
        };
        self.input_sets.push(set);
        Ok(())
    }
    pub fn add_unary(
        &mut self,
        kind: u8,
        predicate: u32,
        selector: RawValue,
        set: usize,
    ) -> Result<()> {
        if kind > 5 {
            return Err(bad("unknown unary feature kind"));
        }
        let feature = Feature {
            kind,
            predicate,
            selector: self.abs.decode(selector, &self.input_origins)?,
        };
        if self.raw_feature(feature).is_some() {
            return Err(bad("duplicate unary feature row"));
        }
        let set = *self
            .input_sets
            .get(set)
            .ok_or_else(|| bad("unary set index"))?;
        if set == ALL {
            return Err(bad("unary row must use finite raw keys"));
        }
        let index = id(self.unary.len())?;
        self.unary.push(Unary { feature, set });
        self.unary_index
            .entry(hash(&feature.words()))
            .or_default()
            .push(index);
        Ok(())
    }
    pub fn add_rect(
        &mut self,
        predicate: u32,
        selector: RawValue,
        owner: usize,
        source: usize,
        target: usize,
    ) -> Result<()> {
        let set = |i| {
            self.input_sets
                .get(i)
                .copied()
                .ok_or_else(|| bad("rectangle set index"))
        };
        let r = Rect {
            predicate,
            selector: self.abs.decode(selector, &self.input_origins)?,
            owner: set(owner)?,
            source: set(source)?,
            target: set(target)?,
        };
        if r.owner == ALL || r.source == ALL {
            return Err(bad("rectangle axes require finite raw keys"));
        }
        let at = id(self.rectangles.len())?;
        let source = self.sets.values(r.source).unwrap();
        let mut atomic = false;
        let mut numeric = false;
        for v in source.iter() {
            self.rect_sources
                .entry(hash(&[predicate, *v]))
                .or_default()
                .push(at);
            atomic |= self.abs.atomic(*v);
            numeric |= self.abs.numeric(*v);
        }
        if atomic {
            self.rect_atomic.entry(predicate).or_default().push(at);
        }
        if numeric {
            self.rect_numeric.entry(predicate).or_default().push(at);
        }
        self.rectangles.push(r);
        Ok(())
    }
    pub fn finish(&mut self, arena: &Arena, predicates: Vec<u32>) -> Result<()> {
        self.predicates = predicates;
        self.predicates.sort_unstable();
        self.predicates.dedup();
        self.families.clear();
        self.family_index.clear();
        let mut members = Vec::<Vec<u32>>::new();
        for row in &self.unary {
            if row.feature.kind == 4 && !self.predicates.contains(&row.feature.predicate) {
                return Err(bad("payload predicate not declared"));
            }
            let key = [u32::from(row.feature.kind), row.feature.predicate];
            let h = hash(&key);
            let old = self
                .family_index
                .get(&h)
                .and_then(|ids| {
                    ids.iter().find(|i| {
                        let f = self.families[**i as usize];
                        f.kind == row.feature.kind && f.predicate == row.feature.predicate
                    })
                })
                .copied();
            let i = if let Some(i) = old {
                i
            } else {
                let i = id(self.families.len())?;
                self.families.push(Family {
                    kind: row.feature.kind,
                    predicate: row.feature.predicate,
                    set: EMPTY,
                });
                self.family_index.entry(h).or_default().push(i);
                members.push(Vec::new());
                i
            };
            members[i as usize].extend_from_slice(&self.sets.values(row.set).unwrap());
        }
        // Freeze one union per family; retained prefix versions would charge
        // the sum of every growing family prefix, not its final contents.
        for (family, members) in self.families.iter_mut().zip(members) {
            family.set = self.sets.intern(members);
        }
        let mut link_predicates = arena
            .nodes
            .iter()
            .filter_map(|n| match n.as_ref() {
                Node::Compound(p, _)
                    if matches!(arena.name(*p), "guideline_arg" | "guideline_pp") =>
                {
                    Some(*p as u32)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        link_predicates.sort_unstable();
        link_predicates.dedup();
        if self
            .rectangles
            .iter()
            .any(|r| link_predicates.binary_search(&r.predicate).is_err())
        {
            return Err(bad("nonlink/unknown rectangle predicate"));
        }
        self.sets.freeze();
        Ok(())
    }
    fn raw_feature(&self, feature: Feature) -> Option<SetId> {
        self.unary_index
            .get(&hash(&feature.words()))
            .and_then(|ids| {
                ids.iter().find_map(|i| {
                    let row = self.unary[*i as usize];
                    (row.feature == feature).then_some(row.set)
                })
            })
    }
    fn family(&self, feature: Feature) -> SetId {
        self.family_index
            .get(&hash(&[u32::from(feature.kind), feature.predicate]))
            .and_then(|ids| {
                ids.iter().find_map(|i| {
                    let f = self.families[*i as usize];
                    (f.kind == feature.kind && f.predicate == feature.predicate).then_some(f.set)
                })
            })
            .unwrap_or(EMPTY)
    }
    fn rows(&self, f: Feature) -> [SetId; 2] {
        if f.selector == Abs::ANY {
            [self.family(f), EMPTY]
        } else {
            [
                self.raw_feature(f).unwrap_or(EMPTY),
                self.raw_feature(Feature {
                    selector: Abs::ANY,
                    ..f
                })
                .unwrap_or(EMPTY),
            ]
        }
    }
    fn tag(&self, arena: &Arena, f: Feature) -> Option<SetId> {
        if f.kind != 5 {
            return None;
        }
        let t = self.abs.exact(f.selector)?;
        if arena.atom_name(t) == Some("$sb1_base") {
            Some(self.sets.one(Abs::PATIENT))
        } else if arena.named(t, "$sb1_arith", 2).is_some() {
            Some(self.sets.one(Abs::NUMBER))
        } else {
            None
        }
    }
    pub fn top(&self, v: u32) -> bool {
        v == Abs::ANY
            || self.generic && (v == Abs::PATIENT || v == Abs::NUMBER || self.abs.atomic(v))
    }
    pub fn has(&self, arena: &Arena, v: u32, f: Feature) -> bool {
        if let Some(tag) = self.tag(arena, f) {
            return self.sets.meet(self.sets.one(v), tag, &self.abs) != EMPTY;
        }
        if f.kind != 5 && self.top(v) {
            return true;
        }
        self.rows(f)
            .iter()
            .any(|set| self.sets.has(*set, v) || self.sets.has(*set, Abs::ANY))
    }
    pub fn sources(&self, arena: &Arena, f: Feature) -> SetId {
        if let Some(tag) = self.tag(arena, f) {
            return tag;
        }
        let mut out = EMPTY;
        for set in self.rows(f) {
            if self.sets.has(set, Abs::ANY) {
                return ALL;
            }
            out = self.sets.union(out, set);
        }
        if self.generic && f.kind != 5 {
            out = self.sets.union(out, self.sets.one(Abs::PATIENT));
        }
        out
    }
    pub fn unary_width(&self, f: Feature) -> usize {
        self.rows(f)
            .iter()
            .map(|s| self.sets.len(*s))
            .fold(0, usize::saturating_add)
    }
    fn source_rows(&self, predicate: u32, v: u32, out: &mut Vec<u32>) {
        if let Some(rows) = self.rect_sources.get(&hash(&[predicate, v])) {
            out.extend(rows.iter().filter(|i| {
                let r = self.rectangles[**i as usize];
                r.predicate == predicate && self.sets.has(r.source, v)
            }));
        }
    }
    fn candidates(&self, predicate: u32, source: SetId) -> Vec<u32> {
        let mut out = Vec::new();
        self.source_rows(predicate, Abs::ANY, &mut out);
        for v in self.sets.values(source).expect("finite link source").iter() {
            self.source_rows(predicate, *v, &mut out);
            for broad in [Abs::PATIENT, Abs::NUMBER] {
                if self.abs.intersect(*v, broad).is_some() {
                    self.source_rows(predicate, broad, &mut out);
                }
            }
            if *v == Abs::PATIENT
                && let Some(rows) = self.rect_atomic.get(&predicate)
            {
                out.extend(rows);
            }
            if *v == Abs::NUMBER
                && let Some(rows) = self.rect_numeric.get(&predicate)
            {
                out.extend(rows);
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }
    pub fn support(
        &self,
        predicate: u32,
        selector: u32,
        owner: SetId,
        source: SetId,
        target: SetId,
    ) -> (SetId, SetId, SetId) {
        let key = [predicate, selector, owner, source, target];
        let h = hash(&key);
        if let Some(rows) = self.supports.borrow().get(&h)
            && let Some(row) = rows.iter().find(|r| r.key == key)
        {
            return row.value;
        }
        let owners = self.sets.raw_keys(owner);
        let (mut ao, mut asrc, mut at) = (EMPTY, EMPTY, EMPTY);
        if self.sets.has(source, Abs::ANY) {
            ao = owner;
            asrc = self.sets.raw_one(Abs::ANY);
            at = target;
        }
        let top = |set| {
            self.sets.intern(
                self.sets
                    .values(set)
                    .unwrap()
                    .iter()
                    .copied()
                    .filter(|v| self.top(*v))
                    .collect(),
            )
        };
        let (top_o, top_s) = (top(owners), top(source));
        let patient = self
            .sets
            .meet(self.sets.one(Abs::PATIENT), target, &self.abs);
        if top_o != EMPTY && top_s != EMPTY && patient != EMPTY {
            ao = self.sets.union(
                ao,
                if self.sets.has(top_o, Abs::ANY) {
                    ALL
                } else {
                    top_o
                },
            );
            asrc = self.sets.union(asrc, top_s);
            at = self.sets.union(at, patient);
        }
        for i in self.candidates(predicate, source) {
            let r = self.rectangles[i as usize];
            if selector != Abs::ANY && r.selector != Abs::ANY && r.selector != selector {
                continue;
            }
            let s = self.sets.compatible(source, r.source, &self.abs);
            if s == EMPTY {
                continue;
            }
            let t = self.sets.meet(r.target, target, &self.abs);
            if t == EMPTY {
                continue;
            }
            let o = self.sets.compatible(owners, r.owner, &self.abs);
            if o == EMPTY {
                continue;
            }
            asrc = self.sets.union(asrc, s);
            at = self.sets.union(at, t);
            ao = self
                .sets
                .union(ao, if self.sets.has(o, Abs::ANY) { ALL } else { o });
        }
        let value = (ao, asrc, self.sets.normalize(at, &self.abs));
        self.supports
            .borrow_mut()
            .entry(h)
            .or_default()
            .push(Support { key, value });
        value
    }
    pub fn require_unary(&self, feature: Feature, contribution: SetId) -> Result<()> {
        let stored = self.raw_feature(feature).unwrap_or(EMPTY);
        let raw = self.sets.raw_keys(contribution);
        if self.sets.subset_raw(raw, stored) {
            Ok(())
        } else {
            Err(bad(format!("missing seed/transfer unary {:?}", feature)))
        }
    }
    pub fn require_rect(
        &self,
        predicate: u32,
        selector: u32,
        owner: SetId,
        source: SetId,
        target: SetId,
    ) -> Result<()> {
        let owners = self.sets.values(self.sets.raw_keys(owner)).unwrap();
        let sources = self.sets.values(self.sets.raw_keys(source)).unwrap();
        for s in sources.iter() {
            let mut rows = Vec::new();
            self.source_rows(predicate, *s, &mut rows);
            for o in owners.iter() {
                let mut stored = EMPTY;
                for i in &rows {
                    let r = self.rectangles[*i as usize];
                    if r.selector == selector && self.sets.has(r.owner, *o) {
                        stored = self.sets.union(stored, r.target);
                    }
                }
                if !self.sets.subset_raw(target, stored) {
                    return Err(bad(format!(
                        "missing seed/transfer link {predicate}:{selector} owner={o} source={s}"
                    )));
                }
            }
        }
        Ok(())
    }
    pub fn unary_rows(&self) -> impl Iterator<Item = (Feature, SetId)> + '_ {
        self.unary.iter().map(|row| (row.feature, row.set))
    }
    pub fn reset_scratch(&self) {
        self.supports.borrow_mut().clear();
        self.sets.reset_scratch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn family_finalization_keeps_only_input_roots_and_the_final_family() {
        let mut arena = Arena::default();
        let selectors = (0..100)
            .map(|i| arena.atom(&format!("noun{i}")))
            .collect::<Vec<_>>();
        let values = (0..100)
            .map(|i| arena.atom(&format!("value{i}")))
            .collect::<Vec<_>>();
        let mut m = May::begin(&arena, true, &[]).unwrap();
        for (i, (selector, value)) in selectors.iter().zip(&values).enumerate() {
            m.add_set(Some(std::iter::once(RawValue {
                kind: 3,
                at: *value,
            })))
            .unwrap();
            m.add_unary(
                0,
                0,
                RawValue {
                    kind: 3,
                    at: *selector,
                },
                i,
            )
            .unwrap();
        }
        m.finish(&arena, Vec::new()).unwrap();
        let family = m.family(Feature {
            kind: 0,
            predicate: 0,
            selector: Abs::ANY,
        });
        let mut expected = values.iter().map(|t| m.abs.terms[*t]).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(&*m.sets.values(family).unwrap(), &expected);
        assert_eq!(
            m.sets.retained_cells(),
            103,
            "2 reserved + 100 input sets + one family; obsolete prefix unions are scratch"
        );
    }
}
