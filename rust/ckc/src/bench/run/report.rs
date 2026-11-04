use super::config::{Config, Random};
use super::execute::{Ledger, Phase};
use super::procfs::State;
use super::{Plan, Result, RunRow, field, ns};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) fn write(path: &Path, bytes: impl AsRef<[u8]>) -> Result<()> {
    fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}
pub(super) fn digest(path: &Path) -> Result<(u64, String)> {
    let mut input = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut hash = Sha256::new();
    let mut size = 0_u64;
    let mut buffer = [0_u8; 65536];
    loop {
        let read = input.read(&mut buffer).map_err(|e| e.to_string())?;
        if read == 0 {
            break;
        }
        size = size.checked_add(read as u64).ok_or("file size overflow")?;
        hash.update(&buffer[..read]);
    }
    Ok((
        size,
        hash.finalize().iter().map(|b| format!("{b:02x}")).collect(),
    ))
}
// One digest per executable file per harness process, keyed by its identity + length +
// mtime: re-hashing every executable after every run (the 63 MB unoptimized test ckc +
// a 34 MB interpreter, ≈ 9 s per run) pushed the paired harness test past its 30 s wall.
fn executable_digest(path: &Path) -> Result<(u64, String)> {
    use std::collections::BTreeMap;
    use std::os::unix::fs::MetadataExt;
    use std::sync::{Mutex, OnceLock};
    type Key = (PathBuf, u64, u64, u64, i64, i64);
    static CACHE: OnceLock<Mutex<BTreeMap<Key, (u64, String)>>> = OnceLock::new();
    let m = fs::metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let key = (
        path.to_owned(),
        m.dev(),
        m.ino(),
        m.len(),
        m.mtime(),
        m.mtime_nsec(),
    );
    let cache = CACHE.get_or_init(Default::default);
    if let Some(hit) = cache.lock().unwrap_or_else(|e| e.into_inner()).get(&key) {
        return Ok(hit.clone());
    }
    let value = digest(path)?;
    cache
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(key, value.clone());
    Ok(value)
}
fn hash_text(text: &str) -> String {
    crate::trust::sha256_hex(text.as_bytes())
}
fn pair(text: &mut String, key: &str, value: impl AsRef<str>) {
    text.push_str(&format!("{}\t{}\n", field(key), field(value.as_ref())));
}
fn read(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path)
        .map(|text| text.trim().to_owned())
        .unwrap_or_else(|_| "unreadable".to_owned())
}
pub(super) fn ticks() -> Result<f64> {
    let result = Command::new("getconf")
        .arg("CLK_TCK")
        .output()
        .map_err(|e| format!("getconf: {e}"))?;
    if !result.status.success() {
        return Err("getconf CLK_TCK failed".to_owned());
    }
    let ticks = String::from_utf8_lossy(&result.stdout)
        .trim()
        .parse::<f64>()
        .map_err(|_| "invalid CLK_TCK")?;
    if !ticks.is_finite() || ticks <= 0.0 {
        return Err("invalid CLK_TCK".to_owned());
    }
    Ok(ticks)
}
pub(super) fn metadata(config: &Config, plan: &[Plan], ticks: f64) -> Result<()> {
    let mut settings = "key\tvalue\n".to_owned();
    for (key, value) in [
        ("format", "sb1-harness-2".to_owned()),
        ("root", config.root.display().to_string()),
        ("producer", config.producer.clone()), ("checker", config.checker.clone()),
        ("orchestrator", config.orchestrator.clone().unwrap_or_default()),
        ("serve", config.serve.clone().unwrap_or_default()),
        ("queries", config.queries.join(",")),
        ("load_deadline_ns", ns(config.load_deadline).to_string()),
        ("keep_products", config.keep_products.to_string()),
        ("process_mode", if config.serve.is_some() { "served" } else if config.orchestrator.is_some() { "orchestrated" } else { "separate" }.to_owned()),
        ("served_scope", "one-server-per-arm;load-once-reported-in-loads.tsv;call=request-write-to-closed-sink;call-deadline=deadline_ns;load-deadline=load_deadline_ns;lost-server-later-calls-indeterminate;times-in-run-dirs-call-relative;memory.tsv=campaign-peak-incl-loads".to_owned()),
        ("checker_rc_scope", "checker-or-combined-orchestrator;producer-rc-empty-in-orchestrated-mode".to_owned()),
        ("query", config.query.clone()), ("blocks", config.blocks.to_string()),
        ("seed", config.seed.to_string()), ("random", "splitmix64;unbiased-paired-order".to_owned()),
        ("deadline_ns", ns(config.deadline).to_string()), ("sample_period_ns", ns(config.period).to_string()),
        ("memory_bytes", config.memory_bytes.to_string()), ("size_bytes", config.size_bytes.to_string()),
        ("clock_ticks_per_second", ticks.to_string()),
        ("sampler_start", "before-first-spawn".to_owned()),
        ("sampler_end", "after-process-teardown;before-publication;final-boundary-sample".to_owned()),
        ("sampler_schedule", "fixed-period;boundary-samples;overrun-skips-catch-up".to_owned()),
        ("process_identity", "pid+proc-stat-starttime;parent-closure+owned-process-groups+previously-observed-lives".to_owned()),
        ("rss_scope", "harness+owned-descendants;live-RSS-summed-per-scan;loaded-RSS-included".to_owned()),
        ("rss_units", "bytes;proc-kB-times-1024;budget-GB-times-1000000000".to_owned()),
        ("hwm_scope", "each-observed-process-lifetime;harness-HWM-spans-the-campaign".to_owned()),
        ("sampling_limit", "spikes-and-process-lifetimes-shorter-than-period-may-be-missed".to_owned()),
        ("hwm_limit", "observed-HWM-sum-is-not-a-simultaneous-peak-or-a-bound-on-unobserved-processes".to_owned()),
        ("process_limit", "unobserved-double-fork+setsid-between-scans-may-escape-tracking".to_owned()),
        ("cpu_source", "proc-root-own+waited-children-ticks;harness-own-ticks;quantized-at-CLK_TCK".to_owned()),
        ("cpu_limit", "unwaited-or-unobserved-descendant-CPU-may-be-missed;child-reported-phase-CPU-kept-apart".to_owned()),
        ("score", "orchestrated=closed-answer-sink-end-minus-declared-load;post-sink-lifecycle-reported-separately;legacy=whole-invocation-minus-load".to_owned()),
        ("sink_clock", "same-host-UNIX-ns-mapped-to-controller-monotonic-bracket;clock-step-refused;uncertainty-reported".to_owned()),
        ("lifecycle_cpu", "post-sink-root-CPU-from-CLK_TCK;parent-stage-CPU-conservatively-before-sink;later-parent-CPU-in-lifecycle".to_owned()),
        ("summary_quantile", "linear-interpolation-at-(n-1)*q;IQR=Q75-Q25;incomplete-arm-has-no-latency-summary".to_owned()),
        ("ratio_bootstrap", "ratio-of-medians;whole-pairs;10000-resamples;nearest-rank-one-sided-95-upper;fixed-seed".to_owned()),
        ("binary_identities", "observed-proc-exe-paths-hashed-after-run;explicit-identity-files-hashed-before-run;script-sources-need-identity-file".to_owned()),
        ("cleanup_grace_ns", "2000000000".to_owned()),
        ("child_phase_positions", "reported-apart;duration-only-rows-do-not-imply-measured-offsets".to_owned()),
        ("sink", "per-run/answers.txt;orchestrated=explicit-file-flush+close+atomic-marker;legacy=checker-stdout;publish-after-success-only".to_owned()),
        ("caps", "one-invocation-deadline;sampled-process-tree-RSS;polled-total-product-bytes".to_owned()),
        ("resource_limits", "producer-enforces-SWI-table+stack;commands+identity-files-pin-the-settings".to_owned()),
        ("affinity_threads", "inherited-from-reference-protocol;recorded-in-host.tsv".to_owned()),
        ("exclusions", "none;every-run-kept;interference-rerun-is-a-whole-block-new-campaign".to_owned()),
        ("claim", "benchmark-outcome-only;not-the-public-verified-patient-answer-claim".to_owned()),
    ] { pair(&mut settings, key, value); }
    write(&config.out.join("settings.tsv"), settings)?;
    let mut host = "key\tvalue\n".to_owned();
    pair(&mut host, "kernel", read("/proc/sys/kernel/osrelease"));
    pair(&mut host, "loadavg", read("/proc/loadavg"));
    pair(
        &mut host,
        "available_parallelism",
        std::thread::available_parallelism()
            .map(|n| n.to_string())
            .unwrap_or_else(|_| "unreadable".to_owned()),
    );
    let cpu = read("/proc/cpuinfo");
    pair(
        &mut host,
        "cpu_model",
        cpu.lines()
            .find_map(|line| {
                line.strip_prefix("model name")
                    .and_then(|s| s.split_once(':'))
                    .map(|(_, s)| s.trim())
            })
            .unwrap_or("unreadable"),
    );
    for line in read("/proc/self/status").lines().filter(|line| {
        line.starts_with("Cpus_allowed_list:") || line.starts_with("Mems_allowed_list:")
    }) {
        if let Some((name, value)) = line.split_once(':') {
            pair(&mut host, name, value.trim());
        }
    }
    for line in read("/proc/meminfo")
        .lines()
        .filter(|line| line.starts_with("Swap") || line.starts_with("MemTotal:"))
    {
        if let Some((name, value)) = line.split_once(':') {
            pair(&mut host, name, value.trim());
        }
    }
    for name in [
        "OMP_NUM_THREADS",
        "OPENBLAS_NUM_THREADS",
        "MKL_NUM_THREADS",
        "RAYON_NUM_THREADS",
        "CKC_JOBS",
    ] {
        pair(
            &mut host,
            name,
            std::env::var(name).unwrap_or_else(|_| "unset".to_owned()),
        );
    }
    for (directory, prefix, suffix) in [
        ("/sys/devices/system/cpu", "cpu", "cpufreq/scaling_governor"),
        ("/sys/class/thermal", "thermal_zone", "temp"),
    ] {
        if let Ok(entries) = fs::read_dir(directory) {
            let mut paths: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().starts_with(prefix))
                .map(|e| e.path().join(suffix))
                .filter(|p| p.is_file())
                .collect();
            paths.sort();
            for path in paths {
                pair(&mut host, &path.display().to_string(), read(&path));
            }
        }
    }
    pair(
        &mut host,
        "harness_sha256",
        executable_digest(&std::env::current_exe().map_err(|e| e.to_string())?)?.1,
    );
    pair(
        &mut host,
        "shell_sha256",
        executable_digest(Path::new("/bin/sh"))?.1,
    );
    pair(
        &mut host,
        "producer_command_sha256",
        hash_text(&config.producer),
    );
    pair(
        &mut host,
        "checker_command_sha256",
        hash_text(&config.checker),
    );
    if let Some(command) = &config.orchestrator {
        pair(&mut host, "orchestrator_command_sha256", hash_text(command));
    }
    if let Some(command) = &config.serve {
        pair(&mut host, "serve_command_sha256", hash_text(command));
    }
    write(&config.out.join("host.tsv"), host)?;
    let mut identities = "arm\tpath\tbytes\tsha256\n".to_owned();
    let mut seen = std::collections::BTreeSet::new();
    let queries: std::collections::BTreeSet<_> =
        plan.iter().map(|e| (e.arm, e.query.clone())).collect();
    for entry in plan {
        if !seen.insert(entry.arm) {
            continue;
        }
        let mut paths = vec![
            config.root.join("guidelines/schema.pl"),
            entry.composition.clone(),
        ];
        paths.extend(
            queries
                .iter()
                .filter(|(arm, _)| *arm == entry.arm)
                .map(|(_, q)| q.clone()),
        );
        paths.push(entry.directory.join("manifest.tsv"));
        let composition = fs::read_to_string(&entry.composition).map_err(|e| e.to_string())?;
        for line in composition
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
        {
            let path = PathBuf::from(line);
            paths.push(if path.is_absolute() {
                path
            } else {
                config.root.join(path)
            });
        }
        for path in paths.into_iter().chain(config.identities.iter().cloned()) {
            let (size, hash) = digest(&path)?;
            identities.push_str(&format!(
                "{}\t{}\t{size}\t{hash}\n",
                entry.arm,
                field(&path.display().to_string())
            ));
        }
    }
    write(&config.out.join("identities.tsv"), identities)?;
    let mut plan_text = "run\tblock\torder\tarm\tcomposition\tquery\n".to_owned();
    for entry in plan {
        plan_text.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            entry.run,
            entry.block,
            entry.order,
            entry.arm,
            field(&entry.composition.display().to_string()),
            field(&entry.query.display().to_string())
        ));
    }
    write(&config.out.join("plan.tsv"), plan_text)
}
pub(super) fn per_run(path: &Path, ledger: &Ledger, phases: &[Phase], state: &State) -> Result<()> {
    let mut intervals = "phase\tkind\tstart_ns\tend_ns\twall_ns\tcpu_seconds\n".to_owned();
    for row in &ledger.intervals {
        intervals.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{:.9}\n",
            row.phase,
            row.kind,
            row.start,
            row.end,
            row.end - row.start,
            row.cpu
        ));
    }
    write(&path.join("intervals.tsv"), intervals)?;
    let mut lifecycle = "phase\tside\tstart_ns\tend_ns\twall_ns\tcpu_seconds\n".to_owned();
    for row in ledger
        .intervals
        .iter()
        .filter(|row| row.kind == "lifecycle")
    {
        lifecycle.push_str(&format!(
            "{}\tload\t{}\t{}\t{}\t{:.9}\n",
            row.phase,
            row.start,
            row.end,
            row.end - row.start,
            row.cpu
        ));
    }
    write(&path.join("lifecycle-phases.tsv"), lifecycle)?;
    let mut subphases = "process\tphase\twall_ns\tcpu_seconds\tstart_seconds\tkind\n".to_owned();
    for row in phases {
        subphases.push_str(&format!(
            "{}\t{}\t{}\t{:.9}\t{}\t{}\n",
            row.role,
            field(&row.name),
            row.wall,
            row.cpu,
            row.start.map(|s| format!("{s:.9}")).unwrap_or_default(),
            row.kind
        ));
    }
    write(&path.join("phases.tsv"), subphases)?;
    let mut samples = File::create(path.join("rss.tsv")).map_err(|e| e.to_string())?;
    let mut detail = File::create(path.join("rss-processes.tsv")).map_err(|e| e.to_string())?;
    writeln!(samples, "start_ns\tend_ns\tphase\trss_bytes\tprocess_count")
        .map_err(|e| e.to_string())?;
    writeln!(detail, "sample_start_ns\tsample_end_ns\tpid\tstart_ticks\tppid\tpgrp\tstate\trss_bytes\thwm_bytes\town_cpu_ticks\twaited_child_cpu_ticks").map_err(|e| e.to_string())?;
    for row in &state.samples {
        writeln!(
            samples,
            "{}\t{}\t{}\t{}\t{}",
            row.start,
            row.end,
            row.phase,
            row.rss,
            row.processes.len()
        )
        .map_err(|e| e.to_string())?;
        for p in &row.processes {
            writeln!(
                detail,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                row.start,
                row.end,
                p.pid,
                p.start,
                p.parent,
                p.group,
                p.state,
                p.rss,
                p.hwm,
                p.own_ticks,
                p.children_ticks
            )
            .map_err(|e| e.to_string())?;
        }
    }
    let mut processes = "pid\tstart_ticks\tfirst_seen_ns\tlast_seen_ns\tsamples\tpeak_rss_bytes\thwm_bytes\tobserved_own_cpu_ticks\tlast_state\n".to_owned();
    for seen in state.processes.values() {
        processes.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            seen.process.pid,
            seen.process.start,
            seen.first,
            seen.last,
            seen.samples,
            seen.peak_rss,
            seen.hwm,
            seen.own_ticks,
            seen.process.state
        ));
    }
    write(&path.join("processes.tsv"), processes)?;
    let mut executables = "path\tbytes\tsha256\tstatus\n".to_owned();
    for executable in &state.executables {
        match executable_digest(executable) {
            Ok((size, hash)) => executables.push_str(&format!(
                "{}\t{size}\t{hash}\tread\n",
                field(&executable.display().to_string())
            )),
            Err(_) => executables.push_str(&format!(
                "{}\t\t\tunavailable\n",
                field(&executable.display().to_string())
            )),
        }
    }
    write(&path.join("executables.tsv"), executables)
}
pub(super) const RUN_HEADER: &str = "run\tblock\torder\tarm\toutcome\treason\twall_ns\tload_ns\tscored_ns\tcpu_seconds\tpeak_rss_bytes\tobserved_hwm_sum_bytes\tsample_period_ns\tmax_sample_gap_ns\tmemory_uncertainty\tproducer_rc\tchecker_rc\toutput_bytes\toutput_sha256\tphase_intervals\tprocess_high_water_marks\tsink_end_ns\tteardown_ns\tsink_clock_uncertainty_ns\n";
pub(super) fn run_row(row: &RunRow, ledger: &Ledger, state: &State, period: u64) -> String {
    let intervals = ledger
        .intervals
        .iter()
        .map(|r| format!("{}:{}-{}", r.phase, r.start, r.end))
        .collect::<Vec<_>>()
        .join(";");
    let hwm = state
        .processes
        .values()
        .map(|r| format!("{}@{}:{}", r.process.pid, r.process.start, r.hwm))
        .collect::<Vec<_>>()
        .join(";");
    format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.9}\t{}\t{}\t{}\t{}\tpossibly-missed-subperiod-spike\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        row.run,
        row.block,
        row.order,
        row.arm,
        row.outcome,
        field(&row.reason),
        row.wall,
        row.load,
        row.scored.map(|n| n.to_string()).unwrap_or_default(),
        row.cpu,
        row.peak_rss,
        row.hwm_sum,
        period,
        state.max_gap,
        row.producer_rc.map(|n| n.to_string()).unwrap_or_default(),
        row.checker_rc.map(|n| n.to_string()).unwrap_or_default(),
        row.output_bytes.map(|n| n.to_string()).unwrap_or_default(),
        row.output_hash.as_deref().unwrap_or(""),
        intervals,
        hwm,
        row.sink_end.map(|n| n.to_string()).unwrap_or_default(),
        row.teardown,
        row.sink_uncertainty
            .map(|n| n.to_string())
            .unwrap_or_default()
    )
}
fn percentile(values: &[f64], quantile: f64) -> f64 {
    let rank = (values.len() - 1) as f64 * quantile;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    values[lower] + (values[upper] - values[lower]) * rank.fract()
}
fn stats(mut values: Vec<f64>) -> (f64, f64, f64) {
    values.sort_by(f64::total_cmp);
    (
        percentile(&values, 0.5),
        percentile(&values, 0.75) - percentile(&values, 0.25),
        values[values.len() - 1],
    )
}
pub(super) fn summary(config: &Config, runs: &[RunRow]) -> Result<()> {
    let queries: std::collections::BTreeSet<&str> = runs.iter().map(|r| r.query.as_str()).collect();
    let mut text = "arm\tquery\ttotal\tverified\trefused\tindeterminate\tstatus\tmedian_seconds\tiqr_seconds\tmax_seconds\tmedian_peak_rss_bytes\tiqr_peak_rss_bytes\tmax_peak_rss_bytes\n".to_owned();
    for arm in ["baseline", "target"] {
        for query in &queries {
            let rows: Vec<_> = runs
                .iter()
                .filter(|r| r.arm == arm && r.query == *query)
                .collect();
            if rows.is_empty() {
                continue;
            }
            let good: Vec<_> = rows.iter().filter(|r| r.outcome == "verified").collect();
            let refused = rows.iter().filter(|r| r.outcome == "refused").count();
            text.push_str(&format!(
                "{arm}\t{}\t{}\t{}\t{refused}\t{}\t{}",
                field(query),
                rows.len(),
                good.len(),
                rows.len() - good.len() - refused,
                if good.len() == rows.len() {
                    "complete"
                } else {
                    "incomplete"
                }
            ));
            if good.len() != rows.len() {
                text.push_str("\t\t\t\t\t\t\n");
                continue;
            }
            let (median, iqr, max) = stats(
                good.iter()
                    .map(|r| r.scored.unwrap_or_default() as f64 / 1e9)
                    .collect(),
            );
            let (rss, rss_iqr, rss_max) = stats(good.iter().map(|r| r.peak_rss as f64).collect());
            text.push_str(&format!(
                "\t{median:.9}\t{iqr:.9}\t{max:.9}\t{rss:.0}\t{rss_iqr:.0}\t{rss_max:.0}\n"
            ));
        }
    }
    write(&config.out.join("summary.tsv"), text)?;
    if config.paired.is_none() {
        return Ok(());
    }
    let mut ratio =
        "query\tpairs\tstatus\tratio_of_medians\tone_sided_95_upper\tbootstrap_resamples\tseed\n"
            .to_owned();
    for query in &queries {
        let scored = |block: u64, arm: &str| {
            runs.iter()
                .find(|r| {
                    r.block == block && r.arm == arm && r.query == *query && r.outcome == "verified"
                })
                .and_then(|r| r.scored)
                .map(|n| n as f64)
        };
        let pairs: Vec<_> = (1..=config.blocks)
            .filter_map(|block| {
                let baseline = scored(block, "baseline")?;
                let target = scored(block, "target")?;
                (baseline > 0.0).then_some((baseline, target))
            })
            .collect();
        if pairs.len() != config.blocks as usize {
            ratio.push_str(&format!(
                "{}\t{}\tnot-estimable\t\t\t10000\t{}\n",
                field(query),
                pairs.len(),
                config.seed
            ));
            continue;
        }
        let observed = stats(pairs.iter().map(|p| p.1).collect()).0
            / stats(pairs.iter().map(|p| p.0).collect()).0;
        let mut random = Random::new(config.seed);
        let mut estimates = Vec::with_capacity(10000);
        for _ in 0..10000 {
            let selected: Vec<_> = (0..pairs.len())
                .map(|_| pairs[random.below(pairs.len())])
                .collect();
            estimates.push(
                stats(selected.iter().map(|p| p.1).collect()).0
                    / stats(selected.iter().map(|p| p.0).collect()).0,
            );
        }
        estimates.sort_by(f64::total_cmp);
        let upper = estimates[(0.95 * estimates.len() as f64).ceil() as usize - 1];
        ratio.push_str(&format!(
            "{}\t{}\tcomplete\t{observed:.9}\t{upper:.9}\t10000\t{}\n",
            field(query),
            pairs.len(),
            config.seed
        ));
    }
    write(&config.out.join("ratio.tsv"), ratio)
}
pub(super) fn model_lines(path: &Path) -> Result<u64> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut lines = 0;
    for line in BufReader::new(file).split(b'\n') {
        line.map_err(|e| e.to_string())?;
        lines += 1;
    }
    Ok(lines)
}
