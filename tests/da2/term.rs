use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Term {
    Atom(String),
    Var(String),
    Call(String, Vec<Term>),
    List(Vec<Term>),
}
pub fn a(s: impl ToString) -> Term {
    Term::Atom(s.to_string())
}
pub fn v(s: impl ToString) -> Term {
    Term::Var(s.to_string())
}
pub fn c(s: &str, xs: Vec<Term>) -> Term {
    Term::Call(s.into(), xs)
}
pub fn quote(s: &str) -> String {
    let digits = s.strip_prefix('-').unwrap_or(s);
    if s == "-"
        || (!digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
        || (s.bytes().next().is_some_and(|b| b.is_ascii_lowercase())
            && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'))
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}
impl Term {
    pub fn name(&self) -> &str {
        match self {
            Self::Atom(s) | Self::Call(s, _) => s,
            _ => "",
        }
    }
    pub fn args(&self) -> &[Term] {
        match self {
            Self::Call(_, xs) | Self::List(xs) => xs,
            _ => &[],
        }
    }
    pub fn text(&self, vars: &mut BTreeMap<String, usize>) -> String {
        match self {
            Self::Atom(s) => quote(s),
            Self::Var(s) => {
                let next = vars.len();
                let n = *vars.entry(s.clone()).or_insert(next);
                let letter = char::from(b'A' + (n % 26) as u8);
                if n < 26 {
                    letter.to_string()
                } else {
                    format!("{letter}{}", n / 26)
                }
            }
            Self::Call(s, xs) => format!(
                "{}({})",
                quote(s),
                xs.iter()
                    .map(|x| x.text(vars))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::List(xs) => format!(
                "[{}]",
                xs.iter()
                    .map(|x| x.text(vars))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
    pub fn render(&self) -> String {
        self.text(&mut BTreeMap::new())
    }
    pub fn sub(&self, map: &BTreeMap<Term, Term>) -> Term {
        if let Some(t) = map.get(self) {
            return t.clone();
        }
        match self {
            Self::Call(n, xs) => c(n, xs.iter().map(|x| x.sub(map)).collect()),
            Self::List(xs) => Self::List(xs.iter().map(|x| x.sub(map)).collect()),
            _ => self.clone(),
        }
    }
}
pub fn split(s: &str) -> Vec<&str> {
    let (mut depth, mut quoted, mut escaped, mut start) = (0i32, false, false, 0usize);
    let mut parts = Vec::new();
    for (i, ch) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '\'' {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        match ch {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(s[start..i].trim());
                start = i + 1;
            }
            _ => (),
        }
    }
    assert_eq!(depth, 0, "balanced term: {s}");
    if !s.is_empty() {
        parts.push(s[start..].trim());
    }
    parts
}
pub fn parse(s: &str) -> Term {
    let s = s.trim();
    if s.starts_with('[') {
        assert!(s.ends_with(']'));
        return Term::List(split(&s[1..s.len() - 1]).into_iter().map(parse).collect());
    }
    let (mut quoted, mut escaped) = (false, false);
    for (i, ch) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '\'' {
            quoted = !quoted;
        } else if ch == '(' && !quoted {
            assert!(s.ends_with(')'));
            return c(
                parse(&s[..i]).name(),
                split(&s[i + 1..s.len() - 1])
                    .into_iter()
                    .map(parse)
                    .collect(),
            );
        }
    }
    if s.starts_with('\'') {
        a(s[1..s.len() - 1].replace("\\'", "'").replace("\\\\", "\\"))
    } else if s
        .bytes()
        .next()
        .is_some_and(|b| b.is_ascii_uppercase() || b == b'_')
    {
        v(s)
    } else {
        a(s)
    }
}
pub fn envelope(dump: &str, field: &str) -> Term {
    let prefix = format!("{field}(");
    let line = dump
        .lines()
        .find(|s| s.starts_with(&prefix))
        .unwrap_or_else(|| panic!("dump lacks {field}: {dump}"));
    let term = parse(line.strip_suffix('.').unwrap());
    assert_eq!(term.args().len(), 1);
    term.args()[0].clone()
}
pub fn strip(t: &Term) -> &Term {
    if t.name() == "-" && t.args().len() == 2 {
        &t.args()[0]
    } else {
        t
    }
}
pub fn leaves(t: &Term) -> Vec<Term> {
    match t {
        Term::List(xs) => xs.iter().flat_map(leaves).collect(),
        _ => vec![t.clone()],
    }
}
pub fn conjunction(xs: &[Term]) -> Term {
    xs.iter()
        .rev()
        .cloned()
        .reduce(|tail, head| c(",", vec![head, tail]))
        .expect("nonempty conjunction")
}

// Exact fractions for the generated DA2 domain (<=30 decimal digits); no float conversions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rat {
    pub n: i128,
    pub d: i128,
}
impl Rat {
    pub fn new(n: i128, d: i128) -> Self {
        assert_ne!(d, 0);
        let (mut x, mut y) = (n.abs(), d.abs());
        while y != 0 {
            (x, y) = (y, x % y);
        }
        Self {
            n: n / x * d.signum(),
            d: d.abs() / x,
        }
    }
    pub fn decimal(s: &str) -> Self {
        if let Some((i, f)) = s.split_once('.') {
            let d = 10i128.pow(f.len() as u32);
            Self::new(
                i.parse::<i128>().unwrap() * d + f.parse::<i128>().unwrap(),
                d,
            )
        } else {
            Self::new(s.parse().unwrap(), 1)
        }
    }
    pub fn mul(self, rhs: Self) -> Self {
        let a = Self::new(self.n, rhs.d);
        let b = Self::new(rhs.n, self.d);
        Self::new(a.n.checked_mul(b.n).unwrap(), a.d.checked_mul(b.d).unwrap())
    }
    pub fn div(self, rhs: Self) -> Option<Self> {
        (rhs.n != 0).then(|| self.mul(Self::new(rhs.d, rhs.n)))
    }
    pub fn term(self) -> Term {
        if self.d == 1 {
            a(self.n)
        } else {
            c("rational", vec![a(self.n), a(self.d)])
        }
    }
}
pub fn numeric_lexemes(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if !b[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i + 1 < b.len() && b[i] == b'.' && b[i + 1].is_ascii_digit() {
            i += 1;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
        }
        let blocked_before =
            start > 0 && (b[start - 1].is_ascii_alphanumeric() || b"-_.".contains(&b[start - 1]));
        let blocked_after = i < b.len() && (b[i].is_ascii_alphanumeric() || b"-_".contains(&b[i]));
        if !blocked_before && !blocked_after {
            out.push(&line[start..i]);
        }
    }
    out
}
pub fn numeric_token(t: &Term) -> bool {
    if let Term::Call(name, args) = t {
        return name == "$decimal" && matches!(args.as_slice(), [Term::Atom(_)]);
    }
    let Term::Atom(text) = t else {
        return false;
    };
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let unsigned = text.strip_prefix('-').unwrap_or(text);
    let (mantissa, exponent) = unsigned
        .split_once('e')
        .map_or((unsigned, None), |(m, e)| (m, Some(e)));
    if exponent.is_some_and(|e| !digits(e.strip_prefix(['-', '+']).unwrap_or(e))) {
        return false;
    }
    let mut parts = mantissa.split('.');
    digits(parts.next().unwrap()) && parts.next().is_none_or(digits) && parts.next().is_none()
}
