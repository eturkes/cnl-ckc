use std::process::ExitCode;
mod agent_law;
mod agent_spec;
mod census;
mod common;
mod compendium;
pub(crate) mod dist;
mod dist_archive;
mod dist_check;
mod dist_probes;
mod fork;
mod functors;
mod inventories;
mod projection;
mod text;

mod adjudication;
mod adjudication_fixtures;
mod corpus;
mod coverage;
mod cycle;
mod derived;
mod documents;
mod green;
mod lexicon;
mod par;
pub(crate) mod pipeline;
mod pipeline_emit;
mod pipeline_release;
mod probes;
mod process;
mod queries;
mod query_fixtures;
mod red;
pub(crate) mod schema;
fn check() -> common::Result {
    fork::check()?;
    probes::docid()?;
    probes::trace_numeric()?;
    probes::wall()?;
    adjudication_fixtures::check()?;
    compendium::check()?;
    agent_spec::check()?;
    agent_law::check()?;
    let plans = inventories::guidelines()?;
    let red = inventories::red()?;
    let green = inventories::green()?;
    inventories::prolog()?;
    schema::check()?;
    for g in &plans {
        corpus::check(g)?;
    }
    dist_check::check()?;
    let swipl = process::swipl()?;
    let scratch = process::Scratch::new()?;
    let stage = process::stage(&scratch, &swipl)?;
    let mut documents = 0;
    for g in &plans {
        documents::check(&scratch, &swipl, &stage, g)?;
        documents += g.docids.len();
    }
    query_fixtures::check(&scratch, &swipl, &stage)?;
    par::map(&red, |probe| red::run(&swipl, &stage, probe))?;
    let proofs = par::map(&green, |probe| green::run(&swipl, &stage, probe))?;
    drop(scratch);
    println!(
        "ckc: check ok {} guidelines {documents} documents {} red probes {} green probes ({} proof)",
        plans.len(),
        red.len(),
        green.len(),
        proofs.iter().filter(|p| **p).count()
    );
    Ok(())
}
// The shell's one SWI-Prolog mechanism for callers outside `check` (certify): the
// SWIPL | PATH program under the 10.0.2 guard, each run inside the group wall.
pub(crate) fn swipl_program() -> std::result::Result<std::path::PathBuf, ExitCode> {
    process::swipl().map_err(|f| common::emit(Err(f)))
}
pub(crate) fn swipl_run(
    command: &mut std::process::Command,
    input: Option<&[u8]>,
) -> std::result::Result<(i32, Vec<u8>), &'static str> {
    match process::walled(command, input, process::WALL) {
        Err(_) => Err("swipl_unavailable."),
        Ok((true, _)) => Err("swipl_timeout."),
        Ok((false, o)) => Ok((o.rc, o.out)),
    }
}
pub fn run(root: &str) -> ExitCode {
    if std::env::set_current_dir(root).is_err() {
        return common::emit(Err(common::fail("check", format!("unreadable {root}"))));
    }
    common::emit(check())
}
