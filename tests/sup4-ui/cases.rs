use super::oracle::{self, Model, Row};
use super::{dq10, rev_k};
use super::support::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn extracted(page: &str, opening: &str) -> String {
    page.split_once(opening)
        .map_or_else(String::new, |(_, tail)| {
            let (body, _) = tail.split_once("</section>\n").expect("section close");
            format!("{opening}{body}</section>\n")
        })
}
fn stripped(c: &mut Checks, model: &Model, path: &str, body: &[u8], label: &str) -> Vec<u8> {
    if path.ends_with(".html") {
        let page = std::str::from_utf8(body).unwrap();
        c.strip(page, path, &model.insertion(path), label)
            .unwrap_or_else(|| page.into())
            .into_bytes()
    } else {
        body.to_owned()
    }
}

// The pins hold pre-REV-K renderer bytes: a page's DQ10 + REV-K sites map back before its hash.
fn pre_rev_k(path: &str, bytes: &[u8]) -> Vec<u8> {
    if path.ends_with(".html") {
        rev_k::map(&dq10::map(std::str::from_utf8(bytes).unwrap(), false), false).into_bytes()
    } else {
        bytes.to_owned()
    }
}

#[test]
fn p2_p3_frozen_pages_and_row_removal_differ_only_by_insertions() {
    let pins: BTreeMap<_, _> = include_str!("baseline.sha256")
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let (digest, name) = line.split_once('\t').unwrap();
            (name.to_owned(), digest.to_owned())
        })
        .collect();
    assert_eq!(pins.len(), 28, "2 fixtures × 2 controls × 7 files");
    let mut seen = BTreeSet::new();
    let mut c = Checks::default();
    for case in CASES {
        let fx = Fixture::new(case);
        let model = Model::load(&fx.tree);
        let mut fixed_inputs = files(&fx.tree);
        fixed_inputs.remove("guidelines/alpha/vocabulary.tsv");
        let with_rows = fx.render();
        let normalized: Files = with_rows
            .iter()
            .map(|(path, bytes)| {
                (
                    path.clone(),
                    stripped(&mut c, &model, path, bytes, &format!("P2 {case}/{path}")),
                )
            })
            .collect();
        for (path, bytes) in &normalized {
            let key = format!("{case}/rows/{path}");
            let expected = pins.get(&key).expect("pin inventory");
            seen.insert(key.clone());
            c.equal(
                sha(pre_rev_k(path, bytes)).as_bytes(),
                expected.as_bytes(),
                &format!("P2 baseline sha256 {key}"),
            );
        }
        fx.remove_rows();
        let no_sup = fx.render();
        let mut after_inputs = files(&fx.tree);
        after_inputs.remove("guidelines/alpha/vocabulary.tsv");
        c.require(
            fixed_inputs == after_inputs,
            format!("P3 {case}: PL/manifests/ledgers fixed"),
        );
        c.require(
            fx.guideline().join("vocabulary.tsv").exists() == (case == "mixed"),
            format!("P3 {case}: remove file iff no row remains"),
        );
        c.require(
            normalized.keys().eq(no_sup.keys()),
            format!("P3 {case}: page/source inventory fixed"),
        );
        for (path, bytes) in &no_sup {
            let key = format!("{case}/removed/{path}");
            seen.insert(key.clone());
            c.equal(
                sha(pre_rev_k(path, bytes)).as_bytes(),
                pins[&key].as_bytes(),
                &format!("P2 removed baseline sha256 {key}"),
            );
            c.equal(
                bytes,
                &normalized[path],
                &format!("P3 {case}/{path}: exact row-removal difference"),
            );
            if path.ends_with(".html") {
                c.require(
                    !page(&no_sup, path).contains(oracle::NOTICE_OPEN)
                        && !page(&no_sup, path).contains(oracle::SECTION_OPEN),
                    format!("P3 {case}/{path}: removed insertions"),
                );
            }
        }
    }
    c.require(
        seen == pins.keys().cloned().collect(),
        "P2 every committed pin reached",
    );
    c.finish("P2/P3 pins and row-removal");
}

#[test]
fn p4_notice_exact_once_escaped_and_anchored() {
    let mut c = Checks::default();
    for case in CASES {
        let fx = Fixture::new(case);
        let model = Model::load(&fx.tree);
        let pages = fx.render();
        for id in model.docs.keys() {
            let path = format!("g/alpha/doc/{id}.html");
            let actual = page(&pages, &path);
            let expected = model.notice(id);
            c.equal(
                extracted(actual, oracle::NOTICE_OPEN).as_bytes(),
                expected.as_bytes(),
                &format!("P4 {case}/{id}: exact notice"),
            );
            c.require(
                actual.matches(oracle::NOTICE_OPEN).count() == usize::from(!expected.is_empty()),
                format!("P4 {case}/{id}: notice iff matching document row, once"),
            );
            c.strip(
                actual,
                &path,
                &expected,
                &format!("P4 {case}/{id}: placement"),
            );
        }
    }
    c.finish("P4 exact notice");
}

