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
    let hex = "0".repeat(64);
    let bigint = "9".repeat(5000);
    let artifact = |number: &str, ordinal: &str| {
        format!(
            "% probe traced against the loaded composition by ace_to_pl trace mode; do not edit.\n'$guideline_traces'(v1,probe,query_sha256('{hex}'),answers_sha256('{hex}'),result(solutions([sol([{number}],proved([clause(sentence(doc,{ordinal}),clause_sha256('{hex}'),[])]))]))).\n"
        )
    };
    let accepted = matches!(
        ckc_kernel::contract::v1_check(artifact(&bigint, "1").as_bytes()),
        ckc_kernel::EV1Verdict::Ok
    );
    if !accepted {
        return Err(violation(
            "trace-numeric",
            "5000-digit integer payload rejected by trace parser",
        ));
    }
    Err(fail("trace-numeric", "K3 coordinate binding pending"))
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
