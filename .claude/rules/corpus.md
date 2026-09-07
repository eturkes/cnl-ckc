---
paths:
  - "guidelines/**"
  - ".agent/compendium.md"
  - ".agent/compendium.tsv"
  - ".agent/queue.md"
---

# Corpus authoring law

Protocol = docs/REFERENCE.md § Operating; supported ACE constructs = the `ace_to_pl.pl` header. Compendium + harvest technique: read `.agent/archive/harvest-technique.md` before compendium edits, queue promotion, or fetch/access work.

- ACE facts: noun copula needs the indefinite article (`X is a noun.`); unknown content words without a ulex entry surface in `ape_messages`; consult `audit/lexicon-rejects.tsv` before proposing lexicon entries (lexicon law fires on ulex touch).
- Coverage ledger (`check_coverage`): pinned 3-line generic header (further `#` lines before rows only), 5-col rows, status grammar `ace(<docid>)`/`restates(<id>)`/`uncovered(<class>: <reason>)`/`pending`, classes heading|process|external|aim|descriptive|notice. Per-file region closure against each evidence file's `identify the N payloads below` census + bracketed `[<id> | ...]` locator ids (locator-less evidence closes on count alone); ace↔docid bijection; restates = single-step to a non-restated existing target; meter `goal: coverage ok <id> <n> regions; ace=<a> restates=<r> uncovered=<u> pending=<p>`. Document completion = check green ∧ pending=0.
- Inexpressible protocol (interim until roadmap M7): a normative statement the controlled language cannot express ⇒ region stays `pending`, record an `inexpressible` blocker in `.agent/queue.md`, guideline row goes blocked, round moves on — blocked rows stay terminal for the exhaustion clause, and the blockers bank the M7 gap census.
- Align authoring (`goal.py align <gid> <docid>` stdin rows `group<TAB>side<TAB>occurrence<TAB>span`): occurrence = 1-based raw-substring count, non-overlapping left-to-right, INCLUDING hits inside longer words (ace `opioid-therapy` occ1 sits inside `nonopioid-therapy`; standalone = occ2). Rubric = REFERENCE Operating step 4. Every ACE doc needs an align file (`check_alignment` inventory gate); any ACE/passage edit → re-author (render stops on stale offsets). Exemplars: `cdc-2022-opioid` rec01 (sub-span groups; g7 src occ1+occ3 with an overlap-partitioning middle), rec08-imp13 (2-src-span group). Content gate = check_ui corpus render.
- `rights.tsv`: header `profile	statement	url	retrieved	note`; profiles redistributable|reconstructable|restricted; row 1 = operative.
- Committed corpus + query artifacts are byte-stable while M5 is open, except plan-ruled re-pins.
