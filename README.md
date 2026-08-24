# cnl-ckc

cnl-ckc is a minimal, fully auditable pipeline. It turns clinical
guidelines into executable Prolog through controlled natural language:

1. **Fetch** — acquire a guideline source document and record it under
   `guidelines/<id>/source/`. Digests, rights, and attribution go in
   that guideline's `README.md`.
2. **Normalize** — author the recommendations as Attempto Controlled
   English (ACE), one document per ruled source region, under
   `guidelines/<id>/ace/`. Domain vocabulary lives in
   `guidelines/<id>/lexicon.ulex`.
3. **Compile** — `tools/goal.py` stages the vendored APE parser and
   overlays `vendor/clex/clex_lexicon.pl` as the general-English base
   lexicon. It compiles every ACE document to plain Prolog under
   `guidelines/<id>/pl/` via `vendor/ape/prolog/ace_to_pl.pl`.

Claude Code's built-in `/goal` command drives all three steps
("Operating" below).

## Audit story

Every program artifact that a human must read is controlled natural
language, a direct compilation of it, or part of one small named
compiler base:

- **ACE → Prolog.** All guideline Prolog (`guidelines/*/pl/`) compiles
  from ACE. A compiled file quotes each source sentence beside its
  clauses, and it carries the SHA-256 of the exact ACE and lexicon
  bytes that produced it. The compiler also derives each sentence's
  proof obligation and discharges it against the document's own
  clauses, alone and co-loaded with the rest of the batch. A shipped
  clause is therefore one that the compiler has proved follows from the
  sentence it quotes. A committed query answer ships with a proof
  trace. Each clause application in that trace names one committed
  clause line by SHA-256.
- **E-- → Python.** All first-party Python (`tools/goal.py`,
  `tools/regen.py`, `tools/ui.py`) compiles from E-- (`tools/*.emm`),
  an English-like language. `tools/regen.py --check` proves that every
  committed `.py` is byte-identical to a fresh compile of its `.emm`.
  It also flags any tracked Python outside `vendor/e--/src/` that has
  no `.emm` source. The reviewer interface is E-- like the rest. It
  uses the `Try:`/`Catch <name>:` verbs that the E-- fork added for it,
  so a failing standard-library call becomes a named HTTP outcome
  instead of a generic server error.
- **Review is recorded, not asserted.** A reviewer decision names the
  exact bytes it judged. Each ledger row pins a bundle digest over the
  document's ACE text, its coverage row, its source region payload, and
  its compiled clauses. The reviewer interface reads committed files
  only, so what a reviewer approves is what the repository holds.
  "Reviewer interface" below states the workflow.
- **The compiler base is closed and named.** Two vendored forks perform
  those compilations. They are the trusted computing base that a human
  must read directly. `vendor/ape/` holds the ACE parser plus the
  hand-authored `prolog/ace_to_pl.pl` compiler — the one first-party
  Prolog artifact not itself compiled from ACE. `vendor/e--/` holds the
  hand-authored Python that compiles E--. Question projection, query
  answering, and proof tracing are modes of that same `ace_to_pl.pl`,
  so those artifact families added no new trusted code. Both trees are
  pruned to their load closures. `vendor/*/PROVENANCE` records upstream, fork
  base, import commit, license, first-party inventory, and trust
  boundary. Git history is the change record, and `goal.py check` reads
  it. A vendored file counts as touched when a commit after the
  recorded import commit changed it, or when the working tree edits it.
  Every touched tracked file must carry a change notice in its first 40
  lines. The notice must be dated where the tree's license is a GPL
  family (§5(a)) and undated where it is Apache-2.0 (§4(b)). Every
  untouched file must carry none. Declared first-party files are
  exempt; they state origin and license in their own headers. A third
  vendored tree, `vendor/clex/` — the COMLEX-derived general-English
  base lexicon consulted beneath every per-guideline `lexicon.ulex` —
  is not a fork. Its `PROVENANCE` declares `Pristine: yes`, so the same
  gate verifies content instead of history. Every tracked
  non-first-party file must match its `MANIFEST.sha256` digest and must
  carry no change notice.
