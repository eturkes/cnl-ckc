use crate::finite_oracle::ground::{self, Grounded, Template, Term};
use crate::{core, producer};
use sb1_ta::domain::Value;
use sb1_ta::features::{Feature, Label};
use std::collections::BTreeSet;

const SEED: u64 = 6071227972663472132;
struct Random(u64);
impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
#[derive(Clone)]
struct Case {
    name: String,
    documents: Vec<(String, bool)>,
    queries: Vec<String>,
}
fn identity(doc: &str, s: usize, deps: &str) -> String {
    format!("'$guideline_id'(product,{doc},{s},ref(1),[{deps}])")
}
fn cases() -> Vec<Case> {
    let mut rng = Random(SEED);
    let mut out = Vec::new();
    for i in 0..256 {
        let salt = rng.next();
        let doc = format!("kb_{salt:x}");
        let noun = format!("seed_{salt:x}");
        let next = format!("next_{salt:x}");
        let end = format!("end_{salt:x}");
        let a = identity(&doc, 1, "");
        let b = identity(&doc, 2, "");
        let c = identity(&doc, 3, "");
        let role = 1 + (salt & 1);
        let other = 3 - role;
        let mut patient = None;
        let (text, queries) = match i % 8 {
            0 => (
                format!(
                    "guideline_entity(actual,{a},{noun},countable).\nguideline_entity(actual,X,{next},countable) :- guideline_entity(actual,X,{noun},countable).\nguideline_entity(actual,X,{end},countable) :- guideline_entity(actual,X,{next},countable).\n"
                ),
                vec![format!("guideline_entity(actual,A,{end},countable)")],
            ),
            1 => (
                format!(
                    "guideline_arg(actual,{a},{role},{a}).\nguideline_arg(actual,{a},{other},{b}).\nguideline_exclusion({doc}) :- guideline_arg(actual,X,{role},X).\n"
                ),
                vec![
                    format!("guideline_arg(actual,A,{role},A)"),
                    format!("guideline_arg(actual,A,{other},A)"),
                ],
            ),
            2 => (
                format!(
                    "guideline_arg({a},{b},{role},{c}).\nguideline_arg(actual,{b},{other},{a}).\nguideline_exclusion({doc}) :- guideline_arg({a},X,{role},Y), guideline_arg(actual,X,{other},Y).\n"
                ),
                vec![
                    format!("guideline_exclusion({doc})"),
                    format!("guideline_arg({a},A,{role},B)"),
                ],
            ),
            3 => {
                patient = Some(format!(
                    "guideline_arg(actual,p_a_{salt:x},{role},p_b_{salt:x}).\nguideline_entity(actual,p_b_{salt:x},{noun},countable).\n"
                ));
                (
                    format!(
                        "guideline_entity(actual,X,{end},countable) :- guideline_arg(actual,X,{role},Y), guideline_entity(actual,Y,{noun},countable).\n"
                    ),
                    vec![
                        format!("guideline_entity(actual,A,{end},countable)"),
                        format!("guideline_entity(actual,p_a_{salt:x},{end},countable)"),
                    ],
                )
            }
            4 => {
                let target = if salt & 2 == 0 { &b } else { &c };
                (
                    format!(
                        "guideline_entity(actual,{a},{noun},countable).\nguideline_arg(actual,{a},{role},{b}).\nguideline_entity(actual,{target},{next},countable).\nguideline_exclusion({doc}) :- guideline_entity(actual,X,{noun},countable), \\+ (guideline_arg(actual,X,{role},Y), guideline_entity(actual,Y,{next},countable)).\n"
                    ),
                    vec![
                        format!("guideline_exclusion({doc})"),
                        format!(
                            "guideline_entity(actual,A,{noun},countable), \\+ (guideline_arg(actual,A,{role},B), guideline_entity(actual,B,{next},countable))"
                        ),
                    ],
                )
            }
            5 => {
                let made = identity(&doc, 2, "X");
                let nested = identity(&doc, 3, "X");
                (
                    format!(
                        "guideline_entity(actual,{a},{noun},countable).\nguideline_entity(actual,X,{next},countable) :- guideline_entity(actual,X,{noun},countable).\nguideline_entity(actual,{made},{end},countable) :- guideline_entity(actual,X,{next},countable).\nguideline_entity(actual,{nested},final,countable) :- guideline_entity(actual,X,{end},countable).\n"
                    ),
                    vec![
                        format!("guideline_entity(actual,A,{end},countable)"),
                        "guideline_entity(actual,A,final,countable)".into(),
                    ],
                )
            }
            6 => {
                let fact = if salt & 2 == 0 {
                    format!("guideline_exclusion(e_{doc}).\n")
                } else {
                    String::new()
                };
                (
                    format!(
                        "guideline_entity(actual,{a},{noun},countable).\n{fact}guideline_exclusion(d_{doc}) :- guideline_entity(actual,X,{noun},countable), \\+ guideline_exclusion(e_{doc}).\nguideline_exclusion(f_{doc}) :- guideline_entity(actual,X,{noun},countable), \\+ guideline_exclusion(d_{doc}).\n"
                    ),
                    vec![
                        format!("guideline_exclusion(d_{doc})"),
                        format!("guideline_exclusion(f_{doc})"),
                    ],
                )
            }
            _ => (
                format!(
                    "guideline_entity(actual,{a},{noun},countable).\nguideline_entity(actual,X,{next},countable) :- guideline_entity(actual,X,{noun},countable).\nguideline_entity(actual,X,{noun},countable) :- guideline_entity(actual,X,{next},countable).\n"
                ),
                vec![format!("guideline_entity(actual,A,{next},countable)")],
            ),
        };
        let mut documents = vec![(text, false)];
        if let Some(p) = patient {
            documents.push((p, true));
        }
        out.push(Case {
            name: format!("generated-{i:03}"),
            documents,
            queries,
        });
    }
    for (name, text) in [
        (
            "inactive-c17",
            include_str!("../fixtures/inactive-c17.pl.in"),
        ),
        ("active-c17", include_str!("../fixtures/active-c17.pl.in")),
        (
            "inactive-c18",
            include_str!("../fixtures/inactive-c18.pl.in"),
        ),
        ("active-c18", include_str!("../fixtures/active-c18.pl.in")),
        (
            "forwarding-k1-with-noncyclic-complement",
            crate::plant_tests::MIXED,
        ),
        (
            "patient-identity-root",
            "guideline_entity(actual,X,out,countable) :- guideline_entity(actual,X,seed,countable).\n",
        ),
        (
            "known-origin-root",
            "guideline_entity(actual,'$guideline_id'(product,known,1,ref(1),[]),out,countable).\n",
        ),
        (
            "nonleast-fact-only",
            "guideline_entity(actual,'$guideline_id'(product,known,1,ref(1),[]),seed,countable).\n",
        ),
        ("empty-program", ""),
        (
            "correlated-naf-alias",
            "guideline_arg(actual,a,1,b).\nguideline_entity(actual,c,blocked,countable).\nguideline_exclusion(k) :- \\+ (guideline_arg(actual,a,1,X), guideline_entity(actual,X,blocked,countable)).\n",
        ),
    ] {
        let mut documents = vec![(text.into(), false)];
        let mut queries = vec!["guideline_entity(actual,A,out,countable)".into()];
        if name == "patient-identity-root" {
            documents.push(("guideline_entity(actual,'$guideline_id'(product,patient_world,1,ref(1),[]),seed,countable).\n".into(),true));
            queries.push("guideline_entity(actual,'$guideline_id'(product,patient_world,1,ref(1),[]),out,countable)".into());
        }
        if name == "known-origin-root" {
            queries.push(
                "guideline_entity(actual,'$guideline_id'(product,known,1,ref(1),[]),out,countable)"
                    .into(),
            );
        }
        if name == "forwarding-k1-with-noncyclic-complement" {
            queries = vec![
                "guideline_entity(actual,A,z,countable)".into(),
                "guideline_entity(actual,A,aux,countable)".into(),
            ];
        }
        if name == "correlated-naf-alias" {
            queries.push("guideline_exclusion(k)".into());
        }
        out.push(Case {
            name: name.into(),
            documents,
            queries,
        });
    }
    assert_eq!(out.len(), 266);
    out
}
fn source(case: &Case) -> sb1_ta::source::Program {
    let mut p = sb1_ta::source::Program::new();
    for (i, (text, patient)) in case.documents.iter().enumerate() {
        p.add(format!("doc-{i}").into(), text.clone(), *patient)
            .unwrap();
    }
    sb1_checker::program::admit(&p.arena, &p.clauses).unwrap();
    p
}
fn hint(case: &Case) -> (sb1_ta::source::Program, sb1_ta::hints::Hints) {
    let p = source(case);
    let h = sb1_ta::hints::export(&p, sb1_ta::hints::analyze(&p).unwrap()).unwrap();
    (p, h)
}
fn top(p: &sb1_ta::source::Program, h: &mut sb1_ta::hints::Hints) {
    let value = |t| {
        if matches!(p.arena.node(t), sb1_checker::term::Node::Var(_)) {
            return Value::Any;
        }
        if let Some(a) = p.arena.named(t, "$guideline_id", 5) {
            return Value::Origin(
                h.model
                    .origins
                    .iter()
                    .position(|o| *o == [a[0], a[1], a[2], a[3]])
                    .unwrap(),
            );
        }
        Value::Exact(t)
    };
    let mut values = BTreeSet::from([Value::Any, Value::Number, Value::Patient]);
    values.extend((0..h.model.origins.len()).map(Value::Origin));
    for t in 0..p.arena.nodes.len() {
        if matches!(
            p.arena.node(t),
            sb1_checker::term::Node::Atom(_) | sb1_checker::term::Node::Integer(_)
        ) {
            values.insert(Value::Exact(t));
        }
    }
    let mut features = BTreeSet::new();
    let mut labels = BTreeSet::new();
    for c in &p.clauses {
        let (pred, a) = p.arena.call(c.head).unwrap();
        let name = p.arena.name(pred);
        let own = match name {
            "guideline_entity" => Some(Feature::Noun(value(a[2]))),
            "guideline_event" => Some(Feature::Lemma(value(a[2]))),
            "guideline_operator" => Some(Feature::Operator(value(a[2]))),
            "guideline_property" => Some(Feature::Property(value(a[2]))),
            _ => None,
        };
        if let Some(f) = own {
            features.insert(f);
        }
        if !matches!(
            name,
            "guideline_schema_version"
                | "guideline_document"
                | "guideline_supersession"
                | "guideline_exclusion"
                | "guideline_evaluation_date"
        ) {
            features.insert(Feature::Payload(
                pred,
                own.map_or(Value::Any, |_| value(a[2])),
            ));
        }
        if matches!(name, "guideline_arg" | "guideline_pp") {
            labels.insert(Label(pred, value(a[2])));
        }
    }
    let root = h.model.sets.len();
    h.model.sets.push(Some(std::rc::Rc::from(
        values.into_iter().collect::<Vec<_>>(),
    )));
    let all = h.model.sets.len();
    h.model.sets.push(None);
    h.model.predicates.extend(features.iter().filter_map(|f| {
        if let Feature::Payload(predicate, _) = f {
            Some(*predicate)
        } else {
            None
        }
    }));
    h.model.predicates.sort_unstable();
    h.model.predicates.dedup();
    h.model.unary = features.into_iter().map(|f| (f, root)).collect();
    h.model.rectangles = labels.into_iter().map(|l| (l, root, root, all)).collect();
    let positive = p.clauses.iter().all(|c| {
        c.body
            .iter()
            .all(|g| !matches!(g, sb1_checker::program::Goal::Not(_)))
    });
    let growing = p.clauses.iter().any(|c| {
        fn visit(a: &sb1_checker::term::Arena, t: usize) -> bool {
            if let Some(x) = a.named(t, "$guideline_id", 5) {
                return !a.list(x[4]).unwrap().is_empty();
            }
            a.call(t)
                .is_some_and(|(_, xs)| xs.iter().any(|t| visit(a, *t)))
        }
        visit(&p.arena, c.head)
    });
    if positive && !growing {
        h.clause_ranks = Some(vec![1; p.clauses.len()]);
        for row in &mut h.flow.sources {
            row.sources = std::rc::Rc::from((0..h.flow.nodes.len()).collect::<Vec<_>>());
        }
        h.flow.ranks = Some(vec![1; h.flow.nodes.len()]);
    }
}
fn edges(g: &core::Csr) -> BTreeSet<(usize, usize, bool)> {
    (0..g.nodes())
        .flat_map(|i| {
            g.row(i)
                .iter()
                .map(move |e| (i, (e >> 1) as usize, e & 1 != 0))
        })
        .collect()
}
fn unary(m: &core::May) -> BTreeSet<(u8, u32, u32, u32)> {
    m.unary_rows()
        .flat_map(|(f, s)| {
            m.sets
                .values(s)
                .unwrap()
                .iter()
                .map(|v| (f.kind, f.predicate, f.selector, *v))
                .collect::<Vec<_>>()
        })
        .collect()
}
fn templates(p: &core::Program) -> Vec<Template> {
    p.clauses
        .iter()
        .map(|c| Template::from(&p.arena, c))
        .collect()
}
fn qbytes(text: &str) -> Vec<u8> {
    let template = include_str!("../../../../bench/sb/queries/q-one.pl.in");
    let header = template.lines().take(3).collect::<Vec<_>>().join("\n");
    format!(
        "{header}\n'$guideline_query_projection'(goal({}),answers([])).\n",
        ground::canonical_goal(text)
    )
    .into_bytes()
}
fn verify_answers(case: &Case, c: &mut core::analysis::Checked, oracle: &Grounded) -> usize {
    let (l, u) = oracle.wfm();
    assert_eq!(
        l, u,
        "accepted certificate over non-total independent WFM: {}",
        case.name
    );
    let stable = oracle.program.stable_models();
    assert_eq!(stable.len(), 1, "{}", case.name);
    assert_eq!(stable[0], oracle.program.wfm().0);
    let all_templates = templates(&c.program);
    let mut count = 0;
    for text in &case.queries {
        let selected = c
            .select(&qbytes(text), &mut core::Budget::default())
            .unwrap_or_else(|e| panic!("{} query {text}: {e:?}", case.name));
        let mut chosen = selected.into_iter().collect::<BTreeSet<_>>();
        chosen.extend((0..c.program.clauses.len()).filter(|i| c.program.patient[*i]));
        let slice = Grounded::new(
            &all_templates
                .iter()
                .enumerate()
                .filter(|(i, _)| chosen.contains(i))
                .map(|(_, t)| t.clone())
                .collect::<Vec<_>>(),
        );
        let query = ground::query(text);
        assert_eq!(
            oracle.answers(&query),
            slice.answers(&query),
            "{} full/sliced exact answers: {text}",
            case.name
        );
        count += 1;
    }
    assert_eq!(
        c.select(b"sb1-q-all 0\n", &mut core::Budget::default())
            .unwrap()
            .len(),
        c.program.patient.iter().filter(|p| !**p).count()
    );
    count + 1
}
fn refined_map(
    c: &core::analysis::Checked,
    d: &core::analysis::Diagnostics,
    oracle: &Grounded,
) -> usize {
    let Some((r, _)) = &d.refined else {
        return 0;
    };
    let origin = |o: usize| c.model.abs.origins[o].map(|t| Term::from(&c.program.arena, t));
    let mut count = 0;
    for value in &oracle.values {
        let Some(o) = value.origin() else {
            continue;
        };
        let Some(index) = (0..c.model.abs.origins.len()).find(|i| origin(*i) == o) else {
            continue;
        };
        let Term::Call(_, a) = value else {
            unreachable!()
        };
        let tags = if r.registry.precise[index] {
            a[4].list()
                .unwrap()
                .iter()
                .map(|t| {
                    t.origin()
                        .and_then(|o| (0..c.model.abs.origins.len()).find(|i| origin(*i) == o))
                        .map_or(core::refinement::Tag::Scalar, core::refinement::Tag::Origin)
                })
                .collect()
        } else {
            vec![]
        };
        assert!(
            r.registry.keys.iter().any(|k| k.origin == index
                && k.precise == r.registry.precise[index]
                && k.deps == tags),
            "missing concrete→mixed type for {value:?} tags={tags:?}"
        );
        count += 1;
    }
    count
}
fn malformed(case: &Case, kind: &str) -> bool {
    let (p, mut h) = hint(case);
    let applies = match kind {
        "unseeded" if !h.model.unary.is_empty() => {
            h.model.unary.clear();
            true
        }
        "rank" if !p.clauses.is_empty() => {
            h.clause_ranks = Some(vec![0; p.clauses.len()]);
            true
        }
        "source" => {
            let at = h.flow.sources.iter().position(|r| {
                let mut vars = BTreeSet::new();
                p.arena.vars(p.clauses[r.clause].head, &mut vars);
                vars.contains(&r.variable)
            });
            if let Some(at) = at {
                h.flow.sources.remove(at);
                true
            } else {
                false
            }
        }
        "tag" if h.refined.is_some() => {
            h.refined.as_mut().unwrap().registry.pop();
            true
        }
        _ => false,
    };
    if !applies {
        return false;
    }
    let p = producer::program(p);
    let result = producer::decode(&p, h)
        .and_then(|h| core::analysis::check(p, h, &mut core::Budget::default()));
    assert!(
        matches!(result, Err(core::Failure::Certificate(_))),
        "{} {kind} malformed hint failed to refuse as certificate",
        case.name
    );
    true
}

