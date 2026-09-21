// ckc: thin unverified shell (enumerated, fixture-covered). Subcommands:
// trust-audit (zero-trust gate) + align-check (M5.1 verified seam) + v1
// (M5.2 KB-consumption seams) + check (M5.3 validators).
use std::process::ExitCode;

mod align_cli;
mod check;
mod trust;
mod v1_cli;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("trust-audit") => trust::run(args.get(2).map(|s| s.as_str()).unwrap_or(".")),
        Some("align-check") if args.len() == 5 => align_cli::run(&args[2], &args[3], &args[4]),
        Some("v1") => v1_cli::run(&args[2..]),
        Some("check") if args.len() <= 3 => {
            check::run(args.get(2).map(|s| s.as_str()).unwrap_or("."))
        }
        _ => {
            eprintln!(
                "usage: ckc trust-audit [workspace-root] | ckc align-check <align.tsv> <src.txt> <ace.txt> | ckc v1 <check|render|aggregate-check|recursion-check|answer|trace|trace-check> … | ckc check [repo-root]"
            );
            ExitCode::from(2)
        }
    }
}
