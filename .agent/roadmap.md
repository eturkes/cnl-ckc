# roadmap

Charter = `.agent/initial-prompt.md`. Pipeline: E-- → Python (glue only) | ACE → APE-fork DRS → project IR → Prolog (all clinical semantics). Derived artifacts byte-accepted; corrections via regeneration only.

## Ledger

| id | milestone | deliverable | gate | status |
|---|---|---|---|---|
| M0 | feasibility + governance | upstream pins/licenses provenance doc; APE-on-SWI-9.2.9 probe; /goal persistence contract; Clex decision | — | REVIEWED |
| M1 | E-- trusted bootstrap | vendored hash-pinned e-- fork; strict slot-free canonical entrypoint; minimal glue extensions; self-checking `.emm→.py` regen/acceptance tool | M0 | REVIEWED |
| M2 | deterministic ACE front end | vendored APE fork buildable on SWI 9.2.9; isolated hand-Prolog adapter → canonical byte-stable DRS; user-lexicon mechanism; stable doc/sentence IDs | M0 (APE strategy) + M1 (glue) | REVIEWED |
| M3 | IR contract + vertical slice | versioned IR spec; hand-Prolog DRS→IR lowering + validator; thin e2e slice ACE→DRS→IR→Prolog→answer+explanation | M2 | IN-PROGRESS |
| M4 | rule compiler + kernel + explanations | definite-rule+NAF subset; safety/stratification validation; deterministic justifications traced to ACE sentence IDs | M3 | UNPLANNED |
| M5 | first guideline + manual pipeline | E---generated CLI orchestrating full chain; one real guideline authored in ACE e2e; guideline registry + English↔ACE mapping store; full-chain acceptance | M4 | UNPLANNED |
| M6 | resumable /goal workflow | registry-driven idempotent dynamic workflow; durable cross-session state; bounded termination (registry + discovery frontier empty) | M5 | UNPLANNED |

## M0 — feasibility + governance (REVIEWED)

Pins/licenses/decisions → `docs/provenance.md` (sole authority).
- M0.1 DONE (main=36% 98K/272K, impl=20% 53K/272K): governance doc + contracts → commit `2832a96`.
- M0.2 DONE (main=14% 39K/272K, impl=35% 94K/272K): APE-on-SWI-9.2.9 probe; evidence banked in M2 seed → commit `24a90ee`.

## M1 — E-- trusted bootstrap (REVIEWED)

Governance/pins → `docs/provenance.md`; fork identity/TCB allowlist/fidelity procedure → `vendor/e--/PROVENANCE`; strict contract → `docs/strict-profile.md`; regen contract → `docs/regen.md`; executable gates → `.github/workflows/ci.yml` + `tests/strict-harness.sh` (57) + `tests/regen-harness.sh` (16).
- M1.1 DONE (main=22% 60K/272K, impl=22% 59K/272K): vendored fork verbatim at pin + manifest/patch scaffold → commit `743e90a`.
- M1.2 DONE (main=40% 109K/272K, impl=47% 128K/272K): validated strict entrypoint (patches 0001/0002) → commit `fbafd49`.
- M1.3 DONE (main=20% 55K/272K, impl=46% 124K/272K): TCB language extensions (patch 0003) → commit `eb0aadb`.
- M1.4 DONE (main=33% 90K/272K, impl=36% 99K/272K): self-checking regen tool + trust-bootstrap CI → commit `167d2d7`.
- Review: 15 findings → 11 confirmed, 4 refuted; fixes = fail-loud iterdir traversal + single-pass buffered regenerate (regen.emm), patch 0004 Apache §4(b) notices, provenance patch enumeration, .gitattributes first-party gaps, record collapse → review commit.

## M2 — deterministic ACE front end (REVIEWED)

Context = `docs/research/cnl-ace.md` (fail-closed clinical profile, lexicon policy, §7 design decisions) + `docs/research/ir-rule-semantics.md` §7 (parse success ≠ profile acceptance); full texts in `docs/references/` (README = citations/hashes/rights). Charter boundary: adapter + serializer = hand-Prolog (parser machinery); all first-party Python = E-- generated.

