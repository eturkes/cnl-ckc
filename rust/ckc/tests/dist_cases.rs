// M5.6 P3: native replay of tests/dist/red.sh; fixture bytes + envelopes stay pinned.
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

// FC2 changes argv only; R44 retains goal: envelopes, cases.tsv stays verbatim.
const COMMAND_MAP: [(&str, &[&str]); 3] = [
    ("tools/dist.py", &["dist"]),
    ("tools/goal.py release-manifest", &["release-manifest"]),
    ("tools/goal.py check", &["check"]),
];
const GIT_ENV: [(&str, &str); 10] = [
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_CONFIG_SYSTEM", "/dev/null"),
    ("GIT_DEFAULT_HASH", "sha1"),
    ("GIT_AUTHOR_NAME", "fixture"),
    ("GIT_AUTHOR_EMAIL", "fixture@localhost"),
    ("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+00:00"),
    ("GIT_COMMITTER_NAME", "fixture"),
    ("GIT_COMMITTER_EMAIL", "fixture@localhost"),
    ("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+00:00"),
    ("LC_ALL", "C.UTF-8"),
];
const RIGHTS: &str = "profile\tstatement\turl\tretrieved\tnote\n";
const REVIEW_1: &str = "# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256\n";
const REVIEW_2: &str = "# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: python3 -P tools/goal.py review-manifest <id>; do not edit.\n";
const LEDGER: &str = "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n";
const SCHEMA: &str = "## Compiled Prolog schema (v1)\n\nFixture schema bytes stay verbatim.\n\n### Load\n\nRun `swipl -q -s data/guidelines/g-red/pl/doc-a.pl`.\n\n";
const NOTICE: &str = "Fixture KB notice.\nFirst-party fixture text.\n";
const DATE: &str = "2026-01-01T00:00:00Z";
static NEXT_TMP: AtomicUsize = AtomicUsize::new(0);
type TestResult<T> = Result<T, Box<dyn Error>>;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

struct TempDir(PathBuf);
impl TempDir {
    fn new() -> std::io::Result<Self> {
        loop {
            let path = std::env::temp_dir().join(format!(
                "ckc-dist-{}-{}",
                std::process::id(),
                NEXT_TMP.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
    }
}
impl Drop for TempDir {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.0) {
            eprintln!("dist-red: cleanup {}: {error}", self.0.display());
        }
    }
}

#[derive(Clone, Debug)]
struct Run {
    rc: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    issues: Vec<String>,
}
impl Run {
    fn check(&mut self, condition: bool, detail: impl Into<String>) {
        if !condition {
            self.issues.push(detail.into());
        }
    }
    fn carry(&mut self, previous: &Self) {
        self.issues.extend(previous.issues.iter().cloned());
    }
}
#[derive(Debug)]
struct SetupFailure(Run);
impl std::fmt::Display for SetupFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setup failure rc={}", self.0.rc)
    }
}
impl Error for SetupFailure {}

fn require_ok(mut run: Run, phase: &str) -> TestResult<Run> {
    if run.rc != 0 {
        run.issues.push(format!("setup {phase}"));
        return Err(Box::new(SetupFailure(run)));
    }
    Ok(run)
}

