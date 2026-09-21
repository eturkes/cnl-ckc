use super::common::*;
use super::{inventories::RedProbe, process};
use std::path::Path;
pub(super) fn run(swipl: &Path, stage: &Path, probe: &RedProbe) -> Result {
    let n = name(&probe.path);
    let input = read(&probe.path, "red-probe")?;
    let lexicon = probe.path.with_extension("ulex");
    let mut tail = vec![show(stage), "red-probe".to_owned()];
    if lexicon.is_file() {
        tail.push(show(&lexicon));
    }
    let out = process::bounded(
        &mut process::compiler(swipl, stage, &tail),
        &format!("red-probe {n}"),
        Some(&input),
    )?;
    if out.rc != probe.rc {
        return Err(violation(
            "red-exit",
            format!("status {} for probe: {n}", out.rc),
        ));
    }
    if !out.out.is_empty() {
        return Err(violation(
            "red-stdout",
            format!("non-empty stdout for probe: {n}"),
        ));
    }
    if out.err.iter().filter(|b| **b == b'\n').count() != 1 || !out.err.ends_with(b"\n") {
        return Err(violation(
            "red-stderr",
            format!("stderr is not one LF line for probe: {n}"),
        ));
    }
    let pin = probe.path.with_extension("expect");
    if !pin.is_file() {
        return Err(violation(
            "red-expect",
            format!("missing expect pin for probe: {n}"),
        ));
    }
    if out.err != read(&pin, "red-expect")? {
        return Err(violation(
            "red-expect",
            format!("stderr differs from expect pin for probe: {n}"),
        ));
    }
    let stderr = String::from_utf8_lossy(&out.err);
    if !stderr.starts_with(&format!("ace_to_pl_error({},", probe.class)) {
        return Err(violation(
            "red-class",
            format!("stderr class mismatch for probe: {n}"),
        ));
    }
    if !stderr.ends_with(").\n") {
        return Err(violation(
            "red-stderr",
            format!("stderr lacks canonical term suffix for probe: {n}"),
        ));
    }
    Ok(())
}
