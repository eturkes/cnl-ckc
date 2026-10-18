use super::{Scratch, root};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DOC: &str = include_str!("../v1/eso-c-001/doc.pl.in");
const QUERY: &str = include_str!("../v1/eso-c-001/query.pl.in");
const RULE: &str = "guideline_event(actual,token,answer) :- guideline_operator(actual,A,may), guideline_operator(A,B,can), guideline_entity(B,witness,q,countable).\n";

fn digest(bytes: &[u8]) -> String {
    let hex: String = Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    if hex.as_bytes()[0].is_ascii_lowercase() {
        hex
    } else {
        format!("'{hex}'")
    }
}

fn context(i: usize) -> String {
    format!("'$guideline_id'(context,doc,1,box({i}),[])")
}

struct Facts {
    outer: Vec<usize>,
    edges: Vec<(usize, usize)>,
    content: Vec<usize>,
}

impl Facts {
    fn new(outer: usize, edges: usize, content: usize, order: usize) -> Self {
        let mut facts = Self {
            outer: (0..2).filter(|i| outer & (1 << i) != 0).collect(),
            edges: (0..4)
                .filter(|i| edges & (1 << i) != 0)
                .map(|i| (i / 2, i % 2))
                .collect(),
            content: (0..2).filter(|i| content & (1 << i) != 0).collect(),
        };
        if order & 1 != 0 {
            facts.outer.reverse();
        }
        if order & 2 != 0 {
            facts.edges.reverse();
        }
        if order & 4 != 0 {
            facts.content.reverse();
        }
        facts
    }

    fn outer_line(i: usize) -> String {
        format!("guideline_operator(actual,{},may).\n", context(i + 1))
    }

    fn edge_line(o: usize, i: usize) -> String {
        format!(
            "guideline_operator({},{},can).\n",
            context(o + 1),
            context(i + 3)
        )
    }

    fn content_line(i: usize) -> String {
        format!(
            "guideline_entity({},witness,q,countable).\n",
            context(i + 3)
        )
    }

    fn document(&self, rule: bool) -> String {
        let mut doc = DOC.split_once("% S1:").unwrap().0.to_owned();
        doc.push_str("% S1: ESO generated finite-domain probe.\n");
        doc.push_str("guideline_property(actual,token,irrelevant,pos).\n");
        if rule {
            doc.push_str(RULE);
        }
        for &i in &self.outer {
            doc.push_str(&Self::outer_line(i));
        }
        for &(o, i) in &self.edges {
            doc.push_str(&Self::edge_line(o, i));
        }
        for &i in &self.content {
            doc.push_str(&Self::content_line(i));
        }
        doc
    }

    // Relational join, not a search interpreter: all 2×2 edge subsets are finite.
    fn solutions(&self) -> Vec<usize> {
        (0..2)
            .filter(|i| {
                self.content.contains(i)
                    && self
                        .edges
                        .iter()
                        .any(|(o, j)| j == i && self.outer.contains(o))
            })
            .collect()
    }

    // D1-D4: content binds the inner box, its edge binds the outer box, then
    // the root edge runs. Forest positions retain the question/body order.
    fn first_proof(&self, rule: bool) -> String {
        for &i in &self.content {
            for &(o, j) in &self.edges {
                if j == i && self.outer.contains(&o) {
                    let nodes = [
                        node(&Self::outer_line(o), ""),
                        node(&Self::edge_line(o, i), ""),
                        node(&Self::content_line(i), ""),
                    ]
                    .join(",");
                    return format!(
                        "proved([{}])",
                        if rule { node(RULE, &nodes) } else { nodes }
                    );
                }
            }
        }
        "unproved(finite_failure)".into()
    }
}

fn node(line: &str, children: &str) -> String {
    format!(
        "clause(sentence(doc,1),clause_sha256({}),[{children}])",
        digest(line.as_bytes())
    )
}

fn query(wh: bool, left: bool, rule: bool) -> String {
    let prefix = QUERY.split_once("'$guideline_query_projection'").unwrap().0;
    let outer = "guideline_operator(actual,A,may)";
    let edge = "guideline_operator(A,B,can)";
    let content = "guideline_entity(B,witness,q,countable)";
    let goal = if rule {
        "guideline_event(actual,token,answer)".into()
    } else if left {
        format!("','(','({outer},{edge}),{content})")
    } else {
        format!("','({outer},','({edge},{content}))")
    };
    let answers = if wh { "answer(B,wh(what))" } else { "" };
    format!("{prefix}'$guideline_query_projection'(goal({goal}),answers([{answers}])).\n")
}

