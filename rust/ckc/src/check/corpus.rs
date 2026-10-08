use super::common::*;
use super::{
    adjudication, census, coverage, inventories::Guideline, lexicon, projection, vocabulary,
};
pub(super) fn check(g: &Guideline) -> Result {
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
        vocabulary::check(&g.path, id)?;
    }
    let c = coverage::check(g, true)?;
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
    lexicon::check(g)?;
    temporal(g)
}

// m7t D1: the temporal.tsv grammar section (the kernel parses the same bytes
// for compile + certify).
fn temporal(g: &Guideline) -> Result {
    let Some(path) = &g.temporal else {
        return Ok(());
    };
    let bytes = corpus(path, "temporal")?;
    let rel = format!("guidelines/{}/temporal.tsv", name(&g.path));
    coverage::meter(&coverage::verdict(ckc_kernel::contract::check_temporal(
        rel.as_bytes(),
        &bytes,
    ))?);
    Ok(())
}
