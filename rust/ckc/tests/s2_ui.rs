// S2 D7/P6: diff-blind UI checks; oracle = 7fb74681:ckc-spec/src/ui.rs + D7.
// Copula Action: seed-3 ui::{participant, verb_html}.
// CKC_S2B_TEST_BIN selects a prebuilt executable; candidate output is never an oracle.
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

const QUALIFIER: &str = "<h3>Qualifiers as compiled</h3>";
const QUALIFIER_WORDS: &str = "<h2>Qualifier words</h2>";
const PREFERENCE: &str = "<h3>Preferences as compiled</h3>";
const GRADE: &str = "<h3>Grade as compiled</h3>";
const TIMING: &str = "<h3>Timing as compiled</h3>";
const PREFERENCE_WORDS: &str = "<h2>Preference words</h2>";
const GRADE_WORDS: &str = "<h2>Grade words</h2>";
const TIME_WORDS: &str = "<h2>Time words</h2>";
const HEADER: &str = "# format: kind\tlemma\tvalue\n";
const DOC_ROUTE: &str = "/g/alpha/doc/d.html";
const VIEWPORT: &str = "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">";
const NARROW: &str =
    "@media screen and (max-width: 40rem) { table { display: block; overflow-x: auto; } }";
const KINDS: [(&str, &str, &str); 7] = [
    ("usuality", "usually", "How often"),
    ("ideality", "ideally", "Best case"),
    ("focus", "especially", "Emphasis"),
    ("manner", "carefully", "Manner"),
    ("force", "strongly", "Strength of wording"),
    ("possibility", "possibly", "Possibility"),
    ("applicability", "feasible", "Condition of use"),
];
static NEXT: AtomicU64 = AtomicU64::new(0);
type Files = BTreeMap<PathBuf, Vec<u8>>;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn program() -> PathBuf {
    std::env::var_os("CKC_S2B_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
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
        for entry in fs::read_dir(here).unwrap() {
            let path = entry.unwrap().path();
            assert!(!path.is_symlink(), "fixture symlink: {}", path.display());
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                out.insert(
                    path.strip_prefix(base).unwrap().into(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut out = Files::new();
    walk(base, base, &mut out);
    out
}

struct Fixture {
    scratch: PathBuf,
    tree: PathBuf,
}
impl Fixture {
    fn new(source: &Path) -> Self {
        let parent = root().join("rust/target/s2-ui");
        fs::create_dir_all(&parent).unwrap();
        let scratch = loop {
            let path = parent.join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => break path,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
                Err(e) => panic!("fixture scratch: {e}"),
            }
        };
        let tree = scratch.join("tree");
        for (path, bytes) in files(source) {
            let target = if path.to_str().unwrap().ends_with(".pl.in") {
                path.with_extension("")
            } else {
                path
            };
            write(&tree.join(target), &bytes);
        }
        Self { scratch, tree }
    }

    fn plain() -> Self {
        let fx = Self::new(&root().join("tests/s2-ui/tree"));
        fx.accepted();
        fx.repin();
        fx
    }

    fn guideline(&self) -> PathBuf {
        self.tree.join("guidelines/alpha")
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(program())
            .args(args)
            .current_dir(&self.scratch)
            .env("GIT_CEILING_DIRECTORIES", &self.scratch)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("TMPDIR", &self.scratch)
            .output()
            .unwrap_or_else(|e| panic!("{} {args:?}: {e}", program().display()))
    }

    fn success(&self, args: &[&str]) -> Output {
        let out = self.run(args);
        assert_eq!(
            out.status.code(),
            Some(0),
            "fixture {args:?}: stdout={}, stderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.stderr.is_empty(),
            "fixture {args:?}: stderr={:?}",
            out.stderr
        );
        out
    }

    fn accepted(&self) {
        self.success(&[
            "v1",
            "check",
            self.guideline().join("pl/d.pl").to_str().unwrap(),
        ]);
    }

    fn repin(&self) {
        // Custody setup only; the command supplies no expected HTML.
        let out = self.success(&["derive-review-manifest", self.guideline().to_str().unwrap()]);
        write(
            &self.guideline().join("audit/review-manifest.tsv"),
            &out.stdout,
        );
    }

    fn document(&self, bundles: &[String]) {
        self.document_with_comment(bundles, "A clinician chooses an option.");
    }

    fn document_with_comment(&self, bundles: &[String], comment: &str) {
        let seed = read(&root().join("tests/s2-ui/tree/guidelines/alpha/pl/d.pl.in"));
        let prefix = seed.split_once("% S1:").unwrap().0;
        let body = bundles
            .iter()
            .enumerate()
            .map(|(i, clauses)| format!("% S{}: {comment}\n{clauses}", i + 1))
            .collect::<String>();
        write(
            &self.guideline().join("pl/d.pl"),
            format!("{prefix}{body}").as_bytes(),
        );
        self.accepted();
        self.repin();
    }

    fn table(&self, rows: Option<&str>) {
        let path = self.guideline().join("vocabulary.tsv");
        if let Some(rows) = rows {
            write(&path, format!("{HEADER}{rows}").as_bytes());
        } else if path.exists() {
            fs::remove_file(path).unwrap();
        }
    }

    fn request(&self, route: &str) -> String {
        let out = self.success(&[
            "ui",
            "request",
            "GET",
            route,
            self.tree.to_str().unwrap(),
            "--token",
            "s2-ui",
        ]);
        let text = String::from_utf8(out.stdout).unwrap();
        let (head, body) = text.split_once("\n\n").expect("HTTP boundary");
        assert_eq!(head.lines().next(), Some("HTTP 200"), "{route}: {head}");
        assert!(body.starts_with("<!doctype html>"));
        body.into()
    }

    fn render(&self) -> Files {
        let out = self.scratch.join("out");
        if out.exists() {
            fs::remove_dir_all(&out).unwrap();
        }
        self.success(&[
            "ui",
            "render",
            out.to_str().unwrap(),
            self.tree.to_str().unwrap(),
        ]);
        let result = files(&out);
        assert!(result.contains_key(Path::new("index.html")));
        result
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.scratch) {
            eprintln!("S2 UI cleanup {}: {e}", self.scratch.display());
        }
    }
}

#[derive(Default)]
struct Checks {
    count: usize,
    failures: Vec<String>,
}
impl Checks {
    fn require(&mut self, condition: bool, label: &str) {
        self.count += 1;
        if !condition {
            self.failures.push(label.into());
        }
    }

    fn equal(&mut self, got: &str, expected: &str, label: &str) {
        self.count += 1;
        if got != expected {
            let at = got
                .bytes()
                .zip(expected.bytes())
                .position(|(a, b)| a != b)
                .unwrap_or(got.len().min(expected.len()));
            self.failures.push(format!(
                "{label}: bytes expected={}, actual={}; first difference={at}; expected {:?}, actual {:?}",
                expected.len(), got.len(),
                String::from_utf8_lossy(&expected.as_bytes()[at..expected.len().min(at + 110)]),
                String::from_utf8_lossy(&got.as_bytes()[at..got.len().min(at + 110)])
            ));
        }
    }

    fn section(&mut self, page: &str, heading: &str, expected: &str, label: &str) {
        self.equal(section(page, heading).unwrap_or(""), expected, label);
        self.require(
            page.matches(heading).count() == usize::from(!expected.is_empty()),
            &format!("{label}: section count"),
        );
    }

    fn qualifiers(&mut self, page: &str, rows: &[String], label: &str) {
        self.section(page, QUALIFIER, &expected_section(QUALIFIER, rows), label);
    }

    fn finish(self, row: &str) {
        println!(
            "S2 UI {row}: {} checks, {} failures",
            self.count,
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "{row}:\n{}",
            self.failures.join("\n")
        );
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn atom(s: &str) -> String {
    if s == "-"
        || (s.bytes().next().is_some_and(|b| b.is_ascii_lowercase())
            && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'))
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}

fn id(s: usize, n: usize) -> String {
    format!("'$guideline_id'(product,d,{s},ref({n}),[])")
}

fn facts(literals: &[String]) -> String {
    literals.iter().map(|l| format!("{l}.\n")).collect()
}

fn rules(heads: &[String], body: &str) -> String {
    heads.iter().map(|h| format!("{h} :- {body}.\n")).collect()
}

fn action(
    s: usize,
    slot: usize,
    verb: &str,
    object: Option<&str>,
    ops: &[&str],
    qualifiers: &[(&str, &str)],
) -> Vec<String> {
    let mut result = Vec::new();
    let mut ctx = "actual".to_owned();
    for (i, op) in ops.iter().enumerate() {
        let next = format!("'$guideline_id'(context,d,{s},box({}),[])", slot + i);
        result.push(format!("guideline_operator({ctx},{next},{})", atom(op)));
        ctx = next;
    }
    let e = id(s, slot);
    let subject = id(s, slot + 1);
    let object_id = id(s, slot + 2);
    result.push(format!(
        "guideline_entity({ctx},{subject},clinician,countable)"
    ));
    result.push(format!("guideline_event({ctx},{e},{})", atom(verb)));
    result.push(format!("guideline_arg({ctx},{e},1,{subject})"));
    if let Some(noun) = object {
        result.push(format!(
            "guideline_entity({ctx},{object_id},{},countable)",
            atom(noun)
        ));
        result.push(format!("guideline_arg({ctx},{e},2,{object_id})"));
    }
    result.extend(qualifiers.iter().map(|(class, word)| {
        format!(
            "guideline_qualifier({ctx},{e},{},{})",
            atom(class),
            atom(word)
        )
    }));
    result
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

fn qualifier_row(
    s: usize,
    part: &str,
    stated: &str,
    action: &str,
    kind: &str,
    word: &str,
) -> String {
    row(&[&s.to_string(), part, stated, action, kind, word])
}

fn section<'a>(page: &'a str, heading: &str) -> Option<&'a str> {
    let open = format!("<section>\n{heading}\n");
    let start = page.find(&open)?;
    let end = page[start..].find("</section>")? + start + "</section>".len();
    Some(&page[start..end])
}

fn expected_section(heading: &str, rows: &[String]) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let (prose, headers, table) = match heading {
        QUALIFIER => (
            "Each row is one qualifier word that the compiler read from the ACE text. A qualifier describes the wording of its action or condition. A qualifier on a condition must also hold in the facts that meet the condition. An excluded condition tests the condition together with its qualifier.",
            "<th>Sentence</th><th>Part</th><th>Stated as</th><th>Action</th><th>Kind</th><th>Word</th>",
            "<table>",
        ),
        QUALIFIER_WORDS => (
            "The compiler reads a qualifier only through the words in this table. The table applies to every document in this guideline. Each qualifier that the compiler read appears on its document page under Qualifiers as compiled.",
            "<th>Word</th><th>Read as</th>",
            "<table class=\"compact\">",
        ),
        _ => panic!("unexpected section {heading}"),
    };
    format!(
        "<section>\n{heading}\n<p>{prose}</p>\n{table}\n<thead><tr>{headers}</tr></thead>\n<tbody>{}</tbody>\n</table>\n</section>",
        rows.join("\n")
    )
}

fn visible(page: &str) -> String {
    let mut out = page.to_owned();
    for (open, close) in [
        ("<style", "</style>"),
        ("<pre", "</pre>"),
        ("<script", "</script>"),
        ("<!--", "-->"),
    ] {
        while let Some(start) = out.find(open) {
            let end = out[start..].find(close).expect("closed element") + start + close.len();
            out.replace_range(start..end, " ");
        }
    }
    let mut in_tag = false;
    out.chars()
        .filter_map(|c| match c {
            '<' => {
                in_tag = true;
                None
            }
            '>' => {
                in_tag = false;
                Some(' ')
            }
            _ if in_tag => None,
            _ => Some(c),
        })
        .collect()
}

#[test]
fn u1_no_qualifier_pages_are_base_identical() {
    let mut c = Checks::default();
    for name in ["basic", "timing", "temporal-words"] {
        let case = root().join("tests/ui/green").join(name);
        let fx = Fixture::new(&case.join("tree"));
        let actual = fx.render();
        let expected = files(&case.join("golden"));
        c.require(
            actual.keys().eq(expected.keys()),
            &format!("{name}: file inventory"),
        );
        for (path, bytes) in &expected {
            c.require(
                actual.get(path) == Some(bytes),
                &format!("{name}/{}: base bytes", path.display()),
            );
        }
        for (path, bytes) in &actual {
            let text = std::str::from_utf8(bytes).unwrap();
            for heading in [QUALIFIER, QUALIFIER_WORDS] {
                c.require(
                    !text.contains(heading),
                    &format!("{name}/{}: no {heading}", path.display()),
                );
            }
        }
    }
    let fx = Fixture::plain();
    fx.table(Some("qualifier\tcarefully\tmanner\n"));
    for comment in [
        "No qualifier literal.",
        "guideline_qualifier(actual,e,manner,carefully).",
    ] {
        fx.document_with_comment(
            &[concat!(
                "guideline_entity(actual,p,guideline_qualifier,countable).\n",
                "guideline_property(actual,p,manner,pos).\n"
            )
            .into()],
            comment,
        );
        c.qualifiers(
            &fx.request(DOC_ROUTE),
            &[],
            "an unused table row and predicate words/comments alone are not qualifiers",
        );
    }
    c.finish("U1 control: green at base");
}

#[test]
fn u2_qualifier_only_and_section_placement() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let expected = [qualifier_row(
        1,
        "statement",
        "should",
        "review record",
        "Manner",
        "carefully",
    )];
    for mask in 0..8 {
        let mut clauses = String::new();
        if mask & 1 != 0 {
            clauses.push_str(concat!(
                "guideline_event(actual,clock,wait).\n",
                "guideline_entity(actual,anchor,visit,countable).\n",
                "guideline_order(actual,clock,after,anchor).\n"
            ));
        }
        if mask & 2 != 0 {
            clauses.push_str(concat!(
                "guideline_entity(actual,label,category,countable).\n",
                "guideline_grade(actual,rec,strength,strong,label).\n"
            ));
        }
        if mask & 4 != 0 {
            clauses.push_str(concat!(
                "guideline_event(actual,choice,choose).\n",
                "guideline_entity(actual,option,option,countable).\n",
                "guideline_preference(actual,choice,option,none).\n"
            ));
        }
        let mut literals = action(
            1,
            100,
            "review",
            Some("record"),
            &["should"],
            &[("manner", "carefully")],
        );
        if mask % 2 == 0 {
            literals.rotate_right(1);
        }
        clauses.push_str(&facts(&literals));
        fx.document(&[clauses]);
        let page = fx.request(DOC_ROUTE);
        c.qualifiers(
            &page,
            &expected,
            &format!("section mask={mask}: exact qualifier bytes"),
        );
        let mut before = None;
        for (bit, heading) in [(1, TIMING), (2, GRADE), (4, PREFERENCE), (8, QUALIFIER)] {
            let present = bit == 8 || mask & bit != 0;
            c.require(
                page.matches(heading).count() == usize::from(present),
                &format!("mask={mask}: {heading} iff rows"),
            );
            if let Some(at) = page.find(heading) {
                if let Some(previous) = before {
                    c.require(
                        previous < at,
                        &format!("mask={mask}: timing → grade → preference → qualifier"),
                    );
                }
                before = Some(at);
            }
        }
        if mask & 4 != 0 {
            let joined = format!(
                "{}\n{}",
                section(&page, PREFERENCE).unwrap(),
                expected_section(QUALIFIER, &expected)
            );
            c.require(
                page.contains(&joined),
                "qualifier section immediately follows preference section",
            );
        }
    }
    c.finish("U2");
}

#[test]
fn u3_all_classes_and_codec_fallbacks() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let mut combined = Vec::new();
    let mut expected = Vec::new();
    for (i, (class, word, kind)) in KINDS.iter().enumerate() {
        let literals = action(
            1,
            100 * i + 1,
            "review",
            Some("record"),
            &["should"],
            &[(*class, *word)],
        );
        let rows = [qualifier_row(
            1,
            "statement",
            "should",
            "review record",
            kind,
            word,
        )];
        fx.document(&[facts(&literals)]);
        c.qualifiers(&fx.request(DOC_ROUTE), &rows, &format!("class={class}"));
        combined.extend(literals);
        expected.extend(rows);
    }
    fx.document(&[facts(&combined)]);
    c.qualifiers(
        &fx.request(DOC_ROUTE),
        &expected,
        "all seven kinds, literal order",
    );
    for class in ["unexpected", "''", "7", "class(manner)", "[]"] {
        fx.document(&[format!(
            "guideline_qualifier(actual,missing,{class},carefully).\n"
        )]);
        c.qualifiers(
            &fx.request(DOC_ROUTE),
            &[qualifier_row(
                1,
                "statement",
                "as a fact",
                "not stated",
                "not stated",
                "carefully",
            )],
            &format!("codec-legal class={class}: not stated"),
        );
    }
    for word in ["''", "word(carefully)", "7", "[]"] {
        fx.document(&[format!(
            "guideline_qualifier(actual,missing,manner,{word}).\n"
        )]);
        c.qualifiers(
            &fx.request(DOC_ROUTE),
            &[qualifier_row(
                1,
                "statement",
                "as a fact",
                "not stated",
                "Manner",
                "not stated",
            )],
            &format!("codec-legal word={word}: not stated"),
        );
    }
    c.finish("U3");
}

#[test]
fn u4_parts_and_generated_operator_paths() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let operators = ["should", "must", "can", "may", "-"];
    let mut paths = vec![Vec::new()];
    for outer in operators {
        paths.push(vec![outer]);
        for inner in operators {
            paths.push(vec![outer, inner]);
        }
    }
    for length in [5, 17, 32] {
        paths.push(
            (0..length)
                .map(|i| operators[(i * 3) % operators.len()])
                .collect(),
        );
    }
    paths.push(vec!["unrecognized"]);
    for ops in paths {
        let words = ops
            .iter()
            .map(|op| match *op {
                "-" => "not",
                "unrecognized" => "not stated",
                other => other,
            })
            .collect::<Vec<_>>()
            .join(" ");
        let stated = if words.is_empty() {
            "as a fact"
        } else {
            &words
        };
        let mut bundles = Vec::new();
        let mut expected = Vec::new();
        for (i, part) in ["statement", "condition", "excluded condition"]
            .iter()
            .enumerate()
        {
            let s = i + 1;
            let literals = action(
                s,
                100,
                "review",
                Some("record"),
                &ops,
                &[("manner", "carefully")],
            );
            bundles.push(match i {
                0 => facts(&literals),
                1 => format!(
                    "guideline_event(actual,result,wait) :- {}.\n",
                    literals.join(", ")
                ),
                _ => format!(
                    "guideline_event(actual,result,wait) :- \\+ ({}).\n",
                    literals.join(", ")
                ),
            });
            expected.push(qualifier_row(
                s,
                part,
                stated,
                "review record",
                "Manner",
                "carefully",
            ));
        }
        fx.document(&bundles);
        c.qualifiers(
            &fx.request(DOC_ROUTE),
            &expected,
            &format!("parts × path={ops:?}"),
        );
    }
    fx.document(&[concat!(
        "guideline_event(actual,e,review).\n",
        "guideline_entity(actual,o,record,countable).\n",
        "guideline_arg(actual,e,2,o).\n",
        "guideline_event(actual,result,wait) :- \\+ guideline_qualifier(actual,e,manner,carefully).\n",
        "guideline_qualifier(orphan,e,focus,especially).\n"
    ).into()]);
    c.qualifiers(
        &fx.request(DOC_ROUTE),
        &[
            qualifier_row(
                1,
                "excluded condition",
                "as a fact",
                "review record",
                "Manner",
                "carefully",
            ),
            qualifier_row(
                1,
                "statement",
                "as a fact",
                "review record",
                "Emphasis",
                "especially",
            ),
        ],
        "singleton NAF + context with no operator edge",
    );
    c.finish("U4");
}

