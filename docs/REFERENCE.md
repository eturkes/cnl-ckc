# cnl-ckc technical reference

This file is the technical reference for developers and for the agents
that maintain the corpus. The root `README.md` is the simplified
overview.

cnl-ckc is a minimal, fully auditable pipeline. It turns clinical
guidelines into executable Prolog through controlled natural language:

1. **Fetch** — acquire a guideline source document and record it under
   `guidelines/<id>/source/`. Digests, rights, and attribution go in
   that guideline's `README.md`.
2. **Normalize** — author the recommendations as Attempto Controlled
   English (ACE), one document per ruled source region, under
   `guidelines/<id>/ace/`. Domain vocabulary lives in
   `guidelines/<id>/lexicon.ulex`.
3. **Compile** — `ckc compile` stages the vendored APE parser and
   overlays `vendor/clex/clex_lexicon.pl` as the general-English base
   lexicon. It compiles every ACE document to plain Prolog under
   `guidelines/<id>/pl/` via `vendor/ape/prolog/ace_to_pl.pl`.

A Claude Code session drives all three steps from one pasted prompt
("Operating" below).

## Audit story

Every program artifact that a human must read is controlled natural
language, a direct compilation of it, or part of one small named
compiler base:

- **ACE → Prolog, certified.** All guideline Prolog (`guidelines/*/pl/`)
  compiles from ACE. A compiled file quotes each source sentence beside
  its clauses, and it carries the SHA-256 of the exact ACE and lexicon
  bytes that produced it. The compiler also derives each sentence's
  proof obligation and discharges it against the document's own
  clauses, alone and co-loaded with the rest of the batch. A committed
  query answer ships with a proof trace. Each clause application in that
  trace names one committed clause line by SHA-256. `ckc certify
  <guideline>` then certifies every committed document and query: it
  parses the ACE again with the upstream APE parser, and it checks that
  the committed clauses and each query's projection are exactly what
  the specification `rust/ckc-spec/src/emit.rs` states for that parse,
  that the custody digests match, and that the proof obligations replay
  through the verified engine. The check is machine-verified against the
  committed specification under a pinned verifier TCB (see
  `.claude/rules/rust.md`). The emitter `vendor/ape/prolog/ace_to_pl.pl`
  produces the artifacts, but no human needs to read it: the
  specification and the certifier's verdict are the audit surface.
  `tests/certify/cases.tsv` holds 17 hostile edits of committed
  artifacts that the certifier must reject; `just certify` runs the
  corpus certification and that battery.
- **A verified kernel checks every logical step.** All first-party program logic except the ACE emitter (`vendor/ape/prolog/ace_to_pl.pl`, whose output `ckc certify` certifies) lives in `rust/`: a short, human-read formal
  specification (`rust/ckc-spec/`), an implementation with
  machine-checked proofs (`rust/ckc-kernel/`, never read by a human),
  and a thin shell (`rust/ckc/`) that moves bytes between the file
  system, SWI-Prolog, the loopback socket, and the kernel.
  `cargo verus verify --no-cheating` under the pinned verifier
  (`rust/verus.lock`) proves every kernel entry point against its
  specification: clause-file acceptance, the composition modes, query
  answers, proof traces and trace checks, the corpus validators, the
  release manifest, emission certification, and the reviewer pages.
  `ckc trust-audit` pins the trusted surface byte for byte
  (`rust/trust/spec-manifest.tsv`), enumerates every escape site, and
  rejects any dependency outside the allowlist. The claim is
  "machine-verified against the committed specification under a
  pinned verifier TCB", nothing stronger: git, the file system, subprocesses, the clock, the loopback socket and HTTP request plumbing, archive assembly, the process and legal validators (fork-notice policy, compendium format, rights profiles), the pinned dependencies and SWI-Prolog stay trusted software under fixture gates. The trace theorem certifies each negation leaf by the finite failure of the negated goal exactly as it was called. The trace prover cuts a negation call that is not safe, so such a call never becomes a leaf.
- **Review is recorded, not asserted.** A reviewer decision names the
  exact bytes it judged. Each ledger row pins a bundle digest over the
  document's ACE text, its coverage row, its source region payload, and
  its compiled clauses. The reviewer interface reads committed files
  only, so what a reviewer approves is what the repository holds.
  "Reviewer interface" below states the workflow.
- **The compiler base is closed and named.** One vendored fork performs
  the ACE compilation, and a human must read only its upstream parser.
  `vendor/ape/` holds the ACE parser plus the
  first-party `prolog/ace_to_pl.pl` emitter. The emitter is outside the
  human-read trust story: `ckc certify` certifies its output against
  `rust/ckc-spec/src/emit.rs`, and the trusted part of that tree is the
  upstream parser, run as a separate pinned SWI-Prolog process by the
  thin driver `rust/ckc/prolog/drs_dump.pl`, which never loads the
  emitter. The tree is pruned to its load closure.
  `vendor/*/PROVENANCE` records upstream, fork base, import commit,
  license, first-party inventory, and trust boundary. Git history is
  the change record, and `ckc check` reads it. A vendored file counts as touched when a commit after the
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
  `<expected-error-class>--<name>.ace`. `tests/green/` holds compiler
  acceptance probes; each `.expect` file pins the compiled document or
  question. `tests/adjudication/` holds
  ledger-validator fixtures. `tests/queries/` holds query, answer, and
  trace fixtures. `tests/ui/` holds reviewer-interface fixtures.
  `tests/copy/` holds copy-register fixtures. `tests/certify/` holds
  certification mutants that `ckc certify --cases` must reject.
  `tests/v1/`, `tests/align/`, `tests/align-probes/`,
  `tests/check-mutants/`, and `tests/dist/` hold the clause-file,
  alignment, `ckc check`, and export batteries that `cargo test`
  replays. Every case pins exact output bytes. `tests/consumer/` holds
  the patient worlds and drivers of the consumer-recipe check
  ("Loading a whole knowledge base"). `just check` is the
  full acceptance gate, with `just certify` beside it. It runs
  `.agent/spec-lint`, `ckc check`, `ckc ui check`, and a fresh
  compile whose output must equal the committed corpus.
  `.agent/spec-lint` checks the shape of `.agent/spec.md`, the
  `Phase` section included. `ckc check`:

  1. validates the compendium ledger — the `.agent/compendium.md`
     organizations table plus the `.agent/compendium.tsv` guideline
     rows: row vocabulary, canonical ordering, the one-active-row
     promotion invariant — and prints the terminal meter that the
     corpus prompt's exhaustion clause reads. It then checks the shape
     of `.agent/spec.md`: the five sections in order, each `Tasks` row
     open (`- [ ]`) or ticked with its commit (`- [x] <sha>`), and the
     `.agent/deferred.md` pointer as the last `Tasks` line;
  2. validates layout, source records, and Prolog/lexicon inventory
     closure;
  3. validates every guideline's vocabulary table grammar first, then
     its corpus ledgers — projection-notes
     header bytes and per-document totality, with each row's region
     resolving to the coverage row that carries its `ace(docid)`;
     coverage-ledger closure (row grammar, region totality against each
     evidence file's own payload census and locator inventory, ace-row
     ↔ document-set bijection, single-step restates targets), with a
     per-guideline `ace/restates/uncovered/inexpressible/pending` meter;
     review-manifest freshness against a full re-derivation, then
     adjudication-ledger validation with its verdict meter; lexicon
     liveness and minimality; the schema's product vocabulary;
  4. verifies that the resolved SWI-Prolog is exactly 10.0.2;
  5. recompiles every guideline twice (byte-determinism);
  6. compares the recompile against the committed Prolog (freshness);
  7. load-checks every compiled document;
  8. derives and discharges every document's proof obligations, alone
     and as one co-loaded composition whose manifest is pinned to the
     guideline's whole document set;
  9. scans that composition for left recursion;
  10. re-derives each committed query artifact twice and
      byte-compares it: compiled query, answer, and proof trace.
      Every clause node of a trace must join exactly one committed clause
      line (a comparison leaf names no clause);
  11. asserts that the compiler rejects each red probe with its named
      error class and exit status, and that it compiles each green
      probe to its pinned bytes.

  `ckc ui check` renders every reviewer page twice, byte-compares the
  two renders, and checks page invariants and link closure. The
  kernel's copy theorem covers every string the interface can emit,
  and `cargo test` replays the reviewer fixture corpus against pinned
  pages, pinned refusals, and the clinician copy register.

## Running

The pipeline requires SWI-Prolog 10.0.2 and the pinned Rust toolchain
that `just tools` installs; `just build` writes the `ckc` program to
`rust/target/release/ckc`. `SWIPL` names the SWI-Prolog program (default:
`swipl` on `PATH`). CI pins the SWI 10.0.2 container and runs a
statically linked `ckc` inside it. `just swipl` builds SWI-Prolog 10.0.2
from source into the toolchain directory. The recipe verifies the source
archive against its pinned SHA-256 digest. It skips the build when the
installed copy came from that archive.

```sh
ckc compile <guideline-id>          # ACE → Prolog (+ guidelines/schema.pl)
ckc queries <guideline-id>          # query/answer/trace artifacts
ckc check                           # corpus acceptance gate
ckc certify <guideline-id>          # emission certification against emit.rs
ckc ui serve [<port>]               # reviewer interface
ckc ui render <outdir>              # static page export
ckc ui check                        # interface self-check
ckc release-manifest                # refresh release-manifest.tsv
ckc dist build [<outdir>]           # release archive
just gate                           # every per-push CI check
```

