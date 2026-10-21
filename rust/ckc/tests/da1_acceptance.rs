// DA1 P1–P11; contract-only oracle, seed b2c6494d.
// CKC_DA1_TEST_BIN / CKC_DA1_SCHEMA_FILE select the implementation under test.
#[path = "../../../tests/da1/codec.rs"]
mod codec;
#[path = "../../../tests/da1/common.rs"]
mod common;
#[path = "../../../tests/da1/engine.rs"]
mod engine;
#[path = "../../../tests/da1/oracle.rs"]
mod oracle;
#[path = "../../../tests/da1/rational.rs"]
mod rational;
#[path = "../../../tests/da1/relation.rs"]
mod relation;
#[path = "../../../tests/da1/trace_replay.rs"]
mod trace_replay;
