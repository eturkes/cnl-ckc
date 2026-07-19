# CNL IR v1

Status: normative for the project-owned ACE-to-Prolog boundary.

IR v1 is one ground, function-free, positive Datalog record per ACE document. It preserves document identity, sentence/clause identity, and token provenance. It does not carry native Prolog variables or executable Prolog syntax.

## Record grammar

A record is a UTF-8 term stream in this exact order:

```text
cnl_ir_record(1).
<document>
<fact>*
<rule>*
<query>
```

Facts and rules may be absent. Exactly one query is required and is the final term. Every term occupies one LF-terminated line.

The constructors are fixed:

```prolog
document(docid('<docid>'),source_sha256('<hex>'),ulex(none)).
document(docid('<docid>'),source_sha256('<hex>'),ulex(sha256('<hex>'))).

fact(
    fact_id(sentence(S),clause(C)),
    pred(Name,[GroundArg,...]),
    source(sentence(S),tokens([T1,...]))
).

rule(
    rule_id(sentence(S),clause(C)),
    pred(Name,[RuleArg,...]),
    body([BodyLiteral,...]),
    source(sentence(S),tokens([T1,...]))
).

query(
    query_id(sentence(S),clause(C)),
    pred(Name,[GroundArg,...]),
    source(sentence(S),tokens([T1,...]))
).

GroundArg  ::= named(Atom)
RuleArg    ::= named(Atom) | var(N)
BodyLiteral ::= pred(Name,[RuleArg,...])
              | naf(pred(Name,[RuleArg,...]))
```

The layout above is explanatory. Accepted bytes use the canonical single-line forms defined below.

Constraints:

- `S`, `C`, `N`, and every `T` are integers at least 1.
- `Name` is an open atom. Predicate identity is the pair `Name/Arity`.
- Predicate argument lists are proper and non-empty. Zero-arity predicates are outside v1.
- `named(Atom)` is the sole constant form. The atom is open.
- `var(N)` is data, not a native Prolog variable. Native variables are invalid.
- The sole sort is `entity`. V1 has no sort annotation constructor; every argument has that sort.
- Floats, strings, rationals, lists used as argument values, and other atomic or compound argument forms are invalid.
- A rule body is proper and non-empty in an accepted record.
- `naf/1` is reserved syntax. Its shape is parsed, but every occurrence is rejected in v1.
- Unknown constructors, arities, fields, or literal forms are hard errors. No pass drops or approximates unsupported content.

The second line is the `document/3` line copied byte-for-byte from the corresponding M2 `ace_front_end_record(1)` record. `<docid>` is non-empty, contains only `[a-z0-9-]`, and does not begin with `-`. `<hex>` is exactly 64 lowercase hexadecimal characters. Identity atoms remain single-quoted exactly as M2 emits them.

## IDs and provenance

Each item ID is kind-specific:

- fact: `fact_id(sentence(S),clause(C))`
- rule: `rule_id(sentence(S),clause(C))`
- query: `query_id(sentence(S),clause(C))`

The ID sentence must equal the item's `source/2` sentence. `(S,C)` pairs are globally unique across facts, rules, and the query. Within each section, IDs are strictly ascending lexicographically by `(S,C)`.

`source(sentence(S),tokens([T1,...]))` contains a non-empty, strictly ascending list of positive token ordinals. Ordinals refer to the M2 record identified by the same document line: source SHA-256 plus sentence ordinal gives sentence identity, and the token ordinal is local to that sentence.

## Variables and safety

A rule's `var(N)` values are local to that rule. Numbering is dense `1..k` in first-occurrence order while scanning the serialized rule left-to-right: head arguments first, then body literals and their arguments in list order. Repeated occurrences retain their first number.

Every head variable must occur in a positive body literal. Body-only variables are admitted: a head tuple is derived when some assignment to those body-only variables satisfies the body. Facts and queries are ground; a `var/1` in either is a `scope` error.

