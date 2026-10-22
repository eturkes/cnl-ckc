// Contract nt1 P1/P2/P4/P5-certify; diff-blind to the implementation.
// CKC_NT1_TEST_BIN selects the pre-nt1 binary for red replay.
#[path = "../../../tests/nt1/certify.rs"]
mod certify;
#[path = "../../../tests/nt1/codec.rs"]
mod codec;
#[path = "../../../tests/nt1/common.rs"]
mod common;
#[path = "../../../tests/nt1/composition.rs"]
mod composition;
#[path = "../../../tests/nt1/oracle.rs"]
mod oracle;
#[path = "../../../tests/nt1/relation.rs"]
mod relation;
#[path = "../../../tests/bf/reorder.rs"]
mod reorder;
