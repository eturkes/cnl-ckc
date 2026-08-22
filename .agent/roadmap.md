# roadmap

Charter: `.agent/standing-instructions.md`. Shipped: fetch→ACE→Prolog guideline
pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py --check`);
guideline coverage runs as built-in `/goal` rounds — procedure: root
`README.md` § Operating; queue: `.agent/queue.md`. This file plans further
project development; new tasks are planned in a new session on demand.
Closed/parked milestone detail = `.agent/archive/` (read on demand).

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

## M4 — adjudication UI + query/trace compiler foundation — IN-PROGRESS

Goal (rescoped: query-demonstrator tab → M5, graph-explorer tab → M6,
portable KB dist → M7, per the Standing-direction tab law): a local
reviewer UI (strict E--, zero JS) that lists every ACE document with
review status, shows each beside its exact source region(s), projection
notes and compiled Prolog, and records approve/reject + comment
verdicts in a gated audit ledger (shipped M4.1–M4.4), plus the
question→answer→trace compiler foundation the future tabs consume
(shipped M4.5–M4.7); M4.13 closes the milestone by finishing the review
surface per the clinician design law. Design-intent anchor =
`.agent/mockups/review.png` (ballpark-approved); post-close user-led
style iteration (functionality-stable) on demand. Network mockups
anchor M6.

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
  replace), hashlib (digests), subprocess (shared validator), tempfile +
  shutil (snapshot staging + scratch trees), datetime (UTC verdict
  stamps), re (census/anchor parsing),
  tarfile + gzip (dist) — hand-written code owns project semantics only,
  never HTTP parsing, URL grammar, archive encoding, or fs primitives.
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
  wh → `solutions([...])`; yes-no → `yes` | `no(finite_failure)`
  | `indeterminate(limit)` (M4.7 adds the trace-bearing yes); a bound
  hit is never rendered as "no". Answer artifacts carry raw KB atom
  values (v1.3); provenance coordinates ride M4.7 traces and UI
  dereference lands in M5 (query tab); cardinalities display as verbatim schema
  payloads, never computed quantities; no natural-language glue, no
  runtime LLM.
- Derived-artifact machinery (query compile, answer/trace, M6's graph) lands
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
- Clinician-facing design law (all UI surfaces; durable law = project
  `CLAUDE.md` UI/UX rule, mechanical enforcement = M4.13's copy-register
  lint + the design-system invariants): visible page copy carries zero
  plumbing — digests, qids, lemma symbols, raw schema functors ride
  URLs/hidden fields/ledgers only; staleness, verdict + limit panels
  read as plain language (stale = the document or its source changed
  after this verdict; `indeterminate(limit)` = not determinable within
  the system's search bounds; empty `solutions` = none stated in the
  loaded guidelines); closed cardinality operators render as symbols
  (`eq`→`=`, `geq`→`≥`) beside verbatim payload atoms; every page
  carries the scope line — reports what the loaded guideline documents
  state, not clinical advice; capabilities stated plainly (prepared
  questions only; no fake input affordances); chrome = navigation only
  — no accounts/avatars/settings/notifications/feeds/vanity stat
  chips; section/page citations stay (clinician-native); ACE presented
  as the formal statement beside the source quote, never paraphrased.

Units. Shipped: 4.1→4.4 (adjudication UI consumable); 4.5→4.6→4.7
(question/answer/trace artifacts). Remaining: 4.13 alone — last M4
unit. Former 4.8/4.10 = M6 (graph artifact + views), 4.9 = M5, 4.11 =
M7 (unit text banked in those milestones); 4.12's review-scoped docs
fold into 4.13.

- M4.1 DONE (kernel) — strict E-- exception verbs: minimal
  `Try:`/`Catch <name>:` block statements in the vendor/e-- fork
  (charter: extend E-- liberally) — spec.md append, lexer/parser/emitter
  + strict red tests, regen identity; scope = the smallest construct
  letting `tools/ui.emm` express contract-specific 400/409/no-mutation
  outcomes around raising stdlib calls instead of generic wsgiref 500s
  (planrev L6; wsgiref handle_error stays the last-resort boundary).
  Apache fork notices per gate.
- M4.2 DONE (kernel) — adjudication ledger + staleness gate: pinned
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
- M4.3 DONE (kernel) — reviewer UI read core: `tools/ui.emm` serve —
  repo-level guideline picker (generic `guidelines/<id>` discovery,
  namespaced routes, absent-artifact families handled) → guideline index
  w/ status chips + meter → doc view: ACE text, the doc's coverage
  region (current corpus cardinality = exactly one region/doc; payload
  bytes = authority, section/page = display metadata), projection-notes
  kept/dropped, collapsible compiled Prolog, staleness explanation from
  the review manifest — pinned-vs-current review digest mismatch only
  ("bundle differs"): the ledger pins the bundle digest alone, so
  component attribution of historical drift is underivable from current
  state (M4.2 P20). Route/HTTP table + CLI contract (port, unknown-id
  behavior, methods, status codes) = unit contract deliverables. Total
  view-model join gate over EVERY guideline/doc (each docid exactly
  once; every ace() region joined; every projection row joined; no
  orphan routes) + golden pages for representative shapes + hostile
  corpus-string fixtures + design system w/ keyboard/contrast/focus
  invariants + chromiumfish headless visual QA. `ui.py render <outdir>`
  static export, byte-stable, covering every ace() region — carries the
  faithfulness-review polish row's acceptance verbatim (pruned at close).
  CLOSE: contract `.scratch/contracts/m4u3.md` (v1 + RV1-15 + AM1-10 +
  post-DONE amendments); committed gate = goal.emm `check_ui` (live meter
  `goal: ui ok 1 guidelines 188 pages` + fixture meter `goal: ui fixtures
  ok 63 red 11 green`; AM7 case format incl. tree-order/empty-dirs sidecar
  materialization); mutation 45/53 committed-killed (M09/M11/M48 via
  two-fault `missing-ace`) + 6 RV14-scratch (M24 M30 M31 M36 M47 M52) + 2
  RV4-removed, determinism 23/23 (920 renders one digest); F06 fuzz find
  (escaped-text href misread in extract_hrefs) fixed + `escaped-href-text`
  green regression; full check green. main=87% 208K pre-compaction + 26%
  62K post-compaction close,
  mate=104% 249K (rev2-m4u3, died provider-flag; next test-m4u3 92% 221K).
- M4.4 DONE (kernel) — adjudication mutation path shipped wholesale:
  review form (single current-state `<dl>`, CSRF + prefill) + POST
  pipeline steps 3-20 (Host/Origin/CT/decode/field-closure → CSRF →
  T7 subject-drift 409 → ledger CAS 409 → dest guard → mkstemp/write/
  fsync/chmod → snapshot ledger-validate → fault seam → `os.replace` →
  303) + request/serve CLI + `goal.py derive-review-manifest` +
  manifest-parity gate + POST-materialization harness law (after/ +
  absent-globs sidecars). Corpus: 17 verdict-* red cases / 117 rows
  (test-m4u4 harvest + 2 rev2 rows: duplicate-traversal, U+001F).
  Reviewer findings fixed + acceptance-passed: empty-segment 405
  shape, relative-root derive `resolve()`, one-dl panel, usage `*`
  (AM6 regen-clobber — both reviewer lenses caught impl-regenerated
  green). Gate: check ok 80 red/11 green fixtures, 374 pages; battery
  16/16. main=92% 221K peak (boundaries 89% 213K + 92% 221K) /
  mate=102% 245K (test-m4u4 past-window, steered out).
- M4.5 DONE (kernel, oracle) — question projection shipped as
  modes-in-file (spike ruling; vendor-closure polish row stays
  deferred): `question <tree> <qid> [ulex]` vector, pre-parse one-line
  law + post-parse query-sentence clause ahead of the shared empty-DRS
  guard, root/anchor/scan/markers/strip/goals pipeline over the reused
  rule-antecedent flatten, `'$guideline_query'` record +
  `'$guideline_query_projection'(goal(','/2), answers(manifest))`;
  document mode refined to per-Form `question_not_supported(wh(Tag)|
  universal|yesno, S)`; `query_box_check` shape law, condition-order
  Stage A walker (box-carrier descent only), strip anchor-transparency.
  tests/red: 6 pins regenerated + 8 new question families. Contract
  `.scratch/contracts/m4u5.md` R11 = 18 rulings. Evidence green
  (durable `.scratch/m4u5/`, replays in memory): suite 165/165;
  helpers 17/17; oracle differential rows 1-15 (row 13 = ruled whose
  notation); rev 8 findings + rev2 Stage-A decoy all accepted → MAIN
  fixed, acceptance 8/8 + H31; rev2 campaign credited 126/126, 51/51
  mutants killed, 97/97 reject closure, 800-cell determinism; doc
  isolation 186 compile diff-clean. main=99% 237K (2nd window; 1st
  ~200K) / mate=100% 240K (rev+rev2 ceiling; test 93% 224K).
- M4.6 DONE (kernel, oracle) — query artifacts + answers: qid grammar,
  `guidelines/<id>/queries/<qid>.ace` → committed `queries/pl/<qid>.pl`
  w/ binding record (question ACE digest, ulex digest; compiler
  identity ruled out in v1.3), `answer` mode over the aggregate
  manifest — bounded solve (per-solution + whole-query constants;
  inner-inference exhaustion cumulative across backtracking),
  canonical solution identity/dedup/order keys, result algebra incl.
  `indeterminate(limit)` ≠ `no`; committed `queries/answers/<qid>.pl`;
  check re-derives byte-identically, closes the `.ace`/`.pl`/answers
  inventory bijection + answers-golden custody, and requires committed
  demo queries to be `yes`/nonempty-`solutions`; 4 authored cdc
  queries (wh=3 yesno=1); answer subprocesses wall-bounded from birth.
  Contract `.scratch/contracts/m4u6.md` v1.2 + v1.3 Verdicts +
  v1.3.1/2. Evidence green (durable `.scratch/m4u6/`, replays in
  memory): suite 97/97; oracle differential 15/15 byte-identical; rev
  F01-F09 closed (3 compiler fixes: file_name verbatim syntax errors,
  record_shape wrapper functors, census EOF byte_count; F03
  answers-golden custody hole + probe rc1); rev2 campaign 24/24 (19
  committed + 3 fixture kills; G08/G09 compiler-seam kills → polish);
  fixtures 12 red + 4 green, pins 16/16 + answers-golden 8/8;
  compile+queries idempotent (diff-clean + byte-stable). main=2nd
  window ~100K (1st →240K compaction) / mate=102% 245K (test).
- M4.7 DONE (kernel, oracle) — proof traces: `trace` mode over the
  aggregate manifest re-derives each committed answer and emits
  `queries/traces/<qid>.pl` proof trees — clause nodes carry
  `sentence(DocId, S)` + pristine-clause sha256 (digest join back to
  committed `pl/` lines), NAF/limit leaves = explicit non-proof status
  (success-time NAF payload copies), bounded solve mirrors answer mode
  (row + whole-run sentinels); goal.py gains an ASCII-definitional
  trace scanner/validator (token classes v1.3.7, markers v1.3.6),
  four-way stem bijection, freshness + double-run determinism,
  traces-golden custody; 33-tree fixture corpus (24 red + 9 green
  floors pinned in goal.emm) via idempotent scratch generator.
  Contract `.scratch/contracts/m4u7.md` v1..v1.3.9. Evidence green
  (durable `.scratch/m4u7/`, replays in memory): suite 80 cases/332
  checks; check ok (186 documents; 4 traces nodes=68); regen --check +
  guidelines/ diff-clean; diff harness 36 ident + 3 ruled divergent
  (v1.3.1/v1.3.2); rev 6 repros green post-fix (E01-E08 acceptance
  ledger unrun — 2 content-filter deaths; compensated by suite + rev2
  campaign + orc + diff); rev2 50 mutants 41 killed + MAIN spot-check
  M02/M05, survivors ruled v1.3.9 (4 equivalent, 5 → polish
  kill-recipes row), determinism 64/64 byte-stable; tags
  archive/m4u7-{suite,diff,orc}. main=97% 232K (1st window →
  compaction; close in 2nd) / mate=102% 243K (test hw, 3 compactions).
- M4.13 OPEN (kernel) — review-UI design-law close, last M4 unit (no new
  tabs/routes; production surface = the shipped review UI + these
  deltas): apply the design law to shipped pages — build_doc_page stale
  panel + build_review_page subject panel lose visible digests/hex +
  `bundle differs` for plain language per the design-law wording,
  reviewer label plain (`as entered; not verified`), scope line in the
  shared page chrome; review-form CSS styling pass (site design system);
  print-clean; copy-register lint gate = committed check over ui.py
  emitted strings (emoji, gradient/animation CSS props, marketing
  lexemes, relative-time patterns, exclamatory copy) + red fixtures —
  consumes the two pulled polish rows (plumbing copy, form CSS; prune at
  close). Wholesale golden regen under the AM6 hazard law (diff churn vs
  contract before crediting green), fixture update, full check + battery
  green, chromiumfish visual QA. Docs close (absorbs old 4.12,
  review-scoped): README audit story (E-- UI + exception verbs,
  adjudication + rejection workflow, shipped question/answer/trace
  families + TCB closure naming, Running: `ui.py`, loopback security
  posture, licensing prose), guideline README + queue/rounds touch-ups,
  polish-register reconciliation. Milestone close per M1/M3 convention:
  M4 detail → `.agent/archive/m4.md`, live-law pointers (README +
  CLAUDE.md design law + Standing-direction tab law), roadmap slimmed.

- Unit gauges: M4.1 main=88% 212K close, mate=100% 239K (rev; map 69%
  test 71% rev2 57%). M4.2 main=94% 226K pre-compaction mid-close (33%
  80K post-compaction close), mate=89% 214K (rev; map 71% test 33%
  rev2 76%).

Sizing (planrev L1: five of seven M3 MAIN kernels hit 200-245K
pre-compaction; the ≤175K blanket was refuted): every unit sizes
against its nearest analog at planning-of-work time and targets ≤150K
MAIN with coordination reserve — M4.1 vs the E-- parser's existing block
statements (S); M4.2 vs the coverage gate + digest machinery (M); M4.3 +
M4.4 = the split that replaced the one-window-refuted UI unit (M4.3
largest E-- authoring, hard checkpoint at view-model contract; M4.4
narrow but adversarial); M4.5 vs M3.1/M3.2 compiler analogs (202-224K ⇒
narrowest possible grammar, oracle prep banked in parallel); M4.6/M4.7
single-concern compiler modes; M4.13 = copy/CSS/lint/docs increment on
the M4.3 chassis w/ the AM6 regen machinery — smallest M4 unit, ≤1
window. Kernel units take the full portfolio battery
(test/orc/rev/rev2 where `oracle`); prod-tagged artifacts ride their
named validator.

Assurance: durable gates = `python3 -P tools/goal.py check` + `python3
-P tools/regen.py --check`, extended in-place per unit (adjudication +
review-manifest, query/answer/trace freshness + inventory bijections,
M4.13's copy-register lint); every contract-defining
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

## M5 — query demonstrator tab — PARKED

Precondition: user promotes after M4 closes; refresh the plan in a
fresh session at promotion (M2 precedent). Consumes the shipped
M4.5–M4.7 artifacts (committed queries/answers/traces); develops +
wires per the Standing-direction tab law; governing law = CLAUDE.md
design law + the archived M4 UI/result-algebra rulings; M4.13's
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
  views stay out of v1. Derived-artifact machinery rides the M4
  compiler-closure ruling (smallest auditable closure).
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
