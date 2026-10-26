# Review ledger

Adjudication of judgment-bearing rows (`CLAUDE.md` `Engineering`: check set fixed before the diff is read; every row adjudicated; an accepted ruling holds until new evidence reverses it). Rows: `id | lens | check | verdict | evidence`. Verdicts: `open` → `pass` | `fail→fixed <sha>` | `info→deferred` (informational finding, `.agent/deferred.md` row) | `register` (outside the acceptance contract, noted).

Closed ledgers: `.agent/archive/review-sup.md` (supersession: consultant + unit reviews SUP1, SUP3, SUP4, visual QA, phase-close lenses A–E, SEC-1..SEC-7), `.agent/archive/review-sec.md` (security review of the release implementation: check set S1–P5 + trace T-M..T-E2), `.agent/archive/review-da.md` (dose-arithmetic: unit reviews DA1, DA2, DA3, phase-close lenses A–E, SEC-1..SEC-7), `.agent/archive/review-cs.md` (condition-scope: unit reviews CS0, CS1, CS2, phase-close lenses CL-A..D, SEC-1..SEC-5, close commit), `.agent/archive/review-nt.md` (numeric-threshold review: C1–C10, S1–S8, G1–G6, V1–V7, K1–K8, N-R1..N-R9, SEC-1..SEC-7), `.agent/archive/review-eso.md` (eso closing review E-01..E-11), `.agent/archive/review-implement.md` (M5 review R-01..R-14, IMPLEMENT close S-01..S-10, U2 C-01..C-04, M5.5 close U5-01..U5-05 — every row adjudicated).

## REV — code review of the implementation (pre-release + release)

