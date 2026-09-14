use std::process::ExitCode;
mod common;
mod fork;
mod inventories;
mod text;

fn check() -> common::Result {
    fork::check()?;
    let plans = inventories::guidelines()?;
    let red = inventories::red()?;
    inventories::prolog()?;
    for g in &plans {
        for id in &g.docids {
            let _ = (g.ace(id), g.pl(id), &g.lexicon);
        }
    }
    for probe in &red {
        let _ = (&probe.path, &probe.class, probe.rc);
    }
    Err(common::fail("check", "shell implementation pending"))
}
pub fn run(root: &str) -> ExitCode {
    if std::env::set_current_dir(root).is_err() {
        return common::emit(Err(common::fail("check", format!("unreadable {root}"))));
    }
    common::emit(check())
}
