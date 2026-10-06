// N2 contract P1, P3–P5; CKC_UI_TEST_BIN selects the executable for red replay.
// Each case owns its copied corpus, git history, loopback port, and temp paths.
#![cfg(target_os = "linux")]

use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

const PAGE: &str = "/g/alpha/doc/a-10.html";
const LEDGER: &str = "guidelines/alpha/audit/adjudication.tsv";
const REFUSAL: &str = "commit does not hold the reviewed bundle";
const TOKEN: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const NOW: &str = "2026-01-01T00:00:00Z";
const GIT_ENV: [(&str, &str); 9] = [
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_CONFIG_SYSTEM", "/dev/null"),
    ("GIT_AUTHOR_NAME", "fixture"),
    ("GIT_AUTHOR_EMAIL", "fixture@localhost"),
    ("GIT_COMMITTER_NAME", "fixture"),
    ("GIT_COMMITTER_EMAIL", "fixture@localhost"),
    ("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+00:00"),
    ("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+00:00"),
    ("GIT_DEFAULT_HASH", "sha1"),
];

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

fn run(argv: &[&str], cwd: &Path, tmp: &Path) -> Vec<u8> {
    let out = Command::new(argv[0])
        .args(&argv[1..])
        .current_dir(cwd)
        .envs(GIT_ENV)
        .env("TMPDIR", tmp)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{argv:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let target = to.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_tree(&path, &target);
        } else {
            fs::copy(&path, &target).unwrap();
        }
    }
}

fn inventory(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                out.push((path.clone(), fs::read(&path).unwrap()));
            }
        }
    }
    out.sort();
    out
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Case {
    work: Scratch,
    tree: PathBuf,
    tmp: PathBuf,
}
impl Case {
    fn new(label: &str) -> Self {
        let work = Scratch(root().join(format!(
            "rust/target/ui-commit/{}/{label}",
            std::process::id()
        )));
        let _ = fs::remove_dir_all(&work.0);
        let tree = work.0.join("tree");
        let tmp = work.0.join("tmp");
        fs::create_dir_all(&tmp).unwrap();
        copy_tree(&root().join("tests/ui/green/basic/tree"), &tree);
        Self { work, tree, tmp }
    }

    fn git(&self, args: &[&str]) -> String {
        let mut argv = vec!["git"];
        argv.extend_from_slice(args);
        String::from_utf8(run(&argv, &self.tree, &self.tmp))
            .unwrap()
            .trim()
            .to_owned()
    }

    fn init(&self) -> String {
        self.git(&["init", "-q", "-b", "main"]);
        self.commit("commit fixture A")
    }

    fn commit(&self, message: &str) -> String {
        self.git(&["add", "-A"]);
        self.git(&["commit", "-q", "-m", message]);
        self.git(&["rev-parse", "HEAD"])
    }

    fn guidelines(&self) -> Vec<(PathBuf, Vec<u8>)> {
        inventory(&self.tree.join("guidelines"))
    }

    fn cli(&self, method: &str, body: &str, commit: Option<&str>) -> Response {
        let bin = program();
        let mut argv = vec![
            bin.to_str().unwrap(),
            "ui",
            "request",
            method,
            PAGE,
            self.tree.to_str().unwrap(),
            "--token",
            TOKEN,
            "--now",
            NOW,
        ];
        if method == "POST" {
            argv.extend([
                "--header",
                "Content-Type:application/x-www-form-urlencoded",
                "--body",
                body,
            ]);
        }
        if let Some(commit) = commit {
            argv.extend(["--commit", commit]);
        }
        let raw = run(&argv, &self.work.0, &self.tmp);
        Response::parse(&raw, b"\n\n")
    }

    fn page(&self, commit: Option<&str>) -> Response {
        let page = self.cli("GET", "", commit);
        assert_eq!(page.status, 200, "GET: {:?}", page.detail());
        page
    }
}

struct Response {
    status: u16,
    body: Vec<u8>,
}
impl Response {
    fn parse(raw: &[u8], separator: &[u8]) -> Self {
        let split = raw
            .windows(separator.len())
            .position(|w| w == separator)
            .expect("header end");
        let head = String::from_utf8_lossy(&raw[..split]);
        let status = head
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        Self {
            status,
            body: raw[split + separator.len()..].to_vec(),
        }
    }