#[test]
fn u5_actions_objects_copula_and_join_precedence() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    for slot in [1, 17, 1009] {
        for qualified in 1..=3 {
            let mut literals = Vec::new();
            let mut expected = Vec::new();
            for (i, noun) in ["record", "regimen"].iter().enumerate() {
                let qualifiers = if qualified & (1 << i) != 0 {
                    vec![("manner", "carefully")]
                } else {
                    Vec::new()
                };
                let mut terms = action(1, slot + i * 100, "review", Some(noun), &[], &qualifiers);
                terms.rotate_left(slot % 3);
                literals.extend(terms);
                if !qualifiers.is_empty() {
                    expected.push(qualifier_row(
                        1,
                        "statement",
                        "as a fact",
                        &format!("review {noun}"),
                        "Manner",
                        "carefully",
                    ));
                }
            }
            fx.document(&[facts(&literals)]);
            c.qualifiers(
                &fx.request(DOC_ROUTE),
                &expected,
                &format!("renaming={slot}, qualified={qualified}: same verb, distinct objects"),
            );
        }
    }
    fx.document(&[concat!(
        "guideline_entity(actual,therapy,treatment,countable).\n",
        "guideline_property(actual,property,feasible,pos).\n",
        "guideline_event(actual,copula,be).\n",
        "guideline_arg(actual,copula,1,therapy).\n",
        "guideline_arg(actual,copula,2,property).\n",
        "guideline_qualifier(actual,copula,applicability,feasible).\n"
    )
    .into()]);
    c.qualifiers(
        &fx.request(DOC_ROUTE),
        &[qualifier_row(
            1,
            "statement",
            "as a fact",
            "treatment",
            "Condition of use",
            "feasible",
        )],
        "property-only copula complement has no noun",
    );
    for (clauses, expected_action) in [
        ("", "not stated"),
        ("guideline_event(actual,e,review).\n", "review"),
        (
            "guideline_event(actual,e,review).\nguideline_arg(actual,e,2,missing).\n",
            "review",
        ),
        (
            "guideline_event(actual,e,review).\nguideline_arg(actual,e,1,o).\nguideline_arg(actual,e,3,o).\nguideline_entity(actual,o,record,countable).\n",
            "review",
        ),
        (
            "guideline_event(actual,e,review).\nguideline_arg(actual,e,2,o).\nguideline_entity(actual,o,noun(record),countable).\n",
            "review not stated",
        ),
        (
            "guideline_event(actual,e,verb(review)).\nguideline_arg(actual,e,2,o).\nguideline_entity(actual,o,record,countable).\n",
            "not stated record",
        ),
        (
            "guideline_event(actual,e,review).\nguideline_arg(actual,e,2,first).\nguideline_arg(actual,e,2,second).\nguideline_entity(actual,first,record,countable).\nguideline_entity(actual,second,regimen,countable).\n",
            "review record",
        ),
    ] {
        fx.document(&[format!(
            "{clauses}guideline_qualifier(actual,e,manner,carefully).\n"
        )]);
        c.qualifiers(
            &fx.request(DOC_ROUTE),
            &[qualifier_row(
                1,
                "statement",
                "as a fact",
                expected_action,
                "Manner",
                "carefully",
            )],
            &format!("action fallback: {expected_action}; {clauses:?}"),
        );
    }
    for key in ["e", "event(key)", "7", "[]"] {
        fx.document(&[format!(concat!(
            "guideline_event(actual,{key},'global-action').\n",
            "guideline_arg(actual,{key},2,global).\n",
            "guideline_entity(actual,global,'global-object',countable).\n",
            "guideline_entity(actual,local,'global-local-name',countable).\n",
            "guideline_qualifier(actual,{key},manner,carefully) :- guideline_event(actual,{key},'local-action'), guideline_arg(actual,{key},2,local), guideline_entity(actual,local,'local-object',countable).\n"
        ), key = key)]);
        c.qualifiers(
            &fx.request(DOC_ROUTE),
            &[qualifier_row(
                1,
                "statement",
                "as a fact",
                "local-action local-object",
                "Manner",
                "carefully",
            )],
            &format!("clause-local event/arg/noun precede heads; key={key}"),
        );
    }
    fx.document(&[
        "guideline_qualifier(actual,e,manner,carefully).\n".into(),
        "guideline_event(actual,e,review).\nguideline_arg(actual,e,2,o).\nguideline_entity(actual,o,record,countable).\n".into(),
    ]);
    c.qualifiers(
        &fx.request(DOC_ROUTE),
        &[qualifier_row(
            1,
            "statement",
            "as a fact",
            "review record",
            "Manner",
            "carefully",
        )],
        "joins may use later sentence heads",
    );
    c.finish("U5");
}

