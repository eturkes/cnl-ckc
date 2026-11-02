use super::*;
use crate::term::parse_term;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);
const ADULT: &str = include_str!("../../../../tests/consumer/adult.pl.in");
const QUERY: &str =
    include_str!("../../../../guidelines/cdc-2022-opioid/queries/pl/dosage-reduction-content.pl");

struct Fixture {
    directory: PathBuf,
    request: Request,
}
impl Fixture {
    fn new(clauses: &[&str], derived: &[(&str, usize, &[&str], &str)]) -> Self {
        let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target/fixtures")
            .join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(directory.join("guidelines")).unwrap();
        fs::write(
            directory.join("guidelines/schema.pl"),
            ckc_kernel::contract::schema_file(),
        )
        .unwrap();
        let prefix = ADULT
            .split("% S1:")
            .next()
            .unwrap()
            .lines()
            .filter(|l| !l.starts_with('%'))
            .collect::<Vec<_>>()
            .join("\n");
        let source = format!(
            "% adult.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{prefix}\n% S1: SB1 fixture.\n{}\n",
            clauses.join("\n")
        );
        assert!(
            canonical(source.as_bytes()),
            "noncanonical fixture:\n{source}"
        );
        fs::write(directory.join("document.pl"), source).unwrap();
        fs::write(directory.join("composition.tsv"), "document.pl\n").unwrap();
        let request = Request {
            root: directory.clone(),
            composition: directory.join("composition.tsv"),
            model: directory.join("model.txt"),
            certificate: directory.join("certificate.txt"),
            query: None,
        };
        let loaded = load(&request, &mut Counters::default()).unwrap();
        let mut claims = HashMap::new();
        for c in &loaded.clauses {
            if c.body.is_empty() {
                claims.insert(
                    loaded.arena.render(c.head),
                    (c.ordinal, "[]".to_string(), "0".to_string()),
                );
            }
        }
        for &(fact, ordinal, values, level) in derived {
            let values = values
                .iter()
                .enumerate()
                .map(|(i, t)| format!("=(v{},{t})", i + 1))
                .collect::<Vec<_>>()
                .join(",");
            claims.insert(
                fact.to_string(),
                (ordinal, format!("[{values}]"), level.to_string()),
            );
        }
        let mut keys = claims.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        fs::write(
            &request.model,
            keys.iter().map(|k| format!("{k}.\n")).collect::<String>(),
        )
        .unwrap();
        let mut certificate = format!("sb1-certificate 0\ncomposition {}\n", identity(&loaded));
        for k in keys {
            let (ord, vals, level) = &claims[&k];
            certificate.push_str(&format!("{k}\t1:{ord}\t{vals}\t{level}\n"));
        }
        fs::write(&request.certificate, certificate).unwrap();
        Self { directory, request }
    }
    fn check(&self) -> Outcome {
        let parsed = check(&self.request);
        let kernel = check_with_codec(&self.request, SourceCodec::Kernel);
        assert_eq!(
            (parsed.rc, &parsed.stdout, &parsed.stderr),
            (kernel.rc, &kernel.stdout, &kernel.stderr)
        );
        let original = fs::read_to_string(&self.request.certificate).unwrap();
        if let Some(indexed) = self.indexed_certificate(&original) {
            fs::write(&self.request.certificate, indexed).unwrap();
            for mode in [SourceCodec::Parsed, SourceCodec::Kernel] {
                let next = check_with_codec(&self.request, mode);
                assert_eq!(
                    (parsed.rc, &parsed.stdout, &parsed.stderr),
                    (next.rc, &next.stdout, &next.stderr)
                );
            }
            fs::write(&self.request.certificate, original).unwrap();
        }
        parsed
    }
    fn indexed_certificate(&self, certificate: &str) -> Option<String> {
        let mut lines = certificate.lines();
        if lines.next()? != "sb1-certificate 0" {
            return None;
        }
        let mut out = format!("sb1-certificate 1\n{}\n", lines.next()?);
        let model = fs::read_to_string(&self.request.model).unwrap();
        let positions = model
            .lines()
            .enumerate()
            .map(|(i, f)| (f.strip_suffix('.').unwrap(), i + 1))
            .collect::<HashMap<_, _>>();
        for line in lines {
            let (fact, rest) = line.split_once('\t')?;
            out.push_str(&format!("{}\t{rest}\n", positions.get(fact)?));
        }
        if !certificate.ends_with('\n') {
            out.pop();
        }
        Some(out)
    }
    fn use_indices(&self) {
        let certificate = fs::read_to_string(&self.request.certificate).unwrap();
        fs::write(
            &self.request.certificate,
            self.indexed_certificate(&certificate).unwrap(),
        )
        .unwrap();
    }
    fn query(&mut self, text: &str) {
        let path = self.directory.join("query.pl");
        fs::write(&path, text).unwrap();
        self.request.query = Some(path);
    }
    fn edit_certificate(&self, f: impl FnOnce(String) -> String) {
        fs::write(
            &self.request.certificate,
            f(fs::read_to_string(&self.request.certificate).unwrap()),
        )
        .unwrap();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.directory).unwrap();
    }
}
fn accepted(f: &Fixture) -> Outcome {
    let o = f.check();
    assert_eq!(o.rc, 0, "{}", o.stderr);
    assert!(o.stderr.is_empty());
    o
}
fn refused(f: &Fixture, stage: &str) {
    let o = f.check();
    assert_eq!(o.rc, 1, "{}", o.stderr);
    assert!(o.stdout.is_empty());
    assert!(
        o.stderr.starts_with(&format!("refused {stage} ")),
        "{}",
        o.stderr
    );
    assert_eq!(o.stderr.lines().count(), 1);
}
const SEED: &str = "guideline_property(actual,a,seed,true).";
const P: &str = "guideline_property(actual,a,p,true)";
const Q: &str = "guideline_property(actual,a,q,true)";

