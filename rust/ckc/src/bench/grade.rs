// Contract sb D6 + § Carried rulings (affinity + interference exclusion, operationalized there): grades
// one campaign chunk = a `ckc bench run` directory + the state directory `tools/sb-campaign-unit`
// writes (rc, host records, 1 s host samples, the unit's CPU list) against the frozen instances + the
// run manifest. Report (TSV on stdout): input identity per arm, each call's output digest against the
// manifest's union (×100) or recipe oracle row, the 300 s + 32 GB budget per call, X1–X4 per block,
// R over every block + over the accepted ones. rc 1 = an identity mismatch, a wrong output or a call
// whose (instance, query) the manifest gives no expected output (`sha_unchecked`: its correctness is
// unestablished) — the chunk is no evidence; rc 2 = usage or an unreadable input.
use super::generate::identity_frame;
use super::stats::{ratio, stats};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::Path;
use std::process::ExitCode;

type Result<T> = std::result::Result<T, String>;
type Row = BTreeMap<String, String>;

const HELP: &str = "usage: ckc bench grade <run-dir> <state-dir> <frozen-root> <run-manifest>";
const BUDGET_NS: u64 = 300_000_000_000;
const BUDGET_BYTES: u64 = 32_000_000_000;
const FOREIGN_CPUS: f64 = 0.5;
const GAP_US: i64 = 5_000_000;
const PINNED: &str = "2-3,6-7";
// X3 fields beside the kernel (the record's second line): exact keys, since lscpu's
// `CPU(s) scaling MHz:` varies run to run.
const CONFIG_KEYS: [&str; 5] = [
    "Model name:",
    "CPU(s):",
    "governors:",
    "online:",
    "slice-cpus:",
];

pub fn run(args: &[String]) -> ExitCode {
    match grade(args) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => {
            eprintln!("ckc: bench: grade: {error}");
            ExitCode::from(2)
        }
    }
}

fn text(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}
fn table(path: &Path) -> Result<Vec<Row>> {
    let text = text(path)?;
    let mut lines = text.lines();
    let head: Vec<&str> = lines
        .next()
        .ok_or_else(|| format!("{}: empty", path.display()))?
        .split('\t')
        .collect();
    Ok(lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            head.iter()
                .map(|h| (*h).to_owned())
                .zip(l.split('\t').map(str::to_owned))
                .collect()
        })
        .collect())
}
fn cell<'a>(row: &'a Row, key: &str) -> Result<&'a str> {
    row.get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing column {key}"))
}
// An empty cell = not measured (an indeterminate call has no score).
fn number(row: &Row, key: &str) -> Result<Option<u64>> {
    let value = cell(row, key)?;
    if value.is_empty() {
        return Ok(None);
    }
    value
        .parse()
        .map(Some)
        .map_err(|_| format!("{key}: not a number: {value}"))
}

#[derive(Default)]
struct Manifest {
    composition: HashMap<String, String>,
    // instance → (path, sha256, bytes) per frozen query file
    queries: HashMap<String, Vec<(String, String, String)>>,
    // (instance, query) → expected output sha256; a union row wins over a recipe oracle row
    outputs: HashMap<(String, String), String>,
}
fn manifest(path: &Path) -> Result<Manifest> {
    let mut m = Manifest::default();
    let mut oracle = Vec::new();
    for line in text(path)?.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 6 {
            continue;
        }
        match (f[0], f[2]) {
            ("instance", "composition") => {
                m.composition.insert(f[1].into(), f[4].into());
            }
            ("query", _) => m.queries.entry(f[1].into()).or_default().push((
                f[3].into(),
                f[4].into(),
                f[5].into(),
            )),
            ("union", "q-all" | "q-one" | "q-patient") => {
                m.outputs.insert((f[1].into(), f[2].into()), f[4].into());
            }
            ("oracle", name) => {
                if let Some(query) = name.strip_prefix("recipe:").filter(|q| q.starts_with("q-")) {
                    oracle.push(((f[1].to_owned(), query.to_owned()), f[4].to_owned()));
                }
            }
            _ => {}
        }
    }
    for (key, sha) in oracle {
        m.outputs.entry(key).or_insert(sha);
    }
    Ok(m)
}

