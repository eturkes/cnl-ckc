// SUP1 P1/P2/P5b/P6; contract-only models. CKC_SUP1_TEST_BIN selects CLI lanes;
// raw-byte vocabulary properties exercise the linked tree's public kernel binding.
#[path = "../../../tests/cs1-surface/model.rs"]
mod base;
#[path = "../../../tests/bf/reorder.rs"]
mod binding_first;
#[path = "../../../tests/sup1-surface/model.rs"]
mod model;
#[path = "../../../tests/sup1-surface/portability.rs"]
mod portability;
#[path = "../../../tests/sup1-surface/support.rs"]
mod support;
#[path = "../../../tests/sup1-surface/surface.rs"]
mod surface;
#[path = "../../../tests/sup1-surface/table.rs"]
mod table;
