use super::base::HEADER;
use super::model::{DATE, URL, atom};
use super::support::*;
use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

fn name(bytes: &[u8]) -> bool {
    !bytes.is_empty()
        && bytes[0] != b'-'
        && bytes
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
}
fn lemma(bytes: &[u8]) -> bool {
    !bytes.is_empty()
        && bytes.iter().all(|b| *b > 32 && *b != 127)
        && std::str::from_utf8(bytes).is_ok()
}
// Leap years repeat every 400 years; count common month days + one leap day.
fn calendar(d: &[u8]) -> bool {
    if d.len() != 8 || !d.iter().all(u8::is_ascii_digit) {
        return false;
    }
    let number = |s: &[u8]| s.iter().fold(0u32, |n, b| n * 10 + u32::from(b - b'0'));
    let (y, m, day) = (number(&d[..4]), number(&d[4..6]), number(&d[6..]));
    let days = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            28 + u32::from(y.is_multiple_of(400) || (y.is_multiple_of(4) && !y.is_multiple_of(100)))
        }
        _ => return false,
    };
    (1..=days).contains(&day)
}
fn value(v: &[u8]) -> bool {
    if v.len() < 9 || v[8] != b' ' || !calendar(&v[..8]) {
        return false;
    }
    let rest = &v[9..];
    let Some(at) = rest.iter().position(|b| *b == b' ') else {
        return false;
    };
    let (url, citation) = (&rest[..at], &rest[at + 1..]);
    url.starts_with(b"https://")
        && url.len() > 8
        && url.iter().all(|b| (33..=126).contains(b))
        && !citation.is_empty()
        && citation[0] != b' '
        && citation.last() != Some(&b' ')
        && citation.iter().all(|b| *b >= 32 && *b != 127)
        && std::str::from_utf8(citation).is_ok()
}
// Closed generated domain: the two new row kinds + the named inherited-order controls.
fn verdict(bytes: &[u8]) -> Result<usize, String> {
    let Some(body) = bytes.strip_prefix(HEADER.as_bytes()) else {
        return Err("header".into());
    };
    if body.is_empty() {
        return Err("no rows".into());
    }
    if body.last() != Some(&b'\n') {
        return Err("final newline".into());
    }
    let (mut supersession, mut superseded) = (BTreeSet::new(), BTreeSet::new());
    let (mut scope, mut exclusion) = (None, None);
    let rows: Vec<_> = body[..body.len() - 1].split(|b| *b == b'\n').collect();
    for (i, row) in rows.iter().enumerate() {
        let f: Vec<_> = row.split(|b| *b == b'\t').collect();
        let why = if row.starts_with(b"#") {
            Some("comment")
        } else if f.len() != 3 {
            Some("field count")
        } else if !lemma(f[1]) {
            Some("lemma")
        } else {
            match f[0] {
                b"supersession" if !name(f[1]) => Some("supersession document"),
                b"supersession" | b"superseded" if !value(f[2]) => Some("supersession value"),
                b"supersession" => (!supersession.insert(f[1])).then_some("duplicate supersession"),
                b"superseded" => (!superseded.insert(f[1])).then_some("duplicate superseded"),
                b"scope" => {
                    scope = Some(i + 1);
                    None
                }
                b"exclusion" => {
                    exclusion = Some(i + 1);
                    None
                }
                b"unit" if f[1] == b"hour" && f[2] == b"hour" => None,
                _ => Some("kind"),
            }
        };
        if let Some(why) = why {
            return Err(format!("{why} at row {}", i + 1));
        }
    }
    match (scope, exclusion) {
        (Some(n), None) => Err(format!("scope without exclusion at row {n}")),
        (None, Some(n)) => Err(format!("exclusion without scope at row {n}")),
        _ => Ok(rows.len()),
    }
}
fn table(rows: &[String]) -> Vec<u8> {
    format!("{HEADER}{}\n", rows.join("\n")).into_bytes()
}
fn row(kind: &str, id: &str, val: &str) -> String {
    format!("{kind}\t{id}\t{val}")
}
fn valid() -> String {
    format!("{DATE} {URL} Act")
}
#[derive(Clone)]
struct Plant {
    id: String,
    text: Vec<u8>,
}
fn plants() -> Vec<Plant> {
    let mut out = Vec::new();
    for kind in ["supersession", "superseded"] {
        let id = if kind == "supersession" {
            "doc"
        } else {
            "S39-14"
        };
        let mut add = |label: &str, v: &str| {
            out.push(Plant {
                id: format!("{kind}/{label}"),
                text: table(&[row(kind, id, v)]),
            })
        };
        for (label, v) in [
            ("ordinary", valid()),
            ("leap-four", "20240229 https://a C".into()),
            ("leap-four-hundred", "20000229 https://a C".into()),
            ("year-zero", "00000229 https://a C".into()),
            ("first-date", "00000101 https://a C".into()),
            ("last-date", "99991231 https://a C".into()),
            ("citation-split", "20221229 https://a b C".into()),
            ("unicode-citation", "20221229 https://a 名称 § Ω 🗎".into()),
            (
                "quoted-citation",
                "20221229 https://a/'\\ A 'quoted' \\ citation".into(),
            ),
            ("seven-digits", "2022122 https://a C".into()),
            ("nine-digits", "202212299 https://a C".into()),
            ("letter-date", "2022x229 https://a C".into()),
            ("nonleap", "20230229 https://a C".into()),
            ("february-thirty", "20240230 https://a C".into()),
            ("century", "19000229 https://a C".into()),
            ("month-zero", "20220001 https://a C".into()),
            ("month-thirteen", "20221301 https://a C".into()),
            ("day-zero", "20221200 https://a C".into()),
            ("day-thirty-two", "20221232 https://a C".into()),
            ("http", "20221229 http://a C".into()),
            ("empty-url", "20221229 https:// C".into()),
            ("no-separator", "20221229https://a C".into()),
            ("empty-url-token", "20221229  https://a C".into()),
            ("citation-leading-space", "20221229 https://a  C".into()),
            ("url-del", "20221229 https://a\u{7f} C".into()),
            ("url-control", "20221229 https://a\u{1f} C".into()),
            ("url-unicode", "20221229 https://é C".into()),
            ("empty-citation", "20221229 https://a ".into()),
            ("missing-citation", "20221229 https://a".into()),
            ("citation-control", "20221229 https://a C\u{1f}D".into()),
            ("citation-del", "20221229 https://a C\u{7f}D".into()),
            ("citation-trailing-space", "20221229 https://a C ".into()),
        ] {
            add(label, &v);
        }
        for point in [
            0x80, 0x9f, 0xa0, 0x2028, 0x2029, 0x2066, 0x2069, 0xfeff, 0x10ffff,
        ] {
            let ch = char::from_u32(point).unwrap();
            out.push(Plant {
                id: format!("{kind}/unicode-U+{point:04X}"),
                text: table(&[row(kind, id, &format!("20221229 https://a C{ch}D"))]),
            });
        }
        for suffix in [
            vec![0xff],
            vec![0xc0, 0xaf],
            vec![0xed, 0xa0, 0x80],
            vec![0xf4, 0x90, 0x80, 0x80],
            vec![0xe2, 0x82],
        ] {
            let mut bytes = format!("{HEADER}{kind}\t{id}\t20221229 https://a C").into_bytes();
            bytes.extend(&suffix);
            bytes.push(b'\n');
            out.push(Plant {
                id: format!("{kind}/invalid-utf8-{suffix:x?}"),
                text: bytes,
            });
        }
        for next in [valid(), "20230101 https://other Other".into(), "bad".into()] {
            out.push(Plant {
                id: format!("{kind}/duplicate-{next}"),
                text: table(&[row(kind, id, &valid()), row(kind, id, &next)]),
            });
        }
    }
    for id in [
        "", "Doc", "-doc", "é", "doc_bad", "9", "a-", "a--9", "#doc", "do c", "d\u{7f}c",
        "d\u{1f}c",
    ] {
        for kind in ["supersession", "superseded"] {
            out.push(Plant {
                id: format!("{kind}/id-{id:?}"),
                text: table(&[row(kind, id, &valid())]),
            });
        }
    }
    for (id, rows) in [
        (
            "distinct",
            vec![
                row("supersession", "one", &valid()),
                row("supersession", "two", &valid()),
                row("superseded", "R1", &valid()),
                row("superseded", "R2", &valid()),
            ],
        ),
        (
            "cross-kind-id",
            vec![
                row("supersession", "doc", &valid()),
                row("superseded", "doc", &valid()),
            ],
        ),
        ("id-before-value", vec![row("supersession", "Doc", "bad")]),
        (
            "value-before-duplicate",
            vec![
                row("supersession", "doc", &valid()),
                row("supersession", "doc", "bad"),
            ],
        ),
        (
            "scope-before-invalid-row",
            vec![
                "scope\trecommendation\talpha".into(),
                row("supersession", "doc", "bad"),
            ],
        ),
        (
            "supersession-before-unpaired",
            vec![
                row("supersession", "doc", &valid()),
                "scope\trecommendation\talpha".into(),
            ],
        ),
        (
            "superseded-before-unpaired",
            vec![
                row("superseded", "R1", &valid()),
                "exclusion\tapply\tscope".into(),
            ],
        ),
    ] {
        out.push(Plant {
            id: id.into(),
            text: table(&rows),
        });
    }
    out
}
fn inherited() -> Vec<Plant> {
    let mut out = [
        "#supersession\t\tbad\textra",
        "supersession\t",
        "supersession\t\tbad",
        "BAD\t\tbad",
        "BAD\tx\tbad",
        "supersession\tbad lemma\tbad",
        "supersession\tdoc\tbad\textra",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, s)| Plant {
        id: format!("inherited-order-{i}"),
        text: table(&[s.into()]),
    })
    .collect::<Vec<_>>();
    for (id, text) in [
        ("header", b"bad\n".to_vec()),
        ("no-rows", HEADER.as_bytes().to_vec()),
        (
            "newline",
            format!("{HEADER}supersession\tdoc\t{}", valid()).into_bytes(),
        ),
    ] {
        out.push(Plant {
            id: id.into(),
            text,
        });
    }
    out
}
fn ui_tree(scratch: &Scratch) -> PathBuf {
    let tree = scratch.0.join("tree");
    copy_tree(&root().join("tests/ui/green/basic/tree"), &tree);
    tree
}
fn raw_check(p: &Plant, checks: &mut Checks) {
    let path = "tests/sup1-surface/vocabulary.tsv";
    let got = ckc_kernel::contract::check_vocabulary(path.as_bytes(), &p.text);
    let rendered = ckc_kernel::contract::check_render(&got);
    let got = Run {
        rc: i32::from(rendered.rc),
        stdout: String::from_utf8(rendered.out).unwrap(),
        stderr: String::new(),
    };
    let expected = match verdict(&p.text) {
        Ok(n) => Run::ok(format!("ckc: vocabulary ok {path} {n} rows\n")),
        Err(why) => Run {
            rc: 1,
            stdout: format!("ckc: vocabulary: {path}: {why}\n"),
            stderr: String::new(),
        },
    };
    checks.output(&got, &expected, &p.id);
}
fn preflight(bytes: &[u8]) -> Option<String> {
    let Ok(raw) = std::str::from_utf8(bytes) else {
        return Some("file not UTF-8: vocabulary.tsv".into());
    };
    raw.chars().find(|c| matches!(u32::from(*c), 0x00..=0x08 | 0x0b..=0x1f | 0x7f | 0x202a..=0x202e | 0x2066..=0x2069))
        .map(|c| format!("unsupported control U+{:04X} in vocabulary.tsv", u32::from(c)))
}
fn ui_check(scratch: &Scratch, tree: &Path, p: &Plant, checks: &mut Checks) {
    write(tree.join("guidelines/alpha/vocabulary.tsv"), &p.text);
    let got = native(&scratch.0, &["ui", "check", tree.to_str().unwrap()]);
    let expected = match preflight(&p.text) {
        Some(why) => Err(why),
        None => verdict(&p.text).map_err(|why| format!("vocabulary.tsv: {why}")),
    };
    match expected {
        Ok(n) => checks.require(
            got.success(),
            format!("{} {n} rows accepted; {}", p.id, got.describe()),
        ),
        Err(why) => checks.output(
            &got,
            &Run::err(1, format!("ui: viewmodel: alpha {why}\n")),
            &p.id,
        ),
    }
}
#[test]
fn p1_table_all_contract_plants() {
    let mut checks = Checks::default();
    for p in plants() {
        raw_check(&p, &mut checks);
    }
    checks.finish("P1 raw-byte table plants");
}
#[test]
fn p1_inherited_error_order_characterization() {
    let scratch = Scratch::new("table-order");
    let tree = ui_tree(&scratch);
    let mut checks = Checks::default();
    for p in inherited() {
        raw_check(&p, &mut checks);
        ui_check(&scratch, &tree, &p, &mut checks);
    }
    checks.finish("P1 inherited grammar characterization");
}
#[test]
fn p1_gregorian_400_year_cycle_and_month_boundary_property() {
    let mut dates = BTreeSet::new();
    for year in 0..400 {
        dates.insert(format!("{year:04}0229"));
    }
    for year in [0, 1900, 2000, 2023, 2024, 9600, 9999] {
        for month in 0..=13 {
            for day in [0, 1, 28, 29, 30, 31, 32] {
                dates.insert(format!("{year:04}{month:02}{day:02}"));
            }
        }
    }
    let mut checks = Checks::default();
    for (i, date) in dates.iter().enumerate() {
        let kind = if i % 2 == 0 {
            "supersession"
        } else {
            "superseded"
        };
        let p = Plant {
            id: format!("date-{date}/{kind}"),
            text: table(&[row(kind, "doc", &format!("{date} https://a C"))]),
        };
        raw_check(&p, &mut checks);
    }
    checks.finish("P1 Gregorian generated domain");
}
#[test]
fn p1_docid_and_url_citation_byte_properties() {
    let mut checks = Checks::default();
    let alphabet = ["a", "0", "-", "A", "_", " ", "é"];
    let mut words = vec![String::new()];
    let mut layer = words.clone();
    for _ in 0..3 {
        layer = layer
            .iter()
            .flat_map(|p| alphabet.iter().map(move |c| format!("{p}{c}")))
            .collect();
        words.extend(layer.clone());
    }
    words.push("a".repeat(1024));
    for word in words {
        let p = Plant {
            id: format!("docid-{word:?}"),
            text: table(&[row("supersession", &word, &valid())]),
        };
        raw_check(&p, &mut checks);
    }
    for byte in 0u8..=255 {
        if matches!(byte, b'\t' | b'\n' | b' ') {
            continue;
        }
        for url in [true, false] {
            let mut text = if url {
                format!("{HEADER}supersession\tdoc\t20221229 https://a").into_bytes()
            } else {
                format!("{HEADER}superseded\tR\t20221229 https://a C").into_bytes()
            };
            text.push(byte);
            text.extend_from_slice(if url { b" C\n" } else { b"D\n" });
            raw_check(
                &Plant {
                    id: format!("byte-{byte}/{url}"),
                    text,
                },
                &mut checks,
            );
        }
    }
    checks.finish("P1 docid + byte-language generated domain");
}
#[test]
fn p1_ui_file_preflight_characterization() {
    let scratch = Scratch::new("ui-file");
    let tree = ui_tree(&scratch);
    let mut checks = Checks::default();
    let mut cases = plants();
    for byte in 0u8..=255 {
        let mut text = format!("{HEADER}supersession\tdoc\t20221229 https://a C").into_bytes();
        text.push(byte);
        text.extend_from_slice(b"D\n");
        cases.push(Plant {
            id: format!("ui-byte-{byte}"),
            text,
        });
    }
    for p in cases.into_iter().filter(|p| preflight(&p.text).is_some()) {
        ui_check(&scratch, &tree, &p, &mut checks);
    }
    assert!(checks.count > 0, "file refusal controls");
    checks.finish("P1 already-passing UI file preflight");
}
#[test]
fn p1_fork_row_laws_and_fixture_docid_exemption() {
    let compiler = compiler();
    let scratch = Scratch::new("fork-table");
    let mut checks = Checks::default();
    for p in plants().into_iter().chain(inherited()) {
        // A fixture table may name any document; the probe itself is intentionally unmatched.
        let got = compiler.run(
            &scratch.0,
            "table-probe",
            "doc",
            "A man sleeps.\n",
            Some(&p.text),
        );
        match verdict(&p.text) {
            Ok(_) => {
                checks.require(
                    got.success(),
                    format!("{} accepted fixture table; {}", p.id, got.describe()),
                );
                if got.success() {
                    checks.require(
                        got.stdout
                            .contains(&format!("vocabulary(sha256({}))", atom(&sha(&p.text)))),
                        format!("{} raw-table custody", p.id),
                    );
                }
            }
            Err(why) => {
                checks.require(
                    got.rc == 2
                        && got.stdout.is_empty()
                        && got.stderr.starts_with("ace_to_pl_error(vocabulary_load,")
                        && got.stderr.lines().count() == 1,
                    format!(
                        "{} expected vocabulary_load ({why}); {}",
                        p.id,
                        got.describe()
                    ),
                );
                // The fork's existing row envelope spells grammar reasons as underscore atoms.
                if std::str::from_utf8(&p.text).is_ok()
                    && let Some((reason, n)) = why.split_once(" at row ")
                    && (reason.starts_with("supersession")
                        || reason.starts_with("duplicate supers"))
                {
                    checks.require(
                        got.stderr.contains(&format!(
                            "vocabulary(row({n},{}))",
                            reason.replace(' ', "_")
                        )),
                        format!(
                            "{} expected row({n},{}); {}",
                            p.id,
                            reason.replace(' ', "_"),
                            got.describe()
                        ),
                    );
                }
            }
        }
    }
    checks.finish("P1 fork row grammar + fixture exemption");
}
#[test]
fn p1_ui_new_rows_have_no_reading_or_guideline_page_effect() {
    let scratch = Scratch::new("page-intake");
    let tree = ui_tree(&scratch);
    let base = table(&["unit\thour\thour".into()]);
    write(tree.join("guidelines/alpha/vocabulary.tsv"), &base);
    let request = || {
        native(
            &scratch.0,
            &[
                "ui",
                "request",
                "GET",
                "/g/alpha/index.html",
                tree.to_str().unwrap(),
            ],
        )
    };
    let before = request();
    before.require_ok("baseline guideline page");
    assert!(
        before.stdout.contains("<h2>Status</h2>"),
        "positive guideline page control"
    );
    let mut checks = Checks::default();
    for (label, rows) in [
        (
            "matching",
            vec![
                row("supersession", "a-2", &valid()),
                row("superseded", "r20", &valid()),
            ],
        ),
        (
            "unmatched",
            vec![
                row("supersession", "absent-document", &valid()),
                row("superseded", "absent-region", &valid()),
            ],
        ),
    ] {
        let mut rows = rows;
        rows.insert(0, "unit\thour\thour".into());
        write(tree.join("guidelines/alpha/vocabulary.tsv"), table(&rows));
        checks.output(&request(), &before, label);
    }
    checks.finish("P1 UI intake + unchanged guideline page");
}
#[test]
fn p1_check_corpus_cross_ledger_and_meter() {
    let scratch = Scratch::new("corpus-check");
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
        "#!/bin/sh\n[ \"$1\" = --version ] && { echo 'SWI-Prolog version 10.0.2 for x86_64-linux'; exit 0; }\necho 'sup1-surface: reached SWI stage' >&2\nexit 3\n",
    );
    fs::set_permissions(&stop, fs::Permissions::from_mode(0o755)).unwrap();
    let path = "guidelines/cdc-2022-opioid/vocabulary.tsv";
    let original = fs::read(tree.join(path)).unwrap();
    let coverage =
        fs::read_to_string(tree.join("guidelines/cdc-2022-opioid/coverage.tsv")).unwrap();
    let rows: Vec<Vec<_>> = coverage
        .lines()
        .filter(|s| !s.starts_with('#'))
        .map(|s| s.split('\t').collect())
        .collect();
    let ace = rows.iter().find(|r| r[4].starts_with("ace(")).unwrap();
    let doc = ace[4]
        .strip_prefix("ace(")
        .unwrap()
        .strip_suffix(')')
        .unwrap();
    let uncovered = rows
        .iter()
        .find(|r| r[4].starts_with("uncovered("))
        .unwrap()[0];
    let restated = rows.iter().find(|r| r[4].starts_with("restates(")).unwrap()[0];
    let mut checks = Checks::default();
    for (label, new, expected) in [
        (
            "absent-document",
            row("supersession", "sup1-absent", &valid()),
            Some("supersession document"),
        ),
        (
            "ace-region",
            row("superseded", ace[0], &valid()),
            Some("superseded region"),
        ),
        (
            "absent-region",
            row("superseded", "SUP1-ABSENT", &valid()),
            Some("superseded region"),
        ),
        (
            "valid",
            format!(
                "{}\n{}\n{}",
                row("supersession", doc, &valid()),
                row("superseded", uncovered, &valid()),
                row("superseded", restated, &valid())
            ),
            None,
        ),
    ] {
        write(tree.join(path), table(&["unit\thour\thour".into(), new]));
        let got = command_env(&program(), &tree, &["check"], b"", &[("SWIPL", &stop)]);
        if let Some(why) = expected {
            checks.require(
                got.rc == 1
                    && got.violation() == format!("ckc: supersession: {path}: {why} at row 2"),
                format!(
                    "{label}: expected supersession/{why} row 2; {}",
                    got.describe()
                ),
            );
        } else {
            checks.require(
                got.stdout
                    .contains(&format!("ckc: vocabulary ok {path} 4 rows\n")),
                format!(
                    "{label}: grammar meter counts both kinds; {}",
                    got.describe()
                ),
            );
            checks.require(
                got.stdout.contains(&format!(
                    "ckc: supersession ok {path} 1 documents 2 passages\n"
                )),
                format!("{label}: cross-ledger meter; {}", got.describe()),
            );
        }
    }
    write(tree.join(path), original);
    let fixture = "tests/green/about--spacing-frame.vocabulary.tsv";
    let mut fixture_table = fs::read_to_string(tree.join(fixture)).unwrap();
    fixture_table.push_str(&format!(
        "{}\n{}\n",
        row("supersession", "absent-document", &valid()),
        row("superseded", "ABSENT-REGION", &valid())
    ));
    write(tree.join(fixture), &fixture_table);
    let got = command_env(&program(), &tree, &["check"], b"", &[("SWIPL", &stop)]);
    checks.require(
        got.stdout.contains("ckc: coverage ok cdc-2022-opioid "),
        format!(
            "fixture exemption reaches the parsed corpus coverage; {}",
            got.describe()
        ),
    );
    checks.require(
        got.stderr.contains("sup1-surface: reached SWI stage"),
        format!(
            "fixture path skips corpus law and reaches SWI stop; {}",
            got.describe()
        ),
    );
    checks.finish("P1 corpus cross-ledger laws + meter (five bounded HEAD-clone checks)");
}
