// Direct probes of compiler guards that real APE output never reaches: a guard reads the
// DRS through arg/3 + == (compiler.md), so a malformed term is refused and left unbound.
// CKC_COMPILER_GUARDS_COMPILER selects a compiler copy for red replay.
use std::path::PathBuf;
use std::process::Command;

#[test]
fn recurrence_frame_guard_refuses_without_binding() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let compiler = std::env::var_os("CKC_COMPILER_GUARDS_COMPILER")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("vendor/ape/prolog/ace_to_pl.pl"));
    let out = Command::new("timeout")
        .args([
            "--kill-after=2",
            "60",
            "swipl",
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
        ])
        .arg(root.join("tests/compiler-guards/frame-nonbinding.pl.in"))
        .args(["-g", "probe_main", "-t", "halt(9)", "--"])
        .arg(&compiler)
        .env("LC_ALL", "C.UTF-8")
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
