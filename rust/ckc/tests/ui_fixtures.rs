// M5.5 P4/P6: unchanged legacy fixture bytes grade the shell envelope.
// FC2 argv map: python3 -P tools/ui.py ARGS -> ckc ui ARGS.
// Copy fixture map: goal.py copy_scan_source(FILE) -> ckc ui copy-check FILE;
// red = rc1 + expect.txt on stdout; green = rc0 + empty stdout; stderr empty.
// CKC_UI_TEST_BIN selects a prebuilt executable for diff-blind/red replay.
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TMP: AtomicU64 = AtomicU64::new(0);

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn read(path: &Path) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn text(path: &Path) -> String {
    String::from_utf8(read(path)).unwrap()
}

fn digest_hex(bytes: &[u8]) -> String {
    const HEX: &[u8] = b"0123456789abcdef";
    Sha256::digest(bytes)
        .iter()
        .flat_map(|b| [HEX[(b >> 4) as usize], HEX[(b & 15) as usize]])
        .map(char::from)
        .collect()
}

fn bytes_equal(actual: &[u8], expected: &[u8], label: &str) {
    if actual != expected {
        let offset = actual
            .iter()
            .zip(expected)
            .position(|(a, b)| a != b)
            .unwrap_or(actual.len().min(expected.len()));
        panic!(
            "{label}: expected {} bytes/{}, actual {} bytes/{}; offset {offset}; expected {:?}, actual {:?}",
            expected.len(),
            digest_hex(expected),
            actual.len(),
            digest_hex(actual),
            String::from_utf8_lossy(&expected[offset..expected.len().min(offset + 120)]),
            String::from_utf8_lossy(&actual[offset..actual.len().min(offset + 120)])
        );
    }
}

fn paths(base: &Path) -> Vec<PathBuf> {
    fn walk(base: &Path, here: &Path, output: &mut Vec<PathBuf>) {
        if !here.is_dir() {
            return;
        }
        let mut entries = fs::read_dir(here)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.file_name().unwrap() == ".git" {
                continue;
            }
            assert!(
                !path.is_symlink(),
                "fixture/output symlink: {}",
                path.display()
            );
            output.push(path.strip_prefix(base).unwrap().to_path_buf());
            if path.is_dir() {
                walk(base, &path, output);
            }
        }
    }
    let mut output = Vec::new();
    walk(base, base, &mut output);
    output.sort();
    output
}

fn files(base: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    paths(base)
        .into_iter()
        .filter(|p| base.join(p).is_file())
        .map(|p| {
            let data = read(&base.join(&p));
            (p, data)
        })
        .collect()
}

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let base = root().join("rust/target/ui-fixtures");
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
            eprintln!("fixture cleanup {}: {e}", self.0.display());
        }
    }
}

