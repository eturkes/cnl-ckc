// Contract sb D1/D6 + § Carried rulings (warm multi-call): one server per arm prepares its
// knowledge base once (load: reported, never scored); a call is scored from its request
// write to its closed answer sink; RSS is sampled across the whole campaign, load
// included, and flushed per window (each call, the idle gaps, the loads) so the harness
// never holds the campaign's samples.
use super::config::Config;
use super::execute::{self, Interval, Ledger, Phase, Stage};
use super::procfs::{self, Sampler, State};
use super::{Plan, Result, RunRow, field, ns, private_directory, report, sink};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};

// Bulky per-call products: hashed into products.tsv, removed unless --keep-products yes.
const PRODUCTS: [&str; 4] = [
    "model.txt",
    "certificate.txt",
    "slice.pl.in",
    "selected-clauses.tsv",
];

struct Server {
    arm: &'static str,
    child: Child,
    stdin: Option<ChildStdin>,
    lines: Receiver<String>,
    lost: Option<String>,
}
impl Server {
    fn pid(&self) -> u32 {
        self.child.id()
    }
    fn ticks(&self) -> u64 {
        procfs::stat(self.pid()).map_or(0, |p| p.own_ticks.saturating_add(p.children_ticks))
    }
    // A lost server stays lost: its later calls read indeterminate, never a hidden reload.
    fn lose(&mut self, reason: &str) {
        let _ = Command::new("/bin/kill")
            .args(["-KILL", "--", &format!("-{}", self.pid())])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.stdin = None;
        self.lost.get_or_insert_with(|| reason.to_owned());
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        if matches!(self.child.try_wait(), Ok(None)) {
            self.lose("dropped");
        }
    }
}
fn spawn(
    config: &Config,
    arm: &'static str,
    composition: &Path,
    sampler: &Sampler,
) -> Result<Server> {
    let command = config.serve.as_deref().ok_or("serve command")?;
    let stderr =
        File::create(config.out.join(format!("arm-{arm}.stderr"))).map_err(|e| e.to_string())?;
    let mut child = Command::new("/bin/sh")
        .args(["-c", &format!("exec {command} \"$@\""), "ckc-bench"])
        .arg("--root")
        .arg(&config.root)
        .arg(composition)
        .arg(config.out.join(format!("arm-{arm}")))
        .env_remove("CKC_BENCH_SINK")
        .env("CKC_BENCH_ROOT", &config.root)
        .env("CKC_BENCH_COMPOSITION", composition)
        .env("CKC_BENCH_MEMORY_BYTES", config.memory_bytes.to_string())
        .env("CKC_BENCH_SIZE_BYTES", config.size_bytes.to_string())
        .env(
            "CKC_BENCH_DEADLINE_MS",
            config.deadline.as_millis().to_string(),
        )
        .current_dir(&config.root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(stderr)
        .process_group(0)
        .spawn()
        .map_err(|e| format!("serve {arm}: {e}"))?;
    let stdout = child.stdout.take().ok_or("serve stdout")?;
    let stdin = child.stdin.take();
    let (send, lines) = mpsc::channel();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            if send.send(line).is_err() {
                break;
            }
        }
    });
    let server = Server {
        arm,
        child,
        stdin,
        lines,
        lost: None,
    };
    sampler.register(server.pid())?;
    Ok(server)
}
// The server's next stdout line before `until`; the memory + size caps poll meanwhile.
fn line(
    server: &Server,
    until: Instant,
    sampler: &Sampler,
    config: &Config,
    work: Option<&Path>,
) -> std::result::Result<String, &'static str> {
    loop {
        if let Some(reason) = sampler.violation(config.memory_bytes) {
            return Err(reason);
        }
        if work.is_some_and(|dir| {
            dir.is_dir() && execute::capped(dir, config.size_bytes).unwrap_or(true)
        }) {
            return Err("size");
        }
        let now = Instant::now();
        if now >= until {
            return Err("deadline");
        }
        match server
            .lines
            .recv_timeout((until - now).min(Duration::from_millis(20)))
        {
            Ok(line) => return Ok(line),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return Err("server-exit"),
        }
    }
}
// Contract sb P8: a producer child that lives between two RSS samples leaves no reading, so the
// orchestrator reports its exact wait4 peak; bound = Σ HWM of every other observed process + that
// peak ≥ the call's simultaneous peak (one producer per call). None = no producer report.
fn memory_bound(directory: &Path, window: &State) -> Option<u64> {
    let text = fs::read_to_string(directory.join("checker-counters.tsv")).ok()?;
    let value = |name: &str| {
        let prefix = format!("runtime\t{name}\t");
        text.lines()
            .find_map(|l| l.strip_prefix(prefix.as_str())?.parse::<u64>().ok())
    };
    let (pid, peak) = (value("producer_pid")?, value("producer_maxrss_bytes")?);
    let others: u64 = window
        .processes
        .values()
        .filter(|p| u64::from(p.process.pid) != pid)
        .map(|p| p.hwm)
        .sum();
    Some(others.saturating_add(peak))
}
// Call-relative window: every timestamp minus the call's request time.
fn rebase(mut window: State, start: u64) -> State {
    for sample in &mut window.samples {
        sample.start = sample.start.saturating_sub(start);
        sample.end = sample.end.saturating_sub(start);
    }
    for seen in window.processes.values_mut() {
        seen.first = seen.first.saturating_sub(start);
        seen.last = seen.last.saturating_sub(start);
    }
    window
}
fn append_samples(path: &Path, label: &str, window: &State) -> Result<()> {
    let fresh = !path.exists();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    if fresh {
        writeln!(
            file,
            "window\tstart_ns\tend_ns\tphase\trss_bytes\tprocess_count"
        )
        .map_err(|e| e.to_string())?;
    }
    for s in &window.samples {
        writeln!(
            file,
            "{label}\t{}\t{}\t{}\t{}\t{}",
            s.start,
            s.end,
            s.phase,
            s.rss,
            s.processes.len()
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn products(directory: &Path, keep: bool) -> Result<()> {
    let mut text = "file\tbytes\tsha256\tkept\n".to_owned();
    for name in PRODUCTS {
        let path = directory.join(name);
        if !path.is_file() {
            continue;
        }
        let (size, hash) = report::digest(&path)?;
        text.push_str(&format!(
            "{name}\t{size}\t{hash}\t{}\n",
            if keep { "yes" } else { "no" }
        ));
        if !keep {
            fs::remove_file(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    report::write(&directory.join("products.tsv"), text)
}
fn empty_row(entry: &Plan) -> RunRow {
    RunRow {
        run: entry.run,
        query: super::query_name(entry),
        block: entry.block,
        order: entry.order,
        arm: entry.arm,
        outcome: "verified",
        reason: String::new(),
        wall: 0,
        load: 0,
        scored: None,
        sink_end: None,
        teardown: 0,
        sink_uncertainty: None,
        cpu: 0.0,
        peak_rss: 0,
        hwm_sum: 0,
        memory_bound: None,
        producer_rc: None,
        checker_rc: None,
        output_bytes: None,
        output_hash: None,
    }
}
struct Call {
    row: RunRow,
    ledger: Ledger,
    phases: Vec<Phase>,
    window: State,
    detail: String,
}
fn call(
    config: &Config,
    entry: &Plan,
    server: &mut Server,
    directory: &Path,
    origin: Instant,
    sampler: &Sampler,
    ticks: f64,
) -> Result<Call> {
    let mut row = empty_row(entry);
    let mut ledger = Ledger::new(ticks);
    let mut phases = Vec::new();
    let mut detail = String::new();
    if let Some(lost) = &server.lost {
        row.outcome = "indeterminate";
        row.reason = format!("server-lost:{lost}");
        private_directory(directory)?;
        return Ok(Call {
            row,
            ledger,
            phases,
            window: sampler.drain(),
            detail,
        });
    }
    sampler.phase("call");
    let clock = sink::Clock::capture(origin)?;
    let cpu_before = server.ticks();
    let start = ns(origin.elapsed());
    let request = format!(
        "{}\t{}\t{}\n",
        entry.run,
        entry.query.display(),
        directory.display()
    );
    let written = server
        .stdin
        .as_mut()
        .ok_or("serve stdin")
        .and_then(|stdin| {
            stdin
                .write_all(request.as_bytes())
                .and_then(|_| stdin.flush())
                .map_err(|_| "serve stdin")
        });
    let answered = match written {
        Ok(()) => line(
            server,
            Instant::now() + config.deadline,
            sampler,
            config,
            Some(directory),
        ),
        Err(_) => Err("server-exit"),
    };
    let end = ns(origin.elapsed());
    let cpu_after = server.ticks();
    row.wall = end - start;
    row.cpu = cpu_after.saturating_sub(cpu_before) as f64 / ticks;
    let mut sink_at = None;
    match answered {
        Err(reason) => {
            server.lose(reason);
            row.outcome = "indeterminate";
            row.reason = reason.to_owned();
        }
        Ok(text) => {
            let fields: Vec<_> = text.split('\t').collect();
            let rc = match fields.as_slice() {
                ["done", id, rc] if *id == entry.run.to_string() => rc.parse::<i32>().ok(),
                _ => None,
            };
            match rc {
                None => {
                    server.lose("protocol");
                    row.outcome = "indeterminate";
                    row.reason = "protocol".to_owned();
                    detail = text;
                }
                Some(0) => match clock.complete(directory, origin, start, end, cpu_after) {
                    Ok(sink) => {
                        row.checker_rc = Some(0);
                        row.sink_end = Some(sink.at - start);
                        row.sink_uncertainty = Some(sink.uncertainty);
                        row.scored = Some(sink.at - start);
                        row.teardown = end - sink.at;
                        row.output_bytes = Some(sink.bytes);
                        row.output_hash = Some(sink.sha256.clone());
                        sink_at = Some(sink.at);
                        let stage = Stage {
                            role: "call",
                            start,
                            end,
                            rc: 0,
                            reason: None,
                            sink: Some(sink),
                            sink_error: None,
                        };
                        match execute::phases(directory, &stage) {
                            Ok(reported) => phases = reported,
                            Err(error) => {
                                row.outcome = "indeterminate";
                                row.reason = "phase-report".to_owned();
                                detail = error;
                            }
                        }
                    }
                    Err(error) => {
                        row.checker_rc = Some(0);
                        row.outcome = "indeterminate";
                        row.reason = "sink-report".to_owned();
                        detail = error;
                    }
                },
                Some(rc) => {
                    row.checker_rc = Some(rc);
                    row.outcome = if rc == 1 { "refused" } else { "indeterminate" };
                    row.reason = "orchestrator".to_owned();
                }
            }
        }
    }
    let scored_end = sink_at.unwrap_or(end) - start;
    ledger.intervals.push(Interval {
        phase: "call".to_owned(),
        kind: "subprocess",
        start: 0,
        end: scored_end,
        cpu: row.cpu,
    });
    if scored_end < end - start {
        ledger.intervals.push(Interval {
            phase: "done-report".to_owned(),
            kind: "lifecycle",
            start: scored_end,
            end: end - start,
            cpu: 0.0,
        });
    }
    ledger.cursor = end - start;
    sampler.phase("idle");
    if !directory.is_dir() {
        private_directory(directory)?;
    }
    if row.outcome == "verified" {
        products(directory, config.keep_products)?;
    } else {
        row.output_bytes = None;
        row.output_hash = None;
        row.scored = None;
    }
    let window = rebase(sampler.drain(), start);
    row.peak_rss = window.peak_rss;
    row.hwm_sum = window.processes.values().map(|p| p.hwm).sum();
    row.memory_bound = memory_bound(directory, &window);
    Ok(Call {
        row,
        ledger,
        phases,
        window,
        detail,
    })
}
// Every arm's server: target first, then the paired baseline.
fn load(config: &Config, origin: Instant, sampler: &Sampler) -> Result<(Vec<Server>, Vec<Phase>)> {
    let mut servers = Vec::new();
    let mut phases = Vec::new();
    // `rss_at_ready_bytes` = the arm's server group at its ready line: the resident
    // prepared knowledge base the record adds the census receipt's VmHWM to.
    let mut table =
        "arm\tstart_ns\tready_ns\tload_ns\trss_at_ready_bytes\toutcome\tdetail\n".to_owned();
    let arms = std::iter::once(("target", &config.target))
        .chain(config.paired.iter().map(|dir| ("baseline", dir)));
    for (arm, dir) in arms {
        sampler.phase(&format!("load-{arm}"));
        let start = ns(origin.elapsed());
        let mut server = spawn(config, arm, &dir.join("composition.tsv"), sampler)?;
        let answered = line(
            &server,
            Instant::now() + config.load_deadline,
            sampler,
            config,
            None,
        );
        let ready = ns(origin.elapsed());
        let resident: u64 = sampler
            .boundary()?
            .iter()
            .filter(|p| p.group == server.pid())
            .map(|p| p.rss)
            .sum();
        let (outcome, detail) = match answered {
            Ok(text) if text == "ready" => {
                let stage = Stage {
                    role: "load",
                    start,
                    end: ready,
                    rc: 0,
                    reason: None,
                    sink: None,
                    sink_error: None,
                };
                match execute::phases(&config.out.join(format!("arm-{arm}")), &stage) {
                    Ok(reported) => {
                        phases.extend(reported);
                        ("ready", String::new())
                    }
                    Err(error) => ("indeterminate", format!("phase-report: {error}")),
                }
            }
            Ok(text) => ("indeterminate", format!("protocol: {text}")),
            Err(reason) => ("indeterminate", reason.to_owned()),
        };
        if outcome != "ready" {
            server.lose(&format!("load-{}", detail.split(':').next().unwrap_or("")));
        }
        table.push_str(&format!(
            "{arm}\t{start}\t{ready}\t{}\t{resident}\t{outcome}\t{}\n",
            ready - start,
            field(&detail)
        ));
        servers.push(server);
    }
    sampler.phase("idle");
    report::write(&config.out.join("loads.tsv"), table)?;
    Ok((servers, phases))
}
pub(super) fn campaign(config: &Config, plan: &[Plan]) -> Result<Vec<RunRow>> {
    let ticks = report::ticks()?;
    private_directory(&config.out)?;
    report::metadata(config, plan, ticks)?;
    let origin = Instant::now();
    let sampler = Sampler::start(origin, config.period)?;
    let (mut servers, load_phases) = load(config, origin, &sampler)?;
    let load_window = config.out.join("load");
    private_directory(&load_window)?;
    report::per_run(
        &load_window,
        &Ledger::new(ticks),
        &load_phases,
        &sampler.drain(),
    )?;
    let mut raw = File::create(config.out.join("runs.tsv")).map_err(|e| e.to_string())?;
    raw.write_all(report::RUN_HEADER.as_bytes())
        .map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    let idle = config.out.join("idle-rss.tsv");
    // Block b's first call waits for ready + (b − 1) × spacing (original block numbers under a
    // sparse replay); every wait = idle time outside the calls, recorded in schedule.tsv.
    let ready = ns(origin.elapsed());
    let mut schedule = "block\tplanned_ns\tstarted_ns\twaited_ns\n".to_owned();
    let mut current = None;
    for entry in plan {
        if current != Some(entry.block) {
            current = Some(entry.block);
            let planned = ready.saturating_add(ns(config.spacing).saturating_mul(entry.block - 1));
            let now = ns(origin.elapsed());
            if planned > now {
                std::thread::sleep(Duration::from_nanos(planned - now));
            }
            let started = ns(origin.elapsed());
            schedule.push_str(&format!(
                "{}\t{planned}\t{started}\t{}\n",
                entry.block,
                started - now
            ));
        }
        append_samples(
            &idle,
            &format!("before-run-{}", entry.run),
            &sampler.drain(),
        )?;
        let directory = config.out.join(format!("run-{:06}", entry.run));
        let server = servers
            .iter_mut()
            .find(|s| s.arm == entry.arm)
            .ok_or("plan arm without a server")?;
        let Call {
            row,
            ledger,
            phases,
            window,
            detail,
        } = call(config, entry, server, &directory, origin, &sampler, ticks)?;
        report::per_run(&directory, &ledger, &phases, &window)?;
        if !detail.is_empty() {
            report::write(
                &directory.join("diagnostic.txt"),
                format!("{}\n", field(&detail)),
            )?;
        }
        raw.write_all(report::run_row(&row, &ledger, &window, ns(config.period)).as_bytes())
            .map_err(|e| e.to_string())?;
        raw.flush().map_err(|e| e.to_string())?;
        rows.push(row);
    }
    report::write(&config.out.join("schedule.tsv"), schedule)?;
    // Teardown: EOF on every stdin; a server still alive after the grace period is killed.
    let mut teardown = "arm\tstart_ns\tend_ns\toutcome\n".to_owned();
    for server in &mut servers {
        let start = ns(origin.elapsed());
        server.stdin = None;
        let grace = Instant::now() + Duration::from_secs(120);
        let outcome = loop {
            match server.child.try_wait() {
                Ok(Some(status)) => break format!("exit {status}"),
                Ok(None) if Instant::now() >= grace => {
                    server.lose("teardown-deadline");
                    break "killed".to_owned();
                }
                Ok(None) => std::thread::sleep(Duration::from_millis(10)),
                Err(error) => break format!("wait {error}"),
            }
        };
        teardown.push_str(&format!(
            "{}\t{start}\t{}\t{}\n",
            server.arm,
            ns(origin.elapsed()),
            field(&outcome)
        ));
    }
    report::write(&config.out.join("teardown.tsv"), teardown)?;
    let state = sampler.finish();
    append_samples(&idle, "teardown", &state)?;
    // D1 memory: the sampled maximum of summed live RSS over the owned process tree,
    // whole campaign (loads + calls + teardown).
    report::write(
        &config.out.join("memory.tsv"),
        format!(
            "key\tvalue\npeak_summed_rss_bytes\t{}\npeak_single_hwm_bytes\t{}\nsampler_error\t{}\n",
            state.peak_rss,
            state.peak_single_hwm,
            field(state.error.as_deref().unwrap_or("none"))
        ),
    )?;
    Ok(rows)
}
