# SB finite-program oracle — enumeration manifest v1

Status = proposed; main freezes/amends this manifest through `LEAD-DIRECTIVES.txt` before oracle implementation. Scope = contract `sb` D5-2 + P2: finite whole-model semantics, not the producer/checker algorithm. Exhaustive = the parameter domains below, not every admitted program up to these maxima.

Semantic sources: `bench/sb/semantics.md`; `.agent/spec.md` Decisions Patient answers; `docs/REFERENCE.md` Comparison conditions + Arithmetic built-ins. Loading a whole knowledge base supplies the schema indicators + SWI comparison recipe. Producer, checker, kernel + `guidelines/schema.pl` implementation remain unread.

## Count

| family | exhaustive parameter domain | cases |
| --- | --- | ---: |
| G | ≤3 distinct clauses from the 66-clause ground catalogue; signed graph has no negative cycle | 10,847 |
| V | 256 extensional databases × 79 subsets of ≤2 variable-rule templates | 20,224 |
| C4 | 8 source comparators × 7 counts × 5 condition comparators × 7 counts × positive/NAF | 3,920 |
| C6 | 8 source comparators × 3 source counts × 5 condition comparators × 3 boundary offsets × 49 unit pairs | 17,640 |
| A-free | 2 operations × 56² operand pairs × free-output/NAF-output modes | 12,544 |
| A-bound | 2 operations × 9² operand pairs × 56 fully bound result pairs | 9,072 |
| A-partial | 2 operations × 9² operand pairs × (8 bound comparators + 7 bound counts) | 2,430 |
| I | 2 carrier domains × 64 extensional databases × 130 subsets of ≤3 identity-rule templates | 16,640 |
| B-shape | 2 wrong-shape terms × (4 + 6 + 6 + 6) argument positions | 44 |
| NC | eight named negative-cycle controls, outside admission | 8 |
| **admitted cases** | every family except NC | **93,361** |
| **all cases** | admitted + refusal controls | **93,369** |

Counts = parameterized test cases after the stated reductions. Families remain separate: no cross-family quotient of semantically equivalent programs. Each encoded case has its own document identity; no claim of 93,369 programs distinct modulo renaming/equivalence.

G count by clause count:

| clauses | raw subsets | admitted | negative-cycle exclusions |
| ---: | ---: | ---: | ---: |
| 0 | 1 | 1 | 0 |
| 1 | 66 | 48 | 18 |
| 2 | 2,145 | 933 | 1,212 |
| 3 | 45,760 | 9,865 | 35,895 |
| total | 47,972 | 10,847 | 37,125 |

## Vocabulary + schema encoding

Abstract predicates: `d/1`, `p/1`, `q/1`, `r/1`, `e/2`, `f/1`, `g/1`, `h/2`, `v/2`. Ordinary carrier = ordered atoms `[a,b]`. Each case uses `Doc = sb_oracle_<family>_<index>`; index follows the family loops below, starting at 0. Reserved atoms never alias carrier atoms.

| abstract literal | schema literal |
| --- | --- |
| `d(X)` | `guideline_entity(actual,X,domain,countable)` |
| `p(X)` | `guideline_entity(actual,X,p,countable)` |
| `q(X)` | `guideline_entity(actual,X,q,countable)` |
| `r(X)` | `guideline_operator(actual,X,should)` |
| `e(X,Y)` | `guideline_preference(actual,e,X,Y)` |
| `f(X)` | `guideline_entity(actual,X,f,countable)` |
| `g(X)` | `guideline_entity(actual,X,g,countable)` |
| `h(X,Y)` | `guideline_preference(actual,h,X,Y)` |
| `v(S,N)` | `guideline_cardinality(actual,result,na,S,N)` |

The encoding substitutes terms, including variables/identities, unchanged. Bodies + NAF conjunctions keep their structure. Built-ins keep their names + arguments. Comparison = every ground schema fact, including extensional facts + metadata, not deontic counts alone. `r/1` also exercises a should-box carrier.

Every case includes these facts, with its own Doc and carrier D:

```prolog
guideline_schema_version(2).
guideline_document(Doc,ace_sha256(H),ulex(none),vocabulary(none)).
guideline_supersession(Doc,1,source(oracle,'https://example.invalid/sb-oracle'),20220101).
d(D[0]).
d(D[1]).
```

