// TAS-P2 P2-5: a pre-load small launcher owns wait4; its resident HWM is charged separately.
use std::collections::{BTreeMap, HashSet};
use std::ffi::OsString;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const SCRIPT: &str = include_str!("producer_launcher.py");
fn malformed() -> io::Error {
    io::Error::other("malformed/missing/stale launcher report")
}
fn number<T: std::str::FromStr>(text: &str) -> io::Result<T> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err(malformed());
    }
    text.parse().map_err(|_| malformed())
}
fn proc_fields(pid: u32) -> io::Result<(u32, u32, u64)> {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat"))?;
    let fields = stat
        .rsplit_once(')')
        .ok_or_else(malformed)?
        .1
        .split_whitespace()
        .collect::<Vec<_>>();
    Ok((
        number(fields.get(1).ok_or_else(malformed)?)?,
        number(fields.get(2).ok_or_else(malformed)?)?,
        number(fields.get(19).ok_or_else(malformed)?)?,
    ))
}
fn hwm(pid: u32) -> io::Result<u64> {
    fs::read_to_string(format!("/proc/{pid}/status"))?
        .lines()
        .find_map(|s| {
            s.strip_prefix("VmHWM:")
                .and_then(|s| s.split_whitespace().next())
        })
        .ok_or_else(malformed)
        .and_then(number::<u64>)?
        .checked_mul(1024)
        .ok_or_else(malformed)
}
unsafe extern "C" {
    fn kill(pid: i32, signal: i32) -> i32;
}