#[test]
fn p4_notice_stale_reading_highlight_and_plain_branches() {
    let mut c = Checks::default();
    for stale in [false, true] {
        for readings in [false, true] {
            for highlighting in [false, true] {
                let fx = Fixture::new("mixed");
                let row = Row::new(
                    true,
                    "a-2",
                    "20240229",
                    "https://example.test/a?x=&y='\"<>'",
                    "Loi & <article> \"é\"",
                );
                fx.table(&oracle::table(&[row], readings));
                if stale {
                    fx.stale("a-2");
                }
                if highlighting {
                    fx.highlight();
                }
                let model = Model::load(&fx.tree);
                let pages = fx.render();
                let label = format!("P4 stale={stale} reading={readings} highlight={highlighting}");
                let path = "g/alpha/doc/a-2.html";
                let actual = page(&pages, path);
                c.require(
                    actual.contains(oracle::STALE) == stale,
                    format!("{label}: stale-state control"),
                );
                c.require(
                    actual.contains("<p class=\"hl-note\">") == highlighting,
                    format!("{label}: highlight control"),
                );
                c.require(
                    actual.contains("<script>") == (readings || highlighting),
                    format!("{label}: script branch control"),
                );
                c.equal(
                    extracted(actual, oracle::NOTICE_OPEN).as_bytes(),
                    model.notice("a-2").as_bytes(),
                    &label,
                );
                let before: Files = pages
                    .iter()
                    .map(|(path, body)| {
                        (
                            path.clone(),
                            stripped(&mut c, &model, path, body, &format!("{label}/{path}")),
                        )
                    })
                    .collect();
                fx.remove_rows();
                let after = fx.render();
                for (path, body) in &before {
                    c.equal(
                        body,
                        &after[path],
                        &format!("{label}/{path}: row-removal identity"),
                    );
                }
            }
        }
    }
    c.finish("P4 8 leading branches");
}

#[test]
fn p5_section_exact_order_and_empty_cells() {
    let mut c = Checks::default();
    for case in CASES {
        let fx = Fixture::new(case);
        let model = Model::load(&fx.tree);
        let pages = fx.render();
        let path = "g/alpha/index.html";
        let actual = page(&pages, path);
        let section = extracted(actual, oracle::SECTION_OPEN);
        let expected = model.section();
        c.equal(
            section.as_bytes(),
            expected.as_bytes(),
            &format!("P5 {case}: exact section + file order"),
        );
        c.require(
            actual.matches(oracle::SECTION_OPEN).count() == 1,
            format!("P5 {case}: section once"),
        );
        c.require(
            section.matches("<tr>").count() == model.rows.len() + 1,
            format!("P5 {case}: each SUP row + one header"),
        );
        c.strip(
            actual,
            path,
            &expected,
            &format!("P5 {case}: placement after meanings/status before Documents"),
        );
        c.require(
            actual.contains("<h2>Word meanings</h2>") == (case == "mixed"),
            format!("P5 {case}: predecessor control"),
        );
        if case == "mixed" {
            c.require(section.contains("<tr><td></td><td></td><td><a href=\"https://example.test/missing\">Missing document source</a></td><td>1900-01-01</td></tr>"), "P5 missing supersession key: both cells empty");
            c.require(section.contains("<tr><td>r1</td><td></td><td><a href=\"https://example.test/existing\">Passage with an ACE document</a></td><td>2000-02-29</td></tr>"), "P5 superseded region with document: document cell empty");
            c.require(
                !section.contains(">missing<"),
                "P5 missing document identifier stays invisible",
            );
        }
        fx.remove_rows();
        let without = fx.render();
        c.require(
            !page(&without, path).contains(oracle::SECTION_OPEN),
            format!("P5 {case}: no section with rows removed"),
        );
    }
    c.finish("P5 exact section");
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
fn p4_p5_generated_dates_escaping_and_permutations() {
    let mut rows = vec![
        Row::new(
            true,
            "a-2",
            "00000229",
            "https://example.test/zero",
            "Year zero",
        ),
        Row::new(
            true,
            "a-10",
            "20240229",
            "https://example.test/leap",
            "Leap day",
        ),
        Row::new(
            true,
            "z",
            "99991231",
            "https://example.test/last",
            "Final calendar day",
        ),
    ];
    let mut url_chars = BTreeSet::new();
    for (i, (year, month)) in [0u32, 1, 100, 400, 1900, 2000, 2024, 9999]
        .into_iter()
        .flat_map(|year| (1..=12).map(move |m| (year, m)))
        .enumerate()
    {
        let leap =
            year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100));
        let day = match month {
            2 => 28 + u32::from(leap),
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };
        let ch = char::from(33 + (i % 94) as u8);
        url_chars.insert(ch);
        rows.push(Row::new(
            false,
            &format!("passage-{i}<&\"'漢>"),
            &format!("{year:04}{month:02}{day:02}"),
            &format!("https://example.test/{ch}?x=1&y='\"<>'"),
            &format!("Law {ch} & <article> \"é\" — 漢🙂  two spaces"),
        ));
    }
    assert_eq!(
        url_chars.len(),
        94,
        "every admitted printable ASCII URL byte"
    );
    assert_eq!(
        rows.len(),
        99,
        "96 year/month endpoints + 3 document notices"
    );
    let mut rng = Rng(0xb12d_573e_2a0c_4491);
    let mut c = Checks::default();
    for permutation in 0..12 {
        for i in (1..rows.len()).rev() {
            let j = rng.next(i + 1);
            rows.swap(i, j);
        }
        let fx = Fixture::new("sup-only");
        fx.table(&oracle::table(&rows, permutation % 2 == 0));
        let model = Model::load(&fx.tree);
        let pages = fx.render();
        let label = format!("P4/P5 generated permutation {permutation}");
        c.equal(
            extracted(page(&pages, "g/alpha/index.html"), oracle::SECTION_OPEN).as_bytes(),
            model.section().as_bytes(),
            &format!("{label}: 99 rows, original order and escaped spans"),
        );
        let normalized: Files = pages
            .iter()
            .map(|(path, body)| {
                (
                    path.clone(),
                    stripped(&mut c, &model, path, body, &format!("{label}/{path}")),
                )
            })
            .collect();
        for id in model.docs.keys() {
            let path = format!("g/alpha/doc/{id}.html");
            let actual = page(&pages, &path);
            c.equal(
                extracted(actual, oracle::NOTICE_OPEN).as_bytes(),
                model.notice(id).as_bytes(),
                &format!("{label}/{id}: date and source"),
            );
        }
        fx.remove_rows();
        let without = fx.render();
        for (path, body) in &normalized {
            c.equal(
                body,
                &without[path],
                &format!("{label}/{path}: only D3/D4 changed"),
            );
        }
    }
    c.finish("P4/P5 generated domain seed=b12d573e2a0c4491");
}

