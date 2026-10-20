use super::oracle::{self, Mark, Row};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

pub const BASE_ARCHIVE_SHA: &str =
    "ca5d8a38bb4fbb72ba20937e38069ae6004cb566e5e7e32d3a013e3b6accca23";
pub const GIT_ENV: [(&str, &str); 9] = [
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_CONFIG_SYSTEM", "/dev/null"),
    ("GIT_DEFAULT_HASH", "sha1"),
    ("GIT_AUTHOR_NAME", "fixture"),
    ("GIT_AUTHOR_EMAIL", "fixture@localhost"),
    ("GIT_COMMITTER_NAME", "fixture"),
    ("GIT_COMMITTER_EMAIL", "fixture@localhost"),
    ("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+00:00"),
    ("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+00:00"),
];
const COVERAGE: &str = "# format: id<TAB>file<TAB>page<TAB>section<TAB>status\n# status: ace(<docid>) | restates(<id>) | uncovered(<class>: <one-clause reason>) | pending\n# uncovered classes: heading | process | external | aim | descriptive | notice | inexpressible\n";
static NEXT: AtomicU64 = AtomicU64::new(0);
pub type Files = BTreeMap<PathBuf, Vec<u8>>;

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
pub fn program() -> PathBuf {
    std::env::var_os("CKC_UR_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"))
}
pub fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}
pub fn write(path: &Path, text: impl AsRef<[u8]>) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}
pub fn sha(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn atom(s: &str) -> String {
    if s.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}
pub fn all_rows() -> Vec<Row> {
    oracle::rows(include_str!("all-vocabulary.tsv.in"))
}

pub struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new(label: &str) -> Self {
        let path = root().join(format!(
            "rust/target/ur-ui/{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!path.exists(), "scratch collision");
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0)
            .unwrap_or_else(|e| eprintln!("ur cleanup {}: {e}", self.0.display()));
    }
}

pub fn command(bin: &Path, cwd: &Path, args: &[&str]) -> Output {
    Command::new(bin)
        .args(args)
        .current_dir(cwd)
        .envs(GIT_ENV)
        .env("GIT_CEILING_DIRECTORIES", cwd)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("TMPDIR", cwd)
        .output()
        .unwrap_or_else(|e| panic!("{} {args:?}: {e}", bin.display()))
}
pub fn ok(output: &Output, label: &str) {
    assert!(
        output.status.success() && output.stderr.is_empty(),
        "{label}: rc={:?}, stdout={}, stderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
pub fn native(cwd: &Path, args: &[&str]) -> Output {
    command(&program(), cwd, args)
}

pub fn files(base: &Path) -> Files {
    fn walk(base: &Path, here: &Path, out: &mut Files) {
        if !here.is_dir() {
            return;
        }
        for entry in fs::read_dir(here).unwrap() {
            let path = entry.unwrap().path();
            if path.file_name().unwrap() == ".git" {
                continue;
            }
            assert!(!path.is_symlink(), "fixture symlink {}", path.display());
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                out.insert(
                    path.strip_prefix(base).unwrap().to_owned(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut out = Files::new();
    walk(base, base, &mut out);
    out
}
pub fn copy_tree(from: &Path, to: &Path, materialize: bool) {
    for (rel, bytes) in files(from) {
        let rel = if materialize && rel.to_string_lossy().ends_with(".pl.in") {
            rel.with_extension("")
        } else {
            rel
        };
        write(&to.join(rel), bytes);
    }
}
pub fn page<'a>(pages: &'a Files, path: &str) -> &'a str {
    std::str::from_utf8(
        pages
            .get(Path::new(path))
            .unwrap_or_else(|| panic!("page missing: {path}")),
    )
    .unwrap()
}
pub fn document<'a>(pages: &'a Files, id: &str) -> &'a str {
    page(pages, &format!("g/alpha/doc/{id}.html"))
}
pub fn block<'a>(page: &'a str, start: &str, end: &str) -> &'a str {
    page.split_once(start)
        .and_then(|(_, s)| s.split_once(end))
        .unwrap_or_else(|| panic!("missing {start:?} … {end:?}"))
        .0
}
pub fn ace_html(page: &str) -> &str {
    block(
        page,
        "<h3>Attempto Controlled English (ACE)</h3>\n<pre class=\"prose\">",
        "</pre>",
    )
}
pub fn reserved(page: &str) -> &str {
    let ace = page
        .split_once("<h3>Attempto Controlled English (ACE)</h3>\n")
        .unwrap()
        .1;
    ace.split_once("</pre>\n")
        .unwrap()
        .1
        .split_once("</section>")
        .unwrap()
        .0
}
pub fn script(page: &str) -> Option<&str> {
    page.split_once("<script>")
        .map(|(_, tail)| tail.split_once("</script>").expect("script end").0)
}
pub fn response(bytes: &[u8]) -> (&str, &str) {
    std::str::from_utf8(bytes)
        .unwrap()
        .split_once("\n\n")
        .expect("HTTP response boundary")
}

#[derive(Clone)]
pub struct Doc {
    pub id: String,
    pub ace: String,
    pub marks: Vec<Mark>,
    pub exclusions: Vec<usize>,
    pub pl: Option<String>,
}
impl Doc {
    pub fn new(id: &str, ace: &str) -> Self {
        Self {
            id: id.into(),
            ace: ace.into(),
            marks: Vec::new(),
            exclusions: Vec::new(),
            pl: None,
        }
    }
    pub fn canonical(&self, table: Option<&str>) -> String {
        let mut out = include_str!("canonical-header.pl.in")
            .replace("@DOC@", &self.id)
            .replace("@ATOM@", &atom(&self.id))
            .replace("@ACE_SHA@", &atom(&sha(&self.ace)))
            .replace(
                "@TABLE_SHA@",
                &table.map_or_else(|| "none".into(), |t| format!("sha256({})", atom(&sha(t)))),
            );
        for (i, line) in self
            .ace
            .split('\n')
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            out += &format!("% S{}: {line}\n", i + 1);
            out += if self.exclusions.contains(&(i + 1)) {
                "guideline_exclusion(alpha).\n"
            } else {
                "guideline_event(actual,e,wait).\n"
            };
        }
        out
    }
}

