# cix2 — clause-index build at high cardinality (contract)

Row: `.agent/deferred.md` `Clause-index build at high cardinality (cix reviewer-1 G1)` (new find of the dq13 queue run, funded after the session-start rows). Decisions: Verification line, Search order (eso), Language boundary. Defect: `k2_index::build_index` (contract cix D3) costs O(N·P + Σ_p n_p·C_p) — `collect_keys` searches the keys linearly per clause, `positions_all` scans the whole composition once per predicate, `classes_of` searches the classes linearly per clause, `refine` scans a predicate's whole `all` list once per class — so a predicate whose clauses carry many distinct first-argument atoms builds in quadratic time; `find_pred` + `find_class` scan entries linearly. The pre-index engine (base 89c7ee87) has no build, so cix made such compositions slow.

Measured (one predicate `guideline_property/4` with n distinct atom first arguments + one rule, `ckc v1 answer`, `/usr/bin/time -f '%e %U'`; cix 9fb13414… vs base 3564e28e…; inputs `.scratch/dq13/g1/gen.py`): n = 1,000 → 0.02 s; 3,000 → 0.19 s; 10,000 → 3.03 s wall / 2.99 s user (base 0.03 / 0.01); 30,000 → 61.38 / 58.39 s (base 0.05 / 0.04); answers cmp-equal. CDC composition today: 12,732 clauses, 25 predicates, the largest class count = `guideline_document/4` with 386.

## Tier

`kernel`: uninspected `ckc-kernel` (`k2_index.rs` + proofs). No `ckc-spec`, shell, fork, codec, corpus or fixture change; the index stays invisible to the spec's `call` (cix D2/D4 + `call_skip` unchanged). Kernel unit flow = `.claude/rules/rust.md` (consultant on this contract, seeded `wt/cix2`, diff-blind tester, reviewer on the kernel diff, `just verify` 0/0, `just gate`).

## Design

D1 Key hash (pinned, kernel spec + exec): `fold(bytes, h)` = for each byte b in order, h ← (h · 31 + b) mod 2^64 (u64 `wrapping_mul` + `wrapping_add`, the same recurrence in spec and exec); predicate key (name, arity) → `fold(name, (arity as u64).wrapping_add(1))`; class `Atom(name)` → `fold(name, 1)`, `Comp(name, arity)` → `fold(name, (arity as u64).wrapping_add(2))` (spec: the seeds (arity + 1) mod 2^64 and (arity + 2) mod 2^64). A hash only routes; every match is decided by the exact key comparison (name bytes + arity; class kind + name bytes + arity), so a collision costs time, never correctness. Equal-length keys collide when their byte polynomials agree (`Aa` and `BB`: 65·31 + 97 = 66·31 + 66): the deterministic collision cases of P3.

D2 Routing maps (`BTreeMap<u64, Vec<usize>>`, vstd-specified integer keys): `routes` (predicate hash → the entry indices with that hash) on the index and `routes` (class hash → class-list indices) on each predicate entry. Validity (part of `index_over` / `pred_over`): every entry index occurs exactly once, in its own key's bucket; every bucket index is in range and its entry hashes to that bucket; predicate keys pairwise distinct; class keys pairwise distinct within an entry; cix D3 exactness + coverage unchanged.

