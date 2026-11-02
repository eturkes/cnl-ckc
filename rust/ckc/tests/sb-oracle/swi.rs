use super::model::{Program, rendered_model};
use super::oracle::Outcome;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Observed {
    pub truth: BTreeSet<String>,
    pub undefined: BTreeSet<String>,
}

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

struct Stage(PathBuf);

impl Stage {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = root().join("rust/target/sb-oracle").join(format!(
            "{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove private SWI stage");
    }
}

fn runtime() -> String {
    std::env::var("CKC_SB_ORACLE_SWIPL").unwrap_or_else(|_| "swipl".to_owned())
}

fn pinned_runtime() {
    static PIN: OnceLock<()> = OnceLock::new();
    PIN.get_or_init(|| {
        let output = Command::new(runtime())
            .args([
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-g",
                "current_prolog_flag(version_data,V),write_canonical(V),nl,halt",
            ])
            .output()
            .expect("SWI version query");
        assert!(
            output.status.success(),
            "SWI version status {:?}",
            output.status
        );
        assert!(
            output.stderr.is_empty(),
            "SWI version stderr {:?}",
            output.stderr
        );
        assert_eq!(
            String::from_utf8(output.stdout).unwrap().trim(),
            "swi(10,0,2,[])"
        );
    });
}

pub fn parse(text: &str) -> Result<BTreeMap<(String, String), Observed>, String> {
    let mut records = BTreeMap::new();
    let mut current: Option<((String, String), Observed)> = None;
    for line in text.lines() {
        let fields = line.split('\t').collect::<Vec<_>>();
        match fields.as_slice() {
            ["B", id, mode] if current.is_none() && ["plain", "wfs"].contains(mode) => {
                current = Some(((id.to_string(), mode.to_string()), Observed::default()));
            }
            [status @ ("T" | "U"), fact] if fact.starts_with("guideline_") => {
                let (_, model) = current.as_mut().ok_or("fact outside a case")?;
                let target = if *status == "T" {
                    &mut model.truth
                } else {
                    &mut model.undefined
                };
                if !target.insert(fact.to_string()) {
                    return Err(format!("duplicate {status} fact: {fact}"));
                }
            }
            ["E", id, mode] => {
                let (key, model) = current.take().ok_or("end without begin")?;
                if key != (id.to_string(), mode.to_string()) {
                    return Err("mismatched case terminator".to_owned());
                }
                if !model.truth.is_disjoint(&model.undefined) {
                    return Err("fact both true and undefined".to_owned());
                }
                if records.insert(key, model).is_some() {
                    return Err("duplicate case".to_owned());
                }
            }
            _ => return Err(format!("invalid SWI protocol line: {line}")),
        }
    }
    if current.is_some() {
        return Err("truncated model".to_owned());
    }
    Ok(records)
}

pub fn models(programs: &[Program]) -> BTreeMap<(String, String), Observed> {
    pinned_runtime();
    let stage = Stage::new();
    let data = stage.0.join("cases.in");
    let driver = stage.0.join("driver.pl");
    fs::write(
        &data,
        programs.iter().map(Program::render).collect::<String>(),
    )
    .unwrap();
    fs::write(&driver, include_str!("driver.pl.in")).unwrap();
    let schema = std::env::var_os("CKC_SB_ORACLE_SCHEMA")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("guidelines/schema.pl"));
    // The schema is a black-box runtime input; its implementation is never read.
    let output = Command::new("timeout")
        .arg("60s")
        .arg(runtime())
        .args([
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "--threads=false",
            "--stack-limit=512m",
            "--table-space=512m",
            "-s",
        ])
        .arg(&driver)
        .args(["-g", "main", "-t", "halt", "--"])
        .arg(&schema)
        .arg(&data)
        .output()
        .expect("SWI oracle comparison");
    assert!(
        output.status.success(),
        "SWI status {:?}; stderr {}; first case {}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
        programs[0].render()
    );
    assert!(
        output.stderr.is_empty(),
        "SWI stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    let result = parse(&text).unwrap_or_else(|e| panic!("SWI model protocol: {e}"));
    let expected = programs
        .iter()
        .map(|p| if p.refused { 1 } else { 2 })
        .sum::<usize>();
    assert_eq!(result.len(), expected, "every case/mode must terminate");
    result
}

pub fn compare(program: &Program, expected: &Outcome, observed: &Observed) -> Result<(), String> {
    let doc = program.id();
    let truth = rendered_model(&expected.truth, &doc);
    let undefined = rendered_model(&expected.undefined, &doc);
    if truth == observed.truth && undefined == observed.undefined {
        return Ok(());
    }
    let sample = |a: &BTreeSet<String>, b: &BTreeSet<String>| {
        a.difference(b).take(4).cloned().collect::<Vec<_>>()
    };
    Err(format!(
        "{doc}: exact-model mismatch; missing={:?}; added={:?}; undefined-missing={:?}; undefined-added={:?}\n{}",
        sample(&truth, &observed.truth),
        sample(&observed.truth, &truth),
        sample(&undefined, &observed.undefined),
        sample(&observed.undefined, &undefined),
        program.render()
    ))
}
