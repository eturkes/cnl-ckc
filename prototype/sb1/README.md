# SB1 thin-path prototype

Untrusted producer + checker construction for contract `sb` D7. Prototype acceptance never means the verified patient-answer claim. The phase close retires this directory.

## Producer

Run SWI-Prolog 10.0.2 from the pinned toolchain. All paths below are repository-relative; pass the root explicitly. A composition lists its documents and patient world, one path per line. Its relative paths resolve against `--root`, not the working directory.

```sh
swipl -q -f none -F none -s prototype/sb1/producer.prolog \
  -g sb1_main -t 'halt(9)' -- \
  --root "$PWD" <composition.tsv> <empty-output-dir>
```

Outputs follow `bench/sb/interfaces.md`: canonical `model.txt`, `sb1-certificate 0` envelope + composition identity + one support claim per fact, and `producer-phases.tsv` (load, eval, certificate, write; wall seconds + CPU seconds). The write interval covers model + certificate serialization. Its own timing-table publication remains in the harness's outer interval. Existing output files fail at `write`; use a new directory for each run. Catchable failure: rc 1 + one `sb1-producer <stage> <detail>` stderr line; stages = load | eval | certificate | write | resource. A native signal can prevent that line. The harness must classify an abnormal producer exit as indeterminate, never accept partial files. Success: rc 0, empty stdout + stderr.

The recipe tables every schema predicate. The producer enumerates all of them, including metadata and schema-version facts. Candidate instances read a fixed model; NAF tests the whole correlated conjunction, with local variables existential. A min-heap settles facts by their lowest founded level. Each positive prerequisite owns its waiting incidences. Facts start at 0; rules start at 1 + the maximum positive level (empty maximum = 0). Cyclic alternative supports cannot mask a founded one. A remaining unsupported fact fails at `certificate`.

The source loader admits only schema literals, the four schema built-ins, conjunctions and NAF; document directives = the schema's multifile/discontiguous declarations. It checks left-to-right variable binding. The Rust prototype still owns the full admission check and every acceptance decision.

## Producer controls

```sh
python3 -I prototype/sb1/producer-tests/run.py "$PWD"
python3 -I prototype/sb1/producer-tests/run.py "$PWD" --consumer
```

The first run checks canonical bytes against the fork's writer, independent SHA-256 framing, correlated NAF, alternate cyclic/founded supports, unfounded residuals, minimal levels, valuation order, metadata, deterministic bytes and failure stages. Load, certificate and write failures use real inputs; eval and resource exception paths use injected faults. The second adds the CDC corpus with each committed consumer world: adult, minor, cancer and the dose-arithmetic world. A separate SWI process loads the REFERENCE recipe and compares exact model sets. It re-instantiates every certificate claim from the original document, checks each positive level and evaluates NAF + built-ins against the fixed model. Temporary files stay in `.scratch/` and the runner removes them.
