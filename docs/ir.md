# CNL IR and execution records

Status: normative for the project-owned ACE-to-Prolog boundary and the run-side execution artifacts.

This document defines IR v1, program record v2, and answer record v2. IR v1 is one ground, function-free, positive Datalog record per ACE document. It preserves document identity, sentence/clause identity, and token provenance. It does not carry native Prolog variables or executable Prolog syntax. Program v2 adds run-side negation as failure and executable `wh(who)` goals; answer v2 binds the exact program bytes and carries NAF and wh certificates.

## IR v1 record grammar

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

## IR v1 variables and safety

A rule's `var(N)` values are local to that rule. Numbering is dense `1..k` in first-occurrence order while scanning the serialized rule left-to-right: head arguments first, then body literals and their arguments in list order. Repeated occurrences retain their first number.

Every head variable must occur in a positive body literal. Body-only variables are admitted: a head tuple is derived when some assignment to those body-only variables satisfies the body. Facts and queries are ground; a `var/1` in either is a `scope` error.

## IR v1 semantics

Ignoring provenance and IDs, a record denotes a finite positive, function-free Datalog program:

1. Each `fact/3` contributes its ground `pred/2` atom.
2. Each `rule/4` contributes one positive implication from its body atoms to its head atom.
3. The program meaning is its least Herbrand model over the `named/1` constants present in the record.
4. The single ground query is `proved` exactly when its atom belongs to that model; otherwise its outcome is `not_proved`.

`not_proved` is rendered as unknown, never false. V1 has no `false` outcome and no explicit-negation constructor.

Positive dependencies are directed from each rule-head predicate key to each positive body predicate key. Any directed cycle, including a self-loop, is rejected in v1. Recursion is deferred rather than assigned an implementation-dependent operational meaning.

### Reserved negation as failure

`naf(pred(Name,Args))` is the sole reserved NAF body-literal form. IR v1 still rejects every occurrence with class `naf`; the same rejection is surfaced at stage `compile` when an IR v1 artifact containing NAF reaches compilation. Program record v2, defined below, admits and executes this form only in rule bodies. ACE-side NAF lowering remains outside IR v1 and is deferred to M4.2.

## Canonical bytes

Input is decoded only after strict RFC 3629 UTF-8 validation. Overlong encodings, surrogate encodings, code points above `U+10FFFF`, stray continuation bytes, and truncated sequences are invalid.

The base serializer is `src/prolog/drs_canon.pl` `canonical_line/2`. Its effective writer contract is:

```prolog
write_term(Term,[
    quoted(true),
    ignore_ops(true),
    numbervars(true),
    character_escapes(true)
]).
```

The writer appends `.` and LF. Terms must be acyclic, contain no attributed variables or pre-existing `'$VAR'/1` term, and use only the canonical writer's admitted term kinds.

The shared IR/program/answer serializer has exactly two shape-guarded forced-quote exceptions. Term 2 is forced only after proving the complete `document(docid(Atom),source_sha256(Atom),ulex(none|sha256(Atom)))` shape, preserving the byte-verbatim M2 identity line. In an answer record, term 3 is forced only after proving the complete `program(sha256(Atom))` shape, so its digest atom always remains single-quoted. A malformed document or digest lookalike falls through to `canonical_line/2`; in particular, malformed digest terms do not enter the forced-quote branch. The complete-shape proof keeps program and IR term 3, which is ordinarily a clause or fact, on the generic serializer path.

Validation reserializes the parsed term stream with this record serializer and requires exact equality with the decoded input. Strict UTF-8 makes text equality equivalent to byte equality. Therefore comments, blank lines, CRLF, a missing final LF, extra spaces, alternate operator/list notation, and alternate atom quoting fail the fixed-point gate. Every accepted record ends in exactly one LF.

Canonical serialization is performed on copies because `canonical_line/2` numbers native variables destructively. A native variable can survive the byte gate only in its canonical variable spelling; shape validation then rejects it. SWI strings and rationals lie outside `canonical_line/2` and therefore fail as `canonical` before shape. Floats are serializable in every field position, including all term-2 document fields, but no defined record field shape admits them, so they fail `shape`.

## Validator