// arm → recorded path → (bytes, sha256): the run's own `identities.tsv`, hashed before its first call.
type Identities = HashMap<String, HashMap<String, (String, String)>>;
fn identities(run: &Path) -> Result<Identities> {
    let mut arms = Identities::new();
    for line in text(&run.join("identities.tsv"))?.lines().skip(1) {
        let f: Vec<&str> = line.split('\t').collect();
        if let [arm, path, bytes, sha] = f[..] {
            arms.entry(arm.into())
                .or_default()
                .insert(path.into(), (bytes.into(), sha.into()));
        }
    }
    Ok(arms)
}
// The composition identity recomputed from the arm's recorded sizes + digests (composition lines are
// root-relative; the run root = the schema row's prefix ⇒ one lookup per line) + each frozen query's
// digest. Pass = the manifest's identity, ≥ 1 query checked, none differing.
fn identity(
    rows: &HashMap<String, (String, String)>,
    instance: &Path,
    iid: &str,
    m: &Manifest,
) -> Result<(bool, String)> {
    let roots: Vec<&str> = rows
        .keys()
        .filter_map(|p| p.strip_suffix("/guidelines/schema.pl"))
        .collect();
    let [root] = roots[..] else {
        return Ok((false, format!("schema rows {}", roots.len())));
    };
    let size = |v: &(String, String)| v.0.parse::<u64>().map_err(|_| format!("size {}", v.0));
    let schema = &rows[&format!("{root}/guidelines/schema.pl")];
    let composition = text(&instance.join("composition.tsv"))?;
    let mut files = Vec::new();
    for line in composition.lines() {
        let Some(row) = rows.get(&format!("{root}/{line}")) else {
            return Ok((false, format!("no identity row for {line}")));
        };
        files.push((line, size(row)?, row.1.as_str()));
    }
    let got = identity_frame((size(schema)?, &schema.1), files);
    let mut bad = usize::from(m.composition.get(iid) != Some(&got));
    let mut checked = 0;
    for (path, sha, bytes) in m.queries.get(iid).into_iter().flatten() {
        let suffix = format!("/{path}");
        let hits: Vec<_> = rows
            .iter()
            .filter(|(p, _)| p.ends_with(&suffix))
            .map(|(_, v)| v)
            .collect();
        if !hits.is_empty() {
            checked += 1;
            bad += usize::from(hits.len() != 1 || hits[0].0 != *bytes || hits[0].1 != *sha);
        }
    }
    let verdict = if bad == 0 { "MATCH" } else { "MISMATCH" };
    Ok((
        bad == 0 && checked > 0,
        format!("{verdict}\tqueries-checked={checked}\tbad={bad}"),
    ))
}

