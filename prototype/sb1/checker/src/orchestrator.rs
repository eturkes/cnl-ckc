use crate::digest::{Sha256, sha256};
use crate::engine;
use crate::program::{Clause, Goal};
use crate::query::Query;
use crate::slice::{HeadIndex, static_slice};
use crate::{
    Arena, Clock, Counters, Failure, Loaded, Request, SourceCodec, SourceProfile, admit_loaded,
    admit_range, append_source, arena_counters, composition_paths, decode_expected, read, utf8,
};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClauseId {
    pub document: usize,
    pub ordinal: usize,
}
#[derive(Clone, Debug)]
pub struct ClauseSource {
    pub id: ClauseId,
    pub text: String,
}

// The producer/materializer consumes only original clause ids. A typed provider
// can retain its own prepared graph; its patient-specific selection is charged.
pub trait SliceProvider {
    fn prepare(&mut self, knowledge: &[ClauseSource]) -> Result<(), Failure>;
    fn select(&mut self, patient: &[ClauseSource], query: &[u8]) -> Result<Vec<ClauseId>, Failure>;
}

#[derive(Clone, Debug)]
pub enum Provider {
    All,
    Static,
    Ids(PathBuf),
}
#[derive(Clone, Debug)]
pub struct OrchestratedRequest {
    pub root: PathBuf,
    pub composition: PathBuf,
    pub output: PathBuf,
    pub query: PathBuf,
    pub provider: Provider,
    pub producer: PathBuf,
    pub swipl: PathBuf,
}
#[derive(Clone, Debug)]
pub struct Interval {
    pub name: &'static str,
    pub kind: &'static str,
    pub seconds: f64,
    pub cpu: f64,
    pub start: f64,
}
fn phase<T>(
    phases: &mut Vec<Interval>,
    origin: Instant,
    name: &'static str,
    kind: &'static str,
    f: impl FnOnce() -> Result<T, Failure>,
) -> Result<T, Failure> {
    let start = origin.elapsed().as_secs_f64();
    let clock = Clock::start()?;
    let result = f();
    let measured = clock.stop(name)?;
    phases.push(Interval {
        name,
        kind,
        seconds: measured.seconds,
        cpu: measured.cpu,
        start,
    });
    result
}
fn source(arena: &Arena, clause: &Clause) -> ClauseSource {
    fn goals(arena: &Arena, body: &[Goal]) -> String {
        body.iter()
            .map(|goal| match goal {
                Goal::Positive(term) | Goal::Builtin(term) => arena.render(*term),
                Goal::Not(inner) => format!("\\+ ({})", goals(arena, inner)),
            })
            .collect::<Vec<_>>()
            .join(",")
    }
    let mut text = arena.render(clause.head);
    if !clause.body.is_empty() {
        text.push_str(" :- ");
        text.push_str(&goals(arena, &clause.body));
    }
    ClauseSource {
        id: ClauseId {
            document: clause.document,
            ordinal: clause.ordinal,
        },
        text,
    }
}
fn identity_record(path: &str, bytes: &[u8]) -> String {
    format!("file\0{path}\0{}\0{}\n", bytes.len(), sha256(bytes))
}

fn mark(origin: Instant, directory: &Path, event: &str) -> Result<(), Failure> {
    if std::env::var_os("SB1_STAGE_TRACE").as_deref() != Some(std::ffi::OsStr::new("1")) {
        return Ok(());
    }
    let wall = origin.elapsed().as_nanos();
    let cpu = crate::cpu_ns()?;
    let path = directory.join("orchestrator-trace.tsv");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| Failure::io(&path, e))?;
    if file.metadata().map_err(|e| Failure::io(&path, e))?.len() == 0 {
        writeln!(file, "event\twall_ns\tcpu_ns\tpid").map_err(|e| Failure::io(&path, e))?;
    }
    writeln!(file, "{event}\t{wall}\t{cpu}\t{}", std::process::id())
        .map_err(|e| Failure::io(&path, e))
}

