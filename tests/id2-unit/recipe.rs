use super::*;

struct Recipe {
    declaration: String,
    table: String,
    check: String,
    section: String,
}
fn recipe() -> Result<Recipe, String> {
    let body = read(root().join("docs/REFERENCE.md"));
    let tail = body
        .split_once("### Loading a whole knowledge base\n")
        .ok_or("recipe heading missing")?
        .1;
    let section = tail
        .split_once("\n### ")
        .map_or(tail, |(s, _)| s)
        .to_owned();
    let mut blocks = Vec::new();
    let mut rest = section.as_str();
    while let Some((_, tail)) = rest.split_once("```prolog\n") {
        let (block, tail) = tail
            .split_once("```\n")
            .ok_or("unterminated recipe code block")?;
        blocks.push(block.to_owned());
        rest = tail;
    }
    if blocks.len() < 3 {
        return Err(format!(
            "D5 requires declaration/table/post-load blocks; found {}",
            blocks.len()
        ));
    }
    Ok(Recipe {
        declaration: blocks[0].clone(),
        table: blocks[1].clone(),
        check: blocks[2].trim().trim_end_matches('.').to_owned(),
        section,
    })
}
fn need_recipe(c: &mut Cases) -> Option<Recipe> {
    match recipe() {
        Ok(r) => Some(r),
        Err(e) => {
            c.check("recipe blocks", false, e);
            None
        }
    }
}
fn pinned_swipl(c: &mut Cases, s: &Scratch) -> PathBuf {
    let bin = std::env::var_os("SWIPL")
        .map(PathBuf::from)
        .unwrap_or_else(|| "swipl".into());
    let out = run(&bin, &s.0, &["--version".into()], &[]);
    c.check(
        "SWI-Prolog pin",
        out.rc == 0
            && out.stderr.is_empty()
            && out.stdout.starts_with("SWI-Prolog version 10.0.2 "),
        format!("{out:?}"),
    );
    bin
}
fn checked(
    s: &Scratch,
    r: &Recipe,
    documents: &[PathBuf],
    before_check: &str,
    c: &mut Cases,
) -> Out {
    let bin = pinned_swipl(c, s);
    let declaration = s.file("declaration.pl", &r.declaration);
    let table = s.file("table.pl", &r.table);
    let paths = [declaration, table]
        .into_iter()
        .chain(documents.iter().cloned())
        .map(|p| quote(&p))
        .collect::<Vec<_>>()
        .join(",");
    let goal = format!(
        "consult([{paths}]), {before_check} catch((({},write('ok'),nl)), E, ((E=error(schema_identifier(expected(2)),C),var(C))->write('error(schema_identifier(expected(2)),_).'),nl;(write_term(E,[quoted(true),ignore_ops(true)]),nl,halt(32)))),halt(0)",
        r.check
    );
    run(
        &bin,
        &s.0,
        &[
            "-q".into(),
            "-f".into(),
            "none".into(),
            "-F".into(),
            "none".into(),
            "-g".into(),
            goal,
            "-t".into(),
            "halt(9)".into(),
        ],
        &[],
    )
}

#[test]
fn p4_recipe_exposes_declaration_table_and_post_load_check() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let mut c = Cases::default();
    if let Some(r) = need_recipe(&mut c) {
        let compact: String = r
            .declaration
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        c.check(
            "multifile declaration block",
            compact == ":-multifile(guideline_schema_version/1).",
            r.declaration,
        );
        c.check(
            "one table directive",
            r.table.trim_start().starts_with(":- table ") && r.table.matches(":-").count() == 1,
            &r.table,
        );
        c.check(
            "cached-load instruction",
            r.section.contains("abolish_all_tables/0") || r.section.contains("abolish_all_tables"),
            "later consultations invalidate completed tables",
        );
        c.check(
            "post-load identifier check",
            r.check.contains("aggregate_all") && r.check.contains("schema_identifier"),
            r.check,
        );
    }
    c.finish();
}

