# scale-semantics

Scope = cross-guideline semantics: clinical vocabulary; derived/formula/recursion/scope checks; identity + guards; native evaluation; SWI consumer recipe; schema/review evolution. Fixed checks = SC1 time, SC2 memory, SC3 depth, SC4 repeated work, SC5 gates, SC6 composition scope, SC7 human/process throughput, SC8 artifacts. 48 check × component rows adjudicated below.

Code/reference/corpus baseline = `9129d719`; 39 inspected code/reference/rule files byte-compared identical through `eda3f56c9b4e`. Spec rulings read at the baseline; main progressed live phase bookkeeping in parallel. Scale = CDC-like ×1/×100/×1,000, a stress scenario, not a prediction of heterogeneous guidelines. `C` = clauses, `D` = documents, `G` = guidelines, `U` = user-lexicon entries, `W` = Clex entries, `M` = manufactured entity keys, `A` = derived answers. Current census = 492 documents, 15,577 rule clauses, 3,268 fact lines, 9,617,517 compiled bytes; 2,025 ulex entries; 93 derived-entity rulings; 80 formula clauses. Findings = scalability revision needs under the all-guidelines/one-patient target, not claims that a current CDC answer is clinically wrong. Existing user rulings remain binding; changing one requires a user ruling.

All probe files = `.scratch/agents/imp/scale-semantics/`; repository target read-only. Severity: `major` = blocks the corresponding scaling/composition milestone; `minor` = avoidable algorithmic cost. F6 + F8 are established debts, independently checked here rather than new discoveries.