## Reviewer interface

`ckc ui serve` starts the reviewer interface on `127.0.0.1`. The
server is a standard-library loopback socket in the `ckc` shell, and
every page byte comes from the verified renderer. The interface
lists every guideline, then every document. Each document appears
beside the exact source passage it was written from, its compiled
Prolog, and its decision history. A reviewer answers one question per
document: does the ACE representation appropriately reflect the
original passage? The answer, the reviewer name, and an optional
comment append to `guidelines/<id>/audit/adjudication.tsv`.

The pages fit the screen of a phone. Each page sets its layout width
to the width of the screen. On a screen narrower than 40rem, a table
that is wider than the screen scrolls sideways inside its own box,
and the rest of the page keeps the width of the screen. Printed pages
do not change. `just mobile` loads every rendered page at the width of
a phone in a pinned headless browser. The check fails when a page is
wider than the screen or when a wide table cannot show its last
column.

The document page links the two texts through an authored alignment.
The author of the ACE text records which part of the passage each
ACE term came from, in `guidelines/<id>/align/<docid>.tsv`. The page
highlights each linked pair in one shared color, so "category-B"
in the ACE text and "category: B" in the passage read as one unit.
One link can join several places in each text, and a link can cover
part of a hyphenated ACE term. Text without a recorded link stays
plain. The palette holds twelve muted tints, so colors repeat on
pages with many links. A pointer over a highlight emphasizes that
link exactly. A click on a highlight dims the rest of both texts, so
one link reads alone. A second click, or the pointer leaving the
highlight, restores the pointer behavior. A checkbox above the texts
turns the highlighting off; it starts on. ACE function words render
in a muted color, so the domain terms stand out. A document without
an alignment file renders without highlights. On paper the highlights
print as dotted underlines, and the click emphasis does not apply.

The alignment file is data, not a claim the gate can judge: the gate
verifies structure only. Each row is `group`, `side` (`src` or
`ace`), `start` (a character offset), and `span` (the verbatim
text). Every span must match its text at the offset, spans on one
side must not overlap, and every group needs at least one span on
each side. A file that breaks these rules stops the render. Whether
an alignment is helpful is a judgment for the reviewer, who sees it
on the page beside the texts it describes.

The guideline page lists the meanings of the grade, evidence, preference,
alternative, qualifier and scope words of `vocabulary.tsv` in the table
"Word meanings", after the status table. Each row names one word and
how the compiler reads it in every document of the guideline, for
example `category-B-recommendation` as a conditional recommendation. On
a document page, each such word in the ACE text has a thin solid
underline. Select the word with a click or a tap, or reach it with Tab
and press Enter or Space. The line under the ACE text then shows its
reading, for example "In this guideline, the compiler reads usually as
a qualifier: How often." Select the word again, or press Escape, to show
the hint again. The line states how the table reads the word. It does
not state that this occurrence compiled that way. The two scope words
have their reading only in the sentences that the compiler read as
scope exclusions. Time words have no reading, because they mean what
they say. Every typed fact reads in the ACE text itself, so no page
repeats the compiled tables. A table that fails the `ckc check` grammar
stops the interface with `ui: viewmodel: <id> vocabulary.tsv: <why>`.

The interface reads committed files. When the working tree holds
uncommitted guideline changes, the pages render the last commit
instead. Three consequences follow. An uncommitted edit is not
reviewable, and it does not outdate an existing decision. A document
that was never committed is not listed. Every recorded decision names
a commit that holds the reviewed bundle. The page carries the commit
that it was rendered from, and the form sends that commit back with the
decision. The interface derives the bundle at the posted commit again
and accepts the decision only when that bundle carries the reviewed
digest. The posted commit must be the current commit or one of its
ancestors. `ckc check` derives the bundle again at the recorded commit
and requires the recorded digest. The decision ledger is the one file
the interface writes, so its own writes are not uncommitted work.
`ckc check` is the exception: it reads the working tree, because it is
the gate you run before you commit.

The interface binds the loopback address only, and it has no accounts.
Each start of `ckc ui serve` draws a new random key. The command writes
a sign-in page that only its own account can read, and it opens that
page in the browser. The page gives the key to the browser as a cookie.
The key never appears on a command line or in the terminal while a
browser opens; when no browser opens, the terminal shows the sign-in
address. Every request must carry the cookie. The interface refuses a
request without it, or with a foreign `Host` header, before it reads
anything else. It never reads a request body larger than 64 KiB, and
each connection has 10 seconds to send its request. Browsers send a
cookie for 127.0.0.1 to every port of that address. Another account on
the same computer can therefore capture the key if the reviewer opens a
page that this account serves on the computer.

The write path is narrow and guarded. The interface accepts a decision
only when every guard passes. The request must
carry the process token and a loopback `Host` header, and any
`Origin` header must match it. The subject digest must still match a
fresh derivation from committed state and the derivation at the posted
commit. The ledger digest must still match the ledger on disk. A failed
check returns a refusal and writes nothing. The ledger write itself is a
compare-and-swap through a same-directory temporary file, a flush, an
`fsync`, and an atomic rename under an exclusive lock, and the
verified ledger validator that the gate uses approves the new ledger
bytes before the rename, so the gate and the interface cannot drift
apart. Pages carry one fixed script, which
drives the click emphasis on highlights and the line of word readings. The `Content-Security-Policy`
header names that script by hash, so the browser runs no other
script, and the page check rejects any other script element. The same
header forbids other sites to show the pages inside a frame, and it
lets the form send a decision to the interface only.

Reviewer names are self-asserted and are not verified. A decision is
current when the document still matches the version it was recorded
against, and outdated otherwise. A document whose current version
carries both an approved and a rejected decision reads contested.

## Compiled Prolog schema

Status: one schema with the identifier `1`. Every document and every
question compiles under it. Until the first publication the schema
evolves in place: each revision recompiles the whole corpus and every
fixture, and the identifier stays `1`. From the first publication on,
schema `1` is frozen. A change to the predicate set or to the meaning
of a predicate makes the next identifier, `2`, for every document and
question at once, and the release notes name the change. A consumer
combines documents of one identifier only. The predicate set below is
the public interface. An optional per-guideline table, `vocabulary.tsv`,
adds the time annotations of the section "Vocabulary table and time
annotations" and the grade and preference annotations of the section
"Strength annotations" below. The compiler takes the table from an explicit
invocation argument and never infers it from content. Authored
questions reject in document compiles; the separate question mode below
compiles one question into a query projection over the same vocabulary.

Sentences project onto a closed reserved vocabulary. Source words —
nouns, verbs, adjectives, prepositions — stay opaque data atoms and
never become predicate functors. The schema is therefore language- and
domain-neutral:

| Predicate | Meaning |
| --- | --- |
| `guideline_schema_version(Version)` | schema identifier of this document (`1`) |
| `guideline_document(DocId, ace_sha256(H), ulex(none \| sha256(U)), vocabulary(none \| sha256(T)))` | document record + source digests |
| `guideline_entity(Context, Ref, Noun, Class)` | an entity referent and its noun/class |
| `guideline_cardinality(Context, Ref, Unit, Comparison, Count)` | that referent's quantity payload, verbatim (a formula target: the computed result, section "Formulas in ACE") |
| `guideline_event(Context, EventRef, Lemma)` | an event referent and its verb |
| `guideline_arg(Context, EventRef, Position, Ref)` | participant at `Position` (1-based, DRS order) |
| `guideline_pp(Context, EventRef, Preposition, Ref)` | prepositional attachment on the event |
| `guideline_property(Context, PropertyRef, Lemma, Polarity)` | adjectival property (`pos`) |
| `guideline_operator(OuterContext, InnerContext, Operator)` | reified modal/negation wrapper: `InnerContext`'s content stands under `Operator` relative to `OuterContext` |

The six time annotations (`guideline_interval/6`,
`guideline_recurrence/4`, `guideline_frequency/5`, `guideline_order/4`,
`guideline_recurrence_window/7`, `guideline_range/3`), the three
strength annotations (`guideline_grade/5`, `guideline_preference/4`,
`guideline_qualifier/4`), the unit annotation `guideline_unit/3` and
the scope exclusion record `guideline_exclusion/1` complete the
vocabulary. The comparison built-ins
`guideline_satisfies/4` and `guideline_satisfies/6` test the conditions
of rules and questions; the section "Comparison conditions" below defines
them. The arithmetic built-ins `guideline_product/6` and
`guideline_quotient/6` compute a quantity from other quantities; the
section "Arithmetic built-ins" below defines them. The section "Scope exclusions" below defines the exclusion record
and the guard that it adds to recommendation rules. The sections "Vocabulary table and time
annotations" and "Strength annotations" below define them. In the record, `H` is the SHA-256 of the raw ACE bytes,
`U` the SHA-256 of the guideline's user lexicon, and `T` the SHA-256
of its raw `vocabulary.tsv` bytes. `none` means that the file does not
exist.

`Comparison` is exactly one of `eq`, `geq`, `greater`, `leq`, `less`,
`exactly`, `na`, or `about` (an approximate time quantity, below). A
plain number (`eq`) states at least that many, as in ACE: `3 cars` allows
more cars. `exactly` states the count itself. The compiler copies units
and counts through as data and performs no modal inference: a cardinality
clause of a fact or a rule head records what the sentence said, nothing
more. The one exception is the target of a formula: its count is the
result of the arithmetic built-ins (section "Formulas in ACE"). A condition with a comparison compares; the
section "Comparison conditions" below states how.

