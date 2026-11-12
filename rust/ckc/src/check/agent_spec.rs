use super::common::*;
use std::path::Path;
// `.agent/spec.md` = the template's five sections, each once and in order; `## Tasks`
// holds `- [ ] <unit>` and `- [x] <sha> <unit>` rows and ends on the deferral-queue pointer.
// Both read the lines `.agent/spec-lint` reads: fenced blocks skipped by its rule (`unfenced`).
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
// spec-lint's fence rule (Markdown's): ≤ 3 leading spaces, then ≥ 3 backticks (no backtick after
// them) or tildes open a block; a line repeating the opener's character, at least as long, with
// blanks alone after it closes the block; opener, content + closer are skipped.
fn unfenced(text: &str) -> Vec<&str> {
    let mut fence: Option<(u8, usize)> = None;
    let mut kept = Vec::new();
    for line in text.lines() {
        let t = &line[line.bytes().take(3).take_while(|&b| b == b' ').count()..];
        let mark = t.bytes().next().filter(|&b| b == b'`' || b == b'~');
        let run = mark.map_or(0, |m| t.bytes().take_while(|&b| b == m).count());
        let rest = &t[run..];
        if let Some((open, len)) = fence {
            if mark == Some(open) && run >= len && rest.bytes().all(|b| b == b' ' || b == b'\t') {
                fence = None;
            }
        } else if let Some(m) = mark.filter(|&m| run >= 3 && (m == b'~' || !rest.contains('`'))) {
            fence = Some((m, run));
        } else {
            kept.push(line);
        }
    }
    kept
}
pub(super) fn check() -> Result {
    let text = corpus_text(Path::new(".agent/spec.md"), "agent-spec")?;
    let rows = shape(&text)?;
    println!("ckc: agent-spec ok {rows} task rows");
    Ok(())
}
fn shape(text: &str) -> Result<usize> {
    let lines = unfenced(text);
    let heads: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|l| l.starts_with("## "))
        .collect();
    if heads != SECTIONS {
        return Err(violation(
            "agent-spec",
            format!(
                "sections must read Intent, Artifacts, Decisions, Tasks, Phase: {}",
                heads.join(" | ")
            ),
        ));
    }
    let tasks: Vec<&str> = lines
        .iter()
        .copied()
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
    Ok(rows.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;

    // Each plant lands at the end of `Decisions`; expected = Markdown's reading, which
    // `.agent/spec-lint` gives too (rc 0 | 1 on the planted copy).
    #[test]
    fn fenced_lines_read_as_spec_lint_reads_them() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let spec = fs::read_to_string(root.join(".agent/spec.md")).unwrap();
        let at = spec.find("\n## Tasks\n").unwrap();
        let path = std::env::temp_dir().join(format!("ckc-agent-spec-{}.md", std::process::id()));
        let mut off = Vec::new();
        for (plant, ok) in [
            ("```\n## Fake\n```\n", true),
            ("````\n```\n## Fake\n````\n", true),
            ("~~~\n```\n## Fake\n~~~\n", true),
            ("```make``` builds it.\n## Fake\n", false),
            ("## Fake\n", false),
            ("   ```\n## Fake\n   ```\n", true),
            ("    ```\n## Fake\n    ```\n", false),
            ("```\n## Fake\n``` x\n```\n", true),
            ("~~~~\n## Fake\n~~~\n## Fake\n~~~~\n", true),
        ] {
            let text = format!("{}\n{plant}{}", &spec[..at], &spec[at..]);
            fs::write(&path, &text).unwrap();
            let lint = Command::new("sh")
                .arg(root.join(".agent/spec-lint"))
                .arg(&path)
                .output()
                .unwrap();
            let got = (lint.status.code(), shape(&text).is_ok());
            if got != (Some(if ok { 0 } else { 1 }), ok) {
                off.push(format!(
                    "{plant:?}: spec-lint rc {:?}, agent-spec ok {}",
                    got.0, got.1
                ));
            }
        }
        let _ = fs::remove_file(&path);
        assert!(off.is_empty(), "{}", off.join("\n"));
    }
}
