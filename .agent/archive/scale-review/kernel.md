# scale-kernel

Target = `9129d719b9166f62f61e35bbaa913dc75cd1ddfe`; read-only. HEAD advanced to `eda3f56c9b4e216bd341a29687ecaed70c2abbad`; target kernel/spec + two shell files byte-unchanged (`git diff --exit-code 9129d719 HEAD -- …` rc 0; positive control = 10 changed non-target paths). Fixed check set = SC1 time; SC2 memory; SC3 stack; SC4 repeated work; SC5 gate scaling; SC6 composition scope; SC7 human/process; SC8 artifact limits. Findings = scalability revisions, not correctness claims absent a demonstrated contract violation. Cost projections label measured vs derived; ×100/×1,000 = stress envelope.

Scope convention: ×N multiplies the ACTIVE composition unless a row explicitly names a per-guideline gate loop. Current check/certify process guidelines separately: at fixed guideline size their total time scales roughly with G, and their peak memory does not automatically become the all-guideline union's memory. This is also the SC6 coverage gap: those gates do not stress a growing union. A 300 s aggregate watchdog applies to each manifest the check wrapper passes, not to all guidelines as one union today.

Notation: D = manifest documents; C = loaded clauses including records; O = proof obligations; V = per-guideline vocabulary rows; U = distinct coverage keys; T = executed proof calls; N = trace nodes/events; h = proof depth. Bare source filenames below = `rust/ckc-kernel/src/`; `v1_cli.rs`/`certify_cli.rs` = `rust/ckc/src/`; `rust/ckc-spec/…` = `rust/ckc-spec/…`; `rust/ckc/src/check/…` = `rust/ckc/src/check/…`. Exact full-path anchors = probe `anchors.tsv`.

## Coverage

| target | SC1 | SC2 | SC3 | SC4 | SC5 | SC6 | SC7 | SC8 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| C1 manifest + load + composition reader | revise S1 | revise S2 | ok | revise S1/S2 | revise S1/S2/S13 | ok | ok | revise S2 |
| C2 term arena + store + unification + arithmetic | revise S16 | revise S2 | ok | revise S16 | revise S16 | ok | ok | revise S2 |
| C3 clause index | revise S6 | ok | ok | revise S6/S10 | revise S6 | ok | ok | ok |
| C4 aggregate replay + payload/key/coverage checks | revise S3/S4/S5 | revise S2/S5 | ok | revise S3/S5 | revise S3/S4/S5/S13 | revise-before-2nd-guideline S19 | ok | revise S2 |
| C5 recursion + guard scan | ok S18 | ok S18 | ok S18 | revise S2 | revise S1/S13 | revise-before-2nd-guideline S12 | ok | ok |
| C6 answer engine + selection + output | revise S3/S6/S11/S16 | revise S2/S11 | ok | revise S11/S16 | revise S11 | revise S11 | ok | revise S11 |
| C7 trace production + materialization | revise S7/S8/S10 | revise S2/S8/S10/S20 | revise S8 | revise S7/S10/S20 | revise S7/S10/S20 | ok | ok | revise S10/S11/S20 |
| C8 trace-check + replay | revise S7/S8/S9/S10 | revise S2/S9/S10/S20 | revise S8 | revise S7/S9/S10/S20 | revise S7/S9/S10/S20 | ok | ok | revise S10/S11/S20 |
| C9 sort + dedup | revise S3 | ok | ok | revise S3 | revise S3 | ok | ok | ok |
| C10 certify_doc + certify_query kernel | revise S14/S16/S17 | revise S14/S17 | revise S15 | revise S13/S14/S17 | revise S13/S14/S17 | ok | revise S13 | ok |
| C11 v1_cli + certify_cli I/O | revise S10/S13 | revise S2/S10/S13 | ok | revise S10/S13 | revise S13 | revise-before-2nd-guideline S12/S19 | revise S13 | revise S2/S10/S13 |

## Findings

