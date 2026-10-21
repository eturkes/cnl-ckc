use crate::k2_engine::EClause;
use crate::k2_index::{EClass, build_index, hash_class, hash_pred, next_candidate};
use crate::k2_term::{ETermArena, push_atom, push_comp, push_int, push_nil, push_var};
use std::collections::BTreeSet;
use std::hint::black_box;
use vstd::prelude::*;

#[derive(Clone, Debug)]
enum Term {
    Var(usize),
    Int(i64),
    Nil,
    Atom(Vec<u8>),
    Comp(Vec<u8>, Vec<Term>),
}

#[derive(Debug, Eq, PartialEq)]
enum Class<'a> {
    Atom(&'a [u8]),
    Comp(&'a [u8], usize),
}

type Goal = (Vec<u8>, Vec<Term>);

fn atom(name: impl AsRef<[u8]>) -> Term {
    Term::Atom(name.as_ref().to_vec())
}

fn comp(name: impl AsRef<[u8]>, args: Vec<Term>) -> Term {
    assert!(!args.is_empty());
    Term::Comp(name.as_ref().to_vec(), args)
}

fn class(term: &Term) -> Option<Class<'_>> {
    match term {
        Term::Atom(name) => Some(Class::Atom(name)),
        Term::Comp(name, args) => Some(Class::Comp(name, args.len())),
        _ => None,
    }
}

// Contract cix D2: independent term model, no index or hash in the oracle.
fn linear(heads: &[Term], goal: &Goal, from: usize) -> Option<usize> {
    heads.iter().enumerate().skip(from).find_map(|(j, head)| {
        let Term::Comp(name, args) = head else {
            return None;
        };
        if *name != goal.0 || args.len() != goal.1.len() {
            return None;
        }
        let matches = match (args.first().and_then(class), goal.1.first().and_then(class)) {
            (Some(head), Some(goal)) => head == goal,
            _ => true,
        };
        matches.then_some(j)
    })
}

verus! {

fn integer_root(arena: &mut ETermArena, value: i64) -> usize {
    push_int(
        arena,
        value.to_string().into_bytes(),
        value.unsigned_abs().to_string().into_bytes(),
        value < 0,
        Ghost(value as int),
    )
}

} // verus!
fn put(arena: &mut ETermArena, term: &Term) -> usize {
    match term {
        Term::Var(key) => push_var(arena, *key, format!("V{key}").into_bytes()),
        Term::Int(value) => integer_root(arena, *value),
        Term::Nil => push_nil(arena),
        Term::Atom(name) => push_atom(arena, name.clone()),
        Term::Comp(name, args) => {
            let roots = args.iter().map(|t| put(arena, t)).collect();
            push_comp(arena, name.clone(), roots)
        }
    }
}

fn check_db(heads: &[Term], goals: &[Goal]) -> usize {
    let mut arena = ETermArena { nodes: vec![] };
    let db: Vec<_> = heads
        .iter()
        .map(|head| EClause {
            head: put(&mut arena, head),
            body: vec![],
        })
        .collect();
    let starts: Vec<_> = (0..=heads.len() + 1)
        .chain(std::iter::once(usize::MAX))
        .collect();
    let expected: Vec<Vec<_>> = goals
        .iter()
        .map(|goal| {
            starts
                .iter()
                .map(|&from| linear(heads, goal, from))
                .collect()
        })
        .collect();
    let mut count = 0;
    for front in 0..=db.len() {
        let suffix = db[front..]
            .iter()
            .map(|c| EClause {
                head: c.head,
                body: vec![],
            })
            .collect();
        let ix = build_index(&arena, &suffix);
        for (q, (name, args)) in goals.iter().enumerate() {
            let roots = args.iter().map(|t| put(&mut arena, t)).collect();
            for (k, &from) in starts.iter().enumerate() {
                assert_eq!(
                    next_candidate(&arena, &db, &ix, front, name, &roots, from),
                    expected[q][k],
                    "front={front} from={from} goal={name:?}/{args:?} heads={heads:?}"
                );
                count += 1;
            }
        }
    }
    count
}

fn goal(name: impl AsRef<[u8]>, first: Term) -> Goal {
    (name.as_ref().to_vec(), vec![first])
}