struct Sample {
    t: i64,
    busy: i64,
    cpu: i64,
    swap: Option<i64>,
    core: i64,
    package: i64,
    temp: i64,
}
// None = no sampler ran (p01's protocol); rows without a CPU reading (the cgroup gone) are skipped.
fn samples(path: &Path) -> Result<Option<Vec<Sample>>> {
    if !path.is_file() {
        return Ok(None);
    }
    let mut out = Vec::new();
    for row in table(path)? {
        let int = |k: &str| row.get(k).and_then(|v| v.parse::<i64>().ok());
        if let (Some(t), Some(busy), Some(cpu), Some(core), Some(package), Some(temp)) = (
            int("epoch_us"),
            int("busy_jiffies"),
            int("campaign_cpu_us"),
            int("core_throttle"),
            int("package_throttle"),
            int("max_temp_mc"),
        ) {
            let swap = int("campaign_swap_bytes");
            out.push(Sample {
                t,
                busy,
                cpu,
                swap,
                core,
                package,
                temp,
            });
        }
    }
    Ok(Some(out))
}
// X1 window = the last sample ≤ s → the first ≥ e (cumulative counters ⇒ exact over it); X4 = every
// spacing that meets [s, e] + the open edges measured from the samples; an unbracketed block = X4;
// X2 unreadable on every sample of the window = unassessed.
fn interference(samples: Option<&[Sample]>, s: i64, e: i64, hz: f64) -> [String; 7] {
    let verdict = |v: &str, why: &str| [v, why, "NA", "NA", "NA", "NA", "NA"].map(str::to_owned);
    let Some(rows) = samples else {
        return verdict("unassessed", "no-sampler");
    };
    let (Some(first), Some(last)) = (
        rows.iter().min_by_key(|r| r.t),
        rows.iter().max_by_key(|r| r.t),
    ) else {
        return verdict("rejected", "X4");
    };
    let mut ts: Vec<i64> = rows.iter().map(|r| r.t).collect();
    ts.sort_unstable();
    let mut gap = ts
        .windows(2)
        .filter(|w| w[1] > s && w[0] < e)
        .map(|w| w[1] - w[0])
        .max()
        .unwrap_or(0);
    if first.t > s {
        gap = gap.max(first.t - s);
    }
    if last.t < e {
        gap = gap.max(e - last.t);
    }
    let lo = rows
        .iter()
        .filter(|r| r.t <= s)
        .max_by_key(|r| r.t)
        .unwrap_or(first);
    let hi = rows
        .iter()
        .filter(|r| r.t >= e)
        .min_by_key(|r| r.t)
        .unwrap_or(last);
    if hi.t <= lo.t {
        gap = gap.max(GAP_US + 1);
    }
    let span = (hi.t - lo.t).max(1) as f64 / 1e6;
    let foreign = ((hi.busy - lo.busy) as f64 / hz - (hi.cpu - lo.cpu) as f64 / 1e6) / span;
    let inside: Vec<&Sample> = rows.iter().filter(|r| lo.t <= r.t && r.t <= hi.t).collect();
    let swap: Vec<i64> = inside.iter().filter_map(|r| r.swap).collect();
    let mut why: Vec<&str> = [
        ("X1", foreign > FOREIGN_CPUS),
        ("X2", swap.iter().any(|&b| b > 0)),
        ("X4", gap > GAP_US),
    ]
    .into_iter()
    .filter(|(_, bad)| *bad)
    .map(|(w, _)| w)
    .collect();
    let state = if !why.is_empty() {
        "rejected"
    } else if swap.is_empty() {
        why.push("X2-unreadable");
        "unassessed"
    } else {
        "accepted"
    };
    let temp = inside.iter().map(|r| r.temp).max().unwrap_or(0);
    [
        state.to_owned(),
        if why.is_empty() {
            "-".to_owned()
        } else {
            why.join("+")
        },
        format!("{foreign:.3}"),
        format!("{:.2}", gap as f64 / 1e6),
        (hi.core - lo.core).to_string(),
        (hi.package - lo.package).to_string(),
        format!("{:.1}", temp as f64 / 1000.0),
    ]
}
// X3 fields (kernel + CONFIG_KEYS, whitespace-normalized); None = a record without every field
// (p01's pre-sampler format) or no record.
fn host_config(path: &Path) -> Result<Option<Vec<String>>> {
    if !path.is_file() {
        return Ok(None);
    }
    let text = text(path)?;
    let lines: Vec<&str> = text.lines().collect();
    let Some(kernel) = lines.get(1) else {
        return Ok(None);
    };
    let mut fields = vec![kernel.trim().to_owned()];
    for key in CONFIG_KEYS {
        let Some(line) = lines.iter().find(|l| l.starts_with(key)) else {
            return Ok(None);
        };
        fields.push(
            line[key.len()..]
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
        );
    }
    Ok(Some(fields))
}
// The host record's `key: value` line, if present.
fn host_value(path: &Path, key: &str) -> Result<Option<String>> {
    if !path.is_file() {
        return Ok(None);
    }
    Ok(text(path)?
        .lines()
        .find_map(|l| l.strip_prefix(key))
        .map(|v| v.trim().to_owned()))
}
fn closed_us(run: &Path, n: u64) -> Result<Option<i64>> {
    let path = run.join(format!("run-{n:06}/sink-complete.tsv"));
    if !path.is_file() {
        return Ok(None);
    }
    let text = text(&path)?;
    let value = text
        .lines()
        .find_map(|l| l.strip_prefix("closed_unix_ns\t"))
        .ok_or_else(|| format!("{}: no closed_unix_ns", path.display()))?;
    let ns: i64 = value
        .parse()
        .map_err(|_| format!("{}: closed_unix_ns", path.display()))?;
    Ok(Some(ns / 1000))
}

