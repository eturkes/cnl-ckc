# M5.5-cas — reviewer-UI ledger-lock lost update (follow-up contract)

Defect (rev-m5u5-1 U5-02; `.agent/deferred.md` lock row): `LedgerLock::drop`
(`rust/ckc/src/ui/request.rs:383`) unlinks `.adjudication.lock` while holding it
→ a waiter woken on the orphaned inode + a newcomer that created a fresh file at
the path both hold "the lock" → two POSTs from one ledger snapshot both pass the
digest CAS → two 303s, the first accepted row lost. Replay at c3d02cc5 (binary
sha256 fce50147…): native + legacy both lose writer-b
(`.scratch/agents/rev-m5u5-1/cas-overlap-contract.log`; historical evidence only —
its driver waits for both writers at rename, which a correct handoff never yields).

Scope = native shell only: `rust/ckc/src/ui/request.rs` `LedgerLock` + one
tracked regression. Unchanged: `tools/ui.py`/`ui.emm` (legacy keeps the defect
until M5.7), `ckc-spec`, `ckc-kernel`, `rust/ckc/tests/ui_fixtures.rs`,
`tests/ui/**`, every fixture expectation, the POST guard chain, response bytes,
the candidate/ledger write path, the `--fault after-tmp-write` crash path.
Native concurrency correctness supersedes parity with the legacy schedule (R1).

## Tier

shell — no theorem; fixture + regression gated. Verified surface untouched:
`git diff --stat c3d02cc5 -- rust/ckc-spec rust/ckc-kernel rust/trust` empty.

## Invariants (the fix must hold; reviewer row C-01)

- I1 identity: a POST enters the CAS/rename critical section only while holding
  a flock on the inode the lock PATH currently names, validated under the lock:
  `fstat(fd)` (dev, ino) == `stat(path)`.
- I2 release: the validated holder unlinks the path BEFORE unlocking (a newcomer
  must never find a linked inode whose holder already released); a non-validated
  lock owner (woken on an orphan) releases without unlinking.
- I3 progress: mismatch/ENOENT at validation → close, reopen the path, re-lock;
  every retry follows another holder's completed release → bounded (cap → the
  existing 500 envelope `ui: verdict: ledger write failed`).
- I4 cleanup: after every POST outcome the audit dir holds no `.adjudication.lock`
  and no `.adjudication.tsv.*` (the `--fault` crash path keeps its candidate, as
  today).

⇒ at most one validated holder at any time: the path names one inode at a time;
every unlink is performed by a validated holder after its critical section; a
fresh inode gains a validated holder only after that unlink.

## Predicates (acceptance)

- P1 regression `rust/ckc/tests/ui_lock.rs` + `rust/ckc/tests/ui_lock/pause_rename.c`
  (Linux, schedule S1–S9): red on the baseline two ways — (a) worktree source =
  c3d02cc5 + the test alone, `cargo test --test ui_lock` fails at S9(i)/(ii);
  (b) `CKC_UI_TEST_BIN=$ROOT/.scratch/gate/ckc-6094717d` (sha256 fce50147…) fails
  the same way. Seal = `sha256sum` of both files in the tester's commit body +
  `.scratch/m5u5-cas/seal.SHA256SUMS`; byte-identical after the repair.
- P2 fix: `LedgerLock::acquire` identity-validated per I1–I3; `Drop` order kept
  (I2); no new CLI knob, env var or dependency; diff = `rust/ckc/src/ui/request.rs`.
- P3 green: `cargo test --test ui_lock` on the fixed tree, 5/5 runs: one 303 +
  one 409, ledger byte-exact = fixture `after` ledger carrying the accepted
  writer's row, audit dir = 3 files, both rc 0, stderr empty.
- P4 unrelated behavior: `cargo test --workspace` green (ui_fixtures 113/113,
  cli, dist_cases); parity `python3 -P .scratch/m5u5/test/parity.py --rust-bin
  <fixed release binary>` render 345/345 + requests 153/153; `ui_fixtures.rs`,
  `tests/ui/**`, `tools/**` byte-identical to c3d02cc5; trust meter unchanged.
- P5 gate: `just rust` green in the producer worktree; `just gate` green from
  the committed close SHA (MAIN, log `.scratch/gate/m5u5-cas-gate.log`).
- P6 ledgers: `.agent/review.md` C-01..C-04 adjudicated; deferred lock row →
  legacy-limitation row; `spec.md` Artifacts names `ui_lock.rs`; this contract
  archived at close.