#[test]
fn cix2_t4_many_predicates_every_front_and_resume() {
    let mut count = 0;
    for width in [1, 8, 32, 64] {
        let mut heads = vec![];
        let mut goals = vec![];
        for i in 0..width {
            let name = format!("p{i:03}");
            heads.push(comp(&name, vec![atom("a")]));
            goals.push(goal(&name, atom("a")));
            goals.push(goal(&name, atom("missing")));
        }
        for i in (0..width).rev() {
            let name = format!("p{i:03}");
            heads.push(comp(&name, vec![Term::Var(i)]));
        }
        heads.insert(width / 2, comp("p000", vec![atom("a"), Term::Nil]));
        heads.push(comp("p000", vec![atom("a"), atom("b")]));
        goals.extend([
            (b"p000".to_vec(), vec![]),
            (b"p000".to_vec(), vec![atom("a"), Term::Nil]),
            (b"p000".to_vec(), vec![atom("a"); 3]),
            goal("absent", Term::Var(0)),
        ]);
        count += check_db(&heads, &goals);
    }
    println!("cix2-checks predicates {count}");
}

#[test]
fn cix2_t5_many_classes_every_front_and_resume() {
    let mut count = 0;
    for width in [1, 8, 32] {
        let mut heads = vec![];
        let mut goals = vec![];
        for i in 0..width {
            let name = format!("c{i:03}");
            for first in [
                atom(&name),
                comp(&name, vec![atom("x")]),
                comp(&name, vec![atom("x"), atom("y")]),
            ] {
                heads.push(comp("p", vec![first.clone()]));
                goals.push(goal("p", first));
            }
            if i % 8 == 0 {
                heads.push(comp("p", vec![Term::Var(i)]));
                heads.push(comp("p", vec![Term::Int(i as i64)]));
                heads.push(comp("p", vec![Term::Nil]));
            }
        }
        heads.extend([
            comp("p", vec![atom("c000")]),
            comp("p", vec![comp("c000", vec![atom("different-child")])]),
            comp("q", vec![atom("c000")]),
        ]);
        goals.extend([
            goal("p", Term::Var(100)),
            goal("p", Term::Int(i64::MIN)),
            goal("p", Term::Nil),
            goal("p", atom("missing")),
            goal("p", comp("c000", vec![Term::Nil; 3])),
            goal("p", comp("missing", vec![Term::Nil])),
        ]);
        count += check_db(&heads, &goals);
    }
    println!("cix2-checks classes {count}");
}

// D1 polynomial, evaluated from the low-order end rather than by its fold.
fn polynomial(name: &[u8], seed: u64) -> u64 {
    let mut sum = 0_u64;
    let mut power = 1_u64;
    for &byte in name.iter().rev() {
        sum = sum.wrapping_add(u64::from(byte).wrapping_mul(power));
        power = power.wrapping_mul(31);
    }
    sum.wrapping_add(seed.wrapping_mul(power))
}

fn collision_sets() -> Vec<Vec<Vec<u8>>> {
    vec![
        vec![b"Aa".to_vec(), b"BB".to_vec(), vec![64, 128]],
        ["AaAa", "AaBB", "BBAa", "BBBB"]
            .map(|s| s.as_bytes().to_vec())
            .to_vec(),
    ]
}

#[test]
fn cix2_t6_hash_polynomial_boundaries_and_collisions() {
    let mut names: Vec<_> = (0..=u8::MAX).map(|byte| vec![byte]).collect();
    for len in 0..=128 {
        names.push((0..len).map(|i| ((i * 73 + len) % 256) as u8).collect());
        names.push(vec![u8::MAX; len]);
    }
    for name in names {
        assert_eq!(
            hash_class(&EClass::Atom(name.clone())),
            polynomial(&name, 1)
        );
        for arity in [0, 1, 2, 31, usize::MAX - 1, usize::MAX] {
            assert_eq!(
                hash_pred(&name, arity),
                polynomial(&name, (arity as u64).wrapping_add(1)),
                "predicate {name:?}/{arity}"
            );
            assert_eq!(
                hash_class(&EClass::Comp(name.clone(), arity)),
                polynomial(&name, (arity as u64).wrapping_add(2)),
                "compound {name:?}/{arity}"
            );
        }
    }
    for names in collision_sets() {
        assert_eq!(names.iter().collect::<BTreeSet<_>>().len(), names.len());
        for name in &names[1..] {
            for arity in [0, 1, 2, usize::MAX] {
                assert_eq!(hash_pred(&names[0], arity), hash_pred(name, arity));
                assert_eq!(
                    hash_class(&EClass::Comp(names[0].clone(), arity)),
                    hash_class(&EClass::Comp(name.clone(), arity))
                );
            }
            assert_eq!(
                hash_class(&EClass::Atom(names[0].clone())),
                hash_class(&EClass::Atom(name.clone()))
            );
        }
    }
    assert_eq!(hash_pred(&vec![127], 1), hash_pred(&vec![65], 3));
    assert_eq!(
        hash_class(&EClass::Atom(vec![127])),
        hash_class(&EClass::Comp(vec![65], 1))
    );
}

