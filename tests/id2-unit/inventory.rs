use super::*;

fn seed_paths() -> Vec<String> {
    git_ok(&root(), &["ls-tree", "-r", "--name-only", SEED])
        .lines()
        .map(str::to_owned)
        .collect()
}
fn seed_bytes(path: &str) -> Vec<u8> {
    let out = Command::new("git")
        .args(["show", &format!("{SEED}:{path}")])
        .current_dir(root())
        .envs(GIT_ENV)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(
        out.status.success() && out.stderr.is_empty(),
        "historical {path}: {:?}",
        out.status.code()
    );
    out.stdout
}
fn glob(pattern: &str, path: &str) -> bool {
    let parts: Vec<_> = pattern.split('*').collect();
    if parts.len() == 1 {
        return path == pattern || (pattern.ends_with('/') && path.starts_with(pattern));
    }
    let mut rest = path;
    for (i, p) in parts.iter().enumerate() {
        if i == 0 {
            if let Some(tail) = rest.strip_prefix(p) {
                rest = tail;
            } else {
                return false;
            }
        } else if i + 1 == parts.len() {
            return rest.ends_with(p);
        } else if let Some(at) = rest.find(p) {
            rest = &rest[at + p.len()..];
        } else {
            return false;
        }
    }
    true
}
fn required_historical(path: &str) -> bool {
    path.starts_with("tests/s1-compiler/c1/")
        || path.starts_with("tests/s2-compiler/s1/")
        || (path.starts_with("tests/s1-codec/")
            && (path.ends_with("/c1.pl.in") || path.ends_with("/c1-proof.txt.in")))
        || (path.starts_with("tests/bf/probes/")
            && Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("before"))
        || [
            "tests/ui/",
            "tests/ur-ui/",
            "tests/sup4-ui/",
            "tests/frozen-pages/",
            "tests/rd/",
        ]
        .iter()
        .any(|p| path.starts_with(p))
        || ["guidelines/schema.pl", "tests/rev-k/baseline.sha256"].contains(&path)
}

#[test]
fn p1_frozen_transform_classification_and_historical_byte_identity() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let mut c = Cases::default();
    let paths = seed_paths();
    let manifest = root().join(".agent/archive/migrations/id2-transforms.tsv");
    let mut frozen = Vec::new();
    let mut classified = BTreeSet::new();
    if manifest.exists() {
        let body = read(&manifest);
        for row in body
            .lines()
            .filter(|r| !r.is_empty() && !r.starts_with('#'))
        {
            let f: Vec<_> = row.splitn(3, '\t').collect();
            if f.len() == 3 && f[1] == "disposition" {
                continue;
            }
            c.check(
                "transform row shape",
                f.len() == 3
                    && ["frozen", "retarget", "code"].contains(&f[1])
                    && !f[2].trim().is_empty(),
                row,
            );
            if f.len() != 3 {
                continue;
            }
            c.check(
                "unique transform path",
                classified.insert(f[0].to_owned()),
                f[0],
            );
            if f[1] == "frozen" {
                frozen.push(f[0].to_owned());
            }
        }
        c.check(
            "frozen classifications nonempty",
            !frozen.is_empty(),
            format!("{} frozen rows", frozen.len()),
        );
    } else {
        c.check(
            "transform manifest",
            false,
            "missing .agent/archive/migrations/id2-transforms.tsv",
        );
    }
    let historical_test = "tests/sgw/census.rs";
    let function = |body: &str| {
        let at = body
            .find("fn p3_sgw_corpus_reads_as_its_letter_spelled_predecessor() {")
            .unwrap();
        let end = at + body[at..].find("\n}\n").unwrap() + 3;
        body[at..end].to_owned()
    };
    c.check(
        "SGW P3 raw historical comparison unchanged",
        function(&read(root().join(historical_test))) == function(&seed_file(historical_test)),
        historical_test,
    );
    let protected: Vec<_> = paths
        .iter()
        .filter(|p| required_historical(p) || frozen.iter().any(|pat| glob(pat, p)))
        .collect();
    c.check(
        "historical positive control",
        protected.len() > 100,
        format!(
            "{} raw files including SHA256SUMS + page pins",
            protected.len()
        ),
    );
    for path in protected {
        let expected = seed_bytes(path);
        match fs::read(root().join(path)) {
            Ok(got) => {
                if got != expected {
                    c.check(
                        "historical byte equality",
                        false,
                        format!("{path}: sha256 {} != {}", sha(&got), sha(&expected)),
                    );
                }
            }
            Err(e) => c.check("historical path", false, format!("{path}: {e}")),
        }
    }
    c.finish();
}

