# Deterministic regeneration

`tools/regen.py` is derived bootstrap code and must never be corrected by hand.
Edit `tools/regen.emm`, then regenerate its Python bytes. The explicit roots constant is
`tools` only; fixtures are deliberately outside the generation roots.

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
it with committed `regen.py`, and runs the self-check. Across its jobs, CI lints and runs nine
shell harnesses: `tests/strict-harness.sh` (57 gates), `tests/regen-harness.sh` (16),
`tests/adapter-harness.sh` (45), `tests/pipeline-harness.sh` (27),
`tests/ape-vendor-harness.sh` (10), `tests/ir-validate-harness.sh` (60),
`tests/ir-lower-harness.sh` (34), `tests/ir-run-harness.sh` (67), and
`tests/slice-harness.sh` (25). The explicit comparison breaks the self-check trust circle.
Actions are SHA-pinned. The `test` job stays offline after provisioning; the `ape` job performs
one pinned-Clex network fetch inside `tests/ape-vendor-harness.sh` and accepts it only after
digest verification.
