# cnl-ckc spec

## Intent

cnl-ckc compiles the natural language of published clinical guidelines into an executable knowledge base through a layer clinicians can audit: each guideline passage becomes an Attempto Controlled English (ACE) document, reviewed by clinicians beside its source passage, and compiled into plain engine-portable Prolog (schema v1) with proof obligations, custody digests, and committed demonstration queries carrying answers and proof traces. Two goals, in priority order:

1. Clinician-auditable compilation. The ACE/Prolog approach stands; its semantic gaps (dosing arithmetic, probability, temporal logic, preference strength) need a coverage plan. The compiler is not fixed: ACE construct extension, a schema v2, or a companion/alternative target (ProbLog for uncertainty-bearing content) stay open for evaluation. NL→ACE fidelity = clinician review through the UI, permanently outside formal verification.

2. Formal verification of the compilation and of every other logical step the software's function depends on, in the verified-kernel pattern: a small human-read formal specification, uninspected AI-written implementation + proofs, a pinned deterministic checker, an escape-hatch audit — the human-audited layer kept as small as possible. SaMD soundness draws the line: the logical steps from ACE to clauses to answers, and the custody chain binding source, ACE, clauses, review verdicts and export, are verified; git, the filesystem, subprocesses, the clock, pinned mature dependencies and process/legal validators are trusted software under fixture gates. The Prolog stack's retained role (ACE→DRS→v1 emission) must also gain human-reviewable verification.

Scope stays minimal: fetch, normalize to ACE, compile, review locally, export deterministically; no serving layer, no runtime LLM. Everything a human must read is ACE, the formal specification, or a named small Prolog closure until that closure is certified.

## Artifacts

Approved; env = `.claude/rules/ops.md` + `.claude/rules/rust.md`; `ckc` = `rust/target/release/ckc` (`just build`).
- Reviewer UI — `ckc ui serve [<port>] [<repo-root>]` (loopback, reads HEAD); `ckc ui render <out>`, `ckc ui check`, `ckc ui request <method> <path> …`.
- ACE representations + compiled KB — `guidelines/<id>/`; `ckc compile <id>` → `ckc queries <id>` → `ckc check`; export `ckc release-manifest` + `ckc dist build [<dest>]`; emission certification `ckc certify <id>`.
- Corpus-extension workflow — pasted-prompt loop, prompt + procedure = `docs/REFERENCE.md` § Operating; worklist `.agent/queue.md` ← `.agent/compendium.{md,tsv}`; roles = `.claude/rules/corpus.md`.
- Gates — `just gate` = `just rust` (fmt, clippy, Verus `--no-cheating`, build, trust-audit, tests, cargo-deny, gitleaks, zizmor) + `just check` (`ckc check`, `ckc ui check`, fresh compile + queries byte-stable) + `just certify`; `just tools` installs the pinned toolchain; weekly `just kani`, `just outdated`; CI `.github/workflows/ci.yml`.

## Decisions

- Product = KB artifacts (`guidelines/*/{ace,pl}` + lexicons + ledgers + committed queries with answers + traces); compiled Prolog = the public interface (schema v1 per REFERENCE); sole in-repo consumer = the loopback reviewer UI. Feature scope final; new scope only by explicit user direction (M5–M7).
- Language boundary: Prolog owns ACE→knowledge emission (`ace_to_pl.pl` compile/proof/question modes) + stays the portable KB product; hand-authored Prolog confined to that closure. Rust owns the bounded in-repo consumer (v1 read/write, engine, replay, answers, traces, trace check), verification, custody + all non-Prolog tooling — never a general-purpose Prolog runtime, never domain knowledge. The native chain (`just check`, `just certify`) is the sole authority; the retired pre-cutover toolchain lives at tag `legacy`.
- Consumption→Rust affirmed: no mature Prolog verifier ⇒ that role gains human-reviewable verification only by migration; emission stays Prolog, certified at M6 by translation validation.
- Verification line (grading = `archive/rust-rewrite-plan.md` § Verification line): human-read `ckc-spec` + AI-authored `ckc-kernel` (never human-reviewed; agents read + edit it) + pinned Verus `--no-cheating` + `ckc trust-audit`. Theorems = KB semantics (K1–K3), custody chain source↔ACE↔clauses↔verdicts↔export (K4 core), render fidelity + POST guard/ledger CAS (K5 core), M6 emission correspondence. Shell tier (enumerated, fixture-gated, no theorems) = the Intent's trusted software + sockets, HTTP, archive assembly. Every spec line traces to a soundness/custody claim; claim = "machine-verified against the committed spec under a pinned verifier TCB". Fallback (Verus blocker, proof:impl >5×, solver brittleness) → Creusot or a narrowed kernel.
- Spine, soundness-first: M5.2 close → M6 → M5.3–M5.7 re-tiered to the line (M5.8 fork shrink folds into M5.7) → M5 review → IMPLEMENT close; M7 + corpus rounds = MAINTAIN requests (user ruling; `.agent/deferred.md`). Per unit: contract of testable predicates + tier before code (`.agent/contracts/<unit>.md`, archived at close); differential parity before any legacy deletion; corpus + query artifacts byte-stable except ruled re-pins. M5.2 rulings R1–R30 (`archive/contracts/m5u2.md`) bind its sub-units.
- Compiler not fixed: schema version = the swap seam; ACE extension, schema v2, companion targets (ProbLog) = evaluated at M7, not presumed. Interim: a statement ACE cannot express ⇒ region `pending` + `inexpressible` queue blocker, row blocked (banks the M7 census).
- Corpus: knowledge-only fixture-free ACE on the frozen v1 schema; obligations discharged per document + aggregate; answers/traces = machine-derived demonstrations. Neutrality: nothing source-language-specific in tooling/schemas/ledgers; domain rules live in corpus data.
- Deps minimal + enumerated (`rust/trust/deps-allowlist.tsv`: mature, easily reasoned about, dangerous to hand-write); verifier pinned (`rust/verus.lock`). Review: check set fixed before reading; rows adjudicated in `.agent/review.md`.

