// M6 P5/R86: the released binary replays data-tier fixtures where pinned
// SWI-Prolog is available. Cargo tests cover parsing and edits without it.
use crate::certify_cli::name_ok;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

type Result<T> = std::result::Result<T, String>;
const HEADER: &str = "id\tkind\ttarget\top\targ1\targ2\texpected_rc\texpected_stderr";
static NEXT: AtomicUsize = AtomicUsize::new(0);

enum Edit {
    Replace(String, String),
    Swap(usize, usize),
    Truncate(String),
}

struct Case {
    id: String,
    query: bool,
    guideline: String,
    artifact: String,
    target: PathBuf,
    edit: Edit,
    rc: i32,
    stderr: Vec<u8>,
}

fn io<T>(result: std::io::Result<T>) -> Result<T> {
    result.map_err(|error| error.to_string())
}

fn unescape(field: &str) -> Result<String> {
    let mut out = String::new();
    let mut chars = field.chars();
    while let Some(c) = chars.next() {
        out.push(if c == '\\' {
            match chars.next() {
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some('\\') => '\\',
                _ => return Err("invalid field escape".to_owned()),
            }
        } else {
            c
        });
    }
    Ok(out)
}

fn parse(table: &str) -> Result<Vec<Case>> {
    let mut lines = table.lines();
    if !table.ends_with('\n') || lines.next() != Some(HEADER) {
        return Err("table header or final newline".to_owned());
    }
    let mut cases = Vec::new();
    let mut ids = BTreeSet::new();
    let mut targets = BTreeSet::new();
    let mut counts = [0, 0];
    for (row, line) in lines.enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        let bad = |law| format!("row {}: {law}", row + 2);
        if fields.len() != 8 || !name_ok(fields[0]) || !ids.insert(fields[0]) {
            return Err(bad("fields or case id"));
        }
        let query = match fields[1] {
            "doc" => false,
            "query" => true,
            _ => return Err(bad("kind")),
        };
        let parts: Vec<_> = fields[2].split('/').collect();
        let expected = if query { 5 } else { 4 };
        if parts.len() != expected
            || parts[0] != "guidelines"
            || !name_ok(parts[1])
            || parts[expected - 2] != "pl"
            || (query && parts[2] != "queries")
        {
            return Err(bad("target"));
        }
        let artifact = parts[expected - 1]
            .strip_suffix(".pl")
            .filter(|id| name_ok(id))
            .ok_or_else(|| bad("target name"))?;
        let a = unescape(fields[4])?;
        let b = unescape(fields[5])?;
        let edit = match fields[3] {
            "replace-first" if !a.is_empty() => Edit::Replace(a, b),
            "swap-lines" => Edit::Swap(
                a.parse().map_err(|_| bad("first line index"))?,
                b.parse().map_err(|_| bad("second line index"))?,
            ),
            "truncate-from" if !a.is_empty() && b.is_empty() => Edit::Truncate(a),
            _ => return Err(bad("edit")),
        };
        let rc = fields[6].parse().map_err(|_| bad("exit code"))?;
        let stderr = unescape(fields[7])?.into_bytes();
        if rc != 1
            || !stderr.starts_with(format!("ckc: certify: {artifact}: ").as_bytes())
            || !stderr.ends_with(b"\n")
        {
            return Err(bad("rejection envelope"));
        }
        counts[usize::from(query)] += 1;
        targets.insert((query, fields[2]));
        cases.push(Case {
            id: fields[0].to_owned(),
            query,
            guideline: parts[1].to_owned(),
            artifact: artifact.to_owned(),
            target: PathBuf::from(fields[2]),
            edit,
            rc,
            stderr,
        });
    }
    if counts != [12, 4] || targets.len() != 2 {
        return Err("expected 12 document cases, 4 query cases, and 2 targets".to_owned());
    }
    Ok(cases)
}

fn mutate(source: &str, edit: &Edit) -> Result<String> {
    let changed = match edit {
        Edit::Replace(a, b) => {
            if !source.contains(a) {
                return Err("missing replacement anchor".to_owned());
            }
            source.replacen(a, b, 1)
        }
        Edit::Swap(a, b) => {
            let mut lines: Vec<_> = source.split_inclusive('\n').collect();
            if *a >= lines.len() || *b >= lines.len() {
                return Err("line index out of range".to_owned());
            }
            lines.swap(*a, *b);
            lines.concat()
        }
        Edit::Truncate(marker) => {
            source[..source.find(marker).ok_or("missing truncation marker")?].to_owned()
        }
    };
    if changed == source {
        return Err("mutation is a no-op".to_owned());
    }
    Ok(changed)
}

