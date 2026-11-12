---
paths:
  - ".agent/spec-lint"
  - "rust/ckc/src/check/agent_spec.rs"
---

# Spec shape check

`.agent/spec-lint` = the template's copy (each refresh overwrites it) = the first step of `just check` (`spec-lint: ok`); `ckc check` agent-spec re-checks sections + `Tasks` grammar (firing = `rust.md` harnesses) over the lines spec-lint reads: both skip fenced blocks by Markdown's rule (≤ 3 leading spaces; ≥ 3 backticks with no backtick after them, or tildes; a closer repeats the opener's character, at least as long, nothing after it; `check/agent_spec.rs` `unfenced`) ⇒ a fence-rule change lands in both. Parity firing = the unit test `fenced_lines_read_as_spec_lint_reads_them`: nine fence plants in a copy of `.agent/spec.md`, each verdict = spec-lint's rc on that copy (the fence-blind checker reds the six fenced plants, each holding `## Fake`, that spec-lint passes). Firing: `git show 0eb3dd55:.agent/spec.md` (`Phase` line 1 `MAINTAIN, scope = …`) → rc 1 `Phase line 1 = <PHASE> — <scope> on one line`; a line opening on inline code (```` ```make``` builds it. ````) followed by `## Fake` → rc 1, the finding naming section `Fake` on its own line.
