# ACE and CNL notes for the clinical pipeline

This note distills the primary sources in [`docs/references`](../references/README.md)
and internal plan-phase research (2026-07). It is for agents building the M2
deterministic ACE front end and authors preparing the first clinical guideline in M5.

## 1. Recommended position

Use the vendored APE fork as a pinned **ACE-to-DRS reference parser**, not as the definition
of the project's accepted language. Define a smaller, fail-closed Clinical ACE profile:

```text
Clinical ACE record
  -> envelope and lexical validation
  -> pinned APE parse
  -> accepted-DRS validation
  -> canonical DRS serialization
  -> typed clinical IR
  -> deterministic Prolog compilation and explanations
```

APE accepts useful general-language conveniences that are unsafe in a locked clinical
knowledge surface. The project profile therefore needs stricter vocabulary, reference,
modality, quantity, and provenance rules than ACE itself.

## 2. Why an ACE-style pipeline won

ACE has the strongest complete implementation chain found for this project:

1. published construction and interpretation rules;
2. a deterministic SWI-Prolog parser;
3. a documented DRS target rather than an opaque AST;
4. a runtime user-lexicon mechanism;
5. DRS-to-ACE and canonical-paraphrase tooling;
6. executable-rule descendants such as AceRules; and
7. published experience encoding clinical guideline recommendations.

Alternatives supply parts, not the whole chain. EARS, FRET, and BRIDGE-Wiz demonstrate
fixed slots and feedback; Logical English and s(CASP) inform M4 proof verbalization;
PENG/PENGASP demonstrates bidirectional look-ahead; CQL/ELM and FHIR are clinical
interchange targets; and GF is relevant to later multilingual parsing. None replaces the
pinned ACE-to-DRS front end.

Kuhn's PENS classification places ACE around `P4 E3 N4 S3`: deterministically
interpretable and rule/FOL-level expressive, but not small. A restricted clinical profile
should aim for fixed domain semantics and a shorter exact specification, roughly
`P5 E3 N4 S4 D`.

## 3. ACE language facts and gotchas

ACE 6.7 supports facts, existential and universal statements, `if ... then ...` rules,
relative clauses, several verb valencies, measurements, comparisons, generalized
quantifiers, strong negation, negation as failure, and modality. Its relevant modal surface
forms are `can`, `must`, `should`, and `may`, plus sentential forms such as “it is
recommended that”.

The parser represents arithmetic and comparisons but does not evaluate them. ACE is
essentially simple-present and has no native clinical temporal model. A phrase such as “for
7 days” can be parsed, but duration semantics must come from the clinical IR.

ACE removes ambiguity with rigid defaults: prepositional phrases attach to verbs, relative
clauses to the immediately preceding noun, and ambiguous adverbs to the verb on their left.
Coordinator precedence and quantifier scope follow surface rules; plurals default to a
collective reading; and ditransitive readings outrank transitive-plus-preposition readings.

The dangerous cases are silent fallbacks rather than parse failures:

- an unresolved definite noun phrase becomes a new indefinite entity;
- an unknown capitalized word can become a proper name;
- pronouns resolve by accessibility, recency, number, and gender;
- a passive without an explicit agent can become an adjective reading; and
- count/mass or collective/distributive defaults can change meaning.

Clinical ACE should initially use one sentence per assertion or rule, no cross-sentence
anaphora, no pronouns, no unresolved definite descriptions, no inferred proper names, and
no collective plurals. Show authors the canonical interpretation.

## 4. APE and DRS

APE 6.7 is a SWI-Prolog grammar and parser; the published executable identifies itself as
`6.7-131003`. It emits DRS terms and several logic or paraphrase forms. M2 chiefly needs
`-cdrs`, `-cdrspp`, `-cparaphrase1`, and `-cparaphrase2`.

The ACE 6.7 DRS report specifies a reified, relatively flat representation. M2 chiefly
needs implication, ordinary conditions, strong negation, negation as failure, disjunction,
modal boxes, and subordinate or labeled DRS structures. Discourse referents obey
accessibility rules: referents under negation, modality, subordination, questions, or
commands generally do not escape, while an `if` antecedent is accessible in its consequent.

A deterministic APE parse is not complete clinical semantics. The DRS report deliberately
leaves the general semantics of recommendation and admissibility to the application. APE
does not validate unit compatibility, observability, missing-data policy, or numeric
bounds. The project must accept only DRS shapes with an explicit IR mapping.

APE's paraphrase path remains valuable. The audit view should be derived from accepted
structure, equivalent in spirit to `verbalize(parse(text))`, rather than echoing the input.
Test only the supported DRS-to-ACE round-trip fragment.

