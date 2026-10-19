Status: retired, not landed (user ruling at the condition-scope close: the review UI keeps the sections of 53186717; spec Decisions `Review UI`). Work kept at tags `archive/cs1b` (seed: contract + spec ui.rs § Scope exclusions + 15 copy rows; tester-4 suite `cs1b_ui.rs` + `tests/cs1b-ui/`; REFERENCE + rules text), `archive/cs1b-general-3` (K5 kernel: `k5_scope.rs` classifier + soundness, page proofs, strength-section isolation; full strict verify spec 285 + kernel 5025 verified, 0 errors), `archive/cs1b-tester-4` (suite at the seed). The D0 classifier (record sentence = a paired bundle holding a record clause) serves the reading line's scope-pair restriction (Tasks `UI revert`).

# cs1b — scope review surfaces (contract)

Row: `.agent/spec.md` Tasks CS1b (Phase: IMPLEMENT — M7 gap class condition-scope; Decisions: Verification line K5). Gap: CS1 compiles scope exclusions (record clause `guideline_exclusion(Key) :- <condition>` ending each exclusion variant) and guards every should/must/may edge clause of a scoped document (`\+ guideline_exclusion(Key)` last), but no review page shows either: a clinician cannot see which sentences compiled as exclusions, which statements the exclusions gate, or which table words trigger the exclusion form. Intent goal 2 (the ACE ↔ source audit as easy as possible): the condition of an exclusion reads in its own ACE sentence, so these surfaces state the compiled effect + list the exclusion sentences verbatim; no surface re-renders a condition (a lossy renderer would drop a comparison word: `include 18 years` for `includes less than 18 years`; a second condition renderer would also enlarge the audited specification).

## Tier

`kernel`: K5 render fidelity (`rust/ckc-spec/src/ui.rs` human-read; `ckc-kernel` `k5_*` exec + soundness), copy registry append (generators `k5-sound-literals` + `k5-sound-registry`). No compiler, codec, engine, certification or corpus change; review bundle v2 unchanged. Kernel unit flow = `.claude/rules/rust.md` (consultant-2 C1–C8 + one re-review round, seeded `wt/cs1b` on CS1, diff-blind tester, reviewer, `just verify` 0/0, `just gate`); UI law = `.claude/rules/ui.md` + `clinician-design.md`.

## Design

D0 Shared domain + classification (one per-document classifier, reused by D1–D3; opaque in page proofs, as `timing_rows`):
- Scoped guideline: its `vocabulary.tsv` parses (`vocabulary::parse_vocabulary`) and `vocabulary::scoped` holds; Key = the scope row's key. Every surface below requires a scoped guideline; no table, an unscoped table or a table without the pair → no surface anywhere.
- A document classifies only when its pl parses as a canonical document (`v1text::accepts` + `replay::the_v1` = `V1File::Doc`); otherwise it contributes no surface. Classification reads clause structure (heads + body items), never comment or text occurrences.
- Record clause: head `guideline_exclusion(K)` (arity 1) with K the atom Key. Record sentence: a bundle (sentence s) holding ≥1 record clause — one per bundle whatever its variant count (a Horn split = two record clauses, one sentence).
- Guarded edge: head `guideline_operator(_, _, Op)`, Op ∈ {should, must, may}, whose last body item is the singleton NAF `Naf([guideline_exclusion(K)])`, K = Key. Deontic edge: any clause with such a head. A document is gated iff it holds ≥1 deontic edge and every deontic edge is guarded (checked over all its clauses, so the D3 claim holds on any input the UI admits, certified or not).
- Exception note: the record clause body of some paired record sentence holds a `Naf` item (record bodies only — a guard is never a record-body item; unpaired sentences make no claim, as their rows).
- Sentence text: for record sentence s, the s-th nonempty LF-delimited line of the document's raw ACE bytes (the certification line law: `replay` line split, no trimming); the row renders only when that line exists and equals the bundle's sentence text (`b.text`), else the sentence contributes no row (no affirmative "as compiled" claim on an unpaired input).

D1 Guideline page of a scoped guideline: section `<section id="scope-exclusions">` "Scope exclusions" after "Qualifier words":
- paragraph (compiler law, as the word sections state theirs): the compiler gates the should, must and may statements of each document compiled with this table; a gated statement holds only while no scope exclusion of this guideline holds; a sentence reads as a scope exclusion only when it denies the exclusion word of the scope word (table below); each exclusion's condition reads in its sentence; the absence of an exclusion is not proof that a patient belongs to the population of the guideline;
- table Word | Reads as: the scope noun → "every recommendation of this guideline"; the exclusion verb → "denied by a scope exclusion" (lemmas through `pl_word` over the table bytes, as Qualifier words);
- table Document | Sentence | Exclusion: one row per paired record sentence (D0), documents in index order, sentences ascending; Document = link to the document page (its title), Sentence = canonical decimal, Exclusion = the paired ACE line, escaped; duplicates across documents stay separate rows (separate audit items); zero rows → a paragraph stating that this page lists no scope exclusion sentence (zero rows prove only that none paired, not that none compiled), no table.

