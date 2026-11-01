// Contract ws P8: the positive fragment controls. The Phase example (`some`
// X + NAF ¬`at least 1`) and the rational interval case (`less 2` + NAF
// ¬`at most 1`) lie in the fragment; the walk selects −1 and 3/2. Fixtures
// tests/ws-controls/ = the ACE, its APE dump and the compiled document
// (`ckc certify-one doc` accepts each). + D8: `ckc certify` refuses a fork
// payload that differs from the certified one; + P4's fork half: the fork's
// replay keeps a proof found after a pruned branch. CKC_WS_TEST_BIN selects a
// prebuilt executable, CKC_WS_COMPILER a compiler copy for red replay.
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/ws-controls")
        .join(name)
}

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn program() -> PathBuf {
    std::env::var_os("CKC_WS_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")))
}

fn triple(out: std::process::Output) -> (Option<i32>, String, String) {
    (
        out.status.code(),
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
    )
}

// (rc, stdout, stderr) of `ckc certify-one walks ws-<id> …`.
fn walks(id: &str) -> (Option<i32>, String, String) {
    let out = Command::new(program())
        .args(["certify-one", "walks", &format!("ws-{id}")])
        .arg(fixture(&format!("{id}.ace.in")))
        .arg(fixture("lexicon.ulex.in"))
        .arg(fixture(&format!("{id}.dump.in")))
        .arg(fixture(&format!("{id}.pl.in")))
        .output()
        .unwrap();
    triple(out)
}

#[test]
fn phase_example_lies_in_the_fragment_and_selects_minus_one() {
    assert_eq!(
        walks("w1"),
        (
            Some(0),
            "'$guideline_walk'(1,variant(1),[[1,-1,0,rational(1,2),2]],winner([1]),fragment(true)).\n"
                .to_owned(),
            String::new()
        )
    );
}

#[test]
fn rational_interval_case_lies_in_the_fragment_and_selects_three_halves() {
    assert_eq!(
        walks("w2c"),
        (
            Some(0),
            "'$guideline_walk'(1,variant(1),[[1,-1,0,rational(1,2),rational(3,2)]],winner([4]),fragment(true)).\n"
                .to_owned(),
            String::new()
        )
    );
}

// A repository root holding guideline `ws` = the w1 control (document ws-w1), a
// fork copy the test may edit, and Clex + the M6 driver by symlink.
fn certify_root(dir: &Path) -> PathBuf {
    let root = dir.join("root");
    let ws = root.join("guidelines/ws");
    for sub in ["ace", "pl"] {
        std::fs::create_dir_all(ws.join(sub)).unwrap();
    }
    for sub in ["rust/ckc", "vendor"] {
        std::fs::create_dir_all(root.join(sub)).unwrap();
    }
    for (from, to) in [
        ("w1.ace.in", "ace/ws-w1.ace"),
        ("w1.pl.in", "pl/ws-w1.pl"),
        ("lexicon.ulex.in", "lexicon.ulex"),
    ] {
        std::fs::copy(fixture(from), ws.join(to)).unwrap();
    }
    let copy = Command::new("cp")
        .arg("-a")
        .arg(repo().join("vendor/ape"))
        .arg(root.join("vendor/ape"))
        .status()
        .unwrap();
    assert!(copy.success(), "fork copy");
    std::os::unix::fs::symlink(repo().join("vendor/clex"), root.join("vendor/clex")).unwrap();
    std::os::unix::fs::symlink(repo().join("rust/ckc/prolog"), root.join("rust/ckc/prolog"))
        .unwrap();
    root
}

fn certify(root: &Path, locale: &str) -> (Option<i32>, String, String) {
    triple(
        Command::new(program())
            .args(["certify", "ws"])
            .current_dir(root)
            .env("LC_ALL", locale)
            .output()
            .unwrap(),
    )
}

// Contract ws D8: a certified document's payload = the fork's own proof run. Red
// input = the fork with its candidates after the default descending: it selects
// 1/2 for w1 where K2 selects −1, and both payloads replay. The fork's stderr
// stays `ckc check`'s law: under a C locale its source-load warnings leave the
// payload, and so the certification, alone.
#[test]
fn certify_refuses_a_fork_payload_that_differs() {
    let dir = std::env::temp_dir().join(format!("ckc-ws-d8-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let root = certify_root(&dir);
    let positive = certify(&root, "C.UTF-8");
    let c_locale = certify(&root, "C");
    let fork = root.join("vendor/ape/prolog/ace_to_pl.pl");
    let text = std::fs::read_to_string(&fork).unwrap();
    let anchor = "    ws_ascending(Kept, Rest).\n";
    assert_eq!(text.matches(anchor).count(), 1, "candidate order anchor");
    std::fs::write(
        &fork,
        text.replace(
            anchor,
            "    ws_ascending(Kept, Asc),\n    reverse(Asc, Rest).\n",
        ),
    )
    .unwrap();
    let red = certify(&root, "C.UTF-8");
    std::fs::remove_dir_all(&dir).unwrap();
    let ok = (
        Some(0),
        "ckc: certify ok ws 1 documents 0 queries\n".to_owned(),
        String::new(),
    );
    assert_eq!(positive, ok);
    assert_eq!(c_locale, ok);
    assert_eq!(
        red,
        (
            Some(1),
            String::new(),
            "ckc: certify: ws-w1: fork_payload_mismatch.\n".to_owned()
        )
    );
}

// The fork's `ws_replay` on each head of the world in tests/ws-unit/positive-retry.pl.in
// (a recursive clause ahead of the two facts that prove it), all clauses or its rules
// alone: one `yes` | `no` line per head.
fn fork_replay(rules_only: bool) -> (Option<i32>, String, String) {
    let compiler = std::env::var_os("CKC_WS_COMPILER")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("vendor/ape/prolog/ace_to_pl.pl"));
    let head = |r: u8| {
        format!(
            "guideline_entity(actual,'$guideline_id'(product,'ws-p4c',1,ref({r}),[]),bird,countable)"
        )
    };
    let world = repo()
        .join("tests/ws-unit/positive-retry.pl.in")
        .display()
        .to_string()
        .replace('\\', "\\\\")
        .replace('\'', "\\'");
    let goal = format!(
        "open('{world}',read,S),repeat,read_term(S,C,[]),\
         (C==end_of_file->!,close(S);({}==all;C=(_:-_))->assertz(ace_to_pl_proof_world:C),fail;fail),\
         forall(member(H,[{},{}]),(ace_to_pl:ws_replay(H)->writeln(yes);writeln(no))),halt(0)",
        if rules_only { "rules" } else { "all" },
        head(1),
        head(2)
    );
    let swipl = std::env::var_os("SWIPL").unwrap_or_else(|| "swipl".into());
    triple(
        Command::new(swipl)
            .args(["-q", "-f", "none", "-F", "none", "-s"])
            .arg(compiler)
            .args(["-g", &goal, "-t", "halt(9)"])
            .env("LC_ALL", "C.UTF-8")
            .output()
            .unwrap(),
    )
}

// Contract ws P4, the fork half of the positive-retry control: the depth cut prunes the
// recursive branch, then the fact proves the head, and the replay keeps that proof; with
// the facts gone, the pruned search proves nothing.
#[test]
fn fork_replay_keeps_a_proof_found_after_a_pruned_branch() {
    assert_eq!(
        fork_replay(false),
        (Some(0), "yes\nyes\n".to_owned(), String::new())
    );
    assert_eq!(
        fork_replay(true),
        (Some(0), "no\nno\n".to_owned(), String::new())
    );
}
