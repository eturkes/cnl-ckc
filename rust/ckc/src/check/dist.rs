use super::common::*;
use super::{dist_archive, pipeline_release};
use pipeline_release::{ReleasePlan, member};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicU64, Ordering};

fn error(detail: impl AsRef<str>) -> Failure {
    violation("dist", detail)
}
fn row_map<'a>(text: &'a str, kind: &str) -> BTreeMap<&'a str, &'a str> {
    text.split('\n')
        .filter_map(|line| {
            let mut fields = line.split('\t');
            (fields.next()? == kind).then_some((fields.next()?, line))
        })
        .collect()
}
fn drift(committed: &str, derived: &str) -> String {
    for kind in ["member", "source", "label", "meta"] {
        let a = row_map(committed, kind);
        let b = row_map(derived, kind);
        let mut keys: Vec<_> = a.keys().chain(b.keys()).copied().collect();
        keys.sort_unstable();
        for key in keys {
            if a.get(key).copied().unwrap_or("") != b.get(key).copied().unwrap_or("") {
                return key.to_owned();
            }
        }
    }
    "release-manifest.tsv".to_owned()
}
fn prepare(root: &Path) -> Result<ReleasePlan> {
    let plan = pipeline_release::derive(root)?;
    let out = Command::new("git")
        .current_dir(root)
        .args(["show", "HEAD:release-manifest.tsv"])
        .output()
        .map_err(|_| error("manifest-drift release-manifest.tsv"))?;
    if !out.status.success() {
        return Err(error("manifest-drift release-manifest.tsv"));
    }
    let committed = std::str::from_utf8(&out.stdout)
        .map_err(|_| error("manifest-drift release-manifest.tsv"))?;
    if out.stdout != plan.manifest {
        return Err(error(format!(
            "manifest-drift {}",
            drift(committed, &String::from_utf8_lossy(&plan.manifest))
        )));
    }
    if let Some(id) = plan.rejected.first() {
        return Err(error(format!("rejected-verdict {id}")));
    }
    if let Some(id) = plan.contested.first() {
        return Err(error(format!("contested-verdict {id}")));
    }
    Ok(plan)
}
// `<dir>/<target>.tmp.<pid>.<n>`. A killed run skips Drop ⇒ a later run that draws
// its pid finds its names taken and steps past them.
static NEXT: AtomicU64 = AtomicU64::new(0);
struct PendingFile(PathBuf);
impl PendingFile {
    fn write(dir: &Path, target: &str, bytes: &[u8]) -> Result<Self> {
        let pid = std::process::id();
        let (path, mut file) = loop {
            let path = dir.join(format!(
                "{target}.tmp.{pid}.{}",
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(file) => break (path, file),
                Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
                Err(e) => return Err(error(e.to_string())),
            }
        };
        let pending = Self(path);
        file.write_all(bytes).map_err(|e| error(e.to_string()))?;
        Ok(pending)
    }
    fn publish(self, target: &Path) -> Result {
        fs::rename(&self.0, target).map_err(|e| error(e.to_string()))
    }
}
impl Drop for PendingFile {
    fn drop(&mut self) {
        fs::remove_file(&self.0).ok();
    }
}
fn publish(requested: &Path, name: &str, raw: &[u8]) -> Result {
    // pathlib removes dot/empty components before checking the final symlink.
    let normalized: PathBuf = requested.components().collect();
    let dest = if normalized.as_os_str().is_empty() {
        Path::new(".")
    } else {
        &normalized
    };
    if dest.is_symlink() {
        return Err(error(format!(
            "dest destination is a symlink: {}",
            requested.display()
        )));
    }
    if dest.exists() {
        if !dest.is_dir() {
            return Err(error(format!(
                "dest destination is not a directory: {}",
                requested.display()
            )));
        }
    } else {
        fs::create_dir_all(dest).map_err(|e| error(e.to_string()))?;
    }
    let archive = dest.join(name);
    if archive.is_symlink() {
        return Err(error(format!("dest archive path is a symlink: {name}")));
    }
    if archive.exists() {
        if !archive.is_file() {
            return Err(error(format!(
                "dest archive path is not a regular file: {name}"
            )));
        }
        if fs::read(&archive).map_err(|e| error(e.to_string()))? != raw {
            return Err(error(format!("output-collision {name}")));
        }
    }
    let sidecar_name = format!("{name}.sha256");
    let sidecar = dest.join(&sidecar_name);
    if sidecar.is_symlink() {
        return Err(error(format!(
            "dest sidecar path is a symlink: {sidecar_name}"
        )));
    }
    if sidecar.exists() && !sidecar.is_file() {
        return Err(error(format!(
            "dest sidecar path is not a regular file: {sidecar_name}"
        )));
    }
    PendingFile::write(dest, name, raw)?.publish(&archive)?;
    let digest = format!("{}  {name}\n", crate::trust::sha256_hex(raw));
    PendingFile::write(dest, &sidecar_name, digest.as_bytes())?.publish(&sidecar)
}
pub(super) struct Build {
    pub name: String,
    pub raw: Vec<u8>,
    pub shipped: usize,
    pub members: usize,
}
pub(super) fn build(root: &Path, dest: &Path) -> Result<Build> {
    let plan = prepare(root)?;
    let bag = format!(
        "cnl-ckc-kb-g{}",
        plan.head.chars().take(12).collect::<String>()
    );
    let payload_members = plan.payload.iter().map(|(p, b)| member(p, b)).collect();
    let digest = ckc_kernel::contract::dist_digest_lines(&payload_members);
    let mut tags = plan.tags;
    tags.insert("release-manifest.tsv".to_owned(), plan.manifest);
    tags.insert("manifest-sha256.txt".to_owned(), digest);
    let tag_members = tags.iter().map(|(p, b)| member(p, b)).collect();
    let tagmanifest = ckc_kernel::contract::dist_tagmanifest_lines(&tag_members);
    tags.insert("tagmanifest-sha256.txt".to_owned(), tagmanifest);
    let files: BTreeMap<_, _> = plan
        .payload
        .into_iter()
        .chain(tags)
        .map(|(p, b)| (format!("{bag}/{p}"), b))
        .collect();
    let raw = dist_archive::build(&files, plan.epoch)?;
    let name = format!("{bag}.tar.gz");
    publish(dest, &name, &raw)?;
    Ok(Build {
        name,
        raw,
        shipped: plan.shipped,
        members: files.len(),
    })
}
fn refusal(e: Failure) -> ExitCode {
    let message = if e.err.is_empty() { &e.out } else { &e.err };
    let text = String::from_utf8_lossy(message);
    let text = text.trim_end_matches(['\r', '\n']);
    let detail = text.strip_prefix("ckc: dist: ").unwrap_or(text);
    eprintln!("dist: {}", detail.replace('\n', "\\n").replace('\r', "\\r"));
    ExitCode::from(1)
}
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.first().map(String::as_str) != Some("build") || args.len() > 2 {
        eprintln!("dist: usage: ckc dist build [<dest>]");
        return ExitCode::from(2);
    }
    let dest = args.get(1).map(String::as_str).unwrap_or("dist");
    match build(Path::new("."), Path::new(dest)) {
        Ok(built) => {
            println!(
                "dist: ok {} guidelines {} members {} bytes",
                built.shipped,
                built.members,
                built.raw.len()
            );
            println!(
                "dist: sha256={} {}",
                crate::trust::sha256_hex(&built.raw),
                built.name
            );
            ExitCode::SUCCESS
        }
        Err(e) => refusal(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Own-pid pending files a killed run left behind (either name form): publish steps
    // past them, renames its own, and leaves the leftovers untouched.
    #[test]
    fn publish_steps_past_taken_pending_names() {
        let dest = std::env::temp_dir().join(format!("ckc-dist-test.{}", std::process::id()));
        fs::remove_dir_all(&dest).ok();
        fs::create_dir(&dest).unwrap();
        let (pid, next) = (std::process::id(), NEXT.load(Ordering::Relaxed));
        let mut planted = vec![];
        for (target, end) in [("x.tar.gz", next + 6), ("x.tar.gz.sha256", next + 10)] {
            planted.push(format!("{target}.tmp.{pid}"));
            planted.extend((next..end).map(|n| format!("{target}.tmp.{pid}.{n}")));
        }
        for name in &planted {
            fs::write(dest.join(name), b"leftover").unwrap();
        }
        publish(&dest, "x.tar.gz", b"archive").unwrap();
        assert_eq!(fs::read(dest.join("x.tar.gz")).unwrap(), b"archive");
        assert_eq!(
            fs::read(dest.join("x.tar.gz.sha256")).unwrap(),
            format!("{}  x.tar.gz\n", crate::trust::sha256_hex(b"archive")).into_bytes()
        );
        let mut left: Vec<_> = fs::read_dir(&dest)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|n| n.contains(".tmp."))
            .collect();
        left.sort();
        planted.sort();
        assert_eq!(left, planted);
        for name in &planted {
            assert_eq!(fs::read(dest.join(name)).unwrap(), b"leftover", "{name}");
        }
        fs::remove_dir_all(&dest).unwrap();
    }
}
