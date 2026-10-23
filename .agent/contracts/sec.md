# SEC — security review of the release implementation (contract)

Request (user, MAINTAIN; binds the release version): security review of the implementation — threat surface from spec `Intent` + `Artifacts`; the committed scanners run (a stack without one = a finding); a manual pass over remotely reachable code on a check set fixed before reading = the `.agent/review.md` rows, committed before the pass reads code: each untrusted input traced to every sink it reaches, against the repo's own validation + auth patterns. Each finding = severity + `file:line` + a red test or repro; every finding above informational fixed; the rest → `.agent/deferred.md` rows with an acceptance check. MAIN alone (security vocabulary ends a teammate's context ⇒ no tester, reviewer or consultant; the request body outranks `rust.md` kernel unit flow). Gate green at every scoped commit.

## Threat model

Trust model (request): env vars + CLI flags = trusted; the server = the enforcement boundary; framework auto-escaping holds outside raw-HTML sinks — no framework here: the kernel renderer's escaping is itself under review. Repo code, recipes + tests = trusted. Committed corpus data = trusted origin carrying third-party bytes (source passages, coverage text) ⇒ every byte value stays admissible data at render + emission sinks.

Assets:
- AS1 ledger integrity: `guidelines/<id>/audit/adjudication.tsv` rows = the clinician audit record; a forged, altered or dropped verdict = a false audit claim reaching the release labels.
- AS2 release + KB integrity: the BagIt archive, `release-manifest.tsv`, compiled Prolog + `guidelines/schema.pl` = what consumers load into a CDS engine.
- AS3 host integrity: code execution or file writes on the reviewer's machine, a CI runner or a consumer's machine.
- AS4 gate integrity: pinned toolchain + verifier (supply chain of every green claim).
- AS5 confidentiality: secrets (CI tokens, git credentials), reviewer names + comments, local paths.

Untrusted inputs:
- U1 HTTP to `ckc ui serve` (127.0.0.1): any local process or user (raw TCP), any web origin through the reviewer's browser (cross-site GET/POST, DNS-rebound hostnames, framing).
- U2 third-party bytes inside committed data: passages, coverage rows, ACE derived from them, alignment rows, vocabulary rows, ledger rows written by U1.
- U3 network responses read by recipes: downloads (`just tools`, `just swipl`, `rust/kani/bootstrap.sh`), upstream metadata (`just outdated`, `bump-*`), the RustSec advisory DB.
- U4 CI events: push + PR content (fork PRs), dependabot PRs, caches + artifacts passed between jobs.
- U5 git state read by the server: HEAD, posted commit ids, the working-tree ledger overlaid on the HEAD snapshot.

Remotely reachable code: `rust/ckc/src/ui/**` (listener, parser, router, intake, guards, ledger write, snapshot) + the kernel K5 renderer + guard chain it calls; recipes reading U3 (`justfile`, `rust/kani/bootstrap.sh`); `.github/**`; the shipped artifacts consumers execute or extract (`ckc dist build` output, compiled Prolog, `guidelines/schema.pl`, README-dist load line); the page script in the reviewer's browser.

Sink inventory at SEC1 (denominator of I1, I3, A4, D2): `rg -n 'File::create|fs::write|OpenOptions|\.rename\(|fs::rename|set_permissions|create_dir|remove_(file|dir)|symlink_metadata|TcpListener::bind|fs::copy|hard_link|symlink\(' rust/ckc/src` = 153 lines (`rust/ckc/src/ui/` 32: serve.rs:142 bind; request.rs:369–481 ledger candidate, lock, rename; corpus.rs:79–98 snapshot tree; common.rs:72–83 temp dir; pages.rs:19–26 render output; intake.rs + fresh.rs symlink guards); `rg -n 'Command::new' rust/ckc/src` = 25 spawns (U1-reachable: corpus.rs:44, 117, 127, 145 git + tar; the rest = CLI + gate paths); recipe writes (`>`, `tee`, `install`, `cp`, `mv`, `ln -s`, `chmod`, `rm -`) = justfile 41, `rust/kani/bootstrap.sh` 24, `rust/verus-canary/run.sh` 2 lines. The trace table (`.agent/review.md` § SEC trace) carries each U1-reachable sink.

Ruling law: a limit counts as ruled only where a user ruling anchors it (spec `Decisions` or an archived record naming the user); an adjudication resting on an unanchored limit = a finding at its real severity + an `AskUserQuestion`. S5 rule (fixed before the scanner census): a stack without a committed scanner = informational → `.agent/deferred.md` row, unless the manual pass finds a defect in that stack the scanner would catch — then that scanner joins the gate as the fix's acceptance check.

## Severity

- critical: code execution or arbitrary file write on a reviewer, CI-with-credentials or consumer host from untrusted input, no user action.
- high: a forged, altered or suppressed ledger verdict, release member or KB clause from untrusted input without the process token; arbitrary file read through the server; a substituted download passing its pin.
- medium: a high impact gated on one user action (a click on an attacker page) or a local-user position; a pin row written from network data an attacker on the path controls.
- low: a defense-in-depth gap with a constrained attack path (a web page stalling the loopback server, a hardening header whose attack another control already blocks, local-path leaks).
- informational: hardening without a demonstrated attack path; a documented, ruled design limit (self-asserted reviewer names, no accounts).

## Tier

Server, guard + ledger fixes = kernel-tier rigor (no downstream re-check): a red test seen failing on the unfixed revision (revision + command in the commit body), mutation credit for tests pinning already-passing behavior. Recipe + CI fixes = data tier: `recipes.rs` fixture or a repro + `just workflows`. Docs = consistency pass.

## Gate identity

Each scoped commit: the fast gate of the steps it reaches (`ops.md`), then the full gate on that commit (`just ci <unit>`, else the pool form, else `just gate`). Close: `just gate` | `just ci sec` green on a clean tree; every `.agent/review.md` row adjudicated, ledger archived to `.agent/archive/review-sec.md`.

## Findings

Appended per finding: id = its row, severity, `file:line`, red (test | repro + rc), fix, acceptance check.
