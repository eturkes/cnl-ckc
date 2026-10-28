// UR P1–P7: contract-only renderer oracle + frozen pre-unit fixture census.
// CKC_UR_TEST_BIN selects the binary for seed/base replay; default = release ckc.
#[path = "../../../tests/frozen-pages/dq10.rs"]
mod dq10;
#[path = "../../../tests/ur-ui/legacy.rs"]
mod legacy;
#[path = "../../../tests/ur-ui/oracle.rs"]
mod oracle;
#[path = "../../../tests/frozen-pages/rev_k.rs"]
mod rev_k;
#[path = "../../../tests/ur-ui/support.rs"]
mod support;

use oracle::{Mark, Row};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::PathBuf;
use support::*;

fn section(page: &str, heading: &str) -> String {
    let start = format!("<section>\n<h2>{heading}</h2>\n");
    page.split_once(&start)
        .map_or_else(String::new, |(_, tail)| {
            format!(
                "{start}{}</section>\n",
                tail.split_once("</section>\n").unwrap().0
            )
        })
}
fn reading_rows() -> Vec<Row> {
    vec![
        Row::new("qualifier", "usually", "usuality"),
        Row::new("qualifier", "ideally", "ideality"),
    ]
}
fn inspect(fx: &Fixture, name: &str, checks: &mut Checks) -> Files {
    let pages = fx.render();
    for doc in &fx.docs {
        checks.oracle(&pages, doc, &fx.rows, name);
    }
    pages
}

#[test]
fn p1_document_sections_remove_every_compiled_table() {
    let rows = all_rows();
    let mut doc = Doc::new("all-types", "A patient should usually wait.\n");
    let facts = "guideline_entity(actual,p,patient,countable).\nguideline_cardinality(actual,p,na,eq,1).\nguideline_entity(actual,q,day,countable).\nguideline_cardinality(actual,q,na,eq,2).\nguideline_entity(actual,r,day,countable).\nguideline_cardinality(actual,r,na,eq,3).\nguideline_entity(actual,label,'category-B-recommendation',countable).\nguideline_event(actual,e,wait).\nguideline_arg(actual,e,1,p).\nguideline_interval(actual,e,duration,q,day,none).\nguideline_recurrence(actual,e,q,day).\nguideline_frequency(actual,e,p,q,day).\nguideline_order(actual,e,before,p).\nguideline_recurrence_window(actual,e,q,day,r,day,p).\nguideline_range(actual,q,r).\nguideline_grade(actual,p,strength,conditional,label).\nguideline_preference(actual,e,p,none).\nguideline_qualifier(actual,e,usuality,usually).\n";
    doc.pl = Some(
        doc.canonical(Some(&oracle::table(&rows)))
            .replace("guideline_event(actual,e,wait).\n", facts),
    );
    let fx = Fixture::new("p1", Some(rows), vec![doc]);
    let parsed = native(
        &fx.scratch.0,
        &[
            "v1",
            "check",
            fx.tree
                .join("guidelines/alpha/pl/all-types.pl")
                .to_str()
                .unwrap(),
        ],
    );
    ok(&parsed, "P1 canonical all-annotation input");
    let pages = fx.render();
    let page = document(&pages, "all-types");
    let mut c = Checks::default();
    for retired in [
        "Timing as compiled",
        "Grade as compiled",
        "Preferences as compiled",
        "Qualifiers as compiled",
    ] {
        c.require(
            !page.contains(retired),
            format!("P1 retired section still present: {retired}"),
        );
    }
    let expected = [
        "<h2>",
        "<p class=\"provenance\">",
        "<h3>Original passage</h3>",
        "<h3>Attempto Controlled English (ACE)</h3>",
        "<h3>Record a decision</h3>",
        "<summary>Compiled Prolog",
        "<nav class=\"docnav\">",
    ];
    let positions: Vec<_> = expected.iter().map(|s| page.find(s).expect(s)).collect();
    c.require(
        positions.windows(2).all(|p| p[0] < p[1]),
        "P1 D1 section order",
    );
    c.require(
        page.matches("<section").count() == 4,
        "P1 exactly passage, ACE, decision, Prolog sections",
    );
    c.require(
        block(page, "<title>", "</title>") == "Reading fixture",
        "P1 bare tab title",
    );
    for field in ["review_sha256", "ledger_sha256", "commit", "csrf"] {
        c.require(
            page.matches(&format!("<input type=\"hidden\" name=\"{field}\""))
                .count()
                == 1,
            format!("P1 hidden {field}"),
        );
    }
    c.finish("P1");
}

