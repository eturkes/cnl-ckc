use super::common::*;
use super::{coverage, inventories::Guideline};
use std::collections::BTreeSet;
use std::path::Path;
pub(super) fn check(g: &Guideline) -> Result {
    let Some(path) = &g.lexicon else {
        return Ok(());
    };
    let ulex = corpus_text(path, "lexicon-file")?.into_bytes();
    let mut ace = Vec::new();
    for id in &g.docids {
        ace.push(
            text(&g.ace(id), "lexicon-file")?
                .replace("\r\n", "\n")
                .replace('\r', "\n")
                .into_bytes(),
        );
    }
    let clex = corpus_text(Path::new("vendor/clex/clex_lexicon.pl"), "clex-file")?.into_bytes();
    let shadow = g.path.join("audit/lexicon-shadow.tsv");
    if shadow.is_symlink() {
        return Err(violation(
            "lexicon-shadow-file",
            format!("is a symlink: {}", show(&shadow)),
        ));
    }
    let mut rulings = vec![];
    if shadow.is_file() {
        let src = corpus_text(&shadow, "lexicon-shadow-file")?;
        let mut lines = src.split('\n');
        if lines.next() != Some("ulex_entry\tclex_entries\truling") {
            return Err(violation(
                "lexicon-shadow-file",
                format!("first line is not the 3-column header: {}", show(&shadow)),
            ));
        }
        let mut seen = BTreeSet::new();
        for line in lines.filter(|s| !s.is_empty()) {
            let fields: Vec<&str> = line.split('\t').collect();
            let [ulex, clex, ruling] = fields.as_slice() else {
                return Err(violation(
                    "lexicon-shadow-file",
                    format!("row without 3 cells: {line}"),
                ));
            };
            if ulex.is_empty() {
                return Err(violation(
                    "lexicon-shadow-file",
                    format!("empty ulex_entry cell: {line}"),
                ));
            }
            if clex.is_empty() {
                return Err(violation(
                    "lexicon-shadow-file",
                    format!("empty clex_entries cell: {line}"),
                ));
            }
            if ruling.is_empty() {
                return Err(violation(
                    "lexicon-shadow-file",
                    format!("empty ruling for: {ulex}"),
                ));
            }
            if !seen.insert((*ulex, *clex)) {
                return Err(violation(
                    "lexicon-shadow-file",
                    format!("duplicate ruling row: {ulex}"),
                ));
            }
            rulings.push((ulex.as_bytes().to_vec(), clex.as_bytes().to_vec()));
        }
    }
    coverage::meter(&coverage::verdict(ckc_kernel::contract::check_lexicon(
        show(path).as_bytes(),
        &ulex,
        &clex,
        &ace,
        &rulings,
    ))?);
    Ok(())
}
