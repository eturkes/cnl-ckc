---
paths:
  - "vendor/**"
  - "rust/ckc/src/check/fork.rs"
  - ".github/workflows/ci.yml"
---

# Vendored forks

- Fork edits: a modified upstream file needs a prominent in-file notice — GPL-3.0 §5(a) via LGPL (`vendor/ape`; notice + ISO date, write-once = first fork-edit date). `ckc check` `fork-notice` (`rust/ckc/src/check/fork.rs`) enforces per `vendor/<tree>/PROVENANCE`; an unmodified file carrying a notice = a violation; it reads `git log` ⇒ every CI checkout running `ckc check` keeps `fetch-depth: 0`.
