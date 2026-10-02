// M5.4 `align` resolver probes (.agent/contracts/harness.md H5): tests/align-probes/
// cases.tsv rows (25 red, 6 green, 2 guards) run `ckc align <gid> <docid>` with
// stdin.tsv in a private tree holding the probe guideline (fixture/, `.in`
// dropped) after the row's setup. Grade = expected.{rc,stdout,stderr}; green rows
// also write align/<docid>.tsv = expected.tsv, every other row leaves the prior
// output file byte-unchanged. CKC_ALIGN_TEST_BIN selects a prebuilt executable.
use std::fs;
use std::io::Write;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn materialize(from: &Path, to: &Path) {
    for entry in fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap().to_owned();
        if path.is_dir() {
            materialize(&path, &to.join(name));
        } else {
            fs::create_dir_all(to).unwrap();
            fs::copy(&path, to.join(name.strip_suffix(".in").unwrap())).unwrap();
        }
    }
}

// The setups of align_probes/run.py `fixture`, applied to the base probe guideline.
fn setup(tree: &Path, kind: &str) {
    let parent = tree.join("guidelines");
    let probe = parent.join("probe");
    match kind {
        "base" => {}
        "guideline-symlink" => {
            fs::rename(&probe, parent.join("probe-target")).unwrap();
            symlink("probe-target", &probe).unwrap();
        }
        "guideline-not-dir" => {
            fs::remove_dir_all(&probe).unwrap();
            fs::write(&probe, b"not a directory\n").unwrap();
        }
        "ace-unreadable" => {
            fs::set_permissions(probe.join("ace/doc.ace"), fs::Permissions::from_mode(0o000))
                .unwrap();
        }
        "align-symlink" => {
            fs::rename(probe.join("align"), probe.join("align-target")).unwrap();
            symlink("align-target", probe.join("align")).unwrap();
        }
        "unclaimed-doc" | "inexpressible-region" => {
            let path = probe.join("coverage.tsv");
            let text = fs::read_to_string(&path).unwrap();
            assert_eq!(text.matches("\tace(doc)\n").count(), 1);
            let status = if kind == "unclaimed-doc" {
                "\tpending\n"
            } else {
                "\tuncovered(inexpressible: dosing arithmetic beyond v1)\n"
            };
            fs::write(&path, text.replace("\tace(doc)\n", status)).unwrap();
        }
        other => panic!("unknown setup {other}"),
    }
}

#[test]
fn align_probe_suite() {
    let program = std::env::var_os("CKC_ALIGN_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    let suite = root().join("tests/align-probes");
    let scratch = Scratch(root().join(format!("rust/target/align-probes/{}", std::process::id())));
    let _ = fs::remove_dir_all(&scratch.0);
    let table = fs::read_to_string(suite.join("cases.tsv")).unwrap();
    let rows: Vec<Vec<&str>> = table
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(|l| l.split('\t').collect())
        .collect();
    let mut failures = Vec::new();
    for row in &rows {
        let (id, kind, gid, docid) = (row[0], row[1], row[2], row[3]);
        let case = suite.join(id);
        let tree = scratch.0.join(id);
        materialize(&suite.join("fixture"), &tree);
        setup(&tree, kind);
        let target = tree.join("guidelines/probe/align/doc.tsv");
        let before = fs::read(&target).ok();
        let mut child = Command::new(&program)
            .args(["align", gid, docid])
            .current_dir(&tree)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        // A probe rejected before `ckc align` reads stdin closes the pipe first (EPIPE);
        // the rc/stdout/stderr pins below still grade the run.
        match child
            .stdin
            .take()
            .unwrap()
            .write_all(&fs::read(case.join("stdin.tsv.in")).unwrap())
        {
            Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => panic!("{id}: stdin: {e}"),
            _ => {}
        }
        let out = child.wait_with_output().unwrap();
        let read = |name: &str| fs::read(case.join(name)).unwrap();
        let rc: i32 = String::from_utf8(read("expected.rc"))
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let mut problems = Vec::new();
        if out.status.code() != Some(rc) {
            problems.push(format!("rc {:?} expected {rc}", out.status.code()));
        }
        if out.stdout != read("expected.stdout") {
            problems.push(format!("stdout {:?}", String::from_utf8_lossy(&out.stdout)));
        }
        if out.stderr != read("expected.stderr") {
            problems.push(format!("stderr {:?}", String::from_utf8_lossy(&out.stderr)));
        }
        if case.join("expected.tsv").is_file() {
            if fs::read(&target).ok() != Some(read("expected.tsv")) {
                problems.push("align output differs from expected.tsv".into());
            }
        } else if before.is_some() && fs::read(&target).ok() != before {
            problems.push("prior align output changed after a refusal".into());
        }
        if kind == "ace-unreadable" {
            let _ = fs::set_permissions(
                tree.join("guidelines/probe/ace/doc.ace"),
                fs::Permissions::from_mode(0o644),
            );
        }
        if !problems.is_empty() {
            failures.push(format!("{id}: {}", problems.join("; ")));
        }
    }
    assert_eq!(rows.len(), 34, "tests/align-probes row count");
    assert!(
        failures.is_empty(),
        "align probes: {} of {} failed:\n{}",
        failures.len(),
        rows.len(),
        failures.join("\n")
    );
}