fn sidecar(case: &Path, name: &str) -> Vec<String> {
    let path = case.join(name);
    if !path.exists() {
        return Vec::new();
    }
    text(&path)
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

fn write(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

fn fixture_git(tree: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(tree)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env("GIT_DEFAULT_HASH", "sha1")
        .env("GIT_AUTHOR_NAME", "fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@localhost")
        .env("GIT_COMMITTER_NAME", "fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@localhost")
        .env("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+00:00")
        .env("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+00:00")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "fixture git {args:?}: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
}

struct Fixture {
    scratch: Scratch,
    case: PathBuf,
    tree: PathBuf,
    tree_arg: String,
}
impl Fixture {
    fn new(color: &str, name: &str) -> Self {
        let scratch = Scratch::new();
        let relative = format!("tests/ui/{color}/{name}/tree");
        let case = root().join(format!("tests/ui/{color}/{name}"));
        let material = ["tree-order.txt", "empty-dirs.txt", "worktree"]
            .iter()
            .any(|p| case.join(p).exists());
        let tree = scratch.0.join(if material {
            "materialized/tree"
        } else {
            &relative
        });
        fs::create_dir_all(&tree).unwrap();
        let input = files(&case.join("tree"));
        let order = if case.join("tree-order.txt").is_file() {
            sidecar(&case, "tree-order.txt")
                .into_iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>()
        } else {
            input.keys().cloned().collect()
        };
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(
            sorted,
            input.keys().cloned().collect::<Vec<_>>(),
            "{color}/{name} tree-order"
        );
        for rel in order {
            write(&tree.join(&rel), &input[&rel]);
        }
        for rel in sidecar(&case, "empty-dirs.txt") {
            fs::create_dir_all(tree.join(rel)).unwrap();
        }
        if case.join("worktree").is_dir() {
            fixture_git(&tree, &["init", "-q", "-b", "main"]);
            fixture_git(&tree, &["add", "-A"]);
            fixture_git(&tree, &["commit", "-q", "-m", "fixture corpus"]);
            for (rel, data) in files(&case.join("worktree")) {
                write(&tree.join(rel), &data);
            }
        }
        let tree_arg = if material {
            tree.to_str().unwrap().to_owned()
        } else {
            relative
        };
        Self {
            scratch,
            case,
            tree,
            tree_arg,
        }
    }
    fn invoke(&self, args: &[String]) -> Output {
        let program = std::env::var_os("CKC_UI_TEST_BIN")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
        Command::new(program)
            .arg("ui")
            .args(args)
            .current_dir(&self.scratch.0)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .env("PYTHONDONTWRITEBYTECODE", "1")
            .env("TMPDIR", &self.scratch.0)
            .output()
            .unwrap()
    }
}

fn stream(case: &Path, name: &str) -> Vec<u8> {
    if name == "-" {
        Vec::new()
    } else {
        read(&case.join(name))
    }
}

fn result_equal(output: &Output, rc: i32, stdout: &[u8], stderr: &[u8], label: &str) {
    assert_eq!(
        output.status.code(),
        Some(rc),
        "{label}: expected rc{rc}, actual {:?}; stderr={:?}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    bytes_equal(&output.stdout, stdout, &format!("{label} stdout"));
    bytes_equal(&output.stderr, stderr, &format!("{label} stderr"));
}

fn tree_equal(actual: &Path, expected: &Path, label: &str) {
    let actual = files(actual);
    let expected = files(expected);
    assert_eq!(
        actual.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>(),
        "{label}: file inventory"
    );
    for (rel, data) in expected {
        bytes_equal(&actual[&rel], &data, &format!("{label}: {}", rel.display()));
    }
}

fn wildcard(value: &[u8], pattern: &[u8]) -> bool {
    let (mut v, mut p, mut star, mut retry) = (0, 0, None, 0);
    while v < value.len() {
        if p < pattern.len() && (pattern[p] == b'?' || pattern[p] == value[v]) {
            v += 1;
            p += 1;
        } else if p < pattern.len() && pattern[p] == b'*' {
            star = Some(p);
            p += 1;
            retry = v;
        } else if let Some(s) = star {
            retry += 1;
            v = retry;
            p = s + 1;
        } else {
            return false;
        }
    }
    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }
    p == pattern.len()
}

fn glob_matches(path: &Path, pattern: &str) -> bool {
    let path = path.to_str().unwrap().split('/').collect::<Vec<_>>();
    let pattern = pattern.split('/').collect::<Vec<_>>();
    path.len() == pattern.len()
        && path
            .iter()
            .zip(pattern)
            .all(|(v, p)| wildcard(v.as_bytes(), p.as_bytes()))
}

fn hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0, "fixture hex length");
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|part| u8::from_str_radix(std::str::from_utf8(part).unwrap(), 16).unwrap())
        .collect()
}

