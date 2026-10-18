# Review ledger

Adjudication of judgment-bearing rows (`CLAUDE.md` `Engineering`: check set fixed before the diff is read; every row adjudicated; an accepted ruling holds until new evidence reverses it). Rows: `id | lens | check | verdict | evidence`. Verdicts: `open` → `pass` | `fail→fixed <sha>` | `register` (outside the acceptance contract, noted).

Closed ledgers: `.agent/archive/review-implement.md` (M5 review R-01..R-14, IMPLEMENT close S-01..S-10, U2 C-01..C-04, M5.5 close U5-01..U5-05 — every row adjudicated).

## eso (IMPLEMENT — engine search order) closing review

| id | lens | check | verdict | evidence |
|---|---|---|---|---|
| E-01 | correctness/spec | C1–C5: spec D1–D4 exact; K2 + K3 exec = spec; k3_sound meaning kept; tests assert the contract (reviewer-18) | pass | `.scratch/agents/reviewer-18.md`: 119/119 eso rows + restored CDC exact, 21/21 trace pins re-derived, contract.rs byte-identical |
| E-02 | claim soundness | S1: spec Decisions `Search order` dropped D2's both-finite + bound scope (reviewer-19 F1, reviewer-20 G1-F1, reviewer-22 F22-1) | fail→fixed cfd6649a | reviewer-19 safe-bound probe (seed limit, target solutions); Decisions now scope answer sets to searches that end in both orders, name termination/bound trips/first proofs + the not-machine-verified status |
| E-03 | claim soundness | S1: trace.rs header listed only the q7/q8 exceptions to legacy trace law (reviewer-19 F2) | fail→fixed cfd6649a | eso-g-001 seed/target trace differ; header names the eso selection |
| E-04 | claim soundness | S2: contract + seed body presented the 100000 budget as a counted transition total (reviewer-19 F3) | fail→fixed cfd6649a | spike counter 99818; contract Defect line reworded; squash body states the budget |
| E-05 | guarantee-vs-claim | G2 trusted-surface growth (+93 spec lines traced), G3 acceptance held by `eso_acceptance` + unchanged corpus gates (reviewer-20) | pass | `.scratch/agents/reviewer-20.md` |
| E-06 | verification integrity | V1: no escape hatch; trust delta = the `#[path]` row; two new `step_selected` fns carry rlimit(5000) like their K2 siblings, no existing cap changed (reviewer-21 F2 closed on that scope) | pass | `.scratch/agents/reviewer-21.md` |
| E-07 | verification integrity | V2: green-both eso guards lacked mutation credit over their shell callees (reviewer-21 F1, major) | fail→fixed cfd6649a | mutants-1 (v1_cli.rs) + mutants-2 (main.rs, trust.rs) runs + survivor rulings in the squash body |
| E-08 | verification integrity | V3 grading checks unchanged + pins byte-identical; V4 expectations independent of the implementation | pass | `.scratch/agents/reviewer-21.md` |
| E-09 | conformance | K1 registers, K2 code hygiene, K4 review timing (closing reviewers precede the main landing) | pass | `.scratch/agents/reviewer-22.md` |
| E-10 | conformance | K4: contract Tier promised a red-stub seed; seed 7550c0cb kept the base kernel for the blind tester (reviewer-22 F22-2) | fail→fixed cfd6649a | Tier text states the seed as built |
| E-11 | security (MAIN) | untrusted input → sinks in remotely reachable code | pass | `ckc ui serve` never reaches the engine (`v1_answer` callers = `v1_cli.rs` only); engine inputs = local files named by CLI args (trusted); `binds_root` iterative (term depth cannot exhaust the stack) |
