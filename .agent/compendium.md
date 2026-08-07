# compendium

Master list of eligible American clinical guidelines available online:
the issuing-organization universe plus, per swept organization, one row
per guideline. Discovery ledger only — a guideline is fetched exactly
when its row is promoted into `.agent/queue.md` (procedure: root
`README.md` § Operating).

Construction is staged (roadmap M1): the organization universe and one
seed guideline per confirmed issuer land first, per-org index sweeps fill
the guideline rows, and an aggregator cross-check closes the gap list.
Until every organization row reaches a terminal sweep state, the
guideline list is a seed roster, not the complete list.

Coverage is also tiered. M1 harvests organizations whose index enumerates
in ≤2 anonymous fetches and whose artifacts classify without an
authenticated session; the rest carry `swept = blocked(<why>)` and are
enumerated for a later harvest in roadmap M2. So a compendium satisfying
the terminal condition below is exhaustive over its swept organizations
and explicit about the organizations it defers — not a claim that every
American clinical guideline holds a row.

Terminal condition — compendium exhaustion, the standing goal's finite
stop check, reached when both hold:

- every organization row is `CPGs=no`, or carries `swept` =
  `<date> <method>` or `blocked(<why>)`;
- every guideline row is `done`, `blocked(<why>)`, or `excluded(<why>)`.

Root `README.md` § Operating adopts this as the goal's terminal clause at
roadmap M1.5; until that commit lands, the README's own completion clause
governs the running goal.

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
  row belongs to its issuing org(s). Co-issued guideline = one row; the
  org cell joins every issuing org with `+` and the row satisfies each
  member's seed obligation. Each American co-issuer holds its own
  organization row (co-issuance is issuance, so its `CPGs` reads `yes`);
  foreign co-issuers stay out of the organization roster and out of the
  org cell, named in `notes` instead.
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
  table it describes. `<n>` records what the index offered; `n > e + x`
  means entries collapsed as duplicates, aliases, or superseded editions,
  and the sweep report carries that reconciliation. `<method>` names the
  enumeration route: `static-list`, `pdf-index`, `per-topic-pages`,
  `paginated(<n> pages)`, `search-api(<endpoint>)`, `reader-proxy`, joined
  with `+` when a route needs more than one.

Guidelines — `| org | title (year) | URL | access | status | notes |`

- `org` = every American issuing org, joined with `+` for co-issued work.
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
  worked to completion per root `README.md` § Operating. Ruled-out rows
  stay as `excluded(<why>)` — the audit trail of eligibility rulings.

  An org's sweep emits an `excluded(<why>)` row for every entry in that
  org's own guideline index it rules ineligible, and no row for material
  outside that index. The index is therefore reconstructible from the
  table, which is what makes a sweep auditable rather than merely
  asserted. An excluded row records a ruling and never promotes, so its
  `access` may stay `unverified` without a `provisional(...)` status.
- `notes` carries `id=<id>` once fetched (id rules: root `README.md`
  § Operating), foreign co-issuers, and version qualifiers.

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
`README.md` § Operating; on fetch the row gains `id=<id>` and follows the
queue lifecycle (`in-progress` → `done`, or `blocked(<why>)` mirrored
from the queue). A row already `queued` after a halt is resumed rather
than joined by a second promotion.

File order = priority order, deliberately maintained: open access before
gated, `federal` before `society` before `other`, alphabetical by org
within a band, then version year descending and title within an org.

## Organizations