#[test]
fn characterization_p6_check_and_copy_register() {
    let mut c = Checks::default();
    for case in CASES {
        let fx = Fixture::new(case);
        let out = fx.check();
        c.require(
            out.status.code() == Some(0),
            format!(
                "P6 {case}: ui check rc, stderr={}",
                String::from_utf8_lossy(&out.stderr)
            ),
        );
        c.require(out.stderr.is_empty(), format!("P6 {case}: ui check stderr"));
        c.require(
            out.stdout.ends_with(b"ui: check ok\n"),
            format!("P6 {case}: ui check positive output"),
        );
    }
    let fx = Fixture::new("mixed");
    let file = fx.scratch.join("copy.html");
    for (i, literal) in oracle::NEW_COPY.iter().enumerate() {
        write(&file, format!("X = {literal:?}\n"));
        let out = fx.command(&["ui", "copy-check", file.to_str().unwrap()]);
        c.require(
            out.status.code() == Some(0),
            format!(
                "P6 copy row {i}: rc={}, stderr={}",
                out.status,
                String::from_utf8_lossy(&out.stderr)
            ),
        );
        c.require(
            out.stderr.is_empty(),
            format!("P6 copy row {i}: empty stderr"),
        );
    }
    write(&file, "X = \"Get started now!\"\n");
    let bad = fx.command(&["ui", "copy-check", file.to_str().unwrap()]);
    c.require(
        bad.status.code() == Some(1),
        "P6 register control Get started now! rc 1",
    );
    c.require(
        String::from_utf8_lossy(&bad.stdout).contains("exclamatory"),
        "P6 register control diagnostic = exclamatory",
    );
    c.finish("characterization P6 check + copy");
}

#[test]
fn characterization_request_bodies_equal_render() {
    let mut c = Checks::default();
    for case in CASES {
        let fx = Fixture::new(case);
        let pages = fx.render();
        for (path, body) in &pages {
            if !path.ends_with(".html") {
                continue;
            }
            let out = fx.request(&format!("/{path}"));
            ok(&out, "P6 GET");
            let (head, actual) = response(&out);
            c.require(
                head.lines().next() == Some("HTTP 200"),
                format!("GET {case}/{path}: 200"),
            );
            c.require(
                head.contains("Referrer-Policy: no-referrer"),
                format!("GET {case}/{path}: referrer policy"),
            );
            c.equal(
                actual.as_bytes(),
                body,
                &format!("GET {case}/{path}: render bytes"),
            );
        }
    }
    c.finish("characterization route parity");
}

