# Review ledger

Adjudication of judgment-bearing rows (`CLAUDE.md` `Engineering`: check set fixed before the diff is read; every row adjudicated; an accepted ruling holds until new evidence reverses it). Rows: `id | lens | check | verdict | evidence`. Verdicts: `open` → `pass` | `fail→fixed <sha>` | `info→deferred` (informational finding, `.agent/deferred.md` row) | `register` (outside the acceptance contract, noted).

Closed ledgers: `.agent/archive/review-rev.md` (code review of the implementation for the pre-release + release: 24 baseline + 65 lens × module + 24 spec-fit + 132 conformance rows, reviewer-1..10 + MAIN; fix classes REV-V, C1–C14, D1–D6, T1–T4, S, K), `.agent/archive/review-sup.md` (supersession: consultant + unit reviews SUP1, SUP3, SUP4, visual QA, phase-close lenses A–E, SEC-1..SEC-7), `.agent/archive/review-sec.md` (security review of the release implementation: check set S1–P5 + trace T-M..T-E2), `.agent/archive/review-da.md` (dose-arithmetic: unit reviews DA1, DA2, DA3, phase-close lenses A–E, SEC-1..SEC-7), `.agent/archive/review-cs.md` (condition-scope: unit reviews CS0, CS1, CS2, phase-close lenses CL-A..D, SEC-1..SEC-5, close commit), `.agent/archive/review-nt.md` (numeric-threshold review: C1–C10, S1–S8, G1–G6, V1–V7, K1–K8, N-R1..N-R9, SEC-1..SEC-7), `.agent/archive/review-eso.md` (eso closing review E-01..E-11), `.agent/archive/review-implement.md` (M5 review R-01..R-14, IMPLEMENT close S-01..S-10, U2 C-01..C-04, M5.5 close U5-01..U5-05 — every row adjudicated).

## SC — `.sha256` sidecar on published releases (user request: "include that asset"; amend rc.1)

Check set fixed before the first read. Target = the SC unit: main commit (justfile `release-kb`, `rust/ckc/tests/recipes.rs` release-kb test, README § Releases, REFERENCE Export + § Releasing, `.claude/rules/rust.md` recipes bullet, spec Artifacts + Decisions Publication scope) + its release-line twin + the re-signed `kb-2026-10-29-rc.1` notes + the published GitHub pre-release. Tier = shell (a consumer re-checks every published byte: `sha256sum -c`, verify-tag, bag manifests). Outside: security (the `Security review` body; a security defect met here = MAIN alone). Finding = severity (`blocker` | `major` | `minor`) + `file:line` + proof (red test for behavior; `rg -Fn` anchor bytes for text). B rows = MAIN; every other row = reviewer-1 (one reviewer, every lens: small diff), ruled by MAIN on its own check.

| id | lens | check | verdict | evidence |
|---|---|---|---|---|
| B-1 | baseline | full gate on main's closing commit, clean tree (`just ci <unit>`) | open | |
| B-2 | baseline | full gate on the line commit (`just ci <unit>` from the review worktree) | open | |
| B-3 | baseline | `just release-kb kb-2026-10-29-rc.1 <scratch>` on the line commit: rc 0, exactly archive + sidecar, archive sha256 = the published asset's, sidecar = its `<sha256>  <file>` line, `sha256sum -c` OK; malformed identifier → rc 2, nothing written | open | |
| B-4 | baseline | published state from a fresh clone: verify-tag good signature (allowed signer, fingerprint = notes), tag-body `sha256sum -c --ignore-missing` OK, downloaded sidecar `sha256sum -c` OK, bag `sha256sum -c` OK, release page lists both assets | open | |
| C-1 | correctness | `release-kb` read with callers (REFERENCE steps 3/10, recipes.rs) + callees (`ckc dist build`, `install`, `sha256sum`): sidecar names the published file (never ckc's bag-named sidecar); every failure exits non-zero; refusals leave no destination; bash `set -e` pitfalls (`&&` lists, subshells, command substitution) ruled out | open | |
| C-2 | correctness | recipes.rs release-kb test: sidecar bytes + listing + stdout asserted exactly; refusal + planted cases intact; fixture setup/teardown symmetric | open | |
| CL-1 | cleanup | recipe + test: sidecar line derived once; no stale "without the sidecar" text; no dead code; helpers reused | open | |
| A-1 | altitude | sidecar written in the recipe vs inside `ckc dist build`: a special case on shared infrastructure? | open | |
| CS-1 | claim soundness | README § Releases, REFERENCE Export + § Releasing (steps, signing paragraph, "To verify"), `just --list` doc, recipe comment, rust.md bullet, spec Artifacts vs the recipe's behavior | open | |
| CS-2 | claim soundness | rc.1 notes (tag message = GitHub body) vs the published state: assets, checksum line, commit, replacement history, every command runnable as written | open | |
| V-1 | verification integrity | changed test seen red on the unfixed revision (command recorded); no grading check weakened (refusals, planted guard) | open | |
| SF-1 | spec fit | behavior vs `Intent` + Decisions Publication scope as amended by the user's ruling: `kb-only` name kept, sidecar published, notes carry the archive line, signed tag binds the archive bytes; release inputs unchanged ⇒ archive bytes unchanged | open | |
| K-1 | conformance | `CLAUDE.md` Authoring Human-facing (ASD-STE100) on README, REFERENCE, notes | open | |
| K-2 | conformance | `CLAUDE.md` Authoring agent-optimized + Engineering comments (= why) on justfile comment, test comment, rust.md, spec, commit bodies | open | |
| K-3 | conformance | rust.md recipes law: `[doc]` on the public recipe (`just --list` test), ShellCheck over recipe bodies, checked assignments | open | |
| K-4 | conformance | `CLAUDE.md` Execution Git: subject `<scope>: <cause> → <fix>`, body = measurements + SHAs + teammates | open | |
| K-5 | conformance | ops.md Release lines: line commit leaves the line's `.agent/spec.md` + `.agent/deferred.md` alone, leaves every release input unchanged; tag + files verified before the push; remote actions on the user's go | open | |
| K-6 | conformance | spec-lint + agent-law checks (`ckc check`) green over the spec + rules edits | open | |
