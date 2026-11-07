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

The default may-model qualifies argument/preposition links by their fact's
owner context and closes generic rule bodies with those links. `--reference`
selects the preceding owner-forgotten, unary-only generic closure (diagnostic
reference, not a soundness oracle). `--compare-reference` checks stored unary
sets + owner-forgotten link sets against that reference; `--features-only`
ends after closure without an admission verdict. Cost/endpoint comparisons
use immutable SHA-named binaries and the same 28 GiB cap, swap disabled.

Patient/top referents retain KB-derived stored links as well as implicit
patient links. Each patient document is fact-only; its terms/metadata must
avoid every KB identity/docid namespace. Admission refuses a collision with
its source clause. This is an explicit input rule, not an assumed disjointness.
Per-call typed selection still needs a prepared index; the current full-program
refinement is a census, not that implementation.

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
The census emitter preserves the suffix count for replay. It writes
`manifest.tsv` (`sb1-ta-composition 1`): actual emitted document/clause counts
+ patient suffix count, without inventing generator provenance. This also lets
the benchmark harness hash a full or sliced emitted composition.
`--dc` transforms statement guards in memory. `--emit <dir>` writes that
composition. `--shape-only --emit <dir>` emits after source admission + shaping,
without the typed analysis (explicitly no admission verdict); this separates
the DC thin-path cost experiment from analysis completion. Corpus files stay unchanged. Slice emission keeps per-document
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
Each helper's `--help` lists its paths. Workstation timing callers hold the
shared SB1 timing lock + `heavy`; construction census runs can use the pool.
These helpers preserve command vectors, rc + raw output.

`probe.py --ta <SHA-named executable> --root <repo> --composition <file>
--out <new-dir>` runs one capped construction census (28 GiB, swap 0), with
input identities + a full-lifetime RSS trace. `--receipt-dir <dir>
--ta-source-commit <full-SHA> --receipt-host pool|workstation` publishes
`<kb-identity>.tsv`, format `sb1-ta-receipt 1`, after a refined full analysis.
Reference/partial runs emit no receipt. `--redesign` selects the complete exact
candidate; its private experimental receipts keep the command + source/ELF pins.
Canonical receipt provenance changes only at MAIN's redesign-merge ruling.
`--service-unit <name>` uses an
existing detached user service instead of creating a separate scope. The probe
reads its cgroup-v2 membership and requires that service's actual 28 GiB memory
cap + zero swap before starting TA. The detached unit survives an SSH drop;
reattach waits with `offload run --no-sync` and preserves its original inputs.
The KB identity frames the schema
and ordered non-patient document bytes under `dc=1`; `knowledge_bytes` counts
those document bytes alone (the schema is separate). Missing/censored measures
read `unavailable`, never an invented zero. A derived record has two extra
fields, `basis=renaming|subset` + `basis_identity`. Its basis must be admitted;
its target identity/inventory are computed from target bytes; measured costs
and TA provenance remain those of the basis, explicitly not a target run.
`python3 -I probe-tests.py` grades
the receipt structure, refusal/censor states + missing-evidence controls.

## Exact-sharing census

`--census` adds `census-*.tsv`; the explicit oracle's transfer, graph + refusal
semantics stay unchanged. Run the frozen development matrix with a sealed binary:

```sh
python3 -I structure-census --ta <absolute-SHA-named-ta> --root <frozen-repo> \
  --instances <absolute-frozen-package-dir> --out <new-absolute-dir>
```

The matrix = F1a/F1b/F2/F3/F4/MX × G1/G2/G4/G10, seed 0, DC shape, no query.
`commands.tsv` binds argv + binary/composition hashes; `case-status.tsv` keeps rc
+ verdict; `counts.tsv` + `slopes.tsv` aggregate every metric + G1→G2, G2→G4,
G4→G10 + G1→G10 log-log slopes. Bands ≤1.2 / ≥1.7 describe that interval alone;
the middle is inconclusive. Read `smoke_only` + k from the frozen manifests:
mechanism-limited points cannot establish an endpoint scaling law. Walls include
instrumentation + shared-host noise: diagnostics, never confirmation samples.
`--run-manifest <file>` checks every framed composition identity + byte/document
count against it; `--inputs-only` runs that preflight alone. `--without-census`
measures a sealed uninstrumented oracle through the same runner.
`python3 -I census-compare --before <baseline> --after <work-census> --oracle
<oracle-G10> --instances <frozen-package> --out <new-dir>` requires every baseline
counter to reproduce and retains the six G10 instrumentation ratios + frozen
mechanism labels. Changed + missing counter controls fire before that check.

