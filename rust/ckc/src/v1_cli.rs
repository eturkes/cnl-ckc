// ckc v1 <check|render> <pl>: K1 shell seam (contract m5u2 R9/R10/R19).
// Shell owns io + UTF-8 validation; the verified kernel owns acceptance.
// Reject envelope = one stderr line, stdout empty, rc2 (R9); render rc0
// echoes the input bytes exactly — check + echo, never a formatter (R19).
use std::io::Write;
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

pub fn run(args: &[String]) -> ExitCode {
    match args {
        [mode, path] if mode == "check" || mode == "render" => run_file(mode, path),
        _ => {
            eprintln!("usage: ckc v1 <check|render> <pl>");
            ExitCode::from(2)
        }
    }
}
