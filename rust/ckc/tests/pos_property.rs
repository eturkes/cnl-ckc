// pos-property D3/P6: compiler observations + an independent finite-domain lookup oracle.
// CKC_POS_PROPERTY_COMPILER selects a compiler copy for red replay.
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};

const HEADER: &str = "# format: kind\tlemma\tvalue\n";
const TIME: &str = "unit\thour\thour\nrelation\tfor\tduration\nspacing\tat\tinterval\n";
const ULEX: &str =
    "prep(for,for).\nadj_itr(preferred,preferred).\nprep('instead-of','instead-of').\n";
const BOX: &str = "'$guideline_id'(context,posprop,1,box(1),[])";
const ANNOTATIONS: [&str; 9] = [
    "guideline_interval",
    "guideline_recurrence",
    "guideline_frequency",
    "guideline_order",
    "guideline_recurrence_window",
    "guideline_range",
    "guideline_grade",
    "guideline_preference",
    "guideline_qualifier",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn digest_atom(bytes: &[u8]) -> String {
    let hex: String = Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    if hex.as_bytes()[0].is_ascii_lowercase() {
        hex
    } else {
        format!("'{hex}'")
    }
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let kind = entry.file_type().unwrap();
        assert!(!kind.is_symlink());
        let target = to.join(entry.file_name());
        if kind.is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

fn run(cwd: &Path, args: &[String], stdin: &str) -> Output {
    let mut child = Command::new("timeout")
        .args(["--kill-after=2", "60", "swipl"])
        .args(args)
        .current_dir(cwd)
        .env("LC_ALL", "C.UTF-8")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn success<'a>(label: &str, out: &'a Output) -> &'a str {
    assert_eq!(
        out.status.code(),
        Some(0),
        "{label}: stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(out.stderr, b"", "{label}: stderr");
    std::str::from_utf8(&out.stdout).unwrap()
}

struct Compiler {
    scratch: PathBuf,
    stage: PathBuf,
    next: AtomicUsize,
}

impl Compiler {
    fn shared() -> Arc<Self> {
        static SHARED: OnceLock<Mutex<Weak<Compiler>>> = OnceLock::new();
        static SERIAL: AtomicUsize = AtomicUsize::new(0);
        let mut slot = SHARED
            .get_or_init(|| Mutex::new(Weak::new()))
            .lock()
            .unwrap();
        if let Some(compiler) = slot.upgrade() {
            return compiler;
        }
        let scratch = root().join(format!(
            "rust/target/pos-property/{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(
            !scratch.exists(),
            "scratch collision: {}",
            scratch.display()
        );
        fs::create_dir_all(&scratch).unwrap();
        let compiler = Arc::new(Self {
            stage: scratch.join("ape"),
            scratch,
            next: AtomicUsize::new(0),
        });
        let version = run(&compiler.scratch, &["--version".into()], "");
        assert!(success("SWI pin", &version).contains("version 10.0.2 "));
        copy_tree(&root().join("vendor/ape"), &compiler.stage);
        if let Some(path) = std::env::var_os("CKC_POS_PROPERTY_COMPILER") {
            fs::copy(path, compiler.stage.join("prolog/ace_to_pl.pl")).unwrap();
        }
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            compiler.stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let parser = compiler
            .stage
            .join("prolog/parser")
            .to_str()
            .unwrap()
            .replace('\\', "\\\\")
            .replace('\'', "\\'");
        let stage = run(
            &compiler.scratch,
            &[
                "-q".into(),
                "-O".into(),
                "-f".into(),
                "none".into(),
                "-F".into(),
                "none".into(),
                "-g".into(),
                format!("working_directory(_, '{parser}'), [fit_to_plp], halt(0)"),
                "-t".into(),
                "halt(9)".into(),
            ],
            "",
        );
        assert_eq!(stage.status.code(), Some(0), "compiler stage: {stage:?}");
        *slot = Arc::downgrade(&compiler);
        compiler
    }

    fn args(&self, goal: &str) -> Vec<String> {
        let mut args: Vec<String> = ["-q", "-f", "none", "-F", "none", "-s"]
            .iter()
            .map(|s| (*s).into())
            .collect();
        args.push(
            self.stage
                .join("prolog/ace_to_pl.pl")
                .to_str()
                .unwrap()
                .into(),
        );
        args.extend(["-g", goal, "-t", "halt(9)"].map(String::from));
        args
    }

    fn compile(&self, ace: &str, table: &str) -> Output {
        let cwd = self
            .scratch
            .join(self.next.fetch_add(1, Ordering::Relaxed).to_string());
        fs::create_dir(&cwd).unwrap();
        fs::write(cwd.join("vocabulary.tsv"), table).unwrap();
        fs::write(cwd.join("lexicon.ulex"), ULEX).unwrap();
        let mut args = self.args("main");
        args.extend([
            "--".into(),
            self.stage.to_str().unwrap().into(),
            "posprop".into(),
            cwd.join("vocabulary.tsv").to_str().unwrap().into(),
            cwd.join("lexicon.ulex").to_str().unwrap().into(),
        ]);
        run(&cwd, &args, ace)
    }

    fn probe(&self, goals: &[String]) -> Output {
        run(
            &self.scratch,
            &self.args("read_term(user_input,Goal,[]),call(Goal),halt(0)"),
            &format!("({}).\n", goals.join(",\n")),
        )
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.scratch).unwrap();
    }
}

fn table(approximation: &str, preference: &str) -> String {
    format!(
        "{HEADER}{TIME}approximation\t{approximation}\tabout\npreference\t{preference}\tpreferred\nalternative\tinstead-of\talternative\n"
    )
}

fn facts<'a>(stdout: &'a str, predicate: &str) -> Vec<&'a str> {
    let prefix = format!("{predicate}(");
    stdout
        .lines()
        .filter(|line| line.starts_with(&prefix))
        .collect()
}

