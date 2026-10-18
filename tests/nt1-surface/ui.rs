use super::compiler::timed;
use super::model::{self, Goal, Term, a, c, event, lit, obj, v};
use super::support::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

const TIMING: &str = "<h3>Timing as compiled</h3>";
const QUALIFIERS: &str = "<h3>Qualifiers as compiled</h3>";
const PREFERENCES: &str = "<h3>Preferences as compiled</h3>";
struct Fixture {
    scratch: Scratch,
    tree: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let scratch = Scratch::new("ui");
        let tree = scratch.0.join("tree");
        copy_tree(&root().join("tests/s2-ui/tree"), &tree);
        let source = tree.join("guidelines/alpha/pl/d.pl.in");
        fs::rename(&source, source.with_extension("")).unwrap();
        write(&tree.join("guidelines/alpha/vocabulary.tsv"), model::TABLE);
        Self { scratch, tree }
    }
    fn document(&self, bundles: &[String]) -> String {
        let prefix = format!(
            "% d.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document(d,ace_sha256('{}'),ulex(none),vocabulary(none)).\n",
            model::declarations(true),
            "0".repeat(64)
        );
        let body = bundles
            .iter()
            .enumerate()
            .map(|(i, s)| format!("% S{}: A clinician chooses an option.\n{s}", i + 1))
            .collect::<String>();
        let text = format!("{prefix}{body}");
        let g = self.tree.join("guidelines/alpha");
        write(&g.join("pl/d.pl"), &text);
        let derived = native(
            &self.scratch.0,
            &["derive-review-manifest", g.to_str().unwrap()],
        );
        derived.require_ok("UI custody fixture");
        write(&g.join("audit/review-manifest.tsv"), derived.stdout);
        text
    }
    fn request(&self) -> String {
        let run = native(
            &self.scratch.0,
            &[
                "ui",
                "request",
                "GET",
                "/g/alpha/doc/d.html",
                self.tree.to_str().unwrap(),
                "--token",
                "nt1-ui",
            ],
        );
        run.require_ok("UI GET");
        let (head, body) = run.stdout.split_once("\n\n").unwrap();
        assert_eq!(head.lines().next(), Some("HTTP 200"), "{head}");
        body.into()
    }
}
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn row(cells: &[&str]) -> String {
    format!(
        "<tr>{}</tr>",
        cells
            .iter()
            .map(|s| format!("<td>{}</td>", escape(s)))
            .collect::<String>()
    )
}
fn rows(page: &str, heading: &str) -> String {
    let Some((_, section)) = page.split_once(heading) else {
        return String::new();
    };
    let section = section.split_once("</section>").unwrap().0;
    section
        .split_once("<tbody>")
        .unwrap()
        .1
        .split_once("</tbody>")
        .unwrap()
        .0
        .into()
}
fn body(ts: &[Term], excluded: bool) -> String {
    let mut vars = BTreeMap::new();
    let goals = if excluded {
        vec![Goal::Naf(ts.to_vec())]
    } else {
        ts.iter().cloned().map(Goal::Pos).collect()
    };
    format!(
        "guideline_event(actual,result,sleep) :- {}.\n",
        goals
            .iter()
            .map(|g| g.render(&mut vars))
            .collect::<Vec<_>>()
            .join(", ")
    )
}
fn undisclosed(page: &str) -> String {
    let mut s = page.to_owned();
    let start = s
        .find("<details>\n<summary>Compiled Prolog (")
        .expect("disclosure start");
    let end = start + s[start..].find("</details>").unwrap() + "</details>".len();
    s.replace_range(start..end, "");
    let start = s
        .find("<input type=\"hidden\" name=\"review_sha256\"")
        .unwrap();
    let end = start + s[start..].find('\n').unwrap();
    s.replace_range(start..end, "");
    s
}
fn timing_rows() -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut old = Vec::new();
    let mut new = Vec::new();
    let mut expected = Vec::new();
    for shape in [
        "duration",
        "alias",
        "after",
        "before",
        "within",
        "recurrence",
        "window",
        "range",
        "frequency",
    ] {
        for (cmp, word) in [
            ("exactly", "exactly"),
            ("geq", "at least"),
            ("greater", "more than"),
            ("leq", "at most"),
            ("less", "less than"),
        ] {
            if shape == "range" && cmp != "geq" {
                continue;
            }
            let (_, ts, _) = timed(shape, cmp, word);
            let nt = model::annotate(&ts, true, true);
            let quantity = format!(
                "{word} 2 {}",
                if shape == "alias" { "days" } else { "hours" }
            );
            let (action, description, anchor) = match shape {
                "duration" | "alias" => ("wait", format!("lasts {quantity}"), ""),
                "after" | "before" => ("wait", format!("{quantity} {shape}"), "visit"),
                "within" => ("wait", format!("within {quantity} of"), "visit"),
                "recurrence" => ("wait", format!("repeats {quantity} apart"), ""),
                "window" => (
                    "wait",
                    format!("repeats {quantity} apart during less than 7 days from"),
                    "visit",
                ),
                "range" => ("wait", "lasts a minimum of 2 to 7 hours".into(), ""),
                "frequency" => (
                    "read",
                    format!("{word} 2 per 1 day, counted item: book"),
                    "",
                ),
                _ => unreachable!(),
            };
            for excluded in [false, true] {
                old.push(body(&ts, excluded));
                new.push(body(&nt, excluded));
                expected.push(row(&[
                    &old.len().to_string(),
                    if excluded {
                        "excluded condition"
                    } else {
                        "condition"
                    },
                    action,
                    &description,
                    anchor,
                ]));
            }
        }
    }
    (old, new, expected)
}
#[test]
fn p6_timing_bound_joins_all_consumers_both_parts() {
    let fx = Fixture::new();
    let (_, new, expected) = timing_rows();
    let text = fx.document(&new);
    let page = fx.request();
    let mut checks = Checks::default();
    checks.equal(
        &rows(&page, TIMING),
        &expected.join("\n"),
        "82 condition/excluded timing rows",
    );
    checks.require(
        page.contains(&format!(
            "<summary>Compiled Prolog ({} lines)</summary>",
            text.lines().count()
        )),
        "disclosure exact line count",
    );
    checks.require(
        page.contains(&format!("<pre>{}</pre>", escape(&text))),
        "disclosure exact generated bytes",
    );
    checks.require(
        page.matches("<h3>Timing as compiled</h3>").count() == 1,
        "one existing timing table",
    );
    checks.require(
        !page.contains(QUALIFIERS) && !page.contains(PREFERENCES),
        "no new tables",
    );
    checks.finish("P6 timing");
}
#[test]
fn p6_visible_content_is_invariant_under_condition_conversion() {
    let fx = Fixture::new();
    let (old, new, expected) = timing_rows();
    fx.document(&old);
    let before = fx.request();
    fx.document(&new);
    let after = fx.request();
    assert_eq!(
        rows(&before, TIMING),
        expected.join("\n"),
        "nonvacuous unchanged-bound control"
    );
    assert_eq!(
        rows(&after, TIMING),
        expected.join("\n"),
        "nonvacuous converted-bound control"
    );
    assert_eq!(
        undisclosed(&after),
        undisclosed(&before),
        "D11 only disclosure + hidden review digest may change"
    );
}
#[test]
fn p6_qualifier_and_preference_actions_recover_canonical_time_nouns() {
    let fx = Fixture::new();
    let mut bundles = Vec::new();
    let mut expected_q = Vec::new();
    let mut expected_p = Vec::new();
    let mut copula_rows = Vec::new();
    for noun in ["day", "d"] {
        for cmp in ["exactly", "geq", "greater", "leq", "less"] {
            for copula in [false, true] {
                for excluded in [false, true] {
                    let mut ts = obj(1, "treatment", "eq", 1);
                    ts.extend(obj(2, noun, cmp, 2));
                    let action = if copula {
                        ts.push(lit("property", vec![v(3), a("needed"), a("pos")]));
                        ts.extend(event(4, "be", &[2, 3]));
                        "days"
                    } else {
                        ts.extend(event(4, "take", &[1, 2]));
                        "take days"
                    };
                    ts.push(lit("qualifier", vec![v(4), a("usuality"), a("usually")]));
                    ts.push(lit(
                        "entity",
                        vec![a("preferred_option"), a("option"), a("countable")],
                    ));
                    ts.push(lit(
                        "preference",
                        vec![v(4), a("preferred_option"), a("none")],
                    ));
                    let nt = model::annotate(&ts, true, true);
                    bundles.push(body(&nt, excluded));
                    let s = bundles.len().to_string();
                    let part = if excluded {
                        "excluded condition"
                    } else {
                        "condition"
                    };
                    expected_q.push(row(&[
                        &s,
                        part,
                        "as a fact",
                        action,
                        "How often",
                        "usually",
                    ]));
                    expected_p.push(row(&[
                        &s,
                        part,
                        "as a fact",
                        if copula { action } else { "take" },
                        "option",
                        "none named",
                    ]));
                    copula_rows.push(copula);
                }
            }
        }
    }
    fx.document(&bundles);
    let page = fx.request();
    // D11 fixes the copula's unit word, not its surrounding HTML whitespace.
    let action_rows = |heading| {
        rows(&page, heading)
            .split_inclusive('\n')
            .enumerate()
            .map(|(i, row)| {
                if copula_rows.get(i) != Some(&true) {
                    return row.to_owned();
                }
                let start = row.match_indices("<td>").nth(3).unwrap().0 + "<td>".len();
                let end = start + row[start..].find("</td>").unwrap();
                format!(
                    "{}{}{}",
                    &row[..start],
                    row[start..end].trim_matches([' ', '\t', '\n', '\r', '\u{000C}']),
                    &row[end..]
                )
            })
            .collect::<String>()
    };
    let mut checks = Checks::default();
    checks.equal(
        &action_rows(QUALIFIERS),
        &expected_q.join("\n"),
        "qualifier Action: copula + participant, day + d alias",
    );
    checks.equal(
        &action_rows(PREFERENCES),
        &expected_p.join("\n"),
        "preference Action: copula + participant, day + d alias",
    );
    checks.finish("P6 Action");
}
#[test]
fn p6_bound_join_uses_quantity_links_not_nearest_builtin() {
    let fx = Fixture::new();
    let (_, ts, _) = timed("duration", "geq", "at least");
    let nt = model::annotate(&ts, true, true);
    let mut bundles = Vec::new();
    for position in 0..=nt.len() {
        let mut reordered = nt.clone();
        let i = reordered
            .iter()
            .position(|t| t.name() == "guideline_satisfies")
            .unwrap();
        let filter = reordered.remove(i);
        reordered.insert(position.min(reordered.len()), filter);
        reordered.insert(
            0,
            c(
                "guideline_satisfies",
                vec![
                    Term::V("decoy_s".into()),
                    Term::V("decoy_n".into()),
                    Term::V("decoy_u".into()),
                    a("less"),
                    a("99"),
                    a("week"),
                ],
            ),
        );
        bundles.push(body(&reordered, false));
    }
    fx.document(&bundles);
    let page = fx.request();
    let expected = (1..=bundles.len())
        .map(|s| {
            row(&[
                &s.to_string(),
                "condition",
                "wait",
                "lasts at least 2 hours",
                "",
            ])
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        rows(&page, TIMING),
        expected,
        "all filter placements + unrelated bound"
    );
}

#[test]
fn p6_all_unit_words_and_count_boundaries_come_from_the_bound() {
    let fx = Fixture::new();
    let mut bundles = Vec::new();
    let mut expected = Vec::new();
    for unit in ["second", "minute", "hour", "day", "week", "month", "year"] {
        for count in ["0".to_string(), "1".into(), "2".into(), "9".repeat(100)] {
            let mut ts = obj(1, "man", "eq", 1);
            ts.extend(obj(2, unit, "geq", 2));
            if let Term::C(_, args) = &mut ts[3] {
                args[4] = a(&count);
            }
            ts.extend(event(3, "wait", &[1]));
            ts.push(lit(
                "interval",
                vec![v(3), a("duration"), v(2), a(unit), a("none")],
            ));
            let converted = model::annotate(&ts, true, true);
            for excluded in [false, true] {
                bundles.push(body(&converted, excluded));
                let words = format!(
                    "lasts at least {count} {unit}{}",
                    if count == "1" { "" } else { "s" }
                );
                expected.push(row(&[
                    &bundles.len().to_string(),
                    if excluded {
                        "excluded condition"
                    } else {
                        "condition"
                    },
                    "wait",
                    &words,
                    "",
                ]));
            }
        }
    }
    fx.document(&bundles);
    let page = fx.request();
    assert_eq!(
        rows(&page, TIMING),
        expected.join("\n"),
        "seven units × zero/one/two/100 digits × both parts"
    );
}
