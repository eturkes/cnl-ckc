# Deferral-queue runs — completed task rows (archived at each run's close)

## 96b5bed2..4db0f495

- [x] f0e89ff2 Q2 `ace_commit` custody content check (user ruling: recorded commit = the viewed commit).
- [x] 0ea7c076 Q3 non-v1 tests/queries fixtures → canonical v1 (10/10 rewritten); `tests/check/r79-nonv1.tsv` header only.
- [x] fed5db86 Q4 `.agent/spec.md` shape check in `ckc check`.
- [x] 0844a975 Q7 K3 negation certificate = call-time instance; calls failing `naf_safe` cut.
- [x] 4dbe27aa Q8 trace clause identity from the owning `% S<n>:` block.
- [x] e3f94d64 Q9 single-authority UI copy.
- [x] 3616a400 Q10 M5.3 late mutants replayed (`just late-mutants`).
- [x] 0d3898cd Q11 corpus.md Inexpressible cite.
- [x] 604924fb N1 `align_probes` stdin race.
- [x] d2307486 N2 Review records the rendered commit: contract `.agent/archive/contracts/n2.md` (user-approved with D7: posted commit = HEAD or an ancestor).

## d38dac91..close

- [x] 6d42b556 N3 `ckc check` scratch kill-safe: `.goal.tmp.<pid>.<n>` + sweep of leftovers whose creator pid is gone.
- [x] 6661e240 N4 release manifest keyed on input content: `meta head` dropped (user-approved O1); contract `.agent/archive/contracts/n4.md`.
- [x] 143bcb8e N5 pid-named temp paths in `ckc certify --cases` + `ckc dist build` step past same-pid leftovers.
- [x] 7b34752c N6 `ui_commit` harness scratch: case roots `rust/target/ui-commit/<pid>-<label>` + child-process probe `green_case_leaves_no_scratch` (user-approved).
- [x] 7323426f N7 `just verify` warning-free + enforced: 10 ckc-kernel Verus warnings cleaned at their source + the verify step fails on any `^warning` line (user-approved R1–R4); contract `.agent/archive/contracts/n7.md`.

## 2acd0d70..close

- [x] 35ad5f12 M7T schema v2 temporal annotations: interval + recurrence annotations compiled, certified, queried and rendered; CDC corpus on v2 (12 re-authored, 8 annotated unchanged, 317 header-only); contract `.agent/archive/contracts/m7t.md`.
- [x] df87ec51 K1-diag R9 reject offsets: every digest site (quote-aware) + annotation search skipping comments and quoted atoms; tests/v1 dq4-k1-001..007.
