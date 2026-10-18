// Shell-tier fixture test: the `ckc v1` envelope over one committed document
// (contract m5u2 R9/R19). Kernel acceptance = Verus theorems; this pins the
// shell's io + envelope bytes (rc, stdout, stderr).
use std::path::PathBuf;
use std::process::{Command, Output};

const DOC: &str = "guidelines/cdc-2022-opioid/pl/cdc2022-opioid-rec01-imp01.pl";

fn doc_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(DOC)
}

fn ckc(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ckc"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn check_accepts_and_render_echoes_committed_document() {
    let doc = doc_path();
    let bytes = std::fs::read(&doc).unwrap();
    let check = ckc(&["v1", "check", doc.to_str().unwrap()]);
    assert_eq!(
        (check.status.code(), check.stdout.len(), check.stderr.len()),
        (Some(0), 0, 0)
    );
    let render = ckc(&["v1", "render", doc.to_str().unwrap()]);
    assert_eq!(render.status.code(), Some(0));
    assert_eq!(render.stdout, bytes);
    assert!(render.stderr.is_empty());
}

#[test]
fn check_rejects_prepended_byte_at_line_1_column_1() {
    let mut bytes = b"x".to_vec();
    bytes.extend(std::fs::read(doc_path()).unwrap());
    let tmp = std::env::temp_dir().join(format!("ckc-cli-{}.pl", std::process::id()));
    std::fs::write(&tmp, bytes).unwrap();
    let out = ckc(&["v1", "check", tmp.to_str().unwrap()]);
    std::fs::remove_file(&tmp).unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert_eq!(
        out.stderr,
        b"ace_to_pl_error(check_load,noncanonical(1,1)).\n"
    );
}

#[test]
fn check_reports_unreadable_path() {
    let out = ckc(&["v1", "check", "/nonexistent/ckc-cli-probe.pl"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert_eq!(out.stderr, b"ace_to_pl_error(check_load,unreadable).\n");
}

#[test]
fn usage_without_arguments_is_rc2() {
    let out = ckc(&[]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert!(out.stderr.starts_with(b"usage: ckc "));
}

#[test]
fn v1_trace_arities_dispatch_on_the_mode_word() {
    const USAGE: &[u8] = b"usage: ckc v1 <check|render> <pl> | ckc v1 <aggregate-check|recursion-check> <manifest> | ckc v1 answer <manifest> <query.pl> | ckc v1 trace <manifest> <query.pl> <answers.pl> | ckc v1 trace-check <manifest> <query.pl> <answers.pl> <traces.pl>\n";
    for args in [
        &["v1", "answer", "m", "q", "a"][..],
        &["v1", "answer", "m", "q", "a", "t"],
        &["v1", "trace-check", "m", "q", "a"],
        &["v1", "trace", "m", "q", "a", "t"],
    ] {
        let out = ckc(args);
        assert_eq!(
            (
                out.status.code(),
                out.stdout.as_slice(),
                out.stderr.as_slice()
            ),
            (Some(2), &b""[..], USAGE),
            "{args:?}"
        );
    }
}

#[test]
fn pipeline_arity_keeps_legacy_fail_envelopes() {
    let cases = [
        ("compile", " <guideline-id>"),
        ("queries", " <guideline-id>"),
        ("align", " <guideline-id> <docid>"),
        ("review-manifest", " <guideline-id>"),
        ("derive-review-manifest", " <guideline-dir>"),
        ("ledger-validate", " <ledger-path> <manifest-path> <label>"),
        ("release-manifest", ""),
    ];
    for (mode, tail) in cases {
        let args = if mode == "release-manifest" {
            vec![mode, "extra"]
        } else {
            vec![mode]
        };
        let out = ckc(&args);
        assert_eq!(out.status.code(), Some(2), "{mode}");
        assert!(out.stdout.is_empty(), "{mode}");
        assert_eq!(
            out.stderr,
            format!("ckc: usage: expected: ckc {mode}{tail}\n").as_bytes(),
            "{mode}"
        );
    }
}

// R18 regular-file law on direct arguments: a FIFO, a device and a directory
// read as `unreadable` (a FIFO without a writer used to block the reader).
#[test]
#[cfg(target_os = "linux")]
fn check_and_render_reject_non_regular_paths() {
    let dir = std::env::temp_dir().join(format!("ckc-cli-special-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir(&dir).unwrap();
    let fifo = dir.join("fifo.pl");
    assert!(
        Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success()
    );
    for mode in ["check", "render"] {
        for path in [fifo.to_str().unwrap(), "/dev/null", dir.to_str().unwrap()] {
            let mut child = Command::new(env!("CARGO_BIN_EXE_ckc"))
                .args(["v1", mode, path])
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap();
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
            while child.try_wait().unwrap().is_none() {
                if std::time::Instant::now() > deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    let _ = std::fs::remove_dir_all(&dir);
                    panic!("{mode} {path}: still running after 20 s");
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            let out = child.wait_with_output().unwrap();
            assert_eq!(out.status.code(), Some(2), "{mode} {path}");
            assert!(out.stdout.is_empty(), "{mode} {path}");
            assert_eq!(
                out.stderr, b"ace_to_pl_error(check_load,unreadable).\n",
                "{mode} {path}"
            );
        }
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

// `ckc compile` refuses a symlinked `guidelines/schema.pl` (rc 2, as a symlinked pl dir)
// instead of writing the schema bytes through it: the link target stays untouched.
#[test]
#[cfg(unix)]
fn compile_refuses_a_symlinked_schema_file() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = std::env::temp_dir().join(format!("ckc-cli-schema-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("rust/ckc")).unwrap();
    std::os::unix::fs::symlink(repo.join("vendor"), root.join("vendor")).unwrap();
    std::os::unix::fs::symlink(repo.join("rust/ckc/prolog"), root.join("rust/ckc/prolog")).unwrap();
    let fixture = repo.join("tests/queries/green/positive/tree/guidelines");
    let copy = Command::new("cp")
        .arg("-a")
        .arg(&fixture)
        .arg(root.join("guidelines"))
        .status()
        .unwrap();
    assert!(copy.success());
    let outside = root.join("outside.txt");
    std::fs::write(&outside, b"keep\n").unwrap();
    std::os::unix::fs::symlink(&outside, root.join("guidelines/schema.pl")).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_ckc"))
        .args(["compile", "fx"])
        .current_dir(&root)
        .output()
        .unwrap();
    let text =
        String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
    let kept = std::fs::read(&outside).unwrap();
    std::fs::remove_dir_all(&root).unwrap();
    assert_eq!(out.status.code(), Some(2), "{text}");
    assert!(
        text.contains("ckc: schema-file: not a regular file: guidelines/schema.pl"),
        "{text}"
    );
    assert_eq!(kept, b"keep\n");
}
