use super::*;
use graph::{Edge, Refusal};
use sb1_checker::program;
use sb1_checker::term::Node;
use std::path::PathBuf;

fn input(text: &str) -> Program {
    let mut p = Program::new();
    p.add(PathBuf::from("fixture"), text.into(), false).unwrap();
    program::admit(&p.arena, &p.clauses).unwrap();
    p
}
struct Directory(PathBuf);
impl Drop for Directory {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}
fn directory(name: &str) -> Directory {
    let parent = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    std::fs::create_dir_all(&parent).unwrap();
    let dir = Directory(parent.join(format!("{name}-{}", std::process::id())));
    std::fs::create_dir(&dir.0).unwrap();
    dir
}
const CASES: [(&str, &str, Option<Refusal>); 11] = [
    (
        "L1",
        include_str!("../plants/L1.pl.in"),
        Some(Refusal::Negative),
    ),
    (
        "L2",
        include_str!("../plants/L2.pl.in"),
        Some(Refusal::Negative),
    ),
    (
        "L3",
        include_str!("../plants/L3.pl.in"),
        Some(Refusal::Negative),
    ),
    (
        "L4",
        include_str!("../plants/L4.pl.in"),
        Some(Refusal::Negative),
    ),
    (
        "L5",
        include_str!("../plants/L5.pl.in"),
        Some(Refusal::Negative),
    ),
    (
        "G1",
        include_str!("../plants/G1.pl.in"),
        Some(Refusal::Growth),
    ),
    (
        "G2",
        include_str!("../plants/G2.pl.in"),
        Some(Refusal::Growth),
    ),
    ("P3", include_str!("../plants/P3.pl.in"), None),
    ("P4", include_str!("../plants/P4.pl.in"), None),
    ("P5", include_str!("../plants/P5.pl.in"), None),
    ("P6", include_str!("../plants/P6.pl.in"), None),
];

#[test]
fn frozen_plants_and_positive_expectations() {
    for (name, text, expected) in CASES {
        let p = input(text);
        let analysis = Analysis::load(&p).unwrap();
        match expected {
            Some(kind) => {
                assert!(
                    analysis
                        .graph
                        .witnesses
                        .iter()
                        .chain(&analysis.growth.graph.witnesses)
                        .any(|w| w.kind == kind),
                    "{name}: {}",
                    analysis.witnesses(&p)
                );
                for w in analysis
                    .graph
                    .witnesses
                    .iter()
                    .chain(&analysis.growth.graph.witnesses)
                {
                    assert!(!w.cycle.is_empty(), "{name}");
                    for pair in w.cycle.windows(2) {
                        assert_eq!(pair[0].1.to, pair[1].0);
                    }
                    assert_eq!(w.cycle.last().unwrap().1.to, w.cycle[0].0);
                    assert!(w.cycle.iter().any(|(_, e)| match w.kind {
                        Refusal::Negative => e.negative,
                        Refusal::Growth => e.growth,
                    }));
                }
            }
            None => assert!(analysis.admitted(), "{name}: {}", analysis.witnesses(&p)),
        }
    }
}

#[test]
fn variable_head_features_close_across_multiple_rules() {
    let p = input(
        "guideline_entity(actual,seed,original,countable).\nguideline_entity(actual,A,middle,countable) :- guideline_entity(actual,A,original,countable).\nguideline_entity(actual,A,last,countable) :- guideline_entity(actual,A,middle,countable).\nguideline_exclusion(k) :- guideline_entity(actual,A,last,countable).\n",
    );
    let a = Analysis::load(&p).unwrap();
    assert!(a.graph.edges[3].iter().any(|e| e.to == 2));
    assert!(a.graph.edges[2].iter().any(|e| e.to == 1));
    assert!(a.graph.edges[1].iter().any(|e| e.to == 0));
}

#[test]
fn correlated_naf_keeps_links_and_later_content_constraints() {
    let p = input(
        "guideline_event(actual,'$guideline_id'(product,a,1,ref(1),[]),has).\nguideline_arg(actual,'$guideline_id'(product,a,1,ref(1),[]),1,'$guideline_id'(product,a,1,ref(2),[])).\nguideline_entity(actual,'$guideline_id'(product,a,1,ref(2),[]),left,countable).\nguideline_entity(actual,'$guideline_id'(product,a,1,ref(3),[]),right,countable).\nguideline_exclusion(k) :- \\+ (guideline_event(actual,A,has), guideline_arg(actual,A,1,B), guideline_entity(actual,B,right,countable)).\n",
    );
    let a = Analysis::load(&p).unwrap();
    // A patient can supply a witness, but the existing e/x link cannot be
    // paired with the unrelated y/right fact.
    assert!(
        !a.graph.edges[4]
            .iter()
            .any(|e| e.to == 0 || e.to == 1 || e.to == 3)
    );
}