#[test]
fn characterization_empty_selection_omits_both_blocks() {
    let mut c = Checks::default();
    for table in [
        None,
        Some("unit\tday\tday\n"),
        Some("qualifier\treviewer\tmanner\n"),
    ] {
        let fx = Fixture::new("mixed");
        if let Some(body) = table {
            fx.table(&format!("{}{body}", oracle::HEADER));
        } else {
            fs::remove_file(fx.guideline().join("vocabulary.tsv")).unwrap();
        }
        let pages = fx.render();
        for (path, body) in pages {
            if !path.ends_with(".html") {
                continue;
            }
            let html = std::str::from_utf8(&body).unwrap();
            c.require(
                !html.contains(oracle::NOTICE_OPEN),
                format!("no SUP {table:?}/{path}: no notice"),
            );
            c.require(
                !html.contains(oracle::SECTION_OPEN),
                format!("no SUP {table:?}/{path}: no section"),
            );
        }
    }
    c.finish("characterization empty selection");
}

#[test]
fn characterization_invalid_tables_refuse_before_render() {
    let valid = "supersession\ta-2\t20240229 https://example.test/a Law\n";
    let mut invalid = vec![
        String::new(),
        valid.replace("20240229", "20230229"),
        valid.replace("20240229", "00010229"),
        valid.replace("20240229", "20241301"),
        valid.replace("20240229", "20240100"),
        valid.replace("20240229", "202402290"),
        valid.replace("https://", "http://"),
        valid.replace(" Law", "  Law"),
        valid.replace("Law\n", "Law \n"),
        valid.replace("Law\n", "Law\ttext\n"),
        valid.trim_end().to_owned(),
        format!("{valid}{valid}"),
    ];
    invalid.push(format!(
        "{valid}{}{}",
        valid.replace("supersession", "superseded"),
        valid.replace("supersession", "superseded")
    ));
    let fx = Fixture::new("mixed");
    let mut c = Checks::default();
    for (i, body) in invalid.into_iter().enumerate() {
        fx.table(&format!("{}{body}", oracle::HEADER));
        let outdir = fx.scratch.join(format!("invalid-{i}"));
        let out = fx.command(&[
            "ui",
            "render",
            outdir.to_str().unwrap(),
            fx.tree.to_str().unwrap(),
        ]);
        c.require(
            out.status.code() == Some(1),
            format!("D2 invalid table {i}: refusal rc"),
        );
        c.require(
            String::from_utf8_lossy(&out.stderr)
                .starts_with("ui: viewmodel: alpha vocabulary.tsv: "),
            format!("D2 invalid table {i}: grammar diagnostic"),
        );
        c.require(
            !outdir.join("index.html").exists(),
            format!("D2 invalid table {i}: no rendered output"),
        );
    }
    c.finish("characterization D2 invalid-table gate");
}

#[test]
fn characterization_strip_oracle_rejects_absent_duplicate_and_misplaced_blocks() {
    let fx = Fixture::new("mixed");
    let model = Model::load(&fx.tree);
    let notice = model.notice("a-2");
    let before = "<p>None. <a href=\"../records.html\">All decision records</a></p>\n";
    let after = "<section>\n<h3>Original passage</h3>\n";
    let correct = format!("{before}{notice}{after}");
    let path = "g/alpha/doc/a-2.html";
    let mut c = Checks::default();
    c.require(
        oracle::strip(&correct, path, &notice) == Ok(format!("{before}{after}")),
        "oracle correctly placed notice accepted",
    );
    for broken in [
        format!("{before}{after}"),
        format!("{before}{notice}{notice}{after}"),
        format!("{notice}{before}{after}"),
        format!("{before}{notice}<p>Extra</p>\n{after}"),
    ] {
        c.require(
            oracle::strip(&broken, path, &notice).is_err(),
            "oracle planted absent/duplicate/moved/successor notice rejects",
        );
    }
    let section = model.section();
    let before = "<section>\n<h2>Status</h2>\n<table></table>\n</section>\n";
    let after = "<section>\n<h2>Documents</h2>\n";
    c.require(
        oracle::strip(
            &format!("{before}{section}{after}"),
            "g/alpha/index.html",
            &section,
        ) == Ok(format!("{before}{after}")),
        "oracle correctly placed table accepted",
    );
    for broken in [
        format!("{before}{after}"),
        format!("{before}{section}{section}{after}"),
        format!("{section}{before}{after}"),
    ] {
        c.require(
            oracle::strip(&broken, "g/alpha/index.html", &section).is_err(),
            "oracle planted absent/duplicate/moved table rejects",
        );
    }
    c.finish("characterization oracle controls");
}