pub struct Fixture {
    pub scratch: Scratch,
    pub tree: PathBuf,
    pub docs: Vec<Doc>,
    pub rows: Vec<Row>,
}
impl Fixture {
    pub fn new(label: &str, rows: Option<Vec<Row>>, docs: Vec<Doc>) -> Self {
        let scratch = Scratch::new(label);
        let tree = scratch.0.join("tree");
        let g = tree.join("guidelines/alpha");
        write(&g.join("README.md"), "# Reading fixture\n");
        let table = rows.as_ref().map(|rows| oracle::table(rows));
        if let Some(t) = &table {
            write(&g.join("vocabulary.tsv"), t);
        }
        let mut coverage = COVERAGE.to_owned();
        let mut source = format!(
            "Reading fixture evidence\nRegion authority: identify the {} payloads below\n",
            docs.len()
        );
        let mut notes = "# format: docid<TAB>region<TAB>kept<TAB>dropped\n".to_owned();
        for (i, doc) in docs.iter().enumerate() {
            coverage += &format!(
                "r{}\tsource/evidence.txt\tp1\tReading fixture\tace({})\n",
                i + 1,
                doc.id
            );
            notes += &format!(
                "{}\tr{}\tword readings\tno material detail\n",
                doc.id,
                i + 1
            );
            write(&g.join(format!("ace/{}.ace", doc.id)), &doc.ace);
            write(
                &g.join(format!("pl/{}.pl", doc.id)),
                doc.pl
                    .clone()
                    .unwrap_or_else(|| doc.canonical(table.as_deref())),
            );
            let mut groups = BTreeMap::new();
            let mut payload = String::new();
            for mark in &doc.marks {
                if groups.contains_key(&mark.group) {
                    continue;
                }
                if !payload.is_empty() {
                    payload.push(' ');
                }
                let token = format!("linked-{}", mark.group);
                groups.insert(mark.group, (payload.chars().count(), token.clone()));
                payload += &token;
            }
            if payload.is_empty() {
                payload = "usually and ideally appear in the source.".into();
            }
            source += &format!("\n[r{} | p1 | Reading fixture]\n{payload}\n", i + 1);
            if !doc.marks.is_empty() {
                let chars: Vec<_> = doc.ace.chars().collect();
                let mut align = String::new();
                for m in &doc.marks {
                    align += &format!(
                        "{}\tace\t{}\t{}\n",
                        1000 - m.group,
                        m.start,
                        chars[m.start..m.end].iter().collect::<String>()
                    );
                }
                for (group, (start, token)) in groups {
                    align += &format!("{}\tsrc\t{start}\t{token}\n", 1000 - group);
                }
                write(&g.join(format!("align/{}.tsv", doc.id)), align);
            }
        }
        write(&g.join("coverage.tsv"), coverage);
        write(&g.join("source/evidence.txt"), source);
        write(&g.join("audit/projection-notes.tsv"), notes);
        let fx = Self {
            scratch,
            tree,
            docs,
            rows: rows.unwrap_or_default(),
        };
        fx.repin();
        fx
    }
    pub fn repin(&self) {
        let g = self.tree.join("guidelines/alpha");
        let out = native(
            &self.scratch.0,
            &["derive-review-manifest", g.to_str().unwrap()],
        );
        ok(&out, "fixture manifest");
        write(&g.join("audit/review-manifest.tsv"), out.stdout);
    }
    pub fn render(&self) -> Files {
        let outdir = self.scratch.0.join("pages");
        if outdir.exists() {
            fs::remove_dir_all(&outdir).unwrap();
        }
        let out = native(
            &self.scratch.0,
            &[
                "ui",
                "render",
                outdir.to_str().unwrap(),
                self.tree.to_str().unwrap(),
            ],
        );
        ok(&out, "fixture render");
        let result = files(&outdir);
        assert!(
            result.contains_key(Path::new("index.html")),
            "render inventory"
        );
        result
    }
    pub fn check(&self) -> Output {
        native(
            &self.scratch.0,
            &["ui", "check", self.tree.to_str().unwrap()],
        )
    }
    pub fn request(&self, path: &str) -> Output {
        native(
            &self.scratch.0,
            &["ui", "request", "GET", path, self.tree.to_str().unwrap()],
        )
    }
}

