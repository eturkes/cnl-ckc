// ckc: thin unverified shell (enumerated, fixture-covered). Subcommands:
// trust-audit (zero-trust gate) + align-check (M5.1 verified seam) + v1
// (M5.2 KB-consumption seams).
use std::process::ExitCode;

mod align_cli;
mod certify_cases_cli;
mod certify_cli;
mod trust;
mod v1_cli;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("trust-audit") => trust::run(args.get(2).map(|s| s.as_str()).unwrap_or(".")),
        Some("align-check") if args.len() == 5 => align_cli::run(&args[2], &args[3], &args[4]),
        Some("v1") => v1_cli::run(&args[2..]),
        Some("certify") if args.len() == 3 => certify_cli::run(&args[2]),
        Some("certify") if args.len() == 4 && args[2] == "--cases" => {
            certify_cases_cli::run(&args[3])
        }
        Some("certify-one") if args.len() == 8 => certify_cli::run_one(&args[2..]),
        _ => {
            eprintln!(
                "usage: ckc trust-audit [workspace-root] | ckc align-check <align.tsv> <src.txt> <ace.txt> | ckc v1 <check|render|aggregate-check|recursion-check|answer|trace|trace-check> … | ckc certify <guideline-id> | ckc certify --cases <tsv> | ckc certify-one <doc|query> <id> <ace> <ulex|-> <dump> <pl>"
            );
            ExitCode::from(2)
        }
    }
}
