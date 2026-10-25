// Independent projection of the DRS constructs exercised here; APE alone supplies syntax.
use super::support::sha;
use std::collections::BTreeMap;

pub const HEADER: &str = "# format: kind\tlemma\tvalue\n";
pub const BASE: &str =
    "unit\tyear\tyear\nunit\thour\thour\nrelation\tfor\tduration\nrelation\tafter\tafter\n";
pub const ULEX: &str = "noun_sg('cancer-related-pain','cancer-related-pain',neutr).\nnoun_sg('hospital-care','hospital-care',neutr).\nnoun_sg('facility-discharge','facility-discharge',neutr).\nnoun_sg('opioid-dosage-recommendation','opioid-dosage-recommendation',neutr).\nprep(for,for).\n";
pub const KEY: &str = "scope-alpha";
pub fn table(key: Option<&str>) -> String {
    format!(
        "{HEADER}{BASE}{}",
        key.map_or_else(String::new, |k| format!(
            "scope\trecommendation\t{k}\nexclusion\tapply\tscope\n"
        ))
    )
}
pub const INDICATORS: &[(&str, usize)] = &[
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
    ("guideline_supersession", 4),
    ("guideline_evaluation_date", 1),
];
pub fn declarations() -> String {
    INDICATORS
        .iter()
        .map(|(n, a)| format!(":- multifile({n}/{a}).\n:- discontiguous({n}/{a}).\n"))
        .collect()
}
pub fn without_block(text: &str) -> String {
    text.lines()
        .filter(|l| {
            ![
                "(guideline_exclusion/1)",
                "(guideline_supersession/4)",
                "(guideline_evaluation_date/1)",
            ]
            .iter()
            .any(|indicator| l.contains(indicator))
        })
        .map(|l| format!("{l}\n"))
        .collect()
}
pub fn quote(s: &str) -> String {
    if s == "-"
        || (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
        || (s.bytes().next().is_some_and(|b| b.is_ascii_lowercase())
            && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'))
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Term {
    A(String),
    V(String),
    C(String, Vec<Term>),
    L(Vec<Term>),
}
pub fn a(s: &str) -> Term {
    Term::A(s.into())
}
pub fn v(s: &str) -> Term {
    Term::V(s.into())
}
pub fn c(s: &str, xs: Vec<Term>) -> Term {
    Term::C(s.into(), xs)
}
impl Term {
    pub fn name(&self) -> &str {
        match self {
            Self::A(s) | Self::C(s, _) => s,
            _ => "",
        }
    }
    pub fn args(&self) -> &[Term] {
        match self {
            Self::C(_, xs) | Self::L(xs) => xs,
            _ => &[],
        }
    }
    pub fn text(&self, vars: &mut BTreeMap<String, usize>) -> String {
        match self {
            Self::A(s) => quote(s),
            Self::V(s) => {
                let next = vars.len();
                let n = *vars.entry(s.clone()).or_insert(next);
                let ch = char::from(b'A' + u8::try_from(n % 26).unwrap());
                if n < 26 {
                    ch.to_string()
                } else {
                    format!("{ch}{}", n / 26)
                }
            }
            Self::C(n, xs) => format!(
                "{}({})",
                quote(n),
                xs.iter()
                    .map(|x| x.text(vars))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::L(xs) => format!(
                "[{}]",
                xs.iter()
                    .map(|x| x.text(vars))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
    pub fn render(&self) -> String {
        self.text(&mut BTreeMap::new())
    }
    pub fn sub(&self, map: &BTreeMap<Term, Term>) -> Term {
        if let Some(x) = map.get(self) {
            return x.clone();
        }
        match self {
            Self::C(n, xs) => c(n, xs.iter().map(|x| x.sub(map)).collect()),
            Self::L(xs) => Self::L(xs.iter().map(|x| x.sub(map)).collect()),
            _ => self.clone(),
        }
    }
}
pub fn split(s: &str) -> Vec<&str> {
    let (mut depth, mut quoted, mut escaped, mut begin) = (0i32, false, false, 0usize);
    let mut out = Vec::new();
    for (i, ch) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '\'' {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        match ch {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                out.push(s[begin..i].trim());
                begin = i + 1;
            }
            _ => (),
        }
    }
    if !s.is_empty() {
        out.push(s[begin..].trim());
    }
    out
}
pub fn parse(s: &str) -> Term {
    let s = s.trim();
    if s.starts_with('[') {
        return Term::L(split(&s[1..s.len() - 1]).into_iter().map(parse).collect());
    }
    let (mut quoted, mut escaped) = (false, false);
    for (i, ch) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '\'' {
            quoted = !quoted;
        } else if ch == '(' && !quoted {
            return c(
                parse(&s[..i]).name(),
                split(&s[i + 1..s.len() - 1])
                    .into_iter()
                    .map(parse)
                    .collect(),
            );
        }
    }
    if s.starts_with('\'') {
        a(&s[1..s.len() - 1].replace("\\'", "'").replace("\\\\", "\\"))
    } else if s
        .bytes()
        .next()
        .is_some_and(|b| b.is_ascii_uppercase() || b == b'_')
    {
        v(s)
    } else {
        a(s)
    }
}
fn strip(t: &Term) -> &Term {
    if t.name() == "-" && t.args().len() == 2 {
        &t.args()[0]
    } else {
        t
    }
}
fn leaves(t: &Term) -> Vec<Term> {
    if let Term::L(xs) = t {
        xs.iter().flat_map(leaves).collect()
    } else {
        vec![strip(t).clone()]
    }
}
fn gid(role: &str, id: &str, slot: Term, deps: Term) -> Term {
    c("$guideline_id", vec![a(role), a(id), a("1"), slot, deps])
}
fn lit(ctx: &Term, name: &str, args: Vec<Term>) -> Term {
    c(
        &format!("guideline_{name}"),
        std::iter::once(ctx.clone()).chain(args).collect(),
    )
}
#[derive(Clone, Debug)]
enum Item {
    Leaf(Term, Term),
    Edge(Term, Term, String, usize),
    Naf(Vec<Item>),
}
fn flatten(
    cs: &[Term],
    ctx: &Term,
    body: bool,
    id: &str,
    deps: &[Term],
    next: &mut usize,
) -> Vec<Item> {
    let mut out = Vec::new();
    for raw in cs {
        let t = strip(raw);
        if let Term::L(xs) = t {
            out.extend(flatten(xs, ctx, body, id, deps, next));
        } else if ["should", "must", "may", "can", "-"].contains(&t.name()) && t.args().len() == 1 {
            let n = *next;
            *next += 1;
            let inner = if body {
                v(&format!("box:{n}"))
            } else {
                gid(
                    "context",
                    id,
                    c("box", vec![a(&n.to_string())]),
                    Term::L(deps.to_vec()),
                )
            };
            out.push(Item::Edge(ctx.clone(), inner.clone(), t.name().into(), n));
            out.extend(flatten(
                t.args()[0].args()[1].args(),
                &inner,
                body,
                id,
                deps,
                next,
            ));
        } else if t.name() == "~" {
            out.push(Item::Naf(flatten(
                t.args()[0].args()[1].args(),
                ctx,
                body,
                id,
                deps,
                next,
            )));
        } else {
            out.push(Item::Leaf(ctx.clone(), t.clone()));
        }
    }
    out
}
fn refs(items: &[Item]) -> Vec<Term> {
    let mut out = Vec::new();
    for it in items {
        let rs = match it {
            Item::Leaf(_, t) => match t.name() {
                "object" | "property" => vec![t.args()[0].clone()],
                "predicate" => std::iter::once(t.args()[0].clone())
                    .chain(t.args()[2..].iter().cloned())
                    .collect(),
                "modifier_pp" => vec![t.args()[0].clone(), t.args()[2].clone()],
                _ => vec![],
            },
            Item::Naf(xs) => refs(xs),
            Item::Edge(..) => vec![],
        };
        for r in rs {
            if matches!(r, Term::V(_)) && !out.contains(&r) {
                out.push(r);
            }
        }
    }
    out
}
#[derive(Clone, Debug)]
pub enum Goal {
    Pos(Term),
    Naf(Vec<Term>),
}
impl Goal {
    pub fn text(&self, vars: &mut BTreeMap<String, usize>) -> String {
        match self {
            Self::Pos(t) => t.text(vars),
            Self::Naf(ts) if ts.len() == 1 => format!("\\+ {}", ts[0].text(vars)),
            Self::Naf(ts) => format!(
                "\\+ ({})",
                ts.iter()
                    .map(|t| t.text(vars))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
fn object_declarations(t: &Term, out: &mut BTreeMap<Term, Term>) {
    if let Term::C(name, xs) = t
        && name == "object"
        && xs.len() == 6
    {
        out.entry(xs[0].clone()).or_insert_with(|| t.clone());
    }
    for x in t.args() {
        object_declarations(x, out);
    }
}
fn unit(o: &Term, table: bool) -> Option<&str> {
    if table && ["hour", "year"].contains(&o.args()[1].name()) {
        Some(o.args()[1].name())
    } else {
        None
    }
}
fn expand(items: &[Item], body: bool, table: bool, objects: &BTreeMap<Term, Term>) -> Vec<Goal> {
    let mut out = Vec::new();
    for it in items {
        let ts = match it {
            Item::Edge(outer, inner, op, _) => vec![c(
                "guideline_operator",
                vec![outer.clone(), inner.clone(), a(op)],
            )],
            Item::Naf(xs) => {
                let gs = expand(xs, body, table, objects)
                    .into_iter()
                    .map(|g| match g {
                        Goal::Pos(t) => t,
                        _ => panic!("nested NAF outside fixture model"),
                    })
                    .collect();
                out.push(Goal::Naf(gs));
                continue;
            }
            Item::Leaf(ctx, t) => {
                let x = t.args();
                match t.name() {
                    "object" => {
                        let u = unit(t, table);
                        let converted = body
                            && ["exactly", "geq", "greater", "leq", "less"].contains(&x[4].name());
                        let mut ts = Vec::new();
                        if !converted || u.is_none() {
                            ts.push(lit(
                                ctx,
                                "entity",
                                vec![x[0].clone(), x[1].clone(), x[2].clone()],
                            ));
                        }
                        if converted {
                            let s = v(&format!("cmp:{:?}", x[0]));
                            let n = v(&format!("count:{:?}", x[0]));
                            ts.push(lit(
                                ctx,
                                "cardinality",
                                vec![x[0].clone(), x[3].clone(), s.clone(), n.clone()],
                            ));
                            let args = if let Some(u) = u {
                                let uv = v(&format!("unit:{:?}", x[0]));
                                ts.push(lit(ctx, "unit", vec![x[0].clone(), uv.clone()]));
                                vec![s, n, uv, x[4].clone(), x[5].clone(), a(u)]
                            } else {
                                vec![s, n, x[4].clone(), x[5].clone()]
                            };
                            ts.push(c("guideline_satisfies", args));
                        } else {
                            ts.push(lit(
                                ctx,
                                "cardinality",
                                vec![x[0].clone(), x[3].clone(), x[4].clone(), x[5].clone()],
                            ));
                            if let Some(u) = u {
                                ts.push(lit(ctx, "unit", vec![x[0].clone(), a(u)]));
                            }
                        }
                        ts
                    }
                    "predicate" => {
                        std::iter::once(lit(ctx, "event", vec![x[0].clone(), x[1].clone()]))
                            .chain(x[2..].iter().enumerate().map(|(i, r)| {
                                lit(
                                    ctx,
                                    "arg",
                                    vec![x[0].clone(), a(&(i + 1).to_string()), r.clone()],
                                )
                            }))
                            .collect()
                    }
                    "modifier_pp" => {
                        let mut ts = vec![lit(ctx, "pp", x.to_vec())];
                        if table
                            && x[1].name() == "for"
                            && let Some(o) = items.iter().find_map(|item| match item {
                                Item::Leaf(at, object)
                                    if at == ctx
                                        && object.name() == "object"
                                        && object.args()[0] == x[2] =>
                                {
                                    Some(object)
                                }
                                _ => None,
                            })
                            && let Some(u) = unit(o, table)
                        {
                            let uv = if body
                                && ["exactly", "geq", "greater", "leq", "less"]
                                    .contains(&o.args()[4].name())
                            {
                                v(&format!("unit:{:?}", x[2]))
                            } else {
                                a(u)
                            };
                            ts.push(lit(
                                ctx,
                                "interval",
                                vec![x[0].clone(), a("duration"), x[2].clone(), uv, a("none")],
                            ));
                        } else if table
                            && x[1].name() == "after"
                            && objects.get(&x[2]).is_some_and(|o| unit(o, true).is_none())
                        {
                            // Order reads the global noun declaration; an interval joins locally.
                            ts.push(lit(
                                ctx,
                                "order",
                                vec![x[0].clone(), a("after"), x[2].clone()],
                            ));
                        }
                        ts
                    }
                    "property" => vec![lit(ctx, "property", x.to_vec())],
                    _ => panic!("fixture model leaf {t:?}"),
                }
            }
        };
        out.extend(ts.into_iter().map(Goal::Pos));
    }
    out
}
#[derive(Clone)]
pub struct Group {
    pub heads: Vec<Term>,
    pub body: Vec<Goal>,
    pub witness: BTreeMap<Term, Term>,
    pub variant: usize,
}
#[derive(Clone)]
pub struct Product {
    pub id: String,
    pub ace: String,
    pub table: bool,
    pub groups: Vec<Group>,
}
impl Product {
    pub fn from_dump(id: &str, ace: &str, dump: &str, table: bool) -> Self {
        assert!(
            dump.lines().any(|l| l == "messages([])."),
            "{id}: parser messages {dump}"
        );
        let drs = parse(
            dump.lines()
                .find(|l| l.starts_with("drs("))
                .unwrap()
                .trim_end_matches('.'),
        );
        assert_eq!(drs.name(), "drs");
        assert_eq!(drs.args().len(), 1);
        let drs = &drs.args()[0];
        let mut objects = BTreeMap::new();
        object_declarations(drs, &mut objects);
        let roots = leaves(&drs.args()[1]);
        let mut plans: Vec<(Vec<Item>, Vec<Item>, Vec<Term>)> = Vec::new();
        if roots.len() == 1 && roots[0].name() == "=>" {
            let mut ante = roots[0].args()[0].clone();
            let mut cons = roots[0].args()[1].clone();
            let mut shared = Vec::new();
            let mut deps = Vec::new();
            loop {
                deps.extend(ante.args()[0].args().iter().cloned());
                shared.extend(leaves(&ante.args()[1]));
                let cs = leaves(&cons.args()[1]);
                if cons.args()[0].args().is_empty() && cs.len() == 1 && cs[0].name() == "=>" {
                    ante = cs[0].args()[0].clone();
                    cons = cs[0].args()[1].clone();
                } else {
                    break;
                }
            }
            let disj = shared
                .iter()
                .position(|t| t.name() == "v")
                .map(|i| shared.remove(i));
            let mut next = 1;
            let sh = flatten(&shared, &a("actual"), true, id, &[], &mut next);
            if let Some(disj) = disj {
                let arms: Vec<_> = disj
                    .args()
                    .iter()
                    .map(|arm| {
                        (
                            arm.args()[0].args().to_vec(),
                            flatten(arm.args()[1].args(), &a("actual"), true, id, &[], &mut next),
                        )
                    })
                    .collect();
                let cstart = next;
                for (dom, items) in arms {
                    let mut ds = deps.clone();
                    ds.extend(dom);
                    let mut bs = sh.clone();
                    bs.extend(items);
                    let hs = flatten(
                        cons.args()[1].args(),
                        &a("actual"),
                        false,
                        id,
                        &ds,
                        &mut next.clone(),
                    );
                    plans.push((bs, hs, ds));
                }
                assert!(cstart >= 1);
            } else {
                let hs = flatten(
                    cons.args()[1].args(),
                    &a("actual"),
                    false,
                    id,
                    &deps,
                    &mut next,
                );
                plans.push((sh, hs, deps));
            }
        } else {
            plans.push((
                vec![],
                flatten(&roots, &a("actual"), false, id, &[], &mut 1),
                vec![],
            ));
        }
        let mut ordered = Vec::new();
        for (bs, _, _) in &plans {
            for r in refs(bs) {
                if !ordered.contains(&r) {
                    ordered.push(r);
                }
            }
        }
        for (_, hs, _) in &plans {
            for r in refs(hs) {
                if !ordered.contains(&r) {
                    ordered.push(r);
                }
            }
        }
        let ordinal = |r: &Term| ordered.iter().position(|x| x == r).unwrap() + 1;
        let groups = plans
            .into_iter()
            .enumerate()
            .map(|(i, (bs, hs, deps))| {
                let variant = i + 1;
                let bodyrefs = refs(&bs);
                let sko: BTreeMap<_, _> = refs(&hs)
                    .into_iter()
                    .filter(|r| !bodyrefs.contains(r))
                    .map(|r| {
                        let product = gid(
                            "product",
                            id,
                            c("ref", vec![a(&ordinal(&r).to_string())]),
                            Term::L(deps.clone()),
                        );
                        (r, product)
                    })
                    .collect();
                let body = expand(&bs, true, table, &objects);
                let heads = expand(&hs, false, table, &objects)
                    .into_iter()
                    .map(|g| match g {
                        Goal::Pos(t) => t.sub(&sko),
                        _ => panic!("NAF head"),
                    })
                    .collect();
                let mut witness = BTreeMap::new();
                for item in &bs {
                    match item {
                        Item::Edge(_, inner, _, n) => {
                            witness.insert(
                                inner.clone(),
                                gid(
                                    "witness",
                                    id,
                                    c("box", vec![a(&n.to_string())]),
                                    c("variant", vec![a(&variant.to_string())]),
                                ),
                            );
                        }
                        Item::Leaf(..) => {
                            for r in refs(std::slice::from_ref(item)) {
                                let term = gid(
                                    "witness",
                                    id,
                                    c("ref", vec![a(&ordinal(&r).to_string())]),
                                    c("variant", vec![a(&variant.to_string())]),
                                );
                                witness.insert(r, term);
                            }
                        }
                        Item::Naf(_) => (),
                    }
                }
                for g in &body {
                    if let Goal::Pos(t) = g
                        && t.name() == "guideline_satisfies"
                    {
                        let x = t.args();
                        for j in 0..x.len() / 2 {
                            witness.insert(x[j].clone(), x[j + x.len() / 2].clone());
                        }
                    }
                }
                Group {
                    heads,
                    body,
                    witness,
                    variant,
                }
            })
            .collect();
        Self {
            id: id.into(),
            ace: ace.into(),
            table,
            groups,
        }
    }
    pub fn clauses(&self, key: Option<&str>, exclusion: bool) -> String {
        let mut out = String::new();
        for g in &self.groups {
            let mut heads = g.heads.clone();
            if exclusion && let Some(k) = key {
                heads.push(c("guideline_exclusion", vec![a(k)]));
            }
            for h in heads {
                let mut body = g.body.clone();
                if !body.is_empty()
                    && h.name() == "guideline_operator"
                    && ["should", "must", "may"].contains(&h.args()[2].name())
                    && let Some(k) = key
                {
                    body.push(Goal::Naf(vec![c("guideline_exclusion", vec![a(k)])]));
                }
                let mut vars = BTreeMap::new();
                out.push_str(&h.text(&mut vars));
                if !body.is_empty() {
                    out.push_str(" :- ");
                    out.push_str(
                        &body
                            .iter()
                            .map(|b| b.text(&mut vars))
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                }
                out.push_str(".\n");
            }
        }
        out
    }
    pub fn document(&self, key: Option<&str>, exclusion: bool) -> String {
        let tab = if self.table {
            format!("sha256({})", quote(&sha(table(key))))
        } else {
            "none".into()
        };
        format!(
            "% {}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex(sha256({})),vocabulary({tab})).\n% S1: {}\n{}",
            self.id,
            declarations(),
            quote(&self.id),
            quote(&sha(&self.ace)),
            quote(&sha(ULEX)),
            self.ace.trim_end(),
            self.clauses(key, exclusion)
        )
    }
    pub fn proof(&self, key: Option<&str>, exclusion: bool) -> String {
        self.groups
            .iter()
            .map(|g| {
                let facts = g
                    .body
                    .iter()
                    .filter_map(|goal| match goal {
                        Goal::Pos(t) if t.name() != "guideline_satisfies" => {
                            Some(t.sub(&g.witness))
                        }
                        _ => None,
                    })
                    .collect();
                let mut heads: Vec<_> = g.heads.iter().map(|t| t.sub(&g.witness)).collect();
                if exclusion && let Some(k) = key {
                    heads.push(c("guideline_exclusion", vec![a(k)]));
                }
                format!(
                    "{}.\n",
                    c(
                        "$guideline_proof",
                        vec![
                            a(&self.id),
                            a("1"),
                            c("variant", vec![a(&g.variant.to_string())]),
                            c("witness", vec![Term::L(facts)]),
                            c("prove", vec![Term::L(heads)])
                        ]
                    )
                    .render()
                )
            })
            .collect()
    }
}
