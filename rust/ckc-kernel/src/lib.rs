// Verified kernel: `contract` = trusted manifest-pinned binding surface;
// every other module = uninspected impl+proofs whose sole review is
// `cargo verus verify`.
pub mod contract;
mod align_impl;
mod v1_impl;
mod v1_term_impl;
mod k2_manifest;
mod k2_reject;
mod k2_term;
mod k2_sort;
mod k2_engine;
mod k2_output;
mod k2_load;
mod k2_bridge;
mod k2_walk;
mod k2_recursion;
mod k2_payload;
mod k2_aggregate;
mod k2_answers;
mod k2_impl;

pub use ckc_spec::align::{ECheck, EModel, ESpan};
pub use ckc_spec::v1text::EV1Verdict;
pub use ckc_spec::replay::{ERow, ESrc, EOut};
