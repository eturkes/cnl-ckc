// S1 D1–D6 reference: C1 bytes + contract insertions; no implementation calls.
use std::collections::BTreeMap;

pub const HEADER: &str = "# format: kind\tlemma\tvalue\n";
pub const INERT: &str = "unit\ts1-unused-c1-unit\tsecond\n";
pub const NEW_KINDS: [&str; 4] = ["strength", "evidence", "preference", "alternative"];
const INDICATORS: [(&str, usize); 17] = [
    ("schema_version", 1),
    ("document", 4),
    ("entity", 4),
    ("cardinality", 5),
    ("event", 3),
    ("arg", 4),
    ("pp", 4),
    ("property", 4),
    ("operator", 3),
    ("interval", 6),
    ("recurrence", 4),
    ("frequency", 5),
    ("order", 4),
    ("recurrence_window", 7),
    ("range", 3),
    ("grade", 5),
    ("preference", 4),
];

pub fn declarations() -> String {
    INDICATORS
        .iter()
        .map(|(name, n)| {
            format!(
                ":- multifile(guideline_{name}/{n}).\n:- discontiguous(guideline_{name}/{n}).\n"
            )
        })
        .collect()
}

fn new_declarations() -> String {
    declarations()
        .lines()
        .skip(30)
        .map(|s| format!("{s}\n"))
        .collect()
}

pub fn filtered(table: &str) -> String {
    let rows: String = table
        .lines()
        .skip(1)
        .filter(|r| !NEW_KINDS.contains(&r.split('\t').next().unwrap()))
        .map(|r| format!("{r}\n"))
        .collect();
    format!("{HEADER}{}", if rows.is_empty() { INERT } else { &rows })
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

// Pairwise namespace conflicts make row-order symmetry independent of the implementation.
fn conflict(a: &[&str; 3], b: &[&str; 3]) -> Option<&'static str> {
    let is_frame = |k| matches!(k, "spacing" | "window");
    let is_adj = |k| matches!(k, "approximation" | "preference");
    let is_time = |k| matches!(k, "relation" | "frequency" | "range" | "spacing" | "window");
    let is_unique_prep = |k| matches!(k, "relation" | "frequency" | "range");
    if noun(a).is_some()
        && noun(a) == noun(b)
        && (!is_frame(a[0]) || !is_frame(b[0]) || a[1] == b[1])
    {
        Some("duplicate noun")
    } else if is_adj(a[0]) && is_adj(b[0]) && a[1] == b[1] {
        Some("duplicate adjective")
    } else if a[1] == b[1]
        && ((a[0] == "alternative" && (b[0] == "alternative" || is_time(b[0])))
            || (b[0] == "alternative" && is_time(a[0]))
            || (is_unique_prep(a[0]) && is_unique_prep(b[0])))
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
        let fields: Vec<_> = line.split('\t').collect();
        let why = if line.starts_with('#') {
            Some("comment")
        } else if fields.len() != 3 {
            Some("field count")
        } else if !lemma_ok(fields[1]) {
            Some("lemma")
        } else {
            let row = [fields[0], fields[1], fields[2]];
            let value_error = match row[0] {
                "unit"
                    if !["second", "minute", "hour", "day", "week", "month", "year"]
                        .contains(&row[2]) =>
                {
                    Some("unit id")
                }
                "relation" if !["duration", "within", "after", "before"].contains(&row[2]) => {
                    Some("role id")
                }
                "spacing" | "window" if !lemma_ok(row[2]) => Some("frame lemma"),
                "frequency" if row[2] != "period" => Some("frequency value"),
                "approximation" if row[2] != "about" => Some("approximation value"),
                "range" if row[2] != "minimum" => Some("range value"),
                "strength" if !["strong", "conditional"].contains(&row[2]) => {
                    Some("strength value")
                }
                "evidence" if !["high", "moderate", "low", "very_low"].contains(&row[2]) => {
                    Some("evidence value")
                }
                "preference" if row[2] != "preferred" => Some("preference value"),
                "alternative" if row[2] != "alternative" => Some("alternative value"),
                "unit" | "relation" | "spacing" | "window" | "frequency" | "approximation"
                | "range" | "strength" | "evidence" | "preference" | "alternative" => None,
                _ => Some("kind"),
            };
            let duplicate = prior
                .iter()
                .filter_map(|p| conflict(p, &row))
                .min_by_key(|why| match *why {
                    "duplicate noun" => 0,
                    "duplicate adjective" => 1,
                    _ => 2,
                });
            prior.push(row);
            value_error.or(duplicate)
        };
        if let Some(why) = why {
            return Err(format!("{why} at row {}", i + 1));
        }
    }
    Ok(())
}

