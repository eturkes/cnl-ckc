// S1 D8/P6: contract-only UI tests; copy oracle = b83e4a78:ckc-spec/src/ui.rs.
// Copula Action: S2 D7 + seed-3 ui::{participant, verb_html}.
// CKC_S1B_TEST_BIN selects a prebuilt executable; no candidate output supplies expectations.
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

const GRADE: &str = "<h3>Grade as compiled</h3>";
const PREFERENCE: &str = "<h3>Preferences as compiled</h3>";
const TIMING: &str = "<h3>Timing as compiled</h3>";
const GRADE_WORDS: &str = "<h2>Grade words</h2>";
const PREFERENCE_WORDS: &str = "<h2>Preference words</h2>";
const TIME_WORDS: &str = "<h2>Time words</h2>";
const HEADER: &str = "# format: kind\tlemma\tvalue\n";
const VIEWPORT: &str = "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">";
const NARROW: &str =
    "@media screen and (max-width: 40rem) { table { display: block; overflow-x: auto; } }";
const DOC_ROUTE: &str = "/g/alpha/doc/d.html";
static NEXT: AtomicU64 = AtomicU64::new(0);
type Files = BTreeMap<PathBuf, Vec<u8>>;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn program() -> PathBuf {
    std::env::var_os("CKC_S1B_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn write(path: &Path, data: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, data).unwrap();
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
        let parent = root().join("rust/target/s1-ui");
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
        let fx = Self::new(&root().join("tests/s1-ui/tree"));
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
        // Custody setup only; expected HTML never comes from this command.
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
        let seed = read(&root().join("tests/s1-ui/tree/guidelines/alpha/pl/d.pl.in"));
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
            "s1-ui",
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
            eprintln!("S1 UI cleanup {}: {e}", self.scratch.display());
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

    fn finish(self, row: &str) {
        println!(
            "S1 UI {row}: {} checks, {} failures",
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

fn grade(s: usize, slot: usize, dimension: &str, value: &str, label: &str) -> Vec<String> {
    let r = id(s, slot);
    let l = id(s, slot + 1);
    let e = id(s, slot + 2);
    vec![
        format!("guideline_entity(actual,{r},recommendation,countable)"),
        format!("guideline_entity(actual,{l},{},countable)", atom(label)),
        format!("guideline_event(actual,{e},be)"),
        format!("guideline_arg(actual,{e},1,{r})"),
        format!("guideline_arg(actual,{e},2,{l})"),
        format!(
            "guideline_grade(actual,{r},{},{},{l})",
            atom(dimension),
            atom(value)
        ),
    ]
}

fn preference(
    s: usize,
    slot: usize,
    action: &str,
    preferred: &str,
    alternative: Option<&str>,
    ops: &[&str],
) -> Vec<String> {
    let mut result = Vec::new();
    let mut ctx = "actual".to_owned();
    for (i, op) in ops.iter().enumerate() {
        let next = format!("'$guideline_id'(context,d,{s},box({}),[])", slot + i);
        result.push(format!("guideline_operator({ctx},{next},{})", atom(op)));
        ctx = next;
    }
    let p = id(s, slot);
    let a = id(s, slot + 1);
    let e = id(s, slot + 2);
    let subject = id(s, slot + 3);
    result.push(format!(
        "guideline_entity({ctx},{p},{},countable)",
        atom(preferred)
    ));
    if let Some(noun) = alternative {
        result.push(format!(
            "guideline_entity({ctx},{a},{},countable)",
            atom(noun)
        ));
    }
    if action == "be" {
        result.push(format!("guideline_property({ctx},{subject},preferred,pos)"));
    } else {
        result.push(format!(
            "guideline_entity({ctx},{subject},clinician,countable)"
        ));
    }
    result.push(format!("guideline_event({ctx},{e},{})", atom(action)));
    let (arg1, arg2) = if action == "be" {
        (&p, &subject)
    } else {
        (&subject, &p)
    };
    result.push(format!("guideline_arg({ctx},{e},1,{arg1})"));
    result.push(format!("guideline_arg({ctx},{e},2,{arg2})"));
    if alternative.is_some() {
        result.push(format!("guideline_pp({ctx},{e},'instead-of',{a})"));
    }
    result.push(format!(
        "guideline_preference({ctx},{e},{p},{})",
        if alternative.is_some() { &a } else { "none" }
    ));
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
        GRADE => (
            "Each row is one grade that the compiler read from this document. A grade applies to every statement of the document.",
            "<th>Measure</th><th>Value</th><th>Label in the source</th>",
            "<table>",
        ),
        PREFERENCE => (
            "Each row is one preference that the compiler read from the ACE text. A preference names the option that the source prefers. It does not forbid the other option.",
            "<th>Sentence</th><th>Part</th><th>Stated as</th><th>Action</th><th>Preferred</th><th>Instead of</th>",
            "<table>",
        ),
        GRADE_WORDS => (
            "The compiler reads the grade of a recommendation only through the words in this table. The table applies to every document in this guideline. Each grade that the compiler read appears on its document page under Grade as compiled.",
            "<th>Word</th><th>Read as</th>",
            "<table class=\"compact\">",
        ),
        PREFERENCE_WORDS => (
            "The compiler reads a preference only through the words in this table. The table applies to every document in this guideline. Each preference that the compiler read appears on its document page under Preferences as compiled.",
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

fn preference_row(
    s: usize,
    part: &str,
    stated: &str,
    action: &str,
    preferred: &str,
    alternative: &str,
) -> String {
    row(&[&s.to_string(), part, stated, action, preferred, alternative])
}

fn word_value(value: &str) -> &str {
    if value == "very_low" {
        "very low"
    } else {
        value
    }
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
fn u1_no_strength_pages_are_base_identical() {
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
            for heading in [GRADE, PREFERENCE, GRADE_WORDS, PREFERENCE_WORDS] {
                c.require(
                    !text.contains(heading),
                    &format!("{name}/{}: no {heading}", path.display()),
                );
            }
        }
    }
    let fx = Fixture::plain();
    for comment in [
        "No strength literals.",
        "guideline_grade(actual,r,strength,strong,l). guideline_preference(actual,e,p,none).",
    ] {
        fx.document_with_comment(
            &["guideline_entity(actual,p,option,countable).\n".into()],
            comment,
        );
        let page = fx.request(DOC_ROUTE);
        for heading in [GRADE, PREFERENCE] {
            c.section(
                &page,
                heading,
                "",
                "no literals: annotations in comments are not rows",
            );
        }
    }
    c.finish("U1 control: green at base");
}

#[test]
fn u2_grade_measures_values_and_label_joins() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let strengths = ["strong", "conditional"];
    let evidences = ["high", "moderate", "low", "very_low"];
    for strength in strengths {
        for evidence in evidences {
            let mut literals = grade(1, 1, "strength", strength, "source-strength");
            literals.extend(grade(1, 10, "evidence", evidence, "source-evidence"));
            fx.document(&[facts(&literals)]);
            let page = fx.request(DOC_ROUTE);
            let rows = [
                row(&[
                    "Strength of the recommendation",
                    strength,
                    "source-strength",
                ]),
                row(&[
                    "Certainty of the evidence",
                    word_value(evidence),
                    "source-evidence",
                ]),
            ];
            c.section(
                &page,
                GRADE,
                &expected_section(GRADE, &rows),
                &format!("grade {strength}/{evidence}"),
            );
            c.section(&page, PREFERENCE, "", "grade only: no preference section");
            c.require(
                !page.contains(TIMING),
                "grade without timing: no timing section",
            );
        }
    }
    for (dimension, measure, values) in [
        (
            "strength",
            "Strength of the recommendation",
            strengths.as_slice(),
        ),
        (
            "evidence",
            "Certainty of the evidence",
            evidences.as_slice(),
        ),
    ] {
        for value in values {
            fx.document(&[facts(&grade(1, 1, dimension, value, "only-label"))]);
            c.section(
                &fx.request(DOC_ROUTE),
                GRADE,
                &expected_section(GRADE, &[row(&[measure, word_value(value), "only-label"])]),
                &format!("single dimension {dimension}/{value}"),
            );
        }
    }
    for raw in [
        "guideline_grade(actual,r,other,other,missing).\n",
        "guideline_grade(actual,r,dimension(x),7,missing).\n",
    ] {
        fx.document(&[raw.into()]);
        c.section(
            &fx.request(DOC_ROUTE),
            GRADE,
            &expected_section(GRADE, &[row(&["not stated", "not stated", "not stated"])]),
            "codec accepts terms; missing grade words use not stated",
        );
    }
    // Clause-local joins precede document heads, including joins in a body/NAF group.
    fx.document(&[concat!(
        "guideline_entity(actual,l,global,countable).\n",
        "guideline_grade(actual,r,evidence,low,l) :- guideline_entity(actual,l,local,countable).\n",
        "guideline_event(actual,e,wait) :- \\+ (guideline_grade(actual,r,strength,strong,n), guideline_entity(actual,n,excluded,countable)).\n"
    ).into()]);
    c.section(
        &fx.request(DOC_ROUTE),
        GRADE,
        &expected_section(
            GRADE,
            &[
                row(&["Certainty of the evidence", "low", "local"]),
                row(&["Strength of the recommendation", "strong", "excluded"]),
            ],
        ),
        "grade clause-local joins and NAF literals",
    );
    c.finish("U2");
}

#[test]
fn u3_bare_preference_without_timing_is_a_fact() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    for noun in ["option", "nonopioid-therapy", "NSAID", "none"] {
        fx.document(&[facts(&preference(1, 1, "be", noun, None, &[]))]);
        let page = fx.request(DOC_ROUTE);
        c.section(
            &page,
            PREFERENCE,
            &expected_section(
                PREFERENCE,
                &[preference_row(
                    1,
                    "statement",
                    "as a fact",
                    noun,
                    noun,
                    "none named",
                )],
            ),
            &format!("bare preference {noun}"),
        );
        c.section(&page, GRADE, "", "bare preference: no grade");
        c.require(!page.contains(TIMING), "bare preference: no timing");
    }
    c.finish("U3");
}

#[test]
fn u4_alternatives_actions_and_reference_renaming() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    for action in ["prescribe", "choose", "be"] {
        for slot in [1, 17, 1009] {
            let mut literals =
                preference(1, slot, action, "chosen-option", Some("other-option"), &[]);
            // Moving primitive heads cannot change the joined words of this literal.
            literals.rotate_left(slot % 3);
            fx.document(&[facts(&literals)]);
            c.section(
                &fx.request(DOC_ROUTE),
                PREFERENCE,
                &expected_section(
                    PREFERENCE,
                    &[preference_row(
                        1,
                        "statement",
                        "as a fact",
                        if action == "be" {
                            "chosen-option"
                        } else {
                            action
                        },
                        "chosen-option",
                        "other-option",
                    )],
                ),
                &format!("alternative action={action} ref-slot={slot}"),
            );
        }
    }
    fx.document(&[concat!(
        "guideline_entity(actual,p,'global-option',countable).\n",
        "guideline_event(actual,e,'global-action').\n",
        "guideline_preference(actual,e,p,a) :- guideline_entity(actual,p,'local-option',countable), guideline_entity(actual,a,'local-alternative',countable), guideline_event(actual,e,'local-action').\n",
        "guideline_preference(actual,missing_event,missing_option,missing_alternative).\n",
        "guideline_preference(actual,missing_event,missing_option,none).\n"
    ).into()]);
    c.section(
        &fx.request(DOC_ROUTE),
        PREFERENCE,
        &expected_section(
            PREFERENCE,
            &[
                preference_row(
                    1,
                    "statement",
                    "as a fact",
                    "local-action",
                    "local-option",
                    "local-alternative",
                ),
                preference_row(
                    1,
                    "statement",
                    "as a fact",
                    "not stated",
                    "not stated",
                    "not stated",
                ),
                preference_row(
                    1,
                    "statement",
                    "as a fact",
                    "not stated",
                    "not stated",
                    "none named",
                ),
            ],
        ),
        "preference local joins and unresolved endpoints",
    );
    c.finish("U4");
}

#[test]
fn u5_conditions_consequents_naf_and_sentence_deduplication() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let mut bundles = Vec::new();
    let mut rows = Vec::new();
    for sentence in 1..=2 {
        let condition = preference(sentence, 10, "be", "condition-option", None, &[]);
        let excluded = preference(sentence, 50, "be", "excluded-option", None, &[]);
        let consequent = preference(
            sentence,
            100,
            "prescribe",
            "chosen-option",
            Some("other-option"),
            &["should"],
        );
        let body = format!(
            "{}, \\+ ({}), \\+ guideline_preference(actual,missing_event,missing_option,none)",
            condition.join(", "),
            excluded.join(", ")
        );
        bundles.push(
            consequent
                .iter()
                .map(|head| format!("{head} :- {body}.\n"))
                .collect(),
        );
        rows.extend([
            preference_row(
                sentence,
                "condition",
                "as a fact",
                "condition-option",
                "condition-option",
                "none named",
            ),
            preference_row(
                sentence,
                "excluded condition",
                "as a fact",
                "excluded-option",
                "excluded-option",
                "none named",
            ),
            preference_row(
                sentence,
                "excluded condition",
                "as a fact",
                "not stated",
                "not stated",
                "none named",
            ),
            preference_row(
                sentence,
                "statement",
                "should",
                "prescribe",
                "chosen-option",
                "other-option",
            ),
        ]);
    }
    fx.document(&bundles);
    c.section(
        &fx.request(DOC_ROUTE),
        PREFERENCE,
        &expected_section(PREFERENCE, &rows),
        "antecedent + grouped/singleton NAF + consequent; repeats collapse within sentence only",
    );
    c.finish("U5");
}

#[test]
fn u6_operator_paths_are_outermost_first() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let ops = ["should", "must", "can", "may", "-"];
    let mut paths = vec![Vec::new()];
    for a in ops {
        paths.push(vec![a]);
        for b in ops {
            paths.push(vec![a, b]);
        }
    }
    for length in [5, 8, 17, 32] {
        paths.push((0..length).map(|i| ops[(i * 3) % ops.len()]).collect());
    }
    paths.push(vec!["unrecognized"]);
    for path in &paths {
        let words = path
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
        fx.document(&[facts(&preference(1, 1, "be", "option", None, path))]);
        c.section(
            &fx.request(DOC_ROUTE),
            PREFERENCE,
            &expected_section(
                PREFERENCE,
                &[preference_row(
                    1,
                    "statement",
                    stated,
                    "option",
                    "option",
                    "none named",
                )],
            ),
            &format!("operator path {path:?}"),
        );
    }
    // Operators may be clause-local rule goals, not only root-fact heads.
    let terms = preference(1, 1, "be", "option", None, &["must", "-"]);
    let body = terms.join(", ");
    fx.document(&[format!("guideline_event(actual,result,wait) :- {body}.\n")]);
    c.section(
        &fx.request(DOC_ROUTE),
        PREFERENCE,
        &expected_section(
            PREFERENCE,
            &[preference_row(
                1,
                "condition",
                "must not",
                "option",
                "option",
                "none named",
            )],
        ),
        "clause-local operator path",
    );
    c.finish("U6");
}