fn command(
    program: &OsStr,
    args: &[OsString],
    cwd: &Path,
    extra: &[(&str, &str)],
    timeout: u64,
) -> TestResult<Run> {
    let capture = TempDir::new()?;
    let stdout_path = capture.0.join("stdout");
    let stderr_path = capture.0.join("stderr");
    let mut cmd = Command::new(program);
    cmd.args(args)
        .current_dir(cwd)
        .envs(GIT_ENV)
        .env("TZ", "UTC")
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .envs(extra.iter().copied())
        .stdin(Stdio::null())
        .stdout(File::create(&stdout_path)?)
        .stderr(File::create(&stderr_path)?);
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(error) => {
            return Ok(Run {
                rc: 127,
                stdout: Vec::new(),
                stderr: format!("harness: exec {error}\n").into_bytes(),
                issues: Vec::new(),
            });
        }
    };
    let started = Instant::now();
    let rc = loop {
        if let Some(status) = child.try_wait()? {
            break status.code().unwrap_or(128);
        }
        if started.elapsed() >= Duration::from_secs(timeout) {
            child.kill()?;
            child.wait()?;
            break 124;
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let mut stderr = fs::read(stderr_path)?;
    if rc == 124 {
        stderr.extend_from_slice(b"harness: timeout\n");
    }
    Ok(Run {
        rc,
        stdout: fs::read(stdout_path)?,
        stderr,
        issues: Vec::new(),
    })
}

fn digest(data: &[u8]) -> TestResult<String> {
    let mut child = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or("sha256sum stdin")?
        .write_all(data)?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(format!("sha256sum: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    let text = String::from_utf8(output.stdout)?;
    let sha = text.split_whitespace().next().ok_or("empty sha256sum")?;
    if sha.len() != 64
        || !sha
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err("sha256sum grammar".into());
    }
    Ok(sha.to_owned())
}

struct Repo {
    path: PathBuf,
}
impl Repo {
    fn new(path: PathBuf) -> TestResult<Self> {
        fs::create_dir_all(&path)?;
        let repo = Self { path };
        require_ok(repo.git(&["init", "-q", "-b", "main"])?, "git init")?;
        Ok(repo)
    }
    fn git(&self, args: &[&str]) -> TestResult<Run> {
        command(
            OsStr::new("git"),
            &args.iter().map(OsString::from).collect::<Vec<_>>(),
            &self.path,
            &[],
            180,
        )
    }
    fn write(&self, rel: impl AsRef<Path>, data: impl AsRef<[u8]>) -> TestResult<PathBuf> {
        let path = self.path.join(rel);
        fs::create_dir_all(path.parent().ok_or("missing parent")?)?;
        fs::write(&path, data)?;
        Ok(path)
    }
    fn remove(&self, rel: &str) -> TestResult<()> {
        let path = self.path.join(rel);
        if path.is_dir() && !path.is_symlink() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        Ok(())
    }
    fn commit(&self, message: &str) -> TestResult<String> {
        require_ok(self.git(&["add", "-A"])?, "git add")?;
        self.commit_index(message)
    }
    fn commit_index(&self, message: &str) -> TestResult<String> {
        require_ok(self.git(&["commit", "-q", "-m", message])?, "git commit")?;
        self.head()
    }
    fn head(&self) -> TestResult<String> {
        let run = require_ok(self.git(&["rev-parse", "HEAD"])?, "git head")?;
        Ok(String::from_utf8(run.stdout)?.trim().to_owned())
    }
    fn input_head(&self) -> TestResult<String> {
        let run = require_ok(
            self.git(&[
                "log",
                "-1",
                "--format=%H",
                "HEAD",
                "--",
                "guidelines",
                "vendor/ape/prolog/ace_to_pl.pl",
                "vendor/clex/clex_lexicon.pl",
            ])?,
            "input head",
        )?;
        Ok(String::from_utf8(run.stdout)?.trim().to_owned())
    }
    fn show(&self, rel: &str) -> TestResult<Vec<u8>> {
        Ok(require_ok(self.git(&["show", &format!("HEAD:{rel}")])?, "git show")?.stdout)
    }
    fn clean(&self) -> TestResult<bool> {
        let run = self.git(&["status", "--porcelain"])?;
        Ok(run.rc == 0 && run.stdout.is_empty())
    }
}

fn native(
    repo: &Repo,
    mapped: usize,
    args: &[OsString],
    env: &[(&str, &str)],
    timeout: u64,
) -> TestResult<Run> {
    let mut argv: Vec<_> = COMMAND_MAP[mapped].1.iter().map(OsString::from).collect();
    argv.extend_from_slice(args);
    let binary = std::env::var_os("CKC_DIST_BIN")
        .unwrap_or_else(|| OsString::from(env!("CARGO_BIN_EXE_ckc")));
    command(&binary, &argv, &repo.path, env, timeout)
}
fn writer(repo: &Repo) -> TestResult<Run> {
    native(repo, 1, &[], &[], 180)
}
fn prepare_manifest(repo: &Repo) -> TestResult<Vec<u8>> {
    require_ok(writer(repo)?, "release-manifest")?;
    let data = fs::read(repo.path.join("release-manifest.tsv"))?;
    repo.commit("fixture release manifest")?;
    Ok(data)
}
fn process_tmp(repo: &Repo) -> TestResult<PathBuf> {
    let name = repo
        .path
        .file_name()
        .ok_or("repo basename")?
        .to_string_lossy();
    let path = repo
        .path
        .parent()
        .ok_or("repo parent")?
        .join(format!("{name}-process-tmp"));
    fs::create_dir_all(&path)?;
    Ok(path)
}
fn build(repo: &Repo, dest: &str, extra: &[(&str, &str)]) -> TestResult<(Run, PathBuf, PathBuf)> {
    let dest = repo.path.join(dest);
    let tmp = process_tmp(repo)?;
    let tmp_text = tmp.to_str().ok_or("tmp utf8")?;
    let mut env = vec![("TMPDIR", tmp_text)];
    env.extend_from_slice(extra);
    let run = native(
        repo,
        0,
        &["build".into(), dest.clone().into_os_string()],
        &env,
        180,
    )?;
    Ok((run, dest, tmp))
}
fn empty(path: &Path) -> TestResult<bool> {
    Ok(!path.exists() || fs::read_dir(path)?.next().is_none())
}
fn names(path: &Path) -> TestResult<Vec<OsString>> {
    let mut names = fs::read_dir(path)?
        .map(|item| item.map(|item| item.file_name()))
        .collect::<Result<Vec<_>, _>>()?;
    names.sort();
    Ok(names)
}
fn has(bytes: &[u8], needle: &[u8]) -> bool {
    bytes.windows(needle.len()).any(|part| part == needle)
}

fn review_record(gid: &str, docid: &str) -> TestResult<Vec<String>> {
    let mut values = Vec::new();
    let mut block = format!("bundle v2 {docid}\n");
    for name in ["ace", "coverage", "payload", "clauses"] {
        let value = digest(format!("{name}:{gid}:{docid}").as_bytes())?;
        block.push_str(&format!("{name} {value}\n"));
        values.push(value);
    }
    values.push(digest(block.as_bytes())?);
    Ok(values)
}
fn rights_text(rows: &[[&str; 5]]) -> String {
    let mut text = RIGHTS.to_owned();
    for row in rows {
        text.push_str(&row.join("\t"));
        text.push('\n');
    }
    text
}
fn write_guideline(
    repo: &Repo,
    gid: &str,
    profile: &str,
    docs: &[(&str, &str)],
    rights: Option<&[[&str; 5]]>,
    rights_only: bool,
) -> TestResult<()> {
    let statement = format!("Fixture rights statement for {gid}.");
    let url = format!("https://example.invalid/{gid}");
    let default = [[
        profile,
        statement.as_str(),
        url.as_str(),
        "2026-01-01",
        "Fixture note.",
    ]];
    repo.write(
        format!("guidelines/{gid}/rights.tsv"),
        rights_text(rights.unwrap_or(&default)),
    )?;
    if rights_only {
        return Ok(());
    }
    repo.write(
        format!("guidelines/{gid}/README.md"),
        format!("# {gid}\n\n## Rights and attribution\n\nFixture custody text.\n"),
    )?;
    repo.write(
        format!("guidelines/{gid}/source/original.txt"),
        format!("source bytes for {gid}\n"),
    )?;
    let mut manifest = format!("{REVIEW_1}{REVIEW_2}");
    let mut decisions = Vec::new();
    let mut sorted = docs.to_vec();
    sorted.sort();
    for (docid, state) in sorted {
        let values = review_record(gid, docid)?;
        manifest.push_str(&format!("{docid}\t{}\n", values.join("\t")));
        repo.write(
            format!("guidelines/{gid}/ace/{docid}.ace"),
            "Every fixture is a record.\n",
        )?;
        repo.write(
            format!("guidelines/{gid}/pl/{docid}.pl"),
            format!("guideline_document('{gid}','{docid}',[],x).\n"),
        )?;
        let review = values[4].clone();
        match state {
            "approved" | "rejected" => decisions.push((docid, DATE, review, state)),
            "stale" => decisions.push((
                docid,
                DATE,
                digest(format!("{gid}{docid}old").as_bytes())?,
                "approved",
            )),
            "contested" => {
                decisions.push((docid, DATE, review.clone(), "approved"));
                decisions.push((docid, "2026-01-01T00:00:01Z", review, "rejected"));
            }
            "unreviewed" => (),
            _ => return Err(format!("bad fixture state {state}").into()),
        }
    }
    repo.write(
        format!("guidelines/{gid}/audit/review-manifest.tsv"),
        manifest,
    )?;
    if !decisions.is_empty() {
        decisions.sort();
        let mut ledger = LEDGER.to_owned();
        for (docid, when, review, verdict) in decisions {
            ledger.push_str(&format!(
                "{docid}\t{review}\t\t{verdict}\tfixture\t{when}\tfixture decision\n"
            ));
        }
        repo.write(format!("guidelines/{gid}/audit/adjudication.tsv"), ledger)?;
    }
    for (extension, bytes) in [
        ("ace", "Is every fixture a record?\n"),
        ("answers.pl", "answer_manifest(v1,query_a,yes,[]).\n"),
        ("trace.pl", "trace_manifest(v1,query_a,yes,[]).\n"),
    ] {
        repo.write(
            format!("guidelines/{gid}/queries/query-a.{extension}"),
            bytes,
        )?;
    }
    Ok(())
}
fn base_repo(
    path: PathBuf,
    profile: &str,
    docs: &[(&str, &str)],
    rights: Option<&[[&str; 5]]>,
    rights_only: bool,
) -> TestResult<Repo> {
    let repo = Repo::new(path)?;
    repo.write("docs/REFERENCE.md", format!("# Fixture KB\n\nFixture repository.\n\n{SCHEMA}## Operating\n\nRun the checks.\n\n## Licensing\n\nFixture licensing.\n"))?;
    repo.write("NOTICE", NOTICE)?;
    repo.write("vendor/ape/prolog/ace_to_pl.pl", "% fixture compiler\n")?;
    repo.write("vendor/clex/clex_lexicon.pl", "% fixture base lexicon\n")?;
    for directory in ["tools", "tests", ".agent", ".github"] {
        repo.write(format!("{directory}/excluded.txt"), "must not ship\n")?;
    }
    write_guideline(&repo, "g-red", profile, docs, rights, rights_only)?;
    repo.commit("fixture corpus")?;
    Ok(repo)
}
fn basic(path: &Path) -> TestResult<Repo> {
    base_repo(
        path.join("repo"),
        "redistributable",
        &[("doc-a", "unreviewed")],
        None,
        false,
    )
}
fn profile_repo(path: &Path) -> TestResult<Repo> {
    let repo = base_repo(
        path.join("repo"),
        "redistributable",
        &[
            ("doc-approved", "approved"),
            ("doc-stale", "stale"),
            ("doc-unreviewed", "unreviewed"),
        ],
        None,
        false,
    )?;
    write_guideline(
        &repo,
        "g-fetch",
        "reconstructable",
        &[("fetch-doc", "unreviewed")],
        None,
        false,
    )?;
    write_guideline(
        &repo,
        "g-hold",
        "restricted",
        &[("held-a", "approved"), ("held-b", "unreviewed")],
        None,
        false,
    )?;
    repo.commit("profile fixtures")?;
    Ok(repo)
}

struct Header {
    block: [u8; 512],
}
impl Header {
    fn kind(&self) -> u8 {
        self.block[156]
    }
    fn number(&self, start: usize, end: usize) -> TestResult<u64> {
        let text = std::str::from_utf8(&self.block[start..end])?.trim_matches(['\0', ' ']);
        Ok(if text.is_empty() {
            0
        } else {
            u64::from_str_radix(text, 8)?
        })
    }
    fn string(&self, start: usize, end: usize) -> TestResult<String> {
        Ok(String::from_utf8(
            self.block[start..end]
                .split(|byte| *byte == 0)
                .next()
                .ok_or("tar field")?
                .to_vec(),
        )?)
    }
    fn checksum_ok(&self) -> TestResult<bool> {
        let checksum = self
            .block
            .iter()
            .enumerate()
            .map(|(index, byte)| {
                if (148..156).contains(&index) {
                    32
                } else {
                    u64::from(*byte)
                }
            })
            .sum::<u64>();
        Ok(self.number(148, 156)? == checksum)
    }
}
struct Member {
    name: String,
    header: usize,
}
struct Bag {
    archive: PathBuf,
    sidecar: PathBuf,
    raw: Vec<u8>,
    tar: Vec<u8>,
    headers: Vec<Header>,
    members: Vec<Member>,
    root: String,
    rel: BTreeMap<String, Vec<u8>>,
}
impl Bag {
    fn open(dest: &Path) -> TestResult<Self> {
        let archives: Vec<_> = names(dest)?
            .into_iter()
            .filter(|name| name.to_string_lossy().ends_with(".tar.gz"))
            .collect();
        if archives.len() != 1 {
            return Err(format!("archive count={}", archives.len()).into());
        }
        let archive = dest.join(&archives[0]);
        let sidecar = archive.with_file_name(format!(
            "{}.sha256",
            archives[0].to_str().ok_or("archive utf8")?
        ));
        if !sidecar.is_file() {
            return Err("sidecar missing".into());
        }
        let raw = fs::read(&archive)?;
        let decoded = command(
            OsStr::new("gzip"),
            &["-dc".into(), archive.clone().into_os_string()],
            dest,
            &[],
            180,
        )?;
        if decoded.rc != 0 || !decoded.stderr.is_empty() {
            return Err(format!(
                "gzip decode rc={} stderr={}",
                decoded.rc,
                String::from_utf8_lossy(&decoded.stderr)
            )
            .into());
        }
        let tar = decoded.stdout;
        let mut headers = Vec::new();
        let mut members = Vec::new();
        let mut files = BTreeMap::new();
        let mut long_name = None;
        let mut offset = 0;
        while offset + 512 <= tar.len() {
            let block: [u8; 512] = tar[offset..offset + 512].try_into()?;
            if block == [0; 512] {
                break;
            }
            let header = Header { block };
            if !header.checksum_ok()? {
                return Err("tar header checksum".into());
            }
            let size = usize::try_from(header.number(124, 136)?)?;
            let begin = offset.checked_add(512).ok_or("tar offset overflow")?;
            let end = begin.checked_add(size).ok_or("tar size overflow")?;
            let content = tar.get(begin..end).ok_or("truncated tar member")?;
            let short = header.string(0, 100)?;
            let prefix = header.string(345, 500)?;
            let name = if prefix.is_empty() {
                short
            } else {
                format!("{prefix}/{short}")
            };
            if header.kind() == b'L' {
                long_name = Some(String::from_utf8(
                    content
                        .strip_suffix(&[0])
                        .ok_or("GNU longname terminator")?
                        .to_vec(),
                )?);
            } else {
                let name = long_name.take().unwrap_or(name);
                if Path::new(&name)
                    .components()
                    .any(|part| !matches!(part, Component::Normal(_)))
                {
                    return Err(format!("nonrelative tar path {name:?}").into());
                }
                if matches!(header.kind(), b'0' | 0)
                    && files.insert(name.clone(), content.to_vec()).is_some()
                {
                    return Err(format!("duplicate tar member {name}").into());
                }
                members.push(Member {
                    name,
                    header: headers.len(),
                });
            }
            headers.push(header);
            offset = begin
                .checked_add(
                    size.div_ceil(512)
                        .checked_mul(512)
                        .ok_or("tar padding overflow")?,
                )
                .ok_or("tar offset overflow")?;
        }
        if long_name.is_some()
            || offset + 1024 > tar.len()
            || tar[offset..].iter().any(|byte| *byte != 0)
            || !tar.len().is_multiple_of(10240)
        {
            return Err("tar end blocks/record padding".into());
        }
        let roots: BTreeSet<_> = files
            .keys()
            .map(|name| name.split('/').next().unwrap_or("").to_owned())
            .collect();
        if roots.len() != 1 {
            return Err(format!("bag root count={}", roots.len()).into());
        }
        let root = roots.into_iter().next().ok_or("missing bag root")?;
        let prefix = format!("{root}/");
        let mut rel = BTreeMap::new();
        for (name, bytes) in files {
            rel.insert(
                name.strip_prefix(&prefix)
                    .ok_or("member at bag root")?
                    .to_owned(),
                bytes,
            );
        }
        Ok(Self {
            archive,
            sidecar,
            raw,
            tar,
            headers,
            members,
            root,
            rel,
        })
    }
    fn bytes(&self, path: &str) -> &[u8] {
        self.rel.get(path).map_or(&[], Vec::as_slice)
    }
    fn rows(&self) -> TestResult<Vec<Vec<String>>> {
        let text = std::str::from_utf8(self.bytes("release-manifest.tsv"))?;
        if !text.ends_with('\n') || text.contains('\r') {
            return Err("release-manifest newline law".into());
        }
        let rows: Vec<Vec<String>> = text[..text.len() - 1]
            .split('\n')
            .map(|line| line.split('\t').map(str::to_owned).collect())
            .collect();
        for row in &rows {
            let width = match row.first().map(String::as_str) {
                Some("meta" | "label") => 3,
                Some("member") => 4,
                Some("source") => 5,
                _ => return Err("release-manifest row kind".into()),
            };
            if row.len() != width {
                return Err("release-manifest row width".into());
            }
        }
        Ok(rows)
    }
    fn extract(&self, dest: &Path) -> TestResult<PathBuf> {
        let root = dest.join(&self.root);
        for (rel, bytes) in &self.rel {
            let path = root.join(rel);
            fs::create_dir_all(path.parent().ok_or("extract parent")?)?;
            fs::write(path, bytes)?;
        }
        Ok(root)
    }
    fn finals(&self) -> TestResult<Vec<OsString>> {
        let mut expected = vec![
            self.archive
                .file_name()
                .ok_or("archive basename")?
                .to_owned(),
            self.sidecar
                .file_name()
                .ok_or("sidecar basename")?
                .to_owned(),
        ];
        expected.sort();
        Ok(expected)
    }
}
fn bag_after(run: &mut Run, dest: &Path) -> Option<Bag> {
    if run.rc != 0 {
        return None;
    }
    match Bag::open(dest) {
        Ok(bag) => Some(bag),
        Err(error) => {
            run.issues.push(format!("bag parse: {error}"));
            None
        }
    }
}
fn meta(rows: &[Vec<String>], key: &str) -> TestResult<String> {
    let hits: Vec<_> = rows
        .iter()
        .filter(|row| row.len() == 3 && row[0] == "meta" && row[1] == key)
        .collect();
    if hits.len() != 1 {
        return Err(format!("meta {key} count={}", hits.len()).into());
    }
    Ok(hits[0][2].clone())
}
fn digest_lines(data: &[u8]) -> TestResult<Vec<(String, String)>> {
    if !data.ends_with(b"\n") || data.contains(&b'\r') || data.starts_with(b"\xef\xbb\xbf") {
        return Err("digest file encoding/newline".into());
    }
    let mut rows = Vec::new();
    for line in data[..data.len() - 1].split(|byte| *byte == b'\n') {
        if line.len() < 67
            || &line[64..66] != b"  "
            || !line[..64]
                .iter()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
        {
            return Err(format!("digest line grammar {line:?}").into());
        }
        rows.push((
            String::from_utf8(line[66..].to_vec())?,
            String::from_utf8(line[..64].to_vec())?,
        ));
    }
    if rows.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return Err("digest path order/uniqueness".into());
    }
    Ok(rows)
}
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & 0_u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}

