# Certification fixtures

`cases.tsv` pins 41 document edits and seven query edits against `cdc-2022-opioid`.
The binary replayer also certifies all ten unmodified artifacts.
The document cases include removal of a negation-as-failure body item.
Twelve document cases and three query cases edit a schema v2 annotation or the record's temporal digest.
A wrong table always changes that digest, so the digest cases cover a pairing with the wrong table.
Sixteen document cases edit a schema v3 annotation, a reserved v3 form, or the declared schema version.

| Column | Meaning |
|---|---|
| `id` | Unique case name. |
| `kind` | `doc` or `query`. |
| `target` | Repository-relative PL path under `guidelines/<id>/`. |
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

Run `ckc certify --cases tests/certify/cases.tsv` from the repository root with pinned SWI-Prolog on `PATH`.
`just certify` runs the same battery after corpus certification.
`rust/ckc/src/certify_cases_cli.rs` replays each case through the binary in temporary guideline copies.
Each copy contains one document or query, its ACE source, and its lexicon.
The certifier obtains fresh, duplicate DRS dumps through the upstream APE driver.
No stored DRS dump or Python runner participates in this battery.

`just test` validates the table, edit operations, and document ordering without starting SWI-Prolog.
