# Deterministic regeneration

`tools/regen.py` is derived bootstrap code and must never be corrected by hand.
Edit `tools/regen.emm`, then regenerate its Python bytes. The explicit roots constant is
`tools` only; fixtures are deliberately outside the generation roots.

Run from the repository root:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 -P tools/regen.py --check
PYTHONDONTWRITEBYTECODE=1 python3 -P tools/regen.py --regenerate
```

Check mode is the default; `--check` makes it explicit. Discovery produces sorted POSIX
relative paths for every `*.emm` under a root, and each source is compiled in a fresh
vendored strict-CLI subprocess.

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
exceptions remain uncaught. A `Require` assertion checks the vendored strict source, so a
wrong working directory fails immediately.

Regeneration compiles every source once before any write and aborts on any failure. After a
clean first pass, it compiles each source again, writes same-directory `*.tmp.<pid>` bytes,
and atomically installs them with `os.replace`.

CI runs the vendored suite, verifies vendor integrity, strict-compiles `regen.emm` and `cmp`s
it with committed `regen.py`, runs the self-check, lints both shell harnesses, then runs the
strict and regeneration harnesses. The explicit comparison breaks the self-check trust
circle. Actions are SHA-pinned and all post-provision steps stay offline.
