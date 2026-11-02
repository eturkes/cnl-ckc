use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

pub type Id = usize;
pub type Symbol = usize;
pub type Env = Vec<Option<Id>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Node {
    Atom(Symbol),
    Integer(String),
    Nil,
    Compound(Symbol, Vec<Id>),
    Var(usize),
}

#[derive(Default)]
pub struct Arena {
    pub nodes: Vec<Rc<Node>>,
    names: Vec<String>,
    symbols: HashMap<String, Symbol>,
    intern: HashMap<Rc<Node>, Id>,
    pub lookups: u64,
}

impl Arena {
    pub fn symbol(&mut self, name: &str) -> Symbol {
        if let Some(&id) = self.symbols.get(name) {
            return id;
        }
        let id = self.names.len();
        self.names.push(name.into());
        self.symbols.insert(name.into(), id);
        id
    }

    pub fn name(&self, id: Symbol) -> &str {
        &self.names[id]
    }

    pub fn put(&mut self, node: Node) -> Id {
        self.lookups += 1;
        if let Some(&id) = self.intern.get(&node) {
            return id;
        }
        let id = self.nodes.len();
        let node = Rc::new(node);
        self.nodes.push(node.clone());
        self.intern.insert(node, id);
        id
    }

    pub fn atom(&mut self, name: &str) -> Id {
        let name = self.symbol(name);
        self.put(Node::Atom(name))
    }

    pub fn compound(&mut self, name: &str, args: Vec<Id>) -> Id {
        let name = self.symbol(name);
        self.put(Node::Compound(name, args))
    }

    pub fn node(&self, id: Id) -> &Node {
        &self.nodes[id]
    }

    pub fn atom_name(&self, id: Id) -> Option<&str> {
        match self.node(id) {
            Node::Atom(name) => Some(self.name(*name)),
            _ => None,
        }
    }

    pub fn call(&self, id: Id) -> Option<(Symbol, &[Id])> {
        match self.node(id) {
            Node::Compound(name, args) => Some((*name, args)),
            _ => None,
        }
    }

    pub fn named(&self, id: Id, name: &str, arity: usize) -> Option<&[Id]> {
        let (symbol, args) = self.call(id)?;
        (self.name(symbol) == name && args.len() == arity).then_some(args)
    }

    pub fn vars(&self, id: Id, out: &mut BTreeSet<usize>) {
        match self.node(id) {
            Node::Var(v) => {
                out.insert(*v);
            }
            Node::Compound(_, args) => {
                for arg in args {
                    self.vars(*arg, out);
                }
            }
            _ => {}
        }
    }

    pub fn is_ground(&self, id: Id) -> bool {
        match self.node(id) {
            Node::Var(_) => false,
            Node::Compound(_, args) => args.iter().all(|arg| self.is_ground(*arg)),
            _ => true,
        }
    }

    pub fn bound(&self, id: Id, env: &Env) -> bool {
        match self.node(id) {
            Node::Var(v) => env.get(*v).is_some_and(Option::is_some),
            Node::Compound(_, args) => args.iter().all(|arg| self.bound(*arg, env)),
            _ => true,
        }
    }

    pub fn instantiate(&mut self, id: Id, env: &Env) -> Option<Id> {
        match self.node(id).clone() {
            Node::Var(v) => env.get(v).copied().flatten(),
            Node::Compound(name, args) => {
                if self.is_ground(id) {
                    return Some(id);
                }
                let args = args
                    .into_iter()
                    .map(|arg| self.instantiate(arg, env))
                    .collect::<Option<Vec<_>>>()?;
                Some(self.put(Node::Compound(name, args)))
            }
            _ => Some(id),
        }
    }

    pub fn match_ground(&self, pattern: Id, ground: Id, env: &mut Env) -> bool {
        match (self.node(pattern), self.node(ground)) {
            (Node::Var(v), _) => {
                if let Some(value) = env[*v] {
                    value == ground
                } else {
                    env[*v] = Some(ground);
                    true
                }
            }
            (Node::Compound(p, ps), Node::Compound(g, gs)) => {
                p == g
                    && ps.len() == gs.len()
                    && ps
                        .iter()
                        .zip(gs)
                        .all(|(p, g)| self.match_ground(*p, *g, env))
            }
            _ => pattern == ground,
        }
    }

    pub fn list(&self, mut id: Id) -> Option<Vec<Id>> {
        let mut out = Vec::new();
        loop {
            match self.node(id) {
                Node::Nil => return Some(out),
                Node::Compound(name, args) if self.name(*name) == "[|]" && args.len() == 2 => {
                    out.push(args[0]);
                    id = args[1];
                }
                _ => return None,
            }
        }
    }

    pub fn make_list(&mut self, values: &[Id]) -> Id {
        let mut tail = self.put(Node::Nil);
        for &value in values.iter().rev() {
            tail = self.compound("[|]", vec![value, tail]);
        }
        tail
    }

