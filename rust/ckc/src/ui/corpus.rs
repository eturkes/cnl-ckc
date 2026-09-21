use super::common::*;
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
        let dest = snapshot.0.join("corpus");
        fs::create_dir(&dest).map_err(|_| ARCHIVE_ERROR.to_string())?;
        // Git owns tar serialization; preflight its immutable tree before extraction.
        let tree = git(
            &absolute,
            &["ls-tree", "-r", "-z", &commit, "--", "guidelines"],
        )
        .ok_or(ARCHIVE_ERROR)?;
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
        let archive = snapshot.0.join("committed.tar");
        let out = Command::new("git")
            .args(["archive", "--format=tar", "-o"])
            .arg(&archive)
            .args([&commit, "guidelines"])
            .current_dir(&absolute)
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
    pub fn ace_commit(&self, gid: &str, docid: &str) -> Vec<u8> {
        if self.commit.is_empty() {
            return Vec::new();
        }
        let path = self
            .real_root
            .join("guidelines")
            .join(gid)
            .join("ace")
            .join(format!("{docid}.ace"));
        let Some(bytes) = git(
            &self.real_root,
            &[
                "log",
                "-1",
                "--format=%H",
                &self.commit,
                "--",
                &path.to_string_lossy(),
            ],
        ) else {
            return Vec::new();
        };
        let value = text(&bytes).trim().to_owned();
        if valid_hex(&value, 40) {
            value.into_bytes()
        } else {
            Vec::new()
        }
    }
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
