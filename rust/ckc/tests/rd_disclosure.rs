// RD D1/D2/D4/P3: contract literal + frozen fixture inputs; renderer implementation unread.
// CKC_UI_TEST_BIN selects the candidate; CKC_RD_MIGRATION selects the replay script.
#[path = "../../../tests/frozen-pages/rev_k.rs"]
mod rev_k;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

const BASE: &str = "065d814b1acc43992bfbb36f4b356da411d7b61e";
const NOTE: &str = "<p>Your name and comment are published with your decision in the knowledge base and its public repository.</p>";
const NAME: &str = "p_open_your_name_and_comment_are_published";
const ANCHOR: &str = "</fieldset>\n<label for=\"reviewer\">Reviewer name</label>";
const REVIEWER: &str = "<label for=\"reviewer\">Reviewer name</label>";
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
static BASELINE: OnceLock<Files> = OnceLock::new();
static RENDERED: OnceLock<Files> = OnceLock::new();
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
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}

fn read(path: &Path) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn text(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).expect("UTF-8 test data")
}

// The 065d814b goldens + the non-document pins hold pre-REV-K renderer bytes: a current page's
// REV-K sites map back before the comparison.
fn pre_rev_k(page: &[u8]) -> Vec<u8> {
    rev_k::map(text(page), false).into_bytes()
}

fn write(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

fn lines(bytes: &[u8]) -> Vec<&str> {
    text(bytes)
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect()
}

fn files(base: &Path) -> Files {
    fn walk(base: &Path, here: &Path, out: &mut Files) {
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

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let base = root().join("rust/target/rd-disclosure");
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
                Err(e) => panic!("RD scratch: {e}"),
            }
        }
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.0) {
            eprintln!("RD cleanup {}: {e}", self.0.display());
        }
    }
}

fn git(cwd: &Path, args: &[&str]) {
    let result = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .envs(GIT_ENV)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "fixture git {args:?}: {}",
        text(&result.stderr)
    );
}

fn baseline() -> &'static Files {
    BASELINE.get_or_init(|| {
        let work = Scratch::new();
        let archive = work.0.join("base.tar");
        git(
            &root(),
            &[
                "archive",
                "--format=tar",
                "-o",
                archive.to_str().unwrap(),
                BASE,
                "tests/ui",
            ],
        );
        let out = Command::new("tar")
            .args(["-xf", archive.to_str().unwrap(), "-C"])
            .arg(&work.0)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "fixture archive: {}",
            text(&out.stderr)
        );
        files(&work.0.join("tests/ui"))
    })
}

