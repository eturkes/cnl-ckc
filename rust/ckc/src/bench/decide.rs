// Contract sb § Carried rulings (decision tool, replacement round, R pairs, typed analysis at load):
// the confirmation's decision over the frozen plan + its frozen closure + every attempt's
// `runs/<id>` + `state/<id>` under one base (a chunk's replacement = `<chunk>-r`). The closure's
// named preflight must go. Every attempt must be sound: the harness finished (rc 0 | 1), inputs
// match the manifest (every planned query's identity included), runs.tsv holds each planned call
// once, verified outputs match their expected digests, every (instance, query) has one, every
// verified call carries a memory bound, every block's calls were timed (an untimed call = protocol-
// incomplete, never a replaceable block: scientist-8 S6), the lifecycle peak is recorded + ≤ 32 GB,
// each arm has one measured ready load, the policy held, the closure's settings, host keys +
// identity files match, and the plan's sealed executables ran under its settings (seed, blocks,
// queries, spacing, the replayed blocks + the original's draws for them) on its schedule (planned
// on one epoch, no early start, actual requests ascending + spaced). A block is valid where X1–X4
// accept it on a pinned unit; its selected attempt = the first valid one in attempt order
// (latency, memory + output never consulted). Each chunk needs every planned block selected, every
// selected call verified within 300 s + 32 GB (its bound included); R + UB95 over the gating R
// chunk's selected pairs (`stats::ratio`, the chunk seed) ≤ 2, the reported R beside it; every
// measured knowledge identity joins an admitted direct receipt of the final census (full load ≤
// 1,800 s; charged memory = max(the analysis VmHWM + the server's RSS at ready, the attempt's
// lifecycle peak) ≤ 32 GB). rc 0 = go, 1 = hold (one `hold` line per failed condition), 2 = usage
// or an unreadable input.
use super::grade::{
    BUDGET_BYTES, Call, Manifest, Result, Row, assess, cell, exit, manifest, number, policy,
    preflight_assess, table, text,
};
use super::stats::ratio;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const HELP: &str = "usage: ckc bench decide <plan.tsv> <closure.tsv> <base> <frozen-root> <run-manifest> [--receipts <dir> --census <sha256> --host <label> --overhead-ms <n>]";
// The receipt flags come together, in this order (all four or none).
const LOAD_NS: f64 = 1.8e12;
const SPACING_NS: u64 = 30_000_000_000;
const UB95_MAX: f64 = 2.0;
// Block b's first request trails the first held block's by ≥ spacing × Δb less this (the epoch's own
// wait before the first block).
const SLACK_NS: i128 = 1_000_000_000;

pub fn run(args: &[String]) -> ExitCode {
    exit("decide", decide(args))
}

struct Chunk {
    id: String,
    instance: String,
    blocks: u64,
    seed: u64,
    pair: Option<String>,
    ratio: String,
    ckc: String,
    checker: String,
}
impl Chunk {
    fn arms(&self) -> BTreeMap<String, String> {
        let mut arms = BTreeMap::from([("target".to_owned(), self.instance.clone())]);
        if let Some(pair) = &self.pair {
            arms.insert("baseline".into(), pair.clone());
        }
        arms
    }
    // (arm, query) per block: the R pair = q-one on both arms, a chunk = every query on its server.
    fn draws(&self) -> BTreeSet<(String, String)> {
        let draws: &[(&str, &str)] = if self.pair.is_some() {
            &[("baseline", "q-one"), ("target", "q-one")]
        } else {
            &[
                ("target", "q-all"),
                ("target", "q-one"),
                ("target", "q-patient"),
            ]
        };
        draws
            .iter()
            .map(|(a, q)| ((*a).to_owned(), (*q).to_owned()))
            .collect()
    }
}
fn chunks(path: &Path) -> Result<Vec<Chunk>> {
    let mut out: Vec<Chunk> = Vec::new();
    for row in table(path)? {
        let get = |key: &str| cell(&row, key).map(str::to_owned);
        let pair = get("pair")?;
        let c = Chunk {
            id: get("chunk")?,
            instance: get("instance")?,
            blocks: number(&row, "blocks")?.ok_or("plan: blocks")?,
            seed: number(&row, "seed")?.ok_or("plan: seed")?,
            pair: (pair != "-").then_some(pair),
            ratio: get("ratio")?,
            ckc: get("ckc")?,
            checker: get("checker")?,
        };
        let role = match c.pair {
            Some(_) => ["gate", "report"].contains(&c.ratio.as_str()),
            None => c.ratio == "-",
        };
        if !role || c.blocks == 0 || out.iter().any(|o| o.id == c.id) {
            return Err(format!("plan: chunk {}", c.id));
        }
        out.push(c);
    }
    if out.iter().filter(|c| c.ratio == "gate").count() != 1 {
        return Err("plan: expected exactly one gating R chunk".into());
    }
    Ok(out)
}
fn keyed(path: &Path) -> Result<BTreeMap<String, String>> {
    table(path)?
        .iter()
        .map(|r| Ok((cell(r, "key")?.to_owned(), cell(r, "value")?.to_owned())))
        .collect()
}
fn stem(path: &str, parent: bool) -> String {
    let path = Path::new(path);
    let name = if parent {
        path.parent().and_then(Path::file_name)
    } else {
        path.file_stem()
    };
    name.and_then(|n| n.to_str()).unwrap_or("").to_owned()
}

