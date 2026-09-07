---
paths:
  - "guidelines/**"
  - ".agent/compendium.md"
  - ".agent/compendium.tsv"
  - ".agent/queue.md"
---

# Corpus authoring law

Protocol = docs/REFERENCE.md § Operating; supported ACE constructs = the `ace_to_pl.pl` header. Compendium + harvest technique: read `.agent/archive/harvest-technique.md` before compendium edits, queue promotion, or fetch/access work.

## Round roles (corpus `/goal` loop)

Token economics: teammate capacity abundant, MAIN window scarce → MAIN buys orchestration, rulings, faithfulness review, gate reruns, commits; every bulk read, draft, sweep + cross-check routes to a teammate; single-statement fix-ups run MAIN-direct. Mechanics = global `CLAUDE.md` `Subagents`.
- `extract-<id>-<k>` — bulk-reads the source; verbatim extraction evidence with page/byte anchors + an anchored normative-statement inventory. Raw material: MAIN spot-checks verbatim fidelity before adopting.
- `ace-<id>-<k>` — drafts ACE for a MAIN-ruled statement batch: knowledge-only (no witness seed facts, proper-name stand-ins, authored queries), validated in its worktree by `compile` + `check`; lexicon additions ship as a reported delta screened against `audit/lexicon-rejects.tsv` (MAIN appends each refusal). Faithfulness = MAIN review, statement by statement.
- `rev-<id>-<k>` — adversarial: ACE↔extraction fidelity, coverage-claim soundness, README claims, knowledge-only status, obligation count + discharge, `audit/projection-notes.tsv` fidelity; `file:line` findings; disputed semantics ⇒ MAIN rules.
- `audit-<id>-<k>` — diff-blind completeness sweep: source normative statements vs `ace/` inventory vs README uncovered-list; feeds the coverage statement + the stop check.
- `res-<k>` — fetch rounds only: candidate discovery, rights/licensing quotes, URL verification (explicit WebSearch allowance; authenticated-web route per global `CLAUDE.md`).
- MAIN-retained: `ace_to_pl.pl` + lexicon edits, all primary-tree writes, `tools/goal.emm` validator pins, guideline README + queue updates, id choice, rights rulings, statement batch verdicts, gate reruns, commits.
- Flow: spawn `extract` + `audit` early (one bulk read, reused); MAIN batch-rules the inventory (project / uncovered with reason); `ace` drafts against ruled batches while MAIN reviews the previous; MAIN authors the lexicon delta, compiles, checks; `rev` attacks the assembled round pre-commit; MAIN fixes, reruns, commits. Teammates author + compile only — N concurrent in-worktree `goal.py check` runs thrash the machine (10-min subprocess caps trip); MAIN runs the one decisive gate at harvest.
- Round close = global Close order bound here: harvest (report read + artifacts byte-verified or re-derived); ledgers current before the gate (coverage.tsv, `audit/`, guideline README, `.agent/queue.md` pending list ordered, next round first); decisive gate = `python3 -P tools/goal.py compile <id>` + `check` under the toolchain PATH; scoped commit(s) on main. A blocked round closes the same way with `blocked(<why>)` in queue.md. Replayable wave kit exemplar = `.scratch/goalrounds/ace-wave2/` (`merge_wave.py`).

## Authoring law

- ACE facts: noun copula needs the indefinite article (`X is a noun.`); unknown content words without a ulex entry surface in `ape_messages`; consult `audit/lexicon-rejects.tsv` before proposing lexicon entries (lexicon law fires on ulex touch).
- Coverage ledger (`check_coverage`): pinned 3-line generic header (further `#` lines before rows only), 5-col rows, status grammar `ace(<docid>)`/`restates(<id>)`/`uncovered(<class>: <reason>)`/`pending`, classes heading|process|external|aim|descriptive|notice. Per-file region closure against each evidence file's `identify the N payloads below` census + bracketed `[<id> | ...]` locator ids (locator-less evidence closes on count alone); ace↔docid bijection; restates = single-step to a non-restated existing target; meter `goal: coverage ok <id> <n> regions; ace=<a> restates=<r> uncovered=<u> pending=<p>`. Document completion = check green ∧ pending=0.
- Inexpressible protocol (interim until M7 gap coverage, `spec.md` Deferred): a normative statement the controlled language cannot express ⇒ region stays `pending`, record an `inexpressible` blocker in `.agent/queue.md`, guideline row goes blocked, round moves on — blocked rows stay terminal for the exhaustion clause, and the blockers bank the M7 gap census.
- Align authoring (`goal.py align <gid> <docid>` stdin rows `group<TAB>side<TAB>occurrence<TAB>span`): occurrence = 1-based raw-substring count, non-overlapping left-to-right, INCLUDING hits inside longer words (ace `opioid-therapy` occ1 sits inside `nonopioid-therapy`; standalone = occ2). Rubric = REFERENCE Operating step 4. Every ACE doc needs an align file (`check_alignment` inventory gate); any ACE/passage edit → re-author (render stops on stale offsets). Exemplars: `cdc-2022-opioid` rec01 (sub-span groups; g7 src occ1+occ3 with an overlap-partitioning middle), rec08-imp13 (2-src-span group). Content gate = check_ui corpus render.
- `rights.tsv`: header `profile	statement	url	retrieved	note`; profiles redistributable|reconstructable|restricted; row 1 = operative.
- Committed corpus + query artifacts are byte-stable while M5 is open, except plan-ruled re-pins.
