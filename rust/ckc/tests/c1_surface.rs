// C1 D2–D6/D9–D10 + S1 D1/D10 + S2 D1/P9: contract-derived probes; CKC_C1_TEST_BIN selects red replay.
#[path = "support/nt_projection.rs"]
mod nt_projection;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::os::unix::fs::{MetadataExt, PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};

const HEADER: &str = "# format: kind\tlemma\tvalue\n";
const GID: &str = "cdc-2022-opioid";
const TABLE_PATH: &str = "guidelines/cdc-2022-opioid/vocabulary.tsv";
const RETIRED_PATH: &str = "guidelines/cdc-2022-opioid/temporal.tsv";
const ULEX: &str = "prep(per,per).\nprep(for,for).\n";
const ALL_ROWS: &str = "unit\thour\thour\nunit\tday\tday\nunit\tweek\tweek\nunit\tsecond\tsecond\nunit\tminute\tminute\nunit\tmonth\tmonth\nunit\tyear\tyear\nrelation\tfor\tduration\nrelation\tbefore\tbefore\nrelation\tafter\tafter\nrelation\twithin\twithin\nspacing\tat\tinterval\nwindow\tduring\twindow\nfrequency\tper\tperiod\napproximation\tapproximate\tabout\nrange\tto\tminimum\n";
const OLD_KIND_HEADER: &str = "# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma) | window (value: frame noun lemma) | frequency (value: period) | approximation (value: about) | range (value: minimum)\n";
const INDICATORS: [(&str, usize); 19] = [
    ("schema_version", 1),
    ("document", 4),
    ("entity", 4),
    ("cardinality", 5),
    ("event", 3),
    ("arg", 4),
    ("pp", 4),
    ("property", 4),
    ("operator", 3),
    ("interval", 6),
    ("recurrence", 4),
    ("frequency", 5),
    ("order", 4),
    ("recurrence_window", 7),
    ("range", 3),
    ("grade", 5),
    ("preference", 4),
    ("qualifier", 4),
    ("unit", 3),
];
const GIT_ENV: [(&str, &str); 9] = [
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_CONFIG_SYSTEM", "/dev/null"),
    ("GIT_DEFAULT_HASH", "sha1"),
    ("GIT_AUTHOR_NAME", "fixture"),
    ("GIT_AUTHOR_EMAIL", "fixture@localhost"),
    ("GIT_COMMITTER_NAME", "fixture"),
    ("GIT_COMMITTER_EMAIL", "fixture@localhost"),
    ("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+00:00"),
    ("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+00:00"),
];
static NEXT: AtomicU64 = AtomicU64::new(0);

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn program() -> PathBuf {
    std::env::var_os("CKC_C1_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}

fn write(path: &Path, bytes: impl AsRef<[u8]>) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn sha(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        assert!(!entry.file_type().unwrap().is_symlink());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &to.join(entry.file_name()));
        } else {
            fs::copy(entry.path(), to.join(entry.file_name())).unwrap();
        }
    }
}

fn remove(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_dir() {
            fs::remove_dir_all(path).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
    }
}

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let base = root().join("rust/target/c1-surface");
        fs::create_dir_all(&base).unwrap();
        loop {
            let p = base.join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&p) {
                Ok(()) => return Self(p),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
                Err(e) => panic!("scratch: {e}"),
            }
        }
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.0) {
            eprintln!("C1 cleanup {}: {e}", self.0.display());
        }
    }
}

fn command(bin: &Path, cwd: &Path, args: &[String], input: &[u8], env: &[(&str, &Path)]) -> Output {
    let mut cmd = Command::new("timeout");
    cmd.args(["--verbose", "--kill-after=2", "180"])
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
        .stderr(Stdio::piped());
    for (key, value) in env {
        cmd.env(key, value);
    }
    let mut child = cmd.spawn().unwrap();
    if !input.is_empty() {
        child.stdin.take().unwrap().write_all(input).unwrap();
    } else {
        drop(child.stdin.take());
    }
    child.wait_with_output().unwrap()
}

fn git(cwd: &Path, args: &[&str]) -> Vec<u8> {
    let out = command(
        Path::new("git"),
        cwd,
        &args.iter().map(|s| (*s).into()).collect::<Vec<_>>(),
        b"",
        &[],
    );
    assert!(out.status.success(), "git {args:?}: {}", text(&out.stderr));
    out.stdout
}

fn ckc(tree: &Path, args: &[&str]) -> Output {
    command(
        &program(),
        tree,
        &args.iter().map(|s| (*s).into()).collect::<Vec<_>>(),
        b"",
        &[],
    )
}

fn diagnostic(out: &Output) -> String {
    if !out.stderr.is_empty() {
        text(&out.stderr)
    } else {
        text(&out.stdout)
            .lines()
            .last()
            .map_or(String::new(), |s| format!("{s}\n"))
    }
}

fn observed(out: &Output) -> String {
    format!(
        "rc={:?} diagnostic={:?}",
        out.status.code(),
        diagnostic(out).trim_end()
    )
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
    fn success(&mut self, out: &Output, label: &str) -> bool {
        let ok = out.status.success() && out.stderr.is_empty();
        self.require(ok, format!("{label}: {}", observed(out)));
        ok
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

struct CheckTree {
    _scratch: Option<Scratch>,
    dir: PathBuf,
    tree: PathBuf,
    swipl: PathBuf,
}
impl CheckTree {
    fn new() -> Self {
        let scratch = Scratch::new();
        let dir = scratch.0.clone();
        Self::build(Some(scratch), dir)
    }
    // A clone in a fixed directory, emptied first (the run-wide table pool).
    fn at(dir: PathBuf) -> Self {
        remove(&dir);
        fs::create_dir_all(&dir).unwrap();
        Self::build(None, dir)
    }
    fn build(scratch: Option<Scratch>, dir: PathBuf) -> Self {
        let tree = dir.join("tree");
        let head = text(&git(&root(), &["rev-parse", "HEAD"]));
        git(
            &dir,
            &[
                "clone",
                "-q",
                "--shared",
                "--no-checkout",
                root().to_str().unwrap(),
                "tree",
            ],
        );
        git(&tree, &["checkout", "-q", "--detach", head.trim()]);
        let swipl = dir.join("swipl-stop");
        write(
            &swipl,
            "#!/bin/sh\n[ \"$1\" = --version ] && { echo 'SWI-Prolog version 9.2.9 for x86_64-linux'; exit 0; }\necho 'c1-surface: stage reached' >&2\nexit 3\n",
        );
        fs::set_permissions(&swipl, fs::Permissions::from_mode(0o755)).unwrap();
        Self {
            _scratch: scratch,
            dir,
            tree,
            swipl,
        }
    }
    fn table(&self, bytes: &[u8]) {
        remove(&self.tree.join(RETIRED_PATH));
        remove(&self.tree.join(TABLE_PATH));
        write(&self.tree.join(TABLE_PATH), bytes);
    }
    fn run(&self, args: &[&str]) -> Output {
        command(
            &program(),
            &self.tree,
            &args.iter().map(|s| (*s).into()).collect::<Vec<_>>(),
            b"",
            &[("SWIPL", &self.swipl)],
        )
    }
}

fn judge_table(checks: &mut Checks, label: &str, bytes: &[u8], out: &Output) {
    match reference_table(bytes) {
        Ok(rows) => {
            let meter = format!("ckc: vocabulary ok {TABLE_PATH} {rows} rows\n");
            checks.require(
                text(&out.stdout).contains(&meter),
                format!("{label}: missing {meter:?}; {}", observed(out)),
            );
        }
        Err(why) => {
            let expected = format!("ckc: vocabulary: {TABLE_PATH}: {why}\n");
            checks.require(
                out.status.code() == Some(1) && diagnostic(out) == expected,
                format!("{label}: expected rc1 {expected:?}; {}", observed(out)),
            );
        }
    }
}

// Each `ckc check` of a full HEAD clone costs seconds and a test's table cases are independent:
// the table cases of every test share TABLE_WORKERS HEAD clones (checked out once per run in
// rust/target/c1-surface/pool-<k>; a case rewrites the table before its `ckc check`, so a clone
// carries no state between cases), and each test's verdicts fold in case order, so the
// assertions and their failure text are a sequential run's. One pool bounds the concurrent
// checks and checkouts for the whole binary.
const TABLE_WORKERS: usize = 8;

fn table_pool() -> &'static (Mutex<Vec<CheckTree>>, Condvar) {
    static POOL: OnceLock<(Mutex<Vec<CheckTree>>, Condvar)> = OnceLock::new();
    POOL.get_or_init(|| {
        let trees = (0..TABLE_WORKERS)
            .map(|k| CheckTree::at(root().join(format!("rust/target/c1-surface/pool-{k}"))))
            .collect();
        (Mutex::new(trees), Condvar::new())
    })
}

// A pool clone on loan; dropping the loan (a panic included) returns it.
struct Loan(Option<CheckTree>);
impl Loan {
    fn take() -> Self {
        let (free, ready) = table_pool();
        let mut trees = free.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if let Some(tree) = trees.pop() {
                return Self(Some(tree));
            }
            trees = ready.wait(trees).unwrap_or_else(|e| e.into_inner());
        }
    }
}
impl Drop for Loan {
    fn drop(&mut self) {
        let (free, ready) = table_pool();
        free.lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(self.0.take().unwrap());
        ready.notify_one();
    }
}

#[derive(Default)]
struct TableCases(Vec<(String, Vec<u8>)>);
impl TableCases {
    fn add(&mut self, label: &str, bytes: &[u8]) {
        self.0.push((label.into(), bytes.into()));
    }
    fn run(self, checks: &mut Checks) {
        let next = AtomicUsize::new(0);
        let mut outs = std::thread::scope(|s| {
            let workers = (0..TABLE_WORKERS.min(self.0.len()))
                .map(|_| {
                    s.spawn(|| {
                        let mut done = Vec::new();
                        loop {
                            let i = next.fetch_add(1, Ordering::Relaxed);
                            let Some((_, bytes)) = self.0.get(i) else {
                                return done;
                            };
                            let loan = Loan::take();
                            let tree = loan.0.as_ref().unwrap();
                            tree.table(bytes);
                            done.push((i, tree.run(&["check"])));
                        }
                    })
                })
                .collect::<Vec<_>>();
            workers
                .into_iter()
                .flat_map(|w| w.join().unwrap())
                .collect::<Vec<_>>()
        });
        outs.sort_by_key(|(i, _)| *i);
        assert_eq!(outs.len(), self.0.len(), "every table case ran");
        for ((label, bytes), (_, out)) in self.0.iter().zip(&outs) {
            judge_table(checks, label, bytes, out);
        }
    }
}

