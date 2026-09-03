# M5.1 mutation campaign + determinism design

Scope = PREP only. `rust/ckc-kernel/src/align_impl.rs` stayed unread. Phase 2 chooses exact implementation sites mechanically inside MAIN's named worktree.

## Scoring law

- G1 = `cargo verus verify --workspace --locked --offline -- --no-cheating`.
- G2 = `ckc trust-audit`. Current baseline = 15 word tokens + 5 raw tokens + six whitespace-collapse obfuscation checks; green meter includes `assumes=32`.
- G3 = `python3 -P .scratch/m5u1/diff/run_diff.py` with `RUST_BIN` set to the mutant build.
- G4 = `python3 -P .scratch/m5u1/suite/runner.py` with `RUST_BIN` set to the mutant build.
- F1 impl mutant: PASS iff G1 is nonzero. G1 survival + changed runtime = CRITICAL spec gap. G1 survival + byte-identical G3/G4 behavior = equivalent mutant. A G1 kill alone is sufficient even when the mutant might be semantically correct; Verus is sound, not complete.
- F2 spec mutant: PASS iff G1 is nonzero, or G1 is zero and G3/G4 expose the stated divergence. Survival through G1+G3+G4 = unbound clause finding. Severity follows the mutated law.
- C contract mutant: expected G1 zero because the obligation is weaker. Record the unpinned domain; these are binding-necessity demonstrations, not ordinary kills.
- G2 is custody, not a semantic mutation oracle for F2/C: any trusted-file byte edit trivially yields `trusted-surface drift`. Record that control once, then exclude it from F2/C kill scoring. F1 files are outside the spec manifest, so G2 must remain green for F1.
- Every mutant starts from the same committed baseline, changes one semantic operator, uses a private clean `CARGO_TARGET_DIR`, captures stdout/stderr/rc, then restores the mutated file. A compile/type failure is `invalid mutant`, not a kill, unless the mutation is explicitly a compile-surface probe.
- Mutation hygiene: source edits must preserve the exact scanner-visible occurrence census across the 15 word, 5 raw, and 6 split-token families, including comments. Check the token delta before gates. A scanner trip is a confounded/invalid mutant, never its intended G1 kill; replace it with a token-clean equivalent operator.

## Campaign table — F2 file/row laws (checkpoint 1: 14/14)

The current-text anchor quotes at most one line. `G1 red` means a nonzero verification verdict whose useful shape is an unproved exact-outcome postcondition, not a parser/compiler failure. Each fallback witness must be present in G4 unless an existing G3 pin covers it.

| mutant-id | family | file + clause anchor | mutation: exact before → after | expected kill gate + failure shape | rationale / R3 witness |
|---|---|---|---|---|---|
| S01-final-lf | F2 | `ckc-spec/src/align.rs:first_violation` — `if align.len() == 0 \|\| align.last() != '\n' {` | `align.len() == 0 \|\| align.last() != '\n'` → `align.len() == 0` | G1 red; if green, G3 pin 1 reports CLI accepted/reclassified missing-LF input | trailing-LF law; nonempty `0\tsrc\t0\tA` must be `missing trailing newline` |
| S02-strip-one | F2 | `first_violation` — `let body = align.drop_last();` | `align.drop_last()` → `align` | G1 red; if green, G4 valid two-row file becomes final empty-row field error | strip exactly one LF, neither zero nor all; normal valid file retains LF as a row separator |
| S03-empty-file | F2 | `first_violation` — `if body.len() == 0 {` | `body.len() == 0` → `false` | G1 red; if green, G3 pin 2 changes `empty file` to row-1 field error | `"\n"` empty-file law after one-LF stripping |
| S04-row-physical-order | F2 | `rows_violation` — `None => rows_violation(rows.drop_first(), n + 1, src, ace),` | `rows.drop_first()` → `rows.drop_last()` | G1 red; if green, G4 row 1 valid + row 2 invalid is accepted because row 1 is rechecked and row 2 is dropped | physical row order and first failing row |
| S05-fields-exactly-four | F2 | `row_violation` — `if fields.len() != 4 {` | `fields.len() != 4` → `fields.len() < 4` | G1 red; if green, G4 five-field row is accepted instead of `expected 4 tab-separated fields` | row check 1: exactly four fields, not at least four |
| S06-group-canonical-check | F2 | `row_violation` — `} else if !is_canonical_decimal(fields[0]) {` | `!is_canonical_decimal(fields[0])` → `false` | G1 red; if green, G3 pin 4 accepts/reclassifies group `01` | row check 2: group canonical decimal |
| S07-start-canonical-check | F2 | `row_violation` — `} else if !is_canonical_decimal(fields[2]) {` | `!is_canonical_decimal(fields[2])` → `false` | G1 red; if green, G3 pin 5 accepts/reclassifies start `00` | row check 3: start canonical decimal |
| S08-empty-span-check | F2 | `row_violation` — `} else if fields[3].len() == 0 {` | `fields[3].len() == 0` → `false` | G1 red; if green, G3 pin 7 accepts/reclassifies an empty span | row check 4: nonempty span |
| S09-side-vocab-check | F2 | `row_violation` — `} else if fields[1] != "src"@ && fields[1] != "ace"@ {` | append `&& fields[1] != "mid"@` to the predicate | G1 red; if green, G3 pin 6 no longer emits `side must be src or ace` for `mid` | row check 5: exact lowercase `src\|ace`; unknown rows must not disappear from both side collections |
| S10-side-text-selection | F2 | `row_violation` — `let text = if fields[1] == "src"@ {` | complete selection `if src { src } else { ace }` → `if src { ace } else { src }` | G1 red; if green, G4 distinct `src="A"`, `ace="B"` valid pair reports span mismatch | row semantic 6: validate each span against its named side |
| S11-range-end-equality | F2 | `row_violation` — `if end > text.len() {` | `end > text.len()` → `end >= text.len()` | G1 red; if green, G4 one-code-point span ending exactly at `len(text)` is rejected | row check 7: reject only end beyond text; equality is valid |
| S12-span-equality-check | F2 | `row_violation` — `} else if text.subrange(start, end) != fields[3] {` | `!=` → `==` | G1 red; if green, G3 pin 9 accepts mismatch while valid rows emit `span does not match...` | row check 8: exact text subrange equality |
| S13-order-group-before-start | F2 | `row_violation` — `} else if !is_canonical_decimal(fields[0]) {` | swap the complete adjacent GroupCanonical and StartCanonical `else if` arms | G1 red; if green, G4 row `01\tsrc\t00\tA` changes group error to start error | row-check order: group canonical outranks start canonical |
| S14-order-empty-before-side | F2 | `row_violation` — `} else if fields[3].len() == 0 {` | swap the complete adjacent EmptySpan and SideVocab `else if` arms | G1 red; if green, G4 row `0\tmid\t0\t` changes empty-span error to side error | row-check order: empty span outranks side vocabulary |