| id | component (file:line) | operation | check | scaling | cost ×1 | projected ×100 / ×1,000 | breaks at | revision needed | verdict |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| S1 | `rust/ckc-kernel/src/k2_load.rs:105–225,449` | de-duplicate manifest paths + locate each first row | SC1,SC4,SC5 | derived Θ(D²·path-bytes): linear `path_contains` in `unique_paths_exec`, then restart `row_index_exec` for every unique path; runs in every composition load | synthetic 512 docs: answer 0.03 s / recursion 0.03 s (whole operations) | quadratic work ×10⁴ / ×10⁶; isolated CDC-equivalent load overhead not measured | synthetic 8,192 docs: answer 3.21 s; ×100/×1,000 corpus = 49,200/492,000 paths | verified first-occurrence path map; preserve first-error + duplicate semantics | revise |
| S2 | `rust/ckc/src/v1_cli.rs:103–104,164–165`; `rust/ckc-kernel/src/k2_term.rs:17–33`; `rust/ckc-kernel/src/k3_coords.rs:41–51,151–165`; `rust/ckc-kernel/src/v1_read.rs:247–250` | read + parse complete composition into owned term arena | SC2,SC4,SC8 | derived Θ(input bytes + term occurrences + C clause coordinates); owned atom/functor strings per occurrence, no intern table; reader additionally makes rewritten bytes + usize/raw-byte map per file; all modes read payload files, including modes that use only readability | supplied CDC load: 167 MB, 0.72 s; supplied ×2/×4 329/654 MB | linear same-shape RSS ≈16.7 GB / 167 GB in the supplied MB units (baseline supplied, not rerun here); query/search overhead additional | already material on a workstation; a ×100 union exceeds ordinary desktop/CI RAM | persisted/content-keyed verified representation, interning + selective/incremental composition, share immutable clauses across requests; avoid unused payload byte materialization while retaining readable-file contract | revise |
| S3 | `rust/ckc-kernel/src/k2_sort.rs:266–328,332–367,409–462,488–498`; `rust/ckc-kernel/src/k2_aggregate.rs:568,580,858–859`; `rust/ckc-kernel/src/k2_answers.rs:1018` | sort + deduplicate docids, obligation keys, coverage pairs, answer rows | SC1,SC4,SC5 | derived Θ(n²) sort: reverse loop inserts into Vec; sorted/unsorted input trades element shifts against comparison scans; dedup also Θ(u²) via `out.insert(0, ...)`; key_check sorts keys twice | not isolated; included in measured synthetic aggregate 512/2,048/8,192 = 0.05/0.66/12.07 s | sort/dedup work ×10⁴ / ×10⁶ when n follows corpus/results | grows over D, obligation keys, loaded pair occurrences and answers; no timing-specific break asserted | verified O(n log n) sort + linear dedup/merge; reuse sorted keys in key_check | revise |
| S4 | `rust/ckc-kernel/src/k2_aggregate.rs:601–674,858–859,889–900` | obligation coverage set equality | SC1,SC5 | derived Θ(U_loaded·U_covered): for each already sorted/deduped key, restart linear `contains_root`; both directions run on a successful composition | not isolated; every success performs all membership scans | same-shape work ×10⁴ / ×10⁶ | passes become progressively slower; check caller has 300 s per aggregate subprocess (`rust/ckc/src/check/documents.rs:84–89`) | verified linear two-pointer comparison of sorted unique keys, preserving first missing/extra report | revise |
| S5 | `rust/ckc-kernel/src/k2_aggregate.rs:943–980`; `rust/ckc-kernel/src/k2_bridge.rs:435–481,483–575` | replay each obligation | SC1,SC2,SC4,SC5 | derived Θ(O·(C + body-items)) allocation/copy even with perfectly selective proof index: `witness_db_exec` copies every clause body + NAF root Vec for every obligation; index itself correctly built once before replay | selective synthetic 2,048/4,096/8,192 docs+obligations: aggregate 0.66/2.31/12.07 s, rc 0; 25/46/86 MiB RSS | O and C both ×G → work ×10⁴ / ×10⁶; live scratch DB memory linear in C, not O·C | a growing union checked through the current wrapper can hit its 300 s aggregate watchdog before six hours; crossover unmeasured (per-guideline loop does not build that union today) | witness-prefix view over immutable base DB + existing index; no full DB copy per obligation; content-keyed replay cache whose key includes composition inputs | revise |
| S6 | `rust/ckc-kernel/src/k2_index.rs:521–546,992–1088,1283–1337,1659–1704`; `rust/ckc-kernel/src/k2_engine.rs:5461` | indexed K2 predicate call | SC1,SC2,SC4 | derived build O(C log K + hash-chain comparisons), storage Θ(C); lookup O(log K + log candidate-list + collisions); first-argument class only, not later bound args or compound contents, so common `actual` + gid-functor contexts still have Θ(C) candidate sets | source census: 5,704/17,861 semantic clause heads use `actual`; remaining 12,157 not separated by full compound contents (runtime profile unmeasured) | selective index build ≈linear-log / common-context candidate work can remain ×100/×1,000 per call and quadratic across growing results; equal hash chains permit Θ(C²) worst-case construction | larger multi-guideline + patient joins; construction itself has an existing 10³–10⁵-class scaling gate, not a newly demonstrated failure | preserve index mechanism; extend measured selectivity to later bound args/deeper keys + share per composition; exact key checks keep collision correctness | revise |
| S7 | `rust/ckc-kernel/src/k3_machine.rs:492–541`; `rust/ckc-kernel/src/k2_engine.rs:4170–4235`; `rust/ckc-kernel/src/k3_rows.rs:461–551` | first proof for each committed answer | SC1,SC4,SC5 | derived Θ(T·C) clause-scan upper-bound for T trace-machine calls, then repeated from the beginning for each answer; K3 does not use K2's clause index | selective synthetic one-clause proof in last doc: trace 512/2,048/8,192 = 0.06/0.87/5.93 s (includes double load) | with result count and C ×G, scan component ×10⁴ / ×10⁶; fixed result count ×100 / ×1,000 | trace generation remains expensive even after tabled answer mode; no CDC isolated trace timing collected | verified trace candidate index with the same first-proof/order semantics; reuse across rows | revise |
| S8 | `rust/ckc-kernel/src/k3_materialize.rs:151–185,289–451,460–617` | turn proof-event log into tree | SC1,SC2,SC3 | derived Θ(N²·h) path-comparison work: every node calls linear `event_at` over N-event log, comparing O(h)-long paths; recursive node↔children follows proof depth, log/path vectors materialized | source-derived; no long-proof timing or stack-overflow point measured | if N ×G, up to ×10⁴ / ×10⁶ at fixed h; depth may amplify further | long composed proof, independent of clause loading; trace depth = 1,000 but no empirical stack-capacity guarantee | verified log path index or one-pass event/tree builder + explicit stack; share path prefixes | revise |
| S9 | `rust/ckc-kernel/src/k3_coords.rs:331–377`; `rust/ckc-kernel/src/k3_join.rs:381`; `rust/ckc-kernel/src/k3_impl.rs:162–199` | trace-check clause/digest custody join | SC1,SC2,SC4,SC5 | derived Θ(N·C): each clause node scans all composition coordinates/digests to prove exactly one match; checker also regenerates complete first-proof trace before this join; tally allocates one root per visited clause | synthetic 8,192-doc one-node trace-check = 5.92 s (whole command) | growing proof nodes + C → ×10⁴ / ×10⁶ join work | many proofs / guideline compositions; custody becomes a second full scan burden | verified `(docid, sentence, digest) → count` multimap, reused across all proof nodes; retain exact-one semantics | revise |
| S10 | `rust/ckc/src/v1_cli.rs:109–148`; `rust/ckc-kernel/src/k3_impl.rs:29–43,72–85,132–145`; `rust/ckc-kernel/src/k3_print.rs:205–235` | obtain clause digests, then trace/trace-check | SC1,SC2,SC4,SC5,SC8 | derived: EACH trace or trace-check parses whole DB twice; first pass renders every loaded clause to owned lines, shell hashes all, second pass re-parses composition; lines + inputs + digests coexist; no immutable-composition handle/cache | synthetic 8,192-doc trace≈5.93 s vs answer≈3.21 s (single-run, not isolated causal estimate) | ≥2 full-load passes + all-clause hash work per Q; Q and corpus both grow → Θ(Q·C) repeated work, plus S1 | committed queries multiply cost even when only one document/query changed | keep parsed composition + canonical clause digests in a verified immutable handle; invalidation by input digests/schema, then per-query proof work | revise |
| S11 | `rust/ckc-spec/engine.rs:23–44`; `rust/ckc-kernel/src/k2_engine.rs:6017–6115`; `rust/ckc-kernel/src/k3_rows.rs:308,454,563–567` | bounded SLD answers/replay + bounded trace explanation | SC1,SC2,SC5,SC6,SC8 | already routed limitation: no tabled/subgoal-result reuse, repeated derivations; fixed answer 100,000 inference / depth 100, replay 1,000,000 / depth 4,000, trace 100,000 / depth 1,000 + shared 1,000,000 work+nodes | brief/spec: CDC patient enumeration already returns `indeterminate(limit)`; not rerun here | increasing bounds alone cannot turn exponential/repeated search into scalable evaluation; trace path remains separately bounded after new answer mode | ×1, already planned IMPLEMENT scope | tabled/bottom-up verified evaluator + explicit provenance/tracing contract; evaluate aggregate obligations + cross-guideline patients, not answers alone | revise |
| S12 | `rust/ckc-kernel/src/k2_impl.rs:119–124`; `rust/ckc-kernel/src/k2_recursion.rs:795–865`; `rust/ckc-spec/replay.rs:804–810`; `rust/ckc/src/check/documents.rs:156–199` | compose independently checked guideline sets with scope exclusions | SC6 | demonstrated: exclusion-only set and unrelated should-in-body set each pass recursion-check; union fails `scope_condition(unattributed)`; spec intentionally makes one exclusion anywhere enable global body-edge rejection; per-guideline gates cannot establish arbitrary union admissibility | `scope-mix.py`: singles rc 0, union rc 1; matches current spec, not an implementation/spec disagreement | failure can appear at G=2, before any volume limit | 2 compatible-looking guideline sets; routed typed reachability work must grade cross-guideline union | composition-level admission + regression worlds/negative loops for several guidelines; typed reachability keyed by referent/operator paths (existing IMPLEMENT direction) | revise-before-2nd-guideline |
| S13 | `rust/ckc/src/certify_cli.rs:278–279,301–341,382–426`; `justfile:996–997` | re-certify unchanged corpus | SC2,SC4,SC5,SC7 | derived Θ(G·D) APE work with two dumps per artifact, fresh stage + grammar per guideline, fresh table parse in every kernel certificate; thread pool within one guideline, guideline loop serial; all per-guideline results held until parallel batch joins; no changed-input cache | supplied CI ≈8 min/guideline + 2 min fixed cases | ≈13 h 22 min / 133 h 22 min at same per-guideline runtime (simple linear projection; larger vocabulary/replay may worsen) | six-hour Actions job at ≈45 same-cost guidelines; iteration turnaround degrades much earlier | independently keyed certification certificates/artifacts + verified cache invalidation (ACE/PL/dump/table/lexicon/schema/verifier/compiler pins), bounded artifact streaming, CI shards/changed-target fast gate while keeping full coverage | revise |
| S14 | `rust/ckc-kernel/src/m6_drs.rs:925–977`; `rust/ckc-kernel/src/m6_project.rs:476–529`; `rust/ckc-kernel/src/m6_expand.rs:900–946`; `rust/ckc-kernel/src/m6_refs.rs:43–83`; `rust/ckc-kernel/src/m6_order.rs:214–350` | project/canonicalize one document/query | SC1,SC2,SC4 | derived local superlinear work: sentence loop scans all tagged roots, repeated prefix-concat of expanded bodies, linear binding/ref membership per item, repeated readiness scans in bind-first; no whole-corpus lookup here | no isolated timing; fixed small documents distribute independently across G | same-size docs → aggregate ×100 / ×1,000; one document with s sentences / b goals / v variables carries O(s·tagged-roots), ≥O(b²), O(v²) components | large/complex passages, not guideline count alone; no measured break threshold | index roots by sentence; append/move prefixes; set/map variable/binding membership; incremental readiness scheduling; preserve canonical order | revise |
| S15 | `rust/ckc-kernel/src/m6_ante.rs:304–332`; `rust/ckc-kernel/src/m6_flat.rs:375–449`; `rust/ckc-kernel/src/m6_group.rs:515–565`; `rust/ckc-kernel/src/m6_normal.rs:57–91`; `rust/ckc-kernel/src/m6_query_drs.rs:185–250`; `rust/ckc-kernel/src/m6_query_markers.rs:143–174` | traverse per-document DRS/conditions + normalize nested terms | SC3 | derived native recursion proportional to condition-list length / nested term depth (unbounded by G); parser, arena print/compare and K2 solve instead use heap worklists | no stack-failure threshold tested | independent small docs keep stack constant with G; a single long sentence/DRS can grow native stack linearly | per-document depth/width envelope not stated; stack overflow unconfirmed | explicit stacks for unbounded traversals or an audited admission bound with actionable error; keep per-guideline parallel worker stack budget explicit | revise |
| S16 | `rust/ckc-kernel/src/k2_engine.rs:1074–1136,2525–2586,2840,4311,4619`; `rust/ckc-kernel/src/k2_rewrite.rs:145–147`; `rust/ckc-kernel/src/v1_read.rs:363–374`; `rust/ckc-kernel/src/k3_number.rs:253,305`; `rust/ckc-kernel/src/v1_term_impl.rs:9404–9509` | local term/goal traversal, unification, renumbering | SC1,SC2,SC4 | derived local quadratic mechanisms: Vec front remove/insert/copy; repeated first-missing-child scan on wide terms; reader seen/given + parser variable-tracker linear scans per variable; binding substitutes across all remaining pairs/goals/answer template; no hash-consing | no isolated timing; arbitrary-width integer arithmetic uses grade-school multiply/divide/GCD (bit width, not G) | fixed term/goal widths → linear in number of visits; widths/result templates growing with composition can add quadratic factors | long generated conjunctions, wide terms, large answer-variable lists; not a demonstrated present CDC blocker | stack/deque worklists, child cursor, indexed variable/binding state, shared term storage; benchmark representative widths, keep exact-rational semantics | revise |