fn post_hoc_declared_payload_control() -> (usize, usize) {
    let case = Case {
        name: "posthoc-v1-case003-undeclared-payload".into(),
        documents: vec![
            (
                include_str!("../fixtures/t4-v1-case003-kb.pl.in").into(),
                false,
            ),
            (
                include_str!("../fixtures/t4-v1-case003-patient.pl.in").into(),
                true,
            ),
        ],
        queries: vec![],
    };
    let raw = source(&case);
    let shape = (
        raw.clauses.len(),
        Grounded::new(
            &raw.clauses
                .iter()
                .map(|c| Template::from(&raw.arena, c))
                .collect::<Vec<_>>(),
        )
        .atoms
        .len(),
    );
    for (round, malformed) in [false, true, false].iter().enumerate() {
        let (p, mut h) = hint(&case);
        let original_predicates = h.model.predicates.clone();
        top(&p, &mut h);
        if *malformed {
            h.model.predicates = original_predicates;
        }
        if let Some(out) = std::env::var_os("SB1_TAS_T4_OUT") {
            let out = std::path::PathBuf::from(out);
            for (i, (text, _)) in case.documents.iter().enumerate() {
                std::fs::write(out.join(format!("{}-{i}.pl.in", case.name)), text).unwrap();
            }
            let mut file =
                std::fs::File::create(out.join(format!("{}-{round}-hints.txt", case.name)))
                    .unwrap();
            h.write_canonical(&p, &mut file).unwrap();
            let predicates = h
                .model
                .predicates
                .iter()
                .map(|i| format!("{i}\t{}\n", p.arena.name(*i)))
                .collect::<String>();
            std::fs::write(
                out.join(format!("{}-{round}-declared-predicates.tsv", case.name)),
                predicates,
            )
            .unwrap();
        }
        let p = producer::program(p);
        let result = producer::decode(&p, h)
            .and_then(|h| core::analysis::check(p, h, &mut core::Budget::default()));
        if *malformed {
            assert!(
                matches!(result, Err(core::Failure::Certificate(ref detail)) if detail == "payload predicate not declared"),
                "post hoc grammar control must refuse the undeclared payload, not claim a cycle"
            );
        } else {
            assert!(
                result.is_ok(),
                "declared-predicate positive/restored control"
            );
        }
    }
    shape
}

