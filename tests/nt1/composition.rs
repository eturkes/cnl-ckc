use super::common::*;
use super::oracle::Probe;

fn facts(id: &'static str, clauses: Vec<String>) -> (&'static str, Vec<String>, String) {
    let payload = proof(id, &[], &clauses);
    (id, clauses, payload)
}
fn rules(
    id: &'static str,
    clauses: Vec<String>,
    witness: &[String],
    heads: &[String],
) -> (&'static str, Vec<String>, String) {
    (id, clauses, proof(id, witness, heads))
}
fn card(id: &str, comparison: &str, count: &str) -> String {
    format!("guideline_cardinality(actual,{id},na,{comparison},{count})")
}
fn unit(id: &str, name: &str) -> String {
    format!("guideline_unit(actual,{id},{name})")
}
fn entity(id: &str) -> String {
    format!("guideline_entity(actual,{id},dose,countable)")
}
fn eligible(id: &str) -> String {
    format!("guideline_event(actual,{id},eligible)")
}

#[test]
fn p4_patient_value_sets_answer_and_trace_each_bound() {
    let mut c = Cases::new("P4-patients");
    let patients = [
        ("p_about", "about", "60"),
        ("p_eq", "eq", "60"),
        ("p_exactly", "exactly", "60"),
        ("p_geq", "geq", "60"),
        ("p_greater", "greater", "49"),
        ("p_leq", "leq", "70"),
        ("p_na", "na", "60"),
        ("p_point", "exactly", "40"),
        ("p_upper", "leq", "50"),
        ("p_upper_open", "less", "50"),
    ];
    let clauses: Vec<_> = patients
        .iter()
        .flat_map(|(id, s, n)| [entity(id), card(id, s, n)])
        .collect();
    for (b, m) in [
        ("geq", "50"),
        ("greater", "50"),
        ("less", "50"),
        ("leq", "50"),
        ("exactly", "60"),
    ] {
        let rule = format!(
            "{} :- {}, {}, guideline_satisfies(B,C,{b},{m})",
            eligible("A"),
            entity("A"),
            card("A", "B", "C")
        );
        let witness = [entity("w"), card("w", b, m)];
        let members = [
            facts("patients", clauses.clone()),
            rules("policy", vec![rule.clone()], &witness, &[eligible("w")]),
        ];
        for reverse in [false, true] {
            let mut members = members.clone();
            if reverse {
                members.reverse();
            }
            install(&c, &members);
            let mut sols = Vec::new();
            let mut proofs = Vec::new();
            for (id, s, n) in patients {
                let p = Probe::new(&[s, n, b, m]);
                if p.expected() {
                    sols.push(format!("sol([{id}])"));
                    let children = [
                        node("patients", &entity(id), &[]),
                        node("patients", &card(id, s, n), &[]),
                        builtin(&p.goal()),
                    ];
                    proofs.push(format!(
                        "sol([{id}],proved([{}]))",
                        node("policy", &rule, &children)
                    ));
                }
            }
            chain(
                &mut c,
                &format!("{b} {m} reverse={reverse}"),
                &query(&eligible("A"), "[answer(A,wh(what))]"),
                &format!("solutions([{}])", sols.join(",")),
                &format!("solutions([{}])", proofs.join(",")),
            );
        }
    }
    c.finish();
}
#[test]
fn p4_time_unit_compositions_preserve_group_boundaries() {
    let mut c = Cases::new("P4-unit-composition");
    let quantities = [
        ("days_3", "exactly", "3", "day"),
        ("days_45", "exactly", "45", "day"),
        ("month_1", "exactly", "1", "month"),
        ("months_18", "exactly", "18", "month"),
        ("week_eq", "eq", "1", "week"),
        ("week_exact", "exactly", "1", "week"),
    ];
    let clauses: Vec<_> = quantities
        .iter()
        .flat_map(|(id, s, n, u)| [card(id, s, n), unit(id, u)])
        .collect();
    for (b, m, v) in [
        ("greater", "3", "day"),
        ("geq", "1", "year"),
        ("geq", "1", "month"),
        ("exactly", "7", "day"),
    ] {
        let rule = format!(
            "{} :- {}, {}, guideline_satisfies(B,C,D,{b},{m},{v})",
            eligible("A"),
            card("A", "B", "C"),
            unit("A", "D")
        );
        install(
            &c,
            &[
                facts("quantities", clauses.clone()),
                rules(
                    "policy",
                    vec![rule.clone()],
                    &[card("w", b, m), unit("w", v)],
                    &[eligible("w")],
                ),
            ],
        );
        let mut sols = Vec::new();
        let mut proofs = Vec::new();
        for (id, s, n, u) in quantities {
            let p = Probe::new(&[s, n, u, b, m, v]);
            if p.expected() {
                sols.push(format!("sol([{id}])"));
                let children = [
                    node("quantities", &card(id, s, n), &[]),
                    node("quantities", &unit(id, u), &[]),
                    builtin(&p.goal()),
                ];
                proofs.push(format!(
                    "sol([{id}],proved([{}]))",
                    node("policy", &rule, &children)
                ));
            }
        }
        chain(
            &mut c,
            &format!("{b} {m} {v}"),
            &query(&eligible("A"), "[answer(A,wh(what))]"),
            &format!("solutions([{}])", sols.join(",")),
            &format!("solutions([{}])", proofs.join(",")),
        );
    }
    c.finish();
}
#[test]
fn p4_failing_filter_backtracks_to_the_next_cardinality() {
    let mut c = Cases::new("P4-backtrack");
    let rule = format!(
        "{} :- {}, guideline_satisfies(A,B,geq,50)",
        eligible("p"),
        card("p", "A", "B")
    );
    for amounts in [vec!["40", "60"], vec!["60", "40"], vec!["40", "60", "70"]] {
        let clauses: Vec<_> = amounts.iter().map(|n| card("p", "exactly", n)).collect();
        install(
            &c,
            &[
                facts("patients", clauses),
                rules(
                    "policy",
                    vec![rule.clone()],
                    &[card("p", "geq", "50")],
                    &[eligible("p")],
                ),
            ],
        );
        let children = [
            node("patients", &card("p", "exactly", "60"), &[]),
            builtin("guideline_satisfies(exactly,60,geq,50)"),
        ];
        boolean(
            &mut c,
            &format!("{amounts:?}"),
            &eligible("p"),
            Some(&[node("policy", &rule, &children)]),
        );
        // A second successful cardinality supplies another proof, not another answer.
        chain(
            &mut c,
            "distinct wh result",
            &query("guideline_event(actual,A,eligible)", "[answer(A,wh(what))]"),
            "solutions([sol([p])])",
            &format!(
                "solutions([sol([p],proved([{}]))])",
                node("policy", &rule, &children)
            ),
        );
    }
    c.finish();
}
#[test]
fn p4_naf_filters_flip_without_recording_inner_builtins() {
    let mut c = Cases::new("P4-naf");
    for local in [false, true] {
        let body = if local {
            "\\+ (guideline_cardinality(actual,p,na,A,B), guideline_satisfies(A,B,geq,50))"
        } else {
            "guideline_cardinality(actual,p,na,A,B), \\+ guideline_satisfies(A,B,geq,50)"
        };
        let rule = format!("{} :- {body}", eligible("p"));
        for n in ["40", "60"] {
            let fact = card("p", "exactly", n);
            install(
                &c,
                &[
                    facts("patients", vec![fact.clone()]),
                    rules("policy", vec![rule.clone()], &[], &[]),
                ],
            );
            let roots = if n == "40" {
                let children = if local {
                    vec!["naf(','(guideline_cardinality(actual,p,na,'$VAR'(0),'$VAR'(1)),guideline_satisfies('$VAR'(0),'$VAR'(1),geq,50)))".into()]
                } else {
                    vec![
                        node("patients", &fact, &[]),
                        format!("naf(guideline_satisfies(exactly,{n},geq,50))"),
                    ]
                };
                Some(vec![node("policy", &rule, &children)])
            } else {
                None
            };
            boolean(
                &mut c,
                &format!("local={local} n={n}"),
                &eligible("p"),
                roots.as_deref(),
            );
        }
    }
    c.finish();
}
#[test]
fn p4_builtin_roots_and_tampered_leaves() {
    let mut c = Cases::new("P4-builtin-traces");
    let goal = "guideline_satisfies(exactly,60,geq,50)";
    for child in [false, true] {
        let rule = format!("{} :- {goal}", eligible("p"));
        if child {
            install(
                &c,
                &[rules("policy", vec![rule.clone()], &[], &[eligible("p")])],
            );
        } else {
            c.write("m.tsv", "");
        }
        let q = query(
            if child {
                "guideline_event(actual,p,eligible)"
            } else {
                goal
            },
            "[]",
        );
        let roots = if child {
            vec![node("policy", &rule, &[builtin(goal)])]
        } else {
            vec![builtin(goal)]
        };
        let trace_result = format!("yes(proved([{}]))", roots.join(","));
        chain(&mut c, &format!("child={child}"), &q, "yes", &trace_result);
        let a = answers(&q, "yes");
        let t = traces(&q, &a, &trace_result);
        for (label, changed) in [
            ("false-value", "guideline_satisfies(exactly,49,geq,50)"),
            ("still-true-value", "guideline_satisfies(exactly,61,geq,50)"),
            (
                "still-true-bound",
                "guideline_satisfies(exactly,60,greater,50)",
            ),
            ("false-bound", "guideline_satisfies(exactly,60,geq,61)"),
        ] {
            assert_eq!(t.matches(goal).count(), 1);
            c.write("traces.pl", &t.replacen(goal, changed, 1));
            c.expect(
                &format!("child={child} {label}"),
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
    c.write("m.tsv", "");
    let other = "guideline_satisfies(exactly,1,week,geq,7,day)";
    boolean(
        &mut c,
        "two builtin roots, zero clause nodes",
        &conjunction(&[goal.into(), other.into()]),
        Some(&[builtin(goal), builtin(other)]),
    );
    c.finish();
}

#[test]
fn p4_eso_filters_never_bind_or_wait_past_an_unbound_barrier() {
    let mut c = Cases::new("P4-eso-unbound");
    let op = "guideline_operator(actual,geq,should)";
    let content = "guideline_event(geq,e,wait)";
    let filter = "guideline_satisfies(geq,60,geq,50)";
    for later in [false, true] {
        let mut goals = vec![
            "guideline_operator(actual,A,should)".into(),
            "guideline_satisfies(A,60,geq,50)".into(),
        ];
        let mut children = vec![node("boxes", op, &[]), builtin(filter)];
        if later {
            goals.push("guideline_event(A,e,wait)".into());
            children.push(node("boxes", content, &[]));
        }
        let rule = format!("{} :- {}", eligible("p"), goals.join(", "));
        install(
            &c,
            &[
                facts("boxes", vec![op.into(), content.into()]),
                rules("policy", vec![rule.clone()], &[], &[]),
            ],
        );
        boolean(
            &mut c,
            &format!("rule later-binder={later}"),
            &eligible("p"),
            Some(&[node("policy", &rule, &children)]),
        );
        // The right-associated question nests its filter before its binder.
        // Splitting the conjunction evaluates no leaf; the exposed barrier applies next.
        boolean(
            &mut c,
            &format!("question later-binder={later}"),
            &conjunction(&goals),
            Some(&children),
        );
    }
    c.finish();
}
#[test]
fn p4_eso_scans_ground_filters_but_stops_at_naf() {
    let mut c = Cases::new("P4-eso-ground");
    let ops = [
        "guideline_operator(actual,wrong,should)",
        "guideline_operator(actual,geq,should)",
    ];
    let contents = [
        "guideline_event(geq,e,wait)",
        "guideline_event(wrong,e,wait)",
    ];
    let filter = "guideline_satisfies(exactly,60,geq,50)";
    for naf in [false, true] {
        let mut body = vec!["guideline_operator(actual,A,should)".into(), filter.into()];
        if naf {
            body.push("\\+ guideline_property(A,p,blocked,pos)".into());
        }
        body.push("guideline_event(A,e,wait)".into());
        let rule = format!("{} :- {}", eligible("p"), body.join(", "));
        let clauses = ops.iter().chain(&contents).map(|s| (*s).into()).collect();
        install(
            &c,
            &[
                facts("boxes", clauses),
                rules("policy", vec![rule.clone()], &[], &[]),
            ],
        );
        let mut children = vec![node("boxes", ops[usize::from(!naf)], &[]), builtin(filter)];
        if naf {
            children.push("naf(guideline_property(wrong,p,blocked,pos))".into());
        }
        children.push(node("boxes", contents[usize::from(naf)], &[]));
        boolean(
            &mut c,
            &format!("rule naf={naf}"),
            &eligible("p"),
            Some(&[node("policy", &rule, &children)]),
        );
        if !naf {
            boolean(
                &mut c,
                "question ground filter scanned",
                &conjunction(&body),
                Some(&children),
            );
        }
    }
    c.finish();
}
#[test]
fn p4_aggregate_replay_and_leftmost_recursion_law() {
    let mut c = Cases::new("P4-replay-recursion");
    // Loaded sentence identities, not payloads, establish aggregate coverage.
    let quantity = gid("product", "patients", 1, "[]");
    let outcome = gid("product", "policy", 1, "[]");
    let rule = format!(
        "{} :- {}, {}, guideline_satisfies(A,B,C,geq,1,year)",
        eligible(&outcome),
        card(&quantity, "A", "B"),
        unit(&quantity, "C")
    );
    let patients = vec![card(&quantity, "exactly", "18"), unit(&quantity, "month")];
    let members = [
        facts("patients", patients),
        rules(
            "policy",
            vec![rule],
            &[card(&quantity, "geq", "1"), unit(&quantity, "year")],
            &[eligible(&outcome)],
        ),
    ];
    for reverse in [false, true] {
        let mut m = members.clone();
        if reverse {
            m.reverse();
        }
        install(&c, &m);
        c.expect(
            "aggregate with reflexive witness; builtins excluded",
            &["v1", "aggregate-check", "m.tsv"],
            Out::ok("ace_to_pl aggregate ok 2 documents 2 obligations\n"),
        );
        c.expect(
            "acyclic comparison rule",
            &["v1", "recursion-check", "m.tsv"],
            Out::ok("ace_to_pl recursion ok 2 documents 1 rule clauses\n"),
        );
    }
    for arity in [4, 6] {
        let filter = if arity == 4 {
            "guideline_satisfies(exactly,60,geq,50)"
        } else {
            "guideline_satisfies(exactly,1,year,geq,12,month)"
        };
        let rule = format!("{} :- {filter}, {}", eligible("p"), eligible("p"));
        install(&c, &[rules("recursive", vec![rule], &[], &[])]);
        c.expect(
            &format!("builtin /{arity} has no head edge"),
            &["v1", "recursion-check", "m.tsv"],
            Out::ok("ace_to_pl recursion ok 1 documents 1 rule clauses\n"),
        );
    }
    let lit = unit(&gid("product", "recursive", 1, "[]"), "day");
    install(
        &c,
        &[rules(
            "recursive",
            vec![format!("{lit} :- {lit}")],
            &[],
            &[],
        )],
    );
    c.expect(
        "unit is a semantic recursion edge",
        &["v1", "recursion-check", "m.tsv"],
        Out::err(
            1,
            "ace_to_pl_error(proof,left_recursive(sentence(recursive,1),guideline_unit,3)).\n",
        ),
    );
    c.finish();
}

#[test]
fn p4_builtin_depth_zero_prunes_before_evaluation() {
    let mut c = Cases::new("P4-depth");
    for count in [99, 100] {
        for truth in [false, true] {
            let filter = format!(
                "guideline_satisfies(exactly,{},geq,50)",
                if truth { 60 } else { 40 }
            );
            let clauses: Vec<_> = (0..count)
                .map(|i| {
                    let body = if i + 1 == count {
                        filter.clone()
                    } else {
                        format!("guideline_event(actual,p{},wait)", i + 1)
                    };
                    format!("guideline_event(actual,p{i},wait) :- {body}")
                })
                .collect();
            install(&c, &[rules("depth", clauses, &[], &[])]);
            let q = query("guideline_event(actual,p0,wait)", "[]");
            c.write("query.pl", &q);
            let result = if count == 100 {
                "indeterminate(limit)"
            } else if truth {
                "yes"
            } else {
                "no(finite_failure)"
            };
            c.expect(
                &format!("depth-chain={count} truth={truth}"),
                &["v1", "answer", "m.tsv", "query.pl"],
                Out::ok(answers(&q, result)),
            );
        }
    }
    c.finish();
}

#[test]
fn p4_committed_documents_match_the_contract_model() {
    let mut c = Cases::new("P4-committed");
    let patients = vec![card("p", "exactly", "18"), unit("p", "month")];
    let rule = format!(
        "{} :- {}, {}, guideline_satisfies(A,B,C,geq,1,year)",
        eligible("p"),
        card("p", "A", "B"),
        unit("p", "C")
    );
    for (id, clauses) in [
        ("patients", patients.clone()),
        ("policy", vec![rule.clone()]),
    ] {
        let text = fixture(&format!("{id}.pl.in"));
        assert_eq!(
            text,
            document(id, &clauses),
            "committed input = independent canonical model"
        );
        c.check(id, &text);
    }
    install(
        &c,
        &[
            facts("patients", patients.clone()),
            rules(
                "policy",
                vec![rule.clone()],
                &[card("p", "geq", "1"), unit("p", "year")],
                &[eligible("p")],
            ),
        ],
    );
    let mut children: Vec<_> = patients.iter().map(|s| node("patients", s, &[])).collect();
    children.push(builtin("guideline_satisfies(exactly,18,month,geq,1,year)"));
    boolean(
        &mut c,
        "18 months meets at least 1 year",
        &eligible("p"),
        Some(&[node("policy", &rule, &children)]),
    );
    c.finish();
}

#[test]
fn p4_builtin_trace_depth_zero_prunes_before_evaluation() {
    let mut c = Cases::new("P4-trace-depth");
    for count in [999, 1000] {
        for truth in [false, true] {
            let filter = format!(
                "guideline_satisfies(exactly,{},geq,50)",
                if truth { 60 } else { 40 }
            );
            let clauses: Vec<_> = (0..count)
                .map(|i| {
                    let body = if i + 1 == count {
                        filter.clone()
                    } else {
                        format!("guideline_event(actual,p{},wait)", i + 1)
                    };
                    format!("guideline_event(actual,p{i},wait) :- {body}")
                })
                .collect();
            install(&c, &[rules("depth", clauses.clone(), &[], &[])]);
            let q = query("guideline_event(actual,p0,wait)", "[]");
            let a = answers(&q, "yes");
            let result = if count == 1000 {
                "yes(unproved(limit))".into()
            } else if !truth {
                "yes(unproved(finite_failure))".into()
            } else {
                let mut tree = builtin(&filter);
                for clause in clauses.iter().rev() {
                    tree = node("depth", clause, &[tree]);
                }
                format!("yes(proved([{tree}]))")
            };
            let t = traces(&q, &a, &result);
            c.write("query.pl", &q);
            c.write("answers.pl", &a);
            c.write("traces.pl", &t);
            let label = format!("trace-depth-chain={count} truth={truth}");
            c.expect(
                &label,
                &["v1", "trace", "m.tsv", "query.pl", "answers.pl"],
                Out::ok(t),
            );
            c.expect(
                &format!("{label}/check"),
                &[
                    "v1",
                    "trace-check",
                    "m.tsv",
                    "query.pl",
                    "answers.pl",
                    "traces.pl",
                ],
                if count == 999 && truth {
                    Out::ok("ckc: trace-check ok numeric nodes=999\n")
                } else {
                    Out::err(1, "ace_to_pl_error(proof,trace_check(non_demo)).\n")
                },
            );
        }
    }
    c.finish();
}