- **Tests are data.** Each corpus under `tests/` is a fixture set, not
  a program. `tests/red/` holds compiler rejection probes named
  `<expected-error-class>--<name>.ace`. `tests/strict/` holds E--
  sources that the strict compiler must reject or must compile.
  `tests/adjudication/` holds ledger-validator fixtures.
  `tests/queries/` holds query, answer, and trace fixtures.
  `tests/ui/` holds reviewer-interface fixtures. `tests/copy/` holds
  copy-register fixtures. Every case pins exact output bytes. One
  `tools/goal.py check` invocation beside `tools/regen.py --check` (the
  E-- → Python identity above) is the full acceptance gate. `check`:

  1. validates the compendium ledger — the `.agent/compendium.md`
     organizations table plus the `.agent/compendium.tsv` guideline
     rows: row vocabulary, canonical ordering, the one-active-row
     promotion invariant — and prints the terminal meter that the
     `/goal` exhaustion clause reads;
  2. validates layout, source records, and Prolog/lexicon inventory
     closure;
  3. validates every guideline's corpus ledgers — projection-notes
     header bytes and per-document totality, with each row's region
     resolving to the coverage row that carries its `ace(docid)`;
     coverage-ledger closure (row grammar, region totality against each
     evidence file's own payload census and locator inventory, ace-row
     ↔ document-set bijection, single-step restates targets), with a
     per-guideline `ace/restates/uncovered/pending` meter;
     review-manifest freshness against a full re-derivation, then
     adjudication-ledger validation with its verdict meter; lexicon
     liveness and minimality; v1-only product vocabulary;
  4. verifies that the resolved SWI-Prolog is exactly 9.2.9;
  5. recompiles every guideline twice (byte-determinism);
  6. compares the recompile against the committed Prolog (freshness);
  7. load-checks every compiled document;
  8. derives and discharges every document's proof obligations, alone
     and as one co-loaded composition whose manifest is pinned to the
     guideline's whole document set;
  9. scans that composition for left recursion;
  10. re-derives each committed query artifact twice and
      byte-compares it: compiled query, answer, and proof trace.
      Every trace node must join exactly one committed clause line;
  11. renders every reviewer page twice and byte-compares the two
      renders, replays the reviewer fixture corpus against pinned
      pages and pinned refusals, and scans every string the interface
      can emit against the clinician copy register;
  12. asserts that the compiler rejects each red probe with its named
      error class and exit status.

## Running

The pipeline requires SWI-Prolog 9.2.9 and Python ≥ 3.11. The supported
invocations use `python3 -P`. CI pins the SWI 9.2.9 container, whose
Debian Python is 3.11.

```sh
python3 -P tools/goal.py compile <guideline-id>   # ACE → Prolog
python3 -P tools/goal.py queries <guideline-id>   # query/answer/trace artifacts
python3 -P tools/goal.py check                    # corpus acceptance gate
python3 -P tools/regen.py --check                 # E-- → Python identity
python3 -P tools/ui.py serve [<port>]             # reviewer interface
python3 -P tools/ui.py render <outdir>            # static page export
```

## Reviewer interface

`tools/ui.py serve` starts the reviewer interface on `127.0.0.1`. It
lists every guideline, then every document, and shows each document
beside the exact source passage it was written from, its compiled
Prolog, and its decision history. A reviewer answers one question per
document: does the ACE representation appropriately reflect the
original passage? The answer, the reviewer name, and an optional
comment append to `guidelines/<id>/audit/adjudication.tsv`.

The interface reads committed files. When the working tree holds
uncommitted guideline changes, the pages render the last commit
instead. Three consequences follow. An uncommitted edit is not
reviewable, and it does not outdate an existing decision. A document
that was never committed is not listed. Every recorded decision names
the commit that wrote the ACE text the reviewer read. The decision
ledger is the one file the interface writes, so its own writes are not
uncommitted work. `tools/goal.py check` is the exception: it reads the
working tree, because it is the gate you run before you commit.

