use super::compiler::Compiler;
use super::model::{HEADER, quote};
use super::support::*;
use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const PATH: &str = "guidelines/cdc-2022-opioid/vocabulary.tsv";
fn scope(noun: &str, key: &str) -> String {
    format!("scope\t{noun}\t{key}")
}
fn exclusion(verb: &str) -> String {
    format!("exclusion\t{verb}\tscope")
}
fn bytes(rows: &[String]) -> String {
    format!("{HEADER}{}\n", rows.join("\n"))
}
fn lemma(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b > 32 && b != 127)
}
fn key(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('-')
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}
// Scope/exclusion grammar + its named cross-kind noun boundaries; independent row oracle.
fn verdict(text: &str, gid: Option<&str>) -> Result<usize, String> {
    let Some(body) = text.strip_prefix(HEADER) else {
        return Err("header".into());
    };
    if body.is_empty() {
        return Err("no rows".into());
    }
    if !body.ends_with('\n') {
        return Err("final newline".into());
    }
    let rows: Vec<_> = body.strip_suffix('\n').unwrap().split('\n').collect();
    let (mut sc, mut ex) = (None, None);
    let mut nouns = BTreeSet::new();
    for (i, row) in rows.iter().enumerate() {
        let n = i + 1;
        let f: Vec<_> = row.split('\t').collect();
        let why = if row.starts_with('#') {
            Some("comment")
        } else if f.len() != 3 {
            Some("field count")
        } else if !lemma(f[1]) {
            Some("lemma")
        } else {
            match f[0] {
                "scope" => {
                    if !key(f[2]) {
                        Some("scope value")
                    } else if sc.is_some() {
                        Some("duplicate scope")
                    } else if !nouns.insert(f[1]) {
                        Some("duplicate noun")
                    } else {
                        sc = Some((n, f[2]));
                        None
                    }
                }
                "exclusion" => {
                    if f[2] != "scope" {
                        Some("exclusion value")
                    } else if ex.is_some() {
                        Some("duplicate exclusion")
                    } else {
                        ex = Some(n);
                        None
                    }
                }
                "unit" => {
                    if !["second", "minute", "hour", "day", "week", "month", "year"].contains(&f[2])
                    {
                        Some("unit id")
                    } else if !nouns.insert(f[1]) {
                        Some("duplicate noun")
                    } else {
                        None
                    }
                }
                "spacing" | "window" => {
                    if !lemma(f[2]) {
                        Some("frame lemma")
                    } else if !nouns.insert(f[2]) {
                        Some("duplicate noun")
                    } else {
                        None
                    }
                }
                "strength" | "evidence" => {
                    let valid = if f[0] == "strength" {
                        ["strong", "conditional"].contains(&f[2])
                    } else {
                        ["high", "moderate", "low", "very_low"].contains(&f[2])
                    };
                    if !valid {
                        Some(if f[0] == "strength" {
                            "strength value"
                        } else {
                            "evidence value"
                        })
                    } else if !nouns.insert(f[1]) {
                        Some("duplicate noun")
                    } else {
                        None
                    }
                }
                "relation" | "qualifier" | "preference" | "alternative" | "frequency" | "range"
                | "approximation" => None,
                _ => Some("kind"),
            }
        };
        if let Some(why) = why {
            return Err(format!("{why} at row {n}"));
        }
    }
    match (sc, ex) {
        (Some((n, _)), None) => Err(format!("scope without exclusion at row {n}")),
        (None, Some(n)) => Err(format!("exclusion without scope at row {n}")),
        (Some((n, k)), _) if gid.is_some_and(|g| g != k) => Err(format!("scope key at row {n}")),
        _ => Ok(rows.len()),
    }
}
#[derive(Clone)]
struct Plant {
    id: String,
    text: String,
}
fn plants() -> Vec<Plant> {
    let mut out = Vec::new();
    let mut add = |id: &str, rows: Vec<String>| {
        out.push(Plant {
            id: id.into(),
            text: bytes(&rows),
        })
    };
    add(
        "pair",
        vec![scope("recommendation", "alpha"), exclusion("apply")],
    );
    add(
        "pair-reverse",
        vec![exclusion("apply"), scope("recommendation", "alpha")],
    );
    for (i, k) in ["", "Alpha", "-alpha", "al pha", "alpha_", "é", "a/alpha"]
        .iter()
        .enumerate()
    {
        add(
            &format!("bad-key-{i}"),
            vec![scope("recommendation", k), exclusion("apply")],
        );
        add(
            &format!("value-before-duplicate-{i}"),
            vec![
                scope("recommendation", "alpha"),
                scope("recommendation", k),
                exclusion("apply"),
            ],
        );
    }
    for val in ["", "Scope", "other", "scope value"] {
        add(
            &format!("exclusion-value-{val}"),
            vec![
                scope("recommendation", "alpha"),
                format!("exclusion\tapply\t{val}"),
            ],
        );
        add(
            &format!("exclusion-value-before-duplicate-{val}"),
            vec![
                exclusion("apply"),
                format!("exclusion\tapply\t{val}"),
                scope("recommendation", "alpha"),
            ],
        );
    }
    for second in ["recommendation", "other"] {
        add(
            &format!("duplicate-scope-{second}"),
            vec![
                scope("recommendation", "alpha"),
                scope(second, "alpha"),
                exclusion("apply"),
            ],
        );
        add(
            &format!("duplicate-exclusion-{second}"),
            vec![
                scope("recommendation", "alpha"),
                exclusion("apply"),
                exclusion(second),
            ],
        );
    }
    for (i, typed) in [
        "unit\trecommendation\thour",
        "spacing\tat\trecommendation",
        "window\tduring\trecommendation",
        "strength\trecommendation\tstrong",
        "evidence\trecommendation\thigh",
    ]
    .iter()
    .enumerate()
    {
        for reverse in [false, true] {
            let mut rows = vec![scope("recommendation", "alpha"), typed.to_string()];
            if reverse {
                rows.reverse();
            }
            rows.push(exclusion("apply"));
            add(&format!("noun-{i}-{reverse}"), rows);
        }
    }
    for (kind, row) in [
        ("scope", scope("recommendation", "alpha")),
        ("exclusion", exclusion("apply")),
    ] {
        add(
            &format!("pair-missing-{kind}"),
            vec!["unit\thour\thour".into(), row.clone()],
        );
        add(
            &format!("late-row-before-pair-{kind}"),
            vec![row, "BAD\tx\tx".into()],
        );
    }
    for (i, row) in [
        "#scope\t\t\textra",
        "scope\t",
        "scope\tbad lemma\tINVALID",
        "BAD\t\tINVALID",
        "BAD\tx\tINVALID",
        "scope\t\talpha",
        "scope\trecommendation\talpha\textra",
    ]
    .iter()
    .enumerate()
    {
        add(&format!("precedence-{i}"), vec![row.to_string()]);
    }
    for alias in ["x", "hyphen-word", "é", "Ω", "a&<b>", "'", "x\\y", "#lemma"] {
        for reverse in [false, true] {
            let mut rows = vec![scope(alias, "alpha"), exclusion(alias)];
            if reverse {
                rows.reverse();
            }
            add(&format!("alias-{alias}-{reverse}"), rows);
        }
    }
    let paired = bytes(&[scope("recommendation", "alpha"), exclusion("apply")]);
    for (id, text) in [
        ("header", paired.strip_prefix(HEADER).unwrap().to_string()),
        ("no-rows", HEADER.to_string()),
        (
            "no-final-newline",
            paired.strip_suffix('\n').unwrap().to_string(),
        ),
        ("blank-last-row", format!("{paired}\n")),
        (
            "blank-first-row",
            paired.replacen(HEADER, &format!("{HEADER}\n"), 1),
        ),
    ] {
        out.push(Plant {
            id: id.into(),
            text,
        });
    }
    out
}
fn ui_run(tree: &Path, cwd: &Path, text: &str) -> Run {
    write(&tree.join("guidelines/alpha/vocabulary.tsv"), text);
    native(cwd, &["ui", "check", tree.to_str().unwrap()])
}
fn judge_ui(checks: &mut Checks, id: &str, text: &str, got: &Run) {
    match verdict(text, Some("alpha")) {
        Ok(_) => checks.require(
            got.ok(),
            format!("{id} expected accepted table; {}", got.describe()),
        ),
        Err(why) => checks.require(
            got.rc == Some(1)
                && got.stderr == format!("ui: viewmodel: alpha vocabulary.tsv: {why}\n"),
            format!("{id} expected {why}; {}", got.describe()),
        ),
    }
}
#[test]
fn a1_table_grammar_boundaries_and_order() {
    let scratch = Scratch::new("table-ui");
    let tree = scratch.0.join("tree");
    copy_tree(&root().join("tests/ui/green/basic/tree"), &tree);
    let mut checks = Checks::default();
    for p in plants() {
        let got = ui_run(&tree, &scratch.0, &p.text);
        judge_ui(&mut checks, &p.id, &p.text, &got);
    }
    checks.finish("A1 row grammar + precedence");
}
#[test]
fn a1_key_language_generated_property() {
    let scratch = Scratch::new("key-property");
    let tree = scratch.0.join("tree");
    copy_tree(&root().join("tests/ui/green/basic/tree"), &tree);
    let mut checks = Checks::default();
    let alphabet = ["a", "0", "-", "A", "_", " ", "é"];
    let mut words = vec![String::new()];
    let mut layer = vec![String::new()];
    for _ in 0..3 {
        layer = layer
            .iter()
            .flat_map(|p| alphabet.iter().map(move |c| format!("{p}{c}")))
            .collect();
        words.extend(layer.clone());
    }
    words.extend([
        "alpha".into(),
        "a-".into(),
        "a--b".into(),
        "9a".into(),
        "z".repeat(256),
    ]);
    assert_eq!(words.len(), 405);
    for (i, k) in words.iter().enumerate() {
        let text = bytes(&[scope("recommendation", k), exclusion("apply")]);
        let got = ui_run(&tree, &scratch.0, &text);
        judge_ui(&mut checks, &format!("key-{i} {k:?}"), &text, &got);
    }
    checks.finish("A1 key language 405 generated inputs");
}
#[test]
fn a2_check_scope_key_path_and_row_meter() {
    let scratch = Scratch::new("check-key");
    let head = git(&root(), &["rev-parse", "HEAD"]);
    git(
        &scratch.0,
        &[
            "clone",
            "-q",
            "--shared",
            "--no-checkout",
            root().to_str().unwrap(),
            "tree",
        ],
    );
    let tree = scratch.0.join("tree");
    git(&tree, &["checkout", "-q", "--detach", head.trim()]);
    let stop = scratch.0.join("swipl-stop");
    write(
        &stop,
        "#!/bin/sh\n[ \"$1\" = --version ] && { echo 'SWI-Prolog version 10.0.2 for x86_64-linux'; exit 0; }\necho 'cs1-surface: table stage passed' >&2\nexit 3\n",
    );
    fs::set_permissions(&stop, fs::Permissions::from_mode(0o755)).unwrap();
    let mut checks = Checks::default();
    for (id, rows) in [
        (
            "matching",
            vec![
                scope("recommendation", "cdc-2022-opioid"),
                exclusion("apply"),
            ],
        ),
        (
            "matching-reverse",
            vec![
                exclusion("apply"),
                "unit\thour\thour".into(),
                scope("recommendation", "cdc-2022-opioid"),
            ],
        ),
        (
            "mismatch",
            vec![scope("recommendation", "other"), exclusion("apply")],
        ),
        (
            "mismatch-row-three",
            vec![
                "unit\thour\thour".into(),
                exclusion("apply"),
                scope("recommendation", "other"),
            ],
        ),
        (
            "grammar-before-key",
            vec![
                scope("recommendation", "other"),
                "exclusion\tapply\tINVALID".into(),
            ],
        ),
    ] {
        let text = bytes(&rows);
        write(&tree.join(PATH), &text);
        let got = command_env(&program(), &tree, &["check"], "", &[("SWIPL", &stop)]);
        match verdict(&text, Some("cdc-2022-opioid")) {
            Ok(n) => checks.require(
                got.stdout
                    .contains(&format!("ckc: vocabulary ok {PATH} {n} rows\n")),
                format!("{id} meter={n}: {}", got.describe()),
            ),
            Err(why) => checks.require(
                got.rc == Some(1) && got.violation() == format!("ckc: vocabulary: {PATH}: {why}"),
                format!("{id} expected {why}; {}", got.describe()),
            ),
        }
    }
    checks.finish("A2 HEAD-clone path key + pair row count");
}
#[test]
fn a2_ui_key_law_matching_and_mismatch_envelope() {
    let scratch = Scratch::new("ui-key");
    let tree = scratch.0.join("tree");
    copy_tree(&root().join("tests/ui/green/basic/tree"), &tree);
    let mut checks = Checks::default();
    for (id, rows) in [
        (
            "valid",
            vec![scope("recommendation", "alpha"), exclusion("apply")],
        ),
        (
            "bad",
            vec![scope("recommendation", "other"), exclusion("apply")],
        ),
        (
            "bad-row-three",
            vec![
                "unit\thour\thour".into(),
                exclusion("apply"),
                scope("recommendation", "other"),
            ],
        ),
    ] {
        let text = bytes(&rows);
        let got = ui_run(&tree, &scratch.0, &text);
        judge_ui(&mut checks, id, &text, &got);
    }
    checks.finish("A2 UI logical path + existing envelope");
}
#[test]
fn a3_fork_vocabulary_load_row_laws_and_fixture_keys() {
    let scratch = Scratch::new("table-fork");
    let compiler = Compiler::new(&scratch.0);
    let mut checks = Checks::default();
    let mut cases = plants();
    for k in ["0", "a", "a-", "9a", "a--b", "fixture-key"] {
        cases.push(Plant {
            id: format!("fixture-key-{k}"),
            text: bytes(&[scope("recommendation", k), exclusion("apply")]),
        });
    }
    for p in cases {
        let got = compiler.run(
            &scratch.0.join("work"),
            "cs-table",
            "doc",
            "A man sleeps.\n",
            Some(&p.text),
        );
        match verdict(&p.text, None) {
            Ok(_) => {
                checks.require(
                    got.ok(),
                    format!("{} fork fixture table accepted: {}", p.id, got.describe()),
                );
                if got.ok() {
                    checks.require(
                        got.stdout
                            .contains(&format!("vocabulary(sha256({}))", quote(&sha(&p.text)))),
                        format!("{} raw-table custody", p.id),
                    );
                }
            }
            Err(why) => checks.require(
                got.rc == Some(2)
                    && got.stdout.is_empty()
                    && got
                        .stderr
                        .starts_with("ace_to_pl_error(vocabulary_load,error(vocabulary(")
                    && got.stderr.contains("/(read_vocabulary,2)")
                    && got.stderr.lines().count() == 1,
                format!(
                    "{} expected vocabulary_load for {why}; {}",
                    p.id,
                    got.describe()
                ),
            ),
        }
    }
    checks.finish("A3 fork row-law plants + non-corpus key exemption");
}
