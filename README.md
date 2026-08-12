# cnl-ckc

A minimal, fully auditable pipeline that turns clinical guidelines into
executable Prolog through controlled natural language:

1. **Fetch** — a guideline source document is acquired and recorded under
   `guidelines/<id>/source/` with digests, rights, and attribution in that
   guideline's `README.md`.
2. **Normalize** — recommendations are authored as Attempto Controlled
   English (ACE), one document per ruled source region, under
   `guidelines/<id>/ace/`, with domain vocabulary in
   `guidelines/<id>/lexicon.ulex`.
3. **Compile** — `tools/goal.py` stages the vendored APE parser and compiles
   every ACE document to plain Prolog under `guidelines/<id>/pl/` via
   `vendor/ape/prolog/ace_to_pl.pl`.

Claude Code's built-in `/goal` command drives all three steps ("Operating"
below).

## Audit story

Every program artifact a human needs to read is controlled natural language,
a direct compilation of it, or part of one small named compiler base:

- **ACE → Prolog.** All guideline Prolog (`guidelines/*/pl/`) is compiled
  from ACE. The compiled files quote each source sentence beside its clause,
  and each carries the SHA-256 of the exact ACE and lexicon bytes it was
  compiled from. The compiler also derives each sentence's
  obligation and discharges it against the document's own clauses — alone
  and co-loaded with the rest of the batch — so a shipped clause is one
  the compiler has proved follows from the sentence it quotes.
- **E-- → Python.** All first-party Python (`tools/goal.py`,
  `tools/regen.py`) is compiled from E-- (`tools/*.emm`), an English-like
  language. `tools/regen.py --check` proves every committed `.py` is
  byte-identical to a fresh compile of its `.emm` and flags any tracked
  Python outside `vendor/e--/src/` that has no `.emm` source.
- **The compiler base is closed and named.** Two vendored forks perform
  those compilations and are the trusted computing base a human must read
  directly: `vendor/ape/` — the ACE parser plus the hand-authored
  `prolog/ace_to_pl.pl` compiler, the one first-party Prolog artifact not
  itself compiled from ACE — and `vendor/e--/` — the hand-authored Python
  that compiles E--. Both trees are pruned to their load closures;
  `vendor/*/PROVENANCE` records upstream, fork base, import commit, license,
  first-party inventory, and trust boundary. Git history is the change
  record, and `goal.py check` reads it: every tracked vendored file a commit
  after the recorded import commit touched — or the working tree edits —
  must carry a change notice in its first 40 lines, dated where the tree's
  license is a GPL family (§5(a)) and undated where it is Apache-2.0
  (§4(b)); every untouched file must carry none. Declared first-party files
  are exempt and state origin and license in their own headers. A third
  vendored tree, `vendor/clex/` — the COMLEX-derived general-English base
  lexicon consulted beneath every per-guideline `lexicon.ulex` — is not a
  fork: its `PROVENANCE` declares `Pristine: yes`, and the same gate then
  verifies content instead of history — every tracked non-first-party file
  must match its `MANIFEST.sha256` digest and carry no change notice.
- **Tests are data.** `tests/red/` holds rejection probes named
  `<expected-error-class>--<name>.ace`. One `tools/goal.py check` invocation
  beside `tools/regen.py --check` (E-- → Python identity, above) is the
  full acceptance gate: `check` validates the compendium ledger
  (`.agent/compendium.md` organization table plus `.agent/compendium.tsv`
  guideline rows — row vocabulary, canonical ordering, the
  one-active-row promotion invariant — and prints the terminal meter the
  `/goal` exhaustion clause reads), then validates layout, source records,
  and Prolog/lexicon inventory closure, validates every guideline's corpus
  ledgers (projection-notes header bytes and per-document totality;
  coverage-ledger closure — row grammar, region totality against each
  evidence file's own payload census and locator inventory, ace-row ↔
  document-set bijection, single-step restates targets — printing a
  per-guideline `ace/restates/uncovered/pending` meter; lexicon
  liveness and minimality; v1-only product vocabulary), recompiles every
  guideline twice
  (byte-determinism), compares against the committed Prolog (freshness),
  load-checks every compiled document, derives and discharges every
  document's proof obligations alone and as one co-loaded composition whose
  manifest is pinned to the guideline's whole document set, scans that
  composition for left recursion, and asserts each red probe is rejected
  with its named error class and exit status.

