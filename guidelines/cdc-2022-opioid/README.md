# CDC 2022 Opioid Guidelines

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
conventions. The statement payloads are byte-range regions of the file.

## Rights and attribution

CDC states at `https://www.cdc.gov/mmwr/about.html`: "All material in the MMWR
Series is in the public domain and may be used and reprinted without special
permission; citation as to source, however, is appreciated." Source: CDC. CDC
does not endorse this project. The project adds no CDC/HHS marks or
endorsement branding. The unmodified source PDF retains the official marks
that it was published with.

## Formal derivatives (project-authored)

- `ace/cdc2022-opioid-recNN.ace` (NN = 01–12) — one ACE document per Box 3
  recommendation NN. Each document is a deliberately minimal formal
  projection of its recommendation. It is not a complete formalization of
  the clinical semantics, conditions, or qualifiers of the CDC text. It
  holds the recommendation's category and evidence type (the Box 3 closing
  parenthetical) as knowledge facts, plus universal rules that project the
  directive. `rec05` adds a negation-as-failure guard. Read the ACE next
  to the extraction to judge coverage.
- `ace/cdc2022-opioid-recNN-impKK.ace` (NN = 01–05) — one ACE document per
  formalized implementation-consideration payload region of
  recommendations 1–5 (67 documents). `rec02-imp12` is intentionally
  absent: its region was a phantom duplicate removed from the evidence.
  Other skipped KK ordinals mark impl-list regions ruled `restates(...)`
  or `uncovered(...)` in `coverage.tsv`; their ordinals stay reserved.
  Each document holds one or more knowledge sentences — typically
  universal rules (21 documents hold a single sentence, the rest 2 to 25).
  The sentences project the region's directive, per
  `audit/projection-notes.tsv`. For a region that states a benefit or
  possibility rather than an imperative, they project the clinician
  consideration that it supports. Implementation considerations carry no
  category/evidence parenthetical. Three non-clinician actor regions use
  their own classes:
  - S24-02 `health-insurer-or-health-system` (neutral);
  - S36-03 `payer-health-system-or-state-medical-board` (neutral);
  - S35-09 `taper-support-team-member` (human — nurses, pharmacists, and
    behavioral-health specialists serving as taper-support team members).
- `ace/cdc2022-opioid-<region-id>.ace` — one ACE document per formalized
  front-matter or recommendation 1–5 supporting-rationale region outside
  the implementation-consideration lists (107 documents; docid = the
  region id lowercased, S25-04 → `s25-04`). These documents follow the
  same knowledge-only projection contract and
  `audit/projection-notes.tsv` discipline as the implementation
  documents. Non-clinician actor classes ruled in:
  - `primary-care-clinician` and `pain-care-clinician` (human, the
    source-named audiences);
  - `respiratory-depressant-medication-prescriber` (human);
  - `pain-equity-duty-holder` — a neutral superclass that shares the
    S19-07 inequity/communication/access duties across clinician,
    clinical-practice-organization, health-system, and payer.

  Guideline scope and misapplication regions take the guideline or
  recommendation itself as rule subject.
- `ace/` recommendation 6–12 documents — one ACE document per formalized
  recommendation 6–12 body or implementation-consideration region,
  special-populations region, and conclusion region (151 documents;
  docid rules unchanged: reserved-ordinal `recNN-impKK` for
  implementation lists, the region id lowercased otherwise). These
  documents follow the same knowledge-only projection contract and
  `audit/projection-notes.tsv` discipline. Additional actor classes
  ruled in:
  - `care-coordinating-clinician`, `other-clinician`, and
    `substance-use-disorder-treatment-provider` (human);
  - `prolonged-severe-acute-pain-care-mechanism` (neutral);
  - `clinical-practice-organization` and `health-system` carry the
    organization-level naloxone-access and care-coordination duties.
- `lexicon.ulex` — the guideline's APE user lexicon (domain compounds
  shared by all documents).
- `pl/cdc2022-opioid-recNN.pl` — Prolog that
  `vendor/ape/prolog/ace_to_pl.pl` compiled from the ACE. Regenerate it
  with `python3 -P tools/goal.py compile cdc-2022-opioid`; the compiler is
  the sole author of these files. Under the full-Clex base lexicon, six
  documents (s8-06, s26-07, s34-10, s37-15, s38-17, s39-12) read verb
  `to`-complements as ditransitive third arguments
  (`guideline_arg(...,3,...)`). The trimmed demo lexicon yielded
  `guideline_pp(...,to,...)` there. The project reviewed and accepted the
  ditransitive reading as the tighter subcategorized one. Same-surface
  ulex/Clex coexistence rulings live in `audit/lexicon-shadow.tsv`.

These derivatives are project-authored formal projections. Present them as
the project's own work, and credit CDC as the source of the underlying
recommendations. Every document states guideline knowledge alone — no
authored witnesses, probe queries, or named individuals. The compiler
derives each sentence's proof obligation and discharges it against the
document's own clauses, alone and co-loaded with the rest of the corpus.
Those proofs establish internal consistency of the projection, not
guideline conclusions.

