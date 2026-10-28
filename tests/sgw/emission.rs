use super::model::*;
use super::support::*;
use std::fs;

const ULEX: &str = "noun_sg('sgw-dose','sgw-dose',neutr).\nnoun_pl('sgw-doses','sgw-dose',neutr).\nnoun_sg('sgw-input-unit','sgw-input-unit',neutr).\nnoun_pl('sgw-input-units','sgw-input-unit',neutr).\nnoun_sg('sgw-output-unit','sgw-output-unit',neutr).\nnoun_pl('sgw-output-units','sgw-output-unit',neutr).\n";

// Source scene: one dose, one input quantity + contain event; each consequent adds one
// result quantity + include event. Distinct formula outputs, source order, DRS ref slots.
fn formula_scene(n: usize) -> (String, String) {
    let mut ace = "If an sgw-dose contains some sgw-input-units X then ".to_owned();
    ace.push_str(
        &(0..n)
            .map(|i| {
                format!(
                    "the sgw-dose includes some sgw-output-units Y{} and Y{} = X * {}",
                    i + 1,
                    i + 1,
                    i + 2
                )
            })
            .collect::<Vec<_>>()
            .join(" and "),
    );
    ace.push_str(".\n");
    let mut body = vec![
        "guideline_entity(actual,Dose,'sgw-dose',countable)".to_owned(),
        "guideline_cardinality(actual,Dose,na,eq,1)".to_owned(),
        "guideline_entity(actual,Input,'sgw-input-unit',countable)".to_owned(),
        "guideline_cardinality(actual,Input,na,InKind,InValue)".to_owned(),
        "guideline_event(actual,Contain,contain)".to_owned(),
        "guideline_arg(actual,Contain,1,Dose)".to_owned(),
        "guideline_arg(actual,Contain,2,Input)".to_owned(),
    ];
    for i in 0..n {
        body.push(format!(
            "guideline_product(InKind,InValue,exactly,{},OutKind{i},OutValue{i})",
            i + 2
        ));
    }
    let body = body.join(", ");
    let mut clauses = String::new();
    for i in 0..n {
        let id = |slot| {
            format!("'$guideline_id'(product,'sgw-formula',1,ref({slot}),[Dose,Input,Contain])")
        };
        let quantity = id(4 + 2 * i);
        let event = id(5 + 2 * i);
        for head in [
            format!("guideline_entity(actual,{quantity},'sgw-output-unit',countable)"),
            format!("guideline_cardinality(actual,{quantity},na,OutKind{i},OutValue{i})"),
            format!("guideline_event(actual,{event},include)"),
            format!("guideline_arg(actual,{event},1,Dose)"),
            format!("guideline_arg(actual,{event},2,{quantity})"),
        ] {
            clauses.push_str(&number_letters(&format!("{head} :- {body}.\n")));
        }
    }
    (ace, clauses)
}

#[test]
fn p2_generated_formula_emission_and_certification() {
    let compiler = Compiler::new();
    let stage_path = compiler.scratch.0.clone();
    let s = Scratch::new("formula");
    let ulex = s.file("lexicon.ulex", ULEX);
    let mut checks = Checks::default();
    for n in [1, 2, 4, 13, 14] {
        let (ace, clauses) = formula_scene(n);
        let letter = document("sgw-formula", &ace, Some(ULEX), &clauses);
        let anonymous = anonymize(letter.as_bytes());
        assert_eq!(
            read(&anonymous).text,
            letter.as_bytes(),
            "source model round-trip n={n}"
        );
        assert!(singletons(letter.as_bytes()) >= 2 * n);
        if n >= 2 {
            assert!(
                clauses.lines().any(|l| {
                    let counts = letter_counts(l.as_bytes());
                    let spans = runs(l.as_bytes());
                    spans.iter().enumerate().any(|(i, span)| {
                        counts.get(&l.as_bytes()[span.clone()]) == Some(&1)
                            && spans[i + 1..].iter().any(|s| {
                                counts.get(&l.as_bytes()[s.clone()]).is_some_and(|&n| n > 1)
                            })
                    })
                }),
                "source model has a nonterminal gap n={n}"
            );
        }
        if n >= 13 {
            assert!(
                clauses.lines().any(|l| {
                    let counts = letter_counts(l.as_bytes());
                    counts.get(b"Z".as_slice()) == Some(&1)
                        && counts.get(b"A1".as_slice()) == Some(&1)
                }),
                "source model spans singleton Z/A1 boundary"
            );
        }
        let got = compiler.compile(&s.0, "sgw-formula", &ace, Some(&ulex), None, "doc");
        got.require(&format!("ACE formula n={n}"));
        checks.bytes(
            &got.out,
            &anonymous,
            &format!("formula n={n} emitted bytes"),
        );
        checks.require(
            emission_ok(&got.out, letter.as_bytes()),
            format!("formula n={n}: no named singleton + independent clause identity"),
        );
        let dump = compiler.dump(&s.0, &ace, &ulex);
        dump.require("formula DRS syntax");
        s.file("input.ace", &ace);
        s.file("input.dump", &dump.out);
        s.file("letter.pl", &letter);
        s.file("anonymous.pl", &anonymous);
        // The unchanged certificate validates the independently constructed source model too;
        // expected clause bytes never come from the emitter or the certifier.
        for name in ["letter.pl", "anonymous.pl"] {
            let certified = native(
                &s.0,
                &[
                    "certify-one",
                    "doc",
                    "sgw-formula",
                    "input.ace",
                    "lexicon.ulex",
                    "input.dump",
                    name,
                ],
            );
            checks.require(
                certified.clean(),
                format!("formula n={n} certify {name}: {}", certified.description()),
            );
        }
    }
    drop(compiler);
    assert!(
        !stage_path.exists(),
        "compiler stage removed after last use"
    );
    checks.finish("P2 formula scenes: middle gaps + Z/A1 + certification");
}

