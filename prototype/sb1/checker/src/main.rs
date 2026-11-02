use sb1_checker::{Request, SourceCodec, check_with_codec, profile_sources};
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
    let mode = match std::env::var("SB1_SOURCE_CODEC").as_deref() {
        Ok("kernel") => SourceCodec::Kernel,
        Ok("parsed") | Err(_) => SourceCodec::Parsed,
        Ok(_) => {
            eprintln!("usage: SB1_SOURCE_CODEC=kernel|parsed");
            std::process::exit(2);
        }
    };
    if args.first().is_some_and(|a| a == "source-profile") && args.len() == 4 && args[1] == "--root"
    {
        match profile_sources(Path::new(&args[2]), Path::new(&args[3]), mode) {
            Ok(table) => print!("{table}"),
            Err(error) => {
                eprintln!("{}", error.message);
                std::process::exit(error.rc);
            }
        }
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
    let outcome = check_with_codec(&request, mode);
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
