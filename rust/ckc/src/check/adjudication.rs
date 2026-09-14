use super::common::*;
use super::{coverage, inventories, process};
use ckc_kernel::{EBundle, ECoverage, EDecision, ESrc};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
fn hex(bytes: &[u8]) -> Vec<u8> {
    crate::trust::sha256_hex(bytes).into_bytes()
}
fn display(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}
pub(super) fn derive(g: &inventories::Guideline, c: &ECoverage) -> Result<Vec<EBundle>> {
    let mut bundles = Vec::new();
    for id in &g.docids {
        let ace = hex(&corpus(&g.ace(id), "adjudication")?);
        let (row, payload) = ckc_kernel::contract::check_payload(c, id.as_bytes());
        let row = row.ok_or_else(|| {
            violation(
                "adjudication",
                format!("docid without coverage row bytes: {id}"),
            )
        })?;
        let payload = payload.ok_or_else(|| {
            violation(
                "adjudication",
                format!("docid without region payload: {id}"),
            )
        })?;
        let pl = corpus(&g.pl(id), "adjudication")?;
        if std::str::from_utf8(&pl).is_err() {
            return Err(violation(
                "adjudication",
                format!("compiled document encoding: {id}"),
            ));
        }
        let input = ckc_kernel::contract::check_semantic_input(&pl, id.as_bytes())
            .map_err(|d| violation("adjudication", display(&d)))?;
        let mut b = EBundle {
            docid: id.as_bytes().to_vec(),
            ace,
            cov: hex(&row),
            pay: hex(&payload),
            cl: hex(&input),
            review: vec![],
        };
        b.review = hex(&ckc_kernel::contract::check_bundle_block(&b));
        bundles.push(b);
    }
    Ok(bundles)
}
fn manifest(path: &Path) -> Result<Vec<EBundle>> {
    if path.is_symlink() {
        return Err(violation(
            "adjudication",
            format!("manifest is a symlink: {}", show(path)),
        ));
    }
    if !path.exists() {
        return Err(violation(
            "adjudication",
            format!("manifest missing: {}", show(path)),
        ));
    }
    if !path.is_file() {
        return Err(violation(
            "adjudication",
            format!("manifest is not a regular file: {}", show(path)),
        ));
    }
    let raw = read(path, "adjudication")?;
    let src = match std::str::from_utf8(&raw) {
        Ok(_) => ESrc::Bytes(raw),
        Err(e) => ESrc::Bad(e.valid_up_to()),
    };
    let (rows, problem) = ckc_kernel::contract::check_parse_manifest(&src, show(path).as_bytes());
    for (i, row) in rows.iter().enumerate() {
        if row.review != hex(&ckc_kernel::contract::check_bundle_block(row)) {
            return Err(violation(
                "adjudication",
                format!("manifest row {} review_sha256 self-consistency", i + 3),
            ));
        }
    }
    if let Some(d) = problem {
        return Err(violation("adjudication", display(&d)));
    }
    Ok(rows)
}
fn historical(gid: &str, commit: &str) -> Result<BTreeMap<Vec<u8>, Vec<u8>>> {
    let scratch = process::Scratch::new()?;
    let archive = Command::new("git")
        .args(["archive", commit, "--", &format!("guidelines/{gid}")])
        .output()
        .map_err(|_| {
            violation(
                "adjudication",
                format!("ledger commit lacks guideline tree: {gid} {commit}"),
            )
        })?;
    if !archive.status.success() {
        return Err(violation(
            "adjudication",
            format!("ledger commit lacks guideline tree: {gid} {commit}"),
        ));
    }
    let mut child = Command::new("tar")
        .args(["-x", "-C"])
        .arg(&scratch.0)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| {
            violation(
                "adjudication",
                format!("ledger commit tree extraction failed: {gid} {commit}"),
            )
        })?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| fail("adjudication", "archive stdin missing"))?;
    let writer = std::thread::spawn(move || {
        input.write_all(&archive.stdout).ok();
    });
    let out = child.wait_with_output().map_err(|_| {
        violation(
            "adjudication",
            format!("ledger commit tree extraction failed: {gid} {commit}"),
        )
    })?;
    writer
        .join()
        .map_err(|_| fail("adjudication", "archive writer failed"))?;
    if !out.status.success() {
        return Err(violation(
            "adjudication",
            format!("ledger commit tree extraction failed: {gid} {commit}"),
        ));
    }
    let path = scratch.0.join("guidelines").join(gid);
    let g = inventories::collect(&path)?;
    let c = coverage::check(&g, false)?;
    Ok(derive(&g, &c)?
        .into_iter()
        .map(|b| (b.docid, b.review))
        .collect())
}
fn commit_row(
    gid: &str,
    index: usize,
    row: &EDecision,
    bundles: &[EBundle],
    cache: &mut BTreeMap<Vec<u8>, BTreeMap<Vec<u8>, Vec<u8>>>,
) -> Result {
    let commit = display(&row.commit);
    let docid = display(&row.docid);
    let exists = Command::new("git")
        .args(["cat-file", "-e", &format!("{commit}^{{commit}}")])
        .output();
    if !exists.is_ok_and(|r| r.status.success()) {
        return Err(violation(
            "adjudication",
            format!("ledger row {index} commit absent from repository: {commit}"),
        ));
    }
    if bundles
        .iter()
        .any(|b| b.docid == row.docid && b.review == row.digest)
    {
        return Ok(());
    }
    if !cache.contains_key(&row.commit) {
        cache.insert(row.commit.clone(), historical(gid, &commit)?);
    }
    let old = cache
        .get(&row.commit)
        .and_then(|m| m.get(&row.docid))
        .filter(|d| !d.is_empty())
        .ok_or_else(|| {
            violation(
                "adjudication",
                format!("ledger row {index} docid absent at recorded commit: {docid} {commit}"),
            )
        })?;
    if *old != row.digest {
        return Err(violation(
            "adjudication",
            format!("ledger row {index} digest mismatch at recorded commit: {docid} {commit}"),
        ));
    }
    Ok(())
}
fn ledger(path: &Path, bundles: &Vec<EBundle>, label: &str, odb: bool) -> Result<Vec<u8>> {
    if path.is_symlink() {
        return Err(violation(
            "adjudication",
            format!("ledger is a symlink: {}", show(path)),
        ));
    }
    if path.exists() && !path.is_file() {
        return Err(violation(
            "adjudication",
            format!("ledger is not a regular file: {}", show(path)),
        ));
    }
    let src = if !path.exists() {
        ESrc::Missing
    } else {
        let bytes = read(path, "adjudication")?;
        match std::str::from_utf8(&bytes) {
            Ok(_) => ESrc::Bytes(bytes),
            Err(e) => ESrc::Bad(e.valid_up_to()),
        }
    };
    let known = bundles.iter().map(|b| b.docid.clone()).collect();
    // R68 harvest seam: replace Result with (prefix, first violation); walk
    // commit custody on the prefix before returning the grammar violation.
    let decisions = ckc_kernel::contract::check_ledger(&src, &known).map_err(|v| {
        match coverage::verdict(v) {
            Err(e) => e,
            Ok(_) => fail("adjudication", "kernel violation returned success"),
        }
    })?;
    if odb {
        let mut cache = BTreeMap::new();
        for (i, row) in decisions.iter().enumerate() {
            if !row.commit.is_empty() {
                commit_row(label, i + 2, row, bundles, &mut cache)?;
            }
        }
    }
    Ok(ckc_kernel::contract::check_adjudication_meter(
        label.as_bytes(),
        &decisions,
        bundles,
    ))
}
pub(super) fn validate(ledger_path: &Path, manifest_path: &Path, label: &str) -> Result<Vec<u8>> {
    if !valid_docid(label) {
        return Err(fail(
            "usage",
            format!("label must match [a-z0-9-]+: {label}"),
        ));
    }
    ledger(ledger_path, &manifest(manifest_path)?, label, false)
}
pub(super) fn check(g: &inventories::Guideline, c: &ECoverage) -> Result {
    let bundles = derive(g, c)?;
    let derived = ckc_kernel::contract::check_print_manifest(&bundles);
    let path = g.path.join("audit/review-manifest.tsv");
    let hint = format!(
        "; regenerate: python3 -P tools/goal.py review-manifest {}",
        name(&g.path)
    );
    if path.is_symlink() {
        return Err(violation(
            "adjudication",
            format!("manifest is a symlink: {}", show(&path)),
        ));
    }
    if !path.exists() {
        return Err(violation(
            "adjudication",
            format!("manifest missing: {}{hint}", show(&path)),
        ));
    }
    if !path.is_file() {
        return Err(violation(
            "adjudication",
            format!("manifest is not a regular file: {}", show(&path)),
        ));
    }
    if read(&path, "adjudication")? != derived {
        return Err(violation(
            "adjudication",
            format!("manifest stale: {}{hint}", show(&path)),
        ));
    }
    coverage::meter(&ledger(
        &g.path.join("audit/adjudication.tsv"),
        &bundles,
        &name(&g.path),
        true,
    )?);
    Ok(())
}