Set counters: `flat` = stored memberships, `distinct_members` = memberships after
whole-set interning. Atoms partition the element union by identical membership
signatures across every stored set. `atom_cost` = union size + stored set→atom
references; `intern_atom_cost` = union size + distinct set→atom references + one
reference per stored set. These count identifiers/arcs, not allocator bytes.
`unary` is the feature map; `families` is its wildcard-family index;
`unary_all` includes both. Link target rows keep owner/source/label keys distinct.

Natural link rectangles retain each head's latest constant-seed + solved-state
contributions as owner-set × source-set × label × target-set. Their union is
expanded and required to equal the complete stored link map, including top
sentinels. `rectangle_cost` = four descriptor references per distinct rectangle
+ distinct domain memberships; `rectangle_atom_cost` uses atom-factored domains.
This is an exact representation census, not a new solver. Intermediate versions
of growing domains are excluded from the final-structure cost.

`python3 -I census-check --oracle <sealed-822ffa10-ta> --ta <sealed-census-ta>
--root <frozen-repo> --composition <frozen-F1a-G1-composition> --out <new-dir>`
compares all nonmeasurement outputs on the eleven standalone plants + F1a G1.
It also compares every emitted document byte. Frozen plant comments are not
compiler statement groups, so only the generated F1a input takes `--dc`.

`probes.*.distinct` keeps the baseline's alpha-normalized positive/NAF-extended
pattern + distinguished goal/head keys. `full_distinct` also keeps the complete
base-state parent/domain content + the pattern's raw variable numbering; this
prevents an alpha-renamed pattern from aliasing differently placed bindings.
Exact constants, owner/role selectors, repeated variables + All remain distinct.
This conservative full key can miss sharing that jointly normalizing the pattern
and its binding vector would expose. It counts reusable Boolean forced checks,
not a memoized implementation.

Constructive work (`work.rs`; flag-gated): every may-rule solve records the
stored unary/family/link/scalar/owner sets it reads. A write invalidates that
resource's cached content ID; its next read hashes/interns the new version once.
Exact vector equality resolves hash collisions. `evaluations.input_memberships`
adds each resource's width once per evaluation; `distinct_input_memberships`
adds it once per distinct (alpha-body, complete resource/content-ID map) key.
These are input-width proxies for reuse, not bounds on a join solver's runtime.
State content IDs cache once per immutable state version; constraints + head
unification invalidate them. Candidate metrics retain requests, unique full
request keys, chosen-index posting memberships scanned + returned heads.

`census-work.tsv` separates may/graph/base-flow/k1 expansion/k1 may/k1 flow/filter
from posthoc census work. Member visits count explicit union, normalize, clone,
source/owner/forwarding/posting loops; `meet_operand_memberships` is operand
width, `meet_intersection_yields` is yielded matches, not hidden B-tree probes.
SCC visits cover its four whole-edge passes; cycle-witness searches remain
unmeasured. Peak counters cover reverse adjacency + SCC vectors, body/source/
allowed caches, growth snapshots/live root versions, retained constraint states
+ k1 registries/tags. Model tables count native head/scalar index postings and
current persistent set versions/memberships. Persistent row counts grow
monotonically here; final membership counts are not claimed as peak counts
(a finite target domain can become All). Telemetry's retained content versions
+ memo keys are separate from the oracle's
own storage; `Pool` keeps one vector in each of its value table + key map.
`versions.hashed*` counts content-interning requests/charged memberships, not
internal hash-table passes. Unmeasured: allocator/tree/hash slack, symbol/AST
internals, SCC DFS-stack peaks, witness BFS scratch, proof-state recursion peaks
and complete simultaneous process live bytes.
A G10 wrapper wall above roughly 2× the sealed oracle's is reported, not tuned
away by an instrumentation-performance campaign.

