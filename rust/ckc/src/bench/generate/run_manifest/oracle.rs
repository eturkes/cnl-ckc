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
// Contract sb D5 (MAIN ruling; scientist-5 appraisal): at ×100 the oracle = the union of
// per-isolation-group models — the whole-composition recipe grows superlinearly
// (F1a-g100-s0 past its 1,800 s timeout; G1 → G10 2.5 → 50 s); at G10 the whole
// composition runs too and must equal the union byte for byte.
fn groups(c: &Config, p: &Point) -> Result<Vec<Vec<usize>>, String> {
    let m = manifest(&p.dir(&c.root).join("manifest.tsv"))?;
    owner_groups(&m, p.scale).map_err(|e| format!("{}: {e}", p.id()))
}
// The generator's isolation owners: one group per shard, or (active-cluster scope) every
// non-padding shard together + one group per padding shard.
fn owner_groups(m: &BTreeMap<String, String>, scale: usize) -> Result<Vec<Vec<usize>>, String> {
    let clustered = match m.get("isolation_owner_scope").map(String::as_str) {
        Some("active-cluster_vs_padding") => true,
        Some("per-shard") => false,
        other => return Err(format!("isolation scope {other:?}")),
    };
    let mut cluster = Vec::new();
    let mut out = Vec::new();
    for slot in 1..=scale {
        let role = m
            .get(&format!("shard:{slot:04}:role"))
            .ok_or_else(|| format!("shard {slot} role"))?;
        if !matches!(
            role.as_str(),
            "active-source" | "active-transform" | "padding"
        ) {
            return Err(format!("shard {slot} role {role}"));
        }
        if clustered && role != "padding" {
            cluster.push(slot);
        } else {
            out.push(vec![slot]);
        }
    }
    if !cluster.is_empty() {
        out.insert(0, cluster);
    }
    Ok(out)
}
// A group = its own copies + the whole declared patient suffix (every intake projection +
// patient.pl): the isolation check treats each patient document as owner-neutral, so no
// group may lose one (scientist-5 O1); the union deduplicates the shared facts.
fn listing(composition: &str, slots: &[usize]) -> String {
    composition
        .lines()
        .filter(|line| {
            line.ends_with("/patient.pl")
                || line.contains("/intakes/")
                || slots
                    .iter()
                    .any(|s| line.contains(&format!("/copies/{s:04}/")))
        })
        .map(|line| format!("{line}\n"))
        .collect()
}
const MODES: [&str; 2] = ["recipe", "wfs"];
fn compose(c: &Config, p: &Point, recipe: &Path) -> Result<(), String> {
    let out = p.oracle(&c.root).join("union");
    if out.join("receipt.tsv").is_file() {
        return check_compose(c, p, recipe);
    }
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let composition = string(&p.dir(&c.root).join("composition.tsv"))?;
    let groups = groups(c, p)?;
    let knowledge = |text: &str| {
        text.lines()
            .filter(|l| !l.ends_with("/patient.pl") && !l.contains("/intakes/"))
            .count()
    };
    let mut covered = 0;
    let mut models = [String::new(), String::new()];
    let mut text = format!(
        "groups\t{}\nsource_identity\t{}\n",
        groups.len(),
        source(c, p, &p.dir(&c.root).join("composition.tsv"), recipe)?
    );
    for (i, slots) in groups.iter().enumerate() {
        let dir = p.oracle(&c.root).join(format!("groups/{i:04}"));
        let list = listing(&composition, slots);
        covered += knowledge(&list);
        write(&dir.join("composition.tsv"), &list)?;
        text.push_str(&format!(
            "group:{i:04}:slots\t{}\n",
            slots
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ));
        for (k, mode) in MODES.into_iter().enumerate() {
            let run = dir.join(mode);
            one(c, p, &dir.join("composition.tsv"), recipe, mode, &run)?;
            models[k].push_str(&format!("{}\n", run.join("model.out").display()));
            text.push_str(&format!(
                "group:{i:04}:{mode}_sha256\t{}\n",
                digest(&read(&run.join("model.out"))?)
            ));
        }
    }
    if covered != knowledge(&composition) {
        return Err(format!(
            "{}: isolation groups cover {covered} files",
            p.id()
        ));
    }
    for (k, mode) in MODES.into_iter().enumerate() {
        let list = out.join(format!("{mode}-models.txt"));
        write(&list, &models[k])?;
        swi(
            c,
            "sb_manifest_union_models",
            &[list, out.join(format!("{mode}-model.out"))],
            &out.join(format!("{mode}-merge.log")),
        )?;
    }
    // Recipe union = WFS union: no group leaves an undefined or differing fact.
    equal(&out.join("recipe-model.out"), &out.join("wfs-model.out"))?;
    fs::copy(out.join("recipe-model.out"), out.join("model.out")).map_err(|e| e.to_string())?;
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
        text.push_str(&format!(
            "sha256:{name}\t{}\n",
            digest(&read(&out.join(format!("{name}.out")))?)
        ));
    }
    write(&out.join("receipt.tsv"), text)?;
    check_compose(c, p, recipe)
}
fn check_compose(c: &Config, p: &Point, recipe: &Path) -> Result<(), String> {
    let out = p.oracle(&c.root).join("union");
    let receipt = manifest(&out.join("receipt.tsv"))?;
    let groups = groups(c, p)?;
    if receipt.get("groups") != Some(&groups.len().to_string())
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
    for (i, slots) in groups.iter().enumerate() {
        let dir = p.oracle(&c.root).join(format!("groups/{i:04}"));
        let want = slots
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",");
        if receipt.get(&format!("group:{i:04}:slots")) != Some(&want) {
            return Err(format!("{}: union group {i} slots", p.id()));
        }
        for mode in MODES {
            let run = dir.join(mode);
            check_run(c, p, &dir.join("composition.tsv"), recipe, mode, &run)?;
            if receipt.get(&format!("group:{i:04}:{mode}_sha256"))
                != Some(&digest(&read(&run.join("model.out"))?))
            {
                return Err(format!("{}: union group {i} {mode}", p.id()));
            }
        }
    }
    equal(&out.join("recipe-model.out"), &out.join("wfs-model.out"))?;
    equal(&out.join("recipe-model.out"), &out.join("model.out"))?;
    for name in ANSWERS {
        if receipt.get(&format!("sha256:{name}"))
            != Some(&digest(&read(&out.join(format!("{name}.out")))?))
        {
            return Err(format!("{}: union {name} changed", p.id()));
        }
    }
    for name in [
        "recipe-merge.stdout",
        "recipe-merge.stderr",
        "wfs-merge.stdout",
        "wfs-merge.stderr",
        "answers.stdout",
        "answers.stderr",
    ] {
        if !read(&out.join(name))?.is_empty() {
            return Err(format!("{}: union process output {name}", p.id()));
        }
    }
    Ok(())
}
// Oracle points: G10 (whole composition + union, equal) and G100 (union).
pub(super) fn oracle_point(p: &Point) -> bool {
    p.scale == 10 || p.scale == 100
}
fn whole(c: &Config, p: &Point, recipe: &Path, check_only: bool) -> Result<(), String> {
    let composition = p.dir(&c.root).join("composition.tsv");
    for mode in MODES {
        let out = p.oracle(&c.root).join(mode);
        if check_only {
            check_run(c, p, &composition, recipe, mode, &out)?;
        } else {
            one(c, p, &composition, recipe, mode, &out)?;
        }
    }
    for name in ANSWERS {
        equal(
            &p.oracle(&c.root).join("recipe").join(format!("{name}.out")),
            &p.oracle(&c.root).join("wfs").join(format!("{name}.out")),
        )?;
        equal(
            &p.oracle(&c.root).join("recipe").join(format!("{name}.out")),
            &p.oracle(&c.root).join("union").join(format!("{name}.out")),
        )?;
    }
    Ok(())
}
// Red input every package run (scientist-5): on an active-cluster family at G10, the
// per-shard split of the cluster must NOT reproduce the whole model, else the partition
// check would pass a wrong decomposition unnoticed.
fn split_plant(c: &Config, p: &Point, recipe: &Path) -> Result<(), String> {
    let m = manifest(&p.dir(&c.root).join("manifest.tsv"))?;
    if m.get("isolation_owner_scope").map(String::as_str) != Some("active-cluster_vs_padding")
        || owner_groups(&m, p.scale)?
            .first()
            .is_none_or(|g| g.len() < 2)
    {
        return Ok(());
    }
    let out = p.oracle(&c.root).join("split-plant");
    let composition = string(&p.dir(&c.root).join("composition.tsv"))?;
    let mut models = String::new();
    for slot in 1..=p.scale {
        let dir = out.join(format!("{slot:04}"));
        write(&dir.join("composition.tsv"), listing(&composition, &[slot]))?;
        one(
            c,
            p,
            &dir.join("composition.tsv"),
            recipe,
            "recipe",
            &dir.join("recipe"),
        )?;
        models.push_str(&format!("{}\n", dir.join("recipe/model.out").display()));
    }
    write(&out.join("models.txt"), models)?;
    swi(
        c,
        "sb_manifest_union_models",
        &[out.join("models.txt"), out.join("model.out")],
        &out.join("merge.log"),
    )?;
    if read(&out.join("model.out"))? == read(&p.oracle(&c.root).join("recipe/model.out"))? {
        return Err(format!(
            "{}: the split-cluster plant reproduced the whole model",
            p.id()
        ));
    }
    write(
        &out.join("receipt.tsv"),
        "plant\tper-shard-split-of-the-active-cluster\nwhole_model\tdiffers\n",
    )
}
pub(super) fn run(c: &Config) -> Result<(), String> {
    version()?;
    let recipe = recipe(c)?;
    for p in points(c).into_iter().filter(oracle_point) {
        instances::check(c, &p)?;
        compose(c, &p, &recipe)?;
        if p.scale == 10 {
            whole(c, &p, &recipe, false)?;
            split_plant(c, &p, &recipe)?;
            println!(
                "sb-run-manifest: oracle {} whole = union (recipe + WFS)",
                p.id()
            );
        } else {
            println!(
                "sb-run-manifest: oracle {} union (recipe + WFS per isolation group)",
                p.id()
            );
        }
        if p.contrast.is_some() {
            super::contrast::run(c, &p)?;
        }
    }
    Ok(())
}
pub(super) fn check(c: &Config, p: &Point) -> Result<(), String> {
    let recipe = recipe(c)?;
    check_compose(c, p, &recipe)?;
    if p.scale == 10 {
        whole(c, p, &recipe, true)?;
        let m = manifest(&p.dir(&c.root).join("manifest.tsv"))?;
        let cluster = owner_groups(&m, p.scale)?
            .first()
            .is_some_and(|g| g.len() >= 2);
        if m.get("isolation_owner_scope").map(String::as_str) == Some("active-cluster_vs_padding")
            && cluster
            && !string(&p.oracle(&c.root).join("split-plant/receipt.tsv"))?
                .contains("whole_model\tdiffers\n")
        {
            return Err(format!("{}: split-cluster plant receipt", p.id()));
        }
    }
    if p.contrast.is_some() {
        super::contrast::check(c, p)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roles(scope: &str, roles: &[&str]) -> BTreeMap<String, String> {
        let mut m = BTreeMap::from([("isolation_owner_scope".to_owned(), scope.to_owned())]);
        for (i, role) in roles.iter().enumerate() {
            m.insert(format!("shard:{:04}:role", i + 1), (*role).to_owned());
        }
        m
    }
    #[test]
    fn owner_groups_follow_the_isolation_scope() {
        let shared = [
            "active-source",
            "active-transform",
            "active-transform",
            "padding",
            "padding",
        ];
        assert_eq!(
            owner_groups(&roles("active-cluster_vs_padding", &shared), 5).unwrap(),
            [vec![1, 2, 3], vec![4], vec![5]]
        );
        assert_eq!(
            owner_groups(&roles("per-shard", &shared), 5).unwrap(),
            [vec![1], vec![2], vec![3], vec![4], vec![5]]
        );
        // Plants: an unknown scope, a missing role, an unknown role.
        assert!(owner_groups(&roles("whole", &shared), 5).is_err());
        assert!(owner_groups(&roles("per-shard", &shared), 6).is_err());
        assert!(owner_groups(&roles("per-shard", &["active"]), 1).is_err());
    }
    #[test]
    fn every_group_carries_the_whole_patient_suffix() {
        let composition = "r/i/copies/0001/a.pl\nr/i/copies/0001/b.pl\nr/i/copies/0002/c.pl\nr/i/intakes/0002/p.pl\nr/i/intakes/0003/p.pl\nr/i/patient.pl\n";
        assert_eq!(
            listing(composition, &[1]),
            "r/i/copies/0001/a.pl\nr/i/copies/0001/b.pl\nr/i/intakes/0002/p.pl\nr/i/intakes/0003/p.pl\nr/i/patient.pl\n"
        );
        assert_eq!(
            listing(composition, &[2]),
            "r/i/copies/0002/c.pl\nr/i/intakes/0002/p.pl\nr/i/intakes/0003/p.pl\nr/i/patient.pl\n"
        );
        // Slot 1 never matches slot 10's copies.
        assert_eq!(listing("r/i/copies/0010/x.pl\n", &[1]), "");
    }
}
