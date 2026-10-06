use super::common::*;
use super::fresh;
use ckc_kernel::EBundle;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const ARCHIVE_ERROR: &str = "ui: corpus: cannot read the committed guideline files";

pub(super) struct Corpus {
    pub root: PathBuf,
    pub real_root: PathBuf,
    pub commit: String,
    _snapshot: Option<Scratch>,
}
impl Corpus {
    pub fn worktree(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            real_root: root.to_path_buf(),
            commit: String::new(),
            _snapshot: None,
        }
    }
    // Bundle v2 of one guideline at a posted commit. The id passed the kernel's
    // 40-lowercase-hex parse (no leading `-`); the commit must exist and be the
    // snapshot commit or its ancestor. None = no attestation (fail-closed).
    pub fn derive_at(&self, commit: &str, gid: &str) -> Option<Vec<EBundle>> {
        if self.commit.is_empty() || !valid_hex(commit, 40) {
            return None;
        }
        let object = format!("{commit}^{{commit}}");
        git(&self.real_root, &["cat-file", "-e", &object])?;
        git(
            &self.real_root,
            &["merge-base", "--is-ancestor", commit, &self.commit],
        )?;
        let scratch = Scratch::new().ok()?;
        let path = format!("guidelines/{gid}");
        let dest = extract(&self.real_root, commit, &path, &scratch.0).ok()?;
        fresh::derive(&dest.join(&path)).ok()
    }
    pub fn committed(root: &Path) -> Result<Self> {
        let top = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(root)
            .output()
            .map_err(|_| ARCHIVE_ERROR.to_string())?;
        if !top.status.success() {
            return Ok(Self::worktree(root));
        }
        let absolute = root.canonicalize().map_err(|_| ARCHIVE_ERROR.to_string())?;
        let reported = PathBuf::from(text(&top.stdout).trim());
        if reported.canonicalize().ok().as_ref() != Some(&absolute) {
            return Ok(Self::worktree(root));
        }
        let head = git(&absolute, &["rev-parse", "HEAD"]);
        let Some(commit) = head
            .map(|b| text(&b).trim().to_owned())
            .filter(|s| valid_hex(s, 40))
        else {
            return Ok(Self::worktree(root));
        };
        let snapshot = Scratch::new()?;
        let dest = extract(&absolute, &commit, "guidelines", &snapshot.0)?;
        let guidelines = dest.join("guidelines");
        if guidelines.is_dir() {
            for path in entries(&guidelines).map_err(|_| ARCHIVE_ERROR)? {
                if !path.is_dir() {
                    continue;
                }
                let committed = path.join("audit/adjudication.tsv");
                let live = absolute
                    .join("guidelines")
                    .join(name(&path))
                    .join("audit/adjudication.tsv");
                if live.is_file() {
                    let bytes = fs::read(live).map_err(|_| ARCHIVE_ERROR)?;
                    fs::create_dir_all(committed.parent().ok_or(ARCHIVE_ERROR)?)
                        .map_err(|_| ARCHIVE_ERROR)?;
                    fs::write(committed, bytes).map_err(|_| ARCHIVE_ERROR)?;
                } else if committed.is_file() {
                    fs::remove_file(committed).map_err(|_| ARCHIVE_ERROR)?;
                }
            }
        }
        Ok(Self {
            root: dest,
            real_root: absolute,
            commit,
            _snapshot: Some(snapshot),
        })
    }
}
// Git owns tar serialization; preflight its immutable tree before extraction.
fn extract(repo: &Path, commit: &str, path: &str, scratch: &Path) -> Result<PathBuf> {
    let dest = scratch.join("corpus");
    fs::create_dir(&dest).map_err(|_| ARCHIVE_ERROR.to_string())?;
    let tree = git(repo, &["ls-tree", "-r", "-z", commit, "--", path]).ok_or(ARCHIVE_ERROR)?;
    for entry in tree.split(|b| *b == 0).filter(|e| !e.is_empty()) {
        let Some(split) = entry.iter().position(|b| *b == b'\t') else {
            return Err(ARCHIVE_ERROR.into());
        };
        let (meta, path) = (&entry[..split], &entry[split + 1..]);
        let path = std::str::from_utf8(path).map_err(|_| ARCHIVE_ERROR)?;
        if !(meta.starts_with(b"100644 blob ") || meta.starts_with(b"100755 blob "))
            || Path::new(path).is_absolute()
            || Path::new(path)
                .components()
                .any(|p| matches!(p, Component::ParentDir))
            || !path.starts_with("guidelines/")
        {
            return Err(ARCHIVE_ERROR.into());
        }
    }
    let archive = scratch.join("committed.tar");
    let out = Command::new("git")
        .args(["archive", "--format=tar", "-o"])
        .arg(&archive)
        .args([commit, path])
        .current_dir(repo)
        .output()
        .map_err(|_| ARCHIVE_ERROR.to_string())?;
    if !out.status.success() {
        return Err(ARCHIVE_ERROR.into());
    }
    let out = Command::new("tar")
        .args([
            "--extract",
            "--no-same-owner",
            "--no-same-permissions",
            "--file",
        ])
        .arg(&archive)
        .arg("--directory")
        .arg(&dest)
        .output()
        .map_err(|_| ARCHIVE_ERROR.to_string())?;
    if !out.status.success() {
        return Err(ARCHIVE_ERROR.into());
    }
    Ok(dest)
}
fn git(root: &Path, args: &[&str]) -> Option<Vec<u8>> {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| out.stdout)
}
