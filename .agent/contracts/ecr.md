# ecr — the CDC extraction-census corpus round (Tasks CR; contract; archived at round close)

Row = Tasks CR: every in-scope row of `.agent/archive/extraction-census/census.tsv` (234) → an evidence region + its coverage ruling (the census disposition = a proposal; the round rules), ACE for each `ace` ruling (compiled, aligned, certified), F1 (S9-05 class names), F2 (S37-14 + its nine conditions), the waiver caveat (Decisions Supersession), the README coverage statement corrected.

## Tier

`data` (corpus ACE; clinician review, `ckc check`, `ckc certify` re-validate). MAIN drafts + rules every document; one closing `reviewer` covering L0–L6 (`.claude/rules/corpus.md`) before the closing commit.

## Design

- D1 Evidence: 214 new regions in page order at the end of each page block (precedent cde2ff20), ids = next-free seq per page (retired ids unreused), one payload line each, PDF text verbatim with line-end hyphens resolved against the HTML manifestation (`.scratch/m3u6/live-rr7103a1.txt`; boxes + appendix by the PDF layout text); S37-14's payload continues with its nine conditions; the DSM-5 list = one composite region S57-05 (stem + 11 criteria, labels dropped as list scaffolding; user ruling: verbatim); p82 → new file `source/appendix-extraction.txt`; `.agent/archive/extraction-census/map.tsv` = census row → region (234 rows); `audit/census-map.tsv` p082.C01 → S82-02.
- D2 Rulings: README ruling tests over the census proposal. A second occurrence restates the first (X28-004, X34-007, X34-012, X41-015, X42-001, X54-009, X54-011, X56-001, X27-004, X9-008, X8-008); a subset restates its superset (X38-006 → S38-25); a possibility claim the target never types formalizes (X37-005, X39-008: census `restates` → `ace`).
- D3 ACE: one document per `ace` region, docid = region id lowercased; knowledge-only; corpus.md authoring law (prohibitions = SHOULD over a negation, hedges = qualifier rows or `can`, absence conditions = `does not provably`, generic role claims quantify, examples = inert example facts); projection-notes row + align table per document; lexicon deltas screened against `audit/lexicon-rejects.tsv`; every new `vocabulary.tsv` row lands in one batch (table-repin.py once).
- D4 F1: S9-05 types the class names: `If a patient has a pain that lasts for <bound> then the patient has a <class>-pain.` ×3 (the carrier that fires the `has an acute-pain` conditions; a copula consequent yields a `be` edge to a fresh entity that no condition reads); the applicability sentences name the classes; the notes row drops the recorded loss.
- D5 F2: S37-14 = one rule per condition, consequent `should consider` the taper options + `should discuss` them with the patient; absence-of-record conditions (no evidence of benefit) = `does not provably`, observed findings classical (CS2b table).
- D6 Ledgers at close: guideline README (Source of record, Coverage counts + statement: extraction reconciled against two censuses, pp64–100 sweep corrected; waiver caveat), `docs/m7-gap-taxonomy.md` + root README counts that name the document total, `.agent/queue.md` entry dropped, compendium row `done`, Tasks CR ticked.

## Predicates

- P1 `map.tsv`: 234 in-scope rows, each region a `coverage.tsv` row with a nonempty status (`check.py` map pass, rc 0; firing: a dropped map row → rc 1).
- P2 `ckc check` green; coverage meter `pending=0`; review + release manifests current.
- P3 `ckc certify cdc-2022-opioid` green over every document.
- P4 F1 differential under the consumer recipe: a patient world `has a pain that lasts for exactly 4 months` ↔ `has a chronic-pain` → equal `should` answer sets over the rules conditioned on `chronic-pain`.
- P5 S37-14: nine condition rules, each deriving the consider + discuss duties.
- P6 `ckc queries cdc-2022-opioid` re-derived; each committed answer change listed in the commit body.
- P7 `cargo test --test consumer_recipe` green.
- P8 `just gate` (or `just ci <unit>`) green on a clean tree at the closing commit.

## Rulings

- R1 (user, AskUserQuestion): the DSM-5 criteria rows (X57-005..018) = verbatim regions + ACE; the source PDF ships unchanged (Decisions Corpus).
