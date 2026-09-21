use std::process::ExitCode;

const USAGE: &str = "ui: usage: expected: ui serve [<port>] [<root>] | ui render <outdir> [<root>] | ui check [<root>] | ui request <method> <path> [<root>] [--header <name:value>]* [--body <text>] [--body-hex <hex>] [--token <text>] [--now <utc-iso>] [--commit <40hex>] [--fault after-tmp-write]";

// Shell units: committed intake, kernel render/check, request/serve CAS,
// copy fixture envelope. The fixture suite grades each pending entry red.
pub(crate) fn run(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        Some("render" | "check" | "request" | "serve" | "copy-check") => {
            eprintln!("ui: shell integration pending");
            ExitCode::from(1)
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}
