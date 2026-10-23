# Review ledger

Adjudication of judgment-bearing rows (`CLAUDE.md` `Engineering`: check set fixed before the diff is read; every row adjudicated; an accepted ruling holds until new evidence reverses it). Rows: `id | lens | check | verdict | evidence`. Verdicts: `open` → `pass` | `fail→fixed <sha>` | `info→deferred` (informational finding, `.agent/deferred.md` row) | `register` (outside the acceptance contract, noted).

Closed ledgers: `.agent/archive/review-da.md` (dose-arithmetic: unit reviews DA1, DA2, DA3, phase-close lenses A–E, SEC-1..SEC-7), `.agent/archive/review-cs.md` (condition-scope: unit reviews CS0, CS1, CS2, phase-close lenses CL-A..D, SEC-1..SEC-5, close commit), `.agent/archive/review-nt.md` (numeric-threshold review: C1–C10, S1–S8, G1–G6, V1–V7, K1–K8, N-R1..N-R9, SEC-1..SEC-7), `.agent/archive/review-eso.md` (eso closing review E-01..E-11), `.agent/archive/review-implement.md` (M5 review R-01..R-14, IMPLEMENT close S-01..S-10, U2 C-01..C-04, M5.5 close U5-01..U5-05 — every row adjudicated).

## SEC — security review of the release implementation (contract `.agent/contracts/sec.md`)

Check set fixed before the manual pass reads code; surfaces + inputs U1–U5 + severity = the contract. A finding row's evidence names severity, `file:line`, red + fix.

