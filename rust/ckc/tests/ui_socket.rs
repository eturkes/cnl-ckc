// `ckc ui serve` over a real loopback socket (.agent/archive/contracts/harness.md H7;
// native assertions of the M5.5 socket smoke). Fixture = tests/ui/green/basic/tree
// committed into a private git repository; oracles = `ckc ui request` on the
// same corpus. Linux only: listener scope = /proc/net/tcp.
// CKC_UI_TEST_BIN selects a prebuilt executable (red replay).
#![cfg(target_os = "linux")]

use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

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
    while let Some(d) = stack.pop() {
        for entry in fs::read_dir(&d).unwrap() {
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

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

// default-src covers fetches only: framing (clickjacking of the decision form),
// form targets + <base> need their own directives (SEC F1).
fn hardened(response: &Response, what: &str) {
    let csp = response.header("Content-Security-Policy");
    for directive in [
        "frame-ancestors 'none'",
        "form-action 'self'",
        "base-uri 'none'",
    ] {
        assert!(
            csp.split("; ").any(|d| d == directive),
            "{what}: CSP lacks {directive}: {csp}"
        );
    }
}

struct Response {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}
impl Response {
    fn header(&self, name: &str) -> &str {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
            .unwrap_or_else(|| panic!("missing header {name}"))
    }
}

// HTTP/1.1 with `Connection: close`; the response ends at EOF.
fn http(
    port: u16,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: &[u8],
) -> std::io::Result<Response> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))?;
    stream.set_read_timeout(Some(Duration::from_secs(20)))?;
    let mut request = format!("{method} {path} HTTP/1.1\r\nConnection: close\r\n");
    if method == "POST" {
        request += &format!("Content-Length: {}\r\n", body.len());
    }
    if !headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("Host")) {
        request += &format!("Host: 127.0.0.1:{port}\r\n");
    }
    for (k, v) in headers {
        request += &format!("{k}: {v}\r\n");
    }
    request += "\r\n";
    stream.write_all(request.as_bytes())?;
    stream.write_all(body)?;
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw)?;
    let split = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .expect("header end");
    let head = String::from_utf8(raw[..split].to_vec()).unwrap();
    let mut lines = head.split("\r\n");
    let status = lines
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let headers = lines
        .map(|l| {
            let (k, v) = l.split_once(": ").unwrap();
            (k.to_owned(), v.to_owned())
        })
        .collect();
    Ok(Response {
        status,
        headers,
        body: raw[split + 4..].to_vec(),
    })
}

fn field(page: &[u8], name: &str) -> String {
    let text = String::from_utf8_lossy(page);
    let key = format!("name=\"{name}\" value=\"");
    let start = text
        .find(&key)
        .unwrap_or_else(|| panic!("hidden field {name}"))
        + key.len();
    text[start..start + text[start..].find('"').unwrap()].to_owned()
}

fn form(hidden: &[(&str, String)], reviewer: &str) -> String {
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
    let mut pairs: Vec<(String, String)> = hidden
        .iter()
        .map(|(k, v)| ((*k).to_owned(), v.clone()))
        .collect();
    pairs.extend([
        ("verdict".into(), "approved".into()),
        ("reviewer".into(), reviewer.into()),
        ("comment".into(), "socket decision".into()),
    ]);
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", enc(k), enc(v)))
        .collect::<Vec<_>>()
        .join("&")
}

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn private_dir(path: &Path) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    fs::create_dir_all(path).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    path.to_path_buf()
}

// `ckc ui serve` with no browser launcher in reach: it prints the sign-in address.
fn serve(port: u16, tree: &Path, cwd: &Path, tmp: &Path, run: &Path) -> Command {
    let mut command = Command::new(program());
    command
        .args(["ui", "serve", &port.to_string(), tree.to_str().unwrap()])
        .current_dir(cwd)
        .envs(GIT_ENV)
        .env("TMPDIR", tmp)
        .env("XDG_RUNTIME_DIR", run)
        .env_remove("BROWSER")
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WSL_DISTRO_NAME")
        .stdin(Stdio::null());
    command
}