Graph tables count both complete signed adjacency rows + target rows split by
sign/growth flag. `hub_cost` = caller→hub associations + leaf target memberships;
`atom_hub_cost` = caller→hub associations + distinct row→atom arcs + atom→target
memberships. Associations count each (caller, signed target row), not distinct
callers; a caller with both signs keeps both. Component counts + doubled arcs
for forward/reverse SCC storage are explicit. Base + final k1 flow have separate
tables; variable-source + supporting-site incidence sets are counted separately.

A quadratic count is quadratic among the evaluated exact factorings (whole-set
interning, membership-signature atoms, latest head rectangles, exact whole-row
hubs + atom hubs). It is not a lower bound on every exact circuit: e.g. rows
S_i = U minus {i} need G² atom-row references but only O(G) prefix/suffix union
DAG nodes. Such union DAGs + stronger exact query equivalences stay unresolved.
C3's equality dumps + the redesign remain separate work.

## Exact front-stage candidate

`--redesign-front` selects the exact-representation candidate's may closure +
signed clause graph/SCC; `--features-only` stops after closure. The explicit
refined oracle remains separate. Neither front-only path produces a complete
admission verdict or receipt. `--skip-audit` re-evaluates each dirty-round skip
on its current inputs and asserts that the transfer adds nothing. Immutable
set IDs preserve raw-union versus normalized-meet boundaries. State copies
hold parent/set-ID arrays. Owner×source rectangles keep the oracle's marginal
Cartesian writes, including aliased roles; GAC still skips an All source.

Mutable unary writers freeze an interned snapshot only when read. Between
transfers, a bounded scratch charge triggers cache eviction and dense remapping
of model-reachable set IDs; closure completion always collects. No transient
State crosses that safe point. All current posting IDs and rectangle axes and
targets are roots. `front-stats.tsv` separates retained/live entries and bytes,
before/after the final collection. Bytes = finite-value payload + fixed
per-entry pointer/refcount payload; allocator/hash slack stays outside that
ratio. `allocation_charge` separately includes vector/map/cache capacities as
payload lower bounds, not a complete heap measurement. RSS owns the resource
claim.

`--header-audit` checks each native candidate recipe against the oracle's own
index, each small-header verdict against its cloned native prefix, and every
removed pair's failure. The normal path selects the frozen native posting,
point-checks compatibility in the other columns, then unifies goal-root aliases
+ fresh head variables. Survivors run the
full header → GAC → feature-witness recipe and return the GAC state.
`header_column_targets` counts materialized column results (zero on this path);
`header_point_checks` counts visited head-argument pairs. `state_cells_copied`
counts forced + pre-witness state copies alone; recursive witness scratch is
outside that counter.

The signed graph stores caller associations + exact target-set hubs. Sign and
growth flags live on caller associations alone. SCC runs on unsigned virtual
paths; each cyclic signed association expands through the oracle's original-edge
shortest-cycle rule for its witness. A positive graph never expands all original
edges for SCC. `graph_H` = caller associations + hub target memberships; reverse
SCC scratch is additional and stays in RSS.

`--dump-front` emits canonical value/unary/link tables + original clause nodes,
signed edges and original-clause SCC partition. `--dump-graph` emits the latter
three alone. Canonical values retain their abstract kind, so top `number` never
aliases the exact atom `number`. `--front-only` stops the explicit oracle after
its clause SCC stage, without claiming complete admission.

```sh
python3 -I exact-matrix --ta <sealed-ta> --root <frozen-repo> \
  --instances <frozen-package> --run-manifest <run-manifest.tsv> \
  --out <new-dir> --stage features
python3 -I retention-census --ta <sealed-ta> --root <frozen-repo> \
  --instances <frozen-package> --run-manifest <run-manifest.tsv> --out <new-dir>
```

