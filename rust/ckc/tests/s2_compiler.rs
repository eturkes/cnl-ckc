// S2 P2/P3: contract-only oracle + frozen S1 bytes; CKC_S2_TEST_BIN selects red replay.
#[path = "../../../tests/s2-compiler/model.rs"]
mod model;
#[path = "support/nt_projection.rs"]
mod nt_projection;
#[path = "../../../tests/bf/reorder.rs"]
mod reorder;

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

const TABLE_PATH: &str = "guidelines/cdc-2022-opioid/vocabulary.tsv";
const ALIAS_PATH: &str = "guidelines/alpha/vocabulary.tsv";
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
    root().join("tests/s2-compiler")
}
fn program() -> PathBuf {
    std::env::var_os("CKC_S2_TEST_BIN")
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
        let p = root().join(format!("rust/target/s2-compiler/{}", std::process::id()));
        assert!(!p.exists(), "scratch collision: {}", p.display());
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.0) {
            eprintln!("S2 cleanup {}: {e}", self.0.display());
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
    fn reject(&self, class: &str, detail: Option<&str>) -> bool {
        let rc = if matches!(class, "unsupported" | "proof") {
            1
        } else {
            2
        };
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
            version.ok() && version.stdout.contains("version 10.0.2 "),
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
                format!("working_directory(_, '{parser}'), [fit_to_plp], halt."),
                "-t".into(),
                "halt(9)".into(),
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
                let p = cwd.join("vocabulary.tsv");
                write(&p, t);
                p.to_str().unwrap().to_string()
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
                .map(|s| (*s).into()),
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
    expect: String,
    plans: Vec<model::Plan>,
    ace: String,
}
impl Case {
    fn green(&self) -> bool {
        matches!(self.expect.as_str(), "ok" | "naf-qualified")
    }
}
fn cases() -> Vec<Case> {
    fs::read_to_string(fixture().join("cases.tsv"))
        .unwrap()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(|l| {
            let f: Vec<_> = l.split('\t').collect();
            assert_eq!(f.len(), 7, "compiler case fields: {l}");
            Case {
                row: f[0].into(),
                id: f[1].into(),
                mode: f[2].into(),
                table: (f[3] != "-")
                    .then(|| fs::read_to_string(fixture().join(format!("{}.tsv", f[3]))).unwrap()),
                expect: f[4].into(),
                plans: model::plans(
                    f[5],
                    &fs::read_to_string(fixture().join(format!("s1/{}.drs.in", f[1]))).unwrap(),
                ),
                ace: format!("{}\n", f[6].replace("\\n", "\n")),
            }
        })
        .collect()
}
fn frozen(c: &Case, suffix: &str) -> String {
    fs::read_to_string(fixture().join(format!("s1/{}.{suffix}", c.id))).unwrap()
}
// The frozen S1 artifacts predate binding-first bodies (contract bf).
fn expected(c: &Case, table: Option<&str>) -> String {
    reorder::document(&model::rebind(
        &model::annotate(&frozen(c, "pl.in"), table, &c.plans),
        &c.ace,
        &sha(c.ace.as_bytes()),
        table.map(|t| sha(t.as_bytes())).as_deref(),
    ))
}

struct Plant {
    row: String,
    id: String,
    table: String,
    expected: String,
}
fn plants() -> Vec<Plant> {
    let dir = root().join("tests/check-mutants");
    fs::read_to_string(dir.join("s2.tsv"))
        .unwrap()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(|l| {
            let f: Vec<_> = l.split('\t').collect();
            assert_eq!(f.len(), 3);
            let table =
                fs::read_to_string(dir.join(format!("{}.vocabulary.tsv.in", f[1]))).unwrap();
            assert_eq!(
                model::table_verdict(&table),
                Err(f[2].into()),
                "plant oracle {}",
                f[1]
            );
            let expected = fs::read_to_string(dir.join(format!("{}.expect", f[1]))).unwrap();
            assert_eq!(
                expected,
                format!("ckc: vocabulary: {TABLE_PATH}: {}\n", f[2])
            );
            Plant {
                row: f[0].into(),
                id: f[1].into(),
                table,
                expected,
            }
        })
        .collect()
}
struct Alias {
    row: &'static str,
    id: String,
    table: String,
    expected: Result<(), String>,
    compile: bool,
}
fn aliases() -> Vec<Alias> {
    let old = [
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
        ("qualifier", "usuality"),
    ];
    let mut out = Vec::new();
    let mut add = |row, id: String, data: Vec<String>, compile| {
        let table = format!("{}{}\n", model::HEADER, data.join("\n"));
        out.push(Alias {
            row,
            id,
            expected: model::table_verdict(&table),
            table,
            compile,
        });
    };
    for (a, alias) in ["x", "hyphen-word", "é", "Ω", "a&<b>", "'", "x\\y", "#lemma"]
        .iter()
        .enumerate()
    {
        for (kind, value) in old {
            for equal in [true, false] {
                for reverse in [false, true] {
                    let other = if equal { alias } else { "distinct-alias" };
                    let mut data = vec![
                        format!("qualifier\t{alias}\tmanner"),
                        format!("{kind}\t{other}\t{value}"),
                    ];
                    if reverse {
                        data.reverse();
                    }
                    let row = match (kind, equal) {
                        ("qualifier", true) => "T2b",
                        ("approximation" | "preference", true) => "T2c",
                        _ => "T2e",
                    };
                    add(
                        row,
                        format!("alias-{a}-{kind}-{equal}-{reverse}"),
                        data,
                        a == 0 && row == "T2e",
                    );
                }
            }
        }
        for kind in ["spacing", "window"] {
            for reverse in [true, false] {
                let mut data = vec![
                    format!("qualifier\t{alias}\tmanner"),
                    format!("{kind}\tframe-prep\t{alias}"),
                ];
                if reverse {
                    data.reverse();
                }
                add(
                    "T2e",
                    format!("frame-noun-{a}-{kind}-{reverse}"),
                    data,
                    a == 0,
                );
            }
        }
        for class in model::CLASSES {
            add(
                "T2a",
                format!("class-{a}-{class}"),
                vec![format!("qualifier\t{alias}\t{class}")],
                false,
            );
        }
        for bad in ["", "INVALID", "Usually", "manner force", "probability"] {
            add(
                "T2a",
                format!("bad-{a}-{bad}"),
                vec![format!("qualifier\t{alias}\t{bad}")],
                false,
            );
            for (kind, value) in [
                ("qualifier", "usuality"),
                ("preference", "preferred"),
                ("approximation", "about"),
            ] {
                add(
                    "T2d",
                    format!("value-before-{a}-{kind}-{bad}"),
                    vec![
                        format!("{kind}\t{alias}\t{value}"),
                        format!("qualifier\t{alias}\t{bad}"),
                    ],
                    false,
                );
            }
        }
        for row in [
            format!("#qualifier\t{alias}\tmanner"),
            "#qualifier\t\tINVALID\textra".into(),
            "qualifier\t".into(),
            "qualifier\tbad lemma\tINVALID".into(),
        ] {
            add("T2f", format!("grammar-{a}-{row:?}"), vec![row], false);
        }
        add(
            "T2f",
            format!("comment-second-{a}"),
            vec![format!("qualifier\t{alias}\tmanner"), "#\t\t".into()],
            false,
        );
    }
    for first in model::CLASSES {
        for second in model::CLASSES {
            add(
                "T2b",
                format!("class-pair-{first}-{second}"),
                vec![
                    format!("qualifier\tx\t{first}"),
                    format!("qualifier\tx\t{second}"),
                ],
                false,
            );
        }
    }
    for (i, lemma) in ["", "a b", "a\r", "a\u{7f}", "a\u{1f}"].iter().enumerate() {
        add(
            "T2f",
            format!("invalid-lemma-{i}"),
            vec![format!("qualifier\t{lemma}\tmanner")],
            false,
        );
    }
    out
}

enum Job {
    Compile(usize),
    Plant(usize),
    Alias(usize),
    Reclass(usize, usize),
}
struct Results {
    compiler: BTreeMap<String, Observed>,
    erased: BTreeMap<String, Observed>,
    checks: BTreeMap<String, Observed>,
    plant_compiler: BTreeMap<String, Observed>,
    aliases: BTreeMap<String, Observed>,
    alias_compiler: BTreeMap<String, Observed>,
    reclass: BTreeMap<String, Observed>,
}
fn results() -> &'static Results {
    static RESULTS: OnceLock<Results> = OnceLock::new();
    RESULTS.get_or_init(|| {
        let start = Instant::now();
        let scratch = Scratch::new();
        let compiler = Compiler::new(&scratch.0);
        let cases = cases(); let plants = plants(); let aliases = aliases();
        let head = git(&root(), &["rev-parse", "HEAD"]);
        let binary = program(); assert!(binary.is_file(), "CKC_S2_TEST_BIN {}", binary.display());
        let jobs: Vec<_> = (0..cases.len()).map(Job::Compile).chain((0..plants.len()).map(Job::Plant))
            .chain((0..aliases.len()).map(Job::Alias))
            .chain(cases.iter().enumerate().filter(|(_, c)| c.row == "T3l").flat_map(|(i, _)| (0..model::CLASSES.len()).map(move |k| Job::Reclass(i, k)))).collect();
        let found = Mutex::new(Results { compiler: BTreeMap::new(), erased: BTreeMap::new(), checks: BTreeMap::new(), plant_compiler: BTreeMap::new(), aliases: BTreeMap::new(), alias_compiler: BTreeMap::new(), reclass: BTreeMap::new() });
        let next = AtomicUsize::new(0);
        let timing: [AtomicU64; 4] = std::array::from_fn(|_| AtomicU64::new(0));
        let workers = std::thread::available_parallelism().map_or(4, |n| n.get().min(4));
        std::thread::scope(|scope| {
            for worker in 0..workers {
                let (scratch, compiler, cases, plants, aliases, jobs, next, found, head, binary, timing) = (&scratch, &compiler, &cases, &plants, &aliases, &jobs, &next, &found, &head, &binary, &timing);
                scope.spawn(move || {
                    let work = scratch.0.join(format!("worker-{worker}")); fs::create_dir_all(&work).unwrap();
                    let ui = work.join("alias-tree"); copy_tree(&root().join("tests/ui/green/q12-temporal-words/tree"), &ui);
                    git(&work, &["clone", "-q", "--shared", "--no-checkout", root().to_str().unwrap(), "check-tree"]);
                    let tree = work.join("check-tree"); git(&tree, &["checkout", "-q", "--detach", head.trim()]);
                    while let Some(job) = jobs.get(next.fetch_add(1, Ordering::Relaxed)) {
                        let begin = Instant::now();
                        let category = match *job {
                            Job::Compile(i) => {
                                let c = &cases[i];
                                let out = compiler.compile(&work.join("compiler"), &c.id, &c.mode, &c.ace, c.table.as_deref());
                                let erased = c.green().then(|| compiler.compile(&work.join("erased"), &c.id, &c.mode, &frozen(c, "erased.ace.in"), c.table.as_deref().map(model::filtered).as_deref()));
                                let mut r = found.lock().unwrap(); r.compiler.insert(c.id.clone(), out);
                                if let Some(erased) = erased { r.erased.insert(c.id.clone(), erased); }
                                0
                            }
                            Job::Plant(i) => {
                                let p = &plants[i]; write(&tree.join(TABLE_PATH), &p.table);
                                let checked = command(binary, &tree, &["check".into()], b"");
                                git(&tree, &["reset", "-q", "--hard", head.trim()]); git(&tree, &["clean", "-q", "-f", "-d", "-x"]);
                                assert!(git(&tree, &["status", "--porcelain", "--ignored"]).is_empty(), "{} clone reset", p.id);
                                let compiled = compiler.compile(&work.join("table"), "s2-table", "doc", "A man sleeps.\n", Some(&p.table));
                                let mut r = found.lock().unwrap(); r.checks.insert(p.id.clone(), checked); r.plant_compiler.insert(p.id.clone(), compiled); 1
                            }
                            Job::Alias(i) => {
                                let a = &aliases[i]; write(&ui.join(ALIAS_PATH), &a.table);
                                let out = command(binary, &work, &["ui".into(), "check".into(), ui.to_str().unwrap().into()], b"");
                                let compiled = a.compile.then(|| compiler.compile(&work.join("alias-compiler"), "s2-alias", "doc", "A man sleeps.\n", Some(&a.table)));
                                let mut r = found.lock().unwrap(); r.aliases.insert(a.id.clone(), out);
                                if let Some(c) = compiled { r.alias_compiler.insert(a.id.clone(), c); } 2
                            }
                            Job::Reclass(i, k) => {
                                let c = &cases[i]; let class = model::CLASSES[k];
                                let table = format!("{}qualifier\tcarefully\t{class}\n", model::HEADER);
                                let out = compiler.compile(&work.join("reclass"), &c.id, &c.mode, &c.ace, Some(&table));
                                found.lock().unwrap().reclass.insert(format!("{}-{class}", c.id), out); 3
                            }
                        };
                        timing[category].fetch_add(u64::try_from(begin.elapsed().as_millis()).unwrap(), Ordering::Relaxed);
                    }
                });
            }
        });
        eprintln!("S2 runtime: {:.3}s; {} compiler probes + erased controls, {} check/compiler plants, {} aliases, 14 reclassifications; {workers} workers", start.elapsed().as_secs_f64(), cases.len(), plants.len(), aliases.len());
        eprintln!("S2 worker-ms: compiler={} plants={} aliases={} reclassification={}", timing[0].load(Ordering::Relaxed), timing[1].load(Ordering::Relaxed), timing[2].load(Ordering::Relaxed), timing[3].load(Ordering::Relaxed));
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
            "{}: {} of {} assertions fail:\n{}",
            self.row,
            self.failures.len(),
            self.count,
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
fn compiler_row(row: &'static str, predicate: impl Fn(&Case) -> bool) {
    let results = results();
    let mut checks = Checks::new(row);
    for c in cases().iter().filter(|c| c.row == row && predicate(c)) {
        let actual = &results.compiler[&c.id];
        if c.green() {
            checks.require(
                actual.ok(),
                format!("{} compile: {}", c.id, actual.describe()),
            );
            if actual.ok() {
                let predicted = expected(c, c.table.as_deref());
                let projected = nt_projection::erase(&actual.stdout, &predicted);
                checks.require(
                    projected == predicted,
                    format!(
                        "{} exact contract projection: {}",
                        c.id,
                        difference(&predicted, &projected)
                    ),
                );
            }
        } else {
            checks.require(
                actual.reject("unsupported", Some(&c.expect)),
                format!(
                    "{} expected ace_to_pl_error(unsupported,{}).; {}",
                    c.id,
                    c.expect,
                    actual.describe()
                ),
            );
        }
    }
    assert!(checks.count > 0, "empty row {row}");
    checks.finish();
}
fn table_row(row: &'static str) {
    let results = results();
    let mut checks = Checks::new(row);
    for p in plants().iter().filter(|p| p.row == row) {
        let actual = &results.checks[&p.id];
        checks.require(
            actual.rc == Some(1) && format!("{}\n", actual.violation()) == p.expected,
            format!(
                "{} check expected {:?}; {}",
                p.id,
                p.expected.trim_end(),
                actual.describe()
            ),
        );
        let compiled = &results.plant_compiler[&p.id];
        checks.require(
            compiled.reject("vocabulary_load", None),
            format!("{} compiler vocabulary_load: {}", p.id, compiled.describe()),
        );
    }
    for a in aliases().iter().filter(|a| a.row == row) {
        let actual = &results.aliases[&a.id];
        let control = match a.id.as_str() {
            "invalid-lemma-2" => Some("U+000D"),
            "invalid-lemma-3" => Some("U+007F"),
            "invalid-lemma-4" => Some("U+001F"),
            _ => None,
        };
        let matched = match (&a.expected, control) {
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
            matched,
            format!("{} expected {:?}; {}", a.id, a.expected, actual.describe()),
        );
        if a.compile {
            let c = &results.alias_compiler[&a.id];
            checks.require(
                c.ok(),
                format!("{} legal compiler alias: {}", a.id, c.describe()),
            );
        }
    }
    assert!(checks.count > 0);
    checks.finish();
}

#[test]
fn t00_reference_baselines_and_oracle_properties() {
    let cases = cases();
    assert_eq!(cases.len(), 152);
    assert_eq!(cases.iter().filter(|c| c.green()).count(), 110);
    assert_eq!(plants().len(), 15);
    assert_eq!(aliases().len(), 726);
    let dir = fixture().join("s1");
    let sums = fs::read_to_string(dir.join("SHA256SUMS")).unwrap();
    assert_eq!(
        sums.lines().count(),
        fs::read_dir(&dir).unwrap().count() - 1,
        "frozen artifact census"
    );
    for line in sums.lines() {
        let (digest, name) = line.split_once("  ").unwrap();
        assert_eq!(
            sha(&fs::read(dir.join(name)).unwrap()),
            digest,
            "frozen {name}"
        );
    }
    for c in &cases {
        let drs = frozen(c, "drs.in");
        assert!(drs.contains("messages([])."), "{} APE input", c.id);
        if !c.green() {
            continue;
        }
        let old = reorder::document(&frozen(c, "pl.in"));
        let predicted = expected(c, c.table.as_deref());
        assert_eq!(
            model::erase(&predicted, &old),
            old,
            "{} erase(insert(S1)), binding-first",
            c.id
        );
        let planned = c
            .plans
            .iter()
            .map(|p| {
                format!(
                    "{}:{}:{}:{}:{}:{}",
                    p.sentence, p.scope, p.verb, p.nth, p.place, p.word
                )
            })
            .collect();
        assert_eq!(
            model::drs_plans(&drs),
            planned,
            "{} DRS-derived carrier positions",
            c.id
        );
        if let Some(table) = &c.table {
            assert_eq!(model::table_verdict(table), Ok(()));
            assert_eq!(model::table_verdict(&model::filtered(table)), Ok(()));
        }
    }
    assert_eq!(model::table_verdict(model::HEADER), Err("no rows".into()));
    assert_eq!(
        model::table_verdict("qualifier\tx\tmanner\n"),
        Err("header".into())
    );
    assert_eq!(
        model::table_verdict(&format!("{}qualifier\tx\tmanner", model::HEADER)),
        Err("final newline".into())
    );
    assert_eq!(
        model::drs_adverbs(&frozen(
            cases.iter().find(|c| c.id == "two-pre-adverbs").unwrap(),
            "drs.in"
        )),
        ["especially", "carefully"]
    );
    for (id, order) in [
        (
            "post-adverb-before-time",
            [
                "guideline_event",
                "guideline_pp",
                "guideline_interval",
                "guideline_qualifier",
            ],
        ),
        (
            "post-adverb-after-time",
            [
                "guideline_event",
                "guideline_qualifier",
                "guideline_pp",
                "guideline_interval",
            ],
        ),
    ] {
        let c = cases.iter().find(|c| c.id == id).unwrap();
        let predicted = expected(c, c.table.as_deref());
        let actual: Vec<_> = predicted
            .lines()
            .filter_map(|line| line.split_once('('))
            .map(|(name, _)| name)
            .filter(|name| order.contains(name))
            .collect();
        assert_eq!(
            actual, order,
            "{id}: DRS order includes the pp's annotation block"
        );
    }
}
#[test]
fn t2a_table_class_domain() {
    table_row("T2a");
}
#[test]
fn t2b_table_duplicate_qualifier_both_orders() {
    table_row("T2b");
}
#[test]
fn t2c_table_adjective_collision_both_orders() {
    table_row("T2c");
}
#[test]
fn t2d_table_value_precedes_duplicate() {
    table_row("T2d");
}
#[test]
fn t2e_table_noun_preposition_aliases_load() {
    table_row("T2e");
}
#[test]
fn t2f_generated_alias_and_comment_laws() {
    table_row("T2f");
}
#[test]
fn t3a_each_class_preverb_and_final() {
    compiler_row("T3a", |_| true);
}
#[test]
fn t3b_multiple_adverbs_copula_and_passive() {
    compiler_row("T3b", |_| true);
}
#[test]
fn t3c_copula_carrier_and_property_only() {
    compiler_row("T3c", |_| true);
}
#[test]
fn t3d_all_contexts_and_antecedent_variants() {
    compiler_row("T3d", |_| true);
}
#[test]
fn t3e_horn_local_event_ordinals_and_dependencies() {
    compiler_row("T3e", |_| true);
}
#[test]
fn t3f_degree_rejects() {
    compiler_row("T3f", |_| true);
}
#[test]
fn t3g_assertion_guard_and_exemptions() {
    compiler_row("T3g", |_| true);
}
#[test]
fn t3h_first_error_pipeline_order() {
    compiler_row("T3h", |_| true);
}
#[test]
fn t3i_unmapped_adverb_bytes_already_passing() {
    compiler_row("T3i", |c| c.id != "unmapped-other-qualifier");
}
#[test]
fn t3i_unmapped_with_unrelated_qualifier_row() {
    compiler_row("T3i", |c| c.id == "unmapped-other-qualifier");
}
#[test]
fn t3j_same_lemma_distinct_event_identity() {
    compiler_row("T3j", |_| true);
}
#[test]
fn t3k_time_preference_grade_coexistence() {
    compiler_row("T3k", |_| true);
}
#[test]
fn t3l_reclassification_invariance_and_guard() {
    let results = results();
    let mut checks = Checks::new("T3l");
    for c in cases().iter().filter(|c| c.row == "T3l") {
        for class in model::CLASSES {
            let actual = &results.reclass[&format!("{}-{class}", c.id)];
            let asserted = c.id == "reclass-root"
                && ["usuality", "ideality", "possibility", "applicability"].contains(&class);
            if asserted {
                checks.require(
                    actual.reject("unsupported", Some("qualifier_shape(asserted,1)")),
                    format!("{} {class} guard: {}", c.id, actual.describe()),
                );
            } else {
                checks.require(
                    actual.ok(),
                    format!("{} {class}: {}", c.id, actual.describe()),
                );
                if actual.ok() {
                    let table = format!("{}qualifier\tcarefully\t{class}\n", model::HEADER);
                    let predicted = expected(c, Some(&table));
                    let projected = nt_projection::erase(&actual.stdout, &predicted);
                    checks.require(
                        projected == predicted,
                        format!(
                            "{} {class} class-only reclassification: {}",
                            c.id,
                            difference(&predicted, &projected)
                        ),
                    );
                }
            }
        }
    }
    checks.finish();
}
#[test]
fn t3m_every_green_erases_to_s1_and_naf_obligation_boundary() {
    let results = results();
    let mut checks = Checks::new("T3m");
    for c in cases().iter().filter(|c| c.green()) {
        let actual = &results.compiler[&c.id];
        checks.require(
            actual.ok(),
            format!("{} qualified compile: {}", c.id, actual.describe()),
        );
        if actual.ok() {
            if c.expect == "naf-qualified" {
                let predicted = expected(c, c.table.as_deref());
                let projected = nt_projection::erase(&actual.stdout, &predicted);
                checks.require(
                    projected == predicted,
                    format!(
                        "{} exact NAF qualification: {}",
                        c.id,
                        difference(&predicted, &projected)
                    ),
                );
            }
            let old = reorder::document(&frozen(c, "pl.in"));
            let projected = nt_projection::erase(&actual.stdout, &expected(c, c.table.as_deref()));
            let erased = model::erase(&projected, &old);
            checks.require(
                erased == old,
                format!(
                    "{} erased ordered projection: {}",
                    c.id,
                    difference(&old, &erased)
                ),
            );
        }
        let erased = &results.erased[&c.id];
        let rc: i32 = frozen(c, "erased.rc").trim().parse().unwrap();
        if rc == 0 {
            checks.require(
                erased.ok(),
                format!("{} erased compile: {}", c.id, erased.describe()),
            );
            if erased.ok() {
                let old = reorder::document(&frozen(c, "pl.in"));
                let projected = nt_projection::erase(&erased.stdout, &old);
                let normalized = model::erase(&projected, &old);
                checks.require(
                    normalized == old,
                    format!(
                        "{} live erased S1 identity: {}",
                        c.id,
                        difference(&old, &normalized)
                    ),
                );
            }
        } else {
            checks.require(
                erased.rc == Some(rc)
                    && erased.stdout.is_empty()
                    && erased.stderr == frozen(c, "erased.stderr"),
                format!("{} erased-side refusal: {}", c.id, erased.describe()),
            );
            if c.id == "qualified-naf-contradiction" {
                checks.require(
                    erased.reject("proof", Some("underivable_obligation(1,variant(1))")),
                    format!("qualified NAF boundary: {}", erased.describe()),
                );
            } else {
                assert_eq!(
                    c.id, "two-pre-adverbs",
                    "enumerate every noncompiling erasure"
                );
            }
        }
    }
    checks.finish();
}
#[test]
fn t3n_table_absence_and_frozen_s1_identity() {
    compiler_row("T3n", |_| true);
}
#[test]
fn t3o_adverb_question_bytes_already_passing() {
    compiler_row("T3o", |c| {
        c.id == "query-adverb-none" || c.id == "query-adverb-old"
    });
}
#[test]
fn t3o_question_table_digest_and_primitive_identity() {
    compiler_row("T3o", |c| {
        c.id != "query-adverb-none" && c.id != "query-adverb-old"
    });
}