Every one of those values is reachable, but not from every position. An
upper-bounding determiner (`at most n`, `exactly n`, `less than n`) is
scoped: ACE groups the conditions that the determiner binds into a
condition list. Inside a rule or an operator box, the compiler flattens
the list into that context and emits `leq`, `exactly` or `less`. At the
root of an asserted sentence, the compiler flattens the list only when
every upper bound among the root's own conditions is `exactly n` with n
of at least 1. Such a count implies that the counted things exist, so
the conditions hold as facts. A root `at most n`, `less than n` or
`exactly 0` admits zero, so the document rejects with
`unsupported, root_condition([…])` rather than claim that the thing
exists; `tests/red/unsupported--v1-root-at-most.ace` pins the detail.
`at least n` and `more than n` bound below, are never grouped, and
compile in any position.

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
consequent condition. All clauses of a rule variant share one logical body; each clause line renders it with its own canonical variable letters, so two lines of one variant are equal up to variable renaming. In a document whose vocabulary table names a scope, each `should`, `must` or `may` operator edge clause ends with one more goal, the scope guard (section "Scope exclusions"). A nested `if … then if … then …` consequent curries into
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

Every document opens with the same declaration block: `multifile`
and `discontiguous` for all 20 indicators, whether or not it
populates them. After the block come `guideline_schema_version(1).`,
the document record, and then, per sentence, a
`% S<n>: <verbatim sentence>` comment with that sentence's contiguous
clauses in deterministic order. Any set of documents therefore
co-loads into one engine in any order with no warnings. Names that
begin with `guideline_` and the identity constructor `'$guideline_id'`
are reserved: a colliding name rejects the document rather than
silently shadow schema vocabulary. The compiler scans both the parsed
sentence and every lexicon entry — used or not — for one.
`guideline_part/3` is reserved for a later unit and stays unemitted. Shapes still outside the schema (group coordination, disjunctive
consequents, rules scoped inside an operator) reject with a canonical
`ace_to_pl_error(unsupported, …)` line. A question inside a document
rejects with `question_not_supported(Form, S)`, where `Form` is
`wh(Tag)`, `universal`, or `yesno`.

Compilation is checked, not asserted. Every compile derives the
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
exceeds either bound counts as underivable. This per-document replay
runs in SWI-Prolog, whose `call_with_depth_limit` lets a negation
succeed when the depth bound cuts its inner search. A group whose heads stay
non-ground under its own witness rejects with
`proof, nonground_obligation(…)`. A group whose heads do not derive
rejects with `proof, underivable_obligation(…)`. Both exit 1. Witness terms carry the document's own document, sentence, and variant coordinates, so a witness never collides with another document's constructed terms. Success
is modus ponens over that sentence's own projection.

`ckc v1 aggregate-check <manifest>` replays those obligations across a
composition in the verified Rust engine. The manifest is strict: one `<compiled-pl>` TAB
`<payload>` row per document, with the final newline required. A 0-byte
file means the empty composition. The engine loads every document
into one composition; any load diagnostic is a failure. It checks that the
distinct document records equal the manifest row count. It then
re-derives every obligation against the whole batch. In this engine a
negation succeeds only when its inner search fails and no bound cut
that search: the replay never proves an obligation through a negation
that stays undecided in the loaded composition. Co-loading therefore cannot break a document's derivations, and the engine evaluates negation-as-failure against the composition it will actually run in. The replay does not isolate documents: a rule in one document can derive another document's obligation head, so the per-document replay above is the check that each document's own clauses suffice. The check reports
`ace_to_pl aggregate ok <N> documents <G> obligations`.

`ckc v1 recursion-check <manifest>` loads that same composition and
ignores the payload column. It proves that no clause head unifies with its own
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
  wrote (`check_load, payload_term(…)`).

An emptied, truncated, repeated, or misattributed payload therefore
fails the replay instead of shrinking it. Manifest rows bind product to
payload for provenance; replay soundness rests on that composition-wide
coverage. One residue is open by construction: dropping the LAST
variant of a multi-variant sentence leaves coverage complete and
variant numbering contiguous. The schema carries no per-sentence
variant count to check it against.