| S17 | `rust/ckc-kernel/src/m7_temporal.rs:824–845,848–1101,1346–1393`; `rust/ckc-kernel/src/m7_annotate.rs:1615–1628`; `rust/ckc-kernel/src/m6_doc.rs:364`; `rust/ckc-kernel/src/m6_project.rs:516` | parse guideline vocabulary + clone annotation environment | SC1,SC2,SC4,SC5 | derived Θ(V²) string copies + linear duplicate scans: every added row clones entire accumulated table; each document/query reparses it; sentence Env clones table + all DRS declarations again | CDC table = 101 data rows (102 lines incl. header); no isolated timing | fixed V per guideline → ×100 / ×1,000 total work; merged/shared V growing with G would be quadratic per certificate, so do not silently globalize this table | larger per-guideline vocabulary / large-document sentence count; volume breakpoint unmeasured | parse/validate once by table digest, owned/borrowed immutable verified vocabulary; move row buffers + indexed duplicate checks; share Env instead of deep copies | revise |
| S18 | `rust/ckc-kernel/src/k2_manifest.rs:93–118,268–340`; `rust/ckc-kernel/src/k2_recursion.rs:196–357,439–481,603–695,795–865`; `rust/ckc-kernel/src/k2_engine.rs:3560–3622` | manifest grammar + recursion rule census + guard walk | SC1,SC2,SC3,SC4,SC5 | ok local mechanics: forward manifest byte scan; 22 fixed census buckets, Θ(22C) clause predicate tests; per-rule self-unifiability check; iterative body-edge DFS; temporary unification arena truncated; only unrelated whole-load/cache issues S1/S2/S12 remain | synthetic 512/2,048/8,192 recursion-check 0.03/0.29/2.94 s (dominated in large case by path lookup; no rule clauses in these fixtures) | local census/guard work ×100 / ×1,000 at fixed term width | no new independent count/stack limit found; composition admissibility is S12, not discharged by this ok | retain mechanics, share loaded immutable DB; typed dependency change graded separately | ok |

