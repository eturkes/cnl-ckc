# Certification fixtures

`cases.tsv` pins 12 document edits and four query edits against `cdc-2022-opioid`.
The native test also certifies both unmodified artifacts.

| Column | Meaning |
|---|---|
| `id` | Unique case name. |
| `kind` | `doc` or `query`. |
| `target` | PL path relative to `guidelines/cdc-2022-opioid/`. |
| `op` | `replace-first`, `swap-lines`, or `truncate-from`. |
| `arg1` | Original bytes, first line index, or truncation marker. |
| `arg2` | Replacement bytes or second line index; empty for truncation. |
| `expected_rc` | Exact exit code. |
| `expected_stderr` | Exact stderr bytes. |

Fields use `\n`, `\r`, `\t`, and `\\` escapes.
Line indices are zero-based.
`truncate-from` removes the marker and all subsequent bytes.
Every mutation must change its source.
Every rejection must have empty stdout.

Run `just test` with the pinned SWI-Prolog toolchain on `PATH`.
`rust/ckc/tests/certify_cases.rs` runs the built binary in temporary guideline copies.
Each copy contains one document or query, its ACE source, and its lexicon.
The certifier obtains fresh, duplicate DRS dumps through the upstream APE driver.
No stored DRS dump or Python runner participates in this battery.