- M2.1 DONE (main=29% 79K/272K, impl=75% 204K/272K): APE vendor + regression CI → commit `7af31ad`.
- M2.2 DONE (main=42% 115K/272K, impl=41% 111K/272K): fail-closed adapter + canonical serializer → commit `0105517`.
- M2.3 DONE (main=25% 68K/272K, impl=40% 109K/272K): optional per-run Ulex → commit `9900e11`.
- M2.4 DONE (main=29% 79K/272K, impl=40% 108K/272K): stable IDs + front-end glue → commit `1981be8`.
- Review disposition: lenses A–D + external audit; 14 findings validated → fix batches 1 (code/harness A–N: `'$VAR'` validator, strict UTF-8 stdin incl. hand decoder because SWI-9.2.9 `library(utf8)` is unsound, locale pinning, Ulex totality, stream quarantine, load-warning fail, glue Ulex-race/stderr/framing/preflight classes, two-doc zero-write, fixture equality, `make install "swipl=$SWIPL"` + `-f none` pinning, vendor precopy + manifest path-set gates) and 2 (docs truth-alignment); refuted findings booked = stable-ID overstatement, plan-commit convention, generated-file header; DEFERRED to M5 per `docs/ace-front-end.md` = producer/registry identities in records (`docs/research/cnl-ace.md` is a non-binding catalog per scope guard); gates = 57/16/45/27/10.
- Post-close residual audit (banked, non-blocking; shipped behavior unaffected — zero-write + nonzero exit hold in every case): gate-strength backlog = CI UTF-8 red matrix beyond 0xff (full RFC 3629 matrix probe-verified in-session only), cyclic-DRS + noisy-success-isolation fakes, further Ulex warning families, assert PASS_COUNT totals + surface unknown vendor-regression result codes, spawn-OSError + post-run ulex `read_bytes` tracebacks → sanitized error classes, qsave `make install` lacking `-F none` (vendor-anchor-only surface, needs vendored-Makefile patch); design-level (reaches user before any edit) = transient ulex replace-then-restore is invisible to upfront-read + post-run re-verify → true fix is an immutable snapshot handed to the adapter, which changes the adapter argv contract.

## M3 — IR contract + vertical slice (IN-PROGRESS)

Gate M2 confirmed 2026-07-19: all five harnesses rerun green (57/16/45/27/10). Plan basis: four planning lenses (adapter probes on a staged scratch APE tree; IR-contract derivation; architecture/split; DRS-report-6.7 + AceRules evidence), synthesis by MAIN. Nothing in scope depends on an unprobed ACE construct.

### Probe-pinned slice (FAST-PATH for M3.2–M3.4; authority = this block)

- `slice.ace` (one LF-terminated sentence per line): `John is a patient.` / `John waits.` / `Every patient that waits recovers.` / `Does John recover?`; `slice.ulex` = three LF-terminated lines `iv_finsg(recovers, recover).`, `iv_infpl(recover, recover).`, `noun_sg(patient, patient, human).` Adapter (staged tree per pipeline-harness recipe) accepts → stdout `drs([A,B,C],[-(object(A,patient,countable,na,eq,1),/(1,4)),-(predicate(B,be,named('John'),A),/(1,2)),-(predicate(C,wait,named('John')),/(2,2)),=>(drs([D,E],[-(object(D,patient,countable,na,eq,1),/(3,2)),-(predicate(E,wait,D),/(3,4))]),drs([F],[-(predicate(F,recover,D),/(3,5))])),question(drs([G],[-(predicate(G,recover,named('John')),/(4,3))]))]).` exit 0.
- Question shapes: `Does John recover?` → `drs([],[question(drs([A],[-(predicate(A,recover,named('John')),/(1,3))]))]).`; `Who recovers?` → `drs([],[question(drs([A,B],[-(query(A,who),/(1,1)),-(predicate(B,recover,A),/(1,2))]))]).` Antecedent conjunction = extra list conditions only (probed: `Every patient that waits and that coughs recovers.` + `iv_finsg(coughs, cough).`). Implication antecedent referents stay in scope in the consequent (`recover(D)`).
- Without the ulex every content word fails as `ape_messages` word errors (exit 1) — slice records must bind the exact ulex hash.

### Decisions (plan-fixed)

