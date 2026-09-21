use super::common::*;
use super::inventories::Guideline;
use ckc_kernel::{ECoverage, EFileSrc, EVerdict};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::Path;

fn file_source(path: &Path) -> EFileSrc {
    if path.is_symlink() {
        return EFileSrc::Symlink;
    }
    if !path.is_file() {
        return EFileSrc::Missing;
    }
    match std::fs::read(path) {
        Err(_) => EFileSrc::Unreadable,
        Ok(b) => match std::str::from_utf8(&b) {
            Ok(_) => EFileSrc::Bytes(b),
            Err(e) => EFileSrc::Bad(e.valid_up_to()),
        },
    }
}
pub(super) fn verdict(v: EVerdict) -> Result<Vec<u8>> {
    let rendered = ckc_kernel::contract::check_render(&v);
    if rendered.rc == 0 {
        Ok(rendered.out)
    } else {
        Err(Failure {
            rc: rendered.rc,
            out: rendered.out,
            err: vec![],
        })
    }
}
pub(super) fn meter(bytes: &[u8]) {
    std::io::stdout().write_all(bytes).ok();
}
pub(super) fn check(g: &Guideline, emit: bool) -> Result<ECoverage> {
    let path = g.path.join("coverage.tsv");
    let bytes = corpus(&path, "coverage")?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| violation("coverage", format!("invalid_utf8 {}", show(&path))))?;
    let mut seen = BTreeSet::new();
    let mut files = vec![];
    for line in text.split('\n').filter(|s| !s.starts_with('#')) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() == 5 {
            let f = fields[1];
            if f.starts_with("source/") && !f.contains("..") && seen.insert(f) {
                files.push((f.as_bytes().to_vec(), file_source(&g.path.join(f))));
            }
        }
    }
    let ids = g.docids.iter().map(|s| s.as_bytes().to_vec()).collect();
    let c = ckc_kernel::contract::check_coverage(&bytes, &ids, &files, show(&g.path).as_bytes())
        .map_err(|v| match verdict(v) {
            Err(e) => e,
            Ok(_) => fail("coverage", "kernel violation returned success"),
        })?;
    if emit {
        meter(&ckc_kernel::contract::check_coverage_meter(
            name(&g.path).as_bytes(),
            &c,
        ));
    }
    Ok(c)
}
pub(super) fn statuses(c: &ECoverage) -> BTreeMap<String, String> {
    c.rows
        .iter()
        .map(|r| {
            let id = String::from_utf8_lossy(&r.id).into_owned();
            let raw = r.line.strip_suffix(b"\n").unwrap_or(&r.line);
            let status = raw.rsplit(|b| *b == b'\t').next().unwrap_or_default();
            (id, String::from_utf8_lossy(status).into_owned())
        })
        .collect()
}