fn timing() -> String {
    concat!(
        "guideline_event(actual,clock,wait).\n",
        "guideline_entity(actual,anchor,visit,countable).\n",
        "guideline_order(actual,clock,after,anchor).\n"
    )
    .into()
}

#[test]
fn u7_sections_exist_exactly_when_rows_exist_and_keep_order() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    for mask in 0..8 {
        let mut clauses = "guideline_entity(actual,p,patient,countable).\n".to_owned();
        if mask & 1 != 0 {
            clauses.push_str(&timing());
        }
        if mask & 2 != 0 {
            clauses.push_str(&facts(&grade(1, 10, "strength", "conditional", "label")));
        }
        if mask & 4 != 0 {
            clauses.push_str(&facts(&preference(1, 50, "be", "option", None, &[])));
        }
        fx.document(&[clauses]);
        let page = fx.request(DOC_ROUTE);
        let mut previous = None;
        for (bit, heading) in [(1, TIMING), (2, GRADE), (4, PREFERENCE)] {
            let expected = mask & bit != 0;
            c.require(
                page.matches(heading).count() == usize::from(expected),
                &format!("mask={mask}: {heading} iff rows"),
            );
            if let Some(position) = page.find(heading) {
                if let Some(before) = previous {
                    c.require(
                        before < position,
                        &format!("mask={mask}: timing → grade → preferences"),
                    );
                }
                previous = Some(position);
            }
        }
        if mask & 6 == 6 {
            let joined = format!(
                "{}\n{}",
                section(&page, GRADE).unwrap_or("missing-grade"),
                section(&page, PREFERENCE).unwrap_or("missing-preferences")
            );
            c.require(
                page.contains(&joined),
                "grade and preference sections are adjacent",
            );
        }
    }
    c.finish("U7");
}

