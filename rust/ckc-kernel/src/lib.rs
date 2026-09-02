// Verified kernel: `contract` = trusted manifest-pinned binding surface;
// every other module = uninspected impl+proofs whose sole review is
// `cargo verus verify`.
pub mod contract;
mod align_impl;
mod v1_impl;
mod v1_term_impl;

pub use ckc_spec::align::{ECheck, EModel, ESpan};
pub use ckc_spec::v1text::EV1Verdict;
