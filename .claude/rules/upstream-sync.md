---
paths:
  - "CLAUDE.md"
---

# Upstream sync — `CLAUDE.md` template refresh

`CLAUDE.md` = a verbatim copy of `~/Projects/agents/claude/CLAUDE.project.md`; a refresh overwrites it whole, so project law never lives there. Invariants a refresh must keep, else restore:
- Line 1 = `@.agent/spec.md` import (silent when missing — `claude -p` answering a spec-only question with zero tools = the check).
- `Session flow` names `.agent/spec.md` (≤ 8 KB, five sections) + `.agent/review.md`; `Engineering` routes deferrals to `Deferred` rows.
- `.claude/rules/` two-tier bullet (bare | `paths:`) present — the sole carrier of project law + teammate inheritance.
- No `## Claude Code` section; no retired-flow references (session commands, attached roadmap/polish/memory ledgers).
Post-refresh: `git diff HEAD -- CLAUDE.md` → any repo-measured law in the removed lines folds into the owning `.claude/rules/` file; the clinician design law lives only in `rules/clinician-design.md`.