## 5. Lexicon architecture and M2 policy

ACE handles function words in the grammar and content words in a compiled/common lexicon
(Clex) and runtime user lexicon (Ulex). Ulex entries override matching Clex entries. APE
loads them with `-ulexfile` or `-ulextext`; `-noclex` disables the compiled content lexicon,
and `-guess` enables unknown-word class guessing. The clinical adapter should keep guessing
off and normally run without the broad Clex vocabulary.

Ulex entries are Prolog facts. The specification covers count and mass nouns, measurement
nouns, proper names, three verb valencies, adjectives, adverbs, and prepositions. For
example:

```prolog
noun_sg(patient, patient, human).
noun_pl(patients, patient, human).
tv_finsg(administers, administer).
tv_infpl(administer, administer).
tv_pp(administered, administer).
mn_sg(mg, mg).
mn_pl(mg, mg).
```

Aliases can share one logical symbol. Verb facts encode valency and passive forms; noun
facts carry anaphora-related gender. Multiword terms use hyphenation rather than spaces.

Generate project-owned Ulex facts from a versioned terminology registry, including stable
concept ID, surfaces, part of speech, inflections, valency, count/mass status, and terminology
code. Sort and hash the exact facts. Reject unknown tokens, unregistered names, lexical
ambiguity, wrong valency, and aliases with multiple meanings. Broad Clex vocabulary should
not define production Clinical ACE.

## 6. Clinical-guideline findings from Shiffman et al.

### What was translated

The experiment translated **all eleven key-action statements from one guideline**: the
American Academy of Pediatrics practice parameter *The Diagnosis, Treatment, and
Evaluation of the Initial Urinary Tract Infection in Febrile Infants and Young Children*
(1999). The paper does not report a tuberculosis or otitis-media translation experiment.

Three ACE experts independently translated each statement. A pediatrician, another
physician, and a knowledge engineer judged accuracy and naturalness. All eleven
recommendations were expressible, but independent versions varied. The study tested
feasibility, not inter-author reliability. The complete set also needed a small background
ontology and auxiliary rules to connect the recommendations.

The rules were forward-chained under clinician control, using record-derived or asserted
facts. The appendix assumed only patient and doctor, simplifying but not solving actor
identity.

### What ACE had to add

**Clinical terminology.** ACE had grammar words and a large general lexicon but lacked
specialized medical vocabulary. The authors discussed hooks to UMLS, SNOMED, and LOINC and
used word-class prefixes as a temporary workaround. They proposed an editor that admits
only system-known, clearly defined terms. This directly supports M2's generated Ulex and
M5's terminology picker.

**Level of obligation.** At the study's outset ACE had `can` and `must`, which could not
cover guideline recommendations. The work added `should` / “it is recommended that” and
`may` / “it is admissible that”, including negated forms. The appendix mapped evidence
labels historically to `must`, `should`, and `can`. Do not adopt that conflation: action
direction, deontic force, recommendation strength, and evidence certainty must remain
separate IR fields.

Timing was awkward. A seven-to-fourteen-day course was represented as a therapy that
“lasts at least 7 days” and “lasts at most 14 days”. M2 may preserve that parse, but typed
interval and duration semantics belong in project-owned IR.

### What worked and what did not

ACE accurately represented the study's clinical concepts and actions, and a pediatrician
judged the resulting sentences acceptably natural. Formalization also exposed source defects:
vague conditions, facts presented as recommendations, passive voice hiding the actor,
non-executable advice, and the word “consider”, whose fulfillment cannot be measured.

CNL does not resolve those omissions automatically. It forces an author to decide who acts,
under what conditions, with what obligation, and how success can be observed. Variation
among the three translations also shows why M5 needs one canonical authoring profile and
review of the machine-derived interpretation.

### Proposed authoring tool

The paper proposed a look-ahead, WYSIWYM-style editor with four modules:

1. guideline-quality prompts and reusable COGS documentation;
2. a wizard for evidence quality, benefits, harms, costs, and recommendation strength;
3. predictive ACE editing that offers only syntactically valid next words and flags terms
   such as “consider”; and
4. transformation of accepted rules into a computable guideline format and decision-support
   system.

The durable lesson is to make invalid text difficult to enter and display the machine's
current interpretation continuously.

## 7. Explicit M2 design decisions

1. **Pin the parser environment.** Record the APE fork revision, Clinical ACE profile,
   generated Ulex hash, and SWI-Prolog 9.2.9 runtime in every artifact.
