---
paths:
  - "vendor/**"
  - "rust/ckc/src/check/fork.rs"
  - ".github/workflows/ci.yml"
---

# Vendored forks

- Fork edits: a modified upstream file needs a prominent in-file notice — GPL-3.0 §5(a) via LGPL (`vendor/ape`; notice + ISO date, write-once = first fork-edit date). `ckc check` `fork-notice` (`rust/ckc/src/check/fork.rs`) enforces per `vendor/<tree>/PROVENANCE`; an unmodified file carrying a notice = a violation; the notice date must name a calendar day and equal the first committed fork edit of its file (`git log --format=%as --reverse <import>..HEAD`); a `Pristine: yes` tree is checked by content alone (no history read); it reads `git log` ⇒ every CI checkout running `ckc check` keeps `fetch-depth: 0`. Firing = check-mutants `fork-date-invalid` (`0000-99-99`), `fork-date-rewritten` (a later date), `pristine-history-unread` (an unusable clex import commit beside a digest flip → the digest law fires).
