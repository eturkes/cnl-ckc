use super::{Result, ns};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub(super) struct Process {
    pub pid: u32,
    pub parent: u32,
    pub group: u32,
    pub start: u64,
    pub state: char,
    pub own_ticks: u64,
    pub children_ticks: u64,
    pub rss: u64,
    pub hwm: u64,
    pub executable: Option<PathBuf>,
}
impl Process {
    pub fn key(&self) -> (u32, u64) {
        (self.pid, self.start)
    }
    pub fn live(&self) -> bool {
        self.state != 'Z' && self.state != 'X'
    }
}
pub(super) fn stat(pid: u32) -> Option<Process> {
    let bytes = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let fields: Vec<_> = bytes.rsplit_once(')')?.1.split_whitespace().collect();
    let number = |index: usize| fields.get(index)?.parse::<u64>().ok();
    let child_ticks = |index: usize| Some(fields.get(index)?.parse::<i64>().ok()?.max(0) as u64);
    Some(Process {
        pid,
        parent: number(1)?.try_into().ok()?,
        group: number(2)?.try_into().ok()?,
        start: number(19)?,
        state: fields.first()?.chars().next()?,
        own_ticks: number(11)?.saturating_add(number(12)?),
        children_ticks: child_ticks(13)?.saturating_add(child_ticks(14)?),
        rss: 0,
        hwm: 0,
        executable: None,
    })
}
fn status(process: &mut Process) -> Result<bool> {
    let text = match fs::read_to_string(format!("/proc/{}/status", process.pid)) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(format!("/proc/{}/status: {error}", process.pid)),
    };
    for line in text.lines() {
        let mut fields = line.split_whitespace();
        let name = fields.next().unwrap_or("");
        if name == "VmRSS:" || name == "VmHWM:" {
            let count = fields
                .next()
                .and_then(|s| s.parse::<u64>().ok())
                .ok_or("invalid /proc memory value")?;
            if fields.next() != Some("kB") {
                return Err("unexpected /proc memory unit".to_owned());
            }
            let bytes = count.checked_mul(1024).ok_or("/proc memory overflow")?;
            if name == "VmRSS:" {
                process.rss = bytes;
            } else {
                process.hwm = bytes;
            }
        }
    }
    process.executable = fs::read_link(format!("/proc/{}/exe", process.pid)).ok();
    // The second identity read prevents an exit + pid reuse from joining two lives.
    Ok(stat(process.pid).is_some_and(|last| last.start == process.start))
}
#[derive(Clone)]
pub(super) struct Seen {
    pub process: Process,
    pub first: u64,
    pub last: u64,
    pub samples: u64,
    pub peak_rss: u64,
    pub hwm: u64,
    pub own_ticks: u64,
}
pub(super) struct Sample {
    pub start: u64,
    pub end: u64,
    pub phase: String,
    pub rss: u64,
    pub processes: Vec<Process>,
}
pub(super) struct State {
    pub processes: BTreeMap<(u32, u64), Seen>,
    pub samples: Vec<Sample>,
    pub executables: BTreeSet<PathBuf>,
    pub error: Option<String>,
    pub peak_rss: u64,
    pub peak_single_hwm: u64,
    pub max_gap: u64,
    phase: String,
    groups: BTreeMap<u32, u64>,
    stop: bool,
}
impl State {
    fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            samples: Vec::new(),
            executables: BTreeSet::new(),
            error: None,
            peak_rss: 0,
            peak_single_hwm: 0,
            max_gap: 0,
            phase: "orchestration".to_owned(),
            groups: BTreeMap::new(),
            stop: false,
        }
    }
    fn snapshot(&mut self, origin: Instant) -> Result<Vec<Process>> {
        let start = ns(origin.elapsed());
        let mut all = BTreeMap::new();
        for entry in fs::read_dir("/proc").map_err(|e| format!("/proc: {e}"))? {
            let Ok(entry) = entry else { continue };
            let Some(pid) = entry
                .file_name()
                .to_str()
                .and_then(|s| s.parse::<u32>().ok())
            else {
                continue;
            };
            if let Some(process) = stat(pid) {
                all.insert(pid, process);
            }
        }
        let own = std::process::id();
        if !all.contains_key(&own) {
            return Err("the harness is absent from /proc".to_owned());
        }
        let mut owned = BTreeSet::from([own]);
        for process in all.values() {
            let group_owned = self.groups.get(&process.group).is_some_and(|start| {
                all.get(&process.group)
                    .is_some_and(|leader| leader.start == *start)
            });
            if group_owned || self.processes.contains_key(&process.key()) {
                owned.insert(process.pid);
            }
        }
        loop {
            let before = owned.len();
            for process in all.values() {
                if owned.contains(&process.parent) {
                    owned.insert(process.pid);
                }
            }
            if owned.len() == before {
                break;
            }
        }
        let mut processes = Vec::new();
        for pid in owned {
            let Some(mut process) = all.remove(&pid) else {
                continue;
            };
            if status(&mut process)? {
                processes.push(process);
            }
        }
        let end = ns(origin.elapsed());
        let rss = processes.iter().map(|p| p.rss).sum();
        self.peak_rss = self.peak_rss.max(rss);
        for process in &processes {
            if let Some(path) = &process.executable {
                self.executables.insert(path.clone());
            }
            if process.pid != own {
                self.peak_single_hwm = self.peak_single_hwm.max(process.hwm);
            }
            let seen = self.processes.entry(process.key()).or_insert_with(|| Seen {
                process: process.clone(),
                first: start,
                last: end,
                samples: 0,
                peak_rss: 0,
                hwm: 0,
                own_ticks: 0,
            });
            seen.process = process.clone();
            seen.last = end;
            seen.samples += 1;
            seen.peak_rss = seen.peak_rss.max(process.rss);
            seen.hwm = seen.hwm.max(process.hwm);
            seen.own_ticks = seen.own_ticks.max(process.own_ticks);
        }
        if let Some(previous) = self.samples.last() {
            self.max_gap = self.max_gap.max(start - previous.start);
        }
        self.samples.push(Sample {
            start,
            end,
            phase: self.phase.clone(),
            rss,
            processes: processes.clone(),
        });
        Ok(processes)
    }
}
type Shared = Arc<(Mutex<State>, Condvar)>;
pub(super) struct Sampler {
    shared: Shared,
    thread: Option<JoinHandle<()>>,
    origin: Instant,
}
impl Sampler {
    pub fn start(origin: Instant, period: Duration) -> Result<Self> {
        if !Path::new("/proc/self/stat").is_file() {
            return Err("bench run requires Linux /proc".to_owned());
        }
        let mut state = State::new();
        state.snapshot(origin)?;
        let shared = Arc::new((Mutex::new(state), Condvar::new()));
        let worker = shared.clone();
        let thread = thread::spawn(move || {
            let (lock, wake) = &*worker;
            let mut next = Instant::now();
            loop {
                let mut state = lock.lock().unwrap_or_else(|e| e.into_inner());
                if state.stop {
                    break;
                }
                let now = Instant::now();
                next = next.checked_add(period).unwrap_or(now);
                if next <= now {
                    next = now.checked_add(period).unwrap_or(now);
                }
                while !state.stop && Instant::now() < next {
                    let wait = next.saturating_duration_since(Instant::now());
                    let (new_state, _) = wake
                        .wait_timeout(state, wait)
                        .unwrap_or_else(|e| e.into_inner());
                    state = new_state;
                }
                if state.stop {
                    break;
                }
                if let Err(error) = state.snapshot(origin) {
                    state.error = Some(error);
                    break;
                }
            }
        });
        Ok(Self {
            shared,
            thread: Some(thread),
            origin,
        })
    }
    pub fn phase(&self, name: &str) {
        self.shared
            .0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .phase = name.to_owned();
    }
    pub fn register(&self, pid: u32) -> Result<()> {
        let process = stat(pid).ok_or("subprocess identity missing from /proc")?;
        let mut state = self.shared.0.lock().unwrap_or_else(|e| e.into_inner());
        state.groups.insert(pid, process.start);
        state.snapshot(self.origin)?;
        Ok(())
    }
    pub fn unregister(&self, pid: u32) {
        self.shared
            .0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .groups
            .remove(&pid);
    }
    pub fn boundary(&self) -> Result<Vec<Process>> {
        self.shared
            .0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .snapshot(self.origin)
    }
    pub fn violation(&self, memory_bytes: u64) -> Option<&'static str> {
        let state = self.shared.0.lock().unwrap_or_else(|e| e.into_inner());
        if state.error.is_some() {
            Some("sampler")
        } else if state.peak_rss > memory_bytes || state.peak_single_hwm > memory_bytes {
            Some("memory")
        } else {
            None
        }
    }
    pub fn finish(mut self) -> State {
        self.stop();
        let mut state = self.shared.0.lock().unwrap_or_else(|e| e.into_inner());
        if let Err(error) = state.snapshot(self.origin) {
            state.error = Some(error);
        }
        std::mem::replace(&mut *state, State::new())
    }
    fn stop(&mut self) {
        {
            let mut state = self.shared.0.lock().unwrap_or_else(|e| e.into_inner());
            state.stop = true;
            self.shared.1.notify_all();
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
impl Drop for Sampler {
    fn drop(&mut self) {
        self.stop();
    }
}
