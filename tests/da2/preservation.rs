use super::cases::{EXAMPLE, ULEX};
use super::support::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

fn parallel<T: Sync>(items: &[T], f: impl Fn(&T) -> Vec<String> + Sync) -> Vec<String> {
    let next = AtomicUsize::new(0);
    let failures = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let (f, next, failures) = (&f, &next, &failures);
            scope.spawn(move || {
                while let Some(item) = items.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let found = f(item);
                    failures.lock().unwrap().extend(found);
                }
            });
        }
    });
    failures.into_inner().unwrap()
}
fn ulex_for(ace: &Path) -> Option<PathBuf> {
    let name = ace.file_name().unwrap().to_str().unwrap();
    let sidecar = if let Some(stem) = name.strip_suffix(".ace.in") {
        ace.with_file_name(format!("{stem}.ulex.in"))
    } else {
        ace.with_extension("ulex")
    };
    if sidecar.is_file() {
        return Some(sidecar);
    }
    let root = root();
    for parent in ace.ancestors().skip(1).take_while(|p| *p != root) {
        for name in ["lexicon.ulex", "lexicon.ulex.in"] {
            let path = parent.join(name);
            if path.is_file() {
                return Some(path);
            }
        }
    }
    None
}
fn table_for(ace: &Path) -> Option<PathBuf> {
    let sidecar = ace.with_extension("vocabulary.tsv");
    if sidecar.is_file() {
        return Some(sidecar);
    }
    let root = root();
    for parent in ace.ancestors().skip(1).take_while(|p| *p != root) {
        let path = parent.join("vocabulary.tsv");
        if path.is_file() {
            return Some(path);
        }
    }
    None
}
// D8's tree map as a lexical envelope transform: quoted atoms and integers are opaque.
pub(super) fn decimal_envelope(text: &str) -> String {
    let b = text.as_bytes();
    let (mut i, mut quoted, mut escaped) = (0, 0u8, false);
    let mut out = String::new();
    while i < b.len() {
        if escaped {
            out.push(b[i] as char);
            escaped = false;
            i += 1;
            continue;
        }
        if quoted != 0 {
            if b[i] == b'\\' {
                escaped = true;
            } else if b[i] == quoted {
                quoted = 0;
            }
            let ch = text[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
            continue;
        }
        if b[i] == b'\'' || b[i] == b'"' {
            quoted = b[i];
            out.push(b[i] as char);
            i += 1;
            continue;
        }
        let start = i;
        if b[i].is_ascii_digit() && (i == 0 || !b[i - 1].is_ascii_alphanumeric()) {
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            let mut float = false;
            if i + 1 < b.len() && b[i] == b'.' && b[i + 1].is_ascii_digit() {
                float = true;
                i += 1;
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                }
            }
            if i < b.len() && matches!(b[i], b'e' | b'E') {
                float = true;
                i += 1;
                if i < b.len() && matches!(b[i], b'+' | b'-') {
                    i += 1;
                }
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                }
            }
            if float {
                let negative = out.ends_with('-');
                if negative {
                    out.pop();
                }
                out.push_str("'$decimal'('");
                if negative {
                    out.push('-');
                }
                out.push_str(&text[start..i]);
                out.push_str("')");
            } else {
                out.push_str(&text[start..i]);
            }
        } else {
            let ch = text[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}
#[test]
fn p7_fork_replay_schema_per_clause_and_detector_plants() {
    let compiler = compiler();
    let work = Scratch::new("schema");
    let script = root().join("tests/da2/replay-schema.pl.in");
    let fork = compiler.stage.join("prolog/ace_to_pl.pl");
    fs::create_dir_all(work.0.join("rust/ckc")).unwrap();
    std::os::unix::fs::symlink(root().join("vendor"), work.0.join("vendor")).unwrap();
    std::os::unix::fs::symlink(
        root().join("rust/ckc/prolog"),
        work.0.join("rust/ckc/prolog"),
    )
    .unwrap();
    copy_tree(
        &root().join("tests/queries/green/positive/tree/guidelines"),
        &work.0.join("guidelines"),
    );
    let schema = work.0.join("guidelines/schema.pl");
    assert!(
        !schema.exists(),
        "schema printer control starts without output"
    );
    native(&work.0, &["compile", "fx"]).require_ok("public schema_bytes printer");
    assert_eq!(
        fs::read(&schema).unwrap(),
        fs::read(root().join("guidelines/schema.pl")).unwrap(),
        "schema printer equals committed bytes"
    );
    let got = run(
        Path::new("swipl"),
        &work.0,
        &[
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            script.to_str().unwrap(),
            "-g",
            "probe_main",
            "-t",
            "halt(9)",
            "--",
            fork.to_str().unwrap(),
            schema.to_str().unwrap(),
        ],
        b"",
    );
    let mut checks = Checks::default();
    checks.require(got.success(), format!("schema probe: {}", got.summary()));
    checks.require(
        got.stdout.starts_with("schema: same "),
        format!(
            "fork list must equal schema clause by clause: {}",
            got.summary()
        ),
    );
    for kind in ["drop", "reorder", "alter"] {
        checks.require(
            got.stdout.contains(&format!("plant: {kind} caught\n")),
            format!("schema detector plant {kind}: {}", got.summary()),
        );
    }
    checks.finish("P7/schema");
}
#[test]
fn p8_driver_wraps_float_tokens_and_drs_values_as_text_atoms() {
    let compiler = compiler();
    let work = Scratch::new("float-driver");
    let ulex = work.0.join("lexicon.ulex");
    write(&ulex, ULEX);
    let old = root().join("tests/da2/drs-seed.pl.in");
    let mut checks = Checks::default();
    for spelling in [
        "1.5".to_string(),
        "0.15".into(),
        "0.00001".into(),
        "0.10000000000000000000000001".into(),
        "-1.5".into(),
        "(- 1.5)".into(),
        format!("0.{}3", "0".repeat(323)),
        format!("0.{}1", "0".repeat(400)),
    ] {
        let ace = EXAMPLE.replace("1.5", &spelling);
        let baseline = compiler.dump(&work.0, ace.as_bytes(), Some(&ulex), Some(&old));
        let current = compiler.dump(&work.0, ace.as_bytes(), Some(&ulex), None);
        baseline.require_ok("baseline driver");
        assert!(
            baseline.stdout.contains("messages([])."),
            "driver float syntax: {spelling}"
        );
        let expected = decimal_envelope(&baseline.stdout);
        assert_ne!(
            expected, baseline.stdout,
            "float branch positive control: {spelling}"
        );
        checks.output(&current, &Run::ok(expected), &spelling);
    }
    checks.finish("P8/float envelope");
}
#[test]
fn p8_existing_corpus_and_fixture_dumps_preserve_nonfloat_bytes() {
    let root = root();
    let compiler = compiler();
    let old = root.join("tests/da2/drs-seed.pl.in");
    let inputs: Vec<_> = files(&root.join("guidelines"))
        .into_iter()
        .chain(files(&root.join("tests")))
        .filter(|p| {
            let s = p.file_name().unwrap().to_str().unwrap();
            s.ends_with(".ace") || s.ends_with(".ace.in")
        })
        .collect();
    assert!(inputs.len() >= 1147, "seed input census");
    let failures = parallel(&inputs, |ace| {
        let work = Scratch::new("dump-preservation");
        let bytes = fs::read(ace).unwrap();
        let ulex = ulex_for(ace);
        let baseline = compiler.dump(&work.0, &bytes, ulex.as_deref(), Some(&old));
        let current = compiler.dump(&work.0, &bytes, ulex.as_deref(), None);
        let mut want = baseline;
        want.stdout = decimal_envelope(&want.stdout);
        // SWI exception locations name the selected driver; the printed envelope stays literal.
        want.stderr = want.stderr.replace(
            old.to_str().unwrap(),
            root.join("rust/ckc/prolog/drs_dump.pl").to_str().unwrap(),
        );
        if current == want {
            vec![]
        } else {
            vec![format!(
                "{}: got {}; want {}",
                ace.strip_prefix(&root).unwrap().display(),
                current.summary(),
                want.summary()
            )]
        }
    });
    println!(
        "DA2 P8 preservation: {} inputs, {} failures",
        inputs.len(),
        failures.len()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
#[test]
fn p9_corpus_and_six_query_products_answers_traces_unchanged() {
    let root = root();
    let compiler = compiler();
    let inputs: Vec<_> = files(&root.join("guidelines"))
        .into_iter()
        .filter(|p| p.extension().is_some_and(|s| s == "ace"))
        .collect();
    assert!(inputs.len() >= 392, "seed corpus + queries census");
    let failures = parallel(&inputs, |ace| {
        let query = ace.parent().unwrap().file_name().unwrap() == "queries";
        let id = ace.file_stem().unwrap().to_str().unwrap();
        let ulex = ulex_for(ace);
        let table = table_for(ace);
        let got = compiler.compile(
            &root,
            id,
            &fs::read(ace).unwrap(),
            ulex.as_deref(),
            table.as_deref(),
            if query { "question" } else { "doc" },
        );
        let pl = if query {
            ace.parent().unwrap().join("pl").join(format!("{id}.pl"))
        } else {
            ace.parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("pl")
                .join(format!("{id}.pl"))
        };
        let expected = Run::ok(read(pl));
        if got == expected {
            vec![]
        } else {
            vec![format!(
                "{id}: got {}; want {}",
                got.summary(),
                expected.summary()
            )]
        }
    });
    let mut checks = Checks {
        total: inputs.len(),
        failures,
    };
    let queries: Vec<_> = inputs
        .iter()
        .filter(|p| p.parent().unwrap().file_name().unwrap() == "queries")
        .collect();
    assert_eq!(queries.len(), 6);
    let work = Scratch::new("query-preservation");
    write(work.0.join("empty.proof"), "");
    for ace in queries {
        let id = ace.file_stem().unwrap().to_str().unwrap();
        let qdir = ace.parent().unwrap();
        let gdir = qdir.parent().unwrap();
        let manifest: String = files(&gdir.join("pl"))
            .iter()
            .filter(|p| p.extension().is_some_and(|s| s == "pl"))
            .map(|p| {
                format!(
                    "{}\t{}\n",
                    p.display(),
                    work.0.join("empty.proof").display()
                )
            })
            .collect();
        write(work.0.join("m.tsv"), manifest);
        let q = qdir.join("pl").join(format!("{id}.pl"));
        let a = qdir.join("answers").join(format!("{id}.pl"));
        let t = qdir.join("traces").join(format!("{id}.pl"));
        checks.output(
            &native(&work.0, &["v1", "answer", "m.tsv", q.to_str().unwrap()]),
            &Run::ok(read(&a)),
            &format!("{id}/answers"),
        );
        checks.output(
            &native(
                &work.0,
                &[
                    "v1",
                    "trace",
                    "m.tsv",
                    q.to_str().unwrap(),
                    a.to_str().unwrap(),
                ],
            ),
            &Run::ok(read(&t)),
            &format!("{id}/traces"),
        );
    }
    checks.finish("P9/corpus+queries");
}
#[test]
fn p9_every_red_green_probe_and_proof_pin_unchanged() {
    let root = root();
    let compiler = compiler();
    let paths: Vec<_> = files(&root.join("tests/green"))
        .into_iter()
        .chain(files(&root.join("tests/red")))
        .filter(|p| p.extension().is_some_and(|s| s == "ace"))
        .collect();
    assert!(paths.len() >= 237, "seed red + green census");
    let failures = parallel(&paths, |ace| {
        let rel = ace.strip_prefix(&root).unwrap();
        let stem = ace.file_stem().unwrap().to_str().unwrap();
        let green = ace.parent().unwrap().file_name().unwrap() == "green";
        let id = if green { stem } else { "red-probe" };
        let table = rel.with_extension("vocabulary.tsv");
        let ulex = rel.with_extension("ulex");
        let tab = if root.join(&table).is_file() {
            table.to_str().unwrap()
        } else {
            "-"
        };
        let lex = if root.join(&ulex).is_file() {
            ulex.to_str().unwrap()
        } else {
            "-"
        };
        let argv = ace.with_extension("argv");
        let mut tail = if argv.is_file() {
            read(argv)
                .split_whitespace()
                .map(|s| {
                    s.replace("@TREE@", compiler.stage.to_str().unwrap())
                        .replace("@ID@", id)
                        .replace("@TABLE@", tab)
                        .replace("@ULEX@", lex)
                })
                .collect::<Vec<_>>()
        } else {
            let mut tail = Vec::new();
            if stem.contains("--query-") {
                tail.push("question".into());
            }
            tail.extend([
                compiler.stage.to_str().unwrap().into(),
                id.into(),
                tab.into(),
                lex.into(),
            ]);
            tail
        };
        let bytes = fs::read(ace).unwrap();
        let got = compiler.custom(&root, &tail, &bytes);
        let pin = read(ace.with_extension("expect"));
        let want = if green {
            Run::ok(pin)
        } else {
            let class = stem.split_once("--").unwrap().0;
            Run::err(
                if ["usage", "vocabulary_load", "ulex_load"].contains(&class) {
                    2
                } else {
                    1
                },
                pin,
            )
        };
        let mut failures = Vec::new();
        if got != want {
            failures.push(format!(
                "{stem}: got {}; want {}",
                got.summary(),
                want.summary()
            ));
        }
        let proof = ace.with_extension("proof.expect");
        if proof.is_file() {
            tail.push("proof".into());
            let got = compiler.custom(&root, &tail, &bytes);
            let want = Run::ok(read(proof));
            if got != want {
                failures.push(format!(
                    "{stem}/proof: got {}; want {}",
                    got.summary(),
                    want.summary()
                ));
            }
        }
        failures
    });
    println!(
        "DA2 P9 probes: {} inputs, {} failures",
        paths.len(),
        failures.len()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
fn field(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        out.push(if ch == '\\' {
            match chars.next().expect("complete TSV escape") {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\\' => '\\',
                other => panic!("case-table escape {other}"),
            }
        } else {
            ch
        });
    }
    out
}
struct CertBase {
    kind: String,
    id: String,
    ace: PathBuf,
    ulex: Option<PathBuf>,
    table: Option<PathBuf>,
    pl: String,
    dump: String,
}
impl CertBase {
    fn certify(&self, work: &Path, pl: &str) -> Run {
        write(work.join("input.dump"), &self.dump);
        write(work.join("input.pl"), pl);
        let mut args = vec![
            "certify-one",
            &self.kind,
            &self.id,
            self.ace.to_str().unwrap(),
            self.ulex.as_deref().map_or("-", |p| p.to_str().unwrap()),
            "input.dump",
            "input.pl",
        ];
        if let Some(table) = &self.table {
            args.push(table.to_str().unwrap());
        }
        native(work, &args)
    }
}
#[test]
fn p9_all_committed_certification_cases_unchanged() {
    let root = root();
    let compiler = compiler();
    let work = Scratch::new("certify-cases");
    let table = read(root.join("tests/certify/cases.tsv"));
    let rows: Vec<Vec<String>> = table
        .lines()
        .skip(1)
        .map(|line| line.split('\t').map(field).collect())
        .collect();
    assert!(rows.len() >= 55, "seed certification mutation census");
    assert!(rows.iter().all(|row| row.len() == 8), "eight case columns");
    let mut bases = BTreeMap::new();
    let mut checks = Checks::default();
    for row in &rows {
        let target = &row[2];
        if bases.contains_key(target) {
            continue;
        }
        let path = root.join(target);
        let id = path.file_stem().unwrap().to_str().unwrap().to_string();
        let parts: Vec<_> = target.split('/').collect();
        assert_eq!(parts[0], "guidelines");
        let guideline = root.join("guidelines").join(parts[1]);
        let ace = if row[1] == "query" {
            guideline.join("queries").join(format!("{id}.ace"))
        } else {
            assert_eq!(row[1], "doc");
            guideline.join("ace").join(format!("{id}.ace"))
        };
        let ulex = ulex_for(&ace);
        let vocabulary = table_for(&ace);
        let bytes = fs::read(&ace).unwrap();
        let dump = compiler.dump(&work.0, &bytes, ulex.as_deref(), None);
        dump.require_ok(&format!("{id}/dump"));
        assert!(
            dump.stdout.contains("messages([])."),
            "{id}/parser diagnostics"
        );
        let second = compiler.dump(&work.0, &bytes, ulex.as_deref(), None);
        assert_eq!(second, dump, "{id}/duplicate fresh dump");
        let base = CertBase {
            kind: row[1].clone(),
            id,
            ace,
            ulex,
            table: vocabulary,
            pl: read(path),
            dump: dump.stdout,
        };
        let got = base.certify(&work.0, &base.pl);
        checks.require(
            got.success() && !got.stdout.is_empty(),
            format!("{target}/unmodified control: {}", got.summary()),
        );
        bases.insert(target.clone(), base);
    }
    for row in &rows {
        let base = &bases[&row[2]];
        assert_eq!(base.kind, row[1], "one mode per target");
        let changed = match row[3].as_str() {
            "replace-first" => {
                assert!(base.pl.contains(&row[4]), "{} replacement anchor", row[0]);
                base.pl.replacen(&row[4], &row[5], 1)
            }
            "swap-lines" => {
                let mut lines: Vec<_> = base.pl.split_inclusive('\n').collect();
                lines.swap(row[4].parse().unwrap(), row[5].parse().unwrap());
                lines.concat()
            }
            "truncate-from" => {
                base.pl[..base.pl.find(&row[4]).expect("truncation anchor")].to_string()
            }
            op => panic!("case-table operation {op}"),
        };
        assert_ne!(changed, base.pl, "{} mutation must change bytes", row[0]);
        let got = base.certify(&work.0, &changed);
        checks.output(&got, &Run::err(row[6].parse().unwrap(), &row[7]), &row[0]);
    }
    println!(
        "DA2 P9 certification table: {} edits, {} unmodified controls, duplicate fresh dumps",
        rows.len(),
        bases.len()
    );
    checks.finish("P9/certify-cases");
}
