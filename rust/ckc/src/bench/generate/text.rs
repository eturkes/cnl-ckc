// The public K1 seam returns canonical text; this AST keeps its atom spelling +
// variable identities. K1 judges every input and every re-rendered document.
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Atom(String, String),
    Number(String),
    Var(String),
    Compound(String, String, Vec<Term>),
    List(Vec<Term>, Option<Box<Term>>),
}
impl Term {
    pub fn atom(&self) -> Option<&str> {
        match self {
            Self::Atom(_, value) => Some(value),
            _ => None,
        }
    }
    pub fn call(&self) -> Option<(&str, &[Term])> {
        match self {
            Self::Compound(_, name, args) => Some((name, args)),
            _ => None,
        }
    }
    pub fn rename_atom(&mut self, value: String) {
        *self = Self::Atom(atom_text(&value), value);
    }
    pub fn variables(&self, out: &mut BTreeMap<String, usize>) {
        match self {
            Self::Var(v) => *out.entry(v.clone()).or_default() += 1,
            Self::Compound(_, _, args) => {
                for a in args {
                    a.variables(out);
                }
            }
            Self::List(args, tail) => {
                for a in args {
                    a.variables(out);
                }
                if let Some(t) = tail {
                    t.variables(out);
                }
            }
            _ => (),
        }
    }
    fn write(&self, counts: &BTreeMap<String, usize>, singleton: bool, out: &mut String) {
        match self {
            Self::Atom(raw, _) | Self::Number(raw) => out.push_str(raw),
            Self::Var(v) => out.push_str(if singleton && counts.get(v) == Some(&1) {
                "_"
            } else {
                v
            }),
            Self::Compound(raw, _, args) => {
                out.push_str(raw);
                out.push('(');
                write_terms(args, counts, singleton, out);
                out.push(')');
            }
            Self::List(args, tail) => {
                out.push('[');
                write_terms(args, counts, singleton, out);
                if let Some(t) = tail {
                    out.push('|');
                    t.write(counts, singleton, out);
                }
                out.push(']');
            }
        }
    }
}
fn write_terms(args: &[Term], counts: &BTreeMap<String, usize>, singleton: bool, out: &mut String) {
    for (i, a) in args.iter().enumerate() {
        if i != 0 {
            out.push(',');
        }
        a.write(counts, singleton, out);
    }
}
pub fn atom_text(value: &str) -> String {
    let bytes = value.as_bytes();
    if bytes.first().is_some_and(u8::is_ascii_lowercase)
        && bytes
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || *b == b'_')
    {
        return value.to_owned();
    }
    let mut out = String::from("'");
    for c in value.chars() {
        match c {
            '\'' => out.push_str("\\'"),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\u{b}' => out.push_str("\\v"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 || c == '\u{7f}' => {
                out.push_str(&format!("\\u{:04X}", c as u32))
            }
            c => out.push(c),
        }
    }
    out.push('\'');
    out
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Goal {
    Positive(Term),
    Naf(Vec<Term>),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clause {
    pub head: Term,
    pub body: Vec<Goal>,
}
impl Clause {
    pub fn terms(&self) -> impl Iterator<Item = &Term> {
        std::iter::once(&self.head).chain(self.body.iter().flat_map(|g| match g {
            Goal::Positive(t) => std::slice::from_ref(t).iter(),
            Goal::Naf(ts) => ts.iter(),
        }))
    }
    pub fn render(&self, singleton: bool) -> String {
        let mut counts = BTreeMap::new();
        for t in self.terms() {
            t.variables(&mut counts);
        }
        let mut out = String::new();
        self.head.write(&counts, singleton, &mut out);
        if !self.body.is_empty() {
            out.push_str(" :- ");
        }
        for (i, g) in self.body.iter().enumerate() {
            if i != 0 {
                out.push_str(", ");
            }
            match g {
                Goal::Positive(t) => t.write(&counts, singleton, &mut out),
                Goal::Naf(ts) => {
                    out.push_str("\\+ ");
                    if ts.len() != 1 {
                        out.push('(');
                    }
                    for (i, t) in ts.iter().enumerate() {
                        if i != 0 {
                            out.push_str(", ");
                        }
                        t.write(&counts, singleton, &mut out);
                    }
                    if ts.len() != 1 {
                        out.push(')');
                    }
                }
            }
        }
        out.push_str(".\n");
        out
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    Text(String),
    Clause(Clause),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    pub lines: Vec<Line>,
    pub singleton: bool,
}
impl Document {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if let ckc_kernel::EV1Verdict::Reject { at } = ckc_kernel::contract::v1_check(bytes) {
            return Err(format!("canonical input rejected at byte {at}"));
        }
        let read = ckc_kernel::contract::v1_read(bytes);
        let text = std::str::from_utf8(&read).map_err(|e| e.to_string())?;
        let singleton = text.lines().any(|l| l.starts_with("guideline_document("));
        let mut lines = Vec::new();
        for line in text.split_inclusive('\n') {
            if line.starts_with('%') || line.starts_with(":-") || line.trim().is_empty() {
                lines.push(Line::Text(line.to_owned()));
                continue;
            }
            let mut p = Parser {
                text: line.as_bytes(),
                at: 0,
            };
            let head = p.term(0)?;
            let mut body = Vec::new();
            if p.take(b" :- ") {
                loop {
                    if p.take(b"\\+ ") {
                        let grouped = p.take(b"(");
                        let mut terms = vec![p.term(0)?];
                        if grouped {
                            while p.take(b", ") {
                                terms.push(p.term(0)?);
                            }
                            p.need(b")")?;
                        }
                        body.push(Goal::Naf(terms));
                    } else {
                        body.push(Goal::Positive(p.term(0)?));
                    }
                    if !p.take(b", ") {
                        break;
                    }
                }
            }
            p.need(b".\n")?;
            if p.at != p.text.len() {
                return Err("text after clause".into());
            }
            lines.push(Line::Clause(Clause { head, body }));
        }
        let doc = Self { lines, singleton };
        if doc.render() != bytes {
            return Err("canonical read/render differs".into());
        }
        Ok(doc)
    }
    pub fn clauses(&self) -> impl Iterator<Item = &Clause> {
        self.lines.iter().filter_map(|l| match l {
            Line::Clause(c) => Some(c),
            _ => None,
        })
    }
    pub fn render(&self) -> Vec<u8> {
        self.lines
            .iter()
            .map(|l| match l {
                Line::Text(s) => s.clone(),
                Line::Clause(c) => c.render(self.singleton),
            })
            .collect::<String>()
            .into_bytes()
    }
    pub fn validate(&self) -> Result<Vec<u8>, String> {
        let bytes = self.render();
        let parsed = Self::parse(&bytes)?;
        if parsed != *self {
            return Err("canonical AST round trip differs".into());
        }
        Ok(bytes)
    }
}
struct Parser<'a> {
    text: &'a [u8],
    at: usize,
}
impl Parser<'_> {
    fn take(&mut self, token: &[u8]) -> bool {
        if self.text[self.at..].starts_with(token) {
            self.at += token.len();
            true
        } else {
            false
        }
    }
    fn need(&mut self, token: &[u8]) -> Result<(), String> {
        if self.take(token) {
            Ok(())
        } else {
            Err(format!("term syntax at byte {}", self.at))
        }
    }
    fn term(&mut self, depth: usize) -> Result<Term, String> {
        if depth > 256 {
            return Err("term depth exceeds generator parser bound".into());
        }
        if self.take(b"[") {
            let mut args = Vec::new();
            let mut tail = None;
            if !self.take(b"]") {
                args.push(self.term(depth + 1)?);
                while self.take(b",") {
                    args.push(self.term(depth + 1)?);
                }
                if self.take(b"|") {
                    tail = Some(Box::new(self.term(depth + 1)?));
                }
                self.need(b"]")?;
            }
            return Ok(Term::List(args, tail));
        }
        let start = self.at;
        let Some(&first) = self.text.get(start) else {
            return Err("missing term".into());
        };
        if first == b'\'' {
            self.at += 1;
            loop {
                match self.text.get(self.at) {
                    Some(b'\\') => {
                        self.at += 2;
                    }
                    Some(b'\'') => {
                        self.at += 1;
                        break;
                    }
                    Some(_) => self.at += 1,
                    None => return Err("unterminated atom".into()),
                }
            }
        } else if first.is_ascii_digit()
            || first == b'-' && self.text.get(start + 1).is_some_and(u8::is_ascii_digit)
        {
            self.at += usize::from(first == b'-');
            while self.text.get(self.at).is_some_and(u8::is_ascii_digit) {
                self.at += 1;
            }
        } else if first.is_ascii_alphabetic() || first == b'_' {
            while self
                .text
                .get(self.at)
                .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
            {
                self.at += 1;
            }
        } else {
            while self
                .text
                .get(self.at)
                .is_some_and(|b| b"#$&*+-./:<=>?@\\^~".contains(b))
            {
                self.at += 1;
            }
            if start == self.at && b";!{}".contains(&first) {
                self.at += if first == b'{' && self.text.get(start + 1) == Some(&b'}') {
                    2
                } else {
                    1
                };
            }
        }
        if start == self.at {
            return Err(format!("empty term at byte {start}"));
        }
        let raw = std::str::from_utf8(&self.text[start..self.at])
            .map_err(|e| e.to_string())?
            .to_owned();
        if first.is_ascii_uppercase() || first == b'_' {
            return Ok(Term::Var(raw));
        }
        if first.is_ascii_digit()
            || first == b'-' && raw.len() > 1 && raw.as_bytes()[1].is_ascii_digit()
        {
            return Ok(Term::Number(raw));
        }
        let name = decode_atom(&raw)?;
        if self.take(b"(") {
            let mut args = vec![self.term(depth + 1)?];
            while self.take(b",") {
                args.push(self.term(depth + 1)?);
            }
            self.need(b")")?;
            Ok(Term::Compound(raw, name, args))
        } else {
            Ok(Term::Atom(raw, name))
        }
    }
}
fn decode_atom(raw: &str) -> Result<String, String> {
    if !raw.starts_with('\'') {
        return Ok(raw.to_owned());
    }
    let mut out = Vec::new();
    let bytes = raw.as_bytes();
    let mut i = 1;
    while i + 1 < bytes.len() {
        if bytes[i] != b'\\' {
            out.push(bytes[i]);
            i += 1;
            continue;
        }
        i += 1;
        let b = match bytes[i] {
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            b'a' => 7,
            b'b' => 8,
            b'f' => 12,
            b'v' => 11,
            b'u' => {
                let end = i + 5;
                let v = u8::from_str_radix(raw.get(i + 1..end).ok_or("atom unicode escape")?, 16)
                    .map_err(|e| e.to_string())?;
                i = end - 1;
                v
            }
            b'x' => {
                let start = i + 1;
                i = start;
                while bytes.get(i).is_some_and(u8::is_ascii_hexdigit) {
                    i += 1;
                }
                let v = u8::from_str_radix(&raw[start..i], 16).map_err(|e| e.to_string())?;
                if bytes.get(i) != Some(&b'\\') {
                    return Err("atom hex escape".into());
                }
                v
            }
            b => b,
        };
        out.push(b);
        i += 1;
    }
    String::from_utf8(out).map_err(|e| e.to_string())
}