fn cases(color: &str) -> Vec<PathBuf> {
    let out = baseline()
        .keys()
        .filter(|p| p.starts_with(color) && p.file_name().unwrap() == "case.tsv")
        .map(|p| p.parent().unwrap().to_owned())
        .collect::<Vec<_>>();
    assert!(!out.is_empty(), "{color}: fixture census");
    out
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
            tree: Path::new("tests/ui").join(case).join("tree"),
        };
        let tree = fx.scratch.0.join(&fx.tree);
        fs::create_dir_all(&tree).unwrap();
        let input = baseline()
            .iter()
            .filter_map(|(path, data)| {
                path.strip_prefix(case.join("tree"))
                    .ok()
                    .map(|p| (p.to_owned(), data))
            })
            .collect::<BTreeMap<_, _>>();
        let order = baseline()
            .get(&case.join("tree-order.txt"))
            .map(|data| {
                lines(data)
                    .into_iter()
                    .map(PathBuf::from)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| input.keys().cloned().collect());
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(sorted, input.keys().cloned().collect::<Vec<_>>());
        for path in order {
            write(&tree.join(&path), input[&path]);
        }
        if let Some(data) = baseline().get(&case.join("empty-dirs.txt")) {
            for dir in lines(data) {
                fs::create_dir_all(tree.join(dir)).unwrap();
            }
        }
        let overlay = case.join("worktree");
        if baseline().keys().any(|p| p.starts_with(&overlay)) {
            git(&tree, &["init", "-q", "-b", "main"]);
            git(&tree, &["add", "-A"]);
            git(&tree, &["commit", "-q", "-m", "fixture corpus"]);
            for (path, data) in baseline() {
                if let Ok(rel) = path.strip_prefix(&overlay) {
                    write(&tree.join(rel), data);
                }
            }
        }
        fx
    }

    fn invoke(&self, args: &[String]) -> Output {
        Command::new(program())
            .arg("ui")
            .args(args)
            .current_dir(&self.scratch.0)
            .envs(GIT_ENV)
            .env("GIT_CEILING_DIRECTORIES", &self.scratch.0)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("TMPDIR", &self.scratch.0)
            .output()
            .unwrap_or_else(|e| panic!("candidate {args:?}: {e}"))
    }

    fn get(&self, route: &str, extra: &[String]) -> Vec<u8> {
        let mut args = vec![
            "request".into(),
            "GET".into(),
            route.into(),
            self.tree.to_str().unwrap().into(),
        ];
        args.extend_from_slice(extra);
        let out = self.invoke(&args);
        assert_eq!(
            out.status.code(),
            Some(0),
            "GET {route}: {}",
            text(&out.stderr)
        );
        assert!(out.stderr.is_empty(), "GET {route}: stderr");
        out.stdout
    }

    fn render(&self) -> Files {
        let out = self.invoke(&[
            "render".into(),
            "out".into(),
            self.tree.to_str().unwrap().into(),
        ]);
        assert_eq!(
            out.status.code(),
            Some(0),
            "{} render: {}",
            self.case.display(),
            text(&out.stderr)
        );
        assert!(out.stderr.is_empty(), "render stderr");
        let actual = files(&self.scratch.0.join("out"));
        let prefix = self.case.join("golden");
        let expected = baseline()
            .keys()
            .filter_map(|p| p.strip_prefix(&prefix).ok())
            .collect::<Vec<_>>();
        if !expected.is_empty() {
            assert_eq!(
                actual.keys().map(PathBuf::as_path).collect::<Vec<_>>(),
                expected,
                "{}: golden inventory",
                self.case.display()
            );
        }
        assert!(actual.contains_key(Path::new("index.html")), "render index");
        actual
    }
}

fn rendered() -> &'static Files {
    RENDERED.get_or_init(|| {
        let mut out = Files::new();
        for case in cases("green") {
            let fx = Fixture::new(&case);
            for (path, bytes) in fx.render() {
                out.insert(case.join("golden").join(path), bytes);
            }
        }
        out
    })
}

fn response(raw: &[u8]) -> (&str, &[u8]) {
    let split = raw
        .windows(2)
        .position(|p| p == b"\n\n")
        .expect("HTTP boundary");
    (text(&raw[..split]), &raw[split + 2..])
}

fn html_body(bytes: &[u8]) -> Option<&[u8]> {
    let body = if bytes.starts_with(b"HTTP ") {
        response(bytes).1
    } else {
        bytes
    };
    body.starts_with(b"<!doctype html>").then_some(body)
}

fn document_path(path: &Path) -> bool {
    path.extension().is_some_and(|e| e == "html")
        && path
            .parent()
            .and_then(Path::file_name)
            .is_some_and(|s| s == "doc")
}

fn note_errors(page: &str) -> Vec<&'static str> {
    let mut errors = Vec::new();
    if page.matches(NOTE).count() != 1 {
        errors.push("expected exactly one D1 disclosure");
    }
    let form = page
        .split_once("<form ")
        .and_then(|(_, tail)| tail.split_once("</form>"));
    if page.matches("<form ").count() != 1 || page.matches("</form>").count() != 1 {
        errors.push("expected one enclosing form");
    }
    let anchor = format!("</fieldset>\n{NOTE}\n{REVIEWER}");
    if form.is_none_or(|(inner, _)| !inner.contains(&anchor)) {
        errors.push("expected D1 inside form directly between fieldset close and reviewer label");
    }
    errors
}

