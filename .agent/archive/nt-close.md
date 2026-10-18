# IMPLEMENT — M7 gap class numeric-threshold (closed)

Routed request + ruling as they stood in `.agent/spec.md` `Phase`:

IMPLEMENT — M7 gap class numeric-threshold
> M7 gap coverage, class numeric-threshold (user pick; phase IMPLEMENT by user ruling): numeric comparison, ranges, unit conversion; ruled disposition = a schema revision with comparison built-ins (taxonomy, counts + dispositions = `docs/m7-gap-taxonomy.md`). Measured: 19 CDC documents compare a quantity with a number, each bound a `guideline_cardinality(…, <comparison>, <count>)` on the counted noun — 8 compare MME/day with 50 (`geq` in 7, s34-09 also `greater`; `less` in rec07-imp03), the rest durations (`day geq 30`, `day less 7`, `day greater 3`, `year geq 1`, `week geq 1`, `month less 1`), age (`year geq 65`), `percent geq 30` + counts of 2 or more medications, prescriptions or clinicians; matching stays exact Horn structure (REFERENCE § Vocabulary table and time annotations: a fact for `geq 30` does not satisfy a goal for `geq 20`), so a patient at 60 MME/day never matches "at least 50". Accept: the class's disposition implemented under its own unit contract, with ≥1 corpus statement of that class previously approximated now compiled and certified.
> User ruling (AskUserQuestion): "numeric-threshold (Recommended)" — route the class as its own IMPLEMENT request.
Suspended: MAINTAIN — the whole product

Outcome: units NT1 (d7fbf1cd: comparison built-ins `guideline_satisfies/4|6` + `guideline_unit/3`, contract `archive/contracts/nt1.md`) + NT2 (8a51bf0c: 11 approximated CDC thresholds re-authored, two new documents; census `archive/nt-census/`); acceptance = s34-04, s48-02, s10-04, s9-05 (age), rec02-imp07 + s27-02 (joints), s54-15 (tests), s38-03 (opioids), rec05-imp18 (interval), s31-04 + s40-14 (durations) compile with value comparisons and certify; ledger `archive/review-nt.md`; the suspended MAINTAIN scope resumes.