    fn detail(&self) -> Option<String> {
        let body = String::from_utf8_lossy(&self.body);
        let (_, tail) = body.split_once("<!-- ui: verdict: ")?;
        Some(tail.split_once(" -->")?.0.to_owned())
    }
}

fn http(port: u16, method: &str, body: &[u8]) -> std::io::Result<Response> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))?;
    stream.set_read_timeout(Some(Duration::from_secs(20)))?;
    let mut request =
        format!("{method} {PAGE} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n");
    if method == "POST" {
        request += &format!(
            "Content-Length: {}\r\nContent-Type: application/x-www-form-urlencoded\r\nOrigin: http://127.0.0.1:{port}\r\n",
            body.len()
        );
    }
    request += "\r\n";
    stream.write_all(request.as_bytes())?;
    stream.write_all(body)?;
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw)?;
    Ok(Response::parse(&raw, b"\r\n\r\n"))
}

struct Server {
    child: Child,
    port: u16,
}
impl Server {
    fn start(case: &Case) -> Self {
        let port = TcpListener::bind(("127.0.0.1", 0))
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let server = Self {
            child: Command::new(program())
                .args([
                    "ui",
                    "serve",
                    &port.to_string(),
                    case.tree.to_str().unwrap(),
                ])
                .current_dir(&case.work.0)
                .envs(GIT_ENV)
                .env("TMPDIR", &case.tmp)
                .stdout(Stdio::null())
                .stderr(File::create(case.work.0.join("serve.stderr")).unwrap())
                .spawn()
                .unwrap(),
            port,
        };
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            match http(port, "GET", b"") {
                Ok(page) => {
                    assert_eq!(page.status, 200, "server GET: {:?}", page.detail());
                    return server;
                }
                Err(_) => {
                    assert!(Instant::now() < deadline, "server startup: 20 s");
                    std::thread::sleep(Duration::from_millis(25));
                }
            }
        }
    }

    fn page(&self) -> Response {
        let page = http(self.port, "GET", b"").unwrap();
        assert_eq!(page.status, 200, "GET: {:?}", page.detail());
        page
    }

    fn post(&self, body: &str) -> Response {
        http(self.port, "POST", body.as_bytes()).unwrap()
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn optional_field(page: &[u8], name: &str) -> Option<String> {
    let text = String::from_utf8_lossy(page);
    let key = format!("name=\"{name}\" value=\"");
    let start = text.find(&key)? + key.len();
    Some(text[start..start + text[start..].find('"').unwrap()].to_owned())
}

fn field(page: &[u8], name: &str) -> String {
    optional_field(page, name).unwrap_or_else(|| panic!("hidden field {name}"))
}

fn hidden(page: &[u8]) -> Vec<(&'static str, String)> {
    ["review_sha256", "ledger_sha256", "csrf"]
        .into_iter()
        .map(|name| (name, field(page, name)))
        .collect()
}

fn form(hidden: &[(&str, String)]) -> String {
    let enc = |s: &str| -> String {
        s.bytes()
            .map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                    (b as char).to_string()
                }
                b' ' => "+".into(),
                _ => format!("%{b:02X}"),
            })
            .collect()
    };
    let mut pairs = hidden.to_vec();
    pairs.extend([
        ("verdict", "approved".into()),
        ("reviewer", "commit reviewer".into()),
        ("comment", "commit decision".into()),
    ]);
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", enc(k), enc(v)))
        .collect::<Vec<_>>()
        .join("&")
}

fn with_commit(page: &[u8], commit: &str) -> String {
    let mut fields = hidden(page);
    fields.push(("commit", commit.into()));
    form(&fields)
}

fn assert_page_commit(page: &[u8], commit: &str) {
    let body = String::from_utf8_lossy(page);
    let expected = format!("<input type=\"hidden\" name=\"commit\" value=\"{commit}\">");
    assert_eq!(
        body.matches("name=\"commit\"").count(),
        1,
        "one hidden commit field; expected value {commit:?}"
    );
    assert!(body.contains(&expected), "hidden input: {expected}");
    let ledger = body.find("name=\"ledger_sha256\"").unwrap();
    let next = ledger + body[ledger..].find('>').unwrap() + 1;
    assert!(
        body[next..].trim_start().starts_with(&expected),
        "commit follows ledger_sha256"
    );
}

fn leftovers(case: &Case) -> Vec<PathBuf> {
    case.guidelines()
        .into_iter()
        .map(|(path, _)| path)
        .filter(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with(".adjudication.tsv.")
        })
        .collect()
}

