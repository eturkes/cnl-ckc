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
const PRODUCER: &str = "9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d9d";
const SERVE: &str =
    "/pool/bin/checker serve --provider static --ta-receipts none --producer /pool/p";
// schedule.tsv's epoch (ns after the harness's origin): block b is planned at EPOCH + spacing × (b − 1).
const EPOCH_NS: i64 = 5_000_000_000;
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
    times: bool,                          // runs.tsv carries request_unix_ns + end_unix_ns
    attempted: fn(i64, &str, &str) -> bool, // a call the harness wrote (false = a lost server's later call)
    block_start: fn(&Chunk, i64) -> i64,    // block b's first request (µs)
    lifecycle: Option<u64>,                 // memory.tsv's summed-RSS peak (None = no memory.tsv)
    ready: (&'static str, &'static str),    // loads.tsv load_ns + rss_at_ready_bytes, every arm
    started: fn(i64) -> i64,                // schedule.tsv started − planned (ns) per block
    state_files: &'static [(&'static str, &'static str)], // state files replacing the policy ones
    host_keys: fn(&str, String) -> String,  // a planted host.tsv value per key
    producer: &'static str,                 // the producer identity file's digest
    runs_edit: fn(String) -> String,        // runs.tsv as written, then edited
    kb: fn(&str) -> String,                 // the knowledge identity a server reports per instance
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
        times: true,
        attempted: |_, _, _| true,
        block_start: |c, b| c.t0 + (b - 1) * BLOCK_US,
        lifecycle: Some(8_000_000_000),
        ready: ("120000000000", "6000000000"),
        started: |_| 1_000,
        state_files: &[],
        host_keys: |_, value| value,
        producer: PRODUCER,
        runs_edit: |runs| runs,
        kb: kb_of,
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
    let mut t = (c.block_start)(c, b);
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
        text.push_str("online: 0-7\nslice-cpus: 2-3,6-7\nclk_tck: 100\n");
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
            format!("kb_identity\t{}\nreceipt\tunchecked\n", (c.kb)(iid)),
        )
        .unwrap();
    }
    // The served checker's + the producer's identity files (`--identity-file`), hashed per arm.
    for (arm, _) in &arms {
        identities.push_str(&format!("{arm}\t{CHECKER_PATH}\t1000\t{}\n", c.checker));
        identities.push_str(&format!(
            "{arm}\t/pool/prototype/sb1/producer.prolog\t2000\t{}\n",
            c.producer
        ));
    }
    fs::write(run.join("identities.tsv"), identities).unwrap();

    let mut plan = "run\tblock\torder\tarm\tcomposition\tquery\n".to_owned();
    let mut runs = format!(
        "run\tblock\torder\tarm\toutcome\tscored_ns\tpeak_rss_bytes\tobserved_hwm_sum_bytes\toutput_sha256{}{}\n",
        if c.bound_column {
            "\tmemory_bound_bytes"
        } else {
            ""
        },
        if c.times {
            "\trequest_unix_ns\tend_unix_ns"
        } else {
            ""
        }
    );
    let mut n = 0;
    for b in 1..=c.blocks {
        for (k, (arm, query, start, end)) in calls(c, b).into_iter().enumerate() {
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
            let times = match (c.times, (c.attempted)(b, arm, query)) {
                (false, _) => String::new(),
                (true, true) => format!("\t{}\t{}", start * 1000, end * 1000),
                (true, false) => "\t\t".to_owned(),
            };
            runs.push_str(&format!(
                "{n}\t{b}\t{}\t{arm}\t{outcome}\t{score}\t{peak}\t{hwm}\t{output}{bound}{times}\n",
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
    fs::write(run.join("runs.tsv"), (c.runs_edit)(runs)).unwrap();
    let mut loads =
        "arm\tstart_ns\tready_ns\tload_ns\trss_at_ready_bytes\toutcome\tdetail\n".to_owned();
    for (arm, _) in &arms {
        loads.push_str(&format!(
            "{arm}\t0\t1\t{}\t{}\tready\t\n",
            c.ready.0, c.ready.1
        ));
    }
    fs::write(run.join("loads.tsv"), loads).unwrap();
    if let Some(peak) = c.lifecycle {
        fs::write(
            run.join("memory.tsv"),
            format!("key\tvalue\npeak_summed_rss_bytes\t{peak}\npeak_single_hwm_bytes\t1\nsampler_error\tnone\n"),
        )
        .unwrap();
    }
    let spacing: i64 = if c.paired { 30_000_000_000 } else { 0 };
    let held: Vec<i64> = if c.only.is_empty() {
        (1..=c.blocks).collect()
    } else {
        c.only.to_vec()
    };
    let mut schedule = "block\tplanned_ns\tstarted_ns\twaited_ns\n".to_owned();
    for b in held {
        let planned = EPOCH_NS + spacing * (b - 1);
        let started = planned + (c.started)(b);
        schedule.push_str(&format!("{b}\t{planned}\t{started}\t0\n"));
    }
    fs::write(run.join("schedule.tsv"), schedule).unwrap();
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
        ("format", "sb1-harness-3".to_owned()),
        ("sample_period_ns", "50000000".to_owned()),
        ("seed", c.seed.to_string()),
        ("blocks", c.blocks.to_string()),
        ("queries", queries.to_owned()),
        ("spacing_ns", spacing.to_owned()),
        ("only_blocks", only.join(",")),
        ("serve", SERVE.to_owned()),
    ] {
        settings.push_str(&format!("{key}\t{}\n", (c.settings)(key, value)));
    }
    fs::write(run.join("settings.tsv"), settings).unwrap();
    let mut host_keys = "key\tvalue\n".to_owned();
    for (key, value) in [
        ("harness_sha256", c.harness.to_owned()),
        ("OMP_NUM_THREADS", "1".to_owned()),
    ] {
        host_keys.push_str(&format!("{key}\t{}\n", (c.host_keys)(key, value)));
    }
    fs::write(run.join("host.tsv"), host_keys).unwrap();
    // The applied policy (the unit's CPU list from `c.cpus` below), then the planted replacements.
    for (file, body) in policy_files() {
        if file != "unit-cpus.txt" {
            fs::write(state.join(file), body).unwrap();
        }
    }
    for (file, body) in c.state_files {
        fs::write(state.join(file), body).unwrap();
    }

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
    // The frozen closure's rows (source, key, value) + the named preflight's planted state.
    closure: Vec<(&'static str, &'static str, String)>,
    preflight: fn(&mut Preflight),
}
fn closure_rows() -> Vec<(&'static str, &'static str, String)> {
    vec![
        ("settings", "format", "sb1-harness-3".into()),
        ("settings", "sample_period_ns", "50000000".into()),
        ("settings", "serve", SERVE.into()),
        ("host", "OMP_NUM_THREADS", "1".into()),
        ("identity", "prototype/sb1/producer.prolog", PRODUCER.into()),
        ("kb", TARGET, kb_of(TARGET)),
        ("kb", BASELINE, kb_of(BASELINE)),
        ("preflight", "pf", "go".into()),
    ]
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
        closure: closure_rows(),
        preflight: |_| {},
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
    let (out, fixture) = decide_run(k);
    graded(out, fixture)
}
fn decide_run(k: &Campaign) -> (std::process::Output, Fixture) {
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
    let closure = dir.join("closure.tsv");
    let rows: String = k
        .closure
        .iter()
        .map(|(source, key, value)| format!("{source}\t{key}\t{value}\n"))
        .collect();
    fs::write(&closure, format!("source\tkey\tvalue\n{rows}")).unwrap();
    let mut pf = calm("pf");
    (k.preflight)(&mut pf);
    preflight_files(&pf, &dir.join("state/pf"));
    let mut command = Command::new(program());
    command.args(["bench", "decide"]);
    command.args([&plan, &closure, &dir, &frozen, &manifest]);
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
    (command.output().unwrap(), fixture)
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
    // A block's interval = its first call's request write → its last call's end (or closed sink).
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
    // A run that recorded no frozen query proves no query identity: each planned query unrecorded
    // = bad (scientist-8 S8).
    let mut c = clean("identity-no-query");
    c.drop = |rel| rel.contains("/queries/");
    let g = grade(&c);
    assert_eq!(
        (g.rc, &g.rows("identity")[0][3..6]),
        (1, &["MISMATCH", "queries-checked=3", "bad=3"][..])
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

// A call without a closed sink counts in its block; its own request → end still bounds the block
// (scientist-8 S6). A run without the call-time columns falls back to closed sink − score, which
// that call lacks: never timed ⇒ the block is incomplete.
#[test]
fn a_call_without_a_closed_sink_counts_in_its_block() {
    let mut c = clean("unsunk");
    c.sink = |b, _, q| !(b == 2 && q == "q-patient");
    let g = grade(&c);
    let rows = g.rows("block");
    assert_eq!([rows[0][11], rows[1][11]], ["0", "1"]);
    assert_eq!(rows[1][3], calls(&c, 2)[2].3.to_string());
    assert_eq!((rows[1][4], rows[1][13]), ("accepted", "0"));
    c.times = false;
    let g = grade(&c);
    let rows = g.rows("block");
    assert_eq!(
        [rows[1][4], rows[1][5], rows[1][11], rows[1][13]],
        ["incomplete", "untimed", "1", "1"]
    );
    assert_eq!(rows[0][4], "accepted");
    assert_eq!(&g.summary()["incomplete"][..], "1");
}

// Contract sb § Carried rulings Replacement round (scientist-8 S6): a failed call's own time bounds
// its block — interference during it rejects the block, whatever the surviving calls saw — and a
// call the harness never wrote (a lost server's later call) leaves its block incomplete.
#[test]
fn a_failed_calls_time_stays_in_its_blocks_interval() {
    let mut c = clean("failed-head");
    c.outcome = |b, _, q| {
        if b == 2 && q == "q-all" {
            "indeterminate"
        } else {
            "verified"
        }
    };
    // 2 foreign CPUs through block 2's failed first call alone.
    c.foreign = |c, t| {
        let (_, _, s, e) = calls(c, 2)[0];
        if (s..e).contains(&t) { 2.0 } else { 0.0 }
    };
    let g = grade(&c);
    let rows = g.rows("block");
    assert_eq!(rows[1][2], calls(&c, 2)[0].2.to_string(), "{}", g.stdout);
    assert_eq!((rows[1][4], rows[1][5]), ("rejected", "X1"));
    let mut c = clean("never-written");
    c.outcome = |b, _, q| {
        if b == 2 && q != "q-all" {
            "indeterminate"
        } else {
            "verified"
        }
    };
    c.attempted = |b, _, q| !(b == 2 && q != "q-all");
    let g = grade(&c);
    let rows = g.rows("block");
    assert_eq!(
        [rows[1][4], rows[1][5], rows[1][13]],
        ["incomplete", "untimed", "2"]
    );
}

// Contract sb § Carried rulings Decision tool (scientist-8 S8): every planned query needs its own
// identity record, and runs.tsv holds each planned call once, as planned (rc 1 otherwise).
#[test]
fn every_planned_query_and_call_is_recorded_once() {
    let mut c = clean("no-q-all-identity");
    c.drop = |rel| rel.ends_with("q-all.txt");
    let g = grade(&c);
    assert_eq!(g.rc, 1, "{}", g.stdout);
    assert_eq!(
        (g.rows("identity")[0][3], g.rows("identity")[0][5]),
        ("MISMATCH", "bad=1")
    );
    for (name, edit) in [
        (
            "repeated-call",
            (|runs: String| {
                // run 2's row in place of run 1's (the required q-all).
                let lines: Vec<&str> = runs.lines().collect();
                let mut out = vec![lines[0], lines[2]];
                out.extend(&lines[2..]);
                out.join("\n") + "\n"
            }) as fn(String) -> String,
        ),
        ("extra-call", |runs| {
            runs + "2\t999\t1\ttarget\tindeterminate\t\t0\t0\t\t\t\t\n"
        }),
    ] {
        let mut c = clean(name);
        c.runs_edit = edit;
        let g = grade(&c);
        assert_eq!(g.rc, 1, "{name}: {}", g.stdout);
        assert_eq!(&g.summary()["runs_plan"][..], "bad", "{name}");
    }
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
// (seconds after P0): campaign-control 20–80, pinned-control 100–160, host-control 180–240 (each
// burner's own CPU time recorded beside its window), quiet 260 → 260 + quiet; samples every second,
// USER_HZ 100; the state files of an applied protocol (unit, wrapper + sampler CPUs, the shield's
// three records, the sleeper's marker) + an unchanged host record after.
const P0: i64 = 1_900_000_000_000_000;
struct Preflight {
    name: &'static str,
    spans: [(i64, i64); 3],  // the three control windows (s after P0)
    campaign: f64,           // the sampled unit's CPUs during campaign-control
    misattributed: bool,     // its burner runs on a pinned CPU outside the unit's accounting
    pinned_control: f64,     // foreign CPUs on 2 3 6 7 during pinned-control
    pinned_own: Option<f64>, // its burner's own CPUs (None = the `NA` of a failed unit)
    host_control: f64,       // foreign CPUs on the other CPUs during host-control
    host_own: Option<f64>,
    background: f64,       // the shielded jobs' CPUs off the pinned set, throughout
    quiet: fn(i64) -> f64, // foreign CPUs on 2 3 6 7 at second s of the quiet stretch
    quiet_s: i64,
    pinned_column: bool,
    after: Option<&'static str>, // the governor of the host record after (None = no record)
    files: Vec<(&'static str, &'static str)>, // state files replaced after the calm ones
}
fn calm(name: &'static str) -> Preflight {
    Preflight {
        name,
        spans: [(20, 80), (100, 160), (180, 240)],
        campaign: 1.0,
        misattributed: false,
        pinned_control: 1.0,
        pinned_own: Some(1.0),
        host_control: 3.0,
        host_own: Some(3.0),
        background: 0.85,
        quiet: |_| 0.1,
        quiet_s: 600,
        pinned_column: true,
        after: Some("performance"),
        files: Vec::new(),
    }
}
const SHIELDED: &str = "init.scope=0-1,4-5 system.slice=0-1,4-5 user.slice=0-1,4-5 ";
fn policy_files() -> Vec<(&'static str, String)> {
    vec![
        (
            "unit-cpus.txt",
            "CPUAffinity=2-3 6-7\nMainPID=4242\nCpus_allowed_list:\t2-3,6-7\n".into(),
        ),
        (
            "wrapper-cpus.txt",
            "wrapper Cpus_allowed_list:\t0-1,4-5\nsampler Cpus_allowed_list:\t0-1,4-5\n".into(),
        ),
        (
            "shield.txt",
            format!(
                "2026-11-06T00:00:00Z shielded: {SHIELDED}\n2026-11-06T00:15:00Z final: {SHIELDED}\n2026-11-06T00:15:01Z restored: init.scope=0-7 system.slice=0-7 user.slice=0-7 \n"
            ),
        ),
        ("sleeper.txt", "sleeper: 1900000000\n".into()),
    ]
}
fn preflight(p: &Preflight) -> Graded {
    let dir = root().join(format!(
        "rust/target/bench-grade/{}-pf-{}",
        std::process::id(),
        p.name
    ));
    let _ = fs::remove_dir_all(&dir);
    let fixture = Fixture { dir: dir.clone() };
    preflight_files(p, &dir);
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
// A preflight's state directory, as `tools/sb-campaign-unit preflight` writes it.
fn preflight_files(p: &Preflight, dir: &Path) {
    fs::create_dir_all(dir).unwrap();
    let at = |s: i64| P0 + s * 1_000_000;
    let quiet_end = 260 + p.quiet_s;
    let [campaign_span, pinned_span, host_span] = p.spans;
    let mut marks = String::new();
    for (name, (s, e), own) in [
        ("campaign-control", campaign_span, None),
        ("pinned-control", pinned_span, Some(p.pinned_own)),
        ("host-control", host_span, Some(p.host_own)),
        ("quiet", (260, quiet_end), None),
    ] {
        marks.push_str(&format!("{name}-start\t{}\n{name}-end\t{}\n", at(s), at(e)));
        if let Some(own) = own {
            let us = own.map_or("NA".into(), |o: f64| {
                ((o * (e - s) as f64) * 1e6).round().to_string()
            });
            marks.push_str(&format!("{name}-usage_us\t{us}\n"));
        }
    }
    fs::write(dir.join("preflight.tsv"), marks).unwrap();
    fs::write(dir.join("host-before.txt"), host("performance", 77, true)).unwrap();
    if let Some(governor) = p.after {
        fs::write(dir.join("host-after.txt"), host(governor, 81, true)).unwrap();
    }
    for (file, body) in policy_files() {
        fs::write(dir.join(file), body).unwrap();
    }
    for (file, body) in &p.files {
        fs::write(dir.join(file), body).unwrap();
    }
    let within = |(s, e): (i64, i64), t: i64| (s..e).contains(&t);
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
        let burning = within(campaign_span, s);
        let own = if burning && !p.misattributed {
            p.campaign
        } else {
            0.0
        };
        let foreign = if burning && p.misattributed {
            p.campaign
        } else if within(pinned_span, s) {
            p.pinned_control
        } else if (260..quiet_end).contains(&s) {
            (p.quiet)(s - 260)
        } else {
            0.0
        };
        let other = p.background
            + if within(host_span, s) {
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
}
// (window, verdict): the verdict = each row's last field.
fn windows(g: &Graded) -> Vec<(String, String)> {
    g.rows("window")
        .iter()
        .map(|r| (r[1].to_owned(), r[r.len() - 1].to_owned()))
        .collect()
}
fn verdict_line(g: &Graded) -> Vec<String> {
    g.rows("preflight")
        .first()
        .map(|r| r[1..].iter().map(|f| (*f).to_owned()).collect())
        .unwrap_or_default()
}
fn verdict_with(quiet: usize, order: &str, x3: &str, policy: &str, verdict: &str) -> Vec<String> {
    [
        format!("quiet_windows={quiet}"),
        format!("order={order}"),
        format!("x3={x3}"),
        format!("policy={policy}"),
        format!("verdict={verdict}"),
    ]
    .into()
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
// A plant's (name, rc, window verdicts, verdict line); a test compares every plant's reading at once,
// so a red run shows how the binary read each one.
type Reading = (String, i32, Vec<(String, String)>, Vec<String>);
fn reading(p: &Preflight) -> Reading {
    let g = preflight(p);
    (p.name.to_owned(), g.rc, windows(&g), verdict_line(&g))
}
fn held(p: &Preflight, bad: &[&str], quiet: usize, line: Vec<String>) -> Reading {
    (p.name.to_owned(), 1, verdicts(bad, quiet), line)
}

#[test]
fn a_calm_preflight_reads_its_controls_and_goes() {
    let g = preflight(&calm("calm"));
    assert_eq!(g.rc, 0, "{}", g.stdout);
    assert_eq!(windows(&g), verdicts(&[], 4));
    assert_eq!(verdict_line(&g), verdict_with(4, "ok", "same", "ok", "go"));
    let own: Vec<&str> = g.rows("window")[..3].iter().map(|r| r[7]).collect();
    assert_eq!(own, ["1.000", "1.000", "3.000"]);
}

// Each control's planted failure (an accounting that misses foreign work on the pinned CPUs, one
// that books the unit's own burner as foreign, host loops the host column misses while the shielded
// jobs keep their 0.85 CPU, a host burner that never ran under that background, a pinned burner
// without its own record or at half a CPU) and a busy or short quiet stretch hold the campaign; a
// sampler without the pinned column is no input.
#[test]
fn preflight_plants_hold() {
    let hold = |quiet| verdict_with(quiet, "ok", "same", "ok", "hold");
    let mut plants: Vec<(Preflight, &[&str], usize)> = Vec::new();
    let mut p = calm("pinned-blind");
    p.pinned_control = 0.0;
    plants.push((p, &["pinned-control"], 4));
    let mut p = calm("misattributed");
    p.misattributed = true;
    plants.push((p, &["campaign-control"], 4));
    let mut p = calm("host-blind");
    p.host_control = 0.0;
    plants.push((p, &["host-control"], 4));
    let mut p = calm("no-host-burner");
    (p.host_control, p.host_own) = (0.0, None);
    plants.push((p, &["host-control"], 4));
    let mut p = calm("pinned-no-record");
    p.pinned_own = None;
    plants.push((p, &["pinned-control"], 4));
    let mut p = calm("pinned-half");
    (p.pinned_control, p.pinned_own) = (0.5, Some(0.5));
    plants.push((p, &["pinned-control"], 4));
    let mut p = calm("busy-quiet");
    // 0.6 CPU through the third window alone (quiet seconds 300–449).
    p.quiet = |s| if (300..450).contains(&s) { 0.6 } else { 0.1 };
    plants.push((p, &["quiet-3"], 4));
    let mut p = calm("short-quiet");
    p.quiet_s = 450;
    plants.push((p, &[], 3));
    let got: Vec<Reading> = plants.iter().map(|(p, _, _)| reading(p)).collect();
    let want: Vec<Reading> = plants
        .iter()
        .map(|(p, bad, quiet)| held(p, bad, *quiet, hold(*quiet)))
        .collect();
    assert_eq!(got, want);
    let mut p = calm("unpinned");
    p.pinned_column = false;
    assert_eq!(preflight(&p).rc, 2);
}

// The protocol around the controls: 1 s controls (each burner's rate intact), a host control that
// overlaps the pinned one, a governor changed by the end, no host record after, and each policy item
// unapplied hold.
#[test]
fn preflight_protocol_plants_hold() {
    let mut plants: Vec<(Preflight, &[&str], Vec<String>)> = Vec::new();
    let mut p = calm("short-controls");
    p.spans = [(20, 21), (100, 101), (180, 181)];
    let three: &[&str] = &["campaign-control", "pinned-control", "host-control"];
    plants.push((p, three, verdict_with(4, "ok", "same", "ok", "hold")));
    let mut p = calm("overlap");
    p.spans[2] = (150, 210);
    plants.push((p, &[], verdict_with(4, "bad", "same", "ok", "hold")));
    let mut p = calm("governor");
    p.after = Some("powersave");
    plants.push((p, &[], verdict_with(4, "ok", "DIFFERS", "ok", "hold")));
    let mut p = calm("no-after");
    p.after = None;
    plants.push((p, &[], verdict_with(4, "ok", "missing", "ok", "hold")));
    let restored =
        "2026-11-06T00:15:01Z restored: init.scope=0-7 system.slice=0-7 user.slice=0-7 \n";
    for (name, file, body, item) in [
        ("sleeper-none", "sleeper.txt", "sleeper: none\n", "sleeper"),
        (
            "unit-unpinned",
            "unit-cpus.txt",
            "Cpus_allowed_list:\t0-7\n",
            "unit-cpus",
        ),
        (
            "wrapper-unpinned",
            "wrapper-cpus.txt",
            "wrapper Cpus_allowed_list:\t0-7\nsampler Cpus_allowed_list:\t0-1,4-5\n",
            "wrapper-cpus",
        ),
        (
            "never-shielded",
            "shield.txt",
            restored,
            "shield-shielded+shield-final",
        ),
        (
            "shield-lifted",
            "shield.txt",
            "x shielded: user.slice=0-1,4-5 \nx final: user.slice=0-7 \nx restored: user.slice=0-7 \n",
            "shield-final",
        ),
        (
            "no-restore",
            "shield.txt",
            "x shielded: user.slice=0-1,4-5 \nx final: user.slice=0-1,4-5 \n",
            "shield-restored",
        ),
    ] {
        let mut p = calm(name);
        p.files = vec![(file, body)];
        plants.push((p, &[], verdict_with(4, "ok", "same", item, "hold")));
    }
    let got: Vec<Reading> = plants.iter().map(|(p, _, _)| reading(p)).collect();
    let want: Vec<Reading> = plants
        .iter()
        .map(|(p, bad, line)| held(p, bad, 4, line.clone()))
        .collect();
    assert_eq!(got, want);
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

// Contract sb § Carried rulings Replacement round (scientist-8 S6-F2): a block whose every call
// failed on a calm host is a result — its calls' own times bound it, so it stays valid, selected +
// held, and a replacement may not take it over.
#[test]
fn an_all_failed_block_is_a_result_not_a_replacement_entitlement() {
    let mut k = campaign("decide-all-failed");
    let c01 = &mut k.attempts[1].1;
    c01.outcome = |b, _, _| if b == 2 { "indeterminate" } else { "verified" };
    c01.rc = "1";
    let r = replacement(&k.attempts[1].1, &[2]);
    k.attempts.push(("c01-r", r));
    let g = decide(&k);
    assert_eq!((g.rc, g.verdict()), (1, "hold"), "{}", g.stdout);
    let fail = |q: &str| format!("c01: block 2: target {q} indeterminate outside the budget");
    assert_eq!(
        g.holds(),
        [
            "c01: attempt c01-r: replaces a valid block".to_owned(),
            fail("q-all"),
            fail("q-one"),
            fail("q-patient")
        ],
        "{}",
        g.stdout
    );
}

// Contract sb § Carried rulings Decision tool (scientist-8 S7 + S8): every evidence gap holds — a
// call never timed, the lifecycle peak above 32 GB or unrecorded, an unmeasured ready load, unspaced
// or early R blocks, a closure drift (producer bytes, serve provider, RSS period, threads), an
// unapplied policy item, runs.tsv off plan.tsv, a planned query without its identity record.
#[test]
fn every_evidence_gap_holds() {
    type Plant = fn(&mut Chunk);
    let plants: [(&'static str, usize, Plant, &str); 14] = [
        (
            "decide-untimed",
            1,
            |c| {
                c.outcome = |b, _, q| {
                    if b == 2 && q != "q-all" {
                        "indeterminate"
                    } else {
                        "verified"
                    }
                };
                c.attempted = |b, _, q| !(b == 2 && q != "q-all");
            },
            "c01: attempt c01: protocol-incomplete: block 2 holds a call never timed",
        ),
        (
            "decide-lifecycle",
            1,
            |c| c.lifecycle = Some(40_000_000_000),
            "c01: attempt c01: lifecycle peak 40.00 GB above 32 GB",
        ),
        (
            "decide-no-memory",
            1,
            |c| c.lifecycle = None,
            "c01: attempt c01: lifecycle peak unrecorded",
        ),
        (
            "decide-ready",
            1,
            |c| c.ready = ("", ""),
            "c01: attempt c01: arm target: ready load unmeasured",
        ),
        (
            "decide-unspaced",
            0,
            |c| c.block_start = |c, b| c.t0 + (b - 1) * 3_500_000,
            "r01: attempt r01: block 2: first request 3.5 s after block 1's",
        ),
        (
            "decide-early",
            0,
            |c| c.started = |b| if b == 2 { -1 } else { 1_000 },
            "r01: attempt r01: schedule: block 2 started early",
        ),
        (
            "decide-producer",
            1,
            |c| c.producer = OTHER,
            "c01: attempt c01: closure identity prototype/sb1/producer.prolog",
        ),
        (
            "decide-provider",
            1,
            |c| {
                c.settings = |k, v| {
                    if k == "serve" {
                        v.replace("static", "all")
                    } else {
                        v
                    }
                }
            },
            "c01: attempt c01: closure settings serve",
        ),
        (
            "decide-period",
            1,
            |c| {
                c.settings = |k, v| {
                    if k == "sample_period_ns" {
                        "5000000000".into()
                    } else {
                        v
                    }
                }
            },
            "c01: attempt c01: closure settings sample_period_ns",
        ),
        (
            "decide-threads",
            1,
            |c| {
                c.host_keys = |k, v| {
                    if k == "OMP_NUM_THREADS" {
                        "8".into()
                    } else {
                        v
                    }
                }
            },
            "c01: attempt c01: closure host OMP_NUM_THREADS",
        ),
        (
            "decide-sleeper",
            1,
            |c| c.state_files = &[("sleeper.txt", "sleeper: none\n")],
            "c01: attempt c01: policy sleeper",
        ),
        (
            "decide-runs",
            1,
            |c| {
                c.runs_edit = |runs| {
                    let lines: Vec<&str> = runs.lines().collect();
                    let mut out = vec![lines[0], lines[2]];
                    out.extend(&lines[2..]);
                    out.join("\n") + "\n"
                }
            },
            "c01: attempt c01: runs.tsv ≠ plan.tsv (a planned call missing, repeated or moved)",
        ),
        (
            "decide-q-all",
            1,
            |c| c.drop = |rel| rel.ends_with("q-all.txt"),
            "c01: attempt c01: input identity",
        ),
        (
            "decide-kb",
            1,
            |c| c.kb = |_| "e".repeat(64),
            "c01: attempt c01: arm target: knowledge identity off the closure's",
        ),
    ];
    let mut red = Vec::new();
    for (name, index, plant, hold) in plants {
        let mut k = campaign(name);
        plant(&mut k.attempts[index].1);
        let g = decide(&k);
        if g.rc != 1 || !g.holds().contains(&hold) {
            red.push(format!("{name}: rc {} holds {:?}", g.rc, g.holds()));
        }
    }
    assert!(red.is_empty(), "{red:#?}");
}

// The closure's named preflight must go: a hold there holds the campaign; a closure naming none is
// no input (rc 2).
#[test]
fn the_named_preflight_must_go() {
    let mut k = campaign("decide-preflight");
    k.preflight = |p| p.host_control = 0.0;
    let g = decide(&k);
    assert_eq!(g.holds(), ["preflight pf: hold"], "{}", g.stdout);
    assert_eq!(
        g.rows("preflight")[0][1..3],
        ["pf", "quiet_windows=4"],
        "{}",
        g.stdout
    );
    assert_eq!(g.rows("preflight")[0].last(), Some(&"verdict=hold"));
    let mut k = campaign("decide-no-preflight");
    k.closure.retain(|r| r.0 != "preflight");
    let (out, _fixture) = decide_run(&k);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&out.stderr),
        "ckc: bench: decide: closure: no preflight row\n"
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