| S19 | `rust/ckc/src/certify_cli.rs:411–416`; `justfile:996`; `rust/ckc/src/check/documents.rs:156–199`; `rust/ckc-spec/replay.rs:536–548` | treat per-guideline aggregate success as a composition certificate | SC6 | demonstrated non-compositional property: A's NAF-backed obligation and B's positive facts each aggregate-check green; union defeats A's NAF and returns `obligation_failed(rulea,1,variant(1))`; these are canonical K1/K2 fixtures, not APE-to-ACE fidelity examples | `aggregate-mix.py`: singles rc 0, union rc 1; current behavior is correct for its spec | failure can appear at G=2 independent of corpus size; enumerating 2^G unions is not a scalable remedy | before any downstream claim that individually certified guidelines form a certified composition | state certificate/admission scope explicitly; composition-keyed replay/invalidation or verified sufficient summaries + cross-guideline regression worlds; arbitrary unions need not pass, since NAF intentionally changes under added facts | revise-before-2nd-guideline |
| S20 | `rust/ckc-kernel/src/k3_state.rs:489–583`; `rust/ckc-kernel/src/k3_machine.rs:1245–1287`; `rust/ckc-kernel/src/k3_rows.rs:297–359,461–588` | hold trace-search arena across backtracking + successful rows | SC2,SC4,SC5,SC8 | derived: K3 backtracking truncates only the event log, not the term arena (fail takes `&ETermArena`); failed/limited row resets to entry mark, but successful row keeps search temporaries + materialized proof; row loop retains these across all successful solutions. Memory follows allocated search work, not only current proof depth/output | no dedicated peak-memory measurement; fixed transition budgets limit number of calls but not term widths/bytes per call | more proof work/rows → larger live arena even if each final proof is small; increasing evaluator limits lifts today's incidental cap | potentially before output size is itself large; exact CDC threshold unmeasured | verified arena marks + reachability/copy-out of successful proof payloads; prune dead branch temporaries, reuse immutable term/variable identities; explicit byte budget | revise |

