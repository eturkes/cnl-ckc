# Adjudication UI + query/trace compiler foundation (M4)

REVIEWED milestone record; stub = `.agent/roadmap.md`. Live law =
README (schema, Operating, Export, query/trace sections) + the
clinician design law `.claude/rules/clinician-design.md` + the
architecture rulings below.
Review record = the final section of this file.

Goal (rescoped: the query-demonstrator tab, graph-explorer tab +
portable-KB-dist units left M4 and are out of project scope —
feature-complete ruling): a local reviewer UI (strict E--,
zero JS) that lists every ACE document with review status, shows each
beside its exact source region and compiled Prolog,
and records approve/reject + comment verdicts in a gated audit ledger
(M4.1–M4.4), plus the question→answer→trace compiler (M4.5–M4.7); M4.13
closes the review surface under the clinician design law; post-close
user-led style iteration (functionality-stable) on demand.

Terminal state (at M4.13 close d119e25): `python3 -P tools/goal.py check` +
`python3 -P tools/regen.py --check` green — `goal: check ok 1 guidelines
186 documents 29 red probes`, `goal: ui ok 1 guidelines 189 pages`, 4
committed queries with answers + traces; fixture corpora `ui fixtures ok 75
red 13 green`, `adjudication fixtures ok 42 red 9 green`, `copy fixtures ok
10 red 2 green`; `goal: copy ok 1353 literals`;
`.scratch/regen-ui-fixtures.sh` fail=0; clinician QA 16 screens (2
partial-top captures) + 3 print sheets (`.scratch/m4u13-qa/`).

Plan reviewed adversarially at planning time: `planrev-m4` report =
`.scratch/agents/planrev-m4.md` (8 lenses; the unit split, tiers,
question grammar, staleness model + mutation-safety contracts
below arbitrate its findings). Feasibility evidence = `.scratch/m4plan/`:
`ui_spike.emm`/`.py` (strict-E-- WSGI serve + form POST green; NOTE its
`write_text` is NOT atomic — the real ledger write protocol is M4.4's),
`question_probes.sh` + `.out` (9 replayable APE question-DRS families).

Architecture rulings:

- UI = one local WSGI server compiled from strict E-- (`tools/ui.emm` →
  `tools/ui.py`; wsgiref, 127.0.0.1, single-threaded), every page
  server-rendered HTML from committed state; zero JavaScript, zero
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
  stamps), re (census/anchor parsing), tarfile (committed snapshot
  extraction) —
  hand-written code owns project semantics only, never HTTP parsing, URL
  grammar, archive encoding, or fs primitives.
- Serve time reads committed artifacts + the adjudication ledger only —
  literally: `serve`/`render` read the guideline tree at HEAD (temp
  snapshot when the working tree carries uncommitted guideline changes,
  live ledger overlaid), so an uncommitted edit neither shows nor
  outdates a decision and a never-committed document is not reviewable;
  `check` alone reads the working tree, being the pre-commit gate. No
  swipl, no LLM, no socket beyond loopback. All derivation (query
  compile, answers/traces) runs in the pipeline and is committed
  + freshness-gated exactly like `pl/`. Every M4 gate backing a durable
  claim reruns from committed state (project rule); scratch batteries =
  supplemental only.
- Queries = ACE interrogatives, CNL all the way down. Closed v1 query
  grammar (probe corpus `.scratch/m4plan/question_probes.out`):
  existential conjunctive wh (`query(Ref, who|which|what)` referents) +
  existential/ground yes-no + modal operator boxes (render as operator-box
  goals per the rule-body precedent); universal yes-no (parses to an
  implication DRS), disjunction (`v/2`), classical negation and NAF inside
  questions all REJECT with named details — an entailment/countermodel
  semantics is out of scope. Result algebra: wh → `solutions([...])`;
  yes-no → `yes` | `no(finite_failure)` | `indeterminate(limit)` (M4.7
  adds the trace-bearing yes); a bound hit is never rendered as "no".
  Answer artifacts carry raw KB atom values (v1.3); provenance coordinates
  ride M4.7 traces (UI dereference = out of scope);
  cardinalities display as verbatim schema payloads, never computed
  quantities; no natural-language glue, no runtime LLM.
- Derived-artifact machinery (query compile, answer/trace) lands in the
  named first-party Prolog closure of the APE fork: inside
  `ace_to_pl.pl` as modes, or — if the M4.5 spike measures lower total
  trusted code — as one additional first-party module beside it, with
  PROVENANCE `First-party files:`, README audit story and inventory gates
  updated in the same unit. Smallest auditable closure decides,
  not file count. Committed answers/traces are compiler-DERIVED
  products carrying no hand-authored knowledge; hand-authored `.pl` stays
  barred from guideline trees.