## Running

Requires SWI-Prolog 9.2.9 and Python ≥ 3.11 (the supported invocations use
`python3 -P`; CI pins the SWI 9.2.9 container, whose Debian Python is 3.11).

```sh
python3 -P tools/goal.py compile <guideline-id>   # ACE → Prolog
python3 -P tools/goal.py check                    # corpus acceptance gate
python3 -P tools/regen.py --check                 # E-- → Python identity
```

## Compiled Prolog schema (v1)

Status: **frozen**. Every compiled document emits
`guideline_schema_version(1).`, and the predicate set below is the public
ABI — extending it requires a version bump. v1 is the compiler's sole
schema; a future version would be selected by a new explicit invocation
argument, never inferred from content. Authored questions reject as
unsupported.

Sentences project onto a closed reserved vocabulary. Source words —
nouns, verbs, adjectives, prepositions — stay opaque data atoms and never
become predicate functors, so the schema is language- and
domain-neutral:

| Predicate | Meaning |
| --- | --- |
| `guideline_schema_version(Version)` | schema version of this document (`1`) |
| `guideline_document(DocId, ace_sha256(H), ulex(none \| sha256(H)))` | document record + source digests |
| `guideline_entity(Context, Ref, Noun, Class)` | an entity referent and its noun/class |
| `guideline_cardinality(Context, Ref, Unit, Comparison, Count)` | that referent's quantity payload, verbatim |
| `guideline_event(Context, EventRef, Lemma)` | an event referent and its verb |
| `guideline_arg(Context, EventRef, Position, Ref)` | participant at `Position` (1-based, DRS order) |
| `guideline_pp(Context, EventRef, Preposition, Ref)` | prepositional attachment on the event |
| `guideline_property(Context, PropertyRef, Lemma, Polarity)` | adjectival property (`pos` in v1) |
| `guideline_operator(OuterContext, InnerContext, Operator)` | reified modal/negation wrapper: `InnerContext`'s content stands under `Operator` relative to `OuterContext` |

`Comparison` is exactly one of `eq`, `geq`, `greater`, `leq`, `less`,
`exactly`, `na`; units and counts are copied through as data. The
compiler performs no arithmetic, unit conversion, counting, or modal
inference — a cardinality clause records what the sentence said, nothing
more.

Every one of those values is reachable, but not from every position. An
upper-bounding determiner (`at most n`, `exactly n`, `less than n`) is
scoped: at the root of an asserted sentence ACE groups the conditions it
binds into a condition list, and v1 has no construct for that scope, so
the document rejects with `unsupported, root_condition([…])` rather than
flatten an upper bound into an existential claim — `tests/red/
unsupported--v1-root-at-most.ace` pins the detail. The same phrase inside
a rule or an operator box carries no group and compiles, emitting `leq`,
`exactly` or `less` under that context; `at least n` and `more than n`
bound below, are never grouped, and compile in any position.

`Context` is `actual` for asserted content, or a generated operator
context:

```
Context      ::= actual | '$guideline_id'(context, DocId, S, box(B), Deps)
Operator     ::= should | must | can | may | '-'
OperatorEdge ::= guideline_operator(OuterContext, InnerContext, Operator)
```

Modality and classical negation are reified as data, never inference:
each DRS operator box becomes one `guideline_operator/3` edge from its
enclosing context to a fresh box context, emitted before the box's
payload clauses, which carry that box context in their own `Context`
argument. The operator atoms are the DRS functors themselves (`-` is
classical negation), so a non-English frontend produces the same tags;
¬SHOULD(P) and SHOULD(¬P) stay structurally distinct, and the compiler
performs no deontic, modal, or classical inference. `box(B)` numbers a
sentence's operator boxes in whole-sentence preorder; `Deps` follows the
Skolem dependency rule — empty for root facts, the antecedent's
top-level referents for consequent boxes — so a consequent operator
context is a per-antecedent-solution witness. In rule bodies an operator box binds an
existential context variable (`guideline_operator(Outer, V, Op)` plus
payload goals sharing `V`): matching is Horn-monotone structure
satisfaction over whatever documents are loaded, so extra clauses under
a producer context never block a match, and two body boxes may bind one
producer context whose payload satisfies both. A consumer that wants unwrapped
content only keeps `Context = actual`; an operator-aware consumer adds
one recursive walk over `guideline_operator/3` edges (a `ctx_desc/2`
descendant relation) to collect content under a modality.

