use crate::program::{Clause, Goal, PREDICATES, builtin};
use crate::term::{Arena, Id, Node, natural_compare, variable_name};
use std::cmp::Ordering;

fn name_ok(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with('-')
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}
fn hex64(arena: &Arena, id: Id) -> bool {
    arena.atom_name(id).is_some_and(|s| {
        s.len() == 64
            && s.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
fn digest(arena: &Arena, id: Id, wrapper: &str) -> bool {
    arena.named(id, wrapper, 1).is_some_and(|a| {
        arena.atom_name(a[0]) == Some("none")
            || arena
                .named(a[0], "sha256", 1)
                .is_some_and(|a| hex64(arena, a[0]))
    })
}
fn record(arena: &Arena, id: Id) -> bool {
    arena.named(id, "guideline_document", 4).is_some_and(|a| {
        arena.atom_name(a[0]).is_some_and(name_ok)
            && arena
                .named(a[1], "ace_sha256", 1)
                .is_some_and(|a| hex64(arena, a[0]))
            && digest(arena, a[2], "ulex")
            && digest(arena, a[3], "vocabulary")
    })
}
fn term_ok(arena: &Arena, id: Id) -> bool {
    match arena.node(id) {
        Node::Compound(name, args) => {
            arena.name(*name) != "$VAR"
                && !args.is_empty()
                && args.iter().all(|arg| term_ok(arena, *arg))
        }
        _ => true,
    }
}
fn literal(arena: &Arena, id: Id, body: bool) -> bool {
    arena.call(id).is_some_and(|(name, args)| {
        (PREDICATES[2..].contains(&(arena.name(name), args.len())) || body && builtin(arena, id))
            && args.iter().all(|arg| term_ok(arena, *arg))
    })
}
fn render_goal(arena: &Arena, goal: &Goal) -> Option<String> {
    match goal {
        Goal::Positive(term) | Goal::Builtin(term) => {
            literal(arena, *term, true).then(|| arena.render(*term))
        }
        Goal::Not(inner) => {
            if inner.is_empty() || inner.iter().any(|g| matches!(g, Goal::Not(_))) {
                return None;
            }
            let text = inner
                .iter()
                .map(|g| render_goal(arena, g))
                .collect::<Option<Vec<_>>>()?
                .join(", ");
            Some(if inner.len() == 1 {
                format!("\\+ {text}")
            } else {
                format!("\\+ ({text})")
            })
        }
    }
}
pub fn clause_image(arena: &Arena, clause: &Clause) -> Option<String> {
    let mut out = arena.render(clause.head);
    if !clause.body.is_empty() {
        let goals = clause
            .body
            .iter()
            .map(|g| render_goal(arena, g))
            .collect::<Option<Vec<_>>>()?;
        out.push_str(" :- ");
        out.push_str(&goals.join(", "));
    }
    out.push('.');
    Some(out)
}
pub fn read_line(raw: &str, clause: &Clause) -> Option<String> {
    if raw.trim() != raw
        || !clause
            .names
            .iter()
            .enumerate()
            .all(|(i, n)| n == "_" || *n == variable_name(i))
    {
        return None;
    }
    let mut out = String::with_capacity(raw.len());
    let mut start = 0;
    for &(at, var) in &clause.anonymous {
        out.push_str(raw.get(start..at)?);
        out.push_str(&variable_name(var));
        start = at + 1;
    }
    out.push_str(raw.get(start..)?);
    Some(out)
}

fn line_matches(arena: &Arena, clause: &Clause, line: &str) -> bool {
    read_line(line, clause)
        .zip(clause_image(arena, clause))
        .is_some_and(|(read, image)| read == image)
}

// The image is checked afresh in admission. The source parser supplies the same arena used
// by support/closure; neither a cached verdict nor a second parser supplies this boundary.
pub fn document(arena: &Arena, clauses: &[Clause], text: &str) -> bool {
    let Some(text) = text.strip_suffix('\n') else {
        return false;
    };
    let lines = text.split('\n').collect::<Vec<_>>();
    let start = lines
        .iter()
        .take_while(|line| line.starts_with('%'))
        .count();
    if lines[..start].iter().any(|line| line.ends_with('\r')) {
        return false;
    }
    let mut at = start;
    for (name, arity) in PREDICATES {
        for kind in ["multifile", "discontiguous"] {
            if lines.get(at) != Some(&format!(":- {kind}({name}/{arity}).").as_str()) {
                return false;
            }
            at += 1;
        }
    }
    if clauses.len() < 3
        || clauses[0].line != at
        || clauses[1].line != at + 1
        || !clauses[0].body.is_empty()
        || !clauses[1].body.is_empty()
    {
        return false;
    }
    if lines.get(at) != Some(&"guideline_schema_version(2).") || !record(arena, clauses[1].head) {
        return false;
    }
    if !line_matches(arena, &clauses[1], lines[at + 1]) {
        return false;
    }
    at += 2;
    let mut cursor = 2;
    let mut last = "0";
    let mut in_bundle = false;
    let mut members = 0;
    while at < lines.len() {
        let line = lines[at];
        if let Some(marker) = line.strip_prefix("% S") {
            if in_bundle && members == 0 {
                return false;
            }
            let Some((number, text)) = marker.split_once(": ") else {
                return false;
            };
            if number.is_empty()
                || number.starts_with('0')
                || !number.bytes().all(|b| b.is_ascii_digit())
                || natural_compare(number, last) != Ordering::Greater
                || text.is_empty()
            {
                return false;
            }
            last = number;
            in_bundle = true;
            members = 0;
        } else {
            let Some(clause) = clauses.get(cursor) else {
                return false;
            };
            if !in_bundle
                || clause.line != at
                || !literal(arena, clause.head, false)
                || !line_matches(arena, clause, line)
            {
                return false;
            }
            cursor += 1;
            members += 1;
        }
        at += 1;
    }
    in_bundle && members > 0 && cursor == clauses.len()
}
