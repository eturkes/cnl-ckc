# CNL IR and execution records

Status: normative for the project-owned ACE-to-Prolog boundary and the run-side execution artifacts.

This document defines IR v3, program record v3, and answer record v3. IR v3 is one function-free Datalog record per ACE document. It preserves document identity, sentence/clause identity, token provenance, deterministic DNF branch identity, labeled closed-world exceptions, and action-choice metadata. Native Prolog variables and executable Prolog syntax remain outside the artifact language: rule variables and the wh query variable are explicit `var(N)` data. Program v3 executes ordinary clauses plus bare and labeled NAF; `alternative_set` rows remain non-derivational metadata. Answer v3 binds the exact program bytes, carries labeled or bare absence leaves, and copies action-choice metadata.

## IR v3 record grammar

A record is a UTF-8 term stream in this exact section order:

```text
cnl_ir_record(3).
<document>
<fact>*
<closed_world>*
<rule>*
<alternative_set>*
<query>
```

Every section may be empty except the document and query sections. Exactly one query is required and is the final term. Every term occupies one LF-terminated line.

The complete constructors are:

```prolog
document(docid('<docid>'),source_sha256('<hex>'),ulex(none)).
document(docid('<docid>'),source_sha256('<hex>'),ulex(sha256('<hex>'))).

fact(
    fact_id(sentence(S),clause(C)),
    pred(Name,GroundArgs),
    source(sentence(S),tokens([T1,...]))
).

closed_world(
    exception_id(rule(RuleId),literal(L)),
    affects(RuleId),
    predicate_key(Name,arity(A))
).

rule(
    rule_id(sentence(S),clause(C)),
    pred(Name,RuleArgs),
    body([BodyLiteral,...]),
    source(sentence(S),tokens([T1,...]))
).
rule(
    rule_id(sentence(S),clause(C),branch(B)),
    pred(Name,RuleArgs),
    body([BodyLiteral,...]),
    source(sentence(S),tokens([T1,...]))
).

alternative_set(
    alternative_set_id(sentence(S),clause(C)),
    members([Member1,Member2,...]),
    body([PositiveLiteral,...]),
    satisfaction(any_member),
    exclusivity(not_asserted),
    exhaustiveness(not_asserted),
    source(sentence(S),tokens([T1,...]))
).
alternative_set(
    alternative_set_id(sentence(S),clause(C),branch(B)),
    members([Member1,Member2,...]),
    body([PositiveLiteral,...]),
    satisfaction(any_member),
    exclusivity(not_asserted),
    exhaustiveness(not_asserted),
    source(sentence(S),tokens([T1,...]))
).

query(
    query_id(sentence(S),clause(C)),
    pred(Name,GroundArgs),
    source(sentence(S),tokens([T1,...]))
).
query(
    query_id(sentence(S),clause(C)),
    wh(who),
    pred(Name,[var(1)]),
    source(sentence(S),tokens([T1,...]))
).

GroundArg ::= named(Atom)
RuleArg ::= named(Atom) | var(N)
GroundArgs ::= [GroundArg_1,...,GroundArg_n], n >= 1
RuleArgs ::= [RuleArg_1,...,RuleArg_n], n >= 1
PositiveLiteral ::= pred(Name,RuleArgs)
BodyLiteral ::= PositiveLiteral
              | naf(pred(Name,RuleArgs))
              | naf(ExceptionId,pred(Name,RuleArgs))
Member ::= pred(Name,RuleArgs)
ExceptionId ::= exception_id(rule(RuleId),literal(L))
RuleId ::= rule_id(sentence(S),clause(C))
         | rule_id(sentence(S),clause(C),branch(B))
```

The layout above is explanatory. Accepted bytes use the canonical single-line forms defined below.

Constraints:

- `S`, `C`, `B`, `L`, `A`, `N`, and every `T` are integers at least 1 wherever present.
- `Name` and each `named(Atom)` payload are open atoms.
- Every ordinary predicate has a proper non-empty argument list, with no fixed upper bound. Predicate identity is `Name/Arity`; equal names at different arities are distinct.
- `var(N)` is data, not a native Prolog variable. Native variables are invalid.
- The sole sort is `entity`. V3 has no sort annotation constructor; every argument has that sort.
- Floats, strings, rationals, lists used as argument values, and every other atomic or compound argument form are invalid.
- A rule body is proper and non-empty. Its accepted literal forms are positive `pred/2`, bare `naf/1`, and labeled `naf/2` as shown above.
- `naf/1` and `naf/2` are admitted only as rule-body literals around one well-shaped `pred/2`. They are invalid in facts, rule heads, queries, predicate arguments, alternative members/bodies, and every other position.
- An `alternative_set` has at least two positive members, and no two member terms are exactly identical (`==`). Its body is a proper list of positive predicates and may be empty. The three policy fields are fixed exactly to `any_member`, `not_asserted`, and `not_asserted`; no alternate policy value is admitted.
- A wh query is exactly `query(query_id(...),wh(who),pred(Name,[var(1)]),source(...))`. No other marker, pattern arity, or pattern argument is admitted.
- Unknown constructors, arities, fields, or literal forms are hard errors. No pass drops or approximates unsupported content.

The second line is the `document/3` line copied byte-for-byte from the corresponding `ace_front_end_record(1)` front-end record. `<docid>` is non-empty, contains only `[a-z0-9-]`, and does not begin with `-`. `<hex>` is exactly 64 lowercase hexadecimal characters. Identity atoms remain single-quoted exactly as the front end emits them.

## IDs and provenance

Item IDs are kind-specific:

- fact: `fact_id(sentence(S),clause(C))`
- ordinary rule: `rule_id(sentence(S),clause(C))`
- expanded rule branch: `rule_id(sentence(S),clause(C),branch(B))`
- ordinary alternative set: `alternative_set_id(sentence(S),clause(C))`
- expanded alternative-set branch: `alternative_set_id(sentence(S),clause(C),branch(B))`
- query: `query_id(sentence(S),clause(C))`
- labeled exception: `exception_id(rule(RuleId),literal(L))`

The item-ID sentence must equal the item's `source/2` sentence. An unbranched origin `(S,C)` is globally unique across facts, rules, alternative sets, and the query. A branched origin may be shared only by one contiguous group of rules or one contiguous group of alternative sets. Such a group starts at `branch(1)`, is dense and strictly ascending, and contains at least two branches. A singleton retains the legacy two-argument ID. Every rule branch in one origin has an exactly identical head predicate. Every alternative-set branch in one origin has exactly identical members and policy fields; only its antecedent-derived body and source may differ. Divergent same-origin consequents are audit tampering and reject before model construction. Fact and query IDs never admit a branch field.

Within each fact, rule, alternative-set, and query section, item order is strictly ascending by `(S,C,B)`, treating an unbranched item as `B = 0`. Generated siblings share one source clause number: expansion does not consume extra clause ordinals, and the next source item receives the next ordinary clause number.

A closed-world row has no independent source span. Its exception ID embeds the exact affected final rule ID, including any branch component, and `literal(L)` is the labeled literal's one-based position in that rule body. Declaration rows are strictly ordered by affected rule `(S,C,B)` and then literal ordinal. Every affected rule must exist; every declared predicate key must be defined by a fact or rule head in the same record; every declaration must be used by the exact labeled literal; and every labeled literal must have one exact matching declaration. Exception IDs are unique.

`source(sentence(S),tokens([T1,...]))` contains a non-empty, strictly ascending list of positive token ordinals. Ordinals refer to the front-end record identified by the same document line: source SHA-256 plus sentence ordinal gives sentence identity, and the token ordinal is local to that sentence. A DNF branch carries only the anchors selected for that branch plus shared antecedent/head anchors; sibling-only anchors never leak across branch provenance. Alternative-set source covers its members and body under the lowering laws below.

## IR v3 variables and safety

A rule's `var(N)` values are local to that rule. Numbering is dense `1..k` in first-occurrence order while scanning the serialized rule left-to-right: head arguments first, then body literals and their arguments in list order. Bare and labeled NAF arguments participate in the same scan. Repeated occurrences retain their first number.

Canonical rule-body order is all positive literals followed by all NAF literals, preserving source order within each block. A positive literal after either NAF form is invalid. Every variable in a bare or labeled NAF literal must occur in a positive literal of the same rule. Every head variable must likewise occur in a positive body literal; NAF provides no coverage. Body-only variables remain admitted.

An alternative set has its own local numbering scan: members in list order first, then body predicates in list order. Every member variable must occur in its positive body. Its body admits no NAF. Ground root alternative sets therefore use no variables and may have `body([])`.

Facts and yes/no queries are ground. The wh query pattern's sole variable is admitted only in the exact data form `var(1)` described above.

## IR v3 semantics

Ignoring provenance and IDs, compilation is a total map to program v3. Facts become empty-body clauses; rules preserve their bodies; closed-world declarations pass through unchanged; alternative sets lose only `source/2`; a yes/no query becomes `goal/2`; and a wh query becomes the exact `goal/3` form.