fn expected_words(rows: &[(&str, &str, &str)], grades: bool) -> Vec<String> {
    let kinds = if grades {
        ["strength", "evidence"]
    } else {
        ["preference", "alternative"]
    };
    kinds
        .iter()
        .flat_map(|kind| {
            rows.iter()
                .filter(move |r| r.0 == *kind)
                .map(|&(k, lemma, value)| {
                    let meaning = match k {
                        "strength" => {
                            format!("strength of the recommendation: {}", word_value(value))
                        }
                        "evidence" => format!("certainty of the evidence: {}", word_value(value)),
                        "preference" => "marks the preferred option".into(),
                        "alternative" => {
                            "joins the option that the preferred option replaces".into()
                        }
                        _ => unreachable!(),
                    };
                    row(&[lemma, &meaning])
                })
        })
        .collect()
}

#[test]
fn u8_vocabulary_groups_order_values_and_intake_errors() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let input = [
        ("alternative", "instead-of", "alternative"),
        ("evidence", "e-low", "low"),
        ("preference", "preferred", "preferred"),
        ("strength", "s-conditional", "conditional"),
        ("evidence", "e-high", "high"),
        ("strength", "s-strong", "strong"),
        ("evidence", "e-very-low", "very_low"),
        ("preference", "first-line", "preferred"),
        ("alternative", "rather-than", "alternative"),
        ("evidence", "e-moderate", "moderate"),
    ];
    for rotation in 0..input.len() {
        let mut rows = input.to_vec();
        rows.rotate_left(rotation);
        for mask in 0..8 {
            let kept = rows
                .iter()
                .copied()
                .filter(|r| {
                    if matches!(r.0, "strength" | "evidence") {
                        mask & 2 != 0
                    } else {
                        mask & 4 != 0
                    }
                })
                .collect::<Vec<_>>();
            let mut table = if mask & 1 != 0 {
                "unit\tday\tday\n".into()
            } else {
                String::new()
            };
            for (kind, lemma, value) in &kept {
                table.push_str(&format!("{kind}\t{lemma}\t{value}\n"));
            }
            fx.table((!table.is_empty()).then_some(table.as_str()));
            let page = fx.request("/g/alpha/");
            for (grades, heading) in [(true, GRADE_WORDS), (false, PREFERENCE_WORDS)] {
                c.section(
                    &page,
                    heading,
                    &expected_section(heading, &expected_words(&kept, grades)),
                    &format!("vocabulary rotation={rotation} mask={mask}: {heading}"),
                );
            }
            c.require(
                page.matches(TIME_WORDS).count() == usize::from(mask & 1 != 0),
                "time group iff time rows",
            );
            let positions = [TIME_WORDS, GRADE_WORDS, PREFERENCE_WORDS]
                .into_iter()
                .filter_map(|h| page.find(h))
                .collect::<Vec<_>>();
            c.require(
                positions.windows(2).all(|p| p[0] < p[1]),
                "vocabulary: time → grade → preference groups",
            );
            c.require(
                !visible(&page).contains("guideline_"),
                "vocabulary: no visible functor names",
            );
        }
    }
    // S1-specific intake errors remain refusals, rather than an empty vocabulary group.
    for (bad, why) in [
        ("strength\tlabel\twrong\n", "strength value"),
        ("evidence\tlabel\twrong\n", "evidence value"),
        ("preference\tword\twrong\n", "preference value"),
        ("alternative\tword\twrong\n", "alternative value"),
        (
            "strength\tlabel\tstrong\nevidence\tlabel\tlow\n",
            "duplicate noun",
        ),
    ] {
        fx.table(Some(bad));
        let out = fx.run(&["ui", "check", fx.tree.to_str().unwrap()]);
        c.require(
            out.status.code() == Some(1),
            &format!("invalid table {why}: rc=1"),
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        c.require(
            stderr.starts_with(&format!(
                "ui: viewmodel: alpha vocabulary.tsv: {why} at row "
            )) && stderr.ends_with('\n'),
            &format!("invalid S1 table diagnostic: {why}"),
        );
        c.require(out.stdout.is_empty(), "invalid table: empty stdout");
    }
    c.finish("U8");
}