#[test]
fn u6_repeated_rows_keep_first_occurrence_per_sentence() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    for repeats in [1, 2, 5, 17] {
        let mut bundles = Vec::new();
        let mut expected = Vec::new();
        for s in 1..=2 {
            let mut condition = action(
                s,
                10,
                "review",
                Some("record"),
                &[],
                &[("manner", "carefully"), ("focus", "especially")],
            );
            condition.extend(action(
                s,
                50,
                "review",
                Some("record"),
                &[],
                &[("manner", "carefully")],
            ));
            let excluded = action(
                s,
                100,
                "monitor",
                Some("patient"),
                &[],
                &[("manner", "closely")],
            );
            let body = format!("{}, \\+ ({})", condition.join(", "), excluded.join(", "));
            let consequent = action(
                s,
                200,
                "prescribe",
                Some("therapy"),
                &["should"],
                &[("ideality", "ideally")],
            );
            bundles.push(rules(&consequent, &body).repeat(repeats));
            expected.extend([
                qualifier_row(
                    s,
                    "condition",
                    "as a fact",
                    "review record",
                    "Manner",
                    "carefully",
                ),
                qualifier_row(
                    s,
                    "condition",
                    "as a fact",
                    "review record",
                    "Emphasis",
                    "especially",
                ),
                qualifier_row(
                    s,
                    "excluded condition",
                    "as a fact",
                    "monitor patient",
                    "Manner",
                    "closely",
                ),
                qualifier_row(
                    s,
                    "statement",
                    "should",
                    "prescribe therapy",
                    "Best case",
                    "ideally",
                ),
            ]);
        }
        fx.document(&bundles);
        c.qualifiers(
            &fx.request(DOC_ROUTE),
            &expected,
            &format!("repeat={repeats}: dedupe rendered rows within sentence only"),
        );
    }
    fx.document(&[concat!(
        "guideline_qualifier(actual,e,manner,carefully) :- guideline_qualifier(actual,e,focus,especially).\n",
        "guideline_qualifier(actual,e,manner,carefully).\n",
        "guideline_qualifier(actual,e,focus,especially).\n",
        "guideline_qualifier(actual,e,bad,word).\n",
        "guideline_qualifier(actual,e,7,word).\n"
    ).into()]);
    c.qualifiers(
        &fx.request(DOC_ROUTE),
        &[
            qualifier_row(
                1,
                "statement",
                "as a fact",
                "not stated",
                "Manner",
                "carefully",
            ),
            qualifier_row(
                1,
                "condition",
                "as a fact",
                "not stated",
                "Emphasis",
                "especially",
            ),
            qualifier_row(
                1,
                "statement",
                "as a fact",
                "not stated",
                "Emphasis",
                "especially",
            ),
            qualifier_row(
                1,
                "statement",
                "as a fact",
                "not stated",
                "not stated",
                "word",
            ),
        ],
        "head before body; Part distinguishes rows; fallback-equivalent rows dedupe",
    );
    c.finish("U6");
}

