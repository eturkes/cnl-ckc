// nt1 D2/D3 inverse for frozen projections + cs1/sup1 declaration pairs. Only a
// removed time noun/class comes from the source occurrence; bounds, referents,
// links and every other byte stay observed.
use std::collections::BTreeMap;

pub const DECLARATIONS: &str = ":- multifile(guideline_unit/3).\n:- discontiguous(guideline_unit/3).\n:- multifile(guideline_exclusion/1).\n:- discontiguous(guideline_exclusion/1).\n:- multifile(guideline_supersession/4).\n:- discontiguous(guideline_supersession/4).\n:- multifile(guideline_evaluation_date/1).\n:- discontiguous(guideline_evaluation_date/1).\n";

#[derive(Clone, Debug, PartialEq, Eq)]
enum Term {
    Atom(String),
    Var(String),
    App(String, Vec<Term>),
    List(Vec<Term>),
}

fn separators(s: &str, needle: &str) -> Vec<usize> {
    let (mut depth, mut quoted, mut escaped) = (0isize, false, false);
    let mut out = Vec::new();
    for (i, b) in s.bytes().enumerate() {
        if escaped {
            escaped = false;
        } else if quoted && b == b'\\' {
            escaped = true;
        } else if b == b'\'' {
            quoted = !quoted;
        } else if !quoted {
            if depth == 0 && s.as_bytes()[i..].starts_with(needle.as_bytes()) {
                out.push(i);
            }
            match b {
                b'(' | b'[' => depth += 1,
                b')' | b']' => depth -= 1,
                _ => (),
            }
        }
    }
    assert!(depth == 0 && !quoted, "projection delimiters: {s}");
    out
}

fn split(s: &str) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut start = 0;
    let mut out = Vec::new();
    for at in separators(s, ",") {
        out.push(s[start..at].trim());
        start = at + 1;
    }
    out.push(s[start..].trim());
    out
}

impl Term {
    fn parse(s: &str) -> Self {
        if let Some(s) = s.strip_prefix("\\+ ") {
            let inner = s
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .unwrap_or(s);
            return Self::App(
                "$naf".into(),
                split(inner).into_iter().map(Self::parse).collect(),
            );
        }
        if let Some(inner) = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            return Self::List(split(inner).into_iter().map(Self::parse).collect());
        }
        if let Some(&at) = separators(s, "(").first() {
            assert!(s.ends_with(')'), "projection term: {s}");
            Self::App(
                s[..at].into(),
                split(&s[at + 1..s.len() - 1])
                    .into_iter()
                    .map(Self::parse)
                    .collect(),
            )
        } else if s.as_bytes().first().is_some_and(u8::is_ascii_uppercase) || s.starts_with('_') {
            Self::Var(s.into())
        } else {
            Self::Atom(s.into())
        }
    }
    fn args(&self, name: &str) -> Option<&[Self]> {
        match self {
            Self::App(n, args) if n == name => Some(args),
            _ => None,
        }
    }
    fn print(&self) -> String {
        match self {
            Self::Atom(s) | Self::Var(s) => s.clone(),
            Self::App(n, xs) if n == "$naf" => {
                let body = xs.iter().map(Self::print).collect::<Vec<_>>().join(", ");
                if xs.len() == 1 {
                    format!("\\+ {body}")
                } else {
                    format!("\\+ ({body})")
                }
            }
            Self::App(n, xs) => format!(
                "{n}({})",
                xs.iter().map(Self::print).collect::<Vec<_>>().join(",")
            ),
            Self::List(xs) => format!(
                "[{}]",
                xs.iter().map(Self::print).collect::<Vec<_>>().join(",")
            ),
        }
    }
    fn rename(&mut self, names: &mut BTreeMap<String, String>) {
        match self {
            Self::Var(s) => {
                let n = names.len();
                *s = names
                    .entry(s.clone())
                    .or_insert_with(|| {
                        let first = char::from(b'A' + u8::try_from(n % 26).unwrap());
                        if n < 26 {
                            first.to_string()
                        } else {
                            format!("{first}{}", n / 26)
                        }
                    })
                    .clone();
            }
            Self::App(_, xs) | Self::List(xs) => {
                for x in xs {
                    x.rename(names);
                }
            }
            Self::Atom(_) => (),
        }
    }
}

fn canonical_variables<'a>(terms: impl IntoIterator<Item = &'a Term>) {
    let mut names = BTreeMap::new();
    for term in terms {
        let mut normalized = term.clone();
        normalized.rename(&mut names);
        assert_eq!(&normalized, term, "canonical observed variables");
    }
}