## Campaign table — F2 global/decimal laws (checkpoint 2: 12/12; cumulative 26)

| mutant-id | family | file + clause anchor | mutation: exact before → after | expected kill gate + failure shape | rationale / R3 witness |
|---|---|---|---|---|---|
| S15-row-one-based | F2 | `first_violation` — `match rows_violation(rows, 1, src, ace) {` | `rows_violation(rows, 1, src, ace)` → `rows_violation(rows, 0, src, ace)` | G1 red; if green, G4 any bad first row emits `row 0:`; G3 pin 3 also detects it | row detail ordinals start at 1 and use canonical decimal rendering |
| S16-group-set-polarity | F2 | `first_violation` — `if groups_of(srcs) != groups_of(aces) {` | `!=` → `==` | G1 red; if green, G4 a valid matched group emits `every group needs...`, while an unmatched group escapes this check | exact src/ace group-set equality |
| S17-group-set-not-count | F2 | `first_violation` — `if groups_of(srcs) != groups_of(aces) {` | full predicate → `srcs.len() != aces.len()` | G1 red; if green, G4 one group with two disjoint src spans and one ace span is wrongly rejected | equality is over distinct group sets; side multiplicities may differ |
| S18-src-overlap-check | F2 | `first_violation` — `} else if has_overlap(srcs) {` | `has_overlap(srcs)` → `false` | G1 red; if green, G3 pin 10 no longer emits `overlapping src spans` | src-side pairwise overlap rejection |
| S19-ace-overlap-check | F2 | `first_violation` — `} else if has_overlap(aces) {` | `has_overlap(aces)` → `false` | G1 red; if green, G4 overlapping-ace witness is accepted/reclassified | ace-side pairwise overlap rejection; covers the twelfth unpinned error detail |
| S20-order-groups-before-overlap | F2 | `first_violation` — `if groups_of(srcs) != groups_of(aces) {` | swap the complete NotBothSided and OverlapSrc branches | G1 red; if green, G4 unmatched groups + src overlap changes `every group needs...` to `overlapping src spans` | whole-file precedence: both-sidedness outranks overlap |
| S21-order-src-before-ace-overlap | F2 | `first_violation` — `} else if has_overlap(srcs) {` | swap the complete OverlapSrc and OverlapAce branches | G1 red; if green, G4 both sides overlapping changes src detail to ace detail | whole-file precedence: src overlap outranks ace overlap |
| S22-touching-is-legal | F2 | `has_overlap` — `&& spans[i].start < spans[j].end && spans[j].start < spans[i].end` | first `<` → `<=` | G1 red; if green, G4 adjacent `[0,1)` + `[1,2)` spans report overlap | half-open pairwise intersection; touching spans are legal |
| S23-distinct-row-instances | F2 | `has_overlap` — `0 <= i < spans.len() && 0 <= j < spans.len() && i != j` | `i != j` → `true` | G1 red; if green, G4 one span per side self-intersects and valid input reports overlap | compare distinct row instances; duplicates remain distinct and must overlap each other |
| S24-decimal-nonempty | F2 | `is_canonical_decimal` — `&&& s.len() > 0` | coherent two-site weakening: `s.len() > 0` → `true`; final `(s.len() == 1 \|\| s[0] != '0')` → `(s.len() == 0 \|\| s.len() == 1 \|\| s[0] != '0')` | G1 red; if green, G4 empty group field is accepted as numeric zero/reclassified | canonical decimal is nonempty; second edit keeps the mutant total instead of indexing empty `s` |
| S25-decimal-no-leading-zero | F2 | `is_canonical_decimal` — `&&& (s.len() == 1 \|\| s[0] != '0')` | complete clause → `&&& true` | G1 red; if green, G3 pins 4/5 accept or reclassify `01`/`00` | no leading zero except the sole digit `0` |
| S26-decimal-unbounded | F2 | `is_canonical_decimal` — `&&& all_ascii_digits(s)` | append `&&& dec_value(s) <= 18446744073709551615` after this clause | G1 red; if green, G4 matched group id `18446744073709551616` changes success to canonical-decimal error | arbitrary-precision decimal; no `u64` truncation or cap |

