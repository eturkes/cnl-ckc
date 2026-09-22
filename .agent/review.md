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

## M5.5 close review (check set fixed before the integration diff `73ffa227..wt/prod-m5u5s` was read)

id | lens | check | verdict | evidence
--- | --- | --- | --- | ---
U5-01 | contract/parity | P3+P5: `.scratch/m5u5/test/parity.py --rust-bin <committed binary>` render 345/345 + requests 153/153 PASS on the merged tip; P4: `cargo test --test ui_fixtures` 113/113 with `rust/ckc/tests/ui_fixtures.rs` byte-identical to wt/test-m5u5 e31700ab; every FC2 re-pin enumerated | open |
U5-02 | POST/CAS | guard order in `rust/ckc/src/ui/request.rs` = R84 chain verbatim (Host → 405 → 500 → GET → 404 → Origin/403 → 400 ×3 → csrf/403 → 500 ×2 → subject/409 → CAS/409 → 500 → 303); refusal writes nothing; ledger write = mkstemp/fsync/flock/rename with digest CAS under the lock; loopback-only bind; per-process token; socket smoke rc 0 | open |
U5-03 | grading integrity | graders unchanged: `sha256sum -c .scratch/m5u5/test/source.SHA256SUMS` rc 0; `tests/ui/**`, `tools/ui.py`, `rust/ckc/tests/ui_fixtures.rs` byte-identical to main 73ffa227 / e31700ab; no gate/threshold/fixture edit in the diff; the one repair (VT byte in `unhex`) carries red-before-fix evidence (`hex-space-red.log` legacy rc 0 / native rc 2 → `hex-space-green.log` 6/6 identical) | open |
U5-04 | corpus/custody | `git diff --stat 73ffa227 <tip> -- guidelines/ tests/ tools/ vendor/` empty; trust meter green; `escape-allowlist.tsv` deltas = shell quoted-data rows + the R89 `from_utf8_unchecked` site only; `deps-allowlist.tsv` byte-identical; `spec-manifest.tsv` pins the merged spec; kernel impl files unread by MAIN + reviewer (transcript grep) | open |
U5-05 | claim accuracy | close commit body + `spec.md` Artifacts/Deferred + archived contract state exactly the checks run (gate steps, parity counts, verifier verified/error counts, rc); theorem wording = "machine-verified against the committed spec under a pinned verifier TCB"; R84 register notes carried into this ledger's register; nothing claimed beyond P1–P7 evidence | open |