`src/prolog/ir_validate.pl` owns IR semantic passes 4-10. `src/prolog/inference_kernel.pl` owns the isomorphic program-record passes and the inference schedule. `src/prolog/drs_to_ir.pl` owns the M2 envelope and DRS-to-IR semantics; `src/prolog/ir_to_prolog.pl` owns compilation; `src/prolog/explanation.pl` owns certificate construction and replay. `src/prolog/ir_tool.pl` owns byte decoding, parsing, the canonical fixed point, CLI framing, buffering, and error emission. The first failing pass wins. Within a pass, the first offending term in stream order wins; deterministic within-term checks use the order stated below.

1. **Stream/UTF-8** - read stdin as bytes and apply the strict decoder.
2. **Term syntax** - parse a sequence of Prolog terms with pinned reader flags and syntax errors promoted to exceptions.
3. **Canonical fixed point** - serialize every parsed term and compare the complete text.
4. **Envelope** - IR validation requires `cnl_ir_record(1)`, the document line, exactly one query, no term after it, and facts before rules before the query. Query count is checked before trailing-term and interleave checks.
5. **Shape** - require the exact constructors, arities, proper lists, non-empty predicate arguments and token lists, and admitted atomic kinds. In IR v1, `body([])` and well-shaped `naf/1` deliberately survive this pass for dedicated errors later.
6. **Identity** - validate document fields; item-ID kind; positive ID/source/token ordinals; and ID/source sentence equality.
7. **Ordering/uniqueness** - per item, check global duplicate `(S,C)`, then section order, then strict token order.
8. **Scope** - reject data variables in IR facts/queries and enforce dense rule-local first-occurrence numbering.
9. **Safety/NAF** - in IR v1, reject the first NAF literal before checking empty body or head-variable coverage, preserving the dedicated `naf` class.
10. **Cycles** - in IR v1, scan rules and positive body literals in record order and reject the first positive edge that closes a dependency cycle.

Program validation uses the same pass numbers, class vocabulary, first-failure rule, and stream-order discipline, but applies the program v2 rules. Its envelope is `cnl_program_record(2)`, `document/3`, zero or more clauses, and one final goal. A goal is either yes/no `goal/2` or the exact admitted wh `goal/3`; zero or multiple goals are `query_count`. Shape admits `naf(pred(...))` only as a rule-body literal and admits only the exact wh marker and pattern defined below. NAF in a head, goal predicate slot, predicate argument, or any other non-body position belongs to the position-owning shape check and is `shape`. Identity owns clause-kind/body-shape agreement: a `fact_id` with a non-empty body or a `rule_id` with `body([])` fails pass 6 as `identity`. Scope keeps facts and yes/no goals ground, admits the wh query variable exactly as `var(1)`, and retains dense rule-local numbering. For rules that reach pass 9, the implementation checks in this exact deterministic order: a positive literal after any NAF literal, a defensive empty-body assertion, the first NAF variable absent from all positive literals, then the first head variable absent from all positive literals. The empty-body assertion is not externally reachable because pass 6 owns `rule_id` plus `body([])`; the remaining violations are `safety`. Pass 10 scans positive and negative body edges in rule/body stream order and rejects the first edge that closes any directed cycle.

### Error classes

| Class | Pass/stage | Meaning | Exit |
|---|---|---|---:|
| `input_utf8` | Framing 1 | Stdin is not strict RFC 3629 UTF-8. | 1 |
| `syntax` | Framing 2 | The decoded term stream cannot be parsed. A leading UTF-8 BOM reaches this class under pinned SWI 9.2.9. | 1 |
| `canonical` | Framing 3 | Reserialized text differs, or a term is outside the canonical serializer's domain. | 1 |
| `envelope` | Validate/program 4 / lower | The selected record envelope is missing, malformed, wrong-versioned, or has trailing content. | 1 |
| `query_count` | Validate/program 4 | An IR record has zero or multiple `query/3` items, or a program v2 record has zero or multiple final goals across admitted `goal/2` and `goal/3` arities. | 1 |
| `section_order` | Validate/program 4 | A fact occurs after the rule section begins. | 1 |
| `shape` | Validate/program 5 | A constructor, arity, list, or admitted atomic kind is invalid. | 1 |
| `identity` | Validate/program 6 | Document identity, ID kind, body-kind agreement, ordinal bound, or IR sentence agreement is invalid. In program v2, `rule_id` with `body([])` and `fact_id` with a non-empty body are body-kind mismatches. | 1 |
| `ordering` | Validate/program 7 | An ID is duplicated or out of section order, or IR token ordinals are not strictly ascending. | 1 |
| `scope` | Validate/program 8 | A variable occurs outside its admitted position, or rule numbering is not dense first-occurrence order. Program v2's exact wh query variable `var(1)` is admitted. | 1 |
| `naf` | IR validate 9 / compile | A reserved `naf/1` literal occurs in IR v1. Compilation surfaces the same IR-v1 rejection at stage `compile`; program v2 has no blanket `naf` rejection. | 1 |
| `safety` | Validate/program 9 | An IR rule body is empty or its head lacks positive-body coverage; or a program v2 rule violates positives-then-NAF order, has an NAF variable absent from its positive literals, or has a head variable absent from its positive literals. Program `rule_id` plus `body([])` fails earlier as `identity`. | 1 |
| `cycle` | Validate/program 10 | A positive edge closes a cycle in IR v1, or a positive or negative edge closes a cycle in the program v2 signed dependency graph; self-loops count. | 1 |
| `question_count` | Lower | The root DRS has zero or multiple questions, or its sole question is not final. | 1 |
| `wh_query` | Lower | Lowering still rejects an ACE `query/2` wh condition for IR v1. Program v2 `run` executes the exact admitted `wh(who)` goal and does not emit this class. | 1 |
| `resource` | Run | The total expanded nodes across all proof trees the answer record would emit exceeds `certificate_node_cap(1000000)`. | 1 |
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