The write path is narrow and guarded. The interface binds the loopback
address only. It has no accounts and no authentication. A decision is
accepted only when the request carries the process token, the `Host`
header names the loopback listener, any `Origin` header matches it, the
subject digest still matches a fresh derivation from committed state,
and the ledger digest still matches the ledger on disk. A failed check
returns a refusal and writes nothing. The ledger write itself is a
compare-and-swap through a same-directory temporary file, a flush, an
`fsync`, and an atomic rename. The shared validator in `goal.py`
approves the new ledger bytes before the rename, so the gate and the
interface cannot drift apart. Pages carry no JavaScript.

Reviewer names are self-asserted and are not verified. A decision is
current when the document still matches the version it was recorded
against, and outdated otherwise. A document whose current version
carries both an approved and a rejected decision reads contested.

## Compiled Prolog schema (v1)

Status: **frozen**. Every compiled document emits
`guideline_schema_version(1).`. The predicate set below is the public
ABI: an extension requires a version bump. v1 is the compiler's
sole schema. A future version takes a new explicit invocation argument;
the compiler never infers a version from content. Authored questions
reject in document compiles; the separate question mode below compiles
one question into a query projection over the same vocabulary.

Sentences project onto a closed reserved vocabulary. Source words —
nouns, verbs, adjectives, prepositions — stay opaque data atoms and
never become predicate functors. The schema is therefore language- and
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
`exactly`, `na`. The compiler copies units and counts through as data.
It performs no arithmetic, no unit conversion, no counting, and no
modal inference: a cardinality clause records what the sentence said,
nothing more.

Every one of those values is reachable, but not from every position. An
upper-bounding determiner (`at most n`, `exactly n`, `less than n`) is
scoped. At the root of an asserted sentence, ACE groups the conditions
that the determiner binds into a condition list. v1 has no construct
for that scope, so the document rejects with
`unsupported, root_condition([…])` rather than flatten an upper bound
into an existential claim; `tests/red/unsupported--v1-root-at-most.ace`
pins the detail. The same phrase inside a rule or an operator box
carries no group and compiles, emitting `leq`, `exactly`, or `less`
under that context. `at least n` and `more than n` bound below, are
never grouped, and compile in any position.

`Context` is `actual` for asserted content, or a generated operator
context:

```
Context      ::= actual | '$guideline_id'(context, DocId, S, box(B), Deps)
Operator     ::= should | must | can | may | '-'
OperatorEdge ::= guideline_operator(OuterContext, InnerContext, Operator)
```

The compiler reifies modality and classical negation as data, never as
inference. Each DRS operator box becomes one `guideline_operator/3`
edge from its enclosing context to a fresh box context. The compiler
emits that edge before the box's payload clauses, which carry the box
context in their own `Context` argument. The operator atoms are the DRS
functors themselves (`-` is classical negation), so a non-English
frontend produces the same tags. ¬SHOULD(P) and SHOULD(¬P) stay
structurally distinct, and the compiler performs no deontic, modal, or
classical inference. `box(B)` numbers a sentence's operator boxes in
whole-sentence preorder. `Deps` follows the Skolem dependency rule:
empty for root facts, the antecedent's top-level referents for
consequent boxes. A consequent operator context is therefore a
per-antecedent-solution witness. In rule bodies, an operator box binds
an existential context variable — `guideline_operator(Outer, V, Op)`
plus payload goals that share `V`. Matching is Horn-monotone structure
satisfaction over whatever documents are loaded. Extra clauses under a
producer context therefore never block a match, and two body boxes may
bind one producer context whose payload satisfies both. A consumer that
wants unwrapped content only keeps `Context = actual`. An
operator-aware consumer adds one recursive walk over
`guideline_operator/3` edges (a `ctx_desc/2` descendant relation) to
collect content under a modality.

Negation-as-failure ("does not provably …") in a rule antecedent
compiles to an executable `\+ (…)` over the box's goals. Its scope is
the loaded aggregate: the engine evaluates `\+` against whatever
document set is co-loaded, so absence is corpus-relative, never
document-local. Negation-as-failure in a consequent or a root fact
rejects; assertion-by-absence is meaningless.

