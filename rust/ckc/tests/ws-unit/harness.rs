use super::oracle::{self, Q, Term, Walk};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
pub fn fixture(name: &str) -> String {
    fs::read_to_string(root().join("tests/ws-unit").join(name)).unwrap()
}
pub fn sha(s: &str) -> String {
    Sha256::digest(s.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn custody(pl: &str, before: &str, after: &str) -> String {
    fn atom(s: String) -> String {
        if s.as_bytes()[0].is_ascii_lowercase() {
            s
        } else {
            format!("'{s}'")
        }
    }
    let old = format!("ace_sha256({})", atom(sha(before)));
    let new = format!("ace_sha256({})", atom(sha(after)));
    assert!(pl.contains(&old), "ACE custody anchor");
    pl.replace(&old, &new)
}
pub fn p(path: &Path) -> &str {
    path.to_str().unwrap()
}
fn quoted(path: &Path) -> String {
    format!("'{}'", p(path).replace('\\', "\\\\").replace('\'', "\\'"))
}

pub struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new(label: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let dir = root().join(format!(
            "rust/target/ws-unit/{}-{label}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }
    pub fn file(&self, name: &str, text: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, text).unwrap();
        path
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Debug)]
pub struct Run {
    pub rc: Option<i32>,
    pub out: String,
    pub err: String,
}
impl Run {
    pub fn ok(&self) -> bool {
        self.rc == Some(0) && self.err.is_empty()
    }
    pub fn summary(&self) -> String {
        format!(
            "rc={:?}; stdout={}; stderr={}",
            self.rc,
            self.out.chars().take(450).collect::<String>(),
            self.err.chars().take(450).collect::<String>()
        )
    }
}
pub fn run(program: &Path, cwd: &Path, args: &[&str], input: &str) -> Run {
    let mut child = Command::new(program)
        .current_dir(cwd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    Run {
        rc: out.status.code(),
        out: String::from_utf8(out.stdout).unwrap(),
        err: String::from_utf8(out.stderr).unwrap(),
    }
}
pub fn native(s: &Scratch, args: &[&str]) -> Run {
    let bin = std::env::var_os("CKC_WS_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("rust/target/release/ckc"));
    run(&bin, &s.0, args, "")
}
fn swi() -> PathBuf {
    std::env::var_os("SWIPL")
        .map(PathBuf::from)
        .unwrap_or_else(|| "swipl".into())
}
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let kind = entry.file_type().unwrap();
        assert!(
            !kind.is_symlink(),
            "stage source symlink: {}",
            entry.path().display()
        );
        if kind.is_dir() {
            copy_tree(&entry.path(), &to.join(entry.file_name()));
        } else {
            fs::copy(entry.path(), to.join(entry.file_name())).unwrap();
        }
    }
}
pub struct Compiler {
    _scratch: Scratch,
    pub stage: PathBuf,
}
impl Compiler {
    pub fn shared() -> Arc<Self> {
        static CACHE: OnceLock<Mutex<Weak<Compiler>>> = OnceLock::new();
        let mut weak = CACHE
            .get_or_init(|| Mutex::new(Weak::new()))
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(c) = weak.upgrade() {
            return c;
        }
        let s = Scratch::new("stage");
        let stage = s.0.join("ape");
        copy_tree(&root().join("vendor/ape"), &stage);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        for goal in [
            "current_prolog_flag(version_data,swi(10,0,2,_)),halt(0)".to_owned(),
            format!(
                "working_directory(_,{}),[fit_to_plp],halt(0)",
                quoted(&stage.join("prolog/parser"))
            ),
            format!(
                "style_check(-discontiguous),qcompile({}),halt(0)",
                quoted(&stage.join("prolog/lexicon/clex_lexicon.pl"))
            ),
        ] {
            let got = run(
                &swi(),
                &s.0,
                &[
                    "-q", "-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
                ],
                "",
            );
            assert_eq!(got.rc, Some(0), "stage: {}", got.summary());
        }
        let c = Arc::new(Self { _scratch: s, stage });
        *weak = Arc::downgrade(&c);
        c
    }
    pub fn emit(&self, s: &Scratch, id: &str, ace: &str, proof: bool) -> Run {
        let ulex = s.file("lexicon.ulex", "");
        let script = self.stage.join("prolog/ace_to_pl.pl");
        let mut args = vec![
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            p(&script),
            "-g",
            "main",
            "-t",
            "halt(9)",
            "--",
            p(&self.stage),
            id,
            "-",
            p(&ulex),
        ];
        if proof {
            args.push("proof");
        }
        run(&swi(), &s.0, &args, ace)
    }
    pub fn dump(&self, s: &Scratch, ace: &str, ulex: &Path) -> String {
        let script = root().join("rust/ckc/prolog/drs_dump.pl");
        let got = run(
            &swi(),
            &s.0,
            &[
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-s",
                p(&script),
                "-g",
                "main",
                "-t",
                "halt(9)",
                "--",
                p(&self.stage),
                p(ulex),
            ],
            ace,
        );
        assert!(
            got.ok() && got.out.contains("messages([])."),
            "APE fixture: {}\nmessages={}\n{ace}",
            got.summary(),
            got.out
                .rsplit_once("messages(")
                .map(|(_, m)| m)
                .unwrap_or("(absent)")
        );
        got.out
    }
}

pub struct Case {
    pub id: String,
    pub source: String,
    pub emitted: Run,
    pub ace: PathBuf,
    pub pl: PathBuf,
    pub dump: PathBuf,
    pub ulex: PathBuf,
}
impl Case {
    pub fn new(
        s: &Scratch,
        c: &Compiler,
        id: &str,
        ace: &str,
        cousin: Option<(&str, &[(&str, &str)])>,
    ) -> Self {
        let source = s.file(&format!("{id}.ace"), ace);
        let ulex = s.file("lexicon.ulex", "");
        let dump = c.dump(s, ace, &ulex);
        let emitted = c.emit(s, id, ace, false);
        let pl = if emitted.ok() {
            emitted.out.clone()
        } else {
            assert!(
                emitted.err.contains("underivable_obligation"),
                "fixture syntax/shape {id}: {}\n{ace}",
                emitted.summary()
            );
            let (safe, swaps) = cousin.expect("red projection needs a replayable cousin");
            let control = c.emit(s, id, safe, false);
            assert!(control.ok(), "cousin {id}: {}\n{safe}", control.summary());
            let mut pl = custody(&control.out, safe, ace);
            // Source comments + custody change independently of semantic noun substitutions.
            for (i, sentence) in ace.lines().enumerate() {
                let prefix = format!("% S{}: ", i + 1);
                let old = pl
                    .lines()
                    .find(|l| l.starts_with(&prefix))
                    .expect("sentence comment")
                    .to_owned();
                pl = pl.replace(&old, &format!("{prefix}{sentence}"));
            }
            for &(from, to) in swaps {
                assert!(pl.contains(from), "cousin anchor {from}");
                pl = pl.replace(from, to);
            }
            pl
        };
        Self {
            id: id.to_owned(),
            source: ace.to_owned(),
            emitted,
            ace: source,
            pl: s.file(&format!("{id}.pl"), &pl),
            dump: s.file(&format!("{id}.dump"), &dump),
            ulex,
        }
    }
    pub fn call(&self, s: &Scratch, mode: &str) -> Run {
        native(
            s,
            &[
                "certify-one",
                mode,
                &self.id,
                p(&self.ace),
                p(&self.ulex),
                p(&self.dump),
                p(&self.pl),
            ],
        )
    }
    pub fn replay(&self, s: &Scratch, payload: &str) -> Run {
        let proof = s.file(&format!("{}.proof", self.id), payload);
        let manifest = s.file(
            &format!("{}.tsv", self.id),
            &format!("{}\t{}\n", self.pl.display(), proof.display()),
        );
        native(s, &["v1", "aggregate-check", p(&manifest)])
    }
    pub fn synthetic_bound(&self, op: &str, old: i128, new: Q) {
        for path in [&self.dump, &self.pl] {
            let text = fs::read_to_string(path).unwrap();
            let from = format!(",{op},{old})");
            assert!(
                text.contains(&from),
                "synthetic DRS bound anchor {}: {from}",
                path.display()
            );
            fs::write(path, text.replace(&from, &format!(",{op},{new})"))).unwrap();
        }
    }
}

#[derive(Default)]
pub struct Checks {
    errors: Vec<String>,
    checked: usize,
}
impl Checks {
    pub fn check(&mut self, yes: bool, label: impl Into<String>) {
        self.checked += 1;
        if !yes {
            self.errors.push(label.into());
        }
    }
    pub fn success(&mut self, got: &Run, label: &str) {
        self.check(got.ok(), format!("{label}: {}", got.summary()));
    }
    pub fn walk(&mut self, got: &Run, label: &str) -> Option<Vec<Walk>> {
        self.success(got, label);
        if got.ok() {
            Some(oracle::walks(&got.out))
        } else {
            None
        }
    }
    pub fn finish(self) {
        println!(
            "WS checks={} mismatches={}",
            self.checked,
            self.errors.len()
        );
        assert!(self.errors.is_empty(), "{}", self.errors.join("\n"));
    }
}

fn counts(payload: &str, sentence: i128, noun: &str, slot: usize, wrapper: &str) -> Vec<Q> {
    let mut out = Vec::new();
    for proof in oracle::terms(payload) {
        let a = proof.args("$guideline_proof");
        if a[1] != Term::Num(sentence) {
            continue;
        }
        let facts = a[slot].args(wrapper)[0].list();
        let refs: Vec<_> = facts
            .iter()
            .filter_map(|f| match f {
                Term::Fun(n, xs)
                    if n == "guideline_entity" && xs[2] == Term::Atom(noun.to_owned()) =>
                {
                    Some(&xs[1])
                }
                _ => None,
            })
            .collect();
        for f in facts {
            if let Term::Fun(n, xs) = f
                && n == "guideline_cardinality"
                && refs.contains(&&xs[1])
            {
                assert_eq!(
                    xs[3].atom(),
                    "exactly",
                    "selected witness + arithmetic result are points"
                );
                out.push(xs[4].q());
            }
        }
    }
    out
}
pub fn cardinalities(payload: &str, sentence: i128, noun: &str) -> Vec<Q> {
    counts(payload, sentence, noun, 3, "witness")
}
pub fn head_counts(payload: &str, sentence: i128, noun: &str) -> Vec<Q> {
    counts(payload, sentence, noun, 4, "prove")
}
