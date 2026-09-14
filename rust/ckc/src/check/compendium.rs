use super::common::*;
use super::text::{decimal, fold, space, strip};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
const CLASSES: [&str; 3] = ["federal", "society", "other"];
const CPGS: [&str; 3] = ["yes", "unverified", "no"];
fn dollar(s: &str) -> &str {
    s.strip_suffix('\n').unwrap_or(s)
}
fn balanced(s: &str, prefix: &str) -> bool {
    let Some(inner) = dollar(s)
        .strip_prefix(prefix)
        .and_then(|s| s.strip_prefix('('))
        .and_then(|s| s.strip_suffix(')'))
    else {
        return false;
    };
    let mut open = false;
    for c in inner.chars() {
        match c {
            '(' if !open => open = true,
            ')' if open => open = false,
            '(' | ')' => return false,
            _ => (),
        }
    }
    !open
}
fn access(s: &str) -> bool {
    let s = dollar(s);
    if ["open", "unverified"].contains(&s) {
        return true;
    }
    ["paywalled(", "login("].iter().any(|p| {
        s.strip_prefix(p)
            .and_then(|s| s.strip_suffix(')'))
            .is_some_and(|s| !s.is_empty() && !s.contains(')'))
    })
}
fn status(s: &str) -> bool {
    ["unqueued", "queued", "in-progress", "done"].contains(&dollar(s))
        || ["provisional", "blocked", "excluded"]
            .iter()
            .any(|p| balanced(s, p))
}
fn four(cs: &[char]) -> Option<i32> {
    if cs.len() != 4 {
        return None;
    }
    cs.iter()
        .try_fold(0, |n, c| decimal(*c).map(|d| n * 10 + d as i32))
}
fn year(s: &str) -> Option<i32> {
    let s = s.trim_end_matches(space).strip_suffix(')')?;
    let (_, part) = s.rsplit_once('(')?;
    let chars: Vec<char> = part.chars().collect();
    if chars.len() < 4 {
        return None;
    }
    let year = four(&chars[..4])?;
    let rest: String = chars[4..].iter().collect();
    if rest.is_empty() {
        return Some(year);
    }
    let rest = rest
        .strip_prefix(';')?
        .trim_start_matches(space)
        .strip_prefix("pub")?
        .trim_start_matches(space);
    four(&rest.chars().collect::<Vec<_>>())?;
    Some(year)
}
fn swept(s: &str) -> bool {
    if ["pending", "-"].contains(&dollar(s)) || balanced(s, "blocked") {
        return true;
    }
    let c: Vec<char> = dollar(s).chars().collect();
    c.len() >= 12
        && c[4] == '-'
        && c[7] == '-'
        && c[10] == ' '
        && c[..4]
            .iter()
            .chain(&c[5..7])
            .chain(&c[8..10])
            .all(|c| decimal(*c).is_some())
        && !c[11..].contains(&'\n')
}
fn read_compendium(path: &str) -> Result<String> {
    let p = Path::new(path);
    if p.is_symlink() {
        return Err(fail("compendium", format!("is a symlink: {path}")));
    }
    if !p.is_file() {
        return Err(fail("compendium", format!("missing: {path}")));
    }
    // Path.read_text uses universal newline conversion.
    Ok(text(p, "compendium")?
        .replace("\r\n", "\n")
        .replace('\r', "\n"))
}
pub(super) fn check() -> Result {
    let md = read_compendium(".agent/compendium.md")?;
    let tsv = read_compendium(".agent/compendium.tsv")?;
    let Some((_, section)) = md.split_once("\n## Organizations\n") else {
        return Err(violation(
            "compendium-org-table",
            "missing `## Organizations` section in .agent/compendium.md",
        ));
    };
    let section = section.split("\n## Guidelines\n").next().unwrap_or("");
    let header = [
        "org",
        "abbrev",
        "class",
        "CPGs",
        "index URL",
        "enum sources",
        "swept",
    ];
    let mut seen_header = false;
    let mut rows = vec![];
    for line in section.split('\n').map(strip) {
        if !(line.starts_with('|') && line.ends_with('|')) {
            continue;
        }
        let inner = line
            .strip_prefix('|')
            .unwrap_or("")
            .strip_suffix('|')
            .unwrap_or("");
        let cells: Vec<&str> = inner.split('|').map(strip).collect();
        if !seen_header {
            if cells != header {
                return Err(violation(
                    "compendium-org-table",
                    "organization table header mismatch",
                ));
            }
            seen_header = true;
        } else if !cells
            .iter()
            .flat_map(|s| s.chars())
            .all(|c| c == '-' || c == ' ')
        {
            if cells.len() != 7 {
                return Err(violation(
                    "compendium-org-table",
                    format!("organization row without 7 cells: {line}"),
                ));
            }
            rows.push(cells);
        }
    }
    if !seen_header {
        return Err(violation(
            "compendium-org-table",
            "organization table absent - no header row found",
        ));
    }
    if rows.is_empty() {
        return Err(violation(
            "compendium-org-table",
            "organization table holds no rows",
        ));
    }
    let org_count = rows.len();
    let mut classes = BTreeMap::new();
    let mut org_remaining = 0;
    let mut keys = vec![];
    for r in rows {
        let (org, class, cpgs, sweep) = (r[0], r[2], r[3], r[6]);
        if org.is_empty() {
            return Err(violation(
                "compendium-org",
                "empty org cell in organization row",
            ));
        }
        if class.is_empty() {
            return Err(violation(
                "compendium-org",
                format!("empty class cell for: {org}"),
            ));
        }
        if cpgs.is_empty() {
            return Err(violation(
                "compendium-org",
                format!("empty CPGs cell for: {org}"),
            ));
        }
        let Some(ci) = CLASSES.iter().position(|s| *s == class) else {
            return Err(violation(
                "compendium-org",
                format!("bad class `{class}` for: {org}"),
            ));
        };
        let Some(pi) = CPGS.iter().position(|s| *s == cpgs) else {
            return Err(violation(
                "compendium-org",
                format!("bad CPGs `{cpgs}` for: {org}"),
            ));
        };
        if classes.contains_key(org) {
            return Err(violation(
                "compendium-org",
                format!("duplicate organization row: {org}"),
            ));
        }
        if !swept(sweep) {
            return Err(violation(
                "compendium-org",
                format!("bad swept `{sweep}` for: {org}"),
            ));
        }
        classes.insert(org, ci);
        keys.push(((ci, pi, fold(org)), org));
        if cpgs != "no" && ["pending", "-"].contains(&sweep) {
            org_remaining += 1;
        }
    }
    for p in keys.windows(2) {
        if p[1].0 < p[0].0 {
            return Err(violation(
                "compendium-org-order",
                format!(
                    "organization order violates class -> CPGs -> alpha at: {}",
                    p[1].1
                ),
            ));
        }
    }
    let mut tsv_lines = tsv.split('\n');
    if tsv_lines.next() != Some("org\ttitle (year)\tURL\taccess\tstatus\tnotes") {
        return Err(violation(
            "compendium-tsv",
            "first line of .agent/compendium.tsv is not the 6-column header",
        ));
    }
    let mut rows = vec![];
    for line in tsv_lines.filter(|s| !s.is_empty()) {
        let cells: Vec<&str> = line.split('\t').collect();
        if cells.len() != 6 {
            return Err(violation(
                "compendium-tsv",
                format!("row without 6 cells starting: {}", cells[0]),
            ));
        }
        rows.push(cells);
    }
    let row_count = rows.len();
    let (mut active, mut remaining, mut provisional) = (0, 0, 0);
    let (mut seen, mut urls, mut keys) = (BTreeSet::new(), BTreeMap::new(), vec![]);
    for r in rows {
        let (org, title, url, a, s, notes) = (r[0], r[1], r[2], r[3], r[4], r[5]);
        if !access(a) {
            return Err(violation(
                "compendium-row",
                format!("bad access `{a}` for: {title}"),
            ));
        }
        if !status(s) {
            return Err(violation(
                "compendium-row",
                format!("bad status `{s}` for: {title}"),
            ));
        }
        let Some(y) = year(title) else {
            return Err(violation(
                "compendium-row",
                format!("title lacks terminal (year): {title}"),
            ));
        };
        if org.is_empty() {
            return Err(violation(
                "compendium-row",
                format!("empty org cell for: {title}"),
            ));
        }
        if title.is_empty() {
            return Err(violation("compendium-row", "empty title cell"));
        }
        if url.is_empty() {
            return Err(violation(
                "compendium-row",
                format!("empty URL cell for: {title}"),
            ));
        }
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            return Err(violation(
                "compendium-row",
                format!("URL lacks http(s) scheme: {url}"),
            ));
        }
        if ["guestAccessKey", "accessKey=", "token="]
            .iter()
            .any(|s| url.contains(s))
        {
            return Err(violation(
                "compendium-row",
                format!("capability-token URL: {url}"),
            ));
        }
        if ["queued", "in-progress"].contains(&s) {
            active += 1;
        }
        let prov = s.starts_with("provisional(");
        let excluded = s.starts_with("excluded(");
        if a == "unverified" && !(prov || excluded) {
            return Err(violation(
                "compendium-row",
                format!("unverified access must be provisional(...) or excluded(...): {title}"),
            ));
        }
        if s == "unqueued" && notes.contains("unresolved") {
            return Err(violation(
                "compendium-row",
                format!("unresolved row must not be unqueued: {title}"),
            ));
        }
        if s != "done" && !s.starts_with("blocked(") && !excluded {
            remaining += 1;
        }
        if prov {
            provisional += 1;
        }
        let folded = fold(title);
        if !seen.insert(format!("{folded}\t{url}")) {
            return Err(violation(
                "compendium-dup",
                format!("duplicate guideline row: {title}"),
            ));
        }
        if urls.get(url).is_some_and(|t| t != &folded) {
            return Err(violation(
                "compendium-dup",
                format!("URL shared by two rows: {url}"),
            ));
        }
        urls.insert(url, folded.clone());
        let first = strip(org.split('+').next().unwrap_or(""));
        let class = classes.get(first).copied().unwrap_or(2);
        keys.push((
            (usize::from(a != "open"), class, fold(org), -y, folded),
            title,
        ));
    }
    if active > 1 {
        return Err(violation(
            "compendium-active",
            format!("{active} rows queued|in-progress (max 1)"),
        ));
    }
    for p in keys.windows(2) {
        if p[1].0 < p[0].0 {
            return Err(violation(
                "compendium-row-order",
                format!(
                    "guideline order violates access -> class -> org -> year desc -> title at: {}",
                    p[1].1
                ),
            ));
        }
    }
    println!(
        "goal: compendium ok {org_count} organizations {row_count} rows; terminal remaining: orgs={org_remaining} rows={remaining} provisional={provisional}"
    );
    Ok(())
}
