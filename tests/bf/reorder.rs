// Binding-first order (contract bf) applied to compiled-document text, for suites whose
// expectations predate it; a text-level reading of the Definition, independent of the
// compiler, the spec and tests/bf/model.rs. Every rule body: the leftmost goal whose
// waited variables are bound, else the leftmost goal (a trailing scope guard stays last:
// the compiler appends it after ordering), variables then renamed in first-occurrence
// order. Every proof payload: its witness facts in that order, a witness id standing for
// its variable. All other bytes stay the input's.
use std::collections::{BTreeMap, BTreeSet};

// Semantic predicates that wait on their referent (argument 2).
const REFERENT: [(&str, usize); 15] = [
    ("guideline_cardinality", 5),
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
const ID: &str = "'$guideline_id'(witness,";

pub fn document(text: &str) -> String {
    text.split_inclusive('\n').map(line).collect()
}

fn line(l: &str) -> String {
    let (body, nl) = l.strip_suffix('\n').map_or((l, ""), |b| (b, "\n"));
    if body.starts_with('%') || body.starts_with(":-") {
        return l.to_owned();
    }
    match body.strip_suffix('.').and_then(|b| b.split_once(" :- ")) {
        Some((head, goals)) => format!("{}.{nl}", rule(head, goals)),
        None if body.starts_with("'$guideline_proof'(") => format!("{}{nl}", proof(body)),
        None => l.to_owned(),
    }
}

// Top-level parts of `text` split at `sep`, outside quoted atoms and brackets.
fn split(text: &str, sep: char) -> Vec<&str> {
    let (mut depth, mut quoted, mut escaped, mut start) = (0i32, false, false, 0);
    let mut parts = Vec::new();
    for (i, c) in text.char_indices() {
        if escaped {
            escaped = false;
        } else if quoted {
            match c {
                '\\' => escaped = true,
                '\'' => quoted = false,
                _ => {}
            }
        } else {
            match c {
                '\'' => quoted = true,
                '(' | '[' => depth += 1,
                ')' | ']' => depth -= 1,
                _ if c == sep && depth == 0 => {
                    parts.push(text[start..i].trim());
                    start = i + c.len_utf8();
                }
                _ => {}
            }
        }
    }
    parts.push(text[start..].trim());
    parts
}

// The variable tokens of a clause text (outside quoted atoms), in order.
fn clause_vars(text: &str) -> Vec<(usize, usize)> {
    let b = text.as_bytes();
    let (mut out, mut i, mut quoted) = (Vec::new(), 0, false);
    while i < b.len() {
        let c = b[i];
        if quoted {
            if c == b'\\' {
                i += 1;
            } else if c == b'\'' {
                quoted = false;
            }
            i += 1;
        } else if c == b'\'' {
            quoted = true;
            i += 1;
        } else if (c.is_ascii_uppercase() || c == b'_')
            && (i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_'))
        {
            let start = i;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            out.push((start, i));
        } else {
            i += 1;
        }
    }
    out
}

// The byte index of the bracket closing the one that opens at `open`, outside quoted atoms.
fn closing(text: &str, open: usize) -> usize {
    let (mut depth, mut quoted, mut escaped) = (0i32, false, false);
    for (i, c) in text[open..].char_indices() {
        if escaped {
            escaped = false;
        } else if quoted {
            match c {
                '\\' => escaped = true,
                '\'' => quoted = false,
                _ => {}
            }
        } else {
            match c {
                '\'' => quoted = true,
                '(' | '[' => depth += 1,
                ')' | ']' => {
                    depth -= 1;
                    if depth == 0 {
                        return open + i;
                    }
                }
                _ => {}
            }
        }
    }
    panic!("unbalanced: {text}")
}

// The witness ids of a ground fact text, each as its own token.
fn witness_ids(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut from = 0;
    while let Some(at) = text[from..].find(ID) {
        let start = from + at;
        let end = closing(text, start + "'$guideline_id'".len()) + 1;
        out.insert(text[start..end].to_owned());
        from = end;
    }
    out
}

// What a goal waits on, given the variables of its argument texts.
fn waited(goal: &str, vars: &dyn Fn(&str) -> BTreeSet<String>) -> BTreeSet<String> {
    let Some((name, inner)) = goal.split_once('(') else {
        return BTreeSet::new();
    };
    let args = split(inner.strip_suffix(')').expect("goal"), ',');
    let inputs: &[&str] = match (name, args.len()) {
        ("guideline_satisfies", 4) => &args[..2],
        ("guideline_satisfies", 6) => &args[..3],
        ("guideline_product" | "guideline_quotient", 6) => &args[..4],
        (n, a) if REFERENT.contains(&(n, a)) => &args[1..2],
        _ => &[],
    };
    inputs.iter().flat_map(|a| vars(a)).collect()
}

// Positions of `goals` in binding-first order.
fn order(goals: &[&str], vars: &dyn Fn(&str) -> BTreeSet<String>) -> Vec<usize> {
    let naf = |g: &str| g.starts_with("\\+ ");
    let shared: BTreeSet<String> = goals.iter().filter(|g| !naf(g)).flat_map(|g| vars(g)).collect();
    let need: Vec<BTreeSet<String>> = goals
        .iter()
        .map(|g| match naf(g) {
            true => vars(g).intersection(&shared).cloned().collect(),
            false => waited(g, vars),
        })
        .collect();
    let (mut rest, mut bound, mut out): (Vec<usize>, BTreeSet<String>, Vec<usize>) =
        ((0..goals.len()).collect(), BTreeSet::new(), Vec::new());
    while !rest.is_empty() {
        let at = rest.iter().position(|&i| need[i].is_subset(&bound)).unwrap_or(0);
        let i = rest.remove(at);
        if !naf(goals[i]) {
            bound.extend(vars(goals[i]));
        }
        out.push(i);
    }
    out
}

fn rule(head: &str, goals: &str) -> String {
    let mut goals = split(goals, ',');
    let guard = goals.last().filter(|g| g.starts_with("\\+ guideline_exclusion(")).copied();
    if guard.is_some() {
        goals.pop();
    }
    let vars = |t: &str| -> BTreeSet<String> {
        clause_vars(t).into_iter().map(|(a, b)| t[a..b].to_owned()).collect()
    };
    let mut body: Vec<&str> = order(&goals, &vars).into_iter().map(|i| goals[i]).collect();
    body.extend(guard);
    rename(&format!("{head} :- {}", body.join(", ")))
}

// Variables renamed A, B, …, Z, A1, … in first-occurrence order, as numbervars/3 names them.
fn rename(text: &str) -> String {
    let mut names: BTreeMap<&str, String> = BTreeMap::new();
    let (mut out, mut last) = (String::with_capacity(text.len()), 0);
    for (a, b) in clause_vars(text) {
        out.push_str(&text[last..a]);
        let next = names.len();
        out.push_str(names.entry(&text[a..b]).or_insert_with(|| {
            let letter = char::from(b'A' + (next % 26) as u8);
            match next / 26 {
                0 => letter.to_string(),
                n => format!("{letter}{n}"),
            }
        }));
        last = b;
    }
    out.push_str(&text[last..]);
    out
}

fn proof(body: &str) -> String {
    let open = body.find("witness([").expect("proof payload without witness list") + 8;
    let close = closing(body, open);
    let facts = split(&body[open + 1..close], ',');
    let ordered: Vec<&str> = order(&facts, &witness_ids).into_iter().map(|i| facts[i]).collect();
    format!("{}{}{}", &body[..=open], ordered.join(","), &body[close..])
}