#[test]
fn u9_generated_words_escape_once_and_functors_stay_out_of_copy() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let atoms = [
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
        for rotation in 0..atoms.len() {
            let payload = (0..length)
                .map(|i| atoms[(i + rotation) % atoms.len()])
                .collect::<String>();
            let word = if payload.is_empty() {
                payload
            } else {
                format!("word:{payload}")
            };
            let mut literals = grade(1, 1, "evidence", "very_low", &word);
            literals.extend(preference(1, 50, &word, &word, Some(&word), &[]));
            // Decoded quoted atoms need a verbatim PL span, which this comment supplies.
            fx.document_with_comment(&[facts(&literals)], &format!("Literal words: {word}"));
            let shown = if word.is_empty() { "not stated" } else { &word };
            let page = fx.request(DOC_ROUTE);
            c.section(
                &page,
                GRADE,
                &expected_section(
                    GRADE,
                    &[row(&["Certainty of the evidence", "very low", shown])],
                ),
                &format!("grade escaping length={length} rotation={rotation}"),
            );
            c.section(
                &page,
                PREFERENCE,
                &expected_section(
                    PREFERENCE,
                    &[preference_row(
                        1,
                        "statement",
                        "as a fact",
                        shown,
                        shown,
                        shown,
                    )],
                ),
                &format!("preference escaping length={length} rotation={rotation}"),
            );
            c.require(
                !visible(&page).contains("guideline_"),
                "document copy: no functor names",
            );
            for heading in [GRADE, PREFERENCE] {
                c.require(
                    !section(&page, heading).unwrap_or("").contains("<script>"),
                    "word stays text, never markup",
                );
            }
        }
    }
    let mut literals = grade(1, 1, "strength", "strong", "x'y");
    literals.extend(preference(1, 50, "a\\b", "x'y", None, &[]));
    fx.document(&[facts(&literals)]);
    let page = fx.request(DOC_ROUTE);
    c.section(
        &page,
        GRADE,
        &expected_section(
            GRADE,
            &[row(&[
                "Strength of the recommendation",
                "strong",
                "not stated",
            ])],
        ),
        "decoded label absent as a raw PL span",
    );
    c.section(
        &page,
        PREFERENCE,
        &expected_section(
            PREFERENCE,
            &[preference_row(
                1,
                "statement",
                "as a fact",
                "not stated",
                "not stated",
                "none named",
            )],
        ),
        "decoded event/noun absent as raw PL spans",
    );
    for (kind, value, heading, meaning) in [
        (
            "strength",
            "strong",
            GRADE_WORDS,
            "strength of the recommendation: strong",
        ),
        (
            "evidence",
            "very_low",
            GRADE_WORDS,
            "certainty of the evidence: very low",
        ),
        (
            "preference",
            "preferred",
            PREFERENCE_WORDS,
            "marks the preferred option",
        ),
        (
            "alternative",
            "alternative",
            PREFERENCE_WORDS,
            "joins the option that the preferred option replaces",
        ),
    ] {
        let lemma = "x<&>\"'漢";
        fx.table(Some(&format!("{kind}\t{lemma}\t{value}\n")));
        c.section(
            &fx.request("/g/alpha/"),
            heading,
            &expected_section(heading, &[row(&[lemma, meaning])]),
            &format!("vocabulary escaping {kind}"),
        );
    }
    c.finish("U9");
}

