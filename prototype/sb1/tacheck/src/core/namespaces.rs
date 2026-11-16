//! Prepared KB namespaces; fresh patient admission stays linear in patient size.
use super::extension::Original;
use super::{Abs, Program};
use sb1_checker::program::{Goal, admit, parse_clauses};
use sb1_checker::term::{Arena, Id, Node};
use std::collections::HashMap;

#[derive(Default)]
pub struct Namespaces {
    buckets: HashMap<u64, Vec<Id>>,
    documents: usize,
    pub origins: Vec<bool>,
}
fn hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100_0000_01b3)
    })
}
impl Namespaces {
    pub fn build(p: &Program, abs: &Abs) -> Self {
        fn term(p: &Program, abs: &Abs, t: Id, seen: &mut [bool], index: &mut Namespaces) {
            if seen[t] {
                return;
            }
            seen[t] = true;
            if p.arena.named(t, "$guideline_id", 5).is_some() {
                index.origins[abs.origin(abs.terms[t]).unwrap()] = true;
            }
            for key in [
                p.arena.named(t, "$guideline_id", 5).map(|a| a[1]),
                p.arena.named(t, "guideline_document", 4).map(|a| a[0]),
            ]
            .into_iter()
            .flatten()
            {
                let name = p.arena.atom_name(key).expect("admitted KB namespace atom");
                let row = index.buckets.entry(hash(name.as_bytes())).or_default();
                if !row.iter().any(|old| p.arena.atom_name(*old) == Some(name)) {
                    row.push(key);
                }
            }
            if let Some((_, args)) = p.arena.call(t) {
                for t in args {
                    term(p, abs, *t, seen, index);
                }
            }
        }
        fn goals(p: &Program, abs: &Abs, body: &[Goal], seen: &mut [bool], index: &mut Namespaces) {
            for g in body {
                match g {
                    Goal::Positive(t) | Goal::Builtin(t) => term(p, abs, *t, seen, index),
                    Goal::Not(inner) => goals(p, abs, inner, seen, index),
                }
            }
        }
        let mut index = Self {
            origins: vec![false; abs.origins.len()],
            ..Self::default()
        };
        let mut seen = vec![false; p.arena.nodes.len()];
        for (i, c) in p.clauses.iter().enumerate() {
            if p.patient[i] {
                continue;
            }
            index.documents = index.documents.max(c.document);
            term(p, abs, c.head, &mut seen, &mut index);
            goals(p, abs, &c.body, &mut seen, &mut index);
        }
        index
    }
    fn collision(&self, kb: &Program, patient: &Arena, t: Id) -> Option<Id> {
        if let Node::Atom(name) = patient.node(t) {
            let name = patient.name(*name);
            if let Some(keys) = self.buckets.get(&hash(name.as_bytes()))
                && let Some(key) = keys
                    .iter()
                    .find(|key| kb.arena.atom_name(**key) == Some(name))
            {
                return Some(*key);
            }
        }
        patient
            .call(t)
            .and_then(|(_, args)| args.iter().find_map(|t| self.collision(kb, patient, *t)))
    }
    pub fn check_initial(&self, p: &Program) -> std::result::Result<(), String> {
        for (i, c) in p.clauses.iter().enumerate() {
            if !p.patient[i] {
                continue;
            }
            if !c.body.is_empty() {
                return Err(format!(
                    "patient {}:{} contains a rule",
                    c.document, c.ordinal
                ));
            }
            if let Some(key) = self.collision(p, &p.arena, c.head) {
                return Err(format!(
                    "patient {}:{} uses knowledge-base namespace {}",
                    c.document,
                    c.ordinal,
                    p.arena.render(key)
                ));
            }
        }
        Ok(())
    }
    pub fn check(&self, kb: &Program, patient: &[Original<'_>]) -> std::result::Result<(), String> {
        for source in patient {
            if source.document <= self.documents || source.ordinal == 0 {
                return Err("patient clause id overlaps knowledge base".into());
            }
            let mut arena = Arena::default();
            let clauses =
                parse_clauses(&mut arena, &format!("{}.\n", source.text), source.document)?;
            let [clause] = clauses.as_slice() else {
                return Err("patient source is not one clause".into());
            };
            if !clause.body.is_empty() {
                return Err(format!(
                    "patient {}:{} contains a rule",
                    source.document, source.ordinal
                ));
            }
            admit(&arena, &clauses)?;
            if let Some(key) = self.collision(kb, &arena, clause.head) {
                return Err(format!(
                    "patient {}:{} uses knowledge-base namespace {}",
                    source.document,
                    source.ordinal,
                    kb.arena.render(key)
                ));
            }
        }
        Ok(())
    }
}
