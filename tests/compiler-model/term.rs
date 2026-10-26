#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Term {
    Atom(String),
    Var(String),
    App(String, Vec<Term>),
    List(Vec<Term>),
}
impl Term {
    fn parse(s: &str) -> Self {
        let s = s.trim();
        if let Some(inside) = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            return Self::List(if inside.is_empty() {
                Vec::new()
            } else {
                split(inside).into_iter().map(Self::parse).collect()
            });
        }
        let opening = unquoted_open(s);
        if let Some(i) = opening {
            assert!(s.ends_with(')'), "reference term: {s}");
            let inner = &s[i + 1..s.len() - 1];
            Self::App(
                s[..i].to_string(),
                if inner.is_empty() {
                    Vec::new()
                } else {
                    split(inner).into_iter().map(Self::parse).collect()
                },
            )
        } else if s.as_bytes()[0].is_ascii_uppercase() || s.starts_with('_') {
            Self::Var(s.into())
        } else {
            Self::Atom(s.into())
        }
    }
    fn print(&self) -> String {
        match self {
            Self::Atom(s) | Self::Var(s) => s.clone(),
            Self::App(f, xs) => format!(
                "{f}({})",
                xs.iter().map(Self::print).collect::<Vec<_>>().join(",")
            ),
            Self::List(xs) => format!(
                "[{}]",
                xs.iter().map(Self::print).collect::<Vec<_>>().join(",")
            ),
        }
    }
    fn args(&self, f: &str) -> Option<&[Term]> {
        match self {
            Self::App(name, xs) if name == f => Some(xs),
            _ => None,
        }
    }
    fn atom(&self) -> String {
        match self {
            Self::Atom(s) => s
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .unwrap_or(s)
                .replace("\\'", "'"),
            _ => panic!("reference expected atom: {self:?}"),
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

fn atom(s: &str) -> Term {
    if s.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        Term::Atom(s.into())
    } else {
        Term::Atom(format!(
            "'{}'",
            s.replace('\\', "\\\\").replace('\'', "\\'")
        ))
    }
}

fn unquoted_open(s: &str) -> Option<usize> {
    let mut quoted = false;
    let mut escaped = false;
    for (i, b) in s.bytes().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && b == b'\\' {
            escaped = true;
            continue;
        }
        if b == b'\'' {
            quoted = !quoted;
        } else if !quoted && b == b'(' {
            return Some(i);
        }
    }
    None
}

fn split(s: &str) -> Vec<&str> {
    let (mut level, mut quoted, mut escaped, mut start) = (0isize, false, false, 0);
    let mut parts = Vec::new();
    for (i, b) in s.bytes().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && b == b'\\' {
            escaped = true;
            continue;
        }
        if b == b'\'' {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        match b {
            b'(' | b'[' => level += 1,
            b')' | b']' => level -= 1,
            b',' if level == 0 => {
                parts.push(s[start..i].trim());
                start = i + 1;
            }
            _ => (),
        }
        assert!(level >= 0, "reference delimiter: {s}");
    }
    assert!(!quoted && level == 0, "reference delimiter: {s}");
    parts.push(s[start..].trim());
    parts
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Goal {
    Pos(Term),
    Naf(Vec<Term>),
}
impl Goal {
    fn parse(s: &str) -> Self {
        if let Some(n) = s.strip_prefix("\\+ ") {
            let n = n
                .strip_prefix('(')
                .and_then(|n| n.strip_suffix(')'))
                .unwrap_or(n);
            Self::Naf(split(n).into_iter().map(Term::parse).collect())
        } else {
            Self::Pos(Term::parse(s))
        }
    }
    fn print(&self) -> String {
        match self {
            Self::Pos(t) => t.print(),
            Self::Naf(xs) if xs.len() == 1 => format!("\\+ {}", xs[0].print()),
            Self::Naf(xs) => format!(
                "\\+ ({})",
                xs.iter().map(Term::print).collect::<Vec<_>>().join(", ")
            ),
        }
    }
    fn rename(&mut self, names: &mut BTreeMap<String, String>) {
        match self {
            Self::Pos(t) => t.rename(names),
            Self::Naf(xs) => {
                for t in xs {
                    t.rename(names);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Clause {
    head: Term,
    body: Vec<Goal>,
}
impl Clause {
    fn parse(s: &str) -> Self {
        let s = s.strip_suffix('.').unwrap();
        let (head, body) = s.split_once(" :- ").unwrap_or((s, ""));
        Self {
            head: Term::parse(head),
            body: if body.is_empty() {
                Vec::new()
            } else {
                split(body).into_iter().map(Goal::parse).collect()
            },
        }
    }
    fn canonicalize(&mut self, body_first: bool) {
        let mut names = BTreeMap::new();
        if !body_first {
            self.head.rename(&mut names);
        }
        for goal in &mut self.body {
            goal.rename(&mut names);
        }
        if body_first {
            self.head.rename(&mut names);
        }
    }
    fn print(&self) -> String {
        let mut c = self.clone();
        c.canonicalize(false);
        if c.body.is_empty() {
            format!("{}.\n", c.head.print())
        } else {
            format!(
                "{} :- {}.\n",
                c.head.print(),
                c.body
                    .iter()
                    .map(Goal::print)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}