#[derive(PartialEq, Eq)]
struct Artifact {
    kind: u8,
    inode: u64,
    modified: (i64, i64),
    bytes: Vec<u8>,
}

fn artifact_snapshot(tree: &Path, dist: &Path) -> BTreeMap<PathBuf, Artifact> {
    fn walk(base: &Path, here: &Path, label: &Path, out: &mut BTreeMap<PathBuf, Artifact>) {
        let meta = match fs::symlink_metadata(here) {
            Ok(meta) => meta,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return,
            Err(e) => panic!("artifact snapshot {}: {e}", here.display()),
        };
        if meta.is_dir() {
            for entry in fs::read_dir(here).unwrap() {
                walk(base, &entry.unwrap().path(), label, out);
            }
            return;
        }
        let (kind, bytes) = if meta.is_file() {
            (b'f', fs::read(here).unwrap())
        } else if meta.file_type().is_symlink() {
            (
                b'l',
                fs::read_link(here)
                    .unwrap()
                    .as_os_str()
                    .as_encoded_bytes()
                    .to_vec(),
            )
        } else {
            (b'o', Vec::new())
        };
        // Rewriting identical bytes is still a write; preserve inode and mtime too.
        out.insert(
            label.join(here.strip_prefix(base).unwrap()),
            Artifact {
                kind,
                inode: meta.ino(),
                modified: (meta.mtime(), meta.mtime_nsec()),
                bytes,
            },
        );
    }
    let mut out = BTreeMap::new();
    walk(tree, &tree.join("guidelines"), Path::new("repo"), &mut out);
    walk(
        tree,
        &tree.join("release-manifest.tsv"),
        Path::new("repo"),
        &mut out,
    );
    walk(dist, dist, Path::new("dist"), &mut out);
    out
}

fn artifact_snapshot_controls() {
    let scratch = Scratch::new();
    let tree = scratch.0.join("tree");
    let dist = scratch.0.join("dist");
    let files = [
        tree.join("guidelines/probe/pl/doc.pl"),
        tree.join("release-manifest.tsv"),
        dist.join("artifact.tar.gz"),
    ];
    for path in &files {
        write(path, b"unchanged\n");
    }
    let baseline = artifact_snapshot(&tree, &dist);
    assert_eq!(baseline.len(), 3, "artifact snapshot positive control");
    for path in &files {
        let modified = fs::metadata(path).unwrap().modified().unwrap();
        write(path, b"changed\n");
        fs::File::open(path)
            .unwrap()
            .set_modified(modified)
            .unwrap();
        assert!(
            artifact_snapshot(&tree, &dist) != baseline,
            "changed bytes: {}",
            path.display()
        );
        write(path, b"unchanged\n");
        fs::File::open(path)
            .unwrap()
            .set_modified(modified)
            .unwrap();
        assert!(
            artifact_snapshot(&tree, &dist) == baseline,
            "restored bytes"
        );
        fs::File::open(path)
            .unwrap()
            .set_modified(modified + std::time::Duration::from_secs(1))
            .unwrap();
        assert!(
            artifact_snapshot(&tree, &dist) != baseline,
            "retouched artifact: {}",
            path.display()
        );
        fs::File::open(path)
            .unwrap()
            .set_modified(modified)
            .unwrap();
        let backup = scratch.0.join("backup");
        fs::rename(path, &backup).unwrap();
        assert!(
            artifact_snapshot(&tree, &dist) != baseline,
            "removed artifact"
        );
        write(path, b"unchanged\n");
        fs::File::open(path)
            .unwrap()
            .set_modified(modified)
            .unwrap();
        assert!(
            artifact_snapshot(&tree, &dist) != baseline,
            "byte-identical replacement"
        );
        fs::remove_file(path).unwrap();
        fs::rename(&backup, path).unwrap();
        assert!(
            artifact_snapshot(&tree, &dist) == baseline,
            "restored identity"
        );
    }
    write(&dist.join("extra.tar.gz"), b"new\n");
    assert!(artifact_snapshot(&tree, &dist) != baseline, "new artifact");
}

// Independent D4 row oracle; grammar/precedence from the seed specification.
fn reference_table(bytes: &[u8]) -> Result<usize, String> {
    let body = bytes.strip_prefix(HEADER.as_bytes()).ok_or("header")?;
    if body.is_empty() {
        return Err("no rows".into());
    }
    if !body.ends_with(b"\n") {
        return Err("final newline".into());
    }
    let mut units = BTreeSet::new();
    let mut frames = BTreeSet::new();
    let mut pairs = BTreeSet::new();
    let mut preps = BTreeSet::new();
    let mut adjectives = BTreeSet::new();
    let mut count = 0;
    let lemma = |s: &[u8]| {
        !s.is_empty() && std::str::from_utf8(s).is_ok() && s.iter().all(|b| *b > 32 && *b != 127)
    };
    for (i, line) in body[..body.len() - 1].split(|b| *b == b'\n').enumerate() {
        let f: Vec<_> = line.split(|b| *b == b'\t').collect();
        let why = if line.starts_with(b"#") {
            Some("comment")
        } else if f.len() != 3 {
            Some("field count")
        } else if !lemma(f[1]) {
            Some("lemma")
        } else {
            match f[0] {
                b"unit" => {
                    if ![
                        b"second".as_slice(),
                        b"minute",
                        b"hour",
                        b"day",
                        b"week",
                        b"month",
                        b"year",
                    ]
                    .contains(&f[2])
                    {
                        Some("unit id")
                    } else if units.contains(f[1]) || frames.contains(f[1]) {
                        Some("duplicate noun")
                    } else {
                        units.insert(f[1].to_vec());
                        None
                    }
                }
                b"relation" | b"frequency" | b"range" => {
                    let valid = match f[0] {
                        b"relation" => {
                            [b"duration".as_slice(), b"within", b"after", b"before"].contains(&f[2])
                        }
                        b"frequency" => f[2] == b"period",
                        _ => f[2] == b"minimum",
                    };
                    if !valid {
                        Some(match f[0] {
                            b"relation" => "role id",
                            b"frequency" => "frequency value",
                            _ => "range value",
                        })
                    } else if !preps.insert(f[1].to_vec()) {
                        Some("duplicate preposition")
                    } else {
                        None
                    }
                }
                b"spacing" | b"window" => {
                    if !lemma(f[2]) {
                        Some("frame lemma")
                    } else if units.contains(f[2]) || !pairs.insert((f[1].to_vec(), f[2].to_vec()))
                    {
                        Some("duplicate noun")
                    } else {
                        frames.insert(f[2].to_vec());
                        None
                    }
                }
                b"approximation" => {
                    if f[2] != b"about" {
                        Some("approximation value")
                    } else if !adjectives.insert(f[1].to_vec()) {
                        Some("duplicate adjective")
                    } else {
                        None
                    }
                }
                _ => Some("kind"),
            }
        };
        if let Some(why) = why {
            return Err(format!("{why} at row {}", i + 1));
        }
        count += 1;
    }
    Ok(count)
}

// UI file intake precedes the row parser; these frames are retained by C1.
fn reference_ui_table(bytes: &[u8]) -> Result<usize, String> {
    let raw =
        std::str::from_utf8(bytes).map_err(|_| "file not UTF-8: vocabulary.tsv".to_owned())?;
    if let Some(c) = raw.chars().find(|c| matches!(u32::from(*c), 0x00..=0x08 | 0x0b..=0x1f | 0x7f | 0x202a..=0x202e | 0x2066..=0x2069)) {
        return Err(format!("unsupported control U+{:04X} in vocabulary.tsv", u32::from(c)));
    }
    reference_table(bytes).map_err(|why| format!("vocabulary.tsv: {why}"))
}

#[test]
fn b01_exact_header() {
    let mut cases = TableCases::default();
    let mut checks = Checks::new("B1");
    let valid = format!("{HEADER}unit\thour\thour\n");
    cases.add("exact header", valid.as_bytes());
    for i in 0..HEADER.len() {
        let mut changed = valid.as_bytes().to_vec();
        changed[i] = if changed[i] == b'x' { b'y' } else { b'x' };
        cases.add(&format!("header byte {i}"), &changed);
    }
    for prefix in ["\u{feff}", "\n", " "] {
        cases.add("header prefix", format!("{prefix}{valid}").as_bytes());
    }
    cases.add("CRLF header", valid.replacen('\n', "\r\n", 1).as_bytes());
    cases.add(
        "old two-line header",
        format!("{HEADER}# kind: unit (value: hour)\nunit\thour\thour\n").as_bytes(),
    );
    cases.run(&mut checks);
    checks.finish();
}

#[test]
fn b02_comment_precedence() {
    let mut cases = TableCases::default();
    let mut checks = Checks::new("B2");
    for row in [
        "#",
        "#\t\t",
        "#\tbad lemma\tbad\textra",
        "#unit\thour\tbad",
        "##\u{7f}",
    ] {
        for leading in ["", "unit\thour\thour\n"] {
            cases.add(
                &format!("{row:?} after {leading:?}"),
                format!("{HEADER}{leading}{row}\n").as_bytes(),
            );
        }
    }
    cases.add(
        "earlier row violation before comment",
        format!("{HEADER}unit\thour\tbad\n#\n").as_bytes(),
    );
    cases.run(&mut checks);
    checks.finish();
}

