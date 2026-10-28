// SUP1 P1/P2/P5b/P6; contract-only models. CKC_SUP1_TEST_BIN selects CLI lanes;
// raw-byte vocabulary properties exercise the linked tree's public kernel binding.
#[path = "../../../tests/cs1-surface/model.rs"]
mod base;
#[path = "../../../tests/bf/reorder.rs"]
mod binding_first;
#[path = "../../../tests/sup1-surface/model.rs"]
mod model;
#[path = "../../../tests/sup1-surface/portability.rs"]
mod portability;
#[path = "../../../tests/sup1-surface/support.rs"]
mod support;
#[path = "../../../tests/sup1-surface/surface.rs"]
mod surface;
#[path = "../../../tests/sup1-surface/table.rs"]
mod table;

// The child-run probe of `run_wide_stage_leaves_nothing`: the staged compiler (this binary's
// run-wide cache) is present while held.
#[test]
fn stage_probe() {
    let stage = support::compiler();
    let pid = std::process::id();
    assert!(stage.stage.join("prolog/ace_to_pl.pl").is_file());
    let name = stage
        .stage
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    assert!(name.starts_with(&format!("stage-{pid}-")), "{name}");
}

// Setup + teardown stay symmetric (REV R5-R2): a child process runs the probe, and once it exits
// its stage directory is gone.
#[test]
fn run_wide_stage_leaves_nothing() {
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["stage_probe", "--exact"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let pid = child.id();
    let out = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success() && stdout.contains("test result: ok. 1 passed;"),
        "child stage_probe: {stdout}{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let left: Vec<String> = std::fs::read_dir(support::root().join("rust/target/sup1-surface"))
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(&format!("stage-{pid}-")))
        .collect();
    assert!(left.is_empty(), "stage left: {left:?}");
}
