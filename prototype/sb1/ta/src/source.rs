use sb1_checker::program::{self, Clause, Goal, PREDICATES};
use sb1_checker::term::{Arena, Id};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Document {
    pub path: PathBuf,
    pub text: String,
    pub clauses: std::ops::Range<usize>,
    pub patient: bool,
}
pub struct Program {
    pub arena: Arena,
    pub clauses: Vec<Clause>,
    pub documents: Vec<Document>,
    pub sentences: Vec<Option<usize>>,
    pub changed: BTreeSet<usize>,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
impl Program {
    pub fn new() -> Self {
        Self {
            arena: Arena::default(),
            clauses: Vec::new(),
            documents: Vec::new(),
            sentences: Vec::new(),
            changed: BTreeSet::new(),
        }
    }
    fn patient_documents(composition: &Path, text: &str) -> Result<usize, String> {
        let mut count = None;
        let mut add = |value: &str| -> Result<(), String> {
            if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
                return Err("patient suffix count".into());
            }
            let n = value.parse::<usize>().map_err(|_| "patient suffix count")?;
            if count.is_some_and(|old| old != n) {
                return Err("conflicting patient suffix counts".into());
            }
            count = Some(n);
            Ok(())
        };
        for line in text.lines() {
            if let Some(value) = line.strip_prefix("# patient_documents\t") {
                add(value)?;
            }
        }
        let manifest = composition.with_file_name("manifest.tsv");
        match fs::read_to_string(&manifest) {
            Ok(text) => {
                for line in text.lines() {
                    if let Some(value) = line.strip_prefix("patient_documents\t") {
                        add(value)?;
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(format!("{}: {e}", manifest.display())),
        }
        Ok(count.unwrap_or(1))
    }
    pub fn load(root: &Path, composition: &Path, patient_last: bool) -> Result<Self, String> {
        let text = fs::read_to_string(composition).map_err(|e| e.to_string())?;
        let paths = text
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect::<Vec<_>>();
        if paths.is_empty() || !text.ends_with('\n') {
            return Err("empty or unterminated composition".into());
        }
        let patient_documents = if patient_last {
            Self::patient_documents(composition, &text)?
        } else {
            0
        };
        if patient_documents > paths.len() {
            return Err("patient suffix exceeds composition".into());
        }
        let mut out = Self::new();
        for (i, path) in paths.iter().enumerate() {
            let path = root.join(path);
            let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            let start = out.clauses.len();
            out.add(path, text, i >= paths.len() - patient_documents)?;
            for clause in &mut out.clauses[start..] {
                Self::release_clause_metadata(clause);
            }
        }
        program::admit(&out.arena, &out.clauses)?;
        out.validate_patient_boundary()?;
        Ok(out)
    }
    pub fn validate_patient_boundary(&self) -> Result<(), String> {
        fn namespaces(arena: &Arena, t: Id, out: &mut BTreeSet<Id>) {
            if let Some(a) = arena.named(t, "$guideline_id", 5) {
                out.insert(a[1]);
            }
            if let Some(a) = arena.named(t, "guideline_document", 4) {
                out.insert(a[0]);
            }
            if let Some((_, args)) = arena.call(t) {
                for t in args {
                    namespaces(arena, *t, out);
                }
            }
        }
        fn goals(arena: &Arena, body: &[Goal], out: &mut BTreeSet<Id>) {
            for g in body {
                match g {
                    Goal::Positive(t) | Goal::Builtin(t) => namespaces(arena, *t, out),
                    Goal::Not(inner) => goals(arena, inner, out),
                }
            }
        }
        fn collision(arena: &Arena, t: Id, kb: &BTreeSet<Id>) -> Option<Id> {
            if kb.contains(&t) {
                return Some(t);
            }
            arena
                .call(t)
                .and_then(|(_, a)| a.iter().find_map(|t| collision(arena, *t, kb)))
        }
        let mut kb = BTreeSet::new();
        for (i, c) in self.clauses.iter().enumerate() {
            if !self.patient(i) {
                namespaces(&self.arena, c.head, &mut kb);
                goals(&self.arena, &c.body, &mut kb);
            }
        }
        for (i, c) in self.clauses.iter().enumerate() {
            if self.patient(i) {
                if !c.body.is_empty() {
                    return Err(format!(
                        "patient document contains a rule: {}",
                        self.label(i)
                    ));
                }
                if let Some(namespace) = collision(&self.arena, c.head, &kb) {
                    return Err(format!(
                        "patient knowledge-base namespace collision at {}: {}",
                        self.label(i),
                        self.arena.render(namespace),
                    ));
                }
            }
        }
        Ok(())
    }
    pub fn add(&mut self, path: PathBuf, text: String, patient: bool) -> Result<(), String> {
        let start = self.clauses.len();
        let clauses = program::parse_clauses(&mut self.arena, &text, self.documents.len() + 1)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        if patient && clauses.iter().any(|c| !c.body.is_empty()) {
            return Err(format!(
                "patient document contains a rule: {}",
                path.display()
            ));
        }
        let mut sentence = None;
        let mut markers = BTreeMap::new();
        for (line, text) in text.lines().enumerate() {
            if let Some((number, _)) = text.strip_prefix("% S").and_then(|s| s.split_once(": ")) {
                sentence = Some(number.parse::<usize>().map_err(|_| "sentence marker")?);
            }
            markers.insert(line, sentence);
        }
        self.sentences
            .extend(clauses.iter().map(|c| markers[&c.line]));
        self.clauses.extend(clauses);
        self.documents.push(Document {
            path,
            text,
            clauses: start..self.clauses.len(),
            patient,
        });
        Ok(())
    }
    pub fn release_read_metadata(&mut self) {
        for c in &mut self.clauses {
            Self::release_clause_metadata(c);
        }
    }
    pub(crate) fn release_clause_metadata(c: &mut Clause) {
        // TA uses variable ids/counts; source bytes stay in Document.text.
        for name in &mut c.names {
            *name = String::new();
        }
        c.anonymous = Vec::new();
        c.nonlocals = Vec::new();
    }
    pub fn patient(&self, clause: usize) -> bool {
        self.documents[self.clauses[clause].document - 1].patient
    }
    pub fn label(&self, clause: usize) -> String {
        let c = &self.clauses[clause];
        format!(
            "{}:{}:{}",
            self.documents[c.document - 1].path.display(),
            c.line + 1,
            c.ordinal
        )
    }
    pub fn dc_shape(&mut self) -> Result<usize, String> {
        fn visit(goals: &[Goal], f: &mut impl FnMut(Id)) {
            for g in goals {
                match g {
                    Goal::Positive(t) | Goal::Builtin(t) => f(*t),
                    Goal::Not(inner) => visit(inner, f),
                }
            }
        }
        let mut deontic = BTreeSet::new();
        let mut scopes: BTreeMap<usize, BTreeSet<Id>> = BTreeMap::new();
        for (i, c) in self.clauses.iter().enumerate() {
            if self.patient(i) {
                continue;
            }
            let mut inspect = |t| {
                if self
                    .arena
                    .named(t, "guideline_operator", 3)
                    .is_some_and(|a| {
                        self.arena
                            .atom_name(a[2])
                            .is_some_and(|n| matches!(n, "should" | "must" | "may"))
                    })
                    && let Some(s) = self.sentences[i]
                {
                    deontic.insert((c.document, s));
                }
            };
            inspect(c.head);
            visit(&c.body, &mut inspect);
            visit(&c.body, &mut |t| {
                if self.arena.named(t, "guideline_exclusion", 1).is_some() {
                    scopes.entry(c.document).or_default().insert(t);
                }
            });
        }
        let mut added = 0;
        for (i, c) in self.clauses.iter_mut().enumerate() {
            if !self.sentences[i].is_some_and(|s| deontic.contains(&(c.document, s)))
                || [
                    "guideline_document",
                    "guideline_schema_version",
                    "guideline_supersession",
                ]
                .iter()
                .any(|name| {
                    self.arena
                        .call(c.head)
                        .is_some_and(|(n, _)| self.arena.name(n) == *name)
                })
            {
                continue;
            }
            let Some(keys) = scopes.get(&c.document) else {
                continue;
            };
            for &guard in keys {
                if !c.body.iter().any(|g| matches!(g, Goal::Not(inner) if matches!(inner.as_slice(), [Goal::Positive(t)] if *t == guard))) {
                    c.body.push(Goal::Not(vec![Goal::Positive(guard)]));
                    self.changed.insert(i);
                    added += 1;
                }
            }
        }
        program::admit(&self.arena, &self.clauses)?;
        Ok(added)
    }
    pub fn render_clause(&self, i: usize) -> String {
        fn goals(arena: &Arena, body: &[Goal]) -> String {
            body.iter()
                .map(|g| match g {
                    Goal::Positive(t) | Goal::Builtin(t) => arena.render(*t),
                    Goal::Not(inner) if inner.len() == 1 => format!("\\+ {}", goals(arena, inner)),
                    Goal::Not(inner) => format!("\\+ ({})", goals(arena, inner)),
                })
                .collect::<Vec<_>>()
                .join(", ")
        }
        let c = &self.clauses[i];
        let body = if c.body.is_empty() {
            String::new()
        } else {
            format!(" :- {}", goals(&self.arena, &c.body))
        };
        format!("{}{body}.", self.arena.render(c.head))
    }
    pub fn emit(
        &self,
        output: &Path,
        selected: Option<&BTreeSet<usize>>,
    ) -> Result<PathBuf, String> {
        fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut composition = String::new();
        let mut patient_documents = 0;
        let mut documents = 0;
        let mut clauses = 0;
        for (d, doc) in self.documents.iter().enumerate() {
            let chosen = doc
                .clauses
                .clone()
                .filter(|i| selected.is_none_or(|s| s.contains(i)))
                .collect::<Vec<_>>();
            if chosen.is_empty() {
                continue;
            }
            let statements = chosen
                .iter()
                .filter_map(|i| self.sentences[*i])
                .collect::<BTreeSet<_>>();
            let line_clauses = doc
                .clauses
                .clone()
                .map(|i| (self.clauses[i].line, i))
                .collect::<BTreeMap<_, _>>();
            let mut text = String::new();
            for (line, raw) in doc.text.lines().enumerate() {
                if let Some(&i) = line_clauses.get(&line) {
                    let meta = self.arena.call(self.clauses[i].head).is_some_and(|(n, _)| {
                        matches!(
                            self.arena.name(n),
                            "guideline_schema_version" | "guideline_document"
                        )
                    });
                    if !meta && selected.is_some_and(|s| !s.contains(&i)) {
                        continue;
                    }
                    clauses += 1;
                    text.push_str(if self.changed.contains(&i) { "" } else { raw });
                    if self.changed.contains(&i) {
                        text.push_str(&self.render_clause(i));
                    }
                } else if let Some((number, _)) =
                    raw.strip_prefix("% S").and_then(|s| s.split_once(": "))
                {
                    if !number
                        .parse::<usize>()
                        .ok()
                        .is_some_and(|n| statements.contains(&n))
                    {
                        continue;
                    }
                    text.push_str(raw);
                } else {
                    text.push_str(raw);
                }
                text.push('\n');
            }
            let path = output.join(format!("document-{:05}.pl", d + 1));
            fs::write(&path, text).map_err(|e| e.to_string())?;
            composition.push_str(&format!("{}\n", path.display()));
            documents += 1;
            patient_documents += usize::from(doc.patient);
        }
        let path = output.join("composition.tsv");
        let composition = format!("# patient_documents\t{patient_documents}\n{composition}");
        fs::write(&path, composition).map_err(|e| e.to_string())?;
        fs::write(
            output.join("manifest.tsv"),
            format!("format\tsb1-ta-composition 1\ndocuments\t{documents}\nclauses\t{clauses}\npatient_documents\t{patient_documents}\n"),
        ).map_err(|e| e.to_string())?;
        Ok(path)
    }
    pub fn metadata(&self, clause: usize) -> bool {
        self.arena
            .call(self.clauses[clause].head)
            .is_some_and(|(n, _)| {
                matches!(
                    self.arena.name(n),
                    "guideline_schema_version" | "guideline_document" | "guideline_supersession"
                )
            })
    }
    pub fn record_memory(&self, stage: &str) {
        if !crate::memory::enabled() {
            return;
        }
        use crate::memory::{bytes, record};
        use sb1_checker::term::Node;
        fn goals(body: &Vec<Goal>) -> usize {
            bytes::<Goal>(body.capacity())
                + body
                    .iter()
                    .map(|g| match g {
                        Goal::Not(inner) => goals(inner),
                        _ => 0,
                    })
                    .sum::<usize>()
        }
        let term_buffers = self
            .arena
            .nodes
            .iter()
            .map(|n| match n.as_ref() {
                Node::Integer(s) => s.capacity(),
                Node::Compound(_, args) => bytes::<Id>(args.capacity()),
                _ => 0,
            })
            .sum();
        let clause_buffers = self
            .clauses
            .iter()
            .map(|c| {
                goals(&c.body)
                    + bytes::<String>(c.names.capacity())
                    + c.names.iter().map(String::capacity).sum::<usize>()
                    + bytes::<(usize, usize)>(c.anonymous.capacity())
                    + bytes::<usize>(c.nonlocals.capacity())
            })
            .sum();
        record(
            stage,
            &[
                (
                    "source-documents",
                    self.documents.len(),
                    bytes::<Document>(self.documents.capacity()),
                    "buffer-capacity",
                ),
                (
                    "source-text",
                    self.documents.len(),
                    self.documents.iter().map(|d| d.text.capacity()).sum(),
                    "buffer-capacity",
                ),
                (
                    "source-clauses",
                    self.clauses.len(),
                    bytes::<Clause>(self.clauses.capacity()),
                    "buffer-capacity",
                ),
                (
                    "clause-buffers",
                    self.clauses.len(),
                    clause_buffers,
                    "buffer-capacity",
                ),
                (
                    "arena-node-pointers",
                    self.arena.nodes.len(),
                    bytes::<std::rc::Rc<Node>>(self.arena.nodes.capacity()),
                    "buffer-capacity",
                ),
                (
                    "arena-node-bodies",
                    self.arena.nodes.len(),
                    bytes::<Node>(self.arena.nodes.len()),
                    "payload-lower-bound",
                ),
                (
                    "arena-node-buffers",
                    self.arena.nodes.len(),
                    term_buffers,
                    "buffer-capacity",
                ),
                (
                    "arena-intern-entries",
                    self.arena.nodes.len(),
                    bytes::<(std::rc::Rc<Node>, Id)>(self.arena.nodes.len()),
                    "payload-lower-bound",
                ),
                (
                    "sentence-index",
                    self.sentences.len(),
                    bytes::<Option<usize>>(self.sentences.capacity()),
                    "buffer-capacity",
                ),
            ],
        );
    }
    pub fn declarations() -> String {
        PREDICATES
            .into_iter()
            .flat_map(|(name, arity)| {
                [
                    format!(":- multifile({name}/{arity}).\n"),
                    format!(":- discontiguous({name}/{arity}).\n"),
                ]
            })
            .collect()
    }
}
