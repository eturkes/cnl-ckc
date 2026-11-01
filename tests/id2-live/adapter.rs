// Contract id2 D4: frozen identifier-1 bytes stay historical; a live expectation or carrier built
// from them takes the forward edit at exactly `sites` line-anchored sites: the document line
// `guideline_schema_version(1).` → `(2).`, the question record opening `'$guideline_query'(v1,`
// → `(v2,`. Every other byte passes through.
pub fn forward(frozen: &str, sites: usize) -> String {
    let mut seen = 0;
    let live: String = frozen
        .split_inclusive('\n')
        .map(|line| {
            if line.trim_end_matches('\n') == "guideline_schema_version(1)." {
                seen += 1;
                line.replacen("(1)", "(2)", 1)
            } else if let Some(rest) = line.strip_prefix("'$guideline_query'(v1,") {
                seen += 1;
                format!("'$guideline_query'(v2,{rest}")
            } else {
                line.to_owned()
            }
        })
        .collect();
    assert_eq!(seen, sites, "id2 D4 identifier sites");
    live
}
