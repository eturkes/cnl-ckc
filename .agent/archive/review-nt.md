# Review ledger — IMPLEMENT M7 gap class numeric-threshold (closed)

Check sets fixed before the diff was read (`.scratch/agents/nt/review-brief.md`): one reviewer per lens over wt/nt1 5b6fbcfc vs main d8ec1a7c; MAIN's security lens alone. Rows: `id | lens | check | verdict | evidence`. Every row adjudicated; each fix earned one re-review round on its acceptance check.

| id | lens | check | verdict | evidence |
|---|---|---|---|---|
| C1 | correctness | D1 relation: spec, kernel evaluator, schema.pl | pass | reviewer-1: 26,155 native + 26,157 SWI probes rc 0 |
| C2 | correctness | D7 codec: 19 indicators, built-in positions, R9 | pass | F-C2-1 (bare allowed name offset) retired: q18 D3 rules "its start"; tester-4 corrected 10 expectations |
| C3 | correctness | D5 engine: barrier, binds, fail, depth | pass | P4 replay failures = pre-nt1 sentence-identity law (R-C3-1 register → tester-4 R2) |
| C4 | correctness | D6 traces: builtin leaves, check order | pass | reviewer-1 |
| C5 | correctness | D2–D4 + D8 emission + certification | pass | window witness ordinals = base compiler's (R-C5-1 register → tester-5 R5) |
| C6 | correctness | D9 fork mirror + replay schema clauses | pass | 24 fork replay clauses = schema.pl terms |
| C7 | correctness | D10 shell: write, check, carve-out, release, README-dist | fail→fixed 8c8ea322 | F-C7-1 README-dist load line omitted schema.pl; dist_cases a12 red→green; re-review pass |
| C8 | correctness | D11 UI joins + Action words | pass | F-C8-1 (preference Action second participant) retired: D11 clarified 94130ace (preference Action names the verb or a copula subject) |
| C9 | correctness | D12 migration | fail→fixed b69b14cd, 164202d5, 450c1672 | F-C9-1 docstring; F-C9-2 certify mutants left their base occurrence → nt1-certify-repin.py (7 rows) + oracle O2 transport; re-review pass |
| C10 | correctness | docs ↔ behavior | fail→fixed 5067c269 | F-C10-1 question wording (queries.md, compiler.md); re-review pass |
| S1 | claims | value-set inclusion + conservative entailment | pass | reviewer-2 |
| S2 | claims | portability | fail→fixed 5067c269, 8c8ea322 | S2-F1 integer-range caveat; S2-F2 = F-C7-1; re-review pass |
| S3 | claims | corpus behavior | pass | six answer payloads = base bytes |
| S4 | claims | search order | fail→fixed 5067c269, c11202eb | S4-F1 nonground built-in barrier in Decisions + engine.rs overview; re-review pass |
| S5 | claims | verification boundary | fail→fixed 5067c269, bf9e7b1b | S5-F1 trace clause nodes vs comparison leaves (two anchors); re-review pass |
| S6 | claims | taxonomy + README numbers | pass | nt1 counts re-derived; R1 register (pre-nt1 status totals) → fixed in NT2's taxonomy recount |
| S7 | claims | consumer guidance | pass | reviewer-2 |
| S8 | claims | unchanged/identical claims | fail→fixed b69b14cd | S8-F1 = F-C9-1; re-review pass |
| G1 | gaps | relation | pass | reviewer-3: ISO execution of schema.pl = fixture-backed, stated as such |
| G2 | gaps | emission certification | pass | reviewer-3 |
| G3 | gaps | engine + traces | pass | reviewer-3 |
| G4 | gaps | UI | pass | reviewer-3 |
| G5 | gaps | custody + release | fail→fixed 8c8ea322 | G5-F1 = F-C7-1; re-review pass |
| G6 | gaps | trust surface | pass | trust-audit rc 0; escape allowlist unchanged |
| V1 | integrity | red on base + suites as merged | pass | reviewer-4: 21 + 6 files byte-equal to the testers' commits |
| V2 | integrity | already-passing credit | pass | P5 controls: cargo-mutants over inventories.rs + schema.rs, every carve-out + schema::check mutant caught, survivors ruled (d7fbf1cd body); P3 controls: planted fork mutants MP1 MP2 MR1 MR3 killed, MR2 ruled |
| V3 | integrity | frozen suite ports | fail→fixed a7901bc4 | ports 6bdc79e1 (146 tests, 824 frozen files identical); F2 the shared adapter's lines() hid a CR → erase() rejects a CR; re-review pass |
| V4 | integrity | oracle independence | pass | reviewer-4 replay |
| V5 | integrity | nothing weakened | fail→fixed 164202d5 | F1 doc-v2-move swapped other lines; re-review pass |
| V6 | integrity | Verus integrity | pass | the closing gate's verify step = the recorded rc |
| V7 | integrity | contract-derived expectations | pass | reviewer-4 |
| K1 | conformance | register | fail→fixed 5067c269 | K1-F1 review diary out of the contract; re-review pass |
| K2 | conformance | why-comments | pass | reviewer-5 |
| K3 | conformance | rules files | fail→fixed 5067c269 | K3-F1 queries.md; K3-F2 nt1 harnesses + schema-file firing in rust.md; re-review pass |
| K4 | conformance | spec.md | fail→fixed 5067c269, c11202eb | K4-F1 Search order barrier + built-ins bind nothing; re-review pass |
| K5 | conformance | deferred rows | pass | reviewer-5 |
| K6 | conformance | Engineering law | fail→fixed 0d6819c7 | K6-F1 nt1_var/nt1_member → v1_lookup/strict_member, output identical; re-review pass |
| K7 | conformance | hygiene + licenses | register | K7-F1 archives ship NOTICE without LICENSE (pre-existing) → `.agent/deferred.md` row |
| K8 | conformance | commit discipline | register | NT1 lands as one squash commit with the full body |
| SEC-1 | security (MAIN) | file writes in reachable + CLI paths | fail→fixed 8c8ea322 | `ckc compile` wrote guidelines/schema.pl through a planted symlink; schema::write refuses a nonregular target; cli compile_refuses_a_symlinked_schema_file red (rc 0, link target overwritten) → green |
| SEC-2 | security (MAIN) | raw-HTML sinks in the loopback UI | pass | new slots = unit/comparison copy entries + canonical decimals (`unit_html`, `cmp_html`, `udec_bytes`); no corpus byte reaches a page unescaped (K5 render fidelity + copy theorems) |
| SEC-3 | security (MAIN) | path handling in ckc check | pass | schema check reads `symlink_metadata` first; the inventory carve-out admits a regular non-symlink `schema.pl` alone |
| SEC-4 | security (MAIN) | release staging | pass | `git ls-tree` inventory refuses non-regular members; schema.pl ships from its HEAD blob |
| SEC-5 | security (MAIN) | POST guard + ledger CAS | pass | no nt1 change in the POST path |
| SEC-6 | security (MAIN) | Prolog consumer surface | pass | schema.pl guards every public argument (`atom/1`, `integer/1`) before arithmetic; no call of data terms |
| SEC-7 | security (MAIN) | subprocesses + secrets | pass | nt1 adds no subprocess call; no new log or debug output |
| N-R1 | corpus (reviewer-6) | sentence fidelity + recorded interpretations | fail→fixed 8a51bf0c | F1 s31-04's example dosages classified every oral morphine/oxycodone and equianalgesic dosage as the qualifying immediate-release class (source: immediate-release opioids; extended-release named apart) → immediate-release dosage nouns; red 8-case dosage probe rc 1 → rc 0; re-review pass |
| N-R2 | corpus | untouched sentences + whole-region coverage | pass | reviewer-6 |
| N-R3 | corpus | coverage statuses | pass | S31-04 + S40-14 → ace, S42-09 → restates(S40-14) |
| N-R4 | corpus | README + taxonomy counts | fail→fixed 8a51bf0c | F2 README second count mentions 130/176; re-review pass |
| N-R5 | corpus | knowledge-only status | pass | reviewer-6 |
| N-R6 | corpus | obligations + compiled comparisons | pass | 11 single-document proofs rc 0, 39 obligations |
| N-R7 | corpus | projection-notes fidelity | fail→fixed 8a51bf0c | F3 s27-02 note still claimed compressed joint plurality; re-review pass |
| N-R8 | corpus | lexicon delta | pass | reviewer-6 |
| N-R9 | corpus | highlight tables | pass | 11 align-check rc 0 |
