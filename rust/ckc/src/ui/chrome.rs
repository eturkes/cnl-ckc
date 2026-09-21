use super::common::*;
use ckc_kernel::{
    EBundle, ECoverage, ECoverageRow, EDocument, EEvidence, EGuideline, ESrc, EStatus,
};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

pub(super) const COMMIT_BASE: &str = "https://github.com/eturkes/cnl-ckc/commit/";
pub(super) const SCOPE: &str = "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>";
pub(super) struct Chrome {
    pub css: String,
    pub script: String,
    pub csp: String,
}
static CHROME: OnceLock<Result<Chrome>> = OnceLock::new();

fn between<'a>(s: &'a str, open: &str, close: &str) -> Result<&'a str> {
    s.split_once(open)
        .and_then(|(_, tail)| tail.split_once(close))
        .map(|(value, _)| value)
        .ok_or_else(|| "ui: canonical page chrome missing".into())
}
fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let a = chunk[0];
        let b = chunk.get(1).copied().unwrap_or(0);
        let c = chunk.get(2).copied().unwrap_or(0);
        out.push(char::from(TABLE[(a >> 2) as usize]));
        out.push(char::from(TABLE[((a & 3) << 4 | b >> 4) as usize]));
        out.push(if chunk.len() > 1 {
            char::from(TABLE[((b & 15) << 2 | c >> 6) as usize])
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            char::from(TABLE[(c & 63) as usize])
        } else {
            '='
        });
    }
    out
}
fn derive() -> Result<Chrome> {
    // The kernel is the only source of chrome bytes. One aligned typed value
    // exposes its fixed script even when the loaded corpus has no alignments.
    let g = EGuideline {
        gid: b"x".to_vec(),
        readme: None,
        coverage: ECoverage {
            rows: vec![ECoverageRow {
                id: b"x".to_vec(),
                file: b"source/x.txt".to_vec(),
                status: EStatus::Ace(b"x".to_vec()),
                line: b"x\tsource/x.txt\tp1\tx\tace(x)\n".to_vec(),
            }],
            files: vec![b"source/x.txt".to_vec()],
            evidence: vec![EEvidence {
                census: b"1".to_vec(),
                locators: vec![b"x".to_vec()],
                payloads: vec![(b"x".to_vec(), vec![b"x".to_vec()])],
                ordinal: Vec::new(),
            }],
        },
        documents: vec![EDocument {
            bundle: EBundle {
                docid: b"x".to_vec(),
                ace: Vec::new(),
                cov: Vec::new(),
                pay: Vec::new(),
                cl: Vec::new(),
                review: Vec::new(),
            },
            ace: b"x".to_vec(),
            pl: Vec::new(),
            alignment: Some(
                b"# format: group<TAB>side<TAB>start<TAB>span\n1\tsrc\t0\tx\n1\tace\t0\tx\n"
                    .to_vec(),
            ),
        }],
        ledger: ESrc::Missing,
        ledger_digest: b"absent".to_vec(),
        source_names: Vec::new(),
    };
    let page = text(&ckc_kernel::contract::ui_render_document(
        &g,
        &g.documents[0],
        b"",
        b"",
        b"",
    ));
    let css = between(&page, "<style>", "</style>")?.to_owned();
    let code = between(&page, "<script>", "</script>")?;
    let script = format!("<script>{code}</script>");
    let hash = base64(&Sha256::digest(code.as_bytes()));
    Ok(Chrome {
        css,
        script,
        csp: format!("default-src 'none'; style-src 'unsafe-inline'; script-src 'sha256-{hash}'"),
    })
}
pub(super) fn canonical() -> Result<&'static Chrome> {
    CHROME.get_or_init(derive).as_ref().map_err(Clone::clone)
}
