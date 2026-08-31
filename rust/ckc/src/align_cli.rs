// align-check: shell seam for the verified align validator (contract m5u1
// R3/R4). Boundary duties, mirroring the legacy loader byte-for-byte where
// classed: UTF-8 decode of the align file (reject `loader not-utf8`),
// control scan (reject `loader control U+XXXX`, first offender, >=4 upper
// hex digits; permitted controls = TAB/LF; banned = other C0, DEL,
// U+202A-202E, U+2066-2069), then hand code points to the kernel and print
// its verdict verbatim. src/ace text files are read verbatim (no control
// scan — parser-lane semantics). rc: 0 ok, 1 kernel/loader reject, 2 io.
use std::process::ExitCode;

fn control_class(text: &[char]) -> Option<u32> {
    for c in text {
        let cp = *c as u32;
        let bad = (cp < 32 && cp != 9 && cp != 10)
            || cp == 127
            || (cp > 8233 && cp < 8239)
            || (cp > 8293 && cp < 8298);
        if bad {
            return Some(cp);
        }
    }
    None
}

fn read_chars(path: &str) -> Result<Vec<char>, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {}", path, e))?;
    match String::from_utf8(bytes) {
        Ok(s) => Ok(s.chars().collect()),
        Err(_) => Err(format!("{}: not utf-8", path)),
    }
}

pub fn run(align_path: &str, src_path: &str, ace_path: &str) -> ExitCode {
    let align_bytes = match std::fs::read(align_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("ckc: align-check: {}: {}", align_path, e);
            return ExitCode::from(2);
        }
    };
    let align: Vec<char> = match String::from_utf8(align_bytes) {
        Ok(s) => s.chars().collect(),
        Err(_) => {
            println!("loader not-utf8");
            return ExitCode::from(1);
        }
    };
    if let Some(cp) = control_class(&align) {
        println!("loader control U+{:04X}", cp);
        return ExitCode::from(1);
    }
    let (src, ace) = match (read_chars(src_path), read_chars(ace_path)) {
        (Ok(s), Ok(a)) => (s, a),
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("ckc: align-check: {}", e);
            return ExitCode::from(2);
        }
    };
    match ckc_kernel::contract::align_check(&align, &src, &ace) {
        ckc_kernel::ECheck::Ok(m) => {
            let fmt = |v: &Vec<ckc_kernel::ESpan>| {
                v.iter()
                    .map(|s| format!("{}:{}:{}", s.start, s.end, s.index))
                    .collect::<Vec<_>>()
                    .join(",")
            };
            println!("ok count={} src={} ace={}", m.count, fmt(&m.src), fmt(&m.ace));
            ExitCode::SUCCESS
        }
        ckc_kernel::ECheck::Err(detail) => {
            println!("err {}", detail.iter().collect::<String>());
            ExitCode::from(1)
        }
    }
}
