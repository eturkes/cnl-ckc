// Ghost erasure hides the spec-only uses of these imports from rustc.
#![allow(unused_imports)]
// Trusted surface. Certification = read this crate (+ the manifest-pinned
// kernel binding files) + run `cargo verus verify --workspace --locked
// --offline` + `ckc trust-audit`. Nothing else is human-read.
pub mod align;
pub mod answers;
pub mod digest;
pub mod emit;
pub mod engine;
pub mod replay;
pub mod term;
pub mod trace;
pub mod v1text;
