// trust-audit: zero-trust gate over the workspace (contract m5u1 R6; plan
// "Trust-audit gate"). Scans member sources for verifier escape hatches +
// inclusion vectors, enforces kernel-crate structure, diffs Cargo.lock
// against the dep allowlist, and byte-pins the trusted surface via the
// spec manifest. Violations -> stderr `ckc: trust-audit: ...` rc1; green ->
// stdout meter rc0. This file's own token-table lines are themselves
// escape-allowlisted (self-scan).
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const MEMBERS: [&str; 3] = ["ckc-spec", "ckc-kernel", "ckc"];
const KERNEL_CRATES: [&str; 2] = ["ckc-spec", "ckc-kernel"];
// Word-boundary tokens: an occurrence counts when the neighbouring chars are
// not [A-Za-z0-9_] (so `assume` never fires inside `assume_specification`).
const WORD_TOKENS: [&str; 8] = [
    "unsafe",
    "assume",
    "admit",
    "external_body",
    "assume_specification",
    "external_fn_specification",
    "verifier::external",
    "exec_spec_unverified",
];
// Raw substring tokens (inclusion vectors; leading char is non-word).
const RAW_TOKENS: [&str; 4] = ["#[path", "include!", "include_str!", "include_bytes!"];
// Trusted-surface coverage beyond ckc-spec/src: kernel binding files + the
// allowlists + toolchain pins. The manifest must list exactly these + every
// file under ckc-spec/src.
const TRUSTED_KERNEL_FILES: [&str; 1] = ["ckc-kernel/src/contract.rs"];
const TRUSTED_EXTRA: [&str; 4] = [
    "trust/escape-allowlist.tsv",
    "trust/deps-allowlist.tsv",
    "verus.lock",
    "rust-toolchain.toml",
];

pub fn run(root: &str) -> ExitCode {
    let root = PathBuf::from(root);
    let mut v: Vec<String> = Vec::new();

    if !root.join("Cargo.toml").is_file() {
        eprintln!("ckc: trust-audit: no Cargo.toml at {}", root.display());
        return ExitCode::from(2);
    }

    let escape_sites = scan_escapes(&root, &mut v);
    check_structure(&root, &mut v);
    check_deps(&root, &mut v);
    let spec_lines = check_manifest(&root, &mut v);
    let shell_files = count_rs(&root.join("ckc").join("src"));

    if v.is_empty() {
        println!(
            "ckc: trust spec={} shell={} assumes={} deps=ok",
            spec_lines, shell_files, escape_sites
        );
        ExitCode::SUCCESS
    } else {
        for line in &v {
            eprintln!("ckc: trust-audit: {}", line);
        }
        ExitCode::from(1)
    }
}

fn is_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn count_token(line: &str, token: &str, word_bounded: bool) -> usize {
    let lb = line.as_bytes();
    let tb = token.as_bytes();
    let mut n = 0;
    let mut i = 0;
    while i + tb.len() <= lb.len() {
        if &lb[i..i + tb.len()] == tb {
            let pre_ok = i == 0 || !is_word(lb[i - 1]);
            let post_ok = i + tb.len() == lb.len() || !is_word(lb[i + tb.len()]);
            if !word_bounded || (pre_ok && post_ok) {
                n += 1;
                i += tb.len();
                continue;
            }
        }
        i += 1;
    }
    n
}

fn walk_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk_rs(&p, out);
        } else if p.extension().is_some_and(|e| e == "rs") {
            out.push(p);
        }
    }
}

fn count_rs(dir: &Path) -> usize {
    let mut files = Vec::new();
    walk_rs(dir, &mut files);
    files.len()
}

