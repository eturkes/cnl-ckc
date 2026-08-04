# Derivational coverage — planning basis (M9)

Goal: close three traceability gaps with minimal new trusted surface: 13 first-party Bash files / 7,015 lines plus 155 CI-YAML lines outside the derived graph; E-- call-marker inventory = 345 `[[`-bearing lines (338 executable invocations + 36 declarations across 374 markers), with executable targets able to reach unconstrained Python semantics; 10 hand-Prolog modules / 9,705 lines whose transformation, validation, execution, and replay laws lack machine-tied natural-language representation. Recommended order = **B typed call-target vocabulary → A derived acceptance runners → C NL-spec↔Prolog ties**: B stabilizes effects before A authors thousands of E-- runner lines; C then binds the pre-final v3 surface and later laws additively.

Non-binding catalog: each M9 plan adopts only the smallest subset serving its deliverable. Systems below are precedents unless explicitly recommended as dependencies. Project invariants dominate: raw-byte authorities; deterministic ordering/diagnostics/fresh-run output; fail-closed validation; no-network acceptance; regeneration-only correction; Linux/POSIX + Python 3.13 + SWI-Prolog 9.2.9.

## 1 Derived acceptance runners

### 1.1 Prior art and fit

Target = project-local E---authored runner compiled through E-- to Python, not a general framework. Preserve every suite's case set/order, fail policy, exact stdout/stderr bytes, exit/signal class, zero-write guarantees, perturbation reruns, transcript, and hard pass count before deleting Bash.

