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

## 342dc2a6..close

- [x] dba75f60 Q14 guideline page "Time words" section: the `temporal.tsv` words the compiler reads as units, relations or spacing frames (K5); contract `.agent/archive/contracts/q14.md`.
- [x] 68bfba68 Q12 schema v3: guideline_frequency/5, guideline_order/4, guideline_recurrence_window/7 compiled, certified, queried and rendered, guideline_range/3 reserved (user rulings: counts per period, recurrence window, unquantified order); parse_doc_declarations rebuilt pair by pair (the Verus rlimit-count crash); contract `.agent/archive/contracts/q12.md`.
- [x] 68bafb80 Q13 comparison `about` + guideline_range/3 ranged minimum (user ruling: no tolerance); 5 CDC documents re-authored; contract `.agent/archive/contracts/q13.md`.
- [x] 4a12806a Q15 R9 offsets at answers/traces version atoms; contract `.agent/archive/contracts/q15.md`.
- [x] dd631018 Q16 rust.md binding rule names the six `cells_ok` bindings as the ruled exception; contract `.agent/archive/contracts/q16.md`.
- [x] e6287a30 Q17 R9 offsets at quoted version atoms; contract `.agent/archive/contracts/q17.md`.
- [x] a8cd22a8 Q18 R9 offsets at every fixed-name slot (record names + document predicate names, misspelled or quoted); contract `.agent/archive/contracts/q18.md`.

## 89787a1d..close

- [x] 67fcdcd0 N9 pin drift outside the Verus family: rustup-init 1.29.1, verusfmt 0.7.4, cfg-if, hybrid-array, indexmap, libc, unicode-ident (both locks + vendor); contract `.agent/archive/contracts/n9.md`.
- [x] f0b8b313 N10 `just outdated` reports semver-held crates as `held`; only fixable drift reds the weekly job (user ruling: Verus bumped every stable release); contract `.agent/archive/contracts/n10.md`.

## fddc538d..close

- [x] c7e77431 N12 resume-note prefix: the paused run's `RESUME (paused 2026-10-12)` row renamed to `RESUME: …` (refresh find).
- [x] d4bfc7eb N11 cargo-mutants 27.1.0 pinned + `just mutants`; align_suite credited (59 mutants: 32 caught, 27 missed); contract `.agent/archive/contracts/n11.md`.
- [x] bec21066 N13 `just bump-tool <tool> <version>`: a `tools.lock` row from the release; every current row regenerates byte-identical; contract `.agent/archive/contracts/n13.md`.
- [x] de45ac41 N14 recipe checks in `just test` (`rust/ckc/tests/recipes.rs`, planted pre-fix forms); contract `.agent/archive/contracts/n14.md`.
- [x] c2b983d7 N15 unicode-ident 1.0.27; contract `.agent/archive/contracts/n15.md`.
- [x] c047d426 N16 align_suite U+202F + U+206A allowed cases (N11 survivors killed); contract `.agent/archive/contracts/n16.md`.
- [x] 1e3932fb N18 own `=` pins follow upstream: `outdated` drift `pinned by`, `bump-crate` moves the pin (user ruling); contract `.agent/archive/contracts/n18.md`.
- [x] 73fc12c0 N19 sha2 0.11.1 (the trust battery's p20 plant reads the locked version); contract `.agent/archive/contracts/n19.md`.
- [x] d9584b2e N20 recipes.rs fixture roots leave no scratch entry; contract `.agent/archive/contracts/n20.md`.
- Open at close: Q1 Kani (waits on the next `rust/kani.lock` bump), N17 Verus issues filed 2026-10-12/13 (waits on a fixing release), N8 Verus interpreter fixes (waits on the first stable release holding all four; its state + next action = its Tasks row), Q5 M7 strength + Q6 corpus rounds (owed: own sessions, user rulings).

## 8681e3a9..close

- [x] 5ae7878d N21 align_suite survivor rulings: reviewer-1 ruled S08 specified → operand-count usage case; 24 open; contract `.agent/archive/contracts/n21.md`.
- [x] 7fe4245f N22 viewport meta + narrow-screen table rule (user ruling: meta + table scroll box); 0 of 348 pages scroll sideways at 390 px; contract `.agent/archive/contracts/n22.md`.
- [x] fe361e4a N23 separators replaced (user ruling: section-path mark ` > `, spaced nav + provenance, `Not covered: `, bare titles); contract `.agent/archive/contracts/n23.md`.
- [x] d3a4df7d N24 `just check` speedup at equal strength: per-item SWI work on `CKC_JOBS` workers, `check` 3221 s → 1267 s; contract `.agent/archive/contracts/n24.md`.
- [x] bab0e4ad N25 `ckc certify-one` fixture test (user ruling: keep + fixture test); contract `.agent/archive/contracts/n25.md`.
- Open at close: Q1 Kani (waits on the next `rust/kani.lock` bump; Kani latest = the pinned 0.68.0 on 2026-10-15), N17 Verus issues (waits on a fixing stable release; latest stable = 0.2026.10.04, older than the issues), N8 Verus interpreter fixes (waits on the first stable release holding all four; the 2026-10-12 cut failed, latest stable 0.2026.10.04), Q6 corpus rounds (own sessions). Routed: Q5 M7 strength → IMPLEMENT (user ruling).