#[test]
fn classical_box_is_positive_and_the_whole_conjunction_refines_it() {
    let p = input(
        "guideline_operator(actual,'$guideline_id'(product,a,1,box(1),[]),-).\nguideline_entity('$guideline_id'(product,a,1,box(1),[]),x,left,countable).\nguideline_operator(actual,'$guideline_id'(product,b,1,box(1),[]),-).\nguideline_entity('$guideline_id'(product,b,1,box(1),[]),y,right,countable).\nguideline_exclusion(k) :- guideline_operator(actual,A,-), guideline_entity(A,B,right,countable).\n",
    );
    let a = Analysis::load(&p).unwrap();
    assert!(!a.graph.edges[4].iter().any(|e| e.to == 0 || e.to == 1));
    assert!(a.graph.edges[4].iter().any(|e| e.to == 2 && !e.negative));
    assert!(a.graph.edges[4].iter().any(|e| e.to == 3 && !e.negative));
}

#[test]
fn guard_transform_covers_the_statement_and_is_idempotent() {
    let mut p = input(
        "% S1: A statement.\nguideline_entity(actual,'$guideline_id'(product,a,1,ref(1),[]),thing,countable) :- guideline_entity(actual,A,patient,countable).\nguideline_operator(actual,'$guideline_id'(product,a,1,box(1),[]),should) :- guideline_entity(actual,A,patient,countable), \\+ guideline_exclusion(k).\n% S2: Data.\nguideline_entity(actual,x,other,countable).\n",
    );
    assert_eq!(p.dc_shape().unwrap(), 1);
    assert!(matches!(
        p.clauses[0].body.last(),
        Some(program::Goal::Not(_))
    ));
    assert!(p.clauses[2].body.is_empty());
    assert_eq!(p.dc_shape().unwrap(), 0);
}

#[test]
fn witness_is_the_shortest_cycle_through_a_violating_edge() {
    let edge = |to, negative| Edge {
        to,
        negative,
        growth: false,
    };
    let graph = Graph::from_edges(
        vec![
            vec![edge(1, true), edge(2, true)],
            vec![edge(2, false)],
            vec![edge(0, false)],
        ],
        Counters::default(),
    );
    assert_eq!(graph.witnesses.len(), 1);
    assert_eq!(graph.witnesses[0].cycle.len(), 2);
}

#[test]
fn q_one_demand_ignores_an_unrelated_shard() {
    let mut p = input(
        "guideline_entity(actual,x,recommendation,countable).\nguideline_entity(actual,y,unrelated,countable).\n",
    );
    let actual = p.arena.atom("actual");
    let noun = p.arena.atom("recommendation");
    let kind = p.arena.atom("countable");
    let var = p.arena.put(Node::Var(0));
    let term = p
        .arena
        .compound("guideline_entity", vec![actual, var, noun, kind]);
    let q = Question::Goals {
        goals: vec![program::Goal::Positive(term)],
        vars: 1,
    };
    let a = Analysis::load(&p).unwrap();
    let slice = a.slice(&p, &q);
    assert!(slice.clauses.contains(&0));
    assert!(!slice.clauses.contains(&1));
}

#[test]
fn exact_argument_roles_exclude_the_taper_event_from_the_evidence_naf() {
    let p = input(
        "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),taper,countable).\nguideline_entity(actual,'$guideline_id'(product,d,1,ref(2),[]),evidence,countable).\nguideline_event(actual,'$guideline_id'(product,d,1,ref(3),[]),have) :- guideline_entity(actual,A,patient,countable), \\+ (guideline_entity(actual,B,evidence,countable), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).\nguideline_arg(actual,'$guideline_id'(product,d,1,ref(3),[]),1,A) :- guideline_entity(actual,A,patient,countable).\nguideline_arg(actual,'$guideline_id'(product,d,1,ref(3),[]),2,'$guideline_id'(product,d,1,ref(1),[])) :- guideline_entity(actual,A,patient,countable).\n",
    );
    let a = Analysis::load(&p).unwrap();
    assert!(
        a.graph.admitted(),
        "{}",
        a.graph.render_witnesses(&p, &a.model)
    );
    assert!(!a.graph.edges[2].iter().any(|e| e.to == 2 && e.negative));
}