| system | transferable mechanism | fit verdict |
|---|---|---|
| [llvm-lit](https://llvm.org/docs/CommandGuide/lit.html) | unique scratch, per-suite env, timeout, pipefail, XPASS=failure | Borrow mechanisms; reject engine: default parallel/timing-informed order, executable Python config, ambient selectors, text/shell tests. [Testing guide](https://llvm.org/docs/TestingGuide.html). |
| [DejaGnu](https://www.gnu.org/software/dejagnu/manual/) | PASS/FAIL vs XFAIL/XPASS vs infrastructure UNRESOLVED/UNTESTED/UNSUPPORTED | Missing facility ≠ expected subject rejection; reject Expect/Tcl/remote machinery. [States](https://www.gnu.org/software/dejagnu/manual/Output-States.html). |
| [TAP 14](https://testanything.org/tap-version-14-specification.html) | plan detects truncation; bailout distinguishes infrastructure abort | Metadata only: UTF-8/line-ending normalization disqualifies TAP as byte oracle; custom consumer adds needless state/escaping/YAML. |
| [Sharness](https://felipec.github.io/sharness/) | owned trash directory, preserve-on-failure, `test_must_fail` | Borrow naming/scratch; retaining shell makes it unsuitable as destination. [Repository](https://github.com/felipec/sharness). |
| [Bats](https://bats-core.readthedocs.io/en/latest/writing-tests.html) | concise wrapped shell commands | Anti-fit: streams merge by default, command substitution strips trailing newlines, variables cannot retain arbitrary NUL bytes, pipeline/FD behavior is surprising. [Gotchas](https://bats-core.readthedocs.io/en/stable/gotchas.html). |
| [pytest](https://docs.pytest.org/en/stable/how-to/capture-stdout-stderr.html) | parametrized rows, temp paths, fd-level binary capture, rich diffs, strict XPASS | Borrow case records/diff display; reject discovery/plugins/capture/`conftest.py` execution surface and xfail semantics. [xfail](https://docs.pytest.org/en/stable/how-to/skipping.html). |

No surveyed engine states this project's byte/process/filesystem contract as precisely as the existing harnesses. Re-express; do not replace semantics.

### 1.2 Bash→Python/E-- hazards

| hazard | required law |
|---|---|
| environment | Mandatory complete `EnvSpec`; omission never inherits. Pin/clear PATH, HOME, temp, locale, TZ, hash seed, bytecode/Python path, tool keys. Copy only explicitly declared keys. |
| status | Preserve `Exited(code)`, `Signaled(sig)`, `SpawnError(kind)`; Python POSIX signal return codes are negative while shells commonly expose `128+sig`. Flatten only for legacy transcript parity. [subprocess](https://docs.python.org/3.13/library/subprocess.html). |
| pipelines | No `shell=True`; replace pipes with typed list/byte operations. Any irreducible pipeline uses argv vectors, retains every stage status, and declares pipefail. |
| errexit/traps | Translate every conditional/negation branch to explicit checks; `trap EXIT`→scoped `try/finally`; signal cleanup removes only owned scratch then preserves signal semantics. |
| streams | Separate bytes only; no `text=True`, locale decode, strip/split/re-encode. Compare bytes first; escaped preview only after mismatch. Spool unbounded streams. |
| processes | Explicit stdin/DEVNULL, close unrelated FDs, deadlock-safe capture, owned process group, descendant kill on timeout, no surviving background child. |
| filesystem | Exclusive owned scratch; no check-then-create; `lstat`/no symlink-following cleanup; sorted defined snapshots; temps on destination filesystem. |
| locale/path | Authored path text follows project encoding; observed payload stays bytes; ordering = defined byte/ASCII order, never ambient glob/collation/cwd/temp defaults. |
| determinism | Serialize first; rerun in independent scratch under cwd/locale/TZ/hash-seed perturbations; compare declared bytes, outcome, and tree. Parallelism waits for isolation/log-order proof. |
| parity | Compare structured per-case records; normalize only declared scratch-root tokens in runner metadata, never subject bytes. |

Minimal kernel: deterministic `Suite`/`case`; owned `scratch_dir`; shell-free `run_capture`; exact `run_expect` and positive-rejection `run_reject`; byte equality/hash; sorted tree snapshot/equality; deterministic corruption seed; perturbed repeat equality; private atomic file/tree publication; `finish()` asserting registered=executed=passed=expected. No raw subprocess, shell strings, ambient env enumeration, generic globbing, or unrestricted `os/pathlib` on the author surface.

### 1.3 Seeded-red pattern

A seeded red is a passing case whose subject must reject one controlled corruption—never XFAIL/TODO or “any nonzero”:

1. mutate known-good bytes with a named deterministic operation;
2. prove seed digest, corrupt digest, exact changed offset/count, and non-identity;
3. snapshot protected roots;
4. require exact process class + stdout/stderr bytes, or a narrowly justified canonical diagnostic record;
5. require no forbidden tree delta/temp remnant;
6. PASS only when all predicates hold.

Crash, usage error, missing executable, different diagnostic, substring-only match, or unchanged seed = failure. This proves the intended validator boundary still rejects the intended defect.

### 1.4 Pairing protocol

Each old/new suite runs on equivalent clean trees with the same logical labels and emits per-case `{label, outcome, stdout bytes, stderr bytes, before/after tree snapshots, selected file digests}`. Deletion gate = exact case-set/order + green records + seeded-red records + perturbation reruns + transcript/pass-count + post-suite repository/generated-tree cleanliness. Suite exit parity alone cannot detect skipped cases, accepted crashes, byte drift, or leaked files.

Recommended runner API borrows lit/Sharness naming but owns direct byte/process records. Port one suite at a time; retain representative parity records after deleting Bash.

### 1.5 Thin CI

Keep `.github/workflows/ci.yml` hand-authored as declared provider/bootstrap TCB. GitHub resolves workflows before jobs run, so a job cannot generate the workflow that should govern that run. Repository-script entrypoints are supported by [GitHub Actions](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/add-scripts); Rust and Kubernetes similarly delegate behavior to repository tooling ([Rust workflow](https://rust.googlesource.com/rust/+/47e2fec78d64aa5b49e5a963237dcf8daac0ba36/.github/workflows/ci.yml), [Kubernetes verify](https://pkg.go.dev/k8s.io/kubernetes/hack)).

YAML owns triggers, least permissions, timeout/concurrency, checkout, exact environment provisioning/verification, one regeneration-drift gate, and one derived-runner invocation. It owns no case matrix, pipelines, inline assertions, env mutation, or `continue-on-error`; the identical command runs locally. Dhall/haskell-ci-style generated YAML is justified only by a real future matrix, then committed + byte-drift-gated—not generated for immediate use. [Dhall→YAML](https://docs.dhall-lang.org/tutorials/Getting-started_Generate-JSON-or-YAML.html) · [haskell-ci](https://github.com/haskell-CI/haskell-ci) · [workflow limits](https://docs.github.com/en/actions/reference/workflows-and-actions/reusing-workflow-configurations).

**Adopt:** local byte runner + paired conversion + thin CI. **Reject:** framework engine adoption; XFAIL/TODO for mandatory rejection; merged/normalized subject streams; mechanical shell-pipeline ports; inherited env/cwd/temp; shared scratch/symlink-following cleanup; premature parallelism; Bash deletion from aggregate exit parity.

## 2 Typed call-target vocabulary

### 2.1 Escape-hatch governance

| precedent | law to transfer | limit |
|---|---|---|
| [Elm ports](https://guide.elm-lang.org/interop/ports) | one compiler-visible boundary, typed serializable data, centralized rich messages | Adopt compiler-recognized privilege, not arbitrary wrappers. [Limits](https://guide.elm-lang.org/interop/limits) · [kernel predicate](https://github.com/elm/compiler/blob/master/compiler/src/Elm/Package.hs). |
| [Safe Haskell](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/safe_haskell.html) | mechanically checked Safe author code, reviewed Trustworthy kernel, Unsafe excluded; CI-enforced mode | Adopt trust split; labels do not sandbox build/runtime tools. [Safe imports](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/safe_imports.html). |
| [Deno permissions](https://docs.deno.com/runtime/fundamentals/security/) | default deny, resource-specific grants, deny overrides | `allow-run`/FFI can reopen ambient authority; tool identity, argv, env, cwd, writable roots, and sandbox belong to the grant. [Reference](https://docs.deno.com/runtime/reference/permissions/). |
| [WASI capabilities](https://github.com/WebAssembly/WASI/blob/main/docs/Capabilities.md) | no ambient authority; host passes attenuable resource handles | Use `RootCap`/`FileCap`/`DirCap`/`ToolCap`, never strings that imply authority. [Preopens](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-tutorial.md). |
| [Nim FFI/backends](https://nim-lang.github.io/Nim/backends.html) | negative precedent: FFI, emit, dynlib, flags, and compile-time execution are separate escapes | Inventory/close every path; auditing `[[...]]` spelling alone is not closure. [Manual](https://nim-lang.github.io/Nim/manual.html). |
| [Dhall](https://docs.dhall-lang.org/discussions/Safety-guarantees.html) | no general FFI/effects; typed imports + integrity; total normalization | Removing open resolution is stronger than documenting it. [Imports](https://docs.dhall-lang.org/tutorials/Language-Tour.html). |

### 2.2 Decision and enforcement

Strict E-- recognizes semantic construct IDs—e.g. `hash.sha256_file`, `proc.run`, `fs.stage_tree`, `fs.commit_new_tree`, `process.argv`, `process.env_declared`—before Python name resolution. A construct is not a callable name even if its private backend lowers to one.

Single compiler-owned authority:

```text
ConstructSpec{id, arg_shape, result_shape, effect_class,
              authority_args, determinism_preconditions,
              error_model, backend_lowering}
```

Check exact ID/arity/record-enum shape; reject unknown/prefix/wildcard/alias/returned-callable/dotted-method/malformed-capability targets. Runtime validates containment, file kind, symlink policy, tool identity, argv schema, env-key set, and structured errors. Docs/tests derive from or cross-check the same table. Backend = private vendored runtime unreachable by dotted author calls; strict mode is mandatory in regen/CI and source cannot disable it.

Trust split: first-party `.emm` = Safe; compiler/runtime kernel = Trustworthy; arbitrary Python FFI = legacy Unsafe. Exact current-target manifest is only a freeze gate. Permanent allowlisting fails because names constrain neither argv/path/env/symlink/returned-object/error semantics nor library drift; prose is not compiler-enforced. `subprocess.run`, `os.*`, and `pathlib.Path.*` are authority families, not deterministic operations.

### 2.3 Deterministic vocabulary

Bazel declared actions ([hermeticity](https://docs.bazel.build/versions/main/hermeticity.html)), Nix clean build environments ([manual](https://nix.dev/manual/nix/2.35/store/building.html)), and Reproducible Builds' input catalog ([commandments](https://reproducible-builds.org/docs/commandments)) support one rule: construct the observable perimeter; do not subtract a few ambient values.

- Values: immutable `process.argv`; manifest-only `process.env_declared`; fixed `hash.sha256_bytes/file`; capability-bounded `fs.read_bytes`; byte-sorted `fs.list_sorted`; defined-metadata `fs.snapshot`.
- Ownership/publication: exclusive `fs.scratch`; contained `fs.stage_tree`; `fs.commit_new_tree` = validated complete stage + one same-filesystem rename to nonexistent destination; `fs.atomic_replace_file` = same-directory temp + full write/flush/close + replace + remnant cleanup.
- Spawn: `proc.run(ToolCap, Argv, CwdCap, EnvSpec, StdinSpec, TimeoutSpec) -> ProcessResult`, one executable/no shell, separate bytes, `Exited|Signaled|SpawnError`.

`ToolCap` binds a manifest enum to resolved executable identity, expected version/digest, and allowed argv schema. No ambient PATH lookup, arbitrary executable string, shell, self-runtime, or unconstrained interpreter cap. Necessary Python/SWI interpreters receive a separately declared source/input closure.

`proc.run` pins executable/version, argv boundaries, cwd, complete env/absence, PATH behavior, locale/TZ/hash seed/HOME/temp/Python settings, umask/output modes, stdin, binary streams, FD closure, timeout clock, whole-group termination, no retry, stable spawn errors, resource limits, and network/process sandbox policy. Claim Linux/POSIX + pinned runtimes only.

### 2.4 Publication guarantees

- Successful same-filesystem POSIX rename gives atomic namespace visibility; cross-filesystem may fail `EXDEV`. [POSIX](https://pubs.opengroup.org/onlinepubs/9799919799/functions/rename.html) · [Linux](https://www.man7.org/linux/man-pages/man2/renameat.2.html) · [Python `os.replace`](https://docs.python.org/3.13/library/os.html#os.replace).
- Visibility ≠ crash durability. Durable Linux replacement separately requires sync temp, rename, sync directory. [fsync](https://man7.org/linux/man-pages/man2/fsync.2.html).
- Rename is not a multi-file transaction. Publish one complete new tree where destination-nonexistent semantics suffice; otherwise specify manifest/recovery.
- Windows replacement/open-handle/metadata behavior differs; cross-volume copy/move is not atomic. No cross-platform claim without separate constructs/gates. [ReplaceFile](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilea) · [MoveFileEx](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexa).

Migration = freeze all escapes → classify pure/effect/delete → add table+private runtime+positive/negative gates → legacy audit mode emits inventory but cannot produce accepted artifacts → migrate existing E-- tools with parity → author A → delete open resolver at zero legacy targets. Retain unknown/near-miss/prefix/case/Unicode/returned-callable/shell/broad-fs seeds.

**Adopt:** typed constructs + capability values + machine contracts. **Reject:** permanent/prose allowlists, wildcards/factories, source-visible subprocess/shell/`os/pathlib`, inherited environment, tool-by-path spawn, cross-filesystem publication, or describing rename as durable/multi-file/cross-platform-equivalent.

## 3 NL-spec↔Prolog machine ties

### 3.1 Binding-system survey

C must represent every in-scope predicate, clause, error class, table row, and semantic law with a stable NL/spec row plus named machine evidence—without claiming parseable English proves procedural equivalence.

| system | actual tie + fit | maintenance / license |
|---|---|---|
| [ACE / APE](https://github.com/Attempto/APE) | ACE deterministically parses to DRS/logic forms; lexicons bind words/categories, not statements/existing clauses. **R1 high** as already-pinned text validator with separate target bindings; **R2 low** alone. [Interpretation](https://attempto.ifi.uzh.ch/site/docs/ace_interpretationrules.html). | non-archived/maintained; LGPL-3.0; SWI |
| [AceRules](https://acerules.petapico.org/) | ACE itself executes under courteous/stable semantics and yields traces. R2 conceptual for matching tables; adoption low: semantic/runtime/ID/byte mismatch, no external clause-coverage API. [Paper](https://attempto.ifi.uzh.ch/site/pubs/papers/kuhn07acerules.pdf). | non-archived, sporadic; LGPL-3.0 family |
| [AceWiki + Codeco](https://github.com/AceWiki/AceWiki) | reasoner subset vs explicit outside-fragment; Codeco checks grammar but application supplies semantics. Useful R1/R3 pattern; no clause tie; Java/OWL/JPL too broad. [AceWiki](https://arxiv.org/abs/0807.4623) · [Codeco](https://ceur-ws.org/Vol-622/paper1.pdf). | active; LGPL-3.0-or-later |
| PENG / PENG-ASP | bidirectional grammar maps CNL→logic/ASP→equivalent verbalization: R2 conceptual high. No public implementation/binary/software license found; ASP mismatch; no retrofit binding. [PENG](https://aclanthology.org/U09-1011.pdf) · [PENG-ASP](https://aclanthology.org/2021.cnl-1.5.pdf). | research system; unavailable vendor/license basis |
| [Logical English](https://github.com/LogicalContracts/LogicalEnglish) | templates/types compile/query as Prolog or partial s(CASP). R2 design prior art; R1 needs explicit external bindings; full adoption adds evolving runtime. [Project](https://logicalcontracts.com/logical-english/). | active; Apache-2.0 |
| [s(CASP) `#pred`](https://github.com/SWI-Prolog/sCASP/blob/master/prolog/scasp/pr_rules.pl) | machine-consumed predicate-pattern↔NL templates render justifications. Strong R1 surface idea only with strict validator: predicate-not-clause granularity; undefined/wrong-arity heads and placeholders may become silent dead renderers. [Justifications](https://arxiv.org/abs/2009.10238). | non-archived; Apache-2.0 |
| PROLEG | Prolog-like legal rules + derivation traces, but legal text→rule selection remains manual: positive trace and negative binding precedent. [Paper](https://research.nii.ac.jp/~ksatoh/juris-informatics-papers/jurisin2010-ksatoh.pdf). | no canonical licensed upstream; [MIT comparative study](https://github.com/liviorobaldo/compliancecheckers/tree/main/PROLEG) is not established upstream; public data lacks license |

No system combines retrofit clause IDs, reverse coverage, structural-drift detection, and byte-stable evidence. A local binding protocol remains necessary.

### 3.2 Executable-spec mechanics

| system | mechanism to steal | gap / status |
|---|---|---|
| [Gherkin/Cucumber](https://cucumber.io/docs/cucumber/step-definitions/) | zero/one/many step-definition resolution; undefined/ambiguous failure; scenario-row executions; Messages ID graph from AST row→execution→binding→result/source. [Schema](https://github.com/cucumber/messages/blob/main/jsonschema/messages.md). | Strongest pattern: explicit joins + set equalities. Reverse unused-definition coverage is implementation-specific; own it locally. Active; MIT. |
| [Concordion](https://concordion.github.io/concordion/latest/spec/Concordion.html) | named executable examples/rows; `ExpectedToFail` and `Unimplemented` self-invalidate | Borrow names/states; no reverse unused-method coverage; reject HTML/OGNL/JVM reflection. Maintained; Apache-2.0. [States](https://concordion.github.io/concordion/latest/spec/annotation/ImplementationStatus.html). |
| [FIT/FitNesse](https://fitnesse.org/FitNesse/UserGuide/WritingAcceptanceTests/SliM/DecisionTable.html) | table row→one result; missing method stays local/visible | Borrow row-local diagnostics; reject graceful multi-name lookup, positional identity, wiki/reflection runtime, absent reverse coverage. Maintained; CPL-1.0. |

Transfer = stable IDs and complete joins in both directions, not BDD phrasing.

### 3.3 Prolog docs/coverage and CNL-DSL precedents

- **PlDoc:** formal predicate/mode header + prose; parser checks syntax/mode vocabulary when errors enabled, not runtime modes/types/determinism, clause/error coverage, or semantic drift. Optional rendering only. [Comments](https://www.swi-prolog.org/pldoc/man?section=pldoc-comments) · [source](https://github.com/SWI-Prolog/packages-pldoc/blob/master/doc_process.pl). Active/in pinned SWI; vendoring needs file-level review because metadata mixes LGPL, BSD-style headers, and a GPL tag.
- [`library(check)`](https://www.swi-prolog.org/pldoc/man?section=check) finds code faults, not docs. [`library(prolog_coverage)`](https://www.swi-prolog.org/pldoc/man?section=prologcoverage) records static clauses/call sites and exposes hooks; useful execution evidence after mapping clause refs to explicit IDs, not the NL tie. Dynamic predicates are omitted; load/source shape matters.
- **LPdoc/Ciao assertions:** formal interface assertions feed analysis/checks/docs while prose remains separate; useful schema precedent, predicate-level and Ciao-coupled. [LPdoc](https://ciao-lang.org/ciao/build/doc/lpdoc.html/) · [assertions](https://ciao-lang.org/ciao/build/doc/ciao.html/AssrtLang.html). Maintained; LGPL/GPL materials.

CNL/rule-DSL evidence:

| precedent | transferable conclusion | status / license |
|---|---|---|
| Attempto executable specs | CNL can compile through DCG/DRS to Prolog; retrofit equivalence still needs source maps. [1995](https://arxiv.org/abs/cmp-lg/9507009) · [1996](https://arxiv.org/abs/cmp-lg/9603004). | historical R2 precedent |
| [CNL2ASP](https://github.com/dodaro/cnl2asp) / [CNLWizard](https://github.com/dodaro/CNLWizard) | grammar/compiler-as-data and visible symbol stages | ASP/solver + Lark/YAML/Python surface, incomplete source-map/reproducibility fit; active; Apache-2.0 / MIT |
| [RegelSpraak/ALEF](https://github.com/belastingdienst/ALEF) | domain-expert CNL→validated executable artifacts | strongest production R2 precedent; MPS/Java/Maven/domain stack too broad; active; EUPL-1.2. [Paper](https://aclanthology.org/2021.cnl-1.6.pdf). |
| [FRETish/FRET](https://github.com/NASA-SW-VnV/fret) | structured slots→temporal logic with machine-proved compiler semantics | excellent row/verified-compiler precedent, wrong domain/broad Electron stack; active; Apache-2.0. [Proof](https://arxiv.org/abs/2201.03641). |
| [SLEEC](https://sleec.github.io/) | trigger/response/timing/otherwise/unless DSL→CSP | useful condition/action/exception syntax; decisive checker needs activated FDR/tooling; active; EPL-2.0. [Semantics](https://arxiv.org/abs/2307.03697). |
| RuleCNL/SBVR/[Pine.js](https://github.com/balena-io/pinejs) | staged controlled rules→semantic IR→database constraints | executable-CNL evidence, wrong relational Node/SQL target; Pine active, Apache-2.0. [RuleCNL](https://arxiv.org/abs/1406.2096). |
| [Ott](https://github.com/ott-lang/ott) | one typed metalanguage generates human + proof artifacts | strongest single-source R2 shape though not CNL; reimplement narrow row→data/rendering pattern, do not vendor OCaml stack; active; BSD |

### 3.4 Recommended corpus and targets

Use one external deterministic corpus as source of truth; Prolog carries minimal stable target annotations. Container order: existing deterministic record grammar extended once/frozen > data-only Prolog terms via `read_term/3` (never `consult/1`; whitelist/reject directives; canonical write) > small line micro-CNL + local DCG for R2 tables > strict canonical JSON if already available. Avoid YAML.

```prolog
spec_row(id(lower_property_comparative),
         text(ace, Sentence, expected_parse_digest(Digest)),
         binds(all, [predicate(drs_to_ir, lower_property, 8),
                     clause(lower_property_comparative_1),
                     error_class(lower_property_degree)]),
         evidence([test(lower_property_comp_than),
                   replay(property_certificate)]),
         state(bound)).
```

`id` is permanent semantic identity. `text` = ACE, micro-CNL, or explicit plain; ACE replay proves its parsed representation, not correspondence to code. `binds(all|any, Targets)` has explicit aggregation; target kinds include predicate, clause, error/site, R2 row, replay/certificate law. `evidence` names checks that must emit binding events. `state` = bound, strict expected-fail, unimplemented, or `survivor(ReasonCode,Rationale,AlternateEvidence)` with finite reason codes.

Targets: predicate = canonical `Module:Name/Arity`; clause = explicit marker attached to exactly one following static clause by term position; errors/data/laws = stable registry IDs + AST-discovered sites. Duplicate/orphan markers, intervening terms, or multi-clause attachment fail. File/line is diagnostic metadata, never identity. Structural digest canonicalizes variables by first occurrence while preserving modules, controls, literals, functors, and clause order; changed digest invalidates the row until deliberate review.

### 3.5 Coverage validator

Let `S` = active rows, `B` = bindings, `T` = discovered in-scope targets, `E` = bindings exercised by named evidence, `X` = typed survivors.

1. Parse all rows/annotations; canonical unique IDs; one terminal result per active row.
2. Every binding resolves uniquely with matching kind/module/arity/signature unless explicit many-target semantics.
3. **Target coverage:** `T = resolved(B) ∪ X`; nothing in scope is silent.
4. **Reverse liveness:** full CI requires `B = E`; filtered runs cannot satisfy the gate.
5. Named tests/replays must finish and observe exact error/class/clause evidence; incidental global coverage is insufficient. Map SWI coverage/traces to clause IDs.
6. ACE rows replay through the pinned local pipeline and match expected DRS/IR digest.
7. Expected-fail must fail as declared; unimplemented executes nothing; survivor reason/alternate evidence is mandatory and self-invalidating.
8. Target AST changes/deletion invalidate dependent rows even when IDs remain.
9. Emit sorted timestamp/path/duration/randomness-free certificates with row, binding, target, result, source/artifact/tool digests, and exact set differences.
10. Positive controls must reject undefined target, duplicate clause ID, ambiguity, unused binding, unexercised row, stale digest, malformed ACE, wrong error class, and invalid survivor.

This combines Cucumber's ID graph with reverse coverage, structural drift, and exact evidence. PlDoc/coverage may feed it; neither defines completeness.

### 3.6 Rung heuristic and exclusions

- **R2 first:** finite declarative DRS-shape→constructor maps, tag/operator tables, error-condition/message maps, projection/validation matrices. Author typed rows/micro-CNL; compile to Prolog data under one small interpreter; render normalized NL from the same row.
- **R1:** recursive traversal, orchestration, meta-calls, cuts/commit, order-sensitive normalization, first-failure precedence, replay/certificates, I/O/serialization. Bind honest NL intent to target IDs/digests + exact tests; use ACE only where faithful.
- **R3 sparingly:** SWI/runtime facts, performance/resource guards, or procedural invariants whose CNL would imply false declarative equivalence. Require finite reason, rationale, alternate evidence; target stays in `T`.

C planning should decide whether banked validator centralization/unification lands before clause annotations; refactor only where it removes duplicate semantic targets and stabilizes long-lived IDs.

**Adopt:** external corpus + explicit targets/digests + join/set-equality validator + R2/R1/R3. **Reject:** PENG without source/license; surveyed CNL/BDD systems as runtime dependencies; PlDoc or s(CASP) as completeness proof; ACE rewrite of all procedural code; line/proximity/regex identity; generated Prolog without source maps/round-trip/drift gates; incidental coverage as traceability.

## 4 Integrated boundary

| surface | disposition candidate |
|---|---|
| E-- tools + acceptance | in graph: `.emm`→accepted Python, typed B constructs only |
| hand-Prolog semantics | C corpus + stable targets + exact evidence; selected R2 tables may become generated data |
| `.github/workflows/ci.yml` | declared thin provider/bootstrap TCB |
| fixture/source data | declared authored rows, validator/digest gated—not falsely called generated |
| `.agent/context-gauge.sh` | out-of-product developer instrumentation with rationale, unless separately ported by plan |
| docs/agent instructions | human/agent specification/process text outside executable derivation claim |

B closes arbitrary host-call semantics; A moves acceptance inside that closed model; C supplies the machine-tied human layer for the sanctioned Prolog boundary. M8 stays complementary: it can later certify answer semantics from frozen records while C covers ACE→IR→program transformations/explanation laws that M8 leaves human-audited. Minimal adoption avoids a second test framework, workflow generator, solver runtime, or broad CNL stack.