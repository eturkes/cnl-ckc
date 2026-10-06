---
paths:
  - "CLAUDE.md"
---

# Upstream sync — `CLAUDE.md` template refresh

`CLAUDE.md` = a verbatim copy of `~/.local/app/agents/claude/CLAUDE.project.md`; a refresh overwrites it whole, so project law never lives there. last-sync = agents@5471e83.
- Refresh = one migration-only session per `~/.local/app/agents/claude/prompts/refresh.md` (last-sync derivation, delta, obligations, checks). Phase prompt set = `~/.local/app/agents/claude/prompts/` (`resume.md`, `pause.md`, `maintain.md`, …).
Invariants a refresh must keep, else restore:
- Line 1 = `@.agent/spec.md` import (silent when missing — a fresh `claude -p` answering a spec-only question with zero tool calls = the check).
- `Session flow` names `.agent/spec.md` with five sections — `Intent`, `Artifacts` (path each + run command where it runs), `Decisions`, `Tasks` (`- [ ]` open units in order, `- [x] <sha>` once committed, ticked rows cleared at phase close; last line = `.agent/deferred.md` pointer), `Phase` (phase + scope); `ckc check` agent-spec enforces sections + Tasks grammar — plus `.agent/deferred.md` (deferral queue) + `.agent/review.md`; `Engineering` routes deferrals to `.agent/deferred.md` rows.
- `Session flow` teammates bullet cites global `CLAUDE.md` `Subagents` (triggers + mechanics; `rules/corpus.md` + `rules/rust.md` rely on it) + binds `reviewer` on every closing diff — one per lens in IMPLEMENT, one covering every lens elsewhere; `Execution` Git = commit bodies name each teammate the unit used (name, role, verdict).
- `Session flow` rulings bullet: `.claude/rules/` = the repo's rulings on template defaults, each keyed on the clause it adapts, retires or marks inapplicable (index below).
- `Authoring` routes a role's standing rules to `~/.claude/agents/<role>.md` + thinking depth to the launch `--effort` ⇒ project `.claude/settings*.json` carries no model/effort pins and `.claude/agents/` stays absent.
- `.claude/rules/` two-tier bullet (bare | `paths:`) present — the sole carrier of project law + teammate inheritance.
- `Execution` Research = `WebSearch` + verbatim page text (`webtext`) + the signed-in browser.
- No `## Claude Code` section; no retired-flow references (session commands incl. `/goal`, attached roadmap/polish/memory ledgers, `migrate.md`, stored prototype proof, `WebFetch`).
Rulings keyed on template clauses:
- `Session flow` MAINTAIN bodies → `ops.md` corpus loop: the corpus prompt (`docs/REFERENCE.md` § Operating) = the MAINTAIN body for corpus rounds; continue = `resume.md`, stop to continue later = `pause.md`.
- `Session flow` closing diff → `corpus.md` round roles: the one closing `reviewer` covers 7 lenses (L0 completeness, L1–L6 adversarial); user ruling = template form, L0 folded in.
- `Session flow` IMPLEMENT contracts + tiers → `ops.md` unit contracts (`.agent/contracts/<unit>.md`, archived at unit close).
- `Execution` Git → `ops.md` close order (stage, then the decisive check) + `corpus.md` round close (scoped commits, then the release-manifest regen commit).
- `Authoring` human-facing → `clinician-design.md` (design law + copy/design lint gates); `ops.md` presentation rule.
- Prototype location, CI, review ledger, spec layout = template defaults: prototype = `prototype/<name>/`; CI = `.github/workflows/ci.yml`; ledger = `.agent/review.md`.
Post-refresh: `git diff HEAD -- CLAUDE.md` → any repo-measured law in the removed lines outside the upstream delta folds into the owning `.claude/rules/` file; the clinician design law lives only in `rules/clinician-design.md`.
