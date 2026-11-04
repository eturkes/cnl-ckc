use super::config::Config;
use super::procfs::{self, Sampler};
use super::{Result, ns, sink};
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

pub(super) fn trace(
    work: &Path,
    origin: Instant,
    role: &str,
    event: &str,
    child: Option<u32>,
) -> Result<()> {
    if std::env::var_os("SB1_STAGE_TRACE").as_deref() != Some(std::ffi::OsStr::new("1")) {
        return Ok(());
    }
    let wall = ns(origin.elapsed());
    let ticks = own_ticks();
    let path = work
        .parent()
        .ok_or("trace work parent")?
        .join("harness-stage-trace.tsv");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() == 0 {
        writeln!(file, "role\tevent\twall_ns\tparent_own_ticks\tchild_pid\tchild_start_ticks\tchild_state\tchild_own_ticks\tchild_waited_ticks").map_err(|e| e.to_string())?;
    }
    let child = child
        .and_then(procfs::stat)
        .map(|p| {
            format!(
                "{}\t{}\t{}\t{}\t{}",
                p.pid, p.start, p.state, p.own_ticks, p.children_ticks
            )
        })
        .unwrap_or_else(|| "\t\t\t\t".to_owned());
    writeln!(file, "{role}\t{event}\t{wall}\t{ticks}\t{child}").map_err(|e| e.to_string())
}

pub(super) struct Interval {
    pub phase: String,
    pub kind: &'static str,
    pub start: u64,
    pub end: u64,
    pub cpu: f64,
}
pub(super) struct Ledger {
    pub intervals: Vec<Interval>,
    pub cursor: u64,
    own_ticks: u64,
    ticks_per_second: f64,
    lifecycle: bool,
}
fn own_ticks() -> u64 {
    procfs::stat(std::process::id())
        .map(|p| p.own_ticks)
        .unwrap_or(0)
}
impl Ledger {
    pub fn new(ticks_per_second: f64) -> Self {
        Self {
            intervals: Vec::new(),
            cursor: 0,
            own_ticks: own_ticks(),
            ticks_per_second,
            lifecycle: false,
        }
    }
    fn append(&mut self, interval: Interval) {
        if interval.kind == "lifecycle"
            && let Some(last) = self.intervals.last_mut()
            && last.kind == "lifecycle"
            && last.end == interval.start
        {
            last.end = interval.end;
            last.cpu += interval.cpu;
        } else {
            self.intervals.push(interval);
        }
    }
    pub fn gap(&mut self, end: u64) {
        let ticks = own_ticks();
        self.append(Interval {
            phase: if self.lifecycle {
                "teardown"
            } else {
                "orchestration"
            }
            .to_owned(),
            kind: if self.lifecycle {
                "lifecycle"
            } else {
                "orchestration"
            },
            start: self.cursor,
            end,
            cpu: ticks.saturating_sub(self.own_ticks) as f64 / self.ticks_per_second,
        });
        self.cursor = end;
        self.own_ticks = ticks;
    }
    fn process(
        &mut self,
        phase: &str,
        start: u64,
        end: u64,
        cpu: f64,
        sink: Option<&sink::Complete>,
        final_cpu: u64,
    ) {
        let ticks = own_ticks();
        let cpu = cpu + ticks.saturating_sub(self.own_ticks) as f64 / self.ticks_per_second;
        if let Some(sink) = sink {
            let exit_cpu = final_cpu.saturating_sub(sink.cpu_ticks) as f64 / self.ticks_per_second;
            self.append(Interval {
                phase: phase.to_owned(),
                kind: "subprocess",
                start,
                end: sink.at,
                cpu: (cpu - exit_cpu).max(0.0),
            });
            self.append(Interval {
                phase: "teardown".to_owned(),
                kind: "lifecycle",
                start: sink.at,
                end,
                cpu: exit_cpu,
            });
            self.lifecycle = true;
        } else {
            self.append(Interval {
                phase: phase.to_owned(),
                kind: "subprocess",
                start,
                end,
                cpu,
            });
        }
        self.cursor = end;
        self.own_ticks = ticks;
    }
}
pub(super) struct Stage {
    pub role: &'static str,
    pub start: u64,
    pub end: u64,
    pub rc: i32,
    pub reason: Option<&'static str>,
    pub sink: Option<sink::Complete>,
    pub sink_error: Option<String>,
}
pub(super) struct Phase {
    pub role: &'static str,
    pub name: String,
    pub wall: u64,
    pub cpu: f64,
    pub start: Option<f64>,
    pub kind: &'static str,
}
struct Group {
    child: Child,
    finished: bool,
    escaped: Vec<(u32, u64)>,
}
impl Group {
    fn kill(&mut self) {
        let pid = self.child.id();
        let _ = Command::new("/bin/kill")
            .args(["-KILL", "--", &format!("-{pid}")])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        for (pid, start) in &self.escaped {
            if procfs::stat(*pid).is_some_and(|p| p.start == *start && p.live()) {
                let _ = Command::new("/bin/kill")
                    .args(["-KILL", "--", &pid.to_string()])
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
        }
        let _ = self.child.kill();
    }
}
impl Drop for Group {
    fn drop(&mut self) {
        if !self.finished {
            self.kill();
            let _ = self.child.wait();
        }
    }
}
fn bytes_under(path: &Path) -> Result<u64> {
    let mut bytes = 0_u64;
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let meta = entry.file_type().map_err(|e| e.to_string())?;
        if meta.is_file() {
            bytes = bytes.saturating_add(entry.metadata().map_err(|e| e.to_string())?.len());
        } else if meta.is_dir() {
            bytes = bytes.saturating_add(bytes_under(&entry.path())?);
        } else {
            return Err(format!(
                "unexpected subprocess product {}",
                entry.path().display()
            ));
        }
    }
    Ok(bytes)
}
pub(super) fn capped(work: &Path, cap: u64) -> Result<bool> {
    Ok(bytes_under(work)? > cap)
}

