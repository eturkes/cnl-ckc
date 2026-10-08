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
| temporal | durations, frequencies, deadlines, sequencing | 81 | 44 | implemented: schema v2 interval and recurrence annotations (`docs/REFERENCE.md` § Schema v2) |
| strength | recommendation grade, preference of one option over another | 63 | 49 | schema v2: graded recommendation record |
| regimen-choice | one of several permitted options | — | 10 | ACE extension or schema v2 disjunction |
| numeric-threshold | numeric comparison, ranges, unit conversion | — | 27 | schema v2: comparison built-ins |
| dose-arithmetic | weight-based dosing, multiplication | 28 | 6 | schema v2: arithmetic built-ins |
| derived-measure | quantities computed from measurements | — | 3 | schema v2: arithmetic built-ins |
| probability | numeric likelihood, probabilistic update | 53 | 0 | companion target (ProbLog), deferred |
| condition-scope | exceptions and prerequisites | 96 | — | ACE extension (exception clauses) |
| quantification | set scope, example generalized to rule | 68 | — | ACE extension (group coordination) |
| relation-structure | cause and purpose links | 209 | — | authoring fidelity, ACE extension |

Temporal status: the CDC guideline compiles under schema v2, and 20 of
its 337 documents carry annotations. A census of the 81 documents with
a recorded temporal loss (`.agent/archive/m7t-reauthor.tsv`) found 12
whose sources state a bound that the annotations can carry. All 12
were re-authored and now compile with typed annotations. The other 69
keep a recorded loss: their sources state no integer bound (several, a
few, regularly), an approximate or ranged bound, a count per period
(twice per day), or an ordering without a quantity. Those shapes stay
outside schema v2. Four of the 69, and four documents outside the
census, already stated a bound in ACE and gained annotations without an
ACE change.

The CDC counts record what the projections lost or approximated, not
proven limits of v1: v1 already carries Horn prerequisites and
conditional rules, so the condition-scope, quantification and
relation-structure rows mostly call for authoring practice and ACE
construct extensions. The CDC probability count consists mostly of
epistemic qualifiers, not numeric probabilities. A dash means that the source did not measure the
class.
