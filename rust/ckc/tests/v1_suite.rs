// M5.2 v1 suite (.agent/contracts/harness.md H4): tests/v1/cases.tsv rows =
// 230 primary targets + 115 supplemental K3 probes. Every case materializes
// under one private root at its recording layout (.scratch/m5u2/suite/cases/
// <case>/…, `.in` dropped, `.tpl.in` expanded), so argv and path-bearing
// diagnostics keep their recorded bytes; `ckc v1 <mode> <args>` runs with cwd =
// that root and must reproduce rc, stdout and stderr exactly.
// CKC_V1_TEST_BIN selects a prebuilt executable (red replay).
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

const LAYOUT: &str = ".scratch/m5u2/suite/cases";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn digest_hex(bytes: &[u8]) -> String {
    use sha2::Digest;
    sha2::Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

// Balanced `','/2` tree over `first` + (n-1) × `leaf`, paired left to right per layer.
fn comma_tree(n: usize, first: &[u8], leaf: &[u8]) -> Vec<u8> {
    let mut layer: Vec<Vec<u8>> = std::iter::once(first.to_vec())
        .chain(std::iter::repeat_n(leaf.to_vec(), n - 1))
        .collect();
    while layer.len() > 1 {
        layer = layer
            .chunks(2)
            .map(|pair| match pair {
                [a, b] => [b"','(".as_slice(), a, b",", b, b")"].concat(),
                [a] => a.clone(),
                _ => unreachable!(),
            })
            .collect();
    }
    layer.pop().unwrap()
}

// Template lines: `@@sha256 <hex>` first; `@@run <lo> <hi> <text>` = one line per
// i in lo..=hi with `@i@` → i and `@j@` → i+1; `@@tree <n>\t<first>\t<leaf>\t
// <prefix>\t<suffix>` = prefix + comma_tree + suffix; any other line is literal.
fn expand(template: &[u8], label: &str) -> Vec<u8> {
    let text = std::str::from_utf8(template).unwrap();
    let mut lines = text.strip_suffix('\n').unwrap().split('\n');
    let pin = lines.next().unwrap().strip_prefix("@@sha256 ").unwrap();
    let mut out = Vec::new();
    for line in lines {
        if let Some(run) = line.strip_prefix("@@run ") {
            let mut parts = run.splitn(3, ' ');
            let lo: usize = parts.next().unwrap().parse().unwrap();
            let hi: usize = parts.next().unwrap().parse().unwrap();
            let body = parts.next().unwrap();
            for i in lo..=hi {
                out.extend_from_slice(
                    body.replace("@i@", &i.to_string())
                        .replace("@j@", &(i + 1).to_string())
                        .as_bytes(),
                );
                out.push(b'\n');
            }
        } else if let Some(tree) = line.strip_prefix("@@tree ") {
            let f: Vec<&str> = tree.split('\t').collect();
            out.extend_from_slice(f[3].as_bytes());
            out.extend(comma_tree(
                f[0].parse().unwrap(),
                f[1].as_bytes(),
                f[2].as_bytes(),
            ));
            out.extend_from_slice(f[4].as_bytes());
            out.push(b'\n');
        } else {
            assert!(!line.starts_with("@@"), "{label}: unknown directive");
            out.extend_from_slice(line.as_bytes());
            out.push(b'\n');
        }
    }
    assert_eq!(digest_hex(&out), pin, "{label}: template expansion sha256");
    out
}

fn materialize(from: &Path, to: &Path, top: bool) {
    for entry in fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap().to_owned();
        if path.is_dir() {
            if !(top && name == "expect") {
                materialize(&path, &to.join(&name), false);
            }
            continue;
        }
        fs::create_dir_all(to).unwrap();
        let bytes = fs::read(&path).unwrap();
        if let Some(stem) = name.strip_suffix(".tpl.in") {
            fs::write(to.join(stem), expand(&bytes, &path.display().to_string())).unwrap();
        } else {
            fs::write(to.join(name.strip_suffix(".in").unwrap()), bytes).unwrap();
        }
    }
}

struct Row {
    case: String,
    probe: String,
    mode: String,
    rc: i32,
    stderr_lines: Option<usize>,
    args: Vec<String>,
}