#[test]
fn p2_word_meanings_all_rows_exact_copy_and_order() {
    let rows = all_rows();
    let fx = Fixture::new(
        "p2-all",
        Some(rows.clone()),
        vec![Doc::new("words", "usually.\n")],
    );
    let pages = fx.render();
    let page = page(&pages, "g/alpha/index.html");
    let mut c = Checks::default();
    c.equal(
        &section(page, "Word meanings"),
        &oracle::meanings(&rows),
        "P2 all D3 readings in kind/file order",
    );
    for retired in [
        "Time words",
        "Grade words",
        "Preference words",
        "Qualifier words",
    ] {
        c.require(
            !page.contains(&format!("<h2>{retired}</h2>")),
            format!("P2 retired heading: {retired}"),
        );
    }
    c.require(
        page.contains(&format!(
            "{}{}<section>\n<h2>Documents</h2>",
            section(page, "Status"),
            oracle::meanings(&rows)
        )),
        "P2 Word meanings directly between Status and Documents",
    );
    c.finish("P2 all rows");
}

#[test]
fn p2_boundaries_collisions_escaping_and_intake() {
    let pair = || {
        vec![
            Row::new("scope", "recommendation", "alpha"),
            Row::new("exclusion", "apply", "scope"),
        ]
    };
    let mut cases = vec![
        ("none".to_owned(), None),
        ("time".into(), Some(vec![Row::new("unit", "day", "day")])),
        (
            "single".into(),
            Some(vec![Row::new("qualifier", "usually", "usuality")]),
        ),
        ("scope".into(), Some(pair())),
    ];
    for (kind, lemma, value) in [
        ("qualifier", "recommendation", "manner"),
        ("strength", "apply", "strong"),
        ("evidence", "applys", "high"),
        ("preference", "applyes", "preferred"),
        ("alternative", "applies", "alternative"),
    ] {
        let mut rows = pair();
        rows.insert(0, Row::new(kind, lemma, value));
        cases.push((format!("collision-{lemma}"), Some(rows)));
    }
    cases.push((
        "escaped".into(),
        Some(vec![
            Row::new("qualifier", "x<&\"'y", "manner"),
            Row::new("preference", "漢λ", "preferred"),
        ]),
    ));
    let mut c = Checks::default();
    for (label, rows) in cases {
        let fx = Fixture::new(
            &label,
            rows.clone(),
            vec![Doc::new("words", "usually x<&\"'y 漢λ.\n")],
        );
        let pages = fx.render();
        let page = page(&pages, "g/alpha/index.html");
        c.equal(
            &section(page, "Word meanings"),
            &oracle::meanings(rows.as_deref().unwrap_or_default()),
            &format!("P2 {label}"),
        );
        c.require(
            !page.contains("<h2>Time words</h2>"),
            format!("P2 {label}: no time table"),
        );
    }
    let fx = Fixture::new(
        "p2-intake",
        Some(reading_rows()),
        vec![Doc::new("words", "usually.\n")],
    );
    let g = fx.tree.join("guidelines/alpha");
    for (table, why) in [
        (
            "# format: kind\tlemma\tvalue\nqualifier\tusually\tbad\n",
            "qualifier value at row 1",
        ),
        (
            "# format: kind\tlemma\tvalue\nscope\trecommendation\talpha\n",
            "scope without exclusion at row 1",
        ),
        (
            "# format: kind\tlemma\tvalue\nexclusion\tapply\tscope\n",
            "exclusion without scope at row 1",
        ),
        (
            "# format: kind\tlemma\tvalue\nscope\trecommendation\tbeta\nexclusion\tapply\tscope\n",
            "scope key at row 1",
        ),
        ("# format: kind\tlemma\tvalue\n", "no rows"),
    ] {
        write(&g.join("vocabulary.tsv"), table);
        let out = fx.check();
        c.require(
            out.status.code() == Some(1),
            format!("P2 invalid table rc: {why}"),
        );
        c.equal(
            &String::from_utf8_lossy(&out.stderr),
            &format!("ui: viewmodel: alpha vocabulary.tsv: {why}\n"),
            "P2 table envelope",
        );
    }
    write(&g.join("vocabulary.tsv"), oracle::table(&reading_rows()));
    write(&g.join("temporal.tsv"), "retired\n");
    let out = fx.check();
    c.require(out.status.code() == Some(1), "P2 retired name rc");
    c.equal(
        &String::from_utf8_lossy(&out.stderr),
        "ui: viewmodel: alpha temporal.tsv: retired table name (rename to vocabulary.tsv)\n",
        "P2 retired name envelope",
    );
    c.finish("P2 boundaries");
}

