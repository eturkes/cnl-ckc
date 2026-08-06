# rounds — /goal execution map

Token economics (user-set): GPT teammate capacity abundant; Claude (MAIN)
window scarce and the superior orchestrator → MAIN buys orchestration,
rulings, faithfulness review, gate reruns, commits; every bulk read, draft,
sweep, and cross-check routes to a teammate. Fund speculative teammates
freely (extra audits, probes, cross-checks) — teammate cost ≈ 0, MAIN
attention = the budget. Small rounds (single-statement fix-ups) run
MAIN-direct; spawning must buy real bulk.

Mechanics (worktrees, briefs, boundary blocks, markers, roster, gauges,
trust shapes, harvest/TaskStop) = `/session-prompt` execution map + global
`CLAUDE.md` Subagents. Round roles:

- `extract-<id>-<k>` — bulk-reads the source document; drafts verbatim
  extraction evidence with page/byte anchors + a normative-statement
  inventory (every statement found, each anchored). Raw material +
  attention-directing: MAIN spot-checks verbatim fidelity against the
  source before adopting anything.
- `ace-<id>-<k>` — drafts ACE documents for a MAIN-ruled statement batch
  per the corpus modeling pattern, compile+check-validated in its worktree;
  proposed lexicon additions ship as a reported delta only. Faithfulness =
  MAIN review, statement by statement.
- `rev-<id>-<k>` — adversarial: ACE↔extraction fidelity, coverage-claim
  soundness, guideline README claims, CLAUDE.md conformance; findings as
  `file:line` reports; disputed semantics ⇒ MAIN rules.
- `audit-<id>-<k>` — diff-blind completeness sweep: normative statements in
  the source vs `ace/` inventory vs README uncovered-list; feeds the
  coverage statement and the /goal stop check.
- `res-<k>` — fetch rounds only: candidate discovery, rights/licensing
  quotes, URL verification (explicit WebSearch allowance; authenticated-web
  route per global `CLAUDE.md`).

MAIN-retained: `ace_to_pl.pl` + lexicon edits, all primary-tree writes,
guideline README + queue updates, id choice, rights rulings, statement
batch verdicts, gate reruns, commits.

Round flow: spawn `extract` + `audit` early (the bulk read happens once,
reused); MAIN batch-rules the statement inventory (project / uncovered with
reason); `ace` teammates draft against ruled batches while MAIN reviews the
previous batch; MAIN authors the lexicon delta, compiles, runs check; `rev`
attacks the assembled round pre-commit; MAIN fixes, reruns gates, commits.

Round close (standing loop and one-round invocations alike): gate green
(`export PATH="<repo>/.toolchain/bin:$PATH"` → `python3 -P tools/goal.py
compile <id>` + `check`) → ledgers current (coverage.tsv, `audit/`,
guideline README, `.agent/queue.md` — pending list stays ordered, next
round first) → scoped commit(s) on main → every teammate harvested (report
read, artifacts byte-verified or re-derived) + TaskStopped, its watcher
killed, worktree removed + `wt/<name>` branch deleted → roster updated. A
blocked round closes the same way with `blocked(<why>)` recorded in
queue.md.