| org | abbrev | class | CPGs | index URL | enum sources | swept |
|---|---|---|---|---|---|---|
| Advisory Committee on Immunization Practices | ACIP | federal | yes | https://www.cdc.gov/acip-recs/hcp/vaccine-specific/ | PubMed PMID 41505372; Immunize.org ACIP index | 2026-08-07 static-list + reader-proxy; 27 index entries -> 27 eligible + 0 excluded |
| Centers for Disease Control and Prevention | CDC | federal | yes | https://stacks.cdc.gov/cbrowse?parentId=cdc%3A100&pid=cdc%3A100 | GC-publishers; PubMed PMID 36327391; AAFP-PG | pending |
| Defense Health Agency Joint Trauma System | DHA JTS | federal | yes | https://jts.health.mil/index.cfm/CPGs/cpgs | PubMed PMID 34529799; DOI 10.55460/zfqw-dwgr | 2026-08-07 pdf-index; 109 index entries -> 83 eligible + 20 excluded |
| Federal Bureau of Prisons Health Services Division | BOP HSD | federal | yes | https://www.bop.gov/resources/health_care_mngmt.jsp | PubMed PMID 28089415; DOI 10.3201/eid3013.230799 | 2026-08-07 static-list; 47 index entries -> 39 eligible + 8 excluded |
| Health Resources and Services Administration | HRSA | federal | yes | https://www.hrsa.gov/womens-guidelines | GC-publishers; PubMed PMID 24112064 | 2026-08-07 static-list + reader-proxy; 14 index entries -> 14 eligible + 0 excluded |
| Healthcare Infection Control Practices Advisory Committee | HICPAC | federal | yes | https://www.cdc.gov/infection-control/hcp/guidance/index.html | PubMed PMID 28467526; GovInfo GOVPUB-HE20_7000-PURL-gpo136862 | 2026-08-07 static-list + reader-proxy; 49 index entries -> 39 eligible + 8 excluded |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | ClinicalInfo panels | federal | yes | https://clinicalinfo.hiv.gov/en/guidelines | GC-publishers; PubMed PMID 11365496 | 2026-08-07 static-list + reader-proxy; 6 index entries -> 6 eligible + 0 excluded |
| HHS Office of Population Affairs | OPA | federal | yes | https://opa.hhs.gov/reproductive-health/quality-family-planning | PubMed PMID 39570204; DOI 10.1016/j.amepre.2024.09.007 | pending |
| Indian Health Service | IHS | federal | yes | https://www.ihs.gov/forproviders/clinicalresources/ | PubMed PMID 16125270; National Academies/NCBI federal-guideline chapter | 2026-08-07 per-topic-pages; 2 index entries -> 1 eligible + 0 excluded |
| National Asthma Education and Prevention Program | NAEPP | federal | yes | https://www.nhlbi.nih.gov/science/national-asthma-education-and-prevention-program-coordinating-committee-naeppcc | PMC7924476; DOI 10.1016/j.jaci.2020.10.003 | 2026-08-07 per-topic-pages; 1 index entries -> 1 eligible + 0 excluded |
| National Institute of Allergy and Infectious Diseases expert-panel guidelines | NIAID panels | federal | yes | https://www.niaid.nih.gov/diseases-conditions/food-allergy-guidelines | PubMed PMID 28065278; DOI 10.1016/j.jaci.2016.10.010 | pending |
| Substance Abuse and Mental Health Services Administration | SAMHSA | federal | yes | https://library.samhsa.gov/search-endpoint | NCBI Bookshelf NBK572951; DOI 10.1016/j.jsat.2012.01.008 | 2026-08-07 search-api(/search-endpoint); 74 index entries -> 40 eligible + 5 excluded |
| U.S. Preventive Services Task Force | USPSTF | federal | yes | https://www.uspreventiveservicestaskforce.org/uspstf/recommendation-topics | PubMed PMID 40553450; JAMA USPSTF recommendation collection; GC-publishers; AAFP-PG | blocked(anonymous enumeration exhausted: JSON API requires an API key; results view is JS-rendered with no Drupal ajaxViews settings; no sitemap; reader proxy returns the shell only) |
| U.S. Public Health Service Tobacco Use and Dependence Guideline Panel | PHS tobacco panel | federal | yes | https://www.ahrq.gov/prevention/guidelines/tobacco/index.html | NCBI Bookshelf NBK63952; DOI 10.1016/j.amepre.2008.04.009 | 2026-08-07 per-topic-pages; 1 index entries -> 1 eligible + 0 excluded |
| VA/DoD Clinical Practice Guideline Program | VA/DoD CPG | federal | yes | https://www.healthquality.va.gov/guidelines/ | PubMed PMID 39093266; National Academies/NCBI federal-guideline chapter; GC-publishers | 2026-08-07 per-topic-pages; 27 index entries -> 26 eligible + 0 excluded |
| National Institute for Occupational Safety and Health | NIOSH | federal | unverified | https://www.cdc.gov/niosh/pubs/ | - | blocked(guideline products not separately indexed in the 2,058-item publication catalog; issuer determination and enumeration source pair pending) |
| United States Department of Health and Human Services | - | federal | unverified | - | GC-publishers | pending |
| Veterans Health Administration Pharmacy Benefits Management clinical guidance | VA PBM | federal | unverified | https://www.va.gov/formularyadvisor/doc/ | - | pending |
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
| Academy of Nutrition and Dietetics | - | society | yes | https://www.andeal.org/topic-list | GC-publishers; PubMed PMID 38583584 | pending |
| American Academy of Allergy, Asthma & Immunology | AAAAI | society | yes | https://www.allergyparameters.org/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American Academy of Dermatology Association | AAD | society | yes | https://www.aad.org/member/clinical-quality/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Academy of Family Physicians | AAFP | society | yes | https://www.aafp.org/clinical-insights/preventive-care-and-whole-health/community-public-health/clinical-guidelines-and-recommendations-overview | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Academy of Neurology | AAN | society | yes | https://www.aan.com/practice/guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Academy of Ophthalmology | AAO | society | yes | https://www.aao.org/guidelines-browse | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American Academy of Orthopaedic Surgeons | AAOS | society | yes | https://new.aaos.org/quality/quality-programs/clinical-practice-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Academy of Otolaryngology—Head and Neck Surgery | AAO-HNSF | society | yes | https://www.entnet.org/quality-practice/quality-products/clinical-practice-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American Academy of Pediatric Dentistry | - | society | yes | https://www.aapd.org/research/oral-health-policies--recommendations/ | NCRDSCB recognized specialties; official index; GC-publishers | pending |
| American Academy of Pediatrics | AAP | society | yes | https://publications.aap.org/pediatrics/collection/523/Clinical-Practice-Guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Academy of Physical Medicine & Rehabilitation | AAPM&R | society | yes | https://www.aapmr.org/quality-practice/clinical-guidance | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Association for Respiratory Care | AARC | society | yes | https://www.aarc.org/resource/clinical-practice-guidelines/ | GC-publishers; PubMed PMID 40323974 | pending |
| American Association for the Study of Liver Diseases | AASLD | society | yes | https://www.aasld.org/practice-guidelines | CMSS current members; official index; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Association of Clinical Endocrinology | AACE | society | yes | https://pro.aace.com/clinical-guidance | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American College of Allergy, Asthma & Immunology | - | society | yes | - | PubMed PMID 41936423; DOI 10.1016/j.anai.2025.10.026; GC-publishers; AAFP-PG | pending |
| American College of Cardiology | ACC | society | yes | https://www.acc.org/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American College of Chest Physicians (CHEST) | CHEST | society | yes | https://www.chestnet.org/guidelines-and-topic-collections/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American College of Emergency Physicians | ACEP | society | yes | https://www.acep.org/patient-care/clinical-policies | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American College of Medical Genetics and Genomics | ACMG | society | yes | https://www.acmg.net/ACMG/Medical-Genetics-Practice-Resources/Practice-Guidelines.aspx | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American College of Nuclear Medicine | - | society | yes | - | PubMed PMID 40169271; DOI 10.2967/jnmt.125.269834; GC-publishers | pending |
| American College of Obstetricians and Gynecologists | ACOG | society | yes | https://www.acog.org/clinical/clinical-guidance/clinical-practice-guideline | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American College of Occupational and Environmental Medicine | ACOEM | society | yes | https://acoem.org/Guidance-and-Position-Statements/Guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; ChoosingWisely | pending |
| American College of Physicians | ACP | society | yes | https://www.acponline.org/clinical-information/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American College of Radiology | ACR-Radiology | society | yes | https://acsearch.acr.org/list | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American College of Rheumatology | ACR-Rheumatology | society | yes | https://rheumatology.org/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American College of Surgeons | ACS | society | yes | https://www.facs.org/quality-programs/trauma/quality/best-practices-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American Dental Association | ADA-Dental | society | yes | https://www.ada.org/resources/research/science-and-research-institute/evidence-based-dental-research/clinical-practice-guidelines | major-issuer supplement; official index; GC-publishers; ChoosingWisely | pending |
| American Diabetes Association | ADA | society | yes | https://professional.diabetes.org/standards-of-care | major-issuer supplement; official index; GC-publishers; AAFP-PG | pending |
| American Epilepsy Society | AES | society | yes | https://www.aesnet.org/clinical-care/clinical-guidance | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Gastroenterological Association | AGA | society | yes | https://gastro.org/clinical-guidance/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Geriatrics Society | AGS | society | yes | https://www.americangeriatrics.org/publications-tools/guidelines-recommendations | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Heart Association | AHA | society | yes | https://professional.heart.org/en/guidelines-and-statements | major-issuer supplement; official index; GC-publishers; AAFP-PG | pending |
| American Occupational Therapy Association | AOTA | society | yes | https://www.aota.org/practice/practice-essentials/evidencebased-practiceknowledge-translation/practice-guidelines | major-issuer supplement; official index; GC-publishers; ChoosingWisely | pending |
| American Physical Therapy Association | APTA | society | yes | https://www.apta.org/patient-care/evidence-based-practice-resources/cpgs | major-issuer supplement; official index; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Psychiatric Association | APA | society | yes | https://www.psychiatry.org/psychiatrists/practice/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Psychological Association | APA-Psychology | society | yes | https://www.apa.org/about/offices/directorates/guidelines/clinical-practice | GC-publishers; PubMed PMID 34843274 | pending |
| American Society for Clinical Pathology | ASCP | society | yes | https://www.ascp.org/news/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American Society for Colposcopy and Cervical Pathology | - | society | yes | https://www.asccp.org/guidelines | major-issuer supplement; official index; GC-publishers; ChoosingWisely | pending |
| American Society for Gastrointestinal Endoscopy | ASGE | society | yes | https://www.asge.org/home/resources/publications/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | pending |
| American Society for Radiation Oncology | ASTRO | society | yes | https://www.astro.org/provider-resources/guidelines/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American Society for Reproductive Medicine | ASRM | society | yes | https://www.asrm.org/practice-guidance/practice-committee-documents/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Society of Addiction Medicine | ASAM | society | yes | https://www.asam.org/quality-care/clinical-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; AAFP-PG | pending |
| American Society of Anesthesiologists | ASA | society | yes | https://www.asahq.org/standards-and-practice-parameters | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American Society of Breast Surgeons | ASBrS | society | yes | https://www.breastsurgeons.org/resources/statements | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American Society of Clinical Oncology/Association for Clinical Oncology | ASCO | society | yes | https://ascopubs.org/guidelines?doi=10.1200%2FJCO&publicationCode=jco | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Society of Colon & Rectal Surgeons | ASCRS | society | yes | https://fascrs.org/healthcare-providers/education/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; AAFP-PG | pending |
| American Society of Echocardiography | ASE | society | yes | https://www.asecho.org/practice-clinical-resources/ase-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| American Society of Health-System Pharmacists | ASHP | society | yes | https://www.ashp.org/pharmacy-practice/policy-positions-and-guidelines/browse-by-document-type/guidelines | JCPP members; official index; GC-publishers; ChoosingWisely | pending |
| American Society of Hematology | ASH | society | yes | https://www.hematology.org/education/clinicians/guidelines-and-quality-care/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Society of Nephrology | ASN | society | yes | https://www.asn-online.org/policy/guidelines.aspx | PubMed PMID 42265997; DOI 10.1016/j.jacc.2026.03.056; GC-publishers; ChoosingWisely | pending |
| American Society of Plastic Surgeons | ASPS | society | yes | https://www.plasticsurgery.org/for-medical-professionals/quality/evidence-based-clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | pending |
| American Thoracic Society | ATS | society | yes | https://site.thoracic.org/clinicians-researchers/clinical-practice-guidelines-statements-reports | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Urological Association | AUA | society | yes | https://www.auanet.org/guidelines-and-quality/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| Association for Molecular Pathology | - | society | yes | - | PubMed PMID 28165299; DOI 10.1200/JCO.2016.71.9807 | pending |
| Association for the Advancement of Blood & Biotherapies | AABB | society | yes | https://www.aabb.org/news-resources/resources/clinical-practice-guidelines | GC-publishers; PubMed PMID 37824153 | pending |
| Association of periOperative Registered Nurses | - | society | yes | https://www.aorn.org/guidelines-resources/guidelines-for-perioperative-practice | ANA organizational affiliates; official index; GC-publishers | pending |
| Clinical Immunology Society | CIS | society | yes | - | PubMed PMID 41936423; DOI 10.1016/j.anai.2025.10.026 | pending |
| College of American Pathologists | CAP | society | yes | https://www.cap.org/protocols-and-guidelines/cap-guidelines/current-cap-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | pending |
| Infectious Diseases Society of America | IDSA | society | yes | https://www.idsociety.org/practice-guideline/all-practice-guidelines/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| North American Society for Pediatric Gastroenterology, Hepatology and Nutrition | NASPGHAN | society | yes | https://naspghan.org/professional-resources/clinical-guidelines/ | CMSS current members; official index; GC-publishers | pending |
| North American Spine Society | NASS | society | yes | https://www.spine.org/Research-Clinical-Care/Quality-Improvement/Clinical-Guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| Obesity Medicine Association | OMA | society | yes | https://obesitymedicine.org/resources/obesity-algorithm/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | pending |
| Oncology Nursing Society | - | society | yes | https://www.ons.org/clinical-tools/guidelines | ANA organizational affiliates; official index; GC-publishers | pending |
| Post-Acute and Long-Term Care Medical Association | PALTmed | society | yes | https://paltmed.org/products/results?product_type=27 | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| Society for Healthcare Epidemiology of America | - | society | yes | - | PubMed PMID 23327981; DOI 10.2146/ajhp120568 | pending |
| Society for Vascular Surgery | SVS | society | yes | https://vascular.org/research-quality/guidelines-and-reporting-standards/clinical-practice-guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| Society of Breast Imaging | SBI | society | yes | - | GC-publishers; DOI 10.1001/jamasurg.2026.0613 | pending |
| Society of Critical Care Medicine | SCCM | society | yes | https://www.sccm.org/clinical-resources/guidelines/guidelines | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| Society of Family Planning | - | society | yes | https://societyfp.org/clinical/clinical-guidance-library/ | GC-publishers; PubMed PMID 39710335 | pending |
| Society of Gynecologic Oncology | SGO | society | yes | https://www.sgo.org/practice-management/statements-and-recommendations/ | CMSS current members; official index; GC-publishers; ChoosingWisely | pending |
| Society of Interventional Radiology | SIR | society | yes | https://www.sirweb.org/in-practice/guidelines-and-statements/ | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers | pending |
| Society of Nuclear Medicine and Molecular Imaging | SNMMI | society | yes | https://sites.snmmi.org/Web/Web/Clinical-Practice/Procedure-Standards/Default.aspx | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely | pending |
| Society of Surgical Oncology | SSO | society | yes | https://www.surgonc.org/resources/guidelines/ | CMSS current members; official index; GC-publishers; ChoosingWisely | pending |
| Surgical Infection Society | SIS | society | yes | - | PubMed PMID 23327981; DOI 10.2146/ajhp120568 | pending |
| The Society of Thoracic Surgeons | STS | society | yes | https://www.sts.org/resources/clinical-decision-making/clinical-practice-documents-and-policies | AMA HOD roster; AMA SSS roster; CMSS current members; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| Academic Pediatric Association | - | society | unverified | - | ChoosingWisely | pending |
| Academy of Breastfeeding Medicine | - | society | unverified | - | GC-publishers | pending |
| Academy of Consultation-Liaison Psychiatry | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Academy of Managed Care Pharmacy | - | society | unverified | - | JCPP members | pending |
| Academy of Medical-Surgical Nurses | - | society | unverified | - | ANA organizational affiliates | pending |
| Academy of Physicians in Clinical Research | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Aerospace Medical Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Academy of Addiction Psychiatry | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Academy of Ambulatory Care Nursing | - | society | unverified | - | ANA organizational affiliates | pending |
| American Academy of Audiology | - | society | unverified | - | major-issuer supplement | pending |
| American Academy of Child & Adolescent Psychiatry | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | pending |
| American Academy of Clinical Toxicology | - | society | unverified | - | ChoosingWisely | pending |
| American Academy of Cosmetic Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Academy of Dental Sleep Medicine | - | society | unverified | - | GC-publishers | pending |
| American Academy of Emergency Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Academy of Facial Plastic and Reconstructive Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Academy of Insurance Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Academy of Nursing | - | society | unverified | - | ChoosingWisely | pending |
| American Academy of Oral & Maxillofacial Pathology | - | society | unverified | - | NCRDSCB recognized specialties | pending |
| American Academy of Oral & Maxillofacial Radiology | - | society | unverified | - | NCRDSCB recognized specialties; GC-publishers | pending |
| American Academy of Oral Medicine | - | society | unverified | - | NCRDSCB recognized specialties | pending |
| American Academy of Orofacial Pain | - | society | unverified | - | NCRDSCB recognized specialties | pending |
| American Academy of Osteopathy | - | society | unverified | - | AOA specialty-college directory | pending |
| American Academy of Otolaryngic Allergy | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Academy of Pain Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | pending |
| American Academy of Periodontology | - | society | unverified | - | NCRDSCB recognized specialties | pending |
| American Academy of Psychiatry and the Law | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Academy of Sleep Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Academy of Thermology | - | society | unverified | - | GC-publishers | pending |
| American Association for Geriatric Psychiatry | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Association for Hand Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Association for Pediatric Ophthalmology and Strabismus | - | society | unverified | - | ChoosingWisely | pending |
| American Association for the Surgery of Trauma | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| American Association for Thoracic Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | pending |
| American Association of Bronchology and Interventional Pulmonology | - | society | unverified | - | GC-publishers | pending |
| American Association of Clinical Urologists | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Association of Critical-Care Nurses | - | society | unverified | - | ANA organizational affiliates | pending |
| American Association of Endocrine Surgeons | - | society | unverified | - | AMA SSS roster; GC-publishers | pending |
| American Association of Endodontists | - | society | unverified | - | NCRDSCB recognized specialties; GC-publishers | pending |
| American Association of Gynecologic Laparoscopists | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | pending |
| American Association of Hip and Knee Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Association of Neurological Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | pending |
| American Association of Neuromuscular & Electrodiagnostic Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Association of Neuroscience Nurses | - | society | unverified | - | ANA organizational affiliates; GC-publishers; ChoosingWisely | pending |
| American Association of Nurse Anesthesiology | - | society | unverified | - | ANA organizational affiliates | pending |
| American Association of Oral and Maxillofacial Surgeons | - | society | unverified | - | NCRDSCB recognized specialties | pending |
| American Association of Orthodontists | - | society | unverified | - | NCRDSCB recognized specialties | pending |
| American Association of Physicians of Indian Origin | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Association of Physicists in Medicine | - | society | unverified | - | GC-publishers | pending |
| American Association of Plastic Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Association of Psychiatric Pharmacists | - | society | unverified | - | JCPP members | pending |
| American Association of Public Health Dentistry | - | society | unverified | - | NCRDSCB recognized specialties | pending |
| American Association of Public Health Physicians | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Burn Association | - | society | unverified | - | major-issuer supplement | pending |
| American Cannabis Nurses Association | - | society | unverified | - | ANA organizational affiliates | pending |
| American Chiropractic Association | - | society | unverified | - | ChoosingWisely | pending |
| American Clinical Neurophysiology Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American College Health Association | - | society | unverified | - | GC-publishers | pending |
| American College of Clinical Pharmacy | ACCP | society | unverified | - | JCPP members; GC-publishers | pending |
| American College of Correctional Physicians | - | society | unverified | - | AMA SSS roster | pending |
| American College of Foot and Ankle Surgeons | - | society | unverified | - | major-issuer supplement | pending |
| American College of Gastroenterology | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | pending |
| American College of Legal Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American College of Lifestyle Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American College of Medical Quality | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American College of Medical Toxicology | - | society | unverified | - | AMA SSS roster; ChoosingWisely | pending |
| American College of Mohs Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American College of Nurse-Midwives | - | society | unverified | - | major-issuer supplement | pending |
| American College of Osteopathic Emergency Physicians | - | society | unverified | - | AOA specialty-college directory | pending |
| American College of Osteopathic Family Physicians | - | society | unverified | - | AOA specialty-college directory | pending |
| American College of Osteopathic Internists | - | society | unverified | - | AOA specialty-college directory | pending |
| American College of Osteopathic Neurologists and Psychiatrists | - | society | unverified | - | AOA specialty-college directory | pending |
| American College of Osteopathic Obstetricians and Gynecologists | - | society | unverified | - | AOA specialty-college directory | pending |
| American College of Osteopathic Pediatricians | - | society | unverified | - | AOA specialty-college directory | pending |
| American College of Osteopathic Surgeons | - | society | unverified | - | AOA specialty-college directory | pending |
| American College of Preventive Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | pending |
| American College of Prosthodontists | - | society | unverified | - | NCRDSCB recognized specialties | pending |
| American College of Radiation Oncology | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American College of Sports Medicine | - | society | unverified | - | major-issuer supplement; GC-publishers; AAFP-PG | pending |
| American Congress of Rehabilitation Medicine | - | society | unverified | - | GC-publishers | pending |
| American Contact Dermatitis Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Dermatological Association | - | society | unverified | - | AMA SSS roster | pending |
| American Foregut Society | - | society | unverified | - | AMA SSS roster | pending |
| American Glaucoma Society | - | society | unverified | - | GC-publishers | pending |
| American Headache Society | - | society | unverified | - | GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Hernia Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Holistic Nurses Association | - | society | unverified | - | ANA organizational affiliates | pending |
| American Institute of Ultrasound in Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Limb Preservation Society | - | society | unverified | - | GC-publishers | pending |
| American Medical Society for Sports Medicine | - | society | unverified | - | major-issuer supplement; GC-publishers; ChoosingWisely | pending |
| American Medical Women's Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Nephrology Nurses' Association | - | society | unverified | - | ANA organizational affiliates | pending |
| American Nursing Informatics Association | - | society | unverified | - | ANA organizational affiliates | pending |
| American Optometric Association | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| American Orthopaedic Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Orthopaedic Foot & Ankle Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; ChoosingWisely | pending |
| American Osteopathic Academy of Addiction Medicine | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic Academy of Orthopedics | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic Academy of Sports Medicine | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic Association of Prolotherapy Regenerative Medicine | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic College of Allergy and Immunology | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic College of Anesthesiologists | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic College of Dermatology | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic College of Occupational and Preventive Medicine | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic College of Pathologists | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic College of Physical Medicine and Rehabilitation | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic College of Proctology | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic College of Radiology | - | society | unverified | - | AOA specialty-college directory | pending |
| American Osteopathic Colleges of Ophthalmology and Otolaryngology–Head and Neck Surgery | - | society | unverified | - | AOA specialty-college directory | pending |
| American Pediatric Surgical Nurses Association | - | society | unverified | - | ANA organizational affiliates; ChoosingWisely | pending |
| American Pharmacists Association | APhA | society | unverified | - | JCPP members | pending |
| American Podiatric Medical Association | - | society | unverified | - | major-issuer supplement; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| American Psychiatric Nurses Association | - | society | unverified | - | ANA organizational affiliates | pending |
| American Rhinologic Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Roentgen Ray Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Shoulder and Elbow Surgeons | - | society | unverified | - | AMA SSS roster | pending |
| American Society for Aesthetic Plastic Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Society for Bone and Mineral Research | - | society | unverified | - | GC-publishers | pending |
| American Society for Clinical Laboratory Science | - | society | unverified | - | ChoosingWisely | pending |
| American Society for Dermatologic Surgery Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Society for Laser Medicine and Surgery, Inc. | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Society for Metabolic and Bariatric Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Society for Microbiology | - | society | unverified | - | GC-publishers; ChoosingWisely | pending |
| American Society for Nutrition | - | society | unverified | - | GC-publishers | pending |
| American Society for Parenteral and Enteral Nutrition | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| American Society for Reconstructive Microsurgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Society for Surgery of the Hand | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Society for Transplantation and Cellular Therapy | - | society | unverified | - | GC-publishers | pending |
| American Society of Cataract and Refractive Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Society of Clinical Psychopharmacology | - | society | unverified | - | GC-publishers | pending |
| American Society of Consultant Pharmacists | - | society | unverified | - | JCPP members; ChoosingWisely | pending |
| American Society of Cytopathology | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Society of Dentist Anesthesiologists | - | society | unverified | - | NCRDSCB recognized specialties | pending |
| American Society of Dermatopathology | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Society of Extracorporeal Technology | - | society | unverified | - | GC-publishers | pending |
| American Society of Interventional Pain Physicians | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Society of Ketamine Physicians, Psychotherapists, and Practitioners | - | society | unverified | - | GC-publishers | pending |
| American Society of Maxillofacial Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Society of Neuroimaging | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Society of Neuroradiology | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Society of Nuclear Cardiology | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | pending |
| American Society of Ophthalmic Plastic and Reconstructive Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| American Society of Pain and Neuroscience | - | society | unverified | - | AMA SSS roster; GC-publishers | pending |
| American Society of Pediatric Hematology/Oncology | - | society | unverified | - | ChoosingWisely | pending |
| American Society of Pediatric Nephrology | - | society | unverified | - | ChoosingWisely | pending |
| American Society of PeriAnesthesia Nurses | - | society | unverified | - | ANA organizational affiliates | pending |
| American Society of Preventive Cardiology | - | society | unverified | - | GC-publishers | pending |
| American Society of Regional Anesthesia and Pain Medicine | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Society of Retina Specialists | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Society of Transplant Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| American Society of Transplantation | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| American Society of Tropical Medicine and Hygiene | - | society | unverified | - | GC-publishers | pending |
| American Speech-Language-Hearing Association | ASHA | society | unverified | - | major-issuer supplement | pending |
| American Thyroid Association | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| American Urogynecologic Society | - | society | unverified | - | AMA SSS roster; GC-publishers; ChoosingWisely | pending |
| American Vein and Lymphatic Society | - | society | unverified | - | AMA SSS roster; GC-publishers | pending |
| American Venous Forum | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| AMSUS—The Society of Federal Health Professionals | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| APTA Academy of Education | - | society | unverified | - | APTA sections/academies | pending |
| APTA Acute Care | - | society | unverified | - | APTA sections/academies | pending |
| APTA Aquatics | - | society | unverified | - | APTA sections/academies | pending |
| APTA Cardiovascular and Pulmonary | - | society | unverified | - | APTA sections/academies | pending |
| APTA Clinical Electrophysiology and Wound Management | - | society | unverified | - | APTA sections/academies | pending |
| APTA Federal | - | society | unverified | - | APTA sections/academies | pending |
| APTA Geriatrics | - | society | unverified | - | APTA sections/academies | pending |
| APTA Hand and Upper Extremity | - | society | unverified | - | APTA sections/academies | pending |
| APTA Home Health | - | society | unverified | - | APTA sections/academies | pending |
| APTA Leadership and Innovation | - | society | unverified | - | APTA sections/academies | pending |
| APTA Neurology | - | society | unverified | - | APTA sections/academies | pending |
| APTA Oncology | - | society | unverified | - | APTA sections/academies | pending |
| APTA Orthopedics | - | society | unverified | - | APTA sections/academies; GC-publishers | pending |
| APTA Pediatrics | - | society | unverified | - | APTA sections/academies | pending |
| APTA Pelvic Health | - | society | unverified | - | APTA sections/academies | pending |
| APTA Private Practice | - | society | unverified | - | APTA sections/academies | pending |
| APTA Research | - | society | unverified | - | APTA sections/academies | pending |
| APTA Sports | - | society | unverified | - | APTA sections/academies | pending |
| Association for Diagnostics & Laboratory Medicine | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| Association for Nursing Professional Development | - | society | unverified | - | ANA organizational affiliates | pending |
| Association for Professionals in Infection Control and Epidemiology | - | society | unverified | - | GC-publishers | pending |
| Association of Academic Physiatrists | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Association of Academic Radiology | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Association of Nurses in AIDS Care | - | society | unverified | - | ANA organizational affiliates | pending |
| Association of Pediatric Hematology/Oncology Nurses | - | society | unverified | - | ANA organizational affiliates | pending |
| Association of Physicians of Pakistani Descent of North America | - | society | unverified | - | AMA SSS roster | pending |
| Association of Professors of Dermatology | - | society | unverified | - | AMA SSS roster | pending |
| Association of Rehabilitation Nurses | - | society | unverified | - | ANA organizational affiliates | pending |
| Association of Women's Health, Obstetric & Neonatal Nurses | - | society | unverified | - | ANA organizational affiliates | pending |
| Chi Eta Phi Sorority, Incorporated | - | society | unverified | - | ANA organizational affiliates | pending |
| Child Neurology Society | - | society | unverified | - | GC-publishers; AAFP-PG | pending |
| Congress of Neurological Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | pending |
| Dermatology Nurses' Association | - | society | unverified | - | ANA organizational affiliates | pending |
| Digital Medicine Society | - | society | unverified | - | CMSS associate members | pending |
| Eastern Association for the Surgery of Trauma | - | society | unverified | - | major-issuer supplement | pending |
| Emergency Nurses Association | - | society | unverified | - | ANA organizational affiliates; GC-publishers | pending |
| Endocrine Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| Endourological Society | - | society | unverified | - | GC-publishers | pending |
| Enhanced Recovery After Cardiac Surgery Society | - | society | unverified | - | GC-publishers | pending |
| GLMA: Health Professionals Advancing LGBTQ+ Equality | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Health Ministries Association | - | society | unverified | - | ANA organizational affiliates | pending |
| Heart Failure Society of America | - | society | unverified | - | major-issuer supplement; GC-publishers; AAFP-PG | pending |
| Heart Rhythm Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; AAFP-PG | pending |
| Hematology/Oncology Pharmacy Association | - | society | unverified | - | JCPP members | pending |
| HIV Medicine Association | - | society | unverified | - | major-issuer supplement; GC-publishers; ChoosingWisely | pending |
| Hospice and Palliative Nurses Association | - | society | unverified | - | ANA organizational affiliates | pending |
| Infusion Nurses Society | - | society | unverified | - | major-issuer supplement | pending |
| International Academy of Independent Medical Evaluators | - | society | unverified | - | AMA SSS roster | pending |
| International Antiviral Society–USA | - | society | unverified | - | GC-publishers | pending |
| International College of Surgeons—US Section | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| International Nurses Society on Addictions | - | society | unverified | - | ANA organizational affiliates | pending |
| International Pain and Spine Intervention Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| International Society for the Advancement of Spine Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| International Society of Hair Restoration Surgery | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Korean American Medical Association | - | society | unverified | - | AMA SSS roster | pending |
| Musculoskeletal Tumor Society | - | society | unverified | - | GC-publishers | pending |
| National Association of Clinical Nurse Specialists | - | society | unverified | - | ANA organizational affiliates | pending |
| National Association of EMS Physicians | - | society | unverified | - | major-issuer supplement | pending |
| National Association of Epilepsy Centers | - | society | unverified | - | GC-publishers | pending |
| National Association of Indian Nurses of America | - | society | unverified | - | ANA organizational affiliates | pending |
| National Association of Medical Examiners | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| National Association of Neonatal Nurses | - | society | unverified | - | ANA organizational affiliates | pending |
| National Association of Nurse Practitioners in Women's Health | - | society | unverified | - | ANA organizational affiliates | pending |
| National Association of School Nurses | - | society | unverified | - | ANA organizational affiliates | pending |
| National Community Pharmacists Association | - | society | unverified | - | JCPP members | pending |
| National Lipid Association | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| National Medical Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| National Pharmaceutical Association | - | society | unverified | - | JCPP members | pending |
| National Society of Genetic Counselors | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| Neurocritical Care Society | - | society | unverified | - | GC-publishers | pending |
| North American Neuro-Ophthalmology Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| North American Neuroendocrine Tumor Society | - | society | unverified | - | GC-publishers | pending |
| North American Neuromodulation Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| North American Society for Cardiovascular Imaging | - | society | unverified | - | GC-publishers | pending |
| North American Society for Interventional Thyroidology | - | society | unverified | - | GC-publishers | pending |
| Organization for Associate Degree Nursing | - | society | unverified | - | ANA organizational affiliates | pending |
| Orthodox Jewish Nurses Association | - | society | unverified | - | ANA organizational affiliates | pending |
| Orthopaedic Trauma Association | - | society | unverified | - | AMA SSS roster | pending |
| Outpatient Endovascular and Interventional Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Pediatric Endocrine Society | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| Pediatric Infectious Diseases Society | - | society | unverified | - | major-issuer supplement; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| Pediatric Orthopaedic Society of North America | - | society | unverified | - | GC-publishers; ChoosingWisely | pending |
| Philippine Nurses Association of America | - | society | unverified | - | ANA organizational affiliates | pending |
| Preventive Cardiovascular Nurses Association | - | society | unverified | - | ANA organizational affiliates | pending |
| Radiological Society of North America | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| Renal Physicians Association | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Sigma | - | society | unverified | - | ANA organizational affiliates | pending |
| Skin of Color Society | - | society | unverified | - | AMA SSS roster | pending |
| Society for Academic Emergency Medicine | - | society | unverified | - | GC-publishers | pending |
| Society for Adolescent Health and Medicine | - | society | unverified | - | major-issuer supplement | pending |
| Society for Cardiovascular Angiography & Interventions | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| Society for Cardiovascular Magnetic Resonance | - | society | unverified | - | AMA HOD roster; AMA SSS roster; ChoosingWisely | pending |
| Society for Developmental and Behavioral Pediatrics | - | society | unverified | - | GC-publishers | pending |
| Society for Immunotherapy of Cancer | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| Society for Integrative Oncology | - | society | unverified | - | GC-publishers | pending |
| Society for Investigative Dermatology | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Society for Maternal-Fetal Medicine | - | society | unverified | - | AMA SSS roster; GC-publishers; ChoosingWisely; AAFP-PG | pending |
| Society for Obstetric Anesthesia and Perinatology | - | society | unverified | - | GC-publishers | pending |
| Society for Pediatric Anesthesia | - | society | unverified | - | GC-publishers | pending |
| Society for Pediatric Dermatology | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Society for the Advancement of Patient Blood Management | - | society | unverified | - | GC-publishers; ChoosingWisely | pending |
| Society for Vascular Medicine | - | society | unverified | - | GC-publishers; ChoosingWisely; AAFP-PG | pending |
| Society of Abdominal Radiology | - | society | unverified | - | GC-publishers | pending |
| Society of American Gastrointestinal and Endoscopic Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | pending |
| Society of Cardiovascular Anesthesiologists | - | society | unverified | - | GC-publishers | pending |
| Society of Cardiovascular Computed Tomography | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers; ChoosingWisely | pending |
| Society of Dermatology Physician Associates | - | society | unverified | - | GC-publishers | pending |
| Society of Infectious Diseases Pharmacists | - | society | unverified | - | GC-publishers | pending |
| Society of Laparoscopic & Robotic Surgeons | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Society of NeuroInterventional Surgery | - | society | unverified | - | GC-publishers | pending |
| Society of Pediatric Nurses | - | society | unverified | - | ChoosingWisely | pending |
| Society of Radiologists in Ultrasound | - | society | unverified | - | GC-publishers | pending |
| Society of Thoracic Radiology | - | society | unverified | - | GC-publishers | pending |
| Society of Urodynamics Female Pelvic Medicine & Urogenital Reconstruction | - | society | unverified | - | GC-publishers | pending |
| Society of Urologic Oncology | - | society | unverified | - | GC-publishers | pending |
| The American Association of Nurse Attorneys | - | society | unverified | - | ANA organizational affiliates | pending |
| The Menopause Society | - | society | unverified | - | major-issuer supplement; GC-publishers; AAFP-PG | pending |
| The Obesity Society | - | society | unverified | - | major-issuer supplement; GC-publishers | pending |
| The Triological Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster | pending |
| Undersea and Hyperbaric Medical Society | - | society | unverified | - | AMA HOD roster; AMA SSS roster; GC-publishers | pending |
| Urgent Care College of Physicians | - | society | unverified | - | AMA SSS roster | pending |
| Western Trauma Association | - | society | unverified | - | major-issuer supplement | pending |
| Wilderness Medical Society | - | society | unverified | - | major-issuer supplement; GC-publishers; AAFP-PG | pending |
| Wound Healing Society | - | society | unverified | - | major-issuer supplement | pending |
| Wound, Ostomy, and Continence Nurses Society | - | society | unverified | - | ANA organizational affiliates; GC-publishers | pending |
| American Academy of Hospice and Palliative Medicine | AAHPM | society | no | https://aahpm.org/publications/ | excluded: no AAHPM-issued CPG program identified | - |
| American Medical Informatics Association | AMIA | society | no | https://amia.org/public-policy/current-policy-priorities | excluded: policy and implementation guidance, not disease-specific patient-care CPGs | - |
| American Nurses Association | ANA | society | no | https://www.nursingworld.org/practice-policy/nursing-excellence/official-position-statements/ | excluded: position statements and standards, not disease-specific patient-care CPGs | - |
| American Osteopathic Association | - | society | no | https://osteopathic.org/?s=clinical+practice+guideline | excluded: no current AOA-issued patient-care CPG or index found | - |
| Society of General Internal Medicine | SGIM | society | no | https://www.sgim.org/publications/position-statements/ | excluded: position statements; no patient-care CPG program identified | - |
| Society of Hospital Medicine | SHM | society | no | https://www.hospitalmedicine.org/clinical-topics/clinical-guidelines/ | excluded: clinical resources; no SHM-issued CPG library identified | - |
| Alzheimer's Association | - | other | yes | https://www.alz.org/professionals/health-systems-medical-professionals/clinical-practice-guidelines | GC-publishers; PubMed PMID 40729527 | pending |
| American Cancer Society | ACS-Cancer | other | yes | https://www.cancer.org/health-care-professionals/american-cancer-society-prevention-early-detection-guidelines.html | major-issuer supplement; official index; GC-publishers; AAFP-PG | pending |
| Brain Trauma Foundation | - | other | yes | https://braintrauma.org/guidelines | GC-publishers; PubMed PMID 23971957 | pending |
| Clinical Pharmacogenetics Implementation Consortium | - | other | yes | https://www.clinpgx.org/cpic/guidelines | PubMed PMID 41979467; DOI 10.1002/cpt.70291 | pending |
| National Coalition for Hospice and Palliative Care | - | other | yes | https://www.nationalcoalitionhpc.org/ncp/ | GC-publishers; PubMed PMID 30915906 | pending |
| National Comprehensive Cancer Network | NCCN | other | yes | https://www.nccn.org/guidelines/category_1 | PubMed PMID 39019058; DOI 10.6004/jnccn.2024.0035 | pending |
| National Kidney Foundation | NKF | other | yes | https://www.ajkd.org/content/kdoqiguidelines | GC-publishers; PubMed PMID 32778223 | pending |
| Accreditation Council for Pharmacy Education | - | other | unverified | - | JCPP members | pending |
| American Association of Colleges of Nursing | - | other | unverified | - | ANA organizational affiliates | pending |
| American Association of Colleges of Pharmacy | - | other | unverified | - | JCPP members | pending |
| American Hospital Association | - | other | unverified | - | CMSS associate members | pending |
| American Medical Group Association | - | other | unverified | - | AMA HOD roster; AMA SSS roster; CMSS associate members | pending |
| American Red Cross | - | other | unverified | - | major-issuer supplement | pending |
| AO North America | - | other | unverified | - | AMA SSS roster | pending |
| Bone Health and Osteoporosis Foundation | - | other | unverified | - | major-issuer supplement | pending |
| Coalition for Health AI | - | other | unverified | - | CMSS associate members | pending |
| Consortium of Multiple Sclerosis Centers | - | other | unverified | - | GC-publishers | pending |
| Critical Care Societies Collaborative | - | other | unverified | - | ChoosingWisely | pending |
| Cystic Fibrosis Foundation | - | other | unverified | - | major-issuer supplement; GC-publishers | pending |
| Hidradenitis Suppurativa Foundation | - | other | unverified | - | GC-publishers | pending |
| Institute for Addressing Strangulation | - | other | unverified | - | GC-publishers | pending |
| International Essential Tremor Foundation | - | other | unverified | - | GC-publishers | pending |
| Michigan Quality Improvement Consortium | - | other | unverified | - | major-issuer supplement | pending |
| Muscular Dystrophy Association | - | other | unverified | - | GC-publishers | pending |
| National Alliance of State Pharmacy Associations | - | other | unverified | - | JCPP members | pending |
| National Association of Boards of Pharmacy | - | other | unverified | - | JCPP members | pending |
| National Bleeding Disorders Foundation | - | other | unverified | - | major-issuer supplement; GC-publishers | pending |
| National Commission on Correctional Health Care | - | other | unverified | - | GC-publishers | pending |
| National Council for Mental Wellbeing | - | other | unverified | - | GC-publishers | pending |
| National Health Council | - | other | unverified | - | CMSS associate members | pending |
| National Marrow Donor Program | - | other | unverified | - | GC-publishers | pending |
| National Multiple Sclerosis Society | - | other | unverified | - | major-issuer supplement | pending |
| National Network of Depression Centers | - | other | unverified | - | GC-publishers | pending |
| National Pressure Injury Advisory Panel | - | other | unverified | - | major-issuer supplement; GC-publishers | pending |
| National Psoriasis Foundation | - | other | unverified | - | GC-publishers; AAFP-PG | pending |
| National Rosacea Society | - | other | unverified | - | GC-publishers | pending |
| National Tuberculosis Coalition of America | - | other | unverified | - | GC-publishers | pending |
| North American Imaging in MS Cooperative | - | other | unverified | - | GC-publishers | pending |
| Obesity Action Coalition | - | other | unverified | - | GC-publishers | pending |
| Restless Legs Syndrome Foundation | - | other | unverified | - | GC-publishers | pending |
| ROME Foundation | - | other | unverified | - | GC-publishers | pending |
| Spondylitis Association of America | - | other | unverified | - | GC-publishers | pending |
| The OrthoForum | - | other | unverified | - | AMA SSS roster | pending |
| United States Cutaneous Lymphoma Consortium | - | other | unverified | - | GC-publishers | pending |
| Accreditation Council for Continuing Medical Education | - | other | no | - | excluded: training and accreditation body; no patient-care CPG program | - |
| Accreditation Council for Graduate Medical Education | - | other | no | - | excluded: training and accreditation body; no patient-care CPG program | - |
| American Board of Medical Specialties | - | other | no | - | excluded: certification body; no patient-care CPG program | - |
| Association for Hospital Medical Education | - | other | no | - | excluded: training and workforce body; no patient-care CPG program | - |
| Association of American Medical Colleges | - | other | no | - | excluded: training and workforce body; no patient-care CPG program | - |
| Federation of State Medical Boards | - | other | no | - | excluded: certification and workforce body; no patient-care CPG program | - |
| Intealth – Advancing the Global Health Workforce | - | other | no | - | excluded: training and workforce body; no patient-care CPG program | - |
| Kidney Disease: Improving Global Outcomes | KDIGO | other | no | https://kdigo.org/guidelines/ | excluded: non-American; Brussels-based independent foundation since 2013, founded 2003 by NKF, no current US lead sponsor | - |
| National Board of Medical Examiners | - | other | no | - | excluded: examination body; no patient-care CPG program | - |