When no NAF literal is present, ordinary clauses denote a finite positive, function-free Datalog program whose meaning is its least Herbrand model over the `named/1` constants present in the record. With NAF present, IR v3 inherits program v3's standard stratified-model semantics, including per-stratum least fixpoints with lower strata frozen, ground absence tests, and the documented coincidence with the unique perfect, stable, and total well-founded models on this finite cycle-free profile.

The signed predicate dependency graph has a positive edge from each rule-head key to each positive body key and a negative edge to each bare or labeled NAF target key. Any directed cycle over the combined graph, including a positive-only cycle, a mixed-polarity cycle, or a self-loop, is rejected. This full cycle prohibition is stronger than stratification and remains a sufficiency-only restriction. Closed-world and alternative-set metadata create no dependency edges and are never inserted into the clinical atom store.

A yes/no query is `proved` exactly when its ground atom belongs to the completed model; otherwise its outcome is `not_proved`. A wh query enumerates all completed-model instances as specified under answer record v3. `not_proved` is unknown, never false. V3 has no `false` outcome and no explicit-negation constructor.

### Bare and labeled negation as failure

`naf(pred(Name,Args))` is legacy bare NAF. `naf(ExceptionId,pred(Name,Args))` is labeled NAF. Both execute the same ground absence test; neither means classical falsity. The validator requires positive coverage for every NAF variable, and the kernel asserts the substituted atom is ground before testing its absence.

A labeled NAF literal additionally requires an explicit `closed_world/3` declaration. The declaration asserts that the exact `Name/Arity` target is treated as complete for the exact affected rule and body position. It is policy metadata, not a derivable atom. Its label survives compilation, successful witness construction, answer serialization, and replay. Replay requires exact label identity, exact substituted target identity, and absence from the completed model.

Lowering preserves the legacy bytes for an ACE NAF target with no fact or rule definition in the same record: it emits bare `naf/1`. When the target key is defined by any fact or rule head, lowering emits labeled `naf/2` and the matching declaration row. Both forms coexist in v3. Generic IR/program validation admits bare NAF without a declaration even when the target key is defined in-record; labeled NAF always requires its exact declaration correspondence.

ACE `~/1` lowering remains restricted to the exact antecedent profiles documented below. ACE classical `-/1` remains rejected.

### Action-alternative metadata

`alternative_set` represents an action choice without rule-head disjunction. Its canonical member list contains at least two pairwise `==`-distinct positive predicates in source order. For every body grounding, `satisfaction(any_member)` states that any listed member is an acceptable satisfying action. `exclusivity(not_asserted)` does not claim that only one member may hold; `exhaustiveness(not_asserted)` does not claim that the list exhausts all possible actions.

The row is metadata-only in v3. It does not derive its members, does not derive a separate satisfaction atom, does not participate in saturation, and creates no ASP or multiple-model choice point. Compilation preserves the row as program data, and result assembly copies it after the program digest so downstream consumers can inspect the action choice while ordinary query truth remains determined solely by facts and rules.

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

The shared IR/program/answer serializer has exactly two shape-guarded forced-quote exceptions. Term 2 is forced only after proving the complete `document(docid(Atom),source_sha256(Atom),ulex(none|sha256(Atom)))` shape, preserving the byte-verbatim front-end identity line. In an answer record, term 3 is forced only after proving the complete `program(sha256(Atom))` shape, so its digest atom always remains single-quoted. A malformed document or digest lookalike falls through to `canonical_line/2`; in particular, malformed digest terms do not enter the forced-quote branch. The complete-shape proof keeps program and IR term 3, whether clause, fact, declaration, or metadata, on the generic serializer path.

Validation reserializes the parsed term stream with this record serializer and requires exact equality with the decoded input. Strict UTF-8 makes text equality equivalent to byte equality. Therefore comments, blank lines, CRLF, a missing final LF, extra spaces, alternate operator/list notation, and alternate atom quoting fail the fixed-point gate. Every accepted record ends in exactly one LF.

Canonical serialization is performed on copies because `canonical_line/2` numbers native variables destructively. A native variable can survive the byte gate only in its canonical variable spelling; shape validation then rejects it. SWI strings and rationals lie outside `canonical_line/2` and therefore fail as `canonical` before shape. Floats are serializable in every field position, including all term-2 document fields, but no defined record field shape admits them, so they fail `shape`.

## Validator

`src/prolog/ir_validate.pl` owns IR semantic passes 4-10. `src/prolog/inference_kernel.pl` owns the isomorphic program-record passes and the inference schedule. `src/prolog/validation_common.pl` owns shared branch-wrapper shape details and exact alternative-member distinctness. `src/prolog/drs_to_ir.pl` owns the `ace_front_end_record(1)` envelope and DRS-to-IR semantics; `src/prolog/ir_to_prolog.pl` owns compilation; `src/prolog/explanation.pl` owns certificate construction and replay. `src/prolog/ir_tool.pl` owns byte decoding, parsing, the canonical fixed point, CLI framing, buffering, and error emission. The first failing pass wins. Within a pass, the first offending term in stream order wins; deterministic within-term checks use the order stated below.

1. **Stream/UTF-8** - read stdin as bytes and apply the strict decoder.
2. **Term syntax** - parse a sequence of Prolog terms with pinned reader flags and syntax errors promoted to exceptions.
3. **Canonical fixed point** - serialize every parsed term and compare the complete text.
4. **Envelope/sections** - IR requires `cnl_ir_record(3)`, the document line, exactly one final query across admitted `query/3` and `query/4` arities, and sections in exact order: facts, closed-world declarations, rules, alternative sets, query. Query count is checked before trailing-term and section-interleave checks. IR v1, IR v2, and every other non-v3 version are `envelope` errors.
5. **Shape** - require the exact constructors, arities, proper lists, non-empty predicate arguments and token lists, both NAF forms only in rule bodies, exact closed-world wrappers, exact alternative-set policy fields, and exact wh marker/pattern. `body([])` deliberately survives this pass for the IR-specific safety error.
6. **Identity** - validate document fields; ID kind; positive sentence, clause, branch, source, token, literal, and arity ordinals where owned by the item; ID/source sentence equality; and branch fields only on rules and alternative sets.
7. **Ordering/uniqueness** - check complete ID uniqueness, source-origin ownership, per-section `(S,C,B)` order, dense multi-item branch groups beginning at 1, exact same-origin consequent payload coherence, and strict token order. Closed-world declaration ordering is checked in pass 9b because its key includes the embedded affected rule and literal position.
8. **Scope** - keep facts and yes/no queries ground, admit the wh pattern variable only as exact `var(1)`, enforce dense rule-local numbering across head plus body, and enforce dense alternative-local numbering across members plus body.
9. **Safety/NAF** - for each rule, check in this exact order: a positive literal after either NAF form, an empty body, the first NAF variable absent from all positive literals, then the first head variable absent from all positive literals. For each alternative set, reject its first NAF body literal, then the first member variable absent from the positive body.
9b. **Exceptions** - validate declaration affected-rule identity, positive literal/arity ordinals, existing affected rule, in-record target definition, exception-ID uniqueness, declaration order, exact label position and target correspondence, and the absence of unused declarations.
10. **Cycles** - scan positive and bare/labeled-negative body edges in rule/body stream order and reject the first edge that closes any directed cycle.

Program validation uses the same pass numbers, class vocabulary, first-failure rule, and stream-order discipline. Its envelope is `cnl_program_record(3)`, `document/3`, fact clauses, closed-world rows, rule clauses, source-free alternative sets, and one final goal across admitted `goal/2` and `goal/3` arities. Shape, scope, safety, exception, branch-group, and signed-cycle rules mirror IR v3. The deliberate identity asymmetry is clause ownership: in a program record, `fact_id` if and only if the body is `[]`, and `rule_id` if and only if the body is non-empty. Therefore program `rule_id` plus `body([])` fails pass 6 as `identity`, while IR retains distinct `fact/3` and `rule/4` constructors and reports `rule(...,body([]),...)` at pass 9 as `safety`.

### Error classes

