# Extraction census rubric (fixed before coding)

Question per row: does the span hold a statement that the CDC extraction should have taken as a payload region, and if a corpus round took it, how would that round rule it?

Rows: `candidates.tsv` = every span of the CDC PDF text (pdftotext, running heads removed, pages joined) that no extracted payload covers, split at sentence ends + bullets (`•`, sub-bullet glyph `ï`). Layout can glue a heading to the next sentence or cut a sentence at a box, table or footnote; judge the statement the row carries. Mechanical rows (`mech` = reference-list | roster | digits) stay out of this pass.

Context: `.scratch/currency/full.txt` = the same PDF text with form feeds between pages (page N = the Nth form-feed-separated block); `guidelines/cdc-2022-opioid/source/*.txt` = the extracted payloads, each under a `[<region id> | p<page> | <section>]` locator line; `guidelines/cdc-2022-opioid/coverage.tsv` = each region's ruling; `guidelines/cdc-2022-opioid/README.md` § Coverage "Ruling tests" = the ledger's ruling conventions.

## Extraction boundary

Calibrated on the 713 extracted regions: the extraction took every sentence with normative force, then ruled it (`ace`, `restates`, `uncovered`). In-scope classes = normative force:

- `directive` — says what someone should, must, may, need not or is required to do; recommends, prefers, discourages, avoids, permits; an imperative (bullet imperatives included); `it is important/essential to …`; `X is recommended/preferred/warranted/appropriate/indicated/contraindicated`; to anyone (clinicians, patients, practices, health systems, payers, committee members). Extracted examples: S3-01, S19-08, S14-04 (`Members are required to complete the Office of Government Ethics Form 450 …`).
- `action-claim` — in the guideline's own voice: an action, practice, test or treatment can/might help, facilitate, reduce, improve, or is helpful, useful, beneficial, harmful or safer; or a condition might preclude or alter an action. Extracted examples: S21-04 (`Vasoactive effects of triptans and ergot alkaloids might preclude their use …`), S25-07 (`Review of the patient’s history … is helpful in selection of pain treatments.`), S44-13, S50-03.
- `definition` — defines or delimits a term the recommendations use: pain durations, opioid use disorder criteria, category A/B, evidence types, opioid-tolerant, MME. Extracted example: S12-01.
- `scope` — whom, where or what the guideline or a recommendation applies to or not; its voluntary or flexible nature; how it must not be applied. Extracted examples: S3-02, S3-03.
- `reported` — reports guidance, advice, policy, law, regulation or labeling by a third party or by experts, reported expert disagreement about practice included. Extracted examples: S5-02 (state legislation), S20-09 (`ACP recommends …`), S40-01 (`experts have advised that …`).
- `aim` — a goal, aim, aspiration, principle or value of the guideline or of pain care. Extracted examples: S4-01, S4-03.

Out-of-scope classes = no normative force:

- `evidence` — study or review findings, statistics, epidemiology, effect estimates, associations, evidence quality attributed to studies or reviews (`A systematic review found …`, `studies suggest …`, `was associated with …`, `evidence was insufficient …`).
- `background` — history, context, mechanism, a description of a condition, drug or system, with no action claim (the 2016 guideline's history, prevalence narrative, pharmacology).
- `method` — guideline-development process narrative (reviews, GRADE mechanics, workgroup and committee steps, public comment); a process sentence that states a duty = `directive`.
- `scaffolding` — headings, titles, captions, table or figure labels and cells without a statement, contents, mastheads, author lists, affiliations, acknowledgments, disclosures, notices, contact lines, a cross-reference alone (`See Recommendation 5`), citations.
- `fragment` — a broken piece of a sentence whose statement an extracted region carries (name it), or text too broken to read as a statement.

A row holding several types → the first in-scope class it holds in the order directive, action-claim, definition, scope, reported, aim; else the out-of-scope class that fits best.

## Disposition

For the six in-scope classes, the ruling a corpus round would give (REFERENCE § Operating + README "Ruling tests"):

- `ace` — a round would author ACE: a directive, permission, preference, definition or action-claim with a clinical or system subject whose content no extracted region carries. A new condition, permission, qualifier, strategy or can-claim beside an extracted statement also makes it `ace` (README: S49-14, S56-12, S44-13).
- `restates(<id>)` — repeats an extracted region's content (verbatim or near-verbatim, the same duty or claim, nothing new); `<id>` = the canonical region (a region whose own status is not `restates`; follow its link).
- `uncovered(<class>)` — class ∈ `aim`, `descriptive`, `external`, `process`, `heading`, `notice` (the ledger's classes; `external` = reports a third party's guidance or policy; `descriptive` = context, or a fusion of canonical duties without a new one).

Out-of-scope classes → `-`.

## Reason

One clause. For `ace`: the duty, claim or definition a round would type. For `restates`: what matches. For `uncovered`: why. For out-of-scope rows a short phrase (`study finding`, `2016 history`, `GRADE mechanics`, `heading + study finding`).