fn printed_address(stdout: &Path, port: u16) -> String {
    let prefix = "ui: no browser opened. Open this address: ";
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let text = fs::read_to_string(stdout).unwrap_or_default();
        if let Some(line) = text.lines().find(|l| l.starts_with(prefix)) {
            let address = line[prefix.len()..].to_owned();
            let session = address
                .strip_prefix(&format!("http://127.0.0.1:{port}/session/"))
                .unwrap_or_else(|| panic!("sign-in address: {address}"));
            assert!(
                session.len() == 64
                    && session
                        .bytes()
                        .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
            );
            return address;
        }
        assert!(Instant::now() < deadline, "server startup: 20 s ({text})");
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn free_port() -> u16 {
    TcpListener::bind(("127.0.0.1", 0))
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

// tests/ui/green/basic/tree committed into a private repository under rust/target.
fn fixture(name: &str) -> (Scratch, PathBuf, PathBuf) {
    let work = Scratch(root().join(format!(
        "rust/target/ui-socket/{}-{name}",
        std::process::id()
    )));
    let _ = fs::remove_dir_all(&work.0);
    let (tree, tmp) = (work.0.join("tree"), work.0.join("tmp"));
    fs::create_dir_all(&tmp).unwrap();
    copy_tree(&root().join("tests/ui/green/basic/tree"), &tree);
    run(&["git", "init", "-q", "-b", "main"], &tree, &tmp);
    run(&["git", "add", "-A"], &tree, &tmp);
    run(
        &["git", "commit", "-q", "-m", "socket fixture"],
        &tree,
        &tmp,
    );
    (work, tree, tmp)
}

#[test]
fn serve_get_post_cas_and_cleanup_over_loopback() {
    let work = Scratch(root().join(format!("rust/target/ui-socket/{}", std::process::id())));
    let _ = fs::remove_dir_all(&work.0);
    let (tree, oracle, tmp) = (
        work.0.join("tree"),
        work.0.join("oracle"),
        work.0.join("tmp"),
    );
    fs::create_dir_all(&tmp).unwrap();
    copy_tree(&root().join("tests/ui/green/basic/tree"), &tree);
    run(&["git", "init", "-q", "-b", "main"], &tree, &tmp);
    run(&["git", "add", "-A"], &tree, &tmp);
    run(
        &["git", "commit", "-q", "-m", "socket fixture"],
        &tree,
        &tmp,
    );
    // A later commit that leaves every ACE file alone: the ledger row must record
    // this viewed HEAD, not the last commit that touched the document's ACE text.
    fs::write(tree.join("NOTES.txt"), "viewed-commit probe\n").unwrap();
    run(&["git", "add", "-A"], &tree, &tmp);
    run(&["git", "commit", "-q", "-m", "later commit"], &tree, &tmp);
    let revision = String::from_utf8(run(&["git", "rev-parse", "HEAD"], &tree, &tmp))
        .unwrap()
        .trim()
        .to_owned();
    run(
        &[
            "git",
            "clone",
            "-q",
            "--shared",
            tree.to_str().unwrap(),
            "oracle",
        ],
        &work.0,
        &tmp,
    );

    // A port the kernel just handed out; freed before the server binds it.
    let port = TcpListener::bind(("127.0.0.1", 0))
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let stderr_path = work.0.join("serve.stderr");
    let stdout_path = work.0.join("serve.stdout");
    let run_dir = private_dir(&work.0.join("run"));
    let server = Server(
        serve(port, &tree, &work.0, &tmp, &run_dir)
            .stdout(File::create(&stdout_path).unwrap())
            .stderr(File::create(&stderr_path).unwrap())
            .spawn()
            .unwrap(),
    );
    let signin = printed_address(&stdout_path, port);
    // Without the launch cookie every route answers the session page, before any snapshot.
    let closed = http(port, "GET", "/index.html", &[], b"").unwrap();
    assert_eq!(closed.status, 403);
    assert!(contains(
        &closed.body,
        b"<title>Open the review site from ckc</title>"
    ));
    hardened(&closed, "session 403");
    let session = signin.rsplit('/').next().unwrap().to_owned();
    let entered = http(port, "GET", &format!("/session/{session}"), &[], b"").unwrap();
    assert_eq!(
        (
            entered.status,
            entered.header("Location"),
            entered.header("Set-Cookie")
        ),
        (
            303,
            "/",
            format!("ckc-session-{port}={session}; HttpOnly; SameSite=Lax; Path=/").as_str()
        ),
        "sign-in: Lax keeps the cookie on the redirect that follows a file: page"
    );
    let cookie = format!("ckc-session-{port}={session}");
    let other = format!("ckc-session-{port}={}", "0".repeat(64));
    assert_eq!(
        http(port, "GET", "/", &[("Cookie", other.as_str())], b"")
            .unwrap()
            .status,
        403,
        "a cookie of another session"
    );
    let jar = [("Cookie", cookie.as_str())];
    let index = http(port, "GET", "/index.html", &jar, b"").unwrap();
    assert_eq!(index.status, 200);
    let listening: Vec<String> = fs::read_to_string("/proc/net/tcp")
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.split_whitespace().map(str::to_owned).collect::<Vec<_>>())
        .filter(|f| f[1].ends_with(&format!(":{port:04X}")) && f[3] == "0A")
        .map(|f| f[1].clone())
        .collect();
    assert_eq!(
        listening,
        [format!("0100007F:{port:04X}")],
        "one loopback-only listener"
    );

    let page_path = "/g/alpha/doc/a-10.html";
    let page = http(port, "GET", page_path, &jar, b"").unwrap();
    assert_eq!(page.status, 200);
    let csrf = field(&page.body, "csrf");
    assert!(
        csrf.len() == 64
            && csrf
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    );
    assert!(
        page.header("Content-Security-Policy")
            .starts_with("default-src 'none'; style-src 'unsafe-inline'; script-src 'sha256-")
    );
    hardened(&page, page_path);
    for path in [
        "/",
        "/g/alpha/",
        "/g/alpha/records.html",
        "/g/alpha/source/evidence.txt",
    ] {
        let response = http(port, "GET", path, &jar, b"").unwrap();
        assert_eq!(response.status, 200, "{path}");
        hardened(&response, path);
    }
    let cli = run(
        &[
            program().to_str().unwrap(),
            "ui",
            "request",
            "GET",
            page_path,
            tree.to_str().unwrap(),
            "--token",
            &csrf,
        ],
        &work.0,
        &tmp,
    );
    let (cli_head, cli_body) = cli.split_at(cli.windows(2).position(|w| w == b"\n\n").unwrap());
    assert_eq!(
        page.body,
        cli_body[2..],
        "socket GET body = `ckc ui request GET` body"
    );
    for key in [
        "Content-Type",
        "Content-Security-Policy",
        "X-Content-Type-Options",
        "Referrer-Policy",
        "Cache-Control",
    ] {
        let line = format!("{key}: {}", page.header(key));
        assert!(contains(cli_head, line.as_bytes()), "{key}");
    }

    let guidelines = tree.join("guidelines");
    let before = inventory(&guidelines);
    let foreign = http(
        port,
        "GET",
        page_path,
        &[("Host", "foreign.invalid"), ("Cookie", cookie.as_str())],
        b"",
    )
    .unwrap();
    assert_eq!(foreign.status, 403);
    assert!(contains(&foreign.body, b"ui: request: host not allowed"));
    hardened(&foreign, "foreign Host 403");
    assert!(
        inventory(&guidelines) == before,
        "a refused request writes nothing"
    );

    let origin = format!("http://127.0.0.1:{port}");
    let post_headers = [
        ("Content-Type", "application/x-www-form-urlencoded"),
        ("Origin", origin.as_str()),
        ("Cookie", cookie.as_str()),
    ];
    let hidden = |body: &[u8]| {
        vec![
            ("review_sha256", field(body, "review_sha256")),
            ("ledger_sha256", field(body, "ledger_sha256")),
            ("csrf", field(body, "csrf")),
            ("commit", field(body, "commit")),
        ]
    };
    let body = form(&hidden(&page.body), "socket reviewer");
    let accepted = http(port, "POST", page_path, &post_headers, body.as_bytes()).unwrap();
    assert_eq!(
        (accepted.status, accepted.header("Location")),
        (303, page_path)
    );
    hardened(&accepted, "decision 303");
    let ledger = guidelines.join("alpha/audit/adjudication.tsv");
    let after = fs::read(&ledger).unwrap();
    let rows = |b: &[u8]| -> Vec<Vec<u8>> {
        b.split(|&c| c == b'\n')
            .filter(|r| !r.is_empty() && !r.starts_with(b"#"))
            .map(<[u8]>::to_vec)
            .collect()
    };
    let old = before
        .iter()
        .find(|(p, _)| p.ends_with("alpha/audit/adjudication.tsv"))
        .map_or(Vec::new(), |(_, b)| rows(b));
    let new: Vec<Vec<u8>> = rows(&after)
        .into_iter()
        .filter(|r| !old.contains(r))
        .collect();
    assert_eq!(new.len(), 1, "one appended ledger row");
    let row: Vec<String> = String::from_utf8(new[0].clone())
        .unwrap()
        .split('\t')
        .map(str::to_owned)
        .collect();
    assert_eq!(row.len(), 7);
    assert_eq!(
        [&row[0], &row[1], &row[2], &row[3], &row[4], &row[6]],
        [
            "a-10",
            &hidden(&page.body)[0].1,
            &revision,
            "approved",
            "socket reviewer",
            "socket decision"
        ]
    );
    assert!(
        old.iter().all(|r| rows(&after).contains(r)),
        "earlier rows kept"
    );
    let replay = run(
        &[
            program().to_str().unwrap(),
            "ui",
            "request",
            "POST",
            page_path,
            oracle.to_str().unwrap(),
            "--body",
            &body,
            "--token",
            &csrf,
            "--now",
            &row[5],
        ],
        &work.0,
        &tmp,
    );
    assert!(replay.starts_with(b"HTTP 303\n"));
    assert_eq!(
        after,
        fs::read(oracle.join("guidelines/alpha/audit/adjudication.tsv")).unwrap(),
        "socket ledger = CLI ledger"
    );

    let before_stale = inventory(&guidelines);
    assert_eq!(
        http(port, "POST", page_path, &post_headers, body.as_bytes())
            .unwrap()
            .status,
        409
    );
    assert!(
        inventory(&guidelines) == before_stale,
        "a stale POST writes nothing"
    );

    // Forced CAS: hold the ledger lock, let a POST stage its candidate, commit a
    // concurrent ledger, release; the POST must refuse and keep those bytes.
    let fresh = hidden(&http(port, "GET", page_path, &jar, b"").unwrap().body);
    let audit = guidelines.join("alpha/audit");
    let lock_path = audit.join(".adjudication.lock");
    let lock = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .unwrap();
    lock.lock().unwrap();
    let racing = std::thread::spawn({
        let body = form(&fresh, "racing reviewer");
        let origin = origin.clone();
        let cookie = cookie.clone();
        move || {
            let headers = [
                ("Content-Type", "application/x-www-form-urlencoded"),
                ("Origin", origin.as_str()),
                ("Cookie", cookie.as_str()),
            ];
            http(port, "POST", page_path, &headers, body.as_bytes()).unwrap()
        }
    });
    let deadline = Instant::now() + Duration::from_secs(20);
    let candidate = loop {
        assert!(
            !racing.is_finished(),
            "the POST must wait for the held lock"
        );
        let staged = fs::read_dir(&audit)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(".adjudication.tsv.")
            })
            .filter_map(|p| fs::read(p).ok())
            .find(|d| d.ends_with(b"\n") && contains(d, b"\tracing reviewer\t"));
        if let Some(data) = staged {
            break data;
        }
        assert!(Instant::now() < deadline, "candidate creation: 20 s");
        std::thread::sleep(Duration::from_millis(20));
    };
    let winning = String::from_utf8(candidate)
        .unwrap()
        .replace("\tracing reviewer\t", "\trace winner\t");
    fs::write(audit.join(".winner"), &winning).unwrap();
    fs::rename(audit.join(".winner"), &ledger).unwrap();
    lock.unlock().unwrap();
    drop(lock);
    let refused = racing.join().unwrap();
    assert_eq!(refused.status, 409);
    assert!(contains(&refused.body, b"ui: verdict: ledger changed"));
    assert_eq!(
        fs::read(&ledger).unwrap(),
        winning.as_bytes(),
        "CAS refusal keeps the concurrent bytes"
    );
    let leftovers: Vec<String> = fs::read_dir(&audit)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(".adjudication"))
        .collect();
    assert!(
        leftovers.is_empty(),
        "candidate + lock cleanup: {leftovers:?}"
    );
    assert_eq!(fs::read(&stderr_path).unwrap(), b"", "server stderr");

    drop(server);
    assert!(
        TcpStream::connect(("127.0.0.1", port)).is_err(),
        "port released after stop"
    );
}

