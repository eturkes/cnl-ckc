use sb1_checker::{Request, check};
use std::io::Write;
use std::path::{Path, PathBuf};

const USAGE: &str =
    "usage: checker --root <repo> <composition.tsv> <model.txt> <certificate.txt> [<query>]";

fn main() {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    if args.len() == 1 && args[0] == "--help" {
        println!("{USAGE}");
        return;
    }
    if !matches!(args.len(), 5 | 6) || args[0] != "--root" {
        eprintln!("{USAGE}");
        std::process::exit(2);
    }
    let request = Request {
        root: PathBuf::from(&args[1]),
        composition: PathBuf::from(&args[2]),
        model: PathBuf::from(&args[3]),
        certificate: PathBuf::from(&args[4]),
        query: args.get(5).map(PathBuf::from),
    };
    let outcome = check(&request);
    let directory = request
        .model
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    if let Err(error) = outcome.write_metrics(directory) {
        eprintln!("{}", error.message);
        std::process::exit(error.rc);
    }
    if !outcome.stderr.is_empty() {
        eprint!("{}", outcome.stderr);
    }
    if !outcome.stdout.is_empty()
        && let Err(error) = std::io::stdout()
            .lock()
            .write_all(outcome.stdout.as_bytes())
    {
        eprintln!("I/O stdout {error}");
        std::process::exit(2);
    }
    std::process::exit(outcome.rc);
}
