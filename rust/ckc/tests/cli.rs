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