## Program compilation and program record v2

Canonical compilation invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- compile <record.ir.pl >record.program.pl
```

A program v2 record is a ground term stream in this exact order:

```text
cnl_program_record(2).
<document>
<clause>*
<goal>
```

The complete constructors are:

```prolog
clause(
    fact_id(sentence(S),clause(C)),
    pred(Name,[GroundArg,...]),
    body([])
).
clause(
    rule_id(sentence(S),clause(C)),
    pred(Name,[RuleArg,...]),
    body([ProgramBodyLiteral,...])
).
goal(
    query_id(sentence(S),clause(C)),
    pred(Name,[GroundArg,...])
).
goal(
    query_id(sentence(S),clause(C)),
    wh(who),
    pred(Name,[var(1)])
).

GroundArg ::= named(Atom)
RuleArg ::= named(Atom) | var(N)
ProgramBodyLiteral ::= pred(Name,[RuleArg,...])
                     | naf(pred(Name,[RuleArg,...]))
```

Facts and rules may be absent. Exactly one goal is required and is final. Fact clauses precede rule clauses. Program identity binds clause kind to body shape: `fact_id` if and only if the body is `[]`, and `rule_id` if and only if the body is non-empty. The goal ID is `query_id`. `(S,C)` is globally unique across clauses and the goal, and IDs are strictly ascending within the fact section and within the rule section.

A yes/no goal remains `goal(query_id(sentence(S),clause(C)),pred(Name,[GroundArg,...]))`; its predicate is ground. The admitted wh form is exactly `goal(query_id(sentence(S),clause(C)),wh(who),pred(Name,[var(1)]))`. The marker must be exactly `wh(who)`, the predicate must have exactly one argument, and that argument must be exactly the query variable `var(1)`. Any other `goal/3` content is `shape`. Program v2 documents and validates this wh constructor now; execution and answer emission are staged as described below.

A rule body may contain positive `pred/2` literals and NAF literals of the sole form `naf(pred(Name,Args))`. Canonical body order is all positive literals first and then all NAF literals, preserving source order within each block. Any positive literal after an NAF literal is `safety`. Every variable in an NAF literal must occur in some positive literal of the same rule. Every head variable must likewise occur in a positive body literal; NAF does not provide head-variable coverage. A ground NAF-only body is admitted, but a rule body remains non-empty. NAF in a head, a yes/no goal's predicate slot, a wh goal's pattern, a predicate argument, or any other non-body position is owned by that position's shape pass and is `shape`; program v2 has no blanket `naf` class.

The finite function-free predicate dependency graph has one key `Name/Arity` per predicate. A positive edge goes from each rule-head key to each positive body-literal key. A negative edge goes from each rule-head key to each NAF-target key. Any directed cycle over the combined signed graph is rejected, including a positive-only cycle, a mixed-polarity cycle, or a self-loop. This cycle-free predicate dependency graph is a sufficiency-only restriction: full cycle freedom is stronger than stratification, because stratification permits positive-only cycles, so a stratification trivially exists for every admitted program. Finiteness is essential to that statement; on an infinite ground dependency graph, merely excluding negative cycles would not by itself establish the required well-founded stratum assignment.

### Program v2 semantics

With NAF present there is generally no global least model. For example, the rule `p :- naf q` has incomparable minimal classical models `{p}` and `{q}`. Program v2 therefore uses the **standard (stratified) model**: it is minimal and supported, and is computed as a least fixpoint separately at each stratum while all lower strata are frozen as extensional facts. This is the standard construction of Apt, Blair, and Walker, *Foundations of Deductive Databases*, chapter 2, Theorems 7, 8, and 11; Abiteboul, Hull, and Vianu, section 15.2, Theorem 15.2.10 and Proposition 15.2.11, which states “minimal, not necessarily least”; and Green et al., *Foundations and Trends in Databases*, section 2.3.2, where negated predicates are treated as EDBs during a stratum's positive fixpoint.

On this finite cycle-free profile, the standard model coincides with the unique perfect model, the unique stable model, and the total well-founded model. The coincidence follows from Apt and Bol, Theorem 6.10, Theorem 6.20, and Corollaries 7.6-7.7, together with Van Gelder, Ross, and Schlipf, *JACM* 38(3), Theorem 6.1. Thus the admitted NAF reading is robust across these standard semantics.

`not provably P` means that `P` has no derivation under the closed program; it never means classical falsity. This is the closed-world reading, grounded in the van Emden-Kowalski least-model basis for each positive fragment. If an NAF target predicate has no facts or defining clauses, its ground atom is underivable and the NAF test succeeds, as required by that reading.

### Normative stratified kernel schedule

For a predicate key `p`, its stratum is the maximum, over all clauses whose head key is `p`, of every positive dependency's stratum and one plus every NAF dependency's stratum, with minimum 1. Facts contribute a lower bound of 1. A predicate key with no defining clauses has stratum 1. The assignment is well-defined because the signed predicate graph is finite and cycle-free.

Evaluation proceeds in ascending stratum order. A clause belongs to its head key's stratum. Within one stratum, the v1 repeated-pass snapshot fixpoint runs unchanged but is restricted to that stratum's clauses: clauses are visited in ascending stream sequence; each clause sees one store snapshot taken at clause entry; positive body matching is leftmost-outermost over snapshot insertion order; generated heads are inserted immediately into the one growing, insertion-ordered store; `nb_setarg/3`-backed growing-store deduplication retains only the first witness and never materializes all candidate tuples before deduplication. A pass that inserts nothing completes the stratum. The store grows monotonically across strata.

An NAF literal first substitutes its pattern under the bindings established by preceding positive literals. The kernel asserts that the resulting `pred/2` is ground; failure of that internal invariant is stage `run`, class `uncaught`, exit 2. The NAF literal succeeds exactly when the substituted atom is absent from the clause-entry snapshot. Stratification guarantees that the target predicate belongs to a lower, already completed stratum, so snapshot absence is equal to absence from the final completed store. A successful NAF literal contributes the positional witness marker `naf(GroundAtom)`.

A single-stratum NAF-free program evaluates exactly as v1. Apart from the required program/answer envelope changes and the answer digest line, its result bytes and first-witness proof tree are unchanged.

Compilation first applies full IR v1 validation. It drops every `source/2`, maps a fact to an empty-body `clause/3`, maps a rule to `clause/3` without changing its head, body order, or `var(N)` numbering, and maps the query to the final yes/no `goal/2`. Item order and IDs are preserved. This is a total map from valid IR v1 to program v2; IR v1 does not yet emit NAF or wh goals. Before serialization, the generated program stream passes the program v2 validator, so no partially transformed or internally invalid program can reach stdout.

The `run` reader requires exactly `cnl_program_record(2)`. A program v1 record and every other header version fail `envelope` before later shape or safety checks.

## Inference and answer record v2

Canonical inference invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- run <record.program.pl >record.result.pl
```