## Campaign table — F2 Unicode/model/render laws (checkpoint 3: 14/14; cumulative 40)

| mutant-id | family | file + clause anchor | mutation: exact before → after | expected kill gate + failure shape | rationale / R3 witness |
|---|---|---|---|---|---|
| S27-decimal-ascii-only | F2 | `is_ascii_digit` — `'0' <= c && c <= '9'` | `&&` → `\|\|` | G1 red; if green, G4 `+1` group/start stops producing canonical-decimal error | canonical digits are ASCII `0..9`, not any character accepted by a weakened bound |
| S28-code-point-span-length | F2 | `row_violation` — `let end = start + fields[3].len();` | apply exact U8 patch below: both parsed-end sites use `utf8_len(fields[3])` instead of `fields[3].len()` | G1 red; if green, G4 source `éx`, span `é`, start `0` changes success to mismatch/range; h-2 diacritic lane also diverges | offsets and lengths count Unicode code points, not UTF-8 bytes |
| S29-index-first-ace-sorted | F2 | `model_of` — `let order = groups_in_order(sort_raw(aces), Set::empty());` | `sort_raw(aces)` → `aces` | G1 red; if green, G4 valid ACE rows authored in reverse offset order produce reversed display indexes | dense indexes follow first ACE span after canonical sorting, not authored row order |
| S30-index-from-ace-not-src | F2 | `model_of` — `let order = groups_in_order(sort_raw(aces), Set::empty());` | `sort_raw(aces)` → `sort_raw(srcs)` | G1 red; if green, G4 two groups whose src and ACE orders disagree produces swapped indexes | dense index authority is the ACE side only |
| S31-first-ace-sort-key | F2 | `raw_le` — `\|\|\| a.start < b.start` | replace complete `(start,end,group)` comparator with `(group,start,end)` lexicographic comparator | G1 red; if green, G4 group ids `9` then `1` at increasing ACE offsets become group-id ordered | first-ACE order uses sorted `(start,end,group)`, with start as primary key |
| S32-dense-group-dedup | F2 | `groups_in_order` — `} else if seen.contains(spans[0].group) {` | `seen.contains(spans[0].group)` → `false` | G1 red; if green, G4 one group with two disjoint ACE spans yields count `2` instead of `1` | one dense index per distinct authored group; first ACE occurrence wins |
| S33-authored-ids-discarded | F2 | `to_out` — `index: index_in(order, spans[0].group),` | `index_in(order, spans[0].group)` → `spans[0].group` | G1 red; if green, G4 valid group ids `7` and `42` leak as output indexes instead of `0,1` | authored group ids are discarded after dense reindexing |
| S34-count-distinct-groups | F2 | `model_of` — `count: order.len() as int,` | `order.len()` → `aces.len()` | G1 red; if green, G4 one group with two ACE spans reports count `2` | `count` equals dense distinct-group count, not span count |
| S35-src-output-sorted | F2 | `model_of` — `src: sort_out(to_out(srcs, order)),` | `sort_out(to_out(srcs, order))` → `to_out(srcs, order)` | G1 red; if green, G4 reverse-authored src rows remain reversed in CLI projection | each side is independently canonicalized, regardless of authored row order |
| S36-ace-output-sorted | F2 | `model_of` — `ace: sort_out(to_out(aces, order)),` | `sort_out(to_out(aces, order))` → `to_out(aces, order)` | G1 red; if green, G4 reverse-authored ACE rows remain reversed even though indexes are dense | ACE output list also uses per-side canonical order |
| S37-per-side-sort-key | F2 | `out_le` — `\|\|\| a.start < b.start` | replace complete `(start,end,index)` comparator with `(index,start,end)` lexicographic comparator | G1 red; if green, G4 src positions whose display-index order disagrees with offsets become index-sorted | per-side lists sort by `(start,end,index)`, with start primary |
| S38-render-one-byte | F2 | `render` — `Violation::EmptyFile => "empty file"@,` | `"empty file"` → `"empty pile"` (`f`→`p`, one byte) | G1 red; if green, G3 pin 2 reports exact-byte divergence | all 12 detail strings are byte-exact; one-character drift must be visible |
| S39-row-decimal-multidigit | F2 | `dec_str` — `if n < 10 {` | `n < 10` → `n <= 10` | G1 red; if green, G4 invalid row 10 renders `row 9:` because `digit_char(10)` falls through | row numbers use ordinary unpadded canonical decimal beyond pin-only row 1 |
| S40-secondary-sort-key-deadness | F2 | `out_le` — `\|\|\| (a.start == b.start && a.end == b.end && a.index <= b.index)` | complete final disjunct → `\|\|\| false` | G1 red is a valid kill; if G1+G3+G4 all green, classify LOW dead/redundant clause only after proving wellformed nonempty nonoverlap makes equal `(start,end)` impossible | per-side `(start,end,index)` text is bound; this mutant deliberately distinguishes an observable gap from an invariant-redundant tie-break |

