use super::common::*;
use super::text::strip;
use std::path::Path;
const FUNCTORS: [&str; 9] = [
    "guideline_schema_version",
    "guideline_document",
    "guideline_entity",
    "guideline_cardinality",
    "guideline_event",
    "guideline_arg",
    "guideline_pp",
    "guideline_property",
    "guideline_operator",
];
const INDICATORS: [&str; 9] = [
    "guideline_schema_version/1",
    "guideline_document/3",
    "guideline_entity/4",
    "guideline_cardinality/5",
    "guideline_event/3",
    "guideline_arg/4",
    "guideline_pp/4",
    "guideline_property/4",
    "guideline_operator/3",
];
// m7t D3: a v2 guideline (one carrying temporal.tsv) adds the record /4 and
// the two temporal annotations.
const V2_FUNCTORS: [&str; 2] = ["guideline_interval", "guideline_recurrence"];
const V2_INDICATORS: [&str; 3] = [
    "guideline_document/4",
    "guideline_interval/6",
    "guideline_recurrence/4",
];
pub(super) fn check(root: &Path, id: &str) -> Result {
    let v2 = root.join("temporal.tsv").is_file();
    let src = corpus_text(
        &root.join("pl").join(format!("{id}.pl")),
        "product-vocabulary",
    )?;
    for line in src
        .split('\n')
        .map(strip)
        .filter(|s| !s.is_empty() && !s.starts_with('%'))
    {
        if let Some(inner) = line.strip_prefix(":- ") {
            let inner = inner.strip_suffix(").").unwrap_or(inner);
            let Some((kind, indicator)) = inner
                .split_once('(')
                .filter(|(k, _)| ["multifile", "discontiguous"].contains(k))
            else {
                return Err(violation(
                    "product-vocabulary",
                    format!("unauthorized directive in {id}: {line}"),
                ));
            };
            let _ = kind;
            if !INDICATORS.contains(&indicator) && !(v2 && V2_INDICATORS.contains(&indicator)) {
                return Err(violation(
                    "product-vocabulary",
                    format!("undeclared indicator in {id}: {indicator}"),
                ));
            }
        } else {
            let functor = line.split('(').next().unwrap_or("");
            if !FUNCTORS.contains(&functor) && !(v2 && V2_FUNCTORS.contains(&functor)) {
                return Err(violation(
                    "product-vocabulary",
                    format!("unauthorized clause functor in {id}: {functor}"),
                ));
            }
        }
    }
    Ok(())
}
