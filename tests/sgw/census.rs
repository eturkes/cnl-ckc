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

#[test]
fn p3_all_committed_documents_read_as_258ddc44() {
    let mut checks = Checks::default();
    let paths = corpus();
    let old_paths = String::from_utf8(git(&["ls-tree", "-r", "--name-only", BASE, "guidelines"]))
        .unwrap()
        .lines()
        .filter(|p| p.split('/').count() == 4 && p.contains("/pl/") && p.ends_with(".pl"))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert!(
        paths == old_paths,
        "SGW corpus inventory differs: current={} baseline={} first={:?}",
        paths.len(),
        old_paths.len(),
        paths.iter().zip(&old_paths).find(|(a, b)| a != b)
    );
    let mut unchanged = 0;
    for path in &paths {
        let raw = fs::read(root().join(path)).unwrap();
        let letter = baseline(path);
        checks.bytes(
            &read(&raw).text,
            &letter,
            &format!("{path} independent read"),
        );
        checks.bytes(
            &contract::v1_read(&raw),
            &letter,
            &format!("{path} kernel read"),
        );
        if anonymous_count(&raw) == 0 {
            unchanged += 1;
            checks.bytes(
                &contract::v1_read(&raw),
                &raw,
                &format!("{path} no-alias identity"),
            );
        }
    }
    assert!(unchanged > 0, "no-alias positive population");
    println!(
        "SGW P3: {} documents; {unchanged} no-alias identities",
        paths.len()
    );
    checks.finish("P3 corpus replay");
}

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
    let mut checks = Checks::default();
    for path in &paths {
        let raw = fs::read(root().join(path)).unwrap();
        let letter = baseline(path);
        checks.require(
            singletons(&raw) == 0,
            format!("{path}: {} letter singletons remain", singletons(&raw)),
        );
        checks.bytes(&read(&raw).text, &letter, &format!("{path} read identity"));
        checks.bytes(
            &raw,
            &anonymize(&letter),
            &format!("{path} exact D1 spelling"),
        );
    }
    println!(
        "SGW P2 surfaces: {} corpus, {fixture_count} fixture documents, {green_count} green pins",
        corpus().len()
    );
    checks.finish("P2 committed emission surfaces");
}
