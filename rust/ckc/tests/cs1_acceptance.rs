// Contract cs1 P3–P5; expected clauses, proofs and offsets derive from the contract.
// CKC_CS1_TEST_BIN selects a prebuilt binary for seed-red replay.
#[path = "../../../tests/cs1/certify.rs"]
mod certify;
#[path = "../../../tests/cs1/codec.rs"]
mod codec;
#[path = "../../../tests/cs1/common.rs"]
mod common;
#[path = "../../../tests/cs1/composition.rs"]
mod composition;
#[path = "../../../tests/cs1/model.rs"]
mod model;
#[path = "../../../tests/bf/reorder.rs"]
mod reorder;
#[path = "../../../tests/cs1/scan.rs"]
mod scan;
