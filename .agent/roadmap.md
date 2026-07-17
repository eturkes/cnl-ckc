# roadmap

Charter = `.agent/initial-prompt.md`. Pipeline: E-- → Python (glue only) | ACE → APE-fork DRS → project IR → Prolog (all clinical semantics). Derived artifacts byte-accepted; corrections via regeneration only.

## Ledger

| id | milestone | deliverable | gate | status |
|---|---|---|---|---|
| M0 | feasibility + governance | upstream pins/licenses provenance doc; APE-on-SWI-9.2.9 probe; /goal persistence contract; Clex decision | — | REVIEWED |
| M1 | E-- trusted bootstrap | vendored hash-pinned e-- fork; strict slot-free canonical entrypoint; minimal glue extensions; self-checking `.emm→.py` regen/acceptance tool | M0 | UNPLANNED |
| M2 | deterministic ACE front end | vendored APE fork buildable on SWI 9.2.9; isolated hand-Prolog adapter → canonical byte-stable DRS; user-lexicon mechanism; stable doc/sentence IDs | M0 (APE strategy) + M1 (glue) | UNPLANNED |
| M3 | IR contract + vertical slice | versioned IR spec; hand-Prolog DRS→IR lowering + validator; thin e2e slice ACE→DRS→IR→Prolog→answer+explanation | M2 | UNPLANNED |
| M4 | rule compiler + kernel + explanations | definite-rule+NAF subset; safety/stratification validation; deterministic justifications traced to ACE sentence IDs | M3 | UNPLANNED |
| M5 | first guideline + manual pipeline | E---generated CLI orchestrating full chain; one real guideline authored in ACE e2e; guideline registry + English↔ACE mapping store; full-chain acceptance | M4 | UNPLANNED |
| M6 | resumable /goal workflow | registry-driven idempotent dynamic workflow; durable cross-session state; bounded termination (registry + discovery frontier empty) | M5 | UNPLANNED |

## M0 — feasibility + governance (REVIEWED)

Pins/licenses/decisions → `docs/provenance.md` (sole authority).
- M0.1 DONE (main=36% 98K/272K, impl=20% 53K/272K): governance doc + contracts → commit `2832a96`.
- M0.2 DONE (main=14% 39K/272K, impl=35% 94K/272K): APE-on-SWI-9.2.9 probe; evidence banked in M2 seed → commit `24a90ee`.

## Seeds (banked research, 2026-07-18)

