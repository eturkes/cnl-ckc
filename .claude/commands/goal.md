---
description: Fetch a clinical guideline, normalize it to ACE, compile it to Prolog
argument-hint: "[guideline id or source URL; empty = resume incomplete work]"
---

# /goal — guideline → ACE → Prolog

Persistence contract: repository state = the sole persistence; every invocation starts fresh from it and is idempotent. Toolchain: `export PATH="$PWD/.toolchain/bin:$PATH"` (SWI-Prolog 9.2.9 + python3; rebuild recipe in `.agent/memory.md`).

Work selection: `$ARGUMENTS` names a guideline id or source URL. Empty ⇒ resume any incomplete `guidelines/<id>/` (missing/stale `pl/`, failing check); all complete ⇒ ask the user for the next source. Id grammar: `[a-z0-9-]+`, no leading dash. A URL maps to a stable descriptive slug chosen once at fetch time; an already-fetched URL reuses its recorded id.

Round, per guideline:

1. **Fetch** — acquire the source document (authenticated web route per global `CLAUDE.md`; paywall/rights gate ⇒ stop + ask). Persist it immutably under `guidelines/<id>/source/` BEFORE any generative processing; record URL, retrieval date, SHA-256, byte length, rights quote + attribution in `guidelines/<id>/README.md` (model: `guidelines/cdc-2022-opioid/README.md`). Changed remote content ⇒ new versioned id, never mutation of a recorded source.
2. **Normalize** — extract the normative statements verbatim into `source/` evidence; author one ACE document per statement under `ace/` + the shared `lexicon.ulex` (model: the cdc-2022-opioid corpus; supported construct set = header of `vendor/ape/prolog/ace_to_pl.pl`). ACE = minimal faithful projection; state honest coverage in the README.
3. **Compile** — `python3 -P tools/goal.py compile <id>`. Rejection ⇒ adjust ACE/lexicon first; a genuinely new construct ⇒ extend the `ace_to_pl.pl` translation minimally, keeping totality (every sentence → exactly one clause; unrecognized shapes reject) and adding `tests/red/` probes for the new rejection boundary.
4. **Gate** — `python3 -P tools/goal.py check` green (one invocation is the full gate; it already compiles every document twice); `python3 -P tools/regen.py --check` green whenever E-- sources changed.
5. **Close** — one scoped commit per guideline (convention: project `CLAUDE.md`). Unresolvable rights/semantics ⇒ record the blocker in the guideline README, report, continue with other work.

Teammate fan-out (multiple guidelines, bulk extraction) follows `/session-prompt`'s execution map; compiler/lexicon edits stay MAIN-authored.

Termination: every `guidelines/<id>/` compiles and checks green and no new source is assigned ⇒ report done with the gate outputs.