struct Receipts<'a> {
    dir: &'a Path,
    census: &'a str,
    host: &'a str,
    overhead_ns: f64,
}
// A measured prepared load: (attempt, arm, load_ns, rss_at_ready_bytes, the attempt's lifecycle
// summed-RSS peak).
type Load = (String, String, u64, u64, u64);
// The frozen closure (scientist-8 S8 + S9; `bench/sb/confirmation-closure.tsv`, rows `source key
// value`): settings.tsv + host.tsv keys every attempt carries verbatim, identity files (a path
// suffix → its sha256 in identities.tsv), each planned instance's knowledge identity (the 64-hex
// `kb_identity` its server must report: the receipt join's key), the preflight state(s) whose grade
// must go.
#[derive(Default)]
struct Closure {
    settings: Vec<(String, String)>,
    host: Vec<(String, String)>,
    identity: Vec<(String, String)>,
    kb: BTreeMap<String, String>,
    preflight: Vec<String>,
}
fn closure(path: &Path) -> Result<Closure> {
    let mut c = Closure::default();
    for row in table(path)? {
        let (key, value) = (
            cell(&row, "key")?.to_owned(),
            cell(&row, "value")?.to_owned(),
        );
        match cell(&row, "source")? {
            "settings" => c.settings.push((key, value)),
            "host" => c.host.push((key, value)),
            "identity" => c.identity.push((key, value)),
            "kb" => {
                if c.kb.insert(key.clone(), value).is_some() {
                    return Err(format!("closure: kb {key} twice"));
                }
            }
            "preflight" if value == "go" => c.preflight.push(key),
            source => return Err(format!("closure: {source} {key}")),
        }
    }
    if c.preflight.is_empty() {
        return Err("closure: no preflight row".into());
    }
    Ok(c)
}
#[derive(Default)]
struct Decision {
    out: Vec<String>,
    holds: Vec<String>,
    loads: BTreeMap<String, Vec<Load>>,
}

