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

- M5.2 rust engine: limit-exhausting bounded searches are quadratic —
  append-only arena keeps growing across backtracking (sys-dominated
  allocation; T-C605 answer >180s where legacy <30s; contract R30d).
  Real-corpus surfaces unaffected (shallow witness proofs). Fix = arena
  mark-release at choicepoints (restore arena length on backtrack;
  terms above the mark are unreachable from the restored Cfg) or an
  equivalent cost cut, re-proved under the existing prefix discipline
  scoped to live segments. Acceptance: the 10 R30-parked suite cases
  (S4-R16-001, T-C602/603/605/606/607/611/613/625/919) flip active and
  pass under the runner's 30s cap. pri: high — M5.7 cutover replays
  obligations at production bounds and inherits this cost envelope.