fn refusal_error(
    case: &Case,
    response: &Response,
    before: &[(PathBuf, Vec<u8>)],
    status: u16,
    detail: &str,
) -> Option<String> {
    let observed = (response.status, response.detail());
    let expected = (status, Some(detail.to_owned()));
    let unchanged = case.guidelines() == before;
    let candidates = leftovers(case);
    if observed == expected && unchanged && candidates.is_empty() {
        None
    } else {
        Some(format!(
            "response {observed:?}, expected {expected:?}; guidelines unchanged={unchanged}; candidates={candidates:?}"
        ))
    }
}

fn assert_refusal(
    case: &Case,
    response: &Response,
    before: &[(PathBuf, Vec<u8>)],
    status: u16,
    detail: &str,
) {
    if let Some(error) = refusal_error(case, response, before, status, detail) {
        panic!("{error}");
    }
}

fn assert_record(
    case: &Case,
    response: &Response,
    before: &[(PathBuf, Vec<u8>)],
    digest: &str,
    commit: &str,
) {
    assert_eq!(response.status, 303, "POST: {:?}", response.detail());
    let ledger = case.tree.join(LEDGER);
    let rows = |bytes: &[u8]| -> Vec<String> {
        String::from_utf8_lossy(bytes)
            .lines()
            .filter(|row| !row.is_empty() && !row.starts_with('#'))
            .map(str::to_owned)
            .collect()
    };
    let old = before
        .iter()
        .find(|(path, _)| *path == ledger)
        .map_or_else(Vec::new, |(_, bytes)| rows(bytes));
    let after = rows(&fs::read(&ledger).unwrap());
    assert_eq!(after.len(), old.len() + 1, "one appended ledger row");
    assert_eq!(&after[..old.len()], old, "earlier ledger rows kept");
    let row: Vec<&str> = after.last().unwrap().split('\t').collect();
    assert_eq!(row.len(), 7, "ledger field count");
    assert_eq!(row[0], "a-10");
    assert_eq!(row[1], digest, "ledger bundle = page bundle");
    assert_eq!(row[2], commit, "ledger ace_commit = rendered commit");
    assert_eq!(row[3], "approved");
    assert_eq!(row[4], "commit reviewer");
    assert_eq!(row[6], "commit decision");
    let other_files = |entries: &[(PathBuf, Vec<u8>)]| -> Vec<(PathBuf, Vec<u8>)> {
        entries
            .iter()
            .filter(|(path, _)| *path != ledger)
            .cloned()
            .collect()
    };
    assert_eq!(
        other_files(&case.guidelines()),
        other_files(before),
        "only the ledger changes"
    );
    assert!(leftovers(case).is_empty(), "candidate cleanup");
}

fn commits() -> Vec<String> {
    let hex = b"0123456789abcdef";
    let mut out: Vec<String> = hex
        .iter()
        .map(|&b| (b as char).to_string().repeat(40))
        .collect();
    let mut state = 0x3df4_c269_792b_160d_u64;
    for _ in 0..24 {
        let value = (0..40)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                hex[(state & 15) as usize] as char
            })
            .collect();
        out.push(value);
    }
    out
}

fn malformed_commits() -> Vec<String> {
    let mut out = vec!["ABC".into(), "A".repeat(40)];
    for len in [1, 2, 3, 38, 39, 41, 42, 64, 65, 127] {
        out.push("a".repeat(len));
    }
    let base = b"0123456789abcdef0123456789abcdef01234567";
    for position in 0..40 {
        for bad in *b"Ag-" {
            let mut bytes = base.to_vec();
            bytes[position] = bad;
            out.push(String::from_utf8(bytes).unwrap());
        }
    }
    for bad in *b" +%&=/" {
        let mut bytes = base.to_vec();
        bytes[20] = bad;
        out.push(String::from_utf8(bytes).unwrap());
    }
    out
}

fn assert_cases(errors: &[String], count: usize) {
    assert!(
        errors.is_empty(),
        "{}/{} cases failed; first failures: {:?}",
        errors.len(),
        count,
        &errors[..errors.len().min(5)]
    );
}

#[test]
fn p1a_git_page_carries_rendered_commit() {
    let case = Case::new("p1a");
    let commit = case.init();
    let server = Server::start(&case);
    assert_page_commit(&server.page().body, &commit);
}