A yes/no answer record has one of these exact forms:

```prolog
cnl_answer_record(2).
document(docid('<docid>'),source_sha256('<hex>'),ulex(<ulex-term>)).
program(sha256('<hex>')).
answer(query_id(sentence(S),clause(C)),pred(Name,[GroundArg,...]),proved).
proof(Atom,ClauseId,[SubProof,...]).
```

```prolog
cnl_answer_record(2).
document(docid('<docid>'),source_sha256('<hex>'),ulex(<ulex-term>)).
program(sha256('<hex>')).
answer(query_id(sentence(S),clause(C)),pred(Name,[GroundArg,...]),not_proved).
```

Line 3 is mandatory and immediately follows the document line. Its digest is exactly 64 lowercase hexadecimal characters: SHA-256 over the exact raw program-record bytes read by `run`, before UTF-8 decoding, including the final LF. The digest atom is always single-quoted. Some valid hashes begin with a digit and therefore require quotes as Prolog atoms; forcing quotes for every hash is the only uniform fixed byte representation. This digest binds the answer to the program bytes and detects accidental or adversarial modification when independently recomputed, but it is integrity metadata, not authentication.

The document line remains byte-verbatim from the program record. Exactly one top-level `proof/3` term is present if and only if a yes/no answer is `proved`; it is final. `Atom` is a derived ground `pred/2`, `ClauseId` is the cited `fact_id` or `rule_id`, and proof children correspond one-for-one with the cited clause body literals in body order. A fact proof has `[]`. The root atom is `==` to both the answer atom and program goal. `not_proved` remains unknown and carries no proof.