### M1
- E-- current language: `Set…to`, `Do`, `Give back`, If/Otherwise if/Otherwise, While, For each…in, `Define [[f]] taking …` (+`defaulting`, kwargs), `[[name]]` calls, `<…>` lists, `{…}` dicts, `"s"`/nums/True/False/`Nothing`; NO mixed-operator precedence (parens required). Missing upstream: break/continue, imports, exceptions, classes, *args/**kwargs, hints, decorators, nested defs. `transpile()` pure; canonical-detect = try-parse; caches `.emm_cache.json`/`.emm_norm_cache.json`. Caution: fetched `docs/spec.md` self-describes v0.1.7 "no implementation yet" while repo has v0.2.0 tag + package — verify from actual tree at vendor time.
- Strict fork entrypoint > syntax extensions: lexer→parser→emitter only; reject any `{{ }}` slot pre-emission; must not import normalizer/resolver/Anthropic dep; exact UTF-8/LF/trailing-newline contract; stable diagnostic classes + exit codes; regen tool calls this API only (never normalizing CLI / `--run`). Determinism tests under perturbed CWD/locale/TZ/hash-seed/no-API-key/no-LLM-extra.
- Extension seed (minimal, defer rest): `Use dotted.module.` (whole-module import, top-level only); dotted refs (`args.write`, `sys.stderr`); dotted call targets (`[[pathlib.Path]]`, `[[subprocess.run]]`); `Require that <expr>.` → always-active guard raising AssertionError (NOT `assert`, dies under `-O`); `Exit with <expr>.` → SystemExit. Defer: aliases, from-imports, indexing, postfix chains, exceptions, context mgrs, classes, decorators, hints, break/continue.
- Unit sketch (recheck at M1 plan): vendor+pin+TCB file-allowlist (upstream tests pass unchanged) → strict profile spec+impl+fixtures → extension spec + red fixtures (expected stdout/status data + shell harness; never hand-author expected `.py`) → implement extensions in TCB → bootstrap `tools/regen.emm`→`regen.py` (check-mode default; atomic writes; detects drift/orphan `.py`/unauthorized Python/planted hand `conftest.py`; regenerates itself fixed-point) → generated smoke tests + CI (no inline Python, offline, no API key).

### M2
- APE nondeterminism found: global message store gates output (`drs([],[])` on any error); `copy_term` fresh vars → serializer must own canonicalization (`numbervars` in `drs_to_ascii` is deterministic but unquoted → prefer own serializer w/ fixed quoted/canonical options); durations in `get_ape_results` multi-output → bypass, call direct parser API from narrow hand-Prolog adapter; guessing off; fresh SWI process per compilation item; no timings/banners/paths in accepted output.
- Sentence IDs threaded text-wide (StartID monotone); condition annotations = `-SentId/TokId` relative ordinals → durable provenance = doc ID/source digest + sentence ordinal, never raw ordinals alone.
- Python side (M1 glue) starts process + moves bytes only; never inspects DRS semantics.
- SWI 9.2.9 probe @ APE `5f4d5354a45fb772763bf1a9543f508f15b28982`: `make install` builds runnable `ape.exe` (485064 B) with no warnings/errors; source CLI (`swipl -f ape.pl -g ape -t halt -- …`) and direct source load both parse with no load diagnostics. Direct adapter entrypoint = `ace_to_drs:acetext_to_drs/8` (`Guess=off`, `Catch=off`; own message/error policy + serializer).
- User lexicon verified: CLI `-ulexfile FILE`; direct adapter `ulex:discard_ulex/0` then `ulex:read_ulex/1` on a UTF-8 stream; `noun_sg(zorbomat,zorbomat,neutr).` makes `A zorbomat waits.` parse while the bundled Clex has no `zorbomat`.
- Fixture anchor (fresh SWI each): `John waits.` → `drs([A],[-(predicate(A,wait,named('John')),/(1,2))]).`; determiner+relative-clause, if-then, and user-lexicon fixtures also produced non-empty DRSs.
- Determinism: two fresh-process pairs (`John waits.` and relative-clause fixture) each `cmp` byte-identical under fixed quoted/canonical serialization (SHA-256 `42964785…` and `6cc328c3…`); accepted direct output contains no time/path/banner. Upstream `get_ape_results` multi-output is not byte-stable solely because `<duration parser=…>` changed (`0.007`→`0.005`; `cmp` 1).
- Upstream regression: documented full-Clex prerequisite + `test_ape.pl` completed in 7.38 s, exit 0, all 3733 correct (`----` 2813, `0000` 920). Its downloader targets moving Clex `master` (probe SHA-256 `2996fabf…`); without that 3.38 MB prerequisite, 131 cases falsely fail. Loading it over bundled Clex emits 27 static-procedure redefinition warnings.
- Patch inventory — SWI-9.2.9 MUST-FIX: none found for build/source-load/parse. Project MUST-FIX: bypass timed `get_ape_results`, own canonical serializer/error contract, and satisfy the full-Clex regression prerequisite by fetching full Clex at its provenance pin into gitignored scratch (test-only, never committed — provenance Clex scope) or curating a replacement corpus. Probe digest `2996fabf…` = `clex_lexicon.pl` at the Clex pin; Clex `master` currently == pin (review-verified), so fetch-at-pin reproduces the 3733/3733 run exactly. COSMETIC: silence regression Clex redefinition warnings and document explicit `-g ape -t halt` for source CLI.
- Strategy: source-load a narrow hand-Prolog adapter, not `ape.exe`; qsaved `ape.exe` works but is SWI/build-coupled, while the adapter exposes `acetext_to_drs/8` and owns accepted bytes deterministically.

### M3–M4
- IR contract from day one: stable doc/rule/sentence IDs; var scope+safety; explicit truth/unknown semantics; NAF representation + stratification rules; deterministic ordering/serialization; unsupported constructs = hard errors (never lossy lowering). M3 ends with thin executable slice (1 rule, 1 query, 1 explanation) to prevent IR encoding parser accidents; M4 widens subset. AceRules = semantics reference for courteous/stable-model choices.

### M5–M6
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