fn assertions(case: &Path, label: &str) {
    for row in sidecar(case, "assertions.tsv") {
        let fields = row.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 3, "{label} assertion shape");
        let page = read(&case.join("golden").join(fields[0]));
        let needle = hex(fields[2]);
        let found = needle.is_empty() || page.windows(needle.len()).any(|s| s == needle);
        match fields[1] {
            "contains" => assert!(found, "{label}: assertion contains {}", fields[0]),
            "absent" => assert!(!found, "{label}: assertion absent {}", fields[0]),
            op => panic!("{label}: assertion op {op}"),
        }
    }
}

fn fixture_inventory(area: &str, expected: &[(&str, &str)]) {
    let mut actual = BTreeSet::new();
    for color in ["green", "red"] {
        for entry in fs::read_dir(root().join("tests").join(area).join(color)).unwrap() {
            let path = entry.unwrap().path();
            assert!(
                path.is_dir() && !path.is_symlink(),
                "fixture case directory: {}",
                path.display()
            );
            actual.insert(format!(
                "{color}/{}",
                path.file_name().unwrap().to_str().unwrap()
            ));
        }
    }
    let expected = expected
        .iter()
        .map(|(color, name)| format!("{color}/{name}"))
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "tests/{area}: case inventory");
}

fn ui_case(color: &str, name: &str) {
    fixture_inventory("ui", UI_CASES);
    let fx = Fixture::new(color, name);
    let label = format!("tests/ui/{color}/{name}");
    let mut rows = 0;
    for (line, row) in text(&fx.case.join("case.tsv")).lines().enumerate() {
        if row.is_empty() || row.starts_with('#') {
            continue;
        }
        let fields = row.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 4, "{label}: case.tsv row {}", line + 1);
        let out = fx.scratch.0.join(format!("out-{line}"));
        let args = fields[0]
            .split(' ')
            .map(|arg| match arg {
                "@TREE@" => fx.tree_arg.clone(),
                "@OUT@" => out.to_str().unwrap().to_owned(),
                other => other.to_owned(),
            })
            .collect::<Vec<_>>();
        assert_ne!(args[0], "serve", "fixture must terminate");
        let result = fx.invoke(&args);
        result_equal(
            &result,
            fields[1].parse().unwrap(),
            &stream(&fx.case, fields[3]),
            &stream(&fx.case, fields[2]),
            &format!("{label}: row {} argv={}", line + 1, fields[0]),
        );
        if args[0] == "render" && color == "green" {
            tree_equal(&out, &fx.case.join("golden"), &label);
        }
        rows += 1;
    }
    assert!(rows > 0, "{label}: no rows");
    for (rel, data) in files(&fx.case.join("after")) {
        bytes_equal(
            &read(&fx.tree.join(&rel)),
            &data,
            &format!("{label} after/{}", rel.display()),
        );
    }
    for glob in sidecar(&fx.case, "absent-globs.txt") {
        let matched = paths(&fx.tree)
            .into_iter()
            .filter(|p| glob_matches(p, &glob))
            .collect::<Vec<_>>();
        assert!(
            matched.is_empty(),
            "{label}: absent {glob} matched {matched:?}"
        );
    }
    if color == "green" {
        let checked = fx.invoke(&["check".into(), fx.tree_arg.clone()]);
        result_equal(
            &checked,
            0,
            &checked.stdout,
            b"",
            &format!("{label}: final check"),
        );
        assertions(&fx.case, &label);
    }
    eprintln!("ui fixture PASS {color}/{name}: {rows} stream rows");
}

fn copy_case(color: &str, name: &str) {
    fixture_inventory("copy", COPY_CASES);
    let fx = Fixture::new("green", "empty-guidelines");
    let case = root().join(format!("tests/copy/{color}/{name}"));
    let result = fx.invoke(&[
        "copy-check".into(),
        case.join("source.txt").to_str().unwrap().into(),
    ]);
    let expected = if color == "red" {
        read(&case.join("expect.txt"))
    } else {
        Vec::new()
    };
    result_equal(
        &result,
        i32::from(color == "red"),
        &expected,
        b"",
        &format!("tests/copy/{color}/{name}"),
    );
}