A universally quantified sentence becomes one Horn clause per
consequent condition. All clauses of a rule variant share one identical
rendered body. A nested `if … then if … then …` consequent curries into
one rule whose antecedent concatenates the nested domains. An
antecedent that contains one disjunction splits into two variants whose
bodies are the shared conditions plus that arm. The bundle keeps one
sentence comment, with variant-1 clauses and then variant-2 clauses,
contiguous. The compiler Skolemizes a referent introduced only in the
consequent to
`'$guideline_id'(product, DocId, SentenceId, ref(N), Deps)`. `Deps` is
the concatenation of every curried antecedent segment's top-level
referents in DRS order; a split variant appends its arm's top-level
referents. A referent introduced inside an operator or
negation-as-failure (NAF) box contributes nothing to any `Deps` list:
it is consequent-inaccessible and stays a box-local existential. `N` is
the referent's first-occurrence position in the sentence's unsplit
expansion. Both variants of a split mint the same `ref(N)`,
distinguished by `Deps`. The constructor's first argument names the
term's role: `context` for an operator box, `product` for a consequent
referent, `witness` for a proof instance (below). All three name the
compiler's own chosen stand-in — never a source-given name, real-world
identity, or uniqueness claim. Because facts and derived heads share
one vocabulary, a rule body consumes another document's facts or heads
by ordinary resolution.

Every v1 document opens with the same declaration block: `multifile`
and `discontiguous` for all nine indicators, whether or not it
populates them. After the block come `guideline_schema_version(1).`,
the document record, and then, per sentence, a
`% S<n>: <verbatim sentence>` comment with that sentence's contiguous
clauses in deterministic order. Any subset of v1 documents therefore
co-loads into one engine in any order with no warnings. Names that
begin with `guideline_` and the identity constructor `'$guideline_id'`
are reserved: a colliding name rejects the document rather than
silently shadow schema vocabulary. The compiler scans both the parsed
sentence and every lexicon entry — used or not — for one.
`guideline_part/3` is reserved for a later unit and stays unemitted in
v1. Shapes still outside the schema (group coordination, disjunctive
consequents, rules scoped inside an operator) reject with a canonical
`ace_to_pl_error(unsupported, …)` line. A question inside a document
rejects with `question_not_supported(Form, S)`, where `Form` is
`wh(Tag)`, `universal`, or `yesno`.

Compilation is checked, not asserted. Every v1 compile derives the
document's proof obligations and discharges them before it emits
anything. Appending `proof` to the invocation prints the derivation
payload in place of the document. The payload is one
`'$guideline_proof'(DocId, S, variant(K), witness(Facts), prove(Heads))`
term per fact group and per rule variant. `Facts` is that group's own
positive body under a witness substitution; NAF boxes contribute
nothing, by design. `Heads` is that group's clause heads under the same
substitution. The compiler asserts each witness ahead of the document's
rule clauses, proves every head under bounded search, then retracts.
The bounds are depth 4000 inside 1,000,000 inferences; a search that
exceeds either bound counts as underivable. A group whose heads stay
non-ground under its own witness rejects with
`proof, nonground_obligation(…)`. A group whose heads do not derive
rejects with `proof, underivable_obligation(…)`. Both exit 1. Witness
terms carry the document's own document, sentence, and variant
coordinates, so no foreign clause can discharge an obligation. Success
is modus ponens over that sentence's own projection.

`aggregate-check <manifest>` replays those obligations across a
composition. The manifest is strict: one `<compiled-pl>` TAB
`<payload>` row per document, with the final newline required. A 0-byte
file means the empty composition. The compiler loads every document
into one engine; any load diagnostic is a failure. It checks that the
distinct `guideline_document/3` records equal the manifest row count
and that the loaded schema-version set is exactly `[1]`. It then
re-derives every obligation against the whole batch. Co-loading can
therefore neither break a document's derivations nor silently repair
them, and the engine evaluates negation-as-failure against the
composition it will actually run in. The check reports
`ace_to_pl aggregate ok <N> documents <G> obligations`.

`recursion-check <manifest>` loads that same composition and ignores
the payload column. It proves that no clause head unifies with its own
leftmost body goal, renaming the goal apart first the way SLD renames a
clause. A match rejects as `proof, left_recursive(Site, Name, Arity)`,
where `Site` is the offending clause's own `sentence(DocId, S)` when it
carries one. The scan is structural, so its verdict is load-order
independent. It reports
`ace_to_pl recursion ok <N> documents <M> rule clauses`; the count
names what it read.

