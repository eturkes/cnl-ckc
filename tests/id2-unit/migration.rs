use super::*;
use std::collections::BTreeMap;
use std::os::unix::ffi::OsStrExt;

const TRANSFORMS: &str = ".agent/archive/migrations/id2-transforms.tsv";
const KANI: &str = "rust/ckc-kani-harness/src/lib.rs";
const RELEASE: &str = "release-manifest.tsv";

fn repo_at(s: &Scratch, name: &str, revision: &str) -> PathBuf {
    let path = s.0.join(name);
    fs::create_dir_all(&path).unwrap();
    git_ok(&path, &["init", "-q", "-b", "fixture"]);
    let revision = git_ok(&root(), &["rev-parse", &format!("{revision}^{{commit}}")]);
    git_ok(
        &path,
        &[
            "-c",
            "protocol.file.allow=always",
            "fetch",
            "--quiet",
            "--depth=1",
            "--no-tags",
            &text(&root()),
            revision.trim(),
        ],
    );
    git_ok(&path, &["checkout", "--quiet", "--detach", "FETCH_HEAD"]);
    path
}
fn seed_repo(s: &Scratch, name: &str) -> PathBuf {
    repo_at(s, name, SEED)
}

#[derive(Default)]
struct Census {
    code: Vec<String>,
}
impl Census {
    fn from_head(repo: &Path) -> Self {
        if git_ok(repo, &["ls-tree", "--name-only", "HEAD", "--", TRANSFORMS]).is_empty() {
            return Self::default();
        }
        let body = git_ok(repo, &["show", &format!("HEAD:{TRANSFORMS}")]);
        let mut code = Vec::new();
        for row in body
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
        {
            let f: Vec<_> = row.splitn(3, '\t').collect();
            if f.len() == 3 && f[1] == "disposition" {
                continue;
            }
            assert!(
                f.len() == 3 && !f[0].is_empty() && ["frozen", "retarget", "code"].contains(&f[1]),
                "transform row: {row}"
            );
            if f[1] == "code" {
                code.push(f[0].to_owned());
            }
        }
        Self { code }
    }
    fn contains(&self, path: &str, bridge: bool) -> bool {
        if path == KANI {
            return true;
        }
        if path == RELEASE {
            return !bridge;
        }
        (path.starts_with("guidelines/") || path.starts_with("tests/"))
            && !path.ends_with(".rs")
            && !self.code.iter().any(|rule| path_rule(rule, path))
    }
}
fn path_rule(rule: &str, path: &str) -> bool {
    let parts: Vec<_> = rule.split('*').collect();
    if parts.len() == 1 {
        return path == rule || path.starts_with(&format!("{}/", rule.trim_end_matches('/')));
    }
    let mut tail = path;
    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            let Some(rest) = tail.strip_prefix(part) else {
                return false;
            };
            tail = rest;
        } else if i + 1 == parts.len() {
            return tail.ends_with(part);
        } else {
            let Some(at) = tail.find(part) else {
                return false;
            };
            tail = &tail[at + part.len()..];
        }
    }
    true
}