## Coverage

- Verbatim extraction evidence spans the whole document: pages 3–18, the
  Box 3 payload (pages 13–14), pages 19–39, and pages 40–63. That is 718
  payload regions across the four extraction files. The project verified
  each region as rendered PDF text and held duplicate regions to the
  source's own repetition count. Physical pages 64–100 hold references,
  the appendix evidence reviews, and the publication notice. The project's
  sweep of those pages found no clinical normative statements. The sweep
  found only an epistemic caution about interpreting findings on p82, plus
  the standard MMWR access/contact/public-domain notices on p100.
- Formalization covers Box 3 at one document per recommendation: twelve
  documents, each preserving its recommendation's exact category/evidence
  parenthetical and simplified action rules. It adds one document per
  formalized recommendation 1–5 implementation-consideration region
  (67 documents). It adds one document per formalized front-matter and
  recommendation 1–5 rationale region (107 documents). It adds one
  document per formalized recommendation 6–12 body,
  implementation-consideration, special-populations, and conclusion
  region (151 documents). The rules encode
  deontic modality ("should"), conditions, and negation where the source
  region states them. `audit/projection-notes.tsv` (kept/dropped columns)
  and `coverage.tsv` record each document's remaining simplifications per
  region.
- `coverage.tsv` records one status row per payload region (all 718), and
  every region carries a ruling. The statuses are:
  - `ace(<docid>)` — formalized (337);
  - `restates(<id>)` — verbatim or near-verbatim repetition of another
    region (262);
  - `uncovered(<class>: <reason>)` — deliberately not formalized (119;
    classes: heading, process, external, aim, descriptive, notice);
  - `pending` — ruled for formalization with the ACE document not yet
    authored (0 — the recommendation 6–12 worklist closed with the
    ace-wave2 documents).

  Restatement links target the canonical full statement: the Box 3 text
  for recommendation restatements, the main-text list for the guiding
  principles, and otherwise the earliest occurrence. A link may therefore
  point later in page order than an abbreviated echo. Links stay
  single-step: when a target is itself re-ruled `restates`, inbound links
  re-point to the transitive root (the echo relation is transitive). A
  region with its own ACE document keeps `ace(...)` even when it also
  repeats another region.
- Ruling tests. A CDC-voice imperative or endorsement with a trailing
  citation is the guideline's own directive: formalize it.
  `uncovered(external)` is reserved for reporting frames that name a
  third-party body or document (S10-02, S31-06). A region that fuses
  several canonical duties without adding one takes
  `uncovered(descriptive: … canonical at <ids>)` (S8-04, S17-03, S22-06,
  S41-02). Weak-modality considerations ("can", "might be needed")
  formalize as the consideration they support. A novel condition,
  permission, or strategy added to an otherwise-canonical echo makes the
  region formalize rather than restate (S49-14, S52-10, S55-01). An
  emphasis qualifier alone does not (S56-12).
- Projection rulings (wave-1 review). ACE "does not V a N" negates the
  existential — a takes-none reading (S37-13). An unhedged source
  endorsement ("preferred", "are recommended") renders as a should-shaped
  duty. Practice hedges ("typically", "generally", "ideally") drop with a
  notes admission; "can"/"might" render as may. Source or-alternatives may
  split into conjunctive consider/discuss duties or parallel non-exclusive
  permissions, admitted in notes. "Only if"/"only after" restrictions
  render as prohibition-unless-guard conditionals that bind clinician,
  object, and patient in one sentence (S25-04, S25-08 shapes).
  Cross-sentence referential identity is not modeled: fresh existentials
  per rule are the corpus idiom. Each identity or linkage loss lands in
  the notes dropped column.
- Projection rulings (wave-2 review). Material normative-semantics changes
  — altered conditions, population scope, modality force, invented bounds
  or triggers — require an ACE fix regardless of notes disclosure.
  Structural approximations — identity splits, fresh witnesses,
  nominalizations, admitted hedge or example drops — are cured by a notes
  admission (S51-09 line). A source threshold stated as a bare percentage
  may render as an at-least bound when the notes row admits the reading
  (S44-17). Batch-flag sentence counts are advisory; the source text
  governs rule count (S40-01).
- Reconciliation against an independent 702-sentence census of the same
  pages is closed and recorded row-by-row in `audit/census-map.tsv`. 694
  census sentences map to evidence regions. The census exposed 27
  sentences as extraction gaps. The project added them as 25 regions, the
  largest being Box 2's "is" bullet list, and verified them with the same
  fidelity checks. 4 rows are page-assignment artifacts whose sentences
  occur only at pages that other census rows already cover. 4 rows are
  the deliberately excluded p82/p100 statements above.
- `audit/projection-notes.tsv` records, per ACE document, what the rule
  keeps from its source region and what it drops or interprets. That
  includes the regions that state a benefit, criticality, or weak
  possibility rather than an imperative; the projection renders those as
  a clinician consideration, decision, or education action.
