# M5 plan of record — Rust rewrite + E-- retirement

User-directed milestone (explicit direction; reopens development scope for
exactly this work). Operative stub = roadmap § M5; standing law =
`.agent/standing-instructions.md` (amended this milestone). This file =
design record: seam map, verification approach, unit contracts, fixture
consequences, certification story. Read at unit start.

## Mandate

Rewrite every first-party non-Prolog artifact in Rust; retire E-- and its
generated Python permanently (git history = sole record); confine
hand-authored Prolog to the APE fork's ACE emission closure. The Rust is
never human-reviewed: correctness rests on the verified-kernel pattern —
a small human-read formal specification (trusted), an AI-written
implementation and AI-written machine-checked proofs (both uninspected),
a pinned deterministic checker, and an audit gate against smuggled
axioms/escape hatches. Keep the codebase focused, minimal, correct.

## Prolog surface classification (split ruling)

| Surface | Class | Disposition |
| --- | --- | --- |
| `vendor/ape` upstream parser + grammar | upstream TCB | stays (emission path) |
| `ace_to_pl.pl` compile mode (doc → v1 clauses) | fork emission | stays Prolog |
| `ace_to_pl.pl` proof mode (obligation payload emission) | fork emission (DRS-derived) | stays Prolog |
| `ace_to_pl.pl` question mode (ACE question → query pl) | fork emission | stays Prolog |
| `ace_to_pl.pl` in-compile obligation replay | KB-semantics execution | → Rust engine at M5.8 (single semantics owner) |
| `ace_to_pl.pl` check mode (SWI load check) | consumption | → Rust verified reader; optional unverified swipl load smoke kept for engine-portability evidence |
| `ace_to_pl.pl` aggregate-check / recursion-check | consumption | → Rust engine |
| `ace_to_pl.pl` answer / trace modes | consumption | → Rust engine |
| `vendor/clex` lexicon facts | upstream data | stays |
| `guidelines/*/lexicon.ulex` | APE input data (hand-authored vocabulary) | stays; existing liveness/minimality/shadow gates move to Rust |
| `guidelines/*/pl`, `guidelines/*/queries/pl` | APE-emitted knowledge | unchanged bytes |
| `guidelines/*/queries/{answers,traces}` | engine-derived demonstrations | derived by verified Rust engine post-M5.2; carry no knowledge |
| `tests/red/*.ace` + `.expect` | rejection fixtures | data; harness → Rust |

Ruling: "Prolog only through APE" scopes to knowledge Prolog (KB clauses +
query projections, all from clinician-reviewable ACE). Answer/trace
artifacts stay Prolog-term syntax for engine portability but are
machine-derived demonstrations emitted by the verified engine — the
charter's demonstration clause names them. Considered and rejected:
re-formatting answers/traces as non-Prolog (loses loadability, churns two
REFERENCE sections, adds no soundness).

Consequence of the M5.8 fork shrink: a document whose obligations fail
stops rejecting inside the compile process and rejects at the Rust replay
stage of the same pipeline step; proof-class red probes re-pin to that
stage. Emitted artifact bytes are unaffected (replay never shaped
emission). The fork's PROVENANCE trust posture updates to "DRS
interpretation and emission live in Prolog; all KB consumption lives in
the verified kernel".

## Target architecture

`rust/` cargo workspace, three crates:

- `ckc-spec` — the trusted surface. Pure Verus spec: format grammars,
  writer/reader function specs, v1 operational semantics, gate
  soundness statements (`check(x) == ok <==> WellFormed(x)`), UI render
  contract, escaping/CSP invariants, dist derivation function. Everything
  a certifying human reads lives here (plus the shell file list and dep
  allowlist). Budget: report-only meter, aim ≤ ~1.5K lines; review
  adjudicates size + readability.
- `ckc-kernel` — verified impl + proofs. Uninspected by humans; `verus`
  green = its whole review. Pure (no ambient I/O); operates on bytes/
  values the shell hands it.
