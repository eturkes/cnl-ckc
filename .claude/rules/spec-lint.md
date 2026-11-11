---
paths:
  - ".agent/spec-lint"
---

# Spec shape check

`.agent/spec-lint` = the template's copy (each refresh overwrites it) = the first step of `just check` (`spec-lint: ok`); `ckc check` agent-spec re-checks sections + `Tasks` grammar (firing = `rust.md` harnesses). spec-lint skips fenced blocks by Markdown's rule (a closer repeats the opener's character, at least as long, nothing after it); agent-spec has no fence logic, so every `## ` line is a heading and a fenced one passes spec-lint yet fails `ckc check` — interim state until the `.agent/deferred.md` row "`ckc check` agent-spec fence parity" lands. Firing: `git show 0eb3dd55:.agent/spec.md` (`Phase` line 1 `MAINTAIN, scope = …`) → rc 1 `Phase line 1 = <PHASE> — <scope> on one line`; a line opening on inline code (```` ```make``` builds it. ````) followed by `## Fake` → rc 1, the finding naming section `Fake` on its own line.
