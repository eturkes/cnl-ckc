use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Debug)]
pub(super) struct Failure {
    pub rc: u8,
    pub out: Vec<u8>,
    pub err: Vec<u8>,
}
pub(super) type Result<T = ()> = std::result::Result<T, Failure>;

fn message(category: &str, detail: impl AsRef<str>) -> Vec<u8> {
    format!(
        "goal: {category}: {}\n",
        detail.as_ref().replace('\n', "\\n").replace('\r', "\\r")
    )
    .into_bytes()
}
pub(super) fn violation(category: &str, detail: impl AsRef<str>) -> Failure {
    Failure {
        rc: 1,
        out: message(category, detail),
        err: vec![],
    }
}
pub(super) fn fail(category: &str, detail: impl AsRef<str>) -> Failure {
    Failure {
        rc: 2,
        out: vec![],
        err: message(category, detail),
    }
}
pub(super) fn emit(result: Result) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            std::io::stdout().write_all(&e.out).ok();
            std::io::stderr().write_all(&e.err).ok();
            ExitCode::from(e.rc)
        }
    }
}
pub(super) fn show(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
pub(super) fn name(path: &Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}
pub(super) fn entries(path: &Path, category: &str) -> Result<Vec<PathBuf>> {
    let mut paths = fs::read_dir(path)
        .map_err(|_| violation(category, format!("unreadable {}", show(path))))?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|_| violation(category, format!("unreadable {}", show(path))))?;
    paths.sort();
    Ok(paths)
}
pub(super) fn read(path: &Path, category: &str) -> Result<Vec<u8>> {
    fs::read(path).map_err(|_| violation(category, format!("unreadable {}", show(path))))
}
pub(super) fn corpus(path: &Path, category: &str) -> Result<Vec<u8>> {
    if path.is_symlink() {
        return Err(violation(category, format!("is a symlink: {}", show(path))));
    }
    if !path.is_file() {
        return Err(violation(category, format!("missing: {}", show(path))));
    }
    read(path, category)
}
pub(super) fn utf8(bytes: Vec<u8>, path: &Path, category: &str) -> Result<String> {
    String::from_utf8(bytes)
        .map_err(|_| violation(category, format!("invalid_utf8 {}", show(path))))
}
pub(super) fn text(path: &Path, category: &str) -> Result<String> {
    utf8(read(path, category)?, path, category)
}
pub(super) fn corpus_text(path: &Path, category: &str) -> Result<String> {
    utf8(corpus(path, category)?, path, category)
}
pub(super) fn valid_docid(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 250
        && !s.starts_with('-')
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}
pub(super) fn git(args: &[&str]) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .args(args)
        .output()
        .map_err(|_| fail("git", "not executable: git"))?;
    if !out.status.success() {
        return Err(Failure {
            rc: out.status.code().unwrap_or(2) as u8,
            out: vec![],
            err: out.stderr,
        });
    }
    Ok(out.stdout)
}
pub(super) fn git_text(args: &[&str]) -> Result<String> {
    String::from_utf8(git(args)?).map_err(|_| fail("git", "invalid_utf8 output"))
}
