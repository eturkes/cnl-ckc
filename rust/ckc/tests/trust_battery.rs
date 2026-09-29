// Trust-audit hostile battery (.agent/contracts/harness.md H2; review R-07).
// Each plant mutates a private copy of the workspace (target/ + vendor/
// excluded) and must end in rc 1 with a stderr line matching its pattern; the
// clean copy must pass with the meter alone. p1–p15 + the control carry the
// M5.1 battery's patterns byte for byte; p16–p22 = R-07 vectors.
// CKC_TRUST_TEST_BIN selects the audited executable (red replay against stubs).
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .unwrap()
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        if name == "target" || name == "vendor" {
            continue;
        }
        let kind = entry.file_type().unwrap();
        if kind.is_dir() {
            copy_tree(&entry.path(), &to.join(&name));
        } else if kind.is_file() {
            fs::copy(entry.path(), to.join(&name)).unwrap();
        }
    }
}

fn append(rust: &Path, rel: &str, data: &str) {
    let path = rust.join(rel);
    let mut bytes = fs::read(&path).unwrap();
    bytes.extend_from_slice(data.as_bytes());
    fs::write(path, bytes).unwrap();
}

// Replaces the single occurrence of `old`; any other count fails the plant.
fn replace_once(rust: &Path, rel: &str, old: &str, new: &str) {
    let path = rust.join(rel);
    let text = fs::read_to_string(&path).unwrap();
    assert_eq!(text.matches(old).count(), 1, "{rel}: anchor {old:?}");
    fs::write(path, text.replacen(old, new, 1)).unwrap();
}

fn p9_duplicate_escape(rust: &Path) {
    let path = rust.join("ckc/src/trust.rs");
    let text = fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let hits: Vec<usize> = (0..lines.len())
        .filter(|&i| lines[i].trim() == "\"unsafe\",")
        .collect();
    assert_eq!(hits.len(), 1, "p9 allowlisted unsafe line");
    let mut out: Vec<&str> = lines.clone();
    out.insert(hits[0] + 1, lines[hits[0]]);
    fs::write(path, out.concat()).unwrap();
}

type Plant = (&'static str, fn(&Path), &'static str);