## Ranked summary

1. **Already at ×1:** S11 bounded SLD cannot finish the routed patient enumeration. Treat trace/provenance + replay as independent consumers of the evaluator; speeding answers alone leaves S5/S7–S10.
2. **Before guideline 2:** S12/S19 composition-level admission/certificate scope. Two individually green sets demonstrably fail when combined. This is the existing specification's global guard restriction, not a new implementation/spec inconsistency. Add multi-guideline fixtures to the typed-reachability unit.
3. **Well before ×100:** S1/S3–S5 composition quadratics + S2 materialization + S10 repeated whole loads. The current check wrapper permits 300 s per aggregate manifest; if reused for a growing union, that can fail before the six-hour cap. Today the per-guideline gate does not create that union. Exact CDC aggregate crossover unmeasured. Selective synthetic 2,048→8,192 documents+obligations (×4) increases aggregate wall time 0.66→12.07 s (×18.3), with all proof lookups indexed to distinct contexts.
4. **At ×100:** S13 serial per-guideline certification alone projects ≈13 h 22 min from the supplied CI baseline. A ~45-guideline six-hour crossover is an optimistic linear estimate, not a measured threshold. S2 same-shape composition RSS ≈16.7 GB before query/proof expansion; ×1,000 ≈167 GB (same supplied MB units).
5. **As answers/proofs grow:** S6 common-context weak selectivity, S7 Θ(TC) trace search, S8 Θ(N²h) proof materialization, S9 Θ(NC) custody joins + S20 retained trace-search temporaries. Preserve exact proof order/bytes and join uniqueness; cache/intern/index immutable inputs before considering larger limits.
6. **Large passages, not G alone:** S14/S16 local quadratic scans/copies, S15 native recursion, S17 repeated quadratic vocabulary builds. These need explicit per-artifact budgets/benchmarks; no present CDC failure or universal stack threshold demonstrated.

