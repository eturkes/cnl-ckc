use std::path::Path;
use std::process::ExitCode;

mod chrome;
mod common;
mod copy;
mod corpus;
mod fresh;
mod intake;
mod pages;
mod request;
mod serve;

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
            finish(pages::render(
                Path::new(args.get(2).map(String::as_str).unwrap_or(".")),
                dest,
            ))
        }
        Some("check") if args.len() <= 2 => finish(pages::check(Path::new(
            args.get(1).map(String::as_str).unwrap_or("."),
        ))),
        Some("copy-check") if args.len() == 2 => finish_code(copy::check(Path::new(&args[1]))),
        Some("request") => match request::parse(&args[1..]) {
            Some(args) => finish_code(request::execute(&args)),
            None => usage(),
        },
        Some("serve") if args.len() <= 3 => {
            let port = match args.get(1) {
                Some(value) => match value.trim().parse::<u16>() {
                    Ok(port) if port >= 1024 => port,
                    _ => return usage(),
                },
                None => 8377,
            };
            finish(serve::run(
                Path::new(args.get(2).map(String::as_str).unwrap_or(".")),
                port,
            ))
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
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

fn finish_code(result: common::Result<u8>) -> ExitCode {
    match result {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}
fn usage() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::from(2)
}
