mod codec;
mod digest;
mod engine;
mod level;
mod number;
pub mod orchestrator;
pub mod program;
pub mod query;
mod slice;
pub mod term;

use digest::sha256;
use engine::{Claim, Model};
use level::Level;
use program::{Clause, parse_clauses};
use query::Query;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use term::{Arena, Env, parse_checked};

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
    level_max: Level,
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
            "selected_clauses",
            "typed_analysis_hook",
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
            level_max: Level::default(),
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
        out.push_str(&format!("logical\tsupport_level_max\t{}\n", self.level_max));
        for (&name, n) in &self.runtime {
            out.push_str(&format!("runtime\t{name}\t{n}\n"));
        }
        out
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SourceCodec {
    Kernel,
    #[default]
    Parsed,
}

#[derive(Clone, Debug, Default)]
pub struct SourceProfile {
    normalization: f64,
    decoder: f64,
    canonical: f64,
    shape_safety: f64,
}
impl SourceProfile {
    pub fn render(&self) -> String {
        format!(
            "step\tseconds\nnormalization\t{:.9}\ndecoder\t{:.9}\ncanonical\t{:.9}\nshape_safety\t{:.9}\n",
            self.normalization, self.decoder, self.canonical, self.shape_safety
        )
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
    pub source_profile: SourceProfile,
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
            ("checker-source-profile.tsv", self.source_profile.render()),
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
    clauses: std::ops::Range<usize>,
}
struct Loaded {
    arena: Arena,
    clauses: Vec<Clause>,
    documents: Vec<Document>,
    schema: Vec<u8>,
    source_profile: SourceProfile,
}
#[cfg(test)]
fn load(request: &Request, counters: &mut Counters) -> Result<Loaded, Failure> {
    load_with_codec(request, counters, SourceCodec::Parsed)
}
fn load_with_codec(
    request: &Request,
    counters: &mut Counters,
    mode: SourceCodec,
) -> Result<Loaded, Failure> {
    let composition = read(&request.composition)?;
    let text = utf8(&composition, "composition")?;
    if !text.ends_with('\n') {
        return Err(Failure::parse("composition missing LF"));
    }
    let mut loaded = Loaded {
        arena: Arena::default(),
        clauses: Vec::new(),
        documents: Vec::new(),
        schema: read(&request.root.join("guidelines/schema.pl"))?,
        source_profile: SourceProfile::default(),
    };
    for line in composition_paths(text)? {
        append_source(&mut loaded, &request.root, &line, counters, mode)?;
    }
    let documents = &loaded.documents;
    let clauses = &loaded.clauses;
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
    Ok(loaded)
}
fn composition_paths(text: &str) -> Result<Vec<String>, Failure> {
    if !text.ends_with('\n') {
        return Err(Failure::parse("composition missing LF"));
    }
    text.lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .map(|line| {
            if line.contains(['\t', '\r', '\0']) {
                Err(Failure::parse("composition path bytes"))
            } else {
                Ok(line.into())
            }
        })
        .collect()
}
fn append_source(
    loaded: &mut Loaded,
    root: &Path,
    line: &str,
    counters: &mut Counters,
    mode: SourceCodec,
) -> Result<(), Failure> {
    let path = root.join(line);
    let bytes = read(&path)?;
    let raw_text = utf8(&bytes, "source")?;
    let clock = Instant::now();
    let normalized = if mode == SourceCodec::Kernel {
        Some(ckc_kernel::contract::v1_read(&bytes))
    } else {
        None
    };
    loaded.source_profile.normalization += clock.elapsed().as_secs_f64();
    let source = normalized
        .as_deref()
        .map(|b| utf8(b, "read source"))
        .transpose()?
        .unwrap_or(raw_text);
    let clock = Instant::now();
    let mut next = parse_clauses(&mut loaded.arena, source, loaded.documents.len() + 1)
        .map_err(|e| Failure::parse(format!("{line} {e}")))?;
    loaded.source_profile.decoder += clock.elapsed().as_secs_f64();
    counters.logical("source_bytes", bytes.len() as u64);
    let raw_bytes = source
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('%') && !line.starts_with(":-"))
        .map(|line| (line.len() + 1) as u64)
        .sum::<u64>();
    let extra = next
        .iter()
        .flat_map(|c| &c.anonymous)
        .map(|(_, v)| (term::variable_name(*v).len() - 1) as u64)
        .sum::<u64>();
    counters.logical("clause_term_bytes", raw_bytes + extra);
    let start = loaded.clauses.len();
    loaded.clauses.append(&mut next);
    loaded.documents.push(Document {
        path: line.into(),
        bytes,
        clauses: start..loaded.clauses.len(),
    });
    Ok(())
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
    let (term, canonical) = parse_checked(arena, text, false)?;
    if !canonical {
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
    counters.logical(
        "identity_bytes",
        loaded.schema.len() as u64
            + loaded
                .documents
                .iter()
                .map(|d| d.bytes.len() as u64)
                .sum::<u64>(),
    );
    let expected = identity(loaded);
    decode_expected(
        request,
        &mut loaded.arena,
        &loaded.clauses,
        &expected,
        None,
        counters,
    )
}
fn decode_expected(
    request: &Request,
    arena: &mut Arena,
    clauses: &[Clause],
    expected: &str,
    query: Option<Query>,
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
    let indexed = match lines.next() {
        Some("sb1-certificate 0") => false,
        Some("sb1-certificate 1") => true,
        _ => return Err(Failure::refused("envelope", "certificate version")),
    };
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
    if got != expected {
        return Err(Failure::refused("identity", "composition digest mismatch"));
    }
    let bytes = read(&request.model)?;
    counters.logical("model_bytes", bytes.len() as u64);
    let model_text = utf8(&bytes, "model")?;
    if bytes.contains(&b'\r') {
        return Err(Failure::parse("non-LF model"));
    }
    if !bytes.is_empty() && !model_text.ends_with('\n') {
        return Err(Failure::parse("model missing LF"));
    }
    let mut facts = Vec::new();
    let mut positions = HashMap::new();
    for (i, line) in model_text.lines().enumerate() {
        let (fact, canonical) = parse_checked(arena, line, true)
            .map_err(|e| Failure::parse(format!("model line {} {e}", i + 1)))?;
        if !program::model_fact(arena, fact) {
            return Err(Failure::refused(
                "claim",
                format!("model line {} term domain", i + 1),
            ));
        }
        if !canonical {
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
    let clause_ids = clauses
        .iter()
        .enumerate()
        .map(|(i, c)| ((c.document, c.ordinal), i))
        .collect::<HashMap<_, _>>();
    let mut claims = Vec::new();
    let mut levels: Vec<Option<Level>> = vec![None; facts.len()];
    for (i, line) in lines.enumerate() {
        let row = i + 3;
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 4 {
            return Err(Failure::refused("envelope", format!("record {row} fields")));
        }
        let fact = if indexed {
            if fields[0].is_empty() || !fields[0].bytes().all(|b| b.is_ascii_digit()) {
                return Err(Failure::refused(
                    "envelope",
                    format!("record {row} model index"),
                ));
            }
            fields[0]
                .parse::<usize>()
                .ok()
                .and_then(|n| n.checked_sub(1))
                .filter(|&n| n < facts.len())
                .ok_or_else(|| {
                    Failure::refused("claim", format!("record {row} model index out of range"))
                })?
        } else {
            let (fact, canonical) = parse_checked(arena, fields[0], false)
                .map_err(|e| Failure::refused("envelope", format!("record {row} {e}")))?;
            if !canonical {
                return Err(Failure::refused(
                    "envelope",
                    format!("record {row} noncanonical fact"),
                ));
            }
            *positions.get(&fact).ok_or_else(|| {
                Failure::refused("claim", format!("record {row} fact not in model"))
            })?
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
        let env = valuation(arena, fields[2], &clauses[clause])
            .map_err(|e| Failure::refused("claim", format!("record {row} {e}")))?;
        let level = Level::parse(fields[3])
            .ok_or_else(|| Failure::refused("envelope", format!("record {row} level")))?;
        if level > counters.level_max {
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
    let query = if let Some(query) = query {
        query
    } else {
        let query_bytes = request.query.as_deref().map(read).transpose()?;
        Query::parse(arena, query_bytes.as_deref())?
    };
    Ok((Model::new(arena, facts, levels)?, claims, query))
}

fn arena_counters(arena: &Arena, model: &Model, clauses: &[Clause], counters: &mut Counters) {
    counters.logical("interned_terms", arena.nodes.len() as u64);
    counters.runtime("interning_requests", arena.lookups);
    fn depth(
        arena: &Arena,
        id: term::Id,
        cache: &mut HashMap<term::Id, u64>,
        width: &mut usize,
    ) -> u64 {
        if let Some(depth) = cache.get(&id) {
            return *depth;
        }
        let value = match arena.node(id) {
            term::Node::Compound(_, args) => {
                1 + args
                    .iter()
                    .map(|id| depth(arena, *id, cache, width))
                    .max()
                    .unwrap_or(0)
            }
            term::Node::Integer(n) => {
                *width = (*width).max(n.trim_start_matches('-').len());
                0
            }
            _ => 0,
        };
        cache.insert(id, value);
        value
    }
    fn goal_depth(
        arena: &Arena,
        goals: &[program::Goal],
        cache: &mut HashMap<term::Id, u64>,
        width: &mut usize,
    ) -> u64 {
        goals
            .iter()
            .map(|goal| match goal {
                program::Goal::Positive(term) | program::Goal::Builtin(term) => {
                    depth(arena, *term, cache, width).saturating_sub(1)
                }
                program::Goal::Not(inner) => goal_depth(arena, inner, cache, width),
            })
            .max()
            .unwrap_or(0)
    }
    // Prepared arenas also hold dormant sources. Counters traverse the checked
    // slice's reachable terms, not the whole knowledge base after each call.
    let mut cache = HashMap::new();
    let mut width = 0;
    let mut maximum = 0;
    for fact in &model.facts {
        maximum = maximum.max(depth(arena, *fact, &mut cache, &mut width).saturating_sub(1));
    }
    for clause in clauses {
        maximum = maximum.max(depth(arena, clause.head, &mut cache, &mut width).saturating_sub(1));
        maximum = maximum.max(goal_depth(arena, &clause.body, &mut cache, &mut width));
    }
    counters.logical("term_depth_max", maximum);
    counters.logical("number_decimal_digits_max", width as u64);
}

fn admit_loaded(
    loaded: &mut Loaded,
    counters: &mut Counters,
    mode: SourceCodec,
) -> Result<(), Failure> {
    admit_range(
        loaded,
        counters,
        mode,
        0..loaded.documents.len(),
        0..loaded.clauses.len(),
    )
}
fn admit_range(
    loaded: &mut Loaded,
    counters: &mut Counters,
    mode: SourceCodec,
    documents: std::ops::Range<usize>,
    clauses: std::ops::Range<usize>,
) -> Result<(), Failure> {
    let clock = Instant::now();
    let result = (|| {
        for doc in &loaded.documents[documents] {
            let clauses = &loaded.clauses[doc.clauses.clone()];
            let text = utf8(&doc.bytes, "source")?;
            let docid = clauses
                .iter()
                .find_map(|c| loaded.arena.named(c.head, "guideline_document", 4))
                .and_then(|a| loaded.arena.atom_name(a[0]));
            match mode {
                SourceCodec::Kernel => {
                    if canonical(&doc.bytes) {
                        continue;
                    }
                    let code_at = text
                        .lines()
                        .take_while(|line| line.starts_with('%'))
                        .map(|line| line.len() + 1)
                        .sum::<usize>();
                    let adapted=docid.map(|id| format!("% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}",&text[code_at..]));
                    if !adapted.is_some_and(|bytes| canonical(bytes.as_bytes())) {
                        return Err(Failure::parse(format!("noncanonical source {}", doc.path)));
                    }
                    counters.logical("source_header_adapters", 1);
                }
                SourceCodec::Parsed => {
                    if !codec::document(&loaded.arena, clauses, text) {
                        return Err(Failure::parse(format!("noncanonical source {}", doc.path)));
                    }
                    let header=docid.map(|id| format!("% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n:-"));
                    if !header.is_some_and(|h| text.starts_with(&h)) {
                        counters.logical("source_header_adapters", 1);
                    }
                }
            }
        }
        Ok(())
    })();
    loaded.source_profile.canonical += clock.elapsed().as_secs_f64();
    result?;
    let clock = Instant::now();
    let result = program::admit(&loaded.arena, &loaded.clauses[clauses])
        .map_err(|e| Failure::refused("claim", format!("admission {e}")));
    loaded.source_profile.shape_safety += clock.elapsed().as_secs_f64();
    result
}

pub fn profile_sources(
    root: &Path,
    composition: &Path,
    mode: SourceCodec,
) -> Result<String, Failure> {
    let request = Request {
        root: root.into(),
        composition: composition.into(),
        model: PathBuf::new(),
        certificate: PathBuf::new(),
        query: None,
    };
    let mut counters = Counters::default();
    let mut loaded = load_with_codec(&request, &mut counters, mode)?;
    admit_loaded(&mut loaded, &mut counters, mode)?;
    Ok(loaded.source_profile.render())
}

pub fn check(request: &Request) -> Outcome {
    check_with_codec(request, SourceCodec::Parsed)
}

pub fn check_with_codec(request: &Request, mode: SourceCodec) -> Outcome {
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
            loaded = Some(load_with_codec(request, c, mode)?);
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
            admit_loaded(loaded, counters, mode)
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
    let source_profile = loaded.map(|l| l.source_profile).unwrap_or_default();
    match result {
        Ok(stdout) => Outcome {
            rc: 0,
            stdout,
            stderr: String::new(),
            phases,
            counters,
            source_profile,
        },
        Err(failure) => Outcome {
            rc: failure.rc,
            stdout: String::new(),
            stderr: format!("{}\n", failure.message),
            phases,
            counters,
            source_profile,
        },
    }
}

#[cfg(test)]
mod schema_tests;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod codec_tests;
#[cfg(test)]
mod preflight_tests;
#[cfg(test)]
mod transport_tests;