Exact U8 patch for S28 (one coherent unit mutation): insert after `digit_value`, then replace both `dec_value(fields[2]) + fields[3].len()` and `start + fields[3].len()` with the same expressions using `utf8_len(fields[3])`.

```rust
pub open spec fn utf8_width(c: char) -> int {
    let n = c as int;
    if n <= 0x7f { 1 } else if n <= 0x7ff { 2 } else if n <= 0xffff { 3 } else { 4 }
}
pub open spec fn utf8_len(s: Seq<char>) -> int
    decreases s.len(),
{
    if s.len() == 0 { 0 } else { utf8_len(s.drop_last()) + utf8_width(s.last()) }
}
```

## Campaign table — F1 implementation operators (checkpoint 4: 16/16; cumulative 56)

Phase-1 anchors are trusted-spec clauses, not implementation excerpts. Phase 2 may search the uninspected file mechanically for the matching operation, record one exact byte-level site, then apply only the stated operator. If no single site implements the law, mark `not-applicable`; do not synthesize a broad rewrite. Every valid non-equivalent F1 application expects G1 red. On unexpected G1 green, build once and run G3+G4: changed behavior = CRITICAL spec gap; identical behavior = equivalent mutant, no finding.

| mutant-id | family | file + clause anchor | mutation: exact operator before → after; exact site selected phase 2 | expected kill gate + failure shape | rationale / R3 law |
|---|---|---|---|---|---|
| I01-final-lf-comparison | F1 | `ckc-kernel/src/align_impl.rs`; spec `first_violation` — `align.last() != '\n'` | matching comparison `!=` → `==` | G1 red: exact-outcome postcondition cannot prove missing-LF branch | comparison flip; trailing-LF predicate |
| I02-range-comparison | F1 | impl; spec `row_violation` — `if end > text.len() {` | matching range operator `>` → `>=` | G1 red: end-equal witness disagrees with spec | comparison flip; equality boundary is valid |
| I03-end-boundary-plus-one | F1 | impl; spec `row_violation` — `let end = start + fields[3].len();` | matching end expression `start + span_len` → `start + span_len + 1` | G1 red: model/range result differs | boundary `+1`; code-point half-open end |
| I04-strip-boundary-zero | F1 | impl; spec `first_violation` — `let body = align.drop_last();` | terminal-removal count `1` → `0` at the matching slice/pop site | G1 red: valid rows retain the terminal separator | boundary `-1` removal changed to zero; strip exactly one LF |
| I05-row-ordinal-off-by-one | F1 | impl; spec `first_violation` — `match rows_violation(rows, 1, src, ace) {` | initial row ordinal `1` → `0` | G1 red: rendered error index differs | index off-by-one; 1-based row details |
| I06-dense-index-off-by-one | F1 | impl; spec `index_in` — `1 + index_in(order.drop_first(), g)` | assigned dense index `i` → `i + 1` at output construction | G1 red: successful model view differs | index off-by-one; dense `0..count-1` |
| I07-side-payload-swap | F1 | impl; spec `row_violation` — `let text = if fields[1] == "src"@ {` | matching branch payloads `src ↔ ace`, predicates unchanged | G1 red: side-specific text-match result differs | side swap; src/ace validation authority |
| I08-side-collection-swap | F1 | impl; spec `model_of` — `let srcs = side_spans(rows, "src"@);` | destinations for one parsed valid row `srcs ↔ aces` | G1 red: group sets/model lists differ | side swap; result-side custody |
| I09-group-dedup-drop | F1 | impl; spec `groups_in_order` — `seen.contains(spans[0].group)` | bypass seen-hit branch: `if seen` → `if false` (or exact equivalent branch guard) | G1 red: repeated ACE group inflates count/index order | dedup drop; first occurrence per group |
| I10-early-return-after-row | F1 | impl; spec `rows_violation` — `None => rows_violation(rows.drop_first(), n + 1, src, ace),` | after first clean row, return the current success accumulator instead of continuing | G1 red: later invalid row contradicts outcome | early-return insertion; physical all-row scan |
| I11-group-set-check-drop | F1 | impl; spec `first_violation` — `if groups_of(srcs) != groups_of(aces) {` | matching mismatch guard → `false` | G1 red: one-sided group reaches success/overlap path | early-return/check deletion; exact set equality |
| I12-overlap-boundary | F1 | impl; spec `has_overlap` — `spans[i].start < spans[j].end` | matching intersection `<` → `<=` | G1 red: touching spans become illegal | comparison flip; half-open intersection |
| I13-raw-sort-key-component-drop | F1 | impl; spec `raw_le` — `\|\|\| (a.start == b.start && a.end < b.end)` | remove one comparator component from the implementation's `(start,end,group)` key | G1 red if proof depends on the exact key; G1 green + identical G3/G4 = equivalent, no finding | required sort-key drop class; secondary-key observability is constrained by nonoverlap |
| I14-raw-sort-key-swap | F1 | impl; spec `raw_le` — `\|\|\| a.start < b.start` | swap key component order `(start,end,group)` → `(group,start,end)` | G1 red: two groups with id/offset order disagreement change indexes | sort-key component swap; first ACE span order |
| I15-output-sort-key-drop | F1 | impl; spec `out_le` — `\|\|\| a.start < b.start` | drop primary `start` component, retaining the remaining implemented key | G1 red: side output order can become index/end ordered | sort-key component drop; per-side `(start,end,index)` |
| I16-render-byte-substitution | F1 | impl; spec `render` — `Violation::OverlapAce => "overlapping ace spans"@,` | one literal byte `ace` → `axe` (`c`→`x`) | G1 red: error vector view differs byte-for-byte | exact detail rendering; twelfth reachable string |