#[derive(Clone, PartialEq, Eq)]
struct Artifact {
    mode: u32,
    bytes: Vec<u8>,
}
type Projection = BTreeMap<String, Artifact>;
fn tracked(repo: &Path) -> Vec<String> {
    git_ok(repo, &["ls-files", "-z"])
        .split_terminator('\0')
        .map(str::to_owned)
        .collect()
}
fn projection(repo: &Path, census: &Census, bridge: bool) -> Projection {
    tracked(repo)
        .into_iter()
        .filter(|p| census.contains(p, bridge))
        .filter_map(|name| {
            let path = repo.join(&name);
            let metadata = match fs::symlink_metadata(&path) {
                Ok(m) => m,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => return None,
                Err(e) => panic!("projection {name}: {e}"),
            };
            let artifact = if metadata.file_type().is_symlink() {
                Artifact {
                    mode: 0o120000,
                    bytes: fs::read_link(&path)
                        .unwrap()
                        .as_os_str()
                        .as_encoded_bytes()
                        .to_vec(),
                }
            } else {
                assert!(metadata.is_file(), "projection non-file: {name}");
                Artifact {
                    mode: if metadata.permissions().mode() & 0o100 == 0 {
                        0o100644
                    } else {
                        0o100755
                    },
                    bytes: fs::read(&path).unwrap(),
                }
            };
            Some((name, artifact))
        })
        .collect()
}
fn projection_diff(expected: &Projection, got: &Projection) -> Vec<String> {
    let paths: BTreeSet<_> = expected.keys().chain(got.keys()).collect();
    paths
        .into_iter()
        .filter_map(|path| match (expected.get(path), got.get(path)) {
            (Some(a), Some(b)) if a == b => None,
            (Some(a), Some(b)) => Some(format!(
                "{path}: modes {:o}/{:o}; sha256 {}/{}",
                a.mode,
                b.mode,
                sha(&a.bytes),
                sha(&b.bytes)
            )),
            (Some(_), None) => Some(format!("{path}: missing")),
            (None, Some(_)) => Some(format!("{path}: extra")),
            (None, None) => unreachable!(),
        })
        .collect()
}
fn projection_detail(expected: &Projection, got: &Projection) -> String {
    let diff = projection_diff(expected, got);
    format!(
        "{} expected paths, {} actual paths, {} differences{}",
        expected.len(),
        got.len(),
        diff.len(),
        if diff.is_empty() {
            String::new()
        } else {
            format!(
                ": {}",
                diff.iter().take(5).cloned().collect::<Vec<_>>().join("; ")
            )
        }
    )
}
fn remove_path(path: &Path) {
    match fs::symlink_metadata(path) {
        Ok(m) if m.is_dir() => fs::remove_dir(path).unwrap(),
        Ok(_) => fs::remove_file(path).unwrap(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => panic!("remove {}: {e}", path.display()),
    }
}
fn restore_census(seed: &Path, repo: &Path, census: &Census) -> Projection {
    let expected = projection(seed, census, false);
    let current = projection(repo, census, false);
    for name in current.keys().filter(|p| !expected.contains_key(*p)) {
        remove_path(&repo.join(name));
    }
    for (name, artifact) in &expected {
        if current.get(name) == Some(artifact) {
            continue;
        }
        let path = repo.join(name);
        remove_path(&path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        if artifact.mode == 0o120000 {
            symlink(std::ffi::OsStr::from_bytes(&artifact.bytes), &path).unwrap();
        } else {
            fs::write(&path, &artifact.bytes).unwrap();
            fs::set_permissions(
                &path,
                fs::Permissions::from_mode(if artifact.mode == 0o100755 {
                    0o755
                } else {
                    0o644
                }),
            )
            .unwrap();
        }
    }
    expected
}
fn commit_pre_migration(repo: &Path) {
    git_ok(repo, &["add", "-A"]);
    git_ok(
        repo,
        &[
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "fixture ID2 pre-migration state",
        ],
    );
}
struct PreMigration {
    repo: PathBuf,
    closing_head: String,
    census: Census,
}
fn pre_migration(s: &Scratch, name: &str, seed: &Path) -> PreMigration {
    let repo = repo_at(s, name, "HEAD");
    let closing_head = git_ok(&repo, &["rev-parse", "HEAD"]).trim().to_owned();
    let census = Census::from_head(&repo);
    let expected = restore_census(seed, &repo, &census);
    commit_pre_migration(&repo);
    let actual = projection(&repo, &census, false);
    let detail = projection_detail(&expected, &actual);
    assert!(
        expected == actual,
        "pre-migration {name}/seed projection: {detail}"
    );
    println!(
        "pre-migration {name}/seed projection: ok {detail}; code exclusions={}",
        census.code.len()
    );
    PreMigration {
        repo,
        closing_head,
        census,
    }
}
fn migrate(repo: &Path) -> Out {
    run(
        Path::new("python3"),
        repo,
        &[
            "-I".into(),
            text(&script("CKC_ID2_MIGRATION", "revise.py")),
            text(repo),
            text(&program()),
            "--identifier".into(),
            "2".into(),
        ],
        &[("CKC_BIN", text(&program()))],
    )
}
fn oracle(base: &Path, tip: &Path) -> Out {
    run(
        Path::new("python3"),
        &root(),
        &[
            "-I".into(),
            text(&script("CKC_ID2_CHURN", "id2-churn.py")),
            text(base),
            text(tip),
        ],
        &[],
    )
}
fn tree(repo: &Path) -> String {
    git_ok(repo, &["add", "-A"]);
    git_ok(repo, &["write-tree"]).trim().to_owned()
}
fn untracked(repo: &Path) -> String {
    git_ok(repo, &["ls-files", "--others", "--exclude-standard"])
}

// D6/D7 harness control: census selection + physical byte/mode/path plants, independent of migration.
#[test]
fn p5_projection_and_seed_reset_track_bytes_modes_paths_and_code_exclusions() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("projection-control");
    let seed = s.0.join("seed-model");
    let head = s.0.join("head-model");
    let document = "guidelines/fx/pl/doc.pl";
    for repo in [&seed, &head] {
        fs::create_dir_all(repo).unwrap();
        git_ok(repo, &["init", "-q", "-b", "fixture"]);
        for (path, bytes) in [
            (document, "seed-data\n"),
            ("tests/data.in", "seed-input\n"),
            ("tests/kept.rs", "seed Rust code\n"),
            ("tests/unit-code.in", "seed code\n"),
            (KANI, "seed Kani\n"),
            (RELEASE, "seed release\n"),
        ] {
            write(repo.join(path), bytes);
        }
        symlink("data.in", repo.join("tests/link.in")).unwrap();
        write(
            repo.join(TRANSFORMS),
            format!(
                "path-or-prefix\tdisposition\treason\ntests/unit-code.in\tcode\tunit carrier\ntests/unit-files/\tcode\tunit directory\n{KANI}\tcode\texplicit Kani census exception\n"
            ),
        );
        commit(repo, "fixture projection seed");
    }
    write(head.join(document), "changed data\n");
    fs::set_permissions(head.join(document), fs::Permissions::from_mode(0o755)).unwrap();
    fs::remove_file(head.join("tests/data.in")).unwrap();
    write(head.join("tests/new-data.in"), "remove: absent at seed\n");
    for path in [
        "tests/kept.rs",
        "tests/unit-code.in",
        "tests/unit-files/new.in",
    ] {
        write(head.join(path), "unit code\n");
        fs::set_permissions(head.join(path), fs::Permissions::from_mode(0o755)).unwrap();
    }
    fs::remove_file(head.join("tests/link.in")).unwrap();
    symlink("kept.rs", head.join("tests/link.in")).unwrap();
    write(head.join(KANI), "changed Kani\n");
    write(head.join(RELEASE), "changed release\n");
    commit(&head, "fixture unit code + migrated data");
    let census = Census::from_head(&head);
    let expected = projection(&seed, &census, false);
    let expected_paths: BTreeSet<_> = [document, "tests/data.in", "tests/link.in", KANI, RELEASE]
        .into_iter()
        .map(str::to_owned)
        .collect();
    assert!(
        expected.keys().cloned().collect::<BTreeSet<_>>() == expected_paths,
        "projection path domain"
    );
    let doc = &expected[document];
    assert!(
        doc.mode == 0o100644 && doc.bytes == b"seed-data\n",
        "projection raw bytes + mode"
    );
    assert!(
        expected["tests/link.in"].mode == 0o120000 && expected["tests/link.in"].bytes == b"data.in",
        "projection symlink bytes + mode"
    );
    let reset = restore_census(&seed, &head, &census);
    commit_pre_migration(&head);
    let restored = projection(&head, &census, false);
    assert!(
        reset == expected && restored == expected,
        "seed reset: {}",
        projection_detail(&expected, &restored)
    );
    assert!(
        !head.join("tests/new-data.in").exists(),
        "seed-absent data removed"
    );
    for path in [
        "tests/kept.rs",
        "tests/unit-code.in",
        "tests/unit-files/new.in",
    ] {
        assert!(
            read(head.join(path)) == "unit code\n"
                && fs::metadata(head.join(path)).unwrap().permissions().mode() & 0o100 != 0,
            "unit code retained: {path}"
        );
    }
    for plant in ["bytes", "mode", "missing", "extra"] {
        let extra = "tests/bridge-extra.in";
        match plant {
            "bytes" => {
                write(head.join(document), "byte plant\n");
            }
            "mode" => {
                fs::set_permissions(head.join(document), fs::Permissions::from_mode(0o755)).unwrap()
            }
            "missing" => fs::remove_file(head.join(document)).unwrap(),
            "extra" => {
                write(head.join(extra), "path plant\n");
                git_ok(&head, &["add", "--", extra]);
            }
            _ => unreachable!(),
        }
        let changed = projection(&head, &census, true);
        let baseline = projection(&seed, &census, true);
        assert!(changed != baseline, "bridge misses {plant} plant");
        if plant == "extra" {
            fs::remove_file(head.join(extra)).unwrap();
            git_ok(
                &head,
                &["rm", "--quiet", "--cached", "--force", "--", extra],
            );
        }
        let reset = restore_census(&seed, &head, &census);
        commit_pre_migration(&head);
        assert!(
            projection(&head, &census, false) == reset,
            "restore {plant} plant"
        );
        println!("projection control/{plant}: ok bridge rejects and seed reset restores");
    }
    let bridge_seed = projection(&seed, &census, true);
    write(head.join(RELEASE), "P-d release amend\n");
    assert!(
        projection(&head, &census, true) == bridge_seed,
        "release manifest outside bridge"
    );
    assert!(
        projection(&head, &census, false) != expected,
        "release manifest inside migration census"
    );
    write(head.join(KANI), "Kani bridge plant\n");
    assert!(
        projection(&head, &census, true) != bridge_seed,
        "Kani remains inside bridge despite .rs + code row"
    );
    println!(
        "projection control: ok bytes, modes, path set, symlink, .rs/code exclusions, explicit Kani + release scopes"
    );
}

#[test]
#[ignore = "one-time ID2 migration rerun through the current compiler: user ruling, run once at the ID2 commit, then manual (.agent/deferred.md, ID2 migration reruns)"]
fn p5_identifier_migration_two_seed_copies_and_rerun_are_byte_identical() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("idempotence");
    let seed = seed_repo(&s, "seed");
    let a = pre_migration(&s, "a", &seed).repo;
    let b = pre_migration(&s, "b", &seed).repo;
    let mut c = Cases::default();
    let mut migrated = true;
    for (label, repo) in [("copy a", &a), ("copy b", &b)] {
        let out = migrate(repo);
        migrated &= out.rc == 0;
        c.check(
            &format!("migration {label}"),
            out.rc == 0,
            format!("{out:?}"),
        );
        let census = untracked(repo);
        c.check(
            &format!("{label}/untracked census"),
            census.is_empty(),
            format!("{census:?}"),
        );
    }
    if !migrated {
        c.finish();
        return;
    }
    let first = tree(&a);
    c.check(
        "two fresh trees (bytes + modes)",
        first == tree(&b),
        format!("tree a={first}, tree b={}", tree(&b)),
    );
    let out = migrate(&a);
    c.check("rerun exits zero", out.rc == 0, format!("{out:?}"));
    c.check(
        "rerun untracked census",
        untracked(&a).is_empty(),
        "no untracked files",
    );
    c.check(
        "rerun tree unchanged",
        first == tree(&a),
        format!("before={first}; after={}", tree(&a)),
    );
    c.finish();
}