fn decide(args: &[String]) -> Result<bool> {
    let (positional, flags) = args.split_at(args.len().min(5));
    let [plan, closure_path, base, frozen, manifest_path] = positional else {
        return Err(HELP.into());
    };
    let receipts = match flags {
        [] => None,
        [r, dir, c, census, h, host, o, overhead]
            if r == "--receipts" && c == "--census" && h == "--host" && o == "--overhead-ms" =>
        {
            let ms: u64 = overhead
                .parse()
                .map_err(|_| "--overhead-ms: expected a natural number")?;
            Some(Receipts {
                dir: Path::new(dir),
                census,
                host,
                overhead_ns: ms as f64 * 1e6,
            })
        }
        _ => return Err(HELP.into()),
    };
    let (base, frozen) = (Path::new(base), Path::new(frozen));
    let m = manifest(Path::new(manifest_path))?;
    let plan = chunks(Path::new(plan))?;
    let frozen_closure = closure(Path::new(closure_path))?;
    let mut d = Decision::default();
    for id in &frozen_closure.preflight {
        match preflight_assess(&base.join("state").join(id)) {
            Ok((go, report)) => {
                let line = report.last().map_or("", String::as_str);
                let line = line.strip_prefix("preflight\t").unwrap_or(line);
                d.out.push(format!("preflight\t{id}\t{line}"));
                if !go {
                    d.holds.push(format!("preflight {id}: hold"));
                }
            }
            Err(why) => d.holds.push(format!("preflight {id}: {why}")),
        }
    }
    d.out.push(
        "#attempt\tchunk\tattempt\trc\tidentity\tsha_bad\tsha_unchecked\tmemory_unbounded\tpinned\tvalid_blocks"
            .into(),
    );
    d.out
        .push("#block\tchunk\tattempt\tblock\tgrade\tvalidity".into());
    d.out.push("#selected\tchunk\tblock\tattempt".into());
    // Every attempt directory is planned: a chunk's own id or its one replacement.
    let ids: BTreeSet<String> = plan
        .iter()
        .flat_map(|c| [c.id.clone(), format!("{}-r", c.id)])
        .collect();
    let runs = base.join("runs");
    let entries = fs::read_dir(&runs).map_err(|e| format!("{}: {e}", runs.display()))?;
    for entry in entries {
        let name = entry.map_err(|e| e.to_string())?.file_name();
        let name = name.to_string_lossy();
        if !ids.contains(name.as_ref()) {
            d.holds.push(format!("unplanned attempt runs/{name}"));
        }
    }
    for c in &plan {
        chunk(c, base, frozen, &m, &frozen_closure, &mut d)?;
    }
    match &receipts {
        None => {
            d.out.push("receipt_join\tpending".into());
            d.holds
                .push("receipt join pending: census verdicts outstanding".into());
        }
        Some(r) => join(r, &mut d),
    }
    let holds = d.holds.len();
    for h in &d.holds {
        d.out.push(format!("hold\t{h}"));
    }
    d.out.push(format!(
        "decide\tverdict={}\tholds={holds}",
        if holds == 0 { "go" } else { "hold" }
    ));
    println!("{}", d.out.join("\n"));
    Ok(holds == 0)
}