Compiled documents are a normal logic program (Horn clauses plus negation as failure), so termination is the
consuming engine's responsibility, not a property that the schema can
promise. Facts and derived heads share one vocabulary by design; that
is what lets one document's rules consume another's clauses. The same
sharing lets an authored rule whose consequent entity feeds its own
antecedent form a cycle. Under naive SLD such a corpus can diverge in
one clause order and succeed in another. Tabling or bottom-up evaluation removes that clause-order dependence when the recursion reaches finitely many terms; a recursion that builds ever deeper Skolem terms can still run without end. This repository's own gates are
fail-closed against divergence: the check proves every obligation under
the bounded search above. Those bounds cut a divergent branch, and the
check reports it as a failed obligation instead of hanging. Left
recursion — a head that unifies with its own leftmost body goal — is
the shape that makes an open query's termination depend on load order.
The gate checks it rather than asserts its absence. Every
`ckc check` runs `recursion-check` over the loaded composition and
rejects one, printing the rule-clause count that it scanned. The shipped corpus holds none.
The compiler writes each rule body binding-first: a goal that waits
on a value runs after the goal that binds it. A comparison or an
arithmetic built-in waits on its inputs. Any other schema goal except an
entity or an event goal waits on its referent, the second argument. A
negation waits on its variables that positive goals share. Each step
takes the leftmost goal that does not wait, else the leftmost goal. The
checks of the compiler read the order of the sentence, and only the
written body changes. For example, `cdc2022-opioid-s37-13` reads the
content of its negation box before the box, and
`cdc2022-opioid-rec05-imp12` reads its time quantity after the event
that names it. When no remaining goal is ready, the leftmost remaining
goal runs next: in `If it is false that there are at least 3 days then a
patient sleeps.`, no goal binds the context of the negation box, so the
box goal stays first. With the facts of a patient loaded, a broad
question still needs tabling (section "Loading a whole knowledge
base").

Authoring notes:

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

### Loading a whole knowledge base

A consumer loads `guidelines/schema.pl`, every compiled document of the
guidelines that it combines, and the facts of one patient. Before the
first document loads, declare every schema predicate as tabled:

```prolog
:- table guideline_schema_version/1, guideline_document/4, guideline_entity/4, guideline_cardinality/5, guideline_event/3, guideline_arg/4, guideline_pp/4, guideline_property/4, guideline_operator/3, guideline_interval/6, guideline_recurrence/4, guideline_frequency/5, guideline_order/4, guideline_recurrence_window/7, guideline_range/3, guideline_grade/5, guideline_preference/4, guideline_qualifier/4, guideline_unit/3, guideline_exclusion/1.
```

Without tabling, a standard engine does not finish a broad question
when the facts of a patient make the rules apply. For example, plain
left-to-right search over the CDC documents and a patient of three
sentences does not finish the list of every "should" statement. It does
answer the two dose questions of the gate, whether an opioid dosage
includes at least 50 or exactly 60 MME per day. With the declaration, SWI-Prolog 10.0.2 lists
every "should" statement that the composition derives in seconds, and
gives each committed question its committed answer. Some rules derive an
entity of their own, such as a patient in their conclusion, so a
derived statement can concern that entity and not the patient of the
loaded facts. Match each statement to the referents of the patient.

The documents use `\+` for negation as failure. Under tabling, `\+` is
correct when no goal depends on its own negation. For a recursion
through negation, SWI-Prolog gives an answer and no warning. The gate
checks the corpus with the patient worlds in `tests/consumer/` and the
patient world of `tests/queries/green/dose-arithmetic`: it evaluates
each composition again with `tnot/1` in place of `\+`, under the
well-founded semantics, and requires the same answers and no undefined
answer. A composition that you extend needs the same check. The gate
runs this recipe in SWI-Prolog 10.0.2. Other engines are not tested.

The engine of this repository searches without tabling. It answers the
committed questions and the two dose questions of the gate. A
broad question over the facts of a patient can stop at the bounds of
that engine with `indeterminate(limit)`.

### Vocabulary table and time annotations

The time annotations type the time limits that a sentence states. A
guideline opts in with the table `guidelines/<id>/vocabulary.tsv`. The
table maps source words onto closed identifiers, so no annotation names
a source word:

```
# format: kind<TAB>lemma<TAB>value
unit	day	day
relation	before	before
spacing	at	interval
window	during	window
frequency	per	period
approximation	approximate	about
range	to	minimum
strength	category-A-recommendation	strong
evidence	evidence-type-1-recommendation	high
preference	preferred	preferred
alternative	instead-of	alternative
qualifier	ideally	ideality
qualifier	feasible	applicability
scope	recommendation	cdc-2022-opioid
exclusion	apply	scope
```

The header line is fixed bytes. The table holds at least one row, and
every row has three fields and ends with a newline. A row whose first
byte is `#` is a comment and rejects. A `unit` row maps a noun lemma
onto a calendar unit (`second`, `minute`, `hour`, `day`, `week`,
`month`, `year`). A `relation` row maps a preposition onto a role
(`duration`, `within`, `after`, `before`). A `spacing` row pairs a
preposition with a frame noun for recurrence, as in
`at an interval of 3 months`. A `window` row pairs a preposition with
the frame noun of a time window, as in
`during a window of 1 week of the methadone-start`. A `frequency` row
names the preposition of a count per period, as in `per 1 day`; its
value is always `period`. An `approximation` row names an adjective
that marks a time quantity as approximate, as in `2 approximate hours`;
its value is always `about`. A `range` row names the preposition that
joins the upper end of a minimum stated as a range, as in
`for at least 8 hours to 12 hours`; its value is always `minimum`.
A `strength` row maps a grade noun onto the strength of a
recommendation (`strong`, `conditional`). An `evidence` row maps a
grade noun onto the certainty of its evidence (`high`, `moderate`,
`low`, `very_low`). A `preference` row names an adjective that marks
the preferred option on a copula, as in `An NSAID is preferred.`; its
value is always `preferred`. An `alternative` row names the preposition
that joins the option a preference replaces, as in
`… prescribe an immediate-release-opioid instead-of an ER-LA-opioid`;
its value is always `alternative`. A `qualifier` row maps an adverb,
or the adjective of a copula, onto the function class of a qualifier:
`usuality`, `ideality`, `focus`, `manner`, `force`, `possibility` or
`applicability`. The section "Strength annotations" below uses these
rows. A `scope` row names the noun that stands for the recommendations
of the guideline as a whole, and its value is the key of the guideline.
The key uses the document-identifier grammar, and in a guideline
folder it must equal the name of that folder. An `exclusion` row names
the verb that an exclusion denies of that noun; its value is always
`scope`. A table holds at most one row of each of these two kinds, and
it holds both rows or neither. The section "Scope exclusions" below
uses them.

A lemma is one or more bytes above 0x20, none of them 0x7F, that form
valid UTF-8. A noun occurs in at most one `unit`, `strength`,
`evidence` or `scope` row, and a noun in such a row is never also a
frame noun. A
pair of a preposition and a frame noun occurs once across the `spacing`
and `window` rows. A preposition occurs in at most one `relation`,
`frequency` or `range` row. A preposition in an `alternative` row
occurs in no `relation`, `spacing`, `window`, `frequency` or `range`
row. A lemma occurs in at most one `approximation`,
`preference` or `qualifier` row. The later row of a conflict reports
it. The checks run in row order: comment, field count, lemma, kind,
value, duplicates; a `scope` or `exclusion` row without its partner
reports last. `ckc check` rejects any
other table with `ckc: vocabulary: <path>: <why>`, where `<why>` names the
broken law and, for a row law, ends with `at row <n>`. The compiler, and
so `ckc compile`, rejects it with class `vocabulary_load` (below). A
guideline folder that holds a file named `temporal.tsv` is refused by every command that
reads the table: rename the file to `vocabulary.tsv` and replace its
header with the one line above.

The compiler takes the table from an explicit argument. `-` in a file
slot means no file:

```sh
swipl -q -f none -F none -s <ape-tree-dir>/prolog/ace_to_pl.pl -g main \
  -t 'halt(9)' -- <ape-tree-dir> <docid> <vocabulary.tsv | -> <ulex | -> [proof]
```

A table that does not load fails the compile with class
`vocabulary_load` and exit 2; the compiler never falls back to no
table. The word `proof` in the vocabulary or ulex slot rejects with
class `usage` as `vocabulary_arg(proof)` or `ulex_arg(proof)`. A
leading `v1`, `v2` or `v3` token and every other argument vector reject
with class `usage` as `argv(Argv)`. A document compiled with a table
records the table digest in `vocabulary(sha256(T))`; without one it
records `vocabulary(none)`. The declaration block, the identifier line
and every clause shape are the same either way.

| Predicate | Meaning |
| --- | --- |
| `guideline_interval(Context, Event, Role, Quantity, Unit, Anchor)` | `Event` stands in the time relation `Role` to `Anchor`, bounded by `Quantity` in `Unit`; `Anchor` is `none` when the sentence names no reference point |
| `guideline_recurrence(Context, Event, Quantity, Unit)` | successive occurrences of `Event` lie `Quantity` in `Unit` apart |
| `guideline_frequency(Context, Event, Counted, Period, Unit)` | `Event` involves the stated count of `Counted`'s items in each period of `Period` in `Unit` |
| `guideline_order(Context, Event, Relation, Anchor)` | each occurrence of `Event` lies `before` or `after` `Anchor`; no time bound is stated |
| `guideline_recurrence_window(Context, Event, Gap, GapUnit, Anchor, Length, LengthUnit)` | consecutive occurrences of `Event` inside the window that starts at `Anchor` and lasts `Length` in `LengthUnit` lie `Gap` in `GapUnit` apart |
| `guideline_range(Context, Low, High)` | the minimum that the interval quantity `Low` states lies from the count of `Low` to the count of `High`; `High` bounds nothing about the event |

An annotation never replaces a clause. It follows the `guideline_pp`
clause that it types, in the same context and with the same rule body.
A document compiled without a table never carries an annotation.

The compiler reads these patterns from the parsed sentence. A time
quantity is a referent whose noun is a `unit` lemma. Its cardinality
clause carries the comparison and the count. The patterns read
referents by their noun anywhere in the document. A referent whose noun
is no unit and no frame noun is a plain referent.

- Interval: the event carries a `relation` preposition whose object is
  a time quantity. The anchor is the single `of` object of that
  quantity in the same context, as in
  `within at most 4 weeks of the therapy-start`.
- Recurrence: the event carries a `spacing` preposition with its frame
  noun, and the frame has a single `of` object that is a time quantity,
  as in `at an interval of at most 3 months`.
- Order: the event carries a `relation` preposition with role `before`
  or `after`, and its object is a plain referent. The object may come
  from the antecedent or from an earlier sentence.
- Count per period: the event carries a `frequency` preposition whose
  object is a time quantity with an exact count and no `of` object. The
  counted item is the second participant of the event. It is a count
  noun with a comparison and a count of at least 1, as in
  `receives 2 buprenorphine-administrations per 1 day`.
- Scoped recurrence: the event carries a `window` preposition. Its
  frame is one count noun with one `of` object, a time quantity, and
  that quantity has one `of` object, a plain referent. The event must
  also carry a recurrence. Each recurrence on that event becomes a
  `guideline_recurrence_window` clause, and no plain
  `guideline_recurrence` clause is emitted for it.
- Approximate time limit: an `approximation` adjective on a typed time
  quantity, or on the spacing or window frame of the quantity, marks it
  approximate. Its `guideline_cardinality` clause then has the
  comparison `about` in place of `eq`, as in
  `at an approximate interval of 2 hours`. The comparison `about` names
  no tolerance; `about 2` matches only `about 2`. An approximate
  quantity must state an exact count.
- Ranged minimum: a `range` preposition whose object is an exact time
  quantity pairs with the one `at least` interval on the same event,
  as in `for at least 8 hours to 12 hours`. The pair emits
  `guideline_range(Context, Low, High)` beside the interval clause. The
  pair needs the same unit and a smaller lower count. The interval keeps
  its own `at least` comparison.

An interval or recurrence quantity must be a count noun with a
comparison and a count of at least 1. Every other shape of these
patterns rejects with class `unsupported` as `temporal_shape(Why, S)`:

| `Why` | Shape |
| --- | --- |
| `no_bound` | the quantity has no comparison, or it is not a count noun |
| `zero_bound` | the count is below 1 |
| `duration_anchor` | a `duration` role names an anchor |
| `anchor_count` | an interval quantity has two or more `of` objects, a frame does not have exactly one, or a recurrence quantity has one |
| `shared_quantity` | two patterns claim one quantity or one frame |
| `frame_shape` | the frame is not exactly one count noun, or its `of` object is not a time quantity |
| `anchor_shape` | the anchor is itself a time quantity or a frame |
| `frequency_shape` | a count per period breaks its pattern |
| `window_shape` | a time window breaks its pattern |
| `approximate_bound` | an approximate quantity states no exact count |
| `range_shape` | a ranged minimum breaks its pattern |

An `of` link that no pattern consumes rejects. The compiler checks the
patterns left to right, then the shared claims, then each
negation-as-failure scope. The first violation rejects.

Annotations are data, never schedules. `duration` means that `Event`
lasts that long. `after` means that `Event` occurs that long after
`Anchor`, and `before` that long before it. `within` means that `Event`
and `Anchor` lie that far apart, in either order. `geq` and `leq`
include the bound; `greater` and `less` exclude it. A unit is a calendar
label: the compiler calculates no date, and only a condition compares
quantities in different units (section "Comparison conditions"). Two
annotations on one event state both limits, so a range is a pair on one
anchor. A condition compares by value: a fact for `geq 30` meets a
condition for `geq 20`. A count per period does not set
calendar buckets, a first occurrence or a minimum gap. A window does not
set the first occurrence and says nothing about occurrences outside it.

An abbreviation such as `d` or `h` can type as a unit. The corpus
lexicon must declare it as a count noun, and a `unit` row must map it.
ACE also parses `3 d` as a measured number, but the compiler
rejects that form as `unresolved_argument`.

Documents compiled with and without a table load into one composition,
and a question runs against any composition. Certification takes the
raw table bytes, so `ckc certify` checks the record digest and every
annotation against the table.

### Strength annotations

The strength annotations type the grade of a recommendation, the
preference of one option over another and the qualifiers of a
statement, such as `ideally`, `typically` or `when feasible`. They need
the `strength`, `evidence`, `preference`, `alternative` and `qualifier`
rows of the vocabulary table. Only documents carry them; question mode
never adds them.

| Predicate | Meaning |
| --- | --- |
| `guideline_grade(Context, Rec, Dimension, Value, Label)` | the recommendation `Rec` has, in `Dimension` (`strength` or `evidence`), the common value `Value`; `Label` is the referent whose noun is the source's own grade label |
| `guideline_preference(Context, Event, Preferred, Alternative)` | in `Event`, the source prefers the option `Preferred` to `Alternative`; `Alternative` is `none` when the source names no other option |
| `guideline_qualifier(Context, Event, Class, Word)` | the source qualifies `Event` with the word `Word`, whose function class is `Class` |

The compiler reads these patterns from the parsed sentence:

- Grade: a fact sentence states `R is a L` in the context `actual`, and
  the noun of `L` is in a `strength` or `evidence` row, as in
  `A recommendation is a category-A-recommendation and is an evidence-type-4-recommendation.`
  The grade clause follows the `be` event's second participant. A
  document has at most one grade for each dimension. The grade applies
  to every statement of its document: `Rec` carries the document
  identifier. A grade noun in a rule, an operator box, a
  negation-as-failure group or a question adds no grade.
- Preference: an event carries one `alternative` preposition, or a
  copula `X is A` has an `A` from a `preference` row. A preference
  copula prefers `X`; any other event prefers its second participant.
  The alternative is the object of the `alternative` preposition, or
  `none`. The preference clause follows the preposition's `guideline_pp`
  clause, or the copula's second participant when no alternative
  exists. It takes the context of its event, so a preference in a rule
  antecedent becomes a body goal of every clause of the rule, and a
  preference under an operator stands in that operator's context.
- Qualifier: an adverb on an event has a `qualifier` row, as in
  `Every clinician should ideally review a record.` The qualifier clause
  takes the place of the adverb in the order of the sentence. A copula
  `X is A` whose adjective `A` has a `qualifier` row also gets a
  qualifier clause, after the copula's second participant, as in
  `If a treatment is feasible then …`. Each such adjective of the copula
  gets its own clause, and the adjective clauses stay. An adjective on a
  noun, as in `X is a feasible option`, qualifies a thing and adds no
  qualifier. A qualifier takes the context of its event, as a
  preference does. An adverb without a `qualifier` row rejects as
  `condition_shape`.

The class of a qualifier names the function of the word:

| Class | Words, for example | Function |
| --- | --- | --- |
| `usuality` | typically, usually, often, regularly | how usual or how frequent the event is |
| `ideality` | ideally, optimally, preferably | the ideal or best form of the event |
| `focus` | particularly, especially, importantly | the source emphasizes the event in its scope |
| `manner` | carefully, closely, cautiously | how the event is carried out |
| `force` | strongly | the strength of the wording of the advice |
| `possibility` | possibly, potentially, likely | the event is possible or likely |
| `applicability` | appropriate, feasible, possible, needed, indicated | the event or thing fits the case |

The context of a clause, not the class of its qualifier, tells whether
the clause is asserted. An adverb of class `usuality`, `ideality`,
`possibility` or `applicability` weakens what its clause says. Thus such
an adverb cannot qualify an asserted clause: a fact or a rule head that
stands under no modal operator (`should`, `must`, `can`, `may`). A
classical negation alone does not make a clause unasserted. Write the
claim under a modal operator, as in
`… then the patient can possibly need a bowel-therapy.` An adverb of
class `focus`, `manner` or `force` and every copula adjective can
qualify an asserted clause. A qualifier in a condition is a goal that a
matching fact must meet.

A qualifier belongs to the description of its event. A negation or an
operator covers the event together with its qualifiers:
`it is recommended that the clinician does not abruptly discontinue an
opioid` recommends against an abrupt discontinuation, not against every
discontinuation. A program that matches an event must also
match the qualifiers of that event.

Every other shape of these patterns rejects with class `unsupported`:

| Detail | Shape |
| --- | --- |
| `grade_shape(duplicate_dimension, S)` | a second grade of one dimension in one document |
| `preference_shape(alternative_count, S)` | an event with two or more `alternative` prepositions |
| `preference_shape(no_preferred, S)` | an event with an `alternative` preposition and no second participant |
| `preference_shape(endpoint, S)` | a preferred option or an alternative that is not a plain referent (a property, a time quantity or a frame) |
| `qualifier_shape(degree, S)` | a qualifier adverb in a comparative or superlative form, as in `more carefully` |
| `qualifier_shape(asserted, S)` | an adverb of class `usuality`, `ideality`, `possibility` or `applicability` that qualifies an asserted clause |

Annotations are data, never inference. A grade does not rank
recommendations, and a preference does not forbid the alternative or
oblige the preferred option. A qualifier sets no frequency,
probability, priority or condition. Erasing every strength annotation
from a document gives the clauses that the same sentences give without
the strength rows and without the qualifier adverbs; an annotation in a
rule antecedent adds one body goal, which a matching fact must also
carry. A question asks for a grade by its source label and for a
preference by its words; a question cannot contain an adverb. A program
queries `guideline_grade/5`, `guideline_preference/4` and
`guideline_qualifier/4` directly.

### Comparison conditions

A condition of a rule or a question compares a stated quantity with its
bound. The condition `at least 50 MME-units-per-day` is met by a fact
that states `exactly 60`, `60`, `at least 60` or `more than 50`. It is not
met by `at most 70`, `about 60` or a quantity without a number. A condition
is met when every value that the stated quantity allows is a value that
the condition allows. A plain number allows that count or more, `exactly`
allows the count alone, `geq` and `leq` include their bound, and `greater`
and `less` exclude it. Counts are integers or exact rational numbers
(section "Arithmetic built-ins" below), and the comparison treats the
values between them as possible. The comparison is therefore safe and can
miss an entailment that holds only for whole counts: `more than 49` does
not meet `at least 50`.

The compiler converts each body object whose comparison is `exactly`,
`geq`, `greater`, `leq` or `less`. The object keeps its cardinality
literal with two new variables, and the built-in compares them with the
bound:

```
guideline_entity(actual,B,'MME-unit-per-day',countable),
guideline_cardinality(actual,B,na,C,D), guideline_satisfies(C,D,geq,50)
```

A plain-number condition (`eq`), an `about` condition, a quantity without
a number, every rule head and every root fact keep their literals as data.

A time quantity carries its unit in the annotation
`guideline_unit(Context, Quantity, Unit)` wherever it appears, when the
guideline has a vocabulary table. A converted time quantity compares across
units: seconds, minutes, hours, days and weeks convert exactly, and months
convert to years at twelve to one. A day and a month never compare. The
condition leaves out the entity literal of the quantity, because the noun
of a time quantity names its unit, and the unit slots of the annotations
that name the quantity share one variable:

```
guideline_cardinality(actual,F,na,G,H), guideline_unit(actual,F,I),
guideline_satisfies(G,H,I,greater,3,day)
```

A fact `exactly 1 week` meets this condition. A fact `exactly 1 month`
does not.

The built-ins are no document predicates. No clause heads them, and no
declaration names them. The verified engine evaluates them, and
`guidelines/schema.pl` states them in ISO Prolog for other engines. An
engine gives exact results when its integers can hold the absolute value
of each numerator and each numerator multiplied by the other count's
denominator and by 604800, the number of seconds in one week. SWI-Prolog
10.0.2 has unbounded integers. A consumer loads that file with the
documents. `ckc compile` writes the
file, `ckc check` rejects a missing or stale copy, and the release
archive ships it. A built-in call with an argument of another shape
fails and raises no error.

A program that matches patient data states each value as a fact. A
measured value takes `exactly`, as a quantity without a comparator does in
FHIR. A reported bound takes `geq`, `greater`, `leq` or `less`. Each time
quantity also takes its `guideline_unit` literal. An ACE root fact
states an exact value as `exactly n` with n of at least 1; a root
`at most n` or `less than n` rejects, because it admits zero. A ranged minimum (`guideline_range/3`) states an uncertain
minimum, not an interval of values: its upper end and every count-per-
period period keep exact matching. An `about` quantity carries no
tolerance and meets no comparison.

### Arithmetic built-ins

The arithmetic built-ins compute a quantity from other quantities. The
built-in `guideline_product(S1, N1, S2, N2, S, N)` multiplies, and
`guideline_quotient(S1, N1, S2, N2, S, N)` divides. The first four
arguments are inputs: two pairs of a comparison and a count. The last two
arguments are outputs: the comparison and the count of the result.

A count is an integer or an exact rational number. A rational number that
is not an integer has the form `rational(P, Q)`: Q is at least 2, and P and
Q have no common divisor other than 1. For example, 70.5 is
`rational(141, 2)` and 0.15 is `rational(3, 20)`. Each number has exactly
one such form, so two equal numbers always unify. The schema has no
floating-point numbers.

A pair of a comparison and a count states a set of values: `exactly x` is
the value x; `eq x` and `geq x` are the values from x up; `greater x` is
the values above x; `leq x` is the values up to x; `less x` is the values
below x. A built-in multiplies the two sets. When the result is one value
or a set that one comparison can state, the built-in binds the outputs to
it. For example, `geq 60` times `exactly rational(3, 2)` gives `geq 90`. The
result never uses `eq`.

A built-in derives nothing in these cases:

- the product of the two sets contains every number, for example
  `geq -1` times `geq 3`;
- the divisor is zero or is not an exact value;
- an input count is not a number, or its comparison is `about` or `na`.

A built-in that derives nothing does not state that the arithmetic is false.
The verified kernel proves that a derived result never excludes a true
product or quotient of input values (the theorem `arith_result_sound`). `guidelines/schema.pl` states the same
built-ins in ISO Prolog; a consumer loads it with the documents. The
results are exact when the integers of the engine can hold every
intermediate integer, absolute values and products included. SWI-Prolog 10.0.2 has unbounded integers.

An emitted rule can feed its own condition. Plain left-to-right Prolog
then searches without end, and the engine of this repository stops at its
bound and reports `indeterminate(limit)`. Three rule shapes cause this: a
result that uses a verb or a noun of the condition; a first condition that
names a noun that other rules derive, such as `opioid-dosage`; and a
formula in a rule with a time condition, because a compiled time quantity
read its count before its event bound it. Binding-first bodies now read
that count after the event; the corpus keeps the split form. The formula
rules of this repository avoid all three. Each conversion rule starts with the amount of
its opioid, each result word differs from the condition words, and a rule
with a time condition names the result of a separate formula rule. Other
rules of the corpus still feed their conditions once patient facts are
loaded; a broad question therefore needs tabling (section "Loading a whole
knowledge base").

#### Formulas in ACE

A rule can compute a quantity with a formula. The formula is the last part
of the consequent:

```
If some oxycodone-milligrams-per-day X are contained by an opioid-dosage then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5.
```

The compiler reads two extensions of ACE in a formula. These extensions are
specific to cnl-ckc, and they apply only to formulas:

- A quantity noun in a formula stands for the count of the quantity. In the
  example, X is the number of milligrams per day.
- The word `some` directly before the noun of a quantity in a formula sets
  no bound on its count. APE alone reads `some` before a plural noun as at
  least 2. When a word stands between `some` and the noun, as in
  `some red cats X`, the quantity keeps the reading of APE.

A formula must obey these rules:

- The formula stands at the top level of the last consequent of a rule,
  outside each negation, modal box and alternative. One rule can hold more
  than one formula, each for a different quantity.
- The left side names a quantity of the consequent that `some` introduces.
  This quantity is the target. A target is not an operand of a formula, and
  no other formula names it.
- The relation is `=`. The expression uses `*` and `/`, and it holds at
  least one operation. Parentheses group the operations.
- An operand is a quantity of the condition, at the top level of the rule,
  that `some` or a comparison introduces, for example `exactly 40` or
  `at least 40`. Its noun is countable. A time quantity and a measurement,
  for example `at least 2 kg of apples`, are not operands. A condition with
  a comparison keeps its comparison. A plain number, `a`, `about` and a
  quantity without a number are not operands.
- A number in the expression is an integer or a decimal number. The compiler
  takes the value from the ACE text, so the value is exact. Each number of
  the sentence must keep its spelling when APE reads it: `1.50`, `02`, a
  decimal number with more digits than APE keeps and a decimal number that
  APE writes with an exponent reject. A sign before a number of the sentence
  rejects, and so does a character after a number of the formula other than
  a space, `)`, `*`, `/` or the final period.

The compiler emits one arithmetic built-in for each operation of the
expression, inner operations first. The built-ins stand after all other
conditions of the rule. The count of the target takes the result of the
last built-in:

```
guideline_cardinality(actual,'$guideline_id'(product,…,ref(4),…),na,D,E) :-
    guideline_entity(actual,A,'oxycodone-milligram-per-day',countable),
    guideline_cardinality(actual,A,na,F,G), …,
    guideline_product(F,G,exactly,rational(3,2),D,E).
```

An operand with `some` directly before its noun keeps no
`guideline_satisfies` condition. The proof obligation of the rule gives each
operand one exact value that is not zero: 1 for such an operand, n for
`exactly n` and `at least n`, n + 1 for `more than n`, n for `at most n`,
and n − 1 for `less than n`. When this value is 0, the
obligation takes the next integer in the direction that the condition
allows, for example 1 for `at least 0` and −1 for `less than 1`. Only
`exactly 0` keeps 0. The obligation computes the built-ins with these
values, and the head of the rule takes the result. When a built-in derives
nothing for these values, for example for a divisor of 0, the sentence
rejects. The witness value is fixed: when it breaks another condition of
the rule, such as `does not provably contain at least 1 cat`, the
obligation fails although another value would satisfy the rule, and the
compiler rejects the document with `underivable_obligation`.

A formula that breaks a rule rejects with `formula_shape(Why, S)`, class
`unsupported`. The reason names the first check that fails, in this order:
`position`, `relation`, `operator`, `target`, `operand`, `operand_bound`,
`literal`, `witness`. The check `operand` covers each operand of the formula
before the check `operand_bound` starts. A question cannot hold a formula.
The certifier derives each formula again from the DRS and the ACE text and
reports a reject as `unsupported(sentence(S, formula_shape(Why)))`.

### Scope exclusions

A guideline often states where its recommendations do not apply, for
example "The recommendations do not apply to pain related to sickle
cell disease or cancer". A scope exclusion makes such a statement
active: when its condition holds, no recommendation of the guideline
derives. The `scope` and `exclusion` rows of the vocabulary table
(section "Vocabulary table and time annotations") opt a guideline in.
A document that is compiled with both rows is a scoped document.

An exclusion is a rule whose consequent says that a recommendation does
not apply:

```
If a patient has a cancer-related-pain then it is false that a recommendation applies to the patient.
If a patient receives a hospital-care and the patient does not provably receive a pain-management-prescription at a facility-discharge then it is false that a recommendation applies to the patient.
```

The consequent must hold exactly one negation, and the negation must
hold exactly the following conditions: the scope noun as a plain
indefinite (`a recommendation`), the exclusion verb without an object,
and prepositional phrases whose objects come from the condition. The
condition of the rule can be any rule antecedent. A `does not provably`
condition states an exception to the exclusion, as the discharge
prescription in the second example does.

| Predicate | Meaning |
| --- | --- |
| `guideline_exclusion(Key)` | an exclusion of the guideline `Key` holds: its recommendations do not apply in this composition |

An exclusion compiles to its usual clauses and, after them, one record
clause with the same body:
`guideline_exclusion('cdc-2022-opioid') :- <the condition>.` In a
scoped document, every clause whose head is a `should`, `must` or `may`
operator edge ends with the scope guard
`\+ guideline_exclusion(Key)`. No other clause changes: the payload
clauses of a box, the `-` and `can` edges, facts and questions stay as
they are. The guard and the record hold no variables, so a standard
Prolog engine loads the documents without warnings.

When an exclusion holds, no `should`, `must` or `may` edge of that
guideline derives. A program that follows the operator edges from
`actual`, as section "Compiled Prolog schema" describes, then finds no
recommendation of that guideline. The payload clauses of a blocked box
still derive, but no edge from `actual` reaches them. A `can` box whose
only content is a blocked edge stays as an empty possibility. The
guard has no patient argument: one composition must hold the facts of
one patient, because an exclusion that holds for any loaded patient
blocks the guideline for all of them. The absence of an exclusion is
not proof that a patient belongs to the population of the guideline.
Statements about the population or the audience of a guideline stay
data. When the bounds of the engine of this repository cut the inner
search of a negation and that search then ends without a proof, the
result is `indeterminate(limit)`, so a deep exclusion never passes
silently (section "Query answers").

These shapes reject in a scoped document with class `unsupported` as
`scope_shape(Why, S)`. The checks run after the other checks of the
sentence, in this order:

| `Why` | Shape |
| --- | --- |
| `deontic_condition` | a `should`, `must` or `may` box in the condition of a rule |
| `negated_deontic` | a `should`, `must` or `may` box inside a negation in the consequent of a rule |
| `root_deontic` | a `should`, `must` or `may` box in a fact |
| `root` | a fact that denies that a recommendation applies (an unconditional exclusion) |
| `subject` | an exclusion whose subject comes from the condition, as in `Every recommendation does not apply …`, or is another noun than the scope noun |
| `consequent` | any other consequent that denies the exclusion verb of the scope noun: more conditions, a count or a comparison on the subject (`2 recommendations`, `at most 2 recommendations`), an object of the verb, or a prepositional object inside the negation |

The `subject` and `consequent` checks read the first negation of the consequent that denies the exclusion verb. A second such negation counts as one more condition.

`ckc v1 recursion-check` adds one composition check. When the
composition holds an exclusion record, no clause body may hold a
`guideline_operator/3` goal whose operator is `should`, `must`, `may`
or a variable; the check rejects with class `proof` as
`scope_condition(Site)`. The guards then add no loop: no clause can
depend on a guarded edge, so an exclusion depends only on clauses
without a guard. The check covers the composition that it reads; a
program that loads another set of documents must run it again on that
set. It does not prove that the other negations of the documents
terminate.

### Question projection

The compiler's `question` mode compiles one ACE question against the
same vocabulary (`ckc queries` stages the APE tree and runs it):

```sh
swipl -q -f none -F none -s <ape-tree-dir>/prolog/ace_to_pl.pl -g main \
  -t 'halt(9)' -- question <ape-tree-dir> <qid> <vocabulary.tsv | -> <ulex | ->   # ACE on stdin
```

The input must be exactly one sentence line that parses to one root
question box. `<qid>` follows the docid grammar. Success emits four
lines. Line 1 is a generated-file comment. Line 2 is the ground
`'$guideline_query'(v1, Qid, ace_sha256(H), ulex(none | sha256(U)), vocabulary(none | sha256(T)))`
record. Line 3 is a `% Q1:` comment with the verbatim sentence. Line 4
is one `'$guideline_query_projection'(goal(Conj), answers(Manifest))`
term. `Conj` is an explicit `','/2` conjunction of goals rendered
through the rule-antecedent pipeline. The root context is `actual`,
and referents stay variables. A modal box contributes its
`guideline_operator/3` edge before its payload goals, under a shared
existential context variable. `Manifest` lists one `answer(Var, Desc)`
row per wh-placeholder in source order. `Desc` is `noun(Noun, Class)`
when exactly one same-box `object/6` types the placeholder; otherwise
it is `wh(who)` or `wh(what)`. Goal and manifest share variables
inside the one projection term.

A question of a guideline with `vocabulary.tsv` compiles with the
table, and its record carries the table digest. With a table, a
question admits three more shapes. It admits the condition list of an
upper-bounding determiner and flattens that list into its box; a
placeholder inside such a list rejects as `condition_shape`. It admits
`relation/3` conditions. It runs the pattern pass of the section
"Vocabulary table and time annotations" over its goals, with the same
rejects. An `of` link that no pattern consumes rejects. Without a
table, a question admits none of these shapes.

Supported forms: `wh(who | which | what)` and yes-no questions over
conjunctive content, with modal boxes at any nesting of themselves.
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
indicators, no declarations, and no proof obligations. It never
enters a document, aggregate-check, or recursion-check composition.
`ckc check` re-derives every committed query separately and compares
the bytes.

### Query answers

`ckc v1 answer` solves one compiled query against a loaded
composition in the verified Rust engine:

```sh
ckc v1 answer <manifest> <query-pl>
```

`<manifest>` uses the aggregate manifest grammar. The payload column
must name readable files, and the mode never parses payload terms. A
0-byte manifest is the empty composition: the mode still declares the
indicators and solves against no clauses. `<query-pl>` must read
as exactly two terms in order: the ground `'$guideline_query'/5`
record, then the `'$guideline_query_projection'/2` term. A query names
any of the 18 semantic predicates, the comparison built-ins and the
arithmetic built-ins, and it runs against any composition. The mode
reads the query file as data and never consults it. The file must hold the exact canonical bytes that `ckc queries` writes: an added comment or a layout change rejects as `query_file(noncanonical)`.

Success emits a two-line artifact on stdout: a generated-file
comment, then one ground term
`'$guideline_answers'(v1, Qid, query_sha256(H), result(R))`. `H` is
the SHA-256 of the raw query-file bytes. The solver runs under fixed
bounds: depth 100 for each derivation, one 100000-inference budget
that is cumulative across backtracking for the whole query, and
1000000 inferences for the whole run. For a wh query (`answers` rows
present), `R` is `solutions(Sols)`, or `indeterminate(limit)` when a
bound trips. Each distinct solution contributes one `sol(Values)`
row; rows follow the standard order of terms, and values follow the
answer-manifest order. For a yes-no query
(`answers([])`), the first proof is conclusive. `R` is `yes` when a proof is found, `no(finite_failure)` when the search ends without a proof and without a tripped bound, and `indeterminate(limit)` when it ends without a proof after a bound tripped. A negated goal succeeds only when its inner search ends without a proof and no bound cut that search. When the inner search finds a proof, the negation fails, even if a bound cut another branch of that search. When the inner search ends without a proof after a bound cut it, the negation stays undecided: the whole search ends, and `R` is `indeterminate(limit)`, for a yes-no query and for a wh query. A nonground solution rejects
with class `proof` as `nonground_solution(Qid)`. A malformed query
file rejects with class `check_load` as `query_file(<why>)`. Manifest
and composition load failures keep the aggregate classes and details.

The engine searches depth-first and tries clauses in file order. It
runs the leftmost goal first, with one exception. An operator goal
`guideline_operator(Outer, Inner, Op)` waits when `Inner` is still a
variable and a later goal names that variable as its first argument.
Such a later goal is part of the box content, so its result binds the
box context. A built-in never names a box context. A built-in with an
argument that is not bound yet ends the run as a negation does; for an
arithmetic built-in, the outputs count too. A waiting operator goal
therefore never lets a built-in run before its values.
Only the goals before the next negation-as-failure goal
count, and a conjunction counts with its parts. The engine runs the
leftmost goal that does not wait. The clause body that resolves this
goal takes its place, so the waiting operator goal runs directly after
the head unification that binds its context. A rule whose consequent
box is a different box therefore fails before its body runs. The same
rule applies to the goals of a question.

A goal never moves past a negation. A negation runs only after every
goal to its left has run, together with all goals that those goals
started. In every certified document, each variable of a negation
that also occurs outside the negation occurs in an earlier positive
goal of the same body. For such documents, each negation runs with
the same bindings as in the left-to-right order. When the search ends
in both orders, it gives the same answers. A negation in a document
that the compiler did not certify can run with more bindings than in
the left-to-right order, and it can then give a different answer. The
order of the search decides which branch a bound cuts, whether a
search ends, and which proof the engine finds first.

Each step of the search costs one inference: a call or a retry of a
goal (the clause scan and the head unification included), the split of
a conjunction, the entry into a negation, the cut that ends a
negation, a comparison built-in, an arithmetic built-in (also when it
derives nothing or its outputs do not unify), a pruned goal, and a recorded solution. A built-in
leaves no choice point. The step that
reports a yes-no proof costs one inference too.

The compiled Prolog keeps the body order that the compiler writes, so
a standard Prolog engine searches these documents left to right.
`recursion-check` checks the leftmost body goal, which is the first
goal of that order. The engine in this repository can run a later goal
first.

Committed query artifacts live under `guidelines/<id>/queries/`:
`<qid>.ace` sources at the root, compiled queries under `pl/`, answer
artifacts under `answers/`, and proof traces under `traces/`.
`ckc queries <id>` derives every artifact in memory and writes only
after every derivation succeeds. `check`
re-derives each file per query twice and compares the bytes. A
committed answer must be `yes` or nonempty `solutions(...)`;
committed queries stay demonstrations. The `queries` and `check`
commands bound every compile, answer, and trace subprocess at 30
seconds of wall-clock time. A run that exceeds the bound fails the
gate. The direct `ckc v1` invocation above has no process bound.

### Proof traces

`ckc v1 trace` re-proves the positive claims of one answer artifact
against a loaded composition in the verified Rust engine:

```sh
ckc v1 trace <manifest> <query-pl> <answers-pl>
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
clause_sha256(Hex), Children)`. It names the resolved clause by its document and by the `% S<n>:` block that holds the clause's line. A clause that cites an earlier sentence's referent still names its own sentence. `Hex` is the SHA-256 of
that clause's rendered document line with its newline. A re-checked
negation-as-failure goal freezes as a `naf(Goal)` leaf among a
clause node's children. A comparison built-in that held freezes as a
`builtin(Goal)` leaf, the goal as called, with no clause digest. An
arithmetic built-in that computed a result freezes as a `builtin(Goal)`
leaf too, with its outputs bound to the result, and the trace check
computes the result again. Root nodes are clause nodes or
`builtin(Goal)` leaves. The
prover is a directed first-proof interpreter over the loaded
clauses. It selects goals as the answer mode does: an operator goal
waits for its box content. The goals of a row are separate roots, and
each root keeps its place in the proof. Each row's search runs under
depth 1000 and 100000 inferences. Each materialized clause node adds
one inference to the whole run but not to its row. The whole run is bounded at 1000000 inferences. The
interpreter's depth measure is its own; it is not comparable with
the answer mode's engine measure. A negation site re-checks under
the same bounds. A negation call is safe when no variable of the
negated goal occurs in a later goal of the same search level; a
variable that occurs only inside the negation stays free. The prover
cuts an unsafe call like a tripped bound, so the row ends
`unproved(limit)` unless another branch proves it. A bound that trips inside a negation makes the row `unproved(limit)` unless the inner search still finds a proof; an inner proof absorbs the cut, the negation fails, and the row can end `unproved(finite_failure)`. When the
whole-run bound trips, the result becomes `indeterminate(limit)` in
place of the mirror. The direct `ckc v1` invocation has no process
bound.

`check` re-derives each committed trace twice, compares the bytes,
and resolves every `clause_sha256` to exactly one clause line of the
committed document under `pl/`. A committed trace must prove its
answer: `yes(proved(...))`, or solution rows that are all proved.
`check` prints one `ckc: traces <id> <n> traces; nodes=<k>` meter
per guideline beside the queries meter.

## Operating

Guideline work runs as rounds in a Claude Code session that is opened
at the repository root. Paste this prompt as the first message of the
session:

```
Process American clinical guidelines through the pipeline as described in docs/REFERENCE.md "Operating", with teammates per the round roles in .claude/rules/corpus.md: work the in-progress source document to full coverage before fetching the next. When the user asks to stop or wind down, start nothing new, state where work stands, and stop — the request alone ends the run at any point, even mid-round with the worklist unfinished; a pasted pause prompt runs its own wind-down instead. Met when: every fetched guideline is complete, every remaining .agent/queue.md entry is a recorded blocker, and the compendium exhaustion clause in .agent/compendium.md "Protocol" holds: every guideline row of .agent/compendium.tsv done, blocked, or excluded, and every organization row terminal.
```

The session works until the `Met when` condition holds or the user
asks it to stop. The repository is the only persistence, so halting at
any moment is safe. Every round starts by deriving state from the
repository: `.agent/queue.md`, `git status`, `ckc check`, and the
in-progress guideline README's coverage statement. The round then
finishes or discards incomplete work before it takes on anything new. A
user request to stop ends the run at once, mid-round included, even
when the exhaustion clause is not met. The wind-down is to start
nothing new, state where work stands, and stop. The check validates
the fork notices and the adjudication fixtures first. It then
validates the compendium — the `.agent/compendium.md` organizations
table plus `.agent/compendium.tsv`: row format and vocabulary,
canonical ordering, the single-active-row promotion invariant. It prints a terminal meter —
remaining non-terminal organizations and unfinished guideline rows —
that measures the exhaustion clause directly. Corpus validation
(projection-ledger totality, coverage-ledger closure with its
region-status meter, lexicon liveness and minimality, the schema's
product vocabulary) runs for every guideline on every check. The same pass
re-derives each guideline's review manifest and validates its
adjudication ledger. It also re-derives every committed query,
answer, and proof trace, and it joins each clause step of a trace to
its committed clause line. The session lead drafts all ACE,
alone writes the repository, and commits. Source reading and
extraction go to `researcher` teammates. Completeness sweeps and
adversarial review go to `reviewer` teammates. The round roles are in
`.claude/rules/corpus.md`.

Reviewer verdicts live in each guideline's `audit/adjudication.tsv`,
pinned to content digests in `audit/review-manifest.tsv`. When compiled
content changes, regenerate the manifest with
`ckc review-manifest <id>`. Commit the round
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
- Every extracted statement is either authored as knowledge-only ACE,
  compiled, and aligned to its passage, or recorded in the guideline
  README as uncovered with a reason. Compiled means that the
  obligations discharge alone and in aggregate.
- `ckc check` is green, with the guideline's
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
   statement use that same unit. Consult the guideline's
   `audit/lexicon-rejects.tsv` before you propose a lexicon entry.
   Record each refused candidate there as one
   `candidate<TAB>category<TAB>reason` row.
3. **Compile** — run `ckc compile <id>`. When the
   compiler rejects a document, adjust the ACE or the lexicon first.
   Only a genuinely new construct extends the `ace_to_pl.pl`
   translation, minimally. Such an extension keeps totality: every
   sentence compiles to a determinate clause bundle whose obligations
   discharge, and unrecognized shapes reject. It also adds `tests/red/`
   probes for the new rejection boundary.
4. **Align** — after the document compiles, record where each ACE term
   came from:

   ```
   ckc align <id> <docid> < rows.tsv
   ```

   Each input row is `group<TAB>side<TAB>occurrence<TAB>span`. The
   `group` is an integer that names one linked unit across both texts.
   The `side` is `src` for the passage or `ace` for the ACE text. The
   `occurrence` counts the 1-based position of `span` in that side's
   text, as a plain substring. The command resolves each row to a
   character offset and writes `align/<docid>.tsv`. Link each ACE
   domain term to the passage text that it came from. Prefer the
   smallest span a reviewer would point to: "category-B" links to
   "category: B", not to the whole sentence. Link a part of a
   hyphenated term when only that part has direct source text. A
   word-form difference is fine: "assess" can link to "assessment".
   Leave text without a direct counterpart plain — the alignment must
   be helpful, not exhaustive. When a later edit changes the ACE text
   or the passage, author the alignment again; the render stops on
   stale offsets.
5. **Close** — `ckc check` must be green. Make a scoped commit. When
   the commit touches `guidelines/`, the vendored compiler or lexicon,
   `NOTICE`, `LICENSE`, or the `docs/REFERENCE.md` schema section,
   `release-manifest.tsv` goes stale: run `just amend-manifest` after
   the commit. It regenerates the manifest and amends it into the same
   commit. Update `.agent/queue.md` and the guideline README's
   coverage statement.

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
A normative statement that the controlled language cannot express
gets the coverage status `uncovered(inexpressible: <reason>)`, like the
other uncovered classes, and the round moves on. The coverage meter
counts these regions as `inexpressible=<n>`, so they record the
expressiveness gap of the language for later work.

## Export

`ckc dist build` packages the committed corpus as one reproducible
archive:

```
ckc release-manifest   # refresh release-manifest.tsv, then commit it
ckc dist build [<dest>]   # default destination: dist/
```

The archive is a BagIt 1.0 bag named `cnl-ckc-kb-g<head12>.tar.gz`,
where `<head12>` is the source commit prefix: the last commit that
changed `guidelines/`, the vendored compiler, or the lexicon. The root
directory of the bag has the same name. The manifest records content
digests only, so a commit can include its own manifest. A `.sha256` sidecar
accompanies it. To verify a bag, extract it and run
`sha256sum -c manifest-sha256.txt tagmanifest-sha256.txt` from the bag
root. Each line must report OK.

The payload is the committed `guidelines/` corpus, its common member
`guidelines/schema.pl` included, plus `README-dist.md`,
`NOTICE`, `LICENSE`, and `release-manifest.tsv`. `LICENSE` is the
committed repository license, copied unchanged. The profile in each guideline's
`rights.tsv` controls what ships. A `redistributable` guideline ships
whole. A `reconstructable` guideline ships without its `source/` files;
the manifest lists them as `source` rows with fetch URLs. A `restricted`
guideline is held back and appears as a document count alone. The
manifest carries a review label for every shipped document. A rejected
or contested verdict blocks the build.

The build is deterministic: one commit yields byte-identical archives on
any machine. `ckc check` rebuilds the archive twice, compares the
bytes, and re-verifies the checksums on every run. A
stale `release-manifest.tsv` fails the check.

The reviewer interface runs locally on loopback. Web hosting is out of
scope.

### Releasing

A release is a dated snapshot of the knowledge base. A release line
takes the snapshot from `main` to its publication. Development
continues on `main` while the line exists. The line receives only
three kinds of commits: review decisions, corrections of rejected
documents, and the release record.

A release line has the identifier `kb-YYYY-MM-DD`, the date when it
leaves `main`. A second line that leaves `main` on the same date adds
`.2`, `.3`, and so on. These names belong to the line:

- `release/kb-YYYY-MM-DD` is its branch.
- `kb-YYYY-MM-DD-rc.1`, `kb-YYYY-MM-DD-rc.2`, and so on are its release
  candidates, as annotated tags. A candidate is not reviewed.
- `kb-YYYY-MM-DD` is its release, as an annotated tag on the last
  commit of the line. The release notes and `CITATION.cff` state the
  date of the release.

The archive keeps its commit name; the release record outside the
archive binds the identifier to the archive bytes. Every document ships
with its review label; the build blocks a rejected or a contested
document only. A line freezes the schema identifier that it carries: a
later change of the predicate set or of a predicate meaning on `main`
takes the next identifier. The first release line is the first
publication of schema `1`.

To start a release line:

1. Commit the snapshot on `main`. `just gate` and CI must be green.
2. Create the branch `release/kb-YYYY-MM-DD` on that commit.
3. Run `ckc dist build`. Keep the archive and its `.sha256` sidecar.
4. Write the candidate notes, as in step 10 below. Create the annotated
   tag `kb-YYYY-MM-DD-rc.1` on the commit, with the notes as its
   message.
5. Add the review checkout:
   `git worktree add .release/kb-YYYY-MM-DD release/kb-YYYY-MM-DD`.
   In that directory, run
   `CKC_TOOLCHAIN=<repository root>/.toolchain just build`.

To review on the line:

6. In `.release/kb-YYYY-MM-DD`, run `rust/target/release/ckc ui serve`.
   The decisions go to the ledgers of that checkout.
7. Commit each batch of decisions on the branch. Then run
   `just amend-manifest` in the checkout, with `CKC_TOOLCHAIN` set as in
   step 5.
8. If a document is rejected, correct it on the branch and review it
   again. A corrected state gets the next candidate tag.

To release the line:

9. Confirm that the review is complete: `ckc check` prints the label
   counts (`adjudication <id> approved=…`). `just gate` and CI must be
   green on the last commit of the branch.
10. Run `ckc dist build`. Write the release notes. State the identifier,
    the date, the archive name and its SHA-256, the release commit, the
    schema identifier, the source edition and digest of each guideline,
    and the review label counts. Classify the change from the previous
    release twice: the schema change (`none`, or the new identifier) and
    the content change (`none`, `correction`, or `substantive` when a
    change alters whether a recommendation applies to a patient).
11. Create the annotated git tag `kb-YYYY-MM-DD` on that commit, with
    the release notes as its message.
12. Deposit the archive and its sidecar on Zenodo as a new version of
    the project record. Link the preprint that describes the release.
    The version DOI then names the archive bytes.
13. Update `CITATION.cff` on the branch: the version, the release date,
    and the version DOI.
14. Merge the line into `main` with
    `git merge --no-ff release/kb-YYYY-MM-DD`. If
    `release-manifest.tsv` conflicts, run `ckc release-manifest`. If the
    schema on `main` has changed, compile again. Decisions that bind
    clauses which `main` has changed read outdated on `main`.
15. Remove the review checkout:
    `git worktree remove .release/kb-YYYY-MM-DD`.

A consumer pins a release tag. A candidate tag serves integration work
before the review is complete.

## Licensing

First-party work outside `vendor/` is Apache-2.0 WITH LLVM-exception
(`LICENSE`). First-party additions inside a vendored tree adopt that
tree's license; `vendor/ape/prolog/ace_to_pl.pl` is LGPL-3.0-or-later.
Vendored trees keep their own licenses: `vendor/ape`
LGPL-3.0-or-later, `vendor/clex` GPL-3.0-or-later. Rust dependencies
under `rust/vendor/` keep their own permissive licenses
(`rust/trust/deps-allowlist.tsv`, `rust/deny.toml`).
Distribution of the full repository combination conveys under GPLv3.
First-party work remains Apache-2.0 WITH LLVM-exception and
independently reusable. See `NOTICE`.