fn check_notes(pages: &Files) {
    assert!(!pages.is_empty(), "document positive control");
    let errors = pages
        .iter()
        .filter_map(|(path, bytes)| {
            let errors = note_errors(text(bytes));
            (!errors.is_empty()).then(|| format!("{}: {}", path.display(), errors.join("; ")))
        })
        .collect::<Vec<_>>();
    assert!(
        errors.is_empty(),
        "RD disclosure: {} of {} documents failed:\n{}",
        errors.len(),
        pages.len(),
        errors.join("\n")
    );
}

fn add_note(bytes: &[u8]) -> Vec<u8> {
    let page = text(bytes);
    assert_eq!(page.matches(ANCHOR).count(), 1, "frozen carrier anchor");
    assert!(!page.contains(NOTE), "base disclosure already present");
    page.replacen(ANCHOR, &format!("</fieldset>\n{NOTE}\n{REVIEWER}"), 1)
        .into_bytes()
}

fn input_documents() -> usize {
    baseline()
        .keys()
        .filter(|path| {
            path.starts_with("green")
                && path
                    .components()
                    .nth(2)
                    .is_some_and(|part| part.as_os_str() == "tree")
                && path
                    .parent()
                    .and_then(Path::file_name)
                    .is_some_and(|part| part == "ace")
                && path.extension().is_some_and(|ext| ext == "ace")
        })
        .count()
}

#[test]
fn t1_static_documents_have_one_disclosure_at_the_form_anchor() {
    let documents = rendered()
        .iter()
        .filter(|(path, _)| document_path(path))
        .map(|(path, data)| (path.clone(), data.clone()))
        .collect::<Files>();
    assert_eq!(cases("green").len(), 21, "frozen render case census");
    assert_eq!(input_documents(), 48, "frozen input document census");
    assert_eq!(
        documents.len(),
        input_documents(),
        "all input documents render"
    );
    eprintln!("RD render census: 21 fixtures, 48 document pages (45 frozen goldens)");
    check_notes(&documents);
    for (path, data) in documents {
        if let Some(before) = baseline().get(&path) {
            assert!(
                pre_rev_k(&data) == add_note(before),
                "{}: only D1 changes",
                path.display()
            );
        }
    }
}

fn absent(page: &[u8], label: &str) {
    assert!(
        !text(page).contains(NOTE),
        "{label}: disclosure outside a document form"
    );
    assert!(
        !text(page).contains("<form "),
        "{label}: unexpected document form"
    );
}