// Copy that merely spells a resource attribute (`src=`) is text, not a resource:
// `ckc ui check` stays green (M5.5 resource-copy probe: README text; README +
// ledger reviewer field).
#[test]
fn resource_attribute_text_in_copy_passes_ui_check() {
    for (label, fixture) in [("text", "basic"), ("attribute", "verdicts")] {
        let work = Scratch(root().join(format!(
            "rust/target/ui-resource/{}-{label}",
            std::process::id()
        )));
        let _ = fs::remove_dir_all(&work.0);
        let tree = work.0.join("tree");
        copy_tree(
            &root().join(format!("tests/ui/green/{fixture}/tree")),
            &tree,
        );
        fs::write(
            tree.join("guidelines/alpha/README.md"),
            "# Source attribute src=local\n",
        )
        .unwrap();
        if label == "attribute" {
            let path = tree.join("guidelines/alpha/audit/adjudication.tsv");
            let rows: Vec<String> = fs::read_to_string(&path)
                .unwrap()
                .lines()
                .map(|row| {
                    let mut f: Vec<&str> = row.split('\t').collect();
                    if f.len() == 7 && !row.starts_with('#') {
                        f[4] = "src=local";
                    }
                    f.join("\t")
                })
                .collect();
            fs::write(&path, rows.join("\n") + "\n").unwrap();
        }
        let out = Command::new(program())
            .args(["ui", "check", tree.to_str().unwrap()])
            .current_dir(root())
            .env("TMPDIR", &work.0)
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(0),
            "{label}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(out.stderr.is_empty(), "{label}: stderr");
    }
}

