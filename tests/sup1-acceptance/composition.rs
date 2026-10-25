use super::model::*;
use super::support::*;

fn rule_count(members: &[Member]) -> usize {
    members
        .iter()
        .flat_map(|m| &m.clauses)
        .filter(|(_, line)| line.contains(" :- "))
        .count()
}
fn policy_goal(scene: &Scene, at: usize) -> String {
    substitute(&scene.groups[0][0].heads[at], |_| "p".into())
}
fn policy_trace(
    scene: &Scene,
    policy: &Member,
    at: usize,
    patient: &Member,
    dates: &Member,
    date_index: usize,
    value: &str,
) -> String {
    let mut children = vec![
        patient.trace(0, &[]),
        patient.trace(1, &[]),
        dates.trace(date_index, &[]),
    ];
    children.push(format!(
        "builtin(guideline_satisfies(exactly,{value},less,{}))",
        scene.date.unwrap()
    ));
    let head = &scene.groups[0][0].heads[at];
    if scene.scope
        && head.starts_with("guideline_operator(")
        && [",should)", ",must)", ",may)"]
            .iter()
            .any(|end| head.ends_with(end))
    {
        children.push("naf(guideline_exclusion(alpha))".into());
    }
    policy.trace(at, &children)
}
#[test]
fn p4_knowledge_only() {
    let mut c = Cases::new("P4-knowledge-only");
    // The excluded scene would deliberately refute the scoped scenes' witnesses.
    let scenes: Vec<_> = goldens()
        .into_iter()
        .filter(|s| s.id != "excluded")
        .collect();
    let knowledge: Vec<_> = scenes.iter().map(Scene::member).collect();
    for reverse in [false, true] {
        let mut ordered = knowledge.clone();
        if reverse {
            ordered.reverse();
        }
        install(&c, &ordered);
        c.expect(
            &format!("reverse={reverse}/aggregate"),
            &["v1", "aggregate-check", "m.tsv"],
            Out::ok(format!(
                "ace_to_pl aggregate ok {} documents {} obligations\n",
                ordered.len(),
                scenes
                    .iter()
                    .flat_map(|s| &s.groups)
                    .map(Vec::len)
                    .sum::<usize>()
            )),
        );
        for op in ["should", "must", "may"] {
            boolean(
                &mut c,
                &format!("reverse={reverse}/{op}"),
                &format!("guideline_operator(A,B,{op})"),
                None,
            );
        }
        for scene in &scenes {
            let member = scene.member();
            for s in 1..=scene.groups.len() {
                let at = member
                    .clauses
                    .iter()
                    .position(|(sentence, line)| {
                        *sentence == s && line.starts_with("guideline_supersession(")
                    })
                    .unwrap();
                boolean(
                    &mut c,
                    &format!("reverse={reverse}/{}/record-{s}", scene.id),
                    &record(&scene.id, s, scene.date.unwrap()),
                    Some(&[member.trace(at, &[])]),
                );
            }
            assert!(!member.payload.contains("guideline_supersession("));
        }
    }
    c.finish();
}
#[test]
fn p4_date_worlds() {
    let mut c = Cases::new("P4-date-worlds");
    let scenes = [
        recommendation("should-rule", &["should"], None, Some(DATE), false),
        recommendation("must-rule", &["must"], None, Some(DATE), false),
        recommendation("may-rule", &["may"], None, Some(DATE), false),
        recommendation("nested-can", &["should"], Some("can"), Some(DATE), false),
        recommendation("nested-minus", &["should"], Some("-"), Some(DATE), false),
        recommendation("scoped-rule", &["should"], None, Some(DATE), true),
    ];
    let date_sets = [
        vec![],
        vec![DATE - 1],
        vec![DATE],
        vec![DATE + 1],
        vec![DATE - 1, DATE + 1],
        vec![DATE + 1, DATE - 1],
    ];
    for scene in scenes {
        let policy = scene.member();
        for present in [false, true] {
            for excluded in [false, true] {
                let p = patient(present);
                for ds in &date_sets {
                    let values: Vec<_> = ds.iter().map(ToString::to_string).collect();
                    let d = dates(&values);
                    let mut members = vec![
                        policy.clone(),
                        p.clone(),
                        d.clone(),
                        blocked_scope(excluded),
                    ];
                    // Date order controls only which successful fact supplies the first proof.
                    if excluded {
                        members.reverse();
                    }
                    install(&c, &members);
                    let first = ds.iter().position(|n| *n < DATE);
                    for (at, head) in scene.groups[0][0].heads.iter().enumerate() {
                        let edge = head.starts_with("guideline_operator(")
                            && [",should)", ",must)", ",may)"]
                                .iter()
                                .any(|end| head.ends_with(end));
                        let holds =
                            present && first.is_some() && !(edge && scene.scope && excluded);
                        let roots = holds.then(|| {
                            vec![policy_trace(
                                &scene,
                                &policy,
                                at,
                                &p,
                                &d,
                                first.unwrap(),
                                &values[first.unwrap()],
                            )]
                        });
                        boolean(
                            &mut c,
                            &format!(
                                "{}/patient={present}/excluded={excluded}/dates={ds:?}/head={at}",
                                scene.id
                            ),
                            &policy_goal(&scene, at),
                            roots.as_deref(),
                        );
                    }
                    let record_index = policy.clauses.len() - 1;
                    boolean(
                        &mut c,
                        "record unconditional",
                        &record(&scene.id, 1, DATE),
                        Some(&[policy.trace(record_index, &[])]),
                    );
                }
            }
        }
    }
    c.finish();
}
#[test]
fn p4_trace_custody() {
    let mut c = Cases::new("P4-trace-custody");
    let scene = recommendation("policy", &["should"], None, Some(DATE), true);
    let policy = scene.member();
    let p = patient(true);
    let values = vec![(DATE - 1).to_string()];
    let d = dates(&values);
    install(&c, &[policy.clone(), p.clone(), d.clone()]);
    let q = query(&policy_goal(&scene, 0), "[]");
    let date_node = d.trace(0, &[]);
    let leaf = format!(
        "builtin(guideline_satisfies(exactly,{},less,{DATE}))",
        DATE - 1
    );
    let root = policy_trace(&scene, &policy, 0, &p, &d, 0, &values[0]);
    let result = format!("yes(proved([{root}]))");
    chain(
        &mut c,
        "full date + comparison + scope proof",
        &q,
        "yes",
        &result,
    );
    let a = answers(&q, "yes");
    let clean = traces(&q, &a, &result);
    let mutations = [
        ("date-node-dropped", format!(",{date_node}"), String::new()),
        ("comparison-dropped", format!(",{leaf}"), String::new()),
        (
            "date-altered",
            leaf.clone(),
            leaf.replace(&(DATE - 1).to_string(), &DATE.to_string()),
        ),
        (
            "effective-date-altered",
            leaf.clone(),
            leaf.replace(&format!("less,{DATE}"), &format!("less,{}", DATE + 1)),
        ),
        (
            "comparison-reversed",
            leaf.clone(),
            leaf.replace(",less,", ",greater,"),
        ),
        (
            "scope-dropped",
            ",naf(guideline_exclusion(alpha))".into(),
            String::new(),
        ),
    ];
    for (label, from, to) in mutations {
        assert_eq!(clean.matches(&from).count(), 1, "{label}");
        c.write("traces.pl", &clean.replacen(&from, &to, 1));
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
fn p4_transitive() {
    let mut c = Cases::new("P4-transitive");
    let scene = recommendation("retired", &["should"], None, Some(DATE), false);
    let policy = scene.member();
    let p = patient(true);
    let payload = policy_goal(&scene, 1);
    let dependent = Member::raw(
        "dependent",
        &[format!(
            "guideline_property(actual,p,dependent,pos) :- {payload}"
        )],
    );
    let indirect = Member::raw("indirect", &["guideline_property(actual,p,indirect,pos) :- guideline_property(actual,p,dependent,pos)".into()]);
    let control = Member::raw(
        "control",
        &["guideline_property(actual,p,independent,pos)".into()],
    );
    let independent_bytes = control.pl.clone();
    for ds in [
        vec![],
        vec![DATE - 1],
        vec![DATE],
        vec![DATE + 1],
        vec![DATE + 1, DATE - 1],
    ] {
        let values: Vec<_> = ds.iter().map(ToString::to_string).collect();
        let d = dates(&values);
        let first = ds.iter().position(|n| *n < DATE);
        for reverse in [false, true] {
            let mut members = vec![
                policy.clone(),
                p.clone(),
                dependent.clone(),
                indirect.clone(),
                control.clone(),
                d.clone(),
            ];
            if reverse {
                members.reverse();
            }
            install(&c, &members);
            for (i, member) in [dependent.clone(), indirect.clone()].iter().enumerate() {
                let roots = first.map(|index| {
                    let content = policy_trace(&scene, &policy, 1, &p, &d, index, &values[index]);
                    let direct = dependent.trace(0, &[content]);
                    vec![if i == 0 {
                        direct
                    } else {
                        indirect.trace(0, &[direct])
                    }]
                });
                boolean(
                    &mut c,
                    &format!("{}/dates={ds:?}/reverse={reverse}", member.id),
                    &format!("guideline_property(actual,p,{},pos)", member.id),
                    roots.as_deref(),
                );
            }
            let installed_index = members.iter().position(|m| m.id == "control").unwrap();
            assert_eq!(
                std::fs::read(c.dir.join(format!("d{installed_index}.pl"))).unwrap(),
                independent_bytes.as_bytes()
            );
            boolean(
                &mut c,
                &format!("independent/dates={ds:?}/reverse={reverse}"),
                "guideline_property(actual,p,independent,pos)",
                Some(&[control.trace(0, &[])]),
            );
        }
    }
    c.finish();
}
#[test]
fn p4_independent_control() {
    let mut c = Cases::new("P4-independent-characterization");
    let scene = recommendation("ordinary", &["should"], None, None, false);
    let control = scene.member();
    // Only the unchanged baseline adapts its declaration block, like a frozen suite.
    // Actual date-fact independence remains unconditional in p4_transitive.
    let legacy = legacy_binary(&c);
    let adapt = |m: &Member| {
        let mut m = m.clone();
        if legacy {
            m.pl = legacy_block(&m.pl);
        }
        m
    };
    for present in [false, true] {
        let p = patient(present);
        for reverse in [false, true] {
            let mut members = vec![adapt(&control), adapt(&p)];
            if reverse {
                members.reverse();
            }
            install(&c, &members);
            let roots =
                present.then(|| vec![control.trace(0, &[p.trace(0, &[]), p.trace(1, &[])])]);
            boolean(
                &mut c,
                &format!("patient={present}/reverse={reverse}"),
                &policy_goal(&scene, 0),
                roots.as_deref(),
            );
        }
    }
    c.finish();
}
fn rational(n: i64, d: i64) -> String {
    let mut a = n.unsigned_abs();
    let mut b = d as u64;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    let gcd = i64::try_from(a).unwrap();
    if d / gcd == 1 {
        (n / gcd).to_string()
    } else {
        format!("rational({},{})", n / gcd, d / gcd)
    }
}
#[test]
fn p4_numeric_limits() {
    let mut c = Cases::new("P4-numeric-limits");
    for effective in [101, DATE, 20230101, 99991231] {
        let scene = recommendation("policy", &["should"], None, Some(effective), false);
        let policy = scene.member();
        let p = patient(true);
        let mut cases = vec![
            ("date(2022,12,28)".to_owned(), false),
            ("not_a_number".into(), false),
            ("[]".into(), false),
            ("0".into(), 0 < effective),
            ("-1".into(), -1 < effective),
            ("20220230".into(), 20220230 < effective),
        ];
        for denominator in [1, 2, 3, 5] {
            for delta in [-denominator, -1, 0, 1, denominator] {
                let numerator = effective * denominator + delta;
                cases.push((
                    rational(numerator, denominator),
                    numerator < effective * denominator,
                ));
            }
        }
        for (value, holds) in cases {
            let d = dates(std::slice::from_ref(&value));
            install(&c, &[policy.clone(), p.clone(), d.clone()]);
            let roots = holds.then(|| vec![policy_trace(&scene, &policy, 0, &p, &d, 0, &value)]);
            boolean(
                &mut c,
                &format!("effective={effective}/value={value}"),
                &policy_goal(&scene, 0),
                roots.as_deref(),
            );
        }
    }
    c.finish();
}
fn caller_continuation(c: &mut Cases, old_carrier: bool) {
    let legacy = old_carrier && legacy_binary(c);
    let adapt = |mut member: Member| {
        if legacy {
            member.pl = legacy_block(&member.pl);
        }
        if old_carrier {
            member.pl = member
                .pl
                .split_inclusive('\n')
                .map(|line| {
                    if line.starts_with(":-") {
                        line.to_owned()
                    } else {
                        line.replace("guideline_evaluation_date", "guideline_exclusion")
                    }
                })
                .collect();
            for (_, clause) in &mut member.clauses {
                *clause = clause.replace("guideline_evaluation_date", "guideline_exclusion");
            }
        }
        member
    };
    let p = adapt(patient(true));
    let values = vec![(DATE - 1).to_string()];
    let d = adapt(dates(&values));
    let alternatives = adapt(Member::raw(
        "alternatives",
        &[
            "guideline_operator(actual,first,should)".into(),
            "guideline_operator(actual,second,should)".into(),
            "guideline_event(second,e,bind)".into(),
            "guideline_event(first,e,bind)".into(),
        ],
    ));
    let goal = conjunction(&[
        "guideline_operator(actual,A,should)".into(),
        "guideline_event(actual,call,probe)".into(),
        "guideline_event(A,B,bind)".into(),
    ]);
    for guarded in [false, true] {
        let mut body = entity("actual", "p", "patient");
        if guarded {
            body.extend(date_goals("Z", DATE));
        }
        let callee = adapt(Member::raw(
            "callee",
            &[
                format!("guideline_event(actual,call,probe) :- {}", body.join(", ")),
                format!(
                    "guideline_operator(actual,retired,may) :- {}",
                    body.join(", ")
                ),
            ],
        ));
        install(
            c,
            &[alternatives.clone(), callee.clone(), p.clone(), d.clone()],
        );
        let mut children = vec![p.trace(0, &[]), p.trace(1, &[])];
        if guarded {
            children.push(d.trace(0, &[]));
            children.push(format!(
                "builtin(guideline_satisfies(exactly,{},less,{DATE}))",
                DATE - 1
            ));
        }
        // The unbound comparison cuts ESO's look-ahead before the later binder.
        let (edge, binder) = if guarded { (0, 3) } else { (1, 2) };
        let roots = [
            alternatives.trace(edge, &[]),
            callee.trace(0, &children),
            alternatives.trace(binder, &[]),
        ];
        boolean(
            c,
            &format!("guarded={guarded}/same-answer-different-first-proof"),
            &goal,
            Some(&roots),
        );
    }
}
#[test]
fn p4_caller_continuation() {
    let mut c = Cases::new("P4-caller-continuation");
    caller_continuation(&mut c, false);
    c.finish();
}
#[test]
fn p4_caller_model_characterization() {
    let mut c = Cases::new("P4-caller-model-characterization");
    // The solver is unchanged (D9); an existing unary semantic carrier isolates
    // the stack/barrier oracle from the new codec's seed refusal.
    caller_continuation(&mut c, true);
    c.finish();
}
#[test]
fn p4_recursion_check() {
    let mut c = Cases::new("P4-recursion");
    let members: Vec<_> = goldens().iter().map(Scene::member).collect();
    for reverse in [false, true] {
        let mut ordered = members.clone();
        if reverse {
            ordered.reverse();
        }
        install(&c, &ordered);
        c.expect(
            &format!("all/reverse={reverse}"),
            &["v1", "recursion-check", "m.tsv"],
            Out::ok(format!(
                "ace_to_pl recursion ok {} documents {} rule clauses\n",
                ordered.len(),
                rule_count(&ordered)
            )),
        );
    }
    for member in members {
        install(&c, std::slice::from_ref(&member));
        c.expect(
            &member.id,
            &["v1", "recursion-check", "m.tsv"],
            Out::ok(format!(
                "ace_to_pl recursion ok 1 documents {} rule clauses\n",
                rule_count(std::slice::from_ref(&member))
            )),
        );
    }
    c.finish();
}
