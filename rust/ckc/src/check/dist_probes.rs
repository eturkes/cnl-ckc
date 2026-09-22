use super::common::*;
use super::{pipeline_release, process};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

const RIGHTS: &str = "profile\tstatement\turl\tretrieved\tnote\n";
const GIT_ENV: [(&str, &str); 9] = [
    ("GIT_DEFAULT_HASH", "sha1"),
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_CONFIG_SYSTEM", "/dev/null"),
    ("GIT_AUTHOR_NAME", "fixture"),
    ("GIT_AUTHOR_EMAIL", "fixture@localhost"),
    ("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+00:00"),
    ("GIT_COMMITTER_NAME", "fixture"),
    ("GIT_COMMITTER_EMAIL", "fixture@localhost"),
    ("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+00:00"),
];
fn error(detail: impl AsRef<str>) -> Failure {
    violation("dist", detail)
}
fn io<T>(r: std::io::Result<T>) -> Result<T> {
    r.map_err(|e| error(e.to_string()))
}
fn git(repo: &Path, args: &[&str], action: &str) -> Result {
    let result = io(Command::new("git")
        .current_dir(repo)
        .envs(GIT_ENV)
        .args(args)
        .output())?;
    if result.status.success() {
        Ok(())
    } else {
        Err(error(format!("probe git {action} failed")))
    }
}
fn write(repo: &Path, path: &str, bytes: impl AsRef<[u8]>) -> Result {
    let path = repo.join(path);
    if let Some(parent) = path.parent() {
        io(fs::create_dir_all(parent))?;
    }
    io(fs::write(path, bytes))
}
fn commit(repo: &Path, message: &str) -> Result {
    git(repo, &["add", "-A"], "add")?;
    git(repo, &["commit", "-q", "-m", message], "commit")
}
fn rights(profile: &str, statement: &str) -> String {
    format!(
        "{RIGHTS}{profile}\t{statement}\thttps://example.invalid/g-probe\t2026-01-01\tProbe note.\n"
    )
}
fn seed(repo: &Path) -> Result {
    io(fs::create_dir(repo))?;
    git(repo, &["init", "-q", "-b", "main"], "init")?;
    for (path, text) in [
        (
            "docs/REFERENCE.md",
            "# Probe KB\n\n## Compiled Prolog schema (v1)\n\nProbe schema bytes.\n\n## Operating\n\nRun the checks.\n",
        ),
        ("NOTICE", "Probe notice.\n"),
        ("vendor/ape/prolog/ace_to_pl.pl", "% probe compiler\n"),
        ("vendor/clex/clex_lexicon.pl", "% probe lexicon\n"),
        (
            "guidelines/g-probe/source/original.txt",
            "probe source bytes\n",
        ),
        (
            "guidelines/g-probe/ace/doc-a.ace",
            "Every probe is a record.\n",
        ),
        (
            "guidelines/g-probe/pl/doc-a.pl",
            "guideline_document('g-probe','doc-a',[],x).\n",
        ),
    ] {
        write(repo, path, text)?;
    }
    write(
        repo,
        "guidelines/g-probe/rights.tsv",
        rights("redistributable", "Probe rights statement."),
    )
}
fn review(repo: &Path, ids: &[&str]) -> Result {
    let mut text = "# probe review manifest\n".to_owned();
    for id in ids {
        let part = crate::trust::sha256_hex(format!("component:{id}").as_bytes());
        let digest = crate::trust::sha256_hex(format!("review:{id}").as_bytes());
        text += &format!("{id}\t{part}\t{part}\t{part}\t{part}\t{digest}\n");
    }
    write(repo, "guidelines/g-probe/audit/review-manifest.tsv", text)
}
fn ledger(repo: &Path, id: &str, digest: &str, verdict: &str) -> Result {
    write(
        repo,
        "guidelines/g-probe/audit/adjudication.tsv",
        format!(
            "# probe ledger\n{id}\t{digest}\t\t{verdict}\tprobe\t2026-01-01T00:00:00Z\tprobe decision\n"
        ),
    )
}
pub(super) fn build(repo: &Path, dest: &Path) -> Result<process::Output> {
    let exe = io(std::env::current_exe())?;
    let (timed, out) = process::walled(
        Command::new(exe)
            .args(["dist", "build"])
            .arg(dest)
            .current_dir(repo),
        None,
        Duration::from_secs(300),
    )?;
    if timed {
        return Err(error("dist build exceeded 300s wall clock"));
    }
    Ok(out)
}
fn expect_refusal(out: &process::Output, name: &str, detail: &str) -> Result {
    let expected = format!("dist: {detail}\n");
    if out.rc != 1 || !out.out.is_empty() || out.err != expected.as_bytes() {
        return Err(error(format!(
            "probe {name} expected {}; got rc {} stderr {}",
            expected.trim(),
            out.rc,
            String::from_utf8_lossy(&out.err).trim()
        )));
    }
    Ok(())
}
pub(super) fn check(scratch: &Path) -> Result {
    for name in [
        "tamper",
        "rejected",
        "rights-missing",
        "rights-profile",
        "rights-statement",
        "label",
    ] {
        let repo = scratch.join(format!("probe-{name}"));
        seed(&repo)?;
        match name {
            "tamper" => review(&repo, &["doc-a"])?,
            "rejected" => {
                review(&repo, &["doc-a"])?;
                ledger(
                    &repo,
                    "doc-a",
                    &crate::trust::sha256_hex(b"review:doc-a"),
                    "rejected",
                )?;
            }
            "label" => {
                review(&repo, &["doc-a", "doc-b"])?;
                write(
                    &repo,
                    "guidelines/g-probe/pl/doc-b.pl",
                    "guideline_document('g-probe','doc-b',[],x).\n",
                )?;
                ledger(
                    &repo,
                    "doc-b",
                    &crate::trust::sha256_hex(b"stale:doc-b"),
                    "approved",
                )?;
            }
            _ => {}
        }
        commit(&repo, "probe corpus")?;
        let plan = pipeline_release::derive(&repo)?;
        write(&repo, "release-manifest.tsv", &plan.manifest)?;
        commit(&repo, "probe release manifest")?;
        let refusal = match name {
            "tamper" => {
                write(
                    &repo,
                    "guidelines/g-probe/source/original.txt",
                    "tampered probe source\n",
                )?;
                commit(&repo, "probe tamper")?;
                "manifest-drift data/guidelines/g-probe/source/original.txt"
            }
            "rejected" => "rejected-verdict doc-a",
            "rights-missing" => {
                io(fs::remove_file(repo.join("guidelines/g-probe/rights.tsv")))?;
                commit(&repo, "probe rights removed")?;
                "rights g-probe missing"
            }
            "rights-profile" => {
                write(
                    &repo,
                    "guidelines/g-probe/rights.tsv",
                    rights("other", "Probe rights statement."),
                )?;
                commit(&repo, "probe rights profile")?;
                "rights g-probe profile:1"
            }
            "rights-statement" => {
                write(
                    &repo,
                    "guidelines/g-probe/rights.tsv",
                    rights("redistributable", ""),
                )?;
                commit(&repo, "probe rights statement")?;
                "rights g-probe statement:1"
            }
            _ => {
                let text = String::from_utf8_lossy(&plan.manifest);
                for (id, label) in [("doc-a", "unreviewed"), ("doc-b", "stale")] {
                    if !text.contains(&format!("label\t{id}\t{label}\n")) {
                        return Err(error(format!("probe label missing {label} row for {id}")));
                    }
                }
                ""
            }
        };
        let out = build(&repo, Path::new("out"))?;
        if name == "label" {
            if out.rc != 0 || !out.out.starts_with(b"dist: ok ") {
                return Err(error(format!(
                    "probe label expected green build; got rc {}",
                    out.rc
                )));
            }
        } else {
            expect_refusal(&out, name, refusal)?;
        }
    }
    Ok(())
}