#[test]
fn p1_named_intentional_negatives_preserve_their_defects() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("negatives");
    let mut c = Cases::default();
    for (case, file, version) in [
        ("q12-k1-header-over-v3", "input.pl.in", "1"),
        ("t-c908", "doc.pl.in", "1"),
        ("m7t-k1-020", "input.pl.in", "3"),
    ] {
        let input = read(root().join("tests/v1").join(case).join(file));
        c.check(
            &format!("{case}/distinguishing identifier"),
            input.contains(&format!("guideline_schema_version({version}).")),
            format!("identifier {version}"),
        );
        let case_dir = s.0.join(".scratch/m5u2/suite/cases").join(case);
        fs::create_dir_all(&case_dir).unwrap();
        for path in sorted(root().join("tests/v1").join(case)) {
            if path.is_file() && path.extension().is_some_and(|e| e == "in") {
                let name = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .strip_suffix(".in")
                    .unwrap();
                write(case_dir.join(name), fs::read(&path).unwrap());
            }
        }
        let out = if case == "t-c908" {
            native(
                &s.0,
                &[
                    "v1",
                    "answer",
                    ".scratch/m5u2/suite/cases/t-c908/manifest.tsv",
                    ".scratch/m5u2/suite/cases/t-c908/query.pl",
                ],
            )
        } else {
            native(&case_dir, &["v1", "check", "input.pl"])
        };
        let expected = if case == "t-c908" {
            "ace_to_pl_error(check_load,noncanonical('.scratch/m5u2/suite/cases/t-c908/doc.pl')).\n"
                .to_owned()
        } else {
            let at = input.find("guideline_schema_version(").unwrap()
                + "guideline_schema_version(".len();
            let line = input[..at].bytes().filter(|b| *b == b'\n').count() + 1;
            format!("ace_to_pl_error(check_load,noncanonical({line},26)).\n")
        };
        c.same(&format!("{case}/reject"), out, Out::err(2, expected));
    }
    let quoted = read(root().join("tests/v1/q17-k1-003/input.pl.in"));
    c.check(
        "quoted query current tag",
        quoted.contains("'$guideline_query'('v2',"),
        "quoted v2 stays a noncanonical-tag negative",
    );
    s.file("quoted.pl", &quoted);
    let at = quoted.find("'v2'").or_else(|| quoted.find("'v1'")).unwrap();
    let prefix = &quoted.as_bytes()[..at];
    let line = prefix.iter().filter(|b| **b == b'\n').count() + 1;
    let col = prefix
        .iter()
        .rposition(|b| *b == b'\n')
        .map_or(at + 1, |p| at - p);
    c.same(
        "quoted tag rejects at opening quote",
        native(&s.0, &["v1", "check", "quoted.pl"]),
        Out::err(
            2,
            format!("ace_to_pl_error(check_load,noncanonical({line},{col})).\n"),
        ),
    );
    c.finish();
}