#[test]
fn patient_top_domain_removes_subsumed_atomic_choices() {
    use crate::features::{Domain, Value};
    let mut arena = sb1_checker::term::Arena::default();
    let atom = arena.atom("seed");
    let d = Domain::Set(BTreeSet::from([Value::Patient, Value::Exact(atom)]));
    assert_eq!(
        d.meet(&Domain::one(Value::Patient), &arena),
        Domain::one(Value::Patient)
    );
}

#[test]
fn numeric_patient_constants_keep_all_features() {
    let p = input(
        "guideline_operator(actual,'$guideline_id'(product,n,1,box(1),[]),should) :- guideline_entity(actual,5,patient,countable), \\+ guideline_exclusion(n).\nguideline_exclusion(n) :- guideline_operator(actual,A,should).\n",
    );
    let a = Analysis::load(&p).unwrap();
    assert!(
        a.graph
            .witnesses
            .iter()
            .any(|w| w.kind == Refusal::Negative)
    );
}

#[test]
fn growth_refusals_use_origin_nodes_not_clause_sccs() {
    for (name, text, _) in CASES.iter().filter(|(name, _, _)| name.starts_with('G')) {
        let p = input(text);
        let a = Analysis::load(&p).unwrap();
        assert!(a.graph.admitted(), "{name}");
        assert!(!a.growth.admitted(), "{name}");
    }
}

#[test]
fn aliased_numeric_output_preserves_each_arithmetic_origin() {
    let p = input(
        "guideline_entity(actual,'$guideline_id'(product,share,1,ref(1),[A]),result,countable) :- guideline_entity(actual,B,seed,countable), guideline_cardinality(actual,B,na,C,D), guideline_product(C,D,exactly,2,E,A), guideline_product(C,D,exactly,3,E,A).\n",
    );
    let a = Analysis::load(&p).unwrap();
    let count = a.growth.graph.edges[0]
        .iter()
        .filter(|e| matches!(a.growth.nodes[e.to], growth::Origin::Arithmetic { .. }))
        .count();
    assert_eq!(count, 2);
}

#[test]
fn boxed_event_cannot_feed_an_actual_dependency() {
    let p = input(include_str!("../plants/regression-boxed-event.pl.in"));
    let a = Analysis::load(&p).unwrap();
    assert!(a.admitted(), "{}", a.witnesses(&p));
    assert_eq!(a.growth.graph.edge_count(), 0);
}

#[test]
fn body_memoization_preserves_edges_under_head_variable_renumbering() {
    let p = input(
        "guideline_operator(actual,'$guideline_id'(context,a,1,box(1),[]),should) :- guideline_entity(actual,A,person,countable), guideline_event(actual,B,do), guideline_arg(actual,B,1,A), \\+ guideline_exclusion(k).\nguideline_pp(actual,B,to,A) :- guideline_entity(actual,A,person,countable), guideline_event(actual,B,do), guideline_arg(actual,B,1,A), \\+ guideline_exclusion(k).\nguideline_exclusion(k) :- guideline_pp(actual,A,to,B).\n",
    );
    let model = may::Model::build(&p, None, true);
    let cached = Graph::build(&p, &model, None);
    let plain = Graph::build_uncached(&p, &model);
    assert!(cached.body_cache_hits > 0);
    assert_eq!(cached.edges, plain.edges);
    assert_eq!(cached.components, plain.components);
    let constants = input(
        "guideline_entity(actual,x,left,countable).\nguideline_entity(actual,y,right,countable).\nguideline_property(actual,A,first,pos) :- guideline_entity(actual,A,left,countable).\nguideline_property(actual,A,second,pos) :- guideline_entity(actual,A,right,countable).\n",
    );
    let model = may::Model::build(&constants, None, true);
    let cached = Graph::build(&constants, &model, None);
    let plain = Graph::build_uncached(&constants, &model);
    assert_eq!(cached.edges, plain.edges);
    assert_ne!(cached.edges[2], cached.edges[3]);
    for (_, text, _) in CASES {
        let p = input(text);
        let model = may::Model::build(&p, None, true);
        assert_eq!(
            Graph::build(&p, &model, None).edges,
            Graph::build_uncached(&p, &model).edges
        );
    }
}

