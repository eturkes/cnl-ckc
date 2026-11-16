use crate::{core, producer};

fn question(patient: &str, noun: &str) -> Vec<u8> {
    let template = include_str!("../../../../bench/sb/queries/q-one.pl.in");
    let header = template.lines().take(3).collect::<Vec<_>>().join("\n");
    format!("{header}\n'$guideline_query_projection'(goal(guideline_entity(actual,{patient},{noun},countable)),answers([])).\n").into_bytes()
}
fn checked(text: &str) -> core::analysis::Checked {
    let mut source = sb1_ta::source::Program::new();
    source.add("kb".into(), text.into(), false).unwrap();
    let hint = sb1_ta::hints::export(&source, sb1_ta::hints::analyze(&source).unwrap()).unwrap();
    let p = producer::program(source);
    let cert = producer::decode(&p, hint).unwrap();
    core::analysis::check(p, cert, &mut core::Budget::default()).unwrap()
}
#[test]
fn query_roots_cover_fresh_admitted_patient_constants() {
    // Positive finite program: seed(t) entails out(t). For either admitted
    // patient constant t, dropping the sole producer changes the answer.
    for patient in [
        "patient_value",
        "'$guideline_id'(product,patient_world,1,ref(1),[])",
    ] {
        let mut c = checked(
            "guideline_entity(actual,X,out,countable) :- guideline_entity(actual,X,seed,countable).\n",
        );
        let selected = c
            .select(&question(patient, "out"), &mut core::Budget::default())
            .expect("canonical admitted query");
        assert_eq!(selected, vec![0], "patient constant {patient}");
    }
}
#[test]
fn known_kb_origins_stay_rigid_unknown_kb_doc_origins_use_top_and_calls_roll_back() {
    let mut c = checked(
        "guideline_entity(actual,'$guideline_id'(product,kb_doc,1,ref(1),[]),out,countable).\nguideline_entity(actual,'$guideline_id'(product,kb_doc,2,ref(1),[]),unrelated,countable).\nguideline_entity(actual,X,forwarded_out,countable) :- guideline_entity(actual,X,seed,countable).\n",
    );
    let checkpoint = c.program.arena.checkpoint();
    let terms = c.model.abs.terms.clone();
    let origins = c.model.abs.origins.clone();
    assert!(matches!(
        c.select(
            &question(
                "'$guideline_id'(product,patient_world,1,ref(1),[])",
                "forwarded_out"
            ),
            &mut core::Budget { used: 0, limit: 0 }
        ),
        Err(core::Failure::Indeterminate(_))
    ));
    assert_eq!(c.program.arena.checkpoint(), checkpoint);
    assert_eq!(c.model.abs.terms, terms);
    assert_eq!(c.model.abs.origins, origins);
    for _ in 0..2 {
        for (term, noun, expected) in [
            (
                "'$guideline_id'(product,patient_world,1,ref(1),[])",
                "forwarded_out",
                vec![2],
            ),
            (
                "'$guideline_id'(product,kb_doc,1,ref(1),[])",
                "out",
                vec![0],
            ),
            ("'$guideline_id'(product,kb_doc,2,ref(1),[])", "out", vec![]),
            (
                "'$guideline_id'(product,kb_doc,99,ref(1),[])",
                "forwarded_out",
                vec![2],
            ),
        ] {
            assert_eq!(
                c.select(&question(term, noun), &mut core::Budget::default())
                    .unwrap(),
                expected
            );
            assert_eq!(c.program.arena.checkpoint(), checkpoint);
            assert_eq!(c.model.abs.terms, terms);
            assert_eq!(c.model.abs.origins, origins);
        }
    }
}
