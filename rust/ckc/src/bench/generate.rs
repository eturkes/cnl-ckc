// Contract sb D4/P1: position-aware family generation + replayable transform audit.
mod isolation;
mod text;
mod transform;

use self::text::Document;
use self::transform::Rename;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, ExitCode};

const VERSION: &str = "sb1-rename-1";
const QUERY_NAMES: [&str; 3] = ["q-all.pl", "q-one.pl", "q-patient.pl"];

#[derive(Clone)]
struct Options {
    root: PathBuf,
    family: String,
    scale: usize,
    documents: PathBuf,
    patient: PathBuf,
    annex: PathBuf,
    queries: PathBuf,
    out: PathBuf,
    expected: Option<PathBuf>,
    seed: u64,
    commit: Option<String>,
}
fn read(path: &Path) -> Result<Vec<u8>, String> {
    if !fs::metadata(path).is_ok_and(|m| m.is_file()) {
        return Err(format!("not a regular file: {}", path.display()));
    }
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}
fn string(path: &Path) -> Result<String, String> {
    String::from_utf8(read(path)?).map_err(|e| format!("{}: {e}", path.display()))
}
fn digest(bytes: &[u8]) -> String {
    crate::trust::sha256_hex(bytes)
}
fn rooted(root: &Path, path: &str) -> PathBuf {
    if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        root.join(path)
    }
}
fn relative(root: &Path, path: &Path) -> Result<String, String> {
    let rel = path
        .strip_prefix(root)
        .map_err(|_| format!("path is outside --root: {}", path.display()))?;
    let text = rel.to_str().ok_or("non-UTF-8 path")?;
    if text.bytes().any(|b| b"\t\n\r\0".contains(&b))
        || rel.components().any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err("path cannot be represented in the composition".into());
    }
    Ok(text.to_owned())
}
fn resolve_input(root: &Path, path: &str) -> Result<PathBuf, String> {
    let p = rooted(root, path)
        .canonicalize()
        .map_err(|e| format!("input {path}: {e}"))?;
    relative(root, &p)?;
    Ok(p)
}
fn output_path(root: &Path, path: &str) -> Result<PathBuf, String> {
    let p = rooted(root, path);
    if p.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err("output path contains a parent component".into());
    }
    let base = root.join("rust/target/bench");
    let suffix = p
        .strip_prefix(&base)
        .map_err(|_| "output must be under rust/target/bench")?;
    if suffix.as_os_str().is_empty() {
        return Err("output must name a composition directory".into());
    }
    let mut ancestor = p.as_path();
    while !ancestor.exists() {
        ancestor = ancestor.parent().ok_or("output has no existing ancestor")?;
    }
    let physical = ancestor.canonicalize().map_err(|e| e.to_string())?;
    if physical != ancestor {
        return Err("output path traverses a symlink".into());
    }
    relative(root, &p)?;
    Ok(p)
}
fn pairs(args: &[String]) -> Result<BTreeMap<String, String>, String> {
    if !args.len().is_multiple_of(2) {
        return Err("each option needs a value".into());
    }
    let mut out = BTreeMap::new();
    for p in args.as_chunks::<2>().0 {
        if !matches!(
            p[0].as_str(),
            "--root"
                | "--documents"
                | "--patient"
                | "--annex"
                | "--queries"
                | "--out"
                | "--expected"
                | "--seed"
        ) {
            return Err(format!("unknown option {}", p[0]));
        }
        out.insert(p[0].clone(), p[1].clone());
    }
    Ok(out)
}
fn options(args: &[String]) -> Result<Options, String> {
    if args.len() < 2 {
        return Err("usage".into());
    }
    let opts = pairs(&args[2..])?;
    let root = PathBuf::from(opts.get("--root").ok_or("--root is required")?)
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let family = args[0].clone();
    let scale = args[1].parse().map_err(|_| "scale must be an integer")?;
    let source = |flag: &str, default: &str| {
        resolve_input(&root, opts.get(flag).map(String::as_str).unwrap_or(default))
    };
    Ok(Options {
        documents: source("--documents", "guidelines/cdc-2022-opioid/pl")?,
        patient: resolve_input(&root, opts.get("--patient").ok_or("--patient is required")?)?,
        annex: source("--annex", "bench/sb/annex")?,
        queries: source("--queries", "bench/sb/queries")
            .map_err(|e| format!("query input: {e}"))?,
        out: output_path(&root, opts.get("--out").ok_or("--out is required")?)?,
        expected: opts
            .get("--expected")
            .map(|p| resolve_input(&root, p))
            .transpose()?,
        seed: opts
            .get("--seed")
            .map(|s| s.parse().map_err(|_| "seed must be a u64"))
            .transpose()?
            .unwrap_or(0),
        root,
        family,
        scale,
        commit: None,
    })
}
struct Calibration {
    profile: String,
    active: usize,
    rows: Vec<(String, String)>,
    inputs: Vec<PathBuf>,
}
fn table(path: &Path, columns: usize) -> Result<Vec<Vec<String>>, String> {
    let text = string(path)?;
    let mut rows = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let fields: Vec<String> = line.split('\t').map(str::to_owned).collect();
        if fields.len() != columns || fields.iter().any(String::is_empty) {
            return Err(format!("annex {} row {} shape", path.display(), index + 1));
        }
        rows.push(fields);
    }
    if rows.len() < 2 {
        return Err(format!("annex {} has no parameters", path.display()));
    }
    Ok(rows)
}
fn calibration(o: &Options) -> Result<Calibration, String> {
    if !matches!(o.family.as_str(), "F1a" | "F1b" | "F2") {
        return Err(
            "family requires a frozen fixture composition; supported core families: F1a F1b F2"
                .into(),
        );
    }
    if ![1, 2, 4, 10, 100].contains(&o.scale) {
        return Err("scale is not a calibrated nested subset (1,2,4,10,100)".into());
    }
    let families_path = o.annex.join("families.tsv");
    let sources_path = o.annex.join("sources.tsv");
    let families = table(&families_path, 9)?;
    let sources = table(&sources_path, 14)?;
    if families[0].join("\t")
        != "family\tG\tk\tactive_documents\tdormant_kind\tpatient_profile\tmechanism\tnested_subsets\tparam_ids"
        || sources[0].join("\t")
            != "param_id\tfamily\tparameter\tchosen_value\tderivation\tsource_id\tlocator\tsource_sha256\texcerpt_file\tpopulation\twindow\tunit\tevidence_class\tstatus"
    {
        return Err("annex header".into());
    }
    let matches: Vec<_> = families[1..].iter().filter(|r| r[0] == o.family).collect();
    if matches.len() != 1 {
        return Err("annex family row absent or duplicated".into());
    }
    let family = matches[0];
    let mut ids = BTreeMap::new();
    for row in &sources[1..] {
        if ids.insert(row[0].clone(), row).is_some() {
            return Err(format!("annex duplicate parameter {}", row[0]));
        }
    }
    let mut rows = Vec::new();
    let mut inputs = BTreeSet::from([families_path, sources_path]);
    let mut parameters = BTreeMap::new();
    for id in family[8].split(',').map(str::trim) {
        let row = ids
            .get(id)
            .ok_or_else(|| format!("annex missing parameter {id}"))?;
        if row[7].len() != 64
            || !row[7].bytes().all(|b| b.is_ascii_hexdigit())
            || !matches!(
                row[12].as_str(),
                "clinical_data" | "capacity_target" | "engineering_setting"
            )
            || row[11].trim().is_empty()
        {
            return Err(format!("annex parameter {id} units/evidence/digest"));
        }
        parameters.insert(row[2].as_str(), row[3].as_str());
        rows.push((
            format!("annex_param:{id}"),
            digest(row.join("\t").as_bytes()),
        ));
        let excerpt = resolve_input(&o.root, &relative(&o.root, &o.annex.join(&row[8]))?)?;
        inputs.insert(excerpt);
    }
    for (i, field) in [
        "G",
        "k",
        "active_documents",
        "dormant_kind",
        "patient_profile",
        "mechanism",
        "nested_subsets",
    ]
    .iter()
    .enumerate()
    {
        if parameters.get(field).copied() != Some(family[i + 1].as_str()) {
            return Err(format!("annex family parameter mismatch {field}"));
        }
    }
    let endpoint: usize = family[1].parse().map_err(|_| "annex G is not a count")?;
    let active: usize = family[2].parse().map_err(|_| "annex k is not a count")?;
    if endpoint != 100 || active == 0 || active > endpoint {
        return Err("annex G/k range".into());
    }
    Ok(Calibration {
        profile: family[5].split(';').next().unwrap_or(&family[5]).to_owned(),
        active: active.min(o.scale),
        rows,
        inputs: inputs.into_iter().collect(),
    })
}
fn document_paths(o: &Options) -> Result<Vec<PathBuf>, String> {
    let mut paths = if o.documents.is_dir() {
        fs::read_dir(&o.documents)
            .map_err(|e| e.to_string())?
            .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.ends_with(".pl") || n.ends_with(".pl.in"))
            })
            .collect::<Vec<_>>()
    } else if o.documents.extension().is_some_and(|e| e == "tsv") {
        string(&o.documents)?
            .lines()
            .filter(|l| !l.starts_with('#') && !l.is_empty())
            .map(|l| resolve_input(&o.root, l))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        vec![o.documents.clone()]
    };
    paths.sort();
    if paths.is_empty() || paths.windows(2).any(|p| p[0] == p[1]) {
        return Err("documents are empty or duplicated".into());
    }
    for p in &mut paths {
        *p = resolve_input(&o.root, &relative(&o.root, p)?)?;
    }
    Ok(paths)
}
fn load(path: &Path) -> Result<Document, String> {
    Document::parse(&read(path)?).map_err(|e| format!("{}: {e}", path.display()))
}
fn query_path(o: &Options, name: &str) -> Result<PathBuf, String> {
    let p = o.queries.join(name);
    let p = if p.is_file() {
        p
    } else {
        o.queries.join(format!("{name}.in"))
    };
    resolve_input(&o.root, &relative(&o.root, &p)?).map_err(|e| format!("query {name}: {e}"))
}
fn renamer(o: &Options, index: usize, produced: &BTreeSet<String>, k: usize) -> Rename {
    if o.family == "F2" && index <= k {
        Rename::active(index, produced)
    } else if index == 1 && o.family != "F2" {
        Rename::identity()
    } else {
        Rename::dormant(index, o.family != "F1b")
    }
}
fn git_commit(o: &Options) -> Result<String, String> {
    if let Some(commit) = &o.commit {
        return Ok(commit.clone());
    }
    let out = Command::new("git")
        .arg("-C")
        .arg(&o.root)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| e.to_string())?;
    let value = String::from_utf8(out.stdout).map_err(|e| e.to_string())?;
    if !out.status.success()
        || value.trim().len() != 40
        || !value.trim().bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("generator commit is unavailable".into());
    }
    Ok(value.trim().to_owned())
}
struct Product {
    files: BTreeMap<PathBuf, Vec<u8>>,
}
fn build(o: &Options) -> Result<Product, String> {
    let calibration = calibration(o)?;
    let paths = document_paths(o)?;
    let documents = paths
        .iter()
        .map(|p| load(p))
        .collect::<Result<Vec<_>, _>>()?;
    let patient = load(&o.patient)?;
    if patient.clauses().any(|c| {
        let mut variables = BTreeMap::new();
        c.head.variables(&mut variables);
        !c.body.is_empty() || !variables.is_empty()
    }) {
        return Err("patient must contain ground read-only facts, not rules".into());
    }
    let mut docids = BTreeSet::new();
    for doc in documents.iter().chain(std::iter::once(&patient)) {
        let id = doc
            .clauses()
            .filter_map(|c| c.head.call())
            .find(|(n, _)| *n == "guideline_document")
            .and_then(|(_, a)| a.first().and_then(text::Term::atom))
            .ok_or("input is not a document")?;
        if !docids.insert(id) {
            return Err(format!("transform duplicate document identity {id}"));
        }
    }
    transform::reject_namespace_collision(&documents, &patient)?;
    let produced = transform::head_produced(&documents);
    if o.family == "F2"
        && documents
            .iter()
            .flat_map(Document::clauses)
            .filter_map(|c| c.head.call())
            .any(|(name, args)| {
                matches!(
                    name,
                    "guideline_entity" | "guideline_event" | "guideline_property"
                ) && args.get(2).is_some_and(|t| t.atom().is_none())
            })
    {
        return Err(
            "isolation positive variable lexical head requires a closed private spelling range"
                .into(),
        );
    }
    let mut files = BTreeMap::new();
    let mut manifest = vec![
        ("family".into(), o.family.clone()),
        ("scale".into(), o.scale.to_string()),
        ("G".into(), o.scale.to_string()),
        ("k".into(), calibration.active.to_string()),
        ("transform_version".into(), VERSION.into()),
        ("generator_commit".into(), git_commit(o)?),
        ("seed".into(), o.seed.to_string()),
        ("patient_profile".into(), calibration.profile),
        (
            "mode".into(),
            if o.expected.is_some() {
                "oracle-pinned"
            } else {
                "construction"
            }
            .into(),
        ),
    ];
    for (key, path) in [
        ("source_documents", &o.documents),
        ("source_patient", &o.patient),
        ("source_annex", &o.annex),
        ("source_queries", &o.queries),
    ] {
        manifest.push((key.into(), relative(&o.root, path)?));
    }
    let mut inputs = calibration.inputs;
    inputs.extend(paths.clone());
    inputs.push(o.patient.clone());
    if o.documents.is_file() {
        inputs.push(o.documents.clone());
    }
    let expected_hash = match &o.expected {
        Some(path) => {
            inputs.push(path.clone());
            manifest.push(("source_expected".into(), relative(&o.root, path)?));
            digest(&read(path)?)
        }
        None => "pending".into(),
    };
    manifest.push(("expected_output_sha256:q-all".into(), expected_hash));
    manifest.extend(calibration.rows);
    let mut patient_documents = vec![patient.clone()];
    let mut representatives = Vec::new();
    let mut roles = BTreeMap::<String, usize>::new();
    let mut composition = String::new();
    for index in 1..=o.scale {
        let rename = renamer(o, index, &produced, calibration.active);
        let shard = documents
            .iter()
            .map(|d| rename.apply(d))
            .collect::<Result<Vec<_>, _>>()?;
        let bytes: Vec<u8> = shard.iter().flat_map(Document::render).collect();
        let path = PathBuf::from(format!("copies/{index:04}.pl"));
        composition.push_str(&relative(&o.root, &o.out.join(&path))?);
        composition.push('\n');
        manifest.push((format!("copy:{index:04}:sha256"), digest(&bytes)));
        manifest.push((
            format!("copy:{index:04}:documents"),
            documents.len().to_string(),
        ));
        files.insert(path, bytes);
        let role = if o.family == "F2" && index <= calibration.active {
            "active"
        } else if rename.index == 0 {
            "original"
        } else {
            "dormant"
        };
        let count = roles.entry(role.into()).or_default();
        // All copies of a role differ only by the checked injective namespace.
        // Two representatives per role cover same-role cross-copy paths.
        if *count < 2 {
            representatives.push((index, shard));
            *count += 1;
        }
        if o.family == "F2" && index <= calibration.active {
            let mut intake = rename;
            intake.preserve_ids = true;
            patient_documents.push(intake.apply(&patient)?);
        }
    }
    isolation::check(&representatives, &patient_documents)?;
    manifest.push(("isolation".into(), "may-model-owner-provenance-1".into()));
    manifest.push((
        "isolation_representatives".into(),
        representatives
            .iter()
            .map(|(i, _)| i.to_string())
            .collect::<Vec<_>>()
            .join(","),
    ));
    let patient_bytes: Vec<u8> = patient_documents
        .iter()
        .flat_map(Document::render)
        .collect();
    composition.push_str(&relative(&o.root, &o.out.join("patient.pl"))?);
    composition.push('\n');
    files.insert("patient.pl".into(), patient_bytes);
    for (i, name) in QUERY_NAMES.iter().enumerate() {
        let source = query_path(o, name)?;
        inputs.push(source.clone());
        let bytes = if i == 0 {
            read(&source)?
        } else {
            let q = load(&source)?;
            if q.singleton
                || !q
                    .clauses()
                    .any(|c| c.head.call().is_some_and(|(n, _)| n == "$guideline_query"))
            {
                return Err(format!("query {name} is not a question"));
            }
            if i == 1 {
                renamer(o, 1, &produced, calibration.active)
                    .apply(&q)?
                    .render()
            } else {
                q.render()
            }
        };
        manifest.push((format!("query_sha256:{name}"), digest(&bytes)));
        files.insert(PathBuf::from("queries").join(name), bytes);
    }
    inputs.sort();
    inputs.dedup();
    for input in inputs {
        manifest.push((
            format!("input_sha256:{}", relative(&o.root, &input)?),
            digest(&read(&input)?),
        ));
    }
    let schema = read(&o.root.join("guidelines/schema.pl"))?;
    let identity = composition_identity(&schema, &composition, &files, o)?;
    manifest.push(("schema_sha256".into(), digest(&schema)));
    manifest.push(("composition_sha256".into(), identity));
    files.insert("composition.tsv".into(), composition.into_bytes());
    files.insert(
        "manifest.tsv".into(),
        manifest
            .into_iter()
            .map(|(k, v)| format!("{k}\t{v}\n"))
            .collect::<String>()
            .into_bytes(),
    );
    Ok(Product { files })
}
fn composition_identity(
    schema: &[u8],
    composition: &str,
    files: &BTreeMap<PathBuf, Vec<u8>>,
    o: &Options,
) -> Result<String, String> {
    let mut framed = format!("schema\0{}\0{}\n", schema.len(), digest(schema)).into_bytes();
    for line in composition.lines() {
        let path = o.root.join(line);
        let relative = path.strip_prefix(&o.out).map_err(|e| e.to_string())?;
        let bytes = files.get(relative).ok_or("composition file absent")?;
        framed.extend_from_slice(
            format!("file\0{line}\0{}\0{}\n", bytes.len(), digest(bytes)).as_bytes(),
        );
    }
    Ok(digest(&framed))
}
fn manifest(path: &Path) -> Result<BTreeMap<String, String>, String> {
    let mut rows = BTreeMap::new();
    for line in string(path)?.lines() {
        let (key, value) = line.split_once('\t').ok_or("manifest row shape")?;
        if key.is_empty()
            || value.is_empty()
            || value.contains('\t')
            || rows.insert(key.to_owned(), value.to_owned()).is_some()
        {
            return Err("manifest empty/duplicate row".into());
        }
    }
    Ok(rows)
}
fn check(args: &[String]) -> Result<(), String> {
    if args.len() != 3 || args[1] != "--root" {
        return Err("usage".into());
    }
    let root = PathBuf::from(&args[2])
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let out = output_path(&root, &args[0])?;
    let m = manifest(&out.join("manifest.tsv"))?;
    let get = |key: &str| {
        m.get(key)
            .map(String::as_str)
            .ok_or_else(|| format!("manifest missing {key}"))
    };
    if get("transform_version")? != VERSION {
        return Err("transform version mismatch".into());
    }
    let path = |key: &str| resolve_input(&root, get(key)?);
    let o = Options {
        family: get("family")?.into(),
        scale: get("scale")?.parse().map_err(|_| "manifest scale")?,
        documents: path("source_documents")?,
        patient: path("source_patient")?,
        annex: path("source_annex")?,
        queries: path("source_queries")?,
        expected: m
            .get("source_expected")
            .map(|p| resolve_input(&root, p))
            .transpose()?,
        seed: get("seed")?.parse().map_err(|_| "manifest seed")?,
        commit: Some(get("generator_commit")?.into()),
        root,
        out,
    };
    let expected = build(&o)?;
    for (path, want) in &expected.files {
        let got = read(&o.out.join(path))?;
        if got != *want {
            return Err(format!("transform replay differs: {}", path.display()));
        }
    }
    let mut actual = Vec::new();
    collect_files(&o.out, &o.out, &mut actual)?;
    if actual.len() != expected.files.len()
        || actual.iter().any(|p| !expected.files.contains_key(p))
    {
        return Err("transform output inventory differs".into());
    }
    Ok(())
}
fn collect_files(base: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    for item in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let item = item.map_err(|e| e.to_string())?;
        let kind = item.file_type().map_err(|e| e.to_string())?;
        if kind.is_dir() {
            collect_files(base, &item.path(), out)?;
        } else if kind.is_file() {
            out.push(
                item.path()
                    .strip_prefix(base)
                    .map_err(|e| e.to_string())?
                    .to_owned(),
            );
        } else {
            return Err("transform output contains a non-regular member".into());
        }
    }
    Ok(())
}
fn generate(o: &Options) -> Result<(), String> {
    if o.out.exists() {
        return Err("output directory already exists; select a new directory".into());
    }
    let product = build(o)?;
    fs::create_dir_all(&o.out).map_err(|e| e.to_string())?;
    let write = || -> Result<(), String> {
        for (p, bytes) in product.files {
            let destination = o.out.join(p);
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            fs::write(&destination, bytes)
                .map_err(|e| format!("{}: {e}", destination.display()))?;
        }
        Ok(())
    };
    if let Err(e) = write() {
        fs::remove_dir_all(&o.out).map_err(|cleanup| format!("{e}; cleanup: {cleanup}"))?;
        return Err(e);
    }
    Ok(())
}
pub fn run(args: &[String]) -> ExitCode {
    let result = if args.first().is_some_and(|s| s == "check") {
        check(&args[1..])
    } else {
        options(args).and_then(|o| generate(&o))
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            if e == "usage" {
                eprintln!(
                    "ckc: bench gen: usage: ckc bench gen <F1a|F1b|F2> <1|2|4|10|100> --root <repo> --patient <compiled.pl> --queries <dir> --out <rust/target/bench/dir> [--documents <dir|manifest.tsv>] [--annex <dir>] [--expected <q-all-output>] [--seed <u64>]; ckc bench gen check <dir> --root <repo>"
                );
                ExitCode::from(2)
            } else {
                eprintln!("ckc: bench gen: {e}");
                ExitCode::from(1)
            }
        }
    }
}
