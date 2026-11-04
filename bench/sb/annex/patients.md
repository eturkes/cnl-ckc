# Patient profiles (SB1 annex)

`families.tsv.param_ids` resolves every family vector field through `sources.tsv.parameter`. Primary excerpt files preserve raw captured text. TSV bridge quotations collapse layout whitespace and line-wrap hyphenation only. ACE-like patient lines/sketches are source constraints, not compiled/certified fixtures. Unreported, unmeasured and unconfirmed are substantive limits, never successful grading evidence.

## P-adult

Source = `tests/consumer/adult.pl.in:1–7,53–82`; parameters = `P-ADULT-FACTS`, `P-ADULT-AGE`; excerpt = `excerpts/P-ADULT.txt`. Synthetic committed fixture, not an observed patient.

```ace
A clinician prescribes an opioid to a patient.
The patient has an age.
The age includes exactly 45 years.
The patient has a chronic-pain.
```

Use unchanged for F1a/F1b. `k=1` counts the intentionally active CDC shard; F1a's shared-clinician copies can produce nonempty unrelated residue. F1a's oracle must retain that residue rather than call every copied recommendation inactive.

## P-F2

Published hypothetical = Buffel du Vaure et al., BMJ Open 2016, all-six-condition guideline-following scenario. Primary = `https://pmc.ncbi.nlm.nih.gov/articles/PMC4809109/`; supplement = Appendix 1 (profile), Appendix 2 (ten documents). Excerpts = `BMJ2016`, `BMJ2016-SUPP`. Every profile attribute has its `P-F2-*` parameter row.

```text
A male patient is 45–64 years old. No exact age is selected.
The patient has hypertension, type 2 diabetes, stable ischemic heart disease,
COPD, symptomatic knee osteoarthritis, and depression.
The patient smokes and is overweight.
```

Condition-specific source profile (not an independently validated coherent clinical record):
- `P-F2-HYPERTENSION`: Stage II (systolic BP ≥160 mmHg or diastolic BP ≥100 mmHg); source says uncomplicated, without target-organ damage.
- `P-F2-DEPRESSION`: mild to moderate for 3 months; no anxiety association.
- `P-F2-OA`: symptomatic knee osteoarthritis; no current/past upper gastrointestinal problems or CKD; source treatment-response wording retained in the excerpt; overweight.
- `P-F2-CHD`: revascularisation >1 year before; source absent/stable angina conditions ≥60 days; no recent myocardial damage or chronic heart failure; smoker.
- `P-F2-DIABETES`: stabilized oral non-insulin regimen; source says uncomplicated, without target-organ damage.
- `P-F2-COPD`: Group C, high risk/few symptoms; GOLD 3/4 and/or ≥2 exacerbations/year; mMRC 0–1 or CAT<10. GOLD 3 footnote: FEV1/FVC<0.70, predicted FEV1 30–50%.

The article calls all conditions moderate, whereas Appendix 1 explicitly gives severe/very-severe airflow limitation for the COPD profile. Appendix 1 also describes hypertension/diabetes as without target-organ heart damage while the all-six scenario includes CHD; this transcription preserves the exact qualifications and does not adjudicate their clinical consistency. The benchmark uses the source's ten-document count as a mechanism-isolation calibration, not a clinical translation or proof of consistency. No published patient-specific medication regimen, opioid use, exact age, screening history, or present-day treatment advice is invented.

| param_id | Title | Issuer | Year |
| --- | --- | --- | --- |
| `F2-G01` | Hypertension diagnosis and treatment | Institute for Clinical Systems Improvement | 2012 |
| `F2-G02` | Diagnosis and management of type 2 diabetes mellitus in adults | Institute for Clinical Systems Improvement | 2012 |
| `F2-G03` | Management of stable ischemic heart disease: summary of a clinical-practice guideline | American College of Physicians/American College of Cardiology Foundation/American Heart Association/American Association for Thoracic Surgery/Preventive Cardiovascular Nurses Association/Society of Thoracic Surgeons. | 2012 |
| `F2-G04` | Global strategy for the diagnosis, management, and prevention of chronic obstructive pulmonary disease | Global Initiative for Chronic Obstructive Lung Disease | 2013 |
| `F2-G05` | Major depression in adults in primary care | Institute for Clinical Systems Improvement | 2012 |
| `F2-G06` | Recommendations for the use of nonpharmacologic and pharmacologic therapies for osteoarthritis of the hand, hip, and knee | American College of Rheumatology | 2012 |
| `F2-G07` | Tobacco treatment | University of Michigan Health System | 2012 |
| `F2-G08` | Management of overweight and obesity in adults | Michigan Quality Improvement Consortium | 2011 |
| `F2-G09` | Summary of recommendations for clinical prevention | American Academy of Family Physicians | 2013 |
| `F2-G10` | Immunizations | Institute for Clinical Systems Improvement | 2012 |

