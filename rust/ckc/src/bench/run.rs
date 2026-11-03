// Contract `sb` D1/D6/P3: measurement shell; outcomes name benchmark results only.
mod config;
mod execute;
mod procfs;
mod report;
mod sink;

use config::{Config, Random};
use execute::{Invocation, Ledger, Phase};
use procfs::{Sampler, State};
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, String>;
fn ns(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}
fn field(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace('\n', "\\n")
}
struct Plan {
    run: usize,
    block: u64,
    order: usize,
    arm: &'static str,
    directory: PathBuf,
    composition: PathBuf,
    query: PathBuf,
}
struct RunRow {
    run: usize,
    block: u64,
    order: usize,
    arm: &'static str,
    outcome: &'static str,
    reason: String,
    wall: u64,
    load: u64,
    scored: Option<u64>,
    sink_end: Option<u64>,
    teardown: u64,
    sink_uncertainty: Option<u64>,
    cpu: f64,
    peak_rss: u64,
    hwm_sum: u64,
    producer_rc: Option<i32>,
    checker_rc: Option<i32>,
    output_bytes: Option<u64>,
    output_hash: Option<String>,
}
fn plan(config: &Config) -> Result<Vec<Plan>> {
    let mut random = Random::new(config.seed);
    let mut plan = Vec::new();
    for block in 1..=config.blocks {
        let mut arms = vec![("target", config.target.clone())];
        if let Some(baseline) = &config.paired {
            arms.push(("baseline", baseline.clone()));
            if random.below(2) == 1 {
                arms.swap(0, 1);
            }
        }
        for (order, (arm, directory)) in arms.into_iter().enumerate() {
            let composition = directory
                .join("composition.tsv")
                .canonicalize()
                .map_err(|e| format!("{}: {e}", directory.display()))?;
            let query = config.query(&directory)?;
            if !composition.is_file() || !query.is_file() {
                return Err("composition + query must be regular files".to_owned());
            }
            plan.push(Plan {
                run: plan.len() + 1,
                block,
                order: order + 1,
                arm,
                directory,
                composition,
                query,
            });
        }
    }
    Ok(plan)
}
struct Work(PathBuf);
impl Drop for Work {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn private_directory(path: &Path) -> Result<()> {
    fs::DirBuilder::new()
        .mode(0o700)
        .create(path)
        .map_err(|e| format!("{}: {e}", path.display()))
}
fn publish(work: &Path, destination: &Path, verified: bool) -> Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(work)
        .map_err(|e| e.to_string())?
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|e| e.to_string())?;
    entries.retain(|entry| verified || entry.file_name().to_string_lossy().ends_with(".stderr"));
    entries.sort_by_key(|entry| (entry.file_name() == "answers.txt", entry.file_name()));
    for entry in &entries {
        let path = destination.join(entry.file_name());
        if path.exists() {
            return Err(format!(
                "subprocess product collides with {}",
                path.display()
            ));
        }
    }
    let mut published: Vec<PathBuf> = Vec::new();
    let result = (|| {
        for entry in entries {
            let path = destination.join(entry.file_name());
            fs::rename(entry.path(), &path).map_err(|e| e.to_string())?;
            published.push(path);
        }
        fs::remove_dir_all(work).map_err(|e| e.to_string())
    })();
    if let Err(error) = result {
        for path in &published {
            if path.is_dir() {
                let _ = fs::remove_dir_all(path);
            } else {
                let _ = fs::remove_file(path);
            }
        }
        return Err(error);
    }
    Ok(published)
}
fn run_one(config: &Config, entry: &Plan, ticks: f64) -> Result<(RunRow, Ledger, State)> {
    let origin = Instant::now();
    let mut ledger = Ledger::new(ticks);
    let directory = config.out.join(format!("run-{:06}", entry.run));
    private_directory(&directory)?;
    let work = Work(directory.join(".work"));
    private_directory(&work.0)?;
    let sampler = Sampler::start(origin, config.period)?;
    let mut phases: Vec<Phase> = Vec::new();
    let mut row = RunRow {
        run: entry.run,
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
        producer_rc: None,
        checker_rc: None,
        output_bytes: None,
        output_hash: None,
    };
    let mut detail = String::new();
    let mut closed_sink = None;
    let stages = if let Some(command) = &config.orchestrator {
        vec![("orchestrator", command)]
    } else {
        vec![("producer", &config.producer), ("checker", &config.checker)]
    };
    for (role, command) in stages {
        if origin.elapsed() >= config.deadline {
            row.outcome = "indeterminate";
            row.reason = "deadline".to_owned();
            break;
        }
        let invocation = Invocation {
            role,
            command,
            composition: &entry.composition,
            query: &entry.query,
            work: &work.0,
        };
        let stage = match execute::spawn(config, invocation, origin, &sampler, &mut ledger, ticks) {
            Ok(stage) => stage,
            Err(error) => {
                row.outcome = "indeterminate";
                row.reason = role.to_owned();
                detail = error;
                break;
            }
        };
        if role == "producer" {
            row.producer_rc = Some(stage.rc);
        } else {
            row.checker_rc = Some(stage.rc);
        }
        if let Some(reason) = stage.reason {
            row.outcome = "indeterminate";
            row.reason = reason.to_owned();
            break;
        }
        if stage.rc != 0 {
            row.outcome = if matches!(role, "checker" | "orchestrator") && stage.rc == 1 {
                "refused"
            } else {
                "indeterminate"
            };
            row.reason = role.to_owned();
            break;
        }
        if let Some(error) = &stage.sink_error {
            row.outcome = "indeterminate";
            row.reason = "sink-report".to_owned();
            detail = error.clone();
            break;
        }
        if let Some(sink) = &stage.sink {
            row.sink_end = Some(sink.at);
            row.sink_uncertainty = Some(sink.uncertainty);
            closed_sink = Some((sink.bytes, sink.sha256.clone()));
        }
        match execute::phases(&work.0, &stage) {
            Ok(reported) => {
                row.load = row
                    .load
                    .checked_add(
                        reported
                            .iter()
                            .filter(|p| p.kind == "load")
                            .map(|p| p.wall)
                            .sum::<u64>(),
                    )
                    .ok_or("load duration overflow")?;
                phases.extend(reported);
            }
            Err(error) => {
                row.outcome = "indeterminate";
                row.reason = "phase-report".to_owned();
                detail = error;
                break;
            }
        }
        if matches!(role, "producer" | "orchestrator")
            && ["model.txt", "certificate.txt"]
                .iter()
                .any(|name| !work.0.join(name).is_file())
        {
            row.outcome = "indeterminate";
            row.reason = "producer-output".to_owned();
            break;
        }
    }
    if row.outcome == "verified" {
        execute::trace(
            &work.0,
            origin,
            "controller",
            "answer_sink_digest_begin",
            None,
        )?;
        let stdout = work.0.join("checker.stdout");
        let (size, hash) = if let Some(sink) = closed_sink {
            sink
        } else {
            report::digest(&stdout)?
        };
        row.output_bytes = Some(size);
        row.output_hash = Some(hash);
        let certificate_size = fs::metadata(work.0.join("certificate.txt"))
            .map_err(|e| e.to_string())?
            .len();
        let model_facts = report::model_lines(&work.0.join("model.txt"))?;
        let processes = if config.orchestrator.is_some() { 1 } else { 2 };
        let counter = format!(
            "kind\tname\tvalue\nlogical\tmodel_lines\t{model_facts}\nlogical\tcertificate_bytes\t{certificate_size}\nlogical\toutput_bytes\t{size}\nruntime\tphase_processes\t{processes}\n"
        );
        report::write(&work.0.join("harness-counters.tsv"), counter)?;
        if config.orchestrator.is_none() {
            fs::rename(stdout, work.0.join("answers.txt")).map_err(|e| e.to_string())?;
        }
        execute::trace(&work.0, origin, "controller", "answer_sink_renamed", None)?;
    }
    if execute::capped(&work.0, config.size_bytes)? {
        row.outcome = "indeterminate";
        row.reason = "size".to_owned();
    }
    if origin.elapsed() >= config.deadline {
        row.outcome = "indeterminate";
        row.reason = "deadline".to_owned();
    }
    execute::trace(&work.0, origin, "controller", "sampler_finish_begin", None)?;
    let state = sampler.finish();
    execute::trace(&work.0, origin, "controller", "sampler_finish_end", None)?;
    if let Some(error) = &state.error {
        row.outcome = "indeterminate";
        row.reason = "sampler".to_owned();
        detail = error.clone();
    } else if state.peak_rss > config.memory_bytes || state.peak_single_hwm > config.memory_bytes {
        row.outcome = "indeterminate";
        row.reason = "memory".to_owned();
    }
    execute::trace(&work.0, origin, "controller", "publish_begin", None)?;
    let published = publish(&work.0, &directory, row.outcome == "verified")?;
    execute::trace(&work.0, origin, "controller", "publish_end", None)?;
    if origin.elapsed() >= config.deadline {
        row.outcome = "indeterminate";
        row.reason = "deadline".to_owned();
        for path in published {
            if !path
                .file_name()
                .is_some_and(|n| n.to_string_lossy().ends_with(".stderr"))
            {
                if path.is_dir() {
                    fs::remove_dir_all(path)
                } else {
                    fs::remove_file(path)
                }
                .map_err(|e| e.to_string())?;
            }
        }
    }
    execute::trace(&work.0, origin, "controller", "scored_wall_end", None)?;
    row.wall = ns(origin.elapsed());
    ledger.gap(row.wall);
    row.teardown = row.sink_end.map_or(0, |sink| row.wall.saturating_sub(sink));
    row.cpu = ledger.intervals.iter().map(|r| r.cpu).sum();
    row.peak_rss = state.peak_rss;
    row.hwm_sum = state.processes.values().map(|r| r.hwm).sum();
    if row.outcome == "verified" {
        row.scored = Some(
            row.sink_end
                .unwrap_or(row.wall)
                .checked_sub(row.load)
                .ok_or("load exceeds complete sink endpoint")?,
        );
    } else {
        row.output_bytes = None;
        row.output_hash = None;
    }
    report::per_run(&directory, &ledger, &phases, &state)?;
    if !detail.is_empty() {
        report::write(
            &directory.join("diagnostic.txt"),
            format!("{}\n", field(&detail)),
        )?;
    }
    Ok((row, ledger, state))
}
fn campaign(config: Config) -> Result<bool> {
    let plan = plan(&config)?;
    let ticks = report::ticks()?;
    private_directory(&config.out)?;
    report::metadata(&config, &plan, ticks)?;
    let mut raw = File::create(config.out.join("runs.tsv")).map_err(|e| e.to_string())?;
    raw.write_all(report::RUN_HEADER.as_bytes())
        .map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    for entry in &plan {
        let (row, ledger, state) = run_one(&config, entry, ticks)?;
        raw.write_all(report::run_row(&row, &ledger, &state, ns(config.period)).as_bytes())
            .map_err(|e| e.to_string())?;
        raw.flush().map_err(|e| e.to_string())?;
        rows.push(row);
    }
    report::summary(&config, &rows)?;
    let verified = rows.iter().filter(|r| r.outcome == "verified").count();
    let refused = rows.iter().filter(|r| r.outcome == "refused").count();
    eprintln!(
        "ckc: bench: {verified} verified, {refused} refused, {} indeterminate; {}",
        rows.len() - verified - refused,
        config.out.display()
    );
    Ok(verified == rows.len())
}
pub fn run(args: &[String]) -> ExitCode {
    match Config::parse(args).and_then(campaign) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => {
            eprintln!("ckc: bench: {}", field(&error));
            ExitCode::from(2)
        }
    }
}