| Class | Pass/stage | Meaning | Exit |
|---|---|---|---:|
| `input_utf8` | Framing 1 | Stdin is not strict RFC 3629 UTF-8. | 1 |
| `syntax` | Framing 2 | The decoded term stream cannot be parsed. A leading UTF-8 BOM reaches this class under pinned SWI 9.2.9. | 1 |
| `canonical` | Framing 3 | Reserialized text differs, or a term is outside the canonical serializer's domain. | 1 |
| `envelope` | Validate/program 4 / lower | The selected record envelope is missing, malformed, wrong-versioned, or has trailing content. IR readers require v3 and reject every other version, including v1 and v2. | 1 |
| `query_count` | Validate/program 4 | An IR record has zero or multiple final queries across admitted `query/3` and `query/4` arities, or a program v3 record has zero or multiple final goals across admitted `goal/2` and `goal/3` arities. | 1 |
| `section_order` | Validate/program 4 | A fact, declaration, rule, or alternative-set row appears after a later section has begun. | 1 |
| `shape` | Validate/program 5 | A constructor, arity, list, admitted atomic kind, NAF position, closed-world wrapper, alternative policy, or exact wh marker/pattern shape is invalid. | 1 |
| `identity` | Validate/program 6 | Document identity, ID kind, body-kind agreement, branch admission, ordinal bound, or IR ID/source sentence agreement is invalid. Program v3 owns clause kind by body shape; IR retains distinct item constructors. | 1 |
| `ordering` | Validate/program 7 | An ID/origin is duplicated, a section ID is out of `(S,C,B)` order, a branch group is non-dense or singleton, same-origin branch consequents differ, or IR token ordinals are not strictly ascending. | 1 |
| `scope` | Validate/program 8 | A variable occurs outside its admitted position, or rule/alternative numbering is not dense first-occurrence order. The exact wh query variable `var(1)` is admitted. | 1 |
| `safety` | Validate/program 9 | A rule violates positives-then-NAF order, an NAF/head variable lacks positive-body coverage, an IR rule body is empty, an alternative body contains NAF, or an alternative member variable lacks body coverage. | 1 |
| `exception` | Validate/program 9b | A closed-world declaration or labeled NAF has a bad affected rule, target, ID/order, body position, declaration correspondence, or use cardinality. | 1 |
| `cycle` | Validate/program 10 / lower | A positive or bare/labeled-negative dependency edge closes any cycle in the signed predicate graph; self-loops count. Lowering surfaces predictable generated-IR rejection as input content, never `uncaught`. | 1 |
| `disjunction` | Lower | Antecedent `v/2` is malformed, contains NAF in a disjunct, occurs beneath NAF, appears in a question, or expands beyond the per-rule 64-branch cap. | 1 |
| `alternative_set` | Lower | Root/consequent `v/2` fails the admitted two-action profile, contains a non-ground root argument, duplicates a lowered member, violates consequent-domain constraints, or would give metadata an NAF body. | 1 |
| `question_count` | Lower | The root DRS has zero or multiple questions, or its sole question is not final. | 1 |
| `negation` | Lower | Classical `-/1`, non-antecedent or nested `~/1`, malformed recognized antecedent-`~/1` content other than an earlier cross-DRS redeclaration, or positive-after-NAF antecedent order is invalid. A `~/1` sub-DRS redeclaring an already-declared referent fails earlier as `referent`; `v/2`-specific walls use `disjunction`. | 1 |
| `wh_query` | Lower | A `query/2` condition is outside the exact admitted anchored `who` plus intransitive-predicate arrangement, including non-`who`, copular, restricted, multiple, unanchored, extra-condition, or unconsumed-query-variable forms. | 1 |
| `resource` | Run | The total expanded nodes across all proof trees the answer record would emit exceeds `certificate_node_cap(1000000)`. | 1 |
| `copula` | Lower | A factual `object/6` and `be` pair is malformed, ambiguous, or unpaired. Root object handling precedes a later relation, so an unpaired object retains this class. | 1 |
| `referent` | Lower | A DRS referent is undeclared, redeclared, unconsumed, role-reused, unbound in a rule head, unresolved/ambiguous as a root relation argument or property carrier, or not losslessly erasable as an event. A transitive or property-copula event with any occurrence beyond its own event slot in its owning condition tree is `event_in_use`. | 1 |
| `unsupported` | Lower | A constructor or arrangement is outside the admitted DRS profile, including unsupported property degrees/arities, a non-`of` relation name, an invalid n-ary argument shape, or a consumed DRS lemma containing U+0020; provenance is not one-sentence canonical data; or lowering cannot produce valid v3 IR without loss. | 1 |
| `usage` | CLI | Arguments do not select exactly one implemented command (`lower`, `validate`, `compile`, or `run`). | 2 |
| `uncaught` | Any | An unexpected internal exception, including certificate replay failure or an unexpected generated-record validation/serialization failure, escaped a stage. | 2 |

The stage atom is one of `cli`, `validate`, `lower`, `compile`, or `run`. The framing classes (`input_utf8`, `syntax`, `canonical`) plus the validation classes above are together the complete tamper-rejection surface for program artifacts read by `run`; compilation surfaces IR validation failures at stage `compile`.

## DRS lowering