`H` = quoted atom of 64 zeroes; `Doc`, `H`, `D[0]`, `D[1]` above are rendering notation, not Prolog variables. These synthetic metadata records are model facts; they add no implicit rule or guard. Generated programs are semantic fixtures, never KB content. Patient/document provenance distinctions + public-file envelopes are outside this domain.

## Canonicalization + bounds

- Alpha variables = `X,Y,L,K,S,N` in the positions specified below; each clause has a fresh scope. Distinct NAF-only variable names denote distinct local scopes. A repeated local inside one NAF stays correlated across its conjunction.
- G: rule sets + positive/negative literal sets; ascending catalogue indices. Commutative duplicates + repeated identical literals reduce to one representative. G fixes the distinguished ground carrier to `a` rather than doubling by `a↔b`. Predicate labels remain distinct.
- V/I: template subsets, no duplicate rules; EDB bits in the declared order. Other families use the literal Cartesian products below. No dedup by the observed model.
- Max source clauses including the five common facts = 15. Max non-fact clauses = 3. Max top-level body items = 4; max NAF items per body = 2; max conjunction length inside a NAF = 3. Nested NAF = absent. Max distinct variables per source clause = 4.
- Full per-case value universe, including fixed literal arguments, numeric outputs + fresh identities: ≤256 terms; identity dependency depth ≤2; dependency-list length ≤2; integer numerator magnitude + positive denominator <2³². Metadata compounds stay position-fixed, outside variable substitution.
- Conservative finite grounding: ≤4,096 ground clauses including NAF-witness auxiliaries; ≤512 possible schema heads. These are safety bounds, not truncation limits. All templates have finite structural bounds: carrier size 2, constructor dependency DAG `p/e → f → g/h`, no arithmetic feedback, no metadata variables.
- Candidate outside a bound = outside the domain. Enumerator asserts every declared case satisfies all bounds; a violation fails the suite, never removes a case or stops a fixpoint early. The fixed counts above must still hold.

## G — ground recursion + stratified NAF

Atoms = ordered `[p(a),q(a),r(a)]`. Clause pool: each of 3 heads × these 22 bodies, in this order:

1. empty body;
2. one positive atom (3);
3. one NAF atom (3);
4. two distinct positive atoms, increasing atom indices (3);
5. one positive atom + one NAF atom (3×3, same atom allowed);
6. one NAF of a conjunction of two distinct atoms, increasing indices (3).

Choose 0, 1, 2 or 3 pool clauses, in increasing indices. Signed graph: head→each body schema atom; NAF-conjunction members carry negative edges. Retain iff no negative edge lies within an SCC. This allows base-free/founded positive recursion, repeated supports, mutual positive recursion, several strata + conjunctive NAF. The 37,125 graph-rejected subsets are excluded candidates, not truncated positives and not the NC set.

## V — joins + correlated local variables

EDB bits, low index first: `[p(a),p(b),q(a),q(b),e(a,a),e(a,b),e(b,a),e(b,b)]`. Enumerate every 8-bit mask. Select every subset of 0–2 rules from this ordered catalogue (1+12+66 = 79):

```prolog
v0:  p(X) :- q(X).
v1:  q(X) :- p(X).
v2:  p(Y) :- p(X), e(X,Y).
v3:  q(X) :- e(X,Y), p(Y).
v4:  r(X) :- d(X), \+ q(X).
v5:  r(X) :- d(X), \+ (e(X,L), q(L)).
v6:  r(X) :- d(X), \+ (e(X,L), q(L), p(L)).
v7:  r(X) :- d(X), \+ (e(X,L), e(L,X)).
v8:  r(X) :- d(X), \+ (e(X,L), e(K,L), p(K)).
v9:  r(X) :- d(X), \+ (e(X,L), e(X,K), e(L,K)).
v10: r(X) :- d(X), \+ (e(X,L), q(L)), \+ (e(X,K), p(K)).
v11: r(X) :- d(X), e(X,Y), \+ (e(Y,L), q(L)), \+ (e(X,K), p(K)).
```

`X,Y` outside a NAF are outer variables. `L,K` confined to a NAF are existential there; the two NAFs of v10/v11 never share a local. All negative edges leave the r stratum; p/q recurse only positively. No rank filtering changes this Cartesian product.

## Numbers + built-in grids

Ordered sets:

- `C8 = [exactly,eq,geq,greater,leq,less,about,na]`.
- `C5 = [exactly,geq,greater,leq,less]` (converted condition comparators).
- `N7 = [-2,-1,rational(-1,2),0,rational(1,2),1,2]`.
- `C3 = [exactly,geq,less]`; `N3 = [-1,0,1]`.
- `Units = [second,minute,hour,day,week,month,year]`.
- Operations = `[guideline_product,guideline_quotient]`.

Each family iterates parameters left-to-right as its count row lists; the last parameter varies fastest. Numbers always use canonical integer/rational spelling. The finite numeric set also includes C6's computed condition bounds + the outputs of one enumerated arithmetic operation. These outputs are exact rationals, not rounded members of N7.

C4: for every `(S1,N1,S2,N2,mode)` in `C8×N7×C5×N7×[positive,naf]`, one rule:

```prolog
positive: r(a) :- guideline_satisfies(S1,N1,S2,N2).
naf:      r(a) :- \+ guideline_satisfies(S1,N1,S2,N2).
```

C6: parameters `(S1,n,S2,delta,U1,U2)` in `C8×N3×C5×[-1,0,1]×Units×Units`. For comparable units, `m = n*factor(U1)/factor(U2)+delta`; otherwise `m = n+delta`. Factors: second=1, minute=60, hour=3600, day=86400, week=604800; a separate component month=1, year=12. One rule `r(a) :- guideline_satisfies(S1,n,U1,S2,m,U2).` Thus every unit pair crosses its exact converted boundary, including open/closed endpoints, zero + negative values; day/month never convert.

A-free: all `(operation,operand1,operand2,mode)` in `Ops×(C8×N7)²×[free,naf]`:

```prolog
free: v(S,N) :- operation(S1,N1,S2,N2,S,N).
naf:  r(a) :- \+ operation(S1,N1,S2,N2,S,N).
```

In free mode `S,N` are built-in outputs + head variables. In NAF mode both are NAF-local existentials. No image means no free-mode v fact and a true NAF-mode r fact.

A-bound: operand pairs from `(C3×N3)²`; fully bound result `(S,N)` from `C8×N7`. One rule `r(a) :- operation(S1,N1,S2,N2,S,N).` This tests output equality, including the rule that arithmetic never returns `eq`.

A-partial: the same 9² operand pairs; choose either one bound S from C8 or one bound N from N7 (15 modes). The other output is fresh; head `v(S,N)` keeps both values. A bound output that disagrees with the relation fails; there is no overwrite of a bound argument.

## I — nonempty Deps + nested fresh identities, no growth cycle

Carrier variant 0 = `[a,b]`; variant 1 = `[a,Z]`, where `Z = '$guideline_id'(product,Doc,1,ref(0),[])`. EDB bits: `[p(D0),p(D1),e(D0,D0),e(D0,D1),e(D1,D0),e(D1,D1)]` (64 masks).

Source constructors:

```prolog
I(X)   = '$guideline_id'(product,Doc,1,ref(1),[X])
J(X)   = '$guideline_id'(product,Doc,1,ref(2),[X])
K(X,Y) = '$guideline_id'(product,Doc,1,ref(3),[X,Y])
```

`I/J/K` are manifest notation; they expand to the source identities above. Source Deps contain variables only; ground nested Deps arise through derivation. Choose every subset of 0–3 of these nine ordered rules (1+9+36+84 = 130):

```prolog
i0: f(I(X)) :- p(X).
i1: f(I(X)) :- e(X,Y), p(Y).
i2: g(J(X)) :- f(X).
i3: q(X) :- f(I(X)).
i4: r(X) :- d(X), \+ f(I(X)).
i5: r(X) :- d(X), \+ (f(I(L)), e(X,L)).
i6: h(X,Y) :- f(X), g(Y).
i7: g(J(X)) :- p(X).
i8: f(K(X,Y)) :- e(X,Y).
```

Construction flow never returns to p/e or f from g/h/q/r. i0+i2 reaches dependency depth 2; i8+i2+i6 produces joins over fresh nested terms. i3/i4/i5 require matching the identity's Deps, not treating its fixed tag as the whole identity. Repeated founded supports of a fact stay legal.

## B-shape — built-in failure without exceptions

Seeds: `guideline_satisfies(exactly,1,geq,0)`; `guideline_satisfies(exactly,1,day,geq,0,day)`; each arithmetic operation `(exactly,1,exactly,1,exactly,1)`. For each seed, each argument position, each replacement in `[a,Z]`, emit `r(a) :- <mutated call>.` Z is the empty-Deps identity above. These are valid source terms in invalid built-in slots; every call must fail without throwing. Noncanonical rationals/floats are outside source admission and not silently folded into this family.

