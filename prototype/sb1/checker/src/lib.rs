mod digest;
mod engine;
mod number;
mod program;
mod query;
mod term;

use digest::sha256;
use engine::{Claim, Model};
use program::{Clause, parse_clauses};
use query::Query;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use term::{Arena, Env, parse_term};

fn canonical(bytes: &[u8]) -> bool {
    matches!(
        ckc_kernel::contract::v1_check(bytes),
        ckc_spec::v1text::EV1Verdict::Ok
    )
}

#[derive(Debug)]
pub struct Failure {
    pub rc: i32,
    pub message: String,
}
impl Failure {
    fn refused(stage: &str, detail: impl Into<String>) -> Self {
        Self {
            rc: 1,
            message: format!("refused {stage} {}", one_line(&detail.into())),
        }
    }
    fn parse(detail: impl Into<String>) -> Self {
        Self {
            rc: 2,
            message: format!("parse {}", one_line(&detail.into())),
        }
    }
    fn io(path: &Path, err: std::io::Error) -> Self {
        Self {
            rc: 2,
            message: format!(
                "I/O {} {}",
                one_line(&path.display().to_string()),
                one_line(&err.to_string())
            ),
        }
    }
}
fn one_line(text: &str) -> String {
    text.replace(['\r', '\n', '\t'], " ")
}

pub struct Counters {
    logical: BTreeMap<&'static str, u64>,
    runtime: BTreeMap<&'static str, u64>,
    level_max: String,
}
impl Default for Counters {
    fn default() -> Self {
        let logical = [
            "documents",
            "clauses",
            "source_bytes",
            "clause_term_bytes",
            "patient_facts",
            "model_bytes",
            "model_facts",
            "claims",
            "identity_bytes",
            "certificate_bytes",
            "predicate_indices",
            "bound_pattern_indices",
            "support_incidences",
            "support_builtins",
            "support_negations",
            "enabled_instances",
            "answer_rows",
            "output_bytes",
            "interned_terms",
            "term_depth_max",
            "number_decimal_digits_max",
            "source_header_adapters",
        ]
        .into_iter()
        .map(|name| (name, 0))
        .collect();
        let runtime = [
            "index_insertions",
            "index_builds",
            "index_probes",
            "candidate_joins",
            "term_matches",
            "successful_joins",
            "failed_joins",
            "builtin_evaluations",
            "naf_checks",
            "naf_scans",
            "comparisons",
            "interning_requests",
        ]
        .into_iter()
        .map(|name| (name, 0))
        .collect();
        Self {
            logical,
            runtime,
            level_max: String::new(),
        }
    }
}
impl Counters {
    fn logical(&mut self, name: &'static str, n: u64) {
        *self.logical.entry(name).or_default() += n;
    }
    fn runtime(&mut self, name: &'static str, n: u64) {
        *self.runtime.entry(name).or_default() += n;
    }
    pub fn render(&self) -> String {
        let mut out = String::from("kind\tcounter\tvalue\n");
        for (&name, n) in &self.logical {
            out.push_str(&format!("logical\t{name}\t{n}\n"));
        }
        out.push_str(&format!(
            "logical\tsupport_level_max\t{}\n",
            if self.level_max.is_empty() {
                "0"
            } else {
                &self.level_max
            }
        ));
        for (&name, n) in &self.runtime {
            out.push_str(&format!("runtime\t{name}\t{n}\n"));
        }
        out
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub root: PathBuf,
    pub composition: PathBuf,
    pub model: PathBuf,
    pub certificate: PathBuf,
    pub query: Option<PathBuf>,
}
#[derive(Clone, Debug)]
pub struct Phase {
    pub name: &'static str,
    pub seconds: f64,
    pub cpu: f64,
    pub executed: bool,
}

pub struct Outcome {
    pub rc: i32,
    pub stdout: String,
    pub stderr: String,
    pub phases: Vec<Phase>,
    pub counters: Counters,
}
impl Outcome {
    pub fn phase_tsv(&self) -> String {
        let mut out = String::from("phase\tseconds\tcpu\n");
        for p in &self.phases {
            out.push_str(&format!("{}\t{:.9}\t{:.9}\n", p.name, p.seconds, p.cpu));
        }
        out
    }
    pub fn write_metrics(&self, directory: &Path) -> Result<(), Failure> {
        for (name, bytes) in [
            ("checker-phases.tsv", self.phase_tsv()),
            ("checker-counters.tsv", self.counters.render()),
        ] {
            let path = directory.join(name);
            fs::write(&path, bytes).map_err(|e| Failure::io(&path, e))?;
        }
        Ok(())
    }
}

struct Clock {
    wall: Instant,
    cpu: u64,
}
impl Clock {
    fn start() -> Result<Self, Failure> {
        Ok(Self {
            wall: Instant::now(),
            cpu: cpu_ns()?,
        })
    }
    fn stop(self, name: &'static str) -> Result<Phase, Failure> {
        Ok(Phase {
            name,
            seconds: self.wall.elapsed().as_secs_f64(),
            cpu: cpu_ns()?.saturating_sub(self.cpu) as f64 / 1e9,
            executed: true,
        })
    }
}
fn cpu_ns() -> Result<u64, Failure> {
    let path = Path::new("/proc/self/schedstat");
    let text = fs::read_to_string(path).map_err(|e| Failure::io(path, e))?;
    text.split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| Failure::parse("CPU clock /proc/self/schedstat"))
}
fn read(path: &Path) -> Result<Vec<u8>, Failure> {
    fs::read(path).map_err(|e| Failure::io(path, e))
}
fn utf8<'a>(bytes: &'a [u8], kind: &str) -> Result<&'a str, Failure> {
    std::str::from_utf8(bytes).map_err(|e| Failure::parse(format!("{kind} UTF-8 {e}")))
}

