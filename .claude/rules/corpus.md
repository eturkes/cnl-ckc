---
paths:
  - "guidelines/**"
  - ".agent/compendium.md"
  - ".agent/compendium.tsv"
  - ".agent/queue.md"
---

# Corpus authoring law

Protocol = docs/REFERENCE.md § Operating; supported ACE constructs = the `ace_to_pl.pl` header. Compendium + harvest technique: read `.agent/archive/harvest-technique.md` before compendium edits, queue promotion, or fetch/access work.

## Round roles (corpus loop)

MAIN drafts every ACE document; teammates read + attack (triggers, offload economics + mechanics = global `CLAUDE.md` `Subagents`).
- MAIN — ACE per ruled statement batch: knowledge-only (no witness seed facts, proper-name stand-ins, authored queries), validated by `compile` + `check`; lexicon deltas screened against `audit/lexicon-rejects.tsv` (MAIN appends each refusal). Faithfulness = MAIN review, statement by statement. Also MAIN: `ace_to_pl.pl` + lexicon edits, all primary-tree writes, `ckc check` validator pins (`ckc-spec` + fixtures), guideline README + queue updates, id choice, rights rulings, statement batch verdicts, gate reruns, commits.
- `researcher`, extraction — bulk-reads the source; verbatim extraction evidence with page/byte anchors + an anchored normative-statement inventory. Raw material: MAIN spot-checks verbatim fidelity before adopting.
- `researcher`, fetch rounds — candidate discovery, rights/licensing quotes, URL verification (explicit WebSearch allowance; authenticated-web route per global `CLAUDE.md`).
- `reviewer` = the round's closing one (`CLAUDE.md` `Session flow`: one covering every lens; user ruling: L0 folds in) over 7 lenses; `file:line` findings; disputed semantics ⇒ MAIN rules.
  - L0 completeness: source normative statements vs `ace/` inventory vs README uncovered-list; grades the coverage statement + the stop check.
  - Adversarial: L1 ACE↔extraction fidelity, L2 coverage-claim soundness, L3 README claims, L4 knowledge-only status, L5 obligation count + discharge, L6 `audit/projection-notes.tsv` fidelity.
- Flow: spawn extraction early; MAIN batch-rules the inventory (project / uncovered with reason), drafts ACE per ruled batch, authors the lexicon delta, compiles, checks; the closing `reviewer` attacks the assembled round on L0–L6 pre-commit; MAIN fixes, reruns, commits. Teammates never run `ckc check` — N concurrent in-worktree runs thrash the machine (subprocess wall caps trip); MAIN runs the one decisive gate at harvest.
- Round close = `ops.md` Close order (stage, then the decisive check) bound here: harvest (report read + artifacts byte-verified or re-derived); ledgers current before the gate (coverage.tsv, `audit/`, guideline README, `.agent/queue.md` pending list ordered, next round first); decisive gate = `ckc compile <id>` + `ckc queries <id>` + `ckc check` under the toolchain PATH; scoped commit(s) on main; an input-touching commit carries its manifest (commit → `ckc release-manifest` → `git commit --amend`). A blocked round closes the same way with `blocked(<why>)` in queue.md.

## Authoring law

- ACE facts: noun copula needs the indefinite article (`X is a noun.`); unknown content words without a ulex entry surface in `ape_messages`; consult `audit/lexicon-rejects.tsv` before proposing lexicon entries (lexicon law fires on ulex touch).
- Coverage ledger (`check_coverage`): pinned 3-line generic header (further `#` lines before rows only), 5-col rows, status grammar `ace(<docid>)`/`restates(<id>)`/`uncovered(<class>: <reason>)`/`pending`, classes heading|process|external|aim|descriptive|notice|inexpressible. Per-file region closure against each evidence file's `identify the N payloads below` census + bracketed `[<id> | ...]` locator ids (locator-less evidence closes on count alone); ace↔docid bijection; restates = single-step to a non-restated existing target; meter `ckc: coverage ok <id> <n> regions; ace=<a> restates=<r> uncovered=<u> inexpressible=<i> pending=<p>`. Document completion = check green ∧ pending=0.
- Inexpressible protocol: a normative statement the controlled language cannot express ⇒ region `uncovered(inexpressible: <reason>)`, round moves on; the meter's `inexpressible=<i>` banks the gap census (classes + dispositions: `docs/m7-gap-taxonomy.md`).
- Align authoring (`ckc align <gid> <docid>` stdin rows `group<TAB>side<TAB>occurrence<TAB>span`): occurrence = 1-based raw-substring count, non-overlapping left-to-right, INCLUDING hits inside longer words (ace `opioid-therapy` occ1 sits inside `nonopioid-therapy`; standalone = occ2). Rubric = REFERENCE Operating step 4. Every ACE doc needs an align file (`ckc check` align inventory); any ACE/passage edit → re-author (render stops on stale offsets). Exemplars: `cdc-2022-opioid` rec01 (sub-span groups; g7 src occ1+occ3 with an overlap-partitioning middle), rec08-imp13 (2-src-span group). Content gate = `ckc ui check` corpus render.
- Temporal table (`guidelines/<id>/temporal.tsv`; grammar = `docs/REFERENCE.md` § Schema v2 + § Schema v3): its header selects v2|v3 for every document + question of the guideline (the CDC table = v3); any byte change rewrites every record digest → recompile the whole guideline. v3 shapes: `before|after <plain referent>` (order, no bound), `<verb> N <items> per 1 <unit>` (count per period: counted item = the verb's object), `at an interval of … during a window of <cmp> N <unit> of <anchor>` (scoped recurrence: every spacing pp of that event becomes `guideline_recurrence_window`); a unit abbreviation (`d`) = ulex count noun + `unit d day` row. Author timing as `for|during <cmp> N <unit>` (duration), `after|before <cmp> N <unit> of <anchor>`, `within <cmp> N <unit> of <anchor>` (a forward window = an `after` + `within` pair on one anchor), `at an interval of <cmp> N <unit>` (spacing between occurrences; `at most` = maximum gap, never a count per window). Source text with no integer bound (several, a few, approximately N, ranged minima like at least 7–10 days) stays lexical + recorded in projection notes; never invent an endpoint.
- `rights.tsv`: header `profile	statement	url	retrieved	note`; profiles redistributable|reconstructable|restricted; row 1 = operative.
- Committed corpus + query artifacts change only through `ckc compile` / `ckc queries` from their ACE + lexicon inputs, or a ruled re-pin script under `.agent/archive/migrations/`.
