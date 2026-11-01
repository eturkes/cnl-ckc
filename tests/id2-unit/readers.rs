use super::*;

pub(super) fn current_document(id: &str, table: bool) -> String {
    let doc = seed_file("tests/certify-one/doc/pl.txt.in")
        .replace("'fx-doc'", &atom(id))
        .replace("fx-doc", id);
    let doc = swap_once(
        &doc,
        "guideline_schema_version(1).",
        "guideline_schema_version(2).",
    );
    custody(&doc, table)
}
pub(super) fn current_query(table: bool) -> String {
    let q = seed_file("tests/certify-one/query/pl.txt.in");
    custody(
        &swap_once(&q, "'$guideline_query'(v1,", "'$guideline_query'(v2,"),
        table,
    )
}
fn custody(input: &str, table: bool) -> String {
    if table {
        swap_once(
            input,
            "vocabulary(none)",
            &format!(
                "vocabulary(sha256({}))",
                atom(&sha("# format: kind\tlemma\tvalue\nunit\thour\thour\n"))
            ),
        )
    } else {
        input.into()
    }
}
fn old_document(doc: &str) -> String {
    swap_once(
        doc,
        "guideline_schema_version(2).",
        "guideline_schema_version(1).",
    )
}
fn old_query(q: &str) -> String {
    swap_once(q, "'$guideline_query'(v2,", "'$guideline_query'(v1,")
}
pub(super) fn answers(q: &str) -> String {
    format!(
        "% fx-query answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,'fx-query',query_sha256({}),result(no(finite_failure))).\n",
        atom(&sha(q))
    )
}
pub(super) fn traces(q: &str, a: &str) -> String {
    format!(
        "% fx-query traced against the loaded composition by ckc queries; do not edit.\n'$guideline_traces'(v1,'fx-query',query_sha256({}),answers_sha256({}),result(no(finite_failure))).\n",
        atom(&sha(q)),
        atom(&sha(a))
    )
}
fn k1_error(canonical: &str, mutant: &str) -> String {
    let at = canonical
        .bytes()
        .zip(mutant.bytes())
        .position(|(a, b)| a != b)
        .unwrap();
    let prefix = &mutant.as_bytes()[..at];
    let line = prefix.iter().filter(|&&b| b == b'\n').count() + 1;
    let col = prefix
        .iter()
        .rposition(|&b| b == b'\n')
        .map_or(at + 1, |p| at - p);
    format!("ace_to_pl_error(check_load,noncanonical({line},{col})).\n")
}

fn k1(mode: &str) {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new(mode);
    let mut c = Cases::default();
    for table in [false, true] {
        let doc = current_document("fx-doc", table);
        let q = current_query(table);
        for (kind, valid, old) in [
            ("doc", doc.clone(), old_document(&doc)),
            ("query", q.clone(), old_query(&q)),
        ] {
            s.file("input.pl", &valid);
            c.same(
                &format!("{mode}/{kind}/table={table}/current"),
                native(&s.0, &["v1", mode, "input.pl"]),
                Out::ok(if mode == "render" {
                    valid.clone()
                } else {
                    String::new()
                }),
            );
            s.file("input.pl", &old);
            c.same(
                &format!("{mode}/{kind}/table={table}/old"),
                native(&s.0, &["v1", mode, "input.pl"]),
                Out::err(2, k1_error(&valid, &old)),
            );
        }
    }
    c.finish();
}
#[test]
fn p2a_k1_check_offsets_and_custody_profiles() {
    k1("check");
}
#[test]
fn p2a_k1_render_offsets_and_custody_profiles() {
    k1("render");
}

// D1's sole admitted identifier + D2's first-differing-byte property, over numeric/tag domains.
#[test]
fn p2a_k1_identifier_domain_first_difference_property() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("identifier-domain");
    let mut c = Cases::default();
    let doc = current_document("fx-doc", false);
    let q = current_query(false);
    let integers: Vec<_> = (-3..=33)
        .filter(|n| *n != 2)
        .map(|n| n.to_string())
        .chain(["18446744073709551616".into(), "9".repeat(128)])
        .collect();
    for id in integers {
        let bad = swap_once(
            &doc,
            "guideline_schema_version(2).",
            &format!("guideline_schema_version({id})."),
        );
        s.file("input.pl", &bad);
        c.same(
            &format!("document identifier {id}"),
            native(&s.0, &["v1", "check", "input.pl"]),
            Out::err(2, k1_error(&doc, &bad)),
        );
    }
    for id in ["v0", "v1", "v3", "v22", "'v2'"] {
        let bad = swap_once(
            &q,
            "'$guideline_query'(v2,",
            &format!("'$guideline_query'({id},"),
        );
        s.file("input.pl", &bad);
        c.same(
            &format!("query identifier {id}"),
            native(&s.0, &["v1", "check", "input.pl"]),
            Out::err(2, k1_error(&q, &bad)),
        );
    }
    c.finish();
}

