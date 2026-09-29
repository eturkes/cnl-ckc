// Ledger-lock regression (.agent/contracts/m5u5-cas.md P1). The test plays the
// initial holder and a newcomer; the ckc POST is the waiter queued on the inode
// that the holder's release orphans. Schedule S1–S8 of the contract.
// Linux only: waiter state = /proc/locks. CKC_UI_TEST_BIN selects a prebuilt ckc.
#![cfg(target_os = "linux")]

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

const CASE: &str = "tests/ui/red/verdict-ok-append";
const NEWCOMER_ROW: &[u8] = b"a-10\t788c59c39469ffa05dcea391455133cdd38d6257ba389c3045f07ab2194986e1\t\tapproved\tnewcomer\t2026-08-19T12:00:00Z\tnewcomer row\n";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

// The case's POST argv (case.tsv row 1) with @TREE@ bound to the private copy.
fn post_args(case: &Path, tree: &Path) -> Vec<String> {
    let table = fs::read_to_string(case.join("case.tsv")).unwrap();
    let row = table.lines().find(|l| !l.starts_with('#')).unwrap();
    let argv = row.split('\t').next().unwrap();
    argv.split(' ')
        .map(|a| {
            if a == "@TREE@" {
                tree.to_str().unwrap().to_owned()
            } else {
                a.to_owned()
            }
        })
        .collect()
}

// Blocked flock waiters: "<id>: -> FLOCK ADVISORY WRITE <pid> <maj>:<min>:<ino> 0 EOF".
fn waits_on(pid: u32, ino: u64) -> bool {
    fs::read_to_string("/proc/locks").unwrap().lines().any(|l| {
        let f: Vec<&str> = l.split_whitespace().collect();
        f.len() > 6
            && f[1] == "->"
            && f[2] == "FLOCK"
            && f[5] == pid.to_string()
            && f[6].rsplit(':').next() == Some(ino.to_string().as_str())
    })
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Writer(Option<Child>);
impl Writer {
    fn pid(&self) -> u32 {
        self.0.as_ref().unwrap().id()
    }
    // Some(output) once the child has exited.
    fn exited(&mut self) -> Option<Output> {
        let child = self.0.as_mut().unwrap();
        child.try_wait().unwrap()?;
        Some(self.0.take().unwrap().wait_with_output().unwrap())
    }
}
impl Drop for Writer {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn report(output: &Output) -> String {
    format!(
        "status {:?}, stdout {:?}, stderr {:?}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout[..output.stdout.len().min(120)]),
        String::from_utf8_lossy(&output.stderr)
    )
}

// Polls `ready` every 5 ms until the deadline; the writer exiting first fails the step.
fn wait_for(step: &str, writer: &mut Writer, ready: impl Fn(u32) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        if let Some(output) = writer.exited() {
            panic!(
                "{step}: writer exited while the lock was held: {}",
                report(&output)
            );
        }
        if ready(writer.pid()) {
            return;
        }
        assert!(Instant::now() < deadline, "{step}: 60 s deadline passed");
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn locked(path: &Path, create_new: bool) -> File {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .create_new(create_new)
        .truncate(false)
        .open(path)
        .unwrap();
    file.try_lock().unwrap();
    file
}

#[test]
fn queued_post_on_an_orphaned_lock_inode_waits_for_the_newcomer() {
    let case = root().join(CASE);
    let scratch = Scratch(root().join(format!("rust/target/ui-lock/{}", std::process::id())));
    let _ = fs::remove_dir_all(&scratch.0);
    let tree = scratch.0.join("tree");
    copy_tree(&case.join("tree"), &tree);
    let audit = tree.join("guidelines/alpha/audit");
    let ledger = audit.join("adjudication.tsv");
    let lock_path = audit.join(".adjudication.lock");
    let before = fs::read(&ledger).unwrap();
    let fixture_files = names(&audit);

    // S1: the initial holder locks the lock path (ino1).
    let first = locked(&lock_path, false);
    let ino1 = first.metadata().unwrap().ino();
    // S2: writer A queues on ino1.
    let program = std::env::var_os("CKC_UI_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    let mut writer = Writer(Some(
        Command::new(program)
            .arg("ui")
            .args(post_args(&case, &tree))
            .current_dir(&scratch.0)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .env("TMPDIR", &scratch.0)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap(),
    ));
    wait_for("S2 A queued on ino1", &mut writer, |pid| {
        waits_on(pid, ino1)
    });
    // S3: the holder unlinks the path; S4: a newcomer locks a fresh file there (ino2).
    fs::remove_file(&lock_path).unwrap();
    let newcomer = locked(&lock_path, true);
    let ino2 = newcomer.metadata().unwrap().ino();
    assert_ne!(ino1, ino2, "S4: fresh inode");
    // S5: the holder releases ino1; S6: A must queue on ino2, never enter CAS.
    first.unlock().unwrap();
    drop(first);
    wait_for("S6 A queued on ino2", &mut writer, |pid| {
        waits_on(pid, ino2)
    });
    // S7: the newcomer commits its row, then releases in unlink-before-unlock order.
    OpenOptions::new()
        .append(true)
        .open(&ledger)
        .unwrap()
        .write_all(NEWCOMER_ROW)
        .unwrap();
    fs::remove_file(&lock_path).unwrap();
    newcomer.unlock().unwrap();
    drop(newcomer);
    // S8: A reacquires, sees the newcomer's ledger and refuses.
    let deadline = Instant::now() + Duration::from_secs(60);
    let output = loop {
        if let Some(output) = writer.exited() {
            break output;
        }
        assert!(Instant::now() < deadline, "S8 A exit: 60 s deadline passed");
        std::thread::sleep(Duration::from_millis(5));
    };
    assert_eq!(output.status.code(), Some(0), "S8: {}", report(&output));
    assert!(output.stderr.is_empty(), "S8: {}", report(&output));
    assert!(
        output.stdout.starts_with(b"HTTP 409\n"),
        "S8: {}",
        report(&output)
    );
    let mut expected = before;
    expected.extend_from_slice(NEWCOMER_ROW);
    assert_eq!(
        fs::read(&ledger).unwrap(),
        expected,
        "S8: ledger = fixture + newcomer row"
    );
    assert_eq!(
        names(&audit),
        fixture_files,
        "S8: audit dir = fixture files"
    );
}