## Campaign table — contract weakening demonstrations (checkpoint 5: 3/3; grand total 59)

These mutate only `rust/ckc-kernel/src/contract.rs`. Current anchor = `r@ == ckc_spec::align::align_outcome(align@, src@, ace@),`.

| mutant-id | family | file + clause anchor | mutation: exact before → after | expected gate shape | binding necessity demonstrated |
|---|---|---|---|---|---|
| C01-drop-binding | CONTRACT | `contract.rs:align_check` — exact equality above | full equality → `true,` | G1 green; G2 emits trusted-surface drift but is excluded from semantic score | all return behavior becomes unpinned at the public API despite verified delegation |
| C02-success-only | CONTRACT | same | equality → `ckc_spec::align::wellformed(align@, src@, ace@) ==> (r@ == ckc_spec::align::align_outcome(align@, src@, ace@)),` | G1 green | invalid-input variant, precedence, row number, and all 12 error bytes become unpinned |
| C03-error-only | CONTRACT | same | equality → `!ckc_spec::align::wellformed(align@, src@, ace@) ==> (r@ == ckc_spec::align::align_outcome(align@, src@, ace@)),` | G1 green | successful count, dense indexes, side lists, and sort order become unpinned |

## Coverage census

- Total = 59 = 40 F2 + 16 F1 + 3 contract demonstrations. Scored semantic mutants = 56/59; demonstrations = 3/59.
- Required file laws: trailing LF S01/I01; strip one LF S02/I04; empty file S03; physical row order S04/I10; row 1-basing S15/I05; multi-digit row rendering S39.
- Eight row semantics: field count S05; group canonical S06; start canonical S07; empty span S08; side vocabulary S09; named-side text selection S10/I07; range S11/I02; span equality S12. Adjacent precedence = S13+S14.
- Global laws: exact group sets S16+S17/I11; src/ace overlap S18+S19; precedence S20+S21; touching legal S22/I12; distinct row instances S23.
- Canonical decimal: nonempty S24; leading zero S25; unbounded S26; ASCII-only S27.
- Code-point units = S28/I03.
- Success model: first-ACE sorted order S29-S31/I14; dedup+dense order S32/I09; authored ids discarded S33; distinct count S34/I06; per-side sorting/key S35-S37/I13+I15; exact render bytes S38/I16.
- Required F1 operator classes: comparison flip I01/I02/I12; boundary ±1 I03/I04; sort-key drop/swap I13-I15; dedup drop I09; early return I10; side swap I07/I08; index off-by-one I05/I06. Planned applications = 16, exceeding the required 10.

### G4 witness binding at execution

Final G4 = 94 generated cases, baseline `suite: 94/94 passed`. Required fallback witnesses arrived before execution: `091-set-equality-multiplicity` binds S17, `092-ace-order-opposes-group-id` binds S31, and `093-row-ten-fields` binds S39. `094-unbounded-group-over-4300-digits` extends S26's unbounded-decimal surface beyond Python's stock digit guard. S40 remains the declared invariant-redundancy probe. All 40 F2 mutants died at G1, so no F2 fallback run was needed.

## Determinism protocol

### D1 — Verus reproducibility, N=3 clean runs

1. Use the same baseline SHA and immutable toolchain/vendor bytes for all three runs.
2. Raw empty targets are not a valid `--no-cheating` boundary: they rebuild vendored `vstd`, whose admitted library specs fail before workspace verification. Use three private copy-on-write clones of MAIN's verified target cache, point the worktree-only Cargo vendor path at PRIMARY's byte-identical `rust/vendor`, then run `cargo clean -p ckc-spec` and `cargo clean -p ckc-kernel` inside each clone. This forces all 95 workspace obligations while retaining dependency artifacts.
3. From `<WT>/rust`, run the exact fixed gate in each clone: `cargo verus verify --workspace --locked --offline -- --no-cheating`. Restore `.cargo/config.toml` after the campaign.
4. Capture stdout, stderr, and rc separately; capture rc immediately. Extract exactly one verdict tuple `(rc, verified_count, error_count)` from `verification results:: <V> verified, <E> errors`. Missing or multiple verdict lines fail the probe.
5. Pass iff all 3/3 tuples are byte-identical and equal `(0, baseline_V, 0)`. Preserve full logs for diagnosis; wall time, absolute paths, and progress text are not part of the comparison.
6. Run the existing negative control once in a fourth clean target: an `ensures false` proof must return nonzero. This prevents a silently bypassed verifier from satisfying reproducibility.

### D2 — behavioral binary determinism, 2 builds × 2 runs

