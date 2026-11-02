# SB1 certificate checker

Prototype for contract `sb` D5/D7. `accepted` means that this prototype passed its support and closure checks. It is not the verified patient-answer command or a WFM claim. Typed negative/growth-cycle analysis and relevant-slice selection are separate work. This checker examines the whole composition.

## Build and run

Run from the repository root with the pinned toolchain installed:

```bash
T="${CKC_TOOLCHAIN:-$PWD/.toolchain}"
export RUSTUP_HOME="$T/rustup" CARGO_HOME="$T/cargo"
export PATH="$T/verus-x86-linux:$T/cargo/bin:$T/bin:$PATH"
(cd prototype/sb1/checker && cargo build --release --locked --offline)
prototype/sb1/checker/target/release/checker --root "$PWD" \
  <composition.tsv> <model.txt> <certificate.txt> [<query>]
```

The standalone workspace uses vendored dependencies. Its toolchain link follows `rust/rust-toolchain.toml`. It takes no SWI dependency at run time. Linux `/proc/self/schedstat` supplies CPU time for its single thread; a missing clock is an I/O failure.

The files and return codes follow `bench/sb/interfaces.md`. Relative paths listed in the composition resolve against `--root`. The optional query is a canonical `ckc v1` question or the exact `sb1-q-all 0` marker. Answers are emitted only after every check passes. A complete empty q-all result has no stdout bytes and returns 0.

## Representation and checks

- Interned symbols and terms form a DAG. Exact term ids decide equality. Terms retain arbitrary-width canonical integers and rationals.
- Predicate indices and the bound-argument masks needed by the program and query are built once. A fully ground call uses exact model membership. Nested term patterns still receive a structural match.
- Claims cover the model exactly once. The checker applies each dense `vN` valuation to its original clause, checks the head and built-ins, checks existential correlated NAF, and requires every positive prerequisite to have a strictly lower level. Levels need not be minimal and have no machine-integer width limit.
- Closure enumerates each clause's enabled valuations over the proposed model. A missing head refuses even when no certificate row mentions that valuation.
- Question projection and q-all payloads use Prolog standard term order, then deduplicate and serialize canonically.

The default source codec parses each raw document once into the term arena. The parser records each anonymous variable's position and first-occurrence id. Fresh admission checks the canonical image against the raw source: declarations, metadata, bundle markers, literals, NAF, term spelling and variable reading. It then runs the unchanged term-shape and left-to-right safety checks. Canonical-image checking stays in the charged admission phase; no verdict is cached across patients.

`SB1_SOURCE_CODEC=kernel` selects the previous source path: `contract::v1_read`, the prototype decoder, then `contract::v1_check` during admission. It remains a differential oracle. The public kernel API returns a verdict or normalized bytes, not its parsed arena. The single-arena codec is prototype code, not a verified replacement for that kernel API. Its tests establish equivalence on their explicit inputs; full-domain equivalence and a verified parsed-program boundary remain proof work. Questions still use the kernel codec.

Both modes retain the consumer-fixture header adapter. A noncanonical leading comment block is accepted when replacing it with the document record's canonical header makes the document canonical. Directives, sentence markers and clauses retain their exact spelling. The identity always hashes the original bytes. The prototype also has its own exact rational relations and SHA-256; tests compare them with the committed schema and an independent hash implementation.

## Measurements

`checker-phases.tsv` records wall and CPU seconds for load, decode, admission, index, support, closure and answers. Load includes source I/O, reading, parsing and source-term interning. Decode includes model/certificate/query I/O, identity hashing and model-term interning. Admission includes the codec, term grammar and left-to-right safety. Answers includes sorting, serialization and the term-counter pass. Metric-file writes and the final stdout write sit outside these phase rows; the harness must charge those gaps.

`checker-source-profile.tsv` splits source normalization, arena decoding, canonical checking and shape/safety. Normalization and decoding belong to load; canonical checking and shape/safety belong to admission. `checker source-profile --root <repo> <composition.tsv>` runs those source steps without a model or certificate and prints the same split. This diagnostic does not change the seven main phase rows.

`checker-counters.tsv` separates logical input/output cardinalities from runtime work. `enabled_instances` counts every complete closure valuation; `support_incidences` counts the positive prerequisites of selected supports. Successful intermediate joins are runtime work, not a model cardinality. Term depth excludes certificate/output list wrappers. The last composition member supplies the `patient_facts` count, including its metadata facts. Zero-valued counters remain present. On a refused run, later phases have zero rows because they did not execute.

The prototype has no in-process deadline or memory limiter. The harness owns resource limits and abnormal-exit classification. These counters and phase rows do not constitute a scale or latency result.

## Checks

```bash
(cd prototype/sb1/checker && cargo test --locked --offline)
(cd prototype/sb1/checker && cargo clippy --locked --offline --all-targets --no-deps -- -D warnings)
```

Tests require the pinned SWI 10.0.2 for 9,730 built-in relation probes and `sha256sum` for the independent CLI identity control. Positive certificates, every named refusal plant, nested identities, full closure, arbitrary-width levels, canonical answers and all three return codes are covered. Each certificate fixture compares both source codecs. The codec suite also compares all 492 committed CDC documents, 1,560 structural mutations and explicit term, variable, NAF and header cases with the kernel path.

Format only this workspace's files. Cargo's `fmt --all` follows the kernel's workspace and touches files outside the prototype:

```bash
(cd prototype/sb1/checker && rustfmt --edition 2024 --config skip_children=true src/*.rs tests/*.rs)
```

Producer integration, with an existing producer and checker binary:

```bash
python3 -I -W error prototype/sb1/checker/tests/consumer.py \
  --root "$PWD" --producer prototype/sb1/producer.prolog \
  --checker prototype/sb1/checker/target/release/checker \
  --out prototype/sb1/checker/target/consumer
```

The output directory must not exist. This control accepts all four full CDC patient compositions, then compares q-all and a committed q-one question byte for byte with independent queries over each model. It keeps its inputs, outputs and hashes. It does not run the graded campaign.

`tests/admission.py` provides three development steps, run in order: `prepare`, `compare`, `profile`. Each takes `--root`, `--ckc`, `--checker`, `--producer` and `--out`. The output must sit under this workspace's `target/`. Preparation copies a minimal input tree there and generates all nine families at G1/G2 plus F1b at G4/G10. Comparison requires equal answers and work counters from both codecs on each G1/G2 composition. Profiling uses three randomized paired blocks at G2/G4/G10, seed 5102, and retains the plan, executable hashes and raw samples. It fits the medians to an affine curve; the G100 value is an extrapolation, not an endpoint result or a budget claim. Run it with the exclusive CPU reservation required by the experiment. Actual typed analysis and relevance remain outside these admission measurements.

`tests/admission-full.py` confirms admission in the whole checker. It takes `--inputs <prepared run>`, `--checker`, `--out` and one of `--producer` or `--reference`. The producer option regenerates model/certificate inputs. The reference option imports the frozen `F1b-G-baseline` layout only after checking its complete ordered composition identity, then updates the copied certificate's path-only identity. Both modes must match the same q-all bytes on all three paired blocks (seed 5103). The script retains every phase, counter, source-profile and answer file. Its post-load sum still excludes producer work, typed analysis/relevance and final output/metric gaps.