fn blocks<'a>(page: &'a str, open: &str, close: &str) -> Vec<&'a str> {
    let mut remaining = page;
    let mut output = Vec::new();
    while let Some((_, tail)) = remaining.split_once(open) {
        let (body, rest) = tail
            .split_once(close)
            .expect("HTML block has a closing delimiter");
        output.push(body);
        remaining = rest;
    }
    output
}

fn headings(page: &str) -> Vec<String> {
    let mut remaining = page;
    let mut output = Vec::new();
    while let Some((_, tail)) = remaining.split_once("<h") {
        remaining = tail;
        if tail
            .as_bytes()
            .first()
            .is_some_and(|c| (b'1'..=b'6').contains(c))
        {
            let (_, content) = tail.split_once('>').unwrap();
            let close = format!("</h{}>", &tail[..1]);
            let (heading, rest) = content.split_once(&close).unwrap();
            output.push(heading.into());
            remaining = rest;
        }
    }
    output
}

fn colors(css: &str) -> BTreeSet<String> {
    css.split('#')
        .skip(1)
        .filter_map(|s| {
            let color = s
                .chars()
                .take_while(char::is_ascii_hexdigit)
                .collect::<String>();
            matches!(color.len(), 3 | 6).then(|| color.to_ascii_lowercase())
        })
        .collect()
}

fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::new();
    for part in bytes.chunks(3) {
        let n = (u32::from(part[0]) << 16)
            | (u32::from(*part.get(1).unwrap_or(&0)) << 8)
            | u32::from(*part.get(2).unwrap_or(&0));
        output.push(char::from(ALPHABET[((n >> 18) & 63) as usize]));
        output.push(char::from(ALPHABET[((n >> 12) & 63) as usize]));
        output.push(if part.len() > 1 {
            char::from(ALPHABET[((n >> 6) & 63) as usize])
        } else {
            '='
        });
        output.push(if part.len() > 2 {
            char::from(ALPHABET[(n & 63) as usize])
        } else {
            '='
        });
    }
    output
}

