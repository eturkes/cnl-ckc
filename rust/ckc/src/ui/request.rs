use super::chrome;
use super::common::*;
use super::corpus::Corpus;
use super::{fresh, intake};
use ckc_kernel::{
    EPostDocument, EPostGuideline, EPostOutcome, EPostState, ERequest, EResponse, ESrc,
};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) struct Config {
    pub port: u16,
    pub token: Vec<u8>,
    pub now: Vec<u8>,
    pub commit: Option<Vec<u8>>,
    pub fault: bool,
}
impl Config {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            token: Vec::new(),
            now: Vec::new(),
            commit: None,
            fault: false,
        }
    }
}
pub(super) struct Arguments {
    pub root: PathBuf,
    pub request: ERequest,
    pub config: Config,
}
pub(super) struct Response {
    pub status: u16,
    pub body: Vec<u8>,
    pub media: &'static str,
    allow: Vec<u8>,
    location: Vec<u8>,
}
impl Response {
    fn page(body: Vec<u8>) -> Self {
        Self {
            status: 200,
            body,
            media: "text/html; charset=utf-8",
            allow: Vec::new(),
            location: Vec::new(),
        }
    }
    pub fn headers(&self) -> Result<Vec<(&'static str, String)>> {
        let mut headers = vec![
            ("Content-Type", self.media.to_owned()),
            ("Content-Security-Policy", chrome::canonical()?.csp.clone()),
            ("X-Content-Type-Options", "nosniff".to_owned()),
            ("Referrer-Policy", "no-referrer".to_owned()),
            ("Cache-Control", "no-store".to_owned()),
        ];
        if !self.allow.is_empty() {
            headers.push(("Allow", text(&self.allow)));
        }
        if !self.location.is_empty() {
            headers.push(("Location", text(&self.location)));
        }
        Ok(headers)
    }
    pub fn reason(&self) -> &'static str {
        match self.status {
            200 => "OK",
            303 => "See Other",
            400 => "Bad Request",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            409 => "Conflict",
            _ => "Internal Server Error",
        }
    }
}
impl From<EResponse> for Response {
    fn from(r: EResponse) -> Self {
        Self {
            status: r.status,
            body: r.body,
            allow: r.allow,
            location: r.location,
            media: "text/html; charset=utf-8",
        }
    }
}
pub(super) enum Outcome {
    Response(Response),
    Crash,
}