#[test]
fn p3_every_kind_exact_tokens_time_exclusion_and_priority() {
    let mut rows = all_rows();
    rows.extend([
        Row::new("unit", "shared-time", "hour"),
        Row::new("qualifier", "shared-time", "manner"),
        Row::new("qualifier", "shared-grade", "usuality"),
        Row::new("strength", "shared-grade", "strong"),
        Row::new("qualifier", "x<&\"'y", "manner"),
        Row::new("qualifier", "pre_word", "manner"),
        Row::new("qualifier", "-leading", "manner"),
        Row::new("qualifier", "trailing-", "manner"),
        Row::new("qualifier", "a--b", "manner"),
        Row::new("qualifier", "9word", "focus"),
        Row::new("strength", "123", "conditional"),
    ]);
    let words = oracle::reading_rows(&rows)
        .iter()
        .filter(|r| r.kind != "scope" && r.kind != "exclusion")
        .map(|r| r.lemma.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let ace = format!(
        "{words}.\nUsually category-A-recommendations first-line-therapy recommendation apply applies applys applyes.\nday before at-spacing during-window per approximate to interval window.\nλusually—ideally. usually--ideally usually- ideally1 1usually.\n"
    );
    let fx = Fixture::new("p3-kinds", Some(rows), vec![Doc::new("kinds", &ace)]);
    let mut c = Checks::default();
    let pages = inspect(&fx, "P3 kinds", &mut c);
    c.require(
        !block(
            document(&pages, "kinds"),
            "<h3>Original passage</h3>",
            "</section>",
        )
        .contains("class=\"rw\""),
        "P3 source passage has no reading buttons",
    );
    c.finish("P3 kinds and exact tokens");
}

#[test]
fn p3_scope_pair_only_inside_paired_exclusion_sentences() {
    let mut c = Checks::default();
    for verb in [
        "apply", "act", "y", "Apply", "APPLY", "take-off", "take-", "9", "rétry",
    ] {
        let rows = vec![
            Row::new("scope", "recommendation", "alpha"),
            Row::new("exclusion", verb, "scope"),
            Row::new("qualifier", "usually", "usuality"),
            Row::new("strength", "category-A-recommendation", "strong"),
        ];
        let forms = oracle::forms(verb).join(" ");
        let ace = format!(
            "\n\nA recommendation is a category-A-recommendation.\n\nIf recommendation {verb} then usually.\nDoes recommendation {verb}?\n\nrecommendation {forms} usually.\n\nRecommendation recommendations {verb}-long {verb}X.\n"
        );
        let mut doc = Doc::new("scope", &ace);
        doc.exclusions = vec![4];
        let fx = Fixture::new("p3-scope", Some(rows), vec![doc]);
        let parsed = native(
            &fx.scratch.0,
            &[
                "v1",
                "check",
                fx.tree
                    .join("guidelines/alpha/pl/scope.pl")
                    .to_str()
                    .unwrap(),
            ],
        );
        ok(&parsed, "P3 canonical scope input");
        inspect(&fx, &format!("P3 verb={verb}"), &mut c);
    }
    let rows = vec![
        Row::new("scope", "recommendation", "alpha"),
        Row::new("exclusion", "apply", "scope"),
        Row::new("qualifier", "recommendation", "manner"),
    ];
    let mut doc = Doc::new(
        "collision",
        "recommendation applies.\nrecommendation applies.\n",
    );
    doc.exclusions = vec![2];
    let fx = Fixture::new("p3-scope-collision", Some(rows), vec![doc]);
    inspect(&fx, "P3 global row wins over scope", &mut c);
    c.finish("P3 scope forms");
}

#[test]
fn p3_scope_pairing_failures_keep_global_readings() {
    let rows = vec![
        Row::new("scope", "recommendation", "alpha"),
        Row::new("exclusion", "apply", "scope"),
        Row::new("qualifier", "usually", "usuality"),
    ];
    let table = oracle::table(&rows);
    let mut docs = Vec::new();
    for name in [
        "wrong-key",
        "compound-key",
        "variable-key",
        "unpaired",
        "unpaired-space",
        "invalid-pl",
        "body-only",
        "naf-only",
        "nested-record",
        "absent-line",
        "later-clause",
        "mixed-keys",
        "valid",
    ] {
        let mut doc = Doc::new(name, "recommendation applies usually.\n");
        doc.exclusions = vec![1];
        let pl = doc.canonical(Some(&table));
        doc.pl = Some(match name {
            "wrong-key" => pl.replace("guideline_exclusion(alpha).", "guideline_exclusion(beta)."),
            "compound-key" => pl.replace("guideline_exclusion(alpha).", "guideline_exclusion(key(alpha))."),
            "variable-key" => pl.replace("guideline_exclusion(alpha).", "guideline_exclusion(A)."),
            "unpaired" => pl.replace("% S1: recommendation applies usually.", "% S1: A different sentence."),
            "unpaired-space" => pl.replace("% S1: recommendation applies usually.", "% S1:  recommendation applies usually."),
            "invalid-pl" => format!("{pl}malformed(].\n"),
            "body-only" => pl.replace("guideline_exclusion(alpha).", "guideline_event(actual,e,wait) :- guideline_exclusion(alpha)."),
            "naf-only" => pl.replace("guideline_exclusion(alpha).", "guideline_event(actual,e,wait) :- \\+ guideline_exclusion(alpha)."),
            "nested-record" => pl.replace("guideline_exclusion(alpha).", "guideline_event(actual,e,guideline_exclusion(alpha))."),
            "absent-line" => pl.replace("% S1: recommendation applies usually.\nguideline_exclusion(alpha).", "% S1: recommendation applies usually.\nguideline_event(actual,e,wait).\n% S2: recommendation applies usually.\nguideline_exclusion(alpha)."),
            "later-clause" => pl.replace("guideline_exclusion(alpha).", "guideline_event(actual,e,wait).\nguideline_exclusion(alpha).\nguideline_event(actual,f,wait)."),
            "mixed-keys" => pl.replace("guideline_exclusion(alpha).", "guideline_exclusion(beta).\nguideline_exclusion(alpha)."),
            "valid" => pl,
            _ => unreachable!(),
        });
        if !matches!(name, "valid" | "later-clause" | "mixed-keys") {
            doc.exclusions.clear();
        }
        docs.push(doc);
    }
    let mut whitespace = Doc::new(
        "whitespace-line",
        "\n \n\nrecommendation applies usually.\n",
    );
    whitespace.exclusions = vec![2];
    docs.push(whitespace);
    let fx = Fixture::new("p3-pairing", Some(rows), docs);
    let mut c = Checks::default();
    for doc in &fx.docs {
        let input = fx.tree.join(format!("guidelines/alpha/pl/{}.pl", doc.id));
        let out = native(&fx.scratch.0, &["v1", "check", input.to_str().unwrap()]);
        if doc.id == "invalid-pl" {
            assert_eq!(
                out.status.code(),
                Some(2),
                "P3 malformed input positive control"
            );
        } else {
            ok(&out, &format!("P3 canonical input {}", doc.id));
        }
    }
    inspect(&fx, "P3 classifier", &mut c);
    c.finish("P3 pairing failures");
}

#[test]
fn p4_highlight_clipping_boundaries_and_unicode_are_exact() {
    let mut rows = reading_rows();
    rows.push(Row::new(
        "strength",
        "category-B-recommendation",
        "conditional",
    ));
    let cases = [
        ("around-word", "x usually y", vec![(0, 11)]),
        ("inside-word", "category-B-recommendation", vec![(0, 10)]),
        ("cross-start", "x usually y", vec![(0, 4)]),
        ("cross-end", "x usually y", vec![(5, 11)]),
        ("two-words", "usually,ideally", vec![(0, 15)]),
        ("punctuation", "usually,ideally", vec![]),
        ("start-end", "usually usually", vec![(0, 1), (14, 15)]),
        (
            "unicode",
            "λ漢🙂 usually <&\"'> ideally",
            vec![(0, 5), (7, 13)],
        ),
        (
            "unmarked-functions",
            "A patient should usually wait and ideally wait.",
            vec![],
        ),
    ];
    let mut docs: Vec<_> = cases
        .iter()
        .map(|(id, ace, spans)| {
            let mut doc = Doc::new(id, ace);
            doc.marks = spans
                .iter()
                .enumerate()
                .map(|(g, &(a, b))| Mark::new(a, b, g))
                .collect();
            doc
        })
        .collect();
    let prelude = "x ".repeat(47);
    let ace = format!("{prelude}usually ideally category-B-recommendation");
    let mut doc = Doc::new("group-boundary", &ace);
    doc.marks = (0..47).map(|i| Mark::new(2 * i, 2 * i + 1, i)).collect();
    for (g, (a, b)) in [(94, 101), (102, 109), (110, 135)].into_iter().enumerate() {
        doc.marks.push(Mark::new(a, b, 47 + g));
    }
    docs.push(doc);
    let mut shared = Doc::new("shared-group", "usually ideally usually");
    shared.marks = vec![
        Mark::new(0, 3, 0),
        Mark::new(8, 15, 1),
        Mark::new(16, 19, 0),
    ];
    docs.push(shared);
    let fx = Fixture::new("p4", Some(rows), docs);
    let mut c = Checks::default();
    inspect(&fx, "P4 clipping", &mut c);
    c.finish("P4 exact HTML");
}

#[test]
fn p5_reserved_line_distinct_first_occurrence_copy_and_no_reading_branch() {
    let mut rows = all_rows();
    rows.push(Row::new("qualifier", "x<&\"'y", "manner"));
    let long = "a".repeat(512);
    rows.push(Row::new("qualifier", &long, "manner"));
    let words = oracle::reading_rows(&rows)
        .into_iter()
        .filter(|r| r.kind != "scope" && r.kind != "exclusion")
        .rev()
        .map(|r| r.lemma.clone())
        .collect::<Vec<_>>()
        .join(" ");
    let mut repeated = Doc::new(
        "first-occurrence",
        &format!(
            "<&\"'> {words} usually usually ideally.\nrecommendation applies applys applyes apply.\n"
        ),
    );
    repeated.exclusions = vec![2];
    let docs = vec![
        repeated,
        Doc::new(
            "no-readings",
            "Usually category-A-recommendations first-line-therapy recommendation applies day.",
        ),
        Doc::new("hostile-only", "x<&\"'y"),
        Doc::new("empty", ""),
    ];
    let fx = Fixture::new("p5", Some(rows), docs);
    let mut c = Checks::default();
    let pages = inspect(&fx, "P5 line", &mut c);
    for id in ["no-readings", "hostile-only", "empty"] {
        c.require(
            !document(&pages, id).contains("class=\"reading\""),
            format!("P5 no reserved line: {id}"),
        );
    }
    c.finish("P5 first occurrence and escaping");
}

fn base64(bytes: &[u8]) -> String {
    const ABC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let x = (u32::from(chunk[0]) << 16)
            | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8)
            | u32::from(*chunk.get(2).unwrap_or(&0));
        out.push(char::from(ABC[((x >> 18) & 63) as usize]));
        out.push(char::from(ABC[((x >> 12) & 63) as usize]));
        out.push(if chunk.len() > 1 {
            char::from(ABC[((x >> 6) & 63) as usize])
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            char::from(ABC[(x & 63) as usize])
        } else {
            '='
        });
    }
    out
}