- Adjudication review subject = what the reviewer actually saw: each
  ledger row pins `review_sha256`, a canonical bundle digest over the
  doc's ACE bytes + its coverage region row + region payload bytes + its
  semantic-clause digest — `bundle v2`, the projection-notes component
  having left with the disclosure panel it fed (M4.13 v5.6). Clauses are
  canonicalized minus the volatile document-record header, so a pure
  vocabulary append outdates nothing while a reinterpreting ulex/Clex/
  compiler change outdates exactly the semantically-changed docs (the
  append-stability polish row folds in here). Component digests live in a
  check-derived `audit/review-manifest.tsv`; drift attribution stays
  underivable from current state (M4.2 P20), so the UI reports that a
  decision applies to an earlier version and never why. Verdict rows are
  append-only, each pinning the `ace_commit` it judged; history = the
  ledger + git (README Operating gains a commit-after-each-review-batch
  checkpoint); `reviewer` = self-asserted local identifier (canonical
  grammar, no control/tab/newline), date = server-generated UTC ISO — the
  UI displays the self-asserted limitation. Rejected documents = `/goal`
  repair worklist with an Operating priority rule (live rejected docs
  outrank new extraction/fetch work); meters, never check failures.
- Mutation safety (the one write path): POST carries the rendered
  `review_sha256` back as a hidden field and the server re-compares
  against committed state immediately before persist (409 + no mutation
  on drift); ledger writes are compare-and-swap on the prior ledger
  digest through exclusive same-dir temp + flush + fsync + `os.replace`;
  CSRF = per-process unpredictable token required on every POST +
  loopback Host check + same-origin Origin-when-present; validation = ONE
  shared implementation (a `goal.py` ledger-validate subcommand the UI
  invokes via subprocess before persist) so gate and UI grammars cannot
  drift.
- Clinician-facing design law (all UI surfaces; durable law =
  `.claude/rules/clinician-design.md`, mechanical enforcement = M4.13's copy-register
  lint + the design-system invariants): visible page copy carries zero
  plumbing — digests, qids, lemma symbols, raw schema functors ride
  URLs/hidden fields/ledgers only; staleness, verdict + limit panels read
  as plain language (outdated = the document or its source changed after
  this verdict;
  `indeterminate(limit)` = not determinable within the system's search
  bounds; empty `solutions` = none stated in the loaded guidelines);
  closed cardinality operators render as symbols (`eq`→`=`, `geq`→`≥`)
  beside verbatim payload atoms; every page carries the scope line —
  reports what the loaded guideline documents state, not clinical advice;
  capabilities stated plainly (prepared questions only; no fake input
  affordances); chrome = navigation only — no accounts/avatars/settings/
  notifications/feeds/vanity stat chips; section/page citations stay
  (clinician-native); ACE presented as the formal statement beside the
  source quote, never paraphrased.

