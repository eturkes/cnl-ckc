# roadmap

Charter: `.agent/standing-instructions.md`. Shipped: fetch→ACE→Prolog guideline
pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py --check`);
guideline coverage runs as built-in `/goal` rounds — procedure: root
`README.md` § Operating; queue: `.agent/queue.md`. This file plans further
project development; new tasks are planned in a new session on demand.

## Standing direction

- Product = the KB artifacts (`guidelines/*/{ace,pl}` + lexicons +
  provenance/coverage/adjudication ledgers); consumers load them into
  their own engines. Compiled Prolog = a public interface once M3 lands:
  engine-portable plain clauses, schema documented + versioned. No
  SERVING layer in this repo — APIs, CDS integrations, FHIR/CDS-Hooks
  adapters = downstream projects; in-repo query machinery = the check's
  derived probes plus M4's committed sample queries/answers/graph and
  the local loopback reviewer UI (review/demonstration-grade by design,
  never an API; zero runtime LLM).
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

## M1 — American clinical guideline source compendium (easy tier) — REVIEWED

Goal: `.agent/compendium.md` (rules + org table) + `.agent/compendium.tsv`
(guideline rows) = the master list of eligible American clinical guidelines,
discovery-complete per the header's audited protocol. Fetch stays deferred
until a row is promoted into `.agent/queue.md`. This gives the standing
`/goal` a finite terminal condition = compendium exhaustion.

Terminal state: 475 organizations (127 yes / 309 unverified / 39 no), 1,118
guideline rows; every org row terminal (`CPGs=no` | dated manifest |
`blocked(<why>)`); `goal.py check` meter `475 organizations 1118 rows;
terminal remaining: orgs=0 rows=847 provisional=54`. Row-status residue = the
intended `/goal` worklist, not a defect.

Scope rulings (compendium header owns the durable form):

- Easy tier = index enumerates in <=2 anonymous fetches (`static-list`,
  `search-api`, `pdf-index`, or a scripted-UA 403 the `r.jina.ai` reader
  clears) and artifacts classify without an authenticated session. Bounded
  fan-out qualifies (<=30 anonymous static subindexes, enumerable from the
  index in one fetch). Everything else defers as `blocked(<why>)` -> M2.
- M1 closes at ORG-UNIVERSE completeness, not harvest exhaustion: the
  guideline table freezes at M1.2's 1,118 rows while every org row ends
  terminal. An unswept `yes` org records `blocked(easy-tier harvest
  deferred; M2)` — a named gap rather than a hidden one.
- Per-org determination defers to M2 whole: measured 3.2-3.4 tool calls +
  1.2-1.5 WebSearches/org over 346 orgs = 415-519 searches against the
  200/session ceiling, buying `/goal` nothing it can consume now; M1.3b's
  crosswalk is its costed worklist. Bulk prefilter measured: GC u NGC u
  AAFP-PG u PubMed corporate-author flags 9 of 11 true-yes orgs, every hit
  positive evidence only (an omission is silence).
- Harvest shape: `prod` teammates sweep assigned indexes, MAIN merges with
  `.scratch/merge_rows.py` (the only row inserter; reports = source of truth,
  so a repair edits a report and re-merges), each swept org recording
  `<date> <method>; <n> index entries -> <e> eligible + <x> excluded` |
  `blocked(<why>)`. Fan-out method + budgets = `.agent/rounds.md`.

Units — narratives, per-unit rulings, gauges + SHAs:
`git log --grep "(M1[. ]" -p -- .agent/roadmap.md`; durable rulings =
compendium header; technique + tooling contracts = `.agent/memory.md`.

- M1.1 DONE — schema + protocol + 447-org seed roster + 88 seed rows.
- M1.2a DONE — federal easy tier, 13 orgs -> 395 rows; USPSTF + NIOSH
  blocked on exhausted anonymous enumeration.
- M1.2c DONE — M1.2a review remediation + ACIP re-swept at artifact
  granularity (27 -> 68) -> 435 rows; issuance-vs-index custody rulings
  (`indexed-by=`, `off-index(`, MMWR label vs publication year).
- M1.2b DONE — society/other easy tier, 10 orgs -> 1,118 rows / 474 orgs;
  access re-probe promoted 32 rows; ACOG deferred under contract R1.
- M1.3a DONE — 5 gate defects (each red-first) + 2 org-name collision merges
  -> 473 orgs; identity, independence + canonical sorts centralized in
  `compendium_io.py`.
- M1.3a2 DONE (data) — 32 HICPAC re-attributions + 58 R6 close calls + 2
  validated org appends -> 475 orgs.
- M1.3b DONE (data) — org-universe terminalization (427 edits, idempotent),
  669-candidate cross-check over 16 orgs, `/goal` integration.
- M1.5 DONE (kernel) — `/goal`-consumed predicates ported to E--
  `goal.py check`; deep predicates stay `.scratch/` with regeneration paths
  in memory. 48-fixture dual gate green, 35/35 mutation kills.
- M1 REVIEW DONE (data) — 13/13 claims replayed, gates 4/4 green; F-01 AASM
  advisory exclusion reason corrected, F-02 provenance narration pruned.

## M2 — deferred hard-tier harvest — PARKED

PARKED = skipped by `/session-roadmap` MODE dispatch until its named
precondition holds: the user promotes it after `/goal` has consumed a
useful run of easy-tier rows (per-document cost measured). Plan it in a
fresh session then.

The register of what M1 consciously skips. Every entry here is an org
whose `swept` cell reads `blocked(<why>)`, so the compendium stays
terminal-consistent while naming its own gaps. Plan this milestone only
after M1 is REVIEWED and `/goal` has consumed a useful run of easy-tier
rows — the real per-document cost of `/goal` is what should size it.

Deferred classes, from the 46-org measurement
(`.scratch/agents/scout-m1u2.md`):

- Easy-tier orgs determined but never harvested — the terminal-scope
  ruling's direct output, and M2's cheapest class: every org ending
  `CPGs=yes` + `blocked(easy-tier harvest deferred; M2)`, each carrying
  the index URL and tier M1 recorded — ~94 known at the re-cut (81
  pending-since-M1.1 + the scouts' easy-tier `yes`), growing as the
  determination pass runs. The only class whose per-org sweep cost M1
  has already measured.
- Per-org determination pass — the re-cut's wholesale deferral: every
  row `blocked(determination deferred; M2 — <crosswalk signal>)`, the
  crosswalk its costed, prioritized worklist. Protocol-v2 friction
  rulings stay banked in the scout reports: D2's two-fetch cap covers
  own-site index discovery only while D5 buys one bibliographic fetch;
  transport classification runs direct→reader before any artifact
  fetch; a `CPGs=no` needs affirmative evidence (function + exhaustive
  own-site output taxonomy + a co-issuance check), never search
  silence; `WPSI-program` counts nothing toward the ≥2. Sized by the
  measured 3.2–3.4 calls + 1.2–1.5 searches/org against the 200/session
  WebSearch ceiling.
- Cross-check gap adjudication — M1.3b's gap list over the 23
  swept/blocked orgs: eligibility, version + access ruled per gap, rows
  entering through the off-index mechanism. Delivered: 669 candidate
  gaps/16 orgs (`.scratch/m1u3b/gapcheck/gaps.tsv`; record in
  compendium § Protocol layer 3). Determination-pass leads banked here:
  historical-issuer PMIDs ACRO 14585480 (2003) + SCPC 23892939 (2013);
  HVS + SCPC co-issuer attribution audit (eligible shipped AUC rows
  name both while both sit `unverified` + unsettled-blocked); APTA
  academy renames (APTA Neurology → Academy of Neurologic Physical
  Therapy; scout-1's alias-map shortcut: parent-directory crosswalk →
  active academy name/domain → own CPG page).
- ACCM artifact-attribution audit — Q3b/Q3c unresolved
  (`.scratch/agents/res-m1u3a-1.md`); relationship settled (SCCM special
  body; `CONTROLS` recorded); its 3 guideline rows' issuer cells stand
  unverified against artifact text.
- Corpora whose breadth outruns the easy tier — enumeration is one
  anonymous call but per-artifact version and access adjudication is not,
  so the org defers whole rather than being swept against a narrow facet
  (M1.2b ruling R1): ACOG (~2,500 clinical records across 11 guidance
  types, of which the compendium's recorded index sees 61).
- Browser-gated indexes — the authenticated browser is the only route:
  AAP, NKF, HRS, ASCO, ACCP (Cloudflare), CFF index (Akamai), OPA
  (Cloudflare), NIAID (JS CAPTCHA), CPIC (JS-only shell).
- Login corpora — one gate covers a whole corpus, so each is a single
  holder pass: NCCN (free account, ≥91 rows), AORN (eGuidelines
  subscription, 36 rows).
- Heavy tail — enumeration is cheap but per-artifact resolution is not,
  so each wants a solo teammate split by topic or pagination: ACR 279,
  VA PBM ≈582, ASCO ≥139, IDSA ≈105 (109 fetches), AAN 65 (7 AJAX
  pages), ACR-scale others surfacing during M1.3.
- Version-ambiguous collections — the index mixes current with
  superseded, so a version ruling must precede row emission: CDC Stacks
  (786 records over 40 pages).
- Missing or stale enumeration surfaces — a replacement index must be
  found first: AAFP (methodology overview, no artifact index),
  ADA-Dental (404), AUA + ATS + BTF (indexes expose only part of the
  corpus), Endocrine Society (count needs the listing API).
- Access classification deferred wholesale — M1 records `access` from
  the artifact only where an anonymous fetch settles it; the remaining
  rows keep `unverified` + `provisional(…)` and never promote, which is
  the intended M2 worklist rather than a defect.
- Storage format — guideline rows already live in `.agent/compendium.tsv`
  (rules + org table in the `.md`). At ≈4,400 rows revisit whether the
  org table follows.
- Row granularity — the M1.1 header rules appropriate-use criteria and
  committee opinions each into their own row, which is what puts ACR at
  279 and drives the ≈4,400 total. A coarser rule (one row per guideline
  series) would cut the compendium to ≈500 rows; revisiting it is a
  header-level decision, not a harvest decision.

## M3 — projection redesign (clinician-verifiable ACE) — REVIEWED

Goal: replace fixture-bearing legacy projections with source-anchored,
knowledge-only ACE; freeze an engine-portable public Prolog ABI; derive and
replay proof obligations per document + aggregate; migrate the complete
`cdc-2022-opioid` corpus without changing coverage/census/docid custody.

Terminal state (at review close 18ab5a1; live law = README schema section +
the rulings below): 79 ACE = 79 compiled documents on frozen nine-indicator
schema v1, 257 obligations discharged per document + aggregate under bounded
search, lexicon 487 live entries, custody ledgers total + set-equal to the
ACE inventory, queue gate lifted — corpus growth since = `/goal` rounds
under the frozen v1 knowledge-only contract.

Governing authoring rulings:

- Preserve source conditions, objects, numbers + modality; record every
  approximation/drop in projection notes; introduce no unanchored structure.
- Hoist universal restrictors out of modal complements into antecedents; split
  compound "..., and if A then B" statements into separate ACE sentences.
- Render "only ... should ..." restrictions as conditional prohibitions, one
  rule per unmet conjunct; classical negation for unmet facts, NAF only for
  evidentiary absence ("unless there are indications ...").
- Map `might`->`may`, ability/option `can`->`can`, responsibility->`must`;
  modality stays reified data with no deontic inference. Causal rationale never
  becomes a temporal PP. Source `for` takes `prep(for,for)` + a determiner.
- Noun modification = relative clause or compound noun; `of` rejects
  (`condition_shape(relation(A,of,B))`).
- Source upper bounds (`at most n`, `exactly n`, `less than n`) reject at the
  root of an asserted sentence (`root_condition([...])`, ACE scoping them over
  a grouped condition list) and compile inside a rule or operator box; a
  root-position bound is hoisted under its modality or recorded as an
  approximation in the notes. Lower bounds (`at least n`, `more than n`) carry
  no group and compile anywhere.
- Source "or" folds only where the notes name the approximation; disjunctive
  consequents stay unsupported. Defeasibility/backend changes need a named
  consumer question and recompile the unchanged ACE corpus.

Assurance + evidence:

- Durable acceptance = `python3 -P tools/goal.py check` (terminal corpus,
  aggregate replay, 19 committed reds each pinning class + rc + byte-exact
  stderr) + `python3 -P tools/regen.py --check`.
- Closing M3 chain = durable gate -> `.scratch/m3u1/gate_m3u1.py` ->
  `.scratch/m3u2/gate_m3u2.py` -> `.scratch/m3u3/gate_m3u3.py` ->
  `.scratch/m3u7/diff_m3u7.py` -> fresh guideline compile ->
  `git diff --quiet -- guidelines/`.
- Question boundary = `.scratch/m3u7/suite/runner.py` 96 cells; contracts of
  record = `.scratch/contracts/m3u1.md`...`m3u7.md`; regeneration + port
  ownership = `.agent/memory.md` M3 evidence bullet + `.agent/polish.md`.
- Unit gauges: M3.1 main=84% 202K, mate=80% 192K - M3.2 main=93% 224K,
  mate=100% 239K - M3.3 main=102% 245K pre-compaction / 59% 143K close,
  mate=100% 240K - M3.4 main=94% 225K pre-compaction / 49% 117K close,
  mate=58% 139K - M3.5 main=92% 220K, mate=39% 95K - M3.6 main=87% 208K,
  mate=28% 67K - M3.7 main=93% 224K, mate=95% 227K.

Review DONE (9 lenses; 26/28 audited claims replayed clean): fixed HIGH
self-certifying aggregate payloads (composition-derived coverage checks;
battery `.scratch/m3rev/payload_battery.py`) + MED byte-exact `.expect`
pins on every red probe, reserved `schema=v1` ulex basename + widened
legacy witness (both retired with the pre-v1 path deletion); SLD-divergence
risk documented in README + left-recursion scan in
`goal.py check`; residues + deferred hardening = `.agent/polish.md` rows;
decisive chain rerun green at close — narratives, rulings + rerun numbers =
`git log --grep "(M3 review" -p -- .agent/roadmap.md` (close = 18ab5a1);
gauges main=84% 200K close (wave spans 2 windows), mate=86% 206K peak.

Out of scope remains: serving/query API; probabilities, thresholds, arithmetic
or unit conversion absent from source; defeasibility/s(CASP) before a named
consumer question; rec6-12 authoring and pending-region/coverage closure,
which return to `/goal` rounds after review.

Sizing (analogs: M1.5 main=166K narrow kernel; M1.3a main=196K; M1.2b
main=203K oversized harvest; M1.3b main=179K): kernel units project
~150-175K MAIN with a hard checkpoint at contract+manifest before
implementation begins; oracle teammates may span planned successors
(each dispatch <=~170K). Production batches budget by source words
(~1,000/batch), not doc count; MAIN ~150-170K each. Off-spine items live
in `.agent/polish.md`, never as units. Out of scope: defeasibility/
s(CASP) (named consumer question first; backend swap = one backend +
recompile), serving layer, recs 6-12 authoring (post-M3 `/goal` work),
queue items 1-2 (pending-region rulings + coverage closure = `/goal`
rounds), new coverage rulings, probabilities/thresholds/conversions
absent from source.

## M4 — adjudication + demonstration UI, portable KB distribution — IN-PROGRESS

Goal: a local reviewer UI (strict E--, zero JS) that lists every ACE
document with review status, shows each beside its exact source
region(s), projection notes and compiled Prolog, and records
approve/reject + comment verdicts in a gated audit ledger; a
preloaded-query demonstrator whose answers derive from the compiled
Prolog alone and trace answer → Prolog → ACE → source; a knowledge-graph
view with query-path highlighting; a deterministic KB distribution build
shipping the knowledge artifacts without UI/tools/vendor.

Plan reviewed adversarially at planning time: `planrev-m4` report =
`.scratch/agents/planrev-m4.md` (8 lenses; the unit split, tiers, graph
redesign, question grammar, staleness model + mutation-safety contracts
below arbitrate its findings). Feasibility evidence = `.scratch/m4plan/`:
`ui_spike.emm`/`.py` (strict-E-- WSGI serve + form POST green; NOTE its
`write_text` is NOT atomic — the real ledger write protocol is M4.4's),
`question_probes.sh` + `.out` (9 replayable APE question-DRS families),
planrev's live-corpus graph probe (all-pairs body-goal→head unifiability
= 2,750,504 clause-instance pairs / 63,992 sentence-edges over 575
sentence sites ⇒ an untyped all-pairs graph is unusable).

Architecture rulings:

- UI = one local WSGI server compiled from strict E-- (`tools/ui.emm` →
  `tools/ui.py`; wsgiref, 127.0.0.1, single-threaded), every page
  server-rendered HTML/SVG from committed state; zero JavaScript, zero
  committed HTML/CSS files (markup + a small cohesive design system
  emitted by E-- code; typed render helpers per context — text, quoted
  attribute, URL/query component, id — no raw corpus/user string reaches
  markup). E-- bindings: `Use <dotted>.` imports submodules; tuples via
  `[[tuple]](<a, b>)`; `python3 -P` omits the script dir from sys.path ⇒
  one `.emm` per entry point, all under `tools/` (regen roots + dist
  exclusion hold). Stdlib trust surface, enumerated: wsgiref (HTTP),
  html + urllib.parse (escaping/decoding), pathlib/os (fs + atomic
  replace), hashlib (digests), subprocess (shared validator), tarfile +
  gzip (dist) — hand-written code owns project semantics only, never
  HTTP parsing, URL grammar, archive encoding, or fs primitives.
- Serve time reads committed artifacts + the adjudication ledger only —
  no swipl, no LLM, no socket beyond loopback. All derivation (query
  compile, answers/traces, graph) runs in the pipeline and is committed
  + freshness-gated exactly like `pl/`. Every M4 gate backing a durable
  claim reruns from committed state (project rule); scratch batteries =
  supplemental only.
- Queries = ACE interrogatives, CNL all the way down. Closed v1 query
  grammar (probe corpus `.scratch/m4plan/question_probes.out`):
  existential conjunctive wh (`query(Ref, who|which|what)` referents) +
  existential/ground yes-no + modal operator boxes (render as
  operator-box goals per the rule-body precedent); universal yes-no
  (parses to an implication DRS), disjunction (`v/2`), classical
  negation and NAF inside questions all REJECT with named details — an
  entailment/countermodel semantics is out of scope. Result algebra:
  wh → `solutions([...])`; yes-no → `yes(trace)` | `no(finite_failure)`
  | `indeterminate(limit)`; a bound hit is never rendered as "no".
  Answers render KB atoms + provenance coordinates only; cardinalities
  display as verbatim schema payloads, never computed quantities; no
  natural-language glue, no runtime LLM.
- Derived-artifact machinery (query compile, answer/trace, graph) lands
  in the named first-party Prolog closure of the APE fork: inside
  `ace_to_pl.pl` as modes, or — if the M4.5 spike measures lower total
  trusted code — as one additional first-party module beside it, with
  PROVENANCE `First-party files:`, README audit story and inventory
  gates updated in the same unit (the vendor-closure polish row becomes
  mandatory in-unit on that route). Smallest auditable closure decides,
  not file count. Committed answers/traces/graph are compiler-DERIVED
  products carrying no hand-authored knowledge; hand-authored `.pl`
  stays barred from guideline trees.
- Adjudication review subject = what the reviewer actually saw: each
  ledger row pins `review_sha256`, a canonical bundle digest over the
  doc's ACE bytes + its coverage region row + region payload bytes + its
  projection-notes row + its semantic-clause digest (compiled clauses
  canonicalized minus the volatile document-record header, so a pure
  vocabulary append staleness nothing while a reinterpreting ulex/Clex/
  compiler change staleness exactly the semantically-changed docs — the
  append-stability polish row folds in here). Component digests live in
  a check-derived `audit/review-manifest.tsv` so the UI can explain WHY
  a verdict went stale. Verdict rows are current-state; history = git
  (README Operating gains a commit-after-each-review-batch checkpoint);
  `reviewer` = self-asserted local identifier (canonical grammar, no
  control/tab/newline), date = server-generated UTC ISO — the UI
  displays the self-asserted limitation. Rejected documents = `/goal`
  repair worklist with an Operating priority rule (live rejected docs
  outrank new extraction/fetch work); meters, never check failures.
- Mutation safety (the one write path): POST carries the rendered
  `review_sha256` back as a hidden field and the server re-compares
  against committed state immediately before persist (409 + no mutation
  on drift); ledger writes are compare-and-swap on the prior ledger
  digest through exclusive same-dir temp + flush + fsync + `os.replace`;
  CSRF = per-process unpredictable token required on every POST +
  loopback Host check + same-origin Origin-when-present; validation =
  ONE shared implementation (a `goal.py` ledger-validate subcommand the
  UI invokes via subprocess before persist) so gate and UI grammars
  cannot drift.
- Knowledge graph = the typed SEMANTIC graph, not all-pairs resolution
  (probe: 2.75M unifiable pairs ⇒ rejected): nodes = entity nouns,
  event/property lemmas, documents; edges typed from the v1 vocabulary —
  argument participation (event↔entity w/ position), prepositional
  attachment, operator wrapping (modality/negation), rule
  antecedent→consequent flow, document membership — aggregated at
  vocabulary level with per-edge sentence-coordinate attribution,
  size-bounded on the live corpus. Query highlighting selects the
  semantic subgraph a trace's clauses touch.
- Distribution = manifest-driven deterministic archive of
  `guidelines/**` + generated KB docs + release-manifest attestation;
  UI/tools/vendor/.agent excluded by construction. Per-guideline
  distribution profile (`redistributable|reconstructable|restricted`)
  machine-validated from structured rights rows; missing/ambiguous
  rights fail closed. Build refuses while any live rejected verdict
  exists; unreviewed/stale ship labeled with their adjudication class in
  the release manifest. Dist NOTICE separates first-party licenses,
  compiler-derived outputs, and per-source rights records — recorded
  licenses/rights only, no legal conclusions. Verification from a bare
  extracted copy = tool-independent member-digest manifest
  (`sha256sum`-checkable); full regeneration = the attested monorepo
  revision (integrity and replay are different claims, both documented).

Units. Deps: 4.1→4.4; 4.2→4.3→4.4 (checkpoint 1: adjudication UI
consumable); 4.5→4.6→4.7; 4.8 after 4.7 (shared compiler closure edits
serialize 4.5→4.6→4.7→4.8, and graph aggregation reuses trace-unit
canonicalization); 4.3+4.6+4.7→4.9; 4.3+4.7+4.8→4.10 (checkpoint 2:
demonstrator consumable); 4.2..4.8→4.11 (checkpoint 3: portable KB;
its include/canonicalization/rights contract may be banked by scout any
time after 4.2); 4.12 last. Compiler-mode contracts + oracles (4.5/4.6/
4.7/4.8) and the dist contract are parallel-prep-safe while the UI trunk
lands; only the shared-file implementations serialize.

- M4.1 OPEN (kernel) — strict E-- exception verbs: minimal
  `Try:`/`Catch <name>:` block statements in the vendor/e-- fork
  (charter: extend E-- liberally) — spec.md append, lexer/parser/emitter
  + strict red tests, regen identity; scope = the smallest construct
  letting `tools/ui.emm` express contract-specific 400/409/no-mutation
  outcomes around raising stdlib calls instead of generic wsgiref 500s
  (planrev L6; wsgiref handle_error stays the last-resort boundary).
  Apache fork notices per gate.
- M4.2 OPEN (kernel) — adjudication ledger + staleness gate: pinned
  `audit/adjudication.tsv` (literal header + field grammar + transition
  table in the unit contract; `docid TAB review_sha256 TAB verdict TAB
  reviewer TAB date TAB comment`; verdict `approved|rejected`; ≤1
  row/docid, canonical sort), the `review_sha256` bundle + semantic-
  clause-digest definition and derived `audit/review-manifest.tsv`,
  `goal.py check` gate (grammar; unknown docid rejects; bundle match ⇒
  live | mismatch ⇒ stale; absent file = all-unreviewed green) + meter
  `goal: adjudication <id> approved=<a> rejected=<r> stale=<s>
  unreviewed=<u>`; committed fixture battery; acceptance carries the
  append-stability polish row (pure ulex append staleness nothing; a
  reinterpreting append staleness exactly the changed docs; prune row at
  close). README Operating: rejected-documents-outrank-new-work rule +
  review-batch commit checkpoint.
- M4.3 OPEN (kernel) — reviewer UI read core: `tools/ui.emm` serve —
  repo-level guideline picker (generic `guidelines/<id>` discovery,
  namespaced routes, absent-artifact families handled) → guideline index
  w/ status chips + meter → doc view: ACE text, the doc's coverage
  region (current corpus cardinality = exactly one region/doc; payload
  bytes = authority, section/page = display metadata), projection-notes
  kept/dropped, collapsible compiled Prolog, staleness explanation from
  the review manifest. Route/HTTP table + CLI contract (port, unknown-id
  behavior, methods, status codes) = unit contract deliverables. Total
  view-model join gate over EVERY guideline/doc (each docid exactly
  once; every ace() region joined; every projection row joined; no
  orphan routes) + golden pages for representative shapes + hostile
  corpus-string fixtures + design system w/ keyboard/contrast/focus
  invariants + chromiumfish headless visual QA. `ui.py render <outdir>`
  static export, byte-stable, covering every ace() region — carries the
  faithfulness-review polish row's acceptance verbatim (prune at close).
- M4.4 OPEN (kernel) — adjudication mutation path: the verdict form +
  POST pipeline implementing the mutation-safety ruling wholesale
  (hidden subject digest → 409 on drift; shared subprocess validator;
  CAS + tmp/fsync/`os.replace`; CSRF token + Host/Origin checks;
  reviewer-grammar + UTC date stamping); committed write-path battery:
  GET→artifact-change→POST refusal, two-writers-from-same-bytes, foreign
  origin/missing token no-mutation, invalid rows rejected byte-exactly,
  crash-mid-write leaves prior ledger intact.
- M4.5 OPEN (kernel, oracle) — question projection: the closed v1 query
  grammar over `question(drs(...))` boxes (families + rejects per the
  architecture ruling; probe corpus banked) compiling to v1-vocabulary
  goal conjunctions + an answer-variable manifest (each `query/2`
  referent's noun/class); TCB-closure spike (modes-in-file vs one
  first-party module) decides here and updates PROVENANCE/README/
  inventory + pulls the vendor-closure polish row if module route;
  ordinary document compile keeps family-specific question rejection
  (`question_not_supported` refined per family — carries the
  question-diagnostics polish row; prune only with both modes green +
  cross-mode isolation probes); `tests/red/` question reds committed.
- M4.6 OPEN (kernel, oracle) — query artifacts + answers: qid grammar,
  `guidelines/<id>/queries/<qid>.ace` → committed `queries/pl/<qid>.pl`
  w/ binding header (question ACE bytes, ulex digest, compiler
  identity), `answer` mode over the aggregate manifest — bounded solve
  (per-solution + whole-query constants pinned in-contract; M3 bounds
  the baseline), canonical solution identity/dedup/order keys, result
  algebra incl. `indeterminate(limit)` ≠ `no`; committed
  `queries/answers/<qid>.pl`; check re-derives byte-identically,
  closes the `.ace`/`.pl`/answers inventory bijection, and requires
  committed demo queries to be `yes`/nonempty-`solutions`; ≥3 authored
  cdc sample queries; new swipl invocations ship wall-clock-bounded from
  birth (existing-call retrofit stays the bounded-subprocess polish
  row); red probes: positive, genuine-negative, limit-exceeded.
- M4.7 OPEN (kernel, oracle) — proof traces: trace term schema (clause
  applications carrying `sentence(DocId, S)`; NAF/limit leaves = explicit
  non-proof status), meta-interpreter + independent teammate oracle;
  every positive node names a byte-identical committed clause + unique
  (docid, S); serializer adversarial probes (escapes/non-ASCII/operator
  ambience — the renderer-probes polish row's classes for the new
  writers); committed `queries/traces/<qid>.pl`, freshness + double-run
  determinism.
- M4.8 OPEN (kernel, oracle) — semantic knowledge graph: the typed
  vocabulary-aggregated graph per the architecture ruling, differential-
  oracled on synthetic corpora (variable renaming, NAF, modal contexts,
  self/cross-doc edges, non-unifiable controls) + size/performance-gated
  on the live corpus; committed `guidelines/<id>/graph.pl`, freshness by
  re-derivation byte-compare; node/edge counts printed by the meter.
- M4.9 OPEN (kernel) — UI query section: picker (committed queries, ACE
  text), per-solution answer tables (nouns/classes/verbatim cardinality
  payloads; yes/no/indeterminate rendered distinctly), trace chain
  answer → quoted pl clause → `% S<n>` ACE sentence → doc view → source
  region; all-answer route closure gate against the committed join;
  hostile fixtures; serve time = committed artifacts only.
- M4.10 OPEN (kernel) — UI graph section: integer-grid deterministic
  layout (no floats/transcendentals in emitted coordinates; explicit
  decimal formatting), SVG DOM node/edge/link bijection gates vs
  `graph.pl`, bounds/non-overlap invariants, exact highlighted-set
  equality per query (`?query=<qid>` dims the complement), zoom =
  scale-parameter links, pan = scroll, multi-scale headless visual QA.
- M4.11 OPEN (kernel; archive/docs artifacts = prod behind the verifier)
  — portable KB dist: `tools/dist.emm` `build|--check` — include/exclude
  manifest schema, per-guideline distribution profiles (machine-validated
  structured rights rows — carries the source-custody polish row's
  machine-validation into dist eligibility), refuse-on-live-rejected +
  adjudication classes labeled in the release manifest, generated KB
  README (schema pointer, integrity-vs-replay commands, rights quotes) +
  dist NOTICE per the licensing ruling, release-manifest attestation
  (sources + ledgers + ACE + ulex + PL + queries + answers + traces +
  graph + compiler-closure/SWI identity + replay commands) regenerated
  byte-identically by `goal.py check` + verified from a bare extracted
  copy by documented `sha256sum` command — the consumer-release-manifest
  polish row's acceptance verbatim (prune at close); deterministic
  tar.gz (canonical member order/path/mode/uid/gid/uname/mtime + gzip
  mtime/name/OS pinned; symlinks/special files reject) into gitignored
  `dist/`; `--check` = two builds into fresh destinations byte-compared
  + archive digests vs the committed release manifest; cwd/umask/TZ/
  delayed-run determinism fixtures; CI job.
- M4.12 OPEN (docs) — consistency close: README rewrite (audit story: E--
  UI + exception verbs, question support boundary, derived-artifact
  families + TCB closure naming, Running: `ui.py`/`dist.py`, adjudication
  + rejection workflow, query/answer/graph semantics, distribution +
  integrity-vs-replay, loopback security posture, licensing prose);
  guideline README + queue/rounds touch-ups; polish-register
  reconciliation (pulled rows pruned with acceptance evidence, deferred
  UI hardening rows appended with acceptance checks).

Sizing (planrev L1: five of seven M3 MAIN kernels hit 200-245K
pre-compaction; the ≤175K blanket was refuted): every unit sizes
against its nearest analog at planning-of-work time and targets ≤150K
MAIN with coordination reserve — M4.1 vs the E-- parser's existing block
statements (S); M4.2 vs the coverage gate + digest machinery (M); M4.3 +
M4.4 = the split that replaced the one-window-refuted UI unit (M4.3
largest E-- authoring, hard checkpoint at view-model contract; M4.4
narrow but adversarial); M4.5 vs M3.1/M3.2 compiler analogs (202-224K ⇒
narrowest possible grammar, oracle prep banked in parallel); M4.6-M4.8
each single-concern compiler modes; M4.9/M4.10 UI increments on the
M4.3 chassis; M4.11 vs M1.5-style port + new determinism fixtures;
M4.12 docs. Kernel units take the full portfolio battery
(test/orc/rev/rev2 where `oracle`); prod-tagged artifacts ride their
named validator.

Assurance: durable gates = `python3 -P tools/goal.py check` + `python3
-P tools/regen.py --check`, extended in-place per unit (adjudication +
review-manifest, query/answer/trace/graph freshness + inventory
bijections, dist attestation regeneration); every contract-defining
fixture set lands committed and check-reachable, scratch campaigns
supplemental; the UI mutation path is the sole new write surface,
guarded by the shared validator + safety battery.

Out of scope: runtime LLM anywhere in UI or answers; free-text/ad-hoc
query entry (queries enter as committed ACE through ordinary rounds);
universal/entailment question semantics; ACE editing from the UI
(repairs stay `/goal` rounds); remote or multi-user serving, auth
beyond the loopback CSRF posture, TLS; JavaScript or any browser-side
computation; WASM Prolog; append-only verdict event history (git +
batch commits = the record); defeasibility; serving APIs (standing
direction unchanged for downstream consumers).
