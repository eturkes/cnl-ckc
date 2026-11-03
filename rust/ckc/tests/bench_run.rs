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
    fn orchestrated(&self, mode: &str) -> Output {
        let composition = self.composition("target");
        let script = self.0.join("orchestrator.py");
        fs::write(&script, ORCHESTRATOR).unwrap();
        Command::new("timeout")
            .args(["--kill-after=1", "30"])
            .arg(program())
            .args(["bench", "run"])
            .arg(composition)
            .args([
                "--orchestrator",
                &format!("python3 -I '{}' {mode}", script.display()),
            ])
            .args([
                "--query",
                "q-all",
                "--blocks",
                "1",
                "--deadline-ms",
                if mode == "late-timeout" {
                    "1000"
                } else {
                    "5000"
                },
                "--sample-ms",
                "10",
            ])
            .args(["--root", root().to_str().unwrap(), "--out"])
            .arg(self.0.join("result"))
            .env("SB1_STAGE_TRACE", if mode == "traced" { "1" } else { "0" })
            .args(if mode == "late-memory" {
                vec!["--memory-bytes", "42000000"]
            } else {
                Vec::new()
            })
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

const ORCHESTRATOR: &str = r#"
import hashlib, pathlib, subprocess, sys, time
mode = sys.argv[1]
assert sys.argv[2] == '--root' and len(sys.argv) == 7
out = pathlib.Path(sys.argv[5])
start = time.perf_counter()
rows = []
for name in ['load_parse', 'load_admission', 'load_analysis', 'load_index',
             'patient', 'slice', 'materialize', 'producer', 'decode', 'index',
             'support', 'closure', 'answers']:
    before = time.perf_counter() - start
    if name == 'producer':
        subprocess.run([sys.executable, '-I', '-c', 'import time; x=bytearray(16000000); time.sleep(.08)'], check=True)
    else:
        time.sleep(.012)
    elapsed = time.perf_counter() - start - before
    kind = 'load' if name.startswith('load_') else 'scored'
    if mode == 'bad-kind' and name == 'patient': kind = 'load'
    rows.append([name, str(elapsed), '0', str(before), kind])
if mode == 'overlap': rows[-1][3] = '0'
(out / 'orchestrator-phases.tsv').write_text('phase\tseconds\tcpu\tstart_seconds\tkind\n' + ''.join('\t'.join(r)+'\n' for r in rows))
(out / 'model.txt').write_text('model.\n')
(out / 'certificate.txt').write_text('certificate\n')
if mode == 'refused':
    print('refused claim planted', file=sys.stderr)
    sys.exit(1)
if mode == 'producer-error':
    print('indeterminate producer planted', file=sys.stderr)
    sys.exit(3)
answer = b'ORCHESTRATED-ANSWER\n'
digest = hashlib.sha256(answer).hexdigest()
with (out / 'answers.txt').open('wb') as sink:
    sink.write(answer)
    sink.flush()
closed = time.time_ns()
if mode == 'future-sink': closed += 60_000_000_000
fields = pathlib.Path('/proc/self/stat').read_text().rsplit(')',1)[1].split()
cpu = int(fields[11])+int(fields[12])+max(0,int(fields[13]))+max(0,int(fields[14]))
if mode != 'missing-sink':
    if mode == 'bad-sink': digest = '0'*64
    tmp = out / 'sink-complete.tmp'
    tmp.write_text(f'sb1-sink 1\nclosed_unix_ns\t{closed}\nbytes\t{len(answer)}\nsha256\t{digest}\ncpu_ticks\t{cpu}\n')
    tmp.rename(out / 'sink-complete.tsv')
if mode == 'late-teardown':
    memory = bytearray(24*1024*1024)
    for i in range(0,len(memory),4096): memory[i]=1
    time.sleep(.4)
if mode == 'late-memory':
    memory = bytearray(40*1024*1024)
    for i in range(0,len(memory),4096): memory[i]=1
    time.sleep(2)
if mode == 'late-timeout': time.sleep(5)
print('ORCHESTRATED-ANSWER')
"#;

#[test]
fn closed_sink_marker_detaches_late_teardown_from_the_score() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("late-teardown");
    succeeded(&s.orchestrated("late-teardown"));
    let text = s.result("runs.tsv");
    let run = &rows(&text)[0];
    let sink = number(run, "sink_end_ns");
    let teardown = number(run, "teardown_ns");
    assert!(
        teardown >= 350_000_000,
        "planted 400ms post-sink lifecycle stays outside score"
    );
    assert_eq!(number(run, "scored_ns") + number(run, "load_ns"), sink);
    assert_eq!(sink + teardown, number(run, "wall_ns"));
    assert!(
        number(run, "peak_rss_bytes") >= 24_000_000,
        "post-sink allocation remains sampled"
    );
    let ledger_text = s.result("run-000001/intervals.tsv");
    let ledger = rows(&ledger_text);
    let lifecycle = ledger.iter().find(|r| r["kind"] == "lifecycle").unwrap();
    assert_eq!(lifecycle["phase"], "teardown");
    assert_eq!(number(lifecycle, "start_ns"), sink);
    assert_eq!(number(lifecycle, "end_ns"), number(run, "wall_ns"));
    assert_eq!(s.result("run-000001/answers.txt"), "ORCHESTRATED-ANSWER\n");
}

#[test]
fn post_sink_caps_still_reject_partial_invocations() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    for (mode, reason) in [("late-memory", "memory"), ("late-timeout", "deadline")] {
        let s = Scratch::new(mode);
        let output = s.orchestrated(mode);
        assert_eq!(
            output.status.code(),
            Some(1),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = s.result("runs.tsv");
        let row = &rows(&text)[0];
        assert_eq!((row["outcome"], row["reason"]), ("indeterminate", reason));
        no_answers(&s);
    }
}

