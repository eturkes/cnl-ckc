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
  principles; 54 payload regions). SHA-256
  `f3866abbc7a89e12273053bfb6e8d299bbaa733ced2523aefa3a2d999de83ed9`.
- `source/rec01-05-body-extraction.txt` — verbatim extraction of the
  normative statements on physical PDF pages 19–39 (Box 4 and the
  recommendation 1–5 subsections: restated statements, supporting
  rationale, implementation considerations; 305 payload regions). SHA-256
  `ef73b04adf0347d7d14d0b20b936c0498be7041149bd3d3b40621e824862004f`.
- `source/rec06-12-body-extraction.txt` — verbatim extraction of the
  normative statements on physical PDF pages 40–63 (recommendation 6–12
  subsections and end matter through the Conclusion; 318 payload regions).
  SHA-256
  `a35d75c6ff054fef51d772ae3ad56cbcc639f323d4e5cfb616a2321c3fd4a459`.
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
  of its recommendation — an actor-class fact, the recommendation's category
  and evidence type (the Box 3 closing parenthetical) as class facts on the
  named recommendation, one universal rule (rec05 adds a negation-as-failure
  guard), and probe queries for the rule and both class facts — not a
  complete formalization of the clinical semantics, conditions, or qualifiers
  of the CDC text. Read the ACE next to the extraction to judge coverage.
- `lexicon.ulex` — the guideline's APE user lexicon (domain compounds and
  proper names shared by all twelve documents).
- `pl/cdc2022-opioid-recNN.pl` — Prolog compiled from the ACE by
  `vendor/ape/prolog/ace_to_pl.pl`; regenerate via
  `python3 -P tools/goal.py compile cdc-2022-opioid` — the compiler is
  their sole author.

These derivatives are project-authored formal projections; present them as
the project's own work, with CDC credited as the source of the underlying
recommendations. Named individuals (`RecNN`, `RecNN-clinician`) and every
query are project-authored probe fixtures that exercise the compiled
rules; they are not CDC statements, and query proofs establish internal
consistency of the projection, not guideline conclusions.

## Coverage

- Verbatim extraction evidence spans the whole document: pages 3–18, the
  Box 3 payload (pages 13–14), pages 19–39, and pages 40–63 — 693 payload
  regions across the four extraction files, each verified as rendered PDF
  text with duplicate regions held to the source's own repetition count. Physical pages 64–100 hold
  references, the appendix evidence reviews, and the publication notice;
  the project's sweep of those pages found no clinical normative
  statements (an epistemic caution about interpreting findings on p82 and
  the standard MMWR access/contact/public-domain notices on p100).
- Formalization is Box 3 only, at one document per recommendation: each of
  the twelve ACE documents preserves its recommendation's exact
  category/evidence parenthetical and one simplified action rule. The
  rules do not yet encode deontic modality ("should"), conditions,
  objects, timing, or alternatives; Box 3's multi-sentence recommendations
  contain further normative sentences beyond the twelve projected rules,
  and the 677 payload regions outside Box 3 await per-statement ruling —
  formalize, record as uncovered with a reason, or link as a restatement
  of a covered statement. A per-statement coverage manifest is the next
  planned increment.
