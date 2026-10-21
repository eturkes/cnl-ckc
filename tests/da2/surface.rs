use super::cases::{self, Case, TABLE, ULEX};
use super::model::{Goal, Product};
use super::support::*;
use super::term::{Term, a, c, numeric_lexemes, numeric_token};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

#[derive(Clone)]
pub struct Observed {
    pub case: Case,
    pub model: Product,
    pub dump: Run,
    pub doc: Run,
    pub proof: Run,
    pub certified: Run,
}
pub fn results() -> &'static BTreeMap<String, Observed> {
    static RESULTS: OnceLock<Result<BTreeMap<String, Observed>, ()>> = OnceLock::new();
    RESULTS
        .get_or_init(|| std::panic::catch_unwind(collect_results).map_err(|_| ()))
        .as_ref()
        .expect("fixture initialization failed; see the first parser/model diagnostic")
}
fn collect_results() -> BTreeMap<String, Observed> {
    let compiler = compiler();
    let control = Case::new(
        "da2-control",
        cases::EXAMPLE.replace(" and Y = X * 1.5", ""),
        "control",
    );
    let work = Scratch::new("oracle-control");
    let ulex = work.0.join("lexicon.ulex");
    write(&ulex, ULEX);
    let dump = compiler.dump(&work.0, control.ace.as_bytes(), Some(&ulex), None);
    dump.require_ok("formula-free parser control");
    let model = Product::from_dump(&control, &dump.stdout);
    for (mode, expected) in [("doc", model.document()), ("proof", model.proof())] {
        let got = compiler.compile(
            &work.0,
            &control.id,
            control.ace.as_bytes(),
            Some(&ulex),
            None,
            mode,
        );
        assert_eq!(got, Run::ok(expected), "formula-free {mode} oracle control");
    }
    let cases = cases::accepted();
    let next = AtomicUsize::new(0);
    let found = Mutex::new(BTreeMap::new());
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let (compiler, cases, next, found) = (&compiler, &cases, &next, &found);
            scope.spawn(move || {
                while let Some(case) = cases.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let work = Scratch::new(&case.id);
                    let ulex = work.0.join("lexicon.ulex");
                    let table = work.0.join("vocabulary.tsv");
                    write(&ulex, ULEX);
                    write(&table, TABLE);
                    let dump = compiler.dump(&work.0, case.ace.as_bytes(), Some(&ulex), None);
                    dump.require_ok(&format!("{} APE dump", case.id));
                    let model = Product::from_dump(case, &dump.stdout);
                    let doc = compiler.compile(
                        &work.0,
                        &case.id,
                        case.ace.as_bytes(),
                        Some(&ulex),
                        case.table.then_some(table.as_path()),
                        "doc",
                    );
                    let proof = compiler.compile(
                        &work.0,
                        &case.id,
                        case.ace.as_bytes(),
                        Some(&ulex),
                        case.table.then_some(table.as_path()),
                        "proof",
                    );
                    // The certifier gets independently projected clauses even when the seed compiler rejects.
                    let certified = certify(
                        &work.0,
                        &case.id,
                        &case.ace,
                        &dump.stdout,
                        &model.document(),
                        case.table(),
                    );
                    found.lock().unwrap().insert(
                        case.id.clone(),
                        Observed {
                            case: case.clone(),
                            model,
                            dump,
                            doc,
                            proof,
                            certified,
                        },
                    );
                }
            });
        }
    });
    found.into_inner().unwrap()
}
fn family(family: &str) {
    let mut checks = Checks::default();
    for obs in results().values().filter(|o| o.case.family == family) {
        checks.output(&obs.doc, &Run::ok(obs.model.document()), &obs.case.id);
    }
    assert!(checks.total > 0, "family {family}");
    checks.finish(&format!("P1/{family}"));
}
#[test]
fn p1_decimal_product() {
    family("product");
}
#[test]
fn p1_quotient() {
    family("quotient");
}
#[test]
fn p1_postorder_chains_and_parentheses() {
    family("chain");
}
#[test]
fn p1_two_operand_links() {
    family("two-operands");
}
#[test]
fn p1_literal_only_formula() {
    family("literal-only");
}
#[test]
fn p1_comparison_determiners_and_list_spines() {
    family("comparison");
}
#[test]
fn p1_curried_domains() {
    family("curried");
}
#[test]
fn p1_horn_shared_operand_and_variant_identities() {
    family("horn");
}
#[test]
fn p1_two_targets_distinct_output_slots() {
    family("two-targets");
}
#[test]
fn p1_target_shared_by_another_head() {
    family("target-shared");
}
#[test]
fn p1_chain_after_antecedent_naf() {
    family("naf");
}
#[test]
fn p1_chain_before_scoped_modal_guard() {
    family("scoped-modal");
}
#[test]
fn p1_unrelated_some_keeps_filter_and_witness() {
    family("unrelated-some");
}
#[test]
fn p1_generated_formula_tree_differential() {
    family("generated");
}
#[test]
fn p2_accepted_source_numeric_correspondence() {
    assert_eq!(
        numeric_lexemes("0 1.5 X1 covid-19 2-week a2 .3 -4 _5 6a 7-thing 8_9"),
        ["0", "1.5"]
    );
    for text in ["X1", "covid-19", "2-week"] {
        assert!(!numeric_token(&a(text)));
    }
    for text in ["0", "12", "-12", "1.5", "1.0e-5"] {
        assert!(numeric_token(&a(text)));
    }
    assert!(numeric_token(&c("$decimal", vec![a("1.5")])));
    family("literal");
}
#[test]
fn p4_nonzero_divisor_witnesses() {
    family("witness");
}
#[test]
fn p2_integer_width_identity_property_exceeds_machine_ranges() {
    let compiler = compiler();
    let work = Scratch::new("integer-width");
    let ulex = work.0.join("lexicon.ulex");
    write(&ulex, ULEX);
    let mut checks = Checks::default();
    for width in [1, 19, 20, 30, 39, 100, 257] {
        let digits: String = (0..width)
            .map(|i| {
                if i == 0 {
                    '9'
                } else {
                    char::from(b'1' + (i % 9) as u8)
                }
            })
            .collect();
        let small = Case::new(
            format!("da2-integer-width-{width}"),
            "If there is an opioid-dosage then the opioid-dosage includes some MME-units-per-day Y and Y = 7 * 1.",
            "width",
        );
        let parsed = compiler.dump(&work.0, small.ace.as_bytes(), Some(&ulex), None);
        parsed.require_ok("integer identity source scene");
        let mut expected = Product::from_dump(&small, &parsed.stdout);
        expected.case.ace = small.ace.replace("7 * 1", &format!("{digits} * 1"));
        let group = &mut expected.groups[0];
        let node = group
            .body
            .iter_mut()
            .find(|g| matches!(g, Goal::Pos(t) if t.name() == "guideline_product"))
            .unwrap();
        let Goal::Pos(Term::Call(_, args)) = node else {
            unreachable!()
        };
        assert_eq!(
            (&args[0], &args[2], &args[3]),
            (&a("exactly"), &a("exactly"), &a(1))
        );
        args[1] = a(&digits);
        let result = args[5].clone();
        // D4 point arithmetic: N × 1 = N, independent of any machine integer width.
        group.witness.insert(result, a(&digits));
        let case = &expected.case;
        let dump = compiler.dump(&work.0, case.ace.as_bytes(), Some(&ulex), None);
        dump.require_ok("wide integer dump");
        let got = compiler.compile(
            &work.0,
            &case.id,
            case.ace.as_bytes(),
            Some(&ulex),
            None,
            "doc",
        );
        checks.output(
            &got,
            &Run::ok(expected.document()),
            &format!("width {width}/clauses"),
        );
        let got = compiler.compile(
            &work.0,
            &case.id,
            case.ace.as_bytes(),
            Some(&ulex),
            None,
            "proof",
        );
        checks.output(
            &got,
            &Run::ok(expected.proof()),
            &format!("width {width}/fork payload"),
        );
        let got = certify(
            &work.0,
            &case.id,
            &case.ace,
            &dump.stdout,
            &expected.document(),
            None,
        );
        checks.output(
            &got,
            &Run::ok(expected.proof()),
            &format!("width {width}/certifier payload"),
        );
    }
    checks.finish("P2/arbitrary-width identity");
}