## Guidelines

| org | title (year) | URL | access | status | notes |
|---|---|---|---|---|---|
| Advisory Committee on Immunization Practices | Prevention and Control of Seasonal Influenza with Vaccines: Recommendations of the Advisory Committee on Immunization Practices — United States, 2025–26 Influenza Season (2025) | https://www.cdc.gov/mmwr/volumes/74/wr/mm7432a2.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of Clesrovimab for Prevention of Severe Respiratory Syncytial Virus–Associated Lower Respiratory Tract Infections in Infants: Recommendations of the Advisory Committee on Immunization Practices — United States, 2025 (2025) | https://www.cdc.gov/mmwr/volumes/74/wr/mm7432a3.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of the GSK MenACWY-CRM/MenB-4C Pentavalent Meningococcal Vaccine Among Persons Aged ≥10 Years: Recommendations of the Advisory Committee on Immunization Practices — United States, 2025 (2025; pub 2026) | https://www.cdc.gov/mmwr/volumes/75/wr/mm7501a2.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Expanded Recommendations for Use of Pneumococcal Conjugate Vaccines Among Adults Aged ≥50 Years: Recommendations of the Advisory Committee on Immunization Practices — United States, 2024 (2024; pub 2025) | https://www.cdc.gov/mmwr/volumes/74/wr/mm7401a1.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Updated Recommendation for Universal Hepatitis B Vaccination in Adults Aged 19–59 Years — United States, 2024 (2024) | https://www.cdc.gov/mmwr/volumes/73/wr/mm7348a3.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of COVID-19 Vaccines for Persons Aged ≥6 Months: Recommendations of the Advisory Committee on Immunization Practices — United States, 2024–2025 (2024) | https://www.cdc.gov/mmwr/volumes/73/wr/mm7337e2.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of Haemophilus influenzae Type b–Containing Vaccines Among American Indian and Alaska Native Infants: Updated Recommendations of the Advisory Committee on Immunization Practices ― United States, 2024 (2024) | https://www.cdc.gov/mmwr/volumes/73/wr/mm7336a4.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Tick-Borne Encephalitis Vaccine: Recommendations of the Advisory Committee on Immunization Practices, United States, 2023 (2023) | https://www.cdc.gov/mmwr/volumes/72/rr/rr7205a1.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of Inactivated Polio Vaccine Among U.S. Adults: Updated Recommendations of the Advisory Committee on Immunization Practices — United States, 2023 (2023) | https://www.cdc.gov/mmwr/volumes/72/wr/mm7249a3.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of JYNNEOS (Smallpox and Mpox Vaccine, Live, Nonreplicating) for Persons Aged ≥18 Years at Risk for Mpox During an Mpox Outbreak: Recommendations of the Advisory Committee on Immunization Practices — United States, 2023 (2023; pub 2025) | https://www.cdc.gov/mmwr/volumes/74/wr/mm7422a3.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Cholera Vaccine: Recommendations of the Advisory Committee on Immunization Practices, 2022 (2022) | https://www.cdc.gov/mmwr/volumes/71/rr/rr7102a1.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Measles, Mumps, Rubella Vaccine (PRIORIX): Recommendations of the Advisory Committee on Immunization Practices — United States, 2022 (2022) | https://www.cdc.gov/mmwr/volumes/71/wr/mm7146a1.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of a Modified Preexposure Prophylaxis Vaccination Schedule to Prevent Human Rabies: Recommendations of the Advisory Committee on Immunization Practices — United States, 2022 (2022) | https://www.cdc.gov/mmwr/volumes/71/wr/mm7118a2.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of Recombinant Zoster Vaccine in Immunocompromised Adults Aged ≥19 Years: Recommendations of the Advisory Committee on Immunization Practices — United States, 2022 (2022) | https://www.cdc.gov/mmwr/volumes/71/wr/mm7103a2.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Dengue Vaccine: Recommendations of the Advisory Committee on Immunization Practices, United States, 2021 (2021) | https://www.cdc.gov/mmwr/volumes/70/rr/rr7006a1.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of Ebola Vaccine: Expansion of Recommendations of the Advisory Committee on Immunization Practices To Include Two Additional Populations — United States, 2021 (2021; pub 2022) | https://www.cdc.gov/mmwr/volumes/71/wr/mm7108a2.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Licensure of a Diphtheria and Tetanus Toxoids and Acellular Pertussis, Inactivated Poliovirus, Haemophilus influenzae Type b Conjugate, and Hepatitis B Vaccine, and Guidance for Use in Infants (2020) | https://www.cdc.gov/mmwr/volumes/69/wr/mm6905a5.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Prevention of Hepatitis A Virus Infection in the United States: Recommendations of the Advisory Committee on Immunization Practices, 2020 (2020) | https://www.cdc.gov/mmwr/volumes/69/rr/rr6905a1.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Human Papillomavirus Vaccination for Adults: Updated Recommendations of the Advisory Committee on Immunization Practices (2019) | https://www.cdc.gov/mmwr/volumes/68/wr/mm6832a3.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Japanese Encephalitis Vaccine: Recommendations of the Advisory Committee on Immunization Practices (2019) | https://www.cdc.gov/mmwr/volumes/68/rr/rr6802a1.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of Anthrax Vaccine in the United States: Recommendations of the Advisory Committee on Immunization Practices, 2019 (2019) | https://www.cdc.gov/mmwr/volumes/68/rr/rr6804a1.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of Tetanus Toxoid, Reduced Diphtheria Toxoid, and Acellular Pertussis Vaccines: Updated Recommendations of the Advisory Committee on Immunization Practices — United States, 2019 (2019; pub 2020) | https://www.cdc.gov/mmwr/volumes/69/wr/mm6903a5.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Updated Recommendations for the Use of Typhoid Vaccine — Advisory Committee on Immunization Practices, United States, 2015 (2015) | https://www.cdc.gov/mmwr/preview/mmwrhtml/mm6411a4.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Yellow Fever Vaccine Booster Doses: Recommendations of the Advisory Committee on Immunization Practices, 2015 (2015) | https://www.cdc.gov/mmwr/preview/mmwrhtml/mm6423a5.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | FDA Approval of an Extended Period for Administering VariZIG for Postexposure Prophylaxis of Varicella (2012) | https://www.cdc.gov/mmwr/preview/mmwrhtml/mm6112a4.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Addition of History of Intussusception as a Contraindication for Rotavirus Vaccination (2011) | https://www.cdc.gov/mmwr/preview/mmwrhtml/mm6041a5.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices | Use of Combination Measles, Mumps, Rubella, and Varicella Vaccine (2010) | https://www.cdc.gov/mmwr/preview/mmwrhtml/rr5903a1.htm | open | unqueued | - |
| Advisory Committee on Immunization Practices + Healthcare Infection Control Practices Advisory Committee | Recommendations for Using Smallpox Vaccine in a Pre-Event Vaccination Program: Supplemental Recommendations of the Advisory Committee on Immunization Practices and the Healthcare Infection Control Practices Advisory Committee (2003) | https://www.cdc.gov/mmwr/preview/mmwrhtml/rr5207a1.htm | open | unqueued | co-issued |
| Centers for Disease Control and Prevention | CDC Clinical Practice Guideline for Prescribing Opioids for Pain — United States, 2022 (2022) | https://www.cdc.gov/mmwr/volumes/71/rr/rr7103a1.htm | open | in-progress | id=cdc-2022-opioid |
| Defense Health Agency Joint Trauma System | (Acute Extremity) Compartment Syndrome (CS) and the Role of Fasciotomy in Extremity War Wounds (2026) | https://jts.health.mil/assets/docs/cpgs/Extremity_Compartment_Syndrome_and_Fasciotomy_ID17_21_May_2026.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (Guideline for Forward Management of) Acute Mental Health Conditions by Non-specialty Medical Personnel (2026) | https://jts.health.mil/assets/docs/cpgs/Mental_Health_Mar_2026.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (The Use of) Dried Plasma in the Deployed Trauma System and Contingency Operations (2026) | https://jts.health.mil/assets/docs/cpgs/Dried_Plasma_CPG_09_Jun_2026_ID103.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Airway Management in Trauma (2026) | https://jts.health.mil/assets/docs/cpgs/Airway_Management_in_Trauma_28_Jan_2026_ID39.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Chemical, Biological, Radiological and Nuclear (CBRN) Injury Part I: Initial Response to CBRN Agents (2026) | https://jts.health.mil/assets/docs/cpgs/CBRN_Injury_Part1_Initial_Response_08_Apr_2026_ID69_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Healthcare Provider Responsibilities (2026) | https://jts.health.mil/assets/docs/cpgs/K9_Healthcare_Provider_Responsibilities_CPG_C1_22_Jun_2026.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Hypothermia and Cold Injuries (2026) | https://jts.health.mil/assets/docs/cpgs/K9_Hypothermia_CPG_C10_22_June_2026.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | iCOVER Practical Activity Cards (2026) | https://jts.health.mil/assets/docs/cpgs/iCOVER_Practical_Activity_Cards_6FEB2026.pdf | open | excluded(derivative; parent guideline not issued here) | training curriculum |
| Defense Health Agency Joint Trauma System | iCOVER Stand Alone Training (2026) | https://jts.health.mil/assets/docs/cpgs/iCOVER_stand-alone_training_6FEB2026.pdf | open | excluded(derivative; parent guideline not issued here) | training curriculum |
| Defense Health Agency Joint Trauma System | iCOVER Train The Trainer Training (2026) | https://jts.health.mil/assets/docs/cpgs/iCOVER_Train-the-Trainer_6FEB2026.pdf | open | excluded(derivative; parent guideline not issued here) | training curriculum |
| Defense Health Agency Joint Trauma System | Joint En Route Care Guidelines, Committee on En Route Combat Casualty Care (2026) | https://jts.health.mil/assets/docs/cpgs/CoERCCC%20Guidelines%20FY26.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Medical Records Documentation (2026) | https://jts.health.mil/assets/docs/cpgs/K9_Medical_Records_Documentation_CPG_c22_01_APR_2026.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Orthopaedic Trauma: Extremity Fractures (2026) | https://jts.health.mil/assets/docs/cpgs/Orthopaedic_Trauma_Extremity_Fractures_ID56_22_Jun_2026.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Pelvic Fracture Care (2026) | https://jts.health.mil/assets/docs/cpgs/Pelvic_Fracture_Care_17_Feb_2026_ID34v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Snakebite Envenomation (2026) | https://jts.health.mil/assets/docs/cpgs/Snakebite_Envenomation_CPG_ID81_26_Apr_2026_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Spider and Scorpion Envenomation (2026) | https://jts.health.mil/assets/docs/cpgs/Spider_and_Scorpion_Envenomation_21_Jul_2026_ID84_v1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (Emergency) Airway Management (2025) | https://jts.health.mil/assets/docs/cpgs/K9_Airway_Management_CPG_c3_18_Dec_2025.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | (Human) Space Flight Emergencies in the Prehospital Environment (2025) | https://jts.health.mil/assets/docs/cpgs/(JOINTSTAFF)_20251223_CPG_Space_Medicine_final_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (Use of) Traumatic Brain Injury Biomarkers after a Potentially Concussive Event (2025) | https://jts.health.mil/assets/docs/cpgs/Use_of_TBI_Biomarkers_after_Potentially_Concussive_Event_14_Apr_2025_ID90_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Aerial Delivery of Fresh and Stored Blood Products (2025) | https://jts.health.mil/assets/docs/cpgs/Aerial_Delivery_Fresh_Stored_Blood_Products_01_Dec_2025.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Arachnid and Snake Envenomation (2025) | https://jts.health.mil/assets/docs/cpgs/Arachnid_Snake_Envenomation_MWD_CPG_c11_29_Mar_2025_v1.2.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Aural Blast Injury/Acoustic Trauma and Hearing Loss (2025) | https://jts.health.mil/assets/docs/cpgs/Aural_Blast_Injury_Acoustic_Trauma_and_Hearing_Loss_14_Aug_2025_ID05_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Blast, Burn, and Crush Injuries (2025) | https://jts.health.mil/assets/docs/cpgs/K9_Blast_Burn_Crush_Injuries_CPG_c12_30_Dec_2025.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Burn Care (2025) | https://jts.health.mil/assets/docs/cpgs/Burn_Care_CPG_10_June_2025_ID12_v1.3.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Canine Posttraumatic Stress (C-PTS) and Canine Posttraumatic Stress Disorder (C-PTSD) (2025) | https://jts.health.mil/assets/docs/cpgs/K9_C-PTS_C-PTSD_CPG_c18_14_Aug_2025_v1.1.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Cardiopulmonary Resuscitation (CPR) (2025) | https://jts.health.mil/assets/docs/cpgs/K9_CPR_CPG_c5_14_Aug_2025_v1.1.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Chemical, Biological, Radiological, and Nuclear (CBRN) Injury Response Part 4: General Approach to Biological Casualties (2025) | https://jts.health.mil/assets/docs/cpgs/CBRN_Part_4_General_Approach_to_Biological_Casualties_27_Feb_2025_ID101.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Drowning Management (2025) | https://jts.health.mil/assets/docs/cpgs/Drowning_Management_17_Mar_2025_ID64_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Emergency Life-Saving Cranial Procedures by Non-Neurosurgeons in Deployed Setting (2025) | https://jts.health.mil/assets/docs/cpgs/Emergency_Cranial_Procedures_by_Non-neurosurgeons_10_June_2025_ID68_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Euthanasia (2025) | https://jts.health.mil/assets/docs/cpgs/K9_Euthanasia_MWD_CPG_c21_03_Apr_2025_v1.1.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Fostering Resilience and Managing Emotions (2025) | https://jts.health.mil/assets/docs/cpgs/frame_mental_skills_with_human_remains_18JUN25.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Heat Injury (2025) | https://jts.health.mil/assets/docs/cpgs/K9_Heat_Injury_MWD_CPG_c9_29_Mar_2025_v1.1.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | i-STAT Portable Blood Analyzer in Austere Locations (2025) | https://jts.health.mil/assets/docs/cpgs/i-STAT_Portable_CCATT_Final_26_FEB_2025.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Interfacility Transport of Patients Between Medical Treatment Facilities (2025) | https://jts.health.mil/assets/docs/cpgs/Interfacility_Transport_CoERCCC_OPG.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Mechanical Ventilation Basics (2025) | https://jts.health.mil/assets/docs/cpgs/Mechnical_Ventilation_Basics_09_Apr_2025_ID92_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Mechanical Ventilation during Critical Care Air Transport (2025) | https://jts.health.mil/assets/docs/cpgs/Mechanical_Ventilation_CCATT_10_MAR_2025.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Negative Pressure Wound Therapy (2025) | https://jts.health.mil/assets/docs/cpgs/NPWT_CCATT_26_Feb_2025.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Normal Clinical Parameters (2025) | https://jts.health.mil/assets/docs/cpgs/Normal_Clinical_Parameters_MWD_c2_05_May_2025_v1.2.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Nursing Interventions, Wound Care, and Splint Management in Prolonged Casualty Care (2025) | https://jts.health.mil/assets/docs/cpgs/Nursing_Interventions_PCC_08_July_2025_ID70_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Ocular Injuries (2025) | https://jts.health.mil/assets/docs/cpgs/K9_Ocular_Injuries_CPG_c15_30_Dec_2025.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Resuscitative Endovascular Balloon Occlusion of the Aorta (REBOA) for Hemorrhagic Shock (2025) | https://jts.health.mil/assets/docs/cpgs/REBOA_for_Hemorrhagic_Shock_4.3.2026_ID38_v1.4.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Transcutaneous Temporary Transvenous Pacing (2025) | https://jts.health.mil/assets/docs/cpgs/TVTCP_CCATT_26_Jan_2025.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Transfusion of Type A Whole Blood for the Role 3 (2025) | https://jts.health.mil/assets/docs/cpgs/Type_A_Specific_WB_Transfusion_30_May_2025_ID96_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | U.S. Army Aeromedical Evacuation Standard Medical Operating Guidelines (SMOG) (2025) | https://jts.health.mil/assets/docs/cpgs/US_Army_Aeromedical_Evacuation_Standard_Medical_Operating_Guidelines_26NOV2025.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Universal Portable Anesthesia Complete (UPAC) Vaporizer And Mechanical Ventilation (2025) | https://jts.health.mil/assets/docs/cpgs/UPAC_Ventilation_Guide.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Vascular Injury (2025) | https://jts.health.mil/assets/docs/cpgs/Vascular_Injury_09_Apr_2025_ID46_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Wound Management (2025) | https://jts.health.mil/assets/docs/cpgs/K9_Wound_Management_CPG_c14_18_Dec_2025.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | (Acute) Concussion Management and Progressive Return to Activity (2024) | https://jts.health.mil/assets/docs/cpgs/Progressive_Return_to_Activity_Primary_Care_for_Acute_Concussion_Management_January_2024.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (High Bilateral) Amputations and Dismounted Complex Blast Injury (2024) | https://jts.health.mil/assets/docs/cpgs/High_Bilateral_Amputations_Dismounted_Complex_Blast_Injury_05_Aug_2024_ID22_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (Management of) Stroke and Cerebrovascular Emergencies in the Deployed Setting (2024) | https://jts.health.mil/assets/docs/cpgs/Stroke_Cerebrovascular_Emergencies_Deployed_Setting_03_July_2024.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Altitude Emergencies Prehospital Environment (2024) | https://jts.health.mil/assets/docs/cpgs/Altitude_Emergencies_Prehospital_Environment_05_Mar_2024_ID95_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Amputation: Evaluation and Treatment, 10 Oct 2024 (2024) | https://jts.health.mil/assets/docs/cpgs/Amputation_Evaluation_and_Treatment_10_Oct_2024_ID07_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Chemical, Biological, Radiological and Nuclear (CBRN) Injury Response Part 3: Medical Management of Radiation Exposure and Nuclear Events (2024) | https://jts.health.mil/assets/docs/cpgs/CBRN_3_20_Aug_2024_ID93_v1.3.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | CPG Development Process (2024) | https://jts.health.mil/assets/docs/cpgs/JTS_CPG_Development_Process_04_Oct_2024_ID54.pdf | open | excluded(no patient-care recommendations) | - |
| Defense Health Agency Joint Trauma System | En Route Care Patient Packaging (2024) | https://jts.health.mil/assets/docs/cpgs/En_Route_Care_Patient_Packaging_21_Aug_2024_ID97_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Frozen and Deglycerolized Red Blood Cells (2024) | https://jts.health.mil/assets/docs/cpgs/Frozen_Deglycerolized_Red-Blood_Cells_05_Aug_2024_ID26_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Genitourinary (GU) Injury Trauma Management (2024) | https://jts.health.mil/assets/docs/cpgs/Genitourinary_Injury_Trauma_Management_29_Mar_2024_ID42_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | JTS CPG Author Guidance (2024) | https://jts.health.mil/assets/docs/cpgs/JTS_CPG_Author_Guidance_28_Aug_2024.pdf | open | excluded(no patient-care recommendations) | - |
| Defense Health Agency Joint Trauma System | Prevention of Venous Thromboembolism (2024) | https://jts.health.mil/assets/docs/cpgs/Prevention_of_Venous_Thromboembolism_29_Mar_2024_ID36v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Suspected Radio Frequency Electromagnetic Field Overexposure (2024) | https://jts.health.mil/assets/docs/cpgs/Radiofrequency_EMF_Overexposure_CPG_12_Jul_2024_ID98_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Hypothermia Prevention, Monitoring, and Management (2023) | https://jts.health.mil/assets/docs/cpgs/Hypothermia_Prevention_Treatment_07_Jun_2023_ID23.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Invasive Fungal Infection in War Wounds (2023) | https://jts.health.mil/assets/docs/cpgs/Invasive_Fungal_Infection_in_War_Wounds_17_Jul_2023_ID28_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Telemedicine in the Deployed Setting (2023) | https://jts.health.mil/assets/docs/cpgs/Telemedicine_Deployed_Setting_19_Sep_2023_ID94_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Traumatic Brain Injury and Neurosurgery in the Deployed Environment (2023) | https://jts.health.mil/assets/docs/cpgs/TBI_Neurosurgery_Deployed%20Environment_15_Sep_2023_ID30_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Chemical, Biological, Radiological and Nuclear (CBRN) Injury Response Part 2: Medical Management of Chemical Agent Exposure (2022) | https://jts.health.mil/assets/docs/cpgs/CBRN_Injury_Response_Part_2_Medical_Management_25_Mar_2022_ID69.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Hyperkalemia and Dialysis in the Deployed Setting (2022) | https://jts.health.mil/assets/docs/cpgs/Hyperkalemia_and_Dialysis_in_Deployed_Setting_25_Apr_2022_ID52.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (Acute) Coronary Syndrome (2021) | https://jts.health.mil/assets/docs/cpgs/Acute_Coronary_Syndrome_14_May_2021_ID86.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (War) Wounds: Wound Debridement and Irrigation (2021) | https://jts.health.mil/assets/docs/cpgs/War_Wounds_Debridement_and_Irrigation_27_Sep_2021_ID31.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Anesthesia for Trauma Patients (2021) | https://jts.health.mil/assets/docs/cpgs/Anesthesia_for_Trauma_Patients_05_Apr_2021_ID40.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Eye Trauma: Initial Care (2021) | https://jts.health.mil/assets/docs/cpgs/Eye_Trauma_Initial_Care_01_Jun_2021_ID03.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Infection Prevention in Combat-Related Injuries (2021) | https://jts.health.mil/assets/docs/cpgs/Infection_Prevention_in_Combat-related_Injuries_27_Jan_2021_ID24.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Pain, Anxiety and Delirium (2021) | https://jts.health.mil/assets/docs/cpgs/Pain_Anxiety_Delirium_26_Apr_2021_ID29_v1.2.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Prolonged Casualty Care Guidelines (2021) | https://jts.health.mil/assets/docs/cpgs/Prolonged_Casualty_Care_Guidelines_21_Dec_2021_ID91.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (Prehospital) Blood Transfusion (2020) | https://jts.health.mil/assets/docs/cpgs/Prehospital_Blood_Transfusion_30_Oct_2020_ID82.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Blunt Abdominal Trauma, Splenectomy, and Post-Splenectomy Vaccination (2020) | https://jts.health.mil/assets/docs/cpgs/Blunt_Abdominal_Trauma_Splenectomy_Vaccination_13_May_2020_ID09.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Cervical and Thoracolumbar Spine Injury Evaluation, Transport and Surgery in the Deployed Setting (2020) | https://jts.health.mil/assets/docs/cpgs/Cervical_Thoracolumbar_Spine_Injury_19_Jun_2020_ID15.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Documentation Requirements for Combat Casualty Care (2020) | https://jts.health.mil/assets/docs/cpgs/Documentation_Requirements_for_Combat_Casualty_Care_18_Sep_2020_ID11.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Ocular Evaluation and Disposition after Suspected Laser Exposure (2020) | https://jts.health.mil/assets/docs/cpgs/Ocular_Evaluation_Disposition_After_Laser_Exposure_14_Feb_2020_ID79.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Sepsis Management in Prolonged Field Care (2020) | https://jts.health.mil/assets/docs/cpgs/Sepsis_Management_PFC_28_Oct_2020_ID83.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Ventilator Associated Pneumonia (VAP) (2020) | https://jts.health.mil/assets/docs/cpgs/Ventilator_Associated_Pneumonia_(VAP)_07_May_2020_ID45.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (Austere) Resuscitative Surgical Care (2019) | https://jts.health.mil/assets/docs/cpgs/Austere_Resuscitative_Surgical_Care_30_Oct_2019_ID76.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Damage Control Resuscitation (2019) | https://jts.health.mil/assets/docs/cpgs/Damage_Control_Resuscitation_12_Jul_2019_ID18.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Transfusion for MWD (2019) | https://jts.health.mil/assets/docs/cpgs/Transfusion_in_Military_Working_Dog_10_Dec_2019_ID77.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | (Wartime) Thoracic Injury (2018) | https://jts.health.mil/assets/docs/cpgs/Wartime_Thoracic_Injury_26_Dec_2018_ID74.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Damage Control Resuscitation (DCR) in Prolonged Field Care (2018) | https://jts.health.mil/assets/docs/cpgs/Damage_Control_Resuscitation_PFC_01_Oct_2018_ID73.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Documentation in Prolonged Field Care (2018) | https://jts.health.mil/assets/docs/cpgs/Documentation_Prolonged_Field_Care_13_Nov_2018_ID72_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Emergency General Surgery in Deployed Locations (2018) | https://jts.health.mil/assets/docs/cpgs/Emergency_General_Surgery_in_Deployed_Locations_01_Aug_2018_ID71.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Emergent Resuscitative Thoracotomy (ERT) (2018) | https://jts.health.mil/assets/docs/cpgs/Emergent_Resuscitative_Thoracotomy_ERT_18_Jul_2018_ID20.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Military Working Dogs Clinical Management (2018) | https://jts.health.mil/assets/docs/cpgs/MWD_CPG_12_Dec_2018_ID16_v1.8.pdf | open | excluded(veterinary; not human patient care) | veterinary |
| Defense Health Agency Joint Trauma System | Whole Blood Transfusion (2018) | https://jts.health.mil/assets/docs/cpgs/Whole_Blood_Transfusion_15_May_2018_ID21.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (Acute Traumatic) Wound Management in the Prolonged Field Care Setting (2017) | https://jts.health.mil/assets/docs/cpgs/Wound_Management_PFC_24_Jul_2017_ID62.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | (Acute) Respiratory Failure (2017) | https://jts.health.mil/assets/docs/cpgs/Acute_Respiratory_Failure_23_Jan_2017_ID06_v1.1.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Analgesia and Sedation Management During Prolonged Field Care (2017) | https://jts.health.mil/assets/docs/cpgs/Analgesia_and_Sedation_Management_during_PFC_11_May_2017_ID61.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Burn Wound Management in Prolonged Field Care (2017) | https://jts.health.mil/assets/docs/cpgs/Burn_Management_PFC_13_Jan_2017_ID57.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Catastrophic Non-Survivable Brain Injury (2017) | https://jts.health.mil/assets/docs/cpgs/Catastrophic_Non-Survivable_Brain_Injury_27_Jan_2017_ID13.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Frostbite and Immersion Foot Care (2017) | https://jts.health.mil/assets/docs/cpgs/Frostbite_and_Immersion_Foot_Care_26_Jan_2017_ID_59.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Ocular Injuries and Vision-Threatening Conditions in Prolonged Field Care (2017) | https://jts.health.mil/assets/docs/cpgs/Ocular_Injuries_Vision-Threatening_Conditions_PFC_01_Dec_2017_ID66.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Radiology: Imaging Trauma Patients in a Deployed Setting (2017) | https://jts.health.mil/assets/docs/cpgs/Radiology_Imaging_Trauma_Patients_in_Deployed_Setting_13_Mar_2017_ID01.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Traumatic Brain Injury Management in Prolonged Field Care (2017) | https://jts.health.mil/assets/docs/cpgs/Traumatic_Brain_Injury_PFC_06_Dec_2017_ID63.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Unexploded Ordnance (UXO) Management (2017) | https://jts.health.mil/assets/docs/cpgs/Unexploded_Ordnance_(UXO)_Management_14_Mar_2017_ID41.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Crush Syndrome - Prolonged Field Care (2016) | https://jts.health.mil/assets/docs/cpgs/Crush_Syndrome_PFC_28_Dec_2016_ID58.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Inhalation Injury and Toxic Industrial Chemical Exposure (2016) | https://jts.health.mil/assets/docs/cpgs/Inhalation_Injury_Toxic_and_Industrial_Chemical_Exposure_26_Jul_2016_ID25.pdf | open | unqueued | - |
| Defense Health Agency Joint Trauma System | Nutritional Support Using Enteral and Parenteral Methods (2016) | https://jts.health.mil/assets/docs/cpgs/Nutrition_Using_Enteral_and_Parenteral_Methods_04_Aug_2016_ID33.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | BOP Health Services National Formulary: Formulary Report (2026) | https://www.bop.gov/resources/pdfs/winter_2026_formulary_report.pdf | open | excluded(administrative) | formulary change report; no patient-care recommendations |
| Federal Bureau of Prisons Health Services Division | BOP National Formulary (2026) | https://www.bop.gov/resources/pdfs/winter_2026_national_formulary.pdf | open | excluded(administrative) | drug-list formulary; no patient-care recommendations |
| Federal Bureau of Prisons Health Services Division | Care Level Classifications for Medical and Mental Health Conditions or Disabilities (2025) | https://www.bop.gov/resources/pdfs/care_level_classification_guidance_102025_final.pdf | open | excluded(administrative; classifies patients for institution assignment, not patient-care recommendations) | - |
| Federal Bureau of Prisons Health Services Division | Community Resources for Releasing Adults in Custody (2025) | https://www.bop.gov/resources/pdfs/community_resources_for_releasing_offenders_revised.pdf | open | excluded(administrative; directory of external community resources) | continuity-of-care recommendations |
| Federal Bureau of Prisons Health Services Division | Federal Bureau of Prisons Clinical Guidance: Immunization (2025) | https://www.bop.gov/resources/pdfs/immunization_cg_dec_2025_final.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Guidance for Medical Diets Clinical Guidance (2024) | https://www.bop.gov/resources/pdfs/medical_diets_final_dec_2024.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Opioid Use Disorder Clinical Guidance (2024) | https://www.bop.gov/resources/pdfs/combined_oud_modules_352025.pdf | open | unqueued | index label year differs from filename |
| Federal Bureau of Prisons Health Services Division | Preventive Health Care (2024) | https://www.bop.gov/resources/pdfs/preventive_health_care_december_2024.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Respiratory Communicable Illness Clinical Management (2024) | https://www.bop.gov/resources/pdfs/RespiratoryCommunicableIllnessClinicalManagement2024.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Antimicrobial Stewardship Clinical Guidance (2022) | https://www.bop.gov/resources/pdfs/antimicrobial_stewardship_cg.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Poliovirus Clinical Guidance (2022) | https://www.bop.gov/resources/pdfs/poliovirus_cg.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Adult Attention Deficit/Hyperactivity Disorder (ADHD) (2021) | https://www.bop.gov/resources/pdfs/adult_adhd_cd.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Hepatitis C (2021) | https://www.bop.gov/resources/pdfs/hcv_guidance.20210513.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Lipid Management (2021) | https://www.bop.gov/resources/pdfs/Lipid_Management.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Re-Entry Medication Acquisition Guide (2021) | https://www.bop.gov/resources/pdfs/re-entry-medication-acquisition-guide.pdf | open | excluded(administrative; directory of medication assistance programs) | continuity-of-care recommendations |
| Federal Bureau of Prisons Health Services Division | Scabies (2020) | https://www.bop.gov/resources/pdfs/scabies_cg.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Tuberculosis (2020) | https://www.bop.gov/resources/pdfs/TB_CPG.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Withdrawal for Inmates with Substance Use Disorder (2020) | https://www.bop.gov/resources/pdfs/medically_supervised_withdrawal_cg.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Compassionate Release Criteria for Elderly Inmates with Medical Conditions (2019) | https://www.bop.gov/resources/pdfs/2019_compassionate_release_cpg.pdf | open | excluded(administrative; medical criteria for a sentence-reduction determination) | - |
| Federal Bureau of Prisons Health Services Division | Hepatitis A (2019) | https://www.bop.gov/resources/pdfs/hepatitis_a_cpg_112019.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Measles (2019) | https://www.bop.gov/resources/pdfs/measles_cpg_20191113.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Anticoagulation Protocol (2018) | https://www.bop.gov/resources/pdfs/anticoagulationprotocol2018.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Durable Medical Equipment (2018) | https://www.bop.gov/resources/pdfs/durable_medical_equipment_cpg.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Infection Control and Environment of Care in Dental Health-Care Settings (2018) | https://www.bop.gov/resources/pdfs/infection_control_in_dental_healthcare_guidance.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Ophthalmology Guidance (2018) | https://www.bop.gov/resources/pdfs/ophthalmology_guidance201810.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Pain Management of Federal Inmates (2018) | https://www.bop.gov/resources/pdfs/pain_mgmt_inmates_cpg.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Acne Guidance (2017) | https://www.bop.gov/resources/pdfs/acne_guidance_2017.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Diabetes (2017) | https://www.bop.gov/resources/pdfs/201703_diabetes.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Heat-Related Illness (2017) | https://www.bop.gov/resources/pdfs/heat_related_illness.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Management of Food Allergies (2017) | https://www.bop.gov/resources/pdfs/food_allergy_guidance_201711.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Medical Management of Exposures: HIV, HBV, HCV, Human Bites, and Sexual Exposures (2017) | https://www.bop.gov/resources/pdfs/201704_medical_mgmt.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Zika Virus (2017) | https://www.bop.gov/resources/pdfs/zika2017.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Responding to Foodborne Illness Outbreaks (2016) | https://www.bop.gov/resources/pdfs/foodborne_illness.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Varicella Zoster Virus Infections (2016) | https://www.bop.gov/resources/pdfs/varicella2016.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Hypertension (2015) | https://www.bop.gov/resources/pdfs/hypertension_2015.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Interim Ebola Protocol: Inmate Screening and Management (2015) | https://www.bop.gov/resources/pdfs/BOP_Ebola_Protocol_2015.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Depression (2014) | https://www.bop.gov/resources/pdfs/depression.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Lice Protocol (2014) | https://www.bop.gov/resources/pdfs/lice_protocol.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Medical Management of Malnutrition (Undernutrition) (2014) | https://www.bop.gov/resources/pdfs/malnutrition_cpg2.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Prevention and Management of Acute and Chronic Wounds (2014) | https://www.bop.gov/resources/pdfs/wounds.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Asthma (2013) | https://www.bop.gov/resources/pdfs/asthma_cpg.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Nutrition Management After Bariatric Surgery (2013) | https://www.bop.gov/resources/pdfs/2018_nutrition_mgmt_after_bariatric.pdf | open | unqueued | index label year differs from filename |
| Federal Bureau of Prisons Health Services Division | Pandemic Influenza Plan — Module 1: Surveillance and Infection Control (2012) | https://www.bop.gov/resources/pdfs/pan_flu_module_1.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Pandemic Influenza Plan — Module 2: Antiviral Medications and Vaccines (2012) | https://www.bop.gov/resources/pdfs/pan_flu_module_2.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Pandemic Influenza Plan — Module 3: Health Care Delivery (2012) | https://www.bop.gov/resources/pdfs/pan_flu_module_3.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Pandemic Influenza Plan — Module 4: Care for the Deceased (2012) | https://www.bop.gov/resources/pdfs/pan_flu_module_4.pdf | open | excluded(administrative; contingency operations for the deceased, not patient care) | - |
| Federal Bureau of Prisons Health Services Division | Federal Bureau of Prisons Report on Infectious Disease Management (2001) | https://www.bop.gov/resources/pdfs/report.pdf | open | excluded(administrative) | program report summarizing policies and guidelines |
| Health Resources and Services Administration | Screening for Cervical Cancer (2027; pub 2025) | https://www.womenspreventivehealth.org/recommendations/cervical-cancer/ | open | unqueued | WPSI-developed; HRSA-adopted 2025-12; effective plan years 2027; living |
| Health Resources and Services Administration | Breast Cancer Screening for Women at Average Risk (2024) | https://www.womenspreventivehealth.org/recommendations/breast-cancer | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Patient Navigation Services for Breast and Cervical Cancer Screening (2024) | https://www.womenspreventivehealth.org/recommendations/patient-navigation-services-for-breast-and-cervical-cancer-screening | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Screening and Counseling for Intimate Partner and Domestic Violence (2024) | https://www.womenspreventivehealth.org/recommendations/intimate-partner-and-domestic-violence/ | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Screening for Urinary Incontinence (2024) | https://www.womenspreventivehealth.org/recommendations/urinary-incontinence/ | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Screening for Diabetes after Pregnancy (2023) | https://www.womenspreventivehealth.org/recommendations/diabetes-after-pregnancy/ | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Screening for Diabetes in Pregnancy (2023) | https://www.womenspreventivehealth.org/recommendations/diabetes-in-pregnancy | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Breastfeeding Services and Supplies (2022) | https://www.womenspreventivehealth.org/recommendations/breastfeeding-services-and-supplies | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Contraception (2022) | https://www.womenspreventivehealth.org/recommendations/contraception/ | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Counseling for Sexually Transmitted Infections (STIs) (2022) | https://www.womenspreventivehealth.org/recommendations/sexually-transmitted-infections/ | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Human Immunodeficiency Virus Infection (HIV) (2022) | https://www.womenspreventivehealth.org/recommendations/human-immunodeficiency-virus-infection | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Obesity Prevention in Midlife Women (2022) | https://www.womenspreventivehealth.org/recommendations/preventing-obesity-in-midlife-women/ | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Well-Woman Preventive Visits (2022) | https://www.womenspreventivehealth.org/recommendations/well-woman-preventive-visits | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Health Resources and Services Administration | Screening for Anxiety (2020) | https://www.womenspreventivehealth.org/recommendations/screening-for-anxiety/ | open | provisional(year unresolved: living WPSI recommendation, no version year on the artifact, the HRSA index, or the WPSI recommendations table) | WPSI-developed; HRSA-adopted; living |
| Healthcare Infection Control Practices Advisory Committee | Infection Prevention and Control in Healthcare Settings: Mpox (2026) | https://www.cdc.gov/monkeypox/hcp/infection-control/healthcare-settings.html | open | unqueued | living; update 2026-02-09; repaired broken index URL |
| Healthcare Infection Control Practices Advisory Committee | Influenza Infection Control in Health Care Facilities (2026) | https://www.cdc.gov/flu/hcp/infection-control/index.html | open | unqueued | living; update 2026-05-08; repaired legacy URL |
| Healthcare Infection Control Practices Advisory Committee | 2025 US Public Health Service Guidelines for the Management of Occupational Exposures to Human Immunodeficiency Virus and Recommendations for Postexposure Prophylaxis in Healthcare Settings (2025) | https://stacks.cdc.gov/view/cdc/183609/cdc_183609_DS1.pdf | open | unqueued | replaces 2013 guidance linked by index |
| Healthcare Infection Control Practices Advisory Committee | Infection Control in Healthcare Personnel: Epidemiology and Control of Selected Infections Transmitted Among Healthcare Personnel and Patients (2025) | https://www.cdc.gov/infection-control/hcp/healthcare-personnel-epidemiology-control/index.html | open | unqueued | living; update 2025-01-31 |
| Healthcare Infection Control Practices Advisory Committee | MDRO Prevention Strategies (2025) | https://www.cdc.gov/healthcare-associated-infections/php/preventing-mdros/mdro-prevention-strategies.html | open | unqueued | living; update 2025-02-11 |
| Healthcare Infection Control Practices Advisory Committee | Preventing Transmission of Viral Respiratory Pathogens in Healthcare Settings (2025) | https://www.cdc.gov/infection-control/hcp/viral-respiratory-prevention/index.html | open | unqueued | living; update 2025-05-21 |
| Healthcare Infection Control Practices Advisory Committee | MDRO Containment Strategy (2024) | https://www.cdc.gov/healthcare-associated-infections/php/preventing-mdros/mdro-containment-strategy.html | open | unqueued | living; update 2024-03-19 |
| Healthcare Infection Control Practices Advisory Committee | Measles: Infection Control in Healthcare Personnel (2024) | https://www.cdc.gov/infection-control/hcp/healthcare-personnel-epidemiology-control/measles.html | open | unqueued | living; replaces indexed retired interim page |
| Healthcare Infection Control Practices Advisory Committee | Prevention and Control for Hospitalized MERS Patients (2024) | https://www.cdc.gov/mers/hcp/infection-control/index.html | open | unqueued | living; update 2024-05-23 |
| Healthcare Infection Control Practices Advisory Committee | Standard Precautions for All Patient Care (2024) | https://www.cdc.gov/infection-control/hcp/basics/standard-precautions.html | open | unqueued | living; update 2024-02-12 |
| Healthcare Infection Control Practices Advisory Committee | Transmission-Based Precautions (2024) | https://www.cdc.gov/infection-control/hcp/basics/transmission-based-precautions.html | open | unqueued | living; update 2024-02-12 |
| Healthcare Infection Control Practices Advisory Committee | Infection Control Guidance: SARS-CoV-2 (2023) | https://www.cdc.gov/covid/hcp/infection-control/index.html | open | unqueued | living; update 2023-05-08; replaces indexed legacy URL |
| Healthcare Infection Control Practices Advisory Committee | CDC's Core Infection Prevention and Control Practices for Safe Healthcare Delivery in All Settings (2022) | https://www.cdc.gov/infection-control/hcp/core-practices/index.html | open | unqueued | HICPAC-approved 2014; reviewed and updated 2022-10; living |
| Healthcare Infection Control Practices Advisory Committee | Guideline for the Prevention and Control of Infections in Neonatal Intensive Care Unit Patients: Central Line-Associated Bloodstream Infections (2022) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-NICU-CLABSI-508.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Interim Guidance for Infection Control Within Healthcare Settings When Caring for Confirmed Cases, Probable Cases, and Cases Under Investigation for Infection with Novel Influenza A Viruses Associated with Severe Disease (2022) | https://www.cdc.gov/bird-flu/hcp/novel-flu-infection-control/index.html | open | unqueued | living; repaired legacy URL |
| Healthcare Infection Control Practices Advisory Committee | Consideration for Use of Enhanced Barrier Precautions in Skilled Nursing Facilities (2021) | https://www.cdc.gov/infection-control/media/pdfs/EnhancedBarrierPrecautions-508.pdf | open | excluded(no systematically developed recommendations) | index section=guideline reviews/additional products; expert-opinion white paper |
| Healthcare Infection Control Practices Advisory Committee | Assessing Solid Organ Donors and Monitoring Transplant Recipients for Human Immunodeficiency Virus, Hepatitis B Virus, and Hepatitis C Virus Infection — U.S. Public Health Service Guideline (2020) | https://www.cdc.gov/mmwr/volumes/69/rr/rr6904a1.htm | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Clostridioides difficile in Neonatal Intensive Care Unit Patients: A Systematic Review (2020) | https://www.cdc.gov/infection-control/media/pdfs/Cdiff-NICU-508.pdf | open | excluded(evidence review without recommendations) | index section=guideline reviews/systematic reviews |
| Healthcare Infection Control Practices Advisory Committee | Recommendations for Prevention and Control of Infections in Neonatal Intensive Care Unit Patients: Staphylococcus aureus (2020) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-NICU-Saureus-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Testing and Clinical Management of Health Care Personnel Potentially Exposed to Hepatitis C Virus — CDC Guidance, United States, 2020 (2020) | https://www.cdc.gov/mmwr/volumes/69/rr/rr6906a1.htm | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | A Process for Assessing Products for Infection Prevention in Healthcare Settings: A Framework From HICPAC (2019) | https://www.cdc.gov/infection-control/media/pdfs/product-assessment-508.pdf | open | excluded(administrative) | index section=guideline reviews/additional products; methodology |
| Healthcare Infection Control Practices Advisory Committee | Infection Control in Healthcare Personnel: Infrastructure and Routine Practices for Occupational Infection Prevention and Control Services (2019) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-Infection-Control-HCP-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Tuberculosis Infection Control (2019) | https://www.cdc.gov/tb-healthcare-settings/hcp/infection-control/index.html | open | unqueued | living; update 2019-05-14 |
| Healthcare Infection Control Practices Advisory Committee | Update to the CDC and HICPAC Recommendation Categorization Scheme for Infection Control and Prevention Guideline Recommendations (2019) | https://www.cdc.gov/infection-control/media/pdfs/recommendation-scheme-update-508.pdf | open | excluded(administrative) | index section=guideline reviews/additional products; methodology |
| Healthcare Infection Control Practices Advisory Committee | Analysis and Recommendations on NHSN Clostridioides difficile Outcome Measures (2018) | https://www.cdc.gov/infection-control/media/pdfs/NHSN-Cdiff-508.pdf | open | excluded(administrative) | index section=guideline reviews/additional products |
| Healthcare Infection Control Practices Advisory Committee | Recommendation on NHSN CAUTI Definition Age Specification for Fever (2018) | https://www.cdc.gov/infection-control/media/pdfs/NHSN-CAUTI-Change-508.pdf | open | excluded(administrative) | index section=guideline reviews/additional products |
| Healthcare Infection Control Practices Advisory Committee | Centers for Disease Control and Prevention Guideline for the Prevention of Surgical Site Infection, 2017 (2017) | https://www.cdc.gov/infection-control/hcp/surgical-site-infection/index.html | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | 2016 Update to the 2001 CDC Hemodialysis Recommendations (2016) | https://www.cdc.gov/dialysis-safety/hcp/recommendations-resources/index.html | open | unqueued | living; updates selected 2001 recommendations |
| Healthcare Infection Control Practices Advisory Committee | Summary of Infection Prevention Practices in Dental Settings: Basic Expectations for Safe Care (2016) | https://www.cdc.gov/dental-infection-control/media/pdfs/2024/07/safe-care2.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | CDC Guidance for Evaluating Health-Care Personnel for Hepatitis B Virus Protection and for Administering Postexposure Management (2013) | https://www.cdc.gov/mmwr/pdf/rr/rr6210.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Vaccination of Dialysis Patients and Patients with Chronic Kidney Disease (2013) | https://stacks.cdc.gov/view/cdc/143156/cdc_143156_DS1.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Updated CDC Recommendations for the Management of Hepatitis B Virus-Infected Health-Care Providers and Students (2012) | https://www.cdc.gov/mmwr/pdf/rr/rr6103.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Basic Infection Control and Prevention Plan for Outpatient Oncology Settings (2011) | https://www.cdc.gov/healthcare-associated-infections/hcp/prevention-healthcare/infection-control-outpatient-oncology.html | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Guideline for the Prevention and Control of Norovirus Gastroenteritis Outbreaks in Healthcare Settings (2011) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-Norovirus-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Guidelines for the Prevention of Intravascular Catheter-Related Infections (2011) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-BSI-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Immunization of Health-Care Personnel: Recommendations of the Advisory Committee on Immunization Practices (ACIP) (2011) | https://www.cdc.gov/mmwr/pdf/rr/rr6007.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Guideline for Prevention of Catheter-Associated Urinary Tract Infections (2009) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-CAUTI-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Guideline for Disinfection and Sterilization in Healthcare Facilities (2008) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-Disinfection-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Infection Control Requirements for Dialysis Facilities: Clarification Regarding Guidance on Parenteral Medication Vials (2008) | https://www.cdc.gov/mmwr/preview/mmwrhtml/mm5732a3.htm | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Guideline for Isolation Precautions: Preventing Transmission of Infectious Agents in Healthcare Settings (2007) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-Isolation-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Management of Multidrug-Resistant Organisms in Healthcare Settings (2006) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-MDRO-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Guidance on Public Reporting of Healthcare-Associated Infections: Recommendations of the Healthcare Infection Control Practices Advisory Committee (2005) | https://doi.org/10.1016/j.ajic.2005.04.001 | open | excluded(administrative) | index section=guideline reviews/recommendations and guidances |
| Healthcare Infection Control Practices Advisory Committee | Guidelines for Environmental Infection Control in Health-Care Facilities (2003) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-Environmental-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Guidelines for Preventing Health-Care-Associated Pneumonia (2003) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-Healthcare-Associated-Pneumonia-H.pdf | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Guideline for Hand Hygiene in Health-Care Settings (2002) | https://www.cdc.gov/infection-control/media/pdfs/Guideline-Hand-Hygiene-P.pdf | open | unqueued | - |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | Guidelines for the Prevention and Treatment of Opportunistic Infections in Adults and Adolescents With HIV (2026) | https://clinicalinfo.hiv.gov/en/guidelines/hiv-clinical-guidelines-adult-and-adolescent-opportunistic-infections | open | unqueued | living; update 2026-05-27 |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | Guidelines for the Prevention and Treatment of Opportunistic Infections in Children With and Exposed to HIV (2026) | https://clinicalinfo.hiv.gov/en/guidelines/hiv-clinical-guidelines-pediatric-opportunistic-infections | open | unqueued | living; update 2026-04-23 |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | Guidelines for the Use of Antiretroviral Agents in Adults and Adolescents With HIV (2026) | https://clinicalinfo.hiv.gov/en/guidelines/hiv-clinical-guidelines-adult-and-adolescent-arv | open | unqueued | living; update 2026-05-27 |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | Guidelines for the Use of Antiretroviral Agents in Pediatric HIV Infection (2026) | https://clinicalinfo.hiv.gov/en/guidelines/pediatric-arv | open | unqueued | living; update 2026-06-25 |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | Recommendations for the Use of Antiretroviral Drugs During Pregnancy and Interventions to Reduce Perinatal HIV Transmission in the United States (2026) | https://clinicalinfo.hiv.gov/en/guidelines/perinatal | open | unqueued | living; update 2026-06-25 |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | Guidance for Caring for People With HIV Displaced by Disasters (2024) | https://clinicalinfo.hiv.gov/en/guidelines/guidance-caring-people-hiv-displaced-disasters | open | unqueued | living; update 2024-10-11 |
| HHS Office of Population Affairs | Providing Quality Family Planning Services in the United States: Recommendations of the U.S. Office of Population Affairs (Revised 2024) (2024) | https://doi.org/10.1016/j.amepre.2024.09.007 | open | unqueued | - |
| Indian Health Service | Standards of Care and Resources for Type 2 Diabetes (2026) | https://www.ihs.gov/diabetes/clinician-resources/soc/ | open | unqueued | living |
| National Asthma Education and Prevention Program | 2020 Focused Updates to the Asthma Management Guidelines (2020) | https://www.nhlbi.nih.gov/sites/default/files/publications/AsthmaManagementGuidelinesReport-2-4-21.pdf | open | unqueued | - |
| National Institute of Allergy and Infectious Diseases expert-panel guidelines | Addendum Guidelines for the Prevention of Peanut Allergy in the United States: Report of the NIAID-Sponsored Expert Panel (2017) | https://pmc.ncbi.nlm.nih.gov/articles/PMC5226648/ | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | Advisory: The Substance Use Disorder Counseling Competency Framework: An Overview (2021) | https://library.samhsa.gov/sites/default/files/pep20-02-01-017.pdf | open | excluded(no patient-care recommendations) | - |
| Substance Abuse and Mental Health Services Administration | TIP 39: Substance Use Disorder Treatment and Family Therapy (2020) | https://www.ncbi.nlm.nih.gov/books/NBK571080/?report=reader | open | unqueued | 2021 SAMHSA Advisory derivative collapsed |
| Substance Abuse and Mental Health Services Administration | TIP 35: Enhancing Motivation for Change in Substance Use Disorder Treatment (2019) | https://library.samhsa.gov/sites/default/files/tip-35-pep19-02-01-003.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | Brief Interventions and Therapies for Substance Abuse (2015) | https://library.samhsa.gov/sites/default/files/sma15-3601.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | Continuity of Offender Treatment for Substance Use Disorder from Institution to Community (2015) | https://library.samhsa.gov/sites/default/files/sma15-3594.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 27: Comprehensive Case Management for Substance Abuse Treatment (2015) | https://library.samhsa.gov/sites/default/files/sma15-4215.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 41: Substance Abuse Treatment: Group Therapy (2015) | https://library.samhsa.gov/sites/default/files/sma15-3991.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 50: Addressing Suicidal Thoughts and Behaviors in Substance Abuse Treatment (2015) | https://library.samhsa.gov/sites/default/files/sma15-4381.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 60: Using Technology-Based Therapeutic Tools in Behavioral Health Services (2015) | https://library.samhsa.gov/sites/default/files/sma15-4924.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | Addressing the Specific Behavioral Health Needs of Men (2014) | https://library.samhsa.gov/sites/default/files/sma14-4881.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 52: Clinical Supervision and Professional Development of the Substance Abuse Counselor (2014) | https://library.samhsa.gov/sites/default/files/sma14-4435.pdf | open | excluded(no patient-care recommendations) | - |
| Substance Abuse and Mental Health Services Administration | TIP 57: Trauma-Informed Care in Behavioral Health Services (2014) | https://library.samhsa.gov/sites/default/files/sma14-4816.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | Addressing Viral Hepatitis in People With Substance Use Disorders (2013) | https://library.samhsa.gov/sites/default/files/sma13-4794.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 47: Substance Abuse: Clinical Issues in Intensive Outpatient Treatment (2013) | https://library.samhsa.gov/sites/default/files/sma13-4182.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 48: Managing Depressive Symptoms in Substance Abuse Clients During Early Recovery (2013) | https://library.samhsa.gov/sites/default/files/sma13-4353.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 49: Incorporating Alcohol Pharmacotherapies Into Medical Practice (2013) | https://library.samhsa.gov/sites/default/files/sma13-4380.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | A Guide to Substance Abuse Services for Primary Care Clinicians (2012) | https://library.samhsa.gov/sites/default/files/sma12-3581.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | Medication-Assisted Treatment for Opioid Addiction in Opioid Treatment Programs (2012) | https://library.samhsa.gov/sites/default/files/sma12-4108.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 25: Substance Abuse Treatment and Domestic Violence (2012) | https://library.samhsa.gov/sites/default/files/SMA12-3390_508.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 31: Screening and Assessing Adolescents for Substance Use Disorders (2012) | https://library.samhsa.gov/sites/default/files/sma12-4079.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 36: Substance Abuse Treatment for Persons with Child Abuse and Neglect Issues (2012) | https://library.samhsa.gov/sites/default/files/SMA12-3923_508.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 38: Integrating Substance Abuse Treatment and Vocational Services (2012) | https://library.samhsa.gov/sites/default/files/sma12-4216.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 46: Substance Abuse: Administrative Issues in Outpatient Treatment (2012) | https://library.samhsa.gov/sites/default/files/tip-46-administrative-issues-treatment-sma12-4151.pdf | open | excluded(administrative) | - |
| Substance Abuse and Mental Health Services Administration | TIP 54: Managing Chronic Pain in Adults With or in Recovery From Substance Use Disorders (2012) | https://library.samhsa.gov/sites/default/files/sma13-4671.pdf | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | Tobacco Use Cessation Policies in Substance Abuse Treatment: Administrative Issues (2011) | https://library.samhsa.gov/sites/default/files/sma11-4636admin.pdf | open | excluded(administrative) | - |
| Substance Abuse and Mental Health Services Administration | TIP 51: Substance Abuse Treatment: Addressing the Specific Needs of Women (2009) | https://www.ncbi.nlm.nih.gov/books/NBK83252/ | open | unqueued | KAP Keys quick-reference derivative collapsed into this parent protocol; NCBI Bookshelf NBK83252, HHS Publication (SMA) 13-4426 |
| Substance Abuse and Mental Health Services Administration | TIP 32: Treatment of Adolescents With Substance Use Disorders (1999) | https://files.eric.ed.gov/fulltext/ED443038.pdf | open | unqueued | 2021 SAMHSA Advisory derivative collapsed |
| U.S. Preventive Services Task Force | Intimate Partner Violence and Caregiver Abuse of Older or Vulnerable Adults: Screening (2025) | https://www.uspreventiveservicestaskforce.org/uspstf/recommendation/intimate-partner-violence-and-abuse-of-elderly-and-vulnerable-adults-screening | open | unqueued | - |
| U.S. Public Health Service Tobacco Use and Dependence Guideline Panel | Treating Tobacco Use and Dependence: 2008 Update (2008) | https://www.ahrq.gov/sites/default/files/wysiwyg/professionals/clinicians-providers/guidelines-recommendations/tobacco/clinicians/update/treating_tobacco_use08.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Non-Surgical Management of Hip & Knee Osteoarthritis (2026) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/OA/Osteoarthritis-CPG_2026-Guideline_final_20260618.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for Tobacco Use Treatment (2026) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/tobacco/Tobacco-Cessation-CPG_2026-Guideline_final_20260109.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Adult Overweight and Obesity (2025) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/obesity/OBE-CPG_2025-Guideline_final_20251105.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Chronic Insomnia Disorder and Obstructive Sleep Apnea (2025) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/insomnia/I-OSA-CPG_2025-Guideline_final_20250915.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Primary Care Management of Asthma (2025) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/asthma/Asthma-CPG_2025-Guideline_final_20250528.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Primary Care Management of Chronic Kidney Disease (2025) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/ckd/CKD-CPG_2025-Guideline_final_20250509.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline on Lipid Management for Cardiovascular Disease Risk Reduction (2025; pub 2026) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/lipids/Lipids-CPG_2025-Guideline_final_20260106.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for Assessment and Management of Patients at Risk for Suicide (2024) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/MH/srb/VADOD-CPG-Suicide-Risk-Full-CPG-2024_Final_508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for Management of Stroke Rehabilitation (2024) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/Rehab/stroke/VADOD-2024-Stroke-Rehab-CPG-Full-CPG_final_508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for Rehabilitation of Individuals with Lower Limb Amputation (2024; pub 2025) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/Rehab/amp/LLA-CPG_2024-Guideline_final_20250110.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for Tinnitus (2024) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/tinnitus/VADOD-CPG-Tinnitus-Full-CPG-2024_Final_508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for Management of Bipolar Disorder (2023) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/MH/bd/VA-DOD-CPG-BD-Full-CPGFinal508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for Management of First-Episode Psychosis and Schizophrenia (2023) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/MH/scz/VA-DOD-CPG-Schizophrenia-CPG_Finalv231924.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for Management of Headache (2023) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/pain/headache/VA-DOD-CPG-Headache-Full-CPG.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for Management of Posttraumatic Stress Disorder and Acute Stress Disorder (2023) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/MH/ptsd/VA-DoD-CPG-PTSD-Full-CPG-Edited-111624-V5-81825.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Pregnancy (2023) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/WH/up/VA-DOD-CPG-Pregnancy-Full-CPG_508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Type 2 Diabetes Mellitus (2023) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/diabetes/VADOD-Diabetes-CPG_Final_508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Diagnosis and Treatment of Low Back Pain (2022) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/Pain/lbp/VADODLBPCPGFinal508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Major Depressive Disorder (2022) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/MH/mdd/VADODMDDCPGFinal508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Upper Limb Amputation Rehabilitation (2022) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/Rehab/ULA/VADODULACPG_Final_508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Use of Opioids in the Management of Chronic Pain (2022) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/Pain/cot/VADODOpioidsCPG.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management and Rehabilitation of Post-Acute Mild Traumatic Brain Injury (2021) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/Rehab/mtbi/VADODmTBICPGFinal508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Chronic Multisymptom Illness (2021) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/MR/cmi/VADODCMICPG508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Chronic Obstructive Pulmonary Disease (2021) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/copd/VADODCOPDCPGFinal508.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Substance Use Disorders (2021) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/MH/sud/VA-DoD-SUD-CPG_Final_for-508_v3.pdf | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Diagnosis and Management of Hypertension in the Primary Care Setting (2020) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/htn/VADODHypertensionCPG508Corrected792020.pdf | open | unqueued | - |
| Academy of Nutrition and Dietetics | Adult Weight Management Evidence-Based Nutrition Practice Guideline (2022; pub 2023) | https://www.andeal.org/topic.cfm?menu=5276&cat=6109 | open | provisional(canonical artifact URL unresolved) | EAL topic index; recommendations bundle retrievable anonymously |
| American Academy of Dermatology Association | Clinical Practice Guideline: Reconstruction After Skin Cancer Resection (2021) | https://www.aad.org/member/clinical-quality/guidelines/reconstruction-skin-cancer | open | unqueued | - |
| American Academy of Family Physicians | 2025 Recommended Immunization Schedules: Updated Recommendations From the AAFP (2026) | https://www.aafp.org/pubs/afp/issues/2026/0100/practice-guidelines-2025-immunization-schedule.html | open | unqueued | - |
| American Academy of Neurology | Management of Functional Seizures (2025) | https://www.aan.com/Guidelines/Home/GuidelineDetail/1150 | open | unqueued | - |
| American Academy of Ophthalmology | Primary Open-Angle Glaucoma Preferred Practice Pattern (2025) | https://www.aao.org/education/preferred-practice-pattern/primary-open-angle-glaucoma-ppp | open | unqueued | - |
| American Academy of Otolaryngology—Head and Neck Surgery | Clinical Practice Guideline: Adult Sinusitis Update (2025) | https://www.entnet.org/quality-practice/quality-products/clinical-practice-guidelines/cpg-adult-sinusitis/ | open | unqueued | - |
| American Academy of Pediatric Dentistry | Use of Vital Pulp Therapies in Primary Teeth 2024 (2024) | https://www.aapd.org/research/oral-health-policies--recommendations/vital_pulp_therapies_in_primary_teeth_with_deep_caries_lesions/ | open | unqueued | - |
| American Academy of Pediatrics | Opioid Prescribing for Acute Pain Management in Children and Adolescents in Outpatient Settings (2024) | https://publications.aap.org/pediatrics/article/154/5/e2024068752/199482/Opioid-Prescribing-for-Acute-Pain-Management-in | open | unqueued | - |
| American Academy of Physical Medicine & Rehabilitation | Platelet-Rich Plasma (PRP) for Knee Osteoarthritis (2025) | https://www.aapmr.org/quality-practice/clinical-practice-guidelines/clinical-guidance/platelet-rich-plasma-(prp)-for-knee-osteoarthritis | open | unqueued | - |
| American Association for Respiratory Care | AARC and PALISI Clinical Practice Guideline: Pediatric Critical Asthma (2025) | https://www.aarc.org/white-et-al-2025-aarc-and-palisi-clinical-practice-guideline-pediatric-critical-asthma/ | open | unqueued | - |
| American Association for the Study of Liver Diseases | AASLD Practice Guidance on Acute-on-chronic liver failure and the management of critically ill patients with cirrhosis (2024) | https://doi.org/10.1097/HEP.0000000000000671 | open | unqueued | full text retrievable anonymously via Ovid |
| American Association of Clinical Endocrinology | 2025 AACE Clinical Practice Guideline on Pharmacologic Management of Adults With Dyslipidemia (2025) | https://pro.aace.com/clinical-guidance/2025-aace-clinical-practice-guideline-pharmacologic-management-adults | open | unqueued | - |
| American College of Emergency Physicians | Direct Oral Anticoagulants (2026) | https://www.acep.org/siteassets/sites/acep/media/clinical-policies/final-cp-pdfs/direct-oral-anticoagulant-cp.pdf | open | unqueued | - |
| American College of Radiology | ACR Appropriateness Criteria® Acute Trauma to the Foot (2019) | https://acsearch.acr.org/docs/70546/Narrative/ | open | unqueued | - |
| American College of Rheumatology | 2023 Interstitial Lung Disease Guideline – Screening & Monitoring (2023) | https://assets.contentstack.io/v3/assets/bltee37abb6b278ab2c/bltffeaff36ede96636/interstitial-lung-disease-guideline-screening-monitoring-2023.pdf | open | unqueued | - |
| American College of Surgeons | Best Practices Guidelines: Chest Wall Injuries Management (2025) | https://www.facs.org/media/qdgliayt/2025_tr_bestpracticesguidelines_chest-wall.pdf | open | unqueued | - |
| American Diabetes Association | Standards of Care in Diabetes—2026 (2026) | https://diabetesjournals.org/care/issue/49/Supplement_1 | open | unqueued | - |
| American Epilepsy Society | American Epilepsy Society Clinical Practice Guideline: Infantile Epilepsy (2026) | https://aesnet.org/infantile-epilepsy-guidelines | open | unqueued | - |
| American Gastroenterological Association | AGA Living Clinical Practice Guideline on the Pharmacologic Management of Moderate-to-Severe Crohn’s Disease (2025) | https://gastro.org/clinical-guidance/pharmacological-management-of-moderate-to-severe-crohns-disease/ | open | unqueued | - |
| American Geriatrics Society | American Geriatrics Society 2023 updated AGS Beers Criteria® for potentially inappropriate medication use in older adults (2023) | https://pubmed.ncbi.nlm.nih.gov/37139824 | open | unqueued | - |
| American Psychological Association | Clinical Practice Guideline for the Treatment of Posttraumatic Stress Disorder (PTSD) in Adults (2025) | https://www.apa.org/ptsd-guideline/ptsd.pdf | open | unqueued | - |
| American Society for Colposcopy and Cervical Pathology | ASCCP 2019 Risk-Based Management Consensus Guidelines (2019) | https://asccp.org/guidelines/management-guidelines-enduring-guidelines-process/asccp-2019-risk-based-management-consensus-guidelines/ | open | unqueued | - |
| American Society for Reproductive Medicine | Diagnosis and treatment of luteal phase deficiency: a committee opinion (2026) | https://www.asrm.org/practice-guidance/practice-committee-documents/diagnosis-and-treatment-of-luteal-phase-deciency-a-committee-opinion/ | open | unqueued | - |
| American Society of Addiction Medicine | The Joint Clinical Practice Guideline on Benzodiazepine Tapering: Considerations When Benzodiazepine Risks Outweigh Benefits (2025) | https://downloads.asam.org/sitefinity-production-blobs/docs/default-source/guidelines/benzodiazepine-tapering-2025/bzd-tapering-document---final-approved-version-for-distribution-02-28-25.pdf | open | unqueued | - |
| American Society of Anesthesiologists | 2026 American Society of Anesthesiologists Practice Guideline on Perioperative Pain Management Using Local and Regional Analgesia for Cardiothoracic Surgeries, Mastectomy, and Abdominal Surgeries (2026) | https://pubmed.ncbi.nlm.nih.gov/41363869/ | open | unqueued | - |
| American Society of Colon & Rectal Surgeons | The American Society of Colon and Rectal Surgeons Clinical Practice Guidelines for Preventing Surgical Site Infection (2024) | https://pubmed.ncbi.nlm.nih.gov/39082620/ | open | unqueued | - |
| American Society of Echocardiography | Guidelines for the Prevention of Work-Related Musculoskeletal Disorders for Cardiac Sonographers: Recommendations from the American Society of Echocardiography (2026) | https://www.asecho.org/guideline/prevention-of-work-related-musculoskeletal-disorders-for-cardiac-sonographers/ | open | unqueued | - |
| American Thoracic Society | Pulmonary Rehabilitation for Adults with Chronic Respiratory Disease: An Official American Thoracic Society Clinical Practice Guideline (2023) | https://www.thoracic.org/statements/guideline-implementation-tools/matrix-guidelines-and-derivatives-pulmonary-rehab-in-adults-08-23-23.php | open | unqueued | - |
| Association for the Advancement of Blood & Biotherapies | Red Blood Cell Transfusion: 2023 AABB International Guidelines (2023) | https://jamanetwork.com/journals/jama/fullarticle/2810754 | open | unqueued | - |
| College of American Pathologists | Interpretive Diagnostic Error Reduction — 2026 Update (2026) | https://www.cap.org/cap-guidelines/interpretive-diagnostic-error-reduction/ | open | unqueued | - |
| Infectious Diseases Society of America | 2025 Clinical Practice Guideline Update by IDSA on Complicated Urinary Tract Infections (2025) | https://www.idsociety.org/practice-guideline/complicated-urinary-tract-infections/ | open | unqueued | - |
| North American Society for Pediatric Gastroenterology, Hepatology and Nutrition | Updated joint ESPGHAN/NASPGHAN guidelines for management of Helicobacter pylori infection in children and adolescents (2023) (2024) | https://d1pij0k2lbf86p.cloudfront.net/wp-content/uploads/2024/10/J-pediatr-gastroenterol-nutr-2024-Homan-Updated-joint-ESPGHAN-NASPGHAN-guidelines-for-management-of-Helicobacter.pdf | open | unqueued | co-issued with ESPGHAN (foreign) |
| North American Spine Society | Diagnosis and Treatment of Adults with Osteoporotic Vertebral Compression Fractures (2024) | https://www.spine.org/Portals/0/assets/downloads/ResearchClinicalCare/Guidelines/Osteoporotic-Vertebral-Compression-Fractures.pdf | open | unqueued | - |
| Society of Critical Care Medicine | Surviving Sepsis Campaign: International Guidelines for Management of Sepsis and Septic Shock 2026 (2026) | https://www.sccm.org/clinical-resources/guidelines/guidelines/surviving-sepsis-campaign-international-guidelines-for-management-of-sepsis-and-septic-shock-2026 | open | unqueued | - |
| Society of Family Planning | Telemedicine in family planning care (2026) | https://societyfp.org/clinical_guidances/telemedicine-for-abortion-telemedicine-for-contraception/ | open | unqueued | - |
| Society of Gynecologic Oncology | COVID-19 Vaccination, Treatment Recommendations and Special Considerations for Gynecologic Cancer Patients (2022) | https://www.sgo.org/wp-content/uploads/2022/07/COVID-19-Vaccination-and-Treatment-Recommendations-2.pdf | open | unqueued | currency unverified |
| American Cancer Society | Colorectal Cancer Screening Guideline (2018) | https://www.cancer.org/health-care-professionals/american-cancer-society-prevention-early-detection-guidelines/colorectal-cancer-screening-guidelines.html | open | unqueued | - |
| Brain Trauma Foundation | BTF Guidelines for the Management of Penetrating Traumatic Brain Injury, Second Edition (2025) | https://braintrauma.org/s/4_pTBI_2nd_edition_guidelines.pdf | open | unqueued | - |
| Clinical Pharmacogenetics Implementation Consortium | CPIC Guideline for CYP2D6, CYP2C19, CYP2B6, SLC6A4, and HTR2A Genotypes and Serotonin Reuptake Inhibitor Antidepressants (2023) | https://cpicpgx.org/guidelines/cpic-guideline-for-ssri-and-snri-antidepressants/ | open | unqueued | latest-guideline status unverified |
| National Coalition for Hospice and Palliative Care | Clinical Practice Guidelines for Quality Palliative Care, 4th edition (2018) | https://www.nationalcoalitionhpc.org/wp-content/uploads/2020/07/NCHPC-NCPGuidelines_4thED_web_FINAL.pdf | open | unqueued | - |
| National Kidney Foundation | KDOQI Clinical Practice Guideline for Vascular Access: 2019 Update (2019) | https://www.kidney.org/professionals/guidelines/guidelines_commentaries/vascular-access | open | unqueued | - |
| Healthcare Infection Control Practices Advisory Committee | Public Reporting of Healthcare-Associated Infection Surveillance Data: Recommendations From the Healthcare Infection Control Practices Advisory Committee (2013) | https://doi.org/10.7326/0003-4819-159-9-201311050-00011 | unverified | excluded(administrative) | index section=guideline reviews/recommendations and guidances; anti-bot challenge |
| Substance Abuse and Mental Health Services Administration | Advisory: Evidence-Based, Whole Person Care of Pregnant Women Who Have Opioid Use Disorder (2024) | https://library.samhsa.gov/sites/default/files/whole-person-care-pregnant-women-oud-pep23-02-01-002.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Advisory: Cannabidiol (CBD) Potential Harm, Side Effects, and Unknowns (2023) | https://library.samhsa.gov/sites/default/files/pep22-06-04-003.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Advisory: Digital Therapeutics for Management and Treatment in Behavioral Health (2023) | https://library.samhsa.gov/sites/default/files/pep23-06-00-001.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Advisory: Low Barrier Models of Care for Substance Use Disorders (2023) | https://library.samhsa.gov/sites/default/files/advisory-low-barrier-models-of-care-pep23-02-00-005.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Advisory: Competencies for Supervision in Substance Use Disorder Treatment (2021) | https://library.samhsa.gov/sites/default/files/pep20-02-01-018.pdf | unverified | excluded(no patient-care recommendations) | - |
| Substance Abuse and Mental Health Services Administration | Advisory: Opioid Therapy in Patients With Chronic Noncancer Pain Who Are in Recovery From Substance Use Disorders (2021) | https://library.samhsa.gov/sites/default/files/pep20-02-01-022.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Advisory: Treating Patients with Traumatic Brain Injury (2021) | https://library.samhsa.gov/sites/default/files/pep21-05-03-001.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | TIP 63: Medications for Opioid Use Disorder (2021) | https://library.samhsa.gov/sites/default/files/pep21-02-01-002.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | TIP 42: Substance Use Disorder Treatment for People With Co-Occurring Disorders (2020) | https://library.samhsa.gov/sites/default/files/SAMHSA_Digital_Download/PEP20-02-01-004_Final_508.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | TIP 61: Behavioral Health Services for American Indians and Alaska Natives (2019) | https://library.samhsa.gov/sites/default/files/tip_61_aian_full_document_020419_0.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Advisory: Sublingual and Transmucosal Buprenorphine for Opioid Use Disorder: Review and Update (2016) | https://library.samhsa.gov/sites/default/files/sma16-4938.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Advisory: Adults With Attention Deficit Hyperactivity Disorder and Substance Use Disorders (2015) | https://library.samhsa.gov/sites/default/files/sma15-4925.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Diabetes Care for Clients in Behavioral Health Treatment (2013) | https://library.samhsa.gov/sites/default/files/sma13-4780.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Advisory: An Introduction to Extended-Release Injectable Naltrexone for the Treatment of People with Opioid Dependence (2012) | https://library.samhsa.gov/sites/default/files/sma12-4682.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Prescription Medications: Misuse, Abuse, Dependence, and Addiction (2012) | https://library.samhsa.gov/sites/default/files/sma12-4175.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Clients With Substance Use and Eating Disorders (2011) | https://library.samhsa.gov/sites/default/files/sma10-4617.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Tobacco Use Cessation During Substance Abuse Treatment Counseling (2011) | https://library.samhsa.gov/sites/default/files/sma11-4636clin.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| Substance Abuse and Mental Health Services Administration | Protracted Withdrawal (2010) | https://library.samhsa.gov/sites/default/files/sma10-4554.pdf | unverified | provisional(access unverified: library.samhsa.gov per-IP metering returned 202 with no artifact bytes, so the artifact was never reached) | - |
| American Academy of Allergy, Asthma & Immunology + American College of Allergy, Asthma & Immunology + Clinical Immunology Society | 2025 Inborn errors of immunity practice parameter: Guidance from the Joint Task Force on Practice Parameters, the American Academy of Allergy, Asthma & Immunology (AAAAI), the American College of Allergy, Asthma and Immunology (ACAAI) and the Clinical Immunology Society (CIS) (2026) | https://doi.org/10.1016/j.anai.2025.10.026 | unverified | provisional(publisher access unresolved) | title-year 2025; published 2026 |
| American Academy of Orthopaedic Surgeons | Clinical Practice Guideline for the Management of Ankle Osteoarthritis (2026) | https://www.aaos.org/aaos-home/newsroom/press-releases/aaos-releases-clinical-practice-guideline-for-management-of-ankle-osteoarthritis/ | unverified | provisional(canonical artifact URL unresolved) | press release links the full guideline |
| American College of Cardiology + American Heart Association + American Diabetes Association + American Society of Nephrology | 2026 AHA/ACC/ADA/ASN Guideline for the Prevention, Detection, Evaluation, and Management of Cardiovascular-Kidney-Metabolic Syndrome (2026) | https://www.jacc.org/doi/10.1016/j.jacc.2026.03.056 | paywalled(JACC gate unverified) | unqueued | - |
| American College of Chest Physicians (CHEST) | Biologic Management in Severe Asthma for Adults (2025) | https://journal.chestnet.org/article/S0012-3692(25)05380-2/fulltext | paywalled(CHEST journal gate unverified) | unqueued | - |
| American College of Medical Genetics and Genomics | Phenylalanine hydroxylase deficiency diagnosis and management: A 2023 evidence-based clinical guideline of the American College of Medical Genetics and Genomics (ACMG) (2025) | https://www.sciencedirect.com/science/article/abs/pii/S1098360024002235 | paywalled(ScienceDirect) | unqueued | - |
| American College of Obstetricians and Gynecologists | Diagnosis of Endometriosis: ACOG Clinical Practice Guideline No. 11 (2026) | https://www.acog.org/clinical/clinical-guidance/clinical-practice-guideline/articles/2026/03/diagnosis-of-endometriosis | login(ACOG membership or clinical subscription) | unqueued | - |
| American College of Occupational and Environmental Medicine | Initial Approaches to Treatment (2026) | https://acoem.org/Guidance-and-Position-Statements/Guidelines/Initial-Approaches-to-Treatment | paywalled(MDGuidelines subscription) | unqueued | - |
| American College of Physicians | Pharmacologic Treatments with Lifestyle Modifications in Nonpregnant Adults with Overweight or Obesity in Outpatient Settings (2026) | https://www.acpjournals.org/doi/10.7326/ANNALS-25-02714 | paywalled(ACP Journals gate unverified) | unqueued | - |
| American Dental Association | Evidence-based clinical practice guideline for the pharmacologic management of acute dental pain in adolescents, adults, and older adults (2024) | https://www.ada.org/resources/research/science-and-research-institute/oral-health-topics/acute-dental-pain-management-guidelines | paywalled(JADA) | unqueued | - |
| American Occupational Therapy Association | Nonoperative Management of Osteoarthritis and Rheumatoid Arthritis (2025) | https://doi.org/10.5014/ajot.2025.790510 | login(AOTA member/AJOT subscriber) | unqueued | - |
| American Physical Therapy Association | Physical Therapy Management of Children With Developmental Coordination Disorder: A 2026 Evidence-Based Clinical Practice Guideline (2026) | https://doi.org/10.1097/PEP.0000000000001284 | paywalled(Ovid) | unqueued | issued by APTA Academy of Pediatric Physical Therapy |
| American Psychiatric Association | The American Psychiatric Association Practice Guideline for the Prevention and Treatment of Delirium (2025) | https://www.psychiatry.org/news-room/news-releases/apa-published-updated-guideline-for-delirium | unverified | provisional(canonical artifact URL unresolved) | - |
| American Society for Clinical Pathology + College of American Pathologists + Association for Molecular Pathology + American Society of Clinical Oncology/Association for Clinical Oncology | Molecular Biomarkers for the Evaluation of Colorectal Cancer (2017) | https://doi.org/10.1200/JCO.2016.71.9807 | unverified | provisional(publisher access unresolved) | collaborative |
| American Society for Gastrointestinal Endoscopy | ASGE guideline on endoscopic management of benign and malignant colonic strictures (2026) | https://www.giejournal.org/article/S0016-5107(26)00210-5/fulltext | paywalled(GIE gate unverified) | unqueued | - |
| American Society for Radiation Oncology | Radiation Therapy for Gastric Cancer: An ASTRO Clinical Practice Guideline (2025) | https://www.astro.org/provider-resources/guidelines/clinical-practice-guidelines/gastric-cancer-guideline | unverified | provisional(WAF challenge blocks classification) | - |
| American Society of Breast Surgeons + Society of Breast Imaging + College of American Pathologists | 2025 Guidelines for the Management of Infectious and Inflammatory Lesions of the Breast (2025) | https://jamanetwork.com/journals/jamasurgery/fullarticle/2847015 | paywalled(JAMA Network) | unqueued | collaborative |
| American Society of Clinical Oncology/Association for Clinical Oncology | Breast Cancer Follow-Up and Surveillance After Primary Treatment: ASCO Guideline Update (2026) | https://ascopubs.org/toc/jco/0/ja | unverified | provisional(canonical artifact URL unresolved) | journal TOC |
| American Society of Health-System Pharmacists + Infectious Diseases Society of America + Surgical Infection Society + Society for Healthcare Epidemiology of America | Clinical Practice Guidelines for Antimicrobial Prophylaxis in Surgery (2013) | https://doi.org/10.2146/ajhp120568 | unverified | provisional(publisher access unresolved) | collaborative |
| American Society of Hematology | Acute Lymphoblastic Leukemia in Adolescents and Young Adults (2026) | https://www.hematology.org/education/clinicians/guidelines-and-quality-care/clinical-practice-guidelines/acute-lymphoblastic-leukemia-guidelines | unverified | provisional(canonical artifact URL unresolved) | topic index |
| American Society of Plastic Surgeons | Evidence-Based Clinical Practice Guideline: Reconstruction After Skin Cancer Resection (2021) | https://journals.lww.com/plasreconsurg/Fulltext/2021/05000/Evidence_Based_Clinical_Practice_Guideline_.29.aspx | paywalled(PRS journal) | unqueued | reaffirmed 2026 |
| American Urological Association | Medical Management of Kidney Stones: AUA Guideline (2026) | https://www.auanet.org/guidelines-and-quality/guidelines/medical-management-of-kidney-stones | login(free AUA account) | unqueued | - |
| Association of periOperative Registered Nurses | Guidelines for Perioperative Practice, 2026 Edition (2026) | https://www.aornguidelines.org | paywalled(AORN purchase/eGuidelines Plus subscription) | unqueued | - |
| Obesity Medicine Association | 2026 Obesity Algorithm® (2026) | https://obesitymedicine.org/resources/obesity-algorithm/ | paywalled(OMA purchase/membership) | unqueued | - |
| Oncology Nursing Society | ONS/ASCO Guideline on the Management of Antineoplastic Extravasation (2025) | https://www.ons.org/clinical-tools/pep/extravasation-management | login(free ONS account) | unqueued | - |
| Post-Acute and Long-Term Care Medical Association | Acute Change of Condition CPG (2024) | https://paltmed.org/products/acute-change-condition-cpg | login(PALTmed account; $0 electronic copy) | unqueued | year from official path |
| Society for Vascular Surgery | Society for Vascular Surgery clinical practice guidelines on the management of blunt thoracic aortic injury (2026) | https://www.jvascsurg.org/article/S0741-5214(26)00110-2/fulltext | paywalled(JVS gate unverified) | unqueued | - |
| Society of Interventional Radiology | The Society of Interventional Radiology Practice Guidance Document on Venous-Origin Chronic Pelvic Pain in Women (2026) | https://www.jvir.org/article/S1051-0443(25)00794-8/fulltext | paywalled(JVIR Elsevier/member/institutional gate) | unqueued | - |
| Society of Nuclear Medicine and Molecular Imaging + American College of Nuclear Medicine | SNMMI/EANM/ACNM/ANZSNM Procedure Standard / Procedure Guideline for Ventilation-Perfusion Pulmonary Scintigraphy (2026) | https://sites.snmmi.org/Web/Web/Clinical-Practice/Procedure-Standards/Default.aspx | unverified | provisional(canonical artifact URL unresolved) | foreign co-issuers EANM, ANZSNM |
| Society of Surgical Oncology + American Society of Clinical Oncology/Association for Clinical Oncology | Germline Testing in Patients with Breast Cancer: ASCO-Society of Surgical Oncology Guideline (2024) | https://ascopubs.org/doi/full/10.1200/JCO.23.02225 | paywalled(ASCO Publications gate unverified) | unqueued | - |
| The Society of Thoracic Surgeons | STS Consensus on Surgical Management of Oligometastatic Non-small Cell Lung Cancer (NSCLC) (2025) | https://www.annalsthoracicsurgery.org/article/S0003-4975(24)00960-3/fulltext | paywalled(Annals gate unverified) | unqueued | - |
| Alzheimer's Association | Alzheimer's Association Clinical Practice Guideline on the use of blood-based biomarkers in the diagnostic workup of suspected Alzheimer's disease within specialized care settings (2025) | https://pubmed.ncbi.nlm.nih.gov/40729527/ | unverified | provisional(publisher artifact unresolved; org path returns 404) | - |
| National Comprehensive Cancer Network | NCCN Clinical Practice Guidelines in Oncology: Breast Cancer (2026) | https://www.nccn.org/guidelines/guidelines-detail?category=1&id=1419 | login(free NCCN account) | unqueued | exact version unverified |