1. Allocate two absent targets `.../det/d2-build-{a,b}`. In each, run the exact build gate: `cargo build --release --locked --offline`.
2. The runner prints `RUST_BIN` in its first line. Use one stable execution path `.../det/run-bin/ckc`: copy build A there, set `RUST_BIN` to that path, and run `python3 -P .scratch/m5u1/suite/runner.py` twice; then atomically replace the same path with build B and run twice. Capture A1/A2/B1/B2 stdout, stderr, and rc. The worktree-local runner may overwrite its own `last-run.txt` sequentially.
3. Pass iff all 4/4 rc values are zero, all 4/4 stdout files are byte-identical (including the path-bearing first line and final LF), and all 4/4 stderr files are byte-identical. Use `cmp`, not normalized text. Required terminal line after fixture completion = `suite: 94/94 passed`.
4. Run G3 once per source binary through the same stable execution path as a supplementary seam check; both must end `diff: 0 divergences` with byte-identical output after any harness-declared corpus-count field is fixed to the same HEAD.
5. Record both source-binary SHA-256 values as diagnostics before copying. Different executable bytes do not fail D2 when the 4/4 observable suite transcripts match; this probe's contract is behavioral determinism across independently built binaries.

### D3 — trust-audit meter + file-order independence

Static expectation from `rust/ckc/src/trust.rs`: `walk_rs` collects each directory then calls `entries.sort()`; member arrays are fixed; escape/dependency/manifest maps are `BTreeMap`; required manifest paths are sorted. The scanner applies fixed 15-word/5-raw arrays plus six per-file whitespace-collapse comparisons. No meter field depends on mtime or discovery order.

1. Build one baseline release binary. Run `ckc trust-audit` three times without file changes. Capture stdout, stderr, rc. Pass baseline iff 3/3 rc values are zero, stdout meter bytes are identical and contain `assumes=32`, and stderr is empty in all 3/3 runs.
2. Metadata probe: record SHA-256 for every member `.rs` file; touch mtimes in reverse lexical order; rename every `.rs` in each directory to unique non-`.rs` temporary names, then restore the original names in reverse lexical order. Do not invoke the audit while names are temporary. Prove before/after content hashes identical and `git diff --exit-code` clean. Rerun G2; require meter bytes identical to baseline.
3. Creation-order probe: under `rust/ckc/src/.det-order-probe/`, create eight empty `.rs` files `a.rs`…`h.rs` in ascending order and capture G2 output A; delete/recreate the same names+bytes in descending order and capture output B. Require A/B rc zero and stdout/stderr byte-identical. Relative to the no-probe baseline, the shell-file count increases by exactly 8 while `assumes=32` stays fixed; compare A to B, not A to baseline.
4. Remove the probe directory, restore original mtimes only if the phase-2 evidence contract requires them, and rerun G2. Require exact baseline meter bytes and clean tracked status.
5. Any order-sensitive violation ordering also fails D3: repeat the creation-order probe with two benign unallowlisted escape plants in distinct probe files, require rc1 and byte-identical stderr line order A/B, then remove both plants. This exercises sorted diagnostics, not only the green meter.

## Phase-2 cold-start prerequisites

1. MAIN supplies a named isolated worktree at the post-merge baseline SHA. Primary-tree bytes remain untouched. Record `<BASE_SHA>` and require clean tracked status before and after each mutant.
2. Execution used PRIMARY's read-only G3/G4 drivers with the worktree `RUST_BIN`. G3 pins+loader+corpus = `0 divergences`; G4's 94 generated cases, including 091-094, = `94/94 passed`.
3. Use the existing project-local toolchain from the primary repo as read-only state:

```bash
export BASE="$PWD"  # repo root
export WT=<MAIN_NAMED_WORKTREE>
export RUSTUP_HOME="$BASE/.toolchain/rustup"
export CARGO_HOME="$BASE/.toolchain/cargo"
export PATH="$BASE/.toolchain/verus-x86-linux:$CARGO_HOME/bin:$PATH"
cd "$WT"
```

4. Use a worktree-private target root for every run: `export CARGO_TARGET_DIR="$WT/.scratch/m5u1/mutation/targets/<RUN_ID>"`. The directory must not exist before the run. Do not share target/build state with MAIN or another agent.
5. Baseline gates, verbatim, with their required working directories:

```bash
cd "$WT/rust"
cargo verus verify --workspace --locked --offline -- --no-cheating
cargo build --release --locked --offline
ckc trust-audit
cd "$WT"
python3 -P .scratch/m5u1/diff/run_diff.py
python3 -P .scratch/m5u1/suite/runner.py
```

Operationally, set `RUST_BIN="$CARGO_TARGET_DIR/release/ckc"` for ordinary G3/G4 runs and invoke the same binary as `"$RUST_BIN" trust-audit` for G2. D2 alone copies each build to its stable `RUST_BIN` path. Baseline acceptance = G1 rc0/counts stable under `--no-cheating`; G2 rc0 byte-identical meter with `assumes=32`; G3 `0 divergences`; G4 `suite: 94/94 passed` rc0.