#[test]
fn cix2_t7_collision_updates_preserve_both_bucket_chains() {
    let mut count = 0;
    for names in collision_sets() {
        let present = &names[..names.len() - 1];
        for shift in 0..present.len() {
            let a = &present[shift];
            let b = &present[(shift + 1) % present.len()];
            for wild in [false, true] {
                let order: Vec<_> = [a, b, a].into_iter().chain(present.iter()).collect();
                let heads: Vec<_> = order
                    .iter()
                    .map(|name| comp(name, vec![atom("x")]))
                    .collect();
                let goals: Vec<_> = names.iter().map(|name| goal(name, atom("x"))).collect();
                count += check_db(&heads, &goals);
                for compounds in [false, true] {
                    let first = |name: &Vec<u8>| {
                        if compounds {
                            comp(name, vec![atom("child")])
                        } else {
                            atom(name)
                        }
                    };
                    let mut heads: Vec<_> = order
                        .iter()
                        .map(|name| comp("p", vec![first(name)]))
                        .collect();
                    if wild {
                        heads.insert(1, comp("p", vec![Term::Var(0)]));
                        heads.push(comp("p", vec![Term::Nil]));
                    }
                    let goals: Vec<_> = names.iter().map(|name| goal("p", first(name))).collect();
                    count += check_db(&heads, &goals);
                }
            }
        }
    }
    let heads = vec![
        comp([127], vec![atom("x")]),
        comp([65], vec![atom("x"), Term::Nil, Term::Nil]),
        comp([127], vec![atom("x")]),
        comp("p", vec![atom([127])]),
        comp("p", vec![comp([65], vec![Term::Nil])]),
        comp("p", vec![atom([127])]),
    ];
    let goals = vec![
        goal([127], atom("x")),
        (vec![65], vec![atom("x"), Term::Nil, Term::Nil]),
        goal("p", atom([127])),
        goal("p", comp([65], vec![Term::Nil])),
        goal("p", comp([65], vec![Term::Nil; 2])),
    ];
    count += check_db(&heads, &goals);
    println!("cix2-checks collisions {count}");
}

fn stat_ticks(stat: &str) -> u64 {
    let (_, fields) = stat
        .rsplit_once(") ")
        .expect("proc stat command terminator");
    let mut fields = fields.split_whitespace().skip(11);
    let user: u64 = fields.next().unwrap().parse().unwrap();
    let system: u64 = fields.next().unwrap().parse().unwrap();
    user.checked_add(system).unwrap()
}

#[test]
fn cix2_t8_empty_classless_and_byte_boundaries() {
    let mut count = 0;
    let names = [
        vec![],
        vec![0],
        b"a\0b".to_vec(),
        "λ".as_bytes().to_vec(),
        vec![255],
    ];
    for name in names {
        let terms = [
            Term::Var(0),
            Term::Var(usize::MAX),
            Term::Int(i64::MIN),
            Term::Int(i64::MAX),
            Term::Nil,
            atom(&name),
            comp(&name, vec![Term::Nil]),
        ];
        let mut heads = terms.to_vec();
        heads.extend(terms.iter().cloned().map(|t| comp(&name, vec![t])));
        let mut goals: Vec<_> = terms.iter().cloned().map(|t| goal(&name, t)).collect();
        goals.push((name.clone(), vec![]));
        goals.push((name, vec![Term::Nil; 2]));
        count += check_db(&[], &goals);
        count += check_db(&heads, &goals);
    }
    assert_eq!(
        stat_ticks("9 (a ) spaced) R 1 2 3 4 5 6 7 8 9 10 23 17 100 200"),
        40
    );
    assert_eq!(stat_ticks("9 (x) R 1 2 3 4 5 6 7 8 9 10 0 0 100 200"), 0);
    println!("cix2-checks boundaries {count}");
}

