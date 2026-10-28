use super::model::*;
use super::support::*;
use ckc_kernel::contract;
use std::fs;
use std::path::PathBuf;

fn load(s: &Scratch, files: &[PathBuf]) -> Run {
    let names = files
        .iter()
        .map(|p| quoted(p.to_str().unwrap()))
        .collect::<Vec<_>>()
        .join(",");
    swi(
        &s.0,
        &format!("consult([{names}]),writeln(loaded({})),halt.", files.len()),
    )
}

#[test]
fn p1_corpus_and_each_consumer_world_load_without_warnings() {
    let s = Scratch::new("load");
    let schema = root().join("guidelines/schema.pl");
    let mut old = vec![schema.clone()];
    let mut repaired = vec![schema.clone()];
    for id in ["cdc2022-opioid-s33-01", "cdc2022-opioid-s42-15"] {
        let p = format!("guidelines/cdc-2022-opioid/pl/{id}.pl");
        let letter = baseline(&p);
        old.push(s.file(&format!("old-{id}.pl"), &letter));
        repaired.push(s.file(&format!("model-{id}.pl"), anonymize(&letter)));
    }
    load(&s, &repaired).require("independent anonymous spelling removes baseline warnings");
    let control = load(&s, &old);
    let warnings = String::from_utf8_lossy(&control.err)
        .lines()
        .filter(|l| l.contains("Singleton variables:"))
        .count();
    assert_eq!(
        control.rc,
        0,
        "historical warning control: {}",
        control.description()
    );
    assert_eq!(
        warnings,
        65,
        "258ddc44 positive control = 65 singleton diagnostics: {}",
        control.description()
    );
    let mut files = vec![schema];
    files.extend(corpus().iter().map(|p| root().join(p)));
    let mut checks = Checks::default();
    for world in ["none", "dose", "adult", "minor", "cancer"] {
        let mut world_files = files.clone();
        match world {
            "none" => (),
            "dose" => world_files.push(
                root().join("tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl"),
            ),
            name => world_files.push(s.file(
                &format!("{name}.pl"),
                fs::read(root().join(format!("tests/consumer/{name}.pl.in"))).unwrap(),
            )),
        }
        let got = load(&s, &world_files);
        checks.require(
            got.clean() && got.out == format!("loaded({})\n", world_files.len()).as_bytes(),
            format!(
                "world={world}, {} loaded paths: {}",
                world_files.len(),
                got.description()
            ),
        );
    }
    checks.finish("P1 warning-free corpus + worlds");
}

// The live laws on a committed `_`-spelled file: no variable that occurs once keeps a letter;
// the suite's reader and the kernel's agree; respelling the reading gives the file back; the
// reading numbers each line's variables in first-occurrence order (the writer's letters).
// Returns one failure label per broken law.
fn law_failures(raw: &[u8]) -> Vec<&'static str> {
    let reading = read(raw).text;
    let mut out = Vec::new();
    if singletons(raw) != 0 {
        out.push("letter singleton");
    }
    if contract::v1_read(raw) != reading {
        out.push("kernel read differs");
    }
    if anonymize(&reading) != raw {
        out.push("respelling differs");
    }
    // Bytewise (a red fixture holds invalid UTF-8): each line's new letters = A, B, C, … in order.
    let canonical = reading.split(|b| *b == b'\n').all(|line| {
        let mut names: Vec<&[u8]> = Vec::new();
        runs(line).into_iter().all(|span| {
            let word = &line[span];
            if !word[0].is_ascii_uppercase() || names.contains(&word) {
                return true;
            }
            names.push(word);
            word == variable(names.len() - 1).as_bytes()
        })
    });
    if !canonical {
        out.push("non-canonical letters");
    }
    out
}

fn check_laws(checks: &mut Checks, path: &str, raw: &[u8]) {
    for law in law_failures(raw) {
        checks.require(false, format!("{path}: {law}"));
    }
    checks.require(true, format!("{path}: laws"));
}

