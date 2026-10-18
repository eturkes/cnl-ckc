use super::common::*;

fn literal(name: &str, arity: usize) -> String {
    if arity == 0 {
        name.into()
    } else {
        format!("{name}({})", vec!["x"; arity].join(","))
    }
}
fn position(goal: &str, at: usize) -> String {
    let head = "guideline_event(actual,e,wait)";
    match at {
        0 => goal.into(),
        1 => format!("{head} :- {goal}"),
        2 => format!("{head} :- \\+ {goal}"),
        3 => format!("{head} :- \\+ ({goal}, {head})"),
        _ => unreachable!(),
    }
}
fn terms() -> Vec<String> {
    let leaves = [
        "x",
        "0",
        "-1",
        "'two words'",
        "'α'",
        "[]",
        "A",
        "'it\\'s'",
        "'a\\\\b'",
        "99999999999999999999999999999999999999999999999999",
    ];
    (0..30)
        .map(|i| {
            let mut t = leaves[i % leaves.len()].to_owned();
            for level in 0..i / leaves.len() {
                t = match level % 3 {
                    0 => format!("f({t},[])"),
                    1 => format!("[{t},x]"),
                    _ => format!("[x|tail({t})]"),
                };
            }
            t
        })
        .collect()
}
#[test]
fn p1_nineteen_declarations_and_exact_block_boundaries() {
    let mut c = Cases::new("P1-declarations");
    let text = document("doc", &["guideline_event(actual,e,wait)".into()]);
    c.check("19 declarations", &text);
    let pairs: Vec<_> = INDICATORS.iter().map(|&(n, a)| pair(n, a)).collect();
    for count in 0..19 {
        c.mutation(
            &format!("{count} declarations"),
            &text,
            &declarations(),
            &pairs[..count].concat(),
        );
    }
    for (i, pair) in pairs.iter().enumerate() {
        c.mutation(&format!("missing pair {i}"), &text, pair, "");
        c.mutation(&format!("duplicate pair {i}"), &text, pair, &pair.repeat(2));
    }
    for i in 0..18 {
        c.mutation(
            &format!("swapped pairs {i}"),
            &text,
            &(pairs[i].clone() + &pairs[i + 1]),
            &(pairs[i + 1].clone() + &pairs[i]),
        );
    }
    for directive in ["multifile", "discontiguous"] {
        let from = format!(":- {directive}(guideline_unit/3).\n");
        c.mutation(&format!("missing {directive}"), &text, &from, "");
        for arity in [0, 1, 2, 4, 6, 30] {
            c.mutation(
                &format!("{directive} arity {arity}"),
                &text,
                &from,
                &format!(":- {directive}(guideline_unit/{arity}).\n"),
            );
        }
    }
    for arity in [4, 6] {
        c.mutation(
            &format!("builtin declaration {arity}"),
            &text,
            &declarations(),
            &(declarations() + &pair("guideline_satisfies", arity)),
        );
    }
    c.finish();
}
#[test]
fn p1_unit_arbitrary_terms_in_heads_bodies_naf_and_raw_queries() {
    let mut c = Cases::new("P1-unit-terms");
    for term in terms() {
        for slot in 0..3 {
            let mut args = ["actual", "q", "day"];
            args[slot] = &term;
            let lit = format!("guideline_unit({})", args.join(","));
            for at in 0..4 {
                c.check(
                    &format!("slot={slot} position={at} term={term}"),
                    &document("doc", &[position(&lit, at)]),
                );
            }
            c.check(
                &format!("raw query slot={slot} term={term}"),
                &query(&lit, "[]"),
            );
        }
    }
    assert_eq!(c.total, 900);
    c.finish();
}
#[test]
fn p1_builtins_are_body_literals_not_heads() {
    let mut c = Cases::new("P1-builtin-positions");
    for arity in [4, 6] {
        for term in terms() {
            for slot in 0..arity {
                let mut args = vec!["x"; arity];
                args[slot] = &term;
                let lit = format!("guideline_satisfies({})", args.join(","));
                for at in 1..4 {
                    c.check(
                        &format!("/{arity} slot={slot} position={at} {term}"),
                        &document("doc", &[position(&lit, at)]),
                    );
                }
            }
        }
        let lit = literal("guideline_satisfies", arity);
        let text = document("doc", std::slice::from_ref(&lit));
        let at = text.rfind(&lit).unwrap() + "guideline_".len();
        c.reject_at(&format!("builtin /{arity} head"), &text, at);
        for wrong in [0, 1, 2, 3, 5, 7] {
            for at in 1..4 {
                let text = document("doc", &[position(&lit, at)]);
                let bad = literal("guideline_satisfies", wrong);
                let offset = text.rfind(&lit).unwrap();
                // Q18 D3: an allowed bare name is arity zero, at its start.
                let frontier = if wrong == 0 {
                    0
                } else {
                    [4, 6]
                        .iter()
                        .map(|&n| lcp(bad.as_bytes(), literal("guideline_satisfies", n).as_bytes()))
                        .max()
                        .unwrap()
                };
                c.reject_at(
                    &format!("/{arity} wrong arity {wrong} position={at}"),
                    &text.replacen(&lit, &bad, 1),
                    offset + frontier,
                );
            }
        }
    }
    c.finish();
}
fn lcp(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).take_while(|(a, b)| a == b).count()
}
#[test]
fn p1_fixed_name_census_uses_the_grammar_frontier() {
    let mut c = Cases::new("P1-q18");
    for (name, arity) in [
        ("guideline_unit", 3),
        ("guideline_satisfies", 4),
        ("guideline_satisfies", 6),
    ] {
        let start = if name == "guideline_unit" { 0 } else { 1 };
        for at in start..4 {
            let lit = literal(name, arity);
            let text = document("doc", &[position(&lit, at)]);
            c.check(&format!("/{arity} position={at} control"), &text);
            let offset = text.rfind(&lit).unwrap();
            let candidates: Vec<_> = INDICATORS[2..]
                .iter()
                .map(|(n, _)| format!("{n}("))
                .chain((at != 0).then_some("guideline_satisfies(".to_owned()))
                .collect();
            let mut mutations: Vec<String> = (1..name.len()).map(|n| name[..n].into()).collect();
            mutations.extend([
                format!("'{name}'"),
                format!("{name}x"),
                format!("{name}_"),
                "Guideline_unit".into(),
            ]);
            for changed_name in mutations {
                let changed_lit = literal(&changed_name, arity);
                let changed = text.replacen(&lit, &changed_lit, 1);
                let frontier = candidates
                    .iter()
                    .map(|s| lcp(changed_lit.as_bytes(), s.as_bytes()))
                    .max()
                    .unwrap();
                c.reject_at(
                    &format!("/{arity} position={at} name={changed_name}"),
                    &changed,
                    offset + frontier,
                );
            }
        }
    }
    for directive in ["multifile", "discontiguous"] {
        let text = document("doc", &["guideline_unit(actual,q,day)".into()]);
        let from = format!(":- {directive}(guideline_unit/3).");
        for bad in ["guideline_uni", "guideline_unitx", "'guideline_unit'"] {
            c.mutation(
                &format!("declaration {directive} {bad}"),
                &text,
                &from,
                &format!(":- {directive}({bad}/3)."),
            );
        }
    }
    c.finish();
}
#[test]
fn p1_query_names_belong_to_the_answer_whitelist() {
    let mut c = Cases::new("P1-whitelist");
    c.write("m.tsv", "");
    for goal in [
        "guideline_unit(actual,q,day)",
        "guideline_satisfies(exactly,60,geq,50)",
        "guideline_satisfies(exactly,1,week,greater,3,day)",
    ] {
        c.check(goal, &query(goal, "[]"));
        c.answer(goal, goal, !goal.starts_with("guideline_unit("));
    }
    for (name, arities) in [
        ("guideline_unit", vec![0, 1, 2, 4, 5]),
        ("guideline_satisfies", vec![0, 1, 2, 3, 5, 7, 8]),
        ("guideline_unitx", vec![3]),
        ("guideline_satisfiesx", vec![4, 6]),
    ] {
        for arity in arities {
            let q = query(&literal(name, arity), "[]");
            c.check(&format!("raw {name}/{arity}"), &q);
            c.write("query.pl", &q);
            c.expect(
                &format!("whitelist {name}/{arity}"),
                &["v1", "answer", "m.tsv", "query.pl"],
                Out::err(
                    2,
                    format!(
                        "ace_to_pl_error(check_load,query_file(goal_foreign({name},{arity}))).\n"
                    ),
                ),
            );
        }
    }
    c.finish();
}

#[test]
fn p1_unit_arity_boundaries() {
    let mut c = Cases::new("P1-unit-arity");
    let lit = literal("guideline_unit", 3);
    for at in 0..4 {
        let text = document("doc", &[position(&lit, at)]);
        c.check(&format!("position={at}/control"), &text);
        for arity in [0, 1, 2, 4, 5, 6] {
            if arity == 0 {
                assert_eq!(
                    text.matches(&lit).count(),
                    1,
                    "position={at}/arity={arity}: unique anchor"
                );
                c.reject_at(
                    &format!("position={at}/arity={arity}"),
                    &text.replacen(&lit, "guideline_unit", 1),
                    text.rfind(&lit).unwrap(),
                );
            } else {
                c.mutation(
                    &format!("position={at}/arity={arity}"),
                    &text,
                    &lit,
                    &literal("guideline_unit", arity),
                );
            }
        }
    }
    c.finish();
}