## Semantics

Ignoring provenance and IDs, a record denotes a finite positive, function-free Datalog program:

1. Each `fact/3` contributes its ground `pred/2` atom.
2. Each `rule/4` contributes one positive implication from its body atoms to its head atom.
3. The program meaning is its least Herbrand model over the `named/1` constants present in the record.
4. The single ground query is `proved` exactly when its atom belongs to that model; otherwise its outcome is `not_proved`.

`not_proved` is rendered as unknown, never false. V1 has no `false` outcome and no explicit-negation constructor.

Positive dependencies are directed from each rule-head predicate key to each positive body predicate key. Any directed cycle, including a self-loop, is rejected in v1. Recursion is deferred rather than assigned an implementation-dependent operational meaning.

### Reserved negation as failure

`naf(pred(Name,Args))` is the sole reserved NAF body-literal form. V1 rejects it with class `naf`; the inference semantics never execute it.

A future NAF-bearing version must be stratified. Equivalently, there must be a stratum mapping where a rule head's stratum is at least every positive dependency's stratum and strictly greater than every NAF dependency's stratum. V1 is trivially stratified because it admits no NAF literals.

## Canonical bytes

Input is decoded only after strict RFC 3629 UTF-8 validation. Overlong encodings, surrogate encodings, code points above `U+10FFFF`, stray continuation bytes, and truncated sequences are invalid.

All terms except `document/3` use `src/prolog/drs_canon.pl` `canonical_line/2`. Its effective writer contract is:

```prolog
write_term(Term,[
    quoted(true),
    ignore_ops(true),
    numbervars(true),
    character_escapes(true)
]).
```

The writer appends `.` and LF. Terms must be acyclic, contain no attributed variables or pre-existing `'$VAR'/1` term, and use only the canonical writer's admitted term kinds.

The M2 document line is the deliberate exception: M2 always single-quotes all three identity atoms, while `canonical_line/2` removes unnecessary quotes from plain atoms. The shared IR/program/answer record serializer uses the same atom escaping and writer options but forces those identity atoms to remain single-quoted only when term 2 has the complete `document(docid(Atom),source_sha256(Atom),ulex(none|sha256(Atom)))` serialization shape. Malformed document terms fall through to `canonical_line/2`, allowing the lower envelope or IR/program validator to assign its documented `envelope`, `shape`, or `identity` class instead of preempting it as unserializable. This forced-quote exception is the only way to satisfy byte-verbatim document provenance across all three record kinds and a fixed byte representation.

Validation reserializes the parsed term stream with that record serializer and requires exact equality with the decoded input. Strict UTF-8 makes text equality equivalent to byte equality. Therefore comments, blank lines, CRLF, a missing final LF, extra spaces, alternate operator/list notation, and alternate atom quoting fail the fixed-point gate. Every accepted record ends in exactly one LF.

Canonical serialization is performed on copies because `canonical_line/2` numbers native variables destructively. A native variable can survive the byte gate only in its canonical variable spelling; shape validation then rejects it. SWI strings and rationals lie outside `canonical_line/2` and therefore fail as `canonical` before shape. Floats are serializable in every field position, including all term-2 document fields, but no v1 field shape admits them, so they fail `shape`.

## Validator

`src/prolog/ir_validate.pl` owns IR semantic passes 4-10. `src/prolog/inference_kernel.pl` owns the isomorphic program-record passes and the inference schedule. `src/prolog/drs_to_ir.pl` owns the M2 envelope and DRS-to-IR semantics; `src/prolog/ir_to_prolog.pl` owns compilation; `src/prolog/explanation.pl` owns certificate construction and replay. `src/prolog/ir_tool.pl` owns byte decoding, parsing, the canonical fixed point, CLI framing, buffering, and error emission. The first failing pass wins. Within a pass, the first offending term in stream order wins; deterministic within-term checks use the order stated below.

