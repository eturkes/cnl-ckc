# Review ledger

Adjudication of judgment-bearing rows (`CLAUDE.md` `Engineering`: check set fixed before the diff is read; every row adjudicated; an accepted ruling holds until new evidence reverses it). Rows: `id | lens | check | verdict | evidence`. Verdicts: `open` → `pass` | `fail→fixed <sha>` | `register` (outside the acceptance contract, noted).

Closed ledgers: `.agent/archive/review-cs.md` (condition-scope: unit reviews CS0, CS1, CS2, phase-close lenses CL-A..D, SEC-1..SEC-5, close commit), `.agent/archive/review-nt.md` (numeric-threshold review: C1–C10, S1–S8, G1–G6, V1–V7, K1–K8, N-R1..N-R9, SEC-1..SEC-7), `.agent/archive/review-eso.md` (eso closing review E-01..E-11), `.agent/archive/review-implement.md` (M5 review R-01..R-14, IMPLEMENT close S-01..S-10, U2 C-01..C-04, M5.5 close U5-01..U5-05 — every row adjudicated).

## DA2 kernel-unit review (reviewer-2; target `wt/da2` d58035ad; check set R1–R14 fixed before reading; `.scratch/agents/da/reviewer-2/verdicts.md`: 9 pass, 4 finding rows, 1 register)

| id | lens | check | verdict | evidence |
| --- | --- | --- | --- | --- |
| DA2-F1 | correctness (R2, R6, R8) | D5: every leaf's `operand` law before any leaf's `operand_bound`; an unconverted plain-number time quantity → `operand` | open | `If a man contains 2 cats X and contains some water Z then the man includes some dogs Y and Y = X * Z.` and `If a man contains 2 days X then the man includes some dogs Y and Y = X * 2.` (table row `unit day day`) → fork + certifier `operand_bound`, want `operand`; red = `python3 -I .scratch/agents/da/reviewer-2/probe.py control bound-before-mass mass-before-bound plain-time some-time adjective-target` rc 1 |
| DA2-F2 | correctness (R2, R8), major | D1 operand = `object(R, Noun, countable, na, C, N)`: Unit `na` | open | `If a man contains at least 2 kg of apples X then the man includes some dogs Y and Y = X * 2.` (and `exactly 2 kg`) compiles + certifies rc 0, want `operand`; anchors spec `operand_pair` `arg(c, 2) != atom("countable"@)`, fork `( Class \== countable ->`; red = `probe.py control measured-count measured-exact` rc 1 |
| DA2-F3 | docs (R13) | REFERENCE § Formulas in ACE states the `some` immediately-before-noun limit (`some red cats X` keeps its `geq 2` filter) + D1's countable / no-time operand limits; its ACE example = one physical line | open | `docs/REFERENCE.md` on wt/da2: "A `some` operand keeps no `guideline_satisfies` condition."; the printed two-line example → `sentence_lines,counts(lines(2),sentences(1))` |
| DA2-G1 | register (R10) | D4's fixed witness vs an antecedent NAF goal | register | outside D1–D5; queued as `.agent/deferred.md` row `Fixed-witness NAF completeness` on `wt/da2` fc73a04c (lands with DA2) |