| id | component (file:line) | operation | check | scaling | cost ×1 | projected ×100 / ×1,000 | breaks at | revision needed | verdict |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| S1 | Clinical vocabulary — `rust/ckc-kernel/src/k4_lexicon.rs:57,341` | Shadow + lemma-liveness checks | SC1 | Derived: repeated surface scan O(UW), lemma scan O(U²); ruling scans add work | U=2,025, W=97,527; isolated time not measured | Per-guideline implementation repeats 100× / 1,000×; a naive global merged ulex worsens U² | Increasing vocabulary; no measured wall threshold | F1: index surface→readings + lemma→live surfaces; preserve existing reject order | revise |
| S2 | Clinical vocabulary — `rust/ckc/src/check/lexicon.rs:11` | Load ulex + ACE + Clex | SC2 | Derived: O(ACE bytes + U + W) peak per guideline, sequential across G | 157,751 ulex bytes + 3.3 MB Clex + one guideline's ACE | Peak need not grow with G in present loop | No G-driven memory failure established | Keep a compact shared terminology index; no separate memory revision proved | ok |
| S3 | Clinical vocabulary — `rust/ckc-kernel/src/k4_lexicon.rs:142,346` | Lexical validation loops | SC3 | Iterative over entries; no recursion proportional to G | Bounded per-entry parsing | No additional G-driven stack depth | None observed; deeply nested invalid lexical inputs not stress-tested | None specific to cross-guideline scale | ok |
| S4 | Clinical vocabulary — `rust/ckc/src/check/lexicon.rs:19`; `k4_lexicon.rs:453` | Rebuild common Clex indexes | SC4 | Full read + normal forms + surfaces rebuilt for each guideline and invocation | One common lexicon reconstruction | 100 / 1,000 reconstructions of identical Clex | Repeated unchanged work, already present | F1: content-keyed common Clex indexes; per-guideline delta validation | revise |
| S5 | Clinical vocabulary — `rust/ckc/src/check/mod.rs:50` | Gate lexical validation for all guidelines | SC5 | Serial per-guideline work; no cross-guideline shared lexical cache | Isolated lexical wall time unmeasured | Linear G scenario; whole check already exceeds 6 h well below ×100 (S45) | Exact lexical-only threshold unconfirmed | F1: indexed/incremental lexical checks inside sharded gate; unchanged grading | revise |
| S6 | Clinical vocabulary — `docs/REFERENCE.md:344`; `rust/ckc/src/check/lexicon.rs:5` | Clinical concept identity + patient vocabulary | SC6 | Annotation classes normalize units/grades/etc.; clinical noun/verb identity remains exact opaque atoms | Only one production guideline; no inter-guideline concept census exists | Two correct but differently normalized source vocabularies can fail to join; same atom can conflate distinct concepts | Before second guideline | F1: user-ruled cross-guideline clinical-term + patient-input vocabulary contract; reconciliation/disambiguation gate, not domain knowledge in engine | revise-before-2nd-guideline |
| S7 | Clinical vocabulary — `.claude/rules/corpus.md:25`; `docs/REFERENCE.md:1600` | Normalize new source terms | SC7 | Current process authors a guideline-local lexicon and consults its own rejects; no shared reconciliation step | One local terminology set | Editorial drift accumulates with G; pairwise-review time not measured | Before second guideline | F1: reusable canonical clinical terminology + explicit source-specific terms; clinician fidelity remains human judgment | revise-before-2nd-guideline |
| S8 | Clinical vocabulary — `guidelines/cdc-2022-opioid/lexicon.ulex:1`; `vocabulary.tsv:1` | Persist lexical/annotation tables | SC8 | Linear local text; no format-size cap found | Ulex 157,751 B; vocabulary 4,023 B | About 15.8 / 157.8 MB ulex; 0.40 / 4.0 MB annotation tables | No artifact limit established | Retain per-source mappings; shared registry need not duplicate whole ACE | ok |
| S9 | Structural checks — `rust/ckc/src/check/derived.rs:32`; `cycle.rs:275` | Reader counts + formula reachability | SC1 | Derived scan worst O(MD) due full reader-count scan per entity; formula scan bounded-depth branching + candidate-vector scans, not linear C | Derived 0.07 s / 4,152 KiB; formula reserved-slot run 7.18 s / 118,964 KiB | Present separate-guideline loop linear G; naive union can make reader work quadratic and formula reachability high-degree in branching | Globalizing the checks without changing representation | F3: per-noun reader counts/membership; typed dependency graph + reusable indexes/SCC reasoning | revise |
| S10 | Structural checks — `rust/ckc/src/check/cycle.rs:41` | Parse all clauses into a second term AST | SC2 | O(C + reachable-state memo) per scan | Formula scan 118,964 KiB RSS | If naively widened to one union: linear scenario ≈12 / 120 GB before extra reachability states; current per-guideline peak stays local | Before global composition scan at large G | F3: shared parsed representation / compact typed dependency summaries, not one duplicated whole-program AST per checker | revise |
| S11 | Structural checks — `rust/ckc/src/check/cycle.rs:14,280` | Reachability depth + term traversal | SC3 | Reach recursion capped at 7; parser/term helpers recurse with term nesting, not document count | No G-driven stack failure found | Same fixed graph-depth cap; does not become a complete cycle proof at larger G | No measured stack threshold; deep term stress not run | F3/F4 cover incompleteness/finiteness; no separate G-stack finding | ok |
| S12 | Structural checks — `rust/ckc/src/check/corpus.rs:29`; `documents.rs:180` | Re-read/reparse + replay forward/reverse | SC4 | Every invocation redoes derived + formula reads and native aggregate loads; formula memo is fresh per formula clause | All 492 documents scanned afresh | Repeated unchanged work scales at least with C; composition caching must include negative dependencies | Every corpus round | F2/F3: cached per-document dependency summaries + actual-composition validation; invalidate affected negative/positive components | revise |
| S13 | Structural checks — `rust/ckc/src/check/mod.rs:58`; `documents.rs:197` | Run semantic gate | SC5 | Guideline loop is serial; document tasks parallel within each guideline | Formula scan alone 7.18 s; full check-job timing supplied ≈15 min before fresh compile | Formula-only linear scenario ≈12 min / 2 h; full-check linear scenario ≈25 / 250 h, before new global work | 6 h full-check budget around two dozen CDC-sized guidelines in linear scenario | F2/F3: composition checker + incremental/sharded execution; not larger timeouts | revise |
| S14 | Structural checks — `rust/ckc/src/check/mod.rs:50,58`; `documents.rs:175` | Decide cross-guideline admissibility/replay | SC6 | Derived + formula scans and native replay/guard checks receive one guideline only; direct-left-recursion test itself is clause-local and composes | Per-guideline green says nothing about cross-guideline NAF/guard/derived/formula interactions | Reproduced two separately green inputs whose union rejects, in four mechanisms | Two guidelines | F2: validate actual loaded composition; full/selected-composition structural contract; do not enumerate all 2^G subsets | revise-before-2nd-guideline |
| S15 | Structural checks — `rust/ckc/src/check/derived.rs:37`; `.claude/rules/corpus.md:43` | Rule previously inert derived nouns | SC7 | New guideline readers can make old producer entities newly flagged; current per-guideline scan never presents these for ruling | 93 local rulings | New source can require justified changes to old source's ruled interaction set | Second guideline sharing a derived noun | F2: cross-guideline producer/consumer census + focused rulings; never blanket-waive phantom entities | revise-before-2nd-guideline |
| S16 | Structural checks — `rust/ckc/src/check/derived.rs:121` | Store derived-entity dispositions | SC8 | One row per producer document/sentence/noun, not per reader-pair | 93 rows | Repeated-corpus linear scenario 9,300 / 93,000 rows; extra newly exposed producer keys possible | No separate table-size limit established | Keep producer-keyed rows; reader index derived, not duplicated in the ledger | ok |
| S17 | Identity/guards — `rust/ckc/src/check/inventories.rs:224`; `ckc-spec/src/vocabulary.rs:518` | Enforce document IDs + scope keys | SC1 | BTreeMap uniqueness O(D log D); scope key checks local table rows | 492 globally unique document IDs | Same algorithm at 49,200 / 492,000 documents | No semantic collision or superlinear beyond log factor found | Preserve existing global uniqueness + scope-key law | ok |
| S18 | Identity/guards — `docs/REFERENCE.md:452`; `rust/ckc-spec/src/replay.rs:696` | Materialize Skolem witnesses | SC2 | New term per antecedent solution; term-generating cycles can have infinitely many distinct answers even with tabling | Two-rule counterexample exhausts 32 MB table cap in 0.11 s | No finite ×100/×1,000 memory bound follows from current admission checks | Two individually admissible source programs | F4: finite-term/admitted-recursion contract, typed term-generating dependencies + explicit resource/incompleteness semantics | revise-before-2nd-guideline |
| S19 | Identity/guards — `docs/REFERENCE.md:455,556` | Nested dependency identities | SC3 | Skolem Deps retain antecedent terms; a cross-guideline recursive cycle grows term depth without bound | Reproduced two-rule cycle | Increasing G also permits longer acyclic dependency chains; tabling alone supplies no depth bound | Two-rule cycle; exact native stack threshold unconfirmed | F4: distinguish harmless finite cycles from term-generating cycles; protect traversal/serialization as well as evaluation | revise-before-2nd-guideline |
| S20 | Identity/guards — `docs/REFERENCE.md:463`; `inventories.rs:224` | Reuse generated identities | SC4 | Deterministic role + DocId + sentence + ref/box + Deps; no global renumbering needed when another guideline is added | Existing stable names | Linear added IDs; no mandatory re-authoring of unrelated IDs | None observed | Preserve identity stability; cache by full semantic inputs | ok |
| S21 | Identity/guards — `rust/ckc/tests/consumer_recipe.rs:312` | Guard/date fixture executions | SC5 | Three date worlds reload whole corpus; patient-world test schedules eight more loads | Current one-guideline fixtures | F7 load/query cost repeats with G and with each new guard world | Before large compositions | F7: bounded workers + shared immutable program image / cached load, isolated patient/date state | revise |
| S22 | Identity/guards — `rust/ckc-spec/src/vocabulary.rs:518`; `docs/REFERENCE.md:1192,1265` | Exclusions + supersession across guidelines | SC6 | Scope key must equal guideline folder; document IDs globally unique; supersession keyed DocId/sentence; evaluation date intentionally composition-global | Distinct guidelines do not share a scope key accidentally | Metadata scheme supports many guidelines + one patient + one valid evaluation date | No namespace defect established | Preserve these invariants; broaden dependency analysis per F2, not rename everything | ok |
| S23 | Identity/guards — `guidelines/cdc-2022-opioid/vocabulary.tsv:97`; `.claude/rules/corpus.md:37` | Maintain supersession relationships | SC7 | Editorial census + explicit records; loading newer guidance does not itself author a record in an older document | Six table cases | About 600 / 6,000 cases under simple replication; actual future workload unconfirmed | No measured throughput threshold | Process risk: index source/concept/revision relationships; new cases still require a justified ruling. Automatic supersession is not promised | ok |
| S24 | Identity/guards — `docs/REFERENCE.md:452,556` | Emit dependency-bearing terms | SC8 | Finite source clauses can denote unboundedly many ever-deeper terms | Two-rule positive recursion reproducer | No finite derived-artifact/answer bound follows from source size | Two-rule composition | F4: output-size/finiteness contract; source/archive byte size is not a bound on derived entities | revise-before-2nd-guideline |
| S25 | Native evaluation — `rust/ckc-spec/src/engine.rs:23`; `rust/ckc/src/v1_cli.rs:159` | Enumerate patient recommendations | SC1 | Bounded depth-first SLD repeats branches; each charged call can include a large clause scan/unification | CDC + adult open-should query hits external 20 s cap; result not observed. Positive controls pass | Existing algorithm already misses requested finish line at ×1; no credible higher-scale wall forecast | ×1; known IMP scope | F8: verified tabled/bottom-up mode; cost-sensitive multi-argument indexes and composition reuse, not raised search fuel alone | revise |
| S26 | Native evaluation — `rust/ckc/src/v1_cli.rs:159` | Load composition + payload cells | SC2 | Whole input vectors then parsed program; shared-brief supplied baseline, not independently remeasured here | Supplied RSS 167 MB at ×1 | Linear supplied scenario 16.7 / 167 GB; derivation state extra | Already identified by main | F8: compact persistent/streamed program representation + reuse; avoid duplicated readable payload cells in modes that ignore payload contents | revise |
| S27 | Native evaluation — `rust/ckc-spec/src/engine.rs:31` | Derivation depth bounds | SC3 | Answer depth=100, trace=1,000, replay=4,000; bounded completeness, not a stack/finiteness proof | Explicit fixed budgets | Longer legitimate cross-guideline derivations may return limits even without a loop | Input-dependent; no newly measured exact G boundary | F8/F4: contract complete finite evaluation separately from explicit resource limits | revise |
| S28 | Native evaluation — `rust/ckc/src/check/queries.rs:350`; `v1_cli.rs:107,159` | Re-run answers/traces on same program | SC4 | At least 5 native invocations/query in check: 2 answers + 2 traces + trace-check; each reloads composition; trace has further binding passes | Q=6; all per-guideline today | Naively checking all Q against full C gives O(QC) parsing; Q∝G, C∝G ⇒ quadratic setup | Widening query validation to actual compositions without program reuse | F8: reusable immutable program/compiled indexes; cache key includes full composition + patient/date + semantics pin, including NAF dependencies | revise |
| S29 | Native evaluation — `rust/ckc/src/check/queries.rs:117`; `documents.rs:80` | Query/replay subprocess ceilings | SC5 | 30 s per answer/trace, 300 s aggregate; inference fuel does not bound wall-clock scans | Open native query already exceeds external 20 s cap in this probe | Larger compositions can fail wall caps before or after semantic bounds; exact full-gate crossing not measured | Input-dependent, already near one-call gate budget at ×1 | F8 + sharded incremental gates; retain detection strength | revise |
| S30 | Native evaluation — `rust/ckc/src/check/queries.rs:442`; `docs/REFERENCE.md:440` | Scope answers/replay to loaded facts | SC6 | NAF is composition-relative; committed native query manifest contains one guideline only | Real fork-produced NAF pair passes separately, fails union | Neither per-guideline green nor a cached guideline-local answer certifies another document selection | Two guidelines | F2/F8: actual-composition validation + answer provenance/keying; pin demonstrations to their declared composition | revise-before-2nd-guideline |
| S31 | Native evaluation — `.agent/spec.md:60`; `docs/REFERENCE.md:652` | Deliver usable patient enumeration | SC7 | User cannot finish an open clinical question by retrying fixed bounded SLD | Existing phase request explicitly names failure | At ×100/×1,000, manual narrower prepared questions are not a substitute for complete finite evaluation | Already ×1; known phase work | F8: satisfy the existing patient-engine acceptance set, then extend to compositional scale cases | revise |
| S32 | Native evaluation — `docs/REFERENCE.md:1488` | Store answers + first-proof traces | SC8 | O(answer/proof bytes), necessarily output-sensitive; finite source size alone insufficient under F4 | 6 committed demonstration queries | At fixed query density 600 / 6,000 queries; answer sets need not grow linearly | No independent file-format limit established | Keep composition/proof custody; address F4/F8 rather than truncate valid answers silently | ok |
| S33 | SWI recipe — `tests/consumer/driver.pl.in:35`; `rust/ckc/tests/consumer_recipe.rs:100` | Load + table open should goal | SC1 | Measured load roughly linear; query 137/538/2,212 ms at ×1/×2/×4 ≈ quadratic; output 289/614/1,372 boxes | Reserved-slot ×1 total 4.99 s, query 0.137 s | Quadratic overlap scenario query ≈22.8 min / 38.1 h; not a heterogeneous-corpus forecast | Modeled 120 s query cap around 30 copies | F7: workload/answer-size-aware evaluation, dependency isolation/indexes; measure before committing to table-all as scale solution | revise |
| S34 | SWI recipe — `rust/ckc/tests/consumer_recipe.rs:269` | Tables + multiple whole-program processes | SC2 | RSS ×1/×2/×4 = 46,972/77,556/135,624 KiB; eight world processes scheduled together | One should-goal process ≈46 MiB; full enum may use more | Simple observed-slope scenario ≈3 / 30 GB per process, ≈24 / 240 GB for eight; further nonlinear table state possible | CI host memory depends on runner; exact crossing unconfirmed | F7: bounded worker pool, immutable loaded program reuse, per-world state isolation; measure peak full suite | revise |
| S35 | SWI recipe — `docs/REFERENCE.md:556` | Tabled recursive terms | SC3 | Tabling terminates finite variant spaces, not an unbounded term-generating chase | Two-rule cap exhaustion reproduced | Same structural hazard regardless of nominal G scale | Two rules across two guidelines | F4: finiteness/admission analysis; no claim that tabling alone solves every composition | revise-before-2nd-guideline |
| S36 | SWI recipe — `rust/ckc/tests/consumer_recipe.rs:100,269` | Reload for each world / WFS rerun | SC4 | Full consult for each patient/mode; same corpus loaded repeatedly, no warm program image | 4 worlds × recipe/WFS = 8 loads; other tests add loads | Loading alone grows with G per process; new cases multiply it | Every gate run | F7: share/cold-cache immutable program preparation; bounded parallelism; retain independently checked WFS results | revise |
| S37 | SWI recipe — `tests/consumer/driver.pl.in:35`; `consumer_recipe.rs:119` | Gate broad enumerations | SC5 | 120 s per goal + 900 s process; broad overlapping query costs need not be linear in G | Query 0.137 s, four-copy 2.212 s | Overlap model breaches per-goal cap near 30 copies; ×100 query exceeds outer process cap too | Approx. 30 copies in measured-overlap scenario; extrapolated | F7: scale-graded consumer acceptance, explicit budgets, sharding/reuse; unchanged negative-loop and answer checks | revise |
| S38 | SWI recipe — `rust/ckc/tests/consumer_recipe.rs:249,294` | Validate global composition semantics | SC6 | Positive: loads every guideline. Defect: expects per-guideline answer pins unchanged globally and every excluded world to have no modal boxes from any guideline | Both assumptions pass with only CDC | Reproduced legitimate added recommendation ⇒ `differs`; independent applicable guideline ⇒ minor should=1 | Second relevant guideline | F5: source/composition-scoped expected answers + per-guideline exclusions; multikey/cross-guideline tests | revise-before-2nd-guideline |
| S39 | SWI recipe — `rust/ckc/tests/consumer_recipe.rs:260` | Select patient/negation test worlds | SC7 | Four fixed worlds (dose/adult/minor/cancer); observed WFS agreement is fixture evidence, not all-composition stratification | Existing four worlds useful, all run | Many conditions/guidelines require new interaction cases; exhaustive subsets exponential and not proposed | Before making general multi-guideline guarantee | F2/F5: typed structural dependency guarantee + focused negative-loop/comorbidity worlds; keep epistemic scope explicit | revise-before-2nd-guideline |
| S40 | SWI recipe — `tests/consumer/driver.pl.in:35,38` | Materialize/deduplicate all answers | SC8 | findall holds answers+delays, then copied goal list, sorted unique set; ≥O(A) memory, O(A log A) sorting plus term cost | 289 should contexts, ×4 1,372 | Output itself can be superlinear; finite tabled termination does not ensure acceptable materialization size | Workload-dependent, F4 allows infinite A | F7: incremental/canonical set hashing/streaming where semantics permit; resource contract must not silently omit answers | revise |
| S41 | Schema evolution — `docs/REFERENCE.md:328`; `justfile:979,996` | Re-derive all corpus + fixtures | SC1 | Required atomic corpus-wide schema migration; cold work at least O(input+output+certification), executed per guideline | 492 documents + questions + fixtures | 49,200 / 492,000 documents per migration under replication | Repeated schema/emitter changes as corpus grows | F6: content-keyed derivation, validated migration/preservation evidence + atomic versioned snapshot; retain one-identifier rule unless re-ruled | revise |
| S42 | Schema evolution — `rust/ckc/src/check/adjudication.rs:14`; `justfile:979` | Derive manifests/migrate per guideline | SC2 | Current shell iterates guidelines; no schema-policy requirement to hold every guideline's inputs at once | Review probe incl. two 492-doc derivations ≈20 MB RSS | Peak can stay per guideline; total I/O still grows | No independent G-driven memory failure found here | Stream/shard migrations; F6 invalidation cost separate | ok |
| S43 | Schema evolution — `justfile:979,996` | Iterate migration/certification work | SC3 | Shell/guideline loops, not G-depth recursion | One guideline iteration | No extra G-driven stack depth | None established | None specific to schema workflow | ok |
| S44 | Schema evolution — `.agent/spec.md:30`; `justfile:979` | Recompile unchanged inputs at every gate | SC4 | Full fresh compile/query pass plus check/certify; no persistent content-key cache for unchanged documents | Whole corpus repeated | At least linear G per round; a sequence of O(G) corpus additions can incur quadratic cumulative cold work | Ongoing corpus-extension loop | F6: incremental compile/certify keyed by all semantic pins; global interaction checks still run on changed dependency closure | revise |
| S45 | Schema evolution — `justfile:996`; `.claude/rules/rust.md:9` | Serial certify job + full check | SC5 | Per-guideline certify loop, no guideline fan-out; supplied CI cost ≈8 min/guideline + fixture battery | Supplied certify ≈8 min + ≈2 min cases; check ≈15 min before fresh compile | Certify linear scenario ≈13.3 / 133 h; check ≈25 / 250 h before fresh compile | 6 h certify budget around 45 CDC-sized guidelines; check sooner | F6: per-guideline/document CI fan-out + cache with fixed grading; retain global composition checks | revise |
| S46 | Schema evolution — `docs/REFERENCE.md:328`; `.agent/deferred.md:11` | Combine released document schemas | SC6 | User ruling: one identifier per composition; next schema identifier migrates all documents/questions | Schema 1 on release line; main's next semantic change is 2 | No mixed-old/new compatibility follows from current policy; consumers need an atomic snapshot + explicit compatibility/reject law | Next schema revision; before second guideline publication | F6: complete deferred version-law coverage + scalable snapshot/migration contract; do not silently introduce mixed schemas | revise-before-2nd-guideline |
| S47 | Schema/review custody — `rust/ckc-spec/src/check.rs:965,1030`; `adjudication.rs:31` | Restale human verdicts on compiler/schema edits | SC7 | Review v2 includes every non-comment/non-directive clause line, including schema-version fact | Reproduced schema-ID-only edit: ACE/passages/coverage unchanged for 492/492; review digest changes 492/492 | A global rewrite can demand 49,200 / 492,000 new human document decisions despite unchanged ACE/passage | Next global emitter/schema rewrite; queued before second guideline | F6: user-ruled decoupling already in deferred queue; certify clauses to reviewed ACE, re-prove custody | revise-before-2nd-guideline |
| S48 | Schema evolution — `docs/REFERENCE.md:326`; `guidelines/cdc-2022-opioid/pl/` | Versioned corpus/release bytes | SC8 | Raw output linear in corpus size; identity-only migration touches every file; Git delta compression not measured | 9.62 MB compiled bytes | ≈0.962 / 9.62 GB compiled bytes per raw snapshot; packed Git growth unconfirmed | No independent format/repository hard limit established | Preserve stable per-document artifacts; budget migration/export I/O in F6; archive implementation reviewed elsewhere | ok |