- `ckc` — the bin (subcommands mirroring today's CLI: check, compile,
  queries, align, review-manifest, release-manifest, ledger-validate,
  derive-review-manifest, ui serve/render/check/request, dist build,
  trust-audit). Thin unverified shell: process spawn (swipl, git), fs
  (read/write/atomic-rename/fsync), sockets, clock, argv. Every shell fn
  = thin + carries its spec-level assumption; fixture harnesses cover
  shell behavior. Post-cutover invocation = `cargo build --release` →
  `rust/target/release/ckc <cmd>`; `tools/` deletes.

Dependency allowlist (trusted, enumerated, axiomatized where specced):
rustc/std via the Verus-pinned toolchain, `vstd`, `libcrux-sha2 =0.0.8`
(HACL*/F*-verified algorithm core compiled to safe Rust; thin wrapper
allowlisted as trusted; axiomatized as a deterministic pure function —
custody claims never rest on collision resistance), one DEFLATE impl
for dist gzip (`flate2`/`miniz_oxide`; determinism = pinned version;
archive bytes re-baseline once at adoption). Nothing else; vendored +
`--locked --offline`. Kernel bans build scripts, proc macros, `unsafe`,
and feature-dependent exec semantics. Content bytes stay opaque
`Seq<u8>` inside the kernel; UTF-8 decoding sits at the shell/UI
boundary (std `from_utf8` = one allowlisted axiom). The trust-audit
gate diffs `Cargo.lock` against this list.

## Verification approach

Election: **Verus** (verus-lang), per `.scratch/agents/rverif-1.md`
survey. Rationale: verifies the shipped Rust in place (no
model-extraction gap); compile-time checking (`cargo verus verify`);
SMT automation keeps AI proof effort tractable for string/format logic;
weekly pinned releases bundling verifier+vstd+solver; `cargo-verus` +
lockfile give `--locked --offline` CI; MIT; production precedents
(verified mimalloc/IronKV ~5:1 proof:code, VeriSMo, Anvil, verified
storage) and the strongest LLM-proof-synthesis ecosystem (AutoVerus
lineage — Verus's own LLM guide mandates an external "cheat checker",
exactly our trust-audit gate).

Secondary gate: **Kani 0.67.0** bounded model checking over kernel
harnesses (symbolic small inputs, explicit domain + unwind bounds) for
bit-precise panic/overflow/UB pressure — after Verus proofs, never as
their substitute; introduced at M5.2 where it bites hardest.

Claim discipline: the certified claim is "Verus-verified under a pinned
TCB" (verifier + rustc + Z3 + vstd trusted as one hash-pinned upstream)
— not a foundational small-kernel proof; Aeneas→Lean would buy the
smaller checker kernel at the cost of a trusted Rust→Lean translation +
slower AI proof iteration, and was rejected on that trade.

Fallback triggers (assessed at M5.1 spike, re-assessed per unit): a
Verus blocker in supported Rust/vstd, spike proof:impl ratio > ~5×, or
persistent solver brittleness after conversion to byte-oriented code +
explicit lemmas → re-elect Creusot v0.13.0 (accepting the larger
Why3/portfolio TCB + weaker AI-proof ecosystem), or narrow the verified
kernel + widen the enumerated shell (honest-scope reduction, recorded
in `ckc-spec`). The M5.1 spike therefore probes the HARDEST shapes, not
just the spine slice: a throwaway miniature term round-trip proof + a
micro fuel-bounded NAF interpreter proof ride beside the align
validator to exercise the triggers before M5.2 commits.

Toolchain pinning: one immutable weekly Verus release asset, SHA-256
recorded in-repo, launcher-pinned rustup toolchain, committed
`Cargo.lock`, `cargo verus verify --workspace --locked --offline`;
`--rlimit` pinned; a timeout/rlimit change = a proof change, never a
retry. CI = build + verify + trust-audit + `ckc check` beside the pinned
swipl 9.2.9 container steps (swipl remains a pipeline dependency for
emission).