#[test]
fn u7_vocabulary_order_presence_and_intake_errors() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    for rotation in 0..KINDS.len() {
        let mut kinds = KINDS.to_vec();
        kinds.rotate_left(rotation);
        if rotation % 2 == 1 {
            kinds.reverse();
        }
        for mask in 0..8 {
            let mut table = String::new();
            let mut expected = Vec::new();
            for (i, (class, word, kind)) in kinds.iter().enumerate() {
                table.push_str(&format!("qualifier\t{word}\t{class}\n"));
                expected.push(row(&[word, kind]));
                for (at, bit, other) in [
                    (0, 1, "unit\tday\tday\n"),
                    (2, 2, "strength\tlabel\tstrong\n"),
                    (
                        4,
                        4,
                        "preference\tpreferred\tpreferred\nalternative\tinstead-of\talternative\n",
                    ),
                ] {
                    if i == at && mask & bit != 0 {
                        table.push_str(other);
                    }
                }
            }
            fx.table(Some(&table));
            let page = fx.request("/g/alpha/");
            c.section(
                &page,
                QUALIFIER_WORDS,
                &expected_section(QUALIFIER_WORDS, &expected),
                &format!("vocabulary rotation={rotation} mask={mask}: file order"),
            );
            let mut before = None;
            for (bit, heading) in [
                (1, TIME_WORDS),
                (2, GRADE_WORDS),
                (4, PREFERENCE_WORDS),
                (8, QUALIFIER_WORDS),
            ] {
                let present = bit == 8 || mask & bit != 0;
                c.require(
                    page.matches(heading).count() == usize::from(present),
                    &format!("vocabulary mask={mask}: {heading} iff rows"),
                );
                if let Some(at) = page.find(heading) {
                    if let Some(previous) = before {
                        c.require(
                            previous < at,
                            "vocabulary: time → grade → preference → qualifier",
                        );
                    }
                    before = Some(at);
                }
            }
            if mask & 4 != 0 {
                c.require(
                    page.contains(&format!(
                        "{}\n{}",
                        section(&page, PREFERENCE_WORDS).unwrap(),
                        expected_section(QUALIFIER_WORDS, &expected)
                    )),
                    "qualifier words immediately follow preference words",
                );
            }
        }
    }
    for (class, word, kind) in KINDS {
        fx.table(Some(&format!("qualifier\t{word}\t{class}\n")));
        c.section(
            &fx.request("/g/alpha/"),
            QUALIFIER_WORDS,
            &expected_section(QUALIFIER_WORDS, &[row(&[word, kind])]),
            &format!("one qualifier table row: {class}"),
        );
    }
    for table in [
        None,
        Some("unit\tday\tday\n"),
        Some("preference\tpreferred\tpreferred\n"),
        Some("strength\tlabel\tstrong\n"),
    ] {
        fx.table(table);
        c.section(
            &fx.request("/g/alpha/"),
            QUALIFIER_WORDS,
            "",
            "no qualifier rows → no qualifier words section",
        );
    }
    // C1 D4: diagnostic rows count body rows, not the header.
    for (bad, why, row) in [
        ("qualifier\tcarefully\tbad\n", "qualifier value", 1),
        (
            "qualifier\tcarefully\tmanner\nqualifier\tcarefully\tfocus\n",
            "duplicate qualifier",
            2,
        ),
        (
            "qualifier\tcarefully\tmanner\nqualifier\tcarefully\tbad\n",
            "qualifier value",
            2,
        ),
        (
            "qualifier\tword\tmanner\npreference\tword\tpreferred\n",
            "duplicate adjective",
            2,
        ),
        (
            "preference\tword\tpreferred\nqualifier\tword\tmanner\n",
            "duplicate adjective",
            2,
        ),
        (
            "approximation\tword\tabout\nqualifier\tword\tmanner\n",
            "duplicate adjective",
            2,
        ),
        (
            "qualifier\tword\tmanner\napproximation\tword\tabout\n",
            "duplicate adjective",
            2,
        ),
    ] {
        fx.table(Some(bad));
        let out = fx.run(&["ui", "check", fx.tree.to_str().unwrap()]);
        c.require(
            out.status.code() == Some(1),
            &format!("invalid qualifier table {why}: rc=1"),
        );
        c.equal(
            &String::from_utf8(out.stderr).unwrap(),
            &format!("ui: viewmodel: alpha vocabulary.tsv: {why} at row {row}\n"),
            "invalid table: exact intake diagnostic",
        );
        c.require(out.stdout.is_empty(), "invalid table: empty stdout");
    }
    c.finish("U7");
}