| id | lens | check | verdict | evidence |
|---|---|---|---|---|
| S1 | scanner | `just deny`: cargo-deny advisories (fresh RustSec DB) + bans + licenses + sources over every Cargo workspace in the repo | open | |
| S2 | scanner | `just secrets`: gitleaks full history + unstaged + staged | open | |
| S3 | scanner | `just workflows`: zizmor offline + online (`GH_TOKEN`) over `.github/` | open | |
| S4 | scanner | `just outdated`: every pin vs upstream latest; drift = exposure to upstream-fixed defects | open | |
| S5 | scanner | scanner coverage per stack: Rust, Actions, secrets covered; JavaScript (page script, node probes), Bash (recipes, `bootstrap.sh`, `run.sh`), Prolog (fork, `schema.pl`, KB), CI container image — each stack without a committed scanner = a finding | open | |
| I1 | injection | argv + shell: every `Command::new` in `rust/ckc` — argv bytes from U1/U2/U5, option injection (leading `-`, missing `--`), `/bin/sh -c` strings, inherited env | open | |
| I2 | injection | recipes + CI shell: U3 bytes or `${{ github.* }}` event data reaching shell code, `eval`, `source`, or a lock row a later recipe executes or fetches | open | |
| I3 | injection | file paths: request path segments (gid, docid, file names) → filesystem reads + writes + git pathspecs; `git archive \| tar -x` of HEAD into temp trees (member names, `..`, symlinks); dist member names; align + ledger paths | open | |
| I4 | injection | ledger + TSV: POST fields (reviewer, comment, verdict, docid, digests, commit) → `adjudication.tsv` bytes (TAB, CR, LF, NUL, control + non-UTF-8 bytes, row forging, field count); every other TSV the shell writes from data | open | |
| I5 | injection | Prolog: U1/U2 bytes spliced into `swipl -g` goals, Prolog source the shell writes, or compiled KB clauses (atom quoting; a data term becoming a directive or callable goal) | open | |
| I6 | injection | HTTP response headers: request-derived bytes in status line + headers (303 `Location`, `Content-Type`, CSP) | open | |
| I7 | injection | SQL/NoSQL, XML entities, template engines: any driver, XML parser or runtime template engine in the closure | open | |
| A1 | authz | POST guard completeness: every state-changing route requires token + loopback `Host` + matching `Origin` + digest CAS; bypass via method variants, absent/duplicate/case-variant headers, absolute-form targets, HTTP/1.0, pipelining or keep-alive smuggling, body-length mismatch | open | |
| A2 | authz | cross-site browser attacks: CSRF without the token, DNS rebinding, cross-origin reads (CORS, JSONP), clickjacking of the decision form (`frame-ancestors` / `X-Frame-Options`), side effects on GET | open | |
| A3 | authz | listener scope: 127.0.0.1 alone (no `0.0.0.0`, no `::`), port reuse + hijack races, other local users (no accounts: documented), the `ui request` seam | open | |
| A4 | privilege | local integrity: temp files + dirs (predictable names in shared `/tmp`, symlink-following writes, modes), ledger + candidate + lock file handling, writes through symlinked working-tree targets | open | |
| A5 | privilege | CI: workflow + job `permissions`, fork PRs vs secrets + tokens, `pull_request_target` / `workflow_run` absence, cache + artifact poisoning across trust levels, `ci/**` trigger scope, runner kind | open | |
| T1 | token | process token generation: entropy source, length, per-process uniqueness | open | |
| T2 | token | token handling: comparison, placement (hidden field vs URL), leakage (Referer, logs, static render, error bodies, caching), lifetime | open | |
| C1 | crypto | hardcoded secrets + key material in tree + history (S2 + manual grep) | open | |
| C2 | crypto | algorithms in security roles: custody digests, git SHA-1 commit ids in custody, md5 in `bump-tool`, hashes keyed on adversarial input | open | |
| C3 | crypto | randomness: token, nonces, temp names (OS CSPRNG vs time/pid) | open | |
| C4 | crypto | transport: every download + API call (no `-k`, `https` enforced across redirects, `--fail`), cargo / rustup / git transport, advisory DB fetch | open | |
| E1 | deserialization | HTTP parsing: request line, header + body size limits, slow clients vs a single-threaded loop, percent + form decoding (invalid UTF-8, `+`), Content-Length overflow | open | |
| E2 | eval | Prolog consult: files the shell consults (code) vs reads as terms; shipped KB + `schema.pl` + README-dist load line executing anything beyond the schema on consumer load (directives, `initialization`, expansion hooks, operators) | open | |
| E3 | deserialization | other parsers over untrusted bytes: jq/JSON in recipes, ledger TSV readers, tar + gzip extraction (options from data, bombs), node probe CDP JSON | open | |
| E4 | eval | page script DOM sinks: `innerHTML`, `insertAdjacentHTML`, `eval`, `Function`, string timers, `location` fed by page data | open | |
| H1 | raw-HTML | renderer contexts: every corpus, ledger + request byte in element text, quoted attributes, URLs (href from gid/docid/region: scheme + traversal), `<title>`, `<pre>`, `<style>`/`<script>`, comments — escape set per context | open | |
| H2 | raw-HTML | refusal + error responses: request bytes echoed in 4xx/5xx bodies or headers | open | |
| H3 | raw-HTML | response hardening: `Content-Type` + charset on every response, `nosniff`, CSP coverage (`default-src`, `style-src`, `img-src`, `form-action`, `base-uri`, `frame-ancestors`, `object-src`), `Referrer-Policy`, `Cache-Control` on token pages | open | |
| H4 | raw-HTML | static exports: `ckc ui render` pages (escaping, script policy over `file://`), README-dist + release notes Markdown built from data | open | |
| R1 | ssrf | outbound requests from the server or shell: network clients in `ckc`, git remote helpers / `-c` options / `--remote` from data | open | |
| R2 | ssrf | recipes fetching URLs read from network responses: host + scheme constraints, redirects; manifest `source` fetch URLs handed to consumers | open | |
| D1 | exposure | server output: stderr/stdout (request bodies, tokens, reviewer data), error detail (paths, internal errors) in responses | open | |
| D2 | exposure | responses exposing files beyond the reviewed corpus (`.git`, working-tree files), token in cached or static output | open | |
| D3 | exposure | release archive: intended members only; no local paths, user or host names, mtimes, uid/gid; reviewer names + comments (PII) shipped | open | |
| D4 | exposure | CI logs + artifacts: tokens printed, uploaded artifacts, retention | open | |
| P1 | supply-chain | Rust closure: vendored + locked sources, deny sources, build scripts + proc-macros executed at build, allowlist law, yanked crates | open | |
| P2 | supply-chain | toolchain + runtime downloads: every fetched artifact pinned by sha256 before execution (rustup + components, Verus, z3, node, chrome, kani bundle, swipl source); trust root of pin-update recipes | open | |
| P3 | supply-chain | CI: actions pinned by commit SHA, container images by digest, apt packages, cache keys + restore paths, dependabot config | open | |
| P4 | supply-chain | our distribution: archive integrity + authenticity (sidecar, BagIt manifests, determinism, tag, DOI), consumer verify + load instructions, tar member safety (relative paths, regular files, no links or devices, modes) | open | |
| P5 | supply-chain | gate browser + node: `--no-sandbox` Chrome over rendered pages, network reach during probes, pinned versions | open | |

