use crate::finite_oracle::{Program as Finite, Rule, set};
use crate::{core, producer};
use core::analysis::{Certificate, Checked};

const SIGNED: &str = "guideline_entity(actual,a,seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\nguideline_exclusion(d) :- guideline_entity(actual,X,next,countable), \\+ guideline_exclusion(e).\nguideline_exclusion(e).\n";
const GROWTH: &str = "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\nguideline_entity(actual,'$guideline_id'(product,d,2,ref(1),[X]),out,countable) :- guideline_entity(actual,X,next,countable).\n";
pub(crate) const MIXED: &str = "guideline_entity(actual,seed_value,s,countable).\nguideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[X]),z,countable) :- guideline_entity(actual,X,s,countable).\nguideline_entity(actual,'$guideline_id'(product,d,2,ref(1),[X]),w,countable) :- guideline_entity(actual,X,z2,countable).\nguideline_entity(actual,other_value,q,countable).\nguideline_entity(actual,'$guideline_id'(product,d,2,ref(1),[X]),w2,countable) :- guideline_entity(actual,X,q,countable).\nguideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[X]),z,countable) :- guideline_entity(actual,X,w2,countable).\nguideline_entity(actual,X,z2,countable) :- guideline_entity(actual,X,z,countable).\nguideline_entity(actual,'$guideline_id'(product,d,10,ref(1),[X]),aux,countable) :- guideline_entity(actual,X,z,countable).\n";
fn source(text: &str) -> sb1_ta::source::Program {
    let mut p = sb1_ta::source::Program::new();
    p.add("plant".into(), text.into(), false).unwrap();
    p
}
fn proposal(text: &str) -> (core::Program, Certificate) {
    let p = source(text);
    let mut h = sb1_ta::hints::export(&p, sb1_ta::hints::analyze(&p).unwrap()).unwrap();
    let name = std::env::var("SB1_TAS_T2_PLANT").unwrap_or_default();
    if text == SIGNED {
        match name.as_str() {
            "missing-seed" | "missing-transfer" => {
                let noun = if name == "missing-seed" {
                    "seed"
                } else {
                    "next"
                };
                h.model.unary.retain(|(f,_)| !matches!(f,sb1_ta::features::Feature::Noun(sb1_ta::domain::Value::Exact(t)) if p.arena.atom_name(*t)==Some(noun)));
            }
            "rank-zero" => h.clause_ranks.as_mut().unwrap()[0] = 0,
            "rank-missing" => h.clause_ranks = None,
            "weak-positive-rank" => {
                h.clause_ranks.as_mut().unwrap()[0] = h.clause_ranks.as_ref().unwrap()[1] + 1
            }
            "strict-negative-rank" => {
                h.clause_ranks.as_mut().unwrap()[2] = h.clause_ranks.as_ref().unwrap()[3]
            }
            _ => {}
        }
    }
    if text == GROWTH && name == "missing-forwarding-source" {
        h.flow
            .sources
            .iter_mut()
            .find(|r| r.clause == 1 && r.variable == 0)
            .unwrap()
            .sources = std::rc::Rc::from([]);
    }
    if text == MIXED && name == "missing-realizable-tag-tuple" {
        h.refined.as_mut().unwrap().registry.pop();
    }
    if let Some(out) = std::env::var_os("SB1_TAS_T2_DIR") {
        let out = std::path::PathBuf::from(out);
        let label = if text == SIGNED {
            "signed"
        } else if text == GROWTH {
            "growth"
        } else {
            "mixed"
        };
        std::fs::write(out.join(format!("{label}.pl.in")), text).unwrap();
        let mut file = std::fs::File::create(out.join(format!("{label}-hints.txt"))).unwrap();
        h.write_canonical(&p, &mut file).unwrap();
    }
    let p = producer::program(p);
    let cert = producer::decode(&p, h).unwrap_or_else(|e| panic!("T2 decoder outcome {e:?}"));
    (p, cert)
}
fn accept(p: core::Program, c: Certificate) -> Checked {
    core::analysis::check(p, c, &mut core::Budget::default())
        .unwrap_or_else(|e| panic!("T2 checker outcome {e:?}"))
}
fn question(goal: &str) -> Vec<u8> {
    let template = include_str!("../../../../bench/sb/queries/q-one.pl.in");
    let header = template.lines().take(3).collect::<Vec<_>>().join("\n");
    format!("{header}\n'$guideline_query_projection'(goal({goal}),answers([])).\n").into_bytes()
}
fn edges(g: &core::Csr) -> Vec<(usize, usize, bool)> {
    (0..g.nodes())
        .flat_map(|i| {
            g.row(i)
                .iter()
                .map(move |e| (i, (e >> 1) as usize, e & 1 != 0))
        })
        .collect()
}
fn independent_mixed(p: &core::Program, c: &mut Certificate) -> Vec<(String, String)> {
    let mut b = core::Budget::default();
    let r = c.refined.as_mut().unwrap();
    r.flow
        .sources
        .complete_scalars(&r.program, &r.model, &r.flow.types, &mut b)
        .unwrap();
    let g = core::growth::enumerate(&r.program, &r.model, &r.flow.types, &r.flow.sources, &mut b)
        .unwrap();
    let mut out = Vec::new();
    for from in 1..g.nodes() {
        for edge in g.row(from) {
            let to = (edge >> 1) as usize;
            if to != 0 {
                out.push((
                    r.registry
                        .node_label(p, &c.model, &r.flow.types, from)
                        .unwrap(),
                    r.registry
                        .node_label(p, &c.model, &r.flow.types, to)
                        .unwrap(),
                ));
            }
        }
    }
    out.sort();
    out.dedup();
    out
}
fn remove_variant(p: &core::Program, c: &mut Certificate) {
    use core::growth::Sources;
    let r = c.refined.as_mut().unwrap();
    let at = r
        .registry
        .variants
        .iter()
        .position(|v| v.original == Some(2) && !v.tags.is_empty())
        .unwrap();
    r.program.clauses.remove(at);
    r.program.patient.remove(at);
    let mut variants = r.registry.variants.clone();
    variants.remove(at);
    let rows = r
        .flow
        .sources
        .rows
        .iter()
        .filter(|s| s.clause != at)
        .cloned()
        .map(|mut s| {
            if s.clause > at {
                s.clause -= 1;
            }
            s
        })
        .collect();
    r.flow.sources = Sources::new(&r.program, &r.flow.types, rows).unwrap();
    r.registry = core::refinement::Registry::new(
        p,
        &c.model,
        &r.program,
        &r.model,
        r.registry.keys.clone(),
        variants,
        false,
    )
    .unwrap();
}