## Ranked summary

1. **Already ×1 / known phase work — F8:** native broad patient enumeration is not delivered by bounded SLD. Direct native open-should probe did not return within an external 20 s cap; do not confuse inference fuel with a wall-clock bound.
2. **Can fail at two guidelines — F2, F4, F5:** per-guideline green does not establish composition validity; term-generating mutual recursion defeats plain tabling; current recipe fixtures incorrectly silence every guideline / freeze each guideline's answers after extension.
3. **Before second guideline — F1, F6:** establish shared clinical-term/patient-fact identity and discharge the already-queued review-custody decoupling. Neither general-English lemmatization nor the annotation table is a clinical concept interoperability contract.
4. **Around 30 overlapping CDC-like copies, extrapolated — F7:** tabled should-query time approximately quadrupled for each doubling through ×4. A quadratic fit crosses the existing 120 s goal cap around ×30. This is an overlapping replication stress case, not a forecast for 30 arbitrary guidelines.
5. **Dozens of CDC-sized guidelines, linear timing scenario — F6:** existing serial check/certify jobs exceed the 6 h job ceiling before ×100. Per-guideline fan-out alone must not preserve the cross-guideline blind spot in F2.
6. **Large global checks — F3:** globalizing the current AST-based formula scan is not the fix: ≈119 MB already at ×1; repeated reader scans can become quadratic; depth-7 bounded reachability is a heuristic, not a finiteness proof.

