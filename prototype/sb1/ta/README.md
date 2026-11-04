# SB1 typed-analysis census

Prototype for contract `sb` D8. No verified patient-answer claim.

## Admission

`Analysis::load` + `ta` use `k1::Refined` when the base origin graph cycles.
Immediate-Deps-origin types split cyclic base SCCs, validate every retained
edge's projection, and fall back to full k=1 on a surviving cycle. `build_full`
is the reference. A k=1 cycle projects to a base cycle, so acyclic base
components inherit their existing rank. Restricted k1 traces flow only from
cyclic base-SCC source nodes; the may-feature model still includes every rule.
Full k1 keeps every flow row as a reference. Negative clause SCCs still refuse.

`examples/k1-profile.rs` exercises growth alone; `examples/graph-profile.rs`
records the clause phase. A query slice inherits growth admission from the
checked superset: removing rules + replacing patient-top by facts cannot add
flow. The concrete slice's negative graph is checked separately. The retained
`provider::Prepared` seam is pending.

Clause bodies memoize producer sets under an exact alpha-normalized key;
clause nodes remain separate. Variable-source memoization likewise preserves
clause-local arithmetic outputs. Selective unary checks filter narrowed origin
sets; coarse concrete scalar domains retain full posting intersection.

## Scale boundary

The strict-isolation census at ×100 admits. The shared-role census at ×100
is memory-censored in may-feature closure, before graph
construction. Context-qualified owner links + link-consistent generic closure
are the pending refinement; this abstraction remains its reference. The memory
cap stays unchanged. Per-call typed selection also needs a prepared index;
the current full-program refinement is a census, not that implementation.

## Run

From this directory, with the pinned project toolchain:

```sh
cargo build --release --locked
target/release/ta --root <repo> --composition <composition.tsv> --out <metrics-dir> \
  --dc --query <repo>/bench/sb/queries/q-one.pl.in \
  --query <repo>/bench/sb/queries/q-all.txt \
  --query <repo>/bench/sb/queries/q-patient.pl.in --slice-dir <slice-dir>
cargo test -p sb1-ta --lib --locked
```

`--patient last` (default) = the suffix counted by a neighboring generator
manifest's `patient_documents` or an emitted `# patient_documents` line; absent
metadata = one final document. `--patient none` = knowledge base alone. Every
patient document must contain facts; conflicting/out-of-range counts refuse.
The census emitter preserves the suffix count for replay.
`--dc` transforms statement guards in memory. `--emit <dir>` writes that
composition. Corpus files stay unchanged. Slice emission keeps per-document
headers, declarations, metadata + selected sentence comments. Output paths are
absolute; choose an output directory distinct from the inputs.

Exit: 0 admitted; 1 negative/growth SCC; 2 input, grammar or argument failure.
`analysis.tsv`, `phases.tsv`, `transform.tsv`, `witnesses.tsv` = load census.
`--edges` also writes the full refined origin-flow table. `slices.tsv` +
query-specific tables = demand census. Each `<query>-ids.txt` lists original
knowledge clause ids (`document:ordinal`, one per line); patient facts stay
separate. This is the checker's `Provider::Ids` seam, not an incremental typed
provider. A refusal writes witnesses before exit and stops query execution.

`python3 -I census controls …` runs CDC + consumer worlds + CDC/plants.
`python3 -I census endpoints …` runs existing frozen ×100 compositions.
`python3 -I slice-check …` compares whole/slice producer + checker answer bytes.
Each helper's `--help` lists its paths. Caller holds the shared SB1 timing lock
and a `heavy` slot. These helpers preserve command vectors, rc + raw output.

## Feature representation

- Reuse `sb1-checker::{program,term,query}`. The checker delta exposes these three
  modules only. Safety + source-term admission remain the checker's.
- Origin = `(Kind,Doc,Sentence,Slot)`. Deps collapse into that origin. Opaque
  symbols, integers + structural argument roles retain exact values; role 1
  never matches role 2. Unknown numeric values have a separate abstraction.
- Per-origin features = nouns, event lemmas, box operators, properties + context
  payload predicates. Argument/preposition labels map a source origin to a
  target-origin domain. No abstract whole-program relation tuples or Cartesian
  antecedent environments are materialized.
