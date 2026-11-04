use std::fs::{self, OpenOptions};
use std::io::Write;

pub type Row = (&'static str, usize, usize, &'static str);

pub fn enabled() -> bool {
    std::env::var_os("SB1_TA_MEMORY").is_some()
}
pub fn bytes<T>(count: usize) -> usize {
    count
        .checked_mul(std::mem::size_of::<T>())
        .expect("memory counter")
}
pub fn record(stage: &str, rows: &[Row]) {
    let Some(path) = std::env::var_os("SB1_TA_MEMORY") else {
        return;
    };
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("memory trace");
    if file.metadata().expect("memory trace metadata").len() == 0 {
        writeln!(file, "stage\tstructure\tentries\tbytes\tcoverage").expect("memory trace header");
    }
    for (name, entries, bytes, coverage) in rows {
        writeln!(file, "{stage}\t{name}\t{entries}\t{bytes}\t{coverage}")
            .expect("memory trace row");
    }
    let status = fs::read_to_string("/proc/self/status").expect("memory status");
    for name in ["VmRSS:", "VmHWM:"] {
        let kib = status
            .lines()
            .find_map(|line| line.strip_prefix(name))
            .and_then(|v| v.split_whitespace().next())
            .and_then(|v| v.parse::<usize>().ok())
            .expect("memory status value");
        writeln!(file, "{stage}\t{name}\t1\t{}\tlinux-proc", kib * 1024).expect("memory trace RSS");
    }
}