#[test]
fn recursion_base_cyclic_alternative_and_nonminimal_levels() {
    let f = Fixture::new(
        &[
            SEED,
            "guideline_property(actual,A,p,true) :- guideline_property(actual,A,seed,true).",
            "guideline_property(actual,A,q,true) :- guideline_property(actual,A,p,true).",
            "guideline_property(actual,A,p,true) :- guideline_property(actual,A,q,true).",
        ],
        &[
            (P, 4, &["a"], "7000000000000000000000000000000000000000000"),
            (Q, 5, &["a"], "9000000000000000000000000000000000000000000"),
        ],
    );
    let o = accepted(&f);
    assert_eq!(o.stdout, "accepted 5 facts\n");
}

#[test]
fn equivalent_founded_supports_both_accepted() {
    let clauses = [
        SEED,
        "guideline_property(actual,a,other,true).",
        "guideline_property(actual,A,p,true) :- guideline_property(actual,A,seed,true).",
        "guideline_property(actual,A,p,true) :- guideline_property(actual,A,other,true).",
    ];
    let a = Fixture::new(&clauses, &[(P, 5, &["a"], "1")]);
    let b = Fixture::new(&clauses, &[(P, 6, &["a"], "80")]);
    assert_eq!(accepted(&a).stdout, accepted(&b).stdout);
}

#[test]
fn correlated_naf_keeps_whole_conjunction_and_existential_locals() {
    let f = Fixture::new(
        &[
            SEED,
            "guideline_arg(actual,e1,1,a).",
            "guideline_arg(actual,e2,2,blocked).",
            "guideline_exclusion(A) :- guideline_property(actual,A,seed,true), \\+ (guideline_arg(actual,B,1,A), guideline_arg(actual,B,2,blocked)).",
        ],
        &[("guideline_exclusion(a)", 6, &["a"], "1")],
    );
    accepted(&f);
}