## F1 — Clinical interoperability contract absent (`major`; S1/S4/S5/S6/S7)

Positive normalization already exists: time units, strength/evidence values, qualifier classes, preferences, scope keys. These do not give general clinical nouns/verbs a shared concept identity. Exact clinical atoms decide joins: `rust/ckc-spec/src/engine.rs:275` uses `a == b` for rigid non-compound terms. Per-guideline lexical validation receives one ulex, common Clex, that guideline's ACE and its Clex-shadow rulings; it does not receive other guidelines' lexicons or a patient vocabulary registry.

Exact text anchors:
- `docs/REFERENCE.md:344`: `Source words —`
- `docs/REFERENCE.md:345`: `nouns, verbs, adjectives, prepositions — stay opaque data atoms and`
- `rust/ckc/src/check/lexicon.rs:19`: `let clex = corpus_text(Path::new("vendor/clex/clex_lexicon.pl"), "clex-file")?.into_bytes();`
- `rust/ckc-kernel/src/k4_lexicon.rs:209`: `if shared(clex, names, &s) {`
- `rust/ckc-kernel/src/k4_lexicon.rs:356`: `if !lemma_live(ls, toks, &l) {`

Probe `compiler-probes.py`: identical `Every cohort-patient sleeps.` with separate admissible ulex mappings emits `guideline_entity(actual,A,patient,countable)` vs `guideline_entity(actual,A,adult,countable)`; both compile and proof modes rc 0. This proves that the emitter supports local lexical interpretation; it does **not** prove a clinically wrong production mapping. Only one production guideline exists. A shared concept contract, source-specific disambiguation and a patient-input vocabulary contract are missing design/authoring work, not an instruction to put an ontology into the engine. Faithfulness/equivalence judgments remain outside formal verification.