Ten labels = six condition documents + smoking cessation + overweight + prevention + immunizations (`F2-K`). CDC is not one of the ten. Each label denotes a transformed CDC-sized benchmark shard, not a new faithful compilation of that historical guideline. Shared read-only activation facts/private intensional positions must be fixed by the generator's isolation contract; the paper does not supply that map.

Nested prefixes follow this table. G=1/2/4 are smoke-only. G=10 contains the full named active set. G=100 adds 90 common-role-residue shards. The complete union oracle must retain each isolated contribution; nonempty residue is not a failure of the specified construction.

## P-F3

Source-supported **synthetic** eligibility dossier, not an observed person, prevalence estimate, or clinical interoperability ruling. F3 uses the main-selected four-document opioid/OUD cluster plus two kidney/diabetes hubs (`F3-ACTIVE_DOCUMENTS`, `F3-K`). Titles alone do not create edges. The seven source-supported bridge/link rows are in `bridges.tsv`; all sketches are uncompiled.

```text
The same adult Veteran/VA-or-DoD-eligible patient receives outpatient pain care.
A clinician actually considers prescription opioid therapy for that patient.
A persistent/recurrent chronic pain episode lasts >3 months.
A distinct acute pain episode lasts <1 month; opioids are actually considered.
A problematic opioid-use pattern causes clinically significant impairment or distress.
The patient has a vetted qualifying OUD symptom count ≥2 within 12 months.
Tolerance/withdrawal are not counted when solely under appropriate medical supervision.
The patient has T2DM and the source-defined CKD conditions (>3-month kidney abnormality).
There is no pregnancy/nursing/T1DM positive eligibility claim for the T2DM guideline.
There is no CDC-excluding sickle-cell/cancer pain, palliative/end-of-life care,
or hospital/ED/observation care without the actual discharge-prescription exception.
```

Parameters = `P-F3-ELIGIBILITY`, `P-F3-CHRONIC`, `P-F3-ACUTE`, `P-F3-OUD-COUNT`, `P-F3-OUD-WINDOW`, `P-F3-OUD-EXCEPTION`, `P-F3-TIP-DIAGNOSIS`, `P-F3-CKD`, `P-F3-T2DM`, `P-BRIDGE-SCOPE-*`. Strict inequalities and original source units stay as stated. Numeric exact ages, GFR values, extra symptoms and actual treatment receipt are not invented. The dossier states eligibility constraints; it is not itself compiler-ready patient ACE.

TIP 63 supplies an explicit OUD definition as a possible parallel support (`researcher-6:B2/P32`, `P-F3-TIP-DIAGNOSIS`); its hospital medication-management material is not silently activated for an outpatient. The selected four F3 bridge IDs use the exact main-named feeds. TIP/CDC/VA diagnostic alternative supports must all be accounted for when a contrast tries to remove OUD. CKD/T2DM share documented conditions and medication-safety vocabulary (`researcher-7:C-RENAL/C-GABA/C-OUD`), but no unsupported CKD→every-CDC-renal-label alias is adopted.

Rejected or unconfirmed candidate feeds remain excluded:
- `researcher-5:B3`: screening/assessment duty → positive suicide/behavioral diagnosis: rejected.
- `researcher-5:B4`: OUD → VA opioid R3 SUD or R4 actual medication receipt: not a direct bridge; the subtype/receipt prerequisites are missing in those passages. This is **not** `researcher-6:B4`, the selected VA SUD R16/R17 fan-out.
- `researcher-6:B1,B6–B9`: recommendations/offers/continuation/taper/naloxone prescription → actual diagnosis, receipt, administration or plan: rejected. Shared patient inputs remain separate.
- `researcher-7:B1/B2`: CKD definition → T2DM R22/R23 is source-supported candidate hub connectivity, but not one of the main's four selected F3 bridges; R23 additionally needs independently supplied SGLT2 unsuitability. No extra selected edge is invented here.
- `researcher-7:B3`: target-table renal-insufficiency parenthesis → universal diagnostic rule: unconfirmed, not used.
- `researcher-7:B4–B7`: treatment recommendation → current treatment, NSAID avoidance → factual contraindication, gabapentin example → actual drug/type exposure: rejected as direct feeds.
- `researcher-7:B8`: current pregnancy as an all-three-positive trigger: rejected by T2DM scope.

