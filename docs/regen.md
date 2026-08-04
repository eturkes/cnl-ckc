# Deterministic regeneration

`tools/regen.py` is derived bootstrap code: every correction goes through `tools/regen.emm`,
then regenerating its Python bytes — the emitted Python stays byte-authoritative. The explicit
roots constant is `tools` only; fixtures are deliberately outside the generation roots.
Registry-derived Ulex sidecars and full-chain guideline goldens are outside those roots too;
`tests/guideline-harness.sh` owns their `cmp`/`diff` drift gates, not
`tools/regen.py --regenerate`.

Run from the repository root:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 -P tools/regen.py --check
PYTHONDONTWRITEBYTECODE=1 python3 -P tools/regen.py --regenerate
```

Check mode is the default; `--check` makes it explicit. Discovery recursively walks each
root with sorted `Path.iterdir` calls, producing sorted POSIX relative paths for every
`*.emm`; generation roots do not prune dot-directories. Each source is compiled in a
fresh vendored strict-CLI subprocess. The repository-wide `conftest.py` sweep prunes
dot-directory entries before descent.

Violation lines use `regen: <category>: <path>`. Categories have fixed order and sorted paths:

- `compile-error` — the strict CLI rejected an E-- source.
- `missing` — compilation succeeded but sibling generated Python is absent.
- `drift` — generated Python bytes differ from fresh strict output.
- `orphan` — Python under a root has no sibling E-- source.
- `unauthorized` — tracked Python is outside the vendor manifest and generated set.
- `conftest` — a non-hidden on-disk `conftest.py` could hijack pytest discovery.

Violations end with `regen: violations: <N>`; a clean check prints `regen: check ok`.
Successful regeneration prints `regen: wrote <path>` for each target, then
`regen: regenerate ok`.

Exit 0 means success. Exit 1 means violations, compile failures, or an uncaught environment
traceback. Exit 2 is an argparse usage error.

Environment failures are fail-loud by design: filesystem, Git, decode, and subprocess
exceptions remain uncaught. Missing generation roots and unreadable directories reached by
either generation traversal or the non-hidden `conftest.py` sweep therefore produce an
uncaught traceback and exit 1. A `Require` assertion checks the vendored strict source, so a
wrong working directory fails immediately.

Regeneration compiles every source exactly once and buffers each successful stdout payload
before any write. Any compile failure aborts with zero writes. After all sources compile,
it writes same-directory `*.tmp.<pid>` bytes and atomically installs them with `os.replace`.

CI runs the vendored suite, verifies vendor integrity, strict-compiles `regen.emm` and `cmp`s
it with committed `regen.py`, and runs the self-check. Across its jobs, CI lints and runs twelve
shell harnesses: `tests/strict-harness.sh` (57 gates), `tests/regen-harness.sh` (16),
`tests/pipeline-cli-harness.sh` (17), `tests/adapter-harness.sh` (80),
`tests/pipeline-harness.sh` (28),
`tests/ape-vendor-harness.sh` (10), `tests/ir-validate-harness.sh` (122),
`tests/ir-lower-harness.sh` (151), `tests/ir-run-harness.sh` (258),
`tests/slice-harness.sh` (29), `tests/registry-harness.sh` (94), and
`tests/guideline-harness.sh` (45). The explicit
comparison breaks the self-check trust circle.
The IR fixture inventory is pinned before execution: adapter = 18 green ACE/golden pairs and 6 red ACE inputs; direct IR = 12 green and 86 red records; lower chain = 38 complete DRS/IR/program/result stems and 100 red DRS records; direct run = 17 program/result pairs and 70 red programs. The adapter harness compares ACE, golden, and executed green stems plus red ACE and explicit red-dispatch stems; the lower harness compares fixture and executed stems. M6.3 contributes 12 adapter greens plus 2 parser-wall reds, 1 direct-IR green plus 14 paired IR/program reds, 17 complete lower-chain stems plus 17 lower reds, and 4 direct-run greens plus 4 runtime-only reds. Fixture counts, dispatch inventories, semantic negative controls, and expected gate totals are separate assertions.

Actions are SHA-pinned. Every job stays offline after provisioning:
`tests/ape-vendor-harness.sh` consumes the vendored `vendor/clex/clex_lexicon.pl`
under its manifest gates instead of any network fetch.
