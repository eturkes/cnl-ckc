Continue project. First load `.agent/roadmap.md` + `.agent/memory.md`; either missing ⇒ create a minimal stub (roadmap: goal + first UNPLANNED milestone from available context; memory: empty) before proceeding. Task present ⇒ execute exactly it; update other files as needed for consistency. Task empty ⇒ run MODE for the active milestone (first awaiting DONE/REVIEWED).

MODE ← active-milestone status (state-changing closes use a scoped commit; an unchanged BLOCKED recheck closes read-only; convention below):
- UNPLANNED (incl. a still-unsplit future milestone) → PLANNING
- IN-PROGRESS (has an OPEN unit) → WORK-UNIT (lowest OPEN unit)
- IN-PROGRESS (OPEN=0; BLOCKED>0) → WORK-UNIT (lowest BLOCKED, gate recheck)
- IMPLEMENTED (all units DONE; review pending) → MILESTONE-REVIEW

Execution map:
- Every MODE fans work out to subagents (this command = standing opt-in): private/unnamed agents for one-shot scopes (research, review, fix batches); a named teammate where iterating on retained context pays (WORK-UNIT implementation). Batch independent dispatches in one block so they run in parallel. A supplied task uses the same fan-out when MAIN judges it beneficial or the user requests it.
- MAIN owns the one-line scope + acceptance restatement, precondition confirmation, per-agent task definitions, and close; MAIN alone creates repository commits. MAIN verifies independently: inspects every returned diff, reruns the decisive gates, and accepts evidence that traces permitted real inputs.
- Implementation agents land the accepted scope as working-tree changes: reuse project modules/style, run the required lint/format/type-check/tests, confirm touched scripts exit cleanly, route durable guidance, and return diff + evidence. Parallel implementation agents only on disjoint file sets.
- Review agents stay analysis-only; findings return to MAIN.
- Agent hygiene: scope every agent to finish inside ~200K; web-research agents get a bounded question set + an explicit WebSearch allowance (session budget = 200, shared) + BrowserOS MCP for authenticated/paywalled sources; a missing/empty result ⇒ check the transcript tail before re-dispatching; teammates signal nothing on completion/death ⇒ poll the teammate transcript for its assigned completion marker, revive an API-dead teammate via `SendMessage`; resolve every agent (result, or `TaskStop` for teammates/stragglers) before close.
- Context: PLANNING + MILESTONE-REVIEW run past auto-compaction — MAIN checkpoints coherently before each and continues after. Every other run completes within one window, and WORK-UNIT records its usage at close.

PLANNING — split scope into milestones as needed; plan the next milestone.
- Read the prior milestone's commit range and recorded `impl=` context; for the first planned milestone, read the scope-seed commit(s) named by the roadmap. Size future units from implementation usage; treat `main=` as coordination overhead.
- MAIN confirms each milestone precondition through project pipeline/tooling. Met ⇒ clear stale standing block + continue. Unmet ⇒ record standing block + evidence; changed record ⇒ commit `roadmap (M<m> block): …`; unchanged record ⇒ read-only close.
- Fan out parallel research agents over the open questions, one bounded question set each; MAIN discovers code via Serena (`get_symbols_overview` → `find_symbol` → `find_referencing_symbols`), then reconciles `git status`.
- Break the milestone into units that each project to fit one implementing agent inside the one-window aim; planning holds sole sizing authority, so every unit ships as scoped. Sequence gate-independent prep first; mark a gated unit BLOCKED until its precondition is met.
- Close: set the milestone IN-PROGRESS (units enumerated), commit `roadmap (M<m> plan): …`.

WORK-UNIT.
- Read the last completed unit's commit(s), or the planning commit(s) for the milestone's first unit.
- Precondition transition: recheck BLOCKED first. Met ⇒ clear block, set OPEN, continue. Unmet ⇒ retain BLOCKED; materially changed evidence ⇒ update + commit `roadmap (M<m>.<u> block): …`; stable evidence ⇒ read-only close. OPEN + unmet ⇒ set BLOCKED, record condition/evidence, make block commit, close.
- Implement: one implementation teammate (named `impl-m<m>u<u>`) carries the accepted scope, locations, constraints, quality gates + acceptance checks; its report ends with marker `UNIT-DONE`.
- Review: a fresh private review agent scrutinizes the returned diff adversarially (correctness/spec, claim soundness, guarantee-vs-claim gaps) against scope + acceptance + project conventions; MAIN validates findings and `SendMessage`s accepted fixes to the implementation teammate (retained unit context; fix reports end `FIXES-DONE`), re-reviewing material changes; MAIN reruns the decisive gates and `TaskStop`s the teammate before close.
- Close: record `main=<.agent/context.sh full pct used/240K>` + `impl=<peak implementing agent's transcript final pct used/240K>` in the roadmap. Set the unit DONE and, once all units are DONE, the milestone IMPLEMENTED; commit `<scope> (M<m>.<u>): …`.

MILESTONE-REVIEW.
- Read every commit within the milestone.
- Fan out parallel review agents: one per unit (that unit's commits + touched surfaces; correctness/spec) + one cross-cutting (cross-unit integration; instruction/memory conformance; token-efficiency/obsolescence). Each finding supplies severity + `file:line` + divergence + impact + acceptance check.
- MAIN validates + deduplicates findings; accepted implementation findings become one agent task per cohesive fix batch, carrying locations + acceptance checks.
- A requirement-changing design reaches the user before any scope-source edit.
- Close: set the milestone REVIEWED, commit `<scope> (M<m> review): …`. The next session plans the next milestone.

Commit convention — scoped (`<scope>: …`), trace key in parens: unit `(M<m>.<u>)`, block `(M<m> block)` / `(M<m>.<u> block)`, plan `(M<m> plan)`, review `(M<m> review)`. Grep a milestone's history: `git log --grep "(M<m>[. ]"`.

Task: $ARGUMENTS