#[test]
fn p4_recipe_passes_corpus_and_each_patient_world_with_clean_loads() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("recipe-worlds");
    let mut c = Cases::default();
    if let Some(r) = need_recipe(&mut c) {
        let mut docs = vec![root().join("guidelines/schema.pl")];
        for gid in sorted(root().join("guidelines"))
            .into_iter()
            .filter(|p| p.is_dir())
        {
            docs.extend(pl_files(gid.join("pl")));
        }
        c.check(
            "corpus positive control",
            docs.len() > 1,
            format!("{} load files", docs.len()),
        );
        for name in ["corpus", "dose", "adult", "minor", "cancer"] {
            let mut composition = docs.clone();
            if name == "dose" {
                composition.push(root().join(DOSE));
            } else if name != "corpus" {
                composition.push(s.file(
                    &format!("{name}.pl"),
                    fs::read(root().join("tests/consumer").join(format!("{name}.pl.in"))).unwrap(),
                ));
            }
            let out = checked(&s, &r, &composition, "", &mut c);
            c.same(&format!("recipe/{name}"), out, Out::ok("ok\n"));
        }
    }
    c.finish();
}
fn refused(name: &str, versions: &[u8], cached: bool) {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new(name);
    let mut c = Cases::default();
    if let Some(r) = need_recipe(&mut c) {
        let docs: Vec<_> = versions
            .iter()
            .enumerate()
            .map(|(i, v)| {
                s.file(
                    &format!("doc-{i}.pl"),
                    format!("guideline_schema_version({v}).\n"),
                )
            })
            .collect();
        let before = if cached {
            let later = s.file("later.pl", "guideline_schema_version(1).\n");
            format!("{},consult({}),abolish_all_tables,", r.check, quote(&later))
        } else {
            String::new()
        };
        let out = checked(&s, &r, &docs, &before, &mut c);
        c.same(
            name,
            out,
            Out::ok("error(schema_identifier(expected(2)),_).\n"),
        );
    }
    c.finish();
}
#[test]
fn p4_recipe_mixed_composition_throws_exact_schema_exception() {
    refused("recipe-mixed", &[1, 2], false);
}
#[test]
fn p4_recipe_all_old_composition_throws_exact_schema_exception() {
    refused("recipe-all-old", &[1, 1], false);
}
#[test]
fn p4_recipe_empty_composition_throws_exact_schema_exception() {
    refused("recipe-empty", &[], false);
}
#[test]
fn p4_recipe_cached_reload_abolishes_tables_then_refuses_old_clause() {
    refused("recipe-cached", &[2], true);
}

// The D5 literal, independent of the live recipe, grades this harness's exception catcher.
#[test]
fn p4_recipe_driver_controls_exact_schema_exception() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("recipe-driver");
    let mut c = Cases::default();
    let r = Recipe {
        declaration: ":- multifile(guideline_schema_version/1).\n".into(),
        table: ":- table guideline_schema_version/1.\n".into(),
        check: "( aggregate_all(set(V), guideline_schema_version(V), [2]) -> true ; throw(error(schema_identifier(expected(2)), _)) )".into(),
        section: String::new(),
    };
    for (label, versions, expected) in [
        ("current", vec![2], "ok\n"),
        (
            "empty",
            vec![],
            "error(schema_identifier(expected(2)),_).\n",
        ),
        ("old", vec![1], "error(schema_identifier(expected(2)),_).\n"),
        (
            "mixed",
            vec![2, 1],
            "error(schema_identifier(expected(2)),_).\n",
        ),
    ] {
        let docs: Vec<_> = versions
            .iter()
            .enumerate()
            .map(|(i, v)| {
                s.file(
                    &format!("{label}-{i}.pl"),
                    format!("guideline_schema_version({v}).\n"),
                )
            })
            .collect();
        let out = checked(&s, &r, &docs, "", &mut c);
        c.same(&format!("driver/{label}"), out, Out::ok(expected));
    }
    c.finish();
}

#[test]
fn p4_recipe_load_fault_is_not_an_identifier_rejection() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("recipe-load-fault");
    let mut c = Cases::default();
    if let Some(r) = need_recipe(&mut c) {
        let bad = s.file(
            "bad.pl",
            "guideline_schema_version(2).\n:- throw(error(id2_load_fault,fixture)).\n",
        );
        let out = checked(&s, &r, &[bad], "", &mut c);
        c.check(
            "load fault fails clean-load gate",
            out.rc != 0 || !out.stderr.is_empty(),
            format!("{out:?}"),
        );
        c.check(
            "load fault never equals schema refusal",
            out != Out::ok("error(schema_identifier(expected(2)),_).\n"),
            "exact envelope excludes the load fault",
        );
    }
    c.finish();
}