pub(super) struct Invocation<'a> {
    pub role: &'static str,
    pub command: &'a str,
    pub composition: &'a Path,
    pub query: &'a Path,
    pub work: &'a Path,
}
pub(super) fn spawn(
    config: &Config,
    invocation: Invocation<'_>,
    origin: Instant,
    sampler: &Sampler,
    ledger: &mut Ledger,
    ticks: f64,
) -> Result<Stage> {
    let Invocation {
        role,
        command,
        composition,
        query,
        work,
    } = invocation;
    let stdout = File::create(work.join(format!("{role}.stdout"))).map_err(|e| e.to_string())?;
    let stderr = File::create(work.join(format!("{role}.stderr"))).map_err(|e| e.to_string())?;
    let mut process = Command::new("/bin/sh");
    process
        .args(["-c", &format!("exec {command} \"$@\""), "ckc-bench"])
        .arg("--root")
        .arg(&config.root)
        .arg(composition);
    if role == "orchestrator" {
        process.arg(work).arg(query);
    } else if role == "producer" {
        process.arg(work);
    } else {
        process
            .arg(work.join("model.txt"))
            .arg(work.join("certificate.txt"))
            .arg(query);
    }
    process.env_remove("CKC_BENCH_SINK");
    if role == "orchestrator" {
        process.env("CKC_BENCH_SINK", "closed-file-v1");
    }
    process
        .current_dir(&config.root)
        .stdin(Stdio::null())
        .stdout(stdout)
        .stderr(stderr)
        .process_group(0)
        .env("CKC_BENCH_ROOT", &config.root)
        .env("CKC_BENCH_OUT", work)
        .env("CKC_BENCH_COMPOSITION", composition)
        .env("CKC_BENCH_QUERY", query)
        .env("CKC_BENCH_MEMORY_BYTES", config.memory_bytes.to_string())
        .env("CKC_BENCH_SIZE_BYTES", config.size_bytes.to_string())
        .env(
            "CKC_BENCH_DEADLINE_MS",
            config.deadline.as_millis().to_string(),
        );
    sampler.phase(role);
    let start = ns(origin.elapsed());
    ledger.gap(start);
    let sink_clock = if role == "orchestrator" {
        Some(sink::Clock::capture(origin)?)
    } else {
        None
    };
    trace(work, origin, role, "spawn_begin", None)?;
    let child = process.spawn().map_err(|e| format!("{role}: {e}"))?;
    let mut group = Group {
        child,
        finished: false,
        escaped: Vec::new(),
    };
    let pid = group.child.id();
    trace(work, origin, role, "spawned", Some(pid))?;
    sampler.register(pid)?;
    trace(work, origin, role, "registered", Some(pid))?;
    let mut reason = None;
    let mut cpu_ticks = 0;
    loop {
        if origin.elapsed() >= config.deadline {
            reason = Some("deadline");
        }
        if reason.is_none() {
            reason = sampler.violation(config.memory_bytes);
        }
        if reason.is_none() && capped(work, config.size_bytes)? {
            reason = Some("size");
        }
        let root = procfs::stat(pid).ok_or("subprocess disappeared before reaping")?;
        cpu_ticks = cpu_ticks.max(root.own_ticks.saturating_add(root.children_ticks));
        if !root.live() || reason.is_some() {
            trace(work, origin, role, "root_nonlive_or_limit", Some(pid))?;
            let owned = sampler.boundary()?;
            trace(work, origin, role, "exit_boundary_done", Some(pid))?;
            let related: Vec<_> = owned
                .into_iter()
                .filter(|p| p.pid != std::process::id() && p.pid != pid)
                .collect();
            group.escaped = related.iter().map(|p| p.key()).collect();
            if reason.is_none() {
                reason = sampler.violation(config.memory_bytes);
            }
            if reason.is_some() {
                group.kill();
                let cleanup = Instant::now();
                loop {
                    let root_live = procfs::stat(pid).is_some_and(|p| p.live());
                    let descendant_live = group.escaped.iter().any(|(pid, start)| {
                        procfs::stat(*pid).is_some_and(|p| p.start == *start && p.live())
                    });
                    if !root_live && !descendant_live {
                        break;
                    }
                    if cleanup.elapsed() >= Duration::from_secs(2) {
                        return Err("owned process still live after the group kill".to_owned());
                    }
                    std::thread::sleep(Duration::from_millis(2));
                }
                break;
            }
            if related.iter().all(|p| !p.live()) {
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    if let Some(root) = procfs::stat(pid) {
        cpu_ticks = cpu_ticks.max(root.own_ticks.saturating_add(root.children_ticks));
    }
    trace(work, origin, role, "wait_begin", Some(pid))?;
    let status = group.child.wait().map_err(|e| e.to_string())?;
    group.finished = true;
    trace(work, origin, role, "wait_done", None)?;
    sampler.unregister(pid);
    sampler.boundary()?;
    trace(work, origin, role, "post_wait_boundary_done", None)?;
    let (sink, sink_error) = if status.success()
        && reason.is_none()
        && let Some(clock) = sink_clock
    {
        match clock.complete(work, origin, start, ns(origin.elapsed()), cpu_ticks) {
            Ok(sink) => (Some(sink), None),
            Err(error) => (None, Some(error)),
        }
    } else {
        (None, None)
    };
    let end = ns(origin.elapsed());
    ledger.process(
        role,
        start,
        end,
        cpu_ticks as f64 / ticks,
        sink.as_ref(),
        cpu_ticks,
    );
    sampler.phase("orchestration");
    Ok(Stage {
        role,
        start,
        end,
        rc: status
            .code()
            .unwrap_or_else(|| -status.signal().unwrap_or(1)),
        reason,
        sink,
        sink_error,
    })
}

fn seconds(text: &str) -> Result<f64> {
    let seconds = text.parse::<f64>().map_err(|_| "invalid phase seconds")?;
    if !seconds.is_finite() || seconds < 0.0 || seconds > u64::MAX as f64 / 1e9 {
        return Err("phase seconds out of range".to_owned());
    }
    Ok(seconds)
}
pub(super) fn phases(work: &Path, stage: &Stage) -> Result<Vec<Phase>> {
    let path = work.join(format!("{}-phases.tsv", stage.role));
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut lines = text.lines();
    let (offset, kinds) = match lines.next() {
        Some("phase\tseconds\tcpu") if stage.role != "orchestrator" => (false, false),
        Some("phase\tseconds\tcpu\tstart_seconds") if stage.role != "orchestrator" => (true, false),
        Some("phase\tseconds\tcpu\tstart_seconds\tkind")
            if matches!(stage.role, "orchestrator" | "call" | "load") =>
        {
            (true, true)
        }
        _ => return Err("phase report header".to_owned()),
    };
    let mut rows = Vec::new();
    let mut names = std::collections::BTreeSet::new();
    for line in lines {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len()
            != if kinds {
                5
            } else if offset {
                4
            } else {
                3
            }
            || fields[0].is_empty()
            || !names.insert(fields[0])
        {
            return Err("phase report row".to_owned());
        }
        rows.push(Phase {
            role: stage.role,
            name: fields[0].to_owned(),
            wall: (seconds(fields[1])? * 1e9).round() as u64,
            cpu: seconds(fields[2])?,
            start: if offset {
                Some(seconds(fields[3])?)
            } else {
                None
            },
            kind: if kinds {
                match fields[4] {
                    "load" if fields[0].starts_with("load_") => "load",
                    "scored" if !fields[0].starts_with("load_") => "scored",
                    _ => return Err("phase report load/scored boundary".to_owned()),
                }
            } else if fields[0] == "load" {
                "load"
            } else {
                "scored"
            },
        });
    }
    // Warm mode splits the orchestrator table: `load` once per server, `call` per request.
    const SCORED: [&str; 9] = [
        "patient",
        "slice",
        "materialize",
        "producer",
        "decode",
        "index",
        "support",
        "closure",
        "answers",
    ];
    let expected: &[&str] = if stage.role == "call" {
        &SCORED
    } else if stage.role == "load" {
        &[
            "load_parse",
            "load_admission",
            "load_analysis",
            "load_index",
            "load_receipt",
        ]
    } else if stage.role == "orchestrator" {
        &[
            "load_parse",
            "load_admission",
            "load_analysis",
            "load_index",
            "patient",
            "slice",
            "materialize",
            "producer",
            "decode",
            "index",
            "support",
            "closure",
            "answers",
        ]
    } else if stage.role == "producer" {
        &["load", "eval", "certificate", "write"]
    } else {
        &[
            "load",
            "decode",
            "admission",
            "index",
            "support",
            "closure",
            "answers",
        ]
    };
    if rows.len() != expected.len()
        || rows
            .iter()
            .zip(expected)
            .any(|(row, name)| row.name != *name)
    {
        return Err("phase report does not name every phase in order".to_owned());
    }
    let duration = stage.sink.as_ref().map_or(stage.end, |sink| sink.at) - stage.start;
    let total = rows
        .iter()
        .try_fold(0_u64, |sum, row| sum.checked_add(row.wall))
        .ok_or("phase duration overflow")?;
    if total > duration {
        return Err("phase reports exceed the measured subprocess interval".to_owned());
    }
    if offset {
        let mut previous = 0.0;
        for row in &rows {
            let start = row.start.unwrap_or_default();
            let end = start + row.wall as f64 / 1e9;
            if start < previous || end > duration as f64 / 1e9 {
                return Err("phase offsets overlap or exceed the subprocess interval".to_owned());
            }
            previous = end;
        }
    }
    Ok(rows)
}
