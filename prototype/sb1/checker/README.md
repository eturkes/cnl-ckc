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

The public kernel API exposes canonical file checking and underscore-variable reading, but not its term arena, writer or built-in evaluator. This prototype uses `contract::v1_check` and `v1_read`, plus its own interned decoder, canonical renderer, exact rational relations and SHA-256. Tests compare its built-ins with `guidelines/schema.pl` and its framed identity with an independent implementation.

Consumer fixtures have a different leading comment block from compiled documents. For the codec check alone, the checker replaces that block with the canonical header derived from the document record. Every directive, sentence marker and clause remains. The identity always hashes the original bytes.

## Measurements

`checker-phases.tsv` records wall and CPU seconds for load, decode, admission, index, support, closure and answers. Load includes source I/O, reading, parsing and source-term interning. Decode includes model/certificate/query I/O, identity hashing and model-term interning. Admission includes the codec, term grammar and left-to-right safety. Answers includes sorting, serialization and the term-counter pass. Metric-file writes and the final stdout write sit outside these phase rows; the harness must charge those gaps.

`checker-counters.tsv` separates logical input/output cardinalities from runtime work. `enabled_instances` counts every complete closure valuation; `support_incidences` counts the positive prerequisites of selected supports. Successful intermediate joins are runtime work, not a model cardinality. Term depth excludes certificate/output list wrappers. The last composition member supplies the `patient_facts` count, including its metadata facts. Zero-valued counters remain present. On a refused run, later phases have zero rows because they did not execute.

The prototype has no in-process deadline or memory limiter. The harness owns resource limits and abnormal-exit classification. These counters and phase rows do not constitute a scale or latency result.

## Checks

```bash
(cd prototype/sb1/checker && cargo test --locked --offline)
(cd prototype/sb1/checker && cargo clippy --locked --offline --all-targets --no-deps -- -D warnings)
```

Tests require the pinned SWI 10.0.2 for 9,730 built-in relation probes and `sha256sum` for the independent CLI identity control. Positive certificates, every named refusal plant, nested identities, full closure, arbitrary-width levels, canonical answers and all three return codes are covered.

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
