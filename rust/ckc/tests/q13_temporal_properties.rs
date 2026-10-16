// Q13 D1 + c1 D4: generated adjective/preposition aliases + comment rows against an
// independent row oracle (one vocabulary.tsv law, every kind legal).
// CKC_UI_TEST_BIN selects the unfixed binary; the UI intake shares the table contract.
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone, Debug)]
struct Row<'a>(&'a str, &'a str, &'a str);

fn reference(rows: &[Row<'_>]) -> Result<(), String> {
    let mut prepositions = BTreeSet::new();
    let mut adjectives = BTreeSet::new();
    for (i, Row(kind, lemma, value)) in rows.iter().enumerate() {
        let why = if kind.starts_with('#') {
            Some("comment")
        } else if lemma.is_empty() || lemma.bytes().any(|b| b <= 32 || b == 127) {
            Some("lemma")
        } else {
            match *kind {
                "approximation" if *value != "about" => Some("approximation value"),
                "range" if *value != "minimum" => Some("range value"),
                "frequency" if *value != "period" => Some("frequency value"),
                "approximation" if !adjectives.insert(*lemma) => Some("duplicate adjective"),
                "relation" | "frequency" | "range" if !prepositions.insert(*lemma) => {
                    Some("duplicate preposition")
                }
                _ => None,
            }
        };
        if let Some(why) = why {
            return Err(format!("{why} at row {}", i + 1));
        }
    }
    Ok(())
}

#[test]
fn q13_generated_temporal_alias_law() {
    let root = root();
    let scratch = Scratch(root.join(format!(
        "rust/target/q13-temporal-properties/{}",
        std::process::id()
    )));
    let tree = scratch.0.join("tree");
    copy_tree(&root.join("tests/ui/green/q12-temporal-words/tree"), &tree);
    let program = std::env::var_os("CKC_UI_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    let table_path = tree.join("guidelines/alpha/vocabulary.tsv");
    let header = format!(
        "{}\n",
        fs::read_to_string(&table_path)
            .unwrap()
            .lines()
            .next()
            .unwrap()
    );
    let preps = [
        ("relation", "after"),
        ("frequency", "period"),
        ("range", "minimum"),
    ];
    let mut failures = Vec::new();
    let mut checked = 0;
    let mut run = |rows: Vec<Row<'_>>| {
        checked += 1;
        let mut table = header.clone();
        for Row(kind, lemma, value) in &rows {
            table.push_str(&format!("{kind}\t{lemma}\t{value}\n"));
        }
        fs::write(&table_path, table).unwrap();
        let output = Command::new(&program)
            .args(["ui", "check"])
            .arg(&tree)
            .current_dir(&scratch.0)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .output()
            .unwrap();
        let expected = reference(&rows);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let matched = match &expected {
            Ok(()) => output.status.success() && stderr.is_empty(),
            Err(why) => {
                output.status.code() == Some(1)
                    && stderr == format!("ui: viewmodel: alpha vocabulary.tsv: {why}\n")
            }
        };
        if !matched {
            failures.push(format!(
                "case {checked} {rows:?}: expected {expected:?}, rc {:?}, stderr {stderr:?}",
                output.status.code()
            ));
        }
    };
    for lemma in [
        "to",
        "per",
        "for",
        "approximate",
        "unit-like",
        "é",
        "Ω",
        "a&<b>",
        "'",
        "x\\y",
    ] {
        // Equal aliases reject across every ordered preposition-kind pair; fresh aliases commute.
        for (left, lv) in preps {
            for (right, rv) in preps {
                run(vec![Row(left, lemma, lv), Row(right, lemma, rv)]);
                run(vec![Row(left, lemma, lv), Row(right, "distinct-alias", rv)]);
            }
        }
        run(vec![
            Row("approximation", lemma, "about"),
            Row("approximation", lemma, "about"),
        ]);
        run(vec![
            Row("approximation", lemma, "about"),
            Row("approximation", "distinct-alias", "about"),
        ]);
        for (kind, value) in [
            ("unit", "hour"),
            ("relation", "after"),
            ("spacing", "frame"),
            ("window", "window"),
            ("frequency", "period"),
            ("range", "minimum"),
        ] {
            run(vec![
                Row("approximation", lemma, "about"),
                Row(kind, lemma, value),
            ]);
            run(vec![
                Row(kind, lemma, value),
                Row("approximation", lemma, "about"),
            ]);
        }
        for (kind, value) in [("spacing", "frame"), ("window", "window")] {
            run(vec![
                Row("range", lemma, "minimum"),
                Row(kind, lemma, value),
            ]);
            run(vec![
                Row(kind, lemma, value),
                Row("range", lemma, "minimum"),
            ]);
        }
        // A `#` row is a comment, ahead of the row's other laws (c1 D4).
        run(vec![Row("#approximation", lemma, "about")]);
        run(vec![
            Row("approximation", lemma, "about"),
            Row("#range", lemma, "minimum"),
        ]);
        for (kind, value, wrong) in [
            ("approximation", "about", "ABOUT"),
            ("range", "minimum", "maximum"),
        ] {
            run(vec![Row(kind, lemma, wrong)]);
            run(vec![Row(kind, lemma, "")]);
            run(vec![Row(kind, lemma, value), Row(kind, lemma, wrong)]);
        }
    }
    assert_eq!(checked, 440, "Q13 generated domain census");
    assert!(
        failures.is_empty(),
        "Q13 alias law: {} of {checked} differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
