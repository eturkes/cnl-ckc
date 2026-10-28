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
  principles; 121 payload regions). SHA-256
  `0eabcb0b3beee46df3de0a059f1aa98abafbd6d4f46ac895c038fce3d8458775`.
- `source/rec01-05-body-extraction.txt` — verbatim extraction of the
  normative statements on physical PDF pages 19–39 (Box 4 and the
  recommendation 1–5 subsections: restated statements, supporting
  rationale, implementation considerations; 387 payload regions). SHA-256
  `742ad9d7882c83da08ac7af3b2237a5f6fe3cc39849a4805a604f9c993b6a8c4`.
- `source/rec06-12-body-extraction.txt` — verbatim extraction of the
  normative statements on physical PDF pages 40–63 (recommendation 6–12
  subsections and end matter through the Conclusion; 401 payload regions).
  SHA-256
  `35068e04c199d443fa268b826cc5fefa1e25f05d3ab1ebe9c57c711d94e88fcc`.
- `source/appendix-extraction.txt` — verbatim extraction of the
  statements on physical PDF pages 64–100 that carry normative force
  (the appendix evidence reviews on page 82; 2 payload regions). SHA-256
  `aa9b62df229edb16537a705125c13ac4bfc475624d4dcfc3e589911ce875ccfd`.

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
  `audit/projection-notes.tsv`. For a region that states a benefit or a
  possibility rather than an imperative, they state the claim under
  "can" with its qualifier (schema S2). Implementation considerations carry no
  category/evidence parenthetical. Three non-clinician actor regions use
  their own classes:
  - S24-02 `health-insurer-or-health-system` (neutral);
  - S36-03 `payer-health-system-or-state-medical-board` (neutral);
  - S35-09 `taper-support-team-member` (human — nurses, pharmacists, and
    behavioral-health specialists serving as taper-support team members).
