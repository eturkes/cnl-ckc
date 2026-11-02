# SB1 D9 proof spike

Standalone Verus source crate; entry = `lib.rs`. Scope = contract `sb` § Census + spike freeze, D9(a)–(d). Abstract finite domains; no production parser, checker or producer code imported.

## Run

From the repository root:

```sh
just tools
bash prototype/sb1/spike/verify.sh
```

`CKC_TOOLCHAIN` relocates the pinned tool store. Direct Verus loads the release's preverified `vstd`; Cargo's cold-vstd warm-up is unnecessary on this path. The script checks `rust/verus.lock`, runs the entire crate with `--no-cheating --rlimit 10 --num-threads 1`, requires selector-free success + zero warnings, and writes raw time/resource JSON + diagnostics to ignored `target/`. `10` = the verifier's default resource limit; no lemma overrides it. Reported SMT rlimit counts are work, not timeout settings.

`evidence/verify.json` = one committed full-crate run. Its times are verifier diagnostics on a shared host, not SB performance measurements. Replay regenerates the checks, not identical timing bytes.

## Claim map

| frozen statement | primary theorem | hypotheses |
| --- | --- | --- |
| (a) support + closure → reduct least fixed point | `support::support_closure_lfp` | Well-formed finite ground program; M inside its atom universe; every M fact has retained support with strictly lower positive-body levels; closure over every retained rule. |
| (b) signed graph → rank → local stratification → exactly one stable model | `rank::signed_graph_rank`, `origins::clause_covered_unique_model` | Finite well-formed graph; no negative edge with a return path; each ground rule has an abstract clause origin; every positive/NAF dependency reaches **every** possible producer. |
| (c) coverage-closed slice → reduct/model + query preservation | `origins::clause_coverage_slice`, `slice::reduct_projection`, `slice::query_preservation` | Abstract predecessor closure; query roots include all producers; selected operator atoms + every ground query's positive/NAF atoms lie in the induced slice. |
| (d) no growth cycle → finite substitution-closed ground instances | `grounding::substitution_closed`, `finite_grounding`, `nested_identity_substitution` | Finite base + operation templates + type graph; every constructor/arithmetic input is a growth dependency; positive copy edges; no growth cycle; well-typed clause templates whose Deps are variable lists. |

### (a) Semantics

`Instance` = head + positive atom set + NAF items + ground built-in truth. Each NAF item lists its candidate valuations; one valuation is a whole conjunction's atom set. The item passes exactly when **no whole witness** is included in M. Shared variables have already been correlated by grounding; splitting their witnesses is not permitted. An empty witness defeats NAF; an empty candidate list satisfies it. Ground built-ins are immutable booleans in the frozen spike.

`lfp(P, M)` = intersection of every model of the positive reduct. `support::lfp_fixed` proves that this intersection is a fixed point of the immediate-consequence operator. The support theorem proves equality, not only modelhood or a pre-fixed point.

### (b) Coverage + uniqueness

Edges run head/reader → dependency/producer. Reachability = least forward-closed set containing a node. Rank = reachable-node cardinality; positive SCCs may keep one rank, negative edges strictly descend. Clause-origin coverage matches the candidate's clause nodes: a ground atom may have several producers; its rank is the maximum rank of their nodes (zero if producerless). There is no one-producer assumption.

`stratified::perfect` constructs the model by a positive least fixed point at each stratum. `perfect_stable` proves existence; rank induction + reduct minimality prove uniqueness. The stable predicate is exactly (a)'s reduct fixed-point predicate. A single atom→node coverage specialization also verifies in `rank.rs`.

The frozen (b) conclusion is **unique stable model**. An independent three-valued / alternating-fixpoint definition of WFM and its equivalence to this model are outside this spike; no WFM-equivalence theorem is claimed here.

### (c) Queries

Masking a rule's built-in truth to false deletes it semantically while keeping clause ids stable. The slice includes all positive dependencies + every atom of every NAF witness. For arbitrary M, masking and restricting the reduct commute with its least fixed point. Stable models project as a corollary.

q-all = enumeration of designated operator atoms; q-one = the finite table of ground goal-list valuations. q-all's payload lists use that same goal-list theorem. Answer-id equality preserves subsequent projection/dedup; the spike does not encode output-byte serialization or query parsing.

### (d) Grounding

`Value::Identity(symbol, deps)` permits nested identities. A symbol encodes the fixed `(Kind, Doc, Sentence, Slot)` fields. Program templates contain constants, variables, or operation applications whose Deps are variable indices; substitution preserves their order + repetitions.

The may-universe is a finite base repeatedly unioned with finite operation images. Its height comes from the finite type-flow graph alone; neither the proposed model nor a producer-supplied universe is an input. Non-growing copies preserve/increase the bound. Every constructor/arithmetic output strictly increases its dependency rank. `eval` is an arbitrary deterministic function, so finiteness does not assume bounded integers, rational widths, injectivity or identity-only outputs. A partial operation can use any total completion; this only enlarges the overapproximation.

`finite_grounding` proves the full semantic comprehension of typed ground clause instances equal to a constructed finite set, plus closure of every instantiated term. `nested_identity_substitution` specializes the operation image to actual nested identity constructors.

## Controls

`controls.rs` verifies: correlated NAF with incompatible witnesses; empty witness/list meanings; independently necessary support + closure; a rejected one-node growth loop; an inhabited two-constructor chain that generates a nested identity and a legal ground clause instance.

## Production obligations outside the spike

- Real canonical parser/codec, exact term/patient grammars, interning identity + collision handling.
- Safety/shape admission → complete finite ground instances; real clause/type/operation extraction; source literals + patient constants included in the base.
- The may-feature fixpoint's own soundness, matching/unification overapproximation, variable-headed producers, correlations, NAF auxiliary construction + whole-statement guard provenance.
- Concrete clause-origin edge coverage, growth-edge completeness for every real Deps/arithmetic position, SCC cycle detection + rank computation.
- Concrete reachable/support/reduct facts mapped into the program-derived universe; executable indices, joins/anti-joins, levels, canonicalization + resource limits refining these abstract predicates.
- Query grounding + valuation completeness, root seeding + slice extraction, payload traversal, projection/sort/dedup, digests + output serialization.
- Ground built-in truth → the verified schema's exact arithmetic/comparison relations; partial/undefined operation handling.
- WFM equivalence against an independently defined well-founded semantics; certificate/custody/public-surface composition.

All remain obligations of MC/TA/MP. This spike supplies no public verified-answer claim and changes no production gate.
