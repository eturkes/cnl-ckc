# U4 harness — scratch validators → committed checks (contract)

Deferred row "scratch validators → committed Rust harness ≤ M5.7" (user ruling: port everything feasible). A scratch checker whose oracle survives the cutover moves under `tests/` + `rust/ckc/tests/`; a checker whose oracle IS the legacy tree retires after its last replay (U3), recorded in the M5.7 body; generators of committed kernel files are archived as regeneration recipes.

## Tier

tests/shell — pinned-output regression suites + one shell subcommand (`ckc trust-audit --write`). Pins = the scratch suites' target-side expectations, cross-checked against legacy when recorded; the port converts them, never re-derives them from new code.

## Disposition inventory (every live scratch checker)

| scratch | disposition | target |
| --- | --- | --- |
| `m5u1/gen_trust.py` | port (H1) | `ckc trust-audit --write [root]` |
| `m5u1/trust-battery/run.py` (16 plants) | port (H2) | `rust/ckc/tests/trust_battery.rs` |
| `m5u1/suite` (94 align-check cases) | port (H3) | `tests/align/` + `rust/ckc/tests/align_suite.rs` |
| `m5u2/suite` (224 active primary) + `m5u2b/test/matrix.py` (114 active supplemental K3 probes) | port (H4) | `tests/v1/` + `rust/ckc/tests/v1_suite.rs` |
| `m5u4/parity.py` (manifest/ledger commands; legacy pins), `m5u4/align_probes` | port native rows (H5) | `tests/align-probes/` + runner, rows needing no legacy oracle |
| `m5u3/mutants.py` (46 corpus mutants) | port bounded subset (H6) | cases reaching the first violation before the swipl stage, battery ≤ 3 min |
| `m5u5/shell/socket_smoke.py` native assertions (GET, POST 303/409, locked-CAS 409, cleanup) | port (H7) | `rust/ckc/tests/ui_socket.rs` |
| `m5u5/shell/resource_copy_probe.py` | covered | `ui_fixtures.rs` copy rows (verified at port time; else port) |
| `m6/mutants.py` | covered | `tests/certify/cases.tsv` (17) via `just certify` |
| `m5u2/diff/{run_diff,k2_corpus_diff}.py`, `m5u2b/queries_replay.py`, `m5u1/diff/run_diff.py`, `m5u3/parity.py`, `m5u4/align_diff.py`, `m5u5/test/parity.py`, `m5u5/shell/s6_hex_replay.py`, `m5u6/{dist_diff,parity,legacy_ckc}.py`, `m5u2a/prod_accept.py` parity half | retire (legacy oracle) | last replay = U3 logs |
| `m5u2a/prod_accept.py` perf half | review evidence | R-13 remeasured on the closing binary |
| `m5u5/sound/gen_*.py`, `m5u5/copy/*.py`, `m6/prod/gen_symbols.py` (generators of committed kernel files) | archive | `.agent/archive/generators/` + invocation + output sha256 |
| `m5u1/rev*`, `m5u2/logs/*`, `m5u2/map1`, `m5u1/map` | retire (one-shot audits, evidence in archived contracts) | — |

## Storage + execution law

- Inert storage: fixture files carry a `.in` suffix (a tracked `.pl` outside the prolog inventory reds `ckc check` + legacy `goal.py check`); `tests/{v1,align,align-probes}/** -text` in `.gitattributes` (exact bytes: CR, invalid UTF-8).
- Virtual root: each runner materializes a case into a private temp root at the recording layout (`.scratch/m5u2/suite/cases/<id>/…` for H4) with the `.in` suffix dropped, cwd = that root ⇒ argv, embedded manifests and path-bearing diagnostics keep their recorded bytes.
- Generated inputs > 100 KB (T-C602, T-C625, T-C825 variants) = deterministic in-test generators; output sha256 = the scratch file's sha256.

## Predicates (acceptance)

- P1 H1 = explicit rebaseline (gen_trust.py semantics: clear + adopt scanner rows + re-pin manifest), then the read-only audit; never inside a gate recipe. Differential: on the committed tree + a planted-drift copy, `--write` output bytes = `gen_trust.py` output bytes (`cmp`, three files).
- P2 counts: H3 94, H4 224 + 114, H2 16 plants + 1 clean control; every pin byte-equal to its scratch source (cross-check script digest in the commit body).
- P3 each battery seen red: one flipped expected byte → exactly that case fails; H2 additionally red against an audit stub that accepts a plant.
- P4 `cargo test --workspace` wall growth ≤ 5 min total; no new dependency; `just rust` + `ckc check` inventory sections green.
- P5 `.claude/rules/rust.md`: trust regen = `ckc trust-audit --write`; harness list = committed suites; retired scratch harnesses named once with reason.

## Rulings

- R-h1 conversion + archive scripts = committed record under `.agent/archive/migrations/` (base revision, input digests, invocation, output digest); rerun from the same base = byte-identical output.
- R-h2 scratch runners stay usable until U5 deletes the legacy side; ported suites run target-side only.