Severity: S1–S7, S9–S13, S19/S20 = major scalability revisions; S8/S14–S17 = minor until a representative long/large artifact demonstrates a present budget failure. S18 = ok. No correctness finding against the committed verified specifications established. S12/S19 refute blanket transfer of individual green verdicts to arbitrary unions. They match the current specs; arbitrary NAF-bearing unions need not stay green. The revision is an explicit admission/certificate contract plus composition-level coverage, not unconditional union acceptance.

## Evidence + limits

Coverage complete = 11 components × 8 checks = 88 adjudications. `ok` = no additional scalability revision in that component/check, not an all-domain correctness proof.

- SC3 `ok`: manifest/document/term parsers, printer/comparator, K2 SLD and clause index use heap loops/worklists; decimal recursion is bounded by usize digit count. G/D/C add iteration, not native frames. Exceptions = S8 trace materialization + S15 M6 traversals. `copy_tree` recursion follows the fixed vendor tree, not the corpus.
- SC6 `ok`: runtime global document-record uniqueness and exact source ordering are checked over the supplied manifest; no new implementation inconsistency found. S12/S19 own the per-guideline-to-composition gap. Pairwise acceptance alone is not a theorem about every subset of G guidelines.
- SC7 `ok`: no extra manual author/reviewer step inside these kernel/shell functions. Clinician throughput/authoring/release UI = outside assigned slice. S13 owns process turnaround from mandatory fresh certification.
- SC8 `ok`: no separate global guideline/document count cap found in these routines; outputs are Vec buffers. Trace sentence ordinals must be <10^9 PER DOCUMENT, not a global D counter (`rust/ckc-spec/trace.rs:1097`). Whole buffers + bounded explanation size are already findings S2/S10/S11/S13/S20. Archive format/git-host file limits = outside slice.
- Arithmetic = exact byte-digit schoolbook multiplication/division/GCD (`rust/ckc-kernel/src/arith_mag.rs:570–817`), superlinear in numeric bit width, not G/D/C. No measurement says clinical operand widths require an arithmetic replacement; preserve exact-rational semantics. Resource benchmarks should include operand width before raising search limits.
- K2 arena reclamation exists: `run_inner` truncates at restored marks; `finish` copies saved answer rows after truncating work (`rust/ckc-kernel/src/k2_engine.rs:5996–6013,6082`); `head_proved`/`unifiable_apart` restore entry marks. S20 is K3-specific, not a claim that every arena grows with all historical search work.

### Reproduction + checks actually run

All paths below are inside `.scratch/agents/imp/scale-kernel/`. Every measurement = supplied prebuilt `rust/target/release/ckc`, SHA256 `affcbdec1344adf421008ebab26a06d12aeb70617cac50cf772db3e6498226c1`. Binary supplied by brief; not rebuilt here, so no independent build-to-source attestation claimed. Source targets byte-unchanged across the lead's characterization commit.

Synthetic probe (one fact + one obligation/document, THREE loaded clauses/document including records, DISTINCT first-argument contexts; not a CDC copy):

```sh
ROOT=~/Projects/cnl-ckc
export PATH="$ROOT/.toolchain/bin:$PATH"
heavy python3 -I "$ROOT/.scratch/agents/imp/scale-kernel/probe.py" "$ROOT" "$ROOT/.scratch/agents/imp/scale-kernel" --counts 2048,4096,8192 --run
```

