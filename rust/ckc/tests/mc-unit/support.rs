use super::model::{self, Binding, Clause, Goal, Literal, Predicate, Program, Term, empty_binding};
use super::numbers;
use ckc_kernel::{
    EMcBinding, EMcCertificate, EMcClaim, EMcInput, EMcItem, EMcOutcome, EMcReason, EMcStage, ESrc,
    EV1Verdict, contract,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

// The frozen oracle uses underscore-separated case symbols; v1 document names admit hyphens.
// This injective namespace renaming changes no clause, support, or domain member.
pub fn doc(p: &Program) -> String {
    p.id().replace('_', "-")
}
pub const ROOT_ID: &[u8] = b"mc-unit\0original-composition";
pub const INDICATORS: [(&str, usize); 22] = [
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Stage {
    Admission,
    Question,
    Identity,
    Coverage,
    Claim,
    Closure,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Item {
    Clause(usize, usize),
    Goal(usize),
    Row(usize),
    Composition,
    Fact(String),
    Claim(usize),
    Derivable(usize, String),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Input {
    Query(u8, Vec<u8>, Vec<u8>),
    Document(usize),
    Fact(usize),
    Model(usize),
    Claim(usize),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Observed {
    Verified,
    Answer(Vec<u8>),
    Refused(Stage, Item),
    Indeterminate(&'static str),
    Input(Input),
}

pub fn observe(r: Result<EMcOutcome, EMcInput>) -> Observed {
    match r {
        Ok(EMcOutcome::Verified) => Observed::Verified,
        Ok(EMcOutcome::VerifiedResult(b)) => Observed::Answer(b),
        Ok(EMcOutcome::Indeterminate(reason)) => Observed::Indeterminate(match reason {
            EMcReason::TermSize => "term-size",
            EMcReason::NumberSize => "number-size",
            EMcReason::Memory => "memory",
            EMcReason::Deadline => "deadline",
        }),
        Ok(EMcOutcome::Refused(s, i)) => Observed::Refused(
            match s {
                EMcStage::Admission => Stage::Admission,
                EMcStage::Question => Stage::Question,
                EMcStage::Identity => Stage::Identity,
                EMcStage::Coverage => Stage::Coverage,
                EMcStage::Claim => Stage::Claim,
                EMcStage::Closure => Stage::Closure,
            },
            match i {
                EMcItem::Clause { doc, ordinal } => Item::Clause(doc, ordinal),
                EMcItem::Goal(n) => Item::Goal(n),
                EMcItem::Row(n) => Item::Row(n),
                EMcItem::Composition => Item::Composition,
                EMcItem::Fact(b) => Item::Fact(text(b)),
                EMcItem::ClaimAt(n) => Item::Claim(n),
                EMcItem::Derivable { clause, head } => Item::Derivable(clause, text(head)),
            },
        ),
        Err(e) => Observed::Input(match e {
            EMcInput::Query(o) => Input::Query(o.rc, o.out, o.err),
            EMcInput::Document(n) => Input::Document(n),
            EMcInput::Fact(n) => Input::Fact(n),
            EMcInput::ModelLine(n) => Input::Model(n),
            EMcInput::Claim(n) => Input::Claim(n),
        }),
    }
}
fn text(b: Vec<u8>) -> String {
    String::from_utf8(b).expect("canonical UTF-8 output")
}

#[derive(Clone, Debug)]
pub struct Claim {
    pub fact: String,
    pub clause: usize,
    pub valuation: Vec<(usize, String)>,
    pub level: u64,
}
impl Claim {
    pub fn fact(fact: &str, clause: usize) -> Self {
        Self {
            fact: fact.to_owned(),
            clause,
            valuation: vec![],
            level: 0,
        }
    }
    pub fn exec(&self) -> EMcClaim {
        EMcClaim {
            fact: self.fact.as_bytes().to_vec(),
            clause: self.clause,
            valuation: self
                .valuation
                .iter()
                .map(|(var, term)| EMcBinding {
                    var: *var,
                    term: term.as_bytes().to_vec(),
                })
                .collect(),
            level: self.level,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Case {
    pub docs: Vec<Vec<u8>>,
    pub facts: Vec<Vec<u8>>,
    pub patient: usize,
    pub identity: Vec<u8>,
    pub slice: Option<Vec<usize>>,
    pub model: Vec<String>,
    pub cert_id: Vec<u8>,
    pub claims: Vec<Claim>,
}
impl Case {
    pub fn new(p: &Program, truth: &BTreeSet<Literal>, alternate: bool, stretch: u64) -> Self {
        Self {
            docs: vec![document(p)],
            facts: vec![],
            patient: 1,
            identity: ROOT_ID.to_vec(),
            slice: None,
            model: truth.iter().map(|f| f.render(&doc(p))).collect(),
            cert_id: ROOT_ID.to_vec(),
            claims: certificate(p, truth, alternate, stretch),
        }
    }
    pub fn cert(&self) -> EMcCertificate {
        EMcCertificate {
            identity: self.cert_id.clone(),
            claims: self.claims.iter().map(Claim::exec).collect(),
        }
    }
    pub fn check(&self) -> Observed {
        observe(contract::mc_check_model(
            &self.docs,
            &self.facts,
            self.patient,
            &self.identity,
            &self.slice,
            &self.model_bytes(),
            &self.cert(),
        ))
    }
    pub fn answer(&self, q: ESrc) -> Observed {
        let all;
        let ids = if let Some(s) = &self.slice {
            s
        } else {
            all = self.all_ids();
            &all
        };
        observe(contract::mc_answer(
            &self.docs,
            &self.facts,
            self.patient,
            &self.identity,
            ids,
            &self.model_bytes(),
            &self.cert(),
            &q,
        ))
    }
    pub fn all_ids(&self) -> Vec<usize> {
        let n = self
            .docs
            .iter()
            .map(|d| clause_lines(d).len())
            .sum::<usize>()
            + self.facts.len();
        (0..n).collect()
    }
    pub fn model_bytes(&self) -> Vec<Vec<u8>> {
        self.model.iter().map(|s| s.as_bytes().to_vec()).collect()
    }
    pub fn position(&self, f: &str) -> usize {
        self.claims
            .iter()
            .position(|c| c.fact == f)
            .expect("named claim")
    }
    pub fn remove(&mut self, f: &str) {
        let before = self.model.len();
        self.model.retain(|x| x != f);
        self.claims.retain(|c| c.fact != f);
        assert_eq!(before, self.model.len() + 1);
    }
}

#[track_caller]
pub fn canonical(bytes: &[u8]) {
    if let EV1Verdict::Reject { at } = contract::v1_check(bytes) {
        panic!(
            "generated v1 file noncanonical at {at}: {}",
            String::from_utf8_lossy(bytes)
        );
    }
}
pub fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
pub fn clause_lines(bytes: &[u8]) -> Vec<String> {
    std::str::from_utf8(bytes)
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('%') && !s.starts_with(":-"))
        .map(|s| s.strip_suffix('.').expect("one clause per line").to_owned())
        .collect()
}

fn variable_spans(s: &str) -> Vec<(usize, usize)> {
    let b = s.as_bytes();
    let (mut out, mut i) = (vec![], 0);
    while i < b.len() {
        if b[i] == b'\'' {
            i += 1;
            while i < b.len() {
                if b[i] == b'\\' {
                    i += 2;
                } else if b[i] == b'\'' {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
        } else if b[i].is_ascii_alphanumeric() || b[i] == b'_' {
            let start = i;
            i += 1;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            if b[start].is_ascii_uppercase() {
                out.push((start, i));
            }
        } else {
            i += 1;
        }
    }
    out
}
pub fn rename(s: &str, singletons: bool) -> (String, BTreeMap<String, usize>) {
    let spans = variable_spans(s);
    let mut counts = BTreeMap::new();
    let mut ids = BTreeMap::new();
    for (a, b) in &spans {
        let word = &s[*a..*b];
        *counts.entry(word).or_insert(0) += 1;
        let next = ids.len();
        ids.entry(word.to_owned()).or_insert(next);
    }
    let (mut out, mut at) = (String::new(), 0);
    for (a, b) in spans {
        let word = &s[a..b];
        let n = ids[word];
        out.push_str(&s[at..a]);
        if singletons && counts[word] == 1 {
            out.push('_');
        } else {
            out.push(char::from(b'A' + (n % 26) as u8));
            if n >= 26 {
                out.push_str(&(n / 26).to_string());
            }
        }
        at = b;
    }
    out.push_str(&s[at..]);
    (out, ids)
}
fn goals_text(goals: &[Goal], doc: &str) -> String {
    goals
        .iter()
        .map(|g| goal_text(g, doc))
        .collect::<Vec<_>>()
        .join(", ")
}
pub fn goal_text(g: &Goal, doc: &str) -> String {
    match g {
        Goal::Negative(gs) if gs.len() == 1 => format!("\\+ {}", goal_text(&gs[0], doc)),
        Goal::Negative(gs) => format!("\\+ ({})", goals_text(gs, doc)),
        _ => g.render(doc),
    }
}
pub fn clause_text(c: &Clause, doc: &str) -> String {
    if c.body.is_empty() {
        c.head.render(doc)
    } else {
        format!("{} :- {}", c.head.render(doc), goals_text(&c.body, doc))
    }
}
pub fn document(p: &Program) -> Vec<u8> {
    let lines = p
        .clauses
        .iter()
        .skip(2)
        .map(|c| rename(&clause_text(c, &doc(p)), true).0)
        .collect::<Vec<_>>();
    let mut s = prefix(&doc(p), &p.clauses[1].head.render(&doc(p)));
    s.push_str("% S1: Model-checker semantic fixture.\n");
    for line in lines {
        s.push_str(&line);
        s.push_str(".\n");
    }
    let bytes = s.into_bytes();
    canonical(&bytes);
    bytes
}
fn prefix(doc: &str, record: &str) -> String {
    let mut out = format!(
        "% {doc}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n"
    );
    for (name, arity) in INDICATORS {
        out.push_str(&format!(
            ":- multifile({name}/{arity}).\n:- discontiguous({name}/{arity}).\n"
        ));
    }
    out.push_str("guideline_schema_version(2).\n");
    out.push_str(record);
    out.push_str(".\n");
    out
}
pub fn raw_document(doc: &str, lines: &[&str]) -> Vec<u8> {
    let record = format!(
        "guideline_document({},ace_sha256('{}'),ulex(none),vocabulary(none))",
        model::quote(doc),
        "0".repeat(64)
    );
    let mut out = prefix(doc, &record);
    out.push_str("% S1: Model-checker admission fixture.\n");
    for line in lines {
        out.push_str(&rename(line.trim_end_matches('.'), true).0);
        out.push_str(".\n");
    }
    let bytes = out.into_bytes();
    canonical(&bytes);
    bytes
}
pub fn raw_case(lines: &[&str]) -> Case {
    Case {
        docs: vec![raw_document("mc-raw", lines)],
        facts: vec![],
        patient: 1,
        identity: ROOT_ID.to_vec(),
        slice: None,
        model: vec![],
        cert_id: ROOT_ID.to_vec(),
        claims: vec![],
    }
}
pub fn query_text(goal: &str, rows: &str) -> Vec<u8> {
    let projection = rename(
        &format!("'$guideline_query_projection'(goal({goal}),answers({rows}))"),
        false,
    )
    .0;
    let out = format!("% q compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v2,q,ace_sha256('{}'),ulex(none),vocabulary(none)).\n% Q1: Which model values hold?\n{projection}.\n", "0".repeat(64)).into_bytes();
    canonical(&out);
    out
}
pub fn query(goals: &[Goal], projection: &[Term], doc: &str) -> ESrc {
    fn conjunction(gs: &[Goal], doc: &str) -> String {
        if gs.len() == 1 {
            goal_text(&gs[0], doc)
        } else {
            format!(
                "','({},{})",
                goal_text(&gs[0], doc),
                conjunction(&gs[1..], doc)
            )
        }
    }
    assert!(!goals.is_empty());
    let rows = format!(
        "[{}]",
        projection
            .iter()
            .map(|t| format!("answer({},wh(what))", t.render(doc)))
            .collect::<Vec<_>>()
            .join(",")
    );
    ESrc::Bytes(query_text(&conjunction(goals, doc), &rows))
}

// Relational reference: schema joins + functional arithmetic; each NAF keeps one correlated
// substitution for its whole conjunction. No production checker/spec predicate is called.
pub fn join(
    goals: &[Goal],
    initial: &Binding,
    m: &BTreeSet<Literal>,
) -> Vec<(Binding, Vec<Literal>)> {
    let mut rows = vec![(initial.clone(), vec![])];
    for goal in goals {
        let mut next = vec![];
        for (env, positive) in rows {
            match goal {
                Goal::Positive(pattern) => {
                    for fact in m {
                        let mut e = env.clone();
                        if pattern.match_ground(fact, &mut e) {
                            let mut ps = positive.clone();
                            ps.push(fact.clone());
                            next.push((e, ps));
                        }
                    }
                }
                Goal::Builtin(kind, args) => {
                    if let Some(e) = numbers::apply(*kind, args, &env) {
                        next.push((e, positive));
                    }
                }
                Goal::Negative(gs) => {
                    if join(gs, &env, m).is_empty() {
                        next.push((env, positive));
                    }
                }
            }
        }
        rows = next;
    }
    rows
}
fn nonlocals(c: &Clause) -> BTreeSet<usize> {
    let mut all = BTreeSet::new();
    c.head.variables(&mut all);
    for g in &c.body {
        g.variables(&mut all);
    }
    all.into_iter()
        .filter(|x| {
            let mut head = BTreeSet::new();
            c.head.variables(&mut head);
            head.contains(x)
                || c.body.iter().any(|g| {
                    !matches!(g, Goal::Negative(_)) && {
                        let mut vs = BTreeSet::new();
                        g.variables(&mut vs);
                        vs.contains(x)
                    }
                })
                || c.body
                    .iter()
                    .filter(|g| {
                        let mut vs = BTreeSet::new();
                        g.variables(&mut vs);
                        vs.contains(x)
                    })
                    .count()
                    > 1
        })
        .collect()
}
pub fn certificate(
    p: &Program,
    m: &BTreeSet<Literal>,
    alternate: bool,
    stretch: u64,
) -> Vec<Claim> {
    assert!(stretch >= 1);
    let mut found: BTreeMap<Literal, Claim> = BTreeMap::new();
    while found.len() != m.len() {
        let before = found.len();
        for fact in m {
            if found.contains_key(fact) {
                continue;
            }
            let mut choices = vec![];
            for (ci, c) in p.clauses.iter().enumerate() {
                for (env, positive) in join(&c.body, &empty_binding(), m) {
                    if c.head.substitute(&env).as_ref() != Some(fact)
                        || !positive.iter().all(|f| found.contains_key(f))
                    {
                        continue;
                    }
                    let level = if c.body.is_empty() {
                        0
                    } else {
                        positive.iter().map(|f| found[f].level).max().unwrap_or(0) + stretch
                    };
                    let names = rename(&clause_text(c, &doc(p)), true).1;
                    let mut valuation = nonlocals(c)
                        .iter()
                        .map(|v| {
                            let name = Term::Var(*v).render(&doc(p));
                            (
                                names[&name],
                                env[*v]
                                    .as_ref()
                                    .expect("safe non-local binding")
                                    .render(&doc(p)),
                            )
                        })
                        .collect::<Vec<_>>();
                    valuation.sort_by_key(|(i, _)| *i);
                    choices.push(Claim {
                        fact: fact.render(&doc(p)),
                        clause: ci,
                        valuation,
                        level,
                    });
                }
            }
            let winner = if alternate {
                choices.pop()
            } else {
                choices.into_iter().next()
            };
            if let Some(claim) = winner {
                found.insert(fact.clone(), claim);
            }
        }
        assert!(
            found.len() > before,
            "oracle model has no founded certificate: {} {m:?}",
            p.id()
        );
    }
    found.into_values().collect()
}

pub fn stable_reference(p: &Program, m: &BTreeSet<Literal>) -> bool {
    let mut least = BTreeSet::new();
    loop {
        let before = least.len();
        for c in &p.clauses {
            let mut rows = vec![empty_binding()];
            for g in &c.body {
                let mut next = vec![];
                for env in rows {
                    match g {
                        Goal::Positive(pattern) => {
                            for f in &least {
                                let mut e = env.clone();
                                if pattern.match_ground(f, &mut e) {
                                    next.push(e);
                                }
                            }
                        }
                        Goal::Builtin(kind, args) => {
                            if let Some(e) = numbers::apply(*kind, args, &env) {
                                next.push(e);
                            }
                        }
                        Goal::Negative(gs) => {
                            if join(gs, &env, m).is_empty() {
                                next.push(env);
                            }
                        }
                    }
                }
                rows = next;
            }
            for env in rows {
                least.insert(c.head.substitute(&env).expect("safe reduct head"));
            }
        }
        if least.len() == before {
            return least == *m;
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Standard {
    Integer(i128),
    Atom(String),
    Compound(String, Vec<Standard>),
}
fn standard(t: &Term, doc: &str) -> Standard {
    match t {
        Term::Var(_) => panic!("ground result"),
        Term::Atom(s) => Standard::Atom(s.clone()),
        Term::Document => Standard::Atom(doc.to_owned()),
        Term::Number(n) if n.denominator == 1 => Standard::Integer(n.numerator),
        Term::Number(n) => Standard::Compound(
            "rational".into(),
            vec![
                Standard::Integer(n.numerator),
                Standard::Integer(n.denominator),
            ],
        ),
        Term::Compound(n, ts) => {
            Standard::Compound((*n).into(), ts.iter().map(|t| standard(t, doc)).collect())
        }
        Term::Identity(slot, deps) => {
            let mut list = Standard::Atom("[]".into());
            for d in deps.iter().rev() {
                list = Standard::Compound("[|]".into(), vec![standard(d, doc), list]);
            }
            Standard::Compound(
                "$guideline_id".into(),
                vec![
                    Standard::Atom("product".into()),
                    Standard::Atom(doc.into()),
                    Standard::Integer(1),
                    Standard::Compound("ref".into(), vec![Standard::Integer(i128::from(*slot))]),
                    list,
                ],
            )
        }
    }
}
impl Ord for Standard {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        use Standard::*;
        match (self, rhs) {
            (Integer(a), Integer(b)) => a.cmp(b),
            (Integer(_), _) => std::cmp::Ordering::Less,
            (_, Integer(_)) => std::cmp::Ordering::Greater,
            (Atom(a), Atom(b)) => a.cmp(b),
            (Atom(_), _) => std::cmp::Ordering::Less,
            (_, Atom(_)) => std::cmp::Ordering::Greater,
            (Compound(a, xs), Compound(b, ys)) => xs
                .len()
                .cmp(&ys.len())
                .then_with(|| a.cmp(b))
                .then_with(|| xs.cmp(ys)),
        }
    }
}
impl PartialOrd for Standard {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
pub fn answer_oracle(
    m: &BTreeSet<Literal>,
    goals: &[Goal],
    projection: &[Term],
    doc: &str,
) -> Vec<u8> {
    let rows = join(goals, &empty_binding(), m);
    if projection.is_empty() {
        return if rows.is_empty() {
            b"no(finite_failure)".to_vec()
        } else {
            b"yes".to_vec()
        };
    }
    let mut results = rows
        .into_iter()
        .map(|(env, _)| {
            projection
                .iter()
                .map(|t| t.substitute(&env).expect("admitted projection"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    results.sort_by_cached_key(|r| r.iter().map(|t| standard(t, doc)).collect::<Vec<_>>());
    results.dedup();
    format!(
        "solutions([{}])",
        results
            .iter()
            .map(|r| format!(
                "sol([{}])",
                r.iter()
                    .map(|t| t.render(doc))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
    .into_bytes()
}
pub fn unary(p: Predicate, t: Term) -> Literal {
    model::literal(p, vec![t])
}
pub fn pos(p: Predicate, t: Term) -> Goal {
    Goal::Positive(unary(p, t))
}
pub fn program(extra: Vec<Clause>) -> Program {
    let mut p = Program::new("mc", 0, &[model::atom("a"), model::atom("b")]);
    p.clauses.extend(extra);
    p
}
pub fn good(extra: Vec<Clause>) -> (Program, Case) {
    let p = program(extra);
    let m = super::oracle::evaluate(&p).truth;
    let c = Case::new(&p, &m, false, 1);
    (p, c)
}
#[track_caller]
pub fn refusal(case: &Case, stage: Stage, items: &[Item]) {
    let first = case.check();
    let second = case.check();
    assert_eq!(first, second, "D9 deterministic item");
    assert!(
        matches!(&first, Observed::Refused(s, i) if *s == stage && items.contains(i)),
        "expected {stage:?} in {items:?}, got {first:?}"
    );
}
#[track_caller]
pub fn verified(case: &Case) {
    assert_eq!(case.check(), Observed::Verified);
}
