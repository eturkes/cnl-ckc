# M5.2a — engine/load exec-cost revision (contract)

Unit = the R30d spine escalation (`archive/contracts/m5u2.md`): the K2 kernel is
spec-exact but its exec cost makes product-scale consumption unusable. Spec
unchanged; impl + proofs revised; every M5.2 ruling R1–R30 binds.

## Tier

`kernel` — uninspected impl+proofs under the M5.2 spec; review = verifier +
gates + the differential/red batteries below. No new spec line: the perf
change lands entirely in `ckc-kernel` (MAIN never reads the impl).

## Baseline (HEAD f9e18145, release binary)

| lane | legacy (swipl) | rust HEAD |
| --- | --- | --- |
| `aggregate-check` 5-doc manifest | 0.20 s / 26 MB | >120 s timeout, 60 GB RSS |
| `aggregate-check` 1 doc (rec01-imp01, 2 obligations) | — | >60 s, 28 GB |
| `recursion-check` 5 / 20 / 60 / 337 docs | 1.1 s / 49 MB (337) | 0.07 s / 0.83 s 0.67 GB / 5.8 s 4.1 GB / >300 s timeout 56 GB |
| `answer` 4 committed queries (337-doc composition) | ~1.1 s / 47 MB each | OOM-killed (55.7 GB, R30d) |
| red suite 10 parked cases | <30 s each | >30 s (T-C605 >180 s) |

Two defects, both exec-only: (D1) search arena grows append-only across
backtracking — every retried clause head + every binding materializes new
nodes that are never reclaimed (single-document aggregate replay with 2
shallow obligations reaches 28 GB); (D2) load/scan accumulation is
superlinear in document count (recursion-check 20→60 docs = 7× time, 6× RSS;
a structural scan of 9053 rules should run in milliseconds).

## Predicates (acceptance)

- P1 verify: `just verify` green (`cargo verus verify --workspace --locked
  --offline -- --no-cheating` after the vstd warm + first-party clean).
- P2 trust: `just trust` green after `gen_trust.py` regen (kernel-only
  edits leave `spec-manifest.tsv` untouched — a manifest delta = a
  trusted-surface touch and needs a lead ruling).
- P3 parked cases: the 10 R30d `pending-ruling` cases (S4-R16-001,
  T-C602/603/605/606/607/611/613/625/919) flip `active` in
  `.scratch/m5u2/suite/cases/*/case.json` + `coverage.json` census and pass
  under the runner's 30 s cap (`RUST_BIN=… runner.py target`); the rest of
  the suite stays green (139 → 149 passes, 0 failures).
- P4 corpus differential: `k2_corpus_diff.py --lanes answer,recursion,
  aggregate` = 3 lanes green (byte parity with legacy over the 337-doc
  composition, 4 committed queries, 974 obligations). Time bound per
  invocation ≤ 30 s, peak RSS ≤ 2 GB (measured with `/usr/bin/time -v`;
  legacy = 4.3 s / 82 MB for aggregate-337).
- P5 K1 lanes A/H (`run_diff.py --lanes A,H`) unchanged: 349/349.
- P6 legacy chain untouched: no file outside `rust/ckc-kernel/` changes
  (trust regen excepted) → `just legacy` unaffected; `git diff --quiet --
  guidelines/`.
- P7 `just rust` green at the closing commit (fmt-check incl.).

## Design constraints (impl freedom within)

- Semantics fixed by `ckc-spec/src/engine.rs` (`run`/`step`/`call`/`unify`)
  + R15 cost model; no spec edit; no new escape sites (`assumes=34` stays).
- Arena reclamation must respect R25's append-only *view* law only where
  proofs need it: choicepoints (`Alt`) hold `fresh`; on backtrack the
  arena truncates to the resumed `fresh` mark (alts hold only roots below
  their own `fresh`); recorded solution rows persist outside the truncating
  region (copy-out on record). Any scheme meeting P1–P7 is acceptable — the
  spec is the contract, not this sketch.
- Load: parse each member once; the document database is a flat clause
  vector; indicator-major rule census = one pass; no per-document re-walk of
  the whole composition.

## Gate identities

verify/trust/fmt/clippy = `just rust` recipes; suite = `RUST_BIN=rust/target/
release/ckc python3 -P .scratch/m5u2/suite/runner.py target`; corpus =
`python3 -P .scratch/m5u2/diff/k2_corpus_diff.py --rust-bin rust/target/
release/ckc`; K1 lanes = `python3 -P .scratch/m5u2/diff/run_diff.py --rust-bin
rust/target/release/ckc --lanes A,H` (all with `.toolchain/bin` first on PATH).

## Rulings

- R31 perf measurement law: acceptance numbers are wall + peak RSS under
  `/usr/bin/time -v` on the dev workstation (8 cores, 62 GB); a factor ≤ 10×
  legacy wall counts as parity for a verified interpreter; RSS bound = 2 GB
  absolute (product composition ≤ 350 documents by construction).
