# compendium

Master list of eligible American clinical guidelines available online:
the issuing-organization universe plus, per swept organization, one row
per guideline. Discovery ledger only — a guideline is fetched exactly
when its row is promoted into `.agent/queue.md` (procedure: root
`TECHNICAL.md` § Operating).

Construction is staged: the organization universe and one
seed guideline per confirmed issuer land first, per-org index sweeps fill
the guideline rows, and an aggregator cross-check closes the gap list.
Until every organization row reaches a terminal sweep state, the
guideline list is a seed roster, not the complete list.

Coverage is also tiered. The easy tier covers organizations whose index
enumerates in ≤2 anonymous fetches and whose artifacts classify without
an authenticated session; the rest carry `swept = blocked(<why>)` as
named, enumerated gaps — hard-tier harvest is parked as its own later
phase, opening at the terminal condition below (roadmap stub; register
= `.agent/archive/hard-tier-register.md`). A bounded fan-out is easy
tier as well: an index whose entries are topic subindexes qualifies when
the fan-out is enumerable from the index in one fetch, bounded (≤30
subindexes), and every subindex is anonymous and static — enumeration then
costs `1 + <subindexes>` fetches and the sweep method reads
`per-topic-pages`. So a compendium satisfying
the terminal condition below is exhaustive over its swept organizations
and explicit about the organizations it defers — not a claim that every
American clinical guideline holds a row.

Terminal condition — compendium exhaustion, the standing goal's finite
stop check, reached when both hold:

- every organization row is `CPGs=no`, or carries `swept` =
  `<date> <method>` or `blocked(<why>)`;
- every guideline row is `done`, `blocked(<why>)`, or `excluded(<why>)`.

Root `TECHNICAL.md` § Operating adopts this as the goal's terminal clause.

## Eligibility

A guideline gets a row when all hold:

1. American issuing body — US federal agency or panel, US professional
   society/association, US disease-focused organization, or US-led
   multi-society collaboration. A body headquartered abroad with globally
   drawn work groups and no US organization as lead or co-lead sponsor is
   foreign, whatever the nationality of individual chairs.
2. Clinical practice guideline — systematically developed recommendations
   bearing on patient care (screening, diagnosis, treatment, prevention,
   management). Not eligible: evidence reviews without recommendations,
   methodology/coding/billing/administrative documents, patient-facing
   education alone.
3. Current version available online, open or gated.

Rulings (edge rules; extended as cases are ruled):

- US-led multi-society or international collaborations: eligible when a US
  organization is lead or co-lead sponsor; foreign-led work merely
  endorsed by a US body → `excluded(non-American)`.
- Federal advisory panels (ACIP, USPSTF, NIH-convened panels): eligible;
  the convening agency is the issuing org.
- Versions: one row per guideline, the latest current version; a
  superseded edition never gets its own row. Retired without replacement →
  `excluded(retired)`.
- Position statements, consensus statements, appropriate-use criteria:
  eligible when they carry actionable patient-care recommendations;
  otherwise `excluded(no patient-care recommendations)`.
- Endorsement without issuance never makes an org an issuer: a guideline
  row belongs to its issuing org(s). A co-issued guideline is one row that
  satisfies every issuer's seed obligation, and each American co-issuer
  holds its own organization row, co-issuance being issuance. That row
  need not already read `CPGs=yes`: the artifact names who issued it,
  while `CPGs` records whether ≥2 independent enumeration sources have
  established the org as an issuer, which its own sweep settles. A
  co-issuer a sweep discovers therefore enters the roster `unverified`
  with `swept=pending`, and the guideline row ships immediately —
  harvesting an artifact is never hostage to the org universe. Naming an
  org that holds no row at all is a contradiction and fails the gate, as
  does an ELIGIBLE row naming one ruled `CPGs=no`; an `excluded(<why>)`
  row may name such an org, since `CPGs=no` denies eligible issuance while
  that org's own sweep is required to record the entries it ruled out.
  Discovering an eligible artifact the org co-issued refutes a `CPGs=no`
  ruling — the row returns to `unverified` with `swept=pending`, its
  `excluded:` reason dropped, because the ruling was made from the org's
  own site and co-issuance is issuance wherever it is published. Foreign
  co-issuers are not issuing orgs here and stay out of the roster. (Row
  formats owns how both are written.)
- A partner named in the artifact's own TITLE co-issued it when the
  artifact records that partner in development — a writing-committee
  representative it appointed, named in the committee roster sentence or
  as a `<ABBR> rep` annotation. A body appearing only in an endorsement
  list did not. ACC/AHA guidelines state both in one footnote
  (`Developed in Collaboration With and Endorsed by`), so the roster is
  what decides: SGIM sits in the 2025 hypertension guideline's
  "writing committee included representatives from" sentence and SHM
  appointed two named reps to the 2026 pulmonary embolism guideline, so
  both co-issue despite each also endorsing.
- Org-level `CPGs=yes` = the org issues ≥1 eligible artifact;
  artifact-level eligibility is ruled per row during the org's sweep.
- Artifact type never decides eligibility by name — clinical guidance,
  practice parameters, criteria, procedure standards, committee opinions,
  and algorithms qualify when systematically developed and carrying
  patient-care recommendations, and a nominal "guideline" that carries
  neither does not.
- Patient means human patient. Veterinary guidance — including the
  military working-dog guidelines a federal trauma index carries beside
  its human ones — is `excluded(veterinary; not human patient care)`.
- A derivative rendering of another artifact's recommendations is not a
  guideline of its own and gets no row: infographics, pocket cards,
  executive summaries, quick-reference guides, slide sets, and training
  curricula collapse into the guideline they restate. Collapse is a
  dedupe, so it never produces an `excluded(<why>)` row; it is the
  difference the sweep manifest reconciles between `<n>` and `<e> + <x>`.
  A derivative whose parent guideline is absent from the index is instead
  `excluded(derivative; parent guideline not issued here)`.

## Protocol

Discovery-completeness is established in three auditable layers:

1. Organization universe — every candidate issuing org holds a row in
   `## Organizations`. `CPGs=yes` rows carry ≥2 independent enumeration
   sources; `unverified` candidates may carry one until their sweep
   resolves them.

   Two sources are independent when they have distinct owners and
   distinct datasets, and neither is controlled by the issuer or by the
   guideline's developer. All NLM properties (PubMed, PMC, NCBI
   Bookshelf) share one owner, so any number of NLM records count once,
   as do repeated records from any single dataset. Every bibliographic
   source cites a stable identifier — `PubMed PMID <n>`, `DOI <doi>`,
   `NCBI Bookshelf NBK<n>`, `GovInfo <id>` — so the claim is checkable
   from this file alone.