pub struct Launcher {
    child: Child,
    input: Option<ChildStdin>,
    output: BufReader<ChildStdout>,
    sequence: u64,
    poisoned: bool,
    active: Option<(u32, u64)>,
    seen: HashSet<(u32, u64)>,
    startup: String,
    terminal: Option<PathBuf>,
    start_ticks: u64,
}
impl Launcher {
    pub fn start_in(output: Option<&Path>) -> io::Result<Self> {
        Self::start_script_in(SCRIPT, output)
    }
    #[cfg(test)]
    pub(crate) fn start_script(script: &str) -> io::Result<Self> {
        Self::start_script_in(script, None)
    }
    pub(crate) fn start_script_in(script: &str, output: Option<&Path>) -> io::Result<Self> {
        let terminal = output.map(|directory| directory.join("launcher-terminal.tsv"));
        if let Some(path) = &terminal {
            for stale in [
                path.clone(),
                path.with_file_name("launcher-terminal-check.tsv"),
            ] {
                match fs::remove_file(&stale) {
                    Ok(()) => (),
                    Err(error) if error.kind() == io::ErrorKind::NotFound => (),
                    Err(error) => return Err(error),
                }
            }
        }
        let parent = std::process::id();
        let parent_hwm = hwm(parent)?;
        let started = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_nanos();
        let mut command = Command::new("python3");
        command.args(["-I", "-c", script]);
        if let Some(path) = &terminal {
            command.arg(path);
        }
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let input = child.stdin.take();
        let output = BufReader::new(child.stdout.take().ok_or_else(malformed)?);
        let mut launcher = Self {
            child,
            input,
            output,
            sequence: 0,
            poisoned: true,
            active: None,
            seen: HashSet::new(),
            startup: String::new(),
            terminal,
            start_ticks: 0,
        };
        let fields = launcher.report()?;
        let [ready, version, pid, group, ticks, peak, python_version] = fields.as_slice() else {
            return Err(malformed());
        };
        let pid: u32 = number(pid)?;
        let group: u32 = number(group)?;
        let ticks: u64 = number(ticks)?;
        let peak: u64 = number(peak)?;
        if ready != "ready"
            || version != "1"
            || pid != launcher.child.id()
            || ticks == 0
            || peak == 0
            || proc_fields(pid)? != (parent, group, ticks)
            || group != proc_fields(parent)?.1
            || python_version.split('.').count() != 3
            || !python_version.split('.').all(|s| number::<u32>(s).is_ok())
        {
            return Err(malformed());
        }
        launcher.start_ticks = ticks;
        let executable = fs::canonicalize(format!("/proc/{pid}/exe"))?;
        let digest = crate::digest::sha256(&fs::read(format!("/proc/{pid}/exe"))?);
        launcher.startup = format!(
            "format\tsb1-producer-launcher 1\npid\t{pid}\nparent_pid\t{parent}\nstart_ticks\t{ticks}\npgid\t{group}\nvmhwm_bytes\t{peak}\nparent_vmhwm_bytes\t{parent_hwm}\nstarted_unix_ns\t{started}\npython_path\t{}\npython_sha256\t{digest}\npython_version\t{python_version}\nscript_sha256\t{}\n",
            executable.display(),
            crate::digest::sha256(script.as_bytes())
        );
        launcher.poisoned = false;
        Ok(launcher)
    }
    pub fn record_startup(&self, directory: &Path) -> io::Result<()> {
        fs::write(directory.join("producer-launcher.tsv"), &self.startup)
    }
    fn report(&mut self) -> io::Result<Vec<String>> {
        let mut bytes = Vec::new();
        (&mut self.output)
            .take(4097)
            .read_until(b'\n', &mut bytes)?;
        if bytes.last() != Some(&b'\n') || bytes.len() > 4096 {
            return Err(malformed());
        }
        bytes.pop();
        Ok(String::from_utf8(bytes)
            .map_err(|_| malformed())?
            .split('\t')
            .map(str::to_owned)
            .collect())
    }
    pub fn run(&mut self, args: &[OsString], stdout: &Path, stderr: &Path) -> io::Result<Exit> {
        if self.poisoned {
            return Err(malformed());
        }
        self.poisoned = true;
        let result = self.run_inner(args, stdout, stderr);
        if result.is_err() {
            self.kill_active();
        } else {
            self.poisoned = false;
        }
        result
    }
    fn run_inner(&mut self, args: &[OsString], stdout: &Path, stderr: &Path) -> io::Result<Exit> {
        self.sequence = self.sequence.checked_add(1).ok_or_else(malformed)?;
        let sequence = self.sequence;
        // The directory identifies the arm/run; sequence keeps the framing unique per launcher.
        let mut frame = b"sb1-call\0".to_vec();
        frame.extend(sequence.to_le_bytes());
        frame.extend(
            stdout
                .parent()
                .ok_or_else(malformed)?
                .as_os_str()
                .as_bytes(),
        );
        let identity = crate::digest::sha256(&frame);
        let input = self.input.as_mut().ok_or_else(malformed)?;
        input.write_all(&sequence.to_le_bytes())?;
        input.write_all(
            &u32::try_from(args.len() + 3)
                .map_err(|_| malformed())?
                .to_le_bytes(),
        )?;
        for arg in [
            std::ffi::OsStr::new(&identity),
            stdout.as_os_str(),
            stderr.as_os_str(),
        ]
        .into_iter()
        .chain(args.iter().map(|s| s.as_os_str()))
        {
            let bytes = arg.as_bytes();
            input.write_all(
                &u32::try_from(bytes.len())
                    .map_err(|_| malformed())?
                    .to_le_bytes(),
            )?;
            input.write_all(bytes)?;
        }
        input.flush()?;
        let start = self.report()?;
        let [started, id, call, pid, ticks] = start.as_slice() else {
            return Err(malformed());
        };
        let pid: u32 = number(pid)?;
        let ticks: u64 = number(ticks)?;
        if started != "started"
            || number::<u64>(id)? != sequence
            || call != &identity
            || pid == 0
            || ticks == 0
            || self.seen.contains(&(pid, ticks))
        {
            return Err(malformed());
        }
        let (parent, group, actual_ticks) = proc_fields(pid)?;
        if parent != self.child.id() || group != proc_fields(std::process::id())?.1 {
            return Err(malformed());
        }
        self.active = Some((pid, actual_ticks));
        if actual_ticks != ticks {
            return Err(malformed());
        }
        // The launcher waits for this acknowledgement before reaping; zombies retain start_ticks.
        let observed = hwm(pid).unwrap_or(0);
        writeln!(
            self.input.as_mut().ok_or_else(malformed)?,
            "ack\t{sequence}\t{identity}\t{pid}\t{ticks}\t{observed}"
        )?;
        self.input.as_mut().ok_or_else(malformed)?.flush()?;
        let report = self.report()?;
        let [
            exited,
            id,
            call,
            child,
            start,
            kind,
            raw,
            wait4_peak,
            peak,
            cpu_us,
            seen_hwm,
        ] = report.as_slice()
        else {
            return Err(malformed());
        };
        let raw: i32 = number(raw)?;
        let wait4_peak: u64 = number(wait4_peak)?;
        let peak: u64 = number(peak)?;
        let cpu_us: u64 = number(cpu_us)?;
        let seen_hwm: u64 = number(seen_hwm)?;
        let status = ExitStatus::from_raw(raw);
        let expected_kind = if status.code().is_some() {
            "exit"
        } else if status.signal().is_some_and(|s| (1..=64).contains(&s)) {
            "signal"
        } else {
            return Err(malformed());
        };
        if exited != "exited"
            || number::<u64>(id)? != sequence
            || call != &identity
            || number::<u32>(child)? != pid
            || number::<u64>(start)? != ticks
            || !(0..=65535).contains(&raw)
            || kind != expected_kind
            || (kind == "exit" && raw & 255 != 0)
            || (kind == "signal" && raw > 255)
            || wait4_peak == 0
            || !wait4_peak.is_multiple_of(1024)
            || !peak.is_multiple_of(1024)
            || peak != wait4_peak.max(observed).max(seen_hwm)
        {
            return Err(malformed());
        }
        let end = self.report()?;
        if end
            != [
                "complete".to_owned(),
                sequence.to_string(),
                identity.clone(),
            ]
        {
            return Err(malformed());
        }
        self.active = None;
        self.seen.insert((pid, ticks));
        Ok(Exit {
            status,
            pid,
            peak_bytes: peak,
            wait4_peak_bytes: wait4_peak,
            start_ticks: ticks,
            sequence,
            cpu_us,
            identity,
            observed_hwm_bytes: observed.max(seen_hwm),
        })
    }
    fn kill_active(&mut self) {
        if let Some((pid, ticks)) = self.active.take()
            && proc_fields(pid).is_ok_and(|(_, _, actual)| actual == ticks)
            && let Ok(pid) = i32::try_from(pid)
        {
            // SAFETY: this (pid,start_ticks) was checked as the launcher's unreaped child.
            unsafe {
                kill(pid, 9);
            }
        }
    }
    pub fn record_exit(&self, directory: &Path, exit: &Exit) -> io::Result<()> {
        fs::write(
            directory.join("producer-report.tsv"),
            format!(
                "format\tsb1-producer-report 1\nlauncher_pid\t{}\nsequence\t{}\ncall_identity\t{}\nproducer_pid\t{}\nproducer_start_ticks\t{}\nproducer_exit_kind\t{}\nproducer_exit_status\t{}\nproducer_maxrss_bytes\t{}\nproducer_wait4_maxrss_bytes\t{}\nproducer_cpu_us\t{}\nproducer_observed_hwm_bytes\t{}\n",
                self.child.id(),
                exit.sequence,
                exit.identity,
                exit.pid,
                exit.start_ticks,
                if exit.status.code().is_some() {
                    "exit"
                } else {
                    "signal"
                },
                exit.status.into_raw(),
                exit.peak_bytes,
                exit.wait4_peak_bytes,
                exit.cpu_us,
                exit.observed_hwm_bytes
            ),
        )
    }
}
// wait4 on the launcher includes its reaped producers; only its self-report is an own term.
fn terminal_bound(path: &Path, pid: u32, ticks: u64, status: ExitStatus) -> io::Result<u64> {
    if fs::metadata(path)?.len() > 4096 {
        return Err(malformed());
    }
    let text = fs::read_to_string(path)?;
    let mut rows = BTreeMap::new();
    for line in text.lines() {
        let (key, value) = line.split_once('\t').ok_or_else(malformed)?;
        if rows.insert(key, value).is_some() {
            return Err(malformed());
        }
    }
    let get = |key| rows.get(key).copied().ok_or_else(malformed);
    if rows.len() != 8
        || get("format")? != "sb1-launcher-terminal 1"
        || get("resource_scope")? != "self"
        || number::<u32>(get("pid")?)? != pid
        || number::<u64>(get("start_ticks")?)? != ticks
    {
        return Err(malformed());
    }
    let raw: u64 = number(get("ru_maxrss_bytes")?)?;
    let own: u64 = number(get("vmhwm_bytes")?)?;
    let code: i32 = number(get("exit_code")?)?;
    let kind = match code {
        0 => "normal",
        2 => "error",
        _ => return Err(malformed()),
    };
    if raw == 0
        || own == 0
        || !raw.is_multiple_of(1024)
        || !own.is_multiple_of(1024)
        || get("exit_kind")? != kind
        || status.code() != Some(code)
    {
        return Err(malformed());
    }
    Ok(raw.max(own))
}
impl Drop for Launcher {
    fn drop(&mut self) {
        self.kill_active();
        let observed = hwm(self.child.id()).ok();
        drop(self.input.take());
        let status = self.child.wait();
        if let Some(path) = &self.terminal {
            let raw_status = status
                .as_ref()
                .map(|s| (*s).into_raw().to_string())
                .unwrap_or_else(|_| "NA".into());
            let bound =
                status.and_then(|s| terminal_bound(path, self.child.id(), self.start_ticks, s));
            let (verdict, bytes) = match bound {
                Ok(value) => ("bounded", value.to_string()),
                Err(_) => ("unbounded", "NA".into()),
            };
            let _ = fs::write(
                path.with_file_name("launcher-terminal-check.tsv"),
                format!(
                    "format\tsb1-launcher-terminal-check 1\npid\t{}\nstart_ticks\t{}\nverdict\t{verdict}\nown_bound_bytes\t{bytes}\nbefore_eof_observed_hwm_bytes\t{}\nexit_status_raw\t{raw_status}\n",
                    self.child.id(),
                    self.start_ticks,
                    observed.map_or_else(|| "NA".into(), |n| n.to_string())
                ),
            );
            let mut tmp = path.as_os_str().to_owned();
            tmp.push(format!(".{}.tmp", self.child.id()));
            let _ = fs::remove_file(PathBuf::from(tmp));
        }
    }
}
#[derive(Debug)]
pub struct Exit {
    pub status: ExitStatus,
    pub pid: u32,
    pub peak_bytes: u64,
    pub wait4_peak_bytes: u64,
    pub start_ticks: u64,
    pub sequence: u64,
    pub cpu_us: u64,
    pub identity: String,
    pub observed_hwm_bytes: u64,
}

