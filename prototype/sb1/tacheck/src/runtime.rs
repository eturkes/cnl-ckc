//! Untrusted warm adapter. The checking core receives source IR + hints alone.
use crate::{core, producer};
use core::analysis::Checked;
use core::extension::Original;
use sb1_checker::orchestrator::{ClauseId, ClauseSource, SliceProvider};
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, Write};
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;

type Result<T> = std::result::Result<T, sb1_checker::Failure>;
fn io_failure(error: impl std::fmt::Display) -> sb1_checker::Failure {
    sb1_checker::Failure {
        rc: 2,
        message: format!("I/O TAS {error}"),
    }
}
fn error(e: core::Failure) -> sb1_checker::Failure {
    let (rc, message) = match e {
        core::Failure::Admission(s) => (1, format!("refused (admission) {s}")),
        core::Failure::Certificate(s) => (1, format!("refused (certificate) {s}")),
        core::Failure::Indeterminate(s) => (3, format!("indeterminate {s}")),
        core::Failure::Cycle {
            kind,
            edges,
            sites,
            labels,
        } => (
            1,
            format!("refused ({kind} cycle) edges={edges:?} sites={sites:?} labels={labels:?}"),
        ),
    };
    sb1_checker::Failure { rc, message }
}
fn originals(sources: &[ClauseSource]) -> Vec<Original<'_>> {
    sources
        .iter()
        .map(|s| Original {
            document: s.id.document,
            ordinal: s.id.ordinal,
            text: &s.text,
        })
        .collect()
}
fn clocks() -> io::Result<(u64, u64, u64)> {
    let stat = fs::read_to_string("/proc/self/schedstat")?;
    let cpu = stat
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| io::Error::other("schedstat CPU"))?;
    let status = fs::read_to_string("/proc/self/status")?;
    let bytes = |key| -> io::Result<u64> {
        status
            .lines()
            .find_map(|l| l.strip_prefix(key))
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse::<u64>().ok())
            .and_then(|n| n.checked_mul(1024))
            .ok_or_else(|| io::Error::other(format!("status {key}")))
    };
    Ok((cpu, bytes("VmRSS:")?, bytes("VmHWM:")?))
}
struct Recorder {
    origin: Instant,
    phases: File,
    payload: File,
    selections: File,
    info: File,
}
#[derive(Clone)]
pub struct Trace(Rc<RefCell<Recorder>>);
impl Trace {
    fn new(out: &Path) -> Result<Self> {
        fs::create_dir_all(out).map_err(io_failure)?;
        let mut phases = File::create(out.join("ta-phases.tsv")).map_err(io_failure)?;
        writeln!(
            phases,
            "event\twall_ns\tcpu_ns\trss_bytes\tvmhwm_bytes\tpid"
        )
        .map_err(io_failure)?;
        let mut payload = File::create(out.join("ta-payload.tsv")).map_err(io_failure)?;
        writeln!(payload, "stage\tcomponent\tbytes\tkind").map_err(io_failure)?;
        let mut selections = File::create(out.join("ta-selections.tsv")).map_err(io_failure)?;
        writeln!(
            selections,
            "call\twall_ns\tcpu_ns\trss_bytes\tvmhwm_bytes\tclauses\tquery_bytes\toutcome"
        )
        .map_err(io_failure)?;
        let mut info = File::create(out.join("ta-info.tsv")).map_err(io_failure)?;
        writeln!(info, "key\tvalue").map_err(io_failure)?;
        Ok(Self(Rc::new(RefCell::new(Recorder {
            origin: Instant::now(),
            phases,
            payload,
            selections,
            info,
        }))))
    }
    pub fn mark(&self, name: &str) -> Result<()> {
        let (cpu, rss, hwm) = clocks().map_err(io_failure)?;
        let mut r = self.0.borrow_mut();
        let wall = r.origin.elapsed().as_nanos();
        writeln!(
            r.phases,
            "{name}\t{wall}\t{cpu}\t{rss}\t{hwm}\t{}",
            std::process::id()
        )
        .and_then(|_| r.phases.flush())
        .map_err(io_failure)
    }
    fn core_mark(&self, name: &str) -> core::Result<()> {
        self.mark(name)
            .map_err(|e| core::Failure::Indeterminate(e.message))
    }
    fn payload(&self, stage: &str, name: &str, bytes: usize) -> Result<()> {
        let mut r = self.0.borrow_mut();
        writeln!(r.payload, "{stage}\t{name}\t{bytes}\tpayload-lower-bound")
            .and_then(|_| r.payload.flush())
            .map_err(io_failure)
    }
    fn info(&self, key: &str, value: impl std::fmt::Display) -> Result<()> {
        let mut r = self.0.borrow_mut();
        writeln!(r.info, "{key}\t{value}")
            .and_then(|_| r.info.flush())
            .map_err(io_failure)
    }
    pub fn writer<W: Write>(&self, writer: W) -> ProtocolWriter<W> {
        ProtocolWriter {
            inner: writer,
            trace: self.clone(),
            position: 0,
            matched: true,
            ready: false,
        }
    }
}
// Publication is observed at the real protocol flush, after all outer load
// phases/indices/identity files exist. No mutable checker state crosses here.
pub struct ProtocolWriter<W> {
    inner: W,
    trace: Trace,
    position: usize,
    matched: bool,
    ready: bool,
}
impl<W: Write> Write for ProtocolWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        for b in &buf[..n] {
            if *b == b'\n' {
                self.ready |= self.matched && self.position == 5;
                self.position = 0;
                self.matched = true;
            } else {
                self.matched &= b"ready".get(self.position) == Some(b);
                self.position = self.position.saturating_add(1);
            }
        }
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()?;
        if self.ready {
            self.trace
                .mark("server-ready-publication")
                .map_err(|e| io::Error::other(e.message))?;
            self.ready = false;
        }
        Ok(())
    }
}
fn model_payload(trace: &Trace, stage: &str, m: &core::May) -> Result<()> {
    trace.payload(stage, "model-set-members-and-cells", m.sets.payload_bytes())?;
    trace.payload(
        stage,
        "model-rectangles",
        std::mem::size_of_val(m.rectangles.as_slice()),
    )?;
    trace.payload(
        stage,
        "origin-keys",
        m.abs.origins.capacity() * size_of::<[usize; 4]>(),
    )?;
    trace.payload(
        stage,
        "term-abstraction",
        m.abs.terms.capacity() * size_of::<u32>(),
    )
}
fn flow_payload(trace: &Trace, stage: &str, f: &core::growth::Evidence) -> Result<()> {
    trace.payload(
        stage,
        "source-rows",
        f.sources.rows.capacity() * size_of::<core::growth::Source>(),
    )?;
    trace.payload(
        stage,
        "source-type-vectors",
        f.sources
            .rows
            .iter()
            .map(|s| s.types.capacity() * size_of::<u32>())
            .sum(),
    )?;
    trace.payload(
        stage,
        "type-nodes",
        f.types.nodes.capacity() * size_of::<core::growth::TypeNode>(),
    )
}