#[test]
fn selective_unary_checks_equal_full_posting_intersections() {
    use features::{Domain, Expr, Pattern, State, Value};
    for (_, text, _) in CASES {
        let p = input(text);
        let model = may::Model::build(&p, None, true);
        for c in &p.clauses {
            let mut patterns = vec![Pattern::body(
                &p.arena,
                &model.abs,
                &c.body,
                c.names.len(),
                None,
            )];
            for g in &c.body {
                if let program::Goal::Not(inner) = g {
                    patterns.push(Pattern::body(
                        &p.arena,
                        &model.abs,
                        &c.body,
                        c.names.len(),
                        Some(inner),
                    ));
                }
            }
            for pattern in patterns {
                let values = [Value::Patient, Value::Number]
                    .into_iter()
                    .chain((0..model.abs.origins.len()).map(Value::Origin))
                    .collect::<Vec<_>>();
                let mut initial = vec![State::new(c.names.len())];
                for var in 0..c.names.len() {
                    for value in &values {
                        let mut s = State::new(c.names.len());
                        s.constrain(Expr::Var(var), &Domain::one(*value), &p.arena);
                        initial.push(s);
                    }
                }
                for state in initial {
                    let mut counters = Counters::default();
                    assert_eq!(
                        model.solve(&p.arena, &pattern, state.clone(), true, &mut counters),
                        model.solve_reference(&p.arena, &pattern, state, &mut counters)
                    );
                }
            }
        }
    }
}

#[test]
fn one_level_annotations_keep_genuine_growth_plants() {
    for (name, text, expected) in CASES {
        let p = input(text);
        let model = may::Model::build(&p, None, true);
        let base = growth::Growth::build(&p, &model, None);
        let refined = k1::Refined::build(&p, &model, &base).unwrap();
        assert_eq!(
            refined.admitted(),
            expected != Some(Refusal::Growth),
            "{name}: {}",
            refined.render_witnesses(&p, &model)
        );
    }
}

#[test]
fn one_level_annotations_separate_noncomposing_clinical_instances() {
    for text in [
        include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s10-13.pl"),
        include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s9-05.pl"),
    ] {
        let mut p = input(text);
        p.dc_shape().unwrap();
        let model = may::Model::build(&p, None, true);
        let base = growth::Growth::build(&p, &model, None);
        assert!(!base.admitted());
        let refined = k1::Refined::build(&p, &model, &base).unwrap();
        assert!(
            refined.admitted(),
            "{}",
            refined.render_witnesses(&p, &model)
        );
        let full = k1::Refined::build_full(&p, &model, &base).unwrap();
        assert_eq!(refined.admitted(), full.admitted());
        assert!(!refined.full_refinement);
        assert!(refined.refined_templates < model.abs.origins.len());
        assert_eq!(refined.projection_checks, refined.growth.graph.edge_count());
    }
}

#[test]
fn refined_edge_projection_rejects_a_missing_base_edge() {
    let p = input(include_str!("../plants/G1.pl.in"));
    let model = may::Model::build(&p, None, true);
    let mut base = growth::Growth::build(&p, &model, None);
    assert!(!base.graph.edges[0].is_empty());
    base.graph.edges[0].clear();
    let result = k1::Refined::build_restricted(&p, &model, &base);
    assert!(matches!(result,Err(e)if e.contains("projection")));
}

#[test]
fn concrete_scalar_wildcards_narrow_to_observed_values() {
    use features::{Domain, Expr, Pattern, State, Value};
    let p = input(
        "guideline_entity(actual,5,patient,countable).\nguideline_entity(actual,'$guideline_id'(product,scalar,1,ref(1),[A]),result,countable) :- guideline_entity(actual,A,patient,countable).\n",
    );
    let model = may::Model::build(&p, None, false);
    let c = &p.clauses[1];
    let pattern = Pattern::body(&p.arena, &model.abs, &c.body, c.names.len(), None);
    for value in [Value::Patient, Value::Number] {
        let mut s = State::new(c.names.len());
        s.constrain(Expr::Var(0), &Domain::one(value), &p.arena);
        let mut counters = Counters::default();
        assert_eq!(
            model.solve(&p.arena, &pattern, s.clone(), true, &mut counters),
            model.solve_reference(&p.arena, &pattern, s, &mut counters)
        );
    }
    let base = growth::Growth::build(&p, &model, None);
    let refined = k1::Refined::build_full(&p, &model, &base).unwrap();
    let (_, args) = refined
        .program
        .arena
        .call(refined.program.clauses[1].head)
        .unwrap();
    assert!(
        refined
            .model
            .noun_at(&refined.program.arena, args[1], "result")
    );
}