D3 One-pass build over B: for each clause j in order: a compound head routes to its predicate entry — found by a full bucket scan with exact comparison, else created as a fresh index appended to `preds` and to the existing bucket (`remove` the bucket, push, `insert` it back: the old chain kept); the entry is swapped out (`set_and_swap`), j joins `all`, its head class routes the same way to its class list (created when absent) or j joins `wild`, the entry is restored. Prefix invariant at each j: D2 validity + exactness + coverage over `B.take(j)`; a nonmatching entry or class selects nothing at j (ghost lemmas), so only the selected lists change. Work = O(N · (log(P+1) + log(C+1) + (1 + B_p + B_c) · L)) with bucket lengths B_p, B_c and key length L: collision-free keys build in O(N log N · L); distinct keys sharing one bucket keep a quadratic residual (stated, not tested beyond P3's collision cases).

D4 Lookups: `find_pred` + `find_class` route through the maps (hash → bucket → exact comparison); success = an exactly matching key; miss = no entry with that key anywhere (global absence, from D2 validity). `lower_bound`, the prefix scan, `entry_from` + `next_candidate` keep their cix D4 contracts and proofs; `next_candidate`'s callers are unchanged.

D5 Behavior: every answer, trace, verdict, reject and raw search row byte-identical; only build + lookup time changes.

## Predicates (acceptance = the row's check)

- P1 scaling (the row's acceptance; seed-red against the landed cix): a committed direct-kernel scaling probe (an `#[ignore]`d test in the kernel test module, run by `.agent/archive/bench/cix2-scaling <repo-root>`) builds arenas + dbs outside the timed phase — (a) one predicate with 10^3, 10^4, 10^5 distinct atom first arguments, (b) 10^2 and 10^3 predicates (distinct names, 10 clauses each) — and times `build_index` and a declared lookup batch (10^4 `next_candidate` calls per size, goals drawn from the db) separately: user + system CPU = `/proc/self/stat` utime + stime (clock ticks, test-only std file read; the kernel has no getrusage binding), 3 runs per size, medians, raw runs + rc printed. Probe exit: rc 0 iff every size's median build CPU ≤ 1 s (the row's first branch, decided numerically); the per-size growth (build CPU at ×10 size) prints as an observation, ungraded. The row's alternative branch (quadratic terms gone, D3's residual stated) = the reviewer's structural ruling on the diff, recorded beside the probe result. Negative replay: the same probe at the cix revision runs each probe under a 120 s wall timeout; a timed-out run or a completed build > 10 s stops its case (remaining runs + sizes skipped, reported unrun), rc 1. Load policy = cix P1's (no other `heavy` holder + 1-minute load < 2 at each run start, else two complete blocks, both must pass). End-to-end observation (not graded): `ckc v1 answer` CPU on the g1 CLI compositions at 10^3–10^5 for cix2 vs base vs cix, answers cmp-equal to base.
- P2 identity: `ckc check` verdicts + meters unchanged; `just check` byte-stable; every existing pin unchanged — both cix lanes (50 CLI pins over 12 test functions: 24 answer, 23 trace, 3 aggregate; the 9 lookup oracle tests; 600 raw-solve rows) and every other suite.
- P3 differential (diff-blind tester): direct-kernel `build_index` + `next_candidate` against the cix linear D2 oracle over generated databases with many predicates (distinct names; one name at two arities) + many classes (atoms, compounds of equal name at different arities, wild heads interleaved), including deterministic collisions under the pinned D1 hash — predicate names and class atoms `Aa`/`BB` (+ a three-way set, e.g. `AaAa`/`AaBB`/`BBAa`) asserted as distinct keys with equal hashes — in update pattern A, B, A, then lookups of A, B and an absent key sharing the bucket; every resume index + prefix boundary; each answer equal to the oracle.
- P4 `just verify` 0 errors 0 warnings with every theorem kept; `.agent/archive/bench/cix-aggregate <root> <base> <cix2>` → ratio ≤ 2 (the cix ceiling kept on the CDC composition); `just gate` green.

## Credit

P1 = performance red against the landed cix (its revision + binary hash + command + rc recorded), green at the unit. P2 + P3 functional cases = characterization (they hold on cix; red only against empty stubs), kernel callees credited by the D2–D4 proofs (`verus!` bodies yield no mutants); a new CLI characterization test, if any, earns its `v1_cli.rs` mutation run + survivor rulings (cix precedent).

## Out of scope

Multi-argument or deep indexing; K3 traces + recursion-check; the per-obligation witness db copy; a collision-independent worst-case bound (sorting); any spec, schema or corpus change.

## Gate identities

`just gate` (incl. both cix lanes + the new tester suite); P1 = `.agent/archive/bench/cix2-scaling`, recorded in the commit body.

## Rulings

- R1 (tester-2 corner, MAIN directive D1): each probe run carries a 120 s wall timeout; a timeout or a completed build > 10 s stops its case, the rest reported unrun (P1 text).
- R2 (tester-2 read-boundary disclosure): the six-line ghost `class_view` mapping seen beside the seed's types = API spec, not implementation ⇒ blindness kept.
- R3 (MAIN, after harvest): `heavy` (a host CPU-slot wrapper) is optional in the runner (`slot()`) + the probe (holder fallback = the probe alone) ⇒ project files stay portable.

## Flow

Contract → consultant-2 (+ one re-review round) → seed `wt/cix2` (contract + D1/D2 spec + red stubs) → tester (diff-blind: P3 + the P1 probe test) → kernel → P1 → reviewer (kernel diff) → gate → `cix2` commit closing the deferred row.
