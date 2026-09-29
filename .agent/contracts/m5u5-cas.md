# U2 cas — reviewer-UI ledger-lock lost update (contract)

Defect (rev-m5u5-1 U5-02; `.agent/deferred.md` lock row): `LedgerLock` drop (`rust/ckc/src/ui/request.rs`) unlinks `.adjudication.lock` while holding it → a waiter woken on the orphaned inode + a newcomer that created a fresh file at the path both hold "the lock" → two POSTs from one ledger snapshot both pass the digest CAS → two 303s, the first accepted row lost. Same defect in `tools/ui.py` (retires at M5.7).

Scope = `rust/ckc/src/ui/request.rs` `LedgerLock` + one regression test. Unchanged: `ckc-spec`, `ckc-kernel`, `rust/trust/*.tsv`, `rust/ckc/tests/ui_fixtures.rs`, `tests/ui/**`, the POST guard chain, response bytes, the candidate/ledger write path, the `--fault after-tmp-write` crash path.

## Tier

shell — no theorem; regression-gated. `git diff --stat` = `rust/ckc/src/ui/request.rs`, `rust/ckc/tests/ui_lock.rs`, ledgers.

## Invariants

- I1 identity: a POST enters CAS + rename only while holding a flock on the inode that the lock PATH names, checked under the lock: `fstat(fd)` (dev, ino) == `stat(path)`.
- I2 release: the validated holder unlinks the path BEFORE unlocking; a holder that fails validation releases without unlinking.
- I3 progress: mismatch/ENOENT at validation → close, reopen, re-lock; each retry follows another holder's completed release; bounded retries (cap → the existing 500 envelope).
- I4 cleanup: every POST removes its own candidate + the lock inode it validated (the `--fault` path keeps its candidate, as today); once all writers exit, the audit dir holds no `.adjudication.lock` and no `.adjudication.tsv.*`.
⇒ one validated holder at a time.

## Predicates (acceptance)

- P1 regression `rust/ckc/tests/ui_lock.rs` (std only; Linux `/proc/locks`; `#[cfg(target_os = "linux")]`), fixture = copy of `tests/ui/red/verdict-ok-append/tree`, writer A = that case's POST argv. Schedule: S1 test locks the lock path (ino1); S2 spawn A, wait until `/proc/locks` shows A's pid waiting on ino1; S3 test unlinks the path; S4 test creates + locks a fresh file at the path (ino2) = the newcomer holding the lock; S5 test unlocks ino1; S6 wait (60 s cap) for A exit (baseline) or A waiting on ino2 (fixed); S7 test appends one row to the ledger (the newcomer's committed write) + releases ino2 in I2 order; S8 A finishes. Every wait carries a deadline naming its step + child-exit diagnostics; on any assertion failure the test kills + reaps A and releases its own locks. Grade: A never exits while the test holds ino2; A = `HTTP 409`, rc 0, stderr empty; ledger = S + the test row; audit dir = the 3 fixture files. Red: on the unfixed tree (A exits `HTTP 303` during S6). Green: fixed tree, 5/5 runs.
- P2 fix: identity-validated acquire per I1–I3; `Drop` order kept; no new CLI knob, env var or dependency.
- P3 unrelated behavior: `cargo test --workspace` green (ui_fixtures 113/113); `tests/ui/**` + `ui_fixtures.rs` byte-identical; trust meter unchanged; `just rust` green.
- P4 ledgers: `.agent/review.md` C-01..C-04 adjudicated by MAIN; deferred lock row removed; contract archived.

## Rulings

- R1 native correctness > legacy parity (legacy deleted at M5.7).
- R2 the test plays the initial holder AND the newcomer; the ckc binary = the queued waiter whose wake-up on an orphaned inode is the defect. No LD_PRELOAD shim, no C (supersedes the reverted f3fb7860 design: std `File::lock` suffices because the newcomer's critical section is the test's own).
- R3 fix = identity-validated acquisition; persistent lock file rejected (fixture trees compare audit dirs byte for byte).
- R4 MAIN authors test + fix (security-adjacent concurrency material stays on MAIN); one commit, red evidence in the body.