fn input_head_case(path: &Path) -> TestResult<Run> {
    let repo = basic(path)?;
    let head_one = repo.input_head()?;
    require_ok(writer(&repo)?, "release-manifest initial")?;
    let manifest = fs::read(repo.path.join("release-manifest.tsv"))?;
    repo.commit("fixture release manifest")?;
    let mut carried = require_ok(writer(&repo)?, "release-manifest after commit")?;
    carried.check(
        fs::read(repo.path.join("release-manifest.tsv"))? == manifest,
        "manifest changed after manifest-only commit",
    );
    carried.check(repo.clean()?, "writer dirt after manifest-only commit");
    repo.write("docs/unrelated.txt", "unrelated\n")?;
    repo.commit("unrelated docs")?;
    carried.check(
        repo.input_head()? == head_one,
        "unrelated commit changed input head",
    );
    require_ok(writer(&repo)?, "release-manifest after unrelated commit")?;
    carried.check(
        fs::read(repo.path.join("release-manifest.tsv"))? == manifest,
        "unrelated commit changed manifest",
    );
    let (first, dest, _) = build(&repo, "out-one", &[])?;
    let mut first = require_ok(first, "first input-head build")?;
    first.carry(&carried);
    let bag_one = bag_after(&mut first, &dest);
    if let Some(bag) = &bag_one {
        first.check(
            meta(&bag.rows()?, "head")? == head_one,
            "meta head does not bind input head",
        );
        first.check(
            bag.archive.file_name()
                == Some(OsStr::new(&format!(
                    "cnl-ckc-kb-g{}.tar.gz",
                    &head_one[..12]
                ))),
            "archive name does not bind input head",
        );
        first.check(
            bag.root == format!("cnl-ckc-kb-g{}", &head_one[..12]),
            "bag root does not bind input head",
        );
    }
    repo.write(
        "vendor/clex/clex_lexicon.pl",
        "% changed fixture base lexicon\n",
    )?;
    let head_two = repo.commit("compiler input change")?;
    first.check(
        repo.input_head()? == head_two,
        "compiler input commit not selected",
    );
    require_ok(writer(&repo)?, "release-manifest after input change")?;
    repo.commit("updated release manifest")?;
    let (mut second, dest, _) = build(&repo, "out-two", &[])?;
    second.carry(&first);
    if let Some(bag) = bag_after(&mut second, &dest) {
        second.check(
            bag.archive.file_name()
                == Some(OsStr::new(&format!(
                    "cnl-ckc-kb-g{}.tar.gz",
                    &head_two[..12]
                ))),
            "changed input head not in archive name",
        );
        second.check(
            bag_one.is_some_and(|one| one.archive.file_name() != bag.archive.file_name()),
            "input change retained archive name",
        );
    }
    Ok(second)
}
fn writer_case(path: &Path) -> TestResult<Run> {
    let repo = basic(path)?;
    require_ok(writer(&repo)?, "release-manifest first")?;
    let before = fs::read(repo.path.join("release-manifest.tsv"))?;
    repo.commit("fixture release manifest")?;
    let mut second = writer(&repo)?;
    if second.rc == 0 {
        second.check(
            fs::read(repo.path.join("release-manifest.tsv"))? == before,
            "writer changed bytes after commit",
        );
        second.check(repo.clean()?, "writer left worktree dirty");
    }
    Ok(second)
}
fn rights_case(path: &Path, scenario: &str) -> TestResult<Run> {
    let repo = basic(path)?;
    prepare_manifest(&repo)?;
    let fields: [&str; 5] = [
        "redistributable",
        "statement",
        "https://example.invalid",
        "2026-01-01",
        "note",
    ];
    let data = match scenario {
        "rights_missing" | "stage_clean_refusal" => None,
        "rights_header" => Some(b"bad\theader\nredistributable\tstatement\thttps://example.invalid\t2026-01-01\tnote\n".to_vec()),
        "rights_rows" => Some(RIGHTS.as_bytes().to_vec()),
        "rights_fields" => Some(format!("{RIGHTS}redistributable\tstatement\thttps://example.invalid\t2026-01-01\tnote\textra\n").into_bytes()),
        "rights_profile" | "rights_statement" | "rights_url" | "rights_retrieved" => {
            let mut row = fields;
            let (index, value) = match scenario {
                "rights_profile" => (0, "other"),
                "rights_statement" => (1, ""),
                "rights_url" => (2, ""),
                _ => (3, "2026-02-30"),
            };
            row[index] = value;
            Some(rights_text(&[row]).into_bytes())
        }
        "rights_control" => Some([RIGHTS.as_bytes(), b"redistributable\tstatement\thttps://example.invalid\t2026-01-01\tnote\x7f\n"].concat()),
        "rights_utf8" => Some([RIGHTS.as_bytes(), b"redistributable\tbad\xff\thttps://example.invalid\t2026-01-01\tnote\n"].concat()),
        "rights_second_row_invalid" => Some(rights_text(&[
            ["redistributable", "operative", "https://example.invalid/one", "2026-01-01", "one"],
            ["invalid", "second", "https://example.invalid/two", "2026-01-02", "two"],
        ]).into_bytes()),
        "reconstructable_empty_url" => Some(rights_text(&[["reconstructable", "Fixture rights.", "", "2026-01-01", "note"]]).into_bytes()),
        _ => return Err(format!("unknown rights scenario {scenario}").into()),
    };
    if let Some(data) = data {
        repo.write("guidelines/g-red/rights.tsv", data)?;
    } else {
        repo.remove("guidelines/g-red/rights.tsv")?;
    }
    repo.commit(match scenario {
        "stage_clean_refusal" => "rights refusal",
        "reconstructable_empty_url" => "empty reconstructable url",
        _ => "fixture mutation",
    })?;
    let (mut run, dest, tmp) = build(&repo, "out", &[])?;
    if scenario == "stage_clean_refusal" {
        run.check(empty(&dest)?, "refusal left destination finals/stage");
        run.check(empty(&tmp)?, "refusal leaked process temp");
    }
    Ok(run)
}
fn fixture(path: &Path, scenario: &str) -> TestResult<Repo> {
    match scenario {
        "restricted_labels"
        | "bagit_closure"
        | "profile_member_set"
        | "consumer_copy"
        | "sha256_command"
        | "release_member_digests" => profile_repo(path),
        "reconstructable_source" => base_repo(
            path.join("repo"),
            "reconstructable",
            &[("doc-a", "unreviewed")],
            None,
            false,
        ),
        "rights_first_row_operative" => base_repo(
            path.join("repo"),
            "redistributable",
            &[("doc-a", "unreviewed")],
            Some(&[
                [
                    "redistributable",
                    "operative row statement",
                    "https://example.invalid/one",
                    "2026-01-01",
                    "one",
                ],
                [
                    "restricted",
                    "secondary row statement",
                    "https://example.invalid/two",
                    "2026-01-02",
                    "two",
                ],
            ]),
            false,
        ),
        "rights_only" => base_repo(
            path.join("repo"),
            "redistributable",
            &[("doc-a", "unreviewed")],
            None,
            true,
        ),
        "rejected_order" => base_repo(
            path.join("repo"),
            "redistributable",
            &[
                ("z-rejected", "rejected"),
                ("a-rejected", "rejected"),
                ("m-approved", "approved"),
            ],
            None,
            false,
        ),
        "contested" => base_repo(
            path.join("repo"),
            "redistributable",
            &[("doc-contested", "contested")],
            None,
            false,
        ),
        "label_classes" => base_repo(
            path.join("repo"),
            "redistributable",
            &[
                ("doc-approved", "approved"),
                ("doc-stale", "stale"),
                ("doc-unreviewed", "unreviewed"),
            ],
            None,
            false,
        ),
        _ => basic(path),
    }
}
fn before_manifest(repo: &Repo, scenario: &str) -> TestResult<()> {
    match scenario {
        "exec_mode" => {
            fs::set_permissions(
                repo.path.join("guidelines/g-red/ace/doc-a.ace"),
                fs::Permissions::from_mode(0o755),
            )?;
            repo.commit("executable corpus blob")?;
        }
        "longname" => {
            repo.write(
                format!("guidelines/g-red/source/{}.txt", "x".repeat(150)),
                "long path bytes\n",
            )?;
            repo.commit("long member path")?;
        }
        "space_path" | "sha256_command" => {
            repo.write("guidelines/g-red/source/space name.txt", "space path\n")?;
            repo.commit(if scenario == "space_path" {
                "space path"
            } else {
                "verification space path"
            })?;
        }
        _ => (),
    }
    Ok(())
}
fn after_manifest(repo: &Repo, scenario: &str) -> TestResult<()> {
    let message = match scenario {
        "stray_root" => {
            repo.write("guidelines/stray.txt", "stray\n")?;
            "stray guideline root member"
        }
        "no_guidelines" => {
            repo.remove("guidelines")?;
            "remove guidelines"
        }
        "manifest_source_drift" => {
            repo.write(
                "guidelines/g-red/source/original.txt",
                "changed committed source\n",
            )?;
            "source drift"
        }
        "manifest_tamper" => {
            let text = fs::read_to_string(repo.path.join("release-manifest.tsv"))?;
            let mut changed = 0;
            let mut lines = Vec::new();
            for line in text.lines() {
                if line.starts_with("member\tdata/guidelines/g-red/source/original.txt\t") {
                    let mut row: Vec<_> = line.split('\t').map(str::to_owned).collect();
                    row[2] = "0".repeat(64);
                    lines.push(row.join("\t"));
                    changed += 1;
                } else {
                    lines.push(line.to_owned());
                }
            }
            if changed != 1 {
                return Err(format!("source member rows={changed}").into());
            }
            repo.write("release-manifest.tsv", format!("{}\n", lines.join("\n")))?;
            "tamper release manifest"
        }
        "symlink_member" | "member_precedence" => {
            symlink("doc-a.ace", repo.path.join("guidelines/g-red/ace/link.ace"))?;
            if scenario == "member_precedence" {
                repo.remove("guidelines/g-red/rights.tsv")?;
                "symlink plus rights defect"
            } else {
                "symlink member"
            }
        }
        "gitlink_member" => {
            require_ok(
                repo.git(&[
                    "update-index",
                    "--add",
                    "--cacheinfo",
                    &format!("160000,{},guidelines/g-red/submodule", repo.head()?),
                ])?,
                "gitlink setup",
            )?;
            repo.commit_index("gitlink member")?;
            return Ok(());
        }
        "newline_path" => {
            repo.write("guidelines/g-red/ace/bad\nname.ace", "bad path\n")?;
            "newline path"
        }
        "invalid_utf8_path" => {
            let name = OsString::from_vec(b"bad-\xff.ace".to_vec());
            repo.write(Path::new("guidelines/g-red/ace").join(name), "bad path\n")?;
            "invalid utf8 path"
        }
        "backslash_path" => {
            repo.write("guidelines/g-red/ace/bad\\name.ace", "bad path\n")?;
            "backslash path"
        }
        "committed_state" => {
            for (name, bytes) in [
                (
                    "guidelines/g-red/source/original.txt",
                    b"dirty source\n".as_slice(),
                ),
                ("guidelines/g-red/rights.tsv", b"bad\xff"),
                ("docs/REFERENCE.md", b"dirty reference\n"),
                ("NOTICE", b"dirty notice\n"),
                ("vendor/ape/prolog/ace_to_pl.pl", b"% dirty compiler\n"),
                ("release-manifest.tsv", b"dirty manifest\n"),
            ] {
                repo.write(name, bytes)?;
            }
            return Ok(());
        }
        _ => return Ok(()),
    };
    repo.commit(message)?;
    Ok(())
}