Trust-audit gate (`ckc trust-audit`): scans the workspace for
`assume`, `admit`, `#[verifier::external_body]`, `#[verifier::external]`,
`assume_specification`/`external_fn_specification`,
`exec_spec_unverified!`, `unsafe`, build scripts/proc macros in kernel
crates; every hit must match the committed site-exact allowlist; asserts
`[package.metadata.verus] verify=true` on every kernel crate + verifies
the spec-surface hash manifest (trusted files byte-pinned — drift fails
until a human re-approves the manifest) + diffs deps vs allowlist;
meter `ckc: trust spec=<lines> shell=<files> assumes=<sites>
deps=<ok|drift>`. Red-tested with planted-escape fixtures. CI runs full
`cargo verus verify --workspace --locked --offline` (never `focus`) in
a fresh pinned container. Certification claim = read `ckc-spec` + run
`cargo verus verify` + `ckc trust-audit`.

AI-proof workflow: agents write impl + proofs iterating against verifier
errors; proofs are never read by humans and never reviewed for style;
mutation probes at review measure SPEC strength (a mutant that still
verifies = spec gap — the review instrument for underspecification).

## Kernel partition

- K1 v1 artifact reader/writer: canonical clause-file grammar (documents,
  query projections, answers, traces), term model, round-trip theorems
  (parse∘print = id on model; print∘parse = id on canonical bytes),
  digest custody (clause-line sha256, semantic_clause digest, bundle
  digests).