#[test]
fn p4_three_way_payload_bytes_every_accepted_scene() {
    let mut checks = Checks::default();
    for obs in results().values() {
        let want = Run::ok(obs.model.proof());
        checks.output(&obs.proof, &want, &format!("{}/fork payload", obs.case.id));
        checks.output(
            &obs.certified,
            &want,
            &format!("{}/certifier payload", obs.case.id),
        );
    }
    checks.finish("P4/three-way payload");
}
#[test]
fn p5_certify_independent_document_every_accepted_scene() {
    let mut checks = Checks::default();
    for obs in results().values() {
        checks.output(&obs.certified, &Run::ok(obs.model.proof()), &obs.case.id);
    }
    checks.finish("P5/acceptance");
}

fn rejections(filter: impl Fn(&cases::Reject) -> bool, label: &str) {
    let compiler = compiler();
    let work = Scratch::new(label);
    let ulex = work.0.join("lexicon.ulex");
    let table = work.0.join("vocabulary.tsv");
    write(&ulex, ULEX);
    write(&table, TABLE);
    let mut checks = Checks::default();
    for rejected in cases::rejected().into_iter().filter(filter) {
        let case = &rejected.case;
        let dump = compiler.dump(&work.0, case.ace.as_bytes(), Some(&ulex), None);
        dump.require_ok(&format!("{} rejection syntax", case.id));
        assert!(
            dump.stdout.lines().any(|l| l == "messages([])."),
            "{} rejection must reach formula checks: {}",
            case.id,
            dump.stdout
        );
        let got = compiler.compile(
            &work.0,
            &case.id,
            case.ace.as_bytes(),
            Some(&ulex),
            case.table.then_some(table.as_path()),
            rejected.mode,
        );
        checks.output(
            &got,
            &Run::err(
                1,
                format!(
                    "ace_to_pl_error(unsupported,formula_shape({},1)).\n",
                    rejected.why
                ),
            ),
            &case.id,
        );
        // A nonempty canonical placeholder lets the certifier reach projection before clause equality.
        let placeholder = super::model::document(
            &case.id,
            &case.ace,
            Some(ULEX),
            case.table(),
            "guideline_event(actual,placeholder,wait).\n",
        );
        let got = certify(
            &work.0,
            &case.id,
            &case.ace,
            &dump.stdout,
            &placeholder,
            case.table(),
        );
        checks.output(
            &got,
            &Run::err(
                1,
                format!(
                    "ckc: certify: {}: unsupported(sentence(1,formula_shape({}))).\n",
                    case.id, rejected.why
                ),
            ),
            &format!("{}/certifier", case.id),
        );
    }
    assert!(checks.total > 0);
    checks.finish(label);
}
#[test]
fn p2_spelling_sign_suffix_rejections() {
    rejections(
        |p| p.case.id.starts_with("da2-literal-reject-"),
        "P2/literal-rejections",
    );
}
#[test]
fn p3_reject_classes_and_target_operand_boundaries() {
    rejections(
        |p| {
            !p.case.id.starts_with("da2-order-")
                && !p.case.id.starts_with("da2-literal-reject-")
                && p.why != "witness"
        },
        "P3/rejects",
    );
}
#[test]
fn p3_first_error_order() {
    rejections(|p| p.case.id.starts_with("da2-order-"), "P3/precedence");
}
#[test]
fn p4_zero_divisors_reject_witness() {
    rejections(|p| p.why == "witness", "P4/zero-divisors");
}
