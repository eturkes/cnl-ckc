// Scale benchmark (contract `sb`): `ckc bench gen` writes family compositions,
// `ckc bench run` times a producer → checker path over one of them, `ckc bench grade`
// grades one campaign chunk, `ckc bench preflight` the host-only preflight before a campaign.
mod generate;
mod grade;
mod run;
mod stats;

pub fn run(args: &[String]) -> std::process::ExitCode {
    match args.first().map(|s| s.as_str()) {
        Some("gen") => generate::run(&args[1..]),
        Some("run") => run::run(&args[1..]),
        Some("grade") => grade::run(&args[1..]),
        Some("preflight") => grade::preflight(&args[1..]),
        _ => usage(),
    }
}

pub(crate) fn usage() -> std::process::ExitCode {
    eprintln!("ckc: bench: usage: ckc bench <gen|run|grade|preflight> …");
    std::process::ExitCode::from(2)
}
