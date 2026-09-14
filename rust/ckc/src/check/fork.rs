use super::common::*;
use super::text::{lines, strip};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

fn field<'a>(src: &'a str, key: &str) -> &'a str {
    let prefix = format!("{key}: ");
    lines(src)
        .filter_map(|s| s.strip_prefix(&prefix))
        .map(strip)
        .next_back()
        .unwrap_or("")
}
fn scan(path: &Path) -> Result<(String, String)> {
    let bytes = read(path, "fork-notice")?;
    let text = String::from_utf8_lossy(&bytes);
    let mut found = (String::new(), String::new());
    for (i, line) in lines(&text).enumerate() {
        if line.contains("Modified") && line.contains("fork") {
            let dest = if i < 40 { &mut found.0 } else { &mut found.1 };
            if dest.is_empty() {
                *dest = line.to_owned();
            }
        }
    }
    Ok(found)
}
fn dated(s: &str) -> bool {
    s.as_bytes().windows(10).any(|b| {
        b[4] == b'-'
            && b[7] == b'-'
            && b.iter()
                .enumerate()
                .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
    })
}
fn pristine(tree: &Path, tracked: &[String], first: &BTreeSet<String>) -> Result<usize> {
    let prefix = show(tree);
    let path = tree.join("MANIFEST.sha256");
    if !path.is_file() {
        return Err(violation(
            "fork-notice",
            format!("pristine tree lacks MANIFEST.sha256: {prefix}"),
        ));
    }
    let src = text(&path, "fork-notice")?;
    let mut manifest = BTreeMap::new();
    let mut order = Vec::new();
    for line in lines(&src).filter(|s| !s.is_empty()) {
        let Some((digest, rel)) = line.split_once("  ") else {
            return Err(violation(
                "fork-notice",
                format!("malformed manifest row in {prefix}: {line}"),
            ));
        };
        let full = format!("{prefix}/{}", rel.strip_prefix("./").unwrap_or(rel));
        if !manifest.contains_key(&full) {
            order.push(full.clone());
        }
        manifest.insert(full, digest);
    }
    for item in tracked.iter().filter(|s| !first.contains(*s)) {
        let path = Path::new(item);
        let (prominent, buried) = scan(path)?;
        if !prominent.is_empty() || !buried.is_empty() {
            return Err(violation(
                "fork-notice",
                format!("pristine vendored file carries a change notice: {item}"),
            ));
        }
        let Some(expected) = manifest.get(item) else {
            return Err(violation(
                "fork-notice",
                format!("pristine file missing from manifest: {item}"),
            ));
        };
        if crate::trust::sha256_hex(&read(path, "fork-notice")?) != *expected {
            return Err(violation(
                "fork-notice",
                format!("pristine file differs from manifest digest: {item}"),
            ));
        }
    }
    for item in order {
        if !tracked.contains(&item) {
            return Err(violation(
                "fork-notice",
                format!("manifest row without tracked file: {item}"),
            ));
        }
        if first.contains(&item) {
            return Err(violation(
                "fork-notice",
                format!("manifest row names first-party file: {item}"),
            ));
        }
    }
    Ok(0)
}
fn tree(path: &Path) -> Result<usize> {
    let prefix = show(path);
    let prov = path.join("PROVENANCE");
    if !prov.is_file() {
        return Err(violation(
            "fork-notice",
            format!("missing PROVENANCE: {prefix}"),
        ));
    }
    let src = text(&prov, "fork-notice")?;
    let license = field(&src, "License");
    if license.is_empty() {
        return Err(violation(
            "fork-notice",
            format!("PROVENANCE states no License: {prefix}"),
        ));
    }
    let imported = field(&src, "Import commit");
    if imported.is_empty() {
        return Err(violation(
            "fork-notice",
            format!("PROVENANCE states no Import commit: {prefix}"),
        ));
    }
    let needs_date = ["GPL-", "LGPL-", "AGPL-"]
        .iter()
        .any(|p| license.starts_with(p));
    if !needs_date && license != "Apache-2.0" {
        return Err(violation(
            "fork-notice",
            format!("unrecognized license in {prefix}: {license}"),
        ));
    }
    let mut tracked: Vec<String> = lines(&git_text(&["ls-files", "--", &prefix])?)
        .map(str::to_owned)
        .collect();
    tracked.sort();
    let log = git_text(&[
        "log",
        "--name-only",
        "--pretty=format:",
        &format!("{imported}..HEAD"),
        "--",
        &prefix,
    ])?;
    let diff = git_text(&["diff", "--name-only", "HEAD", "--", &prefix])?;
    let touched: BTreeSet<&str> = lines(&log)
        .chain(lines(&diff))
        .map(strip)
        .filter(|s| !s.is_empty())
        .collect();
    let mut first = BTreeSet::new();
    for rel in field(&src, "First-party files")
        .split(',')
        .map(strip)
        .filter(|s| !s.is_empty())
    {
        let full = format!("{prefix}/{rel}");
        if !tracked.contains(&full) {
            return Err(violation(
                "fork-notice",
                format!("declared first-party file is untracked: {full}"),
            ));
        }
        first.insert(full);
    }
    if field(&src, "Pristine") == "yes" {
        return pristine(path, &tracked, &first);
    }
    let mut count = 0;
    for item in tracked.iter().filter(|s| !first.contains(*s)) {
        let modified = touched.contains(item.as_str());
        let (prominent, buried) = scan(Path::new(item))?;
        if modified {
            count += 1;
            if prominent.is_empty() {
                let reason = if buried.is_empty() {
                    "modified vendored file carries no change notice"
                } else {
                    "change notice is not prominent"
                };
                return Err(violation("fork-notice", format!("{reason}: {item}")));
            }
            if needs_date && !dated(&prominent) {
                return Err(violation(
                    "fork-notice",
                    format!("change notice states no date: {item}"),
                ));
            }
        } else if !prominent.is_empty() {
            return Err(violation(
                "fork-notice",
                format!("unmodified vendored file carries a change notice: {item}"),
            ));
        }
    }
    Ok(count)
}
pub(super) fn check() -> Result {
    let root = Path::new("vendor");
    if !root.is_dir() {
        return Err(violation("fork-notice", "missing vendor directory"));
    }
    let mut trees = 0;
    let mut modified = 0;
    for path in entries(root, "fork-notice")? {
        if path.is_symlink() {
            return Err(violation(
                "fork-notice",
                format!("vendor entry is a symlink: {}", show(&path)),
            ));
        }
        if !path.is_dir() {
            return Err(violation(
                "fork-notice",
                format!("vendor entry is not a directory: {}", show(&path)),
            ));
        }
        modified += tree(&path)?;
        trees += 1;
    }
    if trees == 0 {
        return Err(violation("fork-notice", "no vendor trees found"));
    }
    if modified == 0 {
        return Err(violation("fork-notice", "no modified vendored files found"));
    }
    println!("goal: fork notices ok {trees} trees {modified} modified files");
    Ok(())
}
