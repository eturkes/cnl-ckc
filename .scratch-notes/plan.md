# M5.3 shell working plan

Dispatch = prod-m5u3s-1; unit total = 20 section groups; batch ≤15 tool calls, compilable groups committed. Contract read-and-conform-to = .agent/contracts/m5u3.md. Oracle = tools/goal.py.

| order / section | state source | oracle anchor | status | parity evidence |
|---|---|---|---|---|
| 1. fork notices / pristine / vendor | W+I+C | goal.py:522–639 | implemented | baseline + unknown-license exact |
| 2. docid probe | P | goal.py:4235 | verified section | boundary exact stdout/stderr/rc |
| 3. trace-numeric probe | P | goal.py:4246 | blocked K3 | explicit stderr rc2 trace seam pending |
| 4. swipl wall probe | P | goal.py:4280 | verified section | legacy 2.107s / Rust 2.015s; direct + descendant-held pipes |
| 5. adjudication fixtures | W+P | goal.py:3228 | blocked K4 | green-absent-ledger golden mismatch on current kernel |
| 6. compendium orgs / rows | W | goal.py:2044–2259 | implemented | baseline + header mutant exact |
| 7. source record / guideline / pl / alignment inventories | W | goal.py:161–257 | implemented | baseline + missing README exact; sorted derived names |
| 8. red inventory | W | goal.py:1899 | implemented | baseline + orphan ulex exact |
| 9. Prolog INDEX inventory incl drs_driver | I+W | goal.py:423 | implemented | indexed baseline exact incl drs_driver |
| 10. projection ledger | W | goal.py:2260 | implemented | baseline + missing LF exact |
| 11. product vocabulary | W | goal.py:2785 | implemented | baseline + unauthorized functor exact |
| 12. coverage + payloads (kernel) | W | goal.py:2452 | blocked K4 | kernel violation returned success; shell rc2 |
| 13. census map | W | goal.py:2300 | implemented | baseline + invalid census key exact |
| 14. adjudication + review manifest + ledger + commit custody (kernel/git) | W+C | goal.py:2822–3227 | composed; R68 adaptation pending | parse prefix self-hash before grammar; git custody; runtime not green |
| 15. lexicon (kernel) | W | goal.py:2721 | composed | ulex/clex/ACE sources + shadow inventory -> kernel |
| 16. pinned SWI + stage | W+P | goal.py:48–160 | verified section | stage + all 29 ACE red probes exact |
| 17. documents deterministic compile / proof / load + K1/K2 | W+P | goal.py:641–737 | verified focused probe | 1-doc pipeline 12.317s legacy / 10.530s Rust; stale mutation exact |
| 18. query fixtures + K2/K3 seams | W+P | goal.py:3306–3614 | partial; R77/K3 pending | 11/16 pretrace cases exact; 5 pre-R77 fixture rejects are ruled re-pins; K3 seam explicit |
| 19. ACE red probes | W+P | goal.py:1950 | verified 29 probes | exact class/rc/stderr pins; 38.321s legacy / 37.499s Rust |
| 20. final meter | W+P | goal.py:4650 | implemented | emits only after every section succeeds |

Exclusions = strict (M5.7), UI/copy (M5.5), dist (M5.6). Dispatcher = goal.py:4582–4657.

Exit law = violation → escaped detail + stdout LF, rc1; fail → escaped detail + stderr LF, rc2; relay_failure → exact child stderr + child rc. Escaping = LF→\n, CR→\r.

Trace seam = explicit `goal: queries-fixtures: trace seam pending` stderr rc2 until the K3 binding lands; no silent green. Kernel sections use bindings only.

