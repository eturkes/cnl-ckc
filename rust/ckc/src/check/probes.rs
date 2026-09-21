use super::common::*;
use super::{process, queries};
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
    let scratch = process::Scratch::new()?;
    let doc = scratch.0.join("doc.pl");
    let manifest = scratch.0.join("numeric-manifest");
    let query = scratch.0.join("query.pl");
    let answers = scratch.0.join("answers.pl");
    let trace = scratch.0.join("traces.pl");
    process::write(
        &manifest,
        format!("{}\t{}\n", show(&doc), show(&doc)).as_bytes(),
    )?;
    process::write(&query, format!(
        "% probe compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,probe,ace_sha256('{hex}'),ulex(none)).\n% Q1: Is there a patient?\n'$guideline_query_projection'(goal(guideline_entity(actual,A,patient,countable)),answers([])).\n"
    ).as_bytes())?;
    let mut prefix =
        "% doc.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n"
            .to_owned();
    for predicate in [
        "guideline_schema_version/1",
        "guideline_document/3",
        "guideline_entity/4",
        "guideline_cardinality/5",
        "guideline_event/3",
        "guideline_arg/4",
        "guideline_pp/4",
        "guideline_property/4",
        "guideline_operator/3",
    ] {
        prefix += &format!(":- multifile({predicate}).\n:- discontiguous({predicate}).\n");
    }
    prefix += &format!(
        "guideline_schema_version(1).\nguideline_document(doc,ace_sha256('{hex}'),ulex(none)).\n% S1: There is a patient.\n"
    );
    let check = || {
        queries::kernel(
            "trace-check",
            &[&manifest, &query, &answers, &trace],
            30,
            "trace-numeric",
            "wall_clock for numeric probe",
        )
    };
    for ordinal in ["1", "9999999999"] {
        // A fresh derivation reaches coord_of; editing a wide trace alone only tests stale.
        process::write(&doc, format!(
            "{prefix}guideline_entity(actual,'$guideline_id'(product,doc,{ordinal},ref(1),[]),patient,countable).\n"
        ).as_bytes())?;
        process::write(&answers, &queries::answer("probe", &manifest, &query)?)?;
        let fresh = queries::trace("probe", &manifest, &query, &answers)?;
        process::write(&trace, &fresh)?;
        let out = check()?;
        if ordinal == "9999999999" {
            if out.rc != 1
                || !out.out.is_empty()
                || out.err != b"ace_to_pl_error(proof,trace_check(node_shape)).\n"
            {
                return Err(violation(
                    "trace-numeric",
                    "10-digit sentence ordinal accepted by trace parser",
                ));
            }
            continue;
        }
        if out.rc != 0 || !out.err.is_empty() || out.out != b"ckc: trace-check ok probe nodes=1\n" {
            return Err(violation(
                "trace-numeric",
                "canonical sentence ordinal rejected by trace parser",
            ));
        }
        let fresh = String::from_utf8(fresh)
            .map_err(|_| fail("trace-numeric", "invalid UTF-8 trace control"))?;
        let anchor = "sentence(doc,1)";
        if fresh.matches(anchor).count() != 1 {
            return Err(violation(
                "trace-numeric",
                "probe template lost its sentence(doc,1) anchor",
            ));
        }
        process::write(&trace, fresh.replace(anchor, "sentence(doc,01)").as_bytes())?;
        let out = check()?;
        if out.rc != 2
            || !out.out.is_empty()
            || out.err != b"ace_to_pl_error(check_load,trace_file(noncanonical)).\n"
        {
            return Err(violation(
                "trace-numeric",
                "zero-padded sentence ordinal accepted by trace parser",
            ));
        }
    }
    Ok(())
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