No 31/33 broad-topic coding fraction becomes a graph density. Main rules `k=6` active shards: the four opioid/OUD cluster members plus the CKD/T2DM hubs. The joint patient includes CKD and T2DM beside pain and qualifying OUD criteria as an **engineering assumption**, grounded in researcher-7's CKD Appendix N pain/OUD/drug-safety overlap; it is not a clinical prevalence or observed-patient claim (`F3-JOINT-ASSUMPTION`, `F3-JOINT-EVIDENCE`). Padding = 94 F1a-kind common-role-residue copies by main ruling. Exact shared head/body signatures, compiled bridge clauses, active-count validation and exclusion absence still require the fixture's own checks; the ruling does not itself prove every selected source rule enables.

## P-F4

Source-supported synthetic adult Veteran outpatient with actual chronic non-cancer pain and actual opioid-therapy consideration; diagnostic inputs = `P-F3-OUD-COUNT`, `P-F3-OUD-WINDOW`, `P-F3-OUD-EXCEPTION`. Age is adult, with no exact age invented. The actual pain prerequisite is independent of the diagnostic chain (`P-F4-PAIN`). Primary texts/excerpts = `VAOP2022`, `ASAM2020`, `CDC-S60-ACE`; detailed scope controls = `P-BRIDGE-SCOPE-*`.

```text
VA Appendix C: vetted symptoms + impairment/distress → patient P has OUD instance D.
ASAM consequence: P has a fresh SUD witness S; P-has-D/OUD remains unchanged.
CDC s60-11: P has actual pain and S typed SUD → document-specific MUST pain management.
```

`F4-DEPTH`: 3 enumerated semantic rules, 2 guideline hops, 3 source guidelines. Compiled support depth, negative strata, term nesting, SCC/frontier/intermediate counts remain **unmeasured**. The ASAM consequence is a patient-level existential (`F4-SUBTYPE`): its fresh SUD witness adds constructor + support work, measured as compiled, never charged as an identity-preserving type-edge cost. The ASAM consequence is a patient-level existential (`F4-SUBTYPE`): its fresh SUD witness adds constructor + support work, measured as compiled, never charged as an identity-preserving type-edge cost. No source justifies a longer serial chain or multiplication of depth by copies. Only F4-L1..L3 define the positive path; a separate pain-classification branch is not concatenated into it.

Controls from `researcher-9:C6`: remove the vetted diagnostic seed; remove actual pain; remove/rename the subtype link while withholding all other SUD producers; add medically supervised tolerance/withdrawal as the only count contributors; add each CDC age/setting/disease/care exclusion separately. Supervision excludes those two symptoms, not every possible OUD diagnosis. A hospital/ED/observation case with no actual discharge-prescription witness blocks CDC's endpoint; no prescription duty can supply that witness.

`F4-SHORTCUT`: CDC s57-02 independently reads OUD and concludes the same clinical kind of ongoing pain management. Therefore a broken-ASAM-link oracle must address the exact s60-11 context/payload identities, not a deduplicated action name. Fact → subtype → MUST ends at a modal statement; it establishes neither pain care delivered nor medication received. The source keeps a 12-month diagnostic window; the report's year-based ACE sketch is an authoring sketch, not permission to rewrite source units silently.

Every scale carries the complete three-fragment chain on its one CDC-sized active shard (G counts capacity shards: main ruling, builder-1 R6). Padding = G-1 F1a-kind common-role-residue copies (99 at G=100) by main ruling; their unrelated contributions stay in the oracle.

## P-F5a

**Mayo Patient 1, kept separate from F5b.** Primary = `https://pmc.ncbi.nlm.nih.gov/articles/PMC10792778/`; excerpt = `excerpts/MAYO2023.txt`; direct values = `P-F5A-AGE/SEX/PAIN/DURATION/RATING/OXYCODONE/MME/TAPER/UDT`.

```text
The patient is a 57-year-old male.
The patient has chronic bilateral anterior thigh/leg pain from inflammatory myositis.
The one published baseline pain duration is 20 years.
The one published baseline pain-intensity value is VRS 5.
At admission, the patient receives up to five oxycodone 5 mg tablets daily.
The reported baseline daily opioid dose is 37.5 MME/day.
The flexible taper completes on program day 15.
Admission and dismissal each have a distinct UDT event.
```

