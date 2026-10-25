use super::support::*;

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
#[test]
fn p3_declaration_frontier() {
    let mut c = Cases::new("P3-declarations");
    let text = document("doc", &["guideline_event(actual,e,wait)".into()]);
    c.check("22 indicators", &text);
    let pairs: Vec<_> = INDICATORS.iter().map(|&(n, a)| pair(n, a)).collect();
    for count in 0..22 {
        c.mutation(
            &format!("{count} indicators"),
            &text,
            &declarations(),
            &pairs[..count].concat(),
        );
    }
    for (i, p) in pairs.iter().enumerate() {
        c.mutation(&format!("missing pair {i}"), &text, p, "");
        c.mutation(&format!("duplicate pair {i}"), &text, p, &p.repeat(2));
    }
    for i in 0..21 {
        c.mutation(
            &format!("swapped pair {i}"),
            &text,
            &(pairs[i].clone() + &pairs[i + 1]),
            &(pairs[i + 1].clone() + &pairs[i]),
        );
    }
    for (name, arity) in &INDICATORS[20..] {
        for directive in ["multifile", "discontiguous"] {
            let from = format!(":- {directive}({name}/{arity}).\n");
            c.mutation(&format!("missing {directive}/{name}"), &text, &from, "");
            for wrong in [0, 2, 3, 5, 10] {
                c.mutation(
                    &format!("{directive}/{name}/{wrong}"),
                    &text,
                    &from,
                    &format!(":- {directive}({name}/{wrong}).\n"),
                );
            }
        }
    }
    c.mutation(
        "R9 schema +4 lines",
        &text,
        "guideline_schema_version(1).",
        "guideline_schema_version(2).",
    );
    c.mutation(
        "R9 wrapper +4 lines",
        &text,
        "guideline_document(doc,ace_sha256",
        "guideline_document(doc,ace_sha25z",
    );
    c.finish();
}
fn terms() -> Vec<String> {
    let leaves = [
        "value",
        "'doc-id'",
        "0",
        "-1",
        "'α'",
        "'two words'",
        "[]",
        "A",
        "'it\\'s'",
        "'a\\\\b'",
        "999999999999999999999999999999999999999999",
        "rational(1,2)",
    ];
    (0..36)
        .map(|i| {
            let mut value = leaves[i % leaves.len()].to_owned();
            for level in 0..i / leaves.len() {
                value = if level == 0 {
                    format!("source({value},x)")
                } else {
                    format!("[x|tail({value})]")
                };
            }
            value
        })
        .collect()
}
#[test]
fn p3_term_domain() {
    let mut c = Cases::new("P3-term-domain");
    for term in terms() {
        for (name, arity) in &INDICATORS[20..] {
            for slot in 0..*arity {
                let mut args = vec!["x".to_owned(); *arity];
                args[slot] = term.clone();
                let lit = format!("{name}({})", args.join(","));
                for at in 0..4 {
                    c.check(
                        &format!("{name}/slot={slot}/position={at}/term={term}"),
                        &document("doc", &[position(&lit, at)]),
                    );
                }
                c.check(
                    &format!("{name}/slot={slot}/raw-query/{term}"),
                    &query(&lit, "[]"),
                );
            }
        }
    }
    assert_eq!(c.total, 1800);
    c.finish();
}
fn lcp(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).take_while(|(a, b)| a == b).count()
}
#[test]
fn p3_q18_fixed_names() {
    let mut c = Cases::new("P3-q18");
    for (name, arity) in &INDICATORS[20..] {
        let lit = literal(name, *arity);
        for at in 0..4 {
            let text = document("doc", &[position(&lit, at)]);
            c.check(&format!("{name}/position={at}/control"), &text);
            let start = text.rfind(&lit).unwrap();
            let candidates: Vec<_> = INDICATORS[2..]
                .iter()
                .map(|(n, _)| format!("{n}("))
                .chain((at != 0).then_some("guideline_satisfies(".into()))
                .chain((at != 0).then_some("guideline_product(".into()))
                .chain((at != 0).then_some("guideline_quotient(".into()))
                .collect();
            let candidates: Vec<_> = candidates
                .iter()
                .cloned()
                .chain(
                    candidates
                        .iter()
                        .filter(|_| at == 2)
                        .map(|candidate| format!("({candidate}")),
                )
                .collect();
            let mut mutations: Vec<String> = (0..name.len()).map(|n| name[..n].into()).collect();
            mutations.extend([
                format!("'{name}'"),
                format!("{name}x"),
                format!("{name}_"),
                format!("G{}", &name[1..]),
            ]);
            for i in 0..name.len() {
                let mut typo = name.as_bytes().to_vec();
                typo[i] = b'z';
                mutations.push(String::from_utf8(typo).unwrap());
            }
            for changed_name in mutations {
                let changed = literal(&changed_name, *arity);
                let frontier = candidates
                    .iter()
                    .map(|candidate| lcp(changed.as_bytes(), candidate.as_bytes()))
                    .max()
                    .unwrap();
                c.reject_at(
                    &format!("{name}/position={at}/name={changed_name}"),
                    &text.replacen(&lit, &changed, 1),
                    start + frontier,
                );
            }
            for wrong in [0, 1, 2, 3, 4, 5, 6]
                .into_iter()
                .filter(|wrong| wrong != arity)
            {
                let changed = literal(name, wrong);
                if wrong == 0 {
                    c.reject_at(
                        &format!("{name}/position={at}/bare"),
                        &text.replacen(&lit, &changed, 1),
                        start,
                    );
                } else {
                    c.mutation(
                        &format!("{name}/position={at}/arity={wrong}"),
                        &text,
                        &lit,
                        &changed,
                    );
                }
            }
        }
        for directive in ["multifile", "discontiguous"] {
            let text = document("doc", std::slice::from_ref(&lit));
            let anchor = format!(":- {directive}({name}/{arity}).");
            for bad in [
                name[..name.len() - 1].to_owned(),
                format!("{name}x"),
                format!("'{name}'"),
            ] {
                c.mutation(
                    &format!("declaration/{directive}/{bad}"),
                    &text,
                    &anchor,
                    &format!(":- {directive}({bad}/{arity})."),
                );
            }
        }
    }
    c.finish();
}
#[test]
fn p3_query_whitelist() {
    let mut c = Cases::new("P3-query-whitelist");
    c.write("m.tsv", "");
    for (name, arity) in &INDICATORS[20..] {
        let positive = literal(name, *arity);
        boolean(&mut c, &format!("{name}/positive"), &positive, None);
        // Main ruling (contract P3): NAF payloads are a document position; a query
        // goal is positive, and a query file holding `\+` rejects for every
        // predicate (the whitelist law), the new ones included.
        let negative = format!("\\+({positive})");
        c.write("query.pl", &query(&negative, "[]"));
        c.expect(
            &format!("{name}/NAF"),
            &["v1", "answer", "m.tsv", "query.pl"],
            Out::err(
                2,
                "ace_to_pl_error(check_load,query_file(goal_foreign(\\+,1))).\n".to_owned(),
            ),
        );
        let q = query(
            &format!("{name}({})", vec!["A"; *arity].join(",")),
            "[answer(A,wh(what))]",
        );
        chain(
            &mut c,
            &format!("{name}/solutions"),
            &q,
            "solutions([])",
            "solutions([])",
        );
        for wrong in [0, 1, 2, 3, 4, 5, 6]
            .into_iter()
            .filter(|wrong| wrong != arity)
        {
            let q = query(&literal(name, wrong), "[]");
            c.check(&format!("{name}/raw-arity={wrong}"), &q);
            c.write("query.pl", &q);
            c.expect(
                &format!("{name}/whitelist-arity={wrong}"),
                &["v1", "answer", "m.tsv", "query.pl"],
                Out::err(
                    2,
                    format!(
                        "ace_to_pl_error(check_load,query_file(goal_foreign({name},{wrong}))).\n"
                    ),
                ),
            );
        }
        for bad in [name[..name.len() - 1].to_owned(), format!("{name}x")] {
            c.write("query.pl", &query(&literal(&bad, *arity), "[]"));
            c.expect(
                &bad,
                &["v1", "answer", "m.tsv", "query.pl"],
                Out::err(
                    2,
                    format!(
                        "ace_to_pl_error(check_load,query_file(goal_foreign({bad},{arity}))).\n"
                    ),
                ),
            );
        }
    }
    c.finish();
}