fn design_case(name: &str) {
    let fx = Fixture::new("green", name);
    let out = fx.scratch.0.join("design-out");
    let result = fx.invoke(&[
        "render".into(),
        out.to_str().unwrap().into(),
        fx.tree_arg.clone(),
    ]);
    result_equal(
        &result,
        0,
        &stream(&fx.case, "render.stdout.expect"),
        b"",
        &format!("design/{name}: render"),
    );
    let mut checked = 0;
    for (rel, golden_bytes) in files(&fx.case.join("golden")) {
        if rel.extension().is_none_or(|ext| ext != "html")
            || rel.components().any(|p| p.as_os_str() == "source")
        {
            continue;
        }
        let expected = String::from_utf8(golden_bytes).unwrap();
        let actual = text(&out.join(&rel));
        let label = format!("design/{name}/{}", rel.display());
        for required in [
            "<!doctype html>",
            "<html lang=\"en\">",
            "<h1",
            "<main id=\"main\">",
            "<style>",
            "<a class=\"skip\" href=\"#main\">Skip to content</a>",
        ] {
            assert_eq!(actual.matches(required).count(), 1, "{label}: {required}");
        }
        assert!(actual.contains("<nav"), "{label}: navigation");
        assert!(
            !actual.contains("tabindex") && !actual.contains("style="),
            "{label}: inline style/focus"
        );
        let scripts = blocks(&actual, "<script>", "</script>");
        let reference_scripts = blocks(&expected, "<script>", "</script>");
        assert!(scripts.len() <= 1, "{label}: at most one fixed script");
        assert_eq!(
            actual.matches("<script").count(),
            scripts.len(),
            "{label}: only canonical script elements"
        );
        assert_eq!(
            scripts, reference_scripts,
            "{label}: legacy script count/body"
        );
        let css = blocks(&actual, "<style>", "</style>");
        let reference_css = blocks(&expected, "<style>", "</style>");
        assert_eq!(
            colors(css[0]),
            colors(reference_css[0]),
            "{label}: palette hexes"
        );
        assert!(!css[0].contains("url("), "{label}: external CSS resource");
        assert!(!actual.contains(" src="), "{label}: external resource");
        let external = blocks(&actual, "href=\"", "\"")
            .into_iter()
            .filter(|s| s.starts_with("http") || s.starts_with("//"))
            .collect::<Vec<_>>();
        let reference_external = blocks(&expected, "href=\"", "\"")
            .into_iter()
            .filter(|s| s.starts_with("http") || s.starts_with("//"))
            .collect::<Vec<_>>();
        assert_eq!(
            external, reference_external,
            "{label}: only pinned repository-version links"
        );
        assert_eq!(
            headings(&actual),
            headings(&expected),
            "{label}: sentence-case copy headings (corpus titles remain literal)"
        );
        if let Some(script) = scripts.first() {
            let response = fx.invoke(&[
                "request".into(),
                "GET".into(),
                format!("/{}", rel.display()),
                fx.tree_arg.clone(),
            ]);
            assert_eq!(response.status.code(), Some(0), "{label}: request status");
            assert!(response.stderr.is_empty(), "{label}: request stderr");
            let response = String::from_utf8(response.stdout).unwrap();
            let (headers, body) = response.split_once("\n\n").unwrap();
            assert!(headers.starts_with("HTTP 200\n"), "{label}: HTTP status");
            let csp = format!(
                "Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline'; script-src 'sha256-{}'",
                base64(&Sha256::digest(script.as_bytes()))
            );
            assert_eq!(
                headers
                    .lines()
                    .filter(|s| s.starts_with("Content-Security-Policy:"))
                    .collect::<Vec<_>>(),
                vec![csp],
                "{label}: script hash in CSP"
            );
            bytes_equal(
                body.as_bytes(),
                actual.as_bytes(),
                &format!("{label}: request/render body"),
            );
        }
        checked += 1;
    }
    assert!(checked > 0, "design/{name}: no HTML pages");
    let result = fx.invoke(&["check".into(), fx.tree_arg.clone()]);
    result_equal(
        &result,
        0,
        &stream(&fx.case, "check.stdout.expect"),
        b"",
        &format!("design/{name}: check"),
    );
}

