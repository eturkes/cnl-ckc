use super::common::*;
use super::model::*;

fn rules(members: &[Member]) -> usize {
    members
        .iter()
        .flat_map(|m| &m.clauses)
        .filter(|s| s.contains(" :- "))
        .count()
}
fn policy_roots(
    policy: &Member,
    patient: &Member,
    key: Option<&str>,
    nested: bool,
    payload: bool,
) -> Vec<String> {
    let mut children = patient.fact_traces()[..2].to_vec();
    if !payload && let Some(key) = key {
        children.push(format!("naf({})", exclusion(key)));
    }
    let at = usize::from(nested) + usize::from(payload);
    vec![policy.trace(at, &children)]
}
fn policy_goal(id: &str, op: &str, nested: bool, payload: bool, patient_id: &str) -> String {
    let deps = format!("[{}]", gid("product", patient_id, 1, "[]"));
    let ctx = context(id, if nested { 2 } else { 1 }, &deps);
    if payload {
        format!(
            "guideline_event({ctx},{},wait)",
            gid("product", id, 2, &deps)
        )
    } else {
        let outer = if nested {
            context(id, 1, &deps)
        } else {
            "actual".into()
        };
        format!("guideline_operator({outer},{ctx},{op})")
    }
}

#[test]
fn b2_knowledge_only_replay_and_each_edges_own_witness() {
    let mut c = Cases::new("B2-knowledge");
    let mut scenes = Vec::new();
    for (i, op) in ["should", "must", "may"].iter().enumerate() {
        for nested in [false, true] {
            scenes.push(recommendation(
                &format!("policy-{i}-{}", usize::from(nested)),
                Some("alpha"),
                &[*op],
                nested,
            ));
            scenes.push(split_recommendation(
                &format!("split-{i}-{}", usize::from(nested)),
                "alpha",
                op,
                nested,
            ));
        }
    }
    let excluded = patient_exclusion("excluded", "alpha", "pain");
    let mut knowledge: Vec<_> = scenes.iter().map(Scene::member).collect();
    knowledge.push(excluded);
    for reverse in [false, true] {
        let mut members = knowledge.clone();
        if reverse {
            members.reverse();
        }
        install(&c, &members);
        c.expect(
            &format!("reverse={reverse}/aggregate"),
            &["v1", "aggregate-check", "m.tsv"],
            Out::ok(format!(
                "ace_to_pl aggregate ok {} documents {} obligations\n",
                members.len(),
                1 + scenes.iter().map(|s| s.groups.len()).sum::<usize>()
            )),
        );
        c.expect(
            &format!("reverse={reverse}/scan"),
            &["v1", "recursion-check", "m.tsv"],
            Out::ok(format!(
                "ace_to_pl recursion ok {} documents {} rule clauses\n",
                members.len(),
                rules(&members)
            )),
        );
        c.answer(
            "knowledge-only has no matched exclusion",
            &exclusion("alpha"),
            false,
        );
        for scene in &scenes {
            for group in &scene.groups {
                let bind = |s: &str| {
                    substitute(s, |v| {
                        gid(
                            "witness",
                            &scene.id,
                            v as usize - 'A' as usize + 1,
                            &format!("variant({})", group.k),
                        )
                    })
                };
                let witness: Vec<_> = group.body.iter().map(|s| bind(s)).collect();
                let mut with_witness = members.clone();
                with_witness.push(Member::raw("witness-input", witness));
                install(&c, &with_witness);
                for head in &group.heads {
                    c.answer(
                        &format!(
                            "{}/variant={}/reverse={reverse}/witness/{head}",
                            scene.id, group.k
                        ),
                        &bind(head),
                        true,
                    );
                }
                install(&c, &members);
            }
        }
    }
    c.finish();
}

