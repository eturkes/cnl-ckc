# cnl-ckc spec

## Intent

cnl-ckc compiles the natural language of published clinical guidelines into an executable knowledge base through a layer clinicians can audit: each guideline passage becomes an Attempto Controlled English (ACE) document, reviewed by clinicians beside its source passage, and compiled into plain engine-portable Prolog (schema v1, with schema v2 temporal annotations) with proof obligations, custody digests, and committed demonstration queries carrying answers and proof traces. Two goals, in priority order:

1. Clinician-auditable compilation. The ACE/Prolog approach stands; its semantic gaps follow a coverage plan (`docs/m7-gap-taxonomy.md`): typed timing limits (durations, offsets, windows, spacing) are covered by schema v2, while dosing arithmetic, probability, preference strength and the remaining temporal shapes stay open, one unit per class. The compiler is not fixed: ACE construct extension, further schema versions, or a companion/alternative target (ProbLog for uncertainty-bearing content) stay open for evaluation. NL→ACE fidelity = clinician review through the UI, permanently outside formal verification.

2. Formal verification of the compilation and of every other logical step the software's function depends on, in the verified-kernel pattern: a small human-read formal specification, uninspected AI-written implementation + proofs, a pinned deterministic checker, an escape-hatch audit — the human-audited layer kept as small as possible. SaMD soundness draws the line: the logical steps from ACE to clauses to answers, and the custody chain binding source, ACE, clauses, review verdicts and export, are verified; git, the filesystem, subprocesses, the clock, pinned mature dependencies and process/legal validators are trusted software under fixture gates. The Prolog stack's retained role (ACE→DRS→clause emission) keeps human-reviewable verification: translation validation certifies every compiled document and question against the specification.

Scope stays minimal: fetch, normalize to ACE, compile, review locally, export deterministically; no serving layer, no runtime LLM. Everything a human must read is ACE, the formal specification, or a named small Prolog closure until that closure is certified.

## Artifacts

Approved; env = `.claude/rules/ops.md` + `.claude/rules/rust.md`; `ckc` = `rust/target/release/ckc` (`just build`).
- Reviewer UI — `ckc ui serve [<port>] [<repo-root>]` (loopback, reads HEAD); `ckc ui render <out>`, `ckc ui check`, `ckc ui request <method> <path> …`.
- ACE representations + compiled KB — `guidelines/<id>/` (its `temporal.tsv` selects schema v2); `ckc compile <id>` → `ckc queries <id>` → `ckc check`; highlight tables `ckc align <id> <docid>`; review digests `ckc review-manifest <id>`; export `ckc release-manifest` + `ckc dist build [<dest>]`; emission certification `ckc certify <id>`.
- Corpus-extension workflow — pasted-prompt loop, prompt + procedure = `docs/REFERENCE.md` § Operating; worklist `.agent/queue.md` ← `.agent/compendium.{md,tsv}`; roles = `.claude/rules/corpus.md`.
- Gates — `just gate` = `just rust` (fmt, clippy, Verus `--no-cheating`, build, trust-audit, tests, cargo-deny, gitleaks, zizmor) + `just check` (`ckc check`, `ckc ui check`, fresh compile + queries byte-stable) + `just certify`; `just tools` installs the pinned toolchain; weekly `just kani`, `just outdated`; manual `just late-mutants` (~3 h); CI `.github/workflows/ci.yml`.

## Decisions

