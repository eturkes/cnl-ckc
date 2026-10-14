// M5.1 align-check suite (.agent/archive/contracts/harness.md H3): tests/align/<case>/
// holds align.tsv.in, src.txt.in, ace.txt.in + expect. Each case runs
// `ckc align-check <align.tsv> <src.txt> <ace.txt>` on a private copy without
// the `.in` suffix: stdout = expect byte for byte, rc 0 iff expect starts `ok `.
// CKC_ALIGN_TEST_BIN selects a prebuilt executable (red replay).
use std::fs;
use std::path::PathBuf;
use std::process::Command;

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

fn program() -> PathBuf {
    std::env::var_os("CKC_ALIGN_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")))
}

// m5u1 CLI seam: align-check takes exactly three operands; any other count is a
// usage error (rc 2, usage on stderr, stdout empty), even when the first three
// operands name a valid case.
#[test]
fn align_check_operand_count_is_usage() {
    let case = root().join("tests/align/001-basic-zero");
    let operands =
        ["align.tsv.in", "src.txt.in", "ace.txt.in", "surplus"].map(|name| case.join(name));
    for count in [0, 1, 2, 4] {
        let out = Command::new(program())
            .arg("align-check")
            .args(&operands[..count])
            .output()
            .unwrap();
        assert_eq!(
            (out.status.code(), out.stdout.as_slice()),
            (Some(2), &b""[..]),
            "{count} operands"
        );
        assert!(
            out.stderr.starts_with(b"usage: ckc "),
            "{count} operands: stderr {:?}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

#[test]
fn align_check_suite() {
    let program = program();
    let suite = root().join("tests/align");
    let scratch = Scratch(root().join(format!("rust/target/align-suite/{}", std::process::id())));
    let mut cases: Vec<PathBuf> = fs::read_dir(&suite)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    cases.sort();
    let mut failures = Vec::new();
    for case in &cases {
        let id = case.file_name().unwrap().to_string_lossy().to_string();
        let dir = scratch.0.join(&id);
        fs::create_dir_all(&dir).unwrap();
        let inputs = ["align.tsv", "src.txt", "ace.txt"].map(|name| {
            let path = dir.join(name);
            fs::copy(case.join(format!("{name}.in")), &path).unwrap();
            path
        });
        let expect = fs::read(case.join("expect")).unwrap();
        let out = Command::new(&program)
            .arg("align-check")
            .args(&inputs)
            .output()
            .unwrap();
        let rc = if expect.starts_with(b"ok ") { 0 } else { 1 };
        if out.stdout != expect || out.status.code() != Some(rc) {
            failures.push(format!(
                "{id}: rc {:?}/{rc} stdout {:?} expect {:?}",
                out.status.code(),
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&expect)
            ));
        }
    }
    assert_eq!(cases.len(), 96, "tests/align case count");
    assert!(
        failures.is_empty(),
        "align suite: {} of {} failed:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}
