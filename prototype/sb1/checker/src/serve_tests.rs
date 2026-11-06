// Warm server load: the census identity + the typed-analysis receipt gate
// (contract sb § Carried rulings).
use crate::orchestrator::{OrchestratedRequest, Provider, ServeRequest, knowledge_identity, serve};
use crate::{SourceCodec, canonical};
use std::fs;
use std::path::{Path, PathBuf};

const ADULT: &str = include_str!("../../../../bench/sb/fixtures/patients/p-adult.pl.in");
const KNOWLEDGE: &str =
    include_str!("../../../../guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s9-05.pl");

// Pinned by an independent implementation (Python hashlib over the same framing).
#[test]
fn knowledge_identity_matches_the_census_definition() {
    assert_eq!(
        knowledge_identity(b"s", &[b"a", b"bc"]),
        "3eb64e568783278fbd8f3ee9e395f0b61098b556921af13d9e67a8fa5f685e52"
    );
}

fn root(name: &str) -> PathBuf {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/serve-fixtures")
        .join(format!("{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(directory.join("guidelines")).unwrap();
    fs::write(
        directory.join("guidelines/schema.pl"),
        ckc_kernel::contract::schema_file(),
    )
    .unwrap();
    // Knowledge = one CDC document; patient suffix (1, no manifest) = adult.
    assert!(canonical(KNOWLEDGE.as_bytes()) && canonical(ADULT.as_bytes()));
    fs::write(directory.join("knowledge.pl"), KNOWLEDGE).unwrap();
    fs::write(directory.join("adult.pl"), ADULT).unwrap();
    fs::write(
        directory.join("composition.tsv"),
        "knowledge.pl\nadult.pl\n",
    )
    .unwrap();
    directory
}
fn load(directory: &Path, receipts: Option<PathBuf>) -> (Result<(), crate::Failure>, String) {
    load_with(directory, receipts, None)
}
fn load_with(
    directory: &Path,
    receipts: Option<PathBuf>,
    ta_run: Option<PathBuf>,
) -> (Result<(), crate::Failure>, String) {
    let request = ServeRequest {
        base: OrchestratedRequest {
            root: directory.into(),
            composition: directory.join("composition.tsv"),
            output: directory.join("arm"),
            query: "-".into(),
            provider: Provider::Static,
            producer: "producer.prolog".into(),
            swipl: "swipl".into(),
        },
        receipts,
        ta_run,
    };
    let mut out = Vec::new();
    let result = serve(&request, SourceCodec::Parsed, std::io::empty(), &mut out);
    (result, String::from_utf8(out).unwrap())
}
fn identity(directory: &Path) -> String {
    let schema = fs::read(directory.join("guidelines/schema.pl")).unwrap();
    knowledge_identity(&schema, &[KNOWLEDGE.as_bytes()])
}
fn receipt(directory: &Path, identity: &str, verdict: &str, format: &str) -> PathBuf {
    let receipts = directory.join("receipts");
    fs::create_dir_all(&receipts).unwrap();
    fs::write(
        receipts.join(format!("{identity}.tsv")),
        format!(
            "format\t{format}\nkb_identity\t{identity}\nverdict\t{verdict}\ndc\t1\nvmhwm_bytes\t7\n"
        ),
    )
    .unwrap();
    receipts
}

#[test]
fn a_load_needs_an_admitted_receipt_for_its_knowledge_identity() {
    let directory = root("receipt");
    let kb = identity(&directory);
    // No receipt for this identity: refused at load, nothing announced.
    let empty = directory.join("empty");
    fs::create_dir_all(&empty).unwrap();
    let (result, out) = load(&directory, Some(empty));
    assert_eq!(result.unwrap_err().rc, 1);
    assert!(out.is_empty());
    // A refused verdict refuses; a malformed receipt is a parse failure.
    let (result, _) = load(
        &directory,
        Some(receipt(&directory, &kb, "refused", "sb1-ta-receipt 1")),
    );
    assert_eq!(result.unwrap_err().rc, 1);
    let (result, _) = load(
        &directory,
        Some(receipt(&directory, &kb, "admitted", "sb1-ta-receipt 0")),
    );
    assert_eq!(result.unwrap_err().rc, 2);
    // The admitted receipt for exactly this identity: ready, its fields in load.tsv.
    let (result, out) = load(
        &directory,
        Some(receipt(&directory, &kb, "admitted", "sb1-ta-receipt 1")),
    );
    result.unwrap();
    assert_eq!(out, "ready\n");
    let table = fs::read_to_string(directory.join("arm/load.tsv")).unwrap();
    assert!(table.contains(&format!("kb_identity\t{kb}\n")));
    assert!(
        table.contains("receipt_verdict\tadmitted\n") && table.contains("receipt_vmhwm_bytes\t7\n")
    );
    // Unchecked development loads say so.
    fs::remove_dir_all(directory.join("arm")).unwrap();
    let (result, _) = load(&directory, None);
    result.unwrap();
    assert!(
        fs::read_to_string(directory.join("arm/load.tsv"))
            .unwrap()
            .contains("receipt\tunchecked\n")
    );
    fs::remove_dir_all(&directory).unwrap();
}

// `--ta-run` runs the census over the whole composition with the redesigned analysis
// (the one I2 measured; without `--redesign` the census binary runs its old default);
// rc 0 admits, rc 1 refuses, any other exit = rc 2. A committed stand-in, so no
// executable is written while other test threads spawn children (ETXTBSY).
#[test]
fn an_integrated_load_runs_the_redesigned_census() {
    let directory = root("ta-run");
    let census = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fake-census.sh");
    let arm = directory.join("arm");
    fs::write(directory.join("census-rc"), "0\n").unwrap();
    let (result, out) = load_with(&directory, None, Some(census.clone()));
    result.unwrap();
    assert_eq!(out, "ready\n");
    assert_eq!(
        fs::read_to_string(directory.join("census-argv")).unwrap(),
        format!(
            "--root\n{}\n--composition\n{}\n--out\n{}\n--dc\n--patient\nlast\n--redesign\n",
            directory.display(),
            directory.join("composition.tsv").display(),
            arm.join("ta").display()
        )
    );
    assert!(
        fs::read_to_string(arm.join("load.tsv"))
            .unwrap()
            .contains("receipt\tintegrated\n")
    );
    for (code, rc) in [("1", 1), ("2", 2)] {
        fs::remove_dir_all(&arm).unwrap();
        fs::write(directory.join("census-rc"), format!("{code}\n")).unwrap();
        let (result, out) = load_with(&directory, None, Some(census.clone()));
        assert_eq!(result.unwrap_err().rc, rc);
        assert!(out.is_empty());
    }
    fs::remove_dir_all(&directory).unwrap();
}