fn pinned_non_document(page: &[u8], name: &str) {
    absent(page, name);
    let pins = read(&root().join("tests/rd/non-document.sha256"));
    let rows = lines(&pins)
        .into_iter()
        .map(|line| line.split_once('\t').unwrap())
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 6, "non-document pin census");
    let expected = rows
        .iter()
        .find(|(_, key)| *key == name)
        .expect("session pin")
        .0;
    assert_eq!(
        Sha256::digest(pre_rev_k(page))
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>(),
        expected,
        "{name}: unchanged body"
    );
}

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn signin() -> Vec<u8> {
    let fx = Fixture::new(Path::new("green/basic"));
    let run = fx.scratch.0.join("run");
    fs::create_dir(&run).unwrap();
    fs::set_permissions(&run, fs::Permissions::from_mode(0o700)).unwrap();
    let port = TcpListener::bind(("127.0.0.1", 0))
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let stdout = fx.scratch.0.join("server.stdout");
    let stderr = fx.scratch.0.join("server.stderr");
    let mut server = Server(
        Command::new(program())
            .args(["ui", "serve", &port.to_string(), fx.tree.to_str().unwrap()])
            .current_dir(&fx.scratch.0)
            .envs(GIT_ENV)
            .env("GIT_CEILING_DIRECTORIES", &fx.scratch.0)
            .env("TMPDIR", &fx.scratch.0)
            .env("XDG_RUNTIME_DIR", &run)
            .env("BROWSER", "/bin/true")
            .env_remove("DISPLAY")
            .env_remove("WAYLAND_DISPLAY")
            .env_remove("WSL_DISTRO_NAME")
            .stdin(Stdio::null())
            .stdout(fs::File::create(&stdout).unwrap())
            .stderr(fs::File::create(&stderr).unwrap())
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(120);
    let file = run.join(format!("ckc-ui-{port}.html"));
    let page = loop {
        if let Ok(page) = fs::read_to_string(&file)
            && page.ends_with("</html>\n")
        {
            break page;
        }
        assert!(
            server.0.try_wait().unwrap().is_none(),
            "sign-in server exited: {}",
            text(&read(&stderr))
        );
        assert!(
            Instant::now() < deadline,
            "sign-in startup timeout: {}",
            text(&read(&stdout))
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    let address = page
        .split_once("content=\"0; url=")
        .expect("sign-in refresh")
        .1
        .split_once('"')
        .unwrap()
        .0;
    let session = address
        .strip_prefix(&format!("http://127.0.0.1:{port}/session/"))
        .expect("sign-in URL");
    assert_eq!(session.len(), 64);
    assert!(
        session
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    );
    let normalized = page.replace(address, "@SIGNIN_URL@").into_bytes();
    drop(server);
    assert!(
        TcpStream::connect(("127.0.0.1", port)).is_err(),
        "sign-in server stopped"
    );
    normalized
}

#[test]
fn t2_only_document_pages_change_including_session_and_refusal_bodies() {
    let mut kinds = BTreeSet::new();
    let mut static_count = 0;
    for (path, bytes) in rendered() {
        if document_path(path) || html_body(bytes).is_none() {
            continue;
        }
        absent(bytes, &path.display().to_string());
        if let Some(before) = baseline().get(path) {
            assert!(
                pre_rev_k(bytes) == *before,
                "{}: changed non-document bytes",
                path.display()
            );
        } else {
            pinned_non_document(bytes, path.to_str().unwrap());
        }
        kinds.insert(if path.ends_with("golden/index.html") {
            "index"
        } else if path.file_name().unwrap() == "records.html" {
            "records"
        } else {
            "guideline"
        });
        static_count += 1;
    }
    assert_eq!(kinds, BTreeSet::from(["guideline", "index", "records"]));
    let mut statuses = BTreeSet::new();
    let mut request_count = 0;
    for case in cases("green").into_iter().chain(cases("red")) {
        let rows = lines(&baseline()[&case.join("case.tsv")]);
        if !rows.iter().any(|row| row.starts_with("request ")) {
            continue;
        }
        let fx = Fixture::new(&case);
        for line in rows {
            let fields = line.split('\t').collect::<Vec<_>>();
            assert_eq!(fields.len(), 4, "fixture case row");
            if !fields[0].starts_with("request ") {
                continue;
            }
            let args = fields[0]
                .split(' ')
                .map(|arg| {
                    if arg == "@TREE@" {
                        fx.tree.to_str().unwrap().to_owned()
                    } else {
                        arg.to_owned()
                    }
                })
                .collect::<Vec<_>>();
            let result = fx.invoke(&args);
            let label = format!("{} {}", case.display(), fields[3]);
            assert_eq!(
                result.status.code(),
                Some(fields[1].parse::<i32>().unwrap()),
                "{label}: rc"
            );
            if fields[3] == "-" {
                assert!(result.stdout.is_empty(), "{label}: empty stdout");
                continue;
            }
            let expected = &baseline()[&case.join(fields[3])];
            let Some(expected_body) = html_body(expected) else {
                continue;
            };
            if text(expected_body).contains("<form ") {
                continue;
            }
            let (head, body) = response(&result.stdout);
            let (expected_head, _) = response(expected);
            assert_eq!(head.lines().next(), expected_head.lines().next(), "{label}");
            absent(body, &label);
            assert!(
                pre_rev_k(body) == expected_body,
                "{label}: changed non-document body"
            );
            statuses.insert(head.lines().next().unwrap().to_owned());
            request_count += 1;
        }
    }
    for status in [
        "HTTP 200", "HTTP 303", "HTTP 400", "HTTP 403", "HTTP 404", "HTTP 405", "HTTP 409",
        "HTTP 500",
    ] {
        assert!(
            statuses.contains(status),
            "missing {status} negative-page coverage"
        );
    }
    let fx = Fixture::new(Path::new("green/basic"));
    let session = "a".repeat(64);
    for (name, route, status) in [
        ("session-403", "/".to_owned(), "HTTP 403"),
        ("session-303", format!("/session/{session}"), "HTTP 303"),
    ] {
        let raw = fx.get(&route, &["--session".into(), session.clone()]);
        let (head, body) = response(&raw);
        assert_eq!(head.lines().next(), Some(status));
        pinned_non_document(body, name);
    }
    pinned_non_document(&signin(), "signin");
    eprintln!(
        "RD negative census: {static_count} static pages, {request_count} fixture bodies, 3 session/sign-in bodies; HTTP {statuses:?}"
    );
    // The negative predicate needs a live positive control: an entirely old renderer is not green.
    let raw = fx.get("/g/alpha/doc/a-10.html", &[]);
    check_notes(&Files::from([(
        PathBuf::from("T2 document positive control"),
        response(&raw).1.to_vec(),
    )]));
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn hostile_reviewers(fx: &Fixture, payload: &str) -> Vec<String> {
    let path = fx
        .scratch
        .0
        .join(&fx.tree)
        .join("guidelines/alpha/audit/adjudication.tsv");
    let input = read(&path);
    let mut names = Vec::new();
    let rows = text(&input)
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if line.starts_with('#') {
                return line.to_owned();
            }
            let mut fields = line.split('\t').map(str::to_owned).collect::<Vec<_>>();
            assert_eq!(fields.len(), 7, "reviewer input row");
            fields[4] = format!("R{i}{payload}");
            fields[6] = format!("C{i}{payload} {NOTE}");
            names.push(fields[4].clone());
            fields.join("\t")
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    write(&path, rows.as_bytes());
    names
}

#[test]
fn t3_generated_gets_preserve_disclosure_across_states_routes_and_hostile_data() {
    let alphabet = ['a', '0', ' ', '"', '\'', '&', '<', '>', '漢', 'λ', '\n'];
    let mut pages = Files::new();
    let states = ["approved", "rejected", "contested", "stale", "unreviewed"];
    for (variant, length) in [0, 1, 63, 64, 65, 256].into_iter().enumerate() {
        let fx = Fixture::new(Path::new("green/verdicts"));
        let hostile = ["plain", "\"<&>'漢λ", "</form><p>name</p>"][variant % 3];
        let names = hostile_reviewers(&fx, hostile);
        for (state_index, state) in states.iter().enumerate() {
            let canonical = format!("/g/alpha/doc/{state}.html");
            let encoded = format!(
                "/g/%61lpha/doc/%{:02x}{}.html",
                state.as_bytes()[0],
                &state[1..]
            );
            for route in [
                canonical.clone(),
                format!("{canonical}?ignored=%3Cform%3E"),
                encoded,
            ] {
                let token = (0..length)
                    .map(|i| alphabet[(i * 7 + state_index) % alphabet.len()])
                    .collect::<String>();
                let label = format!(
                    "state={state} GET={route} token-length={length} reviewer-variant={variant}"
                );
                let raw = fx.get(&route, &["--token".into(), token.clone()]);
                let (head, body) = response(&raw);
                assert_eq!(head.lines().next(), Some("HTTP 200"), "{label}");
                let page = text(body);
                assert!(
                    page.contains(&format!("class=\"chip chip-{state}\"")),
                    "{label}: decision state"
                );
                assert!(
                    page.contains(&format!("name=\"csrf\" value=\"{}\"", escape(&token))),
                    "{label}: token reaches form escaped"
                );
                for name in &names {
                    assert!(
                        page.contains(&format!("<option value=\"{}\">", escape(name))),
                        "{label}: reviewer reaches form escaped"
                    );
                }
                pages.insert(PathBuf::from(label), body.to_vec());
            }
        }
    }
    assert_eq!(
        pages.len(),
        90,
        "5 states × 3 routes × 6 token/data boundaries"
    );
    let hostile = Fixture::new(Path::new("red/verdict-get-form"));
    for token in [String::new(), "<&>\"'".into(), NOTE.into()] {
        let raw = hostile.get("/g/alpha/doc/a-10.html", &["--token".into(), token.clone()]);
        assert_eq!(response(&raw).0.lines().next(), Some("HTTP 200"));
        pages.insert(
            PathBuf::from(format!("hostile seed token={token:?}")),
            response(&raw).1.to_vec(),
        );
    }
    eprintln!("RD generated GET census: 90 cross-product + 3 hostile-seed bodies");
    check_notes(&pages);
}

fn migration_carrier(path: &Path, bytes: &[u8]) -> bool {
    let path = path.strip_prefix("target-root/tests/ui").unwrap_or(path);
    let expected = path.components().any(|p| p.as_os_str() == "golden")
        || path
            .extension()
            .is_some_and(|s| s == "golden" || s == "expect");
    expected
        && !path
            .components()
            .nth(2)
            .is_some_and(|p| matches!(p.as_os_str().to_str(), Some("tree" | "worktree")))
        && html_body(bytes).is_some_and(|body| text(body).contains(ANCHOR))
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
            "{label}: {}: only D1 may change",
            path.display()
        );
    }
}

#[test]
fn t4_migration_changes_52_carriers_only_and_is_idempotent() {
    let carriers = baseline()
        .iter()
        .filter(|(path, bytes)| migration_carrier(path, bytes))
        .collect::<Vec<_>>();
    assert_eq!(carriers.len(), 52, "independent frozen carrier census");
    assert_eq!(
        carriers
            .iter()
            .filter(|(path, _)| document_path(path))
            .count(),
        45,
        "document carriers"
    );
    assert_eq!(
        carriers
            .iter()
            .filter(|(_, bytes)| bytes.starts_with(b"HTTP 200\n"))
            .count(),
        7,
        "HTTP-200 carriers"
    );
    let assertions = baseline()
        .iter()
        .filter(|(p, _)| p.file_name().unwrap() == "assertions.tsv")
        .collect::<Vec<_>>();
    assert_eq!(assertions.len(), 13);
    assert_eq!(
        assertions
            .iter()
            .map(|(_, b)| lines(b).len())
            .sum::<usize>(),
        122
    );
    eprintln!(
        "RD migration census: 52 = 45 document pages + 7 HTTP-200 bodies; 13 assertion files, 122 rows"
    );
    let source = std::env::var_os("CKC_RD_MIGRATION")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join(".agent/archive/migrations/rd-publication-note.py"));
    assert!(
        source.is_file(),
        "RD migration missing: {}",
        source.display()
    );
    let work = Scratch::new();
    let target = work.0.join("target-root");
    let script = work.0.join("rd-publication-note.py");
    write(&script, &read(&source));
    for (path, bytes) in baseline() {
        write(&target.join("tests/ui").join(path), bytes);
    }
    let sample = baseline()
        .get(Path::new("green/basic/golden/g/alpha/doc/a-10.html"))
        .unwrap();
    for path in [
        "tests/ui/green/rd-probe/tree/golden/page.html",
        "tests/ui/green/rd-probe/worktree/page.expect",
        "tests/ui/green/rd-probe/notes.txt",
        "outside-ui.html",
        "tools/page.golden",
        "rust/ckc/tests/page.expect",
    ] {
        write(&target.join(path), sample);
    }
    write(
        &target.join("tests/ui/green/rd-probe/not-page.expect"),
        format!("not an HTML page\n{ANCHOR}\n").as_bytes(),
    );
    write(
        &work.0.join("tests/ui/caller/golden/g/alpha/doc/page.html"),
        sample,
    );
    for dir in [&target, &work.0] {
        git(dir, &["init", "-q", "-b", "main"]);
        git(dir, &["add", "-A"]);
        git(dir, &["commit", "-q", "-m", "RD migration inputs"]);
    }
    let before = files(&work.0);
    let expected = before
        .iter()
        .map(|(path, bytes)| {
            let data = if path.starts_with("target-root/tests/ui") && migration_carrier(path, bytes)
            {
                add_note(bytes)
            } else {
                bytes.clone()
            };
            (path.clone(), data)
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
            .output()
            .unwrap()
    };
    for args in [vec![], vec![work.0.join("missing-root")]] {
        let refs = args.iter().map(PathBuf::as_path).collect::<Vec<_>>();
        let out = run(&refs);
        assert!(
            !out.status.success(),
            "migration requires an explicit existing root"
        );
        same_files(
            &files(&work.0),
            &before,
            "invalid invocation writes nothing",
        );
    }
    for pass in 1..=2 {
        let result = run(&[&target]);
        assert!(
            result.status.success(),
            "migration pass {pass}: {}",
            text(&result.stderr)
        );
        same_files(
            &files(&work.0),
            &expected,
            &format!("migration pass {pass}"),
        );
    }
    // Generated carrier contents and mixed migrated/unmigrated inputs grade f(f(x)) = f(x).
    for (i, payload) in ["", "plain", "α漢字", "<&>\"'", "\n\n", "</fieldset>"]
        .iter()
        .enumerate()
    {
        for http in [false, true] {
            let page = format!(
                "<!doctype html>\n<html><body><p>{payload}</p>\n<form method=\"post\">\n{ANCHOR}\n</form>\n</body></html>\n"
            );
            let data = if http {
                format!("HTTP 200\nContent-Type: text/html\n\n{page}")
            } else {
                page
            };
            let path = if http {
                let suffix = if i % 2 == 0 { "expect" } else { "golden" };
                target.join(format!(
                    "tests/ui/green/rd-probe/generated-{i}.stdout.{suffix}"
                ))
            } else {
                target.join(format!(
                    "tests/ui/green/rd-probe/golden/g/probe/doc/generated-{i}.html"
                ))
            };
            write(&path, data.as_bytes());
        }
    }
    // A guideline identifier matching an input-directory name is still a golden carrier.
    for gid in ["tree", "worktree"] {
        write(
            &target.join(format!("tests/ui/green/rd-probe/golden/g/{gid}/doc/a.html")),
            sample,
        );
    }
    git(&target, &["add", "-A"]);
    git(&target, &["commit", "-q", "-m", "RD generated carriers"]);
    let mixed = files(&work.0);
    let generated = mixed
        .iter()
        .map(|(path, bytes)| {
            let data = if path.starts_with("target-root/tests/ui") && migration_carrier(path, bytes)
            {
                add_note(bytes)
            } else {
                bytes.clone()
            };
            (path.clone(), data)
        })
        .collect::<Files>();
    assert_eq!(
        mixed
            .iter()
            .filter(|(p, b)| p.starts_with("target-root/tests/ui") && migration_carrier(p, b))
            .count(),
        14
    );
    for pass in 1..=2 {
        let result = run(&[&target]);
        assert!(
            result.status.success(),
            "generated migration pass {pass}: {}",
            text(&result.stderr)
        );
        same_files(
            &files(&work.0),
            &generated,
            &format!("generated pass {pass}"),
        );
    }
}

fn copy_rows(source: &str) -> Vec<(&str, Option<&str>)> {
    let body = source
        .split_once("copy_table! {")
        .expect("copy table marker")
        .1;
    let body = body.split_once("\n}\n").expect("copy table close").0;
    let mut rest = body;
    let mut rows = Vec::new();
    while !rest.trim().is_empty() {
        rest = rest.trim_start();
        if rest.starts_with("//") {
            rest = rest.split_once('\n').map_or("", |(_, tail)| tail);
            continue;
        }
        let len = rest
            .bytes()
            .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'_')
            .count();
        assert!(len > 0, "copy table row grammar");
        let name = &rest[..len];
        rest = rest[len..].trim_start();
        let literal = if let Some(after) = rest.strip_prefix('=') {
            rest = after.trim_start();
            assert!(rest.starts_with('"'), "copy table literal");
            let mut end = 1;
            while rest.as_bytes().get(end) != Some(&b'"') {
                assert!(end < rest.len(), "copy table string close");
                end += if rest.as_bytes()[end] == b'\\' { 2 } else { 1 };
            }
            let literal = &rest[..end + 1];
            rest = rest[end + 1..].trim_start();
            Some(literal)
        } else {
            None
        };
        rest = rest.strip_prefix(';').expect("copy table row terminator");
        rows.push((name, literal));
    }
    rows
}

#[test]
fn t5_copy_registry_appends_exact_literal_as_row_286() {
    let work = Scratch::new();
    let literal = work.0.join("copy.html");
    for (copy, rc) in [(NOTE, 0), ("Get started now!", 1)] {
        write(&literal, format!("X = \"{copy}\"\n").as_bytes());
        let result = Command::new(program())
            .args(["ui", "copy-check"])
            .arg(&literal)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(rc), "copy-check {copy:?}");
        if rc == 1 {
            assert!(
                text(&result.stdout).contains("exclamatory"),
                "copy-check control diagnostic"
            );
        }
    }
    assert_eq!(
        copy_rows("copy_table! {\n    bare;\n    // fake;\n    words = \"semi; quote\\\"\";\n}\n"),
        [("bare", None), ("words", Some("\"semi; quote\\\"\""))]
    );
    // Source-level registry contract only; renderer/proof bodies are not an oracle.
    let source = fs::read_to_string(root().join("rust/ckc-spec/src/ui.rs")).unwrap();
    let rows = copy_rows(&source);
    // SUP4 (contract sup4 D1) appends rows 287–293 after RD's row 286; REV-K (contract rev-k
    // P2) appends rows 294–297.
    assert_eq!(
        rows.len(),
        298,
        "copy_table row count: RD appends at index 286, SUP4 at 287–293, REV-K at 294–297"
    );
    assert_eq!(
        rows.iter()
            .map(|(name, _)| name)
            .collect::<BTreeSet<_>>()
            .len(),
        298,
        "unique registry names"
    );
    assert_eq!(
        rows[286],
        (NAME, Some(format!("\"{NOTE}\"").as_str())),
        "RD last registry row exact bytes"
    );
    assert_eq!(
        rows.iter()
            .filter(|(_, literal)| *literal == Some(format!("\"{NOTE}\"").as_str()))
            .count(),
        1,
        "D1 registered once"
    );
}

