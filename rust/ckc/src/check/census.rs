use super::common::*;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
pub(super) fn check(root: &Path, status: &BTreeMap<String, String>) -> Result {
    let path = root.join("audit/census-map.tsv");
    let text = String::from_utf8(corpus(&path, "census-map")?)
        .map_err(|_| violation("census-map", format!("not UTF-8: {}", show(&path))))?;
    if text.contains('\r') {
        return Err(violation("census-map", "carriage return byte in map"));
    }
    if !text.ends_with('\n') {
        return Err(violation("census-map", "map lacks final newline"));
    }
    if !text.starts_with("# format: census<TAB>region<TAB>disposition\n") {
        return Err(violation("census-map", "header bytes drift"));
    }
    let mut seen = BTreeSet::new();
    for line in text[..text.len() - 1]
        .split('\n')
        .filter(|s| !s.starts_with('#'))
    {
        let fields: Vec<&str> = line.split('\t').collect();
        let [key, region, disposition] = fields.as_slice() else {
            return Err(violation(
                "census-map",
                format!("row without 3 columns: {line}"),
            ));
        };
        let b = key.as_bytes();
        if b.len() != 8
            || b[0] != b'p'
            || b[4..6] != *b".C"
            || !b[1..4].iter().chain(&b[6..]).all(u8::is_ascii_digit)
        {
            return Err(violation(
                "census-map",
                format!("census key grammar: {key}"),
            ));
        }
        if !seen.insert(*key) {
            return Err(violation(
                "census-map",
                format!("duplicate census key: {key}"),
            ));
        }
        if *region != "-" && status.get(*region).is_none_or(String::is_empty) {
            return Err(violation(
                "census-map",
                format!("row names no coverage region: {key} {region}"),
            ));
        }
        if disposition.is_empty() {
            return Err(violation("census-map", format!("empty disposition: {key}")));
        }
    }
    if seen.is_empty() {
        return Err(violation("census-map", "map holds no rows"));
    }
    Ok(())
}
