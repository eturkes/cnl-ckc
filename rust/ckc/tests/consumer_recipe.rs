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

// One SWI-Prolog process under a UTF-8 locale (LC_ALL=C adds source-load warnings).
fn run(s: &Scratch, files: &[PathBuf], goal: &str) -> std::process::Output {
    let files = files.iter().map(|p| quote(p)).collect::<Vec<_>>().join(",");
    Command::new("timeout")
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
        .env("LANG", "C.UTF-8")
        .env("LC_ALL", "C.UTF-8")
        .stdin(Stdio::null())
        .output()
        .unwrap()
}

// The load gate (contract sgw D7): REFERENCE promises that documents co-load with no
// warnings, so rc 0 + an empty stderr; a warning fails it like an error.
fn quiet(out: &std::process::Output) -> Result<(), String> {
    let stderr = String::from_utf8_lossy(&out.stderr);
    if out.status.success() && stderr.is_empty() {
        Ok(())
    } else {
        Err(format!("swipl rc {:?}: {stderr}", out.status.code()))
    }
}

// The gate's stdout.
fn swipl(s: &Scratch, files: &[PathBuf], goal: &str) -> String {
    let out = run(s, files, goal);
    if let Err(why) = quiet(&out) {
        panic!("{why}");
    }
    String::from_utf8(out.stdout).unwrap()
}

// D7 firing: the pre-SGW spelling of a formula document (its reading = the letter form, P3)
// prints `Singleton variables`, which the gate names with the file; the pre-SGW gate (ERROR
// lines only) passes the same load.
#[test]
fn warning_lines_fail_the_load_gate() {
    let s = Scratch::new("warning-gate");
    let doc = root().join("guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s42-15.pl");
    let letters = ckc_kernel::contract::v1_read(&fs::read(&doc).unwrap());
    let pre = s.file("s42-15-letters.pl", &String::from_utf8(letters).unwrap());
    let files = [root().join("guidelines/schema.pl"), pre];
    let out = run(&s, &files, "true");
    let why = quiet(&out).expect_err("a warning passed the gate");
    assert!(
        why.contains("Singleton variables") && why.contains("s42-15-letters.pl"),
        "{why}"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success() && !stderr.lines().any(|l| l.starts_with("ERROR")),
        "the planted ERROR-only gate no longer passes the warning: {stderr}"
    );
    assert!(quiet(&run(&s, &files[..1], "true")).is_ok(), "schema alone");
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
    assert_eq!(declared.len(), 22, "{}: schema declarations", doc.display());
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

// SUP3 (contract sup1 D3/D4): the superseded document's should/must/may boxes derive only for
// an evaluation date before its effective date; its records derive in every composition. The
// earlier date restores exactly the boxes the adult world derived before the table rows.
#[test]
fn superseded_document_blocks_by_default_and_an_earlier_date_restores_it() {
    const DOC: &str = "cdc2022-opioid-rec12-imp08";
    let cases = [
        ("none", None),
        ("before", Some(20221228)),
        ("on", Some(20221229)),
    ];
    let runs: Vec<_> = cases
        .iter()
        .map(|&(name, date)| {
            std::thread::spawn(move || {
                let s = Scratch::new(&format!("superseded-{name}"));
                let mut extra = vec![world(&s, "adult")];
                if let Some(d) = date {
                    extra.push(s.file(
                        "date.pl",
                        &format!(
                            ":- multifile(guideline_evaluation_date/1).\nguideline_evaluation_date({d}).\n"
                        ),
                    ));
                }
                let out = swipl(&s, &load(&s, Mode::Recipe, &extra), &format!("doc_boxes('{DOC}')"));
                let f: Vec<&str> = out
                    .lines()
                    .find_map(|l| l.strip_prefix("boxes "))
                    .unwrap_or_else(|| panic!("{name}: no boxes row\n{out}"))
                    .split(' ')
                    .collect();
                assert!(f.len() == 4 && f[0] == DOC && f[2] == "records", "{name}: {out}");
                (name, (f[1].parse::<usize>().unwrap(), f[3].parse::<usize>().unwrap()))
            })
        })
        .collect();
    let got: BTreeMap<_, _> = runs.into_iter().map(|r| r.join().unwrap()).collect();
    assert_eq!(
        got["none"],
        (0, 3),
        "no evaluation date: blocked, 3 records"
    );
    assert_eq!(got["on"], (0, 3), "the effective date itself: blocked");
    assert_eq!(got["before"], (2, 3), "an earlier date restores the boxes");
}

#[test]
fn dose_questions_answer_under_the_recipe_and_open_goals_diverge_without_it() {
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
    // Firing: plain left-to-right search answers the targeted questions (binding-first bodies,
    // contract bf) but does not exhaust every should box of the same composition.
    let goal = "bounded_all(20, plain, guideline_operator(actual, _, should))";
    let out = swipl(&s, &load(&s, Mode::Plain, &[world]), goal);
    assert!(
        unfinished(&out, "plain"),
        "plain search exhausted every should box: the recipe text may be stale\n{out}"
    );
}

// A bounded run that did not finish: its one report line names the time limit or a resource
// error. Which one a divergent search hits first depends on the host (user ruling: either one).
fn unfinished(out: &str, name: &str) -> bool {
    out.strip_prefix(&format!("bounded {name} "))
        .and_then(|r| r.strip_suffix('\n'))
        .is_some_and(|r| {
            !r.contains('\n')
                && (r == "time_limit_exceeded" || r.starts_with("error(resource_error("))
        })
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