fn replace_first(path: &Path, from: &str, to: &str) {
    let body = read(path);
    assert!(
        body.contains(from),
        "plant missing anchor {}: {from}",
        path.display()
    );
    write(path, body.replacen(from, to, 1));
}
fn result_body(path: &Path, replacement: &str) {
    let body = read(path);
    let at = body.find("result(").expect("result anchor") + "result(".len();
    // Answers/traces end in `result(<term>)).`; the independent plant replaces only that term.
    assert!(body.ends_with(")).\n"), "record ending {}", path.display());
    write(path, format!("{}{replacement})).\n", &body[..at]));
}
fn mutate_digest(path: &Path) {
    let body = read(path);
    let at = body.find("query_sha256(").unwrap() + "query_sha256(".len();
    let at = at + usize::from(body.as_bytes()[at] == b'\'');
    let mut bytes = body.into_bytes();
    bytes[at] = if bytes[at] == b'a' { b'b' } else { b'a' };
    write(path, bytes);
}
fn oracle_case(c: &mut Cases, label: &str, base: &Path, tip: &Path, path: &str) {
    let out = oracle(base, tip);
    c.check(
        label,
        out.rc == 1
            && out
                .stdout
                .lines()
                .chain(out.stderr.lines())
                .any(|line| line.contains(path)),
        format!(
            "rc={} stdout={:?} stderr={:?}; expected rc1 naming {path}",
            out.rc, out.stdout, out.stderr
        ),
    );
}