#[test]
fn t4_arbitrary_hints_against_independent_ground_semantics() {
    let manifest = include_str!("../fixtures/t4-generator.json");
    assert!(manifest.contains(&format!("\"seed_u64\": \"{SEED}\"")));
    let mut summary = String::from(
        "case\tkind\tclauses\tatoms\tF_edges\tFprime_edges\tcanonical\tcoarse\tqueries\ttype_map\tmalformed\tunseeded_refused\trank_refused\tsource_refused\ttag_refused\tdeclared_predicate_refused\n",
    );
    let (mut accepted, mut coarse, mut strict_coarse, mut queries, mut mapping, mut perturbations) =
        (0, 0, 0, 0, 0, 0);
    for case in cases() {
        if let Some(out) = std::env::var_os("SB1_TAS_T4_OUT") {
            let out = std::path::PathBuf::from(out);
            for (i, (text, _)) in case.documents.iter().enumerate() {
                std::fs::write(out.join(format!("{}-{i}.pl.in", case.name)), text).unwrap();
            }
        }
        let (p, h) = hint(&case);
        let p = producer::program(p);
        let cert = producer::decode(&p, h).unwrap();
        let oracle = Grounded::new(&templates(&p));
        core::check_envelope(&p, &cert.model, &mut core::Budget::default()).unwrap();
        let base_edges =
            edges(&core::enumerate_signed(&p, &cert.model, &mut core::Budget::default()).unwrap());
        let (pp, mut hh) = hint(&case);
        top(&pp, &mut hh);
        let pp = producer::program(pp);
        let cc = producer::decode(&pp, hh).unwrap();
        core::check_envelope(&pp, &cc.model, &mut core::Budget::default())
            .unwrap_or_else(|e| panic!("{} top hint is not closed: {e:?}", case.name));
        let original_unary = unary(&cert.model);
        let coarse_unary = unary(&cc.model);
        assert!(
            original_unary.is_subset(&coarse_unary),
            "{} F-prime lost unary incidences",
            case.name
        );
        let strict = original_unary != coarse_unary;
        for row in &cert.model.rectangles {
            for owner in cert.model.sets.values(row.owner).unwrap().iter() {
                for from in cert.model.sets.values(row.source).unwrap().iter() {
                    assert!(
                        cc.model
                            .rectangles
                            .iter()
                            .any(|r| r.predicate == row.predicate
                                && r.selector == row.selector
                                && cc.model.sets.has(r.owner, *owner)
                                && cc.model.sets.has(r.source, *from)
                                && r.target == core::ALL),
                        "{} F-prime lost link rectangle",
                        case.name
                    );
                }
            }
        }
        let larger_edges =
            edges(&core::enumerate_signed(&pp, &cc.model, &mut core::Budget::default()).unwrap());
        assert!(
            base_edges.is_subset(&larger_edges),
            "{} F-prime lost complete edges: {:?}",
            case.name,
            base_edges.difference(&larger_edges).collect::<Vec<_>>()
        );
        let mut state = "refused";
        let mut case_queries = 0;
        let mut type_map = 0;
        match core::analysis::check_diagnostic(p, cert, &mut core::Budget::default()) {
            Ok((mut c, d)) => {
                accepted += 1;
                state = "admitted";
                type_map = refined_map(&c, &d, &oracle);
                mapping += type_map;
                case_queries += verify_answers(&case, &mut c, &oracle);
            }
            Err(core::Failure::Cycle { .. }) => {}
            Err(e) => panic!("{} solver-hint/checker outcome {e:?}", case.name),
        }
        let coarse_state = match core::analysis::check(pp, cc, &mut core::Budget::default()) {
            Ok(mut c) => {
                coarse += 1;
                strict_coarse += usize::from(strict);
                case_queries += verify_answers(&case, &mut c, &oracle);
                "admitted"
            }
            Err(core::Failure::Certificate(_) | core::Failure::Cycle { .. }) => "refused",
            Err(e) => panic!("{} coarse hint resource/admission outcome {e:?}", case.name),
        };
        let mut by_kind = [0; 4];
        for (i, kind) in ["unseeded", "rank", "source", "tag"].iter().enumerate() {
            by_kind[i] = usize::from(malformed(&case, kind));
        }
        let bad = by_kind.iter().sum::<usize>();
        perturbations += bad;
        queries += case_queries;
        summary.push_str(&format!(
            "{}\tprospective\t{}\t{}\t{}\t{}\t{state}\t{coarse_state}\t{case_queries}\t{type_map}\t{bad}\t{}\t{}\t{}\t{}\t0\n",
            case.name,
            source(&case).clauses.len(),
            oracle.atoms.len(),
            base_edges.len(),
            larger_edges.len(),
            by_kind[0], by_kind[1], by_kind[2], by_kind[3]
        ));
        if let Some(out) = std::env::var_os("SB1_TAS_T4_OUT") {
            let out = std::path::PathBuf::from(out);
            std::fs::write(out.join("cases.tsv"), &summary).unwrap();
        }
    }
    let (clauses, atoms) = post_hoc_declared_payload_control();
    perturbations += 1;
    summary.push_str(&format!("posthoc-v1-case003-undeclared-payload\tposthoc-hint-grammar\t{clauses}\t{atoms}\tNA\tNA\tnot-graded\trefused-certificate\t0\t0\t1\t0\t0\t0\t0\t1\n"));
    if let Some(out) = std::env::var_os("SB1_TAS_T4_OUT") {
        std::fs::write(std::path::PathBuf::from(out).join("cases.tsv"), &summary).unwrap();
    }
    assert!(
        accepted > 0
            && coarse > 0
            && strict_coarse > 0
            && queries > 0
            && mapping > 0
            && perturbations > 0
    );
    println!(
        "T4 PASS cases=267 prospective=266 post_hoc_grammar=1 canonical_admitted={accepted} coarse_admitted={coarse} strict_nonleast_closed_admitted={strict_coarse} exact_query_checks={queries} concrete_mixed_types={mapping} malformed_refusals={perturbations}"
    );
}

