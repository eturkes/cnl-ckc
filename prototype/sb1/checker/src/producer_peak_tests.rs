use super::*;
use crate::peak::Launcher;
use std::os::unix::ffi::OsStrExt;

const SCRIPT: &str = include_str!("producer_launcher.py");
const REPORT: &str = "send('exited', sequence, identity, pid, ticks, kind, status, wait4_peak, peak, cpu_us, observed)";
fn fixture() -> (PathBuf, OrchestratedRequest) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/producer-peak")
        .join(std::process::id().to_string());
    fs::create_dir_all(root.join("guidelines")).unwrap();
    fs::write(
        root.join("guidelines/schema.pl"),
        ckc_kernel::contract::schema_file(),
    )
    .unwrap();
    fs::write(
        root.join("knowledge.pl"),
        include_bytes!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s9-05.pl"),
    )
    .unwrap();
    fs::write(
        root.join("patient.pl"),
        include_bytes!("../../../../bench/sb/fixtures/patients/p-adult.pl.in"),
    )
    .unwrap();
    fs::write(root.join("composition.tsv"), "knowledge.pl\npatient.pl\n").unwrap();
    fs::write(root.join("q-all.txt"), "sb1-q-all 0\n").unwrap();
    let request = OrchestratedRequest {
        root: root.clone(),
        composition: root.join("composition.tsv"),
        output: root.join("normal"),
        query: root.join("q-all.txt"),
        provider: Provider::All,
        producer: "unused".into(),
        swipl: "false".into(),
    };
    (root, request)
}
#[test]
fn missing_malformed_zero_or_mismatched_reports_publish_no_peak() {
    let (root, mut request) = fixture();
    let mut prepared =
        PreparedProgram::prepare(&root, &request.composition, SourceCodec::Parsed, None).unwrap();
    fs::create_dir_all(&request.output).unwrap();
    let failure = prepared.call(&request, None).unwrap_err();
    assert_eq!(failure.rc, 3);
    assert!(
        failure
            .message
            .starts_with("indeterminate producer exit status: 1")
    );
    assert!(
        prepared
            .counters
            .render()
            .contains("runtime\tproducer_maxrss_bytes\t")
    );
    let complete = "send('complete', sequence, identity)";
    for (name, anchor, replacement) in [
        ("missing", REPORT, "sys.exit(0)".to_owned()),
        ("malformed", REPORT, "send('garbage')".to_owned()),
        ("zero", REPORT, REPORT.replace(", peak,", ", 0,")),
        ("zero-raw", REPORT, REPORT.replace(", wait4_peak,", ", 0,")),
        (
            "raw-shape",
            REPORT,
            REPORT.replace(", wait4_peak,", ", 'NA',"),
        ),
        (
            "sequence",
            REPORT,
            REPORT.replace(", sequence,", ", sequence + 1,"),
        ),
        (
            "identity",
            REPORT,
            REPORT.replace(", identity,", ", '0' * 64,"),
        ),
        ("pid", REPORT, REPORT.replace(", pid,", ", pid + 1,")),
        ("start", REPORT, REPORT.replace(", ticks,", ", ticks + 1,")),
        ("status", REPORT, REPORT.replace(", status,", ", 65535,")),
        (
            "exit-core-bit",
            REPORT,
            REPORT.replace(", status,", ", 128,"),
        ),
        (
            "signal-high-byte",
            REPORT,
            REPORT
                .replace(", status,", ", 265,")
                .replace(", kind,", ", 'signal',"),
        ),
        ("kind", REPORT, REPORT.replace(", kind,", ", 'signal',")),
        ("cpu-shape", REPORT, REPORT.replace(", cpu_us,", ", 'NA',")),
        ("peak-shape", REPORT, REPORT.replace(", peak,", ", 'NA',")),
        ("duplicate", REPORT, format!("{REPORT}; {REPORT}")),
        (
            "conflict",
            REPORT,
            format!("{REPORT}; {}", REPORT.replace(", peak,", ", peak + 1024,")),
        ),
        ("missing-complete", complete, "sys.exit(0)".to_owned()),
        (
            "wrong-start-before-reap",
            "send('started', sequence, identity, pid, ticks)",
            "send('started', sequence, identity, pid, ticks + 1)".to_owned(),
        ),
        (
            "helper-killed",
            "pid, status, usage = os.wait4(child.pid, 0)",
            "os.kill(os.getpid(), 9); pid, status, usage = os.wait4(child.pid, 0)".to_owned(),
        ),
    ] {
        assert_eq!(SCRIPT.matches(anchor).count(), 1);
        prepared.launcher = Launcher::start_script(&SCRIPT.replace(anchor, &replacement)).unwrap();
        prepared.counters = Counters::default();
        request.output = root.join(name);
        request.swipl = "true".into();
        fs::create_dir_all(&request.output).unwrap();
        let failure = prepared.call(&request, None).unwrap_err();
        assert_eq!(failure.rc, 2, "{name}");
        assert!(
            failure.message.starts_with("launcher-report "),
            "{name}: {failure:?}"
        );
        if std::env::var_os("SB1_PEAK_CONTROL_PLANT_ZERO").is_some() {
            prepared.counters.runtime("producer_maxrss_bytes", 0);
        }
        let metrics = prepared.counters.render();
        assert!(metrics.contains("runtime\tlauncher_report_failure\t1\n"));
        for key in [
            "producer_pid",
            "producer_start_ticks",
            "producer_maxrss_bytes",
            "producer_cpu_us",
            "producer_wait4_maxrss_bytes",
            "producer_observed_hwm_bytes",
        ] {
            assert!(
                !metrics.contains(&format!("runtime\t{key}\t")),
                "{name}: {metrics}"
            );
        }
        assert!(!request.output.join("producer-report.tsv").exists());
        println!("A6 {name} rc=2 reason={} producer_peak=NA", failure.message);
    }
    drop(prepared);
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn launcher_keeps_argv_bytes_status_identity_and_eof_cleanup() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/producer-peak-argv")
        .join(std::process::id().to_string());
    fs::create_dir_all(&root).unwrap();
    let mut launcher = Launcher::start_script_in(SCRIPT, Some(&root)).unwrap();
    launcher.record_startup(&root).unwrap();
    let startup = fs::read_to_string(root.join("producer-launcher.tsv")).unwrap();
    for key in [
        "pid",
        "parent_pid",
        "pgid",
        "start_ticks",
        "vmhwm_bytes",
        "parent_vmhwm_bytes",
        "started_unix_ns",
        "python_path",
        "python_sha256",
        "python_version",
        "script_sha256",
    ] {
        assert!(
            startup
                .lines()
                .any(|line| line.starts_with(&format!("{key}\t"))),
            "{key}"
        );
    }
    let pid: u32 = startup
        .lines()
        .find_map(|line| line.strip_prefix("pid\t"))
        .unwrap()
        .parse()
        .unwrap();
    let args = [
        "python3",
        "-I",
        "-c",
        "import os,sys; os.write(1, b'\\0'.join(os.fsencode(a) for a in sys.argv[1:]))",
        "",
        "two words",
        "a\nb\tc",
        "é",
        "'\"\\",
    ]
    .map(std::ffi::OsString::from);
    let exit = launcher
        .run(&args, &root.join("stdout"), &root.join("stderr"))
        .unwrap();
    assert!(exit.status.success() && exit.pid != pid && exit.start_ticks > 0 && exit.sequence == 1);
    assert!(exit.peak_bytes > 0 && exit.peak_bytes < 64 << 20);
    assert_eq!(
        exit.peak_bytes,
        exit.wait4_peak_bytes.max(exit.observed_hwm_bytes)
    );
    assert_eq!(
        fs::read(root.join("stdout")).unwrap(),
        b"\0two words\0a\nb\tc\0\xc3\xa9\0'\"\\"
    );
    let exit = launcher
        .run(
            &["false".into()],
            &root.join("stdout"),
            &root.join("stderr"),
        )
        .unwrap();
    assert_eq!(exit.status.code(), Some(1));
    assert_eq!(exit.sequence, 2);
    drop(launcher);
    assert!(!Path::new(&format!("/proc/{pid}")).exists());
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn a_small_report_cannot_be_reused_for_a_touched_large_child() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/producer-peak-stale")
        .join(std::process::id().to_string());
    fs::create_dir_all(&root).unwrap();
    let replacement = "cached = (sequence, identity, pid, ticks, kind, status, wait4_peak, peak, cpu_us, observed) if sequence == 1 else cached; send('exited', *cached)".to_owned();
    let mut launcher = Launcher::start_script(&SCRIPT.replace(REPORT, &replacement)).unwrap();
    let small = launcher
        .run(&["true".into()], &root.join("small"), &root.join("stderr"))
        .unwrap();
    assert!(small.peak_bytes < 64 << 20);
    let args = ["python3", "-I", "-c", "import resource; b=bytearray(512<<20); print(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*1024)"].map(std::ffi::OsString::from);
    let error = launcher
        .run(&args, &root.join("large"), &root.join("stderr"))
        .unwrap_err();
    assert!(error.to_string().contains("launcher report"));
    let observed: u64 = fs::read_to_string(root.join("large"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert!(observed >= 512 << 20);
    println!(
        "A6 stale-small small={} large_observed={observed} refused={error}",
        small.peak_bytes
    );
    drop(launcher);
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn two_arm_call_identities_cannot_be_exchanged() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/producer-peak-swap")
        .join(std::process::id().to_string());
    fs::create_dir_all(&root).unwrap();
    let identity = |directory: &Path| {
        let mut bytes = b"sb1-call\0".to_vec();
        bytes.extend(1_u64.to_le_bytes());
        bytes.extend(directory.as_os_str().as_bytes());
        crate::digest::sha256(&bytes)
    };
    for (own, other) in [("target", "baseline"), ("baseline", "target")] {
        let directory = root.join(own);
        fs::create_dir_all(&directory).unwrap();
        let script = SCRIPT.replace(
            REPORT,
            &REPORT.replace(
                ", identity,",
                &format!(", '{}',", identity(&root.join(other))),
            ),
        );
        let mut launcher = Launcher::start_script(&script).unwrap();
        let error = launcher
            .run(
                &["true".into()],
                &directory.join("stdout"),
                &directory.join("stderr"),
            )
            .unwrap_err();
        assert!(error.to_string().contains("launcher report"));
        println!("A6 swapped-arm={own} refused={error}");
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn charged_peak_is_the_maximum_of_two_retained_raw_terms() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/producer-peak-max")
        .join(std::process::id().to_string());
    fs::create_dir_all(&root).unwrap();
    let anchor = "peak = max(wait4_peak, observed)";
    assert_eq!(SCRIPT.matches(anchor).count(), 1);
    let script = SCRIPT.replace(
        anchor,
        "observed = max(observed, wait4_peak + 1024); peak = max(wait4_peak, observed)",
    );
    let mut launcher = Launcher::start_script(&script).unwrap();
    let exit = launcher
        .run(&["true".into()], &root.join("stdout"), &root.join("stderr"))
        .unwrap();
    // The fixture preserves a larger real observation; its injected 1 KiB is only a lower bound.
    assert!(exit.observed_hwm_bytes >= exit.wait4_peak_bytes + 1024);
    assert_eq!(exit.peak_bytes, exit.observed_hwm_bytes);
    launcher.record_exit(&root, &exit).unwrap();
    let report = fs::read_to_string(root.join("producer-report.tsv")).unwrap();
    assert!(report.contains(&format!(
        "producer_wait4_maxrss_bytes\t{}\n",
        exit.wait4_peak_bytes
    )));
    assert!(report.contains(&format!(
        "producer_observed_hwm_bytes\t{}\n",
        exit.observed_hwm_bytes
    )));
    drop(launcher);
    let wrong = script.replace(REPORT, &REPORT.replace(", peak,", ", wait4_peak,"));
    let mut launcher = Launcher::start_script(&wrong).unwrap();
    assert!(
        launcher
            .run(&["true".into()], &root.join("stdout"), &root.join("stderr"))
            .is_err()
    );
    drop(launcher);
    fs::remove_dir_all(root).unwrap();
}

fn tsv(path: &Path) -> std::collections::BTreeMap<String, String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let (key, value) = line.split_once('\t').unwrap();
            (key.to_owned(), value.to_owned())
        })
        .collect()
}