pub struct B1Provider {
    root: PathBuf,
    composition: PathBuf,
    all: bool,
    checked: Option<Checked>,
    calls: usize,
    pub trace: Trace,
}
impl B1Provider {
    pub fn new(root: PathBuf, composition: PathBuf, out: PathBuf, all: bool) -> Result<Self> {
        let trace = Trace::new(&out)?;
        trace.mark("server-before-prepare")?;
        Ok(Self {
            root,
            composition,
            all,
            checked: None,
            calls: 0,
            trace,
        })
    }
}
impl SliceProvider for B1Provider {
    fn prepare(&mut self, knowledge: &[ClauseSource]) -> Result<()> {
        if self.checked.is_some() {
            return Err(error(core::bad("provider already prepared")));
        }
        self.trace.mark("ta-parse")?;
        let mut p = sb1_ta::source::Program::load_knowledge(&self.root, &self.composition)
            .map_err(|s| error(core::Failure::Admission(s)))?;
        self.trace.mark("ta-shape")?;
        p.dc_shape()
            .map_err(|s| error(core::Failure::Admission(s)))?;
        self.trace.mark("ta-shape-binding")?;
        core::extension::check_ir(
            &originals(knowledge),
            &p.arena,
            &p.clauses,
            &vec![false; p.clauses.len()],
        )
        .map_err(error)?;
        self.trace.mark("ta-solver")?;
        let a = sb1_ta::hints::analyze(&p)
            .map_err(|s| error(core::Failure::Indeterminate(format!("solver {s}"))))?;
        self.trace.mark("ta-hint-export")?;
        let hint = sb1_ta::hints::export(&p, a)
            .map_err(|s| error(core::Failure::Indeterminate(format!("hint producer {s}"))))?;
        self.trace.info("hint_version", hint.version)?;
        self.trace.info("patient_top", hint.model.generic)?;
        self.trace
            .info("base_precision_fallbacks", hint.flow.precision_fallbacks)?;
        self.trace.info(
            "proposed_refinement",
            hint.refined.as_ref().map_or("base", |r| {
                if r.full_refinement {
                    "full-k1"
                } else {
                    "mixed-k1"
                }
            }),
        )?;
        if let Some(r) = &hint.refined {
            self.trace.info("full_refinement", r.full_refinement)?;
            self.trace.info("refined_types", r.registry.len())?;
            self.trace.info(
                "refined_source_variants",
                r.variants
                    .iter()
                    .filter(|(original, _)| *original != usize::MAX)
                    .count(),
            )?;
        }
        self.trace
            .payload("handover", "base-may-hint", hint.model.payload_bytes())?;
        if let Some(r) = &hint.refined {
            self.trace
                .payload("handover", "mixed-may-hint", r.model.payload_bytes())?;
        }
        self.trace.mark("ta-handover-solver-released")?;
        let p = producer::program(p);
        // The compact original location table survives a failed consuming check;
        // no second source arena or full rendered-clause copy is retained.
        let labels = p
            .clauses
            .iter()
            .map(|c| (c.document, c.line + 1, c.ordinal))
            .collect::<Vec<_>>();
        let documents = p.documents.clone();
        let cert = producer::decode(&p, hint).map_err(error)?;
        self.trace.mark("ta-hint-decode-complete")?;
        model_payload(&self.trace, "decoded-base", &cert.model)?;
        flow_payload(&self.trace, "decoded-base", &cert.flow)?;
        if let Some(r) = &cert.refined {
            model_payload(&self.trace, "decoded-mixed", &r.model)?;
            flow_payload(&self.trace, "decoded-mixed", &r.flow)?;
        }
        let trace = self.trace.clone();
        let checked =
            core::analysis::check_with_observer(p, cert, &mut core::Budget::default(), |name| {
                trace.core_mark(name)
            })
            .map_err(|e| {
                if let core::Failure::Cycle {
                    kind,
                    edges,
                    sites,
                    labels: growth,
                } = &e
                {
                    let label = |i: usize| {
                        let (d, line, o) = labels[i];
                        format!("{}:{line}:{o}", documents[d - 1])
                    };
                    let mut text = format!("refused ({kind} cycle)");
                    if *kind == "negative" {
                        for (a, b, neg) in edges {
                            text.push_str(&format!(
                                "\n{}\t{}\t{}",
                                label(*a),
                                label(*b),
                                if *neg { "-" } else { "+" }
                            ));
                        }
                    } else {
                        for ((a, b), (c, v)) in growth.iter().zip(sites) {
                            text.push_str(&format!(
                                "\n{a}\t{b}\t{}\t{}",
                                label(*c),
                                sb1_checker::term::variable_name(*v)
                            ));
                        }
                    }
                    sb1_checker::Failure {
                        rc: 1,
                        message: text,
                    }
                } else {
                    error(e)
                }
            })?;
        drop(labels);
        drop(documents);
        model_payload(&self.trace, "retained", &checked.model)?;
        self.trace.payload(
            "retained",
            "csr-offsets",
            checked.graph.offsets.capacity() * size_of::<usize>(),
        )?;
        self.trace.payload(
            "retained",
            "csr-signed-targets",
            checked.graph.targets.capacity() * size_of::<u32>(),
        )?;
        self.trace.payload(
            "retained",
            "source-clause-structs",
            checked.program.clauses.capacity() * size_of::<sb1_checker::program::Clause>(),
        )?;
        self.checked = Some(checked);
        self.trace.info("checked_admission", "admitted")?;
        self.trace.mark("ta-provider-ready")
    }
    fn select(&mut self, patient: &[ClauseSource], query: &[u8]) -> Result<Vec<ClauseId>> {
        let c = self
            .checked
            .as_mut()
            .ok_or_else(|| error(core::Failure::Indeterminate("provider not prepared".into())))?;
        self.calls += 1;
        let start = Instant::now();
        let (cpu, _, _) = clocks().map_err(io_failure)?;
        self.trace.mark("ta-select-begin")?;
        let result = (|| {
            c.check_patient(&originals(patient))
                .map_err(|s| error(core::Failure::Admission(s)))?;
            let selected = if self.all {
                (0..c.program.clauses.len()).collect()
            } else {
                c.select(query, &mut core::Budget::default())
                    .map_err(error)?
            };
            Ok(selected
                .into_iter()
                .map(|i| {
                    let s = &c.program.clauses[i];
                    ClauseId {
                        document: s.document,
                        ordinal: s.ordinal,
                    }
                })
                .collect::<Vec<_>>())
        })();
        let wall = start.elapsed().as_nanos();
        let (end, rss, hwm) = clocks().map_err(io_failure)?;
        let mut r = self.trace.0.borrow_mut();
        let count = result.as_ref().map_or(0, Vec::len);
        let outcome = result
            .as_ref()
            .map_or_else(|e: &sb1_checker::Failure| e.rc.to_string(), |_| "0".into());
        writeln!(
            r.selections,
            "{}\t{wall}\t{}\t{rss}\t{hwm}\t{count}\t{}\t{outcome}",
            self.calls,
            end.saturating_sub(cpu),
            query.len()
        )
        .and_then(|_| r.selections.flush())
        .map_err(io_failure)?;
        drop(r);
        self.trace.mark("ta-select-complete")?;
        result
    }
}
