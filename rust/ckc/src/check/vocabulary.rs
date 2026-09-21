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
pub(super) fn check(root: &Path, id: &str) -> Result {
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
            if !INDICATORS.contains(&indicator) {
                return Err(violation(
                    "product-vocabulary",
                    format!("undeclared indicator in {id}: {indicator}"),
                ));
            }
        } else {
            let functor = line.split('(').next().unwrap_or("");
            if !FUNCTORS.contains(&functor) {
                return Err(violation(
                    "product-vocabulary",
                    format!("unauthorized clause functor in {id}: {functor}"),
                ));
            }
        }
    }
    Ok(())
}