// The term parser, printer + renamer shared by the S1 and S2 reference models.
include!("../compiler-model/term.rs");

fn annotation(t: &Term) -> bool {
    t.args("guideline_grade").is_some() || t.args("guideline_preference").is_some()
}

fn rows(table: Option<&str>) -> Vec<[&str; 3]> {
    table
        .into_iter()
        .flat_map(|t| t.lines().skip(1))
        .map(|l| {
            let f: Vec<_> = l.split('\t').collect();
            [f[0], f[1], f[2]]
        })
        .collect()
}

fn scope_additions(
    scope: &[Term],
    table: &[[&str; 3]],
    facts: bool,
    nouns: &BTreeMap<Term, String>,
) -> Vec<Vec<Term>> {
    let mut additions = vec![Vec::new(); scope.len()];
    for ev in scope.iter().filter_map(|t| t.args("guideline_event")) {
        let args: Vec<_> = scope
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                t.args("guideline_arg")
                    .filter(|a| a[0] == ev[0] && a[1] == ev[1])
                    .map(|a| (i, a))
            })
            .collect();
        let arg = |n: &str| {
            args.iter()
                .find(|(_, a)| a[2].atom() == n)
                .map(|(i, a)| (*i, a[3].clone()))
        };
        let alts: Vec<_> = scope
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                t.args("guideline_pp")
                    .filter(|a| {
                        a[0] == ev[0]
                            && a[1] == ev[1]
                            && table
                                .iter()
                                .any(|r| r[0] == "alternative" && r[1] == a[2].atom())
                    })
                    .map(|a| (i, a[3].clone()))
            })
            .collect();
        let a1 = arg("1");
        let a2 = arg("2");
        if facts
            && ev[0] == atom("actual")
            && ev[2] == atom("be")
            && let (Some((_, r)), Some((i, label))) = (&a1, &a2)
            && let Some(noun) = nouns.get(label)
        {
            for row in table
                .iter()
                .filter(|r| matches!(r[0], "strength" | "evidence") && r[1] == noun)
            {
                additions[*i].push(Term::App(
                    "guideline_grade".into(),
                    vec![
                        ev[0].clone(),
                        r.clone(),
                        atom(row[0]),
                        atom(row[2]),
                        label.clone(),
                    ],
                ));
            }
        }
        let copula = ev[2] == atom("be")
            && a2.as_ref().is_some_and(|(_, a2)| {
                scope
                    .iter()
                    .filter_map(|t| t.args("guideline_property"))
                    .any(|p| {
                        p[0] == ev[0]
                            && p[1] == *a2
                            && p[3] == atom("pos")
                            && table
                                .iter()
                                .any(|r| r[0] == "preference" && r[1] == p[2].atom())
                    })
            });
        if !copula && alts.is_empty() {
            continue;
        }
        assert!(alts.len() <= 1, "green reference has several alternatives");
        let preferred = if copula { a1.as_ref() } else { a2.as_ref() }
            .expect("green preferred endpoint")
            .1
            .clone();
        let (i, alternative) = alts
            .first()
            .cloned()
            .unwrap_or_else(|| (a2.as_ref().unwrap().0, atom("none")));
        additions[i].push(Term::App(
            "guideline_preference".into(),
            vec![ev[0].clone(), ev[1].clone(), preferred, alternative],
        ));
    }
    additions
}

fn counts(additions: &[Vec<Term>]) -> (usize, usize) {
    let grades = additions
        .iter()
        .flatten()
        .filter(|t| t.args("guideline_grade").is_some())
        .count();
    let preferences = additions
        .iter()
        .flatten()
        .filter(|t| t.args("guideline_preference").is_some())
        .count();
    (grades, preferences)
}

