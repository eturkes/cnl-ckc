# SB1 certificate checker

Prototype for contract `sb` D5/D7. `accepted` means that this prototype passed its support and closure checks. It is not the verified patient-answer command or a WFM claim. The standalone mode examines the whole composition. The prepared orchestrator supports conservative slices and a typed-analysis/provider hook. Built-in providers do not yet enforce typed negative/growth-cycle admission.

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

## Prepared calls

```sh
prototype/sb1/checker/target/release/checker orchestrate --provider static \
  --root "$PWD" <composition.tsv> <empty-output-dir> <query>
```

The last composition member is the patient's fact document. `PreparedProgram` parses and admits the preceding knowledge-base documents once. It retains their head indices, serialized clauses and incremental composition-identity prefix. Each call reads and admits the patient afresh, parses the question, selects clauses, writes the slice, owns the SWI child and checks the returned slice model. Model terms and patient state leave the arena after each call. SWI receives only the selected program. Its certificate keeps original clause ids.

`all` reproduces the whole-model path. `static` follows predicate and constant-constrained calls, including correlated NAF. Conjunctive head-pattern constraints narrow the calls conservatively; selecting a rule closes its whole body, not only a query-bound instance. q-all uses all clauses because its payloads range beyond the operator edges. These are prototype providers, not a verified relevance analysis. `SliceProvider::prepare` supplies the load-time analysis hook; `select` receives the patient's source clauses and question bytes and returns original clause ids alone. The typed provider owns its conservative-selection obligation. `--slice-ids` injects a selection for diagnostics; arbitrary selections carry no answer-preservation claim.

In harness mode, `CKC_BENCH_SINK=closed-file-v1` selects a regular `answers.txt` sink. The owner writes, flushes and closes it, then publishes an atomic `sink-complete.tsv` marker with the close time, bytes, digest and CPU ticks. The harness verifies that marker, maps its shared-clock time to its monotonic ledger with reported calibration uncertainty and rejects clock steps. Score ends at close; `teardown_ns` covers the remaining invocation. Direct CLI output stays on stdout.

The CLI retains the prepared knowledge base through process exit. The OS reclaims that process-lifetime heap; the CLI does not walk every dormant allocation after each call. Loaded RSS stays visible to the sampler until exit. Library callers own the prepared value and use its normal lifetime.

The orchestrator writes `orchestrator-phases.tsv` with explicit load/scored kinds and exclusive offsets. The producer's entire child interval is scored once; its own subphases remain diagnostic. Loaded RSS remains charged. The outer harness charges every gap through the complete answer sink. It reports later process teardown as a load-side lifecycle phase, keeps RSS sampling and the operational deadline/caps active through teardown, and reaps the owned process tree. Usage, metrics and outcome details are in `bench/sb/interfaces.md`.

### Gap trace

`SB1_STAGE_TRACE=1` enables diagnostic TSVs without changing load/scored kinds, deadlines or RSS sampling. `orchestrator-trace.tsv` records owner-relative monotonic wall time and absolute thread CPU nanoseconds: patient decode/admission, identity, query/slice, materialization, SWI spawn/wait, model/certificate decode, checks, answer computation, call-state rollback, metrics, stdout write/flush and pre-exit. `harness-stage-trace.tsv` records controller-relative events through owner exit, wait/reap, answer-sink handling, sampler finish and publication. Its child CPU ticks + the pinned `CLK_TCK` can be compared with the owner's pre-exit CPU, up to tick quantization. The two wall origins are separate; startup and post-exit deltas must be identified before comparing them. The stdout guard drop is not an explicit close of file descriptor 1.

