use super::common::*;
use super::{process, text};
use std::path::Path;
use std::process::Command;
use std::time::Duration;
pub(super) struct Counts {
    count: usize,
    wh: usize,
    yesno: usize,
    nodes: usize,
}
impl Counts {
    pub fn query_meter(&self, gid: &str) -> String {
        format!(
            "goal: queries {gid} {} queries; wh={} yesno={}",
            self.count, self.wh, self.yesno
        )
    }
    pub fn trace_meter(&self, gid: &str) -> String {
        format!(
            "goal: traces {gid} {} traces; nodes={}",
            self.count, self.nodes
        )
    }
}
pub(super) fn manifest(path: &Path, pl: &Path) -> Result {
    let mut bytes = String::new();
    if pl.is_dir() {
        for path in entries(pl, "queries")? {
            if name(&path).ends_with(".pl") {
                bytes += &format!("{}\t{}\n", show(&path), show(&path));
            }
        }
    }
    process::write(path, bytes.as_bytes())
}
pub(super) fn kernel(
    mode: &str,
    paths: &[&Path],
    seconds: u64,
    category: &str,
    timeout: &str,
) -> Result<process::Output> {
    // A child enters the public v1 binding; the parent retains the legacy wall.
    let executable = std::env::current_exe().map_err(|e| fail("subprocess", e.to_string()))?;
    let mut cmd = Command::new(executable);
    cmd.args(["v1", mode]).args(paths);
    let (timed, result) = process::walled(&mut cmd, None, Duration::from_secs(seconds))?;
    if timed {
        return Err(if category == "swipl-timeout" {
            fail(category, timeout)
        } else {
            violation(category, timeout)
        });
    }
    Ok(result)
}
fn artifact(out: process::Output, kind: &str, id: &str) -> Result<Vec<u8>> {
    if out.rc != 0 {
        return Err(out.relay());
    }
    if !out.err.is_empty() {
        return Err(fail(
            &format!("{kind}-stderr"),
            format!("non-empty stderr for question: {id}"),
        ));
    }
    if out.out.is_empty() {
        return Err(fail(
            &format!("{kind}-stdout"),
            format!("empty stdout for question: {id}"),
        ));
    }
    if !out.out.ends_with(b"\n") {
        return Err(fail(
            &format!("{kind}-stdout"),
            format!("missing final newline for question: {id}"),
        ));
    }
    Ok(out.out)
}
pub(super) fn question(
    swipl: &Path,
    stage: &Path,
    id: &str,
    ace: &Path,
    lexicon: Option<&Path>,
) -> Result<Vec<u8>> {
    let input = read(ace, "queries")?;
    let mut tail = vec!["question".to_owned(), show(stage), id.to_owned()];
    if let Some(p) = lexicon {
        tail.push(show(p));
    }
    let (timed, out) = process::walled(
        &mut process::compiler(swipl, stage, &tail),
        Some(&input),
        Duration::from_secs(30),
    )?;
    if timed {
        return Err(violation("queries", format!("wall_clock for qid: {id}")));
    }
    artifact(out, "compiler", id)
}
pub(super) fn answer(id: &str, manifest: &Path, query: &Path) -> Result<Vec<u8>> {
    artifact(
        kernel(
            "answer",
            &[manifest, query],
            30,
            "queries",
            &format!("wall_clock for qid: {id}"),
        )?,
        "answer",
        id,
    )
}
pub(super) fn trace_raw(
    manifest: &Path,
    query: &Path,
    answers: &Path,
    timeout_category: &str,
    timeout_detail: &str,
) -> Result<process::Output> {
    kernel(
        "trace",
        &[manifest, query, answers],
        30,
        timeout_category,
        timeout_detail,
    )
}
pub(super) fn trace(id: &str, manifest: &Path, query: &Path, answers: &Path) -> Result<Vec<u8>> {
    artifact(
        trace_raw(
            manifest,
            query,
            answers,
            "queries",
            &format!("wall_clock for qid: {id}"),
        )?,
        "trace",
        id,
    )
}
fn trace_join_error(bytes: &[u8], id: &str) -> Option<Failure> {
    // Decode the verified diagnostic only; the shell never walks proof terms.
    let detail = std::str::from_utf8(bytes)
        .ok()?
        .strip_prefix("ace_to_pl_error(proof,trace_check(join(sentence(")?
        .strip_suffix("))).\n")?;
    let (sentence, count) = detail.split_once("),")?;
    let (docid, ordinal) = sentence.rsplit_once(',')?;
    let docid = docid
        .strip_prefix('\'')
        .and_then(|s| s.strip_suffix('\''))
        .unwrap_or(docid);
    if !valid_docid(docid) || ordinal.parse::<u64>().ok()? == 0 {
        return None;
    }
    let quantity = match count.parse::<usize>().ok()? {
        0 => "no",
        1 => return None,
        _ => "multiple",
    };
    Some(violation(
        "traces",
        format!(
            "trace node resolves to {quantity} committed clause line{}: {id} {docid} S{ordinal}",
            if quantity == "no" { "" } else { "s" }
        ),
    ))
}
pub(super) fn inspect_trace(
    id: &str,
    manifest: &Path,
    query: &Path,
    answers: &Path,
    path: &Path,
) -> Result<usize> {
    let out = kernel(
        "trace-check",
        &[manifest, query, answers, path],
        30,
        "queries",
        &format!("wall_clock for qid: {id}"),
    )?;
    if out.rc != 0 && out.out.is_empty() {
        let error = match (out.rc, out.err.as_slice()) {
            (1, b"ace_to_pl_error(proof,trace_check(stale)).\n") => Some(violation(
                "stale",
                format!(
                    "committed query trace differs from fresh trace: {}",
                    show(path)
                ),
            )),
            (1, b"ace_to_pl_error(proof,trace_check(non_demo)).\n") => {
                Some(violation("traces", format!("non-demo proof for qid: {id}")))
            }
            (1, b"ace_to_pl_error(proof,trace_check(node_shape)).\n")
            | (2, b"ace_to_pl_error(check_load,trace_file(noncanonical)).\n") => Some(violation(
                "traces",
                format!("malformed trace artifact: {}", show(path)),
            )),
            (1, bytes) => trace_join_error(bytes, id),
            _ => None,
        };
        if let Some(error) = error {
            return Err(error);
        }
    }
    let bytes = artifact(out, "trace-check", id)?;
    std::str::from_utf8(&bytes)
        .ok()
        .and_then(|s| s.strip_prefix(&format!("ckc: trace-check ok {id} nodes=")))
        .and_then(|s| s.strip_suffix('\n'))
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| {
            fail(
                "trace-check-stdout",
                format!("invalid meter for question: {id}"),
            )
        })
}
pub(super) fn query_aces(root: &Path) -> Result<Vec<String>> {
    if root.is_symlink() {
        return Err(violation(
            "queries",
            format!("is a symlink: {}", show(root)),
        ));
    }
    if !root.exists() {
        return Ok(vec![]);
    }
    if !root.is_dir() {
        return Err(violation(
            "queries",
            format!("not a directory: {}", show(root)),
        ));
    }
    let mut ids = vec![];
    for p in entries(root, "queries")? {
        let n = name(&p);
        if ["pl", "answers", "traces"].contains(&n.as_str()) {
            if p.is_symlink() || !p.is_dir() {
                return Err(violation(
                    "queries",
                    format!("unsupported entry: {}", show(&p)),
                ));
            }
        } else {
            if !n.ends_with(".ace") {
                return Err(violation(
                    "queries",
                    format!("unsupported entry: {}", show(&p)),
                ));
            }
            if p.is_symlink() || !p.is_file() {
                return Err(violation(
                    "queries",
                    format!("not a regular file: {}", show(&p)),
                ));
            }
            let id = n.strip_suffix(".ace").unwrap_or("");
            if !valid_docid(id) {
                return Err(violation("queries", format!("invalid qid filename: {n}")));
            }
            ids.push(id.to_owned());
        }
    }
    Ok(ids)
}
fn query_dir(root: &Path, subdir: &str) -> Result<Vec<String>> {
    let path = root.join(subdir);
    if !path.is_dir() {
        return Ok(vec![]);
    }
    let mut ids = vec![];
    for p in entries(&path, "queries")? {
        let n = name(&p);
        if !n.ends_with(".pl") {
            return Err(violation(
                "queries",
                format!("unsupported entry: {}", show(&p)),
            ));
        }
        if p.is_symlink() || !p.is_file() {
            return Err(violation(
                "queries",
                format!("not a regular file: {}", show(&p)),
            ));
        }
        let id = n.strip_suffix(".pl").unwrap_or("");
        if !valid_docid(id) {
            return Err(violation("queries", format!("invalid qid filename: {n}")));
        }
        ids.push(id.to_owned());
    }
    Ok(ids)
}
fn answer_result(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let lines: Vec<&str> = text::lines(&text).collect();
    if lines.len() != 2 {
        return String::new();
    }
    lines[1]
        .split_once("result(")
        .and_then(|(_, s)| s.strip_suffix("))."))
        .unwrap_or("")
        .to_owned()
}
pub(super) fn validate(
    scratch: &process::Scratch,
    swipl: &Path,
    stage: &Path,
    gid: &Path,
    lexicon: Option<&Path>,
) -> Result<Counts> {
    let root = gid.join("queries");
    let qids = query_aces(&root)?;
    let pl = query_dir(&root, "pl")?;
    let answers = query_dir(&root, "answers")?;
    let traces = query_dir(&root, "traces")?;
    for (ids, dir) in [(&pl, "pl"), (&answers, "answers"), (&traces, "traces")] {
        if *ids != qids {
            return Err(violation(
                "queries",
                format!(
                    "{dir} inventory differs from ace query set: {}",
                    show(&root.join(dir))
                ),
            ));
        }
    }
    let mut counts = Counts {
        count: qids.len(),
        wh: 0,
        yesno: 0,
        nodes: 0,
    };
    if qids.is_empty() {
        return Ok(counts);
    }
    let mpath = scratch.0.join(format!("queries-manifest-{}", name(gid)));
    manifest(&mpath, &gid.join("pl"))?;
    for id in qids {
        let ace = root.join(format!("{id}.ace"));
        let first = question(swipl, stage, &id, &ace, lexicon)?;
        let second = question(swipl, stage, &id, &ace, lexicon)?;
        if first != second {
            return Err(violation(
                "determinism",
                format!("two query compiles differ for question: {id}"),
            ));
        }
        let pl = root.join("pl").join(format!("{id}.pl"));
        if first != read(&pl, "queries")? {
            return Err(violation(
                "stale",
                format!(
                    "committed query pl differs from fresh compile: {}",
                    show(&pl)
                ),
            ));
        }
        let first_answer = answer(&id, &mpath, &pl)?;
        let second_answer = answer(&id, &mpath, &pl)?;
        if first_answer != second_answer {
            return Err(violation(
                "determinism",
                format!("two answer runs differ for question: {id}"),
            ));
        }
        let answers = root.join("answers").join(format!("{id}.pl"));
        if first_answer != read(&answers, "queries")? {
            return Err(violation(
                "stale",
                format!(
                    "committed query answers differ from fresh answer: {}",
                    show(&answers)
                ),
            ));
        }
        let result = answer_result(&first_answer);
        if result.is_empty() {
            return Err(violation(
                "queries",
                format!("unparsable answer artifact for qid: {id}"),
            ));
        }
        if result != "yes" && !(result.starts_with("solutions(") && result != "solutions([])") {
            return Err(violation(
                "queries",
                format!("non-demo result for qid {id}: {result}"),
            ));
        }
        let text = String::from_utf8_lossy(&first);
        let lines: Vec<&str> = text::lines(&text).collect();
        if lines.len() != 4 {
            return Err(violation(
                "queries",
                format!("unexpected query pl shape for qid: {id}"),
            ));
        }
        if lines[3].ends_with("answers([])).") {
            counts.yesno += 1;
        } else {
            counts.wh += 1;
        }
        let first_trace = trace(&id, &mpath, &pl, &answers)?;
        let second_trace = trace(&id, &mpath, &pl, &answers)?;
        if first_trace != second_trace {
            return Err(violation(
                "determinism",
                format!("two trace runs differ for question: {id}"),
            ));
        }
        let path = root.join("traces").join(format!("{id}.pl"));
        let committed = read(&path, "traces")?;
        if first_trace != committed {
            return Err(violation(
                "stale",
                format!(
                    "committed query trace differs from fresh trace: {}",
                    show(&path)
                ),
            ));
        }
        counts.nodes += inspect_trace(&id, &mpath, &pl, &answers, &path)?;
    }
    Ok(counts)
}
pub(super) fn fixture(
    scratch: &process::Scratch,
    swipl: &Path,
    stage: &Path,
    gid: &Path,
) -> Result<Vec<u8>> {
    if gid.is_symlink() {
        return Err(fail("guideline", format!("is a symlink: {}", show(gid))));
    }
    if !gid.is_dir() {
        return Err(fail("guideline", format!("not a directory: {}", show(gid))));
    }
    let lexicon = gid.join("lexicon.ulex");
    if lexicon.is_symlink() {
        return Err(fail(
            "guideline",
            format!("lexicon is a symlink: {}", show(&lexicon)),
        ));
    }
    let lexicon = lexicon.is_file().then_some(lexicon);
    let swipl = process::swipl().map(|_| swipl)?;
    if !stage.is_dir() {
        return Err(fail("ape-stage", format!("missing stage: {}", show(stage))));
    }
    let counts = validate(scratch, swipl, stage, gid, lexicon.as_deref())?;
    Ok(format!("{}\n", counts.query_meter(&name(gid))).into_bytes())
}
