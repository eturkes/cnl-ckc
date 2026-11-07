//! C0/C5 differential domain; the explicit oracle owns every expected object.
use crate::complete;
use crate::domain::Counters;
use crate::exact;
use crate::exact_growth;
use crate::faults::{Guard, Kind};
use crate::graph::Question;
use crate::source::Program;
use sb1_checker::program::{self, Goal};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

const LOAD: [&str; 11] = [
    "front-values.tsv",
    "front-unary.tsv",
    "front-links.tsv",
    "front-nodes.tsv",
    "front-edges.tsv",
    "front-scc.tsv",
    "flow-base-nodes.tsv",
    "flow-base-edges.tsv",
    "flow-final-nodes.tsv",
    "flow-final-edges.tsv",
    "flow-outcome.tsv",
];
const SLICES: [&str; 3] = ["slice-q_all.tsv", "slice-q_one.tsv", "slice-q_patient.tsv"];
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!(
                "qualification-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}
fn program(text: &str, patient: bool) -> Program {
    let mut p = Program::new();
    p.add("knowledge".into(), text.into(), false).unwrap();
    if patient {
        p.add(
            "patient".into(),
            concat!(
                "guideline_entity(actual,p,person,countable).\n",
                "guideline_entity(actual,7,number,countable).\n",
                "guideline_entity(actual,rational(1,2),number,countable).\n",
                "guideline_arg(actual,p,1,7).\n"
            )
            .into(),
            true,
        )
        .unwrap();
    }
    program::admit(&p.arena, &p.clauses).unwrap();
    p.validate_patient_boundary().unwrap();
    p
}
fn questions(p: &mut Program) -> Vec<Question> {
    let all = Question::parse(&mut p.arena, b"sb1-q-all 0\n").unwrap();
    let first = p
        .clauses
        .iter()
        .find(|c| !c.body.is_empty())
        .unwrap_or(&p.clauses[0]);
    let one = Question::Goals {
        goals: vec![Goal::Positive(first.head)],
        vars: first.names.len(),
    };
    let patient = p
        .clauses
        .iter()
        .enumerate()
        .find(|(i, _)| p.patient(*i))
        .map_or(&p.clauses[0], |(_, c)| c);
    vec![
        all,
        one,
        Question::Goals {
            goals: vec![Goal::Positive(patient.head)],
            vars: patient.names.len(),
        },
    ]
}
fn candidate(p: &Program, oracle: &crate::Analysis, audit: bool) -> complete::Analysis {
    let mut m = exact::Model::build(p, None, true, Some(oracle.model.abs.clone()), audit);
    let graph = crate::exact_graph::build_compressed(p, &mut m, audit);
    let growth = exact_growth::Growth::build(p, &mut m, audit, audit);
    complete::Analysis::from_parts(p, m, graph, growth, audit, audit).unwrap()
}
fn snapshots(
    p: &Program,
    q: &[Question],
    oracle: &crate::Analysis,
    exact: &complete::Analysis,
    out: &Path,
) {
    let left = out.join("oracle");
    let right = out.join("candidate");
    std::fs::create_dir_all(&left).unwrap();
    std::fs::create_dir_all(&right).unwrap();
    crate::front::features(p, &oracle.model.abs, oracle.model.exact_dump(), &left).unwrap();
    crate::front::features(p, &exact.model.abs, exact.model.dump(), &right).unwrap();
    crate::front::graph(p, &oracle.graph, &left).unwrap();
    crate::front::graph_rows(p, |i| exact.graph.row(i), &exact.graph.components, &right).unwrap();
    complete::oracle_dump(p, oracle, &left).unwrap();
    exact.dump(p, &right).unwrap();
    std::fs::write(left.join("verdict.txt"), oracle.admitted().to_string()).unwrap();
    std::fs::write(right.join("verdict.txt"), exact.admitted().to_string()).unwrap();
    for (query, name) in q.iter().zip(["q_all", "q_one", "q_patient"]) {
        if oracle.admitted() {
            complete::slice_dump(p, &oracle.slice(p, query).clauses, &left, name).unwrap();
        } else {
            std::fs::write(
                left.join(format!("slice-{name}.tsv")),
                "not-executed: refused\n",
            )
            .unwrap();
        }
        if exact.admitted() {
            complete::slice_dump(
                p,
                &exact.slice(p, query, false, false).clauses,
                &right,
                name,
            )
            .unwrap();
        } else {
            std::fs::write(
                right.join(format!("slice-{name}.tsv")),
                "not-executed: refused\n",
            )
            .unwrap();
        }
    }
}
fn deltas(out: &Path) -> Vec<(String, String, String)> {
    LOAD.into_iter()
        .chain(SLICES)
        .chain(["verdict.txt"])
        .filter_map(|name| {
            let expected = std::fs::read_to_string(out.join("oracle").join(name)).unwrap();
            let actual = std::fs::read_to_string(out.join("candidate").join(name)).unwrap();
            (expected != actual).then_some((name.into(), expected, actual))
        })
        .collect()
}
fn compare(text: &str, patient: bool) -> bool {
    let mut p = program(text, patient);
    let q = questions(&mut p);
    let oracle = crate::Analysis::load(&p).unwrap();
    let exact = candidate(&p, &oracle, true);
    // Query diagnostics intern scratch; D4's retention boundary is the closed load.
    let roots = exact.model.retention();
    assert_eq!(roots[0].1, roots[1].1);
    assert_eq!(roots[2].1, roots[3].1);
    let out = Directory::new();
    snapshots(&p, &q, &oracle, &exact, &out.0);
    assert!(
        deltas(&out.0).is_empty(),
        "complete C3 differential\n{text}"
    );
    oracle.admitted()
}

#[test]
fn fixed_seed_complete_c3_objects_and_verdicts() {
    let mut admitted = 0;
    let mut rng = 0x6d1a_58c3_f291_047bu64;
    let mut next = |n| {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng as usize % n
    };
    for case in 0..256 {
        let count = 2 + next(4);
        let id = |i| format!("'$guideline_id'(product,d,1,ref({}),[])", i + 1);
        let mut lines = Vec::new();
        for i in 0..count {
            lines.push(format!(
                "guideline_entity(actual,{},n{},countable).",
                id(i),
                next(3)
            ));
            lines.push(format!(
                "guideline_arg({},{},{},{}).",
                if next(2) == 0 {
                    "actual".into()
                } else {
                    id(next(count))
                },
                id(i),
                1 + next(2),
                id(next(count))
            ));
        }
        for _ in 0..3 {
            lines.push(format!("guideline_entity(actual,A,n{},countable) :- guideline_entity(actual,A,n{},countable), guideline_arg(actual,A,{},B), guideline_entity(actual,B,n{},countable).", next(3), next(3), 1 + next(2), next(3)));
        }
        lines.push(format!("guideline_arg(A,A,{},B) :- guideline_entity(actual,A,n{},countable), guideline_entity(actual,B,n{},countable).", 1 + next(2), next(3), next(3)));
        lines.push(format!("guideline_entity(actual,'$guideline_id'(product,d,2,ref(1),[A]),child,countable) :- guideline_entity(actual,A,n{},countable).", next(3)));
        lines.push(format!("guideline_operator(actual,'$guideline_id'(context,d,3,box(1),[A]),{}) :- guideline_entity(actual,A,n{},countable), \\+ (guideline_arg(actual,A,{},B), guideline_entity(actual,B,n{},countable)).", ["should", "must", "may", "-"][next(4)], next(3), 1 + next(2), next(3)));
        lines.push(format!("guideline_entity(actual,'$guideline_id'(context,d,3,box(1),[A]),boxed,countable) :- guideline_entity(actual,A,n{},countable).", next(3)));
        lines.push(format!(
            "guideline_arg(actual,p,1,{}) :- guideline_entity(actual,p,person,countable).",
            id(next(count))
        ));
        lines.push(format!(
            "guideline_cardinality(actual,{},na,exactly,2).",
            id(0)
        ));
        if case % 3 == 0 {
            lines.push(format!("guideline_cardinality(actual,A,na,S,Y) :- guideline_entity(actual,A,n{},countable), guideline_cardinality(actual,A,na,C,X), guideline_product(C,X,exactly,2,S,Y).", next(3)));
        }
        if case % 7 == 0 {
            lines.push("guideline_exclusion(k) :- \\+ guideline_exclusion(j).".into());
            lines.push("guideline_exclusion(j) :- guideline_exclusion(k).".into());
        }
        for i in (1..lines.len()).rev() {
            let j = next(i + 1);
            lines.swap(i, j);
        }
        admitted += usize::from(compare(&(lines.join("\n") + "\n"), true));
    }
    for text in [
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
    ] {
        admitted += usize::from(compare(text, true));
    }
    eprintln!(
        "C0: 256 fixed-seed complete programs + 13 named cases; admitted={admitted}, refused={}; all load C3 objects/verdicts; slices on admitted programs",
        269 - admitted
    );
}

fn fixture(name: &str) -> (String, &'static str, Kind) {
    match name {
        "hub" => ("guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),person,countable).\nguideline_exclusion(k) :- guideline_entity(actual,A,person,countable).\n".into(), "front-edges.tsv", Kind::HubTarget),
        "rectangle" | "owner" => (concat!(
            "guideline_arg(actual,p,1,'$guideline_id'(product,d,1,ref(1),[])).\n",
            "guideline_arg('$guideline_id'(context,d,1,box(1),[]),p,2,'$guideline_id'(product,d,1,ref(2),[])).\n").into(), "front-links.tsv", if name == "owner" { Kind::Owner } else { Kind::Rectangle }),
        "unary" => ("guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),person,countable).\n".into(), "front-unary.tsv", Kind::Unary),
        "sign" => ("guideline_exclusion(a) :- \\+ guideline_exclusion(b).\nguideline_exclusion(c) :- guideline_exclusion(b).\nguideline_exclusion(b) :- guideline_exclusion(a).\n".into(), "front-edges.tsv", Kind::Sign),
        "notification" => ("guideline_entity(actual,A,last,countable) :- guideline_entity(actual,A,middle,countable).\nguideline_entity(actual,A,middle,countable) :- guideline_entity(actual,A,seed,countable).\nguideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable).\n".into(), "front-unary.tsv", Kind::Notification),
        "memo" | "collision" => (concat!(
            "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),left,countable).\n",
            "guideline_entity(actual,'$guideline_id'(product,d,1,ref(2),[]),right,countable).\n",
            "guideline_exclusion(k1) :- guideline_entity(actual,A,left,countable).\n",
            "guideline_exclusion(k2) :- guideline_entity(actual,A,right,countable).\n").into(), "front-edges.tsv", if name == "collision" { Kind::HashCollision } else { Kind::MemoAlias }),
        "k1-restricted" => (include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s10-13.pl").into(), "flow-final-edges.tsv", Kind::K1Support),
        "k1-fallback" => (concat!(
            "guideline_entity(actual,'$guideline_id'(product,g1,1,ref(1),[A]),patient,countable) :- guideline_entity(actual,A,patient,countable).\n",
            "guideline_entity(actual,'$guideline_id'(product,g2,1,ref(1),[A]),person,countable) :- guideline_entity(actual,A,person,countable).\n").into(), "flow-final-edges.tsv", Kind::K1Support),
        _ => panic!("qualification case {name}"),
    }
}
#[test]
fn plant_case() {
    let selected = std::env::var("SB1_TA_Q_CASE").ok();
    let cases = selected.as_deref().map_or_else(
        || {
            vec![
                "hub",
                "rectangle",
                "unary",
                "sign",
                "owner",
                "notification",
                "memo",
                "collision",
                "k1-restricted",
                "k1-fallback",
            ]
        },
        |name| vec![name],
    );
    for name in cases {
        let (text, expected, kind) = fixture(name);
        let mut p = program(&text, false);
        let q = questions(&mut p);
        let oracle = crate::Analysis::load(&p).unwrap();
        let planted = std::env::var_os("SB1_TA_Q_PLANT").is_some();
        let _guard = Guard::set(planted.then_some(kind));
        let exact = candidate(&p, &oracle, !planted);
        let owned = Directory::new();
        let out = std::env::var_os("SB1_TA_Q_OUT").map_or_else(|| owned.0.clone(), PathBuf::from);
        snapshots(&p, &q, &oracle, &exact, &out);
        let delta = deltas(&out);
        let hits = crate::faults::hits();
        eprintln!(
            "C5-case {name}: planted={planted} hits={hits} full_k1={} final_edges={}",
            exact.refined.as_ref().is_some_and(|r| r.full_refinement),
            exact
                .refined
                .as_ref()
                .map_or(0, |r| r.growth.graph.edge_count())
        );
        if planted {
            assert!(hits > 0, "plant did not reach its code path: {name}");
            if name == "k1-fallback" {
                assert!(exact.refined.as_ref().is_some_and(|r| r.full_refinement));
                assert!(hits >= 2);
            }
        }
        let mut record = String::from("case\tcomponent\texpected_component\tplant_hits\n");
        for (component, left, right) in &delta {
            record.push_str(&format!("{name}\t{component}\t{expected}\t{hits}\n"));
            std::fs::write(
                out.join(format!("delta-{component}")),
                format!("expected:\n{left}\nactual:\n{right}"),
            )
            .unwrap();
        }
        std::fs::write(out.join("deltas.tsv"), record).unwrap();
        if planted && kind != Kind::HashCollision {
            assert!(
                delta.iter().any(|(component, _, _)| component == expected),
                "plant lacks its expected nonempty C3 delta: {name}"
            );
        }
        assert!(
            delta.is_empty(),
            "C5 {name}: nonempty C3 delta (plant_hits={hits})"
        );
    }
}

#[test]
fn nested_fallback_zero_one_two_matches_the_reference() {
    for count in 0..=2 {
        let mut lines = vec![
            "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable)."
                .to_string(),
        ];
        let mut body = "guideline_entity(actual,A,seed,countable)".to_string();
        for n in 0..count {
            lines.push(format!(
                "guideline_entity(actual,'$guideline_id'(product,d,2,ref({}),[]),known,countable).",
                n + 1
            ));
            body.push_str(&format!(", guideline_entity(actual,'$guideline_id'(product,d,2,ref({}),[A]),known,countable)", n+1));
        }
        lines.push(format!("guideline_entity(actual,'$guideline_id'(product,d,3,ref(1),[A]),out,countable) :- {body}."));
        let p = program(&(lines.join("\n") + "\n"), false);
        let m = crate::features::Model::build(&p, None, true);
        let base = crate::growth::Growth::build(&p, &m, None);
        let mut model = exact::Model::build(&p, None, true, Some(m.abs.clone()), true);
        let growth = exact_growth::Growth::build(&p, &mut model, true, true);
        assert_eq!(growth.precision_fallbacks, base.precision_fallbacks);
        assert_eq!(growth.precision_fallbacks, count);
        if count > 0 {
            let native = crate::k1::Refined::build(&p, &m, &base).err().unwrap();
            let actual = crate::exact_k1::Refined::build(&p, &mut model, &growth, true, true)
                .err()
                .unwrap();
            assert_eq!(actual, native);
        }
    }
}

#[test]
fn witness_branch_cardinalities_zero_one_two_match_native() {
    use crate::domain::{Domain, Value};
    use crate::features::{Expr, Pattern};
    let p = program(
        "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable).\nguideline_entity(actual,'$guideline_id'(product,d,1,ref(2),[]),seed,countable).\n",
        false,
    );
    let native = crate::features::Model::build(&p, None, true);
    let exact = exact::Model::build(&p, None, true, Some(native.abs.clone()), true);
    let pattern = Pattern::new(&p.arena, &native.abs, Vec::new(), 1);
    for count in 0..=2 {
        let domain = Domain::Set((0..count).map(Value::Origin).collect::<BTreeSet<_>>());
        let before = crate::features::State::audit_state(vec![0], vec![domain.clone()]);
        let mut after = exact::State::new(1);
        after.domains[0] = exact.sets.from_domain(&domain);
        let expected = native
            .feature_witness(&p.arena, &pattern, before, &mut Counters::default())
            .unwrap();
        let actual = exact
            .feature_witness(&p.arena, &pattern, after, &mut Counters::default())
            .unwrap();
        assert_eq!(
            exact.sets.domain(actual.domain(Expr::Var(0), &exact.sets)),
            expected.domain(Expr::Var(0))
        );
    }
}
