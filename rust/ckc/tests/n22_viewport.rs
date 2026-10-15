// N22 D2/D3/P3: frame properties over committed fixture trees + generated requests.
// CKC_UI_TEST_BIN = candidate.
// CKC_N22_MIGRATION selects an alternate migration for black-box replay.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

const CHARSET: &str = "<meta charset=\"utf-8\">";
const VIEWPORT: &str = "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">";
const NARROW: &str =
    "@media screen and (max-width: 40rem) { table { display: block; overflow-x: auto; } }";
const PRINT: &str = "@media print {";
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
static RENDERED: OnceLock<BTreeMap<String, Vec<u8>>> = OnceLock::new();

type Files = BTreeMap<PathBuf, Vec<u8>>;
type Pages = BTreeMap<String, Vec<u8>>;

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

fn read(path: &Path) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
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
                out.insert(path.strip_prefix(base).unwrap().to_owned(), read(&path));
            }
        }
    }
    let mut out = Files::new();
    walk(base, base, &mut out);
    out
}

fn write(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

fn lines(path: &Path) -> Vec<String> {
    if !path.exists() {
        return Vec::new();
    }
    String::from_utf8(read(path))
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

fn cases(color: &str) -> Vec<PathBuf> {
    let mut out = fs::read_dir(root().join("tests/ui").join(color))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    out.sort();
    assert!(!out.is_empty(), "tests/ui/{color}: fixture inventory");
    for case in &out {
        assert!(case.join("case.tsv").is_file(), "{}", case.display());
    }
    out
}

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let base = root().join("rust/target/n22-viewport");
        fs::create_dir_all(&base).unwrap();
        loop {
            let path = base.join(format!(
                "{}-{}",
                std::process::id(),
                NEXT_TMP.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
                Err(e) => panic!("fixture scratch: {e}"),
            }
        }
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.0) {
            eprintln!("N22 cleanup {}: {e}", self.0.display());
        }
    }
}

struct Fixture {
    scratch: Scratch,
    case: PathBuf,
    tree: PathBuf,
}
impl Fixture {
    fn new(case: &Path) -> Self {
        let fx = Self {
            scratch: Scratch::new(),
            case: case.to_owned(),
            tree: case.strip_prefix(root()).unwrap().join("tree"),
        };
        fx.reset();
        fx
    }