// Historical oracle on fixed commits (never the live tree): every corpus document SGW
// re-spelled (4a257599, the first `_`-spelled corpus) reads back as the letter-spelled bytes
// the pre-SGW compiler wrote (258ddc44), in both readers, and respells from them.
#[test]
fn p3_sgw_corpus_reads_as_its_letter_spelled_predecessor() {
    let mut checks = Checks::default();
    let inventory = |rev: &str| {
        String::from_utf8(git(&["ls-tree", "-r", "--name-only", rev, "guidelines"]))
            .unwrap()
            .lines()
            .filter(|p| p.split('/').count() == 4 && p.contains("/pl/") && p.ends_with(".pl"))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    let paths = inventory(SGW);
    assert!(
        !paths.is_empty() && paths == inventory(BASE),
        "SGW corpus inventory differs from {BASE}"
    );
    let mut unchanged = 0;
    for path in &paths {
        let raw = git(&["show", &format!("{SGW}:{path}")]);
        let letter = baseline(path);
        checks.bytes(&read(&raw).text, &letter, &format!("{path} independent read"));
        checks.bytes(&contract::v1_read(&raw), &letter, &format!("{path} kernel read"));
        checks.bytes(&raw, &anonymize(&letter), &format!("{path} D1 spelling"));
        if anonymous_count(&raw) == 0 {
            unchanged += 1;
            checks.bytes(&contract::v1_read(&raw), &raw, &format!("{path} no-alias identity"));
        }
    }
    assert!(unchanged > 0, "no-alias positive population");
    println!("SGW P3: {} documents at {SGW} vs {BASE}; {unchanged} no-alias identities", paths.len());
    checks.finish("P3 historical replay");
}

// Live laws on every committed emission surface: the corpus, the fixture trees, the green
// pins. Each law fires on a planted file first (the checker's own red inputs).
#[test]
fn p2_committed_emission_surfaces_obey_the_spelling_law() {
    let mut paths = corpus();
    let mut fixture_count = 0;
    let mut green_count = 0;
    for path in tracked("tests") {
        let fixture = path.contains("/tree/guidelines/")
            && path.contains("/pl/")
            && !path.contains("/queries/")
            && path.ends_with(".pl");
        let green = path.starts_with("tests/green/")
            && path.ends_with(".expect")
            && !path.ends_with(".proof.expect");
        if fixture || green {
            fixture_count += usize::from(fixture);
            green_count += usize::from(green);
            paths.push(path);
        }
    }
    assert!(
        fixture_count > 0 && green_count > 0,
        "fixture + green positive populations"
    );
    let planted_line = |l: &&str| l.contains(" :- ") && l.contains("(actual,A,") && l.contains(",_,");
    let (sample, text) = paths
        .iter()
        .map(|p| (p, String::from_utf8(fs::read(root().join(p)).unwrap()).unwrap()))
        .find(|(_, text)| text.lines().any(|l| planted_line(&l)))
        .expect("a document with a rule line holding a letter A and an anonymous variable");
    assert!(law_failures(text.as_bytes()).is_empty(), "{sample}: positive control");
    let line = text.lines().find(planted_line).unwrap();
    let plant = |from: &str, to: &str| text.replacen(line, &line.replacen(from, to, 1), 1).into_bytes();
    for (law, planted) in [
        ("letter singleton", plant(",_,", ",Q,")),
        ("respelling differs", plant("(actual,A,", "(actual,_,")),
        ("non-canonical letters", text.replacen(line, &line.replace('A', "Z"), 1).into_bytes()),
    ] {
        assert!(
            law_failures(&planted).contains(&law),
            "{law} plant not caught: {:?}",
            law_failures(&planted)
        );
    }
    let mut checks = Checks::default();
    for path in &paths {
        let raw = fs::read(root().join(path)).unwrap();
        check_laws(&mut checks, path, &raw);
    }
    println!(
        "SGW P2 surfaces: {} corpus, {fixture_count} fixture documents, {green_count} green pins",
        corpus().len()
    );
    checks.finish("P2 committed emission surfaces");
}
