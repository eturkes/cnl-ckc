// Trusted surface. Certification = read this crate (+ the manifest-pinned
// kernel binding files) + run `cargo verus verify --workspace --locked
// --offline` + `ckc trust-audit`. Nothing else is human-read.
pub mod align;