- Query = exactly one yes/no `question/1`, final root condition; wh `query/2` recognized → hard error (M4 owns answer binding). A declarative goal is indistinguishable from a fact — rejected as query encoding.
- Copula normalization (versioned in spec): `object(X,C,countable,na,eq,1)` + `predicate(_,be,named(N),X)` in the same factual DRS → class fact `C(named(N))` carrying both token anchors; every other `be`/`object` arrangement = hard error.
- Event referents: erase iff declared in the same DRS domain and used exactly once; any reuse = hard error (lossless-erasure proof, not silent drop).
- IR v1 = single ground record: `cnl_ir_record(1).`, document line byte-verbatim from the M2 record, then facts/rules/queries with `fact_id/rule_id/query_id(sentence(S),clause(C))`, `var(N)` data (never native vars), `source(sentence(S),tokens([...]))` provenance; shape-fixed name-open predicates, sort vocabulary `{entity}`.
- Semantics: positive function-free Datalog, least Herbrand model; outcomes `proved`/`not_proved` (rendered unknown, never false; no false constructor in v1); NAF reserved in grammar + validator-rejected; stratification stated, trivially satisfied; safety = every var covered by positive body, facts/queries ground; positive dependency cycles rejected.
- Unsupported constructs = deterministic hard errors (no residuals in v1); `ir_tool_error(Stage,Class,Detail).` stderr line, adapter-style exits 0/1/2, zero stdout on failure, canonical fixed-point serialization (drs_canon writer options).
- Tooling: one `src/prolog/ir_tool.pl` CLI — subcommands `lower|validate|compile|run`, fresh `swipl -q -f none -F none` process per stage; semantic modules `drs_to_ir.pl`, `ir_validate.pl`, `ir_to_prolog.pl`, `inference_kernel.pl`, `explanation.pl`; reuse `drs_canon`; M2 modules untouched.
- Program artifact = directive-free versioned term stream, whitelist-validated before install into a private module; kernel = deterministic meta-interpreter, first proof under program clause order = canonical witness, replayed before emission; result record `cnl_answer_record(1).` + verbatim document line + answer/derivation certificate.
- Artifacts per docid: `.ir.pl`/`.program.pl`/`.result.pl`; fixtures `tests/fixtures/ir/` (hand-authored validator green/red inputs) + `tests/fixtures/slice/` (ACE/ulex docs + regenerated chain goldens); zero changes to `tools/ace_front_end.emm`/generated Python; orchestration = bash harness (M5 owns the real E-- CLI).
- Deferrals (named, M4+): intervals/rationals, temporal/dose algebra, direction/strength/certainty, explicit negation + false outcome, NAF execution/exceptions, recursion/tabling, conflict detection, proof enumeration/DAG sharing, prose rendering, program-digest binding inside answer records, wh answers, multi-document composition.

### Units

- M3.1 DONE (main=31% 85K/272K, impl=55% 149K/272K): IR v1 spec + validator + CLI — `docs/ir.md` (grammar fact/3 rule/4+body/1 query/3, pred(Name,Args) name-open, named/1 + rule-only var/1 data, naf reserved+rejected, one ground query, positive acyclic Datalog, proved/not_proved-as-unknown; class table input_utf8/syntax/canonical/envelope/query_count/section_order/shape/identity/ordering/scope/naf/safety/cycle→1, usage/uncaught→2; stage cli pre-dispatch else validate); `src/prolog/ir_tool.pl` (framing: byte-strict UTF-8, pinned reader flags, canonical fixed point vs `drs_canon` `canonical_line/2`, buffered stdout, M2 document line = sole forced-quote serializer exception) + `ir_validate.pl` (passes envelope→shape→identity→ordering→scope→safety/NAF→cycles; within-pass: query-count before trailing/interleave, duplicate-id before section order before tokens, per-rule NAF before safety); fixtures 3 green + 31 red; `tests/ir-validate-harness.sh` 53 gates incl. scratch UTF-8/CRLF/BOM/no-final-LF/spacing/operator-notation, usage, 2× determinism, hard PASS_COUNT=53; CI lint + ape-job step → commit follows.
- M3.2 DONE (main=42% 113K/272K, impl=74% 201K/272K): DRS→IR lowering — `drs_to_ir.pl` (M2 3-term envelope, copula normalization, one-rule/one-question profile, event-erasure + full referent accounting, `==`-exact profile matching after audit caught unification-based matchers silently instantiating canonical native variables incl. aliases of validated referents) + `lower` subcommand sharing validate framing; slice + slice-unknown fixtures front-end-generated with IR goldens (regen byte-equal ×5); 18 red fixtures incl. both aliasing counterexamples; `tests/ir-lower-harness.sh` 27 gates; docs/ir.md lower section + class table (`question_count`/`wh_query`/`copula`/`referent`/`unsupported`); CI wired; all harnesses green (57/16/45/27/10/53/27) → commit follows.
- M3.3 DONE (main=29% 79K/272K, impl=47% 129K/272K): compiler + kernel + explanation — `ir_to_prolog.pl` (compile = full IR validation → provenance-dropping order-preserving total map → program self-validation), `inference_kernel.pl` (program validator isomorphic to IR passes 4–10, same class vocabulary = tamper surface; deterministic least-model kernel: private `cnl_program_db` under setup_call_cleanup, insertion-ordered witness store, first-witness-kept, per-clause entry snapshot, leftmost-outermost `==`-structural matching, normative schedule in docs), `explanation.pl` (witness→proof tree + independent structural replay gate before output; replay failure = uncaught/2); `compile`/`run` subcommands with shared framing + generated-record self-fixed-point; `cnl_program_record(1)`/`cnl_answer_record(1)` grammars in docs/ir.md; slice program/result goldens (proved 3-node proof + not_proved no-proof) + 2 hand-green pairs (name-capture data-only predicates, body-only variable) + 14 class-complete red fixtures; `tests/ir-run-harness.sh` 31 gates APE-free (chain DRS→lower→compile→run vs goldens, stage-pin compile-naf, canonical/UTF-8 scratch, usage, 3× determinism); ir-validate-harness unknown-command probe updated (compile now implemented); CI wired; harnesses 53/27/31/57/16 green + multi-pass nested-proof probe → commit follows.
- M3.4 OPEN (gate M3.3) — end-to-end slice + closure: `tests/slice-harness.sh` (staged APE tree → front end over `tests/fixtures/slice/docs` → per-record lower→validate→compile→run → full byte comparison vs committed goldens, run-twice determinism, zero-downstream-write on failure, vendor cleanliness); docs truth-alignment (ir.md chain section + CI enumeration); CI closure. Projected ≤110K.

