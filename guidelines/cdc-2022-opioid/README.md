# CDC 2022 opioid guideline

Dowell D, Ragan KR, Jones CM, Baldwin GT, Chou R. CDC Clinical Practice
Guideline for Prescribing Opioids for Pain — United States, 2022. MMWR Recomm
Rep 2022;71(No. RR-3):1–95. DOI `http://dx.doi.org/10.15585/mmwr.rr7103a1`.

## Source of record

- `source/rr7103a1-H.pdf` — official CDC PDF, retrieved 2026-07-22 from
  `https://www.cdc.gov/mmwr/volumes/71/rr/pdfs/rr7103a1-H.pdf` (two fetches,
  byte-identical). SHA-256
  `f4e5098d13e9b3dc5cc27bb90137df57d3667350b2add885fd367f279402d18d`,
  1,418,584 bytes. HTML manifestation:
  `https://www.cdc.gov/mmwr/volumes/71/rr/rr7103a1.htm`.
- `source/box3-extraction.txt` — verbatim extraction of Box 3 (the twelve
  recommendations; PDF pages 13–14). SHA-256
  `6776a52087f893b462c1f1092c1ed8799353438809368e0cfd123ca4244d0ac5`. Number
  labels in the extraction are presentation scaffolding, not CDC text edits.
- `source/front-matter-extraction.txt` — verbatim extraction of the
  normative statements on physical PDF pages 3–18 (Summary, Introduction,
  Box 1, Box 2, Scope and Audience, Methods, category framing, guiding
  principles; 66 payload regions). SHA-256
  `aa3306c0eeb15dac65c55103bfdcbf4d64c46c85b5c82bc1c0dd40867281168b`.
- `source/rec01-05-body-extraction.txt` — verbatim extraction of the
  normative statements on physical PDF pages 19–39 (Box 4 and the
  recommendation 1–5 subsections: restated statements, supporting
  rationale, implementation considerations; 309 payload regions). SHA-256
  `31fbc6cce791c4e09ff79e2e3fe4249db0e2635c82e6b5ec6b64e3d57c1dc4ff`.
- `source/rec06-12-body-extraction.txt` — verbatim extraction of the
  normative statements on physical PDF pages 40–63 (recommendation 6–12
  subsections and end matter through the Conclusion; 327 payload regions).
  SHA-256
  `0b6b3f1f2612e3d8ee17f0aa19a244dbeabeae2fee3b7ad7a9b214ab8d4857d6`.
  Each extraction file's header states its locator and scaffolding
  conventions; the statement payloads are byte-range regions of the file.

## Rights and attribution

CDC states at `https://www.cdc.gov/mmwr/about.html`: "All material in the MMWR
Series is in the public domain and may be used and reprinted without special
permission; citation as to source, however, is appreciated." Source: CDC. CDC
does not endorse this project. The project adds no CDC/HHS marks or
endorsement branding; the unmodified source PDF retains the official marks it
was published with.

## Formal derivatives (project-authored)

- `ace/cdc2022-opioid-recNN.ace` (NN = 01–12) — one ACE document per Box 3
  recommendation NN. Each document is a deliberately minimal formal projection
  of its recommendation — the recommendation's category and evidence type
  (the Box 3 closing parenthetical) as knowledge facts plus universal rules
  projecting the directive (rec05 adds a negation-as-failure guard) — not a
  complete formalization of the clinical semantics, conditions, or qualifiers
  of the CDC text. Read the ACE next to the extraction to judge coverage.
- `ace/cdc2022-opioid-recNN-impKK.ace` (NN = 01–05) — one ACE document per
  formalized implementation-consideration payload region of
  recommendations 1–5 (67 documents; `rec02-imp12` is intentionally
  absent — its region was a phantom duplicate removed from the evidence —
  and other skipped KK ordinals mark impl-list regions ruled
  `restates(...)` or `uncovered(...)` in `coverage.tsv`, their ordinals
  held reserved).
  Each holds one universal rule projecting the region's directive — or,
  for regions stating a benefit or possibility rather than an imperative,
  the clinician consideration it supports, per `audit/projection-notes.tsv`;
  implementation considerations carry no category/evidence parenthetical. Three non-clinician actor regions use their own classes:
  S24-02 `health-insurer-or-health-system` (neutral), S36-03
  `payer-health-system-or-state-medical-board` (neutral), and S35-09
  `taper-support-team-member` (human — nurses, pharmacists, and
  behavioral-health specialists serving as taper-support team members).
- `lexicon.ulex` — the guideline's APE user lexicon (domain compounds
  shared by all documents).
- `pl/cdc2022-opioid-recNN.pl` — Prolog compiled from the ACE by
  `vendor/ape/prolog/ace_to_pl.pl`; regenerate via
  `python3 -P tools/goal.py compile cdc-2022-opioid` — the compiler is
  their sole author.

These derivatives are project-authored formal projections; present them as
the project's own work, with CDC credited as the source of the underlying
recommendations. Every document states guideline knowledge alone — no
authored witnesses, probe queries, or named individuals. The compiler
derives each sentence's proof obligation and discharges it against the
document's own clauses, alone and co-loaded with the rest of the corpus;
those proofs establish internal consistency of the projection, not
guideline conclusions.

## Coverage

- Verbatim extraction evidence spans the whole document: pages 3–18, the
  Box 3 payload (pages 13–14), pages 19–39, and pages 40–63 — 718 payload
  regions across the four extraction files, each verified as rendered PDF
  text with duplicate regions held to the source's own repetition count. Physical pages 64–100 hold
  references, the appendix evidence reviews, and the publication notice;
  the project's sweep of those pages found no clinical normative
  statements (an epistemic caution about interpreting findings on p82 and
  the standard MMWR access/contact/public-domain notices on p100).
- Formalization: Box 3 at one document per recommendation (twelve
  documents, each preserving its recommendation's exact category/evidence
  parenthetical and simplified action rules), plus one document per
  formalized recommendation 1–5 implementation-consideration region
  (67 documents).
  The rules encode deontic modality ("should"), conditions, and negation
  where the source region states them; each document's remaining
  simplifications are recorded per region in `audit/projection-notes.tsv`
  (kept/dropped columns) and `coverage.tsv`.
- `coverage.tsv` records one status row per payload region (all 718):
  `ace(<docid>)` formalized (79), `restates(<id>)` verbatim or
  near-verbatim repetition of another region (122), `uncovered(<class>:
  <reason>)` deliberately not formalized (69 — classes: heading,
  process, external, aim, descriptive, notice), or `pending` awaiting a
  ruling (448); 270 rows are ruled, 448 pending. Restatement links
  target the canonical full statement — the Box 3 text for
  recommendation restatements, the main-text list for the guiding
  principles — and otherwise the earliest occurrence, so a link may
  point later in page order than an abbreviated echo; a region with its
  own ACE document keeps `ace(...)` even when it also repeats another
  region.
- Reconciliation against an independent 702-sentence census of the same
  pages is closed and recorded row-by-row in `audit/census-map.tsv`:
  694 census sentences map to evidence regions (the 27 the census
  exposed as extraction gaps were added as 25 regions — the largest
  being Box 2's "is" bullet list — and verified with the same fidelity
  checks), 4 rows are page-assignment artifacts whose sentences occur
  only at pages other census rows already cover, and 4 are the
  deliberately excluded p82/p100 statements above.
- `audit/projection-notes.tsv` records per ACE document what the rule
  keeps from its source region and what it drops or interprets —
  including where a region states a benefit, criticality, or weak
  possibility rather than an imperative and the projection renders it
  as a clinician consideration, decision, or education action.
