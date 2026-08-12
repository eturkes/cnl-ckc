# roadmap

Charter: `.agent/initial-prompt.md`. Shipped: fetch→ACE→Prolog guideline
pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py --check`);
guideline coverage runs as built-in `/goal` rounds — procedure: root
`README.md` § Operating; queue: `.agent/queue.md`. This file plans further
project development; new tasks are planned in a new session on demand.

## Standing direction

- Product = the KB artifacts (`guidelines/*/{ace,pl}` + lexicons +
  provenance/coverage ledgers); consumers load them into their own
  engines. Compiled Prolog = a public interface once M3 lands:
  engine-portable plain clauses, schema documented + versioned. No
  query/serving layer in this repo — APIs, CDS integrations,
  FHIR/CDS-Hooks adapters = downstream projects; the check's derived
  probes are the sole in-repo query machinery (testing-grade by design).
- Multilingual (Japanese first): a port = a new source-language CNL
  frontend + jurisdiction-native corpus (Japanese guidelines, e.g. the
  Minds clearinghouse), never corpus translation — verification requires
  source-language comparison. DRS = the language-neutral waist
  (`ace_to_pl` consumes DRS, not English); `vendor/ape` grammar + ulex
  morphology = the English-specific components. Repo split (per-language
  repos vs agnostic core) is decided when Japanese work is planned; until
  then nothing English-specific enters `tools/`, `vendor/e--`, schemas or
  ledger formats, and all machinery treats lemma symbols as opaque atoms.
- Domain generalization: the pipeline is normative-document
  formalization, not clinical-only; domain rules (eligibility, actor
  classes, corpus protocol) stay in corpus data (compendium header,
  per-guideline files), never in code.

## M1 — American clinical guideline source compendium (easy tier) — REVIEWED

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

Terminal scope (user ruling, taken on M1.3's cost measurement): M1 closes
at ORG-UNIVERSE completeness, not easy-tier harvest exhaustion. Every
organization row ends terminal — `CPGs=yes|no`, `swept` dated or
`blocked(<why>)` — while the guideline table stays at the 1,118 rows M1.2
shipped. An org ruled `yes` whose index no M1 unit swept records
`blocked(easy-tier harvest deferred; M2)`: a named gap rather than a
hidden one, so the compendium is terminal-consistent the moment M1.3
lands. This retires the ≈8–12 further harvest units that sweeping the
whole easy tier would have cost — 150–190 orgs, being the 81 already
`CPGs=yes` + unswept plus the ≈115 easy-tier orgs the determination pass
will surface — and it keeps M2 sized by the real per-document cost of
`/goal`, which cannot be measured until M1 is REVIEWED and `/goal` has
consumed a useful run of easy-tier rows.

Re-cut (user ruling): M1 finishes in one, at most two more sessions,
MVP-first — the spine is what `/goal` consumes: a terminal org universe
and the rewired terminal condition, nothing else. Session A runs M1.3a2
(banked rulings) and chains into M1.3b (terminalization + cross-check +
integration docs); session B runs M1.5 (minimal gate port + light
review) and closes the milestone REVIEWED in the same session — the
user bound overrides the separate-review-session default. The per-org
determination pass defers to M2 whole: measured at 415–519 WebSearches
against the 200/session ceiling, it is 2–3 sessions of spend buying
`/goal` nothing it can consume now, while `blocked(...)` cells carrying
the crosswalk's signal hand M2 a costed, prioritized worklist.
Improvements off the spine land in the M2 register, never in new units.

Method (all units): research fan-out per `.agent/rounds.md` `res` role +
the session-roadmap brief spec. WebSearch budget = 200/session shared →
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
  (`.scratch/check_compendium.py`; M1.5 ports the `/goal`-consumed subset
  into `goal.py check`).
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
    This is the mechanism the cross-check's gap rows need.
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
- M1.2b DONE — society/other easy tier, 10 orgs. Compendium 435 → 1,118
  guideline rows over 447 → 474 organization rows; access open=834
  unverified=259 paywalled=19 login=6; status unqueued=802 excluded=257
  provisional=58 in-progress=1. Six `prod` teammates merged in ONE batch
  behind five idempotent post-merge fixers, every one replaying to
  byte-identical output over the closing tree. Contract =
  `.scratch/contracts/m1u2b.md` (R1 index identity · R2 out-of-universe
  issuance · R3 another rostered issuer's artifact · R4 co-issued rows ·
  R5 versions · R6 eligibility by content with the class-adjudication
  economy · R7 titles · R8 year · R9 access earned · R10 notes · R11 seeds
  and off-index · R12 host discipline; predicates P1-P9 with
  `merge_rows.py --dry-run` as the producer's own gate).
  Rulings, each landed in the compendium header:
  - `CPGs=yes` means the org issues ≥1 ELIGIBLE artifact, and a `CPGs=no`
    sweep is REQUIRED to emit `excluded(<why>)` rows for its index, so an
    `excluded(` row naming a `CPGs=no` org agrees with the header rather
    than contradicting it. The gate was wrong, not the data: it now skips
    the contradiction for `excluded(` rows alone. Two tests added — the
    accept case, proved red against the pre-change tool, and a scoping
    test holding the carve-out away from `provisional(...)`, which names
    an eligible row.
  - Co-development is co-issuance. ACC/AHA bundles collaboration and
    endorsement into one `Developed in Collaboration With and Endorsed by`
    footnote, so the ROSTER SENTENCE decides and title billing does not:
    the 2025 hypertension guideline names SGIM among the writing
    committee's represented organizations, and the 2026 pulmonary embolism
    guideline seats two `(SHM rep)` members. Both M1.1 `CPGs=no` rulings,
    made from the orgs' own sites, are refuted → `unverified` +
    `swept=pending`.
  - `abbrev` is display shorthand, never a key, so the seeded
    `American College of Chest Physicians (CHEST)` row was a second row
    for one org: the plain name is canonical and absorbed it.
  - Access is decided from the artifact, never from transport reaching a
    landing page. Four rows corrected `open` → `paywalled(...)`, found by
    two instruments that agree — `.scratch/audit_open_oa.py` screened the
    184 DOI-carrying `open` rows against Unpaywall (105 flagged, 4 survived
    confirmation against the live landing page) and the live sample flagged
    one of the four independently. Two of the four are the pair
    `prod-m1u2b-2`'s own report called paywalled while its cells shipped
    `open`.
  Access re-probe, the wave's largest correction: 66 rows sat
  `provisional(access unverified: ...)` on challenge verdicts that were
  largely `r.jina.ai` throttling. Re-probed
  (`.scratch/m1u2b/probes/recheck.tsv`) → 33 open, 15 cloudflare, 9
  cookie-wall, 3 http(404), 3 http(403), 3 human-verification; applying
  them moved exactly 32 rows `unverified` → `open`.
  Live sample: `verify_sample.py` seed 12, n=14 across the 9 swept orgs →
  14 substantive, 14 clean on access, title and year. The first run's one
  substantive miss was the verifier's, not the row's — a JACC AUC row
  scored 0.23 because the direct route got a publisher shell while the
  reader returns the artifact whole, so `verify_sample.py` now falls back
  to the reader on a poor title match rather than only on refusal or
  thinness, the better-MATCHING body winning with the stub guard intact.
  SAMHSA (M1.2c rows, met incidentally): the recorded `per-IP metering`
  cause was never measured. `library.samhsa.gov` runs an AWS WAF bot check
  over `/sites/default/files` that escalates to an image CAPTCHA, while
  product pages and `/search-endpoint` answer normally — an anti-bot gate
  needing a human, not a budget needing waiting. Three of the four URLs
  are dead (`Page Not Found` through a browser that had cleared the check)
  and TIP 61 is gone from the library, its product page 404ing too. Rows
  restated; relocation is M1.3's, and the CAPTCHA is a user gate.
  Gate: `check_compendium.py` PASS bare and under `--require-swept` over
  all 10 assigned orgs; `gate_m1u2a.py` 71 tests, the two new ones the
  carve-out's accept case and its scoping bound.
  main=85% 203K/240K, mate=100% 240K/240K.
  Scope rulings made during preparation, each evidence-backed:
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
Determination cost, measured — two disjoint
stratified samples of 20 orgs each (`.scratch/agents/scout-m1u3-1.md`,
`.scratch/agents/scout-m1u3-2.md`) ran the draft protocol end to end:
23 `CPGs=yes` + 15 `CPGs=no` + 2 unsettled ⇒ yes-rate ≈ 60%, and of the
23 yes only 13 are easy tier. Cost = mean 3.2–3.4 tool calls/org, median
3, worst 5, ~3.8K tokens/org ⇒ ~43 orgs before a teammate saturates. The
binding constraint is not context but the 200/session WebSearch ceiling:
at the measured 1.2–1.5 searches/org the 346 orgs cost ≈415–519
searches. Second-source acquisition (D5), not index adjudication, is the
dominant per-org cost in both samples — removed with the pass itself by
the re-cut: determination is M2 work, the crosswalk its worklist.

Bulk prefilter, measured against a 12-org set whose truth the scouts
established (`.scratch/agents/res-m1u3-1.md`): NGC ∪ Guideline Central ∪
AAFP-PG ∪ PubMed corporate-author flags 9 of the 11 true-yes orgs and
flagged none of the one true-no. Each is anonymous and bulk-extractable —
GC = 482 society profiles (419 with counts) from ONE fetch of embedded
Inertia JSON; NGC = a 293-org wayback directory plus a 9,984-summary XML
dump; AAFP-PG = 676 records over 7 Coveo calls; PubMed = one ESearch per
org at ≤3 req/s. Every hit is positive evidence only: an omission is
silence, and a corporate-author hit needs `CollectiveName` validation (a
`Heart Valve Society` hit resolves to the British society). ECRI
Guidelines Trust is offline and enumerates nothing.

M1.1 is NOT a cost analog and none of its reports supports a per-org
number: its 447 rows came from bulk frame extraction plus synthesis, not
a per-org determination pass. The scouts' 3.2–3.4 is the only measured
figure.

- M1.3a DONE — gate correctness + org-name collisions, MAIN-authored.
  Contract `.scratch/contracts/m1u3a.md`. Measurement re-split the unit as
  M1.3 itself was re-split: the five gate defects plus the collision merge
  consumed MAIN's window, so the three ruling batches (HICPAC attribution,
  ACCM, R6 close calls) move to M1.3a2 with their evidence banked rather
  than being ruled at the end of a spent window. Shipped:
  - Five gate defects fixed, each red first against `.scratch/m1u3a/prechange/`
    (the tools snapshotted byte-identical before MAIN touched them) and green
    after, under the diff-blind suite `.scratch/m1u3a/gate_m1u3a.py` (43 tests):
    - G1 the terminal clause is the header's DISJUNCTION — `CPGs=no`, or a dated
      manifest, or `blocked(<why>)` — so protocol D3d's `unverified` +
      `blocked(...)` is terminal; `--terminal` had rejected every `unverified`
      row, contradicting the rules it gates.
    - G2 independence excludes a frame the org or its PARENT controls, which
      `qualifying()` could not express while it received the sources cell
      without the org. `FRAME_OWNER` names the org row behind each frame and
      `CONTROLS` names component units (`APTA <name>` under APTA), so an
      academy's own row no longer counts its parent's listing of it. A
      third-party MEMBERSHIP roster stays independent for its members. The
      existence check fires only for a CITED label: an unused label's owner
      holding no row says nothing about the table.
    - G3 the manifest grammar and arithmetic reach EVERY dated `swept` cell,
      not `CPGs=yes` alone, and `--require-swept` now demands a well-formed
      manifest rather than any `<date> <anything>`. A `CPGs=no` manifest must
      claim 0 eligible, since a `no` ruling denies eligible issuance.
    - G4 a CPGs x swept legality matrix, all 12 cells tested: `-` is legal
      only under `no` (unswept and none owed), `pending` is illegal under `no`
      (a sweep owed for a ruling already made), `blocked` is illegal under `no`
      (that state is what D3d parks at `unverified`), and `unverified` + dated
      is LEGAL and load-bearing — a swept org holding one independent source
      cannot read `yes`.
    - G5 `merge_rows.py` gains the validated append `compendium-orgs-new`; a
      new name in the ordinary block stays an error, because that guard is what
      catches a typo in an existing name. Appends validate class, CPGs, the G4
      matrix, the `excluded:` reason, >=2 independent sources for `yes`, and no
      ladder collision; an identical re-append is a no-op, so a replay converges.
  - Organization-name identity, the independence tables and the canonical sorts
    moved into `compendium_io.py`: the inserter and the gate decide the same
    questions, and a second copy would let them disagree about whether two rows
    name one body.
  - Both ladder collisions resolved by `.scratch/fix_m1u3a_org_collisions.py`
    (replays to byte-identical output), 474 -> 473 org rows with the guideline
    count invariant at 1,118:
    - AAO-HNS. `entnet.org` states the Academy and the Foundation are "two
      separate and independent organizations, each incorporated in the District
      of Columbia", and all three guideline artifacts name the FOUNDATION as
      developer or publisher. The em-dash row was the Foundation misnamed — its
      abbrev already read `AAO-HNSF` and it held the Foundation's CPG index — so
      the two rows merge into the Foundation, the 2025 adult sinusitis row
      repoints to it, and the Academy enters fresh as `unverified`, a real
      distinct body with no eligible artifact yet evidenced. The pair is
      allowlisted in `DISTINCT_ORGS` with that reason, which is the mechanism a
      genuinely-distinct near-match takes.
    - ASPC. The body's own history names the 2005 Connecticut incorporation
      "The American Society for Preventive Cardiology" and no second entity
      answers to the `of` spelling, so the `of` row is a variant absorbed into
      the `for` row.
  Gates green from the closing tree: `check_compendium.py` bare -> PASS (473
  orgs, 1,118 guidelines, 0 rows below 2 independent sources), `gate_m1u2a.py`
  71 tests, `gate_m1u3a.py` 43 tests.
  main=82% 196K/240K, mate=65% 155K/240K.
- M1.3a2 DONE (tier=data) — attribution + eligibility rulings from banked
  evidence, MAIN-authored. Compendium 473 → 475 org rows (G5 appends: U.S.
  Public Health Service — the tobacco-panel row is a distinct body, no
  collision — and Society of Critical Care Anesthesiologists); guideline
  count invariant at 1,118. Two idempotent fixers, each proved by a second
  run reporting zero edits over byte-identical files:
  - `fix_m1u3a2_hicpac.py`: 32 rows re-attributed per
    `.scratch/m1u3a/hicpac-verdicts.tsv` — 25 leave the HICPAC org cell
    (CDC 22, USPHS 2, ACIP 1), every one keeping
    `indexed-by=Healthcare Infection Control Practices Advisory Committee`;
    7 retained rows gain co-issuers (Hand Hygiene 2002 → +SHEA +APIC
    +IDSA; BSI 2011 → its 15 working-group orgs incl. SOCCA; 5 rows →
    CDC + HICPAC); 14 stay byte-identical. HICPAC's manifest
    `49 -> 39e+8x` survives unchanged — every moved row still counts
    there and nowhere new (row 46 stays out of ACIP's `67 -> 63e+3x`
    exactly as rr6007 does). Row 25 ruled CDC alone: the artifact's
    suggested-citation line names CDC and HICPAC's 2019-05-17 approval is
    not issuance. Row 6's quote restored into the verdicts table from
    `res-m1u3-2.md:10`.
  - `fix_m1u3a2_r6.py`: all 58 close calls ruled — 44 keep-eligible (43
    producer keeps stand; p1-01 OVERRIDDEN to keep: its disclaimer
    describes evidence-grading form, and the header's artifact-type rule
    + procedure-standards clause decide, consistent with the six kept
    ATS/ERS technical standards) and 14 AASM conversions in place, no
    collapses: methodology companion (a header "methodology document",
    not a derivative rendering), endorsement letter (not systematically
    developed), 12 health advisories ruled as a class
    `excluded(patient-facing education alone; ...)` — two employer/carer
    sentences do not change the document class. AASM manifest
    `152 -> 67e+18x` → `-> 53e+32x`.
  - ACCM: Q3a settled — honorary/special body within SCCM under SCCM
    Council → `compendium_io.py` `CONTROLS` gains SCCM → ACCM; org row +
    3 guideline rows stand (artifact attributions unresearched); audit →
    M2 register.
  Gates green from the closing tree: `check_compendium.py` bare (475
  orgs, 1,118 guidelines, 0 short-sourced), `gate_m1u2a.py` 71,
  `gate_m1u3a.py` 44. Live spot-check 5/5 substantive across all three
  move classes + the advisory exclusion class (rr6210 CDC suggested
  citation; rr6904a1 PHS; stacks cdc_143156 ACIP; core-practices title;
  AASM advisory page).
  main=84% 201K/240K, mate=- (no teammates).
- M1.3b DONE (tier=data) — org-universe terminalization + cross-check +
  integration; chained after M1.3a2 inside session A.
  Crosswalk (`prod-m1u3b`): 347/347 unverified orgs × four bulk sources —
  GC 145 profiles (128 count>0), NGC 67, AAFP-PG 30 orgs/61 records,
  validated PubMed 126 of 1,044 fetched records (0 scout contradictions);
  validator PASS at production time (post-fixer its coverage predicate
  reports the ruled rows as extra BY DESIGN — terminal state is
  `check_compendium.py --terminal`'s to gate); regeneration
  `.scratch/m1u3b/build_crosswalk.py`, cache-only replay `cmp` rc=0.
  Terminalization (`.scratch/fix_m1u3b_terminal.py`, idempotent: 427
  edits, second run 0): scout verdicts 23 yes (13 easy + 10 hard carrying
  their own mechanisms) + 12 no; named rulings — OSG `no` (its sole
  attributed row is `excluded(derivative)`, gate-consistent), SGIM `yes`
  (DOI 10.1016/j.jacc.2025.05.007 + PMID 40815242; index re-pointed:
  banked URL 404s, live `/advocacy-policy/` path 200), SHM `yes`
  (PMID 22534627 + GC-publishers), VA PBM `unverified` +
  `blocked(<2 independent sources; heavy tail ~582; M2)`; unsettled:
  Army HP&R, HHS umbrella, PCNA (AAFP-PG + PubMed-2014 signal = M2
  priority), NABP; HVS + SCPC scout-`no` OVERRIDDEN → `unverified` +
  `blocked(determination unsettled; M2 — …)`: each co-issues an eligible
  shipped AUC row and the CPGs=no-vs-eligible-row predicate forbids `no`
  while the row stands (SCPC merger in-cell). 81 `yes`+`pending` → M2
  mechanism map (19) or easy-tier default (62); 301 undetermined →
  `blocked(determination deferred; M2 — <signal>)` with AMA-roster-only /
  supplement-only / issuer-of-N annotations. End state: 475 orgs =
  127 yes / 309 unverified / 39 no; ZERO guideline rows touched (1,118).
  Gate touches, pre-ruled or forced: seed-per-yes predicate removed
  (sources-only `yes`); org SWEPT regex → `BAL` (mechanism reasons nest
  parens; the status regex already did).
  Cross-check (`diff-m1u3b`): 23 swept/blocked orgs × 4 sources → 669 candidate
  gaps across 16 orgs at gap-list grade, recorded in compendium
  § Protocol layer 3 + the M2 register; NGC caches = org/count level, so
  zero fabricated titles; `build_gaps.py` cache-only replay `cmp` rc=0.
  Integration (MVP close): README § Operating `/goal` argument →
  compendium exhaustion clause; queue promotion feeds from
  `.agent/compendium.tsv` (cdc-2022-opioid ↔ its row cross-linked);
  header adoption sentence de-futured; memory regeneration bullet.
  SAMHSA relocation (pre-unit): `.scratch/fix_samhsa_relocation.py` +
  `fix_samhsa_protracted.py`, replayed byte-identical; zero
  `provisional(...)` SAMHSA rows.
  Gate: bare PASS; `--terminal` meter `unswept=0 unverified=309
  provisional=54` (row-status violations = the intended `/goal`
  worklist); `gate_m1u2a` 71 OK; `gate_m1u3a` 44 OK; live spot-checks
  SGIM + ANPT indexes 200.
  main=75% 179K/240K, mate=48% 116K/240K (diff-m1u3b peak; prod-m1u3b
  earlier).
- M1.5 DONE (tier=kernel — the E-- port; review runs data-grade) —
  hardening close, session B. Ported into E-- → `tools/goal.py check`
  ONLY the predicates `/goal` consumes: row format/vocabulary (org table
  + TSV), both canonical orderings, promotion invariant, terminal meter
  — compendium stage runs FIRST in `check_command`; contract
  `.scratch/contracts/m1u5.md` (P1-P14, divergences D1-D3 documented).
  The collision ladder, independence semantics, manifest arithmetic +
  unit-gate suites stay `.scratch/` with regeneration paths recorded in
  memory until an M2 harvest needs them — porting them whole would grow
  the audited codebase against the charter. Assurance: 48-fixture dual
  port/reference gate (45 both-verdict + D1/D2/D3 one each) green incl.
  live case; 35/35 mutation kills (two independent campaigns + MAIN
  kill check); adversarial review Q01-Q25 all green, findings F1-F4
  fixed+green; `regen.py --check` green; full check green, meter
  `475 organizations 1118 rows; orgs=0 rows=847 provisional=54`.
  README § Operating + Audit story name the stage; memory delta landed.
  main=69% 166K/240K, mate=100% 239K/240K (rev-m1u5 peak; test 39%,
  rev2 52%).
- M1 REVIEW DONE (data-grade, folded into session B per the re-cut
  ruling) — rev-m1 over 80c7a07 + 5e75811 + 8e90d3c: claims 13/13
  consistent (HICPAC 32-row recount, 475 orgs/1118 rows, terminalizer
  427→0 isolated replay, M1.5 gate identities); gates 4/4 green; live
  sample 10 = 5 pass, 1 partial → F-01, 4 CDC-hosted tool-403 rows
  covered by the M1.3a2 authenticated 5/5 spot-check. F-01 (medium):
  AASM infant-sleep advisory exclusion reason → `excluded(health
  advisory without systematic development)` — live audience =
  childcare providers/policy, so the patient-facing clause fell;
  `.scratch/fix_m1rev_aasm_advisory.py` replays to 0 edits. F-02
  (low): provenance narration pruned from roadmap + memory. Post-fix:
  deep gate PASS 475/1118; unit gate 49 OK incl. live; full check
  green, meter unchanged. main=80% 193K/240K, mate=55% 133K/240K
  (rev-m1).

Unit/milestone close per session-roadmap WORK-UNIT/MILESTONE-REVIEW
protocol (gauges recorded, commits `compendium (M1.<u>): …`).

## M2 — deferred hard-tier harvest — UNPLANNED

The register of what M1 consciously skips. Every entry here is an org
whose `swept` cell reads `blocked(<why>)`, so the compendium stays
terminal-consistent while naming its own gaps. Plan this milestone only
after M1 is REVIEWED and `/goal` has consumed a useful run of easy-tier
rows — the real per-document cost of `/goal` is what should size it.

Deferred classes, from the 46-org measurement
(`.scratch/agents/scout-m1u2.md`):

- Easy-tier orgs determined but never harvested — the terminal-scope
  ruling's direct output, and M2's cheapest class: every org ending
  `CPGs=yes` + `blocked(easy-tier harvest deferred; M2)`, each carrying
  the index URL and tier M1 recorded — ~94 known at the re-cut (81
  pending-since-M1.1 + the scouts' easy-tier `yes`), growing as the
  determination pass runs. The only class whose per-org sweep cost M1
  has already measured.
- Per-org determination pass — the re-cut's wholesale deferral: every
  row `blocked(determination deferred; M2 — <crosswalk signal>)`, the
  crosswalk its costed, prioritized worklist. Protocol-v2 friction
  rulings stay banked in the scout reports: D2's two-fetch cap covers
  own-site index discovery only while D5 buys one bibliographic fetch;
  transport classification runs direct→reader before any artifact
  fetch; a `CPGs=no` needs affirmative evidence (function + exhaustive
  own-site output taxonomy + a co-issuance check), never search
  silence; `WPSI-program` counts nothing toward the ≥2. Sized by the
  measured 3.2–3.4 calls + 1.2–1.5 searches/org against the 200/session
  WebSearch ceiling.
- Cross-check gap adjudication — M1.3b's gap list over the 23
  swept/blocked orgs: eligibility, version + access ruled per gap, rows
  entering through the off-index mechanism. Delivered: 669 candidate
  gaps/16 orgs (`.scratch/m1u3b/gapcheck/gaps.tsv`; record in
  compendium § Protocol layer 3). Determination-pass leads banked here:
  historical-issuer PMIDs ACRO 14585480 (2003) + SCPC 23892939 (2013);
  HVS + SCPC co-issuer attribution audit (eligible shipped AUC rows
  name both while both sit `unverified` + unsettled-blocked); APTA
  academy renames (APTA Neurology → Academy of Neurologic Physical
  Therapy; scout-1's alias-map shortcut: parent-directory crosswalk →
  active academy name/domain → own CPG page).
- ACCM artifact-attribution audit — Q3b/Q3c unresolved
  (`.scratch/agents/res-m1u3a-1.md`); relationship settled (SCCM special
  body; `CONTROLS` recorded); its 3 guideline rows' issuer cells stand
  unverified against artifact text.
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
- Storage format — guideline rows already live in `.agent/compendium.tsv`
  (rules + org table in the `.md`). At ≈4,400 rows revisit whether the
  org table follows.
- Row granularity — the M1.1 header rules appropriate-use criteria and
  committee opinions each into their own row, which is what puts ACR at
  279 and drives the ≈4,400 total. A coarser rule (one row per guideline
  series) would cut the compendium to ≈500 rows; revisiting it is a
  header-level decision, not a harvest decision.

## M3 — projection redesign (clinician-verifiable ACE) — REVIEWED

Goal: replace fixture-bearing legacy projections with source-anchored,
knowledge-only ACE; freeze an engine-portable public Prolog ABI; derive and
replay proof obligations per document + aggregate; migrate the complete
`cdc-2022-opioid` corpus without changing coverage/census/docid custody.

Terminal state:

- Corpus = 79 ACE = 79 compiled Prolog documents, one document per ruled
  source region, sentences per region free (21 single-sentence, rest 2-25);
  zero authored questions, zero witness fixtures.
- Schema v1 = frozen nine-indicator ABI documented in root README + compiler
  header; every corpus document compiles explicitly on v1. Legacy fact/rule
  emission stays byte-frozen; authored questions reject on both paths.
- Proof = 257 generated obligations, discharged per document and forward +
  reverse aggregate under bounded search (depth 4000 inside 10^6 inferences;
  either bound exceeded = underivable).
- Lexicon = 487 live entries, zero `pn_sg`, zero 77-roster mega lemmas.
- Custody = `audit/projection-notes.tsv` total, duplicate-free, set-equal to
  the ACE inventory; `coverage.tsv` + census keys unchanged through M3.
- Queue gate lifted in this review-close commit; further ACE authoring runs
  under the frozen v1 knowledge-only contract.

Governing authoring rulings:

- Preserve source conditions, objects, numbers + modality; record every
  approximation/drop in projection notes; introduce no unanchored structure.
- Hoist universal restrictors out of modal complements into antecedents; split
  compound "..., and if A then B" statements into separate ACE sentences.
- Render "only ... should ..." restrictions as conditional prohibitions, one
  rule per unmet conjunct; classical negation for unmet facts, NAF only for
  evidentiary absence ("unless there are indications ...").
- Map `might`->`may`, ability/option `can`->`can`, responsibility->`must`;
  modality stays reified data with no deontic inference. Causal rationale never
  becomes a temporal PP. Source `for` takes `prep(for,for)` + a determiner.
- Noun modification = relative clause or compound noun; `of` rejects
  (`condition_shape(relation(A,of,B))`).
- Source "or" folds only where the notes name the approximation; disjunctive
  consequents stay unsupported. Defeasibility/backend changes need a named
  consumer question and recompile the unchanged ACE corpus.

Assurance + evidence:

- Durable acceptance = `python3 -P tools/goal.py check` (terminal corpus,
  aggregate replay, 20 committed reds each pinning class + rc + byte-exact
  stderr) + `python3 -P tools/regen.py --check`.
- Closing M3 chain = durable gate -> `.scratch/m3u1/gate_m3u1.py` ->
  `.scratch/m3u2/gate_m3u2.py` -> `.scratch/m3u3/gate_m3u3.py` ->
  `.scratch/m3u7/diff_m3u7.py` -> fresh guideline compile ->
  `git diff --quiet -- guidelines/`.
- Question boundary = `.scratch/m3u7/suite/runner.py` 96 cells; contracts of
  record = `.scratch/contracts/m3u1.md`...`m3u7.md`; regeneration + port
  ownership = `.agent/memory.md` M3 evidence bullet + `.agent/polish.md`.
- Unit gauges: M3.1 main=84% 202K, mate=80% 192K - M3.2 main=93% 224K,
  mate=100% 239K - M3.3 main=102% 245K pre-compaction / 59% 143K close,
  mate=100% 240K - M3.4 main=94% 225K pre-compaction / 49% 117K close,
  mate=58% 139K - M3.5 main=92% 220K, mate=39% 95K - M3.6 main=87% 208K,
  mate=28% 67K - M3.7 main=93% 224K, mate=95% 227K.

Review (9 teammates: 7 unit lenses + cross-cutting + claim-replay audit;
26/28 audited claims replayed clean, the two misses being a stale brief claim
and the README bounds numerals fixed here):

- HIGH, fixed: aggregate payloads were self-certifying — an emptied payload
  beside a real product reported `aggregate ok 1 documents 0 obligations`
  rc0. Replay now checks the obligation set against the loaded composition
  before proving any head (coverage equality with the sentence identities the
  products carry, unique `(doc, sentence, variant)` keys, `1..K` variant
  runs, nonempty head lists, final-LF payload bytes). Battery
  `.scratch/m3rev/payload_battery.py` 9/9. Open residue: a dropped LAST
  variant stays undetected — rule-head `Deps` are variables, so per-sentence
  variant counts are not derivable under the frozen ABI (polish row).
- MEDIUM, fixed: committed reds pinned class/rc/framing but not Detail — a
  mutant rewriting both question details kept `goal.py check` green. Every
  probe now carries a required `.expect` sidecar compared byte-exact;
  orphan/missing pins are violations. 20 probes (added: v1 GROUP root
  detail, `of` noun modifier).
- MEDIUM, fixed: the ulex slot silently accepted the reserved basename
  `schema=v1`; `reserved_argv_token/1` now rejects it like `proof`.
- MEDIUM, fixed: the legacy witness covered only copula facts + one rule, so
  deleting the ground-fact branch killed nothing. Widened to every documented
  legacy family (copula, ground fact arity 1 + 2, positive rule, NAF rule
  with its dynamic declaration); `.scratch/m3rev/test_legacy_families.py`
  kills both branch mutants.
- MEDIUM -> docs: compiled documents are a definite-clause program, so an
  authored consequent entity feeding its own antecedent can diverge under
  naive SLD in one load order and succeed in another (verified: reverse order
  exhausts a 64 MiB stack, forward order proves). Our own replay is
  fail-closed — the same query under the compiler's bounds fails finitely —
  and no clause in the shipped corpus is left-recursive (scan 0/79, positive
  control 1). README carries the consumer note; the scan port is a polish row.
- Docs corrected: schema label candidate -> frozen; fork notice reduced to one
  §5(a)-relevant date plus git history; README gained the search-bound
  numerals, the aggregate-integrity guarantees, the `of` rewrite recipe and
  one-document-per-region wording; the guideline README's one-rule-per-impl
  claim replaced with the measured 21 single-sentence / 2-25 range.
- Deferred to `.agent/polish.md` with acceptance checks: last-variant residue,
  committed legacy-family oracle, committed left-recursion scan, object-
  operator closed-set kill, root-NAF precedence, vendor fork-notice gate.
- Review rerun (decisive chain, this commit): check ok 475 organizations /
  1,118 rows, terminal remaining orgs=0 rows=847 provisional=54, 1 guideline
  79 documents 20 red probes; regen check ok; gate_m3u1 24/10/0; gate_m3u2
  7/5/73; gate_m3u3 64/64; diff_m3u7 100 cases accept=79 reject=21
  divergences=0; question suite 96/96; fresh compile leaves `pl/` + `ace/`
  byte-identical. main=84% 200K/240K (review close; the wave spans 2
  windows), mate=86% 206K/240K (mrev-m3u7 peak).

Out of scope remains: serving/query API; probabilities, thresholds, arithmetic
or unit conversion absent from source; defeasibility/s(CASP) before a named
consumer question; rec6-12 authoring and pending-region/coverage closure,
which return to `/goal` rounds after review.

Sizing (analogs: M1.5 main=166K narrow kernel; M1.3a main=196K; M1.2b
main=203K oversized harvest; M1.3b main=179K): kernel units project
~150-175K MAIN with a hard checkpoint at contract+manifest before
implementation begins; oracle teammates may span planned successors
(each dispatch <=~170K). Production batches budget by source words
(~1,000/batch), not doc count; MAIN ~150-170K each. Off-spine items live
in `.agent/polish.md`, never as units. Out of scope: defeasibility/
s(CASP) (named consumer question first; backend swap = one backend +
recompile), serving layer, recs 6-12 authoring (post-M3 `/goal` work),
queue items 1-2 (pending-region rulings + coverage closure = `/goal`
rounds), new coverage rulings, probabilities/thresholds/conversions
absent from source.
