use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[path = "../../../tests/eso/generated.rs"]
mod generated;

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn result(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes)
        .unwrap()
        .rsplit_once(",result(")
        .unwrap()
        .1
        .strip_suffix(")).\n")
        .unwrap()
}

#[test]
fn restored_root_witnesses_exhaust_with_two_solutions() {
    let root = root();
    let program = std::env::var_os("CKC_ESO_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    let scratch = Scratch(root.join(format!("rust/target/eso-acceptance/{}", std::process::id())));
    fs::create_dir_all(&scratch.0).unwrap();
    let corpus = root.join("guidelines/cdc-2022-opioid");
    let mut docs: Vec<_> = fs::read_dir(corpus.join("pl"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "pl"))
        .collect();
    docs.sort_by(|a, b| {
        a.as_os_str()
            .as_encoded_bytes()
            .cmp(b.as_os_str().as_encoded_bytes())
    });
    assert!(!docs.is_empty());
    let restored = root.join("tests/eso/cdc2022-opioid-s32-06-restored.pl.in");
    let mut replacements = 0;
    let manifest: String = docs
        .iter()
        .map(|path| {
            let pl = if path.file_name().unwrap() == "cdc2022-opioid-s32-06.pl" {
                replacements += 1;
                &restored
            } else {
                path
            };
            format!("{}\t{}\n", pl.display(), pl.display())
        })
        .collect();
    assert_eq!(replacements, 1);
    let manifest_path = scratch.0.join("manifest.tsv");
    fs::write(&manifest_path, manifest).unwrap();
    let expected = fs::read(corpus.join("queries/answers/dosage-reduction-content.pl")).unwrap();
    assert!(result(&expected).starts_with("solutions(["));
    assert_eq!(result(&expected).matches("sol([").count(), 2);
    let output = Command::new(program)
        .args(["v1", "answer"])
        .arg(manifest_path)
        .arg(corpus.join("queries/pl/dosage-reduction-content.pl"))
        .current_dir(&root)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty(), "{:?}", output.stderr);
    assert_eq!(result(&output.stdout), result(&expected));
}
