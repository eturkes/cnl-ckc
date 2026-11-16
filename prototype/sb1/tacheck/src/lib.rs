//! TAS B1 checker. `producer` is untrusted; `core` imports parser/IR alone.
#![forbid(unsafe_code)]
pub mod core;
pub mod diagnostics;
pub mod producer;
pub mod runtime;

#[cfg(test)]
mod arbitrary_tests;
#[cfg(test)]
mod architecture_tests;
#[cfg(test)]
mod extension_tests;
#[cfg(test)]
mod faults;
#[cfg(test)]
#[path = "../tests/finite/mod.rs"]
mod finite_oracle;
#[cfg(test)]
mod patient_tests;
#[cfg(test)]
mod plant_tests;
#[cfg(test)]
mod qualification_tests;
#[cfg(test)]
mod root_tests;
#[cfg(test)]
mod runtime_tests;
