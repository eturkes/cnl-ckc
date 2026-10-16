// S1 D1–D7/P2–P3 + S2 D1/P9: diff-blind compiler + vocabulary probes; CKC_S1_TEST_BIN selects red replay.
#[path = "../../../tests/s1-compiler/model.rs"]
mod model;

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

const QUALIFIER_DECLARATIONS: &str =
    ":- multifile(guideline_qualifier/4).\n:- discontiguous(guideline_qualifier/4).\n";
const TABLE_PATH: &str = "guidelines/cdc-2022-opioid/vocabulary.tsv";
const ALIAS_TABLE_PATH: &str = "guidelines/alpha/vocabulary.tsv";
const GIT_ENV: [(&str, &str); 2] = [
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_CONFIG_SYSTEM", "/dev/null"),
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn fixture() -> PathBuf {
    root().join("tests/s1-compiler")
}

fn program() -> PathBuf {
    std::env::var_os("CKC_S1_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}

fn sha(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn write(path: &Path, bytes: impl AsRef<[u8]>) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        assert!(!entry.file_type().unwrap().is_symlink());
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = root().join(format!("rust/target/s1-compiler/{}", std::process::id()));
        assert!(!path.exists(), "scratch collision: {}", path.display());
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.0) {
            eprintln!("S1 cleanup {}: {e}", self.0.display());
        }
    }
}

#[derive(Debug)]
struct Observed {
    rc: Option<i32>,
    stdout: String,
    stderr: String,
}
impl From<Output> for Observed {
    fn from(out: Output) -> Self {
        Self {
            rc: out.status.code(),
            stdout: String::from_utf8(out.stdout).unwrap(),
            stderr: String::from_utf8(out.stderr).unwrap(),
        }
    }
}
impl Observed {
    fn ok(&self) -> bool {
        self.rc == Some(0) && self.stderr.is_empty()
    }
    fn violation(&self) -> &str {
        if self.stderr.is_empty() {
            self.stdout.lines().last().unwrap_or("")
        } else {
            self.stderr.trim_end_matches('\n')
        }
    }
    fn describe(&self) -> String {
        format!("rc={:?}, diagnostic={:?}", self.rc, self.violation())
    }
    fn compiler_reject(&self, class: &str, detail: Option<&str>) -> bool {
        let rc = if class == "unsupported" { 1 } else { 2 };
        self.rc == Some(rc)
            && self.stdout.is_empty()
            && self.stderr.lines().count() == 1
            && self
                .stderr
                .starts_with(&format!("ace_to_pl_error({class},"))
            && self.stderr.ends_with(").\n")
            && detail.is_none_or(|d| self.stderr == format!("ace_to_pl_error({class},{d}).\n"))
    }
}

fn command(bin: &Path, cwd: &Path, args: &[String], input: &[u8]) -> Observed {
    let mut child = Command::new("timeout")
        .args(["--kill-after=2", "60"])
        .arg(bin)
        .args(args)
        .current_dir(cwd)
        .envs(GIT_ENV)
        .env("LC_ALL", "C.UTF-8")
        .env("GIT_CEILING_DIRECTORIES", cwd)
        .env("TMPDIR", cwd)
        .env("CKC_JOBS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap().into()
}

fn git(cwd: &Path, args: &[&str]) -> String {
    let out = command(
        Path::new("git"),
        cwd,
        &args.iter().map(|s| (*s).into()).collect::<Vec<_>>(),
        b"",
    );
    assert_eq!(out.rc, Some(0), "git {args:?}: {}", out.describe());
    out.stdout
}

struct Compiler {
    stage: PathBuf,
}
impl Compiler {
    fn new(scratch: &Path) -> Self {
        let version = command(Path::new("swipl"), scratch, &["--version".into()], b"");
        assert!(
            version.ok() && version.stdout.contains("version 9.2.9 "),
            "SWI pin: {}",
            version.describe()
        );
        let stage = scratch.join("ape");
        copy_tree(&root().join("vendor/ape"), &stage);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let parser = stage
            .join("prolog/parser")
            .to_str()
            .unwrap()
            .replace('\\', "\\\\")
            .replace('\'', "\\'");
        let goal = format!("working_directory(_, '{parser}'), [fit_to_plp], halt.");
        let out = command(
            Path::new("swipl"),
            scratch,
            &[
                "-O".into(),
                "-f".into(),
                "none".into(),
                "-F".into(),
                "none".into(),
                "-g".into(),
                goal,
                "-t".into(),
                "halt".into(),
            ],
            b"",
        );
        assert_eq!(out.rc, Some(0), "compiler stage: {}", out.describe());
        Self { stage }
    }
    fn compile(
        &self,
        cwd: &Path,
        id: &str,
        mode: &str,
        ace: &str,
        table: Option<&str>,
    ) -> Observed {
        fs::create_dir_all(cwd).unwrap();
        let table_arg = table.map_or_else(
            || "-".into(),
            |t| {
                let path = cwd.join("vocabulary.tsv");
                write(&path, t);
                path.to_str().unwrap().to_string()
            },
        );
        let mut args: Vec<String> = ["-q", "-f", "none", "-F", "none", "-s"]
            .iter()
            .map(|s| (*s).into())
            .collect();
        args.push(
            self.stage
                .join("prolog/ace_to_pl.pl")
                .to_str()
                .unwrap()
                .into(),
        );
        args.extend(
            ["-g", "main", "-t", "halt(9)", "--"]
                .iter()
                .map(|s| (*s).to_string()),
        );
        if mode == "query" {
            args.push("question".into());
        }
        args.extend([
            self.stage.to_str().unwrap().into(),
            id.into(),
            table_arg,
            fixture().join("lexicon.ulex").to_str().unwrap().into(),
        ]);
        if mode == "proof" {
            args.push("proof".into());
        }
        command(Path::new("swipl"), cwd, &args, ace.as_bytes())
    }
}

#[derive(Clone)]
struct Case {
    row: String,
    id: String,
    mode: String,
    table: Option<String>,
    grades: usize,
    preferences: usize,
    expect: String,
    ace: String,
}

fn cases() -> Vec<Case> {
    fs::read_to_string(fixture().join("cases.tsv"))
        .unwrap()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(|l| {
            let f: Vec<_> = l.split('\t').collect();
            assert_eq!(f.len(), 8, "case fields: {l}");
            Case {
                row: f[0].into(),
                id: f[1].into(),
                mode: f[2].into(),
                table: (f[3] != "-")
                    .then(|| fs::read_to_string(fixture().join(format!("{}.tsv", f[3]))).unwrap()),
                grades: f[4].parse().unwrap(),
                preferences: f[5].parse().unwrap(),
                expect: f[6].into(),
                ace: format!("{}\n", f[7].replace("\\n", "\n")),
            }
        })
        .collect()
}

struct TableCase {
    row: &'static str,
    id: String,
    bytes: String,
    why: String,
}

fn table_cases() -> Vec<TableCase> {
    let mut out = Vec::new();
    let mut add = |row, id: String, rows: Vec<String>, why: &str, n| {
        let bytes = format!("{}{}\n", model::HEADER, rows.join("\n"));
        let why = format!("{why} at row {n}");
        assert_eq!(
            model::table_verdict(&bytes),
            Err(why.clone()),
            "table reference {id}"
        );
        out.push(TableCase {
            row,
            id,
            bytes,
            why,
        });
    };
    for kind in model::NEW_KINDS {
        add(
            "E1",
            format!("{kind}-foreign-value"),
            vec![format!("{kind}\tx\tforeign")],
            &format!("{kind} value"),
            1,
        );
    }
    for grade in ["strength", "evidence"] {
        let value = if grade == "strength" {
            "strong"
        } else {
            "high"
        };
        let first = format!("{grade}\tx\t{value}");
        add(
            "E2",
            format!("{grade}-duplicate"),
            vec![first.clone(), first.clone()],
            "duplicate noun",
            2,
        );
        for (kind, row) in [
            ("unit", "unit\tx\thour"),
            ("spacing", "spacing\tat\tx"),
            ("window", "window\tduring\tx"),
        ] {
            for reverse in [false, true] {
                let mut rows = vec![first.clone(), row.into()];
                if reverse {
                    rows.reverse();
                }
                add(
                    "E2",
                    format!("{grade}-{kind}-{reverse}"),
                    rows,
                    "duplicate noun",
                    2,
                );
            }
        }
    }
    for reverse in [false, true] {
        let mut rows = vec!["strength\tx\tstrong".into(), "evidence\tx\thigh".into()];
        if reverse {
            rows.reverse();
        }
        add(
            "E2",
            format!("strength-evidence-{reverse}"),
            rows,
            "duplicate noun",
            2,
        );
    }
    add(
        "E3",
        "preference-duplicate".into(),
        vec!["preference\tx\tpreferred".into(); 2],
        "duplicate adjective",
        2,
    );
    for reverse in [false, true] {
        let mut rows = vec![
            "preference\tx\tpreferred".into(),
            "approximation\tx\tabout".into(),
        ];
        if reverse {
            rows.reverse();
        }
        add(
            "E3",
            format!("preference-approximation-{reverse}"),
            rows,
            "duplicate adjective",
            2,
        );
    }
    add(
        "E4",
        "alternative-duplicate".into(),
        vec!["alternative\tx\talternative".into(); 2],
        "duplicate preposition",
        2,
    );
    for (kind, value) in [
        ("relation", "after"),
        ("frequency", "period"),
        ("range", "minimum"),
        ("spacing", "interval"),
        ("window", "window"),
    ] {
        for reverse in [false, true] {
            let mut rows = vec![
                "alternative\tx\talternative".into(),
                format!("{kind}\tx\t{value}"),
            ];
            if reverse {
                rows.reverse();
            }
            add(
                "E4",
                format!("alternative-{kind}-{reverse}"),
                rows,
                "duplicate preposition",
                2,
            );
        }
    }
    for (kind, value) in [
        ("strength", "strong"),
        ("evidence", "high"),
        ("preference", "preferred"),
        ("alternative", "alternative"),
    ] {
        add(
            "E5",
            format!("{kind}-value-before-duplicate"),
            vec![format!("{kind}\tx\t{value}"), format!("{kind}\tx\tINVALID")],
            &format!("{kind} value"),
            2,
        );
    }
    add(
        "E5",
        "comment-first".into(),
        vec!["#strength\t\tINVALID\textra".into()],
        "comment",
        1,
    );
    add(
        "E5",
        "foreign-kind".into(),
        vec!["foreign\tx\ty".into()],
        "kind",
        1,
    );
    add(
        "E5",
        "field-count-before-lemma".into(),
        vec!["strength\t".into()],
        "field count",
        1,
    );
    add(
        "E5",
        "lemma-before-kind".into(),
        vec!["foreign\tbad lemma\tINVALID".into()],
        "lemma",
        1,
    );
    add(
        "E5",
        "lemma-before-value".into(),
        vec!["strength\t\tINVALID".into()],
        "lemma",
        1,
    );
    out
}

struct AliasCase {
    row: &'static str,
    id: String,
    table: String,
    expected: Result<(), String>,
}

fn aliases() -> Vec<AliasCase> {
    let mut out = Vec::new();
    let kinds = [
        ("unit", "hour"),
        ("relation", "after"),
        ("spacing", "frame"),
        ("window", "window"),
        ("frequency", "period"),
        ("approximation", "about"),
        ("range", "minimum"),
        ("strength", "strong"),
        ("evidence", "high"),
        ("preference", "preferred"),
        ("alternative", "alternative"),
    ];
    let mut add = |row, id: String, data: Vec<String>| {
        let table = format!("{}{}\n", model::HEADER, data.join("\n"));
        let expected = model::table_verdict(&table);
        out.push(AliasCase {
            row,
            id,
            table,
            expected,
        });
    };
    for alias in ["x", "hyphen-word", "é", "Ω", "a&<b>", "'", "x\\y", "#lemma"] {
        for (kind, value, row) in [
            ("strength", "strong", "E2"),
            ("evidence", "high", "E2"),
            ("preference", "preferred", "E3"),
            ("alternative", "alternative", "E4"),
        ] {
            for (other, ov) in kinds {
                for equal in [true, false] {
                    for reverse in [false, true] {
                        let name = if equal { alias } else { "distinct-alias" };
                        let mut data = vec![
                            format!("{kind}\t{alias}\t{value}"),
                            format!("{other}\t{name}\t{ov}"),
                        ];
                        if reverse {
                            data.reverse();
                        }
                        add(
                            row,
                            format!("alias-{kind}-{other}-{alias}-{equal}-{reverse}"),
                            data,
                        );
                    }
                }
            }
            if matches!(kind, "strength" | "evidence") {
                for frame in ["spacing", "window"] {
                    for reverse in [false, true] {
                        let mut data = vec![
                            format!("{kind}\t{alias}\t{value}"),
                            format!("{frame}\tframe-preposition\t{alias}"),
                        ];
                        if reverse {
                            data.reverse();
                        }
                        add(
                            row,
                            format!("alias-{kind}-{frame}-noun-{alias}-{reverse}"),
                            data,
                        );
                    }
                }
            }
        }
        for (kind, values) in [
            ("strength", vec!["strong", "conditional"]),
            ("evidence", vec!["high", "moderate", "low", "very_low"]),
            ("preference", vec!["preferred"]),
            ("alternative", vec!["alternative"]),
        ] {
            for value in &values {
                add(
                    "E1",
                    format!("value-{kind}-{alias}-{value}"),
                    vec![format!("{kind}\t{alias}\t{value}")],
                );
            }
            for bad in ["", "INVALID", "strong preferred"] {
                add(
                    "E1",
                    format!("bad-value-{kind}-{alias}-{bad}"),
                    vec![format!("{kind}\t{alias}\t{bad}")],
                );
                add(
                    "E5",
                    format!("bad-duplicate-{kind}-{alias}-{bad}"),
                    vec![
                        format!("{kind}\t{alias}\t{}", values[0]),
                        format!("{kind}\t{alias}\t{bad}"),
                    ],
                );
            }
        }
        for (first, value, old, bad) in [
            ("strength", "strong", "unit", "foreign"),
            ("preference", "preferred", "approximation", "foreign"),
            ("alternative", "alternative", "relation", "foreign"),
            ("alternative", "alternative", "frequency", "foreign"),
            ("alternative", "alternative", "range", "foreign"),
            ("alternative", "alternative", "spacing", ""),
            ("alternative", "alternative", "window", ""),
        ] {
            add(
                "E5",
                format!("old-value-before-conflict-{first}-{old}-{alias}"),
                vec![
                    format!("{first}\t{alias}\t{value}"),
                    format!("{old}\t{alias}\t{bad}"),
                ],
            );
        }
        for (grade, value) in [("strength", "strong"), ("evidence", "high")] {
            for frame in ["spacing", "window"] {
                for reverse in [false, true] {
                    let mut data = vec![
                        format!("{grade}\t{alias}\t{value}"),
                        "alternative\tx-preposition\talternative".into(),
                    ];
                    if reverse {
                        data.reverse();
                    }
                    data.push(format!("{frame}\tx-preposition\t{alias}"));
                    add(
                        "E5",
                        format!("noun-before-preposition-{grade}-{frame}-{alias}-{reverse}"),
                        data,
                    );
                }
            }
        }
    }
    for lemma in ["", "a b", "a\r", "a\u{7f}"] {
        add(
            "E5",
            format!("invalid-lemma-{lemma:?}"),
            vec![format!("strength\t{lemma}\tstrong")],
        );
    }
    out
}

enum Job {
    Compile(usize),
    Table(usize),
    Alias(usize),
}
struct Results {
    compiler: BTreeMap<String, Observed>,
    checks: BTreeMap<String, Observed>,
    table_compiler: BTreeMap<String, Observed>,
    alias: BTreeMap<String, Observed>,
}

fn results() -> &'static Results {
    static RESULTS: OnceLock<Results> = OnceLock::new();
    RESULTS.get_or_init(|| {
        let started = Instant::now();
        let scratch = Scratch::new();
        let compiler = Compiler::new(&scratch.0);
        let cases = cases();
        let tables = table_cases();
        let aliases = aliases();
        let head = git(&root(), &["rev-parse", "HEAD"]);
        let binary = program();
        assert!(binary.is_file(), "CKC_S1_TEST_BIN: {}", binary.display());
        let jobs: Vec<_> = (0..cases.len()).map(Job::Compile)
            .chain((0..tables.len()).map(Job::Table)).chain((0..aliases.len()).map(Job::Alias)).collect();
        let next = AtomicUsize::new(0);
        let timing: [AtomicU64; 3] = std::array::from_fn(|_| AtomicU64::new(0));
        let found = Mutex::new(Results { compiler: BTreeMap::new(), checks: BTreeMap::new(), table_compiler: BTreeMap::new(), alias: BTreeMap::new() });
        let workers = std::thread::available_parallelism().map_or(4, |n| n.get().min(4));
        std::thread::scope(|scope| {
            for worker in 0..workers {
                let (scratch, compiler, cases, tables, aliases, jobs, next, found, head, binary, timing) = (&scratch, &compiler, &cases, &tables, &aliases, &jobs, &next, &found, &head, &binary, &timing);
                scope.spawn(move || {
                    let work = scratch.0.join(format!("worker-{worker}"));
                    fs::create_dir_all(&work).unwrap();
                    let ui = work.join("alias-tree");
                    copy_tree(&root().join("tests/ui/green/q12-temporal-words/tree"), &ui);
                    let tree = work.join("check-tree");
                    git(&work, &["clone", "-q", "--shared", "--no-checkout", root().to_str().unwrap(), "check-tree"]);
                    git(&tree, &["checkout", "-q", "--detach", head.trim()]);
                    while let Some(job) = jobs.get(next.fetch_add(1, Ordering::Relaxed)) {
                        let job_started = Instant::now();
                        match *job {
                            Job::Compile(i) => {
                                let c = &cases[i];
                                let output = compiler.compile(&work.join("compiler"), &c.id, &c.mode, &c.ace, c.table.as_deref());
                                found.lock().unwrap().compiler.insert(c.id.clone(), output);
                            }
                            Job::Table(i) => {
                                let t = &tables[i];
                                write(&tree.join(TABLE_PATH), &t.bytes);
                                let checked = command(binary, &tree, &["check".into()], b"");
                                git(&tree, &["reset", "-q", "--hard", head.trim()]);
                                git(&tree, &["clean", "-q", "-f", "-d", "-x"]);
                                assert!(git(&tree, &["status", "--porcelain", "--ignored"]).is_empty(), "{} reset left changes", t.id);
                                let compiled = compiler.compile(&work.join("table-compiler"), "s1-table", "doc", "A man sleeps.\n", Some(&t.bytes));
                                let mut results = found.lock().unwrap();
                                results.checks.insert(t.id.clone(), checked);
                                results.table_compiler.insert(t.id.clone(), compiled);
                            }
                            Job::Alias(i) => {
                                let a = &aliases[i];
                                write(&ui.join(ALIAS_TABLE_PATH), &a.table);
                                let output = command(binary, &work, &["ui".into(), "check".into(), ui.to_str().unwrap().into()], b"");
                                found.lock().unwrap().alias.insert(a.id.clone(), output);
                            }
                        }
                        let category = match job { Job::Compile(_) => 0, Job::Table(_) => 1, Job::Alias(_) => 2 };
                        timing[category].fetch_add(u64::try_from(job_started.elapsed().as_millis()).unwrap(), Ordering::Relaxed);
                    }
                });
            }
        });
        eprintln!("S1 runtime: {:.3} s; {} compiler probes, {} table plants (check + compiler), {} generated alias/value cases; one stage, {workers} workers", started.elapsed().as_secs_f64(), cases.len(), tables.len(), aliases.len());
        eprintln!("S1 worker-time: compiler={} ms, table plants={} ms, aliases={} ms", timing[0].load(Ordering::Relaxed), timing[1].load(Ordering::Relaxed), timing[2].load(Ordering::Relaxed));
        found.into_inner().unwrap()
    })
}

struct Checks {
    row: &'static str,
    count: usize,
    failures: Vec<String>,
}
impl Checks {
    fn new(row: &'static str) -> Self {
        Self {
            row,
            count: 0,
            failures: Vec::new(),
        }
    }
    fn require(&mut self, ok: bool, detail: impl Into<String>) {
        self.count += 1;
        if !ok {
            self.failures.push(detail.into());
        }
    }
    fn finish(self) {
        eprintln!(
            "{}: {} assertions, {} failures",
            self.row,
            self.count,
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "{} first failure: {}\n{}",
            self.row,
            self.failures.first().map_or("-", String::as_str),
            self.failures.join("\n")
        );
    }
}

fn difference(expected: &str, actual: &str) -> String {
    let e: Vec<_> = expected.lines().collect();
    let a: Vec<_> = actual.lines().collect();
    let i = e
        .iter()
        .zip(&a)
        .position(|(e, a)| e != a)
        .unwrap_or(e.len().min(a.len()));
    format!(
        "line {} expected {:?}, got {:?}; bytes {} vs {}",
        i + 1,
        e.get(i),
        a.get(i),
        expected.len(),
        actual.len()
    )
}

fn baseline(c: &Case) -> String {
    fs::read_to_string(fixture().join(format!("c1/{}.pl.in", c.id))).unwrap()
}

fn expected(c: &Case) -> String {
    let old = baseline(c);
    let text = if c.mode == "query" {
        old
    } else {
        let (out, grades, preferences) = model::annotate(&old, c.table.as_deref());
        assert_eq!(
            (grades, preferences),
            (c.grades, c.preferences),
            "{} reference annotation census",
            c.id
        );
        out.replacen(
            &model::declarations(),
            &(model::declarations() + QUALIFIER_DECLARATIONS),
            1,
        )
    };
    model::rebind(
        &text,
        c.table.as_deref().map(|t| sha(t.as_bytes())).as_deref(),
    )
}

fn compiler_row(row: &'static str) {
    let results = results();
    let mut checks = Checks::new(row);
    for c in cases().iter().filter(|c| c.row == row) {
        let actual = &results.compiler[&c.id];
        if c.expect == "ok" {
            checks.require(
                actual.ok(),
                format!("{} success: {}", c.id, actual.describe()),
            );
            if actual.ok() {
                let predicted = expected(c);
                checks.require(
                    actual.stdout == predicted,
                    format!(
                        "{} exact C1 + D1/D3/D4: {}",
                        c.id,
                        difference(&predicted, &actual.stdout)
                    ),
                );
            }
        } else {
            checks.require(
                actual.compiler_reject("unsupported", Some(&c.expect)),
                format!(
                    "{} expected ace_to_pl_error(unsupported,{}).; {}",
                    c.id,
                    c.expect,
                    actual.describe()
                ),
            );
        }
    }
    checks.finish();
}

fn table_row(row: &'static str) {
    let results = results();
    let mut checks = Checks::new(row);
    for t in table_cases().iter().filter(|c| c.row == row) {
        let actual = &results.checks[&t.id];
        let expected = format!("ckc: vocabulary: {TABLE_PATH}: {}", t.why);
        checks.require(
            actual.rc == Some(1) && actual.violation() == expected,
            format!(
                "{} check expected {expected:?}; {}",
                t.id,
                actual.describe()
            ),
        );
        let compiled = &results.table_compiler[&t.id];
        checks.require(
            compiled.compiler_reject("vocabulary_load", None),
            format!(
                "{} compiler expected vocabulary_load rc 2; {}",
                t.id,
                compiled.describe()
            ),
        );
    }
    for a in aliases().iter().filter(|a| a.row == row) {
        let actual = &results.alias[&a.id];
        // A control byte in the table meets the UI intake's file-level check before the
        // grammar (retained C1 intake law), so those two lemma cases expect its message.
        let control = match a.id.as_str() {
            "invalid-lemma-\"a\\r\"" => Some("U+000D"),
            "invalid-lemma-\"a\\u{7f}\"" => Some("U+007F"),
            _ => None,
        };
        let ok = match (&a.expected, control) {
            (Ok(()), _) => actual.ok(),
            (Err(_), Some(code)) => {
                actual.rc == Some(1)
                    && actual.stderr
                        == format!(
                            "ui: viewmodel: alpha unsupported control {code} in vocabulary.tsv\n"
                        )
            }
            (Err(why), None) => {
                actual.rc == Some(1)
                    && actual.stderr == format!("ui: viewmodel: alpha vocabulary.tsv: {why}\n")
            }
        };
        checks.require(
            ok,
            format!(
                "{} alias expected {:?}; {}",
                a.id,
                a.expected,
                actual.describe()
            ),
        );
    }
    checks.finish();
}

#[test]
fn e00_reference_baselines_and_oracle_properties() {
    let cases = cases();
    assert_eq!(cases.len(), 88, "contract probe census");
    assert_eq!(table_cases().len(), 43, "table plant census");
    assert_eq!(aliases().len(), 1852, "generated alias/value domain census");
    let sums = fs::read_to_string(fixture().join("c1/SHA256SUMS")).unwrap();
    let green: Vec<_> = cases.iter().filter(|c| c.expect == "ok").collect();
    assert_eq!(
        green.len(),
        sums.lines().count(),
        "C1 baseline digest census"
    );
    for c in green {
        let old = baseline(c);
        let digest = sha(old.as_bytes());
        assert!(
            sums.lines()
                .any(|l| l == format!("{digest}  {}.pl.in", c.id)),
            "{} C1 bytes",
            c.id
        );
        let predicted = expected(c);
        assert_eq!(
            model::erase(&predicted.replacen(QUALIFIER_DECLARATIONS, "", 1), &old),
            old,
            "{} erase(insert(C1)) = C1",
            c.id
        );
        if c.mode == "doc" {
            let without_rows = model::annotate(&old, None).0;
            assert_eq!(
                model::erase(&without_rows, &old),
                old,
                "{} C1 parser/printer round trip",
                c.id
            );
        }
        if let Some(table) = &c.table {
            assert_eq!(model::table_verdict(table), Ok(()));
            assert_eq!(model::table_verdict(&model::filtered(table)), Ok(()));
        }
    }
    assert_eq!(model::table_verdict(model::HEADER), Err("no rows".into()));
    assert_eq!(
        model::table_verdict("strength\tx\tstrong\n"),
        Err("header".into())
    );
    assert_eq!(
        model::table_verdict(&format!("{}strength\tx\tstrong", model::HEADER)),
        Err("final newline".into())
    );
}

#[test]
fn e01_table_value_domains() {
    table_row("E1");
}
#[test]
fn e02_table_grade_noun_conflicts_and_alias_property() {
    table_row("E2");
}
#[test]
fn e03_table_preference_adjective_conflicts_and_alias_property() {
    table_row("E3");
}
#[test]
fn e04_table_alternative_preposition_conflicts_and_alias_property() {
    table_row("E4");
}
#[test]
fn e05_table_validation_precedence() {
    table_row("E5");
}
#[test]
fn e06_grade_both_dimensions_exact() {
    compiler_row("E6");
}
#[test]
fn e07_grade_domains_and_earlier_bindings() {
    compiler_row("E7");
}
#[test]
fn e08_grade_duplicate_dimensions_reject() {
    compiler_row("E8");
}
#[test]
fn e09_grade_nonroot_positions_stay_primitive() {
    compiler_row("E9");
}
#[test]
fn e10_preference_bare_and_attributive() {
    compiler_row("E10");
}
#[test]
fn e11_preference_transitive_and_passive() {
    compiler_row("E11");
}
#[test]
fn e12_preference_combined_copula() {
    compiler_row("E12");
}
#[test]
fn e13_preference_entity_copula() {
    compiler_row("E13");
}
#[test]
fn e14_preference_antecedent_on_every_clause() {
    compiler_row("E14");
}
#[test]
fn e15_preference_scopes_and_bound_endpoints() {
    compiler_row("E15");
}
#[test]
fn e16_preference_invalid_shapes_reject() {
    compiler_row("E16");
}
#[test]
fn e17_preference_event_local_alternatives() {
    compiler_row("E17");
}
#[test]
fn e18_first_error_order_and_insertion_points() {
    compiler_row("E18");
}

#[test]
fn e19_every_green_projection_erases_to_c1() {
    let results = results();
    let mut checks = Checks::new("E19");
    for c in cases()
        .iter()
        .filter(|c| c.mode != "query" && c.expect == "ok")
    {
        let actual = &results.compiler[&c.id];
        checks.require(
            actual.ok(),
            format!("{} projection compile: {}", c.id, actual.describe()),
        );
        if actual.ok() {
            let old = baseline(c);
            let erased = model::erase(&actual.stdout.replacen(QUALIFIER_DECLARATIONS, "", 1), &old);
            checks.require(
                erased == old,
                format!("{} erased projection: {}", c.id, difference(&old, &erased)),
            );
        }
    }
    checks.finish();
}

#[test]
fn e20_table_absence_old_kinds_and_unused_rows() {
    compiler_row("E20");
}
#[test]
fn e21_question_identity_and_no_s1_goals() {
    compiler_row("E21");
}

#[test]
fn e22_all_documents_have_ordered_17_indicator_block() {
    let results = results();
    let mut checks = Checks::new("E22");
    let expected = model::declarations() + QUALIFIER_DECLARATIONS;
    let mut documents: Vec<_> = cases()
        .into_iter()
        .filter(|c| c.mode == "doc" && c.expect == "ok")
        .collect();
    documents.sort_by_key(|c| c.table.is_some());
    for c in &documents {
        let actual = &results.compiler[&c.id];
        checks.require(
            actual.ok(),
            format!("{} declaration compile: {}", c.id, actual.describe()),
        );
        if actual.ok() {
            let declarations: String = actual
                .stdout
                .lines()
                .filter(|l| l.starts_with(":- "))
                .map(|l| format!("{l}\n"))
                .collect();
            checks.require(
                declarations == expected,
                format!(
                    "{} declaration block: {}",
                    c.id,
                    difference(&expected, &declarations)
                ),
            );
        }
    }
    checks.finish();
}