const PLANTS: [Plant; 22] = [
    (
        "p1",
        |r| {
            append(
                r,
                "ckc-kernel/src/lib.rs",
                "\nunsafe fn planted_unsafe() {}\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/lib\.rs: token `unsafe` .* x1$",
    ),
    (
        "p2",
        |r| {
            append(
                r,
                "ckc-kernel/src/lib.rs",
                "\nverus! { proof fn planted_assume() { assume(true); } }\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/lib\.rs: token `assume` .* x1$",
    ),
    (
        "p3",
        |r| {
            append(
                r,
                "ckc-kernel/src/lib.rs",
                "\n#[verifier::external_body]\nfn planted_external_body() {}\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/lib\.rs: token `external_body` .* x1$",
    ),
    (
        "p4",
        |r| fs::write(r.join("ckc-kernel/build.rs"), "fn main() {}\n").unwrap(),
        r"^ckc: trust-audit: ckc-kernel/build\.rs present$",
    ),
    (
        "p5",
        |r| {
            append(
                r,
                "Cargo.lock",
                "\n[[package]]\nname = \"planted-trust-battery\"\nversion = \"0.0.0\"\n",
            )
        },
        r"^ckc: trust-audit: dep not allowlisted: planted-trust-battery 0\.0\.0$",
    ),
    (
        "p6",
        |r| append(r, "ckc-spec/src/align.rs", " "),
        r"^ckc: trust-audit: trusted-surface drift: ckc-spec/src/align\.rs sha256 [0-9a-f]{64} != manifest [0-9a-f]{64}$",
    ),
    (
        "p7",
        |r| replace_once(r, "ckc-kernel/Cargo.toml", "verify = true\n", ""),
        r"^ckc: trust-audit: ckc-kernel/Cargo\.toml missing \[package\.metadata\.verus\] verify = true$",
    ),
    (
        "p8",
        |r| append(r, "ckc-kernel/Cargo.toml", "\n[lib]\nproc-macro = true\n"),
        r"^ckc: trust-audit: ckc-kernel/Cargo\.toml declares proc-macro$",
    ),
    (
        "p9",
        p9_duplicate_escape,
        r#"^ckc: trust-audit: escape count mismatch ckc/src/trust\.rs: token `unsafe` line `"unsafe",`: found 2 allowed 1$"#,
    ),
    (
        "p10",
        |r| {
            append(
                r,
                "ckc-kernel/src/lib.rs",
                "\n#[verifier :: external]\nfn planted_spaced_external() {}\n",
            )
        },
        r"^ckc: trust-audit: whitespace-obfuscated token `verifier::external` in ckc-kernel/src/lib\.rs: 1 collapsed vs 0 plain$",
    ),
    (
        "p11",
        |r| {
            append(
                r,
                "ckc-kernel/src/lib.rs",
                "\n#[verifier::assume_termination]\nfn planted_assume_termination() {}\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/lib\.rs: token `assume_termination` .*$",
    ),
    (
        "p12",
        |r| {
            append(
                r,
                "ckc-kernel/src/lib.rs",
                "\n#[verifier(external)]\nfn planted_verifier_external() {}\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/lib\.rs: token `verifier\(external` .*$",
    ),
    (
        "p13",
        |r| append(r, "ckc-kernel/src/lib.rs", "\ninclude !(\"x\");\n"),
        r"^ckc: trust-audit: whitespace-obfuscated token `include!` in ckc-kernel/src/lib\.rs: 1 collapsed vs 0 plain$",
    ),
    (
        "p14",
        |r| {
            append(
                r,
                "ckc-kernel/src/lib.rs",
                "\n#[verifier /* c */ :: external]\nfn planted_comment_external() {}\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/lib\.rs: token `external` .*$",
    ),
    (
        "p15",
        |r| {
            append(
                r,
                "ckc-kernel/Cargo.toml",
                "\n[lib]\npath = \"../../decoy.rs\"\n",
            )
        },
        r"^ckc: trust-audit: trusted-surface drift: ckc-kernel/Cargo\.toml sha256 [0-9a-f]{64} != manifest [0-9a-f]{64}$",
    ),
    (
        "p16",
        |r| {
            append(
                r,
                "ckc-kernel/src/lib.rs",
                "\n#[path = \"../../decoy.rs\"]\nmod decoy;\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/lib\.rs: token `#\[path` .* x1$",
    ),
    (
        "p17",
        |r| {
            append(
                r,
                "ckc-kernel/src/lib.rs",
                "\npub const PLANTED: &str = include_str!(\"../../decoy.txt\");\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/lib\.rs: token `include_str!` .* x1$",
    ),
    (
        "p18",
        |r| {
            replace_once(
                r,
                "Cargo.toml",
                "members = [\"ckc-spec\", \"ckc-kernel\", \"ckc\"]",
                "members = [\"ckc-spec\", \"ckc-kernel\", \"ckc\", \"decoy\"]",
            )
        },
        r"^ckc: trust-audit: trusted-surface drift: Cargo\.toml sha256 [0-9a-f]{64} != manifest [0-9a-f]{64}$",
    ),
    (
        "p19",
        |r| {
            append(
                r,
                "Cargo.toml",
                "\n[patch.crates-io]\nsha2 = { path = \"../decoy\" }\n",
            )
        },
        r"^ckc: trust-audit: trusted-surface drift: Cargo\.toml sha256 [0-9a-f]{64} != manifest [0-9a-f]{64}$",
    ),
    (
        "p20",
        |r| {
            replace_once(
                r,
                "Cargo.lock",
                "name = \"sha2\"\nversion = \"0.11.0\"",
                "name = \"sha2\"\nversion = \"0.11.1\"",
            )
        },
        r"^ckc: trust-audit: dep not allowlisted: sha2 0\.11\.1$",
    ),
    (
        "p21",
        |r| {
            append(
                r,
                "ckc-kernel/src/k2_engine.rs",
                "\nverus! { proof fn planted_k2() { assume(false); } }\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/k2_engine\.rs: token `assume` .* x1$",
    ),
    (
        "p22",
        |r| {
            append(
                r,
                "ckc-kernel/src/k3_machine.rs",
                "\n#[verifier::external_body]\nfn planted_k3() {}\n",
            )
        },
        r"^ckc: trust-audit: unallowlisted escape ckc-kernel/src/k3_machine\.rs: token `external_body` .* x1$",
    ),
];
const METER: &str = r"^ckc: trust spec=\d+ shell=\d+ assumes=\d+ deps=ok$";

// The regex subset the pinned patterns use: ^…$ anchors, `\x` escapes (`\d` =
// digit), `.*`, `[a-b…]` classes, and `{n}` / `+` repeats on digits + classes.
enum Atom {
    Lit(char),
    Any,
    Set(Vec<(char, char)>),
}

fn parse(pattern: &str) -> Vec<(Atom, usize, usize)> {
    let body = pattern
        .strip_prefix('^')
        .unwrap()
        .strip_suffix('$')
        .unwrap();
    let chars: Vec<char> = body.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let atom = match chars[i] {
            '\\' if chars[i + 1] == 'd' => {
                i += 2;
                Atom::Set(vec![('0', '9')])
            }
            '\\' => {
                i += 2;
                Atom::Lit(chars[i - 1])
            }
            '.' => {
                i += 1;
                Atom::Any
            }
            '[' => {
                let end = i + chars[i..].iter().position(|&c| c == ']').unwrap();
                let mut ranges = Vec::new();
                let mut j = i + 1;
                while j < end {
                    if j + 2 < end && chars[j + 1] == '-' {
                        ranges.push((chars[j], chars[j + 2]));
                        j += 3;
                    } else {
                        ranges.push((chars[j], chars[j]));
                        j += 1;
                    }
                }
                i = end + 1;
                Atom::Set(ranges)
            }
            c => {
                i += 1;
                Atom::Lit(c)
            }
        };
        let (min, max) = match chars.get(i) {
            Some('*') => {
                i += 1;
                (0, usize::MAX)
            }
            Some('+') => {
                i += 1;
                (1, usize::MAX)
            }
            Some('{') => {
                let end = i + chars[i..].iter().position(|&c| c == '}').unwrap();
                let n: usize = chars[i + 1..end]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                i = end + 1;
                (n, n)
            }
            _ => (1, 1),
        };
        out.push((atom, min, max));
    }
    out
}

fn accepts(atom: &Atom, c: char) -> bool {
    match atom {
        Atom::Lit(l) => *l == c,
        Atom::Any => c != '\n',
        Atom::Set(ranges) => ranges.iter().any(|&(a, b)| a <= c && c <= b),
    }
}

fn matches(parts: &[(Atom, usize, usize)], text: &[char]) -> bool {
    let Some(((atom, min, max), rest)) = parts.split_first() else {
        return text.is_empty();
    };
    let run = text
        .iter()
        .take_while(|&&c| accepts(atom, c))
        .count()
        .min(*max);
    (*min..=run).rev().any(|n| matches(rest, &text[n..]))
}

fn full_match(pattern: &str, line: &str) -> bool {
    matches(&parse(pattern), &line.chars().collect::<Vec<_>>())
}

fn audit(rust: &Path) -> Output {
    let program = std::env::var_os("CKC_TRUST_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    Command::new(program)
        .arg("trust-audit")
        .arg(rust)
        .output()
        .unwrap()
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn every_plant_is_rejected_and_the_clean_copy_passes() {
    let scratch = Scratch(workspace().join(format!("target/trust-battery/{}", std::process::id())));
    let _ = fs::remove_dir_all(&scratch.0);
    let baseline = scratch.0.join("baseline");
    copy_tree(&workspace(), &baseline);
    let mut failures = Vec::new();

    let control = scratch.0.join("control");
    copy_tree(&baseline, &control);
    let out = audit(&control);
    let stdout = String::from_utf8_lossy(&out.stdout);
    if out.status.code() != Some(0)
        || !out.stderr.is_empty()
        || !full_match(METER, stdout.strip_suffix('\n').unwrap_or(&stdout))
    {
        failures.push(format!(
            "control: rc {:?} stdout {stdout:?} stderr {:?}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    // One thread per plant: each audit is an independent child process.
    let baseline = &baseline;
    let scratch = &scratch.0;
    let survivors: Vec<String> = std::thread::scope(|scope| {
        let runs: Vec<_> = PLANTS
            .iter()
            .map(|&(id, plant, pattern)| {
                scope.spawn(move || {
                    let copy = scratch.join(id);
                    copy_tree(baseline, &copy);
                    plant(&copy);
                    let out = audit(&copy);
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    (out.status.code() != Some(1)
                        || !stderr.lines().any(|l| full_match(pattern, l)))
                    .then(|| format!("{id}: rc {:?} stderr {stderr:?}", out.status.code()))
                })
            })
            .collect();
        runs.into_iter()
            .filter_map(|run| run.join().unwrap())
            .collect()
    });
    failures.extend(survivors);
    assert!(
        failures.is_empty(),
        "battery: {} of {} rows survived:\n{}",
        failures.len(),
        PLANTS.len() + 1,
        failures.join("\n")
    );
}

// `--write` adopts exactly the drift the read-only audit rejects: after the
// rebaseline the planted copy passes and trust/ names each planted row.
#[test]
fn write_rebaselines_a_planted_copy() {
    let scratch = Scratch(workspace().join(format!("target/trust-write/{}", std::process::id())));
    let _ = fs::remove_dir_all(&scratch.0);
    let copy = scratch.0.join("rust");
    copy_tree(&workspace(), &copy);
    for (id, plant, _) in PLANTS {
        if ["p1", "p5", "p6"].contains(&id) {
            plant(&copy);
        }
    }
    assert_eq!(
        audit(&copy).status.code(),
        Some(1),
        "planted copy fails before --write"
    );
    let program = std::env::var_os("CKC_TRUST_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    let out = Command::new(program)
        .args(["trust-audit", "--write"])
        .arg(&copy)
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "--write: {:?}",
        String::from_utf8_lossy(&out.stderr)
    );
    let out = audit(&copy);
    assert_eq!(
        out.status.code(),
        Some(0),
        "audit after --write: {:?}",
        String::from_utf8_lossy(&out.stderr)
    );
    let trust = |name: &str| fs::read_to_string(copy.join("trust").join(name)).unwrap();
    assert!(
        trust("escape-allowlist.tsv")
            .contains("ckc-kernel/src/lib.rs\tunsafe\t1\tunsafe fn planted_unsafe() {}\n")
    );
    assert!(trust("deps-allowlist.tsv").contains("planted-trust-battery\t0.0.0\n"));
    let spec = fs::read(copy.join("ckc-spec/src/align.rs")).unwrap();
    use sha2::Digest;
    let digest: String = sha2::Sha256::digest(&spec)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert!(trust("spec-manifest.tsv").contains(&format!("{digest}\tckc-spec/src/align.rs\n")));
}