## NC — explicit refusal controls

All keep the common five facts. Only these extra clauses/facts vary. Expected undefined sets below refer to schema literals; auxiliary-witness residuals may also be reported internally.

| id | extra program (abstract) | expected undefined |
| --- | --- | --- |
| NC1 | `p(a) :- \+ p(a).` | `{p(a)}` |
| NC2 | `p(a) :- \+ q(a). q(a) :- \+ p(a).` | `{p(a),q(a)}` |
| NC3 | `p(a) :- q(a). q(a) :- \+ p(a).` | `{p(a),q(a)}` |
| NC4 | `p(a) :- \+ (p(a),q(a)). q(a) :- p(a).` | `{p(a),q(a)}` |
| NC5 | `e(a,a). e(b,b). p(X) :- d(X), \+ (e(X,L),q(L)). q(X) :- p(X).` | `{p(a),p(b),q(a),q(b)}` |
| NC6 | `p(X) :- d(X), \+ q(X). q(X) :- d(X), \+ p(X).` | `{p(a),p(b),q(a),q(b)}` |
| NC7 | `p(a) :- guideline_product(exactly,1,exactly,1,exactly,1), \+ q(a). q(a) :- \+ p(a).` | `{p(a),q(a)}` |
| NC8 | `p(a) :- \+ q(a), \+ r(a). q(a) :- \+ p(a). r(a) :- r(a).` | `{p(a),q(a)}`; `r(a)` false |

NC never enters the accepted-positive denominator, even where a negative-cycle program has a stable model. These eight must expose the specified undefined schema literals under the independent oracle + the tnot rerun.

## Oracle/comparison obligations (step 2)

1. Ground independently from the semantic text over a finite conservative term closure. Translate each NAF conjunction to a witness auxiliary parameterized by its outer variables; its rules enumerate correlated existential witnesses. Preserve the three-valued result of these auxiliaries.
2. For the finite normal ground program, compute `Gamma(M)` by reduct + least positive fixpoint. Alternate lower/upper approximants from bottom/top to convergence. Report true, false + undefined; schema projection removes helper atoms only after convergence. No bounded iteration may count as a complete answer.
3. Every admitted case: undefined set empty; exact schema model = SWI tabled recipe + its tnot rerun. NC: exact true/undefined schema sets = tnot; admission status always refusal. Include metadata + false-head opportunities; counts alone never decide equality.
4. Properties: lower⊆upper each round; convergence is a fixpoint; a two-valued result equals its own reduct least model; simultaneous carrier/predicate renaming, clause-order reversal + alpha-renaming preserve the correspondingly mapped model. Property inputs come from the frozen domain, not checker internals.
5. Seen-red comparison controls per family: a correct-model positive control + a planted unequal model must exercise the real comparison path. Plants include omitted/added schema facts, wrong arithmetic output, mismatched identity Deps, flattened correlated NAF + collapsed undefined status. Record commands + output; no implementation test is credited before its required red evidence.
6. The suite asserts family counts, bounds + exclusions before claiming exhaustive completion. Batch/process layout may optimize transport only; every declared case remains compared and gets a stable id for failure replay.

## Freeze question — local arithmetic outputs

`semantics.md` Meaning says a NAF fails when any local valuation witnesses its whole conjunction. Its U sentence names closure through head terms, but a NAF can generate an output that appears in no head/source term:

```prolog
r(a) :- \+ guideline_quotient(exactly,1,exactly,2,S,N).
```

The witness `S=exactly,N=rational(1,2)` must make r(a) false. Proposed conservative grounding closure includes arithmetic outputs inside NAF, not only head terms. Main must affirm/amend this point at freeze. Enumeration counts do not depend on the answer; correct finite grounding does.

## Coverage limits

Excluded: >2 carrier values; arbitrary catalogues outside these templates; >3 non-fact clauses; larger conjunctions; nested NAF; >4 variables; source compound shapes outside the admitted grammar; constructor/arithmetic growth cycles; non-stratified candidates except NC; multiple-step arithmetic chains; built-in condition comparator `eq` (plain-number conditions stay structural); public certificate transport, support-level validation + question serialization. Fixed finite grids do not establish correctness over unbounded numeric widths. General soundness + admission proofs remain MC/TA's obligations.
