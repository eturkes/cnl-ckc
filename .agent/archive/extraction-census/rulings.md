# Extraction census — CDC 2022 opioid guideline

Question: does `guidelines/cdc-2022-opioid/source/*.txt` (713 regions, extraction `5ad852e3`) hold every normative statement of `rr7103a1-H.pdf`? Answer: no. 234 of the 1,131 judged PDF spans outside it carry normative force; 137 of them = content a corpus round would author as ACE (≈68 content units, table below), 23 restate an extracted region, 74 would be ruled `uncovered`. Rubric + coding + adjudication fixed here; corpus changes = their own round.

## Method

- Candidates = `candidates.py`: pdftotext (poppler 26.10.0) of the source PDF, running heads dropped, pages joined; each of the 713 payloads mapped onto the text by alphanumeric key (710 whole, the S33-01 table by sentence pieces, the column-crossing S32-17 + S48-05 by 40-char windows; 40 key chars of S48-05 unmapped); every unmapped span split at sentence ends + bullets (`•`, sub-bullet `ï`) → 2,798 rows; mechanical classes = `reference-list` (p64 "References" .. p77, p90 "References" .. p99: 1,643), `roster` (p78: 4), `digits` (20); 1,131 rows judged. Known split: the `ï` of "naïve" also ends a row (X22-001/002, X32-006/007, X34-006/007).
- Rubric = `rubric.md`, fixed before coding: in-scope classes directive | action-claim | definition | scope | reported | aim (calibrated on the extracted regions, `uncovered` ones included); out-of-scope evidence | background | method | scaffolding | fragment; disposition ace | restates(<canonical id>) | uncovered(<ledger class>).
- Coding = blind dual: MAIN all 1,131 rows; coder 1 (general-purpose-1, GPT) pp1–42, coder 2 (general-purpose-2, GPT) pp43–100. Validator `validate.py`. Agreement (in vs out of scope): coder 1 0.899, κ 0.749; coder 2 0.915, κ 0.705 (`check.py` prints them).
- Adjudication = MAIN over every disagreement + every predicted `ace`: `adjudication.tsv` (96 rows; basis B = coder ruling accepted 71, A = MAIN kept after review 4, C = consistency ruling 21); `census.tsv` = result (basis M = MAIN's blind ruling unchanged, 1,035).
- Rules settled in adjudication (applied to every page):
  - R1 a bare pointer (resources, publications, tools exist or live at a URL) = scaffolding; reported = the content of third-party guidance, policy, labeling or advice.
  - R2 a patient characteristic → outcome association (risk factor, prognosis), cited or not = evidence; an action → outcome claim (can help or harm) = action-claim; inside an evidence-summary list = evidence.
  - R3 test, data-source and drug properties (detection limits, metabolites, PDMP coverage, pharmacology, formulation) = background; a stated consequence for care (false assurance, harm, confusion) = action-claim; co-medication respiratory risk = action-claim.
  - R4 workgroup statements on the recommendation itself (wording, category, structure, endorsement) = reported | uncovered(process); expert statements on practice = reported | uncovered(external).
  - R5 a sentence fusing extracted statements without new content = uncovered(descriptive) (README ruling test), never restates.
  - R6 notices + contact lines = scaffolding, imperative mood included.
  - R7 the p82 "Findings should be interpreted with caution" = directive | uncovered(process) (the README sweep records it as non-clinical).

## Counts (`census.tsv`)

| class | rows | | disposition | rows |
|---|---:|---|---|---:|
| directive | 20 | | ace | 137 |
| action-claim | 100 | | restates | 23 |
| definition | 39 | | uncovered(external) | 30 |
| scope | 20 | | uncovered(process) | 22 |
| reported | 45 | | uncovered(descriptive) | 14 |
| aim | 10 | | uncovered(aim) | 8 |
| out of scope | 897 | | — | 897 |

## Findings beyond the rows

- F1 pain classes: no document ties `acute-pain` (36 documents) / `chronic-pain` (22) to a duration. S9-05's passage defines acute <1 month, subacute 1–3, chronic >3 months; its ACE states applicability by duration and its projection note records "acute/subacute/chronic class names omitted" — a recorded loss the Authoring principle rules out. Rows X3-002, X3-010..012, X9-011, X19-008 restate S9-05.
- F2 S37-14: payload stops at "…before initiating changes when"; the nine conditions that follow (X37-006..014) were never extracted; the region is ruled `uncovered(external)` for its "Consistent with the HHS Guide …" frame. Contested: README ruling test 1 (CDC-voice directive with a citation → formalize) vs the S10-02 precedent (a CDC-voice directive naming a third-party document → external); coder 1 followed the precedent. Census disposition = ace, pending a user ruling.
- F3 superseded law: rec12-imp08 (`should obtain a waiver to prescribe buprenorphine`) + S39-14, S59-02, S59-08, S59-11 (external) state the DATA waiver, eliminated by Consolidated Appropriations Act 2023 §1262 (89 FR 7528, 2024-02-02: "Section 1262(a)(1) … enacted on December 29, 2022, amended the CSA (21 U.S.C. 823(h)) by eliminating the requirement that practitioners obtain a waiver"). Faithful to the guideline, wrong as current practice; the CDC guideline pages carry no note.
- F4 currency (2026-10-24): the CDC PDF refetched = byte-identical (sha256 f4e5098d…, 1,418,584 bytes); MMWR page `cdc:last_updated` November 3, 2022, no erratum; PubMed 36327391 no CommentsCorrections; CDC clinical-guidance hub (updated 2024-05-07) presents the 2022 guideline.

## Content units (`ace` rows grouped; ids = `census.tsv`)

| area | unit | rows |
|---|---|---|
| scope | applicable settings: offices, clinics, urgent care | X7-004 X9-008 |
| scope | audience relevance (prescribers) + team-based integrated care | X9-005 X9-006 |
| scope | covered acute conditions + procedural pain | X9-009 |
| scope | special populations addressed | X10-008 |
| scope | palliative care + end-of-life care defined (S3-03 exclusion terms) | X10-012 X10-013 X10-014 |
| scope | guideline-purpose can-claims | X8-007 X10-005 X62-016 |
| scope | opioids essential yet risky | X4-018 |
| rec 1 | nonopioids ≥ opioids for named acute conditions | X19-010 |
| rec 1 | evaluation + diagnosis can identify causes + interventions | X20-002 X20-003 X25-003 |
| rec 1 | nonpharmacologic approaches improve acute pain without serious harm | X20-004 |
| rec 1 | acute pain often managed without opioids / best with nonopioid medications | X20-008 X40-009 X42-001 |
| rec 1 | established acute migraine treatments | X20-013 |
| rec 1 | added opioids for superimposed acute pain → escalation risk | X22-002 |
| rec 1 | minimum needed use limits tolerance + withdrawal | X23-001 |
| rec 2 | fibromyalgia medication options | X24-002 X27-006 |
| rec 2 | headache or fibromyalgia: opioid initiation unlikely to outweigh risks | X24-004 X28-004 |
| rec 2 | nonopioid-first strategy for moderate–severe back, hip, knee OA pain | X25-008 |
| rec 2 | NSAIDs + tricyclics potentially inappropriate in older adults | X25-009 X27-007 |
| rec 2 | nonpharmacologic therapy benefits (condition list, TMD, active therapies, activity, low-cost programs) | X25-010 X25-012 X26-001 X26-002 X26-003 X26-008 |
| rec 2 | nonopioid medication risks in older adults, pregnancy, comorbidities | X26-010 |
| rec 2 | a documented treatment plan clarifies expectations | X28-009 |
| rec 2 | multimodal rehabilitation, synergy, medication combinations | X30-005 X30-006 X30-009 |
| rec 3 | ER/LA opioids defined | X31-001 |
| rec 3 | abuse-deterrent label ≠ no risk | X31-008 |
| rec 3 | time-scheduled use → higher daily dosage | X31-005 |
| rec 3 | transdermal fentanyl units + methadone/fentanyl complexity risks | X31-015 X31-016 |
| rec 4 | risk rises continuously with dosage, no safe threshold | X32-005 X33-010 |
| rec 4 | starting dose ≈ 5–10 MME per dose, 20–30 MME/day | X32-007 X34-007 |
| rec 4 | many patients gain nothing at ≥ 50 MME/day | X32-009 |
| rec 4 | diminishing returns beyond 50 MME/day | X32-010 X34-012 |
| rec 4 | lower-dose formulations; rapid increases raise risk | X34-009 X34-011 |
| rec 4 | buprenorphine lacks the continuous dose–overdose relation | X34-001 |
| rec 5 | tapering success: agreement, collaboration, integrated therapies | X35-003 X38-001 X38-006 X38-007 |
| rec 5 | continuing avoids tapering risks; reduction might provoke anxiety | X37-003 X37-015 |
| rec 5 | the nine "consider tapering when" conditions (F2) | X37-006..X37-014 |
| rec 5 | sudden discontinuation + rapid tapering harms | X39-001 X39-002 X41-015 |
| rec 5 | withdrawal medications (alpha-2 agonists, symptom-specific) | X39-004 X39-005 X39-007 |
| rec 5 | buprenorphine properties + pain dosing frequency | X39-009 X39-010 X40-003 |
| rec 6 | opioids sometimes needed; longer courses for prolonged severe pain | X40-010 X40-012 |
| rec 6 | short duration avoids tapers; inpatient course predicts discharge + taper needs | X41-016 X42-002 X42-005 |
| rec 7 | reassessment within 1 and 3 months offers chances to act | X43-006 X44-001 |
| rec 8 | safety critical job defined | X48-012 |
| rec 8 | benzodiazepine co-use respiratory risk | X48-016 X53-004 X55-004 |
| rec 8 | treating depression helps; naloxone reverses respiratory depression | X48-017 X50-001 |
| rec 8 | punitive approaches deter care in pregnancy | X47-006 |
| rec 9 | dismissal on PDMP information harms | X50-009 X51-008 |
| rec 9 | PDMP uses + risk-score bias | X51-005 X51-007 X51-010 |
| rec 9 | ED severe acute pain during PDMP outage: an example exception | X52-001 |
| rec 10 | dismissal after toxicology results harms | X52-004 X54-011 |
| rec 10 | toxicology testing benefits, harms + practice (discussion first, stigma, false results, fentanyl, policies) | X53-001 X53-005 X53-006 X53-010 X53-011 X53-013 X54-001 X54-003 X54-009 |
| rec 11 | concurrent-use risk modifiers (unpredictable use, higher doses, alcohol) | X55-002 |
| rec 11 | benzodiazepines can help; stopping destabilizes; untreated OUD outweighs combined risk | X55-003 X56-001 X56-002 |
| rec 11 | taper-order considerations; CBT aids benzodiazepine tapers | X56-003 X56-005 |
| rec 12 | stigma + recovery framing | X56-008 |
| rec 12 | OUD defined: DSM-5 11 criteria, supervised-use exception, severity grades | X57-001 X57-005..X57-018 |
| rec 12 | FDA-approved OUD medications | X57-019 |
| rec 12 | naltrexone generally precludes opioid pain medications | X58-002 |
| close | strategies against inequities | X62-004 |

## Rerun

`python3 -I .agent/archive/extraction-census/check.py .` → candidates sha256 ok (regenerated from the PDF + the guideline at the census commit 6646052a), validator green, census = codings + adjudication row for row, agreement lines, `map:` line (the CR round: each in-scope row → a region of the live `coverage.tsv` with a ruling, `map.tsv`); rc 0. Firing: one `census.tsv` class edit → `rebuild: 1 rows differ`, rc 1; one dropped `map.tsv` row → `map: … unmapped=1`, rc 1. A different pdftotext build can shift spans → candidates sha256 `DIFFERS`.

## Limits

- Judgment classes; the validator checks grammar + canonical targets only. Dispositions predict a round's rulings; the round rules.
- Sentence segmentation follows pdftotext reading order: boxes, tables + footnotes can glue or cut rows (fragments named in their reasons).
- Mechanical classes (references, rosters, digits) were not read row by row; their sections were located by heading offsets.
- DSM-5 criteria (X57-*) quote the APA manual inside the public-domain MMWR text: rights check before verbatim evidence.