#[test]
fn b03_row_laws_and_order_properties() {
    let mut cases = TableCases::default();
    let mut checks = Checks::new("B3");
    let invalid = [
        "",
        "unit",
        "unit\thour",
        "unit\thour\thour\textra",
        "unit\t\thour",
        "unit\ttwo words\thour",
        "unit\tx\u{7f}\thour",
        "unit\tx\u{1}\thour",
        "unit\tx\tfortnight",
        "unit\tx\tHOUR",
        "relation\tx\tthrough",
        "relation\tx\t",
        "spacing\tat\t",
        "spacing\tat\tbad frame",
        "window\tduring\tbad\u{7f}",
        "frequency\tper\t",
        "frequency\tper\tPERIOD",
        "approximation\tapproximate\t",
        "approximation\tapproximate\tABOUT",
        "range\tto\t",
        "range\tto\tmaximum",
        "other\tx\thour",
        "UNIT\tx\thour",
        "unit\tbad lemma\tbad",
    ];
    for (i, row) in invalid.iter().enumerate() {
        for reversed in [false, true] {
            let valid = "unit\tindependent\tyear\n";
            let body = if reversed {
                format!("{valid}{row}\n")
            } else {
                format!("{row}\n{valid}")
            };
            cases.add(
                &format!("row-law {i}, reversed={reversed}"),
                format!("{HEADER}{body}").as_bytes(),
            );
        }
    }
    let pairs = [
        ("unit\tx\thour", "unit\tx\tday"),
        ("unit\tframe\thour", "spacing\tat\tframe"),
        ("unit\tframe\thour", "window\tduring\tframe"),
        ("spacing\tat\tframe", "spacing\tat\tframe"),
        ("spacing\tat\tframe", "window\tat\tframe"),
        ("window\tat\tframe", "window\tat\tframe"),
        ("approximation\tx\tabout", "approximation\tx\tabout"),
        ("unit\tx\thour", "unit\tx\tbad"),
        ("relation\tx\tbefore", "relation\tx\tbad"),
        ("frequency\tx\tperiod", "frequency\tx\tbad"),
        ("approximation\tx\tabout", "approximation\tx\tbad"),
        ("range\tx\tminimum", "range\tx\tbad"),
    ];
    for (i, (left, right)) in pairs.iter().enumerate() {
        for (a, b) in [(left, right), (right, left)] {
            cases.add(
                &format!("pair {i}: {a:?}/{b:?}"),
                format!("{HEADER}{a}\n{b}\n").as_bytes(),
            );
        }
    }
    let preps = [
        ("relation", "after"),
        ("frequency", "period"),
        ("range", "minimum"),
    ];
    for (left, lv) in preps {
        for (right, rv) in preps {
            cases.add(
                &format!("preposition {left}/{right}"),
                format!("{HEADER}{left}\tx\t{lv}\n{right}\tx\t{rv}\n").as_bytes(),
            );
        }
    }
    // UTF-8, quoting and non-ASCII aliases are data; only the declared key spaces conflict.
    cases.run(&mut checks);
    let ui = UiTree::new();
    let mut generated = 0;
    for alias in ["x", "é", "Ω", "'", "a&<b>", "x\\y", "a-b", "\u{a0}"] {
        for (left, lv) in preps {
            for (right, rv) in preps {
                for fresh in [false, true] {
                    let rhs = if fresh { "distinct" } else { alias };
                    let bytes = format!("{HEADER}{left}\t{alias}\t{lv}\n{right}\t{rhs}\t{rv}\n");
                    ui.check_table(
                        &mut checks,
                        &format!("generated {generated}"),
                        bytes.as_bytes(),
                    );
                    generated += 1;
                }
            }
        }
        for (kind, value) in [
            ("unit", "hour"),
            ("relation", "after"),
            ("spacing", "frame"),
            ("window", "window"),
            ("frequency", "period"),
            ("range", "minimum"),
        ] {
            for reversed in [false, true] {
                let a = format!("approximation\t{alias}\tabout\n");
                let b = format!("{kind}\t{alias}\t{value}\n");
                let body = if reversed { b + &a } else { a + &b };
                ui.check_table(
                    &mut checks,
                    &format!("generated {generated}"),
                    format!("{HEADER}{body}").as_bytes(),
                );
                generated += 1;
            }
        }
    }
    for byte in (0..=32).chain([127]) {
        for position in 0..=2 {
            let mut alias = b"xy".to_vec();
            alias.insert(position, byte);
            let bytes = [HEADER.as_bytes(), b"unit\t", &alias, b"\thour\n"].concat();
            ui.check_table(
                &mut checks,
                &format!("control byte {byte} at {position}"),
                &bytes,
            );
            generated += 1;
        }
    }
    for (left, right) in [
        ("spacing\tat\tframe", "window\tduring\tframe"),
        ("spacing\tat\tframe", "spacing\tfrom\tframe"),
        ("spacing\tat\tframe", "window\tat\tother"),
        ("relation\tx\tafter", "spacing\tx\tframe"),
        ("frequency\tx\tperiod", "window\tx\tframe"),
        ("range\tx\tminimum", "spacing\tx\tframe"),
        ("unit\tx\thour", "relation\tx\tafter"),
    ] {
        for (a, b) in [(left, right), (right, left)] {
            ui.check_table(
                &mut checks,
                "disjoint key spaces",
                format!("{HEADER}{a}\n{b}\n").as_bytes(),
            );
            generated += 1;
        }
    }
    assert_eq!(generated, 356, "C1 alias/control/key-space property domain");
    checks.finish();
}

#[test]
fn b04_all_seven_kinds_and_meter() {
    let mut cases = TableCases::default();
    let mut checks = Checks::new("B4");
    cases.add("all kinds", format!("{HEADER}{ALL_ROWS}").as_bytes());
    let rows: Vec<_> = ALL_ROWS.lines().collect();
    for shift in 0..rows.len() {
        let body = rows
            .iter()
            .cycle()
            .skip(shift)
            .take(rows.len())
            .copied()
            .collect::<Vec<_>>()
            .join("\n");
        cases.add(
            &format!("row rotation {shift}"),
            format!("{HEADER}{body}\n").as_bytes(),
        );
    }
    for unit in ["second", "minute", "hour", "day", "week", "month", "year"] {
        cases.add(unit, format!("{HEADER}unit\tx\t{unit}\n").as_bytes());
    }
    for role in ["duration", "within", "after", "before"] {
        cases.add(role, format!("{HEADER}relation\tx\t{role}\n").as_bytes());
    }
    cases.run(&mut checks);
    checks.finish();
}

#[test]
fn b05_table_boundaries_and_file_laws() {
    let mut cases = TableCases::default();
    let tree = CheckTree::new();
    let mut checks = Checks::new("B5");
    for bytes in [
        b"".as_slice(),
        HEADER.as_bytes(),
        b"# format: kind\tlemma\tvalue",
        b"# format: kind\tlemma\tvalue\nunit\tx\thour",
        b"# format: kind\tlemma\tvalue\n\n",
        b"# format: kind\tlemma\tvalue\nunit\t\xff\thour\n",
        b"# format: kind\tlemma\tvalue\nwindow\tx\t\xff\n",
    ] {
        cases.add(&format!("boundary {bytes:?}"), bytes);
    }
    cases.run(&mut checks);
    for kind in ["directory", "symlink", "dangling"] {
        tree.table(format!("{HEADER}unit\tx\thour\n").as_bytes());
        let path = tree.tree.join(TABLE_PATH);
        remove(&path);
        match kind {
            "directory" => fs::create_dir(&path).unwrap(),
            "symlink" => {
                write(
                    &tree.dir.join("target.tsv"),
                    format!("{HEADER}unit\tx\thour\n"),
                );
                symlink(tree.dir.join("target.tsv"), &path).unwrap();
            }
            _ => symlink(tree.dir.join("missing.tsv"), &path).unwrap(),
        }
        let out = tree.run(&["check"]);
        let diag = diagnostic(&out);
        let expected =
            format!("ckc: guideline: vocabulary.tsv is not a regular file: {TABLE_PATH}\n");
        checks.require(
            out.status.code() == Some(2) && diag == expected,
            format!("{kind}: expected rc2 {expected:?}; {}", observed(&out)),
        );
    }
    checks.finish();
}

#[test]
fn b06_retired_name_refused_by_every_reader() {
    artifact_snapshot_controls();
    let mut checks = Checks::new("B6");
    for (content, retired) in [
        (
            "old-valid",
            format!("{HEADER}{OLD_KIND_HEADER}unit\thour\thour\n"),
        ),
        ("new-valid", format!("{HEADER}unit\thour\thour\n")),
    ] {
        for beside in [false, true] {
            let tree = CheckTree::new();
            remove(&tree.tree.join(TABLE_PATH));
            write(&tree.tree.join(RETIRED_PATH), &retired);
            if beside {
                write(
                    &tree.tree.join(TABLE_PATH),
                    format!("{HEADER}unit\thour\thour\n"),
                );
            }
            git(&tree.tree, &["add", "-A"]);
            git(
                &tree.tree,
                &[
                    "commit",
                    "-q",
                    "--allow-empty",
                    "-m",
                    "retired table fixture",
                ],
            );
            let dest = tree.dir.join("dist");
            let ui_root = tree.tree.to_str().unwrap();
            for args in [
                vec!["check"],
                vec!["compile", GID],
                vec!["queries", GID],
                vec!["certify", GID],
                vec!["release-manifest"],
                vec!["dist", "build", dest.to_str().unwrap()],
                vec!["ui", "check", ui_root],
            ] {
                let before = artifact_snapshot(&tree.tree, &dest);
                assert!(before.contains_key(Path::new("repo/release-manifest.tsv")));
                // Only check promises the table section before SWI; other readers run normally.
                let out = if args[0] == "check" {
                    tree.run(&args)
                } else {
                    ckc(&tree.tree, &args)
                };
                let after = artifact_snapshot(&tree.tree, &dest);
                let diag = diagnostic(&out);
                let named = if args[0] == "check" {
                    diag.lines()
                        .next()
                        .is_some_and(|line| line.contains("temporal.tsv"))
                } else {
                    diag.contains("temporal.tsv")
                };
                let refused = out.status.code().is_some_and(|rc| rc != 0)
                    && !text(&out.stderr).contains("timeout: sending signal")
                    && named;
                checks.require(
                    refused,
                    format!("{content} beside={beside} {args:?}: {}", observed(&out)),
                );
                let changed = before
                    .iter()
                    .filter(|(path, state)| after.get(*path) != Some(*state))
                    .count()
                    + after
                        .keys()
                        .filter(|path| !before.contains_key(*path))
                        .count();
                checks.require(
                    changed == 0,
                    format!("{content} beside={beside} {args:?}: {changed} artifact paths changed"),
                );
                // A missing guard can start corpus certification; the first mismatch already decides red.
                if !checks.failures.is_empty() {
                    checks.finish();
                    return;
                }
            }
        }
    }
    checks.finish();
}

