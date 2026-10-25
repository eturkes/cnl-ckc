use std::collections::BTreeMap;
use std::path::Path;

pub const HEADER: &str = "# format: kind\tlemma\tvalue\n";
pub const NOTICE_OPEN: &str = "<section class=\"superseded\">\n";
pub const SECTION_OPEN: &str = "<section>\n<h2>Superseded passages</h2>\n";
pub const NOTICE_TAIL: &str = " superseded the guideline statements that this page represents. This page shows the original passage and its ACE representation for review.</p>";
pub const EXPLANATION: &str = "<p>Later law, regulation or guidance superseded all or part of each passage in this table.</p>";
pub const THEAD: &str = "<thead><tr><th>Passage</th><th>Document</th><th>Superseded by</th><th>Superseded on</th></tr></thead>";
pub const NEW_COPY: [&str; 7] = [
    "<section class=\"superseded\">",
    "<p>On ",
    "-",
    NOTICE_TAIL,
    "<h2>Superseded passages</h2>",
    EXPLANATION,
    THEAD,
];
pub const STALE: &str = "<section class=\"stale\">\n<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>\n</section>\n";

pub fn escape(raw: &str, attribute: bool) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' if attribute => out.push_str("&quot;"),
            '\'' if attribute => out.push_str("&#x27;"),
            _ => out.push(ch),
        }
    }
    out
}
fn link(url: &str, label: &str) -> String {
    format!(
        "<a href=\"{}\">{}</a>",
        escape(url, true),
        escape(label, false)
    )
}
fn segment(raw: &str) -> String {
    raw.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                char::from(b).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct Row {
    pub document: bool,
    pub key: String,
    pub date: String,
    pub url: String,
    pub citation: String,
}
impl Row {
    pub fn new(document: bool, key: &str, date: &str, url: &str, citation: &str) -> Self {
        Self {
            document,
            key: key.into(),
            date: date.into(),
            url: url.into(),
            citation: citation.into(),
        }
    }
    pub fn line(&self) -> String {
        format!(
            "{}\t{}\t{} {} {}\n",
            if self.document {
                "supersession"
            } else {
                "superseded"
            },
            self.key,
            self.date,
            self.url,
            self.citation
        )
    }
    pub fn iso(&self) -> String {
        format!(
            "{}-{}-{}",
            &self.date[..4],
            &self.date[4..6],
            &self.date[6..]
        )
    }
    fn source(&self) -> String {
        link(&self.url, &self.citation)
    }
}
// Admitted tables alone enter this oracle; malformed tables are refusal cases.
pub fn rows(table: &str) -> Vec<Row> {
    table
        .lines()
        .filter_map(|line| {
            let fields: Vec<_> = line.split('\t').collect();
            if !matches!(fields.first().copied(), Some("supersession" | "superseded")) {
                return None;
            }
            assert_eq!(fields.len(), 3, "oracle row envelope");
            let mut value = fields[2].splitn(3, ' ');
            Some(Row::new(
                fields[0] == "supersession",
                fields[1],
                value.next().unwrap(),
                value.next().unwrap(),
                value.next().unwrap(),
            ))
        })
        .collect()
}

pub fn table(rows: &[Row], readings: bool) -> String {
    let mut out = HEADER.to_owned();
    if readings {
        out.push_str("qualifier\treviewer\tmanner\n");
    }
    for row in rows {
        out.push_str(&row.line());
    }
    out
}
pub fn without_sup(table: &str) -> Option<String> {
    let kept: Vec<_> = table
        .lines()
        .filter(|line| !line.starts_with("supersession\t") && !line.starts_with("superseded\t"))
        .collect();
    (kept.len() > 1).then(|| kept.join("\n") + "\n")
}

pub struct Model {
    pub rows: Vec<Row>,
    pub docs: BTreeMap<String, (String, String)>,
}
impl Model {
    pub fn load(tree: &Path) -> Self {
        let guideline = tree.join("guidelines/alpha");
        let coverage = std::fs::read_to_string(guideline.join("coverage.tsv")).unwrap();
        let entries: Vec<_> = coverage
            .lines()
            .filter(|line| !line.starts_with('#'))
            .map(|line| {
                let f: Vec<_> = line.split('\t').collect();
                assert_eq!(f.len(), 5);
                f
            })
            .collect();
        let mut docs = BTreeMap::new();
        for file in std::fs::read_dir(guideline.join("ace")).unwrap() {
            let file = file.unwrap().path();
            let id = file.file_stem().unwrap().to_str().unwrap();
            let status = format!("ace({id})");
            let row = entries.iter().find(|f| f[4] == status);
            let (region, title) = row.map_or_else(
                || (String::new(), id.to_owned()),
                |f| {
                    let mut sections: Vec<String> = f[3]
                        .split('>')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned)
                        .collect();
                    if let Some(first) = sections.first().cloned() {
                        let words: Vec<_> = first.split(' ').collect();
                        if words.len() == 2
                            && words[1].bytes().all(|b| b.is_ascii_digit())
                            && !words[1].is_empty()
                        {
                            if words[0] == "Rec" {
                                sections[0] = format!("Recommendation {}", words[1]);
                            } else if words[0] == "Box" && sections.len() > 1 {
                                sections.remove(0);
                            }
                        }
                    }
                    let base = sections.join(" > ");
                    let title = if base.is_empty() {
                        id.to_owned()
                    } else if entries
                        .iter()
                        .filter(|other| other[4].starts_with("ace(") && other[3] == f[3])
                        .count()
                        <= 1
                    {
                        base
                    } else {
                        let page = f[2].trim().strip_prefix('p').unwrap_or(f[2].trim());
                        let last = f[0].rsplit('-').next().unwrap();
                        let digits =
                            |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
                        if digits(page) && digits(last) {
                            format!(
                                "{base}, page {page}, passage {}",
                                last.parse::<u64>().unwrap()
                            )
                        } else {
                            format!("{base} ({})", f[0])
                        }
                    };
                    (f[0].to_owned(), title)
                },
            );
            docs.insert(id.to_owned(), (region, title));
        }
        let table = std::fs::read_to_string(guideline.join("vocabulary.tsv")).unwrap_or_default();
        Self {
            rows: rows(&table),
            docs,
        }
    }
    pub fn notice(&self, doc: &str) -> String {
        self.rows
            .iter()
            .find(|r| r.document && r.key == doc)
            .map_or_else(String::new, |r| {
                format!(
                    "{NOTICE_OPEN}<p>On {}, {}{NOTICE_TAIL}\n</section>\n",
                    r.iso(),
                    r.source()
                )
            })
    }
    pub fn section(&self) -> String {
        if self.rows.is_empty() {
            return String::new();
        }
        let body = self
            .rows
            .iter()
            .map(|r| {
                let doc = self.docs.get(&r.key).filter(|_| r.document);
                let passage = if r.document {
                    doc.map_or("", |d| d.0.as_str())
                } else {
                    &r.key
                };
                let document = doc.map_or_else(String::new, |d| {
                    link(&format!("doc/{}.html", segment(&r.key)), &d.1)
                });
                format!(
                    "<tr><td>{}</td><td>{document}</td><td>{}</td><td>{}</td></tr>",
                    escape(passage, false),
                    r.source(),
                    r.iso()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "{SECTION_OPEN}{EXPLANATION}\n<table class=\"compact\">\n{THEAD}\n<tbody>{body}</tbody>\n</table>\n</section>\n"
        )
    }
    pub fn insertion(&self, path: &str) -> String {
        if path == "g/alpha/index.html" {
            self.section()
        } else if let Some(doc) = path
            .strip_prefix("g/alpha/doc/")
            .and_then(|s| s.strip_suffix(".html"))
        {
            self.notice(doc)
        } else {
            String::new()
        }
    }
}

pub fn section_end(page: &str, heading: &str) -> Option<usize> {
    let start = page.find(&format!("<section>\n<h2>{heading}</h2>\n"))?;
    page[start..]
        .find("</section>\n")
        .map(|n| start + n + "</section>\n".len())
}
pub fn insertion_offset(page: &str, path: &str) -> Option<usize> {
    if path == "g/alpha/index.html" {
        section_end(page, "Word meanings").or_else(|| section_end(page, "Status"))
    } else {
        let tally_end = "All decision records</a></p>\n";
        let start = page.find(tally_end)? + tally_end.len();
        Some(
            start
                + if page[start..].starts_with(STALE) {
                    STALE.len()
                } else {
                    0
                },
        )
    }
}
// Strip only an exact, unique insertion at its contractual anchor.
pub fn strip(page: &str, path: &str, insertion: &str) -> Result<String, String> {
    if insertion.is_empty() {
        return if page.contains(NOTICE_OPEN) || page.contains(SECTION_OPEN) {
            Err("unexpected supersession block".into())
        } else {
            Ok(page.into())
        };
    }
    let count = page.matches(insertion).count();
    if count != 1 {
        return Err(format!(
            "exact D3/D4 insertion count: expected 1, got {count}"
        ));
    }
    let start = page.find(insertion).unwrap();
    if insertion_offset(page, path) != Some(start) {
        return Err("D3/D4 insertion misplaced".into());
    }
    let after = &page[start + insertion.len()..];
    let valid_successor = if path == "g/alpha/index.html" {
        after.starts_with("<section>\n<h2>Documents</h2>\n")
    } else {
        [
            "<p class=\"hl-note\">",
            "<script>",
            "<section>\n<h3>Original passage</h3>\n",
        ]
        .iter()
        .any(|s| after.starts_with(s))
    };
    if !valid_successor {
        return Err("D3/D4 insertion successor differs".into());
    }
    Ok(format!("{}{}", &page[..start], after))
}