#[test]
fn u10_new_sections_keep_mobile_and_print_frame_bytes() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    let mut clauses = timing();
    clauses.push_str(&facts(&grade(1, 1, "strength", "strong", "label")));
    clauses.push_str(&facts(&preference(
        1,
        50,
        "prescribe",
        "chosen-option",
        Some("other-option"),
        &["should", "-"],
    )));
    fx.document(&[clauses]);
    fx.table(Some("unit\tday\tday\nstrength\tlabel\tstrong\npreference\tpreferred\tpreferred\nalternative\tinstead-of\talternative\n"));
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
    for (label, page) in &pages {
        c.equal(
            &css(page),
            &expected_css,
            &format!("{label}: stylesheet byte identity (mobile + print included)"),
        );
        c.require(
            page.matches(VIEWPORT).count() == 1
                && page.contains(&format!("<meta charset=\"utf-8\">\n{VIEWPORT}\n<title>")),
            &format!("{label}: viewport position + uniqueness"),
        );
        c.require(
            page.contains(&format!("{NARROW}\n@media print {{\n")),
            &format!("{label}: narrow-table rule before print"),
        );
    }
    for (route, headings) in [
        ("g/alpha/doc/d.html", [GRADE, PREFERENCE]),
        ("g/alpha/index.html", [GRADE_WORDS, PREFERENCE_WORDS]),
    ] {
        for heading in headings {
            c.require(
                pages[route].matches(heading).count() == 1,
                &format!("{route}: frame check exercises {heading}"),
            );
        }
    }
    c.finish("U10; frame invariants control: green at base");
}

