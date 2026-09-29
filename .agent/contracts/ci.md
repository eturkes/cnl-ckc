# U1 ci — red CI surfaces repaired + workflow scanner + pin-drift automation (contract)

Defects on main (GitHub Actions history, f3910477):
- D1 `certify` job red since it landed: the `rust` job's glibc-2.39 binary (ubuntu-24.04) cannot load in the swipl:9.2.9 container (Debian bookworm, glibc 2.36): `GLIBC_2.39' not found`. The M5.7 `check` job inherits the same seam.
- D2 weekly `kani` red: `rust/kani/bootstrap.sh` runs `cargo-kani --version` before the pinned local-bundle setup with `KANI_HOME` unset → on a machine without `~/.kani` the wrapper auto-installs an unpinned bundle from the network + prints setup chatter → `cargo-kani version mismatch`. Local green rests on a leftover `~/.kani/kani-0.67.0`.
- D3 weekly Dependabot `cargo` red: `rust/.cargo/config.toml` replaces crates-io with `rust/vendor`, which Dependabot never fetches → `failed to read root of directory source` (Dependabot's cargo updater cannot resolve a vendored source replacement).
- Gap: `.github/workflows/ci.yml` + `.github/dependabot.yml` carry no static analysis.

## Tier

shell/CI — no kernel, no spec. Gate identity = `just rust` + `just certify`; CI jobs reproduced locally inside the pinned swipl container (podman, digest-pinned image).

## Predicates (acceptance)

- P1 static artifact: `just build-static` builds `rust/target/x86_64-unknown-linux-musl/release/ckc` (statically linked; `just tools` adds the musl target to the pinned toolchain); CI `rust` job uploads it, container jobs run it; `readelf -l` shows no `INTERP` + `readelf -d` no `NEEDED`. Red: the glibc build fails inside `swipl:9.2.9@sha256:3e4b85b1…` with the CI error. Green: the static build runs `ckc certify --cases tests/certify/cases.tsv` + `ckc certify cdc-2022-opioid` inside that container, rc 0.
- P2 Kani cold start: zero `cargo-kani` executions before the pinned setup (both sites: the warm-cache probe + the post-install check); wrapper identity read from `$WRAPPER/.crates.toml`; `KANI_HOME` + isolated `CARGO_HOME`/`RUSTUP_HOME` exported before first use. Red: old script, `HOME` = empty dir + fresh kani prefix → `cargo-kani version mismatch`. Green: fixed script, same setup → `kani: 0.67.0 ok`; `just kani` green with an empty `HOME`.
- P3 workflow scanner: zizmor (sha256-pinned release in `rust/tools.lock`) = `just workflows` step in `just rust` (offline audits; online audits when `GH_TOKEN` is set, weekly job). Red: the pre-fix `.github/` → findings rc≠0; planted `${{ github.event.pull_request.title }}` in a `run:` block → template-injection finding. Green: fixed `.github/` rc 0, no inline ignores without a reason.
- P4 update automation: Dependabot = github-actions only (+ cooldown); cargo entry removed (D3). New `just outdated` (bash + curl + jq; CI weekly `updates` job, `GITHUB_TOKEN` for API rate) compares every pin — `rust/Cargo.lock` crates (crates.io newest stable; vstd follows verus.lock), `rust/tools.lock` tools + rustup, `rust/verus.lock` release, `rust/kani.lock` Kani — against upstream latest; prints `pin current latest`; rc 1 on any drift; rc 2 on any failed or empty upstream answer (never green). Verifier family = one unit: `verus.lock` release vs latest Verus release; `vstd`/`verus_builtin*`/`verus_state_machines_macros` rows follow it (reported, not compared to crates.io); `rust/ckc-kani-harness/Cargo.lock` in scope. Red + green: planted stale `tools.lock` copy → rc 1 naming that pin; a pin set equal to fetched latest → rc 0 (`CKC_OUTDATED_ROOT` fixture root).
- P4b pin binding: `just tools` reinstalls a tool whose recorded pin (`$T/<tool>/.pin` = lock row sha256) differs from `tools.lock` / `verus.lock`; red: warm install + edited lock row → old code skips; green: reinstall.
- P5 docs: `.claude/rules/rust.md` (CI jobs, bump procedure) + `ops.md` current; `just rust` green; `git diff --stat` = justfile, ci.yml, dependabot.yml, tools.lock, kani/bootstrap.sh, rules.

## Rulings

- R-ci1 musl over static glibc (NSS/`getaddrinfo` caveats) and over building inside the container (no linker there); local `just gate` keeps the glibc build — both builds share one source; the container jobs grade the static artifact end to end.
- R-ci2 user ruling: drift job over Dependabot cargo PRs; swipl container digest = deliberate reference-runtime hold, excluded from drift.
- R-ci3 CI cannot run from this session (remote = user); evidence = local container reproduction + `just` recipes; GitHub run = first push.
