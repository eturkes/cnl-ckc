use super::common::*;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

// The browser opens a private file that redirects to the sign-in address, so
// the launch secret never reaches a process argv (`/proc/<pid>/cmdline` is
// readable by every account) or the terminal while a browser opens.
pub(super) struct Opener {
    file: PathBuf,
    address: String,
    _dir: Option<Scratch>,
}
impl Opener {
    pub fn new(port: u16, session: &[u8]) -> Result<Self> {
        let (dir, scratch) = match private_runtime_dir() {
            Some(dir) => (dir, None),
            None => {
                let scratch = Scratch::new()?;
                (scratch.0.clone(), Some(scratch))
            }
        };
        let file = dir.join(format!("ckc-ui-{port}.html"));
        let _ = fs::remove_file(&file);
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&file)
            .and_then(|mut out| out.write_all(&ckc_kernel::contract::ui_signin_page(port, session)))
            .map_err(|e| format!("ui: serve: {e}"))?;
        Ok(Self {
            file,
            address: format!("http://127.0.0.1:{port}/session/{}", text(session)),
            _dir: scratch,
        })
    }
    pub fn announce(&self) {
        if self.open() {
            println!(
                "ui: the review site is open in your browser. Press Enter to open it again. Press Ctrl+C to stop."
            );
        } else {
            println!("ui: no browser opened. Open this address: {}", self.address);
        }
    }
    fn open(&self) -> bool {
        for argv in openers() {
            let Ok(mut child) = Command::new(&argv[0])
                .args(&argv[1..])
                .arg(&self.file)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            else {
                continue;
            };
            // A launcher (xdg-open) exits once the browser has the page; a
            // browser that stays in the foreground counts as opened.
            let until = Instant::now() + Duration::from_secs(5);
            loop {
                match child.try_wait() {
                    Ok(Some(status)) if status.success() => return true,
                    Ok(Some(_)) | Err(_) => break,
                    Ok(None) if Instant::now() >= until => {
                        std::thread::spawn(move || child.wait());
                        return true;
                    }
                    Ok(None) => std::thread::sleep(Duration::from_millis(50)),
                }
            }
        }
        false
    }
}
impl Drop for Opener {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.file);
    }
}
fn private_runtime_dir() -> Option<PathBuf> {
    let dir = PathBuf::from(std::env::var_os("XDG_RUNTIME_DIR")?);
    let meta = fs::symlink_metadata(&dir).ok()?;
    let own = fs::metadata("/proc/self").ok()?.uid();
    (meta.is_dir() && meta.uid() == own && meta.permissions().mode() & 0o777 == 0o700)
        .then_some(dir)
}
// `$BROWSER` (its first `:` entry), then the desktop + WSL launchers.
fn openers() -> Vec<Vec<String>> {
    let mut out = Vec::new();
    if let Some(browser) = std::env::var_os("BROWSER") {
        let browser = browser.to_string_lossy().into_owned();
        let argv: Vec<String> = browser
            .split(':')
            .next()
            .unwrap_or("")
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        if !argv.is_empty() {
            out.push(argv);
        }
    }
    if std::env::var_os("DISPLAY").is_some() || std::env::var_os("WAYLAND_DISPLAY").is_some() {
        out.push(vec!["xdg-open".to_owned()]);
    }
    if std::env::var_os("WSL_DISTRO_NAME").is_some() {
        out.push(vec!["wslview".to_owned()]);
    }
    out
}
