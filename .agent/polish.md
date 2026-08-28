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
  other red batteries or as selftest-style rows in goal.emm.
