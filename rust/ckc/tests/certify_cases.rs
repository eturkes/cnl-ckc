// M6 P5: data-tier mutation pins over the live certifier, including its
// upstream driver and shell envelope. Each fixture holds one artifact.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

const GUIDELINE: &str = "cdc-2022-opioid";
static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    root: PathBuf,
    pl: PathBuf,
    original: String,
    query: bool,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

impl Fixture {
    fn new(repo: &Path, query: bool, target: &str) -> Self {
        let target = Path::new(target);
        let parent = if query { "queries/pl" } else { "pl" };
        assert_eq!(target.parent(), Some(Path::new(parent)));
        assert_eq!(target.extension().and_then(|s| s.to_str()), Some("pl"));
        let id = target.file_stem().unwrap().to_str().unwrap();
        let source = repo.join("guidelines").join(GUIDELINE);
        let original = fs::read_to_string(source.join(target)).unwrap();
        let root = std::env::temp_dir().join(format!(
            "ckc-certify-cases-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let guideline = root.join("guidelines").join(GUIDELINE);
        let fixture = Self {
            pl: guideline.join(target),
            root,
            original,
            query,
        };
        fs::create_dir_all(guideline.join("ace")).unwrap();
        fs::create_dir_all(fixture.pl.parent().unwrap()).unwrap();
        let ace = Path::new(if query { "queries" } else { "ace" }).join(format!("{id}.ace"));
        fs::copy(source.join(&ace), guideline.join(ace)).unwrap();
        fs::copy(source.join("lexicon.ulex"), guideline.join("lexicon.ulex")).unwrap();
        fs::write(&fixture.pl, &fixture.original).unwrap();
        symlink(repo.join("vendor"), fixture.root.join("vendor")).unwrap();
        let driver = "rust/ckc/prolog/drs_dump.pl";
        fs::create_dir_all(fixture.root.join(driver).parent().unwrap()).unwrap();
        fs::copy(repo.join(driver), fixture.root.join(driver)).unwrap();
        fs::create_dir(fixture.root.join("tmp")).unwrap();
        fixture
    }

    fn run(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_ckc"))
            .args(["certify", GUIDELINE])
            .current_dir(&self.root)
            .env("TMPDIR", self.root.join("tmp"))
            .output()
            .unwrap()
    }

    fn baseline(&self) {
        let output = self.run();
        assert_eq!(output.status.code(), Some(0), "{:?}", output.stderr);
        assert!(output.stderr.is_empty());
        assert_eq!(
            output.stdout,
            format!(
                "ckc: certify ok {GUIDELINE} {} documents {} queries\n",
                usize::from(!self.query),
                usize::from(self.query)
            )
            .as_bytes()
        );
    }
}

fn unescape(field: &str) -> String {
    let mut out = String::new();
    let mut chars = field.chars();
    while let Some(c) = chars.next() {
        out.push(if c == '\\' {
            match chars.next().expect("trailing escape") {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\\' => '\\',
                other => panic!("unknown escape: {other}"),
            }
        } else {
            c
        });
    }
    out
}

fn mutate(source: &str, op: &str, a: &str, b: &str) -> String {
    let changed = match op {
        "replace-first" => {
            assert!(
                !a.is_empty() && source.contains(a),
                "missing replacement anchor"
            );
            source.replacen(a, b, 1)
        }
        "swap-lines" => {
            let mut lines: Vec<_> = source.split_inclusive('\n').collect();
            lines.swap(a.parse().unwrap(), b.parse().unwrap());
            lines.concat()
        }
        "truncate-from" => {
            assert!(b.is_empty() && !a.is_empty());
            source[..source.find(a).expect("missing truncation marker")].to_owned()
        }
        other => panic!("unknown edit: {other}"),
    };
    assert_ne!(changed, source, "mutation is a no-op");
    changed
}

#[test]
fn certification_cases_pin_all_mutations_and_unmodified_inputs() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let table = fs::read_to_string(repo.join("tests/certify/cases.tsv")).unwrap();
    let mut lines = table.lines();
    assert_eq!(
        lines.next(),
        Some("id\tkind\ttarget\top\targ1\targ2\texpected_rc\texpected_stderr")
    );
    let mut fixtures = BTreeMap::new();
    let mut ids = BTreeSet::new();
    let mut counts = [0, 0];
    for line in lines {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 8, "{line}");
        let id = fields[0];
        assert!(!id.is_empty() && ids.insert(id), "duplicate or empty id");
        let query = match fields[1] {
            "doc" => false,
            "query" => true,
            other => panic!("unknown kind: {other}"),
        };
        counts[usize::from(query)] += 1;
        let fixture = fixtures.entry((query, fields[2])).or_insert_with(|| {
            let fixture = Fixture::new(&repo, query, fields[2]);
            fixture.baseline();
            fixture
        });
        let changed = mutate(
            &fixture.original,
            fields[3],
            &unescape(fields[4]),
            &unescape(fields[5]),
        );
        fs::write(&fixture.pl, changed).unwrap();
        let output = fixture.run();
        assert_eq!(
            output.status.code(),
            Some(fields[6].parse().unwrap()),
            "{id}"
        );
        assert!(output.stdout.is_empty(), "{id}: unexpected stdout");
        assert_eq!(output.stderr, unescape(fields[7]).as_bytes(), "{id}");
        fs::write(&fixture.pl, &fixture.original).unwrap();
    }
    assert_eq!(counts, [12, 4]);
    assert_eq!(fixtures.len(), 2);
}
