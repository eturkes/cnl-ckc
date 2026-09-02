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

// 1-based line + byte-column of the byte at offset `at` (diagnostic law:
// column counts bytes since the last LF; R9 keeps positions outside the
// theorems, pinned by fixtures).
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
    if let Err(e) = std::str::from_utf8(&bytes) {
        let (l, c) = line_col(&bytes, e.valid_up_to());
        return reject(l, c);
    }
    match ckc_kernel::contract::v1_check(&bytes) {
        ckc_kernel::EV1Verdict::Ok => {
            if mode == "render" {
                std::io::stdout().write_all(&bytes).ok();
            }
            ExitCode::SUCCESS
        }
        ckc_kernel::EV1Verdict::Reject { line, col } => reject(line, col),
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
