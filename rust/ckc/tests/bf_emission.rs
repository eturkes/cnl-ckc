// Contract bf B3/B4/B7: seed DRS-order products + an independent scheduler oracle.
// CKC_BF_TEST_BIN / CKC_BF_COMPILER select prebuilt artifacts for red/mutant replay.
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};

#[path = "../../../tests/bf/model.rs"]
mod model;
#[path = "../../../tests/bf/prolog.rs"]
mod prolog;

const CHANGED: [&str; 6] = [
    "neg-event",
    "neg-box",
    "time",
    "time-neg",
    "formula-time",
    "scope",
];
const IDENTITY: [&str; 3] = ["fallback-days", "ordered", "naf"];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn fixture(id: &str, name: &str) -> PathBuf {
    root().join("tests/bf/probes").join(id).join(name)
}
fn text(id: &str, name: &str) -> String {
    let text = fs::read_to_string(fixture(id, name)).unwrap();
    if name == "before.pl.in" && id != "question" {
        // Frozen pre-bf products gain only the sup1 declaration pairs.
        let anchor = ":- discontiguous(guideline_exclusion/1).\n";
        assert_eq!(text.matches(anchor).count(), 1);
        text.replacen(
            anchor,
            &format!("{anchor}:- multifile(guideline_supersession/4).\n:- discontiguous(guideline_supersession/4).\n:- multifile(guideline_evaluation_date/1).\n:- discontiguous(guideline_evaluation_date/1).\n"),
            1,
        )
    } else {
        text
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
fn run(program: &Path, cwd: &Path, args: &[String], stdin: &str) -> Output {
    let mut child = Command::new("timeout")
        .args(["--kill-after=2", "60"])
        .arg(program)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}
fn success<'a>(label: &str, out: &'a Output) -> &'a str {
    assert!(
        out.status.success() && out.stderr.is_empty(),
        "{label}: rc={:?} stdout={} stderr={}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    std::str::from_utf8(&out.stdout).unwrap()
}

struct Compiler {
    scratch: PathBuf,
    stage: PathBuf,
    next: AtomicUsize,
}
impl Compiler {
    fn shared() -> Arc<Self> {
        static SHARED: OnceLock<Mutex<Weak<Compiler>>> = OnceLock::new();
        static SERIAL: AtomicUsize = AtomicUsize::new(0);
        let mut slot = SHARED
            .get_or_init(|| Mutex::new(Weak::new()))
            .lock()
            .unwrap();
        if let Some(c) = slot.upgrade() {
            return c;
        }
        let scratch = root().join(format!(
            "rust/target/bf-emission/{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!scratch.exists());
        fs::create_dir_all(&scratch).unwrap();
        let c = Arc::new(Self {
            stage: scratch.join("ape"),
            scratch,
            next: AtomicUsize::new(0),
        });
        let version = run(Path::new("swipl"), &c.scratch, &["--version".into()], "");
        assert!(success("SWI pin", &version).contains("version 10.0.2 "));
        copy_tree(&root().join("vendor/ape"), &c.stage);
        if let Some(file) = std::env::var_os("CKC_BF_COMPILER") {
            fs::copy(file, c.stage.join("prolog/ace_to_pl.pl")).unwrap();
        }
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            c.stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        for goal in [
            format!(
                "working_directory(_,{}),[fit_to_plp],halt(0)",
                quote(&c.stage.join("prolog/parser"))
            ),
            format!(
                "style_check(-discontiguous),qcompile({}),halt(0)",
                quote(&c.stage.join("prolog/lexicon/clex_lexicon.pl"))
            ),
        ] {
            let args = [
                "-q", "-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
            ]
            .map(str::to_owned);
            let out = run(Path::new("swipl"), &c.scratch, &args, "");
            assert!(out.status.success(), "APE stage: {out:?}");
        }
        *slot = Arc::downgrade(&c);
        c
    }
    fn emit(&self, id: &str, ace: &str, table: &Path, ulex: &Path, mode: &str) -> Output {
        let mut args = ["-q", "-f", "none", "-F", "none", "-s"]
            .map(str::to_owned)
            .to_vec();
        args.push(self.stage.join("prolog/ace_to_pl.pl").display().to_string());
        args.extend(["-g", "main", "-t", "halt(9)", "--"].map(str::to_owned));
        if mode == "question" {
            args.push(mode.into());
        }
        args.extend([
            self.stage.display().to_string(),
            id.into(),
            table.display().to_string(),
            ulex.display().to_string(),
        ]);
        if mode == "proof" {
            args.push(mode.into());
        }
        run(Path::new("swipl"), &self.scratch, &args, ace)
    }
    fn probe(&self, id: &str, mode: &str) -> Output {
        self.emit(
            &format!("bf-{id}"),
            &text(id, "source.ace"),
            &fixture(id, "vocabulary.tsv"),
            &fixture(id, "lexicon.ulex"),
            mode,
        )
    }
    fn certify(&self, id: &str, mode: &str, pl: &[u8]) -> Output {
        let dir = self
            .scratch
            .join(self.next.fetch_add(1, Ordering::Relaxed).to_string());
        fs::create_dir(&dir).unwrap();
        let path = dir.join("compiled.pl");
        fs::write(&path, pl).unwrap();
        let program = std::env::var_os("CKC_BF_TEST_BIN")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
        let args = [
            "certify-one".into(),
            mode.into(),
            format!("bf-{id}"),
            fixture(id, "source.ace").display().to_string(),
            fixture(id, "lexicon.ulex").display().to_string(),
            fixture(id, "dump.in").display().to_string(),
            path.display().to_string(),
            fixture(id, "vocabulary.tsv").display().to_string(),
        ];
        run(&program, &root(), &args, "")
    }
}
impl Drop for Compiler {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.scratch).unwrap();
    }
}

fn mismatches(id: &str, got: &str, failures: &mut Vec<String>) {
    let before = prolog::clauses(&text(id, "before.pl.in"));
    let want: Vec<_> = before.iter().map(prolog::reordered).collect();
    assert!(
        before
            .iter()
            .cloned()
            .map(prolog::canonical)
            .collect::<Vec<_>>()
            != want,
        "change-sensitive probe is identity: {id}"
    );
    let actual: Vec<_> = prolog::clauses(got)
        .into_iter()
        .map(prolog::canonical)
        .collect();
    if actual != want {
        let at = actual
            .iter()
            .zip(&want)
            .position(|(a, b)| a != b)
            .unwrap_or(actual.len().min(want.len()));
        failures.push(format!(
            "{id}: clause {at} != independent bind_first(pre-unit body); got={:?}; want={:?}",
            actual.get(at),
            want.get(at)
        ));
    }
    let metadata = |s: &str| {
        s.lines()
            .filter(|l| l.starts_with('%') || l.starts_with(":-"))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        metadata(got),
        metadata(&text(id, "before.pl.in")),
        "{id}: metadata changed"
    );
}

#[test]
fn changed_rule_bodies_match_reference_and_certify() {
    let compiler = Compiler::shared();
    let mut failures = Vec::new();
    for id in CHANGED {
        let output = compiler.probe(id, "doc");
        let got = success(id, &output);
        mismatches(id, got, &mut failures);
        let certified = compiler.certify(id, "doc", &output.stdout);
        success(&format!("certify {id}"), &certified);
    }
    assert!(
        failures.is_empty(),
        "{} probes differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn witness_facts_follow_the_body_permutation() {
    let compiler = Compiler::shared();
    let mut failures = Vec::new();
    for id in CHANGED {
        let old = prolog::clauses(&text(id, "before.pl.in"));
        let body = &old.iter().find(|c| !c.body.is_empty()).unwrap().body;
        let expected = prolog::reordered_proof(&text(id, "before.proof.in"), body);
        let out = compiler.probe(id, "proof");
        let actual = prolog::clauses(success(id, &out));
        if actual != expected {
            failures.push(format!("{id}: proof witness != body permutation"));
        }
        let document = compiler.probe(id, "doc");
        success(id, &document);
        let certified = compiler.certify(id, "doc", &document.stdout);
        assert_eq!(
            success(&format!("certify {id}"), &certified),
            success(id, &out),
            "{id}: fork proof != certifier proof"
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn fallback_and_already_ordered_rules_keep_exact_bytes() {
    let compiler = Compiler::shared();
    for id in IDENTITY {
        let before = text(id, "before.pl.in");
        for clause in prolog::clauses(&before) {
            assert_eq!(
                prolog::canonical(clause.clone()),
                prolog::reordered(&clause),
                "identity fixture {id}"
            );
        }
        for mode in ["doc", "proof"] {
            let expected = text(
                id,
                if mode == "doc" {
                    "before.pl.in"
                } else {
                    "before.proof.in"
                },
            );
            let out = compiler.probe(id, mode);
            assert_eq!(success(id, &out), expected, "{id}/{mode}: identity bytes");
        }
        let out = compiler.certify(id, "doc", before.as_bytes());
        assert_eq!(success(id, &out), text(id, "before.proof.in"));
    }
    let body = prolog::clauses(&text("fallback-days", "before.pl.in"))
        .into_iter()
        .find(|c| !c.body.is_empty())
        .unwrap()
        .body;
    let shared = model::positive_vars(&body);
    assert!(
        body.iter()
            .all(|item| !model::ready(item, &Default::default(), &shared)),
        "3-days starts with no ready item"
    );
}

#[test]
fn fact_groups_and_questions_keep_exact_bytes() {
    let compiler = Compiler::shared();
    for (id, mode) in [("facts", "doc"), ("question", "question")] {
        let expected = text(id, "before.pl.in");
        let out = compiler.probe(id, mode);
        assert_eq!(success(id, &out), expected);
        let certified = compiler.certify(
            id,
            if mode == "question" { "query" } else { "doc" },
            &out.stdout,
        );
        success(id, &certified);
        if mode == "doc" {
            let proof = compiler.probe(id, "proof");
            assert_eq!(success(id, &proof), text(id, "before.proof.in"));
            assert_eq!(certified.stdout, proof.stdout);
        }
    }
}

#[test]
fn existing_rejections_keep_their_diagnostics() {
    let compiler = Compiler::shared();
    for name in [
        "unsupported--v1-naf-consequent",
        "unsupported--v1-root-at-most",
        "unsupported--disjunction",
        "unsupported--da2-position-root",
    ] {
        let path = root().join("tests/red").join(name);
        let sidecar = |ext| {
            let p = path.with_extension(ext);
            if p.exists() { p } else { PathBuf::from("-") }
        };
        let ace = fs::read_to_string(path.with_extension("ace")).unwrap();
        let expected = fs::read(path.with_extension("expect")).unwrap();
        let out = compiler.emit(
            "red-probe",
            &ace,
            &sidecar("vocabulary.tsv"),
            &sidecar("ulex"),
            "doc",
        );
        assert_eq!(out.status.code(), Some(1), "{name}: {out:?}");
        assert!(out.stdout.is_empty());
        assert_eq!(out.stderr, expected, "{name}");
    }
}

#[test]
fn committed_corpus_rule_bodies_are_scheduler_fixed_points() {
    let mut changed = Vec::new();
    let mut documents = 0;
    let mut clauses = 0;
    let mut guidelines: Vec<_> = fs::read_dir(root().join("guidelines"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.join("pl").is_dir())
        .collect();
    guidelines.sort();
    for guideline in guidelines {
        let mut docs: Vec<_> = fs::read_dir(guideline.join("pl"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "pl"))
            .collect();
        docs.sort();
        for doc in docs {
            documents += 1;
            let parsed = prolog::clauses(&fs::read_to_string(&doc).unwrap());
            clauses += parsed.iter().filter(|c| !c.body.is_empty()).count();
            if parsed
                .iter()
                .any(|c| prolog::canonical(c.clone()) != prolog::reordered(c))
            {
                changed.push(doc.file_stem().unwrap().to_string_lossy().into_owned());
            }
        }
    }
    assert!(documents > 0 && clauses > 0);
    assert!(
        changed.is_empty(),
        "{} of {documents} documents not in binding-first order ({} rules); first ten: {:?}",
        changed.len(),
        clauses,
        &changed[..changed.len().min(10)]
    );
}