Algorithmic subfinding: `shared` linearly scans W surfaces for each ulex entry; `ruled` repeats that scan; `dead` calls a U-entry `lemma_live` scan for every entry. Provenance-neutral indexes can preserve the exact rejection order while avoiding UW/U² scans and repeated Clex reconstruction.

## F2 — Per-guideline green does not certify the consumed composition (`major`; S12/S14/S15/S30/S39)

Call chain: `check/mod.rs:50` → `corpus::check(g)` → `cycle::check(g)` + `derived::check(g)` (`corpus.rs:29–30`); `check/mod.rs:58` → `documents::check(g)` → manifest from that guideline's `g.docids` (`documents.rs:175–197`). `ckc compile` also aggregate/recursion-checks only its one guideline (`pipeline_emit.rs:150–154`).

Exact anchors: `cycle::check(g)?;`, `derived::check(g)?;`, `for g in &plans {`, `let pairs = par::map(&g.docids, |id| document(swipl, stage, g, &payload_dir, id))?;`.

Red evidence, all target mechanisms unchanged:

```text
# scan = scan.rs includes repository derived.rs + cycle.rs directly;
# only Guideline/filesystem/error-display scaffolding supplied by the probe.
scan derived derived/a     -> rc 0: derived-entities ok a 0 ruled
scan derived derived/b     -> rc 0: derived-entities ok b 0 ruled
scan derived derived/both  -> rc 1: unruled doc-a S1 patient (read by 1 documents)
scan cycle cycle/a         -> rc 0: formula-cycles ok a 1 formula clauses
scan cycle cycle/b         -> rc 0: formula-cycles ok b 0 formula clauses
scan cycle cycle/both      -> rc 1: reaches its own sentence ... doc-b:1 → doc-a:1
ckc v1 recursion-check scope/a.tsv    -> rc 0: 1 documents 1 rule clauses
ckc v1 recursion-check scope/b.tsv    -> rc 0: 1 documents 1 rule clauses
ckc v1 recursion-check scope/both.tsv -> rc 1:
ace_to_pl_error(proof,scope_condition(sentence('scope-b',1))).
```

