# M7 gap taxonomy

This file lists what the controlled language and the v1 schema cannot
carry, and the ruled disposition of each class. A region with such a
statement gets the coverage status `uncovered(inexpressible: <reason>)`.

## Evidence

- Projection census of the CDC 2022 opioid guideline: 337 documents.
  Each document's projection notes record what its ACE text dropped or
  approximated. This measures recorded projection loss. It does not
  prove that v1 cannot express the content.
- Statement sample: 96 normative statements from 12 other guidelines
  of the compendium (6 federal, 6 society), judged against the v1
  schema. The sample is purposive, not a prevalence estimate.

## Classes and dispositions

The user ruled the disposition of each class and chose temporal as
the first class to implement. Each class is a separate feature unit.

| Class | Observed gap | CDC documents | Sample statements | Disposition |
| --- | --- | --- | --- | --- |
| temporal | durations, frequencies, deadlines, sequencing | 81 | 44 | implemented: interval and recurrence annotations; count per period, scoped recurrence, order, approximate bound and ranged minimum (`docs/REFERENCE.md` § Vocabulary table and time annotations) |
| strength | recommendation grade, preference of one option over another, hedges | 63 | 49 | implemented: grade, preference and qualifier annotations (`docs/REFERENCE.md` § Strength annotations) |
| regimen-choice | one of several permitted options | — | 10 | ACE extension or schema revision: disjunction |
| numeric-threshold | numeric comparison, ranges, unit conversion | 19 | 27 | implemented: comparison built-ins and time-unit conversion (`docs/REFERENCE.md` § Comparison conditions) |
| dose-arithmetic | weight-based dosing, multiplication | 28 | 6 | schema revision: arithmetic built-ins |
| derived-measure | quantities computed from measurements | — | 3 | schema revision: arithmetic built-ins |
| probability | numeric likelihood, probabilistic update | 53 | 0 | companion target (ProbLog), deferred |
| condition-scope | exceptions and prerequisites | 96 | — | implemented: scope exclusions that gate the should, must and may statements of a guideline, and exceptions and prerequisites as rule conditions, with `does not provably` for the absence of a record (`docs/REFERENCE.md` § Scope exclusions) |
| quantification | set scope, example generalized to rule | 68 | — | ACE extension (group coordination) |
| relation-structure | cause and purpose links | 209 | — | authoring fidelity, ACE extension |

Strength status: the table's grade nouns give the 12 recommendation
documents 24 grade records, one strength and one evidence value each.
14 documents type 28 preferences: `preferred` and `first-line` copulas
and `instead-of` alternatives. Two of them, `cdc2022-opioid-s22-02` and
`cdc2022-opioid-s37-09`, are new documents for regions that earlier
restated a recommendation without its preference. The table's 56
qualifier rows type the hedges of 158 documents on the actions that
they qualify, in seven classes: manner (64 documents), applicability
(43), possibility (29), usuality (26), focus (23), ideality (5) and
force (2). A "might" claim compiles under "can" with "possibly". An
applicability hedge ("as appropriate", "when feasible") compiles as a
condition. 36 new documents formalize regions that earlier restated a
statement without its qualified content. Eight more new documents
formalize restating regions whose capability claim no target typed;
each states the claim under "can". A census of the 599 formalized and
restating regions (`.agent/archive/s2-census/`) rules each of their 680
hedge occurrences. 13 documents keep a hedge that no event carries. Most
such hedges sit inside a nominalized action ("avoid rapid tapering") or
name a manner that has no English adverb ("coordinated management"),
and stay in the nouns of the ACE. Two emphasize a whole prohibition
("In particular", "special attention" in `cdc2022-opioid-s25-08` and
`cdc2022-opioid-s51-11`); they stay untyped, and the projection notes
record them.

Temporal status: 56 of the CDC guideline's 387 documents carry
annotations. A census of the 81 documents with
a recorded temporal loss (`.agent/archive/m7t-reauthor.tsv`) found 12
whose sources state a bound that the interval and recurrence
annotations can carry. All 12
were re-authored and now compile with typed annotations. The other 69
keep a recorded loss: their sources state no integer bound (several, a
few, regularly), an approximate or ranged bound, a count per period
(twice per day), or an ordering without a quantity. Four of the 69, and
four documents outside the census, already stated a bound in ACE and
gained annotations without an ACE change. The later patterns type the
count per
period (`cdc2022-opioid-s60-14`), the recurrence inside a time window
(`cdc2022-opioid-s44-04`) and the order without a quantity: 30
documents gained 87 order clauses without an ACE change. A unit
abbreviation such as `d` types as a unit through a lexicon count noun
and a `unit` row; no schema change is needed. The approximation and
range rows also type
an approximate bound as the comparison `about` (`cdc2022-opioid-s59-06`,
`cdc2022-opioid-rec07-imp03`) and a minimum stated as a range
(`cdc2022-opioid-s40-01`, `cdc2022-opioid-s59-12`,
`cdc2022-opioid-s47-16`). Bare calendar adverbs (`daily`,
`periodically`) carry no time bound by user ruling; `periodically`
types as a usuality qualifier (schema S2), and `daily` stays a recorded
loss. A relation other than before or
after without a quantity (`during a visit`), a window anchored on a
start or end point that the source does not name, and a measured
number such as `3 d` stay outside the schema.

Numeric-threshold status: 30 of the CDC guideline's 387 documents
compare a quantity with a number, and 34 state a time quantity with its
unit. The 19 documents that stated a comparison in ACE before the
comparison built-ins compile it as a value comparison without an ACE
change. A census of the 702 regions outside Box 3
(`.agent/archive/nt-census/`) recorded 241 numeric statements. It found
11 regions whose sources state a threshold that the ACE approximated: an
age (at least 18, less than 18 and at least 65 years), osteoarthritis
pain in multiple joints and repeatedly negative tests (at least 2), more
than one prescribed opioid, opioid use less often than once a day (an
interval of more than 1 day), opioids continued for at least 1 month,
and certain daily dosages for at least 1 week. All 11 were re-authored.
Two of them, `cdc2022-opioid-s31-04` and `cdc2022-opioid-s40-14`, are new
documents for regions that earlier restated a recommendation without the
threshold. A passage keeps its source unit: `cdc2022-opioid-rec02-imp20`
keeps 30 days and `cdc2022-opioid-s40-14` states 1 month, because a day
and a month never compare. A bare calendar adverb (`daily`) still carries
no bound. A dose rate such as `MME-units-per-day` compares as a count of
its unit noun; a conversion between dose units stays in the
dose-arithmetic class.

Condition-scope status: the scope and exclusion rows of the CDC table make all 386 documents scoped. Twelve exclusion rules gate the 1,151 should, must and may edge clauses of the guideline: four care contexts (S3-03, restated by S9-01), three facility settings with the discharge exception (S7-02), and patients younger than 18 years (S10-04, with the adult audience of S9-05). Thirty-eight absence conditions in 18 documents read `does not provably`, for example a contraindication or a prerequisite step without a record. Observed negative findings and actual status stay classical negations. Eleven census rows gain rule conditions, prohibitions or data. Population and audience statements stay data.

The CDC counts record what the projections lost or approximated, not
proven limits of v1: v1 already carries Horn prerequisites and
conditional rules, so the condition-scope, quantification and
relation-structure rows mostly call for authoring practice and ACE
construct extensions. The CDC probability count consists mostly of
epistemic qualifiers, not numeric probabilities. A dash means that the source did not measure the
class.
