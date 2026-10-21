use super::common::*;

fn raw(id: &'static str, clauses: Vec<String>) -> (&'static str, Vec<String>, String) {
    (id, clauses, String::new())
}
fn answer_result(c: &mut Cases, label: &str, goal: &str, template: &str, result: &str) {
    let q = query(goal, template);
    c.write("query.pl", &q);
    c.expect(
        label,
        &["v1", "answer", "m.tsv", "query.pl"],
        Out::ok(answers(&q, result)),
    );
}
#[test]
fn p5_rules_bind_head_counts_chain_and_propagate_bounds() {
    let mut c = Cases::new("P5-rules");
    let facts = vec![
        "guideline_cardinality(source,bounded,na,geq,60)".into(),
        "guideline_cardinality(source,exact,na,exactly,40)".into(),
        "guideline_cardinality(source,fraction,na,exactly,47)".into(),
    ];
    let product = "guideline_cardinality(actual,A,na,B,C) :- guideline_cardinality(source,A,na,D,E), guideline_product(D,E,exactly,rational(3,2),B,C)";
    let quotient = "guideline_cardinality(half,A,na,B,C) :- guideline_cardinality(actual,A,na,D,E), guideline_quotient(D,E,exactly,2,B,C)";
    install(
        &c,
        &[
            raw("patients", facts.clone()),
            raw("conversion", vec![product.into(), quotient.into()]),
        ],
    );
    answer_result(
        &mut c,
        "all canonical products",
        "guideline_cardinality(actual,A,na,B,C)",
        "[answer(A,wh(what)),answer(B,wh(what)),answer(C,wh(what))]",
        "solutions([sol([bounded,geq,90]),sol([exact,exactly,60]),sol([fraction,exactly,rational(141,2)])])",
    );
    answer_result(
        &mut c,
        "quotient consumes product",
        "guideline_cardinality(half,A,na,B,C)",
        "[answer(A,wh(what)),answer(B,wh(what)),answer(C,wh(what))]",
        "solutions([sol([bounded,geq,45]),sol([exact,exactly,30]),sol([fraction,exactly,rational(141,4)])])",
    );
    c.answer(
        "bounded fact is not exact",
        "guideline_cardinality(actual,bounded,na,exactly,90)",
        false,
    );
    c.answer(
        "bounded fact propagates",
        "guideline_cardinality(actual,bounded,na,geq,90)",
        true,
    );
    c.answer(
        "fraction normalizes",
        "guideline_cardinality(half,fraction,na,exactly,rational(141,4))",
        true,
    );
    let children = [
        node("patients", &facts[0], &[]),
        builtin("guideline_product(geq,60,exactly,rational(3,2),geq,90)"),
    ];
    boolean(
        &mut c,
        "rule output reaches head and trace",
        "guideline_cardinality(actual,bounded,na,geq,90)",
        Some(&[node("conversion", product, &children)]),
    );
    c.finish();
}
#[test]
fn p1_p5_canonical_answer_dedup_keeps_structural_not_numeric_order() {
    let mut c = Cases::new("P1-P5-order");
    let facts = [4, -3, 1, 0, 2, 2]
        .iter()
        .map(|n| format!("guideline_cardinality(source,p,na,exactly,{n})"))
        .collect();
    let rule = "guideline_cardinality(actual,p,na,A,B) :- guideline_cardinality(source,p,na,exactly,C), guideline_quotient(exactly,C,exactly,2,A,B)";
    install(
        &c,
        &[raw("patients", facts), raw("conversion", vec![rule.into()])],
    );
    answer_result(
        &mut c,
        "integers precede compounds; repeated one deduplicates",
        "guideline_cardinality(actual,p,na,exactly,A)",
        "[answer(A,wh(what))]",
        "solutions([sol([0]),sol([1]),sol([2]),sol([rational(-3,2)]),sol([rational(1,2)])])",
    );
    c.finish();
}
#[test]
fn p5b_output_aliases_partial_structures_and_chained_templates() {
    let mut c = Cases::new("P5b-unify");
    c.write("m.tsv", "");
    for goal in [
        "guideline_product(exactly,2,exactly,3,A,A)",
        "guideline_quotient(exactly,2,exactly,3,A,A)",
        "guideline_product(exactly,A,exactly,3,exactly,A)",
        "guideline_quotient(exactly,A,exactly,3,exactly,A)",
        "guideline_product(A,2,exactly,3,A,6)",
        "guideline_product(exactly,3,exactly,rational(1,2),A,rational(A,2))",
        "guideline_product(exactly,2,exactly,3,exactly,rational(A,B))",
        "guideline_product(exactly,0,exactly,3,A,f(A))",
    ] {
        c.answer(goal, goal, false);
    }
    for (goal, template, result) in [
        (
            "guideline_product(exactly,3,exactly,rational(1,2),A,rational(B,2))",
            "[answer(A,wh(what)),answer(B,wh(what))]",
            "solutions([sol([exactly,3])])",
        ),
        (
            "guideline_product(exactly,3,exactly,rational(1,2),exactly,rational(A,B))",
            "[answer(A,wh(what)),answer(B,wh(what))]",
            "solutions([sol([3,2])])",
        ),
        (
            "guideline_quotient(exactly,-3,exactly,2,exactly,rational(A,B))",
            "[answer(A,wh(what)),answer(B,wh(what))]",
            "solutions([sol([-3,2])])",
        ),
    ] {
        answer_result(&mut c, goal, goal, template, result);
    }
    let goals = [
        "guideline_product(exactly,7,exactly,rational(3,2),A,B)".into(),
        "guideline_quotient(A,B,exactly,3,C,D)".into(),
    ];
    let q = query(
        &conjunction(&goals),
        "[answer(A,wh(what)),answer(B,wh(what)),answer(C,wh(what)),answer(D,wh(what))]",
    );
    chain(
        &mut c,
        "root chain binds template",
        &q,
        "solutions([sol([exactly,rational(21,2),exactly,rational(7,2)])])",
        "solutions([sol([exactly,rational(21,2),exactly,rational(7,2)],proved([builtin(guideline_product(exactly,7,exactly,rational(3,2),exactly,rational(21,2))),builtin(guideline_quotient(exactly,rational(21,2),exactly,3,exactly,rational(7,2)))]))])",
    );
    install(
        &c,
        &[raw(
            "alias",
            vec!["guideline_cardinality(source,p,na,exactly,2)".into()],
        )],
    );
    let goals = [
        "guideline_cardinality(source,p,na,exactly,A)".into(),
        "guideline_product(exactly,A,exactly,1,exactly,A)".into(),
    ];
    answer_result(
        &mut c,
        "ground input/output alias",
        &conjunction(&goals),
        "[answer(A,wh(what))]",
        "solutions([sol([2])])",
    );
    c.finish();
}
#[test]
fn p5b_failures_backtrack_and_restore_outputs_at_the_preceding_choicepoint() {
    let mut c = Cases::new("P5b-rollback");
    for bad in ["rational(2,4)", "2"] {
        let facts = vec![
            format!("guideline_cardinality(source,p,na,exactly,{bad})"),
            "guideline_cardinality(source,p,na,exactly,3)".into(),
        ];
        let rule = "guideline_cardinality(actual,p,na,A,B) :- guideline_cardinality(source,p,na,exactly,C), guideline_product(exactly,C,exactly,3,A,B), guideline_satisfies(A,B,exactly,9)";
        install(
            &c,
            &[
                raw("patients", facts.clone()),
                raw("policy", vec![rule.into()]),
            ],
        );
        let q = query(
            "guideline_cardinality(actual,p,na,A,B)",
            "[answer(A,wh(what)),answer(B,wh(what))]",
        );
        let children = [
            node("patients", &facts[1], &[]),
            builtin("guideline_product(exactly,3,exactly,3,exactly,9)"),
            builtin("guideline_satisfies(exactly,9,exactly,9)"),
        ];
        chain(
            &mut c,
            &format!("first count={bad}"),
            &q,
            "solutions([sol([exactly,9])])",
            &format!(
                "solutions([sol([exactly,9],proved([{}]))])",
                node("policy", rule, &children)
            ),
        );
    }
    // A result clause's failed output unification must leave the next clause untouched.
    let clauses = vec![
        "guideline_cardinality(actual,p,na,A,B) :- guideline_product(exactly,2,exactly,3,A,B), guideline_satisfies(A,B,exactly,99)".into(),
        "guideline_cardinality(actual,p,na,A,B) :- guideline_quotient(exactly,7,exactly,2,A,B)".into(),
    ];
    install(&c, &[raw("policy", clauses)]);
    answer_result(
        &mut c,
        "clause rollback",
        "guideline_cardinality(actual,p,na,A,B)",
        "[answer(A,wh(what)),answer(B,wh(what))]",
        "solutions([sol([exactly,rational(7,2)])])",
    );
    c.finish();
}
#[test]
fn p5_eso_nonground_outputs_are_barriers_even_inside_conjunctions() {
    let mut c = Cases::new("P5-eso-barrier");
    let facts = [
        "guideline_operator(actual,wrong,should)",
        "guideline_operator(actual,good,should)",
        "guideline_event(good,e,wait)",
        "guideline_event(wrong,e,wait)",
    ];
    for arithmetic in [
        "guideline_product(exactly,2,exactly,3,B,C)",
        "guideline_quotient(exactly,3,exactly,2,B,rational(C,2))",
    ] {
        let ground = if arithmetic.starts_with("guideline_product") {
            "guideline_product(exactly,2,exactly,3,exactly,6)"
        } else {
            "guideline_quotient(exactly,3,exactly,2,exactly,rational(3,2))"
        };
        let goals = [
            "guideline_operator(actual,A,should)".into(),
            arithmetic.into(),
            "guideline_event(A,e,wait)".into(),
        ];
        let rule = format!("guideline_event(actual,p,done) :- {}", goals.join(", "));
        install(
            &c,
            &[
                raw("boxes", facts.iter().map(|s| (*s).into()).collect()),
                raw("policy", vec![rule.clone()]),
            ],
        );
        let roots = [
            node("boxes", facts[0], &[]),
            builtin(ground),
            node("boxes", facts[3], &[]),
        ];
        boolean(
            &mut c,
            arithmetic,
            "guideline_event(actual,p,done)",
            Some(&[node("policy", &rule, &roots)]),
        );
        boolean(
            &mut c,
            &format!("query {arithmetic}"),
            &conjunction(&goals),
            Some(&roots),
        );
    }
    c.finish();
}
#[test]
fn p5_eso_ground_arithmetic_is_scanned_and_never_a_context_binder() {
    let mut c = Cases::new("P5-eso-ground");
    let facts = [
        "guideline_operator(actual,wrong,should)",
        "guideline_operator(actual,good,should)",
        "guideline_event(good,e,wait)",
        "guideline_event(wrong,e,wait)",
    ];
    for arithmetic in [
        "guideline_product(exactly,2,exactly,3,exactly,6)",
        "guideline_quotient(exactly,3,exactly,2,exactly,rational(3,2))",
    ] {
        let goals = [
            "guideline_operator(actual,A,should)".into(),
            arithmetic.into(),
            "guideline_event(A,e,wait)".into(),
        ];
        let rule = format!("guideline_event(actual,p,done) :- {}", goals.join(", "));
        install(
            &c,
            &[
                raw("boxes", facts.iter().map(|s| (*s).into()).collect()),
                raw("policy", vec![rule.clone()]),
            ],
        );
        let roots = [
            node("boxes", facts[1], &[]),
            builtin(arithmetic),
            node("boxes", facts[2], &[]),
        ];
        boolean(
            &mut c,
            arithmetic,
            "guideline_event(actual,p,done)",
            Some(&[node("policy", &rule, &roots)]),
        );
        boolean(
            &mut c,
            &format!("query {arithmetic}"),
            &conjunction(&goals),
            Some(&roots),
        );
    }
    let looping = "guideline_operator(actual,A,should) :- guideline_operator(actual,A,should)";
    install(
        &c,
        &[raw(
            "boxes",
            vec![looping.into(), facts[1].into(), facts[2].into()],
        )],
    );
    for arithmetic in [
        "guideline_product(exactly,2,exactly,3,exactly,7)",
        "guideline_quotient(exactly,3,exactly,0,exactly,0)",
    ] {
        let goals = [
            "guideline_operator(actual,A,should)".into(),
            arithmetic.into(),
            "guideline_event(A,e,wait)".into(),
        ];
        c.answer(
            "ground-false scan through avoids looping operator",
            &conjunction(&goals),
            false,
        );
    }
    // The numeric output may alias a raw query's box context; arithmetic is still a barrier.
    install(
        &c,
        &[raw(
            "boxes",
            vec![
                "guideline_operator(actual,6,should)".into(),
                "guideline_event(6,e,wait)".into(),
            ],
        )],
    );
    let goals = [
        "guideline_operator(actual,A,should)".into(),
        "guideline_product(exactly,2,exactly,3,exactly,A)".into(),
        "guideline_event(A,e,wait)".into(),
    ];
    c.answer("output aliases context", &conjunction(&goals), true);
    c.finish();
}
#[test]
fn p5c_negation_refutes_or_succeeds_without_leaking_bindings() {
    let mut c = Cases::new("P5c-negation");
    for (inner, succeeds) in [
        ("guideline_product(exactly,2,exactly,3,A,B)", false),
        ("guideline_quotient(exactly,3,exactly,2,A,B)", false),
        ("guideline_product(geq,-1,geq,1,A,B)", true),
        (
            "guideline_product(exactly,rational(2,4),exactly,2,A,B)",
            true,
        ),
        ("guideline_quotient(exactly,3,geq,2,A,B)", true),
        ("guideline_quotient(exactly,3,exactly,0,A,B)", true),
    ] {
        let rule = format!("guideline_event(actual,p,done) :- \\+ {inner}");
        install(&c, &[raw("policy", vec![rule])]);
        c.answer(inner, "guideline_event(actual,p,done)", succeeds);
    }
    let rule = "guideline_event(actual,A,done) :- \\+ (guideline_product(exactly,2,exactly,3,exactly,A), guideline_event(actual,absent,stop)), guideline_product(exactly,2,exactly,5,exactly,A)";
    install(&c, &[raw("scope", vec![rule.into()])]);
    answer_result(
        &mut c,
        "inner six never escapes into outer ten",
        "guideline_event(actual,A,done)",
        "[answer(A,wh(what))]",
        "solutions([sol([10])])",
    );
    c.finish();
}
fn depth_clauses(count: usize, terminal: &str) -> Vec<String> {
    (0..count)
        .map(|i| {
            format!(
                "guideline_event(actual,p{i},wait) :- {}",
                if i + 1 == count {
                    terminal.into()
                } else {
                    format!("guideline_event(actual,p{},wait)", i + 1)
                }
            )
        })
        .collect()
}
#[test]
fn p5_depth_zero_prunes_before_valid_or_invalid_arithmetic() {
    let mut c = Cases::new("P5-depth");
    for count in [99, 100] {
        for (terminal, holds) in [
            ("guideline_product(exactly,2,exactly,3,A,B)", true),
            ("guideline_quotient(exactly,2,exactly,0,A,B)", false),
        ] {
            install(&c, &[raw("depth", depth_clauses(count, terminal))]);
            answer_result(
                &mut c,
                &format!("count={count} {terminal}"),
                "guideline_event(actual,p0,wait)",
                "[]",
                if count == 100 {
                    "indeterminate(limit)"
                } else if holds {
                    "yes"
                } else {
                    "no(finite_failure)"
                },
            );
        }
    }
    c.finish();
}
#[test]
fn p5c_depth_zero_inside_negation_is_indeterminate_never_success() {
    let mut c = Cases::new("P5c-depth");
    for count in [98, 99] {
        for (terminal, holds) in [
            ("guideline_product(exactly,2,exactly,3,A,B)", true),
            ("guideline_quotient(exactly,2,exactly,0,A,B)", false),
        ] {
            install(
                &c,
                &[raw(
                    "depth",
                    depth_clauses(count, &format!("\\+ {terminal}")),
                )],
            );
            for wh in [false, true] {
                answer_result(
                    &mut c,
                    &format!("count={count} wh={wh} {terminal}"),
                    if wh {
                        "guideline_event(actual,p0,A)"
                    } else {
                        "guideline_event(actual,p0,wait)"
                    },
                    if wh { "[answer(A,wh(what))]" } else { "[]" },
                    if count == 99 {
                        "indeterminate(limit)"
                    } else if wh {
                        if holds {
                            "solutions([])"
                        } else {
                            "solutions([sol([wait])])"
                        }
                    } else if holds {
                        "no(finite_failure)"
                    } else {
                        "yes"
                    },
                );
            }
        }
    }
    c.finish();
}
fn balanced(first: &str, rest: &str, count: usize) -> String {
    if count == 1 {
        first.into()
    } else {
        let left = count / 2;
        format!(
            "','({},{})",
            balanced(first, rest, left),
            balanced(rest, rest, count - left)
        )
    }
}
#[test]
fn p5_transition_budget_one_dispatch_no_builtin_choicepoint() {
    let mut c = Cases::new("P5-fuel");
    c.write("m.tsv", "");
    let leaf = "guideline_product(exactly,2,exactly,3,exactly,6)";
    for count in [49999, 50000, 50001] {
        for wh in [false, true] {
            let first = if wh {
                "guideline_product(exactly,2,exactly,3,A,B)"
            } else {
                leaf
            };
            answer_result(
                &mut c,
                &format!("count={count} wh={wh}"),
                &balanced(first, leaf, count),
                if wh {
                    "[answer(A,wh(what)),answer(B,wh(what))]"
                } else {
                    "[]"
                },
                if count > 50000 {
                    "indeterminate(limit)"
                } else if wh {
                    "solutions([sol([exactly,6])])"
                } else {
                    "yes"
                },
            );
        }
    }
    c.finish();
}
#[test]
fn p5_failed_output_unification_costs_one_dispatch_before_retry() {
    let mut c = Cases::new("P5-fuel-clash");
    let first = "guideline_event(actual,p,wait)";
    let leaf = "guideline_product(exactly,2,exactly,3,exactly,6)";
    install(
        &c,
        &[raw(
            "choice",
            vec![
                format!("{first} :- guideline_product(exactly,2,exactly,3,exactly,7)"),
                first.into(),
            ],
        )],
    );
    // N leaves: N-1 splits + N calls + yes; the failed clause adds builtin+retry.
    for count in [49998, 49999, 50000] {
        answer_result(
            &mut c,
            &format!("count={count}"),
            &balanced(first, leaf, count),
            "[]",
            if count <= 49999 {
                "yes"
            } else {
                "indeterminate(limit)"
            },
        );
    }
    c.finish();
}
