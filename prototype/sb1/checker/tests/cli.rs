use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const WORLD: &str = include_str!("../../../../tests/consumer/adult.pl.in");
const SCHEMA: &str = include_str!("../../../../guidelines/schema.pl");

fn hash(bytes: &[u8]) -> String {
    let mut child = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn real_cli_accepts_consumer_world_and_reports_refusal_and_usage() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/cli")
        .join(std::process::id().to_string());
    fs::create_dir_all(root.join("guidelines")).unwrap();
    let _scratch = Scratch(root.clone());
    fs::write(root.join("guidelines/schema.pl"), SCHEMA).unwrap();
    fs::write(root.join("world.pl"), WORLD).unwrap();
    fs::write(root.join("composition.tsv"), "world.pl\n").unwrap();
    let mut facts = BTreeMap::new();
    let mut ordinal = 0;
    for line in WORLD
        .lines()
        .filter(|line| !line.starts_with('%') && !line.starts_with(":-") && !line.is_empty())
    {
        ordinal += 1;
        facts.entry(line.trim_end_matches('.')).or_insert(ordinal);
    }
    let frame = format!(
        "schema\0{}\0{}\nfile\0world.pl\0{}\0{}\n",
        SCHEMA.len(),
        hash(SCHEMA.as_bytes()),
        WORLD.len(),
        hash(WORLD.as_bytes())
    );
    let mut certificate = format!(
        "sb1-certificate 0\ncomposition {}\n",
        hash(frame.as_bytes())
    );
    for (fact, ordinal) in &facts {
        certificate.push_str(&format!("{fact}\t1:{ordinal}\t[]\t0\n"));
    }
    fs::write(
        root.join("model.txt"),
        facts.keys().map(|f| format!("{f}.\n")).collect::<String>(),
    )
    .unwrap();
    fs::write(root.join("certificate.txt"), &certificate).unwrap();
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_checker"))
            .arg("--root")
            .arg(&root)
            .arg(root.join("composition.tsv"))
            .arg(root.join("model.txt"))
            .arg(root.join("certificate.txt"))
            .output()
            .unwrap()
    };
    let out = invoke();
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stderr.is_empty());
    assert_eq!(
        out.stdout,
        format!("accepted {} facts\n", facts.len()).as_bytes()
    );
    assert_eq!(
        fs::read_to_string(root.join("checker-phases.tsv"))
            .unwrap()
            .lines()
            .count(),
        8
    );
    assert!(
        fs::read_to_string(root.join("checker-counters.tsv"))
            .unwrap()
            .contains("logical\tsource_header_adapters\t1\n")
    );
    certificate.pop();
    fs::write(root.join("certificate.txt"), certificate).unwrap();
    let out = invoke();
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert!(out.stderr.starts_with(b"refused envelope "));
    fs::remove_file(root.join("certificate.txt")).unwrap();
    let out = invoke();
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert!(out.stderr.starts_with(b"I/O "));
    let out = Command::new(env!("CARGO_BIN_EXE_checker"))
        .arg("--root")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert!(out.stderr.starts_with(b"usage: checker "));
}