fn expand(scope: &[Term], additions: &[Vec<Term>]) -> Vec<Term> {
    scope
        .iter()
        .zip(additions)
        .flat_map(|(t, more)| std::iter::once(t.clone()).chain(more.iter().cloned()))
        .collect()
}

fn annotated_body(body: &[Goal], table: &[[&str; 3]]) -> (Vec<Goal>, usize) {
    let positive: Vec<_> = body
        .iter()
        .filter_map(|g| match g {
            Goal::Pos(t) => Some(t.clone()),
            _ => None,
        })
        .collect();
    let additions = scope_additions(&positive, table, false, &BTreeMap::new());
    let mut preference_count = counts(&additions).1;
    let mut result = Vec::new();
    let mut p = 0;
    for goal in body {
        match goal {
            Goal::Pos(t) => {
                result.push(Goal::Pos(t.clone()));
                result.extend(additions[p].iter().cloned().map(Goal::Pos));
                p += 1;
            }
            Goal::Naf(xs) => {
                let more = scope_additions(xs, table, false, &BTreeMap::new());
                preference_count += counts(&more).1;
                result.push(Goal::Naf(expand(xs, &more)));
            }
        }
    }
    (result, preference_count)
}

fn sentence(
    lines: &[&str],
    table: &[[&str; 3]],
    nouns: &BTreeMap<Term, String>,
) -> (String, usize, usize) {
    let mut clauses: Vec<_> = lines.iter().map(|s| Clause::parse(s)).collect();
    let facts = clauses.iter().all(|c| c.body.is_empty());
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
    let mut output = vec![String::new(); clauses.len()];
    let (mut grade_count, mut preference_count) = (0, 0);
    for (body, indices) in groups {
        let heads: Vec<_> = indices.iter().map(|&i| clauses[i].head.clone()).collect();
        let additions = scope_additions(&heads, table, facts, nouns);
        let (g, p) = counts(&additions);
        grade_count += g;
        preference_count += p;
        let (new_body, bp) = annotated_body(&body, table);
        preference_count += bp;
        for (slot, &index) in indices.iter().enumerate() {
            for head in std::iter::once(&heads[slot]).chain(additions[slot].iter()) {
                output[index].push_str(
                    &Clause {
                        head: head.clone(),
                        body: new_body.clone(),
                    }
                    .print(),
                );
            }
        }
    }
    (output.concat(), grade_count, preference_count)
}

fn is_clause(line: &str) -> bool {
    line.starts_with("guideline_")
        && !line.starts_with("guideline_document(")
        && !line.starts_with("guideline_schema_version(")
}

fn proof_lists(proof: &Term) -> (&[Term], &[Term]) {
    let args = proof.args("'$guideline_proof'").unwrap();
    let list = |i: usize, name: &str| match &args[i].args(name).unwrap()[0] {
        Term::List(xs) => xs.as_slice(),
        _ => panic!("reference proof list"),
    };
    (list(3, "witness"), list(4, "prove"))
}

fn annotate_proof(baseline: &str, table: &[[&str; 3]]) -> (String, usize, usize) {
    let proofs: Vec<_> = baseline
        .lines()
        .map(|l| Term::parse(l.strip_suffix('.').unwrap()))
        .collect();
    let nouns = proofs
        .iter()
        .flat_map(|p| {
            let (witness, prove) = proof_lists(p);
            witness.iter().chain(prove)
        })
        .filter_map(|t| {
            t.args("guideline_entity")
                .map(|a| (a[1].clone(), a[2].atom()))
        })
        .collect();
    let (mut output, mut grades, mut preferences) = (String::new(), 0, 0);
    for mut proof in proofs {
        let (witness, prove) = proof_lists(&proof);
        let wa = scope_additions(witness, table, false, &nouns);
        let pa = scope_additions(prove, table, witness.is_empty(), &nouns);
        grades += counts(&pa).0;
        preferences += counts(&wa).1 + counts(&pa).1;
        let w = expand(witness, &wa);
        let p = expand(prove, &pa);
        match &mut proof {
            Term::App(_, args) => {
                args[3] = Term::App("witness".into(), vec![Term::List(w)]);
                args[4] = Term::App("prove".into(), vec![Term::List(p)]);
            }
            _ => panic!("reference proof"),
        }
        output.push_str(&format!("{}.\n", proof.print()));
    }
    (output, grades, preferences)
}