#[test]
fn p1b_filesystem_page_carries_empty_commit() {
    let case = Case::new("p1b");
    assert_page_commit(&case.page(None).body, "");
}

#[test]
fn p1c_override_page_carries_commit() {
    let case = Case::new("p1c");
    let inputs = commits();
    let mut errors = Vec::new();
    for commit in &inputs {
        let page = case.page(Some(commit));
        let actual = optional_field(&page.body, "commit");
        if actual.as_deref() == Some(commit) {
            assert_page_commit(&page.body, commit);
        } else {
            errors.push(format!("hidden commit {actual:?}, expected {commit:?}"));
        }
    }
    assert_cases(&errors, inputs.len());
}

#[test]
fn p3_post_records_page_commit_after_unrelated_head_move() {
    let case = Case::new("p3");
    let a = case.init();
    let server = Server::start(&case);
    let page = server.page();
    let mut fields = hidden(&page.body);
    // Submit the actual page fields so the base reaches its incorrect ledger row.
    if let Some(commit) = optional_field(&page.body, "commit") {
        fields.push(("commit", commit));
    }
    fs::write(case.tree.join("NOTES.txt"), "unrelated commit B\n").unwrap();
    let b = case.commit("unrelated commit B");
    assert_ne!(a, b);
    let before = case.guidelines();
    let response = server.post(&form(&fields));
    assert_record(
        &case,
        &response,
        &before,
        &field(&page.body, "review_sha256"),
        &a,
    );
    let current = server.page();
    assert_page_commit(&current.body, &b);
    let before = case.guidelines();
    let response = server.post(&with_commit(&current.body, &b));
    assert_record(
        &case,
        &response,
        &before,
        &field(&current.body, "review_sha256"),
        &b,
    );
}

#[test]
fn p4a_absent_commit_refuses_without_write() {
    let case = Case::new("p4a");
    case.init();
    let server = Server::start(&case);
    let page = server.page();
    let absent = "0".repeat(40);
    let before = case.guidelines();
    let response = server.post(&with_commit(&page.body, &absent));
    assert_refusal(&case, &response, &before, 409, REFUSAL);
}

#[test]
fn p4b_other_bundle_at_ancestor_refuses_without_write() {
    let case = Case::new("p4b");
    let a = case.init();
    let server = Server::start(&case);
    let old_page = server.page();
    fs::write(
        case.tree.join("guidelines/alpha/ace/a-10.ace"),
        "A reviewer must inspect a-10.\n",
    )
    .unwrap();
    let manifest = run(
        &[
            program().to_str().unwrap(),
            "derive-review-manifest",
            case.tree.join("guidelines/alpha").to_str().unwrap(),
        ],
        &case.tree,
        &case.tmp,
    );
    fs::write(
        case.tree.join("guidelines/alpha/audit/review-manifest.tsv"),
        manifest,
    )
    .unwrap();
    let b = case.commit("changed reviewed document B");
    assert_ne!(a, b);
    let page = server.page();
    assert_ne!(
        field(&old_page.body, "review_sha256"),
        field(&page.body, "review_sha256"),
        "ancestor and HEAD hold different bundles"
    );
    let before = case.guidelines();
    let response = server.post(&with_commit(&page.body, &a));
    assert_refusal(&case, &response, &before, 409, REFUSAL);
}

#[test]
fn p4c_git_malformed_commits_refuse_without_write() {
    let case = Case::new("p4c");
    case.init();
    let server = Server::start(&case);
    let page = server.page();
    let before = case.guidelines();
    let inputs = malformed_commits();
    let mut errors = Vec::new();
    for commit in &inputs {
        let response = server.post(&with_commit(&page.body, commit));
        if let Some(error) = refusal_error(&case, &response, &before, 400, "invalid commit") {
            errors.push(format!("{commit:?}: {error}"));
        }
    }
    assert_cases(&errors, inputs.len());
}

#[test]
fn p4d_git_empty_commit_refuses_without_write() {
    let case = Case::new("p4d");
    case.init();
    let server = Server::start(&case);
    let page = server.page();
    let before = case.guidelines();
    let response = server.post(&with_commit(&page.body, ""));
    assert_refusal(&case, &response, &before, 409, REFUSAL);
}

