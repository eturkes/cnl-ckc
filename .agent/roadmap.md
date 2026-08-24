# roadmap

Charter: `.agent/standing-instructions.md`. Shipped: fetch→ACE→Prolog guideline
pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py --check`);
guideline coverage runs as built-in `/goal` rounds — procedure: root
`README.md` § Operating; queue: `.agent/queue.md`. This file plans further
project development; new tasks are planned in a new session on demand.
Milestone detail (reviewed, parked, or review-pending) = `.agent/archive/`
(read on demand); this file keeps a stub + the live work per milestone.

## Standing direction

- Product = the KB artifacts (`guidelines/*/{ace,pl}` + lexicons +
  provenance/coverage/adjudication ledgers); consumers load them into
  their own engines. Compiled Prolog = a public interface once M3 lands:
  engine-portable plain clauses, schema documented + versioned. No
  SERVING layer in this repo — APIs, CDS integrations, FHIR/CDS-Hooks
  adapters = downstream projects; in-repo query machinery = the check's
  derived probes plus the committed sample queries/answers/traces
  (knowledge graph = M6) and the local loopback reviewer UI
  (review/demonstration-grade by design, never an API; zero runtime
  LLM).
- Multilingual (Japanese first): a port = a new source-language CNL
  frontend + jurisdiction-native corpus (Japanese guidelines, e.g. the
  Minds clearinghouse), never corpus translation — verification requires
  source-language comparison. DRS = the language-neutral waist
  (`ace_to_pl` consumes DRS, not English); `vendor/ape` grammar + ulex
  morphology = the English-specific components. Repo split (per-language
  repos vs agnostic core) is decided when Japanese work is planned; until
  then nothing English-specific enters `tools/`, `vendor/e--`, schemas or
  ledger formats, and all machinery treats lemma symbols as opaque atoms.
- Domain generalization: the pipeline is normative-document
  formalization, not clinical-only; domain rules (eligibility, actor
  classes, corpus protocol) stay in corpus data (compendium header,
  per-guideline files), never in code.
- UI tab modularity: the production UI = the wired surface only (local
  loopback, review/demonstration-grade). Each new tab (M5 query
  demonstrator, M6 graph explorer, later ideas) = its own milestone,
  developed without touching the working UI: tab units may land
  committed routes/pages/gates, but no nav link from existing pages,
  and existing-page goldens stay byte-identical across tab units
  (`check_ui` golden compare = the non-interference gate); the tab's
  final unit alone wires nav, regenerates goldens and runs full-UI QA.
  The copy-register lint (M4.13) + design-system invariants gate every
  later tab automatically (scan = the whole ui.py emitted-string
  surface). Tab milestones promote on user demand; refresh the plan in
  a fresh session at promotion (M2 precedent).

## M1 — American clinical guideline source compendium (easy tier) — REVIEWED

Goal: `.agent/compendium.md` (rules + org table) + `.agent/compendium.tsv`
(guideline rows) = the master list of eligible American clinical
guidelines, discovery-complete per the header's audited protocol — the
standing `/goal`'s finite terminal condition (compendium exhaustion).
Closed at ORG-UNIVERSE completeness: 475 organizations, 1,118 guideline
rows, every org row terminal; close meter `terminal remaining: orgs=0
rows=847 provisional=54` — row-status residue = the intended `/goal`
worklist, not a defect. Durable rulings = compendium header; scope
rulings, harvest shape + unit narratives = `.agent/archive/m1.md`;
technique = `.agent/archive/harvest-technique.md`; unit history =
`git log --grep "(M1[. ]" -p -- .agent/roadmap.md`.

## M2 — deferred hard-tier harvest — PARKED

Precondition: the user promotes it once `/goal` has consumed a useful
run of easy-tier rows (per-document cost measured); plan it in a fresh
session then. The register of what M1 consciously skips — every entry
an org whose `swept` cell reads `blocked(<why>)`, keeping the
compendium terminal-consistent while naming its own gaps. Deferral
classes + banked leads = `.agent/archive/m2-register.md`; technique +
gap input = `.agent/archive/harvest-technique.md`.

## M3 — projection redesign (clinician-verifiable ACE) — REVIEWED

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
rulings in `.agent/archive/m3.md` (with assurance + evidence chain,
unit gauges, review record, out-of-scope, sizing analogs); history =
`git log --grep "(M3[. ]" -p -- .agent/roadmap.md`.

## M4 — adjudication UI + query/trace compiler foundation — UNITS SHIPPED, REVIEW PENDING

Goal (rescoped: query-demonstrator tab → M5, graph-explorer tab → M6,
portable KB dist → M7, per the Standing-direction tab law): a local
reviewer UI (strict E--, zero JS) that lists every ACE document with
review status, shows each beside its exact source region(s) and compiled
Prolog, and records approve/reject + comment verdicts in a gated
append-only audit ledger (M4.1–M4.4 + M4.13), plus the
question→answer→trace compiler foundation the future tabs consume
(M4.5–M4.7). All 8 units shipped (4.1–4.7 + 4.13; 4.8–4.11 rescoped into
M5/M6/M7, 4.12 folded into 4.13); the review is the last M4 step.

Terminal state (at M4.13 close d119e25): `goal.py check` + `regen.py
--check` green — 1 guideline, 186 documents, 29 red probes, 189 live UI
pages, 4 committed queries with answers + traces; fixture corpora 75/13
UI, 42/9 adjudication, 10/2 copy (red/green); `goal: copy ok 1353
literals`; clinician QA 14 screens + 3 print sheets.

Live law = README (schema, Operating, query/trace sections) + project
`CLAUDE.md` clinician design law + the Standing-direction tab law above.
Architecture rulings (UI/E-- construction + stdlib trust surface,
committed-read serve surface, query grammar + result algebra, compiler
closure, review-subject bundle v2, mutation safety, design-law
specifics), unit records, gauges, sizing, assurance + out-of-scope =
`.agent/archive/m4.md`; history = `git log --grep "(M4[. ]" -p --
.agent/roadmap.md .agent/archive/m4.md`.

- MILESTONE-REVIEW M4 NEXT — the standing review battery over the shipped
  surface: per-unit reviewers, one cross-cutting lens, one `audit-m4`
  claim replayer whose claim surface = `.agent/archive/m4.md` +
  `.scratch/contracts/m4u*.md` + this stub's terminal-state line (M4
  detail migrated ahead of the review — the roadmap holds no other M4
  claim). Close = set the archive header REVIEWED + append the review
  record there, add the review line to this stub, commit
  `<scope> (M4 review): …`.

## M5 — query demonstrator tab — PARKED

Precondition: user promotes after M4 closes; refresh the plan in a
fresh session at promotion (M2 precedent). Consumes the shipped
M4.5–M4.7 artifacts (committed queries/answers/traces); develops +
wires per the Standing-direction tab law; governing law = CLAUDE.md
design law + the M4 UI/result-algebra rulings in
`.agent/archive/m4.md`; M4.13's
copy-register lint covers the new pages by construction. Banked unit
(old M4.9) — UI query section: picker (committed queries, ACE text),
per-solution answer tables (verbatim KB atom values; closed cardinality
operators as symbols per the design law; yes/no/indeterminate rendered
distinctly in design-law wording); primary trace chain = answer →
`% S<n>` ACE sentence → doc view → source region, formal derivation
(quoted pl clauses) = per-step collapsible disclosure; all-answer route
closure gate against the committed join; hostile fixtures; serve time =
committed artifacts only. Final unit wires the tab (nav + goldens +
full-UI QA).

## M6 — semantic graph explorer tab — PARKED

Precondition: user promotes; refresh the plan in a fresh session at
promotion (M2 precedent). Units = banked old M4.8 (graph artifact) +
old M4.10 (views) + final wire-in per the Standing-direction tab law.
Design-intent anchors = `.agent/mockups/network-revised.png` (entity
exploration, the target) + `network.png` (rejected counterexample:
artifact/review-process framing).

- Ruling (moved from M4): knowledge graph = the typed SEMANTIC graph,
  not all-pairs resolution (probe: 2.75M unifiable pairs ⇒ rejected):
  nodes = entity nouns, event/property lemmas, documents; edges typed
  from the v1 vocabulary — argument participation (event↔entity w/
  position), prepositional attachment, operator wrapping (modality/
  negation), rule antecedent→consequent flow, document membership —
  aggregated at vocabulary level with per-edge sentence-coordinate
  attribution, size-bounded on the live corpus. Query highlighting
  selects the semantic subgraph a trace's clauses touch. Purpose =
  end-user exploration of entity relationships — selecting a node (e.g.
  the opioid-prescription event) reveals its typed neighborhood (the
  approach events/properties it connects to via rule flow + argument
  participation, the patient classes those attach to) with
  sentence-level attribution into doc views; never a pipeline/
  provenance/review-process visualization (adjudication state stays in
  the reviewer section). Mockup boundary: no NL summary prose (no
  runtime LLM — panels render ACE sentences + verbatim payloads); node
  labels + classes stay vocabulary-level/structural (domain groupings
  like "approach"/"patient group" surface only where derivable from
  graph structure — corpus data owns domain rules); search + saved
  views stay out of v1. Derived-artifact machinery rides the
  compiler-closure ruling in `.agent/archive/m4.md` (smallest auditable
  closure).
- Graph artifact (old M4.8; kernel, oracle) — semantic knowledge graph:
  the typed vocabulary-aggregated graph per the ruling above,
  differential-oracled on synthetic corpora (variable renaming, NAF,
  modal contexts, self/cross-doc edges, non-unifiable controls) +
  size/performance-gated on the live corpus; committed
  `guidelines/<id>/graph.pl` (node rows carry display labels — lexicon
  surface form where provided, verbatim atom otherwise — so serve time
  never parses the lexicon), freshness by re-derivation byte-compare;
  node/edge counts printed by the meter.
- Graph views (old M4.10; kernel) — UI graph section, small views by
  construction (no global-hairball page, no zoom machinery): overview =
  typed entity index (nodes grouped by kind w/ degree + document
  counts, every entry a link); node view `?node=<id>` = the ego
  subgraph — focused node + typed 1-hop neighborhood laid out in
  columnar layers by edge class/direction (documents ← focus →
  participating events/entities → their classes), detail panel
  enumerating the node's edges grouped by edge type w/ per-edge
  sentence attributions linking doc views → source regions; query view
  `?query=<qid>` = the trace-touched subgraph alone; every rendered
  node = a link target; edge-type pagination when a neighborhood
  overflows; labels = `graph.pl` display labels; only
  sentence-attributed edges render — no inferred/transitive edges.
  Integer-grid deterministic layout (no floats/transcendentals in
  emitted coordinates; explicit decimal formatting), SVG DOM node/edge/
  link bijection + exact per-view set-equality gates vs `graph.pl`,
  bounds/non-overlap invariants, headless visual QA; serve time =
  endpoint filtering of committed `graph.pl`, no new derivation.

## M7 — portable KB distribution — PARKED

Precondition: user promotes (independent of M5/M6 — the attestation
covers the artifact families existing at build; family-extensible
release manifest, later tabs extend it); refresh the plan in a fresh
session at promotion. The include/canonicalization/rights contract may
be banked by scout any time.

- Ruling (moved from M4): distribution = manifest-driven deterministic
  archive of `guidelines/**` + generated KB docs + release-manifest
  attestation; UI/tools/vendor/.agent excluded by construction.
  Per-guideline distribution profile
  (`redistributable|reconstructable|restricted`) machine-validated from
  structured rights rows; missing/ambiguous rights fail closed. Build
  refuses while any live rejected verdict exists; unreviewed/stale ship
  labeled with their adjudication class in the release manifest. Dist
  NOTICE separates first-party licenses, compiler-derived outputs, and
  per-source rights records — recorded licenses/rights only, no legal
  conclusions. Verification from a bare extracted copy =
  tool-independent member-digest manifest (`sha256sum`-checkable); full
  regeneration = the attested monorepo revision (integrity and replay
  are different claims, both documented).
- Dist unit (old M4.11; kernel; archive/docs artifacts = prod behind
  the verifier) — portable KB dist: `tools/dist.emm` `build|--check` —
  include/exclude manifest schema, per-guideline distribution profiles
  (machine-validated structured rights rows — carries the
  source-custody polish row's machine-validation into dist
  eligibility), refuse-on-live-rejected + adjudication classes labeled
  in the release manifest, generated KB README (schema pointer,
  integrity-vs-replay commands, rights quotes) + dist NOTICE per the
  licensing ruling, release-manifest attestation (sources + ledgers +
  ACE + ulex + PL + queries + answers + traces (+ graph once M6 lands)
  + compiler-closure/SWI identity + replay commands) regenerated
  byte-identically by `goal.py check` + verified from a bare extracted
  copy by documented `sha256sum` command — the
  consumer-release-manifest polish row's acceptance verbatim (prune at
  close); deterministic tar.gz (canonical member order/path/mode/uid/
  gid/uname/mtime + gzip mtime/name/OS pinned; symlinks/special files
  reject) into gitignored `dist/`; `--check` = two builds into fresh
  destinations byte-compared + archive digests vs the committed release
  manifest; cwd/umask/TZ/delayed-run determinism fixtures; CI job.
  README dist/query/graph audit-story sections land with their
  milestones' own close docs.