Canonical lowering invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- lower <record.drs.pl >record.ir.pl
```

Input is exactly one canonical `ace_front_end_record(1)` record with three terms and no trailing content:

```prolog
ace_front_end_record(1).
document(docid('<docid>'),source_sha256('<hex>'),ulex(<ulex-term>)).
drs(Domain,Conditions).
```

The input passes the same strict UTF-8 decoder, pinned term parser, and canonical fixed-point gate as IR validation. Term 2 uses the same forced-quote document serializer. Consequently an admitted `document/3` line is copied byte-for-byte into output term 2. Lowering is buffered: no real stdout bytes are written until the complete output has passed IR v3 validation.

The admitted DRS profile is deliberately total and small:

- Root factual conditions are processed in DRS order. An `object(X,Class,countable,na,eq,1)` root is handled as a copula candidate before property, ordinary-predicate, and relation dispatch: paired with `predicate(E,be,named(Name),X)`, it becomes `pred(Class,[named(Name)])`; both anchors contribute provenance. Every other factual `object` or `be` arrangement rejects as `copula`. This preserves the normative unpaired-object-first precedence even when a later property or relation mentions `X`.
- An anchored positive property is either exact `property(X,Adj,pos)` or exact `property(X,Adj,Degree,Other)` with `Degree == comp_than` or `Degree == pos_as`. At a root, `X` must be a declared carrier resolved by exactly one `predicate(E,be,named(Name),X)` and becomes `named(Name)`; missing or ambiguous bindings are `referent`. The copula event is erased under the single-occurrence law. A predicative property emits unary `pred(Adj,[named(Name)])`. An object-attached attributive property emits its own unary fact after the object's class fact because root emission preserves DRS condition order; multiple attached properties emit one fact each and may reuse the already-consumed naming copula without re-consuming its event.
- A comparative property emits `pred(Name2,[named(Name),Other'])`. `Name2` is formed exactly by `atom_concat(Adj,' ',Prefix), atom_concat(Prefix,Degree,Name2)`: adjective lemma, one U+0020 SPACE, then the exact APE degree atom. Thus `helpful comp_than` and `helpful pos_as` remain distinct predicate names. The comparison argument must be ground under the root profile.
- `predicate(E,Verb,named(Name))` becomes `pred(Verb,[named(Name)])`. A root fact may not retain a discourse referent as its subject.
- `predicate(E,Verb,A1,A2)` with `Verb \== be` becomes `pred(Verb,[A1',A2'])`, preserving subject/object order. Root and yes/no-query arguments must be `named/1`; rule arguments may be named terms or declared rule referents. `E` must be a local event referent with exactly one occurrence in its owning DRS condition tree: its event position in this predicate. Any alias or other use, including a modifier or relation argument, rejects as `referent` with detail `event_in_use`; lowering never drops an observably used event. Exact committed reds pin this law independently at root-fact, rule-antecedent, rule-consequent, and question call sites.
- `relation(R1,of,R2)` becomes the ordinary binary predicate `pred(of,[R1',R2'])`, preserving DRS argument order. `of/2` uses the existing predicate constructor rather than adding a relation-specific IR node, minimizing grammar and trusted-spec surface. At a root, `named/1` arguments pass through; a variable argument must be a declared object referent resolved by exactly one `predicate(E,be,named(Name),R)` copula and becomes `named(Name)`. Its relation source includes the relation anchor plus each copula anchor used for resolution. Missing or ambiguous root bindings reject as `referent`; every relation name other than exact atom `of` rejects as `unsupported`.
- A root `v(Left,Right)` is action-choice metadata, not a factual disjunction. Each operand must be a DRS with exactly one anchored positive transitive `predicate/4`, a local erasable event, a non-`be` verb, and two ground `named/1` semantic arguments. Lowering emits one ground `alternative_set` with `body([])`, two distinct members in left-then-right order, and the fixed policy fields. Other root disjunction profiles reject as `alternative_set`; they never become facts or rules.
- Reserved-separator law: every consumed DRS lemma atom—predicate verb, object class, property adjective, or relation name—must contain no U+0020 SPACE. The first such lemma at its normal DRS dispatch position rejects as `unsupported` with deterministic detail `lemma_space(verb|object_class|adjective|relation)`. This makes comparative name encoding injective by construction; no pass splits or trims a lemma.
- A rule is exactly `=>(drs(ADom,AConds),drs(CDom,CConds))`. An ordinary consequent denotes exactly one positive head. A one-condition consequent admits `predicate/3`, transitive `predicate/4`, or exact `relation(R1,of,R2)`. A two-condition copular consequent admits exactly one carrier followed by one matching `be`: object plus `be` becomes a unary noun head, positive property plus `be` becomes a unary adjective head, and comparative property plus `be` becomes its binary degree-named head. Carrier and event referents are consequent-local and erased; semantic arguments must be named or antecedent-bound. A three-condition object+property+`be` attributive consequent still rejects under the single-head law.
- An action-choice consequent is exactly `CDom == []` and `CConds == [v(Left,Right)]`. Each operand is a DRS with a proper local domain and exactly one anchored positive transitive `predicate/4`; the verb is not `be`, the local event is erasable, and both semantic arguments are named or antecedent-bound. Lowering emits one `alternative_set` whose two members preserve left-then-right source order and must be distinct. It never emits member rules or a disjunctive head. Any outer consequent domain, nested/member profile, duplicate lowered member, or additional consequent condition rejects as `alternative_set` or under the existing single-head wall.
- Antecedent `v/2` is structural DNF. Each operand must be a `drs/2` with proper lists and at least one condition; NAF is forbidden anywhere inside either operand, and `v/2` beneath `~/1` is forbidden. Nested disjunctions otherwise recurse. Conjunction computes a Cartesian product; disjunction concatenates all left branches before all right branches. Earlier conditions are the outer product dimension, so `(A or B), (C or D)` enumerates `A,C`; `A,D`; `B,C`; `B,D`, while `A or (B and C)` has exactly two branches. Structurally equal source branches remain distinct and auditable.
- DNF expansion is bounded per implication, not per record. `dnf_branch_cap(64)` counts final expanded branches with saturating addition/multiplication before materialization: exactly 64 is accepted; 65 or more rejects fail-closed as `ir_tool_error(lower,disjunction,rule(Position,antecedent_branch_cap_exceeded(64))).` No partial record reaches stdout. Multiple rules may each independently reach the cap.
- The outer antecedent domain is present in every branch; each selected disjunct domain is appended only to its descendants. Scope, referent accounting, variable numbering, safety, and provenance are then checked independently per final branch. Shared antecedent/head anchors appear in every branch source; only selected disjunct anchors appear in that branch. A shared suffix `~/1` is duplicated into every branch, while NAF inside a disjunct rejects.
- One final branch retains the legacy `rule_id(sentence(S),clause(C))` or `alternative_set_id(sentence(S),clause(C))`. Two or more final branches share one source clause origin and receive deterministic `branch(1)..branch(N)` IDs in DNF order. Antecedent expansion copies one unchanged consequent: ordinary branch heads are structurally identical, and alternative-set branches retain identical members and policy fields. Expansion consumes one clause counter regardless of branch count. Alternative-set member count does not multiply DNF branch count: a two-branch antecedent plus a two-member action choice emits two branched alternative-set rows. Because alternative bodies are positive metadata contexts, any otherwise valid antecedent NAF with an action-choice consequent rejects explicitly as `alternative_set`, never via the generated-record backstop.
- A positive antecedent condition is an admitted exact `object/6`, intransitive `predicate/3`, transitive `predicate/4`, `relation(R1,of,R2)`, or positive property. A direct property over an antecedent referent becomes a unary or comparative body literal over that referent. When a property carrier has one matching later `predicate(E,be,Subject,Carrier)`, lowering instead uses `Subject` as the literal's first argument, consumes the carrier and the single-use event, and includes both property and `be` anchors. This is the APE attributive/predicative carrier normalization; multiple properties sharing one carrier each emit a literal while the event is erased once. Binary arguments are scanned left-to-right, and condition order is retained after erased carrier `be` conditions are removed.
- An NAF antecedent condition is an unanchored `~/1` around exactly one of the following sub-DRS profiles. The outer entity `X` must already occur in a positive condition of the enclosing antecedent. The nested domain declares exactly the local referents consumed by the profile.

```prolog
~(drs([E],[-(predicate(E,Verb,X),/(S,T))]))
```

This becomes `naf(pred(Verb,[var(N)]))`.

```prolog
~(drs([D,E],[
    -(object(D,Class,countable,na,eq,1),/(S,T1)),
    -(predicate(E,be,X,D),/(S,T2))
]))
```

This becomes `naf(pred(Class,[var(N)]))`. The copula orientation is the same normalization law as the factual form: outer antecedent entity `X` is the subject and local `D` is the copular object. NAF remains limited to these two unary profiles; properties, transitive predicates, and `relation/3` inside `~/1` reject as `negation`.

- Lowering first produces the legacy bare literal for either profile, then classifies its exact `Name/Arity` target against all fact and rule heads in the generated record. An undefined target remains bare and preserves the legacy bare-NAF bytes. A defined target becomes `naf(ExceptionId,Predicate)`, where `ExceptionId = exception_id(rule(FinalRuleId),literal(BodyPosition))`, and lowering emits the matching `closed_world(ExceptionId,affects(FinalRuleId),predicate_key(Name,arity(Arity)))` row. Body position is one-based across positive and NAF literals. A shared NAF suffix on expanded antecedents receives a distinct label and declaration for every final branch rule. Definitions supplied by other expanded branches count by exact predicate key.

- All positive antecedent conditions must precede every `~/1` condition. Lowering preserves source order and rejects a later positive condition with class `negation`; it never silently regroups conditions.
- Exactly one `question(drs(QDom,QConds))` must be the final root condition. A yes/no question is one anchored ground intransitive `predicate/3`, transitive `predicate/4`, or direct exact `relation(named(_),of,named(_))`; or one exact two-condition ground copula. The copular forms mirror rule heads: object+`be` becomes a noun predicate, positive property+`be` becomes a unary adjective predicate, and comparative property+`be` becomes the binary degree-named predicate. The carrier and event must be question-local, the `be` subject must be `named/1`, and all comparison arguments must be ground. Other multi-condition copular questions remain outside this profile.
- The admitted wh question is exactly two anchored conditions in this order: `query(A,who)` followed by one intransitive `predicate(B,Verb,A)`. `QDom` is exactly `[A,B]`; `B` is the erasable event and `A` becomes the IR pattern variable `var(1)`. Both anchors contribute provenance. This yields `query(query_id(...),wh(who),pred(Verb,[var(1)]),source(...))`. A property-based wh arrangement such as APE's query+property+`be` form remains `wh_query`-rejected.
- A predicate event referent is erased only when it is declared by that same DRS domain, used exactly once as an event argument, and never reused as an entity. Property and noun copula carriers are consumed as internal entities; their matching `be` events obey the same single-occurrence erasure law, and a shared root/antecedent property carrier never causes the event to be counted twice. Domain declarations are unique across the admitted DRS tree, including admitted NAF sub-DRSs. Every declared referent must be consumed by copula/property normalization, erased under this event law, bound as a rule variable, or used as the exact wh query referent.
- Rule variables are data terms `var(N)`. Numbering is dense in validator traversal order: head arguments first, then antecedent body literals in DRS order. NAF targets reuse the number established by their positive antecedent binding.
- An output item's sentence is the common `S` from every consumed `/(S,T)` anchor. Mixed-sentence items are rejected. Tokens are deduplicated and sorted strictly ascending. Clause IDs are per-sentence 1-based counters in root emission order. Facts precede rules, each emitted section has ascending `(S,C)` IDs, and the query remains final.

Classical `-/1` negation is never admitted. Root or consequent `~/1`, nested `~/1` and `-/1`, malformed recognized antecedent-`~/1` domains or contents, an unbound NAF entity, and positive-after-NAF antecedent order are `negation`, except that cross-DRS declaration uniqueness runs first. `v/2` uses position-specific handling: the admitted root/consequent profiles become alternative sets, antecedent `v/2` expands as DNF, question `v/2` and malformed/nested-NAF cases are `disjunction`, and invalid action-choice operands are `alternative_set`. The arity-two `-(Condition,/(S,T))` constructor remains the provenance wrapper and is never itself negation.

For `property/3`, exact degree `pos` is the sole admitted degree; APE-emitted `comp` and `sup` therefore reject as `unsupported` at the property's owning condition. `property/4` admits only exact `comp_than` and `pos_as`. Every other property degree and every property arity other than 3 or 4, including transitive-adjective `property/6`, is `unsupported`. An unpaired property question or consequent is likewise `unsupported`; a root comparative carrier with zero or multiple naming copulas is `referent`.

`wh_query` owns every `query/2` outside the exact admitted profile: non-`who` markers; the `which` restriction form; copular wh questions; multiple, unanchored, or extra conditions; a query outside the final question; or a query referent not consumed as the predicate subject. Other unknown DRS content remains `unsupported`. Lowering never drops a condition, silently weakens a term, or emits a partial record.

Lowering is first-failure deterministic. Its outer order is: front-end envelope; root-domain shape; root question count/position; cross-DRS declaration uniqueness; root factual and rule conditions in DRS order; root referent accounting; final-question semantics; output section/order checks; generated-IR validation. An implication owns its root position before anchored-condition dispatch. For an anchored root condition, dispatch is exactly object copula candidate, `be` copula candidate, property copula candidate, transitive `predicate/4`, intransitive `predicate/3`, `relation/3`, then generic unsupported content. The object handler therefore retains unpaired-object-first precedence over any later property or relation. At a property position, adjective separator and degree/arity checks precede carrier binding; a valid carrier then requires exactly one naming `be`, with zero/multiple matches classified `referent`. A transitive predicate proves its event has exactly one condition-tree occurrence before erasing it. Relation separator/name and root argument checks occur at that relation's position.

Within a rule, consequent shape establishes member/head numbering before antecedent lowering. An exact top-level consequent `v/2` claims action-choice normalization; otherwise the existing ordinary single-head profiles apply. DNF counting then validates each disjunct DRS/domain and the `v`/NAF walls with saturating arithmetic. A count above 64 rejects before branch materialization or body lowering. Accepted branches are expanded in the structural order above and independently run the positive-before-NAF check, condition lowering, scope accounting, head/member coverage, provenance assembly, and generated-IR backstop. A predictable generated branch safety/cycle rejection remains stage `lower`, exit 1, never `uncaught`.

A final question containing any `v/2` rejects as `disjunction` before yes/no dispatch. A question containing `query/2` enters the exact wh profile; property-based wh therefore remains `wh_query`. Otherwise an exact two-condition object/property carrier is normalized before the single-condition yes/no profiles. Consumed anchors follow the existing one-sentence, deduplicate, and numeric-sort law. If otherwise valid lowering produces a cyclic rule dependency, generated-IR validation returns `lower,cycle`, exit 1; any other unexpected generated-record rejection remains an internal invariant failure.

### ACE authoring wall for exceptions and alternatives

Pinned APE 6.7 accepts antecedent coordination only with the repeated relative marker, for example `Every patient that waits or that sleeps recovers.` It emits structural antecedent `v/2`. Root and consequent action coordination such as `John offers Mary or arranges Mary.` and `Every patient that waits offers Mary or arranges Mary.` emit the admitted action-choice `v/2` profiles.

Literal `unless` has no authorable route in the pinned grammar: `Every patient recovers unless the patient is a smoker.` fails in APE at `unless`. `Every patient recovers without smoking.` also fails; `without a treatment` may parse as `modifier_pp(...,without,...)`, which is not NAF and remains outside lowering. The implementation never infers exception semantics from ordinary `if`, an explicit negative outcome word, or a `without` modifier.

Use an explicit paired-rule pattern when a complete exception predicate is authorable:

```text
Every patient that coughs is a smoker.
Every patient that is not provably a smoker recovers.
```

The first rule defines `smoker/1`; the second produces the admitted unary `~/1` profile. Because the NAF target is defined in-record, lowering generates the labeled literal and its closed-world declaration. Omitting the defining rule leaves the existing bare-NAF form unchanged. This is the authoring route for a labeled exception; it does not claim that APE parsed `unless` or `without`.

## Program compilation and program record v3

Canonical compilation invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- compile <record.ir.pl >record.program.pl
```

A program v3 record is a ground term stream in this exact section order:

```text
cnl_program_record(3).
<document>
<fact-clause>*
<closed_world>*
<rule-clause>*
<alternative_set>*
<goal>
```

The complete constructors are:

```prolog
clause(
    fact_id(sentence(S),clause(C)),
    pred(Name,GroundArgs),
    body([])
).
closed_world(
    exception_id(rule(RuleId),literal(L)),
    affects(RuleId),
    predicate_key(Name,arity(A))
).
clause(
    RuleId,
    pred(Name,RuleArgs),
    body([ProgramBodyLiteral,...])
).
alternative_set(
    AlternativeSetId,
    members([Member1,Member2,...]),
    body([PositiveLiteral,...]),
    satisfaction(any_member),
    exclusivity(not_asserted),
    exhaustiveness(not_asserted)
).
goal(
    query_id(sentence(S),clause(C)),
    pred(Name,GroundArgs)
).
goal(
    query_id(sentence(S),clause(C)),
    wh(who),
    pred(Name,[var(1)])
).

RuleId ::= rule_id(sentence(S),clause(C))
         | rule_id(sentence(S),clause(C),branch(B))
AlternativeSetId ::= alternative_set_id(sentence(S),clause(C))
                   | alternative_set_id(sentence(S),clause(C),branch(B))
GroundArg ::= named(Atom)
RuleArg ::= named(Atom) | var(N)
GroundArgs ::= [GroundArg_1,...,GroundArg_n], n >= 1
RuleArgs ::= [RuleArg_1,...,RuleArg_n], n >= 1
PositiveLiteral ::= pred(Name,RuleArgs)
ProgramBodyLiteral ::= PositiveLiteral
                     | naf(pred(Name,RuleArgs))
                     | naf(ExceptionId,pred(Name,RuleArgs))
Member ::= pred(Name,RuleArgs)
```

`GroundArgs` and `RuleArgs` are proper lists with arity `n >= 1` and no fixed upper bound. Predicate identity is `Name/n`; zero-arity and improper-list predicates are `shape` errors. The committed `nary-ternary` chain validates, compiles, executes, and replays a three-variable ternary rule. The `nary-same-name-cross-arity` chain proves that `p/2` may depend on `p/1` without forming a cycle, pinning arity as part of predicate identity.

Every metadata/clause section may be empty. Exactly one goal is required and is final. Fact clauses precede closed-world declarations, which precede rule clauses, which precede alternative sets. Program identity binds clause kind to body shape: `fact_id` if and only if the body is `[]`, and `rule_id` if and only if the body is non-empty. The goal ID is `query_id`. Origin and branch-group uniqueness/order mirror IR v3; alternative sets retain their IDs and policy/body/member fields but no source field.

A yes/no goal remains `goal(query_id(sentence(S),clause(C)),pred(Name,GroundArgs))`; its predicate is ground. The admitted wh form is exactly `goal(query_id(sentence(S),clause(C)),wh(who),pred(Name,[var(1)]))`. The marker must be exactly `wh(who)`, the predicate must have exactly one argument, and that argument must be exactly the query variable `var(1)`. Any other `goal/3` content is `shape`. Program v3 executes this constructor and emits the complete answer-v3 wh form defined below.

A rule body may contain positive `pred/2`, bare `naf(pred(...))`, and labeled `naf(ExceptionId,pred(...))` literals. Canonical body order is all positive literals first and then all NAF literals, preserving source order within each block. Every NAF and head variable requires positive-body coverage. A ground NAF-only body is admitted, but a rule body remains non-empty. NAF in a head, goal, predicate argument, alternative row, or any other non-body position is `shape`; program v3 has no blanket `naf` class.

Each labeled literal requires the same exact declaration correspondence as IR v3. Directly tampered program records are validated independently: deleting/changing a declaration, affected rule, target key, label, or body position rejects before model construction. Bare and labeled NAF contribute identical negative dependency edges.

An alternative-set row admits at least two pairwise `==`-distinct positive members, a positive-only body, dense member-then-body variable numbering, fixed policy atoms, and member-variable body coverage. It is not a clause or goal and contributes no store atoms or dependency edges.

The finite function-free predicate dependency graph has one key `Name/Arity` per predicate. A positive edge goes from each rule-head key to each positive body-literal key. A negative edge goes from each rule-head key to each NAF-target key. Any directed cycle over the combined signed graph is rejected, including a positive-only cycle, a mixed-polarity cycle, or a self-loop. This cycle-free predicate dependency graph is a sufficiency-only restriction: full cycle freedom is stronger than stratification, because stratification permits positive-only cycles, so a stratification trivially exists for every admitted program. Finiteness is essential to that statement; on an infinite ground dependency graph, merely excluding negative cycles would not by itself establish the required well-founded stratum assignment.

### Program v3 semantics

With NAF present there is generally no global least model. For example, the rule `p :- naf q` has incomparable minimal classical models `{p}` and `{q}`. Program v3 therefore uses the **standard (stratified) model**: it is minimal and supported, and is computed as a least fixpoint separately at each stratum while all lower strata are frozen as extensional facts. This is the standard construction of Apt, Blair, and Walker, *Foundations of Deductive Databases*, chapter 2, Theorems 7, 8, and 11; Abiteboul, Hull, and Vianu, section 15.2, Theorem 15.2.10 and Proposition 15.2.11, which states “minimal, not necessarily least”; and Green et al., *Foundations and Trends in Databases*, section 2.3.2, where negated predicates are treated as EDBs during a stratum's positive fixpoint.

On this finite cycle-free profile, the standard model coincides with the unique perfect model, the unique stable model, and the total well-founded model. The coincidence follows from Apt and Bol, Theorem 6.10, Theorem 6.20, and Corollaries 7.6-7.7, together with Van Gelder, Ross, and Schlipf, *JACM* 38(3), Theorem 6.1. Thus the admitted NAF reading is robust across these standard semantics.

`not provably P` means that `P` has no derivation under the closed program; it never means classical falsity. Bare and labeled forms use the same operational absence test. A labeled form additionally carries the rule-scoped completeness declaration and audit identity; a bare undefined target remains underivable and therefore succeeds exactly as before. Closed-world and alternative-set rows are metadata, not clinical atoms.

### Normative stratified kernel schedule

For a predicate key `p`, its stratum is the maximum, over all clauses whose head key is `p`, of every positive dependency's stratum and one plus every NAF dependency's stratum, with minimum 1. Facts contribute a lower bound of 1. A predicate key with no defining clauses has stratum 1. The assignment is well-defined because the signed predicate graph is finite and cycle-free.

Evaluation proceeds in ascending stratum order. A clause belongs to its head key's stratum. Within one stratum, the v1 repeated-pass snapshot fixpoint runs unchanged but is restricted to that stratum's clauses: clauses are visited in ascending stream sequence; each clause sees one store snapshot taken at clause entry; positive body matching is leftmost-outermost over snapshot insertion order; generated heads are inserted immediately into the one growing, insertion-ordered store; `nb_setarg/3`-backed growing-store deduplication retains only the first witness and never materializes all candidate tuples before deduplication. A pass that inserts nothing completes the stratum. The store grows monotonically across strata.

An NAF literal first substitutes its pattern under the bindings established by preceding positive literals. The kernel asserts that the resulting `pred/2` is ground; failure of that internal invariant is stage `run`, class `uncaught`, exit 2. The literal succeeds exactly when the substituted atom is absent from the clause-entry snapshot. Stratification guarantees that the target predicate belongs to a lower, already completed stratum, so snapshot absence equals absence from the final completed store. A successful bare literal contributes `naf(GroundAtom)`; a successful labeled literal contributes `naf(ExceptionId,GroundAtom)`. Each form costs one certificate node.

A single-stratum NAF-free program with no branch IDs, labeled NAF, closed-world declarations, or alternative-set rows evaluates exactly as before; its result bytes and first-witness proof tree remain unchanged. Branch IDs affect only cited identity, labeled NAF affects only declaration/leaf identity, and alternative rows add only their explicit result metadata.

Compilation first applies full IR v3 validation. It maps each fact to an empty-body clause, maps each rule to a clause without changing its complete ordinary/branch ID or body, copies every `closed_world/3` row exactly, and maps each `alternative_set/7` to source-free `alternative_set/6` without changing its ID, members, body, or policy fields. A yes/no query becomes the final `goal/2`; an exact wh query becomes `goal(query_id(...),wh(who),pred(Name,[var(1)]))`. Section order, variable numbering, and both NAF forms are preserved. Before serialization, the generated program stream passes the independent program v3 validator, so no partially transformed or internally invalid program can reach stdout.

The `run` reader requires exactly `cnl_program_record(3)`. Program v1, program v2, and every other non-v3 header fail `envelope` before later shape or safety checks.

## Inference and answer record v3

Canonical inference invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- run <record.program.pl >record.result.pl
```

A yes/no answer record has one of these exact layouts:

```text
cnl_answer_record(3).
<document>
program(sha256('<hex>')).
<alternative_set>*
answer(QueryId,GroundPredicate,proved).
<proof>
```

```text
cnl_answer_record(3).
<document>
program(sha256('<hex>')).
<alternative_set>*
answer(QueryId,GroundPredicate,not_proved).
```

Each copied alternative row has the exact program constructor:

```prolog
alternative_set(
    AlternativeSetId,
    members([Member1,Member2,...]),
    body([PositiveLiteral,...]),
    satisfaction(any_member),
    exclusivity(not_asserted),
    exhaustiveness(not_asserted)
).
```

Line 3 is mandatory and immediately follows the document line. Its digest is exactly 64 lowercase hexadecimal characters: SHA-256 over the exact raw program-record bytes read by `run`, before UTF-8 decoding, including the final LF. The digest atom is always single-quoted. Some valid hashes begin with a digit and therefore require quotes as Prolog atoms; forcing quotes for every hash is the only uniform fixed byte representation. This digest binds the answer to the program bytes and detects accidental or adversarial modification when independently recomputed, but it is integrity metadata, not authentication.

The document line remains byte-verbatim from the program record. The digest is line 3 and immediately follows it. Every program `alternative_set/6` row is copied byte-for-byte in program order after the digest and before the answer; no closed-world declaration row is copied separately because the digest binds the complete program and each successful labeled leaf carries its exception ID. Exactly one top-level `proof/3` term is present if and only if a yes/no answer is `proved`; it is final. `Atom` is a derived ground `pred/2`, `ClauseId` is the cited fact/rule ID including any branch field, and proof children correspond one-for-one with the cited clause body literals in body order. A fact proof has `[]`. The root atom is `==` to both the answer atom and program goal. `not_proved` remains unknown and carries no proof.

Proof children have this complete grammar:

```text
SubProof ::= proof(Atom,ClauseId,[SubProof,...])
           | naf(Atom)
           | naf(ExceptionId,Atom)
```

A positive body literal corresponds to a recursive `proof/3` child. A bare NAF literal corresponds to `naf(GroundAtom)`. A labeled NAF literal corresponds to `naf(ExceptionId,GroundAtom)`. Neither absence leaf uses a `proof/3` wrapper. Certificate construction follows the first witness retained by the kernel.

Replay independently checks a total ground substitution for every cited clause. At every positive proof node, the cited fact/rule ID must be `==` to the completed store's retained first-witness ID for that conclusion; a byte-identical sibling clause is not an interchangeable citation. For a positive child, the substituted body predicate must be `==` to the child's conclusion and the child must recursively replay. For an NAF child, replay requires the exact bare-versus-labeled leaf shape used by the cited body position, a ground `pred/2` whose arguments are `named/1`, exact labeled exception-ID identity where present, positional correspondence with the substituted NAF pattern, and `==`-absence of that atom from the completed kernel store. NAF leaves never introduce bindings and retain their existing label-and-position law. Changing a branch clause ID, dropping/adding/swapping a label, changing the ground target, or changing list shape fails replay. Proper `'[|]'/2` list structure is checked throughout, and every rule variable must receive a total ground binding.

The trust boundary is explicit. Saturation and first-witness selection are kernel responsibilities. Replay checks NAF absence against the kernel's completed store, not against syntax alone. Fresh-process deterministic runs make that store reproducible, but an external verifier must recompute the standard model from the bound program bytes to verify both derivability and NAF absence independently. Replay certifies the emitted tree's sound correspondence to the completed store; it does not replace saturation.

Certificates remain trees without shared subproofs, so their size can grow exponentially with rule structure. DAG sharing remains deferred. The kernel predicate `certificate_node_cap(1000000)` defines the proof-emission preflight bound. After the completed model and the complete answer set are known, but before any proof tree is constructed, the kernel memoizes an expanded-node count for each retained first-witness atom using `==` keys. An atom contributes one node plus each witness-body position: a positive child contributes that child's memoized count, and an NAF leaf contributes one. Every count and sum saturates at `Cap+1`.

The preflight total is the sum across every tree the record would emit: all wh answers, the one proved yes/no answer tree, or zero for `not_proved` and `answers([])`. A total above the cap deterministically rejects at stage `run`, class `resource`, exit 1, with zero stdout and detail `certificate_nodes_exceed_cap(1000000)`. A replay failure remains an internal invariant break and surfaces as stage `run`, class `uncaught`, exit 2. Only after the preflight, construction, and replay succeed is the result stream canonically serialized, reparsed, validated as an answer-v3 fixed point, and committed once to stdout.

For a wh goal, answer v3 uses this documented form:

```prolog
cnl_answer_record(3).
document(docid('<docid>'),source_sha256('<hex>'),ulex(<ulex-term>)).
program(sha256('<hex>')).
<alternative_set>*
answer(
    query_id(sentence(S),clause(C)),
    wh(who),
    pred(Name,[var(1)]),
    answers([GroundAtom,...])
).
proof(GroundAtom,ClauseId,[SubProof,...]).
...
```

`answers/1` lists all derived ground instances of the goal pattern. The list is strictly ascending by the canonical serialization bytes of each ground query atom, hence duplicate-free; this project-owned order is independent of evaluation schedule. Zero matches are represented explicitly as `answers([])`. Each listed atom has its own first-witness proof tree, and those top-level proof terms follow the answer term in the same order. The constant universe is exactly the `named/1` constants present anywhere in the program record; wh execution invents no constants.

Answer-record consumers must require exactly `cnl_answer_record(3)` and reject every other answer envelope version. Answer records are terminal in-tree artifacts, so the current CLI produces and self-checks them rather than accepting them as another stage's input. The reparsed self-check requires the shared v3 header, `document/3`, complete `program(sha256(Atom))`, then zero or more ground, well-shaped `alternative_set/6` rows with pairwise `==`-distinct members before the answer. A yes/no answer requires a positive `query_id`, exactly one trailing `proof/3` with a root identical to the answer atom for `proved`, or zero trailing terms for `not_proved`. For wh output, it requires the exact `wh(who)` marker and `pred(Name,[var(1)])` pattern, a proper canonically byte-sorted `answers/1` list of ground unary `pred/2` atoms with `named/1` arguments, and one ordered top-level proof per answer with an identical root atom; `answers([])` requires zero proofs.

`run` validates the program artifact independently rather than trusting the compiler. After the shared framing gates (`input_utf8`, `syntax`, `canonical`), program v3 content rejection is owned by `envelope`, `query_count`, `section_order`, `shape`, `identity`, `ordering`, `scope`, `safety`, `exception`, and `cycle`. The post-model certificate preflight can additionally reject `resource`. Program-side `naf` is not a rejection class. Alternative metadata is validated and copied but never evaluated. Failure emits no result prefix.

## End-to-end document chain

For each `<docid>`, the project-owned artifact chain is
`<docid>.drs.pl` → `<docid>.ir.pl` → `<docid>.program.pl` → `<docid>.result.pl`.

`tools/pipeline.py` is the production orchestrator. The [pipeline contract](pipeline.md)
owns its pre-flight checks, sibling `front/` and `chain/` staging layout, child-failure relay,
manifest bindings, and transactional publication. The front end populates `front/` with its
manifest and DRS records; downstream IR, program, and result records are written to the
sibling `chain/` directory.

For each sorted document ID, the pipeline runs `lower`, `validate`, `compile`, and `run` in
separate ambient-init-free SWI-Prolog processes. Every process receives a fresh disk read of
the preceding artifact. `validate` succeeds with zero stdout and zero stderr; each
transforming stage emits one complete canonical record only after its checks pass. The
individual `ir_tool` commands specified below remain the low-level fixture and diagnostic
interface, but their outputs belong in separate scratch destinations and must never be
appended to a completed front-end `OUT_DIR`.

The slice byte authorities are `tests/fixtures/slice/golden/manifest.pl`,
`tests/fixtures/slice/golden/<docid>.drs.pl`,
`tests/fixtures/slice/ir/<docid>.ir.pl`,
`tests/fixtures/slice/program/<docid>.program.pl`, and
`tests/fixtures/slice/result/<docid>.result.pl`. A generated mismatch is a contract failure,
not a regeneration instruction.

`tests/slice-harness.sh` stages and builds a fresh APE copy, runs all four committed slice
documents twice through the complete chain, compares every artifact-producing stage to its
golden before continuing, and treats validation as a separate zero-stream gate. It also
pins each pass's complete inventory, proves both passes byte-identical, checks that a
lower-stage rejection leaves no non-empty downstream artifact, and verifies staged and
vendored APE cleanliness. In the pinned SWI 9.2.9 CI job, it runs after
`tests/pipeline-harness.sh` and before the registry harness, guideline harness, and final
repository-cleanliness gate.

## CLI

Canonical IR validation invocation from repository root:

```sh
swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' -- validate <record.ir.pl
```

All four commands are implemented: `lower | validate | compile | run`. `lower` accepts one canonical `ace_front_end_record(1)` DRS record, `validate` accepts one canonical IR v3 record, `compile` accepts one canonical IR v3 record, and `run` accepts one canonical program v3 record. Missing arguments, an unknown command, or extra arguments are `usage` errors.

The tool pins encoding, double-quote, back-quote, character-escape, syntax-error, and writer behavior; it does not depend on ambient SWI defaults. `-f none -F none` remains part of every canonical process invocation.

I/O contract:

- Validate success: exit 0, zero stdout, zero stderr.
- Lower success: exit 0, one canonical IR v3 record on stdout, zero stderr.
- Compile success: exit 0, one canonical program v3 record on stdout, zero stderr.
- Run success: exit 0, one canonical answer v3 record on stdout, zero stderr.
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

The active envelopes are independently versioned as `cnl_ir_record(3)`, `cnl_program_record(3)`, and `cnl_answer_record(3)`. V3 includes branch IDs, closed-world declarations, labeled NAF, labeled absence leaves, and alternative-set metadata while retaining every legacy constructor and byte path. Readers reject every unknown constructor or field; v3 has no ignored extension surface.

Any change to constructors, arities, admitted argument forms, section cardinality/order, identity/provenance rules, canonical bytes, or logical semantics requires a new envelope version. Every current reader rejects every non-v3 envelope before payload checks: `validate` and `compile` reject IR v1/v2/other versions, `run` rejects program v1/v2/other versions, and answer consumers reject every non-v3 answer record. The committed `envelope-wrong-version` IR and `envelope-wrong-header` program specimens retain otherwise legal payloads under stale v2 headers and pin exact envelope-first rejection. The v1 specimens pin older-version precedence even though later safety validation would also reject their payloads. Answer self-check probes use stale v2 plus non-current v4 headers for the same law.

## Capability status and deferrals

| Area | Current treatment |
|---|---|
| Intervals and rationals | No constructors; reject. |
| Temporal and dose algebra | No constructors; reject. |
| Direction, strength, and certainty | No annotations; reject. |
| DNF antecedent disjunction | **Shipped:** structural `v/2` expands per rule by Cartesian conjunction and left-before-right disjunction, with deterministic branch IDs and a fail-closed 64-branch cap. |
| Labeled exception IDs and closed-world declarations | **Shipped:** in-record defined NAF targets receive deterministic rule/body-position labels, explicit exact-key declarations, labeled program literals, and replay-checked labeled absence leaves. Lowered undefined targets remain bare and byte-stable; generic hand-authored bare NAF remains admitted without a declaration. |
| Action alternatives | **Shipped metadata:** root/consequent two-action `v/2` lowers to `alternative_set` with `any_member`, nonexclusive, nonexhaustive policy. Rows survive program/result transport but derive no member or satisfaction atom. |
| Explicit negation and a false outcome | Absent; `not_proved` remains unknown. |
| `transitive_relation` core | **Shipped:** positive `predicate/4` lowers to binary `pred/2` data in root facts, rule heads/bodies, and ground yes/no questions; event erasure requires exact single occurrence. |
| Adjectival properties and comparatives | **Shipped:** positive `property/3` degree `pos` lowers predicative and attributive adjective facts/body literals; `property/4` degrees `comp_than` and `pos_as` lower to byte-distinct binary predicate names. Other degrees, property arities, property NAF, and property wh remain deferred/rejected. |
| Copular heads and questions | **Shipped:** noun, positive-adjective, and comparative carrier+`be` arrangements normalize to one rule head or ground yes/no query. Attributive consequents remain rejected by the single-head law. |
| `multi_entity` role edges | **Shipped base:** exact `relation(R1,of,R2)` lowers to `pred(of,[R1',R2'])`, including root named-copula resolution and rule arguments. Other relation lemmas, richer attachment/scope, and multi-condition copular relation questions remain deferred. |
| NAF execution and ACE lowering | **Shipped:** exact antecedent `~/1` profiles lower to bare NAF for undefined targets or labeled NAF plus declaration for defined targets; both execute under the stratified model and replay as exact ground absence leaves. Literal APE `unless`/`without` remains unavailable; the defined-predicate plus `not provably` paired-rule pattern is authorable. |
| Recursion and tabling | Any signed predicate cycle, including a positive-only cycle, is rejected. |
| Conflict detection | No rule-pair conflict analysis. |
| Proof enumeration and DAG sharing | First witness only; certificates remain trees and DAG structure sharing remains deferred. |
| Proof and answer resource preflight | **Shipped:** `certificate_node_cap(1000000)` bounds the total expanded nodes across all trees emitted by one record. |
| Prose rendering | Outside these records. |
| Program-digest binding inside answer records | **Shipped:** answer v3 line 3 binds the exact raw program bytes with SHA-256 for integrity only; it is not authentication. Digest authentication remains deferred. |
| Wh answers | **Shipped:** program v3 executes exact `wh(who)` goals and emits canonically byte-sorted `answers/1` with ordered first-witness proofs or explicit `answers([])`. |
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
cnl_ir_record(3).
document(docid('slice'),source_sha256('bf432c59846951be8568be538cfa2c5fcdc41d35b7ede4d0bc0fd5c4aff7c2c4'),ulex(sha256('6015f9a18e4d4957b30e04342d2ff2700bf0e18b13bf3b95452a2d5563c5b614'))).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred(wait,[named('John')]),source(sentence(2),tokens([2]))).
rule(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])]),source(sentence(3),tokens([2,4,5]))).
query(query_id(sentence(4),clause(1)),pred(recover,[named('John')]),source(sentence(4),tokens([3]))).
```

## Binary transitive grounding example

The adapter-derived `m6-transitive` lower fixture exercises `predicate/4` in every positive position: root fact, rule body, rule head, and ground yes/no question. Its v3 IR payload is:

```prolog
fact(fact_id(sentence(1),clause(1)),pred(like,[named('John'),named('Mary')]),source(sentence(1),tokens([2]))).
fact(fact_id(sentence(2),clause(1)),pred(patient,[named('John')]),source(sentence(2),tokens([2,4]))).
rule(rule_id(sentence(3),clause(1)),pred(help,[var(1),named('Mary')]),body([pred(patient,[var(1)]),pred(like,[var(1),named('Mary')])]),source(sentence(3),tokens([2,4,6]))).
query(query_id(sentence(4),clause(1)),pred(help,[named('John'),named('Mary')]),source(sentence(4),tokens([3]))).
```

The corresponding answer is `proved`; its proof cites both the unary `patient/1` fact and binary `like/2` fact. `transitive-event-in-use.pl` reuses the transitive event as a relation argument, while `transitive-event-in-question.pl` reuses it inside the final question; both pin `ir_tool_error(lower,referent,-(root_condition(1),event_in_use)).` The `transitive-rule-body-event-in-use`, `transitive-rule-head-event-in-use`, and `transitive-question-event-in-use` reds pin the same losslessness law at the remaining transitive lowering call sites.

## Of-role grounding example

The adapter-derived sentence `John is a patient of Mary.` contains `relation(A,of,named('Mary'))`, where copula `predicate(B,be,named('John'),A)` names `A`. Lowering preserves the DRS relation order and emits:

```prolog
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(1),clause(2)),pred(of,[named('John'),named('Mary')]),source(sentence(1),tokens([2,5]))).
rule(rule_id(sentence(2),clause(1)),pred(wait,[var(1)]),body([pred(patient,[var(1)]),pred(of,[var(1),named('Mary')])]),source(sentence(2),tokens([2,3,5]))).
```

The relation fact includes token 5's `of` anchor and token 2's copula anchor used to resolve `A`. The rule body reuses the existing `var(1)` binding. The full fixture derives `wait(named('John'))`; the proof contains a positional `pred(of,[named('John'),named('Mary')])` child. The companion `m6-of-head-query` fixture resolves both root relation arguments through unique copulas, lowers `of/2` in a rule head, and proves a direct ground relation question.

## Property grounding example

The adapter-derived `m6-prop-attr` fixture is:

```text
John is a helpful and careful patient.
Every helpful and careful patient recovers.
Does John recover?
```

Its committed IR bytes preserve root DRS emission order—class, then each attached property—and keep the rule-body literals in the same semantic order after the shared copula is erased:

```prolog
cnl_ir_record(3).
document(docid('m6-prop-attr'),source_sha256('e07ccd306fc8494c02bed3cf26e7af5f2288e4765b54d52ab7451c4aa8297176'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,7]))).
fact(fact_id(sentence(1),clause(2)),pred(helpful,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(1),clause(3)),pred(careful,[named('John')]),source(sentence(1),tokens([2,6]))).
rule(rule_id(sentence(2),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(helpful,[var(1)]),pred(careful,[var(1)])]),source(sentence(2),tokens([2,4,5,6]))).
query(query_id(sentence(3),clause(1)),pred(recover,[named('John')]),source(sentence(3),tokens([3]))).
```

The committed comparative fixtures pin the reserved name encoding in facts, rule heads/bodies, and copular questions. `m6-prop-comp-than` contains:

```prolog
fact(fact_id(sentence(1),clause(1)),pred('helpful comp_than',[named('Mary'),named('John')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(3),clause(1)),pred('helpful comp_than',[var(1),named('Mary')]),body([pred(patient,[var(1)])]),source(sentence(3),tokens([2,3,5]))).
query(query_id(sentence(4),clause(1)),pred('helpful comp_than',[named('John'),named('Mary')]),source(sentence(4),tokens([1,4]))).
```

`m6-prop-pos-as` independently carries exact atom `'helpful pos_as'` through its fact, comparative antecedent literal, ground question, program, answer, and proof artifacts; no lowering or execution stage collapses the two degree names.

## DNF branch grounding example

The `m6-disjunction` fixture lowers the antecedent `patient(X), (wait(X) or sleep(X))` to two rules with one source origin:

```prolog
rule(rule_id(sentence(3),clause(1),branch(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])]),source(sentence(3),tokens([2,4,8]))).
rule(rule_id(sentence(3),clause(1),branch(2)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(sleep,[var(1)])]),source(sentence(3),tokens([2,7,8]))).
```

Only branch 1 is satisfied in the fixture, so the proof cites its complete branch ID:

```prolog
proof(pred(recover,[named('John')]),rule_id(sentence(3),clause(1),branch(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),proof(pred(wait,[named('John')]),fact_id(sentence(2),clause(1)),[])]).
```

`m6-disjunction-mixed` pins nested order and conjunction preservation: its three bodies are `patient,wait`; `patient,sleep,cough`; `patient,smoke`, in that order. `m6-disjunction-cap` pins exactly 64 accepted branches and `disjunction-cap-exceeded` pins 65 as the exact fail-closed boundary.

## Labeled-exception grounding example

The paired pattern “patients that cough are smokers” plus “patients that are not provably smokers recover” defines the exception target and therefore lowers to:

```prolog
closed_world(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),affects(rule_id(sentence(3),clause(1))),predicate_key(smoker,arity(1))).
rule(rule_id(sentence(2),clause(1)),pred(smoker,[var(1)]),body([pred(patient,[var(1)]),pred(cough,[var(1)])]),source(sentence(2),tokens([2,4,5,7]))).
rule(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),pred(smoker,[var(1)]))]),source(sentence(3),tokens([2,4,8,9]))).
```

With `patient('John')` present and `smoker('John')` absent, the result is proved and ends in the exact labeled absence leaf:

```prolog
proof(pred(recover,[named('John')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),naf(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),pred(smoker,[named('John')]))]).
```

Changing the label, body position, affected rule, target key, or ground atom fails validation or replay. If the smoker rule derives `smoker('John')`, the NAF test fails and this recovery rule contributes no witness.

## Action-alternative grounding example

The `m6-alternative-set` fixture represents “every patient offers Mary or arranges Mary” without asserting either member:

```prolog
alternative_set(alternative_set_id(sentence(3),clause(1)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(3),tokens([2,3,6]))).
```

Compilation drops only provenance:

```prolog
alternative_set(alternative_set_id(sentence(3),clause(1)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted)).
```

The identical row appears after the program digest in the result. That fixture's `offer('John','Mary')` query is proved by an independent fact, and its proof cites only that fact. Removing the fact would make the query `not_proved`; the alternative-set row alone cannot derive `offer`, `arrange`, or a satisfaction predicate.

## NAF grounding example

The committed `slice-naf` document is:

```text
John is a patient.
Every patient that does not provably smoke recovers.
Does John recover?
```

Its committed IR bytes are:

```prolog
cnl_ir_record(3).
document(docid('slice-naf'),source_sha256('074d6ca7f0e5127e06af01f24a04ce434010ed1e9e80613a85fd9ad81f78ff6e'),ulex(sha256('7be3ff7a729f2d12bbc7d204b70ab93c419f936f6ad751afd8018c3c09cc0bdc'))).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(pred(smoke,[var(1)]))]),source(sentence(2),tokens([2,7,8]))).
query(query_id(sentence(3),clause(1)),pred(recover,[named('John')]),source(sentence(3),tokens([3]))).
```

This exercises the first admitted antecedent `~/1` profile: the positive
`patient` condition binds the outer entity before the unanchored nested
intransitive `smoke` condition becomes an NAF literal over the same `var(1)`.
The committed result contains this ground absence-checked proof leaf:

```prolog
proof(pred(recover,[named('John')]),rule_id(sentence(2),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),naf(pred(smoke,[named('John')]))]).
```

## Wh grounding example

The committed `slice-wh` document is:

```text
John is a patient.
John waits.
Every patient that waits recovers.
Who recovers?
```

Its committed IR bytes include the exact query/4 constructor:

```prolog
cnl_ir_record(3).
document(docid('slice-wh'),source_sha256('80cf551d677bcd4ccf6b94b7299a147a92b827b84bd624312780c2f0f2c91775'),ulex(sha256('6015f9a18e4d4957b30e04342d2ff2700bf0e18b13bf3b95452a2d5563c5b614'))).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred(wait,[named('John')]),source(sentence(2),tokens([2]))).
rule(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])]),source(sentence(3),tokens([2,4,5]))).
query(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),source(sentence(4),tokens([1,2]))).
```

The committed answer line grounds that pattern to `John`:

```prolog
answer(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),answers([pred(recover,[named('John')])])).
```

## Wh answer grounding example

The committed `tests/fixtures/run/green/wh-multi-order.program.pl` fixture inserts `named(a)` before `named('z z')`, then derives both `recover/1` atoms:

```prolog
cnl_program_record(3).
document(docid('wh-multi-order'),source_sha256('4444444444444444444444444444444444444444444444444444444444444444'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named(a)]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(patient,[named('z z')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)])])).
goal(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)])).
```

The exact fixture-backed answer bytes are:

```prolog
cnl_answer_record(3).
document(docid('wh-multi-order'),source_sha256('4444444444444444444444444444444444444444444444444444444444444444'),ulex(none)).
program(sha256('4d39502bf78b97f0001ae418146e0ecd22b747a8e10abe74788a0747e33abff1')).
answer(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),answers([pred(recover,[named('z z')]),pred(recover,[named(a)])])).
proof(pred(recover,[named('z z')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('z z')]),fact_id(sentence(2),clause(1)),[])]).
proof(pred(recover,[named(a)]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named(a)]),fact_id(sentence(1),clause(1)),[])]).
```

This order differs both from store insertion order and from SWI standard term order. At the first differing byte inside the canonical atom serialization, quoted `'z z'` begins with `0x27` while unquoted `a` begins with `0x61`, so the project-owned canonical-byte order places `'z z'` first.