fn clause_text(head: &Term, terms: &[Term]) -> String {
    if terms.is_empty() {
        format!("{}.", head.print())
    } else {
        format!(
            "{} :- {}.",
            head.print(),
            terms.iter().map(Term::print).collect::<Vec<_>>().join(", ")
        )
    }
}

fn comparison(t: &Term) -> bool {
    matches!(t, Term::Atom(s) if ["exactly", "geq", "greater", "leq", "less"].contains(&s.as_str()))
}

fn source_entity(source: &[Term], ordinal: usize, card: &[Term]) -> Term {
    let old = source
        .iter()
        .filter_map(|t| t.args("guideline_cardinality"))
        .nth(ordinal)
        .expect("source quantity occurrence");
    let noun = source
        .iter()
        .filter_map(|t| t.args("guideline_entity"))
        .find(|a| a[..2] == old[..2])
        .expect("source quantity noun + class");
    Term::App(
        "guideline_entity".into(),
        vec![
            card[0].clone(),
            card[1].clone(),
            noun[2].clone(),
            noun[3].clone(),
        ],
    )
}

fn inverse(items: &[Term], source: &[Term]) -> Vec<Term> {
    let mut out = Vec::new();
    let mut relink = Vec::new();
    let (mut i, mut quantity, mut naf) = (0, 0, 0);
    while i < items.len() {
        let item = &items[i];
        if let Some(inner) = item.args("$naf") {
            let old = source
                .iter()
                .filter_map(|t| t.args("$naf"))
                .nth(naf)
                .expect("source NAF occurrence");
            out.push(Term::App("$naf".into(), inverse(inner, old)));
            naf += 1;
        } else if let Some(card) = item.args("guideline_cardinality") {
            let unit = items.get(i + 1).and_then(|t| t.args("guideline_unit"));
            let builtin_at = i + 1 + usize::from(unit.is_some());
            let builtin = items
                .get(builtin_at)
                .and_then(|t| t.args("guideline_satisfies"));
            if let Some(b) = builtin {
                assert!(
                    matches!(card[3], Term::Var(_)) && matches!(card[4], Term::Var(_)),
                    "D3 fresh comparison/count"
                );
                assert_eq!(&card[3..5], &b[..2], "D3 cardinality/filter link");
                let mut restored = card.to_vec();
                if let Some(u) = unit {
                    assert_eq!(b.len(), 6, "D3 time filter arity");
                    assert_eq!(&u[..2], &card[..2], "D3 unit quantity link");
                    assert_eq!(u[2], b[2], "D3 unit/filter link");
                    assert!(matches!(u[2], Term::Var(_)) && comparison(&b[3]));
                    out.push(source_entity(source, quantity, card));
                    restored[3] = b[3].clone();
                    restored[4] = b[4].clone();
                    relink.push((card[0].clone(), card[1].clone(), u[2].clone(), b[5].clone()));
                } else {
                    assert_eq!(b.len(), 4, "D3 non-time filter arity");
                    assert!(comparison(&b[2]));
                    restored[3] = b[2].clone();
                    restored[4] = b[3].clone();
                }
                out.push(Term::App("guideline_cardinality".into(), restored));
                i = builtin_at;
            } else {
                // D4 proof witnesses have substituted bounds and no filter goals.
                if unit.is_some()
                    && comparison(&card[3])
                    && !items
                        .iter()
                        .filter_map(|t| t.args("guideline_entity"))
                        .any(|a| a[..2] == card[..2])
                {
                    out.push(source_entity(source, quantity, card));
                }
                out.push(item.clone());
            }
            quantity += 1;
        } else if item.args("guideline_unit").is_none() {
            out.push(item.clone());
        }
        i += 1;
    }
    for item in &mut out {
        if let Term::App(name, args) = item {
            let slots: &[(usize, usize)] = match name.as_str() {
                "guideline_interval" => &[(3, 4)],
                "guideline_recurrence" => &[(2, 3)],
                "guideline_recurrence_window" => &[(2, 3), (5, 6)],
                _ => &[],
            };
            for &(quantity, unit) in slots {
                for (ctx, r, var, bound) in &relink {
                    if args[0] == *ctx && args[quantity] == *r {
                        assert_eq!(args[unit], *var, "D3 annotation unit link");
                        args[unit] = bound.clone();
                    }
                }
            }
        }
    }
    out
}

fn goals(t: &Term) -> Vec<Term> {
    if let Some(xs) = t.args("','") {
        assert_eq!(xs.len(), 2);
        [goals(&xs[0]), goals(&xs[1])].concat()
    } else {
        vec![t.clone()]
    }
}

