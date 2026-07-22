# Guideline registry contract

The guideline registry records acquired source artifacts, rights evidence, extraction
evidence, and byte-addressed source regions. A separate terminology registry records the
versioned lexical entries from which production Ulex bytes are emitted. Both formats are
strict UTF-8 canonical Prolog term streams.

The mapping-store grammar for claims and typed residuals is added by M5.1b.

## Tool interface

Run the validator from the repository root:

```text
swipl -q -f none -F none -s src/prolog/registry_tool.pl -g main -t 'halt(9)' -- registry
swipl -q -f none -F none -s src/prolog/registry_tool.pl -g main -t 'halt(9)' -- terminology
swipl -q -f none -F none -s src/prolog/registry_tool.pl -g main -t 'halt(9)' -- ulex
```

Each command reads exactly one file from binary stdin. `registry` and `terminology` emit
zero bytes on success. `ulex` validates a terminology file and emits the derived canonical
Ulex stream. Output is buffered and reaches stdout only after the complete operation
succeeds.

Input must be strict RFC 3629 UTF-8 with LF line endings and no byte-order mark. Every term
is serialized with quoted atoms, ignored operators, numbered variables, character escapes,
a final `.`, and one LF. The decoded input code points must byte-for-code-point equal that
canonical reserialization. This canonical fixed point applies before the selected grammar
is validated.

## Registry v1 grammar

The first term is exactly:

```prolog
cnl_guideline_registry(1).
```

It is followed by one or more `guideline_source/11` rows, then zero or more rows from each
later section in this fixed order:

1. `extraction_evidence/5`
2. `guideline_region/6`
3. `guideline_item_state/3`
4. `guideline_blocked_proposal/4`

Rows within a section are strictly increasing by their documented key. Equal keys are
duplicates; a lower key is out of order. Atom comparison is lexicographic by Unicode scalar
value, independent of locale.

### Source rows

```prolog
guideline_source(
    GuidelineId,
    title(Title),
    issuing_organization(Organization),
    urls(landing_url(LandingUrl),doi(Doi),artifact_url(ArtifactUrl)),
    version(VersionOrDate),
    language(LanguageTag),
    artifact(relpath(ArtifactPath),artifact_sha256(ArtifactHash),byte_length(ArtifactBytes),media_type(ArtifactMediaType)),
    retrieval(first_retrieved_at(FirstRetrievedAt),second_retrieved_at(SecondRetrievedAt),fetch_count(FetchCount),byte_identical(ByteIdentical)),
    cross_manifestation(kind(Kind),url(EvidenceUrl),evidence_sha256(EvidenceHash),byte_length(EvidenceBytes),media_type(EvidenceMediaType),first_retrieved_at(EvidenceFirstRetrievedAt),second_retrieved_at(EvidenceSecondRetrievedAt),fetch_count(EvidenceFetchCount),byte_identical(EvidenceByteIdentical)),
    rights(copyright_status(CopyrightStatus),rights_label(RightsLabel),rights_evidence(quote(RightsQuote),url(RightsUrl),retrieved_at(RightsRetrievedAt)),obligations(Obligations),derivative_mode(DerivativeMode),redistribution_status(RedistributionStatus),may_commit(MayCommit),attribution_text(AttributionText)),
    publication_status(Status,status_checked_at(StatusCheckedAt),evidence(url(StatusEvidenceUrl),updated_at(StatusEvidenceUpdatedAt)))).
```

The source-row key is `GuidelineId`. All scalar fields are nonempty atoms except numeric
byte and fetch counts and the two byte-identity booleans. Byte lengths are positive
integers; fetch counts are at least two; byte-identity values are `true` or `false`.

`ArtifactPath` is the committed primary artifact. `artifact_sha256` names the SHA-256 of
those exact bytes. A cross-manifestation row is evidence only and does not imply that its
bytes are committed. Its digest therefore uses the distinct field name `evidence_sha256`.
Every SHA-256 value is a 64-character lowercase hexadecimal atom.

`Obligations` is a strictly sorted unique proper list drawn from `attribution`, `no_marks`,
`non_endorsement`, and `source_free_availability`. The v1 rights vocabularies are:

