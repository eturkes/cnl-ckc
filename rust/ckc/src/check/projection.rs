use super::common::*;
use std::collections::BTreeSet;
use std::path::Path;
const HEADER: &str = "# format: docid<TAB>region<TAB>kept<TAB>dropped\n# per-document projection loss record: what each minimal rule keeps from its verbatim source\n# region and what it drops or interprets. Header bytes, row shape and per-document row\n# coverage are validated by goal.py check; kept/dropped prose stays document-owned.\n";
pub(super) fn check(root: &Path) -> Result<Vec<(String, String)>> {
    let text = corpus_text(
        &root.join("audit/projection-notes.tsv"),
        "projection-ledger",
    )?;
    if text.contains('\r') {
        return Err(violation(
            "projection-ledger",
            "carriage return byte in ledger",
        ));
    }
    if !text.ends_with('\n') {
        return Err(violation("projection-ledger", "ledger lacks final newline"));
    }
    let Some(body) = text.strip_prefix(HEADER) else {
        return Err(violation("projection-ledger", "header bytes drift"));
    };
    if body.is_empty() {
        return Err(violation("projection-ledger", "ledger holds no rows"));
    }
    let mut seen = BTreeSet::new();
    let mut pairs = vec![];
    for line in body[..body.len() - 1].split('\n') {
        let fields: Vec<&str> = line.split('\t').collect();
        let [docid, region, kept, dropped] = fields.as_slice() else {
            return Err(violation(
                "projection-ledger",
                format!("row without 4 columns: {line}"),
            ));
        };
        if !valid_docid(docid) {
            return Err(violation(
                "projection-ledger",
                format!("invalid docid: {docid}"),
            ));
        }
        for (value, detail) in [
            (region, "empty region"),
            (kept, "empty kept column"),
            (dropped, "empty dropped column"),
        ] {
            if value.is_empty() {
                return Err(violation(
                    "projection-ledger",
                    format!("{detail} for: {docid}"),
                ));
            }
        }
        if !seen.insert(*docid) {
            return Err(violation(
                "projection-ledger",
                format!("duplicate row docid: {docid}"),
            ));
        }
        pairs.push((docid.to_string(), region.to_string()));
    }
    Ok(pairs)
}
