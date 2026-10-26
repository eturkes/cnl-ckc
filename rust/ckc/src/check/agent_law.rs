use super::common::*;
use std::path::Path;
// Repository law a tool decides (CLAUDE.md `Engineering`: deterministic checks own each
// rule a tool can decide): local + generated paths stay ignored, the spec is the sole
// `@` import, every deferred row states its acceptance check, rule globs are quoted,
// operational text names no machine path, one queue entry is in progress at a time.
const IGNORED: [&str; 5] = [
    "/CLAUDE.local.md",
    "/.scratch/",
    "/.release/",
    "/.toolchain/",
    "rust/target/",
];
// A machine path opens a word (line start, space, quote, bracket, `=`), unlike a URL's
// `…/home/…` path segment.
fn machine_path(line: &str) -> bool {
    line.match_indices("/home/").any(|(i, _)| {
        line[..i]
            .chars()
            .next_back()
            .is_none_or(|c| c.is_whitespace() || "`'\"(=<[".contains(c))
    })
}
fn law(detail: String) -> Failure {
    violation("agent-law", detail)
}
pub(super) fn check() -> Result {
    let ignore = corpus_text(Path::new(".gitignore"), "agent-law")?;
    for want in IGNORED {
        if !ignore.lines().any(|l| l == want) {
            return Err(law(format!(".gitignore lacks {want}")));
        }
    }
    let claude = corpus_text(Path::new("CLAUDE.md"), "agent-law")?;
    let imports = claude.lines().filter(|l| l.starts_with('@')).count();
    if claude.lines().next() != Some("@.agent/spec.md") || imports != 1 {
        return Err(law(
            "CLAUDE.md: line 1 must be the sole import `@.agent/spec.md`".into(),
        ));
    }
    let deferred = corpus_text(Path::new(".agent/deferred.md"), "agent-law")?;
    for row in deferred.lines().filter(|l| l.starts_with("- ")) {
        if !row.contains(" Accept: ") {
            let head: String = row.chars().take(60).collect();
            return Err(law(format!(
                ".agent/deferred.md row without `Accept:`: {head}"
            )));
        }
    }
    let mut operational = vec![
        "CLAUDE.md".to_owned(),
        ".agent/spec.md".to_owned(),
        ".agent/deferred.md".to_owned(),
        "justfile".to_owned(),
        "README.md".to_owned(),
    ];
    for rule in entries(Path::new(".claude/rules"), "agent-law")? {
        let name = show(&rule);
        if !name.ends_with(".md") {
            continue;
        }
        let text = corpus_text(&rule, "agent-law")?;
        let mut fences = 0;
        for line in text.lines() {
            if line == "---" {
                fences += 1;
                if fences == 2 {
                    break;
                }
            } else if fences == 1
                && let Some(glob) = line.strip_prefix("  - ")
                && !(glob.len() >= 2 && glob.starts_with('"') && glob.ends_with('"'))
            {
                return Err(law(format!("{name}: unquoted paths: glob {glob}")));
            }
        }
        operational.push(name);
    }
    for path in operational {
        let text = corpus_text(Path::new(&path), "agent-law")?;
        if let Some((n, _)) = text.lines().enumerate().find(|(_, l)| machine_path(l)) {
            return Err(law(format!("{path}:{}: names a machine path", n + 1)));
        }
    }
    let queue = corpus_text(Path::new(".agent/queue.md"), "agent-law")?;
    let active = queue
        .lines()
        .filter(|l| l.starts_with("- ") && l.trim_end().ends_with("| in-progress"))
        .count();
    if active > 1 {
        return Err(law(format!(
            ".agent/queue.md: {active} entries in progress (max 1)"
        )));
    }
    println!("ckc: agent-law ok");
    Ok(())
}
