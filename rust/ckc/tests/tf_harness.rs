// Contract tf: the property harness's own controls, each with its planted input. P1 = runs repeat
// byte for byte whatever PROPTEST_* says (one child process per environment; the child `probe`
// writes its log to CKC_TF_PROBE_OUT); P2 = a planted defect reports its minimal counterexample;
// P3 = the committed shrink bound binds; the law scan = no suite builds its own runner config.
#[path = "support/prop.rs"]
mod harness;

use proptest::arbitrary::any;
use proptest::collection::vec;
use proptest::prop_assert;
use proptest::strategy::Strategy;
use proptest::test_runner::{Config, RngSeed, TestCaseError, TestError, TestRunner};
use std::cell::{Cell, RefCell};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn probe_strategy() -> impl Strategy<Value = (Vec<u32>, bool, Vec<u8>)> {
    (
        vec(0u32..1000, 0..8),
        any::<bool>(),
        (0usize..5).prop_flat_map(|n| vec(any::<u8>(), n)),
    )
}

// The planted defect: wrong on every vector holding an element ≥ 7.
fn planted(v: &[u32]) -> bool {
    v.iter().all(|&x| x < 7)
}

fn planted_report() -> String {
    let caught = std::panic::catch_unwind(|| {
        harness::check(256, vec(0u32..100, 0..20), |v| {
            prop_assert!(planted(&v));
            Ok(())
        })
    });
    *caught
        .expect_err("the planted defect must fail")
        .downcast::<String>()
        .expect("check reports in a String")
}

// P1's child, also a plain test: the harness's 64 inputs + the planted report, beside the same
// strategy under a config that takes the PROPTEST_* defaults (the comparison's firing).
#[test]
fn probe() {
    let seen = RefCell::new(Vec::new());
    harness::check(64, probe_strategy(), |v| {
        seen.borrow_mut().push(format!("{v:?}"));
        Ok(())
    });
    let naive = RefCell::new(Vec::new());
    let config = Config {
        cases: 64,
        rng_seed: RngSeed::Fixed(harness::SEED),
        failure_persistence: None,
        ..Config::default()
    };
    TestRunner::new(config)
        .run(&probe_strategy(), |v| {
            naive.borrow_mut().push(format!("{v:?}"));
            Ok(())
        })
        .unwrap();
    let mut log = String::new();
    for (i, v) in seen.borrow().iter().enumerate() {
        log += &format!("tf-case\t{i}\t{v}\n");
    }
    for line in planted_report().lines() {
        log += &format!("tf-report\t{line}\n");
    }
    for (i, v) in naive.borrow().iter().enumerate() {
        log += &format!("tf-naive\t{i}\t{v}\n");
    }
    if let Some(out) = std::env::var_os("CKC_TF_PROBE_OUT") {
        fs::write(out, log).unwrap();
    }
}

struct Child {
    ok: bool,
    log: String,
    output: String,
}

fn child(label: &str, vars: &[(&str, &str)]) -> Child {
    let out = std::env::temp_dir().join(format!("ckc-tf-probe-{}-{label}", std::process::id()));
    let _ = fs::remove_file(&out);
    let mut cmd = Command::new(std::env::current_exe().unwrap());
    cmd.args(["probe", "--exact"]);
    for (k, _) in std::env::vars_os() {
        if k.to_string_lossy().starts_with("PROPTEST_") {
            cmd.env_remove(&k);
        }
    }
    let run = cmd
        .env("CKC_TF_PROBE_OUT", &out)
        .envs(vars.iter().copied())
        .output()
        .unwrap();
    let log = fs::read_to_string(&out).unwrap_or_default();
    let _ = fs::remove_file(&out);
    let stdout = String::from_utf8_lossy(&run.stdout).into_owned();
    Child {
        ok: run.status.success() && stdout.contains("test result: ok. 1 passed;"),
        log,
        output: stdout + &String::from_utf8_lossy(&run.stderr),
    }
}

fn tagged<'a>(c: &'a Child, tags: &[&str]) -> Vec<&'a str> {
    c.log
        .lines()
        .filter(|l| tags.contains(&l.split('\t').next().unwrap_or("")))
        .collect()
}

