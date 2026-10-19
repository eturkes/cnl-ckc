use super::common::*;

fn attributed_head(id: &str, noun: bool) -> String {
    let r = gid("product", id, 1, "[]");
    if noun {
        format!("guideline_entity(actual,{r},result,countable)")
    } else {
        format!("guideline_event(actual,{r},wait)")
    }
}
fn rule_count(members: &[Member]) -> usize {
    members
        .iter()
        .flat_map(|m| &m.clauses)
        .filter(|s| s.contains(" :- "))
        .count()
}
fn checked(c: &mut Cases, label: &str, members: &[Member], why: Option<&str>) {
    install(c, members);
    c.expect(
        label,
        &["v1", "recursion-check", "m.tsv"],
        why.map_or_else(
            || {
                Out::ok(format!(
                    "ace_to_pl recursion ok {} documents {} rule clauses\n",
                    members.len(),
                    rule_count(members)
                ))
            },
            |why| Out::err(1, format!("ace_to_pl_error(proof,{why}).\n")),
        ),
    );
}
fn body_positions(goal: &str) -> Vec<String> {
    let safe = "guideline_property(actual,p,ready,pos)";
    vec![
        goal.into(),
        format!("{safe}, {goal}"),
        format!("{goal}, {safe}"),
        format!("{safe}, {goal}, {safe}"),
        format!("\\+ {goal}"),
        format!("\\+ ({goal}, {safe})"),
        format!("\\+ ({safe}, {goal})"),
        format!("{safe}, \\+ ({safe}, {goal}, {safe}), {safe}"),
    ]
}

#[test]
fn b4_guard_scan_finds_each_deontic_and_variable_operator_at_every_body_position() {
    let mut c = Cases::new("B4-guard-domain");
    let record = Member::raw("exclusion", vec![exclusion("alpha")]);
    for op in ["should", "must", "may", "A"] {
        let goal = format!("guideline_operator(actual,ctx,{op})");
        for (i, body) in body_positions(&goal).iter().enumerate() {
            let clause = canonical(&format!(
                "{} :- {body}",
                attributed_head("condition", false)
            ));
            let condition = Member::raw("condition", vec![clause]);
            c.check(
                &format!("op={op}/position={i}/canonical"),
                &document("condition", &condition.clauses),
            );
            checked(
                &mut c,
                &format!("op={op}/position={i}/no-exclusion"),
                std::slice::from_ref(&condition),
                None,
            );
            for reverse in [false, true] {
                let mut members = vec![condition.clone(), record.clone()];
                if reverse {
                    members.reverse();
                }
                checked(
                    &mut c,
                    &format!("op={op}/position={i}/reverse={reverse}"),
                    &members,
                    Some("scope_condition(sentence(condition,1))"),
                );
            }
        }
    }
    c.finish();
}

#[test]
fn b4_guard_scan_ignores_nondeontic_operators_and_nested_data_terms() {
    let mut c = Cases::new("B4-negative-controls");
    let record = Member::raw("exclusion", vec![exclusion("alpha")]);
    let mut goals: Vec<_> = [
        "can",
        "-",
        "other",
        "'Should'",
        "0",
        "[]",
        "should(x)",
        "f(A)",
    ]
    .iter()
    .map(|op| format!("guideline_operator(actual,ctx,{op})"))
    .collect();
    goals.extend([
        "guideline_event(actual,e,guideline_operator(actual,ctx,should))".into(),
        "guideline_property(actual,p,','(guideline_operator(actual,ctx,must),guideline_operator(actual,ctx,A)),pos)".into(),
    ]);
    for (i, goal) in goals.iter().enumerate() {
        for (at, body) in body_positions(goal).iter().enumerate() {
            let condition = Member::raw(
                "condition",
                vec![format!("{} :- {body}", attributed_head("condition", false))],
            );
            checked(
                &mut c,
                &format!("goal={i}/position={at}"),
                &[condition, record.clone()],
                None,
            );
        }
    }
    c.finish();
}

#[test]
fn b4_only_an_exclusion_head_activates_the_scan() {
    let mut c = Cases::new("B4-activation");
    let condition = Member::raw(
        "condition",
        vec![format!(
            "{} :- guideline_operator(actual,ctx,should)",
            attributed_head("condition", false)
        )],
    );
    for (label, clauses, active) in [
        ("fact", vec![exclusion("alpha")], true),
        ("unmatched-key", vec![exclusion("other-key")], true),
        ("variable-key", vec!["guideline_exclusion(A)".into()], true),
        (
            "conditional",
            vec![format!(
                "{} :- guideline_event(actual,e,absent)",
                exclusion("alpha")
            )],
            true,
        ),
        (
            "positive-body-only",
            vec![format!(
                "{} :- {}",
                attributed_head("carrier", false),
                exclusion("alpha")
            )],
            false,
        ),
        (
            "naf-body-only",
            vec![format!(
                "{} :- \\+ {}",
                attributed_head("carrier", false),
                exclusion("alpha")
            )],
            false,
        ),
        (
            "nested-head-data",
            vec![format!(
                "guideline_property(actual,{},guideline_exclusion(alpha),pos)",
                gid("product", "carrier", 1, "[]")
            )],
            false,
        ),
    ] {
        let carrier = Member::raw("carrier", clauses);
        checked(
            &mut c,
            label,
            &[condition.clone(), carrier],
            active.then_some("scope_condition(sentence(condition,1))"),
        );
    }
    c.finish();
}

#[test]
fn b4_first_database_clause_wins_and_left_recursion_keeps_precedence() {
    let mut c = Cases::new("B4-order");
    let event = Member::raw(
        "first-event",
        vec![format!(
            "{} :- guideline_operator(actual,ctx,may)",
            attributed_head("first-event", false)
        )],
    );
    let entity = Member::raw(
        "second-entity",
        vec![format!(
            "{} :- \\+ guideline_operator(actual,ctx,must)",
            attributed_head("second-entity", true)
        )],
    );
    let record = Member::raw("exclusion", vec![exclusion("alpha")]);
    for reverse in [false, true] {
        let mut members = vec![event.clone(), entity.clone(), record.clone()];
        if reverse {
            members.reverse();
        }
        let first = if reverse {
            "second-entity"
        } else {
            "first-event"
        };
        checked(
            &mut c,
            &format!("reverse={reverse}"),
            &members,
            Some(&format!("scope_condition(sentence({},1))", atom(first))),
        );
        let head = attributed_head("loop", false);
        members.push(Member::raw("loop", vec![format!("{head} :- {head}")]));
        checked(
            &mut c,
            &format!("left-recursive/reverse={reverse}"),
            &members,
            Some("left_recursive(sentence(loop,1),guideline_event,3)"),
        );
    }
    let raw = Member::raw(
        "unattributed",
        vec!["guideline_event(actual,e,wait) :- guideline_operator(actual,ctx,should)".into()],
    );
    checked(
        &mut c,
        "site without a guideline identity",
        &[raw, record],
        Some("scope_condition(unattributed)"),
    );
    c.finish();
}
