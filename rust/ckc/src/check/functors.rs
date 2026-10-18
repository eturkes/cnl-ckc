use super::common::*;
use super::text::strip;
use std::path::Path;
// c1 D2 + D11, s1 D1, s2 D1, nt1 D2: the product vocabulary = the 19 indicators of the one declaration block.
pub(super) const INDICATORS: [&str; 19] = [
    "guideline_schema_version/1",
    "guideline_document/4",
    "guideline_entity/4",
    "guideline_cardinality/5",
    "guideline_event/3",
    "guideline_arg/4",
    "guideline_pp/4",
    "guideline_property/4",
    "guideline_operator/3",
    "guideline_interval/6",
    "guideline_recurrence/4",
    "guideline_frequency/5",
    "guideline_order/4",
    "guideline_recurrence_window/7",
    "guideline_range/3",
    "guideline_grade/5",
    "guideline_preference/4",
    "guideline_qualifier/4",
    "guideline_unit/3",
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
            if !INDICATORS
                .iter()
                .any(|i| i.split('/').next() == Some(functor))
            {
                return Err(violation(
                    "product-vocabulary",
                    format!("unauthorized clause functor in {id}: {functor}"),
                ));
            }
        }
    }
    Ok(())
}