    pub fn render(&self, id: Id) -> String {
        match self.node(id) {
            Node::Atom(name) => quote_atom(self.name(*name)),
            Node::Integer(n) => n.clone(),
            Node::Nil => "[]".into(),
            Node::Var(v) => variable_name(*v),
            Node::Compound(name, args) if self.name(*name) == "[|]" && args.len() == 2 => {
                let mut out = String::from("[");
                let mut next = id;
                loop {
                    let (_, args) = self.call(next).expect("list cell");
                    out.push_str(&self.render(args[0]));
                    match self.node(args[1]) {
                        Node::Nil => {
                            out.push(']');
                            break;
                        }
                        Node::Compound(n, a) if self.name(*n) == "[|]" && a.len() == 2 => {
                            out.push(',');
                            next = args[1];
                        }
                        _ => {
                            out.push('|');
                            out.push_str(&self.render(args[1]));
                            out.push(']');
                            break;
                        }
                    }
                }
                out
            }
            Node::Compound(name, args) if self.name(*name) == "{}" && args.len() == 1 => {
                format!("{{{}}}", self.render(args[0]))
            }
            Node::Compound(name, args) => {
                let args = args
                    .iter()
                    .map(|id| self.render(*id))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({args})", quote_atom(self.name(*name)))
            }
        }
    }

    pub fn compare(&self, a: Id, b: Id) -> Ordering {
        if a == b {
            return Ordering::Equal;
        }
        fn rank(n: &Node) -> u8 {
            match n {
                Node::Var(_) => 0,
                Node::Integer(_) => 1,
                Node::Atom(_) | Node::Nil => 2,
                Node::Compound(_, _) => 3,
            }
        }
        let left = self.node(a);
        let right = self.node(b);
        rank(left)
            .cmp(&rank(right))
            .then_with(|| match (left, right) {
                (Node::Var(a), Node::Var(b)) => a.cmp(b),
                (Node::Integer(a), Node::Integer(b)) => integer_compare(a, b),
                (Node::Atom(a), Node::Atom(b)) => self.name(*a).cmp(self.name(*b)),
                (Node::Nil, Node::Atom(b)) => "[]".cmp(self.name(*b)),
                (Node::Atom(a), Node::Nil) => self.name(*a).cmp("[]"),
                (Node::Compound(a, aa), Node::Compound(b, bb)) => aa
                    .len()
                    .cmp(&bb.len())
                    .then_with(|| self.name(*a).cmp(self.name(*b)))
                    .then_with(|| {
                        aa.iter()
                            .zip(bb)
                            .map(|(a, b)| self.compare(*a, *b))
                            .find(|o| *o != Ordering::Equal)
                            .unwrap_or(Ordering::Equal)
                    }),
                _ => Ordering::Equal,
            })
    }
}

pub fn natural_compare(a: &str, b: &str) -> Ordering {
    let a = a.trim_start_matches('0');
    let b = b.trim_start_matches('0');
    a.len().cmp(&b.len()).then_with(|| a.cmp(b))
}

pub fn integer_compare(a: &str, b: &str) -> Ordering {
    match (a.strip_prefix('-'), b.strip_prefix('-')) {
        (Some(a), Some(b)) => natural_compare(b, a),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => natural_compare(a, b),
    }
}

pub fn variable_name(v: usize) -> String {
    let letter = char::from(b'A' + (v % 26) as u8);
    if v < 26 {
        letter.to_string()
    } else {
        format!("{letter}{}", v / 26)
    }
}

pub fn quote_atom(name: &str) -> String {
    let bytes = name.as_bytes();
    let alpha = bytes.first().is_some_and(u8::is_ascii_lowercase)
        && bytes
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || *b == b'_');
    let graphic = !bytes.is_empty()
        && name != "."
        && !name.starts_with("/*")
        && bytes.iter().all(|b| b"#$&*+-./:<=>?@\\^~".contains(b));
    if alpha || graphic || matches!(name, ";" | "!" | "{}") {
        return name.into();
    }
    let mut out = String::from("'");
    for c in name.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\x07' => out.push_str("\\a"),
            '\x08' => out.push_str("\\b"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\x0b' => out.push_str("\\v"),
            '\x0c' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            c if c < '\x20' || c == '\x7f' => {
                out.push_str(&format!("\\u{:04X}", c as u32));
            }
            _ => out.push(c),
        }
    }
    out.push('\'');
    out
}

