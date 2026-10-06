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

#[test]
fn align_check_suite() {
    let program = std::env::var_os("CKC_ALIGN_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
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
    assert_eq!(cases.len(), 94, "tests/align case count");
    assert!(
        failures.is_empty(),
        "align suite: {} of {} failed:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}