Proof children have this complete grammar:

```text
SubProof ::= proof(Atom,ClauseId,[SubProof,...])
           | naf(Atom)
```

A positive body literal corresponds to a recursive `proof/3` child. An NAF body literal corresponds positionally to a ground leaf `naf(Atom)`, never to a `proof/3` wrapper. Certificate construction follows the first witness retained by the kernel.

Replay independently checks a total ground substitution for every cited clause. For a positive child, the substituted body predicate must be `==` to the child's conclusion and the child must recursively replay. For an NAF child, replay requires the exact `naf/1` shape, a ground `pred/2` whose arguments are `named/1`, positional correspondence with the cited clause's NAF pattern under the bindings already established by positive literals, and `==`-absence of that atom from the completed kernel store. NAF leaves never introduce bindings. Proper `'[|]'/2` list structure is checked throughout, and all variables, including variables appearing in NAF patterns, must have total ground bindings.

The trust boundary is explicit. Saturation and first-witness selection are kernel responsibilities. Replay checks NAF absence against the kernel's completed store, not against syntax alone. Fresh-process deterministic runs make that store reproducible, but an external verifier must recompute the standard model from the bound program bytes to verify both derivability and NAF absence independently. Replay certifies the emitted tree's sound correspondence to the completed store; it does not replace saturation.

Certificates remain trees without shared subproofs, so their size can grow exponentially with rule structure. DAG sharing remains deferred. The kernel predicate `certificate_node_cap(1000000)` defines the proof-emission preflight bound. After the completed model and the complete answer set are known, but before any proof tree is constructed, the kernel memoizes an expanded-node count for each retained first-witness atom using `==` keys. An atom contributes one node plus each witness-body position: a positive child contributes that child's memoized count, and an NAF leaf contributes one. Every count and sum saturates at `Cap+1`.

The preflight total is the sum across every tree the record would emit: all wh answers, the one proved yes/no answer tree, or zero for `not_proved` and `answers([])`. A total above the cap deterministically rejects at stage `run`, class `resource`, exit 1, with zero stdout and detail `certificate_nodes_exceed_cap(1000000)`. A replay failure remains an internal invariant break and surfaces as stage `run`, class `uncaught`, exit 2. Only after the preflight, construction, and replay succeed is the result stream canonically serialized, reparsed, validated as an answer-v2 fixed point, and committed once to stdout.

For a wh goal, answer v2 uses this documented form:

```prolog
cnl_answer_record(2).
document(docid('<docid>'),source_sha256('<hex>'),ulex(<ulex-term>)).
program(sha256('<hex>')).
answer(
    query_id(sentence(S),clause(C)),
    wh(who),
    pred(Name,[var(1)]),
    answers([GroundAtom,...])
).
proof(GroundAtom,ClauseId,[SubProof,...]).
...
```

`answers/1` lists all derived ground instances of the goal pattern. The list is sorted ascending by the canonical serialization bytes of each ground query atom, a project-owned order independent of evaluation schedule. Zero matches are represented explicitly as `answers([])`. Each listed atom has its own first-witness proof tree, and those top-level proof terms follow the answer term in the same order. The constant universe is exactly the `named/1` constants present anywhere in the program record; wh execution invents no constants.

Answer-record consumers must require exactly `cnl_answer_record(2)` and reject every other answer envelope version. Answer records are terminal in-tree artifacts, so the current CLI produces and self-checks them rather than accepting them as another stage's input. For wh output, the reparsed self-check requires the exact `wh(who)` marker and `pred(Name,[var(1)])` pattern, a proper canonically byte-sorted `answers/1` list of ground unary `pred/2` atoms with `named/1` arguments, and one ordered top-level proof per answer with an identical root atom; `answers([])` requires zero proofs.