1. **Stream/UTF-8** - read stdin as bytes and apply the strict decoder.
2. **Term syntax** - parse a sequence of Prolog terms with pinned reader flags and syntax errors promoted to exceptions.
3. **Canonical fixed point** - serialize every parsed term and compare the complete text.
4. **Envelope** - require `cnl_ir_record(1)`, the document line, exactly one query, no term after it, and facts before rules before the query. Query count is checked before trailing-term and interleave checks.
5. **Shape** - require the exact constructors, arities, proper lists, non-empty predicate arguments and token lists, and admitted atomic kinds. `body([])` and well-shaped `naf/1` deliberately survive this pass for dedicated errors later.
6. **Identity** - validate document fields; item-ID kind; positive ID/source/token ordinals; and ID/source sentence equality.
7. **Ordering/uniqueness** - per item, check global duplicate `(S,C)`, then section order, then strict token order.
8. **Scope** - reject data variables in facts/queries and enforce dense rule-local first-occurrence numbering.
9. **Safety/NAF** - per rule, reject the first NAF literal before checking empty body or head-variable coverage. This preserves the dedicated `naf` class.
10. **Cycles** - scan rules and body literals in record order; reject the first positive edge that closes a dependency cycle.

Program validation uses the same pass numbers, class vocabulary, first-failure rule, and stream-order discipline. Its envelope is `cnl_program_record(1)`, `document/3`, zero or more clauses, and one final `goal/2`; zero or multiple goals are `query_count`. Shape admits only the program grammar below while retaining well-shaped `naf/1` for pass 9. Identity checks document fields, ID/body-kind agreement, positive ordinals, and a `query_id` goal. Ordering checks global `(S,C)` uniqueness and strict order within the fact and rule sections. Scope, safety, NAF, and cycle checks are identical to their IR meanings after provenance erasure.

### Error classes

| Class | Pass/stage | Meaning | Exit |
|---|---|---|---:|
| `input_utf8` | Framing 1 | Stdin is not strict RFC 3629 UTF-8. | 1 |
| `syntax` | Framing 2 | The decoded term stream cannot be parsed. A leading UTF-8 BOM reaches this class under pinned SWI 9.2.9. | 1 |
| `canonical` | Framing 3 | Reserialized text differs, or a term is outside the canonical serializer's domain. | 1 |
| `envelope` | Validate/program 4 / lower | The selected record envelope is missing, malformed, wrong-versioned, or has trailing content. | 1 |
| `query_count` | Validate/program 4 | An IR record has zero or multiple `query/3` items, or a program record has zero or multiple `goal/2` items. | 1 |
| `section_order` | Validate/program 4 | A fact occurs after the rule section begins. | 1 |
| `shape` | Validate/program 5 | A constructor, arity, list, or admitted atomic kind is invalid. | 1 |
| `identity` | Validate/program 6 | Document identity, ID kind, body-kind agreement, ordinal bound, or IR sentence agreement is invalid. | 1 |
| `ordering` | Validate/program 7 | An ID is duplicated or out of section order, or IR token ordinals are not strictly ascending. | 1 |
| `scope` | Validate/program 8 | A variable occurs outside a rule or rule numbering is not dense first-occurrence order. | 1 |
| `naf` | Validate/program 9 | A reserved `naf/1` literal occurs. | 1 |
| `safety` | Validate/program 9 | An IR rule body is empty, or a head variable lacks positive-body coverage. | 1 |
| `cycle` | Validate/program 10 | A positive predicate dependency closes a cycle or self-loop. | 1 |
| `question_count` | Lower | The root DRS has zero or multiple questions, or its sole question is not final. | 1 |
| `wh_query` | Lower | A `query/2` wh condition occurs; v1 requires one ground yes/no query. | 1 |
| `copula` | Lower | A factual `object/6` and `be` pair is malformed, ambiguous, or unpaired. | 1 |
| `referent` | Lower | A DRS referent is undeclared, redeclared, unconsumed, unbound, role-reused, or not losslessly erasable as an event. | 1 |
| `unsupported` | Lower | A constructor or arrangement is outside the admitted DRS profile, provenance is not one-sentence canonical data, or lowering cannot produce valid v1 IR without loss. | 1 |
| `usage` | CLI | Arguments do not select exactly one implemented command (`lower`, `validate`, `compile`, or `run`). | 2 |
| `uncaught` | Any | An unexpected internal exception, including certificate replay failure or generated-record validation/serialization failure, escaped a stage. | 2 |