#[test]
fn u8_generated_words_escape_once_in_each_slot() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let alphabet = [
        "a",
        "<&>",
        "\"quoted\"",
        "x'y",
        "a\\b",
        "&amp;",
        "漢λ",
        "<script>x</script>",
    ];
    for length in [0, 1, 2, 7, 31] {
        for rotation in 0..alphabet.len() {
            let payload = (0..length)
                .map(|i| alphabet[(i + rotation) % alphabet.len()])
                .collect::<String>();
            let words = ["verb", "noun", "word"].map(|prefix| {
                if payload.is_empty() {
                    String::new()
                } else {
                    format!("{prefix}:{payload}")
                }
            });
            let terms = action(
                1,
                1,
                &words[0],
                Some(&words[1]),
                &[],
                &[("manner", &words[2])],
            );
            fx.document_with_comment(
                &[facts(&terms)],
                &format!("Decoded words: {} | {} | {}", words[0], words[1], words[2]),
            );
            let shown = words.each_ref().map(|w| {
                if w.is_empty() {
                    "not stated"
                } else {
                    w.as_str()
                }
            });
            let page = fx.request(DOC_ROUTE);
            c.qualifiers(
                &page,
                &[qualifier_row(
                    1,
                    "statement",
                    "as a fact",
                    &format!("{} {}", shown[0], shown[1]),
                    "Manner",
                    shown[2],
                )],
                &format!("escaping length={length} rotation={rotation}"),
            );
            let table = section(&page, QUALIFIER).unwrap_or("");
            c.require(
                !table.contains("<script>") && !table.contains("<td><"),
                "literal bytes stay text, never elements",
            );
        }
    }
    fx.document(&[facts(&action(
        1,
        1,
        "a\\b",
        Some("x'y"),
        &[],
        &[("manner", "z'w")],
    ))]);
    c.qualifiers(
        &fx.request(DOC_ROUTE),
        &[qualifier_row(
            1,
            "statement",
            "as a fact",
            "not stated not stated",
            "Manner",
            "not stated",
        )],
        "decoded bytes absent verbatim in PL → not stated in all three slots",
    );
    for (class, _, kind) in KINDS {
        let lemma = "x<&>\"'漢";
        fx.table(Some(&format!("qualifier\t{lemma}\t{class}\n")));
        c.section(
            &fx.request("/g/alpha/"),
            QUALIFIER_WORDS,
            &expected_section(QUALIFIER_WORDS, &[row(&[lemma, kind])]),
            &format!("vocabulary escaping class={class}"),
        );
    }
    c.finish("U8");
}