fn closing_bridge(s: &Scratch, pre: &PreMigration, c: &mut Cases) {
    let head = git_ok(&root(), &["rev-parse", "HEAD"]).trim().to_owned();
    c.check(
        "closing HEAD stable during migration",
        head == pre.closing_head,
        format!("before={} after={head}", pre.closing_head),
    );
    let closing = repo_at(s, "closing-tip", &head);
    let expected = projection(&pre.repo, &pre.census, true);
    let actual = projection(&closing, &pre.census, true);
    c.check(
        "closing HEAD bridge",
        expected == actual,
        projection_detail(&expected, &actual),
    );
    let path = "guidelines/cdc-2022-opioid/pl/cdc2022-opioid-rec01.pl";
    let file = closing.join(path);
    let original = fs::read(&file).unwrap();
    let mut plant = original.clone();
    plant.extend_from_slice(b"% ID2 closing-tip-only plant.\n");
    write(&file, plant);
    let changed = projection(&closing, &pre.census, true);
    c.check(
        "bridge tip-only plant changes bytes",
        actual.get(path) != changed.get(path),
        path,
    );
    c.check(
        "bridge rejects closing-tip-only edit",
        expected != changed,
        projection_detail(&expected, &changed),
    );
    c.check(
        "bridge plant leaves migrated copy untouched",
        projection(&pre.repo, &pre.census, true) == expected,
        "migrated copy byte/mode/path equality",
    );
    write(&file, original);
    c.check(
        "bridge closing-tip plant restored",
        projection(&closing, &pre.census, true) == actual,
        "closing tip byte/mode/path equality",
    );
}