    fn reset(&self) {
        let tree = self.scratch.0.join(&self.tree);
        if tree.exists() {
            fs::remove_dir_all(&tree).unwrap();
        }
        fs::create_dir_all(&tree).unwrap();
        let input = files(&self.case.join("tree"));
        let order = if self.case.join("tree-order.txt").exists() {
            lines(&self.case.join("tree-order.txt"))
                .into_iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>()
        } else {
            input.keys().cloned().collect()
        };
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(sorted, input.keys().cloned().collect::<Vec<_>>());
        for rel in order {
            write(&tree.join(&rel), &input[&rel]);
        }
        for dir in lines(&self.case.join("empty-dirs.txt")) {
            fs::create_dir_all(tree.join(dir)).unwrap();
        }
        if self.case.join("worktree").exists() {
            for args in [
                vec!["init", "-q", "-b", "main"],
                vec!["add", "-A"],
                vec!["commit", "-q", "-m", "fixture corpus"],
            ] {
                let result = Command::new("git")
                    .args(&args)
                    .current_dir(&tree)
                    .envs(GIT_ENV)
                    .output()
                    .unwrap();
                assert!(
                    result.status.success(),
                    "fixture git {args:?}: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
            }
            for (rel, data) in files(&self.case.join("worktree")) {
                write(&tree.join(rel), &data);
            }
        }
    }

    fn label(&self) -> String {
        self.case
            .strip_prefix(root().join("tests/ui"))
            .unwrap()
            .display()
            .to_string()
    }

    fn invoke(&self, bin: &Path, args: &[String]) -> Output {
        Command::new(bin)
            .arg("ui")
            .args(args)
            .current_dir(&self.scratch.0)
            .envs(GIT_ENV)
            .env("GIT_CEILING_DIRECTORIES", &self.scratch.0)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("TMPDIR", &self.scratch.0)
            .output()
            .unwrap_or_else(|e| panic!("{} {args:?}: {e}", bin.display()))
    }

    fn render(&self, bin: &Path) -> Files {
        let out = self.scratch.0.join("out");
        if out.exists() {
            fs::remove_dir_all(&out).unwrap();
        }
        let result = self.invoke(
            bin,
            &[
                "render".into(),
                "out".into(),
                self.tree.to_str().unwrap().into(),
            ],
        );
        assert_eq!(
            result.status.code(),
            Some(0),
            "{} render: {}",
            self.label(),
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(result.stderr.is_empty(), "{} render stderr", self.label());
        let output = files(&out);
        assert!(output.contains_key(Path::new("index.html")));
        let golden = self.case.join("golden");
        if golden.is_dir() {
            assert_eq!(
                output.keys().collect::<Vec<_>>(),
                files(&golden).keys().collect::<Vec<_>>(),
                "{} render inventory",
                self.label()
            );
        }
        output
    }

    fn requests(&self, bin: &Path) -> Pages {
        let mut pages = Pages::new();
        for (row, line) in lines(&self.case.join("case.tsv")).iter().enumerate() {
            let fields = line.split('\t').collect::<Vec<_>>();
            assert_eq!(fields.len(), 4, "{} case row", self.label());
            if !fields[0].starts_with("request ") {
                continue;
            }
            let args = fields[0]
                .split(' ')
                .map(|arg| match arg {
                    "@TREE@" => self.tree.to_str().unwrap().to_owned(),
                    other => other.to_owned(),
                })
                .collect::<Vec<_>>();
            let label = format!("{} request[{row}] {}", self.label(), fields[3]);
            let result = self.invoke(bin, &args);
            assert_eq!(
                result.status.code(),
                Some(fields[1].parse::<i32>().unwrap()),
                "{label}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            if fields[3] == "-" {
                assert!(result.stdout.is_empty(), "{label}: expected no body");
                continue;
            }
            let expected = read(&self.case.join(fields[3]));
            let (expected_head, expected_body) = response(&expected, &label);
            let (head, body) = response(&result.stdout, &label);
            assert_eq!(head.lines().next(), expected_head.lines().next(), "{label}");
            if expected_body.starts_with(b"<!doctype html>") {
                assert!(body.starts_with(b"<!doctype html>"), "{label}: HTML body");
                pages.insert(
                    format!("{label}: {}", head.lines().next().unwrap()),
                    body.to_vec(),
                );
            }
        }
        pages
    }
}

fn response<'a>(raw: &'a [u8], label: &str) -> (&'a str, &'a [u8]) {
    let split = raw
        .windows(2)
        .position(|pair| pair == b"\n\n")
        .unwrap_or_else(|| panic!("{label}: HTTP response boundary"));
    (
        std::str::from_utf8(&raw[..split]).unwrap(),
        &raw[split + 2..],
    )
}

fn page_path(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "html")
        && !path
            .components()
            .nth(2)
            .is_some_and(|part| part.as_os_str() == "source")
}

fn rendered() -> &'static Pages {
    RENDERED.get_or_init(|| {
        let mut pages = Pages::new();
        let bin = program();
        let cases = cases("green");
        for case in &cases {
            let fx = Fixture::new(case);
            for (path, data) in fx.render(&bin) {
                if page_path(&path) {
                    pages.insert(format!("{}/{}", fx.label(), path.display()), data);
                }
            }
        }
        assert!(!pages.is_empty());
        eprintln!(
            "N22 render census: {} fixtures, {} pages",
            cases.len(),
            pages.len()
        );
        pages
    })
}

fn viewport_errors(page: &str) -> Vec<&'static str> {
    let mut errors = Vec::new();
    if page.matches(VIEWPORT).count() != 1 || page.matches("<meta name=\"viewport\"").count() != 1 {
        errors.push("expected exactly one viewport meta");
    }
    let head = page
        .split_once("<head>\n")
        .and_then(|(_, s)| s.split_once("</head>"));
    if head.is_none_or(|(s, _)| !s.contains(&format!("{CHARSET}\n{VIEWPORT}\n<title>"))) {
        errors.push("expected viewport line directly after charset and before title in head");
    }
    errors
}

fn narrow_errors(page: &str) -> Vec<&'static str> {
    let Some((_, tail)) = page.split_once("<style>\n") else {
        return vec!["expected stylesheet"];
    };
    let Some((css, _)) = tail.split_once("</style>") else {
        return vec!["expected stylesheet end"];
    };
    let mut errors = Vec::new();
    if css.matches(NARROW).count() != 1 {
        errors.push("expected exactly one narrow-screen table rule");
    }
    if !css.contains(&format!("\n{NARROW}\n{PRINT}\n")) {
        errors.push("expected narrow-screen rule line directly before print rule");
    }
    errors
}

