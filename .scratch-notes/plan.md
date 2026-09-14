# M5.3 shell working plan

Dispatch = prod-m5u3s-1; unit total = 20 section groups; batch ≤15 tool calls, compilable groups committed. Contract read-and-conform-to = .agent/contracts/m5u3.md. Oracle = tools/goal.py.

| order / section | state source | oracle anchor | status | parity evidence |
|---|---|---|---|---|
| 1. fork notices / pristine / vendor | W+I+C | goal.py:522–639 | unknown | unknown |
| 2. docid probe | P | goal.py:4235 | unknown | unknown |
| 3. trace-numeric probe | P | goal.py:4246 | unknown | unknown |
| 4. swipl wall probe | P | goal.py:4280 | unknown | unknown |
| 5. adjudication fixtures | W+P | goal.py:3228 | unknown | unknown |
| 6. compendium orgs / rows | W | goal.py:2044–2259 | unknown | unknown |
| 7. source record / guideline / pl / alignment inventories | W | goal.py:161–257 | unknown | unknown |
| 8. red inventory | W | goal.py:1899 | unknown | unknown |
| 9. Prolog INDEX inventory incl drs_driver | I+W | goal.py:423 | unknown | unknown |
| 10. projection ledger | W | goal.py:2260 | unknown | unknown |
| 11. product vocabulary | W | goal.py:2785 | unknown | unknown |
| 12. coverage + payloads (kernel) | W | goal.py:2452 | unknown | unknown |
| 13. census map | W | goal.py:2300 | unknown | unknown |
| 14. adjudication + review manifest + ledger + commit custody (kernel/git) | W+C | goal.py:2822–3227 | unknown | unknown |
| 15. lexicon (kernel) | W | goal.py:2721 | unknown | unknown |
| 16. pinned SWI + stage | W+P | goal.py:48–160 | unknown | unknown |
| 17. documents deterministic compile / proof / load + K1/K2 | W+P | goal.py:641–737 | unknown | unknown |
| 18. query fixtures + K2/K3 seams | W+P | goal.py:3306–3614 | unknown | unknown |
| 19. ACE red probes | W+P | goal.py:1950 | unknown | unknown |
| 20. final meter | W+P | goal.py:4650 | unknown | unknown |

Exclusions = strict (M5.7), UI/copy (M5.5), dist (M5.6). Dispatcher = goal.py:4582–4657.

Exit law = violation → escaped detail + stdout LF, rc1; fail → escaped detail + stderr LF, rc2; relay_failure → exact child stderr + child rc. Escaping = LF→\n, CR→\r.

Trace seam = explicit `goal: queries-fixtures: trace seam pending` stderr rc2 until the K3 binding lands; no silent green. Kernel sections use bindings only.

Gates = cargo build --release --locked --offline; cargo clippy --workspace --locked --offline --all-targets -q -- -D warnings; verusfmt --edition 2024 --check ckc/src/*.rs ckc/src/check/*.rs ckc/tests/*.rs. Primary parity harness owns full runtime check.
