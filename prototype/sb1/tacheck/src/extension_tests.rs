use crate::{core, producer};
use sb1_checker::program::Goal;

#[test]
fn dc_relation_preserves_heads_and_body_prefix_and_adds_only_scope_naf() {
    let mut source = sb1_ta::source::Program::new();
    source.add("fixture".into(), "guideline_entity(actual,a,seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\n".into(), false).unwrap();
    let text = (0..source.clauses.len())
        .map(|i| source.render_clause(i).trim_end_matches('.').to_owned())
        .collect::<Vec<_>>();
    let original = text
        .iter()
        .enumerate()
        .map(|(i, text)| core::extension::Original {
            document: 1,
            ordinal: i + 1,
            text,
        })
        .collect::<Vec<_>>();
    let key = source.arena.atom("scope");
    let exclusion = source.arena.compound("guideline_exclusion", vec![key]);
    source.clauses[1]
        .body
        .push(Goal::Not(vec![Goal::Positive(exclusion)]));
    let mut shaped = producer::program(source);
    core::extension::check(&original, &shaped).unwrap();
    shaped.clauses[1].body.push(Goal::Positive(exclusion));
    assert!(matches!(
        core::extension::check(&original, &shaped),
        Err(core::Failure::Certificate(_))
    ));
    shaped.clauses[1].body.pop();
    core::extension::check(&original, &shaped).unwrap();
    shaped.clauses[1].ordinal = 3;
    assert!(matches!(
        core::extension::check(&original, &shaped),
        Err(core::Failure::Certificate(_))
    ));
    shaped.clauses[1].ordinal = 2;
    let old = shaped.clauses[1].head;
    shaped.clauses[1].head = shaped.clauses[0].head;
    assert!(matches!(
        core::extension::check(&original, &shaped),
        Err(core::Failure::Certificate(_))
    ));
    shaped.clauses[1].head = old;
    core::extension::check(&original, &shaped).unwrap();
}
