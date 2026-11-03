use sb1_checker::{Request, SourceCodec, check_with_codec, profile_sources};
use std::io::Write;
use std::path::{Path, PathBuf};

const USAGE: &str =
    "usage: checker --root <repo> <composition.tsv> <model.txt> <certificate.txt> [<query>]";

fn orchestrated(
    args: &[std::ffi::OsString],
) -> Result<sb1_checker::orchestrator::OrchestratedRequest, sb1_checker::Failure> {
    use sb1_checker::orchestrator::{OrchestratedRequest, Provider};
    let usage = || {
        sb1_checker::Failure {
        rc: 2,
        message: "usage: checker orchestrate [--provider all|static] [--slice-ids <file>] [--producer <file>] [--swipl <file>] --root <repo> <composition.tsv> <out> <query>".into(),
    }
    };
    let mut provider = Provider::Static;
    let mut producer = None;
    let mut swipl = PathBuf::from("swipl");
    let mut at = 0;
    while args.get(at).is_some_and(|a| a != "--root") {
        let value = args.get(at + 1).ok_or_else(usage)?;
        match args[at].to_str() {
            Some("--provider") => {
                provider = match value.to_str() {
                    Some("all") => Provider::All,
                    Some("static") => Provider::Static,
                    _ => return Err(usage()),
                }
            }
            Some("--slice-ids") => provider = Provider::Ids(value.into()),
            Some("--producer") => producer = Some(PathBuf::from(value)),
            Some("--swipl") => swipl = value.into(),
            _ => return Err(usage()),
        }
        at += 2;
    }
    if args.len() != at + 5 || args[at] != "--root" {
        return Err(usage());
    }
    let root = PathBuf::from(&args[at + 1]);
    Ok(OrchestratedRequest {
        producer: producer.unwrap_or_else(|| root.join("prototype/sb1/producer.prolog")),
        root,
        composition: (&args[at + 2]).into(),
        output: (&args[at + 3]).into(),
        query: (&args[at + 4]).into(),
        provider,
        swipl,
    })
}

fn write_orchestrated(
    directory: &Path,
    text: &str,
    prepared: &sb1_checker::orchestrator::PreparedProgram,
) -> Result<(), sb1_checker::Failure> {
    if std::env::var_os("CKC_BENCH_SINK").as_deref() == Some(std::ffi::OsStr::new("closed-file-v1"))
    {
        prepared.write_complete_sink(directory, text)?;
        return prepared.trace(directory, "pre_process_exit");
    }
    prepared.trace(directory, "stdout_write_begin")?;
    let mut sink = std::io::stdout().lock();
    sink.write_all(text.as_bytes())
        .and_then(|_| sink.flush())
        .map_err(|error| sb1_checker::Failure {
            rc: 2,
            message: format!("I/O stdout {error}"),
        })?;
    prepared.trace(directory, "stdout_flushed")?;
    drop(sink);
    prepared.trace(directory, "stdout_guard_dropped")?;
    prepared.trace(directory, "pre_process_exit")
}

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
    if args.first().is_some_and(|a| a == "orchestrate") {
        match orchestrated(&args[1..]).and_then(|request| {
            let (stdout, prepared) =
                sb1_checker::orchestrator::orchestrate_retained(&request, mode)?;
            Ok((request.output, stdout, prepared))
        }) {
            Ok((directory, stdout, prepared)) => {
                if let Err(error) = write_orchestrated(&directory, &stdout, &prepared) {
                    eprintln!("{}", error.message);
                    std::process::exit(error.rc);
                }
                // The checked KB is process-lifetime state. Walking all of its
                // allocations in Drop would add O(loaded KB) per-call teardown.
                std::process::exit(0);
            }
            Err(error) => {
                eprintln!("{}", error.message);
                std::process::exit(error.rc);
            }
        }
    }
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
