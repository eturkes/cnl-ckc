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
  `forest_valid` — each clause node's goal unifies with a renaming of its
  clause head under a substitution that instantiates the body into the
  children's goals; each naf leaf's payload generalizes the site goal and
  fails finitely under the trace bounds. This is the `proved ⇒ derivable`
  claim; legacy never checked it (map S6 #27/#40).

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
  → follow-up: a typed below-parser kernel seam (`Deferred` row); a trace
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
