#[path = "../../../../rust/ckc/tests/sb-oracle/domain.rs"]
mod domain;
#[path = "../../../../rust/ckc/tests/sb-oracle/model.rs"]
mod model;
#[path = "../../../../rust/ckc/tests/sb-oracle/numbers.rs"]
mod numbers;
#[path = "../../../../rust/ckc/tests/sb-oracle/oracle.rs"]
mod oracle;

use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;

struct BatchModel {
    model: String,
    certificate: String,
    failure: Option<String>,
}
fn parse(text: &str) -> Result<BTreeMap<String, BatchModel>, String> {
    let mut result = BTreeMap::new();
    let mut current: Option<(String, BatchModel)> = None;
    let mut section = "";
    for line in text.lines() {
        if let Some(id) = line.strip_prefix("B\t") {
            if current.is_some() {
                return Err("nested batch begin".into());
            }
            current = Some((
                id.into(),
                BatchModel {
                    model: String::new(),
                    certificate: String::new(),
                    failure: None,
                },
            ));
            section = "";
        } else if let Some(id) = line.strip_prefix("E\t") {
            let (expected, record) = current.take().ok_or("batch end without begin")?;
            if expected != id || section != "certificate" {
                return Err("batch end/section".into());
            }
            if result.insert(expected, record).is_some() {
                return Err("duplicate batch id".into());
            }
        } else if let Some(failure) = line.strip_prefix("F\t") {
            if current.is_some() {
                return Err("batch failure inside model".into());
            }
            let (id, detail) = failure.split_once('\t').ok_or("batch failure fields")?;
            if result
                .insert(
                    id.into(),
                    BatchModel {
                        model: String::new(),
                        certificate: String::new(),
                        failure: Some(detail.into()),
                    },
                )
                .is_some()
            {
                return Err("duplicate batch failure id".into());
            }
        } else if line == "M" && current.is_some() && section.is_empty() {
            section = "model";
        } else if line == "C" && current.is_some() && section == "model" {
            section = "certificate";
        } else {
            let (_, record) = current.as_mut().ok_or("batch data outside model")?;
            let target = match section {
                "model" => &mut record.model,
                "certificate" => &mut record.certificate,
                _ => return Err("batch data before section".into()),
            };
            target.push_str(line);
            target.push('\n');
        }
    }
    if current.is_some() {
        return Err("truncated batch".into());
    }
    Ok(result)
}
fn source(program: &model::Program) -> String {
    program
        .clauses
        .iter()
        .map(|c| {
            let head = c.head.render(&program.id());
            if c.body.is_empty() {
                format!("{head}.\n")
            } else {
                format!(
                    "{head} :- {}.\n",
                    model::render_goals(&c.body, &program.id())
                )
            }
        })
        .collect()
}
fn check_model(
    program: &model::Program,
    record: &BatchModel,
    directory: &Path,
) -> Result<(), String> {
    let mut arena = Arena::default();
    let clauses = parse_clauses(&mut arena, &source(program), 1)?;
    program::admit(&arena, &clauses)?;
    let request = Request {
        root: PathBuf::new(),
        composition: PathBuf::new(),
        model: directory.join("model.txt"),
        certificate: directory.join("certificate.txt"),
        query: None,
    };
    fs::write(&request.model, &record.model).unwrap();
    fs::write(&request.certificate, &record.certificate).unwrap();
    let mut counters = Counters::default();
    let expected = sha256(program.render().as_bytes());
    let (mut proposed, claims, query) = decode_expected(
        &request,
        &mut arena,
        &clauses,
        &expected,
        None,
        &mut counters,
    )
    .map_err(|e| e.message)?;
    proposed.build_indices(&arena, &clauses, query.goals(), &mut counters);
    engine::support(&mut arena, &proposed, &clauses, &claims, &mut counters)
        .map_err(|e| e.message)?;
    engine::closure(&mut arena, &proposed, &clauses, &mut counters).map_err(|e| e.message)
}
fn batch(
    programs: &[model::Program],
    directory: &Path,
    root: &Path,
    summary: &mut String,
    errors: &mut Vec<String>,
) {
    let input = directory.join("cases.in");
    let text = programs
        .iter()
        .map(|program| {
            format!(
                "sb1_case({},{},[{}]).\n",
                program.id(),
                model::quote(&sha256(program.render().as_bytes())),
                program
                    .clauses
                    .iter()
                    .map(|c| c.render(&program.id()))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect::<String>();
    fs::write(&input, text).unwrap();
    let output = Command::new("timeout")
        .args(["120", "swipl", "-q", "-f", "none", "-F", "none", "-s"])
        .arg(root.join("prototype/sb1/oracle-preflight.prolog"))
        .args(["-g", "sb1_batch_main", "-t", "halt(9)", "--"])
        .arg(root)
        .arg(input)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "batch status {:?}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "batch stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut observed = parse(std::str::from_utf8(&output.stdout).unwrap()).unwrap();
    assert_eq!(observed.len(), programs.len());
    for program in programs {
        let expected = oracle::evaluate(program);
        let statistics = &expected.statistics;
        assert!(statistics.values <= 256 && statistics.schema_heads <= 512);
        assert!(statistics.ground_clauses <= 4096 && statistics.identity_depth <= 2);
        assert!(statistics.rounds > 0);
        let transformed = oracle::evaluate(&program.transform());
        assert_eq!(
            transformed.truth,
            expected
                .truth
                .iter()
                .map(model::Literal::transform)
                .collect()
        );
        assert_eq!(
            transformed.undefined,
            expected
                .undefined
                .iter()
                .map(model::Literal::transform)
                .collect()
        );
        let record = observed.remove(&program.id()).expect("every case observed");
        let check = record
            .failure
            .clone()
            .map_or_else(|| check_model(program, &record, directory), Err);
        let actual = record
            .model
            .lines()
            .map(|line| line.strip_suffix('.').expect("model period").to_string())
            .collect::<BTreeSet<_>>();
        let exact = actual == model::rendered_model(&expected.truth, &program.id());
        let matches = if program.refused {
            assert_eq!(expected.undefined, program.expected_undefined);
            check.is_err()
        } else {
            assert!(expected.undefined.is_empty());
            check.is_ok() && exact
        };
        let detail = check
            .as_ref()
            .err()
            .map(String::as_str)
            .unwrap_or("")
            .replace(['\t', '\r', '\n'], " ");
        summary.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            program.id(),
            program.family,
            if program.refused {
                "refusal"
            } else {
                "admitted"
            },
            if check.is_ok() { "accepted" } else { "refused" },
            exact,
            detail
        ));
        if !matches {
            errors.push(format!(
                "{}: expected={} checker={check:?} exact={exact} source={}",
                program.id(),
                if program.refused {
                    "refusal"
                } else {
                    "exact accepted model"
                },
                program.render().trim()
            ));
        }
    }
    assert!(observed.is_empty());
}

#[test]
fn frozen_finite_oracle_matches_prototype_producer_and_checker() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let directory = std::env::var_os("SB1_PREFLIGHT_OUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("target/oracle-preflight")
                .join(std::process::id().to_string())
        });
    fs::create_dir_all(&directory).unwrap();
    let selected = std::env::var("SB1_PREFLIGHT_FAMILY").ok();
    let mut summary = String::from("case\tfamily\texpected\tchecker\texact_model\tdetail\n");
    let mut errors = Vec::new();
    let mut total = 0;
    for family in [
        "G",
        "V",
        "C4",
        "C6",
        "A-free",
        "A-bound",
        "A-partial",
        "I",
        "B-shape",
        "NC",
    ] {
        if selected.as_deref().is_some_and(|f| f != family) {
            continue;
        }
        let mut programs = Vec::new();
        let mut count = 0;
        domain::enumerate(family, |program| {
            program.assert_source_bounds();
            count += 1;
            programs.push(program);
            if programs.len() == 256 {
                batch(&programs, &directory, &root, &mut summary, &mut errors);
                programs.clear();
                fs::write(directory.join("results.tsv"), &summary).unwrap();
            }
        });
        if !programs.is_empty() {
            batch(&programs, &directory, &root, &mut summary, &mut errors);
        }
        assert_eq!(count, model::family_counts()[family]);
        total += count;
        fs::write(directory.join("results.tsv"), &summary).unwrap();
        eprintln!(
            "prototype-preflight: {family} cases={count} cumulative-disagreements={}",
            errors.len()
        );
    }
    if selected.is_none() {
        assert_eq!(total, 93_369);
    }
    fs::write(directory.join("disagreements.txt"), errors.join("\n")).unwrap();
    assert!(
        errors.is_empty(),
        "{} disagreements; first={:?}; output={}",
        errors.len(),
        errors.first(),
        directory.display()
    );
    eprintln!(
        "prototype-preflight: exact cases={total}; disagreements=0; output={}",
        directory.display()
    );
}

#[test]
fn batch_protocol_rejects_truncation_and_duplicate_case_ids() {
    assert!(parse("B\ta\nM\nC\nsb1-certificate 1\nE\ta\n").is_ok());
    for text in [
        "B\ta\nM\n",
        "B\ta\nB\tb\n",
        "B\ta\nM\nC\nE\tb\n",
        "F\ta\tfail\nF\ta\tfail\n",
    ] {
        assert!(parse(text).is_err(), "{text:?}");
    }
}
