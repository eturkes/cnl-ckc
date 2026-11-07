//! S1a failure-point diagnostics + load-slot canonical edge dumps.
use crate::domain::Abstraction;
use crate::features::{Feature, Label, Model};
use crate::graph::Graph;
use crate::growth::{Growth, Origin};
use crate::source::Program;
use sb1_checker::term::{Arena, Symbol};
use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(false);
pub fn enable() {
    ENABLED.store(true, Ordering::Relaxed);
    crate::census::enable();
}
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Role {
    Source,
    Target,
    Owner,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Outcome {
    Edge,
    HeadArg(usize, bool),
    Unary(Feature),
    Predicate(Symbol),
    Link(Label, Role),
    Witness,
}
pub(crate) fn feature_label(arena: &Arena, abs: &Abstraction, feature: Feature) -> String {
    match feature {
        Feature::Noun(v) => format!("noun({})", abs.describe(arena, v)),
        Feature::Lemma(v) => format!("lemma({})", abs.describe(arena, v)),
        Feature::Operator(v) => format!("operator({})", abs.describe(arena, v)),
        Feature::Property(v) => format!("property({})", abs.describe(arena, v)),
        Feature::Payload(p, v) => format!("payload({},{})", arena.name(p), abs.describe(arena, v)),
        Feature::Tag(v) => format!("tag({})", abs.describe(arena, v)),
    }
}
pub(crate) fn field(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
impl Outcome {
    fn label(self, arena: &Arena, abs: &Abstraction) -> (&'static str, String) {
        match self {
            Self::Edge => ("edge", String::new()),
            Self::HeadArg(at, variable) => (
                "head-arg-unify-fails",
                format!(
                    "arg{}:{}",
                    at + 1,
                    if variable { "variable" } else { "rigid" }
                ),
            ),
            Self::Unary(feature) => (
                "unary-constraint-empties",
                feature_label(arena, abs, feature),
            ),
            Self::Predicate(p) => (
                "unary-constraint-empties",
                format!("predicate-absent:{}", arena.name(p)),
            ),
            Self::Link(label, role) => (
                "link-propagation-empties",
                format!(
                    "{}:{}:{role:?}",
                    arena.name(label.0),
                    abs.describe(arena, label.1)
                ),
            ),
            Self::Witness => ("witness-search-fails", String::new()),
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ForcedWork {
    pub state_clones: usize,
    pub domain_clones: usize,
    pub meet_operands: usize,
    pub meet_yields: usize,
    pub meet_wildcards: usize,
    pub owner_rows: usize,
}
impl ForcedWork {
    pub(crate) fn visit(&mut self, key: &str, count: usize) {
        match key {
            "state_clone_member_visits" => self.state_clones += count,
            "domain_clone_member_visits" => self.domain_clones += count,
            "meet_operand_memberships" => self.meet_operands += count,
            "meet_intersection_yields" => self.meet_yields += count,
            "meet_wildcard_member_visits" => self.meet_wildcards += count,
            "owner_row_visits" => self.owner_rows += count,
            _ => (),
        }
    }
    fn add(&mut self, other: Self) {
        self.state_clones += other.state_clones;
        self.domain_clones += other.domain_clones;
        self.meet_operands += other.meet_operands;
        self.meet_yields += other.meet_yields;
        self.meet_wildcards += other.meet_wildcards;
        self.owner_rows += other.owner_rows;
    }
}
#[derive(Clone, Debug)]
struct Document {
    path: PathBuf,
    group: usize,
    kind: &'static str,
    patient: bool,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Class {
    phase: &'static str,
    outcome: Outcome,
    same_group: bool,
    same_document: bool,
    classified: bool,
}
#[derive(Default)]
struct Total {
    checks: usize,
    work: ForcedWork,
}
pub(crate) struct Details {
    documents: Vec<Document>,
    groups: Vec<String>,
    caller: Option<usize>,
    active: Option<(usize, usize, &'static str)>,
    failure: Option<Outcome>,
    totals: BTreeMap<Class, Total>,
}
impl Details {
    pub(crate) fn new(program: &Program) -> Self {
        let mut groups = Vec::new();
        let mut ids = HashMap::new();
        let documents = program
            .documents
            .iter()
            .map(|doc| {
                let copy = doc.path.ancestors().find(|p| {
                    p.parent()
                        .and_then(Path::file_name)
                        .is_some_and(|n| n == "copies")
                });
                let (key, kind) = if doc.patient {
                    (format!("patient:{}", doc.path.display()), "patient")
                } else if let Some(copy) = copy {
                    (copy.display().to_string(), "copy")
                } else if doc.path == Path::new("k1-registry") {
                    ("k1-registry".into(), "registry")
                } else {
                    (doc.path.display().to_string(), "document")
                };
                let next = ids.len();
                let group = *ids.entry(key.clone()).or_insert_with(|| {
                    groups.push(key);
                    next
                });
                Document {
                    path: doc.path.clone(),
                    group,
                    kind,
                    patient: doc.patient,
                }
            })
            .collect();
        Self {
            documents,
            groups,
            caller: None,
            active: None,
            failure: None,
            totals: BTreeMap::new(),
        }
    }
    pub(crate) fn caller(&mut self, program: &Program, clause: usize) {
        self.caller = Some(program.clauses[clause].document - 1);
    }
    pub(crate) fn begin(&mut self, program: &Program, producer: usize, phase: &'static str) {
        assert!(self.active.is_none());
        self.active = Some((
            self.caller.unwrap_or(usize::MAX),
            program.clauses[producer].document - 1,
            phase,
        ));
        self.failure = None;
        crate::work::forced_begin();
    }
    pub(crate) fn fail(&mut self, outcome: Outcome) {
        if self.active.is_some() {
            self.failure = Some(outcome);
        }
    }
    pub(crate) fn last_failure(&self) -> Outcome {
        self.failure
            .expect("every initial forced solve failure is classified")
    }
    pub(crate) fn finish(&mut self, outcome: Outcome) {
        let (caller, producer, phase) = self.active.take().expect("forced diagnostic begin");
        let classified = caller < self.documents.len();
        let same_group =
            classified && self.documents[caller].group == self.documents[producer].group;
        let class = Class {
            phase,
            outcome,
            same_group,
            same_document: caller == producer,
            classified,
        };
        let total = self.totals.entry(class).or_default();
        total.checks += 1;
        total.work.add(crate::work::forced_end());
    }
    pub(crate) fn write(
        &self,
        out: &Path,
        name: &str,
        arena: &Arena,
        abs: &Abstraction,
    ) -> Result<(), String> {
        assert!(self.active.is_none());
        let file = std::fs::File::create(out.join(format!("failures-{name}.tsv")))
            .map_err(|e| e.to_string())?;
        let mut file = std::io::BufWriter::new(file);
        writeln!(file, "phase\toutcome\tdetail\tguideline_relation\tdocument_relation\tchecks\tstate_clone_memberships\tdomain_clone_memberships\tmeet_operand_memberships\tmeet_intersection_yields\tmeet_wildcard_visits\towner_row_visits").map_err(|e| e.to_string())?;
        for (class, total) in &self.totals {
            let (kind, detail) = class.outcome.label(arena, abs);
            writeln!(
                file,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                class.phase,
                kind,
                field(&detail),
                if !class.classified {
                    "unclassified"
                } else if class.same_group {
                    "same"
                } else {
                    "cross"
                },
                if class.same_document { "same" } else { "cross" },
                total.checks,
                total.work.state_clones,
                total.work.domain_clones,
                total.work.meet_operands,
                total.work.meet_yields,
                total.work.meet_wildcards,
                total.work.owner_rows
            )
            .map_err(|e| e.to_string())?;
        }
        file.flush().map_err(|e| e.to_string())?;
        let mut docs = std::io::BufWriter::new(
            std::fs::File::create(out.join(format!("groups-{name}.tsv")))
                .map_err(|e| e.to_string())?,
        );
        writeln!(docs, "document\tgroup\tkind\tpatient\tpath\tgroup_path")
            .map_err(|e| e.to_string())?;
        for (i, doc) in self.documents.iter().enumerate() {
            writeln!(
                docs,
                "{}\t{}\t{}\t{}\t{}\t{}",
                i + 1,
                doc.group + 1,
                doc.kind,
                doc.patient,
                field(&doc.path.display().to_string()),
                field(&self.groups[doc.group])
            )
            .map_err(|e| e.to_string())?;
        }
        docs.flush().map_err(|e| e.to_string())
    }
}

pub fn write_edges(
    out: &Path,
    program: &Program,
    model: &Model,
    graph: &Graph,
    growth: &Growth,
) -> Result<(), String> {
    if !enabled() {
        return Ok(());
    }
    let mut clauses = std::io::BufWriter::new(
        std::fs::File::create(out.join("canonical-clause-edges.tsv")).map_err(|e| e.to_string())?,
    );
    writeln!(
        clauses,
        "from_document\tfrom_clause\tsign\tgrowth\tto_document\tto_clause"
    )
    .map_err(|e| e.to_string())?;
    for (from, row) in graph.edges.iter().enumerate() {
        let source = &program.clauses[from];
        for edge in row {
            let target = &program.clauses[edge.to];
            writeln!(
                clauses,
                "{}\t{}\t{}\t{}\t{}\t{}",
                source.document,
                source.ordinal,
                if edge.negative { "-" } else { "+" },
                edge.growth,
                target.document,
                target.ordinal
            )
            .map_err(|e| e.to_string())?;
        }
    }
    clauses.flush().map_err(|e| e.to_string())?;
    let mut docids = HashMap::new();
    for clause in &program.clauses {
        if let Some(args) = program.arena.named(clause.head, "guideline_document", 4) {
            docids.insert(args[0], clause.document);
        }
    }
    let labels = growth
        .nodes
        .iter()
        .map(|origin| match origin {
            Origin::Skolem(o) => {
                let a = program
                    .arena
                    .named(model.abs.origins[*o], "$guideline_id", 5)
                    .unwrap();
                let document = docids.get(&a[1]).map_or_else(
                    || format!("unmapped({})", program.arena.render(a[1])),
                    |id| id.to_string(),
                );
                format!(
                    "origin({},{},{},{})",
                    program.arena.render(a[0]),
                    document,
                    program.arena.render(a[2]),
                    program.arena.render(a[3])
                )
            }
            Origin::Arithmetic { clause, goal } => {
                let c = &program.clauses[*clause];
                format!("arithmetic({},{},{})", c.document, c.ordinal, goal + 1)
            }
        })
        .collect::<Vec<_>>();
    let mut edges = growth
        .graph
        .edges
        .iter()
        .enumerate()
        .flat_map(|(from, row)| row.iter().map(move |edge| (from, *edge)))
        .map(|(from, edge)| {
            (
                labels[from].as_str(),
                labels[edge.to].as_str(),
                edge.negative,
                edge.growth,
            )
        })
        .collect::<Vec<_>>();
    edges.sort_unstable();
    let mut flow = std::io::BufWriter::new(
        std::fs::File::create(out.join("canonical-base-flow-edges.tsv"))
            .map_err(|e| e.to_string())?,
    );
    writeln!(flow, "from\tto\tsign\tgrowth").map_err(|e| e.to_string())?;
    for (from, to, negative, growth) in edges {
        writeln!(
            flow,
            "{}\t{}\t{}\t{}",
            field(from),
            field(to),
            if negative { "-" } else { "+" },
            growth
        )
        .map_err(|e| e.to_string())?;
    }
    flow.flush().map_err(|e| e.to_string())?;
    let mut nodes = std::io::BufWriter::new(
        std::fs::File::create(out.join("canonical-clause-nodes.tsv")).map_err(|e| e.to_string())?,
    );
    writeln!(nodes, "document\tclause\tpatient").map_err(|e| e.to_string())?;
    for (i, c) in program.clauses.iter().enumerate() {
        writeln!(
            nodes,
            "{}\t{}\t{}",
            c.document,
            c.ordinal,
            program.patient(i)
        )
        .map_err(|e| e.to_string())?;
    }
    nodes.flush().map_err(|e| e.to_string())
}
