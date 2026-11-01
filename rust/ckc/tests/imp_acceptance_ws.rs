// Acceptance oracle = spec Decisions Witness selection.
// CKC_IMP_WS_ID2_TEST_BIN selects a prebuilt ckc; the compiler comes from this tree.
#[path = "support/imp_ws_id2.rs"]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use support::*;

const FIXTURES: &str = "tests/imp-acceptance/ws";

fn fixture(name: &str) -> String {
    fs::read_to_string(root().join(FIXTURES).join(name)).unwrap()
}
fn swi() -> PathBuf {
    std::env::var_os("SWIPL")
        .map(PathBuf::from)
        .unwrap_or_else(|| "swipl".into())
}
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for e in fs::read_dir(from).unwrap() {
        let e = e.unwrap();
        let kind = e.file_type().unwrap();
        assert!(!kind.is_symlink(), "source symlink: {}", e.path().display());
        if kind.is_dir() {
            copy_tree(&e.path(), &to.join(e.file_name()));
        } else {
            fs::copy(e.path(), to.join(e.file_name())).unwrap();
        }
    }
}
struct Compiler {
    stage: PathBuf,
}
impl Compiler {
    fn new(s: &Scratch) -> Self {
        let version = run(
            &swi(),
            &s.0,
            &[
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-g",
                "current_prolog_flag(version_data,swi(A,B,C,_)),format('version ~d.~d.~d ~n',[A,B,C]),halt(0)",
                "-t",
                "halt(9)",
            ],
            b"",
            "SWI pin",
        );
        assert_eq!(version.rc, 0, "SWI pin: {}", version.summary());
        assert!(version.stdout.contains("version 10.0.2 "));
        let stage = s.0.join("ape");
        copy_tree(&root().join("vendor/ape"), &stage);
        fs::copy(
            root().join("vendor/clex/clex_lexicon.pl"),
            stage.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .unwrap();
        for goal in [
            format!(
                "working_directory(_,{}),[fit_to_plp],halt(0)",
                quoted(path(&stage.join("prolog/parser")))
            ),
            format!(
                "style_check(-discontiguous),qcompile({}),halt(0)",
                quoted(path(&stage.join("prolog/lexicon/clex_lexicon.pl")))
            ),
        ] {
            let got = run(
                &swi(),
                &s.0,
                &[
                    "-q", "-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt(9)",
                ],
                b"",
                "compiler stage",
            );
            assert_eq!(got.rc, 0, "compiler stage: {}", got.summary());
        }
        Self { stage }
    }
    fn compile(&self, s: &Scratch, id: &str, ace: &str, mode: &str, label: &str) -> Run {
        let script = self.stage.join("prolog/ace_to_pl.pl");
        let ulex = s.0.join("guidelines/fx/lexicon.ulex");
        let mut args = vec![
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            path(&script),
            "-g",
            "main",
            "-t",
            "halt(9)",
            "--",
            path(&self.stage),
            id,
            "-",
            path(&ulex),
        ];
        if mode == "proof" {
            args.push("proof");
        }
        run(&swi(), &s.0, &args, ace.as_bytes(), label)
    }
    fn dump(&self, s: &Scratch, ace: &str) -> Run {
        let script = root().join("rust/ckc/prolog/drs_dump.pl");
        let ulex = s.0.join("guidelines/fx/lexicon.ulex");
        run(
            &swi(),
            &s.0,
            &[
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "-s",
                path(&script),
                "-g",
                "main",
                "-t",
                "halt(9)",
                "--",
                path(&self.stage),
                path(&ulex),
            ],
            ace.as_bytes(),
            "original ACE dump",
        )
    }
}

// The old fork refuses to emit a satisfiable rule before native replay can run.
// A cousin with a different NAF noun supplies the projection's existing wire form.
// Replacing that noun + source custody is checked independently by certify-one.
fn cousin(ace: &str) -> (String, &'static str, &'static str) {
    let (before, rest) = ace.split_once("does not provably contain ").unwrap();
    let (naf, after) = rest.split_once(" then ").unwrap();
    let (naf, from, to) = if naf.contains("oxycodone-") {
        (
            naf.replace("oxycodone-", "codeine-"),
            ",'codeine-milligram-per-day',countable)",
            ",'oxycodone-milligram-per-day',countable)",
        )
    } else {
        assert!(naf.ends_with(" cat"));
        (
            naf.replace(" cat", " bird"),
            ",bird,countable)",
            ",cat,countable)",
        )
    };
    (
        format!("{before}does not provably contain {naf} then {after}"),
        from,
        to,
    )
}
fn witness_case(
    c: &mut Checks,
    s: &Scratch,
    compiler: &Compiler,
    id: &str,
    ace: &str,
    satisfiable: bool,
) {
    let source = s.file(&format!("guidelines/fx/ace/{id}.ace"), ace);
    let emitted = compiler.compile(s, id, ace, "doc", &format!("{id} compile"));
    c.require(
        if satisfiable {
            emitted.rc == 0
        } else {
            emitted.refused()
        },
        &format!("{id} compiler acceptance={satisfiable}"),
        &emitted,
    );
    let projected = if emitted.rc == 0 {
        emitted.stdout
    } else {
        let (safe, from, to) = cousin(ace);
        let control = compiler.compile(s, id, &safe, "doc", &format!("{id} cousin control"));
        assert_eq!(control.rc, 0, "cousin fixture: {}", control.summary());
        let p = substitute(
            control.stdout,
            &format!("ace_sha256({})", atom(&sha(&safe))),
            &format!("ace_sha256({})", atom(&sha(ace))),
        );
        let p = substitute(
            p,
            &format!("% S1: {}", safe.trim_end()),
            &format!("% S1: {}", ace.trim_end()),
        );
        assert!(p.contains(from), "NAF noun anchor");
        p.replace(from, to)
    };
    let pl = s.file(&format!("guidelines/fx/pl/{id}.pl"), projected);
    let dump = compiler.dump(s, ace);
    assert_eq!(dump.rc, 0, "fixture ACE dump: {}", dump.summary());
    let dump = s.file(&format!("{id}.dump"), dump.stdout);
    let ulex = s.0.join("guidelines/fx/lexicon.ulex");
    let certified = native(
        s,
        &[
            "certify-one",
            "doc",
            id,
            path(&source),
            path(&ulex),
            path(&dump),
            path(&pl),
        ],
        &format!("{id} certification"),
    );
    c.require(
        certified.rc == 0,
        &format!("{id} projection certifies"),
        &certified,
    );
    if certified.rc != 0 {
        return;
    }
    let proof = s.file(&format!("{id}.proof"), certified.stdout);
    let m = s.file(
        &format!("{id}.tsv"),
        format!("{}\t{}\n", pl.display(), proof.display()),
    );
    let replay = native(
        s,
        &["v1", "aggregate-check", path(&m)],
        &format!("{id} native replay"),
    );
    c.require(
        if satisfiable {
            replay.rc == 0
        } else {
            replay.refused() && replay.names(id)
        },
        &format!("{id} native acceptance={satisfiable}; refusal names document"),
        &replay,
    );
}
fn witness_fixture(id: &str, satisfiable: bool) {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new(id);
    s.file("guidelines/fx/lexicon.ulex", fixture("lexicon.ulex.in"));
    let compiler = Compiler::new(&s);
    let mut c = Checks::default();
    if !satisfiable {
        let (safe, _, _) = cousin(&fixture(&format!("{id}.ace.in")));
        witness_case(&mut c, &s, &compiler, "ws-positive-control", &safe, true);
    }
    witness_case(
        &mut c,
        &s,
        &compiler,
        id,
        &fixture(&format!("{id}.ace.in")),
        satisfiable,
    );
    c.finish(id);
}

#[test]
fn w1_phase_example_compiles_certifies_and_replays() {
    witness_fixture("w1", true);
}
#[test]
fn w2a_gap_below_100() {
    witness_fixture("w2a", true);
}
#[test]
fn w2b_defined_divisor_below_one() {
    // `some` sets no bound; any defined divisor below 1 is valid, not only 1/2.
    witness_fixture("w2b", true);
}
#[test]
fn w2c_gap_below_2() {
    witness_fixture("w2c", true);
}
#[test]
fn w2d_gap_above_0() {
    witness_fixture("w2d", true);
}
#[test]
fn w3_contradictory_naf_still_fails_replay() {
    witness_fixture("w3", false);
}
#[test]
fn w4_dense_interval_completeness_property() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("w4");
    s.file("guidelines/fx/lexicon.ulex", fixture("lexicon.ulex.in"));
    let compiler = Compiler::new(&s);
    let mut c = Checks::default();
    let mut count = 0;
    for (lower, upper) in [1, 2, 49, 99]
        .into_iter()
        .flat_map(|lower| [lower - 1, lower, lower + 1].map(|upper| (lower, upper)))
        .chain([(0, 1)])
    {
        for expression in ["X * 1", "1 / X"] {
            let noun = if upper == 1 {
                "milligram"
            } else {
                "milligrams"
            };
            let denied_noun = if lower == 1 {
                "milligram"
            } else {
                "milligrams"
            };
            let ace = format!(
                "If an opioid-dosage contains less than {upper} oxycodone-{noun}-per-day X and does not provably contain at most {lower} oxycodone-{denied_noun}-per-day then the opioid-dosage includes some MME-units-per-day Y and Y = {expression}.\n"
            );
            // Dense-domain oracle: lower < X < upper is inhabited iff lower < upper.
            // All lower bounds are nonnegative, so any inhabitant is a defined divisor.
            witness_case(
                &mut c,
                &s,
                &compiler,
                &format!("w4-{count}"),
                &ace,
                lower < upper,
            );
            count += 1;
        }
    }
    assert_eq!(count, 26, "generated domain");
    println!("IMP W4 generated={count}");
    c.finish("W4 dense interval completeness");
}