`probe-results.jsonl` = raw command, rc, wall/user/sys, peak RSS KiB, stdout digest + bounded stdout/stderr; `probe.log` + `probe-large.log` retain launch output; per-count directories retain full stdout/stderr. 30 native invocations across D=256/512/1,024/2,048/4,096/8,192: all rc 0; all stderr 0 bytes; reports give the expected D documents + D obligations / one proof node. One moderate process at a time under `heavy`, 35 s per command cap. No command timed out. Single run per size/mode, coarse hundredth-second clock, shared-machine noise; no fitted exponent or production-wall-time prediction inferred from these samples.

| D | recursion s / KiB RSS | aggregate s / KiB RSS | answer s / KiB RSS | trace s / KiB RSS | trace-check s / KiB RSS |
| --- | --- | --- | --- | --- | --- |
| 512 | 0.03 / 7,060 | 0.05 / 9,244 | 0.03 / 7,468 | 0.06 / 8,396 | 0.07 / 8,240 |
| 2,048 | 0.29 / 16,728 | 0.66 / 25,816 | 0.38 / 19,388 | 0.87 / 21,052 | 0.85 / 21,972 |
| 4,096 | 1.38 / 31,012 | 2.31 / 47,356 | 0.82 / 33,356 | 1.59 / 37,784 | 1.63 / 37,996 |
| 8,192 | 2.94 / 56,472 | 12.07 / 88,412 | 3.21 / 60,844 | 5.93 / 70,636 | 5.92 / 71,252 |

The toy recursion fixture has no rule clauses: its growing time exposes load-path overhead, not the per-rule unifier's cost. Aggregate combines several independently derived quadratic paths; these timings DO NOT isolate their individual coefficients. ×100/×1,000 production costs remain unconfirmed beyond the supplied CDC load/certify baselines and derived operation counts.

Red composition-scope controls (existing behavior, requested future composability hypothesis):

```sh
python3 -I "$ROOT/.scratch/agents/imp/scale-kernel/scope-mix.py" "$ROOT" "$ROOT/.scratch/agents/imp/scale-kernel"
# two single-set positive controls rc 0
# combined rc 1: ace_to_pl_error(proof,scope_condition(unattributed)).
# script rc 1: AssertionError: individually accepted document sets must remain admissible ...
python3 -I "$ROOT/.scratch/agents/imp/scale-kernel/aggregate-mix.py" "$ROOT" "$ROOT/.scratch/agents/imp/scale-kernel"
# two single-set positive controls rc 0
# combined rc 1: ace_to_pl_error(proof,obligation_failed(rulea,1,variant(1))).
# script rc 1: AssertionError: per-set aggregate success is not closed under composition with NAF
```

Full outputs = `scope-mix.log` + `aggregate-mix.log`. These are red tests of an invalid blanket composition assumption, not regression tests asserting the current kernel is wrong.

Certificate smoke: `ckc certify-one doc fx-doc tests/certify-one/doc/ace.txt.in - tests/certify-one/doc/dump.txt.in tests/certify-one/doc/pl.txt.in` → rc 0. Expected positive control = one payload line, byte-equal to `tests/certify-one/doc/stdout.expect` (`cmp` rc 0); stderr = 0 bytes (`certify-one.out/.err`). No full `just gate`, `ckc check`, `ckc certify`, `just test`, verifier, mutation suite, broad patient query, long-proof stack probe or isolated per-function profiler run. CI times / six-hour limit = supplied brief, not independently refreshed from GitHub. Supplied CDC load figures = not rerun. No clinical fidelity claim made.

Runtime-body read aid = `extract-exec.py` + `exec-*.txt`; it strips most ghost/proof text, retains original line numbers, and is not a Rust/Verus parser. The scalability witnesses below are rematched against ORIGINAL source, not inferred from erased proof text. Proof-only K3/K5 soundness modules and unrelated K4/K5 UI/release code excluded; this audit does not certify their correctness or gate scale.

### Exact source witnesses

`python3 -I "$ROOT/.scratch/agents/imp/scale-kernel/check-anchors.py" "$ROOT" "$ROOT/.scratch/agents/imp/scale-kernel"` → rc 0, `anchors: 35 literals; 52 matches; 0 missing`. Each literal below is short/distinctive input to `rg -Fn`; `anchors.tsv` holds all matches, including repeated caller/clone sites. Complexity evidence = the enclosing loops/functions read, not the existence of a token alone.


#### S1

- `rust/ckc-kernel/src/k2_load.rs:155` — `let duplicate = path_contains(&out, &rows[i].pl);`
- `rust/ckc-kernel/src/k2_load.rs:449` — `let index = row_index_exec(rows, &paths[i]);`