Payloads are evidence, not authority: the gate checks `<G>` against the
composition rather than take it from the payload stream. Before the
replay proves any head, four conditions must hold:

- The obligation set must cover exactly the sentence identities that
  the loaded documents themselves carry (`missing_obligation` /
  `extra_obligation`).
- Its `(document, sentence, variant)` keys must be unique
  (`duplicate_obligation`), with variants numbered `1..K` per sentence
  (`variant_sequence`).
- No group may prove an empty head list (`empty_obligation`).
- A nonempty payload file must end in the newline that its last term
  wrote (`check_load, payload_bytes`).

An emptied, truncated, repeated, or misattributed payload therefore
fails the replay instead of shrinking it. Manifest rows bind product to
payload for provenance; replay soundness rests on that composition-wide
coverage. One residue is open by construction: dropping the LAST
variant of a multi-variant sentence leaves coverage complete and
variant numbering contiguous. The frozen ABI carries no per-sentence
variant count to check it against.

Compiled documents are a definite-clause program, so termination is the
consuming engine's responsibility, not a property that the schema can
promise. Facts and derived heads share one vocabulary by design; that
is what lets one document's rules consume another's clauses. The same
sharing lets an authored rule whose consequent entity feeds its own
antecedent form a cycle. Under naive SLD such a corpus can diverge in
one clause order and succeed in another. An engine with tabling or
bottom-up evaluation is immune. This repository's own gates are
fail-closed against divergence: the check proves every obligation under
the bounded search above. Those bounds cut a divergent branch, and the
check reports it as a failed obligation instead of hanging. Left
recursion — a head that unifies with its own leftmost body goal — is
the shape that makes an open query's termination depend on load order.
The gate checks it rather than asserts its absence. Every
`tools/goal.py check` runs `recursion-check` over the loaded
composition and rejects one, printing the rule-clause count that it
scanned. The shipped corpus holds none.

Authoring notes (v1):

- State guideline knowledge and nothing else in an ACE document: no
  witness seed facts, no proper-name stand-ins, no authored probe
  queries. The compiler derives its own obligations.
- Put reusable domain vocabulary, not per-statement fixtures, in
  `lexicon.ulex`.
- Keep every ulex lexeme reachable. An entry whose lexeme no document
  uses rejects. So does an entry repeated in the file, or one that the
  vendored Clex already provides byte-for-byte.
- Write one statement per sentence: split "…, and if A then B" into
  separate sentences.
- Keep quantified restrictors out of modal complements. A universally
  quantified rule nested inside a modal box rejects
  (`operator_scoped_rule`). State the quantification in the antecedent
  and let the modal box wrap only the consequent.
- A prepositional phrase attaches to the verb.
- Modifying a noun with `of` rejects
  (`condition_shape(relation(A,of,B))`). Rewrite
  `every clinician of a clinic` either as a relative clause,
  `every clinician who works at a clinic`, or as one compound noun with
  its own lexicon entry, `every clinic-clinician`.
- Count nouns take an explicit determiner.
- `for` as a preposition needs a `prep(for,for)` user-lexicon entry.
- An indefinite consequent entity that repeats the noun of the
  antecedent's first condition (`If a patient … then a patient …`)
  mints a fresh referent. That referent's clause head unifies with its
  own leftmost body goal: the document compiles, and `check` then
  rejects the composition as `proof, left_recursive(…)`. Refer back to
  the antecedent referent (`the patient`), or give the consequent
  entity its own noun.

### Question projection

The compiler's `question` mode compiles one ACE question against the
same v1 vocabulary:

```sh
swipl -q -f none -F none -s vendor/ape/prolog/ace_to_pl.pl -g main \
  -t 'halt(9)' -- question <ape-tree-dir> <qid> [<ulex>]   # ACE on stdin
```

