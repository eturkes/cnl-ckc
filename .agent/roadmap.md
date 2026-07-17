# roadmap

Charter = `.agent/initial-prompt.md`. Pipeline: E-- → Python (glue only) | ACE → APE-fork DRS → project IR → Prolog (all clinical semantics). Derived artifacts byte-accepted; corrections via regeneration only.

## Ledger

| id | milestone | deliverable | gate | status |
|---|---|---|---|---|
| M0 | feasibility + governance | upstream pins/licenses provenance doc; APE-on-SWI-9.2.9 probe; /goal persistence contract; Clex decision | — | IN-PROGRESS |
| M1 | E-- trusted bootstrap | vendored hash-pinned e-- fork; strict slot-free canonical entrypoint; minimal glue extensions; self-checking `.emm→.py` regen/acceptance tool | M0 | UNPLANNED |
| M2 | deterministic ACE front end | vendored APE fork buildable on SWI 9.2.9; isolated hand-Prolog adapter → canonical byte-stable DRS; user-lexicon mechanism; stable doc/sentence IDs | M0 (APE strategy) + M1 (glue) | UNPLANNED |
| M3 | IR contract + vertical slice | versioned IR spec; hand-Prolog DRS→IR lowering + validator; thin e2e slice ACE→DRS→IR→Prolog→answer+explanation | M2 | UNPLANNED |
| M4 | rule compiler + kernel + explanations | definite-rule+NAF subset; safety/stratification validation; deterministic justifications traced to ACE sentence IDs | M3 | UNPLANNED |
| M5 | first guideline + manual pipeline | E---generated CLI orchestrating full chain; one real guideline authored in ACE e2e; guideline registry + English↔ACE mapping store; full-chain acceptance | M4 | UNPLANNED |
| M6 | resumable /goal workflow | registry-driven idempotent dynamic workflow; durable cross-session state; bounded termination (registry + discovery frontier empty) | M5 | UNPLANNED |

## M0 — feasibility + governance (IN-PROGRESS)

Pins (`git ls-remote`, 2026-07-18):
- e-- `main` = `da8c3b34d2493180da8df65b127a3841f9a4e609` (tags v0.1.0, v0.2.0=`a6cb6cae0ae1149f51b6065748d96e926ba4890b`; Apache-2.0)
- APE `master` = `5f4d5354a45fb772763bf1a9543f508f15b28982` (LGPL-3.0-or-later incl. bundled `prolog/lexicon/clex_lexicon.pl` ~few-K entries; dormant since ~2013, last release 6.7-131003)
- Clex `master` = `20960a5ce07776cb211a8cfb25dc8c81fcdf25e2` (GPL-3.0, ~100K entries)
- AceRules (tkuhn) `master` = `5b7afb7bdfbce56027997307f9b798af53551223` (semantics reference: courteous logic / stable models)
- RACE: no source (webservice only) → reasoner is ours (M4)

### M0.1 OPEN — governance doc + contracts
Scope: create `docs/provenance.md` (pins above, roles, licenses, TCB boundary definition, per-vendor-dir license layout). Decide Clex: default = exclude (GPL); start from APE bundled lexicon + project-owned declarative lexicon manifest compiled to APE user-lexicon facts — record decision + revisit trigger. Repo top-level license = proposal only, flag for user (requirement-level). Verify Claude Code dynamic-workflows availability in installed version + persistence semantics (docs: resume is session-local) → write /goal persistence contract into M6 seed (durable on-disk registry, idempotent rounds, BLOCKED-proposal mechanism for licensing/semantic escalations, termination = bounded registry + documented frontier empty).
Acceptance: provenance doc committed; Clex decision recorded w/ rationale; /goal contract banked in M6 seed; user-facing license proposal flagged in close report.

### M0.2 OPEN — APE-on-SWI-9.2.9 probe
Scope: scratch clone (`/tmp`) Attempto/APE @ pin; attempt `make install` (ape.exe) AND plain source-load of `prolog/ape.pl` under swipl 9.2.9; parse fixture ACE sentences (incl. 1 user-lexicon word via `-ulexfile`); double-run byte-compare DRS output (determinism); run upstream regression corpus if feasible; inventory needed patches (deprecated builtins etc.).
Acceptance: build/load result + patch inventory + fixture DRS evidence banked into M2 seed; APE strategy decision recorded (source-load adapter vs ape.exe; ape.exe failure nonfatal if source-load works — saved-state exe is SWI-build-coupled, poor accepted artifact); determinism result recorded. No vendor commit (fork lands in M2).

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

### M3–M4
- IR contract from day one: stable doc/rule/sentence IDs; var scope+safety; explicit truth/unknown semantics; NAF representation + stratification rules; deterministic ordering/serialization; unsupported constructs = hard errors (never lossy lowering). M3 ends with thin executable slice (1 rule, 1 query, 1 explanation) to prevent IR encoding parser accidents; M4 widens subset. AceRules = semantics reference for courteous/stable-model choices.

### M5–M6
- Durable state before automation: guideline registry (URL/version/date, content digest, licensing/redistribution status, may-commit flag), mapping store, item states + blocked reasons; M5 proves one manual idempotent round (stop/restart, no dup/lost work). M6 wraps in dynamic workflow: parallel source research, serialized registry/ACE/compiler writes; no mid-run user input → escalations become BLOCKED proposals.