struct UiTree {
    scratch: Scratch,
    tree: PathBuf,
}
impl UiTree {
    fn new() -> Self {
        let scratch = Scratch::new();
        let tree = scratch.0.join("tree");
        copy_tree(&root().join("tests/ui/green/basic/tree"), &tree);
        Self { scratch, tree }
    }
    fn table(&self, bytes: &[u8]) {
        remove(&self.tree.join("guidelines/alpha/temporal.tsv"));
        write(&self.tree.join("guidelines/alpha/vocabulary.tsv"), bytes);
    }
    fn run(&self, args: &[&str]) -> Output {
        ckc(&self.scratch.0, args)
    }
    fn check_table(&self, checks: &mut Checks, label: &str, bytes: &[u8]) {
        self.table(bytes);
        let out = self.run(&["ui", "check", self.tree.to_str().unwrap()]);
        match reference_ui_table(bytes) {
            Ok(_) => {
                checks.success(&out, label);
            }
            Err(why) => {
                let expected = format!("ui: viewmodel: alpha {why}\n");
                checks.require(
                    out.status.code() == Some(1) && out.stderr == expected.as_bytes(),
                    format!("{label}: expected {expected:?}; {}", observed(&out)),
                );
            }
        }
    }
    fn repin(&self) {
        let guideline = self.tree.join("guidelines/alpha");
        let out = self.run(&["derive-review-manifest", guideline.to_str().unwrap()]);
        assert!(out.status.success(), "fixture custody: {}", observed(&out));
        assert!(out.stderr.is_empty());
        write(&guideline.join("audit/review-manifest.tsv"), &out.stdout);
    }
    fn render(&self, checks: &mut Checks, label: &str) -> Option<PathBuf> {
        let dest = self.scratch.0.join("pages");
        remove(&dest);
        let out = self.run(&[
            "ui",
            "render",
            dest.to_str().unwrap(),
            self.tree.to_str().unwrap(),
        ]);
        checks.success(&out, label).then_some(dest)
    }
}

fn atom(s: &str) -> String {
    if s.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}

fn digest_term(bytes: Option<&[u8]>) -> String {
    bytes.map_or("none".into(), |b| format!("sha256({})", atom(&sha(b))))
}

fn document_header(id: &str, ace: &[u8], ulex: Option<&[u8]>, table: Option<&[u8]>) -> String {
    let mut out = format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n"
    );
    for (name, arity) in INDICATORS {
        out.push_str(&format!(
            ":- multifile(guideline_{name}/{arity}).\n:- discontiguous(guideline_{name}/{arity}).\n"
        ));
    }
    out.push_str(&format!("guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex({}),vocabulary({})).\n", atom(id), atom(&sha(ace)), digest_term(ulex), digest_term(table)));
    out
}

fn question_record(id: &str, ace: &[u8], ulex: Option<&[u8]>, table: Option<&[u8]>) -> String {
    format!(
        "'$guideline_query'(v1,{},ace_sha256({}),ulex({}),vocabulary({})).",
        atom(id),
        atom(&sha(ace)),
        digest_term(ulex),
        digest_term(table)
    )
}

fn id_ref(id: &str, n: usize) -> String {
    format!("'$guideline_id'(product,{},1,ref({n}),[])", atom(id))
}

fn simple_facts(id: &str, timed: bool) -> Vec<String> {
    let who = id_ref(id, 1);
    let qty = id_ref(id, 2);
    let event = id_ref(id, if timed { 3 } else { 2 });
    let mut facts = vec![
        format!("guideline_entity(actual,{who},man,countable)"),
        format!("guideline_cardinality(actual,{who},na,eq,1)"),
    ];
    if timed {
        facts.extend([
            format!("guideline_entity(actual,{qty},hour,countable)"),
            format!("guideline_cardinality(actual,{qty},na,eq,2)"),
            format!("guideline_unit(actual,{qty},hour)"),
        ]);
    }
    facts.extend([
        format!("guideline_event(actual,{event},wait)"),
        format!("guideline_arg(actual,{event},1,{who})"),
    ]);
    if timed {
        facts.extend([
            format!("guideline_pp(actual,{event},for,{qty})"),
            format!("guideline_interval(actual,{event},duration,{qty},hour,none)"),
        ]);
    }
    facts
}

struct Compiler {
    scratch: Scratch,
    stage: PathBuf,
}

// One staged compiler per test process (APE + Clex + the compiled parser + Clex qcompiled
// beside its source: every compile then loads the same clauses from the .qlf in ~0.3 s instead
// of ~1.2 s from source). Compiles only read the stage; each Compiler keeps its own scratch for
// inputs. The stage directory is rebuilt at first use in every run.
fn compiler_stage() -> &'static Path {
    static STAGE: OnceLock<PathBuf> = OnceLock::new();
    STAGE.get_or_init(|| {
        let dir = root().join("rust/target/c1-surface/stage");
        remove(&dir);
        let stage = dir.join("ape");
        copy_tree(&root().join("vendor/ape"), &stage);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let goal = format!(
            "working_directory(_, {}), [fit_to_plp], working_directory(_, {}), qcompile(clex_lexicon), halt.",
            atom(stage.join("prolog/parser").to_str().unwrap()),
            atom(stage.join("prolog/lexicon").to_str().unwrap())
        );
        let out = command(
            Path::new("swipl"),
            &dir,
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
            &[],
        );
        assert!(out.status.success(), "compiler stage: {}", observed(&out));
        stage
    })
}

impl Compiler {
    fn new() -> Self {
        let scratch = Scratch::new();
        write(
            &scratch.0.join("-"),
            b"a file named dash is not a table or lexicon\n",
        );
        Self {
            scratch,
            stage: compiler_stage().to_path_buf(),
        }
    }
    fn input(&self, name: &str, bytes: &[u8]) -> String {
        let path = self.scratch.0.join(name);
        write(&path, bytes);
        path.to_str().unwrap().into()
    }
    fn argv(&self, mode: &str, table: &str, ulex: &str) -> Vec<String> {
        let mut args = Vec::new();
        if mode == "question" {
            args.push("question".into());
        }
        args.extend([
            self.stage.to_str().unwrap().into(),
            "c1surface".into(),
            table.into(),
            ulex.into(),
        ]);
        if mode == "proof" {
            args.push("proof".into());
        }
        args
    }
    fn run(&self, args: &[String], ace: &str) -> Output {
        let mut argv = vec![
            "-q".into(),
            "-f".into(),
            "none".into(),
            "-F".into(),
            "none".into(),
            "-s".into(),
            self.stage
                .join("prolog/ace_to_pl.pl")
                .to_str()
                .unwrap()
                .into(),
            "-g".into(),
            "main".into(),
            "-t".into(),
            "halt(9)".into(),
            "--".into(),
        ];
        argv.extend_from_slice(args);
        command(
            Path::new("swipl"),
            &self.scratch.0,
            &argv,
            ace.as_bytes(),
            &[],
        )
    }
    fn compile(&self, mode: &str, ace: &str, table: Option<&[u8]>, ulex: Option<&[u8]>) -> Output {
        let table = table.map_or("-".into(), |b| self.input("vocabulary.tsv", b));
        let ulex = ulex.map_or("-".into(), |b| self.input("lexicon.ulex", b));
        self.run(&self.argv(mode, &table, &ulex), ace)
    }
}

fn product_body(out: &str) -> &str {
    out.find("% S1:").map_or("", |i| &out[i..])
}

fn compiler_reject(
    checks: &mut Checks,
    label: &str,
    out: &Output,
    class: &str,
    detail: Option<&str>,
) {
    let err = text(&out.stderr);
    let expected_prefix = format!("ace_to_pl_error({class},");
    let rc = if class == "unsupported" { 1 } else { 2 };
    let exact = detail.is_none_or(|d| err == format!("ace_to_pl_error({class},{d}).\n"));
    checks.require(
        out.status.code() == Some(rc)
            && out.stdout.is_empty()
            && err.starts_with(&expected_prefix)
            && err.ends_with(").\n")
            && err.lines().count() == 1
            && exact,
        format!(
            "{label}: expected rc{rc} {class} detail={detail:?}; {}",
            observed(out)
        ),
    );
}

#[test]
fn b07_raw_table_digest_custody() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B7");
    let a = format!("{HEADER}unit\thour\thour\nunit\tday\tday\n");
    let b = format!("{HEADER}unit\tday\tday\nunit\thour\thour\n");
    assert_ne!(sha(a.as_bytes()), sha(b.as_bytes()));
    for mode in ["product", "question"] {
        let ace = if mode == "question" {
            "Does a man wait?\n"
        } else {
            "A man waits.\n"
        };
        let mut bodies = Vec::new();
        for table in [&a, &b] {
            let out = compiler.compile(mode, ace, Some(table.as_bytes()), None);
            if checks.success(&out, &format!("{mode} raw bytes")) {
                let rendered = text(&out.stdout);
                let wrapper = format!("vocabulary(sha256({}))", atom(&sha(table.as_bytes())));
                checks.require(
                    rendered.contains(&wrapper),
                    format!("{mode}: raw digest missing {wrapper}"),
                );
                bodies.push(if mode == "product" {
                    product_body(&rendered).to_owned()
                } else {
                    rendered.lines().skip(2).collect::<Vec<_>>().join("\n")
                });
            }
        }
        if bodies.len() == 2 {
            checks.require(
                bodies[0] == bodies[1],
                format!("{mode}: row order altered semantic payload"),
            );
        }
    }
    checks.finish();
}