fn runnable_names() -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for base in [root().join("rust/ckc/tests"), root().join("tests")] {
        for path in files(&base)
            .into_iter()
            .filter(|p| p.extension().is_some_and(|e| e == "rs"))
        {
            let body = read(path);
            let mut test = false;
            let mut ignored = false;
            for line in body.lines() {
                let line = line.trim();
                if line.starts_with("#[test]") {
                    test = true;
                }
                if line.starts_with("#[ignore") {
                    ignored = true;
                }
                if let Some(tail) = line.strip_prefix("fn ") {
                    if test
                        && !ignored
                        && let Some((name, _)) = tail.split_once('(')
                    {
                        names.insert(name.to_owned());
                    }
                    test = false;
                    ignored = false;
                }
            }
        }
    }
    names
}
#[test]
fn p3_dispositions_equal_the_seventy_retired_ids_and_name_live_tests() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let mut c = Cases::default();
    let old = read(root().join(".agent/archive/migrations/c1-dispositions.tsv"));
    let retired: BTreeSet<_> = old
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let f: Vec<_> = l.split('\t').collect();
            (f.len() >= 7 && f[6] == "retired").then(|| (f[0].to_owned(), f[1].to_owned()))
        })
        .collect();
    c.check(
        "retired source control",
        retired.len() == 70,
        format!("{} unique retired ids", retired.len()),
    );
    let path = root().join(".agent/archive/migrations/id2-dispositions.tsv");
    if !path.exists() {
        c.check(
            "ID2 disposition manifest",
            false,
            "missing .agent/archive/migrations/id2-dispositions.tsv",
        );
        c.finish();
        return;
    }
    let body = read(path);
    let mut rows = body
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'));
    c.check(
        "disposition header",
        rows.next() == Some("family\tid\tdisposition\trepresentative"),
        "family, id, disposition, representative",
    );
    let names = runnable_names();
    let mut keys = BTreeSet::new();
    for row in rows {
        let f: Vec<_> = row.split('\t').collect();
        c.check("disposition row shape", f.len() == 4, row);
        if f.len() != 4 {
            continue;
        }
        c.check(
            "disposition uniqueness",
            keys.insert((f[0].to_owned(), f[1].to_owned())),
            format!("{} / {}", f[0], f[1]),
        );
        match f[2] {
            "re-derived" => {
                let name = f[3].rsplit("::").next().unwrap();
                c.check(
                    "representative runnable test exists",
                    names.contains(name),
                    f[3],
                );
            }
            "obsolete" => c.check("obsolete law reason", !f[3].trim().is_empty(), f[3]),
            other => c.check("known disposition", false, other),
        }
    }
    c.check(
        "retired-set equality",
        keys == retired,
        format!(
            "missing={:?}; extra={:?}",
            retired.difference(&keys).collect::<Vec<_>>(),
            keys.difference(&retired).collect::<Vec<_>>()
        ),
    );
    c.finish();
}

#[test]
fn p1_emitter_version_arguments_remain_usage_errors() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("emitter-argv");
    let mut c = Cases::default();
    for version in ["v1", "v2", "v3"] {
        for question in [false, true] {
            let mut args = vec![
                "-q".into(),
                "-f".into(),
                "none".into(),
                "-F".into(),
                "none".into(),
                "-s".into(),
                text(&root().join("vendor/ape/prolog/ace_to_pl.pl")),
                "-g".into(),
                "main".into(),
                "-t".into(),
                "halt(9)".into(),
                "--".into(),
            ];
            if question {
                args.push("question".into());
            }
            args.extend(
                [
                    version,
                    "missing-tree",
                    "fixture",
                    "missing-table",
                    "missing-ulex",
                ]
                .map(str::to_owned),
            );
            let out = run(&readers::real_swipl(), &s.0, &args, &[]);
            c.check(
                &format!("emitter argv/{version}/question={question}"),
                out.rc == 2
                    && out.stdout.is_empty()
                    && out.stderr.starts_with("ace_to_pl_error(usage,argv("),
                format!("{out:?}"),
            );
        }
    }
    c.finish();
}

#[test]
fn p1_cli_protocol_name_stays_v1() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("cli-name");
    let mut c = Cases::default();
    let out = native(&s.0, &["--help"]);
    c.check(
        "CLI v1 name",
        out.rc == 0 && out.stderr.is_empty() && out.stdout.contains("ckc v1 "),
        format!("rc={} has ckc v1 help", out.rc),
    );
    for old in ["v2", "v3"] {
        let out = native(&s.0, &[old, "check", "missing"]);
        c.check(
            &format!("{old} is not a CLI selector"),
            out.rc == 2 && out.stdout.is_empty() && out.stderr.starts_with("usage: ckc "),
            format!("{out:?}"),
        );
    }
    c.finish();
}
