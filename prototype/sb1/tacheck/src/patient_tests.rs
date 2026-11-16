use crate::{core, producer};
use core::extension::Original;

fn fixture() -> (core::Program, core::analysis::Certificate) {
    let mut p = sb1_ta::source::Program::new();
    p.add(
        "kb".into(),
        "guideline_entity(actual,'$guideline_id'(product,kb_doc,1,ref(1),[]),seed,countable).\n"
            .into(),
        false,
    )
    .unwrap();
    p.add("patient".into(), "guideline_entity(actual,'$guideline_id'(product,patient_doc,1,ref(1),[]),patient,countable).\n".into(), true).unwrap();
    let hint = sb1_ta::hints::export(&p, sb1_ta::hints::analyze(&p).unwrap()).unwrap();
    let p = producer::program(p);
    let cert = producer::decode(&p, hint).unwrap();
    (p, cert)
}
#[test]
fn fresh_patients_keep_fact_grammar_and_namespace_disjointness() {
    let (p, cert) = fixture();
    let c = core::analysis::check(p, cert, &mut core::Budget::default()).unwrap();
    let valid = Original {
        document: 2,
        ordinal: 1,
        text: "guideline_entity(actual,'$guideline_id'(product,other_patient,1,ref(1),[]),patient,countable)",
    };
    c.check_patient(&[valid]).unwrap();
    for text in [
        "guideline_entity(actual,'$guideline_id'(product,kb_doc,1,ref(1),[]),patient,countable)",
        "guideline_entity(actual,patient,kb_doc,countable)",
        "guideline_entity(actual,patient,patient,countable) :- guideline_entity(actual,patient,seed,countable)",
    ] {
        assert!(
            c.check_patient(&[Original {
                document: 2,
                ordinal: 1,
                text
            }])
            .is_err(),
            "{text}"
        );
    }
}
#[test]
fn the_checked_program_rechecks_initial_patient_flags_and_namespace_constraints() {
    let (mut p, cert) = fixture();
    p.clauses[1].head = p.clauses[0].head;
    assert!(matches!(
        core::analysis::check(p, cert, &mut core::Budget::default()),
        Err(core::Failure::Admission(_))
    ));
    let (mut p, cert) = fixture();
    let head = p.clauses[0].head;
    p.clauses[1]
        .body
        .push(sb1_checker::program::Goal::Positive(head));
    assert!(matches!(
        core::analysis::check(p, cert, &mut core::Budget::default()),
        Err(core::Failure::Admission(_))
    ));
}
