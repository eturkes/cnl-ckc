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
    Assessment, BUDGET_BYTES, Call, Manifest, Result, Row, assess, cell, exit, manifest, number,
    policy, preflight_assess, table, text,
};
use super::stats::ratio;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const HELP: &str = "usage: ckc bench decide <plan.tsv> <closure.tsv> <base> <frozen-root> <run-manifest> [--receipts <dir> --census <sha256> --host <label> --overhead-ms <n> | --whole-vector]";
// TAS-P2 (`--whole-vector`): the closure's `unit` rows every attempt's unit records must carry.
const UNIT_KEYS: [&str; 4] = ["tool_sha256", "censor_s", "memory.max", "memory.swap.max"];
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
    unit: BTreeMap<String, String>,
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
            "unit" => {
                c.unit.insert(key, value);
            }
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
    let whole = matches!(flags, [w] if w == "--whole-vector");
    let receipts = match flags {
        [] => None,
        [_] if whole => None,
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
    if whole
        && let Some(key) = UNIT_KEYS
            .iter()
            .find(|k| !frozen_closure.unit.contains_key(**k))
    {
        return Err(format!("closure: no unit {key} row (--whole-vector)"));
    }
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
        chunk(c, base, frozen, &m, &frozen_closure, &mut d, whole)?;
    }
    match &receipts {
        None if whole => {}
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
    whole: bool,
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
        // TAS-P2: a whole-vector replacement replays every block.
        Some(_) if whole && listed.is_empty() => (1..=c.blocks).collect(),
        Some(_) if !whole && !listed.is_empty() => listed.clone(),
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
                        .is_some_and(|b| held.contains(&b))
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
// TAS-P2 integrity of one attempt (`.scratch/imp/tas-p2-protocol.md` P2-1, P2-5, P2-6, P2-10): the
// unit's capsule, censor + main-process records against the closure's `unit` rows; no launcher-report
// failure; a complete load bound (U ≥ L, + the unit main ≤ 32 GB); each planned arm's launcher +
// server terminal own term re-derived from its raw self-report, the harness's samples, the unit
// roster's readings + its launcher's startup record, the harness's terminal VmHWM recorded; no
// unplanned directory; every unit-cgroup process classified (harness-observed, a report-bound
// producer, the unit main, or a hook outside the main's lifetime).
// Returns the failed items, the unit main's term + the teardown Σ of terminal own terms.
fn p2(
    run: &Path,
    state: &Path,
    a: &Assessment,
    unit: &BTreeMap<String, String>,
    arms: &BTreeSet<String>,
) -> (Vec<String>, u64, u64) {
    let mut bad = Vec::new();
    let kv = |path: PathBuf| -> BTreeMap<String, String> {
        fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| l.split_once('\t'))
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    };
    let num = |m: &BTreeMap<String, String>, k: &str| m.get(k).and_then(|v| v.parse::<u64>().ok());
    let capsule = kv(state.join("capsule.txt"));
    for key in ["memory.max", "memory.swap.max"] {
        if capsule.get(key) != unit.get(key) {
            bad.push(format!("P2-1: capsule {key}"));
        }
    }
    let censor = kv(state.join("censor.txt"));
    if censor.get("censor_s") != unit.get("censor_s") {
        bad.push("P2-1: censor value".into());
    }
    if censor.contains_key("fired") {
        bad.push("P2-1: the observation censor fired".into());
    }
    let record = kv(state.join("unit-main.tsv"));
    if record.get("tool_sha256") != unit.get("tool_sha256") {
        bad.push("P2-1: unit main executable".into());
    }
    let key = |r: &Row| match (number(r, "pid"), number(r, "start_ticks")) {
        (Ok(Some(p)), Ok(Some(s))) => Some((p, s)),
        _ => None,
    };
    // P2-6: harness + unit-roster terms merge once per identity ⇒ each identity's largest 1 s roster
    // reading (VmHWM or RSS, kB) joins its term: the unit main's, each server's + launcher's (the
    // harness's own term + the producers' stay the harness's).
    let roster = table(&state.join("unit-roster.tsv"));
    let mut read: BTreeMap<(u64, u64), u64> = BTreeMap::new();
    for r in roster.iter().flatten() {
        if let Some(k) = key(r) {
            let kb = ["hwm_kb", "rss_kb"]
                .iter()
                .filter_map(|c| number(r, c).ok().flatten())
                .max()
                .unwrap_or(0);
            let peak = read.entry(k).or_insert(0);
            *peak = (*peak).max(kb.saturating_mul(1024));
        }
    }
    let main_key = num(&record, "pid").zip(num(&record, "start_ticks"));
    let main = num(&record, "vmhwm_kb")
        .filter(|&kb| kb > 0)
        .map(|kb| kb.saturating_mul(1024));
    if main_key.is_none() || main.is_none() {
        bad.push("P2-6: unit main record".into());
    }
    let main = main
        .unwrap_or(0)
        .max(main_key.and_then(|k| read.get(&k).copied()).unwrap_or(0));
    let bound = a
        .report
        .iter()
        .find_map(|l| l.strip_prefix("load_bound\t"))
        .map(|l| {
            let f: Vec<u64> = l.split('\t').map_while(|v| v.parse().ok()).collect();
            (f.first().copied(), f.get(1).copied())
        });
    if !matches!(bound, Some((Some(l), Some(u))) if u >= l && u.saturating_add(main) <= BUDGET_BYTES)
    {
        bad.push("P2-6: load bound incomplete, below its sample or above 32 GB".into());
    }
    let mut known: BTreeSet<(u64, u64)> = BTreeSet::new();
    let mut tables = vec![run.join("load").join("processes.tsv")];
    let mut tail = 0u64;
    // The harness's directories = `load`, `arm-<arm>` per planned arm, `run-<n>` per planned call;
    // any other is unplanned, and an unplanned call's tables never count as harness samples.
    let calls: BTreeSet<String> = a
        .plan
        .iter()
        .filter_map(|r| number(r, "run").ok().flatten())
        .map(|n| format!("run-{n:06}"))
        .collect();
    for entry in fs::read_dir(run).into_iter().flatten().flatten() {
        let (path, name) = (
            entry.path(),
            entry.file_name().to_string_lossy().into_owned(),
        );
        if !path.is_dir() {
            continue;
        }
        if calls.contains(&name) {
            tables.push(path.join("processes.tsv"));
            let counters =
                fs::read_to_string(path.join("checker-counters.tsv")).unwrap_or_default();
            let get = |key: &str| {
                let prefix = format!("runtime\t{key}\t");
                counters
                    .lines()
                    .find_map(|l| l.strip_prefix(prefix.as_str())?.parse::<u64>().ok())
            };
            if get("launcher_report_failure").is_some_and(|v| v > 0) {
                bad.push(format!("P2-5: launcher-report failure in {name}"));
            }
            if let Some(key) = get("producer_pid").zip(get("producer_start_ticks")) {
                known.insert(key);
            }
        } else if name != "load"
            && name
                .strip_prefix("arm-")
                .is_none_or(|arm| !arms.contains(arm))
        {
            bad.push(format!("P2-10: unplanned directory {name}"));
        }
    }
    // Each identity the harness sampled in this attempt → the largest VmHWM it sampled for it.
    let mut sampled: BTreeMap<(u64, u64), u64> = BTreeMap::new();
    for path in &tables {
        for row in table(path).unwrap_or_default() {
            if let (Ok(Some(p)), Ok(Some(s))) = (number(&row, "pid"), number(&row, "start_ticks")) {
                let hwm = number(&row, "hwm_bytes").ok().flatten().unwrap_or(0);
                let peak = sampled.entry((p, s)).or_insert(0);
                *peak = (*peak).max(hwm);
            }
        }
    }
    known.extend(sampled.keys().copied());
    // A terminal check is the measured server's own verdict ⇒ each term is re-derived here: the raw
    // self-report (format, self scope, the check's identity), own bound = max(ru_maxrss, VmHWM) > 0,
    // an identity the harness sampled, each identity once. The term = the largest of that bound, every
    // VmHWM the harness sampled for it, every unit-roster reading of it + the launcher's startup VmHWM:
    // the kernel's recorded peak can trail an RSS it showed before (G1: up to 266,240 B), as wait4's
    // maxrss trails VmHWM (P2-5).
    let id = |m: &BTreeMap<String, String>| num(m, "pid").zip(num(m, "start_ticks"));
    let (mut terminal, mut fenced) = (BTreeSet::new(), Vec::new());
    for arm in arms {
        let dir = run.join(format!("arm-{arm}"));
        let start = kv(dir.join("producer-launcher.tsv"));
        let mut checks = Vec::new();
        for what in ["launcher", "server"] {
            let raw = kv(dir.join(format!("{what}-terminal.tsv")));
            let check = kv(dir.join(format!("{what}-terminal-check.tsv")));
            fenced.extend(id(&check).map(|k| (format!("arm-{arm} {what}"), k)));
            let own = num(&check, "own_bound_bytes");
            let terms = num(&raw, "ru_maxrss_bytes")
                .zip(num(&raw, "vmhwm_bytes"))
                .map(|(r, v)| r.max(v));
            let why = match (check.get("verdict").map(String::as_str), own, id(&check)) {
                (Some("bounded"), Some(own), Some(key)) => {
                    if raw.get("format") != Some(&format!("sb1-{what}-terminal 1"))
                        || raw.get("resource_scope").map(String::as_str) != Some("self")
                    {
                        Some("without its raw self-report")
                    } else if id(&raw) != Some(key) {
                        Some("identity off its raw record")
                    } else if terms != Some(own) || own == 0 {
                        Some("bound off max(ru_maxrss, VmHWM) of its raw record")
                    } else if !sampled.contains_key(&key) {
                        Some("identity never sampled by the harness")
                    } else if !terminal.insert(key) {
                        Some("identity repeated")
                    } else {
                        let startup =
                            num(&start, "vmhwm_bytes").filter(|_| id(&start) == Some(key));
                        let term = own
                            .max(sampled[&key])
                            .max(startup.unwrap_or(0))
                            .max(read.get(&key).copied().unwrap_or(0));
                        tail = tail.saturating_add(term);
                        None
                    }
                }
                _ => Some("unbounded"),
            };
            if let Some(why) = why {
                bad.push(format!("P2-6: arm-{arm} {what} terminal term {why}"));
            }
            checks.push(id(&check));
        }
        // P2-5: the launcher's startup record (published before the load) is mandatory evidence:
        // the launcher's terminal identity, its arm's server as parent, a startup VmHWM > 0 (a term
        // of the launcher's above); the comparisons a missing terminal check leaves open hold above.
        let (launcher, server) = (checks[0], checks[1]);
        let why = if start.get("format").map(String::as_str) != Some("sb1-producer-launcher 1") {
            Some("missing")
        } else if launcher.is_some_and(|k| id(&start) != Some(k)) {
            Some("off its terminal identity")
        } else if server.is_some_and(|(p, _)| num(&start, "parent_pid") != Some(p)) {
            Some("not its server's child")
        } else if num(&start, "vmhwm_bytes").is_none_or(|b| b == 0) {
            Some("peak missing")
        } else {
            None
        };
        if let Some(why) = why {
            bad.push(format!("P2-5: arm-{arm} launcher startup record {why}"));
        }
    }
    match num(&kv(run.join("memory.tsv")), "harness_terminal_hwm_bytes") {
        Some(b) => tail = tail.saturating_add(b),
        None => bad.push("P2-6: harness terminal term unrecorded".into()),
    }
    match (&roster, main_key) {
        (Ok(rows), Some(mk)) => {
            let at = |r: &Row| number(r, "epoch_us").ok().flatten();
            let lives: Vec<u64> = rows
                .iter()
                .filter(|r| key(r) == Some(mk))
                .filter_map(at)
                .collect();
            let span = lives.iter().min().zip(lives.iter().max());
            let mut unresolved = BTreeSet::new();
            for r in rows {
                match (key(r), at(r), span) {
                    (Some(k), _, _) if k == mk || known.contains(&k) => {}
                    (Some(_), Some(t), Some((first, last))) if t < *first || t > *last => {}
                    (Some(k), _, _) => {
                        unresolved.insert((k, cell(r, "comm").unwrap_or("?").to_owned()));
                    }
                    (None, _, _) => {
                        unresolved.insert(((0, 0), "unreadable row".to_owned()));
                    }
                }
            }
            for ((p, _), comm) in unresolved {
                bad.push(format!("P2-6: unit process {p} ({comm}) unresolved"));
            }
            // P2-1: each server + launcher (long-lived: the 1 s roster sees them) inside the
            // capsule's cgroup; its producers inherit it.
            let members: BTreeSet<(u64, u64)> = rows.iter().filter_map(key).collect();
            for (what, k) in fenced {
                if !members.contains(&k) {
                    bad.push(format!("P2-1: {what} outside the unit's cgroup"));
                }
            }
        }
        _ => bad.push("P2-6: unit roster or unit main record unreadable".into()),
    }
    (bad, main, tail)
}
fn chunk(
    c: &Chunk,
    base: &Path,
    frozen: &Path,
    m: &Manifest,
    sealed: &Closure,
    d: &mut Decision,
    whole: bool,
) -> Result<()> {
    let mut attempts = Vec::new();
    // TAS-P2 (P2-4 (2)): an attempt `assess` cannot read (the harness writes runs.tsv after every
    // load ⇒ a censor, kill or stop inside the load; a spawn or sampler error) keeps its partial
    // evidence = hard, its censor record named; the receipt mode stays rc 2.
    let mut unassessable = false;
    for id in [c.id.clone(), format!("{}-r", c.id)] {
        let (run, state) = (base.join("runs").join(&id), base.join("state").join(&id));
        if !run.is_dir() {
            continue;
        }
        match assess(&run, &state, frozen, m) {
            Ok(a) => attempts.push((id, a)),
            Err(e) if whole => {
                unassessable = true;
                d.out.push(format!("vector\t{}\t{id}\thard", c.id));
                d.holds
                    .push(format!("{}: attempt {id}: P2-4: unassessable ({e})", c.id));
                // censor.txt = `key\tvalue` lines, as p2 reads them.
                let censor = fs::read_to_string(state.join("censor.txt")).unwrap_or_default();
                if censor
                    .lines()
                    .any(|l| l.split_once('\t').is_some_and(|(k, _)| k == "fired"))
                {
                    d.holds.push(format!(
                        "{}: attempt {id}: P2-1: the observation censor fired",
                        c.id
                    ));
                }
            }
            Err(e) => return Err(e),
        }
    }
    if attempts.first().map(|(id, _)| id) != Some(&c.id) {
        if !unassessable {
            d.holds.push(format!("{}: no original attempt", c.id));
        }
        return Ok(());
    }
    let mut invalid = BTreeSet::new();
    let mut chosen: BTreeMap<u64, usize> = BTreeMap::new();
    // TAS-P2: each attempt's class + its terms outside the harness's bounds (unit main, teardown Σ).
    let (mut classes, mut extras): (Vec<&str>, Vec<(u64, u64)>) = (Vec::new(), Vec::new());
    for (k, (id, a)) in attempts.iter().enumerate() {
        let (run, state) = (base.join("runs").join(id), base.join("state").join(id));
        let original = (k > 0).then(|| attempts[0].1.plan.as_slice());
        let (held, mut bad) = conform(c, &run, &a.plan, original, sealed, whole)?;
        let (mut main, mut teardown) = (0, 0);
        if whole {
            let arms = c.arms().into_keys().collect();
            let (items, m, tail) = p2(&run, &state, a, &sealed.unit, &arms);
            bad.extend(items);
            (main, teardown) = (m, tail);
        }
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
            Some(p) if p.saturating_add(main) > BUDGET_BYTES => bad.push(format!(
                "lifecycle peak {:.2} GB above 32 GB",
                p as f64 / 1e9
            )),
            Some(_) => {}
        }
        if !whole && k > 0 && !held.is_subset(&invalid) {
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
                if !whole {
                    chosen.entry(b).or_insert(k);
                }
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
        if whole {
            // P2-4: integrity first; an `unassessed` or absent mandatory verdict holds (P2-3: one load
            // window per planned arm, none for another), and a policy item (the unit's CPUs + shield,
            // P2-1) is hard (`bad` above): P2-4 (3) lists no replaceable reason beyond a rejected X1–X4
            // verdict; only a sound attempt whose verdicts rejected it is environmental; the first
            // valid attempt is selected whole, and a replacement follows an environmental original
            // alone.
            let windows: Vec<(&str, &str)> = a
                .report
                .iter()
                .filter_map(|l| l.strip_prefix("load_window\t"))
                .filter_map(|l| {
                    let mut f = l.split('\t');
                    Some((f.next()?, f.nth(2)?))
                })
                .collect();
            let arms = c.arms();
            let mut count: BTreeMap<&str, usize> =
                arms.keys().map(|arm| (arm.as_str(), 0)).collect();
            for &(arm, _) in &windows {
                *count.entry(arm).or_default() += 1;
            }
            for (arm, n) in count {
                if !arms.contains_key(arm) {
                    bad.push(format!("P2-4: load window of unplanned arm-{arm}"));
                } else if n != 1 {
                    bad.push(format!("P2-4: arm-{arm} load window absent or repeated"));
                }
            }
            let verdicts: Vec<&str> = held
                .iter()
                .map(|b| a.verdicts.get(b).map_or("absent", String::as_str))
                .chain(windows.iter().map(|w| w.1))
                .collect();
            if verdicts
                .iter()
                .any(|v| *v != "accepted" && *v != "rejected")
            {
                bad.push("P2-4: a mandatory host verdict unassessed or absent".into());
            }
            // Past these holds each verdict reads accepted | rejected: environmental = one rejected.
            let class = if !bad.is_empty() {
                "hard"
            } else if verdicts.iter().all(|v| *v == "accepted") {
                "valid"
            } else {
                "environmental"
            };
            d.out.push(format!("vector\t{}\t{id}\t{class}", c.id));
            if k > 0 && classes.first() != Some(&"environmental") {
                d.holds.push(format!(
                    "{}: attempt {id}: a replacement after a {} original",
                    c.id,
                    classes.first().copied().unwrap_or("missing")
                ));
            } else if class == "valid" && chosen.is_empty() {
                chosen.extend(held.iter().map(|&b| (b, k)));
            }
            if k > 0 && class == "environmental" {
                d.holds
                    .push(format!("{}: attempt {id}: environmental again", c.id));
            }
            classes.push(class);
        }
        extras.push((main, teardown));
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
        let main = extras[k].0;
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
            if !(x.budget
                && x.bound
                    .is_some_and(|v| v.saturating_add(main) <= BUDGET_BYTES))
            {
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
    // P2-6: U_life = max(each load U, each call U, the teardown Σ of terminal own HWMs) + the unit
    // main — VmHWM is a lifetime peak, so the terminal Σ covers idle + teardown.
    if whole && let Some(&k) = chosen.values().next() {
        let ((id, a), (main, teardown)) = (&attempts[k], extras[k]);
        let load = a
            .report
            .iter()
            .filter_map(|l| l.strip_prefix("load_bound\t"))
            .filter_map(|l| l.split('\t').nth(1)?.parse::<u64>().ok())
            .max()
            .unwrap_or(0);
        let call = a.calls.iter().filter_map(|x| x.bound).max().unwrap_or(0);
        let life = load.max(call).max(teardown).saturating_add(main);
        d.out
            .push(format!("lifecycle_bound\t{}\t{id}\t{life}", c.id));
        if life > BUDGET_BYTES {
            d.holds.push(format!(
                "{}: lifecycle bound {:.2} GB above 32 GB",
                c.id,
                life as f64 / 1e9
            ));
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