fn grade(args: &[String]) -> Result<bool> {
    let [run, state, frozen, manifest_path] = args else {
        return Err(HELP.into());
    };
    let (run, state, frozen) = (Path::new(run), Path::new(state), Path::new(frozen));
    let m = manifest(Path::new(manifest_path))?;
    let mut out = Vec::new();
    let rc = match fs::read_to_string(state.join("rc")) {
        Ok(rc) => rc.trim().to_owned(),
        Err(_) => "missing".into(),
    };
    out.push(format!("chunk_rc\t{rc}"));

    let plan = table(&run.join("plan.tsv"))?;
    let mut arms: BTreeMap<String, String> = BTreeMap::new();
    let mut queries: HashMap<u64, String> = HashMap::new();
    for row in &plan {
        let composition = Path::new(cell(row, "composition")?);
        let iid = composition
            .parent()
            .and_then(Path::file_name)
            .and_then(|n| n.to_str())
            .ok_or("plan composition path")?;
        arms.insert(cell(row, "arm")?.into(), iid.into());
        let query = Path::new(cell(row, "query")?)
            .file_stem()
            .and_then(|n| n.to_str())
            .ok_or("plan query path")?;
        let n = number(row, "run")?.ok_or("plan run")?;
        queries.insert(n, query.into());
    }
    let ids = identities(run)?;
    let mut matched = true;
    let empty = HashMap::new();
    for (arm, iid) in &arms {
        let (pass, detail) = identity(ids.get(arm).unwrap_or(&empty), &frozen.join(iid), iid, &m)?;
        matched &= pass;
        out.push(format!("identity\t{arm}\t{iid}\t{detail}"));
        let load = run.join(format!("arm-{arm}/load.tsv"));
        if load.is_file() {
            let rows: HashMap<String, String> = text(&load)?
                .lines()
                .filter_map(|l| l.split_once('\t'))
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect();
            let get = |k: &str| rows.get(k).map_or("NA", String::as_str);
            out.push(format!(
                "kb\t{arm}\t{}\treceipt={}",
                get("kb_identity"),
                get("receipt")
            ));
        }
    }

    let before = state.join("host-before.txt");
    let x3 = match (
        host_config(&before)?,
        host_config(&state.join("host-after.txt"))?,
    ) {
        (Some(a), Some(b)) if a == b => "same",
        (Some(_), Some(_)) => "DIFFERS",
        _ => "missing",
    };
    out.push(format!("x3_host_config\t{x3}"));
    let cpus = host_value(&state.join("unit-cpus.txt"), "Cpus_allowed_list:")?;
    let pinned = if cpus.as_deref() == Some(PINNED) {
        "ok"
    } else {
        "NOT-PINNED"
    };
    out.push(format!(
        "unit_cpus\t{}\t{pinned}",
        cpus.as_deref().unwrap_or("missing")
    ));
    let hz = match host_value(&before, "clk_tck:")? {
        Some(v) => v.parse::<f64>().map_err(|_| format!("clk_tck: {v}"))?,
        None => 100.0, // USER_HZ on Linux; the pre-clk_tck records come from the pool host (100)
    };

    let runs = table(&run.join("runs.tsv"))?;
    out.push(
        "#call\trun\tblock\tarm\tquery\toutcome\tscored_s\tpeak_rss_gb\thwm_sum_gb\tsha_ok\tbudget"
            .into(),
    );
    let (mut wrong, mut unchecked, mut over) = (0, 0, 0);
    let mut per: BTreeMap<(String, String), Vec<(f64, u64)>> = BTreeMap::new();
    let mut blocks: BTreeMap<u64, (i64, i64)> = BTreeMap::new();
    let mut unsunk: BTreeMap<u64, usize> = BTreeMap::new();
    let mut scored: BTreeMap<(u64, String, String), u64> = BTreeMap::new();
    for row in &runs {
        let n = number(row, "run")?.ok_or("runs.tsv run")?;
        let block = number(row, "block")?.ok_or("runs.tsv block")?;
        let arm = cell(row, "arm")?;
        let query = queries.get(&n).ok_or("runs.tsv run outside plan.tsv")?;
        let outcome = cell(row, "outcome")?;
        let score = number(row, "scored_ns")?;
        let peak = number(row, "peak_rss_bytes")?.unwrap_or(0);
        let hwm = number(row, "observed_hwm_sum_bytes")?.unwrap_or(0);
        let iid = arms.get(arm).ok_or("runs.tsv arm outside plan.tsv")?;
        let sha = match m.outputs.get(&(iid.clone(), query.clone())) {
            None => {
                unchecked += 1;
                "none"
            }
            Some(want) if want == cell(row, "output_sha256")? => "yes",
            Some(_) => {
                wrong += 1;
                "NO"
            }
        };
        let memory = if peak > BUDGET_BYTES {
            "over"
        } else if hwm > BUDGET_BYTES {
            "uncertain"
        } else {
            "ok"
        };
        let budget =
            if outcome == "verified" && score.is_some_and(|s| s <= BUDGET_NS) && memory == "ok" {
                "ok".to_owned()
            } else {
                over += 1;
                format!("fail:{memory}")
            };
        out.push(format!(
            "call\t{n}\t{block}\t{arm}\t{query}\t{outcome}\t{}\t{:.2}\t{:.2}\t{sha}\t{budget}",
            score.map_or("NA".into(), |s| format!("{:.3}", s as f64 / 1e9)),
            peak as f64 / 1e9,
            hwm as f64 / 1e9
        ));
        if let Some(s) = score {
            per.entry((arm.into(), query.clone()))
                .or_default()
                .push((s as f64 / 1e9, peak));
            if outcome == "verified" {
                scored.insert((block, arm.into(), query.clone()), s);
            }
        }
        match (score, closed_us(run, n)?) {
            (Some(s), Some(end)) => {
                let start = end - (s / 1000) as i64;
                let b = blocks.entry(block).or_insert((start, end));
                *b = (b.0.min(start), b.1.max(end));
            }
            _ => *unsunk.entry(block).or_default() += 1,
        }
    }

    out.push("#stat\tarm\tquery\tn\tmedian_s\tiqr_s\tmax_s\tmax_peak_rss_gb".into());
    for ((arm, query), v) in &per {
        let (median, iqr, max) = stats(v.iter().map(|x| x.0).collect());
        let peak = v.iter().map(|x| x.1).max().unwrap_or(0);
        out.push(format!(
            "stat\t{arm}\t{query}\t{}\t{median:.3}\t{iqr:.3}\t{max:.3}\t{:.2}",
            v.len(),
            peak as f64 / 1e9
        ));
    }
    if run.join("loads.tsv").is_file() {
        for l in table(&run.join("loads.tsv"))? {
            out.push(format!(
                "load\t{}\t{:.1}\trss_at_ready_gb={:.2}\t{}",
                cell(&l, "arm")?,
                number(&l, "load_ns")?.unwrap_or(0) as f64 / 1e9,
                number(&l, "rss_at_ready_bytes")?.unwrap_or(0) as f64 / 1e9,
                cell(&l, "outcome")?
            ));
        }
    }

    let host = samples(&state.join("host-samples.tsv"))?;
    out.push("#block\tn\tstart_us\tend_us\tverdict\twhy\tforeign_cpus\tmax_gap_s\tthrottle_core\tthrottle_package\tmax_temp_c\tcalls_without_sink".into());
    let all: BTreeSet<u64> = runs
        .iter()
        .filter_map(|r| number(r, "block").ok().flatten())
        .collect();
    let (mut accepted, mut rejected, mut unassessed) = (BTreeSet::new(), 0, 0);
    for &n in &all {
        let missing = unsunk.get(&n).copied().unwrap_or(0);
        let mut v = match blocks.get(&n) {
            Some(&(s, e)) => interference(host.as_deref(), s, e, hz),
            None => ["unassessed", "no-interval", "NA", "NA", "NA", "NA", "NA"].map(str::to_owned),
        };
        // X3 is the chunk's: a changed host rejects every block; an incomplete record leaves an
        // otherwise accepted block unassessed.
        if x3 == "DIFFERS" {
            v[1] = if v[1] == "-" {
                "X3".into()
            } else {
                format!("{}+X3", v[1])
            };
            v[0] = "rejected".into();
        } else if x3 == "missing" && v[0] == "accepted" {
            v[0] = "unassessed".into();
            v[1] = "X3-missing".into();
        }
        match v[0].as_str() {
            "accepted" => {
                accepted.insert(n);
            }
            "rejected" => rejected += 1,
            _ => unassessed += 1,
        }
        let (s, e) = blocks.get(&n).map_or(("NA".into(), "NA".into()), |b| {
            (b.0.to_string(), b.1.to_string())
        });
        out.push(format!("block\t{n}\t{s}\t{e}\t{}\t{missing}", v.join("\t")));
    }

    if arms.contains_key("baseline") {
        let ratio_path = run.join("ratio.tsv");
        let harness = if ratio_path.is_file() {
            table(&ratio_path)?
        } else {
            Vec::new()
        };
        for h in &harness {
            out.push(format!(
                "ratio_harness\t{}\tpairs={}\t{}\tR={}\tUB95={}",
                cell(h, "query")?,
                cell(h, "pairs")?,
                cell(h, "status")?,
                cell(h, "ratio_of_medians")?,
                cell(h, "one_sided_95_upper")?
            ));
        }
        // The harness names the query by its file (`q-one.pl`), the plan by its path: match stems.
        let seed = harness
            .iter()
            .find(|h| {
                h.get("query")
                    .and_then(|q| Path::new(q).file_stem())
                    .and_then(|s| s.to_str())
                    == Some("q-one")
            })
            .map(|h| number(h, "seed"))
            .transpose()?
            .flatten();
        for (label, set) in [("all", &all), ("accepted", &accepted)] {
            let pairs: Vec<(f64, f64)> = set
                .iter()
                .filter_map(|&b| {
                    let get = |arm: &str| scored.get(&(b, arm.to_owned(), "q-one".to_owned()));
                    let (base, target) = (get("baseline")?, get("target")?);
                    (*base > 0).then_some((*base as f64, *target as f64))
                })
                .collect();
            match seed {
                Some(seed) if !pairs.is_empty() => {
                    let (r, upper) = ratio(&pairs, seed);
                    out.push(format!(
                        "ratio_recomputed\t{label}\tpairs={}\tR={r:.9}\tUB95={upper:.9}",
                        pairs.len()
                    ));
                }
                _ => out.push(format!(
                    "ratio_recomputed\t{label}\tpairs={}\tnot-estimable",
                    pairs.len()
                )),
            }
        }
    }
    out.push(format!(
        "summary\tidentity={}\tsha_bad={wrong}\tsha_unchecked={unchecked}\tbudget_fail={over}\tblocks={}\taccepted={}\trejected={rejected}\tunassessed={unassessed}\tx3={x3}\tpinned={pinned}",
        if matched { "ok" } else { "bad" },
        all.len(),
        accepted.len()
    ));
    println!("{}", out.join("\n"));
    Ok(matched && wrong == 0 && unchecked == 0)
}
