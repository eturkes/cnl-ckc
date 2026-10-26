// S2 D2/D3 reference: pairwise row laws + pinned S1 projection + APE position plans.
use std::collections::{BTreeMap, BTreeSet};

// The term parser, printer + renamer shared by the S1 and S2 reference models.
include!("../compiler-model/term.rs");

pub const HEADER: &str = "# format: kind\tlemma\tvalue\n";
pub const CLASSES: [&str; 7] = [
    "usuality",
    "ideality",
    "focus",
    "manner",
    "force",
    "possibility",
    "applicability",
];
pub const DECLARATIONS: &str =
    ":- multifile(guideline_qualifier/4).\n:- discontiguous(guideline_qualifier/4).\n";

pub fn filtered(table: &str) -> String {
    let rows: String = table
        .lines()
        .skip(1)
        .filter(|r| !r.starts_with("qualifier\t"))
        .map(|r| format!("{r}\n"))
        .collect();
    format!(
        "{HEADER}{}",
        if rows.is_empty() {
            "unit\ts2-unused-s1-unit\tsecond\n"
        } else {
            &rows
        }
    )
}

fn lemma_ok(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b > 32 && b != 127)
}

fn noun<'a>(r: &[&'a str; 3]) -> Option<&'a str> {
    match r[0] {
        "unit" | "strength" | "evidence" => Some(r[1]),
        "spacing" | "window" => Some(r[2]),
        _ => None,
    }
}

fn conflict(a: &[&str; 3], b: &[&str; 3]) -> Option<&'static str> {
    let frame = |k| matches!(k, "spacing" | "window");
    let adj = |k| matches!(k, "approximation" | "preference" | "qualifier");
    let prep = |k| matches!(k, "relation" | "frequency" | "range");
    let temporal = |k| prep(k) || frame(k);
    if noun(a).is_some() && noun(a) == noun(b) && (!frame(a[0]) || !frame(b[0]) || a[1] == b[1]) {
        Some("duplicate noun")
    } else if a[0] == "qualifier" && b[0] == "qualifier" && a[1] == b[1] {
        Some("duplicate qualifier")
    } else if adj(a[0]) && adj(b[0]) && a[1] == b[1] {
        Some("duplicate adjective")
    } else if a[1] == b[1]
        && ((a[0] == "alternative" && (b[0] == "alternative" || temporal(b[0])))
            || (b[0] == "alternative" && temporal(a[0]))
            || (prep(a[0]) && prep(b[0])))
    {
        Some("duplicate preposition")
    } else {
        None
    }
}

