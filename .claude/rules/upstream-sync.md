---
paths:
  - "CLAUDE.md"
---

# Upstream sync — `CLAUDE.md` template refresh

`CLAUDE.md` = a verbatim copy of `~/.local/app/agents/claude/CLAUDE.project.md`; a refresh overwrites it whole, so project law never lives there. last-sync = agents@8fc2e19.
- Refresh = one migration-only session per `~/.local/app/agents/claude/prompts/refresh.md`. last-sync = the upstream commit whose template equals `git show HEAD:CLAUDE.md`: `git -C ~/.local/app/agents log --format=%h -- claude/CLAUDE.project.md | while read -r c; do git -C ~/.local/app/agents show "$c:claude/CLAUDE.project.md" | cmp -s - <(git show HEAD:CLAUDE.md) && { echo "$c"; break; }; done` (the recorded value may be stale). Delta = `git -C ~/.local/app/agents diff <last-sync> HEAD -- claude/CLAUDE.project.md` + the commit bodies of `git -C ~/.local/app/agents log <last-sync>..HEAD -- claude/`.
Invariants a refresh must keep, else restore:
- Line 1 = `@.agent/spec.md` import (silent when missing — a fresh `claude -p` answering a spec-only question with zero tool calls = the check).
- `Session flow` names `.agent/spec.md` with five sections — `Intent`, `Artifacts`, `Decisions`, `Tasks` (`- [ ]` open units in order, `- [x] <sha>` once committed, ticked rows cleared at phase close; last line = `.agent/deferred.md` pointer), `Phase` — plus `.agent/deferred.md` (deferral queue) + `.agent/review.md`; `Engineering` routes deferrals to `.agent/deferred.md` rows.
- `Session flow` teammates bullet cites global `CLAUDE.md` `Subagents` (triggers + mechanics; `rules/corpus.md` + `rules/rust.md` rely on it) + binds `reviewer` to every closing diff; `Execution` Git = commit bodies name each teammate the unit used (name, role, verdict).
- `Authoring` routes a role's standing rules to `~/.claude/agents/<role>.md` + thinking depth to the launch `--effort` ⇒ project `.claude/settings*.json` carries no model/effort pins and `.claude/agents/` stays absent.
- `.claude/rules/` two-tier bullet (bare | `paths:`) present — the sole carrier of project law + teammate inheritance.
- No `## Claude Code` section; no retired-flow references (session commands incl. `/goal`, attached roadmap/polish/memory ledgers).
Post-refresh: `git diff HEAD -- CLAUDE.md` → any repo-measured law in the removed lines outside the upstream delta folds into the owning `.claude/rules/` file; the clinician design law lives only in `rules/clinician-design.md`.
