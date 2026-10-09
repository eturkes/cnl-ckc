use super::common::*;
use super::{process, red};
use std::path::Path;
// q12 P4: a green probe compiles under the docid of its stem (red-probe sidecars)
// to exactly its `.expect` pin, which the kernel reader accepts.
pub(super) fn run(swipl: &Path, stage: &Path, probe: &Path) -> Result {
    let n = name(probe);
    let input = read(probe, "green-probe")?;
    let id = n.strip_suffix(".ace").unwrap_or(&n);
    let tail = red::tail(stage, probe, id)?;
    let out = process::bounded(
        &mut process::compiler(swipl, stage, &tail),
        &format!("green-probe {n}"),
        Some(&input),
    )?;
    if out.rc != 0 || !out.err.is_empty() {
        return Err(violation(
            "green-exit",
            format!("status {} for probe: {n}", out.rc),
        ));
    }
    if out.out != read(&probe.with_extension("expect"), "green-expect")? {
        return Err(violation(
            "green-expect",
            format!("stdout differs from expect pin for probe: {n}"),
        ));
    }
    if !matches!(
        ckc_kernel::contract::v1_check(&out.out),
        ckc_kernel::EV1Verdict::Ok
    ) {
        return Err(violation(
            "green-check",
            format!("kernel reader rejects probe output: {n}"),
        ));
    }
    Ok(())
}
