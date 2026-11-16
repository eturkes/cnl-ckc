// Self-only terminal term: a parent's wait4 includes the peaks of its reaped children.
use std::convert::Infallible;
use std::ffi::CString;
use std::fmt::{self, Write as _};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[repr(C)]
struct Rusage {
    utime: [i64; 2],
    stime: [i64; 2],
    maxrss: i64,
    rest: [i64; 13],
}
const _: () = assert!(std::mem::size_of::<Rusage>() == 144);
unsafe extern "C" {
    fn getrusage(who: i32, usage: *mut Rusage) -> i32;
    fn rename(old: *const std::ffi::c_char, new: *const std::ffi::c_char) -> i32;
    fn _exit(status: i32) -> !;
}
struct Buffer {
    bytes: [u8; 2048],
    len: usize,
}
impl fmt::Write for Buffer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.len.checked_add(text.len()).ok_or(fmt::Error)?;
        self.bytes
            .get_mut(self.len..end)
            .ok_or(fmt::Error)?
            .copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
    }
}
fn malformed() -> io::Error {
    io::Error::other("server terminal identity/HWM unavailable")
}
fn cpath(path: &Path) -> io::Result<CString> {
    CString::new(path.as_os_str().as_bytes()).map_err(io::Error::other)
}
fn publish(source: &CString, target: &CString) -> io::Result<()> {
    // SAFETY: both NUL-terminated path buffers outlive rename; it retains neither pointer.
    if unsafe { rename(source.as_ptr(), target.as_ptr()) } == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
/// EOF-only terminal publication. All allocation, file/path preparation + provider teardown precede
/// the self snapshot; fixed stack buffers + syscalls follow it, then _exit without Rust teardown.
pub fn server_terminal_exit(directory: &Path) -> io::Result<Infallible> {
    let pid = std::process::id();
    let stat = fs::read_to_string("/proc/self/stat")?;
    let ticks: u64 = stat
        .rsplit_once(')')
        .ok_or_else(malformed)?
        .1
        .split_whitespace()
        .nth(19)
        .ok_or_else(malformed)?
        .parse()
        .map_err(|_| malformed())?;
    let raw_path = directory.join("server-terminal.tsv");
    let check_path = directory.join("server-terminal-check.tsv");
    let raw_tmp = directory.join(format!(".server-terminal.{pid}.tmp"));
    let check_tmp = directory.join(format!(".server-terminal-check.{pid}.tmp"));
    let source_raw = cpath(&raw_tmp)?;
    let target_raw = cpath(&raw_path)?;
    let source_check = cpath(&check_tmp)?;
    let target_check = cpath(&check_path)?;
    let mut raw_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(raw_tmp)?;
    let mut check_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(check_tmp)?;
    let mut status = File::open("/proc/self/status")?;
    let mut own = [0u8; 16384];
    let mut raw = Buffer {
        bytes: [0; 2048],
        len: 0,
    };
    let mut check = Buffer {
        bytes: [0; 2048],
        len: 0,
    };
    let mut usage = Rusage {
        utime: [0; 2],
        stime: [0; 2],
        maxrss: 0,
        rest: [0; 13],
    };
    let count = status.read(&mut own)?;
    let text = std::str::from_utf8(&own[..count]).map_err(|_| malformed())?;
    let hwm = text
        .lines()
        .find_map(|s| s.strip_prefix("VmHWM:"))
        .and_then(|s| s.split_whitespace().next())
        .and_then(|s| s.parse::<u64>().ok())
        .and_then(|n| n.checked_mul(1024))
        .filter(|n| *n > 0)
        .ok_or_else(malformed)?;
    // SAFETY: Linux x86_64 RUSAGE_SELF=0; the checked-layout writable object outlives this call.
    if unsafe { getrusage(0, &mut usage) } != 0 {
        return Err(io::Error::last_os_error());
    }
    let rss = u64::try_from(usage.maxrss)
        .ok()
        .and_then(|n| n.checked_mul(1024))
        .filter(|n| *n > 0)
        .ok_or_else(malformed)?;
    let bound = rss.max(hwm);
    write!(raw, "format\tsb1-server-terminal 1\npid\t{pid}\nstart_ticks\t{ticks}\nresource_scope\tself\nru_maxrss_bytes\t{rss}\nvmhwm_bytes\t{hwm}\nexit_kind\tnormal\nexit_code\t0\n").map_err(|_| malformed())?;
    write!(check, "format\tsb1-server-terminal-check 1\npid\t{pid}\nstart_ticks\t{ticks}\nverdict\tbounded\nown_bound_bytes\t{bound}\n").map_err(|_| malformed())?;
    raw_file.write_all(&raw.bytes[..raw.len])?;
    check_file.write_all(&check.bytes[..check.len])?;
    raw_file.sync_all()?;
    check_file.sync_all()?;
    drop(raw_file);
    drop(check_file);
    publish(&source_raw, &target_raw)?;
    publish(&source_check, &target_check)?;
    // SAFETY: normal EOF, both complete self-only records published; no allocation/destructors follow.
    unsafe { _exit(0) }
}
