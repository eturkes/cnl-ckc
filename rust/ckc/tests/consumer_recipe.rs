// Consumer recipe (contract cr): the tabling declaration of REFERENCE § Loading a whole
// knowledge base lets a standard engine answer over every committed document + one patient;
// a well-founded rerun (tnot for every `\+`) must agree with no undefined answer.
// Fixtures `tests/consumer/`; the dose world is the dose-arithmetic query fixture's.
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const HEADING: &str = "### Loading a whole knowledge base\n";
const DOSE: &str = "tests/queries/green/dose-arithmetic/tree/guidelines/fx";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

struct Scratch(PathBuf);
impl Scratch {
    fn new(name: &str) -> Self {
        let dir = root()
            .join("rust/target/consumer-recipe")
            .join(format!("{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Scratch(dir)
    }
    fn file(&self, name: &str, text: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, text).unwrap();
        path
    }
    fn fixture(&self, name: &str) -> PathBuf {
        let text = fs::read_to_string(root().join("tests/consumer").join(format!("{name}.in")));
        self.file(name, &text.unwrap())
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

// The first `prolog` block after the heading.
fn recipe() -> String {
    let text = fs::read_to_string(root().join("docs/REFERENCE.md")).unwrap();
    let at = text.find(HEADING).expect("REFERENCE: no recipe heading") + HEADING.len();
    let at = at
        + text[at..]
            .find("```prolog\n")
            .expect("REFERENCE: no prolog block")
        + 10;
    text[at..at + text[at..].find("```\n").unwrap()].to_owned()
}

fn sorted(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(dir)
        .map(|it| it.map(|e| e.unwrap().path()).collect())
        .unwrap_or_default();
    v.retain(|p| p.extension().is_some_and(|e| e == "pl"));
    v.sort();
    v
}

fn guidelines() -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(root().join("guidelines"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();
    v.sort();
    v
}

fn quote(p: &Path) -> String {
    format!(
        "'{}'",
        p.to_str()
            .unwrap()
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
    )
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Plain,
    Recipe,
    WellFounded,
}

// Load order: the recipe declaration (absent in Plain), the well-founded rewrite, the driver,
// guidelines/schema.pl, every committed document of every guideline, then `extra`.
fn load(s: &Scratch, mode: Mode, extra: &[PathBuf]) -> Vec<PathBuf> {
    let mut v = Vec::new();
    if mode != Mode::Plain {
        v.push(s.file("recipe.pl", &recipe()));
    }
    if mode == Mode::WellFounded {
        v.push(s.fixture("wfs.pl"));
    }
    v.push(s.fixture("driver.pl"));
    v.push(root().join("guidelines/schema.pl"));
    for g in guidelines() {
        v.extend(sorted(&g.join("pl")));
    }
    v.extend(extra.iter().cloned());
    v
}

// One SWI-Prolog process; its stdout, after rc 0 and no ERROR line.
fn swipl(s: &Scratch, files: &[PathBuf], goal: &str) -> String {
    let files = files.iter().map(|p| quote(p)).collect::<Vec<_>>().join(",");
    let out = Command::new("timeout")
        .args([
            "--kill-after=5",
            "900",
            "swipl",
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-g",
        ])
        .arg(format!("consult([{files}]), {goal}"))
        .args(["-t", "halt"])
        .current_dir(&s.0)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success() && !stderr.lines().any(|l| l.starts_with("ERROR")),
        "swipl rc {:?}: {stderr}",
        out.status.code()
    );
    String::from_utf8(out.stdout).unwrap()
}

// `enum <goal> <count> undefined <n> <hash>` rows by goal.
fn enums(out: &str) -> BTreeMap<String, (usize, usize, String)> {
    out.lines()
        .filter_map(|l| l.strip_prefix("enum "))
        .map(|l| {
            let f: Vec<&str> = l.split(' ').collect();
            assert!(f.len() == 5 && f[2] == "undefined", "enum row: {l}");
            (
                f[0].to_owned(),
                (
                    f[1].parse().unwrap(),
                    f[3].parse().unwrap(),
                    f[4].to_owned(),
                ),
            )
        })
        .collect()
}

fn world(s: &Scratch, name: &str) -> PathBuf {
    match name {
        "dose" => root().join(DOSE).join("pl/world.pl"),
        _ => s.fixture(&format!("{name}.pl")),
    }
}

#[test]
fn recipe_tables_every_schema_predicate() {
    let doc = sorted(&guidelines()[0].join("pl")).remove(0);
    let declared: Vec<String> = fs::read_to_string(&doc)
        .unwrap()
        .lines()
        .filter_map(|l| {
            l.strip_prefix(":- multifile(")?
                .strip_suffix(").")
                .map(str::to_owned)
        })
        .collect();
    let recipe = recipe();
    let tabled: Vec<String> = recipe
        .trim_end()
        .strip_prefix(":- table ")
        .and_then(|r| r.strip_suffix('.'))
        .expect("recipe = one `:- table ….` directive")
        .split(',')
        .map(|p| p.trim().to_owned())
        .collect();
    assert_eq!(declared.len(), 20, "{}: schema declarations", doc.display());
    assert_eq!(
        tabled,
        declared,
        "recipe vs the declarations of {}",
        doc.display()
    );
}

#[test]
fn committed_questions_keep_their_answers_under_the_recipe() {
    let s = Scratch::new("questions");
    let mut calls = Vec::new();
    for g in guidelines() {
        for q in sorted(&g.join("queries/pl")) {
            let a = g.join("queries/answers").join(q.file_name().unwrap());
            calls.push(format!("answer({}, {})", quote(&q), quote(&a)));
        }
    }
    assert!(!calls.is_empty(), "no committed question");
    let out = swipl(&s, &load(&s, Mode::Recipe, &[]), &calls.join(", "));
    let rows: Vec<&str> = out.lines().filter(|l| l.starts_with("answer ")).collect();
    assert_eq!(rows.len(), calls.len(), "{out}");
    assert!(rows.iter().all(|r| r.ends_with(" same")), "{out}");
}

// Each world: the recipe finishes every open goal; the well-founded run gives the same answer
// sets with no undefined answer; a scope exclusion silences every should/must/may box.
#[test]
fn patient_worlds_finish_and_agree_with_the_well_founded_model() {
    let cases = [
        ("dose", false),
        ("adult", false),
        ("minor", true),
        ("cancer", true),
    ];
    let runs: Vec<_> = cases
        .iter()
        .flat_map(|&(w, excluded)| {
            [Mode::Recipe, Mode::WellFounded].map(|mode| {
                std::thread::spawn(move || {
                    let s = Scratch::new(&format!("{w}-{}", mode as u8));
                    let files = load(&s, mode, &[world(&s, w)]);
                    (w, excluded, mode, swipl(&s, &files, "enum"))
                })
            })
        })
        .collect();
    let mut by_world: BTreeMap<&str, Vec<(Mode, String)>> = BTreeMap::new();
    let mut excluded_of = BTreeMap::new();
    for r in runs {
        let (w, excluded, mode, out) = r.join().unwrap();
        excluded_of.insert(w, excluded);
        by_world.entry(w).or_default().push((mode, out));
    }
    for (w, outs) in by_world {
        let [(_, a), (_, b)] = <[_; 2]>::try_from(outs).ok().unwrap();
        let (ra, rb) = (enums(&a), enums(&b));
        assert_eq!(ra.len(), 7, "{w}: {a}");
        assert_eq!(ra, rb, "{w}: recipe vs well-founded\n{a}\n{b}");
        assert!(
            ra.values().all(|(_, u, _)| *u == 0),
            "{w}: undefined answers\n{b}"
        );
        let n = |g: &str| ra[g].0;
        if excluded_of[w] {
            assert!(n("exclusion") > 0, "{w}: no exclusion\n{a}");
            assert_eq!(
                (n("should"), n("must"), n("may")),
                (0, 0, 0),
                "{w}: boxes\n{a}"
            );
        } else {
            assert_eq!(n("exclusion"), 0, "{w}: exclusion\n{a}");
            assert!(n("should") > 0, "{w}: no should box\n{a}");
        }
    }
}

#[test]
fn dose_questions_answer_under_the_recipe_and_diverge_without_it() {
    let s = Scratch::new("dose");
    let fx = root().join(DOSE);
    let calls: Vec<String> = ["q-mme", "q-mme-exact"]
        .iter()
        .map(|q| {
            let pl = fx.join(format!("queries/pl/{q}.pl"));
            let ans = fx.join(format!("queries/answers/{q}.pl"));
            format!("answer({}, {})", quote(&pl), quote(&ans))
        })
        .collect();
    let world = fx.join("pl/world.pl");
    let out = swipl(
        &s,
        &load(&s, Mode::Recipe, std::slice::from_ref(&world)),
        &calls.join(", "),
    );
    assert_eq!(out.matches(" same\n").count(), 2, "{out}");
    // Firing: plain left-to-right search over the same composition does not finish.
    let goal = "bounded(20, plain, guideline_entity(actual, _, 'opioid-dosage', countable))";
    let out = swipl(&s, &load(&s, Mode::Plain, &[world]), goal);
    assert_eq!(
        out, "bounded plain time_limit_exceeded\n",
        "plain search finished: the recipe text may be stale"
    );
}

// Firing of the well-founded check: a recursion through negation reads undefined.
#[test]
fn well_founded_run_flags_recursion_through_negation() {
    let s = Scratch::new("negloop");
    let lp = s.file(
        "negloop.pl",
        ":- multifile(guideline_exclusion/1).\nguideline_exclusion(a) :- \\+ guideline_exclusion(b).\nguideline_exclusion(b) :- \\+ guideline_exclusion(a).\n",
    );
    let mut files = vec![
        s.file("recipe.pl", &recipe()),
        s.fixture("wfs.pl"),
        s.fixture("driver.pl"),
    ];
    files.extend([root().join("guidelines/schema.pl"), lp]);
    let out = swipl(&s, &files, "enum(exclusion, guideline_exclusion(_))");
    let row = enums(&out);
    assert_eq!(row["exclusion"].1, 2, "{out}");
}