## Seeds (banked research, 2026-07-18)

### M3–M4
- Research (2026-07-18): `docs/research/ir-rule-semantics.md` = requirements/pitfalls catalog — IR phenomena table, exact-rational (dense-Q) interval algebra + value×marker anti-vacuity tests, NAF/classical-negation/normative-opposition separation, conflict cores + reason-coded no-conflicts, DRS-lowering boundary, degeneration modes; AceRules = bounded differential reference only (reparses text, loses provenance, lacks direction/strength).
- IR contract from day one: stable doc/rule/sentence IDs; var scope+safety; explicit truth/unknown semantics; NAF representation + stratification rules; deterministic ordering/serialization; unsupported constructs = hard errors (never lossy lowering). M3 ends with thin executable slice (1 rule, 1 query, 1 explanation) to prevent IR encoding parser accidents; M4 widens subset. AceRules = semantics reference for courteous/stable-model choices.

### M5–M6
- Research (2026-07-18): `docs/research/guideline-sources.md` = vetted source catalog + per-artifact rights + registry row schema; first-guideline shortlist = CDC 2022 opioid 12-recommendation skeleton (public domain) first, WHO SMART ANC CC0 repo subset as differential oracle second, WHO immunization single-schedule slice third. `docs/research/evaluation-methods.md` = layered acceptance gates (claim_completeness = 1, exact-IR match over behavioral verdicts, proof replay, fresh-process hash equality, degeneration probes w/ positive+negative controls).
- Durable state before automation: guideline registry (URL/version/date, content digest, licensing/redistribution status, may-commit flag), mapping store, item states + blocked reasons; M5 proves one manual idempotent round (stop/restart, no dup/lost work). M6 wraps in dynamic workflow: parallel source research, serialized registry/ACE/compiler writes; no mid-run user input → escalations become BLOCKED proposals.
- Dynamic-workflow availability (verified 2026-07-18): Claude Code `2.1.211` ≥ required `2.1.154`; neither project nor user `.claude/settings.json` sets `disableWorkflows`, and `CLAUDE_CODE_DISABLE_WORKFLOWS` is unset; user confirmed the feature is enabled for their account (2026-07-18) → available, no residual gate. Official refs: `code.claude.com/docs/en/workflows`, `code.claude.com/docs/en/goal`, `code.claude.com/docs/en/skills`.
- Semantics: dynamic workflows = JavaScript scripts orchestrating subagents. Active run state is **not** durable across CLI exit; docs: exiting mid-run ⇒ “the next session starts the workflow fresh.” Definitions under `.claude/workflows/` are durable + invokable as `/<name>`, but invocation reruns the definition rather than resuming engine state. `/goal` = session-scoped Stop-hook completion loop; active goal restoration requires `--resume` of the same saved session. Agent repository writes persist normally.
- M6 persistence contract = **repository files only**:
  1. **State:** all round state in-repo = guideline registry + mapping store + item states/blocked reasons.
  2. **Round:** work selection = deterministic function of registry state; intake persists each fetched source as an immutable content-addressed record (URL, observed version/date, SHA-256, bytes or digest-verified locator, licensing observation) BEFORE any generative processing; changed remote content ⇒ new versioned item, never mutation of a prior item; writes atomic (temp + rename; registry last).
  3. **Recovery:** content-digest-keyed items + idempotent transitions; restart reconciles staged/orphaned artifacts against the registry (adopt or discard deterministically); rerun after any interruption ⇒ no duplicate or lost work.
  4. **Escalation:** no mid-run user input; licensing/semantic decisions become `BLOCKED` proposal registry entries.
  5. **Termination:** quiescent = every item ∈ `{DONE, BLOCKED}` ∧ frontier empty ⇒ pause; report non-success while BLOCKED proposals pend; resolving a proposal returns its item to runnable. Complete = every known guideline DONE or user-approved terminal exclusion ∧ frontier empty.
  6. **Invocation:** `/goal` = required in-session completion driver over the saved workflow definition; every invocation starts fresh + resumes solely from repository state; repository files = the only persistence.