#[test]
fn violated_correlated_naf_is_claim_refusal() {
    let f = Fixture::new(
        &[
            SEED,
            "guideline_arg(actual,e1,1,a).",
            "guideline_arg(actual,e1,2,blocked).",
            "guideline_exclusion(A) :- guideline_property(actual,A,seed,true), \\+ (guideline_arg(actual,B,1,A), guideline_arg(actual,B,2,blocked)).",
        ],
        &[("guideline_exclusion(a)", 6, &["a"], "1")],
    );
    refused(&f, "claim");
    assert!(f.check().stderr.contains("negation witness"));
}

#[test]
fn naf_builtin_uses_same_local_valuation() {
    let f = Fixture::new(
        &[
            SEED,
            "guideline_cardinality(actual,x,na,exactly,3).",
            "guideline_cardinality(actual,y,na,exactly,9).",
            "guideline_exclusion(A) :- guideline_property(actual,A,seed,true), \\+ (guideline_cardinality(actual,_,na,exactly,C), guideline_satisfies(exactly,C,greater,10)).",
        ],
        &[("guideline_exclusion(a)", 6, &["a"], "1")],
    );
    accepted(&f);
}

#[test]
fn omitted_head_is_closure_refusal() {
    let f = Fixture::new(
        &[
            SEED,
            "guideline_property(actual,A,p,true) :- guideline_property(actual,A,seed,true).",
        ],
        &[],
    );
    refused(&f, "closure");
    assert!(f.check().stderr.contains(P));
}

#[test]
fn unsupported_fact_is_claim_refusal() {
    let f = Fixture::new(&[SEED], &[(P, 3, &[], "0")]);
    refused(&f, "claim");
}

#[test]
fn unfounded_cycle_is_claim_refusal() {
    let f = Fixture::new(
        &[
            "guideline_property(actual,A,p,true) :- guideline_property(actual,A,q,true).",
            "guideline_property(actual,A,q,true) :- guideline_property(actual,A,p,true).",
        ],
        &[(P, 3, &["a"], "1"), (Q, 4, &["a"], "2")],
    );
    refused(&f, "claim");
    assert!(f.check().stderr.contains("nondecreasing"));
}

#[test]
fn nondecreasing_level_is_claim_refusal() {
    let f = Fixture::new(
        &[
            SEED,
            "guideline_property(actual,A,p,true) :- guideline_property(actual,A,seed,true).",
            "guideline_property(actual,A,q,true) :- guideline_property(actual,A,p,true).",
        ],
        &[(P, 4, &["a"], "7"), (Q, 5, &["a"], "7")],
    );
    refused(&f, "claim");
    assert!(f.check().stderr.contains("nondecreasing"));
}

#[test]
fn wrong_valuation_is_claim_refusal() {
    let f = Fixture::new(
        &[
            SEED,
            "guideline_exclusion(A) :- guideline_property(actual,A,seed,true).",
        ],
        &[("guideline_exclusion(a)", 4, &["b"], "1")],
    );
    refused(&f, "claim");
}

#[test]
fn wrong_builtin_is_claim_refusal() {
    let f = Fixture::new(
        &["guideline_exclusion(a) :- guideline_satisfies(exactly,3,geq,4)."],
        &[("guideline_exclusion(a)", 3, &[], "1")],
    );
    refused(&f, "claim");
    assert!(f.check().stderr.contains("built-in"));
}

#[test]
fn arithmetic_output_is_generated_in_closure_and_checked_in_claim() {
    let clause = "guideline_property(actual,a,amount,A) :- guideline_product(exactly,3,exactly,2,exactly,A).";
    let good = Fixture::new(
        &[clause],
        &[("guideline_property(actual,a,amount,6)", 3, &["6"], "9")],
    );
    accepted(&good);
    let wrong = Fixture::new(
        &[clause],
        &[("guideline_property(actual,a,amount,7)", 3, &["7"], "9")],
    );
    refused(&wrong, "claim");
}

#[test]
fn truncated_certificate_is_envelope_refusal() {
    let f = Fixture::new(&[SEED], &[]);
    f.edit_certificate(|mut s| {
        s.pop();
        s
    });
    refused(&f, "envelope");
}

