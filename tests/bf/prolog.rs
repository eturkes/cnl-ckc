use super::model::{self, Item, Term};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Clause {
    pub head: Term,
    pub body: Vec<Item>,
}

fn split(text: &str, delimiter: char) -> Vec<&str> {
    let (mut depth, mut quoted, mut escaped, mut start) = (0, false, false, 0);
    let mut parts = Vec::new();
    for (i, c) in text.char_indices() {
        if escaped {
            escaped = false;
        } else if quoted && c == '\\' {
            escaped = true;
        } else if c == '\'' {
            quoted = !quoted;
        } else if !quoted {
            match c {
                '(' | '[' => depth += 1,
                ')' | ']' => depth -= 1,
                _ if c == delimiter && depth == 0 => {
                    parts.push(text[start..i].trim());
                    start = i + c.len_utf8();
                }
                _ => {}
            }
        }
    }
    assert!(!quoted && depth == 0, "unbalanced term: {text}");
    if start < text.len() {
        parts.push(text[start..].trim());
    }
    parts
}

fn unquote(s: &str) -> String {
    if !s.starts_with('\'') {
        return s.to_owned();
    }
    assert!(s.ends_with('\''));
    let mut result = String::new();
    let mut chars = s[1..s.len() - 1].chars();
    while let Some(c) = chars.next() {
        result.push(if c == '\\' { chars.next().unwrap() } else { c });
    }
    result
}

fn term(s: &str, names: &mut BTreeMap<String, usize>) -> Term {
    let s = s.trim();
    assert!(!s.is_empty());
    if s.starts_with('[') {
        assert!(s.ends_with(']'));
        let parts = split(&s[1..s.len() - 1], '|');
        let (front, tail) = match parts.as_slice() {
            [] => return Term::Nil,
            [front] => (*front, Term::Nil),
            [front, tail] => (*front, term(tail, names)),
            _ => panic!("list: {s}"),
        };
        let heads: Vec<_> = split(front, ',').iter().map(|t| term(t, names)).collect();
        return list(heads, tail);
    }
    let (mut quoted, mut escaped) = (false, false);
    for (i, c) in s.char_indices() {
        if escaped {
            escaped = false;
        } else if quoted && c == '\\' {
            escaped = true;
        } else if c == '\'' {
            quoted = !quoted;
        } else if c == '(' && !quoted {
            assert!(s.ends_with(')'));
            return Term::Comp(
                unquote(&s[..i]),
                split(&s[i + 1..s.len() - 1], ',')
                    .iter()
                    .map(|t| term(t, names))
                    .collect(),
            );
        }
    }
    if s.as_bytes()[0].is_ascii_uppercase() || s.starts_with('_') {
        let next = names.len();
        let key = if s == "_" {
            format!("_{next}")
        } else {
            s.to_owned()
        };
        Term::Var(*names.entry(key).or_insert(next))
    } else if let Ok(n) = s.parse() {
        Term::Int(n)
    } else {
        Term::Atom(unquote(s))
    }
}

pub fn list(ts: Vec<Term>, tail: Term) -> Term {
    ts.into_iter().rev().fold(tail, |tail, head| {
        Term::Comp("[|]".into(), vec![head, tail])
    })
}

fn unlist(mut t: &Term) -> Vec<Term> {
    let mut ts = Vec::new();
    while let Term::Comp(n, args) = t {
        assert!(n == "[|]" && args.len() == 2, "list: {t:?}");
        ts.push(args[0].clone());
        t = &args[1];
    }
    assert_eq!(*t, Term::Nil);
    ts
}

pub fn clauses(text: &str) -> Vec<Clause> {
    text.lines()
        .filter(|line| !line.is_empty() && !line.starts_with('%') && !line.starts_with(":-"))
        .map(|line| {
            let line = line.strip_suffix('.').expect("clause terminator");
            let (head, body) = line.split_once(" :- ").unwrap_or((line, ""));
            let mut names = BTreeMap::new();
            let head = term(head, &mut names);
            let body = split(body, ',')
                .into_iter()
                .map(|s| {
                    if let Some(s) = s.strip_prefix("\\+ ") {
                        let s = s
                            .strip_prefix('(')
                            .and_then(|s| s.strip_suffix(')'))
                            .unwrap_or(s);
                        Item::Naf(
                            split(s, ',')
                                .into_iter()
                                .map(|s| term(s, &mut names))
                                .collect(),
                        )
                    } else {
                        Item::Pos(term(s, &mut names))
                    }
                })
                .collect();
            Clause { head, body }
        })
        .collect()
}

fn rename(t: &mut Term, names: &mut BTreeMap<usize, usize>) {
    match t {
        Term::Var(v) => {
            let next = names.len();
            *v = *names.entry(*v).or_insert(next);
        }
        Term::Comp(_, args) => args.iter_mut().for_each(|t| rename(t, names)),
        _ => {}
    }
}
pub fn canonical(mut c: Clause) -> Clause {
    let mut names = BTreeMap::new();
    rename(&mut c.head, &mut names);
    for item in &mut c.body {
        match item {
            Item::Pos(t) => rename(t, &mut names),
            Item::Naf(ts) => ts.iter_mut().for_each(|t| rename(t, &mut names)),
        }
    }
    c
}

pub fn scope_guard(item: &Item) -> bool {
    matches!(item, Item::Naf(ts) if matches!(ts.as_slice(), [Term::Comp(n, args)] if n == "guideline_exclusion" && args.len() == 1))
}

pub fn reordered(c: &Clause) -> Clause {
    let mut result = c.clone();
    let guard = result.body.last().filter(|item| scope_guard(item)).cloned();
    if guard.is_some() {
        result.body.pop();
    }
    result.body = model::ordered(&result.body);
    result.body.extend(guard);
    canonical(result)
}

// One-rule probes: witness facts correspond to positive, non-built-in body
// items in the first clause; the appended scope guard contributes no witness.
pub fn reordered_proof(text: &str, body: &[Item]) -> Vec<Clause> {
    let indices: Vec<_> = body
        .iter()
        .enumerate()
        .filter_map(|(i, item)| matches!(item, Item::Pos(t) if !model::builtin(t)).then_some(i))
        .collect();
    let ranked: BTreeMap<_, _> = indices
        .iter()
        .enumerate()
        .map(|(rank, &i)| (i, rank))
        .collect();
    let permutation: Vec<_> = model::order(body)
        .iter()
        .filter_map(|i| ranked.get(i).copied())
        .collect();
    let mut proofs = clauses(text);
    assert_eq!(proofs.len(), 1);
    let Term::Comp(name, args) = &mut proofs[0].head else {
        panic!("proof term");
    };
    assert_eq!(name, "$guideline_proof");
    let Term::Comp(witness, args) = &mut args[3] else {
        panic!("witness term");
    };
    assert_eq!(witness, "witness");
    let terms = unlist(&args[0]);
    assert_eq!(terms.len(), indices.len(), "witness/body correspondence");
    args[0] = list(
        permutation.iter().map(|&i| terms[i].clone()).collect(),
        Term::Nil,
    );
    proofs
}
