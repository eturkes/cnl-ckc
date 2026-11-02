use super::text::{Document, Goal, Line, Term};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot {
    Lexical,
    Identity,
    Fixed,
}
fn slot(name: &str, index: usize) -> Slot {
    match (name, index) {
        ("guideline_entity" | "guideline_event" | "guideline_property", 2) => Slot::Lexical,
        ("$guideline_id", 1)
        | ("guideline_document" | "guideline_supersession" | "guideline_exclusion", 0) => {
            Slot::Identity
        }
        _ => Slot::Fixed,
    }
}
#[derive(Clone)]
pub struct Rename {
    pub index: usize,
    pub shared: BTreeSet<String>,
    pub private: Option<BTreeSet<String>>,
    pub preserve_ids: bool,
}
impl Rename {
    pub fn identity() -> Self {
        Self {
            index: 0,
            shared: BTreeSet::new(),
            private: None,
            preserve_ids: false,
        }
    }
    pub fn dormant(index: usize, common_roles: bool) -> Self {
        Self {
            index,
            shared: if common_roles {
                BTreeSet::from(["patient".into(), "clinician".into()])
            } else {
                BTreeSet::new()
            },
            private: None,
            preserve_ids: false,
        }
    }
    pub fn active(index: usize, produced: &BTreeSet<String>) -> Self {
        Self {
            index,
            shared: BTreeSet::new(),
            private: Some(produced.clone()),
            preserve_ids: false,
        }
    }
    fn target(&self, name: &str, kind: Slot) -> String {
        let lexical = kind == Slot::Lexical
            && !self.shared.contains(name)
            && self.private.as_ref().is_none_or(|p| p.contains(name));
        if self.index == 0 {
            name.to_owned()
        } else if kind == Slot::Identity && !self.preserve_ids {
            format!("sb{:04}-{name}", self.index)
        } else if lexical {
            format!("sb{:04}__{name}", self.index)
        } else {
            name.to_owned()
        }
    }
    fn term(&self, term: &mut Term) {
        match term {
            Term::Compound(_, name, args) => {
                for (i, a) in args.iter_mut().enumerate() {
                    if let Some(value) = a.atom() {
                        let target = self.target(value, slot(name, i));
                        if target != value {
                            a.rename_atom(target);
                        }
                    }
                    self.term(a);
                }
            }
            Term::List(ts, tail) => {
                for t in ts {
                    self.term(t);
                }
                if let Some(t) = tail {
                    self.term(t);
                }
            }
            _ => (),
        }
    }
    fn header(&self, line: &str, input: &Document) -> String {
        let docid = input
            .clauses()
            .filter_map(|c| c.head.call())
            .find(|(name, _)| *name == "guideline_document")
            .and_then(|(_, args)| args.first().and_then(Term::atom));
        if let Some(docid) = docid {
            let prefix = format!("% {docid}.pl compiled");
            if let Some(rest) = line.strip_prefix(&prefix) {
                return format!("% {}.pl compiled{rest}", self.target(docid, Slot::Identity));
            }
        }
        line.to_owned()
    }
    pub fn apply(&self, input: &Document) -> Result<Document, String> {
        let mut output = input.clone();
        for line in &mut output.lines {
            if let Line::Text(text) = line {
                *text = self.header(text, input);
            }
            if let Line::Clause(c) = line {
                self.term(&mut c.head);
                for g in &mut c.body {
                    match g {
                        Goal::Positive(t) => self.term(t),
                        Goal::Naf(ts) => {
                            for t in ts {
                                self.term(t);
                            }
                        }
                    }
                }
            }
        }
        self.audit(input, &output)?;
        output.validate()?;
        Ok(output)
    }
    pub fn audit(&self, input: &Document, output: &Document) -> Result<(), String> {
        if input.lines.len() != output.lines.len() || input.singleton != output.singleton {
            return Err("transform document shape".into());
        }
        let mut inverse = BTreeMap::new();
        for (before, after) in input.lines.iter().zip(&output.lines) {
            match (before, after) {
                (Line::Text(a), Line::Text(b)) if self.header(a, input) == *b => (),
                (Line::Clause(a), Line::Clause(b)) if a.body.len() == b.body.len() => {
                    audit_term(&a.head, &b.head, None, self, &mut inverse)?;
                    for (a, b) in a.body.iter().zip(&b.body) {
                        match (a, b) {
                            (Goal::Positive(a), Goal::Positive(b)) => {
                                audit_term(a, b, None, self, &mut inverse)?
                            }
                            (Goal::Naf(a), Goal::Naf(b)) if a.len() == b.len() => {
                                for (a, b) in a.iter().zip(b) {
                                    audit_term(a, b, None, self, &mut inverse)?;
                                }
                            }
                            _ => return Err("transform condition/NAF shape".into()),
                        }
                    }
                }
                _ => return Err("transform clause/condition shape".into()),
            }
        }
        Ok(())
    }
}
fn audit_term(
    a: &Term,
    b: &Term,
    at: Option<Slot>,
    rename: &Rename,
    inverse: &mut BTreeMap<(SlotKey, String), String>,
) -> Result<(), String> {
    match (a, b) {
        (Term::Atom(_, source), Term::Atom(_, target)) => {
            let kind = at.unwrap_or(Slot::Fixed);
            if *target != rename.target(source, kind) {
                return Err(format!("transform atom {source}"));
            }
            if kind != Slot::Fixed {
                let key = (
                    if kind == Slot::Lexical {
                        SlotKey::Lexical
                    } else {
                        SlotKey::Identity
                    },
                    target.clone(),
                );
                if inverse
                    .insert(key, source.clone())
                    .is_some_and(|old| old != *source)
                {
                    return Err("transform non-injective rename".into());
                }
            }
        }
        (Term::Number(a), Term::Number(b)) | (Term::Var(a), Term::Var(b)) if a == b => (),
        (Term::Compound(a_raw, name, aa), Term::Compound(b_raw, bn, ba))
            if name == bn && a_raw == b_raw && aa.len() == ba.len() =>
        {
            for (i, (a, b)) in aa.iter().zip(ba).enumerate() {
                audit_term(a, b, Some(slot(name, i)), rename, inverse)?;
            }
        }
        (Term::List(aa, at), Term::List(ba, bt)) if aa.len() == ba.len() => {
            for (a, b) in aa.iter().zip(ba) {
                audit_term(a, b, None, rename, inverse)?;
            }
            match (at, bt) {
                (Some(a), Some(b)) => audit_term(a, b, None, rename, inverse)?,
                (None, None) => (),
                _ => return Err("transform list tail".into()),
            }
        }
        _ => return Err("transform number/variable/term shape".into()),
    }
    Ok(())
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SlotKey {
    Lexical,
    Identity,
}
pub fn head_produced(documents: &[Document]) -> BTreeSet<String> {
    documents
        .iter()
        .flat_map(Document::clauses)
        .filter_map(|c| c.head.call())
        .filter(|(name, _)| {
            matches!(
                *name,
                "guideline_entity" | "guideline_event" | "guideline_property"
            )
        })
        .filter_map(|(_, args)| args.get(2).and_then(Term::atom))
        .map(str::to_owned)
        .collect()
}
pub fn reject_namespace_collision(
    documents: &[Document],
    patient: &Document,
) -> Result<(), String> {
    fn walk(t: &Term) -> bool {
        match t {
            Term::Atom(_, a) => a.strip_prefix("sb").is_some_and(|r| {
                r.len() >= 6
                    && r.as_bytes()[..4].iter().all(u8::is_ascii_digit)
                    && (r[4..].starts_with("__") || r[4..].starts_with('-'))
            }),
            Term::Compound(_, _, aa) => aa.iter().any(walk),
            Term::List(aa, tail) => aa.iter().any(walk) || tail.as_ref().is_some_and(|t| walk(t)),
            _ => false,
        }
    }
    if documents
        .iter()
        .chain(std::iter::once(patient))
        .flat_map(Document::clauses)
        .flat_map(|c| c.terms())
        .any(walk)
    {
        return Err("transform namespace collides with an input atom".into());
    }
    Ok(())
}
