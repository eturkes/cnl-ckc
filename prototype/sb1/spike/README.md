# SB1 D9 proof spike

Standalone Verus source crate; entry = `lib.rs`. Scope = contract `sb` § Census + spike freeze, D9(a)–(f), + row TAS's seam (g)–(k) (`.scratch/imp/tas-check.md` T8). Abstract finite domains; no production parser, checker or producer code imported.

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
| (e) unique stable model = two-valued alternating-fixpoint WFM | `afp::stratified_wfm`, `covered_wfm`, `certified_is_wfm` | Well-formed finite ground program; (b)'s local stratification. AFP definition + finite iteration are proved independently of rank. |
| (f) actual clause selection → atom-slice reduct + query equivalence | `selection::clause_selection_reduct`, `selected_reduct_queries`, `selected_wfm_queries` | Forward-closed selected clause nodes + all-producer coverage; query roots + positive/NAF reads inside the induced slice. WFM query equivalence additionally uses (b)'s no-negative-cycle rank. |
| (g) envelope pruning (TA-T0) → equal WFM + stable models; coverage of the pruned program suffices | `envelope::pruned_wfm`, `pruned_covered_wfm`, `pruned_certified_is_wfm` | Well-formed finite ground program; L closed under every rule whose built-ins hold, NAF dropped (lfp(P⁺) or any post-fixpoint); signed coverage + no negative cycle on `prune(p, l)` alone. |
| (h) closed source hint → every concrete growth dependency is an enumerated refined edge | `flow::hint_sound`, `growth_covered` | Compositional typing (base, k = 1, per-symbol mix); one binder per head variable; may-match soundness over derivable instances; the hint closed over every may-producer of each binder; flat bodies (constants + variables). |
| (i) pruned program's covered graph + forward-closed choice → the SOURCE program's answers | `envelope::pruned_selected_wfm_queries`, `select_prune` | (g)'s envelope; coverage + no negative cycle on `prune(p, l)`; forward-closed choice; operators + every goal's reads inside the pruned program's atom slice. |
| (j) executable CSR reachability → forward-closed selection → (i) | `csr::reach` (exec), `represents_closed`, `csr_selected_wfm_queries` | CSR well-formed; every graph edge stored (`represents`, the checker's build obligation); roots below the node count. |
| (k) a NAF-only extension's coverage covers the original (TAS: the DC-shaped program certifies the original clauses) | `envelope::naf_extension_wfm`, `naf_extension_coverage`, `naf_extension_slice` | `naf_extends(p, q)`: same atoms, heads, positive bodies + built-ins, every NAF item of p kept in q; q's envelope; coverage + no negative cycle on `prune(q, l)`. |

### (a) Semantics

`Instance` = head + positive atom set + NAF items + ground built-in truth. Each NAF item lists its candidate valuations; one valuation is a whole conjunction's atom set. The item passes exactly when **no whole witness** is included in M. Shared variables have already been correlated by grounding; splitting their witnesses is not permitted. An empty witness defeats NAF; an empty candidate list satisfies it. Ground built-ins are immutable booleans in the frozen spike.

`lfp(P, M)` = intersection of every model of the positive reduct. `support::lfp_fixed` proves that this intersection is a fixed point of the immediate-consequence operator. The support theorem proves equality, not only modelhood or a pre-fixed point.

### (b) Coverage + uniqueness

Edges run head/reader → dependency/producer. Reachability = least forward-closed set containing a node. Rank = reachable-node cardinality; positive SCCs may keep one rank, negative edges strictly descend. Clause-origin coverage matches the candidate's clause nodes: a ground atom may have several producers; its rank is the maximum rank of their nodes (zero if producerless). There is no one-producer assumption.

`stratified::perfect` constructs the model by a positive least fixed point at each stratum. `perfect_stable` proves existence; rank induction + reduct minimality prove uniqueness. The stable predicate is exactly (a)'s reduct fixed-point predicate. A single atom→node coverage specialization also verifies in `rank.rs`.

The frozen (b) conclusion is **unique stable model**. Lemma (e) independently defines alternating-fixpoint WFM and proves its equality to this model.

### (c) Queries

Masking a rule's built-in truth to false deletes it semantically while keeping clause ids stable. The slice includes all positive dependencies + every atom of every NAF witness. For arbitrary M, masking and restricting the reduct commute with its least fixed point. Stable models project as a corollary.

q-all = enumeration of designated operator atoms; q-one = the finite table of ground goal-list valuations. q-all's payload lists use that same goal-list theorem. Answer-id equality preserves subsequent projection/dedup; the spike does not encode output-byte serialization or query parsing.

### (d) Grounding

`Value::Identity(symbol, deps)` permits nested identities. A symbol encodes the fixed `(Kind, Doc, Sentence, Slot)` fields. Program templates contain constants, variables, or operation applications whose Deps are variable indices; substitution preserves their order + repetitions.

The may-universe is a finite base repeatedly unioned with finite operation images. Its height comes from the finite type-flow graph alone; neither the proposed model nor a producer-supplied universe is an input. Non-growing copies preserve/increase the bound. Every constructor/arithmetic output strictly increases its dependency rank. `eval` is an arbitrary deterministic function, so finiteness does not assume bounded integers, rational widths, injectivity or identity-only outputs. A partial operation can use any total completion; this only enlarges the overapproximation.

`finite_grounding` proves the full semantic comprehension of typed ground clause instances equal to a constructed finite set, plus closure of every instantiated term. `nested_identity_substitution` specializes the operation image to actual nested identity constructors.

### (e) Alternating-fixpoint WFM

`Γ(M) = lfp(P, M)` uses exactly (a)'s reduct. `afp::lower` independently defines `lfp(Γ²)` as the intersection of all pre-fixed points; `upper = Γ(lower)`. True, false + undefined sets = lower, `Atoms − upper`, `upper − lower`. Rank, perfect model + certificate are absent from this definition.

`gamma_antitone` + `double_monotone` justify the alternating operator. `lower_fixed` + `lower_least` prove its least fixed point. `finite_alternation` proves equality to iteration from the empty set after at most `|Atoms|` double steps. Local stratification excludes distinct alternating pairs, which is stronger than stable-model uniqueness alone. `stratified_wfm` concludes lower = upper = (b)'s perfect model + no undefined atoms. `certified_is_wfm` composes (a) with that theorem.

### (f) Actual clause selection

`selection::select` filters source clause indices by their chosen abstract origins; it returns an actual clause subsequence, with duplicates + source order preserved. `selected_source_index` constructs a source-index witness for any selected output clause; `selection_exact` proves inclusion of every chosen source clause.

The forward-closed selection induces `origins::atom_slice`: all producers of each retained atom are chosen. This slice satisfies (c)'s predecessor-closure premise in both the original + selected programs. Their atom-induced masks have identical effective rules. `clause_selection_reduct` concludes that the selected program's reduct lfp, restricted to the slice, equals (c)'s original-program mask for arbitrary reduct reference M. Extra selected heads outside the slice may still derive; global equality with the mask is not claimed.

`selected_reduct_queries` proves both enumeration + goal-list answer equalities. `selected_stable_agrees` composes the bridge with (b)'s local-stratification uniqueness; `selected_wfm_queries` composes it with (e), so the full + actual selected programs return identical query answers over their independently defined WFM.

### (g) Envelope pruning

`envelope(p, l)` = closure of L under every rule whose built-ins hold, every NAF dropped; leastness is never used, so a checked post-fixpoint (a hinted feature model's concretization) qualifies. `prune` keeps a live instance (built-ins true, positive body inside L) with each NAF item's witnesses cut to those inside L; a dead instance keeps its head alone, never retained, so it needs no edge. Indices stay aligned with the source program.

`envelope_bounds`: Γ_P(I) ⊆ L for every reference set I. `pruned_gamma`: Γ_P(I) = Γ_{P_L}(I) for I ⊆ L only (outside L the equality fails: `p :- not q` at I = {q}). The alternating iterates from ∅ stay inside L, so `pruned_wfm` gives equal lower/upper sets and the same stable models. `pruned_covered_wfm` composes (b) + (e) over the pruned program; `pruned_certified_is_wfm` = a support + closure certificate over the SOURCE program + the pruned program's covering graph ⇒ the model = WFM(source).

### (h) Forwarding + k1

`flow.rs` adds argument structure: flat body goals, heads that may build `Identity(symbol, Deps)` from variables; derivation by depth (no finite universe assumed). A compositional typing gives an identity's type from its symbol + its immediate arguments' types: base origins, k = 1 (the immediate Deps' base origins) and any per-symbol mix are instances, so a restricted refinement needs no separate rank-inheritance argument. The untrusted hint = per rule variable a type set + one binder (a body goal position holding the variable). `closed_hint` checks every may-producer of each binder: a constant's type, a forwarded variable's hinted types, every hinted tuple of a constructor — after a consumer-pattern filter `fits` whose soundness (`fits_sound`: every derivable instance's value fits) is the feature model's obligation; `no_filter` = the unfiltered closure. `hint_sound` (induction on depth) ⇒ every head variable's value in every derivable instance has a hinted type; `growth_covered` ⇒ each concrete Deps dependency (constructor output → one input) is an edge the checker enumerates over every hinted tuple.

### (i) Pruned selection

`select_prune`: clause selection commutes with pruning (`filter_index_map`: both keep source indices aligned). `pruned_selected_wfm_queries` applies (f) to the pruned program, then TA-T0 to the source program and to the selected source clauses (`select_envelope`: a clause subset keeps the envelope), so the selected source clauses answer as the whole source program.

### (j) Executable CSR reachability

`csr::reach` = exec code over `Vec<usize>` offsets/targets + a `Vec<bool>` mark: worklist reachability from the roots, each node pushed once (marked first), termination by unmarked count + stack length. Postconditions: roots marked, mark CSR-closed. `represents_closed` lifts a CSR-closed mark to `rank::forward_closed` for any graph whose every edge the CSR stores; `csr_selected_wfm_queries` composes it with (i).

### (k) NAF-only extension

The census's DC shape appends `\+ guideline_exclusion(Key)` goals alone, so its program extends the original by NAF items. Both share the positive program (`naf_extension_envelope`), every kept witness of the original is a kept witness of the extension (`naf_extension_coverage`), and the pruned atom slices coincide (`naf_extension_slice`): a graph certifying the extension certifies the original's WFM and its forward-closed selections.

## Controls

`controls.rs` verifies: correlated NAF with incompatible witnesses; empty witness/list meanings; independently necessary support + closure; a rejected one-node growth loop; an inhabited two-constructor chain that generates a nested identity and a legal ground clause instance; total WFM for stratified NAF; genuinely undefined WFM for an odd negative self-loop; nonempty enumeration + NAF goal-list answers under proper clause selection, despite an extra selected fact outside the atom slice making global mask/model equality false. `inactive_negative_control`: a rule whose positive body has no producer and whose NAF loops on its own head admits through the pruned program (two-valued WFM), while no graph without a negative cycle covers the unpruned program; `active_negative_control`: its live twin forces a negative self-loop in every envelope's pruned program and leaves the atom undefined in the AFP WFM, so refusal is necessary. `k1_base_refuses`: every closed base-origin hint of a forwarding program forces the growth cycle P → O → P; `k1_forwarding_control`: the k = 1 hint of the same program instantiates every `growth_covered` premise, its refined edges descend a rank, and every concrete Deps dependency descends it. `csr_control`: a two-node CSR satisfying every premise of `represents_closed` + `reach`'s precondition. `naf_extension_control`: a shaped twin whose added guard keeps its witness (a real negative edge) certifies the original program's two-valued WFM.

## Production obligations outside the spike

- Real canonical parser/codec, exact term/patient grammars, interning identity + collision handling.
- Safety/shape admission → complete finite ground instances; real clause/type/operation extraction; source literals + patient constants included in the base.
- The may-feature fixpoint's own soundness (its concretization = an `envelope`), matching/unification overapproximation (`flow::may_sound`), variable-headed producers, correlations, NAF auxiliary construction + whole-statement guard provenance.
- Concrete clause-origin edge coverage of the pruned program, growth-edge completeness for arithmetic outputs + nested body patterns (`flow.rs` covers Deps constructors over flat bodies), the patient extension (a KB checked with patient-top covers every admitted finite patient extension), rank checks.
- Concrete reachable/support/reduct facts mapped into the program-derived universe; executable indices, joins/anti-joins, levels, canonicalization + resource limits refining these abstract predicates.
- Query grounding + valuation completeness, root seeding + slice extraction, payload traversal, projection/sort/dedup, digests + output serialization.
- Ground built-in truth → the verified schema's exact arithmetic/comparison relations; partial/undefined operation handling.
- Certificate/custody/public-surface composition.

All remain obligations of MC/TA/MP. This spike supplies no public verified-answer claim and changes no production gate.
