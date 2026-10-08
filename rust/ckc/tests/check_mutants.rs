// `ckc check` corpus mutants (.agent/archive/contracts/harness.md H6): the 36 M5.3 red
// mutants whose first violation lands before the SWI-Prolog stage (mutants.py
// `mutate`, byte for byte) + 3 `.agent/spec.md` shape plants + 1 ledger bundle-mismatch plant
// + 11 temporal.tsv plants (one per m7t D1 rejection). Each row mutates a
// fresh clone of HEAD and must end in its pinned rc with the pinned first
// violation: whole stderr when nonempty, else the last stdout line.
// CKC_CHECK_TEST_BIN selects a prebuilt executable.
use std::fs;
use std::io::Read;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

const G: &str = "guidelines/cdc-2022-opioid";
const DOC: &str = "guidelines/cdc-2022-opioid/pl/cdc2022-opioid-rec01-imp01.pl";
const COVERAGE: &str = "guidelines/cdc-2022-opioid/coverage.tsv";
const PROJECTION: &str = "guidelines/cdc-2022-opioid/audit/projection-notes.tsv";
const CENSUS: &str = "guidelines/cdc-2022-opioid/audit/census-map.tsv";
const MANIFEST: &str = "guidelines/cdc-2022-opioid/audit/review-manifest.tsv";
const LEDGER: &str = "guidelines/cdc-2022-opioid/audit/adjudication.tsv";
const ULEX: &str = "guidelines/cdc-2022-opioid/lexicon.ulex";
const SHADOW: &str = "guidelines/cdc-2022-opioid/audit/lexicon-shadow.tsv";
const EVIDENCE: &str = "guidelines/cdc-2022-opioid/source/box3-extraction.txt";
const SPEC: &str = ".agent/spec.md";
const TEMPORAL: &str = "guidelines/cdc-2022-opioid/temporal.tsv";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}

fn lines(bytes: &[u8]) -> Vec<Vec<u8>> {
    bytes
        .split_inclusive(|&b| b == b'\n')
        .map(<[u8]>::to_vec)
        .collect()
}

fn replace(t: &Path, rel: &str, old: &[u8], new: &[u8]) {
    let data = fs::read(t.join(rel)).unwrap();
    let hits: Vec<usize> = (0..=data.len().saturating_sub(old.len()))
        .filter(|&i| data[i..].starts_with(old))
        .collect();
    assert_eq!(hits.len(), 1, "{rel}: anchor count");
    fs::write(
        t.join(rel),
        [&data[..hits[0]], new, &data[hits[0] + old.len()..]].concat(),
    )
    .unwrap();
}

fn append(t: &Path, rel: &str, data: &[u8]) {
    let mut bytes = fs::read(t.join(rel)).unwrap();
    bytes.extend_from_slice(data);
    fs::write(t.join(rel), bytes).unwrap();
}

fn truncate_to(t: &Path, rel: &str, keep: usize) {
    let mut bytes = fs::read(t.join(rel)).unwrap();
    bytes.truncate(keep);
    fs::write(t.join(rel), bytes).unwrap();
}

fn new_file(t: &Path, rel: &str, data: &[u8]) {
    let path = t.join(rel);
    assert!(fs::symlink_metadata(&path).is_err(), "{rel} exists");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, data).unwrap();
}

fn remove(t: &Path, rel: &str) {
    fs::remove_file(t.join(rel)).unwrap();
}

fn first_row(t: &Path, rel: &str) -> Vec<u8> {
    lines(&fs::read(t.join(rel)).unwrap())
        .into_iter()
        .find(|r| !r.starts_with(b"#"))
        .unwrap()
}

fn fields(row: &[u8]) -> Vec<Vec<u8>> {
    row.strip_suffix(b"\n")
        .unwrap()
        .split(|&b| b == b'\t')
        .map(<[u8]>::to_vec)
        .collect()
}

fn joined(fields: &[Vec<u8>]) -> Vec<u8> {
    let mut row = fields.join(&b'\t');
    row.push(b'\n');
    row
}

// Rewrites field `i` of the first data row of `rel`.
fn set_field(t: &Path, rel: &str, i: usize, value: &[u8]) {
    let row = first_row(t, rel);
    let mut f = fields(&row);
    f[i] = value.to_vec();
    replace(t, rel, &row, &joined(&f));
}

