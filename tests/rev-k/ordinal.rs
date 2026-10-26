use super::{Scratch, certify, read, root, sha, write};

const NOUNS: [&str; 5] = ["patient", "doctor", "nurse", "man", "woman"];

fn scene(nouns: &[&str], mask: usize) -> (String, String, String, String) {
    let n = nouns.len();
    let branches: Vec<_> = nouns
        .iter()
        .map(|noun| format!("there is a {noun}"))
        .collect();
    let ace = format!("If {} then there is a person.\n", branches.join(" or "));
    let mut variables = b'A'..=b'Z';
    let mut parts = Vec::new();
    let mut clauses = String::new();
    let mut proofs = String::new();
    for (i, noun) in nouns.iter().enumerate() {
        let entity = char::from(variables.next().unwrap());
        let nonground = mask & (1 << i) != 0;
        let amount = if nonground {
            char::from(variables.next().unwrap()).to_string()
        } else {
            "1".into()
        };
        parts.push(format!(
            "drs([{entity}],[-(object({entity},{noun},countable,na,eq,{amount}),/(1,{}))])",
            5 * (i + 1)
        ));
        let product = format!("'$guideline_id'(product,'fx-doc',1,ref({}),[A])", n + 1);
        let count = if nonground { "B" } else { "1" };
        let body = format!(
            "guideline_entity(actual,A,{noun},countable), guideline_cardinality(actual,A,na,eq,{count})"
        );
        clauses.push_str(&format!(
            "guideline_entity(actual,{product},person,countable) :- {body}.\n"
        ));
        clauses.push_str(&format!(
            "guideline_cardinality(actual,{product},na,eq,1) :- {body}.\n"
        ));
        let k = i + 1;
        let witness = format!("'$guideline_id'(witness,'fx-doc',1,ref({k}),variant({k}))");
        let product = format!(
            "'$guideline_id'(product,'fx-doc',1,ref({}),[{witness}])",
            n + 1
        );
        proofs.push_str(&format!("'$guideline_proof'('fx-doc',1,variant({k}),witness([guideline_entity(actual,{witness},{noun},countable),guideline_cardinality(actual,{witness},na,eq,1)]),prove([guideline_entity(actual,{product},person,countable),guideline_cardinality(actual,{product},na,eq,1)])).\n"));
    }
    let mut alternatives = parts.pop().unwrap();
    while let Some(left) = parts.pop() {
        alternatives = format!("drs([],[v({left},{alternatives})])");
    }
    let person = char::from(variables.next().unwrap());
    let words = ace
        .trim()
        .trim_end_matches('.')
        .split_whitespace()
        .map(|word| if word == "If" { "'If'" } else { word })
        .collect::<Vec<_>>()
        .join(",");
    let dump = format!(
        "sentences([[^,{words},'.']]).\ndrs(drs([],[=>({alternatives},drs([{person}],[-(object({person},person,countable,na,eq,1),/(1,{}))]))])).\nmessages([]).\n",
        5 * n + 5
    );
    let base = read(&root().join("tests/certify-one/doc/pl.txt.in"));
    let header = base.split_once("% S1:").unwrap().0;
    let old_digest = sha(read(&root().join("tests/certify-one/doc/ace.txt.in")).as_bytes());
    let digest = sha(ace.as_bytes());
    let atom = if digest.as_bytes()[0].is_ascii_digit() {
        format!("'{digest}'")
    } else {
        digest
    };
    let header = header.replace(
        &format!("ace_sha256('{old_digest}')"),
        &format!("ace_sha256({atom})"),
    );
    let pl = format!("{header}% S1: {ace}{clauses}");
    (ace, dump, pl, proofs)
}

#[test]
fn p3_variant_selection_property() {
    let s = Scratch::new("ordinal-property");
    let ace_path = s.0.join("ace.in");
    let dump_path = s.0.join("dump.in");
    let pl_path = s.0.join("pl.in");
    let mut failures = Vec::new();
    let mut total = 0;
    let mut shapes = Vec::new();
    for a in NOUNS {
        shapes.push(vec![a]);
        for b in NOUNS {
            if a != b {
                shapes.push(vec![a, b]);
            }
        }
    }
    for nouns in shapes {
        for mask in 0..(1 << nouns.len()) {
            let (ace, dump, pl, proof) = scene(&nouns, mask);
            write(&ace_path, &ace);
            write(&dump_path, &dump);
            write(&pl_path, &pl);
            let out = certify(&ace_path, &dump_path, &pl_path);
            let expected = if mask == 0 {
                (Some(0), proof, String::new())
            } else {
                let k = mask.trailing_zeros() + 1;
                (
                    Some(1),
                    String::new(),
                    format!("ckc: certify: fx-doc: nonground_obligation(1,variant({k})).\n"),
                )
            };
            let actual = (
                out.status.code(),
                String::from_utf8(out.stdout).unwrap(),
                String::from_utf8(out.stderr).unwrap(),
            );
            if actual != expected {
                failures.push(format!(
                    "nouns={nouns:?} mask={mask:b}: expected {expected:?}; got {actual:?}"
                ));
            }
            total += 1;
        }
    }
    assert_eq!(total, 90);
    assert!(
        failures.is_empty(),
        "{} / {total} ordinal-model cases failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