## Tasks

- [x] c1069299 U1 ci — `archive/contracts/ci.md`: static musl `ckc` for the container jobs (`certify` red since M6), Kani cold bootstrap, zizmor `just workflows`, weekly `just outdated` drift job in place of Dependabot cargo.
- [x] 86acf59c U1b fmt — `fmt`/`fmt-check` glob `ckc/src/*.rs` only → 37 shell modules under `ckc/src/{check,ui}/` outside the format gate (2 unformatted: `check/documents.rs`, `check/mod.rs`); red = planted misformat there passes the old gate.
- [x] c8cbe43b U2 cas — `archive/contracts/m5u5-cas.md`: identity-validated ledger lock + std-only regression seen red (review C-01..C-04).
- [x] f697dee4 U3 evidence — pre-cutover binary fce50147 + legacy tools: K1 A/H 349/349, K2 0 divergences, K3 D/E 4/4, queries replay 60/0, M5.2 suite 224/25 pending, K3 matrix 114/114, M5.1 94/94, trust battery 16/16, perf 17 rows 0 failures, M5.3 R98 parity 12/12, M5.3 mutants legacy 46/46 + native 38/42 (4 late rows unbound on pre-R80 trees → deferred row).
- [x] 39968a58 U4 harness — `archive/contracts/harness.md` (6e5f5eeb H1+H2, 907e41dd, 257bd537 H3, f9f30002 H4, fba6faa9 H5, 82f37216 H6, c541dfea H7, 39968a58 generators + rules): `ckc trust-audit --write`, trust battery, align + v1 suites, align probes, bounded check mutants → committed; each seen red.
- [x] 24177e16 6ceb2d8d 07f76e88 U5a M5.7 cutover — `.agent/contracts/m5u7.md` P1–P7 (R99–R104): tag `legacy` = 39968a58, R99 re-pin, fork shrink, deletions, CI `check` swap, NOTICE, README/REFERENCE/rules scrub; red only on release-manifest freshness until U5b.
- [x] f697dee4 U5b release-manifest regen from HEAD = U5a → gate steps green on its tree (test step rerun after a one-binary flake).
- [ ] U6 M5 review (fixes 8863529f v1 FIFO gate, 4a9ccce9 spec minimality, 24e2407c docs; ledger: every row adjudicated except R-13 perf + S-10 close state) — `.agent/review.md` R-01..R-14 + S-01..S-10 + C-01..C-04; one `reviewer` per lens (security-vocabulary rows = MAIN); fixes land before close.
- [ ] U7 close — superseded branches deleted after proof (user ruling), Decisions current, deferred rows, archive contracts, `Phase: MAINTAIN`, `just gate` green on the clean closing tree + container `check`/`certify` on the same static artifact.
- Deferral queue = `.agent/deferred.md` (off-path improvements + ruled deferrals; one line + acceptance check each).

## Phase

IMPLEMENT (M5 open; native chain authoritative since the U5a cutover).
