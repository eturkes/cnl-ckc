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
+ project `CLAUDE.md` clinician design law; root `README.md` = the
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
M5.9 milestone review. M5.1 CLOSED: Verus
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
M5.2 IN-FLIGHT
(sessions 1-3 = wave-1 maps → contract v1.0/v1.1 → ckc-spec K1 seed
committed. Session 4 = C1 wave ran: wt/prod-m5u2 seeded 4f6bfba7
(binding + red stub + `ckc v1 check|render` CLI, graded 96/1);
prod-m5u2-k1 checkpointed 171 verified/1 error @0120e5e9 (red =
binding postcondition alone; handoff section in its report);
test-m5u2-2 DONE — phase-2 suite 249 cases/69 legacy pins self-check
green under MAIN rerun, 62-row comparator matrix, 11/11 R22 pins, C10
rows; res-kani-2 DONE — Kani 0.67.0 pinned, interop proven, separate
nested harness crate ruled, bootstrap idempotent; contract v1.2 = R16
RULED CLOSED + R23 argv-seam scope; redstage refreshed byte-equal to
vendor; diff lanes A/H authored + smoked. Session 5 = prod-m5u2-k1-2 drove K1
batches 12-31 (term parser complete incl. variable tracking + all
four file-envelope parser stacks, 404 verified/1 error at wt
b3f97ec3; wip tip e1e2835d; retired at context ceiling after one 503
death; remaining per report Blockers: Document top-level assembly →
bind v1_check_impl → fresh verify → CLI smoke) + C2 spec fully
designed: `.scratch/m5u2/c2-design.md`
= the engine/replay/answers ruling record (fueled machine, R16
comparator, manifest/payload vocab pins, result algebra, binding
sketch) — session 6 writes the spec from it without re-reading the law
sources. Resume surface = contract v1.2 + roster session-5 close
+ extension-close blocks, which order session 6: spawn prod-m5u2-k1-3
(named bg teammate; inherits worktree + report) → 0 errors → harvest +
trust regen + lanes A/H + suite k1 target → C1 main commit → C2 spec
from design record → seed wt + dispatch prod-k2.
Wave reports = anchor lookups only; the contract embeds every ruling.)
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