// The plan's settings, executables + draws: the blocks this attempt holds + each nonconformance; a
// replacement lists its blocks (`only_blocks`) and replays the original's rows for them.
fn conform(
    c: &Chunk,
    run: &Path,
    plan: &[Row],
    original: Option<&[Row]>,
    frozen: &Closure,
) -> Result<(BTreeSet<u64>, Vec<String>)> {
    let mut bad = Vec::new();
    let settings = keyed(&run.join("settings.tsv"))?;
    let host = keyed(&run.join("host.tsv"))?;
    for (source, rows, keys) in [
        ("settings", &settings, &frozen.settings),
        ("host", &host, &frozen.host),
    ] {
        for (key, want) in keys {
            if rows.get(key) != Some(want) {
                bad.push(format!("closure {source} {key}"));
            }
        }
    }
    // identities.tsv: (path, sha256) per recorded file, every arm's.
    let recorded: Vec<(String, String)> = text(&run.join("identities.tsv"))?
        .lines()
        .skip(1)
        .filter_map(|l| match l.split('\t').collect::<Vec<_>>()[..] {
            [_, path, _, sha] => Some((path.to_owned(), sha.to_owned())),
            _ => None,
        })
        .collect();
    for (suffix, want) in &frozen.identity {
        let tail = format!("/{suffix}");
        let hits: Vec<&String> = recorded
            .iter()
            .filter(|(p, _)| p.ends_with(&tail))
            .map(|(_, h)| h)
            .collect();
        if hits.is_empty() || hits.iter().any(|h| *h != want) {
            bad.push(format!("closure identity {suffix}"));
        }
    }
    let (queries, spacing) = match c.pair {
        Some(_) => ("q-one", SPACING_NS),
        None => ("q-all,q-one,q-patient", 0),
    };
    for (key, want) in [
        ("seed", c.seed.to_string()),
        ("blocks", c.blocks.to_string()),
        ("queries", queries.to_owned()),
        ("spacing_ns", spacing.to_string()),
    ] {
        if settings.get(key) != Some(&want) {
            bad.push(format!("settings {key}"));
        }
    }
    if host.get("harness_sha256") != Some(&c.ckc) {
        bad.push("ckc executable".into());
    }
    // The served checker = the serve command's first word, hashed in identities.tsv.
    let served = settings
        .get("serve")
        .and_then(|s| s.split_whitespace().next())
        .unwrap_or("");
    let hashes: Vec<&String> = recorded
        .iter()
        .filter(|(p, _)| p == served)
        .map(|(_, h)| h)
        .collect();
    if hashes.is_empty() || hashes.iter().any(|h| **h != c.checker) {
        bad.push("checker executable".into());
    }
    let listed = settings.get("only_blocks").map_or("", String::as_str);
    let listed: BTreeSet<u64> = if listed.is_empty() {
        BTreeSet::new()
    } else {
        listed
            .split(',')
            .map(|b| b.parse::<u64>().map_err(|_| format!("only_blocks {b}")))
            .collect::<Result<_>>()?
    };
    let held: BTreeSet<u64> = match original {
        None if listed.is_empty() => (1..=c.blocks).collect(),
        Some(_) if !listed.is_empty() => listed.clone(),
        _ => {
            bad.push("settings only_blocks".into());
            listed.clone()
        }
    };
    let (arms, draws) = (c.arms(), c.draws());
    let mut seen: BTreeMap<u64, BTreeSet<(String, String)>> = BTreeMap::new();
    let mut misdrawn = false;
    for row in plan {
        let arm = cell(row, "arm")?;
        let block = number(row, "block")?.ok_or("plan.tsv block")?;
        misdrawn |= arms.get(arm) != Some(&stem(cell(row, "composition")?, true));
        let draw = (arm.to_owned(), stem(cell(row, "query")?, false));
        misdrawn |= !seen.entry(block).or_default().insert(draw);
    }
    misdrawn |= seen.keys().copied().collect::<BTreeSet<_>>() != held;
    misdrawn |= seen.values().any(|s| *s != draws);
    if let Some(original) = original {
        let of = |p: &[Row]| -> Vec<Row> {
            p.iter()
                .filter(|r| {
                    number(r, "block")
                        .ok()
                        .flatten()
                        .is_some_and(|b| listed.contains(&b))
                })
                .cloned()
                .collect()
        };
        misdrawn |= of(plan) != of(original);
    }
    if misdrawn {
        bad.push("plan draws".into());
    }
    Ok((held, bad))
}