Real compiler/verified replay counterexample, not a hand-written projection:

```text
naf-a ACE: If a man does not provably sleep then the man smiles.
naf-b ACE: Every man sleeps.
Each compile + proof rc 0, stderr empty.
ckc v1 aggregate-check compiler/a.tsv -> rc 0: 1 documents 1 obligations
ckc v1 aggregate-check compiler/b.tsv -> rc 0: 1 documents 1 obligations
ckc v1 aggregate-check compiler/both.tsv -> rc 1:
ace_to_pl_error(proof,obligation_failed('naf-a',1,variant(1))).
```

Important positive ruling: direct-left-recursion as currently specified compares each clause's own head and first goal; that property composes over a union. It is not a whole dependency-cycle guarantee. Scope admissibility and NAF replay are composition-dependent; derived/formula producer-consumer paths also cross guideline boundaries.

Acceptance: checker over the actual loaded manifest, with typed dependency summaries reusable across manifests and explicit positive/negative/term-generating edges; independently scoped corpus demonstrations stay tied to their composition. A full-union structural guarantee can cover subprogram structural properties where proven monotone, but a sampled full-union run is not proof for all patient worlds/subsets. Enumerating 2^G subsets is not a proposed solution.

## F3 — Globalizing current scans creates another scaling problem (`minor`; S9–S13)

`derived.rs:35` exact bytes: `.map_or(0, |r| r.iter().filter(|d| **d != doc).count())`. This runs for each manufactured entity during filtering. For M entities with one common noun read by D documents, it visits Θ(MD) reader entries. Compute total per noun minus own membership instead.

`cycle.rs:14` exact bytes: `const DEPTH: usize = 7;`. Each formula starts a fresh search; each reach builds a candidate vector by scanning a predicate/arity bucket (`cycle.rs:293–303`). Distinct unification/binding states defeat the failed-state memo; a fixed-depth walk still has high-degree growth in branching b (a loose bounded-body upper envelope O(F·C·b^7)). The cap makes graph recursion bounded, not completeness established.

Measurements of isolated repository functions through optimized `scan.rs`:
- derived live CDC: rc 0, 93 rulings, 0.07 s, 4,152 KiB RSS. This was not a reserved-slot timing; indicative only.
- formula live CDC under `heavy`: rc 0, 80 formula clauses, 7.18 s, 118,964 KiB RSS (`scan-cycle-heavy.log`). An earlier unreserved run was 11.65 s; projections use the reserved-slot run, not a median or benchmark claim.

Current per-guideline scans retain local peaks. The linear ≈12/120 GB scenarios apply only to a naive one-vector widening to ×100/×1,000. Better representation and incremental typed graph analysis are needed alongside scope correction.

## F4 — Tabling is not a finite-term guarantee (`major`; S18/S19/S24/S35)

Two positive schema programs, fresh output identities retain their triggering input:

```prolog
% skolem-a also has one ground amount seed.
guideline_entity(actual,'$guideline_id'(product,'skolem-a',1,ref(2),[A]),dose,countable) :- guideline_entity(actual,A,amount,countable).
guideline_entity(actual,'$guideline_id'(product,'skolem-b',1,ref(2),[A]),amount,countable) :- guideline_entity(actual,A,dose,countable).
```

Commands + target results:

```text
ckc v1 recursion-check skolem/a.tsv    -> rc 0: 1 documents 1 rule clauses
ckc v1 recursion-check skolem/b.tsv    -> rc 0: 1 documents 1 rule clauses
ckc v1 recursion-check skolem/both.tsv -> rc 0: 2 documents 2 rule clauses
swipl -q -f none -F none -s skolem-query.pl -> rc 1:
unbounded composition: error(resource_error(private_table_space),context(system:'$tbl_wkl_add_answer'/4,_76))
```

Measured 0.11 s, 54,148 KiB process RSS; table cap = 32,000,000 bytes. The exception's anonymous-variable number can differ. The 1 s fallback time cap protects the machine; the observed exception was table space, not time. Synthetic canonical programs demonstrate an admitted schema/recursion-check shape; this is not a claim that the current CDC contains that cycle.

Exact specification anchor: `docs/REFERENCE.md:556`, `a recursion that builds ever deeper Skolem terms can still run without end.` The current documentation states the limit honestly. The new scalable engine needs a finite/admitted evaluation profile or explicit resource/incompleteness semantics, rather than equating verified tabling with universal termination. Preserve harmless finite recursion where the contract permits it. The per-guideline derived scan can miss the producer/consumer link entirely (F2).

## F5 — Consumer fixture expectations do not compose (`major`; S38/S39)

Positive: `consumer_recipe.rs:100–109` loads every guideline. Existing scope keys are guideline-local; the test expectations are not.

### F5a — Exclusion of CDC incorrectly means exclusion of all guidelines

`consumer_recipe.rs:294–300` exact anchors: `if excluded_of[w] {` and `(n("should"), n("must"), n("may")),` followed by `(0, 0, 0),`. The count has no originating guideline filter.

Real fork compile `Every patient should sleep.` under a separate document ID; `ckc v1 check pediatric-guidance.pl` rc 0. Load it with CDC + the existing minor world using the documented tabling declaration. `swipl -q -f none -F none -s pediatric-query.pl` → `minor composition should=1`, rc 1 when applying the current zero-box expectation. CDC's age exclusion does not and must not block an independent guideline applicable to minors.

### F5b — Global answers compared to local demonstration pins

`consumer_recipe.rs:249` exact bytes: `let out = swipl(&s, &load(&s, Mode::Recipe, &[]), &calls.join(", "));`. Each answer file was derived by the per-guideline query path, yet this test compares it against every guideline loaded together.

Real fork compile `A recommendation is a category-A-recommendation.` under another document ID. Then:

```text
swipl ... -s committed-answer-control.pl -> rc 0:
answer 'category-a-recommendations' same
swipl ... -s committed-answer-query.pl -> rc 1:
answer 'category-a-recommendations' differs
```

The extra answer is legitimate open-world composition, not a regression in tabling. Scope the answer pin to its declared document set, or derive a composition-specific expected answer set. Preserve the check that tabling does not alter a question's answers on the *same* composition.

Coverage limit: four patient worlds + their WFS reruns show agreement for those worlds only. Broad positive/negative dependency admission needs a structural argument and focused cross-guideline/comorbidity cases; no finite fixture census proves all patient worlds.

## F6 — Whole-corpus schema migration also restales human review (`major`; S41/S44–S47)

Established deferred item: `.agent/deferred.md`, exact anchor `Review custody decoupling (PR5; due before the second guideline)`. The publication decision already says post-first-publication schema evolution needs an approach that scales. This audit confirms the debt, not a new contrary policy.

`ckc-spec/src/check.rs:965` retains every line except comments, directives and the document record; the schema-version fact remains. `bundle_block` (`check.rs:1030–1042`) includes `clauses `. `adjudication.rs:31–47` hashes that retained input and the resulting bundle.

Red test `python3 -I review-probe.py <root> <probe>` → rc 1:

```text
review-before rc= 0 stderr=
review-after rc= 0 stderr=
documents= 492 schema-lines-mutated= 492 ACE+coverage+passage-unchanged= 492 clause-digests-changed= 492 review-digests-changed= 492
FAIL: identity-only schema migration restales review custody
```

The edit is a simulated identifier migration in a scratch copy, not a claim that the current schema-1 reader accepts schema 2. `derive-review-manifest` accepts the byte change and proves the review invalidation mechanism directly. No real verdict or source artifact was edited. Projected re-review count = 49,200/492,000 documents if all change at the replicated scale. New clinical content still requires clinician review; the revision is to avoid redoing that judgment solely for certified compiler output changes.

Serial gate math uses supplied CI timings, not a new full run: `justfile:979` fresh compile/query loop and `:996` certification loop. About 8 min/guideline certification gives ≈13.3/133 h at ×100/×1,000; ≈45 CDC-sized guidelines reach a 6 h job even before other work. Approx. 15 min/guideline check yields ≈25/250 h before fresh compilation. Startup/cache/heterogeneity can change these estimates; they are linear scenarios. Verifier proof time is not extrapolated by G.

Acceptance: user-ruled review-vs-certification custody split, re-proved K4 chain, retained current verdicts after an unchanged-ACE/passage certified migration; content-keyed incremental derivation/certification + sharded gates; atomic one-identifier snapshots and explicit version-law tests. Existing one-schema-per-composition ruling stays in force unless the user changes it.

## F7 — Tabled recipe has query/output and multi-process scaling costs (`major`; S21/S33/S34/S36/S37/S40)

Command: `heavy python3 -I recipe-scale.py <root> <probe> 30`. Replicas use disjoint document IDs + scope keys, preserve clinical noun/verb atoms, load one original adult patient. One moderate SWI process at a time. All observed run stdout/stderr and timing summaries retained in `recipe-scale30.log` + `recipe-scale/{1,2,4}.{out,time}`.

