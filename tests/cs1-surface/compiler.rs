use super::model::{self, KEY, Product, ULEX, quote};
use super::support::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

pub struct Compiler {
    stage: PathBuf,
}
impl Compiler {
    pub fn new(scratch: &Path) -> Self {
        let version = command(Path::new("swipl"), scratch, &["--version"], "");
        version.require_ok("SWI pin");
        assert!(version.stdout.contains("version 10.0.2 "));
        let stage = scratch.join("ape");
        copy_tree(&root().join("vendor/ape"), &stage);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let goal = format!(
            "working_directory(_,{}),[fit_to_plp],halt.",
            quote(stage.join("prolog/parser").to_str().unwrap())
        );
        let out = command(
            Path::new("swipl"),
            scratch,
            &[
                "-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
            ],
            "",
        );
        assert_eq!(out.rc, Some(0), "APE stage: {}", out.describe());
        let goal = format!(
            "style_check(-discontiguous),qcompile({}),halt.",
            quote(
                stage
                    .join("prolog/lexicon/clex_lexicon.pl")
                    .to_str()
                    .unwrap()
            )
        );
        command(
            Path::new("swipl"),
            scratch,
            &[
                "-q", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
            ],
            "",
        )
        .require_ok("Clex stage");
        Self { stage }
    }
    pub fn run(&self, cwd: &Path, id: &str, mode: &str, ace: &str, table: Option<&str>) -> Run {
        fs::create_dir_all(cwd).unwrap();
        let tab = cwd.join("vocabulary.tsv");
        let ulex = cwd.join("lexicon.ulex");
        if let Some(t) = table {
            write(&tab, t);
        }
        write(&ulex, ULEX);
        let src = self.stage.join("prolog/ace_to_pl.pl");
        let mut args = vec![
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            src.to_str().unwrap(),
            "-g",
            "main",
            "-t",
            "halt(9)",
            "--",
        ];
        if mode == "question" {
            args.push("question");
        }
        args.extend([
            self.stage.to_str().unwrap(),
            id,
            if table.is_some() {
                tab.to_str().unwrap()
            } else {
                "-"
            },
            ulex.to_str().unwrap(),
        ]);
        if mode == "proof" {
            args.push("proof");
        }
        command(Path::new("swipl"), cwd, &args, ace)
    }
    pub fn dump(&self, cwd: &Path, ace: &str) -> Run {
        fs::create_dir_all(cwd).unwrap();
        let ulex = cwd.join("lexicon.ulex");
        write(&ulex, ULEX);
        let driver = root().join("rust/ckc/prolog/drs_dump.pl");
        command(
            Path::new("swipl"),
            cwd,
            &[
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-s",
                driver.to_str().unwrap(),
                "-g",
                "main",
                "-t",
                "halt(9)",
                "--",
                self.stage.to_str().unwrap(),
                ulex.to_str().unwrap(),
            ],
            ace,
        )
    }
}
#[derive(Clone)]
pub struct Probe {
    pub id: String,
    pub ace: String,
    pub row: &'static str,
    pub exclusion: bool,
}
fn probe(id: &str, ace: &str, row: &'static str, exclusion: bool) -> Probe {
    Probe {
        id: format!("cs-{id}"),
        ace: format!("{ace}\n"),
        row,
        exclusion,
    }
}
pub fn probes() -> Vec<Probe> {
    let mut out = vec![
        probe(
            "exclude-zero",
            "If a patient sleeps then it is false that a recommendation applies.",
            "exclude",
            true,
        ),
        probe(
            "exclude-one",
            "If a patient has a cancer-related-pain then it is false that a recommendation applies to the patient.",
            "exclude",
            true,
        ),
        probe(
            "exclude-two",
            "If a patient receives a hospital-care then it is false that a recommendation applies to the patient during the hospital-care.",
            "exclude",
            true,
        ),
        probe(
            "exclude-curried",
            "If a patient has a cancer-related-pain then if the patient sleeps then it is false that a recommendation applies to the patient.",
            "exclude",
            true,
        ),
        probe(
            "exclude-split",
            "If a patient has a cancer-related-pain or a woman sleeps then it is false that a recommendation applies.",
            "exclude",
            true,
        ),
        probe(
            "exclude-naf",
            "If a patient receives a hospital-care and the patient does not provably have a facility-discharge then it is false that a recommendation applies to the patient.",
            "exclude",
            true,
        ),
        probe(
            "exclude-comparison",
            "If a patient has an age and the age includes less than 18 years then it is false that a recommendation applies to the patient.",
            "exclude",
            true,
        ),
        probe(
            "exclude-time-pp",
            "If a patient receives a hospital-care then it is false that a recommendation applies after the hospital-care.",
            "exclude",
            true,
        ),
        probe(
            "exclude-external-duration",
            "If a patient waits for 2 hours then it is false that a recommendation applies for the hours.",
            "exclude",
            true,
        ),
        probe(
            "data-should",
            "If a patient sleeps then it is recommended that a recommendation applies to the patient.",
            "data",
            false,
        ),
        probe(
            "data-can",
            "If a patient sleeps then it is possible that it is false that a recommendation applies to the patient.",
            "data",
            false,
        ),
        probe(
            "data-should-negation",
            "If a patient sleeps then it is recommended that it is false that a recommendation applies to the patient.",
            "data",
            false,
        ),
        probe(
            "data-root-can",
            "It is possible that it is false that a recommendation applies.",
            "data",
            false,
        ),
        probe(
            "data-antecedent",
            "If a recommendation applies to a patient then the patient sleeps.",
            "data",
            false,
        ),
        probe(
            "data-positive",
            "If a patient sleeps then a recommendation applies to the patient.",
            "data",
            false,
        ),
        probe("data-fact", "A recommendation applies.", "data", false),
        probe(
            "data-other-verb",
            "If a patient sleeps then it is false that a recommendation changes.",
            "data",
            false,
        ),
    ];
    for op in ["should", "must", "may"] {
        for (shape, ace) in [
            (
                "top",
                format!("If a patient sleeps then the patient {op} wait."),
            ),
            (
                "can",
                format!("If a patient sleeps then it is possible that the patient {op} wait."),
            ),
            (
                "split",
                format!("If a man sleeps or a woman waits then a patient {op} work."),
            ),
            (
                "naf",
                format!(
                    "If a patient sleeps and the patient does not provably work then the patient {op} wait."
                ),
            ),
            (
                "comparison",
                format!(
                    "If a patient has an age and the age includes less than 18 years then the patient {op} wait."
                ),
            ),
            (
                "two",
                format!(
                    "If a patient sleeps then the patient {op} wait and the patient {op} work."
                ),
            ),
        ] {
            out.push(probe(&format!("guard-{op}-{shape}"), &ace, "guard", false));
        }
    }
    out
}
#[derive(Clone)]
struct Red {
    id: &'static str,
    ace: &'static str,
    why: &'static str,
    sentence: usize,
}
fn reds() -> Vec<Red> {
    vec![
        Red {
            id: "deontic-condition",
            ace: "If a patient should sleep then the patient waits.\n",
            why: "deontic_condition",
            sentence: 1,
        },
        Red {
            id: "deontic-condition-must",
            ace: "If a patient must sleep then the patient waits.\n",
            why: "deontic_condition",
            sentence: 1,
        },
        Red {
            id: "deontic-condition-may",
            ace: "If a patient may sleep then the patient waits.\n",
            why: "deontic_condition",
            sentence: 1,
        },
        Red {
            id: "deontic-split",
            ace: "If a man should sleep or a woman waits then a patient works.\n",
            why: "deontic_condition",
            sentence: 1,
        },
        Red {
            id: "deontic-condition-nested",
            ace: "If it is possible that a patient should sleep then a man waits.\n",
            why: "deontic_condition",
            sentence: 1,
        },
        Red {
            id: "negated-deontic-nested",
            ace: "If a patient sleeps then it is false that it is possible that the patient must wait.\n",
            why: "negated_deontic",
            sentence: 1,
        },
        Red {
            id: "negated-deontic",
            ace: "If a patient sleeps then it is false that the patient should wait.\n",
            why: "negated_deontic",
            sentence: 1,
        },
        Red {
            id: "negated-must",
            ace: "If a patient sleeps then it is false that the patient must wait.\n",
            why: "negated_deontic",
            sentence: 1,
        },
        Red {
            id: "negated-may",
            ace: "If a patient sleeps then it is false that the patient may wait.\n",
            why: "negated_deontic",
            sentence: 1,
        },
        Red {
            id: "root-deontic",
            ace: "A patient should wait.\n",
            why: "root_deontic",
            sentence: 1,
        },
        Red {
            id: "root-nested-deontic",
            ace: "It is possible that a patient must wait.\n",
            why: "root_deontic",
            sentence: 1,
        },
        Red {
            id: "root-exclusion",
            ace: "It is false that a recommendation applies.\n",
            why: "root",
            sentence: 1,
        },
        Red {
            id: "subject-universal",
            ace: "Every recommendation does not apply to a patient.\n",
            why: "subject",
            sentence: 1,
        },
        Red {
            id: "subject-no",
            ace: "No recommendation applies to a patient.\n",
            why: "subject",
            sentence: 1,
        },
        Red {
            id: "subject-noun",
            ace: "If a patient sleeps then it is false that an opioid-dosage-recommendation applies to the patient.\n",
            why: "subject",
            sentence: 1,
        },
        Red {
            id: "consequent-two",
            ace: "If a patient sleeps then it is false that 2 recommendations apply to the patient.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "consequent-geq",
            ace: "If a patient sleeps then it is false that at least 2 recommendations apply to the patient.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "consequent-exact",
            ace: "If a patient sleeps then it is false that exactly 1 recommendation applies to the patient.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "consequent-extra",
            ace: "If a patient sleeps then it is false that a recommendation applies and sleeps.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "consequent-extra-top",
            ace: "If a patient sleeps then the patient waits and it is false that a recommendation applies.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "consequent-property",
            ace: "If a patient sleeps then it is false that a good recommendation applies to the patient.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "consequent-transitive",
            ace: "If a patient sleeps then it is false that a recommendation applies the patient.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "consequent-local-pp",
            ace: "If a patient sleeps then it is false that a recommendation applies to a man.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "consequent-local-time-pp",
            ace: "If a patient sleeps then it is false that a recommendation applies for 2 hours.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "order-condition-negated",
            ace: "If a patient should sleep then it is false that the patient must wait.\n",
            why: "deontic_condition",
            sentence: 1,
        },
        Red {
            id: "order-condition-subject",
            ace: "If a patient should sleep then it is false that an opioid-dosage-recommendation applies to the patient.\n",
            why: "deontic_condition",
            sentence: 1,
        },
        Red {
            id: "order-negated-subject",
            ace: "If a patient sleeps then it is false that an opioid-dosage-recommendation applies to a man who should wait.\n",
            why: "negated_deontic",
            sentence: 1,
        },
        Red {
            id: "order-root-deontic",
            ace: "It is false that a recommendation applies and a patient should wait.\n",
            why: "root_deontic",
            sentence: 1,
        },
        Red {
            // reviewer-2 F1 ruling: subject + consequent read the first verb box; a second one is one more condition.
            id: "order-two-exclusion-boxes",
            ace: "If a patient sleeps then it is false that a recommendation applies to the patient and it is false that a plan applies to the patient.\n",
            why: "consequent",
            sentence: 1,
        },
        Red {
            id: "sentence-two",
            ace: "A man sleeps.\nIt is false that a recommendation applies.\n",
            why: "root",
            sentence: 2,
        },
    ]
}
pub struct Compiled {
    pub probe: Probe,
    pub model: Product,
    pub doc: Run,
    pub proof: Run,
    pub baseline: Run,
    pub base_proof: Run,
    pub no_table: Run,
    pub no_table_model: Product,
}
struct Results {
    docs: BTreeMap<String, Compiled>,
    reds: BTreeMap<String, Run>,
    precedence: Run,
}
fn results() -> &'static Results {
    static RESULTS: OnceLock<Result<Results, ()>> = OnceLock::new();
    RESULTS
        .get_or_init(|| std::panic::catch_unwind(collect).map_err(|_| ()))
        .as_ref()
        .expect("compiler fixture setup failed; see first diagnostic")
}
fn collect() -> Results {
    let scratch = Scratch::new("compiler");
    let compiler = Compiler::new(&scratch.0);
    let cases = probes();
    let next = AtomicUsize::new(0);
    let found = Mutex::new(BTreeMap::new());
    std::thread::scope(|scope| {
        for worker in 0..4 {
            let (scratch, compiler, cases, next, found) =
                (&scratch, &compiler, &cases, &next, &found);
            scope.spawn(move || {
                let cwd = scratch.0.join(format!("worker-{worker}"));
                while let Some(p) = cases.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let dump = compiler.dump(&cwd, &p.ace);
                    dump.require_ok(&format!("{} APE DRS", p.id));
                    let model = Product::from_dump(&p.id, &p.ace, &dump.stdout, true);
                    let no_table_model = Product::from_dump(&p.id, &p.ace, &dump.stdout, false);
                    let doc =
                        compiler.run(&cwd, &p.id, "doc", &p.ace, Some(&model::table(Some(KEY))));
                    let proof =
                        compiler.run(&cwd, &p.id, "proof", &p.ace, Some(&model::table(Some(KEY))));
                    let baseline =
                        compiler.run(&cwd, &p.id, "doc", &p.ace, Some(&model::table(None)));
                    let base_proof =
                        compiler.run(&cwd, &p.id, "proof", &p.ace, Some(&model::table(None)));
                    let no_table = compiler.run(&cwd, &p.id, "doc", &p.ace, None);
                    found.lock().unwrap().insert(
                        p.id.clone(),
                        Compiled {
                            probe: p.clone(),
                            model,
                            doc,
                            proof,
                            baseline,
                            base_proof,
                            no_table,
                            no_table_model,
                        },
                    );
                }
            });
        }
    });
    let mut rejected = BTreeMap::new();
    for r in reds() {
        let dump = compiler.dump(&scratch.0.join("red"), r.ace);
        dump.require_ok(r.id);
        assert!(
            dump.stdout.contains("messages([])."),
            "{} parser: {}",
            r.id,
            dump.stdout
        );
        rejected.insert(
            r.id.into(),
            compiler.run(
                &scratch.0.join("red"),
                r.id,
                "doc",
                r.ace,
                Some(&model::table(Some(KEY))),
            ),
        );
    }
    let precedence = compiler.run(
        &scratch.0.join("precedence"),
        "scope-existing",
        "doc",
        "If a patient should sleep then it is possible that every man waits.\n",
        Some(&model::table(Some(KEY))),
    );
    Results {
        docs: found.into_inner().unwrap(),
        reds: rejected,
        precedence,
    }
}
pub fn compiled(id: &str) -> &'static Compiled {
    &results().docs[id]
}
#[test]
fn a4_exclusion_products_and_proofs_every_shape() {
    let r = results();
    let mut checks = Checks::default();
    for item in r.docs.values().filter(|i| i.probe.row == "exclude") {
        for (mode, got, want) in [
            (
                "document",
                &item.doc,
                item.model.document(Some(KEY), item.probe.exclusion),
            ),
            (
                "proof",
                &item.proof,
                item.model.proof(Some(KEY), item.probe.exclusion),
            ),
        ] {
            checks.require(
                got.ok(),
                format!("{}/{mode}: {}", item.probe.id, got.describe()),
            );
            if got.ok() {
                checks.equal(&got.stdout, &want, &format!("{}/{mode}", item.probe.id));
            }
        }
    }
    checks.finish("A4 D3 exact product + proof");
}
#[test]
fn a5_scope_shape_rejects_precedence_and_sentence_order() {
    let r = results();
    let mut checks = Checks::default();
    for red in reds() {
        let got = &r.reds[red.id];
        let want = format!(
            "ace_to_pl_error(unsupported,scope_shape({},{})).\n",
            red.why, red.sentence
        );
        checks.require(
            got.rc == Some(1) && got.stdout.is_empty() && got.stderr == want,
            format!("{} expected rc=1 {want:?}; {}", red.id, got.describe()),
        );
    }
    checks.require(
        r.precedence.rc == Some(1)
            && r.precedence.stdout.is_empty()
            && r.precedence.stderr == "ace_to_pl_error(unsupported,operator_scoped_rule(can,1)).\n",
        format!(
            "existing reject before scope reason: {}",
            r.precedence.describe()
        ),
    );
    checks.finish("A5 D4 reasons + ordering");
}
#[test]
fn a6_deontic_guards_all_depths_variants_and_body_tail() {
    let mut checks = Checks::default();
    for item in results().docs.values().filter(|i| i.probe.row != "exclude") {
        checks.require(
            item.doc.ok(),
            format!("{}: {}", item.probe.id, item.doc.describe()),
        );
        if item.doc.ok() {
            checks.equal(
                &item.doc.stdout,
                &item.model.document(Some(KEY), false),
                &item.probe.id,
            );
        }
        checks.require(
            item.proof.ok(),
            format!("{} proof: {}", item.probe.id, item.proof.describe()),
        );
        if item.proof.ok() {
            checks.equal(
                &item.proof.stdout,
                &item.model.proof(Some(KEY), false),
                &format!("{} proof unchanged", item.probe.id),
            );
        }
    }
    checks.finish("A6 D5 exact guards + untouched clauses");
}
#[test]
fn a7_unscoped_and_tableless_same_input_model_controls() {
    let mut checks = Checks::default();
    for item in results().docs.values() {
        for (mode, got, want) in [
            (
                "unscoped",
                &item.baseline,
                model::without_block(&item.model.document(None, false)),
            ),
            (
                "tableless",
                &item.no_table,
                model::without_block(&item.no_table_model.document(None, false)),
            ),
            ("proof", &item.base_proof, item.model.proof(None, false)),
        ] {
            checks.require(
                got.ok(),
                format!("{}/{mode}: {}", item.probe.id, got.describe()),
            );
            if got.ok() {
                checks.equal(
                    &model::without_block(&got.stdout),
                    &want,
                    &format!("{}/{mode}", item.probe.id),
                );
            }
        }
    }
    checks.finish("A7 already-passing projection controls");
}
#[test]
fn a7_scoped_question_only_vocabulary_digest_changes() {
    let scratch = Scratch::new("questions");
    let compiler = Compiler::new(&scratch.0);
    let mut checks = Checks::default();
    for (i, ace) in [
        "Does a recommendation apply to a patient?\n",
        "Should a patient wait?\n",
        "Does a patient have less than 18 years?\n",
        "Can a recommendation apply?\n",
    ]
    .iter()
    .enumerate()
    {
        let id = format!("cs-query-{i}");
        let old = compiler.run(
            &scratch.0.join("old"),
            &id,
            "question",
            ace,
            Some(&model::table(None)),
        );
        let new = compiler.run(
            &scratch.0.join("new"),
            &id,
            "question",
            ace,
            Some(&model::table(Some(KEY))),
        );
        old.require_ok(&format!("{id} old question"));
        checks.require(new.ok(), format!("{id}: {}", new.describe()));
        if new.ok() {
            let before = format!("vocabulary(sha256({}))", quote(&sha(model::table(None))));
            let after = format!(
                "vocabulary(sha256({}))",
                quote(&sha(model::table(Some(KEY))))
            );
            assert_eq!(
                old.stdout.matches(&before).count(),
                1,
                "positive digest control"
            );
            checks.equal(&new.stdout, &old.stdout.replacen(&before, &after, 1), &id);
        }
    }
    checks.finish("A7 question custody only");
}
