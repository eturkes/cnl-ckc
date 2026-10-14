---
paths:
  - ".agent/spec-lint"
---

# Spec shape check

`.agent/spec-lint` = the template's copy (each refresh overwrites it) = the first step of `just check` (`spec-lint: ok`); `ckc check` agent-spec re-checks sections + `Tasks` grammar (firing = `rust.md` harnesses). Firing: `git show 0eb3dd55:.agent/spec.md` (`Phase` line 1 `MAINTAIN, scope = …`) → rc 1 `Phase line 1 = <PHASE> — <scope> on one line`.