#[test]
fn u9_new_copy_shows_words_not_internal_names() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let mut literals = Vec::new();
    let mut table = String::new();
    for (i, (class, word, _)) in KINDS.iter().enumerate() {
        literals.extend(action(
            1,
            100 * i + 1,
            "review",
            Some("record"),
            &["should"],
            &[(*class, *word)],
        ));
        table.push_str(&format!("qualifier\t{word}\t{class}\n"));
    }
    fx.document(&[facts(&literals)]);
    fx.table(Some(&table));
    for (route, heading) in [(DOC_ROUTE, QUALIFIER), ("/g/alpha/", QUALIFIER_WORDS)] {
        let page = fx.request(route);
        c.require(
            page.matches(heading).count() == 1,
            &format!("{route}: new section present (non-vacuous copy check)"),
        );
        let shown = visible(&page);
        c.require(
            !shown.contains("guideline_") && !shown.contains("$guideline_id"),
            &format!("{route}: no functor names"),
        );
        let words = shown
            .split(|ch: char| !(ch.is_alphanumeric() || ch == '_'))
            .collect::<Vec<_>>();
        for (class, word, kind) in KINDS {
            c.require(
                !words.contains(&class),
                &format!("{route}: internal class {class} absent"),
            );
            let group = section(&page, heading).unwrap_or("");
            c.require(
                group.contains(&format!("<td>{word}</td>"))
                    && group.contains(&format!("<td>{kind}</td>")),
                &format!("{route}: word + kind for {class}"),
            );
        }
    }
    c.finish("U9");
}