- K2 semantics engine: bounded SLD, leftmost selection, source clause
  order, witness-precedes-rules, NAF, three bound regimes (replay
  4000/1e6; answer 100/1e5 cumulative/1e6 outer; trace MI 1000/1e5
  row/1e6 run), obligation replay laws (coverage from loaded sentence
  identities, uniqueness, variant_sequence, empty_obligation,
  payload_bytes), left-recursion scan (rename-apart unification), answer
  artifact derivation (standard order of terms spec'd), trace derivation.
- K3 trace checker: trace ⊨ composition + query + answers; clause_sha256
  joins exactly one committed line; `proved ⇒ derivable` soundness. The
  legacy Python trace scanner/parser/walker (~1.5K lines of goal.py)
  dies into this.
- K4 validators: coverage.tsv, projection-notes, adjudication +
  review-manifest, align.tsv, rights.tsv, compendium md+tsv, docid
  grammar, fork-notice policy (git bytes in via shell; policy logic
  verified), lexicon liveness/minimality/shadow/redundancy (ulex + clex
  + ACE texts), v1-vocabulary scan, red/queries/adjudication/copy
  fixture harnesses, release-manifest derivation.
- K5 UI: viewmodel, HTML render (escaping theorems, CSP hash, palette/
  script laws), HTTP request-line/header subset parsing, POST guard
  chain (token/Host/Origin/subject-digest/ledger CAS preconditions),
  ledger append bytes. Copy law upgrades: emitted copy becomes a const
  data registry in the kernel; the banned-token gate validates the
  registry, and a render theorem ties every visible text byte to it
  (replaces the Python `ast` walk). Double-render determinism becomes a
  theorem (render = pure function) instead of a runtime byte-compare.

Shell (unverified, enumerated, fixture-covered): subprocess, fs, sockets,
clock, argv/env, exit codes, git/`git archive` tar intake (member
validation logic verified in K4-adjacent code; tar bytes from shell).

## Byte-parity strategy + fixture consequences

Migration gate = dual-run differentials against the legacy tools on HEAD
+ full fixture replay; legacy gates stay CI-authoritative until M5.7.

- FC0 unchanged bytes (target): guidelines/** (all), committed query
  artifacts, tests/red, tests/adjudication, tests/queries, tests/copy,
  tests/ui trees + goldens + expects, meters/stderr formats (`goal:`
  meter prefixes may keep their bytes or re-pin to `ckc:` — decide once
  at M5.3, apply uniformly, regenerate expects mechanically; AM6 hazard:
  diff regen churn against contract before crediting green).
- FC1 harness rebind: every suite's runner moves E--/sh → `ckc check`
  internals; fixture data untouched.
- FC2 enumerated re-pins: fixtures naming legacy identities (argv shapes,
  `python3 -P` invocation strings, proof-class probe stage at M5.8);
  each divergence listed + ruled in the unit record.
- FC3 deletions: tests/strict (100 files, E-- language fixtures),
  regen machinery, `tests/dist/red.sh` shell harness (cases re-expressed
  in the Rust harness).
- FC4 re-baseline: dist archive bytes (DEFLATE impl change) →
  release-manifest regen at adoption; determinism double-build check
  retained.

## Unit contracts

- M5.1 toolchain + spike + trust gate. Election re-verified live (pinned
  install, hello-proof); `rust/` workspace skeleton; spike = align-TSV
  validator verified end-to-end (spec: row grammar + first-violation
  ordering + `accept ⟺ wellformed`) + two throwaway hardest-shape
  probes (miniature term round-trip, micro fuel-bounded NAF interpreter)
  scoring the fallback triggers; differential vs legacy on all 337
  live align files + the 11 parse-error selftest pins; trust-audit v0
  red-tested (≥6 planted escapes); CI job added (verify + trust-audit);
  serena `language_servers` += rust; `.gitignore` + settings deny +=
  `rust/target/`. Acceptance: both CI jobs green; spike differential 0
  divergences; gate kills all plants; fallback triggers evaluated +
  recorded.
- M5.2 v1 kernel (K1-K3). Evaluate Vest (secure-foundations verified
  parser/serializer combinators in Verus) as base-or-model for K1;
  introduce Kani harnesses (explicit domain + unwind bounds) over
  reader/engine. Acceptance: verus + kani green; differential — check
  mode over every committed pl, aggregate replay + recursion scan verdict
  parity on the live composition, answers + traces byte-identical for
  every committed query, tests/queries replay green on unchanged
  fixtures; adversarial mutations of clause files classified identically;
  0 unruled divergences.
- M5.3 check validators (K4). Acceptance: `ckc check` (minus ui/dist/
  strict) section-for-section parity with `goal.py check` on HEAD;
  tests/red + tests/adjudication + tests/copy replay green; copy-register
  successor design (const registry) landed; FC2 list ruled.
- M5.4 pipeline commands. compile (stage_ape orchestration, swipl
  subprocess, emit + payload, write-after-green), queries, align,
  review-manifest, release-manifest, ledger-validate,
  derive-review-manifest. Acceptance: `ckc compile` reproduces the
  committed corpus byte-identically; align resolver differential on live
  inputs; release-manifest byte-identical.
- M5.5 UI (K5). Acceptance: all live pages byte-identical vs `ui.py
  render` dual render; tests/ui replay green (84 red + 15 green, fixture
  bytes unchanged bar ruled FC2); POST protocol + CAS + committed-corpus
  law verbatim; CSP/script/copy laws enforced; escaping + determinism
  theorems verified.
- M5.6 dist. Verified BagIt assembly + deterministic tar writer; gzip
  via allowlisted dep; the 62 red cases re-expressed in the Rust harness;
  double-build determinism; FC4 re-baseline + release-manifest regen.
  Acceptance: red suite parity table; archive verifies via
  `sha256sum -c`; determinism green.
- M5.7 cutover + retirement + scrub. CI swap (drop bootstrap cmp, regen
  --check, goal.py check; add rust jobs as sole authority; keep swipl
  probes + cleanliness); delete `vendor/e--/`, `tools/` (4 .emm + 4 .py),
  `tests/strict/`, `red.sh`; NOTICE drops vendor/e-- (Apache-2.0 tree
  leaves; GPLv3 combination statement re-derived) → release-manifest
  regen; `.gitattributes` `*.emm` line; reference scrub of every tracked
  E-- mention (26-file inventory via `git grep -iE 'e--|\.emm'`):
  README.md, docs/REFERENCE.md (audit story: E-- section → Rust
  verified-kernel section; Running/Operating/Close invocations),
  roadmap stubs, memory (refs + obsolete E-- machinery bullets pruned;
  era-bound scratch bullets condensed to replay-from-era-commit stubs),
  rounds.md, archive files (ui.md, harvest.md), polish row retarget,
  `grammar_functionwords.fit` hit audited (expected false positive);
  serena `language_servers` −python; settings sync. Acceptance: `git
  grep -iE 'e--|\.emm'` empty outside git history; full new decisive
  chain green (verify + trust-audit + ckc check + fresh compile + `git
  diff --quiet -- guidelines/` + release-manifest fresh); corpus
  byte-stable.
- M5.8 fork shrink. Remove in-compile replay from `ace_to_pl.pl`
  (emission-only closure); pipeline step = swipl emit → kernel replay;
  proof-class red probes re-pinned (FC2); header + REFERENCE compile
  contract + PROVENANCE posture updated (fork-notice date law: keep
  first-edit date); measure fork line reduction. Acceptance: corpus
  bytes identical; M5.2 differential battery green against the shrunk
  pipeline; re-pinned probes green.
- M5.9 milestone review. Fixed check set at open: spec audit (the one
  human-read pass — size, readability, spec↔REFERENCE-prose agreement),
  trust-audit hostile probes, kernel mutation campaign scored by
  verus-acceptance (surviving verified mutant = spec gap → fix spec),
  shell fault probes, differential replay, claim-soundness sweep of
  REFERENCE/README certification claims. Close → archive record +
  roadmap stub flip.

Ordering: strictly M5.1 → M5.2 → M5.3 → M5.4 → (M5.5 | M5.6) → M5.7 →
M5.8 → M5.9. Every commit on main keeps legacy gates green until M5.7.
`/goal` rounds + the parked hard-tier harvest run concurrently under
legacy tooling; unit differentials derive doc/fixture counts from HEAD
at run time (no frozen counts in acceptance).

## Sizing

Legacy: 9,031 lines E-- (goal 5008, ui 3060, dist 811, regen 152) →
8,710 generated Python; ace_to_pl.pl 3,598 (consumption modes est.
~1.2-1.6K, measured at M5.8); vendor/e-- 220K bytes; 54 .emm files (50 =
tests/strict), 11 tracked .py. Expect Rust impl ≈ 10-15K lines + proofs
(SOSP-era Verus systems ran ~5:1 proof:code; string/format logic should
land lower), spec ≤ ~1.5K. Fixture corpus riding through: tests/ui 1332
files, queries 297, adjudication 152, red 69, copy 22.

## Certification story (post-M5 trusted surface)

1. ACE corpus + lexicons — clinician review through the UI.
2. `ckc-spec` + shell file list + dep allowlist — the one human-read
   code artifact; certification = read it, run `cargo verus verify` +
   `ckc trust-audit`.
3. APE fork emission closure (`ace_to_pl.pl` post-shrink) + vendored
   APE/Clex — human-read Prolog TCB (unchanged audit story; Prolog-side
   formal verification out of scope).
4. Pinned toolchains: SWI 9.2.9 (digest-pinned container), Verus release
   asset (sha256-pinned) + its rustup toolchain + bundled Z3 + vstd —
   one hash-pinned upstream TCB — plus Kani (secondary gate), git.

Claim wording (honest): "machine-verified against the committed spec
under a pinned verifier TCB" — never "foundationally proved".
Impl + proofs: uninspected, by design, forever.

## Out of scope

New features; schema v2; verifying the APE fork or SWI; web hosting;
scratch/evidence tooling language policy (gitignored scratch stays
free-form); rewriting history (E-- lives in git history).
