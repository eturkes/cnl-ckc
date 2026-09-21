use super::common::*;
use ckc_kernel::{EBundle, EFileSrc};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

fn fail(category: &str, detail: impl AsRef<str>) -> String {
    format!(
        "goal: {category}: {}",
        detail.as_ref().replace('\n', "\\n").replace('\r', "\\r")
    )
}
fn regular(path: &Path, category: &str) -> Result<Vec<u8>> {
    if path.is_symlink() {
        return Err(fail(category, format!("is a symlink: {}", path.display())));
    }
    if !path.is_file() {
        return Err(fail(category, format!("missing: {}", path.display())));
    }
    fs::read(path).map_err(|_| fail(category, format!("unreadable {}", path.display())))
}
fn source(path: &Path) -> EFileSrc {
    if path.is_symlink() {
        return EFileSrc::Symlink;
    }
    if !path.is_file() {
        return EFileSrc::Missing;
    }
    match fs::read(path) {
        Err(_) => EFileSrc::Unreadable,
        Ok(b) => match std::str::from_utf8(&b) {
            Ok(_) => EFileSrc::Bytes(b),
            Err(e) => EFileSrc::Bad(e.valid_up_to()),
        },
    }
}
pub(super) fn derive(root: &Path) -> Result<Vec<EBundle>> {
    let root = root
        .canonicalize()
        .map_err(|_| fail("guideline", format!("not a directory: {}", root.display())))?;
    let ace_dir = root.join("ace");
    if ace_dir.is_symlink() {
        return Err(fail(
            "guideline",
            format!("ace directory is a symlink: {}", ace_dir.display()),
        ));
    }
    if !ace_dir.is_dir() {
        return Err(fail(
            "guideline",
            format!("missing ace directory: {}", ace_dir.display()),
        ));
    }
    let mut ids = Vec::new();
    for path in entries(&ace_dir)? {
        let file = name(&path);
        if path.is_symlink() || !path.is_file() {
            return Err(fail(
                "guideline",
                format!("entry is not a regular file: {file}"),
            ));
        }
        let id = file
            .strip_suffix(".ace")
            .ok_or_else(|| fail("guideline", format!("unsupported ace entry: {file}")))?;
        if !valid_id(id) {
            return Err(fail("docid", format!("invalid document id: {id}")));
        }
        ids.push(id.as_bytes().to_vec());
    }
    if ids.is_empty() {
        return Err(fail(
            "guideline",
            format!("no .ace documents in: {}", ace_dir.display()),
        ));
    }
    let lexicon = root.join("lexicon.ulex");
    if lexicon.is_symlink() {
        return Err(fail(
            "guideline",
            format!("lexicon is a symlink: {}", lexicon.display()),
        ));
    }
    let path = root.join("coverage.tsv");
    let raw = regular(&path, "coverage")?;
    let decoded = std::str::from_utf8(&raw)
        .map_err(|_| fail("coverage", format!("invalid_utf8 {}", path.display())))?;
    let mut seen = BTreeSet::new();
    let mut files = Vec::new();
    for line in decoded.split('\n').filter(|l| !l.starts_with('#')) {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() == 5 {
            let file = fields[1];
            if file.starts_with("source/") && !file.contains("..") && seen.insert(file) {
                files.push((file.as_bytes().to_vec(), source(&root.join(file))));
            }
        }
    }
    let coverage =
        ckc_kernel::contract::check_coverage(&raw, &ids, &files, root.to_string_lossy().as_bytes())
            .map_err(|v| {
                text(&ckc_kernel::contract::check_render(&v).out)
                    .trim()
                    .to_owned()
            })?;
    let mut bundles = Vec::new();
    for id in ids {
        let docid = text(&id);
        let ace = digest(&regular(
            &ace_dir.join(format!("{docid}.ace")),
            "adjudication",
        )?);
        let (row, payload) = ckc_kernel::contract::check_payload(&coverage, &id);
        let row = row.ok_or_else(|| {
            fail(
                "adjudication",
                format!("docid without coverage row bytes: {docid}"),
            )
        })?;
        let payload = payload.ok_or_else(|| {
            fail(
                "adjudication",
                format!("docid without region payload: {docid}"),
            )
        })?;
        let pl = regular(&root.join("pl").join(format!("{docid}.pl")), "adjudication")?;
        if std::str::from_utf8(&pl).is_err() {
            return Err(fail(
                "adjudication",
                format!("compiled document encoding: {docid}"),
            ));
        }
        let semantic = ckc_kernel::contract::check_semantic_input(&pl, &id)
            .map_err(|e| fail("adjudication", text(&e)))?;
        let mut bundle = EBundle {
            docid: id,
            ace,
            cov: digest(&row),
            pay: digest(&payload),
            cl: digest(&semantic),
            review: Vec::new(),
        };
        bundle.review = digest(&ckc_kernel::contract::check_bundle_block(&bundle));
        bundles.push(bundle);
    }
    Ok(bundles)
}
