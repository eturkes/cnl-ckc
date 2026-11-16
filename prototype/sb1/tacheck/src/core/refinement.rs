use super::growth::{Sources, TypeNode, Types};
use super::matching::{Expr, solve, state, witness};
use super::{Abs, Budget, May, Pattern, Program, Result, State, bad, hash};
use sb1_checker::program::Goal;
use sb1_checker::term::{Arena, Id, Node};
use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Tag {
    Scalar,
    Origin(usize),
    Arithmetic(usize, usize),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key {
    pub origin: usize,
    pub precise: bool,
    pub deps: Vec<Tag>,
}
#[derive(Clone, Debug)]
pub struct Variant {
    pub original: Option<usize>,
    pub tags: Vec<(usize, Tag)>,
}
pub struct Registry {
    pub keys: Vec<Key>,
    pub variants: Vec<Variant>,
    pub origin_keys: Vec<usize>,
    pub precise: Vec<bool>,
    pub full_refinement: bool,
    by_original: Vec<Vec<usize>>,
}
fn integer(arena: &Arena, term: Id) -> Option<usize> {
    match arena.node(term) {
        Node::Integer(s) => s.parse().ok(),
        _ => None,
    }
}
pub fn same(a: &Arena, x: Id, b: &Arena, y: Id) -> bool {
    match (a.node(x), b.node(y)) {
        (Node::Var(x), Node::Var(y)) => x == y,
        (Node::Atom(x), Node::Atom(y)) => a.name(*x) == b.name(*y),
        (Node::Integer(x), Node::Integer(y)) => x == y,
        (Node::Nil, Node::Nil) => true,
        (Node::Compound(x, xs), Node::Compound(y, ys)) => {
            a.name(*x) == b.name(*y)
                && xs.len() == ys.len()
                && xs.iter().zip(ys).all(|(x, y)| same(a, *x, b, *y))
        }
        _ => false,
    }
}
fn head_terms(p: &Program, term: Id, out: &mut Vec<Id>) {
    if p.arena.named(term, "$guideline_id", 5).is_some() {
        out.push(term);
    } else if let Some((_, a)) = p.arena.call(term) {
        for t in a {
            head_terms(p, *t, out);
        }
    }
}
impl Registry {
    pub fn new(
        base: &Program,
        bm: &May,
        q: &Program,
        qm: &May,
        keys: Vec<Key>,
        variants: Vec<Variant>,
        full: bool,
    ) -> Result<Self> {
        if variants.len() != q.clauses.len() {
            return Err(bad("k1 clause-variant length"));
        }
        let mut precise = vec![None; bm.abs.origins.len()];
        let mut buckets = HashMap::<u64, Vec<usize>>::new();
        for (i, key) in keys.iter().enumerate() {
            if key.origin >= precise.len() {
                return Err(bad("k1 template outside source origins"));
            }
            if let Some(old) = precise[key.origin]
                && old != key.precise
            {
                return Err(bad("inconsistent per-symbol typing"));
            }
            precise[key.origin] = Some(key.precise);
            if !key.precise && !key.deps.is_empty() {
                return Err(bad("base type carries a tag tuple"));
            }
            for tag in &key.deps {
                Self::valid_tag(base, bm, tag)?;
            }
            let mut words = vec![key.origin as u32, u32::from(key.precise)];
            for tag in &key.deps {
                match tag {
                    Tag::Scalar => words.extend([0, 0, 0]),
                    Tag::Origin(o) => words.extend([1, *o as u32, 0]),
                    Tag::Arithmetic(c, g) => words.extend([2, *c as u32, *g as u32]),
                }
            }
            let h = hash(&words);
            if buckets
                .get(&h)
                .is_some_and(|rows| rows.iter().any(|j| keys[*j] == *key))
            {
                return Err(bad("duplicate semantic k1 registry key"));
            }
            buckets.entry(h).or_default().push(i);
        }
        if full && precise.iter().flatten().any(|p| !p) {
            return Err(bad("full refinement contains a base-typed symbol"));
        }
        let precise = precise
            .into_iter()
            .map(|p| p.unwrap_or(full))
            .collect::<Vec<_>>();
        let mut seen = vec![false; keys.len()];
        let mut origin_keys = Vec::new();
        for parts in &qm.abs.origins {
            let i = q
                .arena
                .named(parts[1], "$sb1_k1", 1)
                .and_then(|a| integer(&q.arena, a[0]))
                .ok_or_else(|| bad("k1 origin namespace"))?;
            let key = keys
                .get(i)
                .ok_or_else(|| bad("k1 origin references absent registry key"))?;
            if seen[i] {
                return Err(bad("registry key has duplicate semantic origins"));
            }
            seen[i] = true;
            let original = bm.abs.origins[key.origin];
            for at in [0, 2, 3] {
                if !same(&base.arena, original[at], &q.arena, parts[at]) {
                    return Err(bad("k1 origin changes constructor kind/sentence/slot"));
                }
            }
            origin_keys.push(i);
        }
        if seen.iter().any(|v| !v) {
            return Err(bad("registry has an unrealized semantic type"));
        }
        let mut by_original = vec![Vec::new(); base.clauses.len()];
        for (i, variant) in variants.iter().enumerate() {
            if let Some(original) = variant.original {
                if original >= base.clauses.len() || base.patient[original] {
                    return Err(bad("k1 original clause index"));
                }
                by_original[original].push(i);
            }
        }
        Ok(Self {
            keys,
            variants,
            origin_keys,
            precise,
            full_refinement: full,
            by_original,
        })
    }
    fn valid_tag(p: &Program, m: &May, tag: &Tag) -> Result<()> {
        match tag {
            Tag::Scalar => Ok(()),
            Tag::Origin(o) if *o < m.abs.origins.len() => Ok(()),
            Tag::Arithmetic(c, g) => {
                let valid =
                    p.clauses
                        .get(*c)
                        .and_then(|c| c.body.get(*g))
                        .is_some_and(|g| match g {
                            Goal::Builtin(t) => p.arena.call(*t).is_some_and(|(name, _)| {
                                matches!(
                                    p.arena.name(name),
                                    "guideline_product" | "guideline_quotient"
                                )
                            }),
                            _ => false,
                        });
                if valid {
                    Ok(())
                } else {
                    Err(bad("k1 arithmetic tag is not an arithmetic site"))
                }
            }
            _ => Err(bad("invalid k1 tag origin")),
        }
    }
    fn decode_tag(base: &May, q: &Program, term: Id) -> Result<Tag> {
        if q.arena.atom_name(term) == Some("$sb1_base") {
            return Ok(Tag::Scalar);
        }
        if let Some(a) = q.arena.named(term, "$sb1_template", 1) {
            let old = integer(&q.arena, a[0]).ok_or_else(|| bad("k1 template tag index"))?;
            let value = *base
                .input_origins
                .get(old)
                .ok_or_else(|| bad("k1 template tag outside source origin map"))?;
            return Ok(Tag::Origin(base.abs.origin(value).unwrap()));
        }
        if let Some(a) = q.arena.named(term, "$sb1_arith", 2) {
            return Ok(Tag::Arithmetic(
                integer(&q.arena, a[0]).ok_or_else(|| bad("k1 arithmetic tag clause"))?,
                integer(&q.arena, a[1]).ok_or_else(|| bad("k1 arithmetic tag goal"))?,
            ));
        }
        Err(bad("unknown k1 tag literal"))
    }
    fn key<'a>(&'a self, q: &Program, qm: &May, term: Id) -> Result<&'a Key> {
        let o = qm
            .abs
            .origin(qm.abs.terms[term])
            .ok_or_else(|| bad("k1 constructor is not an origin"))?;
        let key = &self.keys[self.origin_keys[o]];
        if q.arena.named(term, "$guideline_id", 5).is_none() {
            return Err(bad("k1 constructor shape"));
        }
        Ok(key)
    }
    fn term_eq(
        &self,
        source: (&Program, &May),
        x: Id,
        target: (&Program, &May),
        y: Id,
        tags: &[(usize, Tag)],
    ) -> Result<bool> {
        let (base, bm) = source;
        let (q, qm) = target;
        if let Some(a) = base.arena.named(x, "$guideline_id", 5) {
            let Some(b) = q.arena.named(y, "$guideline_id", 5) else {
                return Ok(false);
            };
            let key = self.key(q, qm, y)?;
            if Some(key.origin) != bm.abs.origin(bm.abs.terms[x]) {
                return Ok(false);
            }
            if ![0, 2, 3, 4]
                .iter()
                .all(|i| same(&base.arena, a[*i], &q.arena, b[*i]))
            {
                return Ok(false);
            }
            let deps = if key.precise {
                base.arena
                    .list(a[4])
                    .ok_or_else(|| bad("source constructor Deps"))?
                    .into_iter()
                    .map(|t| match base.arena.node(t) {
                        Node::Var(v) => tags
                            .iter()
                            .find(|(x, _)| x == v)
                            .map(|(_, t)| t.clone())
                            .ok_or_else(|| bad("missing k1 constructor-variable tag")),
                        _ => Err(bad("source constructor Deps is not variable")),
                    })
                    .collect::<Result<Vec<_>>>()?
            } else {
                Vec::new()
            };
            return Ok(key.deps == deps);
        }
        match (base.arena.node(x), q.arena.node(y)) {
            (Node::Compound(n, xs), Node::Compound(m, ys)) => {
                if base.arena.name(*n) != q.arena.name(*m) || xs.len() != ys.len() {
                    return Ok(false);
                }
                for (x, y) in xs.iter().zip(ys) {
                    if !self.term_eq((base, bm), *x, (q, qm), *y, tags)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            _ => Ok(same(&base.arena, x, &q.arena, y)),
        }
    }
    fn goals_eq(
        &self,
        source: (&Program, &May),
        xs: &[Goal],
        target: (&Program, &May),
        ys: &[Goal],
        tags: &[(usize, Tag)],
    ) -> Result<bool> {
        let (base, bm) = source;
        let (q, qm) = target;
        if xs.len() != ys.len() {
            return Ok(false);
        }
        for (x, y) in xs.iter().zip(ys) {
            let same = match (x, y) {
                (Goal::Positive(x), Goal::Positive(y)) | (Goal::Builtin(x), Goal::Builtin(y)) => {
                    self.term_eq((base, bm), *x, (q, qm), *y, tags)?
                }
                (Goal::Not(x), Goal::Not(y)) => self.goals_eq((base, bm), x, (q, qm), y, tags)?,
                _ => false,
            };
            if !same {
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn required(&self, p: &Program, m: &May, clause: usize) -> Result<Vec<usize>> {
        let mut terms = Vec::new();
        head_terms(p, p.clauses[clause].head, &mut terms);
        let mut vars = BTreeSet::new();
        for t in terms {
            let o = m.abs.origin(m.abs.terms[t]).unwrap();
            if self.precise[o] {
                let a = p.arena.named(t, "$guideline_id", 5).unwrap();
                p.arena.vars(a[4], &mut vars);
            }
        }
        Ok(vars.into_iter().collect())
    }
    pub fn check_variants(&self, base: &Program, bm: &May, q: &Program, qm: &May) -> Result<()> {
        let mut declarations = vec![false; self.keys.len()];
        for (i, variant) in self.variants.iter().enumerate() {
            let c = &q.clauses[i];
            if let Some(original) = variant.original {
                let source = &base.clauses[original];
                let required = self.required(base, bm, original)?;
                if variant.tags.iter().map(|(v, _)| *v).collect::<Vec<_>>() != required {
                    return Err(bad(format!(
                        "{} missing/extra/duplicate k1 variable tag",
                        base.label(original)
                    )));
                }
                for (_, tag) in &variant.tags {
                    Self::valid_tag(base, bm, tag)?;
                }
                if source.names.len() != c.names.len()
                    || c.body.len() != source.body.len() + variant.tags.len()
                    || !self.term_eq((base, bm), source.head, (q, qm), c.head, &variant.tags)?
                    || !self.goals_eq(
                        (base, bm),
                        &source.body,
                        (q, qm),
                        &c.body[..source.body.len()],
                        &variant.tags,
                    )?
                {
                    return Err(bad(format!(
                        "{} k1 variant changes source clause",
                        base.label(original)
                    )));
                }
                for ((variable, tag), goal) in variant.tags.iter().zip(&c.body[source.body.len()..])
                {
                    let valid = if let Goal::Positive(t) = goal
                        && let Some(a) = q.arena.named(*t, "$sb1_type", 2)
                    {
                        matches!(q.arena.node(a[0]), Node::Var(v) if *v == *variable)
                            && Self::decode_tag(bm, q, a[1])? == *tag
                    } else {
                        false
                    };
                    if !valid {
                        return Err(bad("k1 variant tag guard mismatch"));
                    }
                }
            } else {
                if !c.body.is_empty() || !variant.tags.is_empty() {
                    return Err(bad("k1 registry declaration shape"));
                }
                let a = q
                    .arena
                    .named(c.head, "$sb1_origin", 2)
                    .ok_or_else(|| bad("extra k1 clause without source"))?;
                let o = qm
                    .abs
                    .origin(qm.abs.terms[a[0]])
                    .ok_or_else(|| bad("registry declaration is not an identity"))?;
                let k = self.origin_keys[o];
                let identity = q
                    .arena
                    .named(a[0], "$guideline_id", 5)
                    .ok_or_else(|| bad("registry declaration identity shape"))?;
                if !q.arena.list(identity[4]).is_some_and(|d| d.is_empty())
                    || declarations[k]
                    || Self::decode_tag(bm, q, a[1])? != Tag::Origin(self.keys[k].origin)
                {
                    return Err(bad("registry declaration origin/tag mismatch"));
                }
                declarations[k] = true;
            }
        }
        if declarations.iter().any(|v| !v) {
            return Err(bad("missing k1 origin-tag declaration"));
        }
        Ok(())
    }
    fn tag(types: &Types, ty: u32) -> Tag {
        match types.nodes[ty as usize] {
            TypeNode::Scalar => Tag::Scalar,
            TypeNode::Identity(o) => Tag::Origin(o),
            TypeNode::Arithmetic { clause, goal } => Tag::Arithmetic(clause, goal),
        }
    }
    fn tag_value(m: &May, tag: &Tag) -> u32 {
        match tag {
            Tag::Scalar => Abs::PATIENT,
            Tag::Origin(o) => m.abs.origin_values[*o],
            Tag::Arithmetic(..) => Abs::NUMBER,
        }
    }
    fn walk(
        &self,
        source: (&Program, &May),
        original: usize,
        pattern_choices: (&Pattern, &[(usize, Vec<Tag>)]),
        frame: (usize, State),
        tags: &mut Vec<(usize, Tag)>,
        budget: &mut Budget,
    ) -> Result<()> {
        let (base, bm) = source;
        let (pattern, choices) = pattern_choices;
        let (at, state) = frame;
        budget.tick()?;
        if at == choices.len() {
            if witness(&base.arena, bm, pattern, state, budget)? {
                tags.sort_unstable_by_key(|(v, _)| *v);
                if !self.by_original[original]
                    .iter()
                    .any(|q| self.variants[*q].tags == *tags)
                {
                    return Err(bad(format!(
                        "{} missing realizable k1 clause variant {tags:?}",
                        base.label(original)
                    )));
                }
            }
            return Ok(());
        }
        let (var, options) = &choices[at];
        for tag in options {
            let mut next = state.clone();
            if next.constrain(Expr::Var(*var), bm.sets.one(Self::tag_value(bm, tag)), bm)
                && let Some(next) = solve(&base.arena, bm, pattern, next, budget)?
            {
                tags.push((*var, tag.clone()));
                self.walk(
                    (base, bm),
                    original,
                    (pattern, choices),
                    (at + 1, next),
                    tags,
                    budget,
                )?;
                tags.retain(|(v, _)| v != var);
            }
        }
        Ok(())
    }
    pub fn check_complete(
        &self,
        base: &Program,
        bm: &May,
        types: &Types,
        sources: &Sources,
        budget: &mut Budget,
    ) -> Result<()> {
        for (i, c) in base.clauses.iter().enumerate() {
            if base.patient[i] {
                continue;
            }
            let required = self.required(base, bm, i)?;
            if required.is_empty() {
                if !self.by_original[i]
                    .iter()
                    .any(|q| self.variants[*q].tags.is_empty())
                {
                    return Err(bad(format!(
                        "{} missing untagged clause variant",
                        base.label(i)
                    )));
                }
            } else {
                let mut choices = required
                    .into_iter()
                    .map(|v| {
                        let mut options = sources
                            .row(i, v)?
                            .types
                            .iter()
                            .map(|t| Self::tag(types, *t))
                            .collect::<Vec<_>>();
                        options.sort();
                        options.dedup();
                        Ok((v, options))
                    })
                    .collect::<Result<Vec<_>>>()?;
                choices.sort_by_key(|(v, tags)| (tags.len(), *v));
                let pattern = Pattern::body(&base.arena, &bm.abs, &c.body, c.names.len(), None);
                if let Some(s) = state(&base.arena, bm, &pattern, budget)? {
                    self.walk(
                        (base, bm),
                        i,
                        (&pattern, &choices),
                        (0, s),
                        &mut Vec::new(),
                        budget,
                    )?;
                }
            }
            bm.reset_scratch();
        }
        Ok(())
    }
    pub fn node_label(&self, base: &Program, bm: &May, types: &Types, ty: usize) -> Result<String> {
        let origin = |o: usize| {
            bm.abs
                .describe(&base.arena, bm.abs.origin_values[o])
                .trim_start_matches("origin:")
                .to_owned()
        };
        let tag = |tag: &Tag| match tag {
            Tag::Scalar => "base/patient".into(),
            Tag::Origin(o) => origin(*o),
            Tag::Arithmetic(c, g) => format!("arithmetic({},{})", base.detail_label(*c), g + 1),
        };
        Ok(match types.nodes[ty] {
            TypeNode::Scalar => "scalar".into(),
            TypeNode::Identity(o) => {
                let k = &self.keys[self.origin_keys[o]];
                format!(
                    "{}<{}>",
                    origin(k.origin),
                    if k.precise {
                        k.deps.iter().map(&tag).collect::<Vec<_>>().join(";")
                    } else {
                        "*".into()
                    }
                )
            }
            TypeNode::Arithmetic { clause, goal } => {
                let v = &self.variants[clause];
                let original = v
                    .original
                    .ok_or_else(|| bad("arithmetic witness has no original clause"))?;
                format!(
                    "arithmetic({},{})<{}>",
                    base.detail_label(original),
                    goal + 1,
                    v.tags
                        .iter()
                        .map(|(x, t)| format!(
                            "{}={}",
                            sb1_checker::term::variable_name(*x),
                            tag(t)
                        ))
                        .collect::<Vec<_>>()
                        .join(";")
                )
            }
        })
    }
    pub fn refined_tag(&self, types: &Types, ty: u32) -> Result<Tag> {
        match types.nodes[ty as usize] {
            TypeNode::Scalar => Ok(Tag::Scalar),
            TypeNode::Identity(o) => Ok(Tag::Origin(self.keys[self.origin_keys[o]].origin)),
            TypeNode::Arithmetic { clause, goal } => self.variants[clause]
                .original
                .map(|c| Tag::Arithmetic(c, goal))
                .ok_or_else(|| bad("arithmetic output on registry declaration")),
        }
    }
    pub fn check_compositional(
        &self,
        q: &Program,
        qm: &May,
        types: &Types,
        sources: &Sources,
    ) -> Result<()> {
        for (i, c) in q.clauses.iter().enumerate() {
            if self.variants[i].original.is_none() {
                continue;
            }
            let mut terms = Vec::new();
            head_terms(q, c.head, &mut terms);
            for term in terms {
                let key = self.key(q, qm, term)?;
                if !key.precise {
                    continue;
                }
                let a = q.arena.named(term, "$guideline_id", 5).unwrap();
                let deps = q
                    .arena
                    .list(a[4])
                    .ok_or_else(|| bad("refined constructor Deps"))?;
                if deps.len() != key.deps.len() {
                    return Err(bad("refined constructor tag arity"));
                }
                let vars = deps
                    .iter()
                    .map(|t| match q.arena.node(*t) {
                        Node::Var(v) => Ok(*v),
                        _ => Err(bad("refined source Deps is not variable")),
                    })
                    .collect::<Result<Vec<_>>>()?;
                let rows = vars
                    .iter()
                    .map(|v| sources.row(i, *v))
                    .collect::<Result<Vec<_>>>()?;
                if rows.iter().any(|r| r.types.is_empty()) {
                    continue;
                }
                for (row, expected) in rows.iter().zip(&key.deps) {
                    for ty in &row.types {
                        if self.refined_tag(types, *ty)? != *expected {
                            return Err(bad(format!(
                                "{} missing realizable k1 tuple/compositional type",
                                q.label(i)
                            )));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
