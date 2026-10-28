// Contract rev-k P1–P3; CLI-only, independent contrast/count/ordinal models.
// CKC_REVK_TEST_BIN selects the pre-fix binary for red replay. Rendered pages map back
// through DQ10 (a later generation) first.
#[path = "../../../tests/frozen-pages/dq10.rs"]
mod dq10;
#[path = "../../../tests/rev-k/ordinal.rs"]
mod ordinal;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

const BEFORE_CSS: &str = include_str!("../../../tests/rev-k/style.before.in");
const LEDGER_HEADER: &str = "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n";
const CSS_EDITS: [(&str, &str); 4] = [
    (
        "main.hl-click pre.prose { color: #9ca3af; }",
        "main.hl-click pre.prose { color: #4b5563; }",
    ),
    (
        "main.hl-click pre.prose .kw { color: #9ca3af; }",
        "main.hl-click pre.prose .kw { color: #4b5563; }",
    ),
    (
        "main.hl-click pre.prose mark:not(.hl-pick) { background: none; text-decoration-color: #9ca3af; }",
        "main.hl-click pre.prose mark:not(.hl-pick) { background: none; text-decoration-color: #4b5563; }",
    ),
    (
        "padding: 0.45rem 0.6rem; border: 1px solid #e5e7eb;",
        "padding: 0.45rem 0.6rem; border: 1px solid #4b5563;",
    ),
];

type Pages = BTreeMap<String, String>;