// The browser receives the session from a private file the server writes; the
// opener's argv carries that path alone, never the secret; Enter reopens.
#[test]
fn opener_receives_a_private_file_never_the_secret() {
    use std::os::unix::fs::PermissionsExt;
    let (work, tree, tmp) = fixture("opener");
    let args = work.0.join("opener.args");
    let opener = work.0.join("opener.sh");
    fs::write(
        &opener,
        format!("#!/bin/sh\nprintf '%s\\n' \"$@\" >> '{}'\n", args.display()),
    )
    .unwrap();
    fs::set_permissions(&opener, fs::Permissions::from_mode(0o755)).unwrap();
    for (label, runtime_mode) in [
        ("private runtime dir", 0o700),
        ("shared runtime dir", 0o755),
    ] {
        let _ = fs::remove_file(&args);
        let run_dir = work.0.join(format!("run-{runtime_mode:o}"));
        fs::create_dir_all(&run_dir).unwrap();
        fs::set_permissions(&run_dir, fs::Permissions::from_mode(runtime_mode)).unwrap();
        let port = free_port();
        let stdout_path = work.0.join(format!("opener-{runtime_mode:o}.stdout"));
        let mut server = Server(
            serve(port, &tree, &work.0, &tmp, &run_dir)
                .env("BROWSER", &opener)
                .stdin(Stdio::piped())
                .stdout(File::create(&stdout_path).unwrap())
                .stderr(Stdio::null())
                .spawn()
                .unwrap(),
        );
        let deadline = Instant::now() + Duration::from_secs(20);
        let opened = "ui: the review site is open in your browser. Press Enter to open it again. Press Ctrl+C to stop.";
        while fs::read_to_string(&stdout_path)
            .unwrap_or_default()
            .matches(opened)
            .count()
            < 1
        {
            assert!(Instant::now() < deadline, "{label}: opened line");
            std::thread::sleep(Duration::from_millis(50));
        }
        let lines = fs::read_to_string(&args).unwrap();
        let path = PathBuf::from(lines.trim_end());
        assert_eq!(lines.lines().count(), 1, "{label}: one argument: {lines}");
        let page = fs::read_to_string(&path).unwrap();
        let mode = |p: &Path| fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&path), 0o600, "{label}: file mode");
        assert_eq!(mode(path.parent().unwrap()), 0o700, "{label}: dir mode");
        if runtime_mode == 0o700 {
            assert_eq!(path, run_dir.join(format!("ckc-ui-{port}.html")), "{label}");
        } else {
            assert!(
                path.starts_with(&tmp),
                "{label}: temp-dir fallback {path:?}"
            );
        }
        let key = "content=\"0; url=";
        let start = page.find(key).unwrap() + key.len();
        let address = &page[start..start + page[start..].find('"').unwrap()];
        let session = address
            .strip_prefix(&format!("http://127.0.0.1:{port}/session/"))
            .unwrap();
        assert!(!lines.contains(session), "{label}: secret in argv");
        assert!(
            !fs::read_to_string(&stdout_path).unwrap().contains(session),
            "{label}: secret on stdout"
        );
        let entered = http(port, "GET", &format!("/session/{session}"), &[], b"").unwrap();
        assert_eq!(entered.status, 303, "{label}: sign-in");
        server.0.stdin.as_mut().unwrap().write_all(b"\n").unwrap();
        let deadline = Instant::now() + Duration::from_secs(20);
        while fs::read_to_string(&args).unwrap().lines().count() < 2 {
            assert!(Instant::now() < deadline, "{label}: Enter reopens");
            std::thread::sleep(Duration::from_millis(50));
        }
        drop(server);
    }
}

