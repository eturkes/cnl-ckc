# Contract sgw — anonymous variables in clause lines (Tasks SGW)

Tier: kernel (trusted spec reading law + verified reader) + data (corpus re-emission). Gate identity: `just gate` (CI gate `just ci sgw`); fast gate = `just verify`, `cargo test --test v1_suite --test da2_acceptance --test consumer_recipe --test certify_one --test bf_emission`, `ckc compile cdc-2022-opioid`, `ckc queries cdc-2022-opioid`, `ckc check`, `ckc certify cdc-2022-opioid`.

## Problem

SWI-Prolog 10.0.2 prints `Warning: … Singleton variables: [F,G]` 65 times consulting `guidelines/cdc-2022-opioid/pl/cdc2022-opioid-s33-01.pl` (56 lines) + `-s42-15.pl` (9 lines): a formula rule's non-target clauses share the rule body, whose computed outputs (`guideline_product(D,E,exactly,rational(1,2),F,G)`) occur once in them. REFERENCE § Compiled Prolog schema promises that any set of documents "co-loads into one engine in any order with no warnings"; `consumer_recipe` fails only on `ERROR` lines. Prolog's own spelling of a variable that occurs once is `_` (each `_` reads as a new variable); the v1 codec has no `_` spelling (`v1text::var_bytes`, `wf_clause`).

## Design (rulings)

- D1 Emission (fork `ace_to_pl.pl`): every DOCUMENT clause line (rule or fact) spells each variable that occurs exactly once in that line `_`. Every other variable keeps the letter of its number in first-occurrence order counted over ALL variables of the line, `_`s included: `p(A,_,C) :- q(A,C)`, never `p(A,_,B)` (SWI's own `numbervars/4` `singletons(true)` renumbers the rest and is NOT the law). Question files, proof payloads, error lines: unchanged.
- D2 Reading law (trusted spec `v1text`): `read_v1(bytes)` turns a file into its letter spelling, line by line (LF-separated): a line that opens with `%` reads as written; on every other line, outside quoted atoms (a `'` opens one, `\` escapes the next byte inside, `'` closes), each maximal run of identifier bytes (`a-z A-Z 0-9 _`) is read in order: the run `_` becomes `var_bytes(n)` with n = the number of distinct variable spellings met so far on its line (letter runs by spelling + every earlier `_`); a run that opens with `A-Z` and equals a spelling some `_` of its line became = reading fails (`None`); every other byte and run reads as written. Applies to every v1 class (questions, answers + traces hold no `_` today; answers + traces stay ground by `wf_*`).
- D3 Acceptance: the old law becomes `canonical_v1(bytes)` (`∃ f. wf_v1(f) ∧ print_v1(f) == bytes`); `accepts(bytes) = read_v1(bytes) is Some(r) ∧ canonical_v1(r)`; `the_v1(bytes)` = the canonical file of the read bytes. Every model downstream (engine, answers, traces, certification, K4, K5) is the model of the read bytes; no other spec definition changes.
- D4 Model identity: for every document the fork emits, `read_v1(new bytes) == the pre-SGW bytes` (D1 is the exact inverse of D2 on emitter output) ⇒ parsed models, certification verdicts, answers, traces and their clause digests (`sha256` of the canonical re-render) stay byte-identical.
- D5 Reject offsets (R9: a deterministic diagnostic outside the theorems): a failed reading rejects at the raw offset of the refused letter run; a canonical-law reject on the read bytes reports the raw offset of the same byte (each earlier replacement shifts by `len(var_bytes(n)) - 1`); `at <= bytes.len()` stays proven.
- D6 Kernel: an exec reader proven equal to `read_v1` (output bytes + the None case); `v1_parse` + `v1_check_impl` read first and parse the read bytes; their ensures keep `r.is_some() <==> accepts(bytes@)` and `p@ == the_v1(bytes@)`, with `parsed_v1_ok` stated over the read bytes; callers that need the printed length use the read bytes.
- D7 Consumer gate: `consumer_recipe` fails on any SWI-Prolog warning line as on any error line, consulting every committed document + each consumer world; red on the pre-fix corpus.
- D8 Text: REFERENCE § Compiled Prolog schema states the `_` spelling; the compiler header + `.claude/rules/compiler.md` writer bullet + `.claude/rules/queries.md` trace-digest note follow.

## Testable predicates

- P1 Warning-free load: SWI-Prolog 10.0.2 `-f none -F none` consults `guidelines/schema.pl` + every committed document (`guidelines/*/pl/*.pl`) + every consumer world of `consumer_recipe` with empty stderr; the same load of 258ddc44's `cdc2022-opioid-s42-15.pl` prints a `Warning:` line (red).
- P2 Emission: in every document the fork emits (corpus + every compiled fixture tree + `tests/green` pins), no clause line holds a letter variable that occurs once, and every `_` stands for a variable that occurs once; letters follow D1's all-variable count.
- P3 Read identity (corpus replay): `read_v1` of each committed SGW document equals its 258ddc44 bytes; every document without `_` reads to itself.
- P4 Reading law (oracle over generated lines): `_` at line start / between punctuation / before `.` / adjacent to identifier bytes (`a_`, `_a`, `_X`, `__`: no replacement); `_` inside quoted atoms, after `\'`, in `%` lines (no replacement); ≥ 26 variables (`A1`-spelled replacements); a letter equal to a replacement of its line → reject at that letter; per-line reset; the kernel verdict + reject offset agree with an independent reference reader, and every accepted line's SWI `read_term` result is a variant of the kernel model's clause.
- P5 Kernel = spec: `just verify` (`--no-cheating`) proves the exec reader + `v1_parse`/`v1_check_impl` ensures.
- P6 Unchanged verdicts: every existing suite + `ckc check` + `ckc certify` + `just mobile` stays green with its pins unchanged except the compiled bytes D1 changes (the two CDC documents, their review + release manifest digests, compiled fixture documents + `tests/green` pins, the archive).
- P7 Offsets: a file with a `_` before its first canonical-law divergence rejects at the raw offset of that divergence (D5), shown with a replacement of length 1 and of length ≥ 2.
- P8 Consumer gate red: `consumer_recipe` with a warning line planted (a pre-fix document) fails naming it.

## Close checks

- Corpus: `ckc compile` changes exactly the lines P2 names (the corpus replay of P3); `ckc review-manifest` re-pins the two documents' digests (no ledger rows exist: `decisions=0`); `just amend-manifest`.
- Mutation credit: `just mutants` over the shell files the new tests call; every survivor ruled.
- Teammates: consultant on this contract before code; diff-blind tester suite from it (rust.md kernel unit flow).