fn install(s: &Scratch, table: bool) {
    for id in ["alpha", "beta"] {
        let doc = current_document(id, table);
        s.file(&format!("{id}.pl"), &doc);
        s.file(&format!("old-{id}.pl"), old_document(&doc));
        s.file(
            &format!("{id}.proof"),
            seed_file("tests/certify-one/doc/stdout.expect").replace("'fx-doc'", &atom(id)),
        );
    }
}
fn manifest(s: &Scratch, docs: &[&str]) {
    s.file(
        "manifest.tsv",
        docs.iter()
            .map(|name| {
                let id = name.strip_prefix("old-").unwrap_or(name);
                format!("{name}.pl\t{id}.proof\n")
            })
            .collect::<String>(),
    );
}
fn composition_error(path: &str) -> Out {
    Out::err(
        2,
        format!("ace_to_pl_error(check_load,noncanonical('{path}')).\n"),
    )
}
fn query_error() -> Out {
    Out::err(2, "ace_to_pl_error(check_load,query_file(noncanonical)).\n")
}
fn install_query(s: &Scratch, q: &str) {
    let a = answers(q);
    let t = traces(q, &a);
    s.file("query.pl", q);
    s.file("answers.pl", a);
    s.file("traces.pl", t);
}
fn args(mode: &str) -> Vec<&str> {
    let mut a = vec!["v1", mode, "manifest.tsv"];
    if ["answer", "trace", "trace-check"].contains(&mode) {
        a.push("query.pl");
    }
    if ["trace", "trace-check"].contains(&mode) {
        a.push("answers.pl");
    }
    if mode == "trace-check" {
        a.push("traces.pl");
    }
    a
}
fn valid_result(mode: &str, docs: usize, q: &str) -> Out {
    match mode {
        "aggregate-check" => Out::ok(format!(
            "ace_to_pl aggregate ok {docs} documents {docs} obligations\n"
        )),
        "recursion-check" => Out::ok(format!(
            "ace_to_pl recursion ok {docs} documents {} rule clauses\n",
            docs * 5
        )),
        "answer" => Out::ok(answers(q)),
        "trace" => Out::ok(traces(q, &answers(q))),
        "trace-check" => Out::err(1, "ace_to_pl_error(proof,trace_check(non_demo)).\n"),
        _ => unreachable!(),
    }
}
fn composition_reader(mode: &str) {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new(mode);
    let mut c = Cases::default();
    for table in [false, true] {
        install(&s, table);
        let q = current_query(table);
        install_query(&s, &q);
        manifest(&s, &["alpha", "beta"]);
        c.same(
            &format!("{mode}/table={table}/current"),
            native(&s.0, &args(mode)),
            valid_result(mode, 2, &q),
        );
        for (label, docs, first) in [
            ("old alone", vec!["old-alpha"], "old-alpha"),
            ("old first", vec!["old-alpha", "beta"], "old-alpha"),
            ("old last", vec!["alpha", "old-beta"], "old-beta"),
            (
                "manifest order (not lexical)",
                vec!["old-beta", "old-alpha"],
                "old-beta",
            ),
        ] {
            manifest(&s, &docs);
            c.same(
                &format!("{mode}/table={table}/{label}"),
                native(&s.0, &args(mode)),
                composition_error(&format!("{first}.pl")),
            );
        }
        if ["answer", "trace", "trace-check"].contains(&mode) {
            let old = old_query(&q);
            for (label, docs) in [
                ("current composition", vec!["alpha"]),
                ("empty composition", vec![]),
            ] {
                manifest(&s, &docs);
                install_query(&s, &old);
                c.same(
                    &format!("{mode}/table={table}/old query/{label}"),
                    native(&s.0, &args(mode)),
                    query_error(),
                );
            }
            manifest(&s, &[]);
            install_query(&s, &q);
            c.same(
                &format!("{mode}/table={table}/current empty law"),
                native(&s.0, &args(mode)),
                valid_result(mode, 0, &q),
            );
        }
    }
    c.finish();
}
#[test]
fn p2a_aggregate_identifier_refusal_and_manifest_order() {
    composition_reader("aggregate-check");
}
#[test]
fn p2a_recursion_identifier_refusal_and_manifest_order() {
    composition_reader("recursion-check");
}
#[test]
fn p2a_answer_identifier_refusals_and_empty_law() {
    composition_reader("answer");
}
#[test]
fn p2a_trace_identifier_refusals_and_empty_law() {
    composition_reader("trace");
}
#[test]
fn p2a_trace_check_identifier_refusals_and_empty_law() {
    composition_reader("trace-check");
}

