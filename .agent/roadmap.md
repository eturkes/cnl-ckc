# roadmap

Charter: `.agent/initial-prompt.md`. Shipped: fetch→ACE→Prolog guideline
pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py --check`);
guideline coverage runs as built-in `/goal` rounds — procedure: root
`README.md` § Operating; queue: `.agent/queue.md`. This file plans further
project development; new tasks are planned in a new session on demand.

## M1 — American clinical guideline source compendium (easy tier) — IN-PROGRESS

Goal: `.agent/compendium.md` = the complete master list of eligible
American clinical guidelines available online — one row per guideline:
issuing org | title + year | canonical source URL | access
(`open|paywalled(<gate>)|login(<gate>)`) | status
(`unqueued|queued|in-progress|done|blocked(<why>)|excluded(<why>)`) —
discovery-complete per a documented, auditable search protocol. Downloads
stay deferred: an entry is fetched only when promoted into
`.agent/queue.md`. This gives the standing `/goal` a finite terminal
condition — done = compendium exhaustion (every row
`done|blocked|excluded`) — replacing the open-ended web-search clause.
Planned directly with full session context at user direction; planning
waves intentionally skipped.

Tiering (user ruling, measured by `.scratch/agents/scout-m1u2.md`): the
420-org sweep population carries ≈4,400 guideline rows for the 97 known
issuers alone and runs ≈24 teammate waves, so M1 harvests the EASY TIER
and defers the rest to M2 rather than stalling. An org is easy when its
index enumerates in ≤2 anonymous fetches — `static-list`, `search-api`,
`pdf-index`, or a scripted-UA 403 that the `r.jina.ai` reader clears —
and its artifacts classify without an authenticated session. Everything
else is deferred by recording the org's `swept` cell as
`blocked(<why>)`, which the compendium's terminal condition already
accepts, so M1 closes honestly with the deferral set enumerated rather
than hidden. Deferred classes, each with named orgs, live in M2.

Method (all units): research fan-out per `.agent/rounds.md` `res` role +
the session-prompt brief spec. WebSearch budget = 200/session shared →
explicit per-teammate allowances (~30–40); bulk index crawling goes
through BrowserOS/WebFetch instead. BrowserOS (the authenticated
day-to-day browser incl. university journal access) = the route for
paywalled/member/login-gated listings — it is one live browser profile =
single-holder-per-wave resource: one designated holder (MAIN or one named
teammate) per wave; every other teammate stays on WebSearch/WebFetch and
queues BrowserOS verifications to the holder's worklist. Auth, captcha,
2FA, or any human gate → teammate flags it in its report; MAIN batches
the gates and asks the user immediately while other waves continue.

- M1.1 DONE — schema + protocol + seed roster. `.agent/compendium.md`
  ships the header (eligibility + rulings, three-layer discovery
  protocol, independence definition, row formats, enumeration-source
  legend, queue promotion + ordering) with 447 organization rows
  (federal 15/3/12, society 75/283/6, other 7/37/9 as yes/unverified/no)
  and 88 seed guideline rows. Gate green: every `CPGs=yes` row carries ≥2
  independent enumeration sources under the owner rule, every such org
  holds ≥1 seed, and all structural/ordering/vocabulary predicates pass
  (`.scratch/check_compendium.py`; M1.5 ports them into `goal.py check`).
  main=83% 198K/240K, mate=86% 207K/240K.

Harvest units run one shape: `prod` teammates sweep their assigned
indexes behind `.scratch/check_compendium.py`, MAIN merges with
`.scratch/merge_rows.py` (the only row inserter; reports are the source
of truth, so every repair edits a report and re-merges), re-verifies a
sample per wave against live artifacts, and records each swept org as
`<date> <method>; <n> index entries -> <e> eligible + <x> excluded` or
`blocked(<why>)`. Cap = ~200 emitted rows or 4 ordinary orgs per
teammate.

- M1.2a DONE — federal easy tier, 13 orgs, 318 rows; compendium 88 → 395
  guideline rows. Swept with dated manifests: DHA JTS 103 · BOP 47 ·
  HICPAC 47 · SAMHSA 45 · ACIP 27 · VA/DoD 26 · HRSA 14 · ClinicalInfo 6
  · IHS 1 · NAEPP 1 · PHS tobacco 1. Stale ACIP `index URL` repaired.
  Gates green: `check_compendium.py --require-swept` over all 13 assigned
  orgs, and the 25-test `gate_m1u2a.py`. MAIN re-verified 12 rows across
  6 orgs against live artifacts (10 clean on title+year+access; 2 thin
  reads verify nothing and are reported as such).
  Scope corrections against the plan, each evidence-backed:
  - USPSTF 90 → `blocked(...)`. Anonymous enumeration is exhausted: the
    JSON API is key-gated behind a 202, the Drupal view carries no
    `views` key in `drupal-settings-json`, there is no sitemap, and the
    reader proxy returns the shell only. Seed rows preserved.
  - NIOSH → `blocked(...)`, NOT `CPGs=no`: the digital radiography
    guideline (DHHS Pub 2011-198) does qualify, so the org stays
    `unverified` and joins M1.3 rather than being ruled out.
  - VA/DoD 38 → 26: the live index carries 27 topics, one of them a
    superseded 2012 external edition that collapses per the versions rule.
  - SAMHSA ≈32 → 45, from 74 index entries: the TIP facet folds 48
    numbered products into 26 concepts, the Advisory facet adds 14.
  - JTS 106 → 103 rows, of which 20 are excluded: 15
    `excluded(veterinary; not human patient care)` — the index carries
    military-working-dog CPGs, and patient means human patient — plus 3
    parentless iCOVER derivatives and 2 process/author documents.
  Rows left deliberately unresolved, both non-promoting by construction:
  - 36 SAMHSA rows `unverified` + `provisional(...)`. `library.samhsa.gov`
    meters anonymous requests per-IP and answers past the budget with 202
    + zero bytes, its own site root included. 9 artifacts earned an
    individual `open` verdict; the rest were never reached, and access is
    classified from the artifact, so they cannot read `open`.
    `.scratch/fix_m1u2a_samhsa_access.py` is convergent — rerun it after
    `.scratch/probe_until_settled.py` collects more evidence and exactly
    the newly proven rows promote.
  - 13 HRSA/WPSI rows `provisional(year unresolved: ...)`. No version
    year appears on a WPSI recommendation page under either a reader
    proxy or a real browser, beside a service in the HRSA index, or on
    the WPSI recommendations table; the sweep had reported those pages
    establish version years. The cervical-cancer row keeps its year,
    which the index states as a dated adoption.
  main=87% 210K/240K, mate=100% 240K/240K.
- M1.2c DONE — M1.2a review remediation + ACIP re-sweep + attribution
  hardening. Compendium 395 → 435 guideline rows. Five findings closed in
  the M1.2a commit range (BOP administrative exclusions 44+3 → 39+8; SAMHSA
  TIP 51 repointed from its KAP Keys derivative to the parent protocol;
  the access fixer's idempotence keyed on shape, not exact text; the
  per-host probe lock widened to span the request — 4 concurrent before, 1
  after; manifest `e/x` bound to the rows the org's own report emits).
  Rulings, each landed in the compendium header as a general rule rather
  than an ACIP special case:
  - Bounded fan-out is easy tier — an index of topic subindexes qualifies
    when the fan-out is enumerable from the index in one fetch, bounded
    (≤30), and every subindex is anonymous + static. ACIP costs 1 + 27
    fetches, cached in `.scratch/m1u2c/acip-pages/`. `blocked(...)` was
    rejected: the fetches are paid and ACIP is a first-rank federal
    issuer. No aggregate endpoint substitutes — `recs-by-date` stops at
    2024, mixes current with superseded, and one row links the wrong
    artifact, while the topic pages carry the CURRENT/ARCHIVED split the
    versions rule needs.
  - A manifest reconciles an INDEX; the org cell records ISSUANCE. Review,
    liaison input and endorsement are not issuance, so rr6007 reads ACIP —
    an ACIP work group wrote it, ACIP voted it — and carries
    `indexed-by=HICPAC`, counting in HICPAC's manifest.
  - `indexed-by=<org>[ + <org>...]` names the whole set of indexes that
    carried the row, so an artifact its issuers and a third org all index
    counts once in each manifest. Naming exactly the issuers is the
    default and is refused as redundant.
  - `<n>` counts artifact entries; navigation to another site's directory
    is not an entry (IHS reads `1`), so `n − (e + x)` always means
    collapse.
  - `off-index(<why>)` = no index carried the row, so it belongs to no
    manifest and no index sweep retracts it: silence from an index is not
    evidence about a row that was never in one, and only a batch
    re-emitting the artifact replaces it. Two ACIP rows (`mm7501a2`,
    `mm7422a3`) sit there — real current guidelines CDC has yet to index.
    This is the mechanism M1.4's cross-check gap rows need.
  - One artifact in two indexes (`rr5207a1`, ACIP + HICPAC): both reports
    emit it byte-identically, both manifests count it, the merge
    deduplicates to one row and refuses any cell disagreement — decided
    before lifecycle suppression, so a `queued|in-progress|done` row
    cannot hide a disagreement until it clears. The alternative, one
    emitting and one silent, would read as a collapse that never happened.
  - Manifest exactness rests on the report being the row's provenance, so
    it holds only while co-issuers are swept by separate reports. One
    report holding every issuer must name the indexes explicitly or be
    refused.
  - `indexed-by=` and `off-index(` answer one question — a row carries at
    most one, and never two `indexed-by=` markers.
  - MMWR states a label year in the title and a publication year in the
    volume (`year = 1951 + volume`); rows record `title (label-year; pub
    YYYY)` when they differ, since the table sorts on version year. A
    season range names no single label year and keeps the publication year
    alone.
  - Titles are the artifact's own, read off its suggested-citation line,
    never an index's display shorthand.
  - Two current artifacts may be scope-distinct because the later replaced
    one component of the earlier; `notes` then carries the version
    qualifier, quoted from the superseding artifact.
  Harvest: ACIP re-swept at artifact granularity, 27 → 68 rows (66 index
  artifacts + 2 off-index) = 63 eligible + 3 excluded, with 1 collapse for
  the cross-topic duplicate `mm7336a4`. The exclusions state no
  patient-care recommendation, each confirmed against its own text: a
  vaccine replacement-and-destruction notice (`mm5708a6`), surveillance
  case definitions (`rr5501a1`), a vaccination-certificate documentation
  requirement (`mm5651a4`). All five reports merged in one batch, which the
  co-issued row required.
  Defects the unit found and fixed, beyond the review's list:
  - ACIP's `mm6112a4` row (varicella VariZIG) was an ARCHIVED artifact, so
    the M1.2a sweep took a superseded document as a topic's representative.
  - 5 rows dropped the label year; 3 carried an index's shorthand title.
  - The gate's own re-derivation of ACIP's index found 64 of 66 plus false
    strays. Three parse traps cost one artifact each: `Archived
    DTaP-IPV-Hib-HepB …` capitalizes ARCHIVED differently, a listing links
    its own article with a tracking query, and one listing may link several
    articles whose extras are its Print version and Appendix — parts of an
    artifact, not artifacts.
  - The label-year acceptance check was vacuous: it re-ran the fixer's own
    detection regex over shipped rows with no positive control, so a
    `(?!)` mutant of that regex left the whole gate green. It now drives
    the fixer over synthetic positives and negatives, and that mutant kills
    3 subtests. The fixer itself missed a month-qualified label year
    (`— United States, December 2024 (2025)`).
  - Merge and validator attribution: a lifecycle-protected row suppressed
    both emissions of a shared artifact before they were compared; an
    issuer's index sweep silently deleted `off-index` rows; a row could
    carry contradictory provenance markers; one report sweeping every
    issuer of a co-issued row counted it for each off no evidence.
  Every repair is an idempotent `.scratch/fix_m1u2c_*.py` replaying to zero
  edits (eligibility, attribution, ACIP supersede, label years, verbatim
  titles, version qualifiers), so the wave re-derives from the reports.
  Gates green from the merged tree: `check_compendium.py` bare and with
  `--require-swept` over all 13 assigned orgs, and `gate_m1u2a.py` at 56
  tests. Every accepted finding proved red against pre-change tool copies
  in `.scratch/m1u2c/prechange/` and green after. Re-derived by MAIN over
  the final tree: the attribution mutation campaign kills 33 of 33 mutants
  (75.8% → 100% once the 8 survivors' kill tests were ported into the
  gate), and merging the five reports in three different orders is
  byte-identical and idempotent. M1.2b re-derived that claim and corrected
  it: re-merging the five reports over the shipped compendium replays it
  byte-identically (`git diff` empty, SHA-256 `8e30028b5d4158d8…`), and
  replaying them over the pre-merge baseline `d250658` reproduces every
  row while differing in the header prose that same commit authored —
  the merge rewrites the two TABLES only. The SHA-256 recorded here
  originally (`fdf76f4c…`) belongs to no shipped state.
  MAIN re-verified 8 rows against live
  artifacts through the authenticated browser — CDC returns 403 to
  scripted fetches and to headless Chrome alike — 8 of 8 confirming title,
  year, eligibility and access; a whole-corpus scan reconciles all 75 MMWR
  rows to `1951 + volume`, the single exception being an issue cover-dated
  into the next January.
  Deferred to M1.3 with evidence: HICPAC's index carries other bodies'
  guidelines well beyond rr6007. `rr6904a1` states it is a U.S. Public
  Health Service Guideline naming HICPAC nowhere, and `rr6210` is CDC
  guidance where HICPAC supplied liaisons — both shipped under HICPAC as
  issuer. The correct issuer for the PHS artifacts has no organization
  row, and sweeps never add org rows, so the audit belongs to the unit
  that fixes the org universe.
  main=94% 225K/240K, mate=98% 234K/240K.
- M1.2b OPEN, PREPARED — society/other easy tier, now 10 orgs ≈ 530
  emitted rows (eligible + excluded), re-sized from wave-1 measurement
  against the planned 11 orgs ≈ 320. Contract = `.scratch/contracts/m1u2b.md`
  (R1 index identity · R2 out-of-universe issuance · R3 another rostered
  issuer's artifact · R4 co-issued rows · R5 versions · R6 eligibility by
  content with the class-adjudication economy · R7 titles · R8 year ·
  R9 access earned · R10 notes · R11 seeds and off-index · R12 host
  discipline; predicates P1-P9 with `merge_rows.py --dry-run` as the
  producer's own gate). Resume = read that contract, harvest the wave-1
  reports (`.scratch/agents/map-m1u2b-1.md`, `map-m1u2b-2.md`,
  `res-m1u2b-1.md`), fill the contract's per-org index-identity table,
  dispatch ~5 `prod` teammates, merge in ONE batch, gate, verify a live
  sample, close. The harvest needs a full fresh window: preparation alone
  spent one.
  Scope rulings already made, each evidence-backed:
  - ACOG → `blocked(...)` → M2, under contract R1. Its recorded index is
    the `Clinical Practice Guideline` content-type facet: 61 of ~2,500
    clinical records its own Coveo endpoint reports anonymously
    (`.scratch/m1u2b/indexes/acog-facets.json` — Committee Opinion 1332,
    Practice Bulletin 471, Practice Advisory 167, Clinical Updates 167,
    Committee Statement 115, Position Statement 78, CPG 61, Obstetric Care
    Consensus 53, Clinical Consensus 49, Task Force Report 25, Technology
    Assessment 14). Enumeration is one API call; per-artifact version and
    access adjudication is not, so sweeping the narrow facet would record
    ACOG swept while hundreds of eligible artifacts held neither a row nor
    a deferral.
  - APTA Orthopedics holds its own organization row (one of 18 APTA
    academy rows), so the `orthopt.org` sweep emits under it, not under
    the parent APTA row — `APTA sections/academies` is parent-controlled
    and never counts toward its ≥2 independent sources.
  - NPIAP ≈1 was undercounted: it co-leads the fourth-edition
    International Guideline (co-equal with EPUAP + PPPIA, NPIAP chairing
    the governance group) and issues US-only current artifacts beside it.
  - The five `unverified` orgs here (ACG, AASM, CFF, APTA Orthopedics,
    NPIAP) resolve to `CPGs=yes`; ACG and AASM already carry ≥2
    owner-distinct sources, CFF and NPIAP and APTA Orthopedics take the
    additions `res-m1u2b-1` supplies. AOCD → `CPGs=no`.
  Tooling shipped by this preparation: `merge_rows.py` lets an issuer's
  sweep replace a co-issued row its own index carried, printing the diff
  for a MAIN ruling. Tiering splits co-issuers across milestones — ACC
  sweeps here while AHA, ADA and ASN wait for M2 — so the previous rule
  ("sweep every issuer together to replace it") made a co-issued row
  unmaintainable and would have refused ACC's own sweep of its seed row.
  Replacement authority is ownership: being one of the row's issuers, or
  the index that carried it; a third org reaching the same artifact is
  still refused, and a `queued|in-progress|done` row is still suppressed
  rather than replaced. `gate_m1u2a.py` 56 → 59 tests, all three proved
  red first.
  Gate: as M1.2a, plus every assigned org carrying a dated manifest or
  `blocked(<why>)` under `--require-swept`.
- M1.3 OPEN — determination pass over the remaining 314 `unverified`
  orgs, plus the HICPAC issuer audit M1.2c deferred here (every
  HICPAC-indexed row re-attributed to the body that issued it, adding the
  organization rows those issuers need — MAIN's authenticated browser is
  the only route into CDC). Cheap per org (≤2 anonymous fetches): decide `CPGs=yes|no`, record
  `swept` date+method, and sweep inline when the org is both easy and
  small (`<15` rows); everything else takes `blocked(<why>)` and joins
  the M2 register. This is what fixes the org universe, so it precedes
  the cross-check. Gate: zero `unverified` org rows remain; every row
  swept or blocked with a named reason.
- M1.4 OPEN — completeness cross-check, scoped to the easy tier.
  Aggregator sweeps diffed against the org-derived list; gaps feed back as
  rows; MAIN dedupes to one row per guideline (latest current version) and
  rules eligibility edge cases. A gap landing on a deferred org joins the
  M2 register instead of the gap list. Gate: cross-check documented in the
  compendium header with the easy-tier gap list driven to zero; dedupe
  pass recorded.
- M1.5 OPEN — integration. Root `README.md` § Operating + the canonical
  `/goal` argument rewritten → terminal condition = compendium
  exhaustion; `.agent/queue.md` feeds from the compendium (promotion
  rule); cdc-2022-opioid linked to its compendium row; the M1.1 + M1.2
  gate predicates ported from `.scratch/check_compendium.py` into E-- →
  `tools/goal.py check`; `memory.md` delta. Gate: `tools/goal.py check`
  green incl. compendium predicates; `regen.py --check` green; docs
  mutually consistent; scoped commits.

Unit/milestone close per session-prompt WORK-UNIT/MILESTONE-REVIEW
protocol (gauges recorded, commits `compendium (M1.<u>): …`).

## M2 — deferred hard-tier harvest — UNPLANNED

The register of what M1 consciously skips. Every entry here is an org
whose `swept` cell reads `blocked(<why>)`, so the compendium stays
terminal-consistent while naming its own gaps. Plan this milestone only
after M1 is REVIEWED and `/goal` has consumed a useful run of easy-tier
rows — the real per-document cost of `/goal` is what should size it.

Deferred classes, from the 46-org measurement
(`.scratch/agents/scout-m1u2.md`):

- Corpora whose breadth outruns the easy tier — enumeration is one
  anonymous call but per-artifact version and access adjudication is not,
  so the org defers whole rather than being swept against a narrow facet
  (M1.2b ruling R1): ACOG (~2,500 clinical records across 11 guidance
  types, of which the compendium's recorded index sees 61).
- Browser-gated indexes — the authenticated browser is the only route:
  AAP, NKF, HRS, ASCO, ACCP (Cloudflare), CFF index (Akamai), OPA
  (Cloudflare), NIAID (JS CAPTCHA), CPIC (JS-only shell).
- Login corpora — one gate covers a whole corpus, so each is a single
  holder pass: NCCN (free account, ≥91 rows), AORN (eGuidelines
  subscription, 36 rows).
- Heavy tail — enumeration is cheap but per-artifact resolution is not,
  so each wants a solo teammate split by topic or pagination: ACR 279,
  VA PBM ≈582, ASCO ≥139, IDSA ≈105 (109 fetches), AAN 65 (7 AJAX
  pages), ACR-scale others surfacing during M1.3.
- Version-ambiguous collections — the index mixes current with
  superseded, so a version ruling must precede row emission: CDC Stacks
  (786 records over 40 pages).
- Missing or stale enumeration surfaces — a replacement index must be
  found first: AAFP (methodology overview, no artifact index),
  ADA-Dental (404), AUA + ATS + BTF (indexes expose only part of the
  corpus), Endocrine Society (count needs the listing API).
- Access classification deferred wholesale — M1 records `access` from
  the artifact only where an anonymous fetch settles it; the remaining
  rows keep `unverified` + `provisional(…)` and never promote, which is
  the intended M2 worklist rather than a defect.
- Storage format — at ≈4,400 rows the guideline table outgrows a
  markdown file every agent reads whole. Moving guideline rows to
  `.agent/compendium.tsv` (rules + org table staying in the `.md`,
  matching the `coverage.tsv` precedent) is cheap now and expensive
  later; M1 stays on markdown per the easy-first ruling.
- Row granularity — the M1.1 header rules appropriate-use criteria and
  committee opinions each into their own row, which is what puts ACR at
  279 and drives the ≈4,400 total. A coarser rule (one row per guideline
  series) would cut the compendium to ≈500 rows; revisiting it is a
  header-level decision, not a harvest decision.