fn kernel_response(req: &ERequest, state: &EPostState) -> Result<Response> {
    match ckc_kernel::contract::ui_post_outcome(req, state) {
        EPostOutcome::Refused(r) => Ok(r.into()),
        _ => Err("ui: response binding did not return a refusal".into()),
    }
}
fn empty_request(method: &[u8], path: &[u8]) -> ERequest {
    ERequest {
        method: method.to_vec(),
        path: path.to_vec(),
        host: b"127.0.0.1:8377".to_vec(),
        origin: None,
        content_type: Vec::new(),
        body: None,
    }
}
pub(super) fn server_error(detail: String) -> Result<Response> {
    // Shell I/O errors use the same verified error-page branch as model errors.
    kernel_response(
        &empty_request(b"GET", b"/"),
        &EPostState {
            port: 8377,
            token: Vec::new(),
            models: Err(detail.into_bytes()),
            now: Vec::new(),
        },
    )
}
fn not_found() -> Result<Response> {
    kernel_response(
        &empty_request(b"POST", b"/g/x/doc/x.html"),
        &EPostState {
            port: 8377,
            token: Vec::new(),
            models: Ok(Vec::new()),
            now: Vec::new(),
        },
    )
}
fn clone_source(src: &ESrc) -> ESrc {
    match src {
        ESrc::Missing => ESrc::Missing,
        ESrc::Bad(n) => ESrc::Bad(*n),
        ESrc::Bytes(b) => ESrc::Bytes(b.clone()),
    }
}
fn route(path: &[u8]) -> Option<(&str, &str)> {
    let p = std::str::from_utf8(path).ok()?.strip_prefix("/g/")?;
    let parts = p.split('/').collect::<Vec<_>>();
    if parts.len() != 3 || parts[0].is_empty() || parts[1] != "doc" {
        return None;
    }
    let id = parts[2].strip_suffix(".html").filter(|s| !s.is_empty())?;
    Some((parts[0], id))
}
fn post_models(
    corpus: &Corpus,
    view: &intake::View,
    req: &ERequest,
    config: &Config,
) -> Vec<EPostGuideline> {
    let target = route(&req.path);
    view.corpus
        .guidelines
        .iter()
        .enumerate()
        .map(|(gi, g)| {
            let gid = text(&g.gid);
            let active = req.method == b"POST" && target.is_some_and(|(id, _)| id == gid);
            let documents = g
                .documents
                .iter()
                .enumerate()
                .map(|(di, d)| {
                    let commit = if active
                        && target.is_some_and(|(_, id)| id.as_bytes() == d.bundle.docid)
                    {
                        config
                            .commit
                            .clone()
                            .unwrap_or_else(|| corpus.ace_commit(&gid, &text(&d.bundle.docid)))
                    } else {
                        Vec::new()
                    };
                    EPostDocument {
                        docid: d.bundle.docid.clone(),
                        render_error: view.errors[gi][di].as_ref().map(|s| s.as_bytes().to_vec()),
                        commit,
                    }
                })
                .collect();
            let fresh = if active {
                fresh::derive(&corpus.root.join("guidelines").join(&gid))
                    .map_err(String::into_bytes)
            } else {
                Ok(Vec::new())
            };
            EPostGuideline {
                gid: g.gid.clone(),
                documents,
                fresh,
                ledger: clone_source(&g.ledger),
                ledger_digest: g.ledger_digest.clone(),
            }
        })
        .collect()
}
fn read(corpus: &Corpus, view: &intake::View, req: &ERequest) -> Result<Response> {
    let path = text(&req.path);
    if path == "/" || path == "/index.html" {
        return Ok(Response::page(ckc_kernel::contract::ui_render_index(
            &view.corpus,
        )));
    }
    let Some(tail) = path.strip_prefix("/g/") else {
        return not_found();
    };
    let parts = tail.split('/').collect::<Vec<_>>();
    let Some((gi, g)) = view
        .corpus
        .guidelines
        .iter()
        .enumerate()
        .find(|(_, g)| g.gid == parts[0].as_bytes())
    else {
        return not_found();
    };
    if parts.len() == 2 {
        return match parts[1] {
            "" | "index.html" => Ok(Response::page(ckc_kernel::contract::ui_render_guideline(g))),
            "records.html" => Ok(Response::page(ckc_kernel::contract::ui_render_records(g))),
            _ => not_found(),
        };
    }
    if parts.len() != 3 {
        return not_found();
    }
    if parts[1] == "source" {
        if !g.source_names.iter().any(|n| n == parts[2].as_bytes()) {
            return not_found();
        }
        let source = corpus
            .root
            .join("guidelines")
            .join(parts[0])
            .join("source")
            .join(parts[2]);
        let Ok(bytes) = fs::read(source) else {
            return not_found();
        };
        let mut response = Response::page(bytes);
        response.media = intake::media_type(parts[2]).ok_or("ui: source media type missing")?;
        return Ok(response);
    }
    if parts[1] != "doc" {
        return not_found();
    }
    let Some(id) = parts[2].strip_suffix(".html") else {
        return not_found();
    };
    let Some((di, d)) = g
        .documents
        .iter()
        .enumerate()
        .find(|(_, d)| d.bundle.docid == id.as_bytes())
    else {
        return not_found();
    };
    if let Some(error) = &view.errors[gi][di] {
        return server_error(error.clone());
    }
    let prev = di
        .checked_sub(1)
        .map(|i| g.documents[i].bundle.docid.as_slice())
        .unwrap_or(b"");
    let next = g
        .documents
        .get(di + 1)
        .map(|d| d.bundle.docid.as_slice())
        .unwrap_or(b"");
    Ok(Response::page(ckc_kernel::contract::ui_render_document(
        g,
        d,
        prev,
        next,
        &view.corpus.token,
    )))
}
pub(super) fn respond(root: &Path, req: &ERequest, config: &Config) -> Result<Outcome> {
    let corpus = match Corpus::committed(root) {
        Ok(corpus) => corpus,
        Err(e) => return server_error(e).map(Outcome::Response),
    };
    let view = intake::load(&corpus, &config.token);
    let models = match &view {
        Ok(view) => Ok(post_models(&corpus, view, req, config)),
        Err(e) => Err(e.as_bytes().to_vec()),
    };
    let now = if !config.now.is_empty() {
        config.now.clone()
    } else if req.method == b"POST" {
        utc_now()?.into_bytes()
    } else {
        Vec::new()
    };
    let mut state = EPostState {
        port: config.port,
        token: config.token.clone(),
        models,
        now,
    };
    match ckc_kernel::contract::ui_post_outcome(req, &state) {
        EPostOutcome::Refused(response) => Ok(Outcome::Response(response.into())),
        EPostOutcome::Read => read(&corpus, &view?, req).map(Outcome::Response),
        EPostOutcome::Prepared {
            candidate,
            expected_ledger,
            response,
        } => install(
            &corpus,
            req,
            config,
            &mut state,
            &candidate,
            &expected_ledger,
            response,
        ),
    }
}
struct Candidate {
    path: PathBuf,
    remove: bool,
}
impl Drop for Candidate {
    fn drop(&mut self) {
        if self.remove {
            let _ = fs::remove_file(&self.path);
        }
    }
}
fn candidate_file(audit: &Path, bytes: &[u8]) -> Result<Candidate> {
    for _ in 0..32 {
        let nonce = random_hex(12).map_err(|_| "ui: verdict: ledger write failed")?;
        let path = audit.join(format!(".adjudication.tsv.{nonce}"));
        let mut file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
        {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => return Err("ui: verdict: ledger write failed".into()),
        };
        let candidate = Candidate { path, remove: true };
        file.write_all(bytes)
            .and_then(|()| file.sync_all())
            .and_then(|()| file.set_permissions(fs::Permissions::from_mode(0o644)))
            .map_err(|_| "ui: verdict: ledger write failed")?;
        return Ok(candidate);
    }
    Err("ui: verdict: ledger write failed".into())
}
struct LedgerLock {
    path: PathBuf,
    file: File,
}
impl LedgerLock {
    fn acquire(audit: &Path) -> Result<Self> {
        let path = audit.join(".adjudication.lock");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .open(&path)
            .map_err(|_| "ui: verdict: ledger write failed")?;
        file.lock()
            .map_err(|_| "ui: verdict: ledger write failed")?;
        Ok(Self { path, file })
    }
}
impl Drop for LedgerLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
        let _ = self.file.unlock();
    }
}
fn install(
    corpus: &Corpus,
    req: &ERequest,
    config: &Config,
    state: &mut EPostState,
    bytes: &[u8],
    expected: &[u8],
    response: EResponse,
) -> Result<Outcome> {
    let (gid, _) = route(&req.path).ok_or("ui: prepared response has no document route")?;
    let audit = corpus.real_root.join("guidelines").join(gid).join("audit");
    let ledger = audit.join("adjudication.tsv");
    if ledger.is_symlink() || (ledger.exists() && !ledger.is_file()) {
        return server_error("ui: verdict: ledger not a regular file".into())
            .map(Outcome::Response);
    }
    let mut candidate = match candidate_file(&audit, bytes) {
        Ok(candidate) => candidate,
        Err(e) => return server_error(e).map(Outcome::Response),
    };
    if config.fault {
        candidate.remove = false;
        return Ok(Outcome::Crash);
    }
    let lock = match LedgerLock::acquire(&audit) {
        Ok(lock) => lock,
        Err(e) => return server_error(e).map(Outcome::Response),
    };
    let current = if ledger.exists() {
        fs::read(&ledger).ok().map(|b| digest(&b))
    } else {
        Some(b"absent".to_vec())
    };
    if current.as_deref() != Some(expected) {
        drop(lock);
        if let Ok(models) = &mut state.models {
            if let Some(model) = models.iter_mut().find(|m| m.gid == gid.as_bytes()) {
                model.ledger_digest = current.unwrap_or_default();
            }
        }
        return kernel_response(req, state).map(Outcome::Response);
    }
    if fs::rename(&candidate.path, &ledger).is_err() {
        drop(lock);
        return server_error("ui: verdict: ledger write failed".into()).map(Outcome::Response);
    }
    candidate.remove = false;
    drop(lock);
    Ok(Outcome::Response(response.into()))
}