#[test]
fn p6_script_csp_styles_and_page_invariants() {
    let mut combined = Doc::new("combined", "usually,ideally usually.\n");
    combined.marks = vec![Mark::new(0, 5, 0), Mark::new(8, 15, 1)];
    let mut highlight = Doc::new("highlight-only", "domain label.\n");
    highlight.marks = vec![Mark::new(0, 6, 0), Mark::new(7, 12, 1)];
    let fx = Fixture::new(
        "p6",
        Some(reading_rows()),
        vec![
            Doc::new("reading-only", "usually,ideally usually.\n"),
            combined,
            highlight,
            Doc::new("plain", "domain label.\n"),
        ],
    );
    let pages = fx.render();
    let mut c = Checks::default();
    let current_script = script(document(&pages, "combined")).expect("combined highlight script");
    for (path, bytes) in &pages {
        if path.extension().is_none_or(|x| x != "html") {
            continue;
        }
        let html = std::str::from_utf8(bytes).unwrap();
        let label = path.display().to_string();
        c.require(
            html.matches("<script").count() <= 1,
            format!("P6 {label}: at most one script"),
        );
        c.require(
            !html.contains("tabindex") && !html.contains("style="),
            format!("P6 {label}: page invariants"),
        );
        let css = block(html, "<style>\n", "</style>");
        c.require(
            css.contains(&format!(
                "{}{}\n@media print {{\n",
                oracle::CSS,
                oracle::NARROW
            )),
            format!("P6 {label}: D8 screen rules immediately before narrow/print"),
        );
        c.require(
            css.ends_with(&format!("{}}}\n", oracle::PRINT_CSS)),
            format!("P6 {label}: D8 rules last inside print"),
        );
        c.require(
            css.contains("button:focus-visible") && css.contains("outline: 3px solid #1d4ed8"),
            format!("P6 {label}: visible focus retained"),
        );
    }
    for (id, expected) in [
        ("reading-only", true),
        ("combined", true),
        ("highlight-only", true),
        ("plain", false),
    ] {
        let html = document(&pages, id);
        c.require(
            script(html).is_some() == expected,
            format!("P6 {id}: script iff highlights or readings"),
        );
        if let Some(body) = script(html) {
            c.equal(body, current_script, &format!("P6 {id}: one fixed script"));
        }
        let out = fx.request(&format!("/g/alpha/doc/{id}.html"));
        ok(&out, "P6 HTTP request");
        let (head, body) = response(&out.stdout);
        c.require(
            head.contains(&format!(
                "script-src 'sha256-{}'",
                base64(&Sha256::digest(current_script.as_bytes()))
            )),
            format!("P6 {id}: exact script-body CSP digest"),
        );
        c.equal(body, html, &format!("P6 {id}: request/render identity"));
    }
    let out = fx.check();
    c.require(
        out.status.success() && out.stderr.is_empty() && out.stdout.ends_with(b"ui: check ok\n"),
        format!("P6 ui check: {}", String::from_utf8_lossy(&out.stderr)),
    );
    c.finish("P6 script and chrome");
}

