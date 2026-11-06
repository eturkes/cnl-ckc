// Contract sb D6 + § Carried rulings (affinity + interference exclusion, operationalized there): grades
// one campaign chunk = a `ckc bench run` directory + the state directory `tools/sb-campaign-unit`
// writes (rc, host records, 1 s host samples, the unit's CPU list) against the frozen instances + the
// run manifest. Report (TSV on stdout): input identity per arm, each call's output digest against the
// manifest's union (×100) or recipe oracle row, the 300 s + 32 GB budget per call, X1–X4 per block
// over its attempted calls' interval (a call never timed = an `incomplete` block), R over every block
// + over the accepted ones. rc 1 = an identity mismatch, a wrong output, a call whose (instance,
// query) the manifest gives no expected output (`sha_unchecked`: its correctness is unestablished) or
// a runs.tsv that does not hold each planned call once, as planned — the chunk is no evidence; rc 2 =
// usage or an unreadable input. A call that ended short of verified printed no output: its digest
// reads `NA` (a budget fail, never a wrong output). `preflight` grades the host-only preflight of the
// same contract section: its synthetic controls read as planted, every 150 s quiet window passes X1 +
// X4, X3 + the policy hold (rc 0 = go, 1 = hold). `assess` = one chunk's grade as data, shared with
// `ckc bench decide`.
use super::generate::identity_frame;
use super::stats::{ratio, stats};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::Path;
use std::process::ExitCode;

pub(super) type Result<T> = std::result::Result<T, String>;
pub(super) type Row = BTreeMap<String, String>;

const HELP: &str = "usage: ckc bench grade <run-dir> <state-dir> <frozen-root> <run-manifest>";
const PREFLIGHT_HELP: &str = "usage: ckc bench preflight <state-dir>";
const BUDGET_NS: u64 = 300_000_000_000;
pub(super) const BUDGET_BYTES: u64 = 32_000_000_000;
const FOREIGN_CPUS: f64 = 0.5;
const GAP_US: i64 = 5_000_000;
const PINNED: &str = "2-3,6-7";
const HOST: &str = "0-1,4-5";
// A preflight control = a 60 s burner; its marks also frame the launch of its unit.
const CONTROL_US: std::ops::RangeInclusive<i64> = 60_000_000..=65_000_000;
const QUIET_US: i64 = 150_000_000;
const QUIET_WINDOWS: i64 = 4;
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
    exit("grade", grade(args))
}
pub fn preflight(args: &[String]) -> ExitCode {
    exit("preflight", preflight_check(args))
}
pub(super) fn exit(command: &str, verdict: Result<bool>) -> ExitCode {
    match verdict {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => {
            eprintln!("ckc: bench: {command}: {error}");
            ExitCode::from(2)
        }
    }
}

pub(super) fn text(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}
pub(super) fn table(path: &Path) -> Result<Vec<Row>> {
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
pub(super) fn cell<'a>(row: &'a Row, key: &str) -> Result<&'a str> {
    row.get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing column {key}"))
}
// An empty cell = not measured (an indeterminate call has no score).
pub(super) fn number(row: &Row, key: &str) -> Result<Option<u64>> {
    cell(row, key)?;
    optional(row, key)
}
// A column older runs lack reads as not measured.
fn optional(row: &Row, key: &str) -> Result<Option<u64>> {
    match row.get(key).map(String::as_str) {
        None | Some("") => Ok(None),
        Some(value) => value
            .parse()
            .map(Some)
            .map_err(|_| format!("{key}: not a number: {value}")),
    }
}