#[test]
fn default_admission_uses_context_refined_growth() {
    for text in [
        include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s10-13.pl"),
        include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s9-05.pl"),
    ] {
        let mut p = input(text);
        p.dc_shape().unwrap();
        let analysis = Analysis::load(&p).unwrap();
        assert!(!analysis.growth.admitted());
        assert!(analysis.admitted(), "{}", analysis.witnesses(&p));
        assert_eq!(analysis.witnesses(&p).lines().count(), 1);
    }
}

#[test]
fn variable_source_cache_preserves_alpha_bindings_and_local_arithmetic() {
    let mut texts = CASES
        .iter()
        .map(|(_, text, _)| text.to_string())
        .collect::<Vec<_>>();
    texts.push(format!(
        "{}{}",
        include_str!("../plants/G2.pl.in"),
        include_str!("../plants/G2.pl.in")
    ));
    texts.push("guideline_entity(actual,'$guideline_id'(product,root,1,ref(1),[]),patient,countable).\nguideline_event(actual,'$guideline_id'(product,root,1,ref(2),[]),help).\nguideline_arg(actual,'$guideline_id'(product,root,1,ref(2),[]),1,'$guideline_id'(product,root,1,ref(1),[])).\nguideline_entity(actual,'$guideline_id'(product,out,1,ref(1),[A,B]),result,countable) :- guideline_entity(actual,A,patient,countable), guideline_event(actual,B,help), guideline_arg(actual,B,1,A).\nguideline_event(actual,'$guideline_id'(product,out,2,ref(1),[B,A]),other) :- guideline_entity(actual,A,patient,countable), guideline_event(actual,B,help), guideline_arg(actual,B,1,A).\n".into());
    let mut hits = 0;
    for text in texts {
        let p = input(&text);
        let model = may::Model::build(&p, None, true);
        let cached = growth::Growth::build(&p, &model, None);
        let plain = growth::Growth::build_reference(&p, &model);
        assert_eq!(cached.graph.edges, plain.graph.edges);
        assert_eq!(cached.var_sources, plain.var_sources);
        hits += cached.source_cache_hits;
    }
    assert!(hits > 0);
}

#[test]
fn requested_flow_sources_preserve_complete_rows_and_provenance() {
    let texts = CASES.iter().map(|(_, text, _)| *text).chain([
        include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s10-13.pl"),
        include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s9-05.pl"),
    ]);
    for text in texts {
        let p = input(text);
        let model = may::Model::build(&p, None, true);
        let full = growth::Growth::build(&p, &model, None);
        for (from, origin) in full.nodes.iter().enumerate() {
            let sparse = growth::Growth::for_sources(&p, &model, |n| n == origin);
            let row = |g: &growth::Growth| {
                g.graph.edges[from]
                    .iter()
                    .map(|e| (e.to, g.supports(from, e.to).collect::<Vec<_>>()))
                    .collect::<Vec<_>>()
            };
            assert_eq!(row(&sparse), row(&full));
            assert!(
                sparse
                    .graph
                    .edges
                    .iter()
                    .enumerate()
                    .all(|(i, row)| i == from || row.is_empty())
            );
        }
    }
}

#[test]
fn compact_edges_reconstruct_frozen_witnesses() {
    assert_eq!(
        std::mem::size_of::<Edge>(),
        2 * std::mem::size_of::<usize>()
    );
    for (name, text, expected) in CASES {
        if expected.is_none() {
            continue;
        }
        let p = input(text);
        let a = Analysis::load(&p).unwrap();
        let witnesses = a.witnesses(&p);
        assert!(witnesses.lines().count() > 1, "{name}");
        assert!(!witnesses.contains("unconfirmed"), "{name}: {witnesses}");
    }
}

