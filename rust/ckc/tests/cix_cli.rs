use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static SERIAL: AtomicUsize = AtomicUsize::new(0);

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixtures() -> PathBuf {
    root().join("tests/cix")
}

// The test build's own binary by default, so a mutation run grades the mutated shell.
fn program() -> PathBuf {
    std::env::var_os("CKC_CIX_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")))
        .canonicalize()
        .expect("CKC_CIX_TEST_BIN names an existing binary")
}

#[derive(Debug)]
struct Row {
    case: String,
    mode: String,
    query: String,
    pin: String,
}

fn rows() -> Vec<Row> {
    let text = fs::read_to_string(fixtures().join("cases.tsv")).unwrap();
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("case\tmode\tquery\tpin"));
    lines
        .map(|line| {
            let cells: Vec<_> = line.split('\t').collect();
            assert_eq!(cells.len(), 4, "{line}");
            Row {
                case: cells[0].into(),
                mode: cells[1].into(),
                query: cells[2].into(),
                pin: cells[3].into(),
            }
        })
        .collect()
}

struct Stage(PathBuf);

impl Stage {
    fn new(source: &Path) -> Self {
        let path = root().join(format!(
            "rust/target/cix-cli.{}.{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        for file in fs::read_dir(source).unwrap() {
            let file = file.unwrap();
            let name = file.file_name();
            if let Some(name) = name.to_str().unwrap().strip_suffix(".in") {
                fs::copy(file.path(), path.join(name)).unwrap();
            }
        }
        Self(path)
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn run(row: &Row, binary: &Path) {
    let source = fixtures().join(&row.case);
    let stage = Stage::new(&source);
    let pins = source.join("expect");
    let mut args = vec!["v1", row.mode.as_str(), "manifest.tsv"];
    if row.query != "-" {
        args.push(&row.query);
    }
    if row.mode == "trace" {
        let query = row.query.strip_suffix(".pl").unwrap();
        fs::copy(
            pins.join(format!("{query}.answer.stdout")),
            stage.0.join("answers.pl"),
        )
        .unwrap();
        args.push("answers.pl");
    }
    let output = Command::new(binary)
        .args(&args)
        .current_dir(&stage.0)
        .output()
        .unwrap();
    let rc: i32 = fs::read_to_string(pins.join(format!("{}.rc", row.pin)))
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(rc),
        "{row:?}: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        fs::read(pins.join(format!("{}.stdout", row.pin))).unwrap(),
        "{row:?}: stdout"
    );
    assert_eq!(
        output.stderr,
        fs::read(pins.join(format!("{}.stderr", row.pin))).unwrap(),
        "{row:?}: stderr"
    );
}

fn queries(case: &str, names: &[&str]) {
    let selected: Vec<_> = rows()
        .into_iter()
        .filter(|r| {
            r.case == case
                && names
                    .iter()
                    .any(|name| r.query.strip_suffix(".pl") == Some(name))
        })
        .collect();
    assert_eq!(selected.len(), names.len() * 2);
    let binary = program();
    for row in selected {
        run(&row, &binary);
    }
}

#[test]
fn a01_variable_heads_join_every_class() {
    queries("classes", &["atom_absent", "comp_absent", "arity_absent"]);
}

#[test]
fn a02_integer_heads_remain_candidates() {
    queries("classes", &["int", "negative", "zero", "int_absent"]);
}

#[test]
fn a03_nil_heads_remain_candidates() {
    queries("classes", &["nil"]);
}

#[test]
fn a04_atom_heads_match_by_name() {
    queries("classes", &["alpha", "beta", "atom_absent"]);
}

#[test]
fn a05_compound_names_and_arities() {
    queries(
        "classes",
        &["f1", "f1_child", "f2", "g1", "comp_absent", "arity_absent"],
    );
}

#[test]
fn a06_interleaved_predicates_keep_original_positions() {
    queries("classes", &["variable", "bound_later"]);
}

#[test]
fn a07_goal_classes_and_late_binding() {
    queries(
        "classes",
        &[
            "variable",
            "int",
            "nil",
            "alpha",
            "f1",
            "f2",
            "comp_absent",
            "bound_later",
        ],
    );
}

#[test]
fn a08_class_wild_merge_keeps_solutions_and_first_proofs() {
    queries("classes", &["variable", "alpha", "f1"]);
}

#[test]
fn a09_redo_after_rigid_and_occurs_clashes() {
    queries("redo", &["occurs", "child_clash", "finite_failure"]);
}

#[test]
fn a10_negation_and_waiting_operator_goals() {
    queries("selection", &["body", "question"]);
    queries("bounds", &["wh", "yesno"]);
}

#[test]
fn a11_witness_prefix_classes_order_and_reuse() {
    let selected: Vec<_> = rows()
        .into_iter()
        .filter(|r| r.mode == "aggregate-check")
        .collect();
    assert_eq!(selected.len(), 3);
    let binary = program();
    for row in selected {
        run(&row, &binary);
    }
}

#[test]
fn a12_all_streams_and_return_codes() {
    let rows = rows();
    let mut counts = BTreeMap::new();
    for row in &rows {
        *counts.entry(row.mode.as_str()).or_insert(0) += 1;
    }
    assert_eq!(
        counts,
        BTreeMap::from([("aggregate-check", 3), ("answer", 24), ("trace", 23)])
    );
    let binary = program();
    for row in rows {
        run(&row, &binary);
    }
}