pub struct Parser<'a> {
    text: &'a str,
    pub at: usize,
    pub vars: Vec<String>,
    pub anonymous: Vec<(usize, usize)>,
    var_map: HashMap<String, usize>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            at: 0,
            vars: Vec::new(),
            anonymous: Vec::new(),
            var_map: HashMap::new(),
        }
    }

    pub fn error(&self, message: &str) -> String {
        format!("byte {}: {message}", self.at)
    }
    pub fn skip(&mut self) {
        while self
            .text
            .as_bytes()
            .get(self.at)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.at += 1;
        }
    }
    pub fn eat(&mut self, token: &str) -> bool {
        self.skip();
        if self.text[self.at..].starts_with(token) {
            self.at += token.len();
            true
        } else {
            false
        }
    }
    pub fn require(&mut self, token: &str) -> Result<(), String> {
        if self.eat(token) {
            Ok(())
        } else {
            Err(self.error(&format!("expected {token}")))
        }
    }
    pub fn done(&mut self) -> bool {
        self.skip();
        self.at == self.text.len()
    }
    pub fn finish(&mut self) -> Result<(), String> {
        self.eat(".");
        if self.done() {
            Ok(())
        } else {
            Err(self.error("trailing bytes"))
        }
    }

    pub fn term(&mut self, arena: &mut Arena) -> Result<Id, String> {
        self.skip();
        let Some(&first) = self.text.as_bytes().get(self.at) else {
            return Err(self.error("term expected"));
        };
        if first == b'[' {
            self.at += 1;
            if self.eat("]") {
                return Ok(arena.put(Node::Nil));
            }
            let mut items = vec![self.term(arena)?];
            while self.eat(",") {
                items.push(self.term(arena)?);
            }
            let mut tail = if self.eat("|") {
                self.term(arena)?
            } else {
                arena.put(Node::Nil)
            };
            self.require("]")?;
            for item in items.into_iter().rev() {
                tail = arena.compound("[|]", vec![item, tail]);
            }
            return Ok(tail);
        }
        if first == b'{' {
            self.at += 1;
            if self.eat("}") {
                return Ok(arena.atom("{}"));
            }
            let arg = self.term(arena)?;
            self.require("}")?;
            return Ok(arena.compound("{}", vec![arg]));
        }
        if first.is_ascii_uppercase() || first == b'_' {
            let start = self.at;
            let name = self.identifier();
            let v = if name == "_" {
                self.vars.len()
            } else {
                self.var_map.get(&name).copied().unwrap_or(self.vars.len())
            };
            if v == self.vars.len() {
                self.vars.push(name.clone());
            }
            if name != "_" {
                self.var_map.insert(name, v);
            } else {
                self.anonymous.push((start, v));
            }
            return Ok(arena.put(Node::Var(v)));
        }
        if first.is_ascii_digit()
            || (first == b'-'
                && self
                    .text
                    .as_bytes()
                    .get(self.at + 1)
                    .is_some_and(u8::is_ascii_digit))
        {
            let start = self.at;
            if first == b'-' {
                self.at += 1;
            }
            while self
                .text
                .as_bytes()
                .get(self.at)
                .is_some_and(u8::is_ascii_digit)
            {
                self.at += 1;
            }
            let n = &self.text[start..self.at];
            let digits = n.trim_start_matches('-');
            if (digits.len() > 1 && digits.starts_with('0')) || n == "-0" {
                return Err(self.error("noncanonical integer"));
            }
            return Ok(arena.put(Node::Integer(n.into())));
        }
        let name = if first == b'\'' {
            self.quoted()?
        } else if first.is_ascii_lowercase() {
            self.identifier()
        } else {
            let start = self.at;
            while self
                .text
                .as_bytes()
                .get(self.at)
                .is_some_and(|b| b"#$&*+-/:<=>?@\\^~;!".contains(b))
            {
                self.at += 1;
            }
            if self.at == start {
                return Err(self.error("invalid term"));
            }
            self.text[start..self.at].to_string()
        };
        if self.eat("(") {
            let mut args = vec![self.term(arena)?];
            while self.eat(",") {
                args.push(self.term(arena)?);
            }
            self.require(")")?;
            Ok(arena.compound(&name, args))
        } else {
            Ok(arena.atom(&name))
        }
    }

    pub fn identifier(&mut self) -> String {
        let start = self.at;
        while self
            .text
            .as_bytes()
            .get(self.at)
            .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
        {
            self.at += 1;
        }
        self.text[start..self.at].into()
    }

    fn quoted(&mut self) -> Result<String, String> {
        self.at += 1;
        let mut out = String::new();
        loop {
            let Some(c) = self.text[self.at..].chars().next() else {
                return Err(self.error("unterminated atom"));
            };
            self.at += c.len_utf8();
            match c {
                '\'' => return Ok(out),
                '\\' => {
                    let Some(c) = self.text[self.at..].chars().next() else {
                        return Err(self.error("unterminated escape"));
                    };
                    self.at += c.len_utf8();
                    match c {
                        'a' => out.push('\x07'),
                        'b' => out.push('\x08'),
                        't' => out.push('\t'),
                        'n' => out.push('\n'),
                        'v' => out.push('\x0b'),
                        'f' => out.push('\x0c'),
                        'r' => out.push('\r'),
                        '\\' | '\'' => out.push(c),
                        'u' => {
                            let Some(hex) = self.text.get(self.at..self.at + 4) else {
                                return Err(self.error("short unicode escape"));
                            };
                            let value = u32::from_str_radix(hex, 16)
                                .ok()
                                .and_then(char::from_u32)
                                .ok_or_else(|| self.error("invalid unicode escape"))?;
                            out.push(value);
                            self.at += 4;
                        }
                        _ => return Err(self.error("invalid escape")),
                    }
                }
                _ => out.push(c),
            }
        }
    }
}

pub fn parse_term(arena: &mut Arena, text: &str) -> Result<Id, String> {
    let mut parser = Parser::new(text);
    let term = parser.term(arena)?;
    parser.finish()?;
    Ok(term)
}