2. Per-org sweep — each CPG-issuing org's guideline index is swept into
   one row per eligible guideline; the org row's `swept` cell records
   date + method (the provenance line for that org's guideline rows).
   A sweep that rules the org a non-issuer records the sweep and sets
   `CPGs` to `no`.
3. Aggregator cross-check — independent aggregator listings are diffed
   against the org-derived rows; gaps become rows; the diff record lands
   in this section when run.
   Diff record (M1.3b): four bulk sources (GC live profiles; NGC
   historical caches — org/count level, zero title rows; AAFP-PG
   676-record cache; PubMed corporate-author, CollectiveName-validated)
   × the 23 M1-swept/blocked orgs → 669 candidate gaps at gap-list
   grade across 16 orgs (ACC 321, PubMed capped at 294 hits; USPSTF
   130, enumeration was blocked so gaps expected; ACIP 110; ACG 97;
   ASA 44; AASM 42; rest ≤31). `uncertain` = normalized-substring
   match only. Gaps stay unadjudicated candidates (out of project
   scope) — a future ruling would settle eligibility, version + access
   per gap via the off-index mechanism; shipped rows stay untouched. Regeneration:
   `.scratch/m1u3b/gapcheck/build_gaps.py` (cache-only replay
   byte-identical; gaps in `gapcheck/gaps.tsv`).

## Row formats

Organizations — `| org | abbrev | class | CPGs | index URL | enum sources | swept |`

- `class` = `federal` (US federal agency, panel, or program) | `society`
  (membership-based professional association) | `other` (every other
  eligible issuer — disease foundations, coalitions, consortia,
  standalone guideline programs). `CPGs` = `yes|no|unverified`.
- `abbrev` = display shorthand, never a key; collisions are
  disambiguated (`ACR-Radiology`, `ACR-Rheumatology`) purely for reading.
- `enum sources` = independent listings confirming the org
  (semicolon-separated; ≥2 for `CPGs=yes`). `CPGs=no` rows carry
  `excluded: <reason>` here instead.
- `swept` = `pending | <date> <manifest> | blocked(<why>)`; `CPGs=no` rows
  swept before the ruling record that sweep, unswept ones use `-`.

  A swept `CPGs=yes` org states its sweep as a manifest,
  `YYYY-MM-DD <method>; <n> index entries -> <e> eligible + <x> excluded`,
  where `<e>` and `<x>` count that org's guideline rows below and are
  checked against them, so the provenance claim cannot drift from the
  table it describes. `<n>` counts the artifact entries the index offered
  — navigation to another site's directory or clearinghouse is not an
  entry; `n > e + x` means entries collapsed as duplicates, aliases,
  superseded editions, or derivatives, and the sweep report carries that
  reconciliation.

  A manifest reconciles an index, so each row belongs to the sweep whose
  index surfaced it. An entry one org indexes and another org issues takes
  its row under the ISSUER, carries `indexed-by=<indexing org>` in notes,
  and counts in the INDEXING org's manifest — review, input, or
  endorsement is not issuance, so the indexer stays out of the org cell.
  The marker names every index that carried the row, joined with ` + `,
  which is what lets an artifact its issuers and a third org all index
  count once in each of their manifests; naming exactly the issuers is
  already the default, so the marker is then omitted.
  A row no org index carried — a layer-3 cross-check find, or a current
  artifact an index has yet to add — reads `off-index(<why>)` in notes and
  belongs to no manifest at all, since there is no index to reconcile it
  against. A co-issued artifact that BOTH issuers' indexes carry counts in
  both manifests against the one row the table holds: each index did offer
  that entry, so each sweep must account for it, and a silent sweep would
  instead read as a collapse. Both reports emit the row identically and
  the merge deduplicates it. `<method>` names the
  enumeration route: `static-list`, `pdf-index`, `per-topic-pages`,
  `paginated(<n> pages)`, `search-api(<endpoint>)`, `reader-proxy`, joined
  with `+` when a route needs more than one.

Guidelines — `| org | title (year) | URL | access | status | notes |`

- `org` = every American issuing org, joined with ` + ` for co-issued
  work; a foreign co-issuer is named in `notes` instead.
- `title (year)` — year = the artifact's publication or version year; a
  guideline whose label year differs reads `title (label-year; pub
  YYYY)`.
- `URL` = the canonical current artifact — full-text HTML, PDF, or
  publisher DOI landing. Never a capability/token URL, a table of
  contents, a landing page, or a bibliographic citation when the artifact
  itself is reachable.
- `access`, classified from the artifact rather than from transport:
  `open` (retrievable anonymously) | `paywalled(<gate>)` |
  `login(<gate>)` | `unverified` (an anti-bot challenge or equivalent
  blocks classification — never an access class of its own).

  The evidence is an anonymous probe of the canonical artifact URL. A
  scripted-user-agent refusal that a reader proxy clears is transport, so
  the artifact reads `open`; a challenge the proxy also fails leaves
  `unverified` + `provisional(...)`; an entitlement prompt in the artifact
  body names its gate. A landing page that fronts a gated artifact probes
  exactly like an open one, so the probed URL must be the artifact.
- `status` = `unqueued | provisional(<why>) | queued | in-progress |
  done | blocked(<why>) | excluded(<why>)`. `provisional(<why>)` = row
  whose URL, year, or access is unresolved; it never promotes. `done` =
  worked to completion per root `TECHNICAL.md` § Operating. Ruled-out rows
  stay as `excluded(<why>)` — the audit trail of eligibility rulings.

  An org's sweep emits an `excluded(<why>)` row for every entry in that
  org's own guideline index it rules ineligible, and no row for material
  outside that index. The index is therefore reconstructible from the
  table, which is what makes a sweep auditable rather than merely
  asserted. An excluded row records a ruling and never promotes, so its
  `access` may stay `unverified` without a `provisional(...)` status.
- `notes` carries `id=<id>` once fetched (id rules: root `TECHNICAL.md`
  § Operating), foreign co-issuers, version qualifiers,
  `indexed-by=<org>[ + <org>...]` naming the indexes that surfaced the row
  when they are not its issuers, and `off-index(<why>)` when none did. The
  two provenance markers answer one question, so a row carries at most one.

## Enumeration sources

Legend for the enumeration frames; bibliographic sources name their
dataset and stable identifier in the cell itself.

- `AMA HOD roster` — AMA House of Delegates member organizations,
  <https://www.ama-assn.org/house-delegates/hod-organization/member-organizations-ama-house-delegates>
- `AMA SSS roster` — AMA Specialty and Service Society member list,
  <https://www.ama-assn.org/system/files/sss-member-organizations.pdf>
- `CMSS current members` / `CMSS associate members` —
  <https://cmss.org/membership/current-members/>
- `AOA specialty-college directory` —
  <https://osteopathic.org/?aoaad_affiliates_group=specialty>
- `NCRDSCB recognized specialties` —
  <https://ncrdscb.ada.org/recognized-dental-specialties>
- `ANA organizational affiliates` —
  <https://www.nursingworld.org/ana/org-affiliates/>
- `JCPP members` — <https://jcpp.net/about/>
- `APTA sections/academies` —
  <https://www.apta.org/apta-and-you/chapters-sections-academies>
- `GC-publishers` — Guideline Central library, embedded society index
  (481 records), <https://www.guidelinecentral.com/guidelines/>
- `ChoosingWisely` — Choosing Wisely sponsoring organizations, 2012–2023
  retired campaign (historical issuer confirmation only),
  <https://choosingwisely.org/>; archive
  <https://www.aafp.org/afp/collections/choosing-wisely>
- `AAFP-PG` — AAFP/AFP Practice Guidelines collection (676 items),
  <https://www.aafp.org/tag/collection/afp-departments/practice-guidelines>
- `WPSI-program` — Women's Preventive Services Initiative,
  <https://www.womenspreventivehealth.org/about-wpsi/>
- `National Academies/NCBI federal-guideline chapter` —
  <https://www.ncbi.nlm.nih.gov/books/NBK235754/>
- `major-issuer supplement` — major US CPG issuers reached outside the
  membership frames; a working label, not a directory, so it never counts
  toward the ≥2 independent sources.
- `official index` — the issuer's own guideline index; issuer-controlled,
  so it never counts either.

## Queue promotion

Exactly one guideline row sits in `queued|in-progress` at a time. When no
row holds either status, the first `unqueued` row in file order is
promoted: row status → `queued`, one queue entry appended per root
`TECHNICAL.md` § Operating; on fetch the row gains `id=<id>` and follows the
queue lifecycle (`in-progress` → `done`, or `blocked(<why>)` mirrored
from the queue). A row already `queued` after a halt is resumed rather
than joined by a second promotion.

File order = priority order, deliberately maintained: open access before
gated, `federal` before `society` before `other`, alphabetical by org
within a band, then version year descending and title within an org.

## Organizations

| org | abbrev | class | CPGs | index URL | enum sources | swept |
|---|---|---|---|---|---|---|
| Advisory Committee on Immunization Practices | ACIP | federal | yes | https://www.cdc.gov/acip-recs/hcp/vaccine-specific/ | PubMed PMID 41505372; Immunize.org ACIP index | 2026-08-07 per-topic-pages; 67 index entries -> 63 eligible + 3 excluded |
| Centers for Disease Control and Prevention | CDC | federal | yes | https://stacks.cdc.gov/cbrowse?parentId=cdc%3A100&pid=cdc%3A100 | GC-publishers; PubMed PMID 36327391; AAFP-PG | blocked(version-ambiguous collection; CDC Stacks 786 records over 40 pages mixes current with superseded) |
| Defense Health Agency Joint Trauma System | DHA JTS | federal | yes | https://jts.health.mil/index.cfm/CPGs/cpgs | PubMed PMID 34529799; DOI 10.55460/zfqw-dwgr | 2026-08-07 pdf-index; 109 index entries -> 83 eligible + 20 excluded |
| Federal Bureau of Prisons Health Services Division | BOP HSD | federal | yes | https://www.bop.gov/resources/health_care_mngmt.jsp | PubMed PMID 28089415; DOI 10.3201/eid3013.230799 | 2026-08-07 static-list; 47 index entries -> 39 eligible + 8 excluded |
| Health Resources and Services Administration | HRSA | federal | yes | https://www.hrsa.gov/womens-guidelines | GC-publishers; PubMed PMID 24112064 | 2026-08-07 static-list + reader-proxy; 14 index entries -> 14 eligible + 0 excluded |
| Healthcare Infection Control Practices Advisory Committee | HICPAC | federal | yes | https://www.cdc.gov/infection-control/hcp/guidance/index.html | PubMed PMID 28467526; GovInfo GOVPUB-HE20_7000-PURL-gpo136862 | 2026-08-07 static-list + reader-proxy; 49 index entries -> 39 eligible + 8 excluded |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | ClinicalInfo panels | federal | yes | https://clinicalinfo.hiv.gov/en/guidelines | GC-publishers; PubMed PMID 11365496 | 2026-08-07 static-list + reader-proxy; 6 index entries -> 6 eligible + 0 excluded |
| HHS Office of Population Affairs | OPA | federal | yes | https://opa.hhs.gov/reproductive-health/quality-family-planning | PubMed PMID 39570204; DOI 10.1016/j.amepre.2024.09.007 | blocked(browser-gated index (Cloudflare)) |
| Indian Health Service | IHS | federal | yes | https://www.ihs.gov/forproviders/clinicalresources/ | PubMed PMID 16125270; National Academies/NCBI federal-guideline chapter | 2026-08-07 per-topic-pages; 1 index entries -> 1 eligible + 0 excluded |
| National Asthma Education and Prevention Program | NAEPP | federal | yes | https://www.nhlbi.nih.gov/science/national-asthma-education-and-prevention-program-coordinating-committee-naeppcc | PMC7924476; DOI 10.1016/j.jaci.2020.10.003 | 2026-08-07 per-topic-pages; 1 index entries -> 1 eligible + 0 excluded |
| National Institute of Allergy and Infectious Diseases expert-panel guidelines | NIAID panels | federal | yes | https://www.niaid.nih.gov/diseases-conditions/food-allergy-guidelines | PubMed PMID 28065278; DOI 10.1016/j.jaci.2016.10.010 | blocked(browser-gated index (JS CAPTCHA)) |
| Substance Abuse and Mental Health Services Administration | SAMHSA | federal | yes | https://library.samhsa.gov/search-endpoint | NCBI Bookshelf NBK572951; DOI 10.1016/j.jsat.2012.01.008 | 2026-08-07 search-api(/search-endpoint); 74 index entries -> 40 eligible + 5 excluded |
| U.S. Preventive Services Task Force | USPSTF | federal | yes | https://www.uspreventiveservicestaskforce.org/uspstf/recommendation-topics | PubMed PMID 40553450; JAMA USPSTF recommendation collection; GC-publishers; AAFP-PG | blocked(anonymous enumeration exhausted: JSON API requires an API key; results view is JS-rendered with no Drupal ajaxViews settings; no sitemap; reader proxy returns the shell only) |
| U.S. Public Health Service Tobacco Use and Dependence Guideline Panel | PHS tobacco panel | federal | yes | https://www.ahrq.gov/prevention/guidelines/tobacco/index.html | NCBI Bookshelf NBK63952; DOI 10.1016/j.amepre.2008.04.009 | 2026-08-07 per-topic-pages; 1 index entries -> 1 eligible + 0 excluded |
| VA/DoD Clinical Practice Guideline Program | VA/DoD CPG | federal | yes | https://www.healthquality.va.gov/guidelines/ | PubMed PMID 39093266; National Academies/NCBI federal-guideline chapter; GC-publishers | 2026-08-07 per-topic-pages; 27 index entries -> 26 eligible + 0 excluded |
| National Institute for Occupational Safety and Health | NIOSH | federal | unverified | https://www.cdc.gov/niosh/pubs/ | - | blocked(guideline products not separately indexed in the 2,058-item publication catalog; issuer determination and enumeration source pair pending) |
| U.S. Army Surgeon General Health Policy and Services Proponency Office for Rehabilitation and Reintegration | - | federal | unverified | - | - | blocked(determination unsettled; no signal in any bulk source) |
| U.S. Public Health Service | USPHS | federal | unverified | - | - | blocked(determination deferred; AAFP-PG 4; PubMed 2005; issuer of 2 shipped row(s)) |
| United States Department of Health and Human Services | - | federal | unverified | - | GC-publishers | blocked(determination unsettled; GC 8; NGC historical; PubMed 2011) |
| Veterans Health Administration Pharmacy Benefits Management clinical guidance | VA PBM | federal | unverified | https://www.va.gov/formularyadvisor/doc/ | - | blocked(issues CPGs, <2 independent sources; heavy tail ~582 rows) |
| Advisory Committee on Heritable Disorders in Newborns and Children | ACHDNC | federal | no | https://www.hrsa.gov/advisory-committees/heritable-disorders/condition-nomination | excluded: committee terminated; recommendations archival | - |
| Agency for Healthcare Research and Quality | AHRQ | federal | no | https://www.ahrq.gov/gam/summaries/index.html | excluded: current role is evidence review and tools for external issuers | - |
| Agency for Toxic Substances and Disease Registry Medical Management Guidelines | ATSDR MMG | federal | no | https://wwwn.cdc.gov/TSP/MMG/MMGLanding.aspx | excluded: historical archive; no longer updated | - |
| Centers for Medicare & Medicaid Services | CMS | federal | no | https://www.cms.gov/Medicare/Quality-Initiatives-Patient-Assessment-Instruments/MMS/Downloads/Clinical-Guidelines.pdf | excluded: coverage and quality documents are administrative, not CPGs | - |
| Community Preventive Services Task Force | CPSTF | federal | no | https://www.thecommunityguide.org/pages/task-force-findings.html | excluded: community and public-health scope, not individual patient care | - |
| Dietary Guidelines for Americans program | DGA | federal | no | https://odphp.health.gov/our-work/nutrition-physical-activity/dietary-guidelines/current-dietary-guidelines | excluded: population nutrition policy, not patient-care CPG | - |
| Environmental Protection Agency pesticide-poisoning clinical manual program | EPA | federal | no | https://www.epa.gov/pesticide-worker-safety/recognition-and-management-pesticide-poisonings | excluded: no systematic CPG or EPA-endorsement claim | - |
| Food and Drug Administration | FDA | federal | no | https://www.fda.gov/regulatory-information/search-fda-guidance-documents | excluded: regulatory guidance is not patient-care CPG issuance | - |
| National Cancer Institute Physician Data Query | NCI PDQ | federal | no | https://www.cancer.gov/publications/pdq/information-summaries | excluded: PDQ summaries explicitly are not CPGs or treatment recommendations | - |
| National Heart, Lung, and Blood Institute legacy guideline programs | NHLBI legacy | federal | no | https://www.nhlbi.nih.gov/directors-messages/new-partnership-model-clinical-practice-guidelines | excluded: 2013 partnership model ended routine NHLBI CPG issuance | - |
| NIH COVID-19 Treatment Guidelines Panel | NIH COVID panel | federal | no | https://www.ncbi.nlm.nih.gov/books/NBK570371/ | excluded: discontinued; final update is historical and potentially outdated | - |
| Occupational Safety and Health Administration | OSHA | federal | no | https://www.osha.gov/clinicians | excluded: informational and regulatory resources; CPGs referred to ACOEM | - |
| Office of the Surgeon General | - | federal | no | - | excluded: Surgeon General reports and calls to action are population-health documents, not clinical practice guidelines | - |
| Academy of Nutrition and Dietetics | - | society | yes | https://www.andeal.org/topic-list | GC-publishers; PubMed PMID 38583584 | blocked(easy-tier harvest deferred) |
| American Academy of Allergy, Asthma & Immunology | AAAAI | society | yes | https://www.allergyparameters.org/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Academy of Clinical Toxicology | - | society | yes | https://www.clintox.org/resources/position-statements | ChoosingWisely; DOI 10.1007/s13181-025-01101-9 | blocked(easy-tier harvest deferred) |
| American Academy of Dental Sleep Medicine | - | society | yes | https://www.aadsm.org/guidelines.php | GC-publishers; DOI 10.15331/jdsm.4868 | blocked(easy-tier harvest deferred) |
| American Academy of Dermatology Association | AAD | society | yes | https://www.aad.org/member/clinical-quality/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Academy of Family Physicians | AAFP | society | yes | https://www.aafp.org/clinical-insights/preventive-care-and-whole-health/community-public-health/clinical-guidelines-and-recommendations-overview | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(missing enumeration surface; methodology overview, no artifact index) |
| American Academy of Neurology | AAN | society | yes | https://www.aan.com/practice/guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(heavy tail; 65 rows over 7 AJAX pages) |
| American Academy of Ophthalmology | AAO | society | yes | https://www.aao.org/guidelines-browse | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Academy of Oral & Maxillofacial Radiology | - | society | yes | https://aaomr.org/AAOMR/AAOMR/Resources/Position-Papers.aspx | NCRDSCB recognized specialties; GC-publishers | blocked(missing/stale enumeration surface; official index 403, proxy unresolved) |
| American Academy of Orthopaedic Surgeons | AAOS | society | yes | https://new.aaos.org/quality/quality-programs/clinical-practice-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Academy of Otolaryngology–Head and Neck Surgery Foundation | AAO-HNSF | society | yes | https://www.entnet.org/quality-practice/quality-products/clinical-practice-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Academy of Pediatric Dentistry | - | society | yes | https://www.aapd.org/research/oral-health-policies--recommendations/ | NCRDSCB recognized specialties; official index; GC-publishers | blocked(easy-tier harvest deferred) |
| American Academy of Pediatrics | AAP | society | yes | https://publications.aap.org/pediatrics/collection/523/Clinical-Practice-Guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(browser-gated index) |
| American Academy of Physical Medicine & Rehabilitation | AAPM&R | society | yes | https://www.aapmr.org/quality-practice/clinical-guidance | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Academy of Sleep Medicine | AASM | society | yes | https://aasm.org/clinical-resources/practice-standards/ | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely; AAFP-PG | 2026-08-08 static-list + per-topic-pages; 152 index entries -> 53 eligible + 32 excluded |
| American Association for Respiratory Care | AARC | society | yes | https://www.aarc.org/resources/clinical-resources/clinical-practice-guidelines/ + https://www.aarc.org/resource/aarc-issue-papers-position-statements-and-guidance-documents/ | GC-publishers; PubMed PMID 40323974 | 2026-08-08 static-list; 156 index entries -> 76 eligible + 72 excluded |
| American Association for the Study of Liver Diseases | AASLD | society | yes | https://www.aasld.org/practice-guidelines | CMSS current members; official index; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Association of Clinical Endocrinology | AACE | society | yes | https://pro.aace.com/clinical-guidance | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Association of Clinical Urologists | - | society | yes | https://aacuweb.org/position-statements/ | AMA HOD roster; AMA SSS roster; PubMed PMID 23829098 | blocked(easy-tier harvest deferred) |
| American Clinical Neurophysiology Society | - | society | yes | https://www.acns.org/practice/guidelines | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(easy-tier harvest deferred) |
| American College Health Association | - | society | yes | https://www.acha.org/resources-programs/resource-center/guidelines-recommendations-white-papers/ | GC-publishers; PubMed PMID 21823965 | blocked(breadth outruns tier; paginated(4 pages)) |
| American College of Allergy, Asthma & Immunology | - | society | yes | - | PubMed PMID 41936423; DOI 10.1016/j.anai.2025.10.026; GC-publishers; AAFP-PG | blocked(easy-tier harvest deferred) |
| American College of Cardiology | ACC | society | yes | https://www.acc.org/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | 2026-08-08 static-list; 161 index entries -> 101 eligible + 60 excluded |
| American College of Chest Physicians | CHEST | society | yes | https://www.chestnet.org/guidelines-and-topic-collections/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(browser-gated index (Cloudflare)) |
| American College of Emergency Physicians | ACEP | society | yes | https://www.acep.org/patient-care/clinical-policies | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American College of Foot and Ankle Surgeons | - | society | yes | https://www.acfas.org/research/clinical-consensus-documents | PubMed PMID 32131002; DOI 10.1053/j.jfas.2019.09.001 | blocked(missing/stale enumeration surface; index contents unprobed while D5 consumed fetch budget) |
| American College of Gastroenterology | - | society | yes | https://gi.org/guidelines/ | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | 2026-08-08 static-list; 68 index entries -> 67 eligible + 0 excluded |
| American College of Medical Genetics and Genomics | ACMG | society | yes | https://www.acmg.net/ACMG/Medical-Genetics-Practice-Resources/Practice-Guidelines.aspx | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American College of Nuclear Medicine | - | society | yes | - | PubMed PMID 40169271; DOI 10.2967/jnmt.125.269834; GC-publishers | blocked(easy-tier harvest deferred) |
| American College of Obstetricians and Gynecologists | ACOG | society | yes | https://www.acog.org/clinical/clinical-guidance/clinical-practice-guideline | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(breadth outruns easy tier; ~2,500 clinical records across 11 guidance types, recorded index sees 61) |
| American College of Occupational and Environmental Medicine | ACOEM | society | yes | https://acoem.org/Guidance-and-Position-Statements/Guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American College of Physicians | ACP | society | yes | https://www.acponline.org/clinical-information/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American College of Prosthodontists | - | society | yes | https://www.prosthodontics.org/about-acp/acp-position-statements/ | NCRDSCB recognized specialties; PubMed PMID 26711219 | blocked(easy-tier harvest deferred) |
| American College of Radiology | ACR-Radiology | society | yes | https://acsearch.acr.org/list | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(heavy tail; 279 rows) |
| American College of Rheumatology | ACR-Rheumatology | society | yes | https://rheumatology.org/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American College of Sports Medicine | - | society | yes | https://acsm.org/education-resources/pronouncements-scientific-communications/ | GC-publishers; AAFP-PG | blocked(easy-tier harvest deferred) |
| American College of Surgeons | ACS | society | yes | https://www.facs.org/quality-programs/trauma/quality/best-practices-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Dental Association | ADA-Dental | society | yes | https://www.ada.org/resources/research/science-and-research-institute/evidence-based-dental-research/clinical-practice-guidelines | major-issuer supplement; official index; GC-publishers; ChoosingWisely | blocked(stale enumeration surface (index 404)) |
| American Diabetes Association | ADA | society | yes | https://professional.diabetes.org/standards-of-care | major-issuer supplement; official index; GC-publishers; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Epilepsy Society | AES | society | yes | https://www.aesnet.org/clinical-care/clinical-guidance + https://www.aesnet.org/clinical-care/clinical-guidance/practice-parameters | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | 2026-08-08 static-list; 20 index entries -> 16 eligible + 2 excluded |
| American Gastroenterological Association | AGA | society | yes | https://gastro.org/clinical-guidance/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Geriatrics Society | AGS | society | yes | https://www.americangeriatrics.org/publications-tools/guidelines-recommendations | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Heart Association | AHA | society | yes | https://professional.heart.org/en/guidelines-and-statements | major-issuer supplement; official index; GC-publishers; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Occupational Therapy Association | AOTA | society | yes | https://www.aota.org/practice/practice-essentials/evidencebased-practiceknowledge-translation/practice-guidelines | major-issuer supplement; official index; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Pediatric Surgical Nurses Association | - | society | yes | https://www.apsna.org/position-statements/ | ANA organizational affiliates; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Physical Therapy Association | APTA | society | yes | https://www.apta.org/patient-care/evidence-based-practice-resources/cpgs | major-issuer supplement; official index; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Psychiatric Association | APA | society | yes | https://www.psychiatry.org/psychiatrists/practice/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Psychological Association | APA-Psychology | society | yes | https://www.apa.org/about/offices/directorates/guidelines/clinical-practice | GC-publishers; PubMed PMID 34843274 | blocked(easy-tier harvest deferred) |
| American Society for Clinical Pathology | ASCP | society | yes | https://www.ascp.org/news/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Society for Colposcopy and Cervical Pathology | - | society | yes | https://www.asccp.org/guidelines | major-issuer supplement; official index; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Society for Dermatologic Surgery Association | - | society | yes | https://www.asds.net/ASDSA-Advocacy/Positions-Model-Legislation | AMA HOD roster; PubMed PMID 22959232 | blocked(easy-tier harvest deferred) |
| American Society for Gastrointestinal Endoscopy | ASGE | society | yes | https://www.asge.org/home/resources/publications/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | blocked(easy-tier harvest deferred) |
| American Society for Radiation Oncology | ASTRO | society | yes | https://www.astro.org/provider-resources/guidelines/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Society for Reproductive Medicine | ASRM | society | yes | https://www.asrm.org/practice-guidance/practice-committee-documents/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Society for Transplantation and Cellular Therapy | - | society | yes | https://www.astctjournal.org/astct-publications | GC-publishers; PubMed PMID 37422194 | blocked(missing/stale enumeration surface; complete journal collection unprobed within D2 budget) |
| American Society of Addiction Medicine | ASAM | society | yes | https://www.asam.org/quality-care/clinical-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Society of Anesthesiologists | ASA | society | yes | https://www.asahq.org/standards-and-practice-parameters | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | 2026-08-08 static-list; 139 index entries -> 85 eligible + 54 excluded |
| American Society of Breast Surgeons | ASBrS | society | yes | https://www.breastsurgeons.org/resources/statements | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Society of Clinical Oncology/Association for Clinical Oncology | ASCO | society | yes | https://ascopubs.org/guidelines?doi=10.1200%2FJCO&publicationCode=jco | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(browser-gated index; heavy tail >=139 rows) |
| American Society of Colon & Rectal Surgeons | ASCRS | society | yes | https://fascrs.org/healthcare-providers/education/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Society of Echocardiography | ASE | society | yes | https://www.asecho.org/practice-clinical-resources/ase-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Society of Health-System Pharmacists | ASHP | society | yes | https://www.ashp.org/pharmacy-practice/policy-positions-and-guidelines/browse-by-document-type/guidelines | JCPP members; official index; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Society of Hematology | ASH | society | yes | https://www.hematology.org/education/clinicians/guidelines-and-quality-care/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| American Society of Interventional Pain Physicians | - | society | yes | https://asipp.org/guidelines/ | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(easy-tier harvest deferred) |
| American Society of Nephrology | ASN | society | yes | https://www.asn-online.org/policy/guidelines.aspx | PubMed PMID 42265997; DOI 10.1016/j.jacc.2026.03.056; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| American Society of Pediatric Hematology/Oncology | - | society | yes | https://aspho.org/knowledge-center/practice-and-quality | ChoosingWisely; DOI 10.1002/pbc.28967 | blocked(missing/stale enumeration surface; no comprehensive ASPHO CPG index) |
| American Society of Plastic Surgeons | ASPS | society | yes | https://www.plasticsurgery.org/for-medical-professionals/quality/evidence-based-clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | blocked(easy-tier harvest deferred) |
| American Thoracic Society | ATS | society | yes | https://site.thoracic.org/clinicians-researchers/clinical-practice-guidelines-statements-reports | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(partial enumeration surface; index exposes only part of the corpus) |
| American Urological Association | AUA | society | yes | https://www.auanet.org/guidelines-and-quality/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(partial enumeration surface; index exposes only part of the corpus) |
| APTA Neurology | - | society | yes | https://www.neuropt.org/practice-resources/anpt-clinical-practice-guidelines | PubMed PMID 34160028; DOI 10.1093/ptj/pzab153 | blocked(easy-tier harvest deferred) |
| APTA Orthopedics | - | society | yes | https://www.orthopt.org/content/practice/clinical-practice-guidelines + https://www.orthopt.org/content/publications/pub-cpg + https://www.orthopt.org/content/practice/clinical-practice-guidelines/other-related-guidelines | GC-publishers; PubMed PMID 33789434; Crossref 10.2519/jospt.2021.0302 | 2026-08-08 static-list + per-topic-pages; 29 index entries -> 24 eligible + 3 excluded |
| Association for Molecular Pathology | - | society | yes | - | PubMed PMID 28165299; DOI 10.1200/JCO.2016.71.9807 | blocked(easy-tier harvest deferred) |
| Association for the Advancement of Blood & Biotherapies | AABB | society | yes | https://www.aabb.org/news-resources/resources/clinical-practice-guidelines | GC-publishers; PubMed PMID 37824153 | blocked(easy-tier harvest deferred) |
| Association of periOperative Registered Nurses | - | society | yes | https://www.aorn.org/guidelines-resources/guidelines-for-perioperative-practice | ANA organizational affiliates; official index; GC-publishers | blocked(login corpus; eGuidelines subscription, 36 rows) |
| Clinical Immunology Society | CIS | society | yes | - | PubMed PMID 41936423; DOI 10.1016/j.anai.2025.10.026 | blocked(easy-tier harvest deferred) |
| College of American Pathologists | CAP | society | yes | https://www.cap.org/protocols-and-guidelines/cap-guidelines/current-cap-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | blocked(easy-tier harvest deferred) |
| Enhanced Recovery After Cardiac Surgery Society | - | society | yes | https://erascardiac.org/expert-recommendations/ | GC-publishers; PubMed PMID 31054241 | blocked(easy-tier harvest deferred) |
| Hematology/Oncology Pharmacy Association | - | society | yes | https://www.hoparx.org/resources/guidelines-standards-and-summaries/ | JCPP members; PubMed PMID 29400987 | blocked(easy-tier harvest deferred) |
| Infectious Diseases Society of America | IDSA | society | yes | https://www.idsociety.org/practice-guideline/all-practice-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(heavy tail; ~105 rows, 109 fetches) |
| Infusion Nurses Society | - | society | yes | https://www.ins1.org/publications/ | PubMed PMID 38975817; DOI 10.1097/NAN.0000000000000532 | blocked(login corpus; standards/full guidance member or purchase gated) |
| North American Society for Pediatric Gastroenterology, Hepatology and Nutrition | NASPGHAN | society | yes | https://naspghan.org/professional-resources/clinical-guidelines/ | CMSS current members; official index; GC-publishers | blocked(easy-tier harvest deferred) |
| North American Spine Society | NASS | society | yes | https://www.spine.org/Research-Clinical-Care/Quality-Improvement/Clinical-Guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| Obesity Medicine Association | OMA | society | yes | https://obesitymedicine.org/resources/obesity-algorithm/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | blocked(easy-tier harvest deferred) |
| Oncology Nursing Society | - | society | yes | https://www.ons.org/clinical-tools/guidelines | ANA organizational affiliates; official index; GC-publishers | blocked(easy-tier harvest deferred) |
| Pediatric Infectious Diseases Society | - | society | yes | https://pids.org/resources | major-issuer supplement; GC-publishers; ChoosingWisely; AAFP-PG | blocked(missing/stale enumeration surface; PIDS site omits current co-issued guidelines) |
| Post-Acute and Long-Term Care Medical Association | PALTmed | society | yes | https://paltmed.org/products/results?product_type=27 | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| Society for Healthcare Epidemiology of America | - | society | yes | - | PubMed PMID 23327981; DOI 10.2146/ajhp120568 | blocked(easy-tier harvest deferred) |
| Society for Maternal-Fetal Medicine | - | society | yes | https://publications.smfm.org/publications/ | AMA SSS roster; GC-publishers; ChoosingWisely; AAFP-PG | blocked(heavy tail; paginated index ≥10 pages) |
| Society for Vascular Surgery | SVS | society | yes | https://vascular.org/research-quality/guidelines-and-reporting-standards/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| Society of Breast Imaging | SBI | society | yes | - | GC-publishers; DOI 10.1001/jamasurg.2026.0613 | blocked(easy-tier harvest deferred) |
| Society of Critical Care Medicine | SCCM | society | yes | https://www.sccm.org/clinical-resources/guidelines/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| Society of Family Planning | - | society | yes | https://societyfp.org/clinical/clinical-guidance-library/ | GC-publishers; PubMed PMID 39710335 | blocked(easy-tier harvest deferred) |
| Society of General Internal Medicine | SGIM | society | yes | https://www.sgim.org/advocacy-policy/position-statements/ | DOI 10.1016/j.jacc.2025.05.007; PubMed PMID 40815242 | blocked(easy-tier harvest deferred) |
| Society of Gynecologic Oncology | SGO | society | yes | https://www.sgo.org/practice-management/statements-and-recommendations/ | CMSS current members; official index; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| Society of Hospital Medicine | SHM | society | yes | https://www.hospitalmedicine.org/clinical-topics/clinical-guidelines/ | PubMed PMID 22534627; GC-publishers | blocked(easy-tier harvest deferred) |
| Society of Interventional Radiology | SIR | society | yes | https://www.sirweb.org/in-practice/guidelines-and-statements/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | blocked(easy-tier harvest deferred) |
| Society of Nuclear Medicine and Molecular Imaging | SNMMI | society | yes | https://sites.snmmi.org/Web/Web/Clinical-Practice/Procedure-Standards/Default.aspx | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| Society of Pediatric Echocardiography | - | society | yes | - | PubMed PMID 33010859; DOI 10.1016/j.echo.2020.04.026 | blocked(missing/stale enumeration surface; co-issued guidance absent from own site) |
| Society of Surgical Oncology | SSO | society | yes | https://www.surgonc.org/resources/guidelines/ | CMSS current members; official index; GC-publishers; ChoosingWisely | blocked(easy-tier harvest deferred) |
| Surgical Infection Society | SIS | society | yes | - | PubMed PMID 23327981; DOI 10.2146/ajhp120568 | blocked(easy-tier harvest deferred) |
| The Society of Thoracic Surgeons | STS | society | yes | https://www.sts.org/resources/clinical-decision-making/clinical-practice-documents-and-policies | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | blocked(easy-tier harvest deferred) |
| Academic Pediatric Association | - | society | unverified | - | ChoosingWisely | blocked(determination deferred; no signal in any bulk source) |
| Academy of Breastfeeding Medicine | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 27; NGC historical; PubMed 2020) |
| Academy of Consultation-Liaison Psychiatry | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Academy of Managed Care Pharmacy | - | society | unverified | - | JCPP members | blocked(determination deferred; PubMed 2006) |
| Academy of Medical-Surgical Nurses | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Academy of Physicians in Clinical Research | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Aerospace Medical Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; PubMed 2004; AMA-roster-only) |
| American Academy of Addiction Psychiatry | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1) |
| American Academy of Ambulatory Care Nursing | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| American Academy of Audiology | - | society | unverified | - | major-issuer supplement | blocked(determination deferred; PubMed 2000; supplement-only) |
| American Academy of Child & Adolescent Psychiatry | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | blocked(determination deferred; GC 8; NGC historical; AAFP-PG 2) |
| American Academy of Cosmetic Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Academy of Emergency Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1; PubMed 2001) |
| American Academy of Facial Plastic and Reconstructive Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1) |
| American Academy of Insurance Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Academy of Nursing | - | society | unverified | - | ChoosingWisely | blocked(determination deferred; PubMed 2003) |
| American Academy of Oral & Maxillofacial Pathology | - | society | unverified | - | NCRDSCB recognized specialties | blocked(determination deferred; no signal in any bulk source) |
| American Academy of Oral Medicine | - | society | unverified | - | NCRDSCB recognized specialties | blocked(determination deferred; PubMed 2010) |
| American Academy of Orofacial Pain | - | society | unverified | - | NCRDSCB recognized specialties | blocked(determination deferred; PubMed 2011) |
| American Academy of Osteopathy | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Academy of Otolaryngic Allergy | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1) |
| American Academy of Otolaryngology–Head and Neck Surgery | AAO-HNS | society | unverified | - | - | blocked(determination deferred; AAFP-PG 10) |
| American Academy of Pain Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | blocked(determination deferred; GC 1; NGC historical; AAFP-PG 1; PubMed 2004) |
| American Academy of Periodontology | - | society | unverified | - | NCRDSCB recognized specialties | blocked(determination deferred; NGC historical; PubMed 2011) |
| American Academy of Physician Associates | - | society | unverified | - | - | blocked(determination deferred; no signal in any bulk source; issuer of 2 shipped row(s)) |
| American Academy of Psychiatry and the Law | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; PubMed 2014; AMA-roster-only) |
| American Academy of Thermology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| American Association for Geriatric Psychiatry | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; PubMed 2006; AMA-roster-only) |
| American Association for Hand Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Association for Pediatric Ophthalmology and Strabismus | - | society | unverified | - | ChoosingWisely | blocked(determination deferred; PubMed 2012) |
| American Association for the Surgery of Trauma | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 1; PubMed 2003) |
| American Association for Thoracic Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | blocked(determination deferred; GC 27; NGC historical; AAFP-PG 1; PubMed 2015; issuer of 6 shipped row(s)) |
| American Association of Bronchology and Interventional Pulmonology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 2) |
| American Association of Cardiovascular and Pulmonary Rehabilitation | - | society | unverified | - | - | blocked(determination deferred; NGC historical; AAFP-PG 1; PubMed 2014; issuer of 5 shipped row(s)) |
| American Association of Critical-Care Nurses | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; PubMed 2014) |
| American Association of Endocrine Surgeons | - | society | unverified | - | AMA SSS roster; GC-publishers | blocked(determination deferred; GC 3; NGC historical) |
| American Association of Endodontists | - | society | unverified | - | NCRDSCB recognized specialties; GC-publishers | blocked(determination deferred; GC 1; PubMed 2011) |
| American Association of Gynecologic Laparoscopists | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 4; PubMed 2013) |
| American Association of Hip and Knee Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 9) |
| American Association of Neurological Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 2; NGC historical; PubMed 2013; issuer of 1 shipped row(s)) |
| American Association of Neuromuscular & Electrodiagnostic Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely; AAFP-PG | blocked(determination deferred; GC 2; NGC historical; AAFP-PG 1; PubMed 2005) |
| American Association of Neuroscience Nurses | - | society | unverified | - | ANA organizational affiliates; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 1; NGC historical; PubMed 2014) |
| American Association of Nurse Anesthesiology | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| American Association of Nurse Practitioners | - | society | unverified | - | - | blocked(determination deferred; no signal in any bulk source; issuer of 2 shipped row(s)) |
| American Association of Oral and Maxillofacial Surgeons | - | society | unverified | - | NCRDSCB recognized specialties | blocked(determination deferred; NGC historical; PubMed 2014) |
| American Association of Orthodontists | - | society | unverified | - | NCRDSCB recognized specialties | blocked(determination deferred; no signal in any bulk source) |
| American Association of Physicists in Medicine | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1; PubMed 2011) |
| American Association of Psychiatric Pharmacists | - | society | unverified | - | JCPP members | blocked(determination deferred; no signal in any bulk source; issuer of 1 shipped row(s)) |
| American Association of Public Health Dentistry | - | society | unverified | - | NCRDSCB recognized specialties | blocked(determination deferred; no signal in any bulk source) |
| American Association of Public Health Physicians | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Burn Association | - | society | unverified | - | major-issuer supplement | blocked(determination deferred; AAFP-PG 1; PubMed 2003; supplement-only) |
| American Cannabis Nurses Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| American Chiropractic Association | - | society | unverified | - | ChoosingWisely | blocked(determination deferred; no signal in any bulk source) |
| American College of Asthma, Allergy, and Immunology | - | society | unverified | - | - | blocked(determination deferred; GC 21; NGC historical; AAFP-PG 1; issuer of 1 shipped row(s)) |
| American College of Clinical Pharmacy | ACCP | society | unverified | - | JCPP members; GC-publishers | blocked(determination deferred; GC 4; PubMed 2014; issuer of 4 shipped row(s)) |
| American College of Correctional Physicians | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American College of Critical Care Medicine | - | society | unverified | - | - | blocked(determination deferred; PubMed 2016; issuer of 3 shipped row(s)) |
| American College of Legal Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American College of Lifestyle Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 3; issuer of 1 shipped row(s)) |
| American College of Medical Quality | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American College of Medical Toxicology | - | society | unverified | - | AMA SSS roster; ChoosingWisely | blocked(determination deferred; PubMed 2017; issuer of 1 shipped row(s)) |
| American College of Mohs Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; PubMed 2012; AMA-roster-only) |
| American College of Nurse-Midwives | - | society | unverified | - | major-issuer supplement | blocked(determination deferred; PubMed 2020; supplement-only) |
| American College of Osteopathic Emergency Physicians | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American College of Osteopathic Family Physicians | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American College of Osteopathic Internists | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American College of Osteopathic Neurologists and Psychiatrists | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American College of Osteopathic Obstetricians and Gynecologists | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; PubMed 2012) |
| American College of Osteopathic Surgeons | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; PubMed 2004) |
| American College of Preventive Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 3; NGC historical; PubMed 2011; issuer of 2 shipped row(s)) |
| American Congress of Rehabilitation Medicine | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| American Contact Dermatitis Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Dermatological Association | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Foregut Society | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Glaucoma Society | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| American Headache Society | - | society | unverified | - | GC-publishers; ChoosingWisely; AAFP-PG | blocked(determination deferred; GC 4; NGC historical; AAFP-PG 3; PubMed 2018) |
| American Hernia Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Holistic Nurses Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; PubMed 2009) |
| American Institute of Ultrasound in Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 15; NGC historical; PubMed 2015) |
| American Limb Preservation Society | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| American Medical Association | - | society | unverified | - | - | blocked(determination deferred; AAFP-PG 2; PubMed 2002; issuer of 2 shipped row(s)) |
| American Medical Society for Sports Medicine | - | society | unverified | - | major-issuer supplement; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 5; PubMed 2015; issuer of 1 shipped row(s)) |
| American Medical Women's Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; PubMed 2009; AMA-roster-only) |
| American Nephrology Nurses' Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; PubMed 2006) |
| American Nursing Informatics Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| American Optometric Association | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 5; NGC historical) |
| American Orthopaedic Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Orthopaedic Foot & Ankle Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; ChoosingWisely | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic Academy of Addiction Medicine | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic Academy of Orthopedics | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic Academy of Sports Medicine | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; PubMed 2003) |
| American Osteopathic Association of Prolotherapy Regenerative Medicine | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic College of Allergy and Immunology | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic College of Anesthesiologists | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic College of Occupational and Preventive Medicine | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic College of Pathologists | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic College of Proctology | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic College of Radiology | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Osteopathic Colleges of Ophthalmology and Otolaryngology–Head and Neck Surgery | - | society | unverified | - | AOA specialty-college directory | blocked(determination deferred; no signal in any bulk source) |
| American Pharmacists Association | APhA | society | unverified | - | JCPP members | blocked(determination deferred; PubMed 2006; issuer of 1 shipped row(s)) |
| American Podiatric Medical Association | - | society | unverified | - | major-issuer supplement; GC-publishers; ChoosingWisely; AAFP-PG | blocked(determination deferred; GC 2; NGC historical; AAFP-PG 1; PubMed 2012; issuer of 1 shipped row(s)) |
| American Psychiatric Nurses Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; PubMed 2015) |
| American Rhinologic Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1) |
| American Roentgen Ray Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Shoulder and Elbow Surgeons | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Society for Aesthetic Plastic Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Society for Bone and Mineral Research | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 2; NGC historical) |
| American Society for Clinical Laboratory Science | - | society | unverified | - | ChoosingWisely | blocked(determination deferred; no signal in any bulk source) |
| American Society for Laser Medicine and Surgery, Inc. | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Society for Metabolic and Bariatric Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 18; NGC historical; PubMed 2024) |
| American Society for Microbiology | - | society | unverified | - | GC-publishers; ChoosingWisely | blocked(determination deferred; GC 1; NGC historical) |
| American Society for Nutrition | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1; PubMed 2010) |
| American Society for Parenteral and Enteral Nutrition | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 16; NGC historical; PubMed 2016; issuer of 1 shipped row(s)) |
| American Society for Preventive Cardiology | - | society | unverified | - | GC-publishers | blocked(determination deferred; PubMed 2007; issuer of 4 shipped row(s)) |
| American Society for Reconstructive Microsurgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Society for Surgery of the Hand | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Society of Cataract and Refractive Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Society of Clinical Psychopharmacology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 2) |
| American Society of Consultant Pharmacists | - | society | unverified | - | JCPP members; ChoosingWisely | blocked(determination deferred; PubMed 2006) |
| American Society of Cytopathology | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1; PubMed 2012) |
| American Society of Dentist Anesthesiologists | - | society | unverified | - | NCRDSCB recognized specialties | blocked(determination deferred; NGC historical) |
| American Society of Dermatopathology | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Society of Extracorporeal Technology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 6; PubMed 2015) |
| American Society of Ketamine Physicians, Psychotherapists, and Practitioners | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| American Society of Maxillofacial Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; PubMed 2002; AMA-roster-only) |
| American Society of Neuroimaging | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; PubMed 2010; AMA-roster-only) |
| American Society of Neuroradiology | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 2; NGC historical; PubMed 2018) |
| American Society of Nuclear Cardiology | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 5; PubMed 2013; issuer of 8 shipped row(s)) |
| American Society of Ophthalmic Plastic and Reconstructive Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| American Society of Pain and Neuroscience | - | society | unverified | - | AMA SSS roster; GC-publishers | blocked(determination deferred; GC 12) |
| American Society of Pediatric Nephrology | - | society | unverified | - | ChoosingWisely | blocked(determination deferred; no signal in any bulk source) |
| American Society of PeriAnesthesia Nurses | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; PubMed 2007) |
| American Society of Regional Anesthesia and Pain Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 22; NGC historical; PubMed 2012) |
| American Society of Retina Specialists | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; no signal in any bulk source) |
| American Society of Transplant Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1; PubMed 2007) |
| American Society of Transplantation | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 29; NGC historical; PubMed 2014) |
| American Society of Tropical Medicine and Hygiene | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 2) |
| American Speech-Language-Hearing Association | ASHA | society | unverified | - | major-issuer supplement | blocked(determination deferred; PubMed 2000; supplement-only) |
| American Thyroid Association | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 15; NGC historical; PubMed 2015) |
| American Urogynecologic Society | - | society | unverified | - | AMA SSS roster; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 8) |
| American Vein and Lymphatic Society | - | society | unverified | - | AMA SSS roster; GC-publishers | blocked(determination deferred; GC 6) |
| American Venous Forum | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 8; NGC historical; PubMed 2014) |
| AMSUS—The Society of Federal Health Professionals | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| APTA Academy of Education | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Acute Care | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Aquatics | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Cardiovascular and Pulmonary | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Clinical Electrophysiology and Wound Management | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Federal | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Geriatrics | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source; issuer of 1 shipped row(s)) |
| APTA Hand and Upper Extremity | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source; issuer of 3 shipped row(s)) |
| APTA Home Health | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Leadership and Innovation | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Oncology | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Pediatrics | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source; issuer of 2 shipped row(s)) |
| APTA Pelvic Health | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source; issuer of 1 shipped row(s)) |
| APTA Research | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source) |
| APTA Sports | - | society | unverified | - | APTA sections/academies | blocked(determination deferred; no signal in any bulk source; issuer of 6 shipped row(s)) |
| Association for Diagnostics & Laboratory Medicine | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 9) |
| Association for Nursing Professional Development | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Association for Professionals in Infection Control and Epidemiology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 4; NGC historical; PubMed 2011; issuer of 2 shipped row(s)) |
| Association of Academic Physiatrists | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Association of Academic Radiology | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Association of Black Cardiologists | - | society | unverified | - | - | blocked(determination deferred; PubMed 2007; issuer of 3 shipped row(s)) |
| Association of Nurses in AIDS Care | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Association of Pediatric Hematology/Oncology Nurses | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Association of Physicians of Pakistani Descent of North America | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Association of Polysomnography Technologists | - | society | unverified | - | - | blocked(determination deferred; no signal in any bulk source; issuer of 1 shipped row(s)) |
| Association of Professors of Dermatology | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Association of Rehabilitation Nurses | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; NGC historical) |
| Association of Women's Health, Obstetric & Neonatal Nurses | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; NGC historical) |
| Chi Eta Phi Sorority, Incorporated | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Child Neurology Society | - | society | unverified | - | GC-publishers; AAFP-PG | blocked(determination deferred; GC 2; NGC historical; AAFP-PG 1; PubMed 2012; issuer of 2 shipped row(s)) |
| Congress of Neurological Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 55; NGC historical; PubMed 2014) |
| Dermatology Nurses' Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Digital Medicine Society | - | society | unverified | - | CMSS associate members | blocked(determination deferred; no signal in any bulk source) |
| Eastern Association for the Surgery of Trauma | - | society | unverified | - | major-issuer supplement | blocked(determination deferred; NGC historical; AAFP-PG 1; PubMed 2012; supplement-only) |
| Emergency Nurses Association | - | society | unverified | - | ANA organizational affiliates; GC-publishers | blocked(determination deferred; GC 5; NGC historical; PubMed 2026) |
| Endocrine Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely; AAFP-PG | blocked(determination deferred; GC 29; NGC historical; AAFP-PG 4; PubMed 2021) |
| Endourological Society | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| GLMA: Health Professionals Advancing LGBTQ+ Equality | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Health Ministries Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Heart Failure Society of America | - | society | unverified | - | major-issuer supplement; GC-publishers; AAFP-PG | blocked(determination deferred; GC 7; NGC historical; AAFP-PG 1; PubMed 2015; issuer of 5 shipped row(s)) |
| Heart Rhythm Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | blocked(determination deferred; GC 34; NGC historical; AAFP-PG 3; PubMed 2015; issuer of 18 shipped row(s)) |
| Heart Valve Society | - | society | unverified | - | - | blocked(determination unsettled; scout finds no own program yet 1 eligible shipped row names it co-issuer) |
| HIV Medicine Association | - | society | unverified | - | major-issuer supplement; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 3; NGC historical) |
| Hospice and Palliative Nurses Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; PubMed 2004) |
| International Antiviral Society–USA | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 3; AAFP-PG 2) |
| International Nurses Society on Addictions | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| International Pain and Spine Intervention Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1) |
| International Society for Adult Congenital Heart Disease | - | society | unverified | - | - | blocked(determination deferred; no signal in any bulk source; issuer of 2 shipped row(s)) |
| International Society for Heart and Lung Transplantation | - | society | unverified | - | - | blocked(determination deferred; GC 14; NGC historical; PubMed 2013) |
| International Society for the Advancement of Spine Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| International Society of Hair Restoration Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Korean American Medical Association | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Musculoskeletal Tumor Society | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 2) |
| National Association of Clinical Nurse Specialists | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; PubMed 2007) |
| National Association of EMS Physicians | - | society | unverified | - | major-issuer supplement | blocked(determination deferred; PubMed 2014; supplement-only; issuer of 1 shipped row(s)) |
| National Association of Epilepsy Centers | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| National Association of Indian Nurses of America | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| National Association of Medical Examiners | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; PubMed 2004; AMA-roster-only) |
| National Association of Neonatal Nurses | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; PubMed 2012) |
| National Association of Nurse Practitioners in Women's Health | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| National Association of School Nurses | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; PubMed 2024) |
| National Community Pharmacists Association | - | society | unverified | - | JCPP members | blocked(determination deferred; no signal in any bulk source) |
| National Lipid Association | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 18; PubMed 2014; issuer of 2 shipped row(s)) |
| National Medical Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only; issuer of 1 shipped row(s)) |
| National Pharmaceutical Association | - | society | unverified | - | JCPP members | blocked(determination deferred; no signal in any bulk source) |
| National Society of Genetic Counselors | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 13; NGC historical; issuer of 1 shipped row(s)) |
| Neurocritical Care Society | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 17; NGC historical; PubMed 2015) |
| North American Neuro-Ophthalmology Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; PubMed 2019; AMA-roster-only) |
| North American Neuroendocrine Tumor Society | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1; PubMed 2013) |
| North American Neuromodulation Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| North American Society for Cardiovascular Imaging | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1; NGC historical; PubMed 2010; issuer of 3 shipped row(s)) |
| North American Society for Interventional Thyroidology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| Organization for Associate Degree Nursing | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Orthodox Jewish Nurses Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Orthopaedic Trauma Association | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; AAFP-PG 1; AMA-roster-only) |
| Outpatient Endovascular and Interventional Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Pediatric and Congenital Electrophysiology Society | - | society | unverified | - | - | blocked(determination deferred; PubMed 2012; issuer of 1 shipped row(s)) |
| Pediatric Endocrine Society | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 2; PubMed 2015) |
| Pediatric Orthopaedic Society of North America | - | society | unverified | - | GC-publishers; ChoosingWisely | blocked(determination deferred; no signal in any bulk source) |
| Preventive Cardiovascular Nurses Association | - | society | unverified | - | ANA organizational affiliates | blocked(determination unsettled; AAFP-PG 1; PubMed 2014; issuer of 3 shipped row(s)) |
| Radiological Society of North America | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1; issuer of 1 shipped row(s)) |
| Renal Physicians Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; NGC historical; PubMed 2003; AMA-roster-only) |
| Sigma | - | society | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| Skin of Color Society | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Sleep Research Society | - | society | unverified | - | - | blocked(determination deferred; no signal in any bulk source; issuer of 1 shipped row(s)) |
| Society for Academic Emergency Medicine | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 2; PubMed 2015; issuer of 2 shipped row(s)) |
| Society for Adolescent Health and Medicine | - | society | unverified | - | major-issuer supplement | blocked(determination deferred; PubMed 2025; supplement-only) |
| Society for Atherosclerosis Imaging and Prevention | - | society | unverified | - | - | blocked(determination deferred; no signal in any bulk source) |
| Society for Cardiovascular Angiography & Interventions | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 50; NGC historical; issuer of 19 shipped row(s)) |
| Society for Cardiovascular Magnetic Resonance | - | society | unverified | - | AMA HOD roster; AMA SSS roster; ChoosingWisely | blocked(determination deferred; PubMed 2014; issuer of 12 shipped row(s)) |
| Society for Developmental and Behavioral Pediatrics | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| Society for Immunotherapy of Cancer | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 21) |
| Society for Integrative Oncology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 5; PubMed 2014) |
| Society for Investigative Dermatology | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Society for Obstetric Anesthesia and Perinatology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 3; NGC historical) |
| Society for Pediatric Anesthesia | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| Society for Pediatric Dermatology | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Society for the Advancement of Patient Blood Management | - | society | unverified | - | GC-publishers; ChoosingWisely | blocked(determination deferred; GC 1) |
| Society for Vascular Medicine | - | society | unverified | - | GC-publishers; ChoosingWisely; AAFP-PG | blocked(determination deferred; GC 4; NGC historical; AAFP-PG 1; PubMed 2015; issuer of 4 shipped row(s)) |
| Society for Vascular Nursing | - | society | unverified | - | - | blocked(determination deferred; PubMed 2010; issuer of 2 shipped row(s)) |
| Society of Abdominal Radiology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| Society of American Gastrointestinal and Endoscopic Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 29; NGC historical; PubMed 2024) |
| Society of Cardiovascular Anesthesiologists | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 5; NGC historical; PubMed 2015; issuer of 2 shipped row(s)) |
| Society of Cardiovascular Computed Tomography | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | blocked(determination deferred; GC 12; PubMed 2015; issuer of 15 shipped row(s)) |
| Society of Cardiovascular Patient Care | - | society | unverified | - | - | blocked(determination unsettled; merged into ACC 2016 yet 1 eligible shipped row names it co-issuer; PubMed 2013) |
| Society of Critical Care Anesthesiologists | SOCCA | society | unverified | https://www.socca.org/recommendations | DOI 10.1093/cid/cir257; PubMed PMID 21460264 | blocked(determination deferred; no signal in any bulk source; issuer of 1 shipped row(s)) |
| Society of Dermatology Physician Associates | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| Society of Infectious Diseases Pharmacists | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 2) |
| Society of Laparoscopic & Robotic Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Society of NeuroInterventional Surgery | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 3; NGC historical; PubMed 2018) |
| Society of Pediatric Cardiology Training Program Directors | - | society | unverified | - | - | blocked(determination deferred; no signal in any bulk source) |
| Society of Pediatric Nurses | - | society | unverified | - | ChoosingWisely | blocked(determination deferred; PubMed 2007) |
| Society of Radiologists in Ultrasound | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 1; NGC historical; PubMed 2015) |
| Society of Thoracic Radiology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 2; PubMed 2014; issuer of 1 shipped row(s)) |
| Society of Urodynamics Female Pelvic Medicine & Urogenital Reconstruction | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 10; NGC historical) |
| Society of Urologic Oncology | - | society | unverified | - | GC-publishers | blocked(determination deferred; GC 9; NGC historical) |
| The Menopause Society | - | society | unverified | - | major-issuer supplement; GC-publishers; AAFP-PG | blocked(determination deferred; AAFP-PG 3) |
| The Obesity Society | - | society | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 4; NGC historical; PubMed 2013) |
| The Triological Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Undersea and Hyperbaric Medical Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | blocked(determination deferred; GC 1; NGC historical) |
| Urgent Care College of Physicians | - | society | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Vascular and Endovascular Surgery Society | - | society | unverified | - | - | blocked(determination deferred; no signal in any bulk source; issuer of 1 shipped row(s)) |
| Western Trauma Association | - | society | unverified | - | major-issuer supplement | blocked(determination deferred; no signal in any bulk source; supplement-only) |
| Wilderness Medical Society | - | society | unverified | - | major-issuer supplement; GC-publishers; AAFP-PG | blocked(determination deferred; GC 11; AAFP-PG 5; PubMed 2014) |
| Wound Healing Society | - | society | unverified | - | major-issuer supplement | blocked(determination deferred; PubMed 2014; supplement-only) |
| Wound, Ostomy, and Continence Nurses Society | - | society | unverified | - | ANA organizational affiliates; GC-publishers | blocked(determination deferred; GC 1; NGC historical) |
| American Academy of Hospice and Palliative Medicine | AAHPM | society | no | https://aahpm.org/publications/ | excluded: no AAHPM-issued CPG program identified | - |
| American Association of Physicians of Indian Origin | - | society | no | - | excluded: membership, advocacy, CME, and journal body; no clinical-practice guideline program | - |
| American Association of Plastic Surgeons | - | society | no | - | excluded: academic membership society; meetings, research, awards, newsletters only | - |
| American College of Osteopathic Pediatricians | - | society | no | - | excluded: education, journal, and advocacy body that points clinicians to external guideline developers | - |
| American College of Radiation Oncology | - | society | no | - | excluded: accreditation, professional education, and advocacy body; no clinical-practice guideline program | - |
| American Medical Informatics Association | AMIA | society | no | https://amia.org/public-policy/current-policy-priorities | excluded: policy and implementation guidance, not disease-specific patient-care CPGs | - |
| American Nurses Association | ANA | society | no | https://www.nursingworld.org/practice-policy/nursing-excellence/official-position-statements/ | excluded: position statements and standards, not disease-specific patient-care CPGs | - |
| American Osteopathic Association | - | society | no | https://osteopathic.org/?s=clinical+practice+guideline | excluded: no current AOA-issued patient-care CPG or index found | - |
| American Osteopathic College of Dermatology | - | society | no | - | excluded: disease database is patient education; no AOCD-issued CPG program identified | 2026-08-08 static-list; 0 index entries -> 0 eligible + 0 excluded |
| American Osteopathic College of Physical Medicine and Rehabilitation | - | society | no | - | excluded: education/membership society; no clinical-guideline program | - |
| APTA Private Practice | - | society | no | - | excluded: business/trade section; no patient-care recommendation program | - |
| International Academy of Independent Medical Evaluators | - | society | no | - | excluded: certification/training and medicolegal impairment-evaluation body; no patient-care guideline program | - |
| International College of Surgeons—US Section | - | society | no | - | excluded: membership/CME society; no clinical-guideline program | - |
| Philippine Nurses Association of America | - | society | no | - | excluded: workforce, legislative, and professional advocacy statements; no systematically developed patient-care guidance | - |
| The American Association of Nurse Attorneys | - | society | no | - | excluded: health-law association; position papers are legal/workforce policy | - |
| Alzheimer's Association | - | other | yes | https://www.alz.org/professionals/health-systems-medical-professionals/clinical-practice-guidelines | GC-publishers; PubMed PMID 40729527 | blocked(easy-tier harvest deferred) |
| American Cancer Society | ACS-Cancer | other | yes | https://www.cancer.org/health-care-professionals/american-cancer-society-prevention-early-detection-guidelines.html | major-issuer supplement; official index; GC-publishers; AAFP-PG | blocked(easy-tier harvest deferred) |
| Brain Trauma Foundation | - | other | yes | https://braintrauma.org/guidelines | GC-publishers; PubMed PMID 23971957 | blocked(partial enumeration surface; index exposes only part of the corpus) |
| Clinical Pharmacogenetics Implementation Consortium | - | other | yes | https://www.clinpgx.org/cpic/guidelines | PubMed PMID 41979467; DOI 10.1002/cpt.70291 | blocked(browser-gated index (JS-only shell)) |
| Consortium of Multiple Sclerosis Centers | - | other | yes | https://www.mscare.org/recommendations-for-care/ | GC-publishers; DOI 10.3174/ajnr.A4539 | blocked(missing/stale enumeration surface; sibling MRI/best-practice pages not unified) |
| Cystic Fibrosis Foundation | - | other | yes | https://www.cff.org/medical-professionals/clinical-care-guidelines | major-issuer supplement; GC-publishers; PubMed PMID 32115388; Crossref 10.1542/peds.2015-1784 | 2026-08-08 reader-proxy; 39 index entries -> 37 eligible + 2 excluded |
| National Coalition for Hospice and Palliative Care | - | other | yes | https://www.nationalcoalitionhpc.org/ncp/ | GC-publishers; PubMed PMID 30915906 | blocked(easy-tier harvest deferred) |
| National Comprehensive Cancer Network | NCCN | other | yes | https://www.nccn.org/guidelines/category_1 | PubMed PMID 39019058; DOI 10.6004/jnccn.2024.0035 | blocked(login corpus; free account, >=91 rows) |
| National Kidney Foundation | NKF | other | yes | https://www.ajkd.org/content/kdoqiguidelines | GC-publishers; PubMed PMID 32778223 | blocked(browser-gated index) |
| National Pressure Injury Advisory Panel | - | other | yes | https://npiap.com/page/InternationalGuidelines + https://npiap.com/page/PositionStatements | major-issuer supplement; GC-publishers; PubMed PMID 37929973; Crossref 10.1097/ASW.0000000000000079 | 2026-08-08 static-list; 10 index entries -> 3 eligible + 2 excluded |
| North American Imaging in MS Cooperative | - | other | yes | https://www.naimscooperative.org/news-and-publications | GC-publishers; DOI 10.1016/S1474-4422(25)00304-7 | blocked(easy-tier harvest deferred) |
| Accreditation Council for Pharmacy Education | - | other | unverified | - | JCPP members | blocked(determination deferred; no signal in any bulk source) |
| American Association of Colleges of Nursing | - | other | unverified | - | ANA organizational affiliates | blocked(determination deferred; no signal in any bulk source) |
| American Association of Colleges of Pharmacy | - | other | unverified | - | JCPP members | blocked(determination deferred; no signal in any bulk source) |
| American Hospital Association | - | other | unverified | - | CMSS associate members | blocked(determination deferred; no signal in any bulk source) |
| American Medical Group Association | - | other | unverified | - | AMA HOD roster; AMA SSS roster; CMSS associate members | blocked(determination deferred; no signal in any bulk source) |
| American Red Cross | - | other | unverified | - | major-issuer supplement | blocked(determination deferred; AAFP-PG 1; supplement-only) |
| AO North America | - | other | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| Bone Health and Osteoporosis Foundation | - | other | unverified | - | major-issuer supplement | blocked(determination deferred; no signal in any bulk source; supplement-only) |
| Critical Care Societies Collaborative | - | other | unverified | - | ChoosingWisely | blocked(determination deferred; no signal in any bulk source) |
| Hidradenitis Suppurativa Foundation | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 2; PubMed 2012) |
| Institute for Addressing Strangulation | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| Institute for Clinical Systems Improvement | - | other | unverified | - | - | blocked(determination deferred; NGC historical; AAFP-PG 3; issuer of 2 shipped row(s)) |
| International Essential Tremor Foundation | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| Michigan Quality Improvement Consortium | - | other | unverified | - | major-issuer supplement | blocked(determination deferred; NGC historical; supplement-only) |
| Muscular Dystrophy Association | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| National Alliance of State Pharmacy Associations | - | other | unverified | - | JCPP members | blocked(determination deferred; no signal in any bulk source) |
| National Association of Boards of Pharmacy | - | other | unverified | - | JCPP members | blocked(determination unsettled; no signal in any bulk source) |
| National Bleeding Disorders Foundation | - | other | unverified | - | major-issuer supplement; GC-publishers | blocked(determination deferred; GC 27) |
| National Commission on Correctional Health Care | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| National Council for Mental Wellbeing | - | other | unverified | - | GC-publishers | blocked(determination deferred; no signal in any bulk source) |
| National Marrow Donor Program | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1; PubMed 2012) |
| National Multiple Sclerosis Society | - | other | unverified | - | major-issuer supplement | blocked(determination deferred; no signal in any bulk source; supplement-only) |
| National Network of Depression Centers | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 2) |
| National Psoriasis Foundation | - | other | unverified | - | GC-publishers; AAFP-PG | blocked(determination deferred; GC 6; AAFP-PG 1; PubMed 2015) |
| National Rosacea Society | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| National Tuberculosis Coalition of America | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| Obesity Action Coalition | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| Pediatric Acute Lung Injury and Sepsis Investigators | - | other | unverified | - | - | blocked(determination deferred; no signal in any bulk source; issuer of 1 shipped row(s)) |
| Respiratory Therapy Access Working Group | - | other | unverified | - | - | blocked(determination deferred; no signal in any bulk source) |
| Restless Legs Syndrome Foundation | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1) |
| ROME Foundation | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1; PubMed 2006) |
| Spondylitis Association of America | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1; NGC historical) |
| The OrthoForum | - | other | unverified | - | AMA SSS roster | blocked(determination deferred; no signal in any bulk source; AMA-roster-only) |
| United States Cutaneous Lymphoma Consortium | - | other | unverified | - | GC-publishers | blocked(determination deferred; GC 1; PubMed 2011) |
| University HealthSystem Consortium Respiratory Care Network | - | other | unverified | - | - | blocked(determination deferred; no signal in any bulk source; issuer of 1 shipped row(s)) |
| Accreditation Council for Continuing Medical Education | - | other | no | - | excluded: training and accreditation body; no patient-care CPG program | - |
| Accreditation Council for Graduate Medical Education | - | other | no | - | excluded: training and accreditation body; no patient-care CPG program | - |
| American Board of Medical Specialties | - | other | no | - | excluded: certification body; no patient-care CPG program | - |
| Association for Hospital Medical Education | - | other | no | - | excluded: training and workforce body; no patient-care CPG program | - |
| Association of American Medical Colleges | - | other | no | - | excluded: training and workforce body; no patient-care CPG program | - |
| Coalition for Health AI | - | other | no | - | excluded: AI governance/implementation consortium; no clinical-care recommendations | - |
| Federation of State Medical Boards | - | other | no | - | excluded: certification and workforce body; no patient-care CPG program | - |
| Intealth – Advancing the Global Health Workforce | - | other | no | - | excluded: training and workforce body; no patient-care CPG program | - |
| Kidney Disease: Improving Global Outcomes | KDIGO | other | no | https://kdigo.org/guidelines/ | excluded: non-American; Brussels-based independent foundation since 2013, founded 2003 by NKF, no current US lead sponsor | - |
| National Board of Medical Examiners | - | other | no | - | excluded: examination body; no patient-care CPG program | - |
| National Health Council | - | other | no | - | excluded: patient-policy coalition producing advocacy, research, value-assessment, and engagement tools; no clinical-guideline program | - |

## Guidelines

Guideline rows live in `.agent/compendium.tsv` — tab-separated, one row per
guideline, columns `org`, `title (year)`, `URL`, `access`, `status`, `notes`
exactly as `## Row formats` defines them, in the file order `## Queue promotion`
fixes. They moved out of this file at M1.2b: the table passes a thousand
rows there, and every agent reading these rules would otherwise read the whole
corpus with them.