#[derive(Clone, Copy)]
struct PatientState {
    pain: bool,
    fever: bool,
    discharged: bool,
}
// Independent finite-world oracle: no clause parsing, solving, or output-derived data.
fn blocked(states: &[PatientState], first_key: bool) -> bool {
    states
        .iter()
        .any(|s| (if first_key { s.pain } else { s.fever }) && !s.discharged)
}
#[test]
fn b3_generated_patient_worlds_gate_edges_not_payloads_or_other_keys() {
    let mut c = Cases::new("B3-world-property");
    for keys in [["alpha", "beta"], ["9", "a--z-"]] {
        for mask in 0..8 {
            let state = PatientState {
                pain: mask & 1 != 0,
                fever: mask & 2 != 0,
                discharged: mask & 4 != 0,
            };
            let facts = patient("case", state.pain, state.fever, state.discharged);
            let policies = [
                ("should", false, Some(keys[0])),
                ("must", true, Some(keys[0])),
                ("may", false, Some(keys[0])),
                ("should", true, Some(keys[1])),
                ("may", false, Some(keys[1])),
                ("must", true, None),
            ];
            let mut members = vec![
                facts.clone(),
                patient_exclusion("exclude-a", keys[0], "pain"),
                patient_exclusion("exclude-b", keys[1], "fever"),
            ];
            let claims: Vec<_> = policies
                .iter()
                .enumerate()
                .map(|(i, (op, nested, key))| {
                    recommendation(&format!("policy-{i}"), *key, &[*op], *nested).member()
                })
                .collect();
            members.extend(claims.iter().cloned());
            for reverse in [false, true] {
                let mut ordered = members.clone();
                if reverse {
                    ordered.reverse();
                }
                install(&c, &ordered);
                let label = format!("keys={keys:?}/mask={mask}/reverse={reverse}");
                for (i, (op, nested, key)) in policies.iter().enumerate() {
                    let derives = key.is_none_or(|key| !blocked(&[state], key == keys[0]));
                    let goal = policy_goal(&claims[i].id, op, *nested, false, "case");
                    let roots =
                        derives.then(|| policy_roots(&claims[i], &facts, *key, *nested, false));
                    boolean(
                        &mut c,
                        &format!("{label}/edge-{i}"),
                        &goal,
                        roots.as_deref(),
                    );
                    let goal = policy_goal(&claims[i].id, op, *nested, true, "case");
                    boolean(
                        &mut c,
                        &format!("{label}/payload-{i}"),
                        &goal,
                        Some(&policy_roots(&claims[i], &facts, *key, *nested, true)),
                    );
                    if *nested {
                        let ctx = context(
                            &claims[i].id,
                            1,
                            &format!("[{}]", gid("product", "case", 1, "[]")),
                        );
                        boolean(
                            &mut c,
                            &format!("{label}/can-remains-{i}"),
                            &format!("guideline_operator(actual,{ctx},can)"),
                            Some(&[claims[i].trace(0, &facts.fact_traces()[..2])]),
                        );
                    }
                }
                for (i, key) in keys.iter().enumerate() {
                    c.answer(
                        &format!("{label}/record-{key}"),
                        &exclusion(key),
                        blocked(&[state], i == 0),
                    );
                }
                c.expect(
                    &format!("{label}/guard-scan"),
                    &["v1", "recursion-check", "m.tsv"],
                    Out::ok(format!(
                        "ace_to_pl recursion ok {} documents {} rule clauses\n",
                        ordered.len(),
                        rules(&ordered)
                    )),
                );
            }
        }
    }
    c.finish();
}

#[test]
fn b3_should_question_and_facility_discharge_exception() {
    let mut c = Cases::new("B3-question-exception");
    let policy = recommendation("policy", Some("alpha"), &["should"], false).member();
    let excluded = patient_exclusion("excluded", "alpha", "pain");
    for pain in [false, true] {
        for discharge in [false, true] {
            let facts = patient("case", pain, false, discharge);
            install(&c, &[facts.clone(), policy.clone(), excluded.clone()]);
            let qgoal = "','(guideline_operator(actual,A,should),guideline_event(A,B,wait))";
            let roots = if pain && !discharge {
                None
            } else {
                let mut roots = policy_roots(&policy, &facts, Some("alpha"), false, false);
                roots.extend(policy_roots(&policy, &facts, Some("alpha"), false, true));
                Some(roots)
            };
            boolean(
                &mut c,
                &format!("pain={pain}/discharge={discharge}"),
                qgoal,
                roots.as_deref(),
            );
            c.answer(
                "no classical negation of the action invented",
                "','(guideline_operator(actual,A,-),guideline_event(A,B,wait))",
                false,
            );
            if pain && !discharge {
                let p = gid("product", "case", 1, "[]");
                let mut local = entity("actual", "'$VAR'(0)", "facility-discharge");
                local.extend(event("actual", "'$VAR'(1)", "have", &p, Some("'$VAR'(0)")));
                let mut children = facts.fact_traces()[..7].to_vec();
                children.push(format!("naf({})", conjunction(&local)));
                boolean(
                    &mut c,
                    "record retains its sentence and NAF exception",
                    &exclusion("alpha"),
                    Some(&[excluded.trace(excluded.clauses.len() - 1, &children)]),
                );
            }
        }
    }
    c.finish();
}

#[test]
fn b3_any_loaded_patient_excludes_the_key_for_every_patient() {
    let mut c = Cases::new("B3-one-patient-limit");
    let policy = recommendation("policy", Some("alpha"), &["should"], false).member();
    let excluded = patient_exclusion("excluded", "alpha", "pain");
    for discharge in [false, true] {
        let states = [
            PatientState {
                pain: false,
                fever: false,
                discharged: false,
            },
            PatientState {
                pain: true,
                fever: false,
                discharged: discharge,
            },
        ];
        let members = [
            patient("one", false, false, false),
            patient("two", true, false, discharge),
            policy.clone(),
            excluded.clone(),
        ];
        install(&c, &members);
        for id in ["one", "two"] {
            c.answer(
                &format!("patient={id}/discharge={discharge}"),
                &policy_goal("policy", "should", false, false, id),
                !blocked(&states, true),
            );
            c.answer(
                &format!("payload={id}/discharge={discharge}"),
                &policy_goal("policy", "should", false, true, id),
                true,
            );
        }
    }
    c.finish();
}

