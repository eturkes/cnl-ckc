use super::common::*;
use super::model::*;

fn run(
    c: &mut Cases,
    label: &str,
    scene: &Scene,
    pl: &str,
    table: Option<&str>,
    why: Option<&str>,
) {
    c.write("ace.txt", &scene.ace);
    c.write("dump.txt", &scene.dump());
    c.write("cert.pl", pl);
    let mut args = vec![
        "certify-one",
        "doc",
        &scene.id,
        "ace.txt",
        "-",
        "dump.txt",
        "cert.pl",
    ];
    if let Some(table) = table {
        c.write("vocabulary.tsv", table);
        args.push("vocabulary.tsv");
    }
    c.expect(
        label,
        &args,
        why.map_or_else(
            || Out::ok(scene.member().payload),
            |why| Out::err(1, format!("ckc: certify: {}: {why}.\n", scene.id)),
        ),
    );
}
fn clean(c: &mut Cases, label: &str, scene: &Scene) {
    if std::env::var_os("CKC_CS1_KEEP_CASES").is_some() {
        for (suffix, text) in [
            ("ace.in", scene.ace.clone()),
            ("dump.in", scene.dump()),
            ("pl.in", scene.pl()),
            ("proof.in", scene.member().payload),
        ] {
            c.write(&format!("{}.{}", scene.id, suffix), &text);
        }
    }
    c.check(&format!("{label}/canonical"), &scene.pl());
    run(c, label, scene, &scene.pl(), scene.table().as_deref(), None);
}
fn parts(text: &str) -> Vec<String> {
    let (mut depth, mut quoted, mut escaped, mut start) = (0, false, false, 0);
    let mut out = Vec::new();
    for (i, ch) in text.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '\'' {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        match ch {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                out.push(text[start..i].trim().into());
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(text[start..].trim().into());
    out
}
fn erase_scope(clauses: &[String]) -> Vec<String> {
    clauses
        .iter()
        .filter_map(|clause| {
            let (head, body) = clause.split_once(" :- ").unwrap();
            if head.starts_with("guideline_exclusion(") {
                return None;
            }
            let goals: Vec<_> = parts(body)
                .into_iter()
                .filter(|g| !g.starts_with("\\+ guideline_exclusion("))
                .collect();
            Some(format!("{head} :- {}", goals.join(", ")))
        })
        .collect()
}
fn mutant(c: &mut Cases, label: &str, scene: &Scene, clauses: Vec<String>) {
    let original = scene.member().clauses;
    assert_ne!(clauses, original, "{label}: mutant changes bytes");
    assert_eq!(
        erase_scope(&clauses),
        erase_scope(&original),
        "{label}: primitive projection unchanged"
    );
    let pl = document_text(&scene.id, &scene.ace, scene.table().as_deref(), &clauses);
    c.check(&format!("{label}/canonical"), &pl);
    run(
        c,
        label,
        scene,
        &pl,
        scene.table().as_deref(),
        Some("clauses(1)"),
    );
}
fn forms() -> [ExclusionForm; 8] {
    [
        ExclusionForm::Bare,
        ExclusionForm::OnePp,
        ExclusionForm::TwoPp,
        ExclusionForm::Curried,
        ExclusionForm::Split,
        ExclusionForm::Exception,
        ExclusionForm::Comparison,
        ExclusionForm::TimePp,
    ]
}
fn recommendations() -> Vec<Scene> {
    let mut scenes = Vec::new();
    for (i, op) in ["should", "must", "may"].iter().enumerate() {
        for nested in [false, true] {
            scenes.push(recommendation(
                &format!("recommend-{i}-{}", usize::from(nested)),
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
    scenes.push(recommendation(
        "multi-edge",
        Some("alpha"),
        &["should", "must", "may"],
        false,
    ));
    scenes
}

#[test]
fn b6_certifier_accepts_all_exclusion_shapes_and_each_deontic_edge_depth() {
    let mut c = Cases::new("B6-goldens");
    for (i, form) in forms().iter().enumerate() {
        let scene = exclusion_scene(&format!("exclude-{i}"), "alpha", *form);
        clean(&mut c, &format!("{form:?}"), &scene);
    }
    for scene in recommendations() {
        clean(&mut c, &scene.id, &scene);
    }
    for ops in [vec![], vec!["can"]] {
        let scene = recommendation(
            if ops.is_empty() { "plain" } else { "possible" },
            Some("alpha"),
            &ops,
            false,
        );
        clean(&mut c, &scene.id, &scene);
    }
    c.finish();
}

#[test]
fn b6_dropped_duplicated_moved_and_rekeyed_guards_reject() {
    let mut c = Cases::new("B6-guards");
    for scene in recommendations() {
        clean(&mut c, &format!("{}/control", scene.id), &scene);
        let original = scene.member().clauses;
        for at in 0..original.len() {
            let (head, body) = original[at].split_once(" :- ").unwrap();
            let goals = parts(body);
            if goals.last().unwrap() != "\\+ guideline_exclusion(alpha)" {
                continue;
            }
            for action in [
                "drop",
                "duplicate",
                "before-last",
                "first",
                "key",
                "variable-key",
            ] {
                let mut changed = goals.clone();
                match action {
                    "drop" => {
                        changed.pop();
                    }
                    "duplicate" => {
                        changed.push(changed.last().unwrap().clone());
                    }
                    "before-last" => {
                        let guard = changed.pop().unwrap();
                        changed.insert(changed.len() - 1, guard);
                    }
                    "first" => {
                        let guard = changed.pop().unwrap();
                        changed.insert(0, guard);
                    }
                    "key" => {
                        *changed.last_mut().unwrap() = "\\+ guideline_exclusion(beta)".into();
                    }
                    "variable-key" => {
                        *changed.last_mut().unwrap() = "\\+ guideline_exclusion(A)".into();
                    }
                    _ => unreachable!(),
                }
                let mut clauses = original.clone();
                clauses[at] = canonical(&format!("{head} :- {}", changed.join(", ")));
                mutant(
                    &mut c,
                    &format!("{}/clause={at}/{action}", scene.id),
                    &scene,
                    clauses,
                );
            }
        }
    }
    c.finish();
}

#[test]
fn b6_added_guards_on_payload_can_negative_or_asserted_clauses_reject() {
    let mut c = Cases::new("B6-payload-guards");
    let mut scenes = recommendations();
    scenes.push(recommendation("plain", Some("alpha"), &[], false));
    scenes.push(exclusion_scene("exclusion", "alpha", ExclusionForm::OnePp));
    for scene in scenes {
        let original = scene.member().clauses;
        clean(&mut c, &format!("{}/control", scene.id), &scene);
        for (at, clause) in original.iter().enumerate() {
            if clause.contains("\\+ guideline_exclusion(") {
                continue;
            }
            let mut changed = original.clone();
            changed[at].push_str(", \\+ guideline_exclusion(alpha)");
            mutant(
                &mut c,
                &format!("{}/clause={at}", scene.id),
                &scene,
                changed,
            );
        }
    }
    c.finish();
}

#[test]
fn b6_dropped_duplicated_moved_and_rekeyed_records_reject_per_variant() {
    let mut c = Cases::new("B6-records");
    for (i, form) in forms().iter().enumerate() {
        let scene = exclusion_scene(&format!("exclusion-{i}"), "alpha", *form);
        let original = scene.member().clauses;
        clean(&mut c, &format!("{form:?}/control"), &scene);
        for at in 0..original.len() {
            if !original[at].starts_with("guideline_exclusion(") {
                continue;
            }
            for action in [
                "drop",
                "duplicate",
                "before-last",
                "first",
                "key",
                "variable-key",
                "body-goal-dropped",
            ] {
                let mut changed = original.clone();
                match action {
                    "drop" => {
                        changed.remove(at);
                    }
                    "duplicate" => {
                        changed.insert(at, changed[at].clone());
                    }
                    "before-last" => {
                        let record = changed.remove(at);
                        changed.insert(at - 1, record);
                    }
                    "first" => {
                        let record = changed.remove(at);
                        changed.insert(0, record);
                    }
                    "key" => {
                        changed[at] = changed[at].replacen(
                            "guideline_exclusion(alpha)",
                            "guideline_exclusion(beta)",
                            1,
                        );
                    }
                    "variable-key" => {
                        changed[at] = canonical(&changed[at].replacen(
                            "guideline_exclusion(alpha)",
                            "guideline_exclusion(A)",
                            1,
                        ));
                    }
                    "body-goal-dropped" => {
                        let (head, body) = changed[at].split_once(" :- ").unwrap();
                        let mut goals = parts(body);
                        goals.pop();
                        assert!(!goals.is_empty());
                        changed[at] = canonical(&format!("{head} :- {}", goals.join(", ")));
                    }
                    _ => unreachable!(),
                }
                mutant(
                    &mut c,
                    &format!("{form:?}/clause={at}/{action}"),
                    &scene,
                    changed,
                );
            }
        }
    }
    c.finish();
}

#[test]
fn b6_record_on_a_nonexclusion_rule_and_scope_digest_mutants_reject() {
    let mut c = Cases::new("B6-record-custody");
    for ops in [vec![], vec!["should"], vec!["can"]] {
        let scene = recommendation("ordinary", Some("alpha"), &ops, false);
        clean(&mut c, "ordinary/control", &scene);
        let mut clauses = scene.member().clauses;
        clauses.push(format!(
            "{} :- {}",
            exclusion("alpha"),
            scene.groups[0].body.join(", ")
        ));
        mutant(&mut c, &format!("nonexclusion/{ops:?}"), &scene, clauses);
    }
    for scene in [
        recommendation("policy", Some("alpha"), &["should"], false),
        exclusion_scene("exclusion", "alpha", ExclusionForm::OnePp),
    ] {
        let old = scene.table().unwrap();
        let new = old.replace("\talpha\n", "\tbeta\n");
        let pl = scene.pl();
        run(
            &mut c,
            &format!("{}/stale-table-digest", scene.id),
            &scene,
            &pl,
            Some(&new),
            Some("vocabulary"),
        );
        let repinned = pl.replacen(
            &format!("vocabulary({})", digest(Some(&old))),
            &format!("vocabulary({})", digest(Some(&new))),
            1,
        );
        c.check("repinned table, stale guard or record", &repinned);
        run(
            &mut c,
            &format!("{}/rekeyed-table-stale-projection", scene.id),
            &scene,
            &repinned,
            Some(&new),
            Some("clauses(1)"),
        );
        run(
            &mut c,
            &format!("{}/missing-table", scene.id),
            &scene,
            &pl,
            None,
            Some("vocabulary"),
        );
    }
    c.finish();
}

#[test]
fn b6_scoped_question_cannot_gain_a_record_or_guard() {
    let mut c = Cases::new("B6-question-projection");
    let ace = "Is there a patient?\n";
    let dump = "sentences([[^,'Is',there,a,patient,?]]).\ndrs(drs([],[question(drs([A],[-(object(A,patient,countable,na,eq,1),/(1,4))]))])).\nmessages([]).\n";
    let table = fixture("vocabulary.tsv.in");
    let goals = entity("actual", "A", "patient");
    c.write("ace.txt", ace);
    c.write("dump.txt", dump);
    c.write("vocabulary.tsv", &table);
    for mutation in ["control", "record-first", "record-last", "guard-last"] {
        let mut changed = goals.clone();
        match mutation {
            "control" => {}
            "record-first" => changed.insert(0, exclusion("alpha")),
            "record-last" => changed.push(exclusion("alpha")),
            "guard-last" => changed.push("\\+(guideline_exclusion(alpha))".into()),
            _ => unreachable!(),
        }
        let pl = query_text("question", ace, Some(&table), &conjunction(&changed), "[]");
        c.check(mutation, &pl);
        c.write("cert.pl", &pl);
        c.expect(
            mutation,
            &[
                "certify-one",
                "query",
                "question",
                "ace.txt",
                "-",
                "dump.txt",
                "cert.pl",
                "vocabulary.tsv",
            ],
            if mutation == "control" {
                Out::ok("ckc: certify ok question\n")
            } else {
                Out::err(1, "ckc: certify: question: projection.\n")
            },
        );
    }
    c.finish();
}