#[test]
fn b08_document_argv_and_exact_product() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B8");
    let table = format!("{HEADER}{ALL_ROWS}");
    for timed in [false, true] {
        let ace = if timed {
            "A man waits for 2 hours.\n"
        } else {
            "A man waits.\n"
        };
        let table_bytes = timed.then_some(table.as_bytes());
        let ulex = timed.then_some(ULEX.as_bytes());
        let out = compiler.compile("product", ace, table_bytes, ulex);
        if checks.success(&out, &format!("product timed={timed}")) {
            let expected = document_header("c1surface", ace.as_bytes(), ulex, table_bytes)
                + &format!("% S1: {}\n", ace.trim_end())
                + &(simple_facts("c1surface", timed).join(".\n") + ".\n");
            checks.require(
                out.stdout == expected.as_bytes(),
                format!(
                    "product timed={timed}: exact payload differs; actual={:?}",
                    text(&out.stdout)
                ),
            );
        }
    }
    checks.finish();
}

#[test]
fn b09_proof_argv_and_exact_payload() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B9");
    let table = format!("{HEADER}{ALL_ROWS}");
    for with_table in [false, true] {
        for with_ulex in [false, true] {
            let timed = with_table && with_ulex;
            let ace = if timed {
                "A man waits for 2 hours.\n"
            } else {
                "A man waits.\n"
            };
            let out = compiler.compile(
                "proof",
                ace,
                with_table.then_some(table.as_bytes()),
                with_ulex.then_some(ULEX.as_bytes()),
            );
            let label = format!("proof table={with_table} ulex={with_ulex}");
            if checks.success(&out, &label) {
                let expected = format!(
                    "'$guideline_proof'(c1surface,1,variant(1),witness([]),prove([{}])).\n",
                    simple_facts("c1surface", timed).join(",")
                );
                checks.require(
                    out.stdout == expected.as_bytes(),
                    format!(
                        "{label}: expected {expected:?}, actual {:?}",
                        text(&out.stdout)
                    ),
                );
            }
        }
    }
    checks.finish();
}

#[test]
fn b10_question_argv_and_projection() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B10");
    let table = format!("{HEADER}{ALL_ROWS}");
    let ace = "Does a man wait?\n";
    let terms = [
        "guideline_entity(actual,A,man,countable)",
        "guideline_cardinality(actual,A,na,eq,1)",
        "guideline_event(actual,B,wait)",
        "guideline_arg(actual,B,1,A)",
    ];
    let mut conjunction = terms.last().unwrap().to_string();
    for term in terms[..terms.len() - 1].iter().rev() {
        conjunction = format!("','({term},{conjunction})");
    }
    for with_table in [false, true] {
        let table_bytes = with_table.then_some(table.as_bytes());
        let out = compiler.compile("question", ace, table_bytes, None);
        if checks.success(&out, &format!("question table={with_table}")) {
            let expected = format!(
                "% c1surface compiled from ACE question by ace_to_pl question mode; do not edit.\n{}\n% Q1: {}\n'$guideline_query_projection'(goal({conjunction}),answers([])).\n",
                question_record("c1surface", ace.as_bytes(), None, table_bytes),
                ace.trim_end()
            );
            checks.require(
                out.stdout == expected.as_bytes(),
                format!(
                    "question table={with_table}: exact projection differs; actual={:?}",
                    text(&out.stdout)
                ),
            );
        }
    }
    checks.finish();
}

#[test]
fn b11_proof_slot_refusals_precede_loads() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B11");
    for mode in ["product", "proof", "question"] {
        for slot in ["vocabulary", "ulex"] {
            let (table, ulex) = if slot == "vocabulary" {
                ("proof", "missing-ulex")
            } else {
                ("missing-table", "proof")
            };
            let mut args = compiler.argv(mode, table, ulex);
            let tree_slot = usize::from(mode == "question");
            args[tree_slot] = "missing-ape".into();
            let out = compiler.run(&args, "Not parsed.\n");
            compiler_reject(
                &mut checks,
                &format!("{mode} {slot}"),
                &out,
                "usage",
                Some(&format!("{slot}_arg(proof)")),
            );
        }
    }
    checks.finish();
}

#[test]
fn b12_version_selectors_refused_before_loads() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B12");
    for version in ["v1", "v2", "v3"] {
        for tail in [
            vec![],
            vec!["missing-tree"],
            vec!["missing-tree", "c1surface", "missing-table"],
            vec!["missing-tree", "c1surface", "missing-table", "proof"],
            vec!["missing-tree", "c1surface", "missing-table", "missing-ulex"],
            vec![
                "missing-tree",
                "c1surface",
                "missing-table",
                "missing-ulex",
                "proof",
            ],
        ] {
            let args = std::iter::once(version)
                .chain(tail)
                .map(str::to_owned)
                .collect::<Vec<_>>();
            let out = compiler.run(&args, "Not parsed.\n");
            compiler_reject(&mut checks, &format!("{args:?}"), &out, "usage", None);
            checks.require(
                text(&out.stderr).starts_with("ace_to_pl_error(usage,argv("),
                format!(
                    "{args:?}: version diagnostic is not argv: {}",
                    observed(&out)
                ),
            );
        }
    }
    checks.finish();
}

#[test]
fn b13_invalid_argv_shapes() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B13");
    let vectors: &[&[&str]] = &[
        &[],
        &["missing-tree"],
        &["missing-tree", "c1surface"],
        &["missing-tree", "c1surface", "-"],
        &["missing-tree", "c1surface", "-", "-", "not-proof"],
        &["missing-tree", "c1surface", "-", "-", "proof", "extra"],
        &["question"],
        &["question", "missing-tree"],
        &["question", "missing-tree", "c1surface"],
        &["question", "missing-tree", "c1surface", "-"],
        &["question", "missing-tree", "c1surface", "-", "-", "proof"],
        &["x", "x", "x", "x", "x", "x", "x", "x", "x"],
    ];
    for version in ["v1", "v2", "v3"] {
        let args = [
            "question",
            version,
            "missing-tree",
            "c1surface",
            "missing-table",
            "missing-ulex",
        ]
        .map(str::to_owned);
        let out = compiler.run(&args, "Not parsed.\n");
        compiler_reject(
            &mut checks,
            &format!("retired question selector {version}"),
            &out,
            "usage",
            None,
        );
    }
    for vector in vectors {
        let args = vector.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        let out = compiler.run(&args, "Not parsed.\n");
        compiler_reject(&mut checks, &format!("{vector:?}"), &out, "usage", None);
    }
    checks.finish();
}

#[test]
fn b14_table_load_refusals_all_modes() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B14");
    let missing = compiler.scratch.0.join("missing.tsv");
    let directory = compiler.scratch.0.join("directory.tsv");
    fs::create_dir(&directory).unwrap();
    let unreadable = compiler.scratch.0.join("unreadable.tsv");
    write(&unreadable, format!("{HEADER}unit\tx\thour\n"));
    fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o000)).unwrap();
    let malformed = PathBuf::from(compiler.input(
        "malformed.tsv",
        format!("{HEADER}unit\tx\tfortnight\n").as_bytes(),
    ));
    let ulex = compiler.input("input.ulex", ULEX.as_bytes());
    for mode in ["product", "proof", "question"] {
        for (label, path) in [
            ("missing", &missing),
            ("directory", &directory),
            ("unreadable", &unreadable),
            ("malformed", &malformed),
        ] {
            let ace = if mode == "question" {
                "Does a man wait?\n"
            } else {
                "A man waits.\n"
            };
            let out = compiler.run(&compiler.argv(mode, path.to_str().unwrap(), &ulex), ace);
            compiler_reject(
                &mut checks,
                &format!("{mode}/{label}"),
                &out,
                "vocabulary_load",
                None,
            );
        }
    }
    fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o600)).unwrap();
    checks.finish();
}

#[test]
fn b15_document_header_matrix() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B15");
    let table = format!("{HEADER}{ALL_ROWS}");
    let ace = "A man waits.\n";
    for with_table in [false, true] {
        for with_ulex in [false, true] {
            let table_bytes = with_table.then_some(table.as_bytes());
            let ulex = with_ulex.then_some(ULEX.as_bytes());
            let out = compiler.compile("product", ace, table_bytes, ulex);
            if checks.success(&out, &format!("table={with_table} ulex={with_ulex}")) {
                let header = document_header("c1surface", ace.as_bytes(), ulex, table_bytes);
                let actual = text(&out.stdout);
                checks.require(
                    actual.starts_with(&header),
                    format!("table={with_table} ulex={with_ulex}: D2 header differs"),
                );
                checks.require(
                    actual.matches(":- multifile(").count() == 19
                        && actual.matches(":- discontiguous(").count() == 19,
                    "D2 declaration count",
                );
                checks.require(
                    actual.matches("guideline_schema_version(1).\n").count() == 1
                        && !actual.contains("temporal("),
                    "D2 single identifier + vocabulary wrapper",
                );
            }
        }
    }
    checks.finish();
}

#[test]
fn b16_question_record_matrix() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B16");
    let table = format!("{HEADER}{ALL_ROWS}");
    let ace = "Does a man wait?\n";
    for with_table in [false, true] {
        for with_ulex in [false, true] {
            let table_bytes = with_table.then_some(table.as_bytes());
            let ulex = with_ulex.then_some(ULEX.as_bytes());
            let out = compiler.compile("question", ace, table_bytes, ulex);
            if checks.success(&out, &format!("table={with_table} ulex={with_ulex}")) {
                let expected = question_record("c1surface", ace.as_bytes(), ulex, table_bytes);
                let actual = text(&out.stdout);
                checks.require(
                    actual.lines().nth(1) == Some(&expected),
                    format!(
                        "D3 record: expected {expected:?}, actual {:?}",
                        actual.lines().nth(1)
                    ),
                );
                checks.require(
                    actual.lines().count() == 4 && !actual.contains("temporal("),
                    "D3 four-line question + vocabulary wrapper",
                );
            }
        }
    }
    checks.finish();
}