fn flip_first(field: &[u8]) -> Vec<u8> {
    [&[if field[0] != b'0' { b'0' } else { b'1' }], &field[1..]].concat()
}

fn ledger(t: &Path, commit: &[u8], date: &[u8], digest: Option<&[u8]>) {
    let f = fields(&first_row(t, MANIFEST));
    let row = joined(&[
        f[0].clone(),
        digest.map_or(f[f.len() - 1].clone(), <[u8]>::to_vec),
        commit.to_vec(),
        b"approved".to_vec(),
        b"parity".to_vec(),
        date.to_vec(),
        Vec::new(),
    ]);
    let header = b"# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n";
    new_file(t, LEDGER, &[header.as_slice(), &row].concat());
}

const LEDGER_DATE: &[u8] = b"2026-09-14T00:00:00Z";

fn mutate(t: &Path, name: &str) {
    match name {
        "temporal-header" => replace(t, TEMPORAL, b"# format: kind\tlemma\tvalue\n", b"# format: drift\n"),
        "temporal-no-rows" => {
            let header = lines(&fs::read(t.join(TEMPORAL)).unwrap())[..2].concat();
            truncate_to(t, TEMPORAL, header.len());
        }
        "temporal-final-newline" => {
            let len = fs::read(t.join(TEMPORAL)).unwrap().len();
            truncate_to(t, TEMPORAL, len - 1);
        }
        "temporal-field-count" => append(t, TEMPORAL, b"unit\tminute\n"),
        "temporal-lemma" => append(t, TEMPORAL, b"unit\tmy minute\tminute\n"),
        "temporal-unit-id" => append(t, TEMPORAL, b"unit\tfortnight\tfortnight\n"),
        "temporal-duplicate-noun" => append(t, TEMPORAL, b"unit\tday\tweek\n"),
        "temporal-role-id" => append(t, TEMPORAL, b"relation\tthrough\tthrough\n"),
        "temporal-duplicate-preposition" => append(t, TEMPORAL, b"relation\tfor\tafter\n"),
        "temporal-frame-lemma" => append(t, TEMPORAL, b"spacing\tevery\t\n"),
        "temporal-kind" => append(t, TEMPORAL, b"frequency\tdaily\tday\n"),
        "fork-entry" => new_file(t, "vendor/000-parity", b"not a vendor directory\n"),
        "vendor-license" => replace(t, "vendor/clex/PROVENANCE", b"License: GPL-3.0-or-later\n", b""),
        "pristine-digest" => {
            let row = lines(&fs::read(t.join("vendor/clex/MANIFEST.sha256")).unwrap())[0].clone();
            replace(t, "vendor/clex/MANIFEST.sha256", &row, &flip_first(&row));
        }
        "adjudication-fixture-pin" => append(t, "tests/adjudication/green-absent-ledger/golden", b"\n"),
        "compendium-header-cr" => replace(
            t,
            ".agent/compendium.tsv",
            b"org\ttitle (year)\tURL\taccess\tstatus\tnotes\n",
            b"org\r\ttitle (year)\tURL\taccess\tstatus\tnotes\n",
        ),
        "compendium-org-class" => {
            let rows = lines(&fs::read(t.join(".agent/compendium.md")).unwrap());
            let hits: Vec<&Vec<u8>> = rows
                .iter()
                .filter(|r| r.starts_with(b"| Advisory Committee on Immunization Practices |"))
                .collect();
            assert_eq!(hits.len(), 1, "ACIP org row");
            let row = hits[0].clone();
            let text = String::from_utf8(row.clone()).unwrap().replace("| federal |", "| invalid |");
            replace(t, ".agent/compendium.md", &row, text.as_bytes());
        }
        "compendium-row-status" => {
            let row = lines(&fs::read(t.join(".agent/compendium.tsv")).unwrap())[1].clone();
            let mut f = fields(&row);
            f[4] = b"invalid".to_vec();
            replace(t, ".agent/compendium.tsv", &row, &joined(&f));
        }
        "spec-tasks-missing" => replace(t, SPEC, b"\n## Tasks\n", b"\n"),
        "spec-tick-sha" => replace(t, SPEC, b"\n## Tasks\n\n", b"\n## Tasks\n\n- [x] U9 parity unit\n"),
        "spec-tasks-last" => replace(t, SPEC, b"\n\n## Phase\n", b"\n- [ ] U9 parity unit\n\n## Phase\n"),
        "source-readme-missing" => remove(t, &format!("{G}/README.md")),
        "source-evidence-symlink" => {
            symlink("box3-extraction.txt", t.join(G).join("source/000-parity-link")).unwrap()
        }
        "pl-orphan" => new_file(t, &format!("{G}/pl/extra.txt"), b"extra\n"),
        "align-missing" => remove(t, &format!("{G}/align/cdc2022-opioid-rec01-imp01.tsv")),
        "prolog-index-orphan" => {
            new_file(t, "parity-mutant.pl", b"parity_mutant.\n");
            let blob = String::from_utf8(git(t, &["hash-object", "-w", "parity-mutant.pl"])).unwrap();
            let info = format!("100644,{},parity-mutant.pl", blob.trim());
            git(t, &["update-index", "--add", "--cacheinfo", &info]);
        }
        "projection-docid-join" => {
            let row = first_row(t, PROJECTION);
            let tail = &row[row.iter().position(|&b| b == b'\t').unwrap() + 1..];
            replace(t, PROJECTION, &row, &[b"parity-unknown\t".as_slice(), tail].concat());
        }
        "projection-header" => replace(
            t,
            PROJECTION,
            b"# format: docid<TAB>region<TAB>kept<TAB>dropped\n",
            b"# format: drift\n",
        ),
        "product-functor" => append(t, DOC, b"parity_mutant(x).\n"),
        "coverage-status" => set_field(t, COVERAGE, 4, b"invalid"),
        "coverage-file-outside" => set_field(t, COVERAGE, 1, b"elsewhere.txt"),
        "coverage-self-restatement" => {
            let id = fields(&first_row(t, COVERAGE))[0].clone();
            set_field(t, COVERAGE, 4, &[b"restates(".as_slice(), &id, b")"].concat());
        }
        "census-header" => replace(
            t,
            CENSUS,
            b"# format: census<TAB>region<TAB>disposition\n",
            b"# format: drift\n",
        ),
        "manifest-digest" => {
            let f = fields(&first_row(t, MANIFEST));
            set_field(t, MANIFEST, 1, &flip_first(&f[1]));
        }
        "ledger-commit-absent" => ledger(t, &[b'f'; 40], LEDGER_DATE, None),
        "ledger-bundle-mismatch" => {
            // Current digest, recorded commit = one whose s26-07 bundle (clauses) differs.
            let row = b"cdc2022-opioid-s26-07\t76f2750e20462707ff56a4374132c7df935a68c3ad8602103b76558a332ca4f2\ta1bcb8cd328cd54d04aa3597bd05dfaa57719341\tapproved\tparity\t2026-09-14T00:00:00Z\t\n";
            let header = b"# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n";
            new_file(t, LEDGER, &[header.as_slice(), row].concat());
        }
        "ledger-date" => ledger(t, b"", b"2026-02-30T00:00:00Z", None),
        "ledger-bad-digest" => ledger(t, b"", LEDGER_DATE, Some(&[b'g'; 64])),
        "lexicon-duplicate" => {
            let row = lines(&fs::read(t.join(ULEX)).unwrap())[0].clone();
            append(t, ULEX, &row);
        }
        "coverage-duplicate" => {
            let row = first_row(t, COVERAGE);
            replace(t, COVERAGE, &row, &[row.as_slice(), &row].concat());
        }
        "coverage-comment-after" => append(t, COVERAGE, b"# parity late comment\n"),
        "coverage-census-count" => replace(
            t,
            EVIDENCE,
            b"identify the 16 payloads below",
            b"identify the 17 payloads below",
        ),
        "coverage-evidence-symlink" => {
            let old = b"\tsource/box3-extraction.txt\t";
            let rows: Vec<Vec<u8>> = lines(&fs::read(t.join(COVERAGE)).unwrap())
                .into_iter()
                .filter(|r| r.windows(old.len()).any(|w| w == old))
                .collect();
            assert_eq!(rows.len(), 16, "box3 row count");
            for row in rows {
                let text = String::from_utf8(row.clone())
                    .unwrap()
                    .replace("\tsource/box3-extraction.txt\t", "\tsource/parity-evidence/box3.txt\t");
                replace(t, COVERAGE, &row, text.as_bytes());
            }
            let dir = t.join(G).join("source/parity-evidence");
            fs::create_dir(&dir).unwrap();
            symlink("../box3-extraction.txt", dir.join("box3.txt")).unwrap();
        }
        "lexicon-stale-ruling" => append(
            t,
            SHADOW,
            b"noun_sg(parity-shadow,parity-shadow,neutr)\tnoun_mass(parity-shadow,parity-shadow,neutr)\tparity\n",
        ),
        "semantic-undotted-clause" => replace(
            t,
            DOC,
            b"guideline_schema_version(2).\n",
            b"guideline_schema_version(2)\n",
        ),
        "semantic-undotted-record" => {
            let rows: Vec<Vec<u8>> = lines(&fs::read(t.join(DOC)).unwrap())
                .into_iter()
                .filter(|r| r.starts_with(b"guideline_document("))
                .collect();
            assert_eq!(rows.len(), 1, "document record");
            let row = &rows[0];
            replace(t, DOC, row, &[&row[..row.len() - 2], b"\n"].concat());
        }
        "projection-region-join" => set_field(t, PROJECTION, 1, b"B3-01"),
        "census-unknown-region" => set_field(t, CENSUS, 1, b"PARITY-MISSING"),
        "manifest-missing" => remove(t, MANIFEST),
        "lexicon-shadow-missing" => {
            let row = lines(&fs::read(t.join(SHADOW)).unwrap())[1].clone();
            replace(t, SHADOW, &row, b"");
        }
        "red-pin-missing" => remove(t, "tests/red/ape_messages--unknown-word.expect"),
        other => panic!("unknown mutant {other}"),
    }
}