pub(super) fn decode_path(raw: &str) -> Vec<u8> {
    let bytes = raw.split('?').next().unwrap_or("").as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let value = char::from(bytes[i + 1])
                .to_digit(16)
                .zip(char::from(bytes[i + 2]).to_digit(16));
            if let Some((hi, lo)) = value {
                out.push((hi * 16 + lo) as u8);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    text(&out).into_bytes()
}
fn unhex(s: &str) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut bytes = s.bytes().peekable();
    while let Some(c) = bytes.next() {
        if c.is_ascii_whitespace() {
            continue;
        }
        let hi = char::from(c).to_digit(16)?;
        let lo = char::from(bytes.next()?).to_digit(16)?;
        out.push((hi * 16 + lo) as u8);
    }
    Some(out)
}
fn valid_date(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 20 {
        return false;
    }
    for (i, c) in b.iter().enumerate() {
        let separator = match i {
            4 | 7 => Some(b'-'),
            10 => Some(b'T'),
            13 | 16 => Some(b':'),
            19 => Some(b'Z'),
            _ => None,
        };
        if separator.map_or(!c.is_ascii_digit(), |separator| *c != separator) {
            return false;
        }
    }
    let num = |a, z| s[a..z].parse::<u32>().unwrap_or(0);
    let (year, month, day, hour, minute, second) = (
        num(0, 4),
        num(5, 7),
        num(8, 10),
        num(11, 13),
        num(14, 16),
        num(17, 19),
    );
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let days = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    year > 0
        && (1..=12).contains(&month)
        && (1..=days).contains(&day)
        && hour < 24
        && minute < 60
        && second < 60
}
fn utc_now() -> Result<String> {
    let seconds = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => i64::try_from(d.as_secs()).map_err(|_| "ui: clock out of range")?,
        Err(e) => {
            -i64::try_from(e.duration().as_secs()).map_err(|_| "ui: clock out of range")?
                - i64::from(e.duration().subsec_nanos() != 0)
        }
    };
    let day = seconds.div_euclid(86400);
    let tod = seconds.rem_euclid(86400);
    let z = day + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    if !(1..=9999).contains(&year) {
        return Err("ui: clock out of range".into());
    }
    Ok(format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        tod / 3600,
        tod / 60 % 60,
        tod % 60
    ))
}
pub(super) fn parse(args: &[String]) -> Option<Arguments> {
    if args.len() < 2 {
        return None;
    }
    let mut root = None;
    let mut headers = BTreeMap::new();
    let mut config = Config::new(8377);
    let mut body = None;
    let mut seen = BTreeMap::new();
    let mut flags = false;
    let mut i = 2;
    while i < args.len() {
        let flag = args[i].as_str();
        if [
            "--header",
            "--body",
            "--body-hex",
            "--token",
            "--now",
            "--commit",
            "--fault",
        ]
        .contains(&flag)
        {
            flags = true;
            i += 1;
            let value = args.get(i)?;
            let key = if flag == "--body-hex" { "--body" } else { flag };
            if flag != "--header" && seen.insert(key, ()).is_some() {
                return None;
            }
            match flag {
                "--header" => {
                    let (name, value) = value.split_once(':')?;
                    if name.is_empty() {
                        return None;
                    }
                    headers.insert(name.to_lowercase(), value.to_owned());
                }
                "--body" => body = Some(value.as_bytes().to_vec()),
                "--body-hex" => body = Some(unhex(value)?),
                "--token" => config.token = value.as_bytes().to_vec(),
                "--now" => {
                    if !valid_date(value) {
                        return None;
                    }
                    config.now = value.as_bytes().to_vec();
                }
                "--commit" => {
                    if !valid_hex(value, 40) {
                        return None;
                    }
                    config.commit = Some(value.as_bytes().to_vec());
                }
                _ => {
                    if value != "after-tmp-write" {
                        return None;
                    }
                    config.fault = true;
                }
            }
        } else {
            if flag.starts_with("--") || flags || root.is_some() {
                return None;
            }
            root = Some(PathBuf::from(flag));
        }
        i += 1;
    }
    let host = headers
        .remove("host")
        .unwrap_or_else(|| "127.0.0.1:8377".into())
        .into_bytes();
    let origin = headers.remove("origin").map(String::into_bytes);
    let content_type = headers
        .remove("content-type")
        .unwrap_or_else(|| {
            if body.is_some() {
                "application/x-www-form-urlencoded".into()
            } else {
                String::new()
            }
        })
        .into_bytes();
    let request = ERequest {
        method: args[0].as_bytes().to_vec(),
        path: decode_path(&args[1]),
        host,
        origin,
        content_type,
        body,
    };
    Some(Arguments {
        root: root.unwrap_or_else(|| PathBuf::from(".")),
        request,
        config,
    })
}
pub(super) fn execute(args: &Arguments) -> Result<u8> {
    match respond(&args.root, &args.request, &args.config)? {
        Outcome::Crash => Ok(3),
        Outcome::Response(response) => {
            let mut bytes = format!("HTTP {}\n", response.status).into_bytes();
            for (name, value) in response.headers()? {
                bytes.extend_from_slice(format!("{name}: {value}\n").as_bytes());
            }
            bytes.push(b'\n');
            bytes.extend_from_slice(&response.body);
            std::io::stdout()
                .write_all(&bytes)
                .map_err(|e| format!("ui: response: {e}"))?;
            Ok(0)
        }
    }
}