#[test]
fn terminal_self_report_excludes_children_and_missing_or_foreign_is_unbounded() {
    // The launcher's raw self-report inherits this process's high-water (peak.rs `fresh_process`).
    if !crate::peak::fresh_process(
        "orchestrator::producer_peak_tests::terminal_self_report_excludes_children_and_missing_or_foreign_is_unbounded",
    ) {
        return;
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/launcher-terminal")
        .join(std::process::id().to_string());
    for (name, script) in [
        ("normal", SCRIPT.to_owned()),
        ("error", SCRIPT.to_owned()),
        (
            "missing",
            SCRIPT.replace("os.replace(_terminal_tmp, _terminal)", "os._exit(code)"),
        ),
        (
            "pid",
            SCRIPT.replace(
                "report_pid = _launcher_pid",
                "report_pid = _launcher_pid + 1",
            ),
        ),
        (
            "start",
            SCRIPT.replace(
                "report_ticks = _launcher_ticks",
                "report_ticks = _launcher_ticks + 1",
            ),
        ),
        ("killed", SCRIPT.to_owned()),
    ] {
        let directory = root.join(name);
        fs::create_dir_all(&directory).unwrap();
        let mut launcher = Launcher::start_script_in(&script, Some(&directory)).unwrap();
        launcher.record_startup(&directory).unwrap();
        let startup = tsv(&directory.join("producer-launcher.tsv"));
        let pid: u32 = startup["pid"].parse().unwrap();
        if name == "normal" {
            let args =
                ["python3", "-I", "-c", "b=bytearray(512<<20)"].map(std::ffi::OsString::from);
            let exit = launcher
                .run(&args, &directory.join("stdout"), &directory.join("stderr"))
                .unwrap();
            assert!(exit.peak_bytes >= 512 << 20);
        } else if name == "error" {
            assert!(
                launcher
                    .run(
                        &[directory.join("no-program").into_os_string()],
                        &directory.join("stdout"),
                        &directory.join("stderr")
                    )
                    .is_err()
            );
        } else if name == "killed" {
            assert!(
                Command::new("kill")
                    .args(["-KILL", &pid.to_string()])
                    .status()
                    .unwrap()
                    .success()
            );
        }
        let before = fs::read_to_string(format!("/proc/{pid}/status"))
            .ok()
            .and_then(|text| {
                text.lines().find_map(|line| {
                    line.strip_prefix("VmHWM:")
                        .and_then(|s| s.split_whitespace().next())
                        .and_then(|s| s.parse::<u64>().ok())
                })
            })
            .unwrap_or(0)
            * 1024;
        drop(launcher);
        let checked = tsv(&directory.join("launcher-terminal-check.tsv"));
        if matches!(name, "normal" | "error") {
            assert_eq!(checked["verdict"], "bounded", "{name}: {checked:?}");
            let terminal = tsv(&directory.join("launcher-terminal.tsv"));
            assert_eq!(terminal["pid"], startup["pid"]);
            assert_eq!(terminal["start_ticks"], startup["start_ticks"]);
            assert_eq!(terminal["resource_scope"], "self");
            let raw: u64 = terminal["ru_maxrss_bytes"].parse().unwrap();
            let own: u64 = terminal["vmhwm_bytes"].parse().unwrap();
            let bound: u64 = checked["own_bound_bytes"].parse().unwrap();
            assert_eq!(bound, raw.max(own));
            assert!(
                bound >= before && bound <= 64 << 20,
                "{name}: {before} {terminal:?}"
            );
            assert_eq!(
                terminal["exit_kind"],
                if name == "normal" { "normal" } else { "error" }
            );
            println!("D12 {name} own_raw={raw} own_hwm={own} own_bound={bound} before={before}");
        } else {
            assert_eq!(checked["verdict"], "unbounded", "{name}: {checked:?}");
            assert_eq!(checked["own_bound_bytes"], "NA");
            println!("D12 {name} refused terminal bound");
        }
    }
    fs::remove_dir_all(root).unwrap();
}
