# UP2 — every dependency + toolchain pin at its latest release before the cut (contract)

Request (user, MAINTAIN; "applicable for the planned pre-release and full release"): upgrade every dependency + toolchain pin to its latest release; current vs latest per entry; major bumps → breaking changes vs our call sites; a held pin → `.agent/deferred.md` row naming its blocker.

## Census (base 3b21bcb4)

`just outdated` (`.scratch/deps2/outdated.log`): rc 0, `36 pins, 0 drift, 3 held, 0 errors`. MAIN's checks beyond it:

| pin | current | latest | status |
|---|---|---|---|
| Verus (stable) | release/0.2026.10.26.d17fbd0 | same (rolling 0.2026.10.27 = prerelease) | ok |
| rustc (follows Verus) | 1.99.1 | Rust stable 1.99.1 (channel 2026-10-15) | ok |
| Z3 (bundled) | 4.16.0 | z3-5.3.2 | held: Verus row (latest refreshed 5.3.1 → 5.3.2) |
| vstd + verus_* | 0.0.0-2026-10-25-0158 family | `cargo update --dry-run` both workspaces: 0 packages | ok |
| syn / synstructure / convert_case | 2.0.119 / 0.13.2 / 0.4.0 | 3.0.9 / 0.14.0 / 0.12.0 | held: Verus row |
| Kani | 0.68.0 | 0.68.0 | ok |
| Kissat (bundled) / CBMC | 4.0.1 / 6.11.0 | rel-4.0.4 / cbmc-6.11.0 | Kissat held: Kani row |
| Kani installer crates | cfg-if 1.0.4, libc 0.2.189, objc2 0.6.4, windows-link 0.2.1 | 1.0.5, 0.2.191, 0.6.5, 0.100.0 | held: Kani row |
| SWI-Prolog source / CI image | 10.0.2 / 10.0.2@sha256:83f30850… | same (Docker Hub digest equal) | ok |
| tools.lock (rustup-init, just, verusfmt, cargo-deny, gitleaks, zizmor, cargo-mutants, node, chrome-headless-shell) | 1.29.2, 1.59.0, 0.7.4, 0.20.2, 8.30.1, 1.31.1, 27.1.0, 26.12.1, 156.0.8078.47 | same | ok |
| Cargo.lock crates (both workspaces, 30 non-Verus) | — | crates.io max stable | ok |
| actions checkout / cache / upload-artifact / download-artifact | v7.0.1 / v6.1.0 / v7.0.2 / v8.0.2 (SHA-pinned) | same tag + commit | ok |
| runner | ubuntu-26.04 | newest runner-images label ubuntu-26.04 | ok |
| APE / Clex fork base | 5f4d5354 / 20960a5c | upstream HEAD same | ok |
| Python (`.python-version`) | 3.15 | 3.15 (latest 3.15.2) | ok; local interpreter 3.15.1 → 3.15.2 |

No version moved in a tracked file ⇒ no breaking-change review applies.

## Rulings

- R1 (user, AskUserQuestion): "Cut pins serve both" — the release line keeps the cut's pins through the release; newer releases land on main; a RustSec advisory reds the line's `just deny` → fixed on the line (spec Decisions `Publication scope`).

## Tier

data: the edits = deferred-row text + one Decisions clause, re-checked by `ckc check` agent law + `.agent/spec-lint`; no closing-diff reviewer (text-only diff, census reproducible by `just outdated`).

## Predicates

- P1 `.agent/deferred.md`: Verus-family row names z3-5.3.2; both bundled-component rows cite contracts `up` + `up2`; every row keeps ` Accept: `.
- P2 local Python 3.15.2 (uv 0.13.6 download; system uv 0.13.5 lists 3.15.1 alone): `.agent/archive/generators/replay` 6/6 identical; `verus-issues.py` output byte-identical to 3.15.1.
- P3 gate: CI gate `just ci up2` green on the closing commit; each runnable `Artifacts` entry driven by its run command.