#[test]
fn b3_witness_that_meets_an_exclusion_fails_aggregate_replay() {
    let mut c = Cases::new("B3-obligation");
    // This exclusion's body is exactly the recommendation's witness condition.
    let excluded = exclusion_scene("excluded", "alpha", ExclusionForm::OnePp).member();
    for op in ["should", "must", "may"] {
        let policy = recommendation("policy", Some("alpha"), &[op], false).member();
        for reverse in [false, true] {
            let mut members = vec![policy.clone(), excluded.clone()];
            if reverse {
                members.reverse();
            }
            install(&c, &members);
            c.expect(
                &format!("{op}/reverse={reverse}"),
                &["v1", "aggregate-check", "m.tsv"],
                Out::err(
                    1,
                    "ace_to_pl_error(proof,obligation_failed(policy,1,variant(1))).\n",
                ),
            );
        }
    }
    c.finish();
}

#[test]
fn b3_exclusion_query_solutions_are_keyed_sorted_and_distinct() {
    let mut c = Cases::new("B3-record-solutions");
    let a = Member::raw("first", vec![exclusion("z-key"), exclusion("alpha")]);
    let b = Member::raw("second", vec![exclusion("alpha"), exclusion("0")]);
    for reverse in [false, true] {
        let mut members = vec![a.clone(), b.clone()];
        if reverse {
            members.reverse();
        }
        install(&c, &members);
        let source = if reverse { &b } else { &a };
        let roots = [
            b.trace(1, &[]),
            source.trace(if reverse { 0 } else { 1 }, &[]),
            a.trace(0, &[]),
        ];
        let rows = ["'0'", "alpha", "'z-key'"];
        let trace_rows: Vec<_> = rows
            .iter()
            .zip(roots)
            .map(|(key, proof)| format!("sol([{key}],proved([{proof}]))"))
            .collect();
        chain(
            &mut c,
            &format!("reverse={reverse}"),
            &query("guideline_exclusion(A)", "[answer(A,wh(what))]"),
            "solutions([sol(['0']),sol([alpha]),sol(['z-key'])])",
            &format!("solutions([{}])", trace_rows.join(",")),
        );
    }
    c.finish();
}

#[test]
fn b5_guard_trace_leaf_is_required_and_custody_checked() {
    let mut c = Cases::new("B5-trace");
    let facts = patient("case", false, false, false);
    let policy = recommendation("policy", Some("alpha"), &["should"], false).member();
    let excluded = patient_exclusion("excluded", "alpha", "pain");
    install(&c, &[facts.clone(), policy.clone(), excluded]);
    let roots = policy_roots(&policy, &facts, Some("alpha"), false, false);
    let q = query(&policy_goal("policy", "should", false, false, "case"), "[]");
    let result = format!("yes(proved([{}]))", roots.join(","));
    chain(&mut c, "guard NAF event", &q, "yes", &result);
    let a = answers(&q, "yes");
    let clean = traces(&q, &a, &result);
    for (label, from, to) in [
        (
            "rekeyed",
            "naf(guideline_exclusion(alpha))",
            "naf(guideline_exclusion(beta))",
        ),
        ("omitted", ",naf(guideline_exclusion(alpha))", ""),
    ] {
        assert_eq!(clean.matches(from).count(), 1);
        c.write("traces.pl", &clean.replacen(from, to, 1));
        c.expect(
            label,
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
    c.finish();
}

#[test]
fn b5_deep_exclusion_is_indeterminate_at_the_answer_depth_boundary() {
    let mut c = Cases::new("B5-depth");
    let ctx = context("policy", 1, "[]");
    let edge = format!("guideline_operator(actual,{ctx},should)");
    let goal = |n: usize| format!("guideline_event(actual,n{n},chain)");
    // Answer depth 100: edge → NAF → record → n0 leaves 97 at n0.
    for links in [0, 95, 96, 97, 98, 105] {
        for terminal in [false, true] {
            for shallow_proof in [false, true] {
                let mut clauses = vec![
                    format!("{edge} :- \\+ {}", exclusion("alpha")),
                    format!("{} :- {}", exclusion("alpha"), goal(0)),
                ];
                for n in 0..links {
                    clauses.push(format!("{} :- {}", goal(n), goal(n + 1)));
                }
                if terminal {
                    clauses.push(goal(links));
                }
                if shallow_proof {
                    clauses.push(exclusion("alpha"));
                }
                install(&c, &[Member::raw("depth", clauses)]);
                let result = if shallow_proof || (terminal && links <= 96) {
                    "no(finite_failure)"
                } else if links >= 97 {
                    "indeterminate(limit)"
                } else {
                    "yes"
                };
                let label = format!("links={links}/terminal={terminal}/shallow={shallow_proof}");
                let q = query(&edge, "[]");
                if result == "yes" {
                    let clause = format!("{edge} :- \\+ {}", exclusion("alpha"));
                    let roots = [node(
                        "depth",
                        &clause,
                        &[format!("naf({})", exclusion("alpha"))],
                    )];
                    chain(
                        &mut c,
                        &label,
                        &q,
                        result,
                        &format!("yes(proved([{}]))", roots.join(",")),
                    );
                } else {
                    chain(&mut c, &label, &q, result, result);
                }
            }
        }
    }
    c.finish();
}
