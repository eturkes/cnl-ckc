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
// the two temporal annotations; q12 D2: a v3 table adds four more.
const V2_FUNCTORS: [&str; 2] = ["guideline_interval", "guideline_recurrence"];
const V2_INDICATORS: [&str; 3] = [
    "guideline_document/4",
    "guideline_interval/6",
    "guideline_recurrence/4",
];
const V3_FUNCTORS: [&str; 4] = [
    "guideline_frequency",
    "guideline_order",
    "guideline_recurrence_window",
    "guideline_range",
];
const V3_INDICATORS: [&str; 4] = [
    "guideline_frequency/5",
    "guideline_order/4",
    "guideline_recurrence_window/7",
    "guideline_range/3",
];
pub(super) fn check(root: &Path, id: &str) -> Result {
    let table = root.join("temporal.tsv");
    let v2 = table.is_file();
    // Any table admits the v3 vocabulary: the codec decides the exact version, so a
    // malformed header reaches its own `temporal` violation first.
    let v3 = v2;
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
            if !INDICATORS.contains(&indicator)
                && !(v2 && V2_INDICATORS.contains(&indicator))
                && !(v3 && V3_INDICATORS.contains(&indicator))
            {
                return Err(violation(
                    "product-vocabulary",
                    format!("undeclared indicator in {id}: {indicator}"),
                ));
            }
        } else {
            let functor = line.split('(').next().unwrap_or("");
            if !FUNCTORS.contains(&functor)
                && !(v2 && V2_FUNCTORS.contains(&functor))
                && !(v3 && V3_FUNCTORS.contains(&functor))
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
