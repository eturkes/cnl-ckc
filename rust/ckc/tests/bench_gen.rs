// Contract sb D4/P1: small family mechanisms + semantic-transform controls.
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn program() -> PathBuf {
    std::env::var_os("CKC_BENCH_GEN_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")))
}
fn run(args: &[String]) -> Output {
    Command::new(program())
        .args(args)
        .current_dir("/")
        .output()
        .unwrap()
}
fn quiet(out: &Output) {
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
fn refuses(out: &Output, needle: &str) {
    assert_eq!(
        out.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains(needle),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
struct Scratch(PathBuf);
impl Scratch {
    fn new(label: &str) -> Self {
        let p = root().join(format!(
            "rust/target/bench/gen-tests/{}-{label}",
            std::process::id()
        ));
        assert!(!p.exists(), "{}", p.display());
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
    fn generate(&self, family: &str, count: usize, name: &str, extra: &[String]) -> Output {
        let mut patient = root().join("tests/bench-gen/patient.pl.in");
        if family == "F2" {
            let bytes = fs::read_to_string(&patient)
                .unwrap()
                .replace("sb-p-adult", "sb-p-f2");
            patient = self.0.join("p-f2.pl.in");
            fs::write(&patient, bytes).unwrap();
        }
        let mut args = vec![
            "bench".into(),
            "gen".into(),
            family.into(),
            count.to_string(),
            "--root".into(),
            root().display().to_string(),
            "--documents".into(),
            root()
                .join("tests/bench-gen/documents")
                .display()
                .to_string(),
            "--patient".into(),
            patient.display().to_string(),
            "--queries".into(),
            root().join("tests/bench-gen/queries").display().to_string(),
            "--out".into(),
            self.0.join(name).display().to_string(),
        ];
        args.extend_from_slice(extra);
        run(&args)
    }
    fn check(&self, name: &str) -> Output {
        run(&[
            "bench".into(),
            "gen".into(),
            "check".into(),
            self.0.join(name).display().to_string(),
            "--root".into(),
            root().display().to_string(),
        ])
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn files(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    for item in fs::read_dir(dir).unwrap() {
        let p = item.unwrap().path();
        if p.is_dir() {
            for (name, bytes) in files(&p) {
                out.insert(PathBuf::from(p.file_name().unwrap()).join(name), bytes);
            }
        } else {
            out.insert(p.file_name().unwrap().into(), fs::read(&p).unwrap());
        }
    }
    out
}
fn shard(s: &Scratch, name: &str, index: usize) -> PathBuf {
    s.0.join(name).join(format!("copies/{index:04}"))
}
fn shard_text(s: &Scratch, name: &str, index: usize) -> String {
    String::from_utf8(
        files(&shard(s, name, index))
            .into_values()
            .flatten()
            .collect(),
    )
    .unwrap()
}
fn assembled_input_count(family: &str, count: usize) -> usize {
    let shared = matches!(family, "F3" | "MX");
    let base = count * if family == "F4" { 1 } else { 3 };
    let attached = if shared {
        5 + usize::from(count >= 2) * 2 + usize::from(count >= 3)
    } else if family == "F4" {
        2
    } else {
        0
    };
    let projections = if shared { count.min(6) - 1 } else { 0 };
    base + attached + projections + 1
}
fn replace(p: &Path, old: &str, new: &str) {
    let bytes = fs::read_to_string(p).unwrap();
    assert!(bytes.contains(old), "plant anchor {old}");
    fs::write(p, bytes.replacen(old, new, 1)).unwrap();
}

#[test]
fn frozen_queries_are_canonical_reproducible_and_patient_facing() {
    let queries = root().join("bench/sb/queries");
    assert_eq!(
        fs::read(queries.join("q-all.txt")).unwrap(),
        b"sb1-q-all 0\n"
    );
    assert_eq!(
        fs::read(queries.join("q-one.pl.in")).unwrap(),
        fs::read(
            root().join("guidelines/cdc-2022-opioid/queries/pl/category-a-recommendations.pl")
        )
        .unwrap()
    );
    assert_eq!(
        fs::read(queries.join("q-patient.ace")).unwrap(),
        b"Which opioid does a clinician prescribe to a patient?\n"
    );
    quiet(&run(&[
        "v1".into(),
        "check".into(),
        queries.join("q-patient.pl.in").display().to_string(),
    ]));
    let query = fs::read_to_string(queries.join("q-patient.pl.in")).unwrap();
    assert!(!query.contains("recommendation"));
    for noun in ["opioid", "clinician", "patient"] {
        assert!(query.contains(&format!(",{noun},countable)")));
    }
    let out = Command::new("bash")
        .arg(queries.join("regenerate"))
        .arg(root())
        .arg("--check")
        .env("CKC_BIN", program())
        .output()
        .unwrap();
    quiet(&out);
}

#[test]
fn q_all_marker_is_checked_before_writing_a_composition() {
    let s = Scratch::new("q-marker");
    let queries = s.0.join("queries");
    fs::create_dir(&queries).unwrap();
    for name in ["q-one.pl.in", "q-patient.pl.in"] {
        fs::copy(
            root().join("tests/bench-gen/queries").join(name),
            queries.join(name),
        )
        .unwrap();
    }
    fs::write(queries.join("q-all.txt.in"), b"sb1-q-all 1\n").unwrap();
    refuses(
        &s.generate(
            "F1a",
            1,
            "bad-marker",
            &["--queries".into(), queries.display().to_string()],
        ),
        "query q-all marker",
    );
    assert!(!s.0.join("bad-marker").exists());
}

#[test]
fn each_listed_source_is_one_canonical_document() {
    let s = Scratch::new("canonical-files");
    quiet(&s.generate("F3", 2, "f3", &assembly_args("F3")));
    let composition = fs::read_to_string(s.0.join("f3/composition.tsv")).unwrap();
    for path in composition.lines() {
        quiet(&run(&[
            "v1".into(),
            "check".into(),
            root().join(path).display().to_string(),
        ]));
    }
}

fn assembly_patient(family: &str) -> PathBuf {
    root()
        .join("bench/sb/fixtures/patients")
        .join(format!("p-{}.pl.in", family.to_ascii_lowercase()))
}
fn assembly_args(family: &str) -> Vec<String> {
    let mut args = vec![
        "--patient".into(),
        assembly_patient(family).display().to_string(),
        "--queries".into(),
        root().join("bench/sb/queries").display().to_string(),
    ];
    if family == "F4" {
        args.extend([
            "--documents".into(),
            root()
                .join("bench/sb/fixtures/bridges/f4-l3.pl.in")
                .display()
                .to_string(),
        ]);
    }
    args
}
fn manifest_values(dir: &Path) -> BTreeMap<String, String> {
    fs::read_to_string(dir.join("manifest.tsv"))
        .unwrap()
        .lines()
        .map(|l| {
            let (k, v) = l.split_once('\t').unwrap();
            (k.to_owned(), v.to_owned())
        })
        .collect()
}
fn assembled_family(family: &str) {
    let s = Scratch::new(&format!("assembly-{family}"));
    let declared = if matches!(family, "F3" | "MX") {
        6
    } else if family == "F4" {
        3
    } else {
        1
    };
    for count in [1, 2] {
        let name = format!("g{count}");
        quiet(&s.generate(family, count, &name, &assembly_args(family)));
        quiet(&s.check(&name));
        let dir = s.0.join(&name);
        let m = manifest_values(&dir);
        assert_eq!(m["declared_k"], declared.to_string());
        assert_eq!(m["smoke_only"], (count < declared).to_string());
        assert_eq!(
            m["active_shards"],
            if matches!(family, "F3" | "MX") {
                count
            } else {
                1
            }
            .to_string()
        );
        assert_eq!(
            fs::read_to_string(dir.join("composition.tsv"))
                .unwrap()
                .lines()
                .count(),
            assembled_input_count(family, count)
        );
        let roles = fs::read_to_string(dir.join("shards.tsv")).unwrap();
        assert_eq!(roles.lines().count(), count + 1);
        assert!(
            roles
                .lines()
                .nth(1)
                .unwrap()
                .starts_with("1\tactive-source\tCDC\t")
        );
        let first = shard_text(&s, &name, 1);
        if matches!(family, "F3" | "MX") {
            for id in [
                "sb-f3-b1-classifier",
                "sb-f3-b2-classifier",
                "sb-f3-b3-classifier",
                "sb-f3-b4-classifier",
                "sb-f3-b3-reader",
            ] {
                assert!(first.contains(id), "{family}: {id}");
            }
            assert!(m["active_transform_notice"].contains("not compilations"));
            assert_eq!(
                m["shared_bridge_nouns"],
                "acute-pain,chronic-pain,opioid-use-disorder"
            );
            if count == 2 {
                let second = shard_text(&s, &name, 2);
                for id in ["sb-f3-b1-reader", "sb-f3-b2-reader"] {
                    assert!(second.contains(id));
                }
                assert!(second.contains("sb0002-alpha"));
                assert!(
                    roles
                        .lines()
                        .nth(2)
                        .unwrap()
                        .starts_with("2\tactive-transform\tVA-opioids\t")
                );
            }
        } else if family == "F4" {
            for id in ["sb-f4-l1", "sb-f4-l2", "sb-f4-l3"] {
                assert!(first.contains(id));
            }
            assert_eq!(m["k"], "3");
        }
        if !matches!(family, "F3" | "MX") {
            assert_eq!(
                fs::read(dir.join("patient.pl")).unwrap(),
                fs::read(assembly_patient(family)).unwrap()
            );
            if count == 2 {
                assert!(roles.lines().nth(2).unwrap().starts_with("2\tpadding\t"));
            }
        }
    }
}
#[test]
fn assembled_f3() {
    assembled_family("F3");
}
#[test]
fn assembled_f4() {
    assembled_family("F4");
}
#[test]
fn assembled_f5a() {
    assembled_family("F5a");
}
#[test]
fn assembled_f5b() {
    assembled_family("F5b");
}
#[test]
fn assembled_f5c() {
    assembled_family("F5c");
}
#[test]
fn assembled_mixed() {
    assembled_family("MX");
}
#[test]
fn every_nested_point_keeps_declared_sources_and_padding_counts() {
    let s = Scratch::new("nested");
    for family in ["F3", "F4", "F5a", "F5b", "F5c", "MX"] {
        let declared = if matches!(family, "F3" | "MX") {
            6
        } else if family == "F4" {
            3
        } else {
            1
        };
        for count in [4, 10, 100] {
            let name = format!("{family}-{count}");
            quiet(&s.generate(family, count, &name, &assembly_args(family)));
            quiet(&s.check(&name));
            let dir = s.0.join(&name);
            let m = manifest_values(&dir);
            assert_eq!(m["declared_k"], declared.to_string());
            assert_eq!(m["smoke_only"], (count < declared).to_string());
            assert_eq!(
                fs::read_to_string(dir.join("composition.tsv"))
                    .unwrap()
                    .lines()
                    .count(),
                assembled_input_count(family, count)
            );
            let roles = fs::read_to_string(dir.join("shards.tsv")).unwrap();
            let active = if matches!(family, "F3" | "MX") {
                count.min(6)
            } else {
                1
            };
            assert_eq!(
                roles
                    .lines()
                    .skip(1)
                    .filter(|l| l.split('\t').nth(1) == Some("padding"))
                    .count(),
                count - active
            );
            if matches!(family, "F3" | "MX") && count >= 10 {
                for (index, source) in [(3, "VA-SUD"), (4, "TIP63"), (5, "CKD"), (6, "T2DM")] {
                    assert!(
                        roles
                            .lines()
                            .nth(index)
                            .unwrap()
                            .starts_with(&format!("{index}\tactive-transform\t{source}\t"))
                    );
                }
                assert!(shard_text(&s, &name, 3).contains("sb-f3-b4-reader"));
            }
            fs::remove_dir_all(dir).unwrap();
        }
    }
}
#[test]
fn a_patient_profile_mismatch_is_refused() {
    let s = Scratch::new("wrong-profile");
    for (family, patient) in [
        ("F1a", "p-f2.pl.in"),
        ("F2", "p-adult.pl.in"),
        ("F3", "p-adult.pl.in"),
        ("F5c", "p-f5b.pl.in"),
    ] {
        refuses(
            &s.generate(
                family,
                1,
                family,
                &[
                    "--patient".into(),
                    root()
                        .join("bench/sb/fixtures/patients")
                        .join(patient)
                        .display()
                        .to_string(),
                ],
            ),
            "patient profile",
        );
        assert!(!s.0.join(family).exists());
    }
}

#[test]
fn family_composition_is_canonical_and_keeps_schema_values() {
    let s = Scratch::new("families");
    for family in ["F1a", "F1b", "F2"] {
        quiet(&s.generate(family, 2, family, &[]));
        quiet(&s.check(family));
        let out = s.0.join(family);
        let lines = fs::read_to_string(out.join("composition.tsv")).unwrap();
        let paths: Vec<_> = lines.lines().filter(|l| !l.starts_with('#')).collect();
        assert_eq!(paths.len(), if family == "F2" { 9 } else { 7 });
        assert!(paths.iter().all(|p| !Path::new(p).is_absolute()));
        assert!(root().join(paths.last().unwrap()).is_file());
        for path in &paths {
            quiet(&run(&[
                "v1".into(),
                "check".into(),
                root().join(path).display().to_string(),
            ]));
        }
        let copy = shard_text(&s, family, 2);
        for kept in [
            "guideline_schema_version(2).",
            "na,exactly,45",
            "na,exactly,rational(3,2)",
            ",strength,strong,",
            ",manner,carefully)",
            ",for,",
            "),year).",
            ",after,geq,4,week)",
            ",exactly,2,week,after,",
            ",none)",
            "20221229",
            "https://example.org/",
        ] {
            assert!(copy.contains(kept), "{family}: {kept}");
        }
        for name in ["q-all.txt", "q-one.pl", "q-patient.pl"] {
            assert!(out.join("queries").join(name).is_file());
        }
        let manifest = fs::read_to_string(out.join("manifest.tsv")).unwrap();
        for key in [
            "family\t",
            "scale\t2\n",
            "G\t2\n",
            "transform_version\t",
            "generator_commit\t",
            "seed\t0\n",
            "patient_profile\t",
            "input_sha256:",
            "annex_param:",
            "composition_sha256\t",
        ] {
            assert!(manifest.contains(key), "{key}");
        }
    }
}
#[test]
fn namespace_is_injective_and_family_shared_sets_differ() {
    let s = Scratch::new("namespaces");
    for f in ["F1a", "F1b"] {
        quiet(&s.generate(f, 2, f, &[]));
    }
    let shared = shard_text(&s, "F1a", 2);
    let private = shard_text(&s, "F1b", 2);
    assert!(shared.contains(",patient,countable)"));
    assert!(shared.contains(",clinician,countable)"));
    assert!(!private.contains(",patient,countable)"));
    assert!(!private.contains(",clinician,countable)"));
    assert!(shared.contains("sb0002__private-state"));
    assert!(shared.contains("sb0002-scopekey"));
    assert!(shared.contains("sb0002-alpha"));
    assert!(shared.contains("sb0002__act"));
    assert!(shared.contains("sb0002__excluded"));
    assert!(shared.contains("box(1)"));
    assert!(shared.contains("ref(2)"));
    assert!(shared.contains("sb0002__\\u0001"));
    assert!(shared.contains("sb0002__back\\\\slash"));
    assert!(shared.contains("sb0002__quote\\'s"));
}
#[test]
fn f2_intake_keeps_shared_ground_identities_and_private_spelling() {
    let s = Scratch::new("intake");
    quiet(&s.generate("F2", 2, "two", &[]));
    let mut patient = fs::read_to_string(s.0.join("two/patient.pl")).unwrap();
    for bytes in files(&s.0.join("two/intakes")).into_values() {
        patient.push_str(std::str::from_utf8(&bytes).unwrap());
    }
    assert_eq!(manifest_values(&s.0.join("two"))["patient_documents"], "3");
    for noun in [
        "private-state",
        "sb0001__private-state",
        "sb0002__private-state",
    ] {
        assert!(patient.contains(&format!("guideline_entity(actual,'$guideline_id'(product,'sb-p-f2',2,ref(1),[]),'{noun}',countable).")));
    }
    assert!(!patient.contains("sb0001-sb-p-f2"));
    let q = fs::read_to_string(s.0.join("two/queries/q-one.pl")).unwrap();
    assert!(q.contains("sb0001__recommendation"));
    assert_eq!(
        fs::read(s.0.join("two/queries/q-patient.pl")).unwrap(),
        fs::read(root().join("tests/bench-gen/queries/q-patient.pl.in")).unwrap()
    );
}
#[test]
fn repeated_generation_is_byte_identical() {
    let s = Scratch::new("determinism");
    quiet(&s.generate("F1a", 2, "one", &[]));
    let first = files(&s.0.join("one"));
    fs::remove_dir_all(s.0.join("one")).unwrap();
    quiet(&s.generate("F1a", 2, "one", &[]));
    assert_eq!(first, files(&s.0.join("one")));
}
#[test]
fn removed_annex_parameter_refuses_before_writing() {
    let s = Scratch::new("annex");
    let annex = s.0.join("annex");
    fs::create_dir(&annex).unwrap();
    let src = root().join("bench/sb/annex");
    let families = fs::read_to_string(src.join("families.tsv")).unwrap();
    let families = families
        .lines()
        .map(|line| {
            if !line.starts_with("F1a\t") {
                return line.to_owned();
            }
            let (fields, ids) = line.rsplit_once('\t').unwrap();
            let mut ids: Vec<_> = ids.split(',').map(str::trim).collect();
            if !ids.contains(&"P-ADULT-AGE") {
                ids.push("P-ADULT-AGE");
            }
            format!("{fields}\t{}", ids.join(", "))
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    fs::write(annex.join("families.tsv"), families).unwrap();
    fs::create_dir(annex.join("excerpts")).unwrap();
    for item in fs::read_dir(src.join("excerpts")).unwrap() {
        let p = item.unwrap().path();
        fs::copy(&p, annex.join("excerpts").join(p.file_name().unwrap())).unwrap();
    }
    let sources = fs::read_to_string(src.join("sources.tsv")).unwrap();
    fs::write(annex.join("sources.tsv"), &sources).unwrap();
    let extra = ["--annex".into(), annex.display().to_string()];
    quiet(&s.generate("F1a", 2, "positive-annex", &extra));
    for (i, id) in ["F1A-G", "P-ADULT-AGE"].iter().enumerate() {
        let prefix = format!("{id}\t");
        assert_eq!(
            sources.lines().filter(|l| l.starts_with(&prefix)).count(),
            1
        );
        fs::write(
            annex.join("sources.tsv"),
            sources
                .lines()
                .filter(|l| !l.starts_with(&prefix))
                .collect::<Vec<_>>()
                .join("\n")
                + "\n",
        )
        .unwrap();
        let name = format!("bad-{i}");
        refuses(
            &s.generate("F1a", 2, &name, &extra),
            &format!("annex missing parameter {id}"),
        );
        assert!(!s.0.join(name).exists());
    }
}
#[test]
fn semantic_transform_plants_are_rejected() {
    let s = Scratch::new("plants");
    for name in ["collision", "condition", "number"] {
        quiet(&s.generate("F1a", 2, name, &[]));
    }
    replace(
        &shard(&s, "collision", 2).join("sb0002-alpha.pl"),
        "sb0002__private-state",
        "sb0002__recommendation",
    );
    refuses(&s.check("collision"), "transform");
    replace(
        &shard(&s, "number", 2).join("sb0002-alpha.pl"),
        "na,exactly,45",
        "na,exactly,46",
    );
    refuses(&s.check("number"), "transform");
    replace(
        &shard(&s, "condition", 2).join("sb0002-alpha.pl"),
        ", \\+ guideline_exclusion('sb0002-scopekey')",
        "",
    );
    refuses(&s.check("condition"), "transform");
}
#[test]
fn f2_cross_copy_bridge_and_naf_read_are_refused() {
    let s = Scratch::new("isolation");
    let prefix = fs::read_to_string(root().join("tests/bench-gen/prefix.in")).unwrap();
    for (name, stage, body) in [
        (
            "bridge",
            "positive",
            "guideline_cardinality(actual,A,na,eq,2) :- guideline_entity(actual,A,patient,countable).\nguideline_operator(actual,'$guideline_id'(context,plant,1,box(1),[A]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,2).\n",
        ),
        (
            "naf",
            "NAF",
            "guideline_property(actual,A,private,pos) :- guideline_entity(actual,A,patient,countable).\nguideline_operator(actual,'$guideline_id'(context,plant,1,box(1),[A]),should) :- guideline_entity(actual,A,patient,countable), \\+ guideline_property(actual,A,_,pos).\n",
        ),
        (
            "variable-head",
            "positive",
            "guideline_entity(actual,'$guideline_id'(product,plant,1,ref(1),[A]),B,countable) :- guideline_entity(actual,A,B,countable).\n",
        ),
        (
            "head-range",
            "positive",
            "guideline_event(actual,A,B) :- guideline_qualifier(actual,A,manner,B).\n",
        ),
    ] {
        let docs = s.0.join(name);
        fs::create_dir(&docs).unwrap();
        fs::write(docs.join("plant.pl.in"), format!("% plant.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{prefix}guideline_schema_version(2).\nguideline_document(plant,ace_sha256({}),ulex(none),vocabulary(none)).\n% S1: Isolation plant.\n{body}", "a".repeat(64))).unwrap();
        quiet(&run(&[
            "v1".into(),
            "check".into(),
            docs.join("plant.pl.in").display().to_string(),
        ]));
        let mut extra = vec!["--documents".into(), docs.display().to_string()];
        if name == "head-range" {
            let patient = s.0.join("patient-range.pl.in");
            let source = fs::read_to_string(root().join("tests/bench-gen/patient.pl.in"))
                .unwrap()
                .replace("sb-p-adult", "sb-p-f2");
            fs::write(&patient, source + "% S3: A read-only word enters a lexical-variable head.\nguideline_qualifier(actual,'$guideline_id'(product,'sb-p-f2',3,ref(1),[]),manner,carefully).\n").unwrap();
            quiet(&run(&[
                "v1".into(),
                "check".into(),
                patient.display().to_string(),
            ]));
            extra.extend(["--patient".into(), patient.display().to_string()]);
        }
        refuses(
            &s.generate("F2", 2, &format!("out-{name}"), &extra),
            &format!("ckc: bench gen: isolation {stage}"),
        );
    }
}
#[test]
fn uncalibrated_scale_and_missing_query_are_refused() {
    let s = Scratch::new("bad-inputs");
    refuses(&s.generate("F1a", 3, "scale", &[]), "scale");
    let missing = s.0.join("missing");
    fs::create_dir(&missing).unwrap();
    refuses(
        &s.generate(
            "F1a",
            2,
            "queries",
            &["--queries".into(), missing.display().to_string()],
        ),
        "query",
    );
    refuses(
        &s.generate(
            "F1a",
            2,
            "patient-rule",
            &[
                "--patient".into(),
                root()
                    .join("tests/bench-gen/documents/alpha.pl.in")
                    .display()
                    .to_string(),
            ],
        ),
        "patient must contain ground read-only facts",
    );
    refuses(
        &s.generate(
            "F1a",
            2,
            "patient-collision",
            &[
                "--patient".into(),
                root()
                    .join("tests/bench-gen/documents/gamma.pl.in")
                    .display()
                    .to_string(),
            ],
        ),
        "transform duplicate document identity gamma",
    );
}