`python3 -I prototype/sb1/checker/tests/stage-trace-controls.py` checks the analysis on distinct synthetic wall origins and exact absolute CPU remainders; identity-mismatch and incomplete-campaign plants must refuse. Trace writes stay charged. A trace-on run is diagnostic, not interchangeable with an untraced confirmation campaign. Default runs create neither trace file. `python3 -I prototype/sb1/checker/tests/analyze-stage-trace.py <paired-campaign>` validates six complete F1b runs and writes per-run/median stage deltas. `startup_plus_post_pre_exit` contains both the unknown startup offset and post-pre-exit wall; the owner's final CPU remainder uses separate absolute counters and retains its tick uncertainty.

### Development R rerun

Use F1b/F1a compositions at G1 and G100 made from the same root. Build release binaries first. Set `SB1_COMPOSITIONS` to their parent directory, `SB1_R_OUT` to a new output directory and `SB1_TIMING_LOCK` to the shared timing lock. This runs three paired development blocks, not the authorized pilot/confirmation campaign.

```sh
export ROOT="$PWD" CKC="$PWD/rust/target/release/ckc"
export SB1_COMPOSITIONS SB1_R_OUT SB1_TIMING_LOCK
export CHECKER="$PWD/prototype/sb1/checker/target/release/checker"
export PRODUCER="$PWD/prototype/sb1/producer.prolog"
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1 RAYON_NUM_THREADS=1 CKC_JOBS=1
mkdir "$SB1_R_OUT"
# shellcheck disable=SC2016 # Exported paths expand in the child shell.
flock "$SB1_TIMING_LOCK" heavy bash -c '
  exclusive() {
    python3 -I -c "import runpy; runpy.run_path(\"prototype/sb1/producer-tests/timings.py\")[\"exclusive\"]()"
  }
  exclusive || exit
  for family in F1b F1a; do
    exclusive || exit
    "$CKC" bench run "$SB1_COMPOSITIONS/$family-100/composition" \
      --paired "$SB1_COMPOSITIONS/$family-1/composition" --root "$ROOT" \
      --orchestrator "$CHECKER orchestrate --provider static --producer $PRODUCER" \
      --query "$ROOT/bench/sb/queries/q-one.pl.in" --blocks 3 --seed 12051 \
      --deadline-ms 3600000 --memory-bytes 32000000000 --sample-ms 50 \
      --identity-file "$CHECKER" --identity-file "$PRODUCER" --out "$SB1_R_OUT/$family" || exit
    exclusive || exit
  done
'
python3 -I prototype/sb1/checker/tests/summarize-r.py "$SB1_R_OUT"
```

