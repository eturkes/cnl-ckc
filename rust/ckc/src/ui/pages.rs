use super::chrome::{self, COMMIT_BASE, SCOPE};
use super::common::*;
use super::corpus::Corpus;
use super::intake::{self, View};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

struct Tree {
    pages: BTreeMap<String, Vec<u8>>,
    assets: BTreeMap<String, PathBuf>,
    meters: Vec<String>,
    guidelines: usize,
}
impl Tree {
    fn write(&self, dest: &Path) -> Result<()> {
        for (rel, bytes) in &self.pages {
            let path = dest.join(rel);
            fs::create_dir_all(path.parent().ok_or("ui: render: missing parent")?)
                .and_then(|()| fs::write(&path, bytes))
                .map_err(|e| format!("ui: render: {}: {e}", path.display()))?;
        }
        for (rel, source) in &self.assets {
            let path = dest.join(rel);
            fs::create_dir_all(path.parent().ok_or("ui: render: missing parent")?)
                .and_then(|()| fs::copy(source, &path).map(|_| ()))
                .map_err(|e| format!("ui: render: {}: {e}", path.display()))?;
        }
        Ok(())
    }
    fn meter(&self) {
        for line in &self.meters {
            println!("{line}");
        }
        println!(
            "ui: ok {} guidelines {} pages",
            self.guidelines,
            self.pages.len()
        );
    }
}
fn hrefs(page: &str) -> Vec<&str> {
    page.split("<a ")
        .skip(1)
        .filter_map(|part| {
            part.split('>')
                .next()?
                .split_once("href=\"")?
                .1
                .split('"')
                .next()
        })
        .collect()
}
fn resolve_href(page: &str, href: &str) -> Option<String> {
    if href.starts_with('#') || href.starts_with("https://") {
        return None;
    }
    let mut parts = page.split('/').collect::<Vec<_>>();
    parts.pop();
    for part in href.split('#').next().unwrap_or("").split('/') {
        if part == ".." {
            if parts.pop().is_none() {
                return Some(String::new());
            }
        } else if part != "." {
            parts.push(part);
        }
    }
    Some(parts.join("/"))
}
fn tree(corpus: &Corpus, view: &View) -> Result<Tree> {
    let mut pages = BTreeMap::new();
    let mut order = vec!["index.html".to_owned()];
    pages.insert(
        "index.html".to_owned(),
        ckc_kernel::contract::ui_render_index(&view.corpus),
    );
    let mut assets = BTreeMap::new();
    let mut meters = Vec::new();
    for (gi, g) in view.corpus.guidelines.iter().enumerate() {
        let gid = text(&g.gid);
        for asset in &g.source_names {
            let name = text(asset);
            assets.insert(
                format!("g/{gid}/source/{name}"),
                corpus
                    .root
                    .join("guidelines")
                    .join(&gid)
                    .join("source")
                    .join(&name),
            );
        }
        let index = format!("g/{gid}/index.html");
        order.push(index.clone());
        pages.insert(index, ckc_kernel::contract::ui_render_guideline(g));
        for (di, d) in g.documents.iter().enumerate() {
            if let Some(error) = &view.errors[gi][di] {
                return Err(error.clone());
            }
            let path = format!("g/{gid}/doc/{}.html", text(&d.bundle.docid));
            let prev = di
                .checked_sub(1)
                .map(|i| g.documents[i].bundle.docid.as_slice())
                .unwrap_or(b"");
            let next = g
                .documents
                .get(di + 1)
                .map(|d| d.bundle.docid.as_slice())
                .unwrap_or(b"");
            let body = ckc_kernel::contract::ui_render_document(
                g,
                d,
                prev,
                next,
                &view.corpus.token,
                corpus.commit.as_bytes(),
            );
            order.push(path.clone());
            pages.insert(path, body);
        }
        let path = format!("g/{gid}/records.html");
        order.push(path.clone());
        pages.insert(path, ckc_kernel::contract::ui_render_records(g));
        meters.push(format!(
            "ui: {gid} docs={} regions={} pages={}",
            g.documents.len(),
            g.coverage.rows.len(),
            g.documents.len() + 2
        ));
    }
    for path in order {
        for href in hrefs(&text(&pages[&path])) {
            if let Some(resolved) = resolve_href(&path, href)
                && !pages.contains_key(&resolved)
                && !assets.contains_key(&resolved)
            {
                return Err(format!("ui: viewmodel: dangling href {path} {href}"));
            }
        }
    }
    Ok(Tree {
        pages,
        assets,
        meters,
        guidelines: view.corpus.guidelines.len(),
    })
}
pub(super) fn destination_blocked(path: &Path) -> bool {
    if path.is_symlink() {
        return true;
    }
    if path.is_dir() {
        return fs::read_dir(path).map_or(true, |mut it| it.next().is_some());
    }
    path.exists()
}
pub(super) fn render(root: &Path, dest: &Path) -> Result<()> {
    let corpus = Corpus::committed(root)?;
    let view = intake::load(&corpus, b"")?;
    let pages = tree(&corpus, &view)?;
    pages.write(dest)?;
    pages.meter();
    Ok(())
}
fn files(base: &Path, here: &Path, out: &mut BTreeMap<String, Vec<u8>>) -> Result<()> {
    for path in entries(here)? {
        if path.is_dir() {
            files(base, &path, out)?;
        } else {
            let rel = path
                .strip_prefix(base)
                .map_err(|e| format!("ui: render: {e}"))?
                .to_string_lossy()
                .into_owned();
            let bytes =
                fs::read(&path).map_err(|e| format!("ui: render: {}: {e}", path.display()))?;
            out.insert(rel, bytes);
        }
    }
    Ok(())
}
fn strip_between(s: &str, open: &str, close: &str) -> String {
    let mut parts = s.split(open);
    let mut out = parts.next().unwrap_or("").to_owned();
    for part in parts {
        if let Some((_, tail)) = part.split_once(close) {
            out.push_str(tail);
        }
    }
    out
}
fn visible(s: &str) -> String {
    let s = strip_between(s, "<style>", "</style>");
    let s = strip_between(&s, "<script>", "</script>");
    let s = strip_between(&s, "<pre", "</pre>");
    let s = strip_between(&s, "<!--", "-->");
    let mut parts = s.split('<');
    let mut out = vec![parts.next().unwrap_or("")];
    for part in parts {
        if let Some((_, tail)) = part.split_once('>') {
            out.push(tail);
        }
    }
    let joined = out.join(" ");
    let mut decoded = String::new();
    let mut rest = joined.as_str();
    while let Some((before, tail)) = rest.split_once('&') {
        decoded.push_str(before);
        if let Some((entity, after)) = tail.split_once(';') {
            let value = match entity {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "#x27" | "#39" => Some('\''),
                _ => None,
            };
            if let Some(value) = value {
                decoded.push(value);
                rest = after;
                continue;
            }
        }
        decoded.push('&');
        rest = tail;
    }
    decoded.push_str(rest);
    decoded
}
fn word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
fn has_functor(s: &str) -> bool {
    ["ace(", "restates(", "uncovered("].iter().any(|needle| {
        s.match_indices(needle)
            .any(|(i, _)| s[..i].chars().next_back().is_none_or(|c| !word(c)))
    })
}
fn visible_violation(s: &str) -> Option<&'static str> {
    let lower = s.to_lowercase();
    if lower.contains("sha256") {
        return Some("copy-sha256");
    }
    if lower.contains("bundle differs") {
        return Some("copy-bundle-differs");
    }
    if s.split(|c: char| !c.is_ascii_hexdigit())
        .any(|run| run.len() >= 16 && run.bytes().any(|b| b.is_ascii_digit()))
    {
        return Some("copy-hex");
    }
    if has_functor(s) {
        return Some("copy-functor");
    }
    if s.split(|c: char| !word(c)).any(|run| {
        (7..=15).contains(&run.len())
            && run
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            && run.bytes().any(|b| b.is_ascii_digit())
            && run.bytes().any(|b| (b'a'..=b'f').contains(&b))
    }) {
        return Some("copy-short-hex");
    }
    None
}
// Corpus text and quoted attribute values are data, not resource attributes.
fn resource_tag(tag: &str) -> bool {
    let tag = tag.trim_start();
    let end = tag
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '-')
        .unwrap_or(tag.len());
    let (name, mut rest) = tag.split_at(end);
    if ["img", "link", "iframe", "object"]
        .iter()
        .any(|n| name.eq_ignore_ascii_case(n))
    {
        return true;
    }
    while !rest.trim_start().is_empty() {
        rest = rest.trim_start();
        let end = rest
            .find(|c: char| c.is_ascii_whitespace() || c == '=')
            .unwrap_or(rest.len());
        if end == 0 || rest.starts_with('/') {
            break;
        }
        let (name, tail) = rest.split_at(end);
        if name.eq_ignore_ascii_case("src") {
            return true;
        }
        rest = tail.trim_start();
        if let Some(value) = rest.strip_prefix('=') {
            rest = value.trim_start();
            if let Some(quote @ ('\'' | '"')) = rest.chars().next() {
                let tail = &rest[1..];
                rest = tail.split_once(quote).map(|(_, rest)| rest).unwrap_or("");
            } else {
                rest = rest
                    .find(char::is_whitespace)
                    .map(|i| &rest[i..])
                    .unwrap_or("");
            }
        }
    }
    false
}
fn resource_reference(page: &str) -> bool {
    let page = strip_between(page, "<style>", "</style>");
    let page = strip_between(&page, "<script>", "</script>");
    let page = strip_between(&page, "<!--", "-->");
    page.split('<')
        .skip(1)
        .filter_map(|p| p.split_once('>').map(|(tag, _)| tag))
        .any(resource_tag)
}
// contract sup4 D6: a source link = the attribute-escaped url of a `supersession` or
// `superseded` row of the page's own guideline table (the intake already checked its grammar).
fn source_hrefs(table: Option<&[u8]>) -> BTreeSet<String> {
    let Some(table) = table else {
        return BTreeSet::new();
    };
    text(table)
        .lines()
        .filter_map(|line| {
            let mut f = line.split('\t');
            let (kind, _, value) = (f.next()?, f.next()?, f.next()?);
            (f.next().is_none() && matches!(kind, "supersession" | "superseded"))
                .then(|| value.get(9..)?.split(' ').next())
                .flatten()
        })
        .map(|url| {
            url.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;")
        })
        .filter(|url| !url.starts_with(COMMIT_BASE))
        .collect()
}
// Every off-site anchor href = a commit link or one of the page guideline's source links.
// Anchors, not substrings: text escaping keeps `"`, so a citation can spell `href="http`.
fn external_ok(page: &str, sources: &BTreeSet<String>) -> bool {
    hrefs(page)
        .into_iter()
        .filter(|href| href.starts_with("http"))
        .all(|href| href.starts_with(COMMIT_BASE) || sources.contains(href))
}
fn invariant(page: &str, sources: &BTreeSet<String>) -> Result<Option<&'static str>> {
    for (needle, count, label) in [
        ("<!doctype html>", 1, "doctype"),
        ("<html lang=\"en\">", 1, "html-lang"),
        ("<h1", 1, "h1"),
        ("<main id=\"main\">", 1, "main"),
        (
            "<a class=\"skip\" href=\"#main\">Skip to content</a>",
            1,
            "skip",
        ),
        ("<style>", 1, "style"),
    ] {
        if page.matches(needle).count() != count {
            return Ok(Some(label));
        }
    }
    if !page.contains("<nav") {
        return Ok(Some("nav"));
    }
    let canonical = chrome::canonical()?;
    let scripts = page.matches(&canonical.script).count();
    if page.matches("<script").count() != scripts || scripts > 1 {
        return Ok(Some("script"));
    }
    for (needle, label) in [("tabindex", "tabindex"), ("style=", "inline-style")] {
        if page.contains(needle) {
            return Ok(Some(label));
        }
    }
    if !external_ok(page, sources) {
        return Ok(Some("external-href"));
    }
    if resource_reference(page) {
        return Ok(Some("external-resource"));
    }
    if page.matches(SCOPE).count() != 1 {
        return Ok(Some("scope-line"));
    }
    if !page.contains(&format!("<style>{}</style>", canonical.css)) {
        return Ok(Some("palette"));
    }
    Ok(visible_violation(&visible(page)))
}
// The source hrefs of the guideline a page `g/<gid>/…` belongs to; none elsewhere.
fn sources(view: &View, rel: &str) -> BTreeSet<String> {
    let gid = rel.strip_prefix("g/").and_then(|r| r.split('/').next());
    view.corpus
        .guidelines
        .iter()
        .find(|g| Some(text(&g.gid).as_str()) == gid)
        .map(|g| source_hrefs(g.vocabulary.as_deref()))
        .unwrap_or_default()
}
pub(super) fn check(root: &Path) -> Result<()> {
    let corpus = Corpus::worktree(root);
    let first = Scratch::new()?;
    let second = Scratch::new()?;
    let view = intake::load(&corpus, b"")?;
    let one = tree(&corpus, &view)?;
    one.write(&first.0)?;
    let two = tree(&corpus, &intake::load(&corpus, b"")?)?;
    two.write(&second.0)?;
    let mut a = BTreeMap::new();
    let mut b = BTreeMap::new();
    files(&first.0, &first.0, &mut a)?;
    files(&second.0, &second.0, &mut b)?;
    if !a.keys().eq(b.keys()) {
        return Err("ui: render not byte-stable: tree".into());
    }
    for (rel, bytes) in &a {
        if b.get(rel) != Some(bytes) {
            return Err(format!("ui: render not byte-stable: {rel}"));
        }
        if !one.assets.contains_key(rel)
            && let Some(problem) = invariant(&text(bytes), &sources(&view, rel))?
        {
            return Err(format!("ui: page invariant failed: {rel} {problem}"));
        }
    }
    one.meter();
    println!("ui: check ok");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ckc_kernel::{
        EBundle, ECoverage, ECoverageRow, EDocument, EEvidence, EGuideline, ESrc, EStatus,
    };

    const TABLE: &[u8] = b"# format: kind\tlemma\tvalue\nsupersession\tdoc\t20221229 https://a.test/?x=1&y='z' Act, section 1\nsuperseded\tR1\t20221229 https://github.com/eturkes/cnl-ckc/commit/abc Note\nunit\tday\tday\n";

    #[test]
    fn source_links_pass_and_foreign_links_fire() {
        let sources = source_hrefs(Some(TABLE));
        assert_eq!(
            sources.iter().collect::<Vec<_>>(),
            ["https://a.test/?x=1&amp;y=&#x27;z&#x27;"]
        );
        let cited = "<a href=\"https://a.test/?x=1&amp;y=&#x27;z&#x27;\">Act, section 1</a>";
        let commit = "<a href=\"https://github.com/eturkes/cnl-ckc/commit/abc\">v</a>";
        let foreign = "<a href=\"https://b.test/\">Elsewhere</a>";
        let quoted = "<p>Act quoting href=\"https://b.test/\" as text</p>";
        assert!(external_ok(cited, &sources));
        assert!(!external_ok(&format!("{cited}{commit}"), &BTreeSet::new()));
        assert!(external_ok(commit, &BTreeSet::new()));
        assert!(external_ok(quoted, &BTreeSet::new()));
        assert!(!external_ok(foreign, &sources));
        assert!(!external_ok(&format!("{cited}{foreign}"), &sources));
        assert!(!external_ok(cited, &source_hrefs(None)));
    }

    fn href_fired(verdict: Result<Option<&'static str>>) -> bool {
        matches!(verdict, Ok(Some(label)) if label.ends_with("-href"))
    }

    // The whole page invariant over a kernel-rendered page whose citation spells an href.
    #[test]
    fn rendered_source_link_passes_the_page_invariant() {
        let table = b"# format: kind\tlemma\tvalue\nsupersession\tx\t20221229 https://a.test/act Act quoting href=\"https://b.test/\" as text\n";
        let g = EGuideline {
            gid: b"x".to_vec(),
            readme: None,
            coverage: ECoverage {
                rows: vec![ECoverageRow {
                    id: b"x".to_vec(),
                    file: b"source/x.txt".to_vec(),
                    status: EStatus::Ace(b"x".to_vec()),
                    line: b"x\tsource/x.txt\tp1\tx\tace(x)\n".to_vec(),
                }],
                files: vec![b"source/x.txt".to_vec()],
                evidence: vec![EEvidence {
                    census: b"1".to_vec(),
                    locators: vec![b"x".to_vec()],
                    payloads: vec![(b"x".to_vec(), vec![b"x".to_vec()])],
                    ordinal: Vec::new(),
                }],
            },
            documents: vec![EDocument {
                bundle: EBundle {
                    docid: b"x".to_vec(),
                    ace: Vec::new(),
                    cov: Vec::new(),
                    pay: Vec::new(),
                    cl: Vec::new(),
                    review: Vec::new(),
                },
                ace: b"x".to_vec(),
                pl: Vec::new(),
                alignment: None,
            }],
            ledger: ESrc::Missing,
            ledger_digest: b"absent".to_vec(),
            source_names: Vec::new(),
            vocabulary: Some(table.to_vec()),
        };
        let page = text(&ckc_kernel::contract::ui_render_document(
            &g,
            &g.documents[0],
            b"",
            b"",
            b"",
            b"",
        ));
        assert!(page.contains(
            "<a href=\"https://a.test/act\">Act quoting href=\"https://b.test/\" as text</a>"
        ));
        let sources = source_hrefs(g.vocabulary.as_deref());
        assert_eq!(invariant(&page, &sources).unwrap(), None);
        let planted = page.replacen("</main>", "<a href=\"https://b.test/\">x</a></main>", 1);
        assert!(href_fired(invariant(&planted, &sources)));
        assert!(href_fired(invariant(&page, &BTreeSet::new())));
    }
}