#[test]
fn p1_runs_repeat_byte_for_byte_whatever_proptest_variables_say() {
    let first = child("first", &[]);
    let second = child("second", &[]);
    assert!(first.ok && second.ok, "{}{}", first.output, second.output);
    assert_eq!(tagged(&first, &["tf-case"]).len(), 64, "{}", first.log);
    assert!(!tagged(&first, &["tf-report"]).is_empty(), "{}", first.log);
    assert_eq!(first.log, second.log);
    let moved = child(
        "variables",
        &[
            ("PROPTEST_RNG_SEED", "1"),
            ("PROPTEST_CASES", "5"),
            ("PROPTEST_RNG_ALGORITHM", "xs"),
            ("PROPTEST_MAX_SHRINK_ITERS", "1"),
            ("PROPTEST_MAX_SHRINK_TIME", "1"),
            ("PROPTEST_VERBOSE", "2"),
        ],
    );
    assert!(moved.ok, "{}", moved.output);
    let harness_lines = ["tf-case", "tf-report"];
    assert_eq!(
        tagged(&moved, &harness_lines),
        tagged(&first, &harness_lines)
    );
    // Firing: the same strategy under `..Config::default()` follows PROPTEST_RNG_ALGORITHM.
    assert_ne!(tagged(&moved, &["tf-naive"]), tagged(&first, &["tf-naive"]));
    let refused = child("size", &[("PROPTEST_MAX_DEFAULT_SIZE_RANGE", "3")]);
    assert!(
        !refused.ok && refused.output.contains("PROPTEST_MAX_DEFAULT_SIZE_RANGE"),
        "{}",
        refused.output
    );
}

#[test]
fn p2_planted_defect_reports_its_minimal_counterexample() {
    let report = planted_report();
    assert!(
        report.contains("minimal failing input: [\n    7,\n]"),
        "{report}"
    );
    assert!(
        report.contains(&format!("seed {}", harness::SEED)),
        "{report}"
    );
    let strategy = vec(0u32..100, 0..20);
    let failure = |iters: u32| {
        let mut config = harness::config(256);
        config.max_shrink_iters = iters;
        TestRunner::new(config).run(&strategy, |v| {
            prop_assert!(planted(&v));
            Ok(())
        })
    };
    match failure(harness::MAX_SHRINK_ITERS) {
        Err(TestError::Fail(_, v)) => assert_eq!(v, [7]),
        other => panic!("{other:?}"),
    }
    // Firing: with shrinking off, the first failing input stands in the report.
    match failure(0) {
        Err(TestError::Fail(_, v)) => assert_ne!(v, [7]),
        other => panic!("{other:?}"),
    }
}

#[test]
fn p3_shrinking_stops_at_the_committed_iteration_bound() {
    // Every input fails, and 400 u64s would take ~400 × 64 halving steps to shrink.
    let calls = |iters: u32| {
        let mut config = harness::config(1);
        config.max_shrink_iters = iters;
        let n = Cell::new(0u32);
        let result = TestRunner::new(config).run(&vec(any::<u64>(), 400), |_| {
            n.set(n.get() + 1);
            Err(TestCaseError::fail("planted: every input fails"))
        });
        assert!(result.is_err());
        n.get() - 1
    };
    assert_eq!(calls(harness::MAX_SHRINK_ITERS), harness::MAX_SHRINK_ITERS);
    // Firing: a doubled bound lets the same shrink run past the committed one.
    assert!(calls(2 * harness::MAX_SHRINK_ITERS) > harness::MAX_SHRINK_ITERS);
    let config = harness::config(1);
    assert_eq!(
        (config.max_shrink_time, config.max_shrink_iters),
        (0, harness::MAX_SHRINK_ITERS)
    );
}

// The law (rust.md): suites build no runner config of their own, because each of these reads
// PROPTEST_* (the proptest macro re-applies them over any config it is given).
const BYPASS: [&str; 7] = [
    "proptest!",
    "Config::default",
    "Config::with_",
    "ProptestConfig",
    "TestRunner::default",
    "TestRunner::deterministic",
    "contextualize_config",
];

fn bypasses(text: &str) -> Vec<&'static str> {
    BYPASS.into_iter().filter(|t| text.contains(*t)).collect()
}

// Other suites may create + remove scratch entries meanwhile: a vanished entry is skipped.
fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            rs_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

#[test]
fn suites_run_properties_through_the_harness() {
    // Firing: planted suite text holding every bypass.
    assert_eq!(bypasses(&BYPASS.join("\n")), BYPASS);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut files = Vec::new();
    rs_files(&root.join("rust/ckc/tests"), &mut files);
    rs_files(&root.join("tests"), &mut files);
    assert!(files.len() > 60, "the scan reached {} files", files.len());
    let own = [Path::new("support/prop.rs"), Path::new("tf_harness.rs")];
    let hits: Vec<String> = files
        .iter()
        .filter(|f| !own.iter().any(|o| f.ends_with(o)))
        .filter_map(|f| {
            let found = bypasses(&fs::read_to_string(f).ok()?);
            (!found.is_empty()).then(|| format!("{}: {found:?}", f.display()))
        })
        .collect();
    assert!(hits.is_empty(), "{hits:?}");
}