Both runners preflight all twelve G1/G10 input identities. The comparator checks
missing and extra rows; the retention validator rejects an unreferenced byte.
These are untimed construction diagnostics, not campaign measurements.
`--stage graph|compressed` adds signed graph/SCC comparisons; `--oracle-cache`
reuses dumps only after exact framed input identities match (changed-identity
plant included). `permuted-front` reverses knowledge-file order, keeps the patient
suffix, and compares every front object against both oracle orderings.

`interim-front --ta <candidate> --oracle <sealed-refined-oracle> --root <repo>
--instances <package> --out <new-dir> --source-commit <SHA>` implements D4 I1.
The caller holds the SB1 timing lock + `heavy` for the whole window; `probe.py`
gives each child the fixed 28 GiB/swap-0 cap + 12 h censor. It records the paired
full F1b oracle, redesigned F1b/F1a fronts and a dump-emitting F1b oracle. The
pre-run rule file fixes s + both projection formulas and the 1,650 s line.
Front diagnostics cannot issue admission receipts. Wrapper wall includes dump
publication; the frozen front estimand remains may-fixpoint + graph/SCC.

## Exact complete candidate

`--redesign` runs the exact backend through base origin flow + restricted/full
k1 and admits only after both the signed graph + final growth graph admit.
`--redesign-front` stays the partial diagnostic. The explicit oracle remains
independent. Source rows are weak-interned immutable sets; snapshot dirty rounds
skip only unchanged forwarding inputs. `--skip-audit` also re-evaluates those
skips. Cached constraint states join the set collector's roots. K1 preserves
precise Deps tags, support filtering, base-edge projection + full fallback.

`--dump-analysis` emits canonical base/final origin nodes + flow edges and the
k1 outcome. Origin/tag labels keep their semantic identities, not registry ids.
Each requested query also emits `slice-<query>.tsv`: source path + original
clause ordinal + patient flag, sorted independently of file order. Concrete
slice selection uses the exact backend and inherits growth admission from its
checked superset, as the oracle does. It does not recompute diagnostic slice
growth counters. Refusals publish original-edge cycles in `witnesses.tsv`: source clauses,
whole-pattern features and supporting growth sites. `--edges` writes the active
base/refined growth provenance table. The candidate reconstructs these through
its own solver; the explicit oracle keeps its independent implementation.
Receipt provenance remains separate.

```sh
python3 -I complete-matrix --ta <sealed-ta> --root <frozen-repo> \
  --instances <frozen-package> --run-manifest <run-manifest.tsv> \
  --out <new-dir> --permuted
```

The matrix runs full oracle/candidate analyses + all three frozen queries on
G1/G10 × six families, then reversed-knowledge-order F1a G10. It compares every
front object, both flows, the k1 outcome + all slice clause sets by `cmp`; input
identity preflight, missing/extra comparator controls, commands, rc + hashes
stay in the output. `--oracle-cache` reuses only identity-bound full results.
These are capped construction diagnostics, not ×100 or pool budget passes.
`--dump-graph` writes original nodes/edges/SCCs alone; `--dump-front` additionally
flattens features. The graph-only path keeps full analysis/admission.

`interim-full --ta <sealed-ta> --root <frozen-repo> --instances <package>
--oracle-dumps <I1-oracle-dump/metrics> --out <new-dir> --source-commit <HEAD>`
implements D5 I2 after MAIN's GO-I2. Caller holds the timing lock + `heavy` for
the serial F1b ×100 → F1a ×100 window; each probe keeps 28 GiB/swap-0 + 12 h.
Complete wall = parse + may + graph/SCC + origin growth + k1. Both admit + both
walls ≤1,650 s + F1b graph dumps equal the identity-bound oracle ⇒ continue;
censor/OOM/refusal/missing evidence/dump mismatch ⇒ stop. A completed over-budget
F1b keeps F1a's measurement too. The pre-run rule, argv, rc, private receipts +
phase/memory fields remain beside the decision. `interim-full-tests.py` pins the
boundary, failure modes + byte comparator with synthetic controls, not timings.
I2 is a workstation diagnostic, never a C4t/C4m or production-admission grade.

## Exact candidate-request census