fn compiled<'a>(label: &str, out: &'a Output, ace: &str, table: &str) -> &'a str {
    let stdout = success(label, out);
    assert_eq!(
        facts(stdout, "guideline_schema_version"),
        ["guideline_schema_version(1)."]
    );
    assert_eq!(
        facts(stdout, "guideline_document"),
        [format!(
            "guideline_document(posprop,ace_sha256({}),ulex(sha256({})),vocabulary(sha256({}))).",
            digest_atom(ace.as_bytes()),
            digest_atom(ULEX.as_bytes()),
            digest_atom(table.as_bytes())
        )],
        "{label}: source record"
    );
    stdout
}

// Only opaque identities come from stdout; their nouns, contexts, roles and links are the oracle.
fn reference(stdout: &str, predicate: &str, ctx: &str, word: &str) -> String {
    let prefix = format!("{predicate}({ctx},");
    let suffix = match predicate {
        "guideline_entity" => format!(",{word},countable)."),
        "guideline_property" => format!(",{word},pos)."),
        _ => format!(",{word})."),
    };
    let refs: Vec<_> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix(&prefix)?.strip_suffix(&suffix))
        .collect();
    assert_eq!(refs.len(), 1, "unique {predicate}/{ctx}/{word}: {stdout}");
    assert!(refs[0].starts_with("'$guideline_id'(product,posprop,1,ref("));
    refs[0].into()
}

fn annotations(stdout: &str) -> Vec<&str> {
    stdout
        .lines()
        .filter(|line| {
            ANNOTATIONS
                .iter()
                .any(|name| line.starts_with(&format!("{name}(")))
        })
        .collect()
}

