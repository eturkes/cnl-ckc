# roadmap

Charter: `.agent/standing-instructions.md`. Shipped: fetch→ACE→Prolog
guideline pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py
--check`); guideline coverage runs as built-in `/goal` rounds — procedure:
`docs/REFERENCE.md` § Operating; queue: `.agent/queue.md`; deterministic KB
export (`tools/dist.py build`, docs/REFERENCE.md § Export). Feature scope is
final (user ruling) and the project review is CLOSED: the project is
feature complete. The repo's remaining work = `/goal` rounds to
compendium exhaustion + human adjudication through the reviewer UI +
the parked hard-tier harvest below; a bare `/session-roadmap` closes
read-only while no parked precondition is met, and new features exist
only by explicit user direction. Milestone records = `.agent/archive/`
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
  `tools/`, `vendor/e--`, schemas or ledger formats — machinery treats
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
landed the lean KB export): a local reviewer UI (strict E--, zero JS)
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
source↔ACE lexical word highlighting (law = memory "Doc-page
source↔ACE highlight" bullet + REFERENCE § Reviewer interface). Architecture rulings, unit
records, gauges, sizing, assurance + out-of-scope =
`.agent/archive/ui.md`; history = `git log --grep "(M4[. ]" -p --
.agent/roadmap.md .agent/archive/` + `git log --grep "(M4 review)"`.

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
