// Contract bf B6: committed corpus + dose world, without tabling.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const DOSE: &str = "tests/queries/green/dose-arithmetic/tree/guidelines/fx";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
struct Scratch(PathBuf);
impl Scratch {
    fn new(label: &str) -> Self {
        let dir = root().join(format!(
            "rust/target/bf-consumers/{}-{label}",
            std::process::id()
        ));
        assert!(!dir.exists(), "scratch collision: {}", dir.display());
        fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }
    fn file(&self, name: &str, text: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, text).unwrap();
        path
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn quote(path: &Path) -> String {
    format!(
        "'{}'",
        path.to_str()
            .unwrap()
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
    )
}
fn corpus() -> Vec<PathBuf> {
    let mut docs = Vec::new();
    for guideline in fs::read_dir(root().join("guidelines")).unwrap() {
        let pl = guideline.unwrap().path().join("pl");
        if pl.is_dir() {
            docs.extend(
                fs::read_dir(pl)
                    .unwrap()
                    .map(|e| e.unwrap().path())
                    .filter(|p| p.extension().is_some_and(|e| e == "pl")),
            );
        }
    }
    docs.sort();
    assert!(!docs.is_empty());
    docs.push(root().join(DOSE).join("pl/world.pl"));
    docs
}
fn swi(scratch: &Scratch, goal: &str, seconds: &str) -> Output {
    let driver = scratch.file(
        "driver.pl",
        &fs::read_to_string(root().join("tests/consumer/driver.pl.in")).unwrap(),
    );
    let mut files = vec![driver, root().join("guidelines/schema.pl")];
    files.extend(corpus());
    let files = files.iter().map(|p| quote(p)).collect::<Vec<_>>().join(",");
    Command::new("timeout")
        .args([
            "--kill-after=2",
            seconds,
            "swipl",
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-g",
        ])
        .arg(format!("consult([{files}]),{goal}"))
        .args(["-t", "halt"])
        .current_dir(&scratch.0)
        .stdin(Stdio::null())
        .output()
        .unwrap()
}
fn checked(out: &Output) -> &str {
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success() && !stderr.lines().any(|l| l.starts_with("ERROR")),
        "rc={:?}, stdout={}, stderr={stderr}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout)
    );
    std::str::from_utf8(&out.stdout).unwrap()
}

#[test]
fn plain_swi_answers_both_dose_questions_yes_within_60_seconds() {
    let runs: Vec<_> = ["q-mme", "q-mme-exact"].into_iter().map(|q| std::thread::spawn(move || {
        let scratch = Scratch::new(&format!("swi-{q}"));
        let question = root().join(DOSE).join(format!("queries/pl/{q}.pl"));
        let goal = format!("bounded(1,control,true),read_match({},'$guideline_query_projection'(goal(G),answers([]))),bounded(60,dose,G)", quote(&question));
        (q, swi(&scratch, &goal, "75"))
    })).collect();
    let mut failures = Vec::new();
    for run in runs {
        let (q, out) = run.join().unwrap();
        let got = checked(&out);
        if got != "bounded control yes\nbounded dose yes\n" {
            failures.push(format!("{q}: {got}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn native_engine_answers_both_dose_questions_yes() {
    let runs: Vec<_> = ["q-mme", "q-mme-exact"]
        .into_iter()
        .map(|q| {
            std::thread::spawn(move || {
                let scratch = Scratch::new(&format!("native-{q}"));
                let manifest: String = corpus()
                    .iter()
                    .map(|p| format!("{}\t{}\n", p.display(), p.display()))
                    .collect();
                let manifest = scratch.file("manifest.tsv", &manifest);
                let program = std::env::var_os("CKC_BF_TEST_BIN")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
                // A harness watchdog, not a native-engine semantic bound.
                let out = Command::new("timeout")
                    .args(["--kill-after=2", "120"])
                    .arg(program)
                    .args(["v1", "answer"])
                    .arg(manifest)
                    .arg(root().join(DOSE).join(format!("queries/pl/{q}.pl")))
                    .current_dir(root())
                    .stdin(Stdio::null())
                    .output()
                    .unwrap();
                (q, out)
            })
        })
        .collect();
    let mut failures = Vec::new();
    for run in runs {
        let (q, out) = run.join().unwrap();
        let stdout = String::from_utf8_lossy(&out.stdout);
        if !out.status.success() || !out.stderr.is_empty() || !stdout.ends_with(",result(yes)).\n")
        {
            failures.push(format!(
                "{q}: rc={:?}, stdout={stdout}, stderr={}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr)
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn exhausted_plain_should_enumeration_is_still_unfinished_at_20_seconds() {
    let scratch = Scratch::new("open-enumeration");
    let out = swi(
        &scratch,
        "bounded(1,control,true),bounded(20,open,findall(B,guideline_operator(actual,B,should),_))",
        "35",
    );
    assert_eq!(
        checked(&out),
        "bounded control yes\nbounded open time_limit_exceeded\n",
        "plain exhausted enumeration changed; reconsider the documented tabling requirement"
    );
}
