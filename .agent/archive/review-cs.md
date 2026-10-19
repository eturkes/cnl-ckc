# Review ledger — IMPLEMENT M7 gap class condition-scope (closed)

Check sets fixed before each diff was read (unit briefs + `.scratch/cs/close-review-checksets.md`); unit reviews per kernel or data unit, then one reviewer per lens over the phase diff 625091bf..c6b50800 and over the close commit; MAIN's security lens alone. Rows: `id | lens | check | verdict | evidence`. Every row adjudicated; each fix earned one re-review round on its acceptance check. Contract reviews: consultant-1 (CS0/CS1 K1–K10, CS0 Z1–Z5 sound), consultant-2 (CS1b C1–C8 + RR2–RR5; CS1b retired unlanded by user ruling), consultant-3 (CS1b page-proof rlimit V1–V3).

| id | lens | check | verdict | evidence |
|---|---|---|---|---|
| CS0-R1 | correctness | spec frame law: save at entry, absorb on refutation, end incomplete on a cut resume | pass | reviewer-1 |
| CS0-R2 | correctness | kernel refines every Alt construction + consumer | pass | reviewer-1 |
| CS0-R3 | guarantee-vs-claim | no proof crosses an unresolved cut negation into answers, replay or aggregate | pass | reviewer-1 |
| CS0-R4 | claim soundness | REFERENCE § Query answers + schema, queries law, engine header | fail→fixed cc80f406 | F1 replay-overclaim sentence rephrased; re-review pass |
| CS0-R5 | verification integrity | seed-red suite, controls, pins unweakened | pass | tester-1 suite 9 seed-red + 8 credited by mutation (45 mutants: 22 caught, 21 missed ruled) |
| CS0-R6 | conformance | comments, register, dead code | pass | reviewer-1 |
| CS0-G1–G2 | register | queries.md legacy-parity list omitted CS0; stale engine trust pin | fail→fixed cc80f406 | re-review pass |
| CS1-R1 | correctness | spec D1–D6 + D8 | pass | reviewer-2 |
| CS1-R2 | correctness | fork `cs1_*` mirrors the spec | pass | reviewer-2 |
| CS1-R3 | correctness | key law + UI intake logical path; kernel refines | pass | reviewer-2 |
| CS1-R4 | guarantee-vs-claim | D6 guard-scan claim + preconditions; deep exclusion → `indeterminate(limit)` | pass | reviewer-2 |
| CS1-R5 | claim soundness | REFERENCE § Scope exclusions + schema + vocabulary text | pass | reviewer-2 |
| CS1-R6 | verification integrity | suites red at seed, green at tip; migration replayable | pass | reviewer-2 |
| CS1-R7 | conformance | comments, register | pass | reviewer-2 |
| CS1-F1–F3 | minor findings | two exclusion-verb boxes; deontic scan over leaf data; nt1 frozen inputs | fail→fixed 838e4c9d | contract D4 + P8 amended, REFERENCE sentence, pinned case `order-two-exclusion-boxes`; RF1–RF3 pass |
| CS2-D1–D7 | data (L1, L2, L6, align, churn, L4 + L5, queries) | scope re-authoring S3-03, S7-02, S9-01, S10-04 | pass | reviewer-3; aggregate 386 documents / 1,185 obligations rc 0; exclusion query `no(finite_failure)`, patient control `yes` |
| CS2-E1–E6 | data (ruling 4, census, notes, lexicon, align, L4 + L5) | 38 absence rewrites + 11 census rows | fail→fixed c6b50800 | F1–F7 (classical for observed findings + actual status, notes exact per row, S34-07 short course, S29-02 need facts, Recommendation 5 data, S39-16 standard initiation); RF-E1–RF-E7 pass |
| CL-A1–A5 | correctness | spec ↔ kernel ↔ fork; corpus guards + records; rewrites; shell indicators; pitfalls | pass | reviewer-4: 74 boundary probes, 1,151/1,151 guards, 12 records, 501 clauses audited |
| CL-B1 | guarantee-vs-claim | REFERENCE + taxonomy claims vs code | fail→fixed e91c06f3 | reviewer-5 F2: bounds sentence conditioned on the inner search ending without a proof |
| CL-B2–B3 | guarantee-vs-claim | D6 preconditions, one-patient limit, Decisions + Tasks | pass | reviewer-5 |
| CL-B4 | guarantee-vs-claim | commit-body measurements | fail→fixed e91c06f3 | F1 838e4c9d test name, F3 c6b50800 `ui check` claim: corrections recorded; `ckc ui check` rerun rc 0 |
| CL-R1 | register | archived census rulings kept first-pass dispositions | fail→fixed e91c06f3 | 13 rows synced to the final ACE |
| CL-C1–C4 | verification integrity | seed-red evidence, no weakened check, migration replay, gate evidence | pass | reviewer-6 |
| CL-D1 | conformance | registers | pass | reviewer-7 |
| CL-D2 | conformance | comments, rules currency | fail→fixed e91c06f3 | F2 rust.md codec descriptors; sizing bullet ruled live |
| CL-D3 | conformance | spec shape, archives | fail→fixed e91c06f3 | F1 CS2 Tasks tick |
| CB-B1 | guarantee-vs-claim | README scope bullet vs code | fail→fixed in the close commit | reviewer-8 F1: the bounded-search case added; RF-F1 pass |
| CB-B2 | guarantee-vs-claim | Decisions `Review UI` states the user rulings as given | fail→fixed in the close commit | F3: proposals separated from rulings; RF-F3 pass |
| CB-B3–B4 | guarantee-vs-claim | archive records; corpus.md absence classes | pass | reviewer-8 |
| CB-B5 | guarantee-vs-claim | close-commit body measurements | fail→fixed in the close commit | F2: preview checks credited to the pre-repair state (comment/proof-only delta, exec code identical); RF-F2 pass |
| CB-R1 | register | m6-symbols replay drift (CS1 residue, carried by the retired CS1b) | fail→fixed d5ed5805 | regenerated by the committed generator; replay 6/6 identical; re-review pass |
| CB-D1–D4 | conformance | registers, spec shape, rules upkeep, archive hygiene | pass | reviewer-9 |
| SEC-1 | security | key + lemma injection into compiled Prolog | pass | key grammar-bound, quoted atom |
| SEC-2 | security | path handling in key law + UI intake | pass | logical path `guidelines/<gid>/vocabulary.tsv` |
| SEC-3 | security | resource bounds | pass | linear guard scan; deep chains end `indeterminate(limit)` |
| SEC-4 | security | HTML escaping of new UI cells | pass | no UI cell lands (CS1b retired); the retired spec escaped every data cell |
| SEC-5 | security | new network, process or file-write surface | pass | none |
