# fidelity-leads — corpus fidelity leads outside the numeric-threshold class (contract; archived at unit close)

Row = NT census find (`.agent/archive/nt-census/rulings.md`): S19-12 / S22-05 drop the scheduled example `one tablet every 4 hours` (a recurrence the schema carries); S30-07 drops its two-medication example; S52-15 / S54-06 render `several` / `multiple` substances singular; S59-07's projection note claims a typed ≥2-month duration its ACE does not carry. Accept (the row's): each region re-authored or its projection note corrected, align tables + notes current, `ckc check` green.

## Tier

`data` (corpus ACE; re-validated downstream by clinician review beside the source, `ckc check`, per-document certification). MAIN drafts + rules (corpus.md); one closing `reviewer` (L1 fidelity, L2 coverage claims, L6 notes, align spans) before the commit.

## Design

- D1 rec01-imp03 (S19-12; S22-05 restates it): new S3 in source order, between the as-needed example and the taper rule — `A scheduled-opioid-example uses 1 tablet at an interval of 4 hours.` → typed recurrence over a plain 4-hour count (`eq 4`; value comparisons read a plain number as that count or more — REFERENCE § Comparison conditions; a root fact cannot state `exactly 4`: the compiler rejects `root_condition(…exactly,4…)`, measured on the scratch stage); the example's tablet product stays unstated, as in the source. ulex += `scheduled-opioid-example`.
- D2 s30-07 (S30-07): two example facts after the recommendation, one per source example (`or` = two examples) — `A medication-combination-example includes 2 nonopioid-medications that differ in a mechanism-of-action.` + `A medication-combination-example includes a nonopioid-medication and includes an opioid.` (rec01-imp03's example-fact precedent: inert example nouns, no witness of `medication-combination`). ulex += `medication-combination-example`, `mechanism-of-action`, plural `nonopioid-medications`.
- D3 rec10-imp08 (S52-15; S54-06 restates it): S2 → `… can assess an opiate-class and can assess a benzodiazepine-class and can assess some nonprescribed-substances.` — `several` (S52-15) → the ACE plural (cardinality ≥ 2, no numeral: 07f1060e's ruling for `several months`), which also carries S54-06's `multiple` (≥ 2, rec09-imp12 precedent); `benzodiazepines as a class` (same sentence; its note recorded the loss) → `a benzodiazepine-class`, mirroring `an opiate-class`. ulex += `benzodiazepine-class`, plural `nonprescribed-substances`.
- D4 s59-07 (S59-07): lead refuted, no change — the compiled clauses carry the typed duration the note states: `guideline_interval(…,duration,ref(5),month,none)` + `guideline_cardinality(…,ref(5),na,geq,2)` (ACE plural `some months`).
- D5 Ledgers: projection notes of rec01-imp03, s30-07, rec10-imp08 rewritten; align tables re-authored for the three (`ckc align`); coverage.tsv + README counts unchanged (no new document, statuses kept); review manifest re-derived (`ckc review-manifest`); the ulex change recompiles the whole guideline (`ckc compile`) + `ckc queries` (tablet-use-spacing may gain the scheduled example: its question's `at least 4 hours` compares by value since nt1) + `just amend-manifest`.

## Predicates

- P1 `ckc compile cdc-2022-opioid` rc 0; rec01-imp03 carries `guideline_recurrence(…,Q,hour)` with Q `eq 4` on the scheduled example's use; s30-07 carries `guideline_cardinality(actual,…,na,eq,2)` on the nonopioid-medications + the `differ` event with `in` → mechanism-of-action, and the second example's two `include` events; rec10-imp08 carries `guideline_cardinality(…,na,geq,2)` on nonprescribed-substance under the `can` box + `benzodiazepine-class`.
- P2 Diff outside the three documents = ulex digest lines of every document record + the derived query/manifest artifacts alone (`git diff --stat` + a scan: every other changed `pl/*.pl` line = its `guideline_document` record).
- P3 `ckc check` green (coverage, align inventory + offsets, lexicon law, review manifest, release manifest); `ckc certify cdc-2022-opioid` green; `ckc ui check` green; full `just gate` green at the unit commit; row pruned in it.

## Rulings

- R1 (MAIN, reviewer-1 F1): the scheduled spacing stays a plain count here; the existence fact + universal rule that carries an exact root value (compiles, head `exactly 4`) sets corpus practice + reverses NT2's typed ruling for rec01-imp03's amounts → `.agent/deferred.md` row, user ruling. The rec01-imp03 note claims no exact count (re-review pass).
- R2 (MAIN, reviewer-1 G1): S22-05 + S23-07's untyped duties (work with patients to prevent prolonged use; provide + discuss a tapering plan) predate this unit → `.agent/deferred.md` row.
