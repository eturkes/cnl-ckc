use super::common::*;

#[test]
fn p6_evaluated_leaves_are_roots_and_children_and_tampering_is_stale() {
    let mut c = Cases::new("P6-leaves");
    for (call, ground, mutations) in [
        (
            "guideline_product(exactly,3,exactly,rational(1,2),A,B)",
            "guideline_product(exactly,3,exactly,rational(1,2),exactly,rational(3,2))",
            [
                "guideline_product(exactly,3,exactly,rational(1,2),exactly,rational(5,2))",
                "guideline_product(exactly,3,exactly,rational(1,2),exactly,rational(6,4))",
                "guideline_product(exactly,1,exactly,rational(3,2),exactly,rational(3,2))",
                "guideline_product(exactly,3,exactly,rational(1,2),eq,rational(3,2))",
            ],
        ),
        (
            "guideline_quotient(exactly,3,exactly,2,A,B)",
            "guideline_quotient(exactly,3,exactly,2,exactly,rational(3,2))",
            [
                "guideline_quotient(exactly,3,exactly,2,exactly,rational(5,2))",
                "guideline_quotient(exactly,3,exactly,2,exactly,rational(6,4))",
                "guideline_quotient(exactly,6,exactly,4,exactly,rational(3,2))",
                "guideline_quotient(exactly,3,exactly,2,eq,rational(3,2))",
            ],
        ),
    ] {
        for child in [false, true] {
            let rule = format!("guideline_cardinality(actual,p,na,A,B) :- {call}");
            if child {
                install(&c, &[("policy", vec![rule.clone()], String::new())]);
            } else {
                c.write("m.tsv", "");
            }
            let q = query(
                if child {
                    "guideline_cardinality(actual,p,na,A,B)"
                } else {
                    call
                },
                "[answer(A,wh(what)),answer(B,wh(what))]",
            );
            let root = if child {
                node("policy", &rule, &[builtin(ground)])
            } else {
                builtin(ground)
            };
            let result = format!("solutions([sol([exactly,rational(3,2)],proved([{root}]))])");
            chain(
                &mut c,
                &format!("child={child} {call}"),
                &q,
                "solutions([sol([exactly,rational(3,2)])])",
                &result,
            );
            let a = answers(&q, "solutions([sol([exactly,rational(3,2)])])");
            let t = traces(&q, &a, &result);
            for changed in mutations {
                assert_eq!(t.matches(ground).count(), 1);
                c.write("traces.pl", &t.replacen(ground, changed, 1));
                c.expect(
                    &format!("child={child} tamper={changed}"),
                    &[
                        "v1",
                        "trace-check",
                        "m.tsv",
                        "query.pl",
                        "answers.pl",
                        "traces.pl",
                    ],
                    Out::err(1, "ace_to_pl_error(proof,trace_check(stale)).\n"),
                );
            }
        }
    }
    c.finish();
}
#[test]
fn p6_successful_inner_arithmetic_records_only_the_naf_leaf() {
    let mut c = Cases::new("P6-naf");
    let rule = "guideline_event(actual,p,done) :- \\+ (guideline_product(exactly,2,exactly,3,A,B), guideline_event(actual,absent,stop))";
    install(&c, &[("policy", vec![rule.into()], String::new())]);
    let leaf = "naf(','(guideline_product(exactly,2,exactly,3,'$VAR'(0),'$VAR'(1)),guideline_event(actual,absent,stop)))";
    boolean(
        &mut c,
        "local computed six never logged as builtin",
        "guideline_event(actual,p,done)",
        Some(&[node("policy", rule, &[leaf.into()])]),
    );
    let rule =
        "guideline_event(actual,p,done) :- \\+ guideline_quotient(exactly,3,exactly,0,exactly,0)";
    install(&c, &[("policy", vec![rule.into()], String::new())]);
    boolean(
        &mut c,
        "invalid divisor records only NAF",
        "guideline_event(actual,p,done)",
        Some(&[node(
            "policy",
            rule,
            &["naf(guideline_quotient(exactly,3,exactly,0,exactly,0))".into()],
        )]),
    );
    c.finish();
}
#[test]
fn p6_binding_extends_trace_substitution_without_changing_prior_leaves() {
    let mut c = Cases::new("P6-substitution");
    c.write("m.tsv", "");
    let goals = [
        "guideline_satisfies(exactly,rational(1,2),less,1)".into(),
        "guideline_product(exactly,3,exactly,rational(1,2),A,B)".into(),
        "guideline_quotient(A,B,exactly,2,C,D)".into(),
        "guideline_satisfies(C,D,exactly,rational(3,4))".into(),
    ];
    let q = query(
        &conjunction(&goals),
        "[answer(B,wh(what)),answer(D,wh(what))]",
    );
    let roots = [
        "guideline_satisfies(exactly,rational(1,2),less,1)",
        "guideline_product(exactly,3,exactly,rational(1,2),exactly,rational(3,2))",
        "guideline_quotient(exactly,rational(3,2),exactly,2,exactly,rational(3,4))",
        "guideline_satisfies(exactly,rational(3,4),exactly,rational(3,4))",
    ]
    .map(builtin);
    chain(
        &mut c,
        "four root leaves, zero materialized clause nodes",
        &q,
        "solutions([sol([rational(3,2),rational(3,4)])])",
        &format!(
            "solutions([sol([rational(3,2),rational(3,4)],proved([{}]))])",
            roots.join(",")
        ),
    );
    c.finish();
}
#[test]
fn p6_trace_depth_zero_prunes_before_arithmetic_evaluation() {
    let mut c = Cases::new("P6-depth");
    for count in [999, 1000] {
        for truth in [false, true] {
            let terminal = if truth {
                "guideline_product(exactly,2,exactly,3,A,B)"
            } else {
                "guideline_quotient(exactly,2,exactly,0,A,B)"
            };
            let clauses = (0..count)
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
                .collect::<Vec<_>>();
            install(&c, &[("depth", clauses.clone(), String::new())]);
            let q = query("guideline_event(actual,p0,wait)", "[]");
            let a = answers(&q, "yes");
            let result = if count == 1000 {
                "yes(unproved(limit))".into()
            } else if !truth {
                "yes(unproved(finite_failure))".into()
            } else {
                let mut tree = builtin("guideline_product(exactly,2,exactly,3,exactly,6)");
                for clause in clauses.iter().rev() {
                    tree = node("depth", clause, &[tree]);
                }
                format!("yes(proved([{tree}]))")
            };
            c.write("query.pl", &q);
            c.write("answers.pl", &a);
            c.write("traces.pl", &traces(&q, &a, &result));
            c.expect(
                &format!("depth={count} truth={truth}"),
                &["v1", "trace", "m.tsv", "query.pl", "answers.pl"],
                Out::ok(traces(&q, &a, &result)),
            );
            c.expect(
                &format!("trace-check depth={count} truth={truth}"),
                &[
                    "v1",
                    "trace-check",
                    "m.tsv",
                    "query.pl",
                    "answers.pl",
                    "traces.pl",
                ],
                if count == 999 && truth {
                    Out::ok("ckc: trace-check ok arithmetic nodes=999\n")
                } else {
                    Out::err(1, "ace_to_pl_error(proof,trace_check(non_demo)).\n")
                },
            );
        }
    }
    c.finish();
}
#[test]
fn p8_aggregate_replays_computed_heads_and_rejects_wrong_payload_values() {
    let mut c = Cases::new("P8-replay");
    let input = gid("product", "patients", 1, "[]");
    let output = gid("product", "policy", 1, "[]");
    let fact = format!("guideline_cardinality(source,{input},na,geq,60)");
    let head = format!("guideline_cardinality(actual,{output},na,geq,90)");
    let rule = format!(
        "guideline_cardinality(actual,{output},na,A,B) :- guideline_cardinality(source,{input},na,C,D), guideline_product(C,D,exactly,rational(3,2),A,B)"
    );
    for reverse in [false, true] {
        for wrong in [false, true] {
            let payload_head = if wrong {
                head.replace(",geq,90)", ",geq,91)")
            } else {
                head.clone()
            };
            let mut members = vec![
                (
                    "patients",
                    vec![fact.clone()],
                    proof("patients", &[], std::slice::from_ref(&fact)),
                ),
                (
                    "policy",
                    vec![rule.clone()],
                    proof("policy", std::slice::from_ref(&fact), &[payload_head]),
                ),
            ];
            if reverse {
                members.reverse();
            }
            install(&c, &members);
            c.expect(
                &format!("reverse={reverse} wrong={wrong}"),
                &["v1", "aggregate-check", "m.tsv"],
                if wrong {
                    Out::err(
                        1,
                        "ace_to_pl_error(proof,obligation_failed(policy,1,variant(1))).\n",
                    )
                } else {
                    Out::ok("ace_to_pl aggregate ok 2 documents 2 obligations\n")
                },
            );
            c.expect(
                "arithmetic dependency acyclic",
                &["v1", "recursion-check", "m.tsv"],
                Out::ok("ace_to_pl recursion ok 2 documents 1 rule clauses\n"),
            );
        }
    }
    c.finish();
}
#[test]
fn p8_recursion_scan_keeps_leftmost_law_and_builtin_non_edges() {
    let mut c = Cases::new("P8-recursion");
    let h = "guideline_event(actual,p,wait)";
    for first in [
        "guideline_product(exactly,2,exactly,3,A,B)",
        "guideline_quotient(exactly,3,exactly,2,A,B)",
    ] {
        install(
            &c,
            &[(
                "recursive",
                vec![format!("{h} :- {first}, {h}")],
                String::new(),
            )],
        );
        c.expect(
            first,
            &["v1", "recursion-check", "m.tsv"],
            Out::ok("ace_to_pl recursion ok 1 documents 1 rule clauses\n"),
        );
    }
    let h = format!(
        "guideline_unit(actual,{},day)",
        gid("product", "recursive", 1, "[]")
    );
    install(
        &c,
        &[("recursive", vec![format!("{h} :- {h}")], String::new())],
    );
    c.expect(
        "semantic left-recursion control",
        &["v1", "recursion-check", "m.tsv"],
        Out::err(
            1,
            "ace_to_pl_error(proof,left_recursive(sentence(recursive,1),guideline_unit,3)).\n",
        ),
    );
    c.finish();
}
#[test]
fn p7_committed_document_is_the_canonical_independent_fixture() {
    let mut c = Cases::new("P7-fixture");
    let clauses = vec![
        "guideline_cardinality(actual,A,na,B,C) :- guideline_cardinality(source,A,na,D,E), guideline_product(D,E,exactly,rational(3,2),B,C)".into(),
        "guideline_cardinality(half,A,na,B,C) :- guideline_cardinality(actual,A,na,D,E), guideline_quotient(D,E,exactly,2,B,C)".into(),
    ];
    let text = fixture("conversion.pl.in");
    assert_eq!(text, document("conversion", &clauses));
    c.check("fixture", &text);
    c.finish();
}