| Field | Legal atoms |
|---|---|
| `CopyrightStatus` | `public_domain`, `licensed`, `restricted`, `unknown` |
| `DerivativeMode` | `project_authored_mapping`, `source_adaptation`, `none` |
| `RedistributionStatus` | `redistributable`, `reconstructable`, `restricted_internal_only` |
| `MayCommit` | `yes`, `conditional`, `no` |
| `Status` | `current_as_observed`, `superseded`, `archived`, `unknown` |

A `current_as_observed` value records the result of the named status check; it is not a
promise that the issuer will never publish a later version.

### Extraction-evidence rows

```prolog
extraction_evidence(GuidelineId,ExtractionId,relpath(ExtractionPath),extraction_sha256(ExtractionHash),artifact_relpath(ArtifactPath)).
```

The key is `(GuidelineId, ExtractionId)`. `GuidelineId` must name a source row, and
`ArtifactPath` must equal that source row's committed artifact path. `ExtractionHash` is the
SHA-256 of the complete extraction-evidence file. The extraction file, not rendered PDF
layout, is the byte authority for region offsets.

### Region rows

```prolog
guideline_region(GuidelineId,RegionId,ExtractionId,pdf_pages(PhysicalFirst,PhysicalLast,PrintedFirst,PrintedLast),byte_range(Start,End),region_sha256(RegionHash)).
```

The ordering key is `(GuidelineId, RegionId)`, while `RegionId` itself is globally unique.
The source and extraction IDs must exist. Page numbers are positive integers with first not
greater than last. `Start` and `End` are nonnegative integer byte offsets into the named
extraction file, interpreted as the 0-based half-open range `[Start, End)`; `Start` must be
less than `End`. `region_sha256` is the SHA-256 of exactly those bytes.

The CDC Box 3 identifiers are `box3.rec.01` through `box3.rec.12` for recommendation
statements and `box3.grp.01` through `box3.grp.04` for group headings. HTML ordered-list
numbers are presentation markup. The extraction file retains readable `N. ` labels, but
each recommendation range begins after that prefix and ends before its LF. Group-heading
ranges contain only the heading text.

### Reserved workflow rows

```prolog
guideline_item_state(GuidelineId,ItemId,State).
guideline_blocked_proposal(GuidelineId,ItemId,ProposalId,reason(Reason)).
```

The item-state key is `(GuidelineId, ItemId)` and `State` is exactly one of `done`,
`blocked`, or `excluded`. The blocked-proposal key is
`(GuidelineId, ItemId, ProposalId)`. Its source and item-state rows must exist, the matching
state must be `blocked`, and `Reason` is a nonempty atom. These rows are legal but optional
in v1; they reserve stable vocabulary for the later workflow milestone rather than adding
frontier behavior here.

## Terminology v1 grammar

The first term is exactly:

```prolog
cnl_guideline_terminology(1).
```

Every later term is:

```prolog
terminology_entry(EntryId,Template,english_surface(Surface)).
```

`EntryId` is a globally unique stable ID. `Surface` is the nonempty human-readable English
surface represented by the entry. `Template` is exactly one of the 27 Ulex templates in the
[user lexicon contract](ulex.md):

```prolog
adv(WordForm,Lemma).
adv_comp(WordForm,Lemma).
adv_sup(WordForm,Lemma).
adj_itr(WordForm,Lemma).
adj_itr_comp(WordForm,Lemma).
adj_itr_sup(WordForm,Lemma).
adj_tr(WordForm,Lemma,Preposition).
adj_tr_comp(WordForm,Lemma,Preposition).
adj_tr_sup(WordForm,Lemma,Preposition).
noun_sg(WordForm,Lemma,Gender).
noun_pl(WordForm,Lemma,Gender).
noun_mass(WordForm,Lemma,Gender).
mn_sg(WordForm,Lemma).
mn_pl(WordForm,Lemma).
pn_sg(WordForm,Lemma,Gender).
pn_pl(WordForm,Lemma,Gender).
pndef_sg(WordForm,Lemma,Gender).
pndef_pl(WordForm,Lemma,Gender).
iv_finsg(WordForm,Lemma).
iv_infpl(WordForm,Lemma).
tv_finsg(WordForm,Lemma).
tv_infpl(WordForm,Lemma).
tv_pp(WordForm,Lemma).
dv_finsg(WordForm,Lemma,Preposition).
dv_infpl(WordForm,Lemma,Preposition).
dv_pp(WordForm,Lemma,Preposition).
prep(WordForm,Lemma).
```

