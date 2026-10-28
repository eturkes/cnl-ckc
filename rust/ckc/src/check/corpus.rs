use super::common::*;
use super::{
    adjudication, census, coverage, cycle, derived, functors, inventories::Guideline, lexicon,
    projection,
};
use ckc_kernel::ECoverage;
pub(super) fn check(g: &Guideline) -> Result {
    // The table grammar reads one small file; it runs before the corpus ledgers and the
    // lexicon (a full Clex pass, the costliest section), so a table defect stops the run early.
    vocabulary_table(g)?;
    let pairs = projection::check(&g.path)?;
    for (id, _) in &pairs {
        if !g.docids.contains(id) {
            return Err(violation(
                "projection-ledger",
                format!("row for unknown docid: {id}"),
            ));
        }
    }
    for id in &g.docids {
        if !pairs.iter().any(|(d, _)| d == id) {
            return Err(violation(
                "projection-ledger",
                format!("docid missing projection row: {id}"),
            ));
        }
        functors::check(&g.path, id)?;
    }
    cycle::check(g)?;
    derived::check(g)?;
    let c = coverage::check(g, true)?;
    supersession_rows(g, &c)?;
    let status = coverage::statuses(&c);
    for (id, region) in pairs {
        let actual = status.get(&region).map(String::as_str).unwrap_or("");
        if actual.is_empty() {
            return Err(violation(
                "projection-coverage",
                format!("projection row names no coverage region: {id} {region}"),
            ));
        }
        if actual != format!("ace({id})") {
            return Err(violation(
                "projection-coverage",
                format!("coverage region {region} does not carry ace({id}): {actual}"),
            ));
        }
    }
    census::check(&g.path, &status)?;
    adjudication::check(g, &c)?;
    lexicon::check(g)
}

// sup1 D1: a superseded document has its coverage row `ace(<docid>)`; a superseded
// passage is a coverage region without its own document.
fn supersession_rows(g: &Guideline, c: &ECoverage) -> Result {
    let Some(path) = &g.vocabulary else {
        return Ok(());
    };
    let bytes = corpus(path, "vocabulary")?;
    let rel = format!("guidelines/{}/vocabulary.tsv", name(&g.path));
    coverage::meter(&coverage::verdict(
        ckc_kernel::contract::check_supersession(rel.as_bytes(), &bytes, c),
    )?);
    Ok(())
}

// c1 D4: the vocabulary.tsv grammar section (the kernel parses the same bytes
// for compile + certify).
fn vocabulary_table(g: &Guideline) -> Result {
    let Some(path) = &g.vocabulary else {
        return Ok(());
    };
    let bytes = corpus(path, "vocabulary")?;
    let rel = format!("guidelines/{}/vocabulary.tsv", name(&g.path));
    coverage::meter(&coverage::verdict(ckc_kernel::contract::check_vocabulary(
        rel.as_bytes(),
        &bytes,
    ))?);
    Ok(())
}