fn check_pages(pages: &Pages, check: fn(&str) -> Vec<&'static str>) {
    let problems = pages
        .iter()
        .filter_map(|(label, data)| {
            let errors = check(std::str::from_utf8(data).unwrap());
            (!errors.is_empty()).then(|| format!("{label}: {}", errors.join("; ")))
        })
        .collect::<Vec<_>>();
    assert!(
        problems.is_empty(),
        "{} of {} pages failed:\n{}",
        problems.len(),
        pages.len(),
        problems.join("\n")
    );
}

fn generated_requests(bin: &Path) -> Pages {
    let fx = Fixture::new(&root().join("tests/ui/green/basic"));
    let alphabet = ['a', '0', ' ', '"', '\'', '&', '<', '>', '漢', 'λ', '\n'];
    let routes = [
        "/",
        "/g/alpha/",
        "/g/alpha/records.html",
        "/g/alpha/doc/a-10.html",
    ];
    let mut pages = Pages::new();
    for (route, path) in routes.iter().enumerate() {
        for len in [0, 1, 63, 64, 65, 256] {
            let token = (0..len)
                .map(|i| alphabet[(i * 7 + route) % alphabet.len()])
                .collect::<String>();
            let label = format!("generated GET {path} token-length={len}");
            let result = fx.invoke(
                bin,
                &[
                    "request".into(),
                    "GET".into(),
                    (*path).into(),
                    fx.tree.to_str().unwrap().into(),
                    "--token".into(),
                    token,
                ],
            );
            assert_eq!(result.status.code(), Some(0), "{label}");
            assert!(result.stderr.is_empty(), "{label}: stderr");
            let (head, body) = response(&result.stdout, &label);
            assert_eq!(head.lines().next(), Some("HTTP 200"), "{label}");
            assert!(body.starts_with(b"<!doctype html>"), "{label}: HTML");
            pages.insert(label, body.to_vec());
        }
    }
    pages
}

#[test]
fn t1_rendered_pages_have_one_viewport_after_charset() {
    check_pages(rendered(), viewport_errors);
}

#[test]
fn t2_rendered_stylesheets_have_one_narrow_rule_before_print() {
    check_pages(rendered(), narrow_errors);
}

#[test]
fn t3_request_page_and_error_bodies_keep_frame_properties() {
    let mut pages = Pages::new();
    let bin = program();
    for case in cases("green").into_iter().chain(cases("red")) {
        if lines(&case.join("case.tsv"))
            .iter()
            .any(|row| row.starts_with("request "))
        {
            pages.extend(Fixture::new(&case).requests(&bin));
        }
    }
    let statuses = pages
        .keys()
        .filter_map(|label| label.rsplit_once(": HTTP ").map(|(_, code)| code))
        .collect::<BTreeSet<_>>();
    for required in ["200", "303", "400", "403", "404", "405", "409", "500"] {
        assert!(
            statuses.contains(required),
            "missing HTTP {required} page coverage"
        );
    }
    let generated = generated_requests(&bin);
    eprintln!(
        "N22 request census: {} fixture HTML bodies, {} generated bodies; HTTP {statuses:?}",
        pages.len(),
        generated.len()
    );
    pages.extend(generated);
    check_pages(&pages, |page| {
        let mut errors = viewport_errors(page);
        errors.extend(narrow_errors(page));
        errors
    });
}

fn without_additions(page: &str) -> String {
    page.replacen(
        &format!("{CHARSET}\n{VIEWPORT}\n"),
        &format!("{CHARSET}\n"),
        1,
    )
    .replacen(&format!("{NARROW}\n{PRINT}\n"), &format!("{PRINT}\n"), 1)
}

