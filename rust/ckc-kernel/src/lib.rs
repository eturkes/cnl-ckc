// Lint posture (gate = clippy -D warnings): ghost erasure hides spec/proof
// items from rustc, so the unused/dead families stay off; the clippy families
// below propose std APIs without a vstd spec (`is_empty`, `vec!`,
// `RangeInclusive::contains`, slices in place of `&Vec` views, `?`) or count
// ghost parameters against arity/type-size limits.
#![allow(unused_variables, dead_code, unused_imports)]
#![allow(
    clippy::ptr_arg,
    clippy::len_zero,
    clippy::vec_init_then_push,
    clippy::manual_range_contains
)]
#![allow(
    clippy::question_mark,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
// Rewrites clippy proposes here (closures, `matches!`, `if let`,
// `unwrap_or_default`, collapsed branches) change the proof obligations
// Verus sees for no exec gain.
#![allow(
    clippy::single_match,
    clippy::needless_match,
    clippy::match_like_matches_macro,
    clippy::manual_map,
    clippy::manual_filter,
    clippy::manual_unwrap_or_default,
    clippy::manual_unwrap_or,
    clippy::unnecessary_unwrap,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::needless_return,
    clippy::let_and_return,
    non_shorthand_field_patterns
)]
// Verified kernel: `contract` = trusted manifest-pinned binding surface;
// every other module = uninspected impl+proofs whose sole review is
// `cargo verus verify`.
mod align_impl;
pub mod contract;
mod k2_aggregate;
mod k2_answers;
mod k2_bridge;
mod k2_engine;
mod k2_impl;
mod k2_load;
mod k2_manifest;
mod k2_output;
mod k2_payload;
mod k2_recursion;
mod k2_reject;
mod k2_rewrite;
mod k2_sort;
mod k2_store;
mod k2_term;
mod k2_walk;
mod k3_adapter;
mod k3_coords;
mod k3_front;
mod k3_identity;
mod k3_impl;
mod k3_join;
mod k3_machine;
mod k3_materialize;
mod k3_number;
mod k3_print;
mod k3_rows;
pub mod k3_sound;
mod k3_state;
mod k5_bytes;
mod k5_chrome;
mod k5_codes;
mod k5_copy;
mod k5_copy_match;
mod k5_html;
mod k5_impl;
mod k5_ledger;
mod k5_render;
mod k5_sound;
mod k5_sound_build;
mod k5_sound_chrome;
mod k5_sound_codes;
mod k5_sound_context;
mod k5_sound_copy;
mod k5_sound_corpus;
mod k5_sound_encoding;
mod k5_sound_escape;
mod k5_sound_guideline;
mod k5_sound_literals;
mod k5_sound_model;
mod k5_sound_pages;
mod k5_sound_post;
mod k5_sound_record_source;
mod k5_sound_records;
mod k5_sound_reflect;
mod k5_sound_source;
mod k5_sound_summary;
mod k5_utf8;
mod v1_impl;
mod v1_term_impl;

pub use ckc_spec::align::{ECheck, EModel, ESpan};
pub use ckc_spec::replay::{EOut, ERow, ESrc};
pub use ckc_spec::v1text::EV1Verdict;