#[derive(Default)]
pub(super) struct Manifest {
    composition: HashMap<String, String>,
    // instance → (path, sha256, bytes) per frozen query file
    queries: HashMap<String, Vec<(String, String, String)>>,
    // (instance, query) → expected output sha256; a union row wins over a recipe oracle row
    outputs: HashMap<(String, String), String>,
}
pub(super) fn manifest(path: &Path) -> Result<Manifest> {
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
// digest: every query the arm's plan calls (its file stem) needs exactly one identity row with the
// manifest's bytes + digest (scientist-8 S8: an unrecorded planned query proves nothing), and any
// other recorded frozen query must match too. Pass = the manifest's identity, ≥ 1 query checked,
// none differing or missing.
fn identity(
    rows: &HashMap<String, (String, String)>,
    instance: &Path,
    iid: &str,
    m: &Manifest,
    planned: &BTreeSet<String>,
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
    let (mut checked, mut found) = (0, BTreeSet::new());
    for (path, sha, bytes) in m.queries.get(iid).into_iter().flatten() {
        let suffix = format!("/{path}");
        let hits: Vec<_> = rows
            .iter()
            .filter(|(p, _)| p.ends_with(&suffix))
            .map(|(_, v)| v)
            .collect();
        let stem = Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let wanted = planned.contains(stem);
        if wanted {
            found.insert(stem);
        }
        if wanted || !hits.is_empty() {
            checked += 1;
            bad += usize::from(hits.len() != 1 || hits[0].0 != *bytes || hits[0].1 != *sha);
        }
    }
    // A planned query the manifest freezes no file for.
    bad += planned.len() - found.len();
    let verdict = if bad == 0 { "MATCH" } else { "MISMATCH" };
    Ok((
        bad == 0 && checked > 0,
        format!("{verdict}\tqueries-checked={checked}\tbad={bad}"),
    ))
}

struct Sample {
    t: i64,
    busy: i64,
    // Busy jiffies of the campaign's CPUs 2 3 6 7 alone; None = a sampler that predates the column.
    pinned: Option<i64>,
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
            out.push(Sample {
                t,
                busy,
                pinned: int("pinned_busy_jiffies"),
                cpu,
                swap: int("campaign_swap_bytes"),
                core,
                package,
                temp,
            });
        }
    }
    Ok(Some(out))
}
// The samples bracketing [s, e]: the last ≤ s (none: the first) → the first ≥ e (none: the last);
// cumulative counters ⇒ exact over it. `gap` = the largest spacing meeting [s, e] + the open edges
// measured from the samples; a bracket without two distinct sample times reads as a gap past X4.
struct Bracket<'a> {
    lo: &'a Sample,
    hi: &'a Sample,
    inside: Vec<&'a Sample>,
    gap: i64,
}
fn bracket(rows: &[Sample], s: i64, e: i64) -> Option<Bracket<'_>> {
    let first = rows.iter().min_by_key(|r| r.t)?;
    let last = rows.iter().max_by_key(|r| r.t)?;
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
    let inside = rows.iter().filter(|r| lo.t <= r.t && r.t <= hi.t).collect();
    Some(Bracket {
        lo,
        hi,
        inside,
        gap,
    })
}
impl Bracket<'_> {
    fn span(&self) -> f64 {
        (self.hi.t - self.lo.t).max(1) as f64 / 1e6
    }
    // CPUs the campaign's cgroup used on average.
    fn campaign(&self) -> f64 {
        (self.hi.cpu - self.lo.cpu) as f64 / 1e6 / self.span()
    }
    // Foreign CPUs = busy CPUs − the campaign's, host-wide or over the campaign's CPUs (X1).
    fn host(&self, hz: f64) -> f64 {
        (self.hi.busy - self.lo.busy) as f64 / hz / self.span() - self.campaign()
    }
    fn pinned(&self, hz: f64) -> Option<f64> {
        let busy = self.hi.pinned? - self.lo.pinned?;
        Some(busy as f64 / hz / self.span() - self.campaign())
    }
}
// X1 = foreign CPUs on the campaign's CPUs 2 3 6 7 (host-wide foreign = a reported diagnostic: work
// the shield keeps on CPUs 0 1 4 5 never contends for the campaign's cores); X4 = the bracket's gap;
// X2 unreadable on every sample of the window, or a sampler without the pinned column = unassessed.
fn interference(samples: Option<&[Sample]>, s: i64, e: i64, hz: f64) -> [String; 8] {
    let verdict =
        |v: &str, why: &str| [v, why, "NA", "NA", "NA", "NA", "NA", "NA"].map(str::to_owned);
    let Some(rows) = samples else {
        return verdict("unassessed", "no-sampler");
    };
    let Some(b) = bracket(rows, s, e) else {
        return verdict("rejected", "X4");
    };
    let pinned = b.pinned(hz);
    let swap: Vec<i64> = b.inside.iter().filter_map(|r| r.swap).collect();
    let mut why: Vec<&str> = [
        ("X1", pinned.is_some_and(|f| f > FOREIGN_CPUS)),
        ("X2", swap.iter().any(|&b| b > 0)),
        ("X4", b.gap > GAP_US),
    ]
    .into_iter()
    .filter(|(_, bad)| *bad)
    .map(|(w, _)| w)
    .collect();
    let state = if !why.is_empty() {
        "rejected"
    } else {
        if pinned.is_none() {
            why.push("X1-unpinned");
        }
        if swap.is_empty() {
            why.push("X2-unreadable");
        }
        if why.is_empty() {
            "accepted"
        } else {
            "unassessed"
        }
    };
    let temp = b.inside.iter().map(|r| r.temp).max().unwrap_or(0);
    [
        state.to_owned(),
        if why.is_empty() {
            "-".to_owned()
        } else {
            why.join("+")
        },
        pinned.map_or("NA".into(), |f| format!("{f:.3}")),
        format!("{:.2}", b.gap as f64 / 1e6),
        (b.hi.core - b.lo.core).to_string(),
        (b.hi.package - b.lo.package).to_string(),
        format!("{:.1}", temp as f64 / 1000.0),
        format!("{:.3}", b.host(hz)),
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
// X3 over a state's two host records: same | DIFFERS | missing.
fn x3_config(state: &Path) -> Result<&'static str> {
    Ok(
        match (
            host_config(&state.join("host-before.txt"))?,
            host_config(&state.join("host-after.txt"))?,
        ) {
            (Some(a), Some(b)) if a == b => "same",
            (Some(_), Some(_)) => "DIFFERS",
            _ => "missing",
        },
    )
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
// The applied protocol (contract sb § Carried rulings Affinity + Shield), from the state
// `tools/sb-campaign-unit` writes: the unit on CPUs 2 3 6 7 inside sb1.slice's 2 3 6 7, the wrapper +
// its sampler on 0 1 4 5, every shield target on 0 1 4 5 as the unit began (`shielded`) + as it
// ended (`final`, its ExecStopPost's record before the restore), the restore run, the sleeper's
// marker seen (`sleeper: none` = the second slot unheld). Each failed item, by name.
pub(super) fn policy(state: &Path) -> Result<Vec<String>> {
    let mut bad = Vec::new();
    let mut expect = |file: &str, key: &str, want: &str, item: &str| -> Result<()> {
        if host_value(&state.join(file), key)?.as_deref() != Some(want) {
            bad.push(item.to_owned());
        }
        Ok(())
    };
    expect("unit-cpus.txt", "Cpus_allowed_list:", PINNED, "unit-cpus")?;
    expect("host-before.txt", "slice-cpus:", PINNED, "slice-cpus")?;
    expect(
        "wrapper-cpus.txt",
        "wrapper Cpus_allowed_list:",
        HOST,
        "wrapper-cpus",
    )?;
    expect(
        "wrapper-cpus.txt",
        "sampler Cpus_allowed_list:",
        HOST,
        "sampler-cpus",
    )?;
    let path = state.join("shield.txt");
    let shield = if path.is_file() {
        text(&path)?
    } else {
        String::new()
    };
    let records = |tag: &str| -> Vec<String> {
        shield
            .lines()
            .filter_map(|l| {
                l.split_once(&format!(" {tag}: "))
                    .map(|(_, v)| v.to_owned())
            })
            .collect()
    };
    let shielded = |sets: Option<&String>| {
        sets.is_some_and(|v| {
            let mut pairs = v.split_whitespace().peekable();
            pairs.peek().is_some()
                && pairs.all(|p| p.split_once('=').is_some_and(|(_, c)| c == HOST))
        })
    };
    if !shielded(records("shielded").first()) {
        bad.push("shield-shielded".into());
    }
    if !shielded(records("final").last()) {
        bad.push("shield-final".into());
    }
    if records("restored").is_empty() {
        bad.push("shield-restored".into());
    }
    let sleeper = host_value(&state.join("sleeper.txt"), "sleeper:")?;
    if !sleeper.is_some_and(|v| !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit())) {
        bad.push("sleeper".into());
    }
    Ok(bad)
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

pub(super) struct Call {
    pub run: u64,
    pub block: u64,
    pub arm: String,
    pub query: String,
    pub outcome: String,
    pub scored: Option<u64>,
    // Verified within 300 s, sampled peak + bound (or the HWM-sum fallback) within 32 GB.
    pub budget: bool,
    pub bound: Option<u64>,
    // The request write, UNIX ns (None = never attempted or a run before the column).
    pub request: Option<u64>,
}
// One chunk's grade: the report `ckc bench grade` prints + the facts `ckc bench decide` selects on.
pub(super) struct Assessment {
    pub report: Vec<String>,
    pub rc: String,
    pub plan: Vec<Row>,
    pub matched: bool,
    // runs.tsv holds each plan.tsv call once, at its planned block, order + arm.
    pub bijective: bool,
    // arm → its server's knowledge identity (`arm-<arm>/load.tsv`)
    pub kb: BTreeMap<String, String>,
    pub loads: Vec<Row>,
    pub pinned: bool,
    pub calls: Vec<Call>,
    // block → accepted | rejected | unassessed
    pub verdicts: BTreeMap<u64, String>,
    pub wrong: usize,
    pub unchecked: usize,
    pub unbounded: usize,
}
fn grade(args: &[String]) -> Result<bool> {
    let [run, state, frozen, manifest_path] = args else {
        return Err(HELP.into());
    };
    let m = manifest(Path::new(manifest_path))?;
    let a = assess(Path::new(run), Path::new(state), Path::new(frozen), &m)?;
    println!("{}", a.report.join("\n"));
    Ok(a.matched && a.bijective && a.wrong == 0 && a.unchecked == 0)
}
pub(super) fn assess(run: &Path, state: &Path, frozen: &Path, m: &Manifest) -> Result<Assessment> {
    let mut out = Vec::new();
    let rc = match fs::read_to_string(state.join("rc")) {
        Ok(rc) => rc.trim().to_owned(),
        Err(_) => "missing".into(),
    };
    out.push(format!("chunk_rc\t{rc}"));

    let plan = table(&run.join("plan.tsv"))?;
    let mut arms: BTreeMap<String, String> = BTreeMap::new();
    let mut queries: HashMap<u64, String> = HashMap::new();
    // run → (block, order, arm): runs.tsv must hold each planned call once, as planned.
    let mut slots: BTreeMap<u64, (u64, u64, String)> = BTreeMap::new();
    let mut planned: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut bijective = true;
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
        let arm = cell(row, "arm")?;
        let slot = (
            number(row, "block")?.ok_or("plan block")?,
            number(row, "order")?.ok_or("plan order")?,
            arm.to_owned(),
        );
        bijective &= slots.insert(n, slot).is_none();
        planned
            .entry(arm.to_owned())
            .or_default()
            .insert(query.to_owned());
    }
    let ids = identities(run)?;
    let mut matched = true;
    let mut kb = BTreeMap::new();
    let empty = HashMap::new();
    for (arm, iid) in &arms {
        let (pass, detail) = identity(
            ids.get(arm).unwrap_or(&empty),
            &frozen.join(iid),
            iid,
            m,
            planned.get(arm).unwrap_or(&BTreeSet::new()),
        )?;
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
            if let Some(identity) = rows.get("kb_identity") {
                kb.insert(arm.clone(), identity.clone());
            }
        }
    }

    let before = state.join("host-before.txt");
    let x3 = x3_config(state)?;
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
    let hz = clock_ticks(&before)?;

    let runs = table(&run.join("runs.tsv"))?;
    out.push(
        "#call\trun\tblock\tarm\tquery\toutcome\tscored_s\tpeak_rss_gb\thwm_sum_gb\tsha_ok\tbudget\tmemory_basis\tbound_gb"
            .into(),
    );
    let (mut wrong, mut unchecked, mut over, mut unbounded) = (0, 0, 0, 0);
    let mut per: BTreeMap<(String, String), Vec<(f64, u64)>> = BTreeMap::new();
    let mut blocks: BTreeMap<u64, (i64, i64)> = BTreeMap::new();
    let mut unsunk: BTreeMap<u64, usize> = BTreeMap::new();
    let mut untimed: BTreeMap<u64, usize> = BTreeMap::new();
    let mut seen = BTreeSet::new();
    let mut scored: BTreeMap<(u64, String, String), u64> = BTreeMap::new();
    let mut calls = Vec::new();
    for row in &runs {
        let n = number(row, "run")?.ok_or("runs.tsv run")?;
        let block = number(row, "block")?.ok_or("runs.tsv block")?;
        let arm = cell(row, "arm")?;
        let query = queries.get(&n).ok_or("runs.tsv run outside plan.tsv")?;
        let order = number(row, "order")?.ok_or("runs.tsv order")?;
        bijective &= seen.insert(n) && slots.get(&n) == Some(&(block, order, arm.to_owned()));
        let outcome = cell(row, "outcome")?;
        let score = number(row, "scored_ns")?;
        let peak = number(row, "peak_rss_bytes")?.unwrap_or(0);
        let hwm = number(row, "observed_hwm_sum_bytes")?.unwrap_or(0);
        let iid = arms.get(arm).ok_or("runs.tsv arm outside plan.tsv")?;
        let verified = outcome == "verified";
        let sha = match m.outputs.get(&(iid.clone(), query.clone())) {
            None => {
                unchecked += 1;
                "none"
            }
            Some(_) if !verified => "NA",
            Some(want) if want == cell(row, "output_sha256")? => "yes",
            Some(_) => {
                wrong += 1;
                "NO"
            }
        };
        // The harness's bound (a sum of per-process maxima, the producer's exact wait4 peak
        // included) ≥ the simultaneous peak; a verified run without one falls back to the
        // observed HWM sum, which bounds no process the sampler missed (counted:
        // `memory_unbounded`).
        let bound = optional(row, "memory_bound_bytes")?;
        let basis = if bound.is_some() {
            "bound"
        } else {
            unbounded += usize::from(verified);
            "hwm-sum"
        };
        let memory = if peak > BUDGET_BYTES {
            "over"
        } else if bound.unwrap_or(hwm) > BUDGET_BYTES {
            "uncertain"
        } else {
            "ok"
        };
        let budget = if verified && score.is_some_and(|s| s <= BUDGET_NS) && memory == "ok" {
            "ok".to_owned()
        } else {
            over += 1;
            format!("fail:{memory}")
        };
        calls.push(Call {
            run: n,
            block,
            arm: arm.into(),
            query: query.clone(),
            outcome: outcome.into(),
            scored: score,
            budget: budget == "ok",
            bound,
            request: optional(row, "request_unix_ns")?,
        });
        out.push(format!(
            "call\t{n}\t{block}\t{arm}\t{query}\t{outcome}\t{}\t{:.2}\t{:.2}\t{sha}\t{budget}\t{basis}\t{}",
            score.map_or("NA".into(), |s| format!("{:.3}", s as f64 / 1e9)),
            peak as f64 / 1e9,
            hwm as f64 / 1e9,
            bound.map_or("NA".into(), |b| format!("{:.2}", b as f64 / 1e9))
        ));
        if let Some(s) = score {
            per.entry((arm.into(), query.clone()))
                .or_default()
                .push((s as f64 / 1e9, peak));
            if outcome == "verified" {
                scored.insert((block, arm.into(), query.clone()), s);
            }
        }
        // The call's interval (scientist-8 S6): its request write → its answer or failure (the
        // closed sink when later), whatever the outcome; a run written before those columns falls
        // back to closed sink − score, which a call short of verified lacks. A call never timed =
        // a block the protocol left incomplete, never a host verdict.
        let closed = closed_us(run, n)?;
        let timed = if row.contains_key("request_unix_ns") {
            match (number(row, "request_unix_ns")?, number(row, "end_unix_ns")?) {
                (Some(r), Some(f)) => Some((
                    (r / 1000) as i64,
                    ((f / 1000) as i64).max(closed.unwrap_or(i64::MIN)),
                )),
                _ => None,
            }
        } else {
            score
                .zip(closed)
                .map(|(s, end)| (end - (s / 1000) as i64, end))
        };
        if score.is_none() || closed.is_none() {
            *unsunk.entry(block).or_default() += 1;
        }
        match timed {
            Some((start, end)) => {
                let b = blocks.entry(block).or_insert((start, end));
                *b = (b.0.min(start), b.1.max(end));
            }
            None => *untimed.entry(block).or_default() += 1,
        }
    }

    bijective &= seen.len() == slots.len();
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
    let loads = if run.join("loads.tsv").is_file() {
        table(&run.join("loads.tsv"))?
    } else {
        Vec::new()
    };
    for l in &loads {
        out.push(format!(
            "load\t{}\t{:.1}\trss_at_ready_gb={:.2}\t{}",
            cell(l, "arm")?,
            number(l, "load_ns")?.unwrap_or(0) as f64 / 1e9,
            number(l, "rss_at_ready_bytes")?.unwrap_or(0) as f64 / 1e9,
            cell(l, "outcome")?
        ));
    }

    let host = samples(&state.join("host-samples.tsv"))?;
    out.push("#block\tn\tstart_us\tend_us\tverdict\twhy\tforeign_cpus\tmax_gap_s\tthrottle_core\tthrottle_package\tmax_temp_c\tcalls_without_sink\thost_foreign_cpus\tcalls_untimed".into());
    let all: BTreeSet<u64> = runs
        .iter()
        .filter_map(|r| number(r, "block").ok().flatten())
        .collect();
    let (mut accepted, mut rejected, mut unassessed, mut incomplete) = (BTreeSet::new(), 0, 0, 0);
    let mut verdicts = BTreeMap::new();
    for &n in &all {
        let missing = unsunk.get(&n).copied().unwrap_or(0);
        let lacking = untimed.get(&n).copied().unwrap_or(0);
        let mut v = match blocks.get(&n) {
            Some(&(s, e)) => interference(host.as_deref(), s, e, hz),
            None => [
                "unassessed",
                "no-interval",
                "NA",
                "NA",
                "NA",
                "NA",
                "NA",
                "NA",
            ]
            .map(str::to_owned),
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
        // A call never timed leaves the block's interval unknown: incomplete, never replaceable.
        if lacking > 0 {
            v[0] = "incomplete".into();
            v[1] = "untimed".into();
        }
        match v[0].as_str() {
            "accepted" => {
                accepted.insert(n);
            }
            "rejected" => rejected += 1,
            "incomplete" => incomplete += 1,
            _ => unassessed += 1,
        }
        verdicts.insert(n, v[0].clone());
        let (s, e) = blocks.get(&n).map_or(("NA".into(), "NA".into()), |b| {
            (b.0.to_string(), b.1.to_string())
        });
        out.push(format!(
            "block\t{n}\t{s}\t{e}\t{}\t{missing}\t{}\t{lacking}",
            v[..7].join("\t"),
            v[7]
        ));
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
        "summary\tidentity={}\tsha_bad={wrong}\tsha_unchecked={unchecked}\tbudget_fail={over}\tblocks={}\taccepted={}\trejected={rejected}\tunassessed={unassessed}\tincomplete={incomplete}\tx3={x3}\tpinned={pinned}\tmemory_unbounded={unbounded}\truns_plan={}",
        if matched { "ok" } else { "bad" },
        all.len(),
        accepted.len(),
        if bijective { "ok" } else { "bad" }
    ));
    Ok(Assessment {
        report: out,
        rc,
        plan,
        matched,
        bijective,
        kb,
        loads,
        pinned: pinned == "ok",
        calls,
        verdicts,
        wrong,
        unchecked,
        unbounded,
    })
}
// CLK_TCK from the host record; the records before the field come from the pool host (USER_HZ 100).
fn clock_ticks(before: &Path) -> Result<f64> {
    match host_value(before, "clk_tck:")? {
        Some(v) => v.parse::<f64>().map_err(|_| format!("clk_tck: {v}")),
        None => Ok(100.0),
    }
}