Gates = cargo build --release --locked --offline; cargo clippy --workspace --locked --offline --all-targets -q -- -D warnings; verusfmt --edition 2024 --check ckc/src/*.rs ckc/src/check/*.rs ckc/tests/*.rs. Primary parity harness owns full runtime check.

Checkpoint eee1df1: build/fmt passed; its commit body prematurely claimed probes because shell continued after the scratch driver failed to compile (sha2 LowerHex). Corrected driver + rerun on eee1df1: 6/7 exact; guideline inventory rejected valid corpus because mapped filenames were not re-sorted. 2acb40d also overstated parity after a failed assertion; committed correction follows actual evidence.

Executed probe rerun after filename-order fix = 7/7 exact stdout/stderr/rc, section_probes_rc=0; all baseline silent sections compared with a positive-control fork meter and matching red failures.

Structural checkpoint: build + clippy -D warnings + verusfmt green; compendium/projection/census/vocabulary probes = 8/8 exact. Text law pins Unicode 15.1 full casefold (1530 mappings) + 68 decimal blocks; Rust lowercase is not Python casefold.

K4 export integration = 7f5ff6ee: lead-authorized one-line `ckc_spec::check` public reexport. Actual release build + workspace clippy + shell fmt-check pass; scratch type-adapter evidence is superseded. Fresh release checker prints the fork meter then explicit K3 seam-pending stderr rc2. Kernel bodies remain unread; the only kernel mutation is the authorized public reexport.

## Handoff gates + boundaries

- `cargo build --release --locked --offline -q` = rc0; `cargo clippy --workspace --locked --offline --all-targets -q -- -D warnings` = rc0 after 7f5ff6ee. The missing-K4-export blocker is resolved.
- `verusfmt --edition 2024 --check ckc/src/*.rs ckc/src/check/*.rs ckc/tests/*.rs` = rc0. No type adapter is needed for the actual crate gates.
- Exact lead-authorized one-line reexport at `ckc-kernel/src/lib.rs:70` fails that file's `verusfmt --check` (rc1); additional kernel formatting remains outside the one-line exception. Lead notified. Supplied :61 location was stale: the only printed line there was `mod k4_render;`; no kernel body was displayed.
- Fresh `rust/target/release/ckc check` = stdout `goal: fork notices ok 3 trees 9 modified files\n`, stderr `goal: queries-fixtures: trace seam pending\n`, rc2. Expected meter prefix = 1/11; later sections not executed through the dispatcher.
- Pretrace query failures (5/16): empty-solutions, limit-depth, limit-inner-inference, no-finite-failure, yesno-limit-before-proof. Legacy = `goal: queries: non-demo result for qid …` stdout rc1; current K2 = `ace_to_pl_error(check_load,noncanonical('<fixture>/pl/doc.pl')).` stderr rc2. Direct K1 on no-finite-failure doc.pl = `noncanonical(1,14)` (first line = synthetic product comment). R77 rules these as the R39 fixture re-pin, not kernel defects. Main prepares 20 canonical document headers plus canonical staleness encodings; merge main and re-probe after that commit lands. Main at 50571be0 still carries the synthetic headers; no main merge yet.
- K3 integration points = `probes::trace_numeric`, `queries::trace_raw`, `queries::inspect_trace`; each explicitly fails rc2. Trace raw seam carries caller-specific wall-clock category/detail for query, trace-reject, nonfinite probes.
- R68 = replace old `check_ledger -> Result` at adjudication.rs with `(prefix, first violation)`, perform prefix commit checks before returning grammar violation. Old-interface limitation remains explicit, not claimed parity.
- Historical extraction = worktree-local `.goal.tmp.<pid>/guidelines/<gid>` instead of legacy random `/tmp/tmp*`; malformed historical-data diagnostics carry that scratch path. Successful custody is path-independent; exact random-prefix parity not claimed.
- Full `ckc check`, full legacy check, P6 full-corpus wall, full query fixture lanes, cargo tests/Verus/trust, and separate adversarial review = not run. LSP schema unavailable (`ToolSearch LSP` returned no match). No corpus/tests/vendor/spec edits; the kernel has the one authorized public reexport line, with no body changes. No dependencies added.

## Oracle emitter census

Exact source expressions below bind section diagnostics to `tools/goal.py`; dispatcher order/state sources = table above. F = fail/cleanup_and_fail; V = violation/cleanup_violation/queries_violation; R = relay_failure; M = print. Excluded wrappers = strict/UI/copy/dist.

### fail

```text
M tools/goal.py:19 print("goal: " + category + ": " + safe_detail, file=sys.stderr)
```

### violation

```text
M tools/goal.py:24 print("goal: " + category + ": " + safe_detail)
```

### cleanup_and_fail

```text
F tools/goal.py:28 fail(category, detail)
```

### cleanup_violation

```text
V tools/goal.py:31 violation(category, detail)
```

### resolve_swipl

```text
F tools/goal.py:52 fail("swipl-exec", "not executable: " + swipl)
F tools/goal.py:55 fail("swipl-version", "version probe failed: " + swipl_executable)
F tools/goal.py:67 fail("swipl-version", "expected " + swipl_version_required + ", found: " + version_text.strip())
```

### make_scratch

```text
F tools/goal.py:73 fail("scratch", "already exists: " + str(scratch_path))
```

### bounded_swipl_run

```text
F tools/goal.py:105 cleanup_and_fail(scratch_path, "swipl-timeout", label + " exceeded " + str(swipl_wall_seconds) + "s wall clock")
```

### stage_ape

```text
F tools/goal.py:110 cleanup_and_fail(scratch_path, "compiler-source", "missing: " + str(compiler_source))
R tools/goal.py:120 relay_failure(scratch_path, result)
F tools/goal.py:123 cleanup_and_fail(scratch_path, "ape-stage", "missing grammar.plp after build")
```

### compile_doc

```text
R tools/goal.py:141 relay_failure(scratch_path, result)
F tools/goal.py:143 cleanup_and_fail(scratch_path, "compiler-stderr", "non-empty stderr for document: " + docid)
F tools/goal.py:145 cleanup_and_fail(scratch_path, "compiler-stdout", "empty stdout for document: " + docid)
F tools/goal.py:149 cleanup_and_fail(scratch_path, "compiler-stdout", "missing final newline for document: " + docid)
```

### check_doc_load

```text
R tools/goal.py:156 relay_failure(scratch_path, result)
F tools/goal.py:158 cleanup_and_fail(scratch_path, "check-stdout", "non-empty stdout for: " + str(pl_path))
F tools/goal.py:160 cleanup_and_fail(scratch_path, "check-stderr", "non-empty stderr for: " + str(pl_path))
```

### collect_guideline

```text
F tools/goal.py:165 fail("guideline", "ace directory is a symlink: " + str(ace_dir))
F tools/goal.py:167 fail("guideline", "missing ace directory: " + str(ace_dir))
F tools/goal.py:175 fail("guideline", "entry is not a regular file: " + entry_name)
F tools/goal.py:177 fail("guideline", "entry is not a regular file: " + entry_name)
F tools/goal.py:179 fail("guideline", "unsupported ace entry: " + entry_name)
F tools/goal.py:182 fail("docid", "invalid document id: " + docid)
F tools/goal.py:186 fail("guideline", "no .ace documents in: " + str(ace_dir))
F tools/goal.py:190 fail("guideline", "lexicon is a symlink: " + str(lexicon_path))
```

### check_source_record

```text
V tools/goal.py:199 violation("source-record", "README.md is a symlink under: " + str(guideline_path))
V tools/goal.py:201 violation("source-record", "missing README.md under: " + str(guideline_path))
V tools/goal.py:205 violation("source-record", "source is a symlink under: " + str(guideline_path))
V tools/goal.py:207 violation("source-record", "missing source directory under: " + str(guideline_path))
V tools/goal.py:212 violation("source-record", "symlink under source: " + entry.name)
V tools/goal.py:217 violation("source-record", "no source files under: " + str(guideline_path))
```

### check_pl_inventory

```text
V tools/goal.py:222 violation("pl-dir", "is a symlink: " + str(pl_dir))
V tools/goal.py:224 violation("missing-pl", str(pl_dir))
V tools/goal.py:234 violation("pl-entry", "not a regular file: " + entry_name)
V tools/goal.py:236 violation("pl-entry", "not a regular file: " + entry_name)
V tools/goal.py:239 violation("pl-inventory", "committed pl/ does not match ace/ document set")
```

### check_alignment

```text
V tools/goal.py:244 violation("align-dir", "is a symlink: " + str(align_dir))
V tools/goal.py:246 violation("missing-align", str(align_dir))
V tools/goal.py:256 violation("align-entry", "not a regular file: " + entry_name)
V tools/goal.py:258 violation("align-entry", "not a regular file: " + entry_name)
V tools/goal.py:261 violation("align-inventory", "committed align/ does not match ace/ document set")
```

### check_prolog_inventory

```text
V tools/goal.py:441 violation("prolog-inventory", "unauthorized tracked prolog: " + tracked)
```

### license_requires_date

```text
V tools/goal.py:471 violation("fork-notice", "unrecognized license in " + tree_prefix + ": " + license_id)
```

### check_pristine_tree

```text
V tools/goal.py:525 violation("fork-notice", "pristine tree lacks MANIFEST.sha256: " + tree_prefix)
V tools/goal.py:533 violation("fork-notice", "malformed manifest row in " + tree_prefix + ": " + manifest_line)
V tools/goal.py:547 violation("fork-notice", "pristine vendored file carries a change notice: " + tracked_path)
V tools/goal.py:549 violation("fork-notice", "pristine vendored file carries a change notice: " + tracked_path)
V tools/goal.py:552 violation("fork-notice", "pristine file missing from manifest: " + tracked_path)
V tools/goal.py:558 violation("fork-notice", "pristine file differs from manifest digest: " + tracked_path)
V tools/goal.py:562 violation("fork-notice", "manifest row without tracked file: " + full_name)
V tools/goal.py:565 violation("fork-notice", "manifest row names first-party file: " + full_name)
```

### check_vendor_tree

```text
V tools/goal.py:571 violation("fork-notice", "missing PROVENANCE: " + tree_prefix)
V tools/goal.py:577 violation("fork-notice", "PROVENANCE states no License: " + tree_prefix)
V tools/goal.py:580 violation("fork-notice", "PROVENANCE states no Import commit: " + tree_prefix)
V tools/goal.py:590 violation("fork-notice", "declared first-party file is untracked: " + declared_path)
V tools/goal.py:610 violation("fork-notice", "modified vendored file carries no change notice: " + tracked_path)
V tools/goal.py:612 violation("fork-notice", "change notice is not prominent: " + tracked_path)
V tools/goal.py:616 violation("fork-notice", "change notice states no date: " + tracked_path)
V tools/goal.py:619 violation("fork-notice", "unmodified vendored file carries a change notice: " + tracked_path)
```

### check_fork_notices

```text
V tools/goal.py:624 violation("fork-notice", "missing vendor directory")
V tools/goal.py:630 violation("fork-notice", "vendor entry is a symlink: " + entry_name)
V tools/goal.py:632 violation("fork-notice", "vendor entry is not a directory: " + entry_name)
V tools/goal.py:637 violation("fork-notice", "no vendor trees found")
V tools/goal.py:639 violation("fork-notice", "no modified vendored files found")
M tools/goal.py:640 print("goal: fork notices ok " + str(tree_count) + " trees " + str(modified_total) + " modified files")
```

### check_documents

```text
V tools/goal.py:653 cleanup_violation(scratch_path, "determinism", "two compiles differ for document: " + docid)
V tools/goal.py:657 cleanup_violation(scratch_path, "stale", "committed pl differs from fresh compile: " + str(committed_path))
V tools/goal.py:662 cleanup_violation(scratch_path, "determinism", "two proof runs differ for document: " + docid)
V tools/goal.py:669 cleanup_violation(scratch_path, "aggregate-totality", "manifest documents differ from corpus documents: " + str(guideline_path))
```

### run_aggregate

```text
R tools/goal.py:695 relay_failure(scratch_path, result)
F tools/goal.py:697 cleanup_and_fail(scratch_path, "aggregate-stderr", "non-empty stderr for manifest: " + str(manifest_path))
V tools/goal.py:701 cleanup_violation(scratch_path, "aggregate-report", "unexpected report: " + stdout_text.strip())
V tools/goal.py:703 cleanup_violation(scratch_path, "aggregate-report", "unexpected report: " + stdout_text.strip())
```

### run_recursion

```text
R tools/goal.py:710 relay_failure(scratch_path, result)
F tools/goal.py:712 cleanup_and_fail(scratch_path, "recursion-stderr", "non-empty stderr for manifest: " + str(manifest_path))
V tools/goal.py:716 cleanup_violation(scratch_path, "recursion-report", "unexpected report: " + stdout_text.strip())
V tools/goal.py:718 cleanup_violation(scratch_path, "recursion-report", "unexpected report: " + stdout_text.strip())
```

### check_aggregate

```text
V tools/goal.py:731 cleanup_violation(scratch_path, "aggregate-order", "forward and reverse manifests disagree")
M tools/goal.py:733 print("goal: " + recursion_report.strip())
```

### queries_violation

```text
V tools/goal.py:736 violation(category, detail)
V tools/goal.py:737 cleanup_violation(scratch_path, category, detail)
```

### collect_query_aces

```text
V tools/goal.py:741 queries_violation(scratch_path, "queries", "is a symlink: " + str(queries_dir))
V tools/goal.py:745 queries_violation(scratch_path, "queries", "not a directory: " + str(queries_dir))
V tools/goal.py:758 queries_violation(scratch_path, "queries", "unsupported entry: " + str(entry))
V tools/goal.py:760 queries_violation(scratch_path, "queries", "unsupported entry: " + str(entry))
V tools/goal.py:763 queries_violation(scratch_path, "queries", "unsupported entry: " + str(entry))
V tools/goal.py:765 queries_violation(scratch_path, "queries", "not a regular file: " + str(entry))
V tools/goal.py:767 queries_violation(scratch_path, "queries", "not a regular file: " + str(entry))
V tools/goal.py:770 queries_violation(scratch_path, "queries", "invalid qid filename: " + entry_name)
```

### collect_query_dir

```text
V tools/goal.py:781 queries_violation(scratch_path, "queries", "unsupported entry: " + str(entry))
V tools/goal.py:783 queries_violation(scratch_path, "queries", "not a regular file: " + str(entry))
V tools/goal.py:785 queries_violation(scratch_path, "queries", "not a regular file: " + str(entry))
V tools/goal.py:788 queries_violation(scratch_path, "queries", "invalid qid filename: " + entry_name)
```

### run_question_compile

```text
V tools/goal.py:808 queries_violation(scratch_path, "queries", "wall_clock for qid: " + qid)
R tools/goal.py:810 relay_failure(scratch_path, result)
F tools/goal.py:812 cleanup_and_fail(scratch_path, "compiler-stderr", "non-empty stderr for question: " + qid)
F tools/goal.py:814 cleanup_and_fail(scratch_path, "compiler-stdout", "empty stdout for question: " + qid)
F tools/goal.py:817 cleanup_and_fail(scratch_path, "compiler-stdout", "missing final newline for question: " + qid)
```

### run_answer

```text
V tools/goal.py:825 queries_violation(scratch_path, "queries", "wall_clock for qid: " + qid)
R tools/goal.py:827 relay_failure(scratch_path, result)
F tools/goal.py:829 cleanup_and_fail(scratch_path, "answer-stderr", "non-empty stderr for question: " + qid)
F tools/goal.py:831 cleanup_and_fail(scratch_path, "answer-stdout", "empty stdout for question: " + qid)
F tools/goal.py:834 cleanup_and_fail(scratch_path, "answer-stdout", "missing final newline for question: " + qid)
```

### run_trace

```text
V tools/goal.py:842 queries_violation(scratch_path, "queries", "wall_clock for qid: " + qid)
R tools/goal.py:844 relay_failure(scratch_path, result)
F tools/goal.py:846 cleanup_and_fail(scratch_path, "trace-stderr", "non-empty stderr for question: " + qid)
F tools/goal.py:848 cleanup_and_fail(scratch_path, "trace-stdout", "empty stdout for question: " + qid)
F tools/goal.py:851 cleanup_and_fail(scratch_path, "trace-stdout", "missing final newline for question: " + qid)
```

### trace_join_nodes

```text
V tools/goal.py:1484 queries_violation(scratch_path, "traces", "trace node resolves to no committed clause line: " + qid + " " + docid_value + " S" + str(s_value))
V tools/goal.py:1486 queries_violation(scratch_path, "traces", "trace node resolves to multiple committed clause lines: " + qid + " " + docid_value + " S" + str(s_value))
```

### validate_queries

```text
V tools/goal.py:1494 queries_violation(scratch_path, "queries", "pl inventory differs from ace query set: " + str(queries_dir.joinpath("pl")))
V tools/goal.py:1496 queries_violation(scratch_path, "queries", "answers inventory differs from ace query set: " + str(queries_dir.joinpath("answers")))
V tools/goal.py:1498 queries_violation(scratch_path, "queries", "traces inventory differs from ace query set: " + str(queries_dir.joinpath("traces")))
V tools/goal.py:1511 queries_violation(scratch_path, "determinism", "two query compiles differ for question: " + qid)
V tools/goal.py:1515 queries_violation(scratch_path, "stale", "committed query pl differs from fresh compile: " + str(committed_pl))
V tools/goal.py:1519 queries_violation(scratch_path, "determinism", "two answer runs differ for question: " + qid)
V tools/goal.py:1523 queries_violation(scratch_path, "stale", "committed query answers differ from fresh answer: " + str(committed_answers))
V tools/goal.py:1526 queries_violation(scratch_path, "queries", "unparsable answer artifact for qid: " + qid)
V tools/goal.py:1534 queries_violation(scratch_path, "queries", "non-demo result for qid " + qid + ": " + result_text)
V tools/goal.py:1538 queries_violation(scratch_path, "queries", "unexpected query pl shape for qid: " + qid)
V tools/goal.py:1547 queries_violation(scratch_path, "determinism", "two trace runs differ for question: " + qid)
V tools/goal.py:1551 queries_violation(scratch_path, "stale", "committed query trace differs from fresh trace: " + str(committed_trace))
V tools/goal.py:1559 queries_violation(scratch_path, "traces", "malformed trace artifact: " + str(committed_trace))
V tools/goal.py:1562 queries_violation(scratch_path, "traces", "non-demo proof for qid: " + qid)
```

### queries_meter

```text
M tools/goal.py:1570 print("goal: queries " + guideline_name + " " + str(query_count) + " queries; wh=" + str(wh_count) + " yesno=" + str(yesno_count))
```

### traces_meter

```text
M tools/goal.py:1577 print("goal: traces " + guideline_name + " " + str(query_count) + " traces; nodes=" + str(node_count))
```

### red_probe_class

```text
V tools/goal.py:1888 violation("red-probe", "probe name lacks <class>-- prefix: " + probe_name)
```

### red_expected_exit

```text
V tools/goal.py:1898 violation("red-class", "unknown error class: " + class_name)
```

### collect_red_probes

```text
V tools/goal.py:1903 violation("red-dir", "is a symlink: " + str(red_dir))
V tools/goal.py:1905 violation("red-dir", "missing: " + str(red_dir))
V tools/goal.py:1915 violation("red-entry", "not a regular file: " + entry_name)
V tools/goal.py:1917 violation("red-entry", "not a regular file: " + entry_name)
V tools/goal.py:1934 violation("red-entry", "unsupported entry: " + entry_name)
V tools/goal.py:1938 violation("red-entry", "orphan ulex without ace probe: " + ulex_stem)
V tools/goal.py:1942 violation("red-entry", "orphan expect without ace probe: " + expect_stem)
V tools/goal.py:1946 violation("red-entry", "probe lacks expect pin: " + ace_stem)
V tools/goal.py:1948 violation("red-dir", "no red probes found")
```

### run_red_probe

```text
V tools/goal.py:1961 cleanup_violation(scratch_path, "red-exit", "status " + str(result.returncode) + " for probe: " + probe_name)
V tools/goal.py:1963 cleanup_violation(scratch_path, "red-stdout", "non-empty stdout for probe: " + probe_name)
V tools/goal.py:1968 cleanup_violation(scratch_path, "red-stderr", "stderr is not one LF line for probe: " + probe_name)
V tools/goal.py:1970 cleanup_violation(scratch_path, "red-stderr", "stderr is not one LF line for probe: " + probe_name)
V tools/goal.py:1974 cleanup_violation(scratch_path, "red-expect", "missing expect pin for probe: " + probe_name)
V tools/goal.py:1977 cleanup_violation(scratch_path, "red-expect", "stderr differs from expect pin for probe: " + probe_name)
V tools/goal.py:1981 cleanup_violation(scratch_path, "red-class", "stderr class mismatch for probe: " + probe_name)
V tools/goal.py:1983 cleanup_violation(scratch_path, "red-stderr", "stderr lacks canonical term suffix for probe: " + probe_name)
```

### read_compendium_file

```text
F tools/goal.py:1997 fail("compendium", "is a symlink: " + path_text)
F tools/goal.py:1999 fail("compendium", "missing: " + path_text)
```

### org_section_text

```text
V tools/goal.py:2004 violation("compendium-org-table", "missing `## Organizations` section in .agent/compendium.md")
```

### org_table_rows

```text
V tools/goal.py:2027 violation("compendium-org-table", "organization table header mismatch")
V tools/goal.py:2037 violation("compendium-org-table", "organization row without 7 cells: " + line_text)
V tools/goal.py:2040 violation("compendium-org-table", "organization table absent - no header row found")
V tools/goal.py:2042 violation("compendium-org-table", "organization table holds no rows")
```

### check_compendium_orgs

```text
V tools/goal.py:2059 violation("compendium-org", "empty org cell in organization row")
V tools/goal.py:2061 violation("compendium-org", "empty class cell for: " + org_name)
V tools/goal.py:2063 violation("compendium-org", "empty CPGs cell for: " + org_name)
V tools/goal.py:2066 violation("compendium-org", "bad class `" + class_cell + "` for: " + org_name)
V tools/goal.py:2069 violation("compendium-org", "bad CPGs `" + cpgs_cell + "` for: " + org_name)
V tools/goal.py:2072 violation("compendium-org", "duplicate organization row: " + org_name)
V tools/goal.py:2075 violation("compendium-org", "bad swept `" + swept_cell + "` for: " + org_name)
V tools/goal.py:2108 violation("compendium-org-order", "organization order violates class -> CPGs -> alpha at: " + current_name)
```

### load_compendium_rows

```text
V tools/goal.py:2116 violation("compendium-tsv", "first line of .agent/compendium.tsv is not the 6-column header")
V tools/goal.py:2123 violation("compendium-tsv", "row without 6 cells starting: " + first_cell)
```

### check_compendium_rows

```text
V tools/goal.py:2142 violation("compendium-row", "bad access `" + access_cell + "` for: " + title_cell)
V tools/goal.py:2144 violation("compendium-row", "bad status `" + status_cell + "` for: " + title_cell)
V tools/goal.py:2147 violation("compendium-row", "title lacks terminal (year): " + title_cell)
V tools/goal.py:2149 violation("compendium-row", "empty org cell for: " + title_cell)
V tools/goal.py:2151 violation("compendium-row", "empty title cell")
V tools/goal.py:2153 violation("compendium-row", "empty URL cell for: " + title_cell)
V tools/goal.py:2157 violation("compendium-row", "URL lacks http(s) scheme: " + url_cell)
V tools/goal.py:2159 violation("compendium-row", "capability-token URL: " + url_cell)
V tools/goal.py:2168 violation("compendium-row", "unverified access must be provisional(...) or excluded(...): " + title_cell)
V tools/goal.py:2172 violation("compendium-row", "unresolved row must not be unqueued: " + title_cell)
V tools/goal.py:2190 violation("compendium-dup", "duplicate guideline row: " + title_cell)
V tools/goal.py:2196 violation("compendium-dup", "URL shared by two rows: " + url_cell)
V tools/goal.py:2213 violation("compendium-active", str(active_count) + " rows queued|in-progress (max 1)")
V tools/goal.py:2223 violation("compendium-row-order", "guideline order violates access -> class -> org -> year desc -> title at: " + current_title)
```

### check_compendium

```text
M tools/goal.py:2242 print(summary)
```

### read_corpus_file

```text
V tools/goal.py:2256 violation(category, "is a symlink: " + str(file_path))
V tools/goal.py:2258 violation(category, "missing: " + str(file_path))
```

### check_projection_ledger

```text
V tools/goal.py:2265 violation("projection-ledger", "carriage return byte in ledger")
V tools/goal.py:2267 violation("projection-ledger", "ledger lacks final newline")
V tools/goal.py:2270 violation("projection-ledger", "header bytes drift")
V tools/goal.py:2275 violation("projection-ledger", "ledger holds no rows")
V tools/goal.py:2281 violation("projection-ledger", "row without 4 columns: " + row_line)
V tools/goal.py:2287 violation("projection-ledger", "invalid docid: " + docid)
V tools/goal.py:2289 violation("projection-ledger", "empty region for: " + docid)
V tools/goal.py:2291 violation("projection-ledger", "empty kept column for: " + docid)
V tools/goal.py:2293 violation("projection-ledger", "empty dropped column for: " + docid)
V tools/goal.py:2296 violation("projection-ledger", "duplicate row docid: " + docid)
```

### check_census_map

```text
V tools/goal.py:2307 violation("census-map", "not UTF-8: " + str(map_path))
V tools/goal.py:2309 violation("census-map", "carriage return byte in map")
V tools/goal.py:2311 violation("census-map", "map lacks final newline")
V tools/goal.py:2313 violation("census-map", "header bytes drift")
V tools/goal.py:2322 violation("census-map", "row without 3 columns: " + line_text)
V tools/goal.py:2327 violation("census-map", "census key grammar: " + census_key)
V tools/goal.py:2330 violation("census-map", "duplicate census key: " + census_key)
V tools/goal.py:2335 violation("census-map", "row names no coverage region: " + census_key + " " + region_field)
V tools/goal.py:2337 violation("census-map", "empty disposition: " + census_key)
V tools/goal.py:2340 violation("census-map", "map holds no rows")
```

### coverage_status_kind

```text
V tools/goal.py:2347 violation("coverage", "malformed status for " + row_id + ": " + status_text)
V tools/goal.py:2351 violation("coverage", "ace names invalid docid for " + row_id + ": " + inner)
V tools/goal.py:2356 violation("coverage", "malformed status for " + row_id + ": " + status_text)
V tools/goal.py:2360 violation("coverage", "empty restates target for " + row_id)
V tools/goal.py:2365 violation("coverage", "malformed status for " + row_id + ": " + status_text)
V tools/goal.py:2370 violation("coverage", "uncovered without class and reason for " + row_id + ": " + status_text)
V tools/goal.py:2375 violation("coverage", "unknown uncovered class for " + row_id + ": " + class_name)
V tools/goal.py:2377 violation("coverage", "empty uncovered reason for " + row_id)
V tools/goal.py:2379 violation("coverage", "unknown status for " + row_id + ": " + status_text)
```

### bind_locator_payload

```text
V tools/goal.py:2383 violation("coverage", "evidence region " + region_id + " carries " + str(line_count) + " content lines in: " + str(evidence_path))
```

### evidence_regions

```text
V tools/goal.py:2405 violation("coverage", "evidence lacks region-authority census: " + str(evidence_path))
V tools/goal.py:2448 violation("coverage", "payload lines " + str(payload_count) + " differ from census " + str(census_count) + " for: " + str(evidence_path))
```

### check_coverage

```text
V tools/goal.py:2457 violation("coverage", "carriage return byte in ledger")
V tools/goal.py:2459 violation("coverage", "ledger lacks final newline")
V tools/goal.py:2462 violation("coverage", "header bytes drift")
V tools/goal.py:2491 violation("coverage", "comment line after rows: " + row_line)
V tools/goal.py:2496 violation("coverage", "row without 5 columns: " + row_line)
V tools/goal.py:2503 violation("coverage", "empty region id: " + row_line)
V tools/goal.py:2506 violation("coverage", "region id holds a space: " + row_id)
V tools/goal.py:2509 violation("coverage", "file outside source/ for " + row_id + ": " + file_field)
V tools/goal.py:2512 violation("coverage", "file path traversal for " + row_id + ": " + file_field)
V tools/goal.py:2514 violation("coverage", "empty page for: " + row_id)
V tools/goal.py:2516 violation("coverage", "empty section for: " + row_id)
V tools/goal.py:2519 violation("coverage", "duplicate region id: " + row_id)
V tools/goal.py:2538 violation("coverage", "docid claimed by two rows: " + payload)
V tools/goal.py:2541 violation("coverage", "ace names unknown docid for " + row_id + ": " + payload)
V tools/goal.py:2555 violation("coverage", "ledger holds no rows")
V tools/goal.py:2559 violation("coverage", "restates itself: " + row_id)
V tools/goal.py:2562 violation("coverage", "restates unknown region for " + row_id + ": " + target_id)
V tools/goal.py:2565 violation("coverage", "restates a restatement for " + row_id + ": " + target_id)
V tools/goal.py:2569 violation("coverage", "docid without a coverage row: " + docid)
V tools/goal.py:2580 violation("coverage", "rows " + str(claimed_count) + " differ from census " + str(census_count) + " for: " + file_field)
V tools/goal.py:2584 violation("coverage", "locators " + str(locator_count) + " differ from census " + str(census_count) + " for: " + file_field)
V tools/goal.py:2589 violation("coverage", "duplicate evidence locator: " + region_id)
V tools/goal.py:2594 violation("coverage", "coverage row without evidence region: " + claimed_id)
M tools/goal.py:2621 print(meter)
```

### lexicon_entry

```text
V tools/goal.py:2627 violation("lexicon-entry", "malformed entry: " + line_text)
V tools/goal.py:2640 violation("lexicon-entry", "malformed entry: " + line_text)
```

### lexicon_shadow_rulings

```text
V tools/goal.py:2692 violation("lexicon-shadow-file", "is a symlink: " + str(rulings_path))
V tools/goal.py:2700 violation("lexicon-shadow-file", "first line is not the 3-column header: " + str(rulings_path))
V tools/goal.py:2705 violation("lexicon-shadow-file", "row without 3 cells: " + line_text)
V tools/goal.py:2710 violation("lexicon-shadow-file", "empty ulex_entry cell: " + line_text)
V tools/goal.py:2712 violation("lexicon-shadow-file", "empty clex_entries cell: " + line_text)
V tools/goal.py:2714 violation("lexicon-shadow-file", "empty ruling for: " + ulex_cell)
V tools/goal.py:2718 violation("lexicon-shadow-file", "duplicate ruling row: " + ulex_cell)
```

### check_lexicon

```text
V tools/goal.py:2740 violation("lexicon-redundant", "clex already provides: " + line_text)
V tools/goal.py:2743 violation("lexicon-duplicate", "entry repeated in lexicon: " + line_text)
V tools/goal.py:2766 violation("lexicon-shadow", "clex shares surface without ruling: " + line_text + " vs " + clex_key)
V tools/goal.py:2775 violation("lexicon-shadow", "stale ruling matches no live shadow: " + ulex_part)
V tools/goal.py:2783 violation("lexicon-dead-lexeme", "no ace document references: " + surface)
M tools/goal.py:2784 print("goal: lexicon ok " + str(lexicon_path) + " " + str(len(entries)) + " entries " + str(len(clex_entries)) + " clex facts " + str(ruled_count) + " ruled shadows")
```

### check_product_vocabulary

```text
V tools/goal.py:2801 violation("product-vocabulary", "unauthorized directive in " + docid + ": " + line_text)
V tools/goal.py:2805 violation("product-vocabulary", "unauthorized directive in " + docid + ": " + line_text)
V tools/goal.py:2808 violation("product-vocabulary", "undeclared indicator in " + docid + ": " + decl_spec)
V tools/goal.py:2814 violation("product-vocabulary", "unauthorized clause functor in " + docid + ": " + functor)
```

### semantic_clause_digest

```text
V tools/goal.py:2842 violation("adjudication", "compiled document encoding: " + docid)
V tools/goal.py:2844 violation("adjudication", "compiled document lacks final newline: " + docid)
V tools/goal.py:2854 violation("adjudication", "noncanonical document record in: " + docid)
V tools/goal.py:2861 violation("adjudication", "noncanonical clause line in: " + docid)
V tools/goal.py:2864 violation("adjudication", "document record count " + str(record_count) + " for: " + docid)
```

### derive_review_manifest

```text
V tools/goal.py:2888 violation("adjudication", "docid without coverage row bytes: " + docid)
V tools/goal.py:2892 violation("adjudication", "docid without region payload: " + docid)
```

### derive_bundles_at_commit

```text
V tools/goal.py:3001 violation("adjudication", "ledger commit lacks guideline tree: " + gid + " " + commit_field)
V tools/goal.py:3005 violation("adjudication", "ledger commit tree extraction failed: " + gid + " " + commit_field)
```

### check_ledger_commit_row

```text
V tools/goal.py:3024 violation("adjudication", "ledger row " + str(row_number) + " commit absent from repository: " + commit_field)
V tools/goal.py:3033 violation("adjudication", "ledger row " + str(row_number) + " docid absent at recorded commit: " + docid + " " + commit_field)
V tools/goal.py:3035 violation("adjudication", "ledger row " + str(row_number) + " digest mismatch at recorded commit: " + docid + " " + commit_field)
```

### validate_ledger

```text
V tools/goal.py:3044 violation("adjudication", "ledger is a symlink: " + str(ledger_path))
V tools/goal.py:3048 violation("adjudication", "ledger is not a regular file: " + str(ledger_path))
V tools/goal.py:3054 violation("adjudication", "ledger encoding")
V tools/goal.py:3056 violation("adjudication", "ledger carriage-return")
V tools/goal.py:3058 violation("adjudication", "ledger final-newline")
V tools/goal.py:3061 violation("adjudication", "ledger header")
V tools/goal.py:3072 violation("adjudication", "ledger header")
V tools/goal.py:3076 violation("adjudication", "ledger row " + str(row_number) + " field-count " + str(field_count))
V tools/goal.py:3085 violation("adjudication", "ledger row " + str(row_number) + " docid-grammar")
V tools/goal.py:3088 violation("adjudication", "ledger row " + str(row_number) + " unknown-docid " + docid)
V tools/goal.py:3091 violation("adjudication", "ledger row " + str(row_number) + " sort-order " + docid + " " + date_field + " after " + prev_docid + " " + prev_date)
V tools/goal.py:3097 violation("adjudication", "ledger row " + str(row_number) + " hex")
V tools/goal.py:3099 violation("adjudication", "ledger row " + str(row_number) + " ace-commit")
V tools/goal.py:3106 violation("adjudication", "ledger row " + str(row_number) + " verdict")
V tools/goal.py:3108 violation("adjudication", "ledger row " + str(row_number) + " reviewer")
V tools/goal.py:3110 violation("adjudication", "ledger row " + str(row_number) + " reviewer")
V tools/goal.py:3112 violation("adjudication", "ledger row " + str(row_number) + " date")
V tools/goal.py:3114 violation("adjudication", "ledger row " + str(row_number) + " comment")
M tools/goal.py:3144 print(meter)
```

### check_adjudication

```text
V tools/goal.py:3152 violation("adjudication", "manifest is a symlink: " + str(manifest_path))
V tools/goal.py:3154 violation("adjudication", "manifest missing: " + str(manifest_path) + regen_hint)
V tools/goal.py:3156 violation("adjudication", "manifest is not a regular file: " + str(manifest_path))
V tools/goal.py:3160 violation("adjudication", "manifest stale: " + str(manifest_path) + regen_hint)
```

### check_adjudication_fixtures

```text
V tools/goal.py:3231 violation("adjudication-fixtures", "is a symlink: " + str(fixtures_root))
V tools/goal.py:3233 violation("adjudication-fixtures", "missing: " + str(fixtures_root))
V tools/goal.py:3238 violation("adjudication-fixtures", "not a case directory: " + entry_name)
V tools/goal.py:3240 violation("adjudication-fixtures", "not a case directory: " + entry_name)
V tools/goal.py:3242 violation("adjudication-fixtures", "invalid case name: " + entry_name)
V tools/goal.py:3245 violation("adjudication-fixtures", "no fixture cases found: " + str(fixtures_root))
V tools/goal.py:3258 violation("adjudication-fixtures", "unsupported entry: " + case_name + "/" + member_name)
V tools/goal.py:3260 violation("adjudication-fixtures", "not a regular file: " + case_name + "/" + member_name)
V tools/goal.py:3262 violation("adjudication-fixtures", "not a regular file: " + case_name + "/" + member_name)
V tools/goal.py:3270 violation("adjudication-fixtures", "case without manifest.tsv: " + case_name)
V tools/goal.py:3273 violation("adjudication-fixtures", "case pins both expect and golden: " + case_name)
V tools/goal.py:3276 violation("adjudication-fixtures", "case without expect or golden pin: " + case_name)
V tools/goal.py:3283 violation("adjudication-fixtures", "status " + str(result.returncode) + " for case: " + case_name)
V tools/goal.py:3285 violation("adjudication-fixtures", "non-empty stderr for case: " + case_name)
V tools/goal.py:3289 violation("adjudication-fixtures", "stdout differs from expect pin for case: " + case_name)
V tools/goal.py:3293 violation("adjudication-fixtures", "status " + str(result.returncode) + " for case: " + case_name)
V tools/goal.py:3295 violation("adjudication-fixtures", "non-empty stderr for case: " + case_name)
V tools/goal.py:3299 violation("adjudication-fixtures", "stdout differs from golden for case: " + case_name)
V tools/goal.py:3302 violation("adjudication-fixtures", "red case count drift: expected 42 got " + str(red_count))
V tools/goal.py:3304 violation("adjudication-fixtures", "green case count drift: expected 9 got " + str(green_count))
M tools/goal.py:3305 print("goal: adjudication fixtures ok " + str(red_count) + " red " + str(green_count) + " green")
```

### check_queries_fixtures

```text
V tools/goal.py:3309 cleanup_violation(scratch_path, "queries-fixtures", "is a symlink: " + str(fixtures_root))
V tools/goal.py:3311 cleanup_violation(scratch_path, "queries-fixtures", "missing: " + str(fixtures_root))
V tools/goal.py:3316 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + entry.name)
V tools/goal.py:3327 cleanup_violation(scratch_path, "queries-fixtures", "is a symlink: " + str(color_path))
V tools/goal.py:3329 cleanup_violation(scratch_path, "queries-fixtures", "missing: " + str(color_path))
V tools/goal.py:3333 cleanup_violation(scratch_path, "queries-fixtures", "not a case directory: " + case_name)
V tools/goal.py:3335 cleanup_violation(scratch_path, "queries-fixtures", "not a case directory: " + case_name)
V tools/goal.py:3337 cleanup_violation(scratch_path, "queries-fixtures", "invalid case name: " + case_name)
V tools/goal.py:3351 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
V tools/goal.py:3354 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
V tools/goal.py:3358 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
V tools/goal.py:3362 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
V tools/goal.py:3366 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
V tools/goal.py:3370 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
V tools/goal.py:3374 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
V tools/goal.py:3377 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
V tools/goal.py:3379 cleanup_violation(scratch_path, "queries-fixtures", "case without tree: " + case_name)
V tools/goal.py:3382 cleanup_violation(scratch_path, "queries-fixtures", "case pins both expect and golden: " + case_name)
V tools/goal.py:3385 cleanup_violation(scratch_path, "queries-fixtures", "case without expect or golden pin: " + case_name)
V tools/goal.py:3388 cleanup_violation(scratch_path, "queries-fixtures", "red case pins golden: " + case_name)
V tools/goal.py:3391 cleanup_violation(scratch_path, "queries-fixtures", "green case pins expect: " + case_name)
V tools/goal.py:3394 cleanup_violation(scratch_path, "queries-fixtures", "case tree without guidelines: " + case_name)
V tools/goal.py:3399 cleanup_violation(scratch_path, "queries-fixtures", "case tree without one guideline: " + case_name)
V tools/goal.py:3402 cleanup_violation(scratch_path, "queries-fixtures", "case tree without one guideline: " + case_name)
V tools/goal.py:3404 cleanup_violation(scratch_path, "queries-fixtures", "case tree without one guideline: " + case_name)
V tools/goal.py:3409 cleanup_violation(scratch_path, "queries-fixtures", "status " + str(result.returncode) + " for case: " + case_name)
V tools/goal.py:3411 cleanup_violation(scratch_path, "queries-fixtures", "non-empty stderr for case: " + case_name)
V tools/goal.py:3415 cleanup_violation(scratch_path, "queries-fixtures", "stdout differs from expect pin for case: " + case_name)
V tools/goal.py:3419 cleanup_violation(scratch_path, "queries-fixtures", "status " + str(result.returncode) + " for case: " + case_name)
V tools/goal.py:3421 cleanup_violation(scratch_path, "queries-fixtures", "non-empty stderr for case: " + case_name)
V tools/goal.py:3425 cleanup_violation(scratch_path, "queries-fixtures", "stdout differs from golden for case: " + case_name)
V tools/goal.py:3442 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/answers-golden/" + pin_name)
V tools/goal.py:3444 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/answers-golden/" + pin_name)
V tools/goal.py:3446 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/answers-golden/" + pin_name)
V tools/goal.py:3449 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/answers-golden/" + pin_name)
V tools/goal.py:3453 cleanup_violation(scratch_path, "queries-fixtures", "answers-golden qid has no query: " + case_name + "/" + pin_qid)
V tools/goal.py:3456 cleanup_violation(scratch_path, "queries-fixtures", "answers-golden qid has no query: " + case_name + "/" + pin_qid)
V tools/goal.py:3460 cleanup_violation(scratch_path, "queries-fixtures", "answer bytes differ from answers-golden: " + case_name + "/" + pin_qid)
V tools/goal.py:3466 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/traces-golden/" + pin_name)
V tools/goal.py:3468 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/traces-golden/" + pin_name)
V tools/goal.py:3470 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/traces-golden/" + pin_name)
V tools/goal.py:3473 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/traces-golden/" + pin_name)
V tools/goal.py:3477 cleanup_violation(scratch_path, "queries-fixtures", "traces-golden qid has no query: " + case_name + "/" + pin_qid)
V tools/goal.py:3480 cleanup_violation(scratch_path, "queries-fixtures", "traces-golden qid has no query: " + case_name + "/" + pin_qid)
V tools/goal.py:3483 cleanup_violation(scratch_path, "queries-fixtures", "traces-golden qid has no query: " + case_name + "/" + pin_qid)
V tools/goal.py:3487 cleanup_violation(scratch_path, "queries-fixtures", "trace bytes differ from traces-golden: " + case_name + "/" + pin_qid)
V tools/goal.py:3495 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
V tools/goal.py:3497 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
V tools/goal.py:3502 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
V tools/goal.py:3506 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
V tools/goal.py:3511 cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
V tools/goal.py:3516 cleanup_violation(scratch_path, "queries-fixtures", "trace-reject member without partner: " + case_name + "/" + reject_qid + ".answers")
V tools/goal.py:3520 cleanup_violation(scratch_path, "queries-fixtures", "trace-reject member without partner: " + case_name + "/" + reject_qid + ".expect")
V tools/goal.py:3525 cleanup_violation(scratch_path, "queries-fixtures", "trace-reject qid has no query: " + case_name + "/" + reject_qid)
V tools/goal.py:3528 cleanup_violation(scratch_path, "queries-fixtures", "trace-reject qid has no query: " + case_name + "/" + reject_qid)
V tools/goal.py:3535 cleanup_violation(scratch_path, "queries-fixtures", "wall_clock for trace-reject: " + case_name + "/" + reject_qid)
V tools/goal.py:3537 cleanup_violation(scratch_path, "queries-fixtures", "trace-reject status " + str(reject_result.returncode) + " for case: " + case_name + "/" + reject_qid)
V tools/goal.py:3539 cleanup_violation(scratch_path, "queries-fixtures", "trace-reject stdout not empty for case: " + case_name + "/" + reject_qid)
V tools/goal.py:3543 cleanup_violation(scratch_path, "queries-fixtures", "trace-reject stderr differs from expect: " + case_name + "/" + reject_qid)
V tools/goal.py:3547 cleanup_violation(scratch_path, "queries-fixtures", "missing required case: red/" + required_name)
V tools/goal.py:3551 cleanup_violation(scratch_path, "queries-fixtures", "missing required case: green/" + required_name)
V tools/goal.py:3553 cleanup_violation(scratch_path, "queries-fixtures", "no red fixture cases found: " + str(fixtures_root))
V tools/goal.py:3555 cleanup_violation(scratch_path, "queries-fixtures", "no green fixture cases found: " + str(fixtures_root))
V tools/goal.py:3558 cleanup_violation(scratch_path, "queries-fixtures", "golden-pin inventory drifted from pinned 40-entry map: " + str(len(pin_inventory)) + " pins on disk")
V tools/goal.py:3560 cleanup_violation(scratch_path, "queries-fixtures", "red case count drift: expected 24 got " + str(red_count))
V tools/goal.py:3562 cleanup_violation(scratch_path, "queries-fixtures", "green case count drift: expected 11 got " + str(green_count))
M tools/goal.py:3564 print("goal: queries fixtures ok " + str(red_count) + " red " + str(green_count) + " green")
```

### check_trace_nonfinite_probe

```text
V tools/goal.py:3573 cleanup_violation(scratch_path, "trace-nonfinite", "probe template lost its result(bogus) anchor")
V tools/goal.py:3582 cleanup_violation(scratch_path, "trace-nonfinite", "wall_clock for non-finite float probe")
V tools/goal.py:3584 cleanup_violation(scratch_path, "trace-nonfinite", "status " + str(probe_result.returncode) + " for non-finite float probe")
V tools/goal.py:3586 cleanup_violation(scratch_path, "trace-nonfinite", "non-empty stdout for non-finite float probe")
V tools/goal.py:3590 cleanup_violation(scratch_path, "trace-nonfinite", "stderr differs from pinned trace_unserializable line")
```

### check_corpus

```text
V tools/goal.py:4211 violation("projection-ledger", "row for unknown docid: " + ledger_docid)
V tools/goal.py:4215 violation("projection-ledger", "docid missing projection row: " + docid)
V tools/goal.py:4228 violation("projection-coverage", "projection row names no coverage region: " + ledger_docid + " " + ledger_region)
V tools/goal.py:4230 violation("projection-coverage", "coverage region " + ledger_region + " does not carry ace(" + ledger_docid + "): " + actual_status)
```

### check_docid_grammar_probe

```text
V tools/goal.py:4243 violation("docid-grammar", "250-byte docid rejected")
V tools/goal.py:4245 violation("docid-grammar", "251-byte docid accepted")
```

### check_trace_numeric_probe

```text
V tools/goal.py:4267 violation("trace-numeric", "5000-digit integer payload rejected by trace parser")
V tools/goal.py:4273 violation("trace-numeric", "10-digit sentence ordinal accepted by trace parser")
V tools/goal.py:4279 violation("trace-numeric", "zero-padded sentence ordinal accepted by trace parser")
```

### check_swipl_wall_probe

```text
V tools/goal.py:4286 violation("swipl-timeout-probe", "sleeper exited under the wall clock")
V tools/goal.py:4291 violation("swipl-timeout-probe", "descendant-held-pipe sleeper exited under the wall clock")
V tools/goal.py:4294 violation("swipl-timeout-probe", "wall-clock probes overran 30s: descendant pipes survive the kill")
```

### check_command

```text
F tools/goal.py:4593 fail("guidelines", "guidelines directory is a symlink")
F tools/goal.py:4595 fail("guidelines", "missing guidelines directory")
V tools/goal.py:4600 violation("guideline-entry", "symlink: " + entry_name)
V tools/goal.py:4602 violation("guideline-entry", "not a directory: " + entry_name)
V tools/goal.py:4604 violation("guideline-entry", "invalid guideline id: " + entry_name)
V tools/goal.py:4615 violation("guidelines", "no guideline directories")
M tools/goal.py:4650 print("goal: check ok " + str(guideline_count) + " guidelines " + str(document_count) + " documents " + str(probe_count) + " red probes")
```
