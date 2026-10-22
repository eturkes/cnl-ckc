# Contract rv — release versioning + provenance policy

Row: `.agent/spec.md` Tasks PR7 (user rulings at PR5, Decisions Publication scope). Tier: docs (consistency pass); no code, kernel or gate change.

- R1 REFERENCE § Compiled Prolog schema Status: schema `1` frozen from the first publication; a predicate-set or meaning change → identifier `2` for every document + question at once, named in the release notes; one identifier per composition (ships in README-dist).
- R2 REFERENCE § Export → § Releasing: identifier `kb-YYYY-MM-DD` (+ `.2`… same day); archive keeps its commit name; labels-only build; steps gate → `ckc dist build` → release notes (identifier, date, archive name + sha256, commit, schema identifier, per-guideline source edition + digest, label counts, schema change class + content change class with `substantive` = a change in whether a recommendation applies) → annotated tag → Zenodo deposit of the archive + sidecar as a new version (preprint linked) → `CITATION.cff` version/date/DOI.
- R3 `CITATION.cff` at the root, valid CFF 1.2.0 (`uvx cffconvert --validate`), stable fields only until the release.
- R4 Decisions Schema evolution + deferred rows (C1 version-law row re-opens at the first revision; Release review gate = labels only by user ruling) agree with R1–R2; the prior-art survey archived (`.agent/archive/release-prior-art.md`).