#[test]
fn p2_sharing_and_swi_singletons_numbering_plants() {
    let s = Scratch::new("plants");
    let letter = b"guideline_event(A,B,C) :- guideline_event(A,C,x).\n";
    let good = b"guideline_event(A,_,C) :- guideline_event(A,C,x).\n";
    assert!(
        emission_ok(good, letter),
        "positive D1 sharing + gap control"
    );
    let all_uses = b"guideline_event(_,_,C) :- guideline_event(_,C,x).\n";
    assert!(
        !emission_ok(all_uses, letter),
        "anonymizing both shared uses changes the model"
    );
    assert_eq!(
        singletons(all_uses),
        0,
        "singleton-only gate misses shared-use anonymization"
    );
    let renumbered = swi(
        &s.0,
        "read_term_from_atom('p(A,B,C):-q(A,C)',T,[]),numbervars(T,0,_,[singletons(true)]),write_term(T,[quoted(true),numbervars(true),ignore_ops(true)]),halt.",
    );
    renumbered.require("SWI singletons(true) plant");
    assert_eq!(
        renumbered.out, b":-(p(A,_,B),q(A,B))",
        "plant must exercise the runtime numbering choice"
    );
    assert!(emission_ok(b":-(p(A,_,C),q(A,C))", b":-(p(A,B,C),q(A,C))"));
    assert!(
        !emission_ok(&renumbered.out, b":-(p(A,B,C),q(A,C))"),
        "singletons(true) loses the all-variable numbering gap"
    );
    assert!(
        read(&renumbered.out).text.contains(&0),
        "collision refusal fires on the renumbering plant"
    );
    assert!(
        !emission_ok(b"guideline_event(_,_,x).", b"guideline_event(A,A,x)."),
        "two-use shared-variable plant"
    );
    println!("SGW P2 plants: positive control + 3 bad emitters rejected");
}

#[test]
fn p2_question_and_proof_writers_stay_byte_identical() {
    let compiler = Compiler::new();
    let s = Scratch::new("shared-writer");
    let stem = "tests/green/formula--product";
    let ace = fs::read_to_string(root().join(format!("{stem}.ace"))).unwrap();
    let ulex = root().join(format!("{stem}.ulex"));
    let proof = compiler.compile(&s.0, "formula--product", &ace, Some(&ulex), None, "proof");
    proof.require("proof writer");
    assert_eq!(
        proof.out,
        baseline(&format!("{stem}.proof.expect")),
        "proof payload writer unchanged"
    );
    let fx = "tests/queries/green/dose-arithmetic/tree/guidelines/fx";
    let ace = fs::read_to_string(root().join(format!("{fx}/queries/q-mme.ace"))).unwrap();
    let ulex = root().join(format!("{fx}/lexicon.ulex"));
    let table = root().join(format!("{fx}/vocabulary.tsv"));
    let question = compiler.compile(&s.0, "q-mme", &ace, Some(&ulex), Some(&table), "question");
    question.require("question writer");
    assert_eq!(
        question.out,
        baseline(&format!("{fx}/queries/pl/q-mme.pl")),
        "question bytes stay letter-spelled"
    );
}

#[test]
fn p2_compile_command_reemits_the_fixture_tree() {
    let s = Scratch::new("compile-tree");
    let fixture = "tests/queries/green/dose-arithmetic/tree";
    copy_tree(&root().join(fixture), &s.0);
    fs::create_dir_all(s.0.join("rust/ckc")).unwrap();
    std::os::unix::fs::symlink(root().join("vendor"), s.0.join("vendor")).unwrap();
    std::os::unix::fs::symlink(root().join("rust/ckc/prolog"), s.0.join("rust/ckc/prolog"))
        .unwrap();
    let got = native(&s.0, &["compile", "fx"]);
    got.require("ckc compile fx");
    let mut checks = Checks::default();
    for id in ["conversion", "naloxone", "world"] {
        let rel = format!("guidelines/fx/pl/{id}.pl");
        let letter = baseline(&format!("{fixture}/{rel}"));
        let raw = fs::read(s.0.join(&rel)).unwrap();
        checks.bytes(&raw, &anonymize(&letter), &format!("ckc compile {id}"));
        checks.require(
            emission_ok(&raw, &letter),
            format!("ckc compile {id}: D1 + D2"),
        );
    }
    checks.finish("P2 ckc compile fixture tree");
}
