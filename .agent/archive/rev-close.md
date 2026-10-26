# MAINTAIN — release: code review of the implementation (closed)

Routed request + rulings as they stood in `.agent/spec.md` `Phase` (the pre-release request + its rulings stay there):

MAINTAIN — release: code review, formula-rule singleton warnings, Skolem role entities, first release line, review, release (Tasks REV1–REV5, SGW, SKO, RC1, PR8–PR10)
> The below is applicable for the planned pre-release and full release.
> User rulings (REV AskUserQuestion): kernel unit "Fund all three now (Recommended)"; compendium sweep counts "If option 2 is the more correct option, though at the cost of more work, then option 2. Otherwise, option 1."; README rewrite "Option 1. And if the corpus-side census can be fixed before this pre-release and release, then do so now."
> Met when: your finish line holds — for the work it covers, every check-set row in `.agent/review.md` carries a verdict, every finding is fixed with a green acceptance check, blocked on me as a `.agent/deferred.md` row, or routed (`route.md` `Met when`), and the full gate command passes on a clean working tree at the closing commit.

Check set = `.agent/archive/review-rev.md`, fixed at 824921de before the first read: 245 rows (24 baseline + 65 lens × module + 24 spec-fit + 132 conformance) → 167 pass, 75 fail→fixed, 3 info→deferred (SGW, SKO, the formula cycle scan row). Slices: reviewer-1..10 (tables = leads; MAIN ruled each row on its own check) + MAIN's baseline (gate record, every runnable `Artifacts` entry, the REFERENCE § Releasing dry run). MAIN findings: M-F1 (tally "1 decisions"), M-F2 (static `ckc` bytes depended on the host linker), M-F3 (reading-probe race, reviewer-2's lead confirmed).

Tasks closed:
- [x] 824921de REV1 code review check set committed before the first read.
- [x] REV2 baseline rows B-0..B-23 (fixes: B-12 → C14 444ca1f7, B-15 → REV-V c0a773f3; B-23 = the close commit's CI run).
- [x] REV3 lens pass: 216 lens × module, spec-fit + conformance rows from reviewer-1..10, each finding fixed in its class unit below.
- [x] c0a773f3 REV-V Verus release/0.2026.10.26.d17fbd0 (B-15; N17 #3155–#3157 fixed upstream): bump, canary 32 cases, scans.
- [x] ee406205 REV-S compendium sweep manifests = live counts, gated in `ckc check` + a planted mutant (user ruling: option 2).
- [x] 0349764f REV-K UI + certifier kernel unit (user ruling: "Fund all three now"): contrast #4b5563 (dimming + field borders), singular count nouns (copy rows 294–297), certify-one names the obligation's own `variant(K)`, the standing contrast census `rust/ckc/tests/ui_contrast.rs`; contract `archive/contracts/rev-k.md`; consultant-1, tester-1 (diff-blind suite rev_k.rs, tag `archive/rev-k-tester-1`), reviewer-11 (7 pass + R11-F1 fixed in the unit).
- [x] REV4 user questions: one `AskUserQuestion` (three rulings above); no later find needed the user.
- [x] REV5 close = the close commit: ledger archived, ticked rows cleared, rules upkeep (every backticked repo path + cited tag in `.claude/rules/*` + spec resolves; REV's L rows covered each rule bullet), full gate via `just ci rev-close`.

Fix classes (one commit each, `just ci <class>` green on each): C1 64a5efbd stdout/argv products · C2 72d60c87 certify SWIPL + version guard · C3 e14b8e96 400-response headers · C4 89e6d760 shared DevTools-pipe client · C5 fa0ca9dc probe exit statuses in recipes · C6 00a0ddd0 release docid collision · C7 90a19e9b compiler frame guard · C8 b6ce5401 fork notice dates + write-once · C9 6b606c57 trust `--write` = audit set · C10 1c6617e9 kernel allow · C11 6d18af99 dead spec/contract definitions · C12 c3dff0ff compendium issuer guards · C13 7991fa34 agent law + recipe checks · C14 444ca1f7 `ckc --help` + `_timed` total + rust-lld static link · D1 30e1452a counts + paths · D2 ba9173ae law/doc text · D3 477e5fd8 README-dist load cwd + § Releasing · D4 4f3ccde0 `just --list` docs · D5 9f5704b2 CDC README authoring rule · D6 30ba0df9 lexicon scratch recipes retired · T1 d6ee1ec1 s1_codec a05 · T2 c88fc30b compiler-model dedup · T3 3dc87594 socket-suite startup deadlines · T4 f43a7d94 reading-probe focus race · S ee406205 · K 0349764f.

Routed: REV-P (projection-notes census against the authoring principle; user ruling: before the pre-release) → its own corpus session beside SKO (`ops.md` Corpus loop), Tasks REV-P; RC1 waits on it. Deferred with acceptance checks (`.agent/deferred.md`): CI bootstrap repetition (R4-N1), run-wide test caches (R5-R2), Kani accept gains (R4-N2, in the Kani row).

Outcome: every finding of the review is fixed at its root cause with a red-then-green check, routed (REV-P) or deferred with its acceptance check; the reviewer pages meet accessible contrast under a standing census, count nouns agree with their numbers, and certify-one names the failing variant; the remote `main` weekly `updates` job stays red (`1 drift` = the Verus pin) until the user pushes `main` with REV-V.
