use super::common::*;
use super::corpus::Corpus;
use ckc_kernel::{
    EBundle, ECheck, ECorpus, ECoverage, ECoverageRow, EDocument, EEvidence, EGuideline, ESrc,
    EStatus,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub(super) struct View {
    pub corpus: ECorpus,
    pub errors: Vec<Vec<Option<String>>>,
}
pub(super) fn load(corpus: &Corpus, token: &[u8]) -> Result<View> {
    let root = corpus.root.join("guidelines");
    if root.is_symlink() || !root.is_dir() {
        return Err("ui: viewmodel: missing guidelines directory".into());
    }
    let mut guidelines = Vec::new();
    let mut errors = Vec::new();
    for path in entries(&root)? {
        if path.is_symlink() || !path.is_dir() {
            continue;
        }
        let (model, issues) = guideline(&path, &name(&path))?;
        guidelines.push(model);
        errors.push(issues);
    }
    Ok(View {
        corpus: ECorpus {
            guidelines,
            token: token.to_vec(),
        },
        errors,
    })
}

type TableRow<'a> = (&'a str, Vec<&'a str>);
fn table<'a>(s: &'a str, fields: usize, gid: &str, label: &str) -> Result<Vec<TableRow<'a>>> {
    s.split('\n')
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|line| {
            let row: Vec<_> = line.split('\t').collect();
            if row.len() == fields {
                Ok((line, row))
            } else {
                Err(format!(
                    "ui: viewmodel: {gid} malformed {label} row: {line}"
                ))
            }
        })
        .collect()
}
fn empty_evidence() -> EEvidence {
    EEvidence {
        census: Vec::new(),
        locators: Vec::new(),
        payloads: Vec::new(),
        ordinal: Vec::new(),
    }
}
fn coverage(bytes: &[u8], gid: &str) -> Result<(ECoverage, Vec<String>)> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| format!("ui: viewmodel: {gid} file not UTF-8: coverage.tsv"))?;
    let mut rows = Vec::new();
    let mut ids = BTreeSet::new();
    let mut duplicate = BTreeSet::new();
    let mut files = Vec::new();
    for (raw, f) in table(s, 5, gid, "coverage")? {
        let invalid = || format!("ui: viewmodel: {gid} malformed coverage row: {}", f[0]);
        let status = if let Some(tail) = f[4].strip_prefix("ace(") {
            let id = tail.strip_suffix(')').ok_or_else(invalid)?;
            if !valid_id(id) {
                return Err(format!("ui: viewmodel: {gid} invalid docid {id}"));
            }
            if !ids.insert(id.to_owned()) {
                duplicate.insert(id.to_owned());
            }
            EStatus::Ace(id.as_bytes().to_vec())
        } else if let Some(tail) = f[4].strip_prefix("restates(") {
            let target = tail
                .strip_suffix(')')
                .filter(|s| !s.is_empty())
                .ok_or_else(invalid)?;
            EStatus::Restates(target.as_bytes().to_vec())
        } else if let Some(tail) = f[4].strip_prefix("uncovered(") {
            let reason = tail.strip_suffix(')').and_then(|s| s.split_once(": "));
            if !reason.is_some_and(|(_, reason)| !reason.is_empty()) {
                return Err(invalid());
            }
            EStatus::Uncovered
        } else if f[4] == "pending" {
            EStatus::Pending
        } else {
            return Err(invalid());
        };
        let file = f[1].as_bytes().to_vec();
        if !files.contains(&file) {
            files.push(file.clone());
        }
        rows.push(ECoverageRow {
            id: f[0].as_bytes().to_vec(),
            file,
            status,
            line: format!("{raw}\n").into_bytes(),
        });
    }
    if let Some(id) = duplicate.first() {
        return Err(format!("ui: viewmodel: {gid} duplicate docid {id}"));
    }
    let evidence = files.iter().map(|_| empty_evidence()).collect();
    Ok((
        ECoverage {
            rows,
            files,
            evidence,
        },
        ids.into_iter().collect(),
    ))
}
fn stems(path: &Path, suffix: &str) -> Result<Vec<String>> {
    if !path.is_dir() {
        return Ok(Vec::new());
    }
    let mut values = Vec::new();
    for entry in entries(path)? {
        if !entry.is_symlink()
            && entry.is_file()
            && let Some(stem) = name(&entry).strip_suffix(suffix)
        {
            values.push(stem.to_owned());
        }
    }
    values.sort();
    Ok(values)
}
fn inventory(
    root: &Path,
    gid: &str,
    ids: &[String],
    dir: &str,
    suffix: &str,
    label: &str,
) -> Result<()> {
    let found = stems(&root.join(dir), suffix)?;
    for id in ids {
        if !found.contains(id) {
            return Err(format!(
                "ui: viewmodel: {gid} doc {id} missing {label} file"
            ));
        }
    }
    for id in found {
        if !ids.contains(&id) {
            return Err(format!(
                "ui: viewmodel: {gid} orphan {label} file {id}{suffix}"
            ));
        }
    }
    Ok(())
}
fn manifest(bytes: &[u8], gid: &str, ids: &[String]) -> Result<BTreeMap<String, EBundle>> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| format!("ui: viewmodel: {gid} file not UTF-8: audit/review-manifest.tsv"))?;
    let mut bundles = BTreeMap::new();
    for (_, f) in table(s, 6, gid, "review-manifest")? {
        if bundles.contains_key(f[0]) {
            return Err(format!(
                "ui: viewmodel: {gid} duplicate review-manifest doc {}",
                f[0]
            ));
        }
        bundles.insert(
            f[0].to_owned(),
            EBundle {
                docid: f[0].as_bytes().to_vec(),
                ace: f[1].as_bytes().to_vec(),
                cov: f[2].as_bytes().to_vec(),
                pay: f[3].as_bytes().to_vec(),
                cl: f[4].as_bytes().to_vec(),
                review: f[5].as_bytes().to_vec(),
            },
        );
    }
    for id in ids {
        if !bundles.get(id).is_some_and(|b| !b.review.is_empty()) {
            return Err(format!(
                "ui: viewmodel: {gid} doc {id} missing review-manifest row"
            ));
        }
    }
    for id in bundles.keys() {
        if !ids.contains(id) {
            return Err(format!(
                "ui: viewmodel: {gid} orphan review-manifest doc {id}"
            ));
        }
    }
    Ok(bundles)
}
fn census(s: &str) -> Vec<u8> {
    let mut cursor = s;
    while let Some((_, tail)) = cursor.split_once("identify the ") {
        let n = tail.bytes().take_while(u8::is_ascii_digit).count();
        if n > 0 && tail[n..].starts_with(" payloads below") {
            return tail.as_bytes()[..n].to_vec();
        }
        cursor = tail;
    }
    Vec::new()
}
fn evidence(s: &str) -> EEvidence {
    let mut out = empty_evidence();
    out.census = census(s);
    let mut current: Option<(Vec<u8>, Vec<Vec<u8>>)> = None;
    let mut past_blank = false;
    for line in s.split('\n') {
        let locator = line
            .strip_prefix('[')
            .and_then(|v| v.strip_suffix(']'))
            .and_then(|v| v.split_once(" | "))
            .map(|(id, _)| id)
            .filter(|id| !id.is_empty() && !id.contains(' '));
        if let Some(id) = locator {
            if let Some(old) = current.take() {
                out.payloads.push(old);
            }
            out.locators.push(id.as_bytes().to_vec());
            current = Some((id.as_bytes().to_vec(), Vec::new()));
        } else if line.is_empty() {
            past_blank = true;
        } else {
            if let Some((_, lines)) = current.as_mut() {
                lines.push(line.as_bytes().to_vec());
            }
            if past_blank {
                let payload = match line.split_once(". ") {
                    Some((head, tail))
                        if !head.is_empty() && head.bytes().all(|b| b.is_ascii_digit()) =>
                    {
                        tail
                    }
                    _ => line,
                };
                out.ordinal.push(payload.as_bytes().to_vec());
            }
        }
    }
    if let Some(old) = current {
        out.payloads.push(old);
    }
    out
}
fn payloads(root: &Path, gid: &str, ids: &[String], c: &mut ECoverage) -> Result<()> {
    let mut loaded = BTreeSet::new();
    for id in ids {
        let row = c
            .rows
            .iter()
            .find(|r| matches!(&r.status, EStatus::Ace(d) if d == id.as_bytes()))
            .ok_or_else(|| format!("ui: viewmodel: {gid} doc {id} missing coverage row"))?;
        let file = text(&row.file);
        let region = text(&row.id);
        let unresolved =
            || format!("ui: viewmodel: {gid} doc {id} region {region} unresolved in {file}");
        if file.starts_with('/') || file.contains('\\') || file.split('/').any(|s| s == "..") {
            return Err(unresolved());
        }
        let index = c
            .files
            .iter()
            .position(|f| f == &row.file)
            .ok_or_else(unresolved)?;
        if loaded.insert(index) {
            let data = load_text(root, gid, &file)?;
            c.evidence[index] = evidence(&text(&data));
        }
        let ev = &c.evidence[index];
        if !ev.locators.is_empty() {
            let count = ev.locators.iter().filter(|loc| *loc == &row.id).count();
            if count == 0 {
                return Err(unresolved());
            }
            if count > 1 {
                return Err(format!(
                    "ui: viewmodel: {gid} duplicate region locator {region} in {file}"
                ));
            }
            let lines = ev
                .payloads
                .iter()
                .find(|(loc, _)| loc == &row.id)
                .map(|(_, ls)| ls.len())
                .unwrap_or(0);
            if lines == 0 {
                return Err(format!(
                    "ui: viewmodel: {gid} doc {id} region {region} has empty payload"
                ));
            }
            if lines != 1 {
                return Err(unresolved());
            }
        } else {
            let total = c.rows.iter().filter(|r| r.file == row.file).count();
            if ev.census != ev.ordinal.len().to_string().as_bytes() || total != ev.ordinal.len() {
                return Err(format!(
                    "ui: viewmodel: {gid} region census mismatch {file} coverage={total} payloads={}",
                    ev.ordinal.len()
                ));
            }
            let (_, selected) = ckc_kernel::contract::check_payload(c, id.as_bytes());
            if selected.is_none_or(|p| p.is_empty()) {
                return Err(format!(
                    "ui: viewmodel: {gid} doc {id} region {region} has empty payload"
                ));
            }
        }
    }
    Ok(())
}
fn ledger(root: &Path, gid: &str, manifest: &[u8]) -> Result<(ESrc, Vec<u8>)> {
    let path = root.join("audit/adjudication.tsv");
    if !path.is_file() {
        return Ok((ESrc::Missing, b"absent".to_vec()));
    }
    let bytes = fs::read(path)
        .map_err(|_| format!("ui: viewmodel: {gid} missing audit/adjudication.tsv"))?;
    let hash = digest(&bytes);
    let src = match std::str::from_utf8(&bytes) {
        Ok(_) => ESrc::Bytes(bytes),
        Err(e) => ESrc::Bad(e.valid_up_to()),
    };
    let (bundles, issue) = ckc_kernel::contract::check_parse_manifest(
        &ESrc::Bytes(manifest.to_vec()),
        b"manifest.tsv",
    );
    for (i, bundle) in bundles.iter().enumerate() {
        if bundle.review != digest(&ckc_kernel::contract::check_bundle_block(bundle)) {
            return Err(format!(
                "ui: adjudication ledger invalid: goal: adjudication: manifest row {} review_sha256 self-consistency",
                i + 3
            ));
        }
    }
    if let Some(issue) = issue {
        return Err(format!(
            "ui: adjudication ledger invalid: goal: adjudication: {}",
            text(&issue)
        ));
    }
    let ids = bundles.iter().map(|b| b.docid.clone()).collect();
    let (_, issue) = ckc_kernel::contract::check_ledger(&src, &ids);
    if let Some(issue) = issue {
        let rendered = ckc_kernel::contract::check_render(&issue);
        return Err(format!(
            "ui: adjudication ledger invalid: {}",
            text(&rendered.out).trim()
        ));
    }
    Ok((src, hash))
}
pub(super) fn valid_asset(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with('.')
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
}
pub(super) fn media_type(name: &str) -> Option<&'static str> {
    if name.ends_with(".txt") {
        Some("text/plain; charset=utf-8")
    } else if name.ends_with(".pdf") {
        Some("application/pdf")
    } else {
        None
    }
}
fn assets(root: &Path) -> Result<Vec<Vec<u8>>> {
    let source = root.join("source");
    if source.is_symlink() || !source.is_dir() {
        return Ok(Vec::new());
    }
    Ok(entries(&source)?
        .iter()
        .filter(|p| !p.is_symlink() && p.is_file())
        .map(|p| name(p))
        .filter(|n| valid_asset(n) && media_type(n).is_some())
        .map(String::into_bytes)
        .collect())
}
struct Content {
    ace: Vec<u8>,
    pl: Vec<u8>,
    alignment: Option<Vec<u8>>,
}
fn document(root: &Path, gid: &str, c: &ECoverage, bundle: &EBundle) -> Result<Content> {
    let id = text(&bundle.docid);
    let ace_rel = format!("ace/{id}.ace");
    let ace = fs::read(root.join(&ace_rel))
        .map_err(|_| format!("ui: viewmodel: {gid} doc {id} missing ace file"))?;
    let ace = checked_text(ace, gid, &ace_rel)?;
    if digest(&ace) != bundle.ace {
        return Err(format!("ui: digest mismatch: {id} ace"));
    }
    let (_, payload) = ckc_kernel::contract::check_payload(c, &bundle.docid);
    let payload = payload.unwrap_or_default();
    if digest(&payload) != bundle.pay {
        return Err(format!("ui: digest mismatch: {id} payload"));
    }
    let pl = load_text(root, gid, &format!("pl/{id}.pl"))?;
    let alignment = if root.join(format!("align/{id}.tsv")).is_file() {
        let bytes = load_text(root, gid, &format!("align/{id}.tsv"))?;
        let a = text(&bytes).chars().collect::<Vec<_>>();
        let s = text(&payload).chars().collect::<Vec<_>>();
        let d = text(&ace).chars().collect::<Vec<_>>();
        if let ECheck::Err(e) = ckc_kernel::contract::align_check(&a, &s, &d) {
            return Err(format!(
                "ui: viewmodel: {gid} doc {id} align: {}",
                e.iter().collect::<String>()
            ));
        }
        Some(bytes)
    } else {
        None
    };
    Ok(Content { ace, pl, alignment })
}
fn guideline(root: &Path, gid: &str) -> Result<(EGuideline, Vec<Option<String>>)> {
    if !valid_id(gid) {
        return Err(format!("ui: viewmodel: {gid} invalid guideline id"));
    }
    if !root.join("coverage.tsv").is_file() {
        return Err(format!("ui: viewmodel: {gid} missing coverage.tsv"));
    }
    let raw_coverage = load_text(root, gid, "coverage.tsv")?;
    let (mut c, ids) = coverage(&raw_coverage, gid)?;
    inventory(root, gid, &ids, "ace", ".ace", "ace")?;
    if !root.join("audit/review-manifest.tsv").is_file() {
        return Err(format!(
            "ui: viewmodel: {gid} missing audit/review-manifest.tsv"
        ));
    }
    let raw_manifest = load_text(root, gid, "audit/review-manifest.tsv")?;
    let mut bs = manifest(&raw_manifest, gid, &ids)?;
    inventory(root, gid, &ids, "pl", ".pl", "Prolog")?;
    payloads(root, gid, &ids, &mut c)?;
    let (ledger, ledger_digest) = ledger(root, gid, &raw_manifest)?;
    let readme = fs::read(root.join("README.md"))
        .ok()
        .map(|b| checked_text(b, gid, "README.md"))
        .transpose()?;
    let source_names = assets(root)?;
    let mut documents = Vec::new();
    let mut errors = Vec::new();
    for id in ids {
        let bundle = bs
            .remove(&id)
            .ok_or_else(|| format!("ui: viewmodel: {gid} doc {id} missing review-manifest row"))?;
        let (content, error) = match document(root, gid, &c, &bundle) {
            Ok(content) => (content, None),
            Err(error) => (
                Content {
                    ace: Vec::new(),
                    pl: Vec::new(),
                    alignment: None,
                },
                Some(error),
            ),
        };
        documents.push(EDocument {
            bundle,
            ace: content.ace,
            pl: content.pl,
            alignment: content.alignment,
        });
        errors.push(error);
    }
    Ok((
        EGuideline {
            gid: gid.as_bytes().to_vec(),
            readme,
            coverage: c,
            documents,
            ledger,
            ledger_digest,
            source_names,
        },
        errors,
    ))
}