fn timing(stdout: &str, quantity_ctx: &str, event_ctx: &str, cmp: &str, recurrence: bool) {
    let quantity = reference(stdout, "guideline_entity", quantity_ctx, "hour");
    let patient = reference(stdout, "guideline_entity", "actual", "patient");
    let event = reference(stdout, "guideline_event", event_ctx, "wait");
    let mut cardinalities = vec![
        format!("guideline_cardinality(actual,{patient},na,eq,1)."),
        format!("guideline_cardinality({quantity_ctx},{quantity},na,{cmp},2)."),
    ];
    let annotation = if recurrence {
        let frame = reference(stdout, "guideline_entity", event_ctx, "interval");
        cardinalities.insert(
            1,
            format!("guideline_cardinality({event_ctx},{frame},na,eq,1)."),
        );
        format!("guideline_recurrence({event_ctx},{event},{quantity},hour).")
    } else {
        format!("guideline_interval({event_ctx},{event},duration,{quantity},hour,none).")
    };
    assert_eq!(
        facts(stdout, "guideline_cardinality"),
        cardinalities,
        "quantity comparison"
    );
    assert_eq!(
        annotations(stdout),
        [annotation],
        "time annotation + no strength annotation"
    );
}

#[test]
fn approximation_respects_context() {
    let compiler = Compiler::shared();
    let table = table("approximate", "preferred");
    for (label, ace, quantity_ctx, event_ctx, cmp, recurrence) in [
        (
            "root",
            "A patient waits for 2 approximate hours.\n",
            "actual",
            "actual",
            "about",
            false,
        ),
        (
            "modal",
            "A patient should wait for 2 approximate hours.\n",
            BOX,
            BOX,
            "about",
            false,
        ),
        (
            "other-context",
            "A patient waits for 2 hours that should be approximate.\n",
            "actual",
            "actual",
            "eq",
            false,
        ),
        (
            "frame",
            "A patient waits at an approximate interval of 2 hours.\n",
            "actual",
            "actual",
            "about",
            true,
        ),
    ] {
        let out = compiler.compile(ace, &table);
        let stdout = compiled(label, &out, ace, &table);
        timing(stdout, quantity_ctx, event_ctx, cmp, recurrence);
        let property_ctx = if label == "other-context" {
            BOX
        } else {
            quantity_ctx
        };
        let property = reference(stdout, "guideline_property", property_ctx, "approximate");
        assert_eq!(
            facts(stdout, "guideline_property"),
            [format!(
                "guideline_property({property_ctx},{property},approximate,pos)."
            )],
            "{label}: marker context"
        );
        if label != "other-context" {
            let noun = if recurrence { "interval" } else { "hour" };
            assert_eq!(
                property,
                reference(stdout, "guideline_entity", quantity_ctx, noun)
            );
        }
    }
}

#[test]
fn lemma_lists_stay_separate() {
    let compiler = Compiler::shared();
    for (approximation, preference) in [("approximate", "preferred"), ("preferred", "approximate")]
    {
        let table = table(approximation, preference);
        for adjective in ["approximate", "preferred", "happy"] {
            let ace = format!("A patient waits for 2 {adjective} hours.\n");
            let label = format!("approximation={approximation}/adjective={adjective}");
            let out = compiler.compile(&ace, &table);
            let stdout = compiled(&label, &out, &ace, &table);
            timing(
                stdout,
                "actual",
                "actual",
                if adjective == approximation {
                    "about"
                } else {
                    "eq"
                },
                false,
            );
        }
        let ace = format!("A patient waits for at least 2 {approximation} hours.\n");
        let out = compiler.compile(&ace, &table);
        assert_eq!(
            out.status.code(),
            Some(1),
            "approximate non-exact bound: {out:?}"
        );
        assert_eq!(out.stdout, b"");
        assert_eq!(
            out.stderr,
            b"ace_to_pl_error(unsupported,temporal_shape(approximate_bound,1)).\n"
        );
    }
}

