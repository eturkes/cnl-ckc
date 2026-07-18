# IR and rule-semantics requirements for M3-M4

Plan-phase research findings (2026-07) for ACE → APE DRS → project IR → Prolog. This is a requirements/pitfalls catalog, not a binding specification. M3 derives a versioned IR from observed DRS; M4 states and tests the exact logic-program semantics it implements.

## 1. Clinical content the IR must preserve

| Phenomenon | Minimum representation | Failure if collapsed |
| --- | --- | --- |
| Population/eligibility | Typed predicates; quantitative guards; exact `and`/`or`; clause provenance | False conflicts when populations do not overlap |
| Action | Kind, normalized target, route/formulation/dose/frequency/duration as applicable | Clinically distinct actions merge |
| Direction | `for`, `require`, `permit`, `against`, `avoid`, `contraindicate` | Wrong-polarity rules invert conflict results |
| Strength/certainty | Separate strength and evidence-certainty fields | Direction, recommendation force, and evidence grade become conflated |
| Time | Anchors, units, bounds, endpoint openness, event ordering | “within,” “for,” and “before” become indistinguishable |
| Dose | Exact quantity/unit/range plus route, frequency, duration, denominator | Untyped numbers compare unsafely |
| Exception | Labeled condition, affected rule, scope, own source span | Exception becomes an ordinary guard or disappears |
| Contraindication | First-class negative direction, optionally linked as an exception | An exception-only encoding loses the opposing recommendation |
| Factual rule | Strict factual consequent distinct from normative advice | Definition and recommendation use the same predicate |
| Provenance | Document, sentence/region, rule, clause, generated-clause links | Explanations and cores cannot be audited |

### 1.1 Context and population

Initial context shape: finite nonempty DNF; each branch is a nonempty conjunction of positive concepts, explicit negative concepts, and quantity intervals. Later extensions add slot equality and event-relative time. Never flatten DNF: `(adult and renal impairment) or pregnant` is not the conjunction of all three.

Give source blocks and lowered DNF clauses deterministic IDs. Convention semantics belong in versioned mappings: “adult” may normalize to `age >= 18`, but must not also survive as an unrelated Boolean unless declared. Unknown or unsupported clinical content becomes a typed residual, never an omitted atom.

### 1.2 Direction, strength, certainty

Keep four axes distinct: factual polarity; normative direction; recommendation strength; evidence certainty. A small M4 direction policy may group:

- positive = `for | require | permit`;
- against = `against | avoid`;
- contraindicating = `contraindicate | avoid`.

The table is versioned policy, not lexical intuition. Strength/certainty remain explanation-visible even when baseline conflict detection ignores them. They do not become rule priorities until an explicit priority policy exists.

## 2. Exact interval, temporal, and dose algebra

### 2.1 Bounds over a declared domain

Normalize a bound as `bound(rational, lower|upper, open|closed)`:

| Surface | Bound |
| --- | --- |
| at least | closed lower |
| more than | open lower |
| at most | closed upper |
| less than | open upper |

For one quantity, fold to the tightest lower/upper: greater lower wins; lesser upper wins; open wins at equal value. No bounds = all rationals. A two-sided range is empty iff lower > upper, or lower = upper with either side open. Intersection = bound union then normalization; overlap = nonempty intersection. Public predicates fail closed on floats, invalid markers/sides, cyclic terms, or dimension mismatch.

### 2.2 Dense-rational finding

`18 < X < 19` is nonempty over the rationals, for example `37/2`, but empty over integers. An integer or CLP(FD) domain is therefore unsound for generic eligibility overlap. Use exact SWI rationals or a specified CLP(Q) representation; floats make endpoint equality and hashes unstable. Discrete quantities remain possible, but discreteness is a property of each quantity type.

### 2.3 Temporal constraints

Distinguish:

- value intervals such as age or laboratory ranges;
- duration bounds such as “for at least 4 weeks”;
- relative event windows such as “within 6 hours after X”;
- qualitative relations such as before, overlaps, during.

Value/duration ranges use the bound algebra. Relative event windows lower to difference constraints. A later full temporal layer may adopt Allen relations or an Event Calculus, but M4 must not encode them as ordinary numeric attributes without anchors.

