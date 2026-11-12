// Property harness (contract tf; law = rust.md): every suite runs its properties through `check`
// under one committed seed + a count-bounded shrink, with no persistence files. `config` spells
// every proptest Config field, because `..Config::default()` takes the rest from PROPTEST_*
// variables and the proptest macro re-applies them over any config it is given
// (contextualize_config). Default collection sizes (SizeRange::default) read
// PROPTEST_MAX_DEFAULT_SIZE_RANGE outside any config, so `check` refuses to run while it is set.
use proptest::strategy::Strategy;
use proptest::test_runner::{
    Config, RngAlgorithm, RngSeed, TestCaseResult, TestRunner, noop_result_cache,
};

pub const SEED: u64 = 20_261_112;
pub const MAX_SHRINK_ITERS: u32 = 10_000;

pub fn config(cases: u32) -> Config {
    Config {
        cases,
        max_local_rejects: 65_536,
        max_global_rejects: 1024,
        max_flat_map_regens: 1_000_000,
        failure_persistence: None,
        source_file: None,
        test_name: None,
        max_shrink_time: 0,
        max_shrink_iters: MAX_SHRINK_ITERS,
        max_default_size_range: 100,
        result_cache: noop_result_cache,
        verbose: 0,
        rng_algorithm: RngAlgorithm::ChaCha,
        rng_seed: RngSeed::Fixed(SEED),
        _non_exhaustive: (),
    }
}

/// Runs `test` on `cases` inputs drawn from `strategy`; a failure panics at the caller with
/// proptest's report (the shrunk minimal failing input) + the seed.
#[track_caller]
pub fn check<S: Strategy>(cases: u32, strategy: S, test: impl Fn(S::Value) -> TestCaseResult) {
    if let Some(v) = std::env::var_os("PROPTEST_MAX_DEFAULT_SIZE_RANGE") {
        panic!("PROPTEST_MAX_DEFAULT_SIZE_RANGE={v:?} changes default collection sizes: unset it");
    }
    let mut runner = TestRunner::new(config(cases));
    if let Err(e) = runner.run(&strategy, test) {
        panic!("{e}\nseed {SEED}, {cases} cases\n{runner}");
    }
}
