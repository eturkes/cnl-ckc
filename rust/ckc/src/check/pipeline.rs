use super::common::*;
use super::{adjudication, coverage, inventories};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn normalized_path(arg: &str) -> PathBuf {
    PathBuf::from(show(Path::new(arg)))
}

pub(super) fn guideline_path(id: &str, align: bool) -> Result<PathBuf> {
    let category = if align { "align" } else { "guideline" };
    if !valid_docid(id) {
        return Err(fail(category, format!("invalid guideline id: {id}")));
    }
    let path = Path::new("guidelines").join(id);
    validate_path(&path, align)?;
    Ok(path)
}

fn validate_path(path: &Path, align: bool) -> Result {
    let category = if align { "align" } else { "guideline" };
    let prefix = if align { "guideline " } else { "" };
    if path.is_symlink() {
        return Err(fail(
            category,
            format!("{prefix}is a symlink: {}", show(path)),
        ));
    }
    if !path.is_dir() {
        return Err(fail(
            category,
            format!("{prefix}not a directory: {}", show(path)),
        ));
    }
    Ok(())
}

fn align(gid: &str, id: &str) -> Result {
    let path = guideline_path(gid, true)?;
    let g = inventories::collect(&path)?;
    if !g.docids.iter().any(|d| d == id) {
        return Err(fail("align", format!("unknown docid: {id}")));
    }
    let c = coverage::check(&g, false)?;
    let (_, payload) = ckc_kernel::contract::check_payload(&c, id.as_bytes());
    let src = payload.ok_or_else(|| fail("align", format!("no payload for docid: {id}")))?;
    let ace = fs::read(g.ace(id))
        .map_err(|_| fail("align", format!("missing ace file: {}", show(&g.ace(id)))))?;
    let src: Vec<char> = std::str::from_utf8(&src)
        .map_err(|_| fail("align", "invalid_utf8 source"))?
        .chars()
        .collect();
    let ace: Vec<char> = std::str::from_utf8(&ace)
        .map_err(|_| fail("align", "invalid_utf8 ace"))?
        .chars()
        .collect();
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| fail("align", e.to_string()))?;
    // Text-mode stdin uses universal newlines, as the reference command does.
    let input: Vec<char> = input
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .chars()
        .collect();
    let result = match ckc_kernel::contract::align_resolve(&input, &src, &ace) {
        ckc_kernel::EResolve::Err(detail) => {
            return Err(fail("align", detail.iter().collect::<String>()));
        }
        ckc_kernel::EResolve::Ok(r) => r,
    };
    let dir = path.join("align");
    if dir.is_symlink() {
        return Err(fail(
            "align",
            format!("align directory is a symlink: {}", show(&dir)),
        ));
    }
    if !dir.is_dir() {
        fs::create_dir(&dir).map_err(|e| fail("align", e.to_string()))?;
    }
    let text: String = result.text.iter().collect();
    fs::write(dir.join(format!("{id}.tsv")), text.as_bytes())
        .map_err(|e| fail("align", e.to_string()))?;
    println!(
        "goal: align {id} groups {} spans {}",
        result.groups, result.spans
    );
    Ok(())
}

fn review(path: &Path, write: bool) -> Result {
    validate_path(path, false)?;
    let g = inventories::collect(path)?;
    let c = coverage::check(&g, false)?;
    let bundles = adjudication::derive(&g, &c)?;
    let bytes = ckc_kernel::contract::check_print_manifest(&bundles);
    if write {
        let target = path.join("audit/review-manifest.tsv");
        if target.is_symlink() {
            return Err(violation(
                "adjudication",
                format!("manifest is a symlink: {}", show(&target)),
            ));
        }
        if target.exists() && !target.is_file() {
            return Err(violation(
                "adjudication",
                format!("manifest is not a regular file: {}", show(&target)),
            ));
        }
        fs::write(&target, bytes).map_err(|e| fail("adjudication", e.to_string()))?;
        println!(
            "goal: review-manifest {} {} documents",
            name(path),
            g.docids.len()
        );
    } else {
        coverage::meter(&bytes);
    }
    Ok(())
}

pub(crate) fn run(args: &[String]) -> ExitCode {
    emit(match args.first().map(String::as_str) {
        Some("release-manifest") if args.len() == 1 => super::pipeline_release::run(),
        Some("compile") if args.len() == 2 => super::pipeline_emit::compile(&args[1]),
        Some("queries") if args.len() == 2 => super::pipeline_emit::queries(&args[1]),
        Some("align") if args.len() == 3 => align(&args[1], &args[2]),
        Some("review-manifest") if args.len() == 2 => {
            guideline_path(&args[1], false).and_then(|p| review(&p, true))
        }
        Some("derive-review-manifest") if args.len() == 2 => {
            review(&normalized_path(&args[1]), false)
        }
        Some("ledger-validate") if args.len() == 4 => adjudication::validate(
            &normalized_path(&args[1]),
            &normalized_path(&args[2]),
            &args[3],
        )
        .map(|b| coverage::meter(&b)),
        _ => Err(fail(
            "usage",
            match args.first().map(String::as_str) {
                Some("compile") => "expected: goal compile <guideline-id>",
                Some("queries") => "expected: goal queries <guideline-id>",
                Some("align") => "expected: goal align <guideline-id> <docid>",
                Some("review-manifest") => "expected: goal review-manifest <guideline-id>",
                Some("derive-review-manifest") => {
                    "expected: goal derive-review-manifest <guideline-dir>"
                }
                Some("ledger-validate") => {
                    "expected: goal ledger-validate <ledger-path> <manifest-path> <label>"
                }
                Some("release-manifest") => "expected: goal release-manifest",
                _ => "invalid pipeline arguments",
            },
        )),
    })
}
