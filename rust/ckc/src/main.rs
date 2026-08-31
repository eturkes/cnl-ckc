// ckc: thin unverified shell (enumerated, fixture-covered). M5.1 subcommands:
// trust-audit (zero-trust gate) + align-check (verified spike CLI seam).
use std::process::ExitCode;

mod align_cli;
mod trust;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("trust-audit") => trust::run(args.get(2).map(|s| s.as_str()).unwrap_or(".")),
        Some("align-check") if args.len() == 5 => align_cli::run(&args[2], &args[3], &args[4]),
        _ => {
            eprintln!("usage: ckc trust-audit [workspace-root] | ckc align-check <align.tsv> <src.txt> <ace.txt>");
            ExitCode::from(2)
        }
    }
}