D2 Document page with ≥1 paired record sentence: section "Scope exclusions as compiled" after "Qualifiers as compiled": one paragraph — the compiler read sentence N (one) | sentences N (several) as a scope exclusion, N ascending, joined `1` | `1 and 2` | `1, 2 and 3` | `a, b, …, y and z` (any count, multi-digit numbers verbatim): when the condition of such a sentence holds for the patient, the gated should, must and may statements of this guideline do not hold; plus, iff the exception note holds, one more sentence: a condition stated with `does not provably` is an exception of its own sentence — the check uses the loaded facts and rules; when they establish the exception, the exclusion does not follow through the condition that states it (another condition of the same sentence — a Horn-split branch — and other exclusions still apply); a search that the engine's bounds cut does not count as absence.

D3 Gated document page (D0): one paragraph after the provenance + tally lines: the should, must and may statements of this document hold only while no scope exclusion of this guideline holds for the patient, + link "Scope exclusions" → `../index.html#scope-exclusions`. Independent of D2 (a page can show both: S9-01). `can` edges and ordinary statements carry no guard, so the copy names the gated operators.

D4 Byte identity: every document page without a paired record sentence and not gated, every page of an unscoped or table-less guideline, the records pages and the root index stay byte-identical; a scoped guideline's index page changes even with zero record sentences (D1's no-exclusion paragraph).

D5 Copy: every fixed string = a new `copy_table!` row appended at the table end (registry 345 → 345 + k; every new row with a named use outside the table; existing fixed fragments + the operator words reused); no functor names, keys, digests or schema words in visible copy; clinician register; the final wording + placement of D1–D3 = the user's screenshot ruling at the phase close (CDC guideline page, S3-03, S7-02, S9-01, S10-04, one gated recommendation page, one mixed can/should page).

## Predicates (acceptance; seed-red unless marked `already-passing`)

Harness: `rust/ckc/tests/cs1b_ui.rs` over fixtures `tests/cs1b-ui/` (a dedicated binary in `just test`, as `s2_ui.rs`); existing goldens stay comparison inputs, never regenerated by this unit.

- P1 guideline page: a scoped fixture with two documents → the section after Qualifier words, both word rows, one Document | Sentence | Exclusion row per paired record sentence (index order, ascending, ACE line verbatim, a hostile byte sequence escaped, a duplicate exclusion in two documents = two rows); a scoped table with zero record sentences → the no-exclusion paragraph, no table; no table or an unscoped table → no surface (already-passing); a table/key mismatch (record + guards keyed `other`) → the scoped index renders D1 with the no-exclusion paragraph and no document page gains a surface.
- P2 document page: record sentence sets {1}, {1, 2}, {1, 2, 3}, {1, 2, 3, 4}, nonconsecutive {2, 5}, multi-digit {10, 11, 12, 13} → the joins + singular/plural; a Horn-split exclusion (two record clauses in one sentence, NAF in one variant only) → one sentence number + the exception note; exception note present iff a record body holds a NAF item (control: an unrelated rule with NAF and NAF-free records → absent; a guard alone never counts).
- P3 gated statement: present on pages whose deontic edges are all guarded (should, must, may; nested under `can`); absent on a scoped page whose only edges are `-`/`can`, on a mixed page with one unguarded deontic edge, and for each non-qualifying guard shape (a positive `guideline_exclusion` goal, a guard not last, a grouped NAF with another member, another key); present with D2 on a page holding both; the link's path resolves (`ckc ui check` closure) and a direct assertion finds exactly one `id="scope-exclusions"` on its destination (red plant: the id renamed → that assertion fails; `ckc ui check` strips fragments and is not credited for it).
- P4 raw inputs: blank ACE lines keep sentence numbering (the s-th nonempty line); a bundle whose sentence comment differs from its ACE line → no row; a pl that fails the canonical parse → no surface for that document; record or guard text only inside a comment → no record, no guard.
- P5 byte identity: every existing `tests/ui` golden + `tests/s1-ui` + `tests/s2-ui` fixture unchanged against the CS1 tree; `ckc ui check` page invariants; `just mobile` (red inputs + 0 bad pages); copy registry census (`copy_registry.rs`) + `ckc ui copy-check` over the new rows; no functor name in visible text.
- P6 kernel: `just verify` 0 errors 0 warnings; K5 soundness for the three surfaces (an opaque per-document classifier + a per-row ACE-span lemma + a guideline-section fold, composed into the existing page splits); generators replay `identical`.
- P7 screenshots (D5 set; CDC corpus after CS2a) filed for the user's ruling.

## Out of scope

Re-rendering an exclusion's condition; per-recommendation scope (CS2b authors those conditions in the rules); review-bundle digest changes; extending `ckc ui check` to validate fragment ids globally.

## Gate identities

`just gate`; cs1b_ui (new), ui_fixtures, copy_registry, n22_viewport, s1_ui, s2_ui; diff-blind tester suite at the seed; `just mobile`.

## Flow

Contract → consultant-2 (C1–C8: 4 sound, 4 findings amended here; re-review RR2–RR5: 2 pass, RR3 + RR4 findings amended: key-mismatch index, guarded-statement effect, listed-not-compiled, per-condition exception) → seed `wt/cs1b` on CS1 (contract + spec + copy rows + red stubs) → tester (diff-blind: P1–P5) → kernel → reviewer → gate → CS1b commit (squash); screenshots → phase-close ruling.
