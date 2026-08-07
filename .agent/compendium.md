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
- `swept` = `pending | <date> <method> | blocked(<why>)`; `CPGs=no` rows
  swept before the ruling record that sweep, unswept ones use `-`.

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
- `status` = `unqueued | provisional(<why>) | queued | in-progress |
  done | blocked(<why>) | excluded(<why>)`. `provisional(<why>)` = row
  whose URL, year, or access is unresolved; it never promotes. `done` =
  worked to completion per root `README.md` § Operating. Ruled-out rows
  stay as `excluded(<why>)` — the audit trail of eligibility rulings.
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
| Advisory Committee on Immunization Practices | ACIP | federal | yes | https://www.cdc.gov/acip/vaccine-recommendations/index.html | PubMed PMID 41505372; Immunize.org ACIP index | pending |
| Centers for Disease Control and Prevention | CDC | federal | yes | https://stacks.cdc.gov/cbrowse?parentId=cdc%3A100&pid=cdc%3A100 | GC-publishers; PubMed PMID 36327391; AAFP-PG | pending |
| Defense Health Agency Joint Trauma System | DHA JTS | federal | yes | https://jts.health.mil/index.cfm/CPGs/cpgs | PubMed PMID 34529799; DOI 10.55460/zfqw-dwgr | pending |
| Federal Bureau of Prisons Health Services Division | BOP HSD | federal | yes | https://www.bop.gov/resources/health_care_mngmt.jsp | PubMed PMID 28089415; DOI 10.3201/eid3013.230799 | pending |
| Health Resources and Services Administration | HRSA | federal | yes | https://www.hrsa.gov/womens-guidelines | GC-publishers; PubMed PMID 24112064 | pending |
| Healthcare Infection Control Practices Advisory Committee | HICPAC | federal | yes | https://www.cdc.gov/infection-control/hcp/guidance/index.html | PubMed PMID 28467526; GovInfo GOVPUB-HE20_7000-PURL-gpo136862 | pending |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | ClinicalInfo panels | federal | yes | https://clinicalinfo.hiv.gov/en/guidelines | GC-publishers; PubMed PMID 11365496 | pending |
| HHS Office of Population Affairs | OPA | federal | yes | https://opa.hhs.gov/reproductive-health/quality-family-planning | PubMed PMID 39570204; DOI 10.1016/j.amepre.2024.09.007 | pending |
| Indian Health Service | IHS | federal | yes | https://www.ihs.gov/forproviders/clinicalresources/ | PubMed PMID 16125270; National Academies/NCBI federal-guideline chapter | pending |
| National Asthma Education and Prevention Program | NAEPP | federal | yes | https://www.nhlbi.nih.gov/science/national-asthma-education-and-prevention-program-coordinating-committee-naeppcc | PMC7924476; DOI 10.1016/j.jaci.2020.10.003 | pending |
| National Institute of Allergy and Infectious Diseases expert-panel guidelines | NIAID panels | federal | yes | https://www.niaid.nih.gov/diseases-conditions/food-allergy-guidelines | PubMed PMID 28065278; DOI 10.1016/j.jaci.2016.10.010 | pending |
| Substance Abuse and Mental Health Services Administration | SAMHSA | federal | yes | https://www.samhsa.gov/substance-use/treatment/resources/kap/resources | NCBI Bookshelf NBK572951; DOI 10.1016/j.jsat.2012.01.008 | pending |
| U.S. Preventive Services Task Force | USPSTF | federal | yes | https://www.uspreventiveservicestaskforce.org/uspstf/recommendation-topics | PubMed PMID 40553450; JAMA USPSTF recommendation collection; GC-publishers; AAFP-PG | pending |
| U.S. Public Health Service Tobacco Use and Dependence Guideline Panel | PHS tobacco panel | federal | yes | https://www.ahrq.gov/prevention/guidelines/tobacco/index.html | NCBI Bookshelf NBK63952; DOI 10.1016/j.amepre.2008.04.009 | pending |
| VA/DoD Clinical Practice Guideline Program | VA/DoD CPG | federal | yes | https://www.healthquality.va.gov/guidelines/ | PubMed PMID 39093266; National Academies/NCBI federal-guideline chapter; GC-publishers | pending |
| National Institute for Occupational Safety and Health | NIOSH | federal | unverified | https://www.cdc.gov/niosh/pubs/ | - | pending |
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
| Advisory Committee on Immunization Practices | Use of the GSK MenACWY-CRM/MenB-4C Pentavalent Meningococcal Vaccine Among Persons Aged ≥10 Years: Recommendations of the Advisory Committee on Immunization Practices — United States, 2025 (2025) | https://www.cdc.gov/mmwr/volumes/75/wr/mm7501a2.htm | open | unqueued | published=2026-01-08 |
| Centers for Disease Control and Prevention | CDC Clinical Practice Guideline for Prescribing Opioids for Pain — United States, 2022 (2022) | https://www.cdc.gov/mmwr/volumes/71/rr/rr7103a1.htm | open | in-progress | id=cdc-2022-opioid |
| Defense Health Agency Joint Trauma System | Spider and Scorpion Envenomation (2026) | https://jts.health.mil/assets/docs/cpgs/Spider_and_Scorpion_Envenomation_21_Jul_2026_ID84_v1.pdf | open | unqueued | - |
| Federal Bureau of Prisons Health Services Division | Federal Bureau of Prisons Clinical Guidance: Immunization (2025) | https://www.bop.gov/resources/pdfs/immunization_cg_dec_2025_final.pdf | open | unqueued | - |
| Health Resources and Services Administration | Women’s Preventive Services Guidelines (2026) | https://www.hrsa.gov/womens-guidelines | open | unqueued | WPSI-developed; HRSA-adopted; living |
| Healthcare Infection Control Practices Advisory Committee | Centers for Disease Control and Prevention Guideline for the Prevention of Surgical Site Infection, 2017 (2017) | https://www.cdc.gov/infection-control/hcp/surgical-site-infection/index.html | open | unqueued | - |
| HHS HIV/AIDS guideline panels / NIH ClinicalInfo | Guidelines for the Use of Antiretroviral Agents in Pediatric HIV Infection (2026) | https://clinicalinfo.hiv.gov/en/guidelines/pediatric-arv | open | unqueued | living; update 2026-06-30 |
| HHS Office of Population Affairs | Providing Quality Family Planning Services in the United States: Recommendations of the U.S. Office of Population Affairs (Revised 2024) (2024) | https://doi.org/10.1016/j.amepre.2024.09.007 | open | unqueued | - |
| Indian Health Service | Standards of Care and Resources for Type 2 Diabetes (2026) | https://www.ihs.gov/diabetes/clinician-resources/soc/ | open | unqueued | living |
| National Asthma Education and Prevention Program | 2020 Focused Updates to the Asthma Management Guidelines (2020) | https://www.nhlbi.nih.gov/health-topics/asthma-management-guidelines-2020-updates | open | unqueued | - |
| National Institute of Allergy and Infectious Diseases expert-panel guidelines | Addendum Guidelines for the Prevention of Peanut Allergy in the United States: Report of the NIAID-Sponsored Expert Panel (2017) | https://pmc.ncbi.nlm.nih.gov/articles/PMC5226648/ | open | unqueued | - |
| Substance Abuse and Mental Health Services Administration | TIP 63: Medications for Opioid Use Disorder (2021) | https://library.samhsa.gov/product/tip-63-medications-opioid-use-disorder/pep21-02-01-002 | open | unqueued | - |
| U.S. Preventive Services Task Force | Intimate Partner Violence and Caregiver Abuse of Older or Vulnerable Adults: Screening (2025) | https://www.uspreventiveservicestaskforce.org/uspstf/recommendation/intimate-partner-violence-and-abuse-of-elderly-and-vulnerable-adults-screening | open | unqueued | - |
| U.S. Public Health Service Tobacco Use and Dependence Guideline Panel | Treating Tobacco Use and Dependence: 2008 Update (2008) | https://www.ncbi.nlm.nih.gov/books/NBK63952/ | open | unqueued | - |
| VA/DoD Clinical Practice Guideline Program | VA/DoD Clinical Practice Guideline for the Management of Type 2 Diabetes Mellitus (2023) | https://www.healthquality.va.gov/HEALTHQUALITY/guidelines/CD/diabetes/VADOD-Diabetes-CPG_Final_508.pdf | open | unqueued | - |
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