#[derive(Default)]
pub struct Checks {
    pub count: usize,
    pub failures: Vec<String>,
}
impl Checks {
    pub fn require(&mut self, yes: bool, label: impl Into<String>) {
        self.count += 1;
        if !yes {
            self.failures.push(label.into());
        }
    }
    pub fn equal(&mut self, actual: &str, expected: &str, label: &str) {
        self.count += 1;
        if actual != expected {
            let at = actual
                .bytes()
                .zip(expected.bytes())
                .position(|(a, b)| a != b)
                .unwrap_or(actual.len().min(expected.len()));
            self.failures.push(format!(
                "{label}: offset {at}; expected {:?}, actual {:?} ({} vs {} bytes)",
                String::from_utf8_lossy(&expected.as_bytes()[at..expected.len().min(at + 150)]),
                String::from_utf8_lossy(&actual.as_bytes()[at..actual.len().min(at + 150)]),
                expected.len(),
                actual.len()
            ));
        }
    }
    pub fn oracle(&mut self, pages: &Files, doc: &Doc, rows: &[Row], label: &str) {
        let expected = oracle::render(&doc.ace, &doc.marks, rows, &doc.exclusions);
        let actual = document(pages, &doc.id);
        self.equal(
            ace_html(actual),
            &expected.ace_html,
            &format!("{label}/{} ACE", doc.id),
        );
        self.equal(
            reserved(actual),
            &expected.reserved,
            &format!("{label}/{} reserved", doc.id),
        );
        self.require(
            actual.matches("class=\"rw\"").count() == expected.buttons.len(),
            format!("{label}/{} button count", doc.id),
        );
        let spans = if expected.lemmas.is_empty() {
            0
        } else {
            expected.lemmas.len() + 1
        };
        self.require(
            reserved(actual).matches("<span data-w=").count() == spans,
            format!("{label}/{} distinct lemma spans plus hint", doc.id),
        );
    }
    pub fn finish(self, name: &str) {
        eprintln!(
            "UR {name}: {} checks, {} failures",
            self.count,
            self.failures.len()
        );
        assert!(
            self.failures.is_empty(),
            "{name}:\n{}",
            self.failures.join("\n")
        );
    }
}
