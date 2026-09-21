use super::common::*;
use super::text::lines;
use std::path::{Path, PathBuf};

pub(super) struct Guideline {
    pub path: PathBuf,
    pub docids: Vec<String>,
    pub lexicon: Option<PathBuf>,
}
impl Guideline {
    pub fn ace(&self, id: &str) -> PathBuf {
        self.path.join("ace").join(format!("{id}.ace"))
    }
    pub fn pl(&self, id: &str) -> PathBuf {
        self.path.join("pl").join(format!("{id}.pl"))
    }
}
fn source(path: &Path) -> Result {
    let root = show(path);
    let readme = path.join("README.md");
    if readme.is_symlink() {
        return Err(violation(
            "source-record",
            format!("README.md is a symlink under: {root}"),
        ));
    }
    if !readme.is_file() {
        return Err(violation(
            "source-record",
            format!("missing README.md under: {root}"),
        ));
    }
    let src = path.join("source");
    if src.is_symlink() {
        return Err(violation(
            "source-record",
            format!("source is a symlink under: {root}"),
        ));
    }
    if !src.is_dir() {
        return Err(violation(
            "source-record",
            format!("missing source directory under: {root}"),
        ));
    }
    let mut count = 0;
    for p in entries(&src, "source-record")? {
        if p.is_symlink() {
            return Err(violation(
                "source-record",
                format!("symlink under source: {}", name(&p)),
            ));
        }
        if p.is_file() {
            count += 1;
        }
    }
    if count == 0 {
        return Err(violation(
            "source-record",
            format!("no source files under: {root}"),
        ));
    }
    Ok(())
}
pub(super) fn collect(path: &Path) -> Result<Guideline> {
    let ace = path.join("ace");
    if ace.is_symlink() {
        return Err(fail(
            "guideline",
            format!("ace directory is a symlink: {}", show(&ace)),
        ));
    }
    if !ace.is_dir() {
        return Err(fail(
            "guideline",
            format!("missing ace directory: {}", show(&ace)),
        ));
    }
    let mut docids = vec![];
    for p in entries(&ace, "guideline")? {
        let n = name(&p);
        if !p.is_file() || p.is_symlink() {
            return Err(fail(
                "guideline",
                format!("entry is not a regular file: {n}"),
            ));
        }
        let Some(id) = n.strip_suffix(".ace") else {
            return Err(fail("guideline", format!("unsupported ace entry: {n}")));
        };
        if !valid_docid(id) {
            return Err(fail("docid", format!("invalid document id: {id}")));
        }
        docids.push(id.to_owned());
    }
    if docids.is_empty() {
        return Err(fail(
            "guideline",
            format!("no .ace documents in: {}", show(&ace)),
        ));
    }
    docids.sort();
    let lexicon = path.join("lexicon.ulex");
    if lexicon.is_symlink() {
        return Err(fail(
            "guideline",
            format!("lexicon is a symlink: {}", show(&lexicon)),
        ));
    }
    let lexicon = lexicon.is_file().then_some(lexicon);
    Ok(Guideline {
        path: path.to_owned(),
        docids,
        lexicon,
    })
}
fn derived(g: &Guideline, subdir: &str, suffix: &str) -> Result {
    let p = g.path.join(subdir);
    if p.is_symlink() {
        return Err(violation(
            &format!("{subdir}-dir"),
            format!("is a symlink: {}", show(&p)),
        ));
    }
    if !p.is_dir() {
        return Err(violation(&format!("missing-{subdir}"), show(&p)));
    }
    let mut actual = vec![];
    for p in entries(&p, &format!("{subdir}-entry"))? {
        let n = name(&p);
        if !p.is_file() || p.is_symlink() {
            return Err(violation(
                &format!("{subdir}-entry"),
                format!("not a regular file: {n}"),
            ));
        }
        actual.push(n);
    }
    let mut expected: Vec<String> = g.docids.iter().map(|id| format!("{id}.{suffix}")).collect();
    expected.sort();
    if actual != expected {
        return Err(violation(
            &format!("{subdir}-inventory"),
            format!("committed {subdir}/ does not match ace/ document set"),
        ));
    }
    Ok(())
}
pub(super) fn guidelines() -> Result<Vec<Guideline>> {
    let root = Path::new("guidelines");
    if root.is_symlink() {
        return Err(fail("guidelines", "guidelines directory is a symlink"));
    }
    if !root.is_dir() {
        return Err(fail("guidelines", "missing guidelines directory"));
    }
    let mut plans = vec![];
    for p in entries(root, "guidelines")? {
        let n = name(&p);
        if p.is_symlink() {
            return Err(violation("guideline-entry", format!("symlink: {n}")));
        }
        if !p.is_dir() {
            return Err(violation(
                "guideline-entry",
                format!("not a directory: {n}"),
            ));
        }
        if !valid_docid(&n) {
            return Err(violation(
                "guideline-entry",
                format!("invalid guideline id: {n}"),
            ));
        }
        source(&p)?;
        let g = collect(&p)?;
        derived(&g, "pl", "pl")?;
        derived(&g, "align", "tsv")?;
        plans.push(g);
    }
    if plans.is_empty() {
        return Err(violation("guidelines", "no guideline directories"));
    }
    Ok(plans)
}
fn pl_name(s: &str) -> bool {
    s.strip_suffix(".pl").is_some_and(valid_docid)
}
fn known_pl(s: &str) -> bool {
    if s.starts_with("vendor/ape/")
        || matches!(
            s,
            "vendor/clex/clex_lexicon.pl" | "rust/ckc/prolog/drs_dump.pl"
        )
    {
        return true;
    }
    let p: Vec<&str> = s.split('/').collect();
    match p.as_slice() {
        ["guidelines", id, "pl", file] => valid_docid(id) && pl_name(file),
        ["guidelines", id, "queries", dir, file] => {
            valid_docid(id) && ["pl", "answers", "traces"].contains(dir) && pl_name(file)
        }
        ["tests", "ui", color, _, tree, "guidelines", _, "pl", file] => {
            ["red", "green"].contains(color)
                && ["tree", "worktree"].contains(tree)
                && file.ends_with(".pl")
        }
        ["tests", "queries", color, case, pin, file] => {
            ["red", "green"].contains(color)
                && valid_docid(case)
                && ["answers-golden", "traces-golden"].contains(pin)
                && pl_name(file)
        }
        [
            "tests",
            "queries",
            color,
            case,
            "tree",
            "guidelines",
            id,
            "pl",
            file,
        ] => {
            ["red", "green"].contains(color)
                && valid_docid(case)
                && valid_docid(id)
                && pl_name(file)
        }
        [
            "tests",
            "queries",
            color,
            case,
            "tree",
            "guidelines",
            id,
            "queries",
            dir,
            file,
        ] => {
            ["red", "green"].contains(color)
                && valid_docid(case)
                && valid_docid(id)
                && ["pl", "answers", "traces"].contains(dir)
                && pl_name(file)
        }
        _ => false,
    }
}
pub(super) fn prolog() -> Result {
    let text = git_text(&["ls-files", "--", "*.pl"])?;
    let mut tracked: Vec<&str> = lines(&text).collect();
    tracked.sort();
    for path in tracked {
        if !known_pl(path) {
            return Err(violation(
                "prolog-inventory",
                format!("unauthorized tracked prolog: {path}"),
            ));
        }
    }
    Ok(())
}
pub(super) struct RedProbe {
    pub path: PathBuf,
    pub class: String,
    pub rc: i32,
}
pub(super) fn red() -> Result<Vec<RedProbe>> {
    let root = Path::new("tests/red");
    if root.is_symlink() {
        return Err(violation("red-dir", "is a symlink: tests/red"));
    }
    if !root.is_dir() {
        return Err(violation("red-dir", "missing: tests/red"));
    }
    let mut probes = vec![];
    let (mut aces, mut ulex, mut pins) = (vec![], vec![], vec![]);
    for p in entries(root, "red-entry")? {
        let n = name(&p);
        if !p.is_file() || p.is_symlink() {
            return Err(violation("red-entry", format!("not a regular file: {n}")));
        }
        if let Some(stem) = n.strip_suffix(".ace") {
            let Some((class, _)) = stem.split_once("--").filter(|(c, _)| !c.is_empty()) else {
                return Err(violation(
                    "red-probe",
                    format!("probe name lacks <class>-- prefix: {n}"),
                ));
            };
            let rc = match class {
                "input_utf8" | "ape_messages" | "empty_drs" | "sentence_lines" | "unsupported"
                | "safety" | "proof" => 1,
                "usage" | "ape_load" | "ulex_load" | "check_load" | "uncaught" => 2,
                _ => {
                    return Err(violation(
                        "red-class",
                        format!("unknown error class: {class}"),
                    ));
                }
            };
            aces.push(stem.to_owned());
            probes.push(RedProbe {
                path: p,
                class: class.to_owned(),
                rc,
            });
        } else if let Some(stem) = n.strip_suffix(".ulex") {
            ulex.push(stem.to_owned());
        } else if let Some(stem) = n.strip_suffix(".expect") {
            pins.push(stem.to_owned());
        } else {
            return Err(violation("red-entry", format!("unsupported entry: {n}")));
        }
    }
    for (items, kind) in [(&ulex, "ulex"), (&pins, "expect")] {
        for s in items {
            if !aces.contains(s) {
                return Err(violation(
                    "red-entry",
                    format!("orphan {kind} without ace probe: {s}"),
                ));
            }
        }
    }
    for s in aces {
        if !pins.contains(&s) {
            return Err(violation(
                "red-entry",
                format!("probe lacks expect pin: {s}"),
            ));
        }
    }
    if probes.is_empty() {
        return Err(violation("red-dir", "no red probes found"));
    }
    Ok(probes)
}
