// SB1 source fixtures: pure ACE emissions certify; the one rational intake is
// explicit. Full-CDC probes pin the F4 endpoint and F5 co-prescription joins.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

#[path = "../../../tests/singletons/anonymize.rs"]
mod spelling;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn fixtures() -> PathBuf {
    root().join("bench/sb/fixtures")
}

fn program() -> PathBuf {
    std::env::var_os("CKC_SB_FIXTURE_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")))
}

struct Scratch(PathBuf);
impl Scratch {
    fn new(name: &str) -> Self {
        let dir = root().join(format!(
            "rust/target/bench-fixtures/{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }

    fn file(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, bytes).unwrap();
        path
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn checked(out: Output) -> Vec<u8> {
    assert!(
        out.status.success() && out.stderr.is_empty(),
        "rc {:?}: {}{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}

fn regenerate(dir: &Path) -> Output {
    Command::new("bash")
        .arg(fixtures().join("regenerate"))
        .arg(root())
        .arg(program())
        .arg("--check")
        .env("CKC_SB_FIXTURE_DIR", dir)
        .output()
        .unwrap()
}

fn canonical(bytes: &[u8]) {
    assert!(matches!(
        ckc_kernel::contract::v1_check(bytes),
        ckc_kernel::EV1Verdict::Ok
    ));
    let read = String::from_utf8(ckc_kernel::contract::v1_read(bytes)).unwrap();
    let rendered = read
        .split_inclusive('\n')
        .map(|line| {
            if line.starts_with('%') {
                line.to_owned()
            } else {
                spelling::anonymize(line)
            }
        })
        .collect::<String>();
    assert_eq!(rendered.as_bytes(), bytes, "canonical reading/spelling");
}

#[test]
fn fixture_value_references_resolve_to_the_annex() {
    let manifest = fs::read_to_string(fixtures().join("cases.tsv")).unwrap();
    let ids = manifest
        .lines()
        .skip(1)
        .map(|line| line.split('\t').nth(1).unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ids.len(), 19);
    let sources = fs::read_to_string(root().join("bench/sb/annex/sources.tsv")).unwrap();
    let parameters = sources
        .lines()
        .skip(1)
        .map(|line| line.split('\t').next().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let values = fs::read_to_string(fixtures().join("values.tsv")).unwrap();
    assert_eq!(values.lines().count(), 50);
    for line in values.lines().skip(1) {
        let fields: Vec<&str> = line.split('\t').collect();
        assert_eq!(fields.len(), 7, "{line}");
        assert!(fields[0].split(',').all(|id| ids.contains(id)), "{line}");
        assert!(
            fields[4].split(';').all(|id| parameters.contains(id)),
            "{line}"
        );
        assert!(
            [
                "assumption",
                "observed",
                "blocked",
                "authoring_interpretation",
                "unconfirmed"
            ]
            .contains(&fields[5]),
            "{line}"
        );
    }
}

#[test]
fn fixture_lexemes_keep_distinct_surfaces() {
    fn surface(line: &str) -> Option<String> {
        let (_, arguments) = line.split_once('(')?;
        let (first, _) = arguments.split_once(',')?;
        Some(first.trim().trim_matches('\'').to_owned())
    }
    let base = fs::read_to_string(root().join("guidelines/cdc-2022-opioid/lexicon.ulex")).unwrap()
        + &fs::read_to_string(root().join("vendor/clex/clex_lexicon.pl")).unwrap();
    let existing = base
        .lines()
        .filter_map(surface)
        .collect::<std::collections::BTreeSet<_>>();
    let additions = fs::read_to_string(fixtures().join("lexicon.ulex")).unwrap();
    let mut seen = std::collections::BTreeSet::new();
    for line in additions
        .lines()
        .filter(|line| !line.starts_with('%') && !line.is_empty())
    {
        let word = surface(line).unwrap();
        assert!(
            !existing.contains(&word),
            "base/Clex surface shadow: {line}"
        );
        // A noun's singular/plural may share a surface only through an explicit ruling.
        assert!(seen.insert(word), "fixture surface shadow: {line}");
    }
}

#[test]
fn p_adult_is_the_unchanged_consumer_seed_with_a_compiler_header() {
    let consumer = fs::read_to_string(root().join("tests/consumer/adult.pl.in")).unwrap();
    let sentences = consumer
        .lines()
        .filter_map(|line| line.strip_prefix("%   "))
        .collect::<Vec<_>>();
    assert_eq!(sentences.len(), 4);
    let ace = fs::read_to_string(fixtures().join("patients/p-adult.ace")).unwrap();
    assert_eq!(ace, sentences.join("\n") + "\n");
    let product = fs::read_to_string(fixtures().join("patients/p-adult.pl.in")).unwrap();
    assert!(product.starts_with(
        "% sb-p-adult.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n"
    ));
}

#[test]
fn every_ace_recompiles_and_certifies_byte_identically() {
    let manifest = fs::read_to_string(fixtures().join("cases.tsv")).unwrap();
    let rows: Vec<Vec<&str>> = manifest
        .lines()
        .skip(1)
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 19);
    for row in rows {
        assert_eq!(row.len(), 5);
        let bytes = fs::read(fixtures().join(format!("{}.pl.in", row[2]))).unwrap();
        canonical(&bytes);
        if row[0] == "patient" {
            assert!(
                !String::from_utf8(bytes)
                    .unwrap()
                    .lines()
                    .any(|line| line.contains(" :- ")),
                "{}: patient worlds contain root facts only",
                row[1]
            );
        }
    }
    let out = checked(regenerate(&fixtures()));
    assert_eq!(String::from_utf8(out).unwrap().lines().count(), 19);
}

#[test]
fn regeneration_rejects_a_changed_medication_number() {
    let s = Scratch::new("number-plant");
    let copy = s.0.join("fixtures");
    assert!(
        Command::new("cp")
            .arg("-a")
            .arg(fixtures())
            .arg(&copy)
            .status()
            .unwrap()
            .success()
    );
    let path = copy.join("patients/p-f5b.pl.in");
    let text = fs::read_to_string(&path).unwrap();
    assert_eq!(text.matches(",na,exactly,50).").count(), 1);
    let planted = text.replace(",na,exactly,50).", ",na,exactly,51).");
    canonical(planted.as_bytes());
    fs::write(&path, planted).unwrap();
    let out = regenerate(&copy);
    assert!(
        !out.status.success(),
        "changed source value passed regeneration"
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("p-f5b.pl.in"),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn rational_intake_changes_only_the_authorized_cardinality() {
    let stem = fixtures().join("patients/p-f5a");
    let emitted = fs::read_to_string(stem.with_extension("emitted.pl.in")).unwrap();
    let complete = fs::read_to_string(stem.with_extension("pl.in")).unwrap();
    let intake = fs::read_to_string(stem.with_extension("intake.tsv")).unwrap();
    let rows: Vec<&str> = intake.lines().collect();
    assert_eq!(rows.len(), 2);
    let columns: Vec<&str> = rows[1].split('\t').collect();
    assert_eq!(columns.len(), 4);
    assert_eq!(columns[0], "replace");
    assert_eq!(columns[3], "P-F5A-MME");
    assert_eq!(emitted.matches(columns[1]).count(), 1);
    assert_eq!(emitted.replace(columns[1], columns[2]), complete);
    assert!(columns[2].ends_with(",na,exactly,rational(75,2))."));
    canonical(complete.as_bytes());
    // K1 checks term syntax; the intake row above owns the reduced source value.
    assert_eq!(complete.matches("rational(75,2)").count(), 1);
}

fn quote(path: &Path) -> String {
    format!(
        "'{}'",
        path.to_str()
            .unwrap()
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
    )
}

fn recipe_blocks() -> Vec<String> {
    let reference = fs::read_to_string(root().join("docs/REFERENCE.md")).unwrap();
    let section = reference
        .split_once("### Loading a whole knowledge base\n")
        .unwrap()
        .1
        .split("\n### ")
        .next()
        .unwrap();
    let mut remaining = section;
    let mut blocks = Vec::new();
    while let Some((_, tail)) = remaining.split_once("```prolog\n") {
        let (block, rest) = tail.split_once("```\n").unwrap();
        blocks.push(block.to_owned());
        remaining = rest;
    }
    assert_eq!(blocks.len(), 3);
    blocks
}

fn corpus() -> Vec<PathBuf> {
    let mut documents: Vec<PathBuf> = fs::read_dir(root().join("guidelines/cdc-2022-opioid/pl"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "pl"))
        .collect();
    documents.sort();
    documents
}

fn world(s: &Scratch, extra: &[PathBuf], goal: &str) -> String {
    let blocks = recipe_blocks();
    let mut files = vec![
        s.file(
            "recipe.pl",
            format!("{}{}", blocks[0], blocks[1]).as_bytes(),
        ),
        root().join("guidelines/schema.pl"),
    ];
    files.extend(corpus());
    files.extend_from_slice(extra);
    let files = files
        .iter()
        .map(|path| quote(path))
        .collect::<Vec<_>>()
        .join(",");
    let identifier = blocks[2].trim().strip_suffix('.').unwrap();
    let out = Command::new("timeout")
        .args([
            "--kill-after=2",
            "90",
            "swipl",
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-g",
        ])
        .arg(format!("consult([{files}]),{identifier},{goal}"))
        .args(["-t", "halt"])
        .env("LC_ALL", "C.UTF-8")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    String::from_utf8(checked(out)).unwrap()
}

struct Stage {
    scratch: Scratch,
    tree: PathBuf,
    ulex: PathBuf,
}
impl Stage {
    fn new(name: &str) -> Self {
        let scratch = Scratch::new(name);
        let tree = scratch.0.join("ape");
        assert!(
            Command::new("cp")
                .args(["-a"])
                .arg(root().join("vendor/ape"))
                .arg(&tree)
                .status()
                .unwrap()
                .success()
        );
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            tree.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        let built = Command::new("swipl")
            .current_dir(tree.join("prolog/parser"))
            .args([
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-g",
                "[fit_to_plp],halt",
                "-t",
                "halt(9)",
            ])
            .output()
            .unwrap();
        assert!(built.status.success() && tree.join("prolog/parser/grammar.plp").is_file());
        let out = Command::new("swipl")
            .current_dir(tree.join("prolog/lexicon"))
            .args([
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-g",
                "style_check(-discontiguous),qcompile(clex_lexicon,[encoding(utf8)]),halt",
                "-t",
                "halt(9)",
            ])
            .output()
            .unwrap();
        assert!(checked(out).is_empty());
        let mut ulex = fs::read(root().join("guidelines/cdc-2022-opioid/lexicon.ulex")).unwrap();
        ulex.extend(fs::read(fixtures().join("lexicon.ulex")).unwrap());
        let ulex = scratch.file("lexicon.ulex", &ulex);
        Self {
            scratch,
            tree,
            ulex,
        }
    }

    fn emission(&self, id: &str, ace: &str) -> Output {
        let input = self.scratch.file(&format!("{id}.ace"), ace.as_bytes());
        Command::new("swipl")
            .args(["-q", "-f", "none", "-F", "none", "-s"])
            .arg(self.tree.join("prolog/ace_to_pl.pl"))
            .args(["-g", "main", "-t", "halt(9)", "--"])
            .arg(&self.tree)
            .arg(id)
            .arg(fixtures().join("vocabulary.tsv"))
            .arg(&self.ulex)
            .stdin(fs::File::open(input).unwrap())
            .env("LC_ALL", "C.UTF-8")
            .output()
            .unwrap()
    }

    fn compile(&self, id: &str, ace: &str) -> PathBuf {
        let output = self.scratch.0.join(format!("{id}.pl"));
        fs::write(&output, checked(self.emission(id, ace))).unwrap();
        output
    }
}

#[test]
fn r16_required_or_shape_is_rejected_instead_of_weakened() {
    let stage = Stage::new("r16");
    // Minimal OR shape, not a replacement for R16's formulations/settings.
    let out = stage.emission(
        "sb-r16-probe",
        "If a patient has an opioid-use-disorder then every clinician should offer a buprenorphine-treatment to the patient or should offer a methadone-treatment to the patient.\n",
    );
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert_eq!(
        out.stderr,
        b"ace_to_pl_error(unsupported,disjunctive_consequent(1)).\n"
    );
}

fn product(doc: &str, sentence: usize, slot: usize, deps: &[String]) -> String {
    format!(
        "'$guideline_id'(product,'{doc}',{sentence},ref({slot}),[{}])",
        deps.join(",")
    )
}

fn f4_contexts() -> String {
    let p = |sentence, slot| product("sb-p-f4", sentence, slot, &[]);
    let cdc = vec![p(2, 1), p(9, 4), p(9, 2), p(9, 1)];
    let va = vec![
        p(2, 1),
        p(8, 1),
        p(8, 2),
        p(8, 3),
        p(8, 4),
        p(9, 4),
        p(9, 2),
        p(9, 1),
    ];
    [
        ("cdc2022-opioid-s57-05", 5, 6, cdc),
        ("sb-f4-l1", 9, 10, va),
    ]
    .into_iter()
    .map(|(doc, disorder, have, deps)| {
        let subtype = vec![
            p(2, 1),
            product(doc, 1, disorder, &deps),
            product(doc, 1, have, &deps),
        ];
        let target = [
            p(2, 1),
            p(5, 1),
            p(5, 4),
            product("sb-f4-l2", 1, 4, &subtype),
            product("sb-f4-l2", 1, 5, &subtype),
        ]
        .join(",");
        format!("'$guideline_id'(context,'cdc2022-opioid-s60-11',2,box(1),[{target}]).\n")
    })
    .collect()
}

#[test]
fn f4_chain_reaches_the_document_specific_must_and_break_controls_do_not() {
    let stage = Stage::new("f4");
    let ace = fs::read_to_string(fixtures().join("patients/p-f4.ace")).unwrap();
    let target = |doc: &str, name: &str| {
        format!(
            "findall(C,(C='$guideline_id'(context,'{doc}',2,box(1),_),guideline_operator(actual,C,must)),Raw{name}),sort(Raw{name},Rows{name}),length(Rows{name},N{name}),format('{name} ~d~n',[N{name}])"
        )
    };
    let goal = format!(
        "{},{},forall(member(C,RowsTarget),(write_term(C,[quoted(true),ignore_ops(true),numbervars(true)]),writeln('.')))",
        target("cdc2022-opioid-s60-11", "Target"),
        target("cdc2022-opioid-s57-02", "Shortcut")
    );
    for (name, text, subtype, expected) in [
        ("positive", ace.clone(), true, "Target 2\nShortcut 2\n"),
        (
            "missing-seed",
            ace.lines()
                .filter(|line| !line.contains("meets exactly 2"))
                .collect::<Vec<_>>()
                .join("\n")
                + "\n",
            true,
            "Target 0\nShortcut 0\n",
        ),
        (
            "subtype-break",
            ace.clone(),
            false,
            "Target 0\nShortcut 2\n",
        ),
        (
            "missing-pain",
            ace.lines()
                .filter(|line| {
                    !line.contains("has a pain that lasts")
                        && !line.contains("The pain is persistent")
                })
                .collect::<Vec<_>>()
                .join("\n")
                + "\n",
            true,
            "Target 0\nShortcut 0\n",
        ),
        (
            "minor",
            ace.replace("exactly 18 years", "exactly 17 years"),
            true,
            "Target 0\nShortcut 0\n",
        ),
    ] {
        let patient = stage.compile("sb-p-f4", &text);
        let mut files = vec![
            fixtures().join("bridges/f4-l1.pl.in"),
            fixtures().join("bridges/f4-l3.pl.in"),
        ];
        if subtype {
            files.push(fixtures().join("bridges/f4-l2.pl.in"));
        }
        files.push(patient);
        let expected = if name == "positive" {
            format!("{expected}{}", f4_contexts())
        } else {
            expected.to_owned()
        };
        assert_eq!(world(&stage.scratch, &files, &goal), expected, "{name}");
    }
}

fn id(sentence: usize, slot: usize) -> String {
    product("sb-p-f5b", sentence, slot, &[])
}

#[test]
fn f5b_enables_both_opioid_benzodiazepine_valuations() {
    let stage = Stage::new("f5b");
    let goal = "findall(join(V,C),(C='$guideline_id'(context,'cdc2022-opioid-rec11-imp01',2,box(1),V),guideline_operator(actual,C,should)),Raw),sort(Raw,Rows),length(Rows,N),format('joins ~d~n',[N]),forall(member(Row,Rows),(write_term(Row,[quoted(true),ignore_ops(true),numbervars(true)]),writeln('.')))";
    let mut expected = String::from("joins 2\n");
    for (sentence, event) in [(8, 4), (10, 2)] {
        let valuation = format!(
            "[{},{},{},{}]",
            id(1, 1),
            id(sentence, 1),
            id(6, 1),
            id(sentence, event)
        );
        expected.push_str(&format!("join({valuation},'$guideline_id'(context,'cdc2022-opioid-rec11-imp01',2,box(1),{valuation})).\n"));
    }
    let patient = fixtures().join("patients/p-f5b.pl.in");
    assert_eq!(world(&stage.scratch, &[patient], goal), expected);
    let ace = fs::read_to_string(fixtures().join("patients/p-f5b.ace")).unwrap();
    assert_eq!(ace.matches("with the benzodiazepine").count(), 2);
    let control = stage.compile("sb-p-f5b", &ace.replace(" with the benzodiazepine", ""));
    assert_eq!(world(&stage.scratch, &[control], goal), "joins 0\n");
}

#[test]
fn f5c_keeps_every_source_product_and_fractional_strength() {
    use sha2::{Digest, Sha256};

    let source = fs::read_to_string(fixtures().join("patients/p-f5c-products.tsv")).unwrap();
    let cells = source
        .lines()
        .map(|line| line.split('\t').take(5).collect::<Vec<_>>().join("\t") + "\n")
        .collect::<String>();
    let source_matches = |cells: &str| {
        Sha256::digest(cells.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
            == "645a942e6998fc20f970eecf73edcbc20370cc2f551dd22eaee1bce19fa4458f"
    };
    assert!(source_matches(&cells));
    assert_eq!(cells.matches("Tramadol\t50 mg").count(), 1);
    assert!(!source_matches(&cells.replacen(
        "Tramadol\t50 mg",
        "Tramadol\t51 mg",
        1
    )));
    let excerpt =
        fs::read_to_string(root().join("bench/sb/annex/excerpts/PMC9225568.txt")).unwrap();
    let quoted_rows = excerpt
        .lines()
        .filter_map(|line| {
            let columns = line.split('|').map(str::trim).collect::<Vec<_>>();
            (columns.len() == 7 && columns[1].parse::<usize>().is_ok())
                .then(|| columns[1..6].join("\t") + "\n")
        })
        .collect::<String>();
    assert_eq!(cells.split_once('\n').unwrap().1, quoted_rows);
    assert_eq!(source.lines().count(), 25);
    for (number, row) in source.lines().skip(1).enumerate() {
        let fields: Vec<&str> = row.split('\t').collect();
        let kind = match number + 1 {
            2 => "benzodiazepine",
            18 | 19 => "gabapentinoid",
            23 => "opioid-pain-medication",
            _ => "medication-product",
        };
        assert_eq!(fields[5], kind);
    }
    let mapping = fs::read_to_string(fixtures().join("patients/p-f5c-map.tsv")).unwrap();
    let identities = mapping
        .lines()
        .skip(1)
        .map(|line| line.split('\t').nth(3).unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(identities.len(), 24);
    let stem = fixtures().join("patients/p-f5c");
    let emitted = fs::read_to_string(stem.with_extension("emitted.pl.in")).unwrap();
    let complete = fs::read_to_string(stem.with_extension("pl.in")).unwrap();
    let intake = fs::read_to_string(stem.with_extension("intake.tsv")).unwrap();
    let rows: Vec<&str> = intake.lines().skip(1).collect();
    assert_eq!(rows.len(), 25);
    for (sentence, product_sentence, label) in [
        (13, 12, "alprazolam-product-strength"),
        (31, 30, "ipratropium-component-strength"),
        (36, 35, "tiotropium-product-strength"),
        (52, 50, "carvedilol-product-strength"),
        (67, 66, "nitroglycerin-product-strength"),
    ] {
        let strength = product("sb-p-f5c", sentence, 1, &[]);
        let have = product("sb-p-f5c", sentence, 2, &[]);
        let medication = product("sb-p-f5c", product_sentence, 1, &[]);
        let contain = product("sb-p-f5c", sentence, 4, &[]);
        assert!(emitted.contains(&format!(
            "guideline_entity(actual,{strength},'{label}',countable)."
        )));
        assert!(emitted.contains(&format!("guideline_arg(actual,{have},1,{medication}).")));
        assert!(emitted.contains(&format!("guideline_arg(actual,{have},2,{strength}).")));
        assert!(complete.contains(&format!("guideline_arg(actual,{contain},1,{strength}).")));
    }
    let mut expected = emitted;
    for row in rows {
        let fields: Vec<&str> = row.split('\t').collect();
        assert_eq!(fields.len(), 4);
        assert_eq!((fields[0], fields[1]), ("append", "-"));
        assert!(fields[3].starts_with("P-F5C-R"));
        expected.push_str(fields[2]);
        expected.push('\n');
    }
    assert_eq!(complete, expected);
    for (value, count) in [
        ("rational(1,2)", 2),
        ("rational(5,4)", 1),
        ("rational(25,4)", 1),
        ("rational(2,5)", 1),
    ] {
        assert_eq!(complete.matches(value).count(), count, "{value}");
    }
    canonical(complete.as_bytes());
}

#[test]
fn mx_transfers_the_drug_correlation_without_unexamined_doses_or_history() {
    let ace = fs::read_to_string(fixtures().join("patients/p-mx.ace")).unwrap();
    for absent in [
        "milligram",
        "microgram",
        "MME",
        "prescribed-dose",
        "at an interval",
        "pre-interruption",
    ] {
        assert!(!ace.contains(absent), "{absent}");
    }
    assert!(ace.contains("as-needed-anxiety-treatment"));
    let document = fs::read_to_string(fixtures().join("patients/p-mx.pl.in")).unwrap();
    for (kind, expected) in [
        ("'opioid-pain-medication'", 2),
        ("benzodiazepine", 1),
        ("gabapentinoid", 1),
    ] {
        let suffix = format!(",{kind},countable).");
        assert_eq!(
            document
                .lines()
                .filter(
                    |line| line.starts_with("guideline_entity(actual,") && line.ends_with(&suffix)
                )
                .count(),
            expected,
            "{kind}"
        );
    }
}

#[test]
fn f5c_source_mapping_rejects_a_changed_strength() {
    let s = Scratch::new("f5c-source-plant");
    let copy = s.0.join("fixtures");
    assert!(
        Command::new("cp")
            .arg("-a")
            .arg(fixtures())
            .arg(&copy)
            .status()
            .unwrap()
            .success()
    );
    let source = copy.join("patients/p-f5c-products.tsv");
    let text = fs::read_to_string(&source).unwrap();
    let anchor = "Hydroxyzine\t50 mg\t1 tablet at bedtime\tmedication-product\t:50:milligram\t";
    assert_eq!(text.matches(anchor).count(), 1);
    fs::write(
        &source,
        text.replace(anchor, &anchor.replace(":50:", ":51:")),
    )
    .unwrap();
    let out = Command::new("python3")
        .arg("-I")
        .arg(copy.join("f5c-inputs"))
        .arg(root())
        .arg(&copy)
        .arg("--check")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("('strength', '1')"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn f5c_enumerates_every_enabled_medication_join_and_its_unused_facts() {
    let stage = Stage::new("f5c");
    let patient = fixtures().join("patients/p-f5c.pl.in");
    let probe = root().join("rust/ckc/tests/bench-fixtures/f5c-probe.pl.in");
    let docs = corpus()
        .iter()
        .map(|path| quote(path))
        .collect::<Vec<_>>()
        .join(",");
    let goal = |patient: &Path| format!("sb_f5c_probe({},[{docs}])", quote(patient));
    let p = |sentence, slot| product("sb-p-f5c", sentence, slot, &[]);
    let valuation = format!("[{},{},{},{}]", p(1, 1), p(88, 1), p(12, 1), p(97, 1));
    let used_products = format!("[{},{}]", p(12, 1), p(88, 1));
    let mut expected = "products(24).\nclinical_facts(785).\nused_clinical_facts(10).\nunused_clinical_facts(775).\njoins(3).\n".to_owned();
    for (doc, sentence) in [
        ("cdc2022-opioid-rec09-imp13", 1),
        ("cdc2022-opioid-rec11", 2),
        ("cdc2022-opioid-rec11-imp01", 2),
    ] {
        expected.push_str(&format!("join('$guideline_id'(context,'{doc}',{sentence},box(1),{valuation}),should,{used_products}).\n"));
    }
    assert_eq!(
        world(
            &stage.scratch,
            &[patient.clone(), probe.clone()],
            &goal(&patient)
        ),
        expected
    );
    let ace = fs::read_to_string(fixtures().join("patients/p-f5c.ace")).unwrap();
    assert_eq!(ace.matches("prescribes M23 with M2").count(), 1);
    let control = stage.compile(
        "sb-p-f5c",
        &ace.replace("prescribes M23 with M2", "prescribes M23"),
    );
    let mut complete = fs::read_to_string(&control).unwrap();
    let intake = fs::read_to_string(fixtures().join("patients/p-f5c.intake.tsv")).unwrap();
    for row in intake.lines().skip(1) {
        complete.push_str(row.split('\t').nth(2).unwrap());
        complete.push('\n');
    }
    fs::write(&control, complete).unwrap();
    assert_eq!(
        world(&stage.scratch, &[control.clone(), probe], &goal(&control)),
        "products(24).\nclinical_facts(784).\nused_clinical_facts(0).\nunused_clinical_facts(784).\njoins(0).\n"
    );
}
