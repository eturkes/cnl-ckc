# Admitted-composition semantics (oracle input; contract `sb` D5-2)

The independent finite-program oracle implements this text alone: no producer, checker, kernel or `guidelines/schema.pl` code. Normative sources it may read: this file, `.agent/spec.md` Decisions Patient answers, `docs/REFERENCE.md` § Comparison conditions + § Arithmetic built-ins (the built-in relations).

## Syntax

Four term domains, each admitted only where named:
- Source term (clauses of the loaded documents): an atom, an integer, a canonical rational `rational(P, Q)` (Q ≥ 2, gcd(|P|, Q) = 1), a source identity `'$guideline_id'(Kind, Doc, S, Slot, Deps)` (Kind, Doc atoms; S an integer; Slot = `ref(N)` | `box(N)`; Deps = a list of variables in a rule head or body, `[]` in a fact), or a variable.
- Ground term (model facts, valuations): the same without variables, except that an identity's Deps is a list of ground terms (nested identities included): `'$guideline_id'(product, d, 1, ref(2), ['$guideline_id'(product, p, 1, ref(1), [])])`.
- Metadata (ground, position-fixed): `guideline_document(DocId, ace_sha256(H), ulex(none | sha256(U)), vocabulary(none | sha256(T)))` and `guideline_supersession(DocId, S, source(Citation, Url), Date)` carry these compounds at these argument positions alone; they are facts, members of every model, and certified as facts (level 0).
- Patient fact: a ground schema literal of a compiled patient document (atoms, numbers, identities with Deps `[]`).
- Literal: a schema-predicate atom `p(T1, …, Tn)` (the 22 indicators of REFERENCE § Compiled Prolog schema); a built-in `guideline_satisfies/4`, `guideline_satisfies/6`, `guideline_product/6`, `guideline_quotient/6`; a negation `\+ (G1, …, Gm)`, m ≥ 1, each Gi a schema literal or a built-in.
- Clause: `H :- B1, …, Bn` (n ≥ 0; a fact when n = 0), H a schema literal. Safety, left to right: every head variable occurs in a positive body literal or a built-in output; every built-in input and every variable a negation shares with the clause outside it occurs in an earlier positive literal or built-in output. A variable that occurs only inside one negation is local to it.
- Composition P = the clauses of the loaded documents + the patient's facts.

## Meaning

- Valuation of a clause C: θ maps every non-local variable of C to a ground term. Ground instance Cθ.
- Interpretation M: a set of ground schema literals.
- Under M, for a ground instance: a schema literal L holds iff L ∈ M; a built-in holds iff its relation holds for its ground arguments (REFERENCE § Comparison conditions: value-set inclusion; § Arithmetic built-ins: the outputs = the image the section defines, no image ⇒ fails); a negation `\+ (G1, …, Gm)` holds iff no valuation σ of its local variables makes every Giσ hold under M (the whole conjunction, correlated through shared locals).
- Reduct step: T_M(I) = { Hθ | Cθ a ground instance; every positive schema body literal of Cθ ∈ I; every built-in of Cθ holds; every negation of Cθ holds under M }.
- M is a stable model of P iff M = lfp(T_M) (the least fixpoint from ∅).
- An admitted composition (the typed analysis: no negative cycle, no growth cycle) is locally stratified, so it has exactly one stable model, equal to its two-valued well-founded model. The oracle computes the well-founded model of the finite ground program (alternating fixpoint) and reports `undefined` literals if any; for an admitted program the expected count is 0, and the model = the unique stable model. A program outside admission (a negative cycle) may read undefined literals; such a case is a refusal control, never an accepted positive.
- Grounding universe U = the least set of ground terms that holds every ground term of P and is closed under the admitted derivations: for a clause and a valuation into U under which its body holds, the head's terms (fresh identity instances with ground Deps, built-in outputs) belong to U. The typed analysis (no growth cycle) makes U finite. The enumeration manifest admits a program only when U is finite within its stated bounds; a case that would pass a bound is excluded from the domain, never truncated into a smaller apparently complete model.

## Answers

- q-all: the set { (C, O) | `guideline_operator(actual, C, O)` ∈ M, O ∈ {should, must, may} }, each with C's payload = every literal of M whose first argument is C.
- A certificate claims, per fact F ∈ M, a natural-number level and one ground instance Cθ with head F, every built-in holding, every negation holding under M, every positive body literal G ∈ M with level(G) < level(F) (facts of P: level 0; an instance without positive body literals: any level ≥ 1). A checker accepts iff every claim holds and M is closed (T_M(M) ⊆ M); it never requires minimal levels. Accepted ⇒ M = lfp(T_M) (the stable model). Several valid certificates can exist for one M (alternate founded supports): all are accepted.