The stage atom is one of `cli`, `validate`, `lower`, `compile`, or `run`. The framing classes (`input_utf8`, `syntax`, `canonical`) plus the validation classes above are together the complete tamper-rejection surface for program artifacts read by `run`; compilation surfaces IR validation failures at stage `compile`.

## DRS lowering

Canonical lowering invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- lower <record.drs.pl >record.ir.pl
```

Input is exactly one canonical M2 record with three terms and no trailing content:

```prolog
ace_front_end_record(1).
document(docid('<docid>'),source_sha256('<hex>'),ulex(<ulex-term>)).
drs(Domain,Conditions).
```

The input passes the same strict UTF-8 decoder, pinned term parser, and canonical fixed-point gate as IR validation. Term 2 uses the same forced-quote document serializer. Consequently an admitted `document/3` line is copied byte-for-byte into output term 2. Lowering is buffered: no real stdout bytes are written until the complete output has passed IR v1 validation.

The admitted DRS profile is deliberately total and small:

- Root factual conditions may be exact copula pairs or ground intransitive predicates. `object(X,Class,countable,na,eq,1)` paired in the same root DRS with `predicate(E,be,named(Name),X)` becomes `pred(Class,[named(Name)])`. Both anchors contribute provenance. Every other factual `object` or `be` arrangement is rejected rather than approximated.
- `predicate(E,Verb,named(Name))` becomes one ground fact. A root fact may not retain a discourse referent as its subject.
- A rule is exactly `=>(drs(ADom,AConds),drs(CDom,CConds))`. Its antecedent is a non-empty list of positive exact `object/6` and intransitive `predicate/3` conditions. Its consequent is one intransitive `predicate/3` head. Antecedent referents remain in scope in the consequent; named subjects are also admitted. Nested implication, disjunction, negation, copulae inside rules, and all other constructors are rejected.
- Exactly one `question(drs(QDom,QConds))` must be the final root condition. It contains one ground intransitive predicate and becomes the sole IR query. Any `query/2` wh condition is rejected.
- A predicate event referent is erased only when it is declared by that same DRS domain, used exactly once as an event argument, and never reused as an entity. Domain declarations are unique across the admitted DRS tree. Every declared referent must be consumed by copula normalization, erased under this event law, or bound as a rule variable; undeclared, redeclared, unconsumed, event-reused, or unbound-head referents are errors.
- Rule variables are data terms `var(N)`. Numbering is dense in the validator's traversal order: head arguments first, then antecedent body literals in DRS order, each by first occurrence.
- An output item's sentence is the common `S` from every consumed `/(S,T)` anchor. Mixed-sentence items are rejected. Tokens are deduplicated and sorted strictly ascending. Clause IDs are per-sentence 1-based counters in root emission order. Facts must precede rules, each emitted section must have ascending `(S,C)` IDs, and the query remains final.

All other DRS content is a hard error. Lowering never drops a condition, silently weakens a term, or emits a partial record.

Lowering is also first-failure deterministic. Its order is: M2 envelope; root-domain shape; root question count/position; cross-DRS declaration uniqueness; root factual and rule conditions in DRS order; root referent accounting; final-question semantics; output section/order checks; generated-IR validation. A condition-local shape, copula, or groundness error therefore wins over later whole-scope referent accounting. For example, a root predicate whose subject is a domain referent is `unsupported` before a possible later event/entity-role conflict is considered.

## Program compilation

Canonical compilation invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- compile <record.ir.pl >record.program.pl
```

