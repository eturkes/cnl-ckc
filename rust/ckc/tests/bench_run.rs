use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Mutex;

static SERIAL: Mutex<()> = Mutex::new(());

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn program() -> PathBuf {
    std::env::var_os("CKC_BENCH_RUN_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_ckc").into())
}
struct Scratch(PathBuf);
impl Scratch {
    fn new(name: &str) -> Self {
        let path = root().join(format!(
            "rust/target/bench-run/{}-{name}",
            std::process::id()
        ));
        assert!(!path.exists(), "scratch collision: {}", path.display());
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn composition(&self, name: &str) -> PathBuf {
        let dir = self.0.join(name);
        fs::create_dir_all(dir.join("queries")).unwrap();
        fs::write(dir.join("composition.tsv"), "# mock composition\n").unwrap();
        fs::write(dir.join("manifest.tsv"), "family\tmock\n").unwrap();
        fs::write(dir.join("queries/q-all.pl"), "query.\n").unwrap();
        dir
    }
    fn command(&self, role: &str, mode: &str) -> String {
        format!(
            "python3 -I '{}' {role} {mode}",
            root().join("tests/bench-run/mock.py").display()
        )
    }
    fn run(&self, mode: &str, extra: &[&str]) -> Output {
        let composition = self.composition("target");
        self.invoke(&composition, "result", mode, extra)
    }
    fn invoke(&self, composition: &Path, out: &str, mode: &str, extra: &[&str]) -> Output {
        let mut options: Vec<String> = [
            "--query",
            "q-all",
            "--blocks",
            "1",
            "--deadline-ms",
            "5000",
            "--sample-ms",
            "10",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        for pair in extra.as_chunks::<2>().0 {
            if let Some(index) = options.iter().position(|value| value == pair[0]) {
                options[index + 1] = pair[1].to_owned();
            } else {
                options.extend(pair.iter().map(|s| (*s).to_owned()));
            }
        }
        Command::new("timeout")
            .args(["--kill-after=1", "30"])
            .arg(program())
            .args(["bench", "run"])
            .arg(composition)
            .args(["--producer", &self.command("producer", mode)])
            .args(["--checker", &self.command("checker", mode)])
            .args(["--root", root().to_str().unwrap(), "--out"])
            .arg(self.0.join(out))
            .args(options)
            .current_dir(&self.0)
            .output()
            .unwrap()
    }
    fn result(&self, name: &str) -> String {
        fs::read_to_string(self.0.join("result").join(name)).unwrap()
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn rows(text: &str) -> Vec<BTreeMap<&str, &str>> {
    let mut lines = text.lines();
    let header: Vec<_> = lines.next().unwrap().split('\t').collect();
    lines
        .map(|line| {
            let values: Vec<_> = line.split('\t').collect();
            assert_eq!(header.len(), values.len(), "TSV row: {line}");
            header.iter().copied().zip(values).collect()
        })
        .collect()
}
fn number(row: &BTreeMap<&str, &str>, key: &str) -> u64 {
    row.get(key)
        .unwrap_or_else(|| panic!("missing {key}: {row:?}"))
        .parse()
        .unwrap()
}
fn succeeded(output: &Output) {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "the harness writes answers to its frozen sink, never stdout"
    );
}
fn no_answers(s: &Scratch) {
    let run = s.0.join("result/run-000001");
    for path in ["answers.txt", "model.txt", "certificate.txt", ".work"] {
        assert!(!run.join(path).exists(), "partial product: {path}");
    }
}

#[test]
fn exclusive_ledger_accounts_the_path_and_excludes_only_load() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("ledger");
    succeeded(&s.run("parallel", &[]));
    let text = s.result("runs.tsv");
    let run = &rows(&text)[0];
    assert_eq!(run["outcome"], "verified");
    assert_eq!(s.result("run-000001/answers.txt"), "COMPLETE-ANSWER\n");
    let ledger_text = s.result("run-000001/intervals.tsv");
    let ledger = rows(&ledger_text);
    let mut end = 0;
    let mut gaps = 0;
    let mut stages = 0;
    for row in ledger {
        assert_eq!(number(&row, "start_ns"), end, "ledger holes/overlap");
        end = number(&row, "end_ns");
        assert_eq!(number(&row, "wall_ns"), end - number(&row, "start_ns"));
        match row["kind"] {
            "orchestration" => gaps += 1,
            "subprocess" => stages += 1,
            other => panic!("unexpected ledger kind {other}"),
        }
    }
    assert_eq!(stages, 2);
    assert!(gaps >= 3);
    assert_eq!(end, number(run, "wall_ns"));
    assert_eq!(number(run, "scored_ns") + number(run, "load_ns"), end);
    assert_eq!(number(run, "load_ns"), 30_000_000);
    let phase_text = s.result("run-000001/phases.tsv");
    let phases = rows(&phase_text);
    assert_eq!(phases.len(), 11);
    assert!(
        phases
            .iter()
            .all(|row| row["cpu_seconds"].parse::<f64>().unwrap().is_finite())
    );
    assert!(s.result("run-000001/processes.tsv").contains("hwm_bytes"));
    assert!(s.result("host.tsv").contains("harness_sha256\t"));
    assert!(
        s.result("settings.tsv")
            .contains("sampler_start\tbefore-first-spawn")
    );
}

#[test]
fn paired_plan_is_seeded_and_keeps_every_raw_sample() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("paired");
    let target = s.composition("target");
    let baseline = s.composition("baseline");
    for destination in ["result", "again"] {
        succeeded(&s.invoke(
            &target,
            destination,
            "normal",
            &[
                "--paired",
                baseline.to_str().unwrap(),
                "--seed",
                "98172",
                "--blocks",
                "4",
            ],
        ));
    }
    let plan = s.result("plan.tsv");
    let again = fs::read_to_string(s.0.join("again/plan.tsv")).unwrap();
    assert_eq!(plan, again);
    let plan_rows = rows(&plan);
    assert_eq!(plan_rows.len(), 8);
    assert!(
        plan_rows
            .iter()
            .any(|r| r["order"] == "1" && r["arm"] == "target")
    );
    assert!(
        plan_rows
            .iter()
            .any(|r| r["order"] == "1" && r["arm"] == "baseline")
    );
    for block in 1..=4 {
        let block = block.to_string();
        let pair: Vec<_> = plan_rows.iter().filter(|r| r["block"] == block).collect();
        assert_eq!(pair.len(), 2);
        assert_ne!(pair[0]["arm"], pair[1]["arm"]);
        assert_eq!((pair[0]["order"], pair[1]["order"]), ("1", "2"));
    }
    assert_eq!(rows(&s.result("runs.tsv")).len(), 8);
    for run in 1..=8 {
        assert!(rows(&s.result(&format!("run-{run:06}/rss.tsv"))).len() > 1);
    }
}

#[test]
fn touched_allocations_are_seen_across_offsets_and_process_lifetimes() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("allocations");
    succeeded(&s.run("allocations", &[]));
    let rss = s.result("run-000001/rss.tsv");
    let samples = rows(&rss);
    let processes_text = s.result("run-000001/processes.tsv");
    let processes = rows(&processes_text);
    assert!(
        processes.len() >= 7,
        "harness + producer/checker + four allocation children"
    );
    let highwater: Vec<_> = processes
        .iter()
        .filter(|r| number(r, "hwm_bytes") >= 18_000_000)
        .collect();
    assert!(highwater.len() >= 4, "{processes_text}");
    assert!(samples.iter().any(|r| number(r, "rss_bytes") >= 50_000_000));
    assert!(
        processes
            .iter()
            .all(|r| number(r, "last_seen_ns") >= number(r, "first_seen_ns"))
    );
    let run_text = s.result("runs.tsv");
    let run = &rows(&run_text)[0];
    assert_eq!(
        number(run, "peak_rss_bytes"),
        samples
            .iter()
            .map(|r| number(r, "rss_bytes"))
            .max()
            .unwrap()
    );
    assert!(number(run, "observed_hwm_sum_bytes") >= number(run, "peak_rss_bytes"));
}