The input must be exactly one sentence line that parses to one root
question box. `<qid>` follows the docid grammar. Success emits four
lines. Line 1 is a generated-file comment. Line 2 is the ground
`'$guideline_query'(v1, Qid, ace_sha256(H), ulex(none | sha256(H)))`
record. Line 3 is a `% Q1:` comment with the verbatim sentence. Line 4
is one `'$guideline_query_projection'(goal(Conj), answers(Manifest))`
term. `Conj` is an explicit `','/2` conjunction of v1 goals rendered
through the rule-antecedent pipeline. The root context is `actual`,
and referents stay variables. A modal box contributes its
`guideline_operator/3` edge before its payload goals, under a shared
existential context variable. `Manifest` lists one `answer(Var, Desc)`
row per wh-placeholder in source order. `Desc` is `noun(Noun, Class)`
when exactly one same-box `object/6` types the placeholder; otherwise
it is `wh(who)` or `wh(what)`. Goal and manifest share variables
inside the one projection term.

Supported forms: `wh(who | which | what)` and yes-no questions over
conjunctive v1 content, with modal boxes at any nesting of themselves.
Structural and blocker failures reject with class `unsupported` and
exit 1. `query_expected(S)` means that no root question exists.
`query_sentences(N)` means that the input is not exactly one sentence.
`query_unsupported(Blocker, S)` names a blocking construct. Blockers
are `universal`, `disjunction`, `classical_negation`, `naf`, and the
unsupported tags `wh(howm | how | where | when)`. A foreign leaf
rejects as `leaf(Name/Arity)`; a malformed placeholder rejects as
`marker(_)`. Inherited
parser, input, and load failures keep their own classes, details, and
exit codes. The projection emits no document
indicators, no declarations, and no proof obligations, and it never
participates in `check`, `aggregate-check`, or `recursion-check`
compositions.

### Query answers

The compiler's `answer` mode solves one compiled query against a
loaded composition:

```sh
swipl -q -f none -F none -s vendor/ape/prolog/ace_to_pl.pl -g main \
  -t 'halt(9)' -- answer <manifest> <query-pl>
```

`<manifest>` uses the aggregate manifest grammar. The payload column
must name readable files, and the mode never parses payload terms. A
0-byte manifest is the empty composition: the mode still declares the
v1 indicators and solves against no clauses. `<query-pl>` must read
as exactly two terms in order: the ground `'$guideline_query'/4`
record, then the `'$guideline_query_projection'/2` term. The mode
reads the query file as data and never consults it. Comments and
layout carry no meaning here; the `check` gate pins committed bytes
separately.

Success emits a two-line artifact on stdout: a generated-file
comment, then one ground term
`'$guideline_answers'(v1, Qid, query_sha256(H), result(R))`. `H` is
the SHA-256 of the raw query-file bytes. The solver runs under fixed
bounds: depth 100 and 100000 inferences per solution, and 1000000
inferences for the whole run. For a wh query (`answers` rows
present), `R` is `solutions(Sols)`, or `indeterminate(limit)` when a
bound trips. Each distinct solution contributes one `sol(Values)`
row; rows follow the standard order of terms, and values follow the
answer-manifest order. For a yes-no query
(`answers([])`), the first proof is conclusive: `R` is `yes`,
`no(finite_failure)` on exhaustive failure, or `indeterminate(limit)`
when a bound trips before any proof. A nonground solution rejects
with class `proof` as `nonground_solution(Qid)`. A malformed query
file rejects with class `check_load` as `query_file(<why>)`. Manifest
and composition load failures keep the aggregate classes and details.

Committed query artifacts live under `guidelines/<id>/queries/`:
`<qid>.ace` sources at the root, compiled queries under `pl/`, answer
artifacts under `answers/`, and proof traces under `traces/`.
`python3 -P tools/goal.py queries <id>` derives every artifact in
memory and writes only after every derivation succeeds. `check`
re-derives each file per query twice and compares the bytes. A
committed answer must be `yes` or nonempty `solutions(...)`;
committed queries stay demonstrations. The `queries` and `check`
commands bound every compile, answer, and trace subprocess at 30
seconds of wall-clock time. A run that exceeds the bound fails the
gate. The direct `swipl` invocation above has no process bound.

### Proof traces

The compiler's `trace` mode re-proves the positive claims of one
answer artifact against a loaded composition:

```sh
swipl -q -f none -F none -s vendor/ape/prolog/ace_to_pl.pl -g main \
  -t 'halt(9)' -- trace <manifest> <query-pl> <answers-pl>
```

The manifest and query file follow the answer-mode rules.
`<answers-pl>` must read as the answer artifact for the same query:
one ground `'$guideline_answers'` term. The version, qid, and query
hash must match. The result must fit the query's mode. A malformed
or mismatched answers file rejects with class `check_load` as
`answers_file(<why>)`.

Success emits a two-line artifact on stdout: a generated-file
comment, then one ground term `'$guideline_traces'(v1, Qid,
query_sha256(H), answers_sha256(A), result(R))`. `A` is the SHA-256
of the raw answers-file bytes. `R` mirrors the answer result with a
proof in place of each positive claim. `yes` becomes `yes(P)`. Each
`sol(Values)` row becomes `sol(Values, P)` in file order.
`no(finite_failure)` and `indeterminate(limit)` carry no positive
claim and stay verbatim.

`P` is `proved(Nodes)`, `unproved(finite_failure)`, or
`unproved(limit)`. Each proof node is `clause(sentence(DocId, S),
clause_sha256(Hex), Children)`. It names the clause that resolved
the goal by document and sentence number. `Hex` is the SHA-256 of
that clause's rendered document line with its newline. A re-checked
negation-as-failure goal freezes as a `naf(Goal)` leaf among a
clause node's children. Root nodes are always clause nodes. The
prover is a directed first-proof interpreter over the loaded
clauses. Each row's search runs under depth 1000 and 100000
inferences. The whole run is bounded at 1000000 inferences. The
interpreter's depth measure is its own; it is not comparable with
the answer mode's engine measure. A negation site re-checks under
the same bounds. A bound that trips inside a negation makes the
whole row `unproved(limit)`, never a false failure. When the
whole-run bound trips, the result becomes `indeterminate(limit)` in
place of the mirror. The direct `swipl` invocation has no process
bound.

`check` re-derives each committed trace twice, compares the bytes,
and resolves every `clause_sha256` to exactly one clause line of the
committed document under `pl/`. A committed trace must prove its
answer: `yes(proved(...))`, or solution rows that are all proved.
`check` prints one `goal: traces <id> <n> traces; nodes=<k>` meter
per guideline beside the queries meter.

## Operating

Guideline work runs as goal rounds in a Claude Code session opened at
the repository root, driven by the built-in `/goal` stop-condition
command:

```
/goal Process American clinical guidelines through the pipeline as described in README.md "Operating", fanning bulk work out to teammates per .agent/rounds.md: work the in-progress source document to full coverage before fetching the next; done only when either (a) the user has asked to stop, pause, or wind down — the request alone meets this goal at any point, even mid-round with the worklist unfinished; start nothing new, state where work stands, and stop — or (b) every fetched guideline is complete, every remaining .agent/queue.md entry is a recorded blocker, and the compendium exhaustion clause in .agent/compendium.md "Protocol" holds: every guideline row of .agent/compendium.tsv done, blocked, or excluded, and every organization row terminal.
```

The goal re-arms each time Claude tries to stop, and it survives
session resume, so halting at any moment is safe. The repository is the
only persistence. Every round starts by deriving state from it:
`.agent/queue.md`, `git status`, `tools/goal.py check`, and the
in-progress guideline README's coverage statement. The round then
finishes or discards incomplete work before it takes on anything new. A
user request to stop is condition (a) of the goal itself. "Let's stop
here" satisfies the stop check at once, mid-round included, rather than
re-arm against the unmet exhaustion clause. The wind-down is to start
nothing new, state where work stands, and stop. `/goal clear` remains
the unconditional disarm. The check's first stage validates the
compendium — the `.agent/compendium.md` organizations table plus
`.agent/compendium.tsv`: row format and vocabulary, canonical ordering,
the single-active-row promotion invariant. It prints a terminal meter —
remaining non-terminal organizations and unfinished guideline rows —
that measures the exhaustion clause directly. Corpus validation
(projection-ledger totality, coverage-ledger closure with its
region-status meter, lexicon liveness and minimality, v1-only product
vocabulary) runs for every guideline on every check. The same pass
re-derives each guideline's review manifest and validates its
adjudication ledger. It also re-derives every committed query,
answer, and proof trace, and it joins each trace step to its
committed clause line. Bulk work (source
reading, extraction drafting, ACE drafting, adversarial review) fans
out to subagent teammates per `.agent/rounds.md`. The session lead
alone writes the repository and commits.