fn expected_annotation(pattern: &str, mode: &str) -> Vec<String> {
    let r = |n: usize| {
        if mode == "question" {
            char::from(b'A' + u8::try_from(n - 1).unwrap()).to_string()
        } else {
            id_ref("c1surface", n)
        }
    };
    match pattern {
        "interval" => vec![format!(
            "guideline_interval(actual,{},duration,{},hour,none)",
            r(3),
            r(2)
        )],
        "recurrence" => vec![format!(
            "guideline_recurrence(actual,{},{},hour)",
            r(4),
            r(3)
        )],
        "order" => vec![format!("guideline_order(actual,{},before,{})", r(3), r(2))],
        "frequency" => vec![format!(
            "guideline_frequency(actual,{},{},{},day)",
            r(4),
            r(2),
            r(3)
        )],
        "window" => vec![format!(
            "guideline_recurrence_window(actual,{},{},hour,{},{},day)",
            r(7),
            r(3),
            r(6),
            r(5)
        )],
        "approximation" => vec![
            format!("guideline_cardinality(actual,{},na,about,2)", r(2)),
            format!(
                "guideline_interval(actual,{},duration,{},hour,none)",
                r(3),
                r(2)
            ),
        ],
        "range" => vec![
            format!(
                "guideline_interval(actual,{},duration,{},hour,none)",
                r(4),
                r(2)
            ),
            format!("guideline_range(actual,{},{})", r(2), r(3)),
        ],
        _ => unreachable!(),
    }
}

// C1 range question's source occurrences: patient, low hours, high hours, event.
fn range_query_source() -> String {
    let terms = [
        "guideline_entity(actual,A,patient,countable)",
        "guideline_cardinality(actual,A,na,eq,1)",
        "guideline_entity(actual,B,hour,countable)",
        "guideline_cardinality(actual,B,na,geq,8)",
        "guideline_entity(actual,C,hour,countable)",
        "guideline_cardinality(actual,C,na,eq,12)",
        "guideline_event(actual,D,wait)",
        "guideline_arg(actual,D,1,A)",
        "guideline_pp(actual,D,for,B)",
        "guideline_interval(actual,D,duration,B,hour,none)",
        "guideline_pp(actual,D,to,C)",
        "guideline_range(actual,B,C)",
    ];
    let mut goal = terms.last().unwrap().to_string();
    for term in terms[..terms.len() - 1].iter().rev() {
        goal = format!("','({term},{goal})");
    }
    format!("'$guideline_query_projection'(goal({goal}),answers([])).\n")
}

#[test]
fn b17_all_annotation_patterns_in_documents_and_questions() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B17");
    let table = format!("{HEADER}{ALL_ROWS}");
    let patterns = [
        (
            "interval",
            "A patient waits for 2 hours.\n",
            "Does a patient wait for 2 hours?\n",
            "guideline_interval(",
        ),
        (
            "recurrence",
            "A patient waits at an interval of 2 hours.\n",
            "Does a patient wait at an interval of 2 hours?\n",
            "guideline_recurrence(",
        ),
        (
            "order",
            "A patient eats before a meeting.\n",
            "Does a patient eat before a meeting?\n",
            "guideline_order(",
        ),
        (
            "frequency",
            "A man reads 2 books per 1 day.\n",
            "Does a man read 2 books per 1 day?\n",
            "guideline_frequency(",
        ),
        (
            "window",
            "A patient waits at an interval of 2 hours during a window of 3 days of a surgery.\n",
            "Does a patient wait at an interval of 2 hours during a window of 3 days of a surgery?\n",
            "guideline_recurrence_window(",
        ),
        (
            "approximation",
            "A patient waits for 2 approximate hours.\n",
            "Does a patient wait for 2 approximate hours?\n",
            ",na,about,2)",
        ),
        (
            "range",
            "A patient waits for at least 8 hours to 12 hours.\n",
            "Does a patient wait for at least 8 hours to 12 hours?\n",
            "guideline_range(",
        ),
    ];
    for (label, doc, query, needle) in patterns {
        for (mode, ace) in [("product", doc), ("question", query)] {
            let out = compiler.compile(mode, ace, Some(table.as_bytes()), Some(ULEX.as_bytes()));
            if checks.success(&out, &format!("{label}/{mode}")) {
                let rendered = text(&out.stdout);
                let rendered = if label == "range" && mode == "question" {
                    nt_projection::erase(&rendered, &range_query_source())
                } else {
                    rendered
                };
                let payload = if mode == "product" {
                    product_body(&rendered)
                } else {
                    &rendered
                };
                checks.require(
                    payload.contains(needle),
                    format!("{label}/{mode}: missing {needle}"),
                );
                for expected in expected_annotation(label, mode) {
                    checks.require(
                        payload.contains(&expected),
                        format!("{label}/{mode}: wrong annotation arguments; missing {expected}"),
                    );
                }
                checks.require(
                    payload.contains("guideline_pp(")
                        && payload.contains("guideline_event(")
                        && payload.contains("guideline_cardinality("),
                    format!("{label}/{mode}: primitive retained"),
                );
                if label == "window" {
                    checks.require(
                        !payload.contains("guideline_recurrence("),
                        format!("{mode}: scoped recurrence emitted plain recurrence"),
                    );
                }
                if label == "range" {
                    checks.require(
                        payload.contains("guideline_interval(")
                            && payload.contains(",na,geq,8)")
                            && payload.contains(",na,eq,12)"),
                        format!("{mode}: range lost interval or bounds"),
                    );
                }
            }
        }
    }
    for (label, doc, query, needle) in patterns {
        let kind = match label {
            "interval" | "order" => "relation",
            "recurrence" => "spacing",
            "window" => "window",
            other => other,
        };
        let body = ALL_ROWS
            .lines()
            .filter(|line| line.split('\t').next() != Some(kind))
            .map(|line| format!("{line}\n"))
            .collect::<String>();
        let subset = format!("{HEADER}{body}");
        for (mode, ace) in [("product", doc), ("question", query)] {
            let out = compiler.compile(mode, ace, Some(subset.as_bytes()), Some(ULEX.as_bytes()));
            if matches!(label, "recurrence" | "window") {
                compiler_reject(
                    &mut checks,
                    &format!("unclaimed relation {label}/{mode}"),
                    &out,
                    "unsupported",
                    None,
                );
            } else if checks.success(&out, &format!("unmapped pattern {label}/{mode}")) {
                let rendered = text(&out.stdout);
                let payload = if mode == "product" {
                    product_body(&rendered)
                } else {
                    &rendered
                };
                checks.require(
                    !payload.contains(needle),
                    format!("{label}/{mode}: annotation survives removal of its vocabulary kind"),
                );
            }
        }
    }
    let rejects = [
        (
            "frequency_shape",
            "A man reads 2 books per at least 1 approximate day.",
            "Does a man read 2 books per at least 1 approximate day?",
        ),
        (
            "approximate_bound",
            "A man waits at an approximate interval of at least 2 hours.",
            "Does a man wait at an approximate interval of at least 2 hours?",
        ),
        (
            "no_bound",
            "A man waits for 3 kg of approximate hours.",
            "Does a man wait for 3 kg of approximate hours?",
        ),
        (
            "zero_bound",
            "A man waits for 0 days.",
            "Does a man wait for 0 days?",
        ),
        (
            "range_shape",
            "A man waits for at least 8 hours to 8 hours.",
            "Does a man wait for at least 8 hours to 8 hours?",
        ),
        (
            "shared_quantity",
            "A man waits during a window of 3 weeks of a visit for at least 2 hours to 3 hours within the hours.",
            "Does a man wait during a window of 3 weeks of a visit for at least 2 hours to 3 hours within the hours?",
        ),
        (
            "window_shape",
            "A man waits to 3 hours during a window.",
            "Does a man wait to 3 hours during a window?",
        ),
        (
            "anchor_count",
            "A man waits at an interval.",
            "Does a man wait at an interval?",
        ),
        (
            "anchor_shape",
            "A man waits within 3 days of 2 weeks.",
            "Does a man wait within 3 days of 2 weeks?",
        ),
        (
            "duration_anchor",
            "A man waits for 3 days of a visit.",
            "Does a man wait for 3 days of a visit?",
        ),
        (
            "frame_shape",
            "A man waits at an interval of a visit.",
            "Does a man wait at an interval of a visit?",
        ),
    ];
    for (why, doc, query) in rejects {
        for (mode, ace) in [("product", doc), ("question", query)] {
            let out = compiler.compile(
                mode,
                &(ace.to_owned() + "\n"),
                Some(table.as_bytes()),
                Some(ULEX.as_bytes()),
            );
            compiler_reject(
                &mut checks,
                &format!("typed shape {why}/{mode}"),
                &out,
                "unsupported",
                Some(&format!("temporal_shape({why},1)")),
            );
        }
    }
    // The old v2 row subset already names before; no selector can suppress order.
    let rows = format!("{HEADER}relation\tbefore\tbefore\n");
    for (mode, ace) in [
        ("product", "A patient eats before a meeting.\n"),
        ("question", "Does a patient eat before a meeting?\n"),
    ] {
        let out = compiler.compile(mode, ace, Some(rows.as_bytes()), None);
        if checks.success(&out, &format!("former-v2 rows/{mode}")) {
            checks.require(
                text(&out.stdout).contains("guideline_order(actual,"),
                format!("former-v2 rows/{mode}: no order annotation"),
            );
        }
    }
    checks.finish();
}

