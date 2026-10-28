use super::common::*;
use super::inventories::Guideline;
use std::collections::{BTreeMap, BTreeSet};
// Derived-entity scan (corpus.md Derived referents; user rulings "Fund here, then cut" + "Fold
// into SKO"): an actual-context entity that a document derives (rule head) or asserts (fact)
// is a witness for every condition that reads its noun — a rule makes a fresh one per match —
// so each such (document, sentence, noun) whose noun a condition of another document reads
// needs a row of `audit/derived-entities.tsv`: a class those conditions read by design, with
// its reason. Rows name flagged entities only. Text-level over canonical v1 lines.
const TABLE: &str = "audit/derived-entities.tsv";
const HEADER: &str = "# format: docid\tsentence\tnoun\tclass\treason";
const HEAD: &str = "guideline_entity(actual,'$guideline_id'(product,";
const READ: &str = "guideline_entity(actual,";
type Entity = (String, u32, String);

pub(super) fn check(g: &Guideline) -> Result {
    let mut made = BTreeSet::new();
    let mut readers: BTreeMap<String, BTreeSet<&str>> = BTreeMap::new();
    for id in &g.docids {
        let src = corpus_text(&g.pl(id), "derived-entity")?;
        for line in src.split('\n') {
            if let Some(e) = line.strip_prefix(HEAD).and_then(head) {
                made.insert(e);
            }
            if let Some((_, body)) = line.split_once(" :- ") {
                for noun in reads(body) {
                    readers.entry(noun).or_default().insert(id);
                }
            }
        }
    }
    let read_by = |(doc, _, noun): &Entity| {
        readers
            .get(noun)
            .map_or(0, |r| r.iter().filter(|d| **d != doc).count())
    };
    let flagged: BTreeSet<Entity> = made.into_iter().filter(|e| read_by(e) > 0).collect();
    let rows = table(g)?;
    let rel = format!("guidelines/{}/{TABLE}", name(&g.path));
    if let Some(e) = flagged.iter().find(|e| !rows.contains_key(*e)) {
        return Err(violation(
            "derived-entity",
            format!(
                "{rel}: unruled {} S{} {} (read by {} documents)",
                e.0,
                e.1,
                e.2,
                read_by(e)
            ),
        ));
    }
    if let Some((e, n)) = rows.iter().find(|(e, _)| !flagged.contains(*e)) {
        return Err(violation(
            "derived-entity",
            format!(
                "{rel}: row {n} names no flagged entity: {} S{} {}",
                e.0, e.1, e.2
            ),
        ));
    }
    println!(
        "ckc: derived-entities ok {} {} ruled",
        name(&g.path),
        flagged.len()
    );
    Ok(())
}

// `'<doc>',<S>,ref(<n>),[<vars>]),<noun>,` after HEAD.
fn head(rest: &str) -> Option<Entity> {
    let (doc, rest) = atom(rest)?;
    let (s, rest) = rest.strip_prefix(',')?.split_once(",ref(")?;
    let (_, rest) = rest.split_once("]),")?;
    let (noun, rest) = atom(rest)?;
    rest.starts_with(',')
        .then(|| Some((doc, s.parse().ok()?, noun)))
        .flatten()
}

// Nouns of `guideline_entity(actual,<Var>,<noun>,` goals.
fn reads(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(at) = rest.find(READ) {
        rest = &rest[at + READ.len()..];
        let var = rest
            .bytes()
            .take_while(|b| b.is_ascii_alphanumeric() || *b == b'_')
            .count();
        if var > 0
            && (rest.as_bytes()[0].is_ascii_uppercase() || rest.starts_with('_'))
            && let Some((noun, _)) = rest[var..].strip_prefix(',').and_then(atom)
        {
            out.push(noun);
        }
    }
    out
}

// A canonical atom: `'…'` (`\\`, `\'` escapes) or a bare `[a-z][A-Za-z0-9_]*`.
fn atom(s: &str) -> Option<(String, &str)> {
    if let Some(q) = s.strip_prefix('\'') {
        let mut out = String::new();
        let mut chars = q.char_indices();
        while let Some((i, c)) = chars.next() {
            match c {
                '\\' => out.push(chars.next()?.1),
                '\'' => return Some((out, &q[i + 1..])),
                c => out.push(c),
            }
        }
        return None;
    }
    let n = s
        .bytes()
        .take_while(|b| b.is_ascii_alphanumeric() || *b == b'_')
        .count();
    (n > 0 && s.as_bytes()[0].is_ascii_lowercase()).then(|| (s[..n].to_owned(), &s[n..]))
}

// Rows keyed by entity → their 1-based line number; absent table = no rows.
fn table(g: &Guideline) -> Result<BTreeMap<Entity, usize>> {
    let path = g.path.join(TABLE);
    let mut rows = BTreeMap::new();
    if !path.exists() {
        return Ok(rows);
    }
    let rel = format!("guidelines/{}/{TABLE}", name(&g.path));
    let text = corpus_text(&path, "derived-entity")?;
    let bad =
        |n: usize, why: &str| violation("derived-entity", format!("{rel}: {why} at line {n}"));
    let Some(body) = text.strip_suffix('\n') else {
        return Err(bad(text.split('\n').count(), "missing final newline"));
    };
    let mut lines = body.split('\n').enumerate().map(|(i, l)| (i + 1, l));
    if lines.next().map(|(_, l)| l) != Some(HEADER) {
        return Err(bad(1, "header"));
    }
    for (n, line) in lines {
        let f: Vec<&str> = line.split('\t').collect();
        let sentence = f.get(1).and_then(|s| {
            (!s.starts_with('0') && s.bytes().all(|b| b.is_ascii_digit()))
                .then(|| s.parse::<u32>().ok())
                .flatten()
        });
        let (5, Some(s)) = (f.len(), sentence) else {
            return Err(bad(n, "row shape"));
        };
        if !valid_docid(f[0]) || f[2..].iter().any(|x| x.trim().is_empty() || x.trim() != *x) {
            return Err(bad(n, "row field"));
        }
        if rows
            .insert((f[0].to_owned(), s, f[2].to_owned()), n)
            .is_some()
        {
            return Err(bad(n, "duplicate row"));
        }
    }
    Ok(rows)
}
