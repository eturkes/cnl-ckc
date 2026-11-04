// Contract sb Grading authorization: frozen inputs + independent oracle receipts.
mod contrast;
mod instances;
mod oracle;

use super::{collect_files, digest, manifest, read, relative, rooted, string};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const FAMILIES: [&str; 9] = ["F1a", "F1b", "F2", "F3", "F4", "F5a", "F5b", "F5c", "MX"];
const BASE: &str = "rust/target/bench/sb1-frozen";
const RECEIPTS: &str = "rust/target/bench/sb1-receipts";
const ORACLES: &str = "rust/target/bench/sb1-oracles";
const MANIFEST: &str = "bench/sb/annex/run-manifest.tsv";
const DRIVER: &str = "tools/sb-run-manifest-oracle.pl.in";
const HEADER: &str = "kind\tinstance\tname\tpath\tsha256\tbytes\tcount\tvalue\n";
const ANSWERS: [&str; 4] = ["model", "q-all", "q-one", "q-patient"];

struct Config {
    root: PathBuf,
    mode: String,
    family: Option<String>,
    seed: Option<u64>,
    baseline: Option<PathBuf>,
    file: PathBuf,
}
#[derive(Clone)]
struct Point {
    family: String,
    scale: usize,
    seed: u64,
    contrast: Option<&'static str>,
}
impl Point {
    fn id(&self) -> String {
        let base = format!("{}-g{:03}-s{}", self.family, self.scale, self.seed);
        match self.contrast {
            Some(c) => format!("{base}-{c}"),
            None => base,
        }
    }
    fn base(&self) -> Self {
        Self {
            contrast: None,
            ..self.clone()
        }
    }
    fn dir(&self, root: &Path) -> PathBuf {
        root.join(BASE).join(self.id())
    }
    fn receipt(&self, root: &Path) -> PathBuf {
        root.join(RECEIPTS).join(format!("{}.tsv", self.id()))
    }
    fn oracle(&self, root: &Path) -> PathBuf {
        root.join(ORACLES).join(self.id())
    }
}
// Contract sb D2: per family G1/2/4/10/100 seed 0 + the G100 seed-1 holdout, then its
// one-factor contrasts at G100 for both seeds (after their base: the contrast check reads it).
fn points(c: &Config) -> Vec<Point> {
    FAMILIES
        .into_iter()
        .filter(|f| c.family.as_deref().is_none_or(|selected| selected == *f))
        .flat_map(|family| {
            let point = move |scale, seed, contrast| Point {
                family: family.into(),
                scale,
                seed,
                contrast,
            };
            [1, 2, 4, 10, 100]
                .into_iter()
                .map(move |scale| point(scale, 0, None))
                .chain(std::iter::once(point(100, 1, None)))
                .chain(contrast::NAMES.into_iter().flat_map(move |(f, name)| {
                    (f == family)
                        .then(|| [point(100, 0, Some(name)), point(100, 1, Some(name))])
                        .into_iter()
                        .flatten()
                }))
        })
        .filter(|p| c.seed.is_none_or(|seed| p.seed == seed))
        .collect()
}
fn config(args: &[String]) -> Result<Config, String> {
    let mode = args.first().ok_or("run-manifest mode is required")?.clone();
    if !["instances", "oracles", "write", "check", "self-test"].contains(&mode.as_str())
        || !(args.len() - 1).is_multiple_of(2)
    {
        return Err("run-manifest <instances|oracles|write|check|self-test> --root <repo> [--family <family>] [--seed <0|1>] [--baseline <binary>] [--manifest <file>]".into());
    }
    let mut opts = BTreeMap::new();
    for pair in args[1..].as_chunks::<2>().0 {
        if !["--root", "--family", "--seed", "--baseline", "--manifest"].contains(&pair[0].as_str())
            || opts.insert(pair[0].as_str(), pair[1].as_str()).is_some()
        {
            return Err("run-manifest unknown or duplicate option".into());
        }
    }
    let root = PathBuf::from(*opts.get("--root").ok_or("--root is required")?)
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let family = opts.get("--family").map(|s| s.to_string());
    if family.as_deref().is_some_and(|f| !FAMILIES.contains(&f)) {
        return Err("run-manifest family".into());
    }
    let seed = opts
        .get("--seed")
        .map(|s| s.parse::<u64>().map_err(|_| "run-manifest seed"))
        .transpose()?;
    if seed.is_some_and(|s| s > 1) {
        return Err("run-manifest seed must be 0 or 1".into());
    }
    if ["write", "check", "self-test"].contains(&mode.as_str())
        && (family.is_some() || seed.is_some())
    {
        return Err("the run manifest grades the complete frozen inventory".into());
    }
    let baseline = opts.get("--baseline").map(|p| rooted(&root, p));
    let file = rooted(&root, opts.get("--manifest").copied().unwrap_or(MANIFEST));
    relative(&root, &file)?;
    Ok(Config {
        root,
        mode,
        family,
        seed,
        baseline,
        file,
    })
}
fn write(path: &Path, bytes: impl AsRef<[u8]>) -> Result<(), String> {
    fs::create_dir_all(path.parent().ok_or("output parent")?).map_err(|e| e.to_string())?;
    fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}
