# roadmap

Charter: `.agent/standing-instructions.md`. Shipped: fetch→ACE→Prolog
guideline pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py
--check`); guideline coverage runs as built-in `/goal` rounds — procedure:
root `README.md` § Operating; queue: `.agent/queue.md`. Scope is final
(user ruling): the project review below = the last roadmap/polish work.
After it closes, the repo's remaining work = `/goal` rounds to compendium
exhaustion + human adjudication through the reviewer UI; a bare
`/session-roadmap` then closes read-only, and new development exists only
by explicit user direction. Milestone records = `.agent/archive/` —
topical filenames, read on demand; this file keeps a stub per closed
milestone + the one live unit. `M<n>` = commit-message keys (grep
pointers per stub); the key sequence keeps gaps where milestones were
removed at the feature-complete ruling.

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

## Guideline source compendium (M1) — REVIEWED

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

## Projection redesign (M3) — REVIEWED

Goal: fixture-free, source-anchored, knowledge-only ACE; frozen
engine-portable public Prolog ABI (schema v1); proof obligations
derived + replayed per document + aggregate; complete `cdc-2022-opioid`
migration under unchanged coverage/census/docid custody. Terminal at
review close 18ab5a1: 79 ACE = 79 compiled documents on the frozen
nine-indicator schema v1, 257 obligations discharged per document +
aggregate under bounded search, lexicon 487 live entries, custody
ledgers total + set-equal to the ACE inventory, queue gate lifted —
corpus growth since = `/goal` rounds under the frozen v1 knowledge-only
contract. Live law = README schema section + the governing authoring
rulings in `.agent/archive/projection.md` (with assurance + evidence chain,
unit gauges, review record, out-of-scope, sizing analogs); history =
`git log --grep "(M3[. ]" -p -- .agent/roadmap.md`.

## Adjudication UI + query/trace compiler (M4) — UNITS SHIPPED, REVIEW PENDING

Goal (rescoped: the query-demonstrator tab, graph-explorer tab +
portable-KB-dist units left M4 and are out of project scope —
feature-complete ruling; the project review below lands the lean KB
export instead): a local reviewer UI (strict E--, zero JS) that lists
every ACE document with review status, shows each beside its exact
source region(s) and compiled Prolog, and records approve/reject +
comment verdicts in a gated append-only audit ledger (M4.1–M4.4 +
M4.13), plus the question→answer→trace compiler (M4.5–M4.7). All 8
units shipped (4.1–4.7 + 4.13); the review runs inside the project
review below.

Terminal state (at M4.13 close d119e25): `goal.py check` + `regen.py
--check` green — 1 guideline, 186 documents, 29 red probes, 189 live UI
pages, 4 committed queries with answers + traces; fixture corpora 75/13
UI, 42/9 adjudication, 10/2 copy (red/green); `goal: copy ok 1353
literals`; clinician QA 14 screens + 3 print sheets.

Live law = README (schema, Operating, query/trace sections) + project
`CLAUDE.md` clinician design law. Architecture rulings (UI/E--
construction + stdlib trust surface, committed-read serve surface, query
grammar + result algebra, compiler closure, review-subject bundle v2,
mutation safety, design-law specifics), unit records, gauges, sizing,
assurance + out-of-scope = `.agent/archive/ui.md`; history = `git log
--grep "(M4[. ]" -p -- .agent/roadmap.md .agent/archive/`.

## Project review + close (M4) — NEXT (the final roadmap/polish work)

One MILESTONE-REVIEW session (context policy: runs past compaction
across coherent checkpoints) closes all four tracks below; at its close
commits the project is feature complete.

1. M4 review — the standing review battery over the shipped surface:
   per-unit reviewers (4.1–4.7, 4.13), one cross-cutting lens, one
   `audit-m4` claim replayer whose claim surface = `.agent/archive/ui.md`
   + `.scratch/contracts/m4u*.md` + the M4 terminal-state line above
   (the roadmap holds no other M4 claim).
2. Polish discharge — `.agent/polish.md` (pre-pruned to operational
   hardening by user ruling) lands whole in this session, each row
   against its written acceptance; the register closes empty.
3. KB export (KB production bar) — one committed E-- tool
   (`tools/dist.emm` → generated twin, regen-rooted) + `goal.py check`
   gate producing a portable export of the KB from committed state.
   Content = `guidelines/**` (sources, ACE, ulex, pl, ledgers,
   queries/answers/traces) + generated consumer README (schema pointer,
   verification + replay commands, rights quotes) + NOTICE (first-party
   licenses, compiler-derived outputs, per-source rights records —
   recorded facts, no legal conclusions) + content-addressed release
   manifest binding source rows + ledgers + ACE + ulex + PL +
   queries/answers/traces + compiler-closure/SWI identity + replay
   commands. Rights: per-guideline distribution profile
   (`redistributable|reconstructable|restricted`) machine-validated
   from structured rights rows; missing/ambiguous rights fail closed.
   Gates (absorbed polish acceptances): `goal.py check` regenerates the
   release manifest byte-identically; a documented `sha256sum` command
   verifies integrity from a bare extracted copy; a tampered source
   byte fails with a pinned detail, clean corpus green; a live rejected
   verdict refuses the build; unreviewed/stale documents ship labeled
   with their adjudication class; two builds into fresh destinations
   byte-compare equal (canonical member order/path/mode/uid/gid/mtime +
   gzip fields pinned; symlinks/special files reject). Format =
   review-time ruling after a bounded survey (WebSearch ≤ 20) of how
   comparable formalized-KB/corpus releases ship; floor = deterministic
   tar.gz + per-member sha256 manifest into gitignored `dist/`;
   UI/tools/vendor/`.agent` excluded by construction.
4. UI production bar + close — README records: the reviewer UI is
   production at local run (`ui.py serve`, loopback; web hosting out of
   scope) + the export/verification story. Close: `.agent/archive/ui.md`
   header REVIEWED + review record appended (project-close record
   included); `.agent/polish.md` = closed empty register; memory pruned
   to the operating set; this file rewritten to the terminal operating
   stub (header law above; no NEXT remains); commits `<scope> (M4
   review): …` + `<scope> (project close): …`. Decisive gates before
   each close commit: `goal.py check` + `regen.py --check` + fresh
   `goal.py compile cdc-2022-opioid` + `git diff --quiet -- guidelines/`.