#[test]
fn p6_browser_transitions_and_handler_strip_negative_control() {
    let scratch = Scratch::new("browser");
    let tree = scratch.0.join("tree");
    copy_tree(&root().join("tests/ur-ui/browser/tree"), &tree, true);
    let g = tree.join("guidelines/alpha");
    let manifest = native(&scratch.0, &["derive-review-manifest", g.to_str().unwrap()]);
    ok(&manifest, "browser fixture manifest");
    write(&g.join("audit/review-manifest.tsv"), manifest.stdout);
    let pages = scratch.0.join("pages");
    ok(
        &native(
            &scratch.0,
            &[
                "ui",
                "render",
                pages.to_str().unwrap(),
                tree.to_str().unwrap(),
            ],
        ),
        "browser render",
    );
    let tools = std::env::var_os("CKC_TOOLCHAIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join(".toolchain"));
    let probe = root().join("tools/ui-reading-probe.mjs");
    let node = tools.join("bin/node");
    let browser = tools.join("bin/chrome-headless-shell");
    let run = |args: &[&str]| command(&node, &scratch.0, args);
    let green = run(&[
        probe.to_str().unwrap(),
        browser.to_str().unwrap(),
        pages.to_str().unwrap(),
    ]);
    assert_eq!(
        green.status.code(),
        Some(0),
        "P6 browser transitions: stdout={} stderr={}",
        String::from_utf8_lossy(&green.stdout),
        String::from_utf8_lossy(&green.stderr)
    );
    let red = scratch.0.join("no-reading-handler");
    let stripped = run(&[
        probe.to_str().unwrap(),
        "--strip-handler",
        pages.to_str().unwrap(),
        red.to_str().unwrap(),
    ]);
    ok(&stripped, "P6 handler strip with checked anchors");
    let bad = run(&[
        probe.to_str().unwrap(),
        browser.to_str().unwrap(),
        red.to_str().unwrap(),
    ]);
    assert_eq!(
        bad.status.code(),
        Some(1),
        "P6 stripped handler must fail transitions: stdout={} stderr={}",
        String::from_utf8_lossy(&bad.stdout),
        String::from_utf8_lossy(&bad.stderr)
    );
    assert!(
        String::from_utf8_lossy(&bad.stdout).contains("visible reading"),
        "P6 negative control must name a failed reading transition"
    );
}

struct Rng(u64);
impl Rng {
    fn next(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 as usize) % n
    }
}

