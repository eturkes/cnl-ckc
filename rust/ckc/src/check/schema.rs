use super::common::*;
use std::path::Path;
// nt1 D10: `guidelines/schema.pl` = the kernel's schema bytes; `ckc compile` writes it.
pub(crate) const FILE: &str = "schema.pl";
pub(crate) fn bytes() -> Vec<u8> {
    ckc_kernel::contract::schema_file()
}
// A planted symlink or directory at the path is refused, never written through.
pub(super) fn write() -> Result {
    let path = Path::new("guidelines").join(FILE);
    if let Ok(meta) = std::fs::symlink_metadata(&path)
        && !meta.file_type().is_file()
    {
        return Err(fail(
            "schema-file",
            format!("not a regular file: {}", show(&path)),
        ));
    }
    std::fs::write(&path, bytes()).map_err(|e| fail("schema-file", e.to_string()))
}
pub(super) fn check() -> Result {
    let path = Path::new("guidelines").join(FILE);
    let meta = std::fs::symlink_metadata(&path)
        .map_err(|_| violation("schema-file", "missing guidelines/schema.pl"))?;
    if !meta.file_type().is_file() {
        return Err(violation("schema-file", "nonregular guidelines/schema.pl"));
    }
    let have = std::fs::read(&path).map_err(|e| fail("schema-file", e.to_string()))?;
    if have != bytes() {
        return Err(violation(
            "schema-file",
            "stale guidelines/schema.pl (run ckc compile)",
        ));
    }
    Ok(())
}
