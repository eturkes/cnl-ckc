use super::base64;
use super::oracle::{CSS, NARROW, PRINT_CSS};
use super::support::*;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const OLD_CSP: &str = "ceqWjP6N4e7DIvYMclbfnBzryu3YAwfoDBRiNcC8+hw=";
// Later recorded migration: SEC5 (security review F1) appends three CSP
// directives; the replay runs its script after the D10 one.
const SEC5_CSP: &str = "; base-uri 'none'; form-action 'self'; frame-ancestors 'none'";
const PRINT_END: &str =
    ".chip { border: 1px solid #111827; background: none; color: inherit; }\n}\n</style>";

fn baseline() -> Scratch {
    let scratch = Scratch::new("pre-unit");
    let archive = root().join("tests/ur-ui/pre-unit-ui.tar.gz");
    assert_eq!(
        sha(fs::read(&archive).unwrap()),
        BASE_ARCHIVE_SHA,
        "immutable b000bb33 fixture bytes"
    );
    let out = command(
        Path::new("tar"),
        &scratch.0,
        &[
            "-xzf",
            archive.to_str().unwrap(),
            "-C",
            scratch.0.to_str().unwrap(),
        ],
    );
    ok(&out, "pre-unit fixture extraction");
    scratch
}

fn lines(path: &Path) -> Vec<String> {
    if !path.is_file() {
        return Vec::new();
    }
    read(path)
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

fn current_script() -> String {
    let scratch = Scratch::new("script");
    let tree = scratch.0.join("tree");
    copy_tree(&root().join("tests/ui/green/highlight/tree"), &tree, false);
    let outdir = scratch.0.join("pages");
    ok(
        &native(
            &scratch.0,
            &[
                "ui",
                "render",
                outdir.to_str().unwrap(),
                tree.to_str().unwrap(),
            ],
        ),
        "fixed script carrier",
    );
    script(&read(&outdir.join("g/hl/doc/h-1.html")))
        .expect("highlight script")
        .to_owned()
}

fn page_file(path: &Path, bytes: &[u8]) -> bool {
    let relative = path.to_string_lossy();
    if relative.contains("/tree/") || relative.contains("/worktree/") {
        return false;
    }
    let Ok(value) = std::str::from_utf8(bytes) else {
        return false;
    };
    value.starts_with("<!doctype html>")
        || (value.starts_with("HTTP ")
            && value
                .split_once("\n\n")
                .is_some_and(|(_, body)| body.starts_with("<!doctype html>")))
}

fn removal_allowed(path: &str, heading: &str) -> bool {
    let timing_cases = [
        "timing",
        "q12-timing",
        "q13-timing",
        "temporal-words",
        "q12-temporal-words",
        "q13-temporal-words",
    ];
    if heading == "Timing as compiled" {
        timing_cases.iter().any(|name| {
            path == format!("green/{name}/golden/g/alpha/doc/h.html")
                || path == format!("green/{name}/golden/g/alpha/doc/t.html")
        })
    } else {
        ["temporal-words", "q12-temporal-words", "q13-temporal-words"]
            .iter()
            .any(|name| path == format!("green/{name}/golden/g/alpha/index.html"))
    }
}

fn repin_csp(value: &str, new_script: &str) -> String {
    if !value.starts_with("HTTP ") {
        return value.to_owned();
    }
    let (head, body) = value
        .split_once("\n\n")
        .expect("baseline response boundary");
    let anchor = format!("'sha256-{OLD_CSP}'");
    assert!(head.contains(&anchor), "baseline response CSP anchor");
    let hash = base64(&Sha256::digest(new_script.as_bytes()));
    format!(
        "{}\n\n{body}",
        head.replacen(&anchor, &format!("'sha256-{hash}'{SEC5_CSP}"), 1)
    )
}

fn remove_sections(label: &str, text: &mut String) -> String {
    let mut removed = String::new();
    for (level, heading) in [("h3", "Timing as compiled"), ("h2", "Time words")] {
        let start = format!("<section>\n<{level}>{heading}</{level}>\n");
        if let Some(at) = text.find(&start) {
            assert!(
                removal_allowed(label, heading),
                "D10 unexpected {heading} at {label}"
            );
            let end =
                at + text[at..].find("</section>\n").expect("section end") + "</section>\n".len();
            removed += &text[at..end];
            text.replace_range(at..end, "");
        }
    }
    removed
}

fn transformed(path: &Path, bytes: &[u8], new_script: &str) -> Vec<u8> {
    if !page_file(path, bytes) {
        let relative = path.to_string_lossy();
        if !relative.contains("/tree/")
            && !relative.contains("/worktree/")
            && let Ok(value) = std::str::from_utf8(bytes)
        {
            return repin_csp(value, new_script).into_bytes();
        }
        return bytes.to_vec();
    }
    let mut text = std::str::from_utf8(bytes).unwrap().to_owned();
    let label = path.to_str().unwrap();
    assert_eq!(
        text.matches(NARROW).count(),
        1,
        "{label}: baseline narrow-rule anchor"
    );
    text = text.replacen(NARROW, &format!("{CSS}{NARROW}"), 1);
    assert_eq!(
        text.matches(PRINT_END).count(),
        1,
        "{label}: baseline print-end anchor"
    );
    text = text.replacen(
        PRINT_END,
        &PRINT_END.replace("\n}\n</style>", &format!("\n{PRINT_CSS}}}\n</style>")),
        1,
    );
    if let Some(old_script) = script(&text).map(str::to_owned) {
        text = text.replacen(
            &format!("<script>{old_script}</script>"),
            &format!("<script>{new_script}</script>"),
            1,
        );
    }
    remove_sections(label, &mut text);
    repin_csp(&text, new_script).into_bytes()
}

struct LegacyFixture {
    scratch: Scratch,
    case: PathBuf,
    tree: PathBuf,
    tree_arg: String,
}
impl LegacyFixture {
    fn new(case: &Path, label: &Path) -> Self {
        let scratch = Scratch::new("legacy-case");
        let material = ["tree-order.txt", "empty-dirs.txt", "worktree"]
            .iter()
            .any(|p| case.join(p).exists());
        let relative = Path::new("tests/ui").join(label).join("tree");
        let tree = scratch.0.join(if material {
            PathBuf::from("materialized/tree")
        } else {
            relative.clone()
        });
        fs::create_dir_all(&tree).unwrap();
        let input = files(&case.join("tree"));
        let order = if case.join("tree-order.txt").is_file() {
            lines(&case.join("tree-order.txt"))
                .into_iter()
                .map(PathBuf::from)
                .collect()
        } else {
            input.keys().cloned().collect::<Vec<_>>()
        };
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(
            sorted,
            input.keys().cloned().collect::<Vec<_>>(),
            "legacy tree inventory"
        );
        for path in order {
            write(&tree.join(&path), &input[&path]);
        }
        for path in lines(&case.join("empty-dirs.txt")) {
            fs::create_dir_all(tree.join(path)).unwrap();
        }
        if case.join("worktree").is_dir() {
            for args in [
                vec!["init", "-q", "-b", "main"],
                vec!["add", "-A"],
                vec!["commit", "-q", "-m", "fixture corpus"],
            ] {
                ok(
                    &command(Path::new("git"), &tree, &args),
                    "legacy fixture git",
                );
            }
            copy_tree(&case.join("worktree"), &tree, false);
        }
        let tree_arg = if material {
            tree.to_str().unwrap().into()
        } else {
            relative.to_str().unwrap().into()
        };
        Self {
            scratch,
            case: case.to_owned(),
            tree,
            tree_arg,
        }
    }
}

#[test]
fn p7_every_legacy_render_and_http_body_changes_only_at_d10_sites() {
    let base = baseline();
    let ui = base.0.join("tests/ui");
    let new_script = current_script();
    let mut c = Checks::default();
    let mut rendered = 0;
    let mut requested = 0;
    for color in ["green", "red"] {
        let mut cases: Vec<_> = fs::read_dir(ui.join(color))
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        cases.sort();
        for case in cases {
            let label = case.strip_prefix(&ui).unwrap();
            if color == "green" && case.join("golden").is_dir() {
                let fx = LegacyFixture::new(&case, label);
                let outdir = fx.scratch.0.join("pages");
                let out = native(
                    &fx.scratch.0,
                    &[
                        "ui",
                        "render",
                        outdir.to_str().unwrap(),
                        fx.tree.to_str().unwrap(),
                    ],
                );
                ok(&out, "P7 legacy render");
                let output = files(&outdir);
                let golden = files(&case.join("golden"));
                assert_eq!(
                    output.keys().collect::<Vec<_>>(),
                    golden.keys().collect::<Vec<_>>(),
                    "P7 legacy render inventory"
                );
                for (path, expected) in golden {
                    let rel = label.join("golden").join(&path);
                    let want = transformed(&rel, &expected, &new_script);
                    c.equal(
                        std::str::from_utf8(&output[&path]).unwrap(),
                        std::str::from_utf8(&want).unwrap(),
                        &format!("P7 render {}", rel.display()),
                    );
                    if page_file(&rel, &expected) {
                        rendered += 1;
                    }
                }
            }
            let request_rows = lines(&case.join("case.tsv"));
            if !request_rows.iter().any(|l| l.starts_with("request ")) {
                continue;
            }
            let fx = LegacyFixture::new(&case, label);
            for (i, row) in request_rows.iter().enumerate() {
                let f: Vec<_> = row.split('\t').collect();
                assert_eq!(f.len(), 4);
                if !f[0].starts_with("request ") {
                    continue;
                }
                let mut args = vec!["ui".to_owned()];
                args.extend(f[0].split(' ').map(|arg| {
                    if arg == "@TREE@" {
                        fx.tree_arg.clone()
                    } else {
                        arg.into()
                    }
                }));
                let argv: Vec<_> = args.iter().map(String::as_str).collect();
                let out = native(&fx.scratch.0, &argv);
                c.require(
                    out.status.code() == Some(f[1].parse::<i32>().unwrap()),
                    format!("P7 {} request[{i}] rc", label.display()),
                );
                if f[3] == "-" {
                    c.require(out.stdout.is_empty(), "P7 empty HTTP output");
                    continue;
                }
                let bytes = fs::read(fx.case.join(f[3])).unwrap();
                let rel = label.join(f[3]);
                if !bytes.starts_with(b"HTTP ") {
                    continue;
                }
                let want = transformed(&rel, &bytes, &new_script);
                c.equal(
                    std::str::from_utf8(&out.stdout).unwrap(),
                    std::str::from_utf8(&want).unwrap(),
                    &format!("P7 HTTP {}", rel.display()),
                );
                requested += 1;
            }
        }
    }
    assert!(
        rendered >= 100 && requested >= 20,
        "P7 complete positive census: {rendered} pages, {requested} HTTP responses"
    );
    eprintln!("UR P7 census: {rendered} rendered pages; {requested} complete HTTP responses");
    c.finish("P7 legacy bytes");
}

fn retired_assertion(path: &Path, line: &str, before: &Files) -> bool {
    let fields: Vec<_> = line.trim_end_matches('\n').split('\t').collect();
    if fields.len() != 3 || fields[1] != "contains" {
        return false;
    }
    let target = path.parent().unwrap().join("golden").join(fields[0]);
    let mut remaining = String::from_utf8(before[&target].clone()).unwrap();
    let removed = remove_sections(target.to_str().unwrap(), &mut remaining);
    if removed.is_empty() {
        return false;
    }
    assert_eq!(fields[2].len() % 2, 0, "baseline assertion hex pairs");
    let needle: Vec<_> = fields[2]
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect();
    let needle = String::from_utf8(needle).unwrap();
    if !removed.contains(&needle) {
        return false;
    }
    assert!(
        !needle.is_empty() && !remaining.contains(&needle),
        "assertion also pins retained bytes: {} {line}",
        path.display()
    );
    true
}

fn compare_migration(
    actual: &Files,
    before: &Files,
    new_script: &str,
    label: &str,
    c: &mut Checks,
) {
    for (path, bytes) in before {
        if path.file_name().is_some_and(|s| s == "assertions.tsv") {
            let kept = std::str::from_utf8(bytes)
                .unwrap()
                .split_inclusive('\n')
                .filter(|line| !retired_assertion(path, line, before))
                .collect::<String>();
            let got = actual
                .get(path)
                .map(|b| std::str::from_utf8(b).unwrap())
                .unwrap_or("");
            c.require(
                actual.contains_key(path) && got == kept,
                format!(
                    "{label} {}: only D10 section-presence rows may leave",
                    path.display()
                ),
            );
        } else {
            let expected = transformed(path, bytes, new_script);
            c.require(
                actual.get(path).is_some_and(|b| *b == expected),
                format!("{label} {}: byte delta outside D10", path.display()),
            );
        }
    }
    for path in actual.keys() {
        c.require(
            before.contains_key(path),
            format!("{label} unexpected file {}", path.display()),
        );
    }
}

#[test]
fn p7_golden_migration_is_complete_bounded_and_idempotent() {
    let base = baseline();
    let ui = base.0.join("tests/ui");
    let before = files(&ui);
    let retired: usize = before
        .iter()
        .filter(|(path, _)| path.file_name().is_some_and(|s| s == "assertions.tsv"))
        .map(|(path, bytes)| {
            std::str::from_utf8(bytes)
                .unwrap()
                .split_inclusive('\n')
                .filter(|line| retired_assertion(path, line, &before))
                .count()
        })
        .sum();
    assert_eq!(retired, 119, "P7 frozen removed-section assertion census");
    let new_script = current_script();
    let mut c = Checks::default();
    compare_migration(
        &files(&root().join("tests/ui")),
        &before,
        &new_script,
        "P7 committed goldens",
        &mut c,
    );
    let migration = std::env::var_os("CKC_UR_MIGRATION")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join(".agent/archive/migrations/ur-golden-repin.py"));
    c.require(
        migration.is_file(),
        format!("P7 migration missing: {}", migration.display()),
    );
    if migration.is_file() {
        let sentinel = base.0.join("outside-ui.html");
        let sentinel_bytes =
            fs::read(root().join("tests/ui/green/basic/golden/index.html")).unwrap();
        write(&sentinel, &sentinel_bytes);
        let mut first = None;
        for pass in 1..=2 {
            let start = (c.count, c.failures.len());
            let out = command(
                Path::new("python3"),
                &base.0,
                &["-I", migration.to_str().unwrap(), base.0.to_str().unwrap()],
            );
            c.require(
                out.status.success(),
                format!(
                    "P7 migration pass {pass}: rc={:?} stderr={}",
                    out.status.code(),
                    String::from_utf8_lossy(&out.stderr)
                ),
            );
            let sec5 = root().join(".agent/archive/migrations/sec5-csp-repin.py");
            let out = command(
                Path::new("python3"),
                &base.0,
                &["-I", sec5.to_str().unwrap(), base.0.to_str().unwrap()],
            );
            c.require(
                out.status.success(),
                format!("P7 SEC5 migration pass {pass}: rc={:?}", out.status.code()),
            );
            let after = files(&ui);
            compare_migration(
                &after,
                &before,
                &new_script,
                &format!("P7 replay pass {pass}"),
                &mut c,
            );
            c.require(
                fs::read(&sentinel).unwrap() == sentinel_bytes,
                "P7 migration leaves outside-ui bytes alone",
            );
            if let Some(ref first) = first {
                c.require(*first == after, "P7 second replay byte-identical");
            } else {
                first = Some(after);
            }
            eprintln!(
                "UR P7 replay {pass}: {} checks, {} failures",
                c.count - start.0,
                c.failures.len() - start.1
            );
        }
    }
    c.finish("P7 migration");
}
