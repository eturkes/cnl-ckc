# MRG — release-merge rehearsal + runbook (contract)

Row: Tasks MRG. Rulings: Decisions Publication scope (the release on the line → its merge into main), Schema evolution (the line stays at identifier `1`, main at `2`), Review custody, IMP accepted risks (main's labels go stale). Origin: pre-mortem consultant-11 F3 (operability) + consultant-12 D8 (ID2 review): a bare recompile after the merge leaves products stale or loses a correction. Runs after ID2 (main at `2`) and again after DC (whole-statement guards).

## Tier

data + docs (a disposable rehearsal script + the REFERENCE runbook); shallow ⇒ MAIN alone; `reviewer` on the runbook + script (the release path has no downstream re-check before the user's merge).

## Design

- D1 Refs + binaries frozen per run: L0 = the cut commit e4c26978 (identifier `1`); the line's own build (a schema-1 `ckc` built from L0 in a disposable worktree; never main's binary as the line's compiler); M0 = main's tip (identifier `2`) + its build. The schema-1 build shares `.toolchain/` while the cut's pins equal main's (ops.md Release lines: a pin bump on main first gives the line its own store). Scope stated in the script header + runbook: a schema + custody rehearsal, not a replay of every line-tip conflict (the real line tip carries publication + tooling commits the rehearsal does not recreate).
- D2 Line commits on L (in a disposable clone under `.scratch/`): (a) a ledger batch (one approved row for document A, its `ace_commit` = L's commit); (b) a corrected document B (ACE edit + its schema-1 recompile + queries + review manifest + `just amend-manifest` with the line's binary); (c) an ACE correction to document C that main also edits differently (a real ACE conflict); (d) an auto-merging correction to document D (no conflict on main).
- D3 Main commits on M: an ACE edit to C (the overlapping edit).
- D4 Runbook (REFERENCE § Releasing, merge step; this unit writes it): on M, `git merge --no-ff L`; resolve: compiled products, queries, answers, traces, review + release manifests → take neither side's bytes; ACE conflicts → a semantic ruling (the user's, recorded; the rehearsal applies a scripted choice); then with main's binary: `ckc compile` + `ckc queries` for every guideline the merge touches, `ckc review-manifest`, commit the merge, `just amend-manifest` (amends the merge commit, both parents kept), `ckc certify`, `ckc check`.
- D5 Asserts (the script; each a named check, rc 1 on failure): the merge commit has exactly two parents (M, L); L's ref + its archive hash (`ckc dist build` at L, built by the line's binary) unchanged before vs after; every merged product carries identifier `2` (documents) / `v2` (questions); document B's correction + document D's correction present in the merged ACE; C's ACE = the scripted ruling; the ledger row for A byte-identical, its recorded commit still an ancestor of the merge and still attesting (bundle v2 derived at that commit = the row's digest), while A's current label on main reads stale; `ckc check` + `ckc certify` green on the merge.
- D6 Re-run after DC: the same script on DC's tip (guards change every scoped document's clauses ⇒ every current label stale; the asserts unchanged).

## Predicates

- P1 The rehearsal script rc 0 with every D5 assert, on ID2's tip; rerun on DC's tip rc 0.
- P2 Each assert fires: a planted wrong resolution per assert (a dropped correction, a product kept at `1`, a rewritten ledger row, a one-parent merge, a moved line ref) → rc 1 naming it.
- P3 The runbook = the commands the script runs, in order (consistency pass by the reviewer).
- P4 `just gate` green (docs + script only; the release line + its worktree untouched).
