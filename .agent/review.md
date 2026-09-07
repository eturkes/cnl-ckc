# Review ledger

Adjudication of judgment-bearing rows (`CLAUDE.md` Engineering: check set fixed before the diff is read; every row adjudicated; an accepted ruling holds until new evidence reverses it). Rows: `id | lens | check | verdict | evidence`. Verdicts: `open` → `pass` | `fail→fixed <sha>` | `register` (outside the acceptance contract, noted).

## M5 review (spine close; check set fixed at IMPLEMENT open, before any unit diff was read)

id | lens | check | verdict | evidence
--- | --- | --- | --- | ---
R-01 | spec audit | every `ckc-spec` line traces to a soundness/custody claim (Decisions: verification line); redundant layer / duplicate representation / lemma provable from a neighbour = finding | open |
R-02 | spec audit | spec readable end-to-end by one human pass: size meter (`ckc trust-audit` spec=) + per-module purpose comment present; module list = K1 term/v1text/digest, K2 engine/replay/answers, K3 trace, align | open |
R-03 | spec audit | spec ↔ REFERENCE prose agreement on every normative sentence of § Compiled Prolog schema, § Query answers, § Proof traces (row per sentence in the evidence file) | open |
R-04 | claim soundness | README/REFERENCE certification wording = "machine-verified against the committed spec under a pinned verifier TCB"; no "proved correct"/"foundational" claim | open |
R-05 | claim soundness | trusted-surface enumeration complete: every file a certifier must read is in `spec-manifest.tsv`; shell tier enumerated in REFERENCE; deps = `deps-allowlist.tsv` exactly | open |
R-06 | guarantee vs claim | `--no-cheating` actually rejects a planted `assume`/`admit`/`external_body`/`assume_specification` in each kernel crate (4 plants, each killed) | open |
R-07 | guarantee vs claim | trust-audit hostile probes: comment-split token, `#[path]` include, `include_str!`, added Cargo member, `[patch]` source swap, dep version drift — each rejected (replay `.scratch/m5u1/trust-battery/run.py` + new plants for K2/K3 files) | open |
R-08 | guarantee vs claim | mutation campaign scored by verus-acceptance over K2/K3 spec-bound exec fns: ≥20 semantic mutants per kernel unit; a surviving verified mutant = spec gap → spec fix + re-run | open |
R-09 | correctness | differential replay: K1 lanes A/H 349/349; K2 corpus 3 lanes byte-identical; K3 lanes D/E byte-identical to legacy + committed artifacts; tests/queries replay; red suite 0 failures | open |
R-10 | correctness | shell fault probes: unreadable/FIFO/symlink/invalid-UTF-8 manifest cells, zero-byte + oversized files, argv shapes → pinned envelopes (R9/R18/R24) | open |
R-11 | CLAUDE.md conformance | kernel impl never read by MAIN (transcript grep for `ckc-kernel/src/*_impl.rs|k2_*.rs|k3_*.rs` Read/cat = 0 hits outside contract.rs) | open |
R-12 | CLAUDE.md conformance | every unit landed with contract-before-code (`.agent/archive/contracts/`), one scoped commit per unit, gates green at each commit (CI history) | open |
R-13 | correctness | perf acceptance (m5u2a R31) holds at close on the committed binary: 3 corpus lanes ≤30 s / ≤2 GB; 10 R30d cases ≤30 s | open |
R-14 | claim soundness | K3 `k3_sound` theorem statement audited: `forest_valid` = the intended `proved ⇒ derivable` (resolution + NAF finite-failure certificate), no vacuous precondition | open |
