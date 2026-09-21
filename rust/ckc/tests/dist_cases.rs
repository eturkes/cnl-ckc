// M5.6 P3: native replay of tests/dist/red.sh; fixtures + envelopes stay pinned.
use std::path::PathBuf;

// FC2 changes argv only; R44 retains goal: envelopes, cases.tsv stays verbatim.
const COMMAND_MAP: [(&str, &[&str]); 3] = [
    ("tools/dist.py", &["dist"]),
    ("tools/goal.py release-manifest", &["release-manifest"]),
    ("tools/goal.py check", &["check"]),
];

#[test]
fn dist_cases() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rows = std::fs::read_to_string(root.join("tests/dist/cases.tsv")).unwrap();
    assert_eq!(rows.lines().count(), 63);
    assert_eq!(COMMAND_MAP.len(), 3);
    panic!("dist-red: scenario replay seed; pass=0 red=62 total=62");
}
