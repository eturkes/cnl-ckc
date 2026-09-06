# roadmap

Charter: `.agent/standing-instructions.md`. Shipped: fetch→ACE→Prolog
guideline pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py
--check`); guideline coverage runs as built-in `/goal` rounds — procedure:
`docs/REFERENCE.md` § Operating; queue: `.agent/queue.md`; deterministic KB
export (`tools/dist.py build`, docs/REFERENCE.md § Export). Feature scope is
final (user ruling) and the project review is CLOSED: the project is
feature complete. The repo's remaining work = `/goal` rounds to
compendium exhaustion + human adjudication through the reviewer UI +
the parked hard-tier harvest below + the user-directed M5 Rust rewrite
(OPEN below); a bare `/session-roadmap` dispatches the next M5 unit
while M5 stays open, then reverts to read-only close while no parked
precondition is met. New features exist only by explicit user
direction; M5 is one such direction. Milestone records = `.agent/archive/`
— topical filenames, read on demand; this file keeps a stub per closed
or parked milestone. `M<n>` = commit-message trace keys (grep pointers
per stub); the compendium replan restarted the numbering, so key
greps also hit the earlier pipeline era — topic sorts the hits.

## Standing direction

- Product = the KB artifacts (`guidelines/*/{ace,pl}` + lexicons +
  provenance/coverage/adjudication ledgers + committed sample queries
  with answers + traces); consumers load them into their own engines.
  Compiled Prolog = the public interface: engine-portable plain clauses,
  schema v1 documented + versioned. No SERVING layer in this repo — APIs,
  CDS integrations, FHIR/CDS-Hooks adapters = downstream projects; the
  sole in-repo consumer = the local loopback reviewer UI
  (review/demonstration-grade by design, never an API, zero runtime LLM).
- Production bars (user ruling; discharged at the project review): UI
  production-ready = runs locally (web hosting out of scope); KB
  production-ready = exportable in a format standard for this kind of
  work — the charter's deterministic distribution build.
- Neutrality (durable law): nothing source-language-specific enters
  first-party tooling (`tools/`, `rust/`), schemas or ledger formats — machinery treats
  lemma symbols as opaque atoms; `vendor/ape` grammar + ulex morphology =
  the English-specific components. Domain rules (eligibility, actor
  classes, corpus protocol) stay in corpus data (compendium header,
  per-guideline files), never in code.

## Guideline source compendium — REVIEWED

Goal: `.agent/compendium.md` (rules + org table) + `.agent/compendium.tsv`
(guideline rows) = the master list of eligible American clinical
guidelines, discovery-complete per the header's audited protocol — the
standing `/goal`'s finite terminal condition (compendium exhaustion).
Closed at ORG-UNIVERSE completeness: 475 organizations, 1,118 guideline
rows, every org row terminal; close meter `terminal remaining: orgs=0
rows=847 provisional=54` — row-status residue = the intended `/goal`
worklist, not a defect. Durable rulings = compendium header; scope
rulings, harvest shape + unit narratives = `.agent/archive/harvest.md`;
technique = `.agent/archive/harvest-technique.md`; unit history =
`git log --grep "(M1[. ]" -p -- .agent/roadmap.md`.

## Projection redesign — REVIEWED

Goal: fixture-free, source-anchored, knowledge-only ACE; frozen
engine-portable public Prolog ABI (schema v1); proof obligations
derived + replayed per document + aggregate; complete `cdc-2022-opioid`
migration under unchanged coverage/census/docid custody. Terminal at
review close 18ab5a1: 79 ACE = 79 compiled documents on the frozen
nine-indicator schema v1, 257 obligations discharged per document +
aggregate under bounded search, lexicon 487 live entries, custody
ledgers total + set-equal to the ACE inventory, queue gate lifted —
corpus growth since = `/goal` rounds under the frozen v1 knowledge-only
contract. Live law = docs/REFERENCE.md schema section + the governing authoring
rulings in `.agent/archive/projection.md` (with assurance + evidence chain,
unit gauges, review record, out-of-scope, sizing analogs); history =
`git log --grep "(M3[. ]" -p -- .agent/roadmap.md`.

## Adjudication UI + query/trace compiler + KB export — REVIEWED

Goal (rescoped at the feature-complete ruling; the final project review
landed the lean KB export): a local reviewer UI (strict E--; sole JS =
the pinned highlight-emphasis script)
that lists every ACE document with review status, shows each beside its
exact source region and compiled Prolog, and records approve/reject +
comment verdicts in a gated append-only audit ledger (M4.1–M4.4 +
M4.13); the question→answer→trace compiler (M4.5–M4.7); the
deterministic rights-gated KB export (review-session dist track).

Terminal state (review close): `goal.py check` + `regen.py --check`
green — 1 guideline, 186 documents, 29 red probes, 189 live UI pages, 4
committed queries with answers + traces, `goal: dist ok 1 guidelines
408 members`; `tests/dist/red.sh` 62/62; committed
`release-manifest.tsv` (staleness triggers + regen recipe = docs/REFERENCE.md
Operating § Close + memory "KB export" bullet). Review battery =
per-unit + cross-cutting reviewers, 24-row claim replay, diff-blind
dist red suite; findings, residuals + rulings = the review record in
`.agent/archive/ui.md`.

Live law = docs/REFERENCE.md (schema, Operating, Export, query/trace sections)
+ the clinician design law `.claude/rules/clinician-design.md`; root `README.md` = the
simplified overview. User-directed post-review addition: doc-page
source↔ACE authored-alignment highlighting (law = memory "Doc-page
source↔ACE highlight" bullet + REFERENCE § Reviewer interface). Architecture rulings, unit
records, gauges, sizing, assurance + out-of-scope =
`.agent/archive/ui.md`; history = `git log --grep "(M4[. ]" -p --
.agent/roadmap.md .agent/archive/` + `git log --grep "(M4 review)"`.

## Rust rewrite + E-- retirement — OPEN (M5)

User-directed. Goal: every first-party non-Prolog artifact = Rust,
certified in the verified-kernel pattern — small human-read formal spec
(trusted) + uninspected AI-written impl/proofs + pinned checker
(`cargo verus verify`) + escape-hatch trust-audit gate, zero trust in
the code author; E-- + generated Python + tests/strict permanently
retired (git history = sole record); hand-authored Prolog confined to
the APE fork's ACE emission closure, with KB consumption (load checks,
obligation replay, recursion scan, answers, traces) moving to the
verified Rust engine. Split law = `.agent/standing-instructions.md`
(amended). Plan of record = `.agent/archive/rust-rewrite-plan.md`:
seam classification, Verus election + fallback triggers, kernel
partition, unit contracts + acceptance, fixture-consequence classes,
certification story — read at every unit start. Open-milestone
invariants: main green at every commit (legacy `regen.py --check` +
`goal.py check` stay CI-authoritative until the M5.7 cutover); every
unit proves parity by dual-run differential before any legacy
deletion; corpus + committed query artifacts byte-stable except
plan-ruled re-pins (dist archive re-baselines once at M5.6). Units,
strictly ordered: M5.1 toolchain+verified spike+trust gate; M5.2 v1
kernel (reader/writer, bounded engine, replay, answers, traces); M5.3
check validators; M5.4 pipeline commands; M5.5 UI; M5.6 dist; M5.7
cutover+retirement+reference scrub; M5.8 fork shrink to emission-only;
M5.9 milestone review = MILESTONE-REVIEW over ledger
`.agent/review-m5.md`; M5.1 rows (rev 70 + rev2 59 =
`.agent/contracts/m5u1-rev*.md`) enter it adjudicated;
projection ≤ 7 open units × ~70 rows (M5.1 analog; tier per unit
contract), session count calibrates on the first review session.
M5.1 CLOSED: Verus
release/0.2026.08.30.b432e82 + rustc 1.97.1 pinned (`rust/verus.lock`
+ gitignored `.toolchain/`); first verified kernel = align render-side
validator (trusted spec `ckc-spec/src/align.rs` + binding
`ckc-kernel/src/contract.rs`, kernel 95 verified/0 errors under
`--no-cheating`, align_impl.rs uninspected by design); differential 0
divergences (pins 11/11, corpus 337/337 at HEAD), adversarial suite
94/94, trust battery 16/16; probes rt 2.81× / naf 1.77× proof:impl —
NO fallback trigger → M5.2 entry clear; reviews: rev 70-row fixed set
adjudicated + rev2 59/59 mutants (56/56 G1 kills, 3/3 weak-binding
demos caught at G2) + determinism D1-D3 pass; trust-audit v0 committed
(`ckc trust-audit`) + CI `rust` job (pinned-asset digest install).
M5.2 IN-FLIGHT. Contract of record = `.agent/contracts/m5u2.md`
(v1.3: R9 first-divergence offset law; R16 comparator CLOSED; R23
argv-seam scope). K1 SHIPPED on main: spec `ckc-spec/src/{term,
v1text,digest}.rs` + binding `v1_check` (accept ⟺ `accepts`, Reject
offset ≤ len) + uninspected `ckc-kernel/src/v1_{impl,term_impl}.rs`
+ CLI `ckc v1 check|render`; P1 439 verified/0 errors, P2 green,
lanes A/H 349/349, suite 63 K1 cases (26 exact R9 stderr pins via
`.scratch/m5u2/suite/pin_r9.py`; 22 red until the impl threads the
offset). R9 impl ruling: one `&mut usize` viable-boundary parameter
through the EXISTING parsers, no separate scanner (a scanner build on
wt/k1-6-scanner passes the 26 pins yet misreports declaration-block
mutations as file end → rejected; `.scratch/m5u2/suite/probe_decl.py`
= the added acceptance sweep); R9 SHIPPED on main f76108c3: one `&mut usize` viable-boundary
parameter through the existing parsers (parse_doc keeps a
per-iteration bundle offset — a mutable reference inside its loop
invariant killed Z3); P1 448/0, P2 spec=2486, lanes A/H 349/349,
suite 63/249 (26 R9 pins green), probe_decl 670/0, smoke, P6. C2 spec SEEDED + verified:
`engine.rs` (fueled DFS machine, R13 occurs-check, R16 comparator +
sorts, witness/replay probes) + `replay.rs` (manifest total parse,
staged aggregate pipeline, recursion scan, `ESrc/ERow/EOut` mirrors)
+ `answers.rs` (query custody, bounded-once yes-no / wh classify,
`print_answers`); spec 79/0, P1 439/0, trust spec=2442 (P7 aim
exceeded → review adjudicates; rev lens for C1/C2 + K1-K3 adds
proof minimality: a redundant proof layer, duplicate representation
or lemma provable from a neighbour = a finding). Next: contract v1.3 C2 amendments
(shell/kernel protocol, comparator placement, payload_term/R2b
unification, per-dispatch cost model, unique-path reload law,
empty-heads order skew, manifest error/context wrapper,
comparator-matrix path, query-unreadable class map) = contract R24
(a-f). K2 IN FLIGHT: seed wt/prod-m5u2-k2 @b218c69a = bindings
`v1_manifest`/`v1_aggregate_check`/`v1_recursion_check`/`v1_answer`
over `ESrc` + 4 red stubs `ckc-kernel/src/k2_impl.rs` + CLI `ckc v1
aggregate-check|recursion-check|answer`; prod-m5u2-k2-1..3 landed U1
manifest + U2 (arena `k2_term.rs`, sorts `k2_sort.rs`, rejects via
the one printer `k2_reject.rs`; wt @e93c39fd 567/3 = the mode stubs).
Contract R25 (arena ruling): backward child links (`child_roots`,
child < parent), K1's `parse_term` builds the arena through a
threaded `&mut ETermArena`, `the_v1`/`the_payload` (choose)
discharged by the guided `expected` — no K2 re-parser, no
injectivity lemma. K2 KERNEL LANDED ON MAIN a5103d4e (session 9,
squash of wt/prod-m5u2-k2; tags archive/m5u2-k2-wave @8e3cdddf +
archive/m5u2-k1b @31e1c731 keep the batch history; worktrees +
wt/ branches removed): 13 k2_* modules = U1 manifest, U2 term
arena/printer/comparator/sorts, U3 engine (unify w/ termination
measure, machine call/redo + exact backtracking, fueled run/solve,
head_proved/heads_proved/unifiable_apart), U4 modes
(aggregate-check staged pipeline, recursion census, answer
custody/solve/print) + K1 arena bridge w/ exec roots + metadata
(R27 eliminators) + R9 absorbed incl. the k1-12-found
overflow-offset repair. Gates at a5103d4e: P1 983/0 (main),
trust spec=2528 ok, release rc0, suite self-check 170 legacy pins
+ target passed=139 / 0 failures, K1 lanes A/H 349/349. Rulings
R26-R30 = contract (NAF-prune spec-conformant; R27 bridge surface;
R28 suite dispositions; R29 pin fix; R30 activation triage). K2
suite lane COMPLETE: 107 staged k2 cases dispositioned (93 active
green, 10 R30d perf-parked, 4 R28c/d/e pending-ruling); canonical
suite embeds repo-root-relative paths → relocations re-derive path
pins (R30a). OPEN GAP = R30d exec-cost defects (spec + proofs
unaffected): load pipeline quadratic (recursion-check 5 docs
0.07s/54MB → 60 docs 5.6s/4.1GB; corpus OOM) + search arena growth
(corpus answer OOM 55.7GB/7m40s); NEXT UNIT = engine/load perf
revision (fresh worktree off main; design sketch in R30d:
truncate-to-`fresh` on backtrack + persistent solution region +
load-pipeline accumulation fix; acceptance = 10 parked cases
active+green under the 30s cap + `k2_corpus_diff.py` all 3 lanes
green). Corpus differential = `.scratch/m5u2/diff/k2_corpus_diff.py`
(lead-authored; lanes answer/recursion/aggregate; derives proof
payloads via the legacy stage). Evidence: suite 249 cases/170
legacy pins, comparator 62/62, R22 11/11; Kani 0.67.0 pinned
(wt/res-kani-2 kept until C3). Gauge actuals: K1 parser stack =
five prod windows at 149-245K; U3+bridge+U4 = ten prod windows at
121-183K high-water; suite lane = three windows 66-162K; session-9
lead close ~50% 500K/1M (1M window session). Wave reports = anchor
lookups; the contract embeds every ruling; teammate lesson: SEED
report files before dispatch (three suite teammates refused to
create absent .md reports).
`/goal` rounds + the parked harvest continue on
legacy tooling until cutover; M5 acceptance derives corpus/fixture
counts from HEAD at run time. History key: `git log --grep "(M5[. ]"`
(earlier-era M5 hits sort out by topic).

## Hard-tier harvest — PARKED

Precondition: compendium exhaustion — the standing `/goal`'s terminal
condition (easy-tier residue consumed; terminal clause = the
compendium header, adopted by docs/REFERENCE.md § Operating). Promote in a fresh
session then: plan the harvest from the deferral register
(`.agent/archive/hard-tier-register.md` — deferral classes, sizing
measurements, banked leads) + technique =
`.agent/archive/harvest-technique.md`; authoritative gap list =
compendium `blocked(<why>)` org cells + blocked/provisional guideline
rows (the register aligns to them, not the reverse).