fn answer(query: &str, result: &str) -> String {
    format!(
        "% q answered against the loaded composition by ckc queries; do not edit.\n\
         '$guideline_answers'(v1,q,query_sha256({}),result({result})).\n",
        digest(query.as_bytes())
    )
}

fn trace(query: &str, answers: &str, proof: &str) -> String {
    format!(
        "% q traced against the loaded composition by ckc queries; do not edit.\n\
         '$guideline_traces'(v1,q,query_sha256({}),answers_sha256({}),result(yes({proof}))).\n",
        digest(query.as_bytes()),
        digest(answers.as_bytes())
    )
}

fn program() -> PathBuf {
    std::env::var_os("CKC_ESO_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")))
}

fn run(program: &Path, dir: &Path, mode: &str, files: &[&str]) -> Vec<u8> {
    let output = Command::new(program)
        .args(["v1", mode])
        .args(files)
        .current_dir(dir)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{mode}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty(), "{:?}", output.stderr);
    output.stdout
}

fn scratch(name: &str) -> Scratch {
    let dir = root().join(format!(
        "rust/target/eso-properties/{}/{name}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("manifest.tsv"), "doc.pl\tdoc.pl\n").unwrap();
    Scratch(dir)
}

#[test]
fn finite_ground_compositions_match_relational_oracle() {
    let program = program();
    let scratch = scratch("answers");
    let mut checked = 0;
    for outer in 0..4 {
        for edges in 0..16 {
            for content in 0..4 {
                let id = (outer * 16 + edges) * 4 + content;
                let facts = Facts::new(outer, edges, content, id % 8);
                let query = query(true, id % 2 == 0, false);
                fs::write(scratch.0.join("doc.pl"), facts.document(false)).unwrap();
                fs::write(scratch.0.join("query.pl"), &query).unwrap();
                assert!(run(&program, &scratch.0, "check", &["doc.pl"]).is_empty());
                assert!(run(&program, &scratch.0, "check", &["query.pl"]).is_empty());
                let solutions = facts
                    .solutions()
                    .iter()
                    .map(|i| format!("sol([{}])", context(i + 3)))
                    .collect::<Vec<_>>()
                    .join(",");
                let expected = answer(&query, &format!("solutions([{solutions}])"));
                assert_eq!(
                    run(
                        &program,
                        &scratch.0,
                        "answer",
                        &["manifest.tsv", "query.pl"]
                    ),
                    expected.as_bytes(),
                    "outer={outer} edges={edges} content={content} order={}",
                    id % 8
                );
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 256);
}

#[test]
fn generated_first_proofs_follow_content_order() {
    let program = program();
    let scratch = scratch("traces");
    let mut failures = Vec::new();
    let mut checked = 0;
    for edges in 0..16 {
        for order in 0..8 {
            let rule = order % 2 == 0;
            let facts = Facts::new(3, edges, 3, order);
            let query = query(false, order % 3 == 0, rule);
            let answers = answer(&query, "yes");
            let expected = trace(&query, &answers, &facts.first_proof(rule));
            fs::write(scratch.0.join("doc.pl"), facts.document(rule)).unwrap();
            fs::write(scratch.0.join("query.pl"), &query).unwrap();
            fs::write(scratch.0.join("answers.pl"), &answers).unwrap();
            assert!(run(&program, &scratch.0, "check", &["doc.pl"]).is_empty());
            assert!(run(&program, &scratch.0, "check", &["query.pl"]).is_empty());
            let actual = run(
                &program,
                &scratch.0,
                "trace",
                &["manifest.tsv", "query.pl", "answers.pl"],
            );
            if actual != expected.as_bytes() {
                failures.push(format!(
                    "edges={edges} order={order} rule={rule}\nactual={}expected={expected}",
                    String::from_utf8_lossy(&actual)
                ));
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 128);
    assert!(
        failures.is_empty(),
        "{} of {checked} generated traces differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