fn row(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| (*part).to_owned()).collect()
}
fn validate_bag(
    repo: &Repo,
    path: &Path,
    scenario: &str,
    run: &mut Run,
    bag: &Bag,
) -> TestResult<()> {
    let rows = bag.rows()?;
    let source = "data/guidelines/g-red/source/original.txt";
    match scenario {
        "reconstructable_source" => {
            let bytes = repo.show("guidelines/g-red/source/original.txt")?;
            run.check(
                !bag.rel.contains_key(source),
                "reconstructable source shipped",
            );
            run.check(
                rows.contains(&row(&[
                    "source",
                    source,
                    &digest(&bytes)?,
                    &bytes.len().to_string(),
                    "https://example.invalid/g-red",
                ])),
                "reconstructable source row mismatch",
            );
        }
        "rights_first_row_operative" => run.check(
            bag.bytes(source) == repo.show("guidelines/g-red/source/original.txt")?,
            "row 2 changed operative profile",
        ),
        "rights_only" => run.check(
            bag.rel.contains_key("data/guidelines/g-red/rights.tsv"),
            "rights-only guideline absent",
        ),
        "committed_state" => {
            run.check(
                bag.bytes(source) == repo.show("guidelines/g-red/source/original.txt")?,
                "working source leaked",
            );
            run.check(
                has(bag.bytes("README-dist.md"), SCHEMA.as_bytes()),
                "committed schema section absent",
            );
            run.check(
                !has(bag.bytes("README-dist.md"), b"dirty reference"),
                "working docs/REFERENCE.md leaked",
            );
            run.check(
                has(bag.bytes("NOTICE"), &repo.show("NOTICE")?),
                "committed NOTICE absent",
            );
            run.check(
                !has(bag.bytes("NOTICE"), b"dirty notice"),
                "working NOTICE leaked",
            );
            run.check(
                bag.bytes("release-manifest.tsv") == repo.show("release-manifest.tsv")?,
                "working release manifest leaked",
            );
            run.check(
                meta(&rows, "compiler")? == digest(&repo.show("vendor/ape/prolog/ace_to_pl.pl")?)?,
                "working compiler leaked",
            );
            run.check(
                meta(&rows, "head")? == repo.input_head()?,
                "working input changed head",
            );
            run.check(
                repo.show("docs/REFERENCE.md")?.starts_with(b"# Fixture"),
                "fixture precondition",
            );
        }
        "manifest_order" => {
            let ranks: Vec<_> = rows
                .iter()
                .map(|row| match row[0].as_str() {
                    "meta" => 0,
                    "member" => 1,
                    "source" => 2,
                    _ => 3,
                })
                .collect();
            run.check(
                ranks.windows(2).all(|pair| pair[0] <= pair[1]),
                "manifest row group order",
            );
            let fixed: Vec<_> = rows
                .iter()
                .filter(|row| row[0] == "meta" && row[1] != "replay" && row[1] != "generated")
                .map(|row| row[1].as_str())
                .collect();
            run.check(
                fixed
                    == [
                        "schema",
                        "head",
                        "compiler",
                        "base-lexicon",
                        "python",
                        "swipl",
                        "verify",
                    ],
                "fixed meta order",
            );
            let replay: Vec<_> = rows
                .iter()
                .filter(|row| row[0] == "meta" && row[1] == "replay")
                .map(|row| row[2].as_str())
                .collect();
            run.check(replay.len() == 3, "replay row count");
            if replay.len() == 3 {
                run.check(
                    replay[0].contains("compile")
                        && replay[1].contains("check")
                        && replay[2].contains("load"),
                    "replay order",
                );
            }
            let generated: Vec<_> = rows
                .iter()
                .filter(|row| row[0] == "meta" && row[1] == "generated")
                .map(|row| row[2].as_str())
                .collect();
            run.check(
                generated
                    == [
                        "release-manifest.tsv",
                        "manifest-sha256.txt",
                        "tagmanifest-sha256.txt",
                    ],
                "generated row order",
            );
            for kind in ["member", "source", "label"] {
                let paths: Vec<_> = rows
                    .iter()
                    .filter(|row| row[0] == kind)
                    .map(|row| row[1].as_str())
                    .collect();
                run.check(
                    paths.windows(2).all(|pair| pair[0] <= pair[1]),
                    format!("{kind} sort order"),
                );
            }
        }
        "runtime_meta" => {
            for (key, expected) in [
                ("schema", "v1".to_owned()),
                ("head", repo.input_head()?),
                (
                    "compiler",
                    digest(&repo.show("vendor/ape/prolog/ace_to_pl.pl")?)?,
                ),
                (
                    "base-lexicon",
                    digest(&repo.show("vendor/clex/clex_lexicon.pl")?)?,
                ),
                ("python", "3.11".to_owned()),
                ("swipl", "9.2.9".to_owned()),
                (
                    "verify",
                    "sha256sum -c manifest-sha256.txt tagmanifest-sha256.txt".to_owned(),
                ),
            ] {
                run.check(
                    meta(&rows, key)? == expected,
                    format!("meta {key} mismatch"),
                );
            }
        }
        "restricted_labels" | "label_classes" => {
            let actual: Vec<_> = rows
                .iter()
                .filter(|row| row[0] == "label")
                .cloned()
                .collect();
            let mut expected = vec![
                row(&["label", "doc-approved", "approved"]),
                row(&["label", "doc-stale", "stale"]),
                row(&["label", "doc-unreviewed", "unreviewed"]),
            ];
            if scenario == "restricted_labels" {
                expected.push(row(&["label", "fetch-doc", "unreviewed"]));
                run.check(
                    !bag.rel
                        .keys()
                        .any(|path| path.starts_with("data/guidelines/g-hold/")),
                    "restricted payload shipped",
                );
                let readme = bag.bytes("README-dist.md");
                run.check(
                    has(readme, b"g-hold") && has(readme, b"2"),
                    "held-back id/count absent",
                );
                run.check(
                    !has(readme, b"held-a") && !has(readme, b"held-b"),
                    "held-back docid disclosed",
                );
                run.check(
                    has(readme, b"Fixture rights statement for g-hold."),
                    "restricted rights absent from README",
                );
                run.check(
                    has(bag.bytes("NOTICE"), b"Fixture rights statement for g-hold."),
                    "restricted rights absent from NOTICE",
                );
            }
            run.check(actual == expected, "adjudication label classes/order");
        }
        "exec_mode" => {
            let member_path = format!("{}/data/guidelines/g-red/ace/doc-a.ace", bag.root);
            let members: Vec<_> = bag
                .members
                .iter()
                .filter(|member| member.name == member_path)
                .collect();
            run.check(
                members.len() == 1 && bag.headers[members[0].header].number(100, 108)? == 0o644,
                "executable mode not normalized",
            );
            let hits: Vec<_> = rows
                .iter()
                .filter(|row| row[0] == "member" && row[1] == "data/guidelines/g-red/ace/doc-a.ace")
                .collect();
            run.check(
                hits.len() == 1
                    && hits[0][2] == digest(&repo.show("guidelines/g-red/ace/doc-a.ace")?)?,
                "exec member digest mismatch",
            );
        }
        "gzip_header" => {
            run.check(
                bag.raw.get(..10) == Some(&[0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 2, 0xff]),
                format!("gzip header={:02x?}", bag.raw.get(..10)),
            );
            if bag.raw.len() >= 18 {
                let trailer = &bag.raw[bag.raw.len() - 8..];
                run.check(
                    u32::from_le_bytes(trailer[..4].try_into()?) == crc32(&bag.tar),
                    "gzip trailer CRC",
                );
                run.check(
                    u32::from_le_bytes(trailer[4..].try_into()?)
                        == (bag.tar.len() as u64 & 0xffff_ffff) as u32,
                    "gzip trailer ISIZE",
                );
            } else {
                run.check(false, "gzip trailer missing");
            }
        }
        "sidecar" => {
            let basename = bag
                .archive
                .file_name()
                .ok_or("archive basename")?
                .to_str()
                .ok_or("archive utf8")?;
            run.check(
                fs::read(&bag.sidecar)?
                    == format!("{}  {basename}\n", digest(&bag.raw)?).as_bytes(),
                "sidecar grammar/digest",
            );
            run.check(
                names(bag.archive.parent().ok_or("dest")?)? == bag.finals()?,
                "destination final set",
            );
        }
        "success_meter" => {
            let text = std::str::from_utf8(&run.stdout)?;
            let lines: Vec<_> = text.split_inclusive('\n').collect();
            let line_law = lines.len() == 2 && lines.iter().all(|line| line.ends_with('\n'));
            let meter_law = lines.first().is_some_and(|line| {
                let fields: Vec<_> = line.split_whitespace().collect();
                fields.len() == 8
                    && fields[0] == "dist:"
                    && fields[1] == "ok"
                    && fields[3] == "guidelines"
                    && fields[5] == "members"
                    && fields[7] == "bytes"
                    && [2, 4, 6].into_iter().all(|index| {
                        !fields[index].is_empty()
                            && fields[index].bytes().all(|byte| byte.is_ascii_digit())
                    })
            });
            let expected_sha = format!(
                "dist: sha256={} cnl-ckc-kb-g{}.tar.gz\n",
                digest(&bag.raw)?,
                &repo.input_head()?[..12]
            );
            let sha_law = lines.get(1).is_some_and(|line| *line == expected_sha);
            run.check(line_law, "success stdout line count/LF");
            run.check(meter_law, "success meter line");
            run.check(sha_law, "sha meter line");
        }
        "tar_fields" => {
            let head = repo.input_head()?;
            let epoch_run = require_ok(
                repo.git(&["show", "-s", "--format=%ct", &head])?,
                "source epoch",
            )?;
            let epoch = std::str::from_utf8(&epoch_run.stdout)?
                .trim()
                .parse::<u64>()?;
            run.check(
                bag.members
                    .windows(2)
                    .all(|pair| pair[0].name <= pair[1].name),
                "tar member order",
            );
            for member in &bag.members {
                let header = &bag.headers[member.header];
                run.check(
                    matches!(header.kind(), b'0' | 0),
                    "tar contains directory/non-file member",
                );
                run.check(
                    header.number(108, 116)? == 0 && header.number(116, 124)? == 0,
                    format!("tar uid/gid {}", member.name),
                );
                run.check(
                    header.string(265, 297)?.is_empty() && header.string(297, 329)?.is_empty(),
                    format!("tar owner names {}", member.name),
                );
                run.check(
                    header.number(100, 108)? == 0o644,
                    format!("tar mode {}", member.name),
                );
                run.check(
                    header.number(136, 148)? == epoch,
                    format!("tar mtime {}", member.name),
                );
            }
            run.check(
                bag.headers.len() == bag.members.len(),
                "raw header/member count",
            );
            for header in &bag.headers {
                run.check(
                    matches!(header.kind(), b'0' | 0),
                    "raw non-file type/PAX header",
                );
                run.check(
                    &header.block[257..263] == b"ustar\0" && &header.block[263..265] == b"00",
                    "raw USTAR pin",
                );
                run.check(header.checksum_ok()?, "raw tar checksum");
                run.check(
                    header.block[265..329].iter().all(|byte| *byte == 0),
                    "raw owner name",
                );
            }
        }
        "longname" => {
            run.check(
                bag.rel.contains_key(&format!(
                    "data/guidelines/g-red/source/{}.txt",
                    "x".repeat(150)
                )),
                "long member absent",
            );
            run.check(
                bag.headers.iter().any(|header| header.kind() == b'L'),
                "GNU longname fallback absent",
            );
            run.check(
                !bag.headers
                    .iter()
                    .any(|header| matches!(header.kind(), b'x' | b'g')),
                "PAX fallback used",
            );
        }
        "bagit_closure" => {
            let actual: Vec<_> = rows
                .iter()
                .filter(|row| row[0] == "member")
                .map(|row| row[1].clone())
                .collect();
            let mut expected: Vec<_> = bag
                .rel
                .keys()
                .filter(|path| path.starts_with("data/guidelines/"))
                .cloned()
                .collect();
            expected.extend([
                "bagit.txt".to_owned(),
                "README-dist.md".to_owned(),
                "NOTICE".to_owned(),
            ]);
            expected.sort();
            run.check(actual == expected, "release member closure");
            let generated: Vec<_> = rows
                .iter()
                .filter(|row| row[0] == "meta" && row[1] == "generated")
                .cloned()
                .collect();
            run.check(
                generated
                    == [
                        row(&["meta", "generated", "release-manifest.tsv"]),
                        row(&["meta", "generated", "manifest-sha256.txt"]),
                        row(&["meta", "generated", "tagmanifest-sha256.txt"]),
                    ],
                "generated digestless closure",
            );
            let payload: Vec<_> = digest_lines(bag.bytes("manifest-sha256.txt"))?
                .into_iter()
                .map(|(path, _)| path)
                .collect();
            let expected: Vec<_> = bag
                .rel
                .keys()
                .filter(|path| path.starts_with("data/"))
                .cloned()
                .collect();
            run.check(payload == expected, "payload manifest closure");
            let tags: Vec<_> = digest_lines(bag.bytes("tagmanifest-sha256.txt"))?
                .into_iter()
                .map(|(path, _)| path)
                .collect();
            let expected: Vec<_> = bag
                .rel
                .keys()
                .filter(|path| !path.starts_with("data/") && *path != "tagmanifest-sha256.txt")
                .cloned()
                .collect();
            run.check(tags == expected, "tagmanifest closure");
        }
        "profile_member_set" => {
            run.check(
                bag.bytes(source) == repo.show("guidelines/g-red/source/original.txt")?,
                "redistributable source absent",
            );
            run.check(
                !bag.rel
                    .contains_key("data/guidelines/g-fetch/source/original.txt"),
                "reconstructable source present",
            );
            run.check(
                !bag.rel
                    .keys()
                    .any(|path| path.starts_with("data/guidelines/g-hold/")),
                "restricted guideline present",
            );
            for rel in [
                "rights.tsv",
                "ace/fetch-doc.ace",
                "pl/fetch-doc.pl",
                "audit/review-manifest.tsv",
            ] {
                run.check(
                    bag.rel
                        .contains_key(&format!("data/guidelines/g-fetch/{rel}")),
                    format!("reconstructable non-source absent: {rel}"),
                );
            }
            let bytes = repo.show("guidelines/g-fetch/source/original.txt")?;
            run.check(
                rows.contains(&row(&[
                    "source",
                    "data/guidelines/g-fetch/source/original.txt",
                    &digest(&bytes)?,
                    &bytes.len().to_string(),
                    "https://example.invalid/g-fetch",
                ])),
                "reconstructable source authority row",
            );
        }
        "bag_layout" => {
            let required = [
                "bagit.txt",
                "manifest-sha256.txt",
                "tagmanifest-sha256.txt",
                "README-dist.md",
                "NOTICE",
                "release-manifest.tsv",
            ];
            run.check(
                required.iter().all(|path| bag.rel.contains_key(*path)),
                "required BagIt tags",
            );
            run.check(
                bag.bytes("bagit.txt")
                    == b"BagIt-Version: 1.0\nTag-File-Character-Encoding: UTF-8\n",
                "bagit.txt bytes",
            );
            run.check(
                !bag.rel.contains_key("bag-info.txt"),
                "bag-info.txt present",
            );
            run.check(
                bag.rel.keys().all(|path| {
                    required.contains(&path.as_str()) || path.starts_with("data/guidelines/")
                }),
                "bag path outside layout",
            );
            run.check(
                bag.root == format!("cnl-ckc-kb-g{}", &repo.input_head()?[..12]),
                "single bag root name",
            );
            run.check(
                bag.members
                    .iter()
                    .all(|member| matches!(bag.headers[member.header].kind(), b'0' | 0)),
                "serialized directory member",
            );
        }
        "exclusion" => {
            let forbidden = [
                "tools/", "vendor/", "tests/", ".agent/", ".github/", ".git/",
            ];
            run.check(
                !bag.rel
                    .keys()
                    .any(|path| forbidden.iter().any(|prefix| path.starts_with(prefix))),
                "non-guideline repository tree shipped",
            );
            run.check(
                !bag.rel.values().any(|bytes| has(bytes, b"must not ship")),
                "excluded marker bytes shipped",
            );
        }
        "consumer_copy" => {
            let readme = bag.bytes("README-dist.md");
            let notice = bag.bytes("NOTICE");
            run.check(
                has(readme, SCHEMA.as_bytes()),
                "schema section not embedded verbatim",
            );
            run.check(has(notice, NOTICE.as_bytes()), "root NOTICE not embedded");
            for gid in ["g-red", "g-fetch", "g-hold"] {
                let statement = format!("Fixture rights statement for {gid}.");
                run.check(
                    has(readme, statement.as_bytes()),
                    format!("README rights missing {gid}"),
                );
                run.check(
                    has(notice, statement.as_bytes()),
                    format!("NOTICE rights missing {gid}"),
                );
            }
            run.check(
                has(
                    readme,
                    b"sha256sum -c manifest-sha256.txt tagmanifest-sha256.txt",
                ),
                "verification command absent",
            );
            run.check(
                has(readme, b"compile") && has(readme, b"check") && has(readme, b"load"),
                "replay commands absent",
            );
            let lower = notice.to_ascii_lowercase();
            run.check(
                has(&lower, b"compiler") && has(&lower, b"derived"),
                "compiler-derived output statement absent",
            );
        }
        "space_path" => {
            let name = "data/guidelines/g-red/source/space name.txt";
            run.check(
                bag.bytes(name) == b"space path\n",
                "space path payload absent",
            );
            run.check(
                digest_lines(bag.bytes("manifest-sha256.txt"))?
                    .iter()
                    .any(|(path, _)| path == name),
                "space path checksum absent",
            );
        }
        "sha256_command" | "checksum_tamper" => {
            let extracted = bag.extract(&path.join("extract"))?;
            if scenario == "checksum_tamper" {
                let payload = bag
                    .rel
                    .keys()
                    .find(|path| path.starts_with("data/"))
                    .ok_or("tamper probe has no payload")?;
                let mut bytes = bag.bytes(payload).to_vec();
                bytes.extend_from_slice(b"tamper");
                fs::write(extracted.join(payload), bytes)?;
            }
            let verified = command(
                OsStr::new("sha256sum"),
                &[
                    "-c".into(),
                    "manifest-sha256.txt".into(),
                    "tagmanifest-sha256.txt".into(),
                ],
                &extracted,
                &[],
                60,
            )?;
            if scenario == "checksum_tamper" {
                run.check(
                    verified.rc == 1,
                    format!("tampered sha256sum rc={}", verified.rc),
                );
                run.check(
                    has(&verified.stdout, b"FAILED") || has(&verified.stderr, b"FAILED"),
                    "tampered checksum not reported",
                );
            } else {
                run.check(
                    verified.rc == 0,
                    format!("sha256sum verify rc={}", verified.rc),
                );
                run.check(verified.stderr.is_empty(), "sha256sum verify stderr");
            }
        }
        "tag_self_exclusion" => {
            let tags: Vec<_> = digest_lines(bag.bytes("tagmanifest-sha256.txt"))?
                .into_iter()
                .map(|(path, _)| path)
                .collect();
            run.check(
                !tags.iter().any(|path| path == "tagmanifest-sha256.txt"),
                "tagmanifest hashes itself",
            );
            let expected: Vec<_> = bag
                .rel
                .keys()
                .filter(|path| !path.starts_with("data/") && *path != "tagmanifest-sha256.txt")
                .cloned()
                .collect();
            run.check(tags == expected, "tagmanifest root tag set");
        }
        "release_member_digests" => {
            for row in rows.iter().filter(|row| row[0] == "member") {
                let name = &row[1];
                run.check(
                    bag.rel.contains_key(name),
                    format!("member row path absent {name}"),
                );
                if let Some(bytes) = bag.rel.get(name) {
                    run.check(digest(bytes)? == row[2], format!("member digest {name}"));
                    run.check(
                        bytes.len().to_string() == row[3],
                        format!("member bytes {name}"),
                    );
                }
            }
            run.check(
                !rows.iter().any(|row| {
                    row[0] == "member"
                        && [
                            "release-manifest.tsv",
                            "manifest-sha256.txt",
                            "tagmanifest-sha256.txt",
                        ]
                        .contains(&row[1].as_str())
                }),
                "circular member digest row",
            );
        }
        "default_dest" | "stage_clean_success" => run.check(
            names(bag.archive.parent().ok_or("dest")?)? == bag.finals()?,
            "destination final set/stage residue",
        ),
        _ => (),
    }
    Ok(())
}

