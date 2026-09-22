use super::common::*;
use super::process;
use ckc_kernel::EMember;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

type Staged = BTreeMap<String, Vec<u8>>;
type Rights = Vec<Vec<String>>;

pub(super) struct ReleasePlan {
    pub head: String,
    pub epoch: i64,
    pub payload: Staged,
    pub tags: Staged,
    pub manifest: Vec<u8>,
    pub rejected: Vec<String>,
    pub contested: Vec<String>,
    pub shipped: usize,
}

pub(super) struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new() -> Result<Self> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        loop {
            let path = std::env::temp_dir().join(format!(
                "ckc-dist.{}.{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(error(format!("scratch: {e}"))),
            }
        }
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).ok();
    }
}

fn error(detail: impl AsRef<str>) -> Failure {
    violation("dist", detail)
}
fn dgit(root: &Path, args: &[&str], detail: &str) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|_| error(detail))?;
    if !out.status.success() {
        return Err(error(detail));
    }
    Ok(out.stdout)
}
fn head_file(root: &Path, path: &str) -> Result<Vec<u8>> {
    dgit(
        root,
        &["show", &format!("HEAD:{path}")],
        &format!("no-input {path}"),
    )
}
fn rendered_path(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| match *b {
            b'\\' => "\\\\".to_owned(),
            b'\n' => "\\n".to_owned(),
            b'\r' => "\\r".to_owned(),
            b'\t' => "\\t".to_owned(),
            32..=126 => char::from(*b).to_string(),
            _ => format!("\\x{b:02x}"),
        })
        .collect()
}
fn clean(s: &str) -> bool {
    s.chars().all(|c| c >= ' ' && c != '\u{7f}')
}
pub(super) fn path_text(bytes: &[u8]) -> Result<String> {
    let bad = || error(format!("member-path {}", rendered_path(bytes)));
    let s = std::str::from_utf8(bytes).map_err(|_| bad())?;
    if !clean(s)
        || s.contains('\\')
        || s.starts_with('/')
        || s.split('/').any(|s| matches!(s, "" | "." | ".."))
    {
        return Err(bad());
    }
    Ok(s.to_owned())
}
fn date(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 10
        || b[4] != b'-'
        || b[7] != b'-'
        || !b
            .iter()
            .enumerate()
            .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
    {
        return false;
    }
    let y: u32 = s[..4].parse().unwrap_or(0);
    let m: u32 = s[5..7].parse().unwrap_or(0);
    let d: u32 = s[8..].parse().unwrap_or(0);
    let days = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if y.is_multiple_of(4) && (!y.is_multiple_of(100) || y.is_multiple_of(400)) {
                29
            } else {
                28
            }
        }
        _ => 0,
    };
    y > 0 && d > 0 && d <= days
}
fn rights(gid: &str, staged: &Staged) -> Result<Rights> {
    let bad = |s: &str| error(format!("rights {gid} {s}"));
    let bytes = staged
        .get(&format!("guidelines/{gid}/rights.tsv"))
        .ok_or_else(|| bad("missing"))?;
    let text = std::str::from_utf8(bytes).map_err(|_| bad("utf8"))?;
    let (head, body) = text.split_once('\n').unwrap_or((text, ""));
    if head != "profile\tstatement\turl\tretrieved\tnote" {
        return Err(bad("header"));
    }
    if !text.contains('\n') || (!body.is_empty() && !body.ends_with('\n')) {
        return Err(bad("rows"));
    }
    let mut rows = Vec::new();
    for (i, line) in body
        .strip_suffix('\n')
        .into_iter()
        .flat_map(|s| s.split('\n'))
        .enumerate()
    {
        let fields: Vec<String> = line.split('\t').map(str::to_owned).collect();
        let fail_row = |s: &str| bad(&format!("{s}:{}", i + 1));
        if fields.len() != 5 {
            return Err(fail_row("fields"));
        }
        if !matches!(
            fields[0].as_str(),
            "redistributable" | "reconstructable" | "restricted"
        ) {
            return Err(fail_row("profile"));
        }
        if fields[1].is_empty() {
            return Err(fail_row("statement"));
        }
        if fields[2].is_empty() {
            return Err(fail_row("url"));
        }
        if !date(&fields[3]) {
            return Err(fail_row("retrieved"));
        }
        if !fields.iter().all(|s| clean(s)) {
            return Err(fail_row("control"));
        }
        rows.push(fields);
    }
    if rows.is_empty() {
        return Err(bad("rows"));
    }
    Ok(rows)
}
fn review_docs(gid: &str, staged: &Staged) -> BTreeMap<String, String> {
    let mut docs = BTreeMap::new();
    if let Some(text) = staged
        .get(&format!("guidelines/{gid}/audit/review-manifest.tsv"))
        .and_then(|b| std::str::from_utf8(b).ok())
    {
        for line in text
            .split('\n')
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
        {
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() >= 2 {
                docs.insert(fields[0].to_owned(), fields[fields.len() - 1].to_owned());
            }
        }
    }
    docs
}
fn classes(
    gid: &str,
    staged: &Staged,
    docs: &BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    let mut seen = BTreeSet::new();
    let mut approved = BTreeSet::new();
    let mut rejected = BTreeSet::new();
    if let Some(text) = staged
        .get(&format!("guidelines/{gid}/audit/adjudication.tsv"))
        .and_then(|b| std::str::from_utf8(b).ok())
    {
        for line in text
            .split('\n')
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
        {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() == 7 && docs.contains_key(f[0]) {
                seen.insert(f[0].to_owned());
                if docs.get(f[0]).is_some_and(|d| d == f[1]) {
                    if f[3] == "approved" {
                        approved.insert(f[0].to_owned());
                    }
                    if f[3] == "rejected" {
                        rejected.insert(f[0].to_owned());
                    }
                }
            }
        }
    }
    docs.keys()
        .map(|id| {
            let class = if !seen.contains(id) {
                "unreviewed"
            } else {
                match (approved.contains(id), rejected.contains(id)) {
                    (true, true) => "contested",
                    (true, false) => "approved",
                    (false, true) => "rejected",
                    _ => "stale",
                }
            };
            (id.clone(), class.to_owned())
        })
        .collect()
}
fn schema(text: &str) -> Result<String> {
    let anchor = "## Compiled Prolog schema";
    let after = text
        .split_once(&format!("\n{anchor}"))
        .map(|(_, s)| s)
        .or_else(|| text.strip_prefix(anchor))
        .ok_or_else(|| error("no-input docs/REFERENCE.md schema section"))?;
    Ok(match after.split_once("\n## ") {
        Some((s, _)) => format!("{anchor}{s}\n"),
        None => format!("{anchor}{after}"),
    })
}
fn rights_line(gid: &str, row: &[String], open: &str, close: &str) -> String {
    format!(
        "- {gid} ({}): \"{}\" {open}{}, retrieved {}{close}{}\n",
        row[0],
        row[1],
        row[2],
        row[3],
        if row[4].is_empty() {
            String::new()
        } else {
            format!(" {}", row[4])
        }
    )
}
fn readme(
    gids: &BTreeSet<String>,
    rs: &BTreeMap<String, Rights>,
    counts: &BTreeMap<String, usize>,
    labels: &BTreeMap<String, String>,
    schema: &str,
) -> String {
    let mut text = "# cnl-ckc knowledge base export\n\nThis archive is a BagIt 1.0 bag. It holds the compiled clinical-guideline knowledge base from the cnl-ckc repository. The archive name and the `meta head` row in `release-manifest.tsv` give the source commit.\n\n## Verification\n\nRun `sha256sum -c manifest-sha256.txt tagmanifest-sha256.txt` from this directory. Each line must report OK.\n\n## Contents\n\n".to_owned();
    for gid in gids {
        let n = counts.get(gid).copied().unwrap_or(0);
        text += &match rs[gid][0][0].as_str() {
            "restricted" => format!("- {gid}: {n} documents held back (restricted rights).\n"),
            "reconstructable" => format!(
                "- {gid} (reconstructable): {n} documents. Source files are not included. Fetch each source URL in release-manifest.tsv and verify its digest.\n"
            ),
            _ => format!("- {gid} (redistributable): {n} documents.\n"),
        };
    }
    text +=
        "\n## Review status\n\nEach shipped document has a label row in release-manifest.tsv.\n\n";
    for class in ["approved", "rejected", "contested", "stale", "unreviewed"] {
        text += &format!(
            "- {class}: {} documents.\n",
            labels.values().filter(|s| s.as_str() == class).count()
        );
    }
    text += "\n## Rights\n\n";
    for gid in gids {
        for row in &rs[gid] {
            text += &rights_line(gid, row, "Source: ", ".");
        }
    }
    text += "\n## Replay\n\nThese commands run in the source repository at the commit that this archive names.\n\n- compile: python3 -P tools/goal.py compile <guideline-id>\n- check: python3 -P tools/goal.py check\n- load: swipl -q -s data/guidelines/<guideline-id>/pl/<docid>.pl\n\n";
    text + schema
}
pub(super) fn member(path: &str, bytes: &[u8]) -> EMember {
    EMember {
        path: path.as_bytes().to_vec(),
        sha: crate::trust::sha256_hex(bytes).into_bytes(),
        size: bytes.len() as u64,
    }
}

