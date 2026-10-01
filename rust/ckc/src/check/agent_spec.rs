use super::common::*;
use std::path::Path;
// `.agent/spec.md` = the template's five sections, each once and in order; `## Tasks`
// holds `- [ ] <unit>` and `- [x] <sha> <unit>` rows and ends on the deferral-queue pointer.
const SECTIONS: [&str; 5] = [
    "## Intent",
    "## Artifacts",
    "## Decisions",
    "## Tasks",
    "## Phase",
];
const POINTER: &str = "`.agent/deferred.md`";
fn sha(word: &str) -> bool {
    (7..=40).contains(&word.len())
        && word
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub(super) fn check() -> Result {
    let text = corpus_text(Path::new(".agent/spec.md"), "agent-spec")?;
    let heads: Vec<&str> = text.lines().filter(|l| l.starts_with("## ")).collect();
    if heads != SECTIONS {
        return Err(violation(
            "agent-spec",
            format!(
                "sections must read Intent, Artifacts, Decisions, Tasks, Phase: {}",
                heads.join(" | ")
            ),
        ));
    }
    let tasks: Vec<&str> = text
        .lines()
        .skip_while(|l| *l != "## Tasks")
        .skip(1)
        .take_while(|l| !l.starts_with("## "))
        .filter(|l| !l.trim().is_empty())
        .collect();
    let Some((last, rows)) = tasks.split_last() else {
        return Err(violation("agent-spec", "Tasks section is empty"));
    };
    if !last.starts_with("- ") || last.starts_with("- [") || !last.contains(POINTER) {
        return Err(violation(
            "agent-spec",
            format!("Tasks must end on the {POINTER} pointer: {last}"),
        ));
    }
    for (i, row) in rows.iter().enumerate() {
        let n = i + 1;
        if let Some(unit) = row.strip_prefix("- [ ] ") {
            if unit.trim().is_empty() {
                return Err(violation(
                    "agent-spec",
                    format!("Tasks row {n} names no unit"),
                ));
            }
        } else if let Some(rest) = row.strip_prefix("- [x] ") {
            if !rest.split(' ').next().is_some_and(sha) {
                return Err(violation(
                    "agent-spec",
                    format!("Tasks row {n} is ticked without a commit sha: {row}"),
                ));
            }
        } else {
            return Err(violation(
                "agent-spec",
                format!("Tasks row {n} is not a `- [ ]` or `- [x] <sha>` row: {row}"),
            ));
        }
    }
    println!("ckc: agent-spec ok {} task rows", rows.len());
    Ok(())
}