struct Fixture {
    root: PathBuf,
    pl: PathBuf,
    original: String,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

impl Fixture {
    fn new(repo: &Path, case: &Case) -> Result<Self> {
        let original = io(fs::read_to_string(repo.join(&case.target)))?;
        let root = std::env::temp_dir().join(format!(
            "ckc-certify-cases-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        io(fs::create_dir(&root))?;
        let fixture = Self {
            pl: root.join(&case.target),
            root,
            original,
        };
        let gpath = Path::new("guidelines").join(&case.guideline);
        let guideline = fixture.root.join(&gpath);
        io(fs::create_dir_all(guideline.join("ace")))?;
        io(fs::create_dir_all(fixture.pl.parent().ok_or("PL parent")?))?;
        let ace = gpath
            .join(if case.query { "queries" } else { "ace" })
            .join(format!("{}.ace", case.artifact));
        io(fs::copy(repo.join(&ace), fixture.root.join(&ace)))?;
        match fs::read(repo.join(gpath).join("lexicon.ulex")) {
            Ok(bytes) => io(fs::write(guideline.join("lexicon.ulex"), bytes))?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
        io(fs::write(&fixture.pl, &fixture.original))?;
        io(symlink(repo.join("vendor"), fixture.root.join("vendor")))?;
        let driver = "rust/ckc/prolog/drs_dump.pl";
        io(fs::create_dir_all(
            fixture.root.join(driver).parent().ok_or("driver parent")?,
        ))?;
        io(fs::copy(repo.join(driver), fixture.root.join(driver)))?;
        io(fs::create_dir(fixture.root.join("tmp")))?;
        Ok(fixture)
    }

    fn run(&self, binary: &Path, guideline: &str) -> Result<Output> {
        io(Command::new(binary)
            .args(["certify", guideline])
            .current_dir(&self.root)
            .env("TMPDIR", self.root.join("tmp"))
            .output())
    }
}

fn check(output: &Output, rc: i32, stdout: &[u8], stderr: &[u8]) -> Result<()> {
    if output.status.code() != Some(rc) {
        return Err(format!("exit {:?} != {rc}", output.status.code()));
    }
    if output.stdout != stdout {
        return Err(format!(
            "stdout differs ({} bytes != {} bytes)",
            output.stdout.len(),
            stdout.len()
        ));
    }
    if output.stderr != stderr {
        return Err(format!(
            "stderr {:?} != {:?}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(stderr)
        ));
    }
    Ok(())
}

fn replay(path: &str) -> Result<(usize, usize)> {
    let cases = parse(&io(fs::read_to_string(path))?)?;
    let repo = io(std::env::current_dir())?;
    let binary = io(std::env::current_exe())?;
    let mut fixtures = BTreeMap::new();
    for case in &cases {
        let key = (case.query, &case.target);
        if let std::collections::btree_map::Entry::Vacant(entry) = fixtures.entry(key) {
            let fixture = Fixture::new(&repo, case)?;
            let baseline = fixture.run(&binary, &case.guideline)?;
            let meter = format!(
                "ckc: certify ok {} {} documents {} queries\n",
                case.guideline,
                usize::from(!case.query),
                usize::from(case.query)
            );
            check(&baseline, 0, meter.as_bytes(), b"")
                .map_err(|why| format!("{} baseline: {why}", case.id))?;
            entry.insert(fixture);
        }
        let fixture = fixtures.get(&key).ok_or("fixture missing")?;
        let changed =
            mutate(&fixture.original, &case.edit).map_err(|why| format!("{}: {why}", case.id))?;
        io(fs::write(&fixture.pl, changed))?;
        let output = fixture.run(&binary, &case.guideline)?;
        check(&output, case.rc, b"", &case.stderr).map_err(|why| format!("{}: {why}", case.id))?;
        io(fs::write(&fixture.pl, &fixture.original))?;
    }
    Ok((cases.len(), fixtures.len()))
}

pub fn run(path: &str) -> ExitCode {
    match replay(path) {
        Ok((cases, baselines)) => {
            println!("ckc: certify cases ok {cases} mutants {baselines} baselines");
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("ckc: certify cases: {why}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> (PathBuf, String) {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let table = fs::read_to_string(repo.join("tests/certify/cases.tsv")).unwrap();
        (repo, table)
    }

    #[test]
    fn committed_table_parses_and_every_edit_changes_its_source() {
        let (repo, table) = inputs();
        let cases = parse(&table).unwrap();
        assert_eq!(cases.len(), 16);
        for case in cases {
            let source = fs::read_to_string(repo.join(case.target)).unwrap();
            assert_ne!(mutate(&source, &case.edit).unwrap(), source, "{}", case.id);
        }
    }

    #[test]
    fn malformed_table_and_no_op_edits_fail() {
        let (_, table) = inputs();
        assert!(parse(&table.replacen("doc-drop-clause", "doc-swap-ref", 1)).is_err());
        assert!(parse(&table.replacen("\t1\tckc:", "\t0\tckc:", 1)).is_err());
        assert!(unescape("\\q").is_err());
        assert!(mutate("a", &Edit::Replace("a".to_owned(), "a".to_owned())).is_err());
        assert!(mutate("a", &Edit::Swap(0, 1)).is_err());
    }
}