fn sha(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes.as_ref())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn program() -> PathBuf {
    std::env::var_os("CKC_REVK_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")))
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn write(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn files(base: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(base).unwrap() {
        let path = entry.unwrap().path();
        assert!(!path.is_symlink(), "fixture symlink: {}", path.display());
        if path.is_dir() {
            out.extend(files(&path));
        } else {
            out.push(path);
        }
    }
    out.sort();
    out
}

struct Scratch(PathBuf);
impl Scratch {
    fn new(label: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = root().join(format!(
            "rust/target/rev-k/{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!path.exists(), "scratch collision: {}", path.display());
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn copy_tree(&self, case: &str) -> PathBuf {
        let source = root().join(format!("tests/ui/green/{case}/tree"));
        let tree = self.0.join("tree");
        fs::create_dir_all(&tree).unwrap();
        for path in files(&source) {
            let dest = tree.join(path.strip_prefix(&source).unwrap());
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(path, dest).unwrap();
        }
        let empty = source.parent().unwrap().join("empty-dirs.txt");
        if empty.exists() {
            for dir in read(&empty).lines() {
                fs::create_dir_all(tree.join(dir)).unwrap();
            }
        }
        tree
    }
    fn command(&self, args: &[&str]) -> Output {
        Command::new(program())
            .args(args)
            .current_dir(&self.0)
            .env("GIT_CEILING_DIRECTORIES", &self.0)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("TMPDIR", &self.0)
            .output()
            .unwrap()
    }
    fn get(&self, tree: &Path, route: &str) -> String {
        let out = self.command(&["ui", "request", "GET", route, tree.to_str().unwrap()]);
        success(&out, route);
        let raw = String::from_utf8(out.stdout).unwrap();
        let (headers, body) = raw.split_once("\n\n").expect("HTTP body");
        assert_eq!(
            headers.lines().next(),
            Some("HTTP 200"),
            "{route}: {headers}"
        );
        body.into()
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn success(out: &Output, label: &str) {
    assert!(
        out.status.success() && out.stderr.is_empty(),
        "{label}: rc={:?}; stdout={}; stderr={}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn rendered() -> &'static Pages {
    static PAGES: OnceLock<Pages> = OnceLock::new();
    PAGES.get_or_init(|| {
        let mut pages = Pages::new();
        let mut cases: Vec<_> = fs::read_dir(root().join("tests/ui/green"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.is_dir())
            .collect();
        cases.sort();
        assert!(cases.len() >= 21, "green fixture census");
        for case in cases {
            let name = case.file_name().unwrap().to_str().unwrap();
            let s = Scratch::new(name);
            let tree = s.copy_tree(name);
            let target = s.0.join("pages");
            let out = s.command(&[
                "ui",
                "render",
                target.to_str().unwrap(),
                tree.to_str().unwrap(),
            ]);
            success(&out, name);
            let mut count = 0;
            for path in files(&target) {
                if path.extension().is_some_and(|e| e == "html") {
                    let key = format!(
                        "fixture/{name}/{}",
                        path.strip_prefix(&target).unwrap().display()
                    );
                    assert!(pages.insert(key, dq10::map(&read(&path), false)).is_none());
                    count += 1;
                }
            }
            assert!(count > 0, "{name}: no pages");
        }
        pages
    })
}

fn css(page: &str) -> &str {
    assert_eq!(page.matches("<style>").count(), 1, "stylesheet census");
    page.split_once("<style>\n")
        .unwrap()
        .1
        .split_once("</style>")
        .unwrap()
        .0
}

fn screen(style: &str) -> &str {
    style
        .split_once("@media print {")
        .expect("print boundary")
        .0
}

fn property<'a>(style: &'a str, selector: &str, key: &str) -> Option<&'a str> {
    let mut result = None;
    for line in screen(style).lines() {
        let Some((selectors, tail)) = line.split_once('{') else {
            continue;
        };
        if selectors.split(',').any(|s| s.trim() == selector) {
            let declarations = tail.split_once('}').unwrap().0;
            for declaration in declarations.split(';') {
                if let Some((name, value)) = declaration.split_once(':')
                    && name.trim() == key
                {
                    result = Some(value.trim());
                }
            }
        }
    }
    result
}

fn required<'a>(style: &'a str, selector: &str, key: &str) -> &'a str {
    property(style, selector, key)
        .unwrap_or_else(|| panic!("missing screen rule: {selector} / {key}"))
}

fn luminance(color: &str) -> f64 {
    assert_eq!(color.len(), 7, "opaque six-digit CSS color: {color}");
    assert!(color.starts_with('#'));
    [0.2126, 0.7152, 0.0722]
        .into_iter()
        .enumerate()
        .map(|(i, weight)| {
            let value =
                f64::from(u8::from_str_radix(&color[1 + 2 * i..3 + 2 * i], 16).unwrap()) / 255.0;
            weight
                * if value <= 0.04045 {
                    value / 12.92
                } else {
                    ((value + 0.055) / 1.055).powf(2.4)
                }
        })
        .sum()
}

fn contrast(a: &str, b: &str) -> f64 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

fn corrected_css() -> String {
    let mut expected = BEFORE_CSS.to_owned();
    for (old, new) in CSS_EDITS {
        assert_eq!(expected.matches(old).count(), 1, "CSS edit anchor {old}");
        expected = expected.replacen(old, new, 1);
    }
    expected
}

fn color_failures(style: &str, selector: &str, key: &str, threshold: f64) -> Vec<String> {
    let value = required(style, selector, key);
    let color = value.split_whitespace().last().unwrap();
    let background = required(style, "body", "background");
    assert_eq!(background, "#ffffff");
    let ratio = contrast(color, background);
    let mut bad = Vec::new();
    if ratio < threshold {
        bad.push(format!(
            "{selector} / {key}: {color} on {background} = {ratio:.3}:1 < {threshold}:1"
        ));
    }
    if color != "#4b5563" {
        bad.push(format!("{selector} / {key}: expected #4b5563, got {color}"));
    }
    bad
}

fn every_page_color(selector: &str, key: &str, threshold: f64) {
    let mut bad = BTreeSet::new();
    let mut failed = 0;
    for page in rendered().values() {
        let errors = color_failures(css(page), selector, key, threshold);
        failed += usize::from(!errors.is_empty());
        bad.extend(errors);
    }
    assert!(
        bad.is_empty(),
        "{failed}/{} rendered pages fail:\n{}",
        rendered().len(),
        bad.into_iter().collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn p1_unpicked_text_contrast() {
    every_page_color("main.hl-click pre.prose", "color", 4.5);
}

#[test]
fn p1_unpicked_keyword_contrast() {
    every_page_color("main.hl-click pre.prose .kw", "color", 4.5);
}

#[test]
fn p1_unpicked_underline_contrast() {
    every_page_color(
        "main.hl-click pre.prose mark:not(.hl-pick)",
        "text-decoration-color",
        4.5,
    );
}

#[test]
fn p1_text_field_boundary_contrast() {
    every_page_color("input[type=\"text\"]", "border", 3.0);
}

#[test]
fn p1_textarea_boundary_contrast() {
    every_page_color("textarea", "border", 3.0);
}

fn highlighted_failures(style: &str) -> Vec<String> {
    let foregrounds = [
        required(style, "body", "color"),
        required(style, ".kw", "color"),
        required(style, "main.hl-click pre.prose .kw", "color"),
        required(style, "main.hl-click pre.prose mark.hl-pick", "color"),
    ];
    let mut bad = BTreeSet::new();
    for i in 0..48 {
        let base = property(style, &format!("mark.t{i}"), "background")
            .unwrap_or_else(|| required(style, "mark", "background"));
        let hover = required(
            style,
            &format!("main:has(mark.t{i}:hover) mark.t{i}"),
            "background",
        );
        for background in [base, hover] {
            for foreground in foregrounds {
                let ratio = contrast(foreground, background);
                if ratio < 4.5 {
                    bad.insert(format!(
                        "{foreground} on {background} = {ratio:.3}:1 < 4.5:1"
                    ));
                }
            }
        }
    }
    bad.into_iter().collect()
}

#[test]
fn p1_picked_hover_contrast() {
    let mut bad = BTreeSet::new();
    let mut failed = 0;
    for page in rendered().values() {
        let errors = highlighted_failures(css(page));
        failed += usize::from(!errors.is_empty());
        bad.extend(errors);
    }
    assert!(
        bad.is_empty(),
        "{failed}/{} rendered pages fail picked/hover contrast:\n{}",
        rendered().len(),
        bad.into_iter().collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn p1_stylesheet_changes_only_the_four_cells() {
    let expected = corrected_css();
    let failures: Vec<_> = rendered()
        .iter()
        .filter(|(_, page)| css(page) != expected)
        .map(|(name, _)| name)
        .collect();
    assert!(
        failures.is_empty(),
        "{}/{} stylesheets differ from the four-cell contract; first={:?}",
        failures.len(),
        rendered().len(),
        failures.first()
    );
}

#[test]
fn p1_contrast_oracle_rejects_each_isolated_defect() {
    assert_eq!(contrast("#000000", "#ffffff"), 21.0);
    assert_eq!(contrast("#123456", "#123456"), 1.0);
    let fixed = corrected_css();
    let slots = [
        ("main.hl-click pre.prose", "color", 4.5),
        ("main.hl-click pre.prose .kw", "color", 4.5),
        (
            "main.hl-click pre.prose mark:not(.hl-pick)",
            "text-decoration-color",
            4.5,
        ),
        ("input[type=\"text\"]", "border", 3.0),
    ];
    assert!(highlighted_failures(&fixed).is_empty());
    for (i, (old, new)) in CSS_EDITS.into_iter().enumerate() {
        let planted = fixed.replacen(new, old, 1);
        for (j, (selector, key, threshold)) in slots.into_iter().enumerate() {
            assert!(color_failures(&fixed, selector, key, threshold).is_empty());
            assert_eq!(
                !color_failures(&planted, selector, key, threshold).is_empty(),
                i == j,
                "plant={i}, probe={j}"
            );
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Tally {
    n: usize,
    k: usize,
    m: usize,
}
impl Tally {
    fn key(self) -> String {
        format!("n{}-k{}-m{}", self.n, self.k, self.m)
    }
    fn sentence(self) -> String {
        let documents = if self.k == 1 { "document" } else { "documents" };
        if self.n == 0 {
            format!(
                "No decisions are recorded for the {} {documents} in this guideline.",
                self.k
            )
        } else {
            let decisions = if self.n == 1 { "decision" } else { "decisions" };
            format!(
                "Reviewers recorded {} {decisions} on {} of {} {documents}.",
                self.n, self.m, self.k
            )
        }
    }
}

fn tally_cases() -> Vec<Tally> {
    let mut cases = Vec::new();
    for n in [0, 1, 2, 3, 10, 11, 21] {
        for k in [1, 2, 3, 11] {
            if n == 0 {
                cases.push(Tally { n, k, m: 0 });
            } else {
                for m in 1..=n.min(k) {
                    cases.push(Tally { n, k, m });
                }
            }
        }
    }
    cases
}

fn tally_tree(s: &Scratch, tally: Tally, pl: &str) -> PathBuf {
    assert!(tally.m <= tally.n.min(tally.k) && (tally.m == 0) == (tally.n == 0));
    let tree = s.copy_tree("header-only-ledger");
    let g = tree.join("guidelines/alpha");
    fs::remove_dir_all(g.join("ace")).unwrap();
    fs::remove_dir_all(g.join("pl")).unwrap();
    let old_coverage = read(&g.join("coverage.tsv"));
    let mut coverage: String = old_coverage
        .lines()
        .filter(|line| line.starts_with('#'))
        .map(|line| format!("{line}\n"))
        .collect();
    let mut evidence = format!(
        "Fixture evidence\nRegion authority: identify the {} payloads below\n",
        tally.k
    );
    let old_manifest = read(&g.join("audit/review-manifest.tsv"));
    let mut manifest: String = old_manifest
        .lines()
        .filter(|line| line.starts_with('#'))
        .map(|line| format!("{line}\n"))
        .collect();
    let digest_fields = old_manifest
        .lines()
        .find(|line| !line.starts_with('#'))
        .unwrap()
        .split_once('\t')
        .unwrap()
        .1;
    let ace =
        read(&root().join("tests/ui/green/header-only-ledger/tree/guidelines/alpha/ace/a.ace"));
    for i in 1..=tally.k {
        let id = format!("d{i}");
        write(&g.join(format!("ace/{id}.ace")), &ace);
        write(&g.join(format!("pl/{id}.pl")), pl);
        coverage.push_str(&format!(
            "r{i}\tsource/evidence.txt\tp{i}\tFixture section\tace({id})\n"
        ));
        evidence.push_str(&format!(
            "\n[r{i} | p{i} | Fixture section]\nSource payload for {id}.\n"
        ));
        let components: Vec<_> = digest_fields.split('\t').take(4).collect();
        let block = format!(
            "bundle v2 {id}\nace {}\ncoverage {}\npayload {}\nclauses {}\n",
            components[0], components[1], components[2], components[3]
        );
        manifest.push_str(&format!(
            "{id}\t{}\t{}\n",
            components.join("\t"),
            sha(block.as_bytes())
        ));
    }
    let mut rows: Vec<_> = manifest
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect();
    rows.sort();
    manifest = old_manifest
        .lines()
        .filter(|line| line.starts_with('#'))
        .chain(rows)
        .map(|line| format!("{line}\n"))
        .collect();
    let mut ledger = LEDGER_HEADER.to_owned();
    for i in 0..tally.n {
        ledger.push_str(&format!(
            "d{}\t{}\t\tapproved\tReader {}\t2026-01-01T00:00:00Z\tCount control.\n",
            i % tally.m + 1,
            "0".repeat(64),
            i + 1
        ));
    }
    let mut rows: Vec<_> = ledger.lines().collect();
    rows.sort();
    ledger = rows.join("\n") + "\n";
    write(&g.join("coverage.tsv"), &coverage);
    write(&g.join("source/evidence.txt"), &evidence);
    write(&g.join("audit/review-manifest.tsv"), &manifest);
    write(&g.join("audit/adjudication.tsv"), &ledger);
    tree
}

fn tallies() -> &'static Pages {
    static PAGES: OnceLock<Pages> = OnceLock::new();
    PAGES.get_or_init(|| {
        let mut pages = Pages::new();
        for tally in tally_cases() {
            let s = Scratch::new(&tally.key());
            let tree = tally_tree(&s, tally, "fixture_fact('d1').\n");
            for route in ["index", "records"] {
                pages.insert(
                    format!("tally/{}/{route}", tally.key()),
                    dq10::map(&s.get(&tree, &format!("/g/alpha/{route}.html")), false),
                );
            }
        }
        pages
    })
}

fn sentence_in(page: &str) -> &str {
    let prefix = if page.contains("<p>Reviewers recorded ") {
        "<p>Reviewers recorded "
    } else {
        "<p>No decisions are recorded for "
    };
    assert_eq!(page.matches(prefix).count(), 1, "tally paragraph census");
    let start = page.find(prefix).unwrap() + 3;
    let end = page[start..].find('.').unwrap() + start + 1;
    &page[start..end]
}

fn check_tallies(route: &str) {
    let mut bad = Vec::new();
    for tally in tally_cases() {
        let key = format!("tally/{}/{route}", tally.key());
        let actual = sentence_in(&tallies()[&key]);
        let expected = tally.sentence();
        if actual != expected {
            bad.push(format!("{key}: expected {expected:?}; got {actual:?}"));
        }
    }
    assert!(
        bad.is_empty(),
        "{} mismatches over {} generated tallies:\n{}",
        bad.len(),
        tally_cases().len(),
        bad.join("\n")
    );
}

#[test]
fn p2_guideline_count_model() {
    check_tallies("index");
}

#[test]
fn p2_records_count_model() {
    check_tallies("records");
}

const LINE_CASES: [(&str, &str, usize); 7] = [
    ("empty", "", 0),
    ("one", "fixture_fact('d1').\n", 1),
    ("two", "fixture_fact('d1').\nfixture_fact('d2').\n", 2),
    ("one-unterminated", "fixture_fact('d1').", 1),
    (
        "two-unterminated",
        "fixture_fact('d1').\nfixture_fact('d2').",
        2,
    ),
    ("one-blank", "\n", 1),
    ("two-blank", "\n\n", 2),
];

fn line_pages() -> &'static Pages {
    static PAGES: OnceLock<Pages> = OnceLock::new();
    PAGES.get_or_init(|| {
        LINE_CASES
            .into_iter()
            .map(|(label, pl, _)| {
                let s = Scratch::new(label);
                let tree = s.copy_tree("header-only-ledger");
                write(&tree.join("guidelines/alpha/pl/a.pl"), pl);
                (
                    format!("line/{label}"),
                    dq10::map(&s.get(&tree, "/g/alpha/doc/a.html"), false),
                )
            })
            .collect()
    })
}

#[test]
fn p2_compiled_line_count_model() {
    let mut bad = Vec::new();
    for (label, _, n) in LINE_CASES {
        let page = &line_pages()[&format!("line/{label}")];
        let text = page
            .split_once("<summary>Compiled Prolog (")
            .unwrap()
            .1
            .split_once("</summary>")
            .unwrap()
            .0;
        let noun = if n == 1 { "line" } else { "lines" };
        let expected = format!("{n} {noun})");
        if text != expected {
            bad.push(format!(
                "{label}: expected Compiled Prolog ({expected}; got Compiled Prolog ({text}"
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

fn before_form(page: &str) -> String {
    let style = css(page);
    let (screen, print) = style.split_once("@media print {").unwrap();
    let mut old_style = screen.to_owned();
    for (old, new) in CSS_EDITS {
        old_style = old_style.replace(new, old);
    }
    old_style.push_str("@media print {");
    old_style.push_str(print);
    let mut result = page.replacen(style, &old_style, 1);
    for prefix in [
        "<p>Reviewers recorded ",
        "<p>No decisions are recorded for ",
    ] {
        if let Some(start) = result.find(prefix) {
            let end = start + result[start..].find("</p>").unwrap();
            let before = result[start..end]
                .replace("recorded 1 decision on ", "recorded 1 decisions on ")
                .replace(" of 1 document.", " of 1 documents.")
                .replace(
                    "for the 1 document in this guideline.",
                    "for the 1 documents in this guideline.",
                );
            result.replace_range(start..end, &before);
        }
    }
    result.replace(
        "<summary>Compiled Prolog (1 line)</summary>",
        "<summary>Compiled Prolog (1 lines)</summary>",
    )
}

#[test]
fn p2_preservation_oracle_keeps_unrelated_bytes() {
    let before = format!(
        "<style>\n{BEFORE_CSS}</style><p>Reviewers recorded 1 decisions on 1 of 1 documents.</p><p>No decisions are recorded for the 1 documents in this guideline.</p><summary>Compiled Prolog (1 lines)</summary><pre>Reviewers recorded 1 decision on 1 of 1 document. Compiled Prolog (1 line)</pre><footer>outside-sentinel</footer>"
    );
    let after = before
        .replace(BEFORE_CSS, &corrected_css())
        .replace(
            "<p>Reviewers recorded 1 decisions on 1 of 1 documents.</p>",
            "<p>Reviewers recorded 1 decision on 1 of 1 document.</p>",
        )
        .replace(
            "<p>No decisions are recorded for the 1 documents in this guideline.</p>",
            "<p>No decisions are recorded for the 1 document in this guideline.</p>",
        )
        .replace(
            "<summary>Compiled Prolog (1 lines)</summary>",
            "<summary>Compiled Prolog (1 line)</summary>",
        );
    assert_eq!(before_form(&after), before);
    for (old, new) in [
        ("outside-sentinel", "lost-sentinel"),
        (
            "border-bottom: 1px solid #e5e7eb",
            "border-bottom: 1px solid #000000",
        ),
        (
            "Compiled Prolog (1 line)</pre>",
            "Compiled Prolog (1 lines)</pre>",
        ),
    ] {
        assert!(after.contains(old), "plant anchor: {old}");
        assert_ne!(
            before_form(&after.replacen(old, new, 1)),
            before,
            "plant: {old}"
        );
    }
    let (screen, print) = after.split_once("@media print {").unwrap();
    let print_plant = format!(
        "{screen}@media print {{{}",
        print.replacen("#4b5563", "#000000", 1)
    );
    assert_ne!(before_form(&print_plant), before);
}

#[test]
fn p2_unchanged_bytes() {
    let pins = read(&root().join("tests/rev-k/baseline.sha256"));
    let pins: BTreeMap<_, _> = pins
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let (digest, name) = line.split_once("  ").unwrap();
            (name.to_owned(), digest.to_owned())
        })
        .collect();
    let pages: Pages = rendered()
        .iter()
        .chain(tallies())
        .chain(line_pages())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert_eq!(
        pins.keys().collect::<Vec<_>>(),
        pages.keys().collect::<Vec<_>>(),
        "baseline case census"
    );
    let mut bad = Vec::new();
    for (name, page) in pages {
        let actual = sha(before_form(&page).as_bytes());
        if actual != pins[&name] {
            bad.push(format!("{name}: non-contract bytes changed"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

fn certify(ace: &Path, dump: &Path, pl: &Path) -> Output {
    let s = Scratch::new("certify");
    s.command(&[
        "certify-one",
        "doc",
        "fx-doc",
        ace.to_str().unwrap(),
        "-",
        dump.to_str().unwrap(),
        pl.to_str().unwrap(),
    ])
}

fn rejection(case: &str, ace: &str, sentence: usize, variant: usize) {
    let fixtures = root().join("tests/rev-k/certify");
    let out = certify(
        &root().join(ace),
        &fixtures.join(format!("{case}.dump.in")),
        &fixtures.join(format!("{case}.pl.in")),
    );
    let expected =
        format!("ckc: certify: fx-doc: nonground_obligation({sentence},variant({variant})).\n");
    assert_eq!(
        (
            out.status.code(),
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap()
        ),
        (Some(1), String::new(), expected),
        "{case}"
    );
}

#[test]
fn p3_variant_one() {
    rejection("one", "tests/certify-one/doc/ace.txt.in", 1, 1);
}

#[test]
fn p3_variant_two() {
    rejection("two-second", "tests/rev-k/certify/two.ace.in", 1, 2);
}

#[test]
fn p3_first_of_two_variants() {
    rejection("two-first", "tests/rev-k/certify/two.ace.in", 1, 1);
}

#[test]
fn p3_later_sentence() {
    rejection("later", "tests/rev-k/certify/later.ace.in", 2, 1);
}

#[test]
fn p3_first_nonground_wins() {
    rejection("two-both", "tests/rev-k/certify/two.ace.in", 1, 1);
}

#[test]
fn p3_ground_control() {
    let base = root().join("tests/certify-one/doc");
    let out = certify(
        &base.join("ace.txt.in"),
        &base.join("dump.txt.in"),
        &base.join("pl.txt.in"),
    );
    success(&out, "single ground control");
    assert_eq!(out.stdout, fs::read(base.join("stdout.expect")).unwrap());
    let fx = root().join("tests/rev-k/certify");
    let out = certify(
        &fx.join("two.ace.in"),
        &fx.join("two-ground.dump.in"),
        &fx.join("two-ground.pl.in"),
    );
    success(&out, "two ground controls");
    let proof = String::from_utf8(out.stdout).unwrap();
    assert_eq!(proof.lines().count(), 2);
    for (i, line) in proof.lines().enumerate() {
        assert!(line.starts_with(&format!(
            "'$guideline_proof'('fx-doc',1,variant({}),",
            i + 1
        )));
    }
}

#[test]
fn p3_clause_mismatch_first() {
    let fx = root().join("tests/rev-k/certify");
    let s = Scratch::new("clause-mismatch");
    for (case, ace) in [
        ("one", root().join("tests/certify-one/doc/ace.txt.in")),
        ("two-both", fx.join("two.ace.in")),
        ("later", fx.join("later.ace.in")),
    ] {
        let pl = read(&fx.join(format!("{case}.pl.in")));
        assert!(pl.contains(",person,countable)"));
        let path = s.0.join(format!("{case}.pl.in"));
        write(
            &path,
            &pl.replacen(",person,countable)", ",persom,countable)", 1),
        );
        let out = certify(&ace, &fx.join(format!("{case}.dump.in")), &path);
        assert_eq!(
            (
                out.status.code(),
                out.stdout.as_slice(),
                out.stderr.as_slice()
            ),
            (
                Some(1),
                &b""[..],
                &b"ckc: certify: fx-doc: clauses(1).\n"[..]
            ),
            "{case}"
        );
    }
}