pub struct PreparedProgram {
    loaded: Loaded,
    index: HeadIndex,
    sources: Vec<ClauseSource>,
    ids: HashMap<ClauseId, usize>,
    identity: Sha256,
    patient: String,
    root: PathBuf,
    codec: SourceCodec,
    origin: Instant,
    pub phases: Vec<Interval>,
    pub counters: Counters,
}
impl PreparedProgram {
    pub fn prepare(
        root: &Path,
        composition: &Path,
        codec: SourceCodec,
        mut external: Option<&mut dyn SliceProvider>,
    ) -> Result<Self, Failure> {
        let origin = Instant::now();
        let mut phases = Vec::new();
        let mut counters = Counters::default();
        let (mut loaded, patient) = phase(&mut phases, origin, "load_parse", "load", || {
            let bytes = read(composition)?;
            let mut paths = composition_paths(utf8(&bytes, "composition")?)?;
            let patient = paths
                .pop()
                .ok_or_else(|| Failure::parse("empty composition"))?;
            let mut loaded = Loaded {
                arena: Arena::default(),
                clauses: Vec::new(),
                documents: Vec::new(),
                schema: read(&root.join("guidelines/schema.pl"))?,
                source_profile: SourceProfile::default(),
            };
            for path in paths {
                append_source(&mut loaded, root, &path, &mut counters, codec)?;
            }
            counters.logical("documents", loaded.documents.len() as u64);
            counters.logical("clauses", loaded.clauses.len() as u64);
            Ok((loaded, patient))
        })?;
        phase(&mut phases, origin, "load_admission", "load", || {
            admit_loaded(&mut loaded, &mut counters, codec)
        })?;
        let sources = phase(&mut phases, origin, "load_analysis", "load", || {
            let sources = loaded
                .clauses
                .iter()
                .map(|c| source(&loaded.arena, c))
                .collect::<Vec<_>>();
            if let Some(provider) = external.as_mut() {
                provider.prepare(&sources)?;
                counters.logical("typed_analysis_hook", 1);
            }
            Ok(sources)
        })?;
        let (index, ids, identity) = phase(&mut phases, origin, "load_index", "load", || {
            let index = HeadIndex::build(&loaded.arena, &loaded.clauses, 0);
            let ids = sources
                .iter()
                .enumerate()
                .map(|(i, source)| (source.id, i))
                .collect();
            let mut identity = Sha256::default();
            identity.update(
                format!(
                    "schema\0{}\0{}\n",
                    loaded.schema.len(),
                    sha256(&loaded.schema)
                )
                .as_bytes(),
            );
            for document in &loaded.documents {
                identity.update(identity_record(&document.path, &document.bytes).as_bytes());
            }
            Ok((index, ids, identity))
        })?;
        Ok(Self {
            loaded,
            index,
            sources,
            ids,
            identity,
            patient,
            root: root.into(),
            codec,
            origin,
            phases,
            counters,
        })
    }

    pub fn trace(&self, directory: &Path, event: &str) -> Result<(), Failure> {
        mark(self.origin, directory, event)
    }