- `ace/cdc2022-opioid-<region-id>.ace` — one ACE document per formalized
  front-matter or recommendation 1–5 supporting-rationale region outside
  the implementation-consideration lists (127 documents; docid = the
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
  special-populations region, and conclusion region (176 documents;
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
- `ace/` extraction-census documents — one ACE document per formalized
  region that the extraction census added (110 documents; docid = the
  region id lowercased, S57-05 → `s57-05`). They follow the same
  knowledge-only contract and `audit/projection-notes.tsv` discipline. An
  example item uses an example-only noun (`a sleep-apnea-example`), and an
  option claim keeps its option inside `it is possible that`, so that no
  derived entity matches the rules that read the real noun.
- `lexicon.ulex` — the guideline's APE user lexicon (domain compounds
  shared by all documents).
- `pl/cdc2022-opioid-recNN.pl` — Prolog that
  `vendor/ape/prolog/ace_to_pl.pl` compiled from the ACE. Regenerate it
  with `ckc compile cdc-2022-opioid`; the compiler is
  the sole author of these files. Under the full-Clex base lexicon, a
  verb that Clex subcategorizes as ditransitive reads its `to`-complement
  as a third argument (`guideline_arg(...,3,...)`). The trimmed demo
  lexicon yielded `guideline_pp(...,to,...)` in six documents (s8-06,
  s26-07, s34-10, s37-15, s38-17, s39-12). The project reviewed and
  accepted the ditransitive reading as the tighter subcategorized one. Same-surface
  ulex/Clex coexistence rulings live in `audit/lexicon-shadow.tsv`.

These derivatives are project-authored formal projections. Present them as
the project's own work, and credit CDC as the source of the underlying
recommendations. Every document states guideline knowledge alone — no
authored witnesses, probe queries, or named individuals. For each
sentence, the compiler checks that the emitted clauses derive the
conclusion of the sentence from sample facts that satisfy its
conditions, for the document alone and with the rest of the corpus
loaded. This replay check finds clauses that do not carry their
sentence. It does not show that a sentence states the guideline
correctly: the clinician review checks that.

## Coverage

- Verbatim extraction evidence spans the whole document: pages 3–18, the
  Box 3 payload (pages 13–14), pages 19–39, pages 40–63, and two
  appendix statements on page 82. That is 927 payload regions across five
  extraction files. The project verified each region as rendered PDF text
  and held duplicate regions to the source's own repetition count. An
  extraction census (`.agent/archive/extraction-census/`) coded every PDF
  text span outside the first 713 regions twice, blind, and adjudicated
  the disagreements. It found 234 statements with normative force that the
  first extraction had left out. Each of them is now part of a region:
  214 new regions, plus the nine tapering conditions that complete S37-14.
  `map.tsv` in that folder records the region of every census row.
  Physical pages 64–100 hold references, rosters, the appendix evidence
  reviews and the publication notice. Of these pages, only page 82 holds
  statements with normative force (an FDA advisory and a caution about
  reading one finding). The MMWR access, contact and public-domain notices
  on page 100 stay outside the evidence.
- Formalization covers Box 3 at one document per recommendation: twelve
  documents, each preserving its recommendation's exact category/evidence
  parenthetical and simplified action rules. It adds one document per
  formalized recommendation 1–5 implementation-consideration region
  (67 documents). It adds one document per formalized front-matter and
  recommendation 1–5 rationale region (127 documents). It adds one
  document per formalized recommendation 6–12 body,
  implementation-consideration, special-populations, and conclusion
  region (176 documents). It adds one document per formalized region that
  the extraction census added (110 documents). The rules encode
  deontic modality ("should"), conditions, and negation where the source
  region states them. `audit/projection-notes.tsv` (kept/dropped columns)
  and `coverage.tsv` record each document's remaining simplifications per
  region.
- `coverage.tsv` records one status row per payload region (all 927), and
  every region carries a ruling. The statuses are:
  - `ace(<docid>)` — formalized (492);
  - `restates(<id>)` — verbatim or near-verbatim repetition of another
    region (246);
  - `uncovered(<class>: <reason>)` — deliberately not formalized (189;
    classes: heading, process, external, aim, descriptive, notice);
  - `pending` — ruled for formalization with the ACE document not yet
    authored (0).

  Restatement links target the canonical full statement: the Box 3 text
  for recommendation restatements, the main-text list for the guiding
  principles, and otherwise the earliest occurrence. A link may therefore
  point later in page order than an abbreviated echo. Links stay
  single-step: when a target is itself re-ruled `restates`, inbound links
  re-point to the transitive root (the echo relation is transitive). A
  region with its own ACE document keeps `ace(...)` even when it also
  repeats another region.
- Dose arithmetic. The MME conversion table on physical PDF page 33 and
  all of its notes form one region (S33-01), so its review page shows the
  source of every condition. Its document computes the MME dose of each
  listed opioid with an exact formula. The rules do not apply to a dosage
  that serves the management of opioid use disorder. The two worked
  examples under the table compute the totals that they state.
  No rule adds the doses of several opioids: a total dosage must come
  from the patient data.
- Pain classes. The document of S9-05 types acute, subacute and chronic
  pain by duration: a patient whose pain lasts less than 1 month, at least
  1 and at most 3 months, or more than 3 months has an acute, a subacute or
  a chronic pain. The rules of the other documents that name a pain class
  therefore also match a stated pain duration in months.
- Superseded statements. Recommendation 12 tells clinicians to obtain a
  waiver to prescribe buprenorphine (`cdc2022-opioid-rec12-imp08`). The
  Consolidated Appropriations Act, 2023 (Public Law 117-328), Division FF,
  section 1262(a)(1), eliminated this waiver requirement on December 29,
  2022 (89 FR 7528). The vocabulary table records the supersession of this
  document and of the passages S39-14, S59-02, S59-08 and S60-06, which
  describe the waiver in whole or in part. The knowledge base keeps these
  statements as the guideline states them. It derives the recommendations
  of `cdc2022-opioid-rec12-imp08` only for an evaluation date before
  2022-12-29 (`docs/REFERENCE.md`, section "Superseded statements").
  Passage S59-01 reports the breastfeeding recommendation of the American
  Academy of Pediatrics for people who receive treatment for opioid use
  disorder: it supports, considers or discourages breastfeeding by the
  time since the last return to drug use (90 or more days, 30 to 90 days,
  or less than 30 days). The CDC page "Opioid Use and
  Pregnancy" replaced it on 2024-09-26: breastfeeding is supported if
  nonprescribed substance use stopped by the time of delivery. No source
  states an operative date, so the table records the date of that page
  revision. S59-01 has no document, so this record changes no compiled
  statement. A check of every passage of the guideline against later law,
  regulation and guidance found no other superseded statement.
- Ruling tests. A CDC-voice imperative or endorsement with a trailing
  citation is the guideline's own directive: formalize it. A CDC-voice
  directive whose action is the guideline's own also formalizes when it
  names a supporting third-party document (S37-14 and its nine
  conditions).
  `uncovered(external)` is reserved for reporting frames that name a
  third-party body or document (S10-02, S31-06). A region that fuses
  several canonical duties without adding one takes
  `uncovered(descriptive: … canonical at <ids>)` (S8-04, S17-03, S22-06,
  S41-02). Weak-modality statements formalize: a possible claim ("can",
  "might be needed") as the claim under "can" with its qualifier, and
  optional advice ("might consider") as "may". A novel condition,
  permission, or strategy added to an otherwise-canonical echo makes the
  region formalize rather than restate (S49-14, S52-10, S55-01). A
  qualifier that the canonical statement does not have also makes the
  region formalize, and its document types the qualifier (S56-12). A
  capability or possibility claim ("can") that the canonical statement
  does not have also makes the region formalize, and its document states
  the claim under "can" (S44-13, S49-11, S50-03, S52-04, S54-08, S55-12,
  S57-02, S60-11).
- Projection rulings (wave-1 review). ACE "does not V a N" negates the
  existential — a takes-none reading (S37-13). A source preference
  ("preferred", "instead of", "rather than", "first-line", "first choice")
  types as `guideline_preference` (schema S1); "are recommended" renders
  as a should-shaped duty. A source prohibition ("should not V") renders
  as a recommendation over a negation: "it is recommended that the
  clinician does not V". The ACE form "should not V" states only that no
  duty to V exists. A hedge types as `guideline_qualifier` (schema
  S2) on the action that it qualifies ("typically", "generally",
  "ideally", "carefully"). A "might" claim renders as "can" with
  "possibly". Softened advice ("might consider") renders as "may". An
  applicability hedge ("as appropriate", "when feasible", "as needed")
  renders as a condition on the thing that it judges. A hedge on a thing
  ("effects that opioids might have") stays in its noun. Source or-alternatives may
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
  Batch-flag sentence counts are advisory; the source text governs rule
  count (S40-01). The wave-2 review also admitted structural
  approximations (identity splits, fresh witnesses, nominalizations,
  example drops) with a notes row, and a bare percentage as an at-least
  bound (S44-17). The current authoring rule replaces that admission:
  every meaning that the schema can carry is typed, and a notes row does
  not cure a typable loss. A meaning without a schema form, such as the
  identity of a referent across sentences, stays a gap, and the notes
  record it.
- Reconciliation against an independent 702-sentence census of the same
  pages is closed and recorded row-by-row in `audit/census-map.tsv`. 695
  census sentences map to evidence regions. The census exposed 27
  sentences as extraction gaps. The project added them as 25 regions, the
  largest being Box 2's "is" bullet list, and verified them with the same
  fidelity checks. 4 rows are page-assignment artifacts whose sentences
  occur only at pages that other census rows already cover. The p82
  caution maps to region S82-02. 3 rows are the excluded p100 notices.
- `audit/projection-notes.tsv` records, per ACE document, what the rule
  keeps from its source region and what it drops or interprets. That
  includes the regions that state a benefit, criticality, or weak
  possibility rather than an imperative; the projection renders a
  possibility as a claim under "can" with its qualifier, and a benefit or
  criticality as a clinician consideration, decision, or education
  action.