This is an outpatient rehabilitation case. Baseline and later taper/dismissal observations stay on distinct visits/dates. Do not treat a day number as a count of dose changes, or both UDT events as two simultaneously current records.

| Key (researcher-10 row) | Retained degree / value | Evidence status + param_id |
| --- | --- | --- |
| Distinct baseline opioid (K1) | 1 disclosed oxycodone regimen | Source-supported synthetic retained key; `P-F5A-OPIOID-DEGREE`; no complete-medication-list claim |
| Benzodiazepine / gabapentinoid (K2) | unreported; no count or zero asserted | `P-F5A-UNREPORTED`; no F5b medications imported |
| Numeric pain duration (K3) | 1 retained baseline value, 20 years | Value observed; retained multiplicity synthetic; `P-F5A-DURATION`, `P-F5A-DURATION-DEGREE`, `F5-NO-REPEATED-DURATION` |
| Pain intensity (K3) | 1 retained baseline VRS 5 | `P-F5A-RATING`, `P-F5A-RATING-DEGREE`; distinct from duration |
| Baseline daily dose (K4) | 1 retained value, 37.5 MME/day | `P-F5A-MME`, `P-F5A-DOSE-DEGREE`; taper trajectory is historical |
| UDT (K5) | 2 dated events across the episode; 1 per reported visit | `P-F5A-UDT`, `P-F5A-UDT-DEGREE`; no annual cohort frequency imported |
| PDMP (K6) | unreported; no repeated query record generated | `F5-PDMP-SEARCH`, `P-F5A-UNREPORTED`; absence of disclosure is not no check |
| Clinician role | 1 synthetic shared-role witness | `F5-CLINICIAN-DEGREE`; not an observed staff count |

Repeated numeric duration distribution = **none-found**, bounded to the VA LTOT/NRS cohort, VHA pain-quality, PainCAS/flowsheet studies and PMC/PubMed repeated-assessment/current-episode searches (`F5-DURATION-SEARCH`; `researcher-10:K3`). One disclosed baseline is retained; no duration-key amplification is sourced. Annual tests, refill counts, longitudinal NRS averages and computed monthly dose summaries do not create jointly current values (`researcher-10:K3–K6`).

Window = CMS460v2: gaps ≤7 days; ≥90 supply days for its denominator (`F5-WINDOW-*`). The source does **not** provide pre-admission dated opioid supply sufficient to verify that ≥90-day criterion. The 15-program-day taper is not an exposure-history substitute. Keep CMS episode eligibility **unconfirmed**; apply its dated-overlap convention without fabricating dates.

Intake constraints = `F5-RECORD-MAP`: keep people/episodes and the two named visit times distinct; retain each disclosed baseline once. A source statement can expand into several schema facts, but not several independent measurement entities. The exact source-record ID/dedup function, episode-to-schema keys and compiled context IDs/product-oracle values remain **unconfirmed** until main freezes the fixture; this transcription does not invent a key format or repeated-observation policy.

## P-F5b

**Sayles case, kept separate from Mayo Patient 1.** Primary = `https://www.psychiatrist.com/pcc/malignant-catatonia-induced-by-concurrent-opioid-benzodiazepine-withdrawal/`; excerpt = `excerpts/SAYLES2023.txt`. The observed tuple is a **home prescription list before interruption**, not confirmed medication adherence at hospital arrival.

```text
The patient is a 30-year-old man.
The patient has chronic pain secondary to Legg-Calves-Perthes disease.
The home prescriptions jointly list:
  alprazolam 1 mg every 4 hours as needed for anxiety;
  fentanyl 50 micrograms/hour patch every 72 hours;
  oxycodone 30 mg 5 times/day;
  gabapentin 900 mg 3 times/day.
```

Direct source parameters = `P-F5B-AGE/SEX/PAIN/ALPRAZOLAM/FENTANYL/OXYCODONE/GABAPENTIN/RX-STATE`.