fn migration_page(path: &Path, bytes: &[u8]) -> bool {
    let expected_file = path.components().any(|p| p.as_os_str() == "golden")
        || path
            .extension()
            .is_some_and(|ext| ext == "golden" || ext == "expect");
    if !expected_file
        || path
            .components()
            .any(|p| matches!(p.as_os_str().to_str(), Some("tree" | "worktree")))
    {
        return false;
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    text.starts_with("<!doctype html>")
        || (text.starts_with("HTTP ")
            && text
                .split_once("\n\n")
                .is_some_and(|(_, body)| body.starts_with("<!doctype html>")))
}

fn add_contract_lines(page: &str) -> String {
    assert!(
        !page
            .split_once("</head>")
            .expect("page head")
            .0
            .contains(VIEWPORT)
    );
    page.replacen(
        &format!("{CHARSET}\n"),
        &format!("{CHARSET}\n{VIEWPORT}\n"),
        1,
    )
    .replacen(&format!("{PRINT}\n"), &format!("{NARROW}\n{PRINT}\n"), 1)
}

fn same_files(actual: &Files, expected: &Files, label: &str) {
    assert_eq!(
        actual.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>(),
        "{label}: file inventory"
    );
    for (path, bytes) in expected {
        assert!(
            actual[path] == *bytes,
            "{label}: {}: expected {} bytes, actual {} bytes; only the two contract lines may change",
            path.display(),
            bytes.len(),
            actual[path].len()
        );
    }
}

#[test]
fn t5_migration_changes_only_page_lines_and_is_idempotent() {
    let migration = Path::new(".agent/archive/migrations/n22-viewport.py");
    let source = std::env::var_os("CKC_N22_MIGRATION")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join(migration));
    assert!(
        source.is_file(),
        "N22 migration missing: {}",
        source.display()
    );
    let work = Scratch::new();
    let target = work.0.join("target-root");
    let script = work.0.join(migration);
    write(&script, &read(&source));
    let input = files(&root().join("tests/ui"));
    let mut page_count = 0;
    for (path, bytes) in &input {
        let data = if migration_page(path, bytes) {
            page_count += 1;
            without_additions(std::str::from_utf8(bytes).unwrap()).into_bytes()
        } else {
            bytes.clone()
        };
        write(&target.join("tests/ui").join(path), &data);
    }
    assert!(page_count > 0, "migration page census");
    let example = without_additions(
        std::str::from_utf8(&read(
            &root().join("tests/ui/green/basic/golden/index.html"),
        ))
        .unwrap(),
    );
    for (name, text) in [
        ("tests/ui/green/n22-probe/tree/source.txt", example.clone()),
        ("tests/ui/green/n22-probe/notes.txt", example.clone()),
        ("outside-ui.html", example.clone()),
        (
            "tests/ui/green/n22-probe/non-page.expect",
            format!("not a page\n{CHARSET}\n{PRINT}\n"),
        ),
    ] {
        write(&target.join(name), text.as_bytes());
    }
    for (i, payload) in ["", "plain", "α漢字", "\n\n", NARROW].iter().enumerate() {
        let page = example.replacen("</main>", &format!("<p>{payload}</p>\n</main>"), 1);
        assert!(page.contains(&format!("<p>{payload}</p>")));
        write(
            &target.join(format!(
                "tests/ui/green/n22-probe/golden/g/probe/doc/{i}.html"
            )),
            page.as_bytes(),
        );
        page_count += 1;
    }
    write(
        &work.0.join("tests/ui/caller/golden/index.html"),
        example.as_bytes(),
    );
    // A separate indexed caller root catches cwd selection and bounds old variants.
    for args in [vec!["init", "-q", "-b", "main"], vec!["add", "-A"]] {
        let result = Command::new("git")
            .args(&args)
            .current_dir(&work.0)
            .envs(GIT_ENV)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "migration fixture git {args:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    let before = files(&work.0);
    let expected = before
        .iter()
        .map(|(path, data)| {
            let output = if path.starts_with("target-root/tests/ui") && migration_page(path, data) {
                add_contract_lines(std::str::from_utf8(data).unwrap()).into_bytes()
            } else {
                data.clone()
            };
            (path.clone(), output)
        })
        .collect::<Files>();
    let run = |args: &[&Path]| {
        Command::new("python3")
            .arg("-I")
            .arg(&script)
            .args(args)
            .current_dir(&work.0)
            .envs(GIT_ENV)
            .env("GIT_CEILING_DIRECTORIES", work.0.parent().unwrap())
            .env("PYTHONDONTWRITEBYTECODE", "1")
            .output()
            .unwrap()
    };
    let missing = work.0.join("missing-root");
    for args in [vec![], vec![missing.as_path()]] {
        let result = run(&args);
        assert_eq!(
            result.status.code(),
            Some(1),
            "migration requires an explicit root containing tests/ui: {args:?}"
        );
        same_files(
            &files(&work.0),
            &before,
            "migration invalid root writes nothing",
        );
    }
    for pass in 1..=2 {
        let result = run(&[target.as_path()]);
        assert!(
            result.status.success(),
            "migration pass {pass}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        same_files(
            &files(&work.0),
            &expected,
            &format!("migration pass {pass}"),
        );
        let changed = if pass == 1 { page_count } else { 0 };
        assert_eq!(
            result.stdout,
            format!("n22: {changed} of {page_count} page files\n").as_bytes(),
            "migration pass {pass}: census"
        );
    }
    eprintln!(
        "N22 migration census: {page_count} page files, explicit target root, unchanged caller/nonpage files, byte-identical second pass"
    );
}