#[test]
fn subperiod_spike_retains_the_sampling_uncertainty() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("short-spike");
    succeeded(&s.run("short-spike", &["--sample-ms", "200"]));
    let settings = s.result("settings.tsv");
    assert!(settings.contains(
        "sampling_limit\tspikes-and-process-lifetimes-shorter-than-period-may-be-missed"
    ));
    let text = s.result("runs.tsv");
    let run = &rows(&text)[0];
    assert_eq!(run["memory_uncertainty"], "possibly-missed-subperiod-spike");
    assert_eq!(number(run, "sample_period_ns"), 200_000_000);
    assert!(!s.result("run-000001/rss.tsv").is_empty());
}

#[test]
fn timeout_discards_partial_answers_and_reaps_the_group() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("timeout");
    let output = s.run("timeout", &["--deadline-ms", "250"]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    let text = s.result("runs.tsv");
    let run = &rows(&text)[0];
    assert_eq!(
        (run["outcome"], run["reason"]),
        ("indeterminate", "deadline")
    );
    no_answers(&s);
    let pid = s.result("run-000001/child.pid");
    let path = PathBuf::from(format!("/proc/{}/stat", pid.trim()));
    if let Ok(stat) = fs::read_to_string(path) {
        let state = stat
            .rsplit_once(')')
            .unwrap()
            .1
            .split_whitespace()
            .next()
            .unwrap();
        assert_eq!(state, "Z", "timed-out descendant is still live");
    }
}