6. Materialize each mutation through an idempotent operator script keyed by mutant-id. Before running a gate, assert exactly the intended file changed, exactly one semantic operator differs, and the trust scanner's 15-word/5-raw/6-split occurrence census is unchanged. F1 scripts may read/search `align_impl.rs` only in phase 2; the review must not describe its design.
7. F1 flow: apply → G1. On G1 red, record kill and restore. On G1 green, build → G2 → G3 → G4, then classify CRITICAL behavior-changing survivor or equivalent behavior-preserving survivor.
8. F2 flow: apply → G1. On G1 red, record kill and restore. On G1 green, build → G3 → G4. Record G2's expected manifest-drift control once per trusted file, but never count it as a semantic kill.
9. CONTRACT flow: apply → G1 must stay green; record the unpinned domain. G2 manifest drift confirms custody only. No runtime divergence is expected because the body bytes remain unchanged.
10. Invalid-mutant rule: parser/type/borrow failure before proof obligations = revise the mechanical operator once or mark not-applicable. Never score compilation failure as a semantic kill.
11. Execution ledger row schema: `id | baseline_sha | actual file:line + one-line before | one-line after | changed-file census | G1 rc/V/E | G2 rc/meter | G3 rc/divergences | G4 rc/failures | classification | evidence paths`. Fill one row immediately after each mutant.
12. Restore mutated tracked files after every row and prove the baseline SHA's tracked bytes before the next mutant. Keep logs and targets under worktree-local `.scratch/m5u1/mutation/`; remove them after MAIN harvests the report.

## Campaign verdict

**PASS**: 59/59 mutants adjudicated; 56/56 scored semantic mutants died at G1; 3/3 weaker contracts stayed G1-green and lost exactly the intended public binding while G2 caught their trusted-surface drift. F1 survivors = 0, CRITICAL spec gaps = 0, equivalent survivors = 0. F2 all-gate survivors = 0, unbound clauses = 0. Invalid/confounded mutants = 0. Determinism D1-D3 = PASS.

## Phase-2 execution matrix

Baseline = `777ca8326e6dd7c0cbca53ad5e2fa6089c3a7767`; worktree = `.scratch/worktrees/rev2-m5u1`; MAIN baseline = G1 95/0, G2 rc0 `assumes=32`, G3 0 divergences, G4 94/94, trust battery 16/16. Progress = 59/59 adjudicated.

| mutant-id | family | applied site/operator | G1 verdict + failure shape | downstream gates | classification |
|---|---|---|---|---|---|
| S01-final-lf | F2 | align.rs:204; drop nonempty missing-LF rejection | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S02-strip-one | F2 | align.rs:207; retain terminal LF | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S03-empty-file | F2 | align.rs:208; disable empty-body check | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S04-row-physical-order | F2 | align.rs:193; drop last row while recursing | rc101; 94/1; assertion failed | not run: G1 kill | **PASS—KILLED-G1** |
| S05-fields-exactly-four | F2 | align.rs:152; permit fields beyond four | rc101; 93/2; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S06-group-canonical-check | F2 | align.rs:154; disable group canonicality | rc101; 93/2; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S07-start-canonical-check | F2 | align.rs:156; disable start canonicality | rc101; 93/2; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S08-empty-span-check | F2 | align.rs:158; disable empty-span rejection | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S09-side-vocab-check | F2 | align.rs:160; permit `mid` side | rc101; 93/2; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S10-side-text-selection | F2 | align.rs:163; swap named-side text | rc101; 93/2; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S11-range-end-equality | F2 | align.rs:170; reject end equal to length | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S12-span-equality-check | F2 | align.rs:172; invert span equality | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S13-order-group-before-start | F2 | align.rs:154; swap group/start checks | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S14-order-empty-before-side | F2 | align.rs:158; swap empty/side checks | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S15-row-one-based | F2 | align.rs:212; start row numbering at zero | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S16-group-set-polarity | F2 | align.rs:217; invert group-set mismatch | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S17-group-set-not-count | F2 | align.rs:217; replace set equality with span count | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S18-src-overlap-check | F2 | align.rs:219; disable src overlap | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S19-ace-overlap-check | F2 | align.rs:221; disable ace overlap | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S20-order-groups-before-overlap | F2 | align.rs:217; place src overlap before group sets | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S21-order-src-before-ace-overlap | F2 | align.rs:219; place ace overlap before src | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S22-touching-is-legal | F2 | align.rs:126; treat touching as overlap | rc101; 94/1; precondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S23-distinct-row-instances | F2 | align.rs:125; permit self-pairs | rc101; 94/1; precondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S24-decimal-nonempty | F2 | align.rs:25; permit empty decimal | rc101; 92/3; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S25-decimal-no-leading-zero | F2 | align.rs:27; permit leading zero | rc101; 93/2; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S26-decimal-unbounded | F2 | align.rs:26; cap decimal at u64 max | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S27-decimal-ascii-only | F2 | align.rs:15; weaken digit conjunction | rc101; 92/3; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S28-code-point-span-length | F2 | align.rs:30; replace code-point span length with UTF-8 width | rc101; 93/2; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S29-index-first-ace-sorted | F2 | align.rs:379; use authored ACE order | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S30-index-from-ace-not-src | F2 | align.rs:379; derive order from src | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S31-first-ace-sort-key | F2 | align.rs:277; sort raw spans by group first | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S32-dense-group-dedup | F2 | align.rs:339; disable group dedup | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S33-authored-ids-discarded | F2 | align.rs:368; emit authored group id | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S34-count-distinct-groups | F2 | align.rs:383; count ACE spans | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S35-src-output-sorted | F2 | align.rs:381; retain src row order | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S36-ace-output-sorted | F2 | align.rs:382; retain ACE row order | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S37-per-side-sort-key | F2 | align.rs:305; sort output by index first | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S38-render-one-byte | F2 | align.rs:245; change empty-file detail byte | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S39-row-decimal-multidigit | F2 | align.rs:56; render ten as one digit | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| S40-secondary-sort-key-deadness | F2 | align.rs:308; drop output index tiebreak | rc101; 94/1; postcondition not satisfied | not run: G1 kill | **PASS—KILLED-G1** |
| I01-final-lf-comparison | F1 | align_impl.rs:1431; invert final-LF comparison | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I02-range-comparison | F1 | align_impl.rs:760; reject end equal to length | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I03-end-boundary-plus-one | F1 | align_impl.rs:763; increment validated end | rc101; 94/1; arithmetic overflow obligation | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I04-strip-boundary-zero | F1 | align_impl.rs:1434; remove zero terminal chars | rc101; 94/1; assertion failed | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I05-row-ordinal-off-by-one | F1 | align_impl.rs:1443; start row numbering at zero | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I06-dense-index-off-by-one | F1 | align_impl.rs:1736; increment output index | rc101; 94/1; arithmetic overflow obligation | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I07-side-payload-swap | F1 | align_impl.rs:755; swap validation text payloads | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I08-side-collection-swap | F1 | align_impl.rs:853; route src span into ACE collection | rc101; 94/1; assertion failed | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I09-group-dedup-drop | F1 | align_impl.rs:1567; disable seen-group detection | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I10-early-return-after-row | F1 | align_impl.rs:807; skip recursive tail scan | rc101; 94/1; assertion failed | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I11-group-set-check-drop | F1 | align_impl.rs:1446; disable group-set mismatch | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I12-overlap-boundary | F1 | align_impl.rs:1110; treat touching as intersection | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I13-raw-sort-key-component-drop | F1 | align_impl.rs:989; drop raw end key | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I14-raw-sort-key-swap | F1 | align_impl.rs:979; sort raw spans by group first | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I15-output-sort-key-drop | F1 | align_impl.rs:1765; drop output start key | rc101; 94/1; postcondition not satisfied | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| I16-render-byte-substitution | F1 | align_impl.rs:1361; change ACE overlap literal byte | rc101; 94/1; assertion failed | G2/G3/G4 not run: G1 kill | **PASS—KILLED-G1** |
| C01-drop-binding | C | contract.rs:10; drop public outcome binding | rc0; 95/0 green | G2 rc1: contract manifest drift; G3/G4 n/a | **PASS—WEAK-BINDING DEMO** |
| C02-success-only | C | contract.rs:10; bind only wellformed inputs | rc0; 95/0 green | G2 rc1: contract manifest drift; G3/G4 n/a | **PASS—WEAK-BINDING DEMO** |
| C03-error-only | C | contract.rs:10; bind only invalid inputs | rc0; 95/0 green | G2 rc1: contract manifest drift; G3/G4 n/a | **PASS—WEAK-BINDING DEMO** |

