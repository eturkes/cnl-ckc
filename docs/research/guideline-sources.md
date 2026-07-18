# Guideline and corpus sources for M5

Plan-phase research findings (2026-07), checked against the cited public pages. Every acquired version still needs a registry row with rights holder, URL, observed version/date, digest, permitted uses, obligations, redistribution status, and `may_commit`.

## 1. Permission vocabulary

- `yes`: identified artifact is public domain, CC0, or clearly redistributable under obligations this repository can meet.
- `conditional`: commit only expressly allowed artifact classes/adaptations, with attribution and license separation.
- `no`: keep raw content outside Git; commit metadata, digests, offsets, and allowed independent mappings.
- `unknown`: no usable grant established; treat as `no` until reviewed.

A page, publication, repository, annex, and embedded terminology can have different rights. Never inherit one artifact’s license across the others.

## 2. English candidates

| Source | Organization; observed version/date | URL | Rights/access | Format; ACE fitness | `may_commit` |
| --- | --- | --- | --- | --- | --- |
| **CDC Clinical Practice Guideline for Prescribing Opioids for Pain — United States, 2022** | CDC; 2022-11-04; 12 numbered recommendations in four groups; DOI `10.15585/mmwr.rr7103a1` | `https://www.cdc.gov/mmwr/volumes/71/rr/rr7103a1.htm`; `https://pmc.ncbi.nlm.nih.gov/articles/PMC9639433/` | MMWR material is public domain and reusable without permission; cite CDC, avoid agency marks, inspect separately credited material. | HTML/PDF/PMC XML. **High:** explicit adult exclusions, direction, dose, duration, follow-up, harms. Use the 12-recommendation skeleton, not the full 95-page rationale. | **yes**, subject to third-party exceptions |
| **WHO Digital Adaptation Kit for Antenatal Care** | WHO; 2021-02-17; ISBN `9789240020306`; 88 pages + four annexes | `https://www.who.int/publications/i/item/9789240020306` | CC BY-NC-SA 3.0 IGO: attribution, noncommercial, ShareAlike. Not silently Apache-relicensable. | PDF + XLSX data dictionary, decision logic, indicators, requirements. **High semantics, medium first-slice fit:** strong oracle but too large wholesale. | **conditional**, segregated under IGO terms |
| **WHO SMART ANC repository** | WHO; `v0.3.0`, 2025-07-10 | `https://github.com/WorldHealthOrganization/smart-anc` | Repository `LICENSE.md` is CC0. The implemented WHO publication has separate IGO terms; record both and identify reused bytes. | FHIR R4 IG, CQL, PlanDefinition/Library bundles/tests. **High differential-oracle fit:** independently author ACE for a few recommendations, then compare behavior. | **yes** for CC0-covered repo bytes; otherwise conditional |
| **WHO Digital Adaptation Kit for Immunizations** | WHO; publication 2025-01-27, ISBN `9789240099456`; repository `v1.0.0`, 2026-04-09 | `https://www.who.int/publications/i/item/9789240099456`; `https://github.com/WorldHealthOrganization/smart-immunizations` | Publication and repository license text: CC BY-NC-SA 3.0 IGO. | 96-page PDF + extensive CQL/FHIR. **Medium first-slice, high later:** use one schedule to stress age, sequence, timing, exceptions. | **conditional** |
| **HL7 CPG-on-FHIR** | HL7 CDS WG; `hl7.fhir.uv.cpg#2.0.0`, STU2, active 2024-11-26 | `https://hl7.org/fhir/uv/cpg/` | IG uses CC0; embedded UCUM/LOINC/SNOMED/other terminologies retain separate terms. | FHIR R4 IG with CHF, CKD, VA CKD and activity examples. **Medium:** excellent conformance oracle, not authoritative narrative guidance. | **yes** for CC0 examples excluding third-party terminology |
| **EBM-NLP** | Nye et al.; ACL 2018; 4,993 PICO-annotated abstracts | `https://github.com/bepnye/EBM-NLP` | License observed as MIT; verify exact data/code and article-text rights before vendoring. | Span annotations. **Low as guideline; useful for population/intervention/outcome annotation design.** | **conditional** |
| **GGPONC 2.0** | Charité/German Guideline Program in Oncology; LREC 2022 | `https://aclanthology.org/2022.lrec-1.389/` | No project-ready redistribution decision established; German and SNOMED-grounded. | Annotated oncology guideline corpus. **Low for first English source; useful later for adjudication design.** | **unknown** |

## 3. Japanese/regulatory sources: conditional relevance

