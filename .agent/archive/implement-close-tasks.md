# IMPLEMENT close — task checklist (archived at phase close)

- [x] c1069299 U1 ci — `archive/contracts/ci.md`: static musl `ckc` for the container jobs (`certify` red since M6), Kani cold bootstrap, zizmor `just workflows`, weekly `just outdated` drift job in place of Dependabot cargo.
- [x] 86acf59c U1b fmt — `fmt`/`fmt-check` glob `ckc/src/*.rs` only → 37 shell modules under `ckc/src/{check,ui}/` outside the format gate (2 unformatted: `check/documents.rs`, `check/mod.rs`); red = planted misformat there passes the old gate.
- [x] c8cbe43b U2 cas — `archive/contracts/m5u5-cas.md`: identity-validated ledger lock + std-only regression seen red (review C-01..C-04).
- [x] f697dee4 U3 evidence — pre-cutover binary fce50147 + legacy tools: K1 A/H 349/349, K2 0 divergences, K3 D/E 4/4, queries replay 60/0, M5.2 suite 224/25 pending, K3 matrix 114/114, M5.1 94/94, trust battery 16/16, perf 17 rows 0 failures, M5.3 R98 parity 12/12, M5.3 mutants legacy 46/46 + native 38/42 (4 late rows unbound on pre-R80 trees → deferred row).
- [x] 39968a58 U4 harness — `archive/contracts/harness.md` (6e5f5eeb H1+H2, 907e41dd, 257bd537 H3, f9f30002 H4, fba6faa9 H5, 82f37216 H6, c541dfea H7, 39968a58 generators + rules): `ckc trust-audit --write`, trust battery, align + v1 suites, align probes, bounded check mutants → committed; each seen red.
- [x] 24177e16 6ceb2d8d 07f76e88 U5a M5.7 cutover — `.agent/contracts/m5u7.md` P1–P7 (R99–R104): tag `legacy` = 39968a58, R99 re-pin, fork shrink, deletions, CI `check` swap, NOTICE, README/REFERENCE/rules scrub; red only on release-manifest freshness until U5b.
- [x] f697dee4 U5b release-manifest regen from HEAD = U5a → gate steps green on its tree (test step rerun after a one-binary flake).
- [x] 4469d143 U6 M5 review (fixes 8863529f v1 FIFO gate, 4a9ccce9 spec minimality, 24e2407c docs; ledger: every row adjudicated except R-13 perf + S-10 close state) — `.agent/review.md` R-01..R-14 + S-01..S-10 + C-01..C-04; one `reviewer` per lens (security-vocabulary rows = MAIN); fixes land before close.
- [x] (closing commit) U7 close — superseded branches deleted after proof (user ruling), Decisions current, deferred rows, archive contracts, `Phase: MAINTAIN`, `just gate` green on the clean closing tree + container `check`/`certify` on the same static artifact.
- Deferral queue = `.agent/deferred.md` (off-path improvements + ruled deferrals; one line + acceptance check each).


Review ledger = `.agent/review.md` (every row adjudicated); IMPLEMENT-close run record = the closing commit's body.
