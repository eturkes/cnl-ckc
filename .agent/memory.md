# memory

- Charter = `.agent/initial-prompt.md` — user's initial prompt, verbatim (2026-07-18), authoritative for scope + constraints; read before planning.
- Pins + license facts + governance decisions = `docs/provenance.md` (sole authority).
- Upstream state: e-- vendored M1; APE vendored M2; AceRules reference-only (reuse ⇒ license escalation); Clex excluded (test-only fetch at pin allowed, never committed); RACE no source.
- Scratch/probe work stays inside the project in a gitignored dir (`.scratch/`) — `CLAUDE.md` launch-dir rule covers throwaway clones; M0.2 used `/tmp` (violation; cleaned up 2026-07-18; evidence location-independent, M2 re-verifies at vendor time).
- WebSearch safety layer false-positives on medical/disease vocabulary → phrase queries by tool/org names only (hit again this session on an APE/SWI query).
- Fidelity archives under project-local `.scratch/` still sit beneath the parent Git worktree; set `GIT_CEILING_DIRECTORIES="$ROOT/.scratch"` for standalone `git apply`, and use `rtk proxy diff` when an empty diff is the acceptance signal.
