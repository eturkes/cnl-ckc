// Contract sb D6 + § Carried rulings (affinity + interference exclusion, operationalized there):
// `ckc bench grade` over synthetic chunks. Every exclusion rule fires on its planted host record and
// its control passes; planted identity + output-digest mismatches refuse (rc 1); R over the accepted
// blocks drops a rejected block's pair. Expected identities come from the manifest framing written
// out here, independent of the grade's code.
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
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

#[derive(Clone)]
struct Chunk {
    name: &'static str,
    blocks: i64,
    paired: bool,
    foreign: fn(&Chunk, i64) -> f64,           // foreign CPUs at t
    swap: fn(&Chunk, i64) -> Option<i64>,      // the unit's swap bytes at t (None = NA)
    sampled: fn(&Chunk, i64) -> bool,          // a sample exists at t
    sampler: bool,                             // host-samples.tsv written
    after: (&'static str, u32, bool), // after-record governor, scaling MHz %, every key present
    cpus: Option<&'static str>,       // the unit's Cpus_allowed_list
    score: fn(i64, &str, &str) -> u64, // ns per (block, arm, query)
    memory: fn(i64, &str, &str) -> (u64, u64), // (peak RSS, HWM sum) per call
    output: fn(i64, &str, &str) -> Option<String>, // a planted output digest
    identity: fn(&str, &str, String) -> Option<String>, // a planted identities.tsv digest per path
    drop_row: Option<&'static str>,   // an identities.tsv path suffix left out
}
fn clean(name: &'static str) -> Chunk {
    Chunk {
        name,
        blocks: 2,
        paired: false,
        foreign: |_, _| 0.0,
        swap: |_, _| Some(0),
        sampled: |_, _| true,
        sampler: true,
        after: ("performance", 55, true),
        cpus: Some("2-3,6-7"),
        score: |_, _, _| 1_500_000_000,
        memory: |_, _, _| (1_000_000_000, 1_200_000_000),
        output: |_, _, _| None,
        identity: |_, _, _| None,
        drop_row: None,
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
    let mut t = T0 + (b - 1) * BLOCK_US;
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
}
fn grade(c: &Chunk) -> Graded {
    let dir = root().join(format!(
        "rust/target/bench-grade/{}-{}",
        std::process::id(),
        c.name
    ));
    let _ = fs::remove_dir_all(&dir);
    let fixture = Fixture { dir: dir.clone() };
    let (run, state, frozen) = (dir.join("run"), dir.join("state"), dir.join("frozen"));
    for d in [&run, &state] {
        fs::create_dir_all(d).unwrap();
    }
    let arms: Vec<(&str, &str)> = if c.paired {
        vec![("target", TARGET), ("baseline", BASELINE)]
    } else {
        vec![("target", TARGET)]
    };
    let schema = b"schema bytes\n";
    let mut manifest = "kind\tinstance\tname\tpath\tsha256\tbytes\tcount\tvalue\n".to_owned();
    let mut identities = "arm\tpath\tbytes\tsha256\n".to_owned();
    let mut record = |arm: &str, rel: &str, bytes: &[u8]| {
        if c.drop_row.is_some_and(|s| rel.ends_with(s)) {
            return;
        }
        let digest = (c.identity)(arm, rel, sha(bytes)).unwrap_or_else(|| sha(bytes));
        identities.push_str(&format!("{arm}\t/pool/{rel}\t{}\t{digest}\n", bytes.len()));
    };
    for (arm, iid) in &arms {
        let base = format!("rust/target/bench/sb1-frozen/{iid}");
        let lines: Vec<String> = ["a.pl", "b.pl"]
            .iter()
            .map(|f| format!("{base}/copies/0001/{f}"))
            .collect();
        fs::create_dir_all(frozen.join(iid)).unwrap();
        fs::write(
            frozen.join(iid).join("composition.tsv"),
            lines.join("\n") + "\n",
        )
        .unwrap();
        // The manifest's composition identity, framed as the run manifest defines it.
        let mut framed = format!("schema\0{}\0{}\n", schema.len(), sha(schema));
        record(arm, "guidelines/schema.pl", schema);
        for line in &lines {
            let bytes = format!("document {line}\n");
            framed.push_str(&format!(
                "file\0{line}\0{}\0{}\n",
                bytes.len(),
                sha(bytes.as_bytes())
            ));
            record(arm, line, bytes.as_bytes());
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
            record(arm, &format!("{base}/queries/{file}"), bytes.as_bytes());
            let want = sha(format!("output {iid} {q}\n").as_bytes());
            if *iid == TARGET {
                manifest.push_str(&format!("union\t{iid}\t{q}\t-\t{want}\t0\t0\t-\n"));
            } else {
                manifest.push_str(&format!("oracle\t{iid}\trecipe:{q}\t-\t{want}\t0\t0\t-\n"));
            }
        }
        fs::create_dir_all(run.join(format!("arm-{arm}"))).unwrap();
        fs::write(
            run.join(format!("arm-{arm}/load.tsv")),
            format!("kb_identity\tkb-{iid}\nreceipt\tunchecked\n"),
        )
        .unwrap();
    }
    fs::write(run.join("identities.tsv"), identities).unwrap();
    let manifest_path = dir.join("run-manifest.tsv");
    fs::write(&manifest_path, manifest).unwrap();

    let mut plan = "run\tblock\torder\tarm\tcomposition\tquery\n".to_owned();
    let mut runs = "run\tblock\torder\tarm\toutcome\tscored_ns\tpeak_rss_bytes\tobserved_hwm_sum_bytes\toutput_sha256\n".to_owned();
    let mut n = 0;
    for b in 1..=c.blocks {
        for (k, (arm, query, _, end)) in calls(c, b).into_iter().enumerate() {
            n += 1;
            let iid = if arm == "target" { TARGET } else { BASELINE };
            let base = format!("/pool/rust/target/bench/sb1-frozen/{iid}");
            plan.push_str(&format!(
                "{n}\t{b}\t{}\t{arm}\t{base}/composition.tsv\t{base}/queries/{}\n",
                k + 1,
                query_file(query)
            ));
            let output = (c.output)(b, arm, query)
                .unwrap_or_else(|| sha(format!("output {iid} {query}\n").as_bytes()));
            let (peak, hwm) = (c.memory)(b, arm, query);
            runs.push_str(&format!(
                "{n}\t{b}\t{}\t{arm}\tverified\t{}\t{peak}\t{hwm}\t{output}\n",
                k + 1,
                (c.score)(b, arm, query)
            ));
            let sink = run.join(format!("run-{n:06}"));
            fs::create_dir_all(&sink).unwrap();
            fs::write(
                sink.join("sink-complete.tsv"),
                format!("sb1-sink 1\nclosed_unix_ns\t{}\nbytes\t1\n", end * 1000),
            )
            .unwrap();
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
        fs::write(
            run.join("ratio.tsv"),
            format!("query\tpairs\tstatus\tratio_of_medians\tone_sided_95_upper\tbootstrap_resamples\tseed\nq-one\t{}\tcomplete\t2.000000000\t2.000000000\t10000\t{SEED}\n", c.blocks),
        )
        .unwrap();
    }

    fs::write(state.join("rc"), "0\n").unwrap();
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
        let mut rows = "epoch_us\tbusy_jiffies\tidle_jiffies\tcampaign_cpu_us\tcampaign_swap_bytes\tpswpin\tpswpout\tcore_throttle\tpackage_throttle\tmax_temp_mc\n".to_owned();
        let end = block_span(c, c.blocks).1 + 30_000_000;
        let (mut t, mut busy, mut cpu) = (T0 - 30_000_000, 1_000_i64, 0_i64);
        while t <= end {
            if (c.sampled)(c, t) {
                let swap = (c.swap)(c, t).map_or("NA".into(), |b| b.to_string());
                rows.push_str(&format!(
                    "{t}\t{busy}\t5000\t{cpu}\t{swap}\t0\t0\t0\t0\t50000\n"
                ));
            }
            // The next second: the campaign holds 2 CPUs, foreign work the rest (USER_HZ 100).
            busy += ((2.0 + (c.foreign)(c, t)) * 100.0).round() as i64;
            cpu += 2_000_000;
            t += 1_000_000;
        }
        fs::write(state.join("host-samples.tsv"), rows).unwrap();
    }
    let out = Command::new(program())
        .args(["bench", "grade"])
        .args([&run, &state, &frozen, &manifest_path])
        .output()
        .unwrap();
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
fn within(c: &Chunk, b: i64, t: i64) -> bool {
    let (s, e) = block_span(c, b);
    (s - 1_000_000..=e + 1_000_000).contains(&t)
}
fn accepted(n: usize) -> Vec<(String, String)> {
    vec![("accepted".to_owned(), "-".to_owned()); n]
}

#[test]
fn clean_chunk_matches_identity_and_accepts_every_block() {
    let g = grade(&clean("clean"));
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
    assert_eq!(g.rows("kb")[0][2], format!("kb-{TARGET}"));
}

#[test]
fn foreign_cpu_above_half_rejects_its_block_alone() {
    let mut c = clean("x1");
    // Block 1 carries 0.6 foreign CPUs, block 2 0.4: X1's line sits at 0.5.
    c.foreign = |c, t| {
        if within(c, 1, t) {
            0.6
        } else if within(c, 2, t) {
            0.4
        } else {
            0.0
        }
    };
    let g = grade(&c);
    assert_eq!(g.blocks()[0], ("rejected".into(), "X1".into()));
    assert_eq!(g.blocks()[1], ("accepted".into(), "-".into()));
    assert_eq!(g.rows("block")[0][6], "0.600");
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
    assert_eq!(
        grade(&c).blocks(),
        vec![("unassessed".to_owned(), "X2-unreadable".to_owned()); 2]
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
    c.drop_row = Some("copies/0001/a.pl");
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
fn budget_boundaries_hold_at_300_s_and_32_gb() {
    let mut c = clean("budget");
    c.blocks = 1;
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
    let mut c = clean("budget-over");
    c.blocks = 1;
    c.memory = |_, _, _| (32_000_000_001, 32_000_000_001);
    assert!(grade(&c).rows("call").iter().all(|r| r[10] == "fail:over"));
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
    assert_eq!(g.rows("ratio_harness")[0][1..3], ["q-one", "pairs=3"]);
    // The baseline arm's outputs grade against its recipe oracle rows.
    assert!(
        g.rows("call")
            .iter()
            .filter(|r| r[3] == "baseline")
            .all(|r| r[9] == "yes")
    );
}
