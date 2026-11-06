// Contract sb P8: the producer's exact peak RSS. A short-lived child can start + exit between two of
// the harness's 50 ms RSS samples, so the orchestrator reaps it with wait4 and reports `ru_maxrss`
// (KiB on Linux) = the high-water mark of its whole life.
use std::io;
use std::os::unix::process::ExitStatusExt;
use std::process::{Child, ExitStatus};

// 64-bit Linux `struct rusage`: two `timeval`s (2 × i64 each), then 14 `long`s, ru_maxrss first.
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

pub struct Exit {
    pub status: ExitStatus,
    pub pid: u32,
    pub peak_bytes: u64,
}

// Reaps `child` (std never waits on it again) → its status, pid + exact peak RSS.
pub fn wait(child: Child) -> io::Result<Exit> {
    let pid = child.id();
    let raw = i32::try_from(pid).map_err(|_| io::Error::other("pid width"))?;
    let mut status = 0;
    let mut usage = Rusage {
        utime: [0; 2],
        stime: [0; 2],
        maxrss: 0,
        rest: [0; 13],
    };
    loop {
        // SAFETY: `raw` names this process's own unreaped child; both pointers outlive the call.
        let rc = unsafe { wait4(raw, &mut status, 0, &mut usage) };
        if rc == raw {
            break;
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
    drop(child);
    Ok(Exit {
        status: ExitStatus::from_raw(status),
        pid,
        peak_bytes: u64::try_from(usage.maxrss)
            .unwrap_or(0)
            .saturating_mul(1024),
    })
}

#[cfg(test)]
mod tests {
    use super::wait;
    use std::process::{Command, Stdio};

    // The plant: dd's one 512 MiB read touches its whole buffer ⇒ peak ≥ 512 MiB; a trivial child
    // stays small; a failing child keeps its status. A misread layout (ru_maxrss at another
    // offset) reads 0 or a time value instead.
    #[test]
    fn wait_reports_the_exact_peak_and_the_status() {
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
