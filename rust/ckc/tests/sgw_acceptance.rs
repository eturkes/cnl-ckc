// Contract sgw P1–P4/P7; source-only models. CKC_SGW_TEST_BIN selects the CLI;
// kernel bindings grade the linked tree. History = fixed commits (258ddc44 letter spelling,
// 4a257599 the first `_`-spelled corpus); the live tree = the spelling + reading laws + pins.
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
