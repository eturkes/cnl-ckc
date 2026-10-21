// D3/D4 scene projection. APE supplies syntax/order only; arithmetic reads source lexemes.
use super::cases::{Case, ULEX};
use super::support::sha;
use super::term::*;
use std::collections::{BTreeMap, BTreeSet};

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
];
pub fn declarations() -> String {
    INDICATORS
        .iter()
        .map(|(n, k)| format!(":- multifile({n}/{k}).\n:- discontiguous({n}/{k}).\n"))
        .collect()
}
pub fn document(
    id: &str,
    ace: &str,
    ulex: Option<&str>,
    table: Option<&str>,
    clauses: &str,
) -> String {
    let digest = |s: Option<&str>| {
        s.map_or_else(|| "none".into(), |s| format!("sha256({})", quote(&sha(s))))
    };
    format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex({}),vocabulary({})).\n% S1: {}\n{clauses}",
        declarations(),
        quote(id),
        quote(&sha(ace)),
        digest(ulex),
        digest(table),
        ace.trim_end()
    )
}
pub fn lit(ctx: &Term, name: &str, args: Vec<Term>) -> Term {
    c(
        &format!("guideline_{name}"),
        std::iter::once(ctx.clone()).chain(args).collect(),
    )
}
pub fn gid(kind: &str, id: &str, slot: Term, deps: Term) -> Term {
    c("$guideline_id", vec![a(kind), a(id), a(1), slot, deps])
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
            Self::Naf(xs) if xs.len() == 1 => format!("\\+ {}", xs[0].text(vars)),
            Self::Naf(xs) => format!(
                "\\+ ({})",
                xs.iter()
                    .map(|t| t.text(vars))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
#[derive(Clone, Debug)]
enum Item {
    Leaf(Term, Term),
    Edge(Term, Term, String, usize),
    Naf(Vec<Item>),
}
fn flatten(
    raw: &[Term],
    ctx: &Term,
    body: bool,
    id: &str,
    deps: &[Term],
    boxno: &mut usize,
) -> Vec<Item> {
    let mut out = Vec::new();
    for raw in raw {
        let t = strip(raw);
        if let Term::List(xs) = t {
            out.extend(flatten(xs, ctx, body, id, deps, boxno));
        } else if ["should", "must", "may", "can", "-"].contains(&t.name()) && t.args().len() == 1 {
            let n = *boxno;
            *boxno += 1;
            let inner = if body {
                v(format!("box:{n}"))
            } else {
                gid(
                    "context",
                    id,
                    c("box", vec![a(n)]),
                    Term::List(deps.to_vec()),
                )
            };
            out.push(Item::Edge(ctx.clone(), inner.clone(), t.name().into(), n));
            out.extend(flatten(
                t.args()[0].args()[1].args(),
                &inner,
                body,
                id,
                deps,
                boxno,
            ));
        } else if t.name() == "~" {
            out.push(Item::Naf(flatten(
                t.args()[0].args()[1].args(),
                ctx,
                body,
                id,
                deps,
                boxno,
            )));
        } else {
            out.push(Item::Leaf(ctx.clone(), raw.clone()));
        }
    }
    out
}
fn refs(items: &[Item]) -> Vec<Term> {
    let mut out = Vec::new();
    for item in items {
        let rs = match item {
            Item::Leaf(_, raw) => {
                let t = strip(raw);
                match t.name() {
                    "object" | "property" => vec![t.args()[0].clone()],
                    "predicate" => std::iter::once(t.args()[0].clone())
                        .chain(t.args()[2..].iter().cloned())
                        .collect(),
                    "modifier_pp" => vec![t.args()[0].clone(), t.args()[2].clone()],
                    _ => vec![],
                }
            }
            Item::Naf(xs) => refs(xs),
            Item::Edge(..) => vec![],
        };
        for r in rs {
            if matches!(r, Term::Var(_)) && !out.contains(&r) {
                out.push(r);
            }
        }
    }
    out
}
fn anchor(raw: &Term) -> usize {
    assert_eq!(raw.name(), "-");
    raw.args()[1].args()[1].name().parse().unwrap()
}
fn pair(r: &Term) -> (Term, Term) {
    (v(format!("s:{r:?}")), v(format!("n:{r:?}")))
}
fn convertible(t: &Term) -> bool {
    ["exactly", "geq", "greater", "leq", "less"].contains(&t.name())
}
fn operands(expr: &Term, into: &mut BTreeSet<Term>) {
    if matches!(expr, Term::Var(_)) {
        into.insert(expr.clone());
    } else if expr.name() == "expr" {
        operands(&expr.args()[1], into);
        operands(&expr.args()[2], into);
    }
}
pub fn witness_value(cmp: &str, n: i128, some: bool) -> i128 {
    if some {
        return 1;
    }
    match cmp {
        "exactly" => n,
        "geq" => {
            if n == 0 {
                1
            } else {
                n
            }
        }
        "leq" => {
            if n == 0 {
                -1
            } else {
                n
            }
        }
        "greater" => {
            if n + 1 == 0 {
                n + 2
            } else {
                n + 1
            }
        }
        "less" => {
            if n - 1 == 0 {
                n - 2
            } else {
                n - 1
            }
        }
        _ => panic!("operand witness {cmp}"),
    }
}
fn expand(
    items: &[Item],
    body: bool,
    operands: &BTreeSet<Term>,
    tokens: &[Term],
    targets: &BTreeMap<Term, (Term, Term)>,
) -> Vec<Goal> {
    let mut out = Vec::new();
    for item in items {
        let ts = match item {
            Item::Naf(xs) => {
                let inner = expand(xs, body, &BTreeSet::new(), tokens, targets)
                    .into_iter()
                    .map(|g| match g {
                        Goal::Pos(t) => t,
                        _ => panic!("fixture nested NAF"),
                    })
                    .collect();
                out.push(Goal::Naf(inner));
                continue;
            }
            Item::Edge(outer, inner, op, _) => vec![c(
                "guideline_operator",
                vec![outer.clone(), inner.clone(), a(op)],
            )],
            Item::Leaf(ctx, raw) => {
                let t = strip(raw);
                let x = t.args();
                match t.name() {
                    "object" => {
                        let mut ts = vec![lit(
                            ctx,
                            "entity",
                            vec![x[0].clone(), x[1].clone(), x[2].clone()],
                        )];
                        let converted = body && convertible(&x[4]);
                        let p = if converted {
                            pair(&x[0])
                        } else if let Some(p) = targets.get(&x[0]) {
                            p.clone()
                        } else {
                            (x[4].clone(), x[5].clone())
                        };
                        ts.push(lit(
                            ctx,
                            "cardinality",
                            vec![x[0].clone(), x[3].clone(), p.0.clone(), p.1.clone()],
                        ));
                        let some_operand =
                            operands.contains(&x[0]) && tokens[anchor(raw) - 1].name() == "some";
                        if converted && !some_operand {
                            ts.push(c(
                                "guideline_satisfies",
                                vec![p.0, p.1, x[4].clone(), x[5].clone()],
                            ));
                        }
                        ts
                    }
                    "predicate" => {
                        std::iter::once(lit(ctx, "event", vec![x[0].clone(), x[1].clone()]))
                            .chain(x[2..].iter().enumerate().map(|(i, r)| {
                                lit(ctx, "arg", vec![x[0].clone(), a(i + 1), r.clone()])
                            }))
                            .collect()
                    }
                    "modifier_pp" => vec![lit(ctx, "pp", x.to_vec())],
                    "property" => vec![lit(ctx, "property", x.to_vec())],
                    "formula" => vec![],
                    other => panic!("source-scene leaf {other}: {t:?}"),
                }
            }
        };
        out.extend(ts.into_iter().map(Goal::Pos));
    }
    out
}
fn chain(
    expr: &Term,
    lexemes: &mut std::slice::Iter<'_, &str>,
    goals: &mut Vec<Goal>,
    witness: &mut BTreeMap<Term, Term>,
    values: &BTreeMap<Term, Rat>,
    node: &mut usize,
) -> ((Term, Term), Rat) {
    if matches!(expr, Term::Var(_)) {
        return (pair(expr), values[expr]);
    }
    if ["int", "real"].contains(&expr.name()) {
        let value = Rat::decimal(lexemes.next().expect("literal lexeme"));
        return ((a("exactly"), value.term()), value);
    }
    assert_eq!(expr.name(), "expr");
    let x = expr.args();
    let (left, lval) = chain(&x[1], lexemes, goals, witness, values, node);
    let (right, rval) = chain(&x[2], lexemes, goals, witness, values, node);
    let result = if x[0].name() == "*" {
        lval.mul(rval)
    } else {
        assert_eq!(x[0].name(), "/");
        lval.div(rval).expect("admissible witness")
    };
    let output = (
        v(format!("chain:{}:s", *node)),
        v(format!("chain:{}:n", *node)),
    );
    *node += 1;
    goals.push(Goal::Pos(c(
        if x[0].name() == "*" {
            "guideline_product"
        } else {
            "guideline_quotient"
        },
        vec![
            left.0,
            left.1,
            right.0,
            right.1,
            output.0.clone(),
            output.1.clone(),
        ],
    )));
    witness.insert(output.0.clone(), a("exactly"));
    witness.insert(output.1.clone(), result.term());
    (output, result)
}
#[derive(Clone, Debug)]
pub struct Group {
    pub heads: Vec<Term>,
    pub body: Vec<Goal>,
    pub witness: BTreeMap<Term, Term>,
    pub variant: usize,
}
#[derive(Clone, Debug)]
pub struct Product {
    pub case: Case,
    pub groups: Vec<Group>,
}
impl Product {
    pub fn from_dump(case: &Case, dump: &str) -> Self {
        assert_eq!(
            envelope(dump, "messages"),
            Term::List(vec![]),
            "{}: APE messages",
            case.id
        );
        let drs = envelope(dump, "drs");
        let sentences = envelope(dump, "sentences");
        assert_eq!(sentences.args().len(), 1, "one-sentence model");
        let tokens = sentences.args()[0].args();
        let roots = leaves(&drs.args()[1]);
        assert_eq!(roots.len(), 1, "one rule scene {}", case.id);
        let rule = strip(&roots[0]);
        assert_eq!(rule.name(), "=>", "rule scene {}", case.id);
        let (mut ante, mut cons) = (rule.args()[0].clone(), rule.args()[1].clone());
        let (mut shared, mut deps) = (Vec::new(), Vec::new());
        loop {
            deps.extend(ante.args()[0].args().iter().cloned());
            shared.extend(leaves(&ante.args()[1]));
            let cs = leaves(&cons.args()[1]);
            if cons.args()[0].args().is_empty() && cs.len() == 1 && strip(&cs[0]).name() == "=>" {
                let r = strip(&cs[0]);
                ante = r.args()[0].clone();
                cons = r.args()[1].clone();
            } else {
                break;
            }
        }
        let disj = shared
            .iter()
            .position(|t| strip(t).name() == "v")
            .map(|i| shared.remove(i));
        let mut boxno = 1;
        let shared_items = flatten(&shared, &a("actual"), true, &case.id, &[], &mut boxno);
        let mut plans = Vec::new();
        if let Some(disj) = disj {
            let mut arms = Vec::new();
            for arm in strip(&disj).args() {
                arms.push((
                    arm.args()[0].args().to_vec(),
                    flatten(
                        arm.args()[1].args(),
                        &a("actual"),
                        true,
                        &case.id,
                        &[],
                        &mut boxno,
                    ),
                ));
            }
            for (dom, items) in arms {
                let mut ds = deps.clone();
                ds.extend(dom);
                let mut bs = shared_items.clone();
                bs.extend(items);
                let hs = flatten(
                    cons.args()[1].args(),
                    &a("actual"),
                    false,
                    &case.id,
                    &ds,
                    &mut boxno.clone(),
                );
                plans.push((bs, hs, ds));
            }
        } else {
            plans.push((
                shared_items,
                flatten(
                    cons.args()[1].args(),
                    &a("actual"),
                    false,
                    &case.id,
                    &deps,
                    &mut boxno,
                ),
                deps,
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
        let lexemes = numeric_lexemes(&case.ace);
        assert_eq!(
            lexemes.len(),
            tokens.iter().filter(|t| numeric_token(t)).count(),
            "accepted numeric correspondence {}",
            case.id
        );
        let groups = plans
            .into_iter()
            .enumerate()
            .map(|(i, (bs, hs, deps))| {
                let variant = i + 1;
                let bodyrefs = refs(&bs);
                let skolems: BTreeMap<_, _> = refs(&hs)
                    .into_iter()
                    .filter(|r| !bodyrefs.contains(r))
                    .map(|r| {
                        let sk = gid(
                            "product",
                            &case.id,
                            c("ref", vec![a(ordinal(&r))]),
                            Term::List(deps.clone()),
                        );
                        (r, sk)
                    })
                    .collect();
                let forms: Vec<_> = hs
                    .iter()
                    .filter_map(|item| match item {
                        Item::Leaf(ctx, raw)
                            if *ctx == a("actual") && strip(raw).name() == "formula" =>
                        {
                            Some(raw)
                        }
                        _ => None,
                    })
                    .collect();
                let mut used = BTreeSet::new();
                for f in &forms {
                    operands(&strip(f).args()[2], &mut used);
                }
                let mut witness = BTreeMap::new();
                let mut values = BTreeMap::new();
                for item in &bs {
                    match item {
                        Item::Edge(_, inner, _, n) => {
                            witness.insert(
                                inner.clone(),
                                gid(
                                    "witness",
                                    &case.id,
                                    c("box", vec![a(n)]),
                                    c("variant", vec![a(variant)]),
                                ),
                            );
                        }
                        Item::Leaf(_, raw) => {
                            for r in refs(std::slice::from_ref(item)) {
                                witness.insert(
                                    r.clone(),
                                    gid(
                                        "witness",
                                        &case.id,
                                        c("ref", vec![a(ordinal(&r))]),
                                        c("variant", vec![a(variant)]),
                                    ),
                                );
                            }
                            let t = strip(raw);
                            if t.name() == "object" && convertible(&t.args()[4]) {
                                let x = t.args();
                                let (s, n) = pair(&x[0]);
                                if used.contains(&x[0]) {
                                    let count = witness_value(
                                        x[4].name(),
                                        x[5].name().parse().unwrap(),
                                        tokens[anchor(raw) - 1].name() == "some",
                                    );
                                    let value = Rat::new(count, 1);
                                    values.insert(x[0].clone(), value);
                                    witness.insert(s, a("exactly"));
                                    witness.insert(n, value.term());
                                } else {
                                    witness.insert(s, x[4].clone());
                                    witness.insert(n, x[5].clone());
                                }
                            }
                        }
                        Item::Naf(_) => (),
                    }
                }
                let mut targets = BTreeMap::new();
                let mut chain_goals = Vec::new();
                let mut node = 0;
                for raw in forms {
                    let f = strip(raw);
                    let before = tokens[..anchor(raw) + 1]
                        .iter()
                        .filter(|t| numeric_token(t))
                        .count();
                    let mut literals = lexemes[before..].iter();
                    let (pair, _) = chain(
                        &f.args()[2],
                        &mut literals,
                        &mut chain_goals,
                        &mut witness,
                        &values,
                        &mut node,
                    );
                    assert!(targets.insert(f.args()[0].clone(), pair).is_none());
                }
                let mut body = expand(&bs, true, &used, tokens, &BTreeMap::new());
                body.extend(chain_goals);
                let heads = expand(&hs, false, &used, tokens, &targets)
                    .into_iter()
                    .map(|g| match g {
                        Goal::Pos(t) => t.sub(&skolems),
                        _ => panic!("NAF head"),
                    })
                    .collect();
                Group {
                    heads,
                    body,
                    witness,
                    variant,
                }
            })
            .collect();
        Self {
            case: case.clone(),
            groups,
        }
    }
    pub fn clauses(&self) -> String {
        let mut out = String::new();
        for g in &self.groups {
            for h in &g.heads {
                let mut body = g.body.clone();
                if self.case.table
                    && h.name() == "guideline_operator"
                    && ["should", "must", "may"].contains(&h.args()[2].name())
                {
                    body.push(Goal::Naf(vec![c(
                        "guideline_exclusion",
                        vec![a("da2-scope")],
                    )]));
                }
                let mut vars = BTreeMap::new();
                out.push_str(&h.text(&mut vars));
                if !body.is_empty() {
                    out.push_str(" :- ");
                    out.push_str(
                        &body
                            .iter()
                            .map(|g| g.text(&mut vars))
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                }
                out.push_str(".\n");
            }
        }
        out
    }
    pub fn document(&self) -> String {
        document(
            &self.case.id,
            &self.case.ace,
            Some(ULEX),
            self.case.table(),
            &self.clauses(),
        )
    }
    pub fn proof(&self) -> String {
        self.groups
            .iter()
            .map(|g| {
                let facts = g
                    .body
                    .iter()
                    .filter_map(|goal| match goal {
                        Goal::Pos(t)
                            if ![
                                "guideline_satisfies",
                                "guideline_product",
                                "guideline_quotient",
                            ]
                            .contains(&t.name()) =>
                        {
                            Some(t.sub(&g.witness))
                        }
                        _ => None,
                    })
                    .collect();
                let heads = g.heads.iter().map(|t| t.sub(&g.witness)).collect();
                format!(
                    "{}.\n",
                    c(
                        "$guideline_proof",
                        vec![
                            a(&self.case.id),
                            a(1),
                            c("variant", vec![a(g.variant)]),
                            c("witness", vec![Term::List(facts)]),
                            c("prove", vec![Term::List(heads)])
                        ]
                    )
                    .render()
                )
            })
            .collect()
    }
}
