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
        "scope",
        "'guideline-2'",
        "0",
        "-1",
        "'α'",
        "'two words'",
        "[]",
        "A",
        "'it\\'s'",
        "'a\\\\b'",
        "99999999999999999999999999999999999999999999999999",
        "'[]'",
    ];
    (0..48)
        .map(|i| {
            let mut term = leaves[i % leaves.len()].to_owned();
            for level in 0..i / leaves.len() {
                term = match level % 3 {
                    0 => format!("f({term},[])"),
                    1 => format!("[{term},x]"),
                    _ => format!("[x|tail({term})]"),
                };
            }
            term
        })
        .collect()
}
fn lcp(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).take_while(|(a, b)| a == b).count()
}

#[test]
fn b1_twenty_indicators_replace_nineteen_at_the_exact_frontier() {
    let mut c = Cases::new("B1-block");
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
            &format!("swapped pairs {i}"),
            &text,
            &(pairs[i].clone() + &pairs[i + 1]),
            &(pairs[i + 1].clone() + &pairs[i]),
        );
    }
    for directive in ["multifile", "discontiguous"] {
        let from = format!(":- {directive}(guideline_exclusion/1).\n");
        c.mutation(&format!("missing {directive}"), &text, &from, "");
        for arity in [0, 2, 3, 10] {
            c.mutation(
                &format!("{directive} /{arity}"),
                &text,
                &from,
                &format!(":- {directive}(guideline_exclusion/{arity}).\n"),
            );
        }
    }
    c.mutation(
        "schema offset after inserted two lines",
        &text,
        "guideline_schema_version(1).",
        "guideline_schema_version(2).",
    );
    c.mutation(
        "record wrapper offset after inserted two lines",
        &text,
        "guideline_document(doc,ace_sha256",
        "guideline_document(doc,ace_sha25x",
    );
    c.finish();
}

#[test]
fn b1_exclusion_term_domain_in_heads_bodies_naf_and_raw_queries() {
    let mut c = Cases::new("B1-terms");
    // Structural reader law is broader than D2's ground emitted key law.
    for term in terms() {
        let lit = format!("guideline_exclusion({term})");
        for at in 0..4 {
            c.check(
                &format!("position={at} term={term}"),
                &document("doc", &[position(&lit, at)]),
            );
        }
        c.check(&format!("raw query term={term}"), &query(&lit, "[]"));
    }
    assert_eq!(c.total, 480);
    c.finish();
}

#[test]
fn b1_q18_exclusion_names_and_arities_use_the_grammar_frontier() {
    let mut c = Cases::new("B1-q18");
    let name = "guideline_exclusion";
    let lit = literal(name, 1);
    for at in 0..4 {
        let text = document("doc", &[position(&lit, at)]);
        c.check(&format!("position={at}/control"), &text);
        let offset = text.rfind(&lit).unwrap();
        let candidates: Vec<_> = INDICATORS[2..]
            .iter()
            .map(|(n, _)| format!("{n}("))
            .chain((at != 0).then_some("guideline_satisfies(".to_owned()))
            .collect();
        let candidates: Vec<_> = candidates
            .iter()
            .cloned()
            .chain(
                candidates
                    .iter()
                    .filter(|_| at == 2)
                    .map(|name| format!("({name}")),
            )
            .collect();
        let mut mutations: Vec<String> = (0..name.len()).map(|n| name[..n].into()).collect();
        mutations.extend([
            format!("'{name}'"),
            format!("{name}x"),
            format!("{name}_"),
            "Guideline_exclusion".into(),
        ]);
        for bad_name in mutations {
            let bad_lit = literal(&bad_name, 1);
            let frontier = candidates
                .iter()
                .map(|s| lcp(bad_lit.as_bytes(), s.as_bytes()))
                .max()
                .unwrap();
            c.reject_at(
                &format!("position={at}/name={bad_name}"),
                &text.replacen(&lit, &bad_lit, 1),
                offset + frontier,
            );
        }
        for arity in [0, 2, 3, 4, 5, 6] {
            let bad = literal(name, arity);
            if arity == 0 {
                c.reject_at(
                    &format!("position={at}/arity=0"),
                    &text.replacen(&lit, &bad, 1),
                    offset,
                );
            } else {
                c.mutation(&format!("position={at}/arity={arity}"), &text, &lit, &bad);
            }
        }
    }
    for directive in ["multifile", "discontiguous"] {
        let text = document("doc", std::slice::from_ref(&lit));
        let from = format!(":- {directive}(guideline_exclusion/1).");
        for bad in [
            "guideline_exclusio",
            "guideline_exclusionx",
            "'guideline_exclusion'",
        ] {
            c.mutation(
                &format!("declaration {directive}/{bad}"),
                &text,
                &from,
                &format!(":- {directive}({bad}/1)."),
            );
        }
    }
    c.finish();
}

#[test]
fn b1_raw_exclusion_questions_reach_the_answer_whitelist() {
    let mut c = Cases::new("B1-whitelist");
    c.write("m.tsv", "");
    for term in ["alpha", "'key-9'", "'α'", "[]", "A", "f(A)"] {
        let goal = format!("guideline_exclusion({term})");
        c.check(term, &query(&goal, "[]"));
        boolean(&mut c, term, &goal, None);
    }
    for arity in [0, 2, 3, 4, 5] {
        let q = query(&literal("guideline_exclusion", arity), "[]");
        c.check(&format!("raw /{arity}"), &q);
        c.write("query.pl", &q);
        c.expect(&format!("whitelist /{arity}"), &["v1", "answer", "m.tsv", "query.pl"], Out::err(2, format!("ace_to_pl_error(check_load,query_file(goal_foreign(guideline_exclusion,{arity}))).\n")));
    }
    for name in ["guideline_exclusio", "guideline_exclusionx"] {
        let q = query(&literal(name, 1), "[]");
        c.check(name, &q);
        c.write("query.pl", &q);
        c.expect(
            name,
            &["v1", "answer", "m.tsv", "query.pl"],
            Out::err(
                2,
                format!("ace_to_pl_error(check_load,query_file(goal_foreign({name},1))).\n"),
            ),
        );
    }
    c.finish();
}