#[test]
fn preference_copula_selects_subject() {
    let compiler = Compiler::shared();
    for (approximation, preference) in [("approximate", "preferred"), ("preferred", "approximate")]
    {
        let table = table(approximation, preference);
        for adjective in ["approximate", "preferred", "happy"] {
            for with_alternative in [false, true] {
                let pp = if with_alternative {
                    " instead-of a cat"
                } else {
                    ""
                };
                let ace = format!("A dog is {adjective}{pp}.\n");
                let label = format!(
                    "preference={preference}/adjective={adjective}/alternative={with_alternative}"
                );
                let out = compiler.compile(&ace, &table);
                if adjective != preference && with_alternative {
                    assert_eq!(out.status.code(), Some(1), "{label}: {out:?}");
                    assert_eq!(out.stdout, b"", "{label}: reject stdout");
                    assert_eq!(
                        out.stderr, b"ace_to_pl_error(unsupported,preference_shape(endpoint,1)).\n",
                        "{label}: reject stderr"
                    );
                    continue;
                }
                let stdout = compiled(&label, &out, &ace, &table);
                let mut expected = Vec::new();
                if adjective == preference {
                    let subject = reference(stdout, "guideline_entity", "actual", "dog");
                    let event = reference(stdout, "guideline_event", "actual", "be");
                    let alternative = if with_alternative {
                        reference(stdout, "guideline_entity", "actual", "cat")
                    } else {
                        "none".into()
                    };
                    expected.push(format!(
                        "guideline_preference(actual,{event},{subject},{alternative})."
                    ));
                }
                assert_eq!(
                    annotations(stdout),
                    expected,
                    "{label}: preference endpoints"
                );
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Property {
    ctx: &'static str,
    referent: &'static str,
    lemma: &'static str,
    degree: &'static str,
}

#[derive(Clone, Copy)]
enum Item {
    Property(Property),
    Other(&'static str),
}

impl Item {
    fn term(self) -> String {
        match self {
            Self::Property(p) => format!(
                "anchored({},property({},{},{}))",
                p.ctx, p.referent, p.lemma, p.degree
            ),
            Self::Other(term) => term.into(),
        }
    }
}

fn lookup(wrapper: &str, lemmas: &str, items: &str, ctx: &str, referent: &str) -> String {
    match wrapper {
        "approximate" => format!(
            "ace_to_pl:v3_approximate(t([],[],[],[],[],{lemmas},[]),{items},{ctx},{referent})"
        ),
        "preferred" => format!("ace_to_pl:s1_preferred_prop({items},{lemmas},{ctx},{referent})"),
        _ => unreachable!(),
    }
}

fn assert_probe_lines(label: &str, out: &Output, expected: &[String]) {
    let stdout = success(label, out);
    let lines: Vec<_> = stdout.lines().collect();
    assert_eq!(lines.len(), expected.len(), "{label}: probe census");
    for (actual, expected) in lines.iter().zip(expected) {
        assert_eq!(actual, expected, "{label}: lookup truth + determinism");
    }
}

#[test]
fn lookup_contract_matrix() {
    let compiler = Compiler::shared();
    let item = |ctx, referent, lemma, degree| {
        Item::Property(Property {
            ctx,
            referent,
            lemma,
            degree,
        })
    };
    let domain = [
        item("ctx", "target", "approximate", "pos"),
        item("ctx", "target", "preferred", "pos"),
        item("ctx", "target", "happy", "pos"),
        item("other", "target", "approximate", "pos"),
        item("ctx", "other", "approximate", "pos"),
        item("ctx", "target", "approximate", "comp"),
        item("ctx", "target", "preferred", "sup"),
        Item::Other("anchored(ctx,property(target,7,pos))"),
        Item::Other("anchored(ctx,property(target,lemma(approximate),pos))"),
        Item::Other("anchored(ctx,property(target,approximate))"),
        Item::Other("anchored(ctx,property(target,approximate,pos,extra))"),
        Item::Other("anchored(ctx,other(target,approximate,pos))"),
        Item::Other("irrelevant"),
    ];
    let lemma_sets: [&[&str]; 6] = [
        &[],
        &["approximate"],
        &["preferred"],
        &["happy"],
        &["approximate", "preferred"],
        &["preferred", "approximate", "preferred"],
    ];
    let mut scopes = vec![Vec::new()];
    for a in domain {
        scopes.push(vec![a]);
        for b in domain {
            scopes.push(vec![a, b]);
        }
    }
    for n in [3, 8, 64] {
        let mut late = vec![Item::Other("irrelevant"); n];
        late.push(domain[0]);
        scopes.push(late);
    }
    let mut goals = Vec::new();
    let mut expected = Vec::new();
    for wrapper in ["approximate", "preferred"] {
        for (l, lemmas) in lemma_sets.iter().enumerate() {
            for (s, scope) in scopes.iter().enumerate() {
                let present = scope.iter().any(|item| {
                    matches!(item, Item::Property(p) if p.ctx == "ctx" && p.referent == "target" && p.degree == "pos" && lemmas.contains(&p.lemma))
                });
                let label = format!("{wrapper}_lemmas{l}_scope{s}");
                let items = scope
                    .iter()
                    .map(|item| item.term())
                    .collect::<Vec<_>>()
                    .join(",");
                let call = lookup(
                    wrapper,
                    &format!("[{}]", lemmas.join(",")),
                    &format!("[{items}]"),
                    "ctx",
                    "target",
                );
                goals.push(format!("findall(ok,{call},Found{label}),length(Found{label},N{label}),format('{label}=~d~n',[N{label}])"));
                expected.push(format!("{label}={}", usize::from(present)));
            }
        }
    }
    assert_eq!(expected.len(), 2232, "finite-domain census");
    let out = compiler.probe(&goals);
    assert_probe_lines("D3 existential property", &out, &expected);
}

#[test]
fn lookup_preserves_unbound_referents() {
    let compiler = Compiler::shared();
    let mut goals = Vec::new();
    let mut expected = Vec::new();
    for wrapper in ["approximate", "preferred"] {
        for (label, items, ctx, referent, invariant, verdict) in [
            (
                "distinct",
                "[anchored(ctx,property(A,approximate,pos))]",
                "ctx",
                "B",
                "var(A),var(B),A\\==B",
                "miss",
            ),
            (
                "same",
                "[anchored(ctx,property(A,approximate,pos))]",
                "ctx",
                "A",
                "var(A)",
                "hit",
            ),
            (
                "ground-versus-var",
                "[anchored(ctx,property(target,approximate,pos))]",
                "ctx",
                "A",
                "var(A)",
                "miss",
            ),
            (
                "var-versus-ground",
                "[anchored(ctx,property(A,approximate,pos))]",
                "ctx",
                "target",
                "var(A)",
                "miss",
            ),
            (
                "context-distinct",
                "[anchored(A,property(target,approximate,pos))]",
                "B",
                "target",
                "var(A),var(B),A\\==B",
                "miss",
            ),
            (
                "context-same",
                "[anchored(A,property(target,approximate,pos))]",
                "A",
                "target",
                "var(A)",
                "hit",
            ),
            (
                "compound-ground",
                "[anchored(scope(1),property(ref(1),approximate,pos))]",
                "scope(1)",
                "ref(1)",
                "true",
                "hit",
            ),
            (
                "compound-distinct",
                "[anchored(ctx,property(ref(A),approximate,pos))]",
                "ctx",
                "ref(B)",
                "var(A),var(B),A\\==B",
                "miss",
            ),
            (
                "compound-same",
                "[anchored(ctx,property(ref(A),approximate,pos))]",
                "ctx",
                "ref(A)",
                "var(A)",
                "hit",
            ),
            (
                "degree-var",
                "[anchored(ctx,property(target,approximate,A))]",
                "ctx",
                "target",
                "var(A)",
                "miss",
            ),
            (
                "lemma-var",
                "[anchored(ctx,property(target,A,pos))]",
                "ctx",
                "target",
                "var(A)",
                "miss",
            ),
        ] {
            let call = lookup(wrapper, "[approximate]", items, ctx, referent);
            // findall isolates each case; the invariant is checked before bindings unwind.
            goals.push(
                format!("findall(V-State,(({call}->V=hit;V=miss),(({invariant})->State=unbound;State=changed)),Results),format('{wrapper}/{label}=~w~n',[Results])")
                    .replace("Results", &format!("Results{}", goals.len())),
            );
            expected.push(format!("{wrapper}/{label}=[{verdict}-unbound]"));
        }
    }
    let out = compiler.probe(&goals);
    assert_probe_lines("D3 non-binding identity", &out, &expected);
}