fn pairs(path: &Path, rows: &[(&str, String)]) -> Result<(), String> {
    write(
        path,
        rows.iter()
            .map(|(k, v)| format!("{k}\t{v}\n"))
            .collect::<String>(),
    )
}
fn tree(dir: &Path) -> Result<String, String> {
    let mut paths = Vec::new();
    collect_files(dir, dir, &mut paths)?;
    paths.sort();
    let mut listing = String::new();
    for p in paths {
        let bytes = read(&dir.join(&p))?;
        listing.push_str(&format!(
            "{}\t{}\t{}\n",
            digest(&bytes),
            bytes.len(),
            p.display()
        ));
    }
    Ok(listing)
}
fn equal(a: &Path, b: &Path) -> Result<(), String> {
    if read(a)? == read(b)? {
        Ok(())
    } else {
        Err(format!(
            "oracle bytes differ: {} / {}",
            a.display(),
            b.display()
        ))
    }
}
fn execute(command: &mut Command, log: &Path) -> Result<(), String> {
    let out = command
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("{command:?}: {e}"))?;
    write(&log.with_extension("stdout"), &out.stdout)?;
    write(&log.with_extension("stderr"), &out.stderr)?;
    if !out.status.success() || !out.stdout.is_empty() || !out.stderr.is_empty() {
        return Err(format!(
            "{command:?}: rc {:?}; stdout {} B; stderr {}",
            out.status.code(),
            out.stdout.len(),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(())
}
fn line_count(bytes: &[u8]) -> usize {
    bytes.iter().filter(|&&b| b == b'\n').count()
}
fn row(out: &mut String, fields: [&str; 8]) -> Result<(), String> {
    if fields
        .iter()
        .any(|s| s.is_empty() || s.contains(['\t', '\r', '\n', '\0']))
    {
        return Err("run-manifest field shape".into());
    }
    out.push_str(&fields.join("\t"));
    out.push('\n');
    Ok(())
}
fn file_row(
    out: &mut String,
    root: &Path,
    kind: &str,
    id: &str,
    name: &str,
    path: &Path,
    value: &str,
) -> Result<(), String> {
    let bytes = read(path)?;
    row(
        out,
        [
            kind,
            id,
            name,
            &relative(root, path)?,
            &digest(&bytes),
            &bytes.len().to_string(),
            &line_count(&bytes).to_string(),
            value,
        ],
    )
}
fn settings(out: &mut String, c: &Config) -> Result<(), String> {
    for (name, value) in [
        ("format", "sb1-run-manifest-1"),
        ("deadline_ms", "1800000"),
        ("scored_budget_ms", "300000"),
        ("memory_bytes", "48000000000"),
        ("pass_memory_bytes", "32000000000"),
        ("sample_ms", "50"),
        ("size_bytes", "8000000000"),
        ("swi_stack_limit", "32g"),
        ("swi_table_space", "32g"),
        ("swi_version", "10.0.2"),
        ("oracle", "independent-consumer-recipe+tnot"),
        ("harness", "ckc-bench-run;source-pinned"),
        ("ckc_version", env!("CARGO_PKG_VERSION")),
        ("oracle_version", "sb1-independent-recipe-1"),
        ("package_rerun", "tools/sb-run-manifest-rerun <repo>"),
        (
            "oracle_rerun",
            "tools/sb-run-manifest oracles --root <repo>",
        ),
    ] {
        row(out, ["setting", "-", name, "-", "-", "0", "0", value])?;
    }
    let mut paths = Vec::new();
    for dir in ["bench/sb", "rust/ckc/src/bench"] {
        let mut found = Vec::new();
        let base = c.root.join(dir);
        collect_files(&base, &base, &mut found)?;
        paths.extend(found.into_iter().map(|p| base.join(p)));
    }
    for path in [
        DRIVER,
        "tools/sb-run-manifest",
        "tools/sb-run-manifest-rerun",
        "rust/tools.lock",
        "rust/verus.lock",
        "rust/rust-toolchain.toml",
        "tests/consumer/driver.pl.in",
        "tests/consumer/wfs.pl.in",
        "docs/REFERENCE.md",
    ] {
        paths.push(c.root.join(path));
    }
    paths.sort();
    paths.dedup();
    for path in paths {
        if relative(&c.root, &path)? != MANIFEST {
            file_row(
                out,
                &c.root,
                "source",
                "-",
                &relative(&c.root, &path)?,
                &path,
                "frozen-input-or-tool",
            )?;
        }
    }
    Ok(())
}
fn inventory(c: &Config) -> Result<String, String> {
    let mut out = HEADER.to_owned();
    settings(&mut out, c)?;
    for point in points(c) {
        let dir = point.dir(&c.root);
        let id = point.id();
        instances::check(c, &point)?;
        let m = manifest(&dir.join("manifest.tsv"))?;
        let composition = string(&dir.join("composition.tsv"))?;
        let schema = read(&c.root.join("guidelines/schema.pl"))?;
        let mut identity = format!("schema\0{}\0{}\n", schema.len(), digest(&schema));
        let mut shards = BTreeMap::<usize, (usize, usize, String)>::new();
        let mut documents = 0;
        let mut bytes_total = 0;
        for listed in composition
            .lines()
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
        {
            let bytes = read(&c.root.join(listed))?;
            let clauses = std::str::from_utf8(&bytes)
                .map_err(|e| e.to_string())?
                .lines()
                .filter(|s| !s.trim().is_empty() && !s.starts_with('%') && !s.starts_with(":-"))
                .count();
            let frame = format!("file\0{listed}\0{}\0{}\n", bytes.len(), digest(&bytes));
            identity.push_str(&frame);
            documents += 1;
            bytes_total += bytes.len();
            if let Some(rest) = listed.split("/copies/").nth(1) {
                let slot: usize = rest
                    .split('/')
                    .next()
                    .ok_or("shard path")?
                    .parse()
                    .map_err(|_| "shard slot")?;
                let entry = shards.entry(slot).or_default();
                entry.0 += clauses;
                entry.1 += bytes.len();
                entry.2.push_str(&frame);
            }
        }
        let composition_digest = digest(identity.as_bytes());
        if m.get("composition_sha256") != Some(&composition_digest) || shards.len() != point.scale {
            return Err(format!(
                "{id}: composition identity or shard inventory differs"
            ));
        }
        // Seed 1 = a representation/order holdout of the same clinical workload
        // (scientist-3 G7), not a fresh patient or topology.
        let mut value = format!(
            "family={};G={};seed={};role={}",
            point.family,
            point.scale,
            point.seed,
            if point.seed == 0 {
                "development"
            } else {
                "holdout-representation"
            }
        );
        if let Some(contrast) = point.contrast {
            value.push_str(&format!(";contrast={contrast};base={}", point.base().id()));
        }
        row(
            &mut out,
            [
                "instance",
                &id,
                "composition",
                &relative(&c.root, &dir.join("composition.tsv"))?,
                &composition_digest,
                &bytes_total.to_string(),
                &documents.to_string(),
                &value,
            ],
        )?;
        file_row(
            &mut out,
            &c.root,
            "generation",
            &id,
            "receipt",
            &point.receipt(&c.root),
            "two-byte-identical-generations",
        )?;
        file_row(
            &mut out,
            &c.root,
            "generation",
            &id,
            "manifest",
            &dir.join("manifest.tsv"),
            "input-digests+variation",
        )?;
        for (slot, (clauses, bytes, framed)) in shards {
            let key = format!("{slot:04}");
            let role = &m[&format!("shard:{slot:04}:role")];
            row(
                &mut out,
                [
                    "shard",
                    &id,
                    &key,
                    &relative(&c.root, &dir.join(format!("copies/{slot:04}")))?,
                    &digest(framed.as_bytes()),
                    &bytes.to_string(),
                    &clauses.to_string(),
                    role,
                ],
            )?;
        }
        for name in super::QUERY_NAMES {
            file_row(
                &mut out,
                &c.root,
                "query",
                &id,
                name,
                &dir.join("queries").join(name),
                "frozen-query",
            )?;
        }
        if point.scale == 100 {
            oracle::check(c, &point)?;
            for mode in ["recipe", "wfs"] {
                for answer in ANSWERS {
                    file_row(
                        &mut out,
                        &c.root,
                        "oracle",
                        &id,
                        &format!("{mode}:{answer}"),
                        &point
                            .oracle(&c.root)
                            .join(mode)
                            .join(format!("{answer}.out")),
                        "exact;undefined=0",
                    )?;
                }
                file_row(
                    &mut out,
                    &c.root,
                    "oracle",
                    &id,
                    &format!("{mode}:receipt"),
                    &point.oracle(&c.root).join(mode).join("receipt.tsv"),
                    "rc=0;stdout=0;stderr=0",
                )?;
            }
            if ["F1a", "F1b", "F2"].contains(&point.family.as_str()) {
                for answer in ANSWERS {
                    file_row(
                        &mut out,
                        &c.root,
                        "union",
                        &id,
                        answer,
                        &point
                            .oracle(&c.root)
                            .join("union")
                            .join(format!("{answer}.out")),
                        "100-isolated-contributions=whole",
                    )?;
                }
                file_row(
                    &mut out,
                    &c.root,
                    "union",
                    &id,
                    "receipt",
                    &point.oracle(&c.root).join("union/receipt.tsv"),
                    "copy-by-copy-model-union",
                )?;
            }
            if point.contrast.is_some() {
                for name in ["receipt.tsv", "removed.out", "added.out"] {
                    file_row(
                        &mut out,
                        &c.root,
                        "contrast",
                        &id,
                        name,
                        &point.oracle(&c.root).join("contrast").join(name),
                        "delta-vs-base-recipe-model",
                    )?;
                }
            }
        }
    }
    Ok(out)
}
fn counts(c: &Config) -> String {
    let all = points(c);
    let contrasts = all.iter().filter(|p| p.contrast.is_some()).count();
    let endpoints = all.iter().filter(|p| p.scale == 100).count();
    format!(
        "{} instances ({contrasts} contrasts), {endpoints} endpoint oracles",
        all.len()
    )
}
fn validate(want: &str, got: &str) -> Result<(), String> {
    if want == got {
        return Ok(());
    }
    let line = want
        .lines()
        .zip(got.lines())
        .position(|(a, b)| a != b)
        .map_or(want.lines().count().min(got.lines().count()) + 1, |i| i + 1);
    Err(format!("run-manifest differs at row {line}"))
}
pub(super) fn run(args: &[String]) -> Result<(), String> {
    let c = config(args)?;
    match c.mode.as_str() {
        "instances" => instances::run(&c),
        "oracles" => oracle::run(&c),
        "write" => {
            let text = inventory(&c)?;
            write(&c.file, text)?;
            println!("sb-run-manifest: frozen {}", counts(&c));
            Ok(())
        }
        "check" => {
            validate(&inventory(&c)?, &string(&c.file)?)?;
            println!("sb-run-manifest: ok {}", counts(&c));
            Ok(())
        }
        "self-test" => {
            let want = inventory(&c)?;
            let got = string(&c.file)?;
            validate(&want, &got)?;
            let mut lines: Vec<_> = got.lines().map(str::to_owned).collect();
            let line = lines
                .iter_mut()
                .find(|s| s.starts_with("oracle\t") && s.contains(":q-all\t"))
                .ok_or("oracle digest plant anchor")?;
            let mut fields: Vec<_> = line.split('\t').map(str::to_owned).collect();
            let first = if fields[4].starts_with('0') { "1" } else { "0" };
            fields[4].replace_range(..1, first);
            *line = fields.join("\t");
            let plant = lines.join("\n") + "\n";
            if validate(&want, &plant).is_ok() {
                return Err("run-manifest wrong digest passed".into());
            }
            println!(
                "sb-run-manifest: positive control passed; planted wrong oracle digest refused"
            );
            Ok(())
        }
        _ => unreachable!(),
    }
}
