// Scale benchmark (contract `sb`): `ckc bench gen` writes family compositions,
// `ckc bench run` times a producer → checker path over one of them.
mod generate;
mod run;

pub fn run(args: &[String]) -> std::process::ExitCode {
    match args.first().map(|s| s.as_str()) {
        Some("gen") => generate::run(&args[1..]),
        Some("run") => run::run(&args[1..]),
        _ => usage(),
    }
}

pub(crate) fn usage() -> std::process::ExitCode {
    eprintln!("ckc: bench: usage: ckc bench <gen|run> …");
    std::process::ExitCode::from(2)
}
