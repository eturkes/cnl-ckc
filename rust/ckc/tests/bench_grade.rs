// Contract sb D6 + § Carried rulings (affinity + interference exclusion, operationalized there):
// `ckc bench grade` over synthetic chunks. Every exclusion rule fires on its planted host record and
// its control passes; planted identity + output-digest mismatches refuse (rc 1); R over the accepted
// blocks drops a rejected block's pair. Expected identities come from the manifest framing written
// out here, independent of the grade's code.
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn program() -> PathBuf {
    std::env::var_os("CKC_BENCH_GRADE_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_ckc").into())
}
fn sha(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

const T0: i64 = 1_800_000_000_000_000; // µs: block 1's first request
const BLOCK_US: i64 = 60_000_000; // blocks start 60 s apart
const GAP_US: i64 = 500_000; // between one call's closed sink + the next request
const TARGET: &str = "T-g100-s0";
const BASELINE: &str = "T-g001-s0";
const SEED: u64 = 1102;
const HARNESS: &str = "4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a";
const CHECKER: &str = "c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4";
const CENSUS: &str = "3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c";
const OTHER: &str = "0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f";
const CHECKER_PATH: &str = "/pool/bin/checker";
const SCHEMA: &[u8] = b"schema bytes\n";

#[derive(Clone)]
struct Chunk {
    name: &'static str,
    blocks: i64,
    paired: bool,
    foreign: fn(&Chunk, i64) -> f64, // foreign CPUs on the campaign's CPUs 2 3 6 7 at t
    host_extra: fn(&Chunk, i64) -> f64, // foreign CPUs on the other CPUs at t
    pinned_column: bool,             // host-samples.tsv carries pinned_busy_jiffies
    swap: fn(&Chunk, i64) -> Option<i64>, // the unit's swap bytes at t (None = NA)
    sampled: fn(&Chunk, i64) -> bool, // a sample exists at t
    sampler: bool,                   // host-samples.tsv written
    after: (&'static str, u32, bool), // after-record governor, scaling MHz %, every key present
    cpus: Option<&'static str>,      // the unit's Cpus_allowed_list
    score: fn(i64, &str, &str) -> u64, // ns per (block, arm, query)
    memory: fn(i64, &str, &str) -> (u64, u64), // (peak RSS, HWM sum) per call
    bound: fn(i64, &str, &str) -> Option<u64>, // the harness's memory bound per call (None = empty)
    bound_column: bool,              // runs.tsv carries memory_bound_bytes
    output: fn(i64, &str, &str) -> Option<String>, // a planted output digest
    expected: fn(&str, &str) -> bool, // the manifest carries an expected output per (instance, query)
    identity: fn(&str, &str, String) -> Option<String>, // a planted identities.tsv digest per path
    drop: fn(&str) -> bool,           // identities.tsv paths left out
    mirror: fn(&str) -> bool,         // identities.tsv paths recorded again under a second root
    sink: fn(i64, &str, &str) -> bool, // a closed sink written per (block, arm, query)
    throttle: fn(&Chunk, i64) -> (i64, i64), // cumulative core + package throttle counts at t
    seed: u64,                        // settings.tsv seed (+ the R bootstrap's)
    only: &'static [i64],             // a replacement's replayed blocks (empty = every block)
    t0: i64,                          // block 1's first request (µs)
    rc: &'static str,                 // the unit's recorded rc
    // Per call: short of `verified` = no score, output, bound or closed sink.
    outcome: fn(i64, &str, &str) -> &'static str,
    settings: fn(&str, String) -> String, // a planted settings.tsv value per key
    harness: &'static str,                // host.tsv harness_sha256
    checker: &'static str,                // the served checker's identities.tsv digest
}
fn clean(name: &'static str) -> Chunk {
    Chunk {
        name,
        blocks: 2,
        paired: false,
        foreign: |_, _| 0.0,
        host_extra: |_, _| 0.0,
        pinned_column: true,
        swap: |_, _| Some(0),
        sampled: |_, _| true,
        sampler: true,
        after: ("performance", 55, true),
        cpus: Some("2-3,6-7"),
        score: |_, _, _| 1_500_000_000,
        memory: |_, _, _| (1_000_000_000, 1_200_000_000),
        bound: |_, _, _| Some(1_300_000_000),
        bound_column: true,
        output: |_, _, _| None,
        expected: |_, _| true,
        identity: |_, _, _| None,
        drop: |_| false,
        mirror: |_| false,
        sink: |_, _, _| true,
        throttle: |_, _| (0, 0),
        seed: SEED,
        only: &[],
        t0: T0,
        rc: "0",
        outcome: |_, _, _| "verified",
        settings: |_, value| value,
        harness: HARNESS,
        checker: CHECKER,
    }
}
// Block b's calls, in order: (arm, query, start µs, end µs).
fn calls(c: &Chunk, b: i64) -> Vec<(&'static str, &'static str, i64, i64)> {
    let order: Vec<(&str, &str)> = if c.paired {
        vec![("target", "q-one"), ("baseline", "q-one")]
    } else {
        vec![
            ("target", "q-all"),
            ("target", "q-one"),
            ("target", "q-patient"),
        ]
    };
    let mut t = c.t0 + (b - 1) * BLOCK_US;
    order
        .into_iter()
        .map(|(arm, query)| {
            let end = t + ((c.score)(b, arm, query) / 1000) as i64;
            let call = (arm, query, t, end);
            t = end + GAP_US;
            call
        })
        .collect()
}
fn block_span(c: &Chunk, b: i64) -> (i64, i64) {
    let v = calls(c, b);
    (v[0].2, v[v.len() - 1].3)
}
fn host(governor: &str, mhz: u32, full: bool) -> String {
    let mut text = format!(
        "2026-11-05T00:00:00Z\n6.6.0-test\nCPU(s):                                  8\nModel name:                              Intel(R) Core(TM) i7-8665U CPU @ 1.90GHz\nCPU(s) scaling MHz:                      {mhz}%\ngovernors:  8 {governor} \n"
    );
    if full {
        text.push_str("online: 0-7\nslice-cpus: NA\nclk_tck: 100\n");
    }
    text
}
fn query_file(q: &str) -> String {
    if q == "q-all" {
        format!("{q}.txt")
    } else {
        format!("{q}.pl")
    }
}
struct Fixture {
    dir: PathBuf,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}
struct Graded {
    rc: i32,
    stdout: String,
    _fixture: Fixture,
}
impl Graded {
    fn rows(&self, kind: &str) -> Vec<Vec<&str>> {
        self.stdout
            .lines()
            .filter(|l| l.split('\t').next() == Some(kind))
            .map(|l| l.split('\t').collect())
            .collect()
    }
    fn blocks(&self) -> Vec<(String, String)> {
        self.rows("block")
            .iter()
            .map(|r| (r[4].to_owned(), r[5].to_owned()))
            .collect()
    }
    fn summary(&self) -> BTreeMap<String, String> {
        self.rows("summary")[0][1..]
            .iter()
            .filter_map(|kv| kv.split_once('='))
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }
    fn holds(&self) -> Vec<&str> {
        self.rows("hold").iter().map(|r| r[1]).collect()
    }
    // `decide verdict=<go|hold> holds=<n>`: n = the hold rows printed.
    fn verdict(&self) -> &str {
        let line = &self.rows("decide")[0];
        assert_eq!(
            line[2],
            format!("holds={}", self.holds().len()),
            "{}",
            self.stdout
        );
        line[1].strip_prefix("verdict=").expect("decide verdict=")
    }
}
fn instance_base(iid: &str) -> String {
    format!("rust/target/bench/sb1-frozen/{iid}")
}
fn composition(iid: &str) -> Vec<String> {
    ["a.pl", "b.pl"]
        .iter()
        .map(|f| format!("{}/copies/0001/{f}", instance_base(iid)))
        .collect()
}
// The knowledge identity a server reports for an instance (64 hex, as the census names receipts).
fn kb_of(iid: &str) -> String {
    sha(format!("kb {iid}").as_bytes())
}
// The frozen instances + the run manifest, shared by a campaign's attempts: both instances, every
// query, the expected outputs `c.expected` keeps (union rows for the target, recipe oracle rows
// for the baseline). Expected identities come from the manifest framing written out here,
// independent of the grade's code.
fn freeze(c: &Chunk, dir: &Path) -> (PathBuf, PathBuf) {
    let frozen = dir.join("frozen");
    let mut manifest = "kind\tinstance\tname\tpath\tsha256\tbytes\tcount\tvalue\n".to_owned();
    for iid in [TARGET, BASELINE] {
        let base = instance_base(iid);
        let lines = composition(iid);
        fs::create_dir_all(frozen.join(iid)).unwrap();
        fs::write(
            frozen.join(iid).join("composition.tsv"),
            lines.join("\n") + "\n",
        )
        .unwrap();
        let mut framed = format!("schema\0{}\0{}\n", SCHEMA.len(), sha(SCHEMA));
        for line in &lines {
            let bytes = format!("document {line}\n");
            framed.push_str(&format!(
                "file\0{line}\0{}\0{}\n",
                bytes.len(),
                sha(bytes.as_bytes())
            ));
        }
        manifest.push_str(&format!(
            "instance\t{iid}\tcomposition\t{base}/composition.tsv\t{}\t0\t0\t-\n",
            sha(framed.as_bytes())
        ));
        for q in ["q-all", "q-one", "q-patient"] {
            let file = query_file(q);
            let bytes = format!("query {iid} {q}\n");
            manifest.push_str(&format!(
                "query\t{iid}\t{file}\t{base}/queries/{file}\t{}\t{}\t1\tfrozen\n",
                sha(bytes.as_bytes()),
                bytes.len()
            ));
            let want = sha(format!("output {iid} {q}\n").as_bytes());
            let row = if iid == TARGET {
                format!("union\t{iid}\t{q}\t-\t{want}\t0\t0\t-\n")
            } else {
                format!("oracle\t{iid}\trecipe:{q}\t-\t{want}\t0\t0\t-\n")
            };
            if (c.expected)(iid, q) {
                manifest.push_str(&row);
            }
        }
    }
    let path = dir.join("run-manifest.tsv");
    fs::write(&path, manifest).unwrap();
    (frozen, path)
}
// One attempt's run + state directories, as `ckc bench run` + `tools/sb-campaign-unit` write them.
fn attempt(c: &Chunk, run: &Path, state: &Path) {
    for d in [run, state] {
        fs::create_dir_all(d).unwrap();
    }
    let arms: Vec<(&str, &str)> = if c.paired {
        vec![("target", TARGET), ("baseline", BASELINE)]
    } else {
        vec![("target", TARGET)]
    };
    let mut identities = "arm\tpath\tbytes\tsha256\n".to_owned();
    let mut record = |arm: &str, rel: &str, bytes: &[u8]| {
        if (c.drop)(rel) {
            return;
        }
        let digest = (c.identity)(arm, rel, sha(bytes)).unwrap_or_else(|| sha(bytes));
        identities.push_str(&format!("{arm}\t/pool/{rel}\t{}\t{digest}\n", bytes.len()));
        if (c.mirror)(rel) {
            identities.push_str(&format!(
                "{arm}\t/mirror/{rel}\t{}\t{digest}\n",
                bytes.len()
            ));
        }
    };
    for (arm, iid) in &arms {
        let base = instance_base(iid);
        record(arm, "guidelines/schema.pl", SCHEMA);
        for line in composition(iid) {
            record(arm, &line, format!("document {line}\n").as_bytes());
        }
        for q in ["q-all", "q-one", "q-patient"] {
            let file = query_file(q);
            let bytes = format!("query {iid} {q}\n");
            record(arm, &format!("{base}/queries/{file}"), bytes.as_bytes());
        }
        fs::create_dir_all(run.join(format!("arm-{arm}"))).unwrap();
        fs::write(
            run.join(format!("arm-{arm}/load.tsv")),
            format!("kb_identity\t{}\nreceipt\tunchecked\n", kb_of(iid)),
        )
        .unwrap();
    }
    // The served checker's identity file (`--identity-file`), hashed per arm.
    for (arm, _) in &arms {
        identities.push_str(&format!("{arm}\t{CHECKER_PATH}\t1000\t{}\n", c.checker));
    }
    fs::write(run.join("identities.tsv"), identities).unwrap();

    let mut plan = "run\tblock\torder\tarm\tcomposition\tquery\n".to_owned();
    let mut runs = format!(
        "run\tblock\torder\tarm\toutcome\tscored_ns\tpeak_rss_bytes\tobserved_hwm_sum_bytes\toutput_sha256{}\n",
        if c.bound_column {
            "\tmemory_bound_bytes"
        } else {
            ""
        }
    );
    let mut n = 0;
    for b in 1..=c.blocks {
        for (k, (arm, query, _, end)) in calls(c, b).into_iter().enumerate() {
            // A replacement keeps the full plan's run numbers for the blocks it replays.
            n += 1;
            if !c.only.is_empty() && !c.only.contains(&b) {
                continue;
            }
            let iid = if arm == "target" { TARGET } else { BASELINE };
            let base = format!("/pool/{}", instance_base(iid));
            plan.push_str(&format!(
                "{n}\t{b}\t{}\t{arm}\t{base}/composition.tsv\t{base}/queries/{}\n",
                k + 1,
                query_file(query)
            ));
            let outcome = (c.outcome)(b, arm, query);
            let verified = outcome == "verified";
            let (peak, hwm) = (c.memory)(b, arm, query);
            let (score, output, bound) = if verified {
                (
                    (c.score)(b, arm, query).to_string(),
                    (c.output)(b, arm, query)
                        .unwrap_or_else(|| sha(format!("output {iid} {query}\n").as_bytes())),
                    (c.bound)(b, arm, query).map_or(String::new(), |v| v.to_string()),
                )
            } else {
                (String::new(), String::new(), String::new())
            };
            let bound = if c.bound_column {
                format!("\t{bound}")
            } else {
                String::new()
            };
            runs.push_str(&format!(
                "{n}\t{b}\t{}\t{arm}\t{outcome}\t{score}\t{peak}\t{hwm}\t{output}{bound}\n",
                k + 1
            ));
            let sink = run.join(format!("run-{n:06}"));
            fs::create_dir_all(&sink).unwrap();
            if verified && (c.sink)(b, arm, query) {
                fs::write(
                    sink.join("sink-complete.tsv"),
                    format!("sb1-sink 1\nclosed_unix_ns\t{}\nbytes\t1\n", end * 1000),
                )
                .unwrap();
            }
        }
    }
    fs::write(run.join("plan.tsv"), plan).unwrap();
    fs::write(run.join("runs.tsv"), runs).unwrap();
    let mut loads =
        "arm\tstart_ns\tready_ns\tload_ns\trss_at_ready_bytes\toutcome\tdetail\n".to_owned();
    for (arm, _) in &arms {
        loads.push_str(&format!("{arm}\t0\t1\t120000000000\t6000000000\tready\t\n"));
    }
    fs::write(run.join("loads.tsv"), loads).unwrap();
    if c.paired {
        // The harness's spelling (`run/report.rs`): the query column holds the query file name.
        fs::write(
            run.join("ratio.tsv"),
            format!("query\tpairs\tstatus\tratio_of_medians\tone_sided_95_upper\tbootstrap_resamples\tseed\nq-one.pl\t{}\tcomplete\t2.000000000\t2.000000000\t10000\t{}\n", c.blocks, c.seed),
        )
        .unwrap();
    }
    let (queries, spacing) = if c.paired {
        ("q-one", "30000000000")
    } else {
        ("q-all,q-one,q-patient", "0")
    };
    let only: Vec<String> = c.only.iter().map(i64::to_string).collect();
    let mut settings = "key\tvalue\n".to_owned();
    for (key, value) in [
        ("seed", c.seed.to_string()),
        ("blocks", c.blocks.to_string()),
        ("queries", queries.to_owned()),
        ("spacing_ns", spacing.to_owned()),
        ("only_blocks", only.join(",")),
        (
            "serve",
            format!("{CHECKER_PATH} serve --provider static --ta-receipts none --producer /pool/p"),
        ),
    ] {
        settings.push_str(&format!("{key}\t{}\n", (c.settings)(key, value)));
    }
    fs::write(run.join("settings.tsv"), settings).unwrap();
    fs::write(
        run.join("host.tsv"),
        format!("key\tvalue\nharness_sha256\t{}\n", c.harness),
    )
    .unwrap();

    fs::write(state.join("rc"), format!("{}\n", c.rc)).unwrap();
    fs::write(state.join("host-before.txt"), host("performance", 77, true)).unwrap();
    fs::write(
        state.join("host-after.txt"),
        host(c.after.0, c.after.1, c.after.2),
    )
    .unwrap();
    if let Some(cpus) = c.cpus {
        fs::write(
            state.join("unit-cpus.txt"),
            format!("CPUAffinity=2-3 6-7\nMainPID=1\nCpus_allowed_list:\t{cpus}\n"),
        )
        .unwrap();
    }
    if c.sampler {
        let pinned_head = if c.pinned_column {
            "\tpinned_busy_jiffies"
        } else {
            ""
        };
        let mut rows = format!(
            "epoch_us\tbusy_jiffies\tidle_jiffies\tcampaign_cpu_us\tcampaign_swap_bytes\tpswpin\tpswpout\tcore_throttle\tpackage_throttle\tmax_temp_mc{pinned_head}\n"
        );
        let end = block_span(c, c.blocks).1 + 30_000_000;
        let (mut t, mut busy, mut pinned, mut cpu) = (c.t0 - 30_000_000, 1_000_i64, 400_i64, 0_i64);
        while t <= end {
            if (c.sampled)(c, t) {
                let swap = (c.swap)(c, t).map_or("NA".into(), |b| b.to_string());
                let (core, package) = (c.throttle)(c, t);
                let pinned_cell = if c.pinned_column {
                    format!("\t{pinned}")
                } else {
                    String::new()
                };
                rows.push_str(&format!(
                    "{t}\t{busy}\t5000\t{cpu}\t{swap}\t0\t0\t{core}\t{package}\t50000{pinned_cell}\n"
                ));
            }
            // The next second: the campaign holds 2 of its CPUs, foreign work the rest of them +
            // the host's other CPUs (USER_HZ 100).
            let own = ((2.0 + (c.foreign)(c, t)) * 100.0).round() as i64;
            pinned += own;
            busy += own + ((c.host_extra)(c, t) * 100.0).round() as i64;
            cpu += 2_000_000;
            t += 1_000_000;
        }
        fs::write(state.join("host-samples.tsv"), rows).unwrap();
    }
}
fn graded(out: std::process::Output, fixture: Fixture) -> Graded {
    assert!(
        out.status.code() != Some(2),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    Graded {
        rc: out.status.code().unwrap(),
        stdout: String::from_utf8(out.stdout).unwrap(),
        _fixture: fixture,
    }
}
fn scratch(name: &str) -> Fixture {
    let dir = root().join(format!(
        "rust/target/bench-grade/{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    Fixture { dir }
}
fn grade(c: &Chunk) -> Graded {
    let fixture = scratch(c.name);
    let (frozen, manifest) = freeze(c, &fixture.dir);
    let (run, state) = (fixture.dir.join("run"), fixture.dir.join("state"));
    attempt(c, &run, &state);
    let out = Command::new(program())
        .args(["bench", "grade"])
        .args([&run, &state, &frozen, &manifest])
        .output()
        .unwrap();
    graded(out, fixture)
}

// A confirmation campaign for `ckc bench decide`: the plan rows, each attempt (`runs/<id>` +
// `state/<id>`), the census receipt per knowledge identity (None = no `--receipts`), runs/
// directories outside the plan.
struct Campaign {
    name: &'static str,
    plan: String,
    attempts: Vec<(&'static str, Chunk)>,
    receipts: Option<fn(&str) -> Option<String>>,
    extra: &'static [&'static str],
}
fn plan_row(chunk: &str, paired: bool, blocks: i64, seed: u64, ratio: &str) -> String {
    let pair = if paired { BASELINE } else { "-" };
    format!("{chunk}\t{TARGET}\t{blocks}\t{seed}\t{pair}\t{ratio}\t{HARNESS}\t{CHECKER}\n")
}
// An admitted direct receipt of the census binary on the pool: full load = 120 s prepared load +
// 50 s parse + 1,500 s analysis + the 10 s overhead = 1,680 s; charged = 20 + 6 GB at ready.
fn receipt_body(kb: &str, plants: &[(&str, &str)]) -> String {
    let mut fields: Vec<(&str, String)> = vec![
        ("format", "sb1-ta-receipt 1".into()),
        ("kb_identity", kb.into()),
        ("knowledge_documents", "2".into()),
        ("knowledge_bytes", "100".into()),
        ("verdict", "admitted".into()),
        ("ta_source_commit", "a".repeat(40)),
        ("ta_binary_sha256", CENSUS.into()),
        ("dc", "1".into()),
        ("cap_bytes", "30064771072".into()),
        ("vmhwm_bytes", "20000000000".into()),
        ("parse_seconds", "50.0".into()),
        ("analysis_seconds", "1500.0".into()),
        ("host", "pool".into()),
        ("command", "ta --dc --patient last".into()),
    ];
    for &(key, value) in plants {
        match fields.iter_mut().find(|f| f.0 == key) {
            Some(f) => f.1 = value.into(),
            None => fields.push((key, value.into())),
        }
    }
    fields.iter().map(|(k, v)| format!("{k}\t{v}\n")).collect()
}
// r01 = the gating R pair (seed 1102), c01 = a three-query chunk (seed 1103), both clean.
fn campaign(name: &'static str) -> Campaign {
    let mut r01 = clean(name);
    r01.paired = true;
    let mut c01 = clean(name);
    c01.seed = SEED + 1;
    Campaign {
        name,
        plan: plan_row("r01", true, 2, SEED, "gate") + &plan_row("c01", false, 2, SEED + 1, "-"),
        attempts: vec![("r01", r01), ("c01", c01)],
        receipts: Some(|kb| Some(receipt_body(kb, &[]))),
        extra: &[],
    }
}
// A replacement of chunk `of`: its blocks, seed + plan size, one hour after the original.
fn replacement(of: &Chunk, only: &'static [i64]) -> Chunk {
    let mut r = clean(of.name);
    (r.paired, r.blocks, r.seed, r.only) = (of.paired, of.blocks, of.seed, only);
    r.t0 = T0 + 3_600_000_000;
    r
}
fn decide(k: &Campaign) -> Graded {
    let fixture = scratch(k.name);
    let dir = fixture.dir.clone();
    let (frozen, manifest) = freeze(&k.attempts[0].1, &dir);
    fs::create_dir_all(dir.join("runs")).unwrap();
    for (id, c) in &k.attempts {
        attempt(c, &dir.join("runs").join(id), &dir.join("state").join(id));
    }
    for extra in k.extra {
        fs::create_dir_all(dir.join("runs").join(extra)).unwrap();
    }
    let plan = dir.join("plan.tsv");
    fs::write(
        &plan,
        format!(
            "chunk\tinstance\tblocks\tseed\tpair\tratio\tckc\tchecker\n{}",
            k.plan
        ),
    )
    .unwrap();
    let mut command = Command::new(program());
    command.args(["bench", "decide"]);
    command.args([&plan, &dir, &frozen, &manifest]);
    if let Some(receipt) = k.receipts {
        let receipts = dir.join("receipts");
        fs::create_dir_all(&receipts).unwrap();
        for iid in [TARGET, BASELINE] {
            let kb = kb_of(iid);
            if let Some(body) = receipt(&kb) {
                fs::write(receipts.join(format!("{kb}.tsv")), body).unwrap();
            }
        }
        command.arg("--receipts").arg(&receipts);
        command.args([
            "--census",
            CENSUS,
            "--host",
            "pool",
            "--overhead-ms",
            "10000",
        ]);
    }
    graded(command.output().unwrap(), fixture)
}
fn within(c: &Chunk, b: i64, t: i64) -> bool {
    let (s, e) = block_span(c, b);
    (s - 1_000_000..=e + 1_000_000).contains(&t)
}
fn accepted(n: usize) -> Vec<(String, String)> {
    vec![("accepted".to_owned(), "-".to_owned()); n]
}

#[test]
fn clean_chunk_matches_identity_and_accepts_every_block() {
    let c = clean("clean");
    let g = grade(&c);
    assert_eq!(g.rc, 0, "{}", g.stdout);
    assert_eq!(g.blocks(), accepted(2));
    let identity = &g.rows("identity")[0];
    assert_eq!(
        (identity[3], identity[4], identity[5]),
        ("MATCH", "queries-checked=3", "bad=0")
    );
    assert!(
        g.rows("call")
            .iter()
            .all(|r| r[9] == "yes" && r[10] == "ok")
    );
    let s = g.summary();
    assert_eq!(
        (
            s["identity"].as_str(),
            s["x3"].as_str(),
            s["pinned"].as_str()
        ),
        ("ok", "same", "ok")
    );
    assert_eq!(
        [
            "blocks",
            "accepted",
            "rejected",
            "unassessed",
            "budget_fail",
            "sha_bad",
            "sha_unchecked"
        ]
        .map(|k| &s[k][..]),
        ["2", "2", "0", "0", "0", "0", "0"]
    );
    assert_eq!(g.rows("kb")[0][2], kb_of(TARGET));
    // A block's interval = its first call's request write (closed sink − score) → its last sink.
    let (start, end) = block_span(&c, 1);
    assert_eq!(
        g.rows("block")[0][2..4],
        [start.to_string(), end.to_string()]
    );
    assert_eq!(
        g.rows("stat")[0][1..],
        ["target", "q-all", "2", "1.500", "0.000", "1.500", "1.00"]
    );
}

#[test]
fn foreign_cpu_above_half_rejects_its_block_alone() {
    let mut c = clean("x1");
    // Blocks 1–3 carry 0.6, 0.4 + exactly 0.5 foreign CPUs: X1 rejects above its 0.5 line alone.
    c.blocks = 3;
    c.foreign = |c, t| {
        if within(c, 1, t) {
            0.6
        } else if within(c, 2, t) {
            0.4
        } else if within(c, 3, t) {
            0.5
        } else {
            0.0
        }
    };
    let g = grade(&c);
    assert_eq!(g.blocks()[0], ("rejected".into(), "X1".into()));
    assert_eq!(g.blocks()[1..], accepted(2));
    assert_eq!(
        [g.rows("block")[0][6], g.rows("block")[2][6]],
        ["0.600", "0.500"]
    );
    let s = g.summary();
    assert_eq!(
        ["accepted", "rejected", "unassessed"].map(|k| &s[k][..]),
        ["2", "1", "0"]
    );
}

#[test]
fn unit_swap_rejects_its_block_and_unreadable_swap_leaves_it_unassessed() {
    let mut c = clean("x2");
    // The sample 1 s into block 2 holds 4096 swap bytes (samples sit on whole seconds from T0 − 30 s).
    c.swap = |c, t| {
        Some(if t == block_span(c, 2).0 + 1_000_000 {
            4096
        } else {
            0
        })
    };
    let g = grade(&c);
    assert_eq!(g.blocks()[0], ("accepted".into(), "-".into()));
    assert_eq!(
        g.blocks()[1],
        ("rejected".into(), "X2".into()),
        "{}",
        g.stdout
    );
    let mut c = clean("x2-unreadable");
    c.swap = |_, _| None;
    let g = grade(&c);
    assert_eq!(
        g.blocks(),
        vec![("unassessed".to_owned(), "X2-unreadable".to_owned()); 2]
    );
    let s = g.summary();
    assert_eq!(
        ["accepted", "rejected", "unassessed"].map(|k| &s[k][..]),
        ["0", "0", "2"]
    );
}

#[test]
fn sampling_gaps_reject_through_x4() {
    let mut c = clean("x4-hole");
    c.sampled = |c, t| !within(c, 1, t);
    let g = grade(&c);
    assert_eq!(
        g.blocks()[0],
        ("rejected".into(), "X4".into()),
        "{}",
        g.stdout
    );
    assert_eq!(g.blocks()[1], ("accepted".into(), "-".into()));
    let mut c = clean("x4-tail");
    // Sampling ends 6 s before block 2 begins.
    c.sampled = |c, t| t < block_span(c, 2).0 - 6_000_000;
    assert_eq!(grade(&c).blocks()[1].1, "X4");
    // A 4 s block whose samples ended 3 s before it: the uncovered stretch from the last sample is
    // 7 s, though the block alone is shorter than X4's line.
    let mut c = clean("x4-short-tail");
    c.score = |_, _, _| 1_000_000_000;
    c.sampled = |c, t| t < block_span(c, 2).0 - 3_000_000;
    assert_eq!(grade(&c).blocks()[1].1, "X4");
    // A 1.6 s block whose samples ended 0.5 s before it: the open edge stays under X4's line, yet
    // no sample brackets the block, so X1 has no window ⇒ X4.
    let mut c = clean("x4-unbracketed-short");
    c.score = |_, _, _| 200_000_000;
    c.sampled = |c, t| t < block_span(c, 2).0 - 500_000;
    assert_eq!(grade(&c).blocks()[1].1, "X4");
    let mut c = clean("x4-tail-ok");
    // Sampling ends 0.5 s before block 2's last sink: an edge under one period passes.
    c.sampled = |c, t| t <= block_span(c, 2).1 - 500_000;
    assert_eq!(grade(&c).blocks(), accepted(2));
    // A spacing of exactly 5 s inside block 1: X4's line is strict.
    let mut c = clean("x4-five");
    c.sampled = |c, t| {
        let s = block_span(c, 1).0;
        !(s < t && t < s + 5_000_000)
    };
    let g = grade(&c);
    assert_eq!(g.blocks(), accepted(2));
    assert_eq!(g.rows("block")[0][7], "5.00");
    // 31 s blocks whose sampling begins inside block 1 (window = first sample → the first after
    // the block) or ends inside block 2 (the last sample before the block → the last sample): the
    // open edge alone decides X4.
    let long: fn(i64, &str, &str) -> u64 = |_, _, _| 10_000_000_000;
    let mut c = clean("x4-head-edge");
    c.score = long;
    c.sampled = |c, t| t >= block_span(c, 1).0 + 6_000_000;
    let g = grade(&c);
    assert_eq!(g.blocks()[0], ("rejected".into(), "X4".into()));
    assert_eq!(g.rows("block")[0][7], "6.00");
    assert_eq!(g.blocks()[1], ("accepted".into(), "-".into()));
    let mut c = clean("x4-head-edge-ok");
    c.score = long;
    c.sampled = |c, t| t >= block_span(c, 1).0 + 4_000_000;
    let g = grade(&c);
    assert_eq!(g.blocks(), accepted(2));
    assert_eq!(g.rows("block")[0][7], "4.00");
    let mut c = clean("x4-tail-edge");
    c.score = long;
    c.sampled = |c, t| t <= block_span(c, 2).1 - 6_000_000;
    let g = grade(&c);
    assert_eq!(g.blocks()[0], ("accepted".into(), "-".into()));
    assert_eq!(g.blocks()[1], ("rejected".into(), "X4".into()));
    assert_eq!(g.rows("block")[1][7], "6.00");
    // A 7 s spacing that ends at block 1's first request or begins at its last sink covers none of
    // the block (the X1 window starts + ends on those samples).
    let mut c = clean("x4-meets-start");
    c.sampled = |c, t| {
        let s = block_span(c, 1).0;
        !(s - 7_000_000 < t && t < s)
    };
    assert_eq!(grade(&c).blocks(), accepted(2));
    let mut c = clean("x4-meets-end");
    c.score = |_, _, _| 2_000_000_000; // block 1 ends on a whole second
    c.sampled = |c, t| {
        let e = block_span(c, 1).1;
        !(e < t && t < e + 7_000_000)
    };
    assert_eq!(grade(&c).blocks(), accepted(2));
    let mut c = clean("x4-head");
    // Sampling begins after block 1 ends: block 1 is unbracketed.
    c.sampled = |c, t| t > block_span(c, 1).1 + 100_000;
    assert_eq!(grade(&c).blocks()[0].1, "X4");
    let mut c = clean("x4-empty");
    c.sampled = |_, _| false;
    assert_eq!(
        grade(&c).blocks(),
        vec![("rejected".to_owned(), "X4".to_owned()); 2]
    );
    let mut c = clean("no-sampler");
    c.sampler = false;
    assert_eq!(
        grade(&c).blocks(),
        vec![("unassessed".to_owned(), "no-sampler".to_owned()); 2]
    );
}

#[test]
fn host_change_rejects_every_block_while_scaling_mhz_is_ignored() {
    let mut c = clean("x3-governor");
    c.after = ("powersave", 55, true);
    let g = grade(&c);
    assert_eq!(
        g.blocks(),
        vec![("rejected".to_owned(), "X3".to_owned()); 2]
    );
    assert_eq!(g.summary()["x3"], "DIFFERS");
    let mut c = clean("x3-mhz");
    c.after = ("performance", 12, true);
    assert_eq!(grade(&c).blocks(), accepted(2));
    let mut c = clean("x3-missing");
    c.after = ("performance", 55, false);
    let g = grade(&c);
    assert_eq!(
        g.blocks(),
        vec![("unassessed".to_owned(), "X3-missing".to_owned()); 2]
    );
    assert_eq!(g.summary()["x3"], "missing");
}

#[test]
fn affinity_record_flags_an_unpinned_unit() {
    let mut c = clean("unpinned");
    c.cpus = Some("0-7");
    assert_eq!(grade(&c).summary()["pinned"], "NOT-PINNED");
    let mut c = clean("no-cpus");
    c.cpus = None;
    let g = grade(&c);
    assert_eq!(g.rows("unit_cpus")[0][1..], ["missing", "NOT-PINNED"]);
}

#[test]
fn identity_mismatches_refuse() {
    let mut c = clean("identity-file");
    c.identity = |_, rel, _| rel.ends_with("b.pl").then(|| "0".repeat(64));
    let g = grade(&c);
    assert_eq!((g.rc, g.rows("identity")[0][3]), (1, "MISMATCH"));
    assert_eq!(g.summary()["identity"], "bad");
    let mut c = clean("identity-missing-row");
    c.drop = |rel| rel.ends_with("copies/0001/a.pl");
    let g = grade(&c);
    assert_eq!(g.rc, 1);
    assert!(
        g.rows("identity")[0][3].starts_with("no identity row for"),
        "{}",
        g.stdout
    );
    let mut c = clean("identity-query");
    c.identity = |_, rel, _| rel.ends_with("q-one.pl").then(|| "1".repeat(64));
    let g = grade(&c);
    assert_eq!((g.rc, g.rows("identity")[0][5]), (1, "bad=1"));
    // A run that recorded no frozen query proves no query identity.
    let mut c = clean("identity-no-query");
    c.drop = |rel| rel.contains("/queries/");
    let g = grade(&c);
    assert_eq!(
        (g.rc, &g.rows("identity")[0][3..6]),
        (1, &["MATCH", "queries-checked=0", "bad=0"][..])
    );
    assert_eq!(g.summary()["identity"], "bad");
    // One frozen query recorded under two roots (same bytes + digest) is ambiguous.
    let mut c = clean("identity-query-twice");
    c.mirror = |rel| rel.ends_with("q-one.pl");
    let g = grade(&c);
    assert_eq!((g.rc, g.rows("identity")[0][5]), (1, "bad=1"));
}

#[test]
fn a_wrong_output_digest_refuses() {
    let mut c = clean("output");
    c.output = |b, _, q| (b == 2 && q == "q-patient").then(|| "f".repeat(64));
    let g = grade(&c);
    assert_eq!(g.rc, 1);
    assert_eq!(g.summary()["sha_bad"], "1");
    assert_eq!(g.rows("call").iter().filter(|r| r[9] == "NO").count(), 1);
}

#[test]
fn a_call_without_an_expected_output_refuses() {
    // A manifest without the R baseline's q-one output: those calls read unchecked, never matched.
    let mut c = clean("unchecked");
    c.paired = true;
    let g = grade(&c);
    assert_eq!(g.rc, 0, "{}", g.stdout);
    assert_eq!(g.summary()["sha_unchecked"], "0");
    c.expected = |iid, q| !(iid == BASELINE && q == "q-one");
    let g = grade(&c);
    assert_eq!(g.rc, 1, "{}", g.stdout);
    let s = g.summary();
    assert_eq!(
        (s["sha_bad"].as_str(), s["sha_unchecked"].as_str()),
        ("0", "2")
    );
    let calls = g.rows("call");
    assert_eq!(
        calls
            .iter()
            .map(|r| (r[3], r[9]))
            .filter(|&(arm, sha)| arm == "baseline" && sha == "none")
            .count(),
        2
    );
    assert!(
        calls
            .iter()
            .filter(|r| r[3] == "target")
            .all(|r| r[9] == "yes")
    );
}

#[test]
fn budget_boundaries_hold_at_300_s_and_32_gb() {
    let mut c = clean("budget");
    c.blocks = 1;
    // The HWM-sum fallback's line (a run without the bound column; the bound's own line =
    // the_memory_bound_grades_each_call_and_its_absence_is_counted).
    c.bound_column = false;
    c.score = |_, _, q| {
        if q == "q-all" {
            300_000_000_000
        } else if q == "q-one" {
            300_000_000_001
        } else {
            1_000_000_000
        }
    };
    c.memory = |_, _, q| {
        if q == "q-patient" {
            (32_000_000_000, 32_000_000_001)
        } else {
            (32_000_000_000, 32_000_000_000)
        }
    };
    let g = grade(&c);
    let budget: Vec<&str> = g.rows("call").iter().map(|r| r[10]).collect();
    assert_eq!(budget, ["ok", "fail:ok", "fail:uncertain"]);
    assert_eq!(g.summary()["budget_fail"], "2");
    let mut c = clean("budget-over");
    c.blocks = 1;
    c.memory = |_, _, _| (32_000_000_001, 32_000_000_001);
    let g = grade(&c);
    assert!(g.rows("call").iter().all(|r| r[10] == "fail:over"));
    assert_eq!(g.summary()["budget_fail"], "3");
}

#[test]
fn throttle_counts_are_recorded_and_never_exclude() {
    let mut c = clean("throttle");
    // Core + package throttle counters step by 3 + 7 inside block 1 (from 10 + 20).
    c.throttle = |c, t| {
        if t >= block_span(c, 1).0 + 3_000_000 {
            (13, 27)
        } else {
            (10, 20)
        }
    };
    let g = grade(&c);
    assert_eq!(g.blocks(), accepted(2));
    let rows = g.rows("block");
    assert_eq!(rows[0][8..10], ["3", "7"]);
    assert_eq!(rows[1][8..10], ["0", "0"]);
}

#[test]
fn a_call_without_a_closed_sink_counts_in_its_block() {
    let mut c = clean("unsunk");
    c.sink = |b, _, q| !(b == 2 && q == "q-patient");
    let g = grade(&c);
    let rows = g.rows("block");
    assert_eq!([rows[0][11], rows[1][11]], ["0", "1"]);
    // Block 2's interval ends at its last closed sink (the q-one call's).
    assert_eq!(rows[1][3], calls(&c, 2)[1].3.to_string());
}

#[test]
fn accepted_blocks_drop_a_rejected_pair_from_r() {
    let mut c = clean("ratio");
    c.paired = true;
    c.blocks = 3;
    // Block 1: target = 2 × baseline, accepted; blocks 2 + 3: 10 × under 0.8 foreign CPUs, rejected.
    c.score = |b, arm, _| {
        if arm == "baseline" {
            1_000_000_000
        } else if b == 1 {
            2_000_000_000
        } else {
            10_000_000_000
        }
    };
    c.foreign = |c, t| {
        if within(c, 2, t) || within(c, 3, t) {
            0.8
        } else {
            0.0
        }
    };
    let g = grade(&c);
    assert_eq!(g.rc, 0, "{}", g.stdout);
    let r: Vec<Vec<&str>> = g.rows("ratio_recomputed");
    assert_eq!(
        r[0][1..],
        ["all", "pairs=3", "R=10.000000000", "UB95=10.000000000"]
    );
    assert_eq!(
        r[1][1..],
        ["accepted", "pairs=1", "R=2.000000000", "UB95=2.000000000"]
    );
    assert_eq!(g.rows("ratio_harness")[0][1..3], ["q-one.pl", "pairs=3"]);
    // The baseline arm's outputs grade against its recipe oracle rows.
    assert!(
        g.rows("call")
            .iter()
            .filter(|r| r[3] == "baseline")
            .all(|r| r[9] == "yes")
    );
}

#[test]
fn r_over_no_accepted_pair_is_not_estimable() {
    let mut c = clean("ratio-none");
    c.paired = true;
    c.foreign = |_, _| 0.8;
    let g = grade(&c);
    assert_eq!(g.rc, 0, "{}", g.stdout);
    assert_eq!(
        g.rows("ratio_recomputed")[1][1..],
        ["accepted", "pairs=0", "not-estimable"]
    );
}

#[test]
fn a_zero_baseline_score_leaves_r() {
    let mut c = clean("ratio-zero");
    c.paired = true;
    c.blocks = 3;
    c.score = |b, arm, _| match (arm, b) {
        ("baseline", 1) => 0,
        ("baseline", _) => 1_000_000_000,
        _ => 2_000_000_000,
    };
    let g = grade(&c);
    assert_eq!(
        g.rows("ratio_recomputed")[0][1..],
        ["all", "pairs=2", "R=2.000000000", "UB95=2.000000000"]
    );
}

#[test]
fn stat_rows_and_r_take_interpolated_medians() {
    let mut c = clean("medians");
    c.blocks = 4;
    // q-all scores 1, 2, 3, 10 s: median 2.5, IQR 4.75 − 1.75 = 3 (linear interpolation).
    c.score = |b, _, q| {
        if q == "q-all" {
            [1, 2, 3, 10][b as usize - 1] * 1_000_000_000
        } else {
            1_500_000_000
        }
    };
    let g = grade(&c);
    assert_eq!(
        g.rows("stat")[0][1..],
        ["target", "q-all", "4", "2.500", "3.000", "10.000", "1.00"]
    );
    // Two pairs (1 s, 2 s) + (3 s, 4 s): R = median 3 / median 2; the bootstrap's resamples take
    // R ∈ {4/3, 3/2, 2} with weights ¼, ½, ¼ ⇒ its 95th percentile = 2.
    let mut c = clean("ratio-even");
    c.paired = true;
    c.score = |b, arm, _| match (arm, b) {
        ("baseline", 1) => 1_000_000_000,
        ("baseline", _) => 3_000_000_000,
        (_, 1) => 2_000_000_000,
        _ => 4_000_000_000,
    };
    let g = grade(&c);
    assert_eq!(
        g.rows("ratio_recomputed")[1][1..],
        ["accepted", "pairs=2", "R=1.500000000", "UB95=2.000000000"]
    );
}

#[test]
fn the_bootstrap_draws_every_pair_with_equal_chance() {
    // Eight accepted pairs, four 3× (target 3 s, baseline 1 s) + four 1×: R = median 2 / median 1.
    // A resample (8 draws) reaches R = 3 once ≥ 5 draws are 3× pairs: P = 93/256 ≈ 36 % under
    // uniform draws ⇒ UB95 = 3; draws biased to ¼ against the 3× half leave P ≈ 2.7 % ⇒ UB95 = 2.
    // The 3× pairs sit last, then first: a bias may favor either half.
    let late: fn(i64, &str, &str) -> u64 = |b, arm, _| {
        if arm == "target" && b >= 5 {
            3_000_000_000
        } else {
            1_000_000_000
        }
    };
    let early: fn(i64, &str, &str) -> u64 = |b, arm, _| {
        if arm == "target" && b <= 4 {
            3_000_000_000
        } else {
            1_000_000_000
        }
    };
    for (name, score) in [("bootstrap-late", late), ("bootstrap-early", early)] {
        let mut c = clean(name);
        c.paired = true;
        c.blocks = 8;
        c.score = score;
        let g = grade(&c);
        assert_eq!(
            g.rows("ratio_recomputed")[1][1..],
            ["accepted", "pairs=8", "R=2.000000000", "UB95=3.000000000"],
            "{name}"
        );
    }
}

// X1 reads the campaign's CPUs alone: shielded work on the host's other CPUs (≈ 0.9 CPU, as the
// user's own jobs) never rejects, and stays reported as host-wide foreign CPU (the last column).
#[test]
fn host_foreign_off_the_pinned_cpus_never_rejects() {
    let mut c = clean("x1-host");
    c.host_extra = |_, _| 0.9;
    c.foreign = |_, _| 0.2;
    let g = grade(&c);
    assert_eq!(g.blocks(), accepted(2), "{}", g.stdout);
    for row in g.rows("block") {
        assert_eq!((row[6], row[12]), ("0.200", "1.100"));
    }
}

// A sampler that predates the pinned column cannot assess X1: its blocks read unassessed, never
// accepted; X2 + X4 still reject on their own.
#[test]
fn a_sampler_without_the_pinned_column_leaves_x1_unassessed() {
    let mut c = clean("x1-unpinned");
    c.pinned_column = false;
    let g = grade(&c);
    assert_eq!(
        g.blocks(),
        vec![("unassessed".to_owned(), "X1-unpinned".to_owned()); 2]
    );
    assert_eq!(g.rows("block")[0][6], "NA");
    c.name = "x1-unpinned-x4";
    // No sample inside block 2 (whole seconds from T0 − 30 s): its bracket spans > 5 s.
    c.sampled = |c, t| !within(c, 2, t);
    let g = grade(&c);
    assert_eq!(
        g.blocks()[1],
        ("rejected".into(), "X4".into()),
        "{}",
        g.stdout
    );
}

// Contract sb P8: a run's memory bound grades each call (bound > 32 GB = uncertain = a budget
// fail, however small the sampled peak); a run without the bound column falls back to the HWM
// sum, each such call counted in `memory_unbounded`.
#[test]
fn the_memory_bound_grades_each_call_and_its_absence_is_counted() {
    let mut c = clean("bound");
    c.bound = |b, _, q| {
        Some(if b == 1 && q == "q-all" {
            32_000_000_001
        } else {
            32_000_000_000
        })
    };
    let g = grade(&c);
    assert_eq!(g.rc, 0, "{}", g.stdout);
    let calls = g.rows("call");
    assert_eq!(
        (calls[0][10], calls[0][11], calls[0][12]),
        ("fail:uncertain", "bound", "32.00")
    );
    assert!(calls[1..].iter().all(|r| r[10] == "ok" && r[11] == "bound"));
    let s = g.summary();
    assert_eq!(
        (&s["budget_fail"][..], &s["memory_unbounded"][..]),
        ("1", "0")
    );
    c.name = "bound-absent";
    c.bound_column = false;
    let g = grade(&c);
    assert!(
        g.rows("call")
            .iter()
            .all(|r| r[10] == "ok" && r[11] == "hwm-sum" && r[12] == "NA")
    );
    assert_eq!(&g.summary()["memory_unbounded"][..], "6");
}

// Contract sb § Carried rulings (preflight): `ckc bench preflight` over synthetic samples. Marks
// (seconds after P0): campaign-control 20–80, pinned-control 100–160, host-control 180–240,
// quiet 260 → 260 + quiet; samples every second, USER_HZ 100.
const P0: i64 = 1_900_000_000_000_000;
struct Preflight {
    name: &'static str,
    campaign: f64,         // the sampled unit's CPUs during campaign-control
    misattributed: bool,   // its burner runs on a pinned CPU outside the unit's accounting
    pinned_control: f64,   // foreign CPUs on 2 3 6 7 during pinned-control
    host_control: f64,     // foreign CPUs on the other CPUs during host-control
    background: f64,       // the shielded jobs' CPUs off the pinned set, throughout
    quiet: fn(i64) -> f64, // foreign CPUs on 2 3 6 7 at second s of the quiet stretch
    quiet_s: i64,
    pinned_column: bool,
}
fn calm(name: &'static str) -> Preflight {
    Preflight {
        name,
        campaign: 1.0,
        misattributed: false,
        pinned_control: 1.0,
        host_control: 1.0,
        background: 0.85,
        quiet: |_| 0.1,
        quiet_s: 600,
        pinned_column: true,
    }
}
fn preflight(p: &Preflight) -> Graded {
    let dir = root().join(format!(
        "rust/target/bench-grade/{}-pf-{}",
        std::process::id(),
        p.name
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let fixture = Fixture { dir: dir.clone() };
    let at = |s: i64| P0 + s * 1_000_000;
    let quiet_end = 260 + p.quiet_s;
    let mut marks = String::new();
    for (name, s, e) in [
        ("campaign-control", 20, 80),
        ("pinned-control", 100, 160),
        ("host-control", 180, 240),
        ("quiet", 260, quiet_end),
    ] {
        marks.push_str(&format!("{name}-start\t{}\n{name}-end\t{}\n", at(s), at(e)));
    }
    fs::write(dir.join("preflight.tsv"), marks).unwrap();
    fs::write(dir.join("host-before.txt"), host("performance", 77, true)).unwrap();
    let head = if p.pinned_column {
        "\tpinned_busy_jiffies"
    } else {
        ""
    };
    let mut rows = format!(
        "epoch_us\tbusy_jiffies\tidle_jiffies\tcampaign_cpu_us\tcampaign_swap_bytes\tpswpin\tpswpout\tcore_throttle\tpackage_throttle\tmax_temp_mc{head}\n"
    );
    let (mut busy, mut pinned, mut cpu) = (1_000_i64, 400_i64, 0_i64);
    for s in -5..=quiet_end + 5 {
        let cell = if p.pinned_column {
            format!("\t{pinned}")
        } else {
            String::new()
        };
        rows.push_str(&format!(
            "{}\t{busy}\t5000\t{cpu}\t0\t0\t0\t0\t0\t50000{cell}\n",
            at(s)
        ));
        // Second s → s + 1.
        let burning = (20..80).contains(&s);
        let own = if burning && !p.misattributed {
            p.campaign
        } else {
            0.0
        };
        let foreign = if burning && p.misattributed {
            p.campaign
        } else if (100..160).contains(&s) {
            p.pinned_control
        } else if (260..quiet_end).contains(&s) {
            (p.quiet)(s - 260)
        } else {
            0.0
        };
        let other = p.background
            + if (180..240).contains(&s) {
                p.host_control
            } else {
                0.0
            };
        let pinned_step = ((own + foreign) * 100.0).round() as i64;
        pinned += pinned_step;
        busy += pinned_step + (other * 100.0).round() as i64;
        cpu += (own * 1e6).round() as i64;
    }
    fs::write(dir.join("host-samples.tsv"), rows).unwrap();
    let out = Command::new(program())
        .args(["bench", "preflight"])
        .arg(&dir)
        .output()
        .unwrap();
    Graded {
        rc: out.status.code().unwrap(),
        stdout: String::from_utf8(out.stdout).unwrap(),
        _fixture: fixture,
    }
}
fn windows(g: &Graded) -> Vec<(String, String)> {
    g.rows("window")
        .iter()
        .map(|r| (r[1].to_owned(), r[8].to_owned()))
        .collect()
}
fn verdicts(bad: &[&str], quiet: usize) -> Vec<(String, String)> {
    ["campaign-control", "pinned-control", "host-control"]
        .into_iter()
        .map(str::to_owned)
        .chain((1..=quiet).map(|k| format!("quiet-{k}")))
        .map(|n| {
            let v = if bad.contains(&n.as_str()) {
                "bad"
            } else {
                "ok"
            };
            (n, v.to_owned())
        })
        .collect()
}

#[test]
fn a_calm_preflight_reads_its_controls_and_goes() {
    let g = preflight(&calm("calm"));
    assert_eq!(g.rc, 0, "{}", g.stdout);
    assert_eq!(windows(&g), verdicts(&[], 4));
    assert_eq!(
        g.rows("preflight")[0][1..],
        ["quiet_windows=4", "verdict=go"]
    );
}

// Each control's planted failure (an accounting that misses foreign work on the pinned CPUs, one
// that books the unit's own burner as foreign, a host-only burner the host column misses) and a
// busy or short quiet stretch hold the campaign; a sampler without the pinned column is no input.
#[test]
fn preflight_plants_hold() {
    let mut p = calm("pinned-blind");
    p.pinned_control = 0.0;
    let g = preflight(&p);
    assert_eq!((g.rc, windows(&g)), (1, verdicts(&["pinned-control"], 4)));
    let mut p = calm("misattributed");
    p.misattributed = true;
    let g = preflight(&p);
    assert_eq!((g.rc, windows(&g)), (1, verdicts(&["campaign-control"], 4)));
    let mut p = calm("host-blind");
    (p.host_control, p.background) = (0.0, 0.0);
    let g = preflight(&p);
    assert_eq!((g.rc, windows(&g)), (1, verdicts(&["host-control"], 4)));
    let mut p = calm("busy-quiet");
    // 0.6 CPU through the third window alone (quiet seconds 300–449).
    p.quiet = |s| if (300..450).contains(&s) { 0.6 } else { 0.1 };
    let g = preflight(&p);
    assert_eq!((g.rc, windows(&g)), (1, verdicts(&["quiet-3"], 4)));
    let mut p = calm("short-quiet");
    p.quiet_s = 450;
    let g = preflight(&p);
    assert_eq!((g.rc, windows(&g)), (1, verdicts(&[], 3)));
    assert_eq!(g.rows("preflight")[0][2], "verdict=hold");
    let mut p = calm("unpinned");
    p.pinned_column = false;
    assert_eq!(preflight(&p).rc, 2);
}

// A call that ends short of verified printed no output: its digest reads NA and it fails the
// budget, while the chunk stays graded evidence (rc 0) and its missing bound counts as no unbounded
// verified call.
#[test]
fn a_call_short_of_verified_fails_the_budget_without_a_wrong_output() {
    let mut c = clean("indeterminate");
    c.outcome = |b, _, q| {
        if b == 2 && q == "q-patient" {
            "indeterminate"
        } else {
            "verified"
        }
    };
    let g = grade(&c);
    assert_eq!(g.rc, 0, "{}", g.stdout);
    let calls = g.rows("call");
    let call = calls
        .iter()
        .find(|r| r[2] == "2" && r[4] == "q-patient")
        .unwrap();
    assert_eq!(
        (call[5], call[6], call[9], call[10]),
        ("indeterminate", "NA", "NA", "fail:ok")
    );
    let s = g.summary();
    assert_eq!(
        ["sha_bad", "budget_fail", "memory_unbounded"].map(|k| &s[k][..]),
        ["0", "1", "0"]
    );
}

// Contract sb § Carried rulings (decision tool, replacement round, R pairs, typed analysis at
// load): `ckc bench decide` over synthetic campaigns. A clean campaign goes once every measured
// knowledge identity joins its receipt, and holds while the census verdicts are pending.
#[test]
fn a_clean_campaign_goes_once_its_receipts_join() {
    let mut k = campaign("decide-clean");
    let g = decide(&k);
    assert_eq!((g.rc, g.verdict()), (0, "go"), "{}", g.stdout);
    assert!(g.holds().is_empty());
    let selected: Vec<&str> = g.rows("selected").iter().map(|r| r[3]).collect();
    assert_eq!(selected, ["r01", "r01", "c01", "c01"]);
    assert_eq!(
        g.rows("ratio")[0][1..],
        [
            "r01",
            "gate",
            "pairs=2",
            "R=1.000000000",
            "UB95=1.000000000"
        ]
    );
    let receipts = g.rows("receipt");
    let loads: Vec<(&str, &str)> = receipts.iter().map(|r| (r[1], r[2])).collect();
    let (target, baseline) = (kb_of(TARGET), kb_of(BASELINE));
    let mut want = vec![(target.as_str(), "2"), (baseline.as_str(), "1")];
    want.sort();
    assert_eq!(loads, want);
    assert!(receipts.iter().all(|r| r[3..] == ["1680.0", "26.00", "ok"]));
    k.receipts = None;
    let g = decide(&k);
    assert_eq!((g.rc, g.verdict()), (1, "hold"), "{}", g.stdout);
    assert_eq!(
        g.holds(),
        ["receipt join pending: census verdicts outstanding"]
    );
    assert_eq!(g.rows("receipt_join")[0][1], "pending");
}

// An original block that X1–X4 reject takes its replacement's attempt, whatever its outcomes (here a
// memory bound past 32 GB + an indeterminate call); without the replacement the chunk holds.
#[test]
fn replacements_fill_invalid_blocks_blind_to_their_outcomes() {
    let mut k = campaign("decide-replace");
    k.plan = plan_row("r01", true, 2, SEED, "gate") + &plan_row("c01", false, 3, SEED + 1, "-");
    let c01 = &mut k.attempts[1].1;
    c01.blocks = 3;
    c01.foreign = |c, t| if within(c, 2, t) { 0.8 } else { 0.0 };
    c01.bound = |b, _, q| {
        Some(if b == 2 && q == "q-all" {
            40_000_000_000
        } else {
            1_300_000_000
        })
    };
    c01.outcome = |b, _, q| {
        if b == 2 && q == "q-patient" {
            "indeterminate"
        } else {
            "verified"
        }
    };
    let g = decide(&k);
    assert_eq!(
        g.holds(),
        ["c01: block 2: no valid attempt"],
        "{}",
        g.stdout
    );
    let mut r = replacement(&k.attempts[1].1, &[2]);
    r.score = |_, _, _| 2_500_000_000;
    k.attempts.push(("c01-r", r));
    let g = decide(&k);
    assert_eq!((g.rc, g.verdict()), (0, "go"), "{}", g.stdout);
    let c01: Vec<&str> = g
        .rows("selected")
        .iter()
        .filter(|r| r[1] == "c01")
        .map(|r| r[3])
        .collect();
    assert_eq!(c01, ["c01", "c01-r", "c01"]);
    let blocks = g.rows("block");
    assert!(
        blocks
            .iter()
            .any(|r| r[1..] == ["c01", "c01", "2", "rejected", "invalid"])
    );
    assert!(
        blocks
            .iter()
            .any(|r| r[1..] == ["c01", "c01-r", "2", "accepted", "valid"])
    );
}

// A valid original block's budget failure stands, and a replacement may not touch a valid block.
#[test]
fn a_valid_original_budget_failure_stays_a_failure() {
    let mut k = campaign("decide-budget");
    k.attempts[1].1.bound = |b, _, q| {
        Some(if b == 1 && q == "q-all" {
            40_000_000_000
        } else {
            1_300_000_000
        })
    };
    let fail = "c01: block 1: target q-all verified outside the budget";
    let g = decide(&k);
    assert_eq!(g.holds(), [fail], "{}", g.stdout);
    let r = replacement(&k.attempts[1].1, &[1]);
    k.attempts.push(("c01-r", r));
    let g = decide(&k);
    assert_eq!(
        g.holds(),
        ["c01: attempt c01-r: replaces a valid block", fail],
        "{}",
        g.stdout
    );
}

// Every attempt is sound: settings, executables, harness rc, inputs + bounds as planned; a wrong
// output holds even in a block its replacement covers.
#[test]
fn every_attempt_must_be_sound() {
    type Plant = fn(&mut Chunk);
    let plants: [(&'static str, usize, Plant, &str); 9] = [
        (
            "decide-seed",
            1,
            |c| c.settings = |k, v| if k == "seed" { "7".into() } else { v },
            "c01: attempt c01: settings seed",
        ),
        (
            "decide-queries",
            1,
            |c| c.settings = |k, v| if k == "queries" { "q-one".into() } else { v },
            "c01: attempt c01: settings queries",
        ),
        (
            "decide-spacing",
            0,
            |c| c.settings = |k, v| if k == "spacing_ns" { "0".into() } else { v },
            "r01: attempt r01: settings spacing_ns",
        ),
        (
            "decide-only",
            1,
            |c| c.settings = |k, v| if k == "only_blocks" { "1".into() } else { v },
            "c01: attempt c01: settings only_blocks",
        ),
        (
            "decide-ckc",
            1,
            |c| c.harness = OTHER,
            "c01: attempt c01: ckc executable",
        ),
        (
            "decide-checker",
            0,
            |c| c.checker = OTHER,
            "r01: attempt r01: checker executable",
        ),
        (
            "decide-rc",
            1,
            |c| c.rc = "2",
            "c01: attempt c01: harness rc 2",
        ),
        (
            "decide-identity",
            1,
            |c| c.identity = |_, rel, _| rel.ends_with("b.pl").then(|| "0".repeat(64)),
            "c01: attempt c01: input identity",
        ),
        (
            "decide-unbounded",
            1,
            |c| c.bound = |b, _, q| (b != 2 || q != "q-one").then_some(1_300_000_000),
            "c01: attempt c01: 1 verified calls without a memory bound",
        ),
    ];
    for (name, index, plant, hold) in plants {
        let mut k = campaign(name);
        plant(&mut k.attempts[index].1);
        let g = decide(&k);
        assert_eq!((g.rc, g.verdict()), (1, "hold"), "{name}: {}", g.stdout);
        assert!(g.holds().contains(&hold), "{name}: {:?}", g.holds());
    }
    let mut k = campaign("decide-wrong");
    let c01 = &mut k.attempts[1].1;
    c01.foreign = |c, t| if within(c, 2, t) { 0.8 } else { 0.0 };
    c01.output = |b, _, q| (b == 2 && q == "q-patient").then(|| "f".repeat(64));
    let r = replacement(&k.attempts[1].1, &[2]);
    k.attempts.push(("c01-r", r));
    let g = decide(&k);
    assert_eq!(
        g.holds(),
        ["c01: attempt c01: 1 wrong output digests"],
        "{}",
        g.stdout
    );
}

// R gates on the plan's gating chunk alone: UB95 above 2 there holds; the same pairs on the
// reported chunk stand beside a passing gate.
#[test]
fn r_gates_on_its_gating_chunk_alone() {
    let triple: fn(i64, &str, &str) -> u64 = |_, arm, _| {
        if arm == "target" {
            3_000_000_000
        } else {
            1_000_000_000
        }
    };
    let mut k = campaign("decide-r");
    k.attempts[0].1.score = triple;
    let g = decide(&k);
    assert_eq!(
        g.holds(),
        ["r01: R upper bound 3.000 above 2"],
        "{}",
        g.stdout
    );
    assert_eq!(
        g.rows("ratio")[0][1..],
        [
            "r01",
            "gate",
            "pairs=2",
            "R=3.000000000",
            "UB95=3.000000000"
        ]
    );
    let mut k = campaign("decide-r-report");
    let mut r02 = clean("decide-r-report");
    (r02.paired, r02.seed, r02.score) = (true, SEED + 2, triple);
    k.attempts.push(("r02", r02));
    k.plan += &plan_row("r02", true, 2, SEED + 2, "report");
    let g = decide(&k);
    assert_eq!((g.rc, g.verdict()), (0, "go"), "{}", g.stdout);
    assert_eq!(
        g.rows("ratio")[1][1..],
        [
            "r02",
            "report",
            "pairs=2",
            "R=3.000000000",
            "UB95=3.000000000"
        ]
    );
}

// Each measured knowledge identity needs an admitted direct receipt of the named census binary on
// the named host, a full load ≤ 1,800 s + a charged memory ≤ 32 GB (both lines inclusive).
#[test]
fn the_receipt_join_fails_closed() {
    type Receipt = fn(&str) -> Option<String>;
    let plants: [(&'static str, Receipt, String); 9] = [
        ("receipt-missing", |_| None, "no receipt".into()),
        (
            "receipt-refused",
            |kb| Some(receipt_body(kb, &[("verdict", "refused")])),
            "verdict refused".into(),
        ),
        (
            "receipt-census",
            |kb| Some(receipt_body(kb, &[("ta_binary_sha256", OTHER)])),
            format!("ta_binary_sha256 {OTHER}"),
        ),
        (
            "receipt-host",
            |kb| Some(receipt_body(kb, &[("host", "workstation")])),
            "host workstation".into(),
        ),
        (
            "receipt-derived",
            |kb| Some(receipt_body(kb, &[("basis", "renaming")])),
            "derived (an alias carries admission alone)".into(),
        ),
        (
            "receipt-unmeasured",
            |kb| Some(receipt_body(kb, &[("analysis_seconds", "unavailable")])),
            "analysis_seconds unmeasured".into(),
        ),
        (
            "receipt-twice",
            |kb| Some(receipt_body(kb, &[]) + "verdict\tadmitted\n"),
            "malformed receipt".into(),
        ),
        (
            "receipt-slow",
            |kb| Some(receipt_body(kb, &[("analysis_seconds", "1620.5")])),
            "full load 1800.5 s above 1,800 s".into(),
        ),
        (
            "receipt-memory",
            |kb| Some(receipt_body(kb, &[("vmhwm_bytes", "26000000001")])),
            "charged memory 32.00 GB above 32 GB".into(),
        ),
    ];
    for (name, receipt, why) in plants {
        let mut k = campaign(name);
        k.receipts = Some(receipt);
        let g = decide(&k);
        assert_eq!((g.rc, g.verdict()), (1, "hold"), "{name}: {}", g.stdout);
        let mut want: Vec<String> = [TARGET, BASELINE]
            .map(|iid| format!("receipt {}: {why}", kb_of(iid)))
            .into();
        want.sort();
        assert_eq!(g.holds(), want, "{name}");
    }
    let mut k = campaign("receipt-lines");
    k.receipts = Some(|kb| {
        Some(receipt_body(
            kb,
            &[
                ("analysis_seconds", "1620.0"),
                ("vmhwm_bytes", "26000000000"),
            ],
        ))
    });
    let g = decide(&k);
    assert_eq!((g.rc, g.verdict()), (0, "go"), "{}", g.stdout);
    assert!(
        g.rows("receipt")
            .iter()
            .all(|r| r[3..] == ["1800.0", "32.00", "ok"])
    );
}

// Every runs/ directory is a planned attempt, and every planned chunk ran.
#[test]
fn every_attempt_directory_is_planned_and_every_chunk_ran() {
    let mut k = campaign("decide-unplanned");
    k.extra = &["c01-r2"];
    assert_eq!(decide(&k).holds(), ["unplanned attempt runs/c01-r2"]);
    let mut k = campaign("decide-missing");
    k.plan += &plan_row("c02", false, 2, SEED + 3, "-");
    assert_eq!(decide(&k).holds(), ["c02: no original attempt"]);
}
