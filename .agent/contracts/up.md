# UP — every dependency + toolchain pin at its latest release for the first release (contract)

Request (user, MAINTAIN): upgrade every dependency + toolchain pin to its latest release; major bumps → changelog breaking changes vs our call sites; a held pin → `.agent/deferred.md` row naming its blocker. Census (`.scratch/deps/outdated.log` + MAIN's upstream checks): Verus 0.2026.10.18, Kani 0.68.0, every `tools.lock` row, the vstd family, `actions/checkout` v7.0.1 + `actions/cache` v6.1.0, the APE + Clex fork bases (= upstream HEAD) = latest; drift = `actions/upload-artifact` v7.0.1 → v7.0.2, `actions/download-artifact` v8.0.1 → v8.0.2, runner `ubuntu-24.04` → `ubuntu-26.04` (GA, runner-images #14747), SWI-Prolog 9.2.9 → 10.0.2 (major), Node 24.22.1 → 26.12.1 (major; Current line), Python 3.13 → 3.15; held = syn 3.0.9, synstructure 0.14.0, convert_case 0.12.0 (`verus_builtin_macros` requires syn ^2.0, synstructure ^0.13.2, convert_case ^0.4).

## Rulings

- R1 (user, AskUserQuestion): SWI-Prolog 10.0.2 = the reference runtime, and `just outdated` tracks SWI drift — lifts contract `ci` R-ci2 (9.2.9 hold, container excluded from drift).
- R2 (user, AskUserQuestion): Node 26.12.1 now; `outdated` reads node's newest release (replaces the newest-LTS rule).
- R3 (MAIN): `brace_terms(true)` stays out of the writers — no `{}`/1 term reaches the fork's writers or `drs_dump.pl` (APE DRS + emission vocabulary; 0 pinned stderr bytes with a brace), so no gate could observe the option.

## Tier

data: every surface a unit touches is re-checked by the gate for free — emitted bytes by the byte-stable recompile + `ckc certify`, the kernel literal by Verus, the driver by every red + green probe replay, `outdated` by `recipes.rs`. One closing-diff `reviewer` (every lens).

## Predicates

UP1 actions + runner
- P1 `ci.yml`: upload-artifact @cf430e03 (v7.0.2), download-artifact @9000827c (v8.0.2), each `runs-on: ubuntu-26.04`.
- P2 the SWI-Prolog cache key carries the runner OS release (`/etc/os-release` ID + VERSION_ID): a swipl built on one image never restores on another (`runner.os` = `Linux` on both).
- P3 the apt build dependencies resolve on `ubuntu:26.04` (container `apt-cache policy` candidates).
- P4 `just workflows` green; the CI gate run green (the `rust` job on 26.04 incl. `just swipl` + `just mobile`).

UP2 SWI-Prolog 10.0.2 (R1)
- P1 pins: `just swipl` = the 10.0.2 source archive (sha256 e42cc098… = swi-prolog.org's published `.sha256`), cmake `SWIPL_PACKAGES_GUI=OFF` (10.0 renamed set `X` → `GUI`); CI container = `docker.io/library/swipl:10.0.2@sha256:83f30850…` (Docker Hub tag digest; Debian bookworm, glibc 2.36); the identity gate accepts exactly 10.0.2.
- P2 driver: `ace_to_pl.pl` `main`'s recovery lets `unwind(_)` pass (halt/1 raises `unwind(halt(S))` from 9.3.13; catch/3 rethrows it after the recovery goal) and keeps class `uncaught` rc 2 for every other exception. Red: HEAD's driver under 10.0.2, `ckc compile cdc-2022-opioid` → `ace_to_pl_error(uncaught,unwind(halt(0))).` rc 2. Green: fixed driver → `ckc: compile ok 382 documents`, guidelines/ byte-identical; a genuine uncaught exception (stdout write error) → `ace_to_pl_error(uncaught,…)` rc 2.
- P3 byte stability under 10.0.2: `ckc compile` + `ckc queries` leave guidelines/ unchanged; `ckc check`, `ckc ui check`, `ckc certify` per guideline + `--cases`, `cargo test` (SWI-backed suites incl. `consumer_recipe`) green with every expected byte unchanged except the version pins.
- P4 release record: `meta swipl 10.0.2` in ckc-spec `meta_block` + kernel `release_rows::meta` (Verus re-verifies), `release-manifest.tsv`, the `dist_cases` pin.
- P5 writer deltas recorded, none reaching a committed byte: under `ignore_ops(true)` a `{}`/1 term prints `{}(a)` (swipl-devel e8783f23; R3); floats print shortest with a two-digit exponent (`10000.0` → `1.0e+04`, `1.0e-5` → `1.0e-05`) ⇒ REFERENCE's literal rule (a decimal APE writes with an exponent rejects) covers more spellings, fail-closed; every corpus + fixture float literal prints identically (probe); `variant_sha1/2` differs on compounds (the consumer driver compares within one runtime).
- P6 drift (R1): `just outdated` reports `swipl` (the recipe's version vs swi-prolog.org's latest stable) + `swipl-image` (the ci.yml container digest vs Docker Hub's digest of that latest version's tag); `recipes.rs`: a newer upstream → drift rc 1, equal → ok, a failed answer → rc 2, each with its planted pre-rule form slipping through.
- P7 text: README, REFERENCE, ops.md, rust.md, CLAUDE.local.md, the `probe-pins` docstring + the v1text escape-pin comment (re-probed: ASCII 0–127 escapes identical) name 10.0.2.

UP3 Node 26.12.1 (R2)
- P1 `tools.lock` node row = 26.12.1 (`just bump-tool`, nodejs.org SHASUMS256.txt).
- P2 `outdated` reads index.json's newest release; `recipes.rs` flips its case: a newer non-LTS release reports drift.
- P3 Node 25 + 26 semver-major changes vs `tools/*.mjs` call sites → adapted or ruled no-effect.
- P4 `just mobile` green under 26.12.1, red inputs firing.

UP4 Python 3.15
- P1 `.python-version` + the `swipl` recipe's shim = 3.15.
- P2 `.agent/archive/generators/replay` 6/6 identical + `verus-issues.py` output byte-identical, 3.15.1 vs 3.13.17.

UP5 held crates
- P1 one `.agent/deferred.md` row naming each held crate + `verus_builtin_macros`' requirement on it (one blocker, one acceptance check); Accept = a Verus bump whose macros admit the newer version.

## Gate identity

Closing commit: full `just gate` (pool form, `CLAUDE.local.md`) + the CI gate `just ci up`; `just outdated` rc 0 apart from the UP5 holds; each runnable `Artifacts` entry driven by its run command.