fn collision_case(repo: &Repo) -> TestResult<Run> {
    let (first, dest, tmp) = build(repo, "out", &[])?;
    let mut first = require_ok(first, "collision seed build")?;
    let Some(bag) = bag_after(&mut first, &dest) else {
        return Ok(first);
    };
    let mut tampered = bag.raw.clone();
    tampered.extend_from_slice(b"tamper");
    fs::write(&bag.archive, &tampered)?;
    let sidecar = fs::read(&bag.sidecar)?;
    let mut second = native(
        repo,
        0,
        &["build".into(), dest.into_os_string()],
        &[("TMPDIR", tmp.to_str().ok_or("tmp utf8")?)],
        180,
    )?;
    second.carry(&first);
    second.check(
        fs::read(&bag.archive)? == tampered,
        "collision archive replaced",
    );
    second.check(
        fs::read(&bag.sidecar)? == sidecar,
        "collision sidecar replaced",
    );
    second.check(empty(&tmp)?, "collision leaked process temp");
    Ok(second)
}
fn determinism_case(repo: &Repo) -> TestResult<Run> {
    let (first, one, _) = build(
        repo,
        "out-one",
        &[
            ("PYTHONHASHSEED", "1"),
            ("TZ", "Pacific/Honolulu"),
            ("LC_ALL", "C"),
        ],
    )?;
    let mut first = require_ok(first, "determinism build one")?;
    let (mut second, two, _) = build(
        repo,
        "out-two",
        &[
            ("PYTHONHASHSEED", "99991"),
            ("TZ", "UTC"),
            ("LC_ALL", "C.UTF-8"),
        ],
    )?;
    let bag_one = bag_after(&mut first, &one);
    let bag_two = bag_after(&mut second, &two);
    second.carry(&first);
    if let (Some(one), Some(two)) = (bag_one, bag_two) {
        second.check(
            one.raw == two.raw,
            "archive bytes differ across environment/dest",
        );
        second.check(
            fs::read(one.sidecar)? == fs::read(two.sidecar)?,
            "sidecar bytes differ",
        );
    }
    Ok(second)
}
fn destination_case(repo: &Repo, path: &Path, scenario: &str) -> TestResult<Run> {
    let dest = repo.path.join("out");
    let outside = path.join("outside");
    let sentinel = path.join("sentinel");
    let archive = dest.join(format!("cnl-ckc-kb-g{}.tar.gz", &repo.input_head()?[..12]));
    let sidecar = archive.with_file_name(format!(
        "{}.sha256",
        archive
            .file_name()
            .ok_or("archive basename")?
            .to_string_lossy()
    ));
    if scenario == "symlink_dest" {
        fs::create_dir(&outside)?;
        symlink(&outside, &dest)?;
    } else {
        fs::create_dir(&dest)?;
        if scenario == "archive_symlink" {
            fs::write(&sentinel, b"sentinel\n")?;
            symlink(&sentinel, &archive)?;
        } else {
            fs::create_dir(&sidecar)?;
        }
    }
    let (mut run, _, tmp) = build(repo, "out", &[])?;
    match scenario {
        "symlink_dest" => run.check(empty(&outside)?, "symlinked destination received finals"),
        "archive_symlink" => {
            run.check(
                fs::read(&sentinel)? == b"sentinel\n",
                "archive symlink target changed",
            );
            run.check(archive.is_symlink(), "archive symlink replaced");
        }
        _ => {
            run.check(sidecar.is_dir(), "sidecar collision directory replaced");
            run.check(
                !archive.exists(),
                "archive published before sidecar refusal",
            );
        }
    }
    run.check(empty(&tmp)?, "destination refusal leaked process temp");
    Ok(run)
}
fn check_case(path: &Path, blocked: bool) -> TestResult<Run> {
    let clone_path = path.join(if blocked {
        "blocked-clone"
    } else {
        "live-clone"
    });
    require_ok(
        command(
            OsStr::new("git"),
            &[
                "clone".into(),
                "-q".into(),
                "--shared".into(),
                repo_root().into_os_string(),
                clone_path.clone().into_os_string(),
            ],
            path,
            &[],
            180,
        )?,
        "check clone",
    )?;
    let repo = Repo { path: clone_path };
    let mut writer_ok = true;
    let mut writer_rc = 0;
    if blocked {
        let guidelines = repo.path.join("guidelines");
        let dirs: Vec<_> = names(&guidelines)?
            .into_iter()
            .filter(|name| guidelines.join(name).is_dir())
            .collect();
        for name in &dirs {
            let rights = guidelines.join(name).join("rights.tsv");
            if !rights.exists() {
                let gid = name.to_str().ok_or("guideline utf8")?;
                fs::write(
                    rights,
                    rights_text(&[[
                        "redistributable",
                        "Fixture blocked-check rights.",
                        &format!("https://example.invalid/{gid}"),
                        "2026-01-01",
                        "Fixture note.",
                    ]]),
                )?;
            }
        }
        let guideline = guidelines.join(dirs.first().ok_or("blocked clone has no guidelines")?);
        let manifest = fs::read_to_string(guideline.join("audit/review-manifest.tsv"))?;
        let mut rows: Vec<_> = manifest
            .lines()
            .skip(2)
            .filter(|line| !line.is_empty())
            .map(|line| line.split('\t').collect::<Vec<_>>())
            .collect();
        if rows.len() < 2 || rows.iter().any(|row| row.len() != 6) {
            return Err("blocked clone needs two valid review rows".into());
        }
        rows.sort_by(|a, b| a[0].cmp(b[0]));
        let mut decisions = [
            (rows[0][0], DATE, rows[0][5], "rejected"),
            (rows[1][0], DATE, rows[1][5], "approved"),
            (rows[1][0], "2026-01-01T00:00:01Z", rows[1][5], "rejected"),
        ];
        decisions.sort();
        let mut ledger = LEDGER.to_owned();
        for (docid, when, review, verdict) in decisions {
            ledger.push_str(&format!(
                "{docid}\t{review}\t\t{verdict}\tfixture\t{when}\tfixture blocked decision\n"
            ));
        }
        fs::write(guideline.join("audit/adjudication.tsv"), ledger)?;
        repo.commit("blocked distribution corpus")?;
        let written = writer(&repo)?;
        writer_rc = written.rc;
        writer_ok = writer_rc == 0 && repo.path.join("release-manifest.tsv").is_file();
        if writer_ok {
            repo.commit("blocked release manifest")?;
        }
    }
    let mut run = native(
        &repo,
        2,
        &[],
        &[("SWIPL", "__dist_red_missing_swipl__")],
        420,
    )?;
    let text = String::from_utf8_lossy(&run.stdout);
    let meters: Vec<_> = text
        .lines()
        .filter(|line| line.starts_with("goal: dist "))
        .map(str::to_owned)
        .collect();
    run.check(
        writer_ok,
        format!("blocked setup release-manifest rc={writer_rc}"),
    );
    if blocked {
        run.check(
            meters == ["goal: dist blocked rejected=1 contested=1"],
            format!("blocked meter={meters:?}"),
        );
    } else {
        run.check(
            meters.len() == 1,
            format!("goal check dist meter count={}", meters.len()),
        );
        if let Some(meter) = meters.first() {
            let blocked_grammar = meter
                .strip_prefix("goal: dist blocked rejected=")
                .and_then(|suffix| suffix.split_once(" contested="))
                .is_some_and(|(rejected, contested)| {
                    [rejected, contested].iter().all(|part| {
                        !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit())
                    })
                });
            run.check(
                meter.starts_with("goal: dist ok ") || blocked_grammar,
                "goal check dist meter grammar",
            );
        }
    }
    Ok(run)
}
fn scenario(path: &Path, name: &str) -> TestResult<Run> {
    match name {
        "input_head" => return input_head_case(path),
        "writer_idempotent" => return writer_case(path),
        "goal_check_live" => return check_case(path, false),
        "goal_check_blocked" => return check_case(path, true),
        "rights_missing"
        | "rights_header"
        | "rights_rows"
        | "rights_fields"
        | "rights_profile"
        | "rights_statement"
        | "rights_url"
        | "rights_retrieved"
        | "rights_control"
        | "rights_utf8"
        | "rights_second_row_invalid"
        | "reconstructable_empty_url"
        | "stage_clean_refusal" => return rights_case(path, name),
        "reconstructable_source"
        | "rights_first_row_operative"
        | "stray_root"
        | "no_guidelines"
        | "rights_only"
        | "committed_state"
        | "manifest_order"
        | "runtime_meta"
        | "manifest_source_drift"
        | "manifest_tamper"
        | "restricted_labels"
        | "rejected_order"
        | "contested"
        | "label_classes"
        | "exec_mode"
        | "output_collision"
        | "gzip_header"
        | "sidecar"
        | "symlink_member"
        | "gitlink_member"
        | "member_precedence"
        | "success_meter"
        | "byte_determinism"
        | "tar_fields"
        | "longname"
        | "bagit_closure"
        | "profile_member_set"
        | "bag_layout"
        | "exclusion"
        | "consumer_copy"
        | "newline_path"
        | "invalid_utf8_path"
        | "backslash_path"
        | "space_path"
        | "sha256_command"
        | "checksum_tamper"
        | "tag_self_exclusion"
        | "release_member_digests"
        | "usage_missing"
        | "usage_mode"
        | "default_dest"
        | "symlink_dest"
        | "archive_symlink"
        | "sidecar_directory"
        | "stage_clean_success" => (),
        _ => return Err(format!("unknown dist scenario {name}").into()),
    }
    let repo = fixture(path, name)?;
    if name == "usage_missing" || name == "usage_mode" {
        let args = if name == "usage_mode" {
            vec!["inspect".into()]
        } else {
            Vec::new()
        };
        return native(&repo, 0, &args, &[], 180);
    }
    before_manifest(&repo, name)?;
    prepare_manifest(&repo)?;
    after_manifest(&repo, name)?;
    match name {
        "output_collision" => return collision_case(&repo),
        "byte_determinism" => return determinism_case(&repo),
        "symlink_dest" | "archive_symlink" | "sidecar_directory" => {
            return destination_case(&repo, path, name);
        }
        _ => (),
    }
    let (mut run, dest, tmp) = if name == "default_dest" {
        let tmp = process_tmp(&repo)?;
        (
            native(
                &repo,
                0,
                &["build".into()],
                &[("TMPDIR", tmp.to_str().ok_or("tmp utf8")?)],
                180,
            )?,
            repo.path.join("dist"),
            tmp,
        )
    } else {
        build(&repo, "out", &[])?
    };
    match name {
        "rejected_order" | "contested" => {
            run.check(empty(&dest)?, "rejected/contested build left finals");
            if name == "rejected_order" {
                run.check(empty(&tmp)?, "rejected build leaked process temp");
            }
        }
        "newline_path" => run.check(
            has(&run.stderr, b"\\n"),
            "newline path rendering not escaped",
        ),
        "invalid_utf8_path" => run.check(
            has(&run.stderr, b"\\xff"),
            "non-UTF8 rendering not ascii escaped",
        ),
        "backslash_path" => run.check(
            has(&run.stderr, b"\\\\"),
            "backslash path rendering not escaped",
        ),
        "stage_clean_success" => run.check(empty(&tmp)?, "success leaked process temp"),
        _ => (),
    }
    if let Some(bag) = bag_after(&mut run, &dest) {
        validate_bag(&repo, path, name, &mut run, &bag)?;
    }
    Ok(run)
}