// Contract sb § Carried rulings (preflight, its rule fixed before the confirmation): the marks of
// `preflight.tsv` (`<window>-start|end`, epoch µs; `<control>-usage_us` = a wrapper burner's own
// cgroup CPU µs, `NA` = its unit failed) frame three synthetic controls of 60–65 s each, in order —
// a burner inside the sampled unit on CPU 2 (the unit's CPU time 0.9–1.1 CPUs of its window, pinned
// foreign ≤ 0.2), one loop outside it on CPU 3 (own time 0.9–1.1, pinned foreign within 0.2 of it),
// three loops on CPUs 0 1 4 (own time 2.7–3.3, pinned foreign ≤ 0.2, host foreign ≥ own − 0.2: the
// four host CPUs hold ≤ 4 CPUs of work, so a host column blind to the loops reads ≤ 4.2 − own,
// whatever the background) — + the quiet stretch, cut into consecutive 150 s windows (≥ 4), each
// passing X1 (pinned foreign ≤ 0.5); every window passes X4 (gap ≤ 5 s); X3 same + the policy
// applied. An own rate over its window = did the burner run its 60 s; beside a sampled rate it
// divides by the samples' bracket, which holds the burner's whole life. rc 0 = go, rc 1 = hold.
fn preflight_check(args: &[String]) -> Result<bool> {
    let [state] = args else {
        return Err(PREFLIGHT_HELP.into());
    };
    let (go, out) = preflight_assess(Path::new(state))?;
    println!("{}", out.join("\n"));
    Ok(go)
}
// The preflight's verdict + report lines, shared with `ckc bench decide` (the closure's named
// preflight).
pub(super) fn preflight_assess(state: &Path) -> Result<(bool, Vec<String>)> {
    let hz = clock_ticks(&state.join("host-before.txt"))?;
    let rows = samples(&state.join("host-samples.tsv"))?.ok_or("no host-samples.tsv")?;
    let (mut marks, mut usage) = (HashMap::new(), HashMap::new());
    for line in text(&state.join("preflight.tsv"))?.lines() {
        let Some((name, at)) = line.split_once('\t') else {
            continue;
        };
        if let Some(control) = name.strip_suffix("-usage_us") {
            usage.insert(control.to_owned(), at.parse::<u64>().ok());
        } else {
            let at: i64 = at.parse().map_err(|_| format!("preflight.tsv: {line}"))?;
            marks.insert(name.to_owned(), at);
        }
    }
    let span = |name: &str| -> Result<(i64, i64)> {
        let edge = |edge: &str| {
            marks
                .get(&format!("{name}-{edge}"))
                .copied()
                .ok_or_else(|| format!("preflight.tsv: no {name}-{edge}"))
        };
        Ok((edge("start")?, edge("end")?))
    };
    let mut windows = Vec::new();
    for name in [
        "campaign-control",
        "pinned-control",
        "host-control",
        "quiet",
    ] {
        let (s, e) = span(name)?;
        windows.push((name.to_owned(), s, e));
    }
    let ordered = windows.iter().all(|w| w.1 < w.2) && windows.windows(2).all(|p| p[0].2 <= p[1].1);
    let (_, quiet, end) = windows.pop().ok_or("preflight.tsv: no quiet")?;
    let count = (end - quiet) / QUIET_US;
    for k in 0..count {
        let s = quiet + k * QUIET_US;
        windows.push((format!("quiet-{}", k + 1), s, s + QUIET_US));
    }
    let mut out = vec![
        "#window\tname\tstart_us\tend_us\tcampaign_cpus\tpinned_foreign_cpus\thost_foreign_cpus\town_cpus\tmax_gap_s\tverdict"
            .to_owned(),
    ];
    let mut go = count >= QUIET_WINDOWS && ordered;
    for (name, s, e) in &windows {
        let b = bracket(&rows, *s, *e).ok_or("host-samples.tsv: no samples")?;
        let pinned = b
            .pinned(hz)
            .ok_or("host-samples.tsv: no pinned_busy_jiffies")?;
        let (campaign, host) = (b.campaign(), b.host(hz));
        // The burner's CPU-seconds: the sampled unit's own cgroup time for campaign-control, the
        // wrapper burner's own record for the other two.
        let own = match name.as_str() {
            "campaign-control" => Some((b.hi.cpu - b.lo.cpu) as f64 / 1e6),
            name => usage.get(name).copied().flatten().map(|u| u as f64 / 1e6),
        };
        let window = (e - s) as f64 / 1e6;
        let ran = |lo: f64, hi: f64| {
            CONTROL_US.contains(&(e - s)) && own.is_some_and(|o| (lo..=hi).contains(&(o / window)))
        };
        let sampled = own.map(|o| o / b.span());
        let read = match name.as_str() {
            "campaign-control" => ran(0.9, 1.1) && pinned <= 0.2,
            "pinned-control" => ran(0.9, 1.1) && sampled.is_some_and(|o| (pinned - o).abs() <= 0.2),
            "host-control" => {
                ran(2.7, 3.3) && pinned <= 0.2 && sampled.is_some_and(|o| host >= o - 0.2)
            }
            _ => pinned <= FOREIGN_CPUS,
        };
        let ok = read && b.gap <= GAP_US;
        go &= ok;
        out.push(format!(
            "window\t{name}\t{s}\t{e}\t{campaign:.3}\t{pinned:.3}\t{host:.3}\t{}\t{:.2}\t{}",
            own.map_or("NA".into(), |o| format!("{:.3}", o / window)),
            b.gap as f64 / 1e6,
            if ok { "ok" } else { "bad" }
        ));
    }
    let x3 = x3_config(state)?;
    let policy = policy(state)?;
    go &= x3 == "same" && policy.is_empty();
    out.push(format!(
        "preflight\tquiet_windows={count}\torder={}\tx3={x3}\tpolicy={}\tverdict={}",
        if ordered { "ok" } else { "bad" },
        if policy.is_empty() {
            "ok".to_owned()
        } else {
            policy.join("+")
        },
        if go { "go" } else { "hold" }
    ));
    Ok((go, out))
}