// 180 s cap: a mutant the checker misses runs on into the SWI-Prolog stage.
fn check(program: &Path, tree: &Path, swipl: Option<&Path>) -> Result<Output, String> {
    let mut command = Command::new(program);
    command
        .arg("check")
        .current_dir(tree)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(swipl) = swipl {
        command.env("SWIPL", swipl);
    }
    let mut child = command.spawn().unwrap();
    let mut pipes = (child.stdout.take().unwrap(), child.stderr.take().unwrap());
    let out = std::thread::spawn(move || {
        let (mut a, mut b) = (Vec::new(), Vec::new());
        let err = std::thread::spawn(move || {
            pipes.1.read_to_end(&mut b).unwrap();
            b
        });
        pipes.0.read_to_end(&mut a).unwrap();
        (a, err.join().unwrap())
    });
    let deadline = Instant::now() + Duration::from_secs(180);
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            let (stdout, stderr) = out.join().unwrap();
            return Ok(Output {
                status,
                stdout,
                stderr,
            });
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err("timeout 180 s (no violation before the SWI-Prolog stage)".into());
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn first_violation(out: &Output) -> Vec<u8> {
    if out.status.code() == Some(0) {
        Vec::new()
    } else if !out.stderr.is_empty() {
        out.stderr.clone()
    } else {
        lines(&out.stdout).pop().unwrap_or_default()
    }
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn program() -> PathBuf {
    std::env::var_os("CKC_CHECK_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")))
}

