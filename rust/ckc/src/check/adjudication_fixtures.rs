use super::adjudication;
use super::common::*;
use std::path::Path;
pub(super) fn check() -> Result {
    let root = Path::new("tests/adjudication");
    if root.is_symlink() {
        return Err(violation(
            "adjudication-fixtures",
            "is a symlink: tests/adjudication",
        ));
    }
    if !root.is_dir() {
        return Err(violation(
            "adjudication-fixtures",
            "missing: tests/adjudication",
        ));
    }
    let cases = entries(root, "adjudication-fixtures")?;
    for p in &cases {
        let n = name(p);
        if p.is_symlink() || !p.is_dir() {
            return Err(violation(
                "adjudication-fixtures",
                format!("not a case directory: {n}"),
            ));
        }
        if !valid_docid(&n) {
            return Err(violation(
                "adjudication-fixtures",
                format!("invalid case name: {n}"),
            ));
        }
    }
    if cases.is_empty() {
        return Err(violation(
            "adjudication-fixtures",
            "no fixture cases found: tests/adjudication",
        ));
    }
    let (mut red, mut green) = (0, 0);
    for case in cases {
        let n = name(&case);
        let mut names = Vec::new();
        for p in entries(&case, "adjudication-fixtures")? {
            let m = name(&p);
            if !["expect", "golden", "ledger.tsv", "manifest.tsv"].contains(&m.as_str()) {
                return Err(violation(
                    "adjudication-fixtures",
                    format!("unsupported entry: {n}/{m}"),
                ));
            }
            if p.is_symlink() || !p.is_file() {
                return Err(violation(
                    "adjudication-fixtures",
                    format!("not a regular file: {n}/{m}"),
                ));
            }
            names.push(m);
        }
        let has = |s: &str| names.iter().any(|n| n == s);
        if !has("manifest.tsv") {
            return Err(violation(
                "adjudication-fixtures",
                format!("case without manifest.tsv: {n}"),
            ));
        }
        if has("expect") && has("golden") {
            return Err(violation(
                "adjudication-fixtures",
                format!("case pins both expect and golden: {n}"),
            ));
        }
        if !has("expect") && !has("golden") {
            return Err(violation(
                "adjudication-fixtures",
                format!("case without expect or golden pin: {n}"),
            ));
        }
        let (rc, out, err) = match adjudication::validate(
            &case.join("ledger.tsv"),
            &case.join("manifest.tsv"),
            "fx",
        ) {
            Ok(out) => (0, out, vec![]),
            Err(e) => (e.rc, e.out, e.err),
        };
        let expected_rc = u8::from(has("expect"));
        if rc != expected_rc {
            return Err(violation(
                "adjudication-fixtures",
                format!("status {rc} for case: {n}"),
            ));
        }
        if !err.is_empty() {
            return Err(violation(
                "adjudication-fixtures",
                format!("non-empty stderr for case: {n}"),
            ));
        }
        let pin = if has("expect") { "expect" } else { "golden" };
        if out != read(&case.join(pin), "adjudication-fixtures")? {
            let label = if pin == "expect" {
                "expect pin"
            } else {
                "golden"
            };
            return Err(violation(
                "adjudication-fixtures",
                format!("stdout differs from {label} for case: {n}"),
            ));
        }
        if has("expect") {
            red += 1;
        } else {
            green += 1;
        }
    }
    if red != 42 {
        return Err(violation(
            "adjudication-fixtures",
            format!("red case count drift: expected 42 got {red}"),
        ));
    }
    if green != 9 {
        return Err(violation(
            "adjudication-fixtures",
            format!("green case count drift: expected 9 got {green}"),
        ));
    }
    println!("goal: adjudication fixtures ok {red} red {green} green");
    Ok(())
}