#[test]
fn p4e_nonancestor_same_bundle_refuses_without_write() {
    let case = Case::new("p4e");
    let a = case.init();
    let server = Server::start(&case);
    let page = server.page();
    case.git(&["commit", "--amend", "-q", "-m", "same tree rewritten B"]);
    let b = case.git(&["rev-parse", "HEAD"]);
    assert_ne!(a, b);
    assert_eq!(
        field(&page.body, "review_sha256"),
        field(&server.page().body, "review_sha256"),
        "amend keeps the reviewed bundle"
    );
    let before = case.guidelines();
    let response = server.post(&with_commit(&page.body, &a));
    assert_refusal(&case, &response, &before, 409, REFUSAL);
}

#[test]
fn p5a_missing_commit_reports_field_error() {
    let case = Case::new("p5a");
    let page = case.page(None);
    let before = case.guidelines();
    let response = case.cli("POST", &form(&hidden(&page.body)), None);
    assert_refusal(&case, &response, &before, 400, "missing field commit");
}

#[test]
fn p5b_duplicate_commit_reports_field_error() {
    let case = Case::new("p5b");
    let page = case.page(None);
    let before = case.guidelines();
    let hex = "a".repeat(40);
    let inputs = [("", ""), ("", hex.as_str()), (hex.as_str(), "")];
    let mut errors = Vec::new();
    for (first, second) in inputs {
        let mut fields = hidden(&page.body);
        fields.extend([("commit", first.into()), ("commit", second.into())]);
        let response = case.cli("POST", &form(&fields), None);
        if let Some(error) = refusal_error(&case, &response, &before, 400, "duplicate field commit")
        {
            errors.push(format!("{first:?}, {second:?}: {error}"));
        }
    }
    assert_cases(&errors, inputs.len());
}

#[test]
fn p5c_filesystem_malformed_commits_report_invalid_commit() {
    let case = Case::new("p5c");
    let page = case.page(None);
    let before = case.guidelines();
    let inputs = malformed_commits();
    let mut errors = Vec::new();
    for commit in &inputs {
        let response = case.cli("POST", &with_commit(&page.body, commit), None);
        if let Some(error) = refusal_error(&case, &response, &before, 400, "invalid commit") {
            errors.push(format!("{commit:?}: {error}"));
        }
    }
    assert_cases(&errors, inputs.len());
}

#[test]
fn p5d_filesystem_empty_commit_is_recorded() {
    let case = Case::new("p5d");
    let page = case.page(None);
    let before = case.guidelines();
    let response = case.cli("POST", &with_commit(&page.body, ""), None);
    assert_record(
        &case,
        &response,
        &before,
        &field(&page.body, "review_sha256"),
        "",
    );
}

#[test]
fn p5e_filesystem_nonempty_commit_refuses_without_write() {
    let case = Case::new("p5e");
    let page = case.page(None);
    let before = case.guidelines();
    let inputs = commits();
    let mut errors = Vec::new();
    for commit in &inputs {
        let response = case.cli("POST", &with_commit(&page.body, commit), None);
        if let Some(error) = refusal_error(&case, &response, &before, 409, REFUSAL) {
            errors.push(format!("{commit}: {error}"));
        }
    }
    assert_cases(&errors, inputs.len());
}

#[test]
fn p5f_matching_override_commit_is_recorded() {
    let inputs = commits();
    let mut errors = Vec::new();
    for (index, commit) in inputs.iter().enumerate() {
        let case = Case::new(&format!("p5f-{index}"));
        let page = case.page(Some(commit));
        let before = case.guidelines();
        let response = case.cli("POST", &with_commit(&page.body, commit), Some(commit));
        if response.status == 303 {
            assert_record(
                &case,
                &response,
                &before,
                &field(&page.body, "review_sha256"),
                commit,
            );
        } else {
            errors.push(format!(
                "{commit}: POST {}, {:?}; expected 303",
                response.status,
                response.detail()
            ));
        }
    }
    assert_cases(&errors, inputs.len());
}

#[test]
fn p5g_other_override_commit_refuses_without_write() {
    let case = Case::new("p5g");
    let inputs = commits();
    let mut errors = Vec::new();
    for (index, commit) in inputs.iter().enumerate() {
        let other = &inputs[(index + 1) % inputs.len()];
        assert_ne!(commit, other);
        let page = case.page(Some(commit));
        let before = case.guidelines();
        let response = case.cli("POST", &with_commit(&page.body, other), Some(commit));
        if let Some(error) = refusal_error(&case, &response, &before, 409, REFUSAL) {
            errors.push(format!("{commit} -> {other}: {error}"));
        }
    }
    assert_cases(&errors, inputs.len());
}
