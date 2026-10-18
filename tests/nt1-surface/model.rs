// nt1 D2–D4 oracle. Source scenes own referent ordinals; no compiler output seeds it.
use super::support::sha;
use std::collections::BTreeMap;

pub const TABLE: &str = "# format: kind\tlemma\tvalue\nunit\tsecond\tsecond\nunit\tminute\tminute\nunit\thour\thour\nunit\tday\tday\nunit\td\tday\nunit\tweek\tweek\nunit\tmonth\tmonth\nunit\tyear\tyear\nrelation\tfor\tduration\nrelation\tafter\tafter\nrelation\tbefore\tbefore\nrelation\twithin\twithin\nspacing\tat\tinterval\nwindow\tduring\twindow\nfrequency\tper\tperiod\napproximation\tapproximate\tabout\nrange\tto\tminimum\nqualifier\tusually\tusuality\nqualifier\tneeded\tapplicability\npreference\tpreferred\tpreferred\n";
pub const ULEX: &str = "prep(per,per).\nprep(for,for).\nnoun_sg(d,d,neutr).\nnoun_pl(d,d,neutr).\nadj_itr(approximate,approximate).\nadj_itr(needed,needed).\nadj_itr(preferred,preferred).\n";
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
];
pub fn declarations(nt: bool) -> String {
    INDICATORS
        .iter()
        .filter(|(n, _)| nt || *n != "guideline_unit")
        .map(|(n, a)| format!(":- multifile({n}/{a}).\n:- discontiguous({n}/{a}).\n"))
        .collect()
}
pub fn quote(s: &str) -> String {
    let digits = s.strip_prefix('-').unwrap_or(s);
    if s == "[]"
        || s == "-"
        || (!digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
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
pub fn v(n: usize) -> Term {
    Term::V(format!("r{n}"))
}
pub fn c(s: &str, args: Vec<Term>) -> Term {
    Term::C(s.into(), args)
}
pub fn lit(s: &str, args: Vec<Term>) -> Term {
    c(
        &format!("guideline_{s}"),
        std::iter::once(a("actual")).chain(args).collect(),
    )
}
impl Term {
    pub fn args(&self) -> &[Term] {
        if let Self::C(_, xs) = self { xs } else { &[] }
    }
    pub fn name(&self) -> &str {
        match self {
            Self::A(n) | Self::C(n, _) => n,
            _ => "",
        }
    }
    fn text(&self, vars: &mut BTreeMap<String, usize>) -> String {
        match self {
            Self::A(x) => quote(x),
            Self::V(x) => {
                let next = vars.len();
                let n = *vars.entry(x.clone()).or_insert(next);
                let first = char::from(b'A' + u8::try_from(n % 26).unwrap());
                if n < 26 {
                    first.to_string()
                } else {
                    format!("{first}{}", n / 26)
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
    pub fn substitute(&self, f: &impl Fn(&Term) -> Option<Term>) -> Term {
        if let Some(t) = f(self) {
            return t;
        }
        match self {
            Self::C(n, xs) => c(n, xs.iter().map(|x| x.substitute(f)).collect()),
            Self::L(xs) => Self::L(xs.iter().map(|x| x.substitute(f)).collect()),
            _ => self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
pub enum Goal {
    Pos(Term),
    Naf(Vec<Term>),
}
impl Goal {
    pub fn render(&self, vars: &mut BTreeMap<String, usize>) -> String {
        match self {
            Self::Pos(t) => t.text(vars),
            Self::Naf(xs) if xs.len() == 1 => format!("\\+ {}", xs[0].text(vars)),
            Self::Naf(xs) => format!(
                "\\+ ({})",
                xs.iter()
                    .map(|x| x.text(vars))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
pub fn pos(ts: Vec<Term>) -> Vec<Goal> {
    ts.into_iter().map(Goal::Pos).collect()
}
pub fn obj(r: usize, noun: &str, cmp: &str, count: usize) -> Vec<Term> {
    vec![
        lit("entity", vec![v(r), a(noun), a("countable")]),
        lit(
            "cardinality",
            vec![v(r), a("na"), a(cmp), a(&count.to_string())],
        ),
    ]
}
pub fn event(e: usize, verb: &str, args: &[usize]) -> Vec<Term> {
    std::iter::once(lit("event", vec![v(e), a(verb)]))
        .chain(
            args.iter()
                .enumerate()
                .map(|(i, r)| lit("arg", vec![v(e), a(&(i + 1).to_string()), v(*r)])),
        )
        .collect()
}
pub fn gid(kind: &str, id: &str, ordinal: usize, deps: Term) -> Term {
    c(
        "$guideline_id",
        vec![
            a(kind),
            a(id),
            a("1"),
            c("ref", vec![a(&ordinal.to_string())]),
            deps,
        ],
    )
}
#[derive(Clone)]
pub struct Group {
    pub heads: Vec<Term>,
    pub body: Vec<Goal>,
    pub refs: Vec<usize>,
    pub variant: usize,
}
#[derive(Clone)]
pub struct Probe {
    pub row: &'static str,
    pub id: String,
    pub ace: String,
    pub table: bool,
    pub groups: Vec<Group>,
    pub query: Option<Vec<Term>>,
    pub alias: bool,
}
fn unit(noun: &str) -> Option<&str> {
    match noun {
        "second" | "minute" | "hour" | "day" | "week" | "month" | "year" => Some(noun),
        "d" => Some("day"),
        _ => None,
    }
}
fn convertible(cmp: &str) -> bool {
    ["exactly", "geq", "greater", "leq", "less"].contains(&cmp)
}
fn quantities(ts: &[Term], table: bool) -> BTreeMap<(Term, Term), (Term, Term, Option<Term>)> {
    ts.iter()
        .filter(|t| t.name() == "guideline_cardinality")
        .map(|t| {
            let x = t.args();
            let u = table
                .then(|| {
                    ts.iter()
                        .find(|t| t.name() == "guideline_entity" && t.args()[..2] == x[..2])
                })
                .flatten()
                .and_then(|t| unit(t.args()[2].name()))
                .map(a);
            (
                (x[0].clone(), x[1].clone()),
                (x[3].clone(), x[4].clone(), u),
            )
        })
        .collect()
}
pub fn annotate(ts: &[Term], table: bool, condition: bool) -> Vec<Term> {
    let qs = quantities(ts, table);
    let mut out = Vec::new();
    let fresh = |r: &Term, k: &str| Term::V(format!("{k}:{r:?}"));
    for t in ts {
        let x = t.args();
        let q = if x.len() >= 2 {
            qs.get(&(x[0].clone(), x[1].clone()))
        } else {
            None
        };
        let converted = q.is_some_and(|(cmp, _, _)| condition && convertible(cmp.name()));
        if t.name() == "guideline_entity" && converted && q.unwrap().2.is_some() {
            continue;
        }
        if t.name() == "guideline_cardinality" {
            let (cmp, n, u) = q.unwrap();
            if converted {
                let s = fresh(&x[1], "s");
                let value = fresh(&x[1], "n");
                out.push(c(
                    "guideline_cardinality",
                    vec![
                        x[0].clone(),
                        x[1].clone(),
                        x[2].clone(),
                        s.clone(),
                        value.clone(),
                    ],
                ));
                let args = if let Some(u) = u {
                    let uv = fresh(&x[1], "u");
                    out.push(c(
                        "guideline_unit",
                        vec![x[0].clone(), x[1].clone(), uv.clone()],
                    ));
                    vec![s, value, uv, cmp.clone(), n.clone(), u.clone()]
                } else {
                    vec![s, value, cmp.clone(), n.clone()]
                };
                out.push(c("guideline_satisfies", args));
            } else {
                out.push(t.clone());
                if let Some(u) = u {
                    out.push(c(
                        "guideline_unit",
                        vec![x[0].clone(), x[1].clone(), u.clone()],
                    ));
                }
            }
            continue;
        }
        let mut t = t.clone();
        if condition {
            let slots: &[(usize, usize)] = match t.name() {
                "guideline_interval" => &[(3, 4)],
                "guideline_recurrence" => &[(2, 3)],
                "guideline_recurrence_window" => &[(2, 3), (5, 6)],
                _ => &[],
            };
            if let Term::C(_, xs) = &mut t {
                for &(r, u) in slots {
                    if qs
                        .get(&(xs[0].clone(), xs[r].clone()))
                        .is_some_and(|(cmp, _, unit)| convertible(cmp.name()) && unit.is_some())
                    {
                        xs[u] = fresh(&xs[r], "u");
                    }
                }
            }
        }
        out.push(t);
    }
    out
}
pub fn body_annotate(body: &[Goal], table: bool) -> Vec<Goal> {
    // Positive runs share identities; NAF-local referents have their own scope.
    let positive: Vec<_> = body
        .iter()
        .filter_map(|g| {
            if let Goal::Pos(t) = g {
                Some(t.clone())
            } else {
                None
            }
        })
        .collect();
    let expanded = annotate(&positive, table, true);
    let mut it = expanded.into_iter();
    let mut out = Vec::new();
    for g in body {
        match g {
            Goal::Naf(ts) => out.push(Goal::Naf(annotate(ts, table, true))),
            Goal::Pos(t) => {
                if t.name() == "guideline_entity" {
                    let q = quantities(&positive, table);
                    if q.get(&(t.args()[0].clone(), t.args()[1].clone()))
                        .is_some_and(|(cmp, _, u)| convertible(cmp.name()) && u.is_some())
                    {
                        continue;
                    }
                }
                let first = it.next().unwrap();
                out.push(Goal::Pos(first));
                if t.name() == "guideline_cardinality" {
                    let q = quantities(&positive, table);
                    let (_, _, u) = &q[&(t.args()[0].clone(), t.args()[1].clone())];
                    if u.is_some() {
                        out.push(Goal::Pos(it.next().unwrap()));
                    }
                    if convertible(t.args()[3].name()) {
                        out.push(Goal::Pos(it.next().unwrap()));
                    }
                }
            }
        }
    }
    assert!(it.next().is_none());
    out
}
fn conjunction(ts: &[Term]) -> Term {
    assert!(!ts.is_empty());
    ts.iter()
        .rev()
        .cloned()
        .reduce(|tail, t| c(",", vec![t, tail]))
        .unwrap()
}
impl Probe {
    pub fn clauses(&self, nt: bool) -> String {
        let mut out = String::new();
        for group in &self.groups {
            let heads = if nt {
                annotate(&group.heads, self.table, false)
            } else {
                group.heads.clone()
            };
            let body = if nt {
                body_annotate(&group.body, self.table)
            } else {
                group.body.clone()
            };
            for h in heads {
                let mut vars = BTreeMap::new();
                out.push_str(&h.text(&mut vars));
                if !body.is_empty() {
                    out.push_str(" :- ");
                    out.push_str(
                        &body
                            .iter()
                            .map(|g| g.render(&mut vars))
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                }
                out.push_str(".\n");
            }
        }
        out
    }
    pub fn expected(&self, nt: bool) -> String {
        let table = if self.table {
            format!("sha256({})", quote(&sha(TABLE.as_bytes())))
        } else {
            "none".into()
        };
        let ace = quote(&sha(self.ace.as_bytes()));
        let ulex = quote(&sha(ULEX.as_bytes()));
        if let Some(ts) = &self.query {
            let goals = if nt {
                annotate(ts, self.table, true)
            } else {
                ts.clone()
            };
            return format!(
                "% {} compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,{},ace_sha256({ace}),ulex(sha256({ulex})),vocabulary({table})).\n% Q1: {}\n{}.\n",
                self.id,
                quote(&self.id),
                self.ace.trim_end(),
                c(
                    "$guideline_query_projection",
                    vec![
                        c("goal", vec![conjunction(&goals)]),
                        c("answers", vec![Term::L(vec![])])
                    ]
                )
                .render()
            );
        }
        format!(
            "% {}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({ace}),ulex(sha256({ulex})),vocabulary({table})).\n% S1: {}\n{}",
            self.id,
            declarations(nt),
            quote(&self.id),
            self.ace.trim_end(),
            self.clauses(nt)
        )
    }
    pub fn proof(&self, nt: bool) -> String {
        self.groups
            .iter()
            .map(|g| {
                let sub = |t: &Term| {
                    g.refs.iter().find(|&&n| *t == v(n)).map(|&n| {
                        gid(
                            "witness",
                            &self.id,
                            n,
                            c("variant", vec![a(&g.variant.to_string())]),
                        )
                    })
                };
                let body = if nt {
                    body_annotate(&g.body, self.table)
                } else {
                    g.body.clone()
                };
                let positives: Vec<_> = body
                    .iter()
                    .filter_map(|goal| {
                        if let Goal::Pos(t) = goal {
                            Some(t.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                let mut values = BTreeMap::new();
                for t in &positives {
                    if t.name() == "guideline_satisfies" {
                        let x = t.args();
                        let n = x.len() / 2;
                        for i in 0..n {
                            values.insert(x[i].clone(), x[i + n].clone());
                        }
                    }
                }
                let facts = positives
                    .iter()
                    .filter(|t| t.name() != "guideline_satisfies")
                    .map(|t| t.substitute(&|x| values.get(x).cloned().or_else(|| sub(x))))
                    .collect();
                let heads = if nt {
                    annotate(&g.heads, self.table, false)
                } else {
                    g.heads.clone()
                };
                let prove = heads.iter().map(|t| t.substitute(&sub)).collect();
                format!(
                    "{}.\n",
                    c(
                        "$guideline_proof",
                        vec![
                            a(&self.id),
                            a("1"),
                            c("variant", vec![a(&g.variant.to_string())]),
                            c("witness", vec![Term::L(facts)]),
                            c("prove", vec![Term::L(prove)])
                        ]
                    )
                    .render()
                )
            })
            .collect()
    }
}
pub fn rule(
    id: &str,
    ace: &str,
    row: &'static str,
    table: bool,
    body: Vec<Goal>,
    deps: Vec<usize>,
    next: usize,
) -> Probe {
    let e = gid(
        "product",
        id,
        next,
        Term::L(deps.iter().map(|&n| v(n)).collect()),
    );
    Probe {
        row,
        id: id.into(),
        ace: format!("{ace}\n"),
        table,
        alias: false,
        query: None,
        groups: vec![Group {
            heads: vec![
                lit("event", vec![e.clone(), a("sleep")]),
                lit("arg", vec![e, a("1"), v(1)]),
            ],
            body,
            refs: deps,
            variant: 1,
        }],
    }
}

// Canonical term reader used only by the independent erasure property.
fn split(s: &str) -> Vec<&str> {
    let (mut depth, mut quoted, mut escape, mut start) = (0i32, false, false, 0usize);
    let mut out = Vec::new();
    for (i, ch) in s.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        if quoted && ch == '\\' {
            escape = true;
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
                out.push(s[start..i].trim());
                start = i + 1;
            }
            _ => (),
        }
    }
    if !s.is_empty() {
        out.push(s[start..].trim());
    }
    out
}
fn parse(s: &str) -> Term {
    let s = s.trim();
    if s.starts_with('[') {
        return Term::L(split(&s[1..s.len() - 1]).into_iter().map(parse).collect());
    }
    let mut quoted = false;
    let mut escape = false;
    let mut open = None;
    for (i, ch) in s.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        if quoted && ch == '\\' {
            escape = true;
            continue;
        }
        if ch == '\'' {
            quoted = !quoted;
        } else if ch == '(' && !quoted {
            open = Some(i);
            break;
        }
    }
    if let Some(i) = open {
        return c(
            parse(&s[..i]).name(),
            split(&s[i + 1..s.len() - 1])
                .into_iter()
                .map(parse)
                .collect(),
        );
    }
    if s.starts_with('\'') {
        return a(&s[1..s.len() - 1].replace("\\'", "'").replace("\\\\", "\\"));
    }
    if s.bytes()
        .next()
        .is_some_and(|b| b.is_ascii_uppercase() || b == b'_')
    {
        Term::V(s.into())
    } else {
        a(s)
    }
}
fn erase_terms(ts: &[Term], alias: bool) -> Vec<Term> {
    let mut substitutions = BTreeMap::new();
    for t in ts.iter().filter(|t| t.name() == "guideline_satisfies") {
        let x = t.args();
        let n = x.len() / 2;
        for i in 0..n {
            substitutions.insert(x[i].clone(), x[i + n].clone());
        }
    }
    let mut out = Vec::new();
    for t in ts {
        if ["guideline_unit", "guideline_satisfies"].contains(&t.name()) {
            continue;
        }
        if t.name() == "guideline_cardinality" {
            let x = t.args();
            if substitutions.contains_key(&x[3])
                && let Some(u) = ts
                    .iter()
                    .find(|u| u.name() == "guideline_unit" && u.args()[..2] == x[..2])
            {
                let unit = u.args()[2].substitute(&|x| substitutions.get(x).cloned());
                let noun = if alias && unit.name() == "day" {
                    a("d")
                } else {
                    unit
                };
                out.push(c(
                    "guideline_entity",
                    vec![x[0].clone(), x[1].clone(), noun, a("countable")],
                ));
            }
        }
        out.push(t.substitute(&|x| substitutions.get(x).cloned()));
    }
    out
}
fn flatten(t: &Term) -> Vec<Term> {
    if t.name() == "," {
        t.args().iter().flat_map(flatten).collect()
    } else {
        vec![t.clone()]
    }
}
pub fn erase(text: &str, alias: bool) -> String {
    let mut out = String::new();
    for line in text.lines() {
        if line.contains("(guideline_unit/3)") {
            continue;
        }
        if line.starts_with("'$guideline_query_projection'") {
            let t = parse(line.trim_end_matches('.'));
            let old = erase_terms(&flatten(&t.args()[0].args()[0]), alias);
            out.push_str(&format!(
                "{}.\n",
                c(
                    "$guideline_query_projection",
                    vec![c("goal", vec![conjunction(&old)]), t.args()[1].clone()]
                )
                .render()
            ));
            continue;
        }
        if !line.starts_with("guideline_")
            || line.starts_with("guideline_schema_version(")
            || line.starts_with("guideline_document(")
        {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        let (head, body) = line
            .trim_end_matches('.')
            .split_once(" :- ")
            .map_or((line.trim_end_matches('.'), None), |(h, b)| (h, Some(b)));
        let head = parse(head);
        if head.name() == "guideline_unit" {
            continue;
        }
        let mut vars = BTreeMap::new();
        out.push_str(&head.text(&mut vars));
        if let Some(body) = body {
            let mut positive = Vec::new();
            let mut parts = Vec::new();
            for part in split(body) {
                if let Some(neg) = part.strip_prefix("\\+ ") {
                    if !positive.is_empty() {
                        parts.extend(erase_terms(&positive, alias).into_iter().map(Goal::Pos));
                        positive.clear();
                    }
                    let neg = if neg.starts_with('(') {
                        &neg[1..neg.len() - 1]
                    } else {
                        neg
                    };
                    parts.push(Goal::Naf(erase_terms(
                        &split(neg).into_iter().map(parse).collect::<Vec<_>>(),
                        alias,
                    )));
                } else {
                    positive.push(parse(part));
                }
            }
            parts.extend(erase_terms(&positive, alias).into_iter().map(Goal::Pos));
            out.push_str(" :- ");
            out.push_str(
                &parts
                    .iter()
                    .map(|g| g.render(&mut vars))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
        out.push_str(".\n");
    }
    out
}

// APE is the trusted parser, not the numeric emitter. Its upper-bound lists
// determine condition order and referent ordinals before D2/D3 are applied.
pub fn source_order(p: &mut Probe, dump: &str) {
    fn leaves(t: &Term, out: &mut Vec<Term>) {
        match t {
            Term::C(n, xs) if n == "drs" && xs.len() == 2 => leaves(&xs[1], out),
            Term::C(n, xs) if n == "-" && xs.len() == 2 => leaves(&xs[0], out),
            Term::C(n, _)
                if ["object", "predicate", "modifier_pp", "relation", "property"]
                    .contains(&n.as_str()) =>
            {
                out.push(t.clone())
            }
            Term::C(_, xs) | Term::L(xs) => {
                for x in xs {
                    leaves(x, out);
                }
            }
            _ => (),
        }
    }
    fn raw_ref(t: &Term) -> usize {
        let Term::V(s) = t else {
            panic!("DRS referent {t:?}")
        };
        assert!(s.as_bytes()[0].is_ascii_uppercase());
        1 + usize::from(s.as_bytes()[0] - b'A') + 26 * s[1..].parse::<usize>().unwrap_or(0)
    }
    fn scene_ref(t: &Term) -> usize {
        if let Term::V(s) = t {
            s.strip_prefix('r').unwrap().parse().unwrap()
        } else if t.name() == "$guideline_id" {
            t.args()[3].args()[0].name().parse().unwrap()
        } else {
            panic!("scene referent {t:?}")
        }
    }
    fn key(name: &str, args: &[Term], number: fn(&Term) -> usize) -> String {
        match name {
            "object" | "predicate" | "property" => format!("{name}:{}", number(&args[0])),
            "modifier_pp" => format!(
                "pp:{}:{}:{}",
                number(&args[0]),
                args[1].name(),
                number(&args[2])
            ),
            _ => panic!("source key {name}"),
        }
    }
    fn term_keys(ts: &[Term]) -> Vec<String> {
        let mut pp = String::new();
        ts.iter()
            .map(|t| {
                let x = t.args();
                match t.name() {
                    "guideline_entity" | "guideline_cardinality" => {
                        key("object", &x[1..], scene_ref)
                    }
                    "guideline_event" | "guideline_arg" => key("predicate", &x[1..], scene_ref),
                    "guideline_property" => key("property", &x[1..], scene_ref),
                    "guideline_pp" => {
                        pp = key("modifier_pp", &x[1..], scene_ref);
                        pp.clone()
                    }
                    "guideline_interval"
                    | "guideline_recurrence"
                    | "guideline_frequency"
                    | "guideline_range"
                    | "guideline_recurrence_window" => {
                        assert!(!pp.is_empty());
                        pp.clone()
                    }
                    other => panic!("scene key {other}"),
                }
            })
            .collect()
    }
    fn ordered(ts: &[Term], positions: &BTreeMap<String, usize>) -> Vec<Term> {
        let mut pairs: Vec<_> = term_keys(ts).into_iter().zip(ts.iter().cloned()).collect();
        pairs.sort_by_key(|(key, _)| {
            *positions
                .get(key)
                .unwrap_or_else(|| panic!("DRS lacks source key {key}"))
        });
        pairs.into_iter().map(|(_, t)| t).collect()
    }
    assert!(
        dump.lines().any(|l| l == "messages([])."),
        "{}: parser diagnostics {dump}",
        p.id
    );
    let line = dump.lines().find(|l| l.starts_with("drs(")).unwrap();
    let mut source = Vec::new();
    leaves(&parse(line.trim_end_matches('.')), &mut source);
    let mut ordinals = BTreeMap::new();
    let mut positions = BTreeMap::new();
    for (position, leaf) in source.iter().enumerate() {
        let x = leaf.args();
        let refs: Vec<_> = match leaf.name() {
            "object" | "property" => vec![&x[0]],
            // Relation metadata does not introduce witness referents.
            "relation" => Vec::new(),
            _ => x.iter().filter(|x| matches!(x, Term::V(_))).collect(),
        };
        for r in refs {
            let next = ordinals.len() + 1;
            ordinals.entry(raw_ref(r)).or_insert(next);
        }
        if leaf.name() != "relation" {
            let k = key(leaf.name(), x, raw_ref);
            assert!(
                positions.insert(k.clone(), position).is_none(),
                "{} duplicate source key {k}",
                p.id
            );
        }
    }
    let rename = |t: &Term| {
        if let Term::V(s) = t {
            Some(v(
                ordinals[&s.strip_prefix('r').unwrap().parse::<usize>().unwrap()]
            ))
        } else if t.name() == "ref" {
            Some(c(
                "ref",
                vec![a(
                    &ordinals[&t.args()[0].name().parse::<usize>().unwrap()].to_string()
                )],
            ))
        } else {
            None
        }
    };
    if let Some(query) = &p.query {
        p.query = Some(
            ordered(query, &positions)
                .iter()
                .map(|t| t.substitute(&rename))
                .collect(),
        );
    }
    for g in &mut p.groups {
        g.heads = ordered(&g.heads, &positions)
            .iter()
            .map(|t| t.substitute(&rename))
            .collect();
        let positives: Vec<_> = g
            .body
            .iter()
            .filter_map(|x| {
                if let Goal::Pos(t) = x {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect();
        let mut iter = ordered(&positives, &positions).into_iter();
        for goal in &mut g.body {
            match goal {
                Goal::Pos(t) => *t = iter.next().unwrap().substitute(&rename),
                Goal::Naf(ts) => {
                    *ts = ordered(ts, &positions)
                        .iter()
                        .map(|t| t.substitute(&rename))
                        .collect()
                }
            }
        }
        g.refs = g.refs.iter().map(|n| ordinals[n]).collect();
    }
}