### 2.4 Units and dosing

Store source number/unit, canonical quantity/unit, exact conversion, route, frequency, duration, and denominators such as `mg/kg/day`. Compare only dimension-compatible normalized values. “30 mg/day” and “0.5 mg/kg/day” are not directly comparable without body weight or a quantified relation. Conversion tables are committed, versioned inputs.

### 2.5 Value-by-marker test anti-pattern

A presence-mask test can pass while code rejects rationals, accepts floats, or ignores invalid markers. Cross representative values with openness and side markers, and compare to an independent full-law oracle. Include negative, zero, positive rational, malformed float-like value; open/closed/unknown; lower/upper/unknown. Every mutant needs an accepted base reaching the mutation, plus hand-oracled boundary, adjacency, reversed, single-sided, and malformed cases.

## 3. Negation, exceptions, and model choice

### 3.1 Do not conflate three operations

- Explicit/classical negation asserts the contrary proposition.
- Negation as failure (NAF), Prolog `\+ Goal`, means `Goal` cannot be proved under the selected operational semantics.
- Normative opposition relates directions on the same action.

Missing data is not explicit negative evidence. Plain SLDNF needs ground goals, termination, and suitable stratification; otherwise its declarative reading is unsafe.

### 3.2 Baseline exception profile

A thin PROLEG-style kernel is acceptable:

```text
applicable(R, C) :- guards_hold(R, C), \+ exception_holds(R, C).
```

Require labeled, range-restricted exceptions and a declared closed-world fixture policy. Keep the exception first-class in IR. Compile by lane:

- patient-fixture evaluation: NAF guard;
- symbolic conflict analysis: explicit negated context constraint;
- explanation: guards proved plus no labeled exception established.

Differential tests cover only the shared fragment; these lanes are not globally equivalent.

### 3.3 Stratification and stable models

M4 should reject recursion through negation: build the predicate dependency graph and forbid cycles containing a negative edge. This yields a unique stratified interpretation. If later evidence requires recursive defaults, specify one of:

- tabled well-founded semantics, including `undefined`;
- stable models, including zero/one/many models;
- cautious versus brave consequences.

Never collapse `undefined`, multiple models, and false. Stable-model adoption adds grounding/model costs unless a goal-directed engine such as s(CASP) is chosen.

## 4. Courteous/defeasible findings

Richer theories distinguish facts, strict rules, defeasible rules, defeaters, and a superiority relation. They fit guideline exceptions, but priority elicitation is brittle; ambiguity-blocking and ambiguity-propagating variants can disagree.

AceRules-style lessons:

- courteous reasoning resolves a head versus strong-negated head through explicit `overrides` priorities;
- no priority can skeptically suppress both sides;
- direction-blind NAF agrees only when the positive rule is not the courteous winner;
- if priority favors the positive rule, courteous reasoning may derive it while NAF still blocks it;
- exception-only modeling can yield a negative courteous conclusion without an explicit guideline `against`/`contraindicate` claim;
- the native rule tuple lacks this project’s direction/strength fields;
- upstream rule generation reparses text and loses DRS sentence/token provenance;
- courteous reasoning resolves conflicts; it does not report every underlying source conflict.

Use AceRules as a bounded differential reference, not the project IR. Lower DRS directly, carry modality/provenance explicitly, and defer priorities until real guidelines show the stratified exception profile underfits.

## 5. Rule-pair conflicts and conflict cores

### 5.1 Eligibility and overlap

A pair is conflict-eligible iff normalized action keys match and one direction is positive while the other is against/contraindicating. Then enumerate DNF branch pairs canonically. A branch pair overlaps iff:

- no concept is both required and excluded;
- every shared quantity’s exact-rational intersection is nonempty;
- later slot/temporal constraints are jointly satisfiable.

For independent Boolean concepts plus rational intervals this is a decision, not patient search. A pair conflicts if any branch pair overlaps. A rule’s own empty context is `condition_unsatisfiable`.

Compute raw overlap without exceptions and conditioned overlap with symbolic exception constraints. Raw overlap plus conditioned non-overlap is `exception_resolved_conflict`, not an opaque no-conflict.