pub fn table_verdict(bytes: &str) -> Result<(), String> {
    let body = bytes.strip_prefix(HEADER).ok_or("header")?;
    if body.is_empty() {
        return Err("no rows".into());
    }
    if !body.ends_with('\n') {
        return Err("final newline".into());
    }
    let mut prior = Vec::new();
    for (i, line) in body[..body.len() - 1].split('\n').enumerate() {
        let f: Vec<_> = line.split('\t').collect();
        let why = if line.starts_with('#') {
            Some("comment")
        } else if f.len() != 3 {
            Some("field count")
        } else if !lemma_ok(f[1]) {
            Some("lemma")
        } else {
            let r = [f[0], f[1], f[2]];
            let value = match r[0] {
                "unit"
                    if !["second", "minute", "hour", "day", "week", "month", "year"]
                        .contains(&r[2]) =>
                {
                    Some("unit id")
                }
                "relation" if !["duration", "within", "after", "before"].contains(&r[2]) => {
                    Some("role id")
                }
                "spacing" | "window" if !lemma_ok(r[2]) => Some("frame lemma"),
                "frequency" if r[2] != "period" => Some("frequency value"),
                "approximation" if r[2] != "about" => Some("approximation value"),
                "range" if r[2] != "minimum" => Some("range value"),
                "strength" if !["strong", "conditional"].contains(&r[2]) => Some("strength value"),
                "evidence" if !["high", "moderate", "low", "very_low"].contains(&r[2]) => {
                    Some("evidence value")
                }
                "preference" if r[2] != "preferred" => Some("preference value"),
                "alternative" if r[2] != "alternative" => Some("alternative value"),
                "qualifier" if !CLASSES.contains(&r[2]) => Some("qualifier value"),
                "unit" | "relation" | "spacing" | "window" | "frequency" | "approximation"
                | "range" | "strength" | "evidence" | "preference" | "alternative"
                | "qualifier" => None,
                _ => Some("kind"),
            };
            let duplicate =
                prior
                    .iter()
                    .filter_map(|p| conflict(p, &r))
                    .min_by_key(|why| match *why {
                        "duplicate noun" => 0,
                        "duplicate qualifier" => 1,
                        "duplicate adjective" => 2,
                        _ => 3,
                    });
            prior.push(r);
            value.or(duplicate)
        };
        if let Some(why) = why {
            return Err(format!("{why} at row {}", i + 1));
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Plan {
    pub sentence: usize,
    pub scope: String,
    pub verb: String,
    pub nth: usize,
    pub place: String,
    pub word: String,
    pub post_conditions: Vec<&'static str>,
}

pub fn plans(text: &str, drs: &str) -> Vec<Plan> {
    if text == "-" {
        return Vec::new();
    }
    let order = drs_order(drs);
    text.split(';')
        .map(|p| {
            let f: Vec<_> = p.split(':').collect();
            assert_eq!(f.len(), 6);
            Plan {
                sentence: f[0].parse().unwrap(),
                scope: f[1].into(),
                verb: f[2].into(),
                nth: f[3].parse().unwrap(),
                place: f[4].into(),
                word: f[5].into(),
                post_conditions: order.get(p).expect("DRS carrier plan").clone(),
            }
        })
        .collect()
}

fn qualifier(t: &Term) -> bool {
    t.args("guideline_qualifier").is_some()
}
fn semantic(line: &str) -> bool {
    line.starts_with("guideline_")
        && !line.starts_with("guideline_document(")
        && !line.starts_with("guideline_schema_version(")
}
fn record(line: &str) -> bool {
    line.starts_with("guideline_document(") || line.starts_with("'$guideline_query'(")
}
fn rows(table: Option<&str>) -> BTreeMap<String, String> {
    table
        .into_iter()
        .flat_map(|t| t.lines().skip(1))
        .filter_map(|line| {
            let f: Vec<_> = line.split('\t').collect();
            (f[0] == "qualifier").then(|| (f[1].into(), f[2].into()))
        })
        .collect()
}

fn annotation(ev: &[Term], class: &str, word: &str) -> Term {
    Term::App(
        "guideline_qualifier".into(),
        vec![ev[0].clone(), ev[1].clone(), atom(class), atom(word)],
    )
}

struct Added {
    before: Vec<Vec<Term>>,
    after: Vec<Vec<Term>>,
}
impl Added {
    fn empty(len: usize) -> Self {
        Self {
            before: vec![Vec::new(); len],
            after: vec![Vec::new(); len],
        }
    }
    fn expand(&self, input: &[Term]) -> Vec<Term> {
        input
            .iter()
            .enumerate()
            .flat_map(|(i, t)| {
                self.before[i]
                    .iter()
                    .cloned()
                    .chain(std::iter::once(t.clone()))
                    .chain(self.after[i].iter().cloned())
            })
            .collect()
    }
}

// The baseline supplies primitive identities; the plan supplies APE condition positions.
// Copula annotations are derived from retained properties, never a case output table.
fn additions(
    input: &[Term],
    table: &BTreeMap<String, String>,
    plan: &[&Plan],
    nouns: &BTreeSet<Term>,
) -> Added {
    let mut out = Added::empty(input.len());
    let events: Vec<_> = input
        .iter()
        .enumerate()
        .filter_map(|(i, t)| t.args("guideline_event").map(|e| (i, e)))
        .collect();
    let args = |ev: &[Term]| {
        input
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                t.args("guideline_arg")
                    .filter(|a| a[0] == ev[0] && a[1] == ev[1])
                    .map(|a| (i, a))
            })
            .collect::<Vec<_>>()
    };
    for (_, ev) in &events {
        if ev[2] != atom("be") {
            continue;
        }
        let av = args(ev);
        let Some((mut end, complement)) = av
            .iter()
            .find(|(_, a)| a[2] == Term::Atom("2".into()))
            .map(|(i, a)| (*i, &a[3]))
        else {
            continue;
        };
        if nouns.contains(complement) {
            continue;
        }
        while input
            .get(end + 1)
            .is_some_and(|t| t.args("guideline_grade").is_some())
        {
            end += 1;
        }
        for p in input.iter().filter_map(|t| t.args("guideline_property")) {
            if p[0] == ev[0]
                && p[1] == *complement
                && p[3] == atom("pos")
                && let Some(class) = table.get(&p[2].atom())
            {
                out.after[end].push(annotation(ev, class, &p[2].atom()));
            }
        }
    }
    for p in plan {
        let Some((index, ev)) = events
            .iter()
            .filter(|(_, ev)| ev[2].atom() == p.verb)
            .nth(p.nth - 1)
        else {
            continue;
        };
        let class = table.get(&p.word).expect("planned qualifier row");
        let q = annotation(ev, class, &p.word);
        match p.place.as_str() {
            "pre" => out.before[*index].push(q),
            "preprop" => {
                let av = args(ev);
                let complement = &av
                    .iter()
                    .find(|(_, a)| a[2] == Term::Atom("2".into()))
                    .unwrap()
                    .1[3];
                let start = input
                    .iter()
                    .position(|t| {
                        t.args("guideline_property")
                            .is_some_and(|a| a[0] == ev[0] && a[1] == *complement)
                    })
                    .unwrap();
                out.before[start].push(q);
            }
            "post" => {
                let starts: Vec<_> = input
                    .iter()
                    .enumerate()
                    .skip(*index)
                    .filter_map(|(i, t)| {
                        let Term::App(name, _) = t else {
                            return None;
                        };
                        [
                            "guideline_entity",
                            "guideline_event",
                            "guideline_pp",
                            "guideline_property",
                            "guideline_operator",
                        ]
                        .contains(&name.as_str())
                        .then_some((i, name.as_str()))
                    })
                    .collect();
                let count = p.post_conditions.len();
                assert_eq!(
                    starts[1..=count]
                        .iter()
                        .map(|(_, name)| *name)
                        .collect::<Vec<_>>(),
                    p.post_conditions,
                    "DRS conditions after {} before {}",
                    p.verb,
                    p.word
                );
                // The next primitive starts a new block; args, cardinality and every
                // time/strength annotation stay with the preceding DRS condition.
                let end = starts.get(count + 1).map_or(input.len(), |(i, _)| *i) - 1;
                out.after[end].push(q);
            }
            x => panic!("reference placement {x}"),
        }
    }
    out
}

fn body_terms(body: &[Goal]) -> Vec<Term> {
    body.iter()
        .flat_map(|g| match g {
            Goal::Pos(t) => vec![t.clone()],
            Goal::Naf(ts) => ts.clone(),
        })
        .collect()
}
fn nouns(terms: &[Term]) -> BTreeSet<Term> {
    terms
        .iter()
        .filter_map(|t| t.args("guideline_entity").map(|a| a[1].clone()))
        .collect()
}

fn annotated_body(
    body: &[Goal],
    table: &BTreeMap<String, String>,
    plan: &[Plan],
    sentence: usize,
    declared: &BTreeSet<Term>,
) -> Vec<Goal> {
    let positive: Vec<_> = body
        .iter()
        .filter_map(|g| match g {
            Goal::Pos(t) => Some(t.clone()),
            _ => None,
        })
        .collect();
    let selected: Vec<_> = plan
        .iter()
        .filter(|p| p.sentence == sentence && p.scope == "body")
        .collect();
    let add = additions(&positive, table, &selected, declared);
    let mut out = Vec::new();
    let mut i = 0;
    for g in body {
        match g {
            Goal::Pos(t) => {
                out.extend(add.before[i].iter().cloned().map(Goal::Pos));
                out.push(Goal::Pos(t.clone()));
                out.extend(add.after[i].iter().cloned().map(Goal::Pos));
                i += 1;
            }
            Goal::Naf(ts) => {
                let selected: Vec<_> = plan
                    .iter()
                    .filter(|p| p.sentence == sentence && p.scope == "naf")
                    .collect();
                out.push(Goal::Naf(
                    additions(ts, table, &selected, declared).expand(ts),
                ));
            }
        }
    }
    out
}

fn sentence(
    lines: &[&str],
    table: &BTreeMap<String, String>,
    plan: &[Plan],
    number: usize,
    global: &BTreeSet<Term>,
) -> String {
    let mut clauses: Vec<_> = lines.iter().map(|l| Clause::parse(l)).collect();
    for c in &mut clauses {
        c.canonicalize(true);
    }
    let mut groups: Vec<(Vec<Goal>, Vec<usize>)> = Vec::new();
    for (i, c) in clauses.iter().enumerate() {
        if let Some((_, indices)) = groups.iter_mut().find(|(b, _)| *b == c.body) {
            indices.push(i);
        } else {
            groups.push((c.body.clone(), vec![i]));
        }
    }
    let mut out = vec![String::new(); clauses.len()];
    for (body, indices) in groups {
        let heads: Vec<_> = indices.iter().map(|&i| clauses[i].head.clone()).collect();
        let mut declared = nouns(&[heads.clone(), body_terms(&body)].concat());
        declared.extend(global.iter().cloned());
        let selected: Vec<_> = plan
            .iter()
            .filter(|p| p.sentence == number && p.scope == "head")
            .collect();
        let add = additions(&heads, table, &selected, &declared);
        let body = annotated_body(&body, table, plan, number, &declared);
        for (i, &index) in indices.iter().enumerate() {
            for head in add.before[i]
                .iter()
                .chain(std::iter::once(&heads[i]))
                .chain(&add.after[i])
            {
                out[index].push_str(
                    &Clause {
                        head: head.clone(),
                        body: body.clone(),
                    }
                    .print(),
                );
            }
        }
    }
    out.concat()
}

fn proof_lists(proof: &Term) -> (&[Term], &[Term]) {
    let a = proof.args("'$guideline_proof'").unwrap();
    let list = |i: usize, name: &str| match &a[i].args(name).unwrap()[0] {
        Term::List(xs) => xs.as_slice(),
        _ => panic!("proof list"),
    };
    (list(3, "witness"), list(4, "prove"))
}
fn annotate_proof(text: &str, table: &BTreeMap<String, String>, plan: &[Plan]) -> String {
    let mut out = String::new();
    for l in text.lines() {
        let mut p = Term::parse(l.strip_suffix('.').unwrap());
        let s: usize = p.args("'$guideline_proof'").unwrap()[1]
            .atom()
            .parse()
            .unwrap();
        let (w, h) = proof_lists(&p);
        let declared = nouns(&[w, h].concat());
        let selected = |scope| {
            plan.iter()
                .filter(|p| p.sentence == s && p.scope == scope)
                .collect::<Vec<_>>()
        };
        let w = additions(w, table, &selected("body"), &declared).expand(w);
        let h = additions(h, table, &selected("head"), &declared).expand(h);
        let Term::App(_, a) = &mut p else {
            unreachable!()
        };
        a[3] = Term::App("witness".into(), vec![Term::List(w)]);
        a[4] = Term::App("prove".into(), vec![Term::List(h)]);
        out.push_str(&format!("{}.\n", p.print()));
    }
    out
}

pub fn annotate(text: &str, table: Option<&str>, plan: &[Plan]) -> String {
    let table = rows(table);
    if text.starts_with("'$guideline_proof'(") {
        return annotate_proof(text, &table, plan);
    }
    if text.starts_with("'$guideline_query'(") {
        return text.into();
    }
    let global: BTreeSet<_> = text
        .lines()
        .filter(|l| semantic(l))
        .filter_map(|l| {
            let c = Clause::parse(l);
            c.head
                .args("guideline_entity")
                .and_then(|a| matches!(a[1], Term::App(_, _)).then(|| a[1].clone()))
        })
        .collect();
    let lines: Vec<_> = text.lines().collect();
    let (mut i, mut number, mut out) = (0, 0, String::new());
    while i < lines.len() {
        if semantic(lines[i]) {
            let start = i;
            while i < lines.len() && semantic(lines[i]) {
                i += 1;
            }
            out.push_str(&sentence(&lines[start..i], &table, plan, number, &global));
        } else {
            if let Some(s) = lines[i].strip_prefix("% S") {
                number = s.split(':').next().unwrap().parse().unwrap();
            }
            out.push_str(lines[i]);
            out.push('\n');
            if lines[i] == ":- discontiguous(guideline_preference/4)." {
                out.push_str(DECLARATIONS);
            }
            i += 1;
        }
    }
    out
}

pub fn rebind(text: &str, ace: &str, ace_hash: &str, table_hash: Option<&str>) -> String {
    let sentences: Vec<_> = ace.lines().filter(|s| !s.is_empty()).collect();
    text.lines()
        .map(|line| {
            if record(line) {
                let mut r = Term::parse(line.strip_suffix('.').unwrap());
                let Term::App(_, args) = &mut r else {
                    unreachable!()
                };
                for a in args.iter_mut() {
                    if a.args("ace_sha256").is_some() {
                        *a = Term::App("ace_sha256".into(), vec![atom(ace_hash)]);
                    }
                }
                let digest = table_hash.map_or_else(
                    || atom("none"),
                    |h| Term::App("sha256".into(), vec![atom(h)]),
                );
                *args.last_mut().unwrap() = Term::App("vocabulary".into(), vec![digest]);
                format!("{}.\n", r.print())
            } else if let Some(s) = line.strip_prefix("% S") {
                let n: usize = s.split(':').next().unwrap().parse().unwrap();
                format!("% S{n}: {}\n", sentences[n - 1])
            } else {
                format!("{line}\n")
            }
        })
        .collect()
}

fn erase_lists(t: &mut Term) {
    match t {
        Term::App(_, xs) => xs.iter_mut().for_each(erase_lists),
        Term::List(xs) => {
            xs.retain(|x| !qualifier(x));
            xs.iter_mut().for_each(erase_lists);
        }
        _ => (),
    }
}

pub fn erase(text: &str, baseline: &str) -> String {
    let old_record = baseline.lines().find(|l| record(l));
    let comments: BTreeMap<_, _> = baseline
        .lines()
        .filter_map(|l| {
            l.strip_prefix("% S")
                .map(|s| (s.split(':').next().unwrap(), l))
        })
        .collect();
    let mut out = String::new();
    for line in text.lines() {
        if DECLARATIONS.lines().any(|d| d == line) {
            continue;
        }
        if line.starts_with("'$guideline_proof'(") {
            let mut p = Term::parse(line.strip_suffix('.').unwrap());
            erase_lists(&mut p);
            out.push_str(&format!("{}.\n", p.print()));
        } else if record(line) {
            out.push_str(old_record.unwrap());
            out.push('\n');
        } else if let Some(s) = line.strip_prefix("% S") {
            out.push_str(comments[s.split(':').next().unwrap()]);
            out.push('\n');
        } else if semantic(line) {
            let mut c = Clause::parse(line);
            if qualifier(&c.head) {
                continue;
            }
            c.body.retain_mut(|g| match g {
                Goal::Pos(t) => !qualifier(t),
                Goal::Naf(ts) => {
                    ts.retain(|t| !qualifier(t));
                    !ts.is_empty()
                }
            });
            out.push_str(&c.print());
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

pub fn drs_adverbs(text: &str) -> Vec<String> {
    fn visit(t: &Term, out: &mut Vec<String>) {
        if let Some(a) = t.args("modifier_adv") {
            out.push(a[1].atom());
        }
        match t {
            Term::App(_, xs) | Term::List(xs) => {
                for x in xs {
                    visit(x, out);
                }
            }
            _ => (),
        }
    }
    let mut out = Vec::new();
    let line = text.lines().find(|l| l.starts_with("drs(")).unwrap();
    visit(&Term::parse(line.strip_suffix('.').unwrap()), &mut out);
    out
}

pub fn drs_plans(text: &str) -> BTreeSet<String> {
    drs_order(text).into_keys().collect()
}

fn drs_order(text: &str) -> BTreeMap<String, Vec<&'static str>> {
    type Item = (usize, Term);
    type Scope = (String, Vec<Item>);
    fn flatten(t: &Term, scopes: &mut Vec<Scope>) -> Vec<Vec<Item>> {
        if let Some(a) = t.args("-")
            && a.len() == 2
            && let Some(anchor) = a[1].args("/")
        {
            return vec![vec![(anchor[0].atom().parse().unwrap(), a[0].clone())]];
        }
        if let Some(a) = t.args("=>") {
            let body = flatten(&a[0], scopes);
            scopes.extend(body.into_iter().map(|b| ("body".into(), b)));
            let head = flatten(&a[1], scopes);
            scopes.extend(head.into_iter().map(|b| ("head".into(), b)));
            return vec![Vec::new()];
        }
        if let Some(a) = t.args("~") {
            let groups = flatten(&a[0], scopes);
            scopes.extend(groups.into_iter().map(|b| ("naf".into(), b)));
            return vec![Vec::new()];
        }
        if let Some(a) = t.args("v") {
            return flatten(&a[0], scopes)
                .into_iter()
                .chain(flatten(&a[1], scopes))
                .collect();
        }
        if let Some(a) = t.args("drs") {
            if a.len() == 1 {
                return flatten(&a[0], scopes);
            }
            let Term::List(conds) = &a[1] else {
                panic!("DRS conditions")
            };
            let mut out = vec![Vec::new()];
            for c in conds {
                let next = flatten(c, scopes);
                out = out
                    .into_iter()
                    .flat_map(|left| {
                        next.iter()
                            .map(move |right| [left.clone(), right.clone()].concat())
                    })
                    .collect();
            }
            return out;
        }
        if let Term::App(op, a) = t
            && ["should", "must", "can", "may", "-", "question"].contains(&op.as_str())
        {
            let mut inside = flatten(&a[0], scopes);
            if op != "question" {
                for items in &mut inside {
                    if let Some((sentence, _)) = items.first() {
                        items.insert(0, (*sentence, Term::App("operator".into(), Vec::new())));
                    }
                }
            }
            return inside;
        }
        panic!("DRS condition outside fixture grammar: {t:?}");
    }
    let line = text.lines().find(|l| l.starts_with("drs(")).unwrap();
    let t = Term::parse(line.strip_suffix('.').unwrap());
    let mut scopes = Vec::new();
    let root = flatten(&t, &mut scopes);
    scopes.extend(root.into_iter().map(|b| ("head".into(), b)));
    let mut out = BTreeMap::new();
    for (scope, items) in scopes {
        for (i, (s, c)) in items.iter().enumerate() {
            let Some(adv) = c.args("modifier_adv") else {
                continue;
            };
            let (j, (_, pred)) = items
                .iter()
                .enumerate()
                .find(|(_, (n, p))| n == s && p.args("predicate").is_some_and(|p| p[0] == adv[0]))
                .unwrap();
            let pred = pred.args("predicate").unwrap();
            let nth = items[..=j]
                .iter()
                .filter(|(n, p)| n == s && p.args("predicate").is_some_and(|p| p[1] == pred[1]))
                .count();
            let place = if i < j {
                if pred[1] == atom("be")
                    && items[i + 1..j]
                        .iter()
                        .any(|(_, p)| p.args("property").is_some_and(|p| p[0] == pred[3]))
                {
                    "preprop"
                } else {
                    "pre"
                }
            } else {
                "post"
            };
            let key = format!(
                "{s}:{scope}:{}:{nth}:{place}:{}",
                pred[1].atom(),
                adv[1].atom()
            );
            let blocks = if place == "post" {
                items[j + 1..i]
                    .iter()
                    .filter_map(|(_, item)| {
                        let Term::App(name, _) = item else {
                            return None;
                        };
                        match name.as_str() {
                            "object" => Some("guideline_entity"),
                            "predicate" => Some("guideline_event"),
                            "modifier_pp" => Some("guideline_pp"),
                            "property" => Some("guideline_property"),
                            "operator" => Some("guideline_operator"),
                            "modifier_adv" => None,
                            other => panic!("DRS block outside fixture grammar: {other}"),
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            };
            if let Some(previous) = out.insert(key.clone(), blocks.clone()) {
                assert_eq!(previous, blocks, "variant-specific DRS post order: {key}");
            }
        }
    }
    out
}
