// Trusted surface. Certification = read this crate (+ the manifest-pinned
// kernel binding files) + run `cargo verus verify --workspace --locked
// --offline` + `ckc trust-audit`. Nothing else is human-read.
pub mod align;
pub mod digest;
pub mod engine;
pub mod term;
pub mod v1text;