#[test]
fn released_parser_metadata_keeps_admission_edges_and_source() {
    for (_, text, _) in CASES {
        let mut p = input(text);
        let counts = p.clauses.iter().map(|c| c.names.len()).collect::<Vec<_>>();
        let source = (0..p.clauses.len())
            .map(|i| p.render_clause(i))
            .collect::<Vec<_>>();
        let before = Analysis::load(&p).unwrap();
        p.release_read_metadata();
        program::admit(&p.arena, &p.clauses).unwrap();
        let after = Analysis::load(&p).unwrap();
        assert_eq!(
            counts,
            p.clauses.iter().map(|c| c.names.len()).collect::<Vec<_>>()
        );
        assert_eq!(
            source,
            (0..p.clauses.len())
                .map(|i| p.render_clause(i))
                .collect::<Vec<_>>()
        );
        assert_eq!(before.admitted(), after.admitted());
        assert_eq!(before.graph.edges, after.graph.edges);
        assert_eq!(before.growth.graph.edges, after.growth.graph.edges);
        assert_eq!(before.growth.var_sources, after.growth.var_sources);
    }
}

#[test]
fn emitted_compositions_carry_truthful_manifest_counts() {
    let mut p = input(
        "guideline_schema_version(2).\nguideline_document(d,ace_sha256(h),ulex(none),vocabulary(none)).\nguideline_entity(actual,x,left,countable).\nguideline_entity(actual,y,right,countable).\n",
    );
    p.add(
        PathBuf::from("patient"),
        "guideline_entity(actual,p,patient,countable).\n".into(),
        true,
    )
    .unwrap();
    let dir = directory("emission-manifest");
    for (name, selected, counts) in [
        ("full", None, (2, 5, 1)),
        ("subset", Some(BTreeSet::from([2, 4])), (2, 4, 1)),
        ("knowledge", Some(BTreeSet::from([2])), (1, 3, 0)),
    ] {
        let out = dir.0.join(name);
        let composition = p.emit(&out, selected.as_ref()).unwrap();
        let manifest = std::fs::read_to_string(out.join("manifest.tsv")).unwrap();
        let actual = Program::load(&dir.0, &composition, true).unwrap();
        assert_eq!(actual.documents.len(), counts.0);
        assert_eq!(actual.clauses.len(), counts.1);
        assert_eq!(
            actual.documents.iter().filter(|d| d.patient).count(),
            counts.2
        );
        assert_eq!(
            manifest,
            format!(
                "format\tsb1-ta-composition 1\ndocuments\t{}\nclauses\t{}\npatient_documents\t{}\n",
                counts.0, counts.1, counts.2
            )
        );
    }
}

#[test]
fn patient_projection_suffix_is_per_call_and_replays() {
    use std::fs;
    let dir = directory("patient-suffix");
    fs::write(
        dir.0.join("kb.pl"),
        "guideline_entity(actual,k,seed,countable).\n",
    )
    .unwrap();
    fs::write(
        dir.0.join("first.pl"),
        "guideline_entity(actual,p,first,countable).\n",
    )
    .unwrap();
    fs::write(
        dir.0.join("last.pl"),
        "guideline_entity(actual,p,last,countable).\n",
    )
    .unwrap();
    let composition = dir.0.join("composition.tsv");
    fs::write(&composition, "kb.pl\nfirst.pl\nlast.pl\n").unwrap();
    let manifest = dir.0.join("manifest.tsv");
    fs::write(&manifest, "patient_documents\t2\n").unwrap();
    let flags = |p: &Program| p.documents.iter().map(|d| d.patient).collect::<Vec<_>>();
    let p = Program::load(&dir.0, &composition, true).unwrap();
    assert_eq!(flags(&p), [false, true, true]);
    let emitted = p.emit(&dir.0.join("emitted"), None).unwrap();
    assert_eq!(
        flags(&Program::load(&dir.0, &emitted, true).unwrap()),
        flags(&p)
    );
    fs::remove_file(&manifest).unwrap();
    assert_eq!(
        flags(&Program::load(&dir.0, &composition, true).unwrap()),
        [false, false, true]
    );
    fs::write(&manifest, "patient_documents\t2\n").unwrap();
    fs::write(
        dir.0.join("first.pl"),
        "guideline_property(actual,A,result,pos) :- guideline_entity(actual,A,seed,countable).\n",
    )
    .unwrap();
    assert!(
        Program::load(&dir.0, &composition, true)
            .err()
            .unwrap()
            .contains("patient document contains a rule")
    );
    fs::write(&manifest, "patient_documents\t4\n").unwrap();
    assert!(
        Program::load(&dir.0, &composition, true)
            .err()
            .unwrap()
            .contains("patient suffix")
    );
}