struct Document {
    path: String,
    bytes: Vec<u8>,
}
struct Loaded {
    arena: Arena,
    clauses: Vec<Clause>,
    documents: Vec<Document>,
    schema: Vec<u8>,
}
fn load(request: &Request, counters: &mut Counters) -> Result<Loaded, Failure> {
    let composition = read(&request.composition)?;
    let text = utf8(&composition, "composition")?;
    if !text.ends_with('\n') {
        return Err(Failure::parse("composition missing LF"));
    }
    let schema = read(&request.root.join("guidelines/schema.pl"))?;
    let mut arena = Arena::default();
    let mut clauses = Vec::new();
    let mut documents = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if line.contains(['\t', '\r', '\0']) {
            return Err(Failure::parse("composition path bytes"));
        }
        let path = request.root.join(line);
        let bytes = read(&path)?;
        utf8(&bytes, "source")?;
        let read = ckc_kernel::contract::v1_read(&bytes);
        let mut next = parse_clauses(&mut arena, utf8(&read, "read source")?, documents.len() + 1)
            .map_err(|e| Failure::parse(format!("{line} {e}")))?;
        counters.logical("source_bytes", bytes.len() as u64);
        counters.logical(
            "clause_term_bytes",
            utf8(&read, "read source")?
                .lines()
                .filter(|line| {
                    !line.is_empty() && !line.starts_with('%') && !line.starts_with(":-")
                })
                .map(|line| (line.len() + 1) as u64)
                .sum(),
        );
        clauses.append(&mut next);
        documents.push(Document {
            path: line.into(),
            bytes,
        });
    }
    if documents.is_empty() {
        return Err(Failure::parse("empty composition"));
    }
    counters.logical("documents", documents.len() as u64);
    counters.logical("clauses", clauses.len() as u64);
    counters.logical(
        "patient_facts",
        clauses
            .iter()
            .filter(|c| c.document == documents.len() && c.body.is_empty())
            .count() as u64,
    );
    Ok(Loaded {
        arena,
        clauses,
        documents,
        schema,
    })
}
fn identity(loaded: &Loaded) -> String {
    let mut frame = format!(
        "schema\0{}\0{}\n",
        loaded.schema.len(),
        sha256(&loaded.schema)
    );
    for doc in &loaded.documents {
        frame.push_str(&format!(
            "file\0{}\0{}\0{}\n",
            doc.path,
            doc.bytes.len(),
            sha256(&doc.bytes)
        ));
    }
    sha256(frame.as_bytes())
}

