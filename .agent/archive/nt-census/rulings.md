# Numeric-threshold census — rulings

Census = `census.tsv` (241 numeric statements: id, region, file:line, span, quantity, comparator, bound, unit, normative, ace, proposal) over `regions.tsv` (702 regions: box 3 excluded; its 16 regions hold one numeric statement, B3-10 `within 1–4 weeks`, typed in rec07). Rulings (MAIN unless marked user):

- `typed` (75): no ACE change; the nt1 migration converts each condition.
- `not-threshold` (42) + `out-of-class` (58): no ACE change; classes recorded (dose-arithmetic, derived-measure, concurrency or temporal overlap, descriptive statistics).
- A `re-author` proposal on an `uncovered(external|process|descriptive)` region: the region keeps its class (third-party policy, process, context); no document.
- Terms of art naming an approach (`multimodal`, `multidisciplinary`, `single-modality`; c1-015..018, 041, 042, 055, 056; c2-141..144): no count statement → lexical.
- `single or few joints` (c1-043, 048): `few` states no integer → lexical.
- c1-086 (S37-10 `one or more medications`): the indefinite already states at least one → no change.
- c1-084, c1-091 (`10% per month`): a rate relative to the dose = dose-arithmetic → lexical.
- c1-088 (S38-14), c2-040 (S44-03): descriptive study findings → no change.
- c2-109, c2-120 (concurrent controlled medications): concurrency, no count → no change.

Re-authored (interpretation per document in `guidelines/cdc-2022-opioid/audit/projection-notes.tsv`):

| document / region | source threshold | ACE |
|---|---|---|
| s9-05 / S9-05 | outpatients aged ≥18 years | `If an outpatient has an age and the age includes at least 18 years then the outpatient is an adult-outpatient.` |
| s10-04 / S10-04 | aged <18 years | `If a patient has an age and the age includes less than 18 years and …` |
| s34-04 / S34-04 | aged ≥65 years | `If a patient has an age and the age includes at least 65 years and …` (rec08-imp12 shape) |
| s48-02 / S48-02 | aged ≥65 years | same shape, both sentences |
| new s40-14 / S40-14 (S42-09 restates it) | opioids continued ≥1 month | own document in Rec 6 wording, `during at least 1 month`; rec02-imp20 keeps `at least 30 days` for S24-18 + S28-14 (user ruling: each passage keeps its source unit) |
| rec05-imp18 / S35-18 (+ S39-06) | taken less frequently than once a day | `uses an opioid at an interval of more than 1 day`, inside the discontinuation-goal + smallest-dose context |
| s38-03 / S38-03 | patients prescribed more than one opioid (e.g.) | `If a patient receives more than 1 prescribed-opioid then …` on the decrease-first decision |
| rec02-imp07 / S24-05 | osteoarthritis pain in multiple joints | `an osteoarthritis-pain in at least 2 joints` (precedent rec09-imp12: multiple clinicians → at least 2) |
| s27-02 / S27-02 | multiple joints | same |
| s54-15 / S54-15 | tests repeatedly negative | `at least 2 negative-tests` |
| new s31-04 / S31-04 (restated S30-11 before) | certain IR dosages daily for at least 1 week (e.g. 60 mg oral morphine, 30 mg oral oxycodone daily) | own document: ER/LA only after `during at least 1 week` (Prohibitions form); `daily` stays in the dosage noun (user ruling: a bare calendar adverb carries no bound); the example dosages as qualifying minimums |

Corpus fidelity leads outside the class → `.agent/deferred.md` row: S19-12 / S22-05 scheduled example `one tablet every 4 hours` dropped; S30-07 two-medication example dropped; S52-15 / S54-06 `several` / `multiple` substances rendered singular; S59-07 projection note claims a typed ≥2-month duration the ACE does not carry.
