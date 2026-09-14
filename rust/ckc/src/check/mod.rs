use std::process::ExitCode;
mod census;
mod common;
mod compendium;
mod fork;
mod inventories;
mod projection;
mod text;
mod vocabulary;

fn check() -> common::Result {
    fork::check()?;
    compendium::check()?;
    let plans = inventories::guidelines()?;
    let red = inventories::red()?;
    inventories::prolog()?;
    for g in &plans {
        projection::check(&g.path)?;
        let status: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
        let _ = census::check;
        let _ = status;
        for id in &g.docids {
            vocabulary::check(&g.path, id)?;
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
