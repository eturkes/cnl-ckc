use super::*;

fn compare(p: &Program, full: bool) {
    let native = crate::features::Model::build(p, None, true);
    let base = crate::growth::Growth::build(p, &native, None);
    let expected = if full {
        crate::k1::Refined::build_full(p, &native, &base)
    } else {
        crate::k1::Refined::build(p, &native, &base)
    }
    .unwrap();
    let mut model = Model::build(p, None, true, Some(native.abs.clone()), true);
    let growth = Growth::build(p, &mut model, true, true);
    let actual = if full {
        Refined::build_full(p, &mut model, &growth, true, true)
    } else {
        Refined::build(p, &mut model, &growth, true, true)
    }
    .unwrap();
    assert_eq!(actual.full_refinement, expected.full_refinement);
    assert_eq!(actual.admitted(), expected.admitted());
    let retained = model.retention();
    assert_eq!(retained[0].1, retained[1].1);
    assert_eq!(retained[2].1, retained[3].1);
    assert_eq!(actual.generated_variants, expected.generated_variants);
    assert_eq!(actual.candidate_assignments, expected.candidate_assignments);
    assert_eq!(actual.rejected_assignments, expected.rejected_assignments);
    assert_eq!(actual.refined_templates, expected.refined_templates);
    assert_eq!(actual.projection_checks, expected.projection_checks);
    assert_eq!(format!("{:?}", actual.keys), format!("{:?}", expected.keys));
    assert_eq!(actual.program.clauses.len(), expected.program.clauses.len());
    for i in 0..actual.program.clauses.len() {
        assert_eq!(
            actual.program.render_clause(i),
            expected.program.render_clause(i)
        );
    }
    assert_eq!(actual.model.dump(), expected.model.exact_dump());
    assert_eq!(actual.growth.nodes, expected.growth.nodes);
    for i in 0..actual.growth.nodes.len() {
        assert_eq!(actual.growth.graph.row(i), expected.growth.graph.edges[i]);
    }
    let mut a = actual.canonical_edges(p, &model);
    a.sort_unstable();
    let mut b = expected.canonical_edges(p, &native);
    b.sort_unstable();
    assert_eq!(a, b);
    assert_eq!(
        actual.model.stats.borrow().audited,
        actual.model.stats.borrow().skipped
    );
}
#[test]
fn k1_registry_tags_support_filter_projection_and_fallback_match_oracle() {
    let fixtures = [
        include_str!("../plants/L1.pl.in"),
        include_str!("../plants/L2.pl.in"),
        include_str!("../plants/L3.pl.in"),
        include_str!("../plants/L4.pl.in"),
        include_str!("../plants/L5.pl.in"),
        include_str!("../plants/G1.pl.in"),
        include_str!("../plants/G2.pl.in"),
        include_str!("../plants/P3.pl.in"),
        include_str!("../plants/P4.pl.in"),
        include_str!("../plants/P5.pl.in"),
        include_str!("../plants/P6.pl.in"),
        include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s10-13.pl"),
        include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s9-05.pl"),
        "guideline_entity(actual,'$guideline_id'(product,share,1,ref(1),[A]),result,countable) :- guideline_entity(actual,B,seed,countable), guideline_cardinality(actual,B,na,C,D), guideline_product(C,D,exactly,2,E,A), guideline_product(C,D,exactly,3,E,A).\n",
    ];
    for text in fixtures {
        let mut p = Program::new();
        p.add("fixture".into(), text.into(), false).unwrap();
        p.dc_shape().unwrap();
        for full in [false, true] {
            compare(&p, full);
        }
    }
}
#[test]
fn a_missing_base_projection_edge_is_refused() {
    let mut p = Program::new();
    p.add(
        "fixture".into(),
        include_str!("../plants/G1.pl.in").into(),
        false,
    )
    .unwrap();
    let mut model = Model::build(&p, None, true, None, true);
    let mut growth = Growth::build(&p, &mut model, true, true);
    assert!(!growth.graph.row(0).is_empty());
    let edges = (0..growth.nodes.len())
        .map(|i| {
            if i == 0 {
                Vec::new()
            } else {
                growth.graph.row(i)
            }
        })
        .collect();
    let components = growth.graph.components.clone();
    let sizes = growth.graph.sizes.clone();
    let witnesses = std::mem::take(&mut growth.graph.witnesses);
    growth.graph = crate::exact_scc::Compressed::from_edges(edges, Counters::default());
    // Match the oracle's planted post-SCC adjacency corruption: selection stays
    // cyclic, so only the independent edge-projection check can reject it.
    growth.graph.components = components;
    growth.graph.sizes = sizes;
    growth.graph.witnesses = witnesses;
    let result = Refined::build_restricted(&p, &mut model, &growth, true, true);
    assert!(matches!(result, Err(e) if e.contains("projection")));
}