fn conjunction(xs: &[Term]) -> Term {
    match xs {
        [last] => last.clone(),
        [first, rest @ ..] => Term::App("','".into(), vec![first.clone(), conjunction(rest)]),
        [] => panic!("nonempty projected question"),
    }
}

fn clause(line: &str) -> bool {
    line.starts_with("guideline_")
        && !line.starts_with("guideline_schema_version(")
        && !line.starts_with("guideline_document(")
        && !line.starts_with("guideline_unit(")
}

pub fn erase(text: &str, source: &str) -> String {
    assert!(
        text.is_empty() || text.ends_with('\n'),
        "canonical observed final newline"
    );
    // `lines()` would drop a CR before LF and hide it from the frozen comparison.
    assert!(!text.contains('\r'), "canonical observed bytes hold no CR");
    let old_clauses: Vec<_> = source.lines().filter(|l| clause(l)).collect();
    let old_queries: Vec<_> = source
        .lines()
        .filter(|l| l.starts_with("'$guideline_query_projection'("))
        .collect();
    let old_proofs: Vec<_> = source
        .lines()
        .filter(|l| l.starts_with("'$guideline_proof'("))
        .collect();
    let (mut c, mut q, mut p) = (0, 0, 0);
    let mut out = String::new();
    for line in text.replace(DECLARATIONS, "").lines() {
        if line.starts_with("guideline_unit(") {
            continue;
        }
        let projected = if clause(line) {
            let body = line.strip_suffix('.').unwrap();
            let (head, mut terms) = if let Some(at) = separators(body, " :- ").first().copied() {
                (
                    Term::parse(&body[..at]),
                    split(&body[at + 4..])
                        .into_iter()
                        .map(Term::parse)
                        .collect(),
                )
            } else {
                (Term::parse(body), Vec::new())
            };
            assert_eq!(
                clause_text(&head, &terms),
                line,
                "canonical observed clause bytes"
            );
            canonical_variables(std::iter::once(&head).chain(&terms));
            if !terms.is_empty() {
                let old = old_clauses
                    .get(c)
                    .expect("source clause occurrence")
                    .strip_suffix('.')
                    .unwrap();
                let old_body = separators(old, " :- ")
                    .first()
                    .map(|at| &old[at + 4..])
                    .expect("source rule body");
                terms = inverse(
                    &terms,
                    &split(old_body)
                        .into_iter()
                        .map(Term::parse)
                        .collect::<Vec<_>>(),
                );
            }
            c += 1;
            let mut head = head;
            let mut names = BTreeMap::new();
            head.rename(&mut names);
            for t in &mut terms {
                t.rename(&mut names);
            }
            clause_text(&head, &terms)
        } else if line.starts_with("'$guideline_query_projection'(") {
            let mut t = Term::parse(line.strip_suffix('.').unwrap());
            assert_eq!(
                format!("{}.", t.print()),
                line,
                "canonical observed record bytes"
            );
            canonical_variables([&t]);
            let old = Term::parse(
                old_queries
                    .get(q)
                    .expect("source question occurrence")
                    .strip_suffix('.')
                    .unwrap(),
            );
            if let (Term::App(_, a), Term::App(_, b)) = (&mut t, old) {
                let Term::App(_, goal) = &mut a[0] else {
                    panic!("query goal wrapper")
                };
                let old = b[0].args("goal").unwrap();
                goal[0] = conjunction(&inverse(&goals(&goal[0]), &goals(&old[0])));
            }
            q += 1;
            t.rename(&mut BTreeMap::new());
            format!("{}.", t.print())
        } else if line.starts_with("'$guideline_proof'(") {
            let mut t = Term::parse(line.strip_suffix('.').unwrap());
            assert_eq!(
                format!("{}.", t.print()),
                line,
                "canonical observed record bytes"
            );
            canonical_variables([&t]);
            let old = Term::parse(
                old_proofs
                    .get(p)
                    .expect("source proof occurrence")
                    .strip_suffix('.')
                    .unwrap(),
            );
            if let (Term::App(_, a), Term::App(_, b)) = (&mut t, old) {
                for i in [3, 4] {
                    let (Term::App(_, wrapper), Term::App(_, old_wrapper)) = (&mut a[i], &b[i])
                    else {
                        panic!("proof wrapper")
                    };
                    let (Term::List(xs), Term::List(old)) = (&mut wrapper[0], &old_wrapper[0])
                    else {
                        panic!("proof list")
                    };
                    *xs = inverse(xs, old);
                }
            }
            p += 1;
            t.rename(&mut BTreeMap::new());
            format!("{}.", t.print())
        } else {
            line.into()
        };
        out.push_str(&projected);
        out.push('\n');
    }
    out
}
