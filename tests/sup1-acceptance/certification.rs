use super::model::*;
use super::support::*;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        assert!(!entry.file_type().unwrap().is_symlink());
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}
struct Compiler {
    work: Cases,
    stage: PathBuf,
}
impl Compiler {
    fn new() -> Self {
        let work = Cases::new("stage");
        let version = command(Path::new("swipl"), &work.dir, &["--version"], "");
        assert_eq!(version.rc, 0, "{}", version.summary());
        assert!(version.stdout.contains("version 10.0.2 "));
        let stage = work.dir.join("ape");
        copy_tree(&root().join("vendor/ape"), &stage);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let goal = format!(
            "working_directory(_,{}),[fit_to_plp],halt.",
            atom(stage.join("prolog/parser").to_str().unwrap())
        );
        let built = command(
            Path::new("swipl"),
            &work.dir,
            &[
                "-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
            ],
            "",
        );
        assert_eq!(built.rc, 0, "APE stage: {}", built.summary());
        let goal = format!(
            "style_check(-discontiguous),qcompile({}),halt.",
            atom(
                stage
                    .join("prolog/lexicon/clex_lexicon.pl")
                    .to_str()
                    .unwrap()
            )
        );
        assert_eq!(
            command(
                Path::new("swipl"),
                &work.dir,
                &[
                    "-q", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)"
                ],
                ""
            ),
            Out::ok("")
        );
        Self { work, stage }
    }
    fn run(&self, scene: &Scene, mode: &str) -> Out {
        let table = scene.table();
        self.work
            .write("vocabulary.tsv", table.as_deref().unwrap_or(""));
        let tab_path = self.work.dir.join("vocabulary.tsv");
        let script = if mode == "dump" {
            root().join("rust/ckc/prolog/drs_dump.pl")
        } else {
            self.stage.join("prolog/ace_to_pl.pl")
        };
        let mut args = vec![
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            script.to_str().unwrap(),
            "-g",
            "main",
            "-t",
            "halt(9)",
            "--",
            self.stage.to_str().unwrap(),
        ];
        if mode != "dump" {
            args.extend([
                scene.id.as_str(),
                if table.is_some() {
                    tab_path.to_str().unwrap()
                } else {
                    "-"
                },
                "-",
            ]);
            if mode == "proof" {
                args.push("proof");
            }
        }
        command(Path::new("swipl"), &self.work.dir, &args, &scene.ace)
    }
}
struct Observation {
    scene: Scene,
    dump: Out,
    compiled: Out,
    payload: Out,
    baseline: Scene,
    baseline_compiled: Out,
    baseline_payload: Out,
}
fn observations() -> &'static [Observation] {
    static RESULTS: OnceLock<Vec<Observation>> = OnceLock::new();
    RESULTS.get_or_init(|| {
        let compiler = Compiler::new();
        goldens()
            .into_iter()
            .map(|scene| {
                let dump = compiler.run(&scene, "dump");
                let compiled = compiler.run(&scene, "compile");
                let payload = compiler.run(&scene, "proof");
                let mut baseline = scene.clone();
                baseline.date = None;
                let baseline_compiled = compiler.run(&baseline, "compile");
                let baseline_payload = compiler.run(&baseline, "proof");
                Observation {
                    scene,
                    dump,
                    compiled,
                    payload,
                    baseline,
                    baseline_compiled,
                    baseline_payload,
                }
            })
            .collect()
    })
}
fn certify_one(c: &Cases, scene: &Scene, dump: &str, pl: &str) -> Out {
    c.write("source.ace", &scene.ace);
    c.write("source.dump", dump);
    c.write("cert.pl", pl);
    let mut args = vec![
        "certify-one",
        "doc",
        scene.id.as_str(),
        "source.ace",
        "-",
        "source.dump",
        "cert.pl",
    ];
    if let Some(table) = scene.table() {
        c.write("vocabulary.tsv", &table);
        args.push("vocabulary.tsv");
    }
    c.run(&args)
}
#[test]
fn p5_model_baselines() {
    let mut c = Cases::new("P5-model-characterization");
    let legacy = legacy_binary(&c);
    for observation in observations() {
        let scene = &observation.baseline;
        let expected = scene.member();
        let pl = if legacy {
            legacy_block(&expected.pl)
        } else {
            expected.pl
        };
        c.compare(
            &format!("{}/without-row/fork=model", scene.id),
            observation.baseline_compiled.clone(),
            Out::ok(&pl),
        );
        c.compare(
            &format!("{}/without-row/proof=model", scene.id),
            observation.baseline_payload.clone(),
            Out::ok(&expected.payload),
        );
        c.compare(
            &format!("{}/without-row/certify-one=model", scene.id),
            certify_one(&c, scene, &observation.dump.stdout, &pl),
            Out::ok(expected.payload),
        );
    }
    let scene = &observations()[0].baseline;
    let member = scene.member();
    let pl = if legacy {
        legacy_block(&member.pl)
    } else {
        member.pl
    };
    let native = Cases::new("native-certify-control");
    install_certify_root(&native, scene, &pl);
    let output = native.run(&["certify", "sup1"]);
    assert_eq!(
        (output.rc, output.stderr.as_str()),
        (0, ""),
        "native guideline control: {}",
        output.summary()
    );
    c.finish();
}
#[test]
fn p5_goldens() {
    let mut c = Cases::new("P5-goldens");
    for observation in observations() {
        let scene = &observation.scene;
        let expected = scene.member();
        assert_eq!(
            observation.dump.rc,
            0,
            "{}/APE: {}",
            scene.id,
            observation.dump.summary()
        );
        assert!(
            observation.dump.stdout.contains("messages([])."),
            "{}",
            observation.dump.summary()
        );
        c.check(&format!("{}/model-codec", scene.id), &expected.pl);
        c.compare(
            &format!("{}/fork-product=model", scene.id),
            observation.compiled.clone(),
            Out::ok(&expected.pl),
        );
        c.compare(
            &format!("{}/fork-witness=model", scene.id),
            observation.payload.clone(),
            Out::ok(&expected.payload),
        );
        c.compare(
            &format!("{}/certify-one=model", scene.id),
            certify_one(&c, scene, &observation.dump.stdout, &expected.pl),
            Out::ok(&expected.payload),
        );
        let record_count = expected
            .pl
            .lines()
            .filter(|line| line.starts_with("guideline_supersession("))
            .count();
        assert_eq!(record_count, scene.groups.len());
        assert!(!expected.payload.contains("guideline_supersession("));
        if scene.id == "year-boundary" {
            assert!(
                expected
                    .payload
                    .contains("guideline_evaluation_date(20230100)")
            );
        }
    }
    c.finish();
}
struct Mutant {
    family: &'static str,
    label: String,
    scene: Scene,
    dump: String,
    pl: String,
    sentence: usize,
}
fn mutation(
    observation: &Observation,
    family: &'static str,
    label: String,
    bundles: Vec<Vec<String>>,
    sentence: usize,
) -> Mutant {
    let scene = &observation.scene;
    let pl = document_text(&scene.id, &scene.ace, scene.table().as_deref(), &bundles);
    assert_ne!(pl, scene.member().pl, "{label}: changed bytes");
    Mutant {
        family,
        label,
        scene: scene.clone(),
        dump: observation.dump.stdout.clone(),
        pl,
        sentence,
    }
}
fn mutants() -> Vec<Mutant> {
    let mut out = Vec::new();
    for o in observations() {
        let original = o.scene.bundles();
        for (sentence, clauses) in original.iter().enumerate() {
            let last = clauses.len() - 1;
            assert!(clauses[last].starts_with("guideline_supersession("));
            for family in [
                "record-drop",
                "record-move",
                "record-date",
                "record-citation",
                "record-url",
                "record-sentence",
                "record-document",
            ] {
                let mut changed = original.clone();
                let target = &mut changed[sentence];
                match family {
                    "record-drop" => {
                        target.pop();
                    }
                    "record-move" => {
                        let r = target.pop().unwrap();
                        target.insert(0, r);
                    }
                    "record-date" => {
                        target[last] = target[last].replace(
                            &format!(",{})", o.scene.date.unwrap()),
                            &format!(",{})", o.scene.date.unwrap() + 1),
                        );
                    }
                    "record-citation" => {
                        target[last] = target[last].replace(CITATION, "Another law");
                    }
                    "record-url" => {
                        target[last] = target[last].replace(URL, "https://example.org/other");
                    }
                    "record-sentence" => {
                        target[last] = target[last].replacen(
                            &format!(",{},source(", sentence + 1),
                            &format!(",{},source(", sentence + 2),
                            1,
                        );
                    }
                    "record-document" => {
                        target[last] = target[last].replacen(
                            &format!("guideline_supersession({},", atom(&o.scene.id)),
                            "guideline_supersession(other,",
                            1,
                        );
                    }
                    _ => unreachable!(),
                }
                out.push(mutation(
                    o,
                    family,
                    format!("{}/S{}/{family}", o.scene.id, sentence + 1),
                    changed,
                    sentence + 1,
                ));
            }
            for (at, clause) in clauses[..last].iter().enumerate() {
                let Some((head, body)) = clause.split_once(" :- ") else {
                    continue;
                };
                let goals = parts(body);
                if let Some(date_index) = goals
                    .iter()
                    .position(|g| g.starts_with("guideline_evaluation_date("))
                {
                    for family in [
                        "guard-drop",
                        "date-goal-drop",
                        "comparison-drop",
                        "guard-date",
                    ] {
                        let mut modified = goals.clone();
                        match family {
                            "guard-drop" => {
                                modified.drain(date_index..date_index + 2);
                            }
                            "date-goal-drop" => {
                                modified.remove(date_index);
                            }
                            "comparison-drop" => {
                                modified.remove(date_index + 1);
                            }
                            "guard-date" => {
                                modified[date_index + 1] = modified[date_index + 1].replace(
                                    &format!(",{})", o.scene.date.unwrap()),
                                    &format!(",{})", o.scene.date.unwrap() + 1),
                                );
                            }
                            _ => unreachable!(),
                        }
                        let mut changed = original.clone();
                        changed[sentence][at] =
                            canonical(&format!("{head} :- {}", modified.join(", ")));
                        out.push(mutation(
                            o,
                            family,
                            format!("{}/S{}/clause={at}/{family}", o.scene.id, sentence + 1),
                            changed,
                            sentence + 1,
                        ));
                    }
                    if goals.last().unwrap() == "\\+ guideline_exclusion(alpha)" {
                        let mut modified = goals.clone();
                        let guard = modified.pop().unwrap();
                        modified.insert(date_index, guard);
                        let mut changed = original.clone();
                        changed[sentence][at] =
                            canonical(&format!("{head} :- {}", modified.join(", ")));
                        out.push(mutation(
                            o,
                            "guard-after-scope",
                            format!("{}/guard-after-scope", o.scene.id),
                            changed,
                            sentence + 1,
                        ));
                    }
                } else {
                    let mut modified = goals.clone();
                    modified.extend(date_goals("Z", o.scene.date.unwrap()));
                    let mut changed = original.clone();
                    changed[sentence][at] =
                        canonical(&format!("{head} :- {}", modified.join(", ")));
                    out.push(mutation(
                        o,
                        "guard-on-nondeontic",
                        format!("{}/clause={at}/guard-on-nondeontic", o.scene.id),
                        changed,
                        sentence + 1,
                    ));
                }
            }
        }
        if original.len() > 1 {
            let mut changed = original.clone();
            let record = changed[0].pop().unwrap();
            changed[1].push(record);
            out.push(mutation(
                o,
                "record-other-sentence",
                format!("{}/record-other-sentence", o.scene.id),
                changed,
                1,
            ));
        }
    }
    let o = &observations()[0];
    let mut scene = o.scene.clone();
    scene.date = None;
    let mut bundles = scene.bundles();
    bundles[0].push(record(&scene.id, 1, DATE));
    let pl = document_text(&scene.id, &scene.ace, scene.table().as_deref(), &bundles);
    out.push(Mutant {
        family: "record-unsuperseded",
        label: "record-unsuperseded".into(),
        scene,
        dump: o.dump.stdout.clone(),
        pl,
        sentence: 1,
    });
    out
}
fn install_certify_root(c: &Cases, scene: &Scene, pl: &str) {
    use std::os::unix::fs::symlink;
    symlink(root().join("vendor"), c.dir.join("vendor")).unwrap();
    fs::create_dir_all(c.dir.join("rust/ckc")).unwrap();
    symlink(
        root().join("rust/ckc/prolog"),
        c.dir.join("rust/ckc/prolog"),
    )
    .unwrap();
    c.write(&format!("guidelines/sup1/ace/{}.ace", scene.id), &scene.ace);
    // The guideline command reads its lexicon file; certify-one above uses no lexicon.
    let pinned = pl.replacen("ulex(none)", &format!("ulex({})", digest(Some(""))), 1);
    c.write(&format!("guidelines/sup1/pl/{}.pl", scene.id), &pinned);
    c.write("guidelines/sup1/lexicon.ulex", "");
    if let Some(table) = scene.table() {
        c.write("guidelines/sup1/vocabulary.tsv", &table);
    }
    fs::create_dir_all(c.dir.join("guidelines/sup1/queries")).unwrap();
}
#[test]
fn p5_document_mutants() {
    let mut c = Cases::new("P5-mutants");
    let mut native_families = BTreeSet::new();
    let all = mutants();
    for m in &all {
        c.check(&format!("{}/codec", m.label), &m.pl);
        let expected = Out::err(
            1,
            format!("ckc: certify: {}: clauses({}).\n", m.scene.id, m.sentence),
        );
        c.compare(
            &format!("{}/certify-one", m.label),
            certify_one(&c, &m.scene, &m.dump, &m.pl),
            expected.clone(),
        );
        if native_families.insert(m.family) {
            let work = Cases::new("native-certify");
            install_certify_root(&work, &m.scene, &m.pl);
            c.compare(
                &format!("{}/ckc-certify", m.label),
                work.run(&["certify", "sup1"]),
                expected,
            );
        }
    }
    assert_eq!(
        native_families.len(),
        15,
        "every mutant family reaches the guideline command"
    );
    println!(
        "SUP1 P5 mutant documents={} native families={}",
        all.len(),
        native_families.len()
    );
    c.finish();
}
