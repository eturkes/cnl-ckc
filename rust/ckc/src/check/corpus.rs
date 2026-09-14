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
    lexicon::check(g)
}