struct Case {
    id: String,
    family: String,
    scenario: String,
    rc: i32,
    stderr: String,
}
fn read_cases() -> TestResult<Vec<Case>> {
    let text = fs::read_to_string(repo_root().join("tests/dist/cases.tsv"))?;
    let mut lines = text.lines();
    if lines.next() != Some("id\tfamily\tscenario\texpected_rc\texpected_stderr_prefix") {
        return Err("invalid cases.tsv header".into());
    }
    let mut cases = Vec::new();
    let mut ids = BTreeSet::new();
    let mut scenarios = BTreeSet::new();
    let mut families = BTreeMap::new();
    let mut codes = BTreeMap::new();
    for line in lines {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 5
            || !ids.insert(fields[0].to_owned())
            || !scenarios.insert(fields[2].to_owned())
        {
            return Err(format!("invalid/duplicate case {line:?}").into());
        }
        let rc = fields[3].parse()?;
        cases.push(Case {
            id: fields[0].to_owned(),
            family: fields[1].to_owned(),
            scenario: fields[2].to_owned(),
            rc,
            stderr: fields[4].to_owned(),
        });
        *families.entry(fields[1]).or_insert(0) += 1;
        *codes.entry(rc).or_insert(0) += 1;
    }
    let expected = BTreeMap::from([
        ("manifest", 8),
        ("rights", 16),
        ("labels", 4),
        ("determinism", 11),
        ("members", 9),
        ("verification", 4),
        ("runner", 10),
    ]);
    if cases.len() != 62
        || families != expected
        || codes != BTreeMap::from([(0, 29), (1, 29), (2, 4)])
    {
        return Err(format!(
            "case census: total={} families={families:?} rc={codes:?}",
            cases.len()
        )
        .into());
    }
    Ok(cases)
}
fn evaluate(case: &Case, run: &Run) -> bool {
    let stderr_ok = if case.stderr == "-" {
        run.stderr.is_empty()
    } else {
        run.stderr.starts_with(case.stderr.as_bytes())
    };
    let refusal_ok = case.rc != 1
        || (run.stdout.is_empty()
            && run.stderr.ends_with(b"\n")
            && run.stderr.iter().filter(|byte| **byte == b'\n').count() == 1);
    run.rc == case.rc && stderr_ok && refusal_ok && run.issues.is_empty()
}
#[test]
fn dist_cases() {
    let cases = read_cases().expect("dist TSV contract");
    let mut passed = 0;
    for case in &cases {
        let sandbox = TempDir::new().expect("case temporary directory");
        let run = match scenario(&sandbox.0, &case.scenario) {
            Ok(run) => run,
            Err(error) => error.downcast_ref::<SetupFailure>().map_or_else(
                || Run {
                    rc: 125,
                    stdout: Vec::new(),
                    stderr: format!("harness: {error}\n").into_bytes(),
                    issues: Vec::new(),
                },
                |setup| setup.0.clone(),
            ),
        };
        let ok = evaluate(case, &run);
        passed += usize::from(ok);
        let stderr = String::from_utf8_lossy(&run.stderr);
        let actual = stderr.lines().next().unwrap_or("-");
        let issue = run.issues.first().map_or("-", String::as_str);
        println!(
            "{}\t{}\t{}\texpected rc={} stderr^={:?}\tactual rc={} stderr={:?}\tissue={issue}",
            if ok { "PASS" } else { "RED" },
            case.id,
            case.family,
            case.rc,
            case.stderr,
            run.rc,
            actual
        );
    }
    println!(
        "dist-red: pass={passed} red={} total={}",
        cases.len() - passed,
        cases.len()
    );
    assert_eq!(
        passed,
        cases.len(),
        "dist scenarios failed; per-case diagnostics above"
    );
}