- Load features start from every fixed head + fact. Variable-headed producers
  propagate features to every origin permitted by their full positive-body
  unary constraints. Monotone closure includes every rule's possible feature
  contributions; NAF + built-in truth are over-approximated.
- A call pattern holds the whole positive conjunction's features + links. For
  a NAF goal, the NAF item's entire conjunction joins those outer constraints.
  A proposed edge first unifies that goal with the candidate clause's head,
  then propagates all constraints. A finite-domain search finds one joint
  feature witness; missing precision stays a may-match. Classical `-` remains
  a positive operator, distinct from NAF.
- Negative analysis: nodes = individual clauses; edges = consumer → possible
  direct producer, signed positive/NAF. A negative SCC refuses.
- Growth analysis: nodes = Skolem origin templates + numeric arithmetic outputs.
  An edge points from a constructor/output to an origin that its Deps/input can
  read in a whole-conjunction instance. Every source comes from a compatible
  producer head; forwarded variables close transitively. Unary origin domains
  alone cannot provide a source: they omit the actual/boxed location. Bound
  labels stay in a fixed finite alphabet; numeric outputs carry arithmetic flow.
  An origin-flow cycle refuses. Unrefined nested-body patterns conservatively
  expose every origin, counted by `precision_fallbacks`.
- Each refusal keeps a shortest cycle. Witnesses give clauses, origin templates,
  every goal of the correlated conjunction + selected feature/link bindings.
  `growth-edges.tsv` keeps the full origin graph.
- Generic patient constants have every feature. Their links stay within the
  patient/atomic domain, distinct from KB Skolem origins in census worlds.
  A general patient/KB identity-collision policy remains a production obligation.
- Query roots use full-conjunction patterns; q-all also roots every payload
  predicate of each possible should/must/may context. Typed transitive closure
  gives a candidate slice. Its concrete-patient feature closure replaces the
  generic top; a second typed reachability pass selects the final slice. NAF
  dependencies, selected document metadata + the patient facts remain.

## Scope + counters

This algorithm is unverified. Production owes parser/abstraction soundness,
closed-feature coverage, correlation + substitution proofs, growth adequacy and
slice refinement. A cycle in the may-graph need not compose into a ground cycle:
Deps collapse can mix instances of one origin. A surviving CDC cycle follows the
contract's refinement/ruling path; no unchecked exception admits it.

The tuple experiment is retired in git history; its measurements + false
witnesses remain in the builder report. Collapsing all integers there also
collapsed argument roles 1/2, allowing a subject link to masquerade as an object
link. The exact-role regression pins this boundary in the feature algorithm.

`features` = unary/link incidences, not model facts. `feature_insertions` counts
new incidences; `writer_insertions` counts successful feature write operations;
`candidate_pairs` = forced goal/head probes; `worklist_steps` = feature-producing
rule evaluations; legacy `join_rows` = constraint-reduction iterations.
`load-analysis` excludes parsing; `phases.tsv` reports parse/transform separately.
RSS = this process's Linux `VmHWM`, not summed producer/checker RSS. The thin-path
harness owns the latter. No census time is a D6 confirmation sample by itself.

Edges retain target + sign/kind flags; the source is their adjacency row. Goals,
NAF locations + growth conjunctions reconstruct only for cycle witnesses or an
explicit edge dump. Flow incidences + variable-source sets preserve every
supporting site; k1 accepts an edge when any site has the required source tag.
Parser spelling/anonymous/valuation caches retire after admission (variable
counts + original text remain). K1 releases rendered IR text capacity; temporary
constraint, forwarding and body caches retire before SCC allocation.

`SB1_TA_MEMORY=<new-file.tsv>` records phase RSS/HWM + structure sizes. Vector
rows report allocated capacity; map/tree rows report payload lower bounds.
Allocator metadata, tree slack, hash control bytes + symbol tables remain
unattributed: these rows are not a complete heap census or a substitute for RSS.
Transient growth/SCC rows precede cache release. Phase/transform tables publish
incrementally, so a censored run retains every completed phase. Memory tracing
is diagnostic instrumentation, not the confirmation configuration.
