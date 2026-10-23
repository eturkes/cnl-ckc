# SEC7 — reviewer session: the browser receives a per-launch session from `ckc ui serve` itself (contract)

Findings F2 + F3 + F5 of contract `sec` (rulings R1, R3). Reviewers are clinicians: no visible, typed or launched secret string; nothing that looks improvised.

## Tier

kernel: the guard chain lives in the human-read spec (`rust/ckc-spec/src/ui.rs`) and nothing downstream re-checks it. Red tests seen failing on the unfixed revision; Verus `--no-cheating` over every changed proof; the request body waives the `rust.md` kernel-flow teammates (MAIN alone).

## Predicates

Spec + kernel
- P1 `Request.cookie: Option<Bytes>` (the Cookie header; the shell joins repeats with `; `), `PostState.session: Bytes` (empty = no session: the `ui request` seam + render), `Response.set_cookie: Bytes` (empty = no header).
- P2 `gate(req, port, session) -> Option<PostOutcome>`, read before anything else, over `req`, `port` + `session` alone:
  1. Host ≠ `127.0.0.1:<port>` → the Host 403 (unchanged first refusal).
  2. session nonempty ∧ GET ∧ path = `/session/` + session → 303, `location` = `/`, `set_cookie` = `ckc-session-<port>=<session>; HttpOnly; SameSite=Lax; Path=/` (Lax: the browser arrives from a `file:` page, a cross-site top-level GET, and Strict would withhold the cookie on the redirect), body = `error_page` of `cnl-ckc reviewer` + `Opening the review site.`
  3. the method guard (unchanged, kept next to Host: proof order) → 405.
  4. session nonempty ∧ no `;`-separated, space-trimmed cookie pair = `ckc-session-<port>=<session>` → 403 page `Open the review site from ckc` + `This browser has not opened the review site yet. In the window where ckc ui serve runs, press Enter. The review site opens in this browser.`, detail `ui: request: no review session`.
  5. else None.
  `post_outcome(req, s)` = `gate(req, s.port, s.session)` when Some, else the existing chain from `models` on (unchanged).
- P3 `signin_page(port, session)`: the redirect page the browser opens from disk — a registry-bound, well-escaped page whose `<meta http-equiv="refresh">` targets `http://127.0.0.1:<port>/session/<session>`.
- P4 bindings `ui_gate`, `ui_signin_page` with `out@ ==` their spec; `good(post_outcome(req, s))` (every response = a well-escaped page of registry copy) over every state, new outcomes included; new copy rows append to `copy_table!` (reuse existing fragments: `cnl_ckc_reviewer`, `http_127_0_0_1`, `copy_3` `=`, page frame rows), each with a named use outside the table.

Shell
- P5 `ckc ui serve`: session = 32 `/dev/urandom` bytes as hex, ≠ token; after the request line + headers, `ui_gate` decides before any body read, snapshot or derivation; a Some answer is written and the connection closes.
- P6 body read only after the gate passes ∧ `may_read` holds (Host + Origin + form content type: a cookie-carrying page on another 127.0.0.1 port must not make the server read its body before the Origin refusal), capped at 64 KiB (a larger `Content-Length` ⇒ body None ⇒ the kernel's 400); one deadline of 10 s covers request line + headers + body, a fresh 10 s the answer (a slow snapshot or ledger write never cuts off an answer already earned); after the answer the server half-closes and drains ≤ 128 KiB within 1 s (an unread refused body would otherwise turn the close into a reset that drops the answer).
- P7 opener: `signin_page` bytes in `$XDG_RUNTIME_DIR/ckc-ui-<port>.html` when that dir is ours + mode 0700 (else a fresh 0700 temp dir), file mode 0600, rewritten at each launch; opener = `$BROWSER` | `xdg-open` (when `DISPLAY` or `WAYLAND_DISPLAY` is set) | `wslview` (when `WSL_DISTRO_NAME` is set); its argv carries the file path, never the secret. Opened → stdout says the site is open, then how to reopen (Enter) and stop (Ctrl+C); no opener → stdout gives the sign-in address. Each Enter on stdin runs the opener again. Residual (documented): a server without a terminal on stdin cannot reopen; its reviewer restarts it. Terminal copy (shell, lowercase after `ui:`): `ui: the review site is open in your browser. Press Enter to open it again. Press Ctrl+C to stop.` | `ui: no browser opened. Open this address: http://127.0.0.1:<port>/session/<session>`.
- P8 `ui request` seam: `--session <64 lowercase hex>` sets the state's session; `--header Cookie:…` passes the cookie; without `--session` the session steps of P2 stay off (every existing fixture unchanged).

Docs
- P9 REFERENCE § Reviewer interface (session, guard order, the cross-port residual of R1), README § Reviewing (the site opens itself; Enter reopens it), `.claude/rules/ui.md` (session law + tests).

## Red tests (seen failing on the unfixed revision)

- T1 `ui_socket.rs`: GET without the cookie → 403 session page; sign-in (no `Sec-Fetch-*` headers) → 303 `Location: /` + `Set-Cookie: ckc-session-<port>=<session>; HttpOnly; SameSite=Lax; Path=/` byte for byte; GET with the cookie → 200; decision POST with cookie + token → 303 + one ledger row; a cookie of another session → 403; foreign Host → 403 Host page.
- T2 opener: a fake `$BROWSER` records its argv (the file path, no secret); the file is 0600 and holds the sign-in URL; with no opener, stdout gives the sign-in address.
- T3 pre-snapshot order: `ui request --session S` against a root whose snapshot fails: no cookie → the 403 session page and a foreign Host → the Host 403 (unfixed: usage rc 2, then 500).
- T4 body cap: `Content-Length` 64 KiB + 1 → the 400 missing-body page with no body read; T5 deadline: a client stalled mid-headers is cut within 12 s while the next client is served.

## Gate identity

`just verify` (canary first), `just test`, `ckc ui check`, `just mobile`, `just check`, `.agent/archive/generators/replay` identical after the regenerated k5-sound files land; full gate = `just ci sec7`; `ckc ui serve` driven by its run command (sign-in in the pinned browser, a decision recorded).