Every template argument is a nonempty atom. A gender position accepts exactly `undef`,
`neutr`, `human`, `masc`, or `fem`. Terminology v1 deliberately requires a named
preposition atom for ditransitive templates; it does not admit the Ulex empty-atom
shorthand.

Rows are strictly increasing by `(kind, WordForm)`, comparing first the template functor
atom and then its first argument by Unicode scalar value. No two rows may share that key,
and no two rows may share `EntryId`. Lemmas may repeat across inflections and approved
aliases.

## Ulex emission

`ulex` performs the same terminology validation, then emits only each row's `Template`.
The output is UTF-8 with LF line endings, one canonical fact per line ending in `.`, in the
validated strict `(kind, WordForm)` order. The output contains no version fact, entry ID, or
`english_surface/1` wrapper. Consequently identical terminology bytes produce identical
Ulex bytes in fresh processes.

This emission contract is the deterministic-production requirement in
[the user lexicon contract](ulex.md). Intersection checks remain the responsibility of APE
when the generated file is loaded; registry validation proves template shape, ordering, and
uniqueness, not the complete APE cross-category intersection policy.

## Naming rules

Guideline, extraction, region, entry, item, and proposal IDs are lowercase ASCII stable IDs.
They start with a letter, contain letters or digits inside each segment, and use single `-`
or `.` separators between nonempty segments. Underscores are reserved for Prolog constructor
names. IDs are permanent provenance handles and must not be reassigned after publication.

Repository paths are nonempty POSIX-relative atoms: no leading `/`, empty segment, `.`
segment, or `..` segment is legal. URLs, media types, timestamps, titles, labels, reasons,
and surface strings are preserved as nonempty atoms. V1 records timestamps as supplied by
the acquisition evidence; UTC instants use a trailing `Z`.

## Errors and exits

A rejection writes exactly one canonical LF-terminated line to stderr and zero bytes to
stdout:

```prolog
registry_tool_error(Stage,Class,Detail).
```

`Stage` is `registry`, `terminology`, `ulex`, or `cli`. The version fact is checked before
any row, so a file supplied to the wrong subcommand deterministically fails with class
`version` rather than with a row error.

| Class | Meaning | Exit |
|---|---|---:|
| `input_utf8` | Stdin is not strict RFC 3629 UTF-8. | 1 |
| `syntax` | The decoded Prolog stream cannot be parsed. | 1 |
| `canonical` | Canonical reserialization differs from input. | 1 |
| `version` | The required v1 version fact is missing or different. | 1 |
| `row` | A row constructor or arity is unknown, or no required source/entry row exists. | 1 |
| `shape` | A nested constructor, scalar type, ID, path, enum, count, page locator, or list is invalid. | 1 |
| `digest` | A digest is not exactly 64 lowercase hexadecimal characters. | 1 |
| `range` | A byte range or page interval is reversed or empty. | 1 |
| `reference` | A source, artifact, extraction, state, or blocked-state reference does not resolve. | 1 |
| `duplicate` | A stable ID or semantic key is repeated. | 1 |
| `ordering` | Registry sections or rows are not in strict key order. | 1 |
| `template` | A terminology template kind or arity is not one of the 27 admitted forms. | 1 |
| `gender` | A gender atom is outside the admitted vocabulary. | 1 |
| `usage` | Arguments do not select exactly one subcommand. | 2 |
| `uncaught` | An unexpected internal exception escaped validation or serialization. | 2 |

Exit 0 means success, exit 1 means input-content rejection, and exit 2 means usage or
internal failure. Validation is deterministic and reports the first failure in the pass
order above.

## Rights posture

A digest proves byte identity, not permission. A source row therefore preserves the issuer,
rights label, verbatim evidence quote, evidence URL and retrieval time, obligations,
redistribution decision, commit decision, attribution text, and separately observed
publication status. Rights apply only to the identified artifact and region evidence; they
do not silently extend to logos, separately credited third-party material, terminology, or
other manifestations.

Project-authored ACE, terminology, mappings, and logical artifacts must be labeled as
project derivatives. They do not alter the registered source text and must not imply issuer
endorsement. Artifact-adjacent attribution and non-endorsement requirements remain binding
when registered public-domain material is redistributed.
