# Contract rev-k — review UI contrast, count nouns, certifier variant ordinal

Tier: kernel (spec `rust/ckc-spec/src/ui.rs` + `emit.rs`; kernel K5 + M6 mirrors; shell untouched). Gate: `just gate` (verify `--no-cheating`, mobile, check, certify). User ruling (REV AskUserQuestion): "Fund all three now (Recommended)".

## Predicates

- P1 contrast. On every rendered page: `main.hl-click pre.prose` text, `main.hl-click pre.prose .kw` and the underline of an unpicked mark under `main.hl-click` = `#4b5563` (7.56:1 on #ffffff); the `input[type="text"], textarea` border = `#4b5563`. Every other stylesheet byte is unchanged. Check: a computed WCAG relative-luminance test over the emitted CSS (text rules ≥ 4.5:1, the form-field boundary ≥ 3:1 against #ffffff), red on HEAD (2.539:1 + 1.238:1).
- P2 count nouns. Every count noun takes its number: the guideline + records tally `Reviewers recorded <n> decision(s) on <m> of <k> document(s).` (`decision` iff n = 1, `document` iff k = 1), the empty tally `No decisions are recorded for the <k> document(s) in this guideline.` (`document` iff k = 1; ui.rs:784–789), the document page's `Compiled Prolog (<l> line(s))` (`line` iff l = 1; ui.rs:1875–1877). Four singular rows append at the registry end (294–297; RD 286 + SUP4 287–293 fixed); registry = 298; each new name has a named use outside the table; K5 render fidelity + `copy_ok` hold (kernel k5_model.rs:201–235, k5_sound_summary.rs:94–137, k5_document.rs + k5_sound_document.rs concatenation trees). Check: guideline + records routes over n ∈ {0,1,2} × k ∈ {1,2} × m ∈ {1,2} (two decisions on one document stay two decisions) + line counts 0/1/2 against an independent model; byte identity outside the selected suffixes + P1's CSS; red on HEAD for every singular case.
- P3 variant ordinal. `certify-one` rejecting a nonground obligation names that obligation's own variant: spec `replay::variant(o)` (= `variant(o.k)`, o = the selected first nonground obligation) in place of `variant_slot(0)` (emit.rs:4812); kernel ghost m6_doc.rs:191 the same; exec m6_doc.rs:254 copies the built `oa[2]` wrapper (as it copies the sentence `oa[1]`). Check: k = 1 and k = 2 nonground groups, a later-sentence k = 1 after ground obligations, both variants nonground → the first, a ground accepting control; clause mismatch still wins before the nonground check; red on HEAD (`variant(0)`).

## Out of scope

Other `#e5e7eb` borders (decorative separators), any other copy, every non-diagnostic certifier output.

## Migrations

- CSS lives in 227 committed fixture carriers under tests/ui/ (107 `.html` + 120 `.golden`, HTTP bodies + error pages included): one idempotent script under `.agent/archive/migrations/` (explicit root; anchored CSS + UI-text edits outside corpus/`pre` blocks; exact first-pass inventory; second pass byte-identical; out-of-scope sentinels unchanged).
- Frozen oracles keep their historical identities + SHA pins: UR (`tests/ur-ui/legacy.rs` expected-byte model + replay chain UR → SEC5 → SEC7 → RD gains the REV-K transform), RD (`rd_disclosure.rs` historical base + six SHA pins), SUP4 (`tests/sup4-ui/baseline.sha256` 28 pins): each old-baseline comparison applies the narrow inverse REV-K transform; the new suite pins the new bytes.
- Generators: chrome, encoding, literals + registry regenerate (generators.tsv 2–6; size asserts in k5-sound-literals:15 + k5-sound-registry:14 → 298); patterns replay unchanged; `copy_registry.rs:346` + `rd_disclosure.rs:1046–1057` row counts; ui.md + rust.md registry counts; trust-audit re-pin after format.
- Close: diff-blind tester suite (contract only), affected UI/UR/RD/SUP4/copy-registry/certify-one suites, generator replay, `just verify` (selector-free, `--no-cheating`), `just mobile`, `just gate`, live `ckc ui serve` + certify-one smoke; reviewer on the kernel diff before its commit.
