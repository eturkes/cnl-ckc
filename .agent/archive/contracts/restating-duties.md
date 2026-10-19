# restating-duties — S22-05 + S23-07 carry duties their restate target never types (contract; archived at unit close)

Row = reviewer-1 G1 at the fidelity-leads review (this session's new find): S22-05 + S23-07 = `restates(S19-12)` (`cdc2022-opioid-rec01-imp03`), yet S22-05 adds `Clinicians should work with patients to prevent prolonged opioid use` + `encourage and include an opioid taper` (`source/rec01-05-body-extraction.txt:92`) and S23-07 states `To minimize withdrawal symptoms, clinicians should provide and discuss an opioid tapering plan when opioids will be used around the clock for more than a few days` (`:124`); rec01-imp03 types only `encourage` + `recommend` an opioid taper. Accept (the row's): each duty compiled in a document (the region's own, coverage `ace(<docid>)`, or typed in the target it restates), the purpose carried as the corpus carries purposes (`during|for a <purpose>` PP), align tables + projection notes current, `ckc check` green.

## Tier

`data` (corpus ACE; clinician review, `ckc check`, certification re-validate). MAIN drafts + rules; one closing `reviewer` (L0–L6 over the two regions) before the commit.

## Design

- D1 S22-05 → own document `cdc2022-opioid-s22-05` (a region with its own document formalizes its whole content: precedent s49-11, 15d439fb): S1 `Every clinician should work with a patient for a prolonged-opioid-use-prevention.` (purpose PP, precedent rec05-imp10 `for a withdrawal-minimization`); S2–S4 = the restated preference + the as-needed + scheduled examples (rec01-imp03 S1–S3 wording); S5 = the taper rule with S22-05's verbs: `… should encourage an opioid-taper and should include an opioid-taper.` Coverage S22-05 → `ace(cdc2022-opioid-s22-05)`.
- D2 S23-07 → own document `cdc2022-opioid-s23-07`: `If a patient has a prolonged-around-the-clock-opioid-use for a few-day-period then every clinician should provide an opioid-taper-plan to the patient for a withdrawal-minimization and should discuss an opioid-taper-plan with the patient for a withdrawal-minimization.` (rec01-imp03's antecedent for `when opioids will be used around the clock for more than a few days`; each duty carries the purpose; a referent introduced under one `should` box is inaccessible from the next ⇒ two plan referents, recorded). Coverage S23-07 → `ace(cdc2022-opioid-s23-07)`.
- D3 ulex += `prolonged-opioid-use-prevention` → whole-guideline recompile; `ckc queries` (tablet-use-spacing gains s22-05's two examples); review manifest +2 rows.
- D4 Ledgers: align tables for both, projection notes (2 rows), guideline README coverage counts (ace 387, restates 212), `docs/m7-gap-taxonomy.md` totals that name the document count or the preference count (re-derived by grep over `pl/`).
- D5 (reviewer-2 F1 + G2, accepted): the as-needed example's `for moderate to severe pain` sits inside its parenthetical ⇒ the general prescribe/advise duties carry no severity restriction: s22-05 S2 + rec01-imp03 S1 drop both `for a moderate-to-severe-pain` PPs (the as-needed example keeps its PP); both align tables + notes follow. README's ditransitive note (reviewer-2 G1) stops reading as an exhaustive six-document census (32 documents carry a third argument).

## Predicates

- P1 `ckc compile` rc 0 (387 documents); s22-05 carries the work event (`with` patient, `for` prolonged-opioid-use-prevention) under `should`, 2 preference records, both example recurrences, the encourage + include taper events; s23-07 carries provide (`to` patient, `for` withdrawal-minimization) + discuss (`with` patient, `for` withdrawal-minimization) under `should` in a rule on prolonged-around-the-clock-opioid-use.
- P2 Diff outside the two new documents = ulex digest record lines + derived query/manifest artifacts + the ledgers of D4.
- P3 `ckc check` green (coverage meter ace=387 restates=212, align, lexicon, review + release manifests); full `just gate` green at the unit commit; row pruned in it.

## Rulings

- R1 (MAIN, reviewer-2 F1/G2): accepted → D5, one re-review round (R2 + R6 + R7 over s22-05 + rec01-imp03).
- R2 (MAIN, reviewer-2 G1): fixed here (README ditransitive note), outside the re-review.
