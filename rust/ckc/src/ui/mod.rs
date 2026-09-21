use std::process::ExitCode;
use std::path::Path;

mod chrome;
mod common;
mod corpus;
mod intake;
mod pages;

const USAGE: &str = "ui: usage: expected: ui serve [<port>] [<root>] | ui render <outdir> [<root>] | ui check [<root>] | ui request <method> <path> [<root>] [--header <name:value>]* [--body <text>] [--body-hex <hex>] [--token <text>] [--now <utc-iso>] [--commit <40hex>] [--fault after-tmp-write]";

// Shell units: committed intake, kernel render/check, request/serve CAS,
// copy fixture envelope. The fixture suite grades each pending entry red.
pub(crate) fn run(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        Some("render") if (2..=3).contains(&args.len()) => {
            let dest = Path::new(&args[1]);
            if pages::destination_blocked(dest) {
                eprintln!("ui: render: destination not empty: {}", args[1]);
                return ExitCode::from(2);
            }
            finish(pages::render(Path::new(args.get(2).map(String::as_str).unwrap_or(".")), dest))
        }
        Some("check") if args.len() <= 2 => {
            finish(pages::check(Path::new(args.get(1).map(String::as_str).unwrap_or("."))))
        }
        Some("request" | "serve" | "copy-check") => {
            eprintln!("ui: shell integration pending");
            ExitCode::from(1)
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn finish(result: common::Result<()>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => { eprintln!("{error}"); ExitCode::from(1) }
    }
}