#[test]
fn p2b_manifest_then_query_then_composition_precedence() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("precedence");
    let mut c = Cases::default();
    for table in [false, true] {
        install(&s, table);
        install_query(&s, &old_query(&current_query(table)));
        for mode in ["answer", "trace", "trace-check"] {
            manifest(&s, &["old-alpha"]);
            c.same(
                &format!("{mode}/table={table}/both old"),
                native(&s.0, &args(mode)),
                query_error(),
            );
            s.file("manifest.tsv", "missing-tab\n");
            let prior = native(&s.0, &args(mode));
            // The manifest envelope is not respelled by ID2. Compare the same manifest fault
            // with a current query + current document instead of assuming an unpinned detail.
            install_query(&s, &current_query(table));
            let control = native(&s.0, &args(mode));
            c.check(
                &format!("{mode}/manifest control"),
                control.rc == 2
                    && control.stdout.is_empty()
                    && control.stderr.contains("aggregate_manifest("),
                format!("{control:?}"),
            );
            c.same(
                &format!("{mode}/table={table}/manifest first"),
                prior,
                control,
            );
            install_query(&s, &old_query(&current_query(table)));
        }
    }
    c.finish();
}

#[test]
fn p2a_certify_one_old_document_and_question_envelopes() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("certify-one");
    let mut c = Cases::default();
    for (case, mode, id) in [
        ("doc", "doc", "fx-doc"),
        ("v3", "doc", "fx-timed"),
        ("query", "query", "fx-query"),
    ] {
        let base = format!("tests/certify-one/{case}");
        for (src, dst) in [("ace.txt.in", "input.ace"), ("dump.txt.in", "input.dump")] {
            s.file(dst, seed_file(&format!("{base}/{src}")));
        }
        let seed = seed_file(&format!("{base}/pl.txt.in"));
        let current = if mode == "doc" {
            swap_once(
                &seed,
                "guideline_schema_version(1).",
                "guideline_schema_version(2).",
            )
        } else {
            swap_once(&seed, "'$guideline_query'(v1,", "'$guideline_query'(v2,")
        };
        let mut argv = vec![
            "certify-one",
            mode,
            id,
            "input.ace",
            "-",
            "input.dump",
            "input.pl",
        ];
        if case == "v3" {
            s.file(
                "vocabulary.tsv",
                seed_file(&format!("{base}/vocabulary.tsv.in")),
            );
            argv.push("vocabulary.tsv");
        }
        s.file("input.pl", current);
        c.same(
            &format!("certify-one/{case}/current"),
            native(&s.0, &argv),
            Out::ok(seed_file(&format!("{base}/stdout.expect"))),
        );
        s.file("input.pl", seed);
        c.same(
            &format!("certify-one/{case}/old"),
            native(&s.0, &argv),
            Out::err(1, format!("ckc: certify: {id}: noncanonical.\n")),
        );
    }
    c.finish();
}

#[test]
fn p1_answers_traces_keep_v1_and_wrong_protocol_refuses() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("protocol");
    let mut c = Cases::default();
    let q = current_query(false);
    let a = answers(&q);
    let t = traces(&q, &a);
    for (kind, valid, from, to) in [
        (
            "answers",
            a,
            "'$guideline_answers'(v1,",
            "'$guideline_answers'(v2,",
        ),
        (
            "traces",
            t,
            "'$guideline_traces'(v1,",
            "'$guideline_traces'(v2,",
        ),
    ] {
        s.file("input.pl", &valid);
        c.same(
            &format!("{kind}/v1"),
            native(&s.0, &["v1", "check", "input.pl"]),
            Out::ok(""),
        );
        let bad = swap_once(&valid, from, to);
        s.file("input.pl", &bad);
        c.same(
            &format!("{kind}/v2"),
            native(&s.0, &["v1", "check", "input.pl"]),
            Out::err(2, k1_error(&valid, &bad)),
        );
    }
    for gid in sorted(root().join("guidelines"))
        .into_iter()
        .filter(|p| p.is_dir())
    {
        for (sub, marker) in [
            ("answers", "'$guideline_answers'(v1,"),
            ("traces", "'$guideline_traces'(v1,"),
        ] {
            for path in pl_files(gid.join("queries").join(sub)) {
                c.check(
                    &format!("protocol {}", path.strip_prefix(root()).unwrap().display()),
                    read(&path).matches(marker).count() == 1,
                    "one v1 protocol record",
                );
            }
        }
    }
    c.finish();
}

