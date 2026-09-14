use std::process::ExitCode;
mod census;
mod common;
mod compendium;
mod fork;
mod inventories;
mod projection;
mod text;
mod vocabulary;

mod adjudication;
mod adjudication_fixtures;
mod corpus;
mod coverage;
mod documents;
mod lexicon;
mod probes;
mod process;
mod queries;
mod query_fixtures;
mod red;
fn check() -> common::Result {
    fork::check()?;
    probes::docid()?;
    probes::trace_numeric()?;
    probes::wall()?;
    adjudication_fixtures::check()?;
    compendium::check()?;
    let plans = inventories::guidelines()?;
    let red = inventories::red()?;
    inventories::prolog()?;
    for g in &plans {
        corpus::check(g)?;
    }
    let swipl = process::swipl()?;
    let scratch = process::Scratch::new()?;
    let stage = process::stage(&scratch, &swipl)?;
    let mut documents = 0;
    for g in &plans {
        documents::check(&scratch, &swipl, &stage, g)?;
        documents += g.docids.len();
    }
    query_fixtures::check(&scratch, &swipl, &stage)?;
    for probe in &red {
        red::run(&swipl, &stage, probe)?;
    }
    drop(scratch);
    println!(
        "goal: check ok {} guidelines {documents} documents {} red probes",
        plans.len(),
        red.len()
    );
    Ok(())
}
pub fn run(root: &str) -> ExitCode {
    if std::env::set_current_dir(root).is_err() {
        return common::emit(Err(common::fail("check", format!("unreadable {root}"))));
    }
    common::emit(check())
}