#[test]
fn orchestrated_sink_marker_requires_closed_bytes_and_a_valid_time() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    for mode in ["missing-sink", "bad-sink", "future-sink"] {
        let s = Scratch::new(mode);
        let output = s.orchestrated(mode);
        assert_eq!(
            output.status.code(),
            Some(1),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = s.result("runs.tsv");
        let row = &rows(&text)[0];
        assert_eq!(
            (row["outcome"], row["reason"]),
            ("indeterminate", "sink-report")
        );
        no_answers(&s);
    }
}

#[test]
fn orchestrated_path_keeps_one_owned_tree_and_the_explicit_load_boundary() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("orchestrated");
    succeeded(&s.orchestrated("normal"));
    assert_eq!(s.result("run-000001/answers.txt"), "ORCHESTRATED-ANSWER\n");
    let run_text = s.result("runs.tsv");
    let run = &rows(&run_text)[0];
    assert_eq!(run["outcome"], "verified");
    let ledger_text = s.result("run-000001/intervals.tsv");
    let ledger = rows(&ledger_text);
    assert_eq!(
        ledger.iter().filter(|r| r["kind"] == "subprocess").count(),
        1
    );
    let mut end = 0;
    for interval in ledger {
        assert_eq!(number(&interval, "start_ns"), end);
        end = number(&interval, "end_ns");
    }
    assert_eq!(end, number(run, "wall_ns"));
    assert_eq!(
        number(run, "scored_ns") + number(run, "load_ns") + number(run, "teardown_ns"),
        end
    );
    assert_eq!(
        number(run, "scored_ns") + number(run, "load_ns"),
        number(run, "sink_end_ns")
    );
    let phase_text = s.result("run-000001/phases.tsv");
    let phases = rows(&phase_text);
    assert_eq!(phases.len(), 13);
    assert!(phases.iter().all(|r| r["process"] == "orchestrator"));
    assert_eq!(phases.iter().filter(|r| r["kind"] == "load").count(), 4);
    assert_eq!(
        number(run, "load_ns"),
        phases
            .iter()
            .filter(|r| r["kind"] == "load")
            .map(|r| number(r, "wall_ns"))
            .sum::<u64>()
    );
    let processes_text = s.result("run-000001/processes.tsv");
    assert!(
        rows(&processes_text).len() >= 3,
        "harness + orchestrator + producer child"
    );
    assert!(number(run, "peak_rss_bytes") >= 20_000_000);
}

#[test]
fn orchestrated_trace_records_exit_and_cleanup_without_changing_answers() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("orchestrated-trace");
    succeeded(&s.orchestrated("traced"));
    assert_eq!(s.result("run-000001/answers.txt"), "ORCHESTRATED-ANSWER\n");
    let text = s.result("run-000001/harness-stage-trace.tsv");
    let trace = rows(&text);
    let mut last = 0;
    for row in &trace {
        let wall = number(row, "wall_ns");
        assert!(wall >= last);
        last = wall;
    }
    for event in [
        "spawn_begin",
        "registered",
        "root_nonlive_or_limit",
        "exit_boundary_done",
        "wait_begin",
        "wait_done",
        "post_wait_boundary_done",
        "answer_sink_digest_begin",
        "answer_sink_renamed",
        "sampler_finish_end",
        "publish_end",
        "scored_wall_end",
    ] {
        assert!(
            trace.iter().any(|row| row["event"] == event),
            "missing {event}"
        );
    }
    let exited = trace
        .iter()
        .find(|row| row["event"] == "root_nonlive_or_limit")
        .unwrap();
    assert!(matches!(exited["child_state"], "Z" | "X"));
    assert!(!exited["child_own_ticks"].is_empty());
}

#[test]
fn orchestrated_reports_fail_closed_and_keep_refusal_distinct() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    for (mode, outcome, reason) in [
        ("refused", "refused", "orchestrator"),
        ("producer-error", "indeterminate", "orchestrator"),
        ("overlap", "indeterminate", "phase-report"),
        ("bad-kind", "indeterminate", "phase-report"),
    ] {
        let s = Scratch::new(&format!("orchestrated-{mode}"));
        let output = s.orchestrated(mode);
        assert_eq!(
            output.status.code(),
            Some(1),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = s.result("runs.tsv");
        let row = &rows(&text)[0];
        assert_eq!((row["outcome"], row["reason"]), (outcome, reason));
        no_answers(&s);
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
    let controls = s.result("run-000001/controls.tsv");
    let mut allocated = BTreeMap::new();
    let mut completed = 0;
    for line in controls.lines() {
        let values: Vec<_> = line.split('\t').collect();
        let timestamp = values[2].parse::<u64>().unwrap();
        if values[0] == "allocated" {
            allocated.insert(values[1], timestamp);
        } else if values[0] == "freed" {
            assert!(
                timestamp - allocated[values[1]] < 200_000_000,
                "the control's actual allocation lifetime must be shorter than the sampling period"
            );
            completed += 1;
        }
    }
    assert_eq!(completed, 2);
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
fn named_q_all_accepts_the_generators_plain_text_marker() {
    let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = Scratch::new("q-all-marker");
    let target = s.composition("target");
    fs::remove_file(target.join("queries/q-all.pl")).unwrap();
    fs::write(target.join("queries/q-all.txt"), "sb1-q-all 0\n").unwrap();
    succeeded(&s.invoke(&target, "result", "normal", &[]));
    assert!(s.result("plan.tsv").contains("queries/q-all.txt"));
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
