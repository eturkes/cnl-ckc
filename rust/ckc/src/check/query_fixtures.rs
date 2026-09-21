use super::common::*;
use super::{process, queries};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const RED_REQUIRED: &[&str] = &[
    "bad-qid",
    "empty-solutions",
    "limit-depth",
    "limit-inner-inference",
    "malformed-query-file",
    "missing-trace",
    "no-finite-failure",
    "orphan-answers",
    "orphan-pl",
    "orphan-trace",
    "stale-answers",
    "stale-pl",
    "stale-trace",
    "trace-digest-join",
    "trace-indeterminate-mirror",
    "trace-naf-depth-cut",
    "trace-naf-inference-cut",
    "trace-naf-proved",
    "trace-no-mirror",
    "trace-row-failure-after-cut",
    "trace-unproved-finite",
    "trace-unproved-limit",
    "uncompiled-ace",
    "yesno-limit-before-proof",
];
const GREEN_REQUIRED: &[&str] = &[
    "absent-queries",
    "canonical-sort",
    "depth-backtrack",
    "empty-queries",
    "numeric-bigint",
    "positive",
    "trace-direct-rejects",
    "trace-multi-proof",
    "trace-naf",
    "trace-positive-rule",
    "trace-serializer",
];
const PIN_EXPECTED: &[&str] = &[
    "green/canonical-sort/answers-golden/q-sort",
    "green/depth-backtrack/answers-golden/q-back",
    "green/numeric-bigint/answers-golden/q-big",
    "green/numeric-bigint/traces-golden/q-big",
    "green/positive/answers-golden/q-wh",
    "green/positive/answers-golden/q-yes",
    "green/trace-direct-rejects/trace-reject/qid-mismatch",
    "green/trace-direct-rejects/trace-reject/query-sha256-mismatch",
    "green/trace-direct-rejects/trace-reject/result-mode-mismatch",
    "green/trace-direct-rejects/trace-reject/result-shape",
    "green/trace-direct-rejects/traces-golden/qid-mismatch",
    "green/trace-direct-rejects/traces-golden/query-sha256-mismatch",
    "green/trace-direct-rejects/traces-golden/result-mode-mismatch",
    "green/trace-direct-rejects/traces-golden/result-shape",
    "green/trace-multi-proof/traces-golden/q-multi",
    "green/trace-naf/traces-golden/q-naf",
    "green/trace-positive-rule/traces-golden/q-rule",
    "green/trace-serializer/traces-golden/q-serializer",
    "red/empty-solutions/answers-golden/q-empty",
    "red/empty-solutions/traces-golden/q-empty",
    "red/limit-depth/answers-golden/q-depth",
    "red/limit-depth/traces-golden/q-depth",
    "red/limit-inner-inference/answers-golden/q-inner",
    "red/limit-inner-inference/traces-golden/q-inner",
    "red/missing-trace/traces-golden/q-missing-trace",
    "red/no-finite-failure/answers-golden/q-no",
    "red/no-finite-failure/traces-golden/q-no",
    "red/orphan-trace/traces-golden/q-orphan",
    "red/stale-trace/traces-golden/q-stale-trace",
    "red/trace-digest-join/traces-golden/q-digest",
    "red/trace-indeterminate-mirror/traces-golden/q-indeterminate",
    "red/trace-naf-depth-cut/traces-golden/q-naf-depth",
    "red/trace-naf-inference-cut/traces-golden/q-naf-inference",
    "red/trace-naf-proved/traces-golden/q-naf-proved",
    "red/trace-no-mirror/traces-golden/q-no-trace",
    "red/trace-row-failure-after-cut/traces-golden/q-row-cut",
    "red/trace-unproved-finite/traces-golden/q-unproved-finite",
    "red/trace-unproved-limit/traces-golden/q-unproved-limit",
    "red/yesno-limit-before-proof/answers-golden/q-yn-limit",
    "red/yesno-limit-before-proof/traces-golden/q-yn-limit",
];
fn bad(case: &str, name: &str) -> Failure {
    violation(
        "queries-fixtures",
        format!("unsupported entry: {case}/{name}"),
    )
}
const R79_CASES: &[&str] = &[
    "empty-solutions",
    "limit-depth",
    "limit-inner-inference",
    "no-finite-failure",
    "numeric-bigint",
    "trace-digest-join",
    "trace-indeterminate-mirror",
    "trace-naf-inference-cut",
    "trace-no-mirror",
    "yesno-limit-before-proof",
];
const R90_STALE_ANSWERS: &[(&str, &str)] = &[
    ("trace-naf-depth-cut", "q-naf-depth"),
    ("trace-naf-proved", "q-naf-proved"),
    ("trace-unproved-finite", "q-unproved-finite"),
    ("trace-unproved-limit", "q-unproved-limit"),
];
fn non_v1_expectations(root: &Path) -> Result<BTreeMap<String, (u8, Vec<u8>)>> {
    let path = root.join("r79-nonv1.tsv");
    let source = corpus_text(&path, "queries-fixtures")?;
    let bad_table = || violation("queries-fixtures", "invalid R79 expectation table");
    if source.contains('\r') || !source.ends_with('\n') {
        return Err(bad_table());
    }
    let mut lines = source.split_terminator('\n');
    if lines.next() != Some("case\trc\texpected_stderr") {
        return Err(bad_table());
    }
    let mut rows = BTreeMap::new();
    for line in lines {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 3 || !R79_CASES.contains(&fields[0]) || fields[1] != "2" {
            return Err(bad_table());
        }
        let stderr = fields[2].replace("\\n", "\n");
        if !stderr.starts_with("ace_to_pl_error(check_load,noncanonical('")
            || !stderr.ends_with("')).\n")
            || stderr[..stderr.len() - 1].contains('\n')
            || rows
                .insert(fields[0].to_owned(), (2, stderr.into_bytes()))
                .is_some()
        {
            return Err(bad_table());
        }
    }
    if rows.len() != R79_CASES.len() || R79_CASES.iter().any(|name| !rows.contains_key(*name)) {
        return Err(bad_table());
    }
    Ok(rows)
}
fn case_result(
    case: &Path,
    gid: &Path,
    is_red: bool,
    non_v1: Option<&(u8, Vec<u8>)>,
    scratch: &process::Scratch,
    swipl: &Path,
    stage: &Path,
) -> Result {
    let n = name(case);
    let pin = if is_red { "expect" } else { "golden" };
    let (rc, stdout, stderr) = match queries::fixture(scratch, swipl, stage, gid) {
        Ok(bytes) => (0, bytes, Vec::new()),
        Err(e) => (e.rc, e.out, e.err),
    };
    if let Some((expected_rc, expected_stderr)) = non_v1 {
        if rc != *expected_rc || !stdout.is_empty() || stderr != *expected_stderr {
            return Err(violation(
                "queries-fixtures",
                format!("R79 rejection differs from pin for case: {n}"),
            ));
        }
        // R80 substitutes the native rejection; the original pin remains a required readable member.
        read(&case.join(pin), "queries-fixtures")?;
        return Ok(());
    }
    if rc != u8::from(is_red) {
        return Err(violation(
            "queries-fixtures",
            format!("status {rc} for case: {n}"),
        ));
    }
    if !stderr.is_empty() {
        return Err(violation(
            "queries-fixtures",
            format!("non-empty stderr for case: {n}"),
        ));
    }
    let mut expected = read(&case.join(pin), "queries-fixtures")?;
    // R90 re-pins these verdicts only; original files and trace goldens remain required.
    if is_red && let Some((_, id)) = R90_STALE_ANSWERS.iter().find(|(case, _)| *case == n) {
        expected = format!(
            "goal: stale: committed query answers differ from fresh answer: {}\n",
            show(&gid.join("queries/answers").join(format!("{id}.pl"))),
        )
        .into_bytes();
    }
    if stdout != expected {
        let p = if is_red { "expect pin" } else { "golden" };
        return Err(violation(
            "queries-fixtures",
            format!("stdout differs from {p} for case: {n}"),
        ));
    }
    Ok(())
}
fn golden(
    case: &Path,
    color: &str,
    lane: &str,
    gid: &Path,
    manifest: &Path,
    inventory: &mut Vec<String>,
    non_v1: bool,
) -> Result {
    let n = name(case);
    for p in entries(&case.join(lane), "queries-fixtures")? {
        let pn = name(&p);
        let id = pn.strip_suffix(".pl").filter(|s| valid_docid(s));
        if p.is_symlink() || !p.is_file() || id.is_none() {
            return Err(bad(&n, &format!("{lane}/{pn}")));
        }
        let id = id.unwrap_or("");
        inventory.push(format!("{color}/{n}/{lane}/{id}"));
        let root = gid.join("queries");
        let ace = root.join(format!("{id}.ace"));
        let pl = root.join("pl").join(format!("{id}.pl"));
        let answers = root.join("answers").join(format!("{id}.pl"));
        if !ace.is_file() || !pl.is_file() || (lane == "traces-golden" && !answers.is_file()) {
            return Err(violation(
                "queries-fixtures",
                format!("{lane} qid has no query: {n}/{id}"),
            ));
        }
        let expected = read(&p, "queries-fixtures")?;
        if non_v1 {
            continue;
        }
        let bytes = if lane == "answers-golden" {
            queries::answer(id, manifest, &pl)?
        } else {
            queries::trace(id, manifest, &pl, &answers)?
        };
        if bytes != expected {
            let kind = if lane == "answers-golden" {
                "answer"
            } else {
                "trace"
            };
            return Err(violation(
                "queries-fixtures",
                format!("{kind} bytes differ from {lane}: {n}/{id}"),
            ));
        }
    }
    Ok(())
}
fn rejects(
    case: &Path,
    color: &str,
    gid: &Path,
    manifest: &Path,
    inventory: &mut Vec<String>,
) -> Result {
    let n = name(case);
    let root = case.join("trace-reject");
    let (mut answers, mut expects) = (vec![], vec![]);
    for p in entries(&root, "queries-fixtures")? {
        let pn = name(&p);
        let pair = pn
            .strip_suffix(".answers")
            .map(|s| (s, true))
            .or_else(|| pn.strip_suffix(".expect").map(|s| (s, false)));
        if p.is_symlink() || !p.is_file() || pair.is_none_or(|(id, _)| !valid_docid(id)) {
            return Err(bad(&n, &format!("trace-reject/{pn}")));
        }
        let (id, is_answer) = pair.unwrap_or(("", false));
        if is_answer {
            answers.push(id.to_owned());
        } else {
            expects.push(id.to_owned());
        }
    }
    for (ids, other, suffix) in [
        (&answers, &expects, "answers"),
        (&expects, &answers, "expect"),
    ] {
        for id in ids {
            if !other.contains(id) {
                return Err(violation(
                    "queries-fixtures",
                    format!("trace-reject member without partner: {n}/{id}.{suffix}"),
                ));
            }
        }
    }
    for id in answers {
        inventory.push(format!("{color}/{n}/trace-reject/{id}"));
        let query = gid.join("queries/pl").join(format!("{id}.pl"));
        if !query.is_file() || !gid.join("queries").join(format!("{id}.ace")).is_file() {
            return Err(violation(
                "queries-fixtures",
                format!("trace-reject qid has no query: {n}/{id}"),
            ));
        }
        let out = queries::trace_raw(
            manifest,
            &query,
            &root.join(format!("{id}.answers")),
            "queries-fixtures",
            &format!("wall_clock for trace-reject: {n}/{id}"),
        )?;
        if out.rc != 2 {
            return Err(violation(
                "queries-fixtures",
                format!("trace-reject status {} for case: {n}/{id}", out.rc),
            ));
        }
        if !out.out.is_empty() {
            return Err(violation(
                "queries-fixtures",
                format!("trace-reject stdout not empty for case: {n}/{id}"),
            ));
        }
        if out.err != read(&root.join(format!("{id}.expect")), "queries-fixtures")? {
            return Err(violation(
                "queries-fixtures",
                format!("trace-reject stderr differs from expect: {n}/{id}"),
            ));
        }
    }
    Ok(())
}
fn nonfinite(scratch: &process::Scratch) -> Result {
    let gid = Path::new("tests/queries/green/trace-direct-rejects/tree/guidelines/fx");
    let manifest = scratch.0.join("nonfinite-manifest");
    queries::manifest(&manifest, &gid.join("pl"))?;
    let template = text(
        Path::new("tests/queries/green/trace-direct-rejects/trace-reject/result-shape.answers"),
        "trace-nonfinite",
    )?
    .replace("\r\n", "\n")
    .replace('\r', "\n");
    let candidate = template.replace("result(bogus)", "result(solutions([sol([1.0Inf])]))");
    if candidate == template {
        return Err(violation(
            "trace-nonfinite",
            "probe template lost its result(bogus) anchor",
        ));
    }
    let answers = scratch.0.join("nonfinite-answers.pl");
    process::write(&answers, candidate.as_bytes())?;
    let out = queries::trace_raw(
        &manifest,
        &gid.join("queries/pl/result-shape.pl"),
        &answers,
        "trace-nonfinite",
        "wall_clock for non-finite float probe",
    )?;
    if out.rc != 2 {
        return Err(violation(
            "trace-nonfinite",
            format!("status {} for non-finite float probe", out.rc),
        ));
    }
    if !out.out.is_empty() {
        return Err(violation(
            "trace-nonfinite",
            "non-empty stdout for non-finite float probe",
        ));
    }
    if out.err != b"ace_to_pl_error(check_load,answers_file(noncanonical)).\n" {
        return Err(violation(
            "trace-nonfinite",
            "stderr differs from pinned answers_file(noncanonical) line",
        ));
    }
    Ok(())
}
pub(super) fn check(scratch: &process::Scratch, swipl: &Path, stage: &Path) -> Result {
    let root = Path::new("tests/queries");
    if root.is_symlink() {
        return Err(violation("queries-fixtures", "is a symlink: tests/queries"));
    }
    if !root.is_dir() {
        return Err(violation("queries-fixtures", "missing: tests/queries"));
    }
    for p in entries(root, "queries-fixtures")? {
        let n = name(&p);
        if !["red", "green", "r79-nonv1.tsv"].contains(&n.as_str()) {
            return Err(violation(
                "queries-fixtures",
                format!("unsupported entry: {n}"),
            ));
        }
    }
    let non_v1 = non_v1_expectations(root)?;
    let (mut red, mut green, mut pins) = (vec![], vec![], vec![]);
    for color in ["red", "green"] {
        let path = root.join(color);
        if path.is_symlink() {
            return Err(violation(
                "queries-fixtures",
                format!("is a symlink: {}", show(&path)),
            ));
        }
        if !path.is_dir() {
            return Err(violation(
                "queries-fixtures",
                format!("missing: {}", show(&path)),
            ));
        }
        for case in entries(&path, "queries-fixtures")? {
            let n = name(&case);
            if case.is_symlink() || !case.is_dir() {
                return Err(violation(
                    "queries-fixtures",
                    format!("not a case directory: {n}"),
                ));
            }
            if !valid_docid(&n) {
                return Err(violation(
                    "queries-fixtures",
                    format!("invalid case name: {n}"),
                ));
            }
            let mut members = BTreeSet::new();
            for p in entries(&case, "queries-fixtures")? {
                let m = name(&p);
                if p.is_symlink() {
                    return Err(bad(&n, &m));
                }
                match m.as_str() {
                    "tree" | "answers-golden" | "traces-golden" | "trace-reject" if p.is_dir() => {}
                    "expect" | "golden" if p.is_file() => (),
                    _ => return Err(bad(&n, &m)),
                }
                members.insert(m);
            }
            if !members.contains("tree") {
                return Err(violation(
                    "queries-fixtures",
                    format!("case without tree: {n}"),
                ));
            }
            let expect = members.contains("expect");
            let golden_pin = members.contains("golden");
            if expect && golden_pin {
                return Err(violation(
                    "queries-fixtures",
                    format!("case pins both expect and golden: {n}"),
                ));
            }
            if !expect && !golden_pin {
                return Err(violation(
                    "queries-fixtures",
                    format!("case without expect or golden pin: {n}"),
                ));
            }
            if color == "red" && golden_pin {
                return Err(violation(
                    "queries-fixtures",
                    format!("red case pins golden: {n}"),
                ));
            }
            if color == "green" && expect {
                return Err(violation(
                    "queries-fixtures",
                    format!("green case pins expect: {n}"),
                ));
            }
            let gids = case.join("tree/guidelines");
            if !gids.is_dir() {
                return Err(violation(
                    "queries-fixtures",
                    format!("case tree without guidelines: {n}"),
                ));
            }
            let gids = entries(&gids, "queries-fixtures")?;
            if gids.len() != 1 || !gids[0].is_dir() || !valid_docid(&name(&gids[0])) {
                return Err(violation(
                    "queries-fixtures",
                    format!("case tree without one guideline: {n}"),
                ));
            }
            let gid = &gids[0];
            let native_expectation = non_v1.get(&n);
            case_result(
                &case,
                gid,
                expect,
                native_expectation,
                scratch,
                swipl,
                stage,
            )?;
            if color == "red" {
                red.push(n.clone());
            } else {
                green.push(n.clone());
            }
            let manifest = scratch
                .0
                .join(format!("queries-fixture-manifest-{color}-{n}"));
            if ["answers-golden", "traces-golden", "trace-reject"]
                .iter()
                .any(|s| members.contains(*s))
            {
                queries::manifest(&manifest, &gid.join("pl"))?;
            }
            for lane in ["answers-golden", "traces-golden"] {
                if members.contains(lane) {
                    golden(
                        &case,
                        color,
                        lane,
                        gid,
                        &manifest,
                        &mut pins,
                        native_expectation.is_some(),
                    )?;
                }
            }
            if members.contains("trace-reject") {
                rejects(&case, color, gid, &manifest, &mut pins)?;
            }
        }
    }
    for (required, present, color) in [
        (RED_REQUIRED, &red, "red"),
        (GREEN_REQUIRED, &green, "green"),
    ] {
        for n in required {
            if !present.iter().any(|p| p == n) {
                return Err(violation(
                    "queries-fixtures",
                    format!("missing required case: {color}/{n}"),
                ));
            }
        }
    }
    if red.is_empty() {
        return Err(violation(
            "queries-fixtures",
            "no red fixture cases found: tests/queries",
        ));
    }
    if green.is_empty() {
        return Err(violation(
            "queries-fixtures",
            "no green fixture cases found: tests/queries",
        ));
    }
    pins.sort();
    if pins != PIN_EXPECTED {
        return Err(violation(
            "queries-fixtures",
            format!(
                "golden-pin inventory drifted from pinned 40-entry map: {} pins on disk",
                pins.len()
            ),
        ));
    }
    if red.len() != 24 {
        return Err(violation(
            "queries-fixtures",
            format!("red case count drift: expected 24 got {}", red.len()),
        ));
    }
    if green.len() != 11 {
        return Err(violation(
            "queries-fixtures",
            format!("green case count drift: expected 11 got {}", green.len()),
        ));
    }
    nonfinite(scratch)?;
    println!(
        "goal: queries fixtures ok {} red {} green",
        red.len(),
        green.len()
    );
    Ok(())
}
