use super::*;

fn recipe(c: &Config) -> Result<PathBuf, String> {
    let reference = string(&c.root.join("docs/REFERENCE.md"))?;
    let section = reference
        .split("### Loading a whole knowledge base\n")
        .nth(1)
        .ok_or("consumer recipe heading")?
        .split("\n### ")
        .next()
        .ok_or("consumer recipe section")?;
    let blocks: Vec<_> = section
        .split("```prolog\n")
        .skip(1)
        .map(|s| s.split("```\n").next().unwrap())
        .collect();
    if blocks.len() != 3 {
        return Err("consumer recipe requires three Prolog blocks".into());
    }
    let check = blocks[2]
        .trim()
        .strip_suffix('.')
        .ok_or("consumer identifier check terminator")?;
    let path = c.root.join(ORACLES).join("recipe.pl");
    write(
        &path,
        format!(
            "{}{}\nsb_manifest_identifier_check :- {check}.\n",
            blocks[0], blocks[1]
        ),
    )?;
    Ok(path)
}
fn version() -> Result<String, String> {
    let out = Command::new("swipl")
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8(out.stdout).map_err(|e| e.to_string())?;
    if !out.status.success()
        || !out.stderr.is_empty()
        || !text.starts_with("SWI-Prolog version 10.0.2 ")
    {
        return Err(format!(
            "independent oracle requires SWI-Prolog 10.0.2: {text}"
        ));
    }
    Ok(text.trim().to_owned())
}
fn executable_hash() -> Result<String, String> {
    for dir in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
        let path = dir.join("swipl");
        if path.is_file() {
            return Ok(digest(&read(
                &path.canonicalize().map_err(|e| e.to_string())?,
            )?));
        }
    }
    Err("SWI-Prolog executable is absent".into())
}
fn swi(c: &Config, goal: &str, argv: &[PathBuf], log: &Path) -> Result<(), String> {
    execute(
        Command::new("timeout")
            .args([
                "--kill-after=5",
                "1800",
                "swipl",
                "-q",
                "-f",
                "none",
                "-F",
                "none",
                "--stack-limit=32g",
                "--table-space=32g",
                "-s",
            ])
            .arg(c.root.join(DRIVER))
            .args(["-g", goal, "-t", "halt(9)", "--"])
            .args(argv)
            .env("LANG", "C.UTF-8")
            .env("LC_ALL", "C.UTF-8"),
        log,
    )
}
fn source(c: &Config, p: &Point, composition: &Path, recipe: &Path) -> Result<String, String> {
    let m = manifest(&p.dir(&c.root).join("manifest.tsv"))?;
    let mut frame = format!("composition\t{}\n", m["composition_sha256"]);
    for path in [
        composition.to_owned(),
        c.root.join(DRIVER),
        recipe.to_owned(),
        c.root.join("tests/consumer/wfs.pl.in"),
    ]
    .into_iter()
    .chain(
        super::super::QUERY_NAMES
            .into_iter()
            .map(|name| p.dir(&c.root).join("queries").join(name)),
    ) {
        frame.push_str(&format!(
            "{}\t{}\n",
            relative(&c.root, &path)?,
            digest(&read(&path)?)
        ));
    }
    Ok(digest(frame.as_bytes()))
}
fn check_run(
    c: &Config,
    p: &Point,
    composition: &Path,
    recipe: &Path,
    mode: &str,
    out: &Path,
) -> Result<(), String> {
    let receipt = manifest(&out.join("receipt.tsv"))?;
    if receipt.get("source_identity") != Some(&source(c, p, composition, recipe)?)
        || receipt.get("mode").map(String::as_str) != Some(mode)
        || receipt.get("rc").map(String::as_str) != Some("0")
        || receipt.get("undefined").map(String::as_str) != Some("0")
        || receipt
            .get("swipl_version")
            .is_none_or(|v| !v.starts_with("SWI-Prolog version 10.0.2 "))
        || receipt
            .get("swipl_executable_sha256")
            .is_none_or(|v| v.len() != 64)
    {
        return Err(format!("{} {mode}: oracle receipt", p.id()));
    }
    for name in ANSWERS {
        let path = out.join(format!("{name}.out"));
        if receipt.get(&format!("sha256:{name}")) != Some(&digest(&read(&path)?)) {
            return Err(format!("{} {mode}: oracle {name} differs", p.id()));
        }
    }
    let status = manifest(&out.join("status.tsv"))?;
    if status.get("undefined").map(String::as_str) != Some("0")
        || status.get("mode").map(String::as_str) != Some(mode)
        || status.get("model_facts")
            != Some(&line_count(&read(&out.join("model.out"))?).to_string())
        || !read(&out.join("process.stdout"))?.is_empty()
        || !read(&out.join("process.stderr"))?.is_empty()
    {
        return Err(format!("{} {mode}: oracle status/load", p.id()));
    }
    Ok(())
}
fn one(
    c: &Config,
    p: &Point,
    composition: &Path,
    recipe: &Path,
    mode: &str,
    out: &Path,
) -> Result<(), String> {
    if out.join("receipt.tsv").is_file() {
        return check_run(c, p, composition, recipe, mode, out);
    }
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let argv = [
        c.root.clone(),
        composition.to_owned(),
        p.dir(&c.root).join("queries"),
        out.to_owned(),
        recipe.to_owned(),
        PathBuf::from(mode),
    ];
    swi(c, "sb_manifest_oracle", &argv, &out.join("process.log"))?;
    for name in ["q-one", "q-patient"] {
        if let ckc_kernel::EV1Verdict::Reject { at } =
            ckc_kernel::contract::v1_check(&read(&out.join(format!("{name}.out")))?)
        {
            return Err(format!(
                "{} {mode}: {name} canonical answer rejected at {at}",
                p.id()
            ));
        }
    }
    let mut rows = vec![
        ("source_identity", source(c, p, composition, recipe)?),
        ("mode", mode.into()),
        ("rc", "0".into()),
        ("undefined", "0".into()),
        ("swipl_version", version()?),
        ("swipl_executable_sha256", executable_hash()?),
    ];
    let output_hashes: Vec<_> = ANSWERS
        .into_iter()
        .map(|name| {
            Ok((
                format!("sha256:{name}"),
                digest(&read(&out.join(format!("{name}.out")))?),
            ))
        })
        .collect::<Result<_, String>>()?;
    rows.extend(output_hashes.iter().map(|(k, v)| (k.as_str(), v.clone())));
    pairs(&out.join("receipt.tsv"), &rows)?;
    check_run(c, p, composition, recipe, mode, out)
}
fn union(c: &Config, p: &Point, recipe: &Path) -> Result<(), String> {
    let out = p.oracle(&c.root).join("union");
    if out.join("receipt.tsv").is_file() {
        return check_union(c, p, recipe);
    }
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let composition = string(&p.dir(&c.root).join("composition.tsv"))?;
    let mut models = String::new();
    let mut hashes = Vec::new();
    for slot in 1..=p.scale {
        let isolated = p.oracle(&c.root).join(format!("isolated/{slot:04}"));
        let isolated_composition = isolated.join("composition.tsv");
        let copy = format!("/copies/{slot:04}/");
        let intake = format!("/intakes/{slot:04}/");
        let listing = composition
            .lines()
            .filter(|line| {
                line.contains(&copy) || line.contains(&intake) || line.ends_with("/patient.pl")
            })
            .map(|line| format!("{line}\n"))
            .collect::<String>();
        write(&isolated_composition, listing)?;
        one(c, p, &isolated_composition, recipe, "recipe", &isolated)?;
        models.push_str(&format!("{}\n", isolated.join("model.out").display()));
        hashes.push((
            format!("copy:{slot:04}:sha256"),
            digest(&read(&isolated.join("model.out"))?),
        ));
    }
    let list = out.join("models.txt");
    write(&list, models)?;
    swi(
        c,
        "sb_manifest_union_models",
        &[list, out.join("model.out")],
        &out.join("merge.log"),
    )?;
    equal(
        &out.join("model.out"),
        &p.oracle(&c.root).join("recipe/model.out"),
    )?;
    swi(
        c,
        "sb_manifest_union_answers",
        &[
            c.root.clone(),
            out.join("model.out"),
            p.dir(&c.root).join("queries"),
            out.clone(),
            recipe.to_owned(),
        ],
        &out.join("answers.log"),
    )?;
    for name in ANSWERS {
        equal(
            &out.join(format!("{name}.out")),
            &p.oracle(&c.root).join("recipe").join(format!("{name}.out")),
        )?;
    }
    let mut text = format!(
        "isolated_copies\t{}\nsource_identity\t{}\n",
        p.scale,
        source(c, p, &p.dir(&c.root).join("composition.tsv"), recipe)?
    );
    for (key, value) in hashes {
        text.push_str(&format!("{key}\t{value}\n"));
    }
    for name in ANSWERS {
        text.push_str(&format!(
            "sha256:{name}\t{}\n",
            digest(&read(&out.join(format!("{name}.out")))?)
        ));
    }
    write(&out.join("receipt.tsv"), text)?;
    check_union(c, p, recipe)
}
fn check_union(c: &Config, p: &Point, recipe: &Path) -> Result<(), String> {
    let out = p.oracle(&c.root).join("union");
    let receipt = manifest(&out.join("receipt.tsv"))?;
    if receipt.get("isolated_copies") != Some(&p.scale.to_string())
        || receipt.get("source_identity")
            != Some(&source(
                c,
                p,
                &p.dir(&c.root).join("composition.tsv"),
                recipe,
            )?)
    {
        return Err(format!("{}: union receipt", p.id()));
    }
    for slot in 1..=p.scale {
        let isolated = p.oracle(&c.root).join(format!("isolated/{slot:04}"));
        check_run(
            c,
            p,
            &isolated.join("composition.tsv"),
            recipe,
            "recipe",
            &isolated,
        )?;
        if receipt.get(&format!("copy:{slot:04}:sha256"))
            != Some(&digest(&read(&isolated.join("model.out"))?))
        {
            return Err(format!("{}: union copy {slot}", p.id()));
        }
    }
    for name in ANSWERS {
        let path = out.join(format!("{name}.out"));
        if receipt.get(&format!("sha256:{name}")) != Some(&digest(&read(&path)?)) {
            return Err(format!("{}: union {name} changed", p.id()));
        }
        equal(
            &path,
            &p.oracle(&c.root).join("recipe").join(format!("{name}.out")),
        )?;
    }
    for name in [
        "merge.stdout",
        "merge.stderr",
        "answers.stdout",
        "answers.stderr",
    ] {
        if !read(&out.join(name))?.is_empty() {
            return Err(format!("{}: union process output {name}", p.id()));
        }
    }
    Ok(())
}
pub(super) fn run(c: &Config) -> Result<(), String> {
    version()?;
    let recipe = recipe(c)?;
    for p in points(c).into_iter().filter(|p| p.scale == 100) {
        instances::check(c, &p)?;
        let composition = p.dir(&c.root).join("composition.tsv");
        for mode in ["recipe", "wfs"] {
            one(
                c,
                &p,
                &composition,
                &recipe,
                mode,
                &p.oracle(&c.root).join(mode),
            )?;
            println!(
                "sb-run-manifest: oracle {} {mode} complete; undefined=0",
                p.id()
            );
        }
        for name in ANSWERS {
            equal(
                &p.oracle(&c.root).join("recipe").join(format!("{name}.out")),
                &p.oracle(&c.root).join("wfs").join(format!("{name}.out")),
            )?;
        }
        if p.contrast.is_some() {
            super::contrast::run(c, &p)?;
        } else if ["F1a", "F1b", "F2"].contains(&p.family.as_str()) {
            union(c, &p, &recipe)?;
        }
        println!(
            "sb-run-manifest: endpoint {} exact recipe/WFS agreement",
            p.id()
        );
    }
    Ok(())
}
pub(super) fn check(c: &Config, p: &Point) -> Result<(), String> {
    let recipe = recipe(c)?;
    for mode in ["recipe", "wfs"] {
        check_run(
            c,
            p,
            &p.dir(&c.root).join("composition.tsv"),
            &recipe,
            mode,
            &p.oracle(&c.root).join(mode),
        )?;
    }
    for name in ANSWERS {
        equal(
            &p.oracle(&c.root).join("recipe").join(format!("{name}.out")),
            &p.oracle(&c.root).join("wfs").join(format!("{name}.out")),
        )?;
    }
    if p.contrast.is_some() {
        super::contrast::check(c, p)?;
    } else if ["F1a", "F1b", "F2"].contains(&p.family.as_str()) {
        check_union(c, p, &recipe)?;
    }
    Ok(())
}
