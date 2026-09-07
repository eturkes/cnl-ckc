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
- Corpus-extension workflow — `/goal` loop, body + procedure = `docs/REFERENCE.md` § Operating; worklist `.agent/queue.md` ← `.agent/compendium.{md,tsv}`; roles = `.claude/rules/corpus.md`.
In build: verified kernel `rust/` — gate `just rust` (`.claude/rules/rust.md`); `rust/target/release/ckc {v1 check|render|aggregate-check|recursion-check|answer, align-check, trust-audit}`. Tooling `just tools`; full gate `just gate`.

## Decisions

- Product = KB artifacts (`guidelines/*/{ace,pl}` + lexicons + ledgers + committed queries with answers + traces); compiled Prolog = the public interface (schema v1 per REFERENCE); sole in-repo consumer = the loopback reviewer UI. Feature scope final; new scope only by explicit user direction (M5–M7).
- Language boundary: Prolog owns ACE→knowledge emission (`ace_to_pl.pl` compile/proof/question modes) + stays the portable KB product; hand-authored Prolog confined to that closure. Rust owns the bounded in-repo consumer (v1 read/write, engine, replay, answers, traces, trace check), verification, custody + all non-Prolog tooling — never a general-purpose Prolog runtime, never domain knowledge. E-- + generated Python retire at cutover; until then `regen.py --check` + `goal.py check` stay CI-authoritative.
- Consumption→Rust affirmed: no mature Prolog verifier ⇒ that role gains human-reviewable verification only by migration; emission stays Prolog, certified at M6 by translation validation.
- Verification line (grading = `archive/rust-rewrite-plan.md` § Verification line): human-read `ckc-spec` + uninspected `ckc-kernel` (never read) + pinned Verus `--no-cheating` + `ckc trust-audit`. Theorems = KB semantics (K1–K3), custody chain source↔ACE↔clauses↔verdicts↔export (K4 core), render fidelity + POST guard/ledger CAS (K5 core), M6 emission correspondence. Shell tier (enumerated, fixture-gated, no theorems) = the Intent's trusted software + sockets, HTTP, archive assembly. Every spec line traces to a soundness/custody claim; claim = "machine-verified against the committed spec under a pinned verifier TCB". Fallback (Verus blocker, proof:impl >5×, solver brittleness) → Creusot or a narrowed kernel.
- Spine, soundness-first: M5.2 close → M6 → M5.3–M5.7 re-tiered to the line (M5.8 fork shrink folds into M5.4) → M5 review → M7. Per unit: contract of testable predicates + tier before code (`.agent/contracts/<unit>.md`, archived at close); differential parity before any legacy deletion; corpus + query artifacts byte-stable except ruled re-pins. M5.2 rulings R1–R30 (`archive/contracts/m5u2.md`) bind its sub-units.
- Compiler not fixed: schema version = the swap seam; ACE extension, schema v2, companion targets (ProbLog) = evaluated at M7, not presumed. Interim: a statement ACE cannot express ⇒ region `pending` + `inexpressible` queue blocker, row blocked (banks the M7 census).
- Corpus: knowledge-only fixture-free ACE on the frozen v1 schema; obligations discharged per document + aggregate; answers/traces = machine-derived demonstrations. Neutrality: nothing source-language-specific in tooling/schemas/ledgers; domain rules live in corpus data.
- Deps minimal + enumerated (`rust/trust/deps-allowlist.tsv`: mature, easily reasoned about, dangerous to hand-write); verifier pinned (`rust/verus.lock`). Review: check set fixed before reading; rows adjudicated in `.agent/review.md`.

## Deferred

- M5.2a engine/load perf (IN PROGRESS, `.agent/contracts/m5u2a.md`): arena reclamation + load/scan fix, spec unchanged. Accept: 10 R30d-parked cases green ≤30 s; k2 corpus 3 lanes ≤30 s / ≤2 GB; P1/P2.
- M5.2b K3 trace + trace-check + Kani (IN PROGRESS, `.agent/contracts/m5u2b.md`; spec `ckc-spec/src/trace.rs` landed). Accept: lanes D/E byte-identical; tests/queries replay; R32–R34 ruled; `k3_sound` discharged; P1–P7.
- M6 emission certification: verified DRS→v1 correspondence checker per committed compile. Accept: every committed document + query certifies in CI; `ace_to_pl.pl` leaves the human-read trust story (REFERENCE, `vendor/ape/PROVENANCE`).
- M5.3 validators (K4). Accept: `ckc check` section parity with `goal.py check`; red/adjudication/copy replay green; FC2 re-pins ruled.
- M5.4 pipeline (`ckc compile|queries|align|review-manifest|release-manifest|ledger-validate`) + fork shrink (in-compile replay → kernel; proof-class probes re-pinned). Accept: corpus + manifest byte-identical.
- M5.5 UI (K5). Accept: pages byte-identical vs `ui.py render`; tests/ui 84 red + 15 green; POST/CAS/committed-corpus law verbatim; fidelity theorems.
- M5.6 dist (shell tier, pinned dep). Accept: 62 red cases re-expressed; `sha256sum -c` + double-build determinism; FC4 re-baseline.
- M5.7 cutover: CI swap; delete `vendor/e--`, `tools/`, `tests/strict`, `red.sh`; NOTICE/REFERENCE/README/rules scrub. Accept: `git grep -iE 'e--|\.emm'` empty; new chain green.
- M5 review (`.agent/review.md`): spec audit, trust-audit hostile probes, mutation campaign scored by verus-acceptance, shell fault probes, differential replay, claim-soundness sweep. Accept: rows all adjudicated.
- M7 gap coverage: `inexpressible` census class in the coverage grammar; taxonomy from banked blockers + compendium sampling; disposition per class (ACE extension | schema v2 | companion target). Accept: user-ruled plan; first class dispositioned.
- Scratch validators (`.scratch/m5u*/`) = temporary encodings. Accept: committed Rust harness ≤ M5.7.
- Corpus rounds to exhaustion (legacy tooling). Accept: meter `terminal remaining: orgs=0 rows=0 provisional=0`; then hard-tier harvest (`archive/hard-tier-register.md`).
- `goal align` fail-path probes (low). Accept: one probe per `fail("align",…)` branch → rc 2 + detail.

## Phase

IMPLEMENT (M5 open; legacy gates authoritative until M5.7).