#[test]
fn u10_qualifier_pages_keep_mobile_and_print_frame_bytes() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    fx.document(&[facts(&action(
        1,
        1,
        "review",
        Some("record"),
        &["should", "-"],
        &[("manner", "carefully")],
    ))]);
    fx.table(Some(
        "qualifier\tcarefully\tmanner\npreference\tpreferred\tpreferred\n",
    ));
    let baseline = read(&root().join("tests/ui/green/basic/golden/index.html"));
    let css = |page: &str| {
        page.split_once("<style>\n")
            .unwrap()
            .1
            .split_once("</style>")
            .unwrap()
            .0
            .to_owned()
    };
    let expected_css = css(&baseline);
    let mut pages = fx
        .render()
        .into_iter()
        .filter(|(path, _)| path.extension().is_some_and(|e| e == "html"))
        .map(|(path, bytes)| {
            (
                path.display().to_string(),
                String::from_utf8(bytes).unwrap(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for route in ["/", "/g/alpha/", "/g/alpha/records.html", DOC_ROUTE] {
        pages.insert(format!("GET {route}"), fx.request(route));
    }
    c.require(
        pages.contains_key("g/alpha/doc/d.html") && pages.contains_key("g/alpha/index.html"),
        "frame check covers both qualifier-bearing input pages",
    );
    for (label, page) in &pages {
        c.equal(
            &css(page),
            &expected_css,
            &format!("{label}: full stylesheet identity (mobile + print included)"),
        );
        c.require(
            page.matches(VIEWPORT).count() == 1
                && page.contains(&format!("<meta charset=\"utf-8\">\n{VIEWPORT}\n<title>")),
            &format!("{label}: viewport position + uniqueness"),
        );
        c.require(
            page.contains(&format!("{NARROW}\n@media print {{\n")),
            &format!("{label}: narrow rule immediately before print"),
        );
    }
    c.finish("U10 control: green at base");
}

#[test]
fn u11_copula_subjects_keep_scope_and_ordinary_action_objects() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    for noun in ["state-PDMP-record", "treatment", "word:<&>\"'漢"] {
        for adjective in ["appropriate", "needed"] {
            let copula = vec![
                format!("guideline_entity(actual,subject,{},countable)", atom(noun)),
                format!("guideline_property(actual,property,{adjective},pos)"),
                "guideline_event(actual,copula,be)".into(),
                "guideline_arg(actual,copula,1,subject)".into(),
                "guideline_arg(actual,copula,2,property)".into(),
                "guideline_entity(actual,other,'concurrent-substance-use-assessment',countable)"
                    .into(),
                "guideline_pp(actual,copula,for,other)".into(),
                format!("guideline_qualifier(actual,copula,applicability,{adjective})"),
            ];
            for part in ["statement", "condition", "excluded condition"] {
                let mut clauses = match part {
                    "statement" => facts(&copula),
                    "condition" => format!(
                        "guideline_event(actual,result,wait) :- {}.\n",
                        copula.join(", ")
                    ),
                    _ => format!(
                        "guideline_event(actual,result,wait) :- \\+ ({}).\n",
                        copula.join(", ")
                    ),
                };
                clauses.push_str(&facts(&action(
                    1,
                    100,
                    "review",
                    Some("record"),
                    &["should"],
                    &[("manner", "carefully")],
                )));
                fx.document_with_comment(&[clauses], &format!("{noun} is {adjective}."));
                c.qualifiers(
                    &fx.request(DOC_ROUTE),
                    &[
                        qualifier_row(1, part, "as a fact", noun, "Condition of use", adjective),
                        qualifier_row(
                            1,
                            "statement",
                            "should",
                            "review record",
                            "Manner",
                            "carefully",
                        ),
                    ],
                    &format!(
                        "copula subject={noun} adjective={adjective} part={part}; pp not an object"
                    ),
                );
            }
        }
    }
    for key in ["e", "event(key)", "7", "[]"] {
        fx.document(&[format!(concat!(
            "guideline_event(actual,{key},be).\n",
            "guideline_arg(actual,{key},1,global).\n",
            "guideline_entity(actual,global,'global-subject',countable).\n",
            "guideline_entity(actual,local,'global-local-name',countable).\n",
            "guideline_qualifier(actual,{key},applicability,needed) :- guideline_event(actual,{key},be), guideline_arg(actual,{key},1,local), guideline_entity(actual,local,'local-subject',countable).\n"
        ), key = key)]);
        c.qualifiers(
            &fx.request(DOC_ROUTE),
            &[qualifier_row(
                1,
                "statement",
                "as a fact",
                "local-subject",
                "Condition of use",
                "needed",
            )],
            &format!("copula clause-local subject precedes heads; key={key}"),
        );
    }
    fx.document(&[facts(&action(
        1,
        1,
        "be",
        Some("role"),
        &["should"],
        &[("manner", "carefully")],
    ))]);
    c.qualifiers(
        &fx.request(DOC_ROUTE),
        &[qualifier_row(
            1,
            "statement",
            "should",
            "clinician role",
            "Manner",
            "carefully",
        )],
        "copula adverb with a noun complement keeps the second-participant suffix",
    );
    c.finish("U11");
}

#[test]
fn u12_copula_missing_subject_and_first_participant_fallbacks() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    for (args, expected) in [
        ("", "not stated"),
        ("guideline_arg(actual,e,1,missing).\n", "not stated"),
        ("guideline_arg(actual,e,3,subject).\n", "not stated"),
        ("guideline_arg(actual,e,'1',subject).\n", "not stated"),
        (
            "guideline_arg(actual,e,1,first).\nguideline_arg(actual,e,1,subject).\n",
            "not stated",
        ),
        (
            "guideline_arg(actual,e,1,subject).\nguideline_arg(actual,e,1,missing).\n",
            "subject-noun",
        ),
        (
            "guideline_arg(actual,e,1,subject).\nguideline_entity(actual,subject,noun(subject),countable).\n",
            "not stated",
        ),
        (
            "guideline_arg(actual,e,1,subject).\nguideline_entity(actual,subject,'',countable).\n",
            "not stated",
        ),
    ] {
        fx.document(&[format!(
            concat!(
                "{args}",
                "guideline_entity(actual,subject,'subject-noun',countable).\n",
                "guideline_event(actual,e,be).\n",
                "guideline_arg(actual,e,2,property).\n",
                "guideline_property(actual,property,needed,pos).\n",
                "guideline_qualifier(actual,e,applicability,needed).\n"
            ),
            args = args
        )]);
        c.qualifiers(
            &fx.request(DOC_ROUTE),
            &[qualifier_row(
                1,
                "statement",
                "as a fact",
                expected,
                "Condition of use",
                "needed",
            )],
            &format!("copula subject fallback: {args:?}"),
        );
    }
    c.finish("U12");
}
