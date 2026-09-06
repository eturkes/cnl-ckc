# polish register

Register drained at the final project review: every row landed in-session
(bounded swipl wall clock, ledger-validate foreign-cwd, shared 250-byte docid
bound, GET Host check, census-map gate). Git history holds the rows dropped at
the feature-complete ruling.

- `goal align` fail-path probes: the resolver's `fail("align", …)` branches
  (bad int, unknown side, occurrence miss, overlap, one-sided) have no
  fixture coverage; render-side twins exist (`tests/ui/red/align-*` +
  selftest pins). Acceptance: a probe script drives `goal.py align` over one
  input per branch and asserts rc 2 + the stderr detail; wire it like the
  other red batteries or as selftest-style rows in goal.emm. pri: low —
  targets the legacy `goal.py` surface, which the M5.7 cutover retires.

- M5.2 rust engine perf: ESCALATED TO SPINE (contract R30d escalation —
  corpus answer lane OOM-killed, 55.7 GB peak RSS at 7m40s). The
  roadmap carries the engine-revision unit; this register entry only
  tracks the follow-through check: after the fix, the 10 R30-parked
  suite cases flip active and pass under the runner's 30s cap, and the
  k2 corpus differential (.scratch/m5u2/diff/k2_corpus_diff.py) runs
  all three lanes green. pri: closed into spine.
