# QLF — precompiled Clex lexicon in the production APE stages (contract)

Row: Tasks QLF (scale review item 4). Accept: each production stage qcompiles Clex once, and every per-document SWI-Prolog process loads the QLF; products byte-identical; per-process + gate times recorded.

## Tier

shell (staging of the pinned SWI-Prolog + fork; an equal-strength gate speedup: no check leaves the gate, each still fires on its planted input); shallow ⇒ MAIN alone + self-review; advisor on the files before commit.

## Design

- D1 One stage builder for every production stage: `check::process::stage_ape(root, dir, swipl)` copies `vendor/ape`, copies `vendor/clex/clex_lexicon.pl` over the fork's placeholder, builds the parser (`[fit_to_plp]` in `prolog/parser`, as today), then runs `style_check(-discontiguous), qcompile(clex_lexicon, [encoding(utf8)])` in `prolog/lexicon` (the options `clex.pl` loads with) in its own `swipl -q -f none -F none` run (no `-O`: the per-document processes load without it). Users: `check::process::stage` (`ckc check`, `ckc compile`, `ckc queries`) + `certify_cli::Stage::new` (`ckc certify`), whose duplicate copy + build code goes.
- D2 The builder fails the stage (`ape-stage`, the SWI output relayed) unless the lexicon run exits 0 with empty stdout + stderr and leaves `clex_lexicon.qlf`, as it requires `grammar.plp` after the parser run. Reason: from source, a malformed lexicon reaches stderr in every per-document process (`compiler-stderr`); from a QLF, only the qcompile run reports it, so that run must carry the refusal.
- D3 Per-document processes keep loading `clex_lexicon` without an extension (`clex.pl`: `load_files(ClexFile, [encoding(utf8)])`) ⇒ SWI-Prolog loads the up-to-date `.qlf` beside the staged source (manual § Quick load files: "When no extension is specified, it will load the `.qlf` file when present and up-to-date, and the `.pl` file otherwise"); no fork edit.
- D4 The QLF lives inside each run's stage alone, written by the run's own `swipl` (10.0.2 guard) ⇒ no QLF crosses SWI-Prolog versions or outlives its run.

## Evidence (orientation probe, `.scratch/imp/qlf/probe`)

Pinned SWI-Prolog 10.0.2, staged APE + Clex: loading `clex` from source 1.24 s wall, from the QLF 0.10 s; both = 17,420 `noun_sg/3` clauses, 81 non-ASCII nouns (`'aide-mémoire'` read back), the 97,527 clauses + `dynamic` on every lexical predicate; qcompile with the D1 options prints nothing; `verbose_load(full)` reports `clex_lexicon loaded into clex` (QLF) vs `clex_lexicon compiled into clex` (source); no fork code asserts into `clex`.

## Predicates

- P1 `stage_ape` over the repository → a process loading the staged `clex` with `verbose_load(full)` reports `clex_lexicon loaded into clex` and never `compiled into clex` (shell unit test; red at d3eca5fb, whose stage holds no QLF).
- P2 `stage_ape` over a root whose `clex_lexicon.pl` holds a syntax error → `Err` naming `ape-stage` with SWI-Prolog's message (shell unit test; red at d3eca5fb: the stage builds).
- P3 Products byte-identical: `just gate` green = fresh compile + queries byte-stable against the committed corpus, `ckc check` (every per-document compile, proof + question, red + green probe), `ckc certify` per guideline + its cases; no gate step or case removed.
- P4 Times recorded in the commit body: the probe's per-process load; the CI jobs of this unit's run vs run `ci/orient` (same workflow; d3eca5fb).
- P5 `vendor/` unchanged.