    pub fn write_complete_sink(&self, directory: &Path, text: &str) -> Result<(), Failure> {
        self.trace(directory, "answer_sink_write_begin")?;
        let digest = sha256(text.as_bytes());
        let path = directory.join("answers.txt");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| Failure::io(&path, e))?;
        file.write_all(text.as_bytes())
            .and_then(|_| file.flush())
            .map_err(|e| Failure::io(&path, e))?;
        drop(file);
        let closed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Failure::parse(e.to_string()))?
            .as_nanos();
        let stat = Path::new("/proc/self/stat");
        let cpu = fs::read_to_string(stat).map_err(|e| Failure::io(stat, e))?;
        let fields = cpu
            .rsplit_once(')')
            .ok_or_else(|| Failure::parse("sink CPU stat"))?
            .1
            .split_whitespace()
            .collect::<Vec<_>>();
        let mut ticks = 0_u64;
        for index in [11, 12, 13, 14] {
            let value = fields
                .get(index)
                .and_then(|s| s.parse::<i64>().ok())
                .ok_or_else(|| Failure::parse("sink CPU fields"))?;
            ticks = ticks.saturating_add(value.max(0) as u64);
        }
        self.trace(directory, "answer_sink_closed")?;
        let tmp = directory.join("sink-complete.tmp");
        fs::write(&tmp, format!("sb1-sink 1\nclosed_unix_ns\t{closed}\nbytes\t{}\nsha256\t{digest}\ncpu_ticks\t{ticks}\n", text.len())).map_err(|e| Failure::io(&tmp, e))?;
        let marker = directory.join("sink-complete.tsv");
        fs::rename(&tmp, &marker).map_err(|e| Failure::io(&marker, e))?;
        self.trace(directory, "sink_marker_published")
    }

    pub fn call(
        &mut self,
        request: &OrchestratedRequest,
        external: Option<&mut dyn SliceProvider>,
    ) -> Result<String, Failure> {
        self.trace(&request.output, "call_begin")?;
        let checkpoint = self.loaded.arena.checkpoint();
        let documents = self.loaded.documents.len();
        let clauses = self.loaded.clauses.len();
        let result = self.call_inner(request, external);
        let finished = self.trace(&request.output, "call_inner_done");
        self.loaded.documents.truncate(documents);
        let docs = self.trace(&request.output, "patient_documents_dropped");
        self.loaded.clauses.truncate(clauses);
        let clauses = self.trace(&request.output, "patient_clauses_dropped");
        self.loaded.arena.rewind(checkpoint);
        let arena = self.trace(&request.output, "arena_rewound");
        finished.and(docs).and(clauses).and(arena)?;
        result
    }

    fn call_inner(
        &mut self,
        request: &OrchestratedRequest,
        external: Option<&mut dyn SliceProvider>,
    ) -> Result<String, Failure> {
        let document_count = self.loaded.documents.len();
        let base_count = self.loaded.clauses.len();
        let identity = phase(&mut self.phases, self.origin, "patient", "scored", || {
            mark(self.origin, &request.output, "patient_decode_begin")?;
            append_source(
                &mut self.loaded,
                &self.root,
                &self.patient,
                &mut self.counters,
                self.codec,
            )?;
            mark(self.origin, &request.output, "patient_decode_end")?;
            let document = self.loaded.documents.last().expect("patient document");
            if self.loaded.clauses[document.clauses.clone()]
                .iter()
                .any(|c| !c.body.is_empty())
            {
                return Err(Failure::refused("claim", "patient contains a rule"));
            }
            let clause_count = self.loaded.clauses.len();
            mark(self.origin, &request.output, "patient_admission_begin")?;
            admit_range(
                &mut self.loaded,
                &mut self.counters,
                self.codec,
                document_count..document_count + 1,
                base_count..clause_count,
            )?;
            mark(self.origin, &request.output, "patient_admission_end")?;
            let patient_clauses = (clause_count - base_count) as u64;
            self.counters.logical("documents", 1);
            self.counters.logical("clauses", patient_clauses);
            self.counters.logical("patient_facts", patient_clauses);
            mark(self.origin, &request.output, "identity_begin")?;
            let mut identity = self.identity.clone();
            let document = self.loaded.documents.last().expect("patient document");
            identity.update(identity_record(&document.path, &document.bytes).as_bytes());
            self.counters
                .logical("identity_bytes", document.bytes.len() as u64);
            let identity = identity.finish();
            mark(self.origin, &request.output, "identity_end")?;
            Ok(identity)
        })?;
        let (query, selected, patient_sources) =
            phase(&mut self.phases, self.origin, "slice", "scored", || {
                mark(self.origin, &request.output, "query_decode_begin")?;
                let bytes = read(&request.query)?;
                let query = Query::parse(&mut self.loaded.arena, Some(&bytes))?;
                mark(self.origin, &request.output, "query_decode_end")?;
                let patient_sources = self.loaded.clauses[base_count..]
                    .iter()
                    .map(|c| source(&self.loaded.arena, c))
                    .collect::<Vec<_>>();
                mark(self.origin, &request.output, "slice_select_begin")?;
                let selected = if let Some(provider) = external {
                    let ids = provider.select(&patient_sources, &bytes)?;
                    Self::resolve_ids(&self.ids, self.sources.len(), &ids, &patient_sources)?
                } else {
                    match &request.provider {
                        Provider::All => (0..self.loaded.clauses.len()).collect(),
                        Provider::Static => {
                            let patient = HeadIndex::build(
                                &self.loaded.arena,
                                &self.loaded.clauses[base_count..],
                                base_count,
                            );
                            static_slice(
                                &mut self.loaded.arena,
                                &self.loaded.clauses,
                                &self.index,
                                &patient,
                                &query,
                            )
                        }
                        Provider::Ids(path) => {
                            let text = read(path)?;
                            let ids = utf8(&text, "clause selection")?
                                .lines()
                                .map(|line| {
                                    let (document, ordinal) = line
                                        .split_once(':')
                                        .ok_or_else(|| Failure::parse("clause id"))?;
                                    Ok(ClauseId {
                                        document: document
                                            .parse()
                                            .map_err(|_| Failure::parse("clause document"))?,
                                        ordinal: ordinal
                                            .parse()
                                            .map_err(|_| Failure::parse("clause ordinal"))?,
                                    })
                                })
                                .collect::<Result<Vec<_>, Failure>>()?;
                            Self::resolve_ids(
                                &self.ids,
                                self.sources.len(),
                                &ids,
                                &patient_sources,
                            )?
                        }
                    }
                };
                mark(self.origin, &request.output, "slice_select_end")?;
                self.counters
                    .logical("selected_clauses", selected.len() as u64);
                Ok((query, selected, patient_sources))
            })?;
        mark(self.origin, &request.output, "slice_clone_begin")?;
        let selected_clauses = selected
            .iter()
            .map(|i| self.loaded.clauses[*i].clone())
            .collect::<Vec<_>>();
        mark(self.origin, &request.output, "slice_clone_end")?;
        let slice = request.output.join("slice.pl.in");
        phase(
            &mut self.phases,
            self.origin,
            "materialize",
            "scored",
            || {
                mark(self.origin, &request.output, "slice_write_begin")?;
                let file = File::create(&slice).map_err(|e| Failure::io(&slice, e))?;
                let mut writer = BufWriter::new(file);
                let mut ids = String::new();
                for row in &selected {
                    let source = if *row < base_count {
                        &self.sources[*row]
                    } else {
                        &patient_sources[*row - base_count]
                    };
                    writeln!(
                        writer,
                        "sb1_clause({},{},({})).",
                        source.id.document, source.id.ordinal, source.text
                    )
                    .map_err(|e| Failure::io(&slice, e))?;
                    ids.push_str(&format!("{}:{}\n", source.id.document, source.id.ordinal));
                }
                writer.flush().map_err(|e| Failure::io(&slice, e))?;
                let path = request.output.join("selected-clauses.tsv");
                fs::write(&path, ids).map_err(|e| Failure::io(&path, e))?;
                mark(self.origin, &request.output, "slice_write_end")?;
                Ok(())
            },
        )?;
        phase(&mut self.phases, self.origin, "producer", "scored", || {
            let stdout = request.output.join("producer.stdout");
            let stderr = request.output.join("producer.stderr");
            mark(self.origin, &request.output, "producer_spawn_begin")?;
            let status = Command::new(&request.swipl)
                .args([
                    "--stack-limit=32g",
                    "--table-space=32g",
                    "-q",
                    "-f",
                    "none",
                    "-F",
                    "none",
                    "-s",
                ])
                .arg(&request.producer)
                .args(["-g", "sb1_main", "-t", "halt(9)", "--", "--root"])
                .arg(&request.root)
                .arg("--slice")
                .arg(&identity)
                .arg(&slice)
                .arg(&request.output)
                .stdin(Stdio::null())
                .stdout(File::create(&stdout).map_err(|e| Failure::io(&stdout, e))?)
                .stderr(File::create(&stderr).map_err(|e| Failure::io(&stderr, e))?)
                .status()
                .map_err(|e| Failure::io(&request.swipl, e))?;
            mark(self.origin, &request.output, "producer_exited")?;
            if !status.success()
                || fs::metadata(&stdout)
                    .map_err(|e| Failure::io(&stdout, e))?
                    .len()
                    != 0
                || fs::metadata(&stderr)
                    .map_err(|e| Failure::io(&stderr, e))?
                    .len()
                    != 0
            {
                return Err(Failure {
                    rc: 3,
                    message: format!("indeterminate producer {status}; see {}", stderr.display()),
                });
            }
            Ok(())
        })?;
        let check_request = Request {
            root: request.root.clone(),
            composition: request.composition.clone(),
            model: request.output.join("model.txt"),
            certificate: request.output.join("certificate.txt"),
            query: None,
        };
        let (mut model, claims, query) =
            phase(&mut self.phases, self.origin, "decode", "scored", || {
                mark(
                    self.origin,
                    &request.output,
                    "model_certificate_decode_begin",
                )?;
                let result = decode_expected(
                    &check_request,
                    &mut self.loaded.arena,
                    &selected_clauses,
                    &identity,
                    Some(query),
                    &mut self.counters,
                );
                mark(self.origin, &request.output, "model_certificate_decode_end")?;
                result
            })?;
        mark(self.origin, &request.output, "check_begin")?;
        phase(&mut self.phases, self.origin, "index", "scored", || {
            model.build_indices(
                &self.loaded.arena,
                &selected_clauses,
                query.goals(),
                &mut self.counters,
            );
            Ok(())
        })?;
        phase(&mut self.phases, self.origin, "support", "scored", || {
            engine::support(
                &mut self.loaded.arena,
                &model,
                &selected_clauses,
                &claims,
                &mut self.counters,
            )
        })?;
        phase(&mut self.phases, self.origin, "closure", "scored", || {
            engine::closure(
                &mut self.loaded.arena,
                &model,
                &selected_clauses,
                &mut self.counters,
            )
        })?;
        mark(self.origin, &request.output, "check_end")?;
        phase(&mut self.phases, self.origin, "answers", "scored", || {
            mark(self.origin, &request.output, "answers_compute_begin")?;
            let output = query.answers(&mut self.loaded.arena, &model, &mut self.counters)?;
            self.counters.logical("output_bytes", output.len() as u64);
            arena_counters(
                &self.loaded.arena,
                &model,
                &selected_clauses,
                &mut self.counters,
            );
            mark(self.origin, &request.output, "answers_compute_end")?;
            Ok(output)
        })
    }

    fn resolve_ids(
        knowledge_ids: &HashMap<ClauseId, usize>,
        base_count: usize,
        ids: &[ClauseId],
        patient: &[ClauseSource],
    ) -> Result<Vec<usize>, Failure> {
        let patient_ids = patient
            .iter()
            .enumerate()
            .map(|(i, source)| (source.id, base_count + i))
            .collect::<HashMap<_, _>>();
        let mut seen = HashSet::new();
        let mut rows = ids
            .iter()
            .map(|id| {
                if !seen.insert(*id) {
                    return Err(Failure::parse("duplicate selected clause id"));
                }
                knowledge_ids
                    .get(id)
                    .or_else(|| patient_ids.get(id))
                    .copied()
                    .ok_or_else(|| {
                        Failure::parse(format!(
                            "selected clause id {}:{} absent",
                            id.document, id.ordinal
                        ))
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        rows.sort_unstable();
        Ok(rows)
    }

    pub fn write_metrics(&self, directory: &Path) -> Result<(), Failure> {
        self.trace(directory, "metrics_write_begin")?;
        let mut text = String::from("phase\tseconds\tcpu\tstart_seconds\tkind\n");
        for phase in &self.phases {
            text.push_str(&format!(
                "{}\t{:.9}\t{:.9}\t{:.9}\t{}\n",
                phase.name, phase.seconds, phase.cpu, phase.start, phase.kind
            ));
        }
        for (name, text) in [
            ("orchestrator-phases.tsv", text),
            ("checker-counters.tsv", self.counters.render()),
            (
                "checker-source-profile.tsv",
                self.loaded.source_profile.render(),
            ),
        ] {
            let path = directory.join(name);
            fs::write(&path, text).map_err(|e| Failure::io(&path, e))?;
        }
        self.trace(directory, "metrics_write_end")?;
        Ok(())
    }
}

pub fn orchestrate(request: &OrchestratedRequest, codec: SourceCodec) -> Result<String, Failure> {
    orchestrate_retained(request, codec).map(|(stdout, _prepared)| stdout)
}

pub fn orchestrate_retained(
    request: &OrchestratedRequest,
    codec: SourceCodec,
) -> Result<(String, PreparedProgram), Failure> {
    fs::create_dir_all(&request.output).map_err(|e| Failure::io(&request.output, e))?;
    let mut prepared = PreparedProgram::prepare(&request.root, &request.composition, codec, None)?;
    let result = prepared.call(request, None);
    prepared.write_metrics(&request.output)?;
    prepared.trace(&request.output, "result_return_ready")?;
    Ok((result?, prepared))
}
