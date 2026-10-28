// Contract sgw P1–P4/P7; source-only models. CKC_SGW_TEST_BIN selects the CLI;
// kernel bindings grade the linked tree. Historical bytes = 258ddc44, never re-pinned.
#[path = "../../../tests/sgw/census.rs"]
mod census;
#[path = "../../../tests/sgw/emission.rs"]
mod emission;
#[path = "../../../tests/sgw/model.rs"]
mod model;
#[path = "../../../tests/sgw/reader.rs"]
mod reader;
#[path = "../../../tests/sgw/support.rs"]
mod support;