// The actual schedule (scientist-8 S8): schedule.tsv = one row per held block, ascending, each
// planned on one epoch (planned = epoch + spacing × (block − 1)) + started no earlier; the calls'
// request writes ascend in run order, and each block's first request trails the first held block's
// by ≥ spacing × Δblock − SLACK_NS. Each failed item, by name.
fn schedule(run: &Path, held: &BTreeSet<u64>, spacing: u64, calls: &[Call]) -> Vec<String> {
    let mut bad = Vec::new();
    let Ok(rows) = table(&run.join("schedule.tsv")) else {
        return vec!["schedule.tsv unreadable".into()];
    };
    let (mut blocks, mut epoch) = (Vec::new(), None);
    for row in &rows {
        let get = |key: &str| number(row, key).ok().flatten().map(i128::from);
        let (Some(b), Some(planned), Some(started)) =
            (get("block"), get("planned_ns"), get("started_ns"))
        else {
            bad.push("schedule.tsv row unreadable".into());
            continue;
        };
        blocks.push(b as u64);
        let e = planned - i128::from(spacing) * (b - 1);
        if *epoch.get_or_insert(e) != e {
            bad.push(format!("schedule: block {b} planned off the epoch"));
        }
        if started < planned {
            bad.push(format!("schedule: block {b} started early"));
        }
    }
    if blocks != held.iter().copied().collect::<Vec<_>>() {
        bad.push("schedule: blocks differ from the held blocks".into());
    }
    let mut ordered: Vec<&Call> = calls.iter().collect();
    ordered.sort_by_key(|x| x.run);
    let (mut last, mut first) = (None, BTreeMap::new());
    for x in ordered {
        let Some(at) = x.request else { continue };
        if last.is_some_and(|l| at <= l) {
            bad.push(format!(
                "run {}: requested before the call ahead of it",
                x.run
            ));
        }
        last = Some(at);
        first.entry(x.block).or_insert(at);
    }
    if let Some((&b0, &r0)) = first.iter().next() {
        for (&b, &at) in &first {
            let gap = i128::from(at) - i128::from(r0);
            let due = i128::from(spacing) * (i128::from(b) - i128::from(b0)) - SLACK_NS;
            if b > b0 && gap < due {
                bad.push(format!(
                    "block {b}: first request {:.1} s after block {b0}'s",
                    gap as f64 / 1e9
                ));
            }
        }
    }
    bad
}
fn chunk(
    c: &Chunk,
    base: &Path,
    frozen: &Path,
    m: &Manifest,
    sealed: &Closure,
    d: &mut Decision,
) -> Result<()> {
    let mut attempts = Vec::new();
    for id in [c.id.clone(), format!("{}-r", c.id)] {
        let run = base.join("runs").join(&id);
        if run.is_dir() {
            let a = assess(&run, &base.join("state").join(&id), frozen, m)?;
            attempts.push((id, a));
        }
    }
    if attempts.first().map(|(id, _)| id) != Some(&c.id) {
        d.holds.push(format!("{}: no original attempt", c.id));
        return Ok(());
    }
    let mut invalid = BTreeSet::new();
    let mut chosen: BTreeMap<u64, usize> = BTreeMap::new();
    for (k, (id, a)) in attempts.iter().enumerate() {
        let (run, state) = (base.join("runs").join(id), base.join("state").join(id));
        let original = (k > 0).then(|| attempts[0].1.plan.as_slice());
        let (held, mut bad) = conform(c, &run, &a.plan, original, sealed)?;
        let spacing = if c.pair.is_some() { SPACING_NS } else { 0 };
        bad.extend(schedule(&run, &held, spacing, &a.calls));
        let applied = policy(&state)?;
        if !applied.is_empty() {
            bad.push(format!("policy {}", applied.join("+")));
        }
        if !a.bijective {
            bad.push("runs.tsv ≠ plan.tsv (a planned call missing, repeated or moved)".into());
        }
        let untimed: Vec<String> = held
            .iter()
            .filter(|b| a.verdicts.get(b).map(String::as_str) == Some("incomplete"))
            .map(u64::to_string)
            .collect();
        if !untimed.is_empty() {
            bad.push(format!(
                "protocol-incomplete: block {} holds a call never timed",
                untimed.join(",")
            ));
        }
        // The campaign-wide summed-RSS peak (loads, calls, idle, teardown) = memory too (scientist-8
        // S7): recorded, the sampler error-free, ≤ 32 GB.
        let memory = keyed(&run.join("memory.tsv")).unwrap_or_default();
        let lifecycle = memory
            .get("peak_summed_rss_bytes")
            .and_then(|v| v.parse::<u64>().ok());
        if memory.get("sampler_error").map(String::as_str) != Some("none") {
            bad.push("lifecycle memory: sampler error or no record".into());
        }
        match lifecycle {
            None => bad.push("lifecycle peak unrecorded".into()),
            Some(p) if p > BUDGET_BYTES => bad.push(format!(
                "lifecycle peak {:.2} GB above 32 GB",
                p as f64 / 1e9
            )),
            Some(_) => {}
        }
        if k > 0 && !held.is_subset(&invalid) {
            bad.push("replaces a valid block".into());
        }
        if !["0", "1"].contains(&a.rc.as_str()) {
            bad.push(format!("harness rc {}", a.rc));
        }
        if !a.matched {
            bad.push("input identity".into());
        }
        for (count, what) in [
            (a.wrong, "wrong output digests"),
            (a.unchecked, "calls without an expected output"),
            (a.unbounded, "verified calls without a memory bound"),
        ] {
            if count > 0 {
                bad.push(format!("{count} {what}"));
            }
        }
        // One ready load per arm, measured (an empty cell is no free load or RSS: scientist-8 S8).
        for arm in c.arms().keys() {
            let loads: Vec<&Row> = a
                .loads
                .iter()
                .filter(|l| l.get("arm") == Some(arm))
                .collect();
            match (&loads[..], a.kb.get(arm)) {
                ([l], Some(kb)) if l.get("outcome").map(String::as_str) == Some("ready") => {
                    let instance = &c.arms()[arm];
                    if sealed.kb.get(instance) != Some(kb) {
                        bad.push(format!("arm {arm}: knowledge identity off the closure's"));
                    }
                    match (number(l, "load_ns")?, number(l, "rss_at_ready_bytes")?) {
                        (Some(ns), Some(rss)) if rss > 0 => {
                            let load = (id.clone(), arm.clone(), ns, rss, lifecycle.unwrap_or(0));
                            d.loads.entry(kb.clone()).or_default().push(load);
                        }
                        _ => bad.push(format!("arm {arm}: ready load unmeasured")),
                    }
                }
                _ => bad.push(format!(
                    "arm {arm}: no single ready load with a knowledge identity"
                )),
            }
        }
        let mut valid = 0;
        for &b in &held {
            let grade = a.verdicts.get(&b).map_or("absent", String::as_str);
            let ok = a.pinned && grade == "accepted";
            if k == 0 && !ok {
                invalid.insert(b);
            }
            if ok {
                valid += 1;
                chosen.entry(b).or_insert(k);
            }
            let validity = if ok { "valid" } else { "invalid" };
            d.out
                .push(format!("block\t{}\t{id}\t{b}\t{grade}\t{validity}", c.id));
        }
        d.out.push(format!(
            "attempt\t{}\t{id}\t{}\t{}\t{}\t{}\t{}\t{}\t{valid}",
            c.id,
            a.rc,
            if a.matched { "ok" } else { "bad" },
            a.wrong,
            a.unchecked,
            a.unbounded,
            if a.pinned { "ok" } else { "NOT-PINNED" }
        ));
        for b in bad {
            d.holds.push(format!("{}: attempt {id}: {b}", c.id));
        }
    }
    let mut pairs = Vec::new();
    let mut fails = 0;
    for b in 1..=c.blocks {
        let Some(&k) = chosen.get(&b) else {
            d.out.push(format!("selected\t{}\t{b}\t-", c.id));
            d.holds
                .push(format!("{}: block {b}: no valid attempt", c.id));
            continue;
        };
        let (id, a) = &attempts[k];
        d.out.push(format!("selected\t{}\t{b}\t{id}", c.id));
        let planned = a
            .plan
            .iter()
            .filter(|r| number(r, "block").ok().flatten() == Some(b))
            .count();
        let calls: Vec<_> = a.calls.iter().filter(|x| x.block == b).collect();
        if calls.len() != planned {
            d.holds.push(format!(
                "{}: block {b}: {} of {planned} planned calls ran",
                c.id,
                calls.len()
            ));
        }
        for x in &calls {
            if !(x.budget && x.bound.is_some_and(|v| v <= BUDGET_BYTES)) {
                fails += 1;
                d.holds.push(format!(
                    "{}: block {b}: {} {} {} outside the budget",
                    c.id, x.arm, x.query, x.outcome
                ));
            }
        }
        if c.pair.is_some() {
            let score = |arm: &str| {
                calls
                    .iter()
                    .find(|x| x.arm == arm && x.query == "q-one" && x.outcome == "verified")
                    .and_then(|x| x.scored)
            };
            if let (Some(base), Some(target)) = (score("baseline"), score("target"))
                && base > 0
            {
                pairs.push((base as f64, target as f64));
            }
        }
    }
    if c.pair.is_some() {
        if pairs.len() as u64 == c.blocks {
            let (r, upper) = ratio(&pairs, c.seed);
            d.out.push(format!(
                "ratio\t{}\t{}\tpairs={}\tR={r:.9}\tUB95={upper:.9}",
                c.id,
                c.ratio,
                pairs.len()
            ));
            if c.ratio == "gate" && upper > UB95_MAX {
                d.holds
                    .push(format!("{}: R upper bound {upper:.3} above 2", c.id));
            }
        } else {
            d.out.push(format!(
                "ratio\t{}\t{}\tpairs={}\tnot-estimable",
                c.id,
                c.ratio,
                pairs.len()
            ));
            if c.ratio == "gate" {
                d.holds.push(format!(
                    "{}: R not estimable over {} of {} pairs",
                    c.id,
                    pairs.len(),
                    c.blocks
                ));
            }
        }
    }
    d.out.push(format!(
        "chunk\t{}\tselected={}\tblocks={}\tattempts={}\tbudget_fail={fails}",
        c.id,
        chosen.len(),
        c.blocks,
        attempts.len()
    ));
    Ok(())
}