A program record is a ground term stream in this exact order:

```prolog
cnl_program_record(1).
document(docid('<docid>'),source_sha256('<hex>'),ulex(<ulex-term>)).
clause(fact_id(sentence(S),clause(C)),pred(Name,[GroundArg,...]),body([])).
clause(rule_id(sentence(S),clause(C)),pred(Name,[RuleArg,...]),body([BodyLiteral,...])).
goal(query_id(sentence(S),clause(C)),pred(Name,[GroundArg,...])).
```

Facts and rules may be absent. Exactly one goal is required and is final. Fact clauses precede rule clauses. The argument and body-literal forms are those of IR v1; `naf/1` remains reserved and rejected. The `document/3` line is byte-verbatim from the IR record and uses the same forced-quote serializer.

Compilation first applies full IR validation. It then drops every `source/2`, maps each fact to an empty-body `clause/3`, maps each rule to `clause/3` without changing its head, body order, or `var(N)` numbering, and maps the query to the final `goal/2`. Item order and IDs are preserved. This is a total map on valid IR v1. Before serialization, the generated program stream passes the program validator; therefore no partially transformed or internally invalid program can reach stdout.

Program identity binds clause kind to body shape: `fact_id` if and only if the body is `[]`, and `rule_id` if and only if the body is non-empty. The goal ID is `query_id`. `(S,C)` is globally unique across clauses and the goal, and IDs are strictly ascending within the fact section and within the rule section. Facts and the goal are ground. Rule variables are dense first-occurrence `1..K` over head then body, every head variable occurs in a body literal, and positive predicate dependencies are acyclic.

## Inference and answer records