fn clone_head(dir: &Path, name: &str, head: &str) -> PathBuf {
    git(
        dir,
        &[
            "clone",
            "-q",
            "--shared",
            "--no-checkout",
            root().to_str().unwrap(),
            name,
        ],
    );
    let tree = dir.join(name);
    git(&tree, &["checkout", "-q", "--detach", head.trim()]);
    tree
}

#[test]
fn check_mutant_battery() {
    let program = program();
    let suite = root().join("tests/check-mutants");
    let scratch = Scratch(root().join(format!("rust/target/check-mutants/{}", std::process::id())));
    let _ = fs::remove_dir_all(&scratch.0);
    fs::create_dir_all(&scratch.0).unwrap();
    let head = String::from_utf8(git(&root(), &["rev-parse", "HEAD"])).unwrap();
    let table = fs::read_to_string(suite.join("cases.tsv")).unwrap();
    let rows: Vec<Vec<&str>> = table
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(|l| l.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 51, "tests/check-mutants row count");
    // Each worker clones HEAD once, then per row: mutate → check → reset + clean
    // back to HEAD (a fresh checkout per row costs ~10× the check itself).
    let next = AtomicUsize::new(0);
    let failures = Mutex::new(Vec::new());
    let workers = std::thread::available_parallelism().map_or(4, |n| n.get().min(4));
    std::thread::scope(|scope| {
        for w in 0..workers {
            let (scratch, head, rows, next, failures, suite, program) =
                (&scratch, &head, &rows, &next, &failures, &suite, &program);
            scope.spawn(move || {
                let tree = clone_head(&scratch.0, &format!("worker-{w}"), head);
                while let Some(row) = rows.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let (name, rc) = (row[0], row[2].parse::<i32>().unwrap());
                    mutate(&tree, name);
                    let expect = fs::read(suite.join(format!("{name}.expect"))).unwrap();
                    let verdict = match check(program, &tree, None) {
                        Err(e) => Some(e),
                        Ok(out)
                            if out.status.code() != Some(rc) || first_violation(&out) != expect =>
                        {
                            Some(format!(
                                "rc {:?} expected {rc}, first violation {:?}",
                                out.status.code(),
                                String::from_utf8_lossy(&first_violation(&out))
                            ))
                        }
                        Ok(_) => None,
                    };
                    if let Some(v) = verdict {
                        failures.lock().unwrap().push(format!("{name}: {v}"));
                    }
                    git(&tree, &["reset", "-q", "--hard", head.trim()]);
                    git(&tree, &["clean", "-q", "-f", "-d", "-x"]);
                    assert!(
                        git(&tree, &["status", "--porcelain", "--ignored"]).is_empty(),
                        "{name}: reset left changes"
                    );
                }
            });
        }
    });
    let mut failures = failures.into_inner().unwrap();
    failures.sort();
    assert!(
        failures.is_empty(),
        "check mutants: {} of {} failed:\n{}",
        failures.len(),
        rows.len(),
        failures.join("\n")
    );
}