#[test]
fn b18_no_table_retains_primitive_and_question_laws() {
    let compiler = Compiler::new();
    let mut checks = Checks::new("B18");
    for (mode, ace) in [
        ("product", "A patient waits for 2 hours.\n"),
        ("question", "Does a patient wait for 2 hours?\n"),
        ("product", "A patient eats before a meeting.\n"),
        ("question", "Does a patient eat before a meeting?\n"),
        ("product", "A man reads 2 books per 1 day.\n"),
        ("question", "Does a man read 2 books per 1 day?\n"),
        ("product", "A patient waits for 2 approximate hours.\n"),
        ("question", "Does a patient wait for 2 approximate hours?\n"),
        (
            "product",
            "A patient waits for at least 8 hours to 12 hours.\n",
        ),
        (
            "question",
            "Does a patient wait for at least 8 hours to 12 hours?\n",
        ),
    ] {
        let out = compiler.compile(mode, ace, None, Some(ULEX.as_bytes()));
        if checks.success(&out, &format!("no-table {mode}: {ace:?}")) {
            let rendered = text(&out.stdout);
            let payload = if mode == "product" {
                product_body(&rendered)
            } else {
                &rendered
            };
            for name in [
                "interval",
                "recurrence",
                "frequency",
                "order",
                "recurrence_window",
                "range",
            ] {
                checks.require(
                    !payload.contains(&format!("guideline_{name}(")),
                    format!("{mode}: no-table emitted {name}"),
                );
            }
            checks.require(
                !payload.contains(",about,") && payload.contains("guideline_pp("),
                format!("{mode}: no-table primitive law"),
            );
        }
    }
    for (mode, ace, why) in [
        (
            "product",
            "Every man of a town waits.\n",
            "condition_shape(relation(A,of,B))",
        ),
        (
            "question",
            "Which man of a town waits?\n",
            "query_unsupported(leaf(/(relation,3)),1)",
        ),
    ] {
        let out = compiler.compile(mode, ace, None, None);
        compiler_reject(
            &mut checks,
            &format!("no-table relation/{mode}"),
            &out,
            "unsupported",
            Some(why),
        );
    }
    for bound in ["at most", "less than", "exactly"] {
        let ace = format!("Does a patient wait for {bound} 2 hours?\n");
        let out = compiler.compile("question", &ace, None, Some(ULEX.as_bytes()));
        compiler_reject(
            &mut checks,
            &format!("no-table nested bound {bound}"),
            &out,
            "unsupported",
            Some("query_unsupported(leaf(list),1)"),
        );
        let table = format!("{HEADER}{ALL_ROWS}");
        let out = compiler.compile(
            "question",
            &ace,
            Some(table.as_bytes()),
            Some(ULEX.as_bytes()),
        );
        checks.success(&out, &format!("table-backed nested bound {bound}"));
    }
    checks.finish();
}

fn html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn html_row(cells: &[&str]) -> String {
    format!(
        "<tr>{}</tr>",
        cells
            .iter()
            .map(|s| format!("<td>{}</td>", html(s)))
            .collect::<String>()
    )
}

fn section_rows(page: &str, heading: &str) -> Vec<String> {
    let Some((_, after)) = page.split_once(heading) else {
        return Vec::new();
    };
    let Some((section, _)) = after.split_once("</section>") else {
        return Vec::new();
    };
    let Some((_, body)) = section.split_once("<tbody>") else {
        return Vec::new();
    };
    let (body, _) = body.split_once("</tbody>").unwrap();
    body.split_inclusive("</tr>")
        .filter_map(|s| s.find("<tr>").map(|i| s[i..].to_owned()))
        .collect()
}

fn ui_document(fx: &UiTree, bodies: &[&str], table: Option<&[u8]>) {
    let ace = "A man waits.\n".repeat(bodies.len());
    let mut pl = document_header("a-10", ace.as_bytes(), None, table);
    for (i, body) in bodies.iter().enumerate() {
        pl.push_str(&format!("% S{}: A man waits.\n{body}\n", i + 1));
    }
    write(&fx.tree.join("guidelines/alpha/ace/a-10.ace"), ace);
    write(&fx.tree.join("guidelines/alpha/pl/a-10.pl"), pl);
    if let Some(table) = table {
        fx.table(table);
    } else {
        remove(&fx.tree.join("guidelines/alpha/vocabulary.tsv"));
    }
    fx.repin();
}

#[test]
fn b19_timing_rows_without_version_gate_and_range_only_absence() {
    let mut checks = Checks::new("B19");
    let table = format!("{HEADER}{ALL_ROWS}");
    let bodies = [
        "guideline_event(actual,e1,wait).\nguideline_cardinality(actual,q1,na,eq,2).\nguideline_interval(actual,e1,duration,q1,hour,none).",
        "guideline_event(actual,e2,wait).\nguideline_cardinality(actual,q2,na,leq,3).\nguideline_recurrence(actual,e2,q2,day).",
        "guideline_event(actual,e3,eat).\nguideline_entity(actual,a3,meeting,countable).\nguideline_order(actual,e3,before,a3).",
        "guideline_event(actual,e4,read).\nguideline_entity(actual,c4,book,countable).\nguideline_cardinality(actual,c4,na,eq,2).\nguideline_cardinality(actual,p4,na,eq,1).\nguideline_frequency(actual,e4,c4,p4,day).",
        "guideline_event(actual,e5,wait).\nguideline_entity(actual,a5,visit,countable).\nguideline_cardinality(actual,g5,na,eq,2).\nguideline_cardinality(actual,l5,na,eq,3).\nguideline_recurrence_window(actual,e5,g5,hour,a5,l5,day).",
        "guideline_event(actual,e6,wait).\nguideline_cardinality(actual,q6,na,about,1).\nguideline_interval(actual,e6,duration,q6,hour,none).",
        "guideline_event(actual,e7,wait).\nguideline_cardinality(actual,q7,na,geq,8).\nguideline_cardinality(actual,h7,na,eq,12).\nguideline_interval(actual,e7,duration,q7,hour,none).\nguideline_range(actual,q7,h7).",
        "guideline_property(actual,e8,ready,pos) :- guideline_event(actual,e8,wait), guideline_cardinality(actual,q8,na,eq,1), guideline_interval(actual,e8,within,q8,week,none).\nguideline_property(actual,e8,prepared,pos) :- guideline_event(actual,e8,wait), guideline_cardinality(actual,q8,na,eq,1), guideline_interval(actual,e8,within,q8,week,none).",
        "guideline_property(actual,e9,ready,pos) :- \\+ (guideline_event(actual,e9,wait), guideline_cardinality(actual,q9,na,eq,2), guideline_interval(actual,e9,duration,q9,day,none)).",
    ];
    let expected = [
        ["1", "statement", "wait", "lasts 2 hours", ""],
        ["2", "statement", "wait", "repeats at most 3 days apart", ""],
        ["3", "statement", "eat", "before", "meeting"],
        [
            "4",
            "statement",
            "read",
            "2 per 1 day, counted item: book",
            "",
        ],
        [
            "5",
            "statement",
            "wait",
            "repeats 2 hours apart during 3 days from",
            "visit",
        ],
        ["6", "statement", "wait", "lasts about 1 hour", ""],
        [
            "7",
            "statement",
            "wait",
            "lasts a minimum of 8 to 12 hours",
            "",
        ],
        ["8", "condition", "wait", "within 1 week of", "not stated"],
        ["9", "excluded condition", "wait", "lasts 2 days", ""],
    ]
    .iter()
    .map(|r| html_row(r))
    .collect::<Vec<_>>();
    for with_table in [false, true] {
        let fx = UiTree::new();
        ui_document(&fx, &bodies, with_table.then_some(table.as_bytes()));
        if let Some(pages) = fx.render(&mut checks, &format!("timing table={with_table}")) {
            let page = fs::read_to_string(pages.join("g/alpha/doc/a-10.html")).unwrap();
            let rows = section_rows(&page, "<h3>Timing as compiled</h3>");
            checks.require(
                rows == expected,
                format!("table={with_table}: expected nine exact timing rows, actual={rows:?}"),
            );
            checks.require(
                page.matches("<h3>Timing as compiled</h3>").count() == 1,
                "one timing section",
            );
            let out = fx.run(&["ui", "check", fx.tree.to_str().unwrap()]);
            checks.success(&out, "UI check over timing rows");
        }
        ui_document(
            &fx,
            &[
                "guideline_cardinality(actual,low,na,geq,8).\nguideline_cardinality(actual,high,na,eq,12).\nguideline_range(actual,low,high).",
            ],
            with_table.then_some(table.as_bytes()),
        );
        if let Some(pages) = fx.render(&mut checks, "range-only") {
            let page = fs::read_to_string(pages.join("g/alpha/doc/a-10.html")).unwrap();
            checks.require(
                !page.contains("<h3>Timing as compiled</h3>"),
                "range-only document rendered timing section",
            );
        }
        ui_document(
            &fx,
            &["guideline_event(actual,e,wait)."],
            with_table.then_some(table.as_bytes()),
        );
        if let Some(pages) = fx.render(&mut checks, "no annotation") {
            let page = fs::read_to_string(pages.join("g/alpha/doc/a-10.html")).unwrap();
            checks.require(
                !page.contains("<h3>Timing as compiled</h3>"),
                "unannotated document rendered timing section",
            );
        }
    }
    checks.finish();
}

fn expected_words(rows: &[(&str, &str, &str)]) -> Vec<String> {
    let mut out = Vec::new();
    for kind in [
        "unit",
        "relation",
        "spacing",
        "window",
        "frequency",
        "approximation",
        "range",
    ] {
        for (k, lemma, value) in rows.iter().filter(|(k, _, _)| *k == kind) {
            let label = if matches!(*k, "spacing" | "window") {
                format!("{lemma} (with {value})")
            } else {
                (*lemma).to_owned()
            };
            let meaning = match *k {
                "unit" => format!("unit of time: {value}"),
                "relation" => match *value {
                    "duration" => "how long the action lasts",
                    "within" => "how far the action lies from a reference point, before or after it",
                    "before" => "how long before a reference point the action occurs, or only that it occurs before it",
                    "after" => "how long after a reference point the action occurs, or only that it occurs after it",
                    _ => unreachable!(),
                }.into(),
                "spacing" => "how far apart repeats of the action lie".into(),
                "window" => "the time window in which the spacing of repeats holds".into(),
                "frequency" => "how many of an item the action involves in each period".into(),
                "approximation" => "marks a time limit as approximate".into(),
                "range" => "joins the upper end of a minimum that is stated as a range".into(),
                _ => unreachable!(),
            };
            out.push(html_row(&[&label, &meaning]));
        }
    }
    out
}

