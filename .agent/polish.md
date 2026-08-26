# polish register

Final register (user ruling): operational hardening for the `/goal` loop +
reviewer UI only. The project review (roadmap NEXT) consumes every row in
its session and closes the register; `/session-polish` may land rows
earlier on spare capacity. Rows outside that bar (suite/battery ports,
assurance-depth increments, speculative compiler extensions) were dropped
at the feature-complete ruling — git history holds them if user direction
ever reopens one. Row: `- [ ] <artifact>: <upgrade> | why: <evidence> |
accept: <check> | size: S|M|L | pri: 1-3`. Done rows are pruned in the
completing commit.

- [ ] goal.py bounded subprocess wrapper: wall-clock bound every swipl
  invocation (stage build, compile, load, aggregate, recursion, red
  probes); timeout = pinned fail-closed detail + `.goal.tmp.*` cleanup |
  why: proof search is depth/inference-bounded but the six existing
  subprocess calls carry no process bound — a hung swipl wedges a `/goal`
  round (M4.6+ derive/answer invocations ship wall-clock-bounded from
  birth; this row = the six existing-call retrofits) | accept: sleeper +
  descendant-held-pipe fixtures yield the pinned timeout detail with
  scratch removed; clean run green | size: M | pri: 2
- [ ] goal.py `ledger-validate` foreign-cwd defect: module-level
  `compiler_source` require resolves `vendor/ape/...` against cwd before
  subcommand dispatch — move the require into the compile/check paths or
  anchor to the script dir | why: the shipped subcommand dies by
  traceback from any cwd but repo root; ui.py works only because it pins
  cwd | accept: `cd /tmp && python3 -P <abs>/tools/goal.py
  ledger-validate <valid ledger> <valid manifest> x` exits without
  traceback; ui.py keeps its cwd= pin regardless | size: S | pri: 2
- [ ] shared docid/gid grammar max-length: hoist the 250-byte bound
  (`.html` headroom on NAME_MAX=255) from ui.py into the goal grammar so
  both tools cite one owner | why: goal accepts longer ids that ui then
  rejects — a `/goal` round could mint a document the reviewer UI cannot
  serve | accept: goal.py check red on a 251-byte docid, ui red fixtures
  unchanged, bound stated once | size: S | pri: 2
- [ ] GET Host-check DNS-rebinding hardening: only the POST path
  validates Host; GET review/doc pages remain readable by rebound
  origins | why: the UI is the production review surface — a hostile
  page rebinding a name to 127.0.0.1 can read corpus + verdict state |
  accept: GET refuses foreign Host with the POST path's law; fixtures
  updated | size: S | pri: 2
- [ ] goal.py census-map gate: validate `audit/census-map.tsv` like
  projection-notes (header bytes, row shape, docid/region join to
  coverage) | why: committed audit artifact, ungated — drift invisible
  to check while `/goal` rounds keep writing it | accept: tampered
  census-map row fails check with pinned detail; clean corpus green |
  size: S | pri: 3