#[test]
#[ignore = "one-time ID2 migration rerun through the current compiler: user ruling, run once at the ID2 commit, then manual (.agent/deferred.md, ID2 migration reruns)"]
fn p6_churn_oracle_accepts_migration_and_fires_on_each_self_plant() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("churn");
    let base = seed_repo(&s, "base");
    let pre = pre_migration(&s, "tip", &base);
    let tip = pre.repo.clone();
    let mut c = Cases::default();
    let migration = migrate(&tip);
    c.check(
        "churn baseline migration",
        migration.rc == 0,
        format!("{migration:?}"),
    );
    if migration.rc != 0 {
        c.finish();
        return;
    }
    closing_bridge(&s, &pre, &mut c);
    let valid = oracle(&base, &tip);
    c.check(
        "valid token + custody-only migration",
        valid.rc == 0,
        format!("{valid:?}"),
    );
    // D7 synthetic domain: base = seed + carrier. Its data atom is not an identifier token.
    let carrier = "tests/id2-churn-data-atom.pl.in";
    let data = "guideline_entity(actual,v1,patient,countable).\n";
    write(base.join(carrier), data);
    write(tip.join(carrier), data);
    git_ok(&base, &["add", "--", carrier]);
    git_ok(&tip, &["add", "--", carrier]);
    let control = oracle(&base, &tip);
    c.check(
        "identical data-v1 carrier control",
        control.rc == 0,
        format!("{control:?}"),
    );
    replace_first(&tip.join(carrier), ",v1,", ",v2,");
    oracle_case(&mut c, "data atom v1", &base, &tip, carrier);
    write(tip.join(carrier), data);
    const DOC: &str = "guidelines/cdc-2022-opioid/pl/cdc2022-opioid-rec01.pl";
    const RULE: &str = "guidelines/cdc-2022-opioid/pl/cdc2022-opioid-rec01-imp01.pl";
    const ANSWERS: &str = "guidelines/cdc-2022-opioid/queries/answers/recommendation-exists.pl";
    const TRACE: &str = "guidelines/cdc-2022-opioid/queries/traces/recommendation-exists.pl";
    const NEGATIVE: &str = "tests/v1/m7t-k1-020/input.pl.in";
    const ERASED: &str = "tests/v1/m7t-k1-020/expect/target.stderr";
    for (label, path) in [
        ("clause body", RULE),
        ("answer result", ANSWERS),
        ("trace tree", TRACE),
        ("wrong digest", ANSWERS),
        ("data number 1", DOC),
        ("declaration", DOC),
        ("metadata", DOC),
        ("erased reject", ERASED),
        ("missing file", DOC),
        ("negative regenerated green", NEGATIVE),
        ("partial migration", DOC),
    ] {
        let p = tip.join(path);
        let original = fs::read(&p).unwrap();
        match label {
            "clause body" => {
                let source = read(&p);
                let (head, body) = source.split_once(" :- ").expect("rule body plant");
                assert!(body.contains("countable"), "body-only plant anchor");
                write(
                    &p,
                    format!("{head} :- {}", body.replacen("countable", "mass", 1)),
                );
            }
            "answer result" => result_body(&p, "no(finite_failure)"),
            "trace tree" => result_body(&p, "yes(proved([]))"),
            "wrong digest" => mutate_digest(&p),
            "data number 1" => replace_first(&p, ",na,eq,1)", ",na,eq,2)"),
            "declaration" => replace_first(
                &p,
                ":- multifile(guideline_entity/4).",
                ":- multifile(guideline_entity/5).",
            ),
            "metadata" => replace_first(&p, "% S1:", "% changed S1:"),
            "erased reject" => {
                write(&p, "");
            }
            "missing file" => {
                fs::remove_file(&p).unwrap();
            }
            "negative regenerated green" => {
                let body = read(&p);
                let at = body.find("guideline_schema_version(").unwrap();
                let end = at + body[at..].find(".\n").unwrap() + 2;
                write(
                    &p,
                    format!(
                        "{}guideline_schema_version(2).\n{}",
                        &body[..at],
                        &body[end..]
                    ),
                );
                let materialized = s.file("regenerated-negative.pl", fs::read(&p).unwrap());
                let green = native(&s.0, &["v1", "check", &text(&materialized)]);
                c.same("negative plant is actually green", green, Out::ok(""));
            }
            "partial migration" => {
                fs::copy(base.join(path), &p).unwrap();
            }
            _ => unreachable!(),
        }
        c.check(
            &format!("{label}/plant changes bytes"),
            fs::read(&p).map_or(true, |b| b != original),
            path,
        );
        oracle_case(&mut c, label, &base, &tip, path);
        write(&p, original);
    }
    let unlisted = "guidelines/cdc-2022-opioid/pl/id2-unlisted.pl";
    write(tip.join(unlisted), read(tip.join(DOC)));
    git_ok(&tip, &["add", "--", unlisted]);
    oracle_case(&mut c, "unlisted file", &base, &tip, unlisted);
    fs::remove_file(tip.join(unlisted)).unwrap();
    git_ok(
        &tip,
        &["rm", "--quiet", "--cached", "--force", "--", unlisted],
    );
    let restored = oracle(&base, &tip);
    c.check(
        "self-plants restored byte-exactly",
        restored.rc == 0,
        format!("{restored:?}"),
    );
    c.finish();
}

