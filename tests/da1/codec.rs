use super::common::*;

fn literal(name: &str, arity: usize) -> String {
    if arity == 0 {
        name.into()
    } else {
        format!("{name}({})", vec!["x"; arity].join(","))
    }
}
fn position(goal: &str, at: usize) -> String {
    match at {
        0 => goal.into(),
        1 => format!("guideline_event(actual,e,wait) :- {goal}"),
        2 => format!("guideline_event(actual,e,wait) :- \\+ {goal}"),
        3 => format!(
            "guideline_event(actual,e,wait) :- \\+ ({goal}, guideline_event(actual,e,wait))"
        ),
        _ => unreachable!(),
    }
}
fn lcp(a: &str, b: &str) -> usize {
    a.bytes().zip(b.bytes()).take_while(|(a, b)| a == b).count()
}
#[test]
fn p7_positive_and_naf_positions_admit_arithmetic_with_generic_terms() {
    let mut c = Cases::new("P7-positions");
    for name in ["guideline_product", "guideline_quotient"] {
        for term in [
            "0",
            "-1",
            "rational(1,2)",
            "rational(2,4)",
            "rational(A,2)",
            "rational(1,A)",
            "A",
            "[]",
            "[x]",
            "[x|tail(A)]",
            "f(A,A)",
            "'two words'",
        ] {
            for slot in 0..6 {
                let mut args = ["x"; 6];
                args[slot] = term;
                let goal = format!("{name}({})", args.join(","));
                for at in 1..4 {
                    c.check(
                        &format!("{name} slot={slot} position={at} term={term}"),
                        &document("codec", &[position(&goal, at)]),
                    );
                }
            }
        }
    }
    c.finish();
}
#[test]
fn p7_arithmetic_heads_reject_at_the_existing_r9_frontier() {
    let mut c = Cases::new("P7-heads");
    for name in ["guideline_product", "guideline_quotient"] {
        let goal = literal(name, 6);
        let text = document("codec", std::slice::from_ref(&goal));
        let at = text.rfind(&goal).unwrap()
            + INDICATORS[2..]
                .iter()
                .map(|(n, _)| lcp(&goal, &format!("{n}(")))
                .max()
                .unwrap();
        c.reject_at(name, &text, at);
    }
    c.finish();
}
#[test]
fn p7_twenty_declarations_stay_exact_and_exclude_builtins() {
    let mut c = Cases::new("P7-declarations");
    let text = document(
        "codec",
        &["guideline_entity(actual,p,rational,countable)".into()],
    );
    c.check("22-indicator control; rational lemma remains data", &text);
    assert_eq!(declarations().lines().count(), 44);
    for name in ["guideline_product", "guideline_quotient"] {
        for arity in [4, 6] {
            c.mutation(
                &format!("no {name}/{arity} declaration"),
                &text,
                &declarations(),
                &(declarations() + &pair(name, arity)),
            );
        }
    }
    c.finish();
}
#[test]
fn p7_wrong_arities_and_name_neighbors_reject_in_bodies_and_whitelist() {
    let mut c = Cases::new("P7-arities");
    c.write("m.tsv", "");
    for name in ["guideline_product", "guideline_quotient"] {
        for arity in [0, 1, 2, 3, 4, 5, 7, 8] {
            let goal = literal(name, arity);
            for at in 1..4 {
                let text = document("codec", &[position(&goal, at)]);
                let start = text.rfind(&goal).unwrap();
                c.reject_at(
                    &format!("{name}/{arity} position={at}"),
                    &text,
                    start
                        + if arity == 0 {
                            0
                        } else {
                            lcp(&goal, &literal(name, 6))
                        },
                );
            }
            let q = query(&goal, "[]");
            c.check(&format!("raw query {name}/{arity}"), &q);
            c.write("query.pl", &q);
            c.expect(
                &format!("foreign arity {name}/{arity}"),
                &["v1", "answer", "m.tsv", "query.pl"],
                Out::err(
                    2,
                    format!(
                        "ace_to_pl_error(check_load,query_file(goal_foreign({name},{arity}))).\n"
                    ),
                ),
            );
        }
        let foreign = format!("{name}x");
        let q = query(&literal(&foreign, 6), "[]");
        c.write("query.pl", &q);
        c.expect(
            &foreign,
            &["v1", "answer", "m.tsv", "query.pl"],
            Out::err(
                2,
                format!("ace_to_pl_error(check_load,query_file(goal_foreign({foreign},6))).\n"),
            ),
        );
    }
    c.finish();
}
#[test]
fn p7_query_whitelist_accepts_both_arithmetic_names() {
    let mut c = Cases::new("P7-whitelist");
    c.write("m.tsv", "");
    for goal in [
        "guideline_product(exactly,2,exactly,3,A,B)",
        "guideline_quotient(exactly,3,exactly,2,A,B)",
    ] {
        c.check(goal, &query(goal, "[]"));
        c.answer(goal, goal, true);
    }
    c.finish();
}
#[test]
fn p1_malformed_rational_compounds_remain_readable_data() {
    let mut c = Cases::new("P1-data");
    for term in super::oracle::BAD_NUMBERS {
        let clause = format!("guideline_cardinality(actual,p,na,exactly,{term})");
        c.check(term, &document("data", std::slice::from_ref(&clause)));
        install(&c, &[("data", vec![clause.clone()], String::new())]);
        c.answer(&format!("structural data {term}"), &clause, true);
    }
    c.finish();
}