#[test]
fn duplicate_and_missing_claims_are_claim_refusals() {
    let duplicate = Fixture::new(&[SEED], &[]);
    duplicate.edit_certificate(|s| format!("{s}{}\n", s.lines().last().unwrap()));
    refused(&duplicate, "claim");
    let missing = Fixture::new(&[SEED], &[]);
    missing.edit_certificate(|s| {
        s.lines()
            .take(s.lines().count() - 1)
            .map(|s| format!("{s}\n"))
            .collect()
    });
    refused(&missing, "claim");
}

#[test]
fn envelope_precedes_identity_and_identity_precedes_claim() {
    let f = Fixture::new(&[SEED], &[]);
    f.edit_certificate(|s| s.replace("sb1-certificate 0", "sb1-certificate 1"));
    refused(&f, "envelope");
    let f = Fixture::new(&[SEED], &[]);
    fs::write(f.request.root.join("guidelines/schema.pl"), "changed\n").unwrap();
    refused(&f, "identity");
}

#[test]
fn q_all_is_canonical_sorted_payload_output() {
    let mut f = Fixture::new(
        &[
            "guideline_operator(actual,z,may).",
            "guideline_operator(actual,a,should).",
            "guideline_property(z,patient,risk,high).",
            "guideline_event(a,event,advise).",
        ],
        &[],
    );
    f.query("sb1-q-all 0\n");
    assert_eq!(
        accepted(&f).stdout,
        "box(a,should,[guideline_event(a,event,advise)]).\nbox(z,may,[guideline_property(z,patient,risk,high)]).\n"
    );
}

#[test]
fn query_projects_sorts_deduplicates_and_binds_digest() {
    let mut f = Fixture::new(
        &[
            "guideline_property(actual,z,answer,true).",
            "guideline_property(actual,a,answer,true).",
        ],
        &[],
    );
    let header = QUERY
        .lines()
        .find(|l| l.starts_with("'$guideline_query'("))
        .unwrap()
        .replace("'dosage-reduction-content'", "tiny");
    let text = format!(
        "% tiny compiled from ACE question by ace_to_pl question mode; do not edit.\n{header}\n% Q1: SB1 fixture?\n'$guideline_query_projection'(goal(guideline_property(actual,A,answer,true)),answers([answer(A,wh(what))])).\n"
    );
    f.query(&text);
    let o = accepted(&f);
    let hash = term::quote_atom(&sha256(text.as_bytes()));
    assert_eq!(
        o.stdout,
        format!(
            "% tiny answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,tiny,query_sha256({hash}),result(solutions([sol([a]),sol([z])]))).\n"
        )
    );
}

#[test]
fn metrics_expose_all_phases_and_separate_counter_kinds() {
    let f = Fixture::new(&[SEED], &[]);
    let o = accepted(&f);
    o.write_metrics(&f.directory).unwrap();
    let phases = fs::read_to_string(f.directory.join("checker-phases.tsv")).unwrap();
    assert_eq!(phases.lines().count(), 8);
    assert!(
        o.phases
            .iter()
            .all(|p| p.executed && p.seconds >= 0.0 && p.cpu >= 0.0)
    );
    let counters = fs::read_to_string(f.directory.join("checker-counters.tsv")).unwrap();
    assert!(counters.contains("logical\tmodel_facts\t3\n"));
    assert!(counters.contains("runtime\tinterning_requests\t"));
}

#[test]
fn canonical_term_roundtrip_and_standard_order_are_not_lexical_sort() {
    let mut a = Arena::default();
    let strings = [
        "'$guideline_id'(product,'Ω',1,ref(2),['x\\n',rational(-1,2)])",
        "'\\u0000'",
        "[]",
        "{}",
        ";",
        "-17",
        "1000",
    ];
    for s in strings {
        let id = parse_term(&mut a, s).unwrap();
        assert_eq!(a.render(id), s);
    }
    let z = parse_term(&mut a, "x(z)").unwrap();
    let aa = parse_term(&mut a, "a(a,a)").unwrap();
    assert_eq!(a.compare(z, aa), std::cmp::Ordering::Less);
    let ten = parse_term(&mut a, "10").unwrap();
    let two = parse_term(&mut a, "2").unwrap();
    assert_eq!(a.compare(ten, two), std::cmp::Ordering::Greater);
}