const CLOCK_TICKS: u64 = 100;
const LOOKUPS: usize = 10_000;

fn cpu_ticks() -> u64 {
    stat_ticks(&std::fs::read_to_string("/proc/self/stat").unwrap())
}

struct ProbeGoal {
    name: Vec<u8>,
    args: Vec<usize>,
    expected: usize,
}

fn probe(case: &str, sizes: &[usize]) {
    if std::env::var("CIX2_CASE").is_ok_and(|selected| selected != case) {
        return;
    }
    let size = std::env::var("CIX2_SIZE")
        .map(|size| size.parse().expect("CIX2_SIZE integer"))
        .unwrap_or(sizes[0]);
    assert!(sizes.contains(&size), "CIX2_SIZE outside declared sizes");
    let (predicates, classes) = match case {
        "classes" => (1, size),
        "predicates" => (size, 10),
        _ => panic!("undeclared probe case"),
    };
    let mut arena = ETermArena { nodes: vec![] };
    let mut db = vec![];
    let mut goals = vec![];
    let mut predicate_hashes = BTreeSet::new();
    for p in 0..predicates {
        let name = format!("p{p:04}").into_bytes();
        assert!(predicate_hashes.insert(hash_pred(&name, 1)));
        let mut class_hashes = BTreeSet::new();
        for c in 0..classes {
            let first = format!("a{c:06}").into_bytes();
            assert!(class_hashes.insert(hash_class(&EClass::Atom(first.clone()))));
            let args = vec![push_atom(&mut arena, first)];
            let head = push_comp(&mut arena, name.clone(), args.clone());
            goals.push(ProbeGoal {
                name: name.clone(),
                args,
                expected: db.len(),
            });
            db.push(EClause { head, body: vec![] });
        }
    }
    let batch: Vec<_> = (0..LOOKUPS)
        .map(|i| &goals[i * goals.len() / LOOKUPS])
        .collect();
    let mut answers = vec![None; LOOKUPS];
    let load = std::fs::read_to_string("/proc/loadavg").unwrap();
    let load = load.split_whitespace().next().unwrap();
    // A host without the `heavy` slot wrapper: the probe is the only holder.
    let status = match std::process::Command::new("heavy").arg("--status").output() {
        Ok(out) => {
            assert!(out.status.success(), "heavy status failed");
            String::from_utf8(out.stdout).unwrap()
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => "slot 1: probe (no heavy)\n".into(),
        Err(e) => panic!("heavy status: {e}"),
    };
    let holders = status
        .lines()
        .filter(|line| line.starts_with("slot "))
        .count();
    println!("\ncix2-clock-ticks {CLOCK_TICKS}");
    println!("cix2-start {case} {size} load {load} holders {holders}");
    for holder in status.lines() {
        println!("cix2-holder {holder}");
    }
    let start = cpu_ticks();
    let ix = black_box(build_index(black_box(&arena), black_box(&db)));
    let build = cpu_ticks().checked_sub(start).unwrap();
    let start = cpu_ticks();
    for (answer, goal) in answers.iter_mut().zip(&batch) {
        *answer = black_box(next_candidate(
            black_box(&arena),
            black_box(&db),
            black_box(&ix),
            0,
            black_box(&goal.name),
            black_box(&goal.args),
            0,
        ));
    }
    let lookup = cpu_ticks().checked_sub(start).unwrap();
    for (answer, goal) in answers.iter().zip(batch) {
        assert_eq!(*answer, Some(goal.expected));
    }
    println!(
        "cix2-probe {case} {size} build {:.2} lookup {:.2}",
        build as f64 / CLOCK_TICKS as f64,
        lookup as f64 / CLOCK_TICKS as f64
    );
}

#[test]
#[ignore = "CPU probe: run .agent/archive/bench/cix2-scaling"]
fn cix2_t1_probe_classes() {
    probe("classes", &[1_000, 10_000, 100_000]);
}

#[test]
#[ignore = "CPU probe: run .agent/archive/bench/cix2-scaling"]
fn cix2_t2_probe_predicates() {
    probe("predicates", &[100, 1_000]);
}