fn erase_lists(term: &mut Term) {
    match term {
        Term::App(_, xs) => {
            for x in xs {
                erase_lists(x);
            }
        }
        Term::List(xs) => {
            xs.retain(|t| !annotation(t));
            for x in xs {
                erase_lists(x);
            }
        }
        _ => (),
    }
}

pub fn annotate(baseline: &str, table: Option<&str>) -> (String, usize, usize) {
    let table = rows(table);
    if baseline.starts_with("'$guideline_proof'(") {
        return annotate_proof(baseline, &table);
    }
    let nouns = baseline
        .lines()
        .filter(|l| is_clause(l))
        .filter_map(|l| {
            let head = Clause::parse(l).head;
            head.args("guideline_entity")
                .map(|a| (a[1].clone(), a[2].atom()))
        })
        .collect();
    let lines: Vec<_> = baseline.lines().collect();
    let mut out = String::new();
    let (mut i, mut grades, mut preferences) = (0, 0, 0);
    while i < lines.len() {
        if is_clause(lines[i]) {
            let start = i;
            while i < lines.len() && is_clause(lines[i]) {
                i += 1;
            }
            let (text, g, p) = sentence(&lines[start..i], &table, &nouns);
            out.push_str(&text);
            grades += g;
            preferences += p;
        } else {
            out.push_str(lines[i]);
            out.push('\n');
            if lines[i] == ":- discontiguous(guideline_range/3)." {
                out.push_str(&new_declarations());
            }
            i += 1;
        }
    }
    (out, grades, preferences)
}

fn is_record(line: &str) -> bool {
    line.starts_with("guideline_document(") || line.starts_with("'$guideline_query'(")
}

pub fn rebind(text: &str, digest: Option<&str>) -> String {
    text.lines()
        .map(|line| {
            if !is_record(line) {
                return format!("{line}\n");
            }
            let mut record = Term::parse(line.strip_suffix('.').unwrap());
            let value = digest.map_or_else(
                || atom("none"),
                |h| Term::App("sha256".into(), vec![atom(h)]),
            );
            match &mut record {
                Term::App(_, args) => {
                    *args.last_mut().unwrap() = Term::App("vocabulary".into(), vec![value])
                }
                _ => panic!("reference record"),
            }
            format!("{}.\n", record.print())
        })
        .collect()
}

pub fn erase(text: &str, baseline: &str) -> String {
    let original_record = baseline
        .lines()
        .find(|l| is_record(l))
        .map(|l| Term::parse(l.strip_suffix('.').unwrap()));
    let delta = new_declarations();
    let mut output = String::new();
    for line in text.lines() {
        if delta.lines().any(|d| d == line) {
            continue;
        }
        if line.starts_with("'$guideline_proof'(") {
            let mut proof = Term::parse(line.strip_suffix('.').unwrap());
            erase_lists(&mut proof);
            output.push_str(&format!("{}.\n", proof.print()));
        } else if is_record(line) {
            let mut record = Term::parse(line.strip_suffix('.').unwrap());
            if let (Term::App(_, args), Some(Term::App(_, old))) = (&mut record, &original_record) {
                *args.last_mut().unwrap() = old.last().unwrap().clone();
            }
            output.push_str(&format!("{}.\n", record.print()));
        } else if is_clause(line) {
            let mut c = Clause::parse(line);
            if annotation(&c.head) {
                continue;
            }
            c.body.retain_mut(|g| match g {
                Goal::Pos(t) => !annotation(t),
                Goal::Naf(xs) => {
                    xs.retain(|t| !annotation(t));
                    !xs.is_empty()
                }
            });
            output.push_str(&c.print());
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }
    output
}