Negation-as-failure ("does not provably …") in a rule antecedent
compiles to an executable `\+ (…)` over the box's goals. Its scope is
the loaded aggregate: `\+` is evaluated against whatever document set is
co-loaded, so absence is corpus-relative, never document-local.
Negation-as-failure in a consequent or a root fact rejects —
assertion-by-absence is meaningless.

A universally quantified sentence becomes one Horn clause per consequent
condition, all sharing one identical rendered body per rule variant: a
nested `if … then if … then …` consequent curries into one rule whose
antecedent concatenates the nested domains, and an antecedent containing
one disjunction splits into two variants whose bodies are the shared
conditions plus that arm — the bundle keeps one sentence comment with
variant-1 clauses then variant-2 clauses, contiguous. Referents introduced
only in the consequent are Skolemized to
`'$guideline_id'(product, DocId, SentenceId, ref(N), Deps)`, where `Deps`
is the concatenation of every curried antecedent segment's top-level
referents in DRS order (a split variant appends its arm's top-level
referents) — referents introduced inside an operator or
negation-as-failure (NAF) box contribute nothing to any `Deps` list;
they are consequent-inaccessible and stay box-local existentials — and
`N` is the
referent's first-occurrence position in the sentence's unsplit
expansion — both variants of a split mint the same `ref(N)`,
distinguished by `Deps`. The constructor's first argument names what the
term stands for: `context` for an operator box, `product` for a
consequent referent, `witness` for a proof instance (below). All three
name the compiler's own chosen stand-in — never a source-given name,
real-world identity, or uniqueness claim. Because facts and derived heads
share one vocabulary, a rule body consumes another document's facts or
heads by ordinary resolution.

Every v1 document opens with the same declaration block — `multifile` and
`discontiguous` for all nine indicators, whether or not it populates
them — followed by `guideline_schema_version(1).`, the document record,
then per sentence a
`% S<n>: <verbatim sentence>` comment and that sentence's contiguous
clauses in deterministic order. Any subset of v1 documents therefore
co-loads into one engine in any order with no warnings. Names beginning
`guideline_` and the identity constructor `'$guideline_id'` are reserved:
a colliding name rejects the document rather than silently shadowing
schema vocabulary, and the compiler scans both the parsed sentence and
every lexicon entry — used or not — for one. `guideline_part/3` is
reserved for a later unit and unemitted in v1; shapes still outside the
schema (group coordination, disjunctive consequents, rules scoped inside
an operator, questions) reject with a canonical
`ace_to_pl_error(unsupported, …)` line.

Compilation is checked, not asserted. Every v1 compile derives the
document's obligations and discharges them before emitting anything;
appending `proof` to the invocation prints the derivation payload in
place of the document. The payload is one
`'$guideline_proof'(DocId, S, variant(K), witness(Facts), prove(Heads))`
term per fact group and per rule variant, where `Facts` is that group's
own positive body under a witness substitution — NAF boxes contribute
nothing, by design — and `Heads` its clause heads under the same
substitution. The compiler asserts each witness ahead of the document's
rule clauses, proves every head under bounded search — depth 4000 inside
1,000,000 inferences, either bound exceeded counting as underivable —
then retracts. A group whose heads stay non-ground under
its own witness rejects with `proof, nonground_obligation(…)`; one whose
heads do not derive rejects with `proof, underivable_obligation(…)`. Both
exit 1. Witness terms carry the document's own document, sentence, and
variant coordinates, so no foreign clause can discharge an obligation:
success is modus ponens over that sentence's own projection.

`aggregate-check <manifest>` replays those obligations across a
composition. The manifest is strict: one `<compiled-pl>` TAB `<payload>`
row per document, final newline required, a 0-byte file meaning the empty
composition. The compiler loads every document into one engine (any load
diagnostic is a failure), checks that the distinct `guideline_document/3`
records equal the manifest row count and that the loaded schema-version
set is exactly `[1]`, then re-derives every obligation against the whole
batch — so co-loading can neither break a document's derivations nor
silently repair them, and negation-as-failure is evaluated against the
composition it will actually run in. It reports
`ace_to_pl aggregate ok <N> documents <G> obligations`.