## Regression schedule (P1; the test = initial holder releasing in the legacy order, R2)

Fixture = copy of `tests/ui/red/verdict-ok-append/tree` (ledger S = 3 rows,
digest c3def2a6…); writers A/B = that case's argv with
`reviewer=writer-a&comment=a` / `reviewer=writer-b&comment=b` (same
`review_sha256`, `ledger_sha256`, `csrf`, `--token`, `--now`, `--commit`).
Binary = `CKC_UI_TEST_BIN` else `CARGO_BIN_EXE_ckc`; child env as
`ui_fixtures.rs` `invoke` + `LD_PRELOAD=<so>` + `CKC_RENAME_GATE=<gate dir>`.
Shim = `cc -shared -fPIC -o <scratch>/pause_rename.so rust/ckc/tests/ui_lock/pause_rename.c`
(interposes `rename`; with `CKC_RENAME_GATE` set and destination leaf
`adjudication.tsv`: create `<gate>/ready`, then wait for `<gate>/release`).
`/proc/locks` waiter line = `N: -> FLOCK  ADVISORY  WRITE <pid> <maj>:<min>:<ino> …`,
matched on pid + ino only (btrfs `st_dev` ≠ `s_dev`).

- S1 test opens `audit/.adjudication.lock` (O_RDWR|O_CREAT, 0600) + flock EX → ino1.
- S2 spawn A; wait until `/proc/locks` shows A waiting on ino1.
- S3 test unlinks the lock path (legacy release step 1; ino1 stays held).
- S4 spawn B; wait for `<gateB>/ready` (B created + locked ino2 uncontended,
  passed CAS, paused at rename); ino2 = `stat(path).ino`.
- S5 test unlocks + closes ino1 (legacy release step 2).
- S6 wait for any of: `<gateA>/ready` (baseline: A passed CAS on the orphan);
  `/proc/locks` shows A waiting on ino2 (fixed: identity mismatch → re-queued
  behind B); A exited.
- S7 create `<gateB>/release`; wait B → outB.
- S8 create `<gateA>/release`; wait A → outA.
- S9 grade in order: (i) status lines {outA, outB} == {`HTTP 303`, `HTTP 409`},
  either assignment (the lost update = two 303s); (ii) ledger bytes ==
  `after/guidelines/alpha/audit/adjudication.tsv` with `new-b` → accepted
  writer, `appended` → its comment; (iii) audit dir == {adjudication.tsv,
  projection-notes.tsv, review-manifest.tsv}; (iv) both rc 0, stderr empty.
  Every wait carries a 60 s safety cap naming its step; the cap is unreachable
  on both the baseline and a correct fix (each wait resolves by construction).

Coverage: queued waiter (A on ino1), fresh arrival (B on ino2), and — fixed
tree — the real `Drop` → acquire handoff (B's release → A's re-acquisition →
validation → retry → 409).

## Rulings

- R1 native correctness > legacy-schedule parity; `tools/ui.py` keeps the
  defect until M5.7 — `.agent/deferred.md` carries that limitation.
- R2 the test plays the initial holder and releases in the legacy order (unlink,
  then unlock) = the schedule every pre-fix native holder and the legacy tool
  produce; the real production handoff under test = B's `Drop` → A's acquisition.
  No existing knob pauses a holder inside `Drop`; a new knob would make the
  baseline fail on argv, not on the outcome.
- R3 Linux-only test (`/proc/locks`, `LD_PRELOAD`, flock) under
  `#[cfg(target_os = "linux")]`; CI + workstation = Linux.
- R4 fix = identity-validated acquisition (deferred row option 1). Rejected: a
  persistent lock file (fixture trees compare audit dirs byte-for-byte; the file
  must be absent after a request); a directory flock (the regression could not
  turn red on the baseline through the file protocol; interim legacy interop
  keeps the file).
- R5 no new CLI knobs, env vars or dependencies; shim + test = test-tree files.
- R6 commits: (1) tester's regression commit (red evidence in the body);
  (2) fix + contract/review/deferred/spec (MAIN); `just gate` from (2).

## Dispatch

roles = `test-m5u5cas-1` (author + seal + red; wt/test-m5u5cas),
`prod-m5u5cas-1` (repair + `just rust`; wt/prod-m5u5cas from the tester's tip),
`rev-m5u5cas-1` (C-01..C-04, independent, own worktree); MAIN = rulings,
integration, reruns, gate. Review rows frozen in `.agent/review.md` § M5.5-cas.