2. **Keep stable IDs outside prose.** Assign immutable guideline-document and sentence or
   recommendation IDs in the registry. Never derive identity from line numbers, APE token
   positions, or DRS variable names. Record revision/predecessor relations explicitly.
3. **Preflight tokens.** Reject unknown words, unregistered proper names, bad units, and
   lexical-category ambiguity before APE. Never enable `-guess`.
4. **Parse one bounded record at a time.** Avoid cross-record anaphora and hidden parser
   state; enforce input-size and nesting limits.
5. **Whitelist DRS constructors.** APE success is not profile acceptance. Reject every
   quantifier, modality, subordination, reference form, arithmetic form, or nested DRS
   without a specified IR meaning.
6. **Turn fallbacks into diagnostics.** Unresolved definites, capitalization-based names,
   passive/adjective ambiguity, and inaccessible references fail loudly.
7. **Canonicalize structurally.** Parse the Prolog term; alpha-rename referents in a fixed
   traversal; serialize with project-owned UTF-8/LF, escaping, and numeric rules. Preserve
   order where accessibility depends on it and sort only proved-commutative collections.
8. **Separate semantics from source mapping.** Keep document ID, sentence ID, source span,
   and original text in an envelope or sidecar, outside the semantic DRS hash.
9. **Show an interpretation view.** Store a canonical paraphrase derived from accepted
   structure and require author confirmation in M5.
10. **Prove reproducibility.** Golden fixtures must be byte-identical across fresh
    processes, with negative profile fixtures and pinned-APE differential fixtures.

Useful errors include `LEX_OOV`, `LEX_AMBIGUOUS`, `REF_UNRESOLVED`, `REF_INACCESSIBLE`,
`DRS_UNSUPPORTED`, `MODAL_UNMAPPED`, `UNIT_UNKNOWN`, and
`CANONICALIZATION_MISMATCH`.

## 8. Consequences for M4

The DRS-to-IR mapping must preserve distinctions Prolog makes operationally important:

- implication becomes an antecedent and consequent;
- strong negation is not failure to prove a positive fact;
- negation as failure is explicit and must not be inferred from missing patient data;
- `must`, `should`, `may`, and `can` remain typed wrappers until domain semantics are fixed;
- disjunction becomes explicit alternatives;
- quantities become typed values with normalized units and open/closed bounds; and
- exceptions remain labeled, source-linked objects rather than anonymous negative atoms.

AceRules is useful precedent for executable controlled-language rules and verbalized proof
traces, not an implicit M4 specification. The project must define priorities, exceptions,
conflict resolution, and missing-data behavior, then compile accepted IR into a small
reviewed Prolog kernel. Explanations should come from proof objects and retain rule IDs,
source IDs, matched conditions, checked exceptions, and overrides.

## 9. M5 authoring checklist

For each recommendation, record the exact source span and immutable ID; identify population,
trigger, actor, action, object, direction, strength, certainty, timing, quantities, and each
exception; select content words from the versioned lexicon; make every `and` / `or` grouping
explicit; parse and show the canonical interpretation; then test positive, negative,
exception, missing-data, and numeric-boundary scenarios.

Lint hidden actors, vague actions, unobservable conditions, unsupported terms, missing
strength or certainty, incompatible units, implicit timing, unresolved references,
unsatisfiable contexts, and exceptions that duplicate or can never overlap the rule.

## 10. Conditional Japanese relevance

ACE is English. If Japanese is later required, keep the typed IR as shared semantic
authority and start with deterministic generation, not broad Japanese parsing. Templates
should use explicit actors and particles, no zero pronouns, one proposition per sentence,
fixed terminology, and exact bound words such as `以上`, `以下`, `未満`, and `超`. Parse only
the system's generated subset if round-trip becomes necessary. GF becomes worthwhile when
Japanese input or several languages justify a shared bidirectional grammar; still require
exactly one parse for every canonical sentence.

## References

- [Shiffman et al. 2010](../references/shiffman2010-writing-cpg-in-cnl.pdf)
- [ACE 6.7 DRS report](../references/drs-report-67.pdf)
- [AceRules](../references/kuhn2007-acerules.pdf)
- [Kuhn's CNL survey](../references/kuhn2014-cnl-survey.pdf)
- [Kaljurand's dissertation](../references/kaljurand2007-ace-semantic-web.pdf)
- [ACE Construction Rules](https://attempto.ifi.uzh.ch/site/docs/ace_constructionrules.html)
- [ACE Interpretation Rules](https://attempto.ifi.uzh.ch/site/docs/ace_interpretationrules.html)
- [ACE Lexicon Specification](https://attempto.ifi.uzh.ch/site/docs/ace_lexicon.html)