// The receipt of one measured knowledge identity: (the largest full load in ns, the largest charged
// memory in bytes) over its loads, or why it fails to join.
fn receipt(r: &Receipts, kb: &str, loads: &[Load]) -> std::result::Result<(f64, u64), String> {
    if kb.len() != 64 || !kb.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return Err("malformed identity".into());
    }
    let body =
        fs::read_to_string(r.dir.join(format!("{kb}.tsv"))).map_err(|_| "no receipt".to_owned())?;
    let lines: Vec<(&str, &str)> = body.lines().filter_map(|l| l.split_once('\t')).collect();
    let fields: BTreeMap<&str, &str> = lines.iter().copied().collect();
    if fields.len() != lines.len() || lines.len() != body.lines().count() {
        return Err("malformed receipt".into());
    }
    for (key, want) in [
        ("format", "sb1-ta-receipt 1"),
        ("kb_identity", kb),
        ("verdict", "admitted"),
        ("dc", "1"),
        ("ta_binary_sha256", r.census),
        ("host", r.host),
    ] {
        if fields.get(key) != Some(&want) {
            return Err(format!("{key} {}", fields.get(key).unwrap_or(&"absent")));
        }
    }
    if fields.contains_key("basis") {
        return Err("derived (an alias carries admission alone)".into());
    }
    let seconds = |key: &str| {
        fields
            .get(key)
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|v| v.is_finite() && *v >= 0.0)
            .ok_or_else(|| format!("{key} unmeasured"))
    };
    let analysis = seconds("parse_seconds")? + seconds("analysis_seconds")?;
    let hwm: u64 = fields
        .get("vmhwm_bytes")
        .and_then(|v| v.parse().ok())
        .filter(|v| *v > 0)
        .ok_or("vmhwm_bytes unmeasured")?;
    let full = loads
        .iter()
        .map(|l| l.2 as f64 + analysis * 1e9 + r.overhead_ns)
        .fold(0.0, f64::max);
    // The record's charge: max(the analysis VmHWM + the server's RSS at ready, the attempt's
    // measured summed-RSS peak).
    let charged = loads
        .iter()
        .map(|l| hwm.saturating_add(l.3).max(l.4))
        .max()
        .unwrap_or(hwm);
    Ok((full, charged))
}
fn join(r: &Receipts, d: &mut Decision) {
    d.out
        .push("#receipt\tkb_identity\tloads\tfull_load_s\tcharged_gb\tstatus".into());
    for (kb, loads) in &d.loads {
        let status = match receipt(r, kb, loads) {
            Ok((full, charged)) => {
                let mut why = Vec::new();
                if full > LOAD_NS {
                    why.push(format!("full load {:.1} s above 1,800 s", full / 1e9));
                }
                if charged > BUDGET_BYTES {
                    why.push(format!(
                        "charged memory {:.2} GB above 32 GB",
                        charged as f64 / 1e9
                    ));
                }
                d.out.push(format!(
                    "receipt\t{kb}\t{}\t{:.1}\t{:.2}\t{}",
                    loads.len(),
                    full / 1e9,
                    charged as f64 / 1e9,
                    if why.is_empty() { "ok" } else { "over" }
                ));
                why
            }
            Err(why) => {
                d.out
                    .push(format!("receipt\t{kb}\t{}\tNA\tNA\t{why}", loads.len()));
                vec![why]
            }
        };
        for why in status {
            d.holds.push(format!("receipt {kb}: {why}"));
        }
    }
}