Canonical inference invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- run <record.program.pl >record.result.pl
```

A result record is a ground term stream in one of these exact forms:

```prolog
cnl_answer_record(1).
document(docid('<docid>'),source_sha256('<hex>'),ulex(<ulex-term>)).
answer(query_id(sentence(S),clause(C)),pred(Name,[GroundArg,...]),proved).
proof(Atom,ClauseId,[SubProof,...]).
```

```prolog
cnl_answer_record(1).
document(docid('<docid>'),source_sha256('<hex>'),ulex(<ulex-term>)).
answer(query_id(sentence(S),clause(C)),pred(Name,[GroundArg,...]),not_proved).
```

The document line is byte-verbatim from the program record. Exactly one `proof/3` term is present if and only if the answer is `proved`; it is the final term. `Atom` is a derived ground `pred/2`, `ClauseId` is the cited `fact_id` or `rule_id`, and subproofs correspond one-for-one with the cited clause body literals in body order. A fact proof has `[]`. The root atom is `==` to both the answer atom and the program goal. `not_proved` remains unknown and carries no proof. V1 certificates are trees without shared subproofs, so their size can grow exponentially with rule structure; DAG sharing and proof-resource bounds remain deferred.

`run` validates a program artifact independently rather than trusting the compiler. This is the defense-in-depth tamper boundary: after the shared framing gates (`input_utf8`, `syntax`, `canonical`), the same `envelope`, `query_count`, `section_order`, `shape`, `identity`, `ordering`, `scope`, `naf`, `safety`, and `cycle` classes define all content rejections. A result record deliberately does not bind a program digest in v1; digest binding is deferred to M4.

The deterministic least-Herbrand-model schedule is normative:

1. Inside `setup_call_cleanup/3`, validated clauses are asserted only as `cnl_program_db:program_clause(Seq,Id,Head,Body)` data facts in stream `Seq` order. Setup starts from `retractall/1`; cleanup always performs full `retractall/1` teardown.
2. The derived-atom store is an ordered list in insertion order. Each entry contains one ground `pred/2` and its witness `by(ClauseId,BodyAtoms)`.
3. Evaluation repeats full passes over clauses in ascending `Seq`. A clause takes one snapshot of the atom list at clause entry.
4. Body literals are matched left-to-right against that snapshot. The leftmost literal is the outermost loop; each literal enumerates snapshot atoms in insertion order. Matching builds an explicit `var(N) -> named(Atom)` map by structural decomposition and `==` comparison, never by native unification against program terms.
5. Every resulting ground head is considered in that enumeration order. If an `==`-equal atom is absent, it is appended with the current clause ID and matched body atoms; otherwise it is ignored. The first witness for an atom is retained forever.
6. Additions are visible to later clauses in the same pass, but a clause's own matches continue to use its entry snapshot. A full pass that appends nothing is the fixpoint.

The constant universe is finite and validated positive dependencies are acyclic, so this schedule terminates. Predicate names are always data, including names that collide with Prolog predicates or database operations.

For a proved goal, `src/prolog/explanation.pl` expands retained witnesses recursively into one ground proof tree and then independently replays it before any output is emitted. Replay requires: (a) root conclusion `==` the goal; (b) every cited clause exists and admits a total ground substitution over all of its `var(N)` values such that the substituted head is `==` the node atom and substituted body atoms are pairwise `==` child conclusions in order; and (c) every child recursively satisfies the same checks. Replay uses its own structural matcher. It certifies derivability (soundness) only; first-witness and schedule integrity are enforced by the kernel's deterministic schedule and fresh-process determinism gates, not by replay. Failure is an internal invariant break and surfaces as stage `run`, class `uncaught`, exit 2. Only after replay succeeds is the result stream canonically serialized, reparsed, checked as its own fixed point, and committed once to stdout.

## End-to-end document chain

For each `<docid>`, the project-owned artifact chain is
`<docid>.drs.pl` → `<docid>.ir.pl` → `<docid>.program.pl` → `<docid>.result.pl`.

The front end emits `<docid>.drs.pl` plus `manifest.pl` for the complete validated document set. Lowering, validation, compilation, and inference then run in separate ambient-init-free SWI processes. Canonical repository-root invocations are:

```sh
ape_tree=/path/to/fresh-ape-tree
docs_dir=/path/to/docs
out_dir=/path/to/new-output
docid=slice

SWIPL=swipl PYTHONDONTWRITEBYTECODE=1 \
  python3 -P tools/ace_front_end.py "$ape_tree" "$docs_dir" "$out_dir"
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- \
  lower <"$out_dir/$docid.drs.pl" >"$out_dir/$docid.ir.pl"
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- \
  validate <"$out_dir/$docid.ir.pl"
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- \
  compile <"$out_dir/$docid.ir.pl" >"$out_dir/$docid.program.pl"
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- \
  run <"$out_dir/$docid.program.pl" >"$out_dir/$docid.result.pl"
```

`validate` succeeds with zero stdout and stderr; each transforming stage commits one complete canonical record only after its own checks pass. Byte authorities live under `tests/fixtures/slice/`: `golden/manifest.pl` and `golden/<docid>.drs.pl`, then `ir/<docid>.ir.pl`, `program/<docid>.program.pl`, and `result/<docid>.result.pl`. A generated mismatch is a contract failure, not a regeneration instruction.

`tests/slice-harness.sh` stages and builds a fresh APE copy, produces two fresh front-end output trees, and chains each fresh DRS through `lower` → `validate` → `compile` → `run`. It byte-compares each artifact-producing stage to its committed golden before continuing and treats `validate` as a separate zero-stream success gate, pins each pass's complete chain file inventory, proves the two passes' complete artifact sets byte-identical, proves the staged APE tree unchanged across both passes, reuses the chain driver on a trailing-term rejection to prove zero non-empty downstream artifacts, and verifies `vendor/` cleanliness before staging and after both passes. CI runs this harness in the pinned SWI 9.2.9 `ape` job after `tests/pipeline-harness.sh` and before the final repository-cleanliness gate.

## CLI

Canonical IR validation invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- validate <record.ir.pl
```

