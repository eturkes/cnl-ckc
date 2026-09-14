# M5.2b — K3 trace + trace-check + Kani (contract)

Unit = K3 of the M5.2 kernel partition (`archive/rust-rewrite-plan.md`): the
trace derivation (`ckc v1 trace`) and the committed-trace acceptance relation
(`ckc v1 trace-check`), plus the Kani secondary gate (P3). M5.2 rulings R1–R30
bind; R28c/R28d/R28e pending rows are ruled here (R32–R34).

## Tier

`kernel` — spec `ckc-spec/src/trace.rs` (MAIN-authored, human-read) +
uninspected `ckc-kernel/src/k3_impl.rs` (+ modules) + shell seam
`ckc v1 trace|trace-check` (hashing at the shell, R3) + differential lanes D/E
+ tests/queries replay + red suite k3 rows + Kani harnesses.

## Spec (the law, `trace.rs`)

- Pipeline order = legacy `trace_mode`: manifest → query custody (answer law
  verbatim) → answers custody (`answers_file(<why>)`, legacy why order over
  the canonical class: `noncanonical` fold (R2b) → `record_shape` (a Traces
  file) → `term_count(N)` (Doc/Query) → `qid_mismatch` →
  `query_sha256_mismatch` → `result_shape` → `result_mode_mismatch` →
  `solutions_list` → per row `solution_shape(I)` | `solution_values(I)` |
  `solution_arity(I,Expected,Actual)`) → composition load + records →
  per-row directed solve → emit. Legacy `record_nonground`/`record_version`/
  `record_query_sha256` fold into `noncanonical` (canonical grammar makes
  them unrepresentable) = R2b class rows.
- Machine = the engine's search (`engine.rs` law) + proof log: every clause
  resolution outside a negation appends `(path, Clause(m))`; a negation site
  pushes a `Naf` choicepoint, runs the inner goal at a fresh `trace_depth()`
  with the row prune flag reset, and (a) inner success → `NafCut` drops the
  site's choicepoints + fails the site, restoring the entry prune flag
  (a cut inside a successful inner proof never reaches the row — legacy
  probe 2026-09-08: `row: plain-fail` under `once(inner)` with a deep
  first branch); (b) inner finite failure → leaf `Naf(inner)` recorded, the
  goal frozen as it stood at site entry; (c) a depth prune inside the site
  followed by inner failure → the row is `unproved(limit)` (`TStep::Limit`
  = the legacy `ace_to_pl_trace_naf_limit` throw). Row bounds
  `trace_depth()=1000` (MI frames: depth spends per goal dispatch as in the
  engine — a DIFFERENT measure from SWI's MI frame count; R15 non-parity
  posture) + `trace_inf()=100000` per row; run bound `trace_run_inf()=1e6`
  over all rows incl. materialization (one charge per clause node).
- Proof forest `build` from the log by path (root i = top-level conjunct i,
  child j of a clause node = body item j); materialization = `clause(
  sentence(D,S), clause_sha256(Hex), Children)` with `identity` = exactly
  one distinct `'$guideline_id'(Role∈{context,product,witness}, D atom,
  S>0, _, _)` pair over the PRISTINE clause (`clause_identity(none|
  multiple(N))` proof rc1 otherwise); `Hex` = shell sha256 of
  `clause_line(db[m])` (R3, `trace_lines`); `naf(Payload)` with payload
  variables numbered `'$VAR'(N)` continuing one run-wide counter (legacy
  whole-term numbervars).
- Mirror law (`trace_result`): `yes` → `yes(P)`; `sol(V)` rows →
  `sol(V, P)` in file order, each on a fresh binding of the query goal to
  the row's values; `no(finite_failure)`/`indeterminate(limit)` verbatim;
  run-bound trip → `indeterminate(limit)`.
- Trace-check (`trace_check_output`): trace custody (`trace_file(<why>)`:
  noncanonical → record_shape → qid/query_sha256/answers_sha256 mismatch) →
  derivation equality (`trace_check(stale)` rc1) → join: every clause node's
  `(D, S, Hex)` resolves to exactly one committed clause line of that
  sentence (`trace_check(join(sentence(D,S), N))`), roots are clause nodes,
  naf leaves only among children, node grammar per the legacy walker
  (`node_shape`), demonstration (`non_demo`: yes(proved) or nonempty
  solutions all proved) — evaluated in the spec's order: per solution row in
  file order, proof presence first then that row's joins (`rows_join`); an
  earlier row's `non_demo` preempts a later row's join failure → meter
  `ckc: trace-check ok <qid> nodes=<k>`.