macro_rules! ui_cases {
    ($($test:ident => ($color:literal, $name:literal),)*) => {
        const UI_CASES: &[(&str, &str)] = &[$(($color, $name)),*];
        $(#[test] fn $test() { ui_case($color, $name); })*
    };
}
macro_rules! copy_cases {
    ($($test:ident => ($color:literal, $name:literal),)*) => {
        const COPY_CASES: &[(&str, &str)] = &[$(($color, $name)),*];
        $(#[test] fn $test() { copy_case($color, $name); })*
    };
}

// Generated only from the committed case directory inventory; drift is a failing test.
ui_cases! {
    ui_green_basic => ("green", "basic"),
    ui_green_empty_guidelines => ("green", "empty-guidelines"),
    ui_green_escaped_href_text => ("green", "escaped-href-text"),
    ui_green_git_uncommitted_edit => ("green", "git-uncommitted-edit"),
    ui_green_git_untracked_document => ("green", "git-untracked-document"),
    ui_green_header_only_ledger => ("green", "header-only-ledger"),
    ui_green_highlight => ("green", "highlight"),
    ui_green_hostile => ("green", "hostile"),
    ui_green_length_boundary => ("green", "length-boundary"),
    ui_green_locatorless_boundary => ("green", "locatorless-boundary"),
    ui_green_missing_readme_fallback => ("green", "missing-readme-fallback"),
    ui_green_multi_guideline_order => ("green", "multi-guideline-order"),
    ui_green_payload_selection => ("green", "payload-selection"),
    ui_green_unicode_digit_region => ("green", "unicode-digit-region"),
    ui_green_verdicts => ("green", "verdicts"),
    ui_red_align_one_sided => ("red", "align-one-sided"),
    ui_red_align_overlap => ("red", "align-overlap"),
    ui_red_align_span_mismatch => ("red", "align-span-mismatch"),
    ui_red_copy_visible_hex => ("red", "copy-visible-hex"),
    ui_red_coverage_restates_unclosed => ("red", "coverage-restates-unclosed"),
    ui_red_coverage_uncovered_noreason => ("red", "coverage-uncovered-noreason"),
    ui_red_coverage_unknown_status => ("red", "coverage-unknown-status"),
    ui_red_digest_mismatch_ace => ("red", "digest-mismatch-ace"),
    ui_red_digest_mismatch_payload => ("red", "digest-mismatch-payload"),
    ui_red_digest_precedence_ace => ("red", "digest-precedence-ace"),
    ui_red_digest_prefix_ace => ("red", "digest-prefix-ace"),
    ui_red_digest_prefix_payload => ("red", "digest-prefix-payload"),
    ui_red_digest_wrong_column => ("red", "digest-wrong-column"),
    ui_red_doc_missing_manifest => ("red", "doc-missing-manifest"),
    ui_red_duplicate_docid => ("red", "duplicate-docid"),
    ui_red_duplicate_docid_order => ("red", "duplicate-docid-order"),
    ui_red_duplicate_locator => ("red", "duplicate-locator"),
    ui_red_duplicate_review_manifest_doc => ("red", "duplicate-review-manifest-doc"),
    ui_red_empty_ledger => ("red", "empty-ledger"),
    ui_red_empty_payload => ("red", "empty-payload"),
    ui_red_first_violation_t1_before_t2 => ("red", "first-violation-t1-before-t2"),
    ui_red_http_403_foreign_host => ("red", "http-403-foreign-host"),
    ui_red_http_404 => ("red", "http-404"),
    ui_red_http_404_doc_near_match => ("red", "http-404-doc-near-match"),
    ui_red_http_404_gid_near_match => ("red", "http-404-gid-near-match"),
    ui_red_http_405 => ("red", "http-405"),
    ui_red_http_405_head => ("red", "http-405-head"),
    ui_red_http_digest_500 => ("red", "http-digest-500"),
    ui_red_http_digest_index_200 => ("red", "http-digest-index-200"),
    ui_red_http_ledger_500 => ("red", "http-ledger-500"),
    ui_red_http_precedence_digest => ("red", "http-precedence-digest"),
    ui_red_http_precedence_model => ("red", "http-precedence-model"),
    ui_red_http_traversal_404 => ("red", "http-traversal-404"),
    ui_red_http_viewmodel_500 => ("red", "http-viewmodel-500"),
    ui_red_invalid_docid => ("red", "invalid-docid"),
    ui_red_invalid_guideline_id => ("red", "invalid-guideline-id"),
    ui_red_invalid_utf8_ace => ("red", "invalid-utf8-ace"),
    ui_red_invalid_utf8_coverage => ("red", "invalid-utf8-coverage"),
    ui_red_invalid_utf8_manifest => ("red", "invalid-utf8-manifest"),
    ui_red_invalid_utf8_prolog => ("red", "invalid-utf8-prolog"),
    ui_red_invalid_utf8_readme => ("red", "invalid-utf8-readme"),
    ui_red_invalid_utf8_source => ("red", "invalid-utf8-source"),
    ui_red_ledger_invalid => ("red", "ledger-invalid"),
    ui_red_locatorless_census_mismatch => ("red", "locatorless-census-mismatch"),
    ui_red_malformed_adjudication_row => ("red", "malformed-adjudication-row"),
    ui_red_malformed_coverage_row => ("red", "malformed-coverage-row"),
    ui_red_malformed_review_manifest_row => ("red", "malformed-review-manifest-row"),
    ui_red_manifest_missing_doc => ("red", "manifest-missing-doc"),
    ui_red_method_matrix => ("red", "method-matrix"),
    ui_red_missing_ace => ("red", "missing-ace"),
    ui_red_missing_audit => ("red", "missing-audit"),
    ui_red_missing_coverage => ("red", "missing-coverage"),
    ui_red_missing_guidelines => ("red", "missing-guidelines"),
    ui_red_missing_manifest_file => ("red", "missing-manifest-file"),
    ui_red_missing_pl => ("red", "missing-pl"),
    ui_red_orphan_ace => ("red", "orphan-ace"),
    ui_red_orphan_pl => ("red", "orphan-pl"),
    ui_red_overlong_docid => ("red", "overlong-docid"),
    ui_red_overlong_guideline_id => ("red", "overlong-guideline-id"),
    ui_red_region_resolve_failure => ("red", "region-resolve-failure"),
    ui_red_unknown_ledger_docid => ("red", "unknown-ledger-docid"),
    ui_red_unsupported_control_bidi_readme => ("red", "unsupported-control-bidi-readme"),
    ui_red_unsupported_control_cr_readme => ("red", "unsupported-control-cr-readme"),
    ui_red_unsupported_control_del_readme => ("red", "unsupported-control-del-readme"),
    ui_red_unsupported_control_isolate_readme => ("red", "unsupported-control-isolate-readme"),
    ui_red_unsupported_control_readme => ("red", "unsupported-control-readme"),
    ui_red_usage_arity => ("red", "usage-arity"),
    ui_red_verdict_405 => ("red", "verdict-405"),
    ui_red_verdict_artifact_drift => ("red", "verdict-artifact-drift"),
    ui_red_verdict_artifact_drift_payload => ("red", "verdict-artifact-drift-payload"),
    ui_red_verdict_cas_conflict => ("red", "verdict-cas-conflict"),
    ui_red_verdict_crash => ("red", "verdict-crash"),
    ui_red_verdict_csrf => ("red", "verdict-csrf"),
    ui_red_verdict_field_grammar => ("red", "verdict-field-grammar"),
    ui_red_verdict_get_form => ("red", "verdict-get-form"),
    ui_red_verdict_host => ("red", "verdict-host"),
    ui_red_verdict_ledger_invalid => ("red", "verdict-ledger-invalid"),
    ui_red_verdict_manifest_derivation => ("red", "verdict-manifest-derivation"),
    ui_red_verdict_ok_append => ("red", "verdict-ok-append"),
    ui_red_verdict_ok_create => ("red", "verdict-ok-create"),
    ui_red_verdict_origin => ("red", "verdict-origin"),
    ui_red_verdict_path_decode => ("red", "verdict-path-decode"),
    ui_red_verdict_request_cli => ("red", "verdict-request-cli"),
    ui_red_verdict_subject_drift => ("red", "verdict-subject-drift"),
}

copy_cases! {
    copy_green_copy_clean => ("green", "copy-clean"),
    copy_green_copy_lookalike => ("green", "copy-lookalike"),
    copy_red_copy_css_animation => ("red", "copy-css-animation"),
    copy_red_copy_css_backdrop => ("red", "copy-css-backdrop"),
    copy_red_copy_css_boxshadow => ("red", "copy-css-boxshadow"),
    copy_red_copy_css_gradient => ("red", "copy-css-gradient"),
    copy_red_copy_css_keyframes => ("red", "copy-css-keyframes"),
    copy_red_copy_css_transition => ("red", "copy-css-transition"),
    copy_red_copy_emoji => ("red", "copy-emoji"),
    copy_red_copy_exclamatory => ("red", "copy-exclamatory"),
    copy_red_copy_marketing => ("red", "copy-marketing"),
    copy_red_copy_relative_time => ("red", "copy-relative-time"),
}

#[test]
fn design_basic() {
    design_case("basic");
}

#[test]
fn design_highlight() {
    design_case("highlight");
}
