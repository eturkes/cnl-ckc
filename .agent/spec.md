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
- Review custody (user ruling): a ledger row's `ace_commit` = the commit the reviewer viewed. The UI records HEAD at save (the guard refuses a decision whose bundle changed since render ⇒ same bundle); `ckc check` re-derives bundle v2 there for every nonempty row; recording the exact rendered commit = `.agent/deferred.md` row "Review records the rendered commit".
- Spine: M5 closed (IMPLEMENT); M7 + corpus rounds = MAINTAIN requests (user ruling; `.agent/deferred.md`). Per unit: contract of testable predicates + tier before code (`.agent/contracts/<unit>.md`, archived at close); differential parity before any legacy deletion; corpus + query artifacts byte-stable except ruled re-pins. M5.2 rulings R1–R30 (`archive/contracts/m5u2.md`) bind its sub-units.
- Compiler not fixed: schema version = the swap seam; ACE extension, schema v2, companion targets (ProbLog) = evaluated at M7, not presumed. A statement ACE cannot express ⇒ region `uncovered(inexpressible: <reason>)`, counted by the coverage meter (user ruling); gap taxonomy + per-class dispositions ruled = `docs/m7-gap-taxonomy.md` (first class: temporal).
- Corpus: knowledge-only fixture-free ACE on the frozen v1 schema; obligations discharged per document + aggregate; answers/traces = machine-derived demonstrations. Neutrality: nothing source-language-specific in tooling/schemas/ledgers; domain rules live in corpus data.
- Deps minimal + enumerated (`rust/trust/deps-allowlist.tsv`: mature, easily reasoned about, dangerous to hand-write); verifier pinned (`rust/verus.lock`). Review: check set fixed before reading; rows adjudicated in `.agent/review.md`.

## Tasks

- [ ] RESUME (paused 2026-10-02): finish line in force = the MAINTAIN Queue body's `Met when` (every in-scope row closed by its own acceptance check in its own commit, blocked on the user with what it owes, or waiting on its re-open trigger; full gate green on a clean tree at the closing commit; final message per the body). Resumed 2026-10-06 (session 17ba207e): Q1 → waits on its re-open trigger (user ruling; root cause + trigger = `.agent/deferred.md` Kani row); N2 contract approved with D7 (user) → kernel unit on `wt/n2` (`.scratch/worktrees/n2`); N3 = new find, funded after N2; release-manifest content keying = new row, user: decide later. Run record = `.scratch/mq/` (`plan.md`, `roster.md` = teammates + verdicts, `advisor-log.md` = calls + rulings + owed disclosures, `q1-diag.md`, `gate-*.log`) + `.scratch/agents/mq/`. Next = N2 (tester + reviewer pair) → N3 → closing review per lens over 3e1bc2a6..tip → full `just gate` on a clean tree → final message (covers the whole queue run: 96b5bed2..tip). Q5 + Q6 stay for their own sessions (user rulings).
- [ ] Q1 Kani engine + trace harnesses: waits on its re-open trigger (user ruling) = a `rust/kani.lock` bump → `engine_answer_tiny` rerun ≤1200 s; root cause = CBMC cannot fold heap-`Vec` enum reads (`.agent/deferred.md` row).
- [x] f0e89ff2 Q2 `ace_commit` custody content check (user ruling: recorded commit = the viewed commit).
- [x] 0ea7c076 Q3 non-v1 tests/queries fixtures → canonical v1 (10/10 rewritten); `tests/check/r79-nonv1.tsv` header only.
- [x] fed5db86 Q4 `.agent/spec.md` shape check in `ckc check`.
- [ ] Q5 M7 gap coverage: inexpressible class + ruled taxonomy landed; first class (temporal) owed as a user-directed feature unit.
- [ ] Q6 corpus rounds to exhaustion: user ruling = separate corpus-prompt sessions (not run here).
- [x] 0844a975 Q7 K3 negation certificate = call-time instance; calls failing `naf_safe` cut.
- [x] 4dbe27aa Q8 trace clause identity from the owning `% S<n>:` block.
- [x] e3f94d64 Q9 single-authority UI copy.
- [x] 3616a400 Q10 M5.3 late mutants replayed (`just late-mutants`).
- [x] 0d3898cd Q11 corpus.md Inexpressible cite.
- [x] 604924fb N1 `align_probes` stdin race.
- [ ] N2 Review records the rendered commit: contract `.agent/contracts/n2.md` (user-approved with D7: posted commit = HEAD or an ancestor).
- [ ] N3 `ckc check` scratch survives a kill (`.agent/deferred.md` row).
- Deferral queue = `.agent/deferred.md` (off-path improvements + ruled deferrals; one line + acceptance check each).

## Phase

MAINTAIN, scope = the whole product (IMPLEMENT closed: native chain authoritative; M7 gap coverage + corpus rounds = MAINTAIN requests, `.agent/deferred.md`).