Target = HEAD 0770ddb9 (tree 966966eb): code, tests, gates, config + docs. Outside: the corpus's ACE↔passage fidelity (the user's release review, Tasks PR8); security (the `Security review` body: a security defect met here = a finding MAIN works alone).
Lenses: B baseline = the full gate + each runnable `Artifacts` entry driven by its run command (a red step | a failed run = a finding); C correctness (each function read with its callers + callees; the language's classic pitfalls ruled out — Rust: panics on input, lossy casts, overflow, paths + symlinks + TOCTOU, exit codes, timeouts + signals, UTF-8; Prolog: unintended unification, cuts, error propagation, encoding; shell: quoting, `set -e` + pipefail; JS: async errors, decoding; wrappers route to the wrapped instance; defaults intended; setup + teardown symmetric), K cleanup (existing helpers reused; derivable state computed; near-duplicates merged; dead code removed; independent steps parallel; startup + hot paths lean; closures capture only what they use), A altitude (a special case on shared infrastructure → the fix due in the mechanism) = `CLAUDE.md` `Engineering` code diff lenses, whole module = the diff; S claim soundness + guarantee-vs-claim gaps (README, REFERENCE, `--help` + usage + error text, comments, `Artifacts`, rule statements vs behavior); V verification integrity (`Engineering`: each claimed behavior pinned by a check that can fail — seen red or mutation-credited, survivors ruled; skipped, ignored, deleted or demoted cases on a ruled `.agent/deferred.md` row; expected values independent of the implementation (AM6); grading checks unchanged); F spec fit = behavior vs `Intent` + each `Decisions` bullet; L conformance = each clause of `CLAUDE.md` + `.claude/rules/*` binding a tracked file (process-only clause → `n/a`; tool-decidable clause → its gate check runs + fires on a planted bad input (an automated plant in the gate, or a manual plant recorded in the row); none → a finding fixed by wiring the check; a compound bullet's row summarizes, its teammate table carries one sub-row per clause (`<bullet>.<k>`: binds a tracked file? tool-decidable? check name, plant evidence)).
Modules: cli = `rust/ckc/src/{main,v1_cli,align_cli,certify_cli,certify_cases_cli}.rs`; check = `rust/ckc/src/check/**`; ui = `rust/ckc/src/ui/**`; trust = `rust/ckc/src/trust.rs` + `rust/trust/**`; driver = `rust/ckc/prolog/drs_dump.pl`; spec = `rust/ckc-spec/**`; kernel = `rust/ckc-kernel/**` + `.agent/archive/generators/**` (proof review = Verus `--no-cheating` + trust-audit; read = `contract.rs` bindings, code outside `verus!`, test modules, generator replay); fork = `vendor/ape/**` + `vendor/clex/**`; schema = `guidelines/schema.pl`; recipes = `justfile`, `rust/kani/bootstrap.sh`, `rust/verus-canary/run.sh`, `rust/{verus,tools,kani}.lock`, `rust/deny.toml`, `rust/rust-toolchain.toml`, `rust/**/Cargo.toml` + `.cargo/config.toml`, `.gitleaks.toml`; ci = `.github/**`; probes = `tools/*.mjs`; harness = `rust/ckc-kani-harness/**` + `rust/verus-canary/{cases.tsv,*.rs}`; tests = `rust/ckc/tests/**` + `tests/**`; docs = `README.md`, `docs/**`, `guidelines/*/README.md`, `CITATION.cff`, `NOTICE`, `LICENSE`, `vendor/*/{PROVENANCE,README.md}`; agent = `.agent/{spec.md,spec-lint,deferred.md,review.md,queue.md,compendium.*}`, `.claude/rules/**`, `.gitignore`, `.gitattributes`, the `.agent/archive/` scripts a rule names as a live recipe; corpus = `guidelines/cdc-2022-opioid/**` (data, re-validated by `ckc check` + `ckc certify`).
Finding = severity (`blocker` | `major` | `minor`) + `file:line` + proof: a red test (command + failing output on HEAD) for behavior, anchor bytes `rg -Fn` matches for text. Fix = root cause + defect-class sweep, one scoped commit per class, fast gate before the commit + `just ci <class>` green on it; full gate on a clean tree at the closing commit. Fix classes ruled up front: pin drift of a crate or a `tools.lock` row = fixed here (`just bump-crate` | `bump-tool`); a Verus, Kani, SWI-Prolog or Node runtime bump = its own unit → `fail→routed` or a deferred row; a fix touching `ckc-spec` | `ckc-kernel` beyond comments or dead-code deletion runs the `rust.md` kernel unit flow (contract, tester, reviewer, consultant) or becomes `fail→deferred` with its acceptance check. REV verdicts add `fail→deferred` (blocked on the user: `.agent/deferred.md` row), `fail→routed`, `n/a` (+ reason). Teammate verdict tables = `.scratch/agents/reviewer-<n>.md` (leads; MAIN's own check decides each row).

### Baseline

| id | lens | check | verdict | evidence |
|---|---|---|---|---|
| B-0 | baseline | `just gate` on HEAD's tree: every step green (CI on the identical tree = the gate record, `ops.md` CI gate) | pass | CI run 40282651714 on 62f32d4a (tree 966966eb = 0770ddb9 = the target code tree; 824921de adds .agent only): rust job fmt, clippy, verify, build, trust (`ckc: trust spec=17687 shell=48 assumes=65 deps=ok`), test (50 binaries, 689 passed, 0 failed, 2 ignored), deny, secrets, workflows, mobile (495 pages 0 bad), build-static; check job (`spec-lint: ok`, `ckc: check ok … 221 red probes 31 green probes (6 proof)`, `ui: ok 1 guidelines 495 pages`); certify job (`ckc: certify ok cdc-2022-opioid 492 documents 6 queries`, `ckc: certify cases ok 55 mutants 15 baselines`); log .scratch/review/ci-40282651714.log |
| B-1 | baseline | `ckc ui serve` (scratch clone): sign-in from the printed address, guideline + document GET 200, one POST decision → 303 + exact ledger row, each page kind screenshotted + inspected, stop → port refuses | pass | review worktree of a scratch clone (release/kb-2026-10-26 @824921de), `ckc ui serve 8899 <worktree>`: unauthenticated GET 403; GET /session/<64hex> 303 + cookie; /, guideline, 2 document pages, records 200; POST decision → 303 + ledger row `cdc2022-opioid-rec01 349d15be… 824921de… approved Dry Run 2026-10-26T08:35:20Z release dry run`; page then shows Approved + tally; screenshots .scratch/review/reldry/shots/shot-{1..5}-1280.png inspected (finding M-F1: "Reviewers recorded 1 decisions"); kill → port refuses (curl rc 7) |
| B-2 | baseline | `ckc ui render <out>`: rc 0, one page per route, a second render byte-identical | pass | `ckc ui render` twice → rc 0, 495 pages, `diff -r` identical (11 s) |
| B-3 | baseline | `ckc ui check`: rc 0 + its green line | pass | CI check job: `ui: ok 1 guidelines 495 pages` |
| B-4 | baseline | `ckc ui request GET <path>` (guideline + document): rc 0, body = the rendered page | pass | `ckc ui request GET` guideline + rec01 document → rc 0, HTTP 200, bodies `cmp`-equal to the rendered pages |
| B-5 | baseline | `ckc compile <id>` (scratch clone): rc 0, `guidelines/` byte-identical to HEAD | pass | CI check job (`just check` on tree 966966eb): `ckc compile cdc-2022-opioid` → `ckc: compile ok 492 documents`, then `git diff --quiet -- guidelines/` → `gate: check ok` (byte-identical) |
| B-6 | baseline | `ckc queries <id>` (scratch clone): rc 0, byte-identical | pass | CI check job: `ckc queries cdc-2022-opioid` (6 questions written) then `git diff --quiet -- guidelines/` → `gate: check ok` |
| B-7 | baseline | `ckc check`: rc 0 + every meter | pass | CI check job: `ckc check` → `ckc: check ok 1 guidelines 492 documents 221 red probes 31 green probes (6 proof)` + every meter (coverage, vocabulary, lexicon, supersession, compendium, adjudication, dist, fork notices, recursion, agent-spec) |
| B-8 | baseline | `ckc align <id> <docid>` over a committed document's rows: rc 0, output = its committed align file | pass | committed align/cdc2022-opioid-rec01.tsv → occurrence rows (.scratch/review/b/align-rederive.py) → `ckc align cdc-2022-opioid cdc2022-opioid-rec01` in a clone: rc 0 `groups 21 spans 59`, git status clean (file byte-identical) |
| B-9 | baseline | `ckc review-manifest <id>`: rc 0, digests agree with the ledger's nonempty rows | pass | `ckc review-manifest cdc-2022-opioid` in a clone: rc 0, 492 documents, git status clean; ledger empty at HEAD (custody re-derivation = `ckc check`, CI green) |
| B-10 | baseline | `ckc release-manifest`: output = committed `release-manifest.tsv` | pass | `ckc release-manifest` at HEAD (via `just amend-manifest`): no diff (`amend-manifest: current`) |
| B-11 | baseline | `ckc dist build <dest>`: rc 0, archive + sidecar, a rebuild byte-identical, contents per REFERENCE § Export | pass | `ckc dist build` rc 0 in two clones + twice in one: archive sha256 70c08d87… each time (`cmp` identical); bag `sha256sum -c` 1523 OK; sidecar OK |
| B-12 | baseline | `just release-ckc <dest>`: asset + sidecar, sidecar sha256 = the asset's | fail | M-F2: `just release-ckc` (pool host, HEAD) → rc 0, `gate: build-static ok`, sidecar `ca43939d…  ckc-x86_64-linux` matches the asset; the same code tree built by CI (run 40282651714) = de4093f2… (build-id note + segment layout differ: host linker) — the "same bytes on every checkout" claim fails across hosts |
| B-13 | baseline | `ckc certify <id>` + `ckc certify --cases tests/certify/cases.tsv`: rc 0 | pass | CI certify job: `ckc: certify ok cdc-2022-opioid 492 documents 6 queries`, `ckc: certify cases ok 55 mutants 15 baselines` |
| B-14 | baseline | `just tools` + `just swipl`: idempotent rerun, rc 0, nothing reinstalled | pass | `just tools` rc 0 in 1.1 s (`tools: ok …`, nothing reinstalled), `just swipl` rc 0 in 0.1 s (`swipl: ok SWI-Prolog version 10.0.2`); only the shims rewritten |
| B-15 | baseline | `just outdated`: rc 0 (`held` only); drift = a finding | open | |
| B-16 | baseline | `just kani`: rc 0 | open | |
| B-17 | baseline | `just late-mutants`: rc 0, each row at its pinned first violation | open | |
| B-18 | baseline | `just mutants` on one small credited target: runs, `mutants.out/` written, rc per cargo-mutants | open | |
| B-19 | baseline | bump recipes at their pinned versions (scratch clone, network): `bump-tool`, `bump-kani`, `bump-verus`, `bump-crate` → lock files byte-identical | pass | scratch clone, CKC_TOOLCHAIN = the primary toolchain: `just bump-tool` zizmor 1.31.1 (GitHub digest), rustup-init 1.29.2 (rustup .sha256), node 26.12.1 (SHASUMS256), chrome-headless-shell 156.0.8078.47 (GCS md5) → rc 0, tools.lock unchanged; `just bump-kani 0.68.0` rc 0, kani.lock unchanged; `just bump-crate crc32fast 1.5.3` rc 0, tree unchanged; bump-verus exercised to the new release in unit REV-V (c0a773f3) |
| B-20 | baseline | `ckc` CLI surface: no args, an unknown subcommand, each subcommand's usage error → usage text + documented rc | pass | `ckc` with no args, `nonsense`, `--help`, `-h`, `help`, `check a b`, `certify`, `align-check`, `certify-one` → rc 2 + the usage line; each subcommand alone → its own `usage: expected: …` rc 2 (compile, queries, align, review-manifest, release-manifest extra, ledger-validate, derive-review-manifest, dist, dist frob, ui, ui frob, v1, v1 frob); `trust-audit --frob` → rc 2 `no Cargo.toml at --frob`. Register: `--help` answers rc 2 |
| B-21 | baseline | `just amend-manifest` on clean HEAD: no change | pass | `CKC_BIN=<HEAD ckc> just amend-manifest` → `amend-manifest: current`, HEAD unchanged |
| B-22 | baseline | REFERENCE § Releasing dry run (scratch clone, throwaway key in a scratch `allowed_signers`): steps 1–5 (line branch, signed candidate tag verified per "To verify a release", `ckc dist build` + `just release-ckc`, review worktree + its `ckc ui serve`) + the merge-back step | pass | scratch clone, throwaway ed25519 key + scratch allowed_signers: branch release/kb-2026-10-26; `ckc dist build` + static program (CI artifact; `just release-ckc` = B-12) + sidecars; notes + `git tag -s` → `verify-tag` "Good git signature"; notes body \| `sha256sum -c --ignore-missing` → 2 OK + "5 lines are improperly formatted" (as documented); bag 1523 OK; `git worktree add` + `CKC_TOOLCHAIN=… just build` in the worktree → `gate: build ok`; `ckc ui serve` there (B-1); batch commit + `just amend-manifest` → amended; main moved + merge --no-ff → manifest conflict → ours + commit + amend-manifest → merge commit with both parents, manifest current; `git worktree remove` ok. Text mismatch (notes list 2 of 4 published files) = R7-F7 |
| B-23 | baseline | `just ci <unit>` on the closing commit: green | open | |

### Lens × module

| id | lens | check | verdict | evidence |
|---|---|---|---|---|
| C-cli | correctness | cli: lens C as defined above | open | |
| K-cli | cleanup | cli: lens K as defined above | open | |
| A-cli | altitude | cli: lens A as defined above | open | |
| S-cli | claim soundness | cli: lens S as defined above | open | |
| V-cli | verification integrity | cli: lens V as defined above | open | |
| C-check | correctness | check: lens C as defined above | open | |
| K-check | cleanup | check: lens K as defined above | open | |
| A-check | altitude | check: lens A as defined above | open | |
| S-check | claim soundness | check: lens S as defined above | open | |
| V-check | verification integrity | check: lens V as defined above | open | |
| C-ui | correctness | ui: lens C as defined above | open | |
| K-ui | cleanup | ui: lens K as defined above | open | |
| A-ui | altitude | ui: lens A as defined above | open | |
| S-ui | claim soundness | ui: lens S as defined above | open | |
| V-ui | verification integrity | ui: lens V as defined above | open | |
| C-trust | correctness | trust: lens C as defined above | open | |
| K-trust | cleanup | trust: lens K as defined above | open | |
| A-trust | altitude | trust: lens A as defined above | open | |
| S-trust | claim soundness | trust: lens S as defined above | open | |
| V-trust | verification integrity | trust: lens V as defined above | open | |
| C-spec | correctness | spec: lens C as defined above | open | |
| K-spec | cleanup | spec: lens K as defined above | open | |
| A-spec | altitude | spec: lens A as defined above | open | |
| S-spec | claim soundness | spec: lens S as defined above | open | |
| V-spec | verification integrity | spec: lens V as defined above | open | |
| C-fork | correctness | fork: lens C as defined above | open | |
| K-fork | cleanup | fork: lens K as defined above | open | |
| A-fork | altitude | fork: lens A as defined above | open | |
| S-fork | claim soundness | fork: lens S as defined above | open | |
| V-fork | verification integrity | fork: lens V as defined above | open | |
| C-recipes | correctness | recipes: lens C as defined above | open | |
| K-recipes | cleanup | recipes: lens K as defined above | open | |
| A-recipes | altitude | recipes: lens A as defined above | open | |
| S-recipes | claim soundness | recipes: lens S as defined above | open | lead: `just --list` shows sentence fragments (just reads a doc comment's last line) for build-static, release-ckc, test, kani, late-mutants, mutants, mobile, check, certify |
| V-recipes | verification integrity | recipes: lens V as defined above | open | |
| C-driver | correctness | driver: lens C as defined above | open | |
| K-driver | cleanup | driver: lens K as defined above | open | |
| S-driver | claim soundness | driver: lens S as defined above | open | |
| V-driver | verification integrity | driver: lens V as defined above | open | |
| C-kernel | correctness | kernel: every `contract.rs` binding the shell calls ensures equality with a spec fn (no hollow binding); code outside `verus!` + the test modules read with their callers | open | |
| K-kernel | cleanup | kernel: generators replay identical (`.agent/archive/generators/replay`); dead modules + unused bindings removed | open | |
| S-kernel | claim soundness | kernel: lens S as defined above | open | |
| V-kernel | verification integrity | kernel: Verus `--no-cheating` over the workspace, the canary, trust-audit escape + manifest pins: each runs in the gate + fires on a planted bad input | open | |
| C-schema | correctness | schema: lens C as defined above | open | |
| S-schema | claim soundness | schema: lens S as defined above | open | |
| V-schema | verification integrity | schema: lens V as defined above | open | |
| C-ci | correctness | ci: lens C as defined above | open | |
| K-ci | cleanup | ci: lens K as defined above | open | |
| S-ci | claim soundness | ci: lens S as defined above | open | |
| V-ci | verification integrity | ci: lens V as defined above | open | |
| C-probes | correctness | probes: lens C as defined above | open | |
| K-probes | cleanup | probes: lens K as defined above | open | |
| S-probes | claim soundness | probes: lens S as defined above | open | |
| V-probes | verification integrity | probes: lens V as defined above | open | |
| C-harness | correctness | harness: lens C as defined above | open | |
| S-harness | claim soundness | harness: lens S as defined above | open | |
| V-harness | verification integrity | harness: lens V as defined above | open | |
| C-tests | correctness | tests: each harness asserts what its name + rules entry claim; fixtures materialize + clean up symmetrically; no test passes vacuously (empty case lists, unmatched globs, swallowed errors) | open | |
| K-tests | cleanup | tests: lens K as defined above | open | |
| V-tests | verification integrity | tests: lens V as defined above | open | |
| S-docs | claim soundness | docs: README, REFERENCE, gap taxonomy, guideline README, CITATION, NOTICE, PROVENANCE: every claim vs the behavior + artifacts it describes (counts, commands, file names, guarantees); human-facing register (`Authoring` `Human-facing`) on shipped text | open | |
| K-docs | cleanup | docs: stale, duplicated or superseded passages; every pointer resolves | open | |
| S-agent | claim soundness | agent: rules + spec `Artifacts` + deferred rows vs the code they describe (law vs code mismatch → fix the stale side); every named path, recipe, test + count resolves | open | |
| K-agent | cleanup | agent: dead law, origin stories, duplicated law across rules files; scratch-only paths named as live recipes | open | |
| V-corpus | verification integrity | corpus: every corpus law `ckc check` enforces fires on a planted bad input (check-mutants + red probes); the committed pins re-derive | open | |

### Spec fit

| id | lens | check | verdict | evidence |
|---|---|---|---|---|
| F-I1 | spec fit | `Intent` goal 1 (executable KB a CDS can query + combine: conditions, timing, severity, strength, preference kept) vs the schema + engine + corpus | open | |
| F-I2 | spec fit | `Intent` goal 2 (clinician-auditable compilation: ACE beside its passage, coverage plan tracking gap classes, review before distribution as easy as possible) vs the reviewer UI + coverage ledger + release flow | open | |
| F-I3 | spec fit | `Intent` goal 3 (verified-kernel pattern: small human-read spec, AI impl + proofs, pinned checker, escape audit; custody chain; translation validation of every document + question; trusted software fixture-gated) vs the workspace + gates | open | |
| F-I4 | spec fit | `Intent` scope (fetch, normalize, compile, review locally, export deterministically; no serving layer; no LLM at run time; human-read = ACE + spec + named Prolog closure) vs the shipped surface | open | |
| F-D1 | spec fit | `Decisions` bullet 1 (Product) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D2 | spec fit | `Decisions` bullet 2 (Language boundary) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D3 | spec fit | `Decisions` bullet 3 (Consumption→Rust) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D4 | spec fit | `Decisions` bullet 4 (Verification line) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D5 | spec fit | `Decisions` bullet 5 (Search order) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D6 | spec fit | `Decisions` bullet 6 (Review custody) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D7 | spec fit | `Decisions` bullet 7 (Spine) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D8 | spec fit | `Decisions` bullet 8 (Schema evolution) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D9 | spec fit | `Decisions` bullet 9 (Strength class) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D10 | spec fit | `Decisions` bullet 10 (Authoring principle) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D11 | spec fit | `Decisions` bullet 11 (Numeric threshold) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D12 | spec fit | `Decisions` bullet 12 (Prohibitions) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D13 | spec fit | `Decisions` bullet 13 (Condition scope) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D14 | spec fit | `Decisions` bullet 14 (Dose arithmetic) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D15 | spec fit | `Decisions` bullet 15 (Review UI) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D16 | spec fit | `Decisions` bullet 16 (Publication scope) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D17 | spec fit | `Decisions` bullet 17 (Supersession) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D18 | spec fit | `Decisions` bullet 18 (Corpus) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D19 | spec fit | `Decisions` bullet 19 (Deps) vs behavior: each binding sentence traced to the code path or check that implements it | open | |
| F-D20 | spec fit | `Decisions` bullet 20 (Review) vs behavior: each binding sentence traced to the code path or check that implements it | open | |

### Conformance (`CLAUDE.md` bullets binding no tracked file = process-only, outside the set: Collaboration 1–4, Execution 2–4 8 9, Authoring 4, Engineering 4–6 9 15 16 18 19, Teammates 1–8, Session flow 5 9–14)

| id | lens | check | verdict | evidence |
|---|---|---|---|---|
| L-ops-1 | conformance | `.claude/rules/ops.md` bullet 1 (Toolchain (dev machines): a system swipl ≠…) | open | |
| L-ops-2 | conformance | `.claude/rules/ops.md` bullet 2 (Rust tooling = just tools (sha256-pinned installs…) | open | |
| L-ops-3 | conformance | `.claude/rules/ops.md` bullet 3 (Deterministic SWI: always -f none -F none…) | open | |
| L-ops-4 | conformance | `.claude/rules/ops.md` bullet 4 (Standing decisive chain = just gate (root…) | open | |
| L-ops-5 | conformance | `.claude/rules/ops.md` bullet 5 (CI gate (CLAUDE.md Execution) = just ci…) | open | |
| L-ops-6 | conformance | `.claude/rules/ops.md` bullet 6 (Release lines (procedure = docs/REFERENCE.md § Releasing…) | open | |
| L-ops-7 | conformance | `.claude/rules/ops.md` bullet 7 (Corpus loop (ruling on CLAUDE.md Session flow…) | open | |
| L-ops-8 | conformance | `.claude/rules/ops.md` bullet 8 (Unit contracts (IMPLEMENT + MAINTAIN): .agent/contracts/<unit>.md =…) | open | |
| L-ops-9 | conformance | `.claude/rules/ops.md` bullet 9 (Authority map: simplified overview → root README.md…) | open | |
| L-ops-10 | conformance | `.claude/rules/ops.md` bullet 10 (Scratch/probe work → gitignored .scratch/; standalone git…) | open | |
| L-ops-11 | conformance | `.claude/rules/ops.md` bullet 11 (WebSearch safety layer false-positives on medical vocabulary…) | open | |
| L-ops-12 | conformance | `.claude/rules/ops.md` bullet 12 (Presentation rule (user): durable files read as…) | open | |
| L-rust-1 | conformance | `.claude/rules/rust.md` bullet 1 (Toolchain = just tools: installs every sha256-pinned…) | open | |
| L-rust-2 | conformance | `.claude/rules/rust.md` bullet 2 (Gate = just rust = fmt-check →…) | open | |
| L-rust-3 | conformance | `.claude/rules/rust.md` bullet 3 (Verifier canary = the first step of…) | open | |
| L-rust-4 | conformance | `.claude/rules/rust.md` bullet 4 (Late ckc check mutants = just late-mutants…) | open | |
| L-rust-5 | conformance | `.claude/rules/rust.md` bullet 5 (Mutation credit (CLAUDE.md Engineering: a test pinning…) | open | |
| L-rust-6 | conformance | `.claude/rules/rust.md` bullet 6 (Kani secondary gate = just kani (~3…) | open | |
| L-rust-7 | conformance | `.claude/rules/rust.md` bullet 7 (verify recipe = the verifier canary (bullet…) | open | |
| L-rust-8 | conformance | `.claude/rules/rust.md` bullet 8 (Format = verusfmt --edition 2024 (just fmt…) | open | |
| L-rust-9 | conformance | `.claude/rules/rust.md` bullet 9 (Lint = cargo clippy --workspace --all-targets --…) | open | |
| L-rust-10 | conformance | `.claude/rules/rust.md` bullet 10 (Dependency policy: rust/deny.toml (advisories; licenses Apache-2.0 /…) | open | |
| L-rust-11 | conformance | `.claude/rules/rust.md` bullet 11 (Secret scan = gitleaks 8.30.1 with .gitleaks.toml…) | open | |
| L-rust-12 | conformance | `.claude/rules/rust.md` bullet 12 (Workflow scan = zizmor 1.31.1, regular persona…) | open | |
| L-rust-13 | conformance | `.claude/rules/rust.md` bullet 13 (Workspace: ckc-spec (trusted, MAIN-authored, human-read) + ckc-kernel…) | open | |
| L-rust-14 | conformance | `.claude/rules/rust.md` bullet 14 (ckc trust-audit (just trust): scans workspace members…) | open | |
| L-rust-15 | conformance | `.claude/rules/rust.md` bullet 15 (Teammate tips kept past their branches =…) | open | |
| L-rust-16 | conformance | `.claude/rules/rust.md` bullet 16 (Kernel unit flow (MAIN implements; worktree mechanics…) | open | |
| L-rust-17 | conformance | `.claude/rules/rust.md` bullet 17 (Kernel proof lessons: spec fns the shell…) | open | |
| L-rust-18 | conformance | `.claude/rules/rust.md` bullet 18 (K2 clause index (contracts archive/contracts/cix.md, cix2.md): call…) | open | |
| L-rust-19 | conformance | `.claude/rules/rust.md` bullet 19 (Unit worktrees: a worktree predating a trusted-surface…) | open | |
| L-rust-20 | conformance | `.claude/rules/rust.md` bullet 20 (Committed harnesses (just test; each env var…) | open | |
| L-rust-21 | conformance | `.claude/rules/rust.md` bullet 21 (Toolchain + bump recipes (swipl, amend-manifest, bump-crate…) | open | |
| L-compiler-1 | conformance | `.claude/rules/compiler.md` bullet 1 (One schema, identifier 1 (contract .agent/archive/contracts/c1.md; law…) | open | |
| L-compiler-2 | conformance | `.claude/rules/compiler.md` bullet 2 (Schema revision migration (declaration indicators added) =…) | open | |
| L-compiler-3 | conformance | `.claude/rules/compiler.md` bullet 3 (Writer: write_canonical_part spells every atom holding a…) | open | |
| L-compiler-4 | conformance | `.claude/rules/compiler.md` bullet 4 (Editing gotchas: read DRS via arg/3 +…) | open | |
| L-compiler-5 | conformance | `.claude/rules/compiler.md` bullet 5 (Modes = compile, proof, question; the composition…) | open | |
| L-compiler-6 | conformance | `.claude/rules/compiler.md` bullet 6 (Bounded replay (per-doc P7 in the fork…) | open | |
| L-compiler-7 | conformance | `.claude/rules/compiler.md` bullet 7 (DRS facts: negation by arity+spelling — -/1…) | open | |
| L-compiler-8 | conformance | `.claude/rules/compiler.md` bullet 8 (Root exact groups (contract .agent/archive/contracts/root-exact.md): APE wraps…) | open | |
| L-compiler-9 | conformance | `.claude/rules/compiler.md` bullet 9 (tests/red: every probe pins class + rc…) | open | |
| L-compiler-10 | conformance | `.claude/rules/compiler.md` bullet 10 (Scratch probe entrypoints: -g main is argv-special…) | open | |
| L-corpus-1 | conformance | `.claude/rules/corpus.md` bullet 1 (MAIN — ACE per ruled statement batch…) | open | |
| L-corpus-2 | conformance | `.claude/rules/corpus.md` bullet 2 (researcher, extraction — bulk-reads the source; verbatim…) | open | |
| L-corpus-3 | conformance | `.claude/rules/corpus.md` bullet 3 (researcher, fetch rounds — candidate discovery, rights/licensing…) | open | |
| L-corpus-4 | conformance | `.claude/rules/corpus.md` bullet 4 (Closing reviewer = one covering every lens…) | open | |
| L-corpus-5 | conformance | `.claude/rules/corpus.md` bullet 5 (L0 completeness: source normative statements vs ace/…) | open | |
| L-corpus-6 | conformance | `.claude/rules/corpus.md` bullet 6 (Adversarial: L1 ACE↔extraction fidelity, L2 coverage-claim soundness…) | open | |
| L-corpus-7 | conformance | `.claude/rules/corpus.md` bullet 7 (Flow: spawn extraction early; MAIN batch-rules the…) | open | |
| L-corpus-8 | conformance | `.claude/rules/corpus.md` bullet 8 (Round close = ops.md Close order (stage…) | open | |
| L-corpus-9 | conformance | `.claude/rules/corpus.md` bullet 9 (ACE facts: noun copula needs the indefinite…) | open | |
| L-corpus-10 | conformance | `.claude/rules/corpus.md` bullet 10 (Coverage ledger (check_coverage): pinned 3-line generic header…) | open | |
| L-corpus-11 | conformance | `.claude/rules/corpus.md` bullet 11 (Inexpressible protocol: a normative statement the controlled…) | open | |
| L-corpus-12 | conformance | `.claude/rules/corpus.md` bullet 12 (Typing = spec Decisions Authoring principle: a…) | open | |
| L-corpus-13 | conformance | `.claude/rules/corpus.md` bullet 13 (Align authoring (ckc align <gid> <docid> stdin…) | open | |
| L-corpus-14 | conformance | `.claude/rules/corpus.md` bullet 14 (Vocabulary table (guidelines/<id>/vocabulary.tsv; grammar = docs/REFERENCE.md §…) | open | |
| L-corpus-15 | conformance | `.claude/rules/corpus.md` bullet 15 (Dose arithmetic (contracts .agent/archive/contracts/da2.md, da3.md; REFERENCE §…) | open | |
| L-corpus-16 | conformance | `.claude/rules/corpus.md` bullet 16 (Hedges (contract .agent/archive/contracts/s2.md D10; shapes = REFERENCE…) | open | |
| L-corpus-17 | conformance | `.claude/rules/corpus.md` bullet 17 (Scope exclusions (contract cs1; REFERENCE § Scope…) | open | |
| L-corpus-18 | conformance | `.claude/rules/corpus.md` bullet 18 (Absence conditions (user ruling, CS2b; per-sentence table…) | open | |
| L-corpus-19 | conformance | `.claude/rules/corpus.md` bullet 19 (Superseded statements (spec Decisions Supersession; contract .agent/archive/contracts/s) | open | |
| L-corpus-20 | conformance | `.claude/rules/corpus.md` bullet 20 (Prohibitions = SHOULD over a negation (spec…) | open | |
| L-corpus-21 | conformance | `.claude/rules/corpus.md` bullet 21 (Generic claims about a role quantify: Every…) | open | |
| L-corpus-22 | conformance | `.claude/rules/corpus.md` bullet 22 (A source example (e.g.) → an inert…) | open | |
| L-corpus-23 | conformance | `.claude/rules/corpus.md` bullet 23 (Derived referents: every indefinite of a rule…) | open | |
| L-corpus-24 | conformance | `.claude/rules/corpus.md` bullet 24 (A vocabulary.tsv byte change re-pins two fixture…) | open | |
| L-corpus-25 | conformance | `.claude/rules/corpus.md` bullet 25 (A committed question's ACE edit re-projects its…) | open | |
| L-corpus-26 | conformance | `.claude/rules/corpus.md` bullet 26 (rights.tsv: header profile statement url retrieved note…) | open | |
| L-corpus-27 | conformance | `.claude/rules/corpus.md` bullet 27 (Committed corpus + query artifacts change only…) | open | |
| L-ui-1 | conformance | `.claude/rules/ui.md` bullet 1 (ckc ui serve/render/check/request/copy-check (rust/ckc/src/ui/): std TcpListener on) | open | |
| L-ui-2 | conformance | `.claude/rules/ui.md` bullet 2 (Committed-corpus law: the reviewer surface reads HEAD…) | open | |
| L-ui-3 | conformance | `.claude/rules/ui.md` bullet 3 (Session (contract .agent/archive/contracts/sec7.md, user ruling R1 of…) | open | |
| L-ui-4 | conformance | `.claude/rules/ui.md` bullet 4 (Rendered commit (review custody): document pages carry…) | open | |
| L-ui-5 | conformance | `.claude/rules/ui.md` bullet 5 (Ledger write: candidate .adjudication.tsv.<nonce> (mode 0600 →…) | open | |
| L-ui-6 | conformance | `.claude/rules/ui.md` bullet 6 (Bundle v2: review_sha256 = sha256 of bundle…) | open | |
| L-ui-7 | conformance | `.claude/rules/ui.md` bullet 7 (Copy law: every fixed string the interface…) | open | |
| L-ui-8 | conformance | `.claude/rules/ui.md` bullet 8 (Highlight: artifact guidelines/<gid>/align/<docid>.tsv rows group side(src/ace) start…) | open | |
| L-ui-9 | conformance | `.claude/rules/ui.md` bullet 9 (Word meanings + reading words (contract .agent/archive/contracts/ur.md…) | open | |
| L-ui-10 | conformance | `.claude/rules/ui.md` bullet 10 (Superseded statements (contract .agent/archive/contracts/sup4.md; Decisions Supersessio) | open | |
| L-ui-11 | conformance | `.claude/rules/ui.md` bullet 11 (Phone layout (n22): page head = charset…) | open | |
| L-ui-12 | conformance | `.claude/rules/ui.md` bullet 12 (Separators (n23, user ruling replacing the generated…) | open | |
| L-ui-13 | conformance | `.claude/rules/ui.md` bullet 13 (Fixture harness = rust/ckc/tests/ui_fixtures.rs over tests/ui/{red,green}/ (85…) | open | |
| L-ui-14 | conformance | `.claude/rules/ui.md` bullet 14 (AM6 hazard: regenerating goldens from the current…) | open | |
| L-queries-1 | conformance | `.claude/rules/queries.md` bullet 1 (One schema (c1): every question record =…) | open | |
| L-queries-2 | conformance | `.claude/rules/queries.md` bullet 2 (Question mode: query sentence law = two-level…) | open | |
| L-queries-3 | conformance | `.claude/rules/queries.md` bullet 3 (Answer (ckc v1 answer <answer-manifest.tsv> <query-pl>; rc0…) | open | |
| L-queries-4 | conformance | `.claude/rules/queries.md` bullet 4 (Trace (ckc v1 trace <answer-manifest.tsv> <query-pl> <answers-pl>…) | open | |
| L-queries-5 | conformance | `.claude/rules/queries.md` bullet 5 (tests/queries/ + guidelines/*/queries/ bytes = committed pins…) | open | |
| L-queries-6 | conformance | `.claude/rules/queries.md` bullet 6 (Empty-stderr assertions need a UTF-8 locale (LC_ALL=C…) | open | |
| L-lexicon-1 | conformance | `.claude/rules/lexicon.md` bullet 1 (Clex = vendor/clex/clex_lexicon.pl (97,527 facts, verbatim upstream…) | open | |
| L-lexicon-2 | conformance | `.claude/rules/lexicon.md` bullet 2 (Clex is authoritative for the words it…) | open | lead: names `.scratch/clex-restore/*.py` + `.scratch/polish-lexshadow/*` (gitignored) as live recipes (Engineering-13) |
| L-lexicon-3 | conformance | `.claude/rules/lexicon.md` bullet 3 (Ulex pn_sg/3 WordForm is case-sensitive to the…) | open | |
| L-lexicon-4 | conformance | `.claude/rules/lexicon.md` bullet 4 (Any lexicon.ulex byte change rewrites the ulex…) | open | |
| L-vendor-1 | conformance | `.claude/rules/vendor.md` bullet 1 (Fork edits: a modified upstream file needs…) | open | |
| L-clinician-design-1 | conformance | `.claude/rules/clinician-design.md` (whole paragraph: palette, state colors, tables, dates, print, phone width, separators, reading words, banned slop, plumbing out of copy, honest scope, copy/design lint gates) | open | |
| L-spec-lint-1 | conformance | `.claude/rules/spec-lint.md` (spec-lint = template copy, first step of `just check`; recorded firing) | open | |
| L-CM-Execution-1 | conformance | `CLAUDE.md` Execution bullet 1 (Install/configure project-local; work within the launch dir…) | open | |
| L-CM-Execution-5 | conformance | `CLAUDE.md` Execution bullet 5 (Git: creds in the global gitconfig; standing…) | open | |
| L-CM-Execution-6 | conformance | `CLAUDE.md` Execution bullet 6 (Gate command = the full check chain…) | open | |
| L-CM-Execution-7 | conformance | `CLAUDE.md` Execution bullet 7 (CI gate: a public GitHub repo whose…) | open | |
| L-CM-Execution-10 | conformance | `CLAUDE.md` Execution bullet 10 (Task close → remove the temporary +…) | open | |
| L-CM-Execution-11 | conformance | `CLAUDE.md` Execution bullet 11 (UI design (artifact, variant or unit): the…) | open | |
| L-CM-Execution-12 | conformance | `CLAUDE.md` Execution bullet 12 (Charts + figures: the data's job picks…) | open | |
| L-CM-Authoring-1 | conformance | `CLAUDE.md` Authoring bullet 1 (AI agents = the sole developers ⇒…) | open | |
| L-CM-Authoring-2 | conformance | `CLAUDE.md` Authoring bullet 2 (State rules, facts + warnings plainly; omit…) | open | |
| L-CM-Authoring-3 | conformance | `CLAUDE.md` Authoring bullet 3 (Future-facing text, esp. prompts → state the…) | open | |
| L-CM-Authoring-5 | conformance | `CLAUDE.md` Authoring bullet 5 (Guidance + checks on text = goals…) | open | |
| L-CM-Authoring-6 | conformance | `CLAUDE.md` Authoring bullet 6 (Instruction files = yours to maintain →…) | open | |
| L-CM-Authoring-7 | conformance | `CLAUDE.md` Authoring bullet 7 (.claude/rules/ reaches MAIN + every teammate of…) | open | |
| L-CM-Authoring-8 | conformance | `CLAUDE.md` Authoring bullet 8 (@path imports (instruction files) + attaches (my…) | open | |
| L-CM-Authoring-9 | conformance | `CLAUDE.md` Authoring bullet 9 (Human-facing = strictly user-facing surfaces a person…) | open | |
| L-CM-Engineering-1 | conformance | `CLAUDE.md` Engineering bullet 1 (Elegant, tightly-scoped modular components; deduplicate; KISS +) | open | |
| L-CM-Engineering-2 | conformance | `CLAUDE.md` Engineering bullet 2 (Code = agent-read artifact → concise within…) | open | |
| L-CM-Engineering-3 | conformance | `CLAUDE.md` Engineering bullet 3 (Comments cost tokens → spend them on…) | open | |
| L-CM-Engineering-7 | conformance | `CLAUDE.md` Engineering bullet 7 (Deterministic checks own every rule a tool…) | open | |
| L-CM-Engineering-8 | conformance | `CLAUDE.md` Engineering bullet 8 (Tests/verification: derive scope from requested outcome +…) | open | |
| L-CM-Engineering-10 | conformance | `CLAUDE.md` Engineering bullet 10 (Verification integrity binds every check on the…) | open | |
| L-CM-Engineering-11 | conformance | `CLAUDE.md` Engineering bullet 11 (Assurance tier, declared per unit with its…) | open | |
| L-CM-Engineering-12 | conformance | `CLAUDE.md` Engineering bullet 12 (Units = the shortest path to the…) | open | |
| L-CM-Engineering-13 | conformance | `CLAUDE.md` Engineering bullet 13 (A gate backing a durable claim must…) | open | |
| L-CM-Engineering-14 | conformance | `CLAUDE.md` Engineering bullet 14 (Repairs to a generated artifact land as…) | open | |
| L-CM-Engineering-17 | conformance | `CLAUDE.md` Engineering bullet 17 (Split assurance by what it costs the…) | open | |
| L-CM-Engineering-20 | conformance | `CLAUDE.md` Engineering bullet 20 (Remotely-exploitable code → highest security standard: periodica) | open | |
| L-CM-Sessionflow-1 | conformance | `CLAUDE.md` Session flow bullet 1 (Phases, Phase: in .agent/spec.md = the phase…) | open | |
| L-CM-Sessionflow-2 | conformance | `CLAUDE.md` Session flow bullet 2 (.agent/spec.md = the sole attached state, imported…) | open | |
| L-CM-Sessionflow-3 | conformance | `CLAUDE.md` Session flow bullet 3 (PROTOTYPE = the fastest path to an…) | open | |
| L-CM-Sessionflow-4 | conformance | `CLAUDE.md` Session flow bullet 4 (IMPLEMENT = the best version of the…) | open | |
| L-CM-Sessionflow-6 | conformance | `CLAUDE.md` Session flow bullet 6 (Routed state: Phase line 1 = <PHASE>…) | open | |
| L-CM-Sessionflow-7 | conformance | `CLAUDE.md` Session flow bullet 7 (This file + .claude/agents/ = copies of…) | open | |
| L-CM-Sessionflow-8 | conformance | `CLAUDE.md` Session flow bullet 8 (Rules upkeep rides the Tasks clearing: each…) | open | |
| L-CM-Sessionflow-15 | conformance | `CLAUDE.md` Session flow bullet 15 (Read cost control = .gitignore kept current…) | open | |