All four commands are implemented: `lower | validate | compile | run`. `lower` accepts one canonical M2 DRS record, `validate` accepts one canonical IR v1 record, `compile` accepts one canonical IR v1 record, and `run` accepts one canonical program v1 record. Missing arguments, an unknown command, or extra arguments are `usage` errors.

The tool pins encoding, double-quote, back-quote, character-escape, syntax-error, and writer behavior; it does not depend on ambient SWI defaults. `-f none -F none` remains part of every canonical process invocation.

I/O contract:

- Validate success: exit 0, zero stdout, zero stderr.
- Lower success: exit 0, one canonical IR v1 record on stdout, zero stderr.
- Compile success: exit 0, one canonical program v1 record on stdout, zero stderr.
- Run success: exit 0, one canonical answer v1 record on stdout, zero stderr.
- Any input-content rejection: exit 1, zero stdout, exactly one LF-terminated stderr line.
- Usage or uncaught internal failure: exit 2, zero stdout, exactly one LF-terminated stderr line.
- All prospective stage output is captured in memory. Real stdout is flushed once, only after the complete stage and every generated-record self-check succeed.

These byte guarantees cover record processing with writable output streams. An operating-system sink failure after the single stdout commit begins is outside the transactional input-error surface because already accepted pipe or file bytes cannot be retracted.

Error form:

```prolog
ir_tool_error(Stage,Class,Detail).
```

After successful dispatch, `Stage` is `lower`, `validate`, `compile`, or `run`; pre-dispatch usage errors use `cli`. The error term uses the same canonical writer. If its detail cannot be serialized, the deterministic replacement detail is `unserializable`; a final fixed `ir_tool_error(cli,uncaught,unserializable).` line is the serialization backstop.

## Versioning

`cnl_ir_record(1)`, `cnl_program_record(1)`, and `cnl_answer_record(1)` are independently versioned envelopes. Any change to a record's constructors, arities, admitted argument forms, section cardinality/order, identity/provenance rules, canonical bytes, or logical semantics requires a new version of that envelope. Readers reject unknown versions. V1 records have no ignored extension field.

## Deferred beyond v1

| Area | V1 treatment |
|---|---|
| Intervals and rationals | No constructors; reject. |
| Temporal and dose algebra | No constructors; reject. |
| Direction, strength, and certainty | No annotations; reject. |
| Explicit negation and a false outcome | Absent; `not_proved` remains unknown. |
| NAF execution and exceptions | `naf/1` reserved and rejected. |
| Recursion and tabling | Positive cycles rejected. |
| Conflict detection | No rule-pair conflict analysis. |
| Proof enumeration and DAG sharing | No proof artifact in the IR. |
| Prose rendering | Outside the IR. |
| Program-digest binding inside answer records | Answer-record format exists; digest binding is deferred to M4. |
| Wh answers | Exactly one ground yes/no query only. |
| Multi-document composition | Exactly one document record per run. |

## Grounding example

The probed slice:

```text
John is a patient.
John waits.
Every patient that waits recovers.
Does John recover?
```

is represented as:

```prolog
cnl_ir_record(1).
document(docid('slice'),source_sha256('bf432c59846951be8568be538cfa2c5fcdc41d35b7ede4d0bc0fd5c4aff7c2c4'),ulex(sha256('6015f9a18e4d4957b30e04342d2ff2700bf0e18b13bf3b95452a2d5563c5b614'))).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred(wait,[named('John')]),source(sentence(2),tokens([2]))).
rule(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])]),source(sentence(3),tokens([2,4,5]))).
query(query_id(sentence(4),clause(1)),pred(recover,[named('John')]),source(sentence(4),tokens([3]))).
```