`recursion-check <manifest>` loads that same composition (payload column
ignored) and proves no clause head unifies with its own leftmost body
goal, renaming the goal apart first the way SLD renames a clause. A
match rejects as `proof, left_recursive(Site, Name, Arity)`, where
`Site` is the offending clause's own `sentence(DocId, S)` when it
carries one. The scan is structural, so its verdict is load-order
independent; it reports `ace_to_pl recursion ok <N> documents <M> rule
clauses`, the count naming what it read.

Payloads are evidence, not authority: `<G>` is checked against the
composition rather than taken from the payload stream. Before any head is
proved, the obligation set must cover exactly the sentence identities the
loaded documents themselves carry (`missing_obligation` /
`extra_obligation`), its `(document, sentence, variant)` keys must be
unique (`duplicate_obligation`) with variants numbered `1..K` per
sentence (`variant_sequence`), no group may prove an empty head list
(`empty_obligation`), and a nonempty payload file must end in the newline
its last term wrote (`check_load, payload_bytes`). An emptied, truncated,
repeated or misattributed payload therefore fails the replay instead of
shrinking it. Manifest rows bind product to payload for provenance;
replay soundness rests on that composition-wide coverage. One residue is
open by construction: dropping the LAST variant of a multi-variant
sentence leaves coverage complete and variant numbering contiguous, and
the frozen ABI carries no per-sentence variant count to check it against.

Compiled documents are a definite-clause program, so termination is the
consuming engine's responsibility, not a property the schema can promise:
facts and derived heads share one vocabulary by design (that is what lets
one document's rules consume another's clauses), which also lets an
authored rule whose consequent entity feeds its own antecedent form a
cycle. Under naive SLD such a corpus can diverge in one clause order and
succeed in another; an engine with tabling or bottom-up evaluation is
immune. This repository's own gates are fail-closed against it: every
obligation is proved under the bounded search above, so a divergent
branch is cut by those bounds and reported as a failed obligation
instead of hanging. Left recursion — a head that unifies with its own
leftmost body goal, the shape that makes an open query's termination
depend on load order — is checked rather than asserted: every
`tools/goal.py check` runs `recursion-check` over the loaded
composition and rejects one, printing the rule-clause count it scanned.
The shipped corpus holds none.

Authoring notes (v1): an ACE document states guideline knowledge and
nothing else — no witness seed facts, no proper-name stand-ins, no
authored probe queries, since the compiler derives its own obligations —
and `lexicon.ulex` carries reusable domain vocabulary rather than
per-statement fixtures. Every ulex lexeme must be reachable: an entry whose
lexeme no document uses rejects, as does one repeated in the file or one the
vendored Clex already provides byte-for-byte. One statement per sentence — split "…, and if A
then B" into separate sentences. Keep quantified restrictors out of
modal complements: a universally quantified rule nested inside a modal
box rejects (`operator_scoped_rule`), so state the quantification in the
antecedent and let the modal box wrap only the consequent. A
prepositional phrase attaches to the verb. Modifying a noun with `of`
rejects (`condition_shape(relation(A,of,B))`) — `every clinician of a
clinic` becomes either a relative clause, `every clinician who works at
a clinic`, or one compound noun with its own lexicon entry,
`every clinic-clinician`. Count nouns take an explicit determiner. `for`
as a preposition needs a `prep(for,for)` user-lexicon entry. An
indefinite consequent entity that repeats the noun of the antecedent's
first condition (`If a patient … then a patient …`) mints a fresh
referent whose clause head unifies with its own leftmost body goal: the
document compiles and `check` then rejects the composition as
`proof, left_recursive(…)`. Refer back to the antecedent referent
(`the patient`) or give the consequent entity its own noun.

## Operating

Guideline work runs as goal rounds in a Claude Code session opened at the
repository root, driven by the built-in `/goal` stop-condition command:

```
/goal Process American clinical guidelines through the pipeline as described in README.md "Operating", fanning bulk work out to teammates per .agent/rounds.md: work the in-progress source document to full coverage before fetching the next; done only when either (a) the user has asked to stop, pause, or wind down — the request alone meets this goal at any point, even mid-round with the worklist unfinished; start nothing new, state where work stands, and stop — or (b) every fetched guideline is complete, every remaining .agent/queue.md entry is a recorded blocker, and the compendium exhaustion clause in .agent/compendium.md "Protocol" holds: every guideline row of .agent/compendium.tsv done, blocked, or excluded, and every organization row terminal.
```