The summary validates complete paired samples and writes `phase-summary.tsv` (each phase's median/max, plus scored gaps) and `cardinalities.tsv`. It preserves every raw sample. A family with any non-accepted run has no latency summary.

## Representation and checks

- Interned symbols and terms form a DAG. Exact term ids decide equality. Terms retain arbitrary-width canonical integers and rationals.
- Predicate indices and the bound-argument masks needed by the program and query are built once. A fully ground call uses exact model membership. Nested term patterns still receive a structural match.
- Envelope 1 claims reference 1-based model lines instead of repeating fact text. Range, duplicate and missing-reference checks establish exact coverage. Envelope 0 remains accepted for differential controls.
- The checker applies each dense `vN` valuation to its original clause, checks the head and built-ins, checks existential correlated NAF, and requires every positive prerequisite to have a strictly lower level. Levels need not be minimal and have no machine-integer width limit: ordinary levels use `u64`; overflow takes the normalized arbitrary-natural path.
- Model input rejects every CR byte before line normalization (parse rc 2); a non-LF certificate still refuses at envelope (rc 1).
- Model and valuation decoding checks canonical spelling in one pass. The parser checks whitespace, atom spelling, list sugar, integers and the required period form. Each interned symbol retains its canonical atom spelling. Full parse/render comparison remains the test oracle.
- Closure enumerates each clause's enabled valuations over the proposed model. A missing head refuses even when no certificate row mentions that valuation.
- Question projection and q-all payloads use Prolog standard term order, then deduplicate and serialize canonically.

The default source codec parses each raw document once into the term arena. The parser records each anonymous variable's position and first-occurrence id. Fresh admission streams the canonical image against the raw source without allocating a rendered string per subterm or a normalized source copy: declarations, metadata, bundle markers, literals, NAF, cached atom spelling and anonymous-variable reading. It then runs the unchanged term-shape and left-to-right safety checks. Standalone mode charges canonical-image checking to its admission phase. Prepared mode performs knowledge-base admission once at load and charges fresh patient admission to each call.

`SB1_SOURCE_CODEC=kernel` selects the previous source path: `contract::v1_read`, the prototype decoder, then `contract::v1_check` during admission. It remains a differential oracle. The public kernel API returns a verdict or normalized bytes, not its parsed arena. The single-arena codec is prototype code, not a verified replacement for that kernel API. Its tests establish equivalence on their explicit inputs; full-domain equivalence and a verified parsed-program boundary remain proof work. Questions still use the kernel codec.

Both modes retain the consumer-fixture header adapter. A noncanonical leading comment block is accepted when replacing it with the document record's canonical header makes the document canonical. Directives, sentence markers and clauses retain their exact spelling. The identity always hashes the original bytes. The prototype also has its own exact rational relations and SHA-256; tests compare them with the committed schema and an independent hash implementation.

## Measurements

Standalone `checker-phases.tsv` records wall and CPU seconds for load, decode, admission, index, support, closure and answers. Load includes source I/O, reading, parsing and source-term interning. Decode includes model/certificate/query I/O, identity hashing and model-term interning. Admission includes the codec, term grammar and left-to-right safety. Answers includes sorting, serialization and the term-counter pass. Metric-file writes and the final stdout write sit outside these phase rows; the harness must charge those gaps.

`checker-source-profile.tsv` splits source normalization, arena decoding, canonical checking and shape/safety. Normalization and decoding belong to load; canonical checking and shape/safety belong to admission. `checker source-profile --root <repo> <composition.tsv>` runs those source steps without a model or certificate and prints the same split. This diagnostic does not change the seven main phase rows.

`checker-counters.tsv` separates logical input/output cardinalities from runtime work. `enabled_instances` counts every complete closure valuation; `support_incidences` counts the positive prerequisites of selected supports. Successful intermediate joins are runtime work, not a model cardinality. Term depth excludes certificate/output list wrappers. In prepared mode, term-depth and number-width meters visit the selected clauses and slice model; `interned_terms` also counts the retained knowledge-base arena. Source-byte/clause counters describe admitted inputs, while `selected_clauses` names the checked slice. The last composition member supplies the `patient_facts` count, including its metadata facts. Zero-valued counters remain present. On a refused run, later phases have zero rows because they did not execute.

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

The finite-oracle preflight uses the frozen 93,369-case domain in `rust/ckc/tests/sb-oracle/`. Batches call the actual producer's evaluation, support-level agenda and envelope writer. The checker applies its source-shape/safety admission, model/envelope decoder, indices, support and closure checks. Source files are supplied as semantic clause terms: public canonical-file envelopes are outside this oracle's declared domain. The eight NC cases remain refusal controls, separate from the 93,361 admitted exact-model cases. Every case and disagreement is recorded; the command fails on any mismatch.

```sh
(cd prototype/sb1/checker && SB1_PREFLIGHT_OUT="$PWD/target/oracle-preflight" \
  cargo test --locked --offline --lib frozen_finite_oracle_matches_prototype_producer_and_checker -- --nocapture)
```

Run the CPU-heavy preflight on the pool. `SB1_PREFLIGHT_FAMILY` selects one family for failure diagnosis; only a run without it covers the whole domain. `tests/orchestrated-corpus.py` compares all/static answers for q-all, q-one and q-patient with the frozen whole-program producer plus independent model queries on the four consumer worlds and every G1/G2/G4 family. It retains exact-comparison rows and identities and removes bulky files only after successful comparison.

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