fn archive(repo: &Path, reference: &str, dest: &Path, paths: &[&str]) {
    let mut args = vec![
        "archive",
        "--format=tar",
        "-o",
        dest.to_str().unwrap(),
        reference,
    ];
    args.extend(paths);
    git_ok(repo, &args);
}
fn review_hash(manifest: &str, id: &str) -> String {
    let row = manifest
        .lines()
        .find(|l| l.starts_with(&format!("{id}\t")))
        .unwrap();
    let f: Vec<_> = row.split('\t').collect();
    assert_eq!(f.len(), 6, "review bundle v2 row");
    f[5].into()
}
fn page(repo: &Path, id: &str) -> Out {
    native(
        repo,
        &[
            "ui",
            "request",
            "GET",
            &format!("/g/cdc-2022-opioid/doc/{id}.html"),
            &text(repo),
        ],
    )
}
#[test]
#[ignore = "one-time ID2 migration rerun through the current compiler: user ruling, run once at the ID2 commit, then manual (.agent/deferred.md, ID2 migration reruns)"]
fn p7_nonempty_ledger_stays_immutable_current_label_stales_and_old_row_attests() {
    let _serial = SERIAL.lock().unwrap_or_else(|p| p.into_inner());
    let s = Scratch::new("custody");
    let seed = seed_repo(&s, "seed");
    let repo = pre_migration(&s, "repo", &seed).repo;
    let mut c = Cases::default();
    let reviewed = git_ok(&repo, &["rev-parse", "HEAD"]).trim().to_owned();
    let g = "guidelines/cdc-2022-opioid";
    let id = "cdc2022-opioid-rec01";
    let row_digest = review_hash(&read(repo.join(g).join("audit/review-manifest.tsv")), id);
    let ledger_path = repo.join(g).join("audit/adjudication.tsv");
    let ledger = format!(
        "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n{id}\t{row_digest}\t{reviewed}\tapproved\tFixture reviewer\t2026-01-01T00:00:00Z\tID2 custody control\n"
    );
    write(&ledger_path, &ledger);
    commit(&repo, "fixture nonempty current review ledger");
    git_ok(&repo, &["tag", "fixture-release"]);
    let release_ref = git_ok(&repo, &["rev-parse", "fixture-release"]);
    let before_archive = s.0.join("release-before.tar");
    archive(&repo, "fixture-release", &before_archive, &[]);
    let release_hash = sha(fs::read(&before_archive).unwrap());
    let before = page(&repo, id);
    c.check(
        "nonempty-ledger current label",
        before.rc == 0
            && before.stderr.is_empty()
            && before.stdout.contains("class=\"chip chip-approved\""),
        format!(
            "rc={} approved={}",
            before.rc,
            before.stdout.contains("class=\"chip chip-approved\"")
        ),
    );
    let migration = migrate(&repo);
    c.check(
        "custody migration",
        migration.rc == 0,
        format!("{migration:?}"),
    );
    if migration.rc != 0 {
        c.finish();
        return;
    }
    c.check(
        "ledger rows byte-identical",
        fs::read(&ledger_path).unwrap() == ledger.as_bytes(),
        "cmp(nonempty ledger before, after)",
    );
    git_ok(&repo, &["add", "-A"]);
    git_ok(
        &repo,
        &[
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "fixture migrated current products",
        ],
    );
    let after = page(&repo, id);
    c.check(
        "current label stale",
        after.rc == 0
            && after.stderr.is_empty()
            && after.stdout.contains("class=\"chip chip-stale\""),
        format!(
            "rc={} stale={}",
            after.rc,
            after.stdout.contains("class=\"chip chip-stale\"")
        ),
    );
    let untouched = page(&repo, "cdc2022-opioid-rec01-imp01");
    c.check(
        "unreviewed stays unreviewed",
        untouched.rc == 0
            && untouched.stderr.is_empty()
            && untouched.stdout.contains("class=\"chip chip-unreviewed\""),
        format!(
            "rc={} unreviewed={}",
            untouched.rc,
            untouched.stdout.contains("class=\"chip chip-unreviewed\"")
        ),
    );
    let old_tar = s.0.join("reviewed.tar");
    archive(&repo, &reviewed, &old_tar, &[g]);
    let history = s.0.join("historical");
    fs::create_dir_all(&history).unwrap();
    c.same(
        "extract old reviewed bytes",
        run(
            Path::new("tar"),
            &s.0,
            &["-xf".into(), text(&old_tar), "-C".into(), text(&history)],
            &[],
        ),
        Out::ok(""),
    );
    let historical = native(&repo, &["derive-review-manifest", &text(&history.join(g))]);
    c.check(
        "old row raw attestation succeeds",
        historical.rc == 0 && historical.stderr.is_empty(),
        format!("rc={} stderr={:?}", historical.rc, historical.stderr),
    );
    if historical.rc == 0 {
        c.check(
            "old bundle still equals ledger row",
            review_hash(&historical.stdout, id) == row_digest,
            format!("recorded review_sha256={row_digest}"),
        );
    }
    c.check(
        "fixture release ref unchanged",
        git_ok(&repo, &["rev-parse", "fixture-release"]) == release_ref,
        release_ref.trim(),
    );
    let after_archive = s.0.join("release-after.tar");
    archive(&repo, "fixture-release", &after_archive, &[]);
    c.check(
        "fixture release archive unchanged",
        sha(fs::read(&after_archive).unwrap()) == release_hash
            && fs::read(&before_archive).unwrap() == fs::read(&after_archive).unwrap(),
        format!("sha256={release_hash}"),
    );
    c.finish();
}
