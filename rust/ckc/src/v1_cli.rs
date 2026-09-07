// ckc v1 <check|render> <pl>: K1 shell seam (contract m5u2 R9/R10/R19).
// Shell owns io + UTF-8 validation; the verified kernel owns acceptance.
// Reject envelope = one stderr line, stdout empty, rc2 (R9); render rc0
// echoes the input bytes exactly — check + echo, never a formatter (R19).
use ckc_kernel::{EOut, ESrc};
use std::ffi::OsStr;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

fn reject(line: u64, col: u64) -> ExitCode {
    eprintln!("ace_to_pl_error(check_load,noncanonical({line},{col})).");
    ExitCode::from(2)
}

// 1-based line + byte column of offset `at` = min(first invalid UTF-8
// byte, kernel first-divergence offset); R9 keeps the position outside the
// theorems, pinned by fixtures.
fn line_col(bytes: &[u8], at: usize) -> (u64, u64) {
    let line = 1 + bytes[..at].iter().filter(|&&b| b == 0x0A).count() as u64;
    let col = 1 + bytes[..at].iter().rev().take_while(|&&b| b != 0x0A).count() as u64;
    (line, col)
}

fn run_file(mode: &str, path: &str) -> ExitCode {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            // R1 SWI-authored open-error class: pinned deterministic detail.
            eprintln!("ace_to_pl_error(check_load,unreadable).");
            return ExitCode::from(2);
        }
    };
    let utf8_at = std::str::from_utf8(&bytes).err().map(|e| e.valid_up_to());
    let kernel_at = match ckc_kernel::contract::v1_check(&bytes) {
        ckc_kernel::EV1Verdict::Ok => None,
        ckc_kernel::EV1Verdict::Reject { at } => Some(at),
    };
    match utf8_at.into_iter().chain(kernel_at).min() {
        Some(at) => {
            let (l, c) = line_col(&bytes, at);
            reject(l, c)
        }
        None => {
            if mode == "render" {
                std::io::stdout().write_all(&bytes).ok();
            }
            ExitCode::SUCCESS
        }
    }
}

// One shell read = one ESrc (R24a): a path that is not a regular file or
// fails to read is Missing (R18); the first invalid UTF-8 byte is Bad.
fn read_src(path: &[u8]) -> ESrc {
    let p = OsStr::from_bytes(path);
    if !std::fs::metadata(p).map(|m| m.is_file()).unwrap_or(false) {
        return ESrc::Missing;
    }
    match std::fs::read(p) {
        Err(_) => ESrc::Missing,
        Ok(b) => match std::str::from_utf8(&b) {
            Err(e) => ESrc::Bad(e.valid_up_to()),
            Ok(_) => ESrc::Bytes(b),
        },
    }
}

fn emit(o: EOut) -> ExitCode {
    std::io::stdout().write_all(&o.out).ok();
    std::io::stderr().write_all(&o.err).ok();
    ExitCode::from(o.rc)
}

fn sha_of(s: &ESrc) -> String {
    match s {
        ESrc::Bytes(b) => crate::trust::sha256_hex(b),
        _ => String::new(),
    }
}

// Trace modes (K3): the kernel names the clause lines to hash, the shell
// hashes them (R3) and hands the digests back with the query/answers digests.
fn run_trace(mpath: &str, query: &str, answers: &str, trace: Option<&str>) -> ExitCode {
    let mp = mpath.as_bytes();
    let m = read_src(mp);
    let rows = match ckc_kernel::contract::v1_manifest(mp, &m) {
        Ok(rows) => rows,
        Err(o) => return emit(o),
    };
    let pls: Vec<ESrc> = rows.iter().map(|r| read_src(&r.pl)).collect();
    let pys: Vec<ESrc> = rows.iter().map(|r| read_src(&r.payload)).collect();
    let qs = read_src(query.as_bytes());
    let qsha = sha_of(&qs);
    let ans = read_src(answers.as_bytes());
    let asha = sha_of(&ans);
    let lines = match ckc_kernel::contract::v1_trace_lines(
        mp,
        &m,
        &pls,
        &pys,
        &qs,
        qsha.as_bytes(),
        &ans,
    ) {
        Ok(lines) => lines,
        Err(o) => return emit(o),
    };
    let digests: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| crate::trust::sha256_hex(l).into_bytes())
        .collect();
    emit(match trace {
        None => ckc_kernel::contract::v1_trace(
            mp,
            &m,
            &pls,
            &pys,
            &qs,
            qsha.as_bytes(),
            &ans,
            asha.as_bytes(),
            &digests,
        ),
        Some(t) => {
            let ts = read_src(t.as_bytes());
            ckc_kernel::contract::v1_trace_check(
                mp,
                &m,
                &pls,
                &pys,
                &qs,
                qsha.as_bytes(),
                &ans,
                asha.as_bytes(),
                &ts,
                &digests,
            )
        }
    })
}

// Composition modes: the kernel owns every verdict byte; the shell reads
// the manifest, one ESrc per manifest cell, and the query (R24).
fn run_mode(mode: &str, mpath: &str, query: Option<&str>) -> ExitCode {
    let mp = mpath.as_bytes();
    let m = read_src(mp);
    let rows = match ckc_kernel::contract::v1_manifest(mp, &m) {
        Ok(rows) => rows,
        Err(o) => return emit(o),
    };
    let pls: Vec<ESrc> = rows.iter().map(|r| read_src(&r.pl)).collect();
    let pys: Vec<ESrc> = rows.iter().map(|r| read_src(&r.payload)).collect();
    emit(match query {
        Some(q) => {
            let qs = read_src(q.as_bytes());
            let qsha = sha_of(&qs);
            ckc_kernel::contract::v1_answer(mp, &m, &pls, &pys, &qs, qsha.as_bytes())
        }
        None if mode == "aggregate-check" => {
            ckc_kernel::contract::v1_aggregate_check(mp, &m, &pls, &pys)
        }
        None => ckc_kernel::contract::v1_recursion_check(mp, &m, &pls, &pys),
    })
}

pub fn run(args: &[String]) -> ExitCode {
    match args {
        [mode, path] if mode == "check" || mode == "render" => run_file(mode, path),
        [mode, m] if mode == "aggregate-check" || mode == "recursion-check" => {
            run_mode(mode, m, None)
        }
        [mode, m, q] if mode == "answer" => run_mode(mode, m, Some(q)),
        [mode, m, q, a] if mode == "trace" => run_trace(m, q, a, None),
        [mode, m, q, a, t] if mode == "trace-check" => run_trace(m, q, a, Some(t)),
        _ => {
            eprintln!(
                "usage: ckc v1 <check|render> <pl> | ckc v1 <aggregate-check|recursion-check> <manifest> | ckc v1 answer <manifest> <query.pl> | ckc v1 trace <manifest> <query.pl> <answers.pl> | ckc v1 trace-check <manifest> <query.pl> <answers.pl> <traces.pl>"
            );
            ExitCode::from(2)
        }
    }
}
