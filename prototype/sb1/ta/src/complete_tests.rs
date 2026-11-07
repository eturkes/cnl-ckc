use super::*;

#[test]
fn refusal_witnesses_match_independent_original_edges_and_provenance() {
    for (name, text) in [
        ("L1", include_str!("../plants/L1.pl.in")),
        ("L2", include_str!("../plants/L2.pl.in")),
        ("L3", include_str!("../plants/L3.pl.in")),
        ("L4", include_str!("../plants/L4.pl.in")),
        ("L5", include_str!("../plants/L5.pl.in")),
        ("G1", include_str!("../plants/G1.pl.in")),
        ("G2", include_str!("../plants/G2.pl.in")),
        ("P3", include_str!("../plants/P3.pl.in")),
        ("P4", include_str!("../plants/P4.pl.in")),
        ("P5", include_str!("../plants/P5.pl.in")),
        ("P6", include_str!("../plants/P6.pl.in")),
    ] {
        let mut p = Program::new();
        p.add("fixture".into(), text.into(), false).unwrap();
        let native = crate::Analysis::load(&p).unwrap();
        let mut model = Model::build(&p, None, true, Some(native.model.abs.clone()), true);
        let graph = crate::exact_graph::build_compressed(&p, &mut model, true);
        let growth = Growth::build(&p, &mut model, true, true);
        let exact = Analysis::from_parts(&p, model, graph, growth, true, true).unwrap();
        assert_eq!(exact.admitted(), native.admitted(), "{name}");
        assert_eq!(exact.witnesses(&p).unwrap(), native.witnesses(&p), "{name}");
        let mut actual = Vec::new();
        let expected = if let Some(r) = &native.refined {
            let mut bytes = Vec::new();
            r.write_edges(&p, &native.model, &mut bytes).unwrap();
            bytes
        } else {
            native.growth.edge_table(&p, &native.model).into_bytes()
        };
        if let Some(r) = &exact.refined {
            r.write_edges(&p, &exact.model, &mut actual).unwrap();
        } else {
            actual = exact.growth.edge_table(&p, &exact.model).into_bytes();
        }
        assert_eq!(actual, expected, "{name}: growth provenance");
    }
}

#[test]
fn concrete_slice_selection_matches_independent_oracle() {
    for text in [
        include_str!("../plants/P3.pl.in"),
        include_str!("../plants/P4.pl.in"),
        include_str!("../plants/P5.pl.in"),
        include_str!("../plants/P6.pl.in"),
        include_str!("../plants/regression-boxed-event.pl.in"),
        "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),person,countable).\nguideline_operator(actual,'$guideline_id'(context,d,1,box(1),[]),should) :- guideline_entity(actual,A,person,countable).\nguideline_event('$guideline_id'(context,d,1,box(1),[]),'$guideline_id'(product,d,2,ref(1),[]),help).\nguideline_arg('$guideline_id'(context,d,1,box(1),[]),'$guideline_id'(product,d,2,ref(1),[]),1,A) :- guideline_entity(actual,A,person,countable).\n",
    ] {
        let mut p = Program::new();
        p.add("knowledge".into(), text.into(), false).unwrap();
        p.add("patient".into(), "guideline_entity(actual,p,person,countable).\nguideline_entity(actual,7,number,countable).\nguideline_arg(actual,p,1,7).\n".into(), true).unwrap();
        let all = Question::parse(&mut p.arena, b"sb1-q-all 0\n").unwrap();
        let mut questions = vec![all];
        for c in &p.clauses {
            questions.push(Question::Goals {
                goals: vec![Goal::Positive(c.head)],
                vars: c.names.len(),
            });
            if !c.body.is_empty() {
                questions.push(Question::Goals {
                    goals: c.body.clone(),
                    vars: c.names.len(),
                });
            }
        }
        let native = crate::Analysis::load(&p).unwrap();
        assert!(native.admitted());
        let mut m = Model::build(&p, None, true, Some(native.model.abs.clone()), true);
        let graph = crate::exact_graph::build_compressed(&p, &mut m, true);
        let growth = Growth::build(&p, &mut m, true, true);
        let exact = Analysis::from_parts(&p, m, graph, growth, true, true).unwrap();
        for q in questions {
            let expected = native.slice(&p, &q);
            let actual = exact.slice(&p, &q, true, true);
            assert_eq!(actual.clauses, expected.clauses);
            assert_eq!(actual.coarse_clauses, expected.coarse_clauses);
            assert_eq!(actual.admitted, expected.admitted());
        }
    }
}