#[test]
fn u11_copula_actions_name_subjects_not_properties_or_options() {
    let fx = Fixture::plain();
    let mut c = Checks::default();
    for adjective in ["preferred", "first-line"] {
        for noun in ["NSAID", "nonopioid-therapy", "word:<&>\"'漢"] {
            let mut terms = preference(1, 1, "be", noun, None, &[]);
            for term in &mut terms {
                *term = term.replace(",preferred,pos)", &format!(",{},pos)", atom(adjective)));
            }
            terms.extend(preference(
                1,
                100,
                "prescribe",
                "therapy",
                Some("alternative"),
                &["should"],
            ));
            fx.document_with_comment(&[facts(&terms)], &format!("{noun} is {adjective}."));
            c.section(
                &fx.request(DOC_ROUTE),
                PREFERENCE,
                &expected_section(
                    PREFERENCE,
                    &[
                        preference_row(1, "statement", "as a fact", noun, noun, "none named"),
                        preference_row(
                            1,
                            "statement",
                            "should",
                            "prescribe",
                            "therapy",
                            "alternative",
                        ),
                    ],
                ),
                &format!("copula subject={noun} adjective={adjective}; ordinary verb unchanged"),
            );
        }
    }
    fx.document(&[concat!(
        "guideline_entity(actual,subject,'global-subject',countable).\n",
        "guideline_entity(actual,preferred,'preferred-option',countable).\n",
        "guideline_entity(actual,complement,'complement-noun',countable).\n",
        "guideline_arg(actual,e,2,complement).\n",
        "guideline_arg(actual,e,1,subject).\n",
        "guideline_event(actual,e,be).\n",
        "guideline_preference(actual,e,preferred,none) :- guideline_event(actual,e,be), guideline_arg(actual,e,1,local), guideline_entity(actual,local,'local-subject',countable).\n"
    ).into()]);
    c.section(
        &fx.request(DOC_ROUTE),
        PREFERENCE,
        &expected_section(
            PREFERENCE,
            &[preference_row(
                1,
                "statement",
                "as a fact",
                "local-subject",
                "preferred-option",
                "none named",
            )],
        ),
        "copula Action joins the clause-local subject, not the Preferred endpoint",
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
                "guideline_entity(actual,preferred,option,countable).\n",
                "guideline_event(actual,e,be).\n",
                "guideline_arg(actual,e,2,property).\n",
                "guideline_property(actual,property,preferred,pos).\n",
                "guideline_preference(actual,e,preferred,none).\n"
            ),
            args = args
        )]);
        c.section(
            &fx.request(DOC_ROUTE),
            PREFERENCE,
            &expected_section(
                PREFERENCE,
                &[preference_row(
                    1,
                    "statement",
                    "as a fact",
                    expected,
                    "option",
                    "none named",
                )],
            ),
            &format!("copula subject fallback: {args:?}"),
        );
    }
    c.finish("U12");
}