Reviewer verdicts live in each guideline's `audit/adjudication.tsv`,
pinned to content digests in `audit/review-manifest.tsv`. When compiled
content changes, regenerate the manifest with
`python3 -P tools/goal.py review-manifest <id>`. Commit the round
before you open the reviewer interface, because the interface reads
committed files and shows nothing else. Commit the ledger and
the manifest together after each review batch; git history is the
verdict history. Rejected and stale counts in the adjudication meter
are worklist entries, not check failures. Live rejected documents
outrank extraction, fetches, and all other new work.

Exactly one source document is in progress at a time. That document
reaches completion — across as many rounds as it takes — before the
round fetches the next one. A source document is complete when three
conditions hold:

- Every normative statement in it is extracted verbatim into `source/`
  evidence.
- Every extracted statement is either authored as knowledge-only ACE
  and compiled, or recorded in the guideline README as uncovered with a
  reason. Compiled means that the obligations discharge alone and in
  aggregate.
- `python3 -P tools/goal.py check` is green, with the guideline's
  coverage meter reading `pending=0`.

While a document is in progress, a round advances it one increment:

1. **Extract** — extract the next batch of normative statements,
   verbatim, into `guidelines/<id>/source/` evidence files.
2. **Normalize** — author one ACE document per ruled source region
   under `ace/`, plus the shared `lexicon.ulex`. Each document holds
   one or more source-anchored knowledge sentences (supported
   constructs: the header of `vendor/ape/prolog/ace_to_pl.pl`). Each
   ACE document is a minimal faithful projection of its region; the
   projection-notes ledger and the guideline README's coverage
   statement use that same unit.
3. **Compile** — run `python3 -P tools/goal.py compile <id>`. When the
   compiler rejects a document, adjust the ACE or the lexicon first.
   Only a genuinely new construct extends the `ace_to_pl.pl`
   translation, minimally. Such an extension keeps totality: every
   sentence compiles to a determinate clause bundle whose obligations
   discharge, and unrecognized shapes reject. It also adds `tests/red/`
   probes for the new rejection boundary.
4. **Close** — `python3 -P tools/goal.py check` must be green; when E--
   sources changed, `python3 -P tools/regen.py --check` must be green
   too. Make a scoped commit. Update `.agent/queue.md` and the
   guideline README's coverage statement.

When no document is in progress, the round fetches the next source: the
queue's next entry. When the queue runs dry, it refills from
`.agent/compendium.tsv` per `.agent/compendium.md` § Queue promotion.
Acquire the document and persist it immutably under
`guidelines/<id>/source/` before any generative processing. Record URL,
retrieval date, SHA-256, byte length, and a rights quote in
`guidelines/<id>/README.md` (model: `guidelines/cdc-2022-opioid/`).
Commit the source record on its own, so that it survives any halt. Ids
follow `[a-z0-9-]+`; choose an id once per source. An already-fetched
URL keeps its recorded id. Changed remote content becomes a new
versioned id, while every recorded source stays immutable. A paywall or
rights gate becomes a recorded queue blocker, and the round moves on.

## Licensing

First-party work outside `vendor/` is Apache-2.0 WITH LLVM-exception
(`LICENSE`). First-party additions inside a vendored tree adopt that
tree's license; `vendor/ape/prolog/ace_to_pl.pl` is LGPL-3.0-or-later.
Vendored trees keep their own licenses: `vendor/e--` Apache-2.0,
`vendor/ape` LGPL-3.0-or-later, `vendor/clex` GPL-3.0-or-later.
Distribution of the full repository combination conveys under GPLv3.
First-party work remains Apache-2.0 WITH LLVM-exception and
independently reusable. See `NOTICE`.
