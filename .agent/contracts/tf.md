# TF — test form: the property harness (contract; the harness half of the deferred row "Test form")

Row: Tasks TF. Rulings: `CLAUDE.md` `Engineering` Tests/verification (contract predicates → property tests; the gate fixes the generator seed; a shrunk counterexample = the minimal repair signal; example tests pin named cases + every shrunk counterexample). Scope ruling (MAIN; the deferred row's own reason for its date — "its contract predicates take the property form" — covers the harness alone): this unit = the harness, closed before MC's `tester`; the coverage-guided fuzz targets for the listed readers (the `ckc ui serve` request + form parsers, the ledger + table TSV readers, the K1 codec readers, the `ckc bench` manifest readers), their corpus replay + campaigns stay the deferred row, narrowed to them, reusing this harness's entries. Tooling: researcher-16's survey (proptest R1 vs quickcheck, bolero, cargo-fuzz, afl.rs, arbtest, fuzzcheck, an in-repo shrinker, derive_fuzztest): `proptest` — `RngSeed::Fixed(seed)` → `ChaCha20Rng::seed_from_u64(seed)` (its `rng.rs`), integrated per-value shrinking; arbtest's `.seed(n)` replays one case unshrunk and its `.minimize()` is a timed random search.

## Tier

shell (test infrastructure: dev-dependency + one config module + its controls); a defect re-checks through P2 + P3 every run.

## Design

- D1 Dependency: `proptest` as a dev-dependency of the crates whose suites hold properties (`ckc`), default features off + the minimal feature set that keeps std strategies (no `fork`, `timeout`, `bit-set`, `regex-syntax` unless a suite needs a regex strategy); every crate of its resolved dev closure added to `rust/trust/deps-allowlist.tsv` with its license passing `just deny`.
- D2 One config module (`rust/ckc/tests/support/prop.rs`, `include!`d or a `mod` per suite): `config(cases)` = `Config { cases, rng_seed: RngSeed::Fixed(SEED), failure_persistence: None, fork: false, timeout: 0, max_shrink_time: 0, max_shrink_iters: <fixed>, ..Config::default() }` built explicitly, so `PROPTEST_*` environment variables cannot change a gate run; SEED + the iteration bound committed constants; the failure report = proptest's (the minimal failing input + the seed).
- D3 Regressions: no persistence files; each shrunk counterexample a suite ever reports → a named example test (`CLAUDE.md`: example tests pin every shrunk counterexample).
- D4 Law: `.claude/rules/rust.md` names the module, the seed + bound, the planted controls (P2 + P3) + their firing, so later suites (MC's tester first) import the module instead of building a Config.

## Predicates

- P1 Determinism: a property suite run twice yields byte-equal case logs (the first 64 generated inputs written by a probe test) + verdicts; the same with `PROPTEST_RNG_SEED`, `PROPTEST_CASES` + `PROPTEST_RNG_ALGORITHM` set to other values (environment ignored).
- P2 Shrink: a planted predicate defect (a test-only predicate wrong on every vector holding an element ≥ 7) → the property fails and its report holds the shrunk minimal counterexample (`[7]`), pinned by an assertion over the captured failure.
- P3 Bound: shrinking stops within the committed iteration bound (no time-based budget: `max_shrink_time` = 0).
- P4 Dependencies: `just deny` + the allowlist check (`ckc trust-audit`) green with the new dev closure; a non-test build's dependency graph unchanged (`cargo tree -e normal` before = after).
- P5 Gate: the harness's suite runs inside `just test`; its wall time in the step ledger.