#### S2

- `rust/ckc-kernel/src/k2_term.rs:32` — `pub nodes: Vec<ENode>,`
- `rust/ckc/src/v1_cli.rs:104` — `let pys: Vec<ESrc> = rows.iter().map(|r| read_src(&r.payload)).collect();`

#### S3

- `rust/ckc-kernel/src/k2_sort.rs:328` — `sorted.insert(i, root);`
- `rust/ckc-kernel/src/k2_sort.rs:462` — `out.insert(0, sorted[i]);`

#### S4

- `rust/ckc-kernel/src/k2_aggregate.rs:673` — `if !contains_root(arena, ys, xs[i]) {`

#### S5

- `rust/ckc-kernel/src/k2_aggregate.rs:969` — `let witnessed = crate::k2_bridge::witness_db_exec(&arena, &obs[i].facts, db);`
- `rust/ckc-kernel/src/k2_bridge.rs:555` — `let clause = copy_clause(arena, &db[j]);`

#### S6

- `rust/ckc-kernel/src/k2_index.rs:541` — `if args.len() == 0 || classless(arena, args[0]) {`
- `rust/ckc-kernel/src/k2_index.rs:705` — `let mut chain = match routes.remove(&h) {`

#### S7

- `rust/ckc-kernel/src/k3_machine.rs:541` — `let matched = crate::k2_engine::next_match(&arena, db, &name, args.len(), ci);`

#### S8

- `rust/ckc-kernel/src/k3_materialize.rs:330` — `let found = event_at(&arena, log, path);`

#### S9

- `rust/ckc-kernel/src/k3_join.rs:381` — `let count = crate::k3_coords::join_count_exec(coords, digests, &c, &h);`

#### S10

- `rust/ckc/src/v1_cli.rs:109` — `let lines = match ckc_kernel::contract::v1_trace_lines(`
- `rust/ckc-kernel/src/k3_impl.rs:30` — `let front = match crate::k3_front::front_exec(`

#### S11

- `rust/ckc-kernel/src/k2_answers.rs:1093` — `let result = crate::k2_engine::solve(`
- `rust/ckc-kernel/src/k3_rows.rs:454` — `let mut left = 1000000usize;`

#### S12

- `rust/ckc-kernel/src/k2_impl.rs:119` — `} else if crate::k2_recursion::holds_exclusion_exec(&arena, &loaded.db) {`

#### S13

- `rust/ckc/src/certify_cli.rs:279` — `let second = stage.dump(root, &ace, usha.is_some())?;`
- `justfile:996` — `for g in guidelines/*/; do "$bin" certify "$(basename "$g")"; done`

#### S14

- `rust/ckc-kernel/src/m6_project.rs:529` — `let roots = of_sentence(arena, tagged, &sn);`
- `rust/ckc-kernel/src/m6_expand.rs:946` — `out = join_bodies(arena, &out, &bs);`

#### S15

- `rust/ckc-kernel/src/m6_group.rs:550` — `let tail = roots_conds_from(arena, roots, i + 1);`
- `rust/ckc-kernel/src/m6_flat.rs:432` — `let r = flatten_list(arena, &a[1], w, env, deps, outer, encl, f1.n);`

#### S16

- `rust/ckc-kernel/src/k2_engine.rs:2840` — `let EPair { left, right } = pairs.remove(0);`
- `rust/ckc-kernel/src/v1_term_impl.rs:9438` — `while i < old_next`

#### S17

- `rust/ckc-kernel/src/m7_temporal.rs:881` — `let mut r = clone_tab(t);`
- `rust/ckc-kernel/src/m7_temporal.rs:1366` — `match add_row_exec(&t, &lines[i]) {`

#### S18

- `rust/ckc-kernel/src/k2_recursion.rs:302` — `buckets = census_row(arena, db, buckets, c);`

#### S19

- `rust/ckc/src/certify_cli.rs:411` — `let aggregate = ckc_kernel::contract::v1_aggregate_check(`
- `rust/ckc-spec/src/replay.rs:541` — `} else if !heads_proved(witness_db(obs[0].facts, db), obs[0].heads) {`

#### S20

- `rust/ckc-kernel/src/k3_state.rs:489` — `pub fn fail(arena: &ETermArena, initial: ECfg, Ghost(db_len): Ghost<nat>) -> (out: EStep)`
- `rust/ckc-kernel/src/k3_state.rs:539` — `c.log.truncate(log_len);`
- `rust/ckc-kernel/src/k3_rows.rs:337` — `crate::k3_state::EOut::Proved(log) => {`

reviewer-1-DONE-1