The goal re-arms each time Claude tries to stop and survives session resume,
so halting at any moment is safe: the repository is the only persistence,
and every round starts by deriving state from it — `.agent/queue.md`,
`git status`, `tools/goal.py check`, and the in-progress guideline README's
coverage statement — then finishes or discards incomplete work before
taking on anything new. A user request to stop is condition (a) of the
goal itself: "let's stop here" satisfies the stop check at once —
mid-round included — rather than re-arming against the unmet exhaustion
clause, and the wind-down is to start nothing new, state where work
stands, and stop; `/goal clear` remains the unconditional disarm. The
check's first stage validates the compendium
(`.agent/compendium.md` organizations table plus `.agent/compendium.tsv`:
row format and vocabulary, canonical ordering, the single-active-row
promotion invariant) and prints a terminal meter — remaining
non-terminal organizations and unfinished guideline rows — that measures
the exhaustion clause directly. Corpus validation (projection-ledger
totality, coverage-ledger closure with its region-status meter, lexicon
liveness and minimality, v1-only product vocabulary) runs
for every guideline on every check. Bulk work (source reading, extraction drafting, ACE
drafting, adversarial review) fans out to subagent teammates per
`.agent/rounds.md`; the session lead alone writes the repository and
commits.

Exactly one source document is in progress at a time and is worked to
completion — across as many rounds as it takes — before the next one is
fetched. A source document is complete when every normative statement in it
has been extracted verbatim into `source/` evidence and either authored as
knowledge-only ACE and compiled — obligations discharging alone and in
aggregate — or recorded in the guideline README as uncovered with a
reason, and `python3 -P tools/goal.py check` is green with the guideline's
coverage meter reading `pending=0`.

While a document is in progress, a round advances it one increment:

1. **Extract** — the next batch of normative statements, verbatim, into
   `guidelines/<id>/source/` evidence files.
2. **Normalize** — one ACE document per ruled source region under `ace/`,
   holding one or more source-anchored knowledge sentences, plus the
   shared `lexicon.ulex` (supported constructs: the header of
   `vendor/ape/prolog/ace_to_pl.pl`). Each ACE document is a minimal
   faithful projection of its region; the projection-notes ledger and the
   guideline README's coverage statement use that same unit.
3. **Compile** — `python3 -P tools/goal.py compile <id>`. A rejection means
   the ACE or lexicon is adjusted first; only a genuinely new construct
   extends the `ace_to_pl.pl` translation, minimally, keeping totality
   (every sentence compiles to a determinate clause bundle whose
   obligations discharge; unrecognized shapes reject) and adding
   `tests/red/` probes for the new rejection boundary.
4. **Close** — `python3 -P tools/goal.py check` green, plus
   `python3 -P tools/regen.py --check` when E-- sources changed; a scoped
   commit; `.agent/queue.md` and the guideline README's coverage statement
   updated.

When no document is in progress, the round fetches the next source — the
queue's next entry, the queue refilling from `.agent/compendium.tsv` per
`.agent/compendium.md` § Queue promotion when it runs dry: acquire the
document and
persist it immutably under `guidelines/<id>/source/` before any generative
processing; record URL, retrieval date, SHA-256, byte length, and a rights
quote in `guidelines/<id>/README.md` (model:
`guidelines/cdc-2022-opioid/`); commit the source record on its own, so it
survives any halt. Ids follow `[a-z0-9-]+`, chosen once per source; an
already-fetched URL keeps its recorded id, and changed remote content
becomes a new versioned id while every recorded source stays immutable. A
paywall or rights gate is recorded as a queue blocker and the round moves
on.

## Licensing

First-party work outside `vendor/`: Apache-2.0 WITH LLVM-exception
(`LICENSE`). First-party additions inside a vendored tree adopt that tree's
license — `vendor/ape/prolog/ace_to_pl.pl` is LGPL-3.0-or-later. Vendored
trees keep their own licenses: `vendor/e--` Apache-2.0, `vendor/ape`
LGPL-3.0-or-later, `vendor/clex` GPL-3.0-or-later. Distribution of the full
repository combination conveys under GPLv3; first-party work remains
Apache-2.0 WITH LLVM-exception and independently reusable. See `NOTICE`.