## Phase-2 aggregate findings

- Semantic score = 56/56 G1 kills: F2 40/40; F1 16/16. G1 result distribution = 47 × 94/1, 7 × 93/2, 2 × 92/3. Primary failure shapes = 47 postcondition, 5 assertion, 2 precondition, 2 arithmetic-obligation failures.
- Contract demonstrations = 3/3 G1-green at 95/0; each release build rc0; each G2 rc1 reports only `ckc-kernel/src/contract.rs` manifest drift after the harness config is restored.
- Token-census preflight = unchanged for 59/59 mutations. Invalid/confounded/compile-only mutants = 0.
- No scored mutant reached downstream runtime gates because every F1/F2 mutant died at G1. Therefore CRITICAL spec gaps = 0, equivalent F1 survivors = 0, and unbound F2 clauses = 0.
- Cache isolation amendment: empty targets rebuild admitted vendored `vstd` under `--no-cheating` and fail before workspace proofs. Per-mutant targets were private copy-on-write clones of the verified dependency cache; each changed workspace file forced its intended proof. `.cargo/config.toml` was restored before G2 and at campaign close.
- Evidence retained under `.scratch/worktrees/rev2-m5u1/.scratch/m5u1/mutation/`: 59 result JSON files, 69 gate logs, 44 determinism transcripts, and the two replay scripts. Rebuildable caches/targets were removed after capture: 23 GB → 479 KB.

## Phase-2 determinism results

- D1 verify reproducibility = **PASS**: 3/3 isolated cache clones, workspace packages cleaned in each; all tuples identical at `rc0, 95 verified, 0 errors`. Negative `ensures false` control = `rc1, 1 verified, 1 error`.
- D2 behavioral binary determinism = **PASS**: 2/2 empty-target release builds rc0; binary SHA-256 both `7e9494b0272e85153aafb6dff42808c27c20184436a57dd00af1688b360ea69c`; 4/4 canonical-path suite runs rc0 and byte-identical at `94/94`; 2/2 G3 runs rc0 and byte-identical at `0 divergences`; stderr byte-identical and empty.
- D3 trust-audit determinism = **PASS**: 3/3 baseline meters byte-identical at `ckc: trust spec=470 shell=3 assumes=32 deps=ok`; metadata/rename reorder preserved 8/8 source hashes and meter bytes; ascending/descending eight-file probes matched at `shell=11`; two red diagnostic orders matched byte-for-byte at rc1; final meter, hashes, and clean tracked status restored.