// Direct wait4 → a child's exact life peak: run_census's census term (contract sb, the direct
// load-memory amendment) + the inherited-floor diagnostic of the original peak control. A zero or
// overflowing ru_maxrss = malformed: a 0 term would understate the load's memory bound.
#[repr(C)]
struct Rusage {
    utime: [i64; 2],
    stime: [i64; 2],
    maxrss: i64,
    rest: [i64; 13],
}
const _: () = assert!(std::mem::size_of::<Rusage>() == 144);
unsafe extern "C" {
    fn wait4(pid: i32, status: *mut i32, options: i32, rusage: *mut Rusage) -> i32;
}
pub(crate) fn wait(child: Child) -> io::Result<Exit> {
    let pid = child.id();
    let raw = i32::try_from(pid).map_err(|_| malformed())?;
    let mut status = 0;
    let mut usage = Rusage {
        utime: [0; 2],
        stime: [0; 2],
        maxrss: 0,
        rest: [0; 13],
    };
    loop {
        // SAFETY: raw is this process's unreaped child; both pointers outlive the call.
        if unsafe { wait4(raw, &mut status, 0, &mut usage) } == raw {
            break;
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
    drop(child);
    let peak = u64::try_from(usage.maxrss)
        .map_err(|_| malformed())?
        .checked_mul(1024)
        .filter(|n| *n > 0)
        .ok_or_else(malformed)?;
    Ok(Exit {
        status: ExitStatus::from_raw(status),
        pid,
        peak_bytes: peak,
        wait4_peak_bytes: peak,
        start_ticks: 0,
        sequence: 0,
        cpu_us: 0,
        identity: String::new(),
        observed_hwm_bytes: 0,
    })
}
// A spawned child's ru_maxrss starts at its parent's RSS high-water: exec folds the old mm's
// high-water into the child's maxrss (measured on the workstation's kernel 7.3: a `true` child of a
// parent that touched 300 MiB reads 329,342,976 B, also after the free). The suite's concurrent
// tests lift this process's high-water past the 64 MiB small-child line ⇒ a test asserting a small
// child of this process runs its body in a fresh copy of this binary holding that test alone: true
// inside that copy (run the body), false here once the copy reports that one test passed.
#[cfg(test)]
pub(crate) fn fresh_process(test: &str) -> bool {
    const KEY: &str = "SB1_FRESH_TEST";
    if std::env::var(KEY).is_ok_and(|name| name == test) {
        return true;
    }
    let out = Command::new(std::env::current_exe().unwrap())
        .args([test, "--exact", "--test-threads=1"])
        .env(KEY, test)
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    println!("{text}");
    assert!(
        out.status.success() && text.contains("test result: ok. 1 passed;"),
        "{test} in a fresh process: {}",
        out.status
    );
    false
}
#[cfg(test)]
mod tests {
    use super::{fresh_process, wait};
    use std::process::{Command, Stdio};
    #[test]
    fn wait_reports_the_exact_peak_and_the_status() {
        if !fresh_process("peak::tests::wait_reports_the_exact_peak_and_the_status") {
            return;
        }
        let big = Command::new("dd")
            .args(["if=/dev/zero", "of=/dev/null", "bs=512M", "count=1"])
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let exit = wait(big).unwrap();
        assert!(exit.status.success());
        assert!(exit.peak_bytes >= 512 << 20, "{}", exit.peak_bytes);
        assert!(exit.peak_bytes < 768 << 20, "{}", exit.peak_bytes);
        let exit = wait(Command::new("true").spawn().unwrap()).unwrap();
        assert!(exit.status.success() && exit.pid > 0);
        assert!(
            exit.peak_bytes > 0 && exit.peak_bytes < 64 << 20,
            "{}",
            exit.peak_bytes
        );
        let exit = wait(Command::new("false").spawn().unwrap()).unwrap();
        assert_eq!(exit.status.code(), Some(1));
    }
}
