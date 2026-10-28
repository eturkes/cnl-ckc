use super::support::sha;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

pub const BASE: &str = "258ddc44";
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

pub fn variable(n: usize) -> String {
    let c = char::from(b'A' + (n % 26) as u8);
    if n < 26 {
        c.to_string()
    } else {
        format!("{c}{}", n / 26)
    }
}

// D2, independently organized: split LF first, lex identifier spans second, then substitute.
// A backslash consumes exactly one byte only while scanning a quoted atom.
pub fn runs(line: &[u8]) -> Vec<Range<usize>> {
    if line.first() == Some(&b'%') {
        return vec![];
    }
    let mut out = Vec::new();
    let mut i = 0;
    while i < line.len() {
        if line[i] == b'\'' {
            i += 1;
            while i < line.len() {
                match line[i] {
                    b'\\' => i = (i + 2).min(line.len()),
                    b'\'' => {
                        i += 1;
                        break;
                    }
                    _ => i += 1,
                }
            }
        } else if line[i].is_ascii_alphanumeric() || line[i] == b'_' {
            let start = i;
            while i < line.len() && (line[i].is_ascii_alphanumeric() || line[i] == b'_') {
                i += 1;
            }
            out.push(start..i);
        } else {
            i += 1;
        }
    }
    out
}

pub struct Reading {
    pub text: Vec<u8>,
    pub raw_at: Vec<usize>,
}

pub fn read(raw: &[u8]) -> Reading {
    let mut text = Vec::new();
    let mut raw_at = Vec::new();
    let mut base = 0;
    for line in raw.split_inclusive(|b| *b == b'\n') {
        let mut named = BTreeSet::new();
        let mut anonymous = BTreeSet::new();
        let mut end = 0;
        for span in runs(line) {
            text.extend_from_slice(&line[end..span.start]);
            raw_at.extend(base + end..base + span.start);
            let word = &line[span.clone()];
            if word == b"_" {
                let name = variable(named.len() + anonymous.len()).into_bytes();
                text.extend_from_slice(&name);
                raw_at.extend(std::iter::repeat_n(base + span.start, name.len()));
                anonymous.insert(name);
            } else {
                if word[0].is_ascii_uppercase() && anonymous.contains(word) {
                    text.extend(std::iter::repeat_n(0, word.len()));
                } else {
                    text.extend_from_slice(word);
                    if word[0].is_ascii_uppercase() {
                        named.insert(word.to_vec());
                    }
                }
                raw_at.extend(base + span.start..base + span.end);
            }
            end = span.end;
        }
        text.extend_from_slice(&line[end..]);
        raw_at.extend(base + end..base + line.len());
        base += line.len();
    }
    raw_at.push(raw.len());
    Reading { text, raw_at }
}

pub fn letter_counts(line: &[u8]) -> BTreeMap<Vec<u8>, usize> {
    let mut counts = BTreeMap::new();
    for span in runs(line) {
        let word = &line[span];
        if word[0].is_ascii_uppercase() {
            *counts.entry(word.to_vec()).or_default() += 1;
        }
    }
    counts
}

pub fn singletons(bytes: &[u8]) -> usize {
    bytes
        .split(|b| *b == b'\n')
        .map(|line| letter_counts(line).values().filter(|&&n| n == 1).count())
        .sum()
}

pub fn anonymous_count(bytes: &[u8]) -> usize {
    bytes
        .split(|b| *b == b'\n')
        .map(|line| {
            runs(line)
                .iter()
                .filter(|s| &line[(*s).clone()] == b"_")
                .count()
        })
        .sum()
}

pub fn anonymize(letter: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for line in letter.split_inclusive(|b| *b == b'\n') {
        let counts = letter_counts(line);
        let mut end = 0;
        for span in runs(line) {
            out.extend_from_slice(&line[end..span.start]);
            let word = &line[span.clone()];
            out.extend_from_slice(if counts.get(word) == Some(&1) {
                b"_"
            } else {
                word
            });
            end = span.end;
        }
        out.extend_from_slice(&line[end..]);
    }
    out
}

pub fn emission_ok(emitted: &[u8], letter: &[u8]) -> bool {
    singletons(emitted) == 0 && read(emitted).text == letter
}

pub fn number_letters(line: &str) -> String {
    let mut names = BTreeMap::new();
    let mut out = String::new();
    let mut end = 0;
    for span in runs(line.as_bytes()) {
        out.push_str(&line[end..span.start]);
        let word = &line[span.clone()];
        if word.as_bytes()[0].is_ascii_uppercase() {
            let next = names.len();
            let n = *names.entry(word).or_insert(next);
            out.push_str(&variable(n));
        } else {
            out.push_str(word);
        }
        end = span.end;
    }
    out.push_str(&line[end..]);
    out
}

pub fn atom(s: &str) -> String {
    if s.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        s.to_owned()
    } else {
        quoted(s)
    }
}

pub fn quoted(s: &str) -> String {
    format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
}

pub fn document(id: &str, ace: &str, ulex: Option<&str>, clauses: &str) -> String {
    let declarations: String = INDICATORS
        .iter()
        .map(|(p, arity)| format!(":- multifile({p}/{arity}).\n:- discontiguous({p}/{arity}).\n"))
        .collect();
    let lex = ulex.map_or_else(|| "none".into(), |s| format!("sha256({})", atom(&sha(s))));
    format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{declarations}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex({lex}),vocabulary(none)).\n% S1: {}\n{clauses}",
        atom(id),
        atom(&sha(ace)),
        ace.trim_end()
    )
}

pub fn doc(clauses: &str) -> String {
    document("sgw", "Synthetic _ A ' clause.\n", None, clauses)
}

pub fn query(goal: &str) -> String {
    format!(
        "% sgw-q compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,'sgw-q',ace_sha256({}),ulex(none),vocabulary(none)).\n% Q1: Synthetic _ A ' query.\n'$guideline_query_projection'(goal({goal}),answers([])).\n",
        "a".repeat(64)
    )
}

pub fn answers(qsha: &str, result: &str) -> String {
    format!(
        "% sgw-q answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,'sgw-q',query_sha256({}),result({result})).\n",
        atom(qsha)
    )
}

pub fn traces(result: &str) -> String {
    format!(
        "% sgw-q traced against the loaded composition by ckc queries; do not edit.\n'$guideline_traces'(v1,'sgw-q',query_sha256({}),answers_sha256({}),result({result})).\n",
        "a".repeat(64),
        "b".repeat(64)
    )
}

pub fn generated_clauses(n: usize, seed: usize) -> String {
    let all = (0..n).map(variable).collect::<Vec<_>>().join(",");
    let repeated = (0..n)
        .filter(|i| (i + seed).is_multiple_of(3) || *i == n - 1)
        .map(variable)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "guideline_event(actual,[{all}],hold) :- guideline_event(actual,[{repeated}],hold), \\+ guideline_event(actual,{},missing).\n",
        variable(n)
    )
}