// Separate mode controls; the frozen T4-v2 generator/case set stays unchanged.
#[test]
fn mode_controls_cover_base_and_full_k1_admission() {
    let base = cases()
        .into_iter()
        .find(|c| c.name == "generated-005")
        .unwrap();
    let (p, h) = hint(&base);
    assert!(h.refined.is_none());
    let p = producer::program(p);
    let oracle = Grounded::new(&templates(&p));
    let cert = producer::decode(&p, h).unwrap();
    let (mut c, d) =
        core::analysis::check_diagnostic(p, cert, &mut core::Budget::default()).unwrap();
    assert!(d.refined.is_none() && !d.base_graph.targets.is_empty());
    let base_checks = verify_answers(&base, &mut c, &oracle);

    let full = Case {
        name: "MODE-full-k1-forwarding".into(),
        documents: vec![(crate::plant_tests::MIXED.into(), false)],
        queries: vec![
            "guideline_entity(actual,A,z,countable)".into(),
            "guideline_entity(actual,A,aux,countable)".into(),
        ],
    };
    let p = source(&full);
    let mut a = sb1_ta::hints::analyze(&p).unwrap();
    a.refined = Some(
        sb1_ta::exact_k1::Refined::build_full(&p, &mut a.model, &a.growth, false, false).unwrap(),
    );
    assert!(a.refined.as_ref().unwrap().full_refinement && a.admitted());
    let h = sb1_ta::hints::export(&p, a).unwrap();
    let p = producer::program(p);
    let oracle = Grounded::new(&templates(&p));
    let cert = producer::decode(&p, h).unwrap();
    let (mut c, d) =
        core::analysis::check_diagnostic(p, cert, &mut core::Budget::default()).unwrap();
    let r = &d.refined.as_ref().unwrap().0;
    assert!(r.registry.full_refinement && r.registry.precise.iter().all(|v| *v));
    let mapped = refined_map(&c, &d, &oracle);
    let full_checks = verify_answers(&full, &mut c, &oracle);
    assert!(mapped > 0);
    println!(
        "MODE PASS base-nested-forwarding query_checks={base_checks}; full-k1-existing-reference query_checks={full_checks} concrete_types={mapped}; automatic producer fallback selection not claimed"
    );
}
