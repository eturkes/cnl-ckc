use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub(super) type Result<T> = std::result::Result<T, String>;

pub(super) fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}
pub(super) fn digest(bytes: &[u8]) -> Vec<u8> {
    crate::trust::sha256_hex(bytes).into_bytes()
}
pub(super) fn valid_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 250
        && !s.starts_with('-')
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}
pub(super) fn valid_hex(s: &str, len: usize) -> bool {
    s.len() == len && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub(super) fn entries(path: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = fs::read_dir(path)
        .map_err(|e| format!("ui: read {}: {e}", path.display()))?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|e| format!("ui: read {}: {e}", path.display()))?;
    paths.sort();
    Ok(paths)
}
pub(super) fn name(path: &Path) -> String {
    path.file_name().unwrap_or_default().to_string_lossy().into_owned()
}
pub(super) fn checked_text(bytes: Vec<u8>, gid: &str, rel: &str) -> Result<Vec<u8>> {
    let decoded = std::str::from_utf8(&bytes)
        .map_err(|_| format!("ui: viewmodel: {gid} file not UTF-8: {rel}"))?;
    if let Some(cp) = decoded.chars().map(u32::from).find(|&c| {
        (c < 32 && c != 9 && c != 10)
            || c == 127
            || (0x202a..=0x202e).contains(&c)
            || (0x2066..=0x2069).contains(&c)
    }) {
        return Err(format!("ui: viewmodel: {gid} unsupported control U+{cp:04X} in {rel}"));
    }
    Ok(bytes)
}
pub(super) fn load_text(root: &Path, gid: &str, rel: &str) -> Result<Vec<u8>> {
    let bytes = fs::read(root.join(rel))
        .map_err(|_| format!("ui: viewmodel: {gid} missing {rel}"))?;
    checked_text(bytes, gid, rel)
}
pub(super) fn random_hex(n: usize) -> std::io::Result<String> {
    let mut bytes = vec![0; n];
    File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

pub(super) struct Scratch(pub PathBuf);
impl Scratch {
    pub fn new() -> Result<Self> {
        for _ in 0..32 {
            let nonce = random_hex(12).map_err(|e| format!("ui: temporary directory: {e}"))?;
            let path = std::env::temp_dir().join(format!("ckc-ui-{nonce}"));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
                Err(e) => return Err(format!("ui: temporary directory: {e}")),
            }
        }
        Err("ui: temporary directory: name collision".into())
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
