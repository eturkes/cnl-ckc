# memory

- Charter = `.agent/initial-prompt.md` — user's initial prompt, verbatim (2026-07-18), authoritative for scope + constraints; read before planning.
- Pins + license facts + governance decisions = `docs/provenance.md` (sole authority).
- Upstream state: e-- vendored M1; APE vendored M2; AceRules reference-only (reuse ⇒ license escalation); Clex excluded (test-only fetch at pin allowed, never committed); RACE no source.
- Scratch/probe/fidelity work stays in project-local gitignored `.scratch/`; for standalone `git apply`/git ops there set `GIT_CEILING_DIRECTORIES="$ROOT/.scratch"`, and use `rtk proxy diff` when an empty diff is the acceptance signal.
- WebSearch safety layer false-positives on medical/disease vocabulary → phrase queries by tool/org names only.
- Strict E-- + regeneration contracts live in `docs/strict-profile.md` + `docs/regen.md`; authoring gaps not captured there: comments, indexing, and postfix chains are unsupported, so bind intermediate call results and retain a loop's final value when the last item is needed.
- Apache-2.0 §4(b): patching a vendored upstream file requires a prominent in-file modification notice — ship it as part of the ordered patch set (e-- patch 0004 is the template).
- Deterministic SWI invocations: always pin `-f none -F none` (ambient init files can flip flags like character_escapes) and set writer options explicitly (`character_escapes(true)` etc.); adapter contract lives in `src/prolog/adapter.pl` header. Bash single-line gate pair: `grep -c '^'` (counts unterminated fragments) + `printf '%s\n' "$(<f)" | cmp - f` (catches missing final LF).
- Scope guard (user, 2026-07-18): minimally viable architecture > completeness — the point is auditability. `docs/research/*.md` = non-binding catalogs; each milestone plan adopts only the minimal subset serving its ledger deliverable. Named uncharted extras — rule-pair conflict detection and the full evaluation battery/thresholds — stay out unless the user asks.
- Presentation rule (user, 2026-07-18): research context in durable files reads as this project's own plan-phase findings — never reference other local working directories, their repo names, or their commits. Findings → `docs/research/` (self-contained); full-text corpus → `docs/references/` (README = citations, SHA-256, rights — Springer items are internal-use-only, do not redistribute).
- ACE front-end orchestration, identity/provenance, record/manifest, failure, and determinism contracts live in `docs/ace-front-end.md`.