fn generator_repo(s: &Scratch) -> PathBuf {
    let path = s.0.join("repo");
    fs::create_dir_all(path.join("rust/ckc")).unwrap();
    fs::create_dir_all(path.join("vendor/clex")).unwrap();
    symlink(root().join("rust/ckc/prolog"), path.join("rust/ckc/prolog")).unwrap();
    symlink(root().join("vendor/ape"), path.join("vendor/ape")).unwrap();
    fs::copy(
        root().join("vendor/clex/clex_lexicon.pl"),
        path.join("vendor/clex/clex_lexicon.pl"),
    )
    .unwrap();
    let out = run(
        Path::new("cp"),
        &s.0,
        &[
            "-a".into(),
            text(&root().join("tests/queries/green/positive/tree/guidelines")),
            text(&path.join("guidelines")),
        ],
        &[],
    );
    assert_eq!(out, Out::ok(""));
    path
}
pub(super) fn real_swipl() -> PathBuf {
    std::env::var_os("SWIPL")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::split_paths(&std::env::var_os("PATH").unwrap())
                .map(|p| p.join("swipl"))
                .find(|p| fs::metadata(p).is_ok_and(|m| m.permissions().mode() & 0o111 != 0))
                .unwrap()
        })
}
fn generator(mode: &str) {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new(mode);
    let repo = generator_repo(&s);
    let mut c = Cases::default();
    // Today's fixture seeds are old products. A current compiler must replace, not admit, them.
    let compile = native(&repo, &["compile", "fx"]);
    c.check(
        "generator compile control",
        compile.rc == 0 && compile.stderr.is_empty(),
        format!("{compile:?}"),
    );
    let product = repo.join(if mode == "compile" {
        "guidelines/fx/pl/d1.pl"
    } else {
        "guidelines/fx/queries/pl/q1.pl"
    });
    let products = if mode == "compile" {
        pl_files(repo.join("guidelines/fx/pl"))
    } else {
        pl_files(repo.join("guidelines/fx/queries/pl"))
    };
    assert!(!products.is_empty(), "generator fixture products");
    let product = if product.exists() {
        product
    } else {
        products[0].clone()
    };
    let old = if mode == "queries" {
        read(&product).replace("'$guideline_query'(v2,", "'$guideline_query'(v1,")
    } else {
        read(&product).replace(
            "guideline_schema_version(2).",
            "guideline_schema_version(1).",
        )
    };
    write(&product, old);
    let generated = native(&repo, &[mode, "fx"]);
    c.check(
        &format!("{mode}/old products overwritten"),
        generated.rc == 0 && generated.stderr.is_empty(),
        format!("{generated:?}"),
    );
    let token = if mode == "compile" {
        "guideline_schema_version(2)."
    } else {
        "'$guideline_query'(v2,"
    };
    c.check(
        &format!("{mode}/new token"),
        read(&product).contains(token),
        format!("{} contains {token}", product.display()),
    );
    let wrapper = s.file("swipl-plant", format!(
        "#!/bin/sh\nt='{tmp}'.$$\ntrap 'rm -f \"$t\"' EXIT HUP INT TERM\n'{real}' \"$@\" > \"$t\"\nr=$?\nif [ \"$r\" -ne 0 ]; then cat \"$t\"; exit \"$r\"; fi\nif grep -Eq 'guideline_schema_version\\([12]\\)\\.|guideline_query.*\\(v[12],' \"$t\"; then printf 'planted\\n' >> '{log}'; fi\nsed -e 's/guideline_schema_version([12])\\./guideline_schema_version(1)./' -e \"s/\\('\\$guideline_query'(\\)v[12],/\\1v1,/\" \"$t\"\n",
        tmp = s.0.join("capture").display(), real = real_swipl().display(), log = s.0.join("planted.log").display()
    ));
    fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o755)).unwrap();
    let planted = run(
        &program(),
        &repo,
        &[mode.into(), "fx".into()],
        &[("SWIPL", text(&wrapper))],
    );
    c.check(
        &format!("{mode}/emitter reached"),
        s.0.join("planted.log").exists(),
        "wrapper observed a document/query record",
    );
    // D2 pins refusal/validation here, not a new generator-specific diagnostic spelling.
    c.check(
        &format!("{mode}/malformed new output refused"),
        planted.rc != 0 && planted.rc != 124,
        format!("{planted:?}"),
    );
    c.finish();
}
#[test]
fn p2c_compile_validates_new_output_and_overwrites_old_products() {
    generator("compile");
}
#[test]
fn p2c_queries_validates_new_output_and_overwrites_old_products() {
    generator("queries");
}