// A kill skips the `ckc check` scratch Drop: a leftover whose creator pid is gone
// must neither stop the run nor outlive it. The fake swipl ends the run at the APE
// stage build, the first step past the scratch.
#[test]
fn check_scratch_leftover() {
    let scratch = Scratch(root().join(format!("rust/target/check-scratch/{}", std::process::id())));
    let _ = fs::remove_dir_all(&scratch.0);
    fs::create_dir_all(&scratch.0).unwrap();
    let head = String::from_utf8(git(&root(), &["rev-parse", "HEAD"])).unwrap();
    let tree = clone_head(&scratch.0, "tree", &head);
    let mut child = Command::new("true").spawn().unwrap();
    let gone = child.id();
    child.wait().unwrap();
    for name in [format!(".goal.tmp.{gone}"), format!(".goal.tmp.{gone}.0")] {
        new_file(&tree, &format!("{name}/ape-stage/leftover"), b"x");
    }
    let swipl = scratch.0.join("swipl");
    fs::write(
        &swipl,
        "#!/bin/sh\n[ \"$1\" = --version ] && { echo 'SWI-Prolog version 9.2.9 for x86_64-linux'; exit 0; }\necho 'fake swipl: stage build' >&2\nexit 3\n",
    )
    .unwrap();
    fs::set_permissions(&swipl, fs::Permissions::from_mode(0o755)).unwrap();
    let out = check(&program(), &tree, Some(&swipl)).unwrap();
    assert_eq!(
        (out.status.code(), String::from_utf8_lossy(&out.stderr)),
        (Some(3), "fake swipl: stage build\n".into()),
        "stdout {}",
        String::from_utf8_lossy(&out.stdout)
    );
    let left: Vec<_> = fs::read_dir(&tree)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(".goal.tmp."))
        .collect();
    assert!(left.is_empty(), "scratch left: {left:?}");
}
