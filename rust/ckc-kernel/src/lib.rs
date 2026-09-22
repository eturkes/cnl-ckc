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
    clippy::collapsible_match,
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
mod k4_bundle;
mod k4_bytes;
mod k4_classify;
mod k4_coverage;
mod k4_coverage_rows;
mod k4_evidence;
mod k4_file_checks;
mod k4_impl;
mod k4_ledger;
mod k4_lexdata;
mod k4_lexicon;
mod k4_lexparse;
mod k4_manifest;
mod k4_numbers;
mod k4_payload;
mod k4_render;
mod k4_rows;
mod k4_scalar;
mod k4_search;
mod k4_semantic;
mod k4_set;
mod k4_words;
mod k5_bytes;
mod k5_chrome;
mod k5_codes;
mod k5_copy;
mod k5_copy_match;
mod k5_document;
mod k5_document_data;
mod k5_form;
mod k5_frame;
mod k5_guideline;
mod k5_highlight;
mod k5_html;
mod k5_impl;
mod k5_index;
mod k5_ledger;
mod k5_model;
mod k5_payload;
mod k5_post;
mod k5_records;
mod k5_records_page;
mod k5_render;
mod k5_response;
mod k5_route;
mod k5_sound;
mod k5_sound_corpus;
mod k5_sound_escape;
mod k5_sound_flow;
mod k5_sound_provenance;
mod k5_sound_source;
mod k5_text;
mod k5_titles;
mod k5_url;
mod k5_utf8;
mod m6_ante;
mod m6_bind;
mod m6_custody;
mod m6_doc;
mod m6_drs;
mod m6_dump;
mod m6_expand;
mod m6_flat;
mod m6_group;
mod m6_impl;
mod m6_model;
mod m6_normal;
mod m6_payload;
mod m6_project;
mod m6_query;
mod m6_query_drs;
mod m6_query_markers;
mod m6_refs;
mod m6_rules;
mod m6_safety;
mod m6_symbols;
mod m6_term;
mod m6_vars;
mod release_impl;
mod release_rows;
mod resolve_data;
mod resolve_impl;
mod resolve_text;
mod v1_impl;
mod v1_term_impl;

pub use ckc_spec::align::{ECheck, EModel, EResolve, EResolved, ESpan};
pub use ckc_spec::check::{
    EBundle, ECoverage, ECoverageRow, EDecision, EEvidence, EFileSrc, ERendered, EStatus, EVerdict,
};
pub use ckc_spec::release::EMember;
pub use ckc_spec::replay::{EOut, ERow, ESrc};
pub use ckc_spec::ui::{
    ECorpus, EDocument, EGuideline, EPage, EPiece, EPostDocument, EPostGuideline, EPostOutcome,
    EPostState, ERecord, ERequest, EResponse,
};
pub use ckc_spec::v1text::EV1Verdict;
