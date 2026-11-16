use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[test]
fn knowledge_preparation_never_reads_or_retains_the_patient_suffix() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap();
    let out = root.join(format!(
        "rust/target/bench/builder-20-kb-load-tests-{}",
        std::process::id()
    ));
    fs::create_dir_all(out.parent().unwrap()).unwrap();
    fs::create_dir(&out).unwrap();
    let case = Case(out);
    fs::write(
        case.0.join("kb.pl.in"),
        "guideline_entity(actual,a,seed,countable).\n",
    )
    .unwrap();
    let composition = case.0.join("composition.tsv");
    fs::write(
        &composition,
        "# patient_documents\t1\nkb.pl.in\nmissing-patient.pl.in\n",
    )
    .unwrap();
    let kb = sb1_ta::source::Program::load_knowledge(&case.0, &composition).unwrap();
    assert_eq!(kb.documents.len(), 1);
    assert_eq!(kb.clauses.len(), 1);
    assert!(!kb.patient(0));
    assert!(sb1_ta::source::Program::load(&case.0, &composition, true).is_err());
}

struct Case(PathBuf);
impl Drop for Case {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
}
#[test]
fn prepared_input_keeps_p3_comments_out_of_dc_statement_grouping() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap();
    let out = root.join(format!(
        "rust/target/bench/builder-20-cli-tests-{}",
        std::process::id()
    ));
    fs::create_dir_all(out.parent().unwrap()).unwrap();
    fs::create_dir(&out).unwrap();
    let case = Case(out);
    let composition = case.0.join("composition.tsv");
    fs::write(
        &composition,
        format!(
            "# patient_documents\t0\n{}\n",
            root.join("prototype/sb1/ta/plants/P3.pl.in").display()
        ),
    )
    .unwrap();
    let run = |args: &[&str]| -> Output {
        Command::new(env!("CARGO_BIN_EXE_sb1-tacheck"))
            .arg(root)
            .arg(&composition)
            .args(args)
            .output()
            .unwrap()
    };
    let before = run(&[]);
    assert_eq!(before.status.code(), Some(1));
    assert!(before.stderr.starts_with(b"refused (negative cycle)\n"));
    let prepared = run(&["--already-shaped"]);
    assert_eq!(
        prepared.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&prepared.stderr)
    );
    assert_eq!(prepared.stdout, b"admitted\tclauses=3\tedges=2\n");
    assert!(prepared.stderr.is_empty());
    let usage = run(&["--not-a-mode"]);
    assert_eq!(usage.status.code(), Some(2));
    assert!(usage.stdout.is_empty());
}