// Run with a 120 s cap (the harness guard; runtime bounds = review R-13).
fn run(program: &Path, cwd: &Path, row: &Row) -> Result<(i32, Vec<u8>, Vec<u8>), String> {
    let mut child = Command::new(program)
        .arg("v1")
        .arg(&row.mode)
        .args(
            row.args
                .iter()
                .map(|a| format!("{LAYOUT}/{}/{a}", row.case)),
        )
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let out = std::thread::spawn(move || {
        let mut buf = Vec::new();
        stdout.read_to_end(&mut buf).unwrap();
        buf
    });
    let err = std::thread::spawn(move || {
        let mut buf = Vec::new();
        stderr.read_to_end(&mut buf).unwrap();
        buf
    });
    let deadline = Instant::now() + Duration::from_secs(120);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err("timeout 120 s".into());
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    Ok((
        status.code().unwrap_or(-1),
        out.join().unwrap(),
        err.join().unwrap(),
    ))
}

#[test]
fn v1_suite() {
    let program = std::env::var_os("CKC_V1_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    let suite = root().join("tests/v1");
    let scratch = Scratch(root().join(format!("rust/target/v1-suite/{}", std::process::id())));
    let _ = fs::remove_dir_all(&scratch.0);
    let rows: Vec<Row> = fs::read_to_string(suite.join("cases.tsv"))
        .unwrap()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            Row {
                case: f[0].into(),
                probe: f[1].into(),
                mode: f[2].into(),
                rc: f[3].parse().unwrap(),
                stderr_lines: (f[4] != "-").then(|| f[4].parse().unwrap()),
                args: f[5]
                    .split(' ')
                    .filter(|a| !a.is_empty())
                    .map(str::to_owned)
                    .collect(),
            }
        })
        .collect();
    let primary = rows.iter().filter(|r| r.probe == "target").count();
    assert_eq!(
        (primary, rows.len() - primary),
        (230, 115),
        "tests/v1 row census"
    );
    let mut cases: Vec<&str> = rows.iter().map(|r| r.case.as_str()).collect();
    cases.dedup();
    for case in &cases {
        materialize(&suite.join(case), &scratch.0.join(LAYOUT).join(case), true);
    }
    let next = AtomicUsize::new(0);
    let failures = Mutex::new(Vec::new());
    let workers = std::thread::available_parallelism().map_or(4, |n| n.get().min(8));
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                while let Some(row) = rows.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let label = format!("{}/{}", row.case, row.probe);
                    let expect = |s: &str| {
                        fs::read(
                            suite
                                .join(&row.case)
                                .join("expect")
                                .join(format!("{}.{s}", row.probe)),
                        )
                        .unwrap()
                    };
                    let verdict = match run(&program, &scratch.0, row) {
                        Err(e) => Some(e),
                        Ok((rc, _, _)) if rc != row.rc => {
                            Some(format!("rc {rc} expected {}", row.rc))
                        }
                        Ok((_, out, _)) if out != expect("stdout") => Some(format!(
                            "stdout {:?}",
                            String::from_utf8_lossy(&out[..out.len().min(300)])
                        )),
                        Ok((_, _, err)) if err != expect("stderr") => Some(format!(
                            "stderr {:?}",
                            String::from_utf8_lossy(&err[..err.len().min(500)])
                        )),
                        Ok((_, _, err)) => row.stderr_lines.and_then(|want| {
                            let got = err.iter().filter(|&&b| b == b'\n').count()
                                + usize::from(!err.is_empty() && !err.ends_with(b"\n"));
                            (got != want).then(|| format!("stderr lines {got} expected {want}"))
                        }),
                    };
                    if let Some(v) = verdict {
                        failures
                            .lock()
                            .unwrap()
                            .push(format!("{label}[{}]: {v}", row.mode));
                    }
                }
            });
        }
    });
    let mut failures = failures.into_inner().unwrap();
    failures.sort();
    assert!(
        failures.is_empty(),
        "v1 suite: {} of {} rows failed:\n{}",
        failures.len(),
        rows.len(),
        failures.join("\n")
    );
}
