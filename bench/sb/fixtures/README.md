# SB1 source fixtures

Contract = `.agent/contracts/sb.md`; source constraints/excerpts = `bench/sb/annex/`. These are benchmark inputs, not KB additions or treatment advice. `cases.tsv` = 19 document inputs; `values.tsv` = exact selections, intake mappings + unresolved limits. P-adult preserves the four sentences in the consumer fixture; the consumer wrapper stays unchanged.

## Reproduction

With the pinned toolchain on PATH:

```sh
bash bench/sb/fixtures/regenerate "$PWD" "$PWD/rust/target/release/ckc"
bash bench/sb/fixtures/regenerate "$PWD" "$PWD/rust/target/release/ckc" --check
(cd rust && cargo test --locked --offline -p ckc --test bench_fixtures)
```

- Stage = private APE copy + full Clex, grammar built, Clex qcompiled with production options. Ulex = CDC's lexicon followed by `lexicon.ulex` (distinct surfaces).
- Per row: `.ace` → `.dump.in` + `.pl.in` (the standard compiler header); `ckc certify-one doc` → `.certified.in` (proof payload). `vocabulary=cdc` uses the CDC table; `basic` uses this directory's timing/qualifier table.
- Fractional intakes: `.emitted.pl.in` = pure ACE output, certified against `.dump.in`; `.intake.tsv` = declared ground-fact operations. F5a replaces its weak amount cardinality by `rational(75,2)`; F5c appends 25 facts for five fractional strengths whose numeric claims ACE leaves absent. `.pl.in` = complete root-fact intake. Tests pin the exact operations/reduced source values, K1 acceptance + the canonical reading/spelling round trip. K1 syntax acceptance alone does not establish the patient-term numeric grammar.
- `f5c-inputs` replays the full 24-row source table into ACE, its identity map, fractional intake + ulex section before compilation. It checks source quantities/units, component order, direction counts, periods, intervals, PRN + time-of-day labels against the retained source cells. The tests rematch every source cell to the annex excerpt and its frozen cell digest.
- Patient inputs contain root facts only. Source bounds do not become invented observations: selected exact values have `assumption` + `engineering_setting` rows. Dated/named visits and the pre-interruption home list remain distinct. The latter's four prescribe events cite the same snapshot; prescription status never asserts actual drug administration.

## Composition

Each F3 bridge has separate `*-classifier` and `*-reader` documents. CDC scope guards belong to CDC readers, not the VA readers. Shared patient-facing nouns are unchanged; actual diagnoses feed recommendation conditions, never the reverse.

| Bridge | Classifier | Reader |
| --- | --- | --- |
| F3-B1 | CDC duration → chronic pain | VA opioid R15 assessment |
| F3-B2 | CDC duration → acute pain | VA opioid R11 prohibition + R16 screening |
| F3-B3 | VA Appendix C vetted count/pattern → OUD | CDC R12 medication offer |
| F3-B4 | CDC s57-05 count/window → OUD | VA SUD R17 extended-release IM naltrexone offer |

- B2 remains compiled but inactive in the chronic-only P-F3/P-MX worlds. `<1 month` has no positive integer witness in that source unit; patient intake does not add a zero, fraction or converted-day claim.
- VA SUD R16's alternatives/settings are blocked by the fork's disjunctive-consequent limit. The suite replays the minimal required OR shape. R17 is a separate retained branch, not an R16 substitute.
- F4-L2 uses `patient-has-OUD → patient-has-SUD`. Its SUD referent is fresh: the schema's noun copula creates a be-link, not a second type on the original disorder. This authoring interpretation awaits the contract's scientist appraisal before grading.
- F2 calibrates the ten-shard mechanism through the published core profile, not a clinical translation of Appendix 1's disjunctive BP/CHD/COPD rules. P-MX is synthetic: F5b's prescription counts/classes + PRN status, without source doses, age or interruption history. The annex supports a qualitative coherence claim, never an observed full tuple; the scientist's grading appraisal remains separate. Neither disposition authorizes a graded campaign.

## Functional probes

`rust/ckc/tests/bench_fixtures.rs` loads the full committed CDC corpus under the REFERENCE recipe. Every SWI process must exit 0 with empty stderr.

- F4: VA diagnosis → ASAM patient-level subtype → **s60-11 S2/box1 MUST**, exact ground identities. The full corpus supplies VA and CDC diagnostic supports, hence two contexts. Missing diagnostic seed, missing actual pain or age 17 → zero targets. Removing the subtype link → zero s60-11 targets while s57-02 still has two shortcut contexts; generic action names cannot grade this contrast.
- F5b: **rec11-imp01 S2/box1 SHOULD** has two enabled valuations: `(clinician s1/r1, opioid s8/r1, alprazolam s6/r1, prescription s8/r4)` and `(clinician s1/r1, opioid s10/r1, alprazolam s6/r1, prescription s10/r2)`. Every slot belongs to `sb-p-f5b`. Removing the two `with` relations keeps medication facts but yields zero joins. Source medication degrees stay 2 opioids × 1 benzodiazepine; G never amplifies them.
- F5c: all 24 Table1 products remain separate, the combination nebulizer product included; PRN, regular + weekly directions stay distinct. The full CDC has three medication joins: **rec09-imp13 S1/box1**, **rec11 S2/box1**, **rec11-imp01 S2/box1**, all SHOULD, valuation `(clinician s1/r1, tramadol s88/r1, alprazolam s12/r1, prescription s97/r1)` in `sb-p-f5c`. The complete intake has 785 unique clinical ground facts (schema/document metadata excluded). Those joins' positive premises use 10; **775 are unused by these medication-join premises**, not by decoding/admission. The same two products support all three joins. This is high-record loading + a 1×1 medication join, not a 24-way join; the two GABA products are retained but do not gain an unobserved administration/CNS-depressant link. Removing `with M2` keeps 24 products and yields zero medication joins.
- Self-plants: fentanyl amount 50 → 51 in a private compiled-fixture copy remains K1 syntax but fails regeneration; F5c's source-50/mapped-51 strength fails the source-to-intake check; a changed source cell fails the frozen transcription check.

The probes do not grade workload realism, throughput, CMS exposure eligibility, MX coherence or the final model checker. Those remain SB1's grading-authorization work.
