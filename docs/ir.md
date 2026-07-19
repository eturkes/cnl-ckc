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

The M2 document line is the deliberate exception: M2 always single-quotes all three identity atoms, while `canonical_line/2` removes unnecessary quotes from plain atoms. The IR document-line serializer uses the same atom escaping and writer options but forces those identity atoms to remain single-quoted. This is the only way to satisfy both byte-verbatim M2 provenance and a fixed byte representation.

Validation reserializes the parsed term stream with that record serializer and requires exact equality with the decoded input. Strict UTF-8 makes text equality equivalent to byte equality. Therefore comments, blank lines, CRLF, a missing final LF, extra spaces, alternate operator/list notation, and alternate atom quoting fail the fixed-point gate. Every accepted record ends in exactly one LF.

Canonical serialization is performed on copies because `canonical_line/2` numbers native variables destructively. A native variable can survive the byte gate only in its canonical variable spelling; shape validation then rejects it. SWI strings and rationals lie outside `canonical_line/2` and therefore fail as `canonical` before shape. Floats are serializable but fail `shape`.

## Validator

`src/prolog/ir_validate.pl` owns semantic passes 4-10. `src/prolog/ir_tool.pl` owns byte decoding, parsing, the canonical fixed point, CLI framing, buffering, and error emission. The first failing pass wins. Within a pass, the first offending term in stream order wins; deterministic within-term checks use the order stated below.

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

### Error classes

| Class | Pass/stage | Meaning | Exit |
|---|---|---|---:|
| `input_utf8` | 1 | Stdin is not strict RFC 3629 UTF-8. | 1 |
| `syntax` | 2 | The decoded term stream cannot be parsed. A leading UTF-8 BOM reaches this class under pinned SWI 9.2.9. | 1 |
| `canonical` | 3 | Reserialized text differs, or a term is outside the canonical serializer's domain. | 1 |
| `envelope` | 4 | Header/document envelope is missing or wrong, or content follows the sole query. | 1 |
| `query_count` | 4 | The record has zero or more than one `query/3` item. | 1 |
| `section_order` | 4 | A fact occurs after the rule section begins. | 1 |
| `shape` | 5 | A constructor, arity, list, or admitted atomic kind is invalid. | 1 |
| `identity` | 6 | Document identity, ID kind, ordinal bound, or sentence agreement is invalid. | 1 |
| `ordering` | 7 | An ID is duplicated/out of section order, or token ordinals are not strictly ascending. | 1 |
| `scope` | 8 | A variable occurs outside a rule or rule numbering is not dense first-occurrence order. | 1 |
| `naf` | 9 | A reserved `naf/1` literal occurs. | 1 |
| `safety` | 9 | A rule body is empty or a head variable lacks positive-body coverage. | 1 |
| `cycle` | 10 | A positive predicate dependency closes a cycle or self-loop. | 1 |
| `usage` | CLI | Arguments are not exactly `validate`. | 2 |
| `uncaught` | Any | An unexpected internal exception escaped a stage. | 2 |

## CLI

Canonical validation invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- validate <record.pl
```

The planned command surface is `lower | validate | compile | run`. In this unit only `validate` is implemented. Missing arguments, an unimplemented/unknown command, or extra arguments are `usage` errors.

The tool pins encoding, double-quote, back-quote, character-escape, syntax-error, and writer behavior; it does not depend on ambient SWI defaults. `-f none -F none` remains part of every canonical process invocation.

I/O contract:

- Validate success: exit 0, zero stdout, zero stderr.
- Any input-content rejection: exit 1, zero stdout, exactly one LF-terminated stderr line.
- Usage or uncaught internal failure: exit 2, zero stdout, exactly one LF-terminated stderr line.
- All prospective stage output is captured in memory. Real stdout is flushed once, only after a successful stage; this framing also applies to later output-producing subcommands.

Error form:

```prolog
ir_tool_error(Stage,Class,Detail).
```

`Stage` is `validate` after successful subcommand dispatch. Pre-dispatch usage errors use `cli`. The error term uses the same canonical writer. If its detail cannot be serialized, the deterministic replacement detail is `unserializable`; a final fixed `ir_tool_error(cli,uncaught,unserializable).` line is the serialization backstop.

## Versioning

Any change to constructors, arities, admitted argument forms, section cardinality/order, identity/provenance rules, canonical bytes, or logical semantics requires a new `cnl_ir_record(N)` version. Readers reject unknown versions. V1 has no ignored extension field.

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
| Program-digest binding inside answer records | Answer-record format deferred. |
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
document(docid('slice'),source_sha256('0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef'),ulex(sha256('1111111111111111111111111111111111111111111111111111111111111111'))).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred(wait,[named('John')]),source(sentence(2),tokens([2]))).
rule(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])]),source(sentence(3),tokens([2,4,5]))).
query(query_id(sentence(4),clause(1)),pred(recover,[named('John')]),source(sentence(4),tokens([3]))).
```