// Guards that read the request alone answer before the snapshot: a corpus
// whose committed tree cannot be extracted still yields their pages.
#[test]
fn gate_answers_before_the_snapshot() {
    let (work, tree, tmp) = fixture("gate");
    std::os::unix::fs::symlink("README.md", tree.join("guidelines/alpha/link.md")).unwrap();
    run(&["git", "add", "-A"], &tree, &tmp);
    run(&["git", "commit", "-q", "-m", "symlink"], &tree, &tmp);
    let request = |extra: &[&str]| {
        let mut argv = vec![
            program().to_str().unwrap().to_owned(),
            "ui".into(),
            "request".into(),
            "GET".into(),
            "/".into(),
            tree.to_str().unwrap().to_owned(),
        ];
        argv.extend(extra.iter().map(|s| (*s).to_owned()));
        let out = Command::new(&argv[0])
            .args(&argv[1..])
            .current_dir(&work.0)
            .envs(GIT_ENV)
            .env("TMPDIR", &tmp)
            .output()
            .unwrap();
        (out.status.code(), out.stdout)
    };
    let (rc, out) = request(&[]);
    assert_eq!(rc, Some(0));
    assert!(out.starts_with(b"HTTP 500\n"), "snapshot failure control");
    let (rc, out) = request(&["--header", "Host:foreign.invalid"]);
    assert_eq!(rc, Some(0));
    assert!(
        out.starts_with(b"HTTP 403\n"),
        "Host refusal before the snapshot"
    );
    assert!(contains(&out, b"ui: request: host not allowed"));
    let session = "a".repeat(64);
    let (rc, out) = request(&["--session", &session]);
    assert_eq!(rc, Some(0), "--session accepted");
    assert!(
        out.starts_with(b"HTTP 403\n"),
        "session refusal before the snapshot"
    );
    assert!(contains(&out, b"ui: request: no review session"));
    let cookie = format!("Cookie:ckc-session-8377={session}");
    let (rc, out) = request(&["--session", &session, "--header", &cookie]);
    assert_eq!(rc, Some(0));
    assert!(out.starts_with(b"HTTP 500\n"), "the cookie passes the gate");
}