fn print_hides_form(page: &str) -> bool {
    let Some((_, tail)) = page.split_once("@media print {") else {
        return false;
    };
    let mut depth = 1;
    let mut end = None;
    for (i, ch) in tail.char_indices() {
        if ch == '{' {
            depth += 1;
        }
        if ch == '}' {
            depth -= 1;
        }
        if depth == 0 {
            end = Some(i);
            break;
        }
    }
    let Some(end) = end else {
        return false;
    };
    tail[..end].split('}').any(|rule| {
        rule.split_once('{')
            .is_some_and(|(selectors, declarations)| {
                selectors
                    .split(',')
                    .any(|selector| selector.trim() == "form")
                    && declarations.split(';').any(|declaration| {
                        declaration.split_once(':').is_some_and(|(key, value)| {
                            key.trim() == "display" && value.trim() == "none"
                        })
                    })
            })
    })
}

#[test]
fn t6_disclosure_lives_inside_the_print_hidden_form() {
    let mut documents = Files::new();
    for (path, bytes) in rendered() {
        if document_path(path) {
            assert!(
                print_hides_form(text(bytes)),
                "{}: print block hides form",
                path.display()
            );
            documents.insert(path.clone(), bytes.clone());
        }
    }
    assert_eq!(documents.len(), input_documents(), "print document census");
    eprintln!("RD print control: 48 forms hidden by the print block; checking D1 containment");
    check_notes(&documents);
}
