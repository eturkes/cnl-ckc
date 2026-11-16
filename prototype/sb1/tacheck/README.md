# TAS B1 checker spike

Development TA admission. No verified patient-answer or production-soundness claim.

```sh
cargo build --release --locked --offline -j 4
cargo test --release --locked --offline -j 4 --lib
target/release/sb1-tacheck <repo-root> <composition.tsv> [--already-shaped]
```

The CLI loads the composition, applies the census's DC guard shape, obtains
untrusted hints, releases solver-only state, then runs the independent core.
Patient suffix = the composition's declared count; absent metadata ⇒ one final
document. A knowledge-only control uses `# patient_documents\t0`.
Prepared controls use `--already-shaped`: CDC was shaped before the plant
programs joined it; their comments are not compiler statement groups.

- rc 0 = admitted; stdout = clause + independently enumerated edge counts.
- rc 1 = refused certificate or an independently checked negative/growth cycle.
  A cycle names original document/clause ids; growth also names source variables.
- rc 2 = source input / usage / I/O failure.
- rc 3 = indeterminate producer/resource/search outcome.

## Boundary

`producer.rs` = untrusted census invocation + conversion. `core/` shares only
`sb1-checker::{program,term,query}`. Feature extraction, set operations, transfer,
may-match, head-index lookup, signed graph, source closure, k1 checks + growth
are independently implemented. Checking-core maps use primitive keys; collisions
are resolved by full key equality. The crate forbids unsafe code.

F = immutable hinted feature rows + owner/source/target rectangles. The core
checks every fixed seed and enabled transfer contribution. NAF + built-in truth
remain over-approximations. Each whole-conjunction goal probes a head-index
superset, then runs its own alias-aware may-match. Search exhaustion returns
indeterminate. Every enumerated signed edge enters the CSR and checks a weak
positive / strict negative clause rank.

Source hints carry one binder per variable. The core adds the scalar type,
checks filtered closure across every compatible binder producer, checks k1's
semantic registry and every source-clause variant, and checks every hinted
constructor tuple's compositional output and strictly descending growth rank.
Arithmetic outputs are checked operationally; arithmetic + nested binding
coverage remain outside MAIN's flat forwarding Verus seam.

The retained query head index + CSR support generic patient-top slices. q-all
keeps every clause. Projected questions root their complete conjunction, NAF
witnesses + bound deontic-context payloads, then take forward reachability.
Selected documents keep metadata. Query arenas and temporary set ids leave at
state-free boundaries.

`core/extension.rs` checks the DC/original binding: same original clause ids,
heads and body prefix; additions = ground scope-exclusion NAF alone. This is
the executable premise of MAIN's `envelope::naf_extension_wfm`, not equality
of the original and DC-shaped models. The thin producer/MC integration has its
own acceptance row in the TAS check.

## Warm provider

```sh
target/release/ta-server serve --provider static --ta-receipts none \
  --root <repo> <composition.tsv> <arm-dir>
```

The server uses the existing thin-path `SliceProvider` seam. Prepare reads the
knowledge prefix alone, DC-shapes it and checks its original-clause binding
before the solver runs. The census exports hints and releases solver-only state.
The independent checker checks the hints and retains the original-clause CSR,
feature model, namespaces and query head index. Each call admits fresh patient
facts, selects original knowledge ids and runs the unchanged producer/certificate/
model-check/answer path. q-all keeps every clause. `static` means typed generic
CSR selection here; `all` still checks TA but selects the whole knowledge base.
The original checker's built-in providers keep their previous behavior.

`--ta-receipts none` is mandatory: prepare itself runs the sole census, so a
separate census invocation is refused. The prepared-control input mode belongs
to T1 alone; every warm load applies the default DC shape. TA checks the shaped
program; the producer and model checker evaluate the original clauses (D1e).
The production cost of evaluating the added DC guards is outside this screen.

`ta-phases.tsv` records incremental wall/CPU boundaries plus RSS and VmHWM;
`server-ready-publication` observes the real protocol flush after the outer load
indices and identity files exist. Consecutive boundaries define exclusive TA
phases; the outer `load-phases.tsv` accounts parsing/indexing and their gaps.
`ta-selections.tsv` records every selection's wall/CPU, boundary RSS/HWM and
clause count. `ta-payload.tsv` reports named buffer payload lower bounds alone:
allocator/hash slack, private indices and source-arena internals are not assigned
to components. Complete process/lifecycle RSS and conservative owned-child bounds
own the memory verdict. `load.tsv` retains the external-receipt label `unchecked`;
`typed_analysis_hook=1` and `ta-provider-ready` distinguish the checked prepare.

`tools/sb-campaign-chunk` accepts `SB1_LOAD_DEADLINE_MS`; its default stays
1,800,000 ms. TAS uses 43,200,000 ms with receipts=none and no separate TA run.
`python3 -I tests/warm.py --help` runs the G1/G10 development correctness matrix:
actual typed/static warm calls vs frozen independent full-program answers,
plus the census's concrete slice identities/counts as a separate diagnostic.

## Canonical qualification

`ta-objects <root> <composition> <out> [--query <file>]…` retains diagnostic
objects after the same checking chain. Normal server/check calls discard them.
`fixtures/t3-normalization.json` fixes semantic ids, scalar-leaf handling,
source-restricted final projection and the independent reachability comparison.
`tests/parity.py` compares the 13-case G1/G10 matrix in both directions against
the sealed census and its dump-only complete-mixed path. The latter uses existing
`build_k1` with the unchanged filter/projection, not the hint export as its oracle.
`--graph-only` omits feature flattening and hint-export diagnostics for T3x.

`qualification_tests::b3*` reproduces the independent 1,177-program ground table.
`plant_tests::t2_probe` plus `tests/plants.py` runs 21 green→plant→restored triples
on a sealed test executable. Faults compile into tests alone. Public acceptance
keeps every seed/closure, source/registry and rank check.

`arbitrary_tests::t4_arbitrary_hints_against_independent_ground_semantics` uses
`fixtures/t4-generator.json`: 256 generated relational cases, ten prospective
named controls and one labeled post-hoc negative hint-grammar control. The v1
manifest and original red inputs stay pinned beside the v2 amendment. Its exact
term/substitution/join oracle shares parser/IR alone.
It checks canonical and larger closed hints, complete-edge monotonicity,
malformed evidence, total independent WFM and exact full/slice query sets.
The finite domain excludes arithmetic and nested body-identity binding;
resource guards fail rather than truncate a program. This is testing evidence,
not a production theorem or a serialization/projection proof.

`tests/acquire.py --selftest` tests the prospective shared-host validity
boundaries. Acquisition requires the recorded scope, timing lock, heavy holder
and MAIN's freeze. Its per-attempt raw host/process records select replacements
from external validity alone. Its valid-acquisition label is not a resource,
correctness or TAS-GO verdict. The harness's summed RSS and conservative child
bounds still decide T7; a cgroup physical peak alone cannot bound summed RSS.

## Checks

`fixtures/controls.tsv` fixes T1's 16 DC controls + four independent inactive/
active cases. Fixture bytes for the latter preserve the independent probes.
Tests cover source seed + forwarding omissions, exhaustion, CSR reachability,
k1 forwarding, scratch reclamation, the DC binding and the source-level
primitive/IR boundary. TAS's full T1–T8 screen is separate from these tests.
