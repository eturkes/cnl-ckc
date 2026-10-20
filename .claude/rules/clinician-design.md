---
paths:
  - "README.md"
  - "docs/**"
  - "guidelines/*/README.md"
  - "rust/ckc/src/ui/**"
  - "rust/ckc-spec/src/ui.rs"
  - "rust/ckc/src/check/pipeline_release.rs"
  - "tests/ui/**"
  - "tests/copy/**"
---

# Clinician design law

This project's human surfaces = clinician-facing in a conservative clinical setting → calm print-like craft: neutral high-contrast palette, color encodes state only (muted, colorblind-safe), typographic hierarchy + real tables, whitespace over boxes, sentence case, absolute ISO dates, print-clean pages, phone-width pages (a wide table scrolls inside its own box, never the page), separators from the material (section paths ` > `, the coverage table's own mark; the document nav's links spaced, no glyph; labels with a colon; the tab title = the page title alone; breadcrumbs ` / `); a word with a table reading = a button styled as its ACE word with a thin link-blue underline, selection by underline thickness + `aria-pressed`, its reading in one line under the ACE box (never a tooltip, never hover-only). AI-slop banned: gradients/glassmorphism/animation, decorative icons + emoji, dashboard stat-card walls, functionless chrome (avatars, bells, settings, teaser affordances), exclamatory/marketing copy, invented acronym expansions. Technical plumbing (digests, internal ids, raw schema functors) stays out of visible copy — URLs, hidden fields + ledgers carry it; status/staleness/limits read as plain language. State capabilities + epistemic scope honestly (prepared questions, bounded search, reports what the loaded guidelines state — not clinical advice). Enforcement: every UI unit carries copy/design lint gates (banned-token scans over emitted strings) + design-system invariants.