#[test]
fn nested_identity_dependencies_are_interned_matched_and_instantiated() {
    let base = "'$guideline_id'(product,adult,1,ref(1),[])";
    let derived = format!(
        "guideline_entity(actual,'$guideline_id'(product,adult,2,ref(1),[{base}]),derived,countable)"
    );
    let source = format!("guideline_entity(actual,{base},base,countable).");
    let f = Fixture::new(
        &[
            &source,
            "guideline_entity(actual,'$guideline_id'(product,adult,2,ref(1),[A]),derived,countable) :- guideline_entity(actual,A,base,countable).",
        ],
        &[(&derived, 4, &[base], "1")],
    );
    accepted(&f);
}

#[test]
fn closure_enumerates_valuations_absent_from_the_certificate() {
    let f = Fixture::new(
        &[
            SEED,
            "guideline_property(actual,b,seed,true).",
            "guideline_property(actual,A,p,true) :- guideline_property(actual,A,seed,true).",
        ],
        &[(P, 5, &["a"], "1")],
    );
    refused(&f, "closure");
    assert!(
        f.check()
            .stderr
            .contains("guideline_property(actual,b,p,true)")
    );
}

#[test]
fn indexed_claims_keep_content_and_reject_bad_ranges_duplicates_and_gaps() {
    let good = Fixture::new(&[SEED], &[]);
    good.use_indices();
    accepted(&good);
    for index in ["0", "4", "18446744073709551616000000000000000"] {
        let bad = Fixture::new(&[SEED], &[]);
        bad.use_indices();
        bad.edit_certificate(|s| {
            let mut lines = s.lines().map(str::to_string).collect::<Vec<_>>();
            let (_, rest) = lines[2].split_once('\t').unwrap();
            lines[2] = format!("{index}\t{rest}");
            lines.join("\n") + "\n"
        });
        refused(&bad, "claim");
        assert!(bad.check().stderr.contains("out of range"));
    }
    let duplicate = Fixture::new(&[SEED], &[]);
    duplicate.use_indices();
    duplicate.edit_certificate(|s| format!("{s}{}\n", s.lines().last().unwrap()));
    refused(&duplicate, "claim");
    let missing = Fixture::new(&[SEED], &[]);
    missing.use_indices();
    missing.edit_certificate(|s| {
        s.lines()
            .take(s.lines().count() - 1)
            .map(|l| format!("{l}\n"))
            .collect()
    });
    refused(&missing, "claim");
}

#[test]
fn noncanonical_model_and_valuation_keep_their_failure_stages() {
    let f = Fixture::new(&[SEED], &[]);
    f.use_indices();
    let model = fs::read_to_string(&f.request.model).unwrap();
    fs::write(&f.request.model, model.replace("actual,", "actual, ")).unwrap();
    let out = f.check();
    assert_eq!(out.rc, 2);
    assert!(out.stderr.starts_with("parse noncanonical model line "));
    let f = Fixture::new(
        &[
            SEED,
            "guideline_property(actual,A,p,true) :- guideline_property(actual,A,seed,true).",
        ],
        &[(P, 4, &["a"], "1")],
    );
    f.use_indices();
    f.edit_certificate(|s| s.replace("[=(v1,a)]", "[=('v1',a)]"));
    refused(&f, "claim");
    assert!(f.check().stderr.contains("noncanonical valuation"));
}

#[test]
fn model_cr_bytes_refuse_before_line_normalization() {
    let f = Fixture::new(&[SEED], &[]);
    let model = fs::read_to_string(&f.request.model).unwrap();
    fs::write(&f.request.model, model.replace('\n', "\r\n")).unwrap();
    let out = f.check();
    assert_eq!(out.rc, 2);
    assert!(out.stdout.is_empty());
    assert_eq!(out.stderr, "parse non-LF model\n");
}
