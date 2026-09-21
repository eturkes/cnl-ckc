use super::common::*;
use super::{documents, inventories, pipeline, process, queries};
use std::fs;
use std::path::Path;
use std::process::Command;

fn certify(
    swipl: &Path,
    stage: &Path,
    id: &str,
    ace: &Path,
    lexicon: Option<&Path>,
    pl: &[u8],
    query: bool,
) -> Result<Vec<u8>> {
    let input = read(ace, "certify")?;
    let ulex = lexicon.map(|p| read(p, "certify")).transpose()?;
    let mut command = Command::new(swipl);
    command
        .args([
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            "rust/ckc/prolog/drs_dump.pl",
            "-g",
            "main",
            "-t",
            "halt(9)",
            "--",
        ])
        .arg(stage);
    if let Some(lexicon) = lexicon {
        command.arg(lexicon);
    }
    let dump = process::bounded(&mut command, &format!("certify {id}"), Some(&input))?;
    if dump.rc != 0 {
        return Err(dump.relay());
    }
    let asha = crate::trust::sha256_hex(&input);
    let usha = ulex
        .as_ref()
        .map(|u| crate::trust::sha256_hex(u).into_bytes());
    let accepted = if query {
        ckc_kernel::contract::certify_query(
            &input,
            asha.as_bytes(),
            usha.as_ref(),
            id.as_bytes(),
            &dump.out,
            pl,
        )
    } else {
        ckc_kernel::contract::certify_doc(
            &input,
            asha.as_bytes(),
            usha.as_ref(),
            id.as_bytes(),
            &dump.out,
            pl,
        )
    };
    if accepted.rc != 0 {
        return Err(Failure {
            rc: accepted.rc,
            out: accepted.out,
            err: accepted.err,
        });
    }
    Ok(accepted.out)
}

fn mkdir(path: &Path) -> Result {
    fs::create_dir(path).map_err(|e| fail("scratch", e.to_string()))
}

// Both names live on the repository filesystem. Roll back the old name if the
// replacement rename fails; no guideline bytes move until all checks pass.
fn replace(new: &Path, target: &Path, backup: &Path) -> Result {
    let old = target.exists();
    if old {
        fs::rename(target, backup).map_err(|e| fail("scratch", e.to_string()))?;
    }
    if let Err(e) = fs::rename(new, target) {
        if old {
            fs::rename(backup, target)
                .map_err(|restore| fail("scratch", format!("{e}; restore: {restore}")))?;
        }
        return Err(fail("scratch", e.to_string()));
    }
    Ok(())
}

pub(super) fn compile(gid: &str) -> Result {
    let path = pipeline::guideline_path(gid, false)?;
    let g = inventories::collect(&path)?;
    let pl = path.join("pl");
    if pl.is_symlink() {
        return Err(fail("pl-dir", format!("is a symlink: {}", show(&pl))));
    }
    if pl.exists() && !pl.is_dir() {
        return Err(fail("pl-dir", format!("not a directory: {}", show(&pl))));
    }
    let swipl = process::swipl()?;
    let scratch = process::Scratch::new()?;
    let stage = process::stage(&scratch, &swipl)?;
    let new = scratch.0.join("pl-new");
    mkdir(&new)?;
    let mut pairs = Vec::new();
    for id in &g.docids {
        let bytes = documents::compile(&swipl, &stage, &g, id, false)?;
        let target = new.join(format!("{id}.pl"));
        process::write(&target, &bytes)?;
        documents::load(&target)?;
        let payload = certify(
            &swipl,
            &stage,
            id,
            &g.ace(id),
            g.lexicon.as_deref(),
            &bytes,
            false,
        )?;
        let proof = scratch.0.join(format!("{id}.proof"));
        process::write(&proof, &payload)?;
        pairs.push((target, proof));
    }
    let manifest = scratch.0.join("compile-manifest");
    documents::write_manifest(&manifest, &pairs)?;
    documents::aggregate("aggregate-check", &manifest, pairs.len())?;
    documents::aggregate("recursion-check", &manifest, pairs.len())?;
    replace(&new, &pl, &scratch.0.join("pl-old"))?;
    for id in &g.docids {
        println!("goal: wrote {}", show(&g.pl(id)));
    }
    println!("goal: compile ok {} documents", g.docids.len());
    Ok(())
}

pub(super) fn queries(gid: &str) -> Result {
    let path = pipeline::guideline_path(gid, false)?;
    let root = path.join("queries");
    let ids = queries::query_aces(&root)?;
    if ids.is_empty() {
        for name in ["pl", "answers", "traces"] {
            let dir = root.join(name);
            if dir.is_dir() {
                fs::remove_dir_all(&dir).map_err(|e| fail("queries", e.to_string()))?;
            }
        }
        println!("goal: queries ok 0 queries");
        return Ok(());
    }
    let lexicon = path.join("lexicon.ulex");
    if lexicon.is_symlink() {
        return Err(fail(
            "guideline",
            format!("lexicon is a symlink: {}", show(&lexicon)),
        ));
    }
    let lexicon = lexicon.is_file().then_some(lexicon);
    let swipl = process::swipl()?;
    let scratch = process::Scratch::new()?;
    let stage = process::stage(&scratch, &swipl)?;
    let manifest = scratch.0.join("queries-manifest");
    queries::manifest(&manifest, &path.join("pl"))?;
    for name in ["pl", "answers", "traces"] {
        mkdir(&scratch.0.join(name))?;
    }
    for id in &ids {
        let ace = root.join(format!("{id}.ace"));
        let bytes = queries::question(&swipl, &stage, id, &ace, lexicon.as_deref())?;
        certify(&swipl, &stage, id, &ace, lexicon.as_deref(), &bytes, true)?;
        let pl = scratch.0.join("pl").join(format!("{id}.pl"));
        process::write(&pl, &bytes)?;
        let answer = queries::answer(id, &manifest, &pl)?;
        let answers = scratch.0.join("answers").join(format!("{id}.pl"));
        process::write(&answers, &answer)?;
        let trace = queries::trace(id, &manifest, &pl, &answers)?;
        let traces = scratch.0.join("traces").join(format!("{id}.pl"));
        process::write(&traces, &trace)?;
        queries::inspect_trace(id, &manifest, &pl, &answers, &traces)?;
    }
    // The three accepted inventories replace their old directories only after
    // every query has certified, answered and passed trace replay.
    for name in ["pl", "answers", "traces"] {
        replace(
            &scratch.0.join(name),
            &root.join(name),
            &scratch.0.join(format!("{name}-old")),
        )?;
    }
    for id in &ids {
        for name in ["pl", "answers", "traces"] {
            println!(
                "goal: wrote {}",
                show(&root.join(name).join(format!("{id}.pl")))
            );
        }
    }
    println!("goal: queries ok {} queries", ids.len());
    Ok(())
}