#[test]
fn memory_cap_and_size_cap_are_indeterminate_without_partial_answers() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    for (mode, cap, reason) in [
        ("memory", "--memory-bytes", "memory"),
        ("size", "--size-bytes", "size"),
    ] {
        let s = Scratch::new(mode);
        let amount = if mode == "memory" {
            "42000000"
        } else {
            "100000"
        };
        let output = s.run(mode, &[cap, amount]);
        assert_eq!(
            output.status.code(),
            Some(1),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.is_empty());
        let text = s.result("runs.tsv");
        let run = &rows(&text)[0];
        assert_eq!((run["outcome"], run["reason"]), ("indeterminate", reason));
        no_answers(&s);
    }
}

#[test]
fn refusal_and_producer_failure_stay_distinct_from_verified_empty_answers() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let empty = Scratch::new("empty");
    succeeded(&empty.run("empty", &[]));
    assert_eq!(empty.result("run-000001/answers.txt"), "");
    assert_eq!(rows(&empty.result("runs.tsv"))[0]["outcome"], "verified");
    for (mode, outcome, reason) in [
        ("refused", "refused", "checker"),
        ("producer-failure", "indeterminate", "producer"),
        ("bad-phases", "indeterminate", "phase-report"),
    ] {
        let s = Scratch::new(mode);
        let output = s.run(mode, &[]);
        assert_eq!(
            output.status.code(),
            Some(1),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.is_empty());
        let text = s.result("runs.tsv");
        let run = &rows(&text)[0];
        assert_eq!((run["outcome"], run["reason"]), (outcome, reason));
        no_answers(&s);
        let summary_text = s.result("summary.tsv");
        let summary = &rows(&summary_text)[0];
        assert_eq!(summary["status"], "incomplete");
        assert_eq!(summary["median_seconds"], "");
    }
}

#[test]
fn usage_error_never_reuses_an_existing_output_directory() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("reuse");
    let target = s.composition("target");
    succeeded(&s.invoke(&target, "result", "normal", &[]));
    let original = s.result("runs.tsv");
    let output = s.invoke(&target, "result", "normal", &[]);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(s.result("runs.tsv"), original);
}