Units — 8 shipped: 4.1→4.4 (adjudication UI consumable); 4.5→4.6→4.7
(question/answer/trace artifacts); 4.13 (review-UI design-law close).
Former 4.8–4.11 rescoped out of M4, later dropped from project scope
(feature-complete ruling); 4.12's review-scoped docs folded into 4.13. Narratives + SHAs = `git log --grep "(M4[. ]" -p --
.agent/roadmap.md .agent/archive/`.

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
  review-batch commit checkpoint. (M4.13 v5.x superseded the one-row-per-
  docid ledger with the append-only ledger + `ace_commit` column.)
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
  62K post-compaction close, mate=104% 249K (rev2-m4u3, died provider-flag;
  next test-m4u3 92% 221K).
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
  modes-in-file (spike ruling): `question <tree> <qid> [ulex]` vector, pre-parse one-line
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
  mutants killed, 97/97 reject closure, 800-cell determinism, doc
  isolation 186 compile diff-clean. main=99% 237K (2nd window; 1st ~200K)
  / mate=100% 240K (rev+rev2 ceiling; test 93% 224K).
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
  committed + 3 fixture kills; G08/G09 compiler-seam kills =
  scratch evidence, port dropped);
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
  M02/M05, survivors ruled v1.3.9 (4 equivalent, 5 =
  scratch-documented suite gaps; port dropped), determinism 64/64 byte-stable; tags
  archive/m4u7-{suite,diff,orc}. main=97% 232K (1st window →
  compaction; close in 2nd) / mate=102% 243K (test hw, 3 compactions).
- M4.13 DONE (kernel) — review-UI design-law close, last M4 unit: the
  design law applied to every shipped page (plumbing out of visible
  copy, plain-language state, scope line in the shared chrome, form CSS,
  print-clean) plus six user-directed revision commits — review-round
  vocabulary + mandatory named reviewer, published source route +
  guideline-first heading order, append-only ledger + records page,
  `ace_commit` version links, the committed-corpus read surface, and the
  removal of the differences panel + every UI trace of committed-versus-
  uncommitted state (bundle v2). Copy law shipped as two gates
  (`check_copy_register` over ui.py's emitted strings +
  `page_invariant_name` inside `ui.py check`), pinned in place by
  `check_copy_chain_slot`. Contract of record `.scratch/contracts/m4u13.md`
  (v1..v4 + v5.1-v5.6 rulings). Close gates: `goal.py check` +
  `regen.py --check` green; `check_ui` 75 red / 13 green fixtures + live
  189 pages; `check_copy_register` 1353 literals; `regen-ui-fixtures.sh`
  fail=0; clinician QA rebuilt (16 screens, 2 of them partial-top
  captures, + 3 print sheets, `.scratch/m4u13-qa/`). The
  committed-corpus law is invisible by user
  ruling, so it rides two `worktree/` git-corpus fixtures rather than
  copy — machinery + non-vacuity probe in `.agent/memory.md`.

Unit gauges: M4.1 main=88% 212K close, mate=100% 239K (rev; map 69% test
71% rev2 57%). M4.2 main=94% 226K pre-compaction mid-close (33% 80K
post-compaction close), mate=89% 214K (rev; map 71% test 33% rev2 76%).

Sizing (planrev L1: five of seven M3 MAIN kernels hit 200-245K
pre-compaction; the ≤175K blanket was refuted): every unit sizes against
its nearest analog at planning-of-work time and targets ≤150K MAIN with
coordination reserve — M4.1 vs the E-- parser's existing block statements
(S); M4.2 vs the coverage gate + digest machinery (M); M4.3 + M4.4 = the
split that replaced the one-window-refuted UI unit (M4.3 largest E--
authoring, hard checkpoint at view-model contract; M4.4 narrow but
adversarial); M4.5 vs M3.1/M3.2 compiler analogs (202-224K ⇒ narrowest
possible grammar, oracle prep banked in parallel); M4.6/M4.7
single-concern compiler modes; M4.13 = copy/CSS/lint/docs increment on the
M4.3 chassis w/ the AM6 regen machinery — smallest M4 unit, ≤1 window.
Kernel units take the full portfolio battery (test/orc/rev/rev2 where
`oracle`); prod-tagged artifacts ride their named validator.

Assurance: durable gates = `python3 -P tools/goal.py check` + `python3 -P
tools/regen.py --check`, extended in-place per unit (adjudication +
review-manifest, query/answer/trace freshness + inventory bijections,
M4.13's copy-register lint); every contract-defining fixture set lands
committed and check-reachable, scratch campaigns supplemental; the UI
mutation path is the sole new write surface, guarded by the shared
validator + safety battery.

Out of scope: runtime LLM anywhere in UI or answers; free-text/ad-hoc
query entry (queries enter as committed ACE through ordinary rounds);
universal/entailment question semantics; ACE editing from the UI (repairs
stay `/goal` rounds); remote or multi-user serving, auth beyond the
loopback CSRF posture, TLS; JavaScript or any browser-side computation;
WASM Prolog; defeasibility; serving APIs (standing direction unchanged for
downstream consumers).

## Final project review record (M4 review + project close)

Review commits (`git log --grep "(M4 review)"`, base 574a192): 5d206b5
polish rows 6+7 → 9071405 README claim batch → 7a00152 F1/F3/F10/F5 +
grammar totality → 3c7a9a5 F13 clinician leaks + fixture adoptions →
941ff09 u3 P1/P2 (manifest dup shadow, dup-detail order, ASCII digit
guards) → ec98622 B1 polish rows 1-5 (walled swipl runner, cwd-free
require, census-map gate, ledger-commit verify; register closed) →
f738a16 B2 (golden-lane pin map, fixture census counts,
bigint/backtracking cases) → 8dc4e45 C (clause census, row-birth
invariant, canonical_error_tree, float_class law + trace probe,
carrier-table anchors) → a2da7d4 + 511a6d2 KB export. Portfolio:
rev-m4u1..u7 + rev-m4u13 per-unit, xrev-m4 cross-cutting, audit-m4
24-row claim replay, res-dist format survey (BagIt 1.0 ruling),
test-dist 62-case diff-blind red suite. gpt-5.6-sol's content filter
killed rev-m4u4 ×4 + xrev-m4 on security vocabulary; reports harvested,
gaps recorded below. Worktree evidence tips preserved as
`archive/m4rev-<name>` tags; worktrees + `wt/` branches deleted.

Fixed findings (acceptance-passed under MAIN rerun): F1 ledger CAS
TOCTOU → flock sibling + pre-replace digest re-verify (council ×2:
xrev e45ce94, u13 e155cce). F2 adjudication fixture floor 42/9. F3
resolve_corpus races → hex resolved once per operation, `git archive
<hex>`, always-snapshot when git usable. F4 trace-reject lane + exact
stem pins. F5 `request --commit` 40-hex law. F6 strict usage/io rc2
in-gate self-probes through exit_two. F7 M4U1_ROOT battery replay
(absolute fixture paths). F8 memory Catch/SystemExit precision. F9
census int totality (locator-less rows = canonical-string compare;
goal-side eager int pinned red) + valid_ui_id encode guard. F10 POST
body read only after route/model/Host/Origin/CT guards; GET never
reads the body. F12 query_anchor payload preemption → carrier-table
walk. F13 clinician id leaks → title links, citation crumbs,
Current/Earlier labels, copy-functor + copy-short-hex invariants. F14
non-finite floats → float_class ∉ {infinite,nan} + goal
check_trace_nonfinite_probe. F15 golden-regen sha1 pin (ui_git_env).

Audit-m4: 24 rows verified except era-bound scratch evidence — m4u6
suite/differential, m4u4 battery, m4u13 diff-blind suite (dispositions
+ current numbers = memory ERA-BOUND annotations; replay from
claim-era commits) — and the QA count (16 screens, archive corrected);
7 archival families skipped on cost (incl. m4u7 determinism 64/64).

Accepted residuals + rulings (no further code change):
- u3 page-invariant lexical over-match: pre-text over-match stays;
  esc_text renders it harmless, unreachable via compiler corpus.
- resolve_corpus filesystem mode remains for git-unusable trees
  (fixture surface); live path = snapshot-only (F3).
- No POST body-size cap: forged Host+Origin = local caller = outside
  the loopback threat model; availability residual (u4 R16).
- rev-m4u4 row 5: no independent reviewer verdict-corpus pass
  (reviewer filter-dead ×4); compensated by the committed verdict-*
  corpus + MAIN acceptance-checklist reruns.
- u1 row 6: mixed-invalid duplicate precedence across strict fixture
  classes = contract spec gap; current behavior stands.
- ui Try-guarded int() accepts Unicode-Nd in Content-Length/port;
  guarded + value-identical → retained.
- F11 derive-review-manifest tmpdir leaks on the violation exit path;
  process-exit scope → retained.
- Cross-class dup-vs-malformed coverage precedence = malformed-first.
- F14 embodiment deviation: trace-reject-pair rc2 design superseded by
  check_trace_nonfinite_probe (TR lane pins rc2-only; observed class =
  rc1 proof).
- test-dist a04 expectation corrected g-fetch → g-red (scenario
  mutates g-red; suite defect).
- README-dist intro head dropped: an embedded input-head drifted
  README-dist.md on every corpus commit, and it key-sorts before
  data/**, breaking the pinned manifest_source_drift path.

KB export shipped (KB production bar): `tools/dist.emm`→`dist.py`
committed-state derivation + BagIt 1.0 build, goal `check_dist` (6
self-probes, live build ×2 byte-compare, layout closure, `sha256sum
-c`), `release-manifest.tsv` writer, `tests/dist/red.sh` 62/62,
`guidelines/cdc-2022-opioid/rights.tsv` (redistributable, MMWR
public-domain quote). Live meter `goal: dist ok 1 guidelines 408
members 1203980 bytes` (input-head a2da7d4). UI production bar
recorded: README Export — loopback-local, web hosting out of scope.
Operating mechanics = memory "KB export" bullet.

Close gates (post-dist, HEAD 511a6d2): full `goal.py check` rc=0 —
`check ok 1 guidelines 186 documents 29 red probes`; meters strict
36/14, adjudication 42/9, ui fixtures 81/14, ui 189 pages, copy 1414
literals, queries fixtures 24/11, dist ok above; `tests/dist/red.sh`
62/62; `regen.py --check` ok; fresh compile + `git diff --quiet --
guidelines/` at the close commits. Session shape: MILESTONE-REVIEW
across 6 compaction checkpoints, MAIN ~13.5M tokens; teammate
high-water 249K (rev2-m4u3 era analog); review teammates ≤67%
pre-harvest.
