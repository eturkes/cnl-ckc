use super::common::*;
use super::process;
use std::process::Command;
use std::time::{Duration, Instant};
pub(super) fn docid() -> Result {
    let long = "a".repeat(250);
    if !valid_docid(&long) {
        return Err(violation("docid-grammar", "250-byte docid rejected"));
    }
    if valid_docid(&(long + "a")) {
        return Err(violation("docid-grammar", "251-byte docid accepted"));
    }
    Ok(())
}
pub(super) fn trace_numeric() -> Result {
    Err(fail("queries-fixtures", "trace seam pending"))
}
pub(super) fn wall() -> Result {
    let start = Instant::now();
    for (cmd, detail) in [
        ("sleep 600", "sleeper exited under the wall clock"),
        (
            "sleep 600 & exec sleep 600",
            "descendant-held-pipe sleeper exited under the wall clock",
        ),
    ] {
        let (timed, _) = process::walled(
            Command::new("/bin/sh").args(["-c", cmd]),
            None,
            Duration::from_secs(1),
        )?;
        if !timed {
            return Err(violation("swipl-timeout-probe", detail));
        }
    }
    if start.elapsed() > Duration::from_secs(30) {
        return Err(violation(
            "swipl-timeout-probe",
            "wall-clock probes overran 30s: descendant pipes survive the kill",
        ));
    }
    Ok(())
}