// A body past the cap is never read: the decision is refused at once.
#[test]
fn oversized_body_is_refused_unread() {
    let (work, tree, tmp) = fixture("cap");
    let run_dir = private_dir(&work.0.join("run"));
    let port = free_port();
    let stdout_path = work.0.join("cap.stdout");
    let _server = Server(
        serve(port, &tree, &work.0, &tmp, &run_dir)
            .stdout(File::create(&stdout_path).unwrap())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let signin = printed_address(&stdout_path, port);
    let cookie = format!("ckc-session-{port}={}", signin.rsplit('/').next().unwrap());
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let head = format!(
        "POST /g/alpha/doc/a-10.html HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nOrigin: http://127.0.0.1:{port}\r\nContent-Type: application/x-www-form-urlencoded\r\nCookie: {cookie}\r\nContent-Length: 65537\r\n\r\n"
    );
    stream.write_all(head.as_bytes()).unwrap();
    let start = Instant::now();
    let mut raw = Vec::new();
    stream
        .read_to_end(&mut raw)
        .expect("answer without waiting for the body");
    assert!(start.elapsed() < Duration::from_secs(5));
    assert!(
        raw.starts_with(b"HTTP/1.0 400 "),
        "{}",
        String::from_utf8_lossy(&raw)
    );
    assert!(contains(&raw, b"ui: verdict: missing body"));
}

// One deadline per connection: a client stalled mid-headers is cut, and the
// next client is served within it.
#[test]
fn stalled_client_is_cut_at_the_deadline() {
    let (work, tree, tmp) = fixture("deadline");
    let run_dir = private_dir(&work.0.join("run"));
    let port = free_port();
    let stdout_path = work.0.join("deadline.stdout");
    let _server = Server(
        serve(port, &tree, &work.0, &tmp, &run_dir)
            .stdout(File::create(&stdout_path).unwrap())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    printed_address(&stdout_path, port);
    let mut stalled = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stalled.write_all(b"GET / HTTP/1.1\r\n").unwrap();
    std::thread::sleep(Duration::from_millis(500));
    let start = Instant::now();
    let next = http(port, "GET", "/", &[], b"").unwrap();
    let waited = start.elapsed();
    assert_eq!(next.status, 403);
    assert!(
        waited < Duration::from_secs(14),
        "next client waited {waited:?} behind a stalled one"
    );
    let mut rest = Vec::new();
    stalled
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let _ = stalled.read_to_end(&mut rest);
    assert!(rest.is_empty() || rest.starts_with(b"HTTP/1.0 400 "));
}

// A refused POST whose body the gate never read must still deliver its whole
// answer: closing with unread bytes would reset the connection instead.
#[test]
fn refused_body_still_gets_the_whole_answer() {
    let (work, tree, tmp) = fixture("linger");
    let run_dir = private_dir(&work.0.join("run"));
    let port = free_port();
    let stdout_path = work.0.join("linger.stdout");
    let _server = Server(
        serve(port, &tree, &work.0, &tmp, &run_dir)
            .stdout(File::create(&stdout_path).unwrap())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    printed_address(&stdout_path, port);
    let origin = format!("http://127.0.0.1:{port}");
    let stale = format!("ckc-session-{port}={}", "0".repeat(64));
    let body = vec![b'a'; 60 * 1024];
    for attempt in 0..20 {
        let response = http(
            port,
            "POST",
            "/g/alpha/doc/a-10.html",
            &[
                ("Content-Type", "application/x-www-form-urlencoded"),
                ("Origin", origin.as_str()),
                ("Cookie", stale.as_str()),
            ],
            &body,
        )
        .unwrap_or_else(|e| panic!("attempt {attempt}: {e}"));
        assert_eq!(response.status, 403, "attempt {attempt}");
        assert!(
            response.body.ends_with(b"</html>\n"),
            "attempt {attempt}: truncated"
        );
        assert_eq!(
            response.body.len(),
            response.header("Content-Length").parse::<usize>().unwrap(),
            "attempt {attempt}"
        );
    }
}

// A request the parser cannot read gets the shared response headers too (CSP
// `hardened`, nosniff, no-store), like every other answer.
#[test]
fn unreadable_request_answer_is_hardened() {
    let (work, tree, tmp) = fixture("bad-request");
    let run_dir = private_dir(&work.0.join("run"));
    let port = free_port();
    let stdout_path = work.0.join("bad-request.stdout");
    let _server = Server(
        serve(port, &tree, &work.0, &tmp, &run_dir)
            .stdout(File::create(&stdout_path).unwrap())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    printed_address(&stdout_path, port);
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream.write_all(b"GET / HTTP/9.9\r\n\r\n").unwrap();
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).unwrap();
    let split = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .expect("header end");
    let head = String::from_utf8(raw[..split].to_vec()).unwrap();
    let mut lines = head.split("\r\n");
    assert!(lines.next().unwrap().starts_with("HTTP/1.0 400 "), "{head}");
    let response = Response {
        status: 400,
        headers: lines
            .map(|l| {
                let (k, v) = l.split_once(": ").unwrap();
                (k.to_owned(), v.to_owned())
            })
            .collect(),
        body: raw[split + 4..].to_vec(),
    };
    hardened(&response, "unreadable request 400");
    assert_eq!(response.header("X-Content-Type-Options"), "nosniff");
    assert_eq!(response.header("Cache-Control"), "no-store");
    assert!(response.body.is_empty());
}