#[test]
fn p2_generated_table_permutations_preserve_reading_priority_and_row_order() {
    let mut rng = Rng(0x3d27_c591_7b20_d8a3);
    let mut rows = all_rows();
    rows.extend([
        Row::new("qualifier", "shared-grade", "usuality"),
        Row::new("strength", "shared-grade", "strong"),
        Row::new("preference", "shared-choice", "preferred"),
        Row::new("alternative", "shared-choice", "alternative"),
        Row::new("qualifier", "shared-modifier", "manner"),
        Row::new("alternative", "shared-modifier", "alternative"),
        Row::new("preference", "evidence-high", "preferred"),
    ]);
    let mut c = Checks::default();
    for i in 0..32 {
        for j in (1..rows.len()).rev() {
            let k = rng.next(j + 1);
            rows.swap(j, k);
        }
        let fx = Fixture::new(
            "table-permutation",
            Some(rows.clone()),
            vec![Doc::new(
                "priority",
                "shared-grade shared-choice shared-modifier evidence-high.\n",
            )],
        );
        let pages = inspect(&fx, &format!("P2 permutation {i}"), &mut c);
        c.equal(
            &section(page(&pages, "g/alpha/index.html"), "Word meanings"),
            &oracle::meanings(&rows),
            &format!("P2 permutation {i}: D3 kind/file order"),
        );
    }
    c.finish("P2 32 table permutations");
}