#[test]
fn b20_time_words_grouping_file_order_and_escaping() {
    let fx = UiTree::new();
    let mut checks = Checks::new("B20");
    let rows = [
        ("range", "upto", "minimum"),
        ("unit", "hr&<tick>", "hour"),
        ("frequency", "per", "period"),
        ("relation", "after", "after"),
        ("approximation", "rough", "about"),
        ("spacing", "at", "span"),
        ("unit", "é", "day"),
        ("window", "during", "window"),
        ("relation", "before", "before"),
        ("spacing", "along", "gap"),
        ("range", "to", "minimum"),
        ("relation", "for", "duration"),
        ("relation", "within", "within"),
        ("approximation", "near", "about"),
    ];
    for shift in 0..rows.len() {
        let rotated = rows
            .iter()
            .cycle()
            .skip(shift)
            .take(rows.len())
            .copied()
            .collect::<Vec<_>>();
        let table = HEADER.to_owned()
            + &rotated
                .iter()
                .map(|(a, b, c)| format!("{a}\t{b}\t{c}\n"))
                .collect::<String>();
        assert!(reference_table(table.as_bytes()).is_ok());
        fx.table(table.as_bytes());
        if let Some(pages) = fx.render(&mut checks, &format!("word rotation {shift}")) {
            let page = fs::read_to_string(pages.join("g/alpha/index.html")).unwrap();
            let actual = section_rows(&page, "<h2>Time words</h2>");
            checks.require(
                actual == expected_words(&rotated),
                format!("word rotation {shift}: ordered rows differ; actual={actual:?}"),
            );
            checks.require(
                page.matches("<h2>Time words</h2>").count() == 1,
                "one time-words section",
            );
        }
    }
    remove(&fx.tree.join("guidelines/alpha/vocabulary.tsv"));
    if let Some(pages) = fx.render(&mut checks, "absent table") {
        let page = fs::read_to_string(pages.join("g/alpha/index.html")).unwrap();
        checks.require(
            !page.contains("<h2>Time words</h2>"),
            "absent table rendered time words",
        );
    }
    checks.finish();
}

#[test]
fn b21_ui_intake_refusals() {
    let fx = UiTree::new();
    let mut checks = Checks::new("B21");
    for bytes in [
        b"".as_slice(),
        b"bad\n",
        HEADER.as_bytes(),
        b"# format: kind\tlemma\tvalue\n# old header\n",
        b"# format: kind\tlemma\tvalue\nunit\tx\tbad\n",
        b"# format: kind\tlemma\tvalue\nunit\t\xff\thour\n",
        b"# format: kind\tlemma\tvalue\nrange\tto\tminimum",
        b"# format: kind\tlemma\tvalue\nrange\tto\tminimum\nrelation\tto\tafter\n",
        b"bad\0\n",
        b"bad\xff\n",
        b"\0\xff\n",
        b"\xff\0\n",
        b"# format: kind\tlemma\tvalue\n#\0\n",
        b"# format: kind\tlemma\tvalue\nunit\tx\x01\0y\thour\n",
        b"# format: kind\tlemma\tvalue\nunit\tx\0\x01y\thour\n",
        b"# format: kind\tlemma\tvalue\nwindow\tx\t\xff\n",
    ] {
        fx.check_table(&mut checks, &format!("UI invalid table {bytes:?}"), bytes);
    }
    for point in [
        0x7e, 0x7f, 0x80, 0x9f, 0x2029, 0x202a, 0x202e, 0x202f, 0x2065, 0x2066, 0x2069, 0x206a,
    ] {
        let c = char::from_u32(point).unwrap();
        let bytes = format!("{HEADER}unit\tx{c}y\thour\n");
        fx.check_table(
            &mut checks,
            &format!("UI file boundary U+{point:04X}"),
            bytes.as_bytes(),
        );
    }
    checks.finish();
}

const EXPORT_SCHEMA: &str = "## Compiled Prolog schema\n\nOne schema, identifier 1.\n\n### Document ABI\n\n`guideline_schema_version(1).` and `vocabulary(none | sha256(T))`.\n\n### All annotations\n\nThe vocabulary.tsv table supplies every annotation.\n\n#### Nested details\n\nC1_SCHEMA_FINAL_SENTINEL\n\n";

struct ExportRepo {
    scratch: Scratch,
    tree: PathBuf,
}
impl ExportRepo {
    fn new(schema: &str, tables: usize) -> Self {
        let scratch = Scratch::new();
        let tree = scratch.0.join("repo");
        copy_tree(&root().join("tests/ui/green/basic/tree"), &tree);
        write(
            &tree.join("docs/REFERENCE.md"),
            format!("# Fixture reference\n\n{schema}## Operating\n\nC1_AFTER_SCHEMA_SENTINEL\n"),
        );
        write(&tree.join("NOTICE"), "C1 fixture notice.\n");
        write(
            &tree.join("vendor/ape/prolog/ace_to_pl.pl"),
            "% fixture compiler\n",
        );
        write(
            &tree.join("vendor/clex/clex_lexicon.pl"),
            "% fixture lexicon\n",
        );
        for gid in ["alpha", "beta"] {
            write(
                &tree.join(format!("guidelines/{gid}/rights.tsv")),
                format!(
                    "profile\tstatement\turl\tretrieved\tnote\nredistributable\tFixture rights.\thttps://example.invalid/{gid}\t2026-01-01\tFixture.\n"
                ),
            );
            remove(&tree.join(format!("guidelines/{gid}/temporal.tsv")));
            remove(&tree.join(format!("guidelines/{gid}/vocabulary.tsv")));
        }
        for gid in ["alpha", "beta"].iter().take(tables) {
            write(
                &tree.join(format!("guidelines/{gid}/vocabulary.tsv")),
                format!("{HEADER}{ALL_ROWS}"),
            );
        }
        git(&tree, &["init", "-q", "-b", "main"]);
        git(&tree, &["add", "-A"]);
        git(&tree, &["commit", "-q", "-m", "export fixture"]);
        Self { scratch, tree }
    }
    fn manifest(&self) -> Output {
        ckc(&self.tree, &["release-manifest"])
    }
    fn build(&self) -> Output {
        let dest = self.scratch.0.join("dist");
        ckc(&self.tree, &["dist", "build", dest.to_str().unwrap()])
    }
}

#[test]
fn b22_release_has_one_schema_row() {
    let mut checks = Checks::new("B22");
    for tables in 0..=2 {
        let repo = ExportRepo::new(EXPORT_SCHEMA, tables);
        let out = repo.manifest();
        if checks.success(&out, &format!("release tables={tables}")) {
            let manifest = fs::read_to_string(repo.tree.join("release-manifest.tsv")).unwrap();
            let rows = manifest
                .lines()
                .filter(|s| s.starts_with("meta\tschema\t"))
                .collect::<Vec<_>>();
            checks.require(
                rows == ["meta\tschema\tv1"],
                format!("release tables={tables}: schema rows={rows:?}"),
            );
            for gid in ["alpha", "beta"].iter().take(tables) {
                let bytes =
                    fs::read(repo.tree.join(format!("guidelines/{gid}/vocabulary.tsv"))).unwrap();
                let member = format!(
                    "member\tdata/guidelines/{gid}/vocabulary.tsv\t{}\t{}",
                    sha(&bytes),
                    bytes.len()
                );
                checks.require(
                    manifest.lines().any(|s| s == member),
                    format!("release tables={tables}: vocabulary member/digest missing {gid}"),
                );
            }
        }
    }
    checks.finish();
}

fn reference_schema() -> String {
    let reference = fs::read_to_string(root().join("docs/REFERENCE.md")).unwrap();
    let start = reference
        .find("## Compiled Prolog schema\n")
        .expect("REFERENCE schema H2");
    let tail = &reference[start..];
    let end = tail[3..].find("\n## ").map_or(tail.len(), |i| i + 4);
    tail[..end].to_owned()
}

fn tar(repo: &ExportRepo, archive: &Path, args: &[&str]) -> Output {
    let mut argv = args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
    argv.insert(1, archive.to_str().unwrap().to_owned());
    command(Path::new("tar"), &repo.scratch.0, &argv, b"", &[])
}

#[test]
fn b23_export_vocabulary_and_complete_schema_section() {
    let mut checks = Checks::new("B23");
    let actual_schema = reference_schema();
    checks.require(
        actual_schema.contains("vocabulary.tsv") && actual_schema.contains("vocabulary("),
        "REFERENCE's schema section still lacks the consolidated vocabulary ABI",
    );
    for schema in [EXPORT_SCHEMA, actual_schema.as_str()] {
        let repo = ExportRepo::new(schema, 1);
        let out = repo.manifest();
        if !checks.success(&out, "release before export") {
            continue;
        }
        git(&repo.tree, &["add", "release-manifest.tsv"]);
        git(&repo.tree, &["commit", "-q", "-m", "fixture manifest"]);
        let out = repo.build();
        if !checks.success(&out, "dist build") {
            continue;
        }
        let archives = fs::read_dir(repo.scratch.0.join("dist"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.to_string_lossy().ends_with(".tar.gz"))
            .collect::<Vec<_>>();
        checks.require(
            archives.len() == 1,
            format!("archive count={}", archives.len()),
        );
        if archives.len() != 1 {
            continue;
        }
        let listing = tar(&repo, &archives[0], &["-tzf"]);
        if !checks.success(&listing, "archive member listing") {
            continue;
        }
        let names = text(&listing.stdout);
        let vocab = names
            .lines()
            .filter(|n| n.ends_with("/data/guidelines/alpha/vocabulary.tsv"))
            .collect::<Vec<_>>();
        checks.require(
            vocab.len() == 1,
            format!("exported vocabulary member count={}", vocab.len()),
        );
        checks.require(
            !names.lines().any(|n| n.ends_with("/temporal.tsv")),
            "retired temporal.tsv member shipped",
        );
        if let Some(member) = vocab.first() {
            let extracted = tar(&repo, &archives[0], &["-xOzf", member]);
            if checks.success(&extracted, "archive vocabulary bytes") {
                checks.require(
                    extracted.stdout == format!("{HEADER}{ALL_ROWS}").as_bytes(),
                    "archive table is not the raw committed vocabulary bytes",
                );
            }
        }
        let readmes = names
            .lines()
            .filter(|n| n.ends_with("/README-dist.md"))
            .collect::<Vec<_>>();
        checks.require(readmes.len() == 1, "README-dist member count");
        if let Some(member) = readmes.first() {
            let extracted = tar(&repo, &archives[0], &["-xOzf", member]);
            if checks.success(&extracted, "archive README-dist bytes") {
                let readme = text(&extracted.stdout);
                checks.require(
                    readme.contains(schema),
                    "README-dist omits or alters bytes inside the schema H2",
                );
                checks.require(
                    !readme.contains("C1_AFTER_SCHEMA_SENTINEL"),
                    "README-dist schema extraction crossed its H2 boundary",
                );
            }
        }
    }
    checks.finish();
}
