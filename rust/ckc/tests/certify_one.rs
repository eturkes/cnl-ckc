// N25: `ckc certify-one` over the committed tests/certify-one fixtures (contract n25 D2;
// regenerate them with .agent/archive/generators/certify-one-fixtures): accepted doc, query
// and table-backed (`v3`) runs pin rc 0 + stdout; every reject pins rc 1 + its stderr line, stdout empty.
// CKC_CERTIFY_ONE_TEST_BIN selects a prebuilt executable.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn fixture(case: &str, name: &str) -> PathBuf {
    root().join("tests/certify-one").join(case).join(name)
}

struct Scratch(PathBuf);
impl Scratch {
    fn new(label: &str) -> Self {
        let dir = root().join(format!(
            "rust/target/certify-one/{}-{label}",
            std::process::id()
        ));
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

// (rc, stdout, stderr) of `ckc certify-one <args>`.
fn certify_one(args: &[&Path]) -> (Option<i32>, Vec<u8>, Vec<u8>) {
    let program = std::env::var_os("CKC_CERTIFY_ONE_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    let out = Command::new(program)
        .arg("certify-one")
        .args(args)
        .output()
        .unwrap();
    (out.status.code(), out.stdout, out.stderr)
}

// doc | query, id, ace, ulex, dump, pl, [vocabulary] for one fixture case, with overrides.
fn argv<'a>(mode: &'a str, id: &'a str, case: &str, swap: &[(usize, &'a Path)]) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = vec![
        mode.into(),
        id.into(),
        fixture(case, "ace.txt.in"),
        "-".into(),
        fixture(case, "dump.txt.in"),
        fixture(case, "pl.txt.in"),
    ];
    if case == "v3" {
        v.push(fixture(case, "vocabulary.tsv.in"));
    }
    for (i, p) in swap {
        if *i == v.len() {
            v.push(p.to_path_buf());
        } else {
            v[*i] = p.to_path_buf();
        }
    }
    v
}

fn rejects(args: Vec<PathBuf>, stderr: &str) {
    let refs: Vec<&Path> = args.iter().map(PathBuf::as_path).collect();
    let (rc, out, err) = certify_one(&refs);
    assert_eq!(
        (
            rc,
            String::from_utf8_lossy(&out),
            String::from_utf8_lossy(&err)
        ),
        (Some(1), "".into(), stderr.into()),
        "{args:?}"
    );
}

#[test]
fn accepted_runs_print_their_pinned_output() {
    for (case, mode, id) in [
        ("doc", "doc", "fx-doc"),
        ("query", "query", "fx-query"),
        ("v3", "doc", "fx-timed"),
    ] {
        let args = argv(mode, id, case, &[]);
        let refs: Vec<&Path> = args.iter().map(PathBuf::as_path).collect();
        let (rc, out, err) = certify_one(&refs);
        assert_eq!((rc, err.as_slice()), (Some(0), &b""[..]), "{case}");
        assert_eq!(
            out,
            fs::read(fixture(case, "stdout.expect")).unwrap(),
            "{case} stdout"
        );
    }
}

#[test]
fn malformed_arguments_reject() {
    rejects(
        argv("chapter", "fx-doc", "doc", &[]),
        "ckc: certify: fx-doc: arguments.\n",
    );
    rejects(
        argv("doc", "FX-doc", "doc", &[]),
        "ckc: certify: FX-doc: arguments.\n",
    );
    rejects(
        argv("doc", "fx_doc", "doc", &[]),
        "ckc: certify: fx_doc: arguments.\n",
    );
    rejects(argv("doc", "", "doc", &[]), "ckc: certify: : arguments.\n");
}

#[test]
fn unreadable_and_invalid_inputs_reject() {
    let s = Scratch::new("inputs");
    let missing = s.0.join("missing");
    let bad = s.file("bad.dump", b"\xff\n");
    for (slot, why) in [
        (2, "unreadable(ace)."),
        (3, "unreadable(ulex)."),
        (4, "unreadable(dump)."),
        (5, "unreadable(pl)."),
    ] {
        rejects(
            argv("doc", "fx-doc", "doc", &[(slot, &missing)]),
            &format!("ckc: certify: fx-doc: {why}\n"),
        );
    }
    rejects(
        argv("doc", "fx-doc", "doc", &[(4, &bad)]),
        "ckc: certify: fx-doc: invalid_utf8(dump).\n",
    );
    rejects(
        argv("doc", "fx-timed", "v3", &[(6, &missing)]),
        "ckc: certify: fx-timed: unreadable(vocabulary).\n",
    );
}

#[test]
fn certification_rejects_altered_inputs() {
    let s = Scratch::new("altered");
    let pl = fs::read_to_string(fixture("doc", "pl.txt.in")).unwrap();
    assert!(pl.contains("person"));
    let tampered = s.file("tampered.pl", pl.replacen("person", "persom", 1).as_bytes());
    rejects(
        argv("doc", "fx-doc", "doc", &[(5, &tampered)]),
        "ckc: certify: fx-doc: clauses(1).\n",
    );
    let mut table = fs::read(fixture("v3", "vocabulary.tsv.in")).unwrap();
    table.extend_from_slice(b"unit\tday\tday\n");
    let foreign = s.file("foreign.tsv", &table);
    rejects(
        argv("doc", "fx-timed", "v3", &[(6, &foreign)]),
        "ckc: certify: fx-timed: vocabulary.\n",
    );
    let v3_table = fixture("v3", "vocabulary.tsv.in");
    rejects(
        argv("doc", "fx-doc", "doc", &[(6, &v3_table)]),
        "ckc: certify: fx-doc: vocabulary.\n",
    );
    rejects(
        argv("query", "fx-doc", "doc", &[]),
        "ckc: certify: fx-doc: record_shape.\n",
    );
}

#[test]
fn wrong_operand_count_is_usage() {
    let one = [Path::new("doc")];
    let mut eight = argv("doc", "fx-timed", "v3", &[]);
    eight.push("extra".into());
    let eight: Vec<&Path> = eight.iter().map(PathBuf::as_path).collect();
    for args in [&one[..], &eight[..]] {
        let (rc, out, err) = certify_one(args);
        assert_eq!(
            (rc, out.as_slice()),
            (Some(2), &b""[..]),
            "{} operands",
            args.len()
        );
        assert!(err.starts_with(b"usage: ckc "), "{} operands", args.len());
    }
}