// Multiset of (relative path, token, trimmed line) -> occurrence count over
// every member's src tree, compared exactly against escape-allowlist.tsv
// (path<TAB>token<TAB>count<TAB>trimmed-line). Returns total matched sites.
fn scan_escapes(root: &Path, v: &mut Vec<String>) -> usize {
    let mut found: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    for m in MEMBERS {
        let mut files = Vec::new();
        walk_rs(&root.join(m).join("src"), &mut files);
        for f in files {
            let rel = f.strip_prefix(root).unwrap_or(&f).to_string_lossy().to_string();
            let Ok(text) = fs::read_to_string(&f) else {
                v.push(format!("unreadable source file: {}", rel));
                continue;
            };
            for line in text.lines() {
                for t in WORD_TOKENS {
                    let n = count_token(line, t, true);
                    if n > 0 {
                        *found
                            .entry((rel.clone(), t.to_string(), line.trim().to_string()))
                            .or_insert(0) += n;
                    }
                }
                for t in RAW_TOKENS {
                    let n = count_token(line, t, false);
                    if n > 0 {
                        *found
                            .entry((rel.clone(), t.to_string(), line.trim().to_string()))
                            .or_insert(0) += n;
                    }
                }
            }
        }
    }

    let mut allowed: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    let allow_path = root.join("trust").join("escape-allowlist.tsv");
    match fs::read_to_string(&allow_path) {
        Ok(text) => {
            for (i, line) in text.lines().enumerate() {
                if i == 0 && line == "path\ttoken\tcount\ttrimmed_line" {
                    continue;
                }
                let parts: Vec<&str> = line.splitn(4, '\t').collect();
                let count = parts.get(2).and_then(|c| c.parse::<usize>().ok());
                match (parts.len(), count) {
                    (4, Some(c)) if c > 0 => {
                        let key = (parts[0].to_string(), parts[1].to_string(), parts[3].to_string());
                        if allowed.insert(key, c).is_some() {
                            v.push(format!("escape-allowlist duplicate row {}", i + 1));
                        }
                    }
                    _ => v.push(format!("escape-allowlist malformed row {}", i + 1)),
                }
            }
        }
        Err(_) => v.push("missing trust/escape-allowlist.tsv".to_string()),
    }

    let mut total = 0;
    for (k, n) in &found {
        total += n;
        match allowed.get(k) {
            Some(c) if c == n => {}
            Some(c) => v.push(format!(
                "escape count mismatch {}: token `{}` line `{}`: found {} allowed {}",
                k.0, k.1, k.2, n, c
            )),
            None => v.push(format!(
                "unallowlisted escape {}: token `{}` line `{}` x{}",
                k.0, k.1, k.2, n
            )),
        }
    }
    for (k, c) in &allowed {
        if !found.contains_key(k) {
            v.push(format!(
                "stale escape-allowlist row {}: token `{}` line `{}` x{}",
                k.0, k.1, k.2, c
            ));
        }
    }
    total
}

// Kernel crates carry [package.metadata.verus] verify = true; no member has
// build.rs, a `build =` manifest key, or `proc-macro = true`.
fn check_structure(root: &Path, v: &mut Vec<String>) {
    for m in MEMBERS {
        if root.join(m).join("build.rs").exists() {
            v.push(format!("{}/build.rs present", m));
        }
        let mp = root.join(m).join("Cargo.toml");
        let Ok(text) = fs::read_to_string(&mp) else {
            v.push(format!("unreadable {}/Cargo.toml", m));
            continue;
        };
        let stripped: Vec<&str> = text.lines().map(|l| l.trim()).collect();
        if stripped.iter().any(|l| l.starts_with("build =") || l.starts_with("build=")) {
            v.push(format!("{}/Cargo.toml declares build script", m));
        }
        if stripped.iter().any(|l| l.starts_with("proc-macro") && l.contains("true")) {
            v.push(format!("{}/Cargo.toml declares proc-macro", m));
        }
        if KERNEL_CRATES.contains(&m) {
            let has = text
                .split("[package.metadata.verus]")
                .nth(1)
                .map(|rest| rest.split('[').next().unwrap_or(""))
                .is_some_and(|body| {
                    body.lines().any(|l| l.trim().replace(' ', "") == "verify=true")
                });
            if !has {
                v.push(format!("{}/Cargo.toml missing [package.metadata.verus] verify = true", m));
            }
        }
    }
}

