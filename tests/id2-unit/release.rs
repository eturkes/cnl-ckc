use super::*;

// D2 release derivation fixtures: real HEAD inventory, rights, six-column review rows + export.
// These are deliberately small non-codec carriers, as dist_cases' fixture builders are.
pub(super) fn fixture_repo(s: &Scratch) -> PathBuf {
    let repo = s.0.join("repo");
    fs::create_dir_all(&repo).unwrap();
    git_ok(&repo, &["init", "-q", "-b", "main"]);
    for (path, body) in [
        (
            "docs/REFERENCE.md",
            "# Fixture\n\n## Compiled Prolog schema (v2)\n\nFixture schema.\n\n## Operating\n\nFixture procedure.\n",
        ),
        ("NOTICE", "Fixture notice.\n"),
        ("LICENSE", "Fixture license.\n"),
        ("vendor/ape/prolog/ace_to_pl.pl", "% fixture compiler\n"),
        ("vendor/clex/clex_lexicon.pl", "% fixture lexicon\n"),
        (
            "guidelines/fx/README.md",
            "# Fixture\n\n## Rights and attribution\n\nFixture text.\n",
        ),
        ("guidelines/fx/source/original.txt", "Fixture source.\n"),
        (
            "guidelines/fx/rights.tsv",
            "profile\tstatement\turl\tretrieved\tnote\nredistributable\tFixture rights.\thttps://example.invalid/id2\t2026-01-01\tFixture.\n",
        ),
        ("guidelines/fx/ace/doc-a.ace", "A patient exists.\n"),
        (
            "guidelines/fx/pl/doc-a.pl",
            "guideline_schema_version(2).\nguideline_document(fx,'doc-a',[],x).\n",
        ),
    ] {
        write(repo.join(path), body);
    }
    let mut bundle = String::from("bundle v2 doc-a\n");
    let values: Vec<_> = ["ace", "coverage", "payload", "clauses"]
        .iter()
        .map(|k| {
            let h = sha(format!("{k}:fx:doc-a"));
            bundle.push_str(&format!("{k} {h}\n"));
            h
        })
        .collect();
    write(
        repo.join("guidelines/fx/audit/review-manifest.tsv"),
        format!(
            "# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256\n# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: ckc review-manifest <id>; do not edit.\ndoc-a\t{}\t{}\n",
            values.join("\t"),
            sha(bundle)
        ),
    );
    write(
        repo.join("guidelines/fx/queries/pl/fx-query.pl"),
        readers::current_query(false),
    );
    write(
        repo.join("guidelines/fx/queries/ace/fx-query.ace"),
        "Is there a patient?\n",
    );
    commit(&repo, "fixture current release carriers");
    repo
}

#[test]
fn p1_release_metadata_is_v2_and_inserted_carrier_exports() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("release-meta");
    let repo = fixture_repo(&s);
    let mut c = Cases::default();
    let manifest = native(&repo, &["release-manifest"]);
    c.check(
        "release-manifest current carriers",
        manifest.rc == 0 && manifest.stderr.is_empty(),
        format!("{manifest:?}"),
    );
    let rows = read(repo.join("release-manifest.tsv"));
    c.check(
        "release schema identity",
        rows.lines()
            .filter(|l| l.starts_with("meta\tschema\t"))
            .collect::<Vec<_>>()
            == ["meta\tschema\tv2"],
        format!(
            "schema rows {:?}",
            rows.lines()
                .filter(|l| l.starts_with("meta\tschema\t"))
                .collect::<Vec<_>>()
        ),
    );
    commit(&repo, "fixture release manifest");
    let out = native(&repo, &["dist", "build", "dist"]);
    c.check(
        "dist current carriers",
        out.rc == 0 && out.stderr.is_empty(),
        format!("{out:?}"),
    );
    let archives: Vec<_> = fs::read_dir(repo.join("dist"))
        .into_iter()
        .flatten()
        .map(|e| e.unwrap().path())
        .filter(|p| p.to_str().unwrap().ends_with(".tar.gz"))
        .collect();
    c.check(
        "one archive",
        archives.len() == 1,
        format!("{} archives", archives.len()),
    );
    if let Some(archive) = archives.first() {
        let extracted = s.0.join("export");
        fs::create_dir_all(&extracted).unwrap();
        let unpack = run(
            Path::new("tar"),
            &s.0,
            &["-xzf".into(), text(archive), "-C".into(), text(&extracted)],
            &[],
        );
        c.same("archive extraction", unpack, Out::ok(""));
        let docs: Vec<_> = files(&extracted)
            .into_iter()
            .filter(|p| p.ends_with("guidelines/fx/pl/doc-a.pl"))
            .collect();
        c.check(
            "inserted carrier shipped",
            docs.len() == 1 && read(&docs[0]) == read(repo.join("guidelines/fx/pl/doc-a.pl")),
            "archive contains the exact identifier-2 carrier",
        );
    }
    c.finish();
}

