// Acceptance oracle = spec Decisions Schema evolution.
// CKC_IMP_WS_ID2_TEST_BIN selects a prebuilt ckc.
#[path = "support/imp_ws_id2.rs"]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use support::*;

const FIXTURES: &str = "tests/imp-acceptance/id2";

fn fixture(name: &str) -> String {
    fs::read_to_string(root().join(FIXTURES).join(name)).unwrap()
}
fn tracked(pattern: &str) -> Vec<PathBuf> {
    let out = Command::new("git")
        .args(["ls-files", "-z", "--", pattern])
        .current_dir(root())
        .output()
        .unwrap();
    assert!(out.status.success(), "committed inventory: {out:?}");
    out.stdout
        .split(|&b| b == 0)
        .filter(|p| !p.is_empty())
        .map(|p| root().join(std::str::from_utf8(p).unwrap()))
        .collect()
}
#[test]
fn i1_committed_documents_and_questions_carry_identifier_2() {
    let docs = tracked(":(glob)guidelines/*/pl/*.pl");
    let questions = tracked(":(glob)guidelines/*/queries/pl/*.pl");
    assert!(!docs.is_empty(), "positive document inventory");
    assert!(!questions.is_empty(), "positive question inventory");
    let mut failures = Vec::new();
    for (paths, prefix, wanted) in [
        (
            &docs,
            "guideline_schema_version(",
            "guideline_schema_version(2).",
        ),
        (&questions, "'$guideline_query'(", "'$guideline_query'(v2,"),
    ] {
        for p in paths {
            let bytes = fs::read_to_string(p).unwrap();
            let lines: Vec<_> = bytes
                .lines()
                .filter(|line| line.starts_with(prefix))
                .collect();
            let correct = match lines.as_slice() {
                [line] if prefix == "guideline_schema_version(" => *line == wanted,
                [line] => line.starts_with(wanted),
                _ => false,
            };
            if !correct {
                failures.push(format!(
                    "{}: {:?}",
                    p.strip_prefix(root()).unwrap().display(),
                    lines
                ));
            }
        }
    }
    println!(
        "IMP I1 documents={} questions={} wrong={}",
        docs.len(),
        questions.len(),
        failures.len()
    );
    assert!(
        failures.is_empty(),
        "I1 identifier 2: {} wrong\n{}",
        failures.len(),
        failures
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn fact(id: &str) -> String {
    format!(
        "guideline_entity(actual,'$guideline_id'(product,{},1,ref(1),[]),patient,countable)",
        atom(id)
    )
}
fn document(id: &str, version: u8) -> String {
    fixture("document.pl.in")
        .replace("@ID@", id)
        .replace("@DOCID@", &atom(id))
        .replace("@VERSION@", &version.to_string())
        .replace("@HASH@", &atom(&sha("A patient exists.\n")))
        .replace("@FACT@", &fact(id))
}
fn question() -> String {
    format!(
        "% id2-query compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v2,'id2-query',ace_sha256({}),ulex(none),vocabulary(none)).\n% Q1: Is there a patient?\n'$guideline_query_projection'(goal(guideline_entity(actual,A,patient,countable)),answers([])).\n",
        atom(&sha("Is there a patient?\n"))
    )
}
fn composition(s: &Scratch, name: &str, versions: [u8; 2], reverse: bool) -> PathBuf {
    let mut rows = Vec::new();
    for (id, version) in ["id2-one", "id2-two"].into_iter().zip(versions) {
        let pl = s.file(&format!("{name}/{id}.pl"), document(id, version));
        let payload = format!(
            "'$guideline_proof'({},1,variant(1),witness([]),prove([{}])).\n",
            atom(id),
            fact(id)
        );
        let proof = s.file(&format!("{name}/{id}.proof"), payload);
        rows.push(format!("{}\t{}\n", pl.display(), proof.display()));
    }
    if reverse {
        rows.reverse();
    }
    s.file(&format!("{name}.tsv"), rows.concat())
}
fn invoke_loader(s: &Scratch, mode: &str, m: &Path, q: &Path, answers: &Path, label: &str) -> Run {
    let mut args = vec!["v1", mode, path(m)];
    if matches!(mode, "answer" | "trace") {
        args.push(path(q));
    }
    if mode == "trace" {
        args.push(path(answers));
    }
    native(s, &args, label)
}
fn identifier_refusal(mode: &str, mixed: bool) {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let row = if mixed { "I2" } else { "I3" };
    let s = Scratch::new(&format!("{row}-{mode}"));
    let mut c = Checks::default();
    let mut qbytes = question();
    let mut q = s.file("question-v2.pl", &qbytes);
    if matches!(mode, "answer" | "trace") {
        let supported = native(
            &s,
            &["v1", "check", path(&q)],
            &format!("{row} v2 question control"),
        );
        c.require(
            supported.rc == 0,
            "identifier-2 question accepts",
            &supported,
        );
        // Keep checking composition admission on a pre-ID2 binary even if its
        // query reader rejects v2 first. That rejection still fails this test.
        if supported.rc != 0 {
            qbytes = substitute(qbytes, "'$guideline_query'(v2,", "'$guideline_query'(v1,");
            q = s.file("question-v1.pl", &qbytes);
        }
    }
    let control_m = composition(&s, "all-two", [2, 2], false);
    // Trace takes the answer command's own protocol bytes. The fallback lets the
    // pre-ID2 revision reach manifest admission even though it cannot read v2.
    let answers = if mode == "trace" {
        let out = native(
            &s,
            &["v1", "answer", path(&control_m), path(&q)],
            &format!("{row} trace answer control"),
        );
        c.require(out.rc == 0, "all-2 answer control", &out);
        let bytes = if out.rc == 0 {
            out.stdout
        } else {
            format!(
                "% id2-query answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,'id2-query',query_sha256({}),result(yes)).\n",
                atom(&sha(&qbytes))
            )
        };
        s.file("answers.pl", bytes)
    } else {
        s.file("answers.pl", b"")
    };
    let control = if mode == "check" {
        native(
            &s,
            &["v1", "check", path(&s.0.join("all-two/id2-one.pl"))],
            &format!("{row} {mode} all-2 control"),
        )
    } else {
        invoke_loader(
            &s,
            mode,
            &control_m,
            &q,
            &answers,
            &format!("{row} {mode} all-2 control"),
        )
    };
    c.require(control.rc == 0, "all-2 composition accepts", &control);
    for reverse in [false, true] {
        let versions = if mixed { [1, 2] } else { [1, 1] };
        let name = if reverse { "reverse" } else { "forward" };
        let m = composition(&s, name, versions, reverse);
        let got = if mode == "check" {
            let input = s.0.join(name).join("id2-one.pl");
            let bytes = fs::read(&input).unwrap();
            let marker = b"guideline_schema_version(1).";
            let digit = bytes
                .windows(marker.len())
                .position(|w| w == marker)
                .expect("identifier-1 input anchor")
                + b"guideline_schema_version(".len();
            let before = &bytes[..digit];
            let line = before.iter().filter(|&&b| b == b'\n').count() + 1;
            let column = before
                .iter()
                .rposition(|&b| b == b'\n')
                .map_or(digit + 1, |at| digit - at);
            let expected = format!("ace_to_pl_error(check_load,noncanonical({line},{column})).\n");
            let got = native(
                &s,
                &["v1", "check", path(&input)],
                &format!("{row} {mode} {name}"),
            );
            c.require(
                got.rc == 2 && got.stdout.is_empty() && got.stderr == expected,
                &format!("{row} {mode} {name}: refuse at identifier-1 digit ({line},{column})"),
                &got,
            );
            continue;
        } else {
            invoke_loader(&s, mode, &m, &q, &answers, &format!("{row} {mode} {name}"))
        };
        let named = got.names("id2-one") || (!mixed && got.names("id2-two"));
        c.require(
            got.refused() && named,
            &format!("{row} {mode} {name}: refuse + name identifier-1 document"),
            &got,
        );
    }
    c.finish(&format!("{row} {mode}"));
}

#[test]
fn i2_check() {
    identifier_refusal("check", true);
}
#[test]
fn i2_aggregate_check() {
    identifier_refusal("aggregate-check", true);
}
#[test]
fn i2_recursion_check() {
    identifier_refusal("recursion-check", true);
}
#[test]
fn i2_answer() {
    identifier_refusal("answer", true);
}
#[test]
fn i2_trace() {
    identifier_refusal("trace", true);
}
#[test]
fn i3_check() {
    identifier_refusal("check", false);
}
#[test]
fn i3_aggregate_check() {
    identifier_refusal("aggregate-check", false);
}
#[test]
fn i3_recursion_check() {
    identifier_refusal("recursion-check", false);
}
#[test]
fn i3_answer() {
    identifier_refusal("answer", false);
}
#[test]
fn i3_trace() {
    identifier_refusal("trace", false);
}