#[test]
fn generic_closure_keeps_owner_and_target_link_constraints() {
    use features::{Mode, Model};
    for owner in ["actual", "'$guideline_id'(context,links,1,box(1),[])"] {
        let target = if owner == "actual" { 2 } else { 3 };
        let p = input(&format!(
            "guideline_event(actual,'$guideline_id'(product,links,1,ref(1),[]),go).\nguideline_entity(actual,'$guideline_id'(product,links,1,ref(2),[]),left,countable).\nguideline_entity(actual,'$guideline_id'(product,links,1,ref(3),[]),right,countable).\nguideline_arg({owner},'$guideline_id'(product,links,1,ref(1),[]),1,'$guideline_id'(product,links,1,ref({target}),[])).\nguideline_entity(actual,A,selected,countable) :- guideline_event(actual,A,go), guideline_arg(actual,A,1,B), guideline_entity(actual,B,right,countable).\n",
        ));
        let refined = Model::build(&p, None, true);
        let reference =
            Model::build_mode(&p, None, true, Some(refined.abs.clone()), Mode::Reference);
        let (_, args) = p.arena.call(p.clauses[0].head).unwrap();
        assert!(!refined.noun_at(&p.arena, args[1], "selected"), "{owner}");
        assert!(reference.noun_at(&p.arena, args[1], "selected"), "{owner}");
        refined.refines_reference(&reference, &p.arena).unwrap();
    }
}

#[test]
fn generic_closure_reads_stored_links_for_patient_and_numeric_sources() {
    for source in ["p", "5", "rational(1,2)"] {
        let p = input(&format!(
            "guideline_arg(actual,{source},1,'$guideline_id'(product,stored,1,ref(1),[])).\nguideline_entity(actual,B,marked,countable) :- guideline_entity(actual,A,patient,countable), guideline_arg(actual,A,1,B).\n",
        ));
        let model = may::Model::build(&p, None, true);
        let (_, args) = p.arena.call(p.clauses[0].head).unwrap();
        assert!(model.noun_at(&p.arena, args[3], "marked"), "{source}");
    }
}

#[test]
fn owner_variables_keep_joint_link_domains_and_unknown_owners_stay_may() {
    use features::{Domain, Expr, Pattern};
    let p = input(
        "guideline_arg(actual,'$guideline_id'(product,owner,1,ref(1),[]),1,'$guideline_id'(product,owner,1,ref(2),[])).\nguideline_arg('$guideline_id'(context,owner,1,box(1),[]),'$guideline_id'(product,owner,1,ref(1),[]),1,'$guideline_id'(product,owner,1,ref(3),[])).\nguideline_exclusion(k) :- guideline_arg(A,'$guideline_id'(product,owner,1,ref(1),[]),1,'$guideline_id'(product,owner,1,ref(3),[])).\n",
    );
    let model = may::Model::build(&p, None, true);
    let c = &p.clauses[2];
    let mut pattern = Pattern::body(&p.arena, &model.abs, &c.body, c.names.len(), None);
    for (index, expected) in [(0, false), (1, true)] {
        let (_, args) = p.arena.call(p.clauses[index].head).unwrap();
        let mut counters = Counters::default();
        assert_eq!(
            model
                .bound_state(
                    &p.arena,
                    &pattern,
                    Expr::Var(0),
                    &Domain::one(model.abs.value(args[0])),
                    &mut counters
                )
                .is_some(),
            expected
        );
    }
    pattern.unary.clear();
    assert!(
        model
            .state(&p.arena, &pattern, &mut Counters::default())
            .is_some()
    );
}

#[test]
fn refinement_comparison_accepts_wildcard_reference_sources() {
    use features::{Mode, Model};
    let p = input(
        "guideline_event(actual,'$guideline_id'(product,cmp,1,ref(1),[]),go).\nguideline_arg(actual,'$guideline_id'(product,cmp,1,ref(1),[]),1,'$guideline_id'(product,cmp,1,ref(2),[])).\nguideline_arg(actual,B,2,B) :- guideline_event(actual,A,go), guideline_arg(actual,A,1,B).\n",
    );
    let model = Model::build(&p, None, true);
    let reference = Model::build_mode(&p, None, true, Some(model.abs.clone()), Mode::Reference);
    model.refines_reference(&reference, &p.arena).unwrap();
    let missing_writer = Model::build_mode(
        &p,
        Some(&BTreeSet::from([0, 1])),
        true,
        Some(model.abs.clone()),
        Mode::Reference,
    );
    assert!(model.refines_reference(&missing_writer, &p.arena).is_err());
}

