// Contract c1 P10: the Kani harness constants (rust/ckc-kani-harness/src/lib.rs) stay
// consistent between `just kani` runs: DOC + QUERIES are accepted schema files, each
// QUERY_SHA256 = sha256 of its query, ANSWERS cites QUERIES[0]'s digest, the answers
// digest = sha256(ANSWERS), the symbolic goal byte is the one byte the two queries
// differ in, and the DOC-bounded unwinds (+ their comments) follow DOC.len().
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

fn source() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../ckc-kani-harness/src/lib.rs");
    fs::read_to_string(path).unwrap()
}

fn hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

// The bytes of the `nth` `concat!(` literal run after `opener`: one `"…",` per line,
// whose only escapes are `\n`, `\\` and `\"`.
fn concat(src: &str, opener: &str, nth: usize) -> Vec<u8> {
    let rest = &src[src.find(opener).unwrap()..];
    let block = rest.split("concat!(\n").nth(nth + 1).unwrap();
    let mut out = Vec::new();
    for line in block.lines() {
        let line = line.trim();
        let Some(lit) = line.strip_prefix('"').and_then(|l| l.strip_suffix("\",")) else {
            break;
        };
        let mut chars = lit.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next().unwrap() {
                    'n' => out.push(b'\n'),
                    other => out.extend(other.to_string().bytes()),
                }
            } else {
                out.extend(c.to_string().bytes());
            }
        }
    }
    out
}

// The 64-hex byte-string literals of `const QUERY_SHA256`, then the one after `&answers,`.
fn digest_literals(src: &str, after: &str, count: usize) -> Vec<String> {
    src[src.find(after).unwrap()..]
        .split("b\"")
        .skip(1)
        .take(count)
        .map(|s| s[..64].to_owned())
        .collect()
}

fn atom(hexdigest: &str) -> String {
    if hexdigest.starts_with(|c: char| c.is_ascii_digit()) {
        format!("'{hexdigest}'")
    } else {
        hexdigest.to_owned()
    }
}

#[test]
fn kani_harness_constants_agree() {
    let src = source();
    let doc = concat(&src, "const DOC", 0);
    let queries = [
        concat(&src, "const QUERIES", 0),
        concat(&src, "const QUERIES", 1),
    ];
    let answers = concat(&src, "const ANSWERS", 0);
    for (label, file) in [
        ("DOC", &doc),
        ("QUERIES[0]", &queries[0]),
        ("QUERIES[1]", &queries[1]),
    ] {
        assert!(
            matches!(
                ckc_kernel::contract::v1_check(file),
                ckc_kernel::EV1Verdict::Ok
            ),
            "{label} is no accepted schema file"
        );
    }
    let qsha = digest_literals(&src, "const QUERY_SHA256", 2);
    for (i, query) in queries.iter().enumerate() {
        assert_eq!(qsha[i], hex(query), "QUERY_SHA256[{i}]");
    }
    let cite = format!("query_sha256({})", atom(&qsha[0]));
    assert!(
        String::from_utf8_lossy(&answers).contains(&cite),
        "ANSWERS cites {cite}"
    );
    assert_eq!(
        digest_literals(&src, "&answers,", 1)[0],
        hex(&answers),
        "answers digest"
    );
    let differ: Vec<usize> = (0..queries[0].len())
        .filter(|&i| queries[0][i] != queries[1][i])
        .collect();
    assert_eq!(queries[0].len(), queries[1].len(), "query lengths");
    assert_eq!(differ.len(), 1, "queries differ in one byte");
    let at = differ[0];
    assert_eq!((queries[0][at], queries[1][at]), (b'b', b'd'), "goal byte");
    assert!(
        src.contains(&format!("query[{at}] = b'b' + 2 * variant;")),
        "goal byte offset {at}"
    );
    let bound = doc.len() + 1;
    assert!(queries.iter().all(|q| q.len() < bound) && answers.len() < bound);
    for name in ["reader_trace_check_small", "engine_answer_tiny"] {
        assert!(
            src.contains(&format!("#[kani::unwind({bound})]\n    fn {name}()")),
            "{name} unwind = DOC.len() + 1 = {bound}"
        );
    }
    let n = doc.len();
    assert!(
        src.contains(&format!("DOC has {n} bytes")),
        "comment DOC has {n} bytes"
    );
    assert!(
        src.contains(&format!("DOC.len() = {n}; bound = {n}")),
        "comment DOC.len() = {n}"
    );
}
