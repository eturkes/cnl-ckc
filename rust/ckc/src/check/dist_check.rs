use super::common::*;
use super::{dist_archive, dist_probes, pipeline_release};
use std::path::{Path, PathBuf};
use std::process::Command;

fn error(detail: impl AsRef<str>) -> Failure {
    violation("dist", detail)
}
fn archive(dir: &Path, second: bool) -> Result<PathBuf> {
    let paths: Vec<_> = entries(dir, "dist")?
        .into_iter()
        .filter(|p| {
            p.file_name()
                .is_some_and(|s| s.to_string_lossy().ends_with(".tar.gz"))
        })
        .collect();
    if paths.len() != 1 {
        return Err(error(format!(
            "{}live build archive count {}",
            if second { "second " } else { "" },
            paths.len()
        )));
    }
    Ok(paths[0].clone())
}
pub(super) fn check() -> Result {
    let root = Path::new(".");
    let plan = pipeline_release::derive(root)?;
    let manifest = Path::new("release-manifest.tsv");
    let hint = "; regenerate: python3 -P tools/goal.py release-manifest";
    if manifest.is_symlink() {
        return Err(error("release manifest is a symlink: release-manifest.tsv"));
    }
    if !manifest.exists() {
        return Err(error(format!(
            "release manifest missing: release-manifest.tsv{hint}"
        )));
    }
    if !manifest.is_file() {
        return Err(error(
            "release manifest is not a regular file: release-manifest.tsv",
        ));
    }
    if read(manifest, "dist")? != plan.manifest {
        return Err(error(format!(
            "release manifest stale: release-manifest.tsv{hint}"
        )));
    }
    let scratch = pipeline_release::Scratch::new()?;
    dist_probes::check(&scratch.0)?;
    if !plan.rejected.is_empty() || !plan.contested.is_empty() {
        println!(
            "goal: dist blocked rejected={} contested={}",
            plan.rejected.len(),
            plan.contested.len()
        );
        return Ok(());
    }
    let first = scratch.0.join("live-one");
    let second = scratch.0.join("live-two");
    let one = dist_probes::build(root, &first)?;
    if one.rc != 0 {
        return Err(error(format!(
            "live build failed: {}",
            String::from_utf8_lossy(&one.err).trim()
        )));
    }
    if !one.out.starts_with(b"dist: ok ") {
        return Err(error(format!(
            "live build meter grammar: {}",
            String::from_utf8_lossy(&one.out).trim()
        )));
    }
    if dist_probes::build(root, &second)?.rc != 0 {
        return Err(error("second live build failed"));
    }
    let raw = read(&archive(&first, false)?, "dist")?;
    if raw != read(&archive(&second, true)?, "dist")? {
        return Err(error("live builds are not byte-identical"));
    }
    let bag = format!(
        "cnl-ckc-kb-g{}",
        plan.head.chars().take(12).collect::<String>()
    );
    let extraction = scratch.0.join("extract");
    let members = dist_archive::extract(&raw, &bag, &extraction)?;
    let verify = Command::new("sha256sum")
        .args(["-c", "manifest-sha256.txt", "tagmanifest-sha256.txt"])
        .current_dir(extraction.join(bag))
        .output()
        .map_err(|e| error(e.to_string()))?;
    if !verify.status.success() {
        return Err(error(format!(
            "sha256sum verification failed rc {}",
            verify.status.code().unwrap_or(1)
        )));
    }
    if !verify.stderr.is_empty() {
        return Err(error("sha256sum verification stderr not empty"));
    }
    drop(scratch);
    println!(
        "goal: dist ok {} guidelines {members} members {} bytes",
        plan.shipped,
        raw.len()
    );
    Ok(())
}