fn valuation(arena: &mut Arena, text: &str, clause: &Clause) -> Result<Env, String> {
    let term = parse_term(arena, text)?;
    if arena.render(term) != text {
        return Err("noncanonical valuation".into());
    }
    let values = arena.list(term).ok_or("valuation list")?;
    if values.len() != clause.nonlocals.len() {
        return Err("valuation variable count".into());
    }
    let mut env = vec![None; clause.names.len()];
    for (i, (&var, value)) in clause.nonlocals.iter().zip(values).enumerate() {
        let pair = arena.named(value, "=", 2).ok_or("valuation pair")?;
        if arena.atom_name(pair[0]) != Some(&format!("v{}", i + 1)) {
            return Err("valuation variable order".into());
        }
        if !program::ground_term(arena, pair[1]) {
            return Err("valuation term domain".into());
        }
        env[var] = Some(pair[1]);
    }
    Ok(env)
}

fn decode(
    request: &Request,
    loaded: &mut Loaded,
    counters: &mut Counters,
) -> Result<(Model, Vec<Claim>, Query), Failure> {
    let bytes = read(&request.certificate)?;
    counters.logical("certificate_bytes", bytes.len() as u64);
    let certificate = std::str::from_utf8(&bytes)
        .map_err(|e| Failure::refused("envelope", format!("UTF-8 {e}")))?;
    if !certificate.ends_with('\n') || certificate.contains('\r') {
        return Err(Failure::refused(
            "envelope",
            "truncated or non-LF certificate",
        ));
    }
    let mut lines = certificate.lines();
    if lines.next() != Some("sb1-certificate 0") {
        return Err(Failure::refused("envelope", "certificate version"));
    }
    let Some(got) = lines.next().and_then(|l| l.strip_prefix("composition ")) else {
        return Err(Failure::refused("envelope", "composition record"));
    };
    if got.len() != 64
        || !got
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(Failure::refused("envelope", "composition digest shape"));
    }
    counters.logical(
        "identity_bytes",
        loaded.schema.len() as u64
            + loaded
                .documents
                .iter()
                .map(|d| d.bytes.len() as u64)
                .sum::<u64>(),
    );
    if got != identity(loaded) {
        return Err(Failure::refused("identity", "composition digest mismatch"));
    }
    let bytes = read(&request.model)?;
    counters.logical("model_bytes", bytes.len() as u64);
    let model_text = utf8(&bytes, "model")?;
    if !bytes.is_empty() && !model_text.ends_with('\n') {
        return Err(Failure::parse("model missing LF"));
    }
    let mut facts = Vec::new();
    let mut positions = HashMap::new();
    for (i, line) in model_text.lines().enumerate() {
        let fact = parse_term(&mut loaded.arena, line)
            .map_err(|e| Failure::parse(format!("model line {} {e}", i + 1)))?;
        if !program::model_fact(&loaded.arena, fact) {
            return Err(Failure::refused(
                "claim",
                format!("model line {} term domain", i + 1),
            ));
        }
        if format!("{}.", loaded.arena.render(fact)) != line {
            return Err(Failure::parse(format!("noncanonical model line {}", i + 1)));
        }
        if positions.insert(fact, facts.len()).is_some() {
            return Err(Failure::refused(
                "claim",
                format!("duplicate model fact line {}", i + 1),
            ));
        }
        facts.push(fact);
    }
    let clause_ids = loaded
        .clauses
        .iter()
        .enumerate()
        .map(|(i, c)| ((c.document, c.ordinal), i))
        .collect::<HashMap<_, _>>();
    let mut claims = Vec::new();
    let mut levels: Vec<Option<String>> = vec![None; facts.len()];
    for (i, line) in lines.enumerate() {
        let row = i + 3;
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 4 {
            return Err(Failure::refused("envelope", format!("record {row} fields")));
        }
        let fact = parse_term(&mut loaded.arena, fields[0])
            .map_err(|e| Failure::refused("envelope", format!("record {row} {e}")))?;
        if loaded.arena.render(fact) != fields[0] {
            return Err(Failure::refused(
                "envelope",
                format!("record {row} noncanonical fact"),
            ));
        }
        let Some(&fact) = positions.get(&fact) else {
            return Err(Failure::refused(
                "claim",
                format!("record {row} fact not in model"),
            ));
        };
        if levels[fact].is_some() {
            return Err(Failure::refused(
                "claim",
                format!("record {row} duplicate claim"),
            ));
        }
        let clause = fields[1]
            .split_once(':')
            .and_then(|(doc, ord)| Some((doc.parse::<usize>().ok()?, ord.parse::<usize>().ok()?)))
            .and_then(|id| clause_ids.get(&id).copied())
            .ok_or_else(|| Failure::refused("claim", format!("record {row} clause id")))?;
        let env = valuation(&mut loaded.arena, fields[2], &loaded.clauses[clause])
            .map_err(|e| Failure::refused("claim", format!("record {row} {e}")))?;
        if fields[3].is_empty() || !fields[3].bytes().all(|b| b.is_ascii_digit()) {
            return Err(Failure::refused("envelope", format!("record {row} level")));
        }
        let level = fields[3].trim_start_matches('0');
        let level = if level.is_empty() { "0" } else { level }.to_string();
        if term::natural_compare(&level, &counters.level_max) == std::cmp::Ordering::Greater {
            counters.level_max = level.clone();
        }
        levels[fact] = Some(level.clone());
        claims.push(Claim {
            clause,
            fact,
            env,
            level,
        });
    }
    let levels = levels
        .into_iter()
        .enumerate()
        .map(|(i, l)| {
            l.ok_or_else(|| {
                Failure::refused("claim", format!("missing claim for model fact {}", i + 1))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    counters.logical("model_facts", facts.len() as u64);
    counters.logical("claims", claims.len() as u64);
    let query_bytes = request.query.as_deref().map(read).transpose()?;
    let query = Query::parse(&mut loaded.arena, query_bytes.as_deref())?;
    Ok((Model::new(&loaded.arena, facts, levels)?, claims, query))
}

fn arena_counters(arena: &Arena, model: &Model, clauses: &[Clause], counters: &mut Counters) {
    counters.logical("interned_terms", arena.nodes.len() as u64);
    counters.runtime("interning_requests", arena.lookups);
    let mut depths: Vec<u64> = Vec::with_capacity(arena.nodes.len());
    let mut max_width = 0usize;
    for node in &arena.nodes {
        depths.push(match node.as_ref() {
            term::Node::Compound(_, args) => {
                1 + args.iter().map(|id| depths[*id]).max().unwrap_or(0)
            }
            _ => 0,
        });
        if let term::Node::Integer(n) = node.as_ref() {
            max_width = max_width.max(n.trim_start_matches('-').len());
        }
    }
    fn goal_depth(goals: &[program::Goal], depths: &[u64]) -> u64 {
        goals
            .iter()
            .map(|goal| match goal {
                program::Goal::Positive(term) | program::Goal::Builtin(term) => {
                    depths[*term].saturating_sub(1)
                }
                program::Goal::Not(inner) => goal_depth(inner, depths),
            })
            .max()
            .unwrap_or(0)
    }
    let term_depth = model
        .facts
        .iter()
        .map(|id| depths[*id].saturating_sub(1))
        .chain(clauses.iter().map(|c| {
            depths[c.head]
                .saturating_sub(1)
                .max(goal_depth(&c.body, &depths))
        }))
        .max()
        .unwrap_or(0);
    counters.logical("term_depth_max", term_depth);
    counters.logical("number_decimal_digits_max", max_width as u64);
}

pub fn check(request: &Request) -> Outcome {
    let mut phases = [
        "load",
        "decode",
        "admission",
        "index",
        "support",
        "closure",
        "answers",
    ]
    .map(|name| Phase {
        name,
        seconds: 0.0,
        cpu: 0.0,
        executed: false,
    })
    .to_vec();
    let mut counters = Counters::default();
    let mut run_phase =
        |name: &'static str, f: &mut dyn FnMut(&mut Counters) -> Result<(), Failure>| {
            let clock = Clock::start()?;
            let result = f(&mut counters);
            *phases.iter_mut().find(|p| p.name == name).unwrap() = clock.stop(name)?;
            result
        };
    let mut loaded = None;
    let result: Result<String, Failure> = (|| {
        run_phase("load", &mut |c| {
            loaded = Some(load(request, c)?);
            Ok(())
        })?;
        let loaded = loaded.as_mut().unwrap();
        let mut decoded = None;
        run_phase("decode", &mut |c| {
            decoded = Some(decode(request, loaded, c)?);
            Ok(())
        })?;
        let (mut model, claims, query) = decoded.unwrap();
        run_phase("admission", &mut |counters| {
            for (i, doc) in loaded.documents.iter().enumerate() {
                if canonical(&doc.bytes) {
                    continue;
                }
                let docid = loaded
                    .clauses
                    .iter()
                    .filter(|c| c.document == i + 1)
                    .find_map(|c| loaded.arena.named(c.head, "guideline_document", 4))
                    .and_then(|args| loaded.arena.atom_name(args[0]));
                let text = utf8(&doc.bytes, "source")?;
                let code_at = text
                    .lines()
                    .take_while(|line| line.starts_with('%'))
                    .map(|line| line.len() + 1)
                    .sum::<usize>();
                let adapted = docid.map(|docid| format!("% {docid}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}",&text[code_at..]));
                if !adapted.is_some_and(|bytes| canonical(bytes.as_bytes())) {
                    return Err(Failure::parse(format!("noncanonical source {}", doc.path)));
                }
                counters.logical("source_header_adapters", 1);
            }
            program::admit(&loaded.arena, &loaded.clauses)
                .map_err(|e| Failure::refused("claim", format!("admission {e}")))
        })?;
        run_phase("index", &mut |c| {
            model.build_indices(&loaded.arena, &loaded.clauses, query.goals(), c);
            Ok(())
        })?;
        run_phase("support", &mut |c| {
            engine::support(&mut loaded.arena, &model, &loaded.clauses, &claims, c)
        })?;
        run_phase("closure", &mut |c| {
            engine::closure(&mut loaded.arena, &model, &loaded.clauses, c)
        })?;
        let mut output = String::new();
        run_phase("answers", &mut |c| {
            output = query.answers(&mut loaded.arena, &model, c)?;
            c.logical("output_bytes", output.len() as u64);
            arena_counters(&loaded.arena, &model, &loaded.clauses, c);
            Ok(())
        })?;
        Ok(output)
    })();
    match result {
        Ok(stdout) => Outcome {
            rc: 0,
            stdout,
            stderr: String::new(),
            phases,
            counters,
        },
        Err(failure) => Outcome {
            rc: failure.rc,
            stdout: String::new(),
            stderr: format!("{}\n", failure.message),
            phases,
            counters,
        },
    }
}

#[cfg(test)]
mod schema_tests;
#[cfg(test)]
mod tests;
