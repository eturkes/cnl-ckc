// Shipped CLI characterization before the patient-engine, reachability + witness units.
// Expectations = base 9129d719 observations; CKC_IMP_TEST_BIN selects a replay binary.
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::Mutex;
use std::time::Instant;

const DOSE: &str = "tests/queries/green/dose-arithmetic/tree/guidelines/fx";
const FORMULAS: [&str; 2] = ["cdc2022-opioid-s33-01", "cdc2022-opioid-s42-15"];
static SERIAL: Mutex<()> = Mutex::new(());

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn fixture(name: &str) -> Vec<u8> {
    fs::read(root().join("tests/imp-characterization").join(name)).unwrap()
}
fn text(path: &Path) -> String {
    path.to_str().unwrap().to_owned()
}
fn program() -> PathBuf {
    std::env::var_os("CKC_IMP_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}
fn swipl() -> PathBuf {
    std::env::var_os("SWIPL")
        .map(PathBuf::from)
        .unwrap_or_else(|| "swipl".into())
}

struct Scratch(PathBuf);
impl Scratch {
    fn new(name: &str) -> Self {
        let path = root().join(format!(
            "rust/target/imp-characterization/{}-{name}",
            std::process::id()
        ));
        assert!(!path.exists(), "scratch collision: {}", path.display());
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn file(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, bytes).unwrap();
        path
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

// Harness watchdog only; wall time is printed, never a performance assertion.
fn run(program: &Path, cwd: &Path, args: &[String], input: &[u8]) -> Output {
    let mut child = Command::new("timeout")
        .args(["--kill-after=5", "900"])
        .arg(program)
        .args(args)
        .current_dir(cwd)
        .env("LANG", "C.UTF-8")
        .env("LC_ALL", "C.UTF-8")
        .env("CKC_JOBS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}
fn native(s: &Scratch, label: &str, mode: &str, paths: &[PathBuf]) -> Output {
    let mut args = vec!["v1".into(), mode.into()];
    args.extend(paths.iter().map(|p| text(p)));
    let start = Instant::now();
    let out = run(&program(), &s.0, &args, b"");
    println!(
        "time: imp {label} {:.3} s; rc={:?} stdout={} stderr={}",
        start.elapsed().as_secs_f64(),
        out.status.code(),
        out.stdout.len(),
        out.stderr.len()
    );
    out
}
fn quiet<'a>(label: &str, out: &'a Output) -> &'a [u8] {
    assert_eq!(
        (out.status.code(), out.stderr.as_slice()),
        (Some(0), &b""[..]),
        "{label}: stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    &out.stdout
}
fn pinned(label: &str, out: &Output, expected: &[u8]) {
    pinned_exit(label, out, 0, expected, b"");
}
fn pinned_exit(label: &str, out: &Output, rc: i32, stdout: &[u8], stderr: &[u8]) {
    assert_eq!(out.status.code(), Some(rc), "{label}: rc");
    assert_eq!(out.stdout, stdout, "{label}: stdout");
    assert_eq!(out.stderr, stderr, "{label}: stderr");
}
fn sorted_pl(dir: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "pl"))
        .collect();
    paths.sort();
    paths
}
fn cdc() -> Vec<PathBuf> {
    let docs = sorted_pl(&root().join("guidelines/cdc-2022-opioid/pl"));
    assert_eq!(docs.len(), 492, "characterized CDC composition");
    docs
}
// Recursion-check and answer modes read the payload column's path, never its bytes.
fn manifest(s: &Scratch, name: &str, docs: &[PathBuf]) -> PathBuf {
    let unused = s.file("readable.payload", b"payload content is ignored here\n");
    let rows: String = docs
        .iter()
        .map(|doc| format!("{}\t{}\n", doc.display(), unused.display()))
        .collect();
    s.file(name, rows.as_bytes())
}

#[test]
fn k1_cdc_and_dose_world_keep_recursion_reports() {
    let _serial = SERIAL.lock().unwrap_or_else(|poison| poison.into_inner());
    let s = Scratch::new("recursion");
    let docs = cdc();
    let m = manifest(&s, "cdc.tsv", &docs);
    let out = native(&s, "K1a cdc", "recursion-check", &[m]);
    pinned("K1a cdc", &out, &fixture("recursion-cdc.stdout"));
    let mut composition = docs;
    composition.push(root().join(DOSE).join("pl/world.pl"));
    let m = manifest(&s, "dose.tsv", &composition);
    let out = native(&s, "K1b dose", "recursion-check", &[m]);
    pinned("K1b dose", &out, &fixture("recursion-dose.stdout"));
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let kind = entry.file_type().unwrap();
        assert!(!kind.is_symlink());
        if kind.is_dir() {
            copy_tree(&entry.path(), &to.join(entry.file_name()));
        } else {
            fs::copy(entry.path(), to.join(entry.file_name())).unwrap();
        }
    }
}
fn quote(path: &Path) -> String {
    format!(
        "'{}'",
        text(path).replace('\\', "\\\\").replace('\'', "\\'")
    )
}
fn stage(s: &Scratch) -> PathBuf {
    let version = run(&swipl(), &s.0, &["--version".into()], b"");
    assert!(String::from_utf8_lossy(quiet("SWI pin", &version)).contains("version 10.0.2 "));
    let stage = s.0.join("ape");
    copy_tree(&root().join("vendor/ape"), &stage);
    fs::copy(
        root().join("vendor/clex/clex_lexicon.pl"),
        stage.join("prolog/lexicon/clex_lexicon.pl"),
    )
    .unwrap();
    for goal in [
        format!(
            "working_directory(_,{}),[fit_to_plp],halt(0)",
            quote(&stage.join("prolog/parser"))
        ),
        format!(
            "style_check(-discontiguous),qcompile({}),halt(0)",
            quote(&stage.join("prolog/lexicon/clex_lexicon.pl"))
        ),
    ] {
        let mut args = ["-q", "-O", "-f", "none", "-F", "none", "-g"]
            .map(str::to_owned)
            .to_vec();
        args.extend([goal, "-t".into(), "halt(9)".into()]);
        // fit_to_plp writes its Reading/Writing progress to stderr.
        let out = run(&swipl(), &s.0, &args, b"");
        assert!(out.status.success(), "APE stage: {out:?}");
    }
    stage
}

// DRS-driver CLI → certify-one stdout. Each payload is derived afresh; no large corpus
// dump, manifest or payload bundle is committed, and aggregate never trusts a fixture list.
fn payload(s: &Scratch, stage: &Path, pl: &Path) -> Vec<u8> {
    let id = pl.file_stem().unwrap().to_str().unwrap();
    let guideline = root().join("guidelines/cdc-2022-opioid");
    let ace = guideline.join("ace").join(format!("{id}.ace"));
    let ulex = guideline.join("lexicon.ulex");
    let mut args = ["-q", "-f", "none", "-F", "none", "-s"]
        .map(str::to_owned)
        .to_vec();
    args.push(text(&root().join("rust/ckc/prolog/drs_dump.pl")));
    args.extend(["-g", "main", "-t", "halt(9)", "--"].map(str::to_owned));
    args.extend([text(stage), text(&ulex)]);
    let dump = run(&swipl(), &s.0, &args, &fs::read(&ace).unwrap());
    let dump = s.file("document.dump", quiet(&format!("dump {id}"), &dump));
    let args = vec![
        "certify-one".into(),
        "doc".into(),
        id.into(),
        text(&ace),
        text(&ulex),
        text(&dump),
        text(pl),
        text(&guideline.join("vocabulary.tsv")),
    ];
    let out = run(&program(), &s.0, &args, b"");
    quiet(&format!("certify-one {id}"), &out).to_vec()
}

// The formula documents' proof payloads (witness values, computed heads), derived afresh;
// `just certify` replays every derived payload in the aggregate check.
#[test]
fn k3_formula_payloads_keep_bytes() {
    let _serial = SERIAL.lock().unwrap_or_else(|poison| poison.into_inner());
    let s = Scratch::new("formulas");
    let stage = stage(&s);
    for id in FORMULAS {
        let start = Instant::now();
        let pl = root().join(format!("guidelines/cdc-2022-opioid/pl/{id}.pl"));
        let bytes = payload(&s, &stage, &pl);
        let hash: String = Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let pin = format!("{} {hash}\n", bytes.len());
        assert_eq!(
            pin.into_bytes(),
            fixture(&format!("{id}.payload.sha256")),
            "K3 {id}"
        );
        println!("time: imp K3 {id} {:.3} s", start.elapsed().as_secs_f64());
    }
}

fn dose_manifest(s: &Scratch) -> PathBuf {
    manifest(s, "dose.tsv", &sorted_pl(&root().join(DOSE).join("pl")))
}

#[test]
fn k4_dose_answers_keep_committed_bytes() {
    let _serial = SERIAL.lock().unwrap_or_else(|poison| poison.into_inner());
    let s = Scratch::new("dose-answers");
    let m = dose_manifest(&s);
    for q in ["q-mme", "q-mme-exact"] {
        let fx = root().join(DOSE).join("queries");
        let query = fx.join("pl").join(format!("{q}.pl"));
        let out = native(&s, &format!("K4 {q}"), "answer", &[m.clone(), query]);
        pinned(
            &format!("K4 {q}"),
            &out,
            &fs::read(fx.join("answers").join(format!("{q}.pl"))).unwrap(),
        );
    }
}

#[test]
fn k4_cdc_dose_open_should_query_keeps_bounded_outcome() {
    let _serial = SERIAL.lock().unwrap_or_else(|poison| poison.into_inner());
    let s = Scratch::new("open-should");
    let mut docs = cdc();
    docs.push(root().join(DOSE).join("pl/world.pl"));
    let m = manifest(&s, "composition.tsv", &docs);
    let query = s.file("open-should.pl", &fixture("open-should.pl.in"));
    let out = native(&s, "K4 open should", "answer", &[m, query]);
    pinned("K4 open should", &out, &fixture("open-should.stdout"));
}

#[test]
fn k5_dose_traces_and_checks_keep_committed_bytes() {
    let _serial = SERIAL.lock().unwrap_or_else(|poison| poison.into_inner());
    let s = Scratch::new("dose-traces");
    let m = dose_manifest(&s);
    for q in ["q-mme", "q-mme-exact"] {
        let fx = root().join(DOSE).join("queries");
        let query = fx.join("pl").join(format!("{q}.pl"));
        let answers = fx.join("answers").join(format!("{q}.pl"));
        let out = native(
            &s,
            &format!("K5 trace {q}"),
            "trace",
            &[m.clone(), query.clone(), answers.clone()],
        );
        pinned(
            &format!("K5 trace {q}"),
            &out,
            &fs::read(fx.join("traces").join(format!("{q}.pl"))).unwrap(),
        );
        let traces = s.file(&format!("{q}.traces.pl"), &out.stdout);
        let checked = native(
            &s,
            &format!("K5 trace-check {q}"),
            "trace-check",
            &[m.clone(), query, answers, traces],
        );
        pinned(
            &format!("K5 trace-check {q}"),
            &checked,
            &fixture(&format!("trace-check-{q}.stdout")),
        );
    }
}