| Source | Organization; observed version/date | URL | Rights and format | Pipeline relevance; `may_commit` |
| --- | --- | --- | --- | --- |
| **Minds library/methodology** | Japan Council for Quality Health Care; manual 2020 v3.0, templates updated 2024-02 | `https://minds.jcqhc.or.jp/` | Public metadata; full-text rights usually remain with society/publisher. Mostly Japanese PDF/HTML. | Strong CQ→PICO→EtD→direction/strength/certainty scaffold; raw full text **no** absent source permission. |
| **J-SSCG2024** | Japanese sepsis guideline; official 2024-12 | Minds/society pages | Society copyright; no blanket grant recorded. | Relevant to sepsis conformance thread, but Japanese and rights-unclear: **unknown/no**. |
| **Hypertension Management and Treatment Guideline 2025** | Japanese Society of Hypertension; observed 2025-08-29 | Minds/society pages | Public availability does not establish redistribution. | Threshold-rich future source: **unknown/no**. |
| **Osteoporosis Prevention and Treatment Guideline 2025** | Japanese society; 2025 edition | Minds/society pages | No reusable-content grant established. | Age/risk thresholds; poor M5 source: **unknown/no**. |
| **JCS/JHFS 2025 Heart Failure Guideline** | Japanese Circulation Society/Japanese Heart Failure Society; 2025 | `https://www.jstage.jst.go.jp/` | J-STAGE metadata and sometimes structured full text; license is item-specific. | Future Japanese XML/extraction and cross-society source: **conditional per article**. |
| **PMDA electronic package inserts** | PMDA; XML aligned to 2019 rules; electronic regime from 2021, paper inserts abolished 2023 | `https://www.pmda.go.jp/PmdaSearch/` | Free access/download; no blanket redistribution grant established. Japanese XML/PDF. | High future fit for dose/contraindication and guideline-vs-label conflicts; raw content **no** until permission row. |
| **MEDIS standard masters** | MEDIS-DC; overview 24th ed., 2025-07 | `https://www.medis.or.jp/` | Free download with attribution terms observed; verify each master/version. CSV/Excel/Access. | Terminology, not guideline: disease/HOT/JLAC identity; **conditional**. |
| **FHIR JP Core** | JAMI/Japan FHIR WG; v1.2.0-a observed 2025-01-24, v1.3.0-dev observed 2026-01-28 | `https://jpfhir.jp/fhir/core/` | Open IG posture; embedded code systems retain ownership. | Patient-data schema, not recommendation content; **conditional**. |
| **JADER** | PMDA; public CSV since 2012 | PMDA public-data pages | Public download; exact reuse row still required. | Adverse-event evaluation source, not guideline; no denominator. |

These sources remain strategically useful for later Japanese coverage or cross-source auditing, but language, extraction, and item-specific rights make them poor first M5 choices.

## 4. Registry design precedents

Plan-phase pipeline experiments left two registry lessons:

- An experiment registry that tracks only pipeline and stage identity cannot govern acquired documents: without acquisition URL, license evidence, content digest, and `may_commit`, no redistribution decision is auditable. The §5 row is the corrective.
- Comparing a fully layered deterministic pipeline, a single accepted-IR stage, and direct formal-target emission: an accepted, validated IR boundary localizes uncertainty best, and deterministic stages then chain artifacts by digest. This project goes further — every ACE→DRS→IR→Prolog stage is deterministic.

Synthetic test documents earn their keep and should be authored as M5 fixtures: one conformance guideline, one opposing-rule guideline, and one no-conflict control, each registered like a real source but flagged AI-generated synthetic with no external rights.

## 5. M5 source-registry row

Minimum fields:

```text
id, title, organization, canonical_url, retrieved_at,
version_or_date, content_sha256, media_type, language,
rights_holder, license_label, license_url,
acquisition_allowed, internal_processing_allowed,
derivative_authoring_allowed, redistribution_status,
may_commit, attribution_text, notice_paths, notes
```

Use `redistribution_status = redistributable | reconstructable | restricted_internal_only`. Blocked exports emit a permission residual without blocking local processing. English↔ACE mappings cite region IDs/digests; store source sentences only when the permission row allows it.

## 6. Shortlist

1. **CDC opioid recommendation skeleton — clear winner.** Public domain, English, 12 structured recommendations; exercises population exclusions, direction, dose/duration, and harm mitigation at manageable size.
2. **WHO SMART ANC subset — best independent oracle.** Select 3–5 recommendations with eligibility, timing, and exceptions; compare ACE→IR→Prolog to CC0 repository artifacts. Keep CC0 repo evidence distinct from the IGO narrative license.
3. **WHO immunization single-schedule slice — best temporal follow-up.** Exercises age intervals, dose sequence, delayed schedules, and contraindications. Clear IGO license, but noncommercial/ShareAlike separation is required and the full corpus is too large.

Minds/J-STAGE/PMDA should follow once Japanese coverage and permission-aware ingestion are ready.
