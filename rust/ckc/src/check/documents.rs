use super::common::*;
use super::{inventories::Guideline, process, queries, text};
use std::path::{Path, PathBuf};
fn compile(swipl: &Path, stage: &Path, g: &Guideline, id: &str, proof: bool) -> Result<Vec<u8>> {
    let bytes = read(&g.ace(id), "guideline")?;
    let mut tail = vec![show(stage), id.to_owned()];
    if let Some(lexicon) = &g.lexicon {
        tail.push(show(lexicon));
    }
    if proof {
        tail.push("proof".to_owned());
    }
    let out = process::bounded(
        &mut process::compiler(swipl, stage, &tail),
        &format!("compile {id}"),
        Some(&bytes),
    )?;
    if out.rc != 0 {
        return Err(out.relay());
    }
    if !out.err.is_empty() {
        return Err(fail(
            "compiler-stderr",
            format!("non-empty stderr for document: {id}"),
        ));
    }
    if out.out.is_empty() {
        return Err(fail(
            "compiler-stdout",
            format!("empty stdout for document: {id}"),
        ));
    }
    if !out.out.ends_with(b"\n") {
        return Err(fail(
            "compiler-stdout",
            format!("missing final newline for document: {id}"),
        ));
    }
    Ok(out.out)
}
fn load(path: &Path) -> Result {
    let bytes = read(path, "check-load")?;
    let utf8 = std::str::from_utf8(&bytes).err().map(|e| e.valid_up_to());
    let at = match ckc_kernel::contract::v1_check(&bytes) {
        ckc_kernel::EV1Verdict::Ok => None,
        ckc_kernel::EV1Verdict::Reject { at } => Some(at),
    };
    if let Some(at) = utf8.into_iter().chain(at).min() {
        let prefix = &bytes[..at];
        let line = 1 + prefix.iter().filter(|b| **b == b'\n').count();
        let column = 1 + prefix.iter().rev().take_while(|b| **b != b'\n').count();
        return Err(Failure {
            rc: 2,
            out: vec![],
            err: format!("ace_to_pl_error(check_load,noncanonical({line},{column})).\n")
                .into_bytes(),
        });
    }
    Ok(())
}
fn write_manifest(path: &Path, pairs: &[(PathBuf, PathBuf)]) -> Result {
    process::write(
        path,
        pairs
            .iter()
            .map(|(pl, payload)| format!("{}\t{}\n", show(pl), show(payload)))
            .collect::<String>()
            .as_bytes(),
    )
}
fn aggregate(mode: &str, path: &Path, count: usize) -> Result<String> {
    let out = queries::kernel(
        mode,
        &[path],
        300,
        "swipl-timeout",
        &format!("{mode} exceeded 300s wall clock"),
    )?;
    if out.rc != 0 {
        return Err(out.relay());
    }
    let (category, suffix) = if mode == "aggregate-check" {
        ("aggregate", " obligations\n")
    } else {
        ("recursion", " rule clauses\n")
    };
    if !out.err.is_empty() {
        return Err(fail(
            &format!("{category}-stderr"),
            format!("non-empty stderr for manifest: {}", show(path)),
        ));
    }
    let text = String::from_utf8_lossy(&out.out).into_owned();
    if !text.starts_with(&format!("ace_to_pl {category} ok {count} documents "))
        || !text.ends_with(suffix)
    {
        return Err(violation(
            &format!("{category}-report"),
            format!("unexpected report: {}", text::strip(&text)),
        ));
    }
    Ok(text)
}
pub(super) fn check(
    scratch: &process::Scratch,
    swipl: &Path,
    stage: &Path,
    g: &Guideline,
) -> Result {
    let payload_dir = scratch.0.join("payloads");
    if !payload_dir.is_dir() {
        std::fs::create_dir(&payload_dir).map_err(|e| fail("scratch", e.to_string()))?;
    }
    let mut pairs = Vec::new();
    for id in &g.docids {
        let first = compile(swipl, stage, g, id, false)?;
        let second = compile(swipl, stage, g, id, false)?;
        if first != second {
            return Err(violation(
                "determinism",
                format!("two compiles differ for document: {id}"),
            ));
        }
        let committed = g.pl(id);
        if first != read(&committed, "stale")? {
            return Err(violation(
                "stale",
                format!(
                    "committed pl differs from fresh compile: {}",
                    show(&committed)
                ),
            ));
        }
        let first = compile(swipl, stage, g, id, true)?;
        let second = compile(swipl, stage, g, id, true)?;
        if first != second {
            return Err(violation(
                "determinism",
                format!("two proof runs differ for document: {id}"),
            ));
        }
        let payload = payload_dir.join(format!("{id}.proof"));
        process::write(&payload, &first)?;
        pairs.push((committed.clone(), payload));
        load(&committed)?;
    }
    let manifest_ids: Vec<String> = pairs
        .iter()
        .map(|(p, _)| name(p).strip_suffix(".pl").unwrap_or("").to_owned())
        .collect();
    if manifest_ids != g.docids {
        return Err(violation(
            "aggregate-totality",
            format!(
                "manifest documents differ from corpus documents: {}",
                show(&g.path)
            ),
        ));
    }
    let forward = scratch.0.join("manifest-forward");
    write_manifest(&forward, &pairs)?;
    let report = aggregate("aggregate-check", &forward, pairs.len())?;
    let reverse = scratch.0.join("manifest-reverse");
    pairs.reverse();
    write_manifest(&reverse, &pairs)?;
    if report != aggregate("aggregate-check", &reverse, pairs.len())? {
        return Err(violation(
            "aggregate-order",
            "forward and reverse manifests disagree",
        ));
    }
    println!(
        "goal: {}",
        text::strip(&aggregate("recursion-check", &forward, pairs.len())?)
    );
    let counts = queries::validate(scratch, swipl, stage, &g.path, g.lexicon.as_deref())?;
    println!("{}", counts.query_meter(&name(&g.path)));
    println!("{}", counts.trace_meter(&name(&g.path)));
    Ok(())
}