`--redesign --candidate-census` records every graph/base-growth/k1-growth
`candidates()` request under `candidate-census/index-*.tsv`. Query execution stays
separate. These diagnostics replay primitives; the returned candidate set stays
the original implementation's. Request rows keep chosen/raw bucket-union widths,
all argument-domain cardinalities (`all` = top), rejecting-column counts,
point/prefix/forced counts + times, candidate/full-context repeat ids and
same/cross-guideline telemetry. Unclassified sources stay explicit. Group paths
never enter a filter or repeat key.

Candidate keys = raw goal id + argument domain set ids + argument-root equality,
phase, NAF position + collection epoch. Raw goal terms own constants; key capture
adds no singleton set. The epoch prevents recycled set-id aliases. Full keys also
keep the ordered positive/NAF-extended pattern + whole parent/domain frame; equal
index arguments alone cannot skip survivor solving. Dictionaries preserve every
key + term, with no symbol abstraction or extra renaming equivalence.

Primitive replays: a sorted-posting heap merge equals the native bucket union;
phase-local composite postings keep `(chosen-bucket, other exact head value)`
rows. Query planning chooses the smallest posting-size bound among constrained
columns, then merges that row union. The replay must equal the original native
set intersected with the selected exact compatibility predicate; every old point
survivor remains. Build, planning/group, merge and remaining-point costs stay
separate. These are lever projections, not an adopted runtime mechanism.

```sh
python3 -I candidate-census --ta <sealed-census-ta> --root <frozen-repo> \
  --instances <package> --run-manifest <manifest> --out <new-dir> \
  --family F1a --scale 1 --baseline <sealed-I2-ta>
```

Caller holds the timing lock + the construction capsule. `--baseline` compares
all eleven load C3 objects. Without it, the matrix defaults to six families ×
G1/G2/G4/G10. The collector validates each row, key identity + independent model
work counters before aggregating. Raw calls, exact keys, groups, composite build
costs, argv, input identities, rc + phase tables remain in the output.

## Qualification

`control-matrix --ta <sealed-ta> --root <repo> --out <new-dir>` runs the sixteen
DC controls with both independent implementations. It compares every load C3
object, refusal table and active growth-support table. Each cycle must map to
reference original edges, signs and supporting clauses, then match the
reference's minimality/tie-break and reconstructed features.
`python3 -I control-matrix-tests.py` plants malformed diagnostic rows; these
validate that reader, separately from implementation-path plants.

`qualification-branches.tsv` inventories physical branches. Rust test
`qualification::fixed_seed_complete_c3_objects_and_verdicts` compares complete
load objects/verdicts on 256 fixed-seed programs + thirteen named cases, and
query slices on admitted programs. Refused programs record query nonexecution.
C5 switches compile into tests alone. Seal the `cargo test --lib --no-run`
executable as `tests-<sha256>`, then run `qualification-check --tests <seal>
--out <new-dir>`. Each defective switch must produce its declared nonempty C3
delta and a failing test process; switch-off runs must match again. The forced
hash-collision control retains exact-key comparison and must stay green.

`determinism-check --ta <seal> --root <frozen-repo> --instances <package>
--manifest <run-manifest.tsv> --out <new-dir>` checks the F1a G10 source/query
identities, runs two fresh candidate processes and compares all fourteen load
and slice objects. Its output keeps both commands, rc and complete-file hashes.

## Candidate projection

`project-candidates` applies the frozen D6/D7/D9 count/cost rule to validated
`candidate-census` output. `--acquisition` supplies the independent harvest's
`counts.tsv` + `cases.tsv`; `--profiles` supplies the LBR period-share table;
`--controls` names a sealed-I2 G10 control directory; `--predecessor` supplies
the incidence-census root; `--i2` supplies the measured endpoint root. The
frozen layout uses sibling `builder-17-l0-*` directories. `--wall-source control`
keeps every wall/residue on the unprofiled controls. F3/F4 inherit F1a per-unit
costs. I2 measurements check the extrapolation; costs stay on G10 calibration.