| Key (researcher-10 row) | Degree / value | Evidence status + param_id |
| --- | --- | --- |
| Distinct opioids (K1) | 2: fentanyl + oxycodone | Observed enumeration from one same-person prescription sentence; `P-F5B-OPIOID-DEGREE` |
| Benzodiazepine (K2) | 1: alprazolam | Observed enumeration; `P-F5B-BZD-DEGREE` |
| Gabapentinoid (K2) | 1: gabapentin | Observed enumeration; `P-F5B-GABA-DEGREE` |
| Numeric pain duration / intensity (K3) | unreported; one qualitative chronic-pain state only | `P-F5B-UNREPORTED`; no numerical duration entity invented; `F5-NO-REPEATED-DURATION` does not license one |
| Total daily MME / observed dose trajectory (K4) | unreported | `P-F5B-UNREPORTED`; individual regimens stay as published, no imported Mayo baseline |
| UDT (K5) | unreported | `P-F5B-UNREPORTED`, `F5-K5-BOUNDARY`; no annual-monitoring multiplicity imported |
| PDMP (K6) | unreported | `F5-PDMP-SEARCH`; no provider query ratio turned into a patient count |
| Clinician role | 1 synthetic shared-role witness | `F5-CLINICIAN-DEGREE`; not an observed clinician-count estimate |

The case says medication interruption preceded admission, drugs were initially held, and home oxycodone/gabapentin were not restarted during admission. Therefore **prescription status is not actual administration**. No medication-fan-out benchmark may recode the entire tuple as actual concurrent inpatient treatment. No OUD diagnosis follows merely from chronic opioid use, withdrawal or addiction consultation. No CKD follows merely from the reported proteinuria at hospital presentation.

Window/dedup = `F5-WINDOW-*`, `F5-RECORD-MAP`. The source lacks dated supply intervals/refills; CMS460v2 ≥90-day exposure eligibility remains **unconfirmed**. Preserve the disclosed pre-interruption snapshot and the source's qualitative chronic-therapy wording; do not invent a 90-day history or laboratory/query/pain-duration repeats. G scales guideline loading, not these fixed source-supported key degrees.

## P-MX

Main's requested structural mixture = F3 topology/eligibility plus F5b's **drug counts, classes and PRN prescription status** (`MX-*`, `P-F5B-OPIOID-DEGREE`, `P-F5B-BZD-DEGREE`, `P-F5B-GABA-DEGREE`): a qualitatively source-supported synthetic scenario (`MX-COHERENCE`, scientist-3 G2), not an observed person. The two F5 people remain separate; Mayo's numeric duration, MME and dated UDT pair are not transferred, and no Sayles dose, age or interruption history is.

```text
Use P-F3's synthetic eligibility dossier and selected six-source topology.
Keep F5b's joint prescription counts: 2 opioids, 1 benzodiazepine, 1 gabapentinoid.
Preserve prescription status, episode identity and current-window caveats.
```

`MX-JOINTNESS`: the case report does not document OUD/T2DM/CKD or the full F3 eligibility dossier. This is a **main-selected synthetic structural mixture**, not an observed F5b patient with those diagnoses; its qualitative coherence is accepted (scientist-3 G2). An intact observed medication correlation is supported; the full joint tuple and its prevalence are not observed.

Matched one-factor diagnostic controls (not additional clinically observed positives):
- `MX-CONTRAST-BRIDGE`: remove the complete preflight-selected bridge-reader group — F3-B4's reader first, F3-B1's only if no clean observable F3-B4 target can be isolated — while upstream OUD/chronic-pain facts and every other classifier + reader stay unchanged; selection by correctness + structure, never by measured speed. A wider multi-producer cut would be labelled target-support-cut, not pure bridge removal.
- `MX-CONTRAST-KEY`: remove the second opioid record — the oxycodone prescription + product of `p-mx.ace` sentence 17 — and its associated fact set; opioid degree 2 → 1; fentanyl, alprazolam, the patient, every other record and G retained; surviving identities never renumbered.
- `MX-CONTRAST-SHARD`: replace the T2DM-labelled shared slot with an F1a-kind padding copy at the same slot; G = 100; intended active-source count 6 → 5; patient facts unchanged. An active-hub-versus-residue intervention: its clause, term, edge + answer deltas are the treatment, reported, since equal G is not equal logical work.

These are diagnostic controls, not additional clinically observed positives. Each contrast's descriptor (disabled documents or facts, the replaced slot) + its expected removed and preserved answer sets are recorded per seed in the frozen run manifest (`run-manifest.tsv`) and checked against the oracle before timing. All clinical/exposure limitations of P-F3 and P-F5b apply. Small G=1/2/4 prefixes are smoke-only for the full mixed topology; G=10/100 retain the selected six-source construction.