## SEC trace — untrusted input → sinks (manual-pass deliverable; validator `rg -c '\| unknown \|' .agent/review.md` → 0 at SEC4)

| id | input | sinks reached (`file:line`) | guards (`file:line`) | verdict |
|---|---|---|---|---|
| T-M | U1 request method | unknown | unknown | unknown |
| T-P | U1 request target: path segments (gid, docid, page names), query, absolute form | unknown | unknown | unknown |
| T-V | U1 HTTP version + framing (request line bytes, CRLF/LF, pipelining, keep-alive) | unknown | unknown | unknown |
| T-H | U1 `Host` header | unknown | unknown | unknown |
| T-O | U1 `Origin` header | unknown | unknown | unknown |
| T-L | U1 `Content-Length`, `Transfer-Encoding`, other headers the server reads | unknown | unknown | unknown |
| T-B | U1 body bytes (size, encoding, form decoding) | unknown | unknown | unknown |
| T-F1 | U1 POST field: token | unknown | unknown | unknown |
| T-F2 | U1 POST field: verdict | unknown | unknown | unknown |
| T-F3 | U1 POST field: reviewer name | unknown | unknown | unknown |
| T-F4 | U1 POST field: comment | unknown | unknown | unknown |
| T-F5 | U1 POST fields: subject + ledger digests | unknown | unknown | unknown |
| T-F6 | U1 POST field: commit | unknown | unknown | unknown |
| T-F7 | U1 POST fields: unknown names, duplicates, missing | unknown | unknown | unknown |
| T-C | U1 connection behavior: slow, idle, many, half-closed | unknown | unknown | unknown |
| T-G1 | U5 HEAD commit + tree read by the snapshot (`git archive`) | unknown | unknown | unknown |
| T-G2 | U5 working-tree ledger overlaid on the snapshot | unknown | unknown | unknown |
| T-K1 | U2 passage + coverage text → pages | unknown | unknown | unknown |
| T-K2 | U2 ACE + alignment + vocabulary rows → pages + page script | unknown | unknown | unknown |
| T-K3 | U2 ledger rows (reviewer, comment) → pages, `ckc check`, dist labels | unknown | unknown | unknown |
| T-K4 | U2 gid/docid names → hrefs, paths, argv | unknown | unknown | unknown |
| T-K5 | U2 compiled Prolog → pages + shipped KB + consumer load | unknown | unknown | unknown |
| T-K6 | U2 `rights.tsv` + manifest URLs → README-dist + consumers | unknown | unknown | unknown |
| T-N1 | U3 downloaded artifacts → execution | unknown | unknown | unknown |
| T-N2 | U3 upstream metadata (JSON, redirects, text) → shell + lock rows | unknown | unknown | unknown |
| T-N3 | U3 advisory DB → `just deny` | unknown | unknown | unknown |
| T-E1 | U4 PR + push content (code, refs, titles) → CI jobs | unknown | unknown | unknown |
| T-E2 | U4 caches + artifacts between jobs | unknown | unknown | unknown |