#[test]
fn patient_namespace_collisions_fail_closed() {
    let kb = "guideline_entity(actual,'$guideline_id'(product,kb,1,ref(1),[]),seed,countable).\n";
    for fact in [
        "guideline_entity(actual,'$guideline_id'(product,kb,1,ref(1),[]),patient,countable).\n",
        "guideline_entity(actual,'$guideline_id'(product,kb,2,ref(4),[]),patient,countable).\n",
        "guideline_entity(actual,kb,patient,countable).\n",
        "guideline_document(kb,ace_sha256(h),ulex(none),vocabulary(none)).\n",
    ] {
        let mut p = input(kb);
        p.add(PathBuf::from("patient"), fact.into(), true).unwrap();
        assert!(
            p.validate_patient_boundary()
                .unwrap_err()
                .contains("namespace collision")
        );
        assert!(
            Analysis::load(&p)
                .err()
                .unwrap()
                .contains("namespace collision")
        );
    }
    let mut p = input(kb);
    p.add(
        PathBuf::from("patient"),
        "guideline_entity(actual,'$guideline_id'(product,world,1,ref(1),[]),patient,countable).\n"
            .into(),
        true,
    )
    .unwrap();
    p.validate_patient_boundary().unwrap();
    assert!(Analysis::load(&p).unwrap().admitted());
}

#[test]
fn top_referent_keeps_knowledge_links_in_negative_cycles() {
    let p = input(
        "guideline_arg(actual,p,1,'$guideline_id'(product,top,1,ref(1),[])) :- \\+ guideline_exclusion(k).\nguideline_exclusion(k) :- guideline_arg(actual,p,1,'$guideline_id'(product,top,1,ref(1),[])).\n",
    );
    let a = Analysis::load(&p).unwrap();
    assert!(
        !a.admitted(),
        "a KB-derived link from patient/top must keep its negative cycle"
    );
    assert!(
        a.graph
            .witnesses
            .iter()
            .any(|w| w.kind == Refusal::Negative)
    );
}

#[test]
fn wildcard_meet_shortcuts_equal_the_unoptimized_relation_on_all_small_domains() {
    use features::{Domain, Value};
    let mut arena = sb1_checker::term::Arena::default();
    let atom = arena.atom("opaque");
    let one = arena.put(Node::Integer("1".into()));
    let two = arena.put(Node::Integer("2".into()));
    let rational = arena.compound("rational", vec![one, two]);
    let values = [
        Value::Any,
        Value::Patient,
        Value::Number,
        Value::Exact(atom),
        Value::Exact(one),
        Value::Exact(rational),
        Value::Origin(0),
        Value::Origin(1),
    ];
    let domains = (0..1 << values.len())
        .map(|mask| {
            Domain::Set(
                values
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| (mask & (1 << i) != 0).then_some(*v))
                    .collect(),
            )
        })
        .chain(std::iter::once(Domain::All))
        .collect::<Vec<_>>();
    for a in &domains {
        for b in &domains {
            assert_eq!(
                a.meet(b, &arena),
                a.meet_reference(b, &arena),
                "{a:?} ∩ {b:?}"
            );
        }
    }
}

#[test]
fn cyclic_source_rows_keep_every_cycle_on_three_node_graphs() {
    for mask in 0..(1 << 9) {
        let edges = (0..3)
            .map(|from| {
                (0..3)
                    .filter(|to| mask & (1 << (from * 3 + to)) != 0)
                    .map(|to| Edge {
                        to,
                        negative: false,
                        growth: true,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let full = Graph::from_edges(edges.clone(), Counters::default());
        let cyclic = full
            .witnesses
            .iter()
            .map(|w| w.component)
            .collect::<BTreeSet<_>>();
        let sparse = edges
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                if cyclic.contains(&full.components[i]) {
                    row
                } else {
                    Vec::new()
                }
            })
            .collect();
        assert_eq!(
            full.admitted(),
            Graph::from_edges(sparse, Counters::default()).admitted()
        );
    }
}
