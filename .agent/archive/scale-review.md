# Scale review (IMP orientation; user request: "review the system for other areas that will need to be revised in order to scale")

Base 9129d719 (one guideline: CDC = 492 documents, 15,577 rule clauses, 3,268 facts, 9.6 MB compiled, 927 coverage regions, 6 questions). Check set SC1–SC8 (`scale-review/brief.md`): superlinear time, memory + whole-corpus materialization, input-driven recursion, repeated work, gate/CI wall time (GitHub 6 h/job), per-guideline checks whose property must hold per combination, human throughput, storage limits. Slices (verdict tables + anchors + measurements): `scale-review/{kernel,check,certify,ui,release,semantics,process}.md` = reviewer-1..7 (reviewer, gpt-6.1-sol); probes stayed in the session's `.scratch/agents/imp/scale-*/` (local). No correctness finding against the committed specs; every item = growth cost or a property not closed under union. Projections = linear extrapolation from one guideline unless marked measured.

Breaking point → item (evidence; fix). Placement proposed to the user: P = this phase; A = "multi-guideline readiness" (mechanical, before guideline 2); B = "clinical interoperability + combinations" (direction first, before guideline 2; lands schema-2 meaning changes once, after A's custody decoupling); C = "×100 readiness" (before dozens of guidelines).

×1 (today)
1. P — bounded SLD cannot finish a patient enumeration (characterization K4: open should over CDC + dose world → `indeterminate(limit)`, 28 s).
2. A — formula-cycle scan bounded at depth 7: a 7-edge cycle passes, a 6-edge cycle rejects (check S9, measured) → complete cycle search.
3. A — `align_impl::split_at_seps` recursion per character: a valid 2-row alignment with 230,000-character spans aborts under 64 MiB (ui S3, rc 134) = deferred row "Alignment split recursion".
4. P — every per-document step starts SWI + reloads raw Clex (996 starts/guideline); the test-stage QLF cache: DRS dump 2.83 → 0.43 s, compile 2.89 → 0.53 s, byte-identical outputs (certify S41–S52; one small fixture, not a corpus forecast) → gate-speed unit.

Before guideline 2
5. B — per-guideline green ≠ combination valid (reproduced; singleton controls green): aggregate union `obligation_failed` when guideline B's `Every man sleeps.` defeats A's `does not provably sleep` (kernel S12, semantics F2); derived-entity + formula-cycle scans 0/0 → union 1 (check S8/S9); recursion-check `scope_condition` on the union only (kernel S19, check S22 after the producer-shaped fixture fix, semantics). NAF makes some unions legitimately red → an explicit combination contract (what a consumer checks on its set + one patient) + per-combination commands + cross-guideline fixtures.
6. P — entity-minting rules defeat tabling: two noun-conversion rules pass left-recursion alone + union; a tabled query fills SWI's 32 MB table cap in 0.11 s (semantics F4) → finite-term admission in the new engine.
7. A — one-guideline test oracles: consumer_recipe requires 0 should/must/may across all guidelines for CDC-excluded worlds (a real fork-compiled `Every patient should sleep.` breaks it: process P4, semantics F5); compares CDC's answer pins against the all-guideline composition (a document adding an answer → `differs`, semantics); `tests/da2/preservation.rs:332` asserts exactly 6 committed questions (release S45/S46).
8. A — review custody: a schema-identifier-only migration keeps 492/492 ACE + passage + coverage digests and changes 492/492 review digests (semantics F6); emission-only changes stale the documents they touch (82cff19d: 85/382) = deferred row "Review custody decoupling".
9. B — no shared clinical concept identity: nouns are opaque atoms matched exactly; each guideline's lexicon maps words alone (`Every cohort-patient sleeps.` → `patient` or `adult` under two admissible ulex files; semantics F1); patient-fact vocabulary unspecified → a user-ruled concept contract (codes such as SNOMED CT/RxNorm/LOINC = a CDS-side option, Intent excludes building the CDS).
10. B — strength schema holds GRADE values only (strength strong|conditional, evidence high..very_low; Decisions Strength class) → USPSTF A–D/I, ACC/AHA class + level need a schema revision (identifier 2) or a mapping (MAIN).
11. B — no model of a whole guideline replaced by a new edition; supersession is per statement (MAIN).
12. B — guidelines sharing vocabulary feed each other's derivations: SWI recipe over 1/2/4 fully overlapping CDC copies (adult world): should boxes 289/614/1,372, enumeration 0.14/0.54/2.21 s (×4 per doubling), loading 4.8/7.2/14.4 s; the 120 s goal cap at ≈30 copies (semantics F7; worst case, not a forecast) → which cross-guideline derivations are intended = a semantic ruling; bears on 6.

≈3 guidelines
13. A — consumer_recipe, sgw P1 + bf_consumers pass the whole corpus as one `swipl -g consult([...])` argument: 44,118 B today, ×3 = 132,217 B → exec E2BIG (release S45, reproduced) → load through a file list.

≈19–60 guidelines (6 h CI job)
14. A — check job ≈1,150 s per CDC-sized guideline → 6 h at ≈19, ×100 ≈32 h (check S19); certify 417 s/guideline + 132 s fixed → 6 h at ≈52, ×100 11.6 h (certify S4/S36, CI run 41022878056); fresh compile + queries → ≈58 (release); corpus rounds run the whole-repo check twice → Σ over G = Θ(G²) (process P1) → content-keyed incremental gates (changed documents + dependents), shards per guideline, a separate combination gate; parallel authoring ownership = a user ruling.
15. A — review history (hidden by today's empty ledger): 2 git children per ledger row + a whole-guideline archive + bundle derivation per distinct reviewed commit, every check (check S13, ui S8, process P2; at one commit per document ≈6 h near 137 guidelines) → cached attestations, batched ancestry.
16. A — one ledger per guideline under a whole-ledger CAS: two POSTs for different documents → 303 then 409 (ui S6) → per-document conflict scope.
17. A — nested parallel maps run W² processes (check S16, reproduced) → one shared budget.

×100
18. C — whole-corpus load per call: `ckc v1` ×1/×2/×4 0.72/1.52/3.06 s, 167/329/654 MB (~17× source bytes; ≈17 GB at ×100); every reviewer-site request snapshots + loads every guideline (ui S1); `ui check` peak 120/229/448 MiB → ≈11 GiB at ×100, ≈110 GiB at ×1,000 (ui S9) → indexed on-disk image + selective reading; prepared per-page models.
19. C — kernel algorithms: manifest dedup O(D²), insertion sort + front-inserting dedup, coverage membership scans, every obligation deep-copying all clause bodies (synthetic 2,048/4,096/8,192 documents, one fact + one obligation each: 0.66/2.31/12.07 s), K3 linear clause scans, N² log lookup, N × C custody joins, double parse/hash in trace modes (kernel S1–S10) → hashing, merge sort, sharing; each re-proved.
20. C — release: `release_rows::sorted` insertion Θ(A²) (pure sorted digests 1,525/3,050/6,100 members = 2.1/6.6/28.2 ms), profile lookups Θ(A·G), whole tar + gzip buffered, the all-guideline bag built twice per check (release S1–S13, check S14/S15) → streaming, per-guideline partitions.
21. C — reviewer site: page titles Θ(D·R) per title → render Θ(G·D²·R) (ui S4); Word meanings re-parse every displayed document (`k5_exclusion.rs:187–232`); `just mobile` = 3 serial full passes (ui S10).
22. C — one whole-KB release line: one rejected document blocks every ready guideline (process P5) → per-guideline release readiness (a user ruling).

≈449 guidelines
23. C — tracked `release-manifest.tsv` 233,737 B → GitHub's 100 MiB blob limit (release S8) → one manifest per guideline.

×1,000
24. C — compiled Prolog ≈9.6 GB in git; 8 HEAD-cloning suites ≈103 GB scratch; release archive ≈1.93 GB vs GitHub's 2 GiB asset limit (narrow), Zenodo 50 GB/record (release S24/S26) → storage plan.
25. C — compendium = 847 eligible or pending guidelines, 416 organizations unswept → committed harvest/refresh tooling (process P6).

Kept as is: agent-law + spec + compendium validators (linear, small); Verus time (code-only); rd/rev-k page pins (fixed fixtures).