// Runner executes this exact probe: green → one planted implementation/input
// path red → fresh-process restored green. A false green fails that runner.
#[test]
fn t2_probe() {
    let name = std::env::var("SB1_TAS_T2_PLANT").unwrap_or_default();
    println!("T2 plant={name}");
    let semantic = Finite {
        atoms: 4,
        rules: vec![
            Rule {
                head: 0,
                positive: set(&[]),
                naf: vec![],
                builtins: true,
            },
            Rule {
                head: 1,
                positive: set(&[0]),
                naf: vec![],
                builtins: true,
            },
            Rule {
                head: 2,
                positive: set(&[1]),
                naf: vec![vec![set(&[3])]],
                builtins: true,
            },
            Rule {
                head: 3,
                positive: set(&[]),
                naf: vec![],
                builtins: true,
            },
        ],
    };
    assert_eq!(semantic.wfm(), (set(&[0, 1, 3]), set(&[0, 1, 3])));
    assert_eq!(semantic.stable_models(), vec![set(&[0, 1, 3])]);
    let (p, c) = proposal(SIGNED);
    let mut checked = accept(p, c);
    let actual = edges(&checked.graph);
    let expected = vec![(1, 0, false), (2, 1, false), (2, 3, true)];
    println!("T2 signed expected={expected:?} actual={actual:?}");
    assert_eq!(actual, expected, "nonempty signed/CSR object delta");
    assert_eq!(
        checked
            .select(
                &question("guideline_entity(actual,patient_value,next,countable)"),
                &mut core::Budget::default()
            )
            .unwrap(),
        vec![0, 1],
        "nonempty root dependency delta"
    );
    assert_eq!(
        checked
            .select(
                &question("\\+(guideline_exclusion(e))"),
                &mut core::Budget::default()
            )
            .unwrap(),
        vec![3],
        "nonempty NAF-root dependency delta; full model has e"
    );
    let (p, c) = proposal(GROWTH);
    let _ = accept(p, c);
    let (p, mut c) = proposal(MIXED);
    if name == "missing-realizable-clause-variant" {
        remove_variant(&p, &mut c);
    }
    if name == "non-cyclic-growth-rank" || name == "inherited-base-rank" {
        let r = c.refined.as_mut().unwrap();
        r.flow
            .sources
            .complete_scalars(
                &r.program,
                &r.model,
                &r.flow.types,
                &mut core::Budget::default(),
            )
            .unwrap();
        let g = core::growth::enumerate(
            &r.program,
            &r.model,
            &r.flow.types,
            &r.flow.sources,
            &mut core::Budget::default(),
        )
        .unwrap();
        let (from, to) = (1..g.nodes())
            .rev()
            .find_map(|from| {
                g.row(from)
                    .iter()
                    .find(|e| **e >> 1 != 0)
                    .map(|e| (from, (*e >> 1) as usize))
            })
            .unwrap();
        println!(
            "T2 non-cyclic rank from={} to={}",
            r.registry
                .node_label(&p, &c.model, &r.flow.types, from)
                .unwrap(),
            r.registry
                .node_label(&p, &c.model, &r.flow.types, to)
                .unwrap()
        );
        r.flow.ranks.as_mut().unwrap()[from] = r.flow.ranks.as_ref().unwrap()[to];
    }
    let candidate = independent_mixed(&p, &mut c);
    let mut op = source(MIXED);
    let mut om = sb1_ta::exact::Model::build(&op, None, true, None, false);
    let og = sb1_ta::exact_growth::Growth::build(&op, &mut om, false, false);
    sb1_ta::exact_k1::set_full_mixed_dump(true);
    let oracle = sb1_ta::exact_k1::Refined::build(&op, &mut om, &og, false, false).unwrap();
    sb1_ta::exact_k1::set_full_mixed_dump(false);
    let mut expected = oracle.canonical_edges(&op, &om);
    expected.sort();
    expected.dedup();
    println!("T2 complete mixed expected={expected:?} actual={candidate:?}");
    assert_eq!(
        candidate, expected,
        "nonempty complete-mixed complement delta (D1f ii)"
    );
    let _ = accept(p, c);
    // Fresh finite fact program keeps payload roots independent of the operator.
    op = source(
        "guideline_operator(actual,'$guideline_id'(product,payload,1,box(1),[]),should).\nguideline_entity('$guideline_id'(product,payload,1,box(1),[]),a,payload,countable).\nguideline_entity(actual,b,unrelated,countable).\n",
    );
    let h = sb1_ta::hints::export(&op, sb1_ta::hints::analyze(&op).unwrap()).unwrap();
    let p = producer::program(op);
    let c = producer::decode(&p, h).unwrap();
    let mut checked = accept(p, c);
    assert_eq!(
        checked
            .select(
                &question("guideline_operator(actual,A,should)"),
                &mut core::Budget::default()
            )
            .unwrap(),
        vec![0, 1],
        "nonempty deontic payload-root delta"
    );
    let (p, c) = proposal(SIGNED);
    let exhausted = core::enumerate_signed(&p, &c.model, &mut core::Budget { used: 0, limit: 0 });
    assert!(
        matches!(exhausted, Err(core::Failure::Indeterminate(_))),
        "exhausted may-match read as no-edge/admitted"
    );
    println!(
        "T2 control PASS signed=3 mixed={} roots/NAF/payload/resource checked",
        expected.len()
    );
}