fn old_product(frame: &str, query: bool) {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new(if query {
        "release-old-query"
    } else {
        "release-old-doc"
    });
    let repo = fixture_repo(&s);
    let mut c = Cases::default();
    let control = native(&repo, &["release-manifest"]);
    c.check(
        "release current control",
        control.rc == 0 && control.stderr.is_empty(),
        format!("{control:?}"),
    );
    let before = fs::read(repo.join("release-manifest.tsv")).unwrap();
    commit(&repo, "fixture current manifest");
    let path = if query {
        "guidelines/fx/queries/pl/fx-query.pl"
    } else {
        "guidelines/fx/pl/doc-a.pl"
    };
    let current = read(repo.join(path));
    let bad = if query {
        swap_once(&current, "'$guideline_query'(v2,", "'$guideline_query'(v1,")
    } else {
        swap_once(
            &current,
            "guideline_schema_version(2).",
            "guideline_schema_version(1).",
        )
    };
    write(repo.join(path), bad);
    commit(&repo, "fixture HEAD product with old schema identifier");
    let out = if frame == "dist" {
        native(&repo, &["dist", "build", "refused-export"])
    } else {
        native(&repo, &["release-manifest"])
    };
    let want = if frame == "dist" {
        Out::err(1, format!("dist: schema-identifier {path}\n"))
    } else {
        Out {
            rc: 1,
            stdout: format!("ckc: dist: schema-identifier {path}\n"),
            stderr: String::new(),
        }
    };
    c.same(
        &format!("{frame}/old {}", if query { "query" } else { "document" }),
        out,
        want,
    );
    c.check(
        "manifest unchanged on refusal",
        fs::read(repo.join("release-manifest.tsv")).unwrap() == before,
        "byte equality",
    );
    c.check(
        "no export assembly on refusal",
        !repo.join("refused-export").exists(),
        "destination absent",
    );
    c.finish();
}
#[test]
fn p2d_dist_refuses_staged_old_document_in_stderr_frame() {
    old_product("dist", false);
}
#[test]
fn p2d_dist_refuses_staged_old_question_in_stderr_frame() {
    old_product("dist", true);
}
#[test]
fn p2d_release_manifest_refuses_staged_old_document_in_stdout_frame() {
    old_product("release-manifest", false);
}
#[test]
fn p2d_release_manifest_refuses_staged_old_question_in_stdout_frame() {
    old_product("release-manifest", true);
}

#[test]
fn p2d_release_products_use_sorted_path_order() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("release-order");
    let repo = fixture_repo(&s);
    write(
        repo.join("guidelines/fx/pl/doc-a.pl"),
        "guideline_schema_version(1).\nguideline_document(fx,'doc-a',[],x).\n",
    );
    let qpath = repo.join("guidelines/fx/queries/pl/fx-query.pl");
    write(
        &qpath,
        read(&qpath).replace("'$guideline_query'(v2,", "'$guideline_query'(v1,"),
    );
    commit(&repo, "fixture two old products");
    let mut c = Cases::default();
    c.same(
        "first sorted product",
        native(&repo, &["release-manifest"]),
        Out {
            rc: 1,
            stdout: "ckc: dist: schema-identifier guidelines/fx/pl/doc-a.pl\n".into(),
            stderr: String::new(),
        },
    );
    c.finish();
}