`run` validates the program artifact independently rather than trusting the compiler. After the shared framing gates (`input_utf8`, `syntax`, `canonical`), program v2 content rejection is owned by `envelope`, `query_count`, `section_order`, `shape`, `identity`, `ordering`, `scope`, `safety`, and `cycle`. The post-model certificate preflight can additionally reject `resource`. Program-side `naf` is no longer a rejection class. Failure emits no result prefix.

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

All four commands are implemented: `lower | validate | compile | run`. `lower` accepts one canonical M2 DRS record, `validate` accepts one canonical IR v1 record, `compile` accepts one canonical IR v1 record, and `run` accepts one canonical program v2 record. Missing arguments, an unknown command, or extra arguments are `usage` errors.

The tool pins encoding, double-quote, back-quote, character-escape, syntax-error, and writer behavior; it does not depend on ambient SWI defaults. `-f none -F none` remains part of every canonical process invocation.

I/O contract:

- Validate success: exit 0, zero stdout, zero stderr.
- Lower success: exit 0, one canonical IR v1 record on stdout, zero stderr.
- Compile success: exit 0, one canonical program v2 record on stdout, zero stderr.
- Run success: exit 0, one canonical answer v2 record on stdout, zero stderr.
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

The independently versioned envelopes are `cnl_ir_record(1)`, `cnl_program_record(2)`, and `cnl_answer_record(2)`. Any change to a record's constructors, arities, admitted argument forms, section cardinality or order, identity/provenance rules, canonical bytes, or logical semantics requires a new version of that envelope. Each reader rejects every other version of the envelope it owns. In particular, `run` rejects program v1 records as `envelope`; answer consumers reject non-v2 answer records. No defined version has an ignored extension field.

## Deferred and staged work

| Area | Current treatment |
|---|---|
| Intervals and rationals | No constructors; reject. |
| Temporal and dose algebra | No constructors; reject. |
| Direction, strength, and certainty | No annotations; reject. |
| Explicit negation and a false outcome | Absent; `not_proved` remains unknown. |
| NAF execution and ACE lowering | NAF executes in program v2 rule bodies; ACE-to-IR lowering and IR support remain deferred to M4.2. |
| Recursion and tabling | Any signed predicate cycle, including a positive-only cycle, is rejected. |
| Conflict detection | No rule-pair conflict analysis. |
| Proof enumeration and DAG sharing | First witness only; certificates remain trees. |
| Proof and answer resource preflight | **Shipped:** `certificate_node_cap(1000000)` bounds the total expanded nodes across all trees emitted by one record. |
| Prose rendering | Outside these records. |
| Program-digest binding inside answer records | **Shipped:** answer v2 line 3 binds the exact raw program bytes with SHA-256. |
| Wh answers | **Shipped:** program v2 executes exact `wh(who)` goals and emits canonically byte-sorted `answers/1` with ordered first-witness proofs or explicit `answers([])`. |
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

## Wh answer grounding example

The committed `tests/fixtures/run/green/wh-multi-order.program.pl` fixture inserts `named(a)` before `named('z z')`, then derives both `recover/1` atoms:

```prolog
cnl_program_record(2).
document(docid('wh-multi-order'),source_sha256('4444444444444444444444444444444444444444444444444444444444444444'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named(a)]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(patient,[named('z z')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)])])).
goal(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)])).
```

The exact fixture-backed answer bytes are:

```prolog
cnl_answer_record(2).
document(docid('wh-multi-order'),source_sha256('4444444444444444444444444444444444444444444444444444444444444444'),ulex(none)).
program(sha256('b8aabde529e4a544979f104e7e65df4d19feb899651c32981f3a4d75ae2adadd')).
answer(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),answers([pred(recover,[named('z z')]),pred(recover,[named(a)])])).
proof(pred(recover,[named('z z')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('z z')]),fact_id(sentence(2),clause(1)),[])]).
proof(pred(recover,[named(a)]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named(a)]),fact_id(sentence(1),clause(1)),[])]).
```

This order differs both from store insertion order and from SWI standard term order. At the first differing byte inside the canonical atom serialization, quoted `'z z'` begins with `0x27` while unquoted `a` begins with `0x61`, so the project-owned canonical-byte order places `'z z'` first.
