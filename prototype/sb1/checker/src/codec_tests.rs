use crate::program::parse_clauses;
use crate::term::Arena;
use crate::{canonical, codec};

const ADULT: &str = include_str!("../../../../tests/consumer/adult.pl.in");
fn accepts(text: &str, parsed: bool) -> bool {
    let mut arena = Arena::default();
    let normalized;
    let source = if parsed {
        text
    } else {
        normalized = ckc_kernel::contract::v1_read(text.as_bytes());
        std::str::from_utf8(&normalized).unwrap()
    };
    let Ok(clauses) = parse_clauses(&mut arena, source, 1) else {
        return false;
    };
    if parsed {
        return codec::document(&arena, &clauses, text);
    }
    if canonical(text.as_bytes()) {
        return true;
    }
    let Some(docid) = clauses
        .iter()
        .find_map(|c| arena.named(c.head, "guideline_document", 4))
        .and_then(|a| arena.atom_name(a[0]))
    else {
        return false;
    };
    let skip = text
        .lines()
        .take_while(|l| l.starts_with('%'))
        .map(|l| l.len() + 1)
        .sum::<usize>();
    let Some(code) = text.get(skip..) else {
        return false;
    };
    canonical(format!("% {docid}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{code}").as_bytes())
}
fn compare(text: &str) {
    assert_eq!(accepts(text, true), accepts(text, false), "{text}");
}

#[test]
fn canonical_image_matches_kernel_on_structural_mutations() {
    let rule = "guideline_exclusion(A) :- guideline_property(actual,A,seed,true), \\+ (guideline_arg(actual,B,1,A), guideline_arg(actual,B,2,blocked)), guideline_satisfies(exactly,3,geq,1).";
    let text = format!("{ADULT}{rule}\n");
    assert!(accepts(&text, true));
    compare(&text);
    let start = ADULT.len();
    let mut cases = 0;
    for at in start..text.len() - 1 {
        let mut deleted = text.clone();
        deleted.remove(at);
        compare(&deleted);
        cases += 1;
        for replacement in *b" _A()',\t" {
            let mut changed = text.as_bytes().to_vec();
            changed[at] = replacement;
            compare(std::str::from_utf8(&changed).unwrap());
            cases += 1;
        }
    }
    for old in [
        ":- multifile(guideline_arg/4).",
        "guideline_schema_version(2).",
        "% S2:",
        "ace_sha256(",
        "ulex(sha256(",
    ] {
        compare(&text.replacen(old, "", 1));
        cases += 1;
    }
    for (from, to) in [
        ("schema_version(2)", "schema_version(1)"),
        ("% S2:", "% S1:"),
        ("% S2:", "% S02:"),
        ("% S2:", "% S999999999999999999999999999999999999999999:"),
        ("ace_sha256('", "ace_sha256('z"),
        ("guideline_document(adult,", "guideline_document('adult',"),
        ("% S2: The patient has an age.", "% S2: "),
    ] {
        compare(&text.replacen(from, to, 1));
        cases += 1;
    }
    assert_eq!(cases, (text.len() - 1 - start) * 9 + 12);
}

#[test]
fn canonical_image_matches_variable_reading_and_term_domains() {
    let prefix = ADULT.split("% S1:").next().unwrap();
    let terms = [
        "a",
        "'[]'",
        "[]",
        "{}",
        "{a}",
        "[a,b]",
        "[a|b]",
        "'Ω'",
        "'x\\n'",
        "rational(-1,2)",
        "f(a)",
        "'$VAR'(1)",
    ];
    for term in terms {
        compare(&format!(
            "{prefix}% S1: Term.\nguideline_evaluation_date({term}).\n"
        ));
    }
    for clause in [
        "guideline_arg(actual,A,_,A).",
        "guideline_arg(actual,_,B,B).",
        "guideline_arg(actual,_,A,A).",
        "guideline_arg(actual,A,_,B).",
        "guideline_arg(actual,_X,A,A).",
        "guideline_arg(actual,B,A,A).",
        "guideline_arg(actual,A,A,[]).",
        "guideline_arg(actual,A,A,0).",
        "guideline_arg(actual,A,A,01).",
        "guideline_exclusion(a) :- \\+ guideline_evaluation_date(1).",
        "guideline_exclusion(a) :- \\+ (guideline_evaluation_date(1)).",
        "guideline_exclusion(a) :- \\+ (guideline_evaluation_date(1), guideline_evaluation_date(2)).",
        "guideline_exclusion(a) :- \\+ (\\+ guideline_evaluation_date(1)).",
        "guideline_exclusion(a) :- ','(guideline_evaluation_date(1),guideline_evaluation_date(2)).",
    ] {
        compare(&format!("{prefix}% S1: Reading.\n{clause}\n"));
    }
    for suffix in ["", "\n", "\r\n", "% extra\n", "% S9: Empty.\n"] {
        compare(&format!("{ADULT}{suffix}"));
    }
    let canonical_header =
        "% adult.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n";
    let code = ADULT
        .lines()
        .skip_while(|l| l.starts_with('%'))
        .collect::<Vec<_>>()
        .join("\n");
    compare(&format!("{canonical_header}{code}\n"));
    compare(&format!("{code}\n"));
}

#[test]
fn canonical_image_matches_every_committed_cdc_document() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../guidelines/cdc-2022-opioid/pl");
    let mut paths = std::fs::read_dir(root)
        .unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "pl"))
        .collect::<Vec<_>>();
    paths.sort();
    assert_eq!(paths.len(), 492);
    for path in paths {
        let text = std::fs::read_to_string(path).unwrap();
        assert!(accepts(&text, true));
        compare(&text);
    }
}

#[test]
fn missing_validation_and_crlf_headers_are_refusals() {
    let bad = format!("{ADULT}guideline_exclusion(A) :- guideline_property(actual,Aseed,true).\n");
    assert!(!accepts(&bad, true));
    compare(&bad);
    let crlf = ADULT.replacen('\n', "\r\n", 1);
    assert!(!accepts(&crlf, true));
    compare(&crlf);
}