Run `--stage baseline --out <new-dir>` first. Inspect every count/stage error.
Then write `baseline-read.json`: each of `x100-count-errors.tsv` and
`unmodified-baseline-errors.tsv` maps to its file SHA256. The same command with
`--stage levers` checks that acknowledgement and the unchanged configuration,
then publishes costs + the fixed selection. Negative/subset-invalid repeat
counts remain non-estimable; a winner-changing non-estimable lever stops for
MAIN. The acknowledgement records workflow, not a formal claim of human review.
`python3 -I project-candidates-tests.py` exercises the arithmetic, eligibility,
return charges and count-domain controls with synthetic inputs.

## Failure-point census

`--failure-census` enables the structure census + S1a detail. It requires a full
refined load without queries. It adds diagnostics alone; no candidate algorithm.

```sh
python3 -I failure-census --ta <sealed-ta> --root <frozen-repo> \
  --instances <frozen-package> --run-manifest <committed-run-manifest> \
  --out <new-absolute-dir>
```

Cases = F1a G2/G4/G10, F3/MX G10 + F1b G10. The runner validates source
identities, every forced-call total + access-width partition, then runs `cmp`
on F1a/F1b G10 clause/base-flow dumps and clause inventories. A one-edge-drop
control for each dump must differ. This tests the comparator, not C5's candidate
implementation-path plants. `census-check --failure-census` compares behavior
with the sealed uninstrumented oracle.

`failures-{base,k1}.tsv`: actual forced invocations, partitioned by phase,
outcome + same/cross input-copy and physical-document relation. Copy = the
`copies/NNNN` input group, not a logical filter; group inventories keep paths,
patient/registry labels. Cached body/source reusers are not extra invocations.
Outcomes = edge, head-argument unification failure (one-based argument, rigid
or variable), unary constraint empty, link propagation empty (label + first
emptied role), witness search failure. A failed branch inside an eventually
successful witness does not change its edge outcome. Work columns count state/
domain copied memberships, meet operand widths/yields/wildcard scans + owner
rows inside that invocation; outside base-state/index work stays phase-only.

`access-{base,k1}.tsv`: one row per distinct evaluation input key; occurrences,
resource widths by explicit enumeration, point membership, set operand +
identity-only access; overlap remains explicit. Widths count stored IDs/top
markers (All = one marker); an All target carries one explicit-access charge,
not enumeration of its ground universe. Set-operand width is not a literal
B-tree visit count. Widest explicit + combined enumeration/operand
resources retain exact feature/link labels. Modes union over repeated keys;
per-key counts retain the largest observed visit/lookup count. Intermediate
state-domain iteration is not attributed to persisted-resource rows. The
whole-phase counters retain its cost. Model-local IDs are diagnostic keys.

`canonical-clause-edges.tsv` uses load-order document/ordinal slots + sign/kind;
`canonical-base-flow-edges.tsv` maps Skolem Doc fields through loaded document
records, retaining kind/sentence/slot, and arithmetic clause/goal coordinates.
These normalize family-specific path/name spelling for the premise check,
not the programs' lexical meaning. `canonical-clause-nodes.tsv` retains every
original clause + patient flag. Full C3 objects remain separate work.

## Feature representation

- Reuse `sb1-checker::{program,term,query}`. The checker delta exposes these three
  modules only. Safety + source-term admission remain the checker's.
- Origin = `(Kind,Doc,Sentence,Slot)`. Deps collapse into that origin. Opaque
  symbols, integers + structural argument roles retain exact values; role 1
  never matches role 2. Unknown numeric values have a separate abstraction.
- Per-origin features = nouns, event lemmas, box operators, properties + context
  payload predicates. Argument/preposition labels map (owner context, source
  origin) to a target-origin domain. No abstract whole-program relation tuples or Cartesian
  antecedent environments are materialized.
- Load features start from every fixed head + fact. Variable-headed producers
  propagate features to every origin permitted by their full positive-body
  unary + owner/source/target link constraints. Monotone closure includes every rule's possible feature
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
- Generic patient constants have every feature. Implicit patient links stay
  within the patient/atomic domain. Rule-produced links from those constants
  may target KB origins and remain in the closure. Patient/KB namespaces are
  checked disjoint at admission; production must preserve this boundary.
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
