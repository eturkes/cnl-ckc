use super::chrome::{self, COMMIT_BASE, SCOPE};
use super::common::*;
use super::corpus::Corpus;
use super::intake::{self, View};
use std::collections::BTreeMap;
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
            let body =
                ckc_kernel::contract::ui_render_document(g, d, prev, next, &view.corpus.token);
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
fn invariant(page: &str) -> Result<Option<&'static str>> {
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
    if page.matches("href=\"http").count() != page.matches(&format!("href=\"{COMMIT_BASE}")).count()
    {
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
pub(super) fn check(root: &Path) -> Result<()> {
    let corpus = Corpus::worktree(root);
    let first = Scratch::new()?;
    let second = Scratch::new()?;
    let one = tree(&corpus, &intake::load(&corpus, b"")?)?;
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
            && let Some(problem) = invariant(&text(bytes))?
        {
            return Err(format!("ui: page invariant failed: {rel} {problem}"));
        }
    }
    one.meter();
    println!("ui: check ok");
    Ok(())
}