pub(super) fn derive(root: &Path) -> Result<ReleasePlan> {
    let head = dgit(
        root,
        &[
            "log",
            "-1",
            "--format=%H",
            "HEAD",
            "--",
            "guidelines",
            "vendor/ape/prolog/ace_to_pl.pl",
            "vendor/clex/clex_lexicon.pl",
        ],
        "no-input head",
    )?;
    let head = String::from_utf8_lossy(&head).trim().to_owned();
    if head.is_empty() {
        return Err(error("no-guidelines"));
    }
    let epoch = dgit(
        root,
        &["show", "-s", "--format=%ct", &head],
        "no-input head",
    )?;
    let epoch = String::from_utf8_lossy(&epoch)
        .trim()
        .parse()
        .map_err(|_| error("no-input head"))?;
    let raw = dgit(
        root,
        &["ls-tree", "-r", "-z", "HEAD", "--", "guidelines"],
        "no-guidelines",
    )?;
    let mut inventory = Vec::new();
    for row in raw.split(|b| *b == 0).filter(|s| !s.is_empty()) {
        let Some(tab) = row.iter().position(|b| *b == b'\t') else {
            return Err(error("no-guidelines"));
        };
        let path = path_text(&row[tab + 1..])?;
        inventory.push((&row[..tab], path));
    }
    if inventory.is_empty() {
        return Err(error("no-guidelines"));
    }
    for (meta, path) in &inventory {
        let f: Vec<&[u8]> = meta.split(|b| *b == b' ').collect();
        if f.len() != 3 || !matches!(f[0], b"100644" | b"100755") || f[1] != b"blob" {
            return Err(error(format!("member-not-regular {path}")));
        }
    }
    let mut gids = BTreeSet::new();
    for (_, path) in &inventory {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() == 2 {
            return Err(error(format!("stray-root-member {path}")));
        }
        gids.insert(parts[1].to_owned());
    }
    let archive = dgit(
        root,
        &["archive", "--format=tar", "HEAD", "guidelines"],
        "no-input archive",
    )?;
    let scratch = Scratch::new()?;
    let (timed, out) = process::walled(
        Command::new("tar").args(["-x", "-C"]).arg(&scratch.0),
        Some(&archive),
        Duration::from_secs(300),
    )?;
    if timed || out.rc != 0 {
        return Err(error("no-input archive"));
    }
    let mut staged = Staged::new();
    for (_, path) in &inventory {
        let archived = scratch.0.join(path);
        // Archive attributes may omit an otherwise valid tracked member.
        if archived.is_file() {
            staged.insert(
                path.clone(),
                fs::read(&archived).map_err(|_| error("no-input archive"))?,
            );
        }
    }
    let compiler = head_file(root, "vendor/ape/prolog/ace_to_pl.pl")?;
    let lexicon = head_file(root, "vendor/clex/clex_lexicon.pl")?;
    let reference = head_file(root, "docs/REFERENCE.md")?;
    let notice = head_file(root, "NOTICE")?;
    let reference =
        std::str::from_utf8(&reference).map_err(|_| error("no-input docs/REFERENCE.md"))?;
    let mut notice = String::from_utf8(notice).map_err(|_| error("no-input docs/REFERENCE.md"))?;
    let schema = schema(reference)?;
    let mut rs = BTreeMap::new();
    for gid in &gids {
        rs.insert(gid.clone(), rights(gid, &staged)?);
    }
    let mut counts = BTreeMap::new();
    let mut labels = BTreeMap::new();
    let mut shipped = 0;
    for gid in &gids {
        let docs = review_docs(gid, &staged);
        counts.insert(gid.clone(), docs.len());
        if rs[gid][0][0] != "restricted" {
            shipped += 1;
            labels.extend(classes(gid, &staged, &docs));
        }
    }
    let readme = readme(&gids, &rs, &counts, &labels, &schema);
    if !notice.ends_with('\n') {
        notice.push('\n');
    }
    notice += "\nRights records for distributed sources:\n";
    for gid in &gids {
        for row in &rs[gid] {
            notice += &rights_line(gid, row, "(", ")");
        }
    }
    notice += "\nThe pl/ Prolog files in the payload are outputs that the vendored ACE compiler derived from the ace/ source documents.\n";
    let tags = Staged::from([
        (
            "bagit.txt".to_owned(),
            b"BagIt-Version: 1.0\nTag-File-Character-Encoding: UTF-8\n".to_vec(),
        ),
        ("README-dist.md".to_owned(), readme.into_bytes()),
        ("NOTICE".to_owned(), notice.into_bytes()),
    ]);
    let tag_members = tags.iter().map(|(p, b)| member(p, b)).collect();
    let members = staged.iter().map(|(p, b)| member(p, b)).collect();
    let profiles = gids
        .iter()
        .map(|g| (g.as_bytes().to_vec(), rs[g][0][0].as_bytes().to_vec()))
        .collect();
    let urls = gids
        .iter()
        .map(|g| (g.as_bytes().to_vec(), rs[g][0][2].as_bytes().to_vec()))
        .collect();
    let label_rows = labels
        .iter()
        .map(|(a, b)| (a.as_bytes().to_vec(), b.as_bytes().to_vec()))
        .collect();
    let manifest = ckc_kernel::contract::release_manifest(
        head.as_bytes(),
        crate::trust::sha256_hex(&compiler).as_bytes(),
        crate::trust::sha256_hex(&lexicon).as_bytes(),
        &members,
        &profiles,
        &urls,
        &label_rows,
        &tag_members,
    );
    let payload = staged
        .into_iter()
        .filter(|(p, _)| {
            let gid = p.split('/').nth(1).unwrap_or("");
            let profile = &rs[gid][0][0];
            profile != "restricted"
                && !(profile == "reconstructable"
                    && p.starts_with(&format!("guidelines/{gid}/source/")))
        })
        .map(|(p, b)| (format!("data/{p}"), b))
        .collect();
    let verdicts = |class: &str| {
        labels
            .iter()
            .filter(|(_, c)| c.as_str() == class)
            .map(|(id, _)| id.clone())
            .collect()
    };
    Ok(ReleasePlan {
        head,
        epoch,
        payload,
        tags,
        manifest,
        rejected: verdicts("rejected"),
        contested: verdicts("contested"),
        shipped,
    })
}

pub(super) fn run() -> Result {
    let plan = derive(Path::new("."))?;
    let target = Path::new("release-manifest.tsv");
    if target.is_symlink() {
        return Err(error("release manifest is a symlink: release-manifest.tsv"));
    }
    fs::write(target, &plan.manifest).map_err(|e| error(e.to_string()))?;
    println!(
        "goal: release-manifest {} guidelines {} members",
        plan.shipped,
        plan.payload.len() + plan.tags.len()
    );
    Ok(())
}
