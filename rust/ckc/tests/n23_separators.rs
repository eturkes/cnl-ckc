// N23 D1–D8: contract-only render/request properties over copied fixture trees.
// CKC_UI_TEST_BIN selects the executable for red replay.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

const VIEWPORT: &str = "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">";
const NARROW: &str =
    "@media screen and (max-width: 40rem) { table { display: block; overflow-x: auto; } }";
const NAV_SPACING: &str = "nav.docnav a + a { margin-left: 1.5rem; }";
const PROVENANCE_SPACING: &str = "p.provenance span + a { margin-left: 0.75rem; }";
const TITLE_SUFFIX: &str = " — cnl-ckc reviewer";
const GIT_ENV: [(&str, &str); 9] = [
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_CONFIG_SYSTEM", "/dev/null"),
    ("GIT_DEFAULT_HASH", "sha1"),
    ("GIT_AUTHOR_NAME", "fixture"),
    ("GIT_AUTHOR_EMAIL", "fixture@localhost"),
    ("GIT_COMMITTER_NAME", "fixture"),
    ("GIT_COMMITTER_EMAIL", "fixture@localhost"),
    ("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+00:00"),
    ("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+00:00"),
];
static NEXT_TMP: AtomicU64 = AtomicU64::new(0);
static FRAMED: OnceLock<BTreeMap<String, String>> = OnceLock::new();
type Files = BTreeMap<PathBuf, Vec<u8>>;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn program() -> PathBuf {
    std::env::var_os("CKC_UI_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")))
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn write(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

fn files(base: &Path) -> Files {
    fn walk(base: &Path, here: &Path, out: &mut Files) {
        if !here.is_dir() {
            return;
        }
        for entry in fs::read_dir(here).unwrap() {
            let path = entry.unwrap().path();
            assert!(!path.is_symlink(), "fixture symlink: {}", path.display());
            if path.file_name().unwrap() == ".git" {
                continue;
            }
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                out.insert(
                    path.strip_prefix(base).unwrap().to_owned(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut out = Files::new();
    walk(base, base, &mut out);
    out
}

fn lines(path: &Path) -> Vec<String> {
    if !path.exists() {
        return Vec::new();
    }
    read(path)
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

struct Fixture {
    scratch: PathBuf,
    tree: PathBuf,
    case: PathBuf,
}
impl Fixture {
    fn new(color: &str, name: &str) -> Self {
        let base = root().join("rust/target/n23-separators");
        fs::create_dir_all(&base).unwrap();
        let scratch = loop {
            let path = base.join(format!(
                "{}-{}",
                std::process::id(),
                NEXT_TMP.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => break path,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
                Err(e) => panic!("fixture scratch: {e}"),
            }
        };
        let fx = Self {
            tree: scratch.join("tree"),
            scratch,
            case: root().join(format!("tests/ui/{color}/{name}")),
        };
        fs::create_dir_all(&fx.tree).unwrap();
        let input = files(&fx.case.join("tree"));
        let order = if fx.case.join("tree-order.txt").is_file() {
            lines(&fx.case.join("tree-order.txt"))
                .into_iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>()
        } else {
            input.keys().cloned().collect()
        };
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(sorted, input.keys().cloned().collect::<Vec<_>>());
        for path in order {
            write(&fx.tree.join(&path), &input[&path]);
        }
        for path in lines(&fx.case.join("empty-dirs.txt")) {
            fs::create_dir_all(fx.tree.join(path)).unwrap();
        }
        if fx.case.join("worktree").is_dir() {
            for args in [
                vec!["init", "-q", "-b", "main"],
                vec!["add", "-A"],
                vec!["commit", "-q", "-m", "fixture corpus"],
            ] {
                let result = Command::new("git")
                    .args(&args)
                    .current_dir(&fx.tree)
                    .envs(GIT_ENV)
                    .output()
                    .unwrap();
                assert!(
                    result.status.success(),
                    "fixture git {args:?}: {:?}",
                    result.stderr
                );
            }
            for (path, bytes) in files(&fx.case.join("worktree")) {
                write(&fx.tree.join(path), &bytes);
            }
        }
        fx
    }

    fn run(&self, args: &[String]) -> Output {
        Command::new(program())
            .args(args)
            .current_dir(&self.scratch)
            .envs(GIT_ENV)
            .env("GIT_CEILING_DIRECTORIES", &self.scratch)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("TMPDIR", &self.scratch)
            .output()
            .unwrap()
    }

    fn ui(&self, args: &[String]) -> Output {
        let mut argv = vec!["ui".into()];
        argv.extend_from_slice(args);
        self.run(&argv)
    }

    fn guideline(&self) -> PathBuf {
        self.tree.join("guidelines/alpha")
    }

    fn edit_coverage(&self, mut edit: impl FnMut(&mut Vec<String>)) {
        let path = self.guideline().join("coverage.tsv");
        let content = read(&path)
            .lines()
            .map(|line| {
                if line.starts_with('#') {
                    line.to_owned()
                } else {
                    let mut row = line.split('\t').map(str::to_owned).collect::<Vec<_>>();
                    assert_eq!(row.len(), 5);
                    edit(&mut row);
                    row.join("\t")
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        write(&path, format!("{content}\n").as_bytes());
    }

    fn replace(&self, relative: &str, old: &str, new: &str) {
        let path = self.guideline().join(relative);
        let input = read(&path);
        assert!(
            input.contains(old),
            "fixture replacement: {relative}: {old:?}"
        );
        write(&path, input.replace(old, new).as_bytes());
    }

    fn repin(&self) {
        // Only fixture custody is derived by the candidate; expected page bytes are not.
        let output = self.run(&[
            "derive-review-manifest".into(),
            self.guideline().to_str().unwrap().into(),
        ]);
        assert_eq!(
            output.status.code(),
            Some(0),
            "fixture manifest: stdout={:?}, stderr={:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        write(
            &self.guideline().join("audit/review-manifest.tsv"),
            &output.stdout,
        );
    }

    fn render(&self) -> Files {
        let out = self.scratch.join("out");
        let output = self.ui(&[
            "render".into(),
            out.to_str().unwrap().into(),
            self.tree.to_str().unwrap().into(),
        ]);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{} render: {}",
            self.case.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        let pages = files(&out);
        assert!(pages.contains_key(Path::new("index.html")));
        pages
    }

    fn check(&self) {
        let output = self.ui(&["check".into(), self.tree.to_str().unwrap().into()]);
        assert_eq!(
            output.status.code(),
            Some(0),
            "ui check: {:?}",
            output.stderr
        );
        assert!(output.stderr.is_empty());
        assert!(output.stdout.ends_with(b"ui: check ok\n"));
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.scratch) {
            eprintln!("N23 cleanup {}: {e}", self.scratch.display());
        }
    }
}

#[derive(Default)]
struct Checks {
    count: usize,
    failures: Vec<String>,
}
impl Checks {
    fn equal(&mut self, actual: &str, expected: &str, label: &str) {
        self.count += 1;
        if actual != expected {
            self.failures
                .push(format!("{label}: expected {expected:?}; actual {actual:?}"));
        }
    }

    fn require(&mut self, condition: bool, label: &str) {
        self.count += 1;
        if !condition {
            self.failures.push(label.into());
        }
    }

    fn finish(self, label: &str) {
        assert!(
            self.failures.is_empty(),
            "{label}: {} failures / {} checks:\n{}",
            self.failures.len(),
            self.count,
            self.failures.join("\n")
        );
    }
}

fn text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn block<'a>(page: &'a str, open: &str, close: &str) -> &'a str {
    page.split_once(open)
        .and_then(|(_, rest)| rest.split_once(close))
        .unwrap_or_else(|| panic!("page lacks {open:?} … {close:?}"))
        .0
}

fn page<'a>(pages: &'a Files, path: &str) -> &'a str {
    std::str::from_utf8(pages.get(Path::new(path)).unwrap()).unwrap()
}

fn doc_title(page: &str) -> &str {
    block(page, "<h2>", " <span class=\"chip ")
}

fn frame_title(page: &str) -> &str {
    if page.split_once("<h2>").is_some_and(|(_, rest)| {
        rest.split_once("</h2>")
            .is_some_and(|(heading, _)| heading.contains(" <span class=\"chip "))
    }) {
        doc_title(page)
    } else {
        block(page, "<h1>", "</h1>")
    }
}

fn response(raw: &[u8]) -> (&str, &str) {
    std::str::from_utf8(raw)
        .unwrap()
        .split_once("\n\n")
        .expect("HTTP boundary")
}

fn framed_pages() -> &'static BTreeMap<String, String> {
    FRAMED.get_or_init(|| {
        let mut pages = BTreeMap::new();
        let mut statuses = BTreeSet::new();
        let mut rendered = 0;
        let mut requested = 0;
        for color in ["green", "red"] {
            let mut cases = fs::read_dir(root().join("tests/ui").join(color))
                .unwrap()
                .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                .collect::<Vec<_>>();
            cases.sort();
            assert!(!cases.is_empty());
            for name in cases {
                if color == "green" {
                    let fx = Fixture::new(color, &name);
                    for (path, bytes) in fx.render() {
                        if path.extension().is_some_and(|ext| ext == "html")
                            && !path.components().any(|part| part.as_os_str() == "source")
                        {
                            let body = String::from_utf8(bytes).unwrap();
                            assert!(body.starts_with("<!doctype html>"));
                            pages.insert(format!("render/{color}/{name}/{}", path.display()), body);
                            rendered += 1;
                        }
                    }
                }
                let case = root().join(format!("tests/ui/{color}/{name}"));
                let rows = lines(&case.join("case.tsv"));
                if !rows.iter().any(|row| row.starts_with("request ")) {
                    continue;
                }
                let fx = Fixture::new(color, &name);
                for (i, row) in rows.iter().enumerate() {
                    let fields = row.split('\t').collect::<Vec<_>>();
                    assert_eq!(fields.len(), 4);
                    if !fields[0].starts_with("request ") {
                        continue;
                    }
                    let args = fields[0]
                        .split(' ')
                        .map(|arg| match arg {
                            "@TREE@" => fx.tree.to_str().unwrap().to_owned(),
                            other => other.to_owned(),
                        })
                        .collect::<Vec<_>>();
                    let output = fx.ui(&args);
                    let label = format!("request/{color}/{name}/{i}");
                    assert_eq!(
                        output.status.code(),
                        Some(fields[1].parse().unwrap()),
                        "{label}"
                    );
                    if fields[3] == "-" {
                        assert!(output.stdout.is_empty(), "{label}: expected empty stdout");
                        continue;
                    }
                    let expected = fs::read(case.join(fields[3])).unwrap();
                    let (expected_head, expected_body) = response(&expected);
                    let (head, body) = response(&output.stdout);
                    assert_eq!(head.lines().next(), expected_head.lines().next(), "{label}");
                    if expected_body.starts_with("<!doctype html>") {
                        assert!(body.starts_with("<!doctype html>"), "{label}");
                        statuses.insert(head.lines().next().unwrap().to_owned());
                        pages.insert(label, body.into());
                        requested += 1;
                    }
                }
            }
        }
        for required in ["200", "303", "400", "403", "404", "405", "409", "500"] {
            assert!(
                statuses.contains(&format!("HTTP {required}")),
                "missing HTTP {required}"
            );
        }
        assert!(rendered > 0 && requested > 0);
        eprintln!(
            "N23 frames: {rendered} rendered pages, {requested} response bodies; {statuses:?}"
        );
        pages
    })
}

#[test]
fn u1_section_paths_escape_once_and_keep_normalization() {
    let mut checks = Checks::default();
    let fx = Fixture::new("green", "basic");
    fx.edit_coverage(|row| {
        row[3] = match row[4].as_str() {
            "ace(a-10)" | "ace(a-2)" => "BOX 4 > Rec 1 > Guiding principle 1".into(),
            "ace(z)" => "  Rec 2 > > BOX 4 > Tail  ".into(),
            _ => row[3].clone(),
        };
        row[0] = match row[0].as_str() {
            "r1" => "S19-12".into(),
            "r2" => "S20-2".into(),
            _ => row[0].clone(),
        };
        if row[4] == "restates(r2)" {
            row[4] = "restates(S20-2)".into();
        }
    });
    fx.replace("source/evidence.txt", "[r1 |", "[S19-12 |");
    fx.replace("source/evidence.txt", "[r2 |", "[S20-2 |");
    fx.repin();
    let output = fx.render();
    for (doc, title) in [
        ("a-10", "Rec 1 &gt; Guiding principle 1, page 1, passage 12"),
        ("a-2", "Rec 1 &gt; Guiding principle 1, page 1, passage 2"),
        ("z", "Recommendation 2 &gt; BOX 4 &gt; Tail"),
    ] {
        checks.equal(
            doc_title(page(&output, &format!("g/alpha/doc/{doc}.html"))),
            title,
            doc,
        );
        checks.require(
            page(&output, "g/alpha/index.html")
                .contains(&format!("<a href=\"doc/{doc}.html\">{title}</a>")),
            &format!("{doc}: guideline link uses the same normalized title"),
        );
    }
    let basic = Fixture::new("green", "basic").render();
    for (doc, region) in [("a-10", "r1"), ("a-2", "r2"), ("z", "r10")] {
        checks.equal(
            doc_title(page(&basic, &format!("g/alpha/doc/{doc}.html"))),
            &format!("Fixture section ({region})"),
            "region disambiguator",
        );
    }
    let atoms = ["Alpha", "Second & <part", "漢字", "\"quoted\" 'text'"];
    for length in [1, 2, 3, 4, 8, 16] {
        for offset in 0..atoms.len() {
            let parts = (0..length)
                .map(|i| atoms[(i + offset) % atoms.len()])
                .collect::<Vec<_>>();
            let section = parts.join(" > ");
            let expected = parts
                .iter()
                .map(|part| text(part))
                .collect::<Vec<_>>()
                .join(" &gt; ");
            let fx = Fixture::new("green", "basic");
            fx.edit_coverage(|row| {
                if row[4] == "ace(a-10)" {
                    row[3] = section.clone();
                }
            });
            fx.repin();
            let output = fx.render();
            let document = page(&output, "g/alpha/doc/a-10.html");
            let label = format!("section length={length}, rotation={offset}");
            checks.equal(doc_title(document), &expected, &label);
            checks.equal(
                block(document, "<title>", "</title>"),
                &expected,
                &format!("{label} title"),
            );
            checks.require(
                page(&output, "g/alpha/index.html")
                    .contains(&format!("<a href=\"doc/a-10.html\">{expected}</a>")),
                &format!("{label}: guideline link uses the same escaped title"),
            );
        }
    }
    checks.finish("D1/D8 sections");
}

#[test]
fn u2_every_framed_page_and_response_has_its_exact_title() {
    let mut checks = Checks::default();
    let mut kinds = BTreeSet::new();
    for (label, page) in framed_pages() {
        let expected = frame_title(page);
        checks.equal(block(page, "<title>", "</title>"), expected, label);
        checks.require(
            page.matches("<title>").count() == 1,
            &format!("{label}: one title"),
        );
        kinds.insert(if label.contains("/doc/") {
            "document"
        } else if label.ends_with("records.html") {
            "records"
        } else if label.ends_with("/g/alpha/index.html") {
            "guideline"
        } else if label.starts_with("request/") {
            "response"
        } else {
            "index"
        });
    }
    assert_eq!(
        kinds,
        BTreeSet::from(["document", "guideline", "index", "records", "response"])
    );
    checks.finish("D2 exact frame titles");
}

#[test]
fn u3_not_covered_labels_use_colons_for_every_class() {
    let mut checks = Checks::default();
    for class in [
        "heading",
        "process",
        "external",
        "aim",
        "descriptive",
        "notice",
        "inexpressible",
    ] {
        for reason in ["fixture heading row", "quoted · dash — remains", "<&\"'漢"] {
            let fx = Fixture::new("green", "basic");
            fx.edit_coverage(|row| {
                if row[0] == "r21" {
                    row[4] = format!("uncovered({class}: {reason})");
                }
            });
            let output = fx.render();
            let guideline = page(&output, "g/alpha/index.html");
            let expected = format!(
                "<tr><td>r21</td><td>Not covered: {}</td><td>Fixture section</td></tr>",
                text(reason)
            );
            let actual = guideline
                .lines()
                .find(|line| line.starts_with("<tr><td>r21</td>"))
                .unwrap();
            checks.equal(actual, &expected, &format!("uncovered {class}: {reason:?}"));
            checks.require(
                !guideline.contains("Not covered — "),
                "D3 generated label absent",
            );
        }
    }
    checks.finish("D3 not-covered copy");
}

fn provenance_line(document: &str) -> &str {
    document
        .split_once("</h2>\n")
        .unwrap()
        .1
        .lines()
        .next()
        .unwrap()
}

#[test]
fn u4_all_four_provenance_branches_have_exact_markup() {
    let mut checks = Checks::default();
    let basic = Fixture::new("green", "basic").render();
    let both = page(&basic, "g/alpha/doc/a-10.html");
    checks.equal(
        provenance_line(both),
        "<p class=\"provenance\"><span>Passage r1</span> <a href=\"../source/evidence.txt\">Source text</a></p>",
        "region + source",
    );
    checks.require(
        both.contains("<nav class=\"crumbs\"><a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">alpha</a> / r1</nav>"),
        "D8 breadcrumb bytes and destinations",
    );
    let hostile = Fixture::new("green", "hostile").render();
    checks.equal(
        provenance_line(page(&hostile, "g/alpha/doc/m.html")),
        "<p class=\"provenance\"><span>Passage region-&lt;&amp;&gt;\"'%?#漢😀العربية</span></p>",
        "region only: escaped text",
    );
    for source in [true, false] {
        let fx = Fixture::new("green", "locatorless-boundary");
        fx.edit_coverage(|row| {
            if row[0] == "r1" {
                row[0].clear();
            }
            if !source {
                row[1] = "source/ordinal<.txt".into();
            }
        });
        if !source {
            fs::rename(
                fx.guideline().join("source/ordinal.txt"),
                fx.guideline().join("source/ordinal<.txt"),
            )
            .unwrap();
        }
        // UI intake accepts these boundary trees with their committed manifests;
        // the separate manifest-derivation command requires a nonempty region id.
        let output = fx.render();
        fx.check();
        let document = page(&output, "g/alpha/doc/a.html");
        let (label, expected) = if source {
            (
                "source only",
                "<p class=\"provenance\"><a href=\"../source/ordinal.txt\">Source text</a></p>",
            )
        } else {
            (
                "neither",
                "<p>No decision is recorded. <a href=\"../records.html\">All decision records</a></p>",
            )
        };
        checks.equal(provenance_line(document), expected, label);
        checks.require(
            document.matches("<p class=\"provenance\">").count() == usize::from(source),
            &format!("{label}: provenance paragraph count"),
        );
        checks.require(
            !document.contains("<span>Passage </span>"),
            "empty region emits no span",
        );
    }
    checks.finish("D4 provenance");
}

#[test]
fn u5_nav_links_keep_destinations_and_use_one_space() {
    let docs = ["a-10", "a-2", "z"];
    let mut checks = Checks::default();
    let mut link_counts = BTreeSet::new();
    for mask in 1..(1 << docs.len()) {
        let fx = Fixture::new("green", "basic");
        let kept = docs
            .iter()
            .enumerate()
            .filter_map(|(i, doc)| ((mask & (1 << i)) != 0).then_some(*doc))
            .collect::<Vec<_>>();
        for doc in docs.iter().filter(|doc| !kept.contains(doc)) {
            fs::remove_file(fx.guideline().join(format!("ace/{doc}.ace"))).unwrap();
            fs::remove_file(fx.guideline().join(format!("pl/{doc}.pl"))).unwrap();
            fx.edit_coverage(|row| {
                if row[4] == format!("ace({doc})") {
                    row[4] = "uncovered(heading: fixture removed document)".into();
                }
            });
        }
        fx.repin();
        let output = fx.render();
        for (i, doc) in kept.iter().enumerate() {
            let mut links = Vec::new();
            if i > 0 {
                links.push(format!(
                    "<a href=\"{}.html\">Previous document</a>",
                    kept[i - 1]
                ));
            }
            links.push("<a href=\"../index.html\">Guideline index</a>".into());
            if i + 1 < kept.len() {
                links.push(format!(
                    "<a href=\"{}.html\">Next document</a>",
                    kept[i + 1]
                ));
            }
            link_counts.insert(links.len());
            let document = page(&output, &format!("g/alpha/doc/{doc}.html"));
            checks.equal(
                block(document, "<nav class=\"docnav\">", "</nav>"),
                &links.join(" "),
                &format!("nav {kept:?}/{doc}"),
            );
        }
    }
    assert_eq!(link_counts, BTreeSet::from([1, 2, 3]));
    checks.finish("D5 nav join property");
}

#[test]
fn u6_spacing_rules_are_unique_and_n22_print_rules_stay() {
    let mut checks = Checks::default();
    for (label, page) in framed_pages() {
        let css = block(page, "<style>\n", "</style>");
        for rule in [NAV_SPACING, PROVENANCE_SPACING] {
            checks.require(
                css.matches(rule).count() == 1,
                &format!("{label}: once: {rule}"),
            );
        }
        let nav_rule = css
            .lines()
            .find(|line| line.starts_with("nav.docnav { "))
            .unwrap();
        checks.require(
            css.contains(&format!(
                "{nav_rule}\n{NAV_SPACING}\n{PROVENANCE_SPACING}\n"
            )),
            &format!("{label}: spacing rules directly follow nav.docnav"),
        );
        checks.require(
            page.contains(&format!("<meta charset=\"utf-8\">\n{VIEWPORT}\n<title>"))
                && page.matches(VIEWPORT).count() == 1,
            &format!("{label}: N22 viewport unchanged"),
        );
        checks.require(
            css.contains(&format!("\n{NARROW}\n@media print {{\n"))
                && css.matches(NARROW).count() == 1,
            &format!("{label}: N22 table rule unchanged"),
        );
        checks.require(
            css.contains(
                "nav.crumbs, nav.docnav, .skip, form, .verdict-entry, .hl-note { display: none; }",
            ),
            &format!("{label}: print still hides nav"),
        );
    }
    checks.finish("D6/D8 stylesheet");
}

#[test]
fn u7_corpus_glyphs_stay_literal_in_text_slots() {
    let mut checks = Checks::default();
    for atom in [
        "·",
        "—",
        " · ",
        " — ",
        "Quoted · passage — remains",
        "Not covered — quoted label",
        TITLE_SUFFIX,
        "Prefix & < > \" ' · — 漢",
    ] {
        let fx = Fixture::new("green", "basic");
        let payload = format!("Glyphs {atom} remain.");
        let title = format!("Corpus {atom} title");
        fx.replace(
            "source/evidence.txt",
            "Payload selected for a-10.",
            &payload,
        );
        write(
            &fx.guideline().join("ace/a-10.ace"),
            format!("{payload}\n").as_bytes(),
        );
        write(
            &fx.guideline().join("README.md"),
            format!("# {title}\n").as_bytes(),
        );
        fx.edit_coverage(|row| {
            if row[4] == "ace(a-10)" {
                row[3] = "Literal · and — survive".into();
            }
        });
        fx.repin();
        let output = fx.render();
        fx.check();
        let document = page(&output, "g/alpha/doc/a-10.html");
        let prose = document
            .split("<pre class=\"prose\">")
            .skip(1)
            .map(|part| {
                part.split_once("</pre>")
                    .unwrap()
                    .0
                    .replace("<span class=\"kw\">", "")
                    .replace("</span>", "")
            })
            .collect::<Vec<_>>();
        assert_eq!(prose.len(), 2);
        checks.equal(&prose[0], &text(&payload), &format!("source {atom:?}"));
        checks.equal(
            &prose[1],
            &format!("{}\n", text(&payload)),
            &format!("ACE {atom:?}"),
        );
        checks.equal(
            doc_title(document),
            "Literal · and — survive",
            "literal section glyphs",
        );
        checks.require(
            document.contains(&format!(
                "<a href=\"../index.html\">{}</a> / r1</nav>",
                text(&title)
            )),
            "breadcrumb glyphs and slash separators",
        );
        checks.equal(
            block(page(&output, "g/alpha/index.html"), "<title>", "</title>"),
            &text(&title),
            &format!("corpus title {atom:?}: remove generated suffix only"),
        );
    }
    checks.finish("D8 corpus fidelity");
}
