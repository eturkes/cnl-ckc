use super::common::*;
use super::{inventories::RedProbe, process};
use std::path::Path;
// m7t D2/D9 + q12 D1: a `<probe>.temporal.tsv` sidecar selects its header's schema
// (a `<probe>.argv` sidecar names the selector outright); a `<stem>--query-…` probe
// compiles in question mode.
pub(super) fn tail(stage: &Path, probe: &Path, id: &str) -> Result<Vec<String>> {
    let table = probe.with_extension("temporal.tsv");
    let forced = probe.with_extension("argv");
    let lexicon = probe.with_extension("ulex");
    let mut tail = if name(probe).contains("--query-") {
        vec!["question".to_owned()]
    } else {
        vec![]
    };
    if table.is_file() {
        let token = if forced.is_file() {
            text(&forced, "red-probe")?.trim().to_owned()
        } else {
            selector(&table)
        };
        tail.extend([token, show(stage), id.to_owned(), show(&table)]);
    } else {
        tail.extend([show(stage), id.to_owned()]);
    }
    if lexicon.is_file() {
        tail.push(show(&lexicon));
    }
    Ok(tail)
}
pub(super) fn run(swipl: &Path, stage: &Path, probe: &RedProbe) -> Result {
    let n = name(&probe.path);
    let input = read(&probe.path, "red-probe")?;
    let tail = tail(stage, &probe.path, "red-probe")?;
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