// Cargo.lock package set (name, version) == deps-allowlist.tsv rows, both
// directions. Workspace members list themselves in the allowlist.
fn check_deps(root: &Path, v: &mut Vec<String>) {
    let mut locked: BTreeMap<(String, String), usize> = BTreeMap::new();
    match fs::read_to_string(root.join("Cargo.lock")) {
        Ok(text) => {
            let mut name: Option<String> = None;
            for line in text.lines() {
                let line = line.trim();
                if let Some(n) = line.strip_prefix("name = \"") {
                    name = n.strip_suffix('"').map(|s| s.to_string());
                } else if let Some(ver) = line.strip_prefix("version = \"") {
                    if let (Some(n), Some(w)) = (name.take(), ver.strip_suffix('"')) {
                        *locked.entry((n, w.to_string())).or_insert(0) += 1;
                    }
                }
            }
        }
        Err(_) => v.push("missing Cargo.lock".to_string()),
    }

    let mut allowed: BTreeMap<(String, String), usize> = BTreeMap::new();
    match fs::read_to_string(root.join("trust").join("deps-allowlist.tsv")) {
        Ok(text) => {
            for (i, line) in text.lines().enumerate() {
                if i == 0 && line == "name\tversion" {
                    continue;
                }
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    *allowed
                        .entry((parts[0].to_string(), parts[1].to_string()))
                        .or_insert(0) += 1;
                } else {
                    v.push(format!("deps-allowlist malformed row {}", i + 1));
                }
            }
        }
        Err(_) => v.push("missing trust/deps-allowlist.tsv".to_string()),
    }

    for (k, _) in &locked {
        if !allowed.contains_key(k) {
            v.push(format!("dep not allowlisted: {} {}", k.0, k.1));
        }
    }
    for (k, _) in &allowed {
        if !locked.contains_key(k) {
            v.push(format!("stale deps-allowlist row: {} {}", k.0, k.1));
        }
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().iter().map(|b| format!("{:02x}", b)).collect()
}

// spec-manifest.tsv (sha256<TAB>path) must cover exactly: every .rs under
// ckc-spec/src + TRUSTED_KERNEL_FILES + TRUSTED_EXTRA, each digest matching
// current bytes. Returns trusted-surface line count (spec + kernel bindings).
fn check_manifest(root: &Path, v: &mut Vec<String>) -> usize {
    let mut required: Vec<String> = Vec::new();
    let mut spec_files = Vec::new();
    walk_rs(&root.join("ckc-spec").join("src"), &mut spec_files);
    for f in &spec_files {
        required.push(f.strip_prefix(root).unwrap_or(f).to_string_lossy().to_string());
    }
    for f in TRUSTED_KERNEL_FILES {
        required.push(f.to_string());
    }
    for f in TRUSTED_EXTRA {
        required.push(f.to_string());
    }
    required.sort();

    let mut listed: BTreeMap<String, String> = BTreeMap::new();
    match fs::read_to_string(root.join("trust").join("spec-manifest.tsv")) {
        Ok(text) => {
            for (i, line) in text.lines().enumerate() {
                if i == 0 && line == "sha256\tpath" {
                    continue;
                }
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 && parts[0].len() == 64 {
                    if listed.insert(parts[1].to_string(), parts[0].to_string()).is_some() {
                        v.push(format!("spec-manifest duplicate path row {}", i + 1));
                    }
                } else {
                    v.push(format!("spec-manifest malformed row {}", i + 1));
                }
            }
        }
        Err(_) => v.push("missing trust/spec-manifest.tsv".to_string()),
    }

    for path in &required {
        match listed.get(path) {
            Some(want) => match fs::read(root.join(path)) {
                Ok(bytes) => {
                    let got = sha256_hex(&bytes);
                    if &got != want {
                        v.push(format!("trusted-surface drift: {} sha256 {} != manifest {}", path, got, want));
                    }
                }
                Err(_) => v.push(format!("trusted file unreadable: {}", path)),
            },
            None => v.push(format!("trusted file missing from spec-manifest: {}", path)),
        }
    }
    for path in listed.keys() {
        if !required.contains(path) {
            v.push(format!("spec-manifest lists non-trusted path: {}", path));
        }
    }

    let mut lines = 0;
    for f in &spec_files {
        if let Ok(text) = fs::read_to_string(f) {
            lines += text.lines().count();
        }
    }
    for f in TRUSTED_KERNEL_FILES {
        if let Ok(text) = fs::read_to_string(root.join(f)) {
            lines += text.lines().count();
        }
    }
    lines
}