| copies | documents | total s | load ms | should enumeration ms | peak RSS KiB | distinct should boxes | rc |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 492 | 4.99 | 4,822 | 137 | 46,972 | 289 | 0 |
| 2 | 984 | 7.74 | 7,163 | 538 | 77,556 | 614 | 0 |
| 4 | 1,968 | 16.60 | 14,353 | 2,212 | 135,624 | 1,372 | 0 |

Inference: enumeration is approximately quadratic in this overlap stress case (×3.93 then ×4.11 for two doublings), whereas loading is much closer to linear. Counts themselves exceed linear 289G by 36/216 boxes at ×2/×4; exact causal contributions of individual CDC rules were not localized. This is not a clinical-answer correctness finding. The registry/derived-entity interaction contract determines which cross-source joins are intended.

A quadratic fit from 137 ms gives 1,370 s at ×100 and 137,000 s at ×1,000; only ×1/×2/×4 were run. This is not a fit to heterogeneous clinical guidelines or a latency promise. The supplied driver caps the query body at 120 s (`driver.pl.in:35`), not the consult: the fit crosses that cap around ×30. An earlier 12 s *external probe* cap stopped ×4 with rc 124; rerunning with 30 s completed. That initial cap is not the product's 120 s limit.

Memory slope over these points is about 29 MiB per added CDC copy plus a fixed baseline. Extrapolating just that slope gives about 3/30 GB per process, before any worsening derived table population. `consumer_recipe.rs:269` schedules four worlds × two modes as eight threads/processes; other test functions add loads. A bounded worker pool is necessary before scaling; a patient/date-isolated warm immutable program can avoid repeated consults.

`driver.pl.in:35–41` also materializes `G-D` pairs with `findall`, copies the goals, sorts to a unique set, then walks delays. Cost depends on answer count/term size, not just source bytes. Streaming/set hashing can reduce intermediates only under a contract that preserves exact answers + undefined detection. Raising time/table limits is not the fix.

## F8 — Native engine's already-open scale gap (`major`; S25–S29/S31)

This is existing IMP scope, not newly requested product functionality. `engine.rs:23–44` specifies depth/inference budgets; the code's cost model charges an entire clause scan/head unification as one inference. The first-argument index helps, but many actual-context schema goals share `actual`; 22 predicate names and first-argument indexing do not partition common clinical noun/verb/operator joins by guideline.

Direct probe:

```text
timeout --kill-after=2 20 ckc v1 answer native-adult.tsv open-should.pl
-> rc 124, no answer result observed within external cap
```

Positive controls, with expected result named first:
- Same query + adult-only manifest: expected one `result(solutions([]))` line; observed that line, rc 0.
- CDC + adult composition `ckc v1 recursion-check native-adult.tsv`: expected 493 accepted documents / 15,577 rule clauses; observed exact counts, rc 0.

The adult fixture was copied with the canonical generated header for the native reader; its facts stayed unchanged. Initial use of the SWI fixture's explanatory header rejected noncanonical; that attempt is set aside, and only the canonicalized-input run supports the timing observation. No `indeterminate(limit)` result was observed in this probe; the existing spec/phase documents that bounded behavior, and this review does not claim otherwise.

The whole-composition loading RSS numbers in S26 are supplied by the shared brief, not duplicated by this agent. Additional compounding work: `queries.rs:347–406` redoes answer×2, trace×2 and trace-check per query; `v1_cli.rs:107–143` trace calls the line-digest binding then the trace/check binding. A naive move to a global manifest would multiply load/setup by every query. Preserve the public Prolog schema and proof/NAF custody while designing reusable verified finite evaluation.

## Reproduction + verification record

Environment prefix for every reproduction:

```sh
ROOT=~/Projects/cnl-ckc
P="$ROOT/.scratch/agents/imp/scale-semantics"
export PATH="$ROOT/.toolchain/bin:$ROOT/.toolchain/verus-x86-linux:$ROOT/.toolchain/cargo/bin:$PATH"
export RUSTUP_HOME="$ROOT/.toolchain/rustup" CARGO_HOME="$ROOT/.toolchain/cargo"
```

Regeneration (all writes confined to P): `python3 -I "$P/compiler-probes.py" "$ROOT" "$P"` then `python3 -I "$P/prepare.py" "$ROOT" "$P"`; build scanner `rustc --edition=2024 -O -o "$P/scan" "$P/scan.rs"`. Individual red commands in details use paths under P; prepend them explicitly when rerunning from another directory. Timing rerun = `heavy python3 -I "$P/recipe-scale.py" "$ROOT" "$P" 30`; review-invalidation red = `python3 -I "$P/review-probe.py" "$ROOT" "$P"`.

Performed: source/call-chain review; actual fork compile + proof probes; native K1/aggregate/recursion/answer CLI probes; unchanged scan-function harness; SWI tabled composition probes; positive controls for per-guideline answers/native query; ×1/×2/×4 tabled timing/RSS with load/query separation; scratch review-digest migration; Rust LSP document-symbol call succeeded. No full gate, `ckc check`, `ckc certify`, full Rust suite, or CI run launched. No source/schema/fixture/ledger/decision edits and no commits by reviewer-6. No website accessed.

Coverage limits: one real production guideline; synthetic cross-guideline counterexamples + replicated-overlap scale scenario; four-copy timing ceiling, one reserved-slot timing per scale, not statistical latency distributions. No 100/1,000-guideline execution; no exhaustive patient/composition enumeration; no generic finite-chase proof; no packed-Git-size measurement; exact future clinical terminology/supersession work unconfirmed. Targeted check modules read, not an adversarial correctness review of every module under `check/` or every clinical passage. Security audit out of scope.

reviewer-6-DONE-1
