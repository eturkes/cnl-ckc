# Contract rev-k — review UI contrast, count nouns, certifier variant ordinal

Tier: kernel (spec `rust/ckc-spec/src/ui.rs` + `emit.rs`; kernel K5 + M6 mirrors; shell untouched). Gate: `just gate` (verify `--no-cheating`, mobile, check, certify). User ruling (REV AskUserQuestion): "Fund all three now (Recommended)".

## Predicates

- P1 contrast. On every rendered page: `main.hl-click pre.prose` text, `main.hl-click pre.prose .kw` and the underline of an unpicked mark under `main.hl-click` = `#4b5563` (7.56:1 on #ffffff); the `input[type="text"], textarea` border = `#4b5563`. Every other stylesheet byte is unchanged. Check: a computed WCAG relative-luminance test over the emitted CSS (text rules ≥ 4.5:1, the form-field boundary ≥ 3:1 against #ffffff), red on HEAD (2.539:1 + 1.238:1).
- P2 count nouns. The guideline + records tally reads `Reviewers recorded <n> decision(s) on <m> of <k> documents.` with `decision` iff n = 1; the empty tally reads `No decisions are recorded for the <k> document(s) in this guideline.` with `document` iff k = 1. Two copy rows append at the registry end (index 294 `sp_decision_on_sp` = " decision on ", 295 `sp_document_in_this_guideline` = " document in this guideline."); the registry = 296 rows; K5 render fidelity + `copy_ok` theorems hold. Check: n ∈ {0, 1, 2} × k ∈ {1, 2} pages against an independent model; red on HEAD for n = 1 and k = 1.
- P3 variant ordinal. `certify-one` rejecting a nonground obligation names that obligation's own variant ordinal: `nonground_obligation(S, variant(K))`, K = the group's `k` (emit.rs `certify_doc`; kernel `m6_doc` ghost + exec). Check: reviewer-6's supplied-dump probe (one rule group, k = 1) → `variant(1)`; red on HEAD (`variant(0)`).

## Out of scope

Other `#e5e7eb` borders (decorative separators), any other copy, every non-diagnostic certifier output.

## Migrations

Golden pages: one idempotent script under `.agent/archive/migrations/` re-pins every page whose bytes change (stylesheet lines + the two tally forms), replayable from a clean base; the k5-sound generators' asserted registry size 294 → 296; `.agent/archive/generators/replay` identical after regeneration.