#[test]
fn oracle_generated_256_cases_cover_d5_d6_composition() {
    let mut rng = Rng(0x6b5a_74d2_190e_f3c1);
    let mut rows = all_rows();
    rows.extend([
        Row::new("qualifier", "lex9-x2", "focus"),
        Row::new("qualifier", "usually2", "manner"),
        Row::new("strength", "123", "conditional"),
        Row::new("qualifier", "9word", "focus"),
        Row::new("qualifier", "x<&\"'y", "manner"),
    ]);
    let reading = oracle::reading_rows(&rows)
        .iter()
        .filter(|r| {
            r.kind != "scope"
                && r.kind != "exclusion"
                && r.lemma.is_ascii()
                && !r.lemma.contains('<')
        })
        .map(|r| r.lemma.clone())
        .collect::<Vec<_>>();
    let separators = [" ", ",", ";", "—", "λ", "--", "<&\"'>", "🙂", "\t"];
    let mut docs = Vec::new();
    let mut modes = BTreeSet::new();
    for i in 0..256 {
        let mut ace = if i % 3 == 0 {
            "\n\n".into()
        } else {
            String::new()
        };
        let mut marks = Vec::new();
        if i % 16 == 0 {
            for g in 0..49 {
                let a = ace.chars().count();
                ace += "x ";
                marks.push(Mark::new(a, a + 1, g));
            }
        }
        ace += "A patient should ";
        let word = &reading[i % reading.len()];
        let start = ace.chars().count();
        ace += word;
        let end = ace.chars().count();
        let mode = i % 8;
        modes.insert(mode);
        let group = marks.len();
        match mode {
            0 => (),
            1 => marks.push(Mark::new(start, end, group)),
            2 if word.len() > 4 => marks.push(Mark::new(start + 1, end - 2, group)),
            3 => marks.push(Mark::new(start - 1, start + 2, group)),
            4 => marks.push(Mark::new(end - 2, end + 1, group)),
            5 => marks.push(Mark::new(start - 1, end + 1, group)),
            6 => marks.push(Mark::new(start + 1, end, group)),
            _ => marks.push(Mark::new(start, start + 2, group)),
        }
        let separator = separators[rng.next(separators.len())];
        ace += if separator == "\t" && marks.iter().any(|m| m.end > end) {
            " "
        } else {
            separator
        };
        for _ in 0..(2 + rng.next(7)) {
            ace += &reading[rng.next(reading.len())];
            ace += separators[rng.next(separators.len())];
        }
        ace += "Usually first-line-therapy category-A-recommendations day lex9-x2.\n";
        let scope_line = "recommendation apply applys applyes applies usually.";
        if i % 5 == 0 {
            ace += "\n";
        }
        ace += scope_line;
        ace += "\nrecommendation applies usually2.\n";
        let mut doc = Doc::new(&format!("generated-{i:03}"), &ace);
        doc.marks = marks;
        if i % 2 == 0 {
            doc.exclusions = vec![2];
        }
        if i % 7 == 0 {
            let pl = doc.canonical(Some(&oracle::table(&rows))).replace(
                "guideline_exclusion(alpha).",
                "guideline_exclusion(other_key).",
            );
            doc.pl = Some(pl);
            doc.exclusions.clear();
        }
        docs.push(doc);
    }
    assert_eq!(modes, BTreeSet::from([0, 1, 2, 3, 4, 5, 6, 7]));
    let fx = Fixture::new("generated", Some(rows), docs);
    let mut c = Checks::default();
    inspect(&fx, "ORACLE seed=6b5a74d2190ef3c1", &mut c);
    c.finish("ORACLE 256 generated cases");
}
