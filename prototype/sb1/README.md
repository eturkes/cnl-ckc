# SB1 thin-path prototype

Untrusted producer + checker construction for contract `sb` D7. Prototype acceptance never means the verified patient-answer claim. The phase close retires this directory.

## Producer

Run SWI-Prolog 10.0.2 from the pinned toolchain. All paths below are repository-relative; pass the root explicitly. A composition lists its documents and patient world, one path per line. Its relative paths resolve against `--root`, not the working directory.

```sh
swipl -q -f none -F none -s prototype/sb1/producer.prolog \
  -g sb1_main -t 'halt(9)' -- \
  --root "$PWD" <composition.tsv> <empty-output-dir>
```

Outputs follow `bench/sb/interfaces.md`: canonical `model.txt`, `sb1-certificate 1` envelope (1-based model-line references in place of repeated fact text) + composition identity + one support claim per fact, and `producer-phases.tsv` (load, eval, certificate, write; wall seconds + CPU seconds). The write interval covers model + certificate serialization. Its own timing-table publication remains in the harness's outer interval. Existing output files fail at `write`; use a new directory for each run. Catchable failure: rc 1 + one `sb1-producer <stage> <detail>` stderr line; stages = load | eval | certificate | write | resource. A native signal can prevent that line. The harness must classify an abnormal producer exit as indeterminate, never accept partial files. Success: rc 0, empty stdout + stderr.

The recipe tables every schema predicate. The producer enumerates all of them, including metadata and schema-version facts. World and model arg/cardinality calls route through a hash of an identity's `(Kind, Doc, Sentence, Slot)` prefix; partial prefixes retain wildcard candidates. Original arguments decide every match, including nested Deps and hash collisions. The key never becomes certificate or model content.

The canonical writer caches each atom's ASCII/Unicode verdict. An ASCII-only term uses SWI's C writer without a per-subterm portray hook. A term holding a Unicode atom keeps the canonical Unicode portray path.

Candidate instances read a fixed model; NAF tests the whole correlated conjunction, with local variables existential. Ground instances are registered one clause batch at a time, preserving clause/instance order without retaining a composition-wide candidate list. A FIFO settles facts by their lowest founded level and insertion order. Each positive prerequisite owns its waiting incidences. Facts start at 0; rules start at 1 + the maximum positive level (empty maximum = 0). Settling level L makes newly ready instances level L+1, so the queue stays monotone; a checked assertion guards this invariant. Cyclic alternative supports cannot mask a founded one. A remaining unsupported fact fails at `certificate`.

The source loader admits only schema literals, the four schema built-ins, conjunctions and NAF; document directives = the schema's multifile/discontiguous declarations. It checks left-to-right variable binding. The Rust prototype still owns the full admission check and every acceptance decision.

## Producer controls

```sh
python3 -I prototype/sb1/producer-tests/run.py "$PWD"
python3 -I prototype/sb1/producer-tests/run.py "$PWD" --consumer
```

The first run checks canonical bytes against the fork's writer, independent SHA-256 framing, correlated NAF, alternate cyclic/founded supports, unfounded residuals, minimal levels, valuation order, metadata, deterministic bytes and failure stages. It includes 36 keyed-vs-linear query pairs (world + model; partial keys, variable heads, nested Deps, collisions) and 136 FIFO-vs-heap cases (deep chains, ties, duplicate prerequisites, cycles, zero-positive rules). A reversed-level queue must fire the monotonicity assertion. Load, certificate and write failures use real inputs; eval and resource exception paths use injected faults. The second adds the CDC corpus with each committed consumer world: adult, minor, cancer and the dose-arithmetic world. A separate SWI process loads the REFERENCE recipe and compares exact model sets. It re-instantiates every certificate claim from the original document, checks each positive level and evaluates NAF + built-ins against the fixed model. Temporary files stay in `prototype/sb1/checker/target/producer-tests/` and the runner removes them.

The complete byte differential uses transport-0 revision `4980bbb2`: four consumer worlds + all nine family variants at G=1/2/4. It compares model.txt bytes and expands model-line references before comparing the exact (fact, clause id, valuation, level) claim rows. The output directory must be new. It keeps SHA-256 identities, comparison rows and phase files; it removes bulky files only after `cmp` succeeds. Generated compositions stay in a minimal fixture root under the checker's `target/` and are removed at close.

```sh
python3 -I prototype/sb1/producer-tests/compare.py "$PWD" \
  --ckc rust/target/release/ckc --out .scratch/producer-compare
```

The development timing runner generates F1a/F1b/MX at G=2/4/10 and calls `ckc bench run` once per point. Pin built `ckc` + checker paths. Set `SB1_TIMING_LOCK` to the lock shared by every timing owner. The runner requires its own heavy holder alone; it retains every harness record + phase/counter/RSS file and removes bulky model/certificate files after recording their SHA-256. One fixed-order observation per cell is diagnostic, not the graded campaign or a feasibility result.

```sh
flock "$SB1_TIMING_LOCK" heavy python3 -I prototype/sb1/producer-tests/timings.py "$PWD" \
  --ckc rust/target/release/ckc --checker prototype/sb1/checker/target/release/checker \
  --out .scratch/producer-development
```
