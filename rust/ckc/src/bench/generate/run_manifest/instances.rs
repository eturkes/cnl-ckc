use super::*;

fn args(c: &Config, p: &Point) -> Vec<String> {
    let patient = if matches!(p.family.as_str(), "F1a" | "F1b") {
        "adult".to_owned()
    } else {
        p.family.to_ascii_lowercase()
    };
    vec![
        p.family.clone(),
        p.scale.to_string(),
        "--root".into(),
        c.root.display().to_string(),
        "--patient".into(),
        format!("bench/sb/fixtures/patients/p-{patient}.pl.in"),
        "--out".into(),
        relative(&c.root, &p.dir(&c.root)).unwrap(),
        "--seed".into(),
        p.seed.to_string(),
    ]
}
fn listing_path(c: &Config, p: &Point, kind: &str) -> PathBuf {
    c.root
        .join(RECEIPTS)
        .join(format!("{}.{kind}.sha256.tsv", p.id()))
}
fn baseline_required(p: &Point) -> bool {
    p.seed == 0 && p.scale < 100
}

pub(super) fn run(c: &Config) -> Result<(), String> {
    for p in points(c) {
        if p.receipt(&c.root).is_file() {
            check(c, &p)?;
            println!(
                "sb-run-manifest: instance {} reused (receipt + current bytes agree)",
                p.id()
            );
            continue;
        }
        let dir = p.dir(&c.root);
        if dir.exists() {
            fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
        }
        let argv = args(c, &p);
        let mut baseline_hash = "not-required".to_owned();
        let mut before = None;
        if baseline_required(&p) {
            let binary = c
                .baseline
                .as_ref()
                .ok_or("nested development points require --baseline <d1995811-ckc>")?;
            baseline_hash = digest(&read(binary)?);
            execute(
                Command::new(binary).args(["bench", "gen"]).args(&argv),
                &p.receipt(&c.root).with_extension("baseline"),
            )?;
            let listing = tree(&dir)?;
            write(&listing_path(c, &p, "baseline"), &listing)?;
            before = Some(listing);
            fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
        }
        let options = super::super::options(&argv)?;
        super::super::generate(&options)?;
        let first = tree(&dir)?;
        if before.as_ref().is_some_and(|baseline| *baseline != first) {
            return Err(format!(
                "{}: seed-0 baseline bytes differ at the same output path",
                p.id()
            ));
        }
        write(&listing_path(c, &p, "first"), &first)?;
        fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
        super::super::generate(&options)?;
        let second = tree(&dir)?;
        if first != second {
            return Err(format!("{}: repeated generation differs", p.id()));
        }
        write(&listing_path(c, &p, "second"), &second)?;
        super::super::check(&[
            dir.display().to_string(),
            "--root".into(),
            c.root.display().to_string(),
        ])?;
        let m = manifest(&dir.join("manifest.tsv"))?;
        pairs(
            &p.receipt(&c.root),
            &[
                ("instance", p.id()),
                ("repetitions", "2".into()),
                ("tree_sha256", digest(first.as_bytes())),
                ("baseline_binary_sha256", baseline_hash),
                (
                    "baseline_tree_sha256",
                    before
                        .as_ref()
                        .map(|s| digest(s.as_bytes()))
                        .unwrap_or_else(|| "not-required".into()),
                ),
                ("generator_commit", m["generator_commit"].clone()),
                (
                    "generator_binary_sha256",
                    digest(&read(&std::env::current_exe().map_err(|e| e.to_string())?)?),
                ),
                ("structural_replay", "passed".into()),
            ],
        )?;
        check(c, &p)?;
        println!(
            "sb-run-manifest: instance {} generated twice; structural replay passed",
            p.id()
        );
    }
    Ok(())
}
pub(super) fn check(c: &Config, p: &Point) -> Result<(), String> {
    let receipt = manifest(&p.receipt(&c.root))?;
    if receipt.get("instance") != Some(&p.id())
        || receipt.get("repetitions").map(String::as_str) != Some("2")
        || receipt.get("structural_replay").map(String::as_str) != Some("passed")
    {
        return Err(format!("{}: generation receipt", p.id()));
    }
    let first = string(&listing_path(c, p, "first"))?;
    let second = string(&listing_path(c, p, "second"))?;
    if first != second
        || receipt.get("tree_sha256") != Some(&digest(first.as_bytes()))
        || tree(&p.dir(&c.root))? != first
    {
        return Err(format!("{}: frozen generation bytes differ", p.id()));
    }
    if baseline_required(p) {
        let before = string(&listing_path(c, p, "baseline"))?;
        if before != first
            || receipt.get("baseline_tree_sha256") != receipt.get("tree_sha256")
            || receipt
                .get("baseline_binary_sha256")
                .is_none_or(|s| s.len() != 64)
        {
            return Err(format!("{}: baseline receipt", p.id()));
        }
    }
    let m = manifest(&p.dir(&c.root).join("manifest.tsv"))?;
    if m.get("family") != Some(&p.family)
        || m.get("scale") != Some(&p.scale.to_string())
        || m.get("seed") != Some(&p.seed.to_string())
        || m.get("generator_commit") != receipt.get("generator_commit")
        || m.get("transform_version").map(String::as_str) != Some(super::super::VERSION)
    {
        return Err(format!("{}: frozen family/vector/version", p.id()));
    }
    for (key, value) in m {
        if let Some(path) = key.strip_prefix("input_sha256:")
            && digest(&read(&c.root.join(path))?) != value
        {
            return Err(format!("{}: source input changed: {path}", p.id()));
        }
    }
    Ok(())
}