- Soundness theorem (`contract::k3_sound`): every derived forest is
  `forest_valid` — ONE answer substitution `th` shared by every node (R54):
  each clause node's goal equals a renaming of its clause head under `th`,
  the renamed body items = the children's goals; each naf leaf's frozen
  payload generalizes the site goal under `th` and fails finitely under the
  trace bounds. This is the `proved ⇒ derivable` claim; legacy never
  checked it (map S6 #27/#40).

## Predicates (acceptance)

- P1 `just verify` green (contract bindings `v1_trace_lines`, `v1_trace`,
  `v1_trace_check`, `k3_sound` discharged; no new escape sites).
- P2 `just trust` green after regen (spec manifest gains `trace.rs`).
- P3 Kani: `rust/ckc-kani-harness/` (separate workspace, res-kani-2 Q6)
  with the align + 2 reader harnesses green under `just kani` (bootstrap
  from committed `rust/kani.lock` + `rust/kani/bootstrap.sh`); secondary
  gate, not in `just rust` (cost), weekly CI. PARTIAL (ruled): an engine
  harness over `v1_answer` exhausts CBMC at the minimum domain (5 bounded
  runs rc124 at 1200 s / ~12 GiB: 3 shapes CaDiCaL, Z3, dereference cache)
  because the whole-file K1 loader replays symbolically before the engine
  → follow-up: a typed below-parser kernel seam (`.agent/deferred.md` row); a trace
  harness follows K3.
- P4 lane D/E: `ckc v1 trace` byte-identical to legacy AND to the 4
  committed `guidelines/*/queries/traces/*.pl`; `k2_corpus_diff.py` gains
  lane `trace`.
- P5 tests/queries replay: every green case's `traces-golden` byte-identical
  under `ckc v1 trace`; every red trace case classifies identically
  (`trace-reject` pairs: rc2 + exact stderr; `trace-*` red goldens
  byte-identical; `trace-check` rejects `red/trace-digest-join` with
  `join(sentence(doc,1), 0)`, `red/stale-trace` with `stale`, the non-demo
  reds with `non_demo`); census per R15 (v1-canonical vs non-v1 content).
- P6 red suite: the 54 k3 rows (`.scratch/m5u2/suite/cases`, `tag: k3`)
  activated (staged → active; pins derived from the branch binary +
  legacy), 0 failures; T-C622/T-C626 ruled (R33) → active or dropped.
- P7 `just rust` green; legacy chain untouched (`just legacy` green).

## Gate identities

`just rust`; `just kani`; `RUST_BIN=… python3 -P .scratch/m5u2/suite/runner.py
target`; `python3 -P .scratch/m5u2/diff/k2_corpus_diff.py --lanes trace`;
tests/queries replay = `python3 -P .scratch/m5u2b/queries_replay.py --rust-bin …`
(lead-authored).

## Rulings

- R32 (R28c close) trace cost model: depth = engine goal-dispatch measure
  with `trace_depth()=1000` per row and per negation site; inferences =
  engine transitions, `trace_inf()=100000` per row, materialization charged
  to the run only (`trace_run_inf()=1e6`), one per clause node. SWI MI
  frame parity = explicit non-goal (R15). T-C622/T-C626 = rust-side probes
  at hand-countable sizes pinned to THIS model; near-boundary legacy parity
  never asserted.
- R33 (R28d) T-C624 stays a policy non-case (no wall clock in the mode).
- R34 (R28e) T-C627 constant-binding mutation → M5 review mutation campaign.
- R35 answers custody folds: `record_nonground`, `record_version(V)`,
  `record_query_sha256` = unreachable through the canonical grammar → the
  `noncanonical` class (R2b enumerated; legacy pins stay oracle evidence).
- R37 K3 scope boundary: per-qid inventory + freshness verdicts (`goal.py check`
  queries section: stale pl/answers/trace, missing/orphan artifacts, demo order)
  = validator tier → M5.3 K4 (`ckc check`); the K3 seam sees compiled files
  only. Suite rows T-C821/T-C822 carry the K3-observable custody portion.
- R38 `k3_sound` hypothesis `bodies_wf(db)`: canonical bodies never carry a
  bare conjunction (`wf_body_item`); a `Pos(','(..))` body would split into two
  goals at one proof path (prod-m5u2b counterexample). The composition stage
  yields `wf_v1` documents, so every shell-facing binding meets it.
- R39 tests/queries fixture header: 20/35 fixture documents open with the
  legacy `% synthetic attributed product; …` line (not `doc_line1`); the K3
  replay + suite copies canonicalize LINE 1 ONLY (goldens are header-
  independent: digests hash clause lines); originals stay byte-identical
  (FC0); the fixture re-pin lands with the queries battery port (M5.3/M5.7,
  FC2 row). `red/trace-digest-join`'s hostile-spacing doc = composition
  `noncanonical(<path>)` class (R24a).
- R36 trace-check `join` law = the public REFERENCE law (exactly one
  committed clause line of the named sentence block), NOT legacy
  `trace_block_table`'s widening to any dot-terminated line (map S6 #19).
- R54 `forest_valid` substitution law: per-node independent substitutions
  (a) never certified the conjunction's joint solution and (b) falsified
  every NAF leaf whose site goal an earlier sibling had instantiated
  (prod-m5u2b-2 counterexample: db `p(a).`, goal `(p(A), \\+ q(A))`, forest
  `[Clause(0,[]), Naf(q(a))]` — ground `q(a)` never instantiates to the
  original root arg `q(A)`). Law = one existential `th` over the forest
  (`forest_valid = exists th. kids_valid(db, th, roots, forest)`); children
  goals stay raw renamed body items, every comparison applies `th` once.
- R55 (R39 extension) the K3 replay copies canonicalize LINE 1 of every
  copied fixture file the K1 reader gates — documents, query projections,
  answers AND traces (goldens are header-independent) — so a legacy header
  such as `% q-stale-trace  traced …` never masks the law under test;
  `red/stale-trace` therefore classifies `stale` (P5) on its canonicalized
  copy. A copy K1 still rejects after the line-1 rewrite = the R15 non-v1
  census class: expectation `noncanonical(<path>)` (R24a), listed per case
  with the K1 reject reason; originals stay byte-identical (FC0).
- R56 `red/stale-trace` under R15/R55: the tracked fixture encodes staleness
  in its header line alone (two-space `% q-stale-trace  traced …`), which
  K1 rejects at (1,17) → the original classifies `trace_file(noncanonical)`
  (R15 non-v1 census); its line-1-canonical copy is byte-identical to the
  fresh derivation → rc0 control; the replay adds a scratch-only derived
  probe (canonical copy with the proof replaced by `unproved(finite_failure)`,
  custody digests preserved) that must classify `stale` = P5's stale law.
  Tracked fixture re-pin = the queries battery port (R39, FC2 row).
- R67 `k3_sound` hypothesis `answers::goal_walk(goal) is None` beside
  `bodies_wf(db)` (the EXISTING query-preflight custody law: every conjunct
  is a v1 semantic-predicate compound — no variable, foreign or negation
  root; prod-k3sound-1 / prod-m5u2b-2 counterexample: db `p(\\+ q(a)).`,
  goal `(p(A), A)` executes the bound variable as a NAF site while
  `forest_valid` sees the raw `Var` root). The trace pipeline enforces it
  on every committed query (answers.rs Query preflight), so every
  shell-facing row meets it; a separate `roots_wf` would duplicate it
  (R-01). Binding: `requires bodies_wf(db), goal_walk(goal) is None,
  derived_forest(db, goal) is Some`.
- R60 T-C622/T-C626 cost probes (R32 model) = CLI-observable BOUNDARY rows
  (no cost-visible seam: the kernel exposes no fuel counters and a
  counters-only binding would be trusted surface for testing alone): active
  k3 rows, `legacy: null` (near-boundary legacy parity = non-goal), hand-
  derived expectations recorded in each manifest (`.scratch/m5u2/diff/
  k3state/r32-proposed-pins.tsv`): T-C622 = N top-level fact roots with row
  search ≤ 100000 while row-charged materialization would exceed it
  (`proved` under the model) + the 1000/1001 positive chain (`proved` vs
  `unproved(limit)`); T-C626 = the NAF fresh-depth reset probe + the
  1000/1001 chain under a NAF site; target pins derived at harvest must
  equal the hand-derived expectation.
- R75 trigger-only: `node_valid`'s Clause existential triggers on the
  nonrecursive `shift(db[m].head, k)` — the recursive `resolves` term is
  fuel-indexed (`rec%resolves(…, fuel)`) and a witness proved at another fuel
  never matches it (prod-k3sound-1 SMT diagnosis + before/after probe);
  no semantic change.

