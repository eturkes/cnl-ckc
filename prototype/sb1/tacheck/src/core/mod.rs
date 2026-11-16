//! Independent primitive-id checking core. Shared surface = sb1-checker's IR.
pub mod analysis;
pub mod extension;
mod graph;
pub mod growth;
mod matching;
mod model;
mod namespaces;
pub mod refinement;
mod sets;
pub use graph::{Csr, build as signed_graph, enumerate as enumerate_signed};
pub use matching::check_envelope;
pub use matching::{Budget, Pattern, State};
pub use model::{Abs, May, RawValue};
use sb1_checker::program::Clause;
use sb1_checker::term::Arena;
pub use sets::{ALL, EMPTY, SetId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Failure {
    Admission(String),
    Certificate(String),
    Cycle {
        kind: &'static str,
        edges: Vec<(usize, usize, bool)>,
        sites: Vec<(usize, usize)>,
        labels: Vec<(String, String)>,
    },
    Indeterminate(String),
}
pub type Result<T> = std::result::Result<T, Failure>;
pub fn bad(message: impl Into<String>) -> Failure {
    Failure::Certificate(message.into())
}

pub struct Program {
    pub arena: Arena,
    pub clauses: Vec<Clause>,
    pub patient: Vec<bool>,
    pub documents: Vec<String>,
}
impl Program {
    pub fn label(&self, clause: usize) -> String {
        let c = &self.clauses[clause];
        format!("{}:{}", self.documents[c.document - 1], c.ordinal)
    }
    pub fn detail_label(&self, clause: usize) -> String {
        let c = &self.clauses[clause];
        format!(
            "{}:{}:{}",
            self.documents[c.document - 1],
            c.line + 1,
            c.ordinal
        )
    }
    pub fn metadata(&self, clause: usize) -> bool {
        self.arena
            .call(self.clauses[clause].head)
            .is_some_and(|(p, _)| {
                matches!(
                    self.arena.name(p),
                    "guideline_schema_version" | "guideline_document" | "guideline_supersession"
                )
            })
    }
}

pub(crate) fn hash(words: &[u32]) -> u64 {
    words.iter().fold(0xcbf2_9ce4_8422_2325, |h, x| {
        (h ^ u64::from(*x)).wrapping_mul(0x100_0000_01b3)
    })
}
pub(crate) fn id(n: usize) -> Result<u32> {
    u32::try_from(n).map_err(|_| Failure::Indeterminate("primitive id space exhausted".into()))
}
