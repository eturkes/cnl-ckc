# cnl-ckc spec

## Intent

cnl-ckc compiles the natural language of published clinical guidelines into an executable knowledge base through a layer clinicians can audit: each guideline passage becomes an Attempto Controlled English (ACE) document, reviewed by clinicians beside its source passage, and compiled into plain engine-portable Prolog (schema v1) with proof obligations, custody digests, and committed demonstration queries carrying answers and proof traces. Two goals, in priority order:

1. Clinician-auditable compilation. The ACE/Prolog approach stands; its semantic gaps (dosing arithmetic, probability, temporal logic, preference strength) need a coverage plan. The compiler is not fixed: ACE construct extension, a schema v2, or a companion/alternative target (ProbLog for uncertainty-bearing content) stay open for evaluation. NL→ACE fidelity = clinician review through the UI, permanently outside formal verification.

2. Formal verification of the compilation and of every other logical step the software's function depends on, in the verified-kernel pattern: a small human-read formal specification, uninspected AI-written implementation + proofs, a pinned deterministic checker, an escape-hatch audit — the human-audited layer kept as small as possible. SaMD soundness draws the line: the logical steps from ACE to clauses to answers, and the custody chain binding source, ACE, clauses, review verdicts and export, are verified; git, the filesystem, subprocesses, the clock, pinned mature dependencies and process/legal validators are trusted software under fixture gates. The Prolog stack's retained role (ACE→DRS→v1 emission) must also gain human-reviewable verification.

Scope stays minimal: fetch, normalize to ACE, compile, review locally, export deterministically; no serving layer, no runtime LLM. Everything a human must read is ACE, the formal specification, or a named small Prolog closure until that closure is certified.

## Artifacts

Approved; env = `.claude/rules/ops.md`.
- Reviewer UI — `python3 -P tools/ui.py serve [<port>]` (loopback, reads HEAD); `render <out>`, `check`.
- ACE representations + compiled KB — `guidelines/<id>/`; `python3 -P tools/goal.py compile <id>` → `queries <id>` → `check`; export `goal.py release-manifest` + `tools/dist.py build`.
- Corpus-extension workflow — pasted-prompt loop, prompt + procedure = `docs/REFERENCE.md` § Operating; worklist `.agent/queue.md` ← `.agent/compendium.{md,tsv}`; roles = `.claude/rules/corpus.md`.
In build: verified kernel `rust/` — gate `just rust` (`.claude/rules/rust.md`); `rust/target/release/ckc {v1 check|render|aggregate-check|recursion-check|answer|trace|trace-check, align-check, trust-audit, check [repo-root] (M5.3, parity with `goal.py check`), compile <id> | queries <id> | align <id> | review-manifest | derive-review-manifest | ledger-validate | release-manifest (M5.4, byte-parity with goal.py), dist build [<dest>] (M5.6, BagIt bag = tools/dist.py members + tags byte-identical; archive DEFLATE bytes differ, FC4), certify <id> | certify --cases tests/certify/cases.tsv, ui serve [<port>] | render <out> | check | request <method> <path> | copy-check <source.txt> (M5.5, byte parity with `tools/ui.py`; frozen suite `rust/ckc/tests/ui_fixtures.rs`, harness `.scratch/m5u5/test/parity.py --rust-bin`)}`; `just certify` (M6). Tooling `just tools`; full gate `just gate` = rust + legacy + certify.

## Decisions

- Product = KB artifacts (`guidelines/*/{ace,pl}` + lexicons + ledgers + committed queries with answers + traces); compiled Prolog = the public interface (schema v1 per REFERENCE); sole in-repo consumer = the loopback reviewer UI. Feature scope final; new scope only by explicit user direction (M5–M7).
- Language boundary: Prolog owns ACE→knowledge emission (`ace_to_pl.pl` compile/proof/question modes) + stays the portable KB product; hand-authored Prolog confined to that closure. Rust owns the bounded in-repo consumer (v1 read/write, engine, replay, answers, traces, trace check), verification, custody + all non-Prolog tooling — never a general-purpose Prolog runtime, never domain knowledge. E-- + generated Python retire at cutover; until then `regen.py --check` + `goal.py check` stay CI-authoritative.
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
- [ ] U3 evidence — pre-cutover binary `.scratch/gate/ckc-f3910477` (sha256 fce50147…), logs `.scratch/gate/`; results ride the U5 body + review rows.
  - M5.3 grading remainder: mutants replay → `.scratch/gate/m5u3-mutants-r80-final.log`; R98 = re-pin the parity dist meter bytes (1481762 → 1490978 B) + rerun parity.
  - R-09 replays done (`.scratch/gate/u3-summary.log`, driver `.scratch/u3/run.sh`): K1 A/H 349/349 ×2, K2 answer/recursion/aggregate 0 divergences + trace D 4/4 E 4/4, queries replay 60/0, M5.2 suite 224 pass/25 pending, K3 matrix 114/114, M5.1 suite 94/94 + diff 0, trust battery 16/16; R-13 perf `.scratch/gate/u3-perf.log` 17 rows 0 failures.
  - Stopped at session close (user direction), rerun both from a clean tree: mutants (partial 22/46, all legacy=ok rust=ok) `python3 -P .scratch/m5u3/mutants.py --check --rust-bin .scratch/gate/ckc-f3910477` → `.scratch/gate/m5u3-mutants-r80-final.log` (~3 h); R98 parity `python3 -P .scratch/m5u3/parity_r98.py --tree .scratch/m5u3/baseline/tree-r80 --legacy-pinned --rust-bin .scratch/gate/ckc-f3910477` → `.scratch/gate/m5u3-parity-r98.log` (~40 min; waits on the legacy lock).
- [x] U4 harness — `archive/contracts/harness.md` (6e5f5eeb H1+H2, 907e41dd, 257bd537 H3, f9f30002 H4, fba6faa9 H5, 82f37216 H6, c541dfea H7, this commit = generators + rules): `ckc trust-audit --write`, trust battery, align + v1 suites, align probes, bounded check mutants → committed; each seen red.
- [ ] U5a M5.7 cutover — `.agent/contracts/m5u7.md` P1–P7 (R99–R104): tag `legacy`, re-pin, fork shrink, deletions, CI `check` swap, NOTICE, README/REFERENCE/rules scrub (known red: release-manifest freshness).
  - drafts: `.scratch/m5u7/{README.draft.md,reference-edits.txt,mech-edits.txt}`
- [ ] U5b release-manifest regen from HEAD = U5a → decisive `just gate`.
- [ ] U6 M5 review — `.agent/review.md` R-01..R-14 + S-01..S-10 + C-01..C-04; one `reviewer` per lens (security-vocabulary rows = MAIN); fixes land before close.
- [ ] U7 close — superseded branches deleted after proof (user ruling), Decisions current, deferred rows, archive contracts, `Phase: MAINTAIN`, `just gate` green on the clean closing tree + container `check`/`certify` on the same static artifact.
- Deferral queue = `.agent/deferred.md` (off-path improvements + ruled deferrals; one line + acceptance check each).

## Phase

IMPLEMENT (M5 open; legacy gates authoritative until M5.7).