### 5.2 Evidence and cores

Every conflict record carries rule IDs, normalized action key, opposing direction facts, selected branch IDs, contributing concept/interval constraints, an overlap witness where feasible, and clause-grain source refs.

Define:

- **opposing-direction core** = two rules + same-action equality + direction assertions;
- **eligibility-overlap witness** = canonical branch pair + satisfiable guard intersection;
- **conflict core** = their union.

If no minimization algorithm runs, call this a canonical contributing set, not a minimal core. No-conflict records are first-class and reason-coded: different action, same direction, concept clash, empty interval, unsatisfiable own guard, or exception-resolved overlap.

## 6. Prolog-target requirements

- Deterministic clause/fact order; canonical IDs, rationals, units, sets, DNF branches.
- Head, negative-goal, and arithmetic variables range-restricted by positive goals.
- No generated cut; operational control lives in a small reviewed kernel.
- Table recursion where admitted; reject programs outside the termination/stratification profile.
- Clean module/database state per run; deterministic dynamic-fact lifecycle.
- Record SWI identity and exact generated bytes; repeat in fresh processes.

Grounding is not just an ASP issue: non-ground NAF, explanations, or arithmetic can change answers or error. IR validation should carry modes and reject code whose negative/arithmetic goals are not ground when called.

Explanations are derivation data: query/answer, applied rule, satisfied guards/values, exception outcomes, subderivations, source refs, conflict/no-conflict reason. Sort siblings, deduplicate shared subproofs, and replay every step against the emitted program. Controlled prose is a renderer, not a certificate.

## 7. DRS-lowering boundary

APE parse success is weaker than profile acceptance. It can accept unintended constructions, silently resolve pronouns/definites, interpret capitalized unknown words as proper names with a warning, and choose a first parse without proving unique interpretation.

M2/M3 must:

1. pin one observed canonical DRS per admitted ACE pattern;
2. reject warnings, unknown lexicon items, pronouns, anaphora, ellipsis, and unregistered surfaces;
3. validate raw ACE and DRS because distinct surfaces may collapse to one DRS;
4. map DRS directly, preserving sentence/token provenance;
5. require a total mapping for every accepted constructor/field;
6. reject or residualize unsupported content instead of dropping it.

Specific hazards: certainty at an unmappable scope; interval-bearing concepts emitted as both concept and bound; negated interval concepts whose complement is not one interval.

## 8. Degeneration modes to retain

- well-formed wrong direction;
- correct verdict despite wrong threshold/condition;
- schema-valid but incoherent variable/operator/value/unit;
- integer-domain or float endpoint errors;
- value-by-marker vacuity;
- DNF collapse;
- exception, negative fact, and contraindication conflation;
- NAF treated as classical negation;
- courteous assumed equivalent to NAF;
- accepted field silently dropped;
- parse success treated as profile success;
- clause-order-dependent answer/termination;
- contributing set mislabeled minimal core;
- no-conflict pair closed without reason evidence.

## 9. Scope recommendation

M3: small lossless IR plus deterministic DRS→IR lowering. M4: definite, range-restricted, stratified Prolog; labeled NAF exceptions; exact-rational overlap; explicit direction groups; deterministic conflict evidence; replayable explanations. Defer priority logic, non-stratified stable models, probabilistic strength, full Event Calculus, and patient-care execution until evidence demands them.

## Public references

- Maher, “Propositional Defeasible Logic has Linear Complexity,” *TPLP* 1(6), 2001.
- Gelfond and Lifschitz, stable-model semantics, 1988; Van Gelder, Ross, and Schlipf, well-founded semantics, *JACM* 38(3), 1991.
- AceRules: `https://github.com/tkuhn/AceRules`; s(CASP): `https://github.com/SWI-Prolog/sCASP`.
- Kowalski and Sergot, Event Calculus, 1986; Allen, interval algebra, 1983; Dechter, Meiri, and Pearl, temporal constraint networks, 1991.
- SWI-Prolog: `https://www.swi-prolog.org/`; core standard ISO/IEC 13211-1.
