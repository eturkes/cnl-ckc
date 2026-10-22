# Contract cr — consumer recipe (whole-KB load with a patient)

Row: `.agent/spec.md` Tasks PR3 (request: publication readiness; Decisions: Search order, Publication scope; Intent goal 1). User pick (AskUserQuestion, this request): "Consumer termination" — diagnose, then fix the corpus shape or ship a documented, gated load recipe.

Defect (measured, HEAD 4848c008): schema.pl + the 382 CDC documents + the 3-sentence dose world (`tests/queries/green/dose-arithmetic` `world.pl`) → plain SWI 9.2.9 q-mme / q-mme-exact rc 124 at 300 s; `guideline_entity(actual,_,'opioid-dosage',countable)` > 30 s (KB alone: 1 solution at once); `ckc v1 answer` `indeterminate(limit)` after 1054 s / 1036 s (nixbox, uncontended). Causes: a body goal with an unbound referent that heads unify with (s37-13 negation box read before its content, its own head among them; rec05-imp12 time-quantity cardinality before its event; 18 clauses / 9 documents self-unify) + exponential re-derivation (Skolem role entities, e.g. rec05-imp04's derived patient). Tabling all schema predicates: every open goal finishes at once (dose world: 346 should, 12 must, 43 may, 61 can, 15 negation, 174 entities), committed answers unchanged; a `tnot` rewrite of all 1623 negations gives the same sets with 0 undefined.

Tier: docs + consumer contract; the gate harness = the only check (no kernel, spec or compiler change).

Predicates:
- P1 `docs/REFERENCE.md` § Compiled Prolog schema holds `### Loading a whole knowledge base` with one `prolog` block = one `:- table` directive naming exactly the indicators every compiled document declares multifile (20 = the 18 semantic predicates + `guideline_schema_version/1` + `guideline_document/4`), in declaration order.
- P2 Under P1's directive, SWI-Prolog 9.2.9 loading schema.pl + every committed document of every guideline answers every committed question with exactly its committed answer.
- P3 For each world (dose; adult 45 years with chronic pain; minor 16 years; cancer-related pain), composed with every document: should, must, may, can, negation boxes, exclusions, entities each finish within 120 s under the recipe; the well-founded run (P1 + `tests/consumer/wfs.pl.in`) gives the same answer set per goal with 0 undefined answers.
- P4 minor + cancer: exclusion holds, should = must = may = 0; dose + adult: no exclusion, should > 0.
- P5 dose world: q-mme + q-mme-exact = committed `yes` under the recipe.
- P6 Firing (each recorded in the unit's commit body): plain search (no directive) over the dose composition does not finish `opioid-dosage` in 20 s; the well-founded run reads a two-clause negation loop as 2 undefined answers.
- P7 Docs: REFERENCE (the recipe, the leftmost-only reach of recursion-check, the formula paragraph's scope), README Scope and limits, README-dist via the schema section, Decisions Search order state the recipe + the measured limits; SWI-Prolog 9.2.9 = the only engine tested.

Gate identity: new binary `consumer_recipe` in `just test` (SWI-backed, `swipl` on PATH); fast gate = `cargo test --test consumer_recipe` + `ckc check` (REFERENCE schema section = release input → `just amend-manifest`).

Out of scope (deferred rows): binding-first emission; a verified tabled/bottom-up engine; Skolem role entities in rule consequents; engines other than SWI.
