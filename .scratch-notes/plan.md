# M5.3 shell working plan

Dispatch = prod-m5u3s-1; unit total = 20 section groups; batch ≤15 tool calls, compilable groups committed. Contract read-and-conform-to = .agent/contracts/m5u3.md. Oracle = tools/goal.py.

| order / section | state source | oracle anchor | status | parity evidence |
|---|---|---|---|---|
| 1. fork notices / pristine / vendor | W+I+C | goal.py:522–639 | implemented | baseline + unknown-license exact |
| 2. docid probe | P | goal.py:4235 | implemented | same 250/251 boundary probe |
| 3. trace-numeric probe | P | goal.py:4246 | blocked K3 | explicit stderr rc2 trace seam pending |
| 4. swipl wall probe | P | goal.py:4280 | implemented | process-group wall + descendant pipes; runtime probe pending |
| 5. adjudication fixtures | W+P | goal.py:3228 | composed | K4 parse/ledger bindings; runtime blocked stubs |
| 6. compendium orgs / rows | W | goal.py:2044–2259 | implemented | baseline + header mutant exact |
| 7. source record / guideline / pl / alignment inventories | W | goal.py:161–257 | implemented | baseline + missing README exact; sorted derived names |
| 8. red inventory | W | goal.py:1899 | implemented | baseline + orphan ulex exact |
| 9. Prolog INDEX inventory incl drs_driver | I+W | goal.py:423 | implemented | indexed baseline exact incl drs_driver |
| 10. projection ledger | W | goal.py:2260 | implemented | baseline + missing LF exact |
| 11. product vocabulary | W | goal.py:2785 | implemented | baseline + unauthorized functor exact |
| 12. coverage + payloads (kernel) | W | goal.py:2452 | composed | EFileSrc first-reference table + display root |
| 13. census map | W | goal.py:2300 | implemented | baseline + invalid census key exact |
| 14. adjudication + review manifest + ledger + commit custody (kernel/git) | W+C | goal.py:2822–3227 | composed; R68 adaptation pending | manifest prefix self-hash before grammar failure; git current/historical custody |
| 15. lexicon (kernel) | W | goal.py:2721 | composed | ulex/clex/ACE sources + shadow inventory -> kernel |
| 16. pinned SWI + stage | W+P | goal.py:48–160 | implemented | 9.2.9 probe + exact compiler argv + APE staging |
| 17. documents deterministic compile / proof / load + K1/K2 | W+P | goal.py:641–737 | implemented | double compile/proof, freshness, K1 load, K2 aggregate/recursion |
| 18. query fixtures + K2/K3 seams | W+P | goal.py:3306–3614 | implemented around K3 seam | 24R/11G inventory + 40 pins; K3 generation/inspection pending |
| 19. ACE red probes | W+P | goal.py:1950 | implemented | compiler class/rc/one-LF/exact expect |
| 20. final meter | W+P | goal.py:4650 | implemented | emits only after every section succeeds |

Exclusions = strict (M5.7), UI/copy (M5.5), dist (M5.6). Dispatcher = goal.py:4582–4657.

Exit law = violation → escaped detail + stdout LF, rc1; fail → escaped detail + stderr LF, rc2; relay_failure → exact child stderr + child rc. Escaping = LF→\n, CR→\r.

Trace seam = explicit `goal: queries-fixtures: trace seam pending` stderr rc2 until the K3 binding lands; no silent green. Kernel sections use bindings only.

Gates = cargo build --release --locked --offline; cargo clippy --workspace --locked --offline --all-targets -q -- -D warnings; verusfmt --edition 2024 --check ckc/src/*.rs ckc/src/check/*.rs ckc/tests/*.rs. Primary parity harness owns full runtime check.

Checkpoint eee1df1: build/fmt passed; its commit body prematurely claimed probes because shell continued after the scratch driver failed to compile (sha2 LowerHex). Corrected driver + rerun on eee1df1: 6/7 exact; guideline inventory rejected valid corpus because mapped filenames were not re-sorted. 2acb40d also overstated parity after a failed assertion; committed correction follows actual evidence.

Executed probe rerun after filename-order fix = 7/7 exact stdout/stderr/rc, section_probes_rc=0; all baseline silent sections compared with a positive-control fork meter and matching red failures.

Structural checkpoint: build + clippy -D warnings + verusfmt green; compendium/projection/census/vocabulary probes = 8/8 exact. Text law pins Unicode 15.1 full casefold (1530 mappings) + 68 decimal blocks; Rust lowercase is not Python casefold.

Composition checkpoint: real cargo build blocked E0432 (ckc_kernel exports no EBundle/ECoverage/EDecision/EFileSrc/EVerdict). A scratch type-only adapter reexports already-public ckc_spec values beside existing ckc_kernel exports; direct rustc -Dwarnings + clippy-driver -Dwarnings pass over complete shell. This is not a cargo/workspace gate. Adapter check prints exact fork meter then explicit trace seam pending stderr rc2. Kernel implementation untouched/unread.