- Product = KB artifacts (`guidelines/*/{ace,pl,align}` + lexicons + temporal tables + ledgers + committed queries with answers + traces); compiled Prolog = the public interface (schema v1, + v2 temporal annotations where a guideline carries `temporal.tsv`, per REFERENCE); sole in-repo consumer = the loopback reviewer UI. Feature scope final; new scope only by explicit user direction (M5–M7).
- Language boundary: Prolog owns ACE→knowledge emission (`ace_to_pl.pl` compile/proof/question modes) + stays the portable KB product; hand-authored Prolog confined to that closure. Rust owns the bounded in-repo consumer (v1 + v2 read/write, engine, replay, answers, traces, trace check), verification, custody + all non-Prolog tooling — never a general-purpose Prolog runtime, never domain knowledge. The native chain (`just check`, `just certify`) is the sole authority; the retired pre-cutover toolchain lives at tag `legacy`.
- Consumption→Rust affirmed: no mature Prolog verifier ⇒ that role gains human-reviewable verification only by migration; emission stays Prolog, certified per document + question by translation validation (M6; v2 annotations since M7T).
- Verification line (grading = `archive/rust-rewrite-plan.md` § Verification line): human-read `ckc-spec` + AI-authored `ckc-kernel` (never human-reviewed; agents read + edit it) + pinned Verus `--no-cheating` + `ckc trust-audit`. Theorems (schemas v1 + v2) = KB semantics (K1–K3), custody chain source↔ACE↔clauses↔verdicts↔export (K4 core), render fidelity incl. the timing table + POST guard/ledger CAS (K5 core), M6 emission correspondence. Shell tier (enumerated, fixture-gated, no theorems) = the Intent's trusted software + sockets, HTTP, archive assembly. Every spec line traces to a soundness/custody claim; claim = "machine-verified against the committed spec under a pinned verifier TCB". Fallback (Verus blocker, proof:impl >5×, solver brittleness) → Creusot or a narrowed kernel.
- Review custody (user rulings): a ledger row's `ace_commit` = the commit the reviewer viewed = the commit the page was rendered from. The page posts it back; the UI records the posted commit only once review bundle v2 derived there equals the reviewed digest and it is HEAD or an ancestor of HEAD (`.claude/rules/ui.md`) ⇒ custody = the recorded commit holds the reviewed bundle (a hand-made POST can name another such commit); `ckc check` re-derives review bundle v2 there for every nonempty row.
- Spine: M5 closed (IMPLEMENT); M7 gap classes + corpus rounds = MAINTAIN requests (user ruling; `.agent/deferred.md`). Per unit: contract of testable predicates + tier before code (`.agent/contracts/<unit>.md`, archived at close); corpus + query artifacts byte-stable except ruled re-pins. M5 rulings (`archive/contracts/m5u*.md`; m5u2 R1–R30 = codec + engine law, cited in code) bind the code they shaped.
- Compiler not fixed: schema version = the swap seam. Schema v2 = v1 + `guideline_interval/6` + `guideline_recurrence/4` (M7T, contract `archive/contracts/m7t.md`): annotations follow the matched v1 clauses, every v1 shape stays byte-identical, a guideline's `temporal.tsv` selects v2 for its documents + questions, a composition holds one version. A statement ACE cannot express ⇒ region `uncovered(inexpressible: <reason>)`, counted by the coverage meter (user ruling). Gap taxonomy + per-class dispositions ruled = `docs/m7-gap-taxonomy.md`: temporal implemented; each remaining class (ACE extension, a later schema, companion targets such as ProbLog) = its own user-directed unit, not presumed.
- Corpus: knowledge-only fixture-free ACE on the frozen v1 schema + v2 annotations via the guideline's `temporal.tsv` (an unused table row stays legal: user ruling, no liveness check); obligations discharged per document + aggregate; answers/traces = machine-derived demonstrations. Neutrality: nothing source-language-specific in tooling/schemas/ledgers; domain rules live in corpus data.
- Deps minimal + enumerated (`rust/trust/deps-allowlist.tsv`: mature, easily reasoned about, dangerous to hand-write); verifier pinned (`rust/verus.lock`). Review: check set fixed before reading; rows adjudicated in `.agent/review.md`.

## Tasks

- [ ] Q1 Kani engine + trace harnesses: waits on its re-open trigger (user ruling) = the next `rust/kani.lock` bump → rerun `engine_answer_tiny` ≤1200 s.
- [ ] Q5 M7 next gap class: the user's pick, in its own MAINTAIN session (user ruling); remaining classes = `docs/m7-gap-taxonomy.md`.
- [ ] Q12 temporal shapes beyond schema v2 (counts per period, recurrence scope, unquantified sequencing, measurement nouns): waits on a user semantics ruling.
- [ ] Q13 approximate + ranged temporal bounds: waits on a user semantics ruling.
- [ ] Q14 guideline-wide temporal mapping display: the next UI unit (user ruling).
- [ ] Q6 corpus rounds to exhaustion: separate corpus-prompt sessions (user ruling).
- Deferral queue = `.agent/deferred.md` (each row above + ruled deferrals; one line + acceptance check each).

## Phase

MAINTAIN, scope = the whole product (IMPLEMENT closed: native chain authoritative; M7 gap classes + corpus rounds = MAINTAIN requests, `.agent/deferred.md`).
