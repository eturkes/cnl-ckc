#![forbid(unsafe_code)]
use sb1_checker::orchestrator::{OrchestratedRequest, Provider, ServeRequest, serve_with_provider};
use sb1_checker::{Failure, SourceCodec};
use sb1_tacheck::runtime::B1Provider;
use std::path::PathBuf;

const USAGE: &str = "usage: ta-server serve [--provider static|all] [--producer <file>] [--swipl <file>] --ta-receipts none --root <repo> <composition.tsv> <arm-dir>";
fn usage() -> Failure {
    Failure {
        rc: 2,
        message: USAGE.into(),
    }
}
fn request(args: &[String]) -> Result<ServeRequest, Failure> {
    if args.first().map(String::as_str) != Some("serve") {
        return Err(usage());
    }
    let mut provider = Provider::Static;
    let mut producer = None;
    let mut swipl = PathBuf::from("swipl");
    let mut receipts = false;
    let mut seen = std::collections::BTreeSet::new();
    let mut at = 1;
    while args.get(at).is_some_and(|s| s != "--root") {
        let key = args.get(at).ok_or_else(usage)?;
        let value = args.get(at + 1).ok_or_else(usage)?;
        if !seen.insert(key) {
            return Err(usage());
        }
        match key.as_str() {
            "--provider" => {
                provider = match value.as_str() {
                    "static" => Provider::Static,
                    "all" => Provider::All,
                    _ => return Err(usage()),
                }
            }
            "--producer" => producer = Some(PathBuf::from(value)),
            "--swipl" => swipl = value.into(),
            "--ta-receipts" if value == "none" => receipts = true,
            _ => return Err(usage()),
        }
        at += 2;
    }
    if !receipts || args.len() != at + 4 || args.get(at).map(String::as_str) != Some("--root") {
        return Err(usage());
    }
    let root = PathBuf::from(&args[at + 1]);
    Ok(ServeRequest {
        base: OrchestratedRequest {
            producer: producer.unwrap_or_else(|| root.join("prototype/sb1/producer.prolog")),
            root,
            composition: (&args[at + 2]).into(),
            output: (&args[at + 3]).into(),
            query: "-".into(),
            provider,
            swipl,
        },
        receipts: None,
        ta_run: None,
    })
}
fn run() -> Result<Option<PathBuf>, Failure> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--help"] {
        println!("{USAGE}");
        return Ok(None);
    }
    let request = request(&args)?;
    let codec = match std::env::var("SB1_SOURCE_CODEC").as_deref() {
        Ok("kernel") => SourceCodec::Kernel,
        Ok("parsed") | Err(_) => SourceCodec::Parsed,
        _ => return Err(usage()),
    };
    let mut provider = B1Provider::new(
        request.base.root.clone(),
        request.base.composition.clone(),
        request.base.output.clone(),
        matches!(request.base.provider, Provider::All),
    )?;
    let trace = provider.trace.clone();
    let result = serve_with_provider(
        &request,
        codec,
        std::io::stdin().lock(),
        trace.writer(std::io::stdout().lock()),
        Some(&mut provider),
    );
    trace.mark("server-serve-returned")?;
    result?;
    Ok(Some(request.base.output))
}
fn main() {
    let result = run().and_then(|output| match output {
        None => Ok(()),
        Some(directory) => match sb1_checker::server_terminal_exit(&directory) {
            Ok(never) => match never {},
            Err(error) => Err(Failure {
                rc: 2,
                message: format!("server-terminal {error}"),
            }),
        },
    });
    if let Err(error) = result {
        eprintln!("{}", error.message);
        std::process::exit(error.rc);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn args(extra: &[&str]) -> Vec<String> {
        [
            vec!["serve"],
            extra.to_vec(),
            vec!["--root", "root", "composition", "out"],
        ]
        .concat()
        .into_iter()
        .map(str::to_owned)
        .collect()
    }
    #[test]
    fn exactly_one_integrated_prepare_and_default_dc_shape_are_mandatory() {
        assert!(request(&args(&["--ta-receipts", "none"])).is_ok());
        for extra in [
            vec![],
            vec!["--ta-run", "census"],
            vec!["--ta-receipts", "receipts"],
            vec!["--ta-receipts", "none", "--already-shaped", "true"],
            vec!["--ta-receipts", "none", "--ta-receipts", "none"],
        ] {
            assert_eq!(request(&args(&extra)).err().unwrap().rc, 2);
        }
    }
}
